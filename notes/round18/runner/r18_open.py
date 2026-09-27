# /// script
# requires-python = ">=3.11"
# ///
"""Round 18: a blank slate. One painter per model, five non-Claude models, run in parallel.

Every lane gets round 17's lane O brief (blank studio, "a picture of your choosing"),
with its own studio path and scratch name. No chains and no reader: each painter
has an empty studio with only the studio notes.

Lanes (see LANES): MIMO, GLM, DSK, BUN (opencode-go) and MUSE (opencode, Zen;
pi's stored `opencode` key is a placeholder, so the runner reads the key stored
under "opencode-go" in ~/.pi/agent/auth.json at run time and passes it with
--api-key; it is never printed or written anywhere).

Sittings: each painter works in up to MAX_SITTINGS sittings, each a new pi
session (same harness, studio and settings). Sitting 1 is the plain launch;
later ones get SITTING_MESSAGE. After each sitting the easel is closed and
leftovers stopped, and the chunks of the painting's log that put marks on
the canvas are counted (painting_chunks.py: a chunk that only queries, waits,
mixes or loads doesn't count). The painter stops after a sitting that added no
such chunks, or after sitting MAX_SITTINGS (see next_sitting). Each sitting is
recorded in run/<lane>/p1_sittings.json.

Each painter gets a fresh blank studio exported from branch r17-base, with
studio_notes.md as notes/studio_notes.md. Painters run in the clean harness
(claude-paint-r17-base/harness/painter): no global extensions, our system
prompt, bash and read only, our compaction (its thresholds set by
compaction.ts; nothing of ours in the studio's .pi/, which --no-approve
ignores), a per-studio TMPDIR (painter.ts). No pi-black (Anthropic only).
After a painter's last sitting, its leftover easel processes are stopped and
finishing (varnish and cracks) starts in the background on a replay of its log
(run/<lane>_finished.png; the painter's own save is untouched).

Resumable: every finished step leaves a marker in the run folder; a rerun
skips it (and starts any finishing that didn't finish). A watchdog stops any
process started in one of this round's studios that runs longer than
WATCHDOG_MIN minutes, and logs it; it also watches that each painting's log
only grows.

    uv run r18_open.py                  # all five lanes
    uv run r18_open.py --only GLM,DSK   # some
    uv run r18_open.py --dry            # print the exact commands (key redacted), run nothing
"""
import argparse
import hashlib
import json
import os
import shlex
import shutil
import subprocess
import threading
import time
from pathlib import Path

from painting_chunks import count_painting_chunks

HOME = Path.home()
A = HOME / "src/a"
BASE = A / "claude-paint-r17-base"
BRANCH = "r17-base"
EXPORT = BASE / "scripts/export_r16_studio"      # honors R16_BRANCH
FINISH = BASE / "scripts/finish_painting"
H = BASE / "harness/painter"
TEMP_GUARD = HOME / ".pi/agent/extensions/persistent-temp.ts"
HERE = Path(__file__).resolve().parent
RUN = HERE / "run"
SESS = HOME / ".pi/agent/sessions"
AUTH = HOME / ".pi/agent/auth.json"
WATCHDOG_MIN = 30
EASEL_HOWTO = os.environ.get("EASEL_HOWTO", "The easel is `bin/easel` in your studio;\n  notes/easel_guide.md has its commands (`open`, `do`, `look`, `note`, `save`,\n  `close`).")
EASEL_SAVE = os.environ.get("EASEL_SAVE", "`bin/easel save`")

HARNESS = ["--no-extensions", "-e", str(H / "painter.ts"), "-e", str(H / "compaction.ts"),
           "-e", str(TEMP_GUARD),
           "--system-prompt", str(H / "system_prompt.md"), "--tools", "bash,read",
           "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve"]
PAINTER_MSG = ("Complete your task autonomously. Read BRIEF.md in this folder and follow it exactly. "
               "That file is your whole brief. Your FINAL message is the reply it asks for.")
SITTING_MESSAGE = ("You're back in your studio. The painting is on the easel as you left it: "
                   "`bin/easel open` picks it up where you stopped. Your brief is in BRIEF.md and "
                   "your journal in notes/journal.md. Your FINAL message is the reply the brief asks for.")
MAX_SITTINGS = 4

BLANK_READING = "notes/easel_guide.md; notes/studio_notes.md;\nnotes/research/oil_paint_physics.md as needed."
OPENING = ("Paint one picture of your choosing in oil, at the easel of claude-paint, a\n"
           "simulator of oil paint on linen. Subject, composition and manner are yours.\n"
           "Work from what you know; don't use reference images or image models.")

KEY_FROM = "opencode-go"      # the auth.json entry whose key MUSE passes with --api-key
REDACTED = "<opencode-go key from auth.json>"


def model(provider, name, thinking, key=False):
    return dict(args=["--provider", provider, "--model", name, "--thinking", thinking], key=key, name=name)


LANES = {
    "MIMO": model("opencode-go", "mimo-v2.6-pro", "medium"),
    "GLM": model("opencode-go", "glm-5.3-flash", "high"),
    "DSK": model("opencode-go", "deepseek-v4.1-flash", "high"),
    "BUN": model("opencode-go", "space-bunny-free", "xhigh"),      # its map goes to max
    "MUSE": model("opencode", "muse-spark-1.3", "high", key=True),  # its map goes to xhigh (max: null)
}
DRY = False

lock = threading.Lock()
finishers = []


def log(msg):
    line = f"{time.strftime('%F %T')} {msg}"
    with lock:
        print(line, flush=True)
        if not DRY:
            with open(RUN / "chains.log", "a") as f:
                f.write(line + "\n")


def api_key():
    return json.loads(AUTH.read_text())[KEY_FROM]["key"]


def model_args(lane, redact=False):
    m = LANES[lane]
    return m["args"] + (["--api-key", REDACTED if redact else api_key()] if m["key"] else [])


def studio(lane):
    """A neutral folder name (nothing about rounds, lanes or models); the mapping stays in the run folder."""
    with lock:
        m = RUN / "studios.json"
        names = json.loads(m.read_text()) if m.exists() else {}
        if lane not in names:
            names[lane] = "paint-studio-" + hashlib.sha1(f"r18{lane}{time.time()}".encode()).hexdigest()[:6]
            if not DRY:
                m.write_text(json.dumps(names, indent=1))
        return A / names[lane]


def session_dir(d):
    return SESS / ("--" + str(d).strip("/").replace("/", "-") + "--")


def brief(d):
    tpl = (HERE / "brief_template.md").read_text()
    return (tpl.replace("{OPENING}", OPENING).replace("{STUDIO}", f"~/src/a/{d.name}")
               .replace("{EASEL_HOWTO}", EASEL_HOWTO).replace("{EASEL_SAVE}", EASEL_SAVE).replace("{SLUG}", d.name.replace("paint-", ""))
               .replace("{READING}", BLANK_READING).replace("{DETAIL}", ""))


def painter_cmd(lane, message=PAINTER_MSG, redact=False):
    return ["pi", "--print"] + HARNESS + model_args(lane, redact) + [message]


def count_chunks(d):
    f = d / "paintings/lua/painting.lua"
    if not f.exists():
        return 0
    return sum(1 for line in f.read_text(errors="replace").splitlines() if line.startswith("--@ chunk"))


def count_painting(d):
    """Chunks of the log that put marks on the canvas (see painting_chunks.py)."""
    f = d / "paintings/lua/painting.lua"
    return count_painting_chunks(f.read_text(errors="replace")) if f.exists() else 0


def next_sitting(sittings, max_sittings=MAX_SITTINGS):
    """The number of the next sitting, or None when the painter is done.

    sittings: the records so far, in order ({'sitting', 'painting_before', 'painting_after',
    'chunks_before', 'chunks_after', 'status'}).
    A sitting cut off by the runner itself stopping (status 'interrupted') is taken again.
    Stop after a completed sitting that added no painting chunks (this includes a first sitting
    that painted nothing: no painting to come back to), or after sitting max_sittings. A record
    without painting counts (from an older runner) is judged by its chunk counts.
    """
    done = [s for s in sittings if s.get("status") == "completed"]
    if not done:
        return 1
    last = done[-1]
    before, after = (("painting_before", "painting_after") if last.get("painting_after") is not None
                     else ("chunks_before", "chunks_after"))
    if last[after] <= last[before]:
        return None
    if last["sitting"] >= max_sittings:
        return None
    return last["sitting"] + 1


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


def paint(lane, d, rd, n=1):
    """Run the painter's sittings until next_sitting says stop; resumable from p1_sittings.json."""
    tag = lane
    sittings = load_sittings(rd, n)
    for s in sittings:
        if s.get("status") == "running":      # the runner stopped during this sitting
            s.update(status="interrupted", chunks_after=count_chunks(d), painting_after=count_painting(d))
            log(f"{tag}: sitting {s['sitting']} was interrupted; it will be taken again")
    save_sittings(rd, n, sittings)
    while (k := next_sitting(sittings)) is not None:
        close_easel(d)
        before = count_chunks(d)
        painting_before = count_painting(d)
        msg = PAINTER_MSG if before == 0 else SITTING_MESSAGE
        known = set(session_dir(d).glob("*.jsonl"))
        rec = dict(sitting=k, status="running", start=time.strftime("%F %T"), end=None,
                   chunks_before=before, chunks_after=None, painting_before=painting_before, painting_after=None, message=msg, sessions=[], exit=None,
                   final_file=str(rd / f"p{n}_s{k}_final.txt"), final=None)
        sittings.append(rec)
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k} ({LANES[lane]['name']}, painter harness, {before} chunks on the easel, {painting_before} painting)")
        t0 = time.time()
        rc = run(painter_cmd(lane, msg), d, rd / f"p{n}_s{k}_final.txt", rd / f"p{n}_s{k}_err.txt")
        close_easel(d)
        new = sorted(set(session_dir(d).glob("*.jsonl")) - known, key=lambda f: f.stat().st_mtime)
        rec.update(status="completed", end=time.strftime("%F %T"), chunks_after=count_chunks(d),
                   painting_after=count_painting(d), exit=rc,
                   sessions=[str(f) for f in new],
                   final=(rd / f"p{n}_s{k}_final.txt").read_text(errors="replace"))
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k} ended (exit {rc}) after {(time.time() - t0) / 60:.0f} min: "
            f"chunks {rec['chunks_before']} -> {rec['chunks_after']} "
            f"(painting {rec['painting_before']} -> {rec['painting_after']}), {len(new)} session file(s)")
    shutil.copy(rd / f"p{n}_s{sittings[-1]['sitting']}_final.txt", rd / f"p{n}_final.txt")
    (rd / f"p{n}.painted").write_text(f"{time.strftime('%F %T')} {len(sittings)} sitting(s)")
    return sittings


def export_cmd(d):
    return [str(EXPORT), "blank", str(d)]


def finish_cmd(d, lane):
    return [str(FINISH), str(d / "paintings/lua/painting.lua"), str(RUN / f"{lane}_finished.png")]


def run(cmd, cwd, out, err):
    with open(out, "w") as o, open(err, "w") as e:
        return subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=o, stderr=e).returncode


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


def finish(lane, d, n=1):
    """Varnish and crack a replay of the painter's log, in the background; the painter's save stays as is."""
    rd = RUN / lane
    tag = lane
    if (rd / f"p{n}.finished").exists():
        return
    lua = d / "paintings/lua/painting.lua"
    if not lua.exists() or not lua.stat().st_size:
        log(f"{tag}: no log to finish at {lua}")
        return

    def go():
        log(f"{tag}: finishing in the background -> {RUN / f'{lane}_finished.png'}")
        rc = run(finish_cmd(d, lane), RUN, rd / f"p{n}_finish.log", rd / f"p{n}_finish_err.txt")
        if rc == 0:
            (rd / f"p{n}.finished").write_text(time.strftime("%F %T"))
            log(f"{tag}: finished painting written")
        else:
            log(f"{tag}: FINISHING FAILED (exit {rc}, see {rd}/p{n}_finish_err.txt)")

    th = threading.Thread(target=go)
    th.start()
    finishers.append(th)


def stop_leftovers(d):
    r = subprocess.run(["/bin/ps", "-Ao", "pid,command"], capture_output=True, text=True).stdout
    for line in r.splitlines()[1:]:
        pid, _, cmd = line.strip().partition(" ")
        if str(d) in cmd and "easel" in cmd and "finish_painting" not in cmd:
            subprocess.run(["kill", pid])
            log(f"stopped leftover {pid}: {cmd[:100]}")


def cwd_of(pid):
    r = subprocess.run(["lsof", "-a", "-d", "cwd", "-p", pid, "-Fn"], capture_output=True, text=True).stdout
    return next((l[1:] for l in r.splitlines() if l.startswith("n")), "")


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


def watchdog(stop):
    """Stop processes working in one of this round's studios that run too long (not pi itself, not the easel server, not finishing)."""
    while not stop.is_set():
        try:
            monitor_histories()
        except Exception as e:
            log(f"monitor error: {e}")
        mine = [str(A / name) for _, name in our_studios()]
        r = subprocess.run(["/bin/ps", "-Ao", "pid,etimes,command"], capture_output=True, text=True).stdout
        for line in r.splitlines()[1:]:
            parts = line.split(None, 2)
            if len(parts) < 3 or not parts[1].isdigit() or int(parts[1]) < WATCHDOG_MIN * 60:
                continue
            pid, secs, cmd = parts
            exe = os.path.basename(cmd.split()[0])
            if (exe in ("pi", "node", "uv", "python3", "Python") or " serve " in cmd
                    or "r18_open" in cmd or "finish_painting" in cmd):
                continue
            where = cwd_of(pid)
            if any(where == s or where.startswith(s + "/") or s in cmd for s in mine):
                subprocess.run(["kill", pid])
                log(f"WATCHDOG stopped {pid} after {int(secs) // 60} min in {where}: {cmd[:140]}")
        stop.wait(60)


def lane_run(lane):
    rd = RUN / lane
    rd.mkdir(parents=True, exist_ok=True)
    env = {"R16_BRANCH": BRANCH}
    d = studio(lane)
    n = 1
    if (rd / f"p{n}.done").exists():
        log(f"{lane}: done already")
        if not DRY:
            finish(lane, d)
        return
    if DRY:
        show(lane, f"export blank studio (then studio_notes.md, BRIEF.md = {rd}/p1_brief.md)", export_cmd(d), BASE, env)
        show(lane, "sitting 1 (painter)", painter_cmd(lane, redact=True), d)
        show(lane, f"after each sitting: close the easel ({d / 'bin/easel'} close), stop leftovers, count painting chunks;\n"
                   f"    sittings 2..{MAX_SITTINGS} while the last one added painting chunks (next_sitting), each",
             painter_cmd(lane, SITTING_MESSAGE, redact=True), d)
        show(lane, "finishing (background, after the last sitting)", finish_cmd(d, lane), RUN)
        return
    (rd / f"p{n}_brief.md").write_text(brief(d))
    if not (rd / f"p{n}.exported").exists():
        if d.exists():
            shutil.rmtree(d)
        log(f"{lane}: exporting blank studio from {BRANCH} to {d}")
        r = subprocess.run(export_cmd(d), capture_output=True, text=True, env={**os.environ, **env})
        (rd / f"p{n}_export.log").write_text(r.stdout + r.stderr)
        if r.returncode:
            log(f"{lane}: EXPORT FAILED, lane stops (see {rd}/p{n}_export.log)")
            return
        (d / "notes" / "studio_notes.md").write_text((HERE / "studio_notes.md").read_text())
        (rd / f"p{n}.exported").write_text(time.strftime("%F %T"))
    (d / "BRIEF.md").write_text(brief(d))
    clear_settings(d)
    if not (rd / f"p{n}.painted").exists():
        paint(lane, d, rd)
    finish(lane, d)
    (rd / f"p{n}.done").write_text(time.strftime("%F %T"))
    log(f"lane {lane} finished")


def main():
    global DRY
    ap = argparse.ArgumentParser()
    ap.add_argument("--only")
    ap.add_argument("--dry", action="store_true")
    a = ap.parse_args()
    DRY = a.dry
    RUN.mkdir(parents=True, exist_ok=True)
    lanes = a.only.split(",") if a.only else list(LANES)
    for l in lanes:
        if l not in LANES:
            raise SystemExit(f"no lane {l}; lanes: {', '.join(LANES)}")
    stop = threading.Event()
    if not DRY:
        threading.Thread(target=watchdog, args=(stop,), daemon=True).start()
    th = [threading.Thread(target=lane_run, args=(l,)) for l in lanes]
    for x in th:
        x.start()
        if DRY:
            x.join()
        else:
            time.sleep(5)
    for x in th:
        x.join()
    if finishers:
        log("waiting for finishing")
    for x in finishers:
        x.join()
    stop.set()
    log("all lanes finished")


if __name__ == "__main__":
    main()
