import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument("name")
parser.add_argument("--limit", type=float, default=900)
parser.add_argument("--sample", action="store_true")
parser.add_argument("--profile", action="store_true")
parser.add_argument("--label")
parser.add_argument("--checkpoint", type=Path)
args = parser.parse_args()
base = Path.home() / ".capy/work/wc-timeouts"
manifest = json.loads((base / "manifest.json").read_text())
item = next(item for item in manifest if item["name"] == args.name)
lane = item["lane"]
original = Path(f"/Users/sarah/src/a/stillwet-runs/rw{lane}-watercolor/paint-studio-8ee1cde{lane}")
root = base / "reproductions" / (args.label or args.name)
if root.exists():
    raise RuntimeError(f"Refusing to reuse {root}")
root.mkdir(parents=True)
if shutil.disk_usage(base).free < 15 * 1024**3:
    raise RuntimeError("Less than 15 GiB free")
for name in ["box", "checkpoint.json"]:
    origin = args.checkpoint if name == "checkpoint.json" and args.checkpoint else original / name
    subprocess.run(["/bin/cp", "-c", str(origin), str(root / name)], check=True)
binary = base / "target/release/watercolor-studio" if args.profile else base / "watercolor-studio"
if not binary.exists():
    shutil.copy2(original / "bin/watercolor-studio", binary)
source = base / f"{args.name}.lua"
command = [str(binary), "--box", "watercolor", "run", str(root), str(source)]
for key, flag in [("scratch", "--scratch"), ("new_scratch", "--new")]:
    if item["arguments"].get(key):
        command.append(flag)
env = {"PATH": "/usr/bin:/bin", "HOME": str(Path.home()), "RAYON_NUM_THREADS": "4"}
started = time.perf_counter()
with (root / "stdout.txt").open("w") as stdout, (root / "stderr.txt").open("w") as stderr:
    process = subprocess.Popen(command, stdout=stdout, stderr=stderr, env=env)
    (root / "pid").write_text(str(process.pid))
    sampled = False
    timed_out = False
    while process.poll() is None:
        elapsed = time.perf_counter() - started
        if args.sample and not sampled and elapsed > 30:
            subprocess.run(["/usr/bin/sample", str(process.pid), "10", "10", "-file", str(root / "sample.txt")], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            sampled = True
        if elapsed > args.limit:
            timed_out = True
            process.terminate()
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
            break
        if shutil.disk_usage(base).free < 15 * 1024**3:
            process.terminate()
            process.wait()
            raise RuntimeError("Disk fell below 15 GiB")
        time.sleep(1)
elapsed = time.perf_counter() - started
result = dict(name=args.name, command=command, env=env, wall_seconds=elapsed, limit=args.limit, timed_out=timed_out, returncode=process.returncode, binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(), state_basis=str(args.checkpoint) if args.checkpoint else "Copied final main checkpoint; new scratch receives original paper dimensions/kind/seed. Historical main state not retained.", stdout=(root / "stdout.txt").read_text(), stderr=(root / "stderr.txt").read_text())
(root / "timing.json").write_text(json.dumps(result, indent=2))
print(json.dumps(result, indent=2))
