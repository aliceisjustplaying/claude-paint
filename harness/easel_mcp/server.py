# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0"]
# ///

import argparse
import asyncio
import base64
import json
import math
import os
from pathlib import Path
import re
import signal
import sys
import time
from datetime import datetime, timezone
from urllib.parse import unquote, urlparse
from uuid import uuid4

from mcp import types
from mcp.server import Server
from mcp.server.stdio import stdio_server


def hide_counters(text):
    text = re.sub(r'\[string "chunk \d+"\]', '[string "chunk"]', text)
    return re.sub(r"the chunk ran longer than \d+ minutes and was stopped", "the chunk didn't finish and was stopped", text)


def tool_words(text):
    text = text.replace("--crop exceeds 1200 pixels per side; choose a smaller crop (crops stay 1:1)", "a crop may be at most 500 units on either side; choose a smaller crop")
    return re.sub(r"--(crop|mode|size|grid|palette|light|survey|compare|hold|at)\b", r"\1", text)


def studio_path(studio, value):
    value = re.sub(r"[\u00a0\u2000-\u200a\u202f\u205f\u3000]", " ", value)
    value = value.removeprefix("@")
    if value == "~":
        value = str(Path.home())
    elif value.startswith("~/"):
        value = str(Path.home() / value[2:])
    elif value.startswith("file://"):
        try:
            url = urlparse(value)
            if url.username is not None or url.password is not None or url.port is not None or url.netloc.endswith(":"):
                raise ValueError("Invalid URL")
            host = url.hostname
        except ValueError:
            raise ValueError("Invalid URL") from None
        if host not in (None, "localhost"):
            raise ValueError(f'File URL host must be "localhost" or empty on {sys.platform}')
        if re.search(r"%2f", url.path, re.IGNORECASE):
            raise ValueError("File URL path must not include encoded / characters")
        if re.search(r"%(?![\da-fA-F]{2})", url.path):
            raise ValueError("URI malformed")
        try:
            value = unquote(url.path or "/", errors="strict")
        except UnicodeDecodeError:
            raise ValueError("URI malformed") from None
    path = (studio / value).resolve()
    if not path.is_relative_to(studio):
        return None
    return path


def look_args(view):
    if view.get("palette"):
        if any(key != "palette" and value is not None and value is not False and value != "" for key, value in view.items()):
            raise ValueError("look: palette takes no other option")
        return ["--palette"]
    args = []
    if view.get("survey"):
        args.append("--survey")
    for key in ("compare", "hold", "at", "crop", "mode", "light"):
        if view.get(key):
            args.extend(["--" + key, str(view[key])])
    if "size" in view:
        size = view["size"]
        args.extend(["--size", str(int(size)) if size == int(size) else str(size)])
    grid = view.get("grid")
    if grid is True:
        args.append("--grid")
    elif isinstance(grid, (int, float)) and not isinstance(grid, bool):
        args.extend(["--grid", str(int(grid)) if grid == int(grid) else str(grid)])
    return args


def image_limits():
    limits = []
    for name, default, minimum in (("PAINTER_MAX_IMAGES", 20, 1), ("PAINTER_MAX_IMAGE_MB", 12, 0.001)):
        raw = os.environ.get(name, "").strip()
        value = float(raw) if raw else default
        if not math.isfinite(value) or value < minimum:
            raise ValueError(f"{name}={raw}: want a number >= {minimum:g}")
        limits.append(value)
    return math.floor(limits[0]), math.floor(limits[1] * 1_000_000 + 0.5)


class Easel:
    def __init__(self, studio):
        self.studio = studio.resolve(strict=True)
        self.lock = asyncio.Lock()
        self.max_images, self.max_chars = image_limits()

    async def step(self, args, text, deadline):
        env = {"PATH": "/usr/bin:/bin", "HOME": os.environ.get("HOME", "")}
        if "RAYON_NUM_THREADS" in os.environ:
            env["RAYON_NUM_THREADS"] = os.environ["RAYON_NUM_THREADS"]
        proc = await asyncio.create_subprocess_exec(
            str(self.studio / "bin/easel"), *args, cwd=self.studio, env=env,
            stdin=asyncio.subprocess.PIPE, stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.STDOUT, start_new_session=True,
        )
        try:
            out, _ = await asyncio.wait_for(proc.communicate((text or "").encode()), max(0.001, deadline - time.monotonic()))
        except BaseException:
            try:
                os.killpg(proc.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            await proc.wait()
            raise
        return proc.returncode, out.decode(errors="replace")

    async def ensure_open(self, deadline):
        opened = rebuilding = False
        completed, total = -1, None
        progress_deadline = deadline
        args = ["status"]
        while True:
            try:
                code, out = await self.step(args, None, progress_deadline)
            except asyncio.TimeoutError:
                raise ValueError("the easel rebuild stalled without progress" if rebuilding else "the easel didn't open in time" if opened else "the easel isn't answering") from None
            progress = re.fullmatch(r"rebuilding from the log \((\d+) of (\d+) chunks\)", out.strip())
            if progress:
                k, n = map(int, progress.groups())
                if code or k > n or k < completed or (total is not None and n != total):
                    raise ValueError(f"the easel returned invalid rebuild progress: {out.strip()}")
                total = n
                if not rebuilding or k > completed:
                    completed = k
                    progress_deadline = time.monotonic() + 30 * 60
                rebuilding = True
                if time.monotonic() >= progress_deadline:
                    raise ValueError("the easel rebuild stalled without progress")
                await asyncio.sleep(min(0.1, progress_deadline - time.monotonic()))
                args = ["status"]
                continue
            if code == 0:
                return rebuilding
            if opened or rebuilding:
                raise ValueError(out.strip() or "the easel didn't open")
            opened = True
            args = ["open"]

    async def at(self, args, text=None):
        wait = 12 * 60 if args[0] == "do" else 3 * 60
        deadline = time.monotonic() + wait
        if await self.ensure_open(deadline):
            deadline = time.monotonic() + wait
        try:
            code, out = await self.step(args, text, deadline)
        except asyncio.TimeoutError:
            raise ValueError("the easel didn't answer; `log` shows whether the chunk ran") from None
        out = out.rstrip()
        if code:
            raise ValueError(out or "the easel gave no answer")
        return out

    def rename_looks(self, said):
        paths, lines = [], []
        for line in said.split("\n"):
            match = re.fullmatch(r"(.+\.png)( \(\d+x\d+.*)?", line)
            source = studio_path(self.studio, match[1]) if match else None
            if source is None or not source.is_file():
                lines.append(line)
                continue
            old = Path(match[1])
            new = old.with_name(f"{uuid4()}.png")
            source.rename(self.studio / new)
            for folder in (".gallery", ".relief"):
                sibling = source.parent / folder / source.name
                if sibling.is_file():
                    try:
                        sibling.rename(sibling.with_name(new.name))
                    except OSError as error:
                        print(f"warning: couldn't rename {folder[1:]} companion: {error}", file=sys.stderr)
            paths.append(str(new))
            caption = re.sub(r"^ \((\d+x\d+), \d+(?:\.\d+)?s\)", r" (\1)", match[2] or "")
            lines.append(str(new) + caption)
        return "\n".join(lines), paths

    def revise_journal(self, old, text):
        path = self.studio / "notes/journal.md"
        if not old.strip():
            raise ValueError("note: `replaces` is empty; give the exact text of the passage to replace")
        if not path.exists():
            raise ValueError("note: notes/journal.md has no entries yet")
        before = path.read_bytes().decode()
        at = before.find(old)
        if at < 0:
            raise ValueError("note: the passage given in `replaces` isn't in notes/journal.md (it has to match exactly)")
        if before.find(old, at + 1) >= 0:
            raise ValueError("note: the passage given in `replaces` is in notes/journal.md more than once; give more of it")
        log = self.studio / "out/easel/journal-revisions.jsonl"
        log.parent.mkdir(parents=True, exist_ok=True)
        with log.open("a") as file:
            file.write(json.dumps({"at": datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z"), "replaced": old, "with": text, "before": before}, ensure_ascii=False) + "\n")
        path.write_bytes((before[:at] + text + before[at + len(old):]).encode())
        return f"revised in {path}\n"

    async def call(self, name, params):
        scratch = params.get("scratch", False)
        on = ["--scratch"] if scratch else []
        tag = "[scratch canvas]\n" if scratch else ""
        if name == "paint":
            if params.get("new_scratch") and not scratch:
                raise ValueError("paint: new_scratch goes with scratch: true")
            try:
                said = await self.at(["do", "-", *on, *(["--new"] if params.get("new_scratch") else [])], params["lua"])
            except ValueError as error:
                raise ValueError(hide_counters(str(error))) from None
            said = re.sub(r"(^|\n)ok · chunk \d+(?: \(\d+(?:\.\d+)? s to compute\))?(\n?)$", r"\1ok\2", said)
            return [types.TextContent(type="text", text=tag + hide_counters(said))]
        if name == "look":
            view = {key: value for key, value in params.items() if key != "scratch"}
            look_args(view)
            if view.get("survey") and view.get("compare"):
                raise ValueError("look: survey and compare are two looks; ask for one")
            if view.get("compare"):
                compare = studio_path(self.studio, view["compare"])
                if compare is None:
                    raise ValueError("look: compare is the path of an earlier look in this studio")
                view["compare"] = str(compare)
            try:
                said = await self.at(["look", *on, *look_args(view)])
            except ValueError as error:
                raise ValueError(tool_words(str(error))) from None
            said, paths = self.rename_looks(said)
            said = tag + said
            if not paths:
                raise ValueError(said)
            images = [types.ImageContent(type="image", mimeType="image/png", data=base64.b64encode((self.studio / path).read_bytes()).decode()) for path in paths]
            if params.get("survey"):
                for path, image in zip(paths, images):
                    if len(image.data) > self.max_chars:
                        raise ValueError(f"look: survey incomplete: {path} alone exceeds the configured image budget; full-detail tiles remain on disk, but this budget cannot deliver them")
                count, chars = 0, 0
                while count < len(images) and count < self.max_images and chars + len(images[count].data) <= self.max_chars:
                    chars += len(images[count].data)
                    count += 1
                if count < len(images):
                    said += f"\nPartial survey: showing tiles 1–{count} of {len(images)} within the image budget. The remaining full-detail tiles have not been shown. Read each remaining file in a separate turn before assessing the whole canvas:\n" + "\n".join(paths[count:]) + "\n"
                images = images[:count]
            return [types.TextContent(type="text", text=said), *images]
        if name == "note":
            said = self.revise_journal(params["replaces"], params["text"]) if "replaces" in params else await self.at(["note", "-"], params["text"])
        elif name == "status":
            said = await self.at(["status", *on])
            said = tag + re.sub(r"^\d+px · ", "", re.sub(r"^\d+ chunks? · ", "", said))
        elif name == "log":
            said = re.sub(r"^--@ chunk \d+[ \t]*$", "--@ chunk", await self.at(["log", *on]), flags=re.MULTILINE)
            units = said.encode("utf-16-le", errors="surrogatepass")
            if len(units) > 100_000:
                whole = "paintings/lua/scratch.lua" if scratch else "paintings/lua/painting.lua"
                said = f"(the first {len(units) // 2 - 50_000} characters are left out; read {whole} for all of it)\n" + units[-100_000:].decode("utf-16-le", errors="replace")
            said = tag + said
        else:
            raise ValueError(f"Unknown tool: {name}")
        return [types.TextContent(type="text", text=said)]


async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--studio", default=os.environ.get("EASEL_ROOT"))
    parser.add_argument("--sessions-root", type=Path, help="opt in to durable viewer session recording")
    args = parser.parse_args()
    if not args.studio:
        parser.error("set --studio or EASEL_ROOT to an exported studio")
    easel = Easel(Path(args.studio))
    recorded = None
    if args.sessions_root:
        from remote import RemoteStudio
        recorded = RemoteStudio(easel.studio, args.sessions_root, response_wait=24 * 60 * 60, easel=easel)
    server = Server("easel")
    definitions = json.loads(Path(__file__).with_name("tools.json").read_text())

    @server.list_tools()
    async def list_tools():
        return [types.Tool(**tool) for tool in definitions]

    @server.call_tool()
    async def call_tool(name, arguments):
        async with easel.lock:
            if recorded:
                return await recorded.call(name, dict(arguments, call_id=str(uuid4())))
            try:
                return types.CallToolResult(content=await easel.call(name, arguments))
            except (ValueError, OSError) as error:
                return types.CallToolResult(isError=True, content=[types.TextContent(type="text", text=str(error))])

    try:
        async with stdio_server() as (read, write):
            await server.run(read, write, server.create_initialization_options())
    finally:
        if recorded:
            await recorded.close()


if __name__ == "__main__":
    asyncio.run(main())
