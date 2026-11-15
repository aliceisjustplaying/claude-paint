#!/usr/bin/env -S uv run --quiet --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["numpy"]
# ///
# The physics behind notes/research/charcoal.md §§2–6, with the numbers the
# engine takes from it (crates/paint/src/dry.rs `CarbonBed`, charcoal.rs):
#   1. charcoal's dry color (R∞) from carbon's measured complex index by Mie
#      in air, as dry_pigment_optics.md §2 does for pigments (its bhmie and
#      size average, dry_pigment_mie.py); compressed charcoal's binder and
#      packing by an effective medium (Bruggeman);
#   2. what a fixative's resin (n 1.5) in the pores does to it, the share it
#      fills from the spray's composition;
#   3. how readily a kneaded eraser takes its particles (van der Waals
#      adhesion, eraser against paper);
#   4. pastel laid into loose charcoal: Kubelka–Munk mixing against the
#      engine's mixing of reflectances.
# Run: uv run notes/research/charcoal_optics.py
import importlib.util, math, os
import numpy as np

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("dry_pigment_mie", os.path.join(here, "dry_pigment_mie.py"))
mie = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mie)

# The engine's linear RGB as three wavelengths (µm).
LAM = (0.61, 0.55, 0.46)

# Cellulose carbonized at 400 and 600 °C, n and k (Jäger, Mutschke &
# Henning 1998, A&A 332:291; Jena OCDB, data/carbon/cel400.lnk and
# cel600.lnk), at the tabulated wavelengths around the three above.
CEL = {
    400: [(0.6310, 1.573, 0.4157), (0.5643, 1.524, 0.4663), (0.4990, 1.452, 0.5126), (0.4467, 1.375, 0.5422)],
    600: [(0.6310, 1.345, 0.7807), (0.5623, 1.224, 0.7793), (0.5012, 1.105, 0.7526), (0.4467, 0.9954, 0.7020)],
}
# Charcoal is fired to about 500 °C (FAO 1987, Simple technologies for
# charcoal making, ch. 4: "a final carbonising temperature of around 500 °C";
# 450–500 °C balances friability against fixed carbon): halfway between.
T_FIRE = 500.0

# Lamp black (soot): Bond & Bergstrom 2006 (Aerosol Sci. Tech. 40:27),
# 1.95 + 0.79i at 550 nm, flat over the visible as they recommend.
SOOT = complex(1.95, 0.79)
GUM = complex(1.47, 0.0)  # dry gum arabic film (estimate within 1.45–1.50)
KAOLIN = complex(1.56, 0.0)  # dry_pigment_optics.md table 2.2
RESIN = 1.5  # an acrylic fixative resin, as fix_pastel takes it

# Bohren & Huffman's convention, m = n + ik.
def char_index(lam, t=T_FIRE):
    def at(rows):
        rows = sorted(rows)
        ls = [r[0] for r in rows]
        return complex(np.interp(lam, ls, [r[1] for r in rows]), np.interp(lam, ls, [r[2] for r in rows]))
    u = (t - 400.0) / 200.0
    return (1 - u) * at(CEL[400]) + u * at(CEL[600])


def bruggeman(comps):
    """Effective index of a fine mixture [(m, volume share)] (Bruggeman)."""
    eps = [m * m for m, _ in comps]
    fs = [f for _, f in comps]
    e = sum(f * x for x, f in zip(eps, fs))
    for _ in range(100):
        F = sum(f * (x - e) / (x + 2 * e) for x, f in zip(eps, fs))
        dF = sum(f * (-3 * x) / (x + 2 * e) ** 2 for x, f in zip(eps, fs))
        e -= F / dF
    return np.sqrt(e)


def ks(idx, d, nmed):
    """KM K and S per unit particle volume (µm⁻¹) per channel: K = 2σa,
    S = 0.75σs(1−g) (dry_pigment_optics.md §2.1), lognormal sizes (σ_ln 0.5)
    about median d µm, in a medium of index nmed."""
    K, S = [], []
    for lam in LAM:
        s, a = mie.sizeavg(idx(lam), nmed, d, lam=lam)
        K.append(2 * a)
        S.append(0.75 * s)
    return np.array(K), np.array(S)


def km(k, s):
    q = k / s
    return 1 + q - np.sqrt(q * q + 2 * q)


def lum(r):
    y = 0.2126 * r[0] + 0.7152 * r[1] + 0.0722 * r[2]
    return y, (116 * np.cbrt(y) - 16 if y > 0.008856 else 903.3 * y)


HAZE = 0.04  # canvas.rs SURFACE_REFLECTANCE: the matte first-surface veil the engine shows over paper and drawing


def seen(r):
    return HAZE + (1 - HAZE) * r


def P(ratio, sigma):
    """The chance a lognormally spread hold (σ_ln `sigma`) is below a pull `ratio` times its median."""
    return 0.5 * (1 + math.erf(math.log(ratio) / (sigma * math.sqrt(2))))


# Vine: splinters of charred cell wall. Willow (Salix) fibres have walls of
# about 2–4 µm (wood anatomy; estimate of the typical range); a splinter
# is about a wall thick, so median 3 µm; R∞ barely depends on it (below).
VINE_D = 3.0
char = lambda lam: char_index(lam)

# Compressed: charcoal powder pressed with a gum binder. Binder 5 % of the
# solids by volume (organic binders in pressed charcoal 2–8 % by weight,
# US 5,221,290; a black chalk's CMC 2.5 %, US 2021/0292589; gum and char
# densities are alike, ~1.4–1.5); solids 60 % of the stick (a pressed powder
# compact; estimate). Abraded, it lays crumbs of that compact (median 3 µm,
# as the splinters).
C_SOLIDS, C_BINDER = 0.60, 0.05
COMP_D = 3.0


def compressed_index(fill=1.0 + 0j, carbon=char, solids=C_SOLIDS, binder=C_BINDER, clay=0.0):
    return lambda lam: bruggeman([(carbon(lam), solids * (1 - binder - clay)), (GUM, solids * binder), (KAOLIN, solids * clay), (fill, 1 - solids)])


def rust(name, kair, sair, kres, sres):
    f = lambda v: "[" + ", ".join(f"{x:.5g}" for x in v) + "]"
    print(f"pub const {name}: CarbonBed = CarbonBed {{ k_air: {f(kair)}, s_air: {f(sair)}, k_resin: {f(kres)}, s_resin: {f(sres)} }};")


if __name__ == "__main__":
    print("== 1. Optics")
    print("charcoal (500 °C) index:", ", ".join(f"{l} µm {char_index(l):.3f}" for l in LAM))
    for l in LAM:
        m = char_index(l)
        print(f"  facet reflectance at normal incidence, {l} µm: carbon/air {abs((m - 1) / (m + 1)) ** 2:.3f}, carbon/resin {abs((m / RESIN - 1) / (m / RESIN + 1)) ** 2:.3f}")
    print("vine R∞ by splinter size (µm): insensitive above a micrometre")
    for d in (1.0, 2.0, 3.0, 5.0, 10.0, 20.0):
        r = km(*ks(char, d, 1.0))
        print(f"  d {d:4.1f}: {np.round(r, 4)}  seen (with the matte veil) L* {lum(seen(r))[1]:.1f}")
    for t in (400.0, 600.0):
        r = km(*ks(lambda lam, t=t: char_index(lam, t), VINE_D, 1.0))
        print(f"  fired at {t:.0f} °C instead: {np.round(r, 4)}")
    print("free fine carbon in air (a bloom of dust), by size (µm):")
    for d in (0.1, 0.2, 0.3, 0.5):
        print(f"  char d {d}: {np.round(km(*ks(char, d, 1.0)), 4)}  soot d {d}: {np.round(km(*ks(lambda l: SOOT, d, 1.0)), 4)}")
    kv, sv = ks(char, VINE_D, 1.0)
    kvr, svr = ks(char, VINE_D, RESIN)
    rv = km(kv, sv)
    print(f"VINE R∞ {np.round(rv, 4)}  Y {lum(rv)[0]:.4f}, seen Y {lum(seen(rv))[0]:.4f} L* {lum(seen(rv))[1]:.1f} (measured powder < 0.05: arXiv 2110.05832)")
    kc, sc = ks(compressed_index(), COMP_D, 1.0)
    kcr, scr = ks(compressed_index(RESIN + 0j), COMP_D, RESIN)
    rc = km(kc, sc)
    print(f"COMPRESSED index {compressed_index()(0.55):.3f}; R∞ {np.round(rc, 4)}  Y {lum(rc)[0]:.4f}, seen L* {lum(seen(rc))[1]:.1f}")
    print("  compressed, sensitivity:")
    for label, idx in [
        ("solids 0.5", compressed_index(solids=0.5)), ("solids 0.7", compressed_index(solids=0.7)),
        ("binder 0.02", compressed_index(binder=0.02)), ("binder 0.10", compressed_index(binder=0.10)),
        ("half the carbon lamp black", compressed_index(carbon=lambda l: 0.5 * (char_index(l) + SOOT))),
        ("clay 0.2 of solids", compressed_index(clay=0.2)), ("clay 0.4 of solids", compressed_index(clay=0.4)),
    ]:
        print(f"    {label:28s} R∞ {np.round(km(*ks(idx, COMP_D, 1.0)), 4)}")
    print("for dry.rs (KM K and S per unit particle volume, µm⁻¹):")
    rust("VINE", kv, sv, kvr, svr)
    rust("COMPRESSED", kc, sc, kcr, scr)

    print("\n== 2. Fixative")
    # Krylon Workable Fixatif SDS (2017, rev. 5), % by weight: acetone 46.5,
    # toluene 20, propane 12.75, butane 12.25, ethyl 3-ethoxypropionate
    # 3.25; the rest, 5.25 %, unlisted (the resin and any additive).
    resin_wt, solvents = 5.25, [(46.5, 0.784), (20.0, 0.867), (3.25, 0.95)]
    v_solv = sum(w / rho for w, rho in solvents)
    v_res = resin_wt / 1.15  # an acrylic resin, ~1.15 g/cm³
    phi = v_res / (v_res + v_solv)
    print(f"resin share of the liquid by volume: {phi:.3f} (what dries in a pore the solution filled)")
    print(f"pastel's light spray wets {0.15}: {0.15 / phi:.1f}× that (solvent flashed off in flight, resin carried to the drying top: unmeasured)")
    for w in (0.0, phi, 0.15, 0.3, 0.6, 1.0):
        r = km((1 - w) * kv + w * kvr, (1 - w) * sv + w * svr)
        print(f"  vine, a share {w:.3f} wetted: R∞ {np.round(r, 4)}  seen Y {lum(seen(r))[0]:.4f} L* {lum(seen(r))[1]:.2f}")
    print("  carbon's scattering air/resin per channel:", np.round(sv / svr, 2), "(pastel's rule: 5)")
    # the engine's rule (graphite.rs Cell::bind): K/S ÷ (1 − w + w/5)
    r_rule = km(kv / sv / (1 - 0.15 + 0.15 / 5), np.ones(3))
    print(f"  the engine's rule at 0.15: R∞ {np.round(r_rule, 4)}  seen L* {lum(seen(r_rule))[1]:.2f}")

    print("\n== 3. The kneaded eraser against the paper")
    # Hamaker constants (10⁻²⁰ J): cellulose 5.8 (Bergström et al. 1999);
    # wood char as activated carbon 5.0–7.8 (adsorption data; graphite is
    # 24 ± 13, Gallego-Fuente et al. 2026): 6.4; a rubber/PVC putty as a
    # hydrocarbon polymer, ~6.5 (non-metallic solids 5–10).
    A_cel, A_c, A_e = 5.8e-20, 6.4e-20, 6.5e-20
    A_pc, A_ec = math.sqrt(A_cel * A_c), math.sqrt(A_e * A_c)
    D0 = 0.165e-9
    W_ec = A_ec / (12 * math.pi * D0 ** 2)
    print(f"eraser–carbon work of adhesion {W_ec * 1e3:.0f} mJ/m² (A/12πD0², D0 0.165 nm)")
    # The putty (E ~0.1 MPa) conforms to the particle (W/E ~ 0.6 µm, above
    # its roughness): JKR pull-off 1.5πWR. Against the fibre a rigid, rough
    # particle above the asperity scale (0.14–0.5 µm) holds 10–150 nN whatever
    # its size (Nagaashi 2021; Moutinho; pastel_brushing.md §1), a decade
    # wide; below it, on one smooth asperity, A·R/6D0² (DMT).
    sigma = math.log(10) / 2  # a decade wide: ±1σ spans 10×
    F_rough = math.sqrt(10e-9 * 150e-9)
    for R_um in (0.05, 0.1, 0.2, 0.3, 0.5, 1.0, 1.5, 3.0, 5.0, 10.0):
        R = R_um * 1e-6
        F_e = 1.5 * math.pi * W_ec * R
        F_h = A_pc * R / (6 * D0 ** 2) if R_um < 0.3 else F_rough
        print(f"  R {R_um:5.2f} µm: eraser {F_e * 1e9:7.1f} nN, paper {F_h * 1e9:6.1f} nN, ratio {F_e / F_h:6.2f}, taken {P(F_e / F_h, sigma):.2f}")
    vine_lift = P(1.5 * math.pi * W_ec * VINE_D / 2 * 1e-6 / F_rough, sigma)
    fine_lift = P(1.5 * math.pi * W_ec / (A_pc / (6 * D0 ** 2)), sigma)
    print(f"VINE_LIFT (splinters R {VINE_D / 2} µm) {vine_lift:.3f}; a particle below the asperity scale {fine_lift:.3f}")
    print("pastel (chalk and pigment 1–20 µm, rough): ", f"{P(1.5 * math.pi * W_ec * 0.5e-6 / F_rough, sigma):.2f}–{P(1.5 * math.pi * W_ec * 10e-6 / F_rough, sigma):.2f}", "(the engine's 0.55)")
    # Gum arabic's glass transition: 62 °C at aw 0.33, 42.6 °C at aw 0.54
    # (Righetto & Netto 2005, Int. J. Food Prop. 8:337); sticky only near Tg.
    print("gum arabic at 20–25 °C and aw ≤ 0.54 is 18–40 K below its Tg: glassy, it bridges no new contact (grip 0)")

    print("\n== 4. Pastel laid into loose charcoal")
    # A yellow pastel stick [0.75, 0.6, 0.1]; its particles scatter as chalk
    # filler in air, S ≈ 0.75 × 0.73 µm⁻¹ (dry_pigment_optics.md table 2.2,
    # d = 1 µm), K from its R∞.
    p = np.array([0.75, 0.6, 0.1])
    s_p = 0.75 * 0.73 * np.ones(3)
    k_p = (1 - p) ** 2 / (2 * p) * s_p
    for c in (0.02, 0.05, 0.1, 0.2):
        lin = (1 - c) * p + c * rv
        kmix = km((1 - c) * k_p + c * kv, (1 - c) * s_p + c * sv)
        print(f"  charcoal {c:.2f} of the volume: the engine (reflectances by volume) {np.round(lin, 3)}, Kubelka–Munk {np.round(kmix, 3)}")
