# /// script
# requires-python = ">=3.11"
# ///
"""Round 16: four chains of three painters at the easel, run in parallel.

Each painter gets a fresh studio exported from branch r16-base (profile
friedrich or blank), the studio notes, and the reader's records of the
earlier painters in its chain merged into notes/studio_notes.md (nothing in
them says there were earlier painters). After a painter finishes, its
leftover easel processes are stopped and a reader writes the record for
the next one.

Resumable: every finished step leaves a marker in the run folder; a rerun
skips it. A watchdog stops any process started in a studio that runs longer
than WATCHDOG_MIN minutes (the painters' shell has no timeout of its own),
and logs it.

    uv run r16_chains.py                 # all four chains
    uv run r16_chains.py --only A,D      # some chains
    uv run r16_chains.py --dry           # print what would run
"""
import argparse
import os
import re
import shutil
import subprocess
import threading
import time
from pathlib import Path

HOME = Path.home()
A = HOME / "src/a"
REPO = A / "claude-paint"
RUN = HOME / "tmp/gallery-fcf9c110/r16/run"
HERE = Path(__file__).parent
SESS = HOME / ".pi/agent/sessions"
WATCHDOG_MIN = 30
EASEL_HOWTO = os.environ.get("EASEL_HOWTO", "The easel is `bin/easel` in your studio;\n  notes/easel_guide.md has its commands (`open`, `do`, `look`, `note`, `save`,\n  `close`).")
EASEL_SAVE = os.environ.get("EASEL_SAVE", "`bin/easel save`")

OPUS = ["--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "high"]
GEMINI = ["--provider", "openrouter", "--model", "google/gemini-3.8-flash", "--thinking", "high"]  # Google direct: prepaid credits depleted (402)
READER = ["--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "medium"]

FRIEDRICH_READING = ("notes/easel_guide.md; notes/studio_notes.md;\n"
                     "notes/research/friedrich_materials.md (his materials and method, sourced);\n"
                     "notes/research/trees.md (how trees are built) and\n"
                     "notes/research/oil_paint_physics.md as needed.")
BLANK_READING = "notes/easel_guide.md; notes/studio_notes.md;\nnotes/research/oil_paint_physics.md as needed."
DETAIL = "- Friedrich's pictures are full of small, particular details; don't stop at broad\n  passages.\n"
TRACKS = {
    "A": dict(profile="friedrich", model=OPUS, reading=FRIEDRICH_READING, detail=DETAIL, opening=(
        "Compose and paint one original winter landscape in the manner of Caspar David\n"
        "Friedrich, at the easel of claude-paint, a simulator of oil paint on linen. The\n"
        "subject, hour and composition within the winter are yours. Work from knowledge\n"
        "and the notes in your studio; don't use reference images, image models or\n"
        "pictures of his work.")),
    "B": dict(profile="friedrich", model=OPUS, reading=FRIEDRICH_READING, detail=DETAIL, opening=(
        "Compose and paint one original summer landscape in the manner of Caspar David\n"
        "Friedrich, at the easel of claude-paint, a simulator of oil paint on linen. The\n"
        "subject, hour and composition within the summer are yours. Work from knowledge\n"
        "and the notes in your studio; don't use reference images, image models or\n"
        "pictures of his work.")),
    "C": dict(profile="blank", model=OPUS, reading=BLANK_READING, detail="", opening=(
        "Paint one picture of your choosing in oil, at the easel of claude-paint, a\n"
        "simulator of oil paint on linen. Subject, composition and manner are yours.\n"
        "Work from what you know; don't use reference images or image models.")),
    "D": dict(profile="blank", model=GEMINI, reading=BLANK_READING, detail="", opening=(
        "Paint one picture of your choosing in oil, at the easel of claude-paint, a\n"
        "simulator of oil paint on linen. Subject, composition and manner are yours.\n"
        "Work from what you know; don't use reference images or image models.")),
}
PAINTERS = 3

lock = threading.Lock()


def log(msg):
    line = f"{time.strftime('%F %T')} {msg}"
    with lock:
        print(line, flush=True)
        with open(RUN / "chains.log", "a") as f:
            f.write(line + "\n")


def studio(track, n):
    """A neutral folder name (nothing about rounds, lanes or order); the mapping stays in the run folder."""
    import hashlib, json
    m = RUN / "studios.json"
    names = json.loads(m.read_text()) if m.exists() else {}
    key = f"{track}{n}"
    if key not in names:
        names[key] = "paint-studio-" + hashlib.sha1(f"r16{key}{time.time()}".encode()).hexdigest()[:6]
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


def pi(args, prompt, cwd, out, err):
    cmd = ["pi", "--print", "--no-context-files", "--no-skills", "--no-prompt-templates"] + args + [prompt]
    with open(out, "w") as o, open(err, "w") as e:
        return subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=o, stderr=e).returncode


def stop_leftovers(d):
    r = subprocess.run(["/bin/ps", "-Ao", "pid,command"], capture_output=True, text=True).stdout
    for line in r.splitlines()[1:]:
        pid, _, cmd = line.strip().partition(" ")
        if str(d) in cmd and "easel" in cmd:
            subprocess.run(["kill", pid])
            log(f"stopped leftover {pid}: {cmd[:100]}")


def cwd_of(pid):
    r = subprocess.run(["lsof", "-a", "-d", "cwd", "-p", pid, "-Fn"], capture_output=True, text=True).stdout
    return next((l[1:] for l in r.splitlines() if l.startswith("n")), "")


def monitor_histories():
    """Each painting's log may only grow: record it, and alert if a history got shorter or changed."""
    import json
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
    """Stop processes working in a round 16 studio that run too long (not pi itself, not the easel server)."""
    while not stop.is_set():
        try:
            monitor_histories()
        except Exception as e:
            log(f"monitor error: {e}")
        r = subprocess.run(["/bin/ps", "-Ao", "pid,etimes,command"], capture_output=True, text=True).stdout
        for line in r.splitlines()[1:]:
            parts = line.split(None, 2)
            if len(parts) < 3 or not parts[1].isdigit() or int(parts[1]) < WATCHDOG_MIN * 60:
                continue
            pid, secs, cmd = parts
            exe = os.path.basename(cmd.split()[0])
            if exe in ("pi", "node", "uv", "python3", "Python") or " serve " in cmd or "r16_chains" in cmd:
                continue
            where = cwd_of(pid)
            if "/paint-studio-" in where or "paint-studio-" in cmd:
                subprocess.run(["kill", pid])
                log(f"WATCHDOG stopped {pid} after {int(secs) // 60} min in {where}: {cmd[:140]}")
        stop.wait(60)


def chain(track, dry):
    t = TRACKS[track]
    rd = RUN / track
    rd.mkdir(parents=True, exist_ok=True)
    for n in range(1, PAINTERS + 1):
        d = studio(track, n)
        tag = f"{track}{n}"
        if (rd / f"p{n}.done").exists():
            log(f"{tag}: done already")
            continue
        (rd / f"p{n}_brief.md").write_text(brief(track, n))
        if dry:
            log(f"{tag}: would export {t['profile']} to {d}, paint with {' '.join(t['model'])}")
            continue
        if not (rd / f"p{n}.exported").exists():
            if d.exists():
                shutil.rmtree(d)
            log(f"{tag}: exporting {t['profile']} studio to {d}")
            r = subprocess.run([str(REPO.parent / "claude-paint-r16-base/scripts/export_r16_studio"), t["profile"], str(d)],
                               capture_output=True, text=True)
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
        log(f"{tag}: painting ({t['model'][3]})")
        t0 = time.time()
        rc = pi(t["model"], "Complete your task autonomously. Read BRIEF.md in this folder and follow it exactly. "
                "That file is your whole brief. Your FINAL message is the reply it asks for.",
                d, rd / f"p{n}_final.txt", rd / f"p{n}_err.txt")
        stop_leftovers(d)
        log(f"{tag}: painter finished (exit {rc}) after {(time.time() - t0) / 60:.0f} min")
        logs = sorted(session_dir(d).glob("*.jsonl"), key=lambda f: f.stat().st_mtime)
        if logs and n < PAINTERS:
            out = rd / f"p{n}_record.md"
            rb = ((HERE / "reader_brief.md").read_text().replace("{LOG}", str(logs[-1]))
                  .replace("{JOURNAL}", str(d / "notes/journal.md")).replace("{OUT}", str(out)))
            (rd / f"p{n}_reader_brief.md").write_text(rb)
            log(f"{tag}: reader")
            pi(READER, f"Read {rd}/p{n}_reader_brief.md and do what it says.", rd, rd / f"p{n}_reader_final.txt", rd / f"p{n}_reader_err.txt")
            log(f"{tag}: record {'written' if out.exists() else 'MISSING'}")
        (rd / f"p{n}.done").write_text(time.strftime("%F %T"))
    log(f"chain {track} finished")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only")
    ap.add_argument("--dry", action="store_true")
    a = ap.parse_args()
    RUN.mkdir(parents=True, exist_ok=True)
    tracks = a.only.split(",") if a.only else list(TRACKS)
    stop = threading.Event()
    if not a.dry:
        threading.Thread(target=watchdog, args=(stop,), daemon=True).start()
    th = [threading.Thread(target=chain, args=(t, a.dry)) for t in tracks]
    for x in th:
        x.start()
        time.sleep(5)
    for x in th:
        x.join()
    stop.set()
    log("all chains finished")


if __name__ == "__main__":
    main()
