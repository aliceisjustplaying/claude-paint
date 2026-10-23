# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Bounded engine-9 physical-volume calibration. Historical tables and inverse stay unchanged."""
import argparse
import hashlib
import json
import os
from pathlib import Path
os.environ["OPENBLAS_NUM_THREADS"] = "1"
import numpy as np
from scipy.optimize import least_squares
from scipy.sparse import eye, lil_matrix, vstack
from build_pigment_tables import ROOT, DATA, OUT, load_data, inside, spectral_array, saunderson, film


def paint_densities():
    data = json.loads((ROOT / "scripts/pigment_densities.json").read_text())
    densities = {}
    for name, paint in data["paints"].items():
        oil = paint.get("oil_mass_fraction")
        if oil is None:
            absorption = paint["oil_absorption_estimate"]
            oil = 1.45 * absorption / (100 + 1.45 * absorption)
        densities[name] = 1 / (oil / data["oil_densities"][paint["oil"]]["value"] + (1 - oil) / paint["pigment_density"])
    return densities


def mass_to_volume(names, fractions, densities):
    volumes = np.array([c / densities[n] if c > 0 else 0 for n, c in zip(names, fractions)])
    return volumes / volumes.sum()


def calibrate_volume(out):
    from pigment_basis_select import measured_colour, oklab

    legacy = json.loads((OUT / "measured.json").read_text())
    declared = json.loads((ROOT / "scripts/pigment_fit_split9.json").read_text())
    densities = paint_densities()
    nm, spectra, samples = load_data()
    order = legacy["order"]
    ids = sorted(set(legacy["joint_fit_samples"] + declared["dark_fit_samples"]))
    assert not set(ids) & set(declared["test_set"]), "held-out spectra cannot calibrate K or S"
    coefficients = np.stack([legacy["absorption"], legacy["scattering"]], axis=1)[:, :, 2:33]
    for paint, name in enumerate(order):
        coefficients[paint] *= legacy["linear_weights"][name] * densities[name] / densities["ZincWhite"]
    fixed_prior = np.clip(np.log(coefficients), -9, 9)
    coefficients = np.exp(fixed_prior)
    concentrations = np.zeros((len(ids), len(order)))
    for row, sid in enumerate(ids):
        names, fractions = samples[sid]
        for name, volume in zip(names, mass_to_volume(names, fractions, densities)):
            if volume > 0:
                concentrations[row, order.index(name)] += volume
    backs = np.array([[inside(spectra[sid + field]) for field in ("_b", "_w")] for sid in ids])
    observed = np.array([[spectra[sid + field] for field in ("_ob", "_ow")] for sid in ids])
    matrix = spectral_array("XYZ_RGB") @ spectral_array("CMF")
    matrix31 = matrix[:, 2:33].copy()
    matrix31[:, 0] += matrix[:, :2].sum(axis=1)
    matrix31[:, -1] += matrix[:, 33:].sum(axis=1)
    reference = np.array(legacy["reference_substrate"])[2:33]
    labs = np.array([oklab(measured_colour(spectra, sid, matrix)) for sid in ids])
    receipts = []

    def residual(values, rows):
        k, s = np.einsum("ij,jkl->kil", concentrations[rows], values)
        rgb = np.clip(saunderson(film(k, s, reference), False) @ matrix31.T, 0, 1)
        color = np.array([oklab(r) for r in rgb]) - labs[rows]
        raw = saunderson(film(k[:, None, :], s[:, None, :], backs[rows]), True) - observed[rows]
        return color, raw

    for iteration in range(10):
        for paint, name in enumerate(order):
            rows = np.flatnonzero(concentrations[:, paint])
            initial = np.log(coefficients[paint]).ravel()

            def objective(p):
                values = coefficients.copy()
                values[paint] = np.exp(p.reshape(2, len(nm)))
                color, raw = residual(values, rows)
                return np.concatenate([color.ravel(), .05 * raw.ravel(), .001 * (p - fixed_prior[paint].ravel()),
                                       .01 * np.diff(p.reshape(2, len(nm)), n=2, axis=1).ravel()])

            result = least_squares(objective, initial, bounds=(-9, 9), max_nfev=120,
                                   ftol=1e-7, xtol=1e-7, gtol=1e-7)
            coefficients[paint] = np.exp(result.x.reshape(2, len(nm)))
            receipts.append({"pass": iteration, "paint": name, "evaluations": result.nfev, "cost": result.cost})
    initial = np.log(coefficients).ravel()
    color_scale = np.array([2 if sid in declared["dark_focus_samples"] else 1 for sid in ids])

    def objective(p):
        color, raw = residual(np.exp(p.reshape(coefficients.shape)), np.arange(len(ids)))
        return np.concatenate([(color * color_scale[:, None]).ravel(), .05 * raw.ravel(), .001 * (p - fixed_prior.ravel()),
                               .01 * np.diff(p.reshape(coefficients.shape), n=2, axis=2).ravel()])

    pattern = lil_matrix((len(ids) * 65, len(initial)), dtype=int)
    for row in range(len(ids)):
        for paint in np.flatnonzero(concentrations[row]):
            for ks in range(2):
                for w in range(len(nm)):
                    column = (paint * 2 + ks) * len(nm) + w
                    pattern[row * 3:row * 3 + 3, column] = 1
                    pattern[len(ids) * 3 + row * 62 + w, column] = 1
                    pattern[len(ids) * 3 + row * 62 + 31 + w, column] = 1
    curvature = lil_matrix((len(order) * 2 * (len(nm) - 2), len(initial)), dtype=int)
    for paint_ks in range(len(order) * 2):
        for w in range(len(nm) - 2):
            curvature[paint_ks * (len(nm) - 2) + w, paint_ks * len(nm) + w:paint_ks * len(nm) + w + 3] = 1
    pattern = vstack([pattern.tocsr(), eye(len(initial), format="csr"), curvature.tocsr()])
    result = least_squares(objective, initial, bounds=(-9, 9), jac_sparsity=pattern, max_nfev=100,
                           ftol=1e-7, xtol=1e-7, gtol=1e-7)
    coefficients = np.exp(result.x.reshape(coefficients.shape))
    _, raw = residual(coefficients, np.arange(len(ids)))
    coefficients = np.concatenate([np.repeat(coefficients[:, :, :1], 2, axis=2), coefficients,
                                   np.repeat(coefficients[:, :, -1:], 5, axis=2)], axis=2)
    out.mkdir(parents=True, exist_ok=True)
    (out / "calibrated.bin").write_bytes(coefficients.astype("<f4").tobytes() + np.ones(len(order), dtype="<f4").tobytes())
    (out / "calibrated-densities.bin").write_bytes(np.array([densities[n] for n in order], dtype="<f4").tobytes())
    metadata = {
        "method": "Physical per-volume additive K/S at 100 um; source mass fractions converted to estimated wet-paint volume fractions. No normalized strength law. Engine 9 and later only.",
        "encoding": "volume-ks", "order": order,
        "absorption": coefficients[:, 0].tolist(), "scattering": coefficients[:, 1].tolist(),
        "linear_weights": {n: 1.0 for n in order}, "paint_densities": densities,
        "density_status": "All wet-paint densities are estimates, not measurements of the Revigo paints; scripts/pigment_densities.json records inputs and sources.",
        "joint_fit_samples": ids, "test_set": declared["test_set"], "old_test_set": legacy["old_test_set"],
        "fit_set": legacy["fit_set"], "additional_fit_samples": legacy["additional_fit_samples"],
        "dark_fit_samples": declared["dark_fit_samples"], "dark_focus_samples": declared["dark_focus_samples"],
        "selection_touched": legacy["selection_touched"], "pure_samples_reported_apart": legacy["pure_samples_reported_apart"],
        "reference_sample": "07A_w", "reference_substrate": legacy["reference_substrate"],
        "fit_rms_reflectance_over_black_and_white": {sid: np.sqrt(np.mean(raw[row] ** 2, axis=1)).tolist() for row, sid in enumerate(ids)},
        "solver": {"coordinate_passes": 10, "coordinate_max_nfev": 120, "joint_max_nfev": 100,
                   "raw_reflectance_residual_scale": .05, "log_coefficient_prior_scale": .001,
                   "fixed_prior": "Legacy K/S times legacy mass strength times wet-paint density / zinc-white wet-paint density, log-clipped to [-9,9]; never reset during optimization",
                   "log_spectral_second_difference_scale": .01,
                   "dark_focus_color_scale": 2, "coordinate_receipts": receipts,
                   "joint_evaluations": result.nfev, "joint_status": result.status, "joint_cost": result.cost},
        "input_sha256": {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in
                         [OUT / "measured.json", OUT / "measured.bin", OUT / "reference.bin",
                          ROOT / "scripts/pigment_fit_split9.json", ROOT / "scripts/pigment_densities.json", *sorted(DATA.glob("*.txt"))]},
        "sha256": {name: hashlib.sha256((out / name).read_bytes()).hexdigest() for name in ["calibrated.bin", "calibrated-densities.bin"]},
    }
    (out / "calibrated.json").write_text(json.dumps(metadata, indent=2) + "\n")
    print(json.dumps(metadata["sha256"], indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, default=OUT)
    args = parser.parse_args()
    calibrate_volume(args.out_dir)
