# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Generate Stillwet's pigment tables from measured reflectance of real oil paints, not Mixbox data.

The primaries are four paints of the Revigo project's reconstruction of Van Gogh's palette (Geldof,
Proano Gaibor, Ligterink, Hendriks and Kirchner 2018, doi:10.1186/s40494-018-0181-6), whose reflectance
measurements are published under CC0 in https://github.com/fligt/revigo-spectra and vendored in
scripts/pigment_data/revigo. Every paint was drawn down 100 um thick over the black and the white field
of an opacity chart, pure and mixed with zinc white and bone black by weight, and measured 400-700 nm
with specular included. Per wavelength this script solves the non-hiding Kubelka-Munk film equation
for each paint's absorption K and scattering S (the paper's own method), mixes the primaries by plain
linear two-constant KM (K and S add by weight concentration), and converts the infinite-thickness
reflectance to linear sRGB through the MIT spectral.js D65/CIE matrix in spectral.rs.
"""

import ast
import hashlib
import json
import os
import re
from pathlib import Path

os.environ["OPENBLAS_NUM_THREADS"] = "1"

import numpy as np
from scipy.optimize import least_squares, minimize

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "scripts/pigment_data/revigo"
OUT = ROOT / "crates/paint/src/mixer"
INVERSE_SIZE = 33
# 21 nodes per forward axis keep the previous 33-node three-axis table's interpolation error on catalog ramps
# (mean 0.006 vs 0.009 OKLab, 99th percentile 0.048 vs 0.055) at 2.3 MiB; 25 would halve the error for twice the size.
FORWARD_SIZE = 21
RAW_FIT_WEIGHT = .05
# An inverse node within this linear-RGB distance below the white primary stores the white vertex and leaves
# the difference to the runtime's RGB residual, so a colour inside such a cell interpolates to the vertex
# instead of to a blend of its neighbours' small chromatic shares.
NEAR_WHITE = 0.07
# Saunderson surface-reflection constants the source used for its specular-included measurements
# (k1 collimated, k2 diffuse). Mixed colours are shown specular-excluded, the matte paint colour.
K1, K2 = 0.04, 0.49
# Latent order: blue, yellow, magenta, black, white. Source sample series per paint are in the data README.
# Chosen by scripts/pigment_basis_select.py on the selection half of the measured mixtures.
PRIMARIES = ["CobaltBlue", "ChromeYellowOtero", "Eosin", "BoneBlack", "ZincWhite"]
# The white and black every ladder was mixed with, fitted first from their own series.
HELPERS = ["ZincWhite", "BoneBlack"]
EXCLUDED_SAMPLES = {"07B", "14F"}  # the same paints ground in a different oil


def spectral_array(name):
    source = (ROOT / "crates/paint/src/spectral.rs").read_text()
    value = source.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", value).strip()))


def load_data():
    spectra = {}
    lines = (DATA / "revigo-aggregated-palette-spectral-data.txt").read_text().splitlines()
    nm = np.array([float(x) for x in lines[0].split("\t")[1:]])
    for line in lines[1:]:
        fields = line.split("\t")
        spectra[fields[0]] = np.array([float(x) for x in fields[1:]]) / 100
    samples = {}
    for line in (DATA / "revigo-mixture-specifications.txt").read_text().splitlines():
        sid, rest = line.split("\t")
        names, fractions = rest.split("|")
        samples[sid] = (names.split("-"), np.array([float(x) for x in fractions.split("-")]) / 100)
    return nm, spectra, samples


def saunderson(rt, specular):
    return (K1 if specular else 0) + (1 - K1) * (1 - K2) * rt / (1 - K2 * rt)


def inside(rm):
    return (rm - K1) / ((1 - K1) * (1 - K2) + K2 * (rm - K1))


def film(K, S, rg):
    """Kubelka-Munk reflectance of a unit-thickness film over a substrate of reflectance rg."""
    a = 1 + K / S
    b = np.sqrt(np.maximum(a * a - 1, 1e-12))
    coth = 1 / np.tanh(np.minimum(b * S, 50))
    return (1 - rg * (a - b * coth)) / (a - rg + b * coth)


def km(q):
    return 1 / (1 + q + np.sqrt(q * (q + 2)))


def warp(x):
    return x**2 / (x**2 + (1 - x)**2)


def fit(nm, spectra, samples, unknown, known, ids):
    """Per wavelength, least-squares K and S of the unknown paints from their samples over both fields."""
    result = {name: (np.zeros(len(nm)), np.zeros(len(nm))) for name in unknown}
    rms = {}
    for w in range(len(nm)):
        def unpack(p):
            d = dict(known)
            for i, name in enumerate(unknown):
                d[name] = (np.exp(p[2 * i]), np.exp(p[2 * i + 1]))
            return d

        def residual(p):
            d = unpack(p)
            r = []
            for sid in ids:
                names, fractions = samples[sid]
                K = sum(c * np.asarray(d[n][0])[..., w] if np.ndim(d[n][0]) else c * d[n][0] for n, c in zip(names, fractions) if c > 0)
                S = sum(c * np.asarray(d[n][1])[..., w] if np.ndim(d[n][1]) else c * d[n][1] for n, c in zip(names, fractions) if c > 0)
                for field, measured in (("_b", "_ob"), ("_w", "_ow")):
                    r.append(saunderson(film(K, S, inside(spectra[sid + field][w])), True) - spectra[sid + measured][w])
            return np.array(r)

        best = None
        for start in ([0.0, 0.0], [2.0, -1.0], [-1.0, 1.0], [3.0, 1.0], [1.0, 3.0]):
            sol = least_squares(residual, np.array(start * len(unknown)), bounds=(-9, 9), method="trf",
                                xtol=1e-12, ftol=1e-12, gtol=1e-12, max_nfev=2000)
            if best is None or sol.cost < best.cost:
                best = sol
        d = unpack(best.x)
        for name in unknown:
            result[name][0][w], result[name][1][w] = d[name]
    for sid in ids:
        names, fractions = samples[sid]
        d = {**known, **result}
        K = sum(c * d[n][0] for n, c in zip(names, fractions) if c > 0)
        S = sum(c * d[n][1] for n, c in zip(names, fractions) if c > 0)
        rms[sid] = [float(np.sqrt(np.mean((saunderson(film(K, S, inside(spectra[sid + field])), True) - spectra[sid + measured]) ** 2)))
                    for field, measured in (("_b", "_ob"), ("_w", "_ow"))]
    return result, rms


def derive():
    nm, spectra, samples = load_data()
    complete = lambda sid: all(sid + suffix in spectra for suffix in ("_b", "_w", "_ob", "_ow"))
    series = {}
    for sid, (names, fractions) in samples.items():
        if sid in EXCLUDED_SAMPLES or not complete(sid):
            continue
        for name, fraction in zip(names, fractions):
            if fraction > 0 and name not in HELPERS:
                series.setdefault(name, []).append(sid)
    known, fit_rms = {}, {}
    whites = sorted(sid for sid, (names, _) in samples.items() if sid not in EXCLUDED_SAMPLES and complete(sid) and set(names) == set(HELPERS))
    found, rms = fit(nm, spectra, samples, HELPERS, {}, whites)
    known.update(found); fit_rms.update(rms)
    used = {name: whites for name in HELPERS}
    for name in PRIMARIES:
        if name in known:
            continue
        ids = sorted(sid for sid in series[name]
                     if all(n in known or n == name for n, c in zip(*samples[sid]) if c > 0))
        found, rms = fit(nm, spectra, samples, [name], known, ids)
        known.update(found); fit_rms.update(rms); used[name] = ids
    return nm, known, fit_rms, used


def main():
    matrix = spectral_array("XYZ_RGB") @ spectral_array("CMF")
    nm_data, fitted, fit_rms, used = derive()
    nm = np.arange(380, 751, 10)
    extend = lambda v: np.concatenate([np.full(2, v[0]), v, np.full(5, v[-1])])
    absorption = np.array([extend(fitted[name][0]) for name in PRIMARIES])
    scatter = np.array([extend(fitted[name][1]) for name in PRIMARIES])
    reflectance = saunderson(km(absorption / scatter), False)

    def mix(c):
        c = np.maximum(c, 0)
        return saunderson(km((c @ absorption) / (c @ scatter)), False) @ matrix.T

    count = len(PRIMARIES)
    vertex = np.eye(count)[-1]
    white = np.clip(mix(vertex), 0, 1)
    gain = (1 - K1) * (1 - K2)

    def unmix(rgb):
        if (white - rgb).max() < NEAR_WHITE:
            return vertex

        def error(c):
            den = c @ scatter
            q = (c @ absorption) / den
            r = km(q)
            surface = gain / (1 - K2 * r) ** 2
            raw = saunderson(r, False) @ matrix.T
            within = (raw > 0) & (raw < 1)
            diff = np.clip(raw, 0, 1) - rgb
            rdiff = raw - rgb
            jac = ((absorption - q * scatter) / den) * (-r / np.sqrt(np.maximum(q * (q + 2), 1e-24))) * surface
            return diff @ diff + RAW_FIT_WEIGHT * (rdiff @ rdiff), 2 * jac @ matrix.T @ (diff * within + RAW_FIT_WEIGHT * rdiff)

        fit = minimize(error, np.full(count, 1 / count), jac=True, method="SLSQP",
                       bounds=[(0, 1)] * count,
                       constraints=[{"type": "eq", "fun": lambda c: c.sum() - 1,
                                     "jac": lambda c: np.ones(count)}],
                       options={"ftol": 1e-11, "maxiter": 100})
        if not np.all(np.isfinite(fit.x)):
            raise RuntimeError(f"invalid unmix at {rgb}")
        c = np.maximum(0, fit.x)
        return c / c.sum()

    OUT.mkdir(exist_ok=True)
    sticks = warp(np.stack(np.meshgrid(*[np.linspace(0, 1, FORWARD_SIZE)] * (count - 1), indexing="ij"), axis=-1).reshape(-1, count - 1))
    rest = np.ones(len(sticks))
    concentrations = []
    for stick in sticks.T:
        concentrations.append(rest * stick)
        rest = rest * (1 - stick)
    concentrations = np.stack([*concentrations, rest], axis=-1)
    raw = mix(concentrations)
    forward = np.clip(raw, 0, 1)
    (OUT / "forward.bin").write_bytes(forward.astype("<f4").tobytes())
    axis = np.linspace(0, 1, INVERSE_SIZE)
    grid = np.stack(np.meshgrid(axis, axis, axis, indexing="ij"), axis=-1).reshape(-1, 3)
    linear = np.where(grid <= .04045, grid / 12.92, ((grid + .055) / 1.055)**2.4)
    inverse = np.array([unmix(rgb) for rgb in linear])
    quantized = np.floor(inverse[:, :-1] * 65535).astype("<u2")
    (OUT / "inverse.bin").write_bytes(quantized.tobytes())
    provenance = {
        "method": "Measured cobalt blue, chrome yellow lemon, eosin lake, bone black and zinc white oil paints (Revigo reconstruction of Van Gogh's palette); "
                  "K and S per wavelength from 100 um films over black and white, pure and mixed with zinc white and bone black; "
                  "plain linear two-constant KM mixing of weight concentrations; infinite-thickness reflectance shown specular-excluded; "
                  "forward table clipped to the sRGB cube, inverse fitted with clipped and raw RGB error with near-white nodes stored as the white vertex, "
                  "plus linear RGB residual",
        "source": "scripts/pigment_data/revigo (CC0, github.com/fligt/revigo-spectra; Geldof et al. 2018, doi:10.1186/s40494-018-0181-6); "
                  "the MIT spectral.rs D65/CIE matrix for the colour conversion",
        "primaries": PRIMARIES,
        "samples_used": used,
        "fit_rms_reflectance_over_black_and_white": fit_rms,
        "saunderson": {"k1": K1, "k2": K2, "measured": "specular included", "shown": "specular excluded"},
        "film_thickness_um": 100,
        "measured_wavelengths_nm": [int(x) for x in nm_data],
        "extension": "380-390 nm hold the 400 nm value, 710-750 nm hold the 700 nm value",
        "inverse_size": INVERSE_SIZE,
        "forward_size": FORWARD_SIZE,
        "latent": "five concentrations in the order of primaries, then the linear RGB residual",
        "concentration_law": "linear",
        "unmix_raw_fit_weight": RAW_FIT_WEIGHT,
        "near_white_distance": NEAR_WHITE,
        "near_white_nodes": int(((white - linear).max(axis=1) < NEAR_WHITE).sum()),
        "white_linear_rgb": white.tolist(),
        "raw_forward_range": [float(raw.min()), float(raw.max())],
        "clipped_forward_fraction": float(((raw < 0) | (raw > 1)).any(axis=1).mean()),
        "wavelengths_nm": nm.tolist(),
        "reflectance": reflectance.tolist(),
        "absorption": absorption.tolist(),
        "scattering": scatter.tolist(),
        "sha256": {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest()
                   for name in ["forward.bin", "inverse.bin"]},
    }
    (OUT / "primaries.json").write_text(json.dumps(provenance, indent=2) + "\n")
    print(json.dumps(provenance["sha256"], indent=2), flush=True)


if __name__ == "__main__":
    main()
