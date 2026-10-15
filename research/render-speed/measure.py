import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument("--result", type=Path, required=True)
parser.add_argument("--timeout", type=float, default=2400)
parser.add_argument("command", nargs=argparse.REMAINDER)
args = parser.parse_args()
command = args.command[1:] if args.command[:1] == ["--"] else args.command
args.result.parent.mkdir(parents=True, exist_ok=True)
before = resource.getrusage(resource.RUSAGE_CHILDREN)
start = time.monotonic()
with args.result.with_suffix(".stdout").open("w") as stdout, args.result.with_suffix(".stderr").open("w") as stderr:
    process = subprocess.Popen(command, stdout=stdout, stderr=stderr)
    try:
        code = process.wait(timeout=args.timeout)
    except subprocess.TimeoutExpired:
        process.terminate()
        process.wait()
        code = "timeout"
after = resource.getrusage(resource.RUSAGE_CHILDREN)
result = {
    "command": command,
    "environment": {k: os.environ[k] for k in ["RAYON_NUM_THREADS", "RENDER_PROFILE", "RENDER_DRY_FAST", "RENDER_INTEGER_POWER", "RENDER_LIVE", "RENDER_SCHED_PROFILE", "RENDER_STROKE_PROFILE", "RENDER_TILE_SCALE", "RENDER_SLICE_FACTOR", "RENDER_STEP_SCALE", "RENDER_REORDER"] if k in os.environ},
    "wall_seconds": time.monotonic() - start,
    "user_seconds": after.ru_utime - before.ru_utime,
    "system_seconds": after.ru_stime - before.ru_stime,
    "peak_rss_native_units": after.ru_maxrss,
    "exit_code": code,
}
args.result.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
raise SystemExit(0 if code == 0 else 1)
