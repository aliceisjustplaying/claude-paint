#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow", "websockets"]
# ///
"""Host-owned finish queue. Never launches a painter or opens a live easel."""

import argparse
import asyncio
import base64
import csv
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
from urllib.parse import urlsplit

REPO = Path(__file__).resolve().parents[2]


def read_json(path):
    return json.loads(path.read_text())


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    pending = path.with_suffix(".pending")
    with pending.open("w") as file:
        json.dump(value, file, indent=2)
        file.write("\n")
        file.flush()
        os.fsync(file.fileno())
    pending.replace(path)
    fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def command(args, **kwargs):
    result = subprocess.run([str(arg) for arg in args], stdin=subprocess.DEVNULL,
                            capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"{Path(args[0]).name} exit {result.returncode}: {result.stderr[-2000:]}")
    return result.stdout


def recover(config, outcome, work):
    studio = Path(config["studio"])
    saved = studio / "out/easel/painting"
    if outcome["status"] != "finalization_failed":
        return saved / "live.ckpt"
    recovered = work / "recovery.json"
    if recovered.exists():
        return Path(read_json(recovered)["checkpoint"])
    env = {k: v for k, v in os.environ.items() if not k.startswith("EASEL_")}
    result = subprocess.run([str(studio / "bin/easel"), "close"], cwd=studio, env=env,
                            stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=120)
    if result.returncode == 0:
        write_json(recovered, {"checkpoint": str(saved / "live.ckpt")})
        return saved / "live.ckpt"
    if "no easel session" not in result.stderr and "no painting open" not in result.stderr:
        raise RuntimeError(f"final save still failing: {result.stderr[-2000:]}")
    checkpoint = work / "recovery.replay.ckpt"
    if not checkpoint.exists():
        pending = checkpoint.with_suffix(".pending")
        shutil.copyfile(saved / "replay.ckpt", pending)
        pending.replace(checkpoint)
    output = work / "recovered.ckpt"
    command([studio / "bin/easel", "run", saved / "committed.lua", "--replay-checkpoint", checkpoint,
             "--require-replay-checkpoint", "--checkpoint-out", output, "--out", work / "recovered.png"],
            cwd=work, env=env)
    write_json(recovered, {"checkpoint": str(output)})
    return output


def cut_frames(saved, work, total, final=None):
    from PIL import Image, ImageChops

    source = saved / "hand-frames"
    with (source / "frames.tsv").open() as file:
        rows = list(csv.DictReader(file, delimiter="\t"))
    with (saved / "frames/index.tsv").open() as file:
        chunks = list(csv.DictReader(file, delimiter="\t"))
    first = next((int(row["chunk"]) for row in chunks if row["painted"] == "1"), None)
    if first is None:
        raise RuntimeError("no recorded paint")
    baseline = next((row for row in reversed(rows)
                     if row["kind"] == "chunk" and int(row["chunks_done"]) < first), None)
    start = 0
    if baseline:
        ground = Image.open(source / baseline["file"]).convert("RGB")
        start = rows.index(baseline) + 1
        while start < len(rows):
            image = Image.open(source / rows[start]["file"]).convert("RGB")
            if ImageChops.difference(ground, image).getbbox():
                break
            start += 1
    rows = rows[start:]
    if not rows:
        raise RuntimeError("no recorded paint frames")
    if final is None and (rows[-1]["kind"] != "final" or int(rows[-1]["chunks_done"]) != total):
        raise RuntimeError("recorded final does not match the finished painting")
    states = [json.loads(line) for line in (source / "frames.states").read_text().splitlines()]
    if not states or any(state["light"] != [115.0, 55.0] for state in states):
        raise RuntimeError("recorded frames are not in gallery light")
    if final is None and states[-1]["identity"]["source"] != list(hashlib.sha256((saved / "committed.lua").read_bytes()).digest()):
        raise RuntimeError("recorded frames do not match the committed log")
    frames = work / "clip-frames"
    frames.mkdir(exist_ok=True)
    for row in rows:
        path = frames / row["file"]
        if not path.exists():
            path.symlink_to((source / row["file"]).resolve())
    if final is not None:
        with Image.open(final) as image:
            width = min(1000, image.width)
            image = image.resize((width, max(1, round(image.height * width / image.width))))
            image.save(frames / "final.png")
        rows.append({**rows[-1], "file": "final.png", "kind": "final", "chunks_done": str(total)})
    with (frames / "frames.tsv").open("w") as file:
        writer = csv.DictWriter(file, fieldnames=list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    return frames


def verify_video(path):
    probe = json.loads(command(["ffprobe", "-v", "error", "-count_frames", "-select_streams", "v:0",
                              "-show_entries", "stream=avg_frame_rate,nb_read_frames,duration:format=duration",
                              "-of", "json", path]))
    stream = probe["streams"][0]
    if (stream["avg_frame_rate"] != "30/1" or int(stream["nb_read_frames"]) != 600
            or float(stream["duration"]) != 20.0 or float(probe["format"]["duration"]) != 20.0):
        raise RuntimeError(f"not an exact 600-frame, 20-second clip: {probe}")
    return probe


def render(config, outcome, work):
    work.mkdir(parents=True, exist_ok=True)
    studio = Path(config["studio"])
    saved = studio / "out/easel/painting"
    text = (saved / "committed.lua").read_text()
    total = sum(line.startswith("--@ chunk") for line in text.splitlines())
    checkpoint = recover(config, outcome, work)
    final = None
    if outcome["status"] == "finalization_failed":
        final = work / "final-gallery.png"
        command([studio / "bin/easel", "render-checkpoint", checkpoint, final,
                 "--mode", "gallery", "--log", saved / "committed.lua"], cwd=work)
    else:
        metadata = (saved / "live.txt").read_text().splitlines()
        if f"chunks {total}" not in metadata or not (saved / "live.png").is_file():
            raise RuntimeError("finished outcome has no matching final save")
    frames = cut_frames(saved, work, total, final)
    output = work / "painting.partial.mp4"
    command([REPO / "scripts/replay_clip", saved / "committed.lua", output,
             "--reuse", "--frames-dir", frames, "--painted-input", "--pace", "dynamic", "--length", "20",
             "--fps", "30", "--final-first", "0.5", "--open-hold", "0", "--max-hold", "16.5",
             "--light", "gallery"])
    probe = verify_video(output)
    output.replace(work / "painting.mp4")
    write_json(work / "video.json", probe)


class OBS:
    def __init__(self, socket):
        self.socket = socket
        self.serial = 0

    async def request(self, kind, data=None):
        self.serial += 1
        request_id = str(self.serial)
        await self.socket.send(json.dumps({"op": 6, "d": {"requestType": kind,
                               "requestId": request_id, "requestData": data or {}}}))
        while True:
            message = json.loads(await asyncio.wait_for(self.socket.recv(), 10))
            if message["op"] != 7 or message["d"]["requestId"] != request_id:
                continue
            reply = message["d"]
            if not reply["requestStatus"]["result"]:
                raise RuntimeError(f"OBS {kind} failed: {reply['requestStatus'].get('code')}")
            return reply.get("responseData", {})


async def stop_assigned(obs, stream, studio):
    scene = stream["scene"]
    expected = stream["url_template"].replace("{painting}", studio.removeprefix("paint-studio-"))
    for check in range(2):
        if (await obs.request("GetCurrentProgramScene"))["currentProgramSceneName"] != scene:
            return False
        items = (await obs.request("GetSceneItemList", {"sceneName": scene}))["sceneItems"]
        visible = [item for item in items if item["sceneItemEnabled"]]
        if len(visible) != 1 or visible[0]["sourceName"] != stream["input"]:
            return False
        settings = await obs.request("GetInputSettings", {"inputName": stream["input"]})
        if settings.get("inputKind") != "browser_source" or settings["inputSettings"].get("url") != expected:
            return False
        if check == 0 and not (await obs.request("GetStreamStatus"))["outputActive"]:
            return True
    await obs.request("StopStream")
    if (await obs.request("GetStreamStatus"))["outputActive"]:
        raise RuntimeError("OBS has not completed the stream stop")
    return True


async def stop_stream(stream, studio):
    import websockets

    async with websockets.connect(stream["endpoint"], open_timeout=10) as socket:
        hello = json.loads(await asyncio.wait_for(socket.recv(), 10))
        if hello["op"] != 0:
            raise RuntimeError("OBS did not send Hello")
        ident = {"rpcVersion": 1, "eventSubscriptions": 0}
        auth = hello["d"].get("authentication")
        if auth:
            password = Path(stream["password_file"]).expanduser().read_text().strip()
            secret = base64.b64encode(hashlib.sha256((password + auth["salt"]).encode()).digest()).decode()
            ident["authentication"] = base64.b64encode(hashlib.sha256((secret + auth["challenge"]).encode()).digest()).decode()
        await socket.send(json.dumps({"op": 1, "d": ident}))
        if json.loads(await asyncio.wait_for(socket.recv(), 10))["op"] != 2:
            raise RuntimeError("OBS did not identify")
        return await stop_assigned(OBS(socket), stream, studio)


class Worker:
    def __init__(self, roots, state, streams):
        if len(streams) > 1:
            raise ValueError("at most one OBS assignment is supported")
        self.roots, self.state, self.streams = roots, state.resolve(), streams

    def jobs(self):
        seen = set()
        for root in self.roots:
            for path in root.glob("*/run-config.json"):
                try:
                    config = read_json(path)
                    key = (str(Path(config["studio"]).resolve()), str(Path(config["run"]).resolve()))
                    if key in seen:
                        continue
                    seen.add(key)
                    outcome_path = Path(config["run"]) / "OPUS/p1_outcome.json"
                    if not outcome_path.exists():
                        continue
                    outcome = read_json(outcome_path)
                    if outcome["status"] not in ("finished", "finalization_failed"):
                        continue
                    identity = hashlib.sha256(json.dumps(key).encode()).hexdigest()[:24]
                    yield config, outcome, outcome_path, self.state / identity
                except (OSError, ValueError, KeyError) as error:
                    print(f"{path}: {error}", file=sys.stderr, flush=True)

    async def process(self, config, outcome, outcome_path, work):
        lock_path = Path(config["run"]) / "runner.lock"
        with lock_path.open("a") as lock:
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError:
                return
            await self.finalize(config, outcome, outcome_path, work)

    async def finalize(self, config, outcome, outcome_path, work):
        work.mkdir(parents=True, exist_ok=True)
        status_path = work / "status.json"
        status = read_json(status_path) if status_path.exists() else {}
        if status.get("done") or status.get("retry_at", 0) > time.time():
            return
        status.setdefault("finished_at", outcome.get("finished_at", outcome_path.stat().st_mtime))
        status["outcome"] = outcome
        try:
            if not status.get("video") or not Path(status["video"]).is_file():
                status["phase"] = "rendering"
                status["attempts"] = status.get("attempts", 0) + 1
                write_json(status_path, status)
                await asyncio.to_thread(render, config, outcome, work)
                status["video"] = str(work / "painting.mp4")
                status["phase"] = "grace"
                status.pop("error", None)
                status.pop("retry_at", None)
                write_json(status_path, status)
            if time.time() < status["finished_at"] + 900:
                return
            studio = Path(config["studio"]).name
            if not self.streams or not await stop_stream(self.streams[0], studio):
                status["phase"] = "stream_not_assigned"
                write_json(status_path, status)
                return
            status["stopped_stream"] = self.streams[0]["endpoint"]
            status.update(phase="done", done=True)
        except Exception as error:
            status.update(phase="failed", error=str(error), retry_at=time.time() + 60)
            print(f"{Path(config['studio']).name}: {error}", file=sys.stderr, flush=True)
        write_json(status_path, status)

    async def scan(self):
        for job in self.jobs():
            await self.process(*job)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--runners", type=Path, action="append", required=True)
    parser.add_argument("--state", type=Path, required=True)
    parser.add_argument("--streams", type=Path, required=True)
    parser.add_argument("--once", action="store_true")
    args = parser.parse_args()
    streams = read_json(args.streams)
    for stream in streams:
        if urlsplit(stream["endpoint"]).scheme not in ("ws", "wss") or "{painting}" not in stream["url_template"]:
            parser.error("each stream needs a websocket endpoint and a URL template containing {painting}")
    args.state.mkdir(parents=True, exist_ok=True)
    with (args.state / "worker.lock").open("w") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        worker = Worker(args.runners, args.state, streams)

        async def run():
            while True:
                await worker.scan()
                if args.once:
                    break
                await asyncio.sleep(30)

        asyncio.run(run())


if __name__ == "__main__":
    main()
