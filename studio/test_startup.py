"""Returned static HTML and the browser's startup/picker lifecycle.

Run with uv run --with pytest --with pillow --with playwright pytest studio.
Without Playwright, these browser-only checks are skipped.
The HTTP fixture models the documented ?p= -> share HTML route, without a deploy.
"""
import functools
from datetime import datetime, timezone
import http.server
import json
import threading
import urllib.parse

import pytest
sync_playwright = pytest.importorskip("playwright.sync_api").sync_playwright

import export_static as E
import studio as S
from test_studio import PAINTER, home, line, look, png_of, say, start

OTHER = "paint-studio-def456"
TITLE = 'Barn & "Sky" <west>'


@pytest.fixture
def exported(home, monkeypatch, request):
    root, studio, log = home
    mode = getattr(request, "param", "paintings")
    other_log = log.parent.with_name(f"--Users-x-src-a-{OTHER}--") / log.name
    other_log.parent.mkdir()
    if mode != "empty":
        title = 'A </script><script>window.titleInjected=true</script> PrivateUser' if mode == "hostile" else TITLE
        log.write_text(start(str(studio)) + line({"type": "model_change", "modelId": "gpt-6.1-sol"})
                       + ("" if mode == "imageless" else look("l1", png_of("red"))) + say(f"**{title}**\n\nA barn."))
    if mode == "paintings":
        other_log.write_text(start(str(studio)) + line({"type": "model_change", "modelId": "claude-opus-5-5"})
                             + look("l1", png_of("blue")) + say("**Blue Pond**\n\nA pond."))
    if mode == "hostile":
        monkeypatch.setattr(S, "HOME", str(root))
        monkeypatch.setattr(S, "USER", "privateuser")
    run = root / "tmp" / "gallery-fixture" / "r24" / "run"
    for lane in ("A", "B"):
        (run / lane).mkdir(parents=True)
        (run / lane / "p1_outcome.json").write_text('{"status":"finished","reason":"done"}')
    (run / "studios.json").write_text(json.dumps({"A1": PAINTER, "B1": OTHER}))
    monkeypatch.setattr(S, "SESSIONS", str(root / ".pi" / "agent" / "sessions"))
    monkeypatch.setattr(S, "RUNS", str(run / "studios.json"))
    monkeypatch.setattr(S, "_lanes", {"at": 0, "map": {}})
    monkeypatch.setattr(E, "working_studios", set)
    # Ensure the featured painter is deterministic, using actual runner completion times.
    import os
    os.utime(run / "A" / "p1_outcome.json", (2000000000, 2000000000))
    out = root / "out"
    monkeypatch.setattr("sys.argv", ["export_static.py", str(out)])
    E.WRITTEN.clear()
    E.main()
    return out


@pytest.fixture
def site(exported):
    requests = []
    responses = []

    class Handler(http.server.SimpleHTTPRequestHandler):
        def do_GET(self):
            requests.append((self.path, dict(self.headers)))
            url = urllib.parse.urlsplit(self.path)
            path = url.path
            if path in ("/studio/", "/studio/index.html"):
                p = urllib.parse.parse_qs(url.query).get("p", [None])[0]
                name = exported / "share" / f"{p}.html" if p else exported / "index.html"
                if name.is_file():
                    self.path = "/" + str(name.relative_to(exported))
            elif path.startswith("/studio/"):
                self.path = self.path[len("/studio"):]
            super().do_GET()

        def log_message(self, *args):
            pass

        def log_request(self, code="-", size="-"):
            responses.append((self.path, code))

    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Handler, directory=str(exported)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    yield f"http://127.0.0.1:{server.server_port}", requests, responses
    server.shutdown()
    server.server_close()


@pytest.fixture
def browser():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        yield browser
        browser.close()


@pytest.mark.parametrize("path,painter,title,model,width,height", [
    ("/studio/", PAINTER, TITLE, "GPT-6.1 Sol", 1280, 720),
    (f"/studio/?p={OTHER}", OTHER, "Blue Pond", "Claude Opus 5.5", 390, 844),
    (f"/studio/share/{PAINTER}.html", PAINTER, TITLE, "GPT-6.1 Sol", 1280, 720),
    (f"/studio/?p={PAINTER}&stream=1", PAINTER, TITLE, "GPT-6.1 Sol", 1920, 1080),
    (f"/studio/?p={PAINTER}&stream=1", PAINTER, TITLE, "GPT-6.1 Sol", 390, 844),
])
def test_returned_canvas_and_header_survive_delayed_metadata_then_become_interactive(browser, site, path, painter, title, model, width, height):
    base, requests, _ = site
    plain = browser.new_context(java_script_enabled=False, viewport={"width": width, "height": height})
    static_page = plain.new_page()
    static_page.goto(base + path)
    assert static_page.locator("#who .t").inner_text() == title
    assert static_page.locator("#who .m").inner_text() == model
    assert static_page.locator("#badge").inner_text() == "Finished"
    assert static_page.locator("#clockbox").is_hidden()
    assert static_page.locator("#img").evaluate("e => e.naturalWidth > 0")
    assert static_page.locator("#img").get_attribute("src") == f"/studio/data/{painter}/v/0.jpg"
    plain.close()
    context = browser.new_context(viewport={"width": width, "height": height})
    context.set_default_timeout(5000)
    page = context.new_page()
    pending = {}
    canvas_requests = []
    page.on("request", lambda r: canvas_requests.append(r.url)
            if urllib.parse.urlsplit(r.url).path == f"/studio/data/{painter}/v/0.jpg" else None)
    page.add_init_script("""window.canvasChanges = [];
      new MutationObserver(ms => { for (const m of ms) if (m.target.id === 'img')
        canvasChanges.push(m.target.getAttribute('src')); }).observe(document, {subtree:true, attributes:true, attributeFilter:['src']});""")
    page.route("**/paintings.json", lambda route: pending.update(gallery=route))
    page.route("**/data/sessions.json", lambda route: pending.update(sessions=route))
    page.goto(base + path, wait_until="domcontentloaded")
    page.wait_for_function("document.querySelector('#img').naturalWidth > 0")
    assert page.locator("#who .t").inner_text() == title
    assert page.locator("#who .m").inner_text() == model
    # Neither sessions nor gallery has returned; the HTML itself owns this first view.
    assert "sessions" in pending and "gallery" in pending
    page.wait_for_function("document.querySelector('#strip img') !== null")
    assert all(src and "/t/" not in src for src in page.evaluate("canvasChanges"))
    assert len(canvas_requests) == 1  # initial HTML and the viewer share one canvas request
    assert any(url.startswith(f"/studio/data/{painter}/events.json") for url, _ in requests)
    rect = page.locator("#img").bounding_box()
    assert rect and rect["width"] > 0 and rect["height"] > 0
    assert rect["x"] >= 0 and rect["x"] + rect["width"] <= width
    page.keyboard.press("Space")  # stream mode hides the transport but retains its keyboard controls
    assert page.locator("#badge").inner_text() == "REPLAY"
    pending["sessions"].continue_()
    page.wait_for_function("document.querySelector('#pklist button') !== null")
    assert page.locator("#who .t").inner_text() == title
    pending["gallery"].fulfill(json=[{"studio": painter, "title": "Gallery title", "url": "/painting/test/"}])
    page.wait_for_function("document.querySelector('#who .t').textContent === 'Gallery title'")
    assert page.locator("#finished").get_attribute("href") == "/painting/test/"
    context.close()


def test_unchanged_picker_survives_revalidation_and_time_changes(browser, site):
    base, _, _ = site
    page = browser.new_page(timezone_id="UTC")
    page.set_default_timeout(5000)
    page.add_init_script("""window.initialPickerReplacements = 0;
      new MutationObserver(ms => { initialPickerReplacements += ms.filter(m => m.target.id === 'pklist').length; })
        .observe(document, {subtree:true, childList:true});""")
    now = 2000000000
    sessions = [dict(p=PAINTER, painter=True, title="Selected", model="gpt-6.1-sol", mtime=now,
                     active=True, look=0, said=False),
                dict(p=OTHER, painter=True, title=None, model="claude-opus-5-5", mtime=now,
                     active=False, look=0, said=True)]
    page.clock.install(time=datetime.fromtimestamp(now, timezone.utc))
    page.route("**/paintings.json", lambda route: route.fulfill(json=[]))
    page.route("**/data/sessions.json", lambda route: route.fulfill(json=sessions))
    page.goto(base + f"/studio/?p={PAINTER}", wait_until="domcontentloaded")
    page.wait_for_function("document.querySelectorAll('#pklist .pk').length === 2")
    assert page.evaluate("initialPickerReplacements") == 1
    assert "May 18" in page.locator("#pklist .pks").first.inner_text()
    page.evaluate("""() => {
      window.firstCard = document.querySelector('#pklist .pk');
      window.pickerReplacements = 0;
      new MutationObserver(ms => { pickerReplacements += ms.filter(m => m.target.id === 'pklist').length; })
        .observe(document.querySelector('#pklist'), {childList:true});
    }""")
    with page.expect_response(lambda r: r.url.endswith("/data/sessions.json")):
        page.clock.run_for(30001)
    assert page.evaluate("firstCard === document.querySelector('#pklist .pk')")
    assert page.evaluate("pickerReplacements") == 0
    page.locator("#who").click()
    focused = page.locator("#pklist .pk.on")
    assert focused.evaluate("e => e === document.activeElement")
    with page.expect_response(lambda r: r.url.endswith("/data/sessions.json")):
        page.clock.run_for(30001)
    assert focused.evaluate("e => e === document.activeElement")
    page.keyboard.press("Escape")
    page.clock.fast_forward(1800000)
    page.wait_for_function("!document.querySelector('#pklist').textContent.includes('painting now')")
    # A quiet, untitled sitting stops being listed at six hours even with identical JSON.
    page.clock.fast_forward(6 * 3600000)
    page.wait_for_function("document.querySelectorAll('#pklist .pk').length === 1")
    assert page.locator("#pklist .pk").count() == 1
    assert "May 18" in page.locator("#pklist .pks").inner_text()
    page.close()


def test_sessions_revalidation_reuses_the_unchanged_http_body(browser, site):
    base, requests, responses = site
    page = browser.new_page()
    page.clock.install()
    page.goto(base + f"/studio/?p={PAINTER}")
    with page.expect_response(lambda r: r.url.endswith("/data/sessions.json")):
        page.clock.fast_forward(30001)
    session_headers = [h for path, h in requests if path == "/studio/data/sessions.json"]
    assert len(session_headers) == 2
    assert "If-Modified-Since" in session_headers[1]
    assert ("/data/sessions.json", 304) in responses
    page.close()


def test_legacy_sitting_and_conflicting_share_query_open_the_requested_painter(browser, site):
    base, _, _ = site
    page = browser.new_page()
    page.set_default_timeout(5000)
    page.route("**/paintings.json", lambda route: route.fulfill(json=[]))
    pending = {}
    page.route("**/code-display.js", lambda route: pending.update(code=route))
    page.route("**/data/sessions.json", lambda route: pending.update(sessions=route))
    old = f"--Users-x-src-a-{OTHER}--/2026-09-29T10-00-00-000Z_s1.jsonl"
    for target in ("/studio/?s=" + urllib.parse.quote(old, safe=""),
                   f"/studio/share/{PAINTER}.html?p={OTHER}&stream=1"):
        pending.clear()
        page.goto(base + target, wait_until="commit")
        page.wait_for_selector("#who", state="attached")
        assert page.locator("#who").is_hidden()
        assert page.locator("#img").is_hidden()
        assert page.title() == "The studio · stillwet"
        pending["code"].continue_()
        page.wait_for_function("document.readyState !== 'loading'")
        assert page.locator("#who .t").inner_text() != TITLE
        src = page.locator("#img").get_attribute("src")
        assert src is None or OTHER in src
        pending["sessions"].continue_()
        page.wait_for_function("document.querySelector('#who .t').textContent === 'Blue Pond'")
        page.wait_for_function("document.querySelector('#img').src.includes('/paint-studio-def456/')")
    page.locator("#who").click()
    page.locator(f'#pklist button[data-q="p={PAINTER}"]').click()
    assert urllib.parse.parse_qs(urllib.parse.urlsplit(page.url).query) == {"p": [PAINTER], "stream": ["1"]}
    page.close()


@pytest.mark.parametrize("exported", ["empty", "imageless", "hostile"], indirect=True)
def test_returned_empty_states_and_hostile_titles_are_safe(browser, site, exported):
    base, _, _ = site
    source = (exported / "index.html").read_bytes()
    context = browser.new_context(java_script_enabled=False)
    page = context.new_page()
    page.goto(base + "/studio/")
    sessions = json.loads((exported / "data" / "sessions.json").read_text())
    if not sessions:
        assert page.locator("#who .t").inner_text() == "Choose a painter"
        assert page.locator("#img").get_attribute("src") is None
        assert page.locator("#badge").inner_text() == ""
    elif sessions[0]["look"] is None:
        assert page.locator("#who .t").inner_text() == TITLE
        assert page.locator("#img").get_attribute("src") is None
        assert page.locator("#nopic").is_visible()
        assert "No picture yet" in page.locator("#nopic").inner_text()
    else:
        expected = 'A </script><script>window.titleInjected=true</script> user'
        assert page.locator("#who .t").inner_text() == expected
        assert b"privateuser" not in source.lower()
        assert str(exported.parent).encode() not in source
        context.close()
        context = browser.new_context()
        page = context.new_page()
        page.route("**/paintings.json", lambda route: route.fulfill(json=[]))
        page.goto(base + "/studio/")
        assert page.locator("#who .t").inner_text() == expected
        assert page.evaluate("typeof window.titleInjected") == "undefined"
    context.close()
