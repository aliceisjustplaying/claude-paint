#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Bone black's absorption per volume for engine 9, behind paint's `mixer::BONE_BLACK_K`
(the easel's `--@ bone-black` switch). See bone_black.md.

    uv run notes/research/bone_black_fit.py <repo root>

1. The fault. Engine 9's bone black K (crates/paint/src/mixer/calibrated.bin, made by
   scripts/calibrate_pigments9.py) dips 4x at 500-520 nm and peaks at 550 nm. That
   calibration fits OkLab colour and weights the spectra only 0.05, so the black and the
   whites traded spectral shape: zinc and lead white got a matching S spike at 550 nm.
   A tint still lands on its measured colour, but a thin film, where the black's
   transmission exp(-K x) shows, passes the 500-520 nm dip and reads teal. This script
   prints films and tints both ways.
2. The fit. Carbon's complex index is nearly flat across the visible (Bond and Bergstrom
   2006), so absorption by its small particles goes as 1/lambda. Fitted
   freely, K picks up the whites' 550 nm spike again, so it takes that shape,
   K(lambda) = K550 (lambda/550 nm)^-alpha. K550 and alpha are fitted to the
   measured Revigo reflectance (both fields of the opacity chart, specular included) of
   the bone-black ladders with zinc white (07C-07F) and lead white (08B, 08C). Recipes are
   converted to wet-paint volume as in calibrate_pigments9.py, and the whites keep
   engine 9's K and S. These ladders hold bone black as their only absorber. Mixtures with
   chromatic paints are reported, not fitted: those paints were co-fitted with the
   dipped black.
3. S stays the table's (about 1e-4 per 100 um; the Revigo spectra don't resolve it).
"""
import json
import sys
from pathlib import Path

import numpy as np
from scipy.optimize import least_squares

root = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(root / "scripts"))
from build_pigment_tables import film, inside, load_data, saunderson, spectral_array  # noqa: E402
from calibrate_pigments9 import mass_to_volume, paint_densities  # noqa: E402

LADDERS = ["07C", "07D", "07E", "07F", "08B", "08C"]
nm, spectra, samples = load_data()
cal = json.loads((root / "crates/paint/src/mixer/calibrated.json").read_text())
order = cal["order"]
K38, S38 = np.array(cal["absorption"]), np.array(cal["scattering"])
Kc, Sc = K38[:, 2:33], S38[:, 2:33]
B = order.index("BoneBlack")
densities = paint_densities()
ids = sorted(s for s, (n, f) in samples.items()
             if "BoneBlack" in n and f[n.index("BoneBlack")] > 0 and s not in {"07B", "14F"}
             and all(s + x in spectra for x in ("_b", "_w", "_ob", "_ow")))
C = np.zeros((len(ids), len(order)))
for r, s in enumerate(ids):
    names, fractions = samples[s]
    for name, v in zip(names, mass_to_volume(names, fractions, densities)):
        if v > 0:
            C[r, order.index(name)] += v
backs = np.array([[inside(spectra[s + f]) for f in ("_b", "_w")] for s in ids])
observed = np.array([[spectra[s + f] for f in ("_ob", "_ow")] for s in ids])

CMF = spectral_array("CMF")
NM38 = 380 + 10 * np.arange(38)


def extend(v):
    return np.concatenate([np.full(2, v[0]), v, np.full(5, v[-1])])


def lab(r38):
    xyz = (CMF @ r38) / CMF.sum(axis=1)
    f = np.where(xyz > (6 / 29) ** 3, np.cbrt(xyz), xyz / (3 * (6 / 29) ** 2) + 4 / 29)
    L, a, b = 116 * f[1] - 16, 500 * (f[0] - f[1]), 200 * (f[1] - f[2])
    return np.array([L, a, b, np.hypot(a, b), np.degrees(np.arctan2(b, a)) % 360])


def power(p, wl):
    return p[0] * (wl / 550.0) ** (-p[1])


def raw(kb):
    K = Kc.copy()
    K[B] = kb
    k, s = C @ K, C @ Sc
    return saunderson(film(k[:, None, :], s[:, None, :], backs), True), k, s


rows = [ids.index(s) for s in LADDERS]
fit = least_squares(lambda p: (raw(power(p, nm))[0] - observed)[rows].ravel(), [30.0, 1.0])
J = fit.jac
sigma = np.sqrt(np.diag(np.linalg.inv(J.T @ J) * (fit.fun ** 2).sum() / (len(fit.fun) - 2)))
K550, alpha = fit.x
print(f"K550 = {K550:.4f} per 100 um of tube paint per volume, alpha = {alpha:.4f} (± {sigma[1]:.3f})")
for subset in (LADDERS[:4], LADDERS[4:]):
    rr = [ids.index(s) for s in subset]
    p = least_squares(lambda p: (raw(power(p, nm))[0] - observed)[rr].ravel(), fit.x).x
    print(f"  on {subset} alone: K550 {p[0]:.2f}, alpha {p[1]:.3f}")

print("\nColour error over the white field (CIELAB dE76, D65), engine 9's table / the power law:")
old = raw(Kc[B])
new = raw(power(fit.x, nm))
errors = []
for r, s in enumerate(ids):
    target = lab(extend(saunderson(inside(spectra[s + "_ow"]), False)))
    d = []
    for k, sc in ((old[1], old[2]), (new[1], new[2])):
        m = lab(extend(saunderson(film(k[r], sc[r], inside(spectra[s + "_w"])), False)))
        d.append(np.linalg.norm(m[:3] - target[:3]))
    errors.append(d)
    print(f"  {s} {'+'.join(n for n, c in zip(*samples[s]) if c > 0):40s} {d[0]:5.2f} {d[1]:5.2f}"
          f"   measured L {target[0]:4.1f} a {target[1]:5.1f} b {target[2]:5.1f} h {target[4]:3.0f}"
          + ("   (fitted)" if s in LADDERS else ""))
errors = np.array(errors)
lad = np.array([ids.index(s) for s in LADDERS])
print(f"  mean: ladders {errors[lad, 0].mean():.2f} -> {errors[lad, 1].mean():.2f}; "
      f"all {len(ids)} {errors[:, 0].mean():.2f} -> {errors[:, 1].mean():.2f}")

ref = np.array(cal["reference_substrate"])


def show(tag, kb):
    sb = S38[B]
    print(f"\n{tag}")
    for um in (1, 3, 10, 30):
        t = um / 100
        r = saunderson(film(kb * t, sb * t, ref), False)
        L, a, b, c, h = lab(r)
        print(f"  film {um:2d} um over white: L {L:5.1f} a {a:6.2f} b {b:6.2f} C {c:5.2f} h {h:4.0f}")
    for white in ("LeadWhite", "ZincWhite"):
        w = order.index(white)
        for c in (0.01, 0.03, 0.1, 0.3):
            r = saunderson(film(c * kb + (1 - c) * K38[w], c * sb + (1 - c) * S38[w], ref), False)
            L, a, b, ch, h = lab(r)
            print(f"  {c:4.2f} by volume in {white:9s}: L {L:5.1f} a {a:6.2f} b {b:6.2f} C {ch:5.2f} h {h:4.0f}")


show("Engine 9's table (before):", K38[B])
show("The power law (after):", power(fit.x, NM38))
