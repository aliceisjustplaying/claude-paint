# /// script
# requires-python = ">=3.11"
# ///
"""Round 17: two chains of three painters at the easel.

Lanes: F (a Friedrich landscape; --season summer|winter, default summer: a
June day) and O (open: a picture of the painter's choosing, blank studio).
Both Opus 5.5, thinking high.

Sittings: each painter works in up to MAX_SITTINGS sittings, each a new pi
session (same harness, studio and settings). Sitting 1 is the plain launch;
later ones get SITTING_MESSAGE. The easel is replayed once before Pi starts
and stays open across recovery sittings. The painter stops after a completed
sitting that added no mark-making chunks, or after MAX_SITTINGS completed
sittings (see next_sitting). Crashes are retried and do not count as a
judgment. Each sitting is recorded in run/<lane>/p<n>_sittings.json.

Each painter gets a fresh studio exported from branch r17-base (profile
friedrich or blank), the studio notes, and the reader's records of the
earlier painters in its chain merged into notes/studio_notes.md (nothing in
them says there were earlier painters). Painters run in the clean harness
(claude-paint-r17-base/harness/painter): no global extensions, our system
prompt, bash and read only, our compaction. After a painter finishes, its
leftover easel processes are stopped, finishing (varnish and cracks, which
the painter build lacks) starts in the background on a replay of its log
(run/<lane><n>_finished.png; the painter's own save is untouched), and a
reader writes the record for the next painter.

Resumable: every finished step leaves a marker in the run folder; a rerun
skips it (and starts any finishing that didn't finish). A watchdog stops any
process started in a studio that runs longer than WATCHDOG_MIN minutes (the
painters' shell has no timeout of its own), and logs it; it also watches
that each painting's log only grows.

    uv run r17_chains.py                         # both chains, F on a June day
    uv run r17_chains.py --season winter         # F paints a winter landscape
    uv run r17_chains.py --only F                # one chain
    uv run r17_chains.py --dry                   # print the exact commands, run nothing
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
BASE = A / "claude-paint-r17-base"
BRANCH = "r17-base"
EXPORT = BASE / "scripts/export_r16_studio"      # honors R16_BRANCH
FINISH = BASE / "scripts/finish_painting"
H = BASE / "harness/painter"
BLACK = HOME / ".pi/agent/git/github.com/aliceisjustplaying/pi-black/extensions/pi-black.ts"
TEMP_GUARD = HOME / ".pi/agent/extensions/persistent-temp.ts"
RUN = HOME / "tmp/gallery-fcf9c110/r17/run"
HERE = Path(__file__).parent
SESS = HOME / ".pi/agent/sessions"
WATCHDOG_MIN = 30
EASEL_HOWTO = os.environ.get("EASEL_HOWTO", "The easel is `bin/easel` in your studio;\n  notes/easel_guide.md has its commands (`open`, `do`, `look`, `note`, `save`,\n  `close`).")
EASEL_SAVE = os.environ.get("EASEL_SAVE", "`bin/easel save`")

OPUS = ["--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "high"]
READER = ["--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "medium"]
HARNESS = ["--no-extensions", "-e", str(H / "painter.ts"), "-e", str(H / "compaction.ts"),
           "-e", str(TEMP_GUARD), "-e", str(BLACK),
           "--system-prompt", str(H / "system_prompt.md"), "--tools", "bash,read",
           "--no-context-files", "--no-skills", "--no-prompt-templates", "--approve"]
PAINTER_MSG = ("Complete your task autonomously. Read BRIEF.md in this folder and follow it exactly. "
               "That file is your whole brief. Your FINAL message is the reply it asks for.")
# DRAFT for Alice: the user message of sittings 2..MAX_SITTINGS.
SITTING_MESSAGE = ("You're back in your studio. The painting is on the easel as you left it: "
                   "`bin/easel open` picks it up where you stopped. Your brief is in BRIEF.md and "
                   "your journal in notes/journal.md. Your FINAL message is the reply the brief asks for.")
MAX_SITTINGS = 4
MAX_CRASHES = 6
CRASH_WAITS = [90, 180, 300, 600, 900, 1200]

FRIEDRICH_READING = ("notes/easel_guide.md; notes/studio_notes.md;\n"
                     "notes/research/friedrich_materials.md (his materials and method, sourced);\n"
                     "notes/research/trees.md (how trees are built) and\n"
                     "notes/research/oil_paint_physics.md as needed.")
BLANK_READING = "notes/easel_guide.md; notes/studio_notes.md;\nnotes/research/oil_paint_physics.md as needed."
DETAIL = "- Friedrich's pictures are full of small, particular details; don't stop at broad\n  passages.\n"


OPENINGS_F = {
    "winter": (
        "Compose and paint one original winter landscape in the manner of Caspar David\n"
        "Friedrich, at the easel of claude-paint, a simulator of oil paint on linen. The\n"
        "subject, hour and composition within the winter are yours. Work from knowledge\n"
        "and the notes in your studio; don't use reference images, image models or\n"
        "pictures of his work."),
    "summer": (
        "Compose and paint one original landscape on a June day in the manner of Caspar\n"
        "David Friedrich, at the easel of claude-paint, a simulator of oil paint on\n"
        "linen. The subject and composition are yours. Work from knowledge and the notes\n"
        "in your studio; don't use reference images, image models or pictures of his\n"
        "work."),
}


def tracks(season):
    return {
        "F": dict(profile="friedrich", model=OPUS, reading=FRIEDRICH_READING, detail=DETAIL, opening=OPENINGS_F[season]),
        "O": dict(profile="blank", model=OPUS, reading=BLANK_READING, detail="", opening=(
            "Paint one picture of your choosing in oil, at the easel of claude-paint, a\n"
            "simulator of oil paint on linen. Subject, composition and manner are yours.\n"
            "Work from what you know; don't use reference images or image models.")),
    }


TRACKS = tracks("summer")
PAINTERS = 3
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


def studio(track, n):
    """A neutral folder name (nothing about rounds, lanes or order); the mapping stays in the run folder."""
    with lock:
        m = RUN / "studios.json"
        names = json.loads(m.read_text()) if m.exists() else {}
        key = f"{track}{n}"
        if key not in names:
            names[key] = "paint-studio-" + hashlib.sha1(f"r17{key}{time.time()}".encode()).hexdigest()[:6]
            m.write_text(json.dumps(names, indent=1))
        return A / names[key]


def session_dir(d):
    return SESS / ("--" + str(d).strip("/").replace("/", "-") + "--")


def brief(track, n):
    t = TRACKS[track]
    tpl = (HERE / "brief_template.md").read_text()
    d = studio(track, n)
    return (tpl.replace("{OPENING}", t["opening"]).replace("{STUDIO}", f"~/src/a/{d.name}")
               .replace("{EASEL_HOWTO}", EASEL_HOWTO).replace("{EASEL_SAVE}", EASEL_SAVE).replace("{SLUG}", d.name.replace("paint-", ""))
               .replace("{READING}", t["reading"]).replace("{DETAIL}", t["detail"]))


def painter_cmd(model, message=PAINTER_MSG):
    return ["pi", "--print"] + HARNESS + model + [message]


def count_chunks(d):
    f = d / "paintings/lua/painting.lua"
    if not f.exists():
        return 0
    return sum(1 for line in f.read_text(errors="replace").splitlines() if line.startswith("--@ chunk"))


def count_painting(d):
    """Chunks of the log that put marks on the canvas."""
    f = d / "paintings/lua/painting.lua"
    return count_painting_chunks(f.read_text(errors="replace")) if f.exists() else 0


def next_sitting(sittings, max_sittings=MAX_SITTINGS, max_crashes=MAX_CRASHES):
    """The number of the next sitting, or None when the painter is done.

    Only completed sittings are judged. Crashed and interrupted sittings are
    retried and do not count toward max_sittings. Stop after a completed
    sitting that added no painting chunks, or after max_sittings completed
    sittings. Older records without painting counts use their chunk counts.
    """
    if not sittings:
        return 1
    done = [s for s in sittings if s.get("status") == "completed"]
    if done:
        last = done[-1]
        before, after = (("painting_before", "painting_after")
                         if last.get("painting_after") is not None
                         else ("chunks_before", "chunks_after"))
        if last[after] <= last[before]:
            return None
        if len(done) >= max_sittings:
            return None
    if sum(1 for s in sittings if s.get("status") == "crashed") >= max_crashes:
        return None
    return max(s["sitting"] for s in sittings) + 1


def session_error(files):
    """The provider error ending the newest Pi session, or None."""
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
    hints = re.findall(r"retry in ([\d.]+)\s*s", text, re.I) + re.findall(r'retryDelay\W+([\d.]+)s', text)
    return max((float(h) for h in hints), default=0.0)


def crash_wait(n_crashes, text):
    return max(CRASH_WAITS[min(n_crashes, len(CRASH_WAITS)) - 1], retry_hint(text) + 15)


def gist(text, n=240):
    t = " ".join(text.split())
    return t[:n] + ("..." if len(t) > n else "")


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
    """Replay once before Pi starts, with progress visible in the runner log."""
    stop_leftovers(d, keep_server=True)
    p = subprocess.Popen([str(d / "bin/easel"), "open"], cwd=d, stdin=subprocess.DEVNULL,
                         stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    for line in p.stdout:
        log(f"{tag}: {line.rstrip()}")
    rc = p.wait()
    if rc:
        log(f"{tag}: EASEL OPEN FAILED (exit {rc})")
        return False
    return True


def paint(track, n, d, rd):
    """Run the painter's sittings until next_sitting says stop; resumable from p<n>_sittings.json."""
    t = TRACKS[track]
    tag = f"{track}{n}"
    sittings = load_sittings(rd, n)
    for s in sittings:
        if s.get("status") == "running":      # the runner stopped during this sitting
            s.update(status="interrupted", chunks_after=count_chunks(d),
                     painting_after=count_painting(d))
            log(f"{tag}: sitting {s['sitting']} was interrupted; it will be taken again")
    for s in sittings:
        if s.get("status") == "completed" and s.get("exit") not in (0, None):
            s.update(status="crashed", reclassified=time.strftime("%F %T"))
            log(f"{tag}: sitting {s['sitting']} exited {s['exit']}: counted as a crash, not a judgment")
    save_sittings(rd, n, sittings)
    while (k := next_sitting(sittings)) is not None:
        if not open_easel(d, tag):
            return False
        before = count_chunks(d)
        painting_before = count_painting(d)
        msg = PAINTER_MSG if before == 0 else SITTING_MESSAGE
        known = set(session_dir(d).glob("*.jsonl"))
        rec = dict(sitting=k, status="running", start=time.strftime("%F %T"), end=None,
                   chunks_before=before, chunks_after=None,
                   painting_before=painting_before, painting_after=None,
                   message=msg, sessions=[], exit=None,
                   final_file=str(rd / f"p{n}_s{k}_final.txt"), final=None)
        sittings.append(rec)
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k} ({t['model'][3]}, painter harness, {before} chunks on the easel, "
            f"{painting_before} painting)")
        t0 = time.time()
        rc = run(painter_cmd(t["model"], msg), d, rd / f"p{n}_s{k}_final.txt", rd / f"p{n}_s{k}_err.txt")
        stop_leftovers(d, keep_server=True)
        new = sorted(set(session_dir(d).glob("*.jsonl")) - known, key=lambda f: f.stat().st_mtime)
        api_error = session_error(new)
        err_text = (rd / f"p{n}_s{k}_err.txt").read_text(errors="replace")
        crashed = rc != 0 or api_error is not None
        why = api_error or err_text.strip() or f"exit {rc}"
        rec.update(status="crashed" if crashed else "completed", end=time.strftime("%F %T"),
                   chunks_after=count_chunks(d), painting_after=count_painting(d), exit=rc,
                   sessions=[str(f) for f in new],
                   error=why[-4000:] if crashed else None,
                   final=(rd / f"p{n}_s{k}_final.txt").read_text(errors="replace"))
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k} ended (exit {rc}{', CRASHED' if crashed else ''}) after {(time.time() - t0) / 60:.0f} min: "
            f"chunks {rec['chunks_before']} -> {rec['chunks_after']} "
            f"(painting {rec['painting_before']} -> {rec['painting_after']}), {len(new)} session file(s)")
        if crashed:
            crashes = sum(1 for s in sittings if s.get("status") == "crashed")
            if crashes >= MAX_CRASHES:
                log(f"{tag}: STOPPED after {crashes} crashes; last error: {gist(why)}")
                close_easel(d)
                return False
            wait = crash_wait(crashes, why + "\n" + err_text)
            log(f"{tag}: crash {crashes} of {MAX_CRASHES}, not a judgment; retrying in {wait:.0f}s: {gist(why)}")
            time.sleep(wait)
    close_easel(d)
    shutil.copy(rd / f"p{n}_s{sittings[-1]['sitting']}_final.txt", rd / f"p{n}_final.txt")
    (rd / f"p{n}.painted").write_text(f"{time.strftime('%F %T')} {len(sittings)} sitting(s)")
    return True


def reader_cmd(prompt):
    """Round 16's reader launch: the machine's own pi setup, cwd the run folder (no painter harness)."""
    return ["pi", "--print", "--no-context-files", "--no-skills", "--no-prompt-templates"] + READER + [prompt]


def export_cmd(profile, d):
    return [str(EXPORT), profile, str(d)]


def finish_cmd(d, track, n):
    return [str(FINISH), str(d / "paintings/lua/painting.lua"), str(RUN / f"{track}{n}_finished.png")]


def run(cmd, cwd, out, err):
    with open(out, "w") as o, open(err, "w") as e:
        return subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=o, stderr=e).returncode


def show(tag, what, cmd, cwd, env=None):
    pre = " ".join(f"{k}={shlex.quote(v)}" for k, v in (env or {}).items())
    log(f"{tag}: {what}:\n    cd {shlex.quote(str(cwd))} && {pre + ' ' if pre else ''}{shlex.join(cmd)}")


def install_settings(d):
    (d / ".pi").mkdir(exist_ok=True)
    shutil.copy(H / "studio-settings.json", d / ".pi" / "settings.json")


def finish(track, n, d):
    """Varnish and crack a replay of the painter's log, in the background; the painter's save stays as is."""
    rd = RUN / track
    tag = f"{track}{n}"
    if (rd / f"p{n}.finished").exists():
        return
    lua = d / "paintings/lua/painting.lua"
    if not lua.exists() or not lua.stat().st_size:
        log(f"{tag}: no log to finish at {lua}")
        return

    def go():
        log(f"{tag}: finishing in the background -> {RUN / f'{tag}_finished.png'}")
        rc = run(finish_cmd(d, track, n), RUN, rd / f"p{n}_finish.log", rd / f"p{n}_finish_err.txt")
        if rc == 0:
            (rd / f"p{n}.finished").write_text(time.strftime("%F %T"))
            log(f"{tag}: finished painting written")
        else:
            log(f"{tag}: FINISHING FAILED (exit {rc}, see {rd}/p{n}_finish_err.txt)")

    th = threading.Thread(target=go)
    th.start()
    finishers.append(th)


def stop_leftovers(d, keep_server=False):
    """Stop easel clients in this studio; optionally preserve its live server."""
    r = subprocess.run(["/bin/ps", "-Ao", "pid=,command="], capture_output=True, text=True).stdout
    for line in r.splitlines():
        pid, _, cmd = line.strip().partition(" ")
        where = cwd_of(pid)
        in_studio = where == str(d) or where.startswith(str(d) + "/") or str(d) in cmd
        server = " serve " in f" {cmd} "
        if in_studio and "easel" in cmd and "finish_painting" not in cmd and not (keep_server and server):
            subprocess.run(["kill", pid])
            log(f"stopped leftover {pid}: {cmd[:100]}")


def cwd_of(pid):
    r = subprocess.run(["lsof", "-a", "-d", "cwd", "-p", pid, "-Fn"], capture_output=True, text=True).stdout
    return next((l[1:] for l in r.splitlines() if l.startswith("n")), "")


def etime_seconds(etime):
    """Seconds from macOS ps's [[dd-]hh:]mm:ss elapsed-time field."""
    m = re.fullmatch(r"(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)", etime.strip())
    if not m:
        return None
    d, h, mi, s = (int(x) if x else 0 for x in m.groups())
    return ((d * 24 + h) * 60 + mi) * 60 + s


def cwds_of_all():
    """pid -> cwd from one lsof call."""
    r = subprocess.run(["lsof", "-a", "-d", "cwd", "-Fpn"], capture_output=True, text=True).stdout
    out, pid = {}, None
    for line in r.splitlines():
        if line.startswith("p"):
            pid = line[1:]
        elif line.startswith("n") and pid:
            out[pid] = line[1:]
    return out


def spared(cmd):
    exe = os.path.basename(cmd.split()[0]) if cmd.split() else ""
    return (exe in ("pi", "node") or " serve " in f" {cmd} " or "easel open" in cmd
            or "finish_painting" in cmd or "r17_chains" in cmd)


def overdue(ps_lines, cwds, studios, limit_s, me=None):
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


def monitor_histories():
    """Each painting's log may only grow: record it, and alert if a history got shorter or changed."""
    m = RUN / "studios.json"
    if not m.exists():
        return
    mon = RUN / "monitor"
    mon.mkdir(exist_ok=True)
    for key, name in json.loads(m.read_text()).items():
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
    """Stop processes working in a round 17 studio that run too long (not pi itself, not the easel server, not finishing)."""
    while not stop.is_set():
        try:
            monitor_histories()
        except Exception as e:
            log(f"monitor error: {e}")
        try:
            studios = [str(A / name) for name in json.loads((RUN / "studios.json").read_text()).values()]
            ps = subprocess.run(["/bin/ps", "-Ao", "pid=,etime=,command="], capture_output=True, text=True).stdout.splitlines()
            for pid, secs, where, cmd in overdue(ps, cwds_of_all(), studios, WATCHDOG_MIN * 60, str(os.getpid())):
                subprocess.run(["kill", pid])
                log(f"WATCHDOG stopped {pid} after {secs // 60} min in {where}: {cmd[:140]}")
        except Exception as e:
            log(f"watchdog error: {e}")
        stop.wait(60)


def chain(track):
    t = TRACKS[track]
    rd = RUN / track
    rd.mkdir(parents=True, exist_ok=True)
    env = {"R16_BRANCH": BRANCH}
    for n in range(1, PAINTERS + 1):
        d = studio(track, n)
        tag = f"{track}{n}"
        if (rd / f"p{n}.done").exists():
            log(f"{tag}: done already")
            if not DRY:
                finish(track, n, d)
            continue
        (rd / f"p{n}_brief.md").write_text(brief(track, n))
        if DRY:
            show(tag, f"export {t['profile']} studio (then studio_notes.md + records"
                      f"{' + trees.md' if t['profile'] == 'friedrich' else ''}, BRIEF.md = {rd}/p{n}_brief.md)",
                 export_cmd(t["profile"], d), BASE, env)
            show(tag, "painter settings", ["cp", str(H / "studio-settings.json"), str(d / ".pi/settings.json")], d)
            show(tag, "sitting 1 (painter)", painter_cmd(t["model"]), d)
            show(tag, f"keep the easel open; sittings 2..{MAX_SITTINGS} while the last completed sitting added painting chunks, each",
                 painter_cmd(t["model"], SITTING_MESSAGE), d)
            show(tag, "finishing (background, after the last sitting)", finish_cmd(d, track, n), RUN)
            if n < PAINTERS:
                show(tag, "reader", reader_cmd(f"Read {rd}/p{n}_reader_brief.md and do what it says."), rd)
            continue
        if not (rd / f"p{n}.exported").exists():
            if d.exists():
                shutil.rmtree(d)
            log(f"{tag}: exporting {t['profile']} studio from {BRANCH} to {d}")
            r = subprocess.run(export_cmd(t["profile"], d), capture_output=True, text=True,
                               env={**os.environ, **env})
            (rd / f"p{n}_export.log").write_text(r.stdout + r.stderr)
            if r.returncode:
                log(f"{tag}: EXPORT FAILED, chain stops (see {rd}/p{n}_export.log)")
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
        (d / "BRIEF.md").write_text(brief(track, n))
        install_settings(d)
        if not (rd / f"p{n}.painted").exists():
            if not paint(track, n, d, rd):
                log(f"{tag}: painting recovery stopped; chain stops")
                return
        finish(track, n, d)
        logs = [p for s in load_sittings(rd, n) for p in s.get("sessions", []) if Path(p).exists()]
        if logs and n < PAINTERS:
            out = rd / f"p{n}_record.md"
            rb = ((HERE / "reader_brief.md").read_text().replace("{LOG}", ", ".join(logs))
                  .replace("{JOURNAL}", str(d / "notes/journal.md")).replace("{OUT}", str(out)))
            (rd / f"p{n}_reader_brief.md").write_text(rb)
            log(f"{tag}: reader")
            run(reader_cmd(f"Read {rd}/p{n}_reader_brief.md and do what it says."), rd,
                rd / f"p{n}_reader_final.txt", rd / f"p{n}_reader_err.txt")
            log(f"{tag}: record {'written' if out.exists() else 'MISSING'}")
        (rd / f"p{n}.done").write_text(time.strftime("%F %T"))
    log(f"chain {track} finished")


def main():
    global TRACKS, DRY
    ap = argparse.ArgumentParser()
    ap.add_argument("--only")
    ap.add_argument("--season", choices=["summer", "winter"], default="summer")
    ap.add_argument("--dry", action="store_true")
    a = ap.parse_args()
    DRY = a.dry
    TRACKS = tracks(a.season)
    RUN.mkdir(parents=True, exist_ok=True)
    sf = RUN / "season.txt"
    if sf.exists() and sf.read_text().strip() != a.season:
        raise SystemExit(f"this run folder's F lane is {sf.read_text().strip()}; rerun with --season {sf.read_text().strip()}")
    if not DRY:
        sf.write_text(a.season)
    lanes = a.only.split(",") if a.only else list(TRACKS)
    stop = threading.Event()
    if not DRY:
        threading.Thread(target=watchdog, args=(stop,), daemon=True).start()
    for lane in lanes:
        chain(lane)
    if finishers:
        log("waiting for finishing")
    for x in finishers:
        x.join()
    stop.set()
    log("all chains finished")


if __name__ == "__main__":
    main()
