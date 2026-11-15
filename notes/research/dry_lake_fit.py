#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""The dye loadings behind paint's `dry::dye_loading` (dry_pigment_optics.md §6).

Inputs:
  powder.csv  Cosentino 2014, "FORS spectral database of historical pigments in
              different binders", CSV/powder.csv from the CHSOS download
              (https://chsopensource.org/new-free-pigments-fors-spectroscopy-database/,
              archive FORS-Spectral-database-of-historical-pigments-in-different-binders-CSV.zip).
              Not redistributed here; its SHA-256 is checked below.
  tubes.json  each tube's engine-9 dry-path K and S per coat:
              DRY_LAKE_FIT_OUT=tubes.json cargo test -p paint --lib -- --ignored --exact \
                  palette::tests::dump_dry_lake_fit_input

  uv run notes/research/dry_lake_fit.py <repo root> <powder.csv> <tubes.json>

The powders were measured with a fiber probe, and the raw reflectance has a
flat floor (carbon blacks read 0.14-0.17) and a gain (white powders read
1.10-1.30). Each spectrum is mapped linearly between the four blacks' mean
(taken as 0.04, a dry black powder's surface reflection: §3) and a white
powder's (taken as 0.93): either chalk and gypsum, or lead white. Both whites
are kept as the measurement's spread. The fit then finds, per lake, the factor
on its oil K that makes its dry color (two-constant KM, its scattering as the
engine's rule for n < 1.9 at the substrate's 1.57) match the corrected
powder's CIELAB, at the midpoint of the two corrections.
"""
import ast
import csv
import hashlib
import json
import re
import sys
from pathlib import Path

import numpy as np
from scipy.optimize import minimize_scalar

POWDER_SHA256 = "86bb05f002d64ad5aa0a1f66c7226f18b62824e77d3aa3e4e2a2aee3f0efae45"
BLACK_TRUE, WHITE_TRUE = 0.04, 0.93  # estimates (see the docstring)
SUBSTRATE_N = 1.57  # alumina hydrate (gibbsite)
LAKES = {"rose madder": "madderlake", "carmine lake": "carminelake", "yellow lake": "yellowlakeresed"}
CONTEXT = {"alizarin (Kremer)": "alizarin", "lac dye lake": "lacdye", "vermilion": "vermilion",
           "ultramarine": "ultramarine"}

root, powder, tubes_path = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
if hashlib.sha256(powder.read_bytes()).hexdigest() != POWDER_SHA256:
    sys.exit(f"{powder}: not the CHSOS 2014 powder.csv this fit was made from")
tubes = json.loads(tubes_path.read_text())
src = (root / "crates/paint/src/spectral.rs").read_text()


def table(name):
    v = src.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", v).strip()))


CMF = table("CMF")
TO_RGB = table("XYZ_RGB") @ CMF
NM = 380 + 10 * np.arange(38)
VIS = (NM >= 400) & (NM <= 700)
WHITE_XYZ = CMF.sum(axis=1)


def km(q):
    q = np.maximum(q, 0)
    return 1 / (1 + q + np.sqrt(q * (q + 2)))


def srgb8(r):
    lin = np.clip(TO_RGB @ r, 0, 1)
    return np.round(np.where(lin <= 0.0031308, 12.92 * lin, 1.055 * lin ** (1 / 2.4) - 0.055) * 255).astype(int)


def lab(r):
    x = CMF @ r / WHITE_XYZ
    f = np.where(x > 0.008856, np.cbrt(x), 7.787 * x + 16 / 116)
    return np.array([116 * f[1] - 16, 500 * (f[0] - f[1]), 200 * (f[1] - f[2])])


rows = list(csv.reader(powder.open()))
head = rows[0]
data = np.array([[float(x) for x in r] for r in rows[1:] if len(r) == len(head)])
nm_src = data[:, 0]


def measured(name):
    # the mean over each 10 nm band, as a fraction
    return np.array([data[(nm_src >= c - 5) & (nm_src < c + 5), head.index(name)].mean() for c in NM]) / 100


BLACK = np.mean([measured(n) for n in ("ivoryblack", "vineblack", "boneblack", "lampblack")], axis=0)
WHITES = {"chalk and gypsum": np.mean([measured("chalk"), measured("gypsum")], axis=0),
          "lead white": measured("leadwhite")}


def corrected(name, white, black_true=BLACK_TRUE, white_true=WHITE_TRUE):
    return black_true + (measured(name) - BLACK) * (white_true - black_true) / (WHITES[white] - BLACK)


# the engine's Mie table (dry.rs `mie_air`)
MIE = [(1.50, 0.68, 700.0), (1.56, 0.71, 43.0), (1.65, 0.73, 9.0), (1.70, 0.80, 5.8), (1.74, 0.75, 3.9),
       (1.84, 0.82, 2.5), (2.00, 0.88, 1.9), (2.15, 0.91, 1.7), (2.30, 0.89, 1.6), (2.40, 1.04, 1.8),
       (2.45, 0.95, 1.5), (3.00, 1.03, 1.4)]


def mie(n):
    if n <= MIE[0][0]:
        return MIE[0][1:]
    for a, b in zip(MIE, MIE[1:]):
        if n <= b[0]:
            u = (n - a[0]) / (b[0] - a[0])
            return a[1] + (b[1] - a[1]) * u, np.exp(np.log(a[2]) + (np.log(b[2]) - np.log(a[2])) * u)
    return MIE[-1][1:]


WHITE_S = np.array(tubes["lead white"]["s"])
SIGMA_WHITE, RATIO_WHITE = mie(2.0)


def dry(parts, loading=None):
    """The engine's dry color (dry.rs `spectral_color`) of (tube, share) parts; `loading` maps a lake to its factor."""
    loading = loading or {}
    K = S = 0
    for name, c in parts:
        t = tubes[name]
        k, s, n = np.array(t["k"]), np.array(t["s"]), t["n"]
        if name in loading:
            k, n = k * loading[name], SUBSTRATE_N
        sigma, ratio = mie(n)
        K = K + c * k
        S = S + c * (ratio * s if n >= 1.9 else RATIO_WHITE * WHITE_S * sigma / SIGMA_WHITE)
    return km(K / S)


def fit(tube, target_lab):
    f = lambda la: np.linalg.norm(lab(dry([(tube, 1)], {tube: np.exp(la)})) - target_lab)
    r = minimize_scalar(f, bounds=(-8, 8), method="bounded")
    return float(np.exp(r.x)), float(r.fun)


print(f"raw floor (blacks) {BLACK[VIS].mean():.3f}; raw whites: chalk and gypsum {WHITES['chalk and gypsum'][VIS].mean():.3f}, "
      f"lead white {WHITES['lead white'][VIS].mean():.3f}")
print("\npowder, corrected with chalk and gypsum | with lead white:  sRGB, L*a*b*")
for label, name in {**LAKES, **CONTEXT}.items():
    a, b = corrected(name, "chalk and gypsum"), corrected(name, "lead white")
    print(f"  {label:18s} {srgb8(a)} {np.round(lab(a), 1)} | {srgb8(b)} {np.round(lab(b), 1)}")

print("\nfit: tube, loading, dE76 at the fit, dry sRGB and L*a*b* now -> with the loading; 1:1 lead white now -> with it")
for tube, name in LAKES.items():
    target = (lab(corrected(name, "chalk and gypsum")) + lab(corrected(name, "lead white"))) / 2
    a, de = fit(tube, target)
    was, now = dry([(tube, 1)]), dry([(tube, 1)], {tube: a})
    tint_was, tint_now = dry([(tube, .5), ("lead white", .5)]), dry([(tube, .5), ("lead white", .5)], {tube: a})
    print(f"  {tube:13s} {a:6.3g}  dE {de:4.1f}  target {np.round(target, 1)}  {srgb8(was)} {np.round(lab(was), 1)} -> "
          f"{srgb8(now)} {np.round(lab(now), 1)}  (dE now {np.linalg.norm(lab(was) - target):.1f})  "
          f"1:1 {srgb8(tint_was)} -> {srgb8(tint_now)}")

print("\nloading under other corrections (black taken as, white taken as): chalk-and-gypsum and lead-white references")
for bt in (0.03, 0.04, 0.05):
    for wt in (0.90, 0.93, 0.96):
        cells = []
        for tube, name in LAKES.items():
            for white in WHITES:
                cells.append(fit(tube, lab(corrected(name, white, bt, wt)))[0])
        print(f"  {bt:.2f} {wt:.2f}  " + "  ".join(f"{t[:6]} {cells[2 * i]:.3g}/{cells[2 * i + 1]:.3g}" for i, t in enumerate(LAKES)))
