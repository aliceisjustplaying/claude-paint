# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""The studio: watch a painter paint, live or replayed.

Reads a painter's pi session log (~/.pi/agent/sessions/<folder>/*.jsonl):
every program write and edit, every command, every image the painter
looked at (the log holds the image bytes it saw) and its words. Serves a
page with the code on the left, the latest look on the right and a
timeline to scrub. Read-only: it never touches the painter.

    uv run studio/studio.py            # then open http://localhost:8765
    uv run studio/studio.py --port 9000
"""
import argparse
import base64
import glob
import http.server
import json
import os
import urllib.parse

SESSIONS = os.path.expanduser("~/.pi/agent/sessions")
HERE = os.path.dirname(os.path.abspath(__file__))
_cache = {}  # path -> {"size", "offset", "events", "images"}


def list_sessions():
    out = []
    for f in glob.glob(os.path.join(SESSIONS, "*paint*", "*.jsonl")):
        d = os.path.basename(os.path.dirname(f))
        out.append({"path": f, "folder": d.strip("-").split("-src-a-")[-1],
                    "mtime": os.path.getmtime(f), "size": os.path.getsize(f)})
    out.sort(key=lambda s: -s["mtime"])
    return out


def _text(content):
    if isinstance(content, str):
        return content
    return "\n".join(x.get("text", "") for x in content if isinstance(x, dict) and x.get("type") == "text")


def parse(path):
    """Parse the log incrementally; returns (events, images)."""
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


class H(http.server.BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def _send(self, code, body, ctype):
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        u = urllib.parse.urlparse(self.path)
        q = urllib.parse.parse_qs(u.query)
        path = q.get("s", [""])[0]
        if path and not os.path.realpath(path).startswith(os.path.realpath(SESSIONS)):
            return self._send(403, b"no", "text/plain")
        if u.path == "/":
            with open(os.path.join(HERE, "index.html"), "rb") as fh:
                return self._send(200, fh.read(), "text/html; charset=utf-8")
        if u.path == "/api/sessions":
            return self._send(200, json.dumps(list_sessions()).encode(), "application/json")
        if u.path == "/api/events":
            since = int(q.get("since", ["0"])[0])
            ev, _ = parse(path)
            return self._send(200, json.dumps({"events": ev[since:], "total": len(ev)}).encode(), "application/json")
        if u.path == "/img":
            _, imgs = parse(path)
            i = int(q.get("i", ["0"])[0])
            if 0 <= i < len(imgs):
                mime, data = imgs[i]
                return self._send(200, base64.b64decode(data), mime)
        self._send(404, b"not found", "text/plain")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8765)
    a = ap.parse_args()
    print(f"studio: http://localhost:{a.port}")
    http.server.ThreadingHTTPServer(("127.0.0.1", a.port), H).serve_forever()


if __name__ == "__main__":
    main()
