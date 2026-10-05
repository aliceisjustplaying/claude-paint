"""Generate the approved unpublished media queue sequentially. Run through uv.

No publication, painting changes, sleeps or polling. One global heavy-job lock.
Receipts make completed items resumable; scratch frames are removed per item.
"""
import fcntl
import hashlib
import json
import os
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import time
import zlib
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
QUEUE = HERE / "queue.json"
STATE = HERE / "state.json"
MIN_FREE = 20 * 1024**3  # Chosen reserve, not a measured render-space requirement.


def stamp():
    return datetime.now(timezone.utc).isoformat()


def scrub(value):
    return str(value).replace(str(Path.home()), "~")


def write_json(path, value):
    part = path.with_suffix(path.suffix + ".part")
    part.write_text(json.dumps(value, indent=2) + "\n")
    part.replace(path)


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1024**2), b""):
            h.update(block)
    return h.hexdigest()


def disk():
    free = shutil.disk_usage(HERE).free
    print(f"disk_free_bytes={free}", flush=True)
    if free < MIN_FREE:
        raise RuntimeError(f"disk reserve: {free} free bytes, {MIN_FREE} required")
    return free


def run(cmd, cwd, log):
    log.write("$ " + scrub(json.dumps([str(a) for a in cmd])) + "\n")
    log.flush()
    subprocess.run([str(a) for a in cmd], cwd=cwd, stdout=log, stderr=log, check=True)


def png(path):
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise RuntimeError(f"not PNG: {path.name}")
    width, height = struct.unpack(">II", data[16:24])
    chunks, texts, pos = [], [], 8
    while pos < len(data):
        length = struct.unpack(">I", data[pos:pos + 4])[0]
        kind, payload = data[pos + 4:pos + 8], data[pos + 8:pos + 8 + length]
        chunks.append(kind.decode("ascii"))
        if kind == b"tEXt":
            texts.append(payload.decode("latin1"))
        elif kind == b"zTXt":
            key, content = payload.split(b"\0", 1)
            texts.append(key.decode("latin1") + zlib.decompress(content[1:]).decode("latin1"))
        elif kind == b"iTXt":
            # Current engine PNGs contain no text. Refuse an unexpected text format.
            raise RuntimeError(f"unexpected iTXt metadata: {path.name}")
        pos += length + 12
    privacy(json.dumps(texts))
    return {"width": width, "height": height, "chunks": chunks, "text": texts,
            "sha256": digest(path), "bytes": path.stat().st_size}


def privacy(text):
    if re.search(r"/Users/|/home/", text, re.I):
        raise RuntimeError("home path in outgoing metadata")


def probe(path):
    raw = subprocess.check_output(["ffprobe", "-v", "error", "-show_streams",
                                   "-show_format", "-of", "json", str(path)], text=True)
    # ffprobe's local filename is diagnostic, not embedded metadata.
    p = json.loads(raw)
    p["format"].pop("filename", None)
    privacy(json.dumps(p))
    videos = [s for s in p["streams"] if s["codec_type"] == "video"]
    if len(videos) != 1 or len(p["streams"]) != 1:
        raise RuntimeError("expected one silent video stream")
    s = videos[0]
    if s["codec_name"] != "h264" or s["pix_fmt"] != "yuv420p":
        raise RuntimeError("expected H264 yuv420p")
    if abs(float(p["format"]["duration"]) - 20) > 1 / 24 + 0.000001:
        raise RuntimeError("expected a 20-second clip")
    atoms, pos = {}, 0
    with path.open("rb") as f:
        while pos < path.stat().st_size:
            f.seek(pos)
            header = f.read(8)
            size, kind = struct.unpack(">I4s", header)
            if size == 1:
                size = struct.unpack(">Q", f.read(8))[0]
            if size == 0:
                size = path.stat().st_size - pos
            if size < 8:
                raise RuntimeError("invalid MP4 atom")
            atoms[kind.decode("latin1")] = pos
            pos += size
    if atoms.get("moov", sys.maxsize) >= atoms.get("mdat", -1):
        raise RuntimeError("MP4 is not faststart")
    p["atoms"] = atoms
    p["sha256"] = digest(path)
    return p


def verify_checkout(item):
    checkout = Path(item["checkout"]).expanduser()
    engine = Path(item["engine"]).expanduser()
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=checkout, text=True).strip()
    if head != item["source_commit"] or digest(engine) != item["engine_sha256"]:
        raise RuntimeError("pinned checkout or binary changed")
    allowed = item.get("verified_untracked_files", {})
    entries = subprocess.check_output(
        ["git", "status", "--porcelain=v1", "-z", "--untracked-files=all"], cwd=checkout)
    seen = set()
    for entry in entries.split(b"\0"):
        if not entry:
            continue
        name = entry[3:].decode()
        if entry[:3] != b"?? " or name not in allowed:
            raise RuntimeError(f"unverified pinned checkout change: {scrub(name)}")
        if digest(checkout / name) != allowed[name]:
            raise RuntimeError(f"verified untracked output changed: {scrub(name)}")
        seen.add(name)
    if seen != set(allowed):
        raise RuntimeError("verified untracked output inventory changed")
    return head


def process(item):
    ident = item["id"]
    out = Path(item["output_dir"]).expanduser()
    out.mkdir(parents=True, exist_ok=True)
    source = Path(item["source_log"]).expanduser()
    finished_log = Path(item["finished_log"]).expanduser()
    finished_png = Path(item["finished_png"]).expanduser()
    normal_png = Path(item["normal_png"]).expanduser()
    check_png = Path(item["check_png"]).expanduser()
    check_log = Path(item["check_log"]).expanduser()
    source_hash = digest(source)
    chunks = len(re.findall(rb"^--@ chunk", source.read_bytes(), re.M))
    if not finished_log.read_bytes().startswith(source.read_bytes()):
        raise RuntimeError("finished log does not retain the complete original painting log")
    if len(re.findall(rb"^--@ chunk", finished_log.read_bytes(), re.M)) != chunks + 1:
        raise RuntimeError("expected exactly one automatic finishing chunk")
    checks = [line for line in check_log.read_text().splitlines() if line.startswith("check:")]
    if not checks or not checks[-1].startswith(f"check: ok: {chunks} chunks"):
        raise RuntimeError("existing full replay check is not successful for this log")
    if digest(normal_png) != digest(check_png):
        raise RuntimeError("normal view differs from the checked full replay")
    final = out / f"{ident}-finished.png"
    normal = out / f"{ident}-normal.png"
    for original, target in [(finished_png, final), (normal_png, normal)]:
        if target.exists() and digest(target) != digest(original):
            raise RuntimeError(f"existing deliverable differs: {target.name}")
        if not target.exists():
            shutil.copyfile(original, target)
    receipt = {"id": ident, "source_log_sha256": source_hash, "chunks": chunks,
               "finished_log_sha256": digest(finished_log), "finished_chunks": chunks + 1,
               "existing_replay_check": checks[-1], "finished_png": png(final),
               "normal_png": png(normal), "finishing": "automatic drying, 0.4 varnish coats and cracks; no relief",
               "privacy": "PNG text metadata inspected; visible image/video review required before publication"}
    if item.get("blocker"):
        receipt.update(status="video blocked", blocker=item["blocker"], verified_at=stamp())
        write_json(out / "receipt.json", receipt)
        return receipt
    engine = Path(item["engine"]).expanduser()
    checkout = Path(item["checkout"]).expanduser()
    head = verify_checkout(item)
    receipt.update(source_commit=head, engine_sha256=digest(engine))
    if item.get("verified_untracked_files"):
        receipt["verified_untracked_files"] = item["verified_untracked_files"]
    video = out / f"{ident}.mp4"
    old = out / "receipt.json"
    if old.exists():
        prior = json.loads(old.read_text())
        if prior.get("status") == "ready" and prior.get("source_log_sha256") == source_hash and prior.get("engine_sha256") == receipt["engine_sha256"] and video.exists() and digest(video) == prior["video"]["sha256"]:
            probe(video)
            print(f"REUSED {ident}", flush=True)
            return prior
    if video.exists():
        raise RuntimeError("unverified existing video; refusing to overwrite it")
    scratch = Path(tempfile.mkdtemp(prefix=f"stillwet-{ident}-"))
    partial = out / f"{ident}.partial.mp4"
    started = time.monotonic()
    try:
        frames = scratch / "frames"
        cmd = [checkout / "scripts/replay_clip", finished_log, partial, "--every", "60",
               "--pace", "dynamic", "--gamma", "0.4", "--ramp", "0.3", "--length", "20",
               "--open-hold", "0.25", "--width", "1920", "--frames-dir", frames,
               "--easel", engine]
        with (out / "replay-clip.log").open("w") as log:
            run(cmd, checkout, log)
            # The workflow records all chunks at original simulation resolution.
            # Reuse of the existing finished render requires exact full-replay bytes.
            if digest(frames / "final-full.png") != digest(finished_png):
                raise RuntimeError("full finished replay differs from the existing finished PNG")
            replay_log = frames.with_suffix(".log").read_text()
            if f"({chunks + 1} chunks," not in replay_log:
                raise RuntimeError("missing successful full finished replay receipt")
            rows = (frames / "frames.tsv").read_text().splitlines()
            if int(rows[-1].split("\t")[3]) != chunks + 1:
                raise RuntimeError("frame index does not reach the finishing chunk")
            width, height = receipt["finished_png"]["width"], receipt["finished_png"]["height"]
            vp = probe(partial)
            vs = vp["streams"][0]
            if abs(vs["height"] - vs["width"] * height / width) > 2:
                raise RuntimeError("clip does not preserve the painting aspect ratio")
            run(["ffmpeg", "-v", "error", "-i", partial, "-f", "null", "-"], checkout, log)
            for number, second in enumerate([0, 6, 12, 19.5]):
                run(["ffmpeg", "-v", "error", "-y", "-ss", str(second), "-i", partial,
                     "-frames:v", "1", "-vf", "scale=640:-2", scratch / f"review-{number}.png"], checkout, log)
            run(["ffmpeg", "-v", "error", "-y", "-framerate", "1", "-i", scratch / "review-%d.png",
                 "-vf", "tile=2x2:padding=4:color=white", "-frames:v", "1", out / "video-review.png"], checkout, log)
        if digest(source) != source_hash:
            raise RuntimeError("painting changed during rendering")
        # Preserve receipts, but scrub diagnostic paths before they leave scratch.
        (out / "full-replay.log").write_text(scrub(replay_log))
        (out / "frames.tsv").write_text("\n".join(rows) + "\n")
        cliplog = out / "replay-clip.log"
        cliplog.write_text(scrub(cliplog.read_text()))
        partial.replace(video)
        receipt.update(status="ready", video=vp, frame_count=len(rows) - 1,
                       replay_seconds=round(time.monotonic() - started, 3),
                       full_finished_replay_byte_exact=True, verified_at=stamp(),
                       clip_options={"every": 60, "pace": "dynamic", "gamma": 0.4,
                                     "ramp": 0.3, "length": 20, "open_hold": 0.25,
                                     "width": 1920, "crf": 23, "fps": 24})
        write_json(out / "receipt.json", receipt)
        return receipt
    finally:
        partial.unlink(missing_ok=True)
        shutil.rmtree(scratch)


def main():
    queue = json.loads(QUEUE.read_text())
    state = json.loads(STATE.read_text()) if STATE.exists() else {"items": {}}
    lockdir = Path(os.environ.get("LOCKRUN_DIR", f"/tmp/lockrun-{os.getuid()}"))
    lockdir.mkdir(mode=0o700, exist_ok=True)
    with (lockdir / "lock").open("a") as lock:
        # Same machine-wide lock as scripts/lockrun; fail immediately if occupied.
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        for item in queue["items"]:
            ident = item["id"]
            print(f"START {ident} {stamp()}", flush=True)
            state.update(status="running", current=ident, updated_at=stamp())
            state.pop("error", None)
            write_json(STATE, state)
            try:
                free = disk()
                receipt = process(item)
                state["items"][ident] = {"status": receipt["status"], "free_bytes_before": free,
                                        "receipt": item["output_dir"] + "/receipt.json"}
                disk()
                print(f"ITEM_COMPLETE {ident} {receipt['status']}", flush=True)
            except Exception as e:
                state.update(status="failed", error=scrub(e), updated_at=stamp())
                state["items"][ident] = {"status": "failed", "error": scrub(e)}
                write_json(STATE, state)
                print(f"FAILED {ident}: {scrub(e)}", flush=True)
                return 1
            write_json(STATE, state)
    state.update(status="completed with recorded blockers" if any(i["status"] != "ready" for i in state["items"].values()) else "completed",
                 current=None, updated_at=stamp())
    write_json(STATE, state)
    print("BATCH_COMPLETE " + state["status"], flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
