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


def test_a_painter_whose_folder_is_gone_is_served_from_the_archive(home, server):
    # a round's worktree, removed: its painting's source is kept in archive/sources/<folder name>
    tmp_path, _, log = home
    gone = tmp_path / "src" / "a" / "claude-paint-r7-arm2"
    rel = "paintings/src/bin/pond_poplars.rs"
    log.write_text(start(str(gone)) + line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": "c1", "name": "bash", "arguments": {"command": "cargo paint pond_poplars"}}]}}))
    with open(os.path.join(os.path.dirname(HERE), "archive", "sources", "claude-paint-r7-arm2", rel), "rb") as fh:
        kept = fh.read()
    assert server(f"/api/file?p={PAINTER}&f={rel}") == (200, kept)
    out = tmp_path / "out"
    r = export(tmp_path, out)
    assert r.returncode == 0, r.stderr
    assert (out / "data" / PAINTER / "file" / rel).read_bytes() == kept


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


def look(i, png):
    """A look and the picture it returned."""
    import base64
    return (line({"type": "message", "message": {"role": "assistant", "content": [
                {"type": "toolCall", "id": i, "name": "look", "arguments": {}}]}})
            + line({"type": "message", "message": {"role": "toolResult", "toolCallId": i, "isError": False, "content": [
                {"type": "image", "mimeType": "image/png", "data": base64.b64encode(png).decode()}]}}))


def test_export_makes_small_web_copies_of_each_look_and_remakes_them_when_it_changes(home):
    Image = pytest.importorskip("PIL.Image")
    import io

    def png(w, h, color):
        buf = io.BytesIO()
        Image.new("RGB", (w, h), color).save(buf, "PNG")
        return buf.getvalue()

    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + look("l1", png(2400, 1600, (200, 30, 30))))
    out = tmp_path / "out"
    assert export(tmp_path, out).returncode == 0
    d = out / "data" / PAINTER
    assert json.loads((d / "events.json").read_text())["web"] == [1]
    assert Image.open(d / "img" / "0.png").size == (2400, 1600)  # the original, for the zoom
    assert max(Image.open(d / "v" / "0.jpg").size) == 1600
    assert max(Image.open(d / "t" / "0.jpg").size) <= 168

    # the session is rewritten with a different picture at the same index: its copies follow it
    log.write_text(start(str(studio)) + look("l1", png(1200, 800, (30, 30, 200))))
    assert export(tmp_path, out).returncode == 0
    assert Image.open(d / "v" / "0.jpg").convert("RGB").getpixel((5, 5))[2] > 150


def png_of(color):
    from PIL import Image
    import io
    buf = io.BytesIO()
    Image.new("RGB", (300, 200), color).save(buf, "PNG")
    return buf.getvalue()


def looks_at_once(*calls):
    """Several looks in one message ((id, arguments, png) each), their pictures coming back one result at a time."""
    import base64
    out = line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": i, "name": "look", "arguments": a} for i, a, _ in calls]}})
    for i, _, png in calls:
        out += line({"type": "message", "message": {"role": "toolResult", "toolCallId": i, "isError": False, "content": [
            {"type": "image", "mimeType": "image/png", "data": base64.b64encode(png).decode()}]}})
    return out


def say(text):
    return line({"type": "message", "message": {"role": "assistant", "content": [{"type": "text", "text": text}]}})


@pytest.mark.parametrize("via", ["live server", "static export"])
def test_the_picker_gets_the_newest_whole_look_and_the_title_from_the_closing_reply(home, server, via):
    # a whole look, then a whole look and a squint asked for together: the picture is the second whole look,
    # not the squint whose result came last; the title is the one the closing reply begins with
    pytest.importorskip("PIL")
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + look("l1", png_of("red"))
                   + looks_at_once(("l2", {"size": 800}, png_of("blue")), ("l3", {"mode": "squint"}, png_of("green")))
                   + say("**The Blue Barn**\n\nA barn at dusk."))
    if via == "live server":
        p = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
        code, picture = server(f"/api/glance?p={PAINTER}&i={p['look']}")
        assert code == 200
    else:
        out = tmp_path / "out"
        r = export(tmp_path, out)
        assert r.returncode == 0, r.stderr
        p = next(s for s in json.loads((out / "data" / "sessions.json").read_text()) if s["p"] == PAINTER)
        picture = (out / "data" / PAINTER / "img" / f"{p['look']}.png").read_bytes()
    assert (p["look"], p["title"]) == (1, "The Blue Barn")
    assert picture == png_of("blue")
