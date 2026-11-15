"""The arithmetic behind notes/research/sanded_paper.md §1 and §3.

Run: uv run --no-project --with numpy python -I notes/research/sanded_paper_coat.py

1. A gritted coat's tooth: grains of FEPA's lognormal spread, as spheres at
   random on the sheet, a binder laid level with the median grain's top that
   dries to its solids, and the stick resting where the tallest grains carry
   its real contact. The tooth is the open volume between the binder and that
   level (paper::Grit::tooth_um does the same sum).
2. The stick's wear on grit against fiber: Rabinowicz's groove area over half
   the bearing area, for a crushed grain's corner (its slope from the measured
   angularity SPQ) and for a fiber crossing.
3. The coat's weight per area, against Colourfix's 40 g/m² (300 g/m² paper
   printed to 340 g/m²), for the binder's wet level.
"""
from math import pi, sqrt
from statistics import NormalDist

import numpy as np

SIG = 0.23  # sd of ln d (FEPA P400/P500 ds94 and ds3 limits)
FACE = 0.01  # the stick's real contact share F/(σp·A0)
ACRYLIC, GLUE = 0.45, 0.07  # binder solids by volume
PAPER_MU = 6.0  # the drawing paper's mean pore depth, µm (Paper::drawing)

zk = np.linspace(-4, 4, 161)
wk = np.exp(-zk**2 / 2)
wk /= wk.sum()


def coat(d50, cover=0.6, solids=ACRYLIC, face=FACE, wet=1.0, sig=SIG):
    """(tooth µm, binder level / d50, face / d50, engaged grains /mm², their mean depth µm)"""
    d = d50 * np.exp(sig * zk - 3 * sig * sig)  # by number: mass median less 3σ²
    r = d / 2
    n = cover / np.sum(wk * pi * r * r)
    zs = np.linspace(0, d.max(), 2000)
    dz = zs[1] - zs[0]
    mr = n * pi * np.sum(wk[None, :] * np.clip(r[None, :] ** 2 - (zs[:, None] - r[None, :]) ** 2, 0, None), axis=1)
    below = np.cumsum(mr) * dz
    level = wet * d50
    binder = solids * (level - np.interp(level, zs, below))
    b = np.interp(binder, zs - below, zs)
    top = zs[np.nonzero(mr >= face)[0].max()]
    sel = (zs > b) & (zs < top)
    eng = d > top
    p = np.sum(wk * eng)
    depth = np.sum(wk * eng * (d - top)) / p
    return np.sum(1 - mr[sel]) * dz, b / d50, top / d50, n * p * 1e6, depth


def coat_weight(d50, wet=1.0, cover=0.6, solids=ACRYLIC, rho_grain=2.65, rho_binder=1.1):
    """The dry coat's weight, g/m² (1 µm at 1 g/cm³ is 1 g/m²): its grains and its dry binder."""
    d = d50 * np.exp(SIG * zk - 3 * SIG * SIG)
    r = d / 2
    n = cover / np.sum(wk * pi * r * r)
    zs = np.linspace(0, d.max(), 2000)
    dz = zs[1] - zs[0]
    mr = n * pi * np.sum(wk[None, :] * np.clip(r[None, :] ** 2 - (zs[:, None] - r[None, :]) ** 2, 0, None), axis=1)
    below = np.cumsum(mr) * dz
    level = wet * d50
    return below[-1] * rho_grain + solids * (level - np.interp(level, zs, below)) * rho_binder


def spq_k(spq):
    """Rabinowicz's K for a corner of angularity SPQ = cos(θ/2): it attacks at 90° − θ/2."""
    return (2 / pi) * np.tan(pi / 2 - np.arccos(spq))


def cube_corner_k(trials=5000, seed=2):
    """Rabinowicz's K (groove area / half the bearing area) for a cube's top corner at random orientation."""
    rng = np.random.default_rng(seed)
    faces = np.array([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]], float)
    ks = []
    for _ in range(trials):
        q = rng.standard_normal(4)
        q /= np.linalg.norm(q)
        a, b, c, e = q
        rot = np.array([[a*a+b*b-c*c-e*e, 2*(b*c-a*e), 2*(b*e+a*c)],
                        [2*(b*c+a*e), a*a-b*b+c*c-e*e, 2*(c*e-a*b)],
                        [2*(b*e-a*c), 2*(c*e+a*b), a*a-b*b-c*c+e*e]])
        up = (faces @ rot.T)
        up = up[up[:, 2] > 0]
        # the cross-section 1 below the corner: n·(x, y, −1) ≤ 0
        pts = []
        for i in range(len(up)):
            for j in range(i + 1, len(up)):
                p = np.linalg.solve(up[[i, j], :2], up[[i, j], 2])
                if np.all(up[:, :2] @ p <= up[:, 2] + 1e-9):
                    pts.append(p)
        pts = np.array(pts)
        cen = pts.mean(0)
        pts = pts[np.argsort(np.arctan2(pts[:, 1] - cen[1], pts[:, 0] - cen[0]))]
        x, y = pts[:, 0], pts[:, 1]
        area = 0.5 * abs(np.dot(x, np.roll(y, 1)) - np.dot(y, np.roll(x, 1)))
        th = rng.uniform(0, pi)
        perp = np.array([-np.sin(th), np.cos(th)])
        width = (pts @ perp).max() - (pts @ perp).min()
        ks.append(width / area)  # (width·1/2) / (area/2)
    return float(np.mean(ks))


def fiber_k(rq, r2, face=FACE, fiber_um=25.0, porosity=0.5):
    """K on the fiber crossings a stick rests on (Greenwood–Williamson, plastic)."""
    nd = NormalDist()
    r1 = fiber_um / 2
    per_layer = np.log(1 / porosity)  # a layer's coverage, ln(1/ε)
    eta = (2 / pi) * (per_layer / fiber_um) ** 2  # crossings per µm²
    need = face / (2 * pi * sqrt(r1 * r2) * eta)  # E[(h − d)+]
    lo, hi = -3.0, 6.0
    for _ in range(60):
        t = (lo + hi) / 2
        lo, hi = (t, hi) if rq * (nd.pdf(t) - t * (1 - nd.cdf(t))) > need else (lo, t)
    p = 1 - nd.cdf(t)
    depth = need / p
    # a ridge long along the fiber, at random to the stroke: (8/3π)(2/π)·δ/a
    return (8 / (3 * pi)) * (2 / pi) * sqrt(depth / (2 * r1)), eta * 1e6, p * eta * 1e6, depth


if __name__ == "__main__":
    print("grade  tooth acrylic, glue (µm)  × paper   binder/d  face/d  engaged/mm²  depth µm")
    for name, d in [("P240", 58.5), ("P320", 46.2), ("P400", 35.0), ("P500", 30.2), ("P600", 25.8), ("P800", 21.8)]:
        a, g = coat(d), coat(d, solids=GLUE)
        print(f"{name}   {a[0]:5.1f} {g[0]:5.1f}   {a[0]/PAPER_MU:.1f}–{g[0]/PAPER_MU:.1f}   {a[1]:.2f}/{g[1]:.2f}   {a[2]:.2f}   {a[3]:4.0f}   {a[4]:.1f}")
    print("P400 acrylic, one input at a time:")
    for label, kw in [("face 1%", dict(face=0.01)), ("face 5%", dict(face=0.05)), ("solids 0.35", dict(solids=0.35)),
                      ("solids 0.55", dict(solids=0.55)), ("wet 1.3 d50", dict(wet=1.3)), ("spread 0.20", dict(sig=0.20)),
                      ("spread 0.27", dict(sig=0.27)), ("cover 0.3", dict(cover=0.3)), ("cover 0.85", dict(cover=0.85)),
                      ("cover 1", dict(cover=1.0))]:
        print(f"  {label:12s} {coat(35.0, **kw)[0]:.1f} µm")
    kc = cube_corner_k()
    k30 = (2 / pi) * np.tan(np.radians(30))
    print(f"grain corner K: cube {kc:.2f}, 30° slopes {k30:.2f}, sphere at P400 {(8/(3*pi))*sqrt(coat(35.0)[4]/(1.27*35.0)):.2f}")
    for rq in (3.0, 4.0, 5.0):
        for r2 in (100.0, 300.0):
            kf, eta, eng, depth = fiber_k(rq, r2)
            print(f"paper Rq {rq} R_along {r2:.0f}: {eta:.0f} crossings/mm², {eng:.0f} carry, {depth:.2f} µm deep, K {kf:.3f}; grit/fiber {k30/kf:.1f}–{kc/kf:.1f}")
    kf = [fiber_k(rq, r2)[0] for rq in (3.0, 4.0, 5.0) for r2 in (100.0, 300.0)]
    mid = (min(kf) + max(kf)) / 2
    print(f"fiber K {min(kf):.3f}–{max(kf):.3f}, middle {mid:.3f}")
    for spq in (0.50, 0.55, 0.60):
        k = spq_k(spq)
        print(f"SPQ {spq:.2f}: K {k:.3f}, grit/fiber {k/max(kf):.2f}–{k/min(kf):.2f}, at the middle {k/mid:.2f}")
    print("coat weight, g/m² (Colourfix: 40):")
    for name, d in [("P400", 35.0), ("P500", 30.2), ("P600", 25.8)]:
        for rho, grit in ((2.65, "quartz"), (3.95, "alumina")):
            print(f"  {name} {grit:7s} wet level d50 {coat_weight(d, 1.0, rho_grain=rho):.1f}, 1.3 d50 {coat_weight(d, 1.3, rho_grain=rho):.1f}")
