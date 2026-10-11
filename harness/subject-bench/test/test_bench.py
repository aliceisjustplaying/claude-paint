import importlib.util
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("bench", Path(__file__).parents[1] / "bench.py")
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)


class BenchTests(unittest.TestCase):
    def test_exact_brief_and_mismatch(self):
        with tempfile.TemporaryDirectory() as directory:
            studio = Path(directory)
            (studio / "BRIEF.md").write_bytes(bench.house(studio))
            self.assertEqual(bench.verify_brief(studio), "")
            (studio / "BRIEF.md").write_bytes(bench.house(studio) + b"\nPaint light.\n")
            with self.assertRaisesRegex(ValueError, "Brief mismatch"):
                bench.verify_brief(studio)

    def test_words_are_primary_and_unknown_is_not_none(self):
        self.assertEqual(bench.classify("Light paint dries slowly.")["light"], "unknown")
        self.assertEqual(bench.classify("Plan: a bowl of apples.")["light"], "none")
        self.assertEqual(bench.classify("Plan: a lamp lighting a kitchen table.")["light"], "main")
        self.assertEqual(bench.classify("Plan: a meadow at sunset.")["light"], "secondary")

    def test_aborted_usage_is_unknown(self):
        _, cost, complete = bench.accounting({"usage": [{"stopReason": "aborted", "usage": {
            "input": 0, "output": 0, "cost": {"total": 0}}}]})
        self.assertIsNone(cost)
        self.assertFalse(complete)

    def test_summary_counts_no_subject_at_cap(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            rows = [{"run": "mock-001", "model": "mock", "no_subject_at_cap": True,
                     "subject": "unknown", "look": "", "stop": "chunk_cap", "light": "unknown"}]
            bench.summarize(root, rows)
            self.assertIn('"no_subject_at_cap": 1', (root / "summary.json").read_text())


if __name__ == "__main__":
    unittest.main()
