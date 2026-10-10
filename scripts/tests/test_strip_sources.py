"""Regression check for the Hopper note's exported painter copy.

    python3 scripts/tests/test_strip_sources.py
"""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]


class HopperPainterCopy(unittest.TestCase):
    def test_all_source_keys_are_removed(self):
        source = REPO / "notes/research/hopper_materials.md"
        original = source.read_text(encoding="utf-8")
        with tempfile.TemporaryDirectory(prefix="strip-sources-") as directory:
            copy = Path(directory) / "hopper.md"
            copy.write_text(original, encoding="utf-8")
            result = subprocess.run(
                [sys.executable, str(REPO / "scripts/strip_sources"), str(copy)],
                capture_output=True, text=True,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            painter = copy.read_text(encoding="utf-8")
        self.assertNotRegex(painter, r"\[[A-Z0-9][^\[\]]*\]")
        self.assertNotRegex(painter, r"https?://|www\.")
        self.assertNotIn("## Sources", painter)
        self.assertNotIn("<!--", painter)
        self.assertNotIn("91.9 × 127.2 cm", painter)
        self.assertIn("## Night and artificial light", painter)
        self.assertIn("almost painting white on white", " ".join(painter.split()))
        self.assertEqual(source.read_text(encoding="utf-8"), original)


if __name__ == "__main__":
    unittest.main()
