# Pastel stick tribology: the stick, the paper, the hand

Research notes for modelling a soft pastel stick that rests on paper roughness under a hand's force, wears to a facet, and leaves particles behind. Compiled 2026-10-05.

**Tags:** **[M]** measured, from a cited source. **[D]** derived here from measured values, arithmetic shown. **[E]** estimate with no direct measurement; treat it as a tunable range.

**The main gap.** I found no published tribology of *pastel* itself: no wear coefficient, hardness or deposited mass per length. The quantitative anchor is one pencil-graphite study, Archambault, Domino & Bertin, "The physics of pencil drawing" (Research Square preprint, 29 Jul 2026, not peer reviewed). It runs drawing as a controlled Archard wear experiment on printer paper. Pastel numbers are scaled from it and flagged. §7 gives a one-hour bench calibration.

---

## 1. The stick

### Composition
- **[M]** Pastel is powdered pigment plus a weak binder: gum tragacanth, gum arabic, or (20th c. on) methyl cellulose. Fillers are chalk/whiting (CaCO₃), kaolin, talc, sometimes gypsum. Soft pastels have more pigment and less binder; hard pastels more binder and filler (WebExhibits; Wikipedia "Pastel").
- **Binder dose.**
  - **[M]** Recipes use tragacanth at 1 part gum to 30 parts water. Methyl cellulose is used at ½ to ⅛ of wallpaper-paste strength (WetCanvas recipe thread).
  - **[D]** Doughing takes about 0.3–0.5 mL of 3.3 % solution per gram of powder, so the finished stick holds **about 1–2 wt % dry binder**.
- **[M] Fillers.** Recipes use filler : pigment = 2:1, and blends such as CaCO₃ : talc : kaolin = 4:1:1. Commercial formulations are unpublished.
- **[M] Conté** is kaolin plus pigment (graphite/carbon for blacks), a cellulose-ether binder and a little wax. **Blackboard chalk** is mostly gypsum (CaSO₄·2H₂O). Older chalks are CaCO₃ bound with kaolin.

### Density
- **[M]** Solid phases: gypsum 2.2–2.4 g/cm³, calcite 2.71, kaolinite ≈ 2.6.
- **[D]** A Sennelier full stick (≈ 13 × 64 mm, 8.5 cm³) is about 9 g (retailer listing). That is **ρ ≈ 1.0–1.1 g/cm³, about 60 % porosity**.
- **[E]** Use ρ ≈ 1.0–1.4 g/cm³ for soft pastel and 1.5–2.0 g/cm³ for hard pastel and Conté.

### Strength and hardness
No measured values exist for artist pastel. Proxies:
- **[M]** Natural chalk rock: uniaxial compressive strength **1–10 MPa** (Mortimore).
- **[M]** Shell-derived blackboard chalk: breaking strength **5.4–5.5 MPa** (56.4 and 54.7 kg/cm²; ASRIC).
- **[M]** Graphite leads, the calibrated analogue:
  - Microindentation hardness σp is **about 50 MPa (6B) to 1,100 MPa (4H)**.
  - Reduced modulus E* is about 3.5 GPa (6B) to 18–20 GPa (hard grades). The grade alignment is uncertain because the table was garbled in extraction.
  - Nanoindentation of F–8H leads gives 338–833 MPa (*Adv. Appl. Ceram.* 2016).
- **[E] Pastel.** Soft pastel σp ≈ **1–10 MPa**, at least 5–50× softer than 6B. Hard pastel and Conté are about 10–50 MPa. The effective modulus of a porous compact is about 0.1–2 GPa.

### Geometry [M]
- **Soft round sticks:** Ø 11–13 × 64–70 mm (Sennelier, Rembrandt, Schmincke).
- **Hard square sticks:** 6–6.35 mm square × 65–92 mm (Conté carré, NuPastel).
- **Pastel-pencil cores:** Ø 4.3–4.7 mm (Faber-Castell Pitt, Stabilo CarbOthello, Caran d'Ache).

---

## 2. Wear and deposited mass

### What is measured (graphite)
- **[M] Archard's law:** V/L = K·F/σp.
- **[M] Linear, no threshold.** Transferred volume is linear in load and in sliding distance, with no threshold seen. Tested at 0.5–2.5 N on 70 g/m² printer paper and 0.2–4 N on frosted glass, at 2 cm/s. Above about 2.5 N the paper deformed permanently.
- **[M] Contact geometry doesn't matter.** A fresh tip (contact Ø 0.5 mm) and a worn one (Ø 1 mm) transfer the same volume. Only load, distance, hardness and counterface count. This is the key simplification for a faceting stick.
- **[M] Wear coefficient.**
  - Graphite on paper: **K ≈ 10⁻³–10⁻²**.
  - On frosted glass: K ≈ 10⁻¹–1. That is **20–100× the paper value at the same rms roughness**.
  - Smooth float glass leaves no trace. On frosted glass, wear rises with h_rms over 0.57–6.74 µm.
  - Sharp, hard asperities matter more than rms height.
- **Speed.**
  - **[M]** Not varied in the study.
  - **[E]** Brittle Archard wear does not depend on speed at 10–200 mm/s.
  - **[M]** Handwriting force is only weakly coupled to kinematics (coherence < 0.5; Schomaker & Plamondon 1990).
  - **Model deposit per mm as independent of speed.** Fast strokes look lighter because people press less and lift sooner.

### Mass per length
- **[D] Graphite,** using lead density ≈ 2 mg/mm³:
  - 6B (K 10⁻², σp 50 MPa): **≈ 0.4 µg/mm at 1 N**.
  - HB (K 3×10⁻³, σp ≈ 300 MPa): ≈ 0.02 µg/mm at 1 N.
- **[D/E] Soft pastel on drawing paper.** Two routes agree to within 10×:
  1. **Archard scaling.** σp 2–5 MPa and K 10⁻²–10⁻¹ (friable compact) give V/(F·L) ≈ 2×10⁻³–5×10⁻² mm²/N.
  2. **Coverage check.** A light pass about 5 mm wide lays down about 0.2–0.5 mg/cm². For comparison, a saturated layer about 20 µm thick at 1.3 g/cm³ is ≈ 2.6 mg/cm². At 2–3 N, a light pass is about 10–25 µg/mm, or ≈ 4×10⁻³ mm²/N.
- **Working value: 2–50 µg/mm per newton; start at 5 µg/mm/N** on Mi-Teintes-type paper.
  - Sanded paper: about 10× more.
  - Hard pastel / Conté: about 0.5–5 µg/mm/N.
- **[D] Stick consumption.** Once a full facet exists, the stick shortens by (V/L)/(πR²) per mm of stroke. For an 11 mm stick at 2 N that is ≈ 0.1 µm per mm, or about 10 mm per 100 m of stroke.
- **Load threshold.**
  - **[M]** Graphite: none.
  - **[E]** Pastel: below about 0.05–0.2 N only the highest fibre tops are touched, and much of the debris stays loose.
  - **[M]** Hard leads (2H, 4H) depend less on load than Archard predicts.

---

## 3. The deposit

### Morphology
- **[M] Discrete fragments, not a film.** Graphite decorates the fibre network "along ridges and valleys where contact with the lead occurs".
- **[M] Fragment-area distribution.**
  - P(A) ∝ **A^(−3/2) over about 4 decades**, robust across grades and loads.
  - The upper cut-off A_max grows with load and with lead softness.
  - Because the exponent is below 2, covered area is dominated by the largest fragments: ⟨A⟩ ∝ A_max^½.
- **[M] Mechanism.** The distribution reproduces the microcontact islands of a self-affine paper surface.
  - Paper height spectrum: C(q) ∝ q^(−2(1+H)), H ≈ 0.5, fibres 5–15 µm.
  - Islands come from a fully plastic bearing-area cut at A_real = F/σp.
- **Simulation recipe:** generate an H ≈ 0.5 heightfield and take the top pixels until their area equals F/σp. Each connected island is a debris source sized by the island.

### Sizes
- **[M] Measured pastel media** (vibration studies, *Stud. Conserv.* 2018):
  - Black: **1–20 µm particles plus ≈ 60 nm grains**.
  - Pink: needle-shaped particles.
  - Pastel moves as **agglomerates**, and adhesion depends on composition.
- **[E] Constituents:** kaolin 0.2–5 µm, whiting 1–20 µm, organic pigments < 1 µm, earths 1–30 µm.
- **[E] Crumbs** from soft pastel on toothy paper run from a few µm up to about 100–300 µm.

### Placement and grey level
- **[M] Where it lands.** Primary deposit sits at contacts: asperity tops and ridge flanks.
- **[M] Coverage vs contact.** Coverage ≈ **6.6 × A_real/A₀** = 6.6·F/(σp·A₀): debris spreads, fragments and collects in texture.
- **[M] Grey level** is set by **area coverage** (checked against scans at 1.5 mm resolution), not by thickness.

### Repeated strokes and saturation
- **[M] Third body.** On repeat passes the deposit carries load, lubricates and limits wear (noted by Archambault et al., not measured).
- **[M] Pencil electrodes** (*ACS Omega* 2021, loads 50–350 g):
  - Resistance plateaued after **≈ 50 passes**, when "graphite particles filled the depths (valleys) of the rough paper".
  - Roughness was "sufficiently reduced and interfered with frictional sliding". This is the measured form of pastellists' "filled tooth".
- **[D] Model.**
  - Coverage after n passes: c = 1 − (1 − c₁)ⁿ.
  - Let the effective roughness amplitude, and with it K, decay as valleys fill, so deposit per pass tends to zero.
  - **[E]** Pastel saturates in about 3–10 heavy layers on standard paper.

---

## 4. Contact

### Real contact area
- **[M] Plastic limit (Bowden–Tabor):** A_real ≈ F/σp. This is appropriate here: cellulose fibre walls are far stiffer than pastel.
- **[M] Elastic limit (Greenwood–Williamson / Persson):** A_real/A₀ ≈ κ·p₀/(E*·h′_rms), κ ≈ 2. Both limits give A_real ∝ F.
- **[D] Pastel at 2 N.**
  - With σp 2–5 MPa, A_real ≈ 0.4–1 mm².
  - On a 20–100 mm² facet, A_real/A₀ ≈ 0.4–5 %.
  - Applying the 6.6× spread gives single-pass coverage of about 3–30 %. That matches the broken texture of a light stroke.

### Paper
- **[M] Roughness.**
  - Copier and Kent paper: Ra 2–3 µm, Rz ≈ 15 µm.
  - Printing paper: Ra ≈ 1 µm.
  - Uncoated depressions: 5–25 µm deep.
- **[M] Sanded papers** (UArt 240–800) carry FEPA grit: P240 ≈ 58 µm, P400 ≈ 35 µm, P600 ≈ 26 µm, P800 ≈ 22 µm.
- **[M] Elastic constants** (Mann, Baum & Habeger 1979, ultrasonic, board):
  - **E₃₃ (z) = 39 MPa**, E₂₂ = 3.47 GPa, E₁₁/E₃₃ ≈ 190.
  - Copy paper in-plane E is about 2–8 GPa.
  - Z-compression is strongly nonlinear.
- **[D] Compliance matters.**
  - A 2 N stick on a 10–50 mm² facet gives 0.04–0.2 MPa nominal pressure.
  - At E_z 10–40 MPa, a 100–250 µm sheet compresses about 0.1–5 µm. That is comparable to Ra, so more fibre tops come into contact.
  - A soft underlay increases this; a hard board reduces it.

### Facet and tilt
- **[M] Facet orientation.** Because wear does not depend on nominal area, the stick wears flat parallel to the paper. Deposit per mm stays constant as the facet grows and pressure drops.
- **[D] Full facet** on a cylinder of radius R at altitude θ: an ellipse with semi-axes R and R/sin θ.
  - 11 mm stick at 45°: 11 × 15.6 mm.
  - At 62°: 11 × 12.5 mm.
  - Laid flat: a strip 30–70 mm long, width 2√(2Rδ) at wear depth δ.
- **[D] Early in wear,** before the full ellipse forms, the facet is an elliptical segment. Its area grows as δ^(3/2), so the first marks are narrow.
- **[D] Re-tilting or rolling** the stick cuts a new, small, sharp-edged facet: the artist's "fresh edge".
- **Friction.**
  - **[M]** Ballpoint ≈ 4 % of normal force (Dooijes, via Schomaker).
  - **[M]** Pencil lines: μ ≈ 0.19–0.22 (pencil-lead patent).
  - **[E]** Pastel: μ ≈ 0.3–0.6.

---

## 5. Hand forces and kinematics

### Forces
- **[M] Handwriting** (Schomaker & Plamondon 1990, 16 writers, ballpoint):
  - Mean axial force **0.82–1.13 N** (84–115 g), SD 0.16–0.27 N.
  - Force spectrum peaks at **2–5 Hz**.
  - Pen angle ≈ 50°, varying less than 10°.
- **[M] Tablet** (Xin, Bi & Ren 2012, Wacom Cintiq, writing and drawing):
  - Mean tip pressure 791/1023 (SD 194) on a 0–4 N scale. The scale is nonlinear, so read it as **≈ 2–3 N**.
  - Normal-use range 1.6–4 N nominal.
  - Pressure always starts from 0 at landing.
- **[M] Grip** (*Measurement* 2013): grip/normal force = 4.3 ± 1.5. Normal force lags grip by **97.7 ± 16 ms**.
- **[E] Pastel force ranges:**

  | Stroke | Normal force |
  |---|---|
  | Feather scumble | 0.1–0.5 N |
  | Normal | 0.5–2 N |
  | Heavy | 2–5 N |
  | Crushing broadside | 5–10 N |

  Light paper deforms permanently above about 2.5 N.

### Speed and profiles
- **[M] Handwriting strokes:** 100–150 ms over 2–10 mm, about 10–100 mm/s (JFDE review). Healthy stroke duration 0.12 s (*Sci. Rep.* 2017). Scribbling runs at 4.5 Hz, circling at 3.5 Hz.
- **[E] Hatching** at 3–5 Hz over 10–40 mm gives 60–400 mm/s peaks. Sweeping pastel strokes last 0.3–1 s.
- **[M] Minimum-jerk** (Flash & Hogan 1985): s(τ) = 10τ³ − 15τ⁴ + 6τ⁵. Bell-shaped speed, **peak/mean = 1.875**.
- **[M] Sigma-lognormal** (Plamondon): v(t) = D/(σ√(2π)(t−t₀))·exp(−[ln(t−t₀)−μ]²/(2σ²)).
  - The profile is asymmetric: fast rise, long tail.
  - O'Reilly & Plamondon (2008) used windows **μ ∈ [−2.0, −1.0]** and **σ ∈ [0.2, 0.5]**, with an example at μ = −1.5, σ = 0.25. The minus signs were lost in extraction and are inferred.
- **Curvature:** the two-thirds power law, v ∝ κ^(−1/3) (Lacquaniti 1983; standard result, not re-verified).

### Onset and lift-off
- **[M]** Force ramps from zero at every landing. Low values occur "especially when the pen tip just lands".
- **[E/D] Ramp model.** Given the 2–5 Hz bandwidth:
  - Ramp-up takes about **30–100 ms**, ramp-down about **20–80 ms**.
  - Lift-off is usually a moving flick, which leaves a tapered tail.
  - Model F(t) as a smoothstep over those windows, plus a 2–5 Hz wobble of ±15–25 % (the SD/mean above).

### Tilt
- **[M] Stylus altitude:** 62.4° ± 7.5°, azimuth 131.6° ± 21.5° (Xin et al.).
- **[E] Pastel:** about 45–70° for tip work, 0–20° for broadside strokes.

---

## 6. Smudging and blending

**No quantitative study of finger or stump blending was found.** The mechanism below is assembled from contact data.
- **[M] The finger.**
  - Finger-pad contact is **124–151 mm² at 1 N**, growing with force as a power law with exponent 0.1–0.5 (≈ 0.4).
  - Friction coefficient is 0.27–1.15, falling with load.
  - Nominal pressure is therefore **≈ 7–8 kPa**, 5–30× below a pastel facet. The skin conforms into valleys that the rigid stick bridges.
- **[E] What the finger does:**
  - **Transport:** it shears loose third-body material (agglomerates on asperity tops) downstream, as a decaying smear kernel about 1–10 mm long.
  - **Breakup:** it breaks agglomerates into 0.1–20 µm grains, raising coverage at lower thickness.
  - **Packing:** it pushes grains into valleys, taking coverage toward 100 % and flattening the tooth. This is the same valley-filling that saturates graphite.
  - **Pick-up:** skin lipids pick up about 10–30 % per pass and redeposit it.
- **[E] Stump / tortillon.** Rolled paper is far stiffer than skin and contacts only a few mm².
  - At 0.5–2 N that is 0.1–1 MPa, similar to the stick, so it compacts and transports but reaches less into valleys.
  - Once loaded with pigment it acts as a low-K secondary stick.
- **[M] Adhesion.** Fixatives raise agglomerate adhesion. Vibration loss is cumulative, with Wöhler-type fatigue (vibration studies).

---

## 7. Calibration experiment (≈ 1 hour)
1. Weigh a soft stick on a 1 mg balance.
2. Draw 2 m of straight lines on Mi-Teintes at a known load, using a kitchen scale under the board, at 1 N and at 3 N.
3. Re-weigh. The predicted loss is 4–300 mg.
4. Compute V/(F·L).
5. Photograph a 5 × 5 mm patch to measure single-pass coverage c₁.

This replaces every [E] in §2–4.

---

## 8. Numbers table

| Quantity | Value | Tag | Source |
|---|---|---|---|
| Archard law, graphite on paper | V ∝ F·L, linear, no threshold (0.5–2.5 N) | M | Archambault et al. 2026 |
| K, graphite on paper | 10⁻³ – 10⁻² | M | Archambault |
| K, graphite on frosted glass | 10⁻¹ – 10⁰ (20–100× paper) | M | Archambault |
| Lead hardness σp | ~50 MPa (6B) – 1,100 MPa (4H) | M | Archambault |
| Lead hardness, F–8H nanoindentation | 338–833 MPa | M | Adv. Appl. Ceram. 2016 |
| Lead reduced modulus E* | ~3.5 – ~20 GPa | M | Archambault |
| Graphite deposit, 6B at 1 N | ~0.4 µg/mm | D | from K, σp |
| Soft pastel deposit | 2–50 µg/mm per N (start at 5) | D/E | §2 |
| Soft pastel σp | 1–10 MPa | E | chalk UCS proxy |
| Natural chalk UCS | 1–10 MPa | M | Mortimore |
| Blackboard chalk breaking strength | 5.4–5.5 MPa | M | ASRIC |
| Gypsum density | 2.2–2.4 g/cm³ | M | — |
| Soft pastel bulk density | ~1.0–1.4 g/cm³ | D/E | Sennelier mass/volume |
| Dry binder fraction | ~1–2 wt % | D | 1:30 tragacanth recipes |
| Debris area PDF | ∝ A^(−3/2), ~4 decades; cut-off rises with F, falls with σp | M | Archambault |
| Coverage vs real contact | ≈ 6.6 · F/(σp·A₀) | M | Archambault |
| Passes to saturation (graphite) | ~50 | M | ACS Omega 2021 |
| Pastel particles | 1–20 µm + 60 nm grains; agglomerates | M | Stud. Conserv. 2018 vibration studies |
| Paper Hurst exponent | H ≈ 0.5; fibres 5–15 µm | M | Archambault |
| Copy/Kent paper roughness | Ra 2–3 µm, Rz ~15 µm | M | patent profilometry |
| Paper E_z | 39 MPa (board); E₁₁/E₃₃ ≈ 190 | M | Mann, Baum & Habeger 1979 |
| Sanded-paper grit | P240 58 µm … P800 22 µm | M | FEPA |
| Handwriting axial force | 0.82–1.13 N mean, SD 0.16–0.27 N | M | Schomaker & Plamondon 1990 |
| Tablet tip force (writing/drawing) | ~2–3 N mean (nonlinear scale) | M/D | Xin, Bi & Ren 2012 |
| Grip/normal force ratio; lag | 4.3 ± 1.5; 97.7 ± 16 ms | M | Measurement 2013 |
| Pen altitude | 62.4° ± 7.5° (stylus); ~50° (ballpoint) | M | Xin et al.; Schomaker |
| Stroke duration / speed | 100–150 ms; 10–100 mm/s | M | JFDE review |
| Force spectrum peak | 2–5 Hz | M | Schomaker & Plamondon |
| Minimum-jerk peak/mean speed | 1.875 | M | Flash & Hogan 1985 |
| Lognormal μ, σ windows | μ −2.0 to −1.0; σ 0.2–0.5 | M | O'Reilly & Plamondon 2008 |
| Force ramp on / off | 30–100 ms / 20–80 ms | E | from bandwidth |
| Ballpoint friction | ~4 % of normal force | M | Dooijes via Schomaker |
| Pencil line friction coefficient | 0.19–0.22 | M | pencil-lead patent |
| Finger contact at 1 N | 124–151 mm², exponent ~0.4 | M | finger-pad contact studies |
| Finger friction coefficient | 0.27–1.15 | M | same |
| Stick sizes | soft Ø 11–13 × 64–70 mm; NuPastel 6.35 mm sq × 92; Conté 6 × 6 × 65; pencil cores 4.3–4.7 mm | M | manufacturers |

---

## Sources

- Archambault, Domino & Bertin, "The physics of pencil drawing", Research Square preprint (2026). https://www.researchsquare.com/article/rs-10448967/v1 (PDF: https://assets-eu.researchsquare.com/files/rs-10448967/v1_covered_a1432c66-bc39-4cdc-a583-0b1a1092867d.pdf)
- Schomaker & Plamondon, "The relation between pen force and pen-point kinematics in handwriting", *Biol. Cybern.* 63, 277–289 (1990). https://www.ai.rug.nl/~lambert/papers/pen-pressure.pdf
- Xin, Bi & Ren, "Natural use profiles for the pen: pressure, tilt, azimuth", CHI 2012. https://www3.cs.stonybrook.edu/~xiaojun/pdf/pta.pdf
- O'Reilly & Plamondon, "Automatic extraction of sigma-lognormal parameters on signatures", ICFHR 2008. http://www.cenparmi.concordia.ca/ICFHR2008/Proceedings/papers/cr1020.pdf
- Flash & Hogan, *J. Neurosci.* 5, 1688 (1985). https://www.jneurosci.org/content/jneuro/5/7/1688.full.pdf
- Mann, Baum & Habeger, "Determination of all nine orthotropic elastic constants for machine-made paper", IPC Tech. Paper 84 (1979). https://repository.gatech.edu/server/api/core/bitstreams/f9fdd8cb-515b-48d4-83f3-d3bc953bc31e/content
- "Vertically and horizontally drawing formation of graphite pencil electrodes on paper by frictional sliding", *ACS Omega* (2021). https://pubs.acs.org/doi/10.1021/acsomega.0c04792
- "Measurement of hardness and friction properties of pencil leads…", *Adv. Appl. Ceram.* 115, 443 (2016). https://www.researchgate.net/publication/303440255
- "Quantification of handwriting performance: force acquisition pen…", *Measurement* (2013). https://www.sciencedirect.com/science/article/abs/pii/S026322411200303X
- "When conservation meets engineering: predicting the damaging effects of vibrations on pastel paintings", *Stud. Conserv.* (2018). https://www.tandfonline.com/doi/full/10.1080/00393630.2018.1504444; "Transport of pastel paintings: fatigue damage due to vibrations". https://www.researchgate.net/publication/326677999
- Mortimore, "The engineering description of chalk: its strength, hardness and density". https://www.researchgate.net/publication/274973370
- ASRIC, "Processing of blackboard chalk from *Pachymelania aurita* and *Lanistes varicus* shells". https://asric.africa/sites/default/files/2025-02/
- Finger-pad contact: Dzidek et al., *J. R. Soc. Interface* 14 (2017). https://royalsocietypublishing.org/rsif/article/14/127/20160935; "Measuring contact area in a sliding human finger-pad contact". https://eprints.whiterose.ac.uk/id/eprint/116164/
- JFDE review, "Temporal features of handwriting". https://jfde.org/index.php/jfde/article/download/255/131
- MS handwriting kinematics, *Sci. Rep.* (2017). https://www.nature.com/articles/s41598-017-18066-7
- Pastel composition: https://www.webexhibits.org/pigments/intro/pastel.html; https://en.wikipedia.org/wiki/Pastel; https://www.wetcanvas.com/forums/topic/instructions-for-making-your-own-pastels/; https://www.liveabout.com/how-to-make-your-own-pastels-2579005; https://en.wikipedia.org/wiki/Cont%C3%A9
- Stick dimensions: Jerry's Artarama / Dick Blick / Dakota Pastels listings (Sennelier, Rembrandt, Schmincke, NuPastel); Conté à Paris (https://eu.conteaparis.com/blogs/tutoriels/qu-est-ce-qu-un-carre); Faber-Castell, Stabilo, Caran d'Ache product pages.
