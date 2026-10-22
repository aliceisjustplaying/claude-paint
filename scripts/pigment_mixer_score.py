# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Score the built mixer against measured paint mixtures it never saw.

Every two-paint mixture in the Revigo data whose pure paints were measured is predicted from the two paints'
measured colours (over the white field, specular excluded) and the measured weight share, through the engine's
own tables (the mixpairs example), and compared with the measured mixture in OKLab. The samples the generator
fitted the primaries on (recorded in primaries.json) are left out. The test half is the score; the selection half, which
scripts/pigment_basis_select.py read when choosing the primaries, is reported separately and labelled.

    cargo build --release -p paint --example mixpairs
    uv run --script scripts/pigment_mixer_score.py target/release/examples/mixpairs [--reference <other mixpairs>] [--out table.md]

The reference is another build of the same example, for instance from main, scored on the same inputs.
"""

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

os.environ["OPENBLAS_NUM_THREADS"] = "1"

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_pigment_tables as gen
from pigment_basis_select import measured_colour, oklab, split

ROOT = Path(__file__).resolve().parents[1]


def hexof(lin):
    c = np.clip(lin, 0, 1)
    s = np.where(c <= 0.0031308, 12.92 * c, 1.055 * c ** (1 / 2.4) - 0.055)
    return "#%02x%02x%02x" % tuple(int(round(255 * x)) for x in s)


def run(binary, inputs):
    out = subprocess.run([binary], input=inputs, capture_output=True, text=True, check=True).stdout.splitlines()
    return [np.array([float(x) for x in line.split()]) for line in out if line.strip()]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mixpairs")
    parser.add_argument("--reference")
    parser.add_argument("--out")
    args = parser.parse_args()
    matrix = gen.spectral_array("XYZ_RGB") @ gen.spectral_array("CMF")
    _, spectra, samples = gen.load_data()
    pure, sides = split(samples, spectra)
    fit_data = set(sum(json.loads((ROOT / "crates/paint/src/mixer/primaries.json").read_text())["samples_used"].values(), []))
    lines = []
    summary = {}
    for side in ("test", "select"):
        held = []
        for sid in sides[side]:
            names, fractions = samples[sid]
            present = [(n, c) for n, c in zip(names, fractions) if c > 0]
            if sid in fit_data:
                continue
            (a, ca), (b, cb) = present
            held.append((sid, a, b, cb / (ca + cb)))
        inputs = "".join(" ".join(f"{x:.6f}" for x in [*measured_colour(spectra, pure[a], matrix), *measured_colour(spectra, pure[b], matrix), t]) + "\n"
                         for _, a, b, t in held)
        new = run(args.mixpairs, inputs)
        reference = run(args.reference, inputs) if args.reference else [None] * len(held)
        errors, reference_errors = [], []
        title = "Test half (untouched by every choice)" if side == "test" else "Selection half (read while choosing the primaries)"
        lines += [f"## {title}", "", "| sample | mixture by weight | paint A | paint B | measured | new | ΔE new" + (" | reference | ΔE reference |" if args.reference else " |"),
                  "|---|---|---|---|---|---|---|" + ("---|---|" if args.reference else "")]
        for (sid, a, b, t), pn, pr in zip(held, new, reference):
            target = measured_colour(spectra, sid, matrix)
            dn = float(np.linalg.norm(oklab(pn) - oklab(target)))
            errors.append(dn)
            row = f"| {sid} | {a} {100 * (1 - t):.1f} % + {b} {100 * t:.1f} % | {hexof(measured_colour(spectra, pure[a], matrix))} | {hexof(measured_colour(spectra, pure[b], matrix))} | {hexof(target)} | {hexof(pn)} | {dn:.3f} |"
            if pr is not None:
                dr = float(np.linalg.norm(oklab(pr) - oklab(target)))
                reference_errors.append(dr)
                row += f" {hexof(pr)} | {dr:.3f} |"
            lines.append(row)
        worst = held[int(np.argmax(errors))][0]
        text = f"{len(held)} mixtures. New mixer: median {np.median(errors):.3f}, mean {np.mean(errors):.3f}, worst {max(errors):.3f} ({worst})."
        if reference_errors:
            text += (f" Reference: median {np.median(reference_errors):.3f}, mean {np.mean(reference_errors):.3f}, worst {max(reference_errors):.3f}"
                     f" ({held[int(np.argmax(reference_errors))][0]}). New closer on {sum(n < r for n, r in zip(errors, reference_errors))} of {len(held)}.")
        lines += ["", text, ""]
        summary[side] = text
    print("\n".join(lines))
    if args.out:
        Path(args.out).write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
