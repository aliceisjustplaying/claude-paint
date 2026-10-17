import asyncio
import base64
import io
import json
import math
from pathlib import Path
import re
import time
from uuid import uuid4

from mcp import types

from painting_chunks import WATERCOLOR_METHODS, WATERCOLOR_VERBS, count_painting_chunks
from server import LOG_PAGE, LOG_SCRATCH_PAGE, READ_PAGE, REPLY_PAGE, Easel, page_of, studio_path, text_pages


def pi_image(data, max_side=2000, max_bytes=4.5 * 1024 * 1024, quality=80):
    """A PNG as pi's read tool gives it to the house painter (pi-coding-agent utils/image-resize-core.js):
    within 2000x2000 and under 4.5 MiB of base64 it goes as it is; otherwise it is scaled to fit and sent as
    PNG or JPEG (quality 80, then 85, 70, 55, 40), the first under the limit, shrinking by 0.75 until one is."""
    from PIL import Image, ImageOps
    opened = Image.open(io.BytesIO(data))
    image = ImageOps.exif_transpose(opened)
    original = image.size
    if max(original) <= max_side and math.ceil(len(data) / 3) * 4 < max_bytes:
        mime = Image.MIME.get(opened.format, "image/png")
        return [types.TextContent(type="text", text=f"Read image file [{mime}]"),
                types.ImageContent(type="image", mimeType=mime, data=base64.b64encode(data).decode())]
    width, height = original
    if width > max_side:
        width, height = max_side, max(1, math.floor(height * max_side / width + 0.5))
    if height > max_side:
        width, height = max(1, math.floor(width * max_side / height + 0.5)), max_side
    while True:
        resized = image.resize((width, height), Image.LANCZOS)
        for quality_step in [None, *dict.fromkeys((quality, 85, 70, 55, 40))]:
            out = io.BytesIO()
            if quality_step is None:
                resized.save(out, "PNG")
            else:
                resized.convert("RGB").save(out, "JPEG", quality=quality_step)
            encoded = base64.b64encode(out.getvalue()).decode()
            if len(encoded) < max_bytes:
                mime = "image/png" if quality_step is None else "image/jpeg"
                note = (f"Read image file [{mime}]\n[Image: original {original[0]}x{original[1]}, displayed at {width}x{height}. "
                        f"Multiply coordinates by {original[0] / width:.2f} to map to original image.]")
                return [types.TextContent(type="text", text=note), types.ImageContent(type="image", mimeType=mime, data=encoded)]
        if (width, height) == (1, 1):
            raise ValueError("read: this image can't be made small enough to send")
        width, height = max(1, math.floor(width * 0.75)), max(1, math.floor(height * 0.75))


class SittingEasel(Easel):
    def __init__(self, studio, break_seconds=0, sitting_seconds=None):
        super().__init__(studio)
        self.watercolor = (self.studio / "bin/watercolor-studio").is_file()
        self.break_seconds = break_seconds
        self.sitting_seconds = sitting_seconds
        self.state_path = self.studio / "out/easel/sittings.json"
        self.state = json.loads(self.state_path.read_text()) if self.state_path.exists() else {
            "sitting": 1, "started": time.time(), "painting_before": 0 if self.watercolor else self.painting_count(),
            "whole": False, "detail": False, "due": None, "complete": False,
        }
        self.save()

    def painting_count(self):
        if self.watercolor:
            return self.state.get("marks", 0)
        path = self.studio / "paintings/lua/painting.lua"
        return count_painting_chunks(path.read_text()) if path.exists() else 0

    def save(self):
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        tmp = self.state_path.with_suffix(".tmp")
        tmp.write_text(json.dumps(self.state) + "\n")
        tmp.replace(self.state_path)

    async def call(self, name, params):
        if name == "read":
            path = studio_path(self.studio, params["path"])
            if path is not None and path.is_dir():
                return [types.TextContent(type="text", text="\n".join(sorted(entry.name + ("/" if entry.is_dir() else "") for entry in path.iterdir())) or "empty folder")]
            if path is None or not path.is_file():
                raise ValueError("read: the path must be a file or a folder in this studio")
            if path.suffix.lower() == ".png":
                if self.page_bytes:
                    return pi_image(path.read_bytes())
                return [types.ImageContent(type="image", mimeType="image/png", data=base64.b64encode(path.read_bytes()).decode())]
            text = path.read_text()
            if self.page_bytes:
                shown = str(path.relative_to(self.studio))
                text = page_of(text, params.get("page"), lambda n, total: READ_PAGE.format(n=n, total=total, path=json.dumps(shown), next=n + 1), "read", self.page_bytes)
            return [types.TextContent(type="text", text=text)]
        if name == "end_sitting":
            if self.state["complete"]:
                return self.completion()
            if self.state["due"] is None:
                if self.painting_count() == self.state["painting_before"] and self.state["whole"] and self.state["detail"]:
                    self.state["complete"] = True
                    self.save()
                    return self.completion()
                self.state["due"] = time.time() + self.break_seconds
                self.save()
            await asyncio.sleep(max(0, self.state["due"] - time.time()))
            said = await self.at(["status"]) if self.watercolor else (await super().call("paint", {"lua": "print(wait(0))"}))[0].text
            clock = re.findall(r"^day \d+, \d\d:\d\d$", said, re.MULTILINE)[-1]
            look = await super().call("look", {})
            self.state.update(sitting=self.state["sitting"] + 1, started=time.time(),
                              painting_before=self.painting_count(), whole=False, detail=False, due=None)
            self.save()
            journal = self.journal()
            message = Path(__file__).with_name("sitting-message.txt").read_text().strip()
            text = f"{message}\n\nSitting {self.state['sitting']}\nPainting clock: {clock}\n\nYour journal (notes/journal.md):\n{journal}"
            return [types.TextContent(type="text", text=self.reply(text)), *look]
        if name == "paint" and (self.state["due"] is not None or self.state["complete"]):
            raise ValueError("paint: the painting is complete" if self.state["complete"] else "paint: this sitting has ended; end_sitting starts the next one")
        before = self.painting_count() if name == "paint" and not params.get("scratch") else None
        result = await super().call(name, params)
        if before is not None and self.watercolor and count_painting_chunks("--@ chunk 1\n" + params["lua"], WATERCOLOR_VERBS, WATERCOLOR_METHODS):
            self.state["marks"] = before + 1
        if before is not None and self.painting_count() > before:
            self.state.update(whole=False, detail=False)
            self.save()
        if name == "look" and not any(params.get(key) for key in ("scratch", "palette")) and params.get("mode") in (None, "", "color"):
            if any(item.type == "image" for item in result):
                self.state["detail" if params.get("crop") else "whole"] = True
                self.save()
        if self.page_bytes and name == "paint":
            result[0] = types.TextContent(type="text", text=self.reply(result[0].text))
        if self.page_bytes and name == "log":
            more = LOG_SCRATCH_PAGE if params.get("scratch") else LOG_PAGE
            result[0] = types.TextContent(type="text", text=page_of(result[0].text, params.get("page"), lambda n, total: more.format(n=n, total=total, next=n + 1), "log", self.page_bytes))
        if name == "status":
            remaining = "untimed" if self.sitting_seconds is None else f"{max(0, self.sitting_seconds - (time.time() - self.state['started'])):.0f} of {self.sitting_seconds:.0f} seconds left"
            result.append(types.TextContent(type="text", text=f"Sitting {self.state['sitting']}, {remaining}"))
        return result

    def reply(self, text):
        if not self.page_bytes or len(text.encode()) <= self.page_bytes:
            return text
        path = self.studio / "out/easel/replies" / f"{uuid4().hex[:12]}.txt"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
        pages = text_pages(text, self.page_bytes)
        shown = str(path.relative_to(self.studio))
        return f"{pages[0]}\n" + REPLY_PAGE.format(file=shown, total=len(pages), path=json.dumps(shown))

    def journal(self):
        path = self.studio / "notes/journal.md"
        return path.read_text() if path.exists() else ""

    def completion(self):
        return [types.TextContent(type="text", text=self.reply(f"The painting is complete. Sitting {self.state['sitting']} added no paint and had a whole-canvas look and a detail crop in normal color.\n\nYour journal (notes/journal.md):\n{self.journal()}"))]
