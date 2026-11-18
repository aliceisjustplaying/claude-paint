#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2"]
# ///
"""The numbers behind dry_pigment_optics.md §8: the dry blacks (dry.rs `carbon_bed`),
the oil earths (mixer.rs `OIL_EARTHS`, the `--@ oil-earths` switch) and the checks
of ultramarine, the other blues, the cobalts and cadmiums, and the earths' shapes.

Inputs (as dry_earth_fit.py):
  CSV/        the CHSOS download of Cosentino 2014 (powder.csv, linseed oil.csv and
              gum arabic refletion probe.csv; SHA-256 checked below)
  tubes.json  each tube's engine-9 dry-path K and S per coat and its index:
              DRY_EARTH_FIT_OUT=tubes.json cargo test -p paint --lib -- --ignored --exact \\
                  palette::tests::dump_dry_earth_fit_input

  uv run notes/research/dry_colours_fit.py <repo root> <CSV dir> <tubes.json>

Steps:
1. The blacks. A carbon-bearing grain is opaque (carbon absorbs within ~0.1 um), so all
   it scatters is what its facets reflect. Its bed's K/S is Mie's for the grain's complex
   index in air (dry_pigment_mie.py, as charcoal_optics.py does for charcoal), whatever
   the oil table's S. Vine black: charred wood (Jager et al. 1998's cellulose chars, the
   500 C mean), 3 um. Bone black: hydroxyapatite with 10 wt% carbon (CAMEO), the carbon
   spread through the grain (Bruggeman), grains 3-15 um (Rublev's bone black; median 7).
2. The black floor of the powders' correction. dry_earth_fit.py takes the four carbon
   blacks' mean as 0.04 (0.03-0.05), from §3's veil estimate. An absolute measurement of
   charcoal powder (Lethuillier et al. 2022, MNRAS, doi:10.1093/mnras/stac1734,
   arXiv:2110.05832, table 4: sieved juniper charcoal, albedo 3.00 % at 550 nm from its
   reflectance factor, normal illumination) and step 1's physics both put it lower.
3. Ultramarine: CHSOS's powder is natural ultramarine (Kremer 10510, Cosentino 2015,
   IJCS 6:287, table 1); the engine's tube is synthetic. Their K/S shapes, against the
   gum-arabic swatch measured with the 45/45 probe.
4. The other blues, the cobalts and cadmiums against their powders, and what a grain
   size other than the rule's 1 um would do.
5. The earths' shape errors: the causes tried.
6. The oil earths: the same K per volume as dry (dry::powder_k times the table's), over
   the scattering those grains have in oil (Mie: the dry rule's S over the air/oil ratio
   at the tube's index), against CHSOS's linseed-oil swatches and Golden's published
   masstones.
"""
import ast
import csv
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path

import numpy as np

SHA256 = {
    "powder.csv": "86bb05f002d64ad5aa0a1f66c7226f18b62824e77d3aa3e4e2a2aee3f0efae45",
}
root, csv_dir, tubes_path = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
for name, sha in SHA256.items():
    if hashlib.sha256((csv_dir / name).read_bytes()).hexdigest() != sha:
        sys.exit(f"{csv_dir / name}: not the CHSOS 2014 file this fit was made from")
tubes = json.loads(tubes_path.read_text())
spec = importlib.util.spec_from_file_location("dry_pigment_mie", root / "notes/research/dry_pigment_mie.py")
mie_mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mie_mod)

src = (root / "crates/paint/src/spectral.rs").read_text()


def table(name):
    v = src.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", v).strip()))


CMF = table("CMF")
TO_RGB = table("XYZ_RGB") @ CMF
NM = 380 + 10 * np.arange(38)
FLAT = (NM >= 550) & (NM <= 700)
WHITE_XYZ = CMF.sum(axis=1)
I550 = list(NM).index(550)


def km(q):
    q = np.maximum(q, 0)
    return 1 + q - np.sqrt(q * (q + 2))


def ks_of(r):
    r = np.clip(r, 1e-5, 1)
    return (1 - r) ** 2 / (2 * r)


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


def loader(fn):
    rows = list(csv.reader((csv_dir / fn).open()))
    head = [h.strip() for h in rows[0]]
    data = np.array([[float(x) for x in r] for r in rows[1:] if len(r) == len(head) and all(x.strip() for x in r)])

    def get(name):
        nm, col = data[:, 0], head.index(name)
        return np.array([data[(nm >= c - 5) & (nm < c + 5), col].mean() for c in NM]) / 100
    return get


POWDER = loader("powder.csv")
OIL = loader("linseed oil.csv")
PROBE = loader("gum arabic refletion probe.csv")


def correction(get, blacks):
    black = np.mean([get(n) for n in blacks], axis=0)
    lead, ti = get("leadwhite"), get("titaniumwhite")

    def corrected(name, black_true=0.04, ti_true=0.945):
        lead_true = black_true + (ti_true - black_true) * (lead - black)[FLAT].mean() / (ti - black)[FLAT].mean()
        return black_true + (get(name) - black) * (lead_true - black_true) / (lead - black)
    return corrected, black


powder, BLACK = correction(POWDER, ("ivoryblack", "vineblack", "boneblack", "lampblack"))
# CHSOS's linseed-oil lamp black column is misnamed "lampblaxk"; its Van Dyke brown is a
# second wavelength column. The oil blacks' floor is taken from the three named ones.
oil_swatch, _ = correction(OIL, ("ivoryblack", "vineblack", "boneblack"))


def saunderson_inverse(rm, k1=0.04, k2=0.6):
    """A glossy oil film measured with the specular included (the sphere): its internal R."""
    x = np.maximum((rm - k1) / (1 - k1), 1e-5)
    return x / ((1 - k2) + k2 * x)


def saunderson(ri, k1=0.04, k2=0.6):
    return k1 + (1 - k1) * (1 - k2) * ri / (1 - k2 * ri)


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
            return a[1] + (b[1] - a[1]) * u, float(np.exp(np.log(a[2]) + (np.log(b[2]) - np.log(a[2])) * u))
    return MIE[-1][1:]


WHITE_S = np.array(tubes["lead white"]["s"])
SIGMA_WHITE, RATIO_WHITE = mie(2.0)


def rule_s(n):
    return RATIO_WHITE * WHITE_S * mie(n)[0] / SIGMA_WHITE


def engine_dry(name):
    """The dry rule as it is without the switches (dry.rs `spectral_color`)."""
    t = tubes[name]
    k, s, n = np.array(t["k"]), np.array(t["s"]), t["n"]
    if n < 0:
        return km(k / np.maximum(-n * s, 1e-12))
    sigma, ratio = mie(n)
    return km(k / (ratio * s if n >= 1.9 else rule_s(n)))


POWDER_K = {"burnt sienna": 31.71, "red earth": 7.718, "raw sienna": 10.08, "yellow ochre": 2.087,
            "raw umber": 16.49, "viridian": 3.910}
CHSOS = {"burnt sienna": "burntsienna", "red earth": "redochre", "raw sienna": "rawsienna",
         "yellow ochre": "yellowochre", "raw umber": "rawumber", "viridian": "viridian"}


def switched_dry(name):
    """With #351's dry-earths: K times powder_k over the Mie S at the tube's index."""
    t = tubes[name]
    return km(np.array(t["k"]) * POWDER_K[name] / rule_s(t["n"]))


# ---------------------------------------------------------------- 1. the blacks
CEL = {400: [(0.6310, 1.573, 0.4157), (0.5643, 1.524, 0.4663), (0.4990, 1.452, 0.5126), (0.4467, 1.375, 0.5422)],
       600: [(0.6310, 1.345, 0.7807), (0.5623, 1.224, 0.7793), (0.5012, 1.105, 0.7526), (0.4467, 0.9954, 0.7020)]}


def char(lam, t=500.0):
    def at(rows):
        rows = sorted(rows)
        ls = [r[0] for r in rows]
        return complex(np.interp(lam, ls, [r[1] for r in rows]), np.interp(lam, ls, [r[2] for r in rows]))
    u = (t - 400.0) / 200.0
    return (1 - u) * at(CEL[400]) + u * at(CEL[600])


def bruggeman(comps):
    eps = [m * m for m, _ in comps]
    fs = [f for _, f in comps]
    e = sum(f * x for x, f in zip(eps, fs))
    for _ in range(200):
        F = sum(f * (x - e) / (x + 2 * e) for x, f in zip(eps, fs))
        dF = sum(f * (-3 * x) / (x + 2 * e) ** 2 for x, f in zip(eps, fs))
        e -= F / dF
    return np.sqrt(e)


HAP_N, HAP_RHO = 1.64, 3.16      # hydroxyapatite (CAMEO bone black 1.65-1.70 for the grains; HAp 3.16 g/cm3)
CARBONATE_RHO = 2.7
CARBON_RHO = 1.8                 # a coke's skeletal density, 1.5-2.0 [E]


def bone_index(lam, carbon_wt=0.10, t=500.0):
    vc, va, vo = carbon_wt / CARBON_RHO, (0.84 - (carbon_wt - 0.10)) / HAP_RHO, 0.06 / CARBONATE_RHO
    fc = vc / (vc + va + vo)
    return bruggeman([(char(lam, t), fc), (complex(HAP_N, 0), 1 - fc)])


def bed(m, d, lam=0.55):
    s, a = mie_mod.sizeavg(m, 1.0, d, lam=lam)
    return 2 * a, 0.75 * s     # K, S per unit particle volume (um^-1), dry_pigment_optics.md §2.1


def black_ratio(index, d):
    k, s = bed(index(0.55), d)
    return s / k, float(km(np.array(k / s)))


print("1. the dry blacks: S/K of the grains in air (Mie, 550 nm), their R-infinity, against the powders")
BLACKS = {"vine black": (lambda lam: char(lam), 3.0, "vineblack"),
          "bone black": (lambda lam: bone_index(lam), 7.0, "boneblack")}
RATIO = {}
for name, (index, d, col) in BLACKS.items():
    ratio, r = black_ratio(index, d)
    RATIO[name] = ratio
    print(f"  {name:11s} m {index(0.55):.3f}, median {d:.0f} um: S/K {ratio:.5f}, R-inf {r:.4f}"
          f" (L* {lab(np.full(38, r))[0]:.1f}); engine dry now {fmt(engine_dry(name))}")
print("  sensitivity (R-inf): vine at 1/10 um", [round(black_ratio(BLACKS['vine black'][0], d)[1], 4) for d in (1.0, 10.0)],
      "fired 400/600 C", [round(black_ratio(lambda lam: char(lam, t), 3.0)[1], 4) for t in (400.0, 600.0)])
print("               bone at 3/15 um", [round(black_ratio(BLACKS['bone black'][0], d)[1], 4) for d in (3.0, 15.0)],
      "20 wt% carbon", round(black_ratio(lambda lam: bone_index(lam, 0.20), 7.0)[1], 4),
      "fired 400/600 C", [round(black_ratio(lambda lam: bone_index(lam, 0.10, t), 7.0)[1], 4) for t in (400.0, 600.0)])

# ------------------------------------------------- 2. the powders' black floor
print("\n2. the correction's black floor")
raw = {n: POWDER(n)[FLAT].mean() for n in ("ivoryblack", "vineblack", "boneblack", "lampblack")}
gain = (powder("leadwhite") - 0.04)[FLAT].mean() / (POWDER("leadwhite") - BLACK)[FLAT].mean()
print("  raw sphere readings 550-700 nm:", {k: round(v, 4) for k, v in raw.items()}, f"(gain to true {gain:.3f})")
print("  measured differences from the four blacks' mean (true units):",
      {k: round((v - BLACK[FLAT].mean()) * gain, 4) for k, v in raw.items()})
cophylab = 0.0300
floor = cophylab - (raw["vineblack"] - BLACK[FLAT].mean()) * gain
print(f"  if vine black is as charcoal powder measures (Lethuillier et al. 2022: 3.00 % at 550 nm), the floor is {floor:.4f}")
for bt in (0.04, floor):
    print(f"  floor {bt:.3f}: bone {fmt(powder('boneblack', bt))}, vine {fmt(powder('vineblack', bt))}, "
          f"burnt sienna {fmt(powder('burntsienna', bt))}")
for name, (_, _, col) in BLACKS.items():
    r = km(np.array(1 / RATIO[name])) * np.ones(38)
    print(f"  {name:11s} physics {fmt(r)}: dE {de(r, powder(col)):.1f} to the repo's correction (floor 0.04, L* bounds "
          f"{min(lab(powder(col, b, t))[0] for b in (0.03, 0.05) for t in (0.90, 0.99)):.1f}-"
          f"{max(lab(powder(col, b, t))[0] for b in (0.03, 0.05) for t in (0.90, 0.99)):.1f}), "
          f"{de(r, powder(col, floor)):.1f} at the charcoal floor {floor:.3f}")

# ------------------------------------------------------------ 3. ultramarine
print("\n3. ultramarine: CHSOS's powder is natural (Kremer 10510); the engine's tube is synthetic")
idx = [7, 12, 17, 22, 27, 32]
print(f"  nm {NM[idx]}")
u_pow = powder("ultramarine")
k = np.array(tubes["ultramarine blue"]["k"])
print("  K/S over its 450 nm value: natural powder", np.round((ks_of(u_pow) / ks_of(u_pow)[7])[idx], 1))
print("                             natural, gum (45/45 probe)", np.round((ks_of(PROBE("ultramarine")) / ks_of(PROBE("ultramarine"))[7])[idx], 1))
print("                             engine synthetic K", np.round((k / k[7])[idx], 1))
print(f"  engine synthetic dry {fmt(engine_dry('ultramarine blue'))}; natural powder {fmt(u_pow)}; dE {de(engine_dry('ultramarine blue'), u_pow):.1f}")
print(f"  engine 'ultramarine ash' (natural, a pale last extraction) dry {fmt(engine_dry('ultramarine ash'))}")

# --------------------------------------------- 4. the other blues, cobalts, cadmiums
print("\n4. against their powders (repo's correction): engine dry (no switch changes these)")
OTHERS = {"Prussian blue": ("prussianblue", 1.56), "cobalt blue": ("cobaltblue", 1.74), "smalt": ("smalt", 1.5),
          "cobalt violet": ("cobaltviolet", 1.68), "cadmium yellow": ("cadmiumyellow", 2.5),
          "cadmium red": ("cadmiumred", 2.6), "vermilion": ("vermilion", 3.0), "Naples yellow": ("naplesyellow", 2.15),
          "red lead": ("redlead", 2.42), "lead white": ("leadwhite", 2.0), "zinc white": ("zincwhite", 2.02)}
for name, (col, n) in OTHERS.items():
    r = engine_dry(name)
    print(f"  {name:15s} {fmt(r):30s} powder {fmt(powder(col)):30s} dE {de(r, powder(col)):5.1f}")
print("  grain size in place of the rule's 1 um (S x sigma'(d)/sigma'(1 um), K as the table's):")
for name, ds in (("smalt", (10.0, 20.0, 40.0)), ("cobalt violet", (3.0, 10.0, 20.0)), ("cobalt blue", (3.0, 10.0))):
    col, n = OTHERS[name]
    s1, _ = mie_mod.sizeavg(n, 1.0, 1.0)
    t = tubes[name]
    row = []
    for d in ds:
        sd, _ = mie_mod.sizeavg(n, 1.0, d, N=21)
        row.append(f"{d:.0f} um dE {de(km(np.array(t['k']) / (rule_s(n) * sd / s1)), powder(col)):.1f}")
    print(f"    {name:13s} " + ", ".join(row))

# ------------------------------------------------------- 5. the earths' shapes
print("\n5. the earths' shapes (with dry-earths): what one more physical term does")
for name in ("red earth", "yellow ochre", "burnt sienna"):
    r = switched_dry(name)
    p = powder(CHSOS[name])
    flat = 0.04 + 0.96 * r
    print(f"  {name:12s} switch {fmt(r)} dE {de(r, p):.1f}; with a flat 4 % first surface {fmt(flat)} dE {de(flat, p):.1f}")
print("  (grain size moves S at all wavelengths alike, so the one factor takes it up: no change of shape)")
print("  yellow ochre 600-700 nm: switch", np.round(switched_dry("yellow ochre")[[22, 27, 32]], 3),
      "powder", np.round(powder("yellowochre")[[22, 27, 32]], 3),
      "- K there is the thin swatch's, near nothing")

# ---------------------------------------------------------- 6. the oil earths
print("\n6. the oil earths: one K per volume (the table's x powder_k), oil S = the dry rule's Mie S / the air-oil ratio")
print("   R-inf (internal) and, as a sphere sees a glossy film, Saunderson (k1 0.04, k2 0.6)")
SF = {}
for name, a in POWDER_K.items():
    t = tubes[name]
    n = t["n"]
    sigma, ratio = mie(n)
    s_oil = rule_s(n) / ratio
    SF[name] = RATIO_WHITE * sigma / SIGMA_WHITE / ratio
    old = km(np.array(t["k"]) / np.array(t["s"]))
    new = km(np.array(t["k"]) * a / s_oil)
    meas = saunderson_inverse(oil_swatch(CHSOS[name]))
    print(f"  {name:12s} old {fmt(old):28s} new {fmt(new):28s} CHSOS oil {fmt(meas):28s} dE {de(old, meas):.1f} -> {de(new, meas):.1f}")
    print(f"  {'':12s} seen with the specular: new {fmt(saunderson(new))}, CHSOS {fmt(oil_swatch(CHSOS[name]))}")
print("  Golden Artist Colors (CIE L*a*b*, acrylic): burnt sienna 33.96/17.64/14.17, raw umber 28.2/1.45/3.33")
print("  the engine shows a pile with the gloss excluded (mixer.rs `outside`); CHSOS's swatches so (the sphere's")
print("  reading less the film's 4 % specular), for the paint test:")
for name in POWDER_K:
    print(f"    {name:12s} {fmt(oil_swatch(CHSOS[name]) - 0.04)}")
print("  tints 1:1 by volume with lead white, R-inf in oil (not measured: no earth tint was found).")
print("  They rest on the grains' 1 um (§7.3): grains of 3 um scatter ~0.27x as much, and the same")
print("  masstone then needs ~0.27x the K, so the tint is lighter:")
WHITE_K = np.array(tubes["lead white"]["k"])
for name in ("burnt sienna", "raw umber", "red earth"):
    t = tubes[name]
    n = t["n"]
    sigma, ratio = mie(n)
    s1, _ = mie_mod.sizeavg(n, 1.0, 1.0)
    s3, _ = mie_mod.sizeavg(n, 1.0, 3.0)
    row = []
    for size in (1.0, s3 / s1):
        k = np.array(t["k"]) * POWDER_K[name] * size
        s_oil = rule_s(n) * size / ratio
        row.append(fmt(km((k + WHITE_K) / (s_oil + WHITE_S))))
    print(f"    {name:12s} 1 um {row[0]}   3 um {row[1]}")

print("\nfor mixer.rs OIL_EARTHS (tube, K factor, oil S as a multiple of lead white's oil S):")
for name, a in POWDER_K.items():
    print(f'    ("{name}", {a}, {SF[name]:.5f}),')
print("for dry.rs carbon_bed (S/K of the grains in air):")
for name, r in RATIO.items():
    print(f'        "{name}" => Some({r:.5f}),')
