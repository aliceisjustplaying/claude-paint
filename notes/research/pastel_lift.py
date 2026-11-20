#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
# How readily a kneaded eraser takes a pastel stick's deposit (the cells'
# `lift`), from adhesion: the law charcoal's lift comes from
# (charcoal_optics.py §3, charcoal.md §4), applied to pastel's particles.
# It is written separately from the engine (crates/paint/src/adhesion.rs),
# so the two check each other. notes/research/pastel_lift.md has the sources.
#
#   1. the law, and charcoal's lifts from it (VINE_LIFT 0.85, the fines' 0.10);
#   2. Hamaker constants from the refractive index (Tabor–Winterton) against
#      measured ones;
#   3. whiting's lift over its measured size distribution, and its bounds;
#   4. sticks: whiting, tints, pure pigments.
#
# Run: uv run notes/research/pastel_lift.py
import math
from statistics import NormalDist

# --- 1. The law (charcoal_optics.py §3, unchanged) -------------------------
A_CEL = 5.8e-20  # cellulose (Bergström et al. 1999)
A_ERASER = 6.5e-20  # a kneaded eraser: polyisobutylene, oil, CaCO3/TiO2 fillers (hydrocarbon, 6–7)
A_CARBON = 6.0e-20  # char: carbon fibres 5.2–6.2, activated carbon 5.0–7.8 (Maurer et al. 2001)
A_WHITING = 10.1e-20  # calcite, measured 10.19 ± 0.55 (Bergström 1997)
D0 = 0.165e-9  # contact cut-off distance
GAMMA = 0.072  # water's surface tension, J/m²
V_M = 18.0e-6  # water's molar volume, m³/mol
R_GAS = 8.314
ASPERITY = 0.1e-6  # the fibre asperity a particle rests on (0.05–0.2 µm)
F_ROUGH = math.sqrt(10e-9 * 150e-9)  # a rough particle's hold on the fibre: 10–150 nN, the geometric middle
SPREAD = math.log(10) / 2  # a decade wide: ±1σ spans ×10
FINE_R = 0.3e-6  # below this a particle sits on one smooth asperity (Nagaashi: 0.14–0.5 µm)


def kelvin_diameter(rh, t_c):
    """Twice the Kelvin radius of a meniscus at relative humidity `rh`: water condenses in a narrower gap."""
    return 2 * GAMMA * V_M / (R_GAS * (t_c + 273.15) * math.log(1 / rh))


def taken(r, a_p, cos_p=1.0, rh=0.5, t_c=20.0, asperity=ASPERITY, rough_scale=1.0):
    """The chance the eraser out-pulls the paper's hold on a particle of radius `r` (m) and Hamaker constant `a_p`."""
    w = math.sqrt(A_ERASER * a_p) / (12 * math.pi * D0**2)
    f_eraser = 1.5 * math.pi * w * r  # JKR: the putty wraps the particle
    a_pc = math.sqrt(A_CEL * a_p)
    f_vdw = a_pc * r / (6 * D0**2) if r < FINE_R else F_ROUGH * rough_scale  # DMT below the asperity scale
    # the water bridge at the contact, both sides wetted (cos θ fibre 1); the
    # eraser's hydrocarbon putty gets none
    f_water = 2 * math.pi * GAMMA * min(r, asperity) * max(cos_p + 1.0, 0.0) if kelvin_diameter(rh, t_c) > D0 else 0.0
    return NormalDist().cdf(math.log(f_eraser / (f_vdw + f_water)) / SPREAD)


print("== 1. The law, and charcoal from it")
print(f"Kelvin diameter at 50 % RH, 20 °C: {kelvin_diameter(0.5, 20) * 1e9:.2f} nm (a bridge forms where the gap is narrower; the contact's is {D0 * 1e9} nm)")
print(f"  a bridge forms above {math.exp(-2 * GAMMA * V_M / (R_GAS * 293.15 * D0)) * 100:.2f} % RH")
print(f"VINE_LIFT: splinters R 1.5 µm, carbon: {taken(1.5e-6, A_CARBON):.3f} (charcoal.rs 0.85)")
print(f"compressed's fines, R 0.1 µm: {taken(0.1e-6, A_CARBON):.3f} (charcoal_optics.py 0.10)")

# --- 2. Hamaker constants from the index -----------------------------------
# Tabor–Winterton: A = ¾kT((ε−1)/(ε+1))² + 3ħω_e/(16√2) (n²−1)²/(n²+1)^1.5.
# The UV absorption frequency ω_e (10¹⁶ rad/s), Bergström 1997 via Lefèvre et
# al.: wide-gap insulators silica 2.03, alumina 2.00, BeO 2.11, calcite 1.90,
# MgO 1.71, quartz 2.03 (mean 1.96); the high-index oxides TiO2 0.735, ZnO
# 0.895 (mean 0.815). Below n 1.85 a pigment is taken as the first, above as
# the second. ε, unknown for most pigments, is taken as n² (its least); the
# static term is under 3 % of A.
HBAR, KT = 1.0546e-34, 1.380649e-23 * 293.15
W_INSULATOR, W_OXIDE, N_SPLIT = 1.96e16, 0.815e16, 1.85


def hamaker_tw(n, w_e=None, eps=None):
    w_e = w_e or (W_INSULATOR if n < N_SPLIT else W_OXIDE)
    eps = eps or n * n
    return 0.75 * KT * ((eps - 1) / (eps + 1)) ** 2 + 3 * HBAR * w_e / (16 * math.sqrt(2)) * (n * n - 1) ** 2 / (n * n + 1) ** 1.5


print("\n== 2. Hamaker constants (10⁻²⁰ J): Tabor–Winterton against measured")
for name, n, w_e, eps, measured in [("calcite", 1.586, 1.90e16, 8.2, 10.1), ("TiO2", 2.464, 0.735e16, 114, 15), ("ZnO", 1.91, 0.895e16, 11.8, 9.2), ("MgO", 1.716, 1.71e16, 9.8, 12.1), ("silica", 1.448, 2.03e16, 3.82, 6.5)]:
    print(f"  {name:7} own ω_e {hamaker_tw(n, w_e, eps) * 1e20:5.1f}; class ω_e, ε = n² {hamaker_tw(n) * 1e20:5.1f}; measured {measured}")
print(f"  hematite (n 3.0, class ω_e) {hamaker_tw(3.0) * 1e20:.1f}; measured ≈ 25 (De Mesquita et al. 2003)")

# --- 3. Whiting over its size distribution ---------------------------------
# Champagne/Meudon whiting, by volume a lognormal through d50 2.4–2.7 µm and
# D97 13–15 µm: σ_ln 0.87–0.94, d32 1.64–1.75 µm (wet_pastel.md §1). What
# the eraser takes is a share of the mark's coverage, and coverage per
# volume goes as 1/d: the take is averaged over the distribution by
# projected area (surface-weighted, median d32·e^(−σ²/2)).
D32, SIGMA = 1.7, 0.905


def stick_take(a_p, cos_p=1.0, d32=D32, sigma=SIGMA, weight="area", **kw):
    med = d32 * math.exp(-sigma * sigma / 2) if weight == "area" else d32 * math.exp(sigma * sigma / 2)
    n, acc, tot = 400, 0.0, 0.0
    for i in range(n):
        z = -6 + 12 * (i + 0.5) / n
        w = math.exp(-z * z / 2)
        acc += w * taken(med * math.exp(sigma * z) / 2 * 1e-6, a_p, cos_p, **kw)
        tot += w
    return acc / tot


print("\n== 3. Whiting's lift")
w0 = stick_take(A_WHITING)
print(f"whiting (d32 {D32} µm, σ_ln {SIGMA}, by area, 50 % RH, asperity 0.1 µm): {w0:.3f}")
print(f"  σ_ln 0.87–0.94: {stick_take(A_WHITING, sigma=0.94):.3f}–{stick_take(A_WHITING, sigma=0.87):.3f}; d32 1.64–1.75: {stick_take(A_WHITING, d32=1.64):.3f}–{stick_take(A_WHITING, d32=1.75):.3f}")
print(f"  asperity 0.2–0.05 µm: {stick_take(A_WHITING, asperity=0.2e-6):.3f}–{stick_take(A_WHITING, asperity=0.05e-6):.3f}")
print(f"  by volume instead of area: {stick_take(A_WHITING, weight='vol'):.3f}")
print(f"  the rough hold scaled with √A (the eraser's is): {stick_take(A_WHITING, rough_scale=math.sqrt(A_WHITING / A_CARBON)):.3f}")
print(f"  a dry room (no bridge, 0.1 % RH): {stick_take(A_WHITING, rh=0.001):.3f}")
print(f"  for the record, the old PASTEL_LIFT 0.55; the audit's rough estimate 0.6–0.75, proposed 0.70")

# --- 4. Sticks --------------------------------------------------------------
print("\n== 4. Pure pigments (whiting's sizes; Hamaker by Tabor–Winterton)")
for name, n, cos_p in [("ultramarine", 1.5, 1), ("Prussian blue", 1.56, 1), ("lake (alumina 1.57)", 1.57, 1), ("green earth", 1.62, 1), ("cobalt blue", 1.74, 1), ("lead white", 2.0, 1), ("chrome yellow", 2.4, 1), ("red earth", 2.7, 1), ("vermilion", 3.0, 1), ("permanent alizarin (PV19, hydrophobic)", 2.03, 0)]:
    print(f"  {name:40} A {hamaker_tw(n) * 1e20:5.1f}  lift {stick_take(hamaker_tw(n), cos_p):.3f}")
print(f"  {'vine or bone black (carbon)':40} A   6.0  lift {stick_take(A_CARBON):.3f}")
