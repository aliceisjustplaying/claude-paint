#!/usr/bin/env python3
import argparse
import csv
import json
from pathlib import Path

import numpy as np

from common import ROI, scratch, sha256, write_json

FIELDS = ("dry_height_um", "wet_paint_um", "solvent_um", "visible_surface_um", "color_y", "gallery_y")
FREQUENCIES = (0.1066667, 0.2133333)
SPOT_HALF_WIDTH = 0.009


def read_grid(path, roi=ROI):
    x0, y0, x1, y1 = roi
    shape = (y1 - y0, x1 - x0)
    seen = np.zeros(shape, dtype=bool)
    arrays = {name: np.empty(shape) for name in FIELDS}
    with Path(path).open(newline="") as stream:
        for row in csv.DictReader(stream):
            x, y = int(row["x_px"]), int(row["y_px"])
            if not x0 <= x < x1 or not y0 <= y < y1:
                raise ValueError(f"CSV pixel outside requested native ROI: {(x, y)}")
            index = (y - y0, x - x0)
            if seen[index]:
                raise ValueError(f"Duplicate CSV pixel: {(x, y)}")
            seen[index] = True
            for name in FIELDS:
                value = float(row[name])
                if not np.isfinite(value):
                    raise ValueError(f"Nonfinite {name} at {(x, y)}")
                arrays[name][index] = value
    if not seen.all():
        raise ValueError("Surface CSV does not cover every native ROI pixel exactly once")
    return arrays


def detrend(array):
    y, x = np.indices(array.shape, dtype=float)
    x -= x.mean()
    y -= y.mean()
    return array - array.mean() - x * np.sum(array * x) / np.sum(x * x) - y * np.sum(array * y) / np.sum(y * y)


def correlation(a, b):
    a, b = a.ravel() - a.mean(), b.ravel() - b.mean()
    denominator = np.linalg.norm(a) * np.linalg.norm(b)
    return float(np.dot(a, b) / denominator) if denominator > 0 else None


def band_rms(array, bandwidth):
    residual = detrend(array)
    window = np.outer(np.hanning(array.shape[0]), np.hanning(array.shape[1]))
    spectrum = np.fft.fft2(residual * window)
    fy = np.abs(np.fft.fftfreq(array.shape[0]))[:, None]
    fx = np.abs(np.fft.fftfreq(array.shape[1]))[None, :]
    union = np.zeros(array.shape, dtype=bool)
    result = {}
    for frequency in FREQUENCIES:
        mask = (np.abs(fx - frequency) <= bandwidth) | (np.abs(fy - frequency) <= bandwidth)
        union |= mask
        power = np.sum(np.abs(spectrum[mask]) ** 2) / (array.size ** 2 * np.mean(window ** 2))
        result[f"{frequency:.7f}"] = float(np.sqrt(power))
    result["combined"] = float(np.sqrt(np.sum(np.abs(spectrum[union]) ** 2) / (array.size ** 2 * np.mean(window ** 2))))
    return result


def lattice_projection(array):
    fy = np.abs(np.fft.fftfreq(array.shape[0]))[:, None]
    fx = np.abs(np.fft.fftfreq(array.shape[1]))[None, :]
    checker, thread = FREQUENCIES
    mask = ((np.abs(fx - checker) < SPOT_HALF_WIDTH) & (np.abs(fy - checker) < SPOT_HALF_WIDTH))
    mask |= ((np.abs(fx - thread) < SPOT_HALF_WIDTH) & (fy < SPOT_HALF_WIDTH))
    mask |= ((np.abs(fy - thread) < SPOT_HALF_WIDTH) & (fx < SPOT_HALF_WIDTH))
    return np.fft.ifft2(np.fft.fft2(array - array.mean()) * mask).real


def summarize(arrays, bandwidth):
    support = arrays["dry_height_um"]
    lattice_support = lattice_projection(support)
    fields = {}
    for name, array in arrays.items():
        lattice = lattice_projection(array)
        fields[name] = {"units": "linear Y" if name.endswith("_y") else "um", "mean": float(array.mean()),
                        "rms_about_mean": float(array.std()), "p05": float(np.quantile(array, 0.05)),
                        "median": float(np.median(array)), "p95": float(np.quantile(array, 0.95)),
                        "grid_band_rms": band_rms(array, bandwidth),
                        "woven_lattice_spot_rms": float(np.sqrt(np.mean(lattice ** 2))),
                        "woven_lattice_spot_dry_support_pearson": correlation(lattice_support, lattice),
                        "dry_support_pearson": correlation(support, array),
                        "plane_removed_dry_support_pearson": correlation(detrend(support), detrend(array))}
    return fields


def analyze(surface, geometry, bandwidth=0.012):
    if not 0 < bandwidth < FREQUENCIES[0] / 2:
        raise ValueError("Invalid grid-band half-width")
    original_digest = sha256(surface)
    relight_digest = sha256(geometry) if geometry else None
    original = read_grid(surface)
    relight = read_grid(geometry) if geometry else None
    if sha256(surface) != original_digest or (geometry and sha256(geometry) != relight_digest):
        raise ValueError("CSV changed while it was being read; no diagnostic result retained")
    result = {"analysis_kind": "Diagnostic numerical analysis of actual surface CSVs; no image filtering or pixel edits",
              "roi_native_pixels_half_open": list(ROI), "native_mm_per_pixel": 160 / 1200,
              "linen_threads_per_cm": 16, "alternating_weave_repeat": 2,
              "frequencies_cycles_per_pixel": list(FREQUENCIES), "band_half_width_cycles_per_pixel": bandwidth,
              "method": "Subtract least-squares plane, apply separable Hann window to numerical fields, sum FFT power in the union of x/y grid-frequency bands. RMS uses mean-window-squared energy correction; overlapping bands count once. Pearson correlations are raw and plane-removed, constant fields yield null.",
              "woven_lattice_spot_method": {
                  "analysis_kind": "Narrow diagnostic projection of numerical CSV fields only; never image filtering",
                  "checkerboard_centers_absolute_fx_fy": [FREQUENCIES[0], FREQUENCIES[0]],
                  "full_thread_centers_absolute_fx_fy": [[FREQUENCIES[1], 0], [0, FREQUENCIES[1]]],
                  "spot_half_width_cycles_per_pixel": SPOT_HALF_WIDTH,
                  "method": "Subtract field mean, take unwindowed FFT with no plane removal, retain the union of rectangular checkerboard and full-thread spots including all conjugate signs, then inverse FFT to a real numerical field. RMS is sqrt(mean(projected_field squared)). Support correlation is Pearson between the equally spot-projected dry support and measured field. Raw ROI correlation is a separate metric; constant projected fields yield null.",
                  "interpretation": "A high projected support correlation shows woven-frequency alignment with dry support, not raw-ROI correlation or proof of causation."},
              "interpretation": "Band RMS measures periodic field energy near the linen weave, not a causal attribution. Support correlation can reflect real relief. Gallery/color are exported linear Y, not PNG gamma values.",
              "datasets": {"lean4": {"csv": str(surface), "csv_sha256": original_digest, "fields": summarize(original, bandwidth)}}}
    if relight is not None:
        equal = {name: np.array_equal(original[name], relight[name]) for name in ("dry_height_um", "wet_paint_um", "solvent_um", "color_y")}
        if not all(equal.values()):
            raise ValueError(f"Geometry-only CSV differs in shared Canvas material/color fields: {equal}")
        result["geometry_only_provenance"] = {"kind": "Engine-18 geometry relight of the SAME saved lean4 Canvas; not a brush replay",
                                              "canvas": str(surface.parent / "final.ckpt"),
                                              "canvas_sha256": sha256(surface.parent / "final.ckpt"),
                                              "shared_canvas_fields_exactly_equal": equal,
                                              "provenance_source": "Captain-supplied control-lean4/surface.csv and geometry-only.csv"}
        result["datasets"]["geometry_only"] = {"csv": str(geometry), "csv_sha256": relight_digest, "fields": summarize(relight, bandwidth)}
        result["relight_changes"] = {name: {"difference_rms": float(np.sqrt(np.mean((relight[name] - original[name]) ** 2))),
                                            "difference_grid_band_rms": band_rms(relight[name] - original[name], bandwidth)}
                                     for name in ("visible_surface_um", "gallery_y")}
    return result


def main():
    parser = argparse.ArgumentParser(description="Native CSV mesh diagnostics; never filters images")
    parser.add_argument("surface", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--geometry-only", type=Path)
    parser.add_argument("--band-half-width", type=float, default=0.012)
    args = parser.parse_args()
    output = scratch(args.output)
    if output.exists():
        raise ValueError("Diagnostic output already exists; choose a new path")
    output.parent.mkdir(parents=True, exist_ok=True)
    result = analyze(args.surface.resolve(), args.geometry_only.resolve() if args.geometry_only else None, args.band_half_width)
    write_json(output, result)
    print(json.dumps({name: {field: data["grid_band_rms"]["combined"] for field, data in dataset["fields"].items()}
                      for name, dataset in result["datasets"].items()}, indent=2))


if __name__ == "__main__":
    main()
