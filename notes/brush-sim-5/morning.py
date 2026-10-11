#!/usr/bin/env python3
import argparse
import fcntl
import hashlib
import json
import math
import os
import re
import shutil
import subprocess
from pathlib import Path

from common import (FILES, REPO, TARGET, TOLERANCE, VARIANTS, artifacts,
                    materials, scratch, sha256, verify, write_json)


def study(source, engine, factor):
    if not math.isfinite(factor) or not 0 < factor <= 8:
        raise ValueError("Load factor must be finite and in (0, 8]")
    if source.count("--@ engine 5") != 1 or source.count(":stroke(") != 18:
        raise ValueError("Unexpected morning study engine or gesture count")
    text = source.replace("--@ engine 5", f"--@ engine {engine}", 1)
    text = text.replace("--@ chunk 1", "--@ chunk 1\nfunction finite_load(h,p,n)\n  while n > 0 do\n    local dip=math.min(n,1)\n    h:load(p,dip)\n    n=n-dip\n  end\nend", 1)
    text, count = re.subn(r"(\w+):load\((\w+),([0-9.]+)\)",
                          lambda m: f"finite_load({m[1]},{m[2]},{float(m[3]) * factor:.17g})", text)
    if count != source.count(":load("):
        raise ValueError("Not every original load was replaced")
    return text


def source_identity():
    paths = subprocess.check_output(["git", "ls-files", "--", "crates/paint/src", "crates/easel/src",
                                     "crates/paint/build.rs", "crates/easel/build.rs", "crates/paint/Cargo.toml",
                                     "crates/easel/Cargo.toml", "Cargo.toml", "Cargo.lock", ".cargo", "rust-toolchain.toml"],
                                    cwd=REPO, text=True).splitlines()
    hashes = {name: sha256(REPO / name) for name in paths}
    return {"git_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO, text=True).strip(),
            "engine_source_sha256": hashlib.sha256(json.dumps(hashes, sort_keys=True).encode()).hexdigest(),
            "engine_source_files_sha256": hashes}


def prepare(root, baseline, lean4):
    if (root / "receipt.json").exists():
        saved = json.loads((root / "receipt.json").read_text())
        for name in ("baseline", "lean4"):
            verify(saved["selected"][name])
        if saved["selected"]["baseline"]["provenance"]["directory"] != str(baseline):
            raise ValueError("Baseline origin differs from prepared controls")
        if saved["selected"]["lean4"]["directory"] != str(lean4):
            raise ValueError("Lean4 origin differs from prepared controls")
        return saved
    old_receipt_path = baseline / "iteration-4-winddown-receipt.json"
    old = json.loads(old_receipt_path.read_text())
    retained = old["selected"]["baseline"]
    directory = root / "controls" / "baseline"
    directory.mkdir(parents=True, exist_ok=True)
    original_hashes = {}
    for name in FILES:
        source = baseline / f"iteration-4-winddown-baseline-{name}"
        original_hashes[source.name] = sha256(source)
        if not (directory / name).exists():
            shutil.copy2(source, directory / name)
        if sha256(directory / name) != original_hashes[source.name]:
            raise ValueError("Baseline import is not byte-identical")
    summary = materials(directory / "materials.csv", 5)
    if summary["deposited_volume"] != TARGET:
        raise ValueError("Retained baseline is not the specified original-volume control")
    if sha256(directory / "study.lua") != retained["source_sha256"]:
        raise ValueError("Baseline Lua hash disagrees with retained receipt")
    baseline_row = {"variant": "baseline", "engine": 5, "factor": 1.0, **summary,
                    "directory": str(directory), "invocations": retained["invocations"],
                    "current_harness_invocations": 0, "final_canvas": None,
                    "checkpoint_available": False, "status": "retained-control",
                    "source_sha256": retained["source_sha256"],
                    "binary_sha256": retained.get("control_binary_sha256", old["binary_sha256"]),
                    "command": None, "command_note": "Original command not recorded in retained receipt",
                    "artifact_sha256": artifacts(directory),
                    "provenance": {"directory": str(baseline), "receipt": str(old_receipt_path),
                                   "receipt_sha256": sha256(old_receipt_path), "original_files_sha256": original_hashes,
                                   "reason": "Retained original-volume engine-5 PNG/CSV/Lua/digests; checkpoint absent"}}
    control_path = lean4 / "receipt.json"
    control = json.loads(control_path.read_text())
    summary = materials(lean4 / "materials.csv", 15)
    if control["factor"] != 1.546 or control["invocations"] != 1 or not summary["matched"]:
        raise ValueError("Expected the already-replayed, matched factor-1.546 lean4 control")
    if summary["initial_volume"] != baseline_row["initial_volume"]:
        raise ValueError("Lean4 and baseline primed initial volumes differ")
    if sha256(lean4 / "study.lua") != control["source_sha256"]:
        raise ValueError("Lean4 Lua hash disagrees with receipt")
    control_binary = Path(control["command"][0])
    if not control_binary.exists() or sha256(control_binary) != control["binary_sha256"]:
        raise ValueError("Lean4 exact control binary is missing or changed")
    lean_row = {"variant": "lean4", **control, **summary, "directory": str(lean4),
                "current_harness_invocations": 0, "status": "retained-control",
                "final_canvas": str(lean4 / "final.ckpt"), "checkpoint_available": True,
                "artifact_sha256": artifacts(lean4, checkpoint=True),
                "provenance": {"receipt": str(control_path), "receipt_sha256": sha256(control_path),
                               "reason": "Reuse the one fresh factor-1.546 engine-15 replay; never repaint"}}
    saved = {"schema": 1, "target": TARGET, "tolerance": TOLERANCE, "width": 1200, "height": 800,
             "strokes": 18, "volume_definition": "Whole final material_total minus primed initial material_total; coats times square canvas units, not mass",
             "matching_scope": "One common multiplier for requested loads; finite repeated dips at most one; same seed, ground and gestures",
             "runs": [], "selected": {"baseline": baseline_row, "lean4": lean_row}}
    write_json(root / "receipt.json", saved)
    return saved


def run_candidate(root, saved, binary, source, name, factor):
    key = f"{name}-{hashlib.sha256(float(factor).hex().encode()).hexdigest()[:16]}"
    directory = root / "calibration" / key
    existing = next((row for row in saved["runs"] if row["key"] == key), None)
    if existing and existing["status"] == "completed":
        if artifacts(directory, checkpoint=True) != existing["artifact_sha256"]:
            raise ValueError(f"Cached candidate changed: {key}")
        return existing
    if existing:
        completion = directory / "process-result.json"
        if not completion.exists():
            raise RuntimeError(f"{key}: interrupted/uncertain invocation; refusing a second replay. Inspect outputs manually.")
        process = json.loads(completion.read_text())
        if process["returncode"] != 0:
            raise RuntimeError(f"{key}: previous invocation failed; no automatic repaint")
        row = existing
    else:
        if sha256(binary) != saved["calibration_identity"]["binary_sha256"] or source_identity()["engine_source_sha256"] != saved["calibration_identity"]["engine_source_sha256"]:
            raise RuntimeError("Binary or engine source changed before invocation; no replay launched")
        if directory.exists():
            raise RuntimeError(f"Untracked candidate directory exists: {directory}; no overwrite")
        directory.mkdir(parents=True)
        text = study(source, VARIANTS[name], factor)
        lua = directory / "study.lua"
        lua.write_text(text)
        command = [str(binary), "run", str(lua), "--width", "1200", "--gallery",
                   "--also-color", str(directory / "color.png"), "--out", str(directory / "gallery.png"),
                   "--material-report", str(directory / "materials.csv"),
                   "--save-final-canvas", str(directory / "final.ckpt"),
                   "--state-digest", str(directory / "digests.txt")]
        overrides = {"EASEL_ROOT": str(root / "runtime"), "EASEL_BRUSH_SIM": "off", "RAYON_NUM_THREADS": "2"}
        row = {"key": key, "variant": name, "engine": VARIANTS[name], "factor": factor,
               "invocations": 1, "invocation_count_status": "Reserved or started; uncertain until process result is recorded",
               "status": "launching", "command": command, "cwd": str(REPO),
               "environment_overrides": overrides, "environment_removed": ["EASEL_BOX"],
               "directory": str(directory), "source_sha256": sha256(lua),
               "binary_sha256": saved["calibration_identity"]["binary_sha256"],
               "engine_source_sha256": saved["calibration_identity"]["engine_source_sha256"],
               "final_canvas": str(directory / "final.ckpt"), "checkpoint_available": True}
        saved["runs"].append(row)
        saved["candidate_invocation_count"] = sum(item["invocations"] for item in saved["runs"])
        write_json(root / "receipt.json", saved)
        env = {**os.environ, **overrides}
        env.pop("EASEL_BOX", None)
        with (directory / "stdout.txt").open("w") as stdout, (directory / "stderr.txt").open("w") as stderr:
            result = subprocess.run(command, cwd=REPO, env=env, stdout=stdout, stderr=stderr)
        write_json(directory / "process-result.json", {"returncode": result.returncode, "command": command})
        row["invocation_count_status"] = "Confirmed one process invocation"
        if result.returncode:
            row["status"] = "failed"
            write_json(root / "receipt.json", saved)
            raise RuntimeError(f"{key}: invocation failed ({result.returncode}); no automatic repaint")
    row.update(materials(directory / "materials.csv", VARIANTS[name]))
    if row["initial_volume"] != saved["selected"]["baseline"]["initial_volume"]:
        raise ValueError("Candidate primed initial differs from baseline")
    row["artifact_sha256"] = artifacts(directory, checkpoint=True)
    row["invocation_count_status"] = "Confirmed one process invocation"
    row["status"] = "completed"
    saved["candidate_invocation_count"] = sum(item["invocations"] for item in saved["runs"])
    write_json(root / "receipt.json", saved)
    print(name, format(factor, ".17g"), row["deposited_volume"], row["percent_error"], flush=True)
    return row


def calibrate(root, binary, max_candidates):
    saved = json.loads((root / "receipt.json").read_text())
    for name in ("baseline", "lean4"):
        verify(saved["selected"][name])
    source_path = REPO / "notes/brush-sim-3/morning.lua"
    source = source_path.read_text()
    identity = {**source_identity(), "binary": str(binary), "binary_sha256": sha256(binary),
                "study_template_sha256": sha256(source_path)}
    if "calibration_identity" in saved:
        for key in ("binary_sha256", "engine_source_sha256", "study_template_sha256"):
            if saved["calibration_identity"][key] != identity[key]:
                raise ValueError("Calibration binary/source identity changed; use a new scratch calibration root")
    else:
        saved["calibration_identity"] = identity
    write_json(root / "receipt.json", saved)
    for name in VARIANTS:
        if name in saved["selected"]:
            verify(saved["selected"][name])
            continue
        lo, hi, factor = 0.0, None, 1.546
        for _ in range(max_candidates):
            row = run_candidate(root, saved, binary, source, name, factor)
            if row["matched"]:
                saved["selected"][name] = dict(row)
                saved["selected"][name]["provenance"] = {"reason": "Accepted one invocation; retained its final Canvas and paired views without rerun"}
                write_json(root / "receipt.json", saved)
                break
            if row["deposited_volume"] < TARGET:
                lo = factor
            else:
                hi = factor
            if hi is None:
                if factor >= 8:
                    raise RuntimeError(f"{name}: target unreachable at factor 8; target is fixed")
                factor = min(8.0, factor * 2)
            else:
                factor = (lo + hi) / 2
        else:
            raise RuntimeError(f"{name}: no match after {max_candidates} candidates")
    saved["candidate_invocation_count"] = sum(row["invocations"] for row in saved["runs"])
    saved["status"] = "matched"
    write_json(root / "receipt.json", saved)


def main():
    parser = argparse.ArgumentParser(description="Iteration-five controls and once-per-candidate calibration. Only calibrate launches replays.")
    sub = parser.add_subparsers(dest="action", required=True)
    prep = sub.add_parser("prepare", help="Import/verify controls only; no replay")
    prep.add_argument("root", type=Path)
    prep.add_argument("--baseline", type=Path, default=Path("/home/user/.capy/drive/captain-stillwet/renders/morning/brush"))
    prep.add_argument("--lean4", type=Path, default=Path("/home/user/.capy/work/brush-sim-5/control-lean4"))
    run = sub.add_parser("calibrate", help="Captain-only: launch each new candidate once, resume cached results")
    run.add_argument("binary", type=Path)
    run.add_argument("root", type=Path)
    run.add_argument("--max-candidates", type=int, default=22)
    args = parser.parse_args()
    root = scratch(args.root)
    root.mkdir(parents=True, exist_ok=True)
    with (root / ".calibration.lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        if args.action == "prepare":
            prepare(root, args.baseline.resolve(), scratch(args.lean4))
            print("Retained controls verified; no replay launched")
        else:
            if not 1 <= args.max_candidates <= 40:
                parser.error("--max-candidates must be 1..40")
            calibrate(root, args.binary.resolve(), args.max_candidates)


if __name__ == "__main__":
    main()
