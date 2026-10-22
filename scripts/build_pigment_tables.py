# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Generate Stillwet's pigment tables from synthetic spectra, not Mixbox data."""

import ast
import hashlib
import json
import os
import re
from pathlib import Path

os.environ["OPENBLAS_NUM_THREADS"] = "1"

import numpy as np
from scipy.optimize import minimize

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "crates/paint/src/mixer"
SIZE = 33
POWER = 1.52
RAW_FIT_WEIGHT = .05


def spectral_array(name):
    source = (ROOT / "crates/paint/src/spectral.rs").read_text()
    value = source.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", value).strip()))


def km(q):
    return 1 / (1 + q + np.sqrt(q * (q + 2)))


def sigmoid(x):
    return 1 / (1 + np.exp(-x))


def warp(x):
    return x**2 / (x**2 + (1 - x)**2)


def effective(c):
    c = np.maximum(c, 0) ** POWER
    return c / c.sum(axis=-1, keepdims=True)


def main():
    matrix = spectral_array("XYZ_RGB") @ spectral_array("CMF")
    nm = np.arange(380, 751, 10)
    blue_band = np.where(nm < 450, np.exp(-((nm - 450) / 37.7043)**2), np.exp(-((nm - 450) / 32.4209)**2))
    reflectance = np.array([
        .005 + .6255 * blue_band,
        .0053 + .93 * sigmoid((nm - 489.017) / 3.4857),
        .0049 + .3602 * sigmoid((438.7356 - nm) / 6.316) + .5927 * sigmoid((nm - 624.8778) / 11.4996),
        .6739 + (.8613 - .6739) * sigmoid((nm - 491.1) / 42.74),
    ])
    reflectance = np.clip(reflectance, 1e-4, .999)
    scatter = np.array([.0317, .1718, .1105, 1.0])[:, None] * np.ones((4, 38))
    absorption = scatter * (1 - reflectance)**2 / (2 * reflectance)

    def mix(c):
        c = effective(c)
        return km((c @ absorption) / (c @ scatter)) @ matrix.T

    def unmix(rgb):
        def error(c):
            den = c @ scatter
            q = (c @ absorption) / den
            r = km(q)
            raw = r @ matrix.T
            inside = (raw > 0) & (raw < 1)
            diff = np.clip(raw, 0, 1) - rgb
            rdiff = raw - rgb
            jac = ((absorption - q * scatter) / den) * (-r / np.sqrt(q * (q + 2)))
            return diff @ diff + RAW_FIT_WEIGHT * (rdiff @ rdiff), 2 * jac @ matrix.T @ (diff * inside + RAW_FIT_WEIGHT * rdiff)
        fit = minimize(error, np.full(4, .25), jac=True, method="SLSQP",
                       bounds=[(0, 1)] * 4,
                       constraints=[{"type": "eq", "fun": lambda c: c.sum() - 1,
                                     "jac": lambda c: np.ones(4)}],
                       options={"ftol": 1e-11, "maxiter": 100})
        if not np.all(np.isfinite(fit.x)):
            raise RuntimeError(f"invalid unmix at {rgb}")
        c = np.maximum(0, fit.x)
        return c / c.sum()

    OUT.mkdir(exist_ok=True)
    axis = np.linspace(0, 1, SIZE)
    grid = np.stack(np.meshgrid(axis, axis, axis, indexing="ij"), axis=-1).reshape(-1, 3)
    u, v, w = warp(grid).T
    concentrations = np.stack([u, (1-u)*v, (1-u)*(1-v)*w, (1-u)*(1-v)*(1-w)], axis=-1)
    raw = mix(concentrations)
    forward = np.clip(raw, 0, 1)
    (OUT / "forward.bin").write_bytes(forward.astype("<f4").tobytes())
    linear = np.where(grid <= .04045, grid / 12.92, ((grid + .055) / 1.055)**2.4)
    inverse = np.array([unmix(rgb) for rgb in linear])
    inverse = inverse ** (1 / POWER)
    inverse /= inverse.sum(axis=1, keepdims=True)
    quantized = np.floor(inverse[:, :3] * 65535).astype("<u2")
    (OUT / "inverse.bin").write_bytes(quantized.tobytes())
    provenance = {
        "method": "Synthetic blue, yellow, magenta and warm-white primaries; power-adjusted independent KM concentrations, "
                  "forward table clipped to the sRGB cube, inverse fitted with clipped and raw RGB error, "
                  "plus linear RGB residual",
        "source": "scripts/build_pigment_tables.py; only the MIT spectral.rs D65/CIE matrix is reused",
        "size": SIZE,
        "concentration_power": POWER,
        "unmix_raw_fit_weight": RAW_FIT_WEIGHT,
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
