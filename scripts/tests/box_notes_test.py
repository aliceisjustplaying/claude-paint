import os
from pathlib import Path
import subprocess
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]
TABLE = ("| tube | pigment | hiding | stiffness | tinting strength | drying |\n"
         "|---|---|---|---|---|---|\n"
         "| carmine lake | fixture pigment | opaque | medium | medium | medium |\n"
         "| yellow ochre | fixture pigment | opaque | medium | medium | medium |\n")


class BoxNotesTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="box-notes-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "bin").mkdir()
        (self.root / "notes/research").mkdir(parents=True)
        (self.root / "bin/box").write_text("fixture\n")
        (self.root / "bin/tubes").write_text("carmine lake\nyellow ochre\n")
        (self.root / "notes/easel_guide.md").write_text(TABLE)
        easel = self.root / "bin/easel"
        easel.write_text('#!/bin/sh\ncd "$(dirname "$0")" || exit 1\n'
                         'if [ "${2:-}" = --markdown ]; then\n'
                         '  cat ../notes/easel_guide.md\n'
                         'else\n  cat tubes\nfi\n')
        easel.chmod(0o755)

    def check_note(self, mapping):
        (self.root / "notes/research/fixture_materials.md").write_text(
            "## 4. Pigments\n\n" + TABLE + "\n## 9. The tubes here\n\n" + mapping + "\n")
        env = dict(os.environ, TMPDIR=str(self.root))
        env.pop("R16_BRANCH", None)
        return subprocess.run([str(REPO / "scripts/tests/box_notes.sh"), str(self.root)],
                              env=env, capture_output=True, text=True)

    def test_parenthetical_punctuation_does_not_split_a_tube(self):
        for remark in ("a period proxy; not found in the late paintings",
                       "a period proxy. Not found in the late paintings",
                       "a proxy, inferred and approximate"):
            with self.subTest(remark=remark):
                result = self.check_note(
                    f"cochineal lake → carmine lake ({remark}), yellow ochre; "
                    "ochre → yellow ochre.")
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertIn("fixture: ok (2 tubes", result.stdout)

    def test_outside_tube_after_parenthetical_is_rejected(self):
        result = self.check_note(
            "cochineal lake → carmine lake (a period proxy; inferred); ochre → missing tube.")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('§9 maps to "missing tube", which is not in the box', result.stderr)

    def test_outside_catalog_tube_in_parenthetical_is_rejected(self):
        result = self.check_note("cochineal lake → carmine lake (not rose madder; a proxy).")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('§9 names "rose madder", which is not in the box', result.stderr)


if __name__ == "__main__":
    unittest.main()
