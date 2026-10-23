import asyncio
import base64
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import AsyncMock, patch


SPEC = importlib.util.spec_from_file_location("finalize", Path(__file__).resolve().parents[1] / "finalize.py")
F = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(F)
STREAM = {"scene": "Stillwet", "input": "Browser Source",
          "url_template": "https://stillwet.art/live/{painting}/?stream=1", "endpoint": "ws://unused"}


class WorkerTests(unittest.IsolatedAsyncioTestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.studio = self.root / "paint-studio-abc123"
        self.saved = self.studio / "out/easel/painting"
        self.saved.mkdir(parents=True)
        self.run = self.root / "run"
        self.outcome = self.run / "OPUS/p1_outcome.json"
        self.config = {"studio": str(self.studio), "run": str(self.run)}
        F.write_json(self.root / "runner/run-config.json", self.config)
        self.worker = F.Worker([self.root], self.root / "state", [STREAM])

    def finish(self, status="finished", finished_at=100):
        F.write_json(self.outcome, {"status": status, "finished_at": finished_at})

    def rendered(self, config, outcome, work):
        (work / "painting.mp4").write_bytes(b"verified by renderer")

    def status(self):
        return F.read_json(next((self.root / "state").glob("*/status.json")))

    @unittest.skipUnless(os.environ.get("EASEL_BIN"), "needs EASEL_BIN for a real recording")
    async def test_real_two_mark_recording_finishes_with_exact_video_and_stream_stop(self):
        from PIL import Image, ImageChops, ImageStat

        env = {key: value for key, value in os.environ.items() if not key.startswith("EASEL_")}
        env.update(EASEL_ROOT=str(self.studio), EASEL_SESSION="painting", RAYON_NUM_THREADS="2")
        easel = str(Path(os.environ["EASEL_BIN"]).resolve())

        def run(*args):
            return F.command([easel, *args], cwd=self.studio, env=env)

        run("open", "painting")
        try:
            run("do", 'canvas{size=300, aspect=5, linen={16,14}, seed=3, ground={{pile={{"lead white",6},{"raw umber",1}}, um=50, apply="roller"}}}')
            run("do", 'b=brush("filbert",8); b:load(pile{{"cobalt blue",1}},0.9)')
            run("do", 'b:stroke({{100,100},{800,130}}, {pressure=0.8})')
            run("do", 'b:stroke({{100,500},{800,530}}, {pressure=0.8})')
            run("do", "tubes()")
        finally:
            run("close")
        before = {path.relative_to(self.saved): hashlib.sha256(path.read_bytes()).hexdigest()
                  for path in self.saved.rglob("*") if path.is_file()}
        self.finish()
        with patch.object(F, "stop_stream", new_callable=AsyncMock, return_value=True) as stop:
            await self.worker.scan()
            status = self.status()
            self.assertTrue(status.get("done"), status)
            stop.assert_awaited_once_with(STREAM, self.studio.name)
        work = Path(status["video"]).parent
        F.verify_video(Path(status["video"]))
        rows = (work / "clip-frames/frames.tsv").read_text().splitlines()[1:]
        self.assertEqual(len({row.split("\t")[1] for row in rows}), 2)
        durations = [float(line.split()[1]) for line in (work / "clip-frames/concat.txt").read_text().splitlines()
                     if line.startswith("duration ")]
        self.assertEqual([round(duration * 30) for duration in durations], [15, 495, 88, 1])
        decoded = subprocess.run(
            ["ffmpeg", "-v", "error", "-threads", "1", "-i", status["video"], "-filter_threads", "1",
             "-vf", "scale=100:20", "-f", "rawvideo", "-pix_fmt", "rgb24", "-threads", "1", "-"],
            capture_output=True, check=True).stdout
        size = 100 * 20 * 3
        frames = [Image.frombytes("RGB", (100, 20), decoded[index:index + size])
                  for index in range(0, len(decoded), size)]
        self.assertEqual(len(frames), 600)
        source = self.saved / "hand-frames"
        reference = Image.open(source / rows[0].split("\t")[0]).convert("RGB").resize((100, 20))
        ground_name = (source / "frames.tsv").read_text().splitlines()[1].split("\t")[0]
        ground = Image.open(source / ground_name).convert("RGB").resize((100, 20))
        self.assertGreater(sum(ImageStat.Stat(ImageChops.difference(frames[15], ground)).mean), 1)
        for frame in frames[15:510]:
            self.assertLess(sum(ImageStat.Stat(ImageChops.difference(frame, reference)).mean), 8)
        for frame in frames[:15] + frames[510:]:
            self.assertLess(sum(ImageStat.Stat(ImageChops.difference(frame, frames[-1])).mean), 2)
        self.assertEqual(before, {path.relative_to(self.saved): hashlib.sha256(path.read_bytes()).hexdigest()
                                  for path in self.saved.rglob("*") if path.is_file()})

    async def test_only_durable_finished_outcomes_trigger(self):
        with patch.object(F, "render", side_effect=self.rendered) as render, patch.object(F, "stop_stream", new_callable=AsyncMock, return_value=True) as stop:
            await self.worker.scan()
            for status in ("running", "crash_limit_reached"):
                self.finish(status)
                await self.worker.scan()
            render.assert_not_called()
            stop.assert_not_awaited()
            self.finish()
            await self.worker.scan()
            render.assert_called_once()
            stop.assert_awaited_once_with(STREAM, self.studio.name)
            self.assertTrue(self.status()["done"])
            await self.worker.scan()
            render.assert_called_once()
            stop.assert_awaited_once()

    async def test_render_retry_survives_worker_restart_without_painter(self):
        self.finish("finalization_failed")
        with patch.object(F, "render", side_effect=RuntimeError("disk full")), patch.object(F, "stop_stream", new_callable=AsyncMock) as stop:
            await self.worker.scan()
            self.assertNotIn("video", self.status())
            self.assertIn("disk full", self.status()["error"])
            stop.assert_not_awaited()
        worker = F.Worker([self.root], self.root / "state", [STREAM])
        with patch.object(F.time, "time", return_value=10**10), patch.object(F, "render", side_effect=self.rendered) as render, patch.object(F, "stop_stream", new_callable=AsyncMock, return_value=True):
            await worker.scan()
            self.assertEqual(self.status()["attempts"], 2)
            self.assertEqual(self.status()["outcome"]["status"], "finalization_failed")
            self.assertTrue(self.status()["done"])
            render.assert_called_once()

    async def test_grace_is_from_finish_not_worker_start_or_render(self):
        self.finish(finished_at=100)
        with patch.object(F, "render", side_effect=self.rendered), patch.object(F, "stop_stream", new_callable=AsyncMock, return_value=True) as stop:
            with patch.object(F.time, "time", return_value=999):
                await self.worker.scan()
                stop.assert_not_awaited()
            with patch.object(F.time, "time", return_value=1000):
                await self.worker.scan()
                stop.assert_awaited_once()

    async def test_a_running_painter_keeps_its_easel_and_stream(self):
        self.finish()
        with (self.run / "runner.lock").open("a") as lock, patch.object(F, "render") as render:
            fcntl.flock(lock, fcntl.LOCK_EX)
            await self.worker.scan()
            render.assert_not_called()

    async def test_an_alias_is_not_a_second_job(self):
        self.finish()
        F.write_json(self.root / "alias/run-config.json", self.config)
        self.assertEqual(len(list(self.worker.jobs())), 1)

    async def test_relative_state_produces_absolute_recovery_paths(self):
        self.finish("finalization_failed")
        worker = F.Worker([self.root], Path("state"), [])
        work = next(worker.jobs())[-1]
        self.assertTrue(work.is_absolute())
        self.assertEqual(work.parent, Path.cwd() / "state")

    async def test_multiple_obs_assignments_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "at most one OBS assignment"):
            F.Worker([self.root], self.root / "state", [STREAM, {**STREAM, "endpoint": "ws://other"}])


class FakeOBS:
    def __init__(self, painting="abc123", scene="Stillwet", extra=False):
        self.painting, self.scene, self.extra = painting, scene, extra
        self.calls = []
        self.active = True
        self.input_kind = "browser_source"

    async def request(self, kind, data=None):
        self.calls.append(kind)
        return self.reply(kind)

    def reply(self, kind):
        if kind == "GetCurrentProgramScene":
            return {"currentProgramSceneName": self.scene}
        if kind == "GetSceneItemList":
            items = [{"sourceName": "Browser Source", "sceneItemEnabled": True}]
            if self.extra:
                items.append({"sourceName": "another painter", "sceneItemEnabled": True})
            return {"sceneItems": items}
        if kind == "GetInputSettings":
            return {"inputKind": self.input_kind, "inputSettings": {
                "url": STREAM["url_template"].replace("{painting}", self.painting)}}
        if kind == "GetStreamStatus":
            return {"outputActive": self.active}
        if kind == "StopStream":
            self.active = False
        return {}


class OBSTests(unittest.IsolatedAsyncioTestCase):
    async def test_only_the_assigned_painting_stream_stops(self):
        for painting, scene, extra in (("other", "Stillwet", False), ("abc123", "Other", False), ("abc123", "Stillwet", True)):
            obs = FakeOBS(painting, scene, extra)
            self.assertFalse(await F.stop_assigned(obs, STREAM, "paint-studio-abc123"))
            self.assertNotIn("StopStream", obs.calls)
        obs = FakeOBS()
        self.assertTrue(await F.stop_assigned(obs, STREAM, "paint-studio-abc123"))
        self.assertEqual(obs.calls[-2:], ["StopStream", "GetStreamStatus"])

    async def test_reassignment_during_check_does_not_stop(self):
        obs = FakeOBS()
        original = obs.request

        async def reassign(kind, data=None):
            reply = await original(kind, data)
            if kind == "GetStreamStatus":
                obs.painting = "other"
            return reply

        obs.request = reassign
        self.assertFalse(await F.stop_assigned(obs, STREAM, "paint-studio-abc123"))
        self.assertNotIn("StopStream", obs.calls)

    async def test_ownership_changes_during_check_do_not_stop(self):
        for field, value in (("extra", True), ("input_kind", "image_source"),
                             ("scene", "Other")):
            with self.subTest(field=field):
                obs = FakeOBS()
                original = obs.request

                async def reassign(kind, data=None):
                    reply = await original(kind, data)
                    if kind == "GetStreamStatus":
                        setattr(obs, field, value)
                    return reply

                obs.request = reassign
                self.assertFalse(await F.stop_assigned(obs, STREAM, "paint-studio-abc123"))
                self.assertNotIn("StopStream", obs.calls)
                self.assertTrue(obs.active)

    async def test_a_non_browser_input_does_not_stop(self):
        obs = FakeOBS()
        obs.input_kind = "image_source"
        self.assertFalse(await F.stop_assigned(obs, STREAM, "paint-studio-abc123"))
        self.assertNotIn("StopStream", obs.calls)

    async def test_real_websocket_auth_and_stop_against_fake_obs(self):
        import websockets

        obs = FakeOBS()
        with tempfile.TemporaryDirectory() as temp:
            password = Path(temp) / "password"
            password.write_text("test password")

            async def server(socket):
                await socket.send(json.dumps({"op": 0, "d": {"authentication": {"salt": "salt", "challenge": "challenge"}}}))
                ident = json.loads(await socket.recv())
                secret = base64.b64encode(hashlib.sha256(b"test passwordsalt").digest()).decode()
                expected = base64.b64encode(hashlib.sha256((secret + "challenge").encode()).digest()).decode()
                self.assertEqual(ident["d"]["authentication"], expected)
                await socket.send(json.dumps({"op": 2}))
                async for text in socket:
                    request = json.loads(text)["d"]
                    data = await obs.request(request["requestType"])
                    await socket.send(json.dumps({"op": 7, "d": {"requestId": request["requestId"],
                                           "requestStatus": {"result": True}, "responseData": data}}))

            async with websockets.serve(server, "127.0.0.1", 0) as server:
                stream = {**STREAM, "endpoint": f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}", "password_file": str(password)}
                self.assertTrue(await F.stop_stream(stream, "paint-studio-abc123"))
                self.assertIn("StopStream", obs.calls)


class FrameTests(unittest.TestCase):
    def test_one_painted_moment_renders_the_full_clip(self):
        from PIL import Image

        with tempfile.TemporaryDirectory() as temp:
            studio = Path(temp) / "studio"
            saved = studio / "out/easel/painting"
            source = saved / "hand-frames"
            source.mkdir(parents=True)
            (saved / "frames").mkdir()
            text = "--@ chunk 1\npaint()\n"
            (saved / "committed.lua").write_text(text)
            (saved / "live.txt").write_text("chunks 1\n")
            (saved / "frames/index.tsv").write_text("chunk\thand_s\tpainted\n1\t25.5\t1\n")
            (source / "frames.tsv").write_text(
                "file\thand_secs\tticks\tchunks_done\tkind\n1.png\t25.5\t0\t1\tfinal\n")
            state = {"light": [115.0, 55.0], "identity": {"source": list(hashlib.sha256(text.encode()).digest())}}
            (source / "frames.states").write_text(json.dumps(state) + "\n")
            Image.new("RGB", (32, 24), "blue").save(source / "1.png")
            (saved / "live.png").write_bytes((source / "1.png").read_bytes())
            work = Path(temp) / "work"
            F.render({"studio": str(studio)}, {"status": "finished"}, work)
            F.verify_video(work / "painting.mp4")

    def test_sparse_saved_recording_renders_the_full_clip_without_replay(self):
        from PIL import Image

        with tempfile.TemporaryDirectory() as temp:
            studio = Path(temp) / "studio"
            saved = studio / "out/easel/painting"
            source = saved / "hand-frames"
            source.mkdir(parents=True)
            (saved / "frames").mkdir()
            text = "".join(f"--@ chunk {chunk}\npaint()\n" for chunk in range(1, 7))
            (saved / "committed.lua").write_text(text)
            (saved / "live.txt").write_text("chunks 6\n")
            (saved / "frames/index.tsv").write_text(
                "chunk\thand_s\tpainted\n" + "".join(
                    f"{chunk}\t{(chunk - 1) * 60}\t{int(chunk > 1)}\n" for chunk in range(1, 7)))
            (source / "frames.tsv").write_text(
                "file\thand_secs\tticks\tchunks_done\tkind\n" + "".join(
                    f"{chunk}.png\t{(chunk - 1) * 60}\t0\t{chunk}\t{'final' if chunk == 6 else 'chunk'}\n"
                    for chunk in range(1, 7)))
            state = {"light": [115.0, 55.0], "identity": {"source": list(hashlib.sha256(text.encode()).digest())}}
            (source / "frames.states").write_text(json.dumps(state) + "\n")
            for chunk, color in enumerate(("white", "blue", "green", "yellow", "cyan", "red"), 1):
                Image.new("RGB", (32, 24), color).save(source / f"{chunk}.png")
            (saved / "live.png").write_bytes((source / "6.png").read_bytes())
            before = {path.relative_to(saved): path.read_bytes() for path in saved.rglob("*") if path.is_file()}
            work = Path(temp) / "work"

            F.render({"studio": str(studio)}, {"status": "finished"}, work)

            video = work / "painting.mp4"
            self.assertEqual(F.read_json(work / "video.json"), F.verify_video(video))
            self.assertFalse((work / "painting.partial.mp4").exists())
            decoded = subprocess.run(
                ["ffmpeg", "-v", "error", "-threads", "1", "-i", str(video), "-filter_threads", "1",
                 "-vf", "scale=1:1", "-f", "rawvideo", "-pix_fmt", "rgb24", "-threads", "1", "-"],
                capture_output=True, check=True).stdout
            pixels = [decoded[index:index + 3] for index in range(0, len(decoded), 3)]
            self.assertEqual(len(pixels), 600)
            final = pixels[-1]
            self.assertGreater(final[0], 240)
            self.assertLess(max(final[1:]), 10)
            self.assertEqual(pixels[:15], [final] * 15)
            self.assertEqual(pixels[510:], [final] * 90)
            self.assertTrue(all(pixel != final and min(pixel) < 240 for pixel in pixels[15:510]))
            self.assertEqual(before, {path.relative_to(saved): path.read_bytes()
                                      for path in saved.rglob("*") if path.is_file()})

    def test_failed_live_save_is_retried_without_opening_a_painter(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            config = {"studio": str(work / "studio")}
            outcome = {"status": "finalization_failed"}
            failed = subprocess.CompletedProcess([], 1, "", "checkpoint disk full")
            with patch.object(F.subprocess, "run", return_value=failed) as close, patch.object(F, "command") as batch:
                with self.assertRaisesRegex(RuntimeError, "final save still failing"):
                    F.recover(config, outcome, work)
                self.assertEqual(close.call_args.args[0][-1], "close")
                batch.assert_not_called()
            success = subprocess.CompletedProcess([], 0, "closed", "")
            with patch.object(F.subprocess, "run", return_value=success) as close:
                expected = work / "studio/out/easel/painting/live.ckpt"
                self.assertEqual(F.recover(config, outcome, work), expected)
                self.assertEqual(F.recover(config, outcome, work), expected)
                close.assert_called_once()

    def test_dead_easel_recovery_requires_a_copy_of_latest_checkpoint(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            saved = work / "studio/out/easel/painting"
            saved.mkdir(parents=True)
            checkpoint = saved / "replay.ckpt"
            checkpoint.write_bytes(b"latest saved data")
            result = subprocess.CompletedProcess([], 1, "", "no easel session painting running")
            with patch.object(F.subprocess, "run", return_value=result), patch.object(F, "command") as batch:
                output = F.recover({"studio": str(work / "studio")}, {"status": "finalization_failed"}, work)
                self.assertEqual(output, work / "recovered.ckpt")
                args = batch.call_args.args[0]
                self.assertIn("--require-replay-checkpoint", args)
                self.assertIn(work / "recovery.replay.ckpt", args)
                self.assertEqual(checkpoint.read_bytes(), b"latest saved data")
                self.assertEqual((work / "recovery.replay.ckpt").read_bytes(), checkpoint.read_bytes())

    def test_interrupted_checkpoint_copy_retries_without_replacing_batch_progress(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            saved = work / "studio/out/easel/painting"
            saved.mkdir(parents=True)
            (saved / "replay.ckpt").write_bytes(b"latest saved data")
            checkpoint = work / "recovery.replay.ckpt"
            result = subprocess.CompletedProcess([], 1, "", "no painting open")
            config = {"studio": str(work / "studio")}
            outcome = {"status": "finalization_failed"}

            def interrupted_copy(source, destination):
                Path(destination).write_bytes(b"partial")
                raise OSError("disk full")

            def interrupted_batch(*args, **kwargs):
                self.assertEqual(checkpoint.read_bytes(), b"latest saved data")
                checkpoint.write_bytes(b"batch progress")
                raise RuntimeError("batch interrupted")

            with patch.object(F.subprocess, "run", return_value=result), patch.object(F, "command") as batch:
                with patch.object(F.shutil, "copyfile", side_effect=interrupted_copy), self.assertRaisesRegex(OSError, "disk full"):
                    F.recover(config, outcome, work)
                self.assertFalse(checkpoint.exists())
                batch.assert_not_called()
                with patch.object(F, "command", side_effect=interrupted_batch), self.assertRaisesRegex(RuntimeError, "batch interrupted"):
                    F.recover(config, outcome, work)
                with patch.object(F.shutil, "copyfile") as copy:
                    F.recover(config, outcome, work)
                copy.assert_not_called()
                self.assertEqual(checkpoint.read_bytes(), b"batch progress")

    def test_bare_ground_is_cut_and_non_friedrich_final_is_unchanged(self):
        from PIL import Image

        with tempfile.TemporaryDirectory() as temp:
            saved = Path(temp) / "saved"
            source = saved / "hand-frames"
            source.mkdir(parents=True)
            (saved / "frames").mkdir()
            text = "--@ chunk 1\ncanvas{}\n--@ chunk 2\npaint()\n"
            (saved / "committed.lua").write_text(text)
            (saved / "frames/index.tsv").write_text("chunk\thand_s\tpainted\n1\t0\t0\n2\t60\t1\n")
            (source / "frames.tsv").write_text("file\thand_secs\tticks\tchunks_done\tkind\n1.png\t0\t0\t1\tchunk\n2.png\t20\t1\t1\ttick\n3.png\t60\t0\t2\tfinal\n")
            state = {"light": [115.0, 55.0], "identity": {"source": list(hashlib.sha256(text.encode()).digest())}}
            (source / "frames.states").write_text(json.dumps(state) + "\n")
            for name, color in (("1", "white"), ("2", "blue"), ("3", "red")):
                Image.new("RGB", (10, 20), color).save(source / f"{name}.png")
            work = Path(temp) / "work"
            work.mkdir()
            frames = F.cut_frames(saved, work, 2)
            self.assertNotIn("1.png", (frames / "frames.tsv").read_text())
            self.assertEqual((frames / "3.png").read_bytes(), (source / "3.png").read_bytes())
            self.assertTrue((frames / "frames.tsv").read_text().splitlines()[1].startswith("2.png"))
            final = work / "recovered-gallery.png"
            Image.new("RGB", (1200, 2400), "red").save(final)
            F.cut_frames(saved, work, 2, final)
            with Image.open(frames / "final.png") as image:
                self.assertEqual(image.size, (1000, 2000))
            state["light"] = None
            (source / "frames.states").write_text(json.dumps(state) + "\n")
            with self.assertRaisesRegex(RuntimeError, "gallery light"):
                F.cut_frames(saved, work, 2)
            (saved / "frames/index.tsv").write_text("chunk\thand_s\tpainted\n1\t0\t0\n")
            with self.assertRaisesRegex(RuntimeError, "no recorded paint"):
                F.cut_frames(saved, work, 2)


if __name__ == "__main__":
    unittest.main()
