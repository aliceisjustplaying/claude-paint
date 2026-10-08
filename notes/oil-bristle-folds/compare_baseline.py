#!/usr/bin/env python3
"""Locked, local baseline proof; restores both files even if a check fails.

Uses only this worktree and its target/. Temporary baseline PNGs are removed.
No other checkout or expected output is edited. Run after the candidate study:
  uv run --no-project scripts/lockrun --timeout 600 -- \
    uv run --no-project python notes/oil-bristle-folds/compare_baseline.py
"""
import csv
import hashlib
import json
import os
import pathlib
import subprocess
import tempfile
import time

BASE = "8eeb9bec3ce146bcd15c37db447665391fb95bdd"
ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "notes/oil-bristle-folds"
BRISTLE = ROOT / "crates/paint/src/bristle.rs"
EXAMPLE = ROOT / "crates/paint/examples/bristle_folds.rs"
assert os.environ.get("LOCKRUN_TOKEN"), "run through scripts/lockrun"
assert ROOT.name == "claude-paint-oil-bristle-folds"
assert subprocess.check_output(["git", "branch", "--show-current"], cwd=ROOT, text=True).strip() == "experiment/oil-bristle-folds"
original, example = BRISTLE.read_bytes(), EXAMPLE.read_bytes()
start = time.monotonic()
try:
    BRISTLE.write_bytes(subprocess.check_output(["git", "show", BASE + ":crates/paint/src/bristle.rs"], cwd=ROOT))
    EXAMPLE.write_bytes(example.replace(b"c.drag_bristle_folds(h, g, None)", b"c.drag(h, g, None)"))
    subprocess.run(["cargo", "build", "--profile", "iter", "-p", "paint", "--example", "bristle_folds"], cwd=ROOT, check=True, timeout=600)
    with tempfile.TemporaryDirectory(prefix="fold-baseline-") as tmp:
        with open(pathlib.Path(tmp) / "metrics.csv", "w") as metrics:
            subprocess.run([str(ROOT / "target/iter/examples/bristle_folds"), tmp], cwd=ROOT, stdout=metrics, check=True, timeout=60)
        files = sorted(OUT.glob("*-baseline-*.png"))
        assert len(files) == 24
        for f in files:
            assert f.read_bytes() == (pathlib.Path(tmp) / f.name).read_bytes(), f.name
        with open(OUT / "metrics.csv") as a, open(pathlib.Path(tmp) / "metrics.csv") as b:
            expected = [r for r in csv.DictReader(a) if r["mode"] == "baseline"]
            actual = [r for r in csv.DictReader(b) if r["mode"] == "baseline"]
            for x, y in zip(expected, actual, strict=True):
                assert {k: v for k, v in x.items() if k != "seconds"} == {k: v for k, v in y.items() if k != "seconds"}
        if "--contracts" in __import__("sys").argv:
            subprocess.run(["cargo", "build", "--release", "-p", "easel"], cwd=ROOT, check=True, timeout=600)
            subprocess.run(["cargo", "test", "--release", "-p", "paint", "--test", "thinner_pigments", "--no-run"], cwd=ROOT, check=True, timeout=600)
            contracts = []
            for command in [
                ["scripts/tests/baseline_state.sh", "target/release/easel"],
                ["cargo", "test", "--release", "-p", "paint", "--test", "thinner_pigments", "--", "--exact", "c13_every_pigment_value_is_af49348s"],
            ]:
                r = subprocess.run(command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=60)
                lines = r.stdout.splitlines()
                evidence = [line for line in lines if any(word in line for word in
                    ["state equal on", "wet.vol:", "state_compare:", "historical catalog fields", "current expanded tube table", "test result:"])]
                contracts.append(dict(command=command, exit=r.returncode, evidence=evidence))
                print("baseline contract", command[0], "exit", r.returncode)
            (OUT / "baseline-contracts.json").write_text(json.dumps(dict(baseline_commit=BASE, checks=contracts), indent=2) + "\n")
        result = dict(baseline_commit=BASE, pngs_equal=24, metric_rows_equal=8,
                      seconds=round(time.monotonic() - start, 3),
                      baseline_bristle_sha256=hashlib.sha256(BRISTLE.read_bytes()).hexdigest(),
                      candidate_bristle_sha256=hashlib.sha256(original).hexdigest())
        (OUT / "baseline-proof.json").write_text(json.dumps(result, indent=2) + "\n")
        print(json.dumps(result))
finally:
    BRISTLE.write_bytes(original)
    EXAMPLE.write_bytes(example)
# Restore the runnable candidate binary in the SAME target directory.
subprocess.run(["cargo", "build", "--profile", "iter", "-p", "paint", "--example", "bristle_folds"], cwd=ROOT, check=True, timeout=600)
