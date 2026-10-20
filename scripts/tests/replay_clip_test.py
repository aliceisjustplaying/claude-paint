import os
from pathlib import Path
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "replay_clip"


class OverlapTests(unittest.TestCase):
    def setUp(self):
        tools = tempfile.TemporaryDirectory()
        self.addCleanup(tools.cleanup)
        ffmpeg = Path(tools.name) / "ffmpeg"
        ffmpeg.write_text("#!/usr/bin/env bash\nexit 99\n")
        ffmpeg.chmod(0o755)
        self.env = {**os.environ, "PATH": f"{tools.name}:{os.environ['PATH']}"}

    def test_disjoint_paths_reach_replay(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            source = root / "source.lua"
            source.write_text("scratch source\n")
            easel = root / "easel"
            easel.write_text('#!/usr/bin/env bash\nprintf called > "$0.called"\nexit 1\n')
            easel.chmod(0o755)
            result = subprocess.run(
                [str(SCRIPT), str(source), str(root / "clip.mp4"), "--easel", str(easel)],
                capture_output=True, text=True, env=self.env,
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(Path(f"{easel}.called").read_text(), "called")
            self.assertEqual(source.read_text(), "scratch source\n")

    def test_overlaps_are_rejected_before_any_write(self):
        cases = (
            "source-in-frames", "source-via-parent-symlink", "source-symlink-in-frames",
            "output-is-source", "output-symlink", "output-hardlink", "output-in-frames",
            "frames-is-output", "output-via-parent-symlink", "source-is-replay-log", "source-is-checkpoint",
            "source-is-replay-progress", "sheet-is-source", "sheet-in-frames",
        )
        for case in cases:
            with self.subTest(case=case), tempfile.TemporaryDirectory() as temp:
                root = Path(temp)
                frames = root / "collision.frames"
                frames.mkdir()
                marker = frames / "frames.tsv"
                marker.write_text("scratch manifest\n")
                source = root / "source.lua"
                source.write_text("scratch source\n")
                out = root / "clip.mp4"
                sheet = root / "sheet.jpg"
                if case == "source-in-frames":
                    source = frames / "source.lua"
                    source.write_text("scratch source\n")
                elif case == "source-via-parent-symlink":
                    (root / "alias").symlink_to(frames, target_is_directory=True)
                    source = root / "alias/source.lua"
                    source.write_text("scratch source\n")
                elif case == "source-symlink-in-frames":
                    (frames / "source.lua").symlink_to(source)
                    source = frames / "source.lua"
                elif case == "output-is-source":
                    out = source
                elif case == "output-symlink":
                    out.symlink_to(source)
                elif case == "output-hardlink":
                    os.link(source, out)
                elif case == "output-in-frames":
                    out = frames / "clip.mp4"
                elif case == "frames-is-output":
                    out = frames
                elif case == "output-via-parent-symlink":
                    (root / "alias").symlink_to(frames, target_is_directory=True)
                    out = root / "alias/clip.mp4"
                elif case == "source-is-replay-log":
                    source = Path(f"{frames}.log")
                    source.write_text("scratch source\n")
                elif case == "source-is-checkpoint":
                    source = Path(f"{frames}.replay.ckpt")
                    source.write_text("scratch source\n")
                elif case == "source-is-replay-progress":
                    source = Path(f"{frames}.replay.ckpt").with_suffix(".replay-progress.json")
                    source.write_text("scratch source\n")
                elif case == "sheet-is-source":
                    sheet = source
                elif case == "sheet-in-frames":
                    sheet = frames / "sheet.jpg"
                result = subprocess.run(
                    [str(SCRIPT), str(source), str(out), "--frames-dir", str(frames),
                     "--sheet", str(sheet), "--easel", "/bin/false"],
                    capture_output=True, text=True, env=self.env,
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertTrue(source.exists(), result.stderr)
                self.assertEqual(source.read_text(), "scratch source\n")
                self.assertTrue(marker.exists(), result.stderr)
                self.assertEqual(marker.read_text(), "scratch manifest\n")
                self.assertIn("overlap", result.stderr)


if __name__ == "__main__":
    unittest.main()
