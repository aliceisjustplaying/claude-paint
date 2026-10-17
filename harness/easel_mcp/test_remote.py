# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0", "uvicorn==0.42.0", "pillow>=10"]
# ///

import asyncio
import base64
from contextlib import asynccontextmanager
import http.server
import importlib.util
from importlib.machinery import SourceFileLoader
import json
from pathlib import Path
import secrets
import socket
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch
from io import StringIO
import plistlib

import httpx
from mcp import ClientSession, types
from mcp.client.streamable_http import streamable_http_client
import uvicorn

from http_server import app_for
from remote import RemoteStudio

PNG = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/a9sAAAAASUVORK5CYII="


class FakeEasel:
    def __init__(self):
        self.calls = []
        self.gate = asyncio.Event()

    async def call(self, name, arguments):
        self.calls.append((name, arguments))
        if arguments.get("lua") == "slow":
            await self.gate.wait()
        if arguments.get("lua") == "bad":
            raise ValueError("the easel's exact error")
        if name == "look":
            return [types.TextContent(type="text", text="out/easel/painting/example.png (1x1)"), types.ImageContent(type="image", mimeType="image/png", data=PNG)]
        return [types.TextContent(type="text", text="ok")]


class RemoteTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="mcp-http-")
        self.root = Path(self.temp.name).resolve()
        self.studio_path = self.root / "paint-studio-aabbcc"
        self.studio_path.mkdir()
        self.sessions = self.root / "sessions"
        self.fake = FakeEasel()
        self.owner = RemoteStudio(self.studio_path, self.sessions, response_wait=0.03, easel=self.fake)
        self.token = secrets.token_urlsafe(32)
        listener = socket.socket()
        listener.bind(("127.0.0.1", 0))
        self.url = f"http://127.0.0.1:{listener.getsockname()[1]}/mcp"
        config = uvicorn.Config(app_for(self.owner, self.token), log_level="error", access_log=False)
        self.http = uvicorn.Server(config)
        self.serving = asyncio.create_task(self.http.serve(sockets=[listener]))
        for _ in range(100):
            if self.http.started:
                break
            await asyncio.sleep(0.01)
        self.assertTrue(self.http.started)

    async def asyncTearDown(self):
        self.http.should_exit = True
        await asyncio.wait_for(self.serving, 5)
        self.temp.cleanup()

    @asynccontextmanager
    async def client(self):
        async with httpx.AsyncClient(headers={"Authorization": "Bearer " + self.token}) as http:
            async with streamable_http_client(self.url, http_client=http) as (read, write, _):
                async with ClientSession(read, write) as client:
                    await client.initialize()
                    yield client

    async def call_done(self, client, name, arguments):
        result = await client.call_tool(name, arguments)
        pending = f"Call {arguments['call_id']} is still running. Retry this same call with the same call_id to get its result."
        if not result.isError and len(result.content) == 1 and getattr(result.content[0], "text", None) == pending:
            task = self.owner.tasks.get(arguments["call_id"])
            if task is not None:
                await asyncio.wait_for(asyncio.shield(task[1]), 5)
            result = await client.call_tool(name, arguments)
        return result

    async def test_completed_call_retries_a_running_reply(self):
        self.owner.response_wait = 0
        async with self.client() as client:
            result = await self.call_done(client, "paint", {"lua": "a", "call_id": "wait-1"})
        self.assertEqual(result.content[0].text, "ok")
        self.assertEqual(len(self.fake.calls), 1)

    async def test_bearer_auth_and_host_gate(self):
        async with httpx.AsyncClient() as http:
            for headers in ({}, {"Authorization": "Bearer wrong"}):
                response = await http.post(self.url, json={}, headers=headers)
                self.assertEqual(response.status_code, 401)
            response = await http.post(self.url, json={}, headers={"Authorization": "Bearer " + self.token, "Host": "evil.example"})
            self.assertEqual(response.status_code, 421)
        self.assertEqual(self.fake.calls, [])

    async def test_tools_require_call_id(self):
        async with self.client() as client:
            tools = await client.list_tools()
            self.assertEqual([tool.name for tool in tools.tools], ["paint", "look", "probe", "note", "status", "log", "reference", "reference_list"])
            for tool in tools.tools:
                self.assertIn("call_id", tool.inputSchema["required"])
            result = await client.call_tool("paint", {"lua": "a"})
            self.assertTrue(result.isError)
        self.assertEqual(self.fake.calls, [])

    async def test_reconnect_returns_cached_result_once(self):
        arguments = {"lua": "a chunk", "call_id": "paint-1"}
        async with self.client() as client:
            first = await self.call_done(client, "paint", arguments)
        async with self.client() as client:
            second = await self.call_done(client, "paint", arguments)
            conflict = await client.call_tool("paint", dict(arguments, lua="a different chunk"))
        self.assertEqual(first.content, second.content)
        self.assertTrue(conflict.isError)
        self.assertEqual(len(self.fake.calls), 1)
        self.assertEqual(len(self.owner.journal.read_text().splitlines()), 4)

    async def test_running_reply_and_same_call_retry(self):
        arguments = {"lua": "slow", "call_id": "slow-1"}
        async with self.client() as client:
            pending = await client.call_tool("paint", arguments)
            self.assertIn("is still running", pending.content[0].text)
            again = await client.call_tool("paint", arguments)
            self.assertIn("is still running", again.content[0].text)
            self.fake.gate.set()
            done = await self.call_done(client, "paint", arguments)
            self.assertEqual(done.content[0].text, "ok")
        self.assertEqual(len(self.fake.calls), 1)
        self.assertNotIn("slow-1", self.owner.tasks)

    async def test_distinct_calls_are_serialized(self):
        async with self.client() as client:
            await client.call_tool("paint", {"lua": "slow", "call_id": "serial-1"})
            await client.call_tool("paint", {"lua": "next", "call_id": "serial-2"})
            self.assertEqual([args["lua"] for _, args in self.fake.calls], ["slow"])
            self.fake.gate.set()
            await self.call_done(client, "paint", {"lua": "next", "call_id": "serial-2"})
        self.assertEqual([args["lua"] for _, args in self.fake.calls], ["slow", "next"])

    async def test_dropped_client_does_not_cancel_chunk(self):
        self.owner.response_wait = 1
        arguments = {"lua": "slow", "call_id": "drop-1"}
        async with self.client() as client:
            request = asyncio.create_task(client.call_tool("paint", arguments))
            for _ in range(100):
                if self.fake.calls:
                    break
                await asyncio.sleep(0.01)
            self.assertEqual(len(self.fake.calls), 1)
            work = self.owner.tasks["drop-1"][1]
            request.cancel()
            await asyncio.gather(request, return_exceptions=True)
        self.fake.gate.set()
        await work
        await asyncio.sleep(0)
        self.assertIn("drop-1", self.owner.completed)
        async with self.client() as client:
            result = await client.call_tool("paint", arguments)
        self.assertEqual(result.content[0].text, "ok")
        self.assertEqual(len(self.fake.calls), 1)

    async def test_completed_task_expires_without_another_request(self):
        self.owner.completed_ttl = 0.03
        arguments = {"lua": "slow", "call_id": "expire-1"}
        async with self.client() as client:
            pending = await client.call_tool("paint", arguments)
            self.assertIn("is still running", pending.content[0].text)
            work = self.owner.tasks["expire-1"][1]
            self.fake.gate.set()
            await work
            await asyncio.sleep(0)
            self.assertIn("expire-1", self.owner.completed)
            await asyncio.sleep(0.05)
            self.assertEqual(self.owner.tasks, {})
            self.assertEqual(self.owner.completed, {})
            cached = await client.call_tool("paint", arguments)
            self.assertEqual(cached.content[0].text, "ok")
        self.assertEqual(len(self.fake.calls), 1)

    async def test_completed_cache_evicts_oldest_at_count_limit(self):
        self.owner.max_completed = 2
        async with self.client() as client:
            for index in range(3):
                await self.call_done(client, "paint", {"lua": "a", "call_id": f"count-{index}"})
            self.assertEqual(list(self.owner.completed), ["count-1", "count-2"])
            cached = await client.call_tool("paint", {"lua": "a", "call_id": "count-0"})
            self.assertEqual(cached.content[0].text, "ok")
            self.assertEqual(list(self.owner.completed), ["count-1", "count-2"])
        self.assertEqual(len(self.fake.calls), 3)

    async def test_queue_bound_rejects_new_work_but_allows_retries(self):
        async with self.client() as client:
            await client.call_tool("paint", {"lua": "slow", "call_id": "running"})
            for index in range(self.owner.max_waiting):
                reply = await client.call_tool("paint", {"lua": f"queued-{index}", "call_id": f"queue-{index}"})
                self.assertIn("is still running", reply.content[0].text)
            extra = {"lua": "extra", "call_id": "overflow"}
            busy = await client.call_tool("paint", extra)
            self.assertTrue(busy.isError)
            self.assertEqual(busy.content[0].text, "The easel is busy. Retry this same call with the same call_id later.")
            self.assertEqual(len(self.owner.tasks), 5)
            self.assertIsNone(self.owner.db.execute("SELECT request FROM calls WHERE id = 'overflow'").fetchone())
            retry = await client.call_tool("paint", {"lua": "queued-0", "call_id": "queue-0"})
            self.assertIn("is still running", retry.content[0].text)
            self.assertEqual(len(self.owner.tasks), 5)
            self.assertEqual([args["lua"] for _, args in self.fake.calls], ["slow"])
            work = [task[1] for task in self.owner.tasks.values()]
            self.fake.gate.set()
            await asyncio.gather(*work)
            await asyncio.sleep(0)
            admitted = await self.call_done(client, "paint", extra)
            self.assertEqual(admitted.content[0].text, "ok")
        self.assertEqual([args["lua"] for _, args in self.fake.calls], ["slow", "queued-0", "queued-1", "queued-2", "queued-3", "extra"])

    async def test_inline_image_and_verbatim_error(self):
        async with self.client() as client:
            look = await self.call_done(client, "look", {"call_id": "look-1"})
            error = await self.call_done(client, "paint", {"lua": "bad", "call_id": "error-1"})
        self.assertEqual(look.content[1].mimeType, "image/png")
        self.assertEqual(base64.b64decode(look.content[1].data), base64.b64decode(PNG))
        self.assertTrue(error.isError)
        self.assertEqual(error.content[0].text, "the easel's exact error")

    async def test_one_adapter_owns_studio(self):
        with self.assertRaisesRegex(ValueError, "already has a remote MCP writer"):
            RemoteStudio(self.studio_path, self.sessions, easel=FakeEasel())

    async def test_existing_viewer_sees_http_paint_and_image(self):
        studio_dir = Path(__file__).resolve().parents[2] / "studio"
        sys.path.insert(0, str(studio_dir))
        spec = importlib.util.spec_from_file_location("remote_test_viewer", studio_dir / "studio.py")
        viewer = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(viewer)
        viewer.SESSIONS = str(self.sessions)
        server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), viewer.H)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        url = f"http://127.0.0.1:{server.server_port}"
        try:
            async with self.client() as client:
                await self.call_done(client, "paint", {"lua": "paint a mark", "call_id": "viewer-paint"})
                await self.call_done(client, "look", {"call_id": "viewer-look"})
            async with httpx.AsyncClient() as viewer_client:
                sessions = (await viewer_client.get(url + "/api/sessions")).json()
                self.assertTrue(sessions[0]["painter"])
                self.assertEqual(sessions[0]["p"], self.studio_path.name)
                events = (await viewer_client.get(url + "/api/events", params={"p": self.studio_path.name})).json()["events"]
                self.assertTrue(any(event["kind"] == "paint" and event["code"] == "paint a mark" for event in events))
                image = next(event for event in events if event["kind"] == "image")
                response = await viewer_client.get(url + "/img", params={"p": self.studio_path.name, "i": image["img"]})
                self.assertEqual(response.content, base64.b64decode(PNG))
        finally:
            await asyncio.to_thread(server.shutdown)
            server.server_close()
            sys.path.remove(str(studio_dir))


class RecoveryTests(unittest.IsolatedAsyncioTestCase):
    async def test_completed_result_survives_adapter_restart(self):
        with tempfile.TemporaryDirectory(prefix="mcp-recovery-") as temp:
            root = Path(temp).resolve()
            path = root / "paint-studio-aabbcc"
            path.mkdir()
            first = RemoteStudio(path, root / "sessions", easel=FakeEasel())
            result = await first.call("paint", {"lua": "a", "call_id": "restart-1"})
            journal = first.journal.read_bytes()
            await first.close()
            with (path / "out/easel/mcp/session.jsonl").open("ab") as file:
                file.write(b"an uncommitted partial projection")
            fake = FakeEasel()
            second = RemoteStudio(path, root / "sessions", easel=fake)
            try:
                cached = await second.call("paint", {"lua": "a", "call_id": "restart-1"})
                self.assertEqual(result.content, cached.content)
                self.assertEqual(fake.calls, [])
                self.assertEqual(second.journal.read_bytes(), journal)
            finally:
                await second.close()

    async def test_interrupted_outcome_fails_closed_after_restart(self):
        with tempfile.TemporaryDirectory(prefix="mcp-uncertain-") as temp:
            root = Path(temp).resolve()
            path = root / "paint-studio-aabbcc"
            path.mkdir()
            first = RemoteStudio(path, root / "sessions", response_wait=0.01, easel=FakeEasel())
            await first.call("paint", {"lua": "slow", "call_id": "uncertain-1"})
            await first.close()
            fake = FakeEasel()
            second = RemoteStudio(path, root / "sessions", easel=fake)
            try:
                result = await second.call("paint", {"lua": "slow", "call_id": "uncertain-1"})
                self.assertTrue(result.isError)
                self.assertIn("will not run again", result.content[0].text)
                self.assertEqual(fake.calls, [])
            finally:
                await second.close()


class HelperTests(unittest.TestCase):
    def load(self, name):
        path = Path(__file__).resolve().parents[2] / "scripts" / name
        loader = SourceFileLoader(name, str(path))
        module = importlib.util.module_from_spec(importlib.util.spec_from_loader(name, loader))
        loader.exec_module(module)
        return module

    def test_registration_uses_latest_url_and_secret_reference(self):
        helper = self.load("easel_mcp_registration")
        config = helper.registration("https://old-url.trycloudflare.com\nhttps://new-url.trycloudflare.com", "EASEL_MCP_AUTHORIZATION")
        self.assertEqual(config["source"]["transport"]["url"], "https://new-url.trycloudflare.com/mcp")
        self.assertEqual(config["secrets"], {"credential_header:Authorization": {"project_env": "EASEL_MCP_AUTHORIZATION"}})
        with self.assertRaises(ValueError):
            helper.registration("a log without a URL")

    def test_launchd_is_staged_without_secret_values(self):
        helper = self.load("prepare_remote_easel_launchd")
        with tempfile.TemporaryDirectory(prefix="mcp-plist-") as temp:
            out = Path(temp).resolve() / "stage"
            argv = ["prepare_remote_easel_launchd", "--studio", str(Path(temp) / "paint-studio-aabbcc"), "--token-file", str(Path(temp) / "token"), "--out", str(out), "--uv", "/path/to/uv", "--cloudflared", "/path/to/cloudflared"]
            with patch.object(sys, "argv", argv), patch.object(sys, "stdout", StringIO()):
                helper.main()
            plists = [plistlib.loads(path.read_bytes()) for path in out.glob("*.plist")]
            self.assertEqual(len(plists), 2)
            tunnel = next(plist for plist in plists if plist["Label"].endswith("tunnel"))
            args = tunnel["ProgramArguments"]
            self.assertIn("--http-host-header", args)
            self.assertIn("localhost:8790", args)
            self.assertFalse(any("EnvironmentVariables" in plist for plist in plists))


if __name__ == "__main__":
    unittest.main()
