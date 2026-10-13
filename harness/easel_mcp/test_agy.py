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
        params = StdioServerParameters(command=sys.executable, args=[str(SERVER), "--studio", str(self.studio),
            "--sessions-root", str(Path(self.temp.name) / "recorded"), "--sittings", "--break-seconds", "0.2", "--sitting-seconds", "600"])
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                status = await session.call_tool("status", {})
                self.assertIn("seconds left", status.content[-1].text)
                await session.call_tool("paint", {"lua": "b:touch(0, 0)"})
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
        metadata = json.loads((self.studio / ".agents/recording.json").read_text())
        self.assertNotIn(str(Path(self.temp.name) / "sessions"), config["mcpServers"]["easel"]["args"])
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
