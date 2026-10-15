import argparse
import hashlib
import json
from pathlib import Path
import tarfile

parser = argparse.ArgumentParser()
parser.add_argument("work", type=Path)
parser.add_argument("output", type=Path)
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)
runs = []
receipts = []
for path in sorted(args.work.glob("*.json")):
    value = json.loads(path.read_text())
    if isinstance(value, dict) and "command" in value and "wall_seconds" in value:
        runs.append({"name": path.stem, **value})
        receipts.append(path)
        receipts.extend(p for p in [path.with_suffix(".stdout"), path.with_suffix(".stderr")] if p.exists())
images = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(args.work.glob("*.png"))}
result = {
    "runs": runs,
    "summed_command_wall_seconds": sum(r["wall_seconds"] for r in runs),
    "summed_child_cpu_seconds": sum(r["user_seconds"] + r["system_seconds"] for r in runs),
    "image_sha256": images,
    "notes": "Single trials unless a stage log identifies in-process repetitions. Linux peak RSS is KiB. Separate Mac and hyper receipts retain their own units and provenance. Summed command time is not elapsed research time or billing.",
}
(args.output / "measurements.json").write_text(json.dumps(result, indent=2) + "\n")
for name in ["frame-summary.json", "dependency-model.json", "perf-self.txt", "scheduler-tests.log", "clip-schedule.json", "encode-slow-probe.json", "encode-veryfast-probe.json"]:
    path = args.work / name
    if path.exists():
        receipts.append(path)
receipts.extend(p for p in (args.work / "mac-receipts").glob("*") if p.suffix != ".png")
with tarfile.open(args.output / "receipts.tar.gz", "w:gz") as archive:
    for path in receipts:
        archive.add(path, arcname=str(path.relative_to(args.work)))
print(json.dumps({k: v for k, v in result.items() if k not in ["runs", "image_sha256"]}, indent=2))
