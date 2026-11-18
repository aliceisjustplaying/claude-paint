#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "scipy==1.17.1"]
# ///
"""The factors behind paint's `dry::powder_k` (the `--@ dry-earths` switch) and the
checks in dry_pigment_optics.md §7.

A pure earth pastel stick (burnt sienna, red earth, raw sienna, yellow ochre, raw umber)
and viridian came out of engine 9's dry rule as pale as a tint. Their oil tables fix K/S
in oil, not K per volume: K from a Kremer gum-arabic chart swatch read as an assumed
100 um oil film, S from an estimated hiding (scripts/pigment_data/researched-tubes.json).
The dry rule keeps K and scales S, so it needs K per volume, which those tables leave
unidentified, the lakes' problem (§6).

Inputs (as dry_lake_fit.py):
  CSV/        the CHSOS download of Cosentino 2014 (powder.csv; SHA-256 checked below)
  tubes.json  each tube's engine-9 dry-path K and S per coat and its index:
              DRY_EARTH_FIT_OUT=tubes.json cargo test -p paint --lib -- --ignored --exact \\
                  palette::tests::dump_dry_earth_fit_input

  uv run notes/research/dry_earth_fit.py <repo root> <CSV dir> <tubes.json>

The steps:
1. The powders are corrected from the instrument as dry_lake_fit.py does (gain from rutile
   titanium white's 0.945 at 550-700 nm, lead white's shape), but with the black floor
   measured: 0.022, what CHSOS's vine black reads less its measured difference from the
   blacks' mean if it reflects as charcoal powder does (3.00 % at 550 nm, Lethuillier et al.
   2022; dry_colours_fit.py step 2), in place of #318's 0.04 from a retired veil estimate.
   L* over the bounds 0.013-0.031 (the Mie vine black, 2.1 %, to 0.9 % above the charcoal).
2. S dry is what the earth's grains scatter in oil, measured (dry.rs `earth_scatter`,
   earth_scatter_lbnl.py: LBNL's acrylic films of burnt sienna, raw sienna and raw umber,
   their S per pigment volume at 550-700 nm), on lead white's scale (its S per volume in oil
   the Mie 3/4 sigma'(2.0) at 1 um, as `rule_scatter` has it), times Mie's air/oil ratio at
   the grains' measured size (Kremer's earths, laser diffraction). Red earth takes burnt
   sienna's S and yellow ochre raw sienna's (no film of either was measured). Viridian keeps
   the rule's Mie S at 1 um (no measurement found).
3. One factor on the oil table's K per volume is fitted so the pure dry colour matches the
   corrected powder's CIELAB (dE76).
4. The check against physics: the K per volume the fit gives, in um^-1 (K/S times the
   measured S per volume dry, the oil S times the air/oil ratio), against the iron oxide's
   own absorption coefficient: the hematite it takes, by volume.
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

SHA256 = {"powder.csv": "86bb05f002d64ad5aa0a1f66c7226f18b62824e77d3aa3e4e2a2aee3f0efae45"}
BLACK_TRUE = (0.013, 0.022, 0.031)
TITANIUM_TRUE = (0.90, 0.945, 0.99)
# tube -> CHSOS powder (Kremer pigments)
EARTHS = {"burnt sienna": "burntsienna", "red earth": "redochre", "raw sienna": "rawsienna",
          "yellow ochre": "yellowochre", "raw umber": "rawumber", "viridian": "viridian"}
# for context: the darks the switch leaves as they are
CONTEXT = {"ultramarine blue": "ultramarine", "Prussian blue": "prussianblue", "vine black": "vineblack",
           "bone black": "boneblack", "green earth": "greenearth", "cobalt blue": "cobaltblue"}

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


def fmt(r):
    return f"{' '.join(f'{v:3d}' for v in srgb8(r))} ({'/'.join(f'{v:.1f}' for v in lab(r))})"


rows = list(csv.reader((csv_dir / "powder.csv").open()))
head = rows[0]
data = np.array([[float(x) for x in r] for r in rows[1:] if len(r) == len(head) and all(x.strip() for x in r)])


def measured(name):
    nm, col = data[:, 0], head.index(name)
    return np.array([data[(nm >= c - 5) & (nm < c + 5), col].mean() for c in NM]) / 100


BLACK = np.mean([measured(n) for n in ("ivoryblack", "vineblack", "boneblack", "lampblack")], axis=0)
LEAD, TITANIUM = measured("leadwhite"), measured("titaniumwhite")


def corrected(name, black_true=BLACK_TRUE[1], titanium_true=TITANIUM_TRUE[1]):
    lead_true = black_true + (titanium_true - black_true) * (LEAD - BLACK)[FLAT].mean() / (TITANIUM - BLACK)[FLAT].mean()
    return black_true + (measured(name) - BLACK) * (lead_true - black_true) / (LEAD - BLACK)


# the engine's Mie table (dry.rs `mie_air`): n, sigma'_air (um^-1 at 1 um), S_air/S_oil
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


def rule_s(n):
    return RATIO_WHITE * WHITE_S * mie(n)[0] / SIGMA_WHITE


# dry.rs `earth_scatter`: S per pigment volume in oil (um^-1, measured) and the Mie air/oil ratio
# at the measured grain size (earth_scatter_lbnl.py)
EARTH_SCATTER = {"burnt sienna": (0.1845, 1.669), "red earth": (0.1845, 1.551), "raw sienna": (0.3817, 2.349),
                 "yellow ochre": (0.3817, 2.139), "raw umber": (0.0993, 1.936)}
WHITE_S_VOL = 0.75 * SIGMA_WHITE / RATIO_WHITE   # lead white's S per volume in oil on the rule's scale


def earth_s(name, medium="air"):
    """S per coat of an earth's grains with the switch, dry (air) or in oil."""
    if name not in EARTH_SCATTER:
        return rule_s(tubes[name]["n"]) if medium == "air" else rule_s(tubes[name]["n"]) / mie(tubes[name]["n"])[1]
    s, ratio = EARTH_SCATTER[name]
    return WHITE_S * s / WHITE_S_VOL * (ratio if medium == "air" else 1.0)


def part_ks(name, factor=None):
    """(K, S) per coat of one tube dry: the engine's rule (dry.rs `spectral_color`), or with the
    switch, its K times `factor` over the Mie S at its index (`rule_scatter`)."""
    t = tubes[name]
    k, s, n = np.array(t["k"]), np.array(t["s"]), t["n"]
    if factor is not None:
        return k * factor, earth_s(name)
    if n < 0:
        return k, -n * s
    sigma, ratio = mie(n)
    return k, (ratio * s if n >= 1.9 else rule_s(n))


def dry(parts, factors=None):
    factors = factors or {}
    K = S = 0
    for name, c in parts:
        k, s = part_ks(name, factors.get(name))
        K, S = K + c * k, S + c * s
    return km(K / S)


def fit(tube, target_lab):
    f = lambda la: np.linalg.norm(lab(dry([(tube, 1)], {tube: np.exp(la)})) - target_lab)
    r = minimize_scalar(f, bounds=(-4, 9), method="bounded")
    return float(np.exp(r.x)), float(r.fun)


print("1. the corrected powders (black 0.022, titanium white 0.945): sRGB (L*a*b*), L* over the bounds")
for tube, powder in {**EARTHS, **CONTEXT}.items():
    ls = [lab(corrected(powder, b, t))[0] for b in BLACK_TRUE for t in TITANIUM_TRUE]
    print(f"  {tube:16s} {powder:13s} {fmt(corrected(powder))}  L* {min(ls):.1f}-{max(ls):.1f}")

print("\n2-3. the factor on K, S the measured S (dry); dE76 to the powder")
print(f"  {'tube':16s} {'n':>4s}  {'engine dry now':26s} dE  {'factor':>7s}  {'with it':26s} dE   factor over the bounds")
FACTORS = {}
for tube, powder in EARTHS.items():
    target = lab(corrected(powder))
    a, de = fit(tube, target)
    FACTORS[tube] = a
    lo_hi = [fit(tube, lab(corrected(powder, b, t)))[0] for b in (BLACK_TRUE[0], BLACK_TRUE[2]) for t in (0.90, 0.99)]
    now = dry([(tube, 1)])
    print(f"  {tube:16s} {tubes[tube]['n']:4.2f}  {fmt(now):26s} {np.linalg.norm(lab(now) - target):4.1f} {a:8.3f}  "
          f"{fmt(dry([(tube, 1)], {tube: a})):26s} {de:4.1f}  {min(lo_hi):.3f}-{max(lo_hi):.3f}")

print("\n   the S dry the switch takes, against #351's (Mie at 1 um) and the rule's without it, at 550 nm:")
i550 = list(NM).index(550)
for tube in EARTHS:
    print(f"  {tube:16s} measured {earth_s(tube)[i550]:.4f}   #351 {rule_s(tubes[tube]['n'])[i550]:.4f}   "
          f"no switch {part_ks(tube)[1][i550]:.4f}   per coat")

print("\n   for the record, the same fit keeping the rule's S now (a factor on K alone, as dry-lakes):")
for tube, powder in EARTHS.items():
    target = lab(corrected(powder))
    t = tubes[tube]
    k0, s0 = part_ks(tube)
    f = lambda la: np.linalg.norm(lab(km(k0 * np.exp(la) / s0)) - target)
    r = minimize_scalar(f, bounds=(-4, 9), method="bounded")
    print(f"  {tube:16s} x{np.exp(r.x):.3f}  dE {r.fun:.1f}")

print("\n   tints 1:1 by volume with lead white, before -> after (not measured; for the guide):")
for tube in EARTHS:
    print(f"  {tube:16s} {fmt(dry([(tube, .5), ('lead white', .5)]))} -> {fmt(dry([(tube, .5), ('lead white', .5)], FACTORS))}")
print("   the painting's sticks (paintings/cassatt6), before -> after:")
for label, parts in {"hair: burnt sienna 2, red earth 1, lead white 0.3": [("burnt sienna", 2), ("red earth", 1), ("lead white", 0.3)],
                     "flesh_sh: lead white 2, red earth 1, viridian 0.3": [("lead white", 2), ("red earth", 1), ("viridian", 0.3)],
                     "brown: vine black 1, burnt sienna 1": [("vine black", 1), ("burnt sienna", 1)]}.items():
    tot = sum(c for _, c in parts)
    p = [(n, c / tot) for n, c in parts]
    print(f"  {label:52s} {fmt(dry(p))} -> {fmt(dry(p, FACTORS))}")

print("\n4. the check against the iron oxides' absorption: K per volume (um^-1) = K/S x S, S dry per volume")
print("   the measured oil S x the air/oil ratio; hematite (k 0.0925 at 550 nm) has K = 2 x 4 pi k / lambda = 4.23")
# hematite: its absorption coefficient alpha = 4 pi k / lambda; K = 2 alpha (diffuse) per volume of oxide
K_HEMATITE = 2 * 4 * np.pi * 0.0925 / 0.55
for tube in ("burnt sienna", "red earth", "raw umber", "yellow ochre", "raw sienna"):
    s_vol, ratio = EARTH_SCATTER[tube]
    k, s = part_ks(tube, FACTORS[tube])
    kv = k / s * s_vol * ratio
    print(f"  {tube:16s} K/S at 500/550/600/650 nm {np.round((k / s)[[12, 17, 22, 27]], 2)}   "
          f"K um^-1 {np.round(kv[[12, 17, 22, 27]], 2)}   at 550 nm as hematite by volume {kv[17] / K_HEMATITE:.0%}")
for tube in ("burnt sienna", "red earth"):
    k0, s0 = part_ks(tube)
    print(f"  {tube:16s} K/S now at 500/550/600/650 nm {np.round((k0 / s0)[[12, 17, 22, 27]], 3)}")
print("\nfor dry.rs powder_k:")
for tube, a in FACTORS.items():
    print(f'        "{tube}" => Some({a:.4g}),')
