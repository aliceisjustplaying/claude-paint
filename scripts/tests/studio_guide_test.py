import ast
import json
from pathlib import Path
import re
import runpy
import subprocess
import unittest

from scripts.tests import export_materials_test as materials


REPO = Path(__file__).resolve().parents[2]
REWRITE = runpy.run_path(str(REPO / "scripts/studio_guide.py"))["rewrite"]
OIL = (REPO / "notes/easel_guide.md").read_text()
WATERCOLOR = (REPO / "notes/watercolor_easel_guide.md").read_text()
BASE = ["paint", "look", "probe", "note", "status", "log"]
LIBRARY = ["read", "reference", "reference_list"]


def tools_in(text):
    table = re.search(r"^\| tool \| what it does \|\n(?:\|[^\n]*\n?)+", text, re.MULTILINE)[0]
    return re.findall(r"^\| `([^`]+)` \|", table, re.MULTILINE)


class StudioGuideTests(unittest.TestCase):
    def fixture(self, library=False):
        fixture = materials.ExportMaterialsTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        guide = re.sub(r"^\| tube \| pigment \|[^\n]*\n(?:\|[^\n]*\n?)+", materials.TABLE, OIL, flags=re.MULTILINE)
        fixture.write(fixture.root / "revisions/HEAD/notes/easel_guide.md", guide)
        if library:
            fixture.write(fixture.root / "revisions/HEAD/notes/reference-library/manifest.json", "[]\n")
            fixture.executable(fixture.repo / "scripts/build_reference_library",
                               'from pathlib import Path\nimport sys\nPath(sys.argv[-1]).write_text("[]\\n")\n')
        return fixture

    def test_oil_tool_rows_follow_library_and_critic(self):
        for library in (False, True):
            for critic in (False, True):
                with self.subTest(library=library, critic=critic):
                    text = REWRITE(OIL, reference_library=library, critic=critic)
                    self.assertEqual(tools_in(text), BASE + (LIBRARY if library else []) + (["ask_critic"] if critic else []))
                    self.assertEqual(REWRITE(text, reference_library=library, critic=critic), text)
                    self.assertNotIn('| `ref: ', text)
                    self.assertIn('| `compare: ', text)
                    self.assertNotIn('`read` reads the files', text)
                    for name in BASE:
                        row = next(line for line in OIL.splitlines() if line.startswith(f"| `{name}` |"))
                        self.assertIn(row, text)
                    self.assertEqual(text.split("## The canvas", 1)[1],
                                     re.sub(r"^\| `ref: [^\n]*\n?", "", OIL.split("## The canvas", 1)[1], flags=re.MULTILINE))

    def test_switches_remove_rows_from_an_already_rewritten_guide(self):
        text = REWRITE(OIL, reference_library=True, critic=True)
        self.assertEqual(REWRITE(text), REWRITE(OIL))

    def test_current_template_has_exact_allow_list(self):
        path = REPO / "archive/collection/runners/gallery-fcf9c110/r41/r21_chains.py"
        values = [ast.literal_eval(node.value) for node in ast.parse(path.read_text()).body
                  if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "TOOLS" for target in node.targets)]
        self.assertEqual(values, [",".join(BASE + LIBRARY + ["ask_critic"])])

    def test_historical_guides_keep_their_supported_tools(self):
        shell = "# The easel\n\nbin/easel open\n"
        self.assertEqual(REWRITE(shell), shell)
        older = re.sub(r"^\| `probe` \|[^\n]*\n", "", OIL, flags=re.MULTILINE)
        self.assertEqual(tools_in(REWRITE(older)), [name for name in BASE if name != "probe"])

    def test_duplicate_tool_tables_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "one tool table"):
            REWRITE(OIL + OIL)

    def test_watercolor_names_exactly_its_seven_mcp_tools(self):
        text = REWRITE(WATERCOLOR, watercolor=True)
        expected = [tool["name"] for tool in json.loads((REPO / "harness/easel_mcp/tools.json").read_text()) if tool["name"] != "probe"]
        self.assertEqual(set(re.findall(r"`([^`]+)`", text.splitlines()[2])), set(expected))
        self.assertIn("seven MCP tools", text)
        self.assertNotIn("`probe`", text)
        self.assertNotIn("`ask_critic`", text)
        self.assertEqual(REWRITE(text, watercolor=True), text)
        self.assertEqual(text.split("`paint` runs a chunk", 1)[1], WATERCOLOR.split("`paint` runs a chunk", 1)[1])

    def test_exported_oil_guide_with_and_without_library(self):
        for library in (False, True):
            with self.subTest(library=library):
                fixture = self.fixture(library)
                dest, result = fixture.export()
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                text = (dest / "notes/easel_guide.md").read_text()
                self.assertEqual(tools_in(text), BASE + (LIBRARY if library else []))
                self.assertNotIn("| `ref: ", text)
                self.assertEqual((dest / "notes/reference-library.json").exists(), library)

    def test_blank_export_checks_tube_parity_before_tool_rewrite(self):
        fixture = self.fixture()
        built = next((fixture.root / "builds").glob("code-*/built/tonn/easel"))
        (built.parent.parent / "default").mkdir()
        fixture.executable(built.parent.parent / "default/easel",
                           f"TABLE = {materials.TABLE!r}\n" + materials.EASEL.replace(
                               'box = (studio / "bin/box").read_text().strip()', 'box = ""'))
        dest, result = fixture.export(profile="blank")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(tools_in((dest / "notes/easel_guide.md").read_text()), BASE)

    def test_profile_check_accepts_only_the_export_rewrite(self):
        fixture = self.fixture(library=True)
        result = subprocess.run([str(fixture.repo / "scripts/tests/export_profiles.sh"), "tonn"],
                                env=dict(fixture.env, R16_BRANCH="HEAD"), capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_friedrich_preparation_adds_critic_row_only_with_switch(self):
        prep = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        for critic in (False, True):
            with self.subTest(critic=critic):
                fixture = self.fixture(library=True)
                studio, result = fixture.export()
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                (studio / "bin/box").write_text("friedrich\n")
                house = REPO / "notes/round29/runner"
                prep["prepare"](studio, house, fixture.root / "runner", house / "studio_notes.md", critic=critic)
                text = (studio / "notes/easel_guide.md").read_text()
                self.assertEqual(tools_in(text), BASE + LIBRARY + (["ask_critic"] if critic else []))

    def test_watercolor_export_rewrites_both_guides(self):
        fixture = self.fixture(library=True)
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
        self.assertEqual((dest / "notes/easel_guide.md").read_text(), REWRITE(WATERCOLOR, watercolor=True))
        self.assertEqual((dest / "notes/easel_guide.md").read_bytes(), (dest / "notes/watercolor_easel_guide.md").read_bytes())


if __name__ == "__main__":
    unittest.main()
