# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Choose the mixer's primaries from the Revigo measurements, judged on the selection half of the mixtures only.

The split is fixed before any score is read, by paint: every mixture a paint takes part in goes to one side,
decided by the parity of the paint's series number in the data (01 eosin on wax, 05 red lead, 09 viridian and
11 ultramarine select; 02 eosin, 04 cochineal, 06 vermilion, 10 emerald green, 12 cobalt blue and 14 chrome
yellow test). A candidate basis is never scored on its fit data: the ladders of its own primaries, that is every
sample whose paints are all primaries or the zinc white and bone black the ladders were mixed with.
scripts/pigment_mixer_score.py scores the built tables on the test half.

Scores per candidate (five primaries: a blue, a yellow, a red, bone black and a white):
  selection: mean OKLab distance between each selection mixture's measured colour and the exact-model mix of
             the two paints' measured colours at the measured weights (lower is better);
  reach:     mean residual the exact model needs to encode the palette catalog's tube colours.
The rule: lowest selection score; candidates within 0.005 of it tie, and the lowest reach wins the tie.

    uv run --script scripts/pigment_basis_select.py
"""

import itertools
import os
import re
import sys
from pathlib import Path

os.environ["OPENBLAS_NUM_THREADS"] = "1"

import numpy as np
from scipy.optimize import minimize

sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_pigment_tables as gen

ROOT = Path(__file__).resolve().parents[1]
CANDIDATES = {
    "blue": ["PrussianBlue", "CobaltBlue", "UltramarineBlue"],
    "yellow": ["ChromeYellowOtero", "ChromeYellow"],
    "red": ["Eosin", "Eosin2PbWax", "Cochineal"],
    "black": ["BoneBlack"],
    "white": ["ZincWhite", "LeadWhite"],
}
SELECT_SERIES = {"01", "05", "09", "11"}
TEST_SERIES = {"02", "04", "06", "10", "12", "14"}
TIE = 0.005


def series_of(name, samples):
    for sid, (names, fractions) in samples.items():
        present = [n for n, c in zip(names, fractions) if c > 0]
        if present == [name]:
            return sid[:2]
    raise KeyError(name)


def split(samples, spectra):
    """Two-paint mixtures with both pure paints measured, keyed by the side their paint's series puts them on."""
    pure = {}
    for sid, (names, fractions) in samples.items():
        present = [(n, c) for n, c in zip(names, fractions) if c > 0]
        if len(present) == 1 and abs(present[0][1] - 1) < 1e-9 and sid + "_ow" in spectra and sid not in gen.EXCLUDED_SAMPLES:
            pure[present[0][0]] = sid
    sides = {"select": [], "test": []}
    for sid in sorted(samples):
        names, fractions = samples[sid]
        present = [(n, c) for n, c in zip(names, fractions) if c > 0]
        if len(present) != 2 or sid + "_ow" not in spectra or sid in gen.EXCLUDED_SAMPLES or not all(n in pure for n, _ in present):
            continue
        paints = {series_of(n, samples) for n, _ in present if n not in gen.HELPERS and n not in ("LeadWhite", "BariumSulphate", "CalciumCarbonate")}
        if paints <= SELECT_SERIES:
            sides["select"].append(sid)
        elif paints <= TEST_SERIES:
            sides["test"].append(sid)
    return pure, sides


def measured_colour(spectra, sid, matrix):
    """Linear RGB of a measured film over the white field, shown specular-excluded."""
    reflectance = gen.saunderson(gen.inside(spectra[sid + "_ow"]), False)
    return np.clip(np.concatenate([np.full(2, reflectance[0]), reflectance, np.full(5, reflectance[-1])]) @ matrix.T, 0, 1)


def oklab(lin):
    lin = np.clip(lin, 0, 1)
    l = 0.4122214708 * lin[0] + 0.5363325363 * lin[1] + 0.0514459929 * lin[2]
    m = 0.2119034982 * lin[0] + 0.6806995451 * lin[1] + 0.1073969566 * lin[2]
    s = 0.0883024619 * lin[0] + 0.2817188376 * lin[1] + 0.6299787005 * lin[2]
    l, m, s = np.cbrt(l), np.cbrt(m), np.cbrt(s)
    return np.array([0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
                     1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
                     0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s])


def catalog():
    source = (ROOT / "crates/paint/src/palette.rs").read_text()
    body = source.split("pub fn catalog()")[1].split("\n}\n")[0]
    colours = []
    for hex_ in re.findall(r'tube\("[^"]+", "[^"]*", "(#[0-9a-f]{6})"', body):
        v = np.array([int(hex_[i:i + 2], 16) / 255 for i in (1, 3, 5)])
        colours.append(np.where(v <= .04045, v / 12.92, ((v + .055) / 1.055) ** 2.4))
    return colours


class Basis:
    def __init__(self, fitted, order, matrix, extend):
        self.order = order
        self.absorption = np.array([extend(fitted[name][0]) for name in order])
        self.scatter = np.array([extend(fitted[name][1]) for name in order])
        self.matrix = matrix

    def mix(self, c):
        c = np.maximum(c, 0)
        return gen.saunderson(gen.km((c @ self.absorption) / np.maximum(c @ self.scatter, 1e-12)), False) @ self.matrix.T

    def encode(self, rgb):
        n = len(self.order)

        def error(c):
            raw = self.mix(c)
            diff = np.clip(raw, 0, 1) - rgb
            rdiff = raw - rgb
            return diff @ diff + gen.RAW_FIT_WEIGHT * (rdiff @ rdiff)

        sol = minimize(error, np.full(n, 1 / n), method="SLSQP", bounds=[(0, 1)] * n,
                       constraints=[{"type": "eq", "fun": lambda c: c.sum() - 1}], options={"ftol": 1e-12, "maxiter": 300})
        c = np.maximum(0, sol.x)
        c /= c.sum()
        return np.concatenate([c, rgb - np.clip(self.mix(c), 0, 1)])

    def decode(self, latent):
        n = len(self.order)
        return np.clip(np.clip(self.mix(latent[:n]), 0, 1) + latent[n:], 0, 1)


def main():
    matrix = gen.spectral_array("XYZ_RGB") @ gen.spectral_array("CMF")
    extend = lambda v: np.concatenate([np.full(2, v[0]), v, np.full(5, v[-1])])
    nm, spectra, samples = gen.load_data()
    pure, sides = split(samples, spectra)
    print("selection mixtures:", " ".join(sides["select"]))
    print("test mixtures (never read here):", " ".join(sides["test"]))
    fitted = {}
    helpers = sorted(sid for sid, (names, _) in samples.items()
                     if sid not in gen.EXCLUDED_SAMPLES and set(names) == set(gen.HELPERS)
                     and all(sid + s in spectra for s in ("_b", "_w", "_ob", "_ow")))
    found, _ = gen.fit(nm, spectra, samples, gen.HELPERS, {}, helpers)
    fitted.update(found)
    for name in sorted(set(sum(CANDIDATES.values(), []))):
        if name in fitted:
            continue
        ids = sorted(sid for sid, (names, fractions) in samples.items()
                     if sid not in gen.EXCLUDED_SAMPLES and all(sid + s in spectra for s in ("_b", "_w", "_ob", "_ow"))
                     and name in names and all(n in fitted or n == name for n, c in zip(names, fractions) if c > 0))
        found, _ = gen.fit(nm, spectra, samples, [name], fitted, ids)
        fitted.update(found)
        print("fitted", name, "from", " ".join(ids), flush=True)
    colours = catalog()
    rows = []
    for order in itertools.product(*CANDIDATES.values()):
        basis = Basis(fitted, order, matrix, extend)
        reach = float(np.mean([np.linalg.norm(basis.encode(c)[len(order):]) for c in colours]))
        encoded = {}
        errors = []
        for sid in sides["select"]:
            names, fractions = samples[sid]
            present = [(n, c) for n, c in zip(names, fractions) if c > 0]
            if all(n in order or n in gen.HELPERS for n, _ in present):
                continue
            latent = 0
            for n, c in present:
                if n not in encoded:
                    encoded[n] = basis.encode(measured_colour(spectra, pure[n], matrix))
                latent = latent + c * encoded[n]
            errors.append(np.linalg.norm(oklab(basis.decode(latent)) - oklab(measured_colour(spectra, sid, matrix))))
        rows.append((float(np.mean(errors)), reach, len(errors), order))
        print(f"selection {rows[-1][0]:.4f} over {len(errors):2d} mixtures, reach {reach:.4f}: {' '.join(order)}", flush=True)
    best = min(r[0] for r in rows)
    tied = sorted((r for r in rows if r[0] <= best + TIE), key=lambda r: r[1])
    print()
    print("| selection ΔE | mixtures | catalog reach | blue | yellow | red | black | white |")
    print("|---|---|---|---|---|---|---|---|")
    for score, reach, count, order in sorted(rows):
        print(f"| {score:.4f} | {count} | {reach:.4f} | " + " | ".join(order) + " |")
    print()
    print("chosen:", " ".join(tied[0][3]), f"(selection {tied[0][0]:.4f}, reach {tied[0][1]:.4f}; {len(tied)} within {TIE} of the best)")


if __name__ == "__main__":
    main()
