# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0", "PyYAML==6.0.3", "Pillow==12.3.0"]
# ///

import asyncio
import base64
import io
import importlib.machinery
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import time
import unittest

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
import yaml

from test_protocol import FAKE, PNG, SERVER
from painting_chunks import classify

WATERCOLOR_FAKE = '''#!{python}
import base64, sys
from pathlib import Path
command, root, rest = sys.argv[3], Path(sys.argv[4]), sys.argv[5:]
sources = root / "sources.txt"
if command == "run":
    text = sys.stdin.read()
    seen = sources.read_text().split("\\0") if sources.exists() else []
    if text in seen:
        print("this exact chunk already ran; submit a new continuation chunk")
        sys.exit(1)
    sources.write_text("\\0".join(seen + [text]))
    print("day 1, 09:00\\nok")
elif command == "status":
    print("watercolor: 1x1 cells\\nday 1, 09:00")
elif command == "render":
    (root / rest[0]).write_bytes(base64.b64decode("{png}"))
    print(rest[0] + " (1x1)")
'''


class AgyTests(unittest.IsolatedAsyncioTestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="agy-proof-")
        self.studio = Path(self.temp.name) / "paint-studio-abcd"
        (self.studio / "bin").mkdir(parents=True)
        (self.studio / "notes").mkdir()
        (self.studio / "BRIEF.md").write_text("approved brief unchanged\n")
        (self.studio / "notes/journal.md").write_text("My handoff, unchanged.\n")
        binary = self.studio / "bin/easel"
        fake = FAKE.format(python=sys.executable, png=PNG)
        fake = fake.replace('    print("day 1, 00:00', '    log = root / "paintings/lua/painting.lua"\n    log.parent.mkdir(parents=True, exist_ok=True)\n    before = log.read_text() if log.exists() else ""\n    log.write_text(before + "--@ chunk %d\\n%s\\n" % (before.count("--@ chunk") + 1, text))\n    print("day 1, 00:00')
        binary.write_text(fake)
        binary.chmod(0o755)

    def tearDown(self):
        self.temp.cleanup()

    async def test_end_sitting_blocks_and_returns_handoff(self):
        self.assertEqual(classify("--@ chunk 1\ndab = b.touch; dab(b, 10, 10)"), [(1, True)])
        self.assertEqual(classify("--@ chunk 1\na = b.touch; d = a; e = d\n--@ chunk 2\ne(b, 10, 10)"), [(1, True), (2, True)])
        params = StdioServerParameters(command=sys.executable, args=[str(SERVER), "--studio", str(self.studio),
            "--sessions-root", str(Path(self.temp.name) / "recorded"), "--sittings", "--break-seconds", "0.2", "--sitting-seconds", "600"])
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                status = await session.call_tool("status", {})
                self.assertIn("seconds left", status.content[-1].text)
                await session.call_tool("paint", {"lua": "dab = b.touch"})
                await session.call_tool("paint", {"lua": "dab(b, 0, 0)"})
                await session.call_tool("look", {})
                await session.call_tool("look", {"crop": "0,0,1,1"})
                progress = []

                async def on_progress(value, total, message):
                    progress.append(value)

                started = time.monotonic()
                task = asyncio.create_task(session.call_tool("end_sitting", {}, progress_callback=on_progress))
                await asyncio.sleep(0.05)
                self.assertFalse(task.done())
                result = await task
                self.assertGreaterEqual(time.monotonic() - started, 0.2)
                self.assertFalse(result.isError)
                self.assertIn("Sitting 2", result.content[0].text)
                self.assertIn("Painting clock: day 1, 00:00", result.content[0].text)
                self.assertIn("You're back at the easel.", result.content[0].text)
                self.assertIn("My handoff, unchanged.", result.content[0].text)
                self.assertTrue(any(item.type == "image" for item in result.content))
                self.assertEqual(progress[0], 0)
                await session.call_tool("look", {"crop": "0,0,1,1"})
                not_done = await session.call_tool("end_sitting", {})
                self.assertIn("Sitting 3", not_done.content[0].text)
                await session.call_tool("look", {})
                await session.call_tool("look", {"crop": "0,0,1,1", "mode": "value"})
                await session.call_tool("look", {"scratch": True, "crop": "0,0,1,1"})
                state = json.loads((self.studio / "out/easel/sittings.json").read_text())
                self.assertFalse(state["detail"])
                await session.call_tool("look", {"crop": "0,0,1,1"})
                await session.call_tool("paint", {"lua": "print(wait(0))"})
                done = await session.call_tool("end_sitting", {})
                self.assertIn("The painting is complete.", done.content[0].text)
                self.assertTrue(json.loads((self.studio / "out/easel/sittings.json").read_text())["complete"])
        events = [json.loads(line) for line in (self.studio / "out/easel/mcp/session.jsonl").read_text().splitlines()]
        results = [event["message"] for event in events if event.get("message", {}).get("role") == "toolResult"]
        self.assertTrue(any(any(item["type"] == "image" for item in result["content"]) for result in results))

    async def test_agy_pages_long_text(self):
        binary = self.studio / "bin/easel"
        binary.write_text(binary.read_text().replace('elif command == "do":\n    text = sys.stdin.read()\n', 'elif command == "do":\n    text = sys.stdin.read()\n    if (root / "do-output.txt").exists():\n        print((root / "do-output.txt").read_text())\n', 1))
        guide = "".join(f"line {i}: wet paint é → ✓ {'x' * (i % 90)}\n" for i in range(900)) + "y" * 20000 + "\nno newline at the end"
        (self.studio / "notes/guide.md").write_text(guide)
        (self.studio / "notes/short.md").write_text("short\n")
        (self.studio / "notes/refs").mkdir()
        (self.studio / "log-output.txt").write_text("--@ chunk 1\n" + "dab(b, 1, 1)\n" * 1500)
        (self.studio / "do-output.txt").write_text("printed\n" * 2000)
        params = StdioServerParameters(command=sys.executable, args=[str(SERVER), "--studio", str(self.studio),
            "--sessions-root", str(Path(self.temp.name) / "recorded"), "--sittings", "--agy"])
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                tools = {tool.name: tool for tool in (await session.list_tools()).tools}
                self.assertTrue(all(tool.meta == {"antigravity/alwaysLoad": True} for tool in tools.values()))
                self.assertIn("page", tools["read"].inputSchema["properties"])
                self.assertIn("page", tools["log"].inputSchema["properties"])

                async def pages(tool, args):
                    texts, page = [], 1
                    while True:
                        result = await session.call_tool(tool, dict(args, page=page) if page > 1 else args)
                        self.assertFalse(result.isError, result.content[0].text)
                        text = result.content[0].text
                        self.assertLessEqual(len(text.encode()), 8200)
                        body, line = text.rsplit("\n", 1)
                        texts.append(body)
                        if line.endswith("the last page."):
                            return "".join(texts), line
                        self.assertIn(f'"page": {page + 1}}}', line)
                        page += 1

                whole, last = await pages("read", {"path": "notes/guide.md"})
                self.assertEqual(whole, guide)
                self.assertEqual(whole.encode(), (self.studio / "notes/guide.md").read_bytes())
                first = (await session.call_tool("read", {"path": "notes/guide.md"})).content[0].text
                self.assertTrue(first.endswith('call read with {"path": "notes/guide.md", "page": 2}'))
                self.assertEqual((await session.call_tool("read", {"path": "notes/short.md"})).content[0].text, "short\n")
                self.assertEqual((await session.call_tool("read", {"path": "notes"})).content[0].text, "guide.md\njournal.md\nrefs/\nshort.md")
                self.assertEqual((await session.call_tool("read", {"path": "notes/refs/"})).content[0].text, "empty folder")
                outside = await session.call_tool("read", {"path": ".."})
                self.assertTrue(outside.isError)
                self.assertEqual(outside.content[0].text, "read: the path must be a file or a folder in this studio")
                beyond = await session.call_tool("read", {"path": "notes/guide.md", "page": 99})
                self.assertTrue(beyond.isError)
                log, _ = await pages("log", {})
                self.assertEqual(log, (self.studio / "log-output.txt").read_text().replace("--@ chunk 1", "--@ chunk").rstrip())
                from PIL import Image
                Image.effect_noise((2916, 1746), 64).convert("RGB").save(self.studio / "notes/photo.png")
                photo = await session.call_tool("read", {"path": "notes/photo.png"})
                self.assertEqual(photo.content[0].text, "Read image file [image/jpeg]\n[Image: original 2916x1746, displayed at 2000x1198. Multiply coordinates by 1.46 to map to original image.]")
                self.assertEqual(photo.content[1].mimeType, "image/jpeg")
                self.assertEqual(Image.open(io.BytesIO(base64.b64decode(photo.content[1].data))).size, (2000, 1198))
                Image.new("RGB", (40, 30)).save(self.studio / "notes/small.png")
                small = await session.call_tool("read", {"path": "notes/small.png"})
                self.assertEqual(small.content[0].text, "Read image file [image/png]")
                self.assertEqual(base64.b64decode(small.content[1].data), (self.studio / "notes/small.png").read_bytes())
                Image.new("RGB", (5000, 1)).save(self.studio / "notes/strip.png")
                self.assertIn("displayed at 2000x1.", (await session.call_tool("read", {"path": "notes/strip.png"})).content[0].text)
                Image.new("RGB", (40, 30)).save(self.studio / "notes/jpeg-named.png", "JPEG")
                self.assertEqual((await session.call_tool("read", {"path": "notes/jpeg-named.png"})).content[1].mimeType, "image/jpeg")
                painted = await session.call_tool("paint", {"lua": "print('x')"})
                saved = painted.content[0].text.rsplit("\n", 1)[1]
                self.assertTrue(saved.startswith("Reply saved in full to out/easel/replies/"))
                path = saved.split(" to ", 1)[1].split(". Page 1", 1)[0]
                full = (self.studio / path).read_text()
                self.assertIn("printed\n" * 2000, full)
                self.assertTrue(painted.content[0].text.startswith("printed\n"))
                rest, _ = await pages("read", {"path": path})
                self.assertEqual(rest, full)

    async def test_watercolor_sittings(self):
        (self.studio / "bin/easel").unlink()
        binary = self.studio / "bin/watercolor-studio"
        binary.write_text(WATERCOLOR_FAKE.format(python=sys.executable, png=PNG))
        binary.chmod(0o755)
        params = StdioServerParameters(command=sys.executable, args=[str(SERVER), "--studio", str(self.studio),
            "--sessions-root", str(Path(self.temp.name) / "recorded"), "--sittings", "--agy"])
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                names = [tool.name for tool in (await session.list_tools()).tools]
                self.assertEqual(names, ["paint", "look", "note", "status", "log", "reference", "reference_list", "end_sitting", "read"])

                async def sitting(lua):
                    self.assertFalse((await session.call_tool("paint", {"lua": lua})).isError)
                    await session.call_tool("look", {"mode": "color"})
                    await session.call_tool("look", {"crop": "0,0,1,1"})
                    return (await session.call_tool("end_sitting", {})).content[0].text

                self.assertIn("Painting clock: day 1, 09:00", await sitting("paper{}"))
                self.assertIn("Sitting 3", await sitting("local b = brush{name='w'}\nb:wash{x=0, y=0, width=1, height=1}"))
                self.assertIn("The painting is complete.", await sitting("print(clock())"))
        self.assertNotIn("print(wait(0))", (self.studio / "sources.txt").read_text())
        repo = SERVER.parents[2]
        loader = importlib.machinery.SourceFileLoader("prepare_agy", str(repo / "scripts/prepare_agy_studio"))
        spec = importlib.util.spec_from_loader(loader.name, loader)
        module = importlib.util.module_from_spec(spec)
        loader.exec_module(module)
        module.prepare(self.studio, Path(self.temp.name) / "sessions")
        self.assertTrue((self.studio / ".agents/mcp_config.json").is_file())

    def test_allowlist_configuration_parses(self):
        repo = SERVER.parents[2]
        script = repo / "scripts/prepare_agy_studio"
        loader = importlib.machinery.SourceFileLoader("prepare_agy", str(script))
        spec = importlib.util.spec_from_loader(loader.name, loader)
        module = importlib.util.module_from_spec(spec)
        loader.exec_module(module)
        module.prepare(self.studio, Path(self.temp.name) / "sessions", 30, 600)
        config = json.loads((self.studio / ".agents/mcp_config.json").read_text())
        self.assertEqual(set(config["mcpServers"]), {"easel"})
        self.assertIn("--sittings", config["mcpServers"]["easel"]["args"])
        self.assertIn("--agy", config["mcpServers"]["easel"]["args"])
        metadata = json.loads((self.studio / ".agents/recording.json").read_text())
        self.assertIn(str((Path(self.temp.name) / "sessions").resolve()), config["mcpServers"]["easel"]["args"])
        self.assertTrue(metadata["output"].endswith("--mcp-src-a-paint-studio-abcd--/agy.jsonl"))
        frontmatter = (self.studio / ".agents/agents/easel-painter.md").read_text().split("---", 2)[1]
        agent = yaml.safe_load(frontmatter)
        self.assertEqual(agent["tools"], [])
        self.assertTrue(agent["inheritCustomizations"])
        self.assertEqual(agent["commandExecutionPolicy"], "off")
        self.assertFalse(agent["subagent"])
        self.assertEqual(agent["skills"], [])
        self.assertEqual(agent["plugins"], [])
        self.assertEqual((self.studio / "BRIEF.md").read_text(), "approved brief unchanged\n")


if __name__ == "__main__":
    unittest.main()
