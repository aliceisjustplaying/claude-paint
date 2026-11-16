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
#   2. what a fixative's resin (n 1.49) in the pores does to it, and the
#      share of the pores a light spray fills (its resin over their volume);
#   3. how readily a kneaded eraser takes its particles (van der Waals
#      adhesion, eraser against paper);
#   4. pastel laid into loose charcoal: Kubelka–Munk mixing against the
#      engine's mixing of reflectances;
#   5. how much a stick lays: vine's hardness, and the area a worn volume
#      covers.
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
GUM = complex(1.52, 0.0)  # dry gum arabic film, measured 1.51–1.54 (dry_pigment_optics.md §1)
KAOLIN = complex(1.56, 0.0)  # dry_pigment_optics.md table 2.2
# A workable fixative's resin: Krylon 1301 resembles poly(n-butyl
# methacrylate) (AIC Conservation Wiki, BPG Adhesives), n_D 1.483 and
# 1.06–1.07 g/cm³ (Scientific Polymer Products; Polysciences). PMMA is
# 1.4905, Paraloid B-72 1.487: 1.49.
RESIN, RESIN_RHO = 1.49, 1.07

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


# canvas.rs SURFACE_REFLECTANCE: the matte first-surface veil the engine
# shows over paint and paper. Not over a bed of loose or fixed particles:
# Mie's scattering of each opaque particle already holds its facets'
# (first-surface) reflection, and KM's R∞ is the whole bed's diffuse return;
# a powder has no smooth interface for a second veil (estimates-audit-2 C1).
# Kept here to show what it would add.
HAZE = 0.04


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
# Its carbon: burnt charcoal ground to splinters and lamp black (the
# historical recipe "burned birch, clay, lamp black", Wikipedia, Charcoal
# (art)). The soot's share is unmeasured (no SDS gives it) and sets two
# things: the fine compact's share of the crumbs (optics, §1) and the share
# of fines in a mark (the eraser, §3). Half (estimates-audit-2 C4, C5; bounds
# 0 to 1, a bench SEM of a mark would set it).
C_FINES = 0.5

# The studio's relative humidity (estimates-audit-2: 50 %). It sets the
# capillary bridge at the paper's asperities (§3) and gum arabic's glass
# transition (§3). The engine's one studio setting, crate::climate::STUDIO
# (20 °C, 50 % RH); keep this in step with it.
STUDIO_RH = 0.50


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
        print(f"  d {d:4.1f}: {np.round(r, 4)}  L* {lum(r)[1]:.1f}")
    for t in (400.0, 600.0):
        r = km(*ks(lambda lam, t=t: char_index(lam, t), VINE_D, 1.0))
        print(f"  fired at {t:.0f} °C instead: {np.round(r, 4)}")
    print("free fine carbon in air (a bloom of dust), by size (µm):")
    for d in (0.1, 0.2, 0.3, 0.5):
        print(f"  char d {d}: {np.round(km(*ks(char, d, 1.0)), 4)}  soot d {d}: {np.round(km(*ks(lambda l: SOOT, d, 1.0)), 4)}")
    kv, sv = ks(char, VINE_D, 1.0)
    kvr, svr = ks(char, VINE_D, RESIN)
    rv = km(kv, sv)
    print(f"VINE R∞ {np.round(rv, 4)}  Y {lum(rv)[0]:.4f} L* {lum(rv)[1]:.1f}, as seen (a bare bed: no veil, the Mie holds the facets)")
    print(f"  with the 4 % veil it would be Y {lum(seen(rv))[0]:.4f} L* {lum(seen(rv))[1]:.1f}, the facets counted twice (measured powder < 0.05: arXiv 2110.05832)")
    # Compressed: its crumbs are a fine compact (the effective medium, pores
    # finer than light) only where the carbon is soot of tens of nm; ground
    # charcoal of 1–10 µm makes them a bed of opaque splinters, as vine's.
    # The two mixed by Kubelka–Munk (K and S by volume), a share C_FINES of
    # the compact.
    kf, sf = ks(compressed_index(), COMP_D, 1.0)
    kfr, sfr = ks(compressed_index(RESIN + 0j), COMP_D, RESIN)
    kc, sc = C_FINES * kf + (1 - C_FINES) * kv, C_FINES * sf + (1 - C_FINES) * sv
    kcr, scr = C_FINES * kfr + (1 - C_FINES) * kvr, C_FINES * sfr + (1 - C_FINES) * svr
    rc = km(kc, sc)
    print(f"fine compact index {compressed_index()(0.55):.3f}; alone R∞ {np.round(km(kf, sf), 4)} (the floor: all fines)")
    ksoot, ssoot = ks(compressed_index(carbon=lambda l: SOOT), COMP_D, 1.0)
    print(f"  the fine compact of soot instead of char: alone R∞ {np.round(km(ksoot, ssoot), 4)}, half with splinters {np.round(km(C_FINES * ksoot + (1 - C_FINES) * kv, C_FINES * ssoot + (1 - C_FINES) * sv), 4)}")
    print(f"COMPRESSED ({C_FINES:.0%} fine compact, the rest splinters): R∞ {np.round(rc, 4)}  Y {lum(rc)[0]:.4f}, seen L* {lum(seen(rc))[1]:.1f}; with no veil (a bare bed) L* {lum(rc)[1]:.1f}")
    print("  compressed, sensitivity:")
    for label, idx in [
        ("solids 0.5", compressed_index(solids=0.5)), ("solids 0.7", compressed_index(solids=0.7)),
        ("binder 0.02", compressed_index(binder=0.02)), ("binder 0.10", compressed_index(binder=0.10)),
        ("half the carbon lamp black", compressed_index(carbon=lambda l: 0.5 * (char_index(l) + SOOT))),
        ("clay 0.2 of solids", compressed_index(clay=0.2)), ("clay 0.4 of solids", compressed_index(clay=0.4)),
    ]:
        print(f"    the fine compact, {label:28s} R∞ {np.round(km(*ks(idx, COMP_D, 1.0)), 4)}")
    print("for dry.rs (KM K and S per unit particle volume, µm⁻¹):")
    rust("VINE", kv, sv, kvr, svr)
    rust("COMPRESSED", kc, sc, kcr, scr)

    print("\n== 2. Fixative")
    # Krylon Workable Fixatif SDS (2017, rev. 5), % by weight: acetone 46.5,
    # toluene 20, propane 12.75, butane 12.25, ethyl 3-ethoxypropionate
    # 3.25; the rest, 5.25 %, unlisted (the resin and any additive).
    resin_wt, solvents = 5.25, [(46.5, 0.784), (20.0, 0.867), (3.25, 0.95)]
    v_solv = sum(w / rho for w, rho in solvents)
    v_res = resin_wt / RESIN_RHO
    phi = v_res / (v_res + v_solv)
    print(f"resin share of the liquid by volume: {phi:.3f} (what dries in a pore the solution filled)")
    # How much resin a light spray lays (estimates-audit-2 C11): an 11 oz
    # (312 g) can covers 15–20 ft² a coat (retail listings: Hobby Lobby,
    # Amazon, Walmart), 8–11 µm of resin at 5.25 % and 1.07 g/cm³; about half
    # reaches the sheet (estimate), and a light spray is a third of a coat
    # (estimate): 1.4–1.8 µm, so 1.5. It fills that much of the pores of the
    # loose bed under it (porosity 0.55, the audit's, as loose pastel's).
    for ft2 in (15, 20):
        t = 312 * resin_wt / 100 / (ft2 * 0.0929) / RESIN_RHO
        print(f"  a labelled coat over {ft2} ft²: {t:.1f} µm of resin; half of it landing, a light third: {t / 6:.1f} µm")
    LIGHT_SPRAY_UM, BED_EPS = 1.5, 0.55
    for loose in (1.0, 3.0, 10.0, 20.0, 30.0):
        print(f"  {LIGHT_SPRAY_UM} µm of resin into {loose:4.1f} µm of loose charcoal: wets {min(1.0, LIGHT_SPRAY_UM / (BED_EPS * loose)):.2f}")
    for w in (0.0, phi, 0.15, 0.3, 0.6, 1.0):
        r = km((1 - w) * kv + w * kvr, (1 - w) * sv + w * svr)
        print(f"  vine, a share {w:.3f} wetted: R∞ {np.round(r, 4)}  Y {lum(r)[0]:.4f} L* {lum(r)[1]:.2f}")
    print("  carbon's scattering air/resin per channel:", np.round(sv / svr, 2), "(pastel's rule: 5)")
    # what the engine's bind needs for carbon: K/S wetted over K/S dry, per
    # channel (pastel's rule is 5 for every particle, Cell::bind)
    gv, gc = (kvr / svr) / (kv / sv), (kcr / scr) / (kc / sc)
    print(f"  K/S wetted / dry: vine {np.round(gv, 3)}, compressed {np.round(gc, 3)}; for graphite.rs CARBON_WET_KS {np.round(0.5 * (gv + gc), 3)} (their mean)")
    # the engine's pastel rule (graphite.rs Cell::bind): K/S ÷ (1 − w + w/5)
    r_rule = km(kv / sv / (1 - 0.15 + 0.15 / 5), np.ones(3))
    print(f"  pastel's rule at 0.15 would give: R∞ {np.round(r_rule, 4)}  L* {lum(r_rule)[1]:.2f}; at 1: L* {lum(km(kv / sv * 5, np.ones(3)))[1]:.2f}")

    print("\n== 3. The kneaded eraser against the paper")
    # Hamaker constants (10⁻²⁰ J): cellulose 5.8 (Bergström et al. 1999);
    # carbon 6.0 (carbon fibres 5.2–6.2, activated carbon 5.0–7.8: Maurer,
    # Mersmann & Peukert 2001 and the values they cite; graphite is 24 ± 13,
    # Gallego-Fuente et al. 2026, but char is no graphite); a kneaded eraser,
    # polyisobutylene with oil and CaCO3/TiO2 fillers (General's MSDS,
    # Derwent and ESPO SDSs), as a hydrocarbon polymer, ~6.5.
    A_cel, A_c, A_e = 5.8e-20, 6.0e-20, 6.5e-20
    A_pc, A_ec = math.sqrt(A_cel * A_c), math.sqrt(A_e * A_c)
    D0 = 0.165e-9
    W_ec = A_ec / (12 * math.pi * D0 ** 2)
    print(f"eraser–carbon work of adhesion {W_ec * 1e3:.0f} mJ/m² (A/12πD0², D0 0.165 nm)")
    # The putty (E ~0.1 MPa) conforms to the particle (W/E ~ 0.6 µm, above
    # its roughness): JKR pull-off 1.5πWR. Against the fibre a rigid, rough
    # particle above the asperity scale (0.14–0.5 µm) holds 10–150 nN whatever
    # its size (Nagaashi 2021; Moutinho; pastel_brushing.md §1), a decade
    # wide; below it, on one smooth asperity, A·R/6D0² (DMT).
    #
    # And water. Char fired at 500 °C is hydrophilic: water soaks into it
    # before a contact angle can be read (Pyrolysis temperature effects on
    # biochar–water interactions, Soil Systems 2019, 3:27; Kinney et al. 2012,
    # Biomass Bioenergy: least hydrophobic at 400–600 °C), and so is
    # cellulose. In a room at STUDIO_RH the Kelvin radius is about a
    # nanometre, so water condenses at each contact, and a bridge at an
    # asperity of radius a holds 4πγa more (cos θ ≈ 1). The eraser's
    # hydrocarbon putty is hydrophobic, so it gets none. The asperity of a
    # fibre's surface a splinter rests on: 0.1 µm (0.05–0.2; Nagaashi's
    # effective contact radii are 0.14–0.5 µm).
    gamma = 0.072
    r_kelvin = 2 * gamma * 18.0e-6 / (8.314 * 293.0 * math.log(1 / STUDIO_RH))
    print(f"Kelvin diameter of a meniscus at {STUDIO_RH:.0%} RH: {r_kelvin * 1e9:.1f} nm (water condenses in any gap narrower)")
    A_ASP = 0.1e-6
    sigma = math.log(10) / 2  # a decade wide: ±1σ spans 10×
    F_rough = math.sqrt(10e-9 * 150e-9)
    bridge = lambda a: 4 * math.pi * gamma * a
    for R_um in (0.05, 0.1, 0.2, 0.3, 0.5, 1.0, 1.5, 3.0, 5.0, 10.0):
        R = R_um * 1e-6
        F_e = 1.5 * math.pi * W_ec * R
        F_h = A_pc * R / (6 * D0 ** 2) if R_um < 0.3 else F_rough
        F_w = bridge(min(R, A_ASP))
        print(f"  R {R_um:5.2f} µm: eraser {F_e * 1e9:7.1f} nN, paper {F_h * 1e9:6.1f} + water {F_w * 1e9:5.1f} nN; taken {P(F_e / (F_h + F_w), sigma):.2f} (dry {P(F_e / F_h, sigma):.2f})")
    Fv = 1.5 * math.pi * W_ec * VINE_D / 2 * 1e-6
    vine_lift = P(Fv / (F_rough + bridge(A_ASP)), sigma)
    print(f"VINE_LIFT (splinters R {VINE_D / 2} µm, bridge at a {A_ASP * 1e6:.1f} µm) {vine_lift:.3f}; a 0.05–0.2 µm asperity: {P(Fv / (F_rough + bridge(0.2e-6)), sigma):.2f}–{P(Fv / (F_rough + bridge(0.05e-6)), sigma):.2f}; dry {P(Fv / F_rough, sigma):.3f}")
    fine_dry = P(1.5 * math.pi * W_ec / (A_pc / (6 * D0 ** 2)), sigma)
    print(f"a fine particle (below the asperity scale), dry: {fine_dry:.3f} (independent of R); with water at R 0.1 µm: {P(1.5 * math.pi * W_ec * 0.1e-6 / (A_pc * 0.1e-6 / (6 * D0 ** 2) + bridge(0.1e-6)), sigma):.3f}")
    fine_wet = P(1.5 * math.pi * W_ec * 0.1e-6 / (A_pc * 0.1e-6 / (6 * D0 ** 2) + bridge(0.1e-6)), sigma)
    print(f"COMPRESSED_LIFT, the share of fines f = C_FINES {C_FINES}, water on both as for vine: {fine_wet:.2f} f + {vine_lift:.2f} (1 − f) = {fine_wet * C_FINES + vine_lift * (1 - C_FINES):.2f} (bounds {fine_wet:.2f}–{vine_lift:.2f} over f)")
    print(f"  (the same mix dry, estimates-audit-2 C4: {fine_dry * C_FINES + P(Fv / F_rough, sigma) * (1 - C_FINES):.2f}; inconsistent with vine's water term, not used)")
    # The water bridge held to the summits that touch (the water-bridge
    # switch; pastel_lift.md §7, pastel_lift.py §5). 4πγa at a smooth 0.1 µm
    # asperity is the smooth-sphere law; the fibre and the splinter are rough
    # far above the 1.5 nm Kelvin diameter, so water condenses only where
    # their summits touch: a splinter's summit a_p (52 nm, Persson & Biele
    # 2021's fracture-surface spectrum) crossing a fibre's cellulose fibril
    # aggregate a_f (10 nm radius, Fahlén & Salmén), an elliptical meniscus,
    # the bridge of a sphere of radius [(1/a_p + 1/a_f)/a_p]^(−1/2). It holds
    # until the Kelvin diameter passes the fibrils' size, 2a_f.
    A_P, A_F = 52e-9, 10e-9
    summit = lambda R, a_p=A_P, a_f=A_F: 1 / math.sqrt((1 / min(R, a_p) + 1 / a_f) / min(R, a_p))
    print(f"held to the summits: a {summit(1.5e-6) * 1e9:.1f} nm, the bridge {bridge(summit(1.5e-6)) * 1e9:.1f} nN (a_p 15–100 nm: {bridge(summit(1.5e-6, 15e-9)) * 1e9:.1f}–{bridge(summit(1.5e-6, 100e-9)) * 1e9:.1f}); spherical above {math.exp(-2 * gamma * 18.0e-6 / (8.314 * 293.0 * 2 * A_F)) * 100:.1f} % RH")
    vine_s = P(Fv / (F_rough + bridge(summit(VINE_D / 2 * 1e-6))), sigma)
    fine_s = P(1.5 * math.pi * W_ec * 0.1e-6 / (A_pc * 0.1e-6 / (6 * D0 ** 2) + bridge(summit(0.1e-6))), sigma)
    print(f"  vine {vine_s:.3f} (a_p 100–15 nm: {P(Fv / (F_rough + bridge(summit(VINE_D / 2 * 1e-6, 100e-9))), sigma):.3f}–{P(Fv / (F_rough + bridge(summit(VINE_D / 2 * 1e-6, 15e-9))), sigma):.3f}); fines {fine_s:.3f}; compressed {fine_s * C_FINES + vine_s * (1 - C_FINES):.3f}")
    print("pastel (chalk and pigment 1–20 µm, rough, dry): ", f"{P(1.5 * math.pi * W_ec * 0.5e-6 / F_rough, sigma):.2f}–{P(1.5 * math.pi * W_ec * 10e-6 / F_rough, sigma):.2f}", "(the engine's 0.55, an engine-wide question)")
    # Gum arabic's glass transition, pure gum: 62 °C at aw 0.33, 42.6 °C at
    # aw 0.54 (Righetto & Netto 2005, Int. J. Food Prop. 8:337); sticky only
    # near Tg. Straight between them, 92 K per unit aw.
    tg = lambda aw: 42.6 - (62.0 - 42.6) / (0.54 - 0.33) * (aw - 0.54)
    print(f"gum arabic's Tg at {STUDIO_RH:.0%} RH (aw = RH at equilibrium): {tg(STUDIO_RH):.1f} °C, {tg(STUDIO_RH) - 25:.0f} K above a 25 °C room: glassy, it bridges no new contact (grip 0)")
    print(f"  it reaches 25 °C only at aw {0.54 + (42.6 - 25) / ((62 - 42.6) / 0.21):.2f}: a room above about 73 % RH")

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

    print("\n== 5. Deposit: vine's hardness and the coverage of what a stick lays")
    # Dynamic hardness of wood charcoal (nine eucalypts, mean wood density
    # 0.79 g/cm³): 29 MPa as wood, about 3 MPa charred at 450 °C, 3.5 at 600
    # (de Abreu Neto et al. 2020, SN Appl. Sci. 2:1020, §3.3; Energy & Fuels
    # 2018, 3.05 MPa at 450 °C). Charcoal is fired at 450–500 °C (FAO 1987).
    print("vine σp: 3 MPa (measured 3.05–3.5 for wood charcoal at 450–600 °C)")
    # Coverage per volume of worn stick. The engine counts what a stick lays
    # as the volume it wore; pastel's deposit covers as 6 µm particles
    # (pastel.rs PARTICLE_UM). Willow charcoal is mostly pores: its true
    # density is 1.44 g/cm³ (willow char; Cutter & McGinnes, Wood and Fiber
    # Science), its apparent density 0.27–0.35 (Cerne, chars at 400–750 °C;
    # 0.68 × willow's 0.35–0.45). What it lays is the cell walls, splinters
    # about a wall (3 µm) thick: plates cover their area per their own
    # thickness, so a stick volume covers as 3/(1 − ε).
    for rho in (0.27, 0.30, 0.35):
        e = 1 - rho / 1.44
        print(f"  stick ρ {rho}: porosity {e:.2f}; its splinters cover as {VINE_D / (1 - e):.1f} µm per stick volume")
    print(f"VINE_COVER_UM 14 (12–16); vine now covers ×{(1 / 3 / 14) / (1 / 4 / 6):.2f} per force of what it did at σp 4 and 6 µm")
    # Compressed: the compact (60 % solids) gives up its 3 µm particles;
    # grains cover their area at 1.5/d of their volume: d/(1.5 × solids).
    print(f"COMPRESSED_COVER_UM {COMP_D / (1.5 * C_SOLIDS):.1f} (3 µm grains of a {C_SOLIDS:.0%}-solids compact)")
