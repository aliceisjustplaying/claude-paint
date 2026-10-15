# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0", "PyYAML==6.0.3"]
# ///

import asyncio
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
                beyond = await session.call_tool("read", {"path": "notes/guide.md", "page": 99})
                self.assertTrue(beyond.isError)
                log, _ = await pages("log", {})
                self.assertEqual(log, (self.studio / "log-output.txt").read_text().replace("--@ chunk 1", "--@ chunk").rstrip())
                painted = await session.call_tool("paint", {"lua": "print('x')"})
                saved = painted.content[0].text.rsplit("\n", 1)[1]
                self.assertTrue(saved.startswith("Reply saved in full to out/easel/replies/"))
                path = saved.split(" to ", 1)[1].split(". Page 1", 1)[0]
                full = (self.studio / path).read_text()
                self.assertIn("printed\n" * 2000, full)
                self.assertTrue(painted.content[0].text.startswith("printed\n"))
                rest, _ = await pages("read", {"path": path})
                self.assertEqual(rest, full)

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
        self.assertEqual(set(agent["tools"]), {"mcp_easel_" + tool for tool in ("paint", "look", "note", "status", "log", "read", "end_sitting")})
        self.assertEqual(agent["commandExecutionPolicy"], "off")
        self.assertFalse(agent["subagent"])
        self.assertEqual(agent["skills"], [])
        self.assertEqual(agent["plugins"], [])
        self.assertEqual((self.studio / "BRIEF.md").read_text(), "approved brief unchanged\n")


if __name__ == "__main__":
    unittest.main()
