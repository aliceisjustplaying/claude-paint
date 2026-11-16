# Pastel stick tribology: the stick, the paper, the hand

Research notes for modeling a soft pastel stick that rests on paper roughness under a hand's force, wears to a facet, and leaves particles behind. Compiled 2026-10-05.

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

### Sharpening a pastel pencil
These are behind the easel's `--@ pastel-pencil-sharpen`. A log without it keeps the pencil it was painted with: 120 mm, Ø 4.5 mm, cut at 0.22 rad and blunted 0.15 mm up (`Stick::pencil_as_before`).

- **[M] The pencil.** 175 mm long: Faber-Castell's Pitt Pastel is listed at 17.5 cm (Jerry's Artarama). The core is 4.4 mm: Stabilo gives the CarbOthello lead as 4.4 mm (stabilo.com), and The Art Gear Guide measured a Pitt core at 4.3 mm.
- **[M, practice] The point's taper, 0.20 rad (11.5°).** A pastel pencil is sharpened with a craft knife or blade, then often touched up with a sharpener or on sandpaper. A plain sharpener tends to break the soft core. Derwent (Martin Aveling) starts the blade a couple of centimetres back and turns the pencil until "a little over a centimetre" of pastel is exposed all round. Emily Rose exposes about half an inch, then finishes on 150/180-grit sandpaper. Over a 4.4 mm core, a bared length L gives τ = atan(2.2 / L): 0.22 at 10 mm, 0.17 at 12.7 mm. "A little over a centimetre", 11 mm, gives 0.197, so **0.20**. The engine's point (`pastel.rs` `Stick::pencil`) is eight cuts at that angle; they reach the full diameter 2.2 / tan 0.20 = 10.85 mm up.
- **[A] The blunt tip.** A freshly cut point crumbles under the lightest touch until its flat bears that touch at the core's indentation hardness. The flat the eight cuts leave is an octagon of apothem a, area 8 tan(π/8) a², so a = √(F / (8 tan(π/8) H)), and the flat sits a / tan τ up the axis. With F = 0.1 N, the bottom of a feather scumble (§5, an estimate), and H = 11.6 MPa (`Stick::mechanics(0.35)`, the pencil's default softness), a = 0.051 mm and the flat is **0.25 mm** up. The engine takes H from each pencil's own softness. The flat stays wider than the core's grains (1–20 µm), so a blade can cut it.
- **[A] What a sharpening costs the core.** No measured loss per sharpening was found. The engine computes it from geometry: the least shift up the axis that puts a fresh point inside the worn core. That is the largest (h − d)/(−n_z) over the wear facets, where h is how far the fresh point reaches along a facet's normal n and d is the facet's support. A knife takes no more core than it must.
- **[A] Which flats the blade leaves: those flatter than the taper.** The blade cuts planes at τ to the axis. A wear facet at φ > τ is blunter than a cut and meets the fresh cone near its tip, so moving the point back clears it. A facet at φ < τ meets the cone far up its side, if at all. Clearing it would take a cut of (h − d)/sin φ, unbounded as φ → 0 and soon longer than the bared cone (≈ 11 mm). So the knife reshapes the point and leaves the flat on the round side (the core laid flat, `p:side`). The threshold is therefore the taper itself, not a separate angle (the PR's first version used 10°).
- **[M + E] Hand time: 16 s.** This is built from MTM-1 predetermined motion times. The metric table used is Németh, *Gépipari szerelés* (1973). 1 TMU = 0.036 s, and an "m-B" move is one whose hand is already moving.

  | step | MTM-1 elements | TMU |
  |---|---|---|
  | pencil to the other hand | G3 | 5.6 |
  | knife taken up and set on the pencil | R30B 12.9 + G1C 8.7 + M30B 13.3 + P1SE 5.6 | 40.5 |
  | the wood shaved round, 8 cuts | each: AP2 10.6 + M3B against 20 N (1.6 + 1.04 × 3.0) + the return m-B 3 cm, 2.25, while the other hand turns the pencil, T30S 2.8 (the longer, 2.8) + P1SE 5.6 = 23.7 | 189.8 |
  | the core's 8 cuts | each: M3B 3.0 + 2.8 + P1SE 5.6 = 11.4 (the soft core needs no pressure) | 91.2 |
  | knife put down | M30B 13.3 + RL1 2.0 | 15.3 |
  | sandpaper taken up | R30B 12.9 + G1A 2.0 + M30B 13.3 + P1SE 5.6 | 33.8 |
  | 5 strokes on it | each: M4B 4.0 + m-B 2.8 | 34.0 |
  | sandpaper put down | M30B 13.3 + RL1 2.0 | 15.3 |
  | pencil back to the drawing hand | G3 5.6 + G2 5.6 | 11.2 |
  | **total** | | **436.7 TMU = 15.7 s** |

  - Estimates in it: the counts (one round of eight wood shavings, since a resharpening moves the wood back only as far as the core, plus eight core cuts to match the engine's point), the 30 cm reaches and the 20 N cutting resistance. The resistance adds 0.5 s in all.
  - Bounds: a full sharpening from bare wood, with two rounds of shavings, takes 626 TMU = 22.6 s. Re-cutting the core alone, with no wood and no sandpaper, takes 164 TMU = 5.9 s.
  - The tables give a trained worker's normal pace, with no allowances.
  - The only timing found otherwise is a beginner's ten minutes per pencil by knife (drawing-pencils.com).
- **[M + E] Shortest pencil held: 75 mm.**
  - A tripod grip pinches the shaft about 20–25 mm up from the tip (occupational-therapy guidance: "about 2 cm", teachhandwriting.co.uk; "about 1 inch", Health Sciences North). A pastel pencil's bared wood starts a couple of centimetres back, so the pinch sits there.
  - The shaft rests on the web of the hand, at the base of the index finger.
  - Index-finger length, tip to the crease at its base, is 62 / 69 / 76 mm for women at the 5th / 50th / 95th percentiles, and 67 / 74 / 82 mm for men (FAA Technical Operations anthropometric survey, *HFDS* appendix B, table 2, item 12).
  - The pinch is at the pad's middle, about 9 mm from the fingertip (an estimate: half the distal phalanx).
  - A pencil still reaching the web of a 5th-percentile woman's hand is therefore 20 + (62 − 9) = 73 mm long, so **75 mm**. For the median woman it is 80 mm, and for the median man 85 mm.
  - Bounds: 45 mm, a stub pinched by the pads alone, up to 95 mm.
  - Informal sources put the point where a pencil is extended or discarded at 65–80 mm overall (Quora). Wikipedia's "Pencil extender" gives no length.
  - The ResearchGate index-finger means quoted in the estimates audit (79.3 / 86.6 mm, a different landmark) could not be reached, so they are not used.

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
  - **[E]** Pastel: below about 0.05–0.2 N only the highest fiber tops are touched, and much of the debris stays loose.
  - **[M]** Hard leads (2H, 4H) depend less on load than Archard predicts.

---

## 3. The deposit

### Morphology
- **[M] Discrete fragments, not a film.** Graphite decorates the fiber network "along ridges and valleys where contact with the lead occurs".
- **[M] Fragment-area distribution.**
  - P(A) ∝ **A^(−3/2) over about 4 decades**, robust across grades and loads.
  - The upper cut-off A_max grows with load and with lead softness.
  - Because the exponent is below 2, covered area is dominated by the largest fragments: ⟨A⟩ ∝ A_max^½.
- **[M] Mechanism.** The distribution reproduces the microcontact islands of a self-affine paper surface.
  - Paper height spectrum: C(q) ∝ q^(−2(1+H)), H ≈ 0.5, fibers 5–15 µm.
  - Islands come from a fully plastic bearing-area cut at A_real = F/σp.
- **Simulation recipe:** generate an H ≈ 0.5 heightfield and take the top pixels until their area equals F/σp. Each connected island is a debris source sized by the island.

### Sizes
- **[M] Measured pastel media** (vibration studies, *Stud. Conserv.* 2018):
  - Black: **1–20 µm particles plus ≈ 60 nm grains**.
  - Pink: needle-shaped particles.
  - Pastel moves as **agglomerates**, and adhesion depends on composition.
- **[E] Constituents:** kaolin 0.2–5 µm, whiting 1–20 µm, organic pigments < 1 µm, earths 1–30 µm.
- **[E] Crumbs** from soft pastel on toothy paper run from a few µm up to about 100–300 µm.

### Placement and gray level
- **[M] Where it lands.** Primary deposit sits at contacts: asperity tops and ridge flanks.
- **[M] Coverage vs contact.** Coverage ≈ **6.6 × A_real/A₀** = 6.6·F/(σp·A₀): debris spreads, fragments and collects in texture.
- **[M] Gray level** is set by **area coverage** (checked against scans at 1.5 mm resolution), not by thickness.

### Repeated strokes and saturation
- **[M] Third body.** On repeat passes the deposit carries load, lubricates and limits wear (noted by Archambault et al., not measured).
- **[M] Pencil electrodes** (*ACS Omega* 2021, loads 50–350 g):
  - Resistance plateaued after **≈ 50 passes**, when "graphite particles filled the depths (valleys) of the rough paper".
  - Roughness was "sufficiently reduced and interfered with frictional sliding". This is the measured form of pastellists' "filled tooth".
- **[D] Model.**
  - Coverage after n passes: c = 1 − (1 − c₁)ⁿ.
  - Let the effective roughness amplitude, and with it K, decay as valleys fill, so deposit per pass tends to zero.
  - **[E]** Pastel saturates in about 3–10 heavy layers on standard paper.

### At the pixel's scale (`--@ crumb-area`)
- **[D] A crumb covers its own area.** A crumb of volume V pressed flat by the face into particles h_p thick (`PARTICLE_UM`, 6 µm) covers A = V/h_p, whatever the pixel. A 250 µm crumb (V ≈ 8.2·10⁶ µm³) covers about 1.4 mm² before the stick's own packing (`Grains::lies`), five pixels at 0.5 mm. The engine had pressed each crumb into the one pixel it fell in. That is right only where a pixel is larger than A. On a 540 mm sheet at 2400 px (0.225 mm a pixel), the largest crumbs, which carry most of the volume (§3, Morphology: area even in volume under A^(−3/2)), covered a fifth to a sixth of their area. The rest of their volume piled into that pixel unseen.
- **[D] Patches fall independently.** Each patch lies as a disc about a point drawn uniformly in the pixel it falls in. Each pixel takes its share of the disc's area as coverage and of its volume as pastel, combined with what is there as c ← c + (1 − c)q. For random placement this gives the expected union at any pixel size. The coarse limit, many small patches in a pixel, is Poisson's 1 − exp(−v/h_p), the face deposit's law.
- **[D] The tooth fills under the pastel, not over the pixel.** The fill line (§3, Repeated strokes) is set by the pastel's depth where it lies: v/c over the covered share c of the pixel, with the rest bare tooth. The material ratio the stick files is min(B(o), 1 − v/µ) there, a concave function of v. Averaging v over the pixel first gave a different answer at each pixel size once the patches were resolved.
- **[D] With the crumb layer** (`--@ pastel-crumb-layer`, pastel_layering.md) each patch lies over the loose pastel as a Kubelka layer at its own depth, volume / q over the share q it covers. It lies over loose pastel on the share l·q of the cell (l is the cell's coverage, less what of it is fixed) and beside it on (1 − l)·q, rather than at the coverage 1 − exp(−v/h_p) of its volume smeared over the pixel. In a light hatched over a loose near-black one (30 mm sheet, both switches), luminance agrees at 0.33 and 0.17 mm a pixel (0.348 and 0.349 over 0.213 and 0.217) within the paired blocks' standard error. The smeared form gives 0.351 here: only 0.003 more, but a pixel wholly under a patch would read as 63 % covered.
- **Check** (tiny scenes with a new painting's switches of 2026-11-16, the veil and fixative-index ones among them, 60 mm of blue wove on a strainer, white soft pastel hatched at 1.2 N every 4 mm, mean linear red over a 36 mm patch). Without the switch: 0.563, 0.475 and 0.404 at 0.667, 0.222 and 0.111 mm a pixel. With it: 0.595, 0.632, 0.633 and 0.631 at 0.667, 0.333, 0.222 and 0.111 mm. Seed-to-seed scatter was ±0.002. The face deposit alone (crumbs off) is 0.302, 0.301 and 0.293 at 0.667, 0.222 and 0.111 mm without any change. The finger and fixative over the same deposit agree to 0.001 at 0.667 and 0.222 mm. The crumbs alone were the cause. At 0.667 mm a pixel is as large as the largest patches, and the patches' landing (the bed's depths, sampled per pixel) is coarser. That width stays a few percent paler.

---

## 4. Contact

### Real contact area
- **[M] Plastic limit (Bowden–Tabor):** A_real ≈ F/σp. This is appropriate here: cellulose fiber walls are far stiffer than pastel.
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
  - At E_z 10–40 MPa, a 100–250 µm sheet compresses about 0.1–5 µm. That is comparable to Ra, so more fiber tops come into contact.
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
| Paper Hurst exponent | H ≈ 0.5; fibers 5–15 µm | M | Archambault |
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
- Pastel pencil sizes: Faber-Castell Pitt Pastel, 17.5 cm, https://www.jerrysartarama.com/faber-castell-pitt-pastel-pencils; Pitt core 4.3 mm, http://www.theartgearguide.com/home/faber-castell-pitt-pastel-pencils; Stabilo CarbOthello lead 4.4 mm, https://www.stabilo.com/us/products/coloring-drawing/chalk-pastel-pencil-stabilo-carbothello/.
- MTM-1 tables (metric): Németh Tibor, *Gépipari szerelés*, Műszaki Könyvkiadó, Budapest, 1973, as tabulated at http://old.bgk.uni-obuda.hu/ggyt/targyak/seged/bagka15nnk/mtm.pdf; worked examples and 1 TMU = 0.036 s, https://industrialeblog.wordpress.com/wp-content/uploads/2016/08/chapter-14.pdf.
- Hand size: FAA *Human Factors Design Standard*, appendix B, "Anthropometric data of Federal Aviation Administration Technical Operations personnel", table 2, https://hf.tc.faa.gov/hfds/download-hfds/hfds_pdfs/App_B_Tech_Ops_Anthropometrics.pdf. Grip height: https://teachhandwriting.co.uk/adaptive-tripod-grip.html; https://hsnsudbury.ca/Portals/1/Services/Newborn_and_Childrens_Care/CTC%20Documents/OT/Pencil%20Grasp.pdf. Stub lengths (informal): https://www.quora.com/How-short-is-too-short-a-pencil-graphite-grey-lead-colour-etc.
- Sharpening pastel pencils: Derwent, "Tips for sharpening pastel pencils with Martin Aveling" (2020), https://blog.derwentart.com/2020/12/04/tips-for-sharpening-pastel-pencils-with-martin-aveling/; Colin Bradley, "How to sharpen pastel pencils", https://www.colinbradleyart.com/blog/how-to-sharpen-pastel-pencils; Emily Rose, "How to sharpen pastel pencils", https://www.emilyrosefineart.co.uk/blog/how-to-sharpen-pastel-pencils; https://drawing-pencils.com/how-to-sharpen-pastel-pencils-a-stepbystep-guide-for-artists/ (anecdotal timing); "Pencil extender", https://en.wikipedia.org/wiki/Pencil_extender.
- Stick dimensions: Jerry's Artarama / Dick Blick / Dakota Pastels listings (Sennelier, Rembrandt, Schmincke, NuPastel); Conté à Paris (https://eu.conteaparis.com/blogs/tutoriels/qu-est-ce-qu-un-carre); Faber-Castell, Stabilo, Caran d'Ache product pages.
