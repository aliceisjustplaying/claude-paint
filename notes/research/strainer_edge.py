#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy", "scipy"]
# ///
# The sheet beside a strainer bar (notes/research/cassatt_papers.md §1,
# "The bar's edge"), for the blue wove (crates/paint/src/paper.rs, `edge_hold`,
# `Paper::give_at`): uv run notes/research/strainer_edge.py [--2d]
#
# 1. #319's mirror formula against a direct sum over the patch: it holds for a
#    patch clear of the bar (d >= a) and not nearer, where it goes negative
#    (the give jumps to the bar's 2.7 mm in, then drops within 0.3 mm).
# 2. The 1D plate pinned at the bar's edge, pressed by a flat facet through
#    the sheet's and cloth's give k: D w'''' - T w'' = k (delta - w), solved by
#    finite differences, against the closed form edge_hold uses.
# 3. (--2d) The full contact of a rigid flat facet straddling the bar, on the
#    pinned half-plane's own Green's function (membrane and bending): the
#    pressure the bay keeps falls off as the 1D solution does.
import sys
import numpy as np
from scipy.special import i0, i1, k0, k1
import scipy.sparse as sp
import scipy.sparse.linalg as sl

# the blue wove on its strainer (Drum and Paper::mounted, the engine's numbers)
T = 0.13                 # N/mm, CANVAS_TENSION_N_PER_MM
ELL = 5.2521524          # mm, sqrt(D/T)
D = ELL * ELL * T        # N mm
A = 6.855265             # mm, ref_patch_mm
K = 0.5521879            # N/mm^3: z_mpa / caliper, 5.52e-4 MPa/um
B = A / ELL

def free(b):
    return 0.5 - 2 / b * k1(b) * (i0(b) - 1)

# --- 1. the mirror formula near the bar -------------------------------------
def mirror_formula(d):
    return free(B) - 2 / B * i1(B) * (i0(B) - 1) * k0(2 * d / ELL)

def direct_sink(d, n=120):
    """The patch's middle below its rim (units F/2piT), the patch's load on the
    pinned half-plane x > 0 by its image, the part over the bar carried by it."""
    rr = (np.arange(n) + 0.5) / n * A
    tt = (np.arange(2 * n) + 0.5) / (2 * n) * 2 * np.pi
    R, TH = np.meshgrid(rr, tt)
    wq = (R * (A / n) * (np.pi / n)).ravel() / (np.pi * A * A)
    qx, qy = (d + R * np.cos(TH)).ravel(), (R * np.sin(TH)).ravel()
    on = qx > 0
    G = lambda r: -(np.log(r) + k0(r / ELL))
    def w(px, py):
        if px <= 0:
            return 0.0
        d1 = np.maximum(np.hypot(px - qx, py - qy), 1e-9)
        d2 = np.maximum(np.hypot(px + qx, py - qy), 1e-9)
        return np.sum(wq[on] * (G(d1[on]) - G(d2[on])))
    th = (np.arange(64) + 0.5) / 64 * 2 * np.pi
    return w(d, 0.0) - np.mean([w(d + A * np.cos(t), A * np.sin(t)) for t in th])

print(f"1. the patch's sink near the bar (F/2piT); free bay {free(B):.4f}")
for d in [0.5, 1, 2, 2.5, 3, 4, 5, 6, A, 8, 10, 20]:
    print(f"   {d:5.2f} mm in: mirror formula {mirror_formula(d):+.4f}  direct {direct_sink(d):+.4f}")

# --- 2. the plate pinned at the bar's edge, on the facet's foundation -------
def edge_hold(s, d=D, t=T, k=K):
    psi = np.arccos(t / (2 * np.sqrt(d * k)))
    l = (d / k) ** 0.25
    al, be = np.cos(psi / 2) / l, np.sin(psi / 2) / l
    return np.where(be * s >= np.pi - psi, 0.0, np.exp(-al * s) * np.sin(psi + be * s) / np.sin(psi))

h, L = 0.02, 440.0
n = int(L / h) - 1
x = (np.arange(n) + 1) * h
e = np.ones(n)
D2 = sp.diags([e[1:], -2 * e, e[1:]], [-1, 0, 1]) / h**2   # w = w'' = 0 at both ends
Kp = D * (D2 @ D2) - T * D2 + K * sp.identity(n)
w = sl.spsolve(Kp.tocsc(), K * e)                           # delta = 1
phi = 1 - w
print(f"2. the share of the bar's pressure kept s mm in; (D/k)^1/4 = {(D / K) ** 0.25:.3f} mm")
for s in [0.1, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0]:
    j = int(round(s / h)) - 1
    print(f"   s {s:4.1f}: finite differences {phi[j]:+.3f}  closed form {float(edge_hold(s)):+.3f}")
psi = np.arccos(T / (2 * np.sqrt(D * K)))
print(f"   first zero {(np.pi - psi) / (np.sin(psi / 2) / (D / K) ** 0.25):.3f} mm; steepest fall {np.min(np.diff(phi[:400])) / h:.3f} /mm")

# --- 3. the facet straddling the bar, in 2D ---------------------------------
if "--2d" in sys.argv:
    g = 0.27
    for xc in [0.0, 2.0]:
        X, Y = np.meshgrid(np.arange(-A, A + 1e-9, g) + xc, np.arange(-A, A + 1e-9, g))
        m = (X - xc) ** 2 + Y**2 <= A * A
        px, py = X[m], Y[m]
        bay = px > 0
        bx, by = px[bay], py[bay]
        r = np.hypot(bx[:, None] - bx[None, :], by[:, None] - by[None, :])
        rb = np.hypot(bx[:, None] + bx[None, :], by[:, None] - by[None, :])
        Gm = (np.log(rb / np.maximum(r, 1e-12)) - k0(np.maximum(r, 1e-12) / ELL) + k0(rb / ELL)) / (2 * np.pi * T) * g * g
        c = (np.arange(16) + 0.5) / 16 * g - g / 2
        rs = np.hypot(*np.meshgrid(c, c))
        np.fill_diagonal(Gm, (np.mean(-np.log(rs) - k0(rs / ELL)) + np.log(2 * bx) + k0(2 * bx / ELL)) / (2 * np.pi * T) * g * g)
        nn = len(px)
        M = np.zeros((nn + 1, nn + 1))
        M[np.arange(nn), np.arange(nn)] = 1 / K
        M[:nn, nn] = -1
        idx = np.where(bay)[0]
        M[np.ix_(idx, idx)] += Gm
        M[nn, :nn] = g * g
        rhs = np.zeros(nn + 1)
        rhs[nn] = 2.5
        sol = np.linalg.solve(M, rhs)
        p, row = sol[:nn], np.abs(py) < g / 2
        bar_p = p[row & (px < 0)].mean()
        print(f"3. a facet centered {xc} mm into the bay: the bay's pressure over the bar's along its middle")
        for xx, pp in sorted(zip(px[row & bay], p[row & bay])):
            if min(abs(xx - q) for q in [0.16, 0.43, 0.97, 2.0, 3.0, 4.0]) < g / 2:
                print(f"   x {xx:5.2f}: {pp / bar_p:+.3f}  (1D {float(edge_hold(xx)):+.3f})")
