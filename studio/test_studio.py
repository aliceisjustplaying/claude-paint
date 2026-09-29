"""The studio viewer's server and static export, on synthetic session logs in a temp folder.

    uv run --with pytest pytest studio
"""
import http.server
import json
import os
import sys
import threading
import urllib.error
import urllib.request

import pytest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import studio as S

PAINTER = "paint-studio-abc123"


def line(d):
    return json.dumps(dict(d, timestamp="2026-09-29T10:00:00.000Z")) + "\n"


def start(cwd):
    return line({"type": "session", "cwd": cwd})


def call(i, lua):
    return line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": i, "name": "paint", "arguments": {"lua": lua}}]}})


def result(i, text):
    return line({"type": "message", "message": {"role": "toolResult", "toolCallId": i, "isError": False,
                                                 "content": [{"type": "text", "text": text}]}})


@pytest.fixture
def home(tmp_path):
    """A home with a painter's studio folder (brief, notes, easel, the painting) and its session folder."""
    studio = tmp_path / "src" / "a" / PAINTER
    for rel, body in {"BRIEF.md": "the brief", "notes/materials.md": "notes", "bin/easel": "binary",
                      "paintings/lua/painting.lua": "canvas{}"}.items():
        (studio / rel).parent.mkdir(parents=True, exist_ok=True)
        (studio / rel).write_text(body)
    sessions = tmp_path / ".pi" / "agent" / "sessions" / f"--Users-x-src-a-{PAINTER}--"
    sessions.mkdir(parents=True)
    return tmp_path, studio, sessions / "2026-09-29T10-00-00-000Z_s1.jsonl"


@pytest.fixture
def server(home, monkeypatch):
    monkeypatch.setattr(S, "SESSIONS", str(home[0] / ".pi" / "agent" / "sessions"))
    srv = http.server.ThreadingHTTPServer(("127.0.0.1", 0), S.H)
    threading.Thread(target=srv.serve_forever, daemon=True).start()
    yield lambda q: get(f"http://127.0.0.1:{srv.server_address[1]}{q}")
    srv.shutdown()
    srv.server_close()


def get(url):
    try:
        with urllib.request.urlopen(url) as r:
            return r.status, r.read()
    except urllib.error.HTTPError as e:
        return e.code, b""


def test_file_endpoint_serves_the_painting_source_only(home, server):
    _, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    assert server(f"/api/file?p={PAINTER}&f=paintings/lua/painting.lua") == (200, b"canvas{}")
    for f in ("BRIEF.md", "notes/materials.md", "bin/easel"):
        assert server(f"/api/file?p={PAINTER}&f={f}")[0] == 404, f
