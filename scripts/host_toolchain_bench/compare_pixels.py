import json
from pathlib import Path
import sys

from PIL import Image

result = Path(sys.argv[1])
baseline_name = sys.argv[2] if len(sys.argv) > 2 else "gnu"
candidates = sys.argv[3:] if len(sys.argv) > 2 else ["mold", "wild", "lld"]
reference = result / baseline_name / "pixels"
names = set(json.loads((reference / "manifest.json").read_text()))
if not names:
    raise AssertionError("no historical pixel cases")
for candidate in candidates:
    actual = result / candidate / "pixels"
    if names != set(json.loads((actual / "manifest.json").read_text())):
        raise AssertionError(f"{candidate}: historical case sets differ")
    for name in sorted(names):
        with Image.open(reference / f"{name}.png") as baseline:
            baseline = baseline.convert("RGBA")
        with Image.open(actual / f"{name}.png") as current:
            current = current.convert("RGBA")
        if baseline.size != current.size or baseline.tobytes() != current.tobytes():
            raise AssertionError(f"{candidate}/{name}: decoded pixels differ from {baseline_name}")
        print(f"{candidate}/{name}: decoded RGBA pixels identical to {baseline_name}")
print(f"pixels: all {len(names)} historical cases identical across {len(candidates) + 1} builds")
