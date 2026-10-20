# /// script
# requires-python = ">=3.10"
# dependencies = ["pillow"]
# ///
"""The studio: watch a painter paint, live or replayed.

Reads a painter's pi session logs (~/.pi/agent/sessions/<folder>/*.jsonl, one
per sitting, stitched into one timeline):
every program write and edit, every command, every image the painter
looked at (the log holds the image bytes it saw) and its words. Serves a
page with the code on the left, the latest look on the right and a
timeline to scrub. Read-only: it never touches the painter.

    uv run studio/studio.py            # then open http://localhost:8765
    uv run studio/studio.py --port 9000
    uv run studio/studio.py --host "$(tailscale ip -4)"   # from your other devices
"""
import argparse
import base64
import glob
import hashlib
import http.server
import io
import json
import os
import re
import threading
import urllib.parse
from collections import OrderedDict
from datetime import datetime
from PIL import Image
from palette_board import palette_board

SESSIONS = os.path.expanduser("~/.pi/agent/sessions")
EXCLUDE_SESSIONS = set()
STREAM_EPOCH = 0
DISPLAY_LIGHTS = ("gallery", "relief")
# --public: painters only, nothing that names this machine's owner (for a link shown to others)
PUBLIC = False
HOME = os.path.expanduser("~")
USER = os.path.basename(HOME)


def scrub(body):
    """The home folder as ~ and the account name as `user`, in a response's text."""
    return re.sub(re.escape(USER.encode()), b"user", body.replace(HOME.encode(), b"~"), flags=re.I)
HERE = os.path.dirname(os.path.abspath(__file__))
VIEWER_ASSETS = {
    "stream.css": "text/css; charset=utf-8",
    "code-display.js": "text/javascript; charset=utf-8",
    "code-format.js": "text/javascript; charset=utf-8",
    "vendor/stylua/stylua_lib_web.js": "text/javascript; charset=utf-8",
    "vendor/stylua/stylua_lib_bg.wasm": "application/wasm",
    "vendor/stylua/LICENSE.md": "text/plain; charset=utf-8",
}
_cache = {}  # path -> {"offset", "events", "images", "calls"}: one session file, parsed so far
_streams = {}  # key -> a stitched stream of session files (see stream())
_locks = {}
_locks_lock = threading.Lock()
_image_cache = OrderedDict()
_image_cache_bytes = 0
IMAGE_CACHE_BYTES = 64 * 1024 * 1024
_image_cache_lock = threading.Lock()


def _lock(key):
    with _locks_lock:
        return _locks.setdefault(key, threading.Lock())


_models = {}  # path -> the painter's model (it doesn't change within a session)


def session_model(path):
    """The model a session ran on: its first model_change entry (or the first
    assistant message's model), read from the top of the log once."""
    if path not in _models:
        m = ""
        try:
            with open(path, "rb") as fh:
                for _, line in zip(range(200), fh):
                    try:
                        d = json.loads(line)
                    except ValueError:
                        continue
                    if d.get("type") == "model_change" and d.get("modelId"):
                        m = d["modelId"]
                        break
                    msg = d.get("message") or {}
                    if msg.get("role") == "assistant" and msg.get("model"):
                        m = msg["model"]
                        break
        except OSError:
            pass
        if not m:
            return ""  # not written yet: ask again next time
        _models[path] = m.split("/")[-1]
    return _models[path]


# Painters' sessions, by folder: round 16+ studios, the round 14-15 chain painters and the
# earlier rounds' painter worktrees (rounds 1-4 and 7-13). Everything else (judges, tests, work on the project)
# shows only with "all".
_thinking = {}  # path -> the session's thinking level


def session_thinking(path):
    """The thinking level a session ran at: its first thinking_level_change entry."""
    if path not in _thinking:
        t = ""
        try:
            with open(path, "rb") as fh:
                for _, line in zip(range(50), fh):
                    try:
                        d = json.loads(line)
                    except ValueError:
                        continue
                    if d.get("type") == "thinking_level_change":
                        t = d.get("thinkingLevel", "")
                        break
        except OSError:
            pass
        if not t:
            return ""
        _thinking[path] = t
    return _thinking[path]


# the runners' studio lists: ~/tmp/gallery-*/r<round>/run/studios.json, {"<lane><n>": "paint-studio-..."}
RUNS = os.path.expanduser("~/src/a/stillwet-runs/**/run/studios.json")
_lanes = {"at": 0.0, "map": {}}


def runner_paths():
    return sorted(set(glob.glob(os.path.expanduser("~/tmp/gallery-*/r*/run/studios.json")) +
                      glob.glob(RUNS, recursive=True)))


def runner_key(path, key):
    for cut in range(len(key) - 1, 0, -1):
        if key[cut:].isdigit() and os.path.isdir(os.path.join(os.path.dirname(path), key[:cut])):
            return key[:cut], key[cut:]
    return re.match(r"^(.*?)(\d*)$", key).groups()


def lanes():
    """paint-studio-... -> (round, lane, painter number, painters in the lane, outcome, the title in the closing
    reply the runner copied to p<n>_final.txt once there is an outcome), from the runners' studio lists (reread
    every 30 s)."""
    import time
    if time.time() - _lanes["at"] > 30:
        m = {}
        for f in runner_paths():
            rnd = os.path.basename(os.path.dirname(os.path.dirname(f)))  # r17, r18g, r19, ...
            try:
                names = json.load(open(f))
            except (OSError, ValueError):
                continue
            keys = {k: runner_key(f, k) for k in names}
            for k, studio in names.items():
                lane, n = keys[k]
                size = sum(1 for l, _ in keys.values() if l == lane)
                number = int(n) if n else 0
                rd = os.path.join(os.path.dirname(f), lane)
                outcome = None
                try:
                    with open(os.path.join(rd, f"p{number}_outcome.json")) as fh:
                        candidate = json.load(fh)
                    if (isinstance(candidate, dict) and candidate.get("status") in
                            ("finished", "cap_reached", "crash_limit_reached") and
                            isinstance(candidate.get("reason"), str)):
                        outcome = candidate
                except (OSError, ValueError):
                    pass
                if outcome is None:
                    try:
                        with open(os.path.join(rd, f"p{number}.painted")) as fh:
                            reason = fh.read()
                        if "NOT FINISHED" in reason and "MAX_SITTINGS" in reason:
                            outcome = {"status": "cap_reached", "reason": reason.strip()}
                        elif re.search(r"the painter is done: sitting \d+ added no painting", reason):
                            outcome = {"status": "finished", "reason": reason.strip()}
                    except OSError:
                        pass
                title = None
                if outcome:
                    try:
                        with open(os.path.join(rd, f"p{number}_final.txt"), errors="replace") as fh:
                            title = title_of(fh.read())
                    except OSError:
                        pass
                m[studio] = (rnd, lane, number, size, outcome, title)
        _lanes.update(at=time.time(), map=m)
    return _lanes["map"]


_subjects = {}  # studio -> (BRIEF.md mtime, what its brief says: see about_brief)


def _artist_of(brief):
    """The artist a brief names, in full ("in the manner of Kendric Tonn" -> "Kendric Tonn"), or ""."""
    m = re.search(r"in\s+the\s+manner\s+of\s+(?:(?:early|later|late)\s+)?"
                  r"((?:[A-Z][\w.'-]*\s+)*[A-Z][\w.'-]*)", brief)
    if not m:
        m = re.search(r"explore\s+((?:[A-Z][\w.-]*\s+)*[A-Z][\w.-]*)['’]s\s+materials", brief)
    return " ".join(m.group(1).split()) if m else ""


def _subject_of(brief):
    """The artist a brief names ("in the manner of Edward Hopper" -> "Hopper"; a surname like "Alma-Tadema"
    whole), "self-portrait" for one, else "free"."""
    artist = _artist_of(brief)
    if artist:
        return artist.split()[-1]
    if "self-portrait" in brief.lower():
        return "self-portrait"
    return "free"


def _reference_artist(brief):
    """The artist whose paintings the studio's reference/ folder holds, when the brief says so ("Pictures of his
    paintings are in reference/", "A picture of one of his paintings is in reference/"), else ""."""
    said = re.search(r"\b(?:his|her|their)\s+paintings?\b[^.]*?\breference/", brief)
    return _artist_of(brief) if said else ""


def about_brief(folder):
    """What the studio's brief says: {"subject": the artist's surname (e.g. "Friedrich", "Hopper"), "self-portrait"
    or "free", "artist": the artist's full name or "", "reference_artist": the artist whose paintings are in
    reference/ or ""}; all "" without a brief."""
    f = os.path.join(os.path.expanduser("~/src/a"), folder, "BRIEF.md")
    try:
        mt = os.path.getmtime(f)
    except OSError:
        return {"subject": "", "artist": "", "reference_artist": ""}
    if folder not in _subjects or _subjects[folder][0] != mt:
        with open(f, errors="replace") as fh:
            brief = fh.read(4000)
        _subjects[folder] = (mt, {"subject": _subject_of(brief), "artist": _artist_of(brief),
                                  "reference_artist": _reference_artist(brief)})
    return _subjects[folder][1]


def subject(folder):
    """The artist the studio's brief names (e.g. "Friedrich", "Hopper"), "self-portrait", else "free" ("" without
    a brief)."""
    return about_brief(folder)["subject"]


def in_reference(path, cwd):
    """A file the painter read is in its studio's reference/ folder: a picture given to it to study, not its canvas.
    path as the read tool got it (relative to the studio, absolute or with ~), cwd the studio's folder."""
    if not path:
        return False
    p = os.path.normpath(os.path.expanduser(path))
    if os.path.isabs(p):
        if not cwd:
            return False
        p = os.path.relpath(p, os.path.normpath(cwd))
    parts = p.split(os.sep)
    return len(parts) > 1 and parts[0].lower() == "reference"


PAINTER = re.compile(r"^(paint-studio-[0-9a-f]+|paint-r\d+-p\d+|claude-paint-r\d+-(arm\d|tree\d|astra|fable|flash|p\d|python)"
                     r"|claude-paint-amnesia-(winter|coast|mountains)|claude-paint-easel[34]-(free|green|near))$")
# a folder whose session files are separate painters (round 1's three painters ran in one worktree): each its own
# painter, named folder-<subject>, by the start of its session file's name
SPLIT = {"claude-paint-fresh": {"claude-paint-fresh-winter": "2026-09-22T21-59-38-550Z",
                                "claude-paint-fresh-coast": "2026-09-22T21-59-38-576Z",
                                "claude-paint-fresh-mountains": "2026-09-22T21-59-38-610Z"}}


def short(d):
    """A session folder's short name: --Users-<you>-src-a-paint-studio-cfa19c-- -> paint-studio-cfa19c."""
    return d.strip("-").split("-src-a-")[-1]


def list_sessions():
    """One entry per painter folder (its sittings stitched: {"p": short name}) and, for
    everything else, one per session file ({"s": path}); newest activity first."""
    files = {}
    for f in glob.glob(os.path.join(SESSIONS, "*paint*", "*.jsonl")):
        if not session_allowed(f):
            continue
        files.setdefault(os.path.basename(os.path.dirname(f)), []).append(f)
    out = []
    for d, fs in files.items():
        name = short(d)
        fs.sort(key=os.path.basename)  # session file names begin with the start time
        info = [{"path": f, "model": session_model(f), "mtime": os.path.getmtime(f), "size": os.path.getsize(f)} for f in fs]
        if name in SPLIT:
            for p, start in SPLIT[name].items():
                for i in info:
                    if os.path.basename(i["path"]).startswith(start):
                        out.append({"p": p, "folder": p, "painter": True, "model": i["model"], "sittings": 1,
                                    "thinking": session_thinking(i["path"]), **about_brief(name),
                                    "round": "", "lane": "", "n": 0, "chain": 0, "outcome": None,
                                    "files": [i["path"]], "mtime": i["mtime"], "size": i["size"]})
            continue
        if PAINTER.match(name):
            numbers = sitting_numbers(fs)
            models = []
            for i in info:
                if i["model"] and i["model"] not in models:
                    models.append(i["model"])
            rnd, lane, n, size, outcome, _ = lanes().get(name, ("", "", 0, 0, None, None))
            out.append({"p": name, "folder": name, "painter": True, "model": " → ".join(models),
                        "sittings": max(numbers, default=0), "sitting_numbers": numbers,
                        "thinking": session_thinking(fs[-1]), **about_brief(name),
                        "round": rnd, "lane": lane, "n": n, "chain": size if size > 1 else 0,
                        "outcome": outcome,
                        "files": fs, "mtime": max(i["mtime"] for i in info), "size": sum(i["size"] for i in info)})
        else:
            out += [dict(i, s=i["path"], folder=name, painter=False) for i in info]
    out.sort(key=lambda s: -s["mtime"])
    return out


def session_allowed(path):
    return os.path.basename(os.path.realpath(path)) not in EXCLUDE_SESSIONS


def painter_files(name):
    """The session files of a painter folder (given by its short name), in start order."""
    if not name or "/" in name or name.startswith("."):
        return []
    for folder, ps in SPLIT.items():
        if name in ps:
            return sorted(f for f in glob.glob(os.path.join(SESSIONS, "*-src-a-" + folder + "--", ps[name] + "*.jsonl")) if session_allowed(f))
    for d in glob.glob(os.path.join(SESSIONS, "*" + name + "*")):
        if short(os.path.basename(d)) == name:
            return sorted((f for f in glob.glob(os.path.join(d, "*.jsonl")) if session_allowed(f)), key=os.path.basename)
    return []


def _text(content):
    if isinstance(content, str):
        return content
    return "\n".join(x.get("text", "") for x in content if isinstance(x, dict) and x.get("type") == "text")


def parse(path):
    """Parse the log incrementally; returns (events, images)."""
    with _lock("f:" + path):
        return _parse(path)


def _parse(path):
    # changed: indices of events whose result came in after them, in arrival order (see stream())
    c = _cache.setdefault(path, {"offset": 0, "events": [], "images": [], "lights": {light: {} for light in DISPLAY_LIGHTS}, "display": {}, "calls": {}, "changed": [], "cwd": ""})
    stat = os.stat(path)
    size = stat.st_size
    stamp = (stat.st_dev, stat.st_ino, stat.st_mtime_ns)
    if size < c["offset"] or c.get("stamp") and (stamp[:2] != c["stamp"][:2] or size <= c.get("size", 0) and stamp != c["stamp"]):
        c.update(offset=0, events=[], images=[], lights={light: {} for light in DISPLAY_LIGHTS}, display={}, calls={}, changed=[], cwd="")
        c["generation"] = c.get("generation", 0) + 1
        c.pop("sig", None)
    c.update(stamp=stamp, size=size)
    for at, line in session_lines(path, c):
        try:
            d = json.loads(line)
        except ValueError:
            continue
        ts = d.get("timestamp", "")
        if d.get("type") == "session":
            c["cwd"] = d.get("cwd", "")
            c["events"].append({"ts": ts, "kind": "start", "cwd": c["cwd"]})
            continue
        if d.get("type") == "model_change":
            c["events"].append({"ts": ts, "kind": "note", "text": "model " + d.get("modelId", "")})
            continue
        if d.get("type") == "compaction":
            clock = summary_clock(d.get("summary", ""))
            if clock:
                c["events"].append({"ts": ts, "kind": "clock", "clock": clock, "text": clock})
            continue
        if d.get("type") != "message":
            continue
        m = d.get("message", {})
        role, content = m.get("role"), m.get("content")
        if role == "assistant" and isinstance(content, list):
            for x in content:
                t = x.get("type")
                if t == "text" and x.get("text", "").strip():
                    c["events"].append({"ts": ts, "kind": "say", "text": x["text"]})
                elif t == "thinking" and x.get("thinking", "").strip():
                    c["events"].append({"ts": ts, "kind": "think", "text": x["thinking"]})
                elif t == "toolCall":
                    name, a = x.get("name"), x.get("arguments", {}) or {}
                    ev = {"ts": ts, "kind": "tool", "name": name, "id": x.get("id")}
                    if name == "write":
                        ev.update(kind="write", path=a.get("path", ""), content=a.get("content", ""))
                    elif name == "edit":
                        edits = a.get("edits") or [{"oldText": a.get("oldText", ""), "newText": a.get("newText", "")}]
                        ev.update(kind="edit", path=a.get("path", ""), edits=edits)
                    elif name in ("bash", "bg_start"):
                        ev.update(kind="cmd", text=a.get("command", ""))
                    elif name == "read":
                        ev.update(kind="read", path=a.get("path", ""))
                        if in_reference(ev["path"], c["cwd"]):
                            ev["ref"] = True  # it reads from reference/ (see in_reference)
                    elif name == "paint":  # the painter harness's easel tools (round 19 on)
                        ev.update(kind="paint", code=a.get("lua", ""))
                        if a.get("scratch"):  # on the scratch canvas beside the painting
                            ev["scratch"] = True
                            if a.get("new_scratch"):
                                ev["new_scratch"] = True
                    elif name == "look":
                        ev.update(kind="look", text=look_text(a))
                    elif name == "note":
                        ev.update(kind="jnote", text=a.get("text", ""))
                    elif name == "notebook":
                        ev.update(kind="notebook", text=a.get("text", ""))
                    elif name == "ask_critic":
                        ev.update(kind="critic", text=a.get("note", ""))
                    else:
                        ev.update(text=json.dumps(a)[:300])
                    c["calls"][x.get("id")] = len(c["events"])
                    c["events"].append(ev)
        elif role == "toolResult" and isinstance(content, list):
            parent = c["calls"].get(m.get("toolCallId"))
            txt = _text(content)
            if parent is not None and (txt or m.get("isError")):
                ev = c["events"][parent]
                if ev["kind"] == "paint" and ev.get("scratch") and re.search(
                        r"^a (?:fresh scratch canvas|scratch canvas beside the painting)|\(the scratch canvas couldn't be reopened, so it is put aside\)", txt, re.M):
                    ev["scratch_reset"] = True
                if txt:
                    ev["out"] = txt if ev["kind"] == "critic" else txt[-1500:]
                    if ev["kind"] == "look" and STICK_HELD.search(txt):
                        ev["held"] = "stick"
                if m.get("isError"):
                    ev["err"] = True
                c["changed"].append(parent)
            picture_paths = re.findall(r"^(.+\.png)(?: \(.*)?$", txt, re.MULTILINE)
            picture_n = 0
            for x in content:
                if x.get("type") == "image" and x.get("data"):
                    idx = len(c["images"])
                    c["images"].append((path, at, picture_n))
                    src = c["events"][parent].get("path", "") if parent is not None else ""
                    ev = {"ts": ts, "kind": "image", "img": idx, "path": src}
                    display_path = ""
                    if parent is not None and c["events"][parent]["kind"] == "look":
                        ev["look"] = c["events"][parent]["text"]  # what the painter asked to see (see is_whole)
                        if c["events"][parent].get("held"):
                            ev["held"] = c["events"][parent]["held"]
                        if picture_n < len(picture_paths):
                            display_path = picture_paths[picture_n]
                    elif parent is not None and c["events"][parent]["kind"] == "read" and not c["events"][parent].get("ref"):
                        display_path = src
                    if display_path and display_status(c["cwd"], display_path) is not None:
                        ev["display_pending"] = True
                        c["display"][idx] = (len(c["events"]), display_path)
                    for light in DISPLAY_LIGHTS:
                        lit = display_companion_path(c["cwd"], display_path, light) if display_path else None
                        if lit:
                            c["lights"][light][idx] = (lit, None, None)
                            ev[light] = True
                    picture_n += 1
                    if re.search(r"palette (?!False)", ev.get("look", "")):
                        chips = palette_board(base64.b64decode(x["data"]))
                        if chips:
                            ev["palette"] = chips
                    if parent is not None and c["events"][parent].get("ref"):
                        ev["ref"] = True  # a reference picture: never the painting
                    c["events"].append(ev)
        elif role == "user":
            t = _text(content)
            if t.strip():
                ev = {"ts": ts, "kind": "user", "text": t[:2000]}
                clock = summary_clock(t)  # the recovery summary can be longer than the displayed text
                if clock:
                    ev["clock"] = clock
                c["events"].append(ev)
    for idx, (event, display_path) in list(c["display"].items()):
        status = display_status(c["cwd"], display_path)
        if status == b"failed\n":
            del c["display"][idx]
            continue
        paths = {light: display_companion_path(c["cwd"], display_path, light) for light in DISPLAY_LIGHTS}
        if status == b"ready\n" and all(paths.values()):
            for light, companion in paths.items():
                c["lights"][light][idx] = (companion, None, None)
                c["events"][event][light] = True
            c["events"][event]["display_pending"] = False
            c["changed"].append(event)
            del c["display"][idx]
    if not any(e["kind"] == "look" for e in c["events"]):
        mark_whole(c, path)
    return c["events"], c["images"]


def session_lines(path, c):
    with open(path, "rb") as fh:
        fh.seek(c["offset"])
        while True:
            at = fh.tell()
            line = fh.readline()
            if not line.endswith(b"\n"):
                return
            c["offset"] = fh.tell()
            yield at, line


def display_status(cwd, path):
    if not cwd or not path:
        return None
    original = os.path.realpath(os.path.join(cwd, path))
    if not original.startswith(os.path.realpath(cwd) + os.sep):
        return None
    marker = os.path.join(os.path.dirname(original), ".display", os.path.basename(original))
    try:
        with open(marker, "rb") as fh:
            return fh.read(16)
    except OSError:
        return b"pending\n" if os.path.lexists(marker) else None


def display_companion_path(cwd, path, light):
    if not cwd or light not in DISPLAY_LIGHTS:
        return None
    original = os.path.realpath(os.path.join(cwd, path))
    companion = os.path.realpath(os.path.join(os.path.dirname(original), "." + light, os.path.basename(original)))
    if not companion.startswith(os.path.realpath(cwd) + os.sep):
        return None
    if display_status(cwd, path) not in (None, b"ready\n"):
        return None
    try:
        with open(companion, "rb") as fh:
            header = fh.read(8)
        return companion if header == b"\x89PNG\r\n\x1a\n" else None
    except OSError:
        return None


def display_companion(cwd, path, light):
    companion = display_companion_path(cwd, path, light)
    if companion:
        with open(companion, "rb") as fh:
            return fh.read()
    return None


def _signature(data):
    """A picture's proportions, its 32 x 24 grays (centered, scaled to unit spread) and its mean chroma (0-255;
    about 0 for a check in grays); None if it can't be read."""
    from PIL import Image
    import io
    try:
        with Image.open(io.BytesIO(base64.b64decode(data))) as im:
            w, h = im.size
            rgb = list(im.convert("RGB").resize((32, 24)).getdata())
    except Exception:
        return None
    px = [0.299 * r + 0.587 * g + 0.114 * b for r, g, b in rgb]
    m = sum(px) / len(px)
    sd = (sum((v - m) ** 2 for v in px) / len(px)) ** 0.5 or 1.0
    chroma = sum(max(c) - min(c) for c in rgb) / len(rgb)  # 0 for a picture in grays
    return w / h, [(v - m) / sd for v in px], chroma


# mark_whole's answers, by session file (its path, size and number of pictures), kept between runs: the export
# runs every minute and decoding every early painter's pictures takes minutes
WHOLE_CACHE = os.path.expanduser("~/.cache/claude-paint-studio/whole.json")
_whole = {}


def _whole_cache():
    if "d" not in _whole:
        try:
            with open(WHOLE_CACHE) as fh:
                _whole["d"] = json.load(fh)
        except (OSError, ValueError):
            _whole["d"] = {}
    return _whole["d"]


def _save_whole_cache():
    os.makedirs(os.path.dirname(WHOLE_CACHE), exist_ok=True)
    tmp = f"{WHOLE_CACHE}.{os.getpid()}.tmp"
    with open(tmp, "w") as fh:
        json.dump(_whole["d"], fh)
    os.replace(tmp, WHOLE_CACHE)


def mark_whole(c, path):
    """Before the look tool a painter opened its renders as image files, the whole canvas and close-ups alike; the
    file's name doesn't say which. A picture is the whole canvas (e["whole"]) when its proportions are the canvas's
    (the most common among its pictures, within 2%), it isn't in grays (a value check) and its 32 x 24 grays correlate at 0.75 or more with the next
    whole picture after it. The chain starts, at the end, from the picture of the canvas's proportions most like the
    others among the last eight. A close-up of the same proportions differs from the whole too much to pass; where
    the painting changed a lot between two looks (an early lay-in) the chain stops, and before that the view shows
    the last picture opened, as without this. Where the easel command that wrote a picture can be read (_look_said),
    it decides instead."""
    ims = [e for e in c["events"] if e["kind"] == "image" and not e.get("ref")]
    key = f"v5:{path}:{c['stamp']}:{os.path.getsize(path)}:{len(ims)}"
    done = _whole_cache().get(key)
    if done is not None and len(done) == len(ims):
        for e, w in zip(ims, done):
            e["whole"] = w
        return
    sig = c.setdefault("sig", {})
    for e in ims:
        if e["img"] not in sig:
            im = read_image(c["images"][e["img"]])
            sig[e["img"]] = _signature(base64.b64encode(im[1])) if im else None
    shapes = [round(sig[e["img"]][0], 2) for e in ims if sig[e["img"]]]
    if not shapes:
        return
    canvas = max(set(shapes), key=shapes.count)
    fits = [e for e in ims if sig[e["img"]] and abs(sig[e["img"]][0] / canvas - 1) < 0.02]
    # a check of the whole in grays (or blurred to grays) isn't the painting as it looks
    chromas = sorted(sig[e["img"]][2] for e in fits)
    if chromas and chromas[len(chromas) // 2] > 10:
        fits = [e for e in fits if sig[e["img"]][2] > 3]
    corr = lambda a, b: sum(x * y for x, y in zip(sig[a["img"]][1], sig[b["img"]][1])) / len(sig[a["img"]][1])
    for e in ims:
        e["whole"] = False
    if not fits:
        return
    tail = fits[-8:]
    anchor = max(tail, key=lambda a: sum(corr(a, b) for b in tail))
    anchor["whole"] = True
    later = anchor
    for e in reversed(fits[:fits.index(anchor)]):
        if corr(e, later) >= 0.75:
            e["whole"] = True
            later = e
    later = anchor
    for e in fits[fits.index(anchor) + 1:]:
        if corr(e, later) >= 0.75:
            e["whole"] = True
            later = e
    for e in ims:  # where the command that made the picture says what it showed, that decides
        said = _look_said(c["events"], e)
        if said is not None:
            e["whole"] = said
    # a round 16-18 studio painter looked through the easel (its pictures under out/easel/): a picture it made
    # elsewhere (a mirrored or cut-out copy in a scratch folder) isn't the canvas as the easel shows it. (Earlier
    # painters rendered their finals into scratch folders themselves: this doesn't apply to them.)
    if os.path.basename(c.get("cwd", "").rstrip("/")).startswith("paint-studio-"):
        for e in ims:
            if "out/easel/" not in e.get("path", ""):
                e["whole"] = False
    _whole_cache()[key] = [e["whole"] for e in ims]
    _save_whole_cache()


def _look_said(events, image):
    """Whether the easel command that wrote this picture asked for the whole canvas (rounds 16-18 called the easel
    from the shell: `easel look --crop ...`, `easel do '...' --look`): True or False from its options, None when no
    command names the file or which of several looks wrote it can't be told."""
    name = os.path.basename(image.get("path", ""))
    if not name:
        return None
    k = events.index(image)
    cmd = next((e for e in reversed(events[:k]) if e["kind"] == "cmd" and name in (e.get("out") or "")), None)
    if cmd is None:
        return None
    text, out = cmd.get("text", ""), cmd.get("out") or ""
    looks = re.findall(r"\beasel\s+look\b([^\n;&|]*)", text)
    files = re.findall(r"look-\d+\.\w+", out)
    if looks and len(files) == len(looks) and name in files:
        opts = looks[files.index(name)]
    elif looks and len(files) < len(looks):  # some looks failed: decided only if all of them agree
        wholes = {not re.search(r"--(crop|mode|light|survey|compare|hold|palette)\b", o) for o in looks}
        return wholes.pop() if len(wholes) == 1 else None
    elif not looks and re.search(r"\beasel\s+do\b", text) and re.search(r"(?m)--look\b", text) and len(files) == 1:
        opts = ""
    else:
        return None
    return not re.search(r"--(crop|mode|light|survey|compare|hold|palette)\b", opts)


def summary_clock(text):
    """The latest recorded time in the painter harness's saved clock section."""
    section = re.search(r"(?m)^## The canvas clock\n([\s\S]*?)(?=\n## |\Z)", text)
    if not section:
        return None
    times = re.findall(r"(?m)^- Latest (?:painting time the easel printed|journal entry stamped): "
                       r"(day ([1-9]\d*), ([01]\d|2[0-3]):([0-5]\d))[ \t]*$", section[1])
    return max(times, key=lambda t: tuple(map(int, t[1:])))[0] if times else None


def look_text(args):
    """A look call's request as one line: "crop 110,540,610,900, size 800", "mode squint" ("" for the plain look)."""
    return ", ".join(f"{k} {v}" for k, v in args.items())


# a look's reply when it held a pastel stick over the canvas ("<look> (WxH, 0.12s): P.glow held at 400,300: <what
# it shows>", main.rs), not a knife or a pile ("... held up to the canvas at 400,300"): the request alone can't tell,
# a stick held without pose or side taking the easel's default pose
STICK_HELD = re.compile(r"\): .+ held at -?[0-9.]+,-?[0-9.]+: ")


def is_whole(look):
    """A look request that shows the whole canvas as it is: no crop, no mode (value, squint, mirror), no light,
    not the palette, not a survey's tile, not a comparison, not a held knife or stick, not the motif pinned beside it and not
    the scratch canvas."""
    if look is None:
        return False
    look = re.sub(r"mode (?:normal|color)(?:\s*,\s*(?:normal|color))*(?=, [a-z]+ |$)", "", look)
    return not re.search(r"crop|mode|light|compare|hold|(?:^|, )(?:ref|mask) |(?:palette|survey|scratch) (?!False)", look)


# a closing reply that begins with the painting's title: "**The Silent Shore**", "### *Hünengrab im Abendlicht* (...)",
# also after an opening kaomoji ("(ᵔᴥᵔ) **The Old Willow at Evening**")
TITLE = re.compile(r"\s*(?:\([^)\n]{1,16}\)\S{0,3}\s+)?(?:#{1,6}\s*)?(\*\*?|__?)([^*_\n]{2,100}?)\1(?![*_\w])")
# or one on a line of its own a little further down ("I'm stopping here. ...\n\n**Quinces, Raking Light**\n\n...")
TITLE_LINE = re.compile(r"[ \t]*(?:#{1,6}[ \t]*)?(\*\*?|__?)([^*_\n]{2,100}?)\1[ \t]*")


# or named in the first line of its closing words: "I've called it **Grainstack, Evening**.", "I've finished
# **Nymphéas, reflet du soir** (...)", "I'm leaving **Luncheonette, Four O'Clock** finished as it is."
NAMED = re.compile(r"[^\n*_]{0,40}?\b(?:call(?:ed|ing)?|title[ds]?|name[ds]?|finish(?:ed|ing)?|left|leav(?:e|ing)"
                   r"|complet(?:ed|e|ing))(?:\s+(?:it|this|(?:the|this|my)\s+(?:painting|picture|canvas)))?\s+(\*\*?|__?)([^*_\n]{2,100}?)\1(?![*_\w])")
LABELED_TITLE = re.compile(r"^[ \t]*(?:#{1,6}[ \t]*)?(?:\*\*|__)?Title:[ \t]*"
                           r"(?:\*\*|__)?[ \t]*(?:\*\*?|__?)?([^*_\n]{2,100}?)(?:\*\*?|__?){0,2}[ \t]*$", re.M | re.I)


def clean_title(t):
    """A title without the quotation marks some painters put inside the bold."""
    return t.strip().strip('"\u201c\u201d').strip() or None


def title_of(say):
    """The painting's title from the painter's last words, if they begin with one or have one alone on a line among
    their first three paragraphs; else None."""
    m = LABELED_TITLE.search(say or "")
    if m:
        return clean_title(m.group(1))
    m = TITLE.match(say or "") or NAMED.match(say or "")
    if m:
        title = clean_title(m.group(2))
        if title and not title.endswith(":"):
            return title
    for para in [p for p in re.split(r"\n\s*\n", say or "") if p.strip()][1:3]:
        m = TITLE_LINE.fullmatch(para.strip("\n"))
        if m:
            title = clean_title(m.group(2))
            if title and not title.endswith(":"):
                return title
    return None


def title_of_closings(says):
    """The title in the newest closing words that name one (a last sitting that only looked may not repeat it)."""
    return next((t for t in map(title_of, reversed(says)) if t), None)


def runner_title(painter):
    """The title in a finished painter's closing reply as its runner recorded it, for a session that ends in later
    words (an agy painter's interview after the painting); None without a runner outcome."""
    return lanes().get(painter, (None,) * 6)[5]


REFERENCE = object()  # in a glance's calls: a read of a reference picture
_warm = None  # the server's first scan of every painter for the picker: set when done (None: scan on request)
_glances = {}  # path -> what the picker needs from one session file, read incrementally without keeping images
_thumbnails = OrderedDict()
_thumbnails_lock = threading.Lock()
GLANCE_SIZE = 360
GLANCE_CACHE_SIZE = 256


def _glance_file(path):
    """One session file, scanned from where the last scan stopped: its image count, its newest whole look and its
    last picture from before the look tool (each as (index in the file, byte offset of the line, which image in the
    line)) and its last words. Images are counted as parse() counts them, so the indices match the stream's.
    Reference pictures (read from the studio's reference/) are counted but are never the whole look or the last
    picture; nor are a look's other pictures (a survey's tile, a comparison, a held knife)."""
    with _lock("g:" + path):
        g = _glances.get(path)
        size = os.path.getsize(path)
        if not g or size < g["offset"]:
            g = _glances[path] = {"offset": 0, "calls": {}, "n": 0, "whole": None, "last": None, "whole_version": None, "last_version": None, "pending": [], "say": "", "cwd": "", "worked": False, "concepts": False}
        with open(path, "rb") as fh:
            fh.seek(g["offset"])
            while True:
                at, line = fh.tell(), fh.readline()
                if not line.endswith(b"\n"):
                    break
                g["offset"] = fh.tell()
                try:
                    d = json.loads(line)
                except ValueError:
                    continue
                if d.get("type") == "session":
                    g["cwd"] = d.get("cwd", "")
                m = (d.get("message") or {}) if d.get("type") == "message" else {}
                content = m.get("content")
                if m.get("role") == "user" and "Use write_concepts" in _text(content or []):
                    g["concepts"] = True
                if not isinstance(content, list):
                    continue
                if m.get("role") == "assistant":
                    for x in content:
                        if x.get("type") == "text" and x.get("text", "").strip():
                            g["say"] = x["text"]
                            g["worked"] = True
                        elif x.get("type") == "toolCall":  # a look's request; REFERENCE for a read from reference/
                            if x.get("name") == "write_concepts":
                                g["concepts"] = True
                            if x.get("name") and x["name"] not in ("look", "read", "status", "log"):
                                g["worked"] = True
                            a = x.get("arguments") or {}
                            g["calls"][x.get("id")] = (look_text(a) if x.get("name") == "look" else REFERENCE
                                                       if x.get("name") == "read" and in_reference(a.get("path", ""), g["cwd"])
                                                       else None)
                elif m.get("role") == "toolResult":
                    look, k = g["calls"].get(m.get("toolCallId")), 0
                    paths = re.findall(r"^(.+\.png)(?: \(.*)?$", _text(content), re.MULTILINE)
                    for x in content:
                        if x.get("type") == "image" and x.get("data"):
                            field = "last" if look is None else "whole" if look is not REFERENCE and is_whole(look) else None
                            if field:
                                version = hashlib.sha256(x["data"].encode()).hexdigest()
                                display_path = paths[k] if k < len(paths) else ""
                                candidate = (g["n"], at, k)
                                if display_status(g["cwd"], display_path) is None:
                                    g[field], g[field + "_version"] = candidate, version
                                else:
                                    g["pending"].append((field, candidate, version, display_path))
                            g["n"] += 1
                            k += 1
        for pending in list(g["pending"]):
            field, candidate, version, display_path = pending
            status = display_status(g["cwd"], display_path)
            if status == b"ready\n" and all(display_companion_path(g["cwd"], display_path, light) for light in DISPLAY_LIGHTS):
                if g[field] is None or candidate[0] > g[field][0]:
                    g[field], g[field + "_version"] = candidate, version
                g["pending"].remove(pending)
            elif status == b"failed\n":
                g["pending"].remove(pending)
        if g["concepts"]:
            g["worked"] = False
        return g


def sitting_numbers(files):
    """Each session's sitting number, or zero for concepts and startup-only sessions."""
    numbers, n = [], 0
    for f in files:
        worked = _glance_file(f)["worked"]
        n += worked
        numbers.append(n if worked else 0)
    return numbers


def glance(files):
    """A painter's picture and title for the picker: {"look": its newest whole look (or, before the look tool, the
    last picture it read; never a reference picture or a look's other pictures; None if it has seen none of its own)
    as a stream image index,
    "title": from its last words or None}, and where that look's bytes are, for /api/glance."""
    gs, base, look, src, last, lsrc = [_glance_file(f) for f in files], 0, None, None, None, None
    version, last_version = None, None
    for f, g in zip(files, gs):
        if g["whole"]:
            look, src = base + g["whole"][0], (f,) + g["whole"][1:]
            version = g["whole_version"]
        if g["last"]:
            last, lsrc = base + g["last"][0], (f,) + g["last"][1:]
            last_version = g["last_version"]
        base += g["n"]
    title = title_of_closings([g["say"] for g in gs if g["say"]])
    if look is None:
        look, src = last, lsrc
        version = last_version
    return {"look": look, "title": title, "src": src, "version": version}


def glance_image(src):
    """The bytes of the image at src (a file, a line's byte offset, which image in the line): (mime, data) or None."""
    path, at, k = src
    with open(path, "rb") as fh:
        fh.seek(at)
        d = json.loads(fh.readline())
    imgs = [x for x in d["message"]["content"] if x.get("type") == "image" and x.get("data")]
    return (imgs[k].get("mimeType", "image/jpeg"), base64.b64decode(imgs[k]["data"])) if k < len(imgs) else None


def glance_thumbnail(g):
    """A memoized JPEG of the picker's look, leaving the original image untouched."""
    im = glance_image(g["src"])
    if not im:
        return None
    key = hashlib.sha256(im[1]).digest()
    with _thumbnails_lock:
        if key not in _thumbnails:
            with Image.open(io.BytesIO(im[1])) as picture:
                picture.thumbnail((GLANCE_SIZE, GLANCE_SIZE), Image.Resampling.LANCZOS)
                buf = io.BytesIO()
                picture.convert("RGB").save(buf, "JPEG", quality=80)
            _thumbnails[key] = ("image/jpeg", buf.getvalue())
            if len(_thumbnails) > GLANCE_CACHE_SIZE:
                _thumbnails.popitem(last=False)
        _thumbnails.move_to_end(key)
        return _thumbnails[key]


def _hhmm(ts):
    try:
        return datetime.fromisoformat(ts.replace("Z", "+00:00")).astimezone().strftime("%H:%M")
    except ValueError:
        return ""


def stream(key, files):
    """The events of several session files (a painter's sittings) as one stream, in start
    order, with a {"kind": "sitting"} event before each counted sitting. Image
    indices are renumbered to be unique across files. Built incrementally: each file is
    parsed from where it stopped, and only the new events are appended. If an earlier
    part changes (it shouldn't: a sitting ends before the next begins) the stream is
    rebuilt and its epoch bumped, so clients know to start over. An event already in the
    stream whose result comes in later is listed in "updates" (its stream index, in
    arrival order), so clients that have it can fetch it again."""
    with _lock("s:" + key):
        st = _streams.get(key)
        parsed = [parse(f) for f in files]
        numbers = sitting_numbers(files)
        parts = [(f, len(ev), len(im)) for f, (ev, im) in zip(files, parsed)]
        generations = [_cache[f].get("generation", 0) for f in files]
        if st:
            old = st["parts"]
            same = (len(old) <= len(parts) and all(o[0] == n[0] for o, n in zip(old, parts))
                    and all(o == n for o, n in zip(old[:-1], parts))
                    and (not old or parts[len(old) - 1][1] >= old[-1][1] and parts[len(old) - 1][2] >= old[-1][2]))
            if (not same or st.get("generations", []) != generations[:len(old)]
                    or st.get("sitting_numbers", []) != numbers[:len(old)]):
                st = {"epoch": st["epoch"] + 1, "parts": [], "events": [], "base": [], "chg": [], "updates": []}
        else:
            st = {"epoch": STREAM_EPOCH, "parts": [], "events": [], "base": [], "chg": [], "updates": []}
        _streams[key] = st
        old, ioff = st["parts"], 0
        for k, (f, n, ni) in enumerate(parts):
            ev = parsed[k][0]
            seen = old[k][1] if k < len(old) else 0
            if seen == 0 and n and numbers[k]:
                st["events"].append({"ts": ev[0]["ts"], "kind": "sitting", "n": numbers[k],
                                     "text": f"sitting {numbers[k]} · {_hhmm(ev[0]['ts'])}"})
            if k == len(st["base"]):
                st["base"].append(len(st["events"]))  # where this file's events begin in the stream
            if k == len(st["chg"]):
                st["chg"].append(0)
            changed = _cache[f]["changed"]
            for j in changed[st["chg"][k]:]:
                if j < seen:  # already sent without its result
                    e = ev[j]
                    st["events"][st["base"][k] + j] = dict(e, img=e["img"] + ioff) if e["kind"] == "image" else e
                    st["updates"].append(st["base"][k] + j)
            st["chg"][k] = len(changed)
            for e in ev[seen:n]:
                st["events"].append(dict(e, img=e["img"] + ioff) if e["kind"] == "image" else e)
            ioff += ni
        st["parts"] = parts
        st["generations"] = generations
        st["sitting_numbers"] = numbers
        st["sittings"] = max(numbers, default=0)
        return st


# the painting sources of painters whose folder is gone (a round's worktree, removed), by folder name
ARCHIVE = os.path.join(os.path.expanduser(os.environ.get("STILLWET_PAINTINGS", "~/src/a/stillwet-paintings")),
                       "other", "claude-paint", "sources")


def painting_sources(events):
    """The folder to read the painting's source files from and those files, relative to it: the ones
    the page may ask for. The folder is the painter's (its last start) or, once that is gone,
    stillwet-paintings/other/claude-paint/sources/<its name>. The files: paintings/lua/painting.lua, paintings/lua/scratch.lua once the
    painter paints there and, from the events, every .lua
    under paintings/lua/ and .rs under paintings/ the painter wrote or edited or rendered with
    `cargo paint <bin>`. Nothing else in the folder (the brief, notes, bin/, settings) is served."""
    cwd = next((e["cwd"] for e in reversed(events) if e["kind"] == "start"), "")
    rels = {"paintings/lua/painting.lua"}
    if any(e["kind"] == "paint" and e.get("scratch") for e in events):
        rels.add("paintings/lua/scratch.lua")
    for e in events:
        if e["kind"] in ("write", "edit"):
            p = e.get("path", "")
            rels.add(p[len(cwd) + 1:] if cwd and p.startswith(cwd + "/") else p)
        m = re.search(r"cargo paint (\w+)", e.get("text", "") or "") if e["kind"] == "cmd" else None
        if m:
            rels.add(f"paintings/src/bin/{m.group(1)}.rs")
    ok = re.compile(r"paintings/(lua/[^/]+\.lua|(?:[^/]+/)*[^/]+\.rs)")
    if cwd and not os.path.isdir(cwd):
        cwd = os.path.join(ARCHIVE, os.path.basename(cwd.rstrip("/")))
    return cwd, {r for r in rels if ok.fullmatch(r) and ".." not in r.split("/")}


def read_image(src):
    global _image_cache_bytes
    if not src:
        return None
    path, at, k = src
    try:
        stat = os.stat(path)
        key = (src, stat.st_dev, stat.st_ino, stat.st_size, stat.st_mtime_ns)
        with _image_cache_lock:
            if key in _image_cache:
                _image_cache.move_to_end(key)
                return _image_cache[key]
            if at is None:
                with open(path, "rb") as fh:
                    im = ("image/png", fh.read())
            else:
                im = glance_image((path, at, k))
            if not im:
                return None
            n = len(im[1])
            if n <= IMAGE_CACHE_BYTES:
                while _image_cache and _image_cache_bytes + n > IMAGE_CACHE_BYTES:
                    _image_cache_bytes -= len(_image_cache.popitem(last=False)[1][1])
                _image_cache[key] = im
                _image_cache_bytes += n
            return im
    except (OSError, ValueError, KeyError, TypeError):
        return None


def image_bytes(st, i, light=None):
    """Image i of a stitched stream: (mime, bytes), or None."""
    if i < 0:
        return None
    for f, _, ni in st["parts"]:
        if i < ni:
            if light:
                return read_image(_cache[f]["lights"].get(light, {}).get(i))
            imgs = _cache[f]["images"]
            return read_image(imgs[i]) if i < len(imgs) else None
        i -= ni
    return None


def image(st, i, light=None):
    """Compatibility view of image i: (mime, base64 data), or None."""
    im = image_bytes(st, i, light)
    return (im[0], base64.b64encode(im[1]).decode()) if im else None


class H(http.server.BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def _send(self, code, body, ctype, cache_control=None, etag=None):
        if PUBLIC and not ctype.startswith("image/") and ctype != "application/wasm":
            body = scrub(body)
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        # a look never changes: the browser keeps it (and asks again for everything else)
        self.send_header("Cache-Control", cache_control or ("private, max-age=86400, immutable" if ctype.startswith("image/") else "no-store"))
        if etag:
            self.send_header("ETag", etag)
        if PUBLIC:
            self.send_header("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; "
                             "style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; media-src 'self' blob:")
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        u = urllib.parse.urlparse(self.path)
        q = urllib.parse.parse_qs(u.query)
        # what to show: ?p=<painter folder> (all its sittings) or ?s=<session file> (one session)
        painter, path = q.get("p", [""])[0], q.get("s", [""])[0]
        if PUBLIC and (path or (painter and not PAINTER.match(painter))):
            return self._send(404, b"not found", "text/plain")
        if path and not os.path.realpath(path).startswith(os.path.realpath(SESSIONS) + os.sep):
            return self._send(403, b"no", "text/plain")
        if u.path == "/":
            with open(os.path.join(HERE, "index.html"), "rb") as fh:
                page = fh.read()
            if PUBLIC:
                page = page.replace(b'<label id="allwrap"', b'<label id="allwrap" hidden')
            return self._send(200, page, "text/html; charset=utf-8")
        asset = u.path.lstrip("/")
        if asset in VIEWER_ASSETS:
            with open(os.path.join(HERE, asset), "rb") as fh:
                return self._send(200, fh.read(), VIEWER_ASSETS[asset])
        if u.path == "/api/sessions":
            ss = list_sessions()
            for s in ss:
                s["epoch"] = STREAM_EPOCH
                if s.get("painter") and (_warm is None or _warm.is_set()):  # (until the first scan is done, without)  # the picker's picture and title (the static export has them in sessions.json)
                    g = glance(s["files"])
                    s.update(look=g["look"], title=g["title"] or runner_title(s["p"]), thumb_version=g["version"])
            if PUBLIC:
                ss = [{k: v for k, v in s.items() if k != "files"} for s in ss if s.get("painter")]
            return self._send(200, json.dumps(ss).encode(), "application/json")
        if u.path == "/api/glance":  # a painter's picture in the picker, read alone (not the painter's whole log)
            files = painter_files(painter)
            g = glance(files) if files else None
            im = glance_thumbnail(g) if g and g["src"] else None
            if not im:
                return self._send(404, b"not found", "text/plain")
            etag = '"' + hashlib.sha256(im[1]).hexdigest() + '"'
            tags = [tag.strip().removeprefix("W/") for tag in self.headers.get("If-None-Match", "").split(",")]
            code = 304 if etag in tags or "*" in tags else 200
            return self._send(code, b"" if code == 304 else im[1], im[0], "private, max-age=300", etag)
        if u.path not in ("/api/events", "/api/file", "/img"):
            return self._send(404, b"not found", "text/plain")
        files = painter_files(painter) if painter else [path] if path and session_allowed(path) and os.path.isfile(path) else []
        if not files:
            return self._send(404, b"no such session", "text/plain")
        st = stream(("p:" + painter) if painter else ("s:" + path), files)
        if u.path == "/api/events":
            since, ev = int(q.get("since", ["0"])[0]), st["events"]
            if q.get("epoch", [str(st["epoch"])])[0] != str(st["epoch"]):
                since = 0  # the stream was rebuilt: the client starts over
            # u: how many updates the client has seen; it gets the rest as [index, event], for indices < since
            u = int(q.get("u", ["0"])[0]) if q.get("epoch", [""])[0] == str(st["epoch"]) else 0
            ups = [[i, ev[i]] for i in st["updates"][u:] if i < since]
            return self._send(200, json.dumps({"events": ev[since:], "total": len(ev), "epoch": st["epoch"],
                                               "sittings": st["sittings"], "updates": ups,
                                               "nupdates": len(st["updates"])}).encode(), "application/json")
        if u.path == "/api/file":  # the painting's current source, from the painter's folder
            cwd, rels = painting_sources(st["events"])
            rel = q.get("f", [""])[0]
            want = os.path.realpath(os.path.join(cwd, rel))
            if cwd and rel in rels and want.startswith(os.path.realpath(cwd) + os.sep) and os.path.isfile(want):
                with open(want, "rb") as fh:
                    return self._send(200, fh.read(), "text/plain; charset=utf-8")
            return self._send(404, b"", "text/plain")
        if u.path == "/img":
            light = next((light for light in DISPLAY_LIGHTS if q.get(light) == ["1"]), None)
            im = image_bytes(st, int(q.get("i", ["0"])[0]), light=light)
            if im:
                return self._send(200, im[1], im[0])
        self._send(404, b"not found", "text/plain")


def main():
    global SESSIONS, STREAM_EPOCH
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--host", default="127.0.0.1", help="e.g. this machine's Tailscale IP to watch from another of your devices")
    ap.add_argument("--sessions", default=SESSIONS, help="where pi keeps its session logs")
    ap.add_argument("--exclude-session", action="append", default=[], metavar="BASENAME", help="omit this session log from the picker and API (repeatable)")
    ap.add_argument("--public", action="store_true", help="painters only, home path and account name scrubbed (a link for others)")
    a = ap.parse_args()
    SESSIONS = os.path.abspath(os.path.expanduser(a.sessions))
    STREAM_EPOCH = int(datetime.now().timestamp() * 1_000_000)
    EXCLUDE_SESSIONS.update(a.exclude_session)
    global PUBLIC
    PUBLIC = a.public
    global _warm
    _warm = threading.Event()

    def warm():  # the picker's pictures and titles, read ahead (the first scan reads every log once: seconds)
        import time
        while True:
            for s in list_sessions():
                if s.get("painter"):
                    glance(s["files"])
            _warm.set()
            time.sleep(30)
    threading.Thread(target=warm, daemon=True).start()
    print(f"studio: http://{a.host}:{a.port}" + (" (public: painters only, scrubbed)" if PUBLIC else ""))
    # a thumbnail strip asks for dozens of images at once: the default queue of 5 connections
    # reset the rest ("Connection reset by peer")
    class Server(http.server.ThreadingHTTPServer):
        request_queue_size = 128
        daemon_threads = True
    Server((a.host, a.port), H).serve_forever()


if __name__ == "__main__":
    main()
