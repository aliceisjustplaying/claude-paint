"""The studio viewer's server and static export, on synthetic session logs in a temp folder.

    uv run --with pytest pytest studio
"""
import http.server
import io
import json
import os
import re
import shutil
import subprocess
import sys
import threading
from html.parser import HTMLParser
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
    thread = threading.Thread(target=srv.serve_forever, kwargs={"poll_interval": 0.01}, daemon=True)
    thread.start()
    fetch = lambda q: get(fetch.base + q)
    fetch.base = f"http://127.0.0.1:{srv.server_address[1]}"
    try:
        yield fetch
    finally:
        srv.shutdown()
        srv.server_close()
        thread.join(timeout=1)
        assert not thread.is_alive()


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


@pytest.mark.parametrize("failed", [False, True])
@pytest.mark.parametrize("verbose", [False, True])
@pytest.mark.parametrize("notice", ["a fresh scratch canvas (the last one's log is paintings/lua/scratch-1.lua)\n",
                                  "a scratch canvas beside the painting, set up as the painting's canvas was\n"])
def test_scratch_reset_metadata_survives_result_truncation_and_errors(home, server, monkeypatch, failed, verbose, notice):
    root, studio, log = home
    monkeypatch.setattr(S, "PUBLIC", True)
    log.write_text(start(str(studio)) + line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": "scratch", "name": "paint", "arguments": {"scratch": True, "lua": "work{}"}}]}}))
    before = json.loads(server(f"/api/events?p={PAINTER}")[1])
    text = notice + ("x" * 1700 if verbose else "") + ("\n(the chunk failed and changed nothing)" if failed else "\nday 1, 09:00\nok")
    with log.open("a") as fh:
        fh.write(line({"type": "message", "message": {"role": "toolResult", "toolCallId": "scratch", "isError": failed,
                       "content": [{"type": "text", "text": text}]}}))
    after = json.loads(server(f"/api/events?p={PAINTER}&since={before['total']}&epoch={before['epoch']}&u={before['nupdates']}")[1])
    event = after["updates"][0][1]
    assert event["scratch_reset"] is True
    assert bool(event.get("err")) is failed
    assert len(event["out"]) <= 1500
    if verbose:
        assert notice.strip() not in event["out"]
    out = root / "exported"
    result = export(root, out)
    assert result.returncode == 0, result.stderr
    exported = json.loads((out / "data" / PAINTER / "events.json").read_text())
    assert next(e for e in exported["events"] if e["kind"] == "paint")["scratch_reset"] is True


@pytest.mark.parametrize("entry", ["compaction", "recovery"])
def test_saved_clock_survives_compaction_and_recovery_text_truncation(home, server, entry):
    _, studio, log = home
    summary = ("## Your journal\n" + "old notes\n" * 300
               + "\n## The canvas clock\n\n"
               "- Latest painting time the easel printed: day 1, 10:00\n"
               "- Latest journal entry stamped: day 2, 12:34\n\n"
               "## The canvas\nA picture.\n")
    record = ({"type": "compaction", "summary": summary} if entry == "compaction" else
              {"type": "message", "message": {"role": "user", "content": [{"type": "text", "text": summary}]}})
    log.write_text(start(str(studio)) + line(record))
    events = json.loads(server(f"/api/events?p={PAINTER}")[1])["events"]
    assert [e["clock"] for e in events if "clock" in e] == ["day 2, 12:34"]


def test_a_painter_whose_folder_is_gone_is_served_from_the_archive(home, server, monkeypatch):
    tmp_path, _, log = home
    gone = tmp_path / "src" / "a" / "claude-paint-r7-arm2"
    rel = "paintings/src/bin/pond_poplars.rs"
    archive = tmp_path / "stillwet-paintings"
    sources = archive / "other/claude-paint/sources"
    kept = b"fn main() {}\n"
    source = sources / gone.name / rel
    source.parent.mkdir(parents=True)
    source.write_bytes(kept)
    monkeypatch.setattr(S, "ARCHIVE", str(sources))
    monkeypatch.setenv("STILLWET_PAINTINGS", str(archive))
    log.write_text(start(str(gone)) + line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": "c1", "name": "bash", "arguments": {"command": "cargo paint pond_poplars"}}]}}))
    assert server(f"/api/file?p={PAINTER}&f={rel}") == (200, kept)
    out = tmp_path / "out"
    r = export(tmp_path, out)
    assert r.returncode == 0, r.stderr
    assert (out / "data" / PAINTER / "file" / rel).read_bytes() == kept


def export(tmp_path, out, *args):
    env = dict(os.environ, HOME=str(tmp_path))
    return subprocess.run([sys.executable, os.path.join(HERE, "export_static.py"), str(out), *args],
                          env=env, capture_output=True, text=True)


def metadata(path):
    class Head(HTMLParser):
        def handle_starttag(self, tag, attrs):
            attrs = dict(attrs)
            if tag == "meta":
                self.values[attrs.get("property") or attrs.get("name")] = attrs.get("content")
    head = Head()
    head.values = {}
    head.feed(path.read_text())
    return head.values


@pytest.mark.parametrize("via", ["live server", "static export"])
@pytest.mark.parametrize("status", ["finished", "cap_reached", "crash_limit_reached", "legacy cap", "legacy finished", "render only", "invalid"])
def test_runner_outcome_reaches_viewer_without_treating_export_as_completion(home, server, monkeypatch, via, status):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + say("I'm leaving it here."))
    run = tmp_path / "tmp" / "gallery-fixture" / "r24" / "run"
    rd = run / "INNS"
    rd.mkdir(parents=True)
    (run / "studios.json").write_text(json.dumps({"INNS1": PAINTER}))
    (rd / "p1.finished").write_text("render exported")
    expected = None
    if status == "legacy cap":
        (rd / "p1.painted").write_text("NOT FINISHED: stopped after 4 sittings (MAX_SITTINGS)")
        expected = "cap_reached"
    elif status == "legacy finished":
        (rd / "p1.painted").write_text("2 record(s): the painter is done: sitting 2 added no painting")
        expected = "finished"
    elif status != "render only":
        (rd / "p1_outcome.json").write_text(json.dumps({"status": status, "reason": "fixture stop reason"}))
        if status != "invalid":
            expected = status
    monkeypatch.setattr(S, "RUNS", str(tmp_path / "tmp" / "gallery-*" / "r*" / "run" / "studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0.0, "map": {}})
    if via == "live server":
        entries = json.loads(server("/api/sessions")[1])
    else:
        out = tmp_path / "out"
        r = export(tmp_path, out)
        assert r.returncode == 0, r.stderr
        entries = json.loads((out / "data" / "sessions.json").read_text())
        assert (out / "index.html").is_file()
    painter = next(s for s in entries if s.get("p") == PAINTER)
    assert (painter["outcome"]["status"] if painter["outcome"] else None) == expected


def old_export(tmp_path, out):
    """An export before asset files and the ownership list: only index.html and data/."""
    assert export(tmp_path, out).returncode == 0
    for p in out.iterdir():
        if p.name in ("index.html", "data"):
            continue
        if p.is_dir():
            shutil.rmtree(p)
        else:
            p.unlink()


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


@pytest.mark.parametrize("public", [False, True])
@pytest.mark.parametrize("lights", [("gallery",), ("relief",), S.DISPLAY_LIGHTS])
def test_gallery_companions_are_display_only_and_exported(home, server, monkeypatch, public, lights):
    import io
    Image = pytest.importorskip("PIL.Image")
    monkeypatch.setattr(S, "PUBLIC", public)
    buf = io.BytesIO()
    Image.new("RGB", (20, 16), (100, 80, 60)).save(buf, "PNG")
    colour = buf.getvalue()
    tmp_path, studio, log = home
    path = "out/easel/painting/look-0001.png"
    lit = {}
    for n, light in enumerate(lights):
        buf = io.BytesIO()
        Image.new("RGB", (20, 16), (120 + n * 30, 90, 70)).save(buf, "PNG")
        lit[light] = buf.getvalue()
        companion = studio / f"out/easel/painting/.{light}/look-0001.png"
        companion.parent.mkdir(parents=True)
        companion.write_bytes(lit[light])
    entry = look("l1", colour)
    rows = entry.splitlines()
    result_row = json.loads(rows[1])
    result_row["message"]["content"].insert(0, {"type": "text", "text": path + " (20x16)"})
    entry = rows[0] + "\n" + line(result_row)
    log.write_text(start(str(studio)) + entry + look("l2", colour))
    events = json.loads(server(f"/api/events?p={PAINTER}")[1])["events"]
    pictures = [e for e in events if e["kind"] == "image"]
    for light in S.DISPLAY_LIGHTS:
        assert [(e["img"], e.get(light, False)) for e in pictures] == [(0, light in lights), (1, False)]
    assert server(f"/img?p={PAINTER}&i=0") == (200, colour)
    for light in S.DISPLAY_LIGHTS:
        response = server(f"/img?p={PAINTER}&i=0&{light}=1")
        if light in lights:
            assert response == (200, lit[light])
        else:
            assert response[0] == 404
        assert server(f"/img?p={PAINTER}&i=1&{light}=1")[0] == 404
    second = log.with_name("2026-09-30T10-00-00-000Z_s2.jsonl")
    second.write_text(start(str(studio)) + entry)
    events = json.loads(server(f"/api/events?p={PAINTER}")[1])["events"]
    assert [e["img"] for e in events if e["kind"] == "image"] == [0, 1, 2]
    for light in lights:
        assert server(f"/img?p={PAINTER}&i=2&{light}=1") == (200, lit[light])
    out = tmp_path / "out"
    exported = export(tmp_path, out)
    assert exported.returncode == 0, exported.stderr
    d = out / "data" / PAINTER
    for light in S.DISPLAY_LIGHTS:
        if light in lights:
            assert (d / light / "0.png").read_bytes() == lit[light]
            assert (d / light / "2.png").read_bytes() == lit[light]
            assert f"data/{PAINTER}/{light}/0.png" in (out / ".studio-export").read_text().splitlines()
        else:
            assert not (d / light / "0.png").exists()
        assert not (d / light / "1.png").exists()
    assert (d / "img/0.png").read_bytes() == colour
    exported_events = json.loads((d / "events.json").read_text())["events"]
    for light in S.DISPLAY_LIGHTS:
        assert [e.get(light, False) for e in exported_events if e["kind"] == "image"] == [light in lights, False, light in lights]
        assert S.display_companion_path(str(studio), str(tmp_path / "outside.png"), light) is None
    assert S.display_companion_path(str(studio), path, "../outside") is None
    assert S.image_bytes(S.stream("p:" + PAINTER, [str(log), str(second)]), 0) == ("image/png", colour)


def test_background_companions_update_without_new_log_bytes(home, server):
    _, studio, log = home
    buf = io.BytesIO()
    S.Image.new("RGB", (20, 16), (100, 80, 60)).save(buf, "PNG")
    colour = buf.getvalue()
    log.write_text(start(str(studio)) + look("old", colour))
    second = log.with_name("2026-09-30T10-00-00-000Z_s2.jsonl")
    path = "out/easel/painting/65c0681641604c1b93c51bdcfd66302e.png"
    marker = studio / "out/easel/painting/.display" / os.path.basename(path)
    marker.parent.mkdir(parents=True)
    marker.write_bytes(b"pending\n")
    rows = look("new", colour).splitlines()
    result_row = json.loads(rows[1])
    result_row["message"]["content"].insert(0, {"type": "text", "text": path + " (20x16)"})
    second.write_text(start(str(studio)) + rows[0] + "\n" + line(result_row))
    initial = json.loads(server(f"/api/events?p={PAINTER}")[1])
    index = next(i for i, event in enumerate(initial["events"]) if event.get("img") == 1)
    assert initial["events"][index]["display_pending"] is True
    assert S.glance([str(log), str(second)])["look"] == 0
    for light in S.DISPLAY_LIGHTS:
        companion = studio / "out/easel/painting" / ("." + light) / os.path.basename(path)
        companion.parent.mkdir()
        companion.write_bytes(colour)
        assert server(f"/img?p={PAINTER}&i=1&{light}=1")[0] == 404
    query = f"/api/events?p={PAINTER}&since={initial['total']}&epoch={initial['epoch']}&u={initial['nupdates']}"
    assert json.loads(server(query)[1])["updates"] == []
    pending = marker.with_suffix(".part")
    pending.write_bytes(b"ready\n")
    pending.replace(marker)
    updated = json.loads(server(query)[1])
    assert updated["events"] == []
    assert [i for i, _ in updated["updates"]] == [index]
    event = updated["updates"][0][1]
    assert event["img"] == 1
    assert event["display_pending"] is False
    assert all(event[light] for light in S.DISPLAY_LIGHTS)
    assert S.glance([str(log), str(second)])["look"] == 1
    for light in S.DISPLAY_LIGHTS:
        assert server(f"/img?p={PAINTER}&i=1&{light}=1") == (200, colour)
    again = json.loads(server(query.rsplit("&u=", 1)[0] + f"&u={updated['nupdates']}")[1])
    assert again["updates"] == []


@pytest.mark.parametrize("light", S.DISPLAY_LIGHTS)
def test_reading_a_saved_survey_tile_gets_its_gallery_companion(home, server, light):
    import base64
    _, studio, log = home
    path = "out/easel/painting/tile.png"
    lit = b"\x89PNG\r\n\x1a\nfixture"
    companion = studio / f"out/easel/painting/.{light}/tile.png"
    companion.parent.mkdir(parents=True)
    companion.write_bytes(lit)
    log.write_text(start(str(studio)) + line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": "r1", "name": "read", "arguments": {"path": path}}]}})
        + line({"type": "message", "message": {"role": "toolResult", "toolCallId": "r1", "content": [
            {"type": "image", "mimeType": "image/png", "data": base64.b64encode(lit).decode()}]}}))
    events = json.loads(server(f"/api/events?p={PAINTER}")[1])["events"]
    assert next(e for e in events if e["kind"] == "image")[light] is True
    assert server(f"/img?p={PAINTER}&i=0&{light}=1") == (200, lit)


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
    first = metadata(out / "share" / (PAINTER + ".html"))
    assert first["og:image"].startswith(f"https://stillwet.art/studio/data/{PAINTER}/v/0.jpg?v=")
    assert (first["og:image:width"], first["og:image:height"], first["og:image:type"]) == ("1600", "1067", "image/jpeg")
    assert first["twitter:card"] == "summary_large_image"

    # the session is rewritten with a different picture at the same index: its copies follow it
    log.write_text(start(str(studio)) + look("l1", png(1200, 800, (30, 30, 200))))
    assert export(tmp_path, out).returncode == 0
    assert Image.open(d / "v" / "0.jpg").convert("RGB").getpixel((5, 5))[2] > 150
    second = metadata(out / "share" / (PAINTER + ".html"))
    assert second["og:image"] != first["og:image"]  # same index, different bytes: caches get a new URL
    assert (second["og:image:width"], second["og:image:height"]) == ("1200", "800")
    assert export(tmp_path, out).returncode == 0
    assert metadata(out / "share" / (PAINTER + ".html"))["og:image"] == second["og:image"]


def test_export_cards_follow_execution_completion_and_fallback(home, monkeypatch):
    # The exporter owns this behavior; real session and runner files are used. Only the OS's
    # current process directories are controlled so a stale record can outlive its process.
    import export_static as E
    pytest.importorskip("PIL")
    tmp_path, studio, log = home
    other = "paint-studio-def456"
    other_log = log.parent.with_name(f"--Users-x-src-a-{other}--") / log.name
    other_log.parent.mkdir()
    log.write_text(start(str(studio)) + look("l1", png_of("red")) + say('**Barn & "Sky" <west>**\n\nA barn.'))
    other_log.write_text(start(str(studio)) + look("l1", png_of("blue")))
    run = tmp_path / "tmp" / "gallery-fixture" / "r24" / "run"
    for lane in ("LIVE", "DONE"):
        (run / lane).mkdir(parents=True)
    (run / "studios.json").write_text(json.dumps({"LIVE1": PAINTER, "DONE1": other}))
    running = run / "LIVE" / "p1_sittings.json"
    running.write_text('[{"status":"running"}]')
    (run / "DONE" / "p1_outcome.json").write_text('{"status":"finished","reason":"reviewed without painting"}')
    monkeypatch.setattr(S, "SESSIONS", str(tmp_path / ".pi" / "agent" / "sessions"))
    monkeypatch.setattr(S, "RUNS", str(run / "studios.json"))
    monkeypatch.setattr(E, "working_studios", lambda: {PAINTER}, raising=False)
    out = tmp_path / "out"
    monkeypatch.setattr(sys, "argv", ["export_static.py", str(out)])

    def render():
        E.WRITTEN.clear()
        S._lanes.update(at=0, map={})
        E.main()
        return metadata(out / "index.html"), metadata(out / "share" / (PAINTER + ".html"))

    front, card = render()
    assert 'Barn & "Sky" <west>' in card["og:title"]  # decoded attribute survives escaping
    assert "&quot;Sky&quot; &lt;west&gt;" in (out / "share" / (PAINTER + ".html")).read_text()
    assert "Live" in front["og:title"]
    assert front["og:image"] == card["og:image"]
    assert card["og:url"] == f"https://stillwet.art/studio/?p={PAINTER}"
    assert front["og:url"] == "https://stillwet.art/studio/"
    assert metadata(out / "share" / (other + ".html"))["og:image"] != card["og:image"]

    # A dead process or an old session cannot leave a saved running record live forever.
    monkeypatch.setattr(E, "working_studios", lambda: set())
    front, card = render()
    assert "Not currently painting" in card["og:title"]
    assert front["og:image"] == metadata(out / "share" / (other + ".html"))["og:image"]
    monkeypatch.setattr(E, "working_studios", lambda: {PAINTER})
    old = log.stat().st_mtime - 3600
    os.utime(log, (old, old))
    front, card = render()
    assert "Live" not in card["og:title"]

    # Runner completion wins even with process and running-record evidence left behind.
    os.utime(log, None)
    (run / "LIVE" / "p1_outcome.json").write_text('{"status":"finished","reason":"reviewed without painting"}')
    front, card = render()
    assert "Finished" in card["og:title"]
    assert front["og:image"] == card["og:image"]


def test_agy_tool_server_marks_its_studio_working_and_live_only_with_a_running_sitting(home, monkeypatch):
    import export_static as E
    tmp_path, studio, log = home
    agy, done = "paint-studio-063cbb", "paint-studio-d64bb7"
    tables = {
        ("ps", "-Ao", "pid=,comm="): "  501 /opt/bin/pi\n  502 agy\n",
        ("ps", "-wwAo", "args="): "\n".join([
            f"/bin/uv run --no-project --script /Users/x/src/a/{agy}/bin/mcp/server.py --studio /Users/x/src/a/{agy}"
            " --sittings --agy --break-seconds 0 --sessions-root /Users/x/.pi/agent/sessions",
            f"/env/bin/python /Users/x/Art Work/{done}/bin/mcp/server.py --studio /Users/x/Art Work/{done} --sittings --agy",
            "/env/bin/python /Users/x/work/watch.py --studio /Users/x/src/a/paint-studio-aaa111 --agy-dir /Users/x/.gemini",
            "/env/bin/python /Users/x/src/a/paint-studio-bbb222/bin/mcp/server.py --studio /Users/x/src/a/paint-studio-bbb222",
            "agy --agent easel-painter", ""]),
    }

    def run(cmd, **kw):
        out = tables.get(tuple(cmd)) or ("p501\nfcwd\nn/Users/x/src/a/" + PAINTER + "\n" if cmd[1:3] == ["-a", "-p"] else "")
        return subprocess.CompletedProcess(cmd, 0, out, "")
    monkeypatch.setattr(E.subprocess, "run", run)
    monkeypatch.setattr(E.platform, "system", lambda: "Darwin")
    assert E.working_studios() == {PAINTER, agy, done}

    run_dir = tmp_path / "src/a/stillwet-runs/cowboy-zombie-agy/run"
    (run_dir / "GEMINI").mkdir(parents=True)
    (run_dir / "studios.json").write_text(json.dumps({"GEMINI1": agy}))
    (run_dir / "GEMINI" / "p1_sittings.json").write_text('[{"sitting": 1, "part": 1, "status": "running"}]')
    monkeypatch.setattr(S, "RUNS", str(tmp_path / "src/a/stillwet-runs/**/run/studios.json"))
    sessions = [{"p": agy, "mtime": 1000.0}, {"p": done, "mtime": 1000.0}]
    E.runner_states(sessions, E.working_studios(), 1060.0)
    assert [s["active"] for s in sessions] == [True, False]  # a finished agy left open has no running sitting


def test_linux_working_studios_reads_proc_without_lsof(monkeypatch):
    import export_static as E
    agy = "paint-studio-063cbb"

    def run(cmd, **kw):
        if cmd == ["ps", "-Ao", "pid=,comm="]:
            return subprocess.CompletedProcess(cmd, 0, "501 pi\n502 pi\n503 pi\n", "")
        assert cmd == ["ps", "-wwAo", "args="]
        return subprocess.CompletedProcess(cmd, 0, f"uv /home/user/{agy}/bin/mcp/server.py --studio /home/user/{agy} --agy", "")

    def readlink(path):
        if path == "/proc/501/cwd":
            return "/home/user/Art Work/" + PAINTER
        raise PermissionError(path)

    monkeypatch.setattr(E.platform, "system", lambda: "Linux")
    monkeypatch.setattr(E.subprocess, "run", run)
    monkeypatch.setattr(E.os, "readlink", readlink)
    assert E.working_studios() == {PAINTER, agy}


@pytest.mark.parametrize("state,working,age,outcome,active", [
    ('{"sitting":1,"complete":false}', True, 60, None, True),
    ('{"sitting":1,"complete":true}', True, 60, None, False),
    ('{"sitting":1,"complete":false}', False, 60, None, False),
    ('{"sitting":1,"complete":false}', True, 1800, None, False),
    ('{"sitting":1,"complete":false}', True, -1, None, False),
    ('{"sitting":1,"complete":false}', True, 60, {"status": "finished"}, False),
    (None, True, 60, None, False),
    ('{"complete":', True, 60, None, False),
    ('[]', True, 60, None, False),
    ('{}', True, 60, None, False),
])
def test_native_sitting_activity_requires_process_and_recent_session(home, monkeypatch, state, working, age, outcome, active):
    import export_static as E
    root, studio, log = home
    monkeypatch.setenv("HOME", str(root))
    monkeypatch.setattr(S, "RUNS", str(root / "no-runner" / "studios.json"))
    if state is not None:
        (studio / "out/easel").mkdir(parents=True)
        (studio / "out/easel/sittings.json").write_text(state)
    session = {"p": PAINTER, "mtime": 1000, "outcome": outcome}
    E.runner_states([session], {PAINTER} if working else set(), 1000 + age)
    assert session["active"] is active
    assert session["completed_at"] == (1000 if outcome else None)


def test_native_sitting_exports_an_untitled_painter_as_live_without_overriding_runner_state(home, monkeypatch):
    import export_static as E
    root, studio, log = home
    monkeypatch.setenv("HOME", str(root))
    monkeypatch.setattr(S, "SESSIONS", str(root / ".pi/agent/sessions"))
    monkeypatch.setattr(S, "RUNS", str(root / "run/studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0.0, "map": {}})
    monkeypatch.setattr(E, "working_studios", lambda: {PAINTER})
    log.write_text(start(str(studio)) + line({"type": "model_change", "modelId": "gemini-4-argon-high"})
                   + look("l1", png_of("blue")))
    (studio / "out/easel").mkdir(parents=True)
    (studio / "out/easel/sittings.json").write_text('{"sitting":1,"complete":false}')
    out = root / "export"
    monkeypatch.setattr(sys, "argv", ["export_static.py", str(out)])
    E.WRITTEN.clear()
    E.main()
    session, = json.loads((out / "data/sessions.json").read_text())
    assert session["active"] is True
    assert session["title"] is None
    assert session["model"] == "gemini-4-argon-high"
    assert "Live" in metadata(out / "index.html")["og:title"]
    assert "Untitled, in progress" in (out / "index.html").read_text()

    (root / "run/ARGON").mkdir(parents=True)
    (root / "run/studios.json").write_text(json.dumps({"ARGON1": PAINTER}))
    (root / "run/ARGON/p1_sittings.json").write_text('[{"status":"finished"}]')
    E.runner_states([session], {PAINTER}, session["mtime"])
    assert session["active"] is False


def test_a_painter_working_elsewhere_exports_as_live(home, monkeypatch):
    import export_static as E
    root, studio, log = home
    monkeypatch.setenv("HOME", str(root))
    monkeypatch.setattr(S, "SESSIONS", str(root / ".pi/agent/sessions"))
    monkeypatch.setattr(S, "RUNS", str(root / "run/studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0.0, "map": {}})
    monkeypatch.setattr(E, "working_studios", set)
    log.write_text(start(str(studio)) + look("l1", png_of("blue")))
    out = root / "export"
    for args, active in (([], False), (["--elsewhere", PAINTER], True)):
        monkeypatch.setattr(sys, "argv", ["export_static.py", str(out), *args])
        E.WRITTEN.clear()
        E.main()
        session, = json.loads((out / "data/sessions.json").read_text())
        assert session["active"] is active


def test_a_second_export_in_one_process_skips_unchanged_looks(home, monkeypatch):
    import export_static as E
    root, studio, log = home
    monkeypatch.setenv("HOME", str(root))
    monkeypatch.setattr(S, "SESSIONS", str(root / ".pi/agent/sessions"))
    monkeypatch.setattr(S, "RUNS", str(root / "run/studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0.0, "map": {}})
    monkeypatch.setattr(E, "EXPORTED", {})
    log.write_text(start(str(studio)) + look("l1", png_of("blue")))
    out = root / "export"
    E.WRITTEN.clear()
    E.export(str(out))
    first = sorted(E.WRITTEN)
    read, image_bytes = [], S.image_bytes
    monkeypatch.setattr(S, "image_bytes", lambda *a, **k: read.append(a) or None)
    with open(log, "a") as fh:
        fh.write(line({"type": "message", "message": {"role": "assistant", "content": [{"type": "text", "text": "More sky."}]}}))
    E.WRITTEN.clear()
    E.export(str(out))
    assert read == []
    assert sorted(E.WRITTEN) == first
    assert (out / "data" / PAINTER / "img/0.png").is_file()
    assert json.loads((out / "data" / PAINTER / "events.json").read_text())["imgext"] == ["png"]

    monkeypatch.setattr(S, "image_bytes", image_bytes)
    replaced = log.with_suffix(".new")
    replaced.write_text(start(str(studio)) + look("l1", png_of("red")))
    os.replace(replaced, log)
    E.WRITTEN.clear()
    E.export(str(out))
    assert (out / "data" / PAINTER / "img/0.png").read_bytes() == png_of("red")


@pytest.mark.parametrize("failed_light", [None, *S.DISPLAY_LIGHTS])
def test_export_retries_failed_image_reads_without_source_changes(home, monkeypatch, failed_light):
    import export_static as E
    root, studio, log = home
    monkeypatch.setenv("HOME", str(root))
    monkeypatch.setattr(S, "SESSIONS", str(root / ".pi/agent/sessions"))
    monkeypatch.setattr(S, "RUNS", str(root / "run/studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0.0, "map": {}})
    monkeypatch.setattr(E, "EXPORTED", {})
    png = png_of("blue")
    path = "out/easel/painting/look-0001.png"
    for light in S.DISPLAY_LIGHTS:
        companion = studio / f"out/easel/painting/.{light}/look-0001.png"
        companion.parent.mkdir(parents=True)
        companion.write_bytes(png)
    rows = look("l1", png).splitlines()
    entry = json.loads(rows[1])
    entry["message"]["content"].insert(0, {"type": "text", "text": path + " (20x16)"})
    log.write_text(start(str(studio)) + rows[0] + "\n" + line(entry))
    reads, image_bytes = [], S.image_bytes
    failing = True

    def read_image(st, i, light=None):
        reads.append(light)
        return None if failing and light == failed_light else image_bytes(st, i, light=light)

    monkeypatch.setattr(S, "image_bytes", read_image)
    out = root / "export"
    E.export(str(out))
    dest = out / "data" / PAINTER
    assert not (dest / (failed_light or "img") / "0.png").exists()
    st = S.stream("p:" + PAINTER, S.painter_files(PAINTER))
    sources = E.image_sources(st, 0)
    assert sources and all(sources)
    failing = False
    E.export(str(out))
    assert E.image_sources(st, 0) == sources
    for folder in ("img", *S.DISPLAY_LIGHTS):
        assert (dest / folder / "0.png").read_bytes() == png
    assert json.loads((dest / "events.json").read_text())["imgext"] == ["png"]
    reads.clear()
    E.export(str(out))
    assert reads == []


def test_export_never_promotes_stale_web_copies_after_failed_conversion(home, monkeypatch):
    import export_static as E
    root, studio, log = home
    monkeypatch.setenv("HOME", str(root))
    monkeypatch.setattr(S, "SESSIONS", str(root / ".pi/agent/sessions"))
    monkeypatch.setattr(S, "RUNS", str(root / "run/studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0.0, "map": {}})
    monkeypatch.setattr(E, "EXPORTED", {})
    log.write_text(start(str(studio)) + look("l1", png_of("blue")))
    out = root / "export"
    E.export(str(out))
    dest = out / "data" / PAINTER
    assert json.loads((dest / "events.json").read_text())["web"] == [1]
    assert all((dest / folder / "0.jpg").is_file() for folder in ("t", "v"))

    truncated = png_of("red")[:48]
    with pytest.raises(OSError):
        E.Image.open(io.BytesIO(truncated)).load()
    replaced = log.with_suffix(".new")
    replaced.write_text(start(str(studio)) + look("l1", truncated))
    os.replace(replaced, log)
    for _ in range(2):
        E.export(str(out))
        assert (dest / "img/0.png").read_bytes() == truncated
        assert json.loads((dest / "events.json").read_text())["web"] == [0]
        assert (str(out), PAINTER, 0) not in E.EXPORTED
        assert metadata(out / "share" / (PAINTER + ".html"))["og:image"] == E.DEFAULT_IMAGE["url"]


def test_empty_and_imageless_exports_use_default_card(home):
    tmp_path, studio, log = home
    out = tmp_path / "out"
    assert export(tmp_path, out).returncode == 0
    assert metadata(out / "index.html")["og:image"] == "https://stillwet.art/img/og-card.jpg"
    log.write_text(start(str(studio)) + say("**No image yet**\n\nA painting."))
    assert export(tmp_path, out).returncode == 0
    card = metadata(out / "share" / (PAINTER + ".html"))
    assert card["og:image"] == "https://stillwet.art/img/og-card.jpg"
    assert "Live" not in card["og:title"]


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


def test_a_held_knife_is_never_the_picker_fallback(home):
    pytest.importorskip("PIL")
    _, studio, log = home
    log.write_text(start(str(studio)) + looks_at_once(
        ("hold", {"hold": "p", "at": "50,50"}, png_of("blue"))))
    assert S.glance([str(log)])["look"] is None


@pytest.mark.parametrize("replacement", ["atomic", "in-place"])
def test_picker_refreshes_after_same_size_session_replacement(home, server, replacement):
    _, studio, log = home
    entries, pictures = [], []
    for color, title in (("blue", "Blue"), ("red", "Reds")):
        buf = io.BytesIO()
        S.Image.new("RGB", (4, 4), color).save(buf, "PNG", compress_level=0)
        pictures.append(buf.getvalue())
        entries.append(start(str(studio)) + look("l1", pictures[-1]) + say(f"**{title}**\n\nA painting."))
    assert len(entries[0].encode()) == len(entries[1].encode())
    log.write_text(entries[0])
    before = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
    events = json.loads(server(f"/api/events?p={PAINTER}")[1])
    assert (before["look"], before["title"]) == (0, "Blue")
    stat = log.stat()
    if replacement == "atomic":
        pending = log.with_suffix(".new")
        pending.write_text(entries[1])
        os.utime(pending, ns=(stat.st_atime_ns, stat.st_mtime_ns))
        pending.replace(log)
        assert log.stat().st_ino != stat.st_ino
    else:
        log.write_text(entries[1])
        os.utime(log, ns=(stat.st_atime_ns, stat.st_mtime_ns + 1_000_000_000))
        assert log.stat().st_ino == stat.st_ino
    assert log.stat().st_size == stat.st_size
    updated = json.loads(server(f"/api/events?p={PAINTER}")[1])
    assert updated["epoch"] > events["epoch"]
    assert server(f"/img?p={PAINTER}&i=0") == (200, pictures[1])
    after = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
    assert (after["look"], after["title"]) == (0, "Reds")
    assert after["thumb_version"] != before["thumb_version"]


def test_glance_only_parses_appended_lines(home, monkeypatch):
    _, studio, log = home
    request, response = look("l2", png_of("red")).splitlines(keepends=True)
    log.write_text(start(str(studio)) + look("l1", png_of("blue")) + say("**Blue**") + request)
    cached = S._glance_file(str(log))
    version = cached["whole_version"]
    appended = response + say("**Reds**")
    with log.open("a") as fh:
        fh.write(appended)
    parsed, loads = [], S.json.loads
    monkeypatch.setattr(S.json, "loads", lambda line: parsed.append(line) or loads(line))
    after = S.glance([str(log)])
    assert S._glances[str(log)] is cached
    assert (cached["n"], after["look"], after["title"]) == (2, 1, "Reds")
    assert after["version"] != version
    assert parsed == appended.encode().splitlines(keepends=True)
    assert S.glance([str(log)]) == after
    assert parsed == appended.encode().splitlines(keepends=True)


def test_glance_is_a_small_cached_jpeg(home, server, monkeypatch):
    import hashlib
    from PIL import Image
    _, studio, log = home
    buf = io.BytesIO()
    Image.new("RGB", (1000, 625), "blue").save(buf, "PNG", compress_level=0)
    log.write_text(start(str(studio)) + look("l1", buf.getvalue()))
    monkeypatch.setattr(S, "PUBLIC", True)
    monkeypatch.setattr(S, "_thumbnails", type(S._thumbnails)())
    monkeypatch.setattr(S, "GLANCE_CACHE_SIZE", 2)
    p = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
    url = server.base + f"/api/glance?p={PAINTER}&i={p['look']}&epoch={p['epoch']}&version={p['thumb_version']}"
    with urllib.request.urlopen(url) as response:
        picture = response.read()
        assert response.headers["Content-Type"] == "image/jpeg"
        assert response.headers["Cache-Control"] == "private, max-age=300"
        etag = response.headers["ETag"]
        assert etag == '"' + hashlib.sha256(picture).hexdigest() + '"'
    with Image.open(io.BytesIO(picture)) as thumbnail:
        assert thumbnail.format == "JPEG"
        assert thumbnail.size == (360, 225)
        assert max(thumbnail.size) <= S.GLANCE_SIZE
    with monkeypatch.context() as cached:
        cached.setattr(S.Image, "open", lambda *args: pytest.fail("thumbnail was decoded again"))
        assert get(url) == (200, picture)
        request = urllib.request.Request(url, headers={"If-None-Match": etag})
        with pytest.raises(urllib.error.HTTPError) as unchanged:
            urllib.request.urlopen(request)
        assert unchanged.value.code == 304
        assert unchanged.value.read() == b""
        assert unchanged.value.headers["ETag"] == etag
        assert unchanged.value.headers["Cache-Control"] == "private, max-age=300"
    size = log.stat().st_size
    for color in ("red", "green"):
        rewritten = io.BytesIO()
        Image.new("RGB", (1000, 625), color).save(rewritten, "PNG", compress_level=0)
        log.write_text(start(str(studio)) + look("l1", rewritten.getvalue()))
        assert log.stat().st_size == size
        with urllib.request.urlopen(request) as response:
            updated = response.read()
            assert response.status == 200
            assert updated != picture
            assert response.headers["ETag"] == '"' + hashlib.sha256(updated).hexdigest() + '"'
    assert len(S._thumbnails) == 2
    assert hashlib.sha256(buf.getvalue()).digest() not in S._thumbnails
    assert hashlib.sha256(rewritten.getvalue()).digest() in S._thumbnails


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
    if via == "live server":
        from PIL import Image
        assert Image.open(io.BytesIO(picture)).getpixel((0, 0)) == (0, 0, 254)
    else:
        assert picture == png_of("blue")


@pytest.mark.parametrize("via", ["live server", "static export", "atelier export"])
def test_mask_looks_never_replace_the_painting(home, server, via):
    from PIL import Image
    import atelier_export as A

    tmp_path, studio, log = home
    out = tmp_path / "out"
    work = atelier_work(tmp_path / "atelier", "test", 1, PAINTER)

    def painting():
        sessions = json.loads(server("/api/sessions")[1])
        if via == "atelier export":
            current = A.easel("test", 1, str(work), [dict(s, active=False) for s in sessions], str(out))
            return None if current["look"] is None else (out / "test/1/look.jpg").read_bytes()
        if via == "static export":
            r = export(tmp_path, out)
            assert r.returncode == 0, r.stderr
            sessions = json.loads((out / "data/sessions.json").read_text())
        current = next(s for s in sessions if s.get("p") == PAINTER)
        if current["look"] is None:
            return None
        if via == "static export":
            return (out / "data" / PAINTER / "img" / f"{current['look']}.png").read_bytes()
        code, picture = server(f"/api/glance?p={PAINTER}&i={current['look']}")
        assert code == 200
        return picture

    log.write_text(start(str(studio)) + looks_at_once(("mask1", {"mask": "sky"}, png_of("red"))))
    assert painting() is None
    with open(log, "a") as fh:
        fh.write(look("plain", png_of("blue"))
                 + looks_at_once(("mask2", {"mask": "sky", "size": 800}, png_of("green"))))
    pixel = Image.open(io.BytesIO(painting())).getpixel((0, 0))
    assert pixel[0] < 5 and pixel[1] < 5 and pixel[2] > 245


@pytest.mark.parametrize("args", [{"mask": "sky"}, {"mask": "sky", "size": 800},
                                  {"size": 800, "mask": "sky"}, {"mode": "normal", "mask": "sky"}])
def test_a_mask_look_is_not_the_whole_painting(args):
    assert not S.is_whole(S.look_text(args))


@pytest.mark.parametrize("asset,ctype", [
    ("stream.css", "text/css"),
    ("code-display.js", "text/javascript"),
    ("code-format.js", "text/javascript"),
    ("vendor/stylua/stylua_lib_web.js", "text/javascript"),
    ("vendor/stylua/stylua_lib_bg.wasm", "application/wasm"),
    ("vendor/stylua/LICENSE.md", "text/plain"),
])
def test_viewer_assets_load_on_the_public_server_and_in_the_export(home, server, monkeypatch, asset, ctype):
    # Styles, module workers and WASM need the correct MIME types and unmodified bytes.
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    with open(os.path.join(HERE, asset), "rb") as fh:
        body = fh.read()
    monkeypatch.setattr(S, "PUBLIC", True)
    with urllib.request.urlopen(server.base + "/" + asset) as r:
        assert (r.status, r.headers.get_content_type(), r.read()) == (200, ctype, body)
    out = tmp_path / "out"
    r = export(tmp_path, out)
    assert r.returncode == 0, r.stderr
    assert (out / asset).read_bytes() == body


def read_picture(i, path, png):
    """A read of an image file and its result, as pi's read tool writes it."""
    import base64
    return (line({"type": "message", "message": {"role": "assistant", "content": [
                {"type": "toolCall", "id": i, "name": "read", "arguments": {"path": path}}]}})
            + line({"type": "message", "message": {"role": "toolResult", "toolCallId": i, "toolName": "read", "isError": False,
                    "content": [{"type": "text", "text": "Read image file [image/png]"},
                                {"type": "image", "mimeType": "image/png", "data": base64.b64encode(png).decode()}]}}))


def held_look(i, args, said, png):
    """A look held up to the canvas, its picture and the easel's words about it."""
    import base64
    return (line({"type": "message", "message": {"role": "assistant", "content": [
                {"type": "toolCall", "id": i, "name": "look", "arguments": args}]}})
            + line({"type": "message", "message": {"role": "toolResult", "toolCallId": i, "isError": False, "content": [
                {"type": "text", "text": said}, {"type": "image", "mimeType": "image/png", "data": base64.b64encode(png).decode()}]}}))


def test_a_stick_held_without_a_pose_is_known_by_the_easels_reply(home):
    # a pastel stick held with no pose or side takes the easel's default pose: its request reads as a knife's, and
    # only the reply (main.rs) says which was held
    pytest.importorskip("PIL")
    _, studio, log = home
    log.write_text(start(str(studio))
                   + held_look("k", {"hold": "k1", "at": "50,50"},
                               "look-0001.png (400x300, 0.08s): k1 held up to the canvas at 50,50\n", png_of("blue"))
                   + held_look("s", {"hold": "P.glow", "at": "50.5,-2"},
                               "look-0002.png (400x300, 0.31s): P.glow held at 50.5,-2: touching 0.4 mm², crumbs settle over 1.2 mm² more\n",
                               png_of("red"))
                   + held_look("p", {"hold": "P.glow", "at": "60,60", "side": "30"},
                               "[scratch canvas]\nlook-0003.png (400x300, 0.30s): P.glow held at 60,60: touching 2.1 mm², crumbs settle over 3.0 mm² more\n",
                               png_of("green")))
    events, _ = S.parse(str(log))
    assert [e.get("held") for e in events if e["kind"] == "look"] == [None, "stick", "stick"]
    assert [e.get("held") for e in events if e["kind"] == "image"] == [None, "stick", "stick"]


@pytest.mark.parametrize("via", ["live server", "static export"])
def test_a_reference_picture_is_marked_and_is_never_the_painters_picture(home, server, via):
    # pictures read from the studio's reference/ are another painter's work, given to this one to study
    pytest.importorskip("PIL")
    tmp_path, studio, log = home
    out = tmp_path / "out"

    def painter_and_events():
        if via == "live server":
            p = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
            code, picture = server(f"/api/glance?p={PAINTER}&i={p['look']}")
            return p, picture if code == 200 else None, json.loads(server(f"/api/events?p={PAINTER}&since=0")[1])["events"]
        r = export(tmp_path, out)
        assert r.returncode == 0, r.stderr
        p = next(s for s in json.loads((out / "data" / "sessions.json").read_text()) if s["p"] == PAINTER)
        picture = None if p["look"] is None else (out / "data" / PAINTER / "img" / f"{p['look']}.png").read_bytes()
        return p, picture, json.loads((out / "data" / PAINTER / "events.json").read_text())["events"]

    # it has only studied a reference so far: it has no picture yet
    log.write_text(start(str(studio)) + read_picture("r1", "reference/his.png", png_of("green")))
    p, picture, _ = painter_and_events()
    assert (p["look"], picture) == (None, None)

    # Diagnostic looks cannot become the picker image through its legacy fallback.
    with open(log, "a") as fh:
        fh.write(looks_at_once(("s1", {"survey": True}, png_of("red")),
                              ("c1", {"compare": "out/earlier.png"}, png_of("green"))))
    p, picture, _ = painter_and_events()
    assert (p["look"], picture) == (None, None)

    # Older painters read their renders; that fallback still works without a whole look.
    with open(log, "a") as fh:
        fh.write(read_picture("old", "out/easel/look-0000.png", png_of("red")))
    p, picture, _ = painter_and_events()
    assert p["look"] == 3
    if via == "live server":
        from PIL import Image
        assert Image.open(io.BytesIO(picture)).getpixel((0, 0)) == (254, 0, 0)
    else:
        assert picture == png_of("red")

    # its own canvas, then the reference again (by its full path), then a render it reads as the early painters did
    with open(log, "a") as fh:
        fh.write(look("l1", png_of("blue")) + read_picture("r2", f"{studio}/reference/his.png", png_of("green"))
                 + read_picture("r3", "out/easel/look-0001.png", png_of("red")))
    p, picture, events = painter_and_events()
    assert p["look"] == 4
    if via == "live server":
        from PIL import Image
        assert Image.open(io.BytesIO(picture)).getpixel((0, 0)) == (0, 0, 254)
    else:
        assert picture == png_of("blue")
    assert [bool(e.get("ref")) for e in events if e["kind"] == "image"] == [True, False, False, False, False, True, False]
    assert [bool(e.get("ref")) for e in events if e["kind"] == "read"] == [True, False, True, False]


TONN = ("Compose and paint one original picture in the manner of Kendric Tonn, at\nthe easel, a simulator of oil paint "
        "on linen. {}, there with his permission: study {} for his\nmanner.")


@pytest.mark.parametrize("brief, says", [
    (TONN.format("Pictures of his paintings are\nin reference/", "them"), ("Tonn", "Kendric Tonn", "Kendric Tonn")),
    (TONN.format("A picture of one of his\npaintings is in reference/", "it"), ("Tonn", "Kendric Tonn", "Kendric Tonn")),
    ("Compose and paint one original landscape in the manner of Caspar David\nFriedrich, at the easel. Work from "
     "knowledge and the\nnotes in your studio; don't use reference images, image models or pictures\nof his work.",
     ("Friedrich", "Caspar David Friedrich", "")),
], ids=["his pictures in reference/", "one picture in reference/", "no reference pictures"])
def test_the_brief_names_the_artist_and_whose_paintings_the_reference_pictures_are(home, server, monkeypatch, brief, says):
    tmp_path, studio, log = home
    monkeypatch.setenv("HOME", str(tmp_path))  # the brief is read from ~/src/a/<the painter's folder>/BRIEF.md
    (studio / "BRIEF.md").write_text("# Paint a picture\n\n" + brief + "\n")
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    p = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
    assert (p["subject"], p["artist"], p["reference_artist"]) == says


@pytest.mark.parametrize("say, title", [
    ("**The Blue Barn**\n\nA barn at dusk.", "The Blue Barn"),
    # a sign-off line first, the title alone on the next paragraph (Space Bunny, paint-studio-ba1453)
    ("I'm stopping here. The painting is finished as far as I can take it.\n\n**Quinces, Raking Light**\n\nA low-key still life.", "Quinces, Raking Light"),
    ("I'll start by reading the brief.", None),
    ("Next I'll **glaze** the sky.\n\nThen the trees.", None),
    # named in the first line (paint-studio-49b647, -12e572, -c8bb05), quotes inside the bold (-b3a3c7)
    ("I've called the painting **Luncheonette, Four O'Clock**.\n\nA corner.", "Luncheonette, Four O'Clock"),
    ("I've finished **Nymphéas, reflet du soir** (*Water Lilies, Evening Reflection*). The clock now stands at day 213.", "Nymphéas, reflet du soir"),
    ("I've left **Grainstack, Evening** as it was at day 415.", "Grainstack, Evening"),
    ("I'm leaving **Luncheonette, Four O'Clock** finished as it is.", "Luncheonette, Four O'Clock"),
    ('**"Self-Portrait with Palette"**\n\nA half-length figure.', "Self-Portrait with Palette"),
    ("I finished the sky, then **glazed** it.", None),
])
def test_the_title_is_where_painters_put_it_in_their_closing_words(say, title):
    assert S.title_of(say) == title


def test_a_last_sitting_that_only_looked_keeps_the_earlier_title():
    # paint-studio-b3a3c7: the second sitting's closing words don't repeat the title
    assert S.title_of_closings(["**The Blue Barn**\n\nA barn.", "The painting is exactly as the journal left it."]) == "The Blue Barn"


def test_title_labels_allow_nested_emphasis_and_skip_reply_headings():
    assert S.title_of("**Title: *Cleaners, Late***\n\nThe painting is finished.") == "Cleaners, Late"
    assert S.title_of("**Saved painting:** `out/painting.png`\n**Log:** `painting.lua`\n\n"
                      "**Title:** *The Last Light on the Flats*\n\nA dusk estuary.") == "The Last Light on the Flats"
    assert S.title_of("**Saved painting:** `out/painting.png`\n\n**Handoff:**\n\nStill unresolved.") is None


def test_brief_artist_can_have_a_period_qualifier_or_materials_instruction():
    assert S._subject_of("Paint in the manner of later Claude Monet, at the easel.") == "Monet"
    assert S._artist_of("Then explore George Inness's materials and paint handling in the studio\nnotes.") == "George Inness"
    assert S._subject_of("Then explore George Inness’s materials and paint handling.") == "Inness"


@pytest.mark.parametrize("via", ["live server", "static export"])
def test_runner_discovery_includes_non_gallery_runs_and_digit_ending_lanes(home, server, monkeypatch, via):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + say("**After Harvest, Storm Coming**"))
    run = tmp_path / "src/a/stillwet-runs/oil-yolo-fixture/run"
    rd = run / "OYO1"
    rd.mkdir(parents=True)
    (run / "studios.json").write_text(json.dumps({"OYO11": PAINTER}))
    (rd / "p1.finished").write_text("render exported")
    (rd / "p1_outcome.json").write_text(json.dumps({"status": "finished", "reason": "reviewed and stopped"}))
    monkeypatch.setattr(S, "RUNS", str(tmp_path / "src/a/stillwet-runs/**/run/studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0.0, "map": {}})
    if via == "live server":
        entries = json.loads(server("/api/sessions")[1])
    else:
        out = tmp_path / "out"
        assert export(tmp_path, out).returncode == 0
        entries = json.loads((out / "data/sessions.json").read_text())
    p = next(s for s in entries if s.get("p") == PAINTER)
    assert (p["lane"], p["n"], p["outcome"]) == ("OYO1", 1, {"status": "finished", "reason": "reviewed and stopped"})
    if via == "static export":
        assert p["completed_at"] == os.path.getmtime(rd / "p1_outcome.json")
    import export_static as E
    (rd / "p1_sittings.json").write_text('[{"status":"running"}]')
    E.runner_states([p], {PAINTER}, p["mtime"])
    assert not p["active"]
    p["outcome"] = None
    E.runner_states([p], {PAINTER}, p["mtime"])
    assert p["active"]


@pytest.mark.parametrize("via", ["live server", "static export"])
def test_finished_painter_title_falls_back_to_the_runners_final_reply(home, server, monkeypatch, via):
    tmp_path, studio, log = home
    closing = "# *Winter Oak by the Bodden at Evening* (*Wintereiche am Bodden*)\n\nA solitary oak on a promontory."
    log.write_text(start(str(studio)) + say(closing) + say("Of course, I'd be delighted to. Ask away."))
    rd = tmp_path / "src/a/stillwet-runs/agy-fixture/run/ARGON"
    rd.mkdir(parents=True)
    (rd.parent / "studios.json").write_text(json.dumps({"ARGON1": PAINTER}))
    (rd / "p1_final.txt").write_text(closing)
    monkeypatch.setattr(S, "RUNS", str(tmp_path / "src/a/stillwet-runs/**/run/studios.json"))

    def title():
        S._lanes.update(at=0, map={})
        if via == "live server":
            entries = json.loads(server("/api/sessions")[1])
        else:
            out = tmp_path / "out"
            assert export(tmp_path, out).returncode == 0
            entries = json.loads((out / "data/sessions.json").read_text())
        return next(s for s in entries if s.get("p") == PAINTER)["title"]

    assert title() is None  # no runner outcome: the final reply isn't read
    (rd / "p1_outcome.json").write_text(json.dumps({"status": "finished", "reason": "the painter is done"}))
    assert title() == "Winter Oak by the Bodden at Evening"
    log.write_text(start(str(studio)) + say(closing) + say("**The Blue Barn**\n\nA barn."))
    assert title() == "The Blue Barn"  # a title in the session's own closing words wins


def test_sittings_exclude_startup_only_crashes_but_keep_painting_and_final_review(home, server):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    crash = log.with_name("2026-09-29T11-00-00-000Z_crash.jsonl")
    crash.write_text(start(str(studio)) + look("l1", png_of("blue")) +
                     line({"type": "message", "message": {"role": "assistant", "content": [], "stopReason": "error"}}))
    review = log.with_name("2026-09-29T12-00-00-000Z_review.jsonl")
    review.write_text(start(str(studio)) + look("l2", png_of("blue")) + say("**The Blue Barn**\n\nI've reviewed it and stopped."))
    p = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
    assert p["sittings"] == 2
    assert json.loads(server(f"/api/events?p={PAINTER}")[1])["sittings"] == 2
    out = tmp_path / "out"
    assert export(tmp_path, out).returncode == 0
    assert json.loads((out / "data" / PAINTER / "events.json").read_text())["sittings"] == 2
    crash.write_text(crash.read_text() + call("c2", "wait(10)"))
    p = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
    assert p["sittings"] == 3


@pytest.mark.parametrize("retry", [False, True])
def test_die_roll_stays_in_journal_without_counting_as_a_sitting(home, server, retry):
    tmp_path, studio, log = home
    prompt = ("Your brief is in BRIEF.md. notes/concepts.md is missing or didn't parse. " if retry else
              "Your brief is in BRIEF.md. This sitting is only for writing notes/concepts.md. ")
    prompt += "Use write_concepts to save the whole file at once. Each save replaces it."
    content = prompt if retry else [{"type": "text", "text": prompt}]
    log.write_text(start(str(studio)) + line({"type": "message", "message": {"role": "user", "content": content}}) +
                   line({"type": "message", "message": {"role": "assistant", "content": [
                       {"type": "toolCall", "id": "brief", "name": "read", "arguments": {"path": "BRIEF.md"}}]}}) +
                   result("brief", "Choose eight pictures.") + say("I'll estimate eight pictures."))
    before = json.loads(server(f"/api/events?p={PAINTER}")[1])
    assert before["sittings"] == 0
    assert not any(e["kind"] == "sitting" for e in before["events"])
    with log.open("a") as fh:
        fh.write(line({"type": "message", "message": {"role": "assistant", "content": [
            {"type": "toolCall", "id": "concepts", "name": "write_concepts", "arguments": {
                "text": "\n\n".join(f"{n}. Picture {n}.\nEstimate: {n * 10}/100" for n in range(1, 9))}}]}}) +
                 result("concepts", "Saved notes/concepts.md.") + say("Saved the eight pictures and their estimates."))
    sitting = log.with_name("2026-09-29T10-10-00-000Z_sitting.jsonl")
    sitting.write_text(start(str(studio)).replace("10:00:00", "10:10:00"))
    waiting = json.loads(server(f"/api/events?p={PAINTER}")[1])
    assert waiting["sittings"] == 0
    with sitting.open("a") as fh:
        fh.write((call("c1", "canvas{}") + result("c1", "ok") + look("l1", png_of("blue"))).replace("10:00:00", "10:12:00"))
    first = json.loads(server(f"/api/events?p={PAINTER}&since={waiting['total']}&epoch={waiting['epoch']}")[1])
    assert first["epoch"] > waiting["epoch"]
    assert first["sittings"] == 1
    assert [(e["n"], e["ts"]) for e in first["events"] if e["kind"] == "sitting"] == [(1, "2026-09-29T10:10:00.000Z")]
    assert any(e.get("name") == "write_concepts" for e in first["events"])
    assert any(e.get("text") == "Saved the eight pictures and their estimates." for e in first["events"])
    assert [e["img"] for e in first["events"] if e["kind"] == "image"] == [0]
    p = next(s for s in json.loads(server("/api/sessions")[1]) if s.get("p") == PAINTER)
    assert (p["sittings"], p["sitting_numbers"]) == (1, [0, 1])
    out = tmp_path / "out"
    assert export(tmp_path, out).returncode == 0
    exported = json.loads((out / "data" / PAINTER / "events.json").read_text())
    assert exported["sittings"] == 1
    assert [e["n"] for e in exported["events"] if e["kind"] == "sitting"] == [1]
    with sitting.open("a") as fh:
        fh.write(say("The sky is laid in.").replace("10:00:00", "10:13:00"))
    more = json.loads(server(f"/api/events?p={PAINTER}&since={first['total']}&epoch={first['epoch']}")[1])
    assert more["epoch"] == first["epoch"]
    assert [e["kind"] for e in more["events"]] == ["say"]
    second = log.with_name("2026-09-29T11-00-00-000Z_review.jsonl")
    second.write_text((start(str(studio)) + say("I've reviewed it and stopped.")).replace("10:00:00", "11:00:00"))
    final = json.loads(server(f"/api/events?p={PAINTER}")[1])
    assert final["sittings"] == 2
    assert [e["n"] for e in final["events"] if e["kind"] == "sitting"] == [1, 2]


def test_muse_wonder_full_display_name_matches_viewer_and_export():
    import export_static as E
    assert E.MODEL_NAMES["muse-wonder-full-1.0"] == "Muse Wonder Full"
    assert "'muse-wonder-full-1.0':'Muse Wonder Full'" in open(os.path.join(HERE, "index.html")).read()


def test_a_palette_look_is_not_the_painting():
    # the easel's palette look (look{palette=true}) shows the mixed piles, not the canvas
    assert S.is_whole(S.look_text({}))
    assert S.is_whole(S.look_text({"size": 800}))
    assert not S.is_whole(S.look_text({"palette": True}))
    assert S.is_whole(S.look_text({"palette": False}))
    # nor is a survey's tile, a comparison or a lit look
    assert not S.is_whole(S.look_text({"survey": True}))
    assert S.is_whole(S.look_text({"survey": False}))
    assert not S.is_whole(S.look_text({"compare": "out/easel/painting/a.png"}))
    assert not S.is_whole(S.look_text({"light": "45,15"}))
    assert not S.is_whole(S.look_text({"hold": "skyP", "at": "400,320"}))
    assert not S.is_whole(S.look_text({"survey": True, "mode": "gallery"}))
    # nor the motif pinned beside the canvas (look{ref=}), even with a size
    assert not S.is_whole(S.look_text({"ref": "motif.jpg"}))
    assert not S.is_whole(S.look_text({"size": 800, "ref": "motif.jpg"}))
    assert not S.is_whole(S.look_text({"mode": "normal", "ref": "motif.jpg"}))


@pytest.mark.parametrize("ending, said", [
    (say("End of this sitting: the sky is laid in."), True),  # between sittings (or done): the website lists it a while
    (line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": "p1", "name": "paint", "arguments": {"code": "x = 1"}}]}}), False),  # cut off mid-step
])
def test_the_export_says_whether_a_run_stopped_on_its_own_words(home, ending, said):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + say("I'll start by reading the brief.") + ending)
    out = tmp_path / "out"
    r = export(tmp_path, out)
    assert r.returncode == 0, r.stderr
    p = next(s for s in json.loads((out / "data" / "sessions.json").read_text()) if s["p"] == PAINTER)
    assert p["said"] is said


@pytest.mark.parametrize("via", ["live server", "static export"])
def test_viewer_preserves_readable_names_and_paint_colors_from_physical_palette_board(home, server, via):
    # Captured Fable palette: ten labeled piles, including a third partial row.
    # The old chart decoder dropped all metadata for this 1000px board.
    pytest.importorskip("PIL.Image")
    tmp_path, studio, log = home
    with open(os.path.join(HERE, "fixtures", "fable-palette-board.png"), "rb") as f:
        png = f.read()
    log.write_text(start(str(studio)) + looks_at_once(("palette", {"palette": True}, png)))
    if via == "static export":
        out = tmp_path / "out"
        r = export(tmp_path, out)
        assert r.returncode == 0, r.stderr
        events = json.loads((out / "data" / PAINTER / "events.json").read_text())["events"]
    else:
        status, data = server(f"/api/events?p={PAINTER}")
        assert status == 200
        events = json.loads(data)["events"]
    piles = next(e for e in events if e["kind"] == "image")["palette"]
    assert [p["name"] for p in piles] == [
        "PILE 1", "WATER1", "WILLOW1", "SKY1", "WATER2", "WILLOW2", "SKY2", "DEEP", "MIDW", "LILAC"]
    # Swatches show the colored paint, rather than the almost-black opacity cards.
    assert all(max(int(p[k][i:i + 2], 16) for i in (1, 3, 5)) > 80
               for p in piles for k in ("thick", "thin"))



def test_notebook_calls_are_events_like_journal_notes(home, server):
    _, studio, log = home
    log.write_text(start(str(studio)) + line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": "n1", "name": "notebook", "arguments": {"text": "Glazes want a dry ground.", "replaces": "x"}},
        {"type": "toolCall", "id": "j1", "name": "note", "arguments": {"text": "Sky first."}}]}}))
    status, body = server(f"/api/events?p={PAINTER}&since=0")
    assert status == 200
    kinds = [(e["kind"], e.get("text")) for e in json.loads(body)["events"] if e["kind"] in ("notebook", "jnote")]
    assert kinds == [("notebook", "Glazes want a dry ground."), ("jnote", "Sky first.")]
    with open(os.path.join(HERE, "index.html")) as fh:
        assert "if (e.kind === 'notebook') return ['notes', '📓', 'writes in the notebook'];" in fh.read()


def test_the_event_slider_is_named_and_described_by_its_position():
    html = open(os.path.join(HERE, "index.html")).read()
    scrub = re.search(r'<input type="range" id="scrub"[^>]*>', html).group(0)
    assert re.search(r'aria-label="[^"]+"', scrub)
    assert 'aria-describedby="pos"' in scrub and 'id="pos"' in html


def test_ask_critic_shows_the_note_and_the_whole_reply(home, server):
    _, studio, log = home
    reply = "The horizon sits too high. " * 100
    log.write_text(start(str(studio)) + line({"type": "message", "message": {"role": "assistant", "content": [
        {"type": "toolCall", "id": "k1", "name": "ask_critic", "arguments": {"note": "Is the mist too heavy?"}}]}})
        + line({"type": "message", "message": {"role": "toolResult", "toolCallId": "k1", "isError": False,
                                                "content": [{"type": "text", "text": reply}]}}))
    status, body = server(f"/api/events?p={PAINTER}&since=0")
    assert status == 200
    asked = [e for e in json.loads(body)["events"] if e["kind"] == "critic"]
    assert [(e["text"], e["out"]) for e in asked] == [("Is the mist too heavy?", reply)]


def atelier_work(root, sid, n, folder, manifest=None, card=None):
    """A work in the atelier fixture: its run record, manifest and, once it hangs, its card."""
    work = root / "works" / f"{sid}-{n:03d}"
    (work / "run").mkdir(parents=True)
    (work / "run" / "studios.json").write_text(json.dumps({"LANE1": folder}))
    if manifest is not None:
        (work / "manifest.json").write_text(json.dumps(dict(manifest, folder=str(root.parent.parent / folder))))
    if card is not None:
        (root / "studios" / sid / "cards").mkdir(parents=True, exist_ok=True)
        (root / "studios" / sid / "cards" / f"{n:03d}.json").write_text(json.dumps(dict(card, number=n, folder=folder)))
    return work


@pytest.mark.parametrize("which", ["only", "skip"])
def test_the_studio_export_selects_atelier_paintings_and_keeps_atelier_scripts_out_of_the_page(home, which):
    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    other = log.parent.with_name("--Users-x-src-a-paint-studio-def456--") / log.name
    other.parent.mkdir()
    other.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1"))
    atelier_work(tmp_path / "src" / "a" / "stillwet-runs" / "atelier", "claude-opus-5-5", 1, PAINTER)
    out = tmp_path / "out"
    r = export(tmp_path, out, f"--{which}-atelier")
    assert r.returncode == 0, r.stderr
    painters = [s["p"] for s in json.loads((out / "data" / "sessions.json").read_text())]
    assert painters == ([PAINTER] if which == "only" else ["paint-studio-def456"])
    page = (out / "index.html").read_text()
    if which == "skip":
        assert '<base href="/studio/">' in page and "<script>" in page
        return
    assert '<base href="/atelier/easel/">' in page
    assert metadata(out / "share" / (PAINTER + ".html"))["og:url"] == f"https://stillwet.art/atelier/easel/?p={PAINTER}"
    for name in (out / "index.html", out / "share" / (PAINTER + ".html")):
        page = name.read_text()
        assert "<script>" not in page  # the atelier's policy allows no inline script
        scripts = re.findall(r'<script(?: async)? src="([^"]+)"></script>', page)
        assert "/v/app.js" in scripts and "code-display.js" in scripts
        code = [(out / s).read_text() for s in scripts if s.startswith("js/")]
        assert any('plausible.init({endpoint:"/v/e"})' in c for c in code)
        assert any("window.STUDIO_STATIC = true;" in c for c in code)
        assert any("window.STUDIO_FEATURED=" in c for c in code)
        assert any("const $ =" in c for c in code)  # the viewer itself


def test_the_atelier_export_writes_atelier_json_and_web_images(home, monkeypatch):
    import atelier_export as A
    import export_static as E
    from PIL import Image
    import io

    def png(w, h, color="red"):
        buf = io.BytesIO()
        Image.new("RGB", (w, h), color).save(buf, "PNG")
        return buf.getvalue()

    tmp_path, studio, log = home
    log.write_text(start(str(studio)) + call("c1", "canvas{}") + result("c1", "ok · chunk 1") + look("l1", png(300, 200)))
    root = tmp_path / "src" / "a" / "stillwet-runs" / "atelier"
    root.mkdir(parents=True)
    (root / "studios.json").write_text(json.dumps({
        "claude-opus-5-5": {"model": "claude-opus-5-5", "name": "Claude Opus 5.5", "harness": "pi", "since": "2026-10-20",
                            "slug": "claude-opus-5-5", "lane": "OPUS"}}))
    first = atelier_work(root, "claude-opus-5-5", 1, "paint-studio-aaa111",
                         {"theme": None, "started": "2026-10-20T19:00:00Z", "sittings": 13, "canvas": [2400, 1500]},
                         {"title": "Boot Hill, Last Light", "words": f"Painted in {tmp_path}/src/a/paint-studio-aaa111.",
                          "finished": "2026-10-20T23:00:00Z"})
    (first / "final").mkdir()
    for light in ("gallery", "color", "relief"):
        (first / "final" / f"{light}.png").write_bytes(png(2400, 1500))
    (first / "journal.md").write_text("Sky first.\n")
    easel = atelier_work(root, "claude-opus-5-5", 2, PAINTER, {"theme": "a harbor at night", "started": "2026-10-21T19:00:00Z"})
    (easel / "run" / "LANE").mkdir()
    (easel / "run" / "LANE" / "p1_sittings.json").write_text('[{"status": "running"}]')
    notebook = root / "studios" / "claude-opus-5-5" / "notebook"
    notebook.mkdir(parents=True)
    (notebook / "001-before.md").write_text("Old line.\n")
    (notebook / "001-after.md").write_text("Old line.\nNew line.\n")
    (notebook.parent / "notebook.md").write_text("Old line.\nNew line.\n")

    monkeypatch.setattr(S, "SESSIONS", str(tmp_path / ".pi" / "agent" / "sessions"))
    monkeypatch.setattr(S, "RUNS", str(tmp_path / "src/a/stillwet-runs/**/run/studios.json"))
    monkeypatch.setattr(S, "HOME", str(tmp_path))
    monkeypatch.setattr(S, "USER", tmp_path.name)
    monkeypatch.setattr(E, "ATELIER", str(root))
    monkeypatch.setattr(E, "working_studios", lambda: {PAINTER})
    S._lanes.update(at=0, map={})
    out = tmp_path / "site" / "atelier" / "data"
    A.export(str(out))

    data = json.loads((out / "atelier.json").read_text())
    assert re.fullmatch(r"\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ", data.pop("updated"))
    opus, = data["studios"]
    assert (opus["slug"], opus["name"], opus["since"]) == ("claude-opus-5-5", "Claude Opus 5.5", "2026-10-20")
    assert opus["easel"] == {"n": 2, "folder": PAINTER, "state": "live", "title": None, "theme": "a harbor at night",
                             "started": "2026-10-21T19:00:00Z", "last": "2026-09-29T10:00:00.000Z",
                             "look": "data/claude-opus-5-5/2/look.jpg"}
    assert opus["walls"] == [{
        "n": 1, "title": "Boot Hill, Last Light", "words": "Painted in ~/src/a/paint-studio-aaa111.",
        "folder": "paint-studio-aaa111", "finished": "2026-10-20T23:00:00Z", "theme": None, "sittings": 13,
        "img": {light: f"data/claude-opus-5-5/1/{light}.jpg" for light in ("gallery", "color", "relief")},
        "canvas": [2400, 1500], "journal": "data/claude-opus-5-5/1/journal.md"}]
    assert opus["notebook"] == {"text": "Old line.\nNew line.\n", "after": 1, "versions": [
        {"after": 1, "added": "New line.", "file": "data/claude-opus-5-5/notebook/1.md"}]}
    site = out.parent
    for rel in [opus["easel"]["look"], opus["walls"][0]["journal"], opus["notebook"]["versions"][0]["file"],
                *opus["walls"][0]["img"].values()]:
        assert (site / rel).is_file(), rel
    assert Image.open(site / opus["walls"][0]["img"]["color"]).size == (1600, 1000)
    assert (site / opus["notebook"]["versions"][0]["file"]).read_text() == "Old line.\nNew line.\n"
    for f in out.rglob("*"):
        if f.is_file():
            body = f.read_bytes()
            assert str(tmp_path).encode() not in body and tmp_path.name.lower().encode() not in body.lower()
            assert b"plausible" not in body.lower()

    # the painter stops: the easel is paused; the painting hangs: its look goes, the easel is empty
    monkeypatch.setattr(E, "working_studios", lambda: set())
    A.export(str(out))
    assert json.loads((out / "atelier.json").read_text())["studios"][0]["easel"]["state"] == "paused"
    (root / "studios" / "claude-opus-5-5" / "cards" / "002.json").write_text(json.dumps(
        {"number": 2, "title": None, "words": "Done.", "folder": PAINTER, "finished": "2026-10-22T01:00:00Z"}))
    A.export(str(out))
    opus = json.loads((out / "atelier.json").read_text())["studios"][0]
    assert opus["easel"] == {"n": None, "folder": None, "state": "empty", "title": None, "theme": None,
                             "started": None, "last": None, "look": None}
    assert [w["n"] for w in opus["walls"]] == [1, 2] and opus["walls"][1]["img"] == {}
    assert not (site / "data/claude-opus-5-5/2/look.jpg").exists()


def test_featured_is_the_live_painting_else_the_newest_whatever_its_outcome():
    import export_static as E
    old = {"p": "old", "mtime": 100, "active": False, "outcome": {"status": "finished"}, "completed_at": 100}
    paused = {"p": "paused", "mtime": 300, "active": False, "outcome": None, "completed_at": None}
    capped = {"p": "capped", "mtime": 200, "active": False, "outcome": {"status": "cap_reached"}, "completed_at": None}
    live = {"p": "live", "mtime": 50, "active": True, "outcome": None, "completed_at": None}
    assert E.featured([old, paused, capped])["p"] == "paused"
    assert E.featured([old, paused, capped, live])["p"] == "live"
    assert E.featured([]) is None
