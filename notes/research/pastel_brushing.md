# Brushing loose pastel off paper with a dry hog-bristle brush

Research notes for the pastel simulator's "brush-off" tool. Compiled 2026-10-05.

**Tags:** **[M]** measured, cited. **[D]** derived here from measured inputs, arithmetic shown. **[E]** estimate; treat as tunable.

**Bottom line.**
- A bristle tip pushes sideways with 10³–10⁵ times the force needed to roll a single pastel particle. Removal is therefore limited by **reach**, i.e. whether the tip touches the particle, not by force.
- Bristles also break loose soft-pastel crumbs (agglomerates) of about 100 µm.
- Pastel sitting deeper in a pore than the tip can reach stays put. That is the "ghost" painters describe.
- Nobody has measured redeposition for this case; it is a gap.

---

## 1. Adhesion of one particle

### Hamaker constants (in air/vacuum)
| Material | A₁₁ (10⁻²⁰ J) | Tag, source |
|---|---|---|
| Calcite CaCO₃ | 10.1 (10.19 ± 0.55 measured); another table gives 24 | M: Bergström 1997; Médout-Marère via Lefèvre |
| Kaolinite | 7.54 (0.73 across water) | M (Lifshitz): Weber & Kaufhold 2021 |
| Cellulose | 5.8 (0.80 across water) | M (ellipsometry + Lifshitz): Bergström et al. 1999 |
| Hematite α-Fe₂O₃ | ≈ 25; iron oxides across water 3.3–3.9 | M: De Mesquita via Lefèvre; Faure et al. 2011 |

- **[D] Mixed pairs.** Use A₁₂ ≈ √(A₁₁A₂₂).
  - CaCO₃–cellulose ≈ 7.7
  - kaolin–cellulose ≈ 6.6
  - Fe₂O₃–cellulose ≈ 12 (all ×10⁻²⁰ J)

### Smooth-sphere formulas, plugged in [D]
D = 0.4 nm, γ_water = 0.072 J/m², θ = 0.
- F_vdW (sphere–flat) = A₁₂R/(6D²)
- F_cap = 4πγR cosθ (upper bound, full meniscus)
- JKR pull-off F = 1.5πWR, with W = A/(12πD₀²). D₀ = 0.165 nm gives W ≈ 0.075 J/m²; D₀ = 0.4 nm gives 0.013 J/m².

| Particle radius R | F_vdW CaCO₃–cellulose | kaolin | Fe₂O₃ | F_cap (ideal) | Weight (calcite) |
|---|---|---|---|---|---|
| 0.5 µm | 40 nN | 34 nN | 63 nN | 0.45 µN | 1.4×10⁻¹⁴ N |
| 1 µm | 80 nN | 69 nN | 125 nN | 0.9 µN | 1.1×10⁻¹³ N |
| **2.5 µm (5 µm chalk)** | **200 nN** | 172 nN | 314 nN | **2.3 µN** | 1.7×10⁻¹² N |
| 5 µm | 400 nN | 344 nN | 627 nN | 4.5 µN | 1.4×10⁻¹¹ N |
| 10 µm | 800 nN | 690 nN | 1.25 µN | 9 µN | 1.1×10⁻¹⁰ N |

- **[D] Gravity is irrelevant.** Smooth adhesion is about 10⁵ × weight at 5 µm.
- **[M] Smooth theory checks out.**
  - Smooth SiO₂ spheres on smooth glass at 0 % RH (NREL AFM; Moutinho et al.): 5 µm sphere measured 314 nN (theory 238); 20 µm sphere measured 684 nN (theory 952).
  - Capillary: at 67 % RH the 20 µm sphere on 0.3 nm-rms glass reached about **1.9 µN**. That is about 20 % of the ideal 4πγR (9 µN).
  - Force was flat up to about 30 % RH.
- **[D] The meniscus at 50 % RH is tiny.** Kelvin radius r_K ≈ 0.52 nm/|ln RH|: 0.57, 0.75 and 1.0 nm at 40, 50 and 60 % RH. Water bridges therefore form only at asperity contacts within about 1–2 nm of each other. On rough particles capillary force scales with **asperity radius**, not particle radius.

### Roughness: the numbers that matter
- **[M] Sphere on rough glass** (Moutinho et al., 20 µm SiO₂):

  | Glass rms | Force at 0 % RH | Force at 67 % RH |
  |---|---|---|
  | 0.3–1.7 nm | 700–850 nN | 1.2–1.9 µN |
  | 8 nm | ≈ 350 nN | — |
  | 22 nm | ≈ 120 nN | ≈ 150 nN |
  | 321 nm | ≈ 20–60 nN | ≈ 60 nN |

  - That is **6× (22 nm) to 15–40× (321 nm) lower**.
  - On rough glass, humidity barely mattered.
  - With real irregular dust (15–50 µm), adhesion **did not grow with particle size**.
- **[M] Irregular 40–65 µm particles in open air** (meteorite powder, sand, glass powder; centrifuge; Nagaashi et al. 2021):
  - Typical cohesion **0.05–0.15 µN**.
  - That is more than 10× below JKR spheres; particle shape costs about 1/7 and fine roughness 1/10–1/25.
  - Effective contact curvature radius is **0.14–0.5 µm**.
  - Prediction: above this size, force is **independent of particle size**.
- **[M] Limestone (CaCO₃) particle–particle pull-off by AFM** (Jones et al. 2003): **5–15 nN** in dry air, with **no significant RH dependence** over 10–90 %.
- **[M] Literature range for roughness.** Rabinovich et al. 2000 model nanoscale roughness as F = (AR/6H₀²)[1/(1+58Rr_rms/λ²) + 1/(1+1.82r_rms/H₀)²], with λ the asperity spacing. They report 1–2 orders of magnitude reduction for rms ≥ ~10 nm. A related study found a 712 nm roughness cut tungsten microparticle adhesion 100× (search-summarised).
- **[D] Recommended simulator value.** Use **F_adh ≈ 10–150 nN per particle–fibre contact**, independent of size for R ≳ 1 µm, with a log-normal/Weibull spread about 1 decade wide. Fe₂O₃ or ultramarine pigment changes it by <2× (Hamaker). Raise by ×1.5–3 from 30 % to 60 % RH only for smooth, fine particles (kaolin plates); leave chalk unchanged.

---

## 2. Detachment by a moving contact

- **[M] Rolling dominates.**
  - Wafer brush scrubbing (Xu et al. 2004, 34 nm SiO₂): force analysis and experiment agree that particles "cannot be lifted directly by a brush"; rolling is the removal path.
  - Rolling resistance of silica microspheres (R ≈ 10 µm, nanoindenter; Fuchs et al. 2014): rolling-friction coefficient ≈ **(1.6–3.3)×10⁻³**, against sliding μ ≈ **0.23**.
  - Heim et al. 1999 (R ≈ 0.95 µm SiO₂): critical rolling force **8.5 ± 1.6 ×10⁻¹⁰ N**, about 10⁻² of the pull-off force.
- **[D] Rolling criterion.** F_t·R ≥ F_adh·a. The JKR zero-load contact radius is a₀ = (6πWR²/K)^{1/3}, K = (4/3)E*. Take calcite at 70 GPa against fibre wall at 8 GPa [E], so E* ≈ 7.9 GPa.

  | R | JKR pull-off | a₀ | Lateral force to roll (pushed at centre height) |
  |---|---|---|---|
  | 2.5 µm | 0.15–0.9 µN | 50–95 nm | **3–33 nN** (a/R ≈ 0.02–0.04) |
  | Rough, F_adh = 10–150 nN | — | — | **≈ 0.2–6 nN** |

- **[D] Force the bristle supplies.**
  - **[M] Bristle properties.** Pig hair is ≈ 160 µm mean diameter, E ≈ 6.4 GPa (Biosystems Eng. 2014, search-summarised). Keratin generally runs 2.5–6 GPa.
  - Per-bristle normal force is capped by Euler buckling: F_cr = π²EI/(4L²), I = πd⁴/64, fixed-free.

    | Free length | d = 100 µm | d = 160 µm | d = 250 µm (base) |
    |---|---|---|---|
    | 10 mm | 0.36–0.78 mN | 2.4–5.1 mN | — |
    | 20 mm | — | 0.6–1.3 mN | 3.6–7.6 mN |

  - Cantilever stiffness 3EI/L³ = 0.01–0.6 N/m at the tip.
  - **[E]** About 1 N of hand load shared by a few hundred touching bristles gives ~1–5 mN each, so most bristles are buckled or bent. With keratin-on-paper μ ≈ 0.3 [E], tangential tip force is **≈ 0.03–1.5 mN**.
  - Nominal tip pressure at 1 mN on a 100 µm tip is ≈ 0.13 MPa.
  - **Consequence:** the margin over the 1–30 nN rolling threshold is 10³–10⁶. **Any loose particle the tip touches goes.**
- **[M] The closest dry-brush data** (Chen et al. 2018, Sol. Energy Mater. Sol. Cells 179:247): a POM tip swept 2.3 µm dust off glass.
  - Dry dust: removal >90 %, **independent of load**.
  - Humidified dust: removal fell; more load restored it.
  - This is consistent with a threshold far below the applied force on dry, smooth substrates.
- **[M] Electrostatics are secondary below ~30 µm.** In dust-removal literature, vdW dominates electrostatics for particles under ~30 µm (search-summarised review).
  - **[D]** A 5 µm particle charged to the air-breakdown limit (σ ≈ 2.7×10⁻⁵ C/m²) carries q ≈ 2×10⁻¹⁵ C. Its image force is ≈ 1–2 nN: small next to adhesion, but not zero for re-attachment.

---

## 3. Conservation data: fragility, fixed vs unfixed

- **[M] Sauvage, Wei & Martinez 2018** (*Stud. Conserv.* 63 sup1 S418):
  - **Setup.** 30 mock-ups of 18th-c. pastel with Girault pastels on unsized rough paper, tensioned to 4 N/mm². Sinusoidal mode-1 excitation. Failure defined as **1 % surface change**.
  - **Unfixed.** Mean life **2.8×10⁵ cycles** at higher displacement amplitude and **1.8×10⁶** at lower. Three specimens survived about **10⁷ cycles**, which suggests an endurance limit.
  - **Fixed** (2 coats Sennelier synthetic fixative), at **4.3 mm amplitude**: no failure in 4 days, outliving unfixed specimens.
  - **What moves.** Loss is **agglomerates**. Black pastel (1–20 µm particles plus 60 nm grains, "not as well embedded") moved; pink (needle-shaped particles) did not.
  - **Gap.** Drive frequency and acceleration are not given in the paper, so no g threshold can be recovered.
- **[M] Thickett 2002** (British Museum, ICOM-CC): building vibration caused loss of **loosely bound pigment at 0.2–0.3 g**; damage across objects at 0.2–0.6 g. These are not pastels, but they are the nearest friable-paint data.
- **[M] Wei, Sauvage & Wölk 2014:** generic low-risk limit of **2 mm/s**.
- **[D] Size check.** Shaking needs a·m > F_adh.
  - A 5 µm particle at 0.1 µN would need ~10⁴ g, so it never shakes off.
  - A 200 µm crumb at ~50 % porosity weighs 5.6×10⁻⁸ N. At 2–10 g that is 0.1–0.6 µN, matching measured irregular-particle cohesion.
  - So vibration loss is **agglomerate loss**, as observed.
- **[M] Fixatives, peel-tape test** (ICR Rome thesis, search-summarised): Laropal K80 4 % gave **no** pigment pick-up; Klucel EXF 1 % gave **2×10⁻³ mg/cm²**. Unfixed controls were not reported in the summary. **No casein or shellac data found.**
- **[M] Practice.**
  - Conservators do not brush powdery media (NEDCC leaflet, search-summarised). Fixed pastel may take only a very soft brush.
  - Pastellists brush corrections off with stiff brushes. This leaves a **"ghost image"** or "hint of colour", and greens stain worst (kemstudios blog; Jackie Simmonds).

---

## 4. Binder and agglomerate strength

- **[M] Without binder, chalk barely holds together** (Cabiscol et al. 2020).
  - Binderless limestone pressed at **10–50 MPa did not make stable tablets**.
  - At 100–400 MPa, tensile strength was σ_t = σ₀(1 + 35 µm/d₅₀), with σ₀ = 114, 227 and 520 kPa respectively, i.e. ≈ 0.1–1 MPa.
  - Pastel sticks are formed at low pressure and are about 60 % porous (see `pastel_stick_tribology.md`). **Their strength is essentially all binder.**
- **[M] Gum binders** (tablet literature): tablet tensile strength rises and friability falls with gum concentration. Tragacanth keeps pellets intact better than acacia. **No numbers exist for 1–3 % tragacanth in chalk.**
- **[D] Rumpf, without binder.** σ_t ≈ ((1−ε)/ε)·F/d². With F = 5–15 nN (Jones et al.) and ε = 0.5–0.6:
  - d = 5 µm: **130–600 Pa**
  - d = 2 µm: 0.8–3.8 kPa
  - d = 1 µm: 3–15 kPa
- **[M] Stick-strength proxies.** Blackboard chalk breaks at ≈ 5.5 MPa; chalk rock UCS is 1–10 MPa (see the tribology note).
- **[E] Strength scale for the simulator.**

  | Material | Tensile strength |
  |---|---|
  | Soft-pastel stick / fresh crumb (1–2 % tragacanth) | **10–100 kPa** (crumbles between fingers) |
  | Loose re-deposited dust aggregates | 0.1–10 kPa (Rumpf) |
  | Hard pastel / Conté | 0.3–3 MPa |

- **[D] Breaking a crumb.** F ≈ σ_t·d².
  - Soft crumb at 10⁴–10⁵ Pa: 100 µm → **0.1–1 mN**; 300 µm → 0.9–9 mN.
  - The per-bristle tangential force is 0.03–1.5 mN, so **bristles partly fragment 50–150 µm soft crumbs and mostly roll or drag larger ones whole.**
  - Fixative-bound clusters are much stronger and are not quantified (gap).

---

## 5. How deep the tip reaches into pores

- **Inputs.** Paper fibres are 20–40 µm wide; pore depth is exponential with mean 5–15 µm (`paper_surface.md`).
- **[D] Geometric reach.** A spherical tip of radius r over a slot of width w sinks h = r − √(r² − w²/4):

  | Tip radius r | w = 20 µm | w = 30 µm | w = 40 µm | w = 60 µm |
  |---|---|---|---|---|
  | 25 µm | 2.1 µm | 5.0 µm | 10 µm | full depth |
  | 50 µm | 1.0 µm | 2.3 µm | 4.2 µm | 10 µm |
  | 75 µm | 0.7 µm | 1.5 µm | 2.7 µm | 6.3 µm |

  - A 50–150 µm tip clears only the **top 1–5 µm** of typical inter-fibre pores. That is less than the mean pore depth, so about 30–80 % of pore-held pastel is out of reach for exponential depth with mean 5–15 µm [D].
- **[M, qualitative] Split tips.** Hog bristles have split ("flagged") tips. **[E]** Flag branches of about 10–40 µm reach further.
- **[E] Fibre give.** Paper fibres deflect under mN loads, adding perhaps 1–3 µm. Repeated strokes at random angles raise the chance of contact but not the reach.
- **[M] Consistent with practice.** The ghost left after brushing, and the worst stains coming from fine organic pigments (sub-µm, deep), both fit this picture.
- **No direct measurement of bristle penetration into paper or textile pores was found.**

---

## 6. Redeposition and charge

- **[M, qualitative] Brushes load up and re-deposit.**
  - Brushes load with removed particles and cross-contaminate (PVA wafer brushes; Sahir et al. 2021).
  - In xerographic fur-brush cleaners, toner collects **at fibre tips** and must be stripped by a detoning roll.
  - **No measured carried-versus-dropped fractions for dry brushes were found.**
- **[E] Tunable starting values.**
  - Each tip contact picks up a few to tens of particles until the tip's capacity saturates.
  - Of what is detached, 30–70 % rides on the bristles and is released downstream over ~1–5 mm.
  - The rest is pushed ahead and drops into the next pores within ~0.1–1 mm. That builds a pale smear along the stroke and a darker lip at the stroke end.
- **[M] Triboelectric affinity** (AlphaLab, nC/J): hair +45, paper +10, cotton +5, wool 0, glass +25. Keratin against paper is a weak pair, so expect weak bristle charging. CaCO₃ and pigments are not listed.
- **[D]** Even at the breakdown limit the image force is ≈ nN (see §2), so charge mainly affects re-attachment of the finest (<2 µm) particles. That matters on dry (<30 % RH) days.

---

## 7. Simulator recipe

1. **Contact test.** For each bristle tip at (x, y), let z_tip be the top-envelope height minus the reach h(r_tip, local pore width) from §5. Add flag reach if flagged [E].
2. **Removal.** Any loose particle above z_tip in the swept footprint is detached with probability ≈ 1 [D]. Particles below z_tip stay.
3. **Agglomerates.** Compute F_t = μ·F_n,bristle with F_n ≤ F_cr. If F_t > σ_t·d², split the crumb, else move it whole. Use σ_t of 10–100 kPa for fresh soft pastel; fixed pastel is a gap, so try ≥10× that.
4. **Fixed pastel.** Treat it as bonded: there is no single-particle removal. Wear goes by an abrasion law, and the brush removes nothing at normal loads [E].
5. **Redeposition** follows §6, with tunable carry fraction and drop distance.
6. **Humidity.** Ignore for chalk. For kaolin-rich or fine pigment, raise adhesion ×1.5–3 at >60 % RH.

### Calibration bench (one afternoon)
- Lay a 2×2 cm swatch on Canson Mi-Teintes or similar.
- Scan, brush N strokes with a #8 hog filbert at known load (kitchen scale), then rescan.
- Fit removed coverage against N, plus the residual ghost. The residual fixes the reach depth.
- Tape-lift the bristles to estimate the carried fraction.

---

## Gaps (no numbers found)
- Brushing force used by pastellists.
- Pastel-stick or crumb tensile strength.
- Tragacanth's quantitative effect on cohesion.
- Casein or shellac fixative adhesion.
- Bristle penetration into paper pores.
- Carried versus redeposited fractions for dry brushes.
- Triboelectric position of chalk and pigments.
- Acceleration (g) thresholds specific to pastel.

## Sources
- Bergström 1997, *Adv. Colloid Interface Sci.* 70:125; table reproduced with CaCO₃/Fe₂O₃ values in Lefèvre et al., Heat Exch. Fouling & Cleaning: https://heatexchanger-fouling.com/wp-content/uploads/2021/09/16_Lefevre_Hamaker_F.pdf
- Bergström et al. 1999, Cellulose Hamaker constant: https://link.springer.com/article/10.1023/A:1009250111253
- Weber & Kaufhold 2021, *Colloid Interface Sci. Commun.* 43:100442 (kaolinite): https://www.sciencedirect.com/science/article/abs/pii/S2215038221000820
- Faure, Salazar-Alvarez & Bergström 2011, *Langmuir* 27:8659 (iron oxides): https://pubmed.ncbi.nlm.nih.gov/21644514
- Moutinho et al. (NREL), AFM adhesion vs RH and roughness: https://www.osti.gov/servlets/purl/1375106
- Jones, Pollock, Geldart & Verlinden 2003, *Powder Technol.* 132:196, doi:10.1016/S0032-5910(03)00072-X
- Nagaashi, Aoki & Nakamura 2021, *Icarus* 360:114357: https://arxiv.org/abs/2101.11837
- Rabinovich et al. 2000, *J. Colloid Interface Sci.* 232 (I and II): https://pubmed.ncbi.nlm.nih.gov/11071727/
- Heim, Blum, Preuss & Butt 1999, *PRL* 83:3328 (rolling force)
- Fuchs et al. 2014, *Granular Matter*, rolling/sliding of silica microspheres: https://arxiv.org/abs/1401.2600
- Xu et al. 2004, *J. Vac. Sci. Technol. B* 22:2844, doi:10.1116/1.1815319
- Chen et al. 2018, *Sol. Energy Mater. Sol. Cells* 179:247, doi:10.1016/j.solmat.2017.12.009
- Sauvage, Wei & Martinez 2018, *Stud. Conserv.* 63 sup1:S418 (open access): https://research.tudelft.nl/files/180898282/When_Conservation_Meets_Engineering_Predicting_the_Damaging_Effects_of_Vibrations_on_Pastel_Paintings.pdf
- Thickett 2002, ICOM-CC Rio, "Vibration damage levels for museum objects": https://www.english-heritage.org.uk/siteassets/home/learn/conservation/collections-advice--guidance/vibration-rio.pdf
- Wei, Sauvage & Wölk 2014, ICOM-CC Melbourne: https://www.icom-cc-publications-online.org/1325/Baseline-limits-for-allowable-vibrations-for-objects
- "The fixing of pastel artworks" (ICR): https://www.academia.edu/4192014/
- Cabiscol et al. 2020, *Adv. Powder Technol.* 31:1280: https://ris.utwente.nl/ws/files/232510350/Cabiscol2020effect.pdf
- Pig-hair tensile properties, *Biosyst. Eng.* 119:35 (2014): https://ui.adsabs.harvard.edu/abs/2014BiSyE.119...35M/abstract
- Sahir et al. 2021, PVA brush loading, doi:10.4028/www.scientific.net/ssp.314.259
- AlphaLab triboelectric series: https://www.alphalabinc.com/triboelectric-series/
- NEDCC 7.2 Surface cleaning of paper: https://www.nedcc.org/free-resources/preservation-leaflets/7.-conservation-procedures/7.2-surface-cleaning-of-paper
- Practitioner reports: http://kemstudios.blogspot.com/2014/02/two-options-for-correcting-pastel.html ; https://jackiesimmonds.com/page/1869/pastels-troubleshooter
