#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import tempfile
import time
import unittest

SCRIPTS = Path(__file__).resolve().parents[1]


class BatchTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=os.environ.get("TMPDIR"))
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        shutil.copy(SCRIPTS / "replay_batch", self.root / "replay_batch")
        self.events = self.root / "events"
        clip = self.root / "replay_clip"
        clip.write_text('''#!/usr/bin/env python3
import json, os, pathlib, subprocess, sys, time
log, out = map(pathlib.Path, sys.argv[1:3])
def event(kind):
    with open(os.environ["EVENTS"], "a") as f:
        f.write(json.dumps([kind, str(out), os.environ["RAYON_NUM_THREADS"], sys.argv[3:]]) + "\\n")
event("start")
if log.read_text() == "block":
    child = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(60)"])
    pathlib.Path(str(out) + ".pid").write_text(str(child.pid))
    child.wait()
time.sleep(0.2)
event("end")
out.write_text("clip")
sys.exit(1 if log.read_text() == "fail" else 0)
''')
        clip.chmod(0o755)

    def manifest(self, workers=(4, 4, 4), contents=None):
        rows = []
        for i, count in enumerate(workers):
            log = self.root / f"{i}.lua"
            log.write_text(contents[i] if contents else "ok")
            rows.append({"log": log.name, "out": f"{i}.mp4", "workers": count})
        manifest = self.root / "jobs.json"
        manifest.write_text(json.dumps(rows))
        return manifest, rows

    def command(self, manifest, *args):
        return ["uv", "run", "--no-project", "--script", str(self.root / "replay_batch"), str(manifest), *args]

    def run_batch(self, manifest, *args):
        return subprocess.run(self.command(manifest, *args), env={**os.environ, "EVENTS": str(self.events)},
                              text=True, capture_output=True, timeout=15)

    def test_budget_and_worker_controls(self):
        manifest, rows = self.manifest((4, 6, 4))
        result = self.run_batch(manifest, "--cpu-budget", "8", "--jobs", "3", "--fps", "24", "--light", "color")
        self.assertEqual(result.returncode, 0, result.stderr)
        active, peak = 0, 0
        for kind, out, workers, flags in map(json.loads, self.events.read_text().splitlines()):
            active += int(workers) * (1 if kind == "start" else -1)
            peak = max(peak, active)
            self.assertLessEqual(active, 8)
            self.assertEqual(flags, ["--fps", "24", "--light", "color", "--encoder-threads", "1"])
        self.assertEqual(peak, 6)
        self.assertEqual(active, 0)
        self.assertTrue(all((self.root / row["out"]).is_file() for row in rows))

    def test_jobs_limit_and_failure_continues(self):
        manifest, _ = self.manifest(contents=["fail", "ok", "ok"])
        result = self.run_batch(manifest, "--cpu-budget", "12", "--jobs", "2")
        self.assertEqual(result.returncode, 1, result.stderr)
        active, peak = 0, 0
        events = list(map(json.loads, self.events.read_text().splitlines()))
        for kind, *_ in events:
            active += 1 if kind == "start" else -1
            peak = max(peak, active)
            self.assertLessEqual(active, 2)
        self.assertEqual(peak, 2)
        self.assertEqual(len(events), 6)

    def test_manifest_validation_before_any_job(self):
        manifest, rows = self.manifest((4, 4))
        variants = [[], {"log": "0.lua"}, [dict(rows[0], workers=True)],
                    [dict(rows[0], workers=9)], [dict(rows[0], encoder_threads=7)],
                    [dict(rows[0], unexpected=1)], [dict(rows[0], log="missing.lua")],
                    [rows[0], dict(rows[1], out=rows[0]["out"])],
                    [rows[0], dict(rows[1], out="0.mp4.frames/nested.mp4")],
                    [dict(rows[0], out="1.lua"), rows[1]]]
        for variant in variants:
            with self.subTest(variant=variant):
                manifest.write_text(json.dumps(variant))
                result = self.run_batch(manifest, "--cpu-budget", "8")
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertFalse(self.events.exists())
        manifest.write_text(json.dumps(rows))
        (self.root / "0.mp4.frames").mkdir()
        self.assertEqual(self.run_batch(manifest).returncode, 2)

    def test_encoder_slots_are_reserved(self):
        manifest, rows = self.manifest((1, 1))
        for row in rows:
            row["encoder_threads"] = 3
        manifest.write_text(json.dumps(rows))
        result = self.run_batch(manifest, "--cpu-budget", "8", "--jobs", "2")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual([json.loads(line)[0] for line in self.events.read_text().splitlines()],
                         ["start", "end", "start", "end"])

    def test_startup_error_does_not_cancel_other_jobs(self):
        manifest, rows = self.manifest()
        (self.root / "blocked").write_text("not a directory")
        rows[1]["out"] = "blocked/clip.mp4"
        manifest.write_text(json.dumps(rows))
        result = self.run_batch(manifest, "--cpu-budget", "8", "--jobs", "2")
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertTrue((self.root / "0.mp4").is_file())
        self.assertTrue((self.root / "2.mp4").is_file())
        self.assertIn("job 2: failed to start", result.stderr)
        events = list(map(json.loads, self.events.read_text().splitlines()))
        self.assertEqual(sum(kind == "end" for kind, *_ in events), 2)

    def test_cancel_stops_encoder_descendants(self):
        manifest, _ = self.manifest((4,), ["block"])
        process = subprocess.Popen(self.command(manifest, "--cpu-budget", "4"), env={**os.environ, "EVENTS": str(self.events)},
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        self.addCleanup(lambda: process.poll() is None and process.kill())
        pidfile = self.root / "0.mp4.pid"
        deadline = time.monotonic() + 10
        while not pidfile.exists() and time.monotonic() < deadline:
            time.sleep(0.05)
        self.assertTrue(pidfile.exists())
        child = int(pidfile.read_text())
        process.send_signal(signal.SIGTERM)
        stdout, stderr = process.communicate(timeout=10)
        self.assertEqual(process.returncode, 130, stdout + stderr)
        if Path(f"/proc/{child}/stat").exists():
            self.assertEqual(Path(f"/proc/{child}/stat").read_text().split()[2], "Z")
        else:
            with self.assertRaises(ProcessLookupError):
                os.kill(child, 0)


class ClipControlsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(dir=os.environ.get("TMPDIR"))
        cls.addClassCleanup(cls.tmp.cleanup)
        cls.root = Path(cls.tmp.name)
        cls.frames = cls.root / "frames"
        cls.frames.mkdir()
        rows = ["file\thand_secs\tticks\tchunks_done\tkind"]
        for index, color in enumerate(("white", "red", "blue")):
            name = f"{index:04d}.png"
            subprocess.run(["ffmpeg", "-v", "error", "-f", "lavfi", "-i", f"color=c={color}:s=64x48",
                            "-frames:v", "1", str(cls.frames / name)], check=True)
            rows.append(f"{name}\t{index * 60}\t{index}\t0\ttick")
        (cls.frames / "frames.tsv").write_text("\n".join(rows) + "\n")
        cls.easel = cls.root / "easel"
        cls.easel.write_text('''#!/usr/bin/env python3
import json, os, pathlib, shutil, sys
args = sys.argv[1:]
pathlib.Path(os.environ["REPLAY_ARGS"]).write_text(json.dumps(args))
shutil.copytree(os.environ["FRAMES"], args[args.index("--frames-dir") + 1])
print("frames written", file=sys.stderr)
''')
        cls.easel.chmod(0o755)

    def clip(self, name, *args):
        out = self.root / f"{name}.mp4"
        result = subprocess.run([str(SCRIPTS / "replay_clip"), "none.lua", str(out), "--easel", str(self.easel),
                                 "--width", "64", "--max-hold", "10", *args],
                                env={**os.environ, "FRAMES": str(self.frames), "REPLAY_ARGS": str(self.root / "args.json")},
                                capture_output=True, timeout=30)
        self.assertEqual(result.returncode, 0, result.stderr.decode())
        return out

    def test_default_contract_and_gallery(self):
        out = self.clip("default")
        metadata = json.loads(subprocess.check_output(["ffprobe", "-v", "error", "-show_entries",
                                                      "format=duration:stream=nb_frames,r_frame_rate", "-of", "json", str(out)]))
        self.assertEqual(metadata["streams"][0]["nb_frames"], "600")
        self.assertEqual(metadata["streams"][0]["r_frame_rate"], "30/1")
        self.assertEqual(float(metadata["format"]["duration"]), 20)
        self.assertIn("--gallery", json.loads((self.root / "args.json").read_text()))
        pixels = subprocess.check_output(["ffmpeg", "-v", "error", "-i", str(out), "-vf", "scale=1:1",
                                          "-f", "rawvideo", "-pix_fmt", "rgb24", "-"])
        colors = []
        for i in range(0, len(pixels), 3):
            r, g, b = pixels[i:i + 3]
            colors.append("blue" if b > r + 50 else "white" if min(r, g, b) > 200 else "red")
        self.assertEqual(colors[:15], ["blue"] * 15)
        self.assertEqual(colors[15], "white")
        self.assertNotIn("blue", colors[15:510])
        self.assertEqual(colors[510:], ["blue"] * 90)

    def test_explicit_legacy_controls(self):
        out = self.clip("legacy", "--fps", "24", "--light", "color", "--encoder-threads", "2", "--final-first", "0", "--length", "8")
        self.assertNotIn("--gallery", json.loads((self.root / "args.json").read_text()))
        count = subprocess.check_output(["ffprobe", "-v", "error", "-show_entries", "stream=nb_frames",
                                         "-of", "csv=p=0", str(out)], text=True).strip()
        self.assertEqual(count, "192")

    def test_invalid_controls_are_refused(self):
        for flag, value in (("--fps", "0"), ("--fps", "nan"), ("--light", "museum"), ("--encoder-threads", "0")):
            with self.subTest(flag=flag, value=value):
                result = subprocess.run([str(SCRIPTS / "replay_clip"), "none.lua", str(self.root / "invalid.mp4"), flag, value],
                                        capture_output=True, timeout=5)
                self.assertEqual(result.returncode, 2)
                self.assertFalse((self.root / "invalid.mp4").exists())


if __name__ == "__main__":
    unittest.main()
