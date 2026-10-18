#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["pillow"]
# ///
import importlib.util
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from PIL import Image


SCRIPT = Path(__file__).resolve().parents[1] / "export_painting.py"
spec = importlib.util.spec_from_file_location("export_painting", SCRIPT)
exporter = importlib.util.module_from_spec(spec)
spec.loader.exec_module(exporter)


class SamplingTests(unittest.TestCase):
    def test_dense_sampling_is_even_and_includes_endpoints(self):
        samples = exporter.sample_process(list(range(1000)))
        self.assertEqual(len(samples), 495)
        self.assertEqual((samples[0], samples[-1]), (0, 999))
        self.assertEqual(set(b - a for a, b in zip(samples, samples[1:])), {2, 3})

    def test_short_process_repeats_evenly_without_changing_order(self):
        self.assertEqual(exporter.sample_process([1, 2, 3], 5), [1, 2, 2, 3, 3])
        samples = exporter.sample_process(["first", "last"])
        self.assertEqual(samples, ["first"] * 247 + ["last"] * 248)

    def test_one_frame_and_exact_count(self):
        self.assertEqual(exporter.sample_process([7]), [7] * 495)
        self.assertEqual(exporter.sample_process(list(range(495))), list(range(495)))

    def test_empty_process_is_rejected(self):
        with self.assertRaises(ValueError):
            exporter.sample_process([])

    def test_house_boundaries_and_profile(self):
        for profile, final in (("friedrich", "gallery"), ("hopper", "plain"), ("blank", "plain")):
            with self.subTest(profile=profile):
                frames = exporter.clip_frames(["first", "middle", "last"], "gallery", "plain", profile)
                self.assertEqual(len(frames), 600)
                self.assertEqual(frames[:15], ["gallery"] * 15)
                self.assertEqual(frames[15], "first")
                self.assertEqual(frames[509], "last")
                self.assertEqual(frames[510:], [final] * 90)


class MetadataTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.frames = Path(self.temp.name)

    def index(self, rows):
        (self.frames / "index.tsv").write_text("chunk\thand_s\tpainted\n" + rows)
        for row in rows.splitlines():
            chunk = int(row.split("\t")[0])
            (self.frames / f"{chunk:06}.jpg").touch()

    def test_first_paint_and_chunk_gaps_and_drying(self):
        self.index("1\t0\t0\n2\t5\t0\n4\t10\t1\n9\t10\t1\n20\t60\t1\n")
        self.assertEqual([p.name for p in exporter.recorded_frames(self.frames)],
                         ["000004.jpg", "000009.jpg", "000020.jpg"])

    def test_no_paint_is_rejected(self):
        self.index("1\t10\t0\n")
        with self.assertRaisesRegex(ValueError, "no first paint"):
            exporter.recorded_frames(self.frames)

    def test_bad_metadata_is_rejected(self):
        for rows in ("1\t1\t1\n1\t2\t1\n", "2\t1\t1\n1\t2\t1\n", "1\t2\t1\n2\t1\t1\n",
                     "1\tnan\t1\n", "1\tinf\t1\n", "1\t-1\t1\n", "1\t1\ttrue\n"):
            with self.subTest(rows=rows):
                self.index(rows)
                with self.assertRaises(ValueError):
                    exporter.recorded_frames(self.frames)

    def test_missing_frame_is_rejected(self):
        self.index("1\t2\t1\n")
        (self.frames / "000001.jpg").unlink()
        with self.assertRaisesRegex(ValueError, "missing recorded frame"):
            exporter.recorded_frames(self.frames)

    def test_missing_columns_are_rejected(self):
        (self.frames / "index.tsv").write_text("filename\thand_s\na.jpg\t1\n")
        with self.assertRaisesRegex(ValueError, "needs chunk, hand_s and painted"):
            exporter.recorded_frames(self.frames)


@unittest.skipUnless(shutil.which("ffmpeg") and shutil.which("ffprobe"), "needs ffmpeg and ffprobe")
class ExportTests(unittest.TestCase):
    def test_live_saves_and_real_encoding_without_replay(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            studio = root / "studio with spaces"
            frames = studio / "out/easel/painting/frames"
            frames.mkdir(parents=True)
            (studio / "bin").mkdir()
            (studio / "bin/box").write_text("hopper\n")
            (frames / "index.tsv").write_text("chunk\thand_s\tpainted\n1\t0\t0\n2\t10\t1\n3\t20\t1\n")
            for chunk, color in ((1, "white"), (2, "red"), (3, "blue")):
                Image.new("RGB", (64, 48), color).save(frames / f"{chunk:06}.jpg")
            easel = root / "external-easel"
            easel.touch(mode=0o755)
            calls = []
            real_run = subprocess.run

            def run(command, **kwargs):
                if command[0] != str(easel):
                    return real_run(command, **kwargs)
                calls.append(command)
                self.assertEqual(command[1], "save")
                self.assertEqual(kwargs["env"]["EASEL_ROOT"], str(studio))
                self.assertEqual(kwargs["env"]["EASEL_SESSION"], "painting")
                self.assertEqual(kwargs["cwd"], studio)
                Image.new("RGB", (1200, 900), "green" if command[-1] == "--gallery" else "yellow").save(command[2])
                return subprocess.CompletedProcess(command, 0)

            out = root / "output"
            with patch.object(exporter.subprocess, "run", side_effect=run):
                info = exporter.export(studio, easel, None, "test", out)
            self.assertEqual(len(calls), 2)
            self.assertEqual(calls[0][-1], "--gallery")
            self.assertEqual(len(calls[1]), 3)
            self.assertEqual(info["format"]["duration"], "20.000000")
            self.assertEqual(sorted(p.name for p in out.iterdir()),
                             ["test-gallery.png", "test-plain.png", "test-timelapse.mp4"])
            with Image.open(out / "test-gallery.png") as image:
                self.assertEqual(image.size, (1200, 900))
            clip = out / "test-timelapse.mp4"
            for frame, expected in ((0, (0, 128, 0)), (14, (0, 128, 0)), (15, (254, 0, 0)),
                                    (509, (0, 0, 254)), (510, (255, 255, 0)), (599, (255, 255, 0))):
                decoded = real_run(["ffmpeg", "-v", "error", "-i", str(clip), "-vf", f"select=eq(n\\,{frame})",
                                    "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgb24", "-"],
                                   check=True, capture_output=True).stdout
                self.assertTrue(all(abs(actual - target) < 8 for actual, target in zip(decoded[:3], expected)),
                                (frame, decoded[:3], expected))
            self.assertFalse(list((studio / "out/easel/painting").glob("export-*")))
            with self.assertRaisesRegex(ValueError, "already exists"):
                exporter.export(studio, easel, "friedrich", "test", out)


class CheckpointTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.studio = self.root / "studio"
        self.session = self.studio / "out/easel/painting"
        frames = self.session / "frames"
        frames.mkdir(parents=True)
        (frames / "index.tsv").write_text("chunk\thand_s\tpainted\n1\t10\t1\n")
        (frames / "000001.jpg").touch()
        self.checkpoint = self.session / "live.ckpt"
        self.checkpoint.write_bytes(b"fixture checkpoint")
        self.log = self.studio / "paintings/lua/painting.lua"
        self.log.parent.mkdir(parents=True)
        self.log.write_text("--@ chunk 1\ncanvas{}\n")
        self.easel = self.root / "easel"
        self.easel.touch(mode=0o755)
        self.out = self.root / "out"

    def test_closed_state_uses_source_validated_render_and_explicit_profile(self):
        for compressed in (False, True):
            with self.subTest(compressed=compressed):
                checkpoint = self.checkpoint
                if compressed:
                    checkpoint = self.checkpoint.with_suffix(".ckpt.zst")
                    self.checkpoint.rename(checkpoint)
                out = self.out / str(compressed)

                def render(command, **kwargs):
                    self.assertNotIn("EASEL_BOX", kwargs["env"])
                    self.assertEqual(command[:3], [str(self.easel), "render-checkpoint", str(checkpoint)])
                    self.assertEqual(command[4:], ["--mode", "gallery" if "gallery.png" in command[3] else "color",
                                                   "--log", str(self.log)])
                    Path(command[3]).write_bytes(b"full resolution PNG fixture")
                    return subprocess.CompletedProcess(command, 0)

                def encode(sequence, path, work):
                    self.assertEqual(sequence[:15], [sequence[0]] * 15)
                    self.assertEqual(sequence[510:], [sequence[0]] * 90)
                    path.write_bytes(b"verified clip fixture")
                    return {"verified": True}

                with patch.dict(os.environ, {"EASEL_BOX": "hopper"}), \
                     patch.object(exporter.subprocess, "run", side_effect=render) as run, \
                     patch.object(exporter, "encode_clip", side_effect=encode), \
                     patch.object(exporter.shutil, "which", return_value="fixture"):
                    self.assertEqual(exporter.export(self.studio, self.easel, "friedrich", "test", out),
                                     {"verified": True})
                self.assertEqual(run.call_count, 2)
                self.assertEqual((out / "test-gallery.png").read_bytes(), b"full resolution PNG fixture")

    def test_explicit_live_export_ignores_a_stale_checkpoint_without_opening(self):
        def save(command, **kwargs):
            self.assertEqual(command[:2], [str(self.easel), "save"])
            self.assertEqual(command[3:], ["--gallery"] if "gallery.png" in command[2] else [])
            self.assertNotIn("EASEL_BOX", kwargs["env"])
            Path(command[2]).write_bytes(b"current live PNG fixture")
            return subprocess.CompletedProcess(command, 0)

        def encode(sequence, path, work):
            path.write_bytes(b"verified live clip fixture")
            return {"verified": True}

        with patch.dict(os.environ, {"EASEL_BOX": "hopper"}), \
             patch.object(exporter.subprocess, "run", side_effect=save) as run, \
             patch.object(exporter, "encode_clip", side_effect=encode), \
             patch.object(exporter.shutil, "which", return_value="fixture"):
            self.assertEqual(exporter.export(self.studio, self.easel, "friedrich", "test", self.out, live=True),
                             {"verified": True})
        self.assertEqual(run.call_count, 2)
        self.assertEqual((self.out / "test-plain.png").read_bytes(), b"current live PNG fixture")

    def test_checkpoint_validation_failure_never_falls_back_to_live_or_replay(self):
        with patch.object(exporter.subprocess, "run", side_effect=subprocess.CalledProcessError(1, "render-checkpoint")) as run, \
             patch.object(exporter, "encode_clip") as encode, \
             patch.object(exporter.shutil, "which", return_value="fixture"):
            with self.assertRaises(subprocess.CalledProcessError):
                exporter.export(self.studio, self.easel, "friedrich", "test", self.out)
        self.assertEqual(run.call_count, 1)
        self.assertEqual(run.call_args.args[0][1], "render-checkpoint")
        encode.assert_not_called()
        self.assertFalse(self.out.exists())

    def test_profile_metadata_is_required_unless_explicit(self):
        with self.assertRaisesRegex(ValueError, "pass --profile"):
            exporter.export(self.studio, self.easel, None, "test", self.out)


if __name__ == "__main__":
    unittest.main()
