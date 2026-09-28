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


_subjects = {}  # studio -> (BRIEF.md mtime, "Friedrich" or "free")


def subject(folder):
    """"Friedrich" if the studio's brief asks for one, else "free" ("" without a brief)."""
    f = os.path.join(os.path.expanduser("~/src/a"), folder, "BRIEF.md")
    try:
        mt = os.path.getmtime(f)
    except OSError:
        return ""
    if folder not in _subjects or _subjects[folder][0] != mt:
        with open(f, errors="replace") as fh:
            _subjects[folder] = (mt, "Friedrich" if "Friedrich" in fh.read(4000) else "free")
    return _subjects[folder][1]


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
                        "thinking": session_thinking(fs[-1]), "subject": subject(name),
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
    c = _cache.setdefault(path, {"offset": 0, "events": [], "images": [], "calls": {}})
    size = os.path.getsize(path)
    if size < c["offset"]:  # rewritten
        c.update(offset=0, events=[], images=[], calls={})
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
            c["events"].append({"ts": ts, "kind": "start", "cwd": d.get("cwd", "")})
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
                    elif name == "paint":  # the painter harness's easel tools (round 19 on)
                        ev.update(kind="paint", code=a.get("lua", ""))
                    elif name == "look":
                        ev.update(kind="look", text=", ".join(f"{k} {v}" for k, v in a.items()))
                    elif name == "note":
                        ev.update(kind="jnote", text=a.get("text", ""))
                    else:
                        ev.update(text=json.dumps(a)[:300])
                    c["calls"][x.get("id")] = len(c["events"])
                    c["events"].append(ev)
        elif role == "toolResult" and isinstance(content, list):
            parent = c["calls"].get(m.get("toolCallId"))
            txt = _text(content)
            if parent is not None and txt:
                c["events"][parent]["out"] = txt[-1500:]
            for x in content:
                if x.get("type") == "image" and x.get("data"):
                    idx = len(c["images"])
                    c["images"].append((x.get("mimeType", "image/jpeg"), x["data"]))
                    src = c["events"][parent].get("path", "") if parent is not None else ""
                    c["events"].append({"ts": ts, "kind": "image", "img": idx, "path": src})
        elif role == "user":
            t = _text(content)
            if t.strip():
                c["events"].append({"ts": ts, "kind": "user", "text": t[:2000]})
    return c["events"], c["images"]


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
    rebuilt and its epoch bumped, so clients know to start over."""
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
                st = {"epoch": st["epoch"] + 1, "parts": [], "events": []}
        else:
            st = {"epoch": 0, "parts": [], "events": []}
        _streams[key] = st
        old, ioff = st["parts"], 0
        for k, (f, n, ni) in enumerate(parts):
            ev = parsed[k][0]
            seen = old[k][1] if k < len(old) else 0
            if seen == 0 and n and k > 0:
                st["events"].append({"ts": ev[0]["ts"], "kind": "sitting", "n": k + 1,
                                     "text": f"sitting {k + 1} · {_hhmm(ev[0]['ts'])}"})
            for e in ev[seen:n]:
                st["events"].append(dict(e, img=e["img"] + ioff) if e["kind"] == "image" else e)
            ioff += ni
        st["parts"] = parts
        return st


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
        self.send_header("Cache-Control", "no-store")
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
        if u.path == "/api/sessions":
            ss = list_sessions()
            if PUBLIC:
                ss = [{k: v for k, v in s.items() if k != "files"} for s in ss if s.get("painter")]
            return self._send(200, json.dumps(ss).encode(), "application/json")
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
            return self._send(200, json.dumps({"events": ev[since:], "total": len(ev), "epoch": st["epoch"],
                                               "sittings": len(files)}).encode(), "application/json")
        if u.path == "/api/file":  # the painting's current source, from the painter's folder
            cwd = next((e["cwd"] for e in reversed(st["events"]) if e["kind"] == "start"), "")
            want = os.path.realpath(os.path.join(cwd, q.get("f", [""])[0]))
            if cwd and want.startswith(os.path.realpath(cwd) + os.sep) and os.path.isfile(want):
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
    print(f"studio: http://{a.host}:{a.port}" + (" (public: painters only, scrubbed)" if PUBLIC else ""))
    http.server.ThreadingHTTPServer((a.host, a.port), H).serve_forever()


if __name__ == "__main__":
    main()
