import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import time

base = Path.home() / ".capy/work/wc-timeouts"
root = base / "history/rw1"
if root.exists():
    raise RuntimeError(f"Refusing to reuse {root}")
root.mkdir(parents=True)
(root / "box").write_text("watercolor\n")
rows = json.loads((base / "census.json").read_text())
env = {"PATH": "/usr/bin:/bin", "HOME": str(Path.home()), "RAYON_NUM_THREADS": "4"}
timings = []
for row in rows:
    if row["lane"] != 1 or row["line"] > 792 or row["arguments"].get("scratch"):
        continue
    if row["line"] in [628, 792]:
        destination = base / f"history/before-L{row['line']}.json"
        subprocess.run(["/bin/cp", "-c", str(root / "checkpoint.json"), str(destination)], check=True)
        print(f"Saved historical replay state {destination}", flush=True)
    if row.get("result", {}).get("isError") or row["line"] == 792:
        continue
    if shutil.disk_usage(base).free < 15 * 1024**3:
        raise RuntimeError("Less than 15 GiB free")
    source = base / "all-calls" / f"{row['name']}.lua"
    command = [str(base / "watercolor-studio"), "--box", "watercolor", "run", str(root), str(source)]
    started = time.perf_counter()
    with (root / f"{row['name']}.out").open("w") as stdout, (root / f"{row['name']}.err").open("w") as stderr:
        result = subprocess.run(command, stdout=stdout, stderr=stderr, env=env, timeout=900)
    timing = dict(name=row["name"], command=command, wall_seconds=time.perf_counter()-started, returncode=result.returncode, source_sha256=hashlib.sha256(source.read_bytes()).hexdigest())
    timings.append(timing)
    (root / "timings.json").write_text(json.dumps(timings, indent=2))
    print(json.dumps(timing), flush=True)
    if result.returncode:
        raise RuntimeError(f"Historical replay failed: {row['name']}")
