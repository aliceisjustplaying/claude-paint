# /// script
# requires-python = ">=3.11"
# ///
"""Round 19: the Friedrich chain (lane F: three painters, one after another) and, if switched
on in LANES, blank-studio single painters, all run in parallel.

Lanes (LANES): F, a chain of PAINTERS=3 Opus 5.5 (thinking high) painters in Friedrich
studios, each after the one before, with the reader's records of the earlier painters merged
into its studio notes (nothing in them says there were earlier painters). The blank lanes
(one painter each, blank studio, "a picture of your choosing") are prefilled in LANES and
commented out: uncomment the ones to run. Lane names never reach a painter: studios get
neutral names (paint-studio-<hash>), and the mapping stays in run/studios.json.

Round 19's changes to what painters see (see CHANGES.md): no `check` in the painter's
easel, brief, guide or notes; no "in one session", no `timeout` rule and no water in the
brief; a plain launch and sitting message; subject-neutral guide examples; research notes
without the developer framing, varnish, craquelure or snow; a neutral compaction header; no
settings file in the studio.

Sittings: the painter works in sittings, each a new pi session (same harness and studio),
until it decides it's done. Sitting 1 gets PAINTER_MSG, later ones SITTING_MESSAGE. After each
sitting the chunks of the painting's log that put marks on the canvas are counted
(painting_chunks.py). The painter stops after a sitting it ended itself that added no such
chunks. MAX_SITTINGS completed sittings is only a safety cap (logged as NOT FINISHED).
A sitting that crashes (pi exits non-zero, or its session ends on a provider error) is no
judgment and counts for nothing: the runner waits (CRASH_WAITS, or longer if the error says
"retry in Ns") and starts another sitting; after MAX_CRASHES crashes the painter stops (NOT
FINISHED). A usage limit (OpenCode Go's 5-hour or weekly window) is no crash: the sitting is
marked 'limited', the runner probes the provider until it answers (LIMIT_PROBE_S, up to
LIMIT_GIVE_UP_H hours), and then the painter carries on in the same session (CONTINUE_MESSAGE,
same sitting number) if it had worked in it, or starts a fresh sitting if not. Each attempt is
recorded in run/<lane>/p<n>_sittings.json (a continued sitting's parts share its number).

After a painter's last sitting, in the background (the next painter doesn't wait):
  - check: scripts/check_painting (replay build) reopens a copy of the log, runs `check`
    and compares the replay with the painter's last save; the result goes to the log
    (run/<lane>/p<n>_check.log, workdir run/<lane>/p<n>_check/);
  - finishing: varnish and cracks on a replay of the log (run/<lane><n>_finished.png; the
    painter's own save is untouched).
Then, in a chain, a reader writes the record for the next painter.

Painters run in the clean harness (claude-paint-r19-base/harness/painter): no global
extensions, our system prompt, bash and read only, our compaction (its thresholds set by
compaction.ts), --no-approve and no .pi/ in the studio. pi-black for Anthropic lanes only.
Per-lane environment (model(..., env=)) reaches the painter's pi: PAINTER_MAX_IMAGES,
PAINTER_MAX_IMAGE_MB, PAINTER_INPUT_TPM (harness/painter/README.md).

Resumable: every finished step leaves a marker in the run folder; a rerun skips it (and
starts any check or finishing that didn't finish). A watchdog stops any process working in
one of this round's studios (its cwd there, or the studio in its command line) that has run
longer than WATCHDOG_MIN minutes, except the painter's pi, the easel server and the runner's
own check and finishing; it logs each stop. It also watches that each painting's log only
grows.

    uv run r19_chains.py                  # the lanes in LANES
    uv run r19_chains.py --only F         # some of them
    uv run r19_chains.py --dry            # print the exact commands, run nothing
    uv run r19_chains.py --briefs DIR     # render every profile's brief into DIR, run nothing
"""
import argparse
import hashlib
import json
import os
import re
import shlex
import shutil
import subprocess
import threading
import time
from pathlib import Path

from painting_chunks import count_painting_chunks

HOME = Path.home()
A = HOME / "src/a"
BASE = A / "claude-paint-r19-base"
BRANCH = "r19-base"
EXPORT = BASE / "scripts/export_r16_studio"      # honors R16_BRANCH
FINISH = BASE / "scripts/finish_painting"
CHECK = BASE / "scripts/check_painting"
H = BASE / "harness/painter"
BLACK = HOME / ".pi/agent/git/github.com/aliceisjustplaying/pi-black/extensions/pi-black.ts"
HERE = Path(__file__).resolve().parent
RUN = HERE / "run"
SESS = HOME / ".pi/agent/sessions"
WATCHDOG_MIN = 30

# the painter's tools (harness/painter/easel-tools.ts): the easel and reading the studio; no shell
TOOLS = "paint,look,note,status,log,read"
HARNESS = ["--no-extensions", "-e", str(H / "painter.ts"), "-e", str(H / "compaction.ts"),
           "--system-prompt", str(H / "system_prompt.md"), "--tools", TOOLS,
           "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve"]
PAINTER_MSG = ("Your brief is in BRIEF.md in this folder. Your last message is your reply: the "
               "painting's title if you give it one and a few sentences about the picture.")
SITTING_MESSAGE = ("You're back at the easel. The painting is as you left it. "
                   "Your brief is in BRIEF.md and your journal in notes/journal.md.")
# A painter works in up to MAX_SITTINGS sittings (completed ones, and crashed ones that painted); it stops
# earlier after a sitting it ends itself without adding paint
MAX_SITTINGS = 4
MAX_CRASHES = 6                                   # crashed sittings (in all) before a painter is stopped
CONTINUE_MESSAGE = "The connection dropped for a while. Carry on where you left off."
CRASH_WAITS = [90, 180, 300, 600, 900, 1200]      # s before the sitting after the 1st, 2nd, ... crash
# A provider's usage limit (OpenCode Go: 5-hour, weekly and monthly windows) is no crash: the
# painter waits it out. The runner asks the provider every LIMIT_PROBE_S (or at the reset time
# the error names) with a one-line prompt outside the studio, and starts the next sitting once
# it answers; after LIMIT_GIVE_UP_H hours it stops the painter (rerun to go on).
LIMIT_PROBE_S = 30 * 60
LIMIT_GIVE_UP_H = 24

READING = {
    "friedrich": ("notes/easel_guide.md; notes/studio_notes.md;\n"
                  "notes/research/friedrich_materials.md (his materials and method, sourced);\n"
                  "notes/research/trees.md (how trees are built) and\n"
                  "notes/research/oil_paint_physics.md as needed."),
    "blank": "notes/easel_guide.md; notes/studio_notes.md;\nnotes/research/oil_paint_physics.md as needed.",
}
OPENING = {
    "friedrich": (
        "Compose and paint one original landscape in the manner of Caspar David\n"
        "Friedrich, at the easel, a simulator of oil paint on linen. The place,\n"
        "subject and composition are yours to invent. Work from knowledge and the\n"
        "notes in your studio; don't use reference images, image models or pictures\n"
        "of his work."),
    "blank": (
        "Paint one picture of your choosing in oil, at the easel, a simulator of oil\n"
        "paint on linen. Subject, composition and manner are yours. Work from what you\n"
        "know; don't use reference images or image models."),
}


def api_key(entry):
    """A provider key stored in pi's auth.json (read at run time; never printed or written)."""
    a = json.loads((HOME / ".pi/agent/auth.json").read_text())
    e = a.get(entry) or {}
    return e.get("key") or e.get("apiKey") or ""


def model(provider, name, thinking, black=False, env=None, key_from=None):
    """black: load pi-black (Anthropic OAuth through the Claude subscription). env: extra environment
    for the painter's pi (harness/painter's PAINTER_MAX_IMAGES, PAINTER_MAX_IMAGE_MB, PAINTER_INPUT_TPM)."""
    return dict(args=["--provider", provider, "--model", name, "--thinking", thinking], name=name,
                black=black, env=env or {}, key_from=key_from)


def lane(profile, m, painters=1):
    """profile: the studio (friedrich or blank). painters > 1: a chain, with a reader between painters."""
    return dict(profile=profile, model=m, painters=painters)


OPUS = model("anthropic", "claude-opus-5-5", "high", black=True)
READER = ["--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "medium"]

# GPT-6 Luna through the ChatGPT subscription (dev runs), thinking max
LUNA = model("openai-codex", "gpt-6-luna", "max")
GEM2 = model("google", "gemini-3.8-flash", "high", env={"PAINTER_MAX_IMAGES": "8", "PAINTER_INPUT_TPM": "900000"})

LANES = {
    "F": lane("friedrich", OPUS, painters=3),
    "LUNAF": lane("friedrich", LUNA),        # a Friedrich, one painter
    "LUNAB": lane("blank", LUNA),            # a picture of its choosing
    # Gemini through AI Studio: its 2M input tokens a minute for gemini-3.8-flash is shared by the
    # two painters (each paces only itself), so 900K each; 8 images; high is the provider's top
    "GEMF": lane("friedrich", GEM2),
    "GEMB": lane("blank", GEM2),
    # 2026-09-28 night, Friedrich each: Kimi K3 and MiMo v2.6 Pro on OpenCode Go (MiMo keeps 4 images:
    # github.com/XiaomiMiMo/MiMo-Code/issues/2508), Muse Spark 1.3 on OpenCode Zen (its key is a
    # placeholder in auth.json: the opencode-go key is passed), each at its highest thinking level
    "KIMIF": lane("friedrich", model("opencode-go", "kimi-k3", "max")),
    "MIMOF": lane("friedrich", model("opencode-go", "mimo-v2.6-pro", "max", env={"PAINTER_MAX_IMAGES": "4"})),
    "MUSEF": lane("friedrich", model("opencode", "muse-spark-1.3", "xhigh", key_from="opencode-go")),
    # compaction test: Luna (272K window) compacting at about 45K (272K - 227K) instead of 172K;
    # no round 19 sitting came near the usual point (900K for 1M windows)
    # Claude Sonnet 5.5 through the Claude subscription (pi-black, as the Opus lanes), thinking max
    "SONF": lane("friedrich", model("anthropic", "claude-sonnet-5-5", "max", black=True)),
    "SONB": lane("blank", model("anthropic", "claude-sonnet-5-5", "max", black=True)),
    "CTEST": lane("friedrich", model("openai-codex", "gpt-6-luna", "max", env={"PAINTER_COMPACT_RESERVE": "227000"})),
    # Blank studio, one painter each: uncomment the lanes to run.
    # "OPUS": lane("blank", model("anthropic", "claude-opus-5-5", "high", black=True)),
    # "BUN": lane("blank", model("opencode-go", "space-bunny-free", "xhigh")),       # its map goes to max
    # "GLM": lane("blank", model("opencode-go", "glm-5.3-flash", "high")),
    # # Google AI Studio caps gemini-3.8-flash at 2M input tokens a minute: 8 images and 1.5M a minute
    # # (harness/painter pace.ts), as round 18g; provider google's map has xhigh/max null, so high is the top
    # "GEM": lane("blank", model("google", "gemini-3.8-flash", "high",
    #                            env={"PAINTER_MAX_IMAGES": "8", "PAINTER_INPUT_TPM": "1500000"})),
    # # MiMo v2.6 answers about an earlier image once 5+ are in the conversation
    # # (github.com/XiaomiMiMo/MiMo-Code/issues/2508): keep 4
    # "MIMO": lane("blank", model("opencode-go", "mimo-v2.6-pro", "medium", env={"PAINTER_MAX_IMAGES": "4"})),
    # "DSK": lane("blank", model("opencode-go", "deepseek-v4.1-flash", "high")),
}
DRY = False

lock = threading.Lock()
background = []


def log(msg):
    line = f"{time.strftime('%F %T')} {msg}"
    with lock:
        print(line, flush=True)
        if not DRY:
            with open(RUN / "chains.log", "a") as f:
                f.write(line + "\n")


def studio(key):
    """A neutral folder name (nothing about rounds, lanes, models or order); the mapping stays in the run folder."""
    with lock:
        m = RUN / "studios.json"
        names = json.loads(m.read_text()) if m.exists() else {}
        if key not in names:
            names[key] = "paint-studio-" + hashlib.sha1(f"r19{key}{time.time()}".encode()).hexdigest()[:6]
            if not DRY:
                m.write_text(json.dumps(names, indent=1))
        return A / names[key]


def session_dir(d):
    return SESS / ("--" + str(d).strip("/").replace("/", "-") + "--")


def brief(profile, d):
    tpl = (HERE / "brief_template.md").read_text()
    return (tpl.replace("{OPENING}", OPENING[profile]).replace("{STUDIO}", f"~/src/a/{d.name}")
               .replace("{READING}", READING[profile]))


def key_args(m):
    # key_from: a provider whose stored key is a placeholder (opencode Zen) gets another entry's key
    return (["--api-key", f"<{m['key_from']} key from auth.json>" if DRY else api_key(m["key_from"])]
            if m.get("key_from") else [])


def painter_cmd(m, message=PAINTER_MSG, session=None):
    """session: a session file to continue (the painter carries on in it) instead of a new one."""
    cont = ["--session", str(session)] if session else []
    return (["pi", "--print"] + HARNESS + (["-e", str(BLACK)] if m["black"] else []) + m["args"] + key_args(m)
            + cont + [message])


def probe_cmd(m):
    """A one-line request to the painter's provider and model: no tools, no session, no studio."""
    return (["pi", "--print", "--no-session", "--no-extensions", "--no-tools", "--no-context-files", "--no-skills",
             "--no-prompt-templates"] + (["-e", str(BLACK)] if m["black"] else []) + m["args"] + key_args(m)
            + ["Reply with the word ok."])


def count_chunks(d):
    f = d / "paintings/lua/painting.lua"
    if not f.exists():
        return 0
    return sum(1 for line in f.read_text(errors="replace").splitlines() if line.startswith("--@ chunk"))


def count_painting(d):
    """Chunks of the log that put marks on the canvas (see painting_chunks.py)."""
    f = d / "paintings/lua/painting.lua"
    return count_painting_chunks(f.read_text(errors="replace")) if f.exists() else 0


def added_painting(s):
    """Whether a sitting added painting chunks (chunk counts for records without painting counts)."""
    before, after = (("painting_before", "painting_after") if s.get("painting_after") is not None
                     else ("chunks_before", "chunks_after"))
    return s.get(after) is not None and s.get(before) is not None and s[after] > s[before]


def next_step(sittings, max_sittings=MAX_SITTINGS, max_crashes=MAX_CRASHES):
    """What the painter does next: ("new", k) a fresh sitting numbered k, ("continue", k) the
    session of sitting k, cut off by a usage limit, carries on, or (None, why) the painter is done.

    sittings: the records so far, in order ({'sitting', 'painting_before', 'painting_after',
    'chunks_before', 'chunks_after', 'status', 'worked'}); a continued sitting's later parts
    repeat its number and carry its painting_before, so a record's counts cover the whole sitting.
    The painter stops after a sitting it ended itself (status 'completed') that added no painting
    chunks (this includes a first sitting that painted nothing), or after max_sittings sittings:
    completed ones, and crashed ones that added painting (cut off after it painted, not judged).
    Sittings cut off by a usage limit ('limited') or the runner stopping ('interrupted') count for
    nothing; if the painter had worked in one (a reply that wasn't an error: 'worked'), its session
    carries on (a limited one once the limit lifts); otherwise a fresh sitting follows. After
    max_crashes crashes the painter stops (not finished).
    A record without painting counts (from an older runner) is judged by its chunk counts.
    """
    if not sittings:
        return ("new", 1)
    done = [s for s in sittings if s.get("status") == "completed"]
    if done and not added_painting(done[-1]):
        return (None, f"the painter is done: sitting {done[-1]['sitting']} added no painting")
    counted = {s["sitting"] for s in sittings if s.get("status") == "completed"
               or (s.get("status") == "crashed" and added_painting(s))}
    if max_sittings and len(counted) >= max_sittings:
        return (None, f"stopped after {max_sittings} sittings (MAX_SITTINGS)")
    if sum(1 for s in sittings if s.get("status") == "crashed") >= max_crashes:
        return (None, f"NOT FINISHED: {max_crashes} crashes (MAX_CRASHES)")
    last = sittings[-1]
    if last.get("status") in ("limited", "interrupted") and last.get("worked") and last.get("sessions"):
        return ("continue", last["sitting"])
    return ("new", max(s["sitting"] for s in sittings) + 1)


def session_worked(files):
    """Whether any of these pi session files holds an assistant reply that wasn't an error."""
    for f in files:
        try:
            for line in open(f, errors="replace"):
                if "assistant" not in line:
                    continue
                m = json.loads(line).get("message") or {}
                if m.get("role") == "assistant" and m.get("stopReason") not in ("error", "aborted"):
                    return True
        except (OSError, ValueError):
            continue
    return False


def session_error(files):
    """The provider error the last of these pi session files ended on (its last assistant message
    has stopReason 'error'), or None."""
    for f in reversed(files):
        try:
            with open(f, "rb") as fh:
                fh.seek(0, 2)
                fh.seek(max(0, fh.tell() - 4_000_000))
                lines = fh.read().decode(errors="replace").splitlines()
        except OSError:
            continue
        for line in reversed(lines):
            try:
                m = json.loads(line).get("message") or {}
            except ValueError:
                continue
            if m.get("role") == "assistant":
                return (m.get("errorMessage") or "unknown provider error") if m.get("stopReason") == "error" else None
    return None


def retry_hint(text):
    """The longest "retry in Ns" (or "retryDelay": "Ns") a provider error asks for, in s (0 if none)."""
    hints = re.findall(r"retry in ([\d.]+)\s*s", text, re.I) + re.findall(r'retryDelay\W+([\d.]+)s', text)
    return max((float(h) for h in hints), default=0.0)


def usage_limit(text):
    """Whether a provider error is a usage limit that resets with time (OpenCode Go's 5-hour,
    weekly and monthly windows), not a crash. An empty balance (Google's 402 "credits are
    depleted") is not: waiting doesn't refill it."""
    return bool(re.search(r"GoUsageLimitError|FreeUsageLimitError|usage limit (exceeded|reached)", text, re.I))


def limit_reset_s(text):
    """Seconds until a usage limit resets, from "Resets in 2hr 15min" (or "Resets in 2 days"), or None."""
    m = re.search(r"resets? in ((?:\s*[\d.]+\s*(?:days?|d|hours?|hrs?|h|minutes?|mins?|m|seconds?|secs?|s)\b)+)", text, re.I)
    if not m:
        return None
    unit = {"d": 86400, "h": 3600, "m": 60, "s": 1}
    return sum(float(v) * unit[u[0].lower()] for v, u in re.findall(r"([\d.]+)\s*([a-z]+)", m.group(1), re.I))


def first_probe_wait(text, ended, now):
    """Seconds before the first probe after a sitting that ended on a usage limit at `ended`
    (epoch s): the reset time the error named (or LIMIT_PROBE_S), less the time since then (a
    resumed runner asks at once when that has passed)."""
    reset = limit_reset_s(text)
    return max(0.0, (reset + 60 if reset else LIMIT_PROBE_S) - (now - ended))


def wait_out_limit(m, tag, text, ended):
    """Wait until the provider answers again (probing it), or give up after LIMIT_GIVE_UP_H hours.
    True when the painter can go on."""
    t0 = time.time()
    wait = first_probe_wait(text, ended, t0)
    while True:
        if time.time() + wait - t0 > LIMIT_GIVE_UP_H * 3600:
            log(f"{tag}: the usage limit hasn't lifted in {LIMIT_GIVE_UP_H} h; painter stops (rerun to go on)")
            return False
        log(f"{tag}: usage limit; asking the provider again in {wait / 60:.0f} min (not a crash)")
        time.sleep(wait)
        (RUN / "probe").mkdir(exist_ok=True)
        try:
            r = subprocess.run(probe_cmd(m), cwd=RUN / "probe", stdin=subprocess.DEVNULL,
                               capture_output=True, text=True, timeout=600)
            out = r.stdout + r.stderr
        except subprocess.TimeoutExpired:
            out = "probe timed out"
        if not usage_limit(out):
            log(f"{tag}: the provider answers again after {(time.time() - t0) / 3600:.1f} h: {gist(out, 80)}")
            return True
        reset = limit_reset_s(out)
        wait = reset + 60 if reset else LIMIT_PROBE_S


def crash_wait(n_crashes, text):
    """Seconds to wait after the n-th crash: CRASH_WAITS, at least the provider's retry hint plus 15 s."""
    return max(CRASH_WAITS[min(n_crashes, len(CRASH_WAITS)) - 1], retry_hint(text) + 15)


def gist(text, n=240):
    """One line of an error for the log."""
    t = " ".join(text.split())
    code = re.search(r'"code"\W+(\d{3})', text)
    quota = re.search(r'quotaId\W+(\w+)', text)
    head = " ".join(x for x in (code and f"HTTP {code.group(1)}", quota and quota.group(1)) if x)
    return (head + ": " if head else "") + (t[:n] + ("..." if len(t) > n else ""))


def load_sittings(rd, n):
    f = rd / f"p{n}_sittings.json"
    return json.loads(f.read_text()) if f.exists() else []


def save_sittings(rd, n, sittings):
    f = rd / f"p{n}_sittings.json"
    tmp = f.with_suffix(".json.tmp")
    tmp.write_text(json.dumps(sittings, indent=1))
    tmp.replace(f)


def close_easel(d):
    """End the painter's easel session cleanly if one is still up (the log stays), then stop leftovers."""
    if (d / "bin/easel").exists():
        subprocess.run(["timeout", "60", str(d / "bin/easel"), "close"], cwd=d,
                       stdin=subprocess.DEVNULL, capture_output=True)
    stop_leftovers(d)


def open_easel(d, tag):
    """Replay the painting before Pi starts, with progress in the runner log, so the sitting message
    ("the painting is as you left it") is true when the painter reads it."""
    p = subprocess.Popen([str(d / "bin/easel"), "open"], cwd=d, stdin=subprocess.DEVNULL,
                         stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    for line in p.stdout:
        if not line.startswith("resuming "):
            log(f"{tag}: {line.rstrip()}")
    rc = p.wait()
    if rc:
        log(f"{tag}: EASEL OPEN FAILED (exit {rc})")
    return rc == 0


def paint(name, n, d, rd):
    """Run the painter's sittings until next_step says stop; resumable from p<n>_sittings.json."""
    m = LANES[name]["model"]
    tag = f"{name}{n}"
    sittings = load_sittings(rd, n)
    for s in sittings:
        if s.get("status") == "running":      # the runner stopped during this sitting
            t0 = time.mktime(time.strptime(s["start"], "%Y-%m-%d %H:%M:%S")) - 5
            files = s.get("sessions") or sorted((f for f in session_dir(d).glob("*.jsonl") if f.stat().st_mtime >= t0),
                                                key=lambda f: f.stat().st_mtime)[-1:]
            s.update(status="interrupted", chunks_after=count_chunks(d), painting_after=count_painting(d),
                     end=time.strftime("%F %T"), sessions=[str(f) for f in files], worked=session_worked(files))
            log(f"{tag}: sitting {s['sitting']} was interrupted (the runner stopped); "
                + ("it carries on in its session" if s["worked"] else "it will be taken again"))
    for s in sittings:
        if s.get("status") == "completed" and s.get("exit") not in (0, None):   # an older runner judged a crash
            s.update(status="crashed", reclassified=time.strftime("%F %T"))
            log(f"{tag}: sitting {s['sitting']} exited {s['exit']}: counted as a crash, not a judgment")
    for s in sittings:
        if s.get("status") == "crashed" and usage_limit(s.get("error") or ""):   # an older runner counted it
            s.update(status="limited", reclassified=time.strftime("%F %T"))
            log(f"{tag}: sitting {s['sitting']} ended on a usage limit: not a crash")
    save_sittings(rd, n, sittings)
    for s in sittings:
        if s.get("status") == "limited" and "worked" not in s:      # an older runner didn't record it
            s["worked"] = session_worked(s.get("sessions") or [])
    save_sittings(rd, n, sittings)
    while True:
        step, k = next_step(sittings)
        if step is None:
            why_done = k
            log(f"{tag}: {why_done}")
            break
        if sittings and sittings[-1].get("status") == "limited":
            last = sittings[-1]
            ended = time.mktime(time.strptime(last["end"], "%Y-%m-%d %H:%M:%S")) if last.get("end") else time.time()
            if not wait_out_limit(m, tag, last.get("error") or "", ended):
                return None
        # one session across sittings: reattaches at once; replays the log only if the server is gone
        stop_leftovers(d, keep_server=True)
        if not open_easel(d, tag):
            log(f"{tag}: painter stops (the easel didn't open); rerun to retry")
            return None
        before = count_chunks(d)
        painting_before = count_painting(d)
        cont = sittings[-1] if step == "continue" else None
        if cont:
            # the same sitting carries on in its session: its counts start where the sitting started
            part = cont.get("part", 1) + 1
            msg, session = CONTINUE_MESSAGE, cont["sessions"][-1]
            first = dict(chunks_before=cont["chunks_before"], painting_before=cont["painting_before"])
        else:
            part, session = 1, None
            msg = PAINTER_MSG if before == 0 else SITTING_MESSAGE
            first = dict(chunks_before=before, painting_before=painting_before)
        stem = f"p{n}_s{k}" + (f"_part{part}" if part > 1 else "")
        known = set(session_dir(d).glob("*.jsonl"))
        rec = dict(sitting=k, part=part, status="running", start=time.strftime("%F %T"), end=None,
                   chunks_after=None, painting_after=None, **first,
                   message=msg, sessions=[], exit=None, final_file=str(rd / f"{stem}_final.txt"), final=None)
        sittings.append(rec)
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k}{f' continues (part {part}, in its session)' if cont else ''} ({m['name']}, painter harness, "
            f"{before} chunks on the easel, {painting_before} painting)")
        t0 = time.time()
        rc = run(painter_cmd(m, msg, session), d, rd / f"{stem}_final.txt", rd / f"{stem}_err.txt", env=m["env"])
        stop_leftovers(d, keep_server=True)
        new = sorted(set(session_dir(d).glob("*.jsonl")) - known, key=lambda f: f.stat().st_mtime)
        if session:
            new = [Path(session)] + new
        api_error = session_error(new)
        err_text = (rd / f"{stem}_err.txt").read_text(errors="replace")
        crashed = rc != 0 or api_error is not None
        why = api_error or err_text.strip() or f"exit {rc}"
        limited = crashed and usage_limit(why + "\n" + err_text)
        rec.update(status="limited" if limited else "crashed" if crashed else "completed", end=time.strftime("%F %T"),
                   chunks_after=count_chunks(d), painting_after=count_painting(d), exit=rc,
                   sessions=[str(f) for f in new], error=why[-4000:] if crashed else None,
                   worked=session_worked(new), final=(rd / f"{stem}_final.txt").read_text(errors="replace"))
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k} ended (exit {rc}{', USAGE LIMIT' if limited else ', CRASHED' if crashed else ''}) after {(time.time() - t0) / 60:.0f} min: "
            f"chunks {rec['chunks_before']} -> {rec['chunks_after']} "
            f"(painting {rec['painting_before']} -> {rec['painting_after']}), {len(new)} session file(s)")
        if limited:
            log(f"{tag}: sitting {k} ended on a usage limit ({gist(why)}): not a crash; the painter waits it out")
        elif crashed:
            crashes = sum(1 for s in sittings if s.get("status") == "crashed")
            if crashes >= MAX_CRASHES:
                log(f"{tag}: last error: {gist(why)}")
            wait = crash_wait(crashes, why + "\n" + err_text)
            if crashes < MAX_CRASHES:
                log(f"{tag}: sitting {k} crashed ({gist(why)}); crash {crashes} of at most {MAX_CRASHES}, "
                    f"not a judgment: another sitting in {wait:.0f} s")
                time.sleep(wait)
    close_easel(d)
    shutil.copy(sittings[-1]["final_file"], rd / f"p{n}_final.txt")
    (rd / f"p{n}.painted").write_text(f"{time.strftime('%F %T')} {len(sittings)} record(s): {why_done}\n")
    return sittings


def reader_cmd(prompt):
    """Round 16's reader launch: the machine's own pi setup, cwd the run folder (no painter harness)."""
    return ["pi", "--print", "--no-context-files", "--no-skills", "--no-prompt-templates"] + READER + [prompt]


def export_cmd(profile, d):
    return [str(EXPORT), profile, str(d)]


def finish_cmd(d, name, n):
    return [str(FINISH), str(d / "paintings/lua/painting.lua"), str(RUN / f"{name}{n}_finished.png")]


def check_cmd(d, name, n):
    return ["nice", str(CHECK), str(d), str(RUN / name / f"p{n}_check")]


def run(cmd, cwd, out, err, env=None):
    with open(out, "w") as o, open(err, "w") as e:
        return subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=o, stderr=e,
                              env={**os.environ, **env} if env else None).returncode


def show(tag, what, cmd, cwd, env=None):
    pre = " ".join(f"{k}={shlex.quote(v)}" for k, v in (env or {}).items())
    log(f"{tag}: {what}:\n    cd {shlex.quote(str(cwd))} && {pre + ' ' if pre else ''}{shlex.join(cmd)}")


def clear_settings(d):
    """No settings file in the studio (compaction.ts sets the thresholds); remove one if present."""
    f = d / ".pi" / "settings.json"
    if f.exists():
        f.unlink()
    if (d / ".pi").is_dir() and not any((d / ".pi").iterdir()):
        (d / ".pi").rmdir()


def in_background(fn):
    th = threading.Thread(target=fn)
    th.start()
    background.append(th)


def finish(name, n, d):
    """Varnish and crack a replay of the painter's log, in the background; the painter's save stays as is."""
    rd = RUN / name
    tag = f"{name}{n}"
    if (rd / f"p{n}.finished").exists():
        return
    lua = d / "paintings/lua/painting.lua"
    if not lua.exists() or not lua.stat().st_size:
        log(f"{tag}: no log to finish at {lua}")
        return

    def go():
        log(f"{tag}: finishing in the background -> {RUN / f'{tag}_finished.png'}")
        rc = run(finish_cmd(d, name, n), RUN, rd / f"p{n}_finish.log", rd / f"p{n}_finish_err.txt")
        if rc == 0:
            (rd / f"p{n}.finished").write_text(time.strftime("%F %T"))
            log(f"{tag}: finished painting written")
        else:
            log(f"{tag}: FINISHING FAILED (exit {rc}, see {rd}/p{n}_finish_err.txt)")

    in_background(go)


def check_result(text):
    """The "check:" line scripts/check_painting ends its report with, or a note that there is none."""
    lines = [l for l in text.splitlines() if l.startswith("check:")]
    return lines[-1] if lines else "check: no result line (see the log)"


def check(name, n, d):
    """Replay the painter's log with the replay build and compare (scripts/check_painting), in the
    background after the painter's last sitting; the result goes to the runner's log."""
    rd = RUN / name
    tag = f"{name}{n}"
    if (rd / f"p{n}.checked").exists():
        return
    lua = d / "paintings/lua/painting.lua"
    if not lua.exists() or not lua.stat().st_size:
        log(f"{tag}: no log to check at {lua}")
        return

    def go():
        work = rd / f"p{n}_check"
        if work.exists():              # an earlier check was cut off
            shutil.rmtree(work)
        log(f"{tag}: checking the log in the background (replay build) -> {rd}/p{n}_check.log")
        t0 = time.time()
        rc = run(check_cmd(d, name, n), RUN, rd / f"p{n}_check.log", rd / f"p{n}_check_err.txt")
        result = check_result((rd / f"p{n}_check.log").read_text(errors="replace"))
        (rd / f"p{n}.checked").write_text(f"{time.strftime('%F %T')} exit {rc}\n{result}\n")
        log(f"{tag}: {'' if rc == 0 else 'CHECK NOT OK: '}{result} (exit {rc}, {(time.time() - t0) / 60:.0f} min)")

    in_background(go)


def stop_leftovers(d, keep_server=False):
    """Stop easel processes of this studio (not the runner's check and finishing; not the
    server with keep_server: the session stays open across sittings)."""
    r = subprocess.run(["/bin/ps", "-Ao", "pid=,command="], capture_output=True, text=True).stdout
    for line in r.splitlines():
        pid, _, cmd = line.strip().partition(" ")
        if keep_server and " serve " in f" {cmd} ":
            continue
        if str(d) in cmd and "easel" in cmd and "finish_painting" not in cmd and "check_painting" not in cmd:
            subprocess.run(["kill", pid])
            log(f"stopped leftover {pid}: {cmd[:100]}")


def our_studios():
    m = RUN / "studios.json"
    return list(json.loads(m.read_text()).items()) if m.exists() else []


def monitor_histories():
    """Each painting's log may only grow: record it, and alert if a history got shorter or changed."""
    mon = RUN / "monitor"
    mon.mkdir(exist_ok=True)
    for key, name in our_studios():
        f = A / name / "paintings" / "lua" / "painting.lua"
        if not f.exists():
            continue
        now = f.read_bytes()
        last = mon / f"{key}.last"
        if last.exists():
            before = last.read_bytes()
            if not now.startswith(before):
                (mon / f"{key}.before-{int(time.time())}").write_bytes(before)
                log(f"ALERT {key} ({name}): the painting's history got shorter or changed ({len(before)} -> {len(now)} bytes)")
        last.write_bytes(now)


def etime_seconds(etime):
    """Seconds from ps's etime ([[dd-]hh:]mm:ss), or None. (macOS ps has no `etimes`: rounds 17
    and 18 asked for it, got a PID and COMMAND listing, and their watchdog skipped every line.)"""
    m = re.fullmatch(r"(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)", etime.strip())
    if not m:
        return None
    d, h, mi, s = (int(x) if x else 0 for x in m.groups())
    return ((d * 24 + h) * 60 + mi) * 60 + s


def spared(cmd):
    """Processes in a studio the watchdog leaves alone: the painter's pi (it titles itself "pi"),
    the easel server (it lives for the whole session), `easel open` (it replays the log, tens of
    minutes for a big painting, and gives up by itself when the replay stalls) and the runner's own
    check and finishing."""
    exe = os.path.basename(cmd.split()[0]) if cmd.split() else ""
    return (exe == "pi" or " serve " in f" {cmd} " or "finish_painting" in cmd or "check_painting" in cmd
            or cmd.rstrip().endswith("easel open"))    # a replay; `open` stops itself after 30 min without progress


def overdue(ps_lines, cwds, studios, limit_s, me=None):
    """The (pid, seconds, cwd, command) of processes to stop: running longer than limit_s, working
    in one of `studios` (cwd in it, or its path in the command line) and not spared.
    ps_lines: `ps -Ao pid=,etime=,command=` lines; cwds: pid -> cwd."""
    out = []
    for line in ps_lines:
        parts = line.split(None, 2)
        if len(parts) < 3 or not parts[0].isdigit():
            continue
        pid, etime, cmd = parts
        secs = etime_seconds(etime)
        if secs is None or secs < limit_s or pid == me or spared(cmd):
            continue
        where = cwds.get(pid, "")
        if any(where == s or where.startswith(s + "/") or s + "/" in cmd or cmd.endswith(s) for s in studios):
            out.append((pid, secs, where, cmd))
    return out


def cwds_of_all():
    """pid -> cwd of every process, from one lsof call."""
    r = subprocess.run(["lsof", "-a", "-d", "cwd", "-Fpn"], capture_output=True, text=True).stdout
    out, pid = {}, None
    for l in r.splitlines():
        if l.startswith("p"):
            pid = l[1:]
        elif l.startswith("n") and pid:
            out[pid] = l[1:]
    return out


def watchdog(stop):
    """Stop processes working in one of this round's studios that run too long; watch the logs grow."""
    while not stop.is_set():
        try:
            monitor_histories()
        except Exception as e:
            log(f"monitor error: {e}")
        try:
            mine = [str(A / name) for _, name in our_studios()]
            ps = subprocess.run(["/bin/ps", "-Ao", "pid=,etime=,command="], capture_output=True, text=True).stdout.splitlines()
            for pid, secs, where, cmd in overdue(ps, cwds_of_all() if mine else {}, mine, WATCHDOG_MIN * 60, str(os.getpid())):
                subprocess.run(["kill", pid])
                log(f"WATCHDOG stopped {pid} after {secs // 60} min in {where}: {cmd[:140]}")
        except Exception as e:
            log(f"watchdog error: {e}")
        stop.wait(60)


def chain(name):
    t = LANES[name]
    rd = RUN / name
    if not DRY:
        rd.mkdir(parents=True, exist_ok=True)
    env = {"R16_BRANCH": BRANCH}
    for n in range(1, t["painters"] + 1):
        d = studio(f"{name}{n}")
        tag = f"{name}{n}"
        if (rd / f"p{n}.done").exists():
            log(f"{tag}: done already")
            if not DRY:
                check(name, n, d)
                finish(name, n, d)
            continue
        if DRY:
            extra = " + trees.md" if t["profile"] == "friedrich" else ""
            show(tag, f"export {t['profile']} studio (then studio_notes.md{' + records' if n > 1 else ''}{extra}, "
                      f"BRIEF.md as briefs/{t['profile']}.md with this studio's path)", export_cmd(t["profile"], d), BASE, env)
            show(tag, f"open the easel ({d / 'bin/easel'} open); it stays open across sittings", [str(d / "bin/easel"), "open"], d)
            show(tag, "sitting 1 (painter)", painter_cmd(t["model"]), d, t["model"]["env"])
            show(tag, f"after each sitting: count painting chunks;\n"
                      f"    more sittings until one the painter ends adds no painting chunks (safety cap {MAX_SITTINGS}; a usage limit continues its session),\n"
                      f"    at the same open easel (reopened, replaying the log, only if its server is gone); closed after the last",
                 painter_cmd(t["model"], SITTING_MESSAGE), d, t["model"]["env"])
            show(tag, "check (background, after the last sitting; result in the log)", check_cmd(d, name, n), RUN)
            show(tag, "finishing (background, after the last sitting)", finish_cmd(d, name, n), RUN)
            if n < t["painters"]:
                show(tag, "reader", reader_cmd(f"Read {rd}/p{n}_reader_brief.md and do what it says."), rd)
            continue
        if not (rd / f"p{n}.exported").exists():
            if d.exists():
                shutil.rmtree(d)
            log(f"{tag}: exporting {t['profile']} studio from {BRANCH} to {d}")
            r = subprocess.run(export_cmd(t["profile"], d), capture_output=True, text=True, env={**os.environ, **env})
            (rd / f"p{n}_export.log").write_text(r.stdout + r.stderr)
            if r.returncode:
                log(f"{tag}: EXPORT FAILED, lane stops (see {rd}/p{n}_export.log)")
                return
            notes = [(HERE / "studio_notes.md").read_text()]
            for k in range(1, n):
                rec = rd / f"p{k}_record.md"
                if rec.exists():
                    notes.append("\n## More notes from the studio\n\n" + rec.read_text())
            (d / "notes" / "studio_notes.md").write_text("\n".join(notes))
            if t["profile"] == "friedrich":
                shutil.copy(HERE / "trees.md", d / "notes" / "research" / "trees.md")
            (rd / f"p{n}.exported").write_text(time.strftime("%F %T"))
        (rd / f"p{n}_brief.md").write_text(brief(t["profile"], d))
        (d / "BRIEF.md").write_text(brief(t["profile"], d))
        clear_settings(d)
        if not (rd / f"p{n}.painted").exists():
            if paint(name, n, d, rd) is None:
                log(f"{name}: chain stops")
                return
        check(name, n, d)
        finish(name, n, d)
        logs = [p for s in load_sittings(rd, n) for p in s.get("sessions", []) if Path(p).exists()]
        if logs and n < t["painters"]:
            out = rd / f"p{n}_record.md"
            rb = ((HERE / "reader_brief.md").read_text().replace("{LOG}", ", ".join(logs))
                  .replace("{JOURNAL}", str(d / "notes/journal.md")).replace("{OUT}", str(out)))
            (rd / f"p{n}_reader_brief.md").write_text(rb)
            log(f"{tag}: reader")
            run(reader_cmd(f"Read {rd}/p{n}_reader_brief.md and do what it says."), rd,
                rd / f"p{n}_reader_final.txt", rd / f"p{n}_reader_err.txt")
            log(f"{tag}: record {'written' if out.exists() else 'MISSING'}")
        (rd / f"p{n}.done").write_text(time.strftime("%F %T"))
    log(f"lane {name} finished")


def render_briefs(out):
    """Every profile's brief, with a placeholder studio (paint-studio-000000) for the path and scratch name."""
    out.mkdir(parents=True, exist_ok=True)
    d = A / "paint-studio-000000"
    for profile in OPENING:
        (out / f"{profile}.md").write_text(brief(profile, d))
        print(out / f"{profile}.md")


def main():
    global DRY
    ap = argparse.ArgumentParser()
    ap.add_argument("--only")
    ap.add_argument("--dry", action="store_true")
    ap.add_argument("--briefs", type=Path)
    a = ap.parse_args()
    if a.briefs:
        render_briefs(a.briefs)
        return
    DRY = a.dry
    lanes = a.only.split(",") if a.only else list(LANES)
    for l in lanes:
        if l not in LANES:
            raise SystemExit(f"no lane {l}; lanes: {', '.join(LANES)}")
    if not DRY:
        RUN.mkdir(parents=True, exist_ok=True)
    stop = threading.Event()
    if not DRY:
        threading.Thread(target=watchdog, args=(stop,), daemon=True).start()
    th = [threading.Thread(target=chain, args=(l,)) for l in lanes]
    for x in th:
        x.start()
        if DRY:
            x.join()          # one lane after the other, so the printout reads in order
        else:
            time.sleep(5)
    for x in th:
        x.join()
    if background:
        log("waiting for checks and finishing")
    for x in background:
        x.join()
    stop.set()
    log("all lanes finished")


if __name__ == "__main__":
    main()
