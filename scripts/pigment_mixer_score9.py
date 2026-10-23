# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1", "pillow==12.3.0"]
# ///
"""Score every modeled Revigo source row using physical volume recipes and engine-9 K/S.

Black mixtures are explicitly reported as fitted, not held out. The revised-12 partition remains
disjoint from fitting but has model-selection exposure. Wet-paint densities are estimates.
"""

import argparse
import csv
import json
import subprocess
from pathlib import Path

import build_pigment_tables as gen
import numpy as np
from calibrate_pigments9 import mass_to_volume
from pigment_basis_select import measured_colour, oklab
from pigment_mixer_score import hexof, sheet


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mixpairs", nargs="?")
    parser.add_argument("--python-only", action="store_true", help="Coefficient oracle only; not runtime validation")
    parser.add_argument("--before", type=Path)
    parser.add_argument("--strengths", type=Path)
    parser.add_argument("--tables", type=Path, default=gen.OUT)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--csv", type=Path, required=True)
    parser.add_argument("--json", type=Path)
    parser.add_argument("--sheet", type=Path)
    args = parser.parse_args()
    if not args.python_only and not args.mixpairs:
        parser.error("mixpairs is required for runtime scoring")
    meta = json.loads((args.tables / "calibrated.json").read_text())
    legacy = json.loads((gen.OUT / "measured.json").read_text())
    if args.strengths:
        strengths = json.loads(args.strengths.read_text())
        assert strengths["encoding"] == "volume-ks"
        assert strengths["paint_strengths"] == meta["linear_weights"]
    assert not set(meta["joint_fit_samples"]) & set(meta["test_set"])
    order = meta["order"]
    packed = np.frombuffer((args.tables / "calibrated.bin").read_bytes(), dtype="<f4")
    coefficients = packed[:len(order) * 2 * 38].reshape(len(order), 2, 38).astype(float)
    assert np.array_equal(packed[len(order) * 2 * 38:], np.ones(len(order)))
    densities = dict(zip(order, np.frombuffer((args.tables / "calibrated-densities.bin").read_bytes(), dtype="<f4")))
    _, spectra, samples = gen.load_data()
    matrix = gen.spectral_array("XYZ_RGB") @ gen.spectral_array("CMF")
    reference = np.array(meta["reference_substrate"], dtype="<f4").astype(float)
    before = {row["sid"]: row for row in csv.DictReader(args.before.open())} if args.before else {}
    rows, inputs, predictions, targets, excluded = [], [], [], [], []
    for sid, (names, fractions) in samples.items():
        present = [n for n, c in zip(names, fractions) if c > 0]
        if sid + "_ow" not in spectra or any(n not in order for n in present):
            excluded.append((sid, " + ".join(present), "alternate oil or unmodeled paint identity"))
            continue
        mass = fractions / fractions.sum()
        volumes = mass_to_volume(names, fractions, densities)
        concentration = np.array([sum(v for n, v in zip(names, volumes) if n == paint) for paint in order])
        k, s = np.einsum("i,ijk->jk", concentration, coefficients)
        predictions.append(np.clip(matrix @ gen.saunderson(gen.film(k, s, reference), False), 0, 1))
        targets.append(measured_colour(spectra, sid, matrix))
        first, second = present[0], present[-1]
        share = float(sum(v for n, v in zip(names, volumes) if n == second)) if len(present) > 1 else 0.0
        inputs.append(f"{first} 1 {second} 1 {share:.12f}\n")
        status = ("fitted" if sid in meta["joint_fit_samples"] else "heldout" if sid in meta["test_set"]
                  else "selection-touched" if sid in meta["selection_touched"] else "unused pure")
        status_before = ("fitted" if sid in legacy["joint_fit_samples"] else "heldout" if sid in legacy["test_set"]
                         else "selection-touched" if sid in legacy["selection_touched"] else "unused")
        rows.append({"sample": sid, "status_before": status_before, "status": status,
                     "black_mixture": len(present) == 2 and "BoneBlack" in present,
                     "recipe_by_mass": " + ".join(f"{n} {100 * c:.6f}%" for n, c in zip(names, mass) if c > 0),
                     "recipe_by_volume_estimated": " + ".join(f"{n} {100 * c:.6f}%" for n, c in zip(names, volumes) if c > 0),
                     "raw_fraction_sum": float(fractions.sum()), "measured": hexof(targets[-1]),
                     "before": before.get(sid, {}).get("actual", ""),
                     "error_before": float(before[sid]["delta"]) if sid in before else None})
    if args.python_only:
        actual = predictions
    else:
        result = subprocess.run([args.mixpairs, "--engine9"], input="".join(inputs), text=True, capture_output=True, check=True)
        actual = [np.array([float(c) for c in line.split()]) for line in result.stdout.splitlines() if line.strip()]
        assert len(actual) == len(rows), "runtime must return every source row"
    for row, target, new, oracle in zip(rows, targets, actual, predictions):
        row.update(after=hexof(new), error_after=float(np.linalg.norm(oklab(new) - oklab(target))),
                   runtime_oracle_delta=float(np.linalg.norm(oklab(new) - oklab(oracle))))
    if not args.python_only:
        assert max(r["runtime_oracle_delta"] for r in rows) < 2e-6, "compiled decoder differs from the supplied tables"
    groups = [("Revised12 heldout (historical and current selection exposure)", [r for r in rows if r["sample"] in meta["test_set"]]),
              ("Old17 (five rows now fitted)", [r for r in rows if r["sample"] in meta["old_test_set"]]),
              ("All black mixtures (all fitted, no dark holdout)", [r for r in rows if r["black_mixture"]]),
              ("All fitted source rows", [r for r in rows if r["status"] == "fitted"]),
              ("Selection-touched, not fitted", [r for r in rows if r["status"] == "selection-touched"]),
              ("Unused pure Prussian13A", [r for r in rows if r["status"] == "unused pure"]),
              ("Every modeled source row", rows)]
    lines = ["# Engine9 physical-volume calibration", "",
             "Runtime decoder checked against the packed-table oracle." if not args.python_only else "Python coefficient oracle only; runtime not checked.", "",
             "All wet-paint densities are estimates. Source recipes are unchanged mass fractions, converted to volume with the documented density model.",
             "No revised12 row is fitted. These are model-selection-exposed rows, not newly blind validation. Alternate-oil07B/14F have no modeled identity and are excluded.", ""]
    records = []
    for name, members in groups:
        errors = [r["error_after"] for r in members]
        lines += [f"## {name}", "",
                  f"n={len(members)}; mean {np.mean(errors):.6f}; worst {max(errors):.6f} ({max(members, key=lambda r: r['error_after'])['sample']})."]
        if all(r["error_before"] is not None for r in members):
            lines.append(f"Before: mean {np.mean([r['error_before'] for r in members]):.6f}; worst {max(r['error_before'] for r in members):.6f}.")
        lines += ["", "| sample | status before → after | source mass recipe | measured | before | after | before ΔOKLab | after ΔOKLab |",
                  "|---|---|---|---|---|---|---:|---:|"]
        for r in members:
            old = f"{r['error_before']:.6f}" if r["error_before"] is not None else "—"
            lines.append(f"| {r['sample']} | {r['status_before']} → {r['status']} | {r['recipe_by_mass']} | {r['measured']} | {r['before']} | {r['after']} | {old} | {r['error_after']:.6f} |")
        lines.append("")
        records.append({"group": name, "rows": members})
    lines += ["## Unmodeled source rows", "", "| sample | identity | why no before/after prediction |", "|---|---|---|"]
    lines += [f"| {sid} | {identity} | {reason} |" for sid, identity, reason in excluded]
    with args.csv.open("w") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    args.out.write_text("\n".join(lines) + "\n")
    if args.json:
        args.json.write_text(json.dumps(records, indent=2) + "\n")
    if args.sheet:
        swatches = []
        for r, target, new in zip(rows, targets, actual):
            old_rgb = None
            if r["before"]:
                c = np.array([int(r["before"][i:i + 2], 16) / 255 for i in (1, 3, 5)])
                old_rgb = np.where(c <= .04045, c / 12.92, ((c + .055) / 1.055) ** 2.4)
            swatches.append((f"{r['sample']} {r['status']}", target, new, old_rgb, r["error_after"], r["error_before"]))
        sheet(args.sheet, [("All source rows: measured | calibrated9 | baseline", swatches)])
    print(f"Scored {len(rows)} rows; " + ("coefficient oracle only" if args.python_only else
          f"maximum runtime/oracle delta {max(r['runtime_oracle_delta'] for r in rows):.9g}"))
    for name, members in groups:
        print(f"{name}: n={len(members)} mean={np.mean([r['error_after'] for r in members]):.6f} worst={max(r['error_after'] for r in members):.6f}")


if __name__ == "__main__":
    main()
