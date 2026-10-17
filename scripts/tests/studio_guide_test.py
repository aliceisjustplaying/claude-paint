import ast
import json
from pathlib import Path
import re
import runpy
import unittest

from scripts.tests import export_materials_test as materials


REPO = Path(__file__).resolve().parents[2]
REWRITE = runpy.run_path(str(REPO / "scripts/studio_guide.py"))["rewrite"]
OIL = (REPO / "notes/easel_guide.md").read_text()
WATERCOLOR = (REPO / "notes/watercolor_easel_guide.md").read_text()


def tools_in(text):
    table = re.search(r"^\| tool \| what it does \|\n(?:\|[^\n]*\n?)+", text, re.MULTILINE)[0]
    return re.findall(r"^\| `([^`]+)` \|", table, re.MULTILINE)


class StudioGuideTests(unittest.TestCase):
    def test_pi_table_matches_prepared_runner_with_and_without_critic(self):
        prep = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        for critic in (False, True):
            with self.subTest(critic=critic):
                text = REWRITE(OIL, reference_library=True, critic=critic)
                self.assertEqual(tools_in(text), prep["TOOLS"].split(",") + (["ask_critic"] if critic else []))
                self.assertNotIn('| `ref: ', text)
                self.assertEqual(REWRITE(text, reference_library=True, critic=critic), text)
                for name in ("paint", "look", "probe", "note", "status", "log"):
                    row = next(line for line in OIL.splitlines() if line.startswith(f"| `{name}` |"))
                    self.assertIn(row, text)

    def test_no_library_does_not_advertise_reference_tools(self):
        text = REWRITE(OIL)
        self.assertEqual(tools_in(text), ["paint", "look", "probe", "note", "status", "log", "read"])
        self.assertNotIn("| `ref: ", text)
        self.assertIn('| `compare: ', text)

    def test_watercolor_lists_exactly_its_mcp_tools(self):
        text = REWRITE(WATERCOLOR, watercolor=True, critic=True)
        opening = text.splitlines()[2]
        expected = [tool["name"] for tool in json.loads((REPO / "harness/easel_mcp/tools.json").read_text()) if tool["name"] != "probe"]
        self.assertEqual(set(re.findall(r"`([^`]+)`", opening)), set(expected))
        self.assertIn("seven MCP tools", opening)
        self.assertNotIn("`probe`", text)
        self.assertNotIn("`ask_critic`", text)
        self.assertEqual(REWRITE(text, watercolor=True), text)
        self.assertEqual(text.split("`paint` runs a chunk", 1)[1], WATERCOLOR.split("`paint` runs a chunk", 1)[1])

    def test_shell_only_historical_guide_keeps_its_text(self):
        text = "# The easel\n\nbin/easel open\n"
        self.assertEqual(REWRITE(text), text)

    def test_older_pi_guide_does_not_gain_an_unsupported_probe(self):
        text = "\n".join(line for line in OIL.splitlines() if not line.startswith("| `probe` |")) + "\n"
        self.assertEqual(tools_in(REWRITE(text)), ["paint", "look", "note", "status", "log", "read"])

    def test_duplicate_tool_tables_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "one tool table"):
            REWRITE(OIL + OIL)

    def test_live_templates_have_probe_and_reference_tools(self):
        paths = [REPO / f"notes/round{number}/runner/r21_chains.py" for number in range(24, 30)]
        paths.append(REPO / "archive/collection/runners/gallery-fcf9c110/r41/r21_chains.py")
        for path in paths:
            with self.subTest(path=path):
                values = [ast.literal_eval(node.value) for node in ast.parse(path.read_text()).body
                          if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "TOOLS" for target in node.targets)]
                self.assertEqual(values, ["paint,look,probe,note,status,log,read,reference,reference_list"])
        bot = (REPO / "harness/delve-bot/src/painter.ts").read_text()
        self.assertIn('const TOOLS = "paint,look,probe,note,status,log,read,reference,reference_list";', bot)

    def test_atelier_table_matches_its_overridden_tool_list(self):
        source = ast.parse((REPO / "atelier/run.py").read_text())
        tools = next(ast.literal_eval(node.value) for node in source.body
                     if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "TOOLS" for target in node.targets))
        self.assertEqual(tools_in(REWRITE(OIL, notebook=True)), tools.split(","))

    def test_export_rewrites_the_guide(self):
        fixture = materials.ExportMaterialsTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        guide = fixture.root / "revisions/HEAD/notes/easel_guide.md"
        guide.write_text(guide.read_text() + '\n| `ref: "motif.png"` | fixture reference image |\n')
        dest, result = fixture.export()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        text = (dest / "notes/easel_guide.md").read_text()
        self.assertEqual(tools_in(text), ["paint", "look", "probe", "note", "status", "log", "read"])
        self.assertNotIn("motif.png", text)

    def test_watercolor_export_rewrites_both_guides(self):
        fixture = materials.ExportMaterialsTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        revision = fixture.root / "revisions/HEAD"
        fixture.write(revision / "notes/watercolor_easel_guide.md", WATERCOLOR)
        for name in ("notes/watercolor_techniques.md", "notes/research/watercolor_physics.md",
                     "experiments/watercolor/THIRD_PARTY_NOTICES.md"):
            fixture.write(revision / name, "Fixture watercolor notes.\n")
        for name in ("server.py", "tools.json", "remote.py", "http_server.py"):
            fixture.write(revision / "harness/easel_mcp" / name, "{}\n")
        fixture.executable(fixture.root / "builds/watercolor-fixture-tree/target/release/watercolor-studio", "")
        dest, result = fixture.export(profile="watercolor")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        text = (dest / "notes/easel_guide.md").read_text()
        self.assertEqual(text, REWRITE(WATERCOLOR, watercolor=True))
        self.assertEqual(text, (dest / "notes/watercolor_easel_guide.md").read_text())


if __name__ == "__main__":
    unittest.main()
