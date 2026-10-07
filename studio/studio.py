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
import http.server
import json
import os
import re
import threading
import urllib.parse
import painter_history as P

PUBLIC = False
_warm = None
HERE = os.path.dirname(os.path.abspath(__file__))
VIEWER_ASSETS = {
    "stream.css": "text/css; charset=utf-8",
    "code-display.js": "text/javascript; charset=utf-8",
    "code-format.js": "text/javascript; charset=utf-8",
    "vendor/stylua/stylua_lib_web.js": "text/javascript; charset=utf-8",
    "vendor/stylua/stylua_lib_bg.wasm": "application/wasm",
    "vendor/stylua/LICENSE.md": "text/plain; charset=utf-8",
}


def summary_clock(text):
    """The latest recorded time in the painter harness's saved clock section."""
    section = re.search(r"(?m)^## The canvas clock\n([\s\S]*?)(?=\n## |\Z)", text)
    if not section:
        return None
    times = re.findall(r"(?m)^- Latest (?:painting time the easel printed|journal entry stamped): "
                       r"(day ([1-9]\d*), ([01]\d|2[0-3]):([0-5]\d))[ \t]*$", section[1])
    return max(times, key=lambda t: tuple(map(int, t[1:])))[0] if times else None


class H(http.server.BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def _send(self, code, body, ctype):
        if PUBLIC and not ctype.startswith("image/") and ctype != "application/wasm":
            body = P.scrub(body)
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        # a look never changes: the browser keeps it (and asks again for everything else)
        self.send_header("Cache-Control", "private, max-age=86400, immutable" if ctype.startswith("image/") else "no-store")
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
        if PUBLIC and (path or (painter and not P.PAINTER.match(painter))):
            return self._send(404, b"not found", "text/plain")
        if path and not os.path.realpath(path).startswith(os.path.realpath(P.SESSIONS) + os.sep):
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
            ss = P.list_sessions()
            for s in ss:
                if s.get("painter") and (_warm is None or _warm.is_set()):  # (until the first scan is done, without)  # the picker's picture and title (the static export has them in sessions.json)
                    g = P.glance(s["files"])
                    s.update(look=g["look"], title=g["title"])
            if PUBLIC:
                ss = [{k: v for k, v in s.items() if k != "files"} for s in ss if s.get("painter")]
            return self._send(200, json.dumps(ss).encode(), "application/json")
        if u.path == "/api/glance":  # a painter's picture in the picker, read alone (not the painter's whole log)
            files = P.painter_files(painter)
            g = P.glance(files) if files else None
            im = P.glance_image(g["src"]) if g and g["src"] else None
            return self._send(200, im[1], im[0]) if im else self._send(404, b"not found", "text/plain")
        if u.path not in ("/api/events", "/api/file", "/img"):
            return self._send(404, b"not found", "text/plain")
        files = P.painter_files(painter) if painter else [path] if path and os.path.isfile(path) else []
        if not files:
            return self._send(404, b"no such session", "text/plain")
        st = P.stream(("p:" + painter) if painter else ("s:" + path), files)
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
            cwd, rels = P.painting_sources(st["events"])
            rel = q.get("f", [""])[0]
            want = os.path.realpath(os.path.join(cwd, rel))
            if cwd and rel in rels and want.startswith(os.path.realpath(cwd) + os.sep) and os.path.isfile(want):
                with open(want, "rb") as fh:
                    return self._send(200, fh.read(), "text/plain; charset=utf-8")
            return self._send(404, b"", "text/plain")
        if u.path == "/img":
            im = P.image(st, int(q.get("i", ["0"])[0]))
            if im:
                return self._send(200, base64.b64decode(im[1]), im[0])
        self._send(404, b"not found", "text/plain")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--host", default="127.0.0.1", help="e.g. this machine's Tailscale IP to watch from another of your devices")
    ap.add_argument("--sessions", default=P.SESSIONS, help="where pi keeps its session logs")
    ap.add_argument("--public", action="store_true", help="painters only, home path and account name scrubbed (a link for others)")
    a = ap.parse_args()
    P.SESSIONS = os.path.abspath(os.path.expanduser(a.sessions))
    global PUBLIC
    PUBLIC = a.public
    global _warm
    _warm = threading.Event()

    def warm():  # the picker's pictures and titles, read ahead (the first scan reads every log once: seconds)
        import time
        while True:
            for s in P.list_sessions():
                if s.get("painter"):
                    P.glance(s["files"])
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
