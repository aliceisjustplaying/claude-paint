#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "pdfplumber==0.11.4", "scipy==1.17.1"]
# ///
"""The measured absorption and scattering behind paint's `dry::lbnl_film` (the `--@ dry-earths`
and `--@ oil-earths` switches) and the checks in dry_pigment_optics.md §10.

#351 gave the earths a K per volume from a Kremer gum-arabic chart swatch (a thin layer read
as an assumed 100 um oil film) times one fitted factor (`dry::powder_k`), and lead white's
spectral shape of S. One factor can't fix a K *shape*. The LBNL Pigment Database (Levinson,
Berdahl & Akbari, Sol. Energy Mater. Sol. Cells 89 (2005) 319-349 and 351-389) measured
both K and S, spectrally, of the same films: acrylic (binder n 1.49; fresh linseed oil 1.48),
each film over a void, a white and a black ground, inverted with its measured thickness.
This script reads them back for every tube the engine has whose pigment LBNL measured:

  burnt sienna  B13  calcined natural iron oxide, PBr 7 (Liquitex), PVC 13 %
  raw sienna    B14  natural iron oxide, PBr 7 (Liquitex), PVC 20.1 %
  raw umber     B15  natural iron oxide, PBr 7 (Liquitex), PVC 10.9 %
  Mars red      R04  synthetic red iron oxide, PR 101 (Liquitex "Red Oxide"), PVC 13 %
  Mars yellow   Y01  synthetic hydrated iron oxide, PY 42 (Liquitex "Yellow Oxide"), PVC 16 %

K and S per pigment volume (um^-1) are each film's K and S (1/mm) over its pigment volume
concentration (the charts' dry-film PVC). They hold in oil as they are (n 1.49 against 1.48).
Dry, K per volume is the same (a pigment absorbs per volume as it does in any medium) and S is
the oil S times Mie's air/oil ratio at the grains' size (the earths' measured sizes,
earth_scatter_lbnl.py; the Mars colours the dry rule's 1 um, no size measured).

Not used, and why (the tubes keep #351's method or the rule):
  red earth, yellow ochre  natural ochres: LBNL has only synthetic iron oxides (R01-R04, Y01)
  Indian red, transparent oxide yellow  other grades of PR 101 / PY 42 than R04's and Y01's
  cadmiums  LBNL's are PY 35 (cadmium zinc sulfide) and PO 20 (sulfoselenide orange); the
            engine's are cadmium sulfide (PY 37) grades and a sulfoselenide red
  cobalt blue U05, chrome yellow Y03, ultramarine U11, Prussian blue U10, bone black B02
            the engine's are measured oil tables already (calibrated on oil tints), not thin
            swatches; U11, U10 and B02's masstone S is below the charts' axis (unreadable)
  viridian  LBNL's chromium greens are anhydrous PG 17, not hydrated PG 18

Inputs:
  a directory with the LBNL chart PDFs (SHA-256 checked below), from
  https://coolcolors.lbl.gov/LBNL-Pigment-Database/assets/spectral-charts/pdf/
  the CHSOS CSV/ directory (Cosentino 2014; as dry_earth_fit.py)
  tubes.json (as dry_earth_fit.py: DRY_EARTH_FIT_OUT=tubes.json cargo test -p paint --lib --
             --ignored --exact palette::tests::dump_dry_earth_fit_input)

  uv run notes/research/earth_lbnl_ks.py <repo root> <pdf dir> <CSV dir> <tubes.json>

Steps:
1. Read each masstone's K and S back (as earth_scatter_lbnl.py), and check them: with the
   film's thickness fitted, they give its measured reflectance over the white and the black
   ground at every wavelength (Kubelka's layer, Saunderson's surface).
2. An independent check: Golden Artist Colors' published CIE L*a*b* of their Burnt Sienna and
   Raw Umber (PBr 7) acrylics, against the LBNL films seen through their surface.
3. The tints: the measured masstone K and S per volume, mixed by K-M with the measured white
   (W03) at 1:4 and 1:9 by volume, against the measured tints.
4. Against CHSOS (Kremer lots): the powders (floor 0.022) dry and the linseed-oil swatches.
5. Lead white's S per volume, the scale an earth's tint with lead white rests on: what a
   measurement-anchored route gives, against the rule's Mie at 1 um.
6. The K per volume against hematite's own (k 0.0925 at 550 nm).
7. Tints 1:1 by volume with lead white, before and after, and the Rust tables.
"""
import ast
import csv
import hashlib
import json
import re
import sys
from pathlib import Path

import numpy as np
import pdfplumber
from scipy.optimize import least_squares

FILES = {
    "B13": ("B13-tint-ladder-spectral-chart.pdf", "24183cf7b77ff6cfd48b56fd2e950c9c8a9f658c69da4ec55e0e534a102bfc07"),
    "B14": ("B14-tint-ladder-spectral-chart.pdf", "c4450c8e019f2e03f0dc19137a90e18d6331493f19e70f344c004dad1531f796"),
    "B15": ("B15-tint-ladder-spectral-chart.pdf", "67ab42c0aa9669fb63669e5dc9ff3d7fcaae9f08154ed6337cd4d5fd6d4f3123"),
    "R04": ("R04-tint-ladder-spectral-chart.pdf", "57f5c3e86bb7b1f111095f7586ef3006c57f0c63c46bd60e7ba53d17f805d7ad"),
    "Y01": ("Y01-tint-ladder-spectral-chart.pdf", "afecfa812461db7d722589b98d926adba62be2f9467b463553de480421af4e62"),
    "W01": ("W01-masstone-spectral-chart.pdf", "823ce5363d9bc2319dbbaa1c59803bffe5ccfd4f5bbfe03fc6f63be5fc8e82d1"),
    "W02": ("W02-masstone-spectral-chart.pdf", "0b7e7b5340f21b2cec5290ac0e9668a30f729be222feb7d889ee81d9d90c03b8"),
    "W03": ("W03-masstone-spectral-chart.pdf", "85cb029b51af1528a242348520de16b6a7f80050a49f7cbd2d0bb9225145bd59"),
    "W04": ("W04-masstone-spectral-chart.pdf", "3a70644306f9bbec813b28b95f1b0524d521eebd059f994eac6ccae4e7992cce"),
}
# dry-film pigment volume concentrations, as the database's pages and charts print them
PVC = {"B13": 0.134, "B14": 0.201, "B15": 0.109, "R04": 0.13, "Y01": 0.16,
       "W01": 0.15, "W02": 0.09, "W03": 0.228, "W04": 0.02}
TUBES = {"burnt sienna": "B13", "raw sienna": "B14", "raw umber": "B15", "Mars red": "R04", "Mars yellow": "Y01"}
CHSOS = {"burnt sienna": "burntsienna", "raw sienna": "rawsienna", "raw umber": "rawumber"}
# Mie's air/oil ratio of S: the earths' at their measured grain size (earth_scatter_lbnl.py
# step 5, dry.rs `earth_grains`), the Mars colours the dry rule's at 1 um (dry.rs `mie_air`)
RATIO_MEASURED = {"burnt sienna": 1.669, "raw sienna": 2.349, "raw umber": 1.936}
SHA256_CSV = {"powder.csv": "86bb05f002d64ad5aa0a1f66c7226f18b62824e77d3aa3e4e2a2aee3f0efae45"}

root, pdf_dir, csv_dir, tubes_path = (Path(a) for a in sys.argv[1:5])
for code, (fn, sha) in FILES.items():
    digest = hashlib.sha256((pdf_dir / fn).read_bytes()).hexdigest()
    if digest != sha:
        sys.exit(f"{pdf_dir / fn}: not the LBNL chart this was made from")
for fn, sha in SHA256_CSV.items():
    if hashlib.sha256((csv_dir / fn).read_bytes()).hexdigest() != sha:
        sys.exit(f"{csv_dir / fn}: not the CHSOS 2014 file this was made from")
tubes = json.loads(tubes_path.read_text())
src = (root / "crates/paint/src/spectral.rs").read_text()


def table(name):
    v = src.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", v).strip()))


CMF = table("CMF")
TO_RGB = table("XYZ_RGB") @ CMF
NM = 380 + 10 * np.arange(38)
WHITE_XYZ = CMF.sum(axis=1)
FLAT = (NM >= 550) & (NM <= 700)
I = {w: list(NM).index(w) for w in (450, 500, 550, 600, 650, 700)}


def lab(r):
    x = CMF @ r / WHITE_XYZ
    f = np.where(x > 0.008856, np.cbrt(x), 7.787 * x + 16 / 116)
    return np.array([116 * f[1] - 16, 500 * (f[0] - f[1]), 200 * (f[1] - f[2])])


def srgb8(r):
    lin = np.clip(TO_RGB @ r, 0, 1)
    return np.round(np.where(lin <= 0.0031308, 12.92 * lin, 1.055 * lin ** (1 / 2.4) - 0.055) * 255).astype(int)


def fmt(r):
    return f"{' '.join(f'{v:3d}' for v in srgb8(r))} ({'/'.join(f'{v:.1f}' for v in lab(r))})"


def de(a, b):
    return float(np.linalg.norm(lab(a) - lab(b)))


def km(q):
    q = np.maximum(q, 0)
    return 1 + q - np.sqrt(q * (q + 2))


def saunderson(ri, k1=0.04, k2=0.6):
    return k1 + (1 - k1) * (1 - k2) * ri / (1 - k2 * ri)


def saunderson_inverse(rm, k1=0.04, k2=0.6):
    x = np.maximum((rm - k1) / (1 - k1), 1e-5)
    return x / ((1 - k2) + k2 * x)


# ------------------------------------------------------------ the charts read back
# (as earth_scatter_lbnl.py: up to four columns of three panels; panel II's decades at
# y 282.73 (0.1) ... 189.4 (1000); panels I and III 0 at 138.73 and 426.73, 1 at 45.4, 333.4)
COLUMNS, WIDTH = (43.2, 194.4, 345.6, 496.8), 133.2
COLOURS = {(0.627, 0.125, 0.941): "K", (1.0, 0.843, 0.0): "S", (0.0, 0.0, 1.0): "T_void",
           (1.0, 0.647, 0.0): "R_white", (0.0, 0.0, 0.0): "R_black"}


def read_chart(path):
    page = pdfplumber.open(path).pages[0]
    cols = [{} for _ in COLUMNS]
    for o in page.curves:
        pts = o["pts"]
        name = COLOURS.get(tuple(round(v, 3) for v in o["stroking_color"]))
        if len(pts) < 100 or not name or (o.get("dash") and o["dash"][0]):
            continue
        c = max(i for i, x0 in enumerate(COLUMNS) if min(q[0] for q in pts) >= x0 - 1)
        x = np.array([300 + (q[0] - COLUMNS[c]) / WIDTH * 2200 for q in pts])
        y = np.array([q[1] for q in pts])
        v = 10 ** (-1 + (282.73 - y) / 23.333) if name in ("K", "S") else (
            (138.73 - y) / 93.33 if name == "T_void" else (426.73 - y) / 93.33)
        cols[c].setdefault(name, []).extend(zip(x, v))
    return [{k: (lambda a: np.interp(NM, a[:, 0], a[:, 1]))(np.array(sorted(v))) for k, v in col.items()}
            for col in cols if col]


charts = {code: read_chart(pdf_dir / fn) for code, (fn, _) in FILES.items()}
KV = {name: charts[code][0]["K"] / PVC[code] / 1000 for name, code in TUBES.items()}
SV = {name: charts[code][0]["S"] / PVC[code] / 1000 for name, code in TUBES.items()}



def km_layer(K, S, h, rg):
    """Kubelka's internal reflectance of a layer (K, S per mm, h mm) over a ground rg."""
    a = 1 + K / S
    b = np.sqrt(a * a - 1)
    coth = 1 / np.tanh(np.maximum(b * S * h, 1e-9))
    return (1 - rg * (a - b * coth)) / (a - rg + b * coth)


print("\n1. each masstone's read K and S against its own measured reflectance over the white and the")
print("   black ground (420-700 nm; the film's thickness and the two grounds' reflectance fitted)")
for name, code in TUBES.items():
    c = charts[code][0]
    m = (NM >= 420) & (NM <= 700)

    def res(p):
        h, gw, gb = p
        return np.concatenate([saunderson(km_layer(c["K"][m], c["S"][m], h, gw)) - c["R_white"][m],
                               saunderson(km_layer(c["K"][m], c["S"][m], h, gb)) - c["R_black"][m]])
    r = least_squares(res, [0.03, 0.8, 0.05], bounds=([1e-4, 0.3, 0.0], [2, 1, 0.3]))
    print(f"  {name:12s} ({code}, PVC {PVC[code]:.1%}): film {r.x[0] * 1000:.0f} um, rms {np.sqrt(np.mean(r.fun ** 2)):.3f}, "
          f"largest {np.abs(r.fun).max():.3f}; R-inf (internal) {fmt(km(c['K'] / c['S']))}")

print("\n   per pigment volume (um^-1) at 450 / 500 / 550 / 600 / 650 / 700 nm:")
for name in TUBES:
    print(f"  {name:12s} K {' / '.join(f'{KV[name][I[w]]:.3f}' for w in I)}")
    print(f"  {'':12s} S {' / '.join(f'{SV[name][I[w]]:.3f}' for w in I)}")

# ------------------------------------------------------------------- 2. Golden
print("\n2. Golden Artist Colors' published masstones (CIE L*a*b*, acrylic, PBr 7), against the LBNL film")
print("   seen through its surface (Saunderson k1 0.04, k2 0.6), and the engine's oil pile as #351 has it")
GOLDEN = {"burnt sienna": (33.96, 17.64, 14.17), "raw umber": (28.2, 1.45, 3.33)}
WHITE_S = np.array(tubes["lead white"]["s"])
WHITE_K = np.array(tubes["lead white"]["k"])
WHITE_S_VOL = 0.75 * 0.88 / 1.9           # lead white's S per volume in oil on the dry rule's scale (§9.4)
POWDER_K_351 = {"burnt sienna": 16.79, "raw sienna": 15.19, "raw umber": 5.620}
S_351 = {"burnt sienna": 0.1845, "raw sienna": 0.3817, "raw umber": 0.0993}


def oil_351(name):
    t = tubes[name]
    return np.array(t["k"]) * POWDER_K_351[name], WHITE_S * S_351[name] / WHITE_S_VOL


for name, g in GOLDEN.items():
    lb = saunderson(km(KV[name] / SV[name]))
    k, s = oil_351(name)
    old = saunderson(km(k / s))
    print(f"  {name:12s} Golden {g}; LBNL seen {fmt(lb)} dE76 {np.linalg.norm(lab(lb) - g):.1f}; "
          f"#351 seen {fmt(old)} dE76 {np.linalg.norm(lab(old) - g):.1f}")

# -------------------------------------------------------------------- 3. tints
print("\n3. tints by K-M with the measured white W03 (by volume of paint), against the measured tints'")
print("   R-infinity (internal), dE76 at 1:4 / 1:9")
W_K, W_S = charts["B13"][3]["K"], charts["B13"][3]["S"]
for name, code in TUBES.items():
    se, ke = SV[name] * PVC[code] * 1000, KV[name] * PVC[code] * 1000
    row = []
    for r, c in ((4, 1), (9, 2)):
        meas = km(charts[code][c]["K"] / charts[code][c]["S"])
        got = km((ke + r * W_K) / (se + r * W_S))
        row.append(f"1:{r} {fmt(got)} measured {fmt(meas)} dE {de(got, meas):.1f}")
    print(f"  {name:12s} " + "  ".join(row))

# -------------------------------------------------------------------- 4. CHSOS
FLOOR, OIL_FLOOR = 0.022, 0.04


def loader(fn):
    rows = list(csv.reader((csv_dir / fn).open()))
    head = [h.strip() for h in rows[0]]
    data = np.array([[float(x) for x in r] for r in rows[1:] if len(r) == len(head) and all(x.strip() for x in r)])

    def get(name):
        nm, col = data[:, 0], head.index(name)
        return np.array([data[(nm >= c - 5) & (nm < c + 5), col].mean() for c in NM]) / 100
    return get


def correction(get, blacks, floor_true):
    black = np.mean([get(n) for n in blacks], axis=0)
    lead, ti = get("leadwhite"), get("titaniumwhite")

    def corrected(name, black_true=floor_true, ti_true=0.945):
        lead_true = black_true + (ti_true - black_true) * (lead - black)[FLAT].mean() / (ti - black)[FLAT].mean()
        return black_true + (get(name) - black) * (lead_true - black_true) / (lead - black)
    return corrected


powder = correction(loader("powder.csv"), ("ivoryblack", "vineblack", "boneblack", "lampblack"), FLOOR)
oil_swatch = correction(loader("linseed oil.csv"), ("ivoryblack", "vineblack", "boneblack"), OIL_FLOOR)
MIE = [(1.50, 0.68, 700.0), (1.56, 0.71, 43.0), (1.65, 0.73, 9.0), (1.70, 0.80, 5.8), (1.74, 0.75, 3.9),
       (1.84, 0.82, 2.5), (2.00, 0.88, 1.9), (2.15, 0.91, 1.7), (2.30, 0.89, 1.6), (2.40, 1.04, 1.8),
       (2.45, 0.95, 1.5), (3.00, 1.03, 1.4)]


def mie_ratio(n):
    for a, b in zip(MIE, MIE[1:]):
        if n <= b[0]:
            u = (n - a[0]) / (b[0] - a[0])
            return float(np.exp(np.log(a[2]) + (np.log(b[2]) - np.log(a[2])) * u))
    return MIE[-1][2]


RATIO = {name: RATIO_MEASURED.get(name, mie_ratio(tubes[name]["n"])) for name in TUBES}


def dry_now(name):
    """The dry rule without the switches (dry.rs `spectral_color`), or #351's switch for the earths."""
    if name in POWDER_K_351:
        k, s = oil_351(name)
        return km(k / (s * RATIO_MEASURED[name]))
    t = tubes[name]
    return km(np.array(t["k"]) / (mie_ratio(t["n"]) * np.array(t["s"])))


def oil_now(name):
    if name in POWDER_K_351:
        k, s = oil_351(name)
        return km(k / s)
    t = tubes[name]
    return km(np.array(t["k"]) / np.array(t["s"]))


print("\n4. pure, R-infinity (internal): #351 -> the LBNL film's K and S; dry with S x the air/oil ratio")
print("   (CHSOS's powders are Kremer's lots, corrected with the floor 0.022; its linseed-oil swatches less")
print("   the glossy surface, Saunderson inverted; the Mars colours have no CHSOS sample)")
for name in TUBES:
    oil_new, dry_new = km(KV[name] / SV[name]), km(KV[name] / (SV[name] * RATIO[name]))
    print(f"  {name:12s} oil {fmt(oil_now(name))} -> {fmt(oil_new)}")
    print(f"  {'':12s} dry {fmt(dry_now(name))} -> {fmt(dry_new)}   (air/oil ratio {RATIO[name]:.3f})")
    if name in CHSOS:
        p, o = powder(CHSOS[name]), saunderson_inverse(oil_swatch(CHSOS[name]))
        print(f"  {'':12s} CHSOS powder {fmt(p)}: dE76 {de(dry_now(name), p):.1f} -> {de(dry_new, p):.1f};  "
              f"linseed swatch {fmt(o)}: dE76 {de(oil_now(name), o):.1f} -> {de(oil_new, o):.1f}")
print("   the engine's oil pile (gloss excluded) against CHSOS's swatches less their 4 % specular, for the")
print("   paint test, and the LBNL films so:")
for name in CHSOS:
    print(f"  {name:12s} CHSOS {fmt(oil_swatch(CHSOS[name]) - 0.04)}; LBNL {fmt(saunderson(km(KV[name] / SV[name])) - 0.04)}")
for name in ("Mars red", "Mars yellow"):
    print(f"  {name:12s} LBNL {fmt(saunderson(km(KV[name] / SV[name])) - 0.04)}")

# ------------------------------------------------------------ 5. lead white's S
print("\n5. lead white's S per volume in oil (um^-1, at 550 nm), the scale of every tint with lead white")
print("   No direct measurement of lead white films was found open (Mitton & Vejnoska measured white lead's")
print("   S in acrylic at seven concentrations, Official Digest 1961; not accessible). A measurement-anchored")
print("   route: white lead's tinting strength is 1/18.5 of rutile's by weight (Shreve, Chemical Process")
print("   Industries; 'comparative tinting strengths of white pigments in a standard blue'), so per volume")
print("   S_lead = S_rutile x (rho_lead / rho_rutile) / 18.5, rho 6.8 (hydrocerussite) and 4.1 (pigment rutile).")
for code in ("W04", "W02", "W01", "W03"):
    s = charts[code][0]["S"][I[550]] / PVC[code] / 1000
    print(f"  rutile {code} (PVC {PVC[code]:4.1%}): S {s:.2f}  ->  lead white {s * 6.8 / 4.1 / 18.5:.3f}")
print(f"  the dry rule's Mie at 1 um: {WHITE_S_VOL:.3f}; at Kremer's Cremnitz white's measured 3 um (0.1-10): 0.50 or")
print("  less (dry_pigment_optics.md §9.5). The tinting pastes' crowding is not stated: a rutile paste at its")
print("  oil absorption is more crowded than W03, which lowers the lead white figure further.")
print(f"  So the measurement-anchored values bracket the rule's {WHITE_S_VOL:.3f}; it stays.")

# --------------------------------------------------------------- 6. hematite
K_HEMATITE = 2 * 4 * np.pi * 0.0925 / 0.55
print(f"\n6. K per volume at 550 nm against a pure hematite bed's {K_HEMATITE:.2f} um^-1 (k 0.0925, Longtin et al. 1988):")
for name in TUBES:
    print(f"  {name:12s} {KV[name][I[550]]:.3f} ({KV[name][I[550]] / K_HEMATITE:.0%})")

# ------------------------------------------------------------- 7. tints, tables
Y = CMF[1]
U = float((Y * WHITE_S).sum() / Y.sum()) / WHITE_S_VOL      # the engine's units per um^-1 (per coat)
print(f"\n7. lead white's engine S per coat, luminance-weighted mean {U * WHITE_S_VOL:.4f}: {U:.4f} per um^-1")
print("   tints 1:1 by volume with lead white, R-inf, #351 -> now: oil; dry")
for name in TUBES:
    k_new, s_new = KV[name] * U, SV[name] * U
    if name in POWDER_K_351:
        k_old, s_old = oil_351(name)
        r_old = RATIO_MEASURED[name]
    else:
        t = tubes[name]
        k_old, s_old, r_old = np.array(t["k"]), np.array(t["s"]), mie_ratio(t["n"])
    lw_dry = WHITE_S * 1.9
    print(f"  {name:12s} oil {fmt(km((k_old + WHITE_K) / (s_old + WHITE_S)))} -> {fmt(km((k_new + WHITE_K) / (s_new + WHITE_S)))}")
    print(f"  {'':12s} dry {fmt(km((k_old + WHITE_K) / (s_old * r_old + lw_dry)))} -> "
          f"{fmt(km((k_new + WHITE_K) / (s_new * RATIO[name] + lw_dry)))}")


def rust(v):
    return "[" + ", ".join(f"{x:.5}" for x in v) + "]"


print("\nfor dry.rs LBNL_FILMS (K and S per pigment volume in oil, um^-1, 380-750 nm in 10 nm steps; air/oil ratio):")
for name in TUBES:
    print(f'    ("{name}", {rust(KV[name])},\n        {rust(SV[name])}, {RATIO[name]:.4}),')


print("\nfor the paint tests:")
print("  the films as the engine shows a pile (gloss excluded: seen through the surface, less its 4 %), and dry")
for name in TUBES:
    print(f"    {name:12s} oil {np.round(lab(saunderson(km(KV[name] / SV[name])) - 0.04), 1)}  dry "
          f"{np.round(lab(km(KV[name] / (SV[name] * RATIO[name]))), 1)}")
# Golden's L*a*b* (seen, the specular included) less the 4 % specular, in XYZ
gl = np.array(GOLDEN["burnt sienna"])
fy = (gl[0] + 16) / 116
f = np.array([fy + gl[1] / 500, fy, fy - gl[2] / 200])
xyz = np.where(f ** 3 > 0.008856, f ** 3, (f - 16 / 116) / 7.787) * (WHITE_XYZ / WHITE_XYZ[1])
xyz = xyz - 0.04 * WHITE_XYZ / WHITE_XYZ[1]
x = xyz / (WHITE_XYZ / WHITE_XYZ[1])
ff = np.where(x > 0.008856, np.cbrt(x), 7.787 * x + 16 / 116)
print(f"  Golden's burnt sienna less its 4 % specular: {np.round([116 * ff[1] - 16, 500 * (ff[0] - ff[1]), 200 * (ff[1] - ff[2])], 1)}")
print("  the measured tints with W03 at 550 / 600 / 650 nm: (lambda, white K, white S (1/mm), R-inf 1:4, 1:9),")
print("  the films' PVC, and the largest |K-M - measured| R-inf over the six")
for name, code in TUBES.items():
    rows, worst = [], 0.0
    for w in (550, 600, 650):
        i = I[w]
        meas = [float(km(charts[code][c]["K"][i] / charts[code][c]["S"][i])) for c in (1, 2)]
        se, ke = SV[name][i] * PVC[code] * 1000, KV[name][i] * PVC[code] * 1000
        for r, m in zip((4, 9), meas):
            worst = max(worst, abs(float(km((ke + r * W_K[i]) / (se + r * W_S[i]))) - m))
        rows.append(f"({w}, {W_K[i]:.4f}, {W_S[i]:.1f}, {meas[0]:.4f}, {meas[1]:.4f})")
    print(f"    {name:12s} PVC {PVC[code]}: [{', '.join(rows)}]  worst {worst:.3f}")
