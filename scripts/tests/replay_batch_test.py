import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]
SIBLINGS = (".frames.replay.ckpt", ".frames.replay.replay-progress.json")


class ReplayBatchTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="replay-batch-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        shutil.copyfile(REPO / "scripts/replay_batch", self.root / "replay_batch")
        clip = self.root / "replay_clip"
        clip.write_text(f"#!{sys.executable}\nfrom pathlib import Path\n"
                        "Path(__file__).with_name('started').touch()\n")
        clip.chmod(0o755)
        (self.root / "painting.lua").write_text("scratch input\n")

    def run_batch(self, rows):
        manifest = self.root / "jobs.json"
        manifest.write_text(json.dumps(rows))
        return subprocess.run(
            [sys.executable, str(self.root / "replay_batch"), str(manifest),
             "--cpu-budget", "3", "--workers", "1"],
            capture_output=True, text=True, timeout=10,
        )

    def assert_refused(self, rows):
        result = self.run_batch(rows)
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertNotIn("started", result.stdout)
        self.assertFalse((self.root / "started").exists())
        self.assertFalse((self.root / "clip.mp4.batch.log").exists())
        return result

    def test_fresh_output_starts(self):
        result = self.run_batch([{"log": "painting.lua", "out": "clip.mp4"}])
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertTrue((self.root / "started").exists())

    def test_existing_replay_siblings_are_refused_before_launch(self):
        for suffix in SIBLINGS:
            for kind in ("file", "directory", "dangling symlink"):
                with self.subTest(suffix=suffix, kind=kind):
                    path = self.root / f"clip.mp4{suffix}"
                    if kind == "file":
                        path.write_bytes(b"invalid checkpoint or progress")
                    elif kind == "directory":
                        path.mkdir()
                    else:
                        path.symlink_to(self.root / "missing")
                    try:
                        result = self.assert_refused([{"log": "painting.lua", "out": "clip.mp4"}])
                        self.assertIn(f"output path already exists: {path}", result.stderr)
                    finally:
                        if kind == "directory":
                            path.rmdir()
                        else:
                            path.unlink()

    def test_replay_siblings_cannot_be_inputs(self):
        for suffix in SIBLINGS:
            with self.subTest(suffix=suffix):
                path = self.root / f"clip.mp4{suffix}"
                path.write_text("scratch input\n")
                try:
                    self.assert_refused([{"log": path.name, "out": "clip.mp4"}])
                finally:
                    path.unlink()

    def test_other_jobs_cannot_own_replay_siblings_or_their_children(self):
        for suffix in SIBLINGS:
            for child in ("", "/nested.mp4"):
                for reverse in (False, True):
                    with self.subTest(suffix=suffix, child=child, reverse=reverse):
                        rows = [{"log": "painting.lua", "out": "clip.mp4"},
                                {"log": "painting.lua", "out": f"clip.mp4{suffix}{child}"}]
                        result = self.assert_refused(rows[::-1] if reverse else rows)
                        self.assertIn("output paths overlap", result.stderr)


if __name__ == "__main__":
    unittest.main()
