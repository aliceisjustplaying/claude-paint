# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Calibrate tinting strengths to the measured Revigo paint mixtures, through the engine's own tables.

The split, fixed before any fit (written here, reproduced by `split_for_strengths`):
  fit set:  for every measured paint that is not one of the mixer's primaries, the zinc-white tint whose paint share is
            nearest 10 % (the usual tinting-strength ratio); for lead white, the 10.2 % bone black tone 08C, with the
            mixer's own black vertex standing in for black's masstone, which the data does not measure;
  test set: every other two-paint mixture whose pure paints are measured: the 50 % and 1 % tints and every mixture
            with lead white, barium sulphate or calcium carbonate; the primaries' own ladders (the K and S fit data
            recorded in primaries.json) and the ten mixtures scripts/pigment_basis_select.py read while choosing the
            primaries are kept out of the test and listed separately.
One free parameter per paint, its tinting strength s: a mixture of weight share t of the paint in a white is the
latent average (1 - t) * lat(white) + t * s * lat(paint), normalised, exactly Palette::pile's rule. s is the value
whose prediction is nearest the measured fit tint in OKLab (log-spaced grid, then a fine grid). The same fit, with
the catalog tube's own colour in place of the measured masstone, gives the strength of each catalog tube that has a
measured counterpart. Nothing in the fit looks at a test sample or at any look target.

    cargo build --release -p paint --example mixpairs
    uv run --script scripts/pigment_strength_fit.py target/release/examples/mixpairs --out scripts/pigment_strengths.json
"""

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path

os.environ["OPENBLAS_NUM_THREADS"] = "1"

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_pigment_tables as gen
from pigment_basis_select import SELECT_SERIES, measured_colour, oklab, series_of

ROOT = Path(__file__).resolve().parents[1]
WHITES = {"ZincWhite", "LeadWhite", "BariumSulphate", "CalciumCarbonate"}
# Catalog tubes with a measured counterpart: tube name -> (Revigo paint, the fit sample's partner white sample).
COUNTERPARTS = {
    "lead white": "LeadWhite", "zinc white": "ZincWhite", "bone black": "BoneBlack",
    "cobalt blue": "CobaltBlue", "ultramarine blue": "UltramarineBlue", "Prussian blue": "PrussianBlue",
    "vermilion": "Vermillion", "orange vermilion": "Vermillion", "Chinese vermilion": "Vermillion",
    "chrome yellow": "ChromeYellow", "lemon chrome": "ChromeYellowOtero",
    "viridian": "ViridianGreen", "emerald green": "EmeraldGreen", "red lead": "Minium", "carmine lake": "Cochineal",
}


def split_for_strengths(samples, spectra, primaries, fit_data):
    pure = {}
    for sid, (names, fractions) in samples.items():
        present = [(n, c) for n, c in zip(names, fractions) if c > 0]
        if len(present) == 1 and abs(present[0][1] - 1) < 1e-9 and sid + "_ow" in spectra and sid not in gen.EXCLUDED_SAMPLES:
            pure[present[0][0]] = sid
    pairs = {}
    for sid in sorted(samples):
        names, fractions = samples[sid]
        present = [(n, c) for n, c in zip(names, fractions) if c > 0]
        if len(present) != 2 or sid + "_ow" not in spectra or sid in gen.EXCLUDED_SAMPLES:
            continue
        white = [n for n, _ in present if n in WHITES]
        if not white or not all(n in pure or n == "BoneBlack" for n, _ in present):
            continue
        paint = [n for n, _ in present if n != white[0]][0]
        share = dict(present)[paint]
        pairs[sid] = (paint, white[0], share)
    fit = {}
    for sid, (paint, white, share) in pairs.items():
        if white != "ZincWhite" or paint in primaries or paint == "BoneBlack":
            continue
        if paint not in fit or abs(share - 0.10) < abs(pairs[fit[paint]][2] - 0.10):
            fit[paint] = sid
    fit["LeadWhite"] = "08C"
    fit_ids = set(fit.values())
    selected = {sid for sid, (paint, white, share) in pairs.items()
                if sid not in fit_ids and paint != "BoneBlack" and series_of(paint, samples) in SELECT_SERIES and white == "ZincWhite"} | {"16E"}
    test = [sid for sid in pairs if sid not in fit_ids and sid not in fit_data and sid not in selected and "BoneBlack" not in pairs[sid][:2]]
    return pure, pairs, fit, sorted(test), sorted(s for s in selected if s not in fit_data)


def predict(binary, rows):
    """rows of (rgb_a, s_a, rgb_b, s_b, t) -> linear RGB predictions."""
    inputs = "".join((f"{a} {sa:.6f} {b} {sb:.6f} {t:.6f}" if isinstance(a, str) else
                      " ".join(f"{x:.6f}" for x in [*a, sa, *b, sb, t])) + "\n" for a, sa, b, sb, t in rows)
    out = subprocess.run([binary], input=inputs, capture_output=True, text=True, check=True).stdout.splitlines()
    return [np.array([float(x) for x in line.split()]) for line in out if line.strip()]


def best_strength(binary, paint_rgb, white_rgb, share, target, white_strength=1.0):
    """The strength (paint over its partner at the measured share) nearest the measured colour in OKLab."""
    grid = np.logspace(-2, 2.5, 451)
    for _ in range(2):
        rows = [(white_rgb, white_strength, paint_rgb, s, share) for s in grid]
        errors = [np.linalg.norm(oklab(p) - oklab(target)) for p in predict(binary, rows)]
        i = int(np.argmin(errors))
        lo, hi = grid[max(i - 1, 0)], grid[min(i + 1, len(grid) - 1)]
        best, error = float(grid[i]), float(errors[i])
        grid = np.linspace(lo, hi, 201)
    return best, error


def catalog_colours():
    source = (ROOT / "crates/paint/src/palette.rs").read_text()
    body = source.split("pub fn catalog()")[1].split("\n}\n")[0]
    out = {}
    for name, hex_, strength in re.findall(r'tube\("([^"]+)", "[^"]*", "(#[0-9a-f]{6})", [0-9.]+, [0-9.]+, ([0-9.]+),', body):
        v = np.array([int(hex_[i:i + 2], 16) / 255 for i in (1, 3, 5)])
        out[name] = (np.where(v <= .04045, v / 12.92, ((v + .055) / 1.055) ** 2.4), float(strength))
    return out


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mixpairs")
    parser.add_argument("--out", default=str(ROOT / "scripts/pigment_strengths.json"))
    parser.add_argument("--measured", action="store_true")
    args = parser.parse_args()
    matrix = gen.spectral_array("XYZ_RGB") @ gen.spectral_array("CMF")
    _, spectra, samples = gen.load_data()
    provenance = json.loads((ROOT / "crates/paint/src/mixer/primaries.json").read_text())
    primaries = provenance["primaries"]
    fit_data = set(sum(provenance["samples_used"].values(), []))
    pure, pairs, fit, test, selected = split_for_strengths(samples, spectra, primaries, fit_data)
    colour = lambda n: measured_colour(spectra, pure[n], matrix) if n != "BoneBlack" else np.zeros(3)
    print("fit set:", " ".join(f"{p}={sid}" for p, sid in sorted(fit.items())))
    print("test set:", " ".join(test))
    print("selection-touched, reported apart:", " ".join(selected))
    if args.measured:
        measured = json.loads((ROOT / "crates/paint/src/mixer/measured.json").read_text())
        assert measured["test_set"] == test
        assert not set(test) & set(sum(measured["samples_used"].values(), []))
        Path(args.out).write_text(json.dumps({
            "encoding": "measured-ks",
            "calibration": "K and S from primary ladders, pure films and the fixed round-3 fit set only; no additional scalar fit",
            "fit_set": {p: fit[p] for p in sorted(fit)},
            "test_set": test,
            "selection_touched": selected,
            "paint_strengths": {p: 1.0 for p in measured["order"]},
            "tube_strengths": {tube: {"paint": paint, "strength": 1.0} for tube, paint in COUNTERPARTS.items()},
        }, indent=2) + "\n")
        print("wrote", args.out)
        return
    strengths = {p: 1.0 for p in primaries}
    errors = {}
    for paint, sid in sorted(fit.items()):
        partner_paint, white, share = pairs[sid]
        if paint == "LeadWhite":
            s, e = best_strength(args.mixpairs, colour("LeadWhite"), colour("BoneBlack"), 1 - share, measured_colour(spectra, sid, matrix))
        else:
            s, e = best_strength(args.mixpairs, colour(paint), colour(white), share, measured_colour(spectra, sid, matrix))
        strengths[paint] = s
        errors[paint] = e
        print(f"  {paint:18s} {sid} share {share:.3f}: strength {s:.3f} (fit residual {e:.3f})", flush=True)
    tubes = {}
    catalog = catalog_colours()
    for tube, paint in COUNTERPARTS.items():
        rgb, current = catalog[tube]
        if paint == "ZincWhite":
            tubes[tube] = {"strength": 1.0, "was": current, "sample": None, "fit_residual": 0.0}
            continue
        if paint == "LeadWhite":
            sid = "08C"; share = 1 - pairs[sid][2]
            s, e = best_strength(args.mixpairs, rgb, colour("BoneBlack"), share, measured_colour(spectra, sid, matrix))
        else:
            sid = fit.get(paint) or min((x for x, (p, w, c) in pairs.items() if p == paint and w == "ZincWhite"), key=lambda x: abs(pairs[x][2] - 0.10))
            share = pairs[sid][2]
            s, e = best_strength(args.mixpairs, rgb, colour("ZincWhite"), share, measured_colour(spectra, sid, matrix))
        tubes[tube] = {"strength": round(s, 2), "was": current, "sample": sid, "fit_residual": round(e, 4)}
        print(f"  tube {tube:18s} <- {paint} ({sid}): strength {s:.2f} (was {current}, residual {e:.3f})", flush=True)
    Path(args.out).write_text(json.dumps({
        "mixpairs": os.path.basename(args.mixpairs),
        "fit_set": {p: fit[p] for p in sorted(fit)},
        "test_set": test,
        "selection_touched": selected,
        "paint_strengths": {p: round(s, 4) for p, s in sorted(strengths.items())},
        "fit_residuals": {p: round(e, 4) for p, e in sorted(errors.items())},
        "tube_strengths": tubes,
    }, indent=2) + "\n")
    print("wrote", args.out)


if __name__ == "__main__":
    main()
