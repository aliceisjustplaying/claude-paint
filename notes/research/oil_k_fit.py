#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""Oil K (and S) per volume for the three tubes whose engine-9 oil tables have too little
absorption: rose madder, green earth, copper green (verdigris). This is the oil-K PR that
dry_pigment_optics.md §6.5–6.6 left for later. The numbers are behind paint's
`mixer::OIL_K` (the `--@ oil-k` switch).

Inputs (as dry_lake_fit.py):
  CSV/        the CHSOS download of Cosentino 2014 (powder.csv and linseed oil.csv are used;
              SHA-256 checked below)
  tubes.json  each tube's engine-9 K and S per coat:
              DRY_LAKE_FIT_OUT=tubes.json cargo test -p paint --lib -- --ignored --exact \\
                  palette::tests::dump_dry_lake_fit_input

  uv run notes/research/oil_k_fit.py <repo root> <CSV dir> <tubes.json>

The steps:
1. **S from grain size, first.** Green earth and verdigris are coarse: about 10 µm and about
   100 µm, not the 1 µm the engine's dry Mie table is at. Their transport scattering per
   volume is Mie's at their size, in air (dry) and in linseed oil (n 1.48). Rose madder's
   alumina is a fine lake substrate, and its oil S is already physical to 1.5×
   (§6.5), so it keeps it.
2. **Then K.** A factor on the oil table's K per volume is fitted so that the pure dry colour
   (the engine's dry rule, with the size-aware S) matches the corrected CHSOS powder.
   Madder's is the dry-lakes loading, 35.7. K per volume is the pigment's in any medium
   (§2.4), so oil takes the same K.
3. **Oil S** is the dry S over Mie's air/oil ratio at the tube's index and size.
4. **The check against oil.** The CHSOS linseed-oil swatches are paint-outs on paper.
   - Correct them as the powders were, from a black oil film's total reflectance and
     titanium white's.
   - Read each swatch as a film of unknown thickness over the oiled paper (the database's
     own "Linseedoil" swatch), using the engine's two-constant film and Saunderson surface.
   - Fit two scalars on the engine's K and S spectra, giving the swatch's K/S. Compare it
     with the engine's, before and after.
"""
import csv
import hashlib
import json
import math
import sys
import importlib.util
from pathlib import Path

import numpy as np
from scipy.optimize import minimize, minimize_scalar

SHA256 = {
    "powder.csv": "86bb05f002d64ad5aa0a1f66c7226f18b62824e77d3aa3e4e2a2aee3f0efae45",
    "linseed oil.csv": "c4978b13edb37a3227ff78873481d57ef7dccf44917a466ca566aca6335439d8",
}
root, csv_dir, tubes_path = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
for name, sha in SHA256.items():
    if hashlib.sha256((csv_dir / name).read_bytes()).hexdigest() != sha:
        sys.exit(f"{csv_dir / name}: not the CHSOS 2014 file this fit was made from")
tubes = json.loads(tubes_path.read_text())
here = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("dry_pigment_mie", here / "dry_pigment_mie.py")
mie_mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mie_mod)

import ast, re  # noqa: E402

src = (root / "crates/paint/src/spectral.rs").read_text()


def table(name):
    v = src.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", v).strip()))


CMF = table("CMF")
TO_RGB = table("XYZ_RGB") @ CMF
NM = 380 + 10 * np.arange(38)
VIS = (NM >= 400) & (NM <= 700)
FLAT = (NM >= 550) & (NM <= 700)
WHITE_XYZ = CMF.sum(axis=1)


def km(q):
    q = np.maximum(q, 0)
    return 1 / (1 + q + np.sqrt(q * (q + 2)))


def srgb8(r):
    lin = np.clip(TO_RGB @ r, 0, 1)
    return np.round(np.where(lin <= 0.0031308, 12.92 * lin, 1.055 * lin ** (1 / 2.4) - 0.055) * 255).astype(int)


def lab(r):
    x = CMF @ r / WHITE_XYZ
    f = np.where(x > 0.008856, np.cbrt(x), 7.787 * x + 16 / 116)
    return np.array([116 * f[1] - 16, 500 * (f[0] - f[1]), 200 * (f[1] - f[2])])


def load(name):
    rows = list(csv.reader((csv_dir / name).open()))
    head = rows[0]
    data = np.array([[float(x) for x in r] for r in rows[1:] if len(r) == len(head) and all(x.strip() for x in r)])
    return head, data


def bands(data, col):
    nm = data[:, 0]
    return np.array([data[(nm >= c - 5) & (nm < c + 5), col].mean() for c in NM]) / 100


# --- the powders, corrected as dry_lake_fit.py corrects them (black 0.04, titanium white 0.945)
p_head, powder = load("powder.csv")
pm = lambda n: bands(powder, p_head.index(n))
P_BLACK = np.mean([pm(n) for n in ("ivoryblack", "vineblack", "boneblack", "lampblack")], axis=0)
P_LEAD, P_TI = pm("leadwhite"), pm("titaniumwhite")


def powder_corrected(name, black=0.04, ti=0.945):
    lead = black + (ti - black) * (P_LEAD - P_BLACK)[FLAT].mean() / (P_TI - P_BLACK)[FLAT].mean()
    return black + (pm(name) - P_BLACK) * (lead - black) / (P_LEAD - P_BLACK)


# --- the engine's dry rule (dry.rs `spectral_color`, its Mie table `mie_air`) at d = 1 um
MIE = [(1.50, 0.68, 700.0), (1.56, 0.71, 43.0), (1.65, 0.73, 9.0), (1.70, 0.80, 5.8), (1.74, 0.75, 3.9),
       (1.84, 0.82, 2.5), (2.00, 0.88, 1.9), (2.15, 0.91, 1.7), (2.30, 0.89, 1.6), (2.40, 1.04, 1.8),
       (2.45, 0.95, 1.5), (3.00, 1.03, 1.4)]


def mie_table(n):
    if n <= MIE[0][0]:
        return MIE[0][1:]
    for a, b in zip(MIE, MIE[1:]):
        if n <= b[0]:
            u = (n - a[0]) / (b[0] - a[0])
            return a[1] + (b[1] - a[1]) * u, math.exp(math.log(a[2]) + (math.log(b[2]) - math.log(a[2])) * u)
    return MIE[-1][1:]


WHITE_K, WHITE_S = np.array(tubes["lead white"]["k"]), np.array(tubes["lead white"]["s"])
SIGMA_WHITE, RATIO_WHITE = mie_table(2.0)
OIL_N = 1.48  # linseed oil


def dry_s_rule(n):
    """The engine's dry S per coat for n < 1.9, at 1 um: lead white's dry S x sigma'(n)/sigma'(2.0)."""
    return RATIO_WHITE * WHITE_S * mie_table(n)[0] / SIGMA_WHITE


# --- 1. scattering by grain size (Mie, lognormal sigma_ln 0.5 by number as dry_pigment_mie.py)
TUBES = {
    # tube: (CHSOS name, index, grain size um (low, mid, high), source)
    "green earth": ("greenearth", 1.62, (5.0, 10.0, 20.0), "Natural Pigments: Nicosia 10 um, Verona 12 um (mean); Kremer 0-120 um sieve"),
    "copper green": ("verdigris", 1.545, (30.0, 100.0, 200.0), "Kremer 44450: main peak 102.5 um (1.8-226), Reichert et al. 2025"),
}


def size_factors(n, d):
    s_air_d, _ = mie_mod.sizeavg(n, 1.0, d)
    s_air_1, _ = mie_mod.sizeavg(n, 1.0, 1.0)
    s_oil_d, _ = mie_mod.sizeavg(n, OIL_N, d)
    return s_air_d / s_air_1, s_air_d / s_oil_d


print("== 1. Grain size: sigma'(d)/sigma'(1 um) in air, and the air/oil ratio at d (Mie, n in oil 1.48)")
SIZE = {}
for tube, (_, n, ds, why) in TUBES.items():
    for d in ds:
        f, r = size_factors(n, d)
        SIZE[(tube, d)] = (f, r)
        print(f"  {tube:13s} n {n} d {d:5.0f} um: size factor {f:.4f}, air/oil {r:.1f}")
    print(f"    ({why})")
f1, r1 = size_factors(1.57, 1.0)
print(f"  check, the engine's table at n 1.57, 1 um: air/oil {r1:.1f} (table 43 at 1.56 by interpolation {mie_table(1.57)[1]:.1f})")


# --- 2. K: a factor on the oil table's K fitted to the corrected powder, with the size-aware S
def fit_k(tube, n, size_f, target):
    k = np.array(tubes[tube]["k"])
    s = dry_s_rule(n) * size_f
    f = lambda la: np.linalg.norm(lab(km(np.exp(la) * k / s)) - target)
    r = minimize_scalar(f, bounds=(-8, 10), method="bounded")
    return float(np.exp(r.x)), float(r.fun)


print("\n== 2. K per volume, fitted to the corrected powders (dE76 at the fit)")
FIT = {}
for tube, (name, n, ds, _) in TUBES.items():
    target = lab(powder_corrected(name))
    a1, de1 = fit_k(tube, n, 1.0, target)
    print(f"  {tube:13s} powder {srgb8(powder_corrected(name))} {np.round(target, 1)}; at 1 um (the note's): x{a1:.3g}, dE {de1:.1f}")
    for d in ds:
        a, de = fit_k(tube, n, SIZE[(tube, d)][0], target)
        FIT[(tube, d)] = a
        print(f"    at {d:5.0f} um: K x{a:.4g}, dE {de:.1f}")
    # the powder's own bounds (black 0.03-0.05, titanium 0.90-0.99) at the middle size
    span = [fit_k(tube, n, SIZE[(tube, ds[1])][0], lab(powder_corrected(name, b, t)))[0] for b in (0.03, 0.05) for t in (0.90, 0.99)]
    print(f"    over the powder correction's bounds at {ds[1]:.0f} um: x{min(span):.3g}-{max(span):.3g}")


# --- 3. oil: K x a, S = dry S / (air/oil); and the engine's masstone and tint with lead white
def oil(tube):
    k, s = np.array(tubes[tube]["k"]), np.array(tubes[tube]["s"])
    if tube == "rose madder":
        return k * 35.7, s
    _, n, ds, _ = TUBES[tube]
    d = ds[1]
    f, ratio = SIZE[(tube, d)]
    return k * FIT[(tube, d)], dry_s_rule(n) * f / ratio


OUT = {}
print("\n== 3. In oil: R_inf masstone and 1:1 by volume with lead white, before -> after (sRGB; L*a*b*)")
for tube in ("rose madder", "green earth", "copper green"):
    k0, s0 = np.array(tubes[tube]["k"]), np.array(tubes[tube]["s"])
    k1, s1 = oil(tube)
    OUT[tube] = (k1, s1)
    for label, k, s in (("before", k0, s0), ("after", k1, s1)):
        m, t = km(k / s), km((k + WHITE_K) / (s + WHITE_S))
        print(f"  {tube:13s} {label:6s} masstone {srgb8(m)} {np.round(lab(m), 1)}; 1:1 white {srgb8(t)} {np.round(lab(t), 1)}")
    print(f"    K x{np.mean(k1 / np.maximum(k0, 1e-12)):.4g}; S at 550 nm per coat {s0[NM == 550][0]:.4f} -> {s1[NM == 550][0]:.4f} (x{s1[NM == 550][0] / s0[NM == 550][0]:.4g})")

# --- 4. the check against the linseed-oil swatches
l_head, linseed = load("linseed oil.csv")
lm = lambda n: bands(linseed, l_head.index(n))
L_BLACK = np.mean([lm(n) for n in ("ivoryblack", "vineblack", "boneblack", "lampblaxk")], axis=0)
L_TI = lm("titaniumwhite")
# A black oil film's total reflectance is its surface's (n 1.48 at normal incidence: 3.7 %)
# and a trace of diffuse return: 0.04-0.05. Titanium white in oil, an opaque film: total
# 0.86-0.93 (rutile in oil reflects less than its dry powder's 0.90-0.99, the contrast being
# lower; the surface adds 0.04) [E within the bounds]. The sphere is linear (dry_lake_fit.py).
L_BLACK_TRUE, L_TI_TRUE = (0.04, 0.045, 0.05), (0.86, 0.895, 0.93)


def oil_corrected(name, black=L_BLACK_TRUE[1], ti=L_TI_TRUE[1]):
    gain = (ti - black) / (L_TI - L_BLACK)[FLAT].mean()
    return black + (lm(name) - L_BLACK) * gain


def outside(r):
    return (1 - 0.04) * (1 - 0.49) * r / (1 - 0.49 * r)


def inside(r):
    return r / ((1 - 0.04) * (1 - 0.49) + 0.49 * r)


def film(k, s, rg):
    tau = np.sqrt(k * (k + 2 * s))
    r = s / (k + s + tau)
    e = np.exp(-2 * tau)
    a, b = 1 - rg * r, r - rg
    return (r * a - b * e) / (a - r * b * e)


def swatch(k, s, x, y, paper):
    """A film of K x, S y (per coat units times its thickness in coats) over the oiled paper, measured with the sphere."""
    rg = inside(np.clip(paper - 0.04, 1e-4, 1))
    return 0.04 + outside(film(np.maximum(k * x, 1e-9), np.maximum(s * y, 1e-9), rg))


def fit_swatch(name, k, s, black=L_BLACK_TRUE[1], ti=L_TI_TRUE[1]):
    meas, paper = oil_corrected(name, black, ti), oil_corrected("Linseedoil", black, ti)
    err = lambda p: float(np.sum((swatch(k, s, math.exp(p[0]), math.exp(p[1]), paper) - meas)[VIS] ** 2))
    best = min((minimize(err, x0, method="Nelder-Mead", options={"xatol": 1e-6, "fatol": 1e-12, "maxiter": 20000}) for x0 in
                [(a, b) for a in (-3, 0, 3, 6) for b in (-4, -1, 2)]), key=lambda r: r.fun)
    x, y = math.exp(best.x[0]), math.exp(best.x[1])
    return x, y, math.sqrt(best.fun / VIS.sum()), meas, swatch(k, s, x, y, paper)


print(f"\n== 4. CHSOS linseed-oil swatches (corrected: black film {L_BLACK_TRUE[1]}, titanium white film {L_TI_TRUE[1]})")
paper = oil_corrected("Linseedoil")
print(f"  the oiled paper the swatches lie on: {srgb8(paper)} {np.round(lab(paper), 1)}; lead white swatch {srgb8(oil_corrected('leadwhite'))}")
for tube, name in (("rose madder", "madderlake"), ("green earth", "greenearth"), ("copper green", "verdigris")):
    meas = oil_corrected(name)
    print(f"  {tube} swatch: {srgb8(meas)} {np.round(lab(meas), 1)}")
    for label, (k, s) in (("before", (np.array(tubes[tube]["k"]), np.array(tubes[tube]["s"]))), ("after", OUT[tube])):
        x, y, rms, _, model = fit_swatch(name, k, s)
        spans = [fit_swatch(name, k, s, b, t) for b in (L_BLACK_TRUE[0], L_BLACK_TRUE[2]) for t in (L_TI_TRUE[0], L_TI_TRUE[2])]
        ratios = [sp[0] / sp[1] for sp in spans]
        print(f"    engine {label:6s}: the swatch is K x{x:.4g}, S x{y:.4g} of one coat: its K/S is x{x / y:.3g} the engine's "
              f"(x{min(ratios):.3g}-{max(ratios):.3g} over the correction's bounds), film {y:.3g} coats if S is right; rms {rms:.3f}")

# --- 5. the same check with the paint fixed: the one film thickness that best gives each swatch
# (a paint's K and S fix its K/S; the swatch's unknown thickness is the only free number)
print("\n== 5. The best film thickness for each paint (dE76 to the swatch at it; thickness in um, a coat being 25 um)")


def fit_thickness(name, k, s, black=L_BLACK_TRUE[1], ti=L_TI_TRUE[1]):
    meas, paper = oil_corrected(name, black, ti), oil_corrected("Linseedoil", black, ti)
    target = lab(meas)
    f = lambda lt: float(np.linalg.norm(lab(swatch(k, s, math.exp(lt), math.exp(lt), paper)) - target))
    r = minimize_scalar(f, bounds=(-8, 8), method="bounded")
    return math.exp(r.x) * 25, r.fun, swatch(k, s, math.exp(r.x), math.exp(r.x), paper)


for tube, name in (("rose madder", "madderlake"), ("green earth", "greenearth"), ("copper green", "verdigris")):
    row = []
    for label, (k, s) in (("before", (np.array(tubes[tube]["k"]), np.array(tubes[tube]["s"]))), ("after", OUT[tube])):
        h, de, model = fit_thickness(name, k, s)
        des = [fit_thickness(name, k, s, b, t)[1] for b in (L_BLACK_TRUE[0], L_BLACK_TRUE[2]) for t in (L_TI_TRUE[0], L_TI_TRUE[2])]
        row.append(f"{label} {h:7.1f} um, dE {de:4.1f} ({min(des):.1f}-{max(des):.1f}) {srgb8(model)}")
    print(f"  {tube:13s} swatch {srgb8(oil_corrected(name))}: " + "; ".join(row))
for tube in ("green earth", "copper green"):
    _, n, ds, _ = TUBES[tube]
    name = TUBES[tube][0]
    for d in ds:
        f, ratio = SIZE[(tube, d)]
        k = np.array(tubes[tube]["k"]) * FIT[(tube, d)]
        s = dry_s_rule(n) * f / ratio
        h, de, _ = fit_thickness(name, k, s)
        print(f"    {tube} at {d:.0f} um: best {h:.1f} um, dE {de:.1f}")

# --- the engine's constants (paint's mixer::OIL_K): K factor on the table's oil K; oil S as
# a multiple of lead white's oil S spectrum (None: the table's); the dry S's size factor
print("\n== The constants for mixer::OIL_K")
print('  ("rose madder", 35.7, None, 1.0)')
for tube in ("green earth", "copper green"):
    _, n, ds, _ = TUBES[tube]
    d = ds[1]
    f, ratio = SIZE[(tube, d)]
    c = RATIO_WHITE * mie_table(n)[0] / SIGMA_WHITE * f / ratio
    print(f'  ("{tube}", {FIT[(tube, d)]:.4g}, Some({c:.4g}), {f:.4g})  # d {d:.0f} um, n {n}, air/oil {ratio:.3g}')
