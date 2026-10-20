# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0"]
# ///

import ast
import base64
import json
import os
from pathlib import Path
import re
import shutil
import sys
import tempfile
import unittest
from functools import wraps
from importlib.machinery import SourceFileLoader
from importlib.util import module_from_spec, spec_from_loader
from unittest.mock import AsyncMock, patch
from io import StringIO

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

SERVER = Path(__file__).with_name("server.py")
# what the painter must not hear about a chunk: the machine's time, the tool's wait, the easel's line and mark counts, chunk numbers
MACHINE_WORDS = r"running for|within|\d+ minutes?\b|budget|limit|\bline \d|\d+ lines|of \d|strokes|touches|chunk \d|\(chunk|\d+ chunks?"
PNG = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/a9sAAAAASUVORK5CYII="
FAKE = '''#!{python}
import base64, json, os, sys
from pathlib import Path
root = Path(__file__).resolve().parent.parent
args = sys.argv[1:]
with (root / "calls.jsonl").open("a") as f:
    f.write(json.dumps({{"args": args, "env": dict(os.environ)}}) + "\\n")
opened = root / "opened"
command = args[0]
if command == "open":
    if (root / "open-fails").exists():
        print("easel startup failed")
        sys.exit(1)
    opened.touch()
    print("open")
elif not opened.exists():
    print("no painting open: easel open")
    sys.exit(1)
elif command == "status":
    print("2 chunks · 2400px · test canvas")
elif command == "do":
    text = sys.stdin.read()
    if text == "fail":
        print('runtime error: [string "chunk 3"]:1: nope')
        sys.exit(1)
    print("day 1, 00:00\\nok · chunk 3 (0.01 s to compute)")
elif command == "look":
    if "--mode" in args and args[args.index("--mode") + 1] == "bad":
        print("look: --mode is invalid")
        sys.exit(1)
    count = 3 if "--survey" in args else 1
    for i in range(count):
        path = root / "out/easel/painting" / ("look-%04d.png" % i)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(base64.b64decode("{png}"))
        for name in (".gallery", ".relief"):
            companion = path.parent / name / path.name
            companion.parent.mkdir(exist_ok=True)
            companion.write_bytes(path.read_bytes())
        print(str(path.relative_to(root)) + " (1x1, 0.01s)")
elif command == "probe":
    if args[-1] == "9,9":
        print("probe: --at 9,9 is outside the canvas (0,0 to 1000,800)")
        sys.exit(1)
    print("at 1,2\\ncolor 10,20,30 · value 18\\nin gallery light 11,21,31 · value 19\\ndrying: dry")
elif command == "note":
    text = sys.stdin.read()
    path = root / "notes/journal.md"
    with path.open("a") as f:
        f.write("day 1, 00:00\\n" + text + "\\n")
    print("noted")
elif command == "log":
    override = root / "log-output.txt"
    print(override.read_text() if override.exists() else "--@ chunk 1\\ncanvas{{}}\\n--@ chunk 2\\nwait(1)")
'''


def connected(test):
    @wraps(test)
    async def run(self):
        env = dict(os.environ, EASEL_ROOT=str(self.studio), EASEL_SESSION="other", EASEL_BOX="other", PAINTER_MAX_IMAGES="2", PAINTER_MAX_IMAGE_MB="12", RAYON_NUM_THREADS="2")
        params = StdioServerParameters(command=sys.executable, args=[str(SERVER)], env=env)
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as self.session:
                self.initialized = await self.session.initialize()
                await test(self)
    return run


class ProtocolTests(unittest.IsolatedAsyncioTestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="emcp-")
        self.studio = Path(self.temp.name)
        (self.studio / "bin").mkdir()
        (self.studio / "notes").mkdir()
        (self.studio / "notes/journal.md").touch()
        binary = self.studio / "bin/easel"
        binary.write_text(FAKE.format(python=sys.executable, png=PNG))
        binary.chmod(0o755)

    def tearDown(self):
        self.temp.cleanup()

    def test_installed_easel_and_its_aliases_are_not_readable(self):
        loader = SourceFileLoader("read_test_server", str(SERVER))
        module = module_from_spec(spec_from_loader(loader.name, loader))
        loader.exec_module(module)
        executable = self.studio / "bin/easel"
        (self.studio / "notes/link").symlink_to(executable)
        (self.studio / "notes/hard-link").hardlink_to(executable)
        shutil.copyfile(executable, self.studio / "notes/copy")
        (self.studio / "notes/binary").write_bytes(bytes([0, 255, 17]))
        for path in (str(executable), "bin/easel", "@bin/easel", "bin/../bin/easel", "notes/link", "notes/hard-link", executable.as_uri()):
            self.assertIsNone(module.studio_path(self.studio, path), path)
        for path in ("notes/copy", "notes/binary", "bin/box"):
            self.assertEqual(module.studio_path(self.studio, path), self.studio / path)

    async def test_painting_header_sets_chunk_and_rebuild_waits(self):
        loader = SourceFileLoader("width_test_server", str(SERVER))
        module = module_from_spec(spec_from_loader(loader.name, loader))
        loader.exec_module(module)
        easel = module.Easel(self.studio)
        log = self.studio / "paintings/lua/painting.lua"
        log.parent.mkdir(parents=True)
        width = self.studio / "bin/width"
        for configured in (None, "2400", "4800"):
            if configured is None:
                width.unlink(missing_ok=True)
            else:
                width.write_text(configured)
            for header, wide in (
                (None, configured == "4800"),
                ("--@ width 4800\n--@ chunk 1\n", True),
                ("--@ chunk 1\n--@ width 4800\n", False),
                ("--@ width 2400\n--@ chunk 1\n", False),
                ("--@ sketch\n--@ width 4800\n--@ chunk 1\n", False),
            ):
                with self.subTest(configured=configured, header=header):
                    if header is None:
                        log.unlink(missing_ok=True)
                    else:
                        log.write_text(header)
                    self.assertEqual(easel.wide(), wide)
                    step = AsyncMock(side_effect=[(0, "rebuilding from the log (0 of 1 chunks)"), (0, "ready"), (0, "ok")])
                    with patch.object(easel, "step", step), patch.object(module.time, "monotonic", return_value=100), patch.object(module.asyncio, "sleep", AsyncMock()):
                        self.assertEqual(await easel.at(["do", "-"], "print(1)"), "ok")
                    wait = (48 if wide else 12) * 60
                    self.assertEqual([call.args[2] for call in step.await_args_list], [100 + wait, 100 + (50 if wide else 30) * 60, 100 + wait])

    async def test_status_replay_open_and_chunk_timeouts_say_what_landed(self):
        loader = SourceFileLoader("landed_test_server", str(SERVER))
        module = module_from_spec(spec_from_loader(loader.name, loader))
        loader.exec_module(module)
        easel = module.Easel(self.studio)
        replay = "rebuilding from the log (2 of 5 chunks)"
        step = AsyncMock(side_effect=[(0, replay), (0, replay)])
        with patch.object(easel, "step", step):
            self.assertEqual(await easel.at(["status"]), replay)
        self.assertEqual([call.args[0] for call in step.await_args_list], [["status"], ["status"]])

        # an easel that has its socket but hasn't read the log yet is asked again, not opened
        step = AsyncMock(side_effect=[(1, module.STARTING), (0, replay), (0, replay)])
        with patch.object(easel, "step", step), patch.object(module.asyncio, "sleep", AsyncMock()):
            self.assertEqual(await easel.at(["status"]), replay)
        self.assertEqual([call.args[0] for call in step.await_args_list], [["status"], ["status"], ["status"]])

        step = AsyncMock(side_effect=[(1, "no painting open: easel open"), module.asyncio.TimeoutError(), (0, "rebuilding from the log (1 of 4 chunks)")])
        with patch.object(easel, "step", step), self.assertRaises(ValueError) as raised:
            await easel.at(["look"])
        self.assertEqual(str(raised.exception), "the easel is reopening the painting from its log and hasn't finished: rebuilding from the log (1 of 4 chunks). `status` shows how far it has got.")
        self.assertEqual([call.args[0] for call in step.await_args_list], [["status"], ["open"], ["status"]])

        # a probe or a note that times out submits no chunk: its recovery doesn't point at the log
        for args, text, said in (
            (["probe", "--at", "1,1"], None, "the easel didn't answer; `probe` changes nothing, so it can be asked again"),
            (["note", "-"], "an entry", "the easel didn't answer; the entry may have been added all the same, so read notes/journal.md before adding it again"),
        ):
            step = AsyncMock(side_effect=[(0, "3 chunks · 2400px · no canvas yet"), module.asyncio.TimeoutError()])
            with self.subTest(said=said), patch.object(easel, "step", step), self.assertRaises(ValueError) as raised:
                await easel.at(args, text)
            self.assertEqual(str(raised.exception), said)

        log = self.studio / "paintings/lua/painting.lua"
        log.parent.mkdir(parents=True)
        running = "painting a chunk (at line 3 of 9): 40 strokes and touches so far, 2 min 30 s on the painting's clock (now day 2, 14:05), running for 12 min 3 s"
        three = "--@ chunk 1\nx = 0\n\n--@ chunk 2\nx = 0\n\n--@ chunk 3\nx = 0\n"
        stopped = "`paint` stopped waiting for the easel's answer"
        ran, nothing = f"{stopped}, but the chunk ran: it is in the log.", f"{stopped}, and the chunk isn't in the log: it changed nothing."
        for status, after, said in (
            (running, None, f"{stopped}. The chunk is still running and isn't in the log yet: painting a chunk: it has taken 2 min 30 s of the painting's time so far, and the painting's clock reads day 2, 14:05. `status` shows how far it has got while it runs; once it ends, `log` shows whether it was kept."),
            ("painting a chunk: starting, running for 12 min 0 s", None, f"{stopped}. The chunk is still running and isn't in the log yet: painting a chunk: starting. `status` shows how far it has got while it runs; once it ends, `log` shows whether it was kept."),
            ("4 chunks · 2400px · no canvas yet", three + "\n--@ chunk 4\nx = 1\n", ran),
            ("the chunk is in the log (chunk 4); the easel is finishing its reply, running for 12 min 1 s", three + "\n--@ chunk 4\nx = 1\n", ran),
            ("3 chunks · 2400px · no canvas yet", None, nothing),
            # a fresh scratch canvas: a new log, its setup and then the chunk, or the setup alone
            ("2 chunks · 2400px · no canvas yet", "--@ chunk 1\ncanvas{}\n\n--@ chunk 2\nx = 1\n", ran),
            ("1 chunks · 2400px · no canvas yet", "--@ chunk 1\ncanvas{}\n", nothing),
        ):
            log.write_text(three)
            replies = [(0, "3 chunks · 2400px · no canvas yet"), module.asyncio.TimeoutError(), (0, status)]

            async def step(args, text, deadline, replies=replies, after=after):
                if args[0] == "do" and after is not None:
                    log.write_text(after)
                reply = replies.pop(0)
                if isinstance(reply, Exception):
                    raise reply
                return reply
            with self.subTest(said=said), patch.object(easel, "step", step), self.assertRaises(ValueError) as raised:
                await easel.at(["do", "-"], "x = 1")
            self.assertEqual(str(raised.exception), said)
            self.assertNotRegex(str(raised.exception), MACHINE_WORDS)

        # a chunk sent with CRLF line ends is kept with them: the log is read as written
        log.write_text(three)
        replies = [(0, "3 chunks · 2400px · no canvas yet"), module.asyncio.TimeoutError(), (0, "4 chunks · 2400px · no canvas yet")]

        async def step(args, text, deadline):
            if args[0] == "do":
                log.write_bytes((three + "\n--@ chunk 4\n").encode() + text.rstrip().encode() + b"\n")
            reply = replies.pop(0)
            if isinstance(reply, Exception):
                raise reply
            return reply
        with patch.object(easel, "step", step), self.assertRaises(ValueError) as raised:
            await easel.at(["do", "-"], "x = 1\r\ny = 2\r\n")
        self.assertEqual(str(raised.exception), ran)

    def test_progress_tells_what_is_happening_and_the_paintings_time(self):
        loader = SourceFileLoader("progress_test_server", str(SERVER))
        module = module_from_spec(spec_from_loader(loader.name, loader))
        loader.exec_module(module)
        for easel, painter in (
            ("painting a chunk (at line 8 of 20): 100 strokes and touches so far, 5 s on the painting's clock (now day 1, 09:00), running for 12 min 3 s",
             "painting a chunk: it has taken 5 s of the painting's time so far, and the painting's clock reads day 1, 09:00"),
            ("painting a chunk (20 lines): 0 strokes and touches so far, 14 min 20 s on the painting's clock (now day 3, 18:40), running for 40 s",
             "painting a chunk: it has taken 14 min 20 s of the painting's time so far, and the painting's clock reads day 3, 18:40"),
            ("painting a chunk: starting, running for 2 s", "painting a chunk: starting"),
            ("the chunk is in the log (chunk 2); the easel is finishing its reply, running for 1 min 15 s", "the chunk is in the log; the easel is finishing its reply"),
            # an easel line worded otherwise still tells only that a chunk is painting
            ("painting a chunk (line 3/9): 40 marks, 12 min elapsed", "painting a chunk"),
            ("canvas{size=900}", "canvas{size=900}"),
        ):
            with self.subTest(easel=easel):
                self.assertEqual(module.progress_words(easel), painter)
                self.assertNotRegex(module.progress_words(easel), MACHINE_WORDS)

    async def test_background_display_paths_are_not_renamed(self):
        loader = SourceFileLoader("display_test_server", str(SERVER))
        module = module_from_spec(spec_from_loader(loader.name, loader))
        loader.exec_module(module)
        easel = module.Easel(self.studio)
        with patch.dict(os.environ, EASEL_ASYNC_DISPLAY="1", EASEL_DISPLAY_THREADS="2", PAINTER_SECRET="s"):
            await easel.step(["open"], None, module.time.monotonic() + 5)
        environment = json.loads((self.studio / "calls.jsonl").read_text().splitlines()[0])["env"]
        self.assertEqual(environment["EASEL_ASYNC_DISPLAY"], "1")
        self.assertEqual(environment["EASEL_DISPLAY_THREADS"], "2")
        self.assertNotIn("PAINTER_SECRET", environment)
        source = self.studio / "out/65c0681641604c1b93c51bdcfd66302e.png"
        marker = source.parent / ".display" / source.name
        marker.parent.mkdir(parents=True)
        source.write_bytes(base64.b64decode(PNG))
        path = str(source.relative_to(self.studio))
        for state in ("pending", "ready", "failed"):
            marker.write_text(state + "\n")
            said, paths = easel.rename_looks(path + " (1x1, 0.01s)")
            self.assertEqual(said, path + " (1x1)")
            self.assertEqual(paths, [path])
            self.assertTrue(source.is_file())

    async def test_opt_in_recording_preserves_stdio_tools_and_results(self):
        studio = self.studio / "paint-studio-abcd"
        studio.mkdir()
        for name in ("bin", "notes"):
            (self.studio / name).rename(studio / name)
        params = StdioServerParameters(command=sys.executable, args=[str(SERVER), "--studio", str(studio), "--sessions-root", str(self.studio / "sessions")])
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                listed = await session.list_tools()
                self.assertEqual([tool.model_dump(exclude_none=True) for tool in listed.tools], json.loads(SERVER.with_name("tools.json").read_text()))
                result = await session.call_tool("paint", {"lua": "canvas{}"})
                self.assertFalse(result.isError)
                failed = await session.call_tool("paint", {"lua": "fail"})
                self.assertTrue(failed.isError)
                look = await session.call_tool("look", {})
                self.assertEqual(look.content[1].type, "image")
        events = [json.loads(line) for line in (studio / "out/easel/mcp/session.jsonl").read_text().splitlines()]
        results = [e["message"] for e in events if e.get("message", {}).get("role") == "toolResult"]
        self.assertEqual(len(results), 3)
        self.assertEqual([r["isError"] for r in results], [False, True, False])
        self.assertEqual(results[2]["content"][1]["data"], look.content[1].data)
        self.assertEqual(len(list((self.studio / "sessions").glob("*/*.jsonl"))), 1)

    @connected
    async def test_initialize_and_tool_wording(self):
        self.assertEqual(self.initialized.serverInfo.name, "easel")
        listed = await self.session.list_tools()
        self.assertEqual([tool.name for tool in listed.tools], ["paint", "look", "probe", "note", "status", "log", "reference", "reference_list"])
        source = SERVER.parent.parent.joinpath("painter/easel-tools.ts").read_text()
        for tool in listed.tools:
            block = source.split(f'name: "{tool.name}"', 1)[1].split("parameters:", 1)[0].split("description:", 1)[1]
            description = "".join(ast.literal_eval(s) for s in re.findall(r'"(?:\\.|[^"\\])*"', block))
            self.assertEqual(tool.description, description)
        paint = listed.tools[0].inputSchema
        self.assertEqual(paint["required"], ["lua"])
        self.assertEqual(set(paint["properties"]), {"lua", "scratch", "new_scratch"})

    @connected
    async def test_paint_and_bare_environment(self):
        reply = await self.session.call_tool("paint", {"lua": "print('hello')", "scratch": True, "new_scratch": True})
        self.assertFalse(reply.isError)
        self.assertEqual(reply.content[0].text, "[scratch canvas]\nday 1, 00:00\nok")
        calls = [json.loads(line) for line in (self.studio / "calls.jsonl").read_text().splitlines()]
        self.assertEqual([call["args"] for call in calls], [["status"], ["open"], ["do", "-", "--scratch", "--new"]])
        self.assertEqual(calls[-1]["env"]["RAYON_NUM_THREADS"], "2")
        self.assertNotIn("EASEL_ROOT", calls[-1]["env"])
        self.assertNotIn("EASEL_SESSION", calls[-1]["env"])
        self.assertNotIn("EASEL_BOX", calls[-1]["env"])

    @connected
    async def test_look_inline_and_companions(self):
        reply = await self.session.call_tool("look", {"mode": "gallery", "grid": 25.123456789, "size": 100.0})
        self.assertFalse(reply.isError)
        self.assertEqual(reply.content[1].type, "image")
        self.assertEqual(reply.content[1].mimeType, "image/png")
        self.assertEqual(base64.b64decode(reply.content[1].data), base64.b64decode(PNG))
        caption = reply.content[0].text
        self.assertRegex(caption, r"out/easel/painting/[0-9a-f-]+\.png \(1x1\)")
        path = self.studio / caption.split(" (", 1)[0]
        self.assertTrue(path.is_file())
        for folder in (".gallery", ".relief"):
            self.assertTrue((path.parent / folder / path.name).is_file())
        calls = [json.loads(line) for line in (self.studio / "calls.jsonl").read_text().splitlines()]
        self.assertEqual(calls[-1]["args"], ["look", "--mode", "gallery", "--size", "100", "--grid", "25.123456789"])

    @connected
    async def test_survey_budget_and_remaining_tiles(self):
        reply = await self.session.call_tool("look", {"survey": True, "mode": "gallery"})
        self.assertEqual(len(reply.content), 3)
        self.assertIn("Partial survey: showing tiles 1–2 of 3", reply.content[0].text)
        self.assertEqual(len(list((self.studio / "out/easel/painting").glob("*.png"))), 3)

    @connected
    async def test_named_masks_and_wide_crop_reach_the_easel(self):
        reply = await self.session.call_tool("look", {"crop": "0,100,1000,200", "mask": "shape,soft", "size": 300})
        self.assertFalse(reply.isError)
        self.assertEqual(reply.content[1].type, "image")
        calls = [json.loads(line) for line in (self.studio / "calls.jsonl").read_text().splitlines()]
        self.assertEqual(calls[-1]["args"], ["look", "--crop", "0,100,1000,200", "--mask", "shape,soft", "--size", "300"])

    @connected
    async def test_easel_errors_without_wrapper(self):
        reply = await self.session.call_tool("paint", {"lua": "fail"})
        self.assertTrue(reply.isError)
        self.assertEqual(reply.content[0].text, 'runtime error: [string "chunk"]:1: nope')
        reply = await self.session.call_tool("look", {"mode": "bad"})
        self.assertTrue(reply.isError)
        self.assertEqual(reply.content[0].text, "look: mode is invalid")

    @connected
    async def test_invalid_combinations_and_compare_confinement(self):
        for name, arguments, error in (
            ("paint", {"lua": "", "new_scratch": True}, "paint: new_scratch goes with scratch: true"),
            ("look", {"palette": True, "size": 0}, "look: palette takes no other option"),
            ("look", {"survey": True, "compare": "a.png"}, "look: survey and compare are two looks; ask for one"),
            ("look", {"compare": "../outside.png"}, "look: compare is the path of an image in this studio"),
            ("look", {"ref": "../outside.png"}, "look: ref is the path of a picture of the motif in this studio"),
            ("look", {"ref": "a.png", "survey": True}, "look: ref is a look of its own: no survey, compare or hold"),
        ):
            reply = await self.session.call_tool(name, arguments)
            self.assertTrue(reply.isError)
            self.assertEqual(reply.content[0].text, error)
        (self.studio / "escape.png").symlink_to(self.studio.parent / "outside.png")
        reply = await self.session.call_tool("look", {"compare": "escape.png"})
        self.assertTrue(reply.isError)
        self.assertFalse((self.studio / "calls.jsonl").exists())

    @connected
    async def test_compare_file_url_rules(self):
        for path, expected in (
            ("file://other-machine/tmp/a.png", f'File URL host must be "localhost" or empty on {sys.platform}'),
            ("file://localhost:80/tmp/a.png", "Invalid URL"),
            (f"file://localhost:{self.studio}/look.png", "Invalid URL"),
            ("file://localhost", "look: compare is the path of an image in this studio"),
            ("file:///tmp/a%2fb.png", "File URL path must not include encoded / characters"),
            ("file:///tmp/%zz.png", "URI malformed"),
            ("file:///tmp/%ff.png", "URI malformed"),
        ):
            reply = await self.session.call_tool("look", {"compare": path})
            self.assertTrue(reply.isError)
            self.assertEqual(reply.content[0].text, expected)
        self.assertFalse((self.studio / "calls.jsonl").exists())
        reply = await self.session.call_tool("look", {"compare": "~root/previous.png"})
        self.assertFalse(reply.isError)
        calls = [json.loads(line) for line in (self.studio / "calls.jsonl").read_text().splitlines()]
        self.assertEqual(calls[-1]["args"], ["look", "--compare", str(self.studio.resolve() / "~root/previous.png")])

    @connected
    async def test_log_tail_counts_utf16_units(self):
        text = "--@ chunk 1\n" + "print('😀')\n" * 4200 + "x"
        (self.studio / "log-output.txt").write_text(text)
        reply = await self.session.call_tool("log", {})
        self.assertFalse(reply.isError)
        expected = "(the first 411 characters are left out; read paintings/lua/painting.lua for all of it)\n('😀')\n" + "print('😀')\n" * 4166 + "x"
        self.assertEqual(reply.content[0].text, expected)

    @connected
    async def test_log_tail_keeps_transport_valid_at_surrogate_boundary(self):
        (self.studio / "log-output.txt").write_text("😀" * 25_000 + "x")
        reply = await self.session.call_tool("log", {})
        self.assertFalse(reply.isError)
        self.assertTrue(reply.content[0].text.startswith("(the first 1 characters are left out; read paintings/lua/painting.lua for all of it)\n�😀"))

    @connected
    async def test_note_revision_status_and_log(self):
        await self.session.call_tool("note", {"text": "a passage"})
        reply = await self.session.call_tool("note", {"text": "a new passage", "replaces": "a passage"})
        self.assertFalse(reply.isError)
        self.assertIn("a new passage", (self.studio / "notes/journal.md").read_text())
        revision = json.loads((self.studio / "out/easel/journal-revisions.jsonl").read_text())
        self.assertEqual(revision["replaced"], "a passage")
        reply = await self.session.call_tool("status", {"scratch": True})
        self.assertEqual(reply.content[0].text, "[scratch canvas]\ntest canvas")
        reply = await self.session.call_tool("log", {"scratch": True})
        self.assertEqual(reply.content[0].text, "[scratch canvas]\n--@ chunk\ncanvas{}\n--@ chunk\nwait(1)")

    @connected
    async def test_probe_point_area_and_errors(self):
        reply = await self.session.call_tool("probe", {"at": "1,2"})
        self.assertFalse(reply.isError)
        self.assertEqual(reply.content[0].text, "at 1,2\ncolor 10,20,30 · value 18\nin gallery light 11,21,31 · value 19\ndrying: dry")
        reply = await self.session.call_tool("probe", {"crop": "0,0,50,40", "scratch": True})
        self.assertTrue(reply.content[0].text.startswith("[scratch canvas]\nat 1,2\n"))
        reply = await self.session.call_tool("probe", {"at": "9,9"})
        self.assertTrue(reply.isError)
        self.assertEqual(reply.content[0].text, "probe: at 9,9 is outside the canvas (0,0 to 1000,800)")
        calls = [json.loads(line)["args"] for line in (self.studio / "calls.jsonl").read_text().splitlines()]
        self.assertEqual([call for call in calls if call[0] == "probe"], [["probe", "--at", "1,2"], ["probe", "--scratch", "--crop", "0,0,50,40"], ["probe", "--at", "9,9"]])
        for arguments in ({}, {"at": "1,2", "crop": "0,0,5,5"}, {"crop": ""}):
            reply = await self.session.call_tool("probe", arguments)
            self.assertTrue(reply.isError)
            self.assertEqual(reply.content[0].text, 'probe: give at: "x,y" or crop: "x0,y0,x1,y1", one of the two')
        self.assertEqual(len(calls), len((self.studio / "calls.jsonl").read_text().splitlines()))

    @connected
    async def test_log_find_numbers_lines_with_context(self):
        (self.studio / "log-output.txt").write_text("--@ chunk 1\ncanvas{}\n--@ chunk 2\nsky = pile{{'lead white', 4}}\na = 1\nb = 2\nw = 0\nc = 3\nd = 4\nmix{{SKY, 1}}")
        reply = await self.session.call_tool("log", {"find": "Sky"})
        self.assertFalse(reply.isError)
        self.assertEqual(reply.content[0].text, "\n".join([
            '2 lines of paintings/lua/painting.lua hold "Sky"',
            "2- canvas{}", "3- --@ chunk", "4: sky = pile{{'lead white', 4}}", "5- a = 1", "6- b = 2",
            "--", "8- c = 3", "9- d = 4", "10: mix{{SKY, 1}}",
        ]))
        reply = await self.session.call_tool("log", {"find": "canvas", "scratch": True})
        self.assertEqual(reply.content[0].text, '[scratch canvas]\n1 line of paintings/lua/scratch.lua holds "canvas"\n1- --@ chunk\n2: canvas{}\n3- --@ chunk\n4- sky = pile{{\'lead white\', 4}}')
        reply = await self.session.call_tool("log", {"find": "mask"})
        self.assertEqual(reply.content[0].text, 'no line of paintings/lua/painting.lua holds "mask"')

    @connected
    async def test_open_failure_does_not_retry(self):
        (self.studio / "open-fails").touch()
        reply = await self.session.call_tool("status", {})
        self.assertTrue(reply.isError)
        self.assertEqual(reply.content[0].text, "easel startup failed")
        calls = [json.loads(line) for line in (self.studio / "calls.jsonl").read_text().splitlines()]
        self.assertEqual([call["args"] for call in calls], [["status"], ["open"]])


class PreparationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="emcp-prep-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.repo = self.root / "repo"
        runtime = self.repo / "harness/easel_mcp"
        runtime.mkdir(parents=True)
        for name in ("server.py", "references.py", "tools.json"):
            shutil.copy2(SERVER.with_name(name), runtime / name)
        (self.repo / "scripts").mkdir()
        script = SERVER.parent.parent.parent / "scripts/prepare_capy_studio"
        loader = SourceFileLoader("prepare_capy_studio", str(script))
        self.prepare = module_from_spec(spec_from_loader(loader.name, loader))
        loader.exec_module(self.prepare)
        self.prepare.__file__ = str(self.repo / "scripts/prepare_capy_studio")
        self.dest = self.root / "studio with spaces"
        self.brief = self.root / "BRIEF.md"
        self.original = "# Paint a picture\n\nAn approved opening.\n- The folder /old/studio: this brief, your notes and the easel.\n\nnotes/easel_guide.md; notes/studio_notes.md\n"
        self.brief.write_text(self.original)
        self.notes = self.root / "studio_notes.md"
        self.notes.write_text("Approved observations.\n")
        self.args = ["prepare_capy_studio", "hopper", str(self.dest), "--brief", str(self.brief), "--studio-notes", str(self.notes)]

    def export(self, command, **options):
        self.assertEqual(command[:2], [str(self.repo / "scripts/export_r16_studio"), "hopper"])
        self.assertEqual(options["env"]["R16_BRANCH"], "HEAD")
        self.assertTrue(Path(options["env"]["TMPDIR"]).is_dir())
        (self.dest / "bin").mkdir(parents=True)
        (self.dest / "notes").mkdir()
        (self.dest / "notes/easel_guide.md").write_text("guide")

    def test_export_registration_and_brief_copy(self):
        output = StringIO()
        with patch.object(sys, "argv", self.args), patch.object(self.prepare.subprocess, "run", side_effect=self.export), patch.object(self.prepare.shutil, "which", return_value="/bin/uv"), patch.object(sys, "stdout", output):
            self.prepare.main()
        registration = json.loads(output.getvalue())
        self.assertEqual(registration["command"], "/bin/uv")
        self.assertEqual(registration["cwd"], str(self.dest))
        self.assertEqual(registration["env"], {})
        self.assertEqual(registration["args"], ["run", "--no-project", "--script", str(self.dest / "bin/mcp/server.py"), "--studio", str(self.dest)])
        self.assertEqual((self.dest / "BRIEF.md").read_text(), self.original.replace("/old/studio", str(self.dest)))
        self.assertEqual((self.dest / "notes/studio_notes.md").read_bytes(), self.notes.read_bytes())
        self.assertEqual((self.dest / "bin/mcp/server.py").read_bytes(), SERVER.read_bytes())

    def test_non_house_brief_fails_before_export(self):
        self.brief.write_text("not a house brief")
        with patch.object(sys, "argv", self.args), patch.object(self.prepare.subprocess, "run") as export, patch.object(sys, "stderr", StringIO()), self.assertRaises(SystemExit):
            self.prepare.main()
        export.assert_not_called()

    def test_optional_recording_registration(self):
        shutil.copy2(SERVER.with_name("remote.py"), self.repo / "harness/easel_mcp/remote.py")
        output = StringIO()
        sessions = self.root / "sessions"
        with patch.object(sys, "argv", self.args + ["--sessions-root", str(sessions)]), patch.object(self.prepare.subprocess, "run", side_effect=self.export), patch.object(self.prepare.shutil, "which", return_value="/bin/uv"), patch.object(sys, "stdout", output):
            self.prepare.main()
        registration = json.loads(output.getvalue())
        self.assertEqual(registration["args"][-2:], ["--sessions-root", str(sessions)])
        self.assertEqual((self.dest / "bin/mcp/remote.py").read_bytes(), SERVER.with_name("remote.py").read_bytes())

    def test_missing_reading_is_rejected(self):
        self.brief.write_text(self.original + "notes/missing.md\n")
        with patch.object(sys, "argv", self.args), patch.object(self.prepare.subprocess, "run", side_effect=self.export), patch.object(self.prepare.shutil, "which", return_value="/bin/uv"), patch.object(sys, "stderr", StringIO()), self.assertRaises(SystemExit):
            self.prepare.main()
        self.assertFalse((self.dest / "BRIEF.md").exists())


@unittest.skipUnless(os.environ.get("EASEL_MCP_STUDIO"), "set EASEL_MCP_STUDIO to an exported painter studio")
class RealEaselTests(unittest.IsolatedAsyncioTestCase):
    async def test_paint_look_and_rollback(self):
        with tempfile.TemporaryDirectory(prefix="emcp-real-") as temp:
            studio = Path(temp)
            shutil.copytree(Path(os.environ["EASEL_MCP_STUDIO"]) / "bin", studio / "bin")
            (studio / "notes").mkdir()
            params = StdioServerParameters(command=shutil.which("uv"), args=["run", "--no-project", "--script", str(studio / "bin/mcp/server.py"), "--studio", str(studio)], cwd=str(studio), env=dict(os.environ, RAYON_NUM_THREADS="2"))
            try:
                async with stdio_client(params) as (read, write):
                    async with ClientSession(read, write) as client:
                        await client.initialize()
                        reply = await client.call_tool("paint", {"lua": 'local t=tubes(); canvas{size=100,aspect=1.25,linen=15,ground={{pile={{t[1],1}},um=80,apply="knife"}}}'})
                        self.assertFalse(reply.isError, reply)
                        reply = await client.call_tool("paint", {"lua": 'local t=tubes(); b=brush("round",4); b:load(pile{{t[#t],1}},0.8); b:stroke({{200,300},{800,320}})'})
                        self.assertFalse(reply.isError, reply)
                        reply = await client.call_tool("paint", {"lua": "print(wait(5))"})
                        self.assertFalse(reply.isError, reply)
                        look = await client.call_tool("look", {"size": 320})
                        self.assertFalse(look.isError, look)
                        image = base64.b64decode(look.content[1].data)
                        self.assertTrue(image.startswith(b"\x89PNG\r\n\x1a\n"))
                        before = (studio / "paintings/lua/painting.lua").read_bytes()
                        error = await client.call_tool("paint", {"lua": 'b:stroke({{100,100},{200,200}}); error("MCP rollback proof")'})
                        self.assertTrue(error.isError)
                        self.assertIn("MCP rollback proof", error.content[0].text)
                        self.assertNotIn('chunk 4', error.content[0].text)
                        self.assertEqual((studio / "paintings/lua/painting.lua").read_bytes(), before)
                        after = await client.call_tool("look", {"size": 320})
                        self.assertEqual(base64.b64decode(after.content[1].data), image)
                        output = os.environ.get("EASEL_MCP_PROOF")
                        if output:
                            Path(output).mkdir(parents=True, exist_ok=True)
                            (Path(output) / "look.png").write_bytes(image)
                            (Path(output) / "caption.txt").write_text(look.content[0].text + "\n")
                            (Path(output) / "painting.lua").write_bytes(before)
                            (Path(output) / "error.txt").write_text(error.content[0].text + "\n")
            finally:
                import subprocess
                subprocess.run([str(studio / "bin/easel"), "close"], cwd=studio, capture_output=True)


if __name__ == "__main__":
    unittest.main()
