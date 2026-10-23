# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Generate Stillwet's pigment tables from measured reflectance of real oil paints, not Mixbox data.

The RGB reconstruction primaries are five paints of the Revigo project's reconstruction of Van Gogh's palette (Geldof,
Proano Gaibor, Ligterink, Hendriks and Kirchner 2018, doi:10.1186/s40494-018-0181-6), whose reflectance
measurements are published under CC0 in https://github.com/fligt/revigo-spectra and vendored in
scripts/pigment_data/revigo. Every paint was drawn down 100 um thick over the black and the white field
of an opacity chart, pure and mixed with zinc white and bone black by weight, and measured 400-700 nm
with specular included. Per wavelength this script solves the non-hiding Kubelka-Munk film equation
for each paint's absorption K and scattering S (the paper's own method), mixes the primaries by plain
linear two-constant KM (K and S add by normalized constant-weight concentration) and converts the
100 um film over a fixed measured white field to linear sRGB through the MIT spectral.js D65/CIE matrix.
"""

import ast
import hashlib
import json
import os
import re
import sys
from pathlib import Path

os.environ["OPENBLAS_NUM_THREADS"] = "1"

import numpy as np
from scipy.optimize import least_squares, minimize

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "scripts/pigment_data/revigo"
OUT = ROOT / "crates/paint/src/mixer"
INVERSE_SIZE = 33
RAW_FIT_WEIGHT = .05
MEASURED_REFINEMENT_STEPS = 3
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
    tau2 = K * (K + 2 * S)
    tau = np.sqrt(tau2)
    u = np.divide(np.tanh(tau), tau, out=np.ones_like(tau), where=np.abs(tau) >= 1e-4)
    u = np.where(np.abs(tau) < 1e-4, 1 - tau2 / 3, u)
    return (rg + (S * (1 - rg) - rg * K) * u) / (1 + (K + S * (1 - rg)) * u)


def km(q):
    return 1 / (1 + q + np.sqrt(q * (q + 2)))


def fit(nm, spectra, samples, unknown, known, ids, initial=None):
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

        if initial is None:
            starts = [np.array(start * len(unknown)) for start in ([0.0, 0.0], [2.0, -1.0], [-1.0, 1.0], [3.0, 1.0], [1.0, 3.0])]
        else:
            start = np.log([[initial[n][0][w], initial[n][1][w]] for n in unknown]).ravel()
            starts = [np.clip(start + np.log(scale), -9, 9) for scale in (1.0, .25, .0625)]
        best = None
        for start in starts:
            sol = least_squares(residual, start, bounds=(-9, 9), method="trf", jac="2-point" if initial is None else "cs",
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
    OUT.mkdir(parents=True, exist_ok=True)
    matrix = spectral_array("XYZ_RGB") @ spectral_array("CMF")
    nm_data, fitted, fit_rms, used = derive()
    measured = derive_measured(fitted, used)
    nm = np.arange(380, 751, 10)
    extend = lambda v: np.concatenate([np.full(2, v[0]), v, np.full(5, v[-1])])
    absorption = np.array([extend(fitted[name][0]) for name in PRIMARIES])
    scatter = np.array([extend(fitted[name][1]) for name in PRIMARIES])
    reference = np.array(measured["reference_substrate"])
    reflectance = saunderson(film(absorption, scatter, reference), False)

    def mix(c):
        c = np.maximum(c, 0)
        c = c / np.maximum(c.sum(axis=-1, keepdims=True), 1e-12)
        return saunderson(film(c @ absorption, c @ scatter, reference), False) @ matrix.T

    count = len(PRIMARIES)
    vertex = np.eye(count)[-1]
    white = np.clip(mix(vertex), 0, 1)
    gain = (1 - K1) * (1 - K2)

    def unmix(rgb):
        if (white - rgb).max() < NEAR_WHITE:
            return vertex

        def error(c):
            total = max(c.sum(), 1e-12)
            k, s = (c @ absorption) / total, (c @ scatter) / total
            r = film(k, s, reference)
            raw = saunderson(r, False) @ matrix.T
            within = (raw > 0) & (raw < 1)
            diff = np.clip(raw, 0, 1) - rgb
            rdiff = raw - rgb
            dk = np.imag(film(k + 1e-20j, s, reference)) / 1e-20
            ds = np.imag(film(k, s + 1e-20j, reference)) / 1e-20
            jac = ((absorption - k) * dk + (scatter - s) * ds) / total
            jac *= gain / (1 - K2 * r) ** 2
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

    axis = np.linspace(0, 1, INVERSE_SIZE)
    grid = np.stack(np.meshgrid(axis, axis, axis, indexing="ij"), axis=-1).reshape(-1, 3)
    linear = np.where(grid <= .04045, grid / 12.92, ((grid + .055) / 1.055)**2.4)
    inverse = np.array([unmix(rgb) for rgb in linear])
    quantized = np.floor(inverse[:, :-1] * 65535).astype("<u2")
    (OUT / "inverse.bin").write_bytes(quantized.tobytes())
    (OUT / "reference.bin").write_bytes(reference.astype("<f4").tobytes())
    provenance = {
        "method": "Measured cobalt blue, chrome yellow lemon, eosin lake, bone black and zinc white oil paints (Revigo reconstruction of Van Gogh's palette); "
                  "K and S per wavelength from 100 um films over black and white, pure and mixed with zinc white and bone black; "
                  "plain linear two-constant KM with calibrated constant recipe weights; finite 100 um film over the fixed measured 07A white field, shown specular-excluded; "
                  "exact spectral decoder clipped to the sRGB cube, inverse fitted with clipped and raw RGB error with near-white nodes stored as the white vertex, "
                  "plus linear RGB residual",
        "source": "scripts/pigment_data/revigo (CC0, github.com/fligt/revigo-spectra; Geldof et al. 2018, doi:10.1186/s40494-018-0181-6); "
                  "the MIT spectral.rs D65/CIE matrix for the colour conversion",
        "primaries": PRIMARIES,
        "samples_used": used,
        "samples_used_scope": "Original primary-fit split; final K/S are jointly calibrated on final_joint_fit_samples",
        "final_joint_fit_samples": measured["joint_fit_samples"],
        "initial_fit_rms_reflectance_over_black_and_white": fit_rms,
        "fit_rms_reflectance_over_black_and_white": {sid: measured["fit_rms_reflectance_over_black_and_white"][sid]
                                                   for sid in sorted(set(sum(used.values(), [])))},
        "saunderson": {"k1": K1, "k2": K2, "measured": "specular included", "shown": "specular excluded"},
        "film_thickness_um": 100,
        "measured_wavelengths_nm": [int(x) for x in nm_data],
        "extension": "380-390 nm hold the 400 nm value, 710-750 nm hold the 700 nm value",
        "inverse_size": INVERSE_SIZE,
        "latent": "18 measured pigment concentrations (the first five are RGB reconstruction primaries), then the linear RGB residual; see measured.json",
        "concentration_law": "linear",
        "unmix_raw_fit_weight": RAW_FIT_WEIGHT,
        "near_white_distance": NEAR_WHITE,
        "near_white_nodes": int(((white - linear).max(axis=1) < NEAR_WHITE).sum()),
        "white_linear_rgb": white.tolist(),
        "wavelengths_nm": nm.tolist(),
        "reflectance": reflectance.tolist(),
        "absorption": absorption.tolist(),
        "scattering": scatter.tolist(),
        "sha256": {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest()
                   for name in ["inverse.bin", "reference.bin", "measured.bin"]},
    }
    (OUT / "primaries.json").write_text(json.dumps(provenance, indent=2) + "\n")
    (OUT / "measured.json").write_text(json.dumps(measured, indent=2) + "\n")
    print(json.dumps(provenance["sha256"], indent=2), flush=True)


def derive_measured(fitted, primary_used):
    from pigment_strength_fit import measured_split, measured_weights

    nm, spectra, samples = load_data()
    pure, pairs, fit_ids, old_test, selected, declared = measured_split(
        samples, spectra, PRIMARIES, set(sum(primary_used.values(), [])))
    order = PRIMARIES + sorted(set(pure) - set(PRIMARIES))
    refit = [n for n in order[len(PRIMARIES):] if n not in ("BariumSulphate", "CalciumCarbonate")]
    used = dict(primary_used)
    for name in [*refit, "BariumSulphate", "CalciumCarbonate"]:
        ids = [] if pure[name] in declared["pure_samples_reported_apart"] else [pure[name]]
        if name in fit_ids:
            ids.append(fit_ids[name])
        ids += declared["additional_fit_samples"].get(name, [])
        assert not set(ids) & set(declared["test_set"]), "test mixtures cannot calibrate K or S"
        found, _ = fit(nm, spectra, samples, [name], fitted, ids)
        fitted.update(found)
        used[name] = ids
    extend = lambda v: np.concatenate([np.full(2, v[0]), v, np.full(5, v[-1])])
    reference = extend(inside(spectra["07A_w"]))
    weights, _ = measured_weights(fitted, samples, spectra, used)
    for _ in range(MEASURED_REFINEMENT_STEPS):
        weighted_samples = {}
        for sid, (names, fractions) in samples.items():
            w = np.array([c * weights.get(n, 1.0) for n, c in zip(names, fractions)])
            weighted_samples[sid] = (names, w / w.sum())
        for name in [*refit, "BariumSulphate", "CalciumCarbonate"]:
            found, _ = fit(nm, spectra, weighted_samples, [name], fitted, used[name])
            fitted.update(found)
        weights, _ = measured_weights(fitted, samples, spectra, used)
    joint_ids = sorted(set(sum(used.values(), [])))
    assert not set(joint_ids) & set(declared["test_set"])
    joint_used = {name: [sid for sid in joint_ids if dict(zip(*samples[sid])).get(name, 0) > 0] for name in order}
    weighted_samples = {}
    for sid, (names, fractions) in samples.items():
        w = np.array([c * weights.get(n, 1.0) for n, c in zip(names, fractions)])
        weighted_samples[sid] = (names, w / w.sum())
    found, rms = fit(nm, spectra, weighted_samples, order, {}, joint_ids, initial=fitted)
    fitted.update(found)
    coefficients = np.array([[extend(fitted[name][0]), extend(fitted[name][1])] for name in order])
    (OUT / "measured.bin").write_bytes(coefficients.astype("<f4").tobytes() + np.array([weights[n] for n in order], dtype="<f4").tobytes())
    return {
        "method": "Linear two-constant KM; each paint retains its own measured K and S. Finite 100 um film over fixed measured 07A white; one constant linear recipe weight per paint, with no concentration law.",
        "order": order,
        "samples_used": joint_used,
        "weight_fit_samples": used,
        "joint_fit_samples": joint_ids,
        "fit_rms_reflectance_over_black_and_white": rms,
        "fit_set": fit_ids,
        "old_test_set": old_test,
        "test_set": declared["test_set"],
        "additional_fit_samples": declared["additional_fit_samples"],
        "pure_samples_reported_apart": declared["pure_samples_reported_apart"],
        "coefficient_fit_weights": weights,
        "linear_weights": weights,
        "reference_sample": "07A_w",
        "reference_substrate": reference.tolist(),
        "local_refinement_steps": MEASURED_REFINEMENT_STEPS,
        "joint_refinement_steps": 1,
        "selection_touched": selected,
        "absorption": coefficients[:, 0].tolist(),
        "scattering": coefficients[:, 1].tolist(),
        "sha256": hashlib.sha256((OUT / "measured.bin").read_bytes()).hexdigest(),
    }


def build_researched():
    from researched_pigments import WAVELENGTHS, build, resample, validate_gum_tints

    data = ROOT / "scripts/pigment_data"
    manifest_path = data / "researched-tubes.json"
    manifest = json.loads(manifest_path.read_text())
    source_hashes = {}
    for directory in ("hyperspectral-pigments", "bath00183"):
        for filename, expected in json.loads((data / directory / "SHA256SUMS.json").read_text()).items():
            path = data / directory / filename
            actual = hashlib.sha256(path.read_bytes()).hexdigest()
            if actual != expected:
                raise ValueError(f"source checksum mismatch: {directory}/{filename}")
            source_hashes[f"{directory}/{filename}"] = actual
    source_hashes[manifest_path.name] = hashlib.sha256(manifest_path.read_bytes()).hexdigest()
    library = data / "hyperspectral-pigments"
    header = (library / "__speclib_medians.hdr").read_text()

    def field(name):
        return [v.strip() for v in re.search(r"^" + re.escape(name) + r"\s*=\s*\{([^}]+)\}", header, re.M).group(1).split(",")]

    nm = np.array(field("wavelength"), dtype=float)
    names = field("spectra names")
    products = np.array(field("pnumber"), dtype=int)
    spectra = np.fromfile(library / "__speclib_medians.sli", dtype="<f4").reshape(len(names), len(nm))
    visible = nm <= nm[np.searchsorted(nm, 750)]
    matrix = spectral_array("XYZ_RGB") @ spectral_array("CMF")

    def bath(sample):
        paths = sorted((data / "bath00183/raw").glob(sample + "_Spectrum*.txt"))
        if len(paths) != 3:
            raise ValueError(f"expected all three Bath spots: {sample}")
        observations = np.array([np.loadtxt(path, skiprows=1) for path in paths])
        if not np.all(observations[:, :, 0] == observations[0, :, 0]):
            raise ValueError(f"inconsistent Bath wavelengths: {sample}")
        band = (observations[0, :, 0] >= 380) & (observations[0, :, 0] <= 750)
        values = observations[:, band, 1]
        return {
            "name": sample,
            "source_nm": observations[0, band, 0].tolist(),
            "source_reflectance": values.mean(axis=0).tolist(),
            "source_files": [str(path.relative_to(data)) for path in paths],
            "spot_range_rms": float(np.sqrt(np.mean(np.ptp(values, axis=0) ** 2))),
            "spot_min": values.min(axis=0).tolist(),
            "spot_max": values.max(axis=0).tolist(),
        }

    entries = []
    for row in manifest["tubes"]:
        source = row["source"]
        if source is None:
            continue
        if source["dataset"] == "hyperspectral-pigments":
            index = names.index(source["sample"])
            if products[index] != source["product"]:
                raise ValueError(f"wrong source product for {row['name']}")
            observation = {"source_nm": nm[visible].tolist(), "source_reflectance": spectra[index, visible].astype(float).tolist()}
        elif source["dataset"] == "bath00183":
            observation = bath(source["sample"])
        else:
            raise ValueError(f"unknown dataset: {source['dataset']}")
        entries.append({**observation, "name": row["name"], "hiding": row["hiding_estimate"],
                        "source_class": row["source_class"], "provenance": source})
    coefficients, metadata = build(
        entries, np.ones(len(WAVELENGTHS)), matrix,
        reference_name="assumed ideal white, not a measured source backing",
        reference_assumption="The non-oil source is modeled as a 100 um oil film over ideal inside-white R=1. Source backing, thickness, loading and binder conversion are unknown. Runtime reference remains measured Revigo 07A_w.",
    )
    order = list(coefficients)
    if not order:
        raise ValueError("no source spectra satisfy the declared stand-in model")
    binary = np.array([coefficients[name] for name in order], dtype="<f4").tobytes()
    metadata.update({"order": order, "source_sha256": source_hashes,
                     "license": "CC BY 4.0; source attribution in scripts/pigment_data/*/ATTRIBUTION.md and THIRD_PARTY_NOTICES.md",
                     "sha256": hashlib.sha256(binary).hexdigest(), "tube_coverage": manifest["tubes"]})
    for row in metadata["tube_coverage"]:
        fit = metadata["entries"].get(row["name"])
        row["runtime_provenance"] = "measured non-oil reflectance; modeled oil stand-in" if row["name"] in coefficients else "catalog RGB and estimated opacity; no measured spectrum wired"
        if fit is not None and row["name"] not in coefficients:
            row["not_wired_reason"] = fit["failure"]
    tints = []
    for sample, fraction in (("LW-LTY_33.3-66.7", .667), ("LW-LTY_50.0-50.0", .5), ("LW-LTY_66.7-33.3", .333)):
        tints.append({**bath(sample), "pigment_mass_fraction": fraction})
    metadata["bath_tint_validation"] = validate_gum_tints(bath("LTY_Pure"), None, tints, {"LW-LTY_50.0-50.0"})
    metadata["bath_measured_tints"] = tints
    metadata["bath_tint_validation"]["interpretation"] = "A failed positive opaque fit is evidence against using this variable-thickness gum/glass ladder as calibrated oil mixing data. Spot spread and all tint means are retained, not filtered to improve the score."
    mixtures = []
    for sample, fraction in (("RL-LTY_33.3-66.7", .667), ("RL-LTY_50.0-50.0", .5), ("RL-LTY_66.7-33.3", .333)):
        mixtures.append({**bath(sample), "pigment_mass_fraction": fraction})
    metadata["bath_chromatic_validation"] = validate_gum_tints(bath("LTY_Pure"), bath("RL_Pure"), mixtures, {"RL-LTY_50.0-50.0"}, partner_label="red lead")
    metadata["bath_measured_chromatic_mixtures"] = mixtures
    metadata["bath_pure_references"] = [bath("LTY_Pure"), bath("RL_Pure")]
    endpoints = np.array([resample(row["source_nm"], row["source_reflectance"])[0] for row in metadata["bath_pure_references"]])
    violations = {}
    for mixture in mixtures:
        observed = resample(mixture["source_nm"], mixture["source_reflectance"])[0]
        error = np.maximum(0, np.maximum(endpoints.min(axis=0) - observed, observed - endpoints.max(axis=0)))
        violations[mixture["name"]] = {"outside_endpoint_bands_nm": WAVELENGTHS[error > 1e-10].tolist(), "maximum_excess_reflectance": float(error.max())}
    metadata["bath_chromatic_validation"]["endpoint_range_check"] = violations
    metadata["bath_chromatic_validation"]["endpoint_range_meaning"] = "Positive opaque two-constant mixtures stay between both pure reflectance endpoints at every wavelength. Violations reject that preparation/model assumption without interpreting a failed fit as measured oil coefficients."
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "researched.bin").write_bytes(binary)
    (OUT / "researched.json").write_text(json.dumps(metadata, indent=2, ensure_ascii=False) + "\n")
    rust = f"pub(super) const RESEARCHED_TUBES: [&str; {len(order)}] = [\n"
    rust += "".join(f"    {json.dumps(name)},\n" for name in order) + "];\n"
    (OUT / "researched.rs").write_text(rust)
    print(json.dumps({"modeled_non_oil_references": len(order), "remaining_catalog_stand_ins": len(manifest["tubes"]) - len(order),
                      "bath_validation": metadata["bath_tint_validation"]["status"], "sha256": metadata["sha256"]}, indent=2))


if __name__ == "__main__":
    if sys.argv[1:] == ["--researched"]:
        build_researched()
    elif not sys.argv[1:]:
        main()
    else:
        raise SystemExit("usage: build_pigment_tables.py [--researched]")
