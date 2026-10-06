# Drawing and pastel paper as a physical support

Research notes for generating a paper sheet from how it is made: fibres, deposition, pressing, felts and wires, sizing. The aim is to avoid noise textures. Compiled 2026-10-05.

**Tags** (same convention as `pastel_stick_tribology.md`):
- **[M]** measured, from a cited source.
- **[D]** derived here from measured values or standard formulas, with the arithmetic shown.
- **[E]** estimate with no direct measurement; treat it as a tunable range.

**The main gap.** I found no published profilometry (Ra/Sq, PSD, Abbott curve) for named artists' papers: Canson Mi-Teintes, Ingres laid papers, Fabriano Tiziano, Arches. Neither manufacturers nor retailers publish caliper. The best measured anchors are:
- stylus Ra/Rz for a Japanese **Kent drawing paper** and office papers;
- a 2026 confocal study of printer paper, which found **self-affine roughness with H ≈ 0.5**;
- Singh's surface **autospectra** of handsheets (variance concentrated at 0.1–1 mm wavelengths);
- PPS data before and after calendering;
- the paper-physics stochastic network theory (Dodson, Kallmes–Corte, Sampson), which gives the generative formulas.

Values for felt marks, honeycomb and laid-line relief are **[E]**. §8 lists measurements that would close the gaps.

---

## 1. Topography: what has been measured

### Roughness amplitudes
- **[M] Stylus profilometer** (Kosaka Surfcorder) results, Ra / Rz in µm:
  - **Kent drawing paper: 2.57 / 15.2**
  - plain copier paper: 2.95 / 15.4
  - recycled paper: 1.97 / 11.1
  - high-quality paper: 1.76 / 11.5
  - photo-print paper back: 1.25 / 6.9

  The authors summarise: "many papers showed roughness Ra of 2–3 µm with fiber texture on the surface" (arXiv 1604.01540). Stylus tip radius and cut-off filter were not stated. Treat these as **micro-roughness with waviness removed**.
- **[M] Air-leak Parker Print-Surf (PPS)**, mechanical-pulp handsheets at 1 MPa clamp: about **4–6 µm uncalendered** and about **2–3 µm calendered**. Calendering also cut surface compressibility. Values were read from the figure, so ±0.5 µm (Singh 2008, Fig. 1).
- **[M] PPS for uncoated base paper**: "below 7, preferably below 5 µm", with **Bendtsen 200–400 mL/min** (patent survey hit, Smithers/USPTO). PPS normal range is 0.2–6.5 µm and the high range 6–15 µm (TMI PPS spec).
- **[M] Instrument ranges.** Bendtsen covers 5–3000 mL/min (variable-area) or 50–5000 mL/min (electronic) (ISO 8791-2 scope). Wire-side and felt-side smoothness of one sheet can differ by **≥70–130 Sheffield units** (patent hit on two-sidedness).
- **[E] Artists' papers.** A fine-grain drawing cartridge should sit near Kent paper (Ra ≈ 2–4 µm). Rough-side pastel and watercolour papers carry a felt or embossed macro-texture on top. Expect **Ra of order 5–20 µm and peak-to-valley of order 30–100 µm** over mm-scale windows. Unmeasured; calibrate (§8).

### Spatial structure
- **[M] Self-affine roughness in printer paper.** Confocal pen, 1.3 µm lateral step, 2 × 2 mm fields. The isotropic PSD is C(q) ∝ q^−2(1+H) with **H ≈ 0.5**, i.e. C ∝ q⁻³. The authors describe "no single characteristic lateral scale", with fibre diameters of 5–15 µm at the surface (Archambault, Domino & Bertin 2026, preprint). Their raw topographies are on contact.engineering.
- **[M] Which wavelengths carry the variance** (Singh 2008, stylus autospectra):
  - Below 0.1 mm, profiles carry **less** variance than a random walk because fibres and furnish components have finite size.
  - Above 1 mm, components are "not reproducible", i.e. waviness.
  - "The distribution of variation in a wavelength band of **0.1 mm to 1 mm** characterises the surface of most grades of paper."
  - Calendering removes variance mainly in that band.
- **[M] Scale classes** used by on-line laser triangulation (ResearchGate abstract, Online surface roughness…):
  - optical roughness below 1 µm;
  - micro-roughness 1–100 µm;
  - macro-roughness 0.1–1 mm.

  Uncoated papers show **two slopes** of roughness against sampling size, attributed to single-fibre directions (Appl. Surf. Sci. 2011 abstract).
- **[D] Autocorrelation.** With H = 0.5 and an upper cutoff, the height ACF is roughly exponential, with a correlation length near the upper cutoff of the self-affine range. For uncoated wove paper that cutoff is about **fibre length to floc size (1–3 mm)**. The lower cutoff is the fibre width (20–40 µm).

### Height distribution shape (skewness, kurtosis, Abbott–Firestone)
- **[M] Dodson's pore heights.** For random fibre networks, the distribution of "surface pore heights" is **exponential with mean proportional to fibre thickness t**.
- **[M] Sampson–Wang model.** Sampson & Wang (2020) give Ra and Rq of stochastic fibrous materials as functions of **porosity and fibre thickness only**. Ra and Rq are linear in each other, with a slope set by the skewness of the depth profile. The model is validated on lab handsheets, commercial papers and carbon-fibre nonwovens (abstract; full text was behind a CAPTCHA, so the exact equations are not reproduced here).
- **[D] Shape statistics** if surface depth below the top envelope is exponential with mean μ:
  - Rq = μ
  - Ra = 2μ/e ≈ 0.74μ, so **Rq/Ra ≈ 1.36** (Gaussian gives 1.25)
  - depth skewness **+2**, i.e. height skewness **−2** (plateaus with deep valleys)
  - kurtosis **9**
  - **Abbott curve**: the material ratio at depth d is 1 − e^(−d/μ). Half the area is reached at d = 0.69μ, and 95 % at 3μ.

  Real papers are pressed and calendered, which truncates peaks and pushes height skewness further negative ("plateau honing"). **[E]** Use a gamma depth distribution with shape k = 1–3 (height skewness −2 to −1.15) and calibrate k.

### Converting air-leak roughness to µm
- **[M] PPS geometry.** PPS reports the "cube-root-mean-cube gap" in µm between paper and a land **51 µm wide, 98 mm long**, at Δp = 6.2 or 19.6 kPa. Clamp is 0.5/1.0/2.0 MPa against a soft backing (ISO 8791-4).
- **[D] Slot-flow inversion.** Plane-Poiseuille flow through a slot gives

  **G = (12 η b Q / (L Δp))^(1/3)**, where η(air) = 1.81 × 10⁻⁵ Pa·s, b = land width (the air's path, across the land), L = land length (the slot's extent, along the land): Q = L·G³·Δp/(12ηb).

- **[M/E] Bendtsen land.** Width 0.15 mm, Δp = 1.47 kPa, on glass at about 0.1 MPa clamp (Singh et al. 1991, Table I). The land length L ≈ 99 mm (ring about 31.5 mm diameter) is **[E]**.
- **[D] Bendtsen to equivalent gap:**

  | Bendtsen (mL/min) | Equivalent gap |
  |---|---|
  | 30 | 4.8 µm |
  | 100 | 7.2 µm |
  | 300 | 10.4 µm |
  | 1000 | 15.5 µm |

  G scales as Q^(1/3). These gaps are taken at low clamp pressure, and air leaking through porous sheets inflates them. PPS at 1 MPa typically reads about 2× lower. Sheffield units need instrument-specific calibration. No general conversion exists (Singh 2008).

---

## 2. Structure that makes the tooth

### Fibres
| Fibre | Length | Width | Wall | Coarseness δ | Source |
|---|---|---|---|---|---|
| Softwood kraft (spruce/pine) | 2–3.5 mm (length-weighted >2 mm; species 3–3.6, redwood 7) | 25–45 µm | 2–4 µm earlywood, 4–8 µm latewood | 0.15–0.25 mg/m | [M] lengths: USPTO/kraft hits; [E] width, wall, δ: textbook ranges (Niskanen 2008) |
| Hardwood kraft (birch, aspen, eucalyptus) | 0.7–1.2 mm (birch 1.2, aspen 0.9, maple 1.0) | 15–25 µm | 2–5 µm | 0.06–0.10 mg/m | [M] lengths; [E] rest |
| Mechanical (groundwood) | short; wiki gives "3–4 mm" average, which looks high for groundwood fines-rich pulp | — | stiff, uncollapsed | high | [M] AIC wiki, flagged |
| Cotton (rag) | native staple 20–30 mm; rag stock is cut/beaten (Hollander "chopped fibres into shorter lengths") | ≈ 20 µm | thick, collapsed ribbon | ≈ 0.12–0.20 mg/m (1.2–2 dtex) | [M] IntechOpen; AIC wiki; [E] δ |
| Cotton linters | 0.6–3.0 mm | ≈ 20 µm | — | — | [M] IntechOpen |
| Flax/linen | 25–30 mm (ultimate fibre); flax-straw pulp 1.41 mm | 20–22 µm (pulp 16.8 µm) | 3.8 µm (lumen 9.5 µm) | — | [M] IntechOpen; Sci. Rep. 2024 |
| Hemp | ≈ 20 mm | ≈ 22 µm | — | — | [M] IntechOpen |

- **Collapsed thickness.** Chemical-pulp fibres collapse to a ribbon about 2× the wall thickness, i.e. **≈ 4–10 µm** for softwood and **≈ 4–8 µm** for hardwood **[E]**. Uncollapsed latewood and mechanical fibres stay tubular at 15–30 µm, which makes a rougher, bulkier surface. In Singh's data, TMP sheets were the roughest at every wavelength.
- **[M] Rag papers.** Rag papers kept long, fibrillated fibres from stampers, while the Hollander (17th c.) cut them shorter. Linen papers retain surface impressions after wetting better than cotton, and cotton better than linters or wood (AIC wiki, BPG Western Papers).

### Formation (flocculation)
- **[M]** Formation is grammage variation at **0.1–100 mm**.
  - The **0.3–3 mm** band is governed by fibre coarseness and fines.
  - The **3–30 mm** band is governed by flocculation.
  - "Mean floc size = half the mean wavelength", where the mean wavelength splits the 2–32 mm spectrum area in half.

  (Formation-spectrum abstracts: Bipolar spectral analysis…; Fourier analysis of light-transmission images; STFI method.)
- **[D]** Random-network reference for CV of local grammage: see §3. Machine papers are more flocculated than random: **[E] 1.5–3× the random CV at 1–10 mm zones**. Handmade sheets are often cloudier. A high-cotton, long-fibre furnish (Mi-Teintes is 60 % cotton) flocculates strongly.

### Felt marks, honeycomb, wire marks
- **[M] Cylinder-mould papers.** The sheet is couched onto **wool or synthetic felt, "which will give the paper its grain"**. Canson cylinder-mould papers have a "unique range of grains due to the use of natural wool felts" (Canson papermaking page; Arches glossary).
- **[M] Fourdrinier papers.** On Canson's Fourdrinier line, "a relief roller" imprints laid lines or designs during water removal. Felts follow, then steam cylinders, then **gelatin at a size press** (Canson).
- **[M] Watercolour finishes.** Rough is pressed between textured felts. Hot-pressed goes between hot metal rollers (Jackson's).
- **[M] Mi-Teintes** is 160 g/m², 60 % cotton, with a **"honeycomb" grain on one face and fine grain on the other** (Canson / Jackson's). The mechanism is not published (felt vs marking roll).
- **[M] Sidedness.** The wire side shows indented wire marks and stronger, thicker felt-hair marks. The felt side shows finer felt-hair marks and, on Fourdrinier sheets, more fines (two-sidedness literature hit).
- **[E] Felt/honeycomb geometry.** Woven-base press felt has a cell period of about **0.3–1.5 mm** and an imprint depth of about **10–50 µm** on soft, wet-pressed rag sheets, decreasing with later dry pressing. Treat the honeycomb as a quasi-periodic cellular height field (Voronoi-like cells of about 0.5–1 mm with rims) with this depth. **Unmeasured.**

### Laid and chain lines
- **[M] Chain lines:** 15–50 mm apart, commonly **≈ 25 mm (1 inch)**. Spacing varies across a single mould by **3–6 mm** and is unequal and not quite parallel (Malta Map Society; Cornell CLiP).
- **[M] Laid lines:** about **10/cm (≈1 mm apart)**. Medieval paper shows **5–15 lines/cm** (IOBA; Heritage Science 2023).
- **[M] Origin.** Laid and chain lines are thickness and density modulations. The sheet is thinner over the wires, which drain more and carry less fibre. They show strongly in transmitted light and β-radiography.
- **[E] Relief.** Surface relief of laid lines is **≈ 5–20 µm**. Fibre-mass deficit over a wire is about 10–30 % on handmade sheets, less on machine "laid" made with a dandy roll. Model them as a grammage modulation, not a surface emboss.

### Pressing and calendering
- **[M]** Calendering lowers PPS from about 5 to about 2.5 µm and lowers compressibility. It strips variance in the 0.1–1 mm band (Singh 2008).
- **[M] Finish grades:** antique (minimal calendering), then machine and English finish, then super-calendered. Calendered surfaces are lost on wetting as fibres swell (AIC wiki). All aqueous treatment roughens paper, especially lignin-rich sheets (AIC wiki, Drying & Flattening).
- **[D] Mechanism for the model.** Wet pressing densifies the whole sheet and imprints the felt. Calendering preferentially compresses **high-grammage, high-caliper spots**. The result is more uniform thickness but non-uniform density, and the light-transmission formation pattern survives while surface relief is flattened.

### Sheet thickness and grammage
- **[M] Typical grammages:**
  - Mi-Teintes 160 g/m²
  - Tiziano 160 g/m² (rag-content, rough, neutrally sized)
  - "C" à grain 224 g/m² (alpha cellulose)
  - Ingres/laid papers about 90–130 g/m²

  (Canson, Fabriano, retailers.)
- **[D] Caliper** (not published by any maker) follows from apparent density ρ_a:
  - 160 g/m² at ρ_a = 0.6–0.8 g/cm³ gives **200–270 µm**.
  - 224 g/m² gives **280–370 µm**.
  - Sheet porosity is ε = 1 − ρ_a/1.5 ≈ **0.45–0.6**, using cellulose density 1.5 g/cm³.

---

## 3. Generative models for the sheet (formulas)

Notation:
- β = sheet grammage (g/m²)
- λ = fibre length
- ω = fibre width
- δ = coarseness (mass per length)
- t_f = collapsed fibre thickness
- ε = porosity

**Random fibre network** (Poisson line process; Kallmes & Corte 1960; Dodson; Sampson 2003). Fibre centres form a 2-D Poisson point process. Orientations are uniform, or von Mises for machine-direction anisotropy.

1. **[M] Fibre grammage** β_f = δ/ω. **Mean coverage** c̄ = β/β_f = βω/δ (number of fibres over a point). Coverage is Poisson: P(c) = c̄^c e^(−c̄)/c!. Fractional open area is e^(−c̄).
2. **[D] Point CV.** var(c) = c̄, so the **point grammage CV is 1/√c̄**.
   - Example, 160 g/m² cotton/softwood mix: δ ≈ 0.18 mg/m and ω ≈ 30 µm give β_f ≈ 6 g/m², **c̄ ≈ 27**, CV_point ≈ 0.19.
3. **[D] Zone CV.** Zones of side x ≫ λ: **CV²(x) ≈ δ λ_w / (β x²)**, where λ_w = E[λ²]/E[λ] is the length-weighted fibre length.
   - Example: δ = 0.18 mg/m, λ_w = 2 mm, β = 160 g/m². CV(1 mm) ≈ 4.7 %. CV(5 mm) ≈ 0.9 %.
   - Real formation adds the floc factor (§2).
4. **[D] Crossings.**
   - **Crossings per fibre, whole sheet (2-D projection):** n_c = (2/π) N λ², with N = β/(δλ) fibres per area, so **n_c = 2βλ/(πδ)**.
   - **Per layer of coverage c̄_layer:** n_c = (2/π) c̄_layer λ/ω. The mean free segment between crossings is ≈ πω/(2c̄_layer). For λ/ω ≈ 60 and c̄_layer ≈ 1 this gives about 38 crossings, with segments of about 50 µm.
5. **[D] Layer ACF.** Grammage autocorrelation of a single layer: α(r) ≈ 1 for r < ω, then falls about as **(ω/πr)(1 − r/λ)** to zero at r = λ. This is an order-of-magnitude form; Dodson 1971 gives it exactly. Grammage is therefore correlated over the fibre width and decorrelated beyond the fibre length. Flocs add correlation at 1–10 mm.
6. **[M] Layered (multiplanar) sheets.** Treat a layer's fractional open area as equal to sheet porosity, so **c̄_layer = ln(1/ε)**. The number of layers is ≈ c̄/c̄_layer. Sampson also gives a closed form for mean pore radius per layer, in fibre widths, as a function of ε; it is not reproduced here because the extracted text was garbled. The relative bonded area for c̄ ≤ 1 is **RBA = 1 − (1 − e^(−c̄))/c̄** (Deng & Dodson). The percolation threshold is coverage ≈ 5.7/A, with A the aspect ratio (Sampson 2003).
7. **[D] Thickness from deposition.**
   - Local caliper: T(x) ≈ c(x)·t_f/(1 − ε). Check: c̄ = 27, t_f ≈ 5 µm, ε ≈ 0.5 gives T ≈ 270 µm, consistent with §2.
   - Wet pressing makes ε lower where c is high, so T varies less than c. **[E]** Use T ∝ c^0.5–0.8.
   - KCL-PAKKA does this explicitly: it drops rectangular-section flexible fibres one by one onto the growing surface, each conforming by energy minimisation. It yields thickness, density, porosity and roughness from fibre flexibility, collapsibility and pressing (BioResources; Aalto theses).
8. **[M] Roughness against grammage.** Roughness depends on porosity and fibre thickness, not on grammage once the sheet is a few layers thick (Sampson & Wang 2020). Below c̄ of about 3–5, holes (e^(−c̄)) dominate. Flexible, well-collapsed fibres give smoother surfaces. Stiff mechanical fibres give rougher, more compressible surfaces (Singh 2008).

**Recipe implied by these** **[D/E]**:
1. Lay a Poisson/von-Mises fibre field (ribbons of width ω and thickness t_f).
2. Accumulate c(x) and multiply by a floc field (log-normal, correlation 1–10 mm).
3. Add the mould grammage modulation (laid/chain lines) before deposition.
4. Top height h(x) = T(x) − (exponential/gamma depth below envelope, mean ~t_f).
5. Imprint the felt or honeycomb (cell 0.5–1 mm, 10–50 µm).
6. Calender: clamp peaks above a percentile and reduce variance in the 0.1–1 mm band.
7. Check: PSD slope about q⁻³ between ω and λ, Ra 2–4 µm for fine grain, Rq/Ra ≈ 1.3–1.4.

---

## 4. Sizing and absorbency

- **[M] History of sizing:**
  - **Gelatin** surface/tub sizing is attested at Fabriano by **1276**. It was used with alum from the 16th c.; alum up to 20 % of the size improves ink resistance.
  - **Alum-rosin** internal sizing was introduced in Germany **1807**, elsewhere by about 1835, and was common by 1870. It is often found on thick 19th-c. Whatman papers.
  - Gelatin-sized paper does not feather ink at a crease. Rosin-sized paper does (AIC wiki, Sizing & Resizing; Western Papers).
  - Canson applies **gelatin at a size press** on its machine papers.
- **[M] Mechanism.** Sizing lowers fibre surface energy and raises the water contact angle. Bare cellulose has a low water contact angle. "Good" sizing is defined as >90° (Hubbe 2007). Cellulose surface free energy is **≈ 48–57 mN/m**, mostly dispersive, with a polar part of 8.6–12.6 mN/m (Coatings 2023). Internal sizing gives **no** true repellency: "liquids will penetrate over time" (AIC wiki).
- **[D] Why oils ignore sizing.** Surface tensions:
  - **[E] (handbook values):** linseed oil ≈ 33–35 mN/m, turpentine ≈ 27 mN/m.
  - **[M]:** cellulose 48–57 mN/m.

  Oils therefore wet cellulose completely, and probably wet rosin- or gelatin-sized fibres at small angles **[E] <~40°**. Sizing barely slows oil. Only pore-filling films (glue or oil grounds, heavy gelatin) do.
- **[D] Lucas–Washburn penetration:** **l² = r γ cosθ t / (2η)**. Ink-oil studies confirm that the rate is set by the γ/η ratio (Omya; BioResources ink–paper papers). With pore radius r = 5 µm and cosθ ≈ 1:
  - **Linseed oil** (η ≈ 0.04 Pa·s, γ = 0.034 N/m): l ≈ **1.5 mm·√(t/s)**.
  - **Turpentine** (η ≈ 1.5 mPa·s, γ = 0.027 N/m): l ≈ **6.7 mm·√(t/s)**, i.e. about 25× faster per unit time squared. Turpentine also evaporates.
- **[D] Halo size is volume-limited.** Wicked area is A ≈ V_oil/(ε·T). Example: 1 µL of oil into a 200 µm sheet with ε = 0.5 gives **≈ 10 mm² (radius about 1.8 mm)**.
- **[D] Why the halo shows.** Pigment is filtered at the surface while binder wicks ahead. Oil-filled pores (n_oil ≈ 1.48 against cellulose ≈ 1.53) kill scattering, so a **translucent, darkened halo** forms around strokes.
- **[M] Degas's peinture à l'essence:** oil leached from the paint on blotting paper, then thinned with turpentine for a dry, matte surface (Norton Simon). This reduces the oil reservoir that drives halos.
- **[M] Damage.** Absorbed linseed oil accelerates cellulose oxidation, discolouration and embrittlement, especially in unsized, unbuffered cotton paper (Banou et al. 2023, Polymers).

---

## 5. How pastel and charcoal adhere; tooth capacity; fixatives

- **[M] Adhesion is mechanical and weak.** Pastel binder content is low. Particles are held "only by the mechanical interaction between rough agglomerates and paper surfaces". Adhesion depends on hand pressure and paper texture. Abrasive surfaces both pull particles from the stick and retain them. Pastels lose agglomerates under vibration: transport shocks reach **8–10 g**, and damage accumulates as fatigue (Sauvage, Wei & Martinez 2018).
- **[M] Particle sizes** in a black pastel: **1–20 µm** particles plus about **60 nm** grains (search-summarised pastel study, provenance uncertain).
- **[M] Pencil analogue** (Archambault et al. 2026), probably transferable in form:
  - Graphite deposits on ridges where the lead contacts paper. The Archard wear coefficient is **K ≈ 10⁻³–10⁻²** on paper, against 10⁻¹–1 on frosted glass of similar h_rms. Paper's soft fibre topography wears the lead 20–100× less.
  - Debris areas follow a power law, **p(A) ∝ A^−3/2**, with an upper cutoff that rises with load and with softness.
  - Grey level is set by **fractional coverage ≈ 6.6 × F_N/(σ_p A₀)**, the real contact fraction.
  - In the simulator: deposit where the stick's rigid bearing plane meets the top of the height field, i.e. Abbott material ratio at the current indentation.
- **[D/E] Tooth capacity.** No published mg/cm² was found. Estimate from the void volume between peak envelope and valleys, which is about the mean depth μ ≈ Rq:
  - Fine grain, Rq 3–5 µm: about 2–3 µm equivalent of 50 %-packed pigment at ρ ≈ 2.5 g/cm³, i.e. **≈ 2–5 g/m² (0.2–0.5 mg/cm²)** before the "tooth fills".
  - Rough or honeycomb, Rq 10–20 µm: **≈ 10–25 g/m² (1–2.5 mg/cm²)**.
  - Sanded papers (P400–P800 grit ≈ 35–22 µm grains; FEPA standard) hold more.

  The tooth "fills" when the bearing plane has descended to about the 90–95 % material ratio.
- **[M] Fixatives:**
  - **Historical:** casein and skim milk (Degas, Sloan); casein "binds effectively in low concentrations with little change in saturation". Also parchment size.
  - **Conservation:** Flieder's tests found **1 % Klucel G in ethanol:water 50:50** and **0.5 % Elvamide 8061** in ethanol:water 90:10 consolidated pastel mock-ups with least change. Both slightly darkened. **Paraloid B-72 at 2 %** in toluene/ethanol 40/60 is airbrushed. At ≥3 % B-72 looks shiny or yellow and puddles. If the solvent flashes before arrival, the resin lands as a "beaded" coating.

  (AIC wiki, Consolidation, Fixing and Facing.)
- **[D] Fixative amount.** Spraying 20–50 g/m² of a 1–2 % solution deposits **0.2–1 g/m² of resin, ≈ 0.2–0.9 µm** if it were a continuous film. In practice the resin penetrates and forms **bridges at particle–particle and particle–fibre contacts**, not a film, so micro-roughness at the 10 µm scale is essentially unchanged.
- **[D] Optical effect.** Darkening comes from refractive-index matching: resin n ≈ 1.48–1.5 replaces air (n = 1) around particles of n ≈ 1.5–1.7. Use a per-particle "wetted" fraction that rises with dose.

---

## 6. Numbers table

| Quantity | Value | Tag | Source |
|---|---|---|---|
| Kent drawing paper Ra / Rz (stylus) | 2.57 / 15.2 µm | M | arXiv 1604.01540 |
| Office/printing papers Ra | 1.25–2.95 µm | M | same |
| Printer paper PSD | C(q) ∝ q⁻³ (H ≈ 0.5), 2 mm field | M | Archambault et al. 2026 |
| Dominant roughness band | 0.1–1 mm wavelength | M | Singh 2008 |
| PPS uncalendered / calendered (1 MPa) | ≈4–6 / ≈2–3 µm | M | Singh 2008 |
| Uncoated base paper | PPS <5–7 µm; Bendtsen 200–400 mL/min | M | Smithers/USPTO |
| PPS land | 51 µm × 98 mm; Δp 6.2/19.6 kPa; 0.5–2 MPa | M | ISO 8791-4 |
| Bendtsen to gap | 100 → 7 µm, 1000 → 15 µm | D | §1 formula |
| Rq/Ra (exponential depths) | 1.36; height skew −2; kurtosis 9 | D | Dodson, Sampson–Wang |
| Softwood / hardwood length | 2–3.5 / 0.7–1.2 mm | M | kraft hits |
| Fibre width (wood/cotton/flax) | 15–45 / ≈20 / 17–22 µm | M/E | IntechOpen, Sci. Rep. 2024 |
| Collapsed fibre thickness | 4–10 µm | E | textbook range |
| Coarseness | SW 0.15–0.25, HW 0.06–0.10, cotton 0.12–0.2 mg/m | E | textbook range |
| Mean coverage, 160 g/m² | ≈27 | D | c̄ = βω/δ |
| Grammage CV random, 1 mm zone | ≈5 % (×1.5–3 real) | D/E | §3 |
| Floc size | 1–15 mm (formation band 3–30 mm) | M | formation literature |
| Chain line spacing | ≈25 mm (15–50), ±3–6 mm | M | CLiP, Malta |
| Laid line density | 5–15 /cm (≈10) | M | IOBA, Heritage Sci. |
| Laid relief | 5–20 µm | E | — |
| Felt/honeycomb cell, depth | 0.3–1.5 mm, 10–50 µm | E | — |
| Drawing paper grammage | 90–250 g/m² (Mi-Teintes 160) | M | makers |
| Caliper at 160 g/m² | 200–270 µm | D | ρ_a 0.6–0.8 |
| Porosity | 0.45–0.6 | D | 1 − ρ_a/1.5 |
| Cellulose SFE | 48–57 mN/m | M | Coatings 2023 |
| Linseed oil / turpentine γ, η | 33–35 / 27 mN/m; 40 / 1.5 mPa·s | E | handbook |
| Oil wicking front (r = 5 µm) | linseed 1.5 mm·√s, turps 6.7 mm·√s | D | Lucas–Washburn |
| Pencil wear K on paper | 10⁻³–10⁻² | M | Archambault et al. |
| Debris area PDF | A^−3/2, cut-off ∝ load/softness | M | same |
| Coverage vs contact | ≈6.6·A_real/A₀ | M | same |
| Pastel tooth capacity | 0.2–2.5 mg/cm² | E | §5 |
| Fixative resin dose | 0.2–1 g/m² (≈0.2–0.9 µm equiv.) | D | §5 |
| Transport accelerations | up to 8–10 g | M | Sauvage et al. 2018 |

---

## 7. Notes for the simulator (physical facts only)

- **Separate grammage from surface.** Paper has two related fields: grammage (formation; laid, chain and felt marks) and surface height. Grammage drives transmitted-light appearance, absorbency and local caliper. Height drives pastel pickup. Pressing and calendering decouple the two.
- **Height is plateau-and-valley.** Pastel loads the valleys last, so tooth "filling" is a march down the Abbott curve.
- **Sizing controls water, not oil.** Oil and turpentine follow pore radius and viscosity.

## 8. Gaps worth one bench measurement
1. Phone-camera photometric stereo or a confocal scan of Mi-Teintes (both faces), Tiziano, an Ingres laid sheet and a cartridge paper. Extract PSD, Ra/Rq, skewness/kurtosis and the Abbott curve.
2. Micrometer caliper of a 10-sheet stack of each paper.
3. Weigh a 10 × 10 cm² patch before and after loading it with soft pastel until refusal, to get the tooth capacity.

---

## Sources
- Archambault, Domino & Bertin, "The physics of pencil drawing", Research Square preprint 2026: https://www.researchsquare.com/article/rs-10448967/v1
- Rachi et al., "Choice of Paper for Multigraphene Growth on Lead Pencil Drawing", arXiv 1604.01540: https://arxiv.org/pdf/1604.01540
- Singh S.P. (2008) "Paper smoothness evaluation methods", BioResources 3(2) 503–516: https://bioresources.cnr.ncsu.edu/BioRes_03/BioRes_03_2_0503_Singh_PaperSmoothnessEval_Methods.pdf
- Singh, Rao & Bristow (1991) "Smoothness characterization of printing papers", IPPTA: https://ippta.co/wp-content/uploads/2021/01/IPPTA-CI-1991-1-13-Smoothness-Characterization-of.pdf
- ISO 8791-4 (Print-surf) preview: https://cdn.standards.iteh.ai/samples/16218/a17f0641bca74072919e36a9c48b2196/ISO-8791-4-1992.pdf ; ISO 8791-2 (Bendtsen): https://www.iso.org/standard/51265.html
- Smithers, Roughness and smoothness: https://www.smithers.com/industries/packaging/manufacturers-and-users/packaging-materials-testing/paper-testing-surface-properties/roughness-and-smoothness
- Sampson & Wang (2020) "A model for roughness statistics of heterogeneous fibrous materials", J. Mater. Sci. 55:2636 (abstract): https://doi.org/10.1007/s10853-019-04193-1
- Sampson (2003) "The statistical geometry of fractional contact area in random fibre networks", JPPS 29(12): https://personalpages.manchester.ac.uk/staff/william.sampson/pdf/FracContAreaJPPS.pdf
- Alava & Niskanen (2006) "The physics of paper", Rep. Prog. Phys. 69:669: https://iopscience.iop.org/article/10.1088/0034-4885/69/3/R03/pdf
- Niskanen (ed.) Paper Physics, 2nd ed. 2008 (Fapet), surface chapter via ForestBioFacts (members only): https://forestbiofacts.com/natural-fibre-products/paper-physics/paper-surface-and-thermal-electrical-and-friction-characteristics/surface-roughness/
- KCL-PAKKA simulation: https://bioresources.cnr.ncsu.edu/resources/kcl-pakka-simulation-of-the-3d-structure-of-paper/
- Pulping and papermaking of non-wood fibres (IntechOpen): https://www.intechopen.com/chapters/62223
- Flax straw pulp (Sci. Rep. 2024): https://www.nature.com/articles/s41598-024-74096-y
- Formation spectra: https://www.researchgate.net/publication/287605229 ; https://www.researchgate.net/publication/271966623
- AIC Wiki: BPG Western Papers https://www.conservation-wiki.com/wiki/BPG_Western_Papers ; Sizing & Resizing https://www.conservation-wiki.com/wiki/Sizing_&_Resizing ; Consolidation, Fixing and Facing https://www.conservation-wiki.com/wiki/BPG_Consolidation,_Fixing,_and_Facing
- Hubbe (2007) "Paper's resistance to wetting – a review of internal sizing", BioResources 2(1): https://bioresources.cnr.ncsu.edu/BioRes_02/BioRes_02_1_106_145_Hubbe_Sizing_of_Paper_Review.pdf
- Surface free energy of cellulose layers, Coatings 13:259 (2023): https://doi.org/10.3390/coatings13020259
- Banou et al. (2023) "Oil media on paper…", Polymers 15:2567: https://pmc.ncbi.nlm.nih.gov/articles/PMC10255726/
- Omya, ink-oil penetration: https://www.omya.com/Documents/Publications/24%20Distribution%20of%20Offset%20Ink%20Constituents.pdf
- Canson, papermaking: https://en.canson.com/papermaking ; Mi-Teintes: https://en.canson.com/mi-teintesr
- Arches glossary, cylinder mould: https://arches-papers.com/glossary/cylinder-mould/
- Fabriano Tiziano: https://fabriano.com/en/product/tiziano/ ; Ingres: https://fabriano.com/en/product/ingres/
- Jackson's, watercolour paper textures: https://www.jacksonsart.com/blog/2016/09/13/understanding-watercolour-paper-visual-guide/
- Cornell CLiP (chain-line patterns): https://people.ece.cornell.edu/johnson/CLiP-Herstmonceux.pdf
- Laid/chain lines in medieval paper, Heritage Science 2023: https://www.nature.com/articles/s40494-023-01013-3 ; Malta Map Society: https://maltamapsociety.mt/glossary/laid-lines/ ; IOBA: https://www.ioba.org/laid-paper
- Sauvage, Wei & Martinez (2018) "When conservation meets engineering… vibrations on pastel paintings", Stud. Conserv. 63 sup1: https://www.tandfonline.com/doi/full/10.1080/00393630.2018.1504444
- Norton Simon Museum, Degas technique note: https://www.nortonsimon.org/art/detail/F.1969.35.1.P
