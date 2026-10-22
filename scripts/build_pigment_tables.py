# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Generate Stillwet's pigment tables from synthetic spectra, not Mixbox data."""

import ast
import hashlib
import json
import re
from pathlib import Path

import numpy as np
from scipy.optimize import minimize

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "crates/paint/src/mixer"
SIZE = 33


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


def main():
    matrix = spectral_array("XYZ_RGB") @ spectral_array("CMF")
    nm = np.arange(380, 751, 10)
    reflectance = np.array([
        .008 + .06 * np.exp(-((nm - 440) / 80)**2),
        .003 + .55 * sigmoid((nm - 481) / 6) + .15 * sigmoid((nm - 500) / 10),
        .008 + .04 * np.exp(-((nm - 430) / 40)**2) + .25 * sigmoid((nm - 600) / 16),
        np.full(38, .93),
    ])
    scatter = np.array([.1, .8, .1, 3.125])[:, None] * np.ones((4, 38))
    absorption = scatter * (1 - reflectance)**2 / (2 * reflectance)
    boundary = np.array([
        (i, j, k, 16 - i - j - k)
        for i in range(17) for j in range(17 - i) for k in range(17 - i - j)
        if min(i, j, k, 16 - i - j - k) == 0
    ]) / 16
    surface = np.array([(i, j, k) for i in range(17) for j in range(17)
                        for k in range(17) if min(i, j, k) == 0 or max(i, j, k) == 16]) / 16
    u, v, w = warp(surface).T
    boundary = np.concatenate([boundary, np.stack([
        u, (1-u)*v, (1-u)*(1-v)*w, (1-u)*(1-v)*(1-w)
    ], axis=-1)])
    denominator = boundary @ scatter
    original = km((boundary @ absorption) / denominator) @ matrix.T
    start = reflectance.copy()

    def objective(flat):
        r = flat.reshape(4, 38)
        k = scatter * (1 - r)**2 / (2 * r)
        q = (boundary @ k) / denominator
        reflected = km(q)
        rgb = reflected @ matrix.T
        outside = rgb - np.clip(rgb, .008, .992)
        pull = rgb - original
        loss = np.sum(pull**2) + 5000 * np.sum(outside**2) + .03 * np.sum((r - start)**2)
        grad_rgb = 2 * pull + 10000 * outside
        grad_r = (grad_rgb @ matrix) * (-reflected / np.sqrt(q * (q + 2))) / denominator
        grad = (boundary.T @ grad_r) * scatter * (.5 - .5 / r**2) + .06 * (r - start)
        return loss, grad.ravel()

    fit = minimize(objective, start.ravel(), jac=True, method="L-BFGS-B",
                   bounds=[(.0001, .999)] * 152,
                   options={"maxiter": 500, "ftol": 1e-12, "gtol": 1e-6, "maxcor": 20})
    reflectance = fit.x.reshape(4, 38)
    absorption = scatter * (1 - reflectance)**2 / (2 * reflectance)

    def mix(c):
        return km((c @ absorption) / (c @ scatter)) @ matrix.T

    def unmix(rgb):
        def error(c):
            den = c @ scatter
            q = (c @ absorption) / den
            r = km(q)
            diff = r @ matrix.T - rgb
            jac = ((absorption - q * scatter) / den) * (-r / np.sqrt(q * (q + 2)))
            return diff @ diff, 2 * jac @ matrix.T @ diff
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
    forward = mix(concentrations)
    assert forward.min() > -.001 and forward.max() < 1.001, (forward.min(), forward.max())
    (OUT / "forward.bin").write_bytes(forward.astype("<f4").tobytes())
    linear = np.where(grid <= .04045, grid / 12.92, ((grid + .055) / 1.055)**2.4)
    inverse = np.array([unmix(rgb) for rgb in linear])
    quantized = np.floor(inverse[:, :3] * 65535).astype("<u2")
    (OUT / "inverse.bin").write_bytes(quantized.tobytes())
    provenance = {
        "method": "Synthetic blue, yellow, magenta and white; independent KM concentrations plus linear RGB residual",
        "source": "scripts/build_pigment_tables.py; only the MIT spectral.rs D65/CIE matrix is reused",
        "size": SIZE,
        "wavelengths_nm": nm.tolist(),
        "absorption": absorption.tolist(),
        "scattering": scatter.tolist(),
        "sha256": {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest()
                   for name in ["forward.bin", "inverse.bin"]},
    }
    (OUT / "primaries.json").write_text(json.dumps(provenance, indent=2) + "\n")
    print(json.dumps(provenance["sha256"], indent=2), flush=True)


if __name__ == "__main__":
    main()
