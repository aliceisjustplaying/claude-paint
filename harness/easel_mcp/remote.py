import asyncio
from collections import OrderedDict
import copy
from datetime import datetime, timezone
import fcntl
import json
import os
from pathlib import Path
import re
import sqlite3

from mcp import types
from mcp.server import Server

from server import Easel, tool_definitions


def stamp():
    return datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")


def text_result(text, error=False):
    return types.CallToolResult(isError=error, content=[types.TextContent(type="text", text=text)])


class RemoteStudio:
    def __init__(self, studio, sessions_root, response_wait=20, easel=None):
        self.studio = Path(studio).resolve(strict=True)
        if not re.fullmatch(r"paint-studio-[0-9a-f]+", self.studio.name):
            raise ValueError("the remote studio folder must be named paint-studio-<hex> for the studio viewer")
        self.state = self.studio / "out/easel/mcp"
        self.state.mkdir(parents=True, exist_ok=True, mode=0o700)
        self.owner = (self.state / "owner.lock").open("a")
        try:
            fcntl.flock(self.owner, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            self.owner.close()
            raise ValueError("this studio already has a remote MCP writer") from None
        self.db = sqlite3.connect(self.state / "calls.sqlite3")
        self.db.execute("PRAGMA journal_mode=WAL")
        self.db.execute("PRAGMA synchronous=FULL")
        self.db.executescript("""
            CREATE TABLE IF NOT EXISTS calls (id TEXT PRIMARY KEY, request TEXT NOT NULL, result TEXT);
            CREATE TABLE IF NOT EXISTS events (seq INTEGER PRIMARY KEY AUTOINCREMENT, body TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
        """)
        self.easel = easel or Easel(self.studio)
        self.response_wait = response_wait
        self.tasks = {}
        self.completed = OrderedDict()
        self.completed_ttl = 10 * 60
        self.max_completed = 8
        self.max_waiting = 4
        self.cleanup_handle = None
        self.closed = False
        self.serial = asyncio.Lock()
        self.journal = self.state / "session.jsonl"
        sessions = Path(sessions_root).expanduser().resolve()
        folder = "--" + str(self.studio).strip("/").replace("/", "-") + "--"
        if "-src-a-" not in folder:
            folder = f"--mcp-src-a-{self.studio.name}--"
        directory = sessions / folder
        directory.mkdir(parents=True, exist_ok=True)
        saved = self.db.execute("SELECT value FROM meta WHERE key = 'viewer_link'").fetchone()
        link = Path(saved[0]) if saved else directory / (stamp().replace(":", "-") + "-easel-mcp.jsonl")
        if link.parent != directory:
            raise ValueError("the studio's viewer session root changed; use its original sessions-root")
        if link.is_symlink():
            if link.resolve() != self.journal:
                raise ValueError("this viewer session folder belongs to another studio")
        elif link.exists():
            raise ValueError("the remote MCP viewer session path already exists")
        else:
            link.symlink_to(self.journal)
        if not saved:
            with self.db:
                self.db.execute("INSERT INTO meta VALUES ('viewer_link', ?)", (str(link),))
        if not self.db.execute("SELECT 1 FROM events LIMIT 1").fetchone():
            with self.db:
                self.event({"type": "session", "cwd": str(self.studio)})
                self.event({"type": "model_change", "modelId": "Capy MCP"})
                self.db.executemany("INSERT INTO meta VALUES (?, ?)", [("projected_seq", "0"), ("projected_bytes", "0")])
        self.project()

    def event(self, body):
        self.db.execute("INSERT INTO events (body) VALUES (?)", (json.dumps(dict(body, timestamp=stamp()), ensure_ascii=False),))

    def project(self):
        meta = dict(self.db.execute("SELECT key, value FROM meta"))
        seq, offset = int(meta["projected_seq"]), int(meta["projected_bytes"])
        rows = self.db.execute("SELECT seq, body FROM events WHERE seq > ? ORDER BY seq", (seq,)).fetchall()
        if not self.journal.exists():
            offset = seq = 0
            rows = self.db.execute("SELECT seq, body FROM events ORDER BY seq").fetchall()
            self.journal.touch(mode=0o600)
        with self.journal.open("r+b") as file:
            file.truncate(offset)
            file.seek(offset)
            for seq, body in rows:
                file.write((body + "\n").encode())
            file.flush()
            os.fsync(file.fileno())
            offset = file.tell()
        with self.db:
            self.db.executemany("UPDATE meta SET value = ? WHERE key = ?", [(str(seq), "projected_seq"), (str(offset), "projected_bytes")])

    async def execute(self, call_id, name, arguments, request):
        async with self.serial:
            with self.db:
                self.db.execute("INSERT INTO calls VALUES (?, ?, NULL)", (call_id, request))
                self.event({"type": "message", "message": {"role": "assistant", "content": [{"type": "toolCall", "id": call_id, "name": name, "arguments": arguments}]}})
            self.project()
            try:
                result = types.CallToolResult(content=await self.easel.call(name, arguments))
            except (OSError, ValueError) as error:
                result = text_result(str(error), error=True)
            serialized = result.model_dump(mode="json", exclude_none=True)
            with self.db:
                self.db.execute("UPDATE calls SET result = ? WHERE id = ?", (json.dumps(serialized, ensure_ascii=False), call_id))
                self.event({"type": "message", "message": {"role": "toolResult", "toolCallId": call_id, "isError": result.isError, "content": serialized["content"]}})
            self.project()
            return result

    def evict_completed(self):
        if self.cleanup_handle:
            self.cleanup_handle.cancel()
            self.cleanup_handle = None
        loop = asyncio.get_running_loop()
        now = loop.time()
        while self.completed:
            completed_at, _ = next(iter(self.completed.values()))
            if len(self.completed) <= self.max_completed and now < completed_at + self.completed_ttl:
                break
            self.completed.popitem(last=False)
        if self.completed:
            completed_at, _ = next(iter(self.completed.values()))
            self.cleanup_handle = loop.call_later(completed_at + self.completed_ttl - now, self.evict_completed)

    def finished(self, call_id, task):
        self.tasks.pop(call_id, None)
        if not task.cancelled() and task.exception() is None and not self.closed:
            self.completed[call_id] = (asyncio.get_running_loop().time(), task)
            self.evict_completed()

    async def call(self, name, arguments):
        self.evict_completed()
        arguments = dict(arguments)
        call_id = arguments.pop("call_id")
        request = json.dumps({"name": name, "arguments": arguments}, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
        row = self.db.execute("SELECT request, result FROM calls WHERE id = ?", (call_id,)).fetchone()
        task = self.tasks.get(call_id)
        if row:
            if row[0] != request:
                return text_result("call_id already belongs to a different call", error=True)
            if row[1] is not None:
                completed = self.completed.get(call_id)
                if completed:
                    return completed[1].result()
                return types.CallToolResult.model_validate_json(row[1])
            if task is None:
                return text_result("This call was interrupted before its result was recorded. It will not run again. Check log and status before continuing.", error=True)
        elif task:
            if task[0] != request:
                return text_result("call_id already belongs to a different call", error=True)
        else:
            if len(self.tasks) >= 1 + self.max_waiting:
                return text_result("The easel is busy. Retry this same call with the same call_id later.", error=True)
            task = (request, asyncio.create_task(self.execute(call_id, name, arguments, request)))
            self.tasks[call_id] = task
            task[1].add_done_callback(lambda done: self.finished(call_id, done))
        try:
            result = await asyncio.wait_for(asyncio.shield(task[1]), self.response_wait)
        except asyncio.TimeoutError:
            return text_result(f"Call {call_id} is still running. Retry this same call with the same call_id to get its result.")
        return result

    async def close(self):
        self.closed = True
        tasks = [task[1] for task in self.tasks.values()]
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        if self.cleanup_handle:
            self.cleanup_handle.cancel()
            self.cleanup_handle = None
        self.tasks.clear()
        self.completed.clear()
        self.db.close()
        self.owner.close()


def mcp_server(studio):
    server = Server("easel")
    definitions = tool_definitions(studio.studio)

    @server.list_tools()
    async def list_tools():
        tools = copy.deepcopy(definitions)
        for tool in tools:
            schema = tool["inputSchema"]
            schema["properties"]["call_id"] = {
                "type": "string", "minLength": 1, "maxLength": 128,
                "description": "A unique ID for this call. Reuse it to retry this same call or retrieve its result while it is running.",
            }
            schema["required"].append("call_id")
        return [types.Tool(**tool) for tool in tools]

    @server.call_tool()
    async def call_tool(name, arguments):
        return await studio.call(name, arguments)

    return server
