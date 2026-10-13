import asyncio
import base64
import json
from pathlib import Path
import re
import time

from mcp import types

from painting_chunks import count_painting_chunks
from server import Easel, studio_path


class SittingEasel(Easel):
    def __init__(self, studio, break_seconds=0, sitting_seconds=None):
        super().__init__(studio)
        self.break_seconds = break_seconds
        self.sitting_seconds = sitting_seconds
        self.state_path = self.studio / "out/easel/sittings.json"
        self.state = json.loads(self.state_path.read_text()) if self.state_path.exists() else {
            "sitting": 1, "started": time.time(), "painting_before": self.painting_count(),
            "whole": False, "detail": False, "due": None, "complete": False,
        }
        self.save()

    def painting_count(self):
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
            if path is None or not path.is_file():
                raise ValueError("read: the path must be a file in this studio")
            if path.suffix.lower() == ".png":
                return [types.ImageContent(type="image", mimeType="image/png", data=base64.b64encode(path.read_bytes()).decode())]
            return [types.TextContent(type="text", text=path.read_text())]
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
            clock_result = await super().call("paint", {"lua": "print(wait(0))"})
            clock = re.findall(r"^day \d+, \d\d:\d\d$", clock_result[0].text, re.MULTILINE)[-1]
            look = await super().call("look", {})
            self.state.update(sitting=self.state["sitting"] + 1, started=time.time(),
                              painting_before=self.painting_count(), whole=False, detail=False, due=None)
            self.save()
            journal = self.journal()
            text = f"Sitting {self.state['sitting']}\nClock: {clock}\n\nYour journal handoff:\n{journal}"
            return [types.TextContent(type="text", text=text), *look]
        if name == "paint" and (self.state["due"] is not None or self.state["complete"]):
            raise ValueError("paint: this sitting has ended")
        before = self.painting_count() if name == "paint" and not params.get("scratch") else None
        result = await super().call(name, params)
        if before is not None and self.painting_count() > before:
            self.state.update(whole=False, detail=False)
            self.save()
        if name == "look" and not any(params.get(key) for key in ("scratch", "palette", "mode", "hold", "compare")):
            if any(item.type == "image" for item in result):
                self.state["detail" if params.get("crop") else "whole"] = True
                self.save()
        if name == "status":
            remaining = "unlimited" if self.sitting_seconds is None else f"{max(0, self.sitting_seconds - (time.time() - self.state['started'])):.0f} seconds"
            result.append(types.TextContent(type="text", text=f"Sitting {self.state['sitting']}: {remaining} remaining"))
        return result

    def journal(self):
        path = self.studio / "notes/journal.md"
        return path.read_text() if path.exists() else ""

    def completion(self):
        return [types.TextContent(type="text", text=f"Painting complete. Sitting {self.state['sitting']} added no paint and included successful whole-canvas and detail looks in normal color.\n\nYour journal handoff:\n{self.journal()}")]
