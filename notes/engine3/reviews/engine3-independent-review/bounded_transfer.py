"""A small reference for conservative, capacity-bounded ONE-WAY exchange.

Not a cloth solver, rag renderer, constitutive law, or drop-in Rust patch.
All quantities use the same volume unit. Requests come from an independently
chosen contact/transfer law. This helper prevents a shared donor or receiver
from being overdrawn by parallel contact requests.

Algorithm: coalesce equal edges, limit by donor availability, then by receiver
spare capacity. Rejected material stays with its donor. There is deliberately
no iterative redistribution: this can underfill receivers and must be assessed
for timestep convergence in a complete simulation. Inputs use the pre-step
state. For rag release/pickup, construct explicit stages from a shared old
state; do not return newly picked-up paint during the same stage.
"""
from __future__ import annotations
from collections import defaultdict
from dataclasses import dataclass
import math
from typing import Sequence

@dataclass(frozen=True)
class Request:
    donor: int
    receiver: int
    volume: float


def _nonnegative_finite(values: Sequence[float], name: str) -> None:
    if any(not math.isfinite(v) or v < 0.0 for v in values):
        raise ValueError(f"{name} must contain finite, nonnegative quantities")


def allocate(available: Sequence[float], spare: Sequence[float],
             requests: Sequence[Request]) -> tuple[Request, ...]:
    """Return bounded edge transfers sorted by (donor, receiver).

    Summed donor outflow <= available; summed receiver inflow <= spare,
    up to floating-point rounding. Output is independent of request ordering
    in this implementation. Does not mutate inputs or throw material away.
    """
    _nonnegative_finite(available, "available")
    _nonnegative_finite(spare, "spare")
    groups: dict[tuple[int, int], list[float]] = defaultdict(list)
    for r in requests:
        if not 0 <= r.donor < len(available) or not 0 <= r.receiver < len(spare):
            raise IndexError("Transfer endpoint outside pool arrays")
        if not math.isfinite(r.volume) or r.volume < 0:
            raise ValueError("Request volume must be finite and nonnegative")
        groups[(r.donor, r.receiver)].append(r.volume)
    edges = [(d, r, math.fsum(groups[(d, r)])) for d, r in sorted(groups)]
    if any(not math.isfinite(v) for _, _, v in edges):
        raise ValueError("Summed requests overflowed")
    donor_demand = [math.fsum(v for d, _, v in edges if d == i) for i in range(len(available))]
    donor_scale = [min(1.0, available[i]/v) if v else 0.0 for i, v in enumerate(donor_demand)]
    first = [(d, r, v * donor_scale[d]) for d, r, v in edges]
    receiver_demand = [math.fsum(v for _, r, v in first if r == i) for i in range(len(spare))]
    receiver_scale = [min(1.0, spare[i]/v) if v else 0.0 for i, v in enumerate(receiver_demand)]
    return tuple(Request(d, r, v * receiver_scale[r]) for d, r, v in first
                 if v * receiver_scale[r] > 0.0)


def move_components(donors: Sequence[Sequence[float]],
                    receivers: Sequence[Sequence[float]],
                    transfers: Sequence[Request]) -> tuple[list[list[float]], list[list[float]]]:
    """Apply volume transfers using each donor's PRE-step component proportions.

    Component axes can represent conserved formulation amounts; color-latent
    coordinates are not themselves material species. No optical mixing is
    implemented. Caller should obtain transfers with allocate(), and include
    locked/immobile material in neither the donor pool nor its available budget.
    """
    rows = list(donors) + list(receivers)
    if not rows:
        return [], []
    count = len(rows[0])
    if count == 0 or any(len(row) != count for row in rows):
        raise ValueError("Pools need a consistent, nonempty component axis")
    for row in rows:
        _nonnegative_finite(row, "components")
    volumes = [math.fsum(row) for row in donors]
    for t in transfers:
        if not 0 <= t.donor < len(donors) or not 0 <= t.receiver < len(receivers):
            raise IndexError("Transfer endpoint outside pool arrays")
        if not math.isfinite(t.volume) or t.volume < 0:
            raise ValueError("Transfer volume must be finite and nonnegative")
    out = [math.fsum(t.volume for t in transfers if t.donor == i) for i in range(len(donors))]
    if any((volumes[i] == 0 and v > 0) or v > volumes[i] + 1e-12 * max(1.0, volumes[i]) for i, v in enumerate(out)):
        raise ValueError("Transfer overdraws donor")
    nd = [[max(0.0, c * (1.0 - out[i] / volumes[i])) if volumes[i] else 0.0
           for c in row] for i, row in enumerate(donors)]
    nr = []
    for j, row in enumerate(receivers):
        nr.append([row[k] + math.fsum(t.volume * donors[t.donor][k] / volumes[t.donor]
                                     for t in transfers if t.receiver == j and t.volume > 0)
                   for k in range(count)])
    return nd, nr
