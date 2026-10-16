# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""The atelier's painter side (SPEC.md sections 2 to 4): two studios, Claude Opus 5.5 (pi, the house runner
through run.py) and Gemini 3.8 Flash (agy, started by hand). Each painting gets its own folder; the studio's
notebook and walls are copied in at start and carried home at finish.

    atelier.py [paths] start <studio> [--theme "<a few words>"] [--prepare]
    atelier.py [paths] finish <studio>

start: under the studio's lock, refuses while the studio's last painting hasn't hung or free disk is under
DISK_FLOOR plus PER_PAINTING for each painting on an easel (this one included). Exports the house every-box studio
(scripts/export_r16_studio every, 2400 px, from this checkout's HEAD), copies in notebook.md and the walls, writes
BRIEF.md from texts/, the notebook's before-copy and works/<id>-<nnn>/ (manifest.json, BRIEF.md, run/studios.json).
Opus: writes the launch agent (launch_agent()) and, without --prepare, loads it. Flash: configures agy
(scripts/prepare_agy_studio) and writes HAND-LAUNCH.md beside the manifest; agy is started by hand.

finish: once the painter is done (Opus: run/OPUS/p1_outcome.json "finished"; Flash: out/easel/sittings.json
complete), under the machine-wide finishing lock: closes the easel, replays the log once (scripts/check_painting,
which must pass), renders color from the session's save (scripts/finish_painting) and gallery and relief from the
same save (easel render-checkpoint). Then under the studio's lock: copies the notebook home, hangs the wall
(walls/<n>.png, 1000 px wide, and cards/<nnn>.json), rewrites walls/list.md, copies the journal and log, writes
manifest.json last, appends to history.md and commits the text records to the local stillwet-runs-data clone
(no push), under the records lock.

Paths (defaults are the M3's): --root ~/src/a/stillwet-runs/atelier, --studios ~/src/a (painting folders),
--data ~/src/a/stillwet-runs-data (records go to <data>/stillwet-runs/atelier/...), --sessions ~/.pi/agent/sessions.
"""
import argparse
from contextlib import contextmanager
from datetime import datetime, timezone
import fcntl
import gzip
import hashlib
import importlib.machinery
import importlib.util
import json
import os
from pathlib import Path
import plistlib
import secrets
import shlex
import shutil
import subprocess
import sys
import tempfile

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
TEXTS = HERE / "texts"
STUDIOS = {
    "claude-opus-5-5": {"name": "Claude Opus 5.5", "model": "claude-opus-5-5", "thinking": "xhigh", "harness": "pi", "lane": "OPUS"},
    "gemini-3-8-flash": {"name": "Gemini 3.8 Flash", "model": "gemini-3.8-flash", "thinking": "high", "harness": "agy", "lane": "FLASH"},
}
AGY_MODEL = "Gemini 3.8 Flash (High)"
LABEL = "art.stillwet.painter.atelier-opus"
WIDTH, BOX = 2400, "every"
DISK_FLOOR = 15 * 10**9
PER_PAINTING = 3 * 10**9
WALL_WIDTH = 1000
RAYON_THREADS = "4"


class Refused(Exception):
    pass


def text(name):
    """A texts/ file without its final newline (each ends with exactly one)."""
    return (TEXTS / name).read_text().removesuffix("\n")


def now():
    return datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")


def write(path, data):
    """Whole or not at all: a temp file beside it, then a rename (the minute export never reads half a file)."""
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    tmp.write_bytes(data.encode() if isinstance(data, str) else data)
    tmp.replace(path)


def write_json(path, value):
    write(path, json.dumps(value, indent=1, ensure_ascii=False) + "\n")


@contextmanager
def locked(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "a") as f:
        fcntl.flock(f, fcntl.LOCK_EX)
        yield


def shown(path):
    """A path as the brief names it: under the home folder as ~/..."""
    try:
        return "~/" + str(path.relative_to(Path.home()))
    except ValueError:
        return str(path)


def render_brief(folder, theme=None, later=False):
    """BRIEF.md from texts/ (SPEC 5.3): later paintings get the walls line and the painter's own files first."""
    opening = text("opening_theme.md").replace("{THEME}", theme) if theme else text("opening_free.md")
    return ((TEXTS / "brief_template.md").read_text()
            .replace("{STUDIO}", shown(folder))
            .replace("{WALLS}", text("walls_line.md") + "\n" if later else "")
            .replace("{READING}", text("reading_later.md" if later else "reading_first.md"))
            .replace("{OPENING}", opening))


def render_walls(cards):
    """walls/list.md (SPEC 5.11): oldest first, each with its image path and the painter's reply verbatim."""
    parts = [text("walls_list_header.md")]
    for card in cards:
        head = f"## {card['number']}. {card['title']}" if card["title"] else f"## {card['number']}"
        parts.append(f"{head}\n\nwalls/{card['number']}.png\n\n{card['words'].rstrip(chr(10))}")
    return "\n\n".join(parts) + "\n"


def title_of(reply):
    """The title in the painter's reply, as the studio viewer reads it."""
    sys.path.insert(0, str(REPO / "studio"))
    try:
        from studio import title_of as viewer_title
    finally:
        sys.path.remove(str(REPO / "studio"))
    return viewer_title(reply)


def house():
    """The house runner (house/r21_chains.py, r40's, as STIL-89 ran it), with its helpers from notes/round24/runner."""
    helpers = str(REPO / "notes/round24/runner")
    if helpers not in sys.path:
        sys.path.insert(0, helpers)
    spec = importlib.util.spec_from_file_location("atelier_house", HERE / "house/r21_chains.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def prepare_agy():
    loader = importlib.machinery.SourceFileLoader("prepare_agy_studio", str(REPO / "scripts/prepare_agy_studio"))
    module = importlib.util.module_from_spec(importlib.util.spec_from_loader(loader.name, loader))
    loader.exec_module(module)
    return module.prepare


def run(command, **options):
    return subprocess.run([str(c) for c in command], stdin=subprocess.DEVNULL, **options)


class Atelier:
    def __init__(self, root, studios, data, sessions):
        self.root, self.studios, self.data, self.sessions = (Path(p).expanduser().absolute() for p in (root, studios, data, sessions))

    def home(self, sid):
        return self.root / "studios" / sid

    def works(self, sid):
        return sorted(self.root.glob(f"works/{sid}-[0-9][0-9][0-9]"))

    def hung(self, sid, work):
        return (self.home(sid) / "cards" / f"{work.name[-3:]}.json").exists()

    def cards(self, sid):
        return [json.loads(p.read_text()) for p in sorted((self.home(sid) / "cards").glob("[0-9][0-9][0-9].json"))]

    def on_easel(self, sid):
        return [w for w in self.works(sid) if not self.hung(sid, w)]

    def history(self, line):
        with locked(self.root / ".git-lock"):
            with open(self.root / "history.md", "a") as f:
                f.write(f"- {now()} {line}\n")

    def options(self):
        return ["--root", self.root, "--studios", self.studios, "--data", self.data, "--sessions", self.sessions]

    def launch_agent(self, sid, n, work):
        """The Opus painting's launch agent: the house runner through run.py, one easel with four workers, at
        Interactive priority (STIL-40: a chunk took 1,093 s as a default agent, 387 s as Interactive)."""
        uv = shutil.which("uv") or "uv"
        return {
            "Label": LABEL,
            "ProgramArguments": ["/usr/bin/caffeinate", "-i", uv, "run", "--script", str(HERE / "run.py"),
                                 *map(str, self.options()), sid, str(n)],
            "EnvironmentVariables": {"HOME": str(Path.home()), "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
                                     "PYTHONDONTWRITEBYTECODE": "1", "RAYON_NUM_THREADS": RAYON_THREADS},
            "WorkingDirectory": str(work),
            "ProcessType": "Interactive",
            "KeepAlive": False,
            "RunAtLoad": True,
            "StandardOutPath": str(work / "run/runner.OPUS.out"),
            "StandardErrorPath": str(work / "run/runner.OPUS.err"),
        }

    def start(self, sid, theme=None, prepare=False):
        info = STUDIOS[sid]
        home = self.home(sid)
        with locked(home / "lock"):
            waiting = self.on_easel(sid)
            if waiting:
                raise Refused(f"{sid}: painting {int(waiting[-1].name[-3:])} hasn't hung yet; the next one starts after its finish")
            running = sum(len(self.on_easel(s)) for s in STUDIOS)
            self.studios.mkdir(parents=True, exist_ok=True)
            free, need = shutil.disk_usage(self.studios).free, DISK_FLOOR + PER_PAINTING * (running + 1)
            if free < need:
                raise Refused(f"{free / 1e9:.1f} GB free under {self.studios}, under the {need / 1e9:.0f} GB floor for {running + 1} painting(s)")
            n = len(self.works(sid)) + 1
            folder = self.studios / f"paint-studio-{secrets.token_hex(3)}"
            while folder.exists():
                folder = self.studios / f"paint-studio-{secrets.token_hex(3)}"
            commit = run(["git", "-C", REPO, "rev-parse", "HEAD"], capture_output=True, text=True, check=True).stdout.strip()
            env = {k: v for k, v in os.environ.items() if k not in ("STUDIO_WIDTH", "EASEL_WIDTH", "EASEL_BOX")}
            run([REPO / "scripts/export_r16_studio", BOX, folder], env={**env, "R16_BRANCH": commit}, check=True)

            cards = self.cards(sid)
            notebook = (home / "notebook.md").read_text() if (home / "notebook.md").exists() else ""
            (folder / "notebook.md").write_text(notebook)
            (folder / "walls").mkdir()
            for card in cards:
                shutil.copy2(home / "walls" / f"{card['number']}.png", folder / "walls" / f"{card['number']}.png")
            if cards:
                shutil.copy2(home / "walls/list.md", folder / "walls/list.md")
            brief = render_brief(folder, theme, later=bool(cards))
            (folder / "BRIEF.md").write_text(brief)
            write(home / "notebook" / f"{n:03d}-before.md", notebook)

            work = self.root / "works" / f"{sid}-{n:03d}"
            rd = work / "run" / info["lane"]
            rd.mkdir(parents=True)
            (work / "BRIEF.md").write_text(brief)
            write_json(work / "run/studios.json", {f"{info['lane']}1": folder.name})
            write_json(work / "manifest.json", {
                "studio": sid, "number": n, "folder": str(folder), "theme": theme, "model": info["model"],
                "thinking": info["thinking"], "harness": info["harness"], "commit": commit,
                "easel_sha256": hashlib.sha256((folder / "bin/easel").read_bytes()).hexdigest(),
                "width": WIDTH, "box": BOX, "started": now()})
            if info["harness"] == "agy":
                self.prepare_flash(folder, work, rd, n)
            else:
                write(work / "launch.plist", plistlib.dumps(self.launch_agent(sid, n, work)))
            with locked(self.root / ".git-lock"):
                studios = json.loads((self.root / "studios.json").read_text()) if (self.root / "studios.json").exists() else {}
                if sid not in studios:
                    studios[sid] = {"name": info["name"], "model": info["model"], "harness": info["harness"],
                                    "lane": info["lane"], "since": now()[:10], "slug": sid}
                    write_json(self.root / "studios.json", studios)
        self.history(f"{sid}: painting {n} prepared in {folder.name} ({f'theme: {theme}' if theme else 'free brief'}, {commit[:7]})")
        if info["harness"] == "pi" and not prepare:
            self.load(work)
            self.history(f"{sid}: painting {n} launch agent {LABEL} loaded")
        return work

    def prepare_flash(self, folder, work, rd, n):
        """agy's configuration in the studio, the pre-armed runner record and HAND-LAUNCH.md outside the studio."""
        prepare_agy()(folder, self.sessions, agent=TEXTS / "easel-painter.md", sitting_message=TEXTS / "sitting_message.md",
                      completion_message=TEXTS / "completion_message.md", notebook=True,
                      env={"RAYON_NUM_THREADS": RAYON_THREADS})
        feed = json.loads((folder / ".agents/recording.json").read_text())["output"]
        write_json(rd / "p1_sittings.json", [{"sitting": 1, "part": 1, "status": "running", "chunks_before": 0,
                                              "painting_before": 0, "message": text("launch_message.md"), "sessions": [feed]}])
        finish = shlex.join(str(p) for p in ["uv", "run", "--script", HERE / "atelier.py", *self.options(), "finish", "gemini-3-8-flash"])
        write(work / "HAND-LAUNCH.md", (HERE / "hand-launch.md").read_text()
              .replace("@STUDIO@", str(folder)).replace("@FOLDER@", folder.name).replace("@NUMBER@", str(n))
              .replace("@MODEL@", AGY_MODEL).replace("@FEED@", feed).replace("@FINAL@", str(rd / "p1_final.txt"))
              .replace("@FINISH@", finish).replace("@HELLO@", text("hello.md")).replace("@LAUNCH@", text("launch_message.md")))

    def load(self, work):
        agent = Path.home() / "Library/LaunchAgents" / f"{LABEL}.plist"
        domain = f"gui/{os.getuid()}"
        run(["launchctl", "bootout", f"{domain}/{LABEL}"], capture_output=True)
        shutil.copy2(work / "launch.plist", agent)
        run(["launchctl", "bootstrap", domain, agent], check=True)

    def reply_and_sittings(self, sid, work, folder, rd):
        """The painter's closing reply and its number of sittings, once it is done; Refused before."""
        if STUDIOS[sid]["harness"] == "pi":
            outcome = json.loads((rd / "p1_outcome.json").read_text()) if (rd / "p1_outcome.json").exists() else {}
            if outcome.get("status") != "finished":
                raise Refused(f"{sid}: the painter isn't done ({outcome.get('reason') or 'no outcome yet'})")
            sittings = json.loads((rd / "p1_sittings.json").read_text())
            return (rd / "p1_final.txt").read_text(), len({s["sitting"] for s in sittings if s.get("status") == "completed"})
        state = json.loads((folder / "out/easel/sittings.json").read_text()) if (folder / "out/easel/sittings.json").exists() else {}
        if state.get("complete") is not True:
            raise Refused(f"{sid}: the painting isn't complete ({folder / 'out/easel/sittings.json'})")
        records = json.loads((rd / "p1_sittings.json").read_text())
        reply = (rd / "p1_final.txt").read_text() if (rd / "p1_final.txt").exists() else house().session_reply(records[-1]["sessions"])
        if not reply:
            raise Refused(f"{sid}: no closing reply in {records[-1]['sessions']}; {rd / 'p1_final.txt'} can hold it")
        if records[-1].get("status") == "running":
            records[-1].update(status="completed", end=datetime.now().strftime("%F %T"), final=reply, final_file=str(rd / "p1_final.txt"))
            write_json(rd / "p1_sittings.json", records)
        write(rd / "p1_final.txt", reply)
        reason = f"the painter is done: sitting {state['sitting']} added no paint and had a whole-canvas look and a detail crop"
        write_json(rd / "p1_outcome.json", {"status": "finished", "reason": reason})
        return reply, state["sitting"]

    def finish(self, sid):
        from PIL import Image
        home = self.home(sid)
        waiting = self.on_easel(sid)
        if not waiting:
            raise Refused(f"{sid}: no painting on the easel")
        work = waiting[-1]
        manifest = json.loads((work / "manifest.json").read_text())
        folder, n = Path(manifest["folder"]), manifest["number"]
        rd = work / "run" / STUDIOS[sid]["lane"]
        if run(["git", "-C", self.data, "rev-parse", "--is-inside-work-tree"], capture_output=True).returncode:
            raise Refused(f"{self.data} isn't a git clone of stillwet-runs-data")
        reply, sittings = self.reply_and_sittings(sid, work, folder, rd)
        log = folder / "paintings/lua/painting.lua"
        final = work / "final"
        with locked(self.root / ".finish-lock"):
            if (folder / "bin/easel").exists():
                run([folder / "bin/easel", "close"], cwd=folder, capture_output=True, timeout=120)
            with tempfile.TemporaryDirectory(prefix="atelier-finish-") as scratch:
                checked = run([REPO / "scripts/check_painting", folder, Path(scratch) / "check"], capture_output=True, text=True)
                (work / "check.log").write_text(checked.stdout + checked.stderr)
                result = ([line for line in checked.stdout.splitlines() if line.startswith("check:")] or ["check: no result line"])[-1]
                if checked.returncode:
                    raise Refused(f"{sid}: painting {n} didn't pass the replay check ({result}; {work / 'check.log'})")
                final.mkdir(exist_ok=True)
                run([REPO / "scripts/finish_painting", log, final / "color.png"], check=True, capture_output=True)
                (final / "color.lua").unlink(missing_ok=True)
                saves = (folder / "out/easel/painting" / name for name in ("live.ckpt", "live.ckpt.zst", "live.ckpt.gz"))
                save = next((p for p in saves if p.exists()), None)
                if save is None:
                    raise Refused(f"{sid}: no saved end state in {folder / 'out/easel/painting'} for gallery and relief")
                if save.suffix == ".gz":
                    plain = Path(scratch) / "live.ckpt"
                    plain.write_bytes(gzip.decompress(save.read_bytes()))
                    save = plain
                easel = run([REPO / "scripts/replay_easel"], capture_output=True, text=True, check=True).stdout.strip()
                for mode in ("gallery", "relief"):
                    run([easel, "render-checkpoint", save, final / f"{mode}.png", "--mode", mode, "--log", log], check=True, capture_output=True)
            color = Image.open(final / "color.png")
            canvas = list(color.size)
            wall = color.convert("RGB").resize((WALL_WIDTH, round(color.height * WALL_WIDTH / color.width)), Image.LANCZOS)

        with locked(home / "lock"):
            notebook = (folder / "notebook.md").read_text() if (folder / "notebook.md").exists() else ""
            write(home / "notebook" / f"{n:03d}-after.md", notebook)
            write(home / "notebook.md", notebook)
            if (folder / "out/easel/notebook-revisions.jsonl").exists():
                write(home / "notebook" / f"{n:03d}-revisions.jsonl", (folder / "out/easel/notebook-revisions.jsonl").read_bytes())
            number = len(self.cards(sid)) + 1
            png = home / "walls" / f".{number}.png.tmp"
            png.parent.mkdir(parents=True, exist_ok=True)
            wall.save(png, "PNG")
            png.replace(home / "walls" / f"{number}.png")
            finished = now()
            write_json(home / "cards" / f"{n:03d}.json", {"number": number, "title": title_of(reply), "words": reply,
                                                          "folder": folder.name, "finished": finished})
            write(home / "walls/list.md", render_walls(self.cards(sid)))
            shutil.copy2(folder / "notes/journal.md", work / "journal.md")
            shutil.copy2(log, work / "painting.lua")
            manifest.update(finished=finished, sittings=sittings, canvas=canvas, check=result)
            write_json(work / "manifest.json", manifest)
        self.history(f"{sid}: painting {n} hung as wall {number} ({result.split(':')[1].strip()} replay check)")
        self.commit(sid, n, work)
        return work

    def commit(self, sid, n, work):
        """The painting's text records into the stillwet-runs-data clone, at their stillwet-runs paths, with the home
        folder written as ~ (the clone's records are redacted so); no images."""
        home = self.home(sid)
        records = [self.root / "studios.json", self.root / "history.md", home / "notebook.md", home / "walls/list.md",
                   *sorted((home / "notebook").glob(f"{n:03d}-*")), *sorted((home / "cards").glob("*.json")),
                   *sorted(p for p in work.rglob("*") if p.is_file() and p.suffix != ".png")]
        target = self.data / self.root.parent.name / self.root.name
        with locked(self.root / ".git-lock"):
            for path in records:
                copy = target / path.relative_to(self.root)
                copy.parent.mkdir(parents=True, exist_ok=True)
                copy.write_bytes(path.read_bytes().replace(str(Path.home()).encode(), b"~"))
            run(["git", "-C", self.data, "add", "--", target], check=True, capture_output=True)
            run(["git", "-C", self.data, "commit", "-q", "-m", f"atelier: {sid} painting {n} hangs"], check=True, capture_output=True)


def with_paths(ap):
    ap.add_argument("--root", default="~/src/a/stillwet-runs/atelier")
    ap.add_argument("--studios", default="~/src/a")
    ap.add_argument("--data", default="~/src/a/stillwet-runs-data")
    ap.add_argument("--sessions", default="~/.pi/agent/sessions")
    return ap


def parser():
    ap = with_paths(argparse.ArgumentParser(description="The atelier's painter side: start and finish a studio's painting."))
    sub = ap.add_subparsers(dest="command", required=True)
    start = sub.add_parser("start")
    start.add_argument("studio", choices=list(STUDIOS))
    start.add_argument("--theme")
    start.add_argument("--prepare", action="store_true", help="prepare the folder and records; load no launch agent")
    finish = sub.add_parser("finish")
    finish.add_argument("studio", choices=list(STUDIOS))
    return ap


def main(argv=None):
    a = parser().parse_args(argv)
    atelier = Atelier(a.root, a.studios, a.data, a.sessions)
    try:
        work = atelier.start(a.studio, a.theme, a.prepare) if a.command == "start" else atelier.finish(a.studio)
    except Refused as why:
        print(f"atelier: {why}", file=sys.stderr)
        return 1
    print(work)
    return 0


if __name__ == "__main__":
    sys.exit(main())
