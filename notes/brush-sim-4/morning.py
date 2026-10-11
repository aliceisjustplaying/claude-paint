#!/usr/bin/env python3
import argparse
import csv
import hashlib
import json
import os
import re
import shutil
import subprocess
from pathlib import Path

VARIANTS = {"baseline": 5, "body3": 13, "lean4": 15, "body4": 16, "wild4": 17}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("binary", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--tolerance", type=float, default=0.0003)
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    source = (repo / "notes/brush-sim-3/morning.lua").read_text()
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=True)
    binary = args.binary.resolve()
    receipt = {"commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip(), "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(), "source_sha256": hashlib.sha256(source.encode()).hexdigest(), "width": 1200, "strokes": 18, "volume_definition": "final material_total minus primed material_total, coats times square canvas units; not physical mass", "matching_scope": "whole study, common multiplier for all requested brush loads; ground and strokes unchanged", "runs": [], "selected": {}}
    cache = {}

    def run(name, factor):
        key = (name, factor)
        if key in cache:
            return cache[key]
        directory = root / "calibration" / f"{name}-{len(receipt['runs']):03d}"
        directory.mkdir(parents=True)
        text = source.replace("--@ engine 5", f"--@ engine {VARIANTS[name]}", 1)
        text = text.replace("--@ chunk 1", "--@ chunk 1\nfunction finite_load(h,p,n)\n  while n > 0 do\n    local dip=math.min(n,1)\n    h:load(p,dip)\n    n=n-dip\n  end\nend", 1)
        text = re.sub(r"(\w+):load\((\w+),([0-9.]+)\)", lambda m: f"finite_load({m[1]},{m[2]},{float(m[3]) * factor:.17g})", text)
        lua = directory / "study.lua"
        lua.write_text(text)
        command = [str(binary), "run", str(lua), "--width", "1200", "--gallery", "--also-color", str(directory / "color.png"), "--out", str(directory / "gallery.png"), "--material-report", str(directory / "materials.csv"), "--save-final-canvas", str(directory / "final.ckpt"), "--state-digest", str(directory / "digests.txt")]
        env = {**os.environ, "EASEL_ROOT": str(root / "runtime"), "EASEL_BRUSH_SIM": "off", "RAYON_NUM_THREADS": "2"}
        env.pop("EASEL_BOX", None)
        result = subprocess.run(command, env=env, cwd=repo, capture_output=True, text=True, check=True)
        (directory / "stdout.txt").write_text(result.stdout)
        (directory / "stderr.txt").write_text(result.stderr)
        rows = list(csv.DictReader((directory / "materials.csv").open()))
        column = "material_total_volume_proxy_coats_times_square_units"
        volume = float(rows[-1][column]) - float(rows[0][column])
        row = {"variant": name, "engine": VARIANTS[name], "factor": factor, "deposited_volume": volume, "initial_volume": float(rows[0][column]), "final_volume": float(rows[-1][column]), "directory": str(directory), "invocations": 1, "source_sha256": hashlib.sha256(text.encode()).hexdigest()}
        receipt["runs"].append(row)
        cache[key] = row
        (root / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        print(name, factor, volume, flush=True)
        return row

    equal = {name: run(name, 1.0) for name in VARIANTS}
    target = equal["baseline"]["deposited_volume"]
    receipt["target"] = target
    receipt["equal_input"] = equal
    for name in VARIANTS:
        best = equal[name]
        lo, hi = 0.0, 1.0
        while best["deposited_volume"] < target * (1 - args.tolerance) and hi < 8:
            hi *= 2
            best = run(name, hi)
        if best["deposited_volume"] < target * (1 - args.tolerance):
            raise RuntimeError(f"{name}: baseline volume unreachable at factor {hi}; choose an explicit common lower target")
        for _ in range(16):
            if abs(best["deposited_volume"] / target - 1) <= args.tolerance:
                break
            factor = (lo + hi) / 2
            row = run(name, factor)
            if row["deposited_volume"] < target:
                lo = factor
            else:
                hi = factor
            if abs(row["deposited_volume"] - target) < abs(best["deposited_volume"] - target):
                best = row
        if abs(best["deposited_volume"] / target - 1) > args.tolerance:
            raise RuntimeError(f"{name}: failed volume match: {best}")
        best["relative_error"] = best["deposited_volume"] / target - 1
        receipt["selected"][name] = best
        destination = root / "matched" / name
        destination.mkdir(parents=True, exist_ok=True)
        for file in Path(best["directory"]).iterdir():
            shutil.copy2(file, destination / file.name)
        for row in receipt["runs"]:
            if row["variant"] == name:
                (Path(row["directory"]) / "final.ckpt").unlink(missing_ok=True)
        (root / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("Matched final states retained without another replay", flush=True)


if __name__ == "__main__":
    main()
