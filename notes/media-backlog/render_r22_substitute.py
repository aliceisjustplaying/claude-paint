"""One authorized historical substitute replay, launched only after batch job2.

Original paintings and exact-replay assertions in render_batch.py stay untouched.
Build, full replay, metrics and encode are sequential under the shared heavy lock.
"""
import fcntl
import json
import math
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time

sys.dont_write_bytecode = True
import render_batch as media

HERE = Path(__file__).resolve().parent
QUEUE = HERE / "r22-substitute-queue.json"
STATE = HERE / "r22-substitute-state.json"


def command(args, cwd, log, env):
    log.write("$ " + media.scrub(json.dumps([str(a) for a in args])) + "\n")
    log.flush()
    subprocess.run([str(a) for a in args], cwd=cwd, env=env, stdout=log,
                   stderr=log, check=True)


def image_difference(original, replay, scratch, log, env):
    a, b = media.png(original), media.png(replay)
    if (a["width"], a["height"]) != (b["width"], b["height"]):
        raise RuntimeError("substitute replay has different dimensions")
    rgb = []
    for number, path in enumerate([original, replay]):
        target = scratch / f"metrics-{number}.rgb"
        command(["ffmpeg", "-v", "error", "-y", "-i", path, "-frames:v", "1",
                 "-pix_fmt", "rgb24", "-f", "rawvideo", target], scratch, log, env)
        rgb.append(target.read_bytes())
    pixels = a["width"] * a["height"]
    if any(len(data) != pixels * 3 for data in rgb):
        raise RuntimeError("RGB comparison size differs")
    total, squares, maximum, changed_channels = 0, 0, 0, 0
    for first, second in zip(*rgb):
        delta = abs(first - second)
        total += delta
        squares += delta * delta
        maximum = max(maximum, delta)
        changed_channels += delta != 0
    changed_pixels = sum(rgb[0][i:i + 3] != rgb[1][i:i + 3]
                         for i in range(0, pixels * 3, 3))
    return {"comparison": "Original finished PNG versus full-resolution substitute finished replay",
            "width": a["width"], "height": a["height"], "pixels": pixels,
            "rgb_byte_range": [0, 255], "mean_absolute_channel_difference": total / (pixels * 3),
            "root_mean_square_channel_difference": math.sqrt(squares / (pixels * 3)),
            "maximum_channel_difference": maximum, "changed_channels": changed_channels,
            "changed_pixels": changed_pixels, "changed_pixel_percent": changed_pixels / pixels * 100,
            "png_byte_exact": a["sha256"] == b["sha256"],
            "rgb_pixel_exact": changed_pixels == 0,
            "interpretation": "Measured image difference; not proof of exact source or physical equivalence."}


def process(item):
    if item["id"] != "r22.1-inns" or item.get("allow_substitute") is not True:
        raise RuntimeError("substitute authorization is confined to r22.1-inns")
    repo = Path(item["repo"]).expanduser()
    out = Path(item["output_dir"]).expanduser()
    out.mkdir(parents=True, exist_ok=True)
    originals = {name: Path(item[name]).expanduser() for name in item["original_sha256"]}
    for name, path in originals.items():
        if media.digest(path) != item["original_sha256"][name]:
            raise RuntimeError(f"original {name} changed")
    source, finished = originals["source_log"], originals["finished_log"]
    chunks = len(re.findall(rb"^--@ chunk", source.read_bytes(), re.M))
    if chunks != 311 or not finished.read_bytes().startswith(source.read_bytes()):
        raise RuntimeError("original complete painting log was not preserved")
    if len(re.findall(rb"^--@ chunk", finished.read_bytes(), re.M)) != 312:
        raise RuntimeError("expected all 311 original chunks plus automatic finishing")
    if media.digest(originals["normal_png"]) != media.digest(originals["check_png"]):
        raise RuntimeError("original normal view differs from its original replay check")
    for name, target in [("finished_png", "r22.1-inns-finished.png"),
                         ("normal_png", "r22.1-inns-normal.png")]:
        path = out / target
        if path.exists() and media.digest(path) != item["original_sha256"][name]:
            raise RuntimeError("existing preserved original differs")
        if not path.exists():
            shutil.copyfile(originals[name], path)
        media.png(path)
    commit = subprocess.check_output(["git", "rev-parse", item["source_commit"] + "^{commit}"],
                                     cwd=repo, text=True).strip()
    if commit != item["source_commit"]:
        raise RuntimeError("historical source commit differs")
    for name, expected in item["source_build_inputs"].items():
        actual = subprocess.check_output(["git", "rev-parse", f"{commit}:{name}"],
                                         cwd=repo, text=True).strip()
        if actual != expected:
            raise RuntimeError("historical source/build input differs")
    video, replay = out / item["substitute_video"], out / item["substitute_png"]
    receipt_path = out / "substitute-receipt.json"
    if receipt_path.exists():
        prior = json.loads(receipt_path.read_text())
        if prior.get("status") == "ready substitute" and prior.get("source_commit") == commit and prior.get("original_sha256") == item["original_sha256"] and video.exists() and replay.exists() and media.digest(video) == prior["video"]["sha256"] and media.digest(replay) == prior["substitute_png"]["sha256"]:
            media.probe(video)
            print("REUSED r22.1-inns authorized substitute", flush=True)
            return prior
    if video.exists() or replay.exists():
        raise RuntimeError("existing unverified substitute deliverables; preserve and stop")
    scratch = Path(tempfile.mkdtemp(prefix="stillwet-r22-substitute-"))
    partial = out / "r22.1-inns-substitute-f334aef.partial.mp4"
    if partial.exists():
        shutil.rmtree(scratch)
        raise RuntimeError("existing partial substitute video; preserve and stop")
    env = dict(os.environ, EASEL_ROOT=str(scratch / "runtime"), EASEL_SESSION="r22-substitute")
    started = time.monotonic()
    try:
        src = scratch / "source"
        src.mkdir()
        archive = scratch / "source.tar"
        with (out / "substitute-build.log").open("w") as log:
            command(["git", "archive", "--format=tar", "--output", archive, commit,
                     "--", ".cargo", "Cargo.toml", "Cargo.lock", "crates", "scripts"], repo, log, env)
            archive_hash = media.digest(archive)
            with tarfile.open(archive) as tar:
                tar.extractall(src, filter="data")
            for name, expected in item["workflow_sha256"].items():
                if media.digest(src / name) != expected:
                    raise RuntimeError("historical replay workflow differs")
            env["CARGO_TARGET_DIR"] = str(src / "target")
            print("BUILD r22.1-inns historical substitute " + commit, flush=True)
            command(["cargo", "build", "--release", "--locked", "-p", "easel"], src, log, env)
        engine = src / "target/release/easel"
        engine_hash = media.digest(engine)
        frames = scratch / "frames"
        with (out / "substitute-replay-clip.log").open("w") as log:
            print("REPLAY r22.1-inns all 312 chunks", flush=True)
            command([src / "scripts/replay_clip", finished, partial, "--every", "60",
                     "--pace", "dynamic", "--gamma", "0.4", "--ramp", "0.3", "--length", "20",
                     "--open-hold", "0.25", "--width", "1920", "--frames-dir", frames,
                     "--easel", engine], src, log, env)
            replay_log = frames.with_suffix(".log").read_text()
            rows = (frames / "frames.tsv").read_text().splitlines()
            if "(312 chunks," not in replay_log or int(rows[-1].split("\t")[3]) != 312:
                raise RuntimeError("missing successful complete substitute replay receipt")
            full = frames / "final-full.png"
            metrics = image_difference(originals["finished_png"], full, scratch, log, env)
            vp = media.probe(partial)
            if (vp["streams"][0]["width"], vp["streams"][0]["height"]) != (1920, 1280):
                raise RuntimeError("substitute video aspect differs from original painting")
            command(["ffmpeg", "-v", "error", "-i", partial, "-f", "null", "-"], src, log, env)
            for number, second in enumerate([0, 6, 12, 19.5]):
                command(["ffmpeg", "-v", "error", "-y", "-ss", str(second), "-i", partial,
                         "-frames:v", "1", "-vf", "scale=640:-2", scratch / f"review-{number}.png"], src, log, env)
            command(["ffmpeg", "-v", "error", "-y", "-framerate", "1", "-i", scratch / "review-%d.png",
                     "-vf", "tile=2x2:padding=4:color=white", "-frames:v", "1", out / "substitute-video-review.png"], src, log, env)
            for name, path in originals.items():
                if media.digest(path) != item["original_sha256"][name]:
                    raise RuntimeError("original changed during substitute replay")
            shutil.copyfile(full, replay)
            partial.replace(video)
        receipt = {"id": item["id"], "status": "ready substitute", "verified_at": media.stamp(),
                   "provenance": "User-authorized closest historical substitute; unavailable original source 0e00118 remains unverified. This video shows the substitute replay, not a guaranteed exact original engine replay.",
                   "source_commit": commit, "source_archive_sha256": archive_hash,
                   "source_build_inputs": item["source_build_inputs"], "engine_sha256": engine_hash,
                   "workflow_sha256": item["workflow_sha256"], "original_sha256": item["original_sha256"],
                   "original_chunks": 311, "automatic_finishing_chunks": 1, "full_replay_chunks": 312,
                   "full_replay_success": True, "sampled_frames": len(rows) - 1,
                   "image_difference": metrics, "substitute_png": media.png(replay), "video": vp,
                   "options": {"length": 20, "width": 1920, "every": 60, "pace": "dynamic", "gamma": 0.4,
                               "ramp": 0.3, "open_hold": 0.25, "fps": 24, "crf": 23},
                   "elapsed_seconds": round(time.monotonic() - started, 3),
                   "visual_review": "Pending inspection of exact substitute PNG and video contact sheet before delivery."}
        (out / "substitute-full-replay.log").write_text(media.scrub(replay_log))
        (out / "substitute-frames.tsv").write_text("\n".join(rows) + "\n")
        media.write_json(out / "substitute-image-difference.json", metrics)
        media.write_json(receipt_path, receipt)
        return receipt
    finally:
        partial.unlink(missing_ok=True)
        for name in ["substitute-build.log", "substitute-replay-clip.log"]:
            path = out / name
            if path.exists():
                path.write_text(media.scrub(path.read_text()))
        shutil.rmtree(scratch)


def main():
    queue = json.loads(QUEUE.read_text())
    if len(queue["items"]) != 1:
        raise RuntimeError("expected one substitute queue item")
    lockdir = Path(os.environ.get("LOCKRUN_DIR", f"/tmp/lockrun-{os.getuid()}"))
    lockdir.mkdir(mode=0o700, exist_ok=True)
    try:
        with (lockdir / "lock").open("a") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            media.write_json(STATE, {"status": "running", "updated_at": media.stamp()})
            media.disk()
            receipt = process(queue["items"][0])
            free = media.disk()
        media.write_json(STATE, {"status": "ready substitute", "updated_at": media.stamp(),
                                "free_bytes_after": free, "source_commit": receipt["source_commit"]})
        print("SUBSTITUTE_COMPLETE r22.1-inns ready substitute", flush=True)
        return 0
    except Exception as e:
        media.write_json(STATE, {"status": "failed", "error": media.scrub(e), "updated_at": media.stamp()})
        print("SUBSTITUTE_FAILED " + media.scrub(e), flush=True)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
