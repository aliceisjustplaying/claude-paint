# /// script
# requires-python = ">=3.11"
# dependencies = ["mcp==1.30.0", "pillow"]
# ///
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys
import tempfile
import unittest

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

REPO = Path(__file__).resolve().parents[2]
BUILD = runpy.run_path(str(REPO / "scripts/build_reference_library"))["build"]
STRIP = runpy.run_path(str(REPO / "scripts/strip_sources"))["strip"]


class LibraryTests(unittest.IsolatedAsyncioTestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.studio = self.root / "paint-studio-abcd"
        (self.studio / "notes").mkdir(parents=True)
        (self.studio / "bin").mkdir()
        (self.studio / "bin/easel").touch()
        (self.studio / "BRIEF.md").write_text("Test studio")
        manifest = json.loads((REPO / "notes/reference-library/manifest.json").read_text())
        self.entries = BUILD(manifest, lambda path: (REPO / path).read_text(), STRIP)
        (self.studio / "notes/reference-library.json").write_text(json.dumps(self.entries, ensure_ascii=False))

    def test_manifest_drift_fails_closed_and_excludes_source_metadata(self):
        manifest = json.loads((REPO / "notes/reference-library/manifest.json").read_text())
        with self.assertRaisesRegex(ValueError, "source changed"):
            BUILD(manifest, lambda path: (REPO / path).read_text() + "changed", STRIP)
        self.assertTrue(all(set(entry) == {"id", "title", "keywords", "text"} for entry in self.entries))
        self.assertNotIn("read before composing", json.dumps(self.entries))
        subprocess.run([str(REPO / "scripts/check_studio_names"), str(self.studio)], check=True)

    def test_friedrich_preparation_changes_only_reading_and_tools(self):
        prep = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        source = ('TOOLS = "paint,look,note,status,log,read"\n'
                  'READING = {}\nREADING["friedrich"] = "notes/research/friedrich_painter.md; notes/research/trees.md"\n'
                  'OPENING = {"friedrich": ("Paint a landscape. "\n'
                  '                        "Let winter be the season.")}\n'
                  'def prepare(t):\n'
                  '    if t["profile"] == "friedrich":\n'
                  '        copy("friedrich_painter.md")\n'
                  '        copy("trees.md")\n'
                  '    return "unchanged"\n'
                  'def paint():\n    return "normal house sitting loop"\n')
        changed = prep["corrected_runner"](source)
        self.assertIn('def paint():\n    return "normal house sitting loop"', changed)
        self.assertNotIn("friedrich_painter.md", changed)
        self.assertNotIn("trees.md", changed)
        self.assertNotIn("Let winter be the season", changed)
        self.assertIn("original picture", changed)
        self.assertIn("reference", changed)
        self.assertNotIn("reference", source)
        with self.assertRaises(ValueError):
            prep["corrected_runner"](changed)
        house = self.root / "house"
        house.mkdir()
        template = "{OPENING}\n- The folder {STUDIO}: this brief, your notes and the easel.\n{READING}\n"
        (house / "brief_template.md").write_text(template)
        (house / "r21_chains.py").write_text(source)
        for name in ("painting_chunks.py", "defaults_used.py", "log_copy.py", "copied_shapes.py"):
            (house / name).write_text("")
        (self.studio / "BRIEF.md").unlink()
        (self.studio / "bin/box").write_text("friedrich")
        (self.studio / "paintings/lua").mkdir(parents=True)
        (self.studio / "notes/journal.md").write_text("")
        notes = self.root / "studio_notes.md"
        notes.write_text("Neutral material notes.\n")
        runner = self.root / "runner"
        prep["prepare"](self.studio, house, runner, notes)
        expected = template.replace("{OPENING}", prep["OPENING"]).replace("{STUDIO}", str(self.studio)).replace("{READING}", prep["READING"])
        self.assertEqual((self.studio / "BRIEF.md").read_text(), expected)
        self.assertEqual((house / "r21_chains.py").read_text(), source)
        self.assertEqual((runner / "studio_notes.md").read_text(), notes.read_text())
        config = json.loads((runner / "run-config.json").read_text())
        self.assertEqual(config["studio"], str(self.studio))
        self.assertEqual(config["source"], str(REPO))
        self.assertTrue((runner / "run.py").is_file())
        with self.assertRaises(ValueError):
            prep["prepare"](self.studio, house, self.root / "another-runner", notes)

    def test_prepared_entrypoint_uses_supplied_studio_and_current_harness(self):
        prep = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        (self.studio / "BRIEF.md").unlink()
        (self.studio / "bin/box").write_text("friedrich")
        (self.studio / "paintings/lua").mkdir(parents=True)
        (self.studio / "notes/journal.md").write_text("")
        house = REPO / "notes/round29/runner"
        runner = self.root / "runner"
        run = self.root / "prepared-run"
        prep["prepare"](self.studio, house, runner, house / "studio_notes.md", run)
        result = subprocess.check_output([sys.executable, str(runner / "run.py"), "--check"])
        config = json.loads(result)
        self.assertEqual(config["studio"], str(self.studio))
        self.assertEqual(config["run"], str(run))
        self.assertIn(str(REPO / "harness/painter/painter.ts"), config["first_sitting_cmd"])
        self.assertIn("paint,look,probe,note,status,log,read,reference,reference_list", config["first_sitting_cmd"])
        self.assertIn("claude-opus-5-5", config["model"]["args"])
        self.assertIn("xhigh", config["model"]["args"])
        self.assertFalse(run.exists())
        direct = subprocess.run([sys.executable, str(runner / "r21_chains.py")], capture_output=True, text=True)
        self.assertNotEqual(direct.returncode, 0)
        self.assertIn("Use the prepared run.py", direct.stderr)

    def test_r41_house_names_the_open_passages_and_opens_later_sittings_with_them(self):
        prep = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        (self.studio / "BRIEF.md").unlink()
        (self.studio / "bin/box").write_text("friedrich")
        (self.studio / "paintings/lua").mkdir(parents=True)
        (self.studio / "notes/journal.md").write_text("")
        house = self.root / "house-r41"
        house.mkdir()
        for name in ("painting_chunks.py", "defaults_used.py", "log_copy.py", "copied_shapes.py", "studio_notes.md"):
            shutil.copy(REPO / "notes/round29/runner" / name, house / name)
        for name in ("brief_template.md", "r21_chains.py"):
            shutil.copy(REPO / "scripts/tests/fixtures/r41" / name, house / name)
        runner = self.root / "runner"
        prep["prepare"](self.studio, house, runner, house / "studio_notes.md", self.root / "prepared-run", critic=True)
        config = json.loads(subprocess.check_output([sys.executable, str(runner / "run.py"), "--check"]))
        self.assertIn("`Open passages:`", config["first_sitting_cmd"][-1])
        self.assertEqual(config["later_sitting_env"]["PAINTER_OPEN_PASSAGES"], "1")
        self.assertEqual(config["later_sitting_env"]["PAINTER_SITTING_RECOVERY"], "1")
        self.assertNotIn("PAINTER_OPEN_PASSAGES", config["first_sitting_env"])
        brief = (self.studio / "BRIEF.md").read_text()
        for words in (brief, (runner / "r21_chains.py").read_text()):
            self.assertNotIn("heart", words)
            self.assertNotIn("Before adding repeated small details", words)
        self.assertIn("`Open passages:`", brief)
        self.assertIn("particular detail", brief)

    async def test_exported_agy_discovers_and_retrieves_every_page(self):
        prepare = runpy.run_path(str(REPO / "scripts/prepare_agy_studio"))["prepare"]
        config = prepare(self.studio, self.root / "sessions")
        runtime = self.studio / "bin/mcp"
        self.assertTrue((runtime / "references.py").is_file())
        server = StdioServerParameters(command=sys.executable, args=[str(runtime / "server.py"), "--studio", str(self.studio), "--sittings", "--agy"], env=dict(os.environ))
        async with stdio_client(server) as (read, write), ClientSession(read, write) as session:
            await session.initialize()
            definitions = await session.list_tools()
            definition = next(tool for tool in definitions.tools if tool.name == "reference")
            self.assertTrue(definition.meta["antigravity/alwaysLoad"])
            self.assertFalse(any(word in definition.description.lower() for word in ("tree", "oak", "friedrich")))
            listing = next(tool for tool in definitions.tools if tool.name == "reference_list")
            self.assertTrue(listing.meta["antigravity/alwaysLoad"])
            self.assertFalse(any(word in listing.description.lower() for word in ("tree", "oak", "friedrich")))
            listed = await session.call_tool("reference_list", {})
            self.assertFalse(listed.isError)
            self.assertLess(len(listed.content[0].text.encode()), 8000)
            self.assertEqual(json.loads(listed.content[0].text), {"references": [{"id": entry["id"], "title": entry["title"]} for entry in self.entries]})
            async def call(params):
                reply = await session.call_tool("reference", params)
                self.assertFalse(reply.isError)
                self.assertLess(len(reply.content[0].text.encode()), 8000)
                return json.loads(reply.content[0].text)
            for query, expected in (("branch taper", "tree-structure"), ("wet sand", "water"), ("fruit cloth", "fruit-cloth"), ("fluorescent", "artificial-light"), ("glazing dry", "oil-glazes")):
                result = await call({"query": query})
                self.assertIn(expected, [match["id"] for match in result["matches"]])
            self.assertEqual(await call({"query": "unfindable-qzjx"}), {"matches": [], "message": "No matching references."})
            for entry in self.entries:
                chunks, number = [], 1
                while number is not None:
                    page = await call({"id": entry["id"], "page": number})
                    chunks.append(page["text"])
                    number = page["next_page"]
                self.assertEqual("".join(chunks), entry["text"])
            self.assertIn("--agy", config["mcpServers"]["easel"]["args"])


if __name__ == "__main__":
    unittest.main()
