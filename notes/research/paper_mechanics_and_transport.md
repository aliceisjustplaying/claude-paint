# Paper mechanics and crumb transport: edge dents, local pore depth, nap, blown-off crumbs

Research notes for the pastel simulator. Compiled 2026-10-06. Builds on `paper_surface.md` (fibre network, exponential pore depths), `pastel_removal_air_blade.md` (Winkler knife dents, puff thresholds) and `pastel_brushing.md` (adhesion 10–150 nN per contact).

**Tags:** **[M]** measured, cited. **[D]** derived here, arithmetic or method shown. **[E]** estimate; tunable.

**Bottom line**
- **Edge dents.** If the fibres' through-thickness shear stiffness is included, a knife edge at 0.05–0.5 N/mm sinks **3–32 µm**, against 9–54 µm for Winkler. That is **0.17–0.80× the Winkler depth**: the factor is smallest for sharp edges at light loads and nearest 1 for blunt edges at heavy loads. The dent spreads over a full width of **80–310 µm**, even where the contact itself is 10–200 µm. Permanent dents are 0–9 µm (Winkler gave 0–20 µm). At pen-ball pressures (>10 MPa) spreading stops mattering and Winkler with Chen's curve matches measured handwriting grooves.
- **Pore depth.** In a random layered network the mean surface pore height is **h̄ = t·ε/(1−ε)**. It depends on *local porosity*, not on the local fibre count. Flocs leave the z-structure unchanged. From pixel to pixel, density changes alter h̄ by only about 4–15 %. Sampling a few pores per pixel causes a much larger scatter of the per-pixel mean depth, CV about 0.5–1 under an independence assumption.
- **Nap.** Torn-free fibre ends rise to roughly **20–100 µm**, the instrument class "short fibre rising" (≤0.1 mm). They are too stiff for pastel pressure to flatten, so they add tooth. They should raise Rz much more than Ra. No measured Ra/Rz change after scraping was found.
- **Crumbs.** A puff's footprint is far shorter than the fetch saltation needs to sustain itself (0.1–1.4 m). Bagnold's 0.8 ratio therefore does not strictly apply; only rolling crumbs and single hops occur. Rolling crumbs stop where u* falls below their stopping threshold. That is about **1.15–1.6× the entrainment radius**, so they form an annulus, at about 2–7 cm for a hard blow from 10 cm. Fines ≤5 µm stay airborne (≤10 % redeposit within 10 cm). About 15–25 % of 20 µm fines redeposit within 10 cm.

---

## 1. Indenting paper with an edge, including the fibres' stiffness

### Measured inputs
- **Z-compression [M].** Chen et al. 2020 (copy paper): σ = 0.636(e^(13.54ε) − 1) MPa.
  - Initial tangent modulus E₀ = **8.6 MPa**.
  - Residual strain ε_r ≈ 0.49ε − 0.027.
- **Reversible compression [M].** Kananen, Rajatora & Niskanen 2001: **ε = φ\*(1 − e^(−p/E\*))**.
  - φ\* = 0.13–0.22, the "porosity of compressible pores".
  - E\* = 4–5 MPa (≈4.5 MPa usable for all furnishes).
  - So the initial tangent is E\*/φ\* ≈ 20–35 MPa after pre-cycling.
  - The law is derived from the **height distribution of pore space**.
- **Anisotropy [M].** Ultrasonic measurement of all nine constants (Baum et al.): **E_x/E_z ≈ 190**. Both z-shear moduli exceed E_z.
- **Through-thickness shear modulus [M].** Paperboard plies, Nygårds 2008 as tabulated in creasing papers: **G_xz ≈ 16–127 MPa**, mostly 30–85 MPa. No value for a cotton drawing paper was found; use **15–50 MPa [E]**.
- **In-plane modulus [E/M].** 2–8 GPa, given in the brief.

### Measured dents and contact widths
- **Handwriting grooves [M].** Three pens on 80 g/m² paper, five writers, hard and soft backing:
  - groove depths **13.9–37.2 µm**;
  - detection limit about 2 µm, roughly the paper's roughness;
  - the paper tears when deformation exceeds about 150 µm.
  - Pen force was not measured (JASQDE 3D groove study). Ballpoint writing forces sit in the 0–2 N range (Schomaker & Plamondon; Dooijes) [M, range only].
- **Hard calender nips [M, method].** Steel–steel nips are computed as pure geometry on a Winkler sheet: w = √(R(C₀−C_N)) + √(R(C_R−C_N)), from ingoing, mid-nip and recovered thickness (Peel 1989). This is valid because nip width (mm) is much larger than sheet thickness and the spreading length below.
- **Printing nips.** These are set by the rubber; nominal width is about 5.4 mm for 300 µm board against a 2 mm blanket. They tell us nothing about paper spreading.
- **Gap.** No measured dent depth or width for a knife, blade, creasing rule on a flat anvil, or stylus with *known* load on paper was found. Creasing studies (Nagasawa; Nygårds) use grooved counter-dies.

### Model: an anisotropic layer on a rigid backing [D]
- **Why in-plane displacement drops out.** E_x is about 190× E_z, so u ≈ 0 to first order. The sheet T then obeys
  **E_z ∂²w/∂z² + G_xz ∂²w/∂x² = 0**
  with w = 0 at the backing. This is the Reissner foundation.
- **Pasternak reduction** (w linear in z):
  - k = E_z/T
  - G_p = G_xz·T/3
  - spreading length **ℓ = T√(G_xz/(3E_z))**
- **Spreading length.** T = 250 µm and E_z = 8.6 MPa give **ℓ = 190 µm (G 15), 350 µm (G 50), 560 µm (G 130)**.
- **Closed forms for the linear (small-strain) part:**
  - Flat edge of width b: **δ = q/(k(b + 2ℓ))**. Winkler is the case ℓ = 0, so the ratio to Winkler is **b/(b+2ℓ)**.
  - Narrow line load, exact for this scalar model: **δ ≈ q/(π√(E_z G_xz))·ln(4T√(G_xz/E_z)/(πb))**.
  - Example: q = 0.05 N/mm, G = 15 MPa, b = 11 µm gives 1.4 µm × ln 38 ≈ **5.0 µm**. The nonlinear finite-element result below is 6.0 µm.
- **Nonlinear solution [D].**
  - Method: finite differences, L-BFGS-B energy minimisation with the cylinder as a contact bound.
  - Chen's σ(ε) in z, linear G_xz, T = 250 µm, rigid backing.
  - "Res" is the permanent dent: Chen's residual law integrated over depth using the local strain.
  - Script: rerun from the formulas above; it is not kept in the repo.

| Edge R | q (N/mm) | Winkler δ / res (µm) | G = 15 MPa: δ / res / dent FWHM | G = 50 MPa: δ / res / FWHM | ratio to Winkler |
|---|---|---|---|---|---|
| 20 µm | 0.05 | 21.8 / 4.0 | 6.0 / 0.5 / 80 µm | 3.8 / 0.4 / 102 | 0.17–0.28 |
| 20 µm | 0.2 | 39.5 / 12.6 | 17.3 / 2.8 / 118 | 11.6 / 1.7 / 158 | 0.29–0.44 |
| 20 µm | 0.5 | 53.9 / 19.7 | 31.8 / 8.8 / 137 | 22.6 / 4.6 / 195 | 0.42–0.59 |
| 100 µm | 0.05 | 14.5 / 0.3 | 5.4 / 0.3 / 102 | 3.5 / 0.2 / 127 | 0.24–0.37 |
| 100 µm | 0.2 | 28.5 / 7.2 | 15.6 / 2.1 / 147 | 10.8 / 1.3 / 195 | 0.38–0.55 |
| 100 µm | 0.5 | 41.2 / 13.4 | 28.7 / 7.3 / 170 | 21.1 / 3.9 / 238 | 0.51–0.70 |
| 500 µm | 0.05 | 9.2 / 0 | 4.6 / 0 / 147 | 3.2 / 0.1 / 170 | 0.35–0.50 |
| 500 µm | 0.2 | 19.6 / 2.8 | 13.3 / 1.0 / 208 | 9.7 / 0.8 / 255 | 0.49–0.68 |
| 500 µm | 0.5 | 30.0 / 7.9 | 24.1 / 5.1 / 238 | 18.8 / 2.8 / 310 | 0.63–0.80 |

- **Contact widths** are much narrower than Winkler's: 9–30 µm for R = 20 µm, 23–80 µm for R = 100, and 51–195 µm for R = 500. The dent outside the contact decays roughly as e^(−|x|/ℓ).
- **Peak strain** at the top under the edge is 0.06–0.29.

**Usable fit [D]** (T = 250 µm, R 20–500 µm, q 0.05–0.5 N/mm, ±15 %):
- **δ₀ ≈ 15.6 µm·(q/0.2 N/mm)^0.72·(R/100 µm)^−0.08** for G = 15 MPa.
- **δ₀ ≈ 10.8 µm·(q/0.2)^0.78·(R/100 µm)^−0.06** for G = 50 MPa.
- To scale to another caliper T or E_z, use the closed forms above.

**Sanity check against handwriting [D].**
- Same model, sphere version, with a 0.25–0.5 mm-radius ball at 1–3 N on 100 µm paper.
- Winkler and the shear model agree to within 1 µm: δ = 30–45 µm, residual **12–19 µm**. That is within the low half of the measured 14–37 µm.
- **Reason:** at >10 MPa the exponential stiffening dominates and shear spreading carries little. So Winkler with Chen's curve is fine for pens, nips and hard pressing. The spreading correction matters mainly **below about 2–5 MPa**, which is the knife regime.

**Further corrections, not in the table [D/E].**
- **Surface skin.** A bonded top fibre layer of thickness h_s and modulus E_p acts as a plate. Its bending length is (E_p h_s³/12k)^(1/4) = **47–151 µm** for h_s 10–30 µm and E_p 2–8 GPa. It would make sharp edges (R ≤ 20 µm) dent less still.
- **Below about one fibre width (25 µm) the continuum fails.** An edge then rides on one or two fibres whose free spans (≈50 µm between crossings) bend.
- **The rough top 10–20 µm has no lateral continuity.** It is already handled by the Abbott curve in the simulator.

**Simulator rule.**
- Replace the Winkler depth with δ₀(q, R) from the fit or table.
- Lower the surface by δ₀·e^(−|x−x_edge|/ℓ) outside the contact, not only under it, with ℓ from the table's dent widths: ℓ ≈ (FWHM − contact width)/(2 ln 2) ≈ 50–85 µm. (The linear shear-lag length T√(G/3E_z), 0.2–0.35 mm, spreads dents 2–3× wider than the table: Chen's curve stiffens with strain, so the dent stays narrower.)
- Make the permanent part the "res" column, concentrated within the contact width.
- Keep Winkler with Chen's curve for wide or high-pressure contacts (p > 5 MPa, or contact width > 2ℓ).

---

## 2. Local pore depth against local fibre density

### Theory and measurements
- **[M] Dodson 2001.** Take a stack of n Poisson layers with solid fraction p per layer, where 1 − p = e^(−c_layer).
  - Surface pore height r, in fibre thicknesses, has
    **P(r) = (1−p)^r p²(n−r−1) / (−1 + (1−p)^n + np)**
    with mean **r̄ = n − 2 + 2/p − (n−1)np/(−1 + (1−p)^n + np)**.
  - Internal pores are similar.
  - **The SD is approximately proportional to the mean.** For n = 20 and p = 0.3: mean ≈ 1.8t, SD ≈ 2.2t.
  - The mean and SD rise with n and fall with p.
- **[D] Large-n limit.** For n ≫ 1 the mean reduces to **r̄ → (1−p)/p**, a geometric distribution with SD/mean = 1/√(1−p).
  - With p = 1 − ε: **h̄ = t·ε/(1−ε)**, which matches the expression quoted by Sampson's group.
  - At 160–300 g/m², n = c̄/c_layer = 25/0.69 to 47/0.69, i.e. 36–68 layers, so the large-n form holds. Finite-n effects matter only below about 10 layers.
  - **Mean pore height depends on local porosity and fibre thickness, not on local grammage.**
- **[M] Niskanen & Rajatora 2002** (handsheet cross-sections): z-pore heights are **approximately exponential**, g(h) = e^(−h/⟨h⟩)/⟨h⟩. Porosity 0.35–0.5, RBA 0.1–0.2, consistent with a simple porosity–RBA–pore-height theory.
- **[M] Sampson 2005 (JPPS),** citing Dodson, Oba & Sampson and Niskanen & Rajatora: the z-structure of **flocculated** sheets, meaning pore height distribution and in-plane density distribution, is **not significantly different from random networks**. Local grammage and local thickness are strongly correlated (bivariate normal; Dodson, Oba & Sampson 2001).
- **[M] Bloch, Engin & Sampson 2019** (µCT): mean pore height is lower in low-grammage sheets despite their higher porosity, so **surface-layer pore heights are smaller than bulk ones**. Thickness against grammage is linear with a positive intercept, the surface contribution. Numbers are behind a bot wall, so this is a gap.
- **[M] Sampson 2011 (in-plane).** CV of local mean pore *diameter* ≈ **1/√(xτ̄)**, and of local mean pore area ≈ 2/√(xτ̄). Here τ̄ = β/δ is fibre length per area. This is the in-plane analogue of the per-pixel question.

### Per-pixel recipe and size of variation [D]
**1. Local coverage CV.** Sampson's line-process variance gives CV(c) ≈ 1/√(τ̄x) for zone side x.

| Sheet (δ 0.12–0.20 mg/m) | τ̄ (mm⁻¹) | CV(c), x = 0.25 mm | x = 0.3 mm |
|---|---|---|---|
| 160 g/m² | 800–1330 | 5.5–7.1 % | 5.0–6.5 % |
| 300 g/m² | 1500–2500 | 4.0–5.2 % | 3.7–4.7 % |

Formation adds a factor of 1.5–3 [E] (paper_surface §2). That gives **6–21 %**.

**2. Local porosity from pressing.** With T ∝ c^press, the local solid fraction is s = s̄(c/c̄)^(1−press).

**3. Local mean depth.** **μ_pix = t·ε_loc/(1−ε_loc)**, with ε_loc = 1 − s.
- Sensitivity: dln μ/dln c = −(1−press)/ε. Denser spots have **shallower** pores.
- With press = 0.65 and ε = 0.5 the factor is 0.7, so **CV(μ) ≈ 0.7·CV(c) ≈ 4–15 %**.
- With no pressing densification (press = 1), μ is uniform.

**4. Pore-sampling scatter.** A pixel holds about n = x²τ_L²/π surface pore polygons, with τ_L = ln(1/ε)/ω.
- Polygon areas have CV ≈ 2 (Miles), so the area-weighted mean depth has **CV_pix ≈ CV_h·√((1 + 3.93)/n)**, where CV_h = 1/√ε.

| ε, ω | n (x = 0.25 mm) | CV_pix | n (x = 0.3) | CV_pix |
|---|---|---|---|---|
| 0.4, 20–30 µm | 19–42 | 0.54–0.81 | 27–60 | 0.45–0.68 |
| 0.5, 20–30 µm | 11–24 | 0.64–0.96 | 15–34 | 0.54–0.80 |
| 0.6, 20–30 µm | 6–13 | 0.80–1.19 | 8–19 | 0.66–0.99 |

- This assumes independent pores, so it is an **upper bound**. The surface depth field is an overlay of several layers' polygons, which means more, smaller cells.
- It dominates the density effect.
- **Suggested use:** μ_pix = μ_loc × Gamma(k, 1/k), with k = n/4.93 ≈ 2–12, i.e. CV 0.3–0.7. Calibrate on a scan.
- The per-pixel Abbott curve is then 1 − e^(−d/μ_pix).
- Surface pores are smaller than bulk (Bloch et al.), so take t·ε/(1−ε) using the *surface* porosity only as an upper value.

---

## 3. Raised nap and cutting

### What is measured
- **[M] Fibre-rising instruments** (FRT Fibro, Rycobel) classify lifted fibres into two groups:
  - **long**: bonded at one end, standing >0.1 mm above the surface;
  - **short**: bonded along most of their length, ≤0.1 mm.

  Lifted-fibre counts come from a CCD silhouette as the sheet bends over a thin roll.
- **[M] Raised-fibre counts in tissue** (US 12,371,858; chemically raised, 15 g/m² crepe): **122–1272 fibres ≥0.1 mm tall per 100 mm** of edge profile, i.e. 1.2–12.7 per mm. This is only an analogue for density.
- **[M] Erasers** (Pearlstein et al. 1982, JAIC 22:1). Whatman No. 1, 87 g/m², 0.16 mm; 10 strokes.
  - Pink Pearl and kneaded rubber **lifted surface fibres from the plane**.
  - Magic Rub gave slight abrasion at the edges.
  - Opaline gave no visible change.
  - No loss of tensile or fold strength.
  - **No profilometry or mass loss** was reported.
- **[M]** Cleaning studies used contact profilometry on 19th-c. paper. Brush-applied cellulose ethers changed roughness most, rigid gels least. Ra values are not accessible.
- **Gap:** no Ra/Rz before and after scraping, rubbing or Taber was found for uncoated paper. Same for pastel uptake against nap.

### Derived picture [D]
- **Surface fibre ends.**
  - Fibres per area: N = β/(δλ) = 0.16/(0.16×10⁻³ × 2) = **500 mm⁻²**, giving 1000 ends per mm² in the sheet.
  - The top layer holds a share c_layer/c̄ ≈ 0.69/25 ≈ 3 %, i.e. about **30 surface ends per mm²**.
  - Free span to the first crossing ≈ πω/(2c_layer) ≈ **55 µm**. Each extra torn bond frees about one more span.
- **Raised height.** A blade dragging at above 1–6.5 mN per fibre tears ends and spans loose. Their lifted length is 1–3 spans, so **50–150 µm**. At 20–60° they stand **20–100 µm** proud. That is the "short fibre rising" class. Loops arise where two bonds of a mid-span go.
- **Do they stay up?**
  - Fibre as a ribbon: E_L ≈ 30 GPa, 25 × 6 µm, so EI ≈ 1.4×10⁻¹¹ N·m².
  - Force to push a free end of length L down by δ: F = 3EIδ/L³.
    - L = 60 µm, δ = 30 µm: **≈ 6 mN**.
    - L = 200 µm, δ = 100 µm: **≈ 0.5 mN**.
  - A pastel stick at about 0.1 MPa nominal puts about 0.15 mN on a 25 × 60 µm end. **Short raised ends survive pastel strokes as rigid teeth; long ones get laid down.**
- **Roughness.** About 1–30 raised ends per mm², each 25 µm wide and 50–150 µm long, cover only **≈0.1–10 %** of the area. **Ra rises little** (<1 µm). **Rz/Rp rise by tens of µm** where nap is present.
- **Tooth [E].** Assume raised ends trap pastel over about 10 % of the area around them (a few fibre widths per end). The added holding volume is then ≈ 0.1·h_nap ≈ 2–10 µm of equivalent pore depth, against μ ≈ 6 µm, i.e. **≈ 1.3–2.5× local tooth** where nap stands. This is consistent with practitioners restoring tooth with fine sandpaper. The 10 % is an assumption and is the number to calibrate.
- **Scalpel (R ≈ 1 µm).**
  - Cutting starts when contact pressure on a fibre exceeds wall hardness (70–420 MPa): q_cut ≈ H·b, with b ≈ max(2R, contact width) ≈ 2–5 µm. That is **≈ 0.15–2 N/mm**, consistent with `pastel_removal_air_blade.md`.
  - Material reached per pass is bounded by the edge's dent δ₀. From §1, R → small, so **≈ 4–30 µm at 0.05–0.5 N/mm**.
  - Only what lies above the track is shaved. That is at most about one fibre layer, t_f/(1−ε) ≈ **10–15 µm**, per pass above q_cut, i.e. ≈ 5–8 g/m² of fibre [E].
  - Below q_cut the edge compresses and raises nap without cutting.
  - **Gap:** no measured cut depth against load for a blade dragged over paper.

---

## 4. Where blown-off crumbs go

### Thresholds and hysteresis
- **[M] Impact over fluid threshold.**
  - Field measurements: u\*_it/u\*_ft = **0.813 ± 0.018, 0.863 ± 0.027 and 0.837 ± 0.007** at three sites (Martin & Kok 2018).
  - Theory gives 0.82 (Kok 2010b); Bagnold gave about 0.8.
  - This ratio holds for **sustained saltation**, where splash replaces lost grains.
- **[D] A puff cannot sustain saltation.**
  - The saturation length is L_sat ≈ 2.2(ρ_p/ρ_a)d (Andreotti; form [M], numbers [D]).

    | Crumb | L_sat |
    |---|---|
    | 50–300 µm at 1250 kg/m³ | 0.11–0.69 m |
    | 50–300 µm at 2500 kg/m³ | 0.23–1.4 m |

  - The footprint is a few cm, so only **rolling (creep), reptation and isolated hops** occur.
  - **[E]** Treat the stopping threshold of a moving crumb as **0.6–0.85 × u\*_ft**. The bond is broken, and rolling resistance is about 10⁻²–10⁻³ of sliding (brushing note).
- **[M] Hop scales** (Kok et al. 2012).
  - 250 µm sand in steady Earth saltation: hop length **L ≈ 0.1 m**; surface grain speed **≈ 1 m/s, constant with u\***.
  - Grains above about 500 µm only reptate, with hops **<1 cm**.
  - Modes by size: suspension below 20 µm, short-term suspension 20–70 µm, saltation 70–500 µm.

### Radial decay of the wall jet
- **[M] Inputs.** Wall-jet maximum velocity decays as **r^(−1.1)**. Jet thickness grows linearly, δ_r ≈ 0.08–0.1 r. Skin friction c_f = 0.06(u_m δ_m/ν)^(−0.3)(r/H)^(−0.16) (Poreh, Tsuei & Cermak 1967; spread table in Colebrook-wall-jet paper).
- **[D] Hence τ_w ∝ r^(−2.3) and u\* ∝ r^(−1.15)** beyond the stagnation zone.
- **[E]** Apply it from the Phares peak at r ≈ 0.09H outward. This slightly overstates u\* between 0.09H and about 0.2H.

**Ring of redeposited crumbs [D].**
- A crumb is entrained where u\*(r) > u\*_ft. It rolls outward and stops where u\*(r) = u\*_stop, so
  **r_stop/r_ft = (u\*_ft/u\*_stop)^(1/1.15) = 1.15 (ratio 0.85), 1.21 (0.8), 1.36 (0.7), 1.56 (0.6)**.
- **Worked example:** hard blow, 21 m/s at H = 100 mm. u\*_max = 1.1 m/s at r ≈ 9 mm.

  | Crumb | u\*_ft | Cleared to | Annulus |
  |---|---|---|---|
  | 100 µm, F_adh 150 nN, r/a 5–2 | 0.56 m/s | r ≈ 16 mm | ≈ 19 mm (ratio 0.8) |
  | 100 µm, F_adh 10 nN | 0.14 m/s | r ≈ 54 mm | ≈ 65 mm |

  With adhesion spread over a decade, **a broad annulus of rolled crumbs at about 2–7 cm**, densest at its inner edge, rings a cleared centre.
- **Hopping crumbs [D].**
  - Settling speeds: v_s(100 µm, 1250 kg/m³) = 0.30 m/s; v_s(300 µm) = 1.4 m/s.
  - Lifted into a wall jet 1–5 mm thick moving at 3–15 m/s, they fly for about 1–15 ms and travel **≈ 0.3–15 cm** per hop.
  - So **hoppers land outside the ring or off the sheet**. **[E]** Assume 10–30 % of moved crumbs hop.
- **Vertical easel.** Gravity acts along the sheet. Rolled crumbs that stop are then unstable above d_c ≈ 0.1–0.3 mm (tapping note) and fall off.

### Fines (1–20 µm)

| d (ρ 2500) | v_s | τ_p |
|---|---|---|
| 1 µm | 0.075 mm/s | 7.7 µs |
| 5 µm | 1.9 mm/s | 0.19 ms |
| 10 µm | 7.5 mm/s | 0.76 ms |
| 20 µm | 30 mm/s | 3.0 ms |

- **They stay airborne [D].** Turbulent velocities in a wall jet are about 0.1–0.2 u_m ≈ 0.3–3 m/s, ≫ v_s.
- **[M fit] Deposition to a smooth wall** (Wood 1981): v_d⁺ = 0.057 Sc^(−2/3) + 4.5×10⁻⁴ τ⁺², capped at about 0.13, with τ⁺ = τ_p u\*²/ν, plus v_s on a horizontal sheet.
- **[D] Fraction redeposited** along a 10 cm path in a 5 mm layer, with U_m ≈ 15u\* [E]:

  | d | u\* = 0.3 m/s | u\* = 1 m/s |
  |---|---|---|
  | 1–2 µm | ~0 % | 0.3 % |
  | 5 µm | 1 % | 9 % |
  | 10 µm | 4.5 % | 17 % |
  | 20 µm | 26 % | 19 % |

  These are upper-side estimates, because u\* decays outward.
- **The rest leaves as a cloud.** In still air 10 µm settles 0.3 m in about 40 s; 1–2 µm effectively does not settle. How much of the cloud returns to a horizontal sheet scales with sheet area over cloud footprint (gap).

**Simulator rule.**
1. Remove crumbs where u\* > u\*_ft.
2. Deposit 70–90 % of them [E] in an annulus r_ft → r_stop.
3. Throw 10–30 % [E] 0.3–15 cm outward.
4. Fines: deposit size-dependent 0–25 % within 10 cm downstream, weighted toward 10–20 µm. Drop the rest.

---

## Numbers table

| Quantity | Value | Tag |
|---|---|---|
| E_z initial (Chen) / reversible E\* (Kananen) | 8.6 / 4–5 MPa (φ\* 0.13–0.22) | M |
| E_x/E_z | ≈190 | M |
| G_xz paperboard plies | 16–127 MPa | M |
| Spreading length ℓ = T√(G/3E_z), T 250 µm | 190–560 µm | D |
| Knife dent δ₀, q 0.05–0.5 N/mm, R 20–500 µm | 3–32 µm (Winkler 9–54) | D |
| Dent FWHM | 80–310 µm | D |
| Permanent dent | 0–9 µm | D |
| Handwriting groove depth, 80 g/m² | 13.9–37.2 µm | M |
| Mean surface pore height | t·ε/(1−ε); SD ≈ mean | M/D |
| Pixel CV of coverage, 0.25–0.3 mm | 4–7 % random, ×1.5–3 flocs | D |
| Pixel CV of μ from density | 4–15 % | D |
| Pixel CV of μ from pore sampling | ≤0.5–1.0 | D |
| Fibre-rising classes | long >0.1 mm, short ≤0.1 mm | M |
| Raised-end height after scraping | 20–100 µm | D/E |
| Force to flatten 60 / 200 µm end | ≈6 / 0.5 mN | D |
| u\*_it/u\*_ft, field | 0.81–0.86 | M |
| Saturation length, crumbs | 0.1–1.4 m | D |
| u\* decay in wall jet | ∝ r^−1.15 | D (from M) |
| Ring radius / entrainment radius | 1.15–1.56 | D/E |
| 250 µm saltation hop / surface speed | 0.1 m / 1 m/s | M |
| Fines redeposited within 10 cm | ≤2 µm ~0; 5 µm 1–9 %; 10 µm 5–17 %; 20 µm 19–26 % | D |

## Gaps
- Measured dent depth and width under a known line load on drawing paper (knife, rule on a flat anvil, stylus).
- G_xz of cotton or drawing papers.
- Surface against bulk pore height numbers (Bloch et al., not readable).
- Ra/Rz after scraping or abrasion.
- Nap density against blade load.
- Pastel uptake against nap.
- Blade cut depth against load.
- Measured redeposition patterns around an air puff on a surface.
- Return fraction of the dust cloud.
- Cessation threshold for single rolling cohesive crumbs.

**Bench checks.**
1. Press a knife edge at known load (kitchen scale) into Mi-Teintes over glass, then read dent depth and width with a dial gauge or phone photometric stereo. This calibrates G_xz.
2. Puff over a uniform crumb layer on a black card and photograph the ring.

## Sources
- Chen, Spiehl, Dörsam et al. 2020, JPMTR 9(2):65: https://www.jpmtr.org/index.php/journal/article/download/13/10/10
- Kananen, Rajatora & Niskanen 2001, Reversible compression of sheet structure, FRS Oxford: https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/03/2001.2.1043.pdf
- Dodson 2001, On the distribution of pore heights in random layered fibre networks, FRS Oxford: https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/03/2001.2.1037.pdf
- Sampson 2005, contact states, JPPS 31(3):127 (cites flocculation/z-structure results): https://personalpages.manchester.ac.uk/staff/william.sampson/pdf/ContactStatesJPPS.pdf
- Sampson 2011, Spatial variability of void structure in thin stochastic fibrous materials: https://arxiv.org/abs/1108.2259
- Niskanen & Rajatora 2002, Statistical geometry of paper cross-sections, JPPS 28(7):228: https://researchgate.net/publication/288818183
- Bloch, Engin & Sampson 2019, Grammage dependence of paper thickness, Appita 72(1):30 (abstract): https://research.manchester.ac.uk/en/publications/grammage-dependence-of-paper-thickness/
- Baum, Brennan & Habeger 1981, Tappi 64(8):97; Georgia Tech nine-constant report: https://repository.gatech.edu/server/api/core/bitstreams/f9fdd8cb-515b-48d4-83f3-d3bc953bc31e/content
- Nygårds G_xz ply data, via creasing paper D: https://www.diva-portal.org/smash/get/diva2:641101/FULLTEXT01.pdf
- Peel 1989, calendering review (nip width formulas): https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/02/1989.2.979.pdf
- Printing nip dynamics (5.4 mm nominal nip): https://link.springer.com/article/10.1007/s41783-020-00091-z
- 3D depth measurement of handwriting grooves, JASQDE: https://journal.asqde.org/articles/10.69525/jasqde.242
- Schomaker & Plamondon, pen force: https://www.ai.rug.nl/~lambert/papers/pen-pressure.pdf
- Pearlstein et al. 1982, Effects of eraser treatment on paper, JAIC 22(1): https://cool.culturalheritage.org/jaic/articles/jaic22-01-001.html
- Fibre-rising tester classes: https://www.rycobel.com/products/fibre-rising-testers
- US 12,371,858 (raised fibre counts): https://patents.google.com/patent/US12371858B2/en
- Kok, Parteli, Michaels & Karam 2012, Rep. Prog. Phys. 75:106901: https://sseh.uchicago.edu/doc/Kok_et_al_2012.pdf
- Martin & Kok 2018, distinct fluid and impact thresholds: https://arxiv.org/abs/1610.10059
- Poreh, Tsuei & Cermak 1967 and wall-jet spread table, via: https://www.osti.gov/servlets/purl/2587227
- Sharma et al. 2022, impinging-jet erosion (Phares τ_max): https://arxiv.org/abs/2206.01839
- Wood 1981, J. Aerosol Sci. 12:275 (deposition-velocity fit; standard form, not re-fetched)
- Andreotti 2004, J. Fluid Mech. 510:47 (saturation length form; not re-fetched)
