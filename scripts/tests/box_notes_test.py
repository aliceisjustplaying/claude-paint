import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]
TUBES = ("lead white", "carmine lake", "yellow ochre")
TABLE = ("| tube | pigment | hiding | stiffness | tinting strength | drying |\n"
         "| --- | --- | --- | --- | --- | --- |\n"
         + "".join(f"| {tube} | pigment | opaque | medium | medium | medium |\n" for tube in TUBES))


class BoxNotesTests(unittest.TestCase):
    def check_note(self, mapping):
        with tempfile.TemporaryDirectory(prefix="box-notes-fixture-") as temp:
            root = Path(temp)
            studio = root / "studio"
            (studio / "bin").mkdir(parents=True)
            (studio / "notes/research").mkdir(parents=True)
            (studio / "bin/box").write_text("giverny\n")
            easel = studio / "bin/easel"
            easel.write_text(f"#!{sys.executable}\nimport sys\n"
                             f"TABLE = {TABLE!r}\nTUBES = {TUBES!r}\n"
                             "print(TABLE if '--markdown' in sys.argv else '\\n'.join(TUBES))\n")
            easel.chmod(0o755)
            (studio / "notes/easel_guide.md").write_text(TABLE)
            (studio / "notes/research/giverny_materials.md").write_text(
                f"## 4. Pigments\n{TABLE}\n## 9. The tubes here\n{mapping}\n")
            return subprocess.run(
                [str(REPO / "scripts/tests/box_notes.sh"), str(studio)],
                env=dict(os.environ, TMPDIR=temp, R16_BRANCH="HEAD"),
                capture_output=True, text=True,
            )

    def test_parenthetical_semicolon_keeps_the_tube_whole(self):
        result = self.check_note(
            "cochineal lake → carmine lake (a period cochineal proxy; not found in the late paintings examined); "
            "other pigments → lead white and yellow ochre.")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("giverny: ok (3 tubes", result.stdout)

    def test_mapping_after_parenthetical_semicolon_still_checks_the_box(self):
        result = self.check_note(
            "cochineal lake → carmine lake (a period cochineal proxy; not found in the late paintings examined); "
            "another pigment → not a box tube.")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('§9 maps to "not a box tube", which is not in the box', result.stderr)


if __name__ == "__main__":
    unittest.main()
