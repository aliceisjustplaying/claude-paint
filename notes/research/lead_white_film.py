#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2"]
# ///
"""The numbers behind paint's `--@ tube-volume` switch (`Palette::tube_volume`,
`mixer::revigo_oil`, `mixer::REVIGO_FILM_UM`) and lead_white_film.md.

Engine 9's measured tubes are the Revigo paints (Geldof et al. 2018), fitted per 100 um of
paint. This script checks
1. what film those coefficients describe: Revigo's lead white 08A, its contrast ratio, and
   the S per pigment volume the engine's lead white implies at 100 um and at 50 um, against
   lead white's S per pigment volume from Mie and from Shreve's tinting strengths over
   LBNL's measured rutile (dry_pigment_optics.md 10.4), Pfund's cryptometer and de Viguerie
   et al.'s drawdown;
2. how much pigment a box's tube holds against the Revigo paint behind its table;
3. the tints by volume with lead white, before and after.

  uv run notes/research/lead_white_film.py <repo root>
"""
import ast
import json
import re
import sys
from pathlib import Path

import numpy as np

root = Path(sys.argv[1])
src = (root / "crates/paint/src/spectral.rs").read_text()


def table(name):
    v = src.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", v).strip()))


CMF = table("CMF")
NM = 380 + 10 * np.arange(38)
WHITE_XYZ = CMF.sum(axis=1)
cal = json.loads((root / "crates/paint/src/mixer/calibrated.json").read_text())
K = {n: np.array(cal["absorption"][i]) for i, n in enumerate(cal["order"])}
S = {n: np.array(cal["scattering"][i]) for i, n in enumerate(cal["order"])}
dens = json.loads((root / "scripts/pigment_densities.json").read_text())


def lab(r):
    x = CMF @ r / WHITE_XYZ
    f = np.where(x > 0.008856, np.cbrt(x), 7.787 * x + 16 / 116)
    return np.array([116 * f[1] - 16, 500 * (f[0] - f[1]), 200 * (f[1] - f[2])])


def y(r):
    return float(CMF[1] @ r / WHITE_XYZ[1])


def outside(r):  # engine 9's surface, specular excluded (mixer::outside)
    return 0.96 * 0.51 * r / (1 - 0.49 * r)


def inside(r):
    return r / (0.96 * 0.51 + 0.49 * r)


def film(k, s, x, rg):  # Kubelka's layer of optical thickness x over internal rg
    a = 1 + k / s
    b = np.sqrt(a * a - 1)
    sh, ch = np.sinh(b * s * x), np.cosh(b * s * x)
    c = a * sh + b * ch
    r, t = sh / c, b / c
    return r + t * t * rg / (1 - r * rg)


def rinf(k, s):
    q = k / s
    return 1 + q - np.sqrt(q * q + 2 * q)


# --- Revigo's lead white, as measured
lines = (root / "scripts/pigment_data/revigo/revigo-aggregated-palette-spectral-data.txt").read_text().split("\n")
rnm = np.array(lines[0].split("\t")[1:], float)
rev = {f[0]: np.interp(NM, rnm, np.array(f[1:], float) / 100) for f in (l.split("\t") for l in lines[1:] if l.strip())}
cr = (y(rev["08A_ob"]) - 0.04) / (y(rev["08A_ow"]) - 0.04)
print(f"Revigo 08A lead white: Y over black {y(rev['08A_ob']):.4f}, over white {y(rev['08A_ow']):.4f} "
      f"(fields {y(rev['08A_b']):.4f}, {y(rev['08A_w']):.4f}); contrast ratio, specular excluded, {cr:.3f}")

lw_s = S["LeadWhite"]
s100 = float(CMF[1] @ lw_s / CMF[1].sum())
print(f"engine 9 lead white S per nominal 100 um, luminance-weighted: {s100:.3f} (at 550 nm {lw_s[17]:.3f})")


def phi(w, rho_p, rho_oil=0.93):
    return 1 / (1 + w / (1 - w) * rho_p / rho_oil)


def rule(oa):
    return 1.45 * oa / (100 + 1.45 * oa)


pw = dens["paints"]["LeadWhite"]
phi_rev = phi(rule(pw["oil_absorption_estimate"]), pw["pigment_density"], dens["oil_densities"]["poppyseed"]["value"])
print(f"Revigo lead white paint: oil {rule(10):.4f} of its weight (rule, OA 10), pigment volume share {phi_rev:.3f}")
for t in (100, 50):
    print(f"  as a {t} um film: S per um of paint {s100 / t:.4f}, per pigment volume {s100 / t / phi_rev:.3f} um^-1")
for label, spv in (("Mie at 1 um (dry::lead_white_scatter)", 0.347), ("Shreve over LBNL rutile, low", 0.25),
                   ("Shreve over LBNL rutile, high", 0.38)):
    print(f"  {label}: {spv} um^-1 per pigment volume -> the film was {s100 / (spv * phi_rev):.0f} um")

# --- Bruce 1926 (NBS Technologic Paper 306, tables 2-3): white lead paint 5B, 68.3 % white lead and
# 30.6 % non-volatile vehicle by weight; contrast ratio (Martens photometer, plate black 0.053, white 0.75
# of MgCO3) against dry film thickness (Ames dial, +-2 um)
BRUCE = [(14, .494), (22, .559), (36, .782), (42, .822), (49, .868), (53, .886), (60, .908), (72, .942), (82, .947),
         (90, .967), (91, .967), (104, .965), (113, .971), (132, .985), (134, .985), (153, .986), (153, .986), (202, .992)]


def cr_of(pigment_um, t, black=0.053, white=0.75, spec=0.0):
    """Engine lead white with `pigment_um` of pigment per area, over gray fields, as a contrast ratio of Y."""
    x = pigment_um / phi_rev / t
    ob, ow = (y(outside(film(K["LeadWhite"], lw_s, x, inside(np.full(38, g))))) + spec for g in (black, white))
    return ob / ow


fits = {}
for oil_dry in (1.0, 0.95):
    phi_b = (68.3 / 6.81) / (68.3 / 6.81 + 30.6 / 0.93 * oil_dry)
    for spec in (0.0, 0.04):
        ts = np.arange(20, 120.01, 0.5)
        # (below Bruce's own criterion of hiding, 0.98: past it the fields differ by less than the 1-2 % the eye
        # and his photometer resolve, Bruce pp. 176-177)
        rms = [np.sqrt(np.mean([(cr_of(d * phi_b, t, spec=spec) - c) ** 2 for d, c in BRUCE if c < 0.98])) for t in ts]
        fits[(oil_dry, spec)] = (ts[int(np.argmin(rms))], min(rms), phi_b)
        print(f"Bruce 5B, dry oil at {oil_dry} of its volume (pigment share {phi_b:.3f}), specular {spec}: "
              f"film per nominal 100 um {fits[(oil_dry, spec)][0]:.1f} um, rms {min(rms):.3f}")
t, _, phi_b = fits[(0.95, 0.0)]
print(f"taken: {t} um. Bruce's films, measured / engine at {t} um / as fitted to 100 um:")
for d, c in BRUCE[::2]:
    print(f"  {d:4d} um: {c:.3f} / {cr_of(d * phi_b, t):.3f} / {cr_of(d * phi_b, 100):.3f}")
print(f"  at his mean hiding thickness 127 um (0.98): {cr_of(127 * phi_b, t):.3f} / {cr_of(127 * phi_b, 100):.3f}")
print(f"lead white's S per pigment volume so: {s100 / t / phi_rev:.3f} um^-1 (Mie at 1 um 0.347; Shreve/LBNL 0.25-0.38)")

# --- the other measurements, as checks
pig_um = 1 / 36 / 6.8 * 1e4  # Pfund 1919: complete hiding at 36 cm^2 per gram of basic carbonate white lead
print(f"Pfund: {pig_um:.1f} um of pigment at complete hiding by eye: contrast ratio {cr_of(pig_um, t, 0.0496, 0.8514):.3f} "
      f"(fitted to 100 um: {cr_of(pig_um, 100, 0.0496, 0.8514):.3f})")
dv_phi = phi(0.108, 6.7)  # de Viguerie et al. 2018: 10.8 % oil, 30 um applicator, over optical black, L* 82.5
for film_um in (10, 15, 30):
    r = outside(film(K["LeadWhite"], lw_s, film_um * dv_phi / phi_rev / t, np.zeros(38)))
    print(f"de Viguerie, L* 82.5 over black: a {film_um} um film is L* {lab(r)[0]:.1f}")
print(f"BYK: a block applicator lays a wet film of about half its gap: 50 um, {100 / 50:.1f}x the optical thickness "
      f"of the fit's 100 um; Bruce's {100 / t:.2f}x")

# --- the box's tubes against the Revigo paints (impressionist box: palette::BOX_GRIND and catalog densities)
BOX = {"lead white": ("LeadWhite", 0.13, 6.81), "zinc white": ("ZincWhite", 0.19, 5.66),
       "cobalt blue": ("CobaltBlue", 0.55, 4.2), "ultramarine blue": ("UltramarineBlue", 0.34, 2.35),
       "vermilion": ("Vermillion", 0.22, 8.1), "viridian": ("ViridianGreen", 0.50, 3.2),
       "chrome yellow": ("ChromeYellow", 0.33, 6.0), "bone black": ("BoneBlack", 0.55, 2.64)}
REV_W = {"ZincWhite": 0.37, "BoneBlack": 0.48, "CobaltBlue": 0.51, "UltramarineBlue": 0.41, "LeadWhite": rule(10),
         "Vermillion": rule(10), "ChromeYellow": rule(20), "ViridianGreen": rule(80)}
print("impressionist box: tube oil, pigment share; Revigo paint's; weight")
weight = {}
for name, (rv, w, rho) in BOX.items():
    weight[name] = phi(w, rho) / phi(REV_W[rv], rho)
    print(f"  {name:17s} {w:.2f} {phi(w, rho):.3f}   {REV_W[rv]:.3f} {phi(REV_W[rv], rho):.3f}   {weight[name]:.3f}")

print("tints by volume with lead white, R-infinity seen outside: as before / tube-volume")
for name in ("cobalt blue", "ultramarine blue", "vermilion"):
    rv = BOX[name][0]
    row = []
    for v in (0.01, 0.03, 0.07, 0.15, 0.30):
        for wt in ((1, 1), (weight["lead white"], weight[name])):
            k = (1 - v) * wt[0] * K["LeadWhite"] + v * wt[1] * K[rv]
            s = (1 - v) * wt[0] * lw_s + v * wt[1] * S[rv]
            row.append("/".join(f"{c:.1f}" for c in lab(outside(rinf(k, s)))))
    print(f"  {name:17s} " + "  ".join(f"{int(v * 100)}%: {row[2 * i]} / {row[2 * i + 1]}" for i, v in enumerate((0.01, 0.03, 0.07, 0.15, 0.30))))
