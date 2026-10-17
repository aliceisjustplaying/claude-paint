# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0", "Pillow==12.3.0"]
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
from datetime import datetime, timedelta, timezone
from urllib.parse import unquote, urlparse
from uuid import uuid4

from mcp import types
from mcp.server import Server
from mcp.server.stdio import stdio_server
import anyio

from references import reference, reference_list


def hide_counters(text):
    text = re.sub(r'\[string "chunk \d+"\]', '[string "chunk"]', text)
    return re.sub(r"the chunk ran longer than \d+ minutes and was stopped", "the chunk didn't finish and was stopped", text)


def tool_words(text):
    text = text.replace("--crop exceeds 1200 pixels per side; choose a smaller crop (crops stay 1:1)", "a crop may be at most 500 units on either side; choose a smaller crop")
    return re.sub(r"--(crop|mask|mode|size|grid|palette|light|survey|compare|ref|hold|at)\b", r"\1", text)


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


PAGE_BYTES = 8000
READ_PAGE = 'Page {n} of {total}. For page {next}, call read with {{"path": {path}, "page": {next}}}'
LOG_PAGE = "Page {n} of {total}. For page {next}, call log with {call}"
REPLY_PAGE = 'Reply saved in full to {file}. Page 1 of {total}. For page 2, call read with {{"path": {path}, "page": 2}}'
LAST_PAGE = "Page {total} of {total}, the last page."
SAVED = " When the reply's text is over 8,000 bytes, the whole text is saved to a file in the studio and this returns its first page."
AGY_DESCRIPTIONS = {
    "read": "Read a file in this studio. PNG files return an image; other files return text. A folder returns the names of what is in it, a folder's ending in /. Text over 8,000 bytes comes back one page at a time, each page ending with one line that gives the page number and the page count.",
    "log": "The painting so far: every chunk that ran, in order (paintings/lua/painting.lua). Text over 8,000 bytes comes back one page at a time, as with read.",
}
AGY_PAGE_PARAMS = {
    "read": "Which page of a long text file to return, counting from 1. Leave it out for page 1.",
    "log": "Which page to return, counting from 1. Leave it out for page 1.",
}


def text_pages(text, limit=PAGE_BYTES):
    pages, page, size = [], [], 0
    for line in text.splitlines(keepends=True):
        while line:
            encoded = line.encode()
            if size + len(encoded) <= limit:
                page.append(line)
                size += len(encoded)
                line = ""
            elif size:
                pages.append("".join(page))
                page, size = [], 0
            else:
                cut = encoded[:limit].decode(errors="ignore")
                pages.append(cut)
                line = line[len(cut):]
    if page or not pages:
        pages.append("".join(page))
    return pages


def page_of(text, page, more, tool, limit=PAGE_BYTES):
    pages = text_pages(text, limit)
    number = 1 if page is None else int(page) if isinstance(page, str) and page.isdigit() else page
    if isinstance(number, float) and number.is_integer():
        number = int(number)
    if isinstance(number, bool) or not isinstance(number, int) or not 1 <= number <= len(pages):
        raise ValueError(f"{tool}: there is no page {page}; pages run from 1 to {len(pages)}")
    if len(pages) == 1:
        return text
    line = more(number, len(pages)) if number < len(pages) else LAST_PAGE.format(n=number, total=len(pages))
    return f"{pages[number - 1]}\n{line}"


def log_find(log, find, whole):
    lines = log.split("\n")
    want = find.lower()
    hits = [i for i, line in enumerate(lines) if want in line.lower()]
    if not hits:
        return f'no line of {whole} holds "{find}"'
    hit = set(hits)
    one = len(hits) == 1
    out = [f'{len(hits)} {"line" if one else "lines"} of {whole} {"holds" if one else "hold"} "{find}"' + (" (the first 100 are shown)" if len(hits) > 100 else "")]
    last = -1
    for i in hits[:100]:
        start = max(i - 2, last + 1)
        if last >= 0 and start > last + 1:
            out.append("--")
        out.extend(f"{k + 1}{':' if k in hit else '-'} {lines[k]}" for k in range(start, min(i + 2, len(lines) - 1) + 1))
        last = max(last, i + 2)
    return "\n".join(out)


def look_args(view):
    if view.get("palette"):
        if any(key != "palette" and value is not None and value is not False and value != "" for key, value in view.items()):
            raise ValueError("look: palette takes no other option")
        return ["--palette"]
    args = []
    if view.get("survey"):
        args.append("--survey")
    for key in ("compare", "ref", "hold", "at", "crop", "mask", "mode", "light"):
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


REPLAY = re.compile(r"rebuilding from the log \((\d+) of (\d+) chunks\)")
# `status` from an easel that is starting, before it has read the log (main.rs `STARTING`)
STARTING = "the easel is starting; retry in a moment"


def log_text(log):
    try:
        # as written: the easel keeps a chunk's own line ends, CRLF included
        with open(log, newline="") as f:
            return f.read()
    except OSError:
        return ""


def kept(log, before, lua):
    """Whether `lua` was kept as a new chunk: the log changed since `before` and its last chunk is
    `lua` as the easel keeps it. A fresh scratch canvas replaces its log, so chunks aren't counted."""
    now = log_text(log)
    return now != before and re.split(r"^--@ chunk \d+\n", now, flags=re.MULTILINE)[-1] == (lua or "").rstrip() + "\n"


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
    def wide(self):
        try:
            wide = False
            for line in (self.studio / "paintings/lua/painting.lua").read_text().splitlines():
                head = line.strip()
                if head.startswith("--@ chunk"):
                    break
                if head == "--@ sketch":
                    return False
                if re.fullmatch(r"--@ width\s+4800", head):
                    wide = True
            return wide
        except FileNotFoundError:
            pass
        try:
            return (self.studio / "bin/width").read_text().strip() == "4800"
        except OSError:
            return False

    def __init__(self, studio):
        self.studio = studio.resolve(strict=True)
        self.lock = asyncio.Lock()
        self.max_images, self.max_chars = image_limits()
        self.page_bytes = None

    async def step(self, args, text, deadline):
        env = {"PATH": "/usr/bin:/bin", "HOME": os.environ.get("HOME", "")}
        if "RAYON_NUM_THREADS" in os.environ:
            env["RAYON_NUM_THREADS"] = os.environ["RAYON_NUM_THREADS"]
        for key in ("EASEL_ASYNC_DISPLAY", "EASEL_DISPLAY_THREADS"):
            if key in os.environ:
                env[key] = os.environ[key]
        binary = self.studio / "bin/easel"
        if (self.studio / "bin/watercolor-studio").is_file():
            binary = self.studio / "bin/watercolor-studio"
            command = {"do": "run", "look": "render", "open": "init"}.get(args[0], args[0])
            tail = ["look.png", "--mode", "color", *args[1:]] if command == "render" else args[1:]
            args = ["--box", "watercolor", command, str(self.studio), *tail]
        proc = await asyncio.create_subprocess_exec(
            str(binary), *args, cwd=self.studio, env=env,
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
            with anyio.CancelScope(shield=True):
                await proc.wait()
            raise
        return proc.returncode, out.decode(errors="replace")

    async def ensure_open(self, deadline, answer_replay=False):
        if (self.studio / "bin/watercolor-studio").is_file():
            return False
        opened = rebuilding = False
        completed, total = -1, None
        progress_deadline = deadline
        args = ["status"]
        while True:
            try:
                code, out = await self.step(args, None, progress_deadline)
            except asyncio.TimeoutError:
                if opened and not rebuilding:
                    # an open that outlasts the tool: the easel answers how far its replay of the log has got
                    try:
                        code, out = await self.step(["status"], None, time.monotonic() + 10)
                    except asyncio.TimeoutError:
                        code, out = 1, ""
                    if code == 0 and REPLAY.fullmatch(out.strip()):
                        raise ValueError(f"the easel is reopening the painting from its log and hasn't finished: {out.strip()}. `status` shows how far it has got.") from None
                raise ValueError("the easel rebuild stalled without progress" if rebuilding else "the easel didn't open in time" if opened else "the easel isn't answering") from None
            progress = REPLAY.fullmatch(out.strip())
            if progress:
                k, n = map(int, progress.groups())
                if code or k > n or k < completed or (total is not None and n != total):
                    raise ValueError(f"the easel returned invalid rebuild progress: {out.strip()}")
                total = n
                if answer_replay:
                    return True
                if not rebuilding or k > completed:
                    completed = k
                    # a chunk may take 40 minutes to replay at 4800 px: the easel's own stall is 50 there
                    progress_deadline = time.monotonic() + (50 if self.wide() else 30) * 60
                rebuilding = True
                if time.monotonic() >= progress_deadline:
                    raise ValueError("the easel rebuild stalled without progress")
                await asyncio.sleep(min(0.1, progress_deadline - time.monotonic()))
                args = ["status"]
                continue
            if code and out.strip() == STARTING:
                # the easel has its socket but hasn't read the log yet: its progress is a moment away
                if time.monotonic() >= progress_deadline:
                    raise ValueError("the easel isn't answering")
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
        wait = (48 if self.wide() else 12) * 60 if args[0] == "do" else 3 * 60
        deadline = time.monotonic() + wait
        if await self.ensure_open(deadline, args[0] == "status"):
            deadline = time.monotonic() + wait
        log = self.studio / "paintings/lua" / ("scratch.lua" if "--scratch" in args else "painting.lua")
        before = log_text(log) if args[0] == "do" else ""
        try:
            code, out = await self.step(args, text, deadline)
        except asyncio.TimeoutError:
            if args[0] != "do" or (self.studio / "bin/watercolor-studio").is_file():
                raise ValueError("the easel didn't answer; `log` shows whether the chunk ran") from None
            # what landed: the chunk still running (its progress), in the log, or neither
            minutes = f"the easel didn't answer within {round(wait / 60)} minutes"
            try:
                code, now = await self.step(["status"], None, time.monotonic() + 10)
            except asyncio.TimeoutError:
                code, now = 1, ""
            now = now.strip()
            if code:
                raise ValueError(f"{minutes}; `log` shows whether the chunk ran") from None
            if now.startswith("painting a chunk"):
                raise ValueError(f"{minutes}. The chunk is still running and isn't in the log yet: {now}. `status` shows how far it has got while it runs; once it ends, `log` shows whether it was kept.") from None
            if kept(log, before, text):
                raise ValueError(f"{minutes}, but the chunk ran: it is in the log.") from None
            raise ValueError(f"{minutes}, and the chunk isn't in the log: it changed nothing.") from None
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
            stable = (source.parent / ".display" / source.name).is_file()
            new = old if stable else old.with_name(f"{uuid4()}.png")
            if not stable:
                source.rename(self.studio / new)
            for folder in (".gallery", ".relief"):
                sibling = source.parent / folder / source.name
                if not stable and sibling.is_file():
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
        if name in ("reference", "reference_list"):
            lookup = reference if name == "reference" else reference_list
            result = lookup(studio_path(self.studio, "notes/reference-library.json"), params)
            return [types.TextContent(type="text", text=json.dumps(result, ensure_ascii=False))]
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
            if view.get("ref"):
                if view.get("survey") or view.get("compare") or view.get("hold"):
                    raise ValueError("look: ref is a look of its own: no survey, compare or hold")
                ref = studio_path(self.studio, view["ref"])
                if ref is None:
                    raise ValueError("look: ref is the path of a picture of the motif in this studio")
                view["ref"] = str(ref)
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
        if name == "probe":
            at, crop = params.get("at"), params.get("crop")
            if bool(at) == bool(crop):
                raise ValueError('probe: give at: "x,y" or crop: "x0,y0,x1,y1", one of the two')
            try:
                said = await self.at(["probe", *on, *(["--at", str(at)] if at else ["--crop", str(crop)])])
            except ValueError as error:
                raise ValueError(tool_words(str(error))) from None
            return [types.TextContent(type="text", text=tag + said)]
        if name == "note":
            said = self.revise_journal(params["replaces"], params["text"]) if "replaces" in params else await self.at(["note", "-"], params["text"])
        elif name == "status":
            said = await self.at(["status", *on])
            said = tag + re.sub(r"^\d+px · ", "", re.sub(r"^\d+ chunks? · ", "", said))
        elif name == "log":
            said = re.sub(r"^--@ chunk \d+[ \t]*$", "--@ chunk", await self.at(["log", *on]), flags=re.MULTILINE)
            units = said.encode("utf-16-le", errors="surrogatepass")
            whole = "paintings/lua/scratch.lua" if scratch else "paintings/lua/painting.lua"
            if params.get("find"):
                said = log_find(said, params["find"], whole)
            elif len(units) > 100_000:
                said = f"(the first {len(units) // 2 - 50_000} characters are left out; read {whole} for all of it)\n" + units[-100_000:].decode("utf-16-le", errors="replace")
            said = tag + said
        else:
            raise ValueError(f"Unknown tool: {name}")
        return [types.TextContent(type="text", text=said)]


def tool_definitions(studio, sittings=False, agy=False):
    definitions = json.loads(Path(__file__).with_name("tools.json").read_text())
    if (studio / "bin/watercolor-studio").is_file():
        definitions = [definition for definition in definitions if definition["name"] != "probe"]
        for definition in definitions:
            if definition["name"] == "paint":
                definition["description"] = "Run a Lua chunk on watercolor paper (notes/watercolor_easel_guide.md). Successful chunks keep the paper, named brushes and clock for the next chunk. A failed chunk keeps the committed state."
            elif definition["name"] == "look":
                definition["description"] = "Look at watercolor paper. The painter sees Color by default; mode can be color, gallery or relief. Gallery uses 115/55/1 light and Relief uses 135/25/1. size sets the long side in pixels, 16..2400. crop is x0,y0,x1,y1 in integer cells and stays at 1:1. Display-only Gallery and Relief companions accompany a Color look."
                definition["inputSchema"]["properties"] = {key: value for key, value in definition["inputSchema"]["properties"].items() if key in ("scratch", "crop", "mode", "size")}
            for key in ("scratch", "new_scratch"):
                if key in definition["inputSchema"]["properties"]:
                    definition["inputSchema"]["properties"][key]["description"] = "true: use separate scratch paper, with the painting's size, kind and seed, its own brushes, clock and log" if key == "scratch" else "with scratch: true, keep the old scratch sheet's log and checkpoint and start a fresh sheet before this chunk"
    if sittings:
        definitions += json.loads(Path(__file__).with_name("sitting-tools.json").read_text())
        next(tool for tool in definitions if tool["name"] == "status")["description"] = "The canvas's setup and the current sitting: its number and, when the sitting is timed, the seconds left. While a chunk is still running, how far it has got; while the easel rebuilds the painting from its log, how many of its chunks are done."
    if agy:
        for definition in definitions:
            name = definition["name"]
            if name in AGY_DESCRIPTIONS:
                definition["description"] = AGY_DESCRIPTIONS[name]
                definition["inputSchema"]["properties"]["page"] = {"type": "integer", "minimum": 1, "description": AGY_PAGE_PARAMS[name]}
            elif name in ("paint", "end_sitting"):
                definition["description"] = definition["description"].rstrip() + SAVED
    return definitions


async def keepalive(context, stopped):
    progress = 0
    while True:
        token = context.meta.progressToken if context.meta else None
        if token is not None:
            await context.session.send_progress_notification(token, progress)
        progress += 1
        try:
            await asyncio.wait_for(stopped.wait(), 15)
            return
        except asyncio.TimeoutError:
            try:
                await context.session.send_request(types.ServerRequest(types.PingRequest()), types.EmptyResult,
                                                   request_read_timeout_seconds=timedelta(seconds=5))
            except Exception:
                return


async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--studio", default=os.environ.get("EASEL_ROOT"))
    parser.add_argument("--sessions-root", type=Path, help="opt in to durable viewer session recording")
    parser.add_argument("--sittings", action="store_true")
    parser.add_argument("--break-seconds", type=float, default=0)
    parser.add_argument("--sitting-seconds", type=float)
    parser.add_argument("--agy", action="store_true", help="agy client: mark every tool alwaysLoad and page text replies over 8,000 bytes")
    args = parser.parse_args()
    if not args.studio:
        parser.error("set --studio or EASEL_ROOT to an exported studio")
    if any(value is not None and (not math.isfinite(value) or value < 0) for value in (args.break_seconds, args.sitting_seconds)):
        parser.error("sitting and break durations must be finite and nonnegative")
    if args.agy and not args.sittings:
        parser.error("--agy goes with --sittings")
    if args.sittings:
        from sittings import SittingEasel
        easel = SittingEasel(Path(args.studio), args.break_seconds, args.sitting_seconds)
    else:
        easel = Easel(Path(args.studio))
    recorded = None
    if args.sessions_root:
        from remote import RemoteStudio
        recorded = RemoteStudio(easel.studio, args.sessions_root, response_wait=None, easel=easel)
    if args.agy:
        easel.page_bytes = PAGE_BYTES
    server = Server("easel")
    definitions = tool_definitions(easel.studio, args.sittings, args.agy)
    meta = {"_meta": {"antigravity/alwaysLoad": True}} if args.agy else {}

    @server.list_tools()
    async def list_tools():
        return [types.Tool(**tool, **meta) for tool in definitions]

    @server.call_tool()
    async def call_tool(name, arguments):
        async with easel.lock:
            stopped = asyncio.Event()
            heartbeat = asyncio.create_task(keepalive(server.request_context, stopped)) if name == "end_sitting" else None
            try:
                if recorded:
                    return await recorded.call(name, dict(arguments, call_id=str(uuid4())), cancel_on_disconnect=True)
                return types.CallToolResult(content=await easel.call(name, arguments))
            except (ValueError, OSError) as error:
                return types.CallToolResult(isError=True, content=[types.TextContent(type="text", text=str(error))])
            finally:
                if heartbeat:
                    stopped.set()
                    with anyio.CancelScope(shield=True):
                        await asyncio.gather(heartbeat, return_exceptions=True)

    try:
        async with stdio_server() as (read, write):
            await server.run(read, write, server.create_initialization_options())
    finally:
        if recorded:
            await recorded.close()


if __name__ == "__main__":
    asyncio.run(main())
