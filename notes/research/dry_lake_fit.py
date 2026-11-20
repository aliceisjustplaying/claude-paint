#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""The dye loadings behind paint's `dry::dye_loading` and the checks in
dry_pigment_optics.md §6.

Inputs:
  CSV/        the CHSOS download of Cosentino 2014, "FORS spectral database of
              historical pigments in different binders", e-conservation 2:54-65
              (https://chsopensource.org/new-free-pigments-fors-spectroscopy-database/,
              archive FORS-Spectral-database-of-historical-pigments-in-different-binders-CSV.zip).
              Not redistributed here; the SHA-256 of the three files used is checked below.
  tubes.json  each tube's engine-9 dry-path K and S per coat:
              DRY_LAKE_FIT_OUT=tubes.json cargo test -p paint --lib -- --ignored --exact \
                  palette::tests::dump_dry_lake_fit_input

  uv run notes/research/dry_lake_fit.py <repo root> <CSV dir> <tubes.json>

The instrument (Cosentino 2014, §2.1): an Ocean Optics USB4000 with an HL-2000
halogen lamp. Every spectrum in the database, the powders included, was taken
with an ISP-R integrating sphere, which includes the specular reflection. The
gum-arabic swatches were also measured with an R400-7 reflection probe at
45/45, which excludes the specular. The white standard is not named.

The raw sphere readings have a flat floor (carbon blacks 0.16-0.17) and a gain
(white powders 1.10-1.31), so the reference was not taken in the samples'
geometry. Physics fixes the correction:
- the sphere is linear: on the 54 gum-arabic swatches it reads the probe's
  value plus a flat 0.18, with a quadratic term below the scatter (printed);
- the floor is a carbon black powder's total reflectance: 0.022, measured
  (juniper charcoal powder reflects 3.00 % at 550 nm, Lethuillier et al. 2022,
  and CHSOS's own differences put vine black 0.0076 above the blacks' mean;
  bounds 0.013-0.031, dry_pigment_optics.md §9.1). #318 first took 0.03-0.05
  from §3's veil estimate, which #329 retired;
- the gain is set by the whites, and no powder can exceed 1. Rutile titanium
  white, the brightest, is flat from 550 to 700 nm and reflects 0.90-0.99 there
  as a dry powder (bulk rutile 0.90-0.91, fine pigment rutile 0.995 against
  Spectralon). Lead white, which has no visible absorption, gives the gain's
  shape across the spectrum (its raw spectrum and titanium white's fall alike
  from 430 to 700 nm: the lamp and reference, not the pigments).
The fit then finds, per lake, the factor on its oil K that makes its dry color
(two-constant KM, its scattering as the engine's rule for n < 1.9 at the
substrate's 1.57) match the corrected powder's CIELAB.

Magenta (fuchsine on alumina) has no powder. Its loading is set by its dye
instead: the K/S a fuchsine lake has at 543 nm, from fuchsine's measured molar
absorptivity, the package effect of a 1 um grain, and the substrate's Mie
scattering times the dependent-scattering (packing) factor. That factor is
computed here (Mie for the grain, Percus-Yevick for the powder's structure).
"""
import ast
import csv
import hashlib
import json
import re
import sys
from pathlib import Path

import numpy as np
from scipy.optimize import minimize_scalar

SHA256 = {
    "powder.csv": "86bb05f002d64ad5aa0a1f66c7226f18b62824e77d3aa3e4e2a2aee3f0efae45",
    "gum Arabic.csv": "56fd4e490d07faadf8f7a5b02a277f18d492900bf9065d43df73690a213e219d",
    "gum arabic refletion probe.csv": "eb9d5a6266c6d1a7ebc937f4ea2bb689b330409850079d3f48944d34b71402d6",
}
BLACK_TRUE = (0.013, 0.022, 0.031)  # a carbon black powder's total reflectance (§9.1): low, mid, high
TITANIUM_TRUE = (0.90, 0.945, 0.99)  # rutile powder, 550-700 nm: low, mid, high
SUBSTRATE_N = 1.57  # alumina hydrate (gibbsite)
LAKES = {"rose madder": "madderlake", "carmine lake": "carminelake", "yellow lake": "yellowlakeresed"}
CONTEXT = {"alizarin (Kremer)": "alizarin", "lac dye lake": "lacdye", "vermilion": "vermilion",
           "ultramarine": "ultramarine", "chalk": "chalk", "gypsum": "gypsum", "zinc white": "zincwhite",
           "titanium white": "titaniumwhite", "lead white": "leadwhite"}
# madder lake made in the lab and measured as powder (L*a*b*): Sattar 2024, J. Chem. Educ. 2020
MADDER_LAB = {"Sattar 2024": (66.1, 25.5, 9.4), "J. Chem. Educ. 2020": (62.6, 23.9, 16.7)}
# the indices `dry::measured_index` gives with the switch, and the CHSOS powder to check each against
INDEX_NOW = {"permanent alizarin": 2.03, "green earth": 1.62, "copper green": 1.545, "lead-tin yellow": 2.30,
             "transparent oxide yellow": 2.35, "Mars yellow": 2.35, "Rinmann's green": 2.02}
INDEX_POWDER = {"green earth": "greenearth", "copper green": "verdigris", "lead-tin yellow": "leadtinyellowI"}

root, csv_dir, tubes_path = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
for name, sha in SHA256.items():
    if hashlib.sha256((csv_dir / name).read_bytes()).hexdigest() != sha:
        sys.exit(f"{csv_dir / name}: not the CHSOS 2014 file this fit was made from")
tubes = json.loads(tubes_path.read_text())
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


def ks(r):
    return (1 - r) ** 2 / (2 * r)


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
    """The mean over each 10 nm band, as a fraction."""
    nm = data[:, 0]
    return np.array([data[(nm >= c - 5) & (nm < c + 5), col].mean() for c in NM]) / 100


head, powder = load("powder.csv")
names = head[1:55]


def measured(name):
    return bands(powder, head.index(name))


# --- the sphere is linear: gum-arabic swatches, sphere against the probe (columns in the same order)
_, sphere_ga = load("gum Arabic.csv")
_, probe_ga = load("gum arabic refletion probe.csv")
S = np.array([bands(sphere_ga, i)[VIS] for i in range(1, 55)])
P = np.array([bands(probe_ga, i)[VIS] for i in range(1, 55)])
# chalk and gypsum nearly match gum arabic (n 1.52-1.6): their swatches are the paper seen through a clear film
keep = [i for i, n in enumerate(names) if n not in ("gypsum", "chalk")]
p, s = P[keep].ravel(), S[keep].ravel()
print("gum-arabic swatches, sphere (specular included) on probe (45/45, specular excluded), 400-700 nm:")
for deg in (1, 2):
    a = np.vstack([p ** k for k in range(deg + 1)]).T
    co, *_ = np.linalg.lstsq(a, s, rcond=None)
    print(f"  degree {deg}: coefficients {np.round(co, 3)}, rms {np.sqrt(np.mean((a @ co - s) ** 2)):.3f}")
madder_probe = bands(probe_ga, names.index("madderlake") + 1)
print(f"  madder lake in gum arabic, probe: {madder_probe[NM == 510][0]:.2f} at 510 nm, {madder_probe[NM == 700][0]:.2f} "
      "at 700 nm (Fonseca 2019, 45/0 on 99% Spectralon, madder lakes in gum arabic: about 0.14 and 0.75)")

BLACK = np.mean([measured(n) for n in ("ivoryblack", "vineblack", "boneblack", "lampblack")], axis=0)
LEAD, TITANIUM = measured("leadwhite"), measured("titaniumwhite")
CHALK_GYPSUM = (measured("chalk") + measured("gypsum")) / 2
print(f"\nraw floor (blacks) {BLACK[VIS].mean():.3f}; raw whites 550-700 nm: titanium {TITANIUM[FLAT].mean():.3f}, "
      f"lead {LEAD[FLAT].mean():.3f}, chalk and gypsum {CHALK_GYPSUM[FLAT].mean():.3f}")


def corrected(name, black_true=BLACK_TRUE[1], titanium_true=TITANIUM_TRUE[1]):
    # lead white's true level, from titanium white's in the flat band
    lead_true = black_true + (titanium_true - black_true) * (LEAD - BLACK)[FLAT].mean() / (TITANIUM - BLACK)[FLAT].mean()
    return black_true + (measured(name) - BLACK) * (lead_true - black_true) / (LEAD - BLACK)


old_cg = 0.04 + (TITANIUM - BLACK) * 0.89 / (CHALK_GYPSUM - BLACK)
print(f"energy check: with chalk and gypsum taken as 0.93 (the earlier correction), titanium white would reflect "
      f"{old_cg[FLAT].mean():.3f} > 1, so that reference is out; lead white comes out at "
      f"{corrected('leadwhite')[FLAT].mean():.3f} (range {corrected('leadwhite', BLACK_TRUE[0], 0.90)[FLAT].mean():.3f}-"
      f"{corrected('leadwhite', BLACK_TRUE[2], 0.99)[FLAT].mean():.3f})")

print(f"\npowder, corrected (black {BLACK_TRUE[1]}, titanium white 0.945): sRGB, L*a*b*; L* over the bounds")
for label, name in {**LAKES, **CONTEXT}.items():
    r = corrected(name)
    ls = [lab(corrected(name, b, t))[0] for b in BLACK_TRUE for t in TITANIUM_TRUE]
    print(f"  {label:18s} {srgb8(r)} {np.round(lab(r), 1)}  L* {min(ls):.1f}-{max(ls):.1f}")
for who, v in MADDER_LAB.items():
    print(f"  madder lake, {who}: {v}, dE76 to the corrected madder {np.linalg.norm(lab(corrected('madderlake')) - v):.1f}")

# the engine's Mie table (dry.rs `mie_air`)
MIE = [(1.50, 0.68, 700.0), (1.56, 0.71, 43.0), (1.65, 0.73, 9.0), (1.70, 0.80, 5.8), (1.74, 0.75, 3.9),
       (1.84, 0.82, 2.5), (2.00, 0.88, 1.9), (2.15, 0.91, 1.7), (2.30, 0.89, 1.6), (2.40, 1.04, 1.8),
       (2.45, 0.95, 1.5), (3.00, 1.03, 1.4)]


def mie(n):
    if n <= MIE[0][0]:
        return MIE[0][1:]
    for a, b in zip(MIE, MIE[1:]):
        if n <= b[0]:
            u = (n - a[0]) / (b[0] - a[0])
            return a[1] + (b[1] - a[1]) * u, np.exp(np.log(a[2]) + (np.log(b[2]) - np.log(a[2])) * u)
    return MIE[-1][1:]


WHITE_S = np.array(tubes["lead white"]["s"])
SIGMA_WHITE, RATIO_WHITE = mie(2.0)


def dry_ks(parts, loading=None, index=None):
    """The engine's dry K and S (dry.rs `spectral_color`) of (tube, share) parts."""
    loading, index = loading or {}, index or {}
    K = S = 0
    for name, c in parts:
        t = tubes[name]
        k, s, n = np.array(t["k"]), np.array(t["s"]), index.get(name, t["n"])
        if name in loading:
            k, n = k * loading[name], SUBSTRATE_N
        sigma, ratio = mie(n)
        K = K + c * k
        S = S + c * (ratio * s if n >= 1.9 else RATIO_WHITE * WHITE_S * sigma / SIGMA_WHITE)
    return K, S


def dry(parts, loading=None, index=None):
    K, S = dry_ks(parts, loading, index)
    return km(K / S)


def fit(tube, target_lab):
    f = lambda la: np.linalg.norm(lab(dry([(tube, 1)], {tube: np.exp(la)})) - target_lab)
    r = minimize_scalar(f, bounds=(-8, 8), method="bounded")
    return float(np.exp(r.x)), float(r.fun)


print("\nfit: tube, loading, dE76 at the fit, dry sRGB and L*a*b* now -> with the loading; 1:1 lead white now -> with it")
LOADING = {}
for tube, name in LAKES.items():
    target = lab(corrected(name))
    a, de = fit(tube, target)
    LOADING[tube] = a
    was, now = dry([(tube, 1)]), dry([(tube, 1)], {tube: a})
    tint_was, tint_now = dry([(tube, .5), ("lead white", .5)]), dry([(tube, .5), ("lead white", .5)], {tube: a})
    print(f"  {tube:13s} {a:6.3g}  dE {de:4.1f}  target {np.round(target, 1)}  {srgb8(was)} {np.round(lab(was), 1)} -> "
          f"{srgb8(now)} {np.round(lab(now), 1)}  (dE now {np.linalg.norm(lab(was) - target):.1f})  "
          f"1:1 {srgb8(tint_was)} -> {srgb8(tint_now)}")

print("\nloading over the correction's bounds (black taken as, titanium white taken as)")
for b in BLACK_TRUE:
    for t in TITANIUM_TRUE:
        cells = [fit(tube, lab(corrected(name, b, t)))[0] for tube, name in LAKES.items()]
        print(f"  {b:.2f} {t:.3f}  " + "  ".join(f"{tube[:6]} {c:.3g}" for tube, c in zip(LAKES, cells)))

# --- the loading against dye chemistry: K/S at the band from dye content, molar absorptivity and Mie
# K = 2 mu_a, S = 3/4 sigma' (§2.1), both per volume of pigment; mu_a = ln10 eps c, c = w rho / M,
# lowered by the package effect of a d = 1 um particle (van de Hulst's absorption efficiency).
D_UM = 1.0
SIGMA_LAKE = mie(SUBSTRATE_N)[0]  # um^-1 per volume, d = 1 um


def bhmie(x, m, theta):
    """Mie amplitudes S1, S2 of a sphere (Bohren & Huffman's BHMIE)."""
    nstop = int(x + 4 * x ** (1 / 3) + 2)
    y = m * x
    nmx = int(max(nstop, abs(y)) + 15)
    d = np.zeros(nmx + 1, dtype=complex)
    for n in range(nmx, 0, -1):
        d[n - 1] = n / y - 1 / (d[n] + n / y)
    mu = np.cos(theta)
    pi0, pi1 = np.zeros_like(mu), np.ones_like(mu)
    s1, s2 = np.zeros_like(mu, dtype=complex), np.zeros_like(mu, dtype=complex)
    psi0, psi1, chi0, chi1 = np.cos(x), np.sin(x), -np.sin(x), np.cos(x)
    xi1 = complex(psi1, -chi1)
    for n in range(1, nstop + 1):
        psi, chi = (2 * n - 1) * psi1 / x - psi0, (2 * n - 1) * chi1 / x - chi0
        xi = complex(psi, -chi)
        an = ((d[n] / m + n / x) * psi - psi1) / ((d[n] / m + n / x) * xi - xi1)
        bn = ((m * d[n] + n / x) * psi - psi1) / ((m * d[n] + n / x) * xi - xi1)
        tau = n * mu * pi1 - (n + 1) * pi0
        f = (2 * n + 1) / (n * (n + 1))
        s1, s2 = s1 + f * (an * pi1 + bn * tau), s2 + f * (an * tau + bn * pi1)
        pi0, pi1 = pi1, ((2 * n + 1) * mu * pi1 - (n + 1) * pi0) / n
        psi0, psi1, chi0, chi1, xi1 = psi1, psi, chi1, chi, xi
    return s1, s2


def structure_py(u, phi):
    """Percus-Yevick hard-sphere structure factor S(q), u = q d, at volume fraction phi."""
    a = (1 + 2 * phi) ** 2 / (1 - phi) ** 4
    b = -6 * phi * (1 + phi / 2) ** 2 / (1 - phi) ** 4
    g = phi * a / 2
    u = np.maximum(u, 0.05)  # the closed form cancels below this; S(q) is flat there
    sn, cs = np.sin(u), np.cos(u)
    G = (a / u ** 2 * (sn - u * cs) + b / u ** 3 * (2 * u * sn + (2 - u ** 2) * cs - 2)
         + g / u ** 5 * (-u ** 4 * cs + 4 * ((3 * u ** 2 - 6) * cs + (u ** 3 - 6 * u) * sn + 6)))
    return 1 / (1 + 24 * phi * G / u)


def packing_factor(phi, n=SUBSTRATE_N, lam=0.545, d50=D_UM, sigma_ln=0.5):
    """Transport scattering in a powder of volume fraction phi over the independent grains'.

    Each grain's Mie phase function, weighted by (1 - cos theta), is multiplied by the
    Percus-Yevick S(q) at q = 2k sin(theta/2) (the quasi-crystalline approximation, host air),
    over a lognormal of sizes as table 2.2's."""
    theta = np.linspace(0, np.pi, 2001)
    k = 2 * np.pi / lam
    z = np.linspace(-3, 3, 41)
    num = den = 0.0
    for d, w in zip(d50 * np.exp(sigma_ln * z), np.exp(-z ** 2 / 2)):
        s1, s2 = bhmie(k * d / 2, complex(n, 0), theta)
        t = (abs(s1) ** 2 + abs(s2) ** 2) * (1 - np.cos(theta)) * np.sin(theta)
        num += w * np.trapezoid(t * structure_py(2 * k * np.sin(theta / 2) * d, phi), theta)
        den += w * np.trapezoid(t, theta)
    return num / den


PHI = (0.40, 0.55)  # a loose powder's solids by volume
FACTORS = {phi: packing_factor(phi) for phi in (0.40, 0.45, 0.50, 0.55)}
PACKING = (min(FACTORS.values()), max(FACTORS.values()))
print(f"\npacking factor on S, n {SUBSTRATE_N} at 1 um (lognormal sigma_ln 0.5), 545 nm, Mie x Percus-Yevick: "
      + ", ".join(f"phi {phi:.2f} {v:.3f}" for phi, v in FACTORS.items())
      + f"; at 0.5 um, phi 0.45: {packing_factor(0.45, d50=0.5):.3f}")
RHO = (2000, 2400)  # g/L, a lake (organic on gibbsite, 2.42 g/cm3)
DYES = {  # band nm, molar mass g/mol, molar absorptivity L/mol/cm (low, high)
    "rose madder": ("madderlake", 510, 240.2, (5.0e3, 1.7e4)),  # alizarin; alizarin red S-Al 1.7e4 at 510 nm
    "carmine lake": ("carminelake", 530, 492.4, (6.84e3, 1.0e4)),  # carminic acid, E1% 139 at 494 nm (EU 231/2012)
}


def package(w):
    """Absorption per volume of a sphere over the thin limit, x = mu_a d."""
    return 1.5 / w * (1 + 2 * np.exp(-w) / w + 2 * (np.exp(-w) - 1) / w ** 2)


def ks_dye(wt, eps, rho, packing, M):
    mu = np.log(10) * eps * wt * rho / M * 1e-4  # um^-1
    return 2 * mu * package(mu * D_UM) / (0.75 * SIGMA_LAKE * packing)


def content_for(target, eps, M):
    """The dye content (wt%) over the bounds of eps, density and packing that gives K/S `target`."""
    ws = []
    for e in eps:
        for rho in RHO:
            for pk in PACKING:
                lo_, hi_ = np.log(1e-6), 0.0
                if ks_dye(1.0, e, rho, pk, M) < target:
                    ws.append(np.inf)
                    continue
                for _ in range(80):
                    mid = (lo_ + hi_) / 2
                    lo_, hi_ = (mid, hi_) if ks_dye(np.exp(mid), e, rho, pk, M) < target else (lo_, mid)
                ws.append(np.exp(lo_) * 100)
    return min(ws), max(ws)


print("\nK/S at the band, pure lake dry: the powder; the engine before and with the loading; dye chemistry")
for tube, (name, band, M, eps) in DYES.items():
    i = NM == band
    k_was, s_was = dry_ks([(tube, 1)])
    k_now, s_now = dry_ks([(tube, 1)], {tube: LOADING[tube]})
    powder_ks = ks(corrected(name))[i][0]
    span = [ks(corrected(name, b, t))[i][0] for b in BLACK_TRUE for t in TITANIUM_TRUE]
    print(f"  {tube} at {band} nm: powder {powder_ks:.2f} ({min(span):.2f}-{max(span):.2f}); engine before "
          f"{(k_was / s_was)[i][0]:.3f}, with the loading {(k_now / s_now)[i][0]:.2f}")
    for wt in (0.01, 0.03, 0.10, 0.20, 0.50, 1.0):
        lo = ks_dye(wt, eps[0], RHO[0], PACKING[1], M)
        hi = ks_dye(wt, eps[1], RHO[1], PACKING[0], M)
        print(f"    dye {wt * 100:4.0f} wt%: K/S {lo:.2f}-{hi:.2f}")
    for label, target in (("the powder", powder_ks), ("the engine before", (k_was / s_was)[i][0]),
                          ("the engine with the loading", (k_now / s_now)[i][0])):
        lo, hi = content_for(target, eps, M)
        print(f"    dye content that gives {label}'s K/S: {lo:.2g}-{hi:.2g} wt% (inf: beyond pure dye)")

# --- the indices given with the switch, against the powders
print("\nindex with the switch: tube, n before -> now, dry sRGB and L*a*b* before -> now; the corrected powder")
for tube, n in INDEX_NOW.items():
    was, now = dry([(tube, 1)]), dry([(tube, 1)], index={tube: n})
    tint = [srgb8(dry([(tube, .5), ("lead white", .5)], index=ix)) for ix in ({}, {tube: n})]
    line = (f"  {tube:24s} {tubes[tube]['n']} -> {n}: {srgb8(was)} {np.round(lab(was), 1)} -> {srgb8(now)} {np.round(lab(now), 1)}"
            f" (1:1 lead white {tint[0]} -> {tint[1]})")
    if tube in INDEX_POWDER:
        target = lab(corrected(INDEX_POWDER[tube]))
        line += (f"; powder {srgb8(corrected(INDEX_POWDER[tube]))} {np.round(target, 1)}, dE76 "
                 f"{np.linalg.norm(lab(was) - target):.1f} -> {np.linalg.norm(lab(now) - target):.1f}")
    print(line)

# --- rose madder in oil (not changed here): what a physical fix would give
# K per volume is the dye's in any medium, so oil K takes the same loading; S in oil is the
# substrate's dry S over its Mie air/oil ratio at n 1.57 (oil 1.48), instead of an estimated hiding.
k, s = np.array(tubes["rose madder"]["k"]), np.array(tubes["rose madder"]["s"])
a = LOADING["rose madder"]
_, s_dry = dry_ks([("rose madder", 1)], {"rose madder": a})
s_mie = s_dry / mie(SUBSTRATE_N)[1]
kw, sw = np.array(tubes["lead white"]["k"]), np.array(tubes["lead white"]["s"])
print(f"\nrose madder in oil: S at 550 nm per coat, oil table {s[NM == 550][0]:.3f}, from Mie {s_mie[NM == 550][0]:.3f}")
for label, kk, ss in (("now", k, s), (f"K x {a:.3g}", a * k, s), (f"K x {a:.3g}, S from Mie", a * k, s_mie)):
    mass, tint = km(kk / ss), km((kk + kw) / (ss + sw))
    print(f"  {label:22s} masstone R_inf {srgb8(mass)} {np.round(lab(mass), 1)}; 1:1 lead white {srgb8(tint)} {np.round(lab(tint), 1)}")

# --- magenta: fuchsine on alumina has no powder; its K/S at the band comes from its dye
# Basic fuchsin (rosaniline hydrochloride, CAS 632-99-5, M 337.8): eps 8.06e4 at 543 nm (PhotochemCAD L04, PBS).
M_FUCHSINE, EPS_FUCHSINE, BAND_FUCHSINE = 337.8, 8.06e4, 543
WT_LAKE = (0.03, 0.10, 0.20)  # a lake's dye content: low, middle, high (§6.3)
pk_mid = sum(PACKING) / 2
print(f"\nmagenta (fuchsine lake) at {BAND_FUCHSINE} nm, dye chemistry, packing {PACKING[0]:.3f}-{PACKING[1]:.3f}:")
for wt in (0.005, 0.01, 0.03, 0.05, 0.10, 0.20, 1.0):
    v = [ks_dye(wt, EPS_FUCHSINE, r, pk, M_FUCHSINE) for r in RHO for pk in PACKING]
    mu_d = np.log(10) * EPS_FUCHSINE * wt * sum(RHO) / 2 / M_FUCHSINE * 1e-4 * D_UM
    print(f"  dye {wt * 100:4.1f} wt%: K/S {min(v):.2f}-{max(v):.2f} (mu_a d {mu_d:.1f})")
target = ks_dye(WT_LAKE[1], EPS_FUCHSINE, sum(RHO) / 2, pk_mid, M_FUCHSINE)
span = [ks_dye(wt, EPS_FUCHSINE, r, pk, M_FUCHSINE) for wt in (WT_LAKE[0], WT_LAKE[2]) for r in RHO for pk in PACKING]
cap = 2 * 1.5 / D_UM / (0.75 * SIGMA_LAKE * pk_mid)
print(f"  a lake's K/S: {target:.2f} at {WT_LAKE[1] * 100:.0f} wt% (over {WT_LAKE[0] * 100:.0f}-{WT_LAKE[2] * 100:.0f} wt%: "
      f"{min(span):.2f}-{max(span):.2f}); the 1 um grain's package limit {cap:.2f}")
u = (BAND_FUCHSINE - 540) / 10


def ks_at_band(tube, a):
    K, S = dry_ks([(tube, 1)], {tube: a})
    r = K / S
    return (1 - u) * r[NM == 540][0] + u * r[NM == 550][0]


ks_unit = ks_at_band("magenta", 1.0)
a_magenta = target / ks_unit
was = dry([("magenta", 1)])
now = dry([("magenta", 1)], {"magenta": a_magenta})
tint_was, tint_now = dry([("magenta", .5), ("lead white", .5)]), dry([("magenta", .5), ("lead white", .5)], {"magenta": a_magenta})
print(f"  engine K/S at {BAND_FUCHSINE} nm with no loading {ks_unit:.4f}: loading {a_magenta:.4g} "
      f"(over the span {min(span) / ks_unit:.3g}-{max(span) / ks_unit:.3g}; yellow lake's {LOADING['yellow lake']:.3g} gives "
      f"K/S {ks_at_band('magenta', LOADING['yellow lake']):.2f})")
print(f"  magenta dry {srgb8(was)} {np.round(lab(was), 1)} -> {srgb8(now)} {np.round(lab(now), 1)}; "
      f"1:1 lead white {srgb8(tint_was)} -> {srgb8(tint_now)}")

# --- for a later PR (not applied): green earth and verdigris dry, a loading on their oil K fitted to
# their corrected powders exactly as for the lakes, at their measured index (their S as the engine has it)
print("\nlater PR, not applied: a loading on the oil K of green earth and verdigris, at their measured index")
for tube, name in (("green earth", "greenearth"), ("copper green", "verdigris")):
    target_lab = lab(corrected(name))
    n = INDEX_NOW[tube]
    f = lambda la: np.linalg.norm(lab(km(np.exp(la) * dry_ks([(tube, 1)], index={tube: n})[0]
                                         / dry_ks([(tube, 1)], index={tube: n})[1])) - target_lab)
    r = minimize_scalar(f, bounds=(-8, 8), method="bounded")
    a = float(np.exp(r.x))
    K, S = dry_ks([(tube, 1)], index={tube: n})
    print(f"  {tube:13s} loading {a:.3g}, dE76 {r.fun:.1f} to the powder {np.round(target_lab, 1)}; dry {srgb8(km(a * K / S))}")
