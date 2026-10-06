# Removing loose pastel: blowing, tapping, brush pickup, and a steel blade

Research notes for the pastel simulator's "blow", "tap" and "knife-scrape" tools. Compiled 2026-10-06. Builds on `pastel_brushing.md` (adhesion F_adh ≈ 10–150 nN per contact, size-independent for irregular grains) and `paper_surface.md` (fibres 20–40 µm wide, exponential pore depth with mean 5–15 µm, caliper 0.2–0.4 mm).

**Tags:** **[M]** measured, cited. **[D]** derived here, arithmetic shown. **[E]** estimate; tunable.

**Bottom line**
- **Blowing** removes crumbs and heaps of 50–300 µm sitting on top of the fibres. A firm puff from 5–10 cm reaches u* ≈ 0.7–2 m/s, against a threshold of about 0.2–0.9 m/s for them. Single 5 µm grains on top need u* ≈ 0.7–8 m/s, so only a close, hard blow takes some of them. Anything deeper than about half a pore width is unreachable, because the flow decays as exp(−4.2 z/w).
- **Tapping** takes off agglomerates of about 50–250 µm and larger (tens of g needed, hundreds of g delivered). It never takes 5 µm grains, which would need 10⁴–10⁵ g. This matches the conservation finding that loss is agglomerates.
- **A brush** saturates at roughly 0.05–0.08 g of powder per cm³ of loaded bristle volume. That is a few mg for a #8 hog brush, i.e. a full load from 1–20 cm² of pastel.
- **A palette knife** at hand loads (0.05–0.2 N/mm) presses at 0.5–2.5 MPa. It sinks about 15–40 µm into the sheet and leaves a few to about 13 µm of permanent flattening, so it scrapes into the tooth and burnishes it. A scalpel (edge ≈ 1 µm) cuts fibre walls above about 0.1–0.8 N/mm and raises nap once its drag per fibre exceeds the fibre–fibre bond strength (about 1–6 mN).

---

## A1. Blowing (aerodynamic resuspension)

### Thresholds

**Loose bed or heap [M fit, D numbers].** Shao & Lu 2000:

> u*t = √( A_N (σ_p g d + γ/(ρ_a d)) ), with σ_p = ρ_p/ρ_a, **A_N = 0.0123**, **γ = 3×10⁻⁴ N/m** (range 1.65–5×10⁻⁴), ρ_a = 1.2 kg/m³.

A Mars-chamber refit on irregular simulant grains gave A_N = 2.7×10⁻³ and γ = 1.2×10⁻⁴, which shows how far the constants can move (Kruss et al. 2019). Values for ρ_p = 2500 kg/m³:

| d (µm) | 1 | 2 | 5 | 10 | 20 | 50 | 100 | 200 | 300 |
|---|---|---|---|---|---|---|---|---|---|
| u*t (m/s) | 1.75 | 1.24 | 0.79 | 0.56 | 0.40 | 0.27 | **0.24** (min) | 0.26 | 0.29 |

- At a porous crumb density of 1250 kg/m³ the values are almost unchanged below 50 µm and about 0.21 m/s at 100–300 µm.
- The formula was fitted mostly on sand. Below 20 µm treat it as an extrapolation.

**Single grain on top of a surface: moment balance [M coefficients, D numbers].** Reeks & Hall 2001 give the mean drag on a sphere of radius r in the viscous sublayer as F_D = 32 ρ_a r² u*². The particle rolls off when F_D·(r/a) ≥ F_adh, where a is the spacing of its contact asperities. Hall measured r/a ≈ 100 on polished steel.

| Case | r/a | F_adh 10 nN | F_adh 150 nN |
|---|---|---|---|
| 5 µm grain | 100 / 10 | 0.65 / 2.0 m/s | 2.5 / 7.9 m/s |
| 20 µm grain | 100 / 10 | 0.16 / 0.51 | 0.63 / 2.0 |
| 100 µm crumb on fibres | 5 / 2 [E] | 0.14 / 0.23 | 0.56 / 0.88 |
| 300 µm crumb | 5 / 2 | 0.05 / 0.08 | 0.19 / 0.29 |

- The particle Reynolds number r⁺ = r u*/ν stays below 3 in every case, so the sublayer formula holds.
- Wall shear fluctuates, so treat removal as a per-puff probability ("rock'n'roll" kinetics), not a hard cut.

### What a puff delivers

**Exhalation speeds [M]** (hot-wire at 25 mm from the lips, 31 people; Aston study):
- Blowing "as hard as possible": mean **12 m/s**, 85th percentile 21 m/s, range of peaks **6–64 m/s**.
- Spread is 55–65 % between people and 25–35 % for one person.
- For comparison, coughing averages 4 m/s.

**Rubber bulb blower [E/D].**
- No trustworthy measurement found.
- Squeeze pressure Δp ≈ 0.5–2 kPa gives U ≈ √(2Δp/ρ) ≈ 30–60 m/s ideal. Assume **20–60 m/s** through a 3–5 mm nozzle.

**Impinging jet, peak wall shear [M]** (Phares et al. 2000, as used by Sharma et al. 2022):

> τ_max = 44.6 ρ_a U_J² Re_J^(−1/2) (H/D)^(−2), at radius r = 0.09 H.

- Free-jet centreline decay: U_c ≈ K·U_J·D/(H+λ), with K ≈ 4–6.2.
- Below H/D ≈ 6 I cap τ at the H/D = 6 value (potential core) [E].

| Source | H = 20 mm | 50 mm | 100 mm | 200 mm |
|---|---|---|---|---|
| Mean blow, 12 m/s, D 8 mm [E] | u* 1.5 m/s | 1.4 | 0.72 | 0.36 |
| Hard blow, 21 m/s | 2.3 | 2.2 | 1.1 | 0.55 |
| Bulb, 30 m/s, D 4 mm | 3.5 | 1.7 | 0.85 | 0.42 |

- Example of the arithmetic: hard blow at 100 mm gives τ = 44.6·1.2·21²·(11200)^(−½)·12.5⁻² = 1.43 Pa, so u* = √(τ/ρ) = 1.09 m/s.
- The peak sits on a ring about 0.1 H in radius. The footprint at 1/e is a few H/D.
- A puff from 5–10 cm clears heaps and crumbs in a spot a few cm across, plus only the weakly held fines on top.

### Particles in pores are sheltered [D]

- Paper roughness is hydraulically smooth. With k = 10–30 µm and u* = 1 m/s, k⁺ = k u*/ν ≈ 0.7–2, so the pores lie inside a viscous Stokes layer.
- Stokes flow over a slot of width w decays with depth as **exp(−4.21 z/w)**. This is Moffatt's 1964 corner-eddy solution: each counter-rotating eddy is about 1.39 w tall and about 360× weaker than the one above.

| Depth z/w | 0.25 | 0.5 | 1 | 1.5 | 2 |
|---|---|---|---|---|---|
| τ(z)/τ_top | 0.35 | 0.12 | 0.015 | 0.0018 | 0.0002 |
| u* multiplier needed | 1.7 | 2.9 | 8.2 | 24 | 67 |

- A 5 µm grain one pore width down (z ≈ 20–40 µm) needs u* ≥ 5–60 m/s. No breath or bulb comes close, so **puffs remove only the top roughly 0.25–0.5 w (5–20 µm) of a pore's filling**.
- **[M, qualitative] Practice agrees.** The AIC wiki says a bulb blower is not to be used on friable or powdery media, may disrupt fibres of soft papers, and "dirt may not be effectively removed".

---

## A2. Tapping and vibration

### Inertial threshold [D]
A particle leaves the surface when m·a > F_adh, with m = πd³ρ/6.

| d | m (ρ 2.5 g/cm³) | a to detach at 10 nN | at 150 nN |
|---|---|---|---|
| 5 µm | 1.6×10⁻¹³ kg | 6.2×10³ g | 9.3×10⁴ g |
| 20 µm | 1.0×10⁻¹¹ | 97 g | 1460 g |
| 50 µm | 1.6×10⁻¹⁰ | 6 g | 93 g |
| 100 µm | 1.3×10⁻⁹ | **0.8 g** | **12 g** |
| 300 µm | 3.5×10⁻⁸ | 0.03 g | 0.4 g |

- **Critical size:** d_c = (6F_adh/(πρa))^(1/3).
  - At 1 g (gravity alone, sheet inverted or vertical): d_c = 92–227 µm. At 50 % crumb porosity it is 116–286 µm.
  - At 10 g: 43–133 µm.
  - At 100 g: 20–62 µm.
- So **heaps and crumbs larger than about 0.1–0.3 mm drop off a vertical easel by gravity alone.**
- In-plane acceleration (tapping the board edge) acts by rolling. That lowers the threshold by a factor a_c/r ≈ 0.02–0.1, but fines only hop to the next pore.

### Accelerations available

- **Tapping a board edge on a table [D/E].** Half-sine impact gives a_peak = π(1+e)v/(2τ), with v = √(2gh).
  - Assumptions: drop h = 1–3 cm, restitution e = 0.3–0.6, contact time τ = 0.5–2 ms [E].
  - Result: **≈ 50–400 g** at the board. A flick of a finger on the back of the board is lower and local [E, tens of g].
  - No measured value was found (gap).
- **Transport [M]:** shocks up to **8–10 g** (Sauvage et al. 2018). Building vibration caused loss of loosely bound pigment at 0.2–0.3 g (Thickett 2002).
- **Sauvage vibration rig [D].**
  - About 10⁷ cycles in "two days" implies a drive of about 58 Hz.
  - The fixed specimens ran at 4.3 mm displacement amplitude, which gives (2πf)²X ≈ **58 g** at sheet centre (29 g if the 4.3 mm was peak-to-peak).
  - Unfixed mock-ups failed (1 % surface change) after 2.8×10⁵ cycles at the higher amplitude and 1.8×10⁶ at the lower. Some survived about 10⁷ cycles.
  - The loss was **agglomerates**: large black 1–20 µm grains "not as well embedded"; needle-shaped pink grains did not move.
- **[M, qualitative] Practice:** pastellists invert the board and tap its edge, or snap a finger against the back, over a tray (Pastel Today; SKH Portraits).

**Simulator rule [D].** Per tap, detach any loose top-of-envelope cluster with m·a_tap > F_adh·n_contacts. Use a_tap ≈ 50–200 g as the default. Pore-held single grains never go. Fatigue (repeated taps) can be a small per-tap probability for borderline clusters, following Sauvage's S–N behaviour.

---

## A3. How much a brush holds

- **[M] Xerographic fur brush** (patent US 8,879,946):
  - Brush is Ø19 mm on a 10 mm shaft, so the pile is 4.5 mm deep.
  - It holds "approximately **3.5 g** of toner in the saturated state".
  - **[D]** For a length of 23–32 cm [E], that is **0.053–0.074 g per cm³ of pile**, i.e. toner fills 5–7 % of pile volume, or 18–26 mg per cm² of brush face.
  - **[M]** Larger Xerox 5100 cleaner brushes need service at about **26–30 g** of toner, when they start to emit clouds (US 5,652,951). Over-full brushes take a permanent radial set.
- **[M] Cosmetic brushes** (US 7,752,702):
  - A 40–50 mm-diameter PTT bristle bundle picked up **1 mg** of pressed powder foundation in 3 strokes with smooth filaments and **2 mg** with 1–25 µm surface roughening.
  - **[D]** That is ≈ 0.1–0.2 mg/cm² of brush face. It is supply-limited, because the powder was pressed.
  - Horse hair's cuticle scales are credited with good "loading" (US 9,949,560).
- **[D] Hog brush capacity.**
  - Monolayer on the bristle surface: 5 µm chalk at 50 % areal coverage is ≈ 0.45 mg/cm² of fibre surface. A 2 mm loaded tip of a 160 µm bristle has 0.01 cm², so about 5 µg per bristle.
  - Interstitial load: a #8 filbert's loaded tip is about 0.1 cm³, which at the fur-brush value gives about 5 mg.
  - **Total about 2–10 mg.** Against a tooth load of 0.2–2.5 mg/cm², the brush saturates after **≈ 1–20 cm²** of fully loaded pastel.
  - Beyond saturation, pickup → 0 and the brush redeposits. Toner collects at fibre tips first.
- **Gap:** no per-contact retention fraction was found for a dry brush on a powder layer.

---

## B4. A steel blade on paper

### Material numbers

- **Fibre wall hardness [M].**
  - S2 wall of unbleached pine kraft fibre: **0.42 ± 0.05 GPa**, indentation modulus 12.2 GPa. Bleaching lowers both (Adusumalli et al. 2010).
  - Wood cell walls: ≈ 0.27–0.34 GPa.
  - Whole pulp fibres indented transversely (in-situ SEM): **70–312 MPa**, transverse modulus 8.3 GPa.
- **Sheet z-compression [M]** (80 g/m² copy paper, 84.7 µm, 6 mm flat punch; Chen et al. 2020):
  - Fit: σ = 0.636 (e^(13.54 ε) − 1) MPa. Initial tangent modulus is ≈ 8.6 MPa.
  - Residual strain after unloading: ε_r ≈ 0.49 ε_max − 0.027.

  | p (MPa) | 0.7 | 1.4 | 2.8 | 4.2 | 7.1 | 14 |
  |---|---|---|---|---|---|---|
  | ε_max | 0.050 | 0.088 | 0.122 | 0.155 | 0.202 | 0.290 |
  | ε_residual | 0.002 | 0.016 | 0.027 | 0.046 | 0.076 | 0.122 |

  - So **permanent flattening starts near 1 MPa**.
  - [M, search-summarised] Calender nips run at tens of MPa (release-paper calenders up to 75 MPa) and line loads of 50–525 kN/m.
- **Fibre-joint strength [M].**
  - Single fibre–fibre joints: mode II **6.5 mN** and mode III **1.1 mN** (softwood); **1.8 ± 0.5 mN** (hardwood).
  - AFM opening (mode I) tests cover 0.01–1 mN.
  - Z-direction tensile strength of sheets is **0.2–1.75 MPa**.
- **Edge radii [M].** A new scalpel is **≈ 0.8–1 µm**; 5 µm already counts as "blunt" (McCarthy et al. 2007; review). A painting knife's edge is **[E] 20–500 µm**: it is not sharpened.
- **Friction coefficient [E].** Steel on paper μ ≈ 0.2–0.4.

### How far the edge sinks [D]

Model: a Winkler (bed-of-springs) layer with Chen's curve, sheet T = 250 µm on a rigid backing, and a cylindrical edge of radius R. Depth profile δ(x) = δ₀ − x²/2R, and line load q = ∫σ(δ/T)dx.

| R | q (N/mm) | depth δ₀ | contact width | mean p | residual dent |
|---|---|---|---|---|---|
| 500 µm (flat of knife) | 0.05 / 0.2 | 9 / 20 µm | 190 / 280 µm | 0.3 / 0.7 MPa | 0 / 3 µm |
| 100 µm (knife edge) | 0.05 / 0.2 / 1 | 14 / 29 / 52 µm | 110 / 150 / 200 µm | 0.5 / 1.3 / 4.9 MPa | 0 / 7 / 19 µm |
| 20 µm | 0.05 / 0.2 | 22 / 40 µm | 60 / 80 µm | 0.8 / 2.5 MPa | 4 / 13 µm |

- Hand scraping is likely 1–5 N over a 10–25 mm edge, i.e. **q ≈ 0.05–0.5 N/mm** [E].
- Winkler ignores the in-plane stiffness of the fibres, which is about 200× the z stiffness, so **these depths are upper bounds**. Real dents spread over a few fibre widths.
- The depths already exceed the pore depth (5–15 µm) and Rq. **A knife therefore reaches the floor of most pores** where a bristle tip reaches only 1–5 µm.
- It also leaves the peaks permanently compressed by a few to about 13 µm. That is burnishing: shallower pores and less tooth.

### Cutting and nap [D]

- **Cutting.** The edge cuts a fibre when the contact pressure on that fibre exceeds the wall hardness. That gives q_cut ≈ H·2R:
  - R = 1 µm (scalpel): q_cut ≈ 0.14–0.84 N/mm, so a scalpel held steeply at ordinary load cuts fibres.
  - R = 5 µm: 0.7–4.2 N/mm.
  - R ≥ 20 µm: 2.8–17 N/mm, so a knife compresses fibres but does not cut them.
- **Nap raising.** Drag per 30 µm of fibre crossed is μq·30 µm:
  - q = 0.05 N/mm: 0.3–0.6 mN, below joint strength.
  - q = 0.2: 1.2–2.4 mN, borderline.
  - q = 1: 6–12 mN, above the 1–6.5 mN joint strength.

  Fibres crossing above the mean plane, and free fibre ends, are torn loose and lifted. **Expect nap above about 0.2–0.5 N/mm, or at any load if the edge catches a fibre end.**
- **[M, qualitative] Conservation practice.** Accretions are best fractured with "light downward pressure" of a scalpel tip, then brushed or scraped away. "Picking off accretions with a scalpel may disturb the paper more." A dull, rounded scalpel is used to scrape swollen adhesive (AIC wiki, Surface Cleaning; Hinge/Tape Removal).
- **[M, qualitative] Eraser abrasion.** SEM studies (Pearlstein 1982; McInnis 1980) show gum and kneaded erasers abrade fibres. Pink Pearl "altered texture". No mass-loss rates are published there.
- **[M, spec] Taber abrasion.** Absorbent-sheet patents target dry abrasion loss below 200–300 mg per 100 revolutions (H-18 wheels, 1 kg, TAPPI T476). On about 30 cm² of track that is ≈ 0.05 mg/cm² per wheel pass for a coarse abrasive wheel. This is not a smooth blade; **the wear law for smooth steel on paper is a gap.**

---

## B5. A knife on pastel in practice [E, practitioner]

- Painting knives are used in pastel to:
  - "scrape off pastel that is too thick (with the edge or tip)";
  - blend (flat side);
  - "push pastel into the surface so it sticks better (flat side)" (M. Chesley Johnson).
- Razor blades held flat are used to scrape fixed or oil pastel. Users warn of damage to the paper. A filled, glossy tooth is restored, if at all, by fine sandpaper.
- Consistent with B4: the edge shears off the heap down to the dent depth, the flat face compacts the rest into pores and flattens peaks. Result: less tooth, a slightly glossy track, nap at high load.

---

## Simulator recipe

1. **Puff** at (x, y, H) with source velocity U.
   - Compute u*(r) from Phares (ring peak at 0.09 H).
   - For each loose top-of-envelope cluster, remove it with probability rising steeply once u* > u*t(d, F_adh, r/a) from §A1.
   - For pore-held material at depth z below the local envelope, scale τ by exp(−4.21 z/w_pore).
   - Removed material is airborne. Redeposit 0–20 % downstream within 1–3 H [E].
2. **Tap** with a_tap of 50–200 g (default) or 1 g for a vertical easel.
   - Detach clusters with ρ_eff·πd³/6·a > F_adh·n_contacts.
   - Pore-held grains never go.
3. **Brush capacity.**
   - Track loaded mass per brush and cap it at ≈ 0.06 g/cm³ × loaded-tip volume.
   - Pickup efficiency falls linearly to 0 at saturation. Above about 80 %, deposit back.
4. **Knife.** Inputs: line load q, edge radius R.
   - Compute dent depth δ₀(q, R) from the Winkler table, clipped by fibre-network spreading [E ×0.3–1].
   - Remove all loose pastel above (envelope − δ₀) along the edge. Push 20–50 % of it ahead as a heap [E] and drop it at the stroke end.
   - Compact the remainder into pores (raise its bond to "pressed" adhesion, about ×3–10 [E]).
   - Permanently lower envelope peaks by ε_r·T from Chen's residual law.
   - If q > q_cut(R), cut and roughen. If μq·w_f > 1–6 mN, raise nap fibres.

## Gaps (no numbers found)
- Measured acceleration of tapping a drawing board.
- Rubber-bulb exit velocity.
- Breath jet diameter and duration.
- Per-contact retention fraction of dry brushes.
- Steel-on-paper wear rate.
- Paper surface (not bulk) indentation hardness.
- Measured knife indentation into drawing paper.
- Any quantitative study of scraping pastel.

## Sources
- Shao & Lu 2000, *JGR* 105:22437, doi:10.1029/2000JD900304: https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2000jd900304
- Kruss, Musiolik, Demirci, Wurm & Teiser 2019, Mars wind erosion (A_N and γ refit): https://arxiv.org/pdf/1911.01692
- Reeks & Hall 2001, *J. Aerosol Sci.* 32:1. Coefficients as quoted in Zhang, Reeks et al.: https://arxiv.org/pdf/1206.1939
- Sharma, Gong, Azadi, Gans, Gondret & Sauret 2022, erosion of cohesive grains by an impinging jet (Phares formula, K, cohesive Shields number): https://arxiv.org/pdf/2206.01839
- Breath exit velocity (Aston/ESR): https://publications.aston.ac.uk/id/eprint/31720/
- Moffatt 1964, *J. Fluid Mech.* 18:1 (viscous eddies); review of the cavity eddies: https://www.sciencedirect.com/science/article/pii/S0898122118302360
- AIC wiki, BPG Surface Cleaning: https://conservation-wiki.com/wiki/BPG_Surface_Cleaning ; Hinge, Tape and Adhesive Removal: https://conservation-wiki.com/wiki/BPG_Hinge,_Tape,_and_Adhesive_Removal
- Sauvage, Wei & Martinez 2018, *Stud. Conserv.* 63 sup1: https://research.tudelft.nl/files/180898282/When_Conservation_Meets_Engineering_Predicting_the_Damaging_Effects_of_Vibrations_on_Pastel_Paintings.pdf
- Thickett 2002, ICOM-CC: https://www.english-heritage.org.uk/siteassets/home/learn/conservation/collections-advice--guidance/vibration-rio.pdf
- Pastel Today, dust handling: https://pasteltoday.com/2022/06/15/pastel-dust-9-practical-ways-to-deal-with-it/ ; SKH Portraits: https://skhportraits.co.uk/how-to-keep-yourself-your-pastel-art-clean-7-top-tips/
- US 8,879,946 (fur brush 3.5 g saturation): https://patents.google.com/patent/US8879946B2/en ; US 5,652,951 (Xerox 5100, 26–30 g): https://patents.google.com/patent/US5652951A/en
- US 7,752,702 (powder pickup 1–2 mg): https://patents.google.com/patent/US7752702B2/en ; US 9,949,560: https://patents.google.com/patent/US9949560B2/en
- Adusumalli et al. 2010, *J. Mater. Sci.*, nanoindentation of pulp fibre walls: https://link.springer.com/article/10.1007/s10853-010-4226-9 ; in-situ SEM transverse micro-indentation (*J. Electron Microsc.* 59:345): https://academic.oup.com/jmicro/article-abstract/59/5/345/861497
- Chen, Spiehl, Dörsam et al. 2020, *J. Print Media Technol. Res.* 9(2):65: https://www.jpmtr.org/index.php/journal/article/download/13/10/10
- Fibre-joint strength: https://link.springer.com/article/10.1007/s10570-016-0895-0 ; https://pubmed.ncbi.nlm.nih.gov/22852699/
- Z-direction strength and calendering pressures (search-summarised): https://www.diva-portal.org/smash/get/diva2:9223/FULLTEXT01.pdf ; https://www.valmet.com/insights/articles/services//enduring-release-paper-calendering
- McCarthy et al. 2007, scalpel sharpness: https://www.researchgate.net/publication/237773083
- Taber abrasion loss spec (TAPPI T476 adaptation): https://patents.google.com/patent/US6383614B1/en
- M. Chesley Johnson, "A painting knife for pastel": https://mchesleyjohnson.substack.com/p/plein-air-painting-essential-tools
