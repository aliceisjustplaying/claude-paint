"""Isolated equations from the uploaded Engine 3 snapshots, not a Rust rerun.

Python float arithmetic (normally binary64) is used deliberately. This does not
reproduce Rust f32 roundoff, brush deposition, cloth mechanics or full rendering.
No third-party packages are required. See SOURCE_RECEIPTS.md for source ranges.
"""
from __future__ import annotations
import json
import math
from typing import Sequence

COAT_UM = 25.0
STROKE_FILM_UM = 6.0
WET_FILM_UM = 2.0
STAIN_COATS = 0.04
SPREAD_MM2_MIN = 0.06
MAX_SUBSTEPS = 64
RGB = tuple[float, float, float]


def require_finite_nonnegative(value: float, name: str) -> float:
    if not math.isfinite(value) or value < 0.0:
        raise ValueError(f'{name} must be finite and nonnegative')
    return value


def stroke_limit_um(thinner: float) -> float:
    """The source's total-liquid ceiling, not an observed deposited thickness."""
    if math.isnan(thinner) or thinner <= 0.0:
        return math.inf
    t = min(thinner, 0.95)
    return STROKE_FILM_UM * (1.0 - t) / t


def nominal_paint_ceiling_um(thinner: float) -> float:
    """Assumes unchanged composition, bare baseline and no extra lateral inflow."""
    t = min(max(require_finite_nonnegative(thinner, 'thinner'), 0.0), 0.95)
    return stroke_limit_um(t) * (1.0 - t)


def minimum_rag_floor_um() -> float:
    return STAIN_COATS * COAT_UM


def most_liftable_um(paint_um: float) -> float:
    """Optimistic full contact/access/rate. No redeposition, no cure limitation."""
    return max(0.0, require_finite_nonnegative(paint_um, 'paint_um') - minimum_rag_floor_um())


def flow_available_um(total_liquid_um: float) -> float:
    return max(0.0, require_finite_nonnegative(total_liquid_um, 'total_liquid_um') - WET_FILM_UM)


def mobility(phi: float) -> float:
    if not math.isfinite(phi) or not 0.0 <= phi <= 1.0:
        raise ValueError('phi must lie in [0, 1]')
    p = min(phi, 0.95)
    return SPREAD_MM2_MIN * p / (1.0 - p)


def flow_step_summary(max_mobility: float, dx_mm: float, dt_min: float) -> dict[str, float | int]:
    require_finite_nonnegative(max_mobility, 'max_mobility')
    if not math.isfinite(dx_mm) or dx_mm <= 0.0:
        raise ValueError('dx_mm must be positive and finite')
    require_finite_nonnegative(dt_min, 'dt_min')
    if max_mobility == 0.0 or dt_min == 0.0:
        return {'required_substeps': 0, 'used_substeps': 0, 'effective_dt_min': 0.0}
    ratio = max_mobility * dt_min / (dx_mm * dx_mm)
    required = max(1, math.ceil(ratio / 0.2))
    n = min(required, MAX_SUBSTEPS)
    k = min(dt_min / n / (dx_mm * dx_mm), 0.2 / max(max_mobility, 1e-12))
    return {'required_substeps': required, 'used_substeps': n,
            'effective_dt_min': n * k * dx_mm * dx_mm}


def luminance(c: Sequence[float]) -> float:
    return sum(w * v for w, v in zip((0.2126, 0.7152, 0.0722), c))


def rgb_hex(s: str) -> RGB:
    h = s.removeprefix('#')
    if len(h) != 6:
        raise ValueError('expected six hexadecimal digits')
    def linear(c: float) -> float:
        return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4
    vals = tuple(linear(int(h[i:i+2], 16) / 255.0) for i in (0, 2, 4))
    return vals  # type: ignore[return-value]


def ks_of(r: float) -> float:
    r = min(max(r, 0.002), 0.995)
    return (1.0 - r) ** 2 / (2.0 * r)


def layer1(k: float, s: float, coats: float) -> tuple[float, float]:
    if s < 1e-6:
        return 0.0, math.exp(-k * coats)
    a = 1.0 + k / s
    b = math.sqrt(max(0.0, a*a - 1.0))
    if b < 1e-3:
        sx = s * coats
        return sx / (1.0 + a*sx), 1.0 / (1.0 + a*sx)
    z = min(b * s * coats, 40.0)
    sh, ch = math.sinh(z), math.cosh(z)
    denom = a*sh + b*ch
    return sh / denom, b / denom


def over_channel(r_infinite: float, s: float, substrate: float, coats: float) -> float:
    if coats <= 0.0:
        return substrate
    r, trans = layer1(s * ks_of(r_infinite), s, coats)
    return r + trans * trans * substrate / max(1e-6, 1.0 - r * substrate)


def hiding_of(r: float, s: float) -> float:
    b = over_channel(r, s, 0.0, 1.0)
    w = over_channel(r, s, 1.0, 1.0)
    return min(max(b / max(w, 1e-6), 0.0), 1.0)


def scatter_for(r: float, hiding: float) -> float:
    h = min(max(hiding, 1e-4), 0.9995)
    lo, hi = -9.0, 9.0
    for _ in range(40):
        mid = 0.5 * (lo + hi)
        if hiding_of(r, math.exp(mid)) < h:
            lo = mid
        else:
            hi = mid
    return math.exp(0.5 * (lo + hi))


def sienna_measurement(name: str, paint_um: float, white: float = 0.8) -> dict[str, float | str]:
    specifications = {'raw': ('#9a6a2b', 0.40), 'burnt': ('#7c3f24', 0.45)}
    color_hex, scalar_hiding = specifications[name]
    color = rgb_hex(color_hex)
    s = scatter_for(luminance(color), scalar_hiding)
    coats = require_finite_nonnegative(paint_um, 'paint_um') / COAT_UM
    black_y = luminance([over_channel(c, s, 0.0, coats) for c in color])
    white_y = luminance([over_channel(c, s, white, coats) for c in color])
    return {'pigment': name, 'paint_um': paint_um, 'masstone_Y': luminance(color),
            'input_scalar_hiding': scalar_hiding, 'scatter_per_coat': s,
            'actual_rgb_contrast_ratio': black_y / white_y,
            'absolute_difference_retained': (white_y - black_y) / white}


def results() -> dict:
    rows = []
    for t in (0.5, 2.0/3.0, 0.75, 0.9, 0.95):
        liquid = stroke_limit_um(t)
        paint = nominal_paint_ceiling_um(t)
        rows.append({'thinner': t, 'nominal_total_liquid_um': liquid,
                     'nominal_nonvolatile_paint_um': paint,
                     'most_rag_liftable_um': most_liftable_um(paint),
                     'flow_available_um': flow_available_um(liquid)})
    dx = 440.0 / 2400.0
    return {
        'scope': 'Equation-level Python calculations, not engine executions or physical measurements.',
        'thresholds': {'nominal_wash_reaches_minimum_rag_floor': 2.0/3.0,
                       'nominal_wash_reaches_flow_floor': 0.75},
        'wash_table': rows,
        'mixed_mobility_counterexample': {
            'assumptions': 'Frozen, post-evaporation coefficients; direct spread(dt=1); 440mm/2400px. An extra phi=.95 pixel below the wet-film floor cannot donate, but source includes it in m_max. No rendered transport is simulated.',
            'active_phi_0p5_alone': flow_step_summary(mobility(0.5), dx, 1.0),
            'with_inactive_phi_0p95_in_same_dirty_region': flow_step_summary(mobility(0.95), dx, 1.0),
        },
        'double_throttle_example': [{'uniform_load': u, 'one_thirst_factor': 1-u*u,
                                    'two_identical_thirst_factors': (1-u*u)**2} for u in (0.0,0.5,0.8,0.95)],
        'sienna_same_synthetic_substrates': [sienna_measurement(name, um)
                                            for um in (3.0, 10.0, 25.0, 30.0)
                                            for name in ('raw', 'burnt')],
        'reviewer_reported_arithmetic_only': {
            'shipping_dry_film_difference_percentage_points': 31.2 - 23.8,
            'shipping_damp_film_difference_percentage_points': 6.8 - 3.9,
            'gross_redeposit_divided_by_remaining_film_NOT_provenance': 6.19 / 8.62,
            'stale_card_current_over_recorded_thickness': 5.36 / 1.64,
        }
    }


if __name__ == '__main__':
    print(json.dumps(results(), indent=2, allow_nan=False))
