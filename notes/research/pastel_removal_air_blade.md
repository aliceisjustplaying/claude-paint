# Removing loose pastel: brush pickup and a steel blade

Research notes for the pastel simulator's dry-brush and knife-scrape tools. Compiled 2026-10-06. Builds on `pastel_brushing.md` (adhesion F_adh ≈ 10–150 nN per contact, size-independent for irregular grains) and `paper_surface.md` (fibers 20–40 µm wide, exponential pore depth with mean 5–15 µm, caliper 0.2–0.4 mm). Blowing and tapping (§A1–A2) belong with the tools that need them and are not here.

**Tags:** **[M]** measured, cited. **[D]** derived here, arithmetic shown. **[E]** estimate; tunable.

**Bottom line**
- **A brush** saturates at roughly 0.05–0.08 g of powder per cm³ of loaded bristle volume. That is a few mg for a #8 hog brush, i.e. a full load from 1–20 cm² of pastel.
- **A palette knife** at hand loads (0.05–0.2 N/mm) presses at 0.5–2.5 MPa. It sinks about 15–40 µm into the sheet and leaves a few to about 13 µm of permanent flattening, so it scrapes into the tooth and burnishes it. A scalpel (edge ≈ 1 µm) cuts fiber walls above about 0.1–0.8 N/mm and raises nap once its drag per fiber exceeds the fiber–fiber bond strength (about 1–6 mN).

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
  - Monolayer on the bristle surface: 5 µm chalk at 50 % areal coverage is ≈ 0.45 mg/cm² of fiber surface. A 2 mm loaded tip of a 160 µm bristle has 0.01 cm², so about 5 µg per bristle.
  - Interstitial load: a #8 filbert's loaded tip is about 0.1 cm³, which at the fur-brush value gives about 5 mg.
  - **Total about 2–10 mg.** Against a tooth load of 0.2–2.5 mg/cm², the brush saturates after **≈ 1–20 cm²** of fully loaded pastel.
  - Beyond saturation, pickup → 0 and the brush redeposits. Toner collects at fiber tips first.
- **Gap:** no per-contact retention fraction was found for a dry brush on a powder layer.

---

## B4. A steel blade on paper

### Material numbers

- **Fiber wall hardness [M].**
  - S2 wall of unbleached pine kraft fiber: **0.42 ± 0.05 GPa**, indentation modulus 12.2 GPa. Bleaching lowers both (Adusumalli et al. 2010).
  - Wood cell walls: ≈ 0.27–0.34 GPa.
  - Whole pulp fibers indented transversely (in-situ SEM): **70–312 MPa**, transverse modulus 8.3 GPa.
- **Sheet z-compression [M]** (80 g/m² copy paper, 84.7 µm, 6 mm flat punch; Chen et al. 2020):
  - Fit: σ = 0.636 (e^(13.54 ε) − 1) MPa. Initial tangent modulus is ≈ 8.6 MPa.
  - Residual strain after unloading: ε_r ≈ 0.49 ε_max − 0.027.

  | p (MPa) | 0.7 | 1.4 | 2.8 | 4.2 | 7.1 | 14 |
  |---|---|---|---|---|---|---|
  | ε_max | 0.050 | 0.088 | 0.122 | 0.155 | 0.202 | 0.290 |
  | ε_residual | 0.002 | 0.016 | 0.027 | 0.046 | 0.076 | 0.122 |

  - So **permanent flattening starts near 1 MPa**.
  - [M, search-summarized] Calender nips run at tens of MPa (release-paper calenders up to 75 MPa) and line loads of 50–525 kN/m.
- **Fiber-joint strength [M].**
  - Single fiber–fiber joints: mode II **6.5 mN** and mode III **1.1 mN** (softwood); **1.8 ± 0.5 mN** (hardwood).
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
- Winkler ignores the in-plane stiffness of the fibers, which is about 200× the z stiffness, so **these depths are upper bounds**. Real dents spread over a few fiber widths.
- The depths already exceed the pore depth (5–15 µm) and Rq. **A knife therefore reaches the floor of most pores** where a bristle tip reaches only 1–5 µm.
- It also leaves the peaks permanently compressed by a few to about 13 µm. That is burnishing: shallower pores and less tooth.

### Cutting and nap [D]

- **Cutting.** The edge cuts a fiber when the contact pressure on that fiber exceeds the wall hardness. That gives q_cut ≈ H·2R:
  - R = 1 µm (scalpel): q_cut ≈ 0.14–0.84 N/mm, so a scalpel held steeply at ordinary load cuts fibers.
  - R = 5 µm: 0.7–4.2 N/mm.
  - R ≥ 20 µm: 2.8–17 N/mm, so a knife compresses fibers but does not cut them.
- **Nap raising.** Drag per 30 µm of fiber crossed is μq·30 µm:
  - q = 0.05 N/mm: 0.3–0.6 mN, below joint strength.
  - q = 0.2: 1.2–2.4 mN, borderline.
  - q = 1: 6–12 mN, above the 1–6.5 mN joint strength.

  Fibers crossing above the mean plane, and free fiber ends, are torn loose and lifted. **Expect nap above about 0.2–0.5 N/mm, or at any load if the edge catches a fiber end.**
- **[M, qualitative] Conservation practice.** Accretions are best fractured with "light downward pressure" of a scalpel tip, then brushed or scraped away. "Picking off accretions with a scalpel may disturb the paper more." A dull, rounded scalpel is used to scrape swollen adhesive (AIC wiki, Surface Cleaning; Hinge/Tape Removal).
- **[M, qualitative] Eraser abrasion.** SEM studies (Pearlstein 1982; McInnis 1980) show gum and kneaded erasers abrade fibers. Pink Pearl "altered texture". No mass-loss rates are published there.
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

1. **Brush capacity.**
   - Track loaded mass per brush and cap it at ≈ 0.06 g/cm³ × loaded-tip volume.
   - Pickup efficiency falls linearly to 0 at saturation. Above about 80 %, deposit back.
2. **Knife.** Inputs: line load q, edge radius R.
   - Compute dent depth δ₀(q, R) from the Winkler table, reduced by the fiber network's through-thickness shear: 0.17–0.80× Winkler (`paper_mechanics_and_transport.md`'s table; use its fit or table).
   - Remove all loose pastel above (envelope − δ₀) along the edge. Push 20–50 % of it ahead as a heap [E] and drop it at the stroke end.
   - Compact the remainder into pores (raise its bond to "pressed" adhesion, about ×3–10 [E]).
   - Permanently lower envelope peaks by ε_r·T from Chen's residual law.
   - If q > q_cut(R), cut and roughen. If μq·w_f > 1–6 mN, raise nap fibers.

## Gaps (no numbers found)
- Per-contact retention fraction of dry brushes.
- Steel-on-paper wear rate.
- Paper surface (not bulk) indentation hardness.
- Measured knife indentation into drawing paper.
- Any quantitative study of scraping pastel.

## Sources
- AIC wiki, BPG Surface Cleaning: https://conservation-wiki.com/wiki/BPG_Surface_Cleaning ; Hinge, Tape and Adhesive Removal: https://conservation-wiki.com/wiki/BPG_Hinge,_Tape,_and_Adhesive_Removal
- US 8,879,946 (fur brush 3.5 g saturation): https://patents.google.com/patent/US8879946B2/en ; US 5,652,951 (Xerox 5100, 26–30 g): https://patents.google.com/patent/US5652951A/en
- US 7,752,702 (powder pickup 1–2 mg): https://patents.google.com/patent/US7752702B2/en ; US 9,949,560: https://patents.google.com/patent/US9949560B2/en
- Adusumalli et al. 2010, *J. Mater. Sci.*, nanoindentation of pulp fiber walls: https://link.springer.com/article/10.1007/s10853-010-4226-9 ; in-situ SEM transverse micro-indentation (*J. Electron Microsc.* 59:345): https://academic.oup.com/jmicro/article-abstract/59/5/345/861497
- Chen, Spiehl, Dörsam et al. 2020, *J. Print Media Technol. Res.* 9(2):65: https://www.jpmtr.org/index.php/journal/article/download/13/10/10
- Fiber-joint strength: https://link.springer.com/article/10.1007/s10570-016-0895-0 ; https://pubmed.ncbi.nlm.nih.gov/22852699/
- Z-direction strength and calendering pressures (search-summarized): https://www.diva-portal.org/smash/get/diva2:9223/FULLTEXT01.pdf ; https://www.valmet.com/insights/articles/services//enduring-release-paper-calendering
- McCarthy et al. 2007, scalpel sharpness: https://www.researchgate.net/publication/237773083
- Taber abrasion loss spec (TAPPI T476 adaptation): https://patents.google.com/patent/US6383614B1/en
- M. Chesley Johnson, "A painting knife for pastel": https://mchesleyjohnson.substack.com/p/plein-air-painting-essential-tools
