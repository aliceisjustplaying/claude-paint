"""The studio viewer's server and static export, on synthetic session logs in a temp folder.

    uv run --with pytest pytest studio
"""
import http.server
import json
import os
import subprocess
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


def test_a_late_result_reaches_a_client_that_already_had_the_call(home, server):
    _, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}"))
    r = json.loads(server(f"/api/events?p={PAINTER}&since=0")[1])
    i = next(k for k, e in enumerate(r["events"]) if e["kind"] == "paint")
    assert "out" not in r["events"][i]
    with open(log, "a") as fh:
        fh.write(result("c1", "ok · chunk 1"))
    r2 = json.loads(server(f"/api/events?p={PAINTER}&since={r['total']}&epoch={r['epoch']}&u={r['nupdates']}")[1])
    assert [(k, e["out"]) for k, e in r2["updates"]] == [(i, "ok · chunk 1")]
    # and only once
    r3 = json.loads(server(f"/api/events?p={PAINTER}&since={r2['total']}&epoch={r2['epoch']}&u={r2['nupdates']}")[1])
    assert r3["updates"] == []


def export(tmp_path, out):
    env = dict(os.environ, HOME=str(tmp_path))
    return subprocess.run([sys.executable, os.path.join(HERE, "export_static.py"), str(out)],
                          env=env, capture_output=True, text=True)


def old_export(tmp_path, out):
    """out as the exporter before the list left it: index.html and data/, no .studio-export."""
    assert export(tmp_path, out).returncode == 0
    (out / ".studio-export").unlink()


@pytest.mark.parametrize("old", [False, True], ids=["other folder", "old export plus a stray file"])
def test_export_refuses_a_folder_it_does_not_own(home, old):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    out = tmp_path / "out"
    out.mkdir()
    if old:
        old_export(tmp_path, out)
    before = sorted(str(p.relative_to(out)) for p in out.rglob("*"))
    (out / "keep.txt").write_text("not the exporter's")
    r = export(tmp_path, out)
    assert r.returncode != 0 and ".studio-export" in r.stderr
    assert sorted(str(p.relative_to(out)) for p in out.rglob("*")) == sorted(before + ["keep.txt"])


def test_export_adopts_an_export_from_before_the_list(home):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    out = tmp_path / "out"
    old_export(tmp_path, out)
    gone = out / "data" / "paint-studio-gone00" / "events.json"
    gone.parent.mkdir()
    gone.write_text("{}")  # a painter the old exporter wrote who is no longer listed
    r = export(tmp_path, out)
    assert r.returncode == 0, r.stderr
    assert not gone.parent.exists()
    assert (out / "data" / PAINTER / "events.json").exists()
    assert "data/sessions.json" in (out / ".studio-export").read_text().splitlines()


def test_export_prunes_only_files_it_wrote(home):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    out = tmp_path / "out"
    assert export(tmp_path, out).returncode == 0
    source = out / "data" / PAINTER / "file" / "paintings" / "lua" / "painting.lua"
    assert source.read_text() == "canvas{}"
    (out / "data" / "mine.txt").write_text("added by hand")
    (studio / "paintings" / "lua" / "painting.lua").unlink()
    r = export(tmp_path, out)
    assert r.returncode == 0, r.stderr
    assert not source.exists()  # the export's own file, stale now
    assert (out / "data" / "mine.txt").read_text() == "added by hand"
