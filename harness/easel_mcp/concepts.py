import os
import tempfile

from mcp import types

from sittings import ReadingEasel


TOOL = {
    "name": "write_concepts",
    "description": "Replace notes/concepts.md with your sixteen numbered pictures and their estimates.",
    "inputSchema": {"type": "object", "properties": {"text": {"type": "string"}}, "required": ["text"], "additionalProperties": False},
}


class ConceptsEasel(ReadingEasel):
    async def call(self, name, params):
        if name == "read":
            return await super().call(name, params)
        if name != "write_concepts":
            raise ValueError(f"Unknown tool: {name}")
        notes = self.studio / "notes"
        with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", prefix=".concepts.md.", suffix=".tmp", dir=notes, delete=False) as staged:
            temporary = staged.name
            try:
                staged.write(params["text"])
                staged.close()
                os.replace(temporary, notes / "concepts.md")
            finally:
                if os.path.exists(temporary):
                    os.unlink(temporary)
        return [types.TextContent(type="text", text="Saved notes/concepts.md.")]
