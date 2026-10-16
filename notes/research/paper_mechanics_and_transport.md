# Paper mechanics: through-thickness stiffness and local pore depth

Research notes for paper as a support (crates/paint/src/paper.rs). Compiled 2026-10-06. Builds on `paper_surface.md` (fiber network, exponential pore depths). How an edge dents the sheet, the nap a blade raises and where blown-off crumbs go belong with the tools that need them and are not here.

**Tags:** **[M]** measured, cited. **[D]** derived here, arithmetic or method shown. **[E]** estimate; tunable.

**Bottom line**
- **Pore depth.** In a random layered network the mean surface pore height is **h̄ = t·ε/(1−ε)**. It depends on *local porosity*, not on the local fiber count. Flocs leave the z-structure unchanged. From pixel to pixel, density changes alter h̄ by only about 4–15 %. Sampling a few pores per pixel causes a much larger scatter of the per-pixel mean depth, CV about 0.5–1 under an independence assumption.

---

## 1. Through-thickness stiffness

- **Z-compression [M].** Chen et al. 2020 (copy paper): σ = 0.636(e^(13.54ε) − 1) MPa.
  - Initial tangent modulus E₀ = **8.6 MPa**.
  - Residual strain ε_r ≈ 0.49ε − 0.027.
- **Reversible compression [M].** Kananen, Rajatora & Niskanen 2001: **ε = φ\*(1 − e^(−p/E\*))**.
  - φ\* = 0.13–0.22, the "porosity of compressible pores".
  - E\* = 4–5 MPa (≈4.5 MPa usable for all furnishes).
  - So the initial tangent is E\*/φ\* ≈ 20–35 MPa after pre-cycling.
  - The law is derived from the **height distribution of pore space**.
- **Anisotropy [M].** Ultrasonic measurement of all nine constants (Baum et al.): **E_x/E_z ≈ 190**. Both z-shear moduli exceed E_z.

---

## 2. Local pore depth against local fiber density

### Theory and measurements
- **[M] Dodson 2001.** Take a stack of n Poisson layers with solid fraction p per layer, where 1 − p = e^(−c_layer).
  - Surface pore height r, in fiber thicknesses, has
    **P(r) = (1−p)^r p²(n−r−1) / (−1 + (1−p)^n + np)**
    with mean **r̄ = n − 2 + 2/p − (n−1)np/(−1 + (1−p)^n + np)**.
  - Internal pores are similar.
  - **The SD is approximately proportional to the mean.** For n = 20 and p = 0.3: mean ≈ 1.8t, SD ≈ 2.2t.
  - The mean and SD rise with n and fall with p.
- **[D] Large-n limit.** For n ≫ 1 the mean reduces to **r̄ → (1−p)/p**, a geometric distribution with SD/mean = 1/√(1−p).
  - With p = 1 − ε: **h̄ = t·ε/(1−ε)**, which matches the expression quoted by Sampson's group.
  - At 160–300 g/m², n = c̄/c_layer = 25/0.69 to 47/0.69, i.e. 36–68 layers, so the large-n form holds. Finite-n effects matter only below about 10 layers.
  - **Mean pore height depends on local porosity and fiber thickness, not on local grammage.**
- **[M] Niskanen & Rajatora 2002** (handsheet cross-sections): z-pore heights are **approximately exponential**, g(h) = e^(−h/⟨h⟩)/⟨h⟩. Porosity 0.35–0.5, RBA 0.1–0.2, consistent with a simple porosity–RBA–pore-height theory.
- **[M] Sampson 2005 (JPPS),** citing Dodson, Oba & Sampson and Niskanen & Rajatora: the z-structure of **flocculated** sheets, meaning pore height distribution and in-plane density distribution, is **not significantly different from random networks**. Local grammage and local thickness are strongly correlated (bivariate normal; Dodson, Oba & Sampson 2001).
- **[M] Bloch, Engin & Sampson 2019** (µCT): mean pore height is lower in low-grammage sheets despite their higher porosity, so **surface-layer pore heights are smaller than bulk ones**. Thickness against grammage is linear with a positive intercept, the surface contribution. Numbers are behind a bot wall, so this is a gap.
- **[M] Sampson 2011 (in-plane).** CV of local mean pore *diameter* ≈ **1/√(xτ̄)**, and of local mean pore area ≈ 2/√(xτ̄). Here τ̄ = β/δ is fiber length per area. This is the in-plane analogue of the per-pixel question.

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

## Numbers table

| Quantity | Value | Tag |
|---|---|---|
| E_z initial (Chen) / reversible E\* (Kananen) | 8.6 / 4–5 MPa (φ\* 0.13–0.22) | M |
| E_x/E_z | ≈190 | M |
| Mean surface pore height | t·ε/(1−ε); SD ≈ mean | M/D |
| Pixel CV of coverage, 0.25–0.3 mm | 4–7 % random, ×1.5–3 flocs | D |
| Pixel CV of μ from density | 4–15 % | D |
| Pixel CV of μ from pore sampling | ≤0.5–1.0 | D |

## Gaps
- Surface against bulk pore height numbers (Bloch et al., not readable).

## Sources
- Chen, Spiehl, Dörsam et al. 2020, JPMTR 9(2):65: https://www.jpmtr.org/index.php/journal/article/download/13/10/10
- Kananen, Rajatora & Niskanen 2001, Reversible compression of sheet structure, FRS Oxford: https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/03/2001.2.1043.pdf
- Dodson 2001, On the distribution of pore heights in random layered fibre networks, FRS Oxford: https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/03/2001.2.1037.pdf
- Sampson 2005, contact states, JPPS 31(3):127 (cites flocculation/z-structure results): https://personalpages.manchester.ac.uk/staff/william.sampson/pdf/ContactStatesJPPS.pdf
- Sampson 2011, Spatial variability of void structure in thin stochastic fibrous materials: https://arxiv.org/abs/1108.2259
- Niskanen & Rajatora 2002, Statistical geometry of paper cross-sections, JPPS 28(7):228: https://researchgate.net/publication/288818183
- Bloch, Engin & Sampson 2019, Grammage dependence of paper thickness, Appita 72(1):30 (abstract): https://research.manchester.ac.uk/en/publications/grammage-dependence-of-paper-thickness/
- Baum, Brennan & Habeger 1981, Tappi 64(8):97; Georgia Tech nine-constant report: https://repository.gatech.edu/server/api/core/bitstreams/f9fdd8cb-515b-48d4-83f3-d3bc953bc31e/content
