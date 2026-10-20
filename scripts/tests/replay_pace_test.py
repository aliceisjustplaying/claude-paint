#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy", "pillow"]
# ///
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import numpy as np
from PIL import Image


SCRIPT = Path(__file__).resolve().parents[1] / "replay_pace.py"
spec = importlib.util.spec_from_file_location("replay_pace", SCRIPT)
pace = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pace)


class ChangeCacheTests(unittest.TestCase):
    def test_replaced_pixels_invalidate_both_adjacent_pairs(self):
        for replaced in range(4):
            with self.subTest(replaced=replaced), tempfile.TemporaryDirectory() as temporary:
                frames = Path(temporary)
                ms = [(f"{i:06}.png", i) for i in range(4)]
                for i, (name, _) in enumerate(ms):
                    Image.new("RGB", (8, 8), (i * 40,) * 3).save(frames / name, compress_level=0)
                cache = frames / "pace-change.tsv"
                original = pace.changes(frames, ms)
                old_cache = cache.read_bytes()
                changed = frames / ms[replaced][0]
                before = changed.stat()
                Image.new("RGB", (8, 8), (255,) * 3).save(changed, compress_level=0)
                os.utime(changed, ns=(before.st_atime_ns, before.st_mtime_ns))
                self.assertEqual(changed.stat().st_size, before.st_size)
                reused = pace.changes(frames, ms)
                self.assertNotEqual(cache.read_bytes(), old_cache)
                cache.unlink()
                fresh = pace.changes(frames, ms)
                np.testing.assert_allclose(reused, fresh, atol=1e-5)
                self.assertFalse(np.array_equal(original, fresh))

    def test_unchanged_frames_reuse_measurements_without_decoding(self):
        with tempfile.TemporaryDirectory() as temporary:
            frames = Path(temporary)
            ms = [("000000.png", 0), ("000001.png", 1)]
            for i, (name, _) in enumerate(ms):
                Image.new("RGB", (8, 8), (i * 255,) * 3).save(frames / name)
            expected = pace.changes(frames, ms)
            with patch.object(pace, "load", side_effect=AssertionError("decoded cached frame")):
                np.testing.assert_allclose(pace.changes(frames, ms), expected)

    def test_legacy_cache_is_recomputed(self):
        with tempfile.TemporaryDirectory() as temporary:
            frames = Path(temporary)
            ms = [("000000.png", 0), ("000001.png", 1)]
            for name, _ in ms:
                Image.new("RGB", (8, 8), "white").save(frames / name)
            (frames / "pace-change.tsv").write_text("000000.png\t000001.png\t123.00000\n")
            np.testing.assert_array_equal(pace.changes(frames, ms), [0, 0])


if __name__ == "__main__":
    unittest.main()
