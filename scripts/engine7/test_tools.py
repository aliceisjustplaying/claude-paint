import contextlib
import io
import json
from pathlib import Path
import tempfile
import unittest

from replay import program
from trace import analyze, instrument


class Tools(unittest.TestCase):
    def test_prefix_preserves_original_chunks(self):
        source = "--@ box hopper\n--@ engine 5\n--@ chunk 1\na()\n--@ chunk 2\nb()\n"
        text, count = program(source, chunks=1)
        self.assertEqual(text, "--@ box hopper\n--@ engine 5\n--@ chunk 1\na()\n")
        self.assertEqual(count, 1)

    def test_engine_override_does_not_change_program(self):
        text, count = program("--@ box hopper\n--@ engine 5\n--@ chunk 1\na()\n", engine=7)
        self.assertEqual(text.count("--@ engine"), 1)
        self.assertIn("--@ engine 7", text)
        self.assertIn("--@ box hopper", text)
        self.assertTrue(text.endswith("a()\n"))
        self.assertEqual(count, 1)

    def test_plain_excludes_only_finishing(self):
        source = "--@ chunk 1\na()\n--@ chunk (finishing)\nfinish()\n"
        self.assertEqual(program(source), ("--@ chunk 1\na()\n", 1))
        self.assertEqual(program(source, finishing=True), (source, 2))

    def test_runtime_time_and_brush_edges(self):
        with tempfile.TemporaryDirectory() as temp:
            log, result = Path(temp) / "stderr", Path(temp) / "result.json"
            log.write_text("E7TRACE\t1\tbrush:stroke\t17\tmarks\tfalse\t0\t0\t0\t1\t1\n"
                           "E7TRACE\t1\tbrush:stroke\t17\tmarks\tfalse\t0\t0\t1\t2\t1\n"
                           "E7TRACE\t1\twork\t0\tpass\ttrue\t0\t2\t2\t0\t1\n")
            with contextlib.redirect_stdout(io.StringIO()):
                analyze(log, result)
            data = json.loads(result.read_text())
            self.assertEqual(data["ready_width_upper_bound"], 1)
            self.assertEqual(data["weighted_span_s"], 3)
            self.assertEqual(data["edge_counts"]["brush"], 1)
            self.assertGreater(data["edge_counts"]["time"], 0)

    def test_independent_brushes_can_have_width_two(self):
        with tempfile.TemporaryDirectory() as temp:
            log, result = Path(temp) / "stderr", Path(temp) / "result.json"
            log.write_text("E7TRACE\t1\tprint\t0\t\tfalse\t0\t0\t0\t0\t1\n"
                           "E7TRACE\t1\tbrush:stroke\t17\tmarks\tfalse\t0\t0\t0\t1\t1\n"
                           "E7TRACE\t1\tbrush:stroke\t18\tmarks\tfalse\t0\t0\t1\t2\t1\n")
            with contextlib.redirect_stdout(io.StringIO()):
                analyze(log, result)
            self.assertEqual(json.loads(result.read_text())["ready_width_upper_bound"], 2)


if __name__ == "__main__":
    unittest.main()
