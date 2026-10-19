#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["pillow>=11,<13"]
# ///
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

from PIL import Image
from PIL.PngImagePlugin import PngInfo


class PaintingComparisonTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.scripts = self.root / "scripts"
        self.scripts.mkdir()
        source = Path(__file__).resolve().parents[1]
        for name in ("check_painting", "compare_images.py"):
            shutil.copy2(source / name, self.scripts / name)
        self.studio = self.root / "studio"
        self.live = self.studio / "out/easel/painting"
        self.live.mkdir(parents=True)
        log = self.studio / "paintings/lua/painting.lua"
        log.parent.mkdir(parents=True)
        log.write_text("--@ chunk 1\ncanvas{}\n")
        (self.live / "committed.lua").write_text(log.read_text())
        (self.live / "live.txt").write_text("chunks 1\nreplayed 0\nbuild same\nplatform linux-x86_64\n")
        (self.live / "live.ckpt").write_bytes(b"different checkpoint")
        self.original = Image.new("RGBA", (4, 4), (100, 100, 100, 255))
        self.original.save(self.live / "live.png")
        self.actual = self.root / "actual.png"
        self.original.save(self.actual)
        self.easel = self.root / "easel"
        self.easel.write_text(f"#!{sys.executable}\n" + '''import os
from pathlib import Path
import shutil
import sys
root = Path(os.environ["EASEL_ROOT"])
if sys.argv[1] == "save":
    shutil.copyfile(os.environ["ACTUAL_PNG"], sys.argv[2])
elif sys.argv[1] == "close":
    output = root / "out/easel/painting"
    (output / "live.ckpt").write_bytes(b"replayed checkpoint")
    (output / "live.txt").write_text("chunks 1\\nreplayed 1\\nbuild same\\nplatform linux-x86_64\\n")
''')
        self.easel.chmod(0o755)
        replay = self.scripts / "replay_easel"
        replay.write_text(f"#!/bin/sh\nprintf '%s\\n' '{self.easel}'\n")
        replay.chmod(0o755)

    def run_check(self):
        return subprocess.run(["bash", str(self.scripts / "check_painting"), str(self.studio), str(self.root / "work")],
                              env=dict(os.environ, ACTUAL_PNG=str(self.actual)), capture_output=True, text=True)

    def test_same_build_rounding_and_checkpoint_difference_pass(self):
        self.original.putpixel((0, 0), (102, 98, 102, 253))
        self.original.save(self.actual)
        result = self.run_check()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("check: ok:", result.stdout)
        self.assertIn("state DIFFERS", result.stdout)
        self.assertIn("diagnostic only", result.stdout)

    def test_same_build_different_png_encoding_passes(self):
        metadata = PngInfo()
        metadata.add_text("encoding", "different bytes")
        self.original.save(self.actual, pnginfo=metadata, compress_level=0)
        result = self.run_check()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("check: ok:", result.stdout)

    def test_visible_error_fails(self):
        self.original.putpixel((0, 0), (103, 100, 100, 255))
        self.original.save(self.actual)
        result = self.run_check()
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("check: DIFFERS from the live canvas:", result.stdout)

    def test_without_comparable_image_checkpoint_drift_is_replay_only(self):
        (self.live / "live.png").unlink()
        result = self.run_check()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("check: replay only:", result.stdout)

    def test_an_already_replayed_image_does_not_claim_live_validation(self):
        (self.live / "live.txt").write_text("chunks 1\nreplayed 1\n")
        result = self.run_check()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("check: replay only:", result.stdout)


if __name__ == "__main__":
    unittest.main()
