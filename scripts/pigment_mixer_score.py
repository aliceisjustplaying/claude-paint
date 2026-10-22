# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1", "pillow==12.3.0"]
# ///
"""Score the built mixer against measured paint mixtures it never saw, with calibrated tinting strengths.

Every two-paint mixture in the Revigo data whose pure paints are measured is predicted from the two paints'
measured colours (over the white field, specular excluded), the measured weight share and each paint's tinting
strength from scripts/pigment_strengths.json (fitted on that file's fit set alone, see pigment_strength_fit.py),
through the engine's own tables (the mixpairs example), and compared with the measured mixture in OKLab. The test
set is the score; the fit set and the mixtures scripts/pigment_basis_select.py read while choosing the primaries
are reported apart and labelled.

    cargo build --release -p paint --example mixpairs
    uv run --script scripts/pigment_mixer_score.py target/release/examples/mixpairs --strengths scripts/pigment_strengths.json \
        [--reference <other mixpairs> --reference-strengths <its strengths.json>] [--out table.md] [--sheet sheet.png]

The reference is another build of the same example (main's mixer) with strengths fitted the same way for it.
"""

import argparse
import json
import os
import sys
from pathlib import Path

os.environ["OPENBLAS_NUM_THREADS"] = "1"

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_pigment_tables as gen
from pigment_basis_select import measured_colour, oklab
from pigment_strength_fit import predict, split_for_strengths

ROOT = Path(__file__).resolve().parents[1]


def hexof(lin):
    c = np.clip(lin, 0, 1)
    s = np.where(c <= 0.0031308, 12.92 * c, 1.055 * c ** (1 / 2.4) - 0.055)
    return "#%02x%02x%02x" % tuple(int(round(255 * x)) for x in s)


def sheet(path, groups):
    from PIL import Image, ImageDraw, ImageFont
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 15)
    title = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 22)
    rows = sum(len(g) + 1 for _, g in groups)
    im = Image.new("RGB", (1000, 70 + 46 * rows), "#f4f1e9")
    d = ImageDraw.Draw(im)
    d.text((18, 14), "Measured paint mixtures: measured | new | main", font=title, fill="#20221f")
    y = 60
    for name, rows_ in groups:
        d.text((18, y + 10), name, font=font, fill="#20221f")
        y += 46
        for label, measured, new, ref, dn, dr in rows_:
            for k, c in enumerate([measured, new, ref]):
                if c is not None:
                    d.rectangle((18 + k * 120, y, 18 + k * 120 + 112, y + 40), fill=hexof(c))
            d.text((380, y + 11), f"{label}   new {dn:.3f}" + (f"   main {dr:.3f}" if dr is not None else ""), font=font, fill="#20221f")
            y += 46
    im.save(path)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mixpairs")
    parser.add_argument("--strengths", required=True)
    parser.add_argument("--reference")
    parser.add_argument("--reference-strengths")
    parser.add_argument("--out")
    parser.add_argument("--sheet")
    args = parser.parse_args()
    matrix = gen.spectral_array("XYZ_RGB") @ gen.spectral_array("CMF")
    _, spectra, samples = gen.load_data()
    provenance = json.loads((ROOT / "crates/paint/src/mixer/primaries.json").read_text())
    pure, pairs, fit, test, selected = split_for_strengths(samples, spectra, provenance["primaries"], set(sum(provenance["samples_used"].values(), [])))
    calibration = json.loads(Path(args.strengths).read_text())
    reference_calibration = json.loads(Path(args.reference_strengths).read_text()) if args.reference_strengths else calibration
    assert calibration["test_set"] == test, "the strengths file was fitted on another split"
    colour = lambda n: measured_colour(spectra, pure[n], matrix) if n != "BoneBlack" else np.zeros(3)
    lines, groups = [], []
    for name, ids in (("Test set (untouched by every fit and choice)", test), ("Fit set (the strengths were fitted on these)", sorted(fit.values())), ("Selection-touched (read while choosing the primaries)", selected)):
        rows = []
        for sid in ids:
            paint, white, share = pairs[sid]
            rows.append((sid, paint, white, share))
        def inputs(strengths):
            out = []
            for sid, paint, white, share in rows:
                s_paint = strengths["paint_strengths"].get(paint, 1.0)
                s_white = strengths["paint_strengths"].get(white, 1.0)
                if paint == "BoneBlack":
                    out.append((colour(white), s_white, colour("BoneBlack"), 1.0, share))
                else:
                    out.append((colour(white), s_white, colour(paint), s_paint, share))
            return out
        new = predict(args.mixpairs, inputs(calibration))
        reference = predict(args.reference, inputs(reference_calibration)) if args.reference else [None] * len(rows)
        errors, reference_errors, swatches = [], [], []
        lines += [f"## {name}", "", "| sample | mixture by weight | strength | measured | new | ΔE new" + (" | main | ΔE main |" if args.reference else " |"), "|---|---|---|---|---|---|" + ("---|---|" if args.reference else "")]
        for (sid, paint, white, share), pn, pr in zip(rows, new, reference):
            target = measured_colour(spectra, sid, matrix)
            dn = float(np.linalg.norm(oklab(pn) - oklab(target)))
            errors.append(dn)
            s_paint = calibration["paint_strengths"].get(paint, 1.0)
            line = f"| {sid} | {white} {100 * (1 - share):.1f} % + {paint} {100 * share:.1f} % | {s_paint:.2f} | {hexof(target)} | {hexof(pn)} | {dn:.3f} |"
            dr = None
            if pr is not None:
                dr = float(np.linalg.norm(oklab(pr) - oklab(target)))
                reference_errors.append(dr)
                line += f" {hexof(pr)} | {dr:.3f} |"
            lines.append(line)
            swatches.append((f"{sid} {white} + {paint} {100 * share:.1f} %", target, pn, pr, dn, dr))
        text = f"{len(rows)} mixtures. New mixer: median {np.median(errors):.3f}, mean {np.mean(errors):.3f}, worst {max(errors):.3f} ({rows[int(np.argmax(errors))][0]})."
        if reference_errors:
            text += (f" Main: median {np.median(reference_errors):.3f}, mean {np.mean(reference_errors):.3f}, worst {max(reference_errors):.3f}"
                     f" ({rows[int(np.argmax(reference_errors))][0]}). New closer on {sum(n < r for n, r in zip(errors, reference_errors))} of {len(rows)}.")
        lines += ["", text, ""]
        groups.append((name, swatches))
    print("\n".join(lines))
    if args.out:
        Path(args.out).write_text("\n".join(lines) + "\n")
    if args.sheet:
        sheet(args.sheet, groups)


if __name__ == "__main__":
    main()
