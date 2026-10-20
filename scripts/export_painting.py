#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["pillow"]
# ///
"""Export a finished studio without opening or replaying it.

    uv run --script scripts/export_painting.py NAME OUT --studio STUDIO
        [--easel PATH] [--profile PROFILE] [--live]

Profile defaults to STUDIO/bin/box. Recorded Gallery JPEGs and frames/index.tsv
(chunk, hand_s, painted) must already exist in out/easel/painting/frames.
Process sampling is uniform over recorded chunks, starting at painted=1.
The clip uses the existing 1000px-wide framing, 15 Gallery-final frames,
495 process frames and 90 final frames (Gallery for Friedrich, plain otherwise).
PNG exports keep the canvas's full resolution. Closed studios render live.ckpt
(or live.ckpt.zst or live.ckpt.gz), validated against paintings/lua/painting.lua,
using a render-checkpoint-capable --easel. --live uses client-only save instead, including
after a studio reopens and its previous checkpoint becomes stale. Without a
checkpoint, save connects to the running studio.
No finishing is applied. A failed render or connection never starts a replay.
"""

import argparse
import csv
import gzip
import json
import math
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

from PIL import Image


def recorded_frames(directory):
    frames = []
    previous_chunk = -1
    previous_hand = 0.0
    with (directory / "index.tsv").open(newline="") as source:
        rows = csv.DictReader(source, delimiter="\t")
        if not {"chunk", "hand_s", "painted"}.issubset(rows.fieldnames or []):
            raise ValueError("frames/index.tsv needs chunk, hand_s and painted columns")
        for row in rows:
            chunk = int(row["chunk"])
            hand = float(row["hand_s"])
            if chunk <= previous_chunk or not math.isfinite(hand) or hand < previous_hand:
                raise ValueError("frame chunks must increase and hand_s must be finite and nondecreasing")
            if row["painted"] not in ("0", "1"):
                raise ValueError("frame painted must be 0 or 1")
            path = directory / f"{chunk:06}.jpg"
            if not path.is_file():
                raise ValueError(f"missing recorded frame: {path}")
            previous_chunk, previous_hand = chunk, hand
            if frames or row["painted"] == "1":
                frames.append(path)
    if not frames:
        raise ValueError("no first paint: frames/index.tsv has no painted=1")
    return frames


def sample_process(frames, count=495):
    if not frames or count < 2:
        raise ValueError("sampling needs recorded paint and at least two output frames")
    return [frames[(2 * i * (len(frames) - 1) + count - 1) // (2 * (count - 1))]
            for i in range(count)]


def clip_frames(process, gallery, plain, profile):
    final = gallery if profile == "friedrich" else plain
    return [gallery] * 15 + sample_process(process) + [final] * 90


def verify_clip(path):
    result = subprocess.run([
        "ffprobe", "-v", "error", "-count_frames", "-show_entries",
        "stream=codec_type,codec_name,pix_fmt,r_frame_rate,avg_frame_rate,nb_read_frames:format=duration,size",
        "-of", "json", str(path),
    ], check=True, capture_output=True, text=True)
    info = json.loads(result.stdout)
    streams = info["streams"]
    if len(streams) != 1:
        raise ValueError("clip must contain exactly one video stream")
    video = streams[0]
    expected = {"codec_type": "video", "codec_name": "h264", "pix_fmt": "yuv420p",
                "r_frame_rate": "30/1", "avg_frame_rate": "30/1", "nb_read_frames": "600"}
    if any(video.get(key) != value for key, value in expected.items()):
        raise ValueError(f"clip is not 600 H.264 frames at 30 fps: {video}")
    if abs(float(info["format"]["duration"]) - 20) > 0.000001:
        raise ValueError("clip duration is not 20.000 seconds")
    if int(info["format"]["size"]) >= 100_000_000:
        raise ValueError("clip is not under 100 MB")
    return info


def encode_clip(sequence, path, work):
    with Image.open(sequence[0]) as image:
        width = min(1000, image.width)
        width -= width % 2
        height = max(2, round(image.height * width / image.width / 2) * 2)
    if width < 2:
        raise ValueError("canvas is too narrow for H.264")
    normalized = {}
    for i, source in enumerate(sequence):
        if source not in normalized:
            frame = work / f"source-{len(normalized):04}.png"
            with Image.open(source) as image:
                image.convert("RGB").resize((width, height), Image.Resampling.LANCZOS).save(frame)
            normalized[source] = frame
        (work / f"clip-{i:04}.png").symlink_to(normalized[source])
    subprocess.run([
        "ffmpeg", "-v", "error", "-nostdin", "-y", "-threads", "1",
        "-framerate", "30", "-i", str(work / "clip-%04d.png"),
        "-frames:v", "600", "-filter_threads", "1", "-c:v", "libx264",
        "-threads:v", "1", "-pix_fmt", "yuv420p", "-crf", "23", "-preset", "slow",
        "-maxrate", "30M", "-bufsize", "60M", "-movflags", "+faststart", str(path),
    ], check=True)
    return verify_clip(path)


def export(studio, easel, profile, name, destination, live=False):
    if not re.fullmatch(r"[A-Za-z0-9_-]+", name):
        raise ValueError("name must contain only letters, digits, _ or -")
    studio = studio.expanduser().resolve(strict=True)
    destination = destination.expanduser().resolve()
    easel = easel.expanduser().resolve(strict=True) if easel else studio / "bin/easel"
    if not easel.is_file() or not os.access(easel, os.X_OK):
        raise ValueError(f"no executable easel: {easel}")
    if profile is None:
        metadata = studio / "bin/box"
        if not metadata.is_file():
            raise ValueError("no bin/box profile metadata; pass --profile")
        profile = metadata.read_text().strip()
    if not profile:
        raise ValueError("profile must not be empty")
    for command in ("ffmpeg", "ffprobe"):
        if not shutil.which(command):
            raise ValueError(f"missing {command}")
    session = studio / "out/easel/painting"
    process = recorded_frames(session / "frames")
    checkpoint = None if live else next((session / filename for filename in ("live.ckpt", "live.ckpt.zst", "live.ckpt.gz")
                                        if (session / filename).is_file()), None)
    log = (studio / "paintings/lua/painting.lua").resolve(strict=True) if checkpoint else None
    names = [f"{name}-gallery.png", f"{name}-plain.png", f"{name}-timelapse.mp4"]
    for filename in names:
        if os.path.lexists(destination / filename):
            raise ValueError(f"output already exists: {destination / filename}")
    env = dict(os.environ, EASEL_ROOT=str(studio), EASEL_SESSION="painting")
    env.pop("EASEL_BOX", None)
    with tempfile.TemporaryDirectory(prefix="export-", dir=session) as temporary:
        work = Path(temporary)
        gallery, plain, clip = [work / filename for filename in names]
        if checkpoint and checkpoint.suffix == ".gz":
            uncompressed = work / "live.ckpt"
            with gzip.open(checkpoint, "rb") as source, uncompressed.open("wb") as target:
                shutil.copyfileobj(source, target)
            checkpoint = uncompressed
        for path, mode in ((gallery, "gallery"), (plain, "color")):
            if checkpoint:
                command = [str(easel), "render-checkpoint", str(checkpoint), str(path),
                           "--mode", mode, "--log", str(log)]
            else:
                command = [str(easel), "save", str(path), *(["--gallery"] if mode == "gallery" else [])]
            subprocess.run(command, cwd=studio, env=env, check=True)
        info = encode_clip(clip_frames(process, gallery, plain, profile), clip, work)
        destination.mkdir(parents=True, exist_ok=True)
        for filename in names:
            with (work / filename).open("rb") as source, (destination / filename).open("xb") as output:
                shutil.copyfileobj(source, output)
    return info


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("name")
    parser.add_argument("out", type=Path)
    parser.add_argument("--studio", type=Path, required=True)
    parser.add_argument("--easel", type=Path, help="defaults to STUDIO/bin/easel")
    parser.add_argument("--profile", help="defaults to STUDIO/bin/box")
    parser.add_argument("--live", action="store_true", help="save from the running studio, ignoring older checkpoints")
    args = parser.parse_args()
    try:
        info = export(args.studio, args.easel, args.profile, args.name, args.out, live=args.live)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"export_painting: {error}\n")
    print(json.dumps(info, indent=2))


if __name__ == "__main__":
    main()
