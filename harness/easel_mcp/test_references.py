# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0", "PyYAML==6.0.3", "Pillow==12.3.0"]
# ///

import importlib.machinery
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import AsyncMock, patch

import yaml

from server import Easel, tool_definitions


class ReferenceTests(unittest.IsolatedAsyncioTestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="reference-proof-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.studio = self.root / "paint-studio-abcd"
        (self.studio / "notes").mkdir(parents=True)
        self.library = self.studio / "notes/reference-library.json"
        self.entries = [
            {"id": "tree", "title": "Tree structure", "keywords": ["Branches", "edges"],
             "text": "Oak limbs overlap.\r\nKeep the gaps open.\n"},
            {"id": "water", "title": "Water study", "keywords": ["Surface", "edges"],
             "text": "Reflections follow ripples, not tree limbs.\n"},
            {"id": "sky", "title": "Sky study", "keywords": ["atmosphere"],
             "text": "Cloud shadows soften at a distance.\n"},
        ]
        self.save()
        self.easel = Easel(self.studio)
        self.easel.at = AsyncMock(side_effect=AssertionError("reference must not call the easel"))

    def save(self):
        self.library.write_text(json.dumps(self.entries, ensure_ascii=False), encoding="utf-8")

    async def call(self, params, easel=None):
        content = await (easel or self.easel).call("reference", params)
        self.assertEqual(len(content), 1)
        self.assertEqual(content[0].type, "text")
        return json.loads(content[0].text)

    async def test_search_matches_all_words_across_fields_in_index_order(self):
        self.assertEqual(await self.call({"query": "  TREE\t EDGES\n"}), {
            "matches": [{"id": "tree", "title": "Tree structure"}, {"id": "water", "title": "Water study"}]})
        self.assertEqual(await self.call({"query": "BRANCH oak"}), {
            "matches": [{"id": "tree", "title": "Tree structure"}]})
        self.assertEqual(await self.call({"query": "SURFACE ripple"}), {
            "matches": [{"id": "water", "title": "Water study"}]})
        self.assertEqual(await self.call({"query": "cloud"}), {
            "matches": [{"id": "sky", "title": "Sky study"}]})

    async def test_no_match_has_no_fallback(self):
        self.assertEqual(await self.call({"query": "oak ripples"}), {
            "matches": [], "message": "No matching references."})

    async def test_list_is_explicit_and_contains_only_catalog_fields(self):
        result = await self.easel.call("reference_list", {})
        self.assertEqual(json.loads(result[0].text), {"references": [{"id": entry["id"], "title": entry["title"]} for entry in self.entries]})
        with self.assertRaises(ValueError):
            await self.easel.call("reference_list", {"query": "tree"})
        self.library.unlink()
        with self.assertRaises(ValueError):
            await self.easel.call("reference_list", {})

    async def test_read_exact_id_and_short_document(self):
        self.assertEqual(await self.call({"id": "tree"}), {
            "id": "tree", "title": "Tree structure", "page": 1, "pages": 1,
            "next_page": None, "text": self.entries[0]["text"]})
        self.assertEqual(await self.call({"id": "tree", "page": 1.0}), await self.call({"id": "tree", "page": 1}))
        for identifier in ("TREE", " tree", "Tree structure", "../outside.txt", "/etc/passwd"):
            with self.subTest(id=identifier), self.assertRaises(ValueError):
                await self.call({"id": identifier})
        self.entries[0]["id"] = "../outside.txt"
        self.save()
        self.assertEqual((await self.call({"id": "../outside.txt"}))["text"], self.entries[0]["text"])

    async def test_all_pages_preserve_full_text_and_unicode_boundaries(self):
        samples = ["", "a" * 4000, "a" * 4001, "a" * 3999 + "😀é\r\n",
                   "汉😀é\u0301\u2003\t\r\n\0\\\"" * 1700 + "no final newline"]
        for text in samples:
            with self.subTest(length=len(text)):
                self.entries[0]["text"] = text
                self.save()
                parts, number = [], 1
                while number is not None:
                    page = await self.call({"id": "tree", "page": number})
                    self.assertEqual(page["page"], number)
                    self.assertLessEqual(len(page["text"].encode("utf-8")), 4000)
                    self.assertEqual(page["next_page"], number + 1 if number < page["pages"] else None)
                    parts.append(page["text"])
                    number = page["next_page"]
                self.assertEqual("".join(parts), text)
                self.assertEqual(len(parts), page["pages"])
                with self.assertRaises(ValueError):
                    await self.call({"id": "tree", "page": len(parts) + 1})
        self.entries[0]["text"] = "a" * 3999 + "😀é"
        self.save()
        self.assertEqual((await self.call({"id": "tree"}))["text"], "a" * 3999)
        self.assertEqual((await self.call({"id": "tree", "page": 2}))["text"], "😀é")

    async def test_response_uses_unescaped_unicode(self):
        self.entries[0]["text"] = "é😀"
        self.save()
        content = await self.easel.call("reference", {"id": "tree"})
        self.assertIn("é😀", content[0].text)
        self.assertNotIn("\\u", content[0].text)

    async def test_invalid_parameters_raise_value_error(self):
        cases = [{}, {"query": "tree", "id": "tree"}, {"query": "tree", "page": 1},
                 {"query": "tree", "page": None}, {"page": 1}, {"path": "../outside.txt"},
                 {"id": "tree", "extra": True}]
        cases += [{"query": value} for value in ("", " \n\t\u2003", None, True, 4, [], {})]
        cases += [{"id": value} for value in (None, True, 4, [], {})]
        cases += [{"id": "tree", "page": value} for value in (None, True, False, 0, -1, 1.5, float("inf"), float("nan"), "1", [], {})]
        for params in cases:
            with self.subTest(params=params), self.assertRaises(ValueError):
                await self.call(params)

    async def test_duplicate_ids_are_rejected_before_any_entry_is_read(self):
        self.entries[1]["id"] = "tree"
        self.save()
        with self.assertRaisesRegex(ValueError, 'duplicate id "tree"'):
            await self.easel.call("reference_list", {})
        with self.assertRaisesRegex(ValueError, 'duplicate id "tree"'):
            await self.call({"query": "ripples"})
        with self.assertRaisesRegex(ValueError, 'duplicate id "tree"'):
            await self.call({"id": "tree"})

    async def test_missing_library_is_clear_and_no_other_notes_are_read(self):
        self.library.unlink()
        (self.studio / "notes/other.json").write_text(json.dumps(self.entries))
        for params in ({"query": "tree"}, {"id": "tree"}):
            with self.subTest(params=params), self.assertRaisesRegex(ValueError, "missing notes/reference-library.json"):
                await self.call(params)

    async def test_library_file_and_directory_symlink_escapes_are_rejected(self):
        outside = self.root / "outside.json"
        outside.write_text(self.library.read_text())
        self.library.unlink()
        self.library.symlink_to(outside)
        with self.assertRaisesRegex(ValueError, "inside this studio"):
            await self.call({"query": "tree"})
        self.library.unlink()
        self.library.parent.rmdir()
        outside_notes = self.root / "outside-notes"
        outside_notes.mkdir()
        (outside_notes / "reference-library.json").write_text(outside.read_text())
        self.library.parent.symlink_to(outside_notes, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "inside this studio"):
            await self.call({"id": "tree"})

    async def test_invalid_library_raises_value_error(self):
        for value in (None, {}, [None], [{"id": "a"}], [dict(self.entries[0], keywords="tree")],
                      [dict(self.entries[0], keywords=[1])], [dict(self.entries[0], text=4)]):
            self.library.write_text(json.dumps(value))
            with self.subTest(value=value), self.assertRaises(ValueError):
                await self.call({"query": "tree"})
        self.library.write_text("not json")
        with self.assertRaises(ValueError):
            await self.call({"id": "tree"})

    async def test_agy_packaging_and_schema(self):
        repo = Path(__file__).resolve().parents[2]
        (self.studio / "bin").mkdir()
        (self.studio / "bin/easel").touch()
        (self.studio / "BRIEF.md").write_text("unchanged\n")
        loader = importlib.machinery.SourceFileLoader("prepare_reference_agy", str(repo / "scripts/prepare_agy_studio"))
        module = importlib.util.module_from_spec(importlib.util.spec_from_loader(loader.name, loader))
        loader.exec_module(module)
        with patch.object(module.shutil, "which", return_value="/usr/bin/uv"):
            module.prepare(self.studio, self.root / "sessions")
        for filename in ("server.py", "references.py", "tools.json"):
            self.assertEqual((self.studio / "bin/mcp" / filename).read_bytes(),
                             (repo / "harness/easel_mcp" / filename).read_bytes())
        frontmatter = (self.studio / ".agents/agents/easel-painter.md").read_text().split("---", 2)[1]
        agent = yaml.safe_load(frontmatter)
        self.assertEqual(agent["tools"], [])
        self.assertTrue(agent["inheritCustomizations"])
        expected = {
            "name": "reference",
            "description": "Search the studio's reference library after choosing a subject or technical question. Give query to find notes, then id to read a result. Text comes in pages; follow next_page until it is null.",
            "inputSchema": {"type": "object", "properties": {
                "query": {"type": "string", "description": "Words to find in the reference text."},
                "id": {"type": "string", "description": "The id of a search result to read."},
                "page": {"type": "integer", "minimum": 1, "description": "The page to read, starting at 1."}},
                "required": [], "additionalProperties": False}}
        for agy in (False, True):
            definitions = tool_definitions(self.studio, sittings=agy, agy=agy)
            self.assertEqual(next(tool for tool in definitions if tool["name"] == "reference"), expected)
        from sittings import SittingEasel
        sitting = SittingEasel(self.studio)
        self.assertEqual(await self.call({"query": "tree"}, sitting), await self.call({"query": "tree"}))


if __name__ == "__main__":
    unittest.main()
