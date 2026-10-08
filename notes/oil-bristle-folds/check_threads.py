#!/usr/bin/env python3
"""Tiny deterministic study on one and four threads, using the own-target binary."""
import csv
import json
import os
import pathlib
import subprocess
import tempfile
import time

root = pathlib.Path(__file__).resolve().parents[2]
out = root / "notes/oil-bristle-folds"
assert os.environ.get("LOCKRUN_TOKEN"), "run through scripts/lockrun"
start = time.monotonic()
files = sorted(out.glob("*-lit.png")) + sorted(out.glob("*-flat.png")) + sorted(out.glob("*-wet-height.png"))
assert len(files) == 48
with open(out / "metrics.csv") as f:
    expected = [{k: v for k, v in r.items() if k != "seconds"} for r in csv.DictReader(f)]
for threads in [1, 4]:
    with tempfile.TemporaryDirectory(prefix="fold-threads-") as tmp:
        with open(pathlib.Path(tmp) / "metrics.csv", "w") as metrics:
            subprocess.run([str(root / "target/iter/examples/bristle_folds"), tmp], cwd=root,
                           env=dict(os.environ, RAYON_NUM_THREADS=str(threads)), stdout=metrics, check=True, timeout=60)
        for f in files:
            assert f.read_bytes() == (pathlib.Path(tmp) / f.name).read_bytes(), (threads, f.name)
        with open(pathlib.Path(tmp) / "metrics.csv") as f:
            actual = [{k: v for k, v in r.items() if k != "seconds"} for r in csv.DictReader(f)]
        assert actual == expected
        assert (out / "drying.csv").read_bytes() == (pathlib.Path(tmp) / "drying.csv").read_bytes()
result = dict(threads=[1, 4], pngs_equal_per_run=48, metric_rows_equal_per_run=16,
              drying_rows_equal_per_run=16, seconds=round(time.monotonic() - start, 3))
(out / "thread-proof.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result))
