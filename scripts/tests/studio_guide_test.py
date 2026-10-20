import json
from pathlib import Path
import re
import runpy
import subprocess
import unittest

from scripts.tests import export_materials_test as materials


REPO = Path(__file__).resolve().parents[2]
OIL = (REPO / "notes/easel_guide.md").read_text()
WATERCOLOR = (REPO / "notes/watercolor_easel_guide.md").read_text()


class StudioGuideTests(unittest.TestCase):
    def fixture(self):
        fixture = materials.ExportMaterialsTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        guide = re.sub(r"^\| tube \| pigment \|[^\n]*\n(?:\|[^\n]*\n?)+", materials.TABLE, OIL, flags=re.MULTILINE)
        fixture.write(fixture.root / "revisions/HEAD/notes/easel_guide.md", guide)
        fixture.write(fixture.root / "revisions/HEAD/notes/reference-library/manifest.json", "[]\n")
        fixture.executable(fixture.repo / "scripts/build_reference_library",
                           'from pathlib import Path\nimport sys\nPath(sys.argv[-1]).write_text("[]\\n")\n')
        return fixture

    def test_committed_oil_guide_names_the_nine_plain_studio_tools(self):
        table = OIL.split("| tool | what it does |\n", 1)[1].split("\n\n", 1)[0]
        self.assertEqual(re.findall(r"^\| `([^`]+)` \|", table, re.MULTILINE),
                         ["paint", "look", "probe", "note", "status", "log", "read", "reference", "reference_list"])
        self.assertNotIn("| `ref: ", OIL)

    def test_plain_export_equals_committed_guide_byte_for_byte(self):
        fixture = self.fixture()
        studio, result = fixture.export(profile="blank")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual((studio / "bin/studio").read_text(), "painter none\n")
        self.assertEqual((studio / "notes/easel_guide.md").read_bytes(),
                         (fixture.root / "revisions/HEAD/notes/easel_guide.md").read_bytes())

    def test_profile_check_accepts_the_committed_guide(self):
        fixture = self.fixture()
        result = subprocess.run([str(fixture.repo / "scripts/tests/export_profiles.sh"), "blank", "tonn"],
                                env=dict(fixture.env, R16_BRANCH="HEAD"), capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_friedrich_preparation_only_appends_one_canonical_critic_row(self):
        prep = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        description = json.loads((REPO / "harness/easel_mcp/critic-tools.json").read_text())[0]["description"]
        row = f"| `ask_critic` | {description} |\n".encode()
        for critic in (False, True):
            with self.subTest(critic=critic):
                fixture = self.fixture()
                studio, result = fixture.export(profile="blank")
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                committed = (fixture.root / "revisions/HEAD/notes/easel_guide.md").read_bytes()
                (studio / "bin/box").write_text("friedrich\n")
                (studio / "bin/studio").write_text("painter friedrich\n")
                house = REPO / "notes/round29/runner"
                prep["prepare"](studio, house, fixture.root / "runner", house / "studio_notes.md", critic=critic)
                expected = committed
                if critic:
                    start = committed.index(b"| tool | what it does |\n")
                    end = committed.index(b"\n\n", start) + 1
                    expected = committed[:end] + row + committed[end:]
                actual = (studio / "notes/easel_guide.md").read_bytes()
                self.assertEqual(actual, expected)
                self.assertEqual(actual.count(row), int(critic))

    def test_watercolor_names_exactly_its_seven_mcp_tools(self):
        expected = [tool["name"] for tool in json.loads((REPO / "harness/easel_mcp/tools.json").read_text()) if tool["name"] != "probe"]
        self.assertEqual(set(re.findall(r"`([^`]+)`", WATERCOLOR.splitlines()[2])), set(expected))
        self.assertNotIn("`probe`", WATERCOLOR)
        self.assertNotIn("`ask_critic`", WATERCOLOR)

    def test_friedrich_invalid_inputs_leave_studio_unchanged_and_allow_retry(self):
        prep = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        for invalid in ("notes", "guide header", "guide end"):
            with self.subTest(invalid=invalid):
                fixture = self.fixture()
                studio, result = fixture.export(profile="blank")
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                (studio / "bin/box").write_text("friedrich\n")
                guide = studio / "notes/easel_guide.md"
                valid_guide = guide.read_text()
                notes = fixture.root / "approved.md"
                if invalid != "notes":
                    notes.write_text("Approved fixture notes.\n")
                    guide.write_text("Malformed guide.\n" if invalid == "guide header" else "| tool | what it does |\n|---|---|\n| `paint` | paint |\n")
                before = {str(p.relative_to(studio)): p.read_bytes() for p in studio.rglob("*") if p.is_file()}
                runner = fixture.root / "runner"
                with self.assertRaises((FileNotFoundError, ValueError)):
                    prep["prepare"](studio, REPO / "notes/round29/runner", runner, notes, critic=True)
                self.assertFalse(runner.exists())
                self.assertEqual({str(p.relative_to(studio)): p.read_bytes() for p in studio.rglob("*") if p.is_file()}, before)
                notes.write_text("Approved fixture notes.\n")
                guide.write_text(valid_guide)
                prep["prepare"](studio, REPO / "notes/round29/runner", runner, notes, critic=True)
                self.assertTrue((runner / "run-config.json").is_file())

    def test_watercolor_export_copies_the_committed_guide_to_both_paths(self):
        fixture = self.fixture()
        revision = fixture.root / "revisions/HEAD"
        fixture.write(revision / "notes/watercolor_easel_guide.md", WATERCOLOR)
        for name in ("notes/watercolor_techniques.md", "notes/research/watercolor_physics.md",
                     "experiments/watercolor/THIRD_PARTY_NOTICES.md"):
            fixture.write(revision / name, "Fixture watercolor notes.\n")
        for name in ("server.py", "tools.json", "remote.py", "http_server.py"):
            fixture.write(revision / "harness/easel_mcp" / name, "{}\n")
        fixture.executable(fixture.root / "builds/watercolor-fixture-tree/target/release/watercolor-studio", "")
        studio, result = fixture.export(profile="watercolor")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        for name in ("easel_guide.md", "watercolor_easel_guide.md"):
            self.assertEqual((studio / "notes" / name).read_bytes(),
                             (revision / "notes/watercolor_easel_guide.md").read_bytes())


if __name__ == "__main__":
    unittest.main()
