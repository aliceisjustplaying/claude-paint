# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0", "PyYAML==6.0.3", "Pillow==12.3.0"]
# ///

import asyncio
import json
import os
from pathlib import Path
import re
import runpy
import shutil
import sys
import time
import unittest
from unittest.mock import patch

import anyio
from mcp import ClientSession, StdioServerParameters, types
from mcp.client.stdio import stdio_client

import test_agy
from test_protocol import SERVER


STUB = '''#!{python}
import json, os, signal, sys, time
from pathlib import Path
studio = Path.cwd()
args = sys.argv[1:]
critic = Path(json.loads(os.environ["PAINTER_CRITIC"])["dir"])
with (critic / "calls.jsonl").open("a") as out:
    out.write(json.dumps({{"cwd": str(studio), "args": args,
        "note": (studio / "notes/to_critic.md").read_text() if (studio / "notes/to_critic.md").exists() else None,
        "state": json.loads((studio / "out/easel/sittings.json").read_text())}}) + "\\n")
if (critic / "fail").exists():
    print("stub critic unavailable", file=sys.stderr)
    sys.exit(1)
if (critic / "hold").exists():
    def stop(*_):
        (critic / "stopped").touch()
        sys.exit(0)
    signal.signal(signal.SIGTERM, stop)
    (critic / "started").touch()
    while True:
        time.sleep(0.01)
sessions = Path(args[args.index("--session-dir") + 1])
sessions.mkdir(parents=True)
reply = "Lift the sky.\\nKeep the shore." + ("\\nMore detail." * 1000 if (critic / "long").exists() else "")
(sessions / "visit.jsonl").write_text(json.dumps({{"message": {{"role": "assistant", "stopReason": "stop",
    "content": [{{"type": "text", "text": reply}}]}}}}) + "\\n")
'''


class CriticTests(unittest.IsolatedAsyncioTestCase):
    setUp = test_agy.AgyTests.setUp
    tearDown = test_agy.AgyTests.tearDown

    def parameters(self, enabled=True, prepared=False):
        root = Path(self.temp.name)
        self.critic = root / "critic"
        self.critic.mkdir(exist_ok=True)
        binary = self.studio / "bin/pi"
        binary.write_text(STUB.format(python=sys.executable))
        binary.chmod(0o755)
        env = dict(os.environ, PATH=str(binary.parent) + os.pathsep + os.environ["PATH"])
        env.pop("PAINTER_CRITIC", None)
        env.pop("PAINTER_CRITIC_CLI", None)
        if enabled:
            env["PAINTER_CRITIC"] = json.dumps({"dir": str(self.critic), "args": [
                "-e", str(root / "pi-black.ts"), "--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "xhigh"]})
        if prepared:
            prepare = runpy.run_path(str(SERVER.parents[2] / "scripts/prepare_agy_studio"))["prepare"]
            with patch.dict(os.environ, env, clear=True):
                config = prepare(self.studio, root / "recorded")["mcpServers"]["easel"]
            return StdioServerParameters(command=config["command"], args=config["args"], cwd=config["cwd"], env={**env, **config["env"]})
        return StdioServerParameters(command=sys.executable, args=[str(SERVER), "--studio", str(self.studio),
            "--sessions-root", str(root / "recorded"), "--sittings", "--agy"], env=env)

    def calls(self):
        return [json.loads(line) for line in (self.critic / "calls.jsonl").read_text().splitlines()]

    async def test_ask_critic_runs_shared_visit_and_keeps_note_reply_and_sessions(self):
        params = self.parameters(prepared=True)
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                tools = {tool.name: tool for tool in (await session.list_tools()).tools}
                self.assertEqual(tools["ask_critic"].meta, {"antigravity/alwaysLoad": True})
                note = "What about the sky?  \n"
                result = await session.call_tool("ask_critic", {"note": note})
                self.assertFalse(result.isError, result.content)
                expected = "From the critic's visit you asked for:\n\nLift the sky.\nKeep the shore.\n"
                self.assertEqual(result.content[0].text, expected)
                self.assertEqual((self.studio / "notes/to_critic.md").read_text(), "What about the sky?\n")
                self.assertEqual((self.studio / "notes/critic.md").read_text(), expected)
                self.assertEqual(next(self.critic.glob("*-asked.md")).read_text(), expected)
                self.assertEqual(json.loads((self.critic / "notes-to-critic.jsonl").read_text())["note"], note)
                result = await session.call_tool("ask_critic", {})
                self.assertEqual(result.content[0].text, expected)
                (self.critic / "long").touch()
                long = await session.call_tool("ask_critic", {"note": "  "})
                self.assertFalse(long.isError, long.content)
                self.assertIn("Reply saved in full to out/easel/replies/", long.content[0].text)
                saved = next((self.studio / "out/easel/replies").glob("*.txt"))
                self.assertEqual(saved.read_text(), (self.studio / "notes/critic.md").read_text())
                self.assertLess(len(long.content[0].text.encode()), 8300)
        calls = self.calls()
        self.assertEqual(len(calls), 3)
        for call in calls:
            self.assertEqual(call["cwd"], str(self.studio))
            self.assertEqual(call["note"], "What about the sky?\n")
            args = call["args"]
            self.assertIn("--no-mcp", args)
            self.assertIn("--no-extensions", args)
            self.assertIn("--no-context-files", args)
            self.assertEqual(args[args.index("--tools") + 1], "look,read")
            self.assertTrue(args[args.index("-e") + 1].endswith("harness/painter/critic.ts"))
            self.assertEqual(args[args.index("--model") + 1], "claude-opus-5-5")
            self.assertEqual(args[args.index("--thinking") + 1], "xhigh")
            self.assertIn(str(Path(self.temp.name) / "pi-black.ts"), args)
        self.assertEqual(len({c["args"][c["args"].index("--session-dir") + 1] for c in calls}), 3)
        self.assertEqual(len(list((self.critic / "sessions").glob("*/visit.jsonl"))), 3)
        events = [json.loads(line) for line in (self.studio / "out/easel/mcp/session.jsonl").read_text().splitlines()]
        replies = [e["message"] for e in events if e.get("message", {}).get("role") == "toolResult"]
        self.assertEqual(len(replies), 3)

    def test_description_matches_pi_word_for_word(self):
        source = (SERVER.parents[1] / "painter/painter.ts").read_text()
        description = re.search(r'name: "ask_critic",.*?description:(.*?)parameters:', source, re.S)[1]
        expected = "".join(re.findall(r'["`]([^"`]+)["`]', description))
        expected = expected.replace("${CRITIQUE}", "notes/critic.md").replace("${TO_CRITIC}", "notes/to_critic.md")
        tool = json.loads(SERVER.with_name("critic-tools.json").read_text())[0]
        self.assertEqual(tool["description"], expected)

    async def test_recorded_sitting_end_visits_before_next_message_and_only_once(self):
        params = self.parameters()
        state = self.studio / "out/easel/sittings.json"
        state.parent.mkdir(parents=True)
        state.write_text(json.dumps({"sitting": 4, "started": time.time(), "painting_before": 0,
            "whole": False, "detail": False, "due": time.time() - 1, "complete": False}))
        (self.studio / "notes/journal.md").write_text("Open passages:\n- the shore: flat\n")
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                result = await session.call_tool("end_sitting", {})
                self.assertFalse(result.isError, result.content)
                text = result.content[0].text
                self.assertTrue(text.startswith("## Passages you left open\n"), text)
                self.assertIn("> Open passages:\n> - the shore: flat", text)
                self.assertIn("> From the critic's visit at the end of a sitting:\n>\n> Lift the sky.\n> Keep the shore.", text)
                self.assertLess(text.index("The critic's notes"), text.index("You're back at the easel."))
                self.assertIn("Sitting 5", text)
                await session.call_tool("look", {})
                await session.call_tool("look", {"crop": "0,0,1,1"})
                complete = await session.call_tool("end_sitting", {})
                self.assertIn("The painting is complete.", complete.content[0].text)
                await session.call_tool("end_sitting", {})
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                complete = await session.call_tool("end_sitting", {})
                self.assertIn("The painting is complete.", complete.content[0].text)
        calls = self.calls()
        self.assertEqual(len(calls), 2)
        self.assertEqual(calls[0]["state"]["sitting"], 4)
        self.assertIsNotNone(calls[0]["state"]["due"])
        self.assertTrue(calls[1]["state"]["complete"])
        self.assertEqual(json.loads(state.read_text())["critic_sitting"], 5)
        self.assertEqual(len(list(self.critic.glob("*-sitting.md"))), 2)

    async def test_failed_visit_keeps_recorded_end_for_retry(self):
        params = self.parameters()
        (self.critic / "fail").touch()
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                result = await session.call_tool("end_sitting", {})
                self.assertTrue(result.isError)
                self.assertIn("stub critic unavailable", result.content[0].text)
                state = json.loads((self.studio / "out/easel/sittings.json").read_text())
                self.assertEqual(state["sitting"], 1)
                self.assertIsNotNone(state["due"])
                self.assertNotIn("critic_sitting", state)
                (self.critic / "fail").unlink()
                result = await session.call_tool("end_sitting", {})
                self.assertFalse(result.isError, result.content)
                self.assertIn("Sitting 2", result.content[0].text)
                self.assertIn("> Lift the sky.", result.content[0].text)

    async def test_cancelled_mcp_visit_stops_critic_process(self):
        params = self.parameters()
        (self.critic / "hold").touch()
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                request_id = session._request_id
                request = asyncio.create_task(session.call_tool("ask_critic", {}))
                with anyio.fail_after(5):
                    while not (self.critic / "started").exists():
                        await asyncio.sleep(0.01)
                await session.send_notification(types.ClientNotification(types.CancelledNotification(
                    params=types.CancelledNotificationParams(requestId=request_id))))
                await asyncio.gather(request, return_exceptions=True)
                with anyio.fail_after(5):
                    while not (self.critic / "stopped").exists():
                        await asyncio.sleep(0.01)
        self.assertFalse((self.studio / "notes/critic.md").exists())

    async def test_disabled_studio_has_no_critic_tool_or_visit(self):
        params = self.parameters(enabled=False)
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                self.assertNotIn("ask_critic", [tool.name for tool in (await session.list_tools()).tools])
                self.assertTrue((await session.call_tool("ask_critic", {})).isError)
                result = await session.call_tool("end_sitting", {})
                self.assertTrue(result.content[0].text.startswith("You're back at the easel."))
        self.assertFalse((self.critic / "calls.jsonl").exists())
        self.assertFalse((self.studio / "notes/critic.md").exists())

    async def test_other_exported_runtimes_ignore_inherited_critic_switch(self):
        params = self.parameters()
        runtime = self.studio / "bin/mcp"
        runtime.mkdir()
        for name in ("server.py", "references.py", "tools.json"):
            shutil.copy2(SERVER.with_name(name), runtime / name)
        params.args = [str(runtime / "server.py"), "--studio", str(self.studio)]
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                self.assertNotIn("ask_critic", [tool.name for tool in (await session.list_tools()).tools])
                self.assertFalse((await session.call_tool("status", {})).isError)
        self.assertFalse((self.critic / "calls.jsonl").exists())

    async def test_watercolor_has_no_critic_tool_or_end_visit(self):
        params = self.parameters()
        (self.studio / "bin/easel").unlink()
        binary = self.studio / "bin/watercolor-studio"
        binary.write_text(test_agy.WATERCOLOR_FAKE.format(python=sys.executable, png=test_agy.PNG))
        binary.chmod(0o755)
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                self.assertNotIn("ask_critic", [tool.name for tool in (await session.list_tools()).tools])
                result = await session.call_tool("end_sitting", {})
                self.assertFalse(result.isError, result.content)
                self.assertTrue(result.content[0].text.startswith("You're back at the easel."))
        self.assertFalse((self.critic / "calls.jsonl").exists())


if __name__ == "__main__":
    unittest.main()
