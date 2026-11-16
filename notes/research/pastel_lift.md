# Pastel's eraser lift from adhesion

How readily one press of a kneaded eraser takes a pastel stick's deposit: the cells'
`lift`, which `graphite.rs` `erase` multiplies into its take. Before this, every pastel
stick laid at `PASTEL_LIFT` = 0.55, a number derived from nothing. Charcoal's lifts (#321)
came from adhesion (charcoal.md §4). This note puts pastel on the same law.

Switch: `--@ pastel-lift` (every new painting). A log without it replays at 0.55. The
water bridge in the law is held to the summits that touch with a second switch,
`--@ water-bridge` (§7), which moves charcoal with pastel.
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
  so changing it is one revision for both, not a pastel-only change. It is made in §7,
  behind its own switch.

## 6. Moved pastel

With the switch, pastel that a finger, a dry or wet brush, a puff or a blade moves keeps
its lift (`pastel::Carried`, volume-weighted over everything carried). Charcoal already
did this (estimates-audit-2 C10). Lift belongs to the particles, not the gesture. Without
the switch, moved pastel is laid at 0.55, as before.

## 7. The water bridge held to the summits that touch (`--@ water-bridge`)

§1 takes the bridge at one smooth 0.1 µm asperity: 4πγa = 90 nN. That is the
smooth-sphere law, F = 4πγR cos θ, and it holds only where both surfaces are smooth on
the scale of the meniscus. At 50 % RH the Kelvin diameter is 1.53 nm; a paper fibre and
a pastel or charcoal particle are rough far above that, so water condenses only between
the summits that touch. This is the asperity regime (Halsey & Levine 1998; Charlaix &
Ciccotti 2010). Code: `adhesion::Bridge::Asperity`, `adhesion::bridge_radius`; numbers:
`pastel_lift.py` §5.

**What measurement says** [M]:
- *Rough particles, humid air, pull-off far below 4πγR.* Irregular grains of 40–65 µm on
  a smooth glass slide (Ra 4 nm) at 30–40 % RH: 55–150 nN in all (centrifuge, Nagaashi,
  Aoki & Nakamura 2021, table 5), against the 10–20 µN that 4πγR gives at their size;
  glass beads (Ra 24 nm) 560 nN.
- *Rough particles show little humidity dependence.* Limestone and hydrated alumina
  particles, particle on particle, 10–90 % RH: no notable change; 5–15 nN in dry air for
  all five powders (Jones, Pollock, Geldart & Verlinden 2003). Arizona road dust of
  15–50 µm on solar glass: the force rises with RH on smooth glass but hardly on rough
  (22–321 nm rms), and it doesn't grow with particle size (Moutinho et al.). AFM tips
  and silica spheres on glass and silicon: the rise with RH depends on roughness (Jones,
  Pollock, Cleaver & Hodges 2002). Nanoscale roughness limits meniscus formation at low
  humidity (Rabinovich et al. 2002). Changes of tip geometry on the sub-10 nm scale
  change the force–humidity curve completely (Farshchi-Tabrizi et al. 2006).
- *From a measured roughness spectrum.* Persson & Biele (2021) compute the capillary
  pull-off of a rough particle on a flat at about 40 % RH from a measured fracture
  surface's spectrum (granite, self-affine, H 1, extrapolated to the atomic scale): 47 nN,
  independent of size; 13 and 6 nN for spectra ×2 and ×3 rougher (meteorites).

**The derivation** [D]:
- *The fibre's summit:* at the bridge's scale a fibre's surface is cellulose fibril
  aggregates, 15–25 nm across by AFM (18 nm in spruce wood, 23 nm in kraft pulp; Fahlén
  & Salmén 2003, 2005): a cylinder, a_f = 10 nm (7.5–12.5).
- *The particle's summit:* 47 nN = 4πγa_p gives a_p = 52 nm for a fracture surface
  (Persson & Biele). The meteorite spectra give 15 and 7 nm. Jones's limestone, with no
  notable humidity dependence, wants no larger. Coccolith calcite's faces are smoother
  (rms 0.45–2.8 nm over 200 nm; JACS 2026), but chalk's carry clay laths 1–4 nm thick
  (*GCA* 2012). A sawdust char is 7 nm rms over 2 µm. Take a_p = 52 nm
  (15–100) [D from a proxy surface; no AFM of a whiting or vine particle was found].
- *The geometry:* a sphere a_p crossing a cylinder a_f. The gap is
  x²(1/a_p + 1/a_f)/2 + y²/2a_p; water fills it where it is below d_K, at the Laplace
  pressure 2γ/d_K. The area π d_K/√(c₁c₂) gives 2πγ·2a, with a = [(1/a_p + 1/a_f)/a_p]^(−1/2).
  For wetted surfaces the bridge is 2πγa(cos θ_p + cos θ_f), §1's form, with a in place of
  the smooth asperity: **a = 20.9 nm, 18.9 nN** (7.8–30 nN over the ranges).
- *Humidity:* the bridge is the summits' while d_K is below the fibrils' size. At 2a_f =
  20 nm the gaps between fibrils fill and one meniscus takes the particle whole: the
  spherical regime, a jump (Charlaix & Ciccotti). That happens at **94.8 % RH**. Below it
  the bridge doesn't change with humidity. Above the 0.16 % RH where water first condenses,
  this matches the plateaus measured on rough contacts.

**Results** (50 % RH):

| | smooth (§4) | held to the summits | over a_p 15–100 nm |
|---|---|---|---|
| bridge | 90.5 nN | 18.9 nN | 7.8–30 nN |
| whiting | 0.608 | **0.763** | 0.742–0.791 |
| ultramarine | 0.572 | 0.742 | |
| lead white | 0.599 | 0.758 | |
| vermilion | 0.690 | 0.804 | |
| bone black | 0.548 | 0.727 | |
| vine (R 1.5 µm) | 0.85 | **0.960** | 0.948–0.972 |
| compressed's fines (R 0.1 µm) | 0.10 | 0.286 | |
| compressed (f = 0.5) | 0.48 | **0.623** | |

The proposal this answers estimated 25 nN, whiting about 0.75 and vine about 0.95. The
derivation lands a little below on the bridge and a little above on the lifts, inside its
own range.

The practitioners' order (vine, then graphite, then compressed; charcoal.md §4) now holds
by lift alone: vine 0.96, graphite 0.9, compressed 0.62.

**One caveat** [open]. Nagaashi's 10–150 nN, the law's rough hold, was measured in air at
30–40 % RH, so it may already hold some of the water. Persson & Biele read the measured
forces as mostly capillary. If they are right, adding any bridge counts some water
twice, and the lifts would be higher still. The bridge here is the smaller error either
way.

**The switch.** `--@ water-bridge` (every new painting), not a reuse of `--@ pastel-lift`.
Charcoal (#321, `--@ charcoal`) may merge first, and its logs must keep 0.85 and 0.48.
Folding the bridge into `--@ pastel-lift` would keep them only while no painting with
that line exists: once #331 merged and was painted with, its whiting at 0.61 would
replay differently. It would also tie charcoal to a pastel switch. The palette carries
the switch (`Palette::water_bridge`) and a save its header (`water_bridge=1`). The canvas
needs no engine bit: a stick's lift is set where it is made, and the cells keep it.

## Estimates left, and why

- The pigments' own size distributions. The engine has one size for all pigments; a
  measured distribution per pigment would change pure-pigment sticks.
- The fibre asperity radius, 0.1 µm (0.05–0.2), without `--@ water-bridge`. It is the same
  for charcoal; Nagaashi's effective contact radii are 0.14–0.5 µm. With the switch, the
  particle's summit radius (52 nm, from a granite fracture surface's spectrum as a proxy):
  an AFM roughness spectrum of a whiting grain, a pigment and a vine splinter would set it.
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
- §7, the water bridge:
  - Nagaashi, Aoki & Nakamura, "Cohesion of regolith: Measurements of meteorite powders",
    *Icarus* (2021), doi:10.1016/j.icarus.2021.114357, arXiv:2101.11837 (table 5; measured at 30–40 % RH).
  - Persson & Biele, "On the stability of spinning asteroids", arXiv:2110.15258 (2021):
    the capillary pull-off of rough "granite" particles, 47/13/6 nN.
  - Jones, Pollock, Geldart & Verlinden, "Inter-particle forces in cohesive powders
    studied by AFM: effects of relative humidity, particle size and wall adhesion",
    *Powder Technol.* 132 (2003) 196–210, doi:10.1016/S0032-5910(03)00072-X (abstract).
  - Jones, Pollock, Cleaver & Hodges, "Adhesion forces between glass and silicon surfaces
    in air studied by AFM: effects of relative humidity, particle size, roughness, and
    surface treatment", *Langmuir* 18 (2002) 8045–8055, doi:10.1021/la0259196.
  - Moutinho, Jiang, To, Perkins, Muller, Al-Jassim & Simpson, "Adhesion mechanisms on
    solar glass: effects of relative humidity, surface roughness, and particle shape and
    size", accepted manuscript (DOE public access, OSTI 1375106),
    https://www.osti.gov/servlets/purl/1375106.
  - Rabinovich, Adler, Esayanur, Ata, Singh & Moudgil, "Capillary forces between surfaces
    with nanoscale roughness", *Adv. Colloid Interface Sci.* 96 (2002) 213–230,
    doi:10.1016/S0001-8686(01)00082-3 (abstract).
  - Farshchi-Tabrizi, Kappl, Cheng, Gutmann & Butt, "On the adhesion between fine
    particles and nanocontacts: an atomic force microscope study", *Langmuir* 22 (2006)
    2171–2184, doi:10.1021/la052760z (abstract).
  - Charlaix & Ciccotti, "Capillary condensation in confined media", *Handbook of
    Nanophysics* (2010), ch. 12 (the asperity, roughness and spherical regimes, after
    Halsey & Levine 1998).
  - Fahlén & Salmén 2003 (*J. Mater. Sci.*) and 2005 (*Biomacromolecules* 6:433), fibril
    aggregates 15–25 nm by AFM, as cited by later papers (the originals weren't reached).
  - "Spatial relations between coccoliths and their confining membrane during crystal
    morphogenesis", *J. Am. Chem. Soc.* 148 (2026) 14526 (coccolith calcite rms
    roughness); "Nano sized clay detected on chalk particle surfaces", *Geochim.
    Cosmochim. Acta* (2012) (AFM; laths 1–4 nm thick).
  - Sawdust biochar, AFM Rq 7.1 nm over 1.6–2 µm: "Sawdust-derived biochar and Ag₂O
    nanoparticles for adsorptive desulfurization…", *J. Nanostructures* (Univ. Kashan),
    https://jns.kashanu.ac.ir/article_115665.html.
- estimates-audit-2.md (branch `notes/estimates-audit-2`), engine-wide fault 2.
