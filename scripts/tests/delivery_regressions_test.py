#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["pillow"]
# ///
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time
import unittest
from unittest.mock import patch

from PIL import Image


REPO = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("exporter", REPO / "scripts/export_painting.py")
exporter = importlib.util.module_from_spec(spec)
spec.loader.exec_module(exporter)


class DeliveryRegressions(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.easel = Path(os.environ.get("EASEL_BIN", REPO / "target/debug/easel")).resolve(strict=True)
        for tool in ("ffmpeg", "ffprobe"):
            if not shutil.which(tool):
                raise RuntimeError(f"missing integration dependency: {tool}")

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.env = dict(os.environ, EASEL_ROOT=str(self.root / "studio"), EASEL_SESSION="painting",
                        RAYON_NUM_THREADS="2")
        for key in ("EASEL_BOX", "EASEL_WIDTH", "EASEL_STATE_SESSION", "EASEL_LOCK_ON_STDIN"):
            self.env.pop(key, None)

    def run_ok(self, command, **kwargs):
        result = subprocess.run(list(map(str, command)), capture_output=True, text=True,
                                timeout=120, env=self.env, **kwargs)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        return result

    def studio(self):
        studio = Path(self.env["EASEL_ROOT"])
        studio.mkdir()
        self.addCleanup(subprocess.run, [str(self.easel), "close"], env=self.env, capture_output=True, timeout=30)
        self.run_ok([self.easel, "open", "painting"])
        self.run_ok([self.easel, "do", 'canvas{size=300,aspect=5,linen={16,14},seed=3,ground={{pile={{"lead white",6},{"raw umber",1}},um=50,apply="roller"}}}'])
        self.run_ok([self.easel, "do", 'p=pile{{"lead white",6},{"cobalt blue",1}}; b=brush("filbert",1); b:load(p,0.8); b:stroke({{100,20},{150,30}})'])
        self.run_ok([self.easel, "save"])
        self.run_ok([self.easel, "close"])
        return studio

    def probe(self, video, length):
        result = self.run_ok(["ffprobe", "-v", "error", "-count_frames", "-show_entries",
                              "stream=nb_read_frames:format=duration", "-of", "json", video])
        info = json.loads(result.stdout)
        self.assertEqual(int(info["streams"][0]["nb_read_frames"]), length * 30)
        self.assertAlmostEqual(float(info["format"]["duration"]), length, places=6)

    def test_b05_fresh_clip_uses_real_easel_and_ffmpeg(self):
        studio = self.studio()
        source = self.root / "source.lua"
        text = (studio / "paintings/lua/painting.lua").read_text()
        for i in range(3, 23):
            text += f'\n--@ chunk {i}\nb:load(p,0.8); b:stroke({{{{100,20}},{{150,30}}}})\n'
        source.write_text(text)
        video = self.root / "fresh.mp4"
        self.run_ok([REPO / "scripts/replay_clip", source, video, "--easel", self.easel, "--width", "64"])
        self.probe(video, 20)
        self.assertEqual(source.read_text(), text)

    def test_b10_independent_check_replays_without_a_checkpoint(self):
        studio = self.studio()
        scripts = self.root / "scripts"
        scripts.mkdir()
        for name in ("check_painting", "compare_images.py"):
            shutil.copy2(REPO / "scripts" / name, scripts / name)
        helper = scripts / "replay_easel"
        helper.write_text(f"#!/bin/sh\nprintf '%s\\n' '{self.easel}'\n")
        helper.chmod(0o755)
        session = studio / "out/easel/painting"
        before = {p.name: p.read_bytes() for p in session.iterdir() if p.is_file()}
        work = self.root / "check"
        result = self.run_ok([scripts / "check_painting", studio, work])
        self.assertIn("check: ok:", result.stdout)
        self.assertIn("max RGB delta 0/255", result.stdout)
        self.assertFalse((work / "out/easel/painting/replay.ckpt").exists())
        self.assertEqual(before, {p.name: p.read_bytes() for p in session.iterdir() if p.is_file()})

    def test_b11_late_collision_rolls_back_only_this_export_and_retry_succeeds(self):
        studio = self.studio()
        out = self.root / "export"
        collision = out / "test-plain.png"
        encode = exporter.encode_clip

        def collide(*args):
            info = encode(*args)
            out.mkdir()
            collision.write_bytes(b"independent output")
            return info

        with patch.object(exporter, "encode_clip", side_effect=collide):
            with self.assertRaises(FileExistsError):
                exporter.export(studio, self.easel, "friedrich", "test", out)
        self.assertEqual(collision.read_bytes(), b"independent output")
        self.assertEqual(sorted(p.name for p in out.iterdir()), [collision.name])
        collision.unlink()
        exporter.export(studio, self.easel, "friedrich", "test", out)
        self.probe(out / "test-timelapse.mp4", 20)

    def test_b13_concurrent_reused_clips_keep_their_own_schedules(self):
        for pace in ("0.6", "dynamic"):
            with self.subTest(pace=pace):
                frames = self.root / f"frames-{pace}"
                frames.mkdir()
                rows = ["file\thand_secs\n"]
                for i in range(21):
                    name = f"{i:06}.png"
                    Image.new("RGB", (64, 48), (i * 10, 30, 100)).save(frames / name)
                    rows.append(f"{name}\t{i * 60}\n")
                (frames / "frames.tsv").write_text("".join(rows))
                before = {p.name: p.read_bytes() for p in frames.iterdir()}
                tools = self.root / f"tools-{pace}"
                tools.mkdir()
                ready, release = tools / "ready", tools / "release"
                ffmpeg = tools / "ffmpeg"
                ffmpeg.write_text(f'''#!/bin/sh
touch '{ready}'
while [ ! -e '{release}' ]; do sleep 0.02; done
exec '{shutil.which("ffmpeg")}' "$@"
''')
                ffmpeg.chmod(0o755)
                source = self.root / "unused.lua"
                source.touch()
                first, second = self.root / f"first-{pace}.mp4", self.root / f"second-{pace}.mp4"
                command = [str(REPO / "scripts/replay_clip"), str(source), str(first), "--reuse",
                           "--frames-dir", str(frames), "--width", "64", "--pace", pace]
                process = subprocess.Popen(command, env=dict(self.env, PATH=f"{tools}:{self.env['PATH']}"),
                                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                try:
                    deadline = time.monotonic() + 120
                    while not ready.exists() and process.poll() is None and time.monotonic() < deadline:
                        time.sleep(0.02)
                    self.assertTrue(ready.exists(), "first clip never reached FFmpeg")
                    command[2] = str(second)
                    self.run_ok([*command, "--length", "10"])
                    release.touch()
                    stdout, stderr = process.communicate(timeout=120)
                    self.assertEqual(process.returncode, 0, stdout + stderr)
                finally:
                    release.touch()
                    if process.poll() is None:
                        process.kill()
                        process.communicate()
                self.probe(first, 20)
                self.probe(second, 10)
                self.assertEqual(before, {p.name: p.read_bytes() for p in frames.iterdir()
                                          if p.name not in ("pace-change.tsv", "concat.txt")})
                self.assertIn("duration ", (frames / "concat.txt").read_text())

    def test_numeric_reused_clip_preserves_backslashes_in_frames_directory(self):
        frames = self.root / r"frames\new"
        frames.mkdir()
        rows = ["file\thand_secs\n"]
        for i in range(3):
            name = f"{i:06}.png"
            Image.new("RGB", (64, 48), (i * 100, 30, 100)).save(frames / name)
            rows.append(f"{name}\t{i * 60}\n")
        (frames / "frames.tsv").write_text("".join(rows))
        source = self.root / "unused.lua"
        source.touch()
        video = self.root / "backslash.mp4"
        self.run_ok([REPO / "scripts/replay_clip", source, video, "--reuse",
                     "--frames-dir", frames, "--width", "64", "--length", "4", "--pace", "0.6"])
        self.probe(video, 4)
        self.assertIn(str(frames / "000000.png"), (frames / "concat.txt").read_text())


if __name__ == "__main__":
    unittest.main()
