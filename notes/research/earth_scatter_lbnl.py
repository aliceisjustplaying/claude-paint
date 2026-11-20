#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==2.4.2", "pdfplumber==0.11.4"]
# ///
"""The earths' scattering per pigment volume behind paint's `dry::earth_scatter` (the
`--@ dry-earths` and `--@ oil-earths` switches) and the checks in dry_pigment_optics.md §9.

The earths' tints with white rest on how much their grains scatter per volume. #351 took
that from Mie at an estimated 1 um. This script reads it from measurements:

  LBNL Pigment Database (Levinson, Berdahl & Akbari, Solar Energy Materials & Solar Cells
  89 (2005) 319-349 and 351-389): Liquitex acrylic films of burnt sienna (B13), raw sienna
  (B14) and raw umber (B15), all natural iron oxide PBr 7, as masstones and as 1:4 and 1:9
  tints by volume with titanium white (W03), each measured over a void, a white and a black
  ground, and inverted to Kubelka-Munk K and S (1/mm) with the films' measured thickness.
  The database's own datafiles are encrypted; its spectral charts (vector PDFs) are not, so
  the curves are read back from the PDFs' drawing paths, calibrated on the charts' own tick
  marks. Two synthetic hematites of known size (R02 0.3 um, R03 0.27 um) check the step from
  Mie to an absolute K-M S.

Inputs:
  a directory with the six chart PDFs (SHA-256 checked below), from
  https://coolcolors.lbl.gov/LBNL-Pigment-Database/assets/spectral-charts/pdf/ :
  B13-tint-ladder-spectral-chart.pdf, B14-tint-ladder-spectral-chart.pdf,
  B15-tint-ladder-spectral-chart.pdf, R02-masstone-spectral-chart.pdf,
  R03-tint-ladder-spectral-chart.pdf, W03-masstone-spectral-chart.pdf

  uv run notes/research/earth_scatter_lbnl.py <repo root> <pdf dir>

Steps:
1. Read the curves back; check them against the films' own reflectance over black.
2. Check Mie (dry_pigment_mie.py, 3/4 sigma'(1-g) per volume) against the two hematites.
3. The earths' S per pigment volume, 550-700 nm (where K does not swamp S), in a binder of
   n 1.49 (fresh linseed oil 1.48).
4. The tints: the measured masstone's K/S with each candidate S per volume, mixed by volume
   with the measured white, against the measured tints. Candidates: the measured S; Mie at
   #351's 1 um; Mie at the laser-diffraction sizes measured for Kremer's earths (Reichert et
   al. 2025, Anal. Bioanal. Chem. 417:4351, table 3: burnt umber 40710 6.4 um, range
   0.2-42; yellow ochre 116421 10.5 um, 0.2-76).
5. The Mie air/oil ratio at those measured sizes (the dry S over the oil S).
"""
import ast
import hashlib
import importlib.util
import re
import sys
from pathlib import Path

import numpy as np
import pdfplumber

FILES = {
    "B13": ("B13-tint-ladder-spectral-chart.pdf", "24183cf7b77ff6cfd48b56fd2e950c9c8a9f658c69da4ec55e0e534a102bfc07"),
    "B14": ("B14-tint-ladder-spectral-chart.pdf", "c4450c8e019f2e03f0dc19137a90e18d6331493f19e70f344c004dad1531f796"),
    "B15": ("B15-tint-ladder-spectral-chart.pdf", "67ab42c0aa9669fb63669e5dc9ff3d7fcaae9f08154ed6337cd4d5fd6d4f3123"),
    "R02": ("R02-masstone-spectral-chart.pdf", "79dbccf027afbcc9453d5c533d73163fe4434f1d5df1b3b849e2d26d1d73e718"),
    "R03": ("R03-tint-ladder-spectral-chart.pdf", "39504bb3616dbd0aec4001151da39220cabe118af6c92823a72c3eb55634562e"),
    "W03": ("W03-masstone-spectral-chart.pdf", "85cb029b51af1528a242348520de16b6a7f80050a49f7cbd2d0bb9225145bd59"),
}
# dry-film pigment volume concentrations, as the charts print them
PVC = {"B13": 0.134, "B14": 0.201, "B15": 0.109, "R02": 0.08, "R03": 0.026, "W03": 0.228}
EARTHS = {"B13": "burnt sienna", "B14": "raw sienna", "B15": "raw umber"}
# the engine's indices (dry.rs `refractive_index`)
INDEX = {"burnt sienna": 2.3, "raw sienna": 2.0, "raw umber": 2.1, "red earth": 2.7, "yellow ochre": 2.1}
BINDER_N = 1.49

root, pdf_dir = Path(sys.argv[1]), Path(sys.argv[2])
for code, (fn, sha) in FILES.items():
    if hashlib.sha256((pdf_dir / fn).read_bytes()).hexdigest() != sha:
        sys.exit(f"{pdf_dir / fn}: not the LBNL chart this was made from")
spec = importlib.util.spec_from_file_location("dry_pigment_mie", root / "notes/research/dry_pigment_mie.py")
mie = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mie)
src = (root / "crates/paint/src/spectral.rs").read_text()


def table(name):
    v = src.split(f"const {name}:")[1].split("=", 1)[1].split(";", 1)[0]
    return np.array(ast.literal_eval(re.sub(r"//[^\n]*", "", v).strip()))


CMF = table("CMF")
NM = 380 + 10 * np.arange(38)
WHITE_XYZ = CMF.sum(axis=1)
BAND = (NM >= 550) & (NM <= 700)


def lab(r):
    x = CMF @ r / WHITE_XYZ
    f = np.where(x > 0.008856, np.cbrt(x), 7.787 * x + 16 / 116)
    return np.array([116 * f[1] - 16, 500 * (f[0] - f[1]), 200 * (f[1] - f[2])])


def km(q):
    q = np.maximum(q, 0)
    return 1 + q - np.sqrt(q * (q + 2))


# ------------------------------------------------------------ 1. the charts read back
# Each chart is up to four columns (masstone, 1:4, 1:9, W03) of three panels: I, the film
# over a void (R, T, A, linear 0-1); II, K and S (1/mm, log 0.1-1000); III, the film over
# white and black (linear 0-1). Each curve is 441 points, 300-2500 nm in 5 nm steps, across
# a panel 133.2 pt wide. The tick marks (read from the same PDFs): panel II's decades at
# y 282.73 (0.1), 259.4, 236.07, 212.73, 189.4 (1000); panels I and III 0 at 138.73 and
# 426.73, 1 at 45.4 and 333.4 (pdfplumber's top-down y).
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
            continue                        # (the dotted curves are K-M's fits, not measurements)
        c = max(i for i, x0 in enumerate(COLUMNS) if min(q[0] for q in pts) >= x0 - 1)
        x = np.array([300 + (q[0] - COLUMNS[c]) / WIDTH * 2200 for q in pts])
        y = np.array([q[1] for q in pts])
        v = 10 ** (-1 + (282.73 - y) / 23.333) if name in ("K", "S") else (
            (138.73 - y) / 93.33 if name == "T_void" else (426.73 - y) / 93.33)
        # a curve broken where its value left the axis comes in pieces: join them
        pieces = cols[c].setdefault(name, [])
        pieces.extend(zip(x, v))
    out = []
    for col in cols:
        if not col:
            continue
        out.append({k: (lambda a: np.interp(NM, a[:, 0], a[:, 1]))(np.array(sorted(v))) for k, v in col.items()})
    return out


charts = {code: read_chart(pdf_dir / fn) for code, (fn, _) in FILES.items()}

def saunderson(ri, k1=0.04, k2=0.6):
    """What a sphere sees of a film whose internal reflectance is ri (dry_pigment_optics.md §3)."""
    return k1 + (1 - k1) * (1 - k2) * ri / (1 - k2 * ri)


print("1. the curves read back: R-infinity from the read K and S (internal), seen through the film's")
print("   surface (Saunderson, k1 0.04, k2 0.6), against the film's measured reflectance over black, for")
print("   each film and wavelength where the film hides its ground (T over a void under 1 %)")
for code, cols in (("B13", (1, 2)), ("B14", (1, 2)), ("B15", (1, 2)), ("R03", (1, 2))):
    for c in cols:
        f = charts[code][c]
        seen = saunderson(km(f["K"] / f["S"]))
        ok = (f["T_void"] < 0.01) & (NM >= 450)
        if ok.any():
            print(f"  {code} {('', '1:4', '1:9')[c]}: {ok.sum():2d} wavelengths {NM[ok].min()}-{NM[ok].max()} nm, "
                  f"seen from K, S - measured: mean {(seen - f['R_black'])[ok].mean():+.3f}, "
                  f"largest {np.abs(seen - f['R_black'])[ok].max():.3f}")

# ----------------------------------------------------- 2. Mie against hematite of known size
print("\n2. Mie's 3/4 sigma'(1-g) per volume against the synthetic hematites' measured S per volume (um^-1),")
print("   in the binder (n 1.49); hematite m at 600/650/700 nm 3.00+0.030i, 2.95+0.012i, 2.92+0.008i")
print("   (Longtin et al. 1988 via Zhang et al. 2015 and Querry 1985); the maker's mean size, width estimated")
HEMATITE = {600: 3.00 + 0.030j, 650: 2.95 + 0.012j, 700: 2.92 + 0.008j}
for code, size in (("R03", 0.27), ("R02", 0.30)):
    f = charts[code][0]
    for w, m in HEMATITE.items():
        meas = f["S"][NM == w][0] / PVC[code] / 1000
        th = [0.75 * mie.sizeavg(m, BINDER_N, size, lam=w / 1000, sig=s)[0] for s in (0.3, 0.5)]
        print(f"  {code} ({size} um, PVC {PVC[code]:.1%}) {w} nm: measured {meas:.2f}, Mie {th[0]:.2f} (sigma 0.3) - {th[1]:.2f} (0.5)")
w03 = charts["W03"][0]
print(f"  titanium white W03 (PVC 22.8 %, crowded) at 550 nm: measured {w03['S'][NM == 550][0] / PVC['W03'] / 1000:.2f}")

# ----------------------------------------------------------- 3. the earths' S per volume
print("\n3. the earths' S per pigment volume in the binder (um^-1): 450 / 550 / 650 / 700 nm, and the 550-700 nm mean")
S_VOL = {}
for code, name in EARTHS.items():
    s = charts[code][0]["S"] / PVC[code] / 1000
    S_VOL[name] = float(s[BAND].mean())
    print(f"  {name:12s} ({code}, PVC {PVC[code]:.1%}): {' / '.join(f'{s[NM == w][0]:.3f}' for w in (450, 550, 650, 700))}"
          f"; mean {S_VOL[name]:.4f}")
print("  (below 550 nm the films absorb so strongly that S is poorly set; it is lower there)")

# ---------------------------------------------------------------------- 4. the tints
def split_volume(mode, lo, hi, n, nm, lam=0.55, N=61):
    """sigma'(1-g) per particle volume for a laser-diffraction volume distribution: its peak and
    its range, read as log-normal on each side of the peak with the range's ends at 3 sigma."""
    sl, sh = np.log(mode / lo) / 3, np.log(hi / mode) / 3
    u = np.linspace(-3, 3, N)
    ds = mode * np.exp(np.where(u < 0, u * sl, u * sh))
    w = np.exp(-0.5 * u * u) * np.where(u < 0, sl, sh)
    s = 0.0
    for d, wi in zip(ds, w):
        _, qs, _, g = mie.bhmie(np.pi * d * nm / lam, n / nm)
        s += wi * 1.5 * qs * (1 - g) / d
    return s / w.sum()


SIZES = {"burnt umber 40710": (6.4, 0.2, 42.0), "yellow ochre 116421": (10.5, 0.2, 76.0)}
print("\n4. tints with titanium white, K-M by volume of paint: the measured masstone's K/S at each S per")
print("   pigment volume (its K so taken that K/S stays), against the measured tints' R-infinity; dE76")
W_K, W_S = charts["B13"][3]["K"], charts["B13"][3]["S"]
for code, name in EARTHS.items():
    mass = charts[code][0]
    ks = mass["K"] / mass["S"]
    n = INDEX[name]
    meas = {r: km(charts[code][c]["K"] / charts[code][c]["S"]) for r, c in ((4, 1), (9, 2))}
    print(f"  {name} ({code}): measured 1:4 {np.round(lab(meas[4]), 1)}, 1:9 {np.round(lab(meas[9]), 1)}")
    cands = [("the measured S (spectral)", mass["S"] / PVC[code] / 1000),
             ("the measured S, 550-700 nm mean", np.full(38, S_VOL[name])),
             ("Mie, #351's 1 um", np.full(38, 0.75 * mie.sizeavg(n, BINDER_N, 1.0)[0]))]
    cands += [(f"Mie, Kremer {k} as measured", np.full(38, 0.75 * split_volume(*v, n, BINDER_N))) for k, v in SIZES.items()]
    for label, sv in cands:
        se = sv * PVC[code] * 1000
        row = []
        for r in (4, 9):
            t = lab(km((ks * se + r * W_K) / (se + r * W_S)))
            row.append(f"1:{r} {np.round(t, 1)} dE {np.linalg.norm(t - lab(meas[r])):4.1f}")
        print(f"    {label:38s} S {sv[BAND].mean():.3f}  " + "  ".join(row))

# -------------------------------------------------------------- 5. the air/oil ratio
print("\n5. the Mie air/oil (n 1.48) ratio of S at the measured sizes, and at #351's 1 um:")
RATIO = {}
for name, n in INDEX.items():
    size = SIZES["yellow ochre 116421"] if name == "yellow ochre" else SIZES["burnt umber 40710"]
    a, o = split_volume(*size, n, 1.0), split_volume(*size, n, 1.48)
    RATIO[name] = a / o
    a1, o1 = mie.sizeavg(n, 1.0, 1.0)[0], mie.sizeavg(n, 1.48, 1.0)[0]
    print(f"  {name:12s} n {n}: measured size {a / o:.3f}   1 um {a1 / o1:.3f}")

print("\nfor dry.rs earth_scatter (S per pigment volume in oil, um^-1; Mie air/oil ratio):")
for name in ("burnt sienna", "raw sienna", "raw umber"):
    print(f'        "{name}" => Some(({S_VOL[name]:.4f}, {RATIO[name]:.3f})),')
print(f'        "red earth" => Some(({S_VOL["burnt sienna"]:.4f}, {RATIO["red earth"]:.3f})),   (burnt sienna\'s S)')
print(f'        "yellow ochre" => Some(({S_VOL["raw sienna"]:.4f}, {RATIO["yellow ochre"]:.3f})),   (raw sienna\'s S)')
