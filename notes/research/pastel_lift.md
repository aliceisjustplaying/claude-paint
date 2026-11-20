# Pastel's eraser lift from adhesion

How readily one press of a kneaded eraser takes a pastel stick's deposit: the cells'
`lift`, which `graphite.rs` `erase` multiplies into its take. Before this, every pastel
stick laid at `PASTEL_LIFT` = 0.55, a number derived from nothing. Charcoal's lifts (#321)
came from adhesion (charcoal.md §4). This note puts pastel on the same law.

Switch: `--@ pastel-lift` (every new painting). A log without it replays at 0.55.
Code: `crates/paint/src/adhesion.rs` (the law), `Palette::pastel_lift` (a stick),
`pastel::Carried` (moved pastel). Numbers: `uv run notes/research/pastel_lift.py`, written
separately from the Rust so that each checks the other.

Tags: [M] measured, [D] derived, [E] estimate.

## 1. The law (charcoal's, unchanged)

A particle goes with the eraser if the eraser pulls harder than the fibre holds it
(charcoal_optics.py §3, charcoal.md §4):
- **Eraser** [D]: the soft putty wraps the particle. JKR pull-off 1.5πWR, with
  W = A_ep/12πD₀², A_ep = √(A_eraser A_p), A_eraser 6.5 ×10⁻²⁰ J (a hydrocarbon polymer),
  D₀ 0.165 nm.
- **Fibre, coarse** [M]: a rough particle above the asperity scale holds 10–150 nN
  whatever its size (Nagaashi et al. 2021; pastel_brushing.md §1). Take the geometric
  middle, 39 nN, spread a decade wide (lognormal, ±1σ = ×10).
- **Fibre, fine** [D]: below R 0.3 µm a particle sits on one smooth asperity: DMT,
  √(A_cel A_p) R/6D₀², A_cel 5.8 ×10⁻²⁰ J.
- **Water** [D]: at the studio's air (`climate::STUDIO`, 20 °C, 50 % RH) the Kelvin
  diameter is 1.53 nm, wider than the contact's gap, so water condenses at every contact
  (the law's only humidity condition: above 0.16 % RH). A bridge at an asperity of radius
  a = 0.1 µm (0.05–0.2) holds 2πγa(cos θ_p + cos θ_fibre) more. The fibre is wetted. The
  eraser's putty is not, so it gets no bridge.

The Rust gives vine's splinters 0.852 (`VINE_LIFT` 0.85) and compressed's fines 0.099
(0.10) (`adhesion::tests::charcoal_follows_the_law`), so the two media follow one law.

## 2. What the audit said the old figure left out

Estimates-audit-2, engine-wide fault 2, said the "0.88–1" in charcoal.md overstated
pastel's lift. That figure came from the law dry, for 1–20 µm particles. It left out three
things, and all three are now in:
1. **Capillary bridges** on hydrophilic chalk and fibre: the water term above.
2. **The fine fraction.** Whiting is 44–47 % by volume below 2 µm. The stick is given
   whiting's measured distribution (below).
3. **Cellulose–chalk Hamaker 7.4–7.7, not cellulose's 5.8.** The particle's own constant
   now enters the eraser's pull and the fine particles' hold (√(A_cel A_calcite) = 7.65).

## 3. A stick's particles

- **Sizes [M].** The particles are whiting's distribution, as #322 gave it (wet_pastel.md
  §1). Champagne/Meudon whiting is a volume lognormal through d50 2.4–2.7 µm and D97
  13–15 µm, so σ_ln = 0.87–0.94 and d₃₂ = 1.64–1.75 µm. Taken: d₃₂ 1.7
  (`wet_pastel::PARTICLE_D_UM`), σ_ln 0.905 (`pastel::PARTICLE_SIGMA`).
  - The pigments get the same distribution, as the dry and wet Mie tables already assume
    one particle size for every pigment [E]. No pigment-by-pigment size distribution is
    in the engine. This is the largest estimate left for pure-pigment sticks.
- **Weighting [D].** What the eraser takes is a share of the mark's coverage, and
  coverage per volume goes as 1/d (the engine's own coverage law, and why #322 uses d₃₂).
  So the take is averaged over the distribution by projected area: a lognormal with
  median d₃₂e^(−σ²/2) = 1.13 µm. By volume (the audit's rough version) it would be 0.81.
- **Composition [D].** By volume, the stick is its pigments plus the whiting its whites
  stand for. The whiting share is the one that gives the stick's dry colour its
  luminance by Kubelka–Munk (`dry::stick_volumes`, the same share `dry::particle_index`
  uses). A white-only pile is all whiting; a pile with no white is all pigment.
- **Hamaker constants [M/D].**
  - Whiting: calcite 10.1 ×10⁻²⁰ J, measured (Bergström 1997).
  - Blacks (`dry::carbon_ratio`: vine, bone, ivory, lamp): char's 6.0, as charcoal uses.
  - Lakes: their alumina substrate's index, 1.57.
  - All others: Tabor–Winterton from the pigment's refractive index, which `dry.rs` already
    holds (its measured index with `--@ dry-lakes`). The UV absorption frequency is set by
    class (Bergström 1997, via Lefèvre et al.):
    - below n 1.85, 1.96 ×10¹⁶ rad/s, the mean for wide-gap insulators (silica 2.03,
      alumina 2.00, BeO 2.11, calcite 1.90, MgO 1.71, quartz 2.03);
    - above it, 0.815, the mean for high-index oxides (TiO₂ 0.735, ZnO 0.895).

    The static ε is taken as n², its lower bound; it contributes under 3 %. Against
    measured constants (10⁻²⁰ J), calculated (measured): calcite 9.6 (10.1), TiO₂ 15.7
    (15), ZnO 8.1 (9.2), MgO 13.3 (12.1), silica 6.1 (6.5), hematite 23.3 (≈ 25).
- **Wetting [E].** Mineral and charred particles are wetted, cos θ 1, as charcoal is.
  Quinacridone (permanent alizarin) and asphaltum (bitumen) are organics that water
  stands on, taken at 90° (cos θ 0). No contact angle on either powder was found.

## 4. Results (50 % RH, a = 0.1 µm)

| Stick | Hamaker (10⁻²⁰ J) | lift |
|---|---|---|
| whiting (any white-only stick) | 10.1 | **0.608** |
| ultramarine | 7.4 | 0.572 |
| Prussian blue | 8.9 | 0.594 |
| a lake (alumina) | 9.2 | 0.597 |
| green earth | 10.5 | 0.613 |
| cobalt blue | 14.0 | 0.642 |
| chrome yellow | 14.8 | 0.648 |
| red earth | 19.1 | 0.672 |
| vermilion | 23.3 | 0.690 |
| permanent alizarin (unwetted) | 9.7 | 0.696 |
| vine or bone black (carbon) | 6.0 | 0.548 |
| *vine charcoal (splinters R 1.5 µm)* | 6.0 | *0.85* |

A tint lies between its pigment and whiting by volume.

**Bounds for whiting:**
- the asperity radius, 0.2–0.05 µm: 0.48–0.70 (the largest uncertainty);
- σ_ln 0.87–0.94: 0.60–0.62;
- d₃₂ 1.64–1.75: 0.60–0.62;
- the rough hold scaled with √A_p, as the eraser's pull is: 0.59;
- a bone-dry room: 0.82;
- by volume, not by area: 0.81.

The audit estimated 0.6–0.75 and proposed 0.70. The old 0.55 sits just under the derived
range, as the audit said.

**The practitioners' order holds** [P]. Vine is "easiest to erase" (charcoal.md §4), and
it lifts at 0.85, above every pastel. The difference is that vine's splinters are
coarser than whiting's fines.

## 5. Checks against measurement

- **Duncan, Vicenzi, Lam & Brogdon-Grantham 2023** (*JAIC* 62:152–167,
  doi:10.1080/01971360.2022.2148903): ten dry-cleaning materials on lamp black over
  papers of three roughnesses, measured by spectrophotometry and photomicrographs. Only
  the abstract and the companion *Microscopy Today* 30(1):30–33 note could be reached; the
  full text is paywalled. They report that tacky materials (kneaded erasers) are "very
  effective" on lamp black and reach clumps deep in rough paper's texture. That is a
  qualitative check: lamp black is a sub-0.1 µm soot in clumps, not a pastel. The
  removal fractions in the paper's tables would calibrate the eraser's side of the law.
  **Open: get the full text.**
- **The water bridge may be too strong** [M, open]. Moutinho et al. measured a 20 µm
  sphere on rough glass. At 22 nm rms the pull-off went from ≈ 120 to ≈ 150 nN between
  0 and 67 % RH; at 321 nm rms it stayed near 60 nN. Jones et al. 2003 found no
  significant change with humidity (10–90 %) for limestone particle pairs
  (pastel_brushing.md §1). Measured on rough contacts, then, the bridge adds about
  20–30 nN. The law's 2πγa·2 at a = 0.1 µm adds 90 nN. With a 25 nN bridge instead,
  whiting would lift about 0.75 and vine about 0.95. Pastel and charcoal share this term,
  so changing it is one revision for both, not a pastel-only change. It isn't made here.

## 6. Moved pastel

With the switch, pastel that a finger, a dry or wet brush, a puff or a blade moves keeps
its lift (`pastel::Carried`, volume-weighted over everything carried). Charcoal already
did this (estimates-audit-2 C10). Lift belongs to the particles, not the gesture. Without
the switch, moved pastel is laid at 0.55, as before.

## Estimates left, and why

- The pigments' own size distributions. The engine has one size for all pigments; a
  measured distribution per pigment would change pure-pigment sticks.
- The fibre asperity radius, 0.1 µm (0.05–0.2). It is the same for charcoal; Nagaashi's
  effective contact radii are 0.14–0.5 µm.
- The organics' wetting (permanent alizarin, bitumen).
- The class split for Tabor–Winterton's ω_e at n 1.85.
- The lognormal spread of the hold (a decade), the eraser's Hamaker constant (6.5) and
  the eraser's modulus (JKR conformity holds over 0.03–0.3 MPa). All are charcoal's.

## Sources

- Bergström, "Hamaker constants of inorganic materials", *Adv. Colloid Interface Sci.* 70
  (1997) 125–169, doi:10.1016/S0001-8686(97)00003-1. Its ω_e and A values were read from
  the table in Lefèvre et al., "Calculation of Hamaker constants applied to the deposition
  of metallic oxide particles…", Heat Exchanger Fouling and Cleaning (2009),
  https://heatexchanger-fouling.com/wp-content/uploads/2021/09/16_Lefevre_Hamaker_F.pdf,
  which also gives De Mesquita et al. 2003 for hematite.
- Bergström et al. 1999 (cellulose); Maurer, Mersmann & Peukert 2001 (carbon); Nagaashi
  et al. 2021; Moutinho et al.; Jones et al. 2003. Each as cited in charcoal.md and
  pastel_brushing.md.
- Israelachvili, *Intermolecular and Surface Forces* (Tabor–Winterton approximation).
- Duncan, Vicenzi, Lam & Brogdon-Grantham, *JAIC* 62 (2023) 152–167; Duncan, Vicenzi &
  Brogdon-Grantham, "Imaging rough paper to evaluate methods for soot removal",
  *Microscopy Today* 30(1) (2022) 30–33, doi:10.1017/S1551929521001516.
- Whiting's size distribution: wet_pastel.md §1 (Art-Chimie, Omya, Kenzai specifications).
- estimates-audit-2.md (branch `notes/estimates-audit-2`), engine-wide fault 2.
