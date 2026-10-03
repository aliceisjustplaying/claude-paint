# /// script
# requires-python = ">=3.10"
# dependencies = []
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
import http.server
import json
import os
import re
import threading
import urllib.parse
from datetime import datetime

SESSIONS = os.path.expanduser("~/.pi/agent/sessions")
# --public: painters only, nothing that names this machine's owner (for a link shown to others)
PUBLIC = False
HOME = os.path.expanduser("~")
USER = os.path.basename(HOME)


def scrub(body):
    """The home folder as ~ and the account name as `user`, in a response's text."""
    return re.sub(re.escape(USER.encode()), b"user", body.replace(HOME.encode(), b"~"), flags=re.I)
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
# earlier rounds' painter worktrees. Everything else (judges, tests, work on the project)
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
    """paint-studio-... -> (round, lane, painter number, painters in the lane), from the runners'
    studio lists (reread every 30 s)."""
    import time
    if time.time() - _lanes["at"] > 30:
        m = {}
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
                m[studio] = (rnd, lane, int(n) if n else 0, size)
        _lanes.update(at=time.time(), map=m)
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


PAINTER = re.compile(r"^(paint-studio-[0-9a-f]+|paint-r\d+-p\d+|claude-paint-r\d+-(arm\d|tree\d|astra|fable|flash|p\d))$")


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
        if PAINTER.match(name):
            models = []
            for i in info:
                if i["model"] and i["model"] not in models:
                    models.append(i["model"])
            rnd, lane, n, size = lanes().get(name, ("", "", 0, 0))
            out.append({"p": name, "folder": name, "painter": True, "model": " → ".join(models), "sittings": len(fs),
                        "thinking": session_thinking(fs[-1]), **about_brief(name),
                        "round": rnd, "lane": lane, "n": n, "chain": size if size > 1 else 0,
                        "files": fs, "mtime": max(i["mtime"] for i in info), "size": sum(i["size"] for i in info)})
        else:
            out += [dict(i, s=i["path"], folder=name, painter=False) for i in info]
    out.sort(key=lambda s: -s["mtime"])
    return out


def painter_files(name):
    """The session files of a painter folder (given by its short name), in start order."""
    if not name or "/" in name or name.startswith("."):
        return []
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


def _parse(path):
    # changed: indices of events whose result came in after them, in arrival order (see stream())
    c = _cache.setdefault(path, {"offset": 0, "events": [], "images": [], "calls": {}, "changed": [], "cwd": ""})
    size = os.path.getsize(path)
    if size < c["offset"]:  # rewritten
        c.update(offset=0, events=[], images=[], calls={}, changed=[], cwd="")
    with open(path, "rb") as fh:
        fh.seek(c["offset"])
        chunk = fh.read()
    last_nl = chunk.rfind(b"\n")
    if last_nl < 0:
        return c["events"], c["images"]
    c["offset"] += last_nl + 1
    for line in chunk[: last_nl + 1].splitlines():
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
                    elif name == "look":
                        ev.update(kind="look", text=look_text(a))
                    elif name == "note":
                        ev.update(kind="jnote", text=a.get("text", ""))
                    else:
                        ev.update(text=json.dumps(a)[:300])
                    c["calls"][x.get("id")] = len(c["events"])
                    c["events"].append(ev)
        elif role == "toolResult" and isinstance(content, list):
            parent = c["calls"].get(m.get("toolCallId"))
            txt = _text(content)
            if parent is not None and (txt or m.get("isError")):
                if txt:
                    c["events"][parent]["out"] = txt[-1500:]
                if m.get("isError"):
                    c["events"][parent]["err"] = True
                c["changed"].append(parent)
            for x in content:
                if x.get("type") == "image" and x.get("data"):
                    idx = len(c["images"])
                    c["images"].append((x.get("mimeType", "image/jpeg"), x["data"]))
                    src = c["events"][parent].get("path", "") if parent is not None else ""
                    ev = {"ts": ts, "kind": "image", "img": idx, "path": src}
                    if parent is not None and c["events"][parent]["kind"] == "look":
                        ev["look"] = c["events"][parent]["text"]  # what the painter asked to see (see is_whole)
                    if parent is not None and c["events"][parent].get("ref"):
                        ev["ref"] = True  # a reference picture: never the painting
                    c["events"].append(ev)
        elif role == "user":
            t = _text(content)
            if t.strip():
                c["events"].append({"ts": ts, "kind": "user", "text": t[:2000]})
    return c["events"], c["images"]


def look_text(args):
    """A look call's request as one line: "crop 110,540,610,900, size 800", "mode squint" ("" for the plain look)."""
    return ", ".join(f"{k} {v}" for k, v in args.items())


def is_whole(look):
    """A look request that shows the whole canvas as it is: no crop, no mode (value, squint, mirror) and not the palette."""
    return look is not None and not re.search(r"crop|mode|palette (?!False)", look)


# a closing reply that begins with the painting's title: "**The Silent Shore**", "### *Hünengrab im Abendlicht* (...)",
# also after an opening kaomoji ("(ᵔᴥᵔ) **The Old Willow at Evening**")
TITLE = re.compile(r"\s*(?:\([^)\n]{1,16}\)\S{0,3}\s+)?(?:#{1,6}\s*)?(\*\*?|__?)([^*_\n]{2,100}?)\1(?![*_\w])")
# or one on a line of its own a little further down ("I'm stopping here. ...\n\n**Quinces, Raking Light**\n\n...")
TITLE_LINE = re.compile(r"[ \t]*(?:#{1,6}[ \t]*)?(\*\*?|__?)([^*_\n]{2,100}?)\1[ \t]*")


def title_of(say):
    """The painting's title from the painter's last words, if they begin with one or have one alone on a line among
    their first three paragraphs; else None."""
    m = TITLE.match(say or "")
    if m:
        return m.group(2).strip()
    for para in [p for p in re.split(r"\n\s*\n", say or "") if p.strip()][1:3]:
        m = TITLE_LINE.fullmatch(para.strip("\n"))
        if m:
            return m.group(2).strip()
    return None


REFERENCE = object()  # in a glance's calls: a read of a reference picture
_warm = None  # the server's first scan of every painter for the picker: set when done (None: scan on request)
_glances = {}  # path -> what the picker needs from one session file, read incrementally without keeping images


def _glance_file(path):
    """One session file, scanned from where the last scan stopped: its image count, its newest whole look and its
    last picture (each as (index in the file, byte offset of the line, which image in the line)) and its last words.
    Images are counted as parse() counts them, so the indices match the stream's. Reference pictures (read from
    the studio's reference/) are counted but are never the whole look or the last picture."""
    with _lock("g:" + path):
        g = _glances.get(path)
        size = os.path.getsize(path)
        if not g or size < g["offset"]:
            g = _glances[path] = {"offset": 0, "calls": {}, "n": 0, "whole": None, "last": None, "say": "", "cwd": ""}
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
                if not isinstance(content, list):
                    continue
                if m.get("role") == "assistant":
                    for x in content:
                        if x.get("type") == "text" and x.get("text", "").strip():
                            g["say"] = x["text"]
                        elif x.get("type") == "toolCall":  # a look's request; REFERENCE for a read from reference/
                            a = x.get("arguments") or {}
                            g["calls"][x.get("id")] = (look_text(a) if x.get("name") == "look" else REFERENCE
                                                       if x.get("name") == "read" and in_reference(a.get("path", ""), g["cwd"])
                                                       else None)
                elif m.get("role") == "toolResult":
                    look, k = g["calls"].get(m.get("toolCallId")), 0
                    for x in content:
                        if x.get("type") == "image" and x.get("data"):
                            if look is not REFERENCE:
                                g["last"] = (g["n"], at, k)
                                if is_whole(look):
                                    g["whole"] = g["last"]
                            g["n"] += 1
                            k += 1
        return g


def glance(files):
    """A painter's picture and title for the picker: {"look": its newest whole look (or, before the look tool, the
    last picture it saw; never a reference picture; None if it has seen none of its own) as a stream image index,
    "title": from its last words or None}, and where that look's bytes are, for /api/glance."""
    gs, base, look, src, last, lsrc = [_glance_file(f) for f in files], 0, None, None, None, None
    for f, g in zip(files, gs):
        if g["whole"]:
            look, src = base + g["whole"][0], (f,) + g["whole"][1:]
        if g["last"]:
            last, lsrc = base + g["last"][0], (f,) + g["last"][1:]
        base += g["n"]
    say = next((g["say"] for g in reversed(gs) if g["say"]), "")
    if look is None:
        look, src = last, lsrc
    return {"look": look, "title": title_of(say), "src": src}


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
    archive/sources/<its name>. The files: paintings/lua/painting.lua and, from the events, every .lua
    under paintings/lua/ and .rs under paintings/ the painter wrote or edited or rendered with
    `cargo paint <bin>`. Nothing else in the folder (the brief, notes, bin/, settings) is served."""
    cwd = next((e["cwd"] for e in reversed(events) if e["kind"] == "start"), "")
    rels = {"paintings/lua/painting.lua"}
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


class H(http.server.BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def _send(self, code, body, ctype):
        if PUBLIC and not ctype.startswith("image/"):
            body = scrub(body)
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        # a look never changes: the browser keeps it (and asks again for everything else)
        self.send_header("Cache-Control", "private, max-age=86400, immutable" if ctype.startswith("image/") else "no-store")
        if PUBLIC:
            self.send_header("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-inline'; "
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
        if u.path == "/stream.css":  # the livestream's layout: the page loads it itself with ?stream=1
            with open(os.path.join(HERE, "stream.css"), "rb") as fh:
                return self._send(200, fh.read(), "text/css; charset=utf-8")
        if u.path == "/api/sessions":
            ss = list_sessions()
            for s in ss:
                if s.get("painter") and (_warm is None or _warm.is_set()):  # (until the first scan is done, without)  # the picker's picture and title (the static export has them in sessions.json)
                    g = glance(s["files"])
                    s.update(look=g["look"], title=g["title"])
            if PUBLIC:
                ss = [{k: v for k, v in s.items() if k != "files"} for s in ss if s.get("painter")]
            return self._send(200, json.dumps(ss).encode(), "application/json")
        if u.path == "/api/glance":  # a painter's picture in the picker, read alone (not the painter's whole log)
            files = painter_files(painter)
            g = glance(files) if files else None
            im = glance_image(g["src"]) if g and g["src"] else None
            return self._send(200, im[1], im[0]) if im else self._send(404, b"not found", "text/plain")
        if u.path not in ("/api/events", "/api/file", "/img"):
            return self._send(404, b"not found", "text/plain")
        files = painter_files(painter) if painter else [path] if path and os.path.isfile(path) else []
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
                                               "sittings": len(files), "updates": ups,
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
            im = image(st, int(q.get("i", ["0"])[0]))
            if im:
                return self._send(200, base64.b64decode(im[1]), im[0])
        self._send(404, b"not found", "text/plain")


def main():
    global SESSIONS
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--host", default="127.0.0.1", help="e.g. this machine's Tailscale IP to watch from another of your devices")
    ap.add_argument("--sessions", default=SESSIONS, help="where pi keeps its session logs")
    ap.add_argument("--public", action="store_true", help="painters only, home path and account name scrubbed (a link for others)")
    a = ap.parse_args()
    SESSIONS = os.path.abspath(os.path.expanduser(a.sessions))
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
