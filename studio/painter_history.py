"""Painter histories shared by the live studio and its static serializer.

stream(key, files) supplies stitched events with resolved image roles; image(st, i)
reads their bytes. glance(files) supplies the picker thumbnail and title without
retaining or decoding image bytes, using persisted pre-look whole classifications.
list_sessions() and runner_states() own the runner and picker lifecycle facts.
Roles are painting-whole, scratch-whole, palette, close-up and reference;
scratch palette/detail looks use scratch-palette and scratch-close-up.
"""
import base64
import glob
import io
import json
import os
import re
import shutil
import subprocess
import threading
import time
from datetime import datetime
from palette_board import palette_board

SESSIONS = os.path.expanduser("~/.pi/agent/sessions")
# --public: painters only, nothing that names this machine's owner (for a link shown to others)
HOME = os.path.expanduser("~")
USER = os.path.basename(HOME)


def scrub(body):
    """The home folder as ~ and the account name as `user`, in a response's text."""
    return re.sub(re.escape(USER.encode()), b"user", body.replace(HOME.encode(), b"~"), flags=re.I)


def scrubbed(body):
    """Whether serialized public text no longer names the local home or account."""
    return HOME.encode() not in body and not re.search(re.escape(USER.encode()), body, re.I)


HERE = os.path.dirname(os.path.abspath(__file__))
_cache = {}  # path -> {"offset", "events", "images", "calls"}: one session file, parsed so far
_streams = {}  # key -> a stitched stream of session files (see stream())
_locks = {}
_locks_lock = threading.Lock()


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
RUNS = os.path.expanduser("~/tmp/gallery-*/r*/run/studios.json")
_lanes = {"at": 0.0, "map": {}}


def lanes():
    """paint-studio-... -> (round, lane, painter number, painters in the lane, outcome), from the runners'
    studio lists (reread every 30 s)."""
    import time
    if time.time() - _lanes["at"] > 30:
        m, states = {}, {}
        for f in glob.glob(RUNS):
            rnd = os.path.basename(os.path.dirname(os.path.dirname(f)))  # r17, r18g, r19, ...
            try:
                names = json.load(open(f))
            except (OSError, ValueError):
                continue
            keys = {k: re.match(r"^(.*?)(\d*)$", k).groups() for k in names}
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
                stem = os.path.join(rd, f"p{number}")
                running = False
                try:
                    with open(stem + "_sittings.json") as fh:
                        sittings = json.load(fh)
                    running = bool(sittings) and sittings[-1].get("status") == "running"
                except (OSError, ValueError, AttributeError, TypeError, KeyError):
                    pass
                completed = next((os.path.getmtime(p) for p in (stem + ".painted", stem + "_outcome.json")
                                  if os.path.isfile(p)), 0)
                states[studio] = (running, completed)
                m[studio] = (rnd, lane, number, size, outcome)
        _lanes.update(at=time.time(), map=m, states=states)
    return _lanes["map"]


_subjects = {}  # studio -> (BRIEF.md mtime, what its brief says: see about_brief)


def _artist_of(brief):
    """The artist a brief names, in full ("in the manner of Kendric Tonn" -> "Kendric Tonn"), or ""."""
    m = re.search(r"in\s+the\s+manner\s+of\s+((?:[A-Z][\w.'-]*\s+)*[A-Z][\w.'-]*)", brief)
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
            models = []
            for i in info:
                if i["model"] and i["model"] not in models:
                    models.append(i["model"])
            rnd, lane, n, size, outcome = lanes().get(name, ("", "", 0, 0, None))
            out.append({"p": name, "folder": name, "painter": True, "model": " → ".join(models), "sittings": len(fs),
                        "thinking": session_thinking(fs[-1]), **about_brief(name),
                        "round": rnd, "lane": lane, "n": n, "chain": size if size > 1 else 0,
                        "outcome": outcome,
                        "files": fs, "mtime": max(i["mtime"] for i in info), "size": sum(i["size"] for i in info)})
        else:
            out += [dict(i, s=i["path"], folder=name, painter=False) for i in info]
    out.sort(key=lambda s: -s["mtime"])
    return out


def painter_files(name):
    """The session files of a painter folder (given by its short name), in start order."""
    if not name or "/" in name or name.startswith("."):
        return []
    for folder, ps in SPLIT.items():
        if name in ps:
            return sorted(glob.glob(os.path.join(SESSIONS, "*-src-a-" + folder + "--", ps[name] + "*.jsonl")))
    for d in glob.glob(os.path.join(SESSIONS, "*" + name + "*")):
        if short(os.path.basename(d)) == name:
            return sorted(glob.glob(os.path.join(d, "*.jsonl")), key=os.path.basename)
    return []


def _text(content):
    if isinstance(content, str):
        return content
    return "\n".join(x.get("text", "") for x in content if isinstance(x, dict) and x.get("type") == "text")


def parse(path):
    """Parse the log incrementally; returns (events, images)."""
    with _lock("f:" + path):
        return _parse(path)


def _entries(path, c):
    """Complete JSONL entries only; never consume an unfinished trailing line."""
    with open(path, "rb") as fh:
        fh.seek(c["offset"])
        while True:
            at, line = fh.tell(), fh.readline()
            if not line.endswith(b"\n"):
                break
            c["offset"] = fh.tell()
            try:
                yield at, json.loads(line)
            except ValueError:
                continue


def _entry_events(d, c, at, images=None):
    """Normalize calls, results and image roles once for both projections.

    The picker omits the image store. Its call map keeps only correlation facts,
    not programs, output text or image data. Full histories also retain results
    on the original call event and decode palette metadata.
    """
    if images is not None:
        from studio import summary_clock
    ts = d.get("timestamp", "")
    typ = d.get("type")
    if typ == "session":
        c["cwd"] = d.get("cwd", "")
        yield {"ts": ts, "kind": "start", "cwd": c["cwd"]}
    elif typ == "model_change":
        yield {"ts": ts, "kind": "note", "text": "model " + d.get("modelId", "")}
    elif typ == "compaction" and images is not None:
        clock = summary_clock(d.get("summary", ""))
        if clock:
            yield {"ts": ts, "kind": "clock", "clock": clock, "text": clock}
    elif typ == "message":
        m = (d.get("message") or {}) if images is None else d.get("message", {})
        role, content = m.get("role"), m.get("content")
        if images is None and not isinstance(content, list):
            return
        if role == "assistant" and isinstance(content, list):
            for x in content:
                t = x.get("type")
                if t == "text" and x.get("text", "").strip():
                    yield {"ts": ts, "kind": "say", "text": x["text"]}
                elif t == "thinking" and x.get("thinking", "").strip():
                    yield {"ts": ts, "kind": "think", "text": x["thinking"]}
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
                            ev["ref"] = True
                    elif name == "paint":
                        ev.update(kind="paint", code=a.get("lua", ""))
                        if a.get("scratch"):
                            ev["scratch"] = True
                            if a.get("new_scratch"):
                                ev["new_scratch"] = True
                    elif name == "look":
                        ev.update(kind="look", text=look_text(a), role=look_role(look_text(a)))
                        c["has_looks"] = True
                    elif name == "note":
                        ev.update(kind="jnote", text=a.get("text", ""))
                    else:
                        ev.update(text=json.dumps(a)[:300])
                    parent = ev if images is not None else {k: v for k, v in ev.items()
                                                            if k in ("kind", "path", "ref", "text", "role")}
                    if images is None and name != "look":
                        parent.pop("text", None)
                    c["calls"][x.get("id")] = (len(c.get("events", [])), parent)
                    yield ev
        elif role == "toolResult" and isinstance(content, list):
            parent_index, parent = c["calls"].get(m.get("toolCallId"), (None, None))
            if images is not None:
                txt = _text(content)
                if parent is not None and (txt or m.get("isError")):
                    if txt:
                        parent["out"] = txt[-1500:]
                    if m.get("isError"):
                        parent["err"] = True
                    c["changed"].append(parent_index)
            k = 0
            for x in content:
                if x.get("type") == "image" and x.get("data"):
                    idx = c["n"]
                    c["n"] += 1
                    if images is not None:
                        images.append((x.get("mimeType", "image/jpeg"), x["data"]))
                    ev = {"ts": ts, "kind": "image", "img": idx, "path": parent.get("path", "") if parent else ""}
                    if parent and parent["kind"] == "look":
                        ev.update(look=parent["text"], role=parent["role"])
                    else:
                        ev["role"] = "close-up"  # resolved after pre-look inference, if applicable
                    if parent and parent.get("ref"):
                        ev.update(ref=True, role="reference")
                    if images is not None and ev["role"] in ("palette", "scratch-palette"):
                        chips = palette_board(base64.b64decode(x["data"]))
                        if chips:
                            ev["palette"] = chips
                    # The picker receives locations separately; these are not serialized event fields.
                    c["src"] = (idx, at, k)
                    k += 1
                    yield ev
        elif role == "user" and images is not None:
            t = _text(content)
            if t.strip():
                ev = {"ts": ts, "kind": "user", "text": t[:2000]}
                clock = summary_clock(t)
                if clock:
                    ev["clock"] = clock
                yield ev


def _parse(path):
    c = _cache.setdefault(path, {"offset": 0, "events": [], "images": [], "calls": {}, "changed": [],
                                "cwd": "", "n": 0, "has_looks": False})
    if os.path.getsize(path) < c["offset"]:
        c.update(offset=0, events=[], images=[], calls={}, changed=[], cwd="", n=0, has_looks=False)
    for at, d in _entries(path, c):
        for ev in _entry_events(d, c, at, c["images"]):
            c["events"].append(ev)
    if not c["has_looks"]:
        mark_whole(c, path)
    for ev in c["events"]:
        if ev["kind"] == "image" and "look" not in ev and not ev.get("ref"):
            ev["role"] = _file_read_role(ev.get("whole"))
    return c["events"], c["images"]


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


def _whole_cache(refresh=False):
    # A light live picker can stay up while a separate exporter fills this cache.
    # Reload only when that persisted file changes, never by decoding pictures.
    try:
        stamp = (WHOLE_CACHE, os.stat(WHOLE_CACHE).st_mtime_ns)
    except OSError:
        stamp = (WHOLE_CACHE, None)
    if "d" not in _whole or refresh and _whole.get("stamp") != stamp:
        try:
            with open(WHOLE_CACHE) as fh:
                _whole["d"] = json.load(fh)
        except (OSError, ValueError):
            _whole["d"] = {}
        _whole["stamp"] = stamp
    return _whole["d"]


def _save_whole_cache():
    os.makedirs(os.path.dirname(WHOLE_CACHE), exist_ok=True)
    tmp = f"{WHOLE_CACHE}.{os.getpid()}.tmp"
    with open(tmp, "w") as fh:
        json.dump(_whole["d"], fh)
    os.replace(tmp, WHOLE_CACHE)
    _whole["stamp"] = (WHOLE_CACHE, os.stat(WHOLE_CACHE).st_mtime_ns)


def _whole_key(path, count):
    return f"v4:{path}:{os.path.getsize(path)}:{count}"


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
    key = _whole_key(path, len(ims))
    done = _whole_cache().get(key)
    if done is not None and len(done) == len(ims):
        for e, w in zip(ims, done):
            e["whole"] = w
        return
    sig = c.setdefault("sig", {})
    for e in ims:
        if e["img"] not in sig:
            sig[e["img"]] = _signature(c["images"][e["img"]][1])
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


def look_text(args):
    """A look call's request as one line: "crop 110,540,610,900, size 800", "mode squint" ("" for the plain look)."""
    return ", ".join(f"{k} {v}" for k, v in args.items())


def is_whole(look):
    """A look request that shows the whole canvas as it is: no crop, no mode (value, squint, mirror), no light,
    not the palette, not a survey's tile, not a comparison, not a held knife and not the scratch canvas."""
    return look is not None and not re.search(r"crop|mode|light|compare|hold|(?:palette|survey|scratch) (?!False)", look)


def look_role(look):
    """Canvas-aware role of an explicit look, preserving historical request semantics."""
    scratch = bool(re.search(r"scratch (?!False)", look))
    prefix = "scratch-" if scratch else ""
    if re.search(r"palette (?!False)", look):
        return prefix + "palette"
    plain = re.sub(r"(?:, )?scratch True", "", look).removeprefix(", ") if scratch else look
    if is_whole(plain):
        return "scratch-whole" if scratch else "painting-whole"
    return prefix + "close-up"


# a closing reply that begins with the painting's title: "**The Silent Shore**", "### *Hünengrab im Abendlicht* (...)",
# also after an opening kaomoji ("(ᵔᴥᵔ) **The Old Willow at Evening**")
TITLE = re.compile(r"\s*(?:\([^)\n]{1,16}\)\S{0,3}\s+)?(?:#{1,6}\s*)?(\*\*?|__?)([^*_\n]{2,100}?)\1(?![*_\w])")
# or one on a line of its own a little further down ("I'm stopping here. ...\n\n**Quinces, Raking Light**\n\n...")
TITLE_LINE = re.compile(r"[ \t]*(?:#{1,6}[ \t]*)?(\*\*?|__?)([^*_\n]{2,100}?)\1[ \t]*")


# or named in the first line of its closing words: "I've called it **Grainstack, Evening**.", "I've finished
# **Nymphéas, reflet du soir** (...)", "I'm leaving **Luncheonette, Four O'Clock** finished as it is."
NAMED = re.compile(r"[^\n*_]{0,40}?\b(?:call(?:ed|ing)?|title[ds]?|name[ds]?|finish(?:ed|ing)?|left|leav(?:e|ing)"
                   r"|complet(?:ed|e|ing))(?:\s+(?:it|this|(?:the|this|my)\s+(?:painting|picture|canvas)))?\s+(\*\*?|__?)([^*_\n]{2,100}?)\1(?![*_\w])")


def clean_title(t):
    """A title without the quotation marks some painters put inside the bold."""
    return t.strip().strip('"\u201c\u201d').strip() or None


def title_of(say):
    """The painting's title from the painter's last words, if they begin with one or have one alone on a line among
    their first three paragraphs; else None."""
    m = TITLE.match(say or "") or NAMED.match(say or "")
    if m:
        return clean_title(m.group(2))
    for para in [p for p in re.split(r"\n\s*\n", say or "") if p.strip()][1:3]:
        m = TITLE_LINE.fullmatch(para.strip("\n"))
        if m:
            return clean_title(m.group(2))
    return None


def title_of_closings(says):
    """The title in the newest closing words that name one (a last sitting that only looked may not repeat it)."""
    return next((t for t in map(title_of, reversed(says)) if t), None)


_glances = {}  # lightweight per-file projections: no image bytes or programs


def _glance_file(path):
    with _lock("g:" + path):
        g = _glances.get(path)
        if not g or os.path.getsize(path) < g["offset"]:
            g = _glances[path] = {"offset": 0, "calls": {}, "n": 0, "whole": None, "last": None,
                                  "say": "", "cwd": "", "has_looks": False, "legacy": []}
        for at, d in _entries(path, g):
            for ev in _entry_events(d, g, at):
                if ev["kind"] == "say":
                    g["say"] = ev["text"]
                elif ev["kind"] == "image":
                    if "look" not in ev and not ev.get("ref"):
                        g["last"] = g["src"]
                    if not g["has_looks"] and not ev.get("ref"):
                        g["legacy"].append(g["src"])
                    if ev["role"] == "painting-whole":
                        g["whole"] = g["src"]
        return g


def _file_read_role(whole=None):
    """A file read is a whole-picture fallback unless the heuristic excludes it.

    Modern sittings have no persisted pre-look classification; their last file
    read must still outrank an earlier sitting's whole when there is no explicit
    whole look, as in the historical live/static picker rule.
    """
    return "close-up" if whole is False else "painting-whole"


def _picker_image(images, eligible=None):
    """Newest explicit whole, else newest legacy whole, else last legacy image.

    A serializer may restrict candidates to pictures with usable web copies.
    Image records carry only role, look presence and img; no pixels are needed.
    """
    mine = [e for e in images if e["role"] != "reference" and
            (eligible is None or 0 <= e["img"] < len(eligible) and eligible[e["img"]])]
    return (next((e for e in reversed(mine) if "look" in e and e["role"] == "painting-whole"), None)
            or next((e for e in reversed(mine) if "look" not in e and e["role"] == "painting-whole"), None)
            or next((e for e in reversed(mine) if "look" not in e), None))


def glance(files):
    """Picker title and thumbnail location without loading/decoding a history's pictures.

    For pre-look sessions, consume mark_whole's persisted answers. A cache miss
    keeps the last-picture fallback until a full history/export computes them.
    """
    images, says, base = [], [], 0
    for f in files:
        g = _glance_file(f)
        if g["say"]:
            says.append(g["say"])
        if g["whole"]:
            i, at, k = g["whole"]
            images.append({"img": base + i, "role": "painting-whole", "look": True, "src": (f, at, k)})
        if not g["has_looks"]:
            done = _whole_cache(refresh=True).get(_whole_key(f, len(g["legacy"])))
            for j, (i, at, k) in enumerate(g["legacy"]):
                whole = done[j] if done is not None and len(done) == len(g["legacy"]) else True
                images.append({"img": base + i, "role": _file_read_role(whole), "src": (f, at, k)})
        elif g["last"]:
            i, at, k = g["last"]
            images.append({"img": base + i, "role": _file_read_role(), "src": (f, at, k)})
        base += g["n"]
    chosen = _picker_image(images)
    return {"look": chosen["img"] if chosen else None, "title": title_of_closings(says),
            "src": chosen["src"] if chosen else None}


def picker(st, eligible=None):
    """Full history's picker facts, with the same thumbnail selection as glance."""
    ev = st["events"]
    closings, say = [], None
    for e in ev:
        if e["kind"] == "start":
            if say:
                closings.append(say)
            say = None
        elif e["kind"] == "say":
            say = e["text"]
    if say:
        closings.append(say)
    chosen = _picker_image([e for e in ev if e["kind"] == "image"], eligible)
    return {"look": chosen["img"] if chosen else None, "title": title_of_closings(closings),
            "said": bool(ev) and ev[-1]["kind"] == "say"}


def glance_image(src):
    """The bytes of the image at src (a file, a line's byte offset, which image in the line): (mime, data) or None."""
    path, at, k = src
    with open(path, "rb") as fh:
        fh.seek(at)
        d = json.loads(fh.readline())
    imgs = [x for x in d["message"]["content"] if x.get("type") == "image" and x.get("data")]
    return (imgs[k].get("mimeType", "image/jpeg"), base64.b64decode(imgs[k]["data"])) if k < len(imgs) else None


def _hhmm(ts):
    try:
        return datetime.fromisoformat(ts.replace("Z", "+00:00")).astimezone().strftime("%H:%M")
    except ValueError:
        return ""


def stream(key, files):
    """The events of several session files (a painter's sittings) as one stream, in start
    order, with a {"kind": "sitting"} event before each sitting after the first. Image
    indices are renumbered to be unique across files. Built incrementally: each file is
    parsed from where it stopped, and only the new events are appended. If an earlier
    part changes (it shouldn't: a sitting ends before the next begins) the stream is
    rebuilt and its epoch bumped, so clients know to start over. An event already in the
    stream whose result comes in later is listed in "updates" (its stream index, in
    arrival order), so clients that have it can fetch it again."""
    with _lock("s:" + key):
        st = _streams.get(key)
        parsed = [parse(f) for f in files]
        parts = [(f, len(ev), len(im)) for f, (ev, im) in zip(files, parsed)]
        if st:
            old = st["parts"]
            same = (len(old) <= len(parts) and all(o[0] == n[0] for o, n in zip(old, parts))
                    and all(o == n for o, n in zip(old[:-1], parts))
                    and (not old or parts[len(old) - 1][1] >= old[-1][1] and parts[len(old) - 1][2] >= old[-1][2]))
            if not same:
                st = {"epoch": st["epoch"] + 1, "parts": [], "events": [], "base": [], "chg": [], "updates": []}
        else:
            st = {"epoch": 0, "parts": [], "events": [], "base": [], "chg": [], "updates": []}
        _streams[key] = st
        old, ioff = st["parts"], 0
        for k, (f, n, ni) in enumerate(parts):
            ev = parsed[k][0]
            seen = old[k][1] if k < len(old) else 0
            if seen == 0 and n and k > 0:
                st["events"].append({"ts": ev[0]["ts"], "kind": "sitting", "n": k + 1,
                                     "text": f"sitting {k + 1} · {_hhmm(ev[0]['ts'])}"})
            if k == len(st["base"]):
                st["base"].append(len(st["events"]))  # where this file's events begin in the stream
            if k == len(st["chg"]):
                st["chg"].append(0)
            changed = _cache[f]["changed"]
            for j in changed[st["chg"][k]:]:
                if j < seen:  # already sent without its result
                    st["updates"].append(st["base"][k] + j)
            st["chg"][k] = len(changed)
            for e in ev[seen:n]:
                st["events"].append(dict(e, img=e["img"] + ioff) if e["kind"] == "image" else e)
            ioff += ni
        st["parts"] = parts
        return st


# the painting sources of painters whose folder is gone (a round's worktree, removed), by folder name
ARCHIVE = os.path.join(os.path.dirname(HERE), "archive", "sources")


def painting_sources(events):
    """The folder to read the painting's source files from and those files, relative to it: the ones
    the page may ask for. The folder is the painter's (its last start) or, once that is gone,
    archive/sources/<its name>. The files: paintings/lua/painting.lua, paintings/lua/scratch.lua once the
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


def image(st, i):
    """Image i of a stitched stream: (mime, base64 data), or None."""
    for f, _, ni in st["parts"]:
        if i < ni:
            imgs = _cache[f]["images"]
            return imgs[i] if 0 <= i < len(imgs) else None
        i -= ni
    return None


def working_studios():
    """Actual Pi process working directories, not persistent easel servers or saved running records."""
    try:
        ps = subprocess.run(["ps", "-Ao", "pid=,comm="], capture_output=True, text=True, check=True, timeout=10)
        pids = [parts[0] for row in ps.stdout.splitlines()
                if len(parts := row.split()) == 2 and os.path.basename(parts[1]) == "pi"]
        lsof = shutil.which("lsof") or "/usr/sbin/lsof"
        if not pids:
            return set()
        r = subprocess.run([lsof, "-a", "-p", ",".join(pids), "-d", "cwd", "-Fn"],
                           capture_output=True, text=True, timeout=10)
        return {os.path.basename(row[1:]) for row in r.stdout.splitlines() if row.startswith("n")}
    except (OSError, subprocess.SubprocessError):
        return set()  # no execution evidence: never claim live


def runner_states(sessions, working, now):
    """Require a running sitting, a live Pi in that studio and recent session activity. Completion
    comes only from the runner outcome, never a title, quiet session or exported render."""
    lanes()
    records = _lanes.get("states", {})
    for s in sessions:
        running, completed = records.get(s["p"], (False, 0))
        s["active"] = bool(not s.get("outcome") and running and s["p"] in working and 0 <= now - s["mtime"] < 1800)
        s["completed_at"] = (completed or s["mtime"]) if (s.get("outcome") or {}).get("status") == "finished" else None


def palette_chips(data, names):
    """Match look.rs chart glyphs before browser color processing or canvas protections."""
    from PIL import Image
    im = Image.open(io.BytesIO(data)).convert("RGB")
    if im.width == 1000:
        return palette_board(data)
    if im.width != 1124:
        return None
    px, rows = im.load(), []
    for y in range(32, im.height):
        white = [x for x in range(8, 532) if px[x, y] == (255, 255, 255)]
        if not white:
            continue
        if rows and y - rows[-1][1] <= 16:
            rows[-1][1] = y
            rows[-1][2] = max(rows[-1][2], white[-1])
        else:
            rows.append([y, y, white[-1]])
    glyphs = dict(zip("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_", (
        "75557 26227 71747 71317 55711 74717 74757 71122 75757 75717 "
        "25755 65656 34443 65556 74647 74644 34553 55755 72227 11152 55655 44447 "
        "57755 65555 25552 65644 25563 65655 34216 72222 55557 55552 55775 55255 55222 71247 00007"
    ).split()))
    signatures = {}
    for name in names:
        if len(name) > 66:
            continue
        points = [(x, y) for y in range(5) for x in range(len(name) * 4)
                  if x % 4 < 3 and int(glyphs[name[x // 4].upper()][y]) & (4 >> (x % 4))]
        top = min(y for _, y in points)
        key = tuple((x, y - top) for x, y in points)
        signatures[key] = name if key not in signatures else None
    chips = []
    for i, (top, bottom, right) in enumerate(rows):
        key = tuple(((x - 10) // 2, (y - top) // 2)
                    for y in range(top, bottom + 1, 2) for x in range(10, right + 1, 2)
                    if px[x, y] == (255, 255, 255))
        name = signatures.get(key)
        if name is None:
            return None
        end = rows[i + 1][0] - 3 if i + 1 < len(rows) else im.height
        y = (top - 3 + end) // 2
        chips.append({"name": name, "thick": "#%02x%02x%02x" % px[608, y],
                      "thin": "#%02x%02x%02x" % px[752, y]})
    return chips or None


def image_count(st):
    """Number of pictures in the stitched history, including references."""
    return sum(n for _, _, n in st["parts"])


def palette_events(st):
    """Include legacy chart chips for static histories, using names known at each look."""
    names = set()
    for e in st["events"]:
        names.update(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", e.get("code", "")))
        if e["kind"] == "image" and e["role"] in ("palette", "scratch-palette"):
            im = image(st, e["img"])
            chips = palette_chips(base64.b64decode(im[1]), names) if im else None
            if chips:
                e["palette"] = chips
