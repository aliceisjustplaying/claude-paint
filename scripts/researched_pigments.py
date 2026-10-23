"""Explicitly modeled oil-film coefficients from reflectance-only appearance targets.

A source spectrum does not identify oil K and S. This model assumes its reflectance
is the desired diffuse appearance of a 100 um oil film on an explicitly supplied
backing. A wavelength-flat S and an estimated catalog contrast ratio at 25 um close
the inverse problem. They are modeling constraints, not experimental oil evidence.
"""

import numpy as np
from scipy.optimize import brentq, least_squares

import build_pigment_tables as gen


WAVELENGTHS = np.arange(380, 751, 10)
LUMINANCE = np.array([.2126, .7152, .0722])


def resample(source_nm, source_reflectance):
    """Interpolate reflectance fractions without clipping or extrapolated slopes."""
    nm = np.asarray(source_nm, dtype=float)
    values = np.asarray(source_reflectance, dtype=float)
    if nm.ndim != 1 or len(nm) < 2 or values.shape != nm.shape:
        raise ValueError("source wavelengths and reflectances must be equal-length vectors with at least two samples")
    if not np.all(np.isfinite(nm)) or not np.all(np.diff(nm) > 0):
        raise ValueError("source wavelengths must be finite and strictly increasing")
    if not np.all(np.isfinite(values)) or np.any((values < 0) | (values > 1)):
        raise ValueError("source reflectance must be finite fractions in [0, 1]; no clipping is permitted")
    return np.interp(WAVELENGTHS, nm, values), {
        "method": "piecewise linear interpolation, constant endpoint extension",
        "source_wavelengths_nm": nm.tolist(),
        "output_wavelengths_nm": WAVELENGTHS.tolist(),
        "extended_wavelengths_nm": WAVELENGTHS[(WAVELENGTHS < nm[0]) | (WAVELENGTHS > nm[-1])].tolist(),
    }


def display_inside(reflectance):
    """Invert the existing diffuse display boundary, not the source's geometry."""
    r = np.asarray(reflectance, dtype=float)
    maximum = float(gen.saunderson(np.array(1.0), False))
    if not np.all(np.isfinite(r)) or np.any((r <= 0) | (r >= maximum)):
        raise ValueError(f"assumed diffuse oil target must be strictly between 0 and {maximum:g}; finite positive K/S cannot reproduce it")
    return r / ((1 - gen.K1) * (1 - gen.K2) + gen.K2 * r)


def opacity(K, S, matrix):
    """Displayed Y-black/Y-white of a 25 um film, with ideal inside backings."""
    y = LUMINANCE @ matrix
    black = gen.saunderson(gen.film(.25 * K, .25 * S, np.zeros_like(K)), False) @ y
    white = gen.saunderson(gen.film(.25 * K, .25 * S, np.ones_like(K)), False) @ y
    if not np.isfinite(black) or not np.isfinite(white) or white <= 0:
        raise ValueError("opacity calculation has nonpositive or nonfinite white luminance")
    return float(black / white)


def _absorption(target, reference, scatter):
    low = np.zeros_like(target)
    high = np.ones_like(target)
    if np.any(gen.film(low, scatter, reference) < target - 1e-13):
        raise ValueError("target exceeds the zero-absorption reflectance at this scattering scale")
    for _ in range(50):
        needs_more = gen.film(high, scatter, reference) > target
        if not np.any(needs_more):
            break
        high = np.where(needs_more, high * 2, high)
    else:
        raise ValueError("could not bracket finite absorption coefficients")
    for _ in range(75):
        mid = (low + high) / 2
        above = gen.film(mid, scatter, reference) > target
        low = np.where(above, mid, low)
        high = np.where(above, high, mid)
    return (low + high) / 2


def build(entries, reference, matrix, *, reference_name=None, reference_assumption=None):
    """Return successful name -> (K, S) arrays and JSON-safe metadata for all rows.

    Entries contain name, source_nm, source_reflectance, hiding and provenance.
    Optional source_rank/source_class labels are copied verbatim. Reference is
    the inside reflectance of the assumed source-fit backing, not necessarily
    the runtime substrate; its name and assumption are recorded as supplied.
    Matrix is XYZ_RGB @ CMF. K and S are
    optical coefficients per 100 um, with no fitted concentration weights.
    Invalid shared inputs raise ValueError; invalid entries get failure records.
    """
    reference = np.asarray(reference, dtype=float)
    matrix = np.asarray(matrix, dtype=float)
    if reference.shape != (len(WAVELENGTHS),) or not np.all(np.isfinite(reference)) or np.any((reference <= 0) | (reference > 1)):
        raise ValueError("reference must contain 38 finite inside reflectances in (0, 1]")
    if matrix.shape != (3, len(WAVELENGTHS)) or not np.all(np.isfinite(matrix)) or np.any(LUMINANCE @ matrix < 0) or (LUMINANCE @ matrix).sum() <= 0:
        raise ValueError("matrix must be finite 3 by 38 with nonnegative, nonzero luminance weights")
    names = [entry["name"] for entry in entries]
    if any(not isinstance(name, str) or not name for name in names) or len(set(names)) != len(names):
        raise ValueError("entry names must be unique nonempty strings")
    coefficients = {}
    metadata = {
        "method": "reflectance-constrained modeled oil-film coefficients, not measured oil K and S",
        "quantity_identified": "reflectance only",
        "coefficient_unit": "per 100 um",
        "reference_substrate": reference.tolist(),
        "reference_sample": reference_name,
        "reference_assumption": reference_assumption,
        "reference_scope": "caller-supplied backing for assumed source appearance fit only; runtime decoding substrate is independent",
        "appearance_assumption": "source diffuse reflectance directly targets displayed 100 um oil-film appearance; source measurement geometry unspecified, not asserted SCI",
        "ks_inference_basis": "positive K at each wavelength and one shared wavelength-flat S, solved jointly with estimated catalog 25 um hiding",
        "flat_s_limitation": "wavelength-dependent scattering and real oil binder effects are not identified; tint and glaze predictions depend on this assumption",
        "saunderson": {"k1": gen.K1, "k2": gen.K2, "target_boundary": "specular excluded display inversion, not source-measurement correction"},
        "opacity_constraint": {"thickness_um": 25, "backings_inside": [0, 1], "quantity": "unclipped displayed luminance black/white", "evidence": "estimated catalog constraint, not experimental validation"},
        "validation": "source-fit residuals are in-sample reconstruction, not held-out validation",
        "entries": {},
    }
    for entry in entries:
        name = entry["name"]
        row = {
            "status": "failed",
            "provenance": entry.get("provenance"),
            "source_rank": entry.get("source_rank"),
            "source_class": entry.get("source_class", "reflectance-only source; oil appearance transfer assumed"),
        }
        metadata["entries"][name] = row
        try:
            measured, interpolation = resample(entry["source_nm"], entry["source_reflectance"])
            row["interpolation"] = interpolation
            target = display_inside(measured)
            hiding = float(entry["hiding"])
            row["target_hiding_estimated"] = hiding if np.isfinite(hiding) else None
            if not np.isfinite(hiding) or not 0 < hiding < 1:
                raise ValueError("estimated hiding must be strictly between 0 and 1 for finite positive coefficients")
            denominator = (1 - reference) * (1 - target)
            required = np.divide(target - reference, denominator, out=np.zeros_like(target), where=target > reference)
            lower = max(1e-12, float(required.max()) * (1 + 1e-10) + 1e-12)

            def at(scatter):
                K = _absorption(target, reference, scatter)
                S = np.full_like(K, scatter)
                return K, S, opacity(K, S, matrix)

            K, S, minimum = at(lower)
            row["minimum_hiding_at_positive_k_boundary"] = minimum
            if hiding < minimum - 1e-9:
                raise ValueError(f"estimated hiding {hiding:g} is below attainable minimum {minimum:g} for this bright appearance target")
            upper = max(1.0, lower * 2)
            for _ in range(40):
                if at(upper)[2] >= hiding:
                    break
                upper *= 2
            else:
                raise ValueError("could not bracket scattering scale for the estimated hiding constraint")
            scatter = lower if abs(minimum - hiding) < 1e-10 else brentq(lambda s: at(s)[2] - hiding, lower, upper, xtol=1e-11, rtol=1e-12)
            K, S, achieved = at(scatter)
            residual = gen.saunderson(gen.film(K, S, reference), False) - measured
            if not np.all(np.isfinite(K)) or not np.all(K > 0) or not np.all(S > 0):
                raise ValueError("solution does not have finite positive K and S")
            if np.max(np.abs(residual)) > 1e-8 or abs(achieved - hiding) > 1e-8:
                raise ValueError("solution fails source-fit or opacity numerical tolerance")
            row.update({
                "status": "modeled",
                "source_fit_rms_reflectance": float(np.sqrt(np.mean(residual ** 2))),
                "source_fit_max_abs_reflectance": float(np.max(np.abs(residual))),
                "source_fit_residual_reflectance": residual.tolist(),
                "achieved_hiding": achieved,
                "hiding_residual": achieved - hiding,
                "scattering_flat_per_100um": scatter,
                "ks_ratio": (K / S).tolist(),
            })
            coefficients[name] = (K, S)
        except (ValueError, KeyError, TypeError, OverflowError) as exc:
            row["failure"] = str(exc)
    metadata["order"] = list(coefficients)
    return coefficients, metadata


def validate_gum_tints(pigment, white, tints, held_out, *, partner_label="lead white"):
    """Fit relative opaque KM on dry pigment mass, then predict unused mixtures.

    Pure entries use source_nm/source_reflectance. Each tint additionally has a
    unique name and pigment_mass_fraction in (0, 1). held_out contains tint names
    excluded from fitting. With white=None, exactly two training tints jointly
    determine relative pigment S and unknown gum-white K/S, if a positive solution
    exists. The legacy white argument supplies the reference partner spectrum;
    partner_label identifies a nonwhite partner. Its S is the unit gauge, not an
    absolute physical coefficient. No gum mass fraction is converted to oil volume.
    """
    report = {
        "status": "failed",
        "domain": "source gum-arabic tints only, not modeled oil-film validation",
        "model": "relative opaque two-constant KM on reported source reflectance, without a source surface correction",
        "assumptions": "source samples are optically opaque; K and S add by dry pigment mass; common binder and preparation across the ladder",
        "fraction_basis": "dry pigment mass, not paint volume; no oil-volume conversion inferred",
        "coefficient_gauge": "white S = 1 at each wavelength; coefficients relative per dry pigment mass, not per 100 um",
        "identifiability": "absolute K, S and film thickness are not identified; all results depend on the opaque dry-mass mixing assumption",
        "repeat_scope": "source spot repeats are repeat measurements, not independent prepared films",
        "oil_validation": False,
    }
    partner_key = "white" if partner_label == "lead white" else "partner"
    if partner_key == "partner":
        report["partner_label"] = partner_label
        report["domain"] = "source gum-arabic mixtures only, not modeled oil-film validation"
        report["coefficient_gauge"] = f"reference partner ({partner_label}) S = 1 at each wavelength; coefficients relative per dry pigment mass, not per 100 um"
    try:
        if not isinstance(partner_label, str) or not partner_label.strip():
            raise ValueError("partner_label must be a nonempty string")
        names = [tint["name"] for tint in tints]
        if len(set(names)) != len(names):
            raise ValueError("tint names must be unique")
        held_out = set(held_out)
        if not held_out or not held_out < set(names):
            raise ValueError("held_out must name at least one actual tint and leave at least one training tint")
        train = [tint for tint in tints if tint["name"] not in held_out]
        test = [tint for tint in tints if tint["name"] in held_out]
        report["training_names"] = [tint["name"] for tint in train]
        report["held_out_names"] = [tint["name"] for tint in test]

        def source(entry):
            values, interpolation = resample(entry["source_nm"], entry["source_reflectance"])
            if np.any((values <= 0) | (values >= 1)):
                raise ValueError("opaque source-domain fit requires reflectances strictly between 0 and 1")
            return values, interpolation

        def fraction(tint):
            f = float(tint["pigment_mass_fraction"])
            if not np.isfinite(f) or not 0 < f < 1:
                raise ValueError("pigment_mass_fraction must be a finite dry pigment mass fraction strictly between 0 and 1")
            return f

        pure_pigment, p_interpolation = source(pigment)
        qp = (1 - pure_pigment) ** 2 / (2 * pure_pigment)
        fractions = np.array([fraction(tint) for tint in train])[:, None]
        training = [source(tint) for tint in train]
        observed = np.array([values for values, _ in training])
        w_interpolation = None
        if white is None:
            if len(train) != 2 or fractions[0, 0] == fractions[1, 0]:
                raise ValueError(f"unknown gum {'white' if partner_key == 'white' else partner_label} requires exactly two training tints with distinct dry pigment mass fractions")
            qt = (1 - observed) ** 2 / (2 * observed)
            slope = fractions / (1 - fractions) * (qt - qp)
            denominator = slope[0] - slope[1]
            if np.any(np.abs(denominator) < 1e-12):
                raise ValueError(f"unknown gum-{'white' if partner_key == 'white' else partner_label} fit is unidentifiable at one or more wavelengths")
            scatter = (qt[1] - qt[0]) / denominator
            qw = qt[0] + slope[0] * scatter
            invalid = ~np.isfinite(scatter) | ~np.isfinite(qw) | (scatter <= 0) | (qw <= 0)
            if np.any(invalid):
                raise ValueError(f"unknown gum-{'white' if partner_key == 'white' else partner_label} fit has no positive exact opaque-KM solution at {WAVELENGTHS[invalid].tolist()} nm")
            report[f"{partner_key}_evidence"] = (
                "unknown gum white fitted from the two training tints, not an independent white measurement"
                if partner_key == "white" else
                f"unknown gum {partner_label} fitted from the two training mixtures, not an independent partner measurement")
        else:
            pure_white, w_interpolation = source(white)
            qw = (1 - pure_white) ** 2 / (2 * pure_white)
            degenerate = np.abs(qp - qw) < 1e-12
            if np.any(degenerate):
                raise ValueError(f"relative scattering is unidentifiable where pure source K/S coincide: {WAVELENGTHS[degenerate].tolist()} nm")
            report[f"{partner_key}_evidence"] = ("supplied source-domain white measurement" if partner_key == "white"
                                                  else f"supplied source-domain {partner_label} measurement")

        def predict(f, scatter):
            return gen.km((f * scatter * qp + (1 - f) * qw) / (f * scatter + 1 - f))

        def residual(log_s):
            return (predict(fractions, np.exp(log_s)) - observed).ravel()

        def jacobian(log_s):
            scatter = np.exp(log_s)
            denominator = fractions * scatter + 1 - fractions
            q = (fractions * scatter * qp + (1 - fractions) * qw) / denominator
            dq = fractions * scatter * (1 - fractions) * (qp - qw) / denominator ** 2
            derivative = -gen.km(q) / np.sqrt(q * (q + 2)) * dq
            return np.concatenate([np.diag(row) for row in derivative])

        if white is not None:
            fit = least_squares(residual, np.zeros(len(WAVELENGTHS)), jac=jacobian, bounds=(-20, 20),
                                xtol=1e-12, ftol=1e-12, gtol=1e-12, max_nfev=2000)
            if not fit.success or np.any(np.abs(fit.x) > 19.99):
                raise ValueError("relative source scattering fit failed or reached numerical bounds")
            scatter = np.exp(fit.x)
        report.update({
            "status": "evaluated_source_domain",
            "pure_interpolation": {"pigment": p_interpolation, partner_key: w_interpolation},
            "relative_coefficients": {"pigment_k": (qp * scatter).tolist(), "pigment_s": scatter.tolist(),
                                      f"{partner_key}_k": qw.tolist(), f"{partner_key}_s": np.ones_like(qw).tolist()},
            "samples": {},
        })
        for tint in [*train, *test]:
            values, interpolation = source(tint)
            prediction = predict(fraction(tint), scatter)
            diff = prediction - values
            report["samples"][tint["name"]] = {
                "scope": "held_out_source_domain" if tint["name"] in held_out else "in_sample_fit",
                "pigment_mass_fraction": fraction(tint),
                "rms_reflectance": float(np.sqrt(np.mean(diff ** 2))),
                "max_abs_reflectance": float(np.max(np.abs(diff))),
                "residual_reflectance": diff.tolist(),
                "predicted_reflectance": prediction.tolist(),
                "interpolation": interpolation,
            }
        report["training_rms_reflectance"] = float(np.sqrt(np.mean(residual(np.log(scatter)) ** 2)))
        report["held_out_rms_reflectance"] = float(np.sqrt(np.mean([
            np.array(report["samples"][tint["name"]]["residual_reflectance"]) ** 2 for tint in test])))
    except (ValueError, KeyError, TypeError, OverflowError) as exc:
        report["status"] = "failed"
        report["failure"] = str(exc)
    return report
