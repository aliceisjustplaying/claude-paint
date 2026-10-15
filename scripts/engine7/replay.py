import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time


def sha256(path):
    return hashlib.file_digest(Path(path).open("rb"), "sha256").hexdigest()


def program(source, engine=None, chunks=None, finishing=False):
    lines = source.splitlines(keepends=True)
    result = []
    count = 0
    for line in lines:
        if line.lstrip().startswith("--@ chunk"):
            if "(finishing)" in line and not finishing:
                break
            count += 1
            if chunks is not None and count > chunks:
                break
        if line.lstrip().startswith("--@ engine") and count == 0:
            if engine is not None:
                continue
        result.append(line)
    if engine is not None:
        result.insert(0, f"--@ engine {engine}\n")
    return "".join(result), min(count, chunks) if chunks is not None else count


def replay(args):
    source = args.source.resolve()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    text, chunks = program(source.read_text(), args.engine, args.chunks, args.finishing)
    prepared = out / "input.lua"
    prepared.write_text(text)
    command = [str(args.easel.resolve()), "run", str(prepared), "--width", str(args.width),
               "--gallery", "--out", str(out / "gallery.png")]
    if args.checkpoint:
        command.extend(["--checkpoint-out", str(out / "final.ckpt")])
    if args.digest:
        command.extend(["--state-digest", str(out / "digests.txt")])
    env = dict(os.environ, RAYON_NUM_THREADS=str(args.workers), EASEL_ROOT=str(out), EASEL_GROUND_CACHE="off")
    start = time.monotonic()
    with (out / "stdout.txt").open("w") as stdout, (out / "stderr.txt").open("w") as stderr:
        child = subprocess.Popen(command, env=env, stdout=stdout, stderr=stderr)
        _, status, usage = os.wait4(child.pid, 0)
        child.returncode = os.waitstatus_to_exitcode(status)
    wall = time.monotonic() - start
    stderr = (out / "stderr.txt").read_text()
    chunk_times = [{"index": int(n), "wall_s": float(t)} for n, t in
                   re.findall(r"^\s+chunk\s+(\d+)\s+([\d.]+)s$", stderr, re.MULTILINE)]
    cpu = usage.ru_utime + usage.ru_stime
    size = re.search(r"canvas\s*\{\s*size\s*=\s*([\d.]+)", text)
    selected_engine = re.search(r"^\s*--@ engine\s+(\d+)", text, re.MULTILINE)
    receipt = {
        "schema": "engine7-bench-v1", "measured": True, "command": command,
        "env": {k: env[k] for k in ("RAYON_NUM_THREADS", "EASEL_ROOT", "EASEL_GROUND_CACHE")},
        "source": str(source), "source_sha256": sha256(source),
        "painting": source.stem, "input_sha256": sha256(prepared), "revision": args.revision,
        "physical_width_mm": float(size[1]) if size else None, "backend": "auto",
        "prepared_sha256": sha256(prepared), "binary_sha256": sha256(args.easel),
        "engine": int(selected_engine[1]) if selected_engine else 1,
        "workers": args.workers, "width_px": args.width,
        "expected_chunks": chunks, "completed_chunks": len(chunk_times),
        "wall_s": wall, "cpu_s": cpu, "cpu_over_wall": cpu / wall,
        "peak_rss_bytes": usage.ru_maxrss * (1 if sys.platform == "darwin" else 1024),
        "exit_code": child.returncode, "chunks": chunk_times,
        "chunk_time_precision_s": 0.01, "ready_width": None,
        "boundary": "process wall, including startup, Gallery render and PNG; no frames",
        "render": {"mode": "gallery", "azimuth": 115, "elevation": 55, "strength": 1},
    }
    (out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({k: receipt[k] for k in ("wall_s", "cpu_over_wall", "peak_rss_bytes", "exit_code")}))
    return child.returncode


def main():
    parser = argparse.ArgumentParser(description="Matched engine replay receipts; source logs stay read-only")
    sub = parser.add_subparsers(dest="action", required=True)
    p = sub.add_parser("prepare")
    p.add_argument("source", type=Path)
    p.add_argument("output", type=Path)
    p.add_argument("--engine", type=int, choices=[5, 7])
    p.add_argument("--chunks", type=int)
    p.add_argument("--finishing", action="store_true")
    r = sub.add_parser("replay")
    r.add_argument("source", type=Path)
    r.add_argument("--easel", type=Path, required=True)
    r.add_argument("--output", type=Path, required=True)
    r.add_argument("--engine", type=int, choices=[5, 7])
    r.add_argument("--chunks", type=int)
    r.add_argument("--finishing", action="store_true")
    r.add_argument("--workers", type=int, default=4)
    r.add_argument("--width", type=int, default=2400)
    r.add_argument("--checkpoint", action="store_true")
    r.add_argument("--digest", action="store_true")
    r.add_argument("--revision", help="Revision used to build this binary, not the current checkout")
    args = parser.parse_args()
    if args.chunks is not None and args.chunks < 1:
        parser.error("--chunks must be positive")
    if args.action == "prepare":
        text, chunks = program(args.source.read_text(), args.engine, args.chunks, args.finishing)
        with args.output.open("x") as f:
            f.write(text)
        print(json.dumps({"chunks": chunks, "sha256": sha256(args.output), "bytes": args.output.stat().st_size}))
        return 0
    if args.workers < 1:
        parser.error("--workers must be positive")
    return replay(args)


if __name__ == "__main__":
    sys.exit(main())
