#!/usr/bin/env python3
"""Independent scalar/source-structure probes for the October 4 Engine 3 packet.

Python 3.10+, standard library only. These do NOT run or render the Rust engine.
Frozen coefficients deliberately isolate mechanisms; outputs are not predictions
of full strokes. See REVIEW.md for scope and source locations.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import math
from pathlib import Path
from typing import Iterable


def remaining_after_stamps(k: float, contact: float, stamps: int,
                           exposure: float = 1.0) -> float:
    """Frozen local model, full access, no floor, capacity loss or redeposition."""
    if not all(math.isfinite(x) for x in (k, contact, exposure)):
        raise ValueError("Parameters must be finite")
    if k < 0 or exposure < 0 or not 0 <= contact <= 1 or stamps < 1:
        raise ValueError("Invalid rate, contact, exposure, or stamp count")
    fraction = contact * -math.expm1(-k * exposure / stamps)
    return (1.0 - fraction) ** stamps


def ideal_path_counts(segment_lengths_mm: Iterable[float],
                      stamp_mm: float = .5, mechanics_mm: float = .25) -> dict:
    """Mirror the two nested ceil rules in exact intended lengths, not f32 ULPs."""
    if stamp_mm <= 0 or mechanics_mm <= 0:
        raise ValueError("Step lengths must be positive")
    stamps = mechanical_steps = 0
    for length in segment_lengths_mm:
        if not math.isfinite(length) or length < 0:
            raise ValueError("Segment lengths must be finite and nonnegative")
        if length == 0:
            continue
        # Small tolerance avoids counting a spurious step at exact decimal ratios.
        n = max(1, math.ceil(length / stamp_mm - 1e-12))
        substeps = max(1, math.ceil((length / n) / mechanics_mm - 1e-12))
        stamps += n
        mechanical_steps += n * substeps
    return {"paint_stamps": stamps, "mechanical_steps": mechanical_steps}


def effective_flow_minutes(pixels: int, phi: float, dt: float = 1.0,
                           width_mm: float = 440.0) -> dict:
    """thinner.rs's CFL limiter at frozen post-evaporation max mobility, fluid=1."""
    if pixels < 1 or width_mm <= 0 or dt <= 0 or not 0 < phi <= .95:
        raise ValueError("Invalid resolution, physical size, dt or solvent fraction")
    dx = width_mm / pixels
    mobility = .06 * phi / (1.0 - phi)
    required = max(1, math.ceil(mobility * dt / (dx * dx) / .2))
    n = min(required, 64)
    k = min(dt / n / (dx * dx), .2 / mobility)
    effective = n * k * dx * dx
    return {"pixels": pixels, "post_evaporation_phi": phi,
            "mobility_mm2_min": mobility, "requested_minutes": dt,
            "required_substeps": required, "actual_substeps": n,
            "effective_flow_minutes": effective,
            "fraction_of_requested_flow_time": effective / dt}


def scheduled_flow_minutes(start_min: float, elapsed_min: float) -> int:
    """Number of full-minute spread(1.0) calls while solvent remains present."""
    if start_min < 0 or elapsed_min < 0:
        raise ValueError("Times must be nonnegative")
    return math.floor(start_min + elapsed_min) - math.floor(start_min)


def film_cap_um(thinner_share: float) -> dict:
    if not 0 < thinner_share <= .95:
        raise ValueError("This probe expects 0 < thinner share <= .95")
    liquid = 6.0 * (1.0 - thinner_share) / thinner_share
    return {"thinner_share": thinner_share, "liquid_cap_um": liquid,
            "nonvolatile_cap_if_same_composition_um": liquid * (1.0 - thinner_share)}


def results(root: Path | None = None) -> dict:
    k = 4.0 * (.7 + .6 * .8) * (1.0 + 8.0 * .5)
    out = {
        "scope": "Independent Python math/source-structure probes; NOT Rust execution or physical validation",
        "pickup": {"k": k, "total_normalized_exposure": 1.0,
                   "assumptions": "clean face, fresh fluid paint, full reach, fixed contact, no floor or redeposition",
                   "cases": [{"contact": c,
                              "remaining_by_stamps": {str(n): remaining_after_stamps(k, c, n)
                                                      for n in (1, 22, 88, 176, 10000)},
                              "continuous_limit": math.exp(-k*c)}
                             for c in (.05, .2, .5, 1.0)]},
        "path_partition": {"one_176mm_segment": ideal_path_counts([176.0]),
                           "1760_segments_of_point1mm": ideal_path_counts([.1]*1760)},
        "flow_cfl": [effective_flow_minutes(n, p) for n in (480, 2400, 4800)
                     for p in (.5, .9, .95)],
        "clock_phase": [{"start_min": s, "actual_elapsed_min": .02,
                         "scheduled_flow_minutes": scheduled_flow_minutes(s, .02)}
                        for s in (.1, .99)],
        "prior_mobile_paint_absorbed_by_one_blot": 1.0 - (1.0-.35)**(4.0*.8),
        "weighted_capacity_counterexample": {
            "capacity_each": 1.0, "held_before": [1.0, 0.0],
            "weights": [.5, .5], "weighted_thirst": .5,
            "assumed_total_pickup": .2, "held_after_without_receiver_budget": [1.1, .1],
            "note": "Any positive weighted pickup overfills the already full cell; .2 is illustrative"},
        "film_caps": [film_cap_um(t) for t in (.1, .25, .5, .75, .9, .95)],
        "ideal_dark_absorber_counterexample": {
            "uncoated_black_Y": 0.0, "uncoated_white_Y": 1.0,
            "coated_black_Y": 0.0, "coated_white_Y": .001,
            "contrast_ratio": 0.0, "absolute_difference_retained": .001,
            "note": "Ratio alone does not quantify surviving absolute substrate modulation"},
    }
    if root is not None:
        snapshots = ("shipping", "prototype", "research", "wip/thinner", "wip/look", "wip/engine3-integration")
        out["pigment_source_sha256"] = {}
        for snapshot in snapshots:
            p = root / snapshot / "crates/paint/src/pigment.rs"
            out["pigment_source_sha256"][snapshot] = hashlib.sha256(p.read_bytes()).hexdigest()
    return out


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, help="Optional extracted engine3-rag-review directory")
    parser.add_argument("--output", type=Path, help="Write JSON here instead of stdout")
    args = parser.parse_args()
    text = json.dumps(results(args.root), indent=2, allow_nan=False) + "\n"
    if args.output:
        args.output.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
