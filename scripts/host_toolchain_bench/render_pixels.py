import hashlib
import json
from pathlib import Path
import subprocess
import sys

from PIL import Image

repo, result, exe = map(Path, sys.argv[1:])
cases = repo / "crates/easel/tests/old_logs"
output = result / "pixels"
output.mkdir()
manifest = {}
for line in (cases / "cases.tsv").read_text().splitlines():
    if not line or line.startswith("#"):
        continue
    name, log, width, *_ = line.split("\t")
    for suffix in ("", ".repeat"):
        with (output / f"{name}{suffix}.log").open("w") as capture:
            subprocess.run(
                [str(exe), "run", str(cases / log), "--width", width,
                 "--out", str(output / f"{name}{suffix}.png"),
                 "--state-digest", str(output / f"{name}{suffix}.dig")],
                cwd=repo, stdout=capture, stderr=subprocess.STDOUT, check=True,
            )
    with Image.open(output / f"{name}.png") as first:
        first = first.convert("RGBA")
    with Image.open(output / f"{name}.repeat.png") as second:
        second = second.convert("RGBA")
    if first.size != second.size or first.tobytes() != second.tobytes():
        raise AssertionError(f"{name}: same-binary decoded pixels differ")
    manifest[name] = {
        "size": first.size,
        "rgba_sha256": hashlib.sha256(first.tobytes()).hexdigest(),
    }
    print(f"{name}: repeat decoded RGBA pixels identical", flush=True)
(output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(f"pixels: {len(manifest)} historical cases rendered twice with exact repeats")
