# Dry pigment optics: pastel vs oil, from refractive-index contrast

Purpose: give the Kubelka–Munk (KM) engine a physical rule for how a pigment looks dry in a
pastel stick (pigment + chalk/kaolin + a little gum, pores full of air) versus bound in
linseed oil, and relate surface micro-roughness to gloss. Short version: **absorption K is
roughly the same; scattering S rises with the index contrast |n_p − n_m|, by a factor of
about 1.5 for very high-index pigments and by 10 to 1000 times for pigments whose index
matches oil.** On top of that, a powder surface adds a few percent of flat, diffuse
"white" surface reflection. "Paler" falls out of these two effects.

Confidence flags: **[H]** handbook value, **[M]** literature value with spread or from a
secondary source, **[C]** my own calculation (Mie, script `dry_pigment_mie.py` in this folder),
**[E]** estimate or extrapolation with no direct measurement found.

---

## 1. Refractive indices

Values are for visible light (~589 nm unless noted). Where a mineral is birefringent, the
principal indices are given and the spread matters, because a "matched" mean can still
scatter through its off-axis index (calcite is the main case). Main sources: Feller (ed.),
*Artists' Pigments* vol. 1; Roy vol. 2; FitzHugh vol. 3; Berrie vol. 4; Eastaugh et al.,
*Pigment Compendium*; Gettens & Stout, *Painting Materials*; webexhibits "Pigments through
the Ages" technical pages, which quote those handbooks.

### Pigments

| Pigment | Composition | n (principal / range) | Mean used | Notes |
|---|---|---|---|---|
| Lead white | basic Pb carbonate (hydrocerussite ± cerussite) | hydrocerussite ω 2.09, ε 1.94; cerussite 1.80/2.08/2.08 | 2.0 | Strongly birefringent (Δ≈0.15–0.27) [H] |
| Zinc white | ZnO | ω 2.013, ε 2.029 | 2.02 | [H] |
| Cobalt blue | CoAl₂O₄ spinel | 1.74 | 1.74 | Isotropic [H] |
| Synthetic ultramarine | Na-Al silicate + S₃⁻ | 1.50 | 1.50 | Isotropic (cubic). Almost exactly matches fresh oil [H] |
| Cerulean blue | Co stannate | ~1.84 | 1.84 | Webexhibits gives "n/a"; 1.84 is from Gettens & Stout [M] |
| Prussian blue | Fe₄[Fe(CN)₆]₃ | 1.56 | 1.56 | Isotropic. Very small particles (0.01–0.2 µm), very high K [H] |
| Emerald green | Cu acetoarsenite | α 1.71, β 1.77–1.78 | 1.75 | Birefringent [H] |
| Viridian | Cr₂O₃·2H₂O | α=β 1.62, γ 2.12 | ~1.8 | Highly birefringent, so it never fully disappears in oil [H] |
| Chrome yellow | PbCrO₄ (crocoite) | 2.29 / 2.36 / 2.66 | 2.4 | Strongly birefringent [H] |
| Orange chrome | PbCrO₄·PbO | 2.42–2.7 | 2.55 | [M] |
| Barium yellow | BaCrO₄ | 1.94–1.98 | 1.96 | [M] |
| Strontium yellow | SrCrO₄ | ~1.92–2.01 | 1.97 | [M] |
| Zinc yellow | K₂O·4ZnCrO₄·3H₂O | ~1.84–1.90 | 1.87 | [M] |
| Cadmium yellow | CdS / (Cd,Zn)S | ω 2.506, ε 2.529 | 2.5 | Index rises toward the absorption edge (anomalous dispersion) [H] |
| Naples yellow | Pb₂Sb₂O₇ | 2.01–2.28 | 2.15 | [H] |
| Indian yellow | Mg/Ca euxanthate | ">1.52", weakly birefringent; ~1.67 often quoted | ~1.6–1.67 | Uncertain [M] |
| Yellow lake | dye on alumina hydrate / chalk | substrate 1.50–1.60 | 1.55 | Optics dominated by the substrate. Transparent in oil [M] |
| Yellow ochre | goethite α-FeOOH + clay/quartz | goethite 2.26 / 2.39 / 2.40–2.52; whole ochre lower | ~2.0–2.3 | Clay and quartz (1.55) dilute it [M] |
| Red ochre / red earth | hematite α-Fe₂O₃ + clay | hematite ω ~3.15–3.22, ε ~2.87–2.94 | ~2.9 (pure) | Strongly absorbing below ~580 nm [H] |
| Raw sienna | goethite-rich earth | 1.87–2.17 | 2.0 | Gettens & Stout [M] |
| Burnt sienna | hematite-rich (calcined) | ~1.85–2.78 | ~2.3 | Wide range [M] |
| Vermilion | HgS | ω 2.905, ε 3.256 | 3.0 | Highest index on the list [H] |
| Red lead | Pb₃O₄ | 2.42 | 2.42 | Slightly birefringent [H] |
| Rose madder lake | alizarin/purpurin on Al hydrate | madder lake 1.66 (alizarin 1.70); hydrate substrate ~1.5–1.6 | ~1.6 | Transparent in oil [H/M] |
| Carmine lake | carminic acid on Al/Ca | ~1.6 | 1.6 | [H] |
| Cobalt violet | Co₃(PO₄)₂ or Co₃(AsO₄)₂ | phosphate ~1.65–1.70; erythrite 1.63/1.66/1.70 | 1.68 | [M] |
| Bone black | ~10–20 % C on hydroxyapatite | apatite matrix 1.63–1.65 | 1.65 + carbon | Absorption comes from the carbon. The matrix scatters in air [M] |
| Vine black | amorphous carbon | complex index ≈ 1.8–2.0 + i(0.5–0.8) | — | Absorption-dominated. Treat with complex n [M] |

### Fillers, binders, media

| Material | n | Notes |
|---|---|---|
| Calcite / chalk / whiting | ω 1.658, ε 1.486 (Δ 0.172) | Mean ~1.6. Huge birefringence, so some scattering remains in oil [H] |
| Kaolin (kaolinite) | 1.553 / 1.559 / 1.565 | [H] |
| Gypsum | 1.520 / 1.523 / 1.530 | Almost a perfect match to oil and gum [H] |
| Barytes / blanc fixe (if used) | 1.64 | [H] |
| Gum arabic, dry film | ~1.51–1.54 | Solutions are lower (water 1.333) [M/E] |
| Gum tragacanth, dry | ~1.5 | No good measurement found [E] |
| Linseed oil, fresh liquid | 1.478–1.48 | [H] |
| Linseed oil film, aged | > 1.525 after ~10 years; up to ~1.57–1.58 after centuries | This is why paintings grow more transparent and pentimenti show through (Laurie; de la Rie) [M] |
| Water | 1.333 | Used for "damp" |
| Air | 1.0003 | |

---

## 2. Scattering vs index contrast

### 2.1 Formulas

Scattering by a particle depends only on the **relative index m = n_p/n_m** and the
**size parameter x = π d n_m / λ₀**, where λ₀ is the wavelength in vacuum. The shorter
wavelength inside the medium shifts x up by a factor of n_m. Three levels of approximation
are in use:

1. **Fresnel / interface picture** (used in coatings textbooks and Natural Pigments-style
   explanations). The scattering power per interface goes as
   R = ((n_p − n_m)/(n_p + n_m))². This is a heuristic, not a scattering coefficient.
2. **Rayleigh / Lorenz–Lorentz** (d ≪ λ). The scattering cross-section goes as
   ((m² − 1)/(m² + 2))² · x⁴. For small m − 1, the prefactor goes as (m − 1)².
3. **Mie theory** (d ~ λ, the pigment regime of 0.1–5 µm). Exact for spheres. In
   coatings practice it is combined with a transport correction: the useful
   back-scattering is σ_s(1 − g), where g is the asymmetry parameter. Two-flux KM is then
   taken as S_KM ≈ (3/4)·σ_s(1 − g) and K_KM ≈ 2·σ_a, per unit volume (Mudgett & Richards
   1971; Ross 1971, "Theoretical light-scattering power of TiO₂ and microvoids"). The
   conversion constants cancel in an air/oil ratio. In the Rayleigh–Gans regime (|m − 1|
   small, which applies to near-matched pigments in oil) σ_s ∝ (m − 1)², so S vanishes
   quadratically as n_p → n_m.

Both simple factors (1 and 2) give almost the same air/oil ratios [C]:

| Pigment (n) | Fresnel ratio air/oil₁.₄₈ | Lorenz–Lorentz ratio |
|---|---|---|
| Vermilion (3.0) | 2.2 | 2.0 |
| Chrome yellow (2.4) | 3.0 | 3.0 |
| Lead white (2.0) | 5.0 | 5.4 |
| Cobalt blue (1.74) | 11 | 13 |
| Chalk, mean (1.59) | 40 | 48 |
| Prussian blue (1.56) | 69 | 82 |
| Gypsum (1.525) | 190 | 230 |
| Ultramarine (1.50) | ~900 | ~1070 |

### 2.2 Mie results (size-averaged, transport-corrected)

Setup: lognormal size distribution (σ_ln 0.5), λ₀ = 550 nm, non-absorbing spheres. The
table gives the transport-scattering per unit particle volume in air (µm⁻¹) and the ratio
S_air / S_oil [C]:

| Pigment (n) | d = 0.3 µm: S_air, ratio vs oil 1.48 / 1.57 | d = 1 µm | d = 3 µm |
|---|---|---|---|
| Vermilion / hematite (3.0) | 5.3, ×1.3 / 1.3 | 1.03, ×1.4 / 1.4 | 0.28, ×1.4 / 1.5 |
| Cadmium yellow (2.45) | 5.7, ×1.6 / 1.9 | 0.95, ×1.5 / 1.7 | 0.25, ×1.7 / 1.8 |
| Chrome yellow (2.4) | 5.5, ×1.7 / 2.0 | 1.04, ×1.8 / 1.9 | 0.24, ×1.6 / 1.8 |
| Goethite ochre (2.3) | 5.4, ×1.8 / 2.3 | 0.89, ×1.6 / 1.7 | 0.24, ×1.8 / 2.0 |
| Naples yellow (2.15) | 5.5, ×2.4 / 3.3 | 0.91, ×1.7 / 1.9 | 0.23, ×1.9 / 2.2 |
| Lead / zinc white (2.0) | 5.2, ×3.5 / 5.5 | 0.88, ×1.9 / 2.4 | 0.21, ×2.0 / 2.5 |
| Cerulean (1.84) | 4.5, ×6.4 / 13 | 0.82, ×2.5 / 4.4 | 0.20, ×2.5 / 3.2 |
| Cobalt blue (1.74) | 3.9, ×11 / 28 | 0.75, ×3.9 / 10 | 0.18, ×2.8 / 5.2 |
| Cobalt violet (1.70) | 3.7, ×14 / 47 | 0.80, ×5.8 / 19 | 0.17, ×3.2 / 8.4 |
| Bone-black matrix (1.65) | 3.4, ×23 / 118 | 0.73, ×9 / 49 | 0.17, ×4.6 / 23 |
| Calcite, birefringence-averaged | — | 0.73 → 0.059 (oil 1.48), 0.017 (oil 1.57): **×12 / ×43** | — |
| Kaolin, birefringence-averaged | — | 0.72 → 0.016: **×45** | — |
| Gypsum, birefringence-averaged | — | 0.68 → 0.005: **×140** | — |
| Prussian blue (1.56) | 2.7, ×87 | 0.71, ×43 | 0.16, ×20 |
| Ultramarine (1.50) | 2.2, ×1200 | 0.68, ×700 | 0.15, ×330 |

Points to take from the table:

- **In air every pigment is a strong scatterer.** At a fixed size, S_air per unit volume
  falls within a factor of ~2 across all pigments, from ultramarine to vermilion, because
  m = n_p is always ≥ 1.5. Dry, particle size and packing matter more than the pigment's
  identity.
- **The air/oil ratio is the oil-side number that varies:** ~1.3–2 for n_p ≥ 2.3, ~2–5 for
  the whites, ~3–15 for the cobalt blues and greens, and 10²–10³ for ultramarine,
  Prussian blue, lakes, gypsum and kaolin.
- **Do not apply the extreme ratios literally.** Measured "transparent" pigments still
  have finite S in oil. The residual comes from birefringence, size tails, crystal
  inclusions, pigment–oil voids, agglomerates and the film's own inhomogeneity. Isolated
  matched spheres scatter almost nothing; real paint does not. **Recommendation:** for
  n_p ≲ 1.65, compute S_dry in absolute terms (Mie or the table above × pigment volume
  fraction) instead of multiplying the measured S_oil by the ratio. For n_p ≳ 1.9,
  S_dry = r · S_oil with r from the table is safe.
- **Ageing oil** (1.48 → 1.57) moves things in the opposite direction from drying out.
  Old lead white and chalk grounds lose hiding.

### 2.3 Dense packing (dependent scattering)

The ratios above assume independent scattering. A pastel layer or a powder is ~40–60 %
solids by volume, and dense packing reduces S per particle, most strongly for small,
high-contrast particles (the TiO₂ "crowding" effect). In air the gaps are air, so contrast
is preserved and a dry powder bed still scatters hard: BaSO₄ and MgO powders are R∞ ≈
0.97–0.99 reflectance standards. **Practical correction [E]:** multiply the
independent-scattering S_air by a packing factor of ~0.5–0.7 for sub-micron pigments and
~0.8–1 for particles ≥ 2 µm. I found no published per-pigment value.

### 2.4 Does K change?

To first order, no. In KM practice K is treated as a property of the pigment
concentration. Mie gives refinements [C]:

- **High-index absorbers** (n 2.4–3.0, k 0.01–0.5): absorption per volume in air vs oil
  is ×0.93–1.16. Unchanged within the noise.
- **Low-index, weakly absorbing particles** (n 1.5 + 0.001–0.05i, d ≈ 1 µm): an isolated
  particle in air absorbs ×1.5–2 more than in oil, because light is trapped by internal
  reflection inside a high-contrast particle. In a dense powder this is partly offset by
  multiple scattering. I would cap it at ×1.0–1.3, or ignore it.
- **Carbon** (vine black, n ≈ 1.9 + 0.6i): absorption is unchanged (A_air/A_oil ≈ 1.05–1.1),
  but S rises ~2.4× in air. A dry carbon black is therefore a little grayer, never
  "pale".

Also note that the **effective K/S of the mixture changes even with K fixed**, because
the white filler's S rises enormously in air. Most of the "pastel look" comes from this.

### 2.5 Worked examples

**Ultramarine** [C, illustrative K/S]. Take masstone-in-oil K/S ≈ 0.5 at 450 nm
(R∞ ≈ 0.38) and ≈ 20 at 600 nm (R∞ ≈ 0.024), which gives a deep blue. Dry, with a
realistic effective S multiplier of ~20 after the residual-scattering cap: K/S = 0.025 at
450 nm gives R∞ ≈ 0.80, and K/S = 1.0 at 600 nm gives R∞ ≈ 0.27. The result is a pale,
bright, chalky blue, not a dark one. This matches the well-known appearance of
ultramarine as a dry powder (light, intensely blue) versus in oil (near-black in masstone,
transparent glaze).

**Why wet or oiled surfaces darken** (Lekner & Dorf 1988, *Appl. Opt.* 27:1278; Twomey,
Bohren & Mergenthaler 1986, *Appl. Opt.* 25:431):

1. A liquid in the pores lowers the contrast, so S falls and g rises (scattering becomes
   more forward). Each photon then travels a longer path between back-scattering events
   and is more likely to be absorbed. In KM terms K/S goes up, so R∞ goes down and chroma
   goes up.
2. The liquid film's top surface traps light by total internal reflection (internal
   diffuse reflectance r_i ≈ 0.47 for water, 0.58–0.60 for oil). The trapped light
   returns to the absorbing layer.

Both effects act together, which is why oiling out or varnishing a matte passage restores
depth.

**Chalk/kaolin filler in a pastel** [C/E]. Chalk at 1 µm has S_air ≈ 0.73 µm⁻¹ per unit
volume, against 0.06 in fresh oil and 0.017 in aged oil. A typical pastel is ~30–80 %
filler by solids for tints and ~0–20 % for "pure" pastels (manufacturer formulations vary;
no single source). The filler dominates the dry S, so dry pastel colors are tints of
their oil masstones even with no lead white.

### 2.6 Recommended engine recipe

For each pigment i with oil data K_i(λ), S_i^oil(λ), and particle index n_i and median
size d_i:

```
K_i^dry(λ)  = K_i(λ)                      # optional ×1.0–1.3 for low-n weak absorbers
S_i^dry(λ)  = n_i ≥ 1.9 ? r(n_i,d_i)·S_i^oil(λ)
                       : φ_pack · σ'_air(n_i,d_i,λ)·c_i   # absolute Mie estimate
filler (chalk/kaolin/gypsum): K small (slightly yellow), S^dry from Mie (~0.7 µm⁻¹·vol frac at 1 µm),
                              S^oil ≈ S^dry/12 (chalk) … /45 (kaolin) … /140 (gypsum)
gum: ~1–5 % by volume, n≈1.52 — treat as part of the medium, ignore
mixture: two-constant KM, K_mix = Σ c_i K_i^dry, S_mix = Σ c_i S_i^dry
```

The Mie script computes r and σ' for any (n, d, λ). Run it per wavelength if the
dispersion matters (e.g. cadmium yellow or vermilion near their absorption edges).

---

## 3. Kubelka–Munk practice

**Masstone inversion** (opaque layer): K/S = (1 − R∞)² / (2R∞). Inverse:
R∞ = 1 + K/S − √((K/S)² + 2K/S). Use the **internal** reflectance R_i, i.e. after removing
the surface reflection (see below), or K/S will be wrong for dark colors.

**Two-constant mixing:** K_mix = Σ c_i K_i and S_mix = Σ c_i S_i (concentrations by
volume or mass, used consistently). Separating K from S requires a second measurement,
usually a tint with white or a layer over black, because R∞ alone gives only the ratio.
The single-constant shortcut (fixed S) breaks down when a filler's S changes by ×10–100
between media. A dry/oil model therefore must be two-constant.

**Saunderson correction:**
R_m = k1 + (1 − k1)(1 − k2) R_i / (1 − k2 R_i)

Values computed from Fresnel equations, integrated over hemispherical diffuse light [C]:

| Interface | k1 normal (specular) | k1 diffuse illumination | k2 (internal, diffuse) |
|---|---|---|---|
| water, n 1.33 | 0.020 | 0.066 | 0.47 |
| fresh oil, n 1.48 | 0.037 | 0.089 | 0.58 |
| n 1.50 (textbook) | 0.040 | 0.092 | **0.60** |
| aged oil, n 1.57 | 0.049 | 0.102 | 0.64 |

Practice:

- **Smooth oil film:** k1 = 0.04 if the specular gloss is excluded from viewing (k1 = 0 if
  the gloss is excluded from the measurement entirely), k2 ≈ 0.4–0.6. The literature
  spread comes from the k2 fitted for real, imperfectly diffuse internal light; Mudgett &
  Richards give a polynomial in n. Effect: dark colors get darker and saturate (R_i 0.05
  → R_m 0.021 with k1 = 0, k2 = 0.6).
- **Matte paint in air** (lean oil, gouache): the interface still exists but is rough, so
  k1 ≈ 0.04–0.09 becomes a flat **diffuse** veil added to the color. Keep k2 ≈ 0.6 for
  the binder-rich skin, or lower it where the surface is porous.
- **Powder or pastel in air:** there is no continuous interface. Diffuse-reflectance
  practice (Kortüm, *Reflectance Spectroscopy*) applies KM directly to R∞ with k2 ≈ 0. The
  first-surface Fresnel reflections from particle facets (~4–20 % per facet for n
  1.5–3.0) are already part of the volume scattering. At the top grains they act as an
  extra spectrally flat diffuse term. **Engine suggestion [E]:** k2 = 0, plus an additive
  flat k1 ≈ 0.02–0.04. This caps the darkest dry blacks at R ≈ 0.03–0.05. Example: R_i
  0.02 gives R_m ≈ 0.05 for powder vs 0.008 for an oil film, about L* 27 against about
  7. That gap is why a black pastel never matches a black oil glaze.

---

## 4. Gloss vs surface micro-roughness

**Davies (1954) / Bennett & Porteus (1961), the scalar Beckmann limit:**
R_spec / R_0 = exp[−(4π σ cos θ / λ)²], valid for RMS height σ ≪ λ. At 550 nm [C]:

| σ (µm) | 20° | 60° | 85° |
|---|---|---|---|
| 0.02 | 0.83 | 0.95 | 1.00 |
| 0.05 | 0.32 | 0.72 | 0.99 |
| 0.10 | 0.01 | 0.27 | 0.96 |
| 0.15 | 0 | 0.05 | 0.92 |
| 0.27 | 0 | 0 | 0.75 |
| 0.50 | 0 | 0 | 0.37 |
| 1.0 | 0 | 0 | 0.02 |

Every surface regains specular sheen at grazing angles (85°), which is why matte paintings
glint when viewed raking. Once σ ≳ 0.2 µm the coherent specular term is gone, and the
**angular width of the glossy lobe is set by RMS facet slope**, not σ (Beckmann /
Torrance–Sparrow microfacet models; Simonot & Elias use slope for paint/varnish
interfaces). A renderer should use σ only to decide whether there is a mirror-like
component, and use a microfacet roughness (slope) parameter for everything rougher.

Measured or typical values:

| Surface | RMS roughness | Source / confidence |
|---|---|---|
| Fresh varnish or glossy medium-rich oil | < 0.1 µm (varnish forms a new interface with RMS < 0.1 µm over rough paint) | Berns & de la Rie 2003 [M] |
| Semi-gloss pigmented coating, 60° gloss 48 | Sq 0.15 µm | NIST PVDF study, LSCM 150× [H] |
| Same coating weathered, 60° gloss 21 | Sq 0.27 µm | NIST [H] |
| Matte or lean oil film, PVC near/above CPVC | ~0.3–2 µm micro-roughness from protruding particles | Coatings literature, no painting measurement found [E] |
| Gouache / distemper (above CPVC, porous) | ~1–5 µm, particle and agglomerate scale | [E] |
| Pastel layer | grains and agglomerates 1–20 µm sitting in paper tooth tens of µm deep; effectively a porous powder, not a surface | [E] |

At the micro scale, gloss is mainly a matter of **microroughness from pigment particles at
the surface** (Rq correlates with gloss better than Ra or Rz). Lowering the oil content
pushes PVC past the CPVC, so the binder no longer covers the particles. The surface
roughens to particle scale and the film becomes porous (air voids). The second effect
raises S by the mechanism in §2, so a lean film is both **matte and lighter**, and
oiling out reverses both. I found **no published profilometry of "peinture à l'essence"**
(blotted oil). The nearest data are matte/flat coatings above CPVC and the
Berns–de la Rie varnish studies (confocal + stylus profilometry). Expect flat coating
values: 60° gloss < 10 and 85° gloss of a few tens.

---

## 5. Measured dry vs bound appearance

- **Cosentino 2014**, "FORS spectral database of historical pigments in different
  binders", *e-conservation Journal* 2:57–68. Covers 54 historical pigments as pure
  powder (per the abstract) and in gum arabic, egg tempera, linseed oil and fresco, with
  free CSV downloads (e-conservation.org / chsopensource.org). This is the dataset to fit
  r(λ) per pigment, by inverting powder and oil R∞ to K/S and checking that K is
  conserved. I could not download the CSVs from this machine (sites 403 or no link found),
  so it is worth a retry by hand.
- **Pictorial materials database** (Springer, *Appl. Phys. A* 2017, 1200
  pigment/binder/varnish combinations). Not reachable from here.
- Conservation-wiki "Inpainting: Pigments" and handprint.com state the qualitative rule:
  large index difference gives more scatter, a lighter and matter look; small difference
  gives darker, higher chroma. Leanly bound pigments are lighter in value than fully bound
  ones.
- Berns & de la Rie, "The effect of the refractive index of a varnish on the appearance of
  oil paintings" (*Stud. Conserv.* 2003), and "The relative importance of surface
  roughness and refractive index…". Measured that saturation and darkness gains on
  varnishing come mostly from smoothing the roughness and partly from the varnish
  index (1.47–1.54).
- Lekner & Dorf 1988 and Twomey et al. 1986 give measured and modeled wet/dry reflectance
  ratios for sand and soils: wet reflectance is ~0.5–0.8 of dry across the visible.

---

## 7. Earths and viridian dry: measured powders (`--@ dry-earths`)

(§6, the dye lakes, comes with `--@ dry-lakes`.)

**The problem [M, the engine's output].** A painter's pastel came out high-key: pure
burnt sienna laid as a light salmon, lighter than the tan sheet it was on, and only
vine-black mixes gave darks. Full-strength sticks laid heavily (three crossing passes at
3 N) on the tan wove preset and on white drawing paper, 2026-11-18 integration build,
L\*a\*b\* (D65) of each swatch's centre, unfixed → fixed:

| Stick | the stick's own color (sRGB) | on tan wove (sheet 57.4/4.7/17.3) | on white (sheet 89.4/−0.1/9.3) |
|---|---|---|---|
| burnt sienna | 198 132 111 | 61.1/21.5/21.3 → 60.3/21.9/21.4 | 66.7/17.7/18.1 → 66.0/18.0/18.1 |
| red earth | 211 129 108 | 61.8/27.2/24.1 → 61.0/27.7/24.3 | 67.0/23.1/20.5 → 66.3/23.6/20.6 |
| raw sienna | 214 164 111 | 69.7/11.4/33.1 → 69.1/11.8/33.3 | 73.9/9.7/28.8 → 73.2/10.1/29.0 |
| raw umber (every box) | 190 151 118 | 64.4/9.6/22.4 → 63.6/9.9/22.6 | — |
| ultramarine | 83 125 228 | 54.6/16.1/−51.1 → 54.1/16.6/−51.6 | 60.9/12.9/−44.5 → 60.4/13.3/−44.9 |
| viridian | 86 173 163 | 64.8/−25.5/−1.3 → 64.3/−25.6/−1.2 | 69.5/−22.3/−0.5 → 69.0/−22.2/−0.4 |
| Prussian blue | 0 60 119 | 31.5/6.7/−28.5 → 31.1/6.7/−28.0 | 44.8/3.3/−18.5 → 44.6/3.2/−18.0 |
| vine black | 51 54 56 | 29.2/0.9/2.9 → 28.8/1.0/2.9 | 42.8/−0.3/3.0 → 42.6/−0.2/3.1 |
| bone black | 0 0 0 | 18.8/2.2/8.1 → 18.8/2.2/8.2 | 38.6/−0.1/4.8 → 38.6/−0.1/4.8 |

On the tan sheet a heavy earth swatch reads its stick's own color to within 1–2 L\*. The
laying (crumbs, coverage, K–M mixing, the veil, fixative) is not what pales it. Each of
the stack's pastel switches was turned off in turn (`pastel-km-mix`, `fixative-index`,
`wet-pastel`, `pastel-crumb-layer`, `crumb-area`, `dry-lakes`, `oil-k`,
`relief-prefilter`, `seat-solver`, `pastel-current-heap`): burnt sienna stays at L\* 61
on tan. Without `pastel-veil` it goes lighter (63.4), as it should. The stick is the
cause: `dry::spectral_color` of the earth tubes.

**Diagnosis [C].** The rule keeps a tube's oil K and raises its S into air (n ≥ 1.9: the
Mie air/oil ratio × its oil S). That needs K per volume, and these tables don't identify
it. Burnt sienna, red earth, raw umber and the others are researched tubes: K comes from a
Kremer gum-arabic colour-chart swatch (a single thin layer over printed grey fields)
read as an assumed 100 µm oil film, and S from an estimated hiding (0.45 for burnt
sienna; `scripts/pigment_data/researched-tubes.json`). A thin swatch read as an opaque
film gives a K/S far too small: burnt sienna's dry K/S is 0.34 at 600 nm, where its
powder's is 12. It is the lakes' fault again (§6).

### 7.1 Measured powders and paints

Cosentino 2014 FORS powders (Kremer pigments; `CSV/powder.csv`, SHA-256 86bb05f0…ae45),
corrected from the instrument exactly as in §6.1 (black floor 0.04, now the measured 0.022: §9.1, rutile titanium white
0.945 at 550–700 nm, lead white's shape), with L\* over the bounds (black 0.03–0.05 ×
titanium white 0.90–0.99) [M/C]:

| Powder (Kremer) | sRGB | L\*a\*b\* | L\* range |
|---|---|---|---|
| burnt sienna (40430) | 115 62 50 | 32.6/21.7/17.6 | 30.0–35.0 |
| red ochre | 143 85 72 | 42.4/22.8/17.8 | 40.3–44.4 |
| raw sienna | 180 126 70 | 57.3/15.1/38.6 | 55.5–59.0 |
| yellow ochre | 211 158 90 | 68.8/12.3/42.9 | 67.1–70.5 |
| raw umber | 104 76 57 | 34.8/9.4/16.1 | 32.3–37.1 |
| burnt umber (no tube) | 86 58 43 | 27.3/10.3/14.4 | 24.2–30.1 |
| viridian | 0 126 114 | 46.3/−36.9/−3.0 | 44.3–48.2 |
| ultramarine | 106 129 177 | 54.0/4.3/−27.9 | 52.1–55.7 |
| Prussian blue | 41 44 84 | 19.7/11.6/−25.2 | 14.7–23.6 |
| vine black | 61 62 61 | 26.2/−0.6/0.6 | 22.9–29.0 |
| bone black | 52 52 52 | 21.7/0.0/0.3 | 17.4–25.1 |

**Manufacturer cross-check [M].** Golden Artist Colors publish CIE L\*a\*b\* for each
colour (method not stated on the page): Fluid Acrylic Burnt Sienna (PBr7, calcined
natural iron oxide) 33.96/17.64/14.17, Raw Umber (PBr7) 28.2/1.45/3.33. These are bound
paints, so they bracket the powders from the dark side. Burnt sienna dry is about as deep
as in a binder, as a high-index pigment should be (its air/oil S ratio is only 1.6, §2.2).

**Pastel sticks.** No measured L\*a\*b\* or reflectance of full-strength soft pastel
sticks was found: no manufacturer data, and no conservation paper with stick
colorimetry. The pastel-fixative studies (Kindl & Brown 1990, the ICR/ISCR fixing
experiments) report ΔE, not stick colours. A pure-pigment stick is the powder with a few
per cent of gum (n 1.5, mostly in the necks between grains), so the powder is the
nearest measured target. It is also what §6 fits.

### 7.2 The rule (switch `--@ dry-earths`)

For burnt sienna, red earth, raw sienna, yellow ochre, raw umber and viridian, as the
catalog has them:
- **S dry** is the Mie scattering of their particles in air at their index, on lead
  white's dry scale (`dry::rule_scatter`: the rule for any pigment under n 1.9, here at
  the table's 1 µm), not the oil table's estimated-hiding S.
- **K dry** is the oil table's K times one factor (`dry::powder_k`), fitted so the pure
  dry colour matches the corrected powder (ΔE76, `dry_earth_fit.py`).

The oil tables stay as they are: what the switch moves is the dry colour of a pastel
stick and of a sheet toned with those tubes. Paper presets keep theirs (their tone parts
are read from a fresh catalog box without the switch).

| Tube | n | dry before (sRGB, L\*a\*b\*) | ΔE | factor | dry after | ΔE | factor over the bounds |
|---|---|---|---|---|---|---|---|
| burnt sienna | 2.3 | 199 132 112 (61.5/23.2/21.6) | 29.2 | **31.71** | 115 65 55 (33.2/20.6/15.3) | 2.6 | 27.8–36.1 |
| red earth | 2.7 | 211 129 108 (62.3/29.4/24.7) | 22.1 | **7.718** | 152 77 66 (41.7/30.4/20.5) | 8.1 | 6.4–9.2 |
| raw sienna | 2.0 | 214 164 111 (71.0/12.0/34.5) | 14.7 | **10.08** | 172 127 78 (56.5/11.7/32.9) | 6.7 | 9.0–11.3 |
| yellow ochre | 2.1 | 254 211 144 (86.9/6.2/38.8) | 19.5 | **2.087** | 243 183 102 (78.3/13.1/48.7) | 11.2 | 1.8–2.4 |
| raw umber | 2.1 | 190 151 118 (65.2/10.1/22.9) | 31.1 | **16.49** | 102 78 56 (35.0/6.9/16.7) | 2.6 | 14.3–19.1 |
| viridian | 1.8 | 87 174 163 (65.6/−28.7/−2.9) | 20.9 | **3.910** | 12 125 114 (47.0/−31.3/−1.9) | 5.7 | 3.5–4.4 |

What is left is the oil tables' K *shape*, which one factor can't fix: red earth's a\* is
too high (30 against 23), yellow ochre's b\* too high (49 against 43) and its L\* still
9 too light. Keeping the rule's old S and fitting a factor on K alone (as dry-lakes)
would give burnt sienna ×9.8 (ΔE 5.9), red earth ×4.9 (16.5), raw sienna ×3.0 (1.6),
yellow ochre ×2.4 (14.7), raw umber ×9.8 (6.9). That is no better overall, and it keeps
an S that rests on an estimated hiding, so the Mie S stands. At 550 nm the S per coat,
Mie against the old: burnt sienna 4.8 against 0.76, red earth 5.3 against 2.1, raw
sienna 4.8 against 1.0, yellow ochre 4.9 against 5.5, raw umber 4.9 against 1.7, and
viridian 4.3, unchanged (it was already under n 1.9).

Tints, 1:1 by volume with lead white, before → after (not measured): burnt sienna
230 188 180 → 143 86 75; red earth 228 163 151 → 176 99 87; raw umber 213 186 166 →
129 101 77; viridian 123 195 187 → 55 153 142. The painting's sticks: "hair" (burnt
sienna 2, red earth 1, lead white 0.3) 210 140 122 → 128 71 61; "brown" (vine black 1,
burnt sienna 1) 148 127 119 → 119 70 59.

### 7.3 The check against hematite's absorption [C]

Per volume of pigment K = 2μ_a and S = ¾σ′ (§2.1). On lead white's absolute scale, burnt
sienna's S at 1 µm is ¾ × 0.89 µm⁻¹, so the fitted K at 550 nm is **4.0 µm⁻¹**
(500/600/650 nm: 13.0, 2.3, 1.2). Pure hematite, k 0.0925 at 550 nm (Longtin et al. 1988,
as tabled by Zhang et al., *ACP* 15:12159, 2015, table 2), has μ_a = 4πk/λ = 2.1 µm⁻¹, so
K = 4.2 µm⁻¹. Querry's 1985 hematite k runs about twice as high, and is held to be too
high (Zhang et al.; Meland et al. 2011). So at 1 µm grains the fit makes burnt sienna
absorb as a bed of nearly pure hematite. Burnt sienna is hematite in clay. Its grains
are larger than 1 µm, and σ′ at 3 µm is a quarter of its 1 µm value (table 2.2), so the
same K/S needs proportionally less K. The 1 µm S is the upper end, and the K it implies is
the most iron oxide could give. The paint test `dry_burnt_sienna_absorbs_no_more_than_hematite`
holds it there. At 650 nm the fitted K (1.2) is more than hematite's (k 0.0097 at 633 nm
gives 0.39). Burnt sienna's red is absorbed more than hematite's alone, perhaps by
manganese oxide or by the powder's own lot, which is not resolved here.

**Estimates left:**
- the grain size (1 µm, the table's), which sets S per volume and so the tints with
  white, not the pure sticks (replaced by measured S per volume: §9);
- no measured tint of an earth with white, dry, was found to pin S;
- the K shapes of red earth, yellow ochre and raw sienna (the oil tables');
- CHSOS doesn't say which lots it measured. Red earth is matched to CHSOS's "red ochre";
  the researched table's source is Kremer 40020, a washed French red ochre.

**Not changed, for a later PR.**
- Brown ochre, the Mars colours, Indian red, bone brown and bitumen have no powder in
  the set.
- Bone black's dry stick is R∞ ≈ 10⁻⁴ (sRGB 0 0 0), where its powder is L\* 21.7: too
  dark. It is `carbon_ratio` × an oil K/S of near zero, and the facet reflection that
  charcoal's beds have (`CarbonBed`) is missing. (Done in §8.1.)
- Ultramarine dry is L\* 53 (powder 54) but over-saturated (b\* −52 against −28).
  (The powder is natural ultramarine, the tube synthetic: §8.3.)
- The earths' oil masstones rest on the same thin-swatch K/S. They may be as pale in
  oil, but this switch doesn't touch oil. (They are: §8.6, `--@ oil-earths`.)

Sources for this section: Cosentino 2014 (as §6); Golden Artist Colors, Fluid Acrylic
Burnt Sienna and Raw Umber product pages (goldenartistcolors.com, CIE L\*a\*b\* listed);
X. L. Zhang et al., "What is the real role of iron oxides in the optical properties of
dust aerosols?", *Atmos. Chem. Phys.* 15 (2015) 12159–12177, table 2 (hematite n, k at
550 and 633 nm); Longtin, Shettle, Hummel & Pryce 1988 (via Zhang); Querry 1985, *Optical
Constants*, US Army contractor report; Meland et al., *JQSRT* 112 (2011), hematite and
goethite scattering at 470, 550, 660 nm; Kremer Pigmente 40430 Dark Burnt Sienna
(PR101/102, calcined Italian earth). Computation: `dry_earth_fit.py` (this folder).

---

## 8. The blacks dry, ultramarine, the earths' shapes, and the earths in oil

Computation: `dry_colours_fit.py` (this folder), with the same inputs as §7. Its output
is quoted below. "Powder" means CHSOS's FORS powder (Kremer pigments, Cosentino 2014),
corrected from the instrument as in §6.1 and §7.1 unless stated otherwise.

### 8.1 The blacks: opaque grains, so the facets set the bed (in `--@ dry-earths`)

**Cause [C].** The dry rule gives a black S = `carbon_ratio` × its oil S. Bone black's
calibrated oil S is near nothing (K/S in oil ~10⁵), so its stick came out at R∞ 10⁻⁴,
sRGB 0 0 0. Vine black has no oil table and falls back to its masstone's K/S at an
estimated hiding, so its stick came out at R∞ 0.20 (L\* 52), too *light*. Neither
number has a physical basis. Carbon absorbs within about 0.07 µm (λ/4πk, k 0.6), so a
carbon-bearing grain of a micrometre or more is opaque. All it scatters is what its facets
reflect, and that reflection is set by its complex index, not by an oil S. This is
charcoal's physics (charcoal.md §2, `charcoal_optics.py`), the same Mie code as §2.

**The rule.** Dry, a black keeps its K (so its tints keep their strength) and takes
S = K × the S/K its grains have in air (Mie, size-averaged, KM S = ¾σs(1−g), K = 2σa;
`dry::carbon_bed`):

| Black | Grain | m at 550 nm | S/K | R∞ (L\*) | Sensitivity |
|---|---|---|---|---|---|
| vine black | charred wood, the mean of Jäger, Mutschke & Henning's 1998 cellulose chars at 400 and 600 °C (charcoal.md §2); 3 µm splinters [E: charcoal.md] | 1.35 + 0.63i | 0.0441 | 0.0211 (16.1) | 1 / 10 µm: 0.0220 / 0.0217; fired at 400 / 600 °C: 0.018 / 0.029 |
| bone black | hydroxyapatite (n 1.64) with 10 wt % carbon [M: CAMEO, after Gettens & Stout; 10–20 % from makers] spread through the grain, Bruggeman; carbon density 1.8 [E: a coke, 1.5–2.0]; 7 µm grains [M: Rublev's bone black, 3–15 µm] | 1.60 + 0.11i | 0.0276 | 0.0134 (11.6) | 3 / 15 µm: 0.0137 / 0.0135; 20 wt % carbon: 0.0133; char at 400 / 600 °C: 0.014 / 0.013 |

R∞ hardly depends on the grain size, the char or the carbon content. The facets decide it.
No other tube changes. Bone brown (a partly charred bone, brown) is not a black and keeps
its rule. The catalog has no lamp, ivory or Mars black.

### 8.2 The measurements, and the powders' black floor

CHSOS can't say how dark a black powder is. Its sphere reads all four carbon blacks at
0.156–0.173 (a floor of stray light), and the correction (§6.1) *assumes* their mean is
0.04 (0.03–0.05). That number came from §3's veil estimate (a flat 2–4 % over a powder),
which #329/#321 later showed double-counts the facets. What CHSOS does measure are the
*differences* between the blacks (true units, gain 0.788): vine +0.0076, lamp +0.0020,
ivory −0.0040, bone −0.0056 against their mean.

**An absolute measurement [M].** Lethuillier et al. (2022, *MNRAS*,
doi:10.1093/mnras/stac1734, arXiv:2110.05832) measured sieved juniper charcoal powder for
cometary-dust analogues: albedo 3.00 % at 550 nm (their table 4; MoHIS hyperspectral
imager, normal illumination; Hapke single-scattering albedo 5.6 %). If vine black reflects
as that charcoal does, CHSOS's black floor is **0.022**, not 0.04. Then the corrected
powders are vine black 20.3/−0.8/0.8, bone black 13.6/0.0/0.5 and burnt sienna
28.5/25.6/23.7 (32.6/21.7/17.6 at 0.04).

| Black | engine dry, before | with the switch (physics) | powder, floor 0.04 (L\* bounds) | ΔE76 | powder, floor 0.022 | ΔE76 |
|---|---|---|---|---|---|---|
| vine black | 124 124 124 (52.0/0.2/0.0) | 40 40 40 (16.1/0/0) | 26.2/−0.6/0.6 (22.9–29.0) | 10.1 | 20.3/−0.8/0.8 | 4.4 |
| bone black | 0 0 0 (0.0/0.1/0.0) | 31 31 31 (11.6/0/0) | 21.7/0.0/0.3 (17.4–25.1) | 10.1 | 13.6/0.0/0.5 | 2.1 |

The physics, the charcoal measurement and CHSOS's measured differences agree with each
other. They don't agree with the 0.04 floor (taken since: §9.1). **Open:** the floor sets every corrected
powder, the earths' and the lakes' (§6, §7) as well as the blacks'. At 0.022 the dark
earths' targets are about 4 L\* deeper (burnt sienna 28.5), so `powder_k` and
`dye_loading` would be refitted. That is left to the lead (it moves #318's merged
fits too). The blacks' rule has no fitted number in it, so it doesn't depend on the floor.

On a sheet, with `--@ pastel-veil` (#329) no veil goes over a pastel bed, so the stick's
R∞ is what's seen. Without it, the canvas's 4 % first-surface veil
(`canvas::SURFACE_REFLECTANCE`) puts a full bone black patch at Y 0.04 whatever its R∞.

### 8.3 Ultramarine: the powder is natural, the tube synthetic

The over-saturation §7 noted (engine b\* −57 against the powder's −28) compares two
different pigments. CHSOS's "ultramarine" is **natural ultramarine, Kremer 10510**
(Cosentino 2015, *Int. J. Conserv. Sci.* 6:287, table 1, which lists the database's 54
pigments with their Kremer numbers). Natural ultramarine is lazurite ground from lapis
lazuli with its gangue (calcite, diopside, pyrite), which greys it. The engine's
"ultramarine blue" is synthetic ultramarine. Their K/S shapes differ as the two pigments
should (relative to 450 nm):

| | 450 | 500 | 550 | 600 | 650 | 700 nm |
|---|---|---|---|---|---|---|
| natural, powder (CHSOS) | 1 | 1.9 | 4.3 | 5.4 | 4.5 | 2.3 |
| natural, gum swatch (CHSOS 45/45 probe, a separate measurement) | 1 | 2.0 | 4.6 | 5.4 | 4.8 | 3.2 |
| synthetic, engine K (calibrated on oil tints) | 1 | 7.7 | 62 | 50 | 44 | 16.5 |

The powder's and the probe-measured swatch's shapes agree, so the shape is the pigment's,
not the powder's scattering or a missing surface term. A first-surface term would also
double-count (§8.2). So the synthetic tube's dry colour is not tested by this powder, and
it is **not changed**. Pourliaka et al. (PoS(BPU11)142) measured seven Kremer
synthetic ultramarine powders with a BaSO₄ sphere. Their spectra are given only as a
figure, so no number from them is used here. The engine's "ultramarine ash" (natural, a
pale last extraction) dries to 206 218 233 (86.6/−0.9/−8.9). It is meant to be paler than
a medium grade like 10510, and no ash powder was measured.

### 8.4 The other blues, the cobalts and the cadmiums: checked, not changed

The engine's dry colour without any switch against the powders (floor 0.04), ΔE76:

| Tube | engine dry | powder (Kremer) | ΔE |
|---|---|---|---|
| Prussian blue | 0 61 120 (25.3/5.9/−39.4) | 41 44 84 (19.7/11.6/−25.2), 45202 | 16.4 |
| cobalt blue | 30 133 230 (54.9/7.3/−57.0) | 41 123 186 (49.8/−2.5/−39.8), 45730 | 20.5 |
| smalt | 179 204 236 (81.1/−1.5/−18.5) | 73 75 155 (35.7/22.3/−44.2), 10000 | 57.3 |
| cobalt violet | 193 186 230 (77.3/11.3/−21.2) | 125 75 158 (40.8/37.0/−37.4), 45800 | 47.5 |
| cadmium yellow | 255 229 85 (91.9/−1.1/72.2) | 239 229 67 (89.2/−14.0/75.0), 21010 | 13.5 |
| cadmium red | 200 93 108 (53.0/44.0/12.1) | 175 41 55 (39.6/53.9/25.2), 21120 | 21.3 |
| vermilion | 214 81 66 (52.5/51.6/36.0) | 180 91 90 (49.4/36.1/17.4), 10610 | 24.4 |
| Naples yellow | 255 228 140 (91.2/−1.4/46.3) | 219 179 93 (75.0/4.4/48.9), 10130 | 17.4 |
| red lead | 255 118 27 (66.6/51.2/69.4) | 235 99 54 (58.9/50.2/50.3), 42500 | 20.6 |

What the numbers show, and why nothing here changes:
- **Smalt and cobalt violet are coarse.** The rule gives every pigment under n 1.9 the
  Mie S of 1 µm grains. Smalt's at 20 µm brings it to ΔE 5.1, and cobalt violet's at
  10 µm to 8.4 (with the oil table's K). But Kremer's smalt 10000 is sold as 0–120 µm,
  and SEM measured 60–120 µm (Studies in Conservation 2021, doi:10.1080/00393630.2021.1940721).
  At those sizes Mie with the grain's absorption (the package effect) makes it far too dark
  (L\* 13–18 against 35.7), so the oil table's K isn't the Kremer smalt's. No size for
  cobalt violet 45800 was found. Neither is changed. A size would be a number chosen to
  fit.
- **Cobalt blue's** error is K's shape (its red end, R 0.85 against 0.36 at 700 nm). No
  grain size helps (3 µm: ΔE 34).
- **Cadmium red, Prussian blue and burnt sienna's blue end** all absorb so strongly that
  their grains are opaque in those bands. The powders there sit at the blacks' level
  (0.025–0.045 at floor 0.04), which is facet reflection, as in §8.1. The rule's K/S comes
  from the oil tables and gives cadmium red 0.13 at 500 nm (powder 0.04) and Prussian blue
  0.005 at 700 nm (powder 0.025). The general fix is the dry K and S from Mie of each
  grain's complex index (its absorption from its K per volume) at its grain size. §8.5
  tries it on the earths. Most tubes have no measured grain size, so it is left for a
  later change.

### 8.5 The earths' shape errors: the causes tried

With §7's switch, red earth's a\* is 30.4 (powder 22.8) and yellow ochre is 9.5 L\* light.
The causes tried, each with no new free number:
- **A first-surface term** (a flat 4 % over the bed): red earth ΔE 8.1 → 5.5, yellow
  ochre 11.2 → 10.7, burnt sienna 2.6 → 12.0. It also double-counts the facets (§8.2,
  #329). Rejected.
- **Grain size per pigment** moves S at every wavelength alike, so the one factor takes it
  up and the shape doesn't change.
- **Mie with the grain's absorption** (complex index from K per volume, 1 µm, the factor
  refitted): red earth 8.1 → 4.3 (a\* 26.7), raw sienna 6.7 → 5.5, yellow ochre
  11.2 → 10.7, viridian 5.7 → 5.5, but burnt sienna 2.6 → 4.9 and raw umber 2.6 → 3.0.
  Its facet floor for n 2.3–2.7 grains (R 0.09–0.13 in the blue) is above the powders'
  (0.035–0.08). Real earth grains are hematite or goethite in clay, with a lower effective
  index. No net gain, so not taken.
- **K's shape.** Yellow ochre's error is its red end: 0.72–0.76 at 600–700 nm against the
  powder's 0.54–0.57. Its K there is the thin swatch's, near nothing. Goethite does absorb
  weakly in the red (its Fe³⁺ crystal-field bands near 650 and 900 nm), which a thin gum
  swatch can't resolve. A measured goethite k(λ) would fix it. The published ones disagree
  (Bedidi & Cervelle 1993 give k 0.07–0.13, flat, held too high by later work: Zhang et al.
  2015, Formenti et al. 2014), so none is used. Red earth's a\* is the same kind of error.
  **Left open.**

### 8.6 The earths in oil (`--@ oil-earths`)

**Cause [C].** The earths' oil tables have the thin-swatch K/S (§7), so their oil
masstones are as pale as their old dry sticks were. A thick drawdown of burnt sienna
comes out at L\* 55 and raw umber at 57.5. A burnt sienna or raw umber in oil is
near-black in masstone (Golden's acrylics: burnt sienna 33.96/17.64/14.17, raw umber
28.2/1.45/3.33; CHSOS's linseed-oil swatches below).

**The rule.** A pigment absorbs per volume the same in any medium (§2.4). So in oil each
earth (and viridian) takes the K per volume its dry powder shows (the table's K ×
`dry::powder_k`, §7). Its S is what those same grains scatter in oil: the dry rule's Mie S
at its index (`dry::rule_scatter`) over Mie's air/oil ratio there, a multiple of lead
white's oil S (`mixer::OIL_EARTHS`). Nothing is fitted to the oil data. That data is the
check.

R∞ in oil (the engine's internal masstone) against CHSOS's linseed-oil swatches
(Kremer, the same pigments as the powders), corrected as the powders with the oil
blacks' floor and inverted for the glossy film's surface (Saunderson, k1 0.04, k2 0.6,
§3), ΔE76:

| Tube | oil before | oil with the switch | CHSOS oil (internal) | ΔE | as a sphere sees it: switch / CHSOS |
|---|---|---|---|---|---|
| burnt sienna | 184 114 94 (55.2/25.4/23.1) | 96 52 43 (27.1/19.0/14.2) | 67 60 64 (26.2/3.8/−1.1) | 43.4 → 21.5 | 29.5/7.5/4.5 / 29.2/1.4/−0.4 |
| red earth | 201 115 94 (57.6/31.8/26.4) | 137 65 55 (36.6/29.7/20.1) | 142 70 70 (38.9/30.2/14.2) | 22.4 → 6.4 | 33.8/16.0/8.8 / 35.0/17.1/7.0 |
| raw sienna | 198 142 86 (63.4/14.7/38.2) | 146 104 57 (47.5/11.4/32.9) | 173 114 78 (53.4/19.3/29.0) | 14.4 → 10.5 | 39.9/7.0/17.0 / 43.8/13.9/17.8 |
| yellow ochre | 251 200 120 (83.5/9.1/46.5) | 234 168 77 (73.5/15.7/55.1) | 207 147 87 (65.6/16.4/40.5) | 20.2 → 16.6 | 62.3/16.2/45.1 / 53.8/15.0/29.9 |
| raw umber | 172 130 96 (57.5/11.7/24.5) | 81 61 42 (27.3/6.1/15.0) | 84 65 65 (29.5/8.5/2.9) | 35.6 → 12.6 | 29.6/2.3/4.8 / 30.4/3.5/1.1 |
| viridian | 0 72 67 (23.5/−52.5/−8.1) | 0 85 75 (31.5/−27.3/−1.0) | 0 92 91 (33.9/−30.4/−7.8) | 24.4 → 7.8 | 31.3/−10.5/−0.4 / 32.5/−12.9/−3.5 |

Every tube moves toward its measured oil paint, and the lightness of each lands within a
few L\* of it. What is left is the same K shape as dry (§8.5): yellow ochre's red end, and
burnt sienna's hue. CHSOS's burnt sienna oil swatch reads near-neutral (a\* 3.8), where
Golden's burnt sienna has a\* 17.6, so the swatch itself is doubtful. Caveats: CHSOS's
swatches were brushed on paper with no control of thickness and aged two months in sun
(Cosentino 2015). Golden's are acrylics, with no method stated.

**Tints are not pinned.** No measured tint of an earth with white was found (CHSOS has
none, and Revigo's ladders have no earths). Tints rest on the grains' size, as the dry tints
do (§7.3). At 1 µm the K is the most iron oxide could give, and a 1:1 by volume with lead
white is dark: R∞ in oil burnt sienna 120 69 59 (35.1/20.9/15.6), raw umber 36.1, red earth
43.7. A thick pile in the engine is darker still, burnt sienna L\* 25 over a masstone of 18.
With 3 µm grains the masstones stay where they are (the factor follows S) and the tints come
out at 46.2, 49.4 and 54.5 (`dry_colours_fit.py`). Kremer gives no size for 40430 or 40610.
A measured tint, or a measured size, would settle it.

**Estimates left:** the earths' 1 µm grains (as §7), which set their tints in oil and dry
(replaced by their measured S: §9);
the oil S takes the dry Mie S over the 1 µm air/oil ratio; the CHSOS floor (§8.2).

Sources for this section: Cosentino 2014 and 2015 (*Int. J. Conserv. Sci.* 6:287–298,
table 1, the Kremer numbers); Lethuillier et al. 2022, "Cometary dust analogues for
physics experiments", *MNRAS*, doi:10.1093/mnras/stac1734 (arXiv:2110.05832), table 4;
Jäger, Mutschke & Henning 1998, *A&A* 332:291 (cellulose chars, Jena OCDB; as charcoal.md);
MFA CAMEO, "Bone black" (10 % carbon, 84 % hydroxyapatite; n 1.65–1.70); Natural
Pigments and Rublev, bone black (10–20 % carbon; particles 3–15 µm); Pourliaka et al.,
"Discriminating natural and synthetic ultramarine", PoS(BPU11)142; "Treating
smalt: a preliminary SEM-EDX study", *Stud. Conserv.* (2021),
doi:10.1080/00393630.2021.1940721 (Kremer smalt 60–120 µm); Kremer Pigmente, 10000 Smalt
(0–120 µm), 45800 Cobalt Violet dark; Formenti et al. 2014, *JGR Atmospheres*,
doi:10.1002/2014JD021668 (goethite and hematite k after Bedidi & Cervelle 1993, *JGR*
98:11941); Zhang et al. 2015 (as §7); Golden Artist Colors (as §7).

## 9. The black floor measured, and the earths' scattering measured

Computation: `earth_scatter_lbnl.py` (the measured scattering and the tint check),
`dry_earth_fit.py` (the refit) and `dry_colours_fit.py` (the blacks, the oil earths),
this folder. This section supersedes §7's and §8's floor of 0.04 and their 1 µm grains.

### 9.1 The floor: 0.022

The CHSOS correction's black floor is now the measured 0.022 (§8.2: juniper charcoal powder
reflects 3.00 % at 550 nm, Lethuillier et al. 2022, and CHSOS's own differences between the
blacks put vine black 0.0076 above their mean). The 0.04 came from §3's veil estimate, which
#329 retired. The bounds are 0.013–0.031: from the Mie vine black (R∞ 0.021, §8.1) to the
same 0.9 % above the charcoal. CHSOS's linseed-oil swatches keep a floor of 0.04: their
blacks are glossy films, whose surface alone reflects 0.037 (Fresnel at n 1.48).

The corrected powders at 0.022 (L\* over the bounds) [M/C]:

| Powder (Kremer) | at 0.04 (§7.1) | at 0.022 | L\* range |
|---|---|---|---|
| burnt sienna | 32.6/21.7/17.6 | 28.4/25.7/24.0 | 25.5–31.0 |
| red ochre | 42.4/22.8/17.8 | 39.8/25.1/20.7 | 37.6–41.8 |
| raw sienna | 57.3/15.1/38.6 | 55.9/15.9/42.6 | 54.1–57.6 |
| yellow ochre | 68.8/12.3/42.9 | 68.0/12.7/45.7 | 66.3–69.7 |
| raw umber | 34.8/9.4/16.1 | 31.1/11.1/20.6 | 28.4–33.5 |
| viridian | 46.3/−36.9/−3.0 | 44.1/−41.3/−3.3 | 42.1–46.0 |
| vine black | 26.2/−0.6/0.6 | 20.1/−0.8/0.8 | 16.0–23.5 |
| bone black | 21.7/0.0/0.3 | 13.3/0.0/0.5 | 6.2–18.2 |

**The blacks.** Their rule (§8.1) has no fitted number, so it doesn't move: vine black dry
16.1/0/0 is ΔE76 4.2 from its powder at 0.022, bone black 11.6/0/0 is 1.8 (10.1 and 10.1 at
0.04).

**A check on the floor [C].** At 0.022 the deepest bands of some powders fall below vine
black's 0.028–0.031: burnt sienna 0.017 at 450 nm, Prussian blue 0.007 at 700 nm, burnt
umber below 0.01. For Prussian blue (n 1.56, a facet reflecting ~5 %) that is physical.
For burnt sienna it is physical only if its grains' surfaces are clay-like (n ~1.55–1.9)
rather than hematite's (n ~3, whose facets reflect ~3× charcoal's). Natural earths are
hematite crystallites of 30–100 nm in and on clay (Hradil et al. 2003), so it is
plausible, but not shown.

### 9.2 The earths' scattering, measured

The earths' tints with white rest on how much their grains scatter per volume (§7.3, §8.6).
#351 took that from Mie at an estimated 1 µm (number median, σ 0.5: a volume median of
2.1 µm).

**Measured grain sizes [M].** Reichert et al. 2025 (*Anal. Bioanal. Chem.* 417:4351, table
3) sized Kremer pigments by laser diffraction (Malvern Mastersizer 2000LF, in ethanol, by
volume): burnt umber 40710, peak 6.4 µm (range 0.2–42); yellow ochre 116421, 10.5 µm
(0.2–76); Cremnitz white 46000, 3 µm (0.1–10). No size was found for burnt sienna 40430,
raw sienna, red ochre, raw umber or viridian. Kremer's catalogue gives none; Natural
Pigments rates its siennas only "medium". The siennas' grains are hematite or goethite
crystallites of 30–100 nm held in clay (Hradil et al. 2003), so "the grain" is the
agglomerate that laser diffraction sees.

**These sizes can't pin S by themselves.** S per volume is carried by the fine tail (Mie σ′
peaks near 0.3–0.5 µm), which laser diffraction resolves worst and which a peak and a range
don't describe. Read as log-normal either side of the peak with the range's ends at 3σ,
burnt umber 40710 gives burnt sienna (n 2.3) 0.30 µm⁻¹ in a binder of n 1.49. Read with the
ends at 4σ it gives 0.18, and from its coarse side alone 0.11.

**Measured scattering [M].** The LBNL Pigment Database (Levinson, Berdahl & Akbari, *Sol.
Energy Mater. Sol. Cells* 89 (2005) 319–349 and 351–389) measured Liquitex acrylic films
(binder n ≈ 1.49, fresh linseed 1.48) of burnt sienna B13, raw sienna B14 and raw umber B15
(natural iron oxides, PBr 7). Each was measured as a masstone and as 1:4 and 1:9 tints by
volume with titanium white W03, over a void, a white and a black ground, and inverted to K–M
K and S with each film's measured thickness. The datafiles are encrypted, but the spectral
charts are vector PDFs: `earth_scatter_lbnl.py` reads the curves back from their drawing
paths, calibrated on the charts' tick marks. Its check: the read K and S, seen through the
film's surface (Saunderson), give each opaque film's measured reflectance over black within
0.009.

| Earth (LBNL) | PVC | S per pigment volume, µm⁻¹: 450 / 550 / 650 / 700 nm | 550–700 mean | #351 (Mie, 1 µm) |
|---|---|---|---|---|
| burnt sienna B13 | 13.4 % | 0.064 / 0.118 / 0.186 / 0.139 | **0.185** | 0.417 |
| raw sienna B14 | 20.1 % | 0.248 / 0.395 / 0.364 / 0.327 | **0.382** | 0.347 |
| raw umber B15 | 10.9 % | 0.042 / 0.096 / 0.097 / 0.084 | **0.099** | 0.383 |

**Mie's absolute scale, checked [M/C].** Two synthetic hematites with the maker's mean size
(R03 0.27 µm at 2.6 % PVC, R02 0.30 µm at 8 %) scatter 2.8–5.3 µm⁻¹ per volume at 600–700 nm.
Mie's ¾σ′(1−g) at those sizes gives 2.5–6.5, depending on the width assumed (σ 0.5–0.3). So
the step from Mie to an absolute K–M S holds to within the size distribution. The natural
earths scatter 10–40× less per volume than those fine hematites. That fits coarse
agglomerates of lower effective index, not 1 µm hematite.

### 9.3 The tints, checked against measured tints

LBNL's tints check the S. Take each masstone's measured K/S, give it a candidate S per
pigment volume (K scaled to keep K/S), and mix it by volume with the measured white by K–M.
The result is compared with the measured tints' R∞ (internal), ΔE76 at 1:4 / 1:9 by volume:

| S per volume | burnt sienna | raw sienna | raw umber |
|---|---|---|---|
| measured, spectral | 5.8 / 4.4 | 3.1 / 3.5 | 2.6 / 1.8 |
| measured, 550–700 nm mean (**taken**) | 5.9 / 4.1 | 2.0 / 0.5 | 6.7 / 4.7 |
| Mie at #351's 1 µm | 13.7 / 10.0 | 1.3 / 0.5 | 17.3 / 13.0 |
| Mie at Kremer burnt umber's measured size (3σ ends) | 9.9 / 7.1 | 2.7 / 2.8 | 13.2 / 9.6 |
| Mie at Kremer yellow ochre's measured size | 7.5 / 5.3 | 4.3 / 4.0 | 10.4 / 7.5 |

The 1 µm grains made burnt sienna's and raw umber's tints too dark: L\* 80 against a
measured 85 for burnt sienna at 1:4, and 81 against 89 for raw umber. Raw sienna's measured
S is near the 1 µm value. The measured S reproduces the measured tints. The flat mean keeps
the engine's way of giving an earth lead white's spectral shape of S. Raw umber's S falls
in the blue (0.042 at 450 nm), which the flat mean misses (b\* 12.6 against 5.9 at 1:4).

### 9.4 The rule now (`--@ dry-earths`, `--@ oil-earths`)

(Burnt sienna, raw sienna and raw umber have since taken their films' K and S, §10.)

An earth's S per pigment volume in oil is the measured 550–700 nm mean (`dry::earth_scatter`).
It enters as a multiple of lead white's oil S, whose S per volume is the dry rule's scale
(¾σ′ in oil at 1 µm, 0.347 µm⁻¹; that scale is the estimate left, §9.5). Dry, it is that
times Mie's air/oil ratio at the grains' measured size: burnt umber 40710's for the siennas,
umber and red earth, yellow ochre 116421's for yellow ochre. The ratio hardly depends on
size: 1.67 against 1.57 at 1 µm for n 2.3, 2.35 against 1.90 for n 2.0. Red earth takes
burnt sienna's S (both are hematite in clay; no red earth film was measured), and yellow
ochre takes raw sienna's (both goethite in clay). These two are proxies. Viridian keeps Mie
at 1 µm: no measurement was found.

| Tube | S in oil (× lead white's) | dry | `powder_k` (was) | dry ΔE76 to powder (was, floor 0.04) | oil ΔE76 to CHSOS (was) |
|---|---|---|---|---|---|
| burnt sienna | 0.531 (1.201) | 0.886 | 16.79 (31.71) | 11.0 (2.6) | 17.0 (18.0) |
| red earth | 0.531 (1.465) | 0.824 | 3.600 (7.718) | 5.0 (8.1) | 5.0 (5.6) |
| raw sienna | 1.099 (1.000) | 2.581 | 15.19 (10.08) | 10.5 (6.7) | 9.9 (8.4) |
| yellow ochre | 1.099 (1.101) | 2.350 | 3.137 (2.087) | 10.3 (11.2) | 21.3 (22.5) |
| raw umber | 0.286 (1.101) | 0.553 | 5.620 (16.49) | 6.4 (2.6) | 9.6 (10.2) |
| viridian | 0.573 (0.573) | 1.710 | 4.298 (3.910) | 10.4 (5.7) | 7.1 (6.8) |

(Oil ΔE: the engine's pile, gloss excluded, against CHSOS's linseed swatches less their
4 % specular, as the paint test has them. "Was" is #351's: dry from §7.2's fit, oil its
test's tolerances less their 2.)

The pure sticks' larger ΔEs come from the floor, not the S: one factor on K takes up any S
scale exactly. At 0.022 the powders are more saturated (burnt sienna C\* 35 against 28) than
the oil tables' K shape reaches with one factor. Burnt sienna's stick is now L\* 31.4 against
28.4, but a\*/b\* 20/15 against 26/24. That is §8.5's shape problem, now larger.

**K against hematite [C].** K per volume = K/S × the measured S dry. Burnt sienna's at
550 nm is 2.1 µm⁻¹, half a pure hematite bed's 4.2 (k 0.0925). With 1 µm grains (§7.3) it
had needed all of it. Red earth comes to 34 %, raw umber 19 % (with manganese oxide), raw
sienna 21 % and yellow ochre 3 % (goethite absorbs less than hematite at 550 nm). These are
plausible for iron oxide in clay. The paint test is `dry_burnt_sienna_absorbs_as_hematite_in_clay`.

**Tints, 1:1 by volume with lead white**, R∞ in oil (`dry_colours_fit.py`), #351 → now:
burnt sienna 35.1 → 38.9 L\*, red earth 43.7 → 47.7, raw umber 36.1 → 44.5, raw sienna 57.2
→ 52.2, yellow ochre 78.9 → 75.6, viridian 45.8 → 44.4. The masstones went deeper with the
floor (burnt sienna 27.1 → 24.8), which takes back part of the lightening. In the engine's
pile (gloss excluded), burnt sienna 1:1 is L\* 28.2 over a masstone of 16.5. Dry, 1:1 burnt
sienna with lead white goes from #351's 143 86 75 (L\* 42) to 155 97 86 (L\* 47.4).

These are not checked against a measured tint *with lead white*: none was found. LBNL's
white is titanium white, about 8× lead white's S per volume. The check (§9.3) is of the
earths' S per volume against LBNL's measured tints. The paint test
`earth_tints_follow_from_their_measured_scattering` holds the engine's S there: each
measured tint's R∞ within 0.06 at 550, 600 and 650 nm.

### 9.5 Estimates left

- Lead white's S per volume, the scale all of these are multiples of: Mie at 1 µm (0.347
  µm⁻¹ in oil; a measurement-anchored bracket, 0.25–0.38, holds it: §10.4). Kremer's Cremnitz white measures 3 µm (0.1–10), and Mie at that size gives
  0.50, or less depending on the tail. No absolute measured S of lead white in oil was
  found. If it is larger, every earth's tint with lead white is lighter in proportion.
- The LBNL earths are Liquitex's, not CHSOS's Kremer lots. Their S stands in for Kremer's.
- Red earth's and yellow ochre's S are proxies (burnt sienna's and raw sienna's). Viridian's
  is Mie at 1 µm.
- The flat 550–700 nm S. The measured S falls in the blue (burnt sienna 0.064 at 450 nm),
  which the engine's lead-white shape doesn't follow.
- The non-additivity seen in LBNL's tints: an earth's K in a tint is 1.6–2× its masstone K
  per volume (dispersion). K–M with the measured masstone S still lands within ΔE 2–6.

### 9.6 #318's dye lakes at the floor 0.022

#318's `dye_loading` (fork/fix/dry-lake-color, `dry_lake_fit.py`) was fitted against the
same correction at 0.04. Rerun at 0.022 (bounds 0.012–0.032), with nothing else changed:

| Lake | target at 0.04 → 0.022 | loading 0.04 → 0.022 | fit ΔE76 | over the bounds |
|---|---|---|---|---|
| rose madder | 54.5/24.6/8.8 → 52.9/26.0/9.4 | 35.7 → **40.5** (+13 %) | 5.0 → 5.3 | 38.1–46.3 |
| carmine lake | 30.7/41.0/7.7 → 26.1/48.6/10.1 | 3.01 → **3.97** (+32 %) | 9.9 → 4.8 | 3.25–4.52 |
| yellow lake | 66.1/2.9/41.9 → 65.1/3.1/45.0 | 12.2 → **13.7** (+12 %) | 3.5 → 3.1 | 12.0–15.5 |
| magenta | dye chemistry, no powder | 6.26, unchanged | — | — |

Sources for this section: Lethuillier et al. 2022 (as §8); Levinson, Berdahl & Akbari,
"Solar spectral optical properties of pigments", Parts I and II, *Sol. Energy Mater. Sol.
Cells* 89 (2005) 319–349, 351–389, and the LBNL Pigment Database spectral charts B13, B14,
B15, R02, R03 and W03 (https://coolcolors.lbl.gov/LBNL-Pigment-Database/, SHA-256 in the
script); A. S. Reichert et al., "Database of DRIFTS and HSI spectra of pigments and dyes
for historical document analysis", *Anal. Bioanal. Chem.* 417 (2025) 4351–4372,
doi:10.1007/s00216-025-05948-3, table 3; D. Hradil, T. Grygar, J. Hradilová & P. Bezdička,
"Clay and iron oxide pigments in the history of painting", *Appl. Clay Sci.* 22 (2003)
223–236 (iron oxide crystallites 30–100 nm in natural ochres); Zhang et al. 2015 and Longtin
et al. 1988 (hematite k, as §7.3); Natural Pigments, Italian Burnt Sienna (particle size
"medium", oil absorption 45–55 g/100 g for its Ardennes burnt sienna).

## 10. The earths' absorption and scattering measured together (LBNL films)

Computation: `earth_lbnl_ks.py` (this folder). This section supersedes §7–§9 for burnt
sienna, raw sienna and raw umber, and adds the Mars colours.

**The problem [C].** §9 took the earths' S from LBNL's films, but kept their K: the thin
gum swatch's shape times one fitted factor (`powder_k`), over lead white's spectral shape of
S. One factor can't fix a shape. LBNL measured K and S *together*, spectrally, of each film
(Levinson, Berdahl & Akbari 2005: each film over a void, a white and a black ground,
inverted with its measured thickness). So every tube whose pigment LBNL measured takes both.

### 10.1 Which tubes LBNL covers

| Tube | LBNL | Pigment | PVC |
|---|---|---|---|
| burnt sienna | B13 | calcined natural iron oxide, PBr 7 (Liquitex) | 13.4 % |
| raw sienna | B14 | natural iron oxide, PBr 7 | 20.1 % |
| raw umber | B15 | natural iron oxide, PBr 7 | 10.9 % |
| Mars red | R04 | synthetic red iron oxide, PR 101 (Liquitex "Red Oxide") | 13 % |
| Mars yellow | Y01 | synthetic hydrated iron oxide, PY 42 (Liquitex "Yellow Oxide") | 16 % |

Not taken, and why:
- **red earth, yellow ochre:** natural ochres. LBNL measured only synthetic iron oxides
  (R01–R04, Y01). They keep §9's method (their table K × `powder_k`, burnt sienna's and raw
  sienna's measured S). On B13's K and S red earth would be 21.9 ΔE76 off its Kremer powder
  dry (5.0 now), on R04's 38.7. Yellow ochre on Y01's (synthetic goethite) would be 15.6 off
  (10.3 now). On raw sienna's film B14, a natural goethite earth, it would be 6.4 off dry and
  9.7 off CHSOS's swatch in oil (18.7 now). That is better on both, but raw sienna is not
  yellow ochre, so it is left as an open question.
- **Indian red, transparent oxide yellow:** other grades of PR 101 and PY 42 than R04's and Y01's.
- **The cadmiums:** LBNL's are PY 35 (cadmium zinc sulfide) and PO 20 (a sulfoselenide
  orange). The engine's are cadmium sulfide (PY 37) grades and a sulfoselenide red.
- **Cobalt blue (U05), chrome yellow (Y03), ultramarine (U11), Prussian blue (U10), bone
  black (B02):** the engine's tables for these are measured oil tables already (calibrated
  on oil tints), not thin swatches. U11's, U10's and B02's masstone S lies below the
  charts' axis, so it can't be read.
- **Viridian:** LBNL's chromium greens are anhydrous PG 17, not hydrated PG 18. Viridian
  keeps §9's method.

### 10.2 The read, checked

Each masstone's K and S, read back from the vector chart as in §9.2, refit to its own
film's reflectance over the white and the black ground (Kubelka's layer, Saunderson's
surface; the thickness and the grounds fitted) [M/C]:

| Film | fitted thickness | rms, largest | R∞ inside the paint |
|---|---|---|---|
| burnt sienna | 15 µm | 0.002, 0.006 | 143 61 22 (36.4/32.5/38.8) |
| raw sienna | 25 µm | 0.006, 0.012 | 188 122 61 (57.0/20.1/43.1) |
| raw umber | 11 µm | 0.002, 0.005 | 93 60 17 (28.5/10.2/31.2) |
| Mars red | 16 µm | 0.003, 0.011 | 192 74 21 (47.2/45.4/51.7) |
| Mars yellow | 18 µm | 0.002, 0.006 | 223 173 70 (73.6/7.9/58.0) |

Raw sienna's chart starts at 445 nm, and its values below are held there [E].

Per pigment volume (µm⁻¹), 450 / 550 / 650 nm:

| | K | S |
|---|---|---|
| burnt sienna | 1.883 / 1.137 / 0.181 | 0.064 / 0.118 / 0.186 |
| raw sienna | 1.583 / 0.563 / 0.139 | 0.248 / 0.395 / 0.364 |
| raw umber | 1.766 / 0.789 / 0.391 | 0.042 / 0.096 / 0.097 |
| Mars red | 3.410 / 2.885 / 0.266 | 0.160 / 0.395 / 0.971 |
| Mars yellow | 1.522 / 0.145 / 0.059 | 0.438 / 0.548 / 0.390 |

**An independent check [M].** Golden Artist Colors publish CIE L\*a\*b\* for their Burnt
Sienna (PBr 7) acrylic: 33.96/17.64/14.17. LBNL's burnt sienna film seen through its surface
(Saunderson, k1 0.04, k2 0.6) is 33.8/17.6/13.9, **ΔE76 0.3**. #351's burnt sienna seen so was
28.7/6.7/4.0 (ΔE 15.8). Golden's Raw Umber (28.2/1.45/3.33) is a cooler umber than
Liquitex's: the film is 6.1 off it, #351's 0.7.

**Tints [M/C].** The masstone K and S per volume, mixed by K–M with the measured white W03,
against the measured tints, ΔE76 at 1:4 / 1:9: burnt sienna 5.8 / 4.4, raw sienna 3.1 / 3.5,
raw umber 2.6 / 1.8, Mars yellow 6.5 / 6.3, Mars red 3.7 / 10.9. Mars red's 1:4 chart reads
badly (its S curve is flat across 450–550 nm, and its read misses its own film by 0.098), so
its tints aren't used as a check. What is left is §9.5's dispersion: an earth's K in a tint
runs 1.6–2× its masstone's per volume.

### 10.3 Against the Kremer powders and swatches: other lots

| Tube | engine #351, oil R∞ | the film, oil R∞ | CHSOS linseed swatch ΔE, #351 → film | #351 dry | the film dry | CHSOS powder ΔE, #351 → film |
|---|---|---|---|---|---|---|
| burnt sienna | 90 47 39 (24.8/18.3/13.7) | 143 61 22 (36.4/32.5/38.8) | 20.7 → 50.1 | 109 61 51 (31.4/20.1/15.0) | 164 77 33 (43.1/33.2/41.2) | 11.0 → 23.8 |
| raw sienna | 133 94 48 (43.0/11.1/32.1) | 188 122 61 (57.0/20.1/43.1) | 13.6 → 14.6 | 168 123 75 (55.1/11.7/33.0) | 212 153 93 (67.6/15.9/40.1) | 10.5 → 12.0 |
| raw umber | 72 53 36 (24.0/5.7/14.1) | 93 60 17 (28.5/10.2/31.2) | 12.8 → 28.4 | 95 72 51 (32.5/6.7/16.2) | 119 80 28 (37.5/10.9/36.0) | 6.4 → 16.6 |
| Mars red | 156 92 92 (46.3/26.4/11.4) | 192 74 21 (47.2/45.4/51.7) | — | 170 106 106 (51.7/25.8/10.9) | 205 86 32 (51.4/44.6/51.9) | — |
| Mars yellow | 237 175 100 (75.9/14.8/46.7) | 223 173 70 (73.6/7.9/58.0) | — | 244 189 123 (80.2/11.9/40.9) | 232 187 93 (78.3/5.4/53.0) | — |

(Without the switches Mars red and Mars yellow are the "#351" columns; the earths' "#351"
is §9's switch. Dry: the film's S times Mie's air/oil ratio, the earths' at their measured
size (1.669, 2.349, 1.936, §9.4), the Mars colours' at the rule's 1 µm (1.454, 1.666) [E].)

The films are further from CHSOS than #351 was. That was expected for the powders, because
#351's factors were *fitted* to them. The swatches weren't fitted, but CHSOS's burnt sienna
swatch is doubtful (a\* 3.8: §8.6). The plain cause is that these are different lots of
natural earth. Kremer's 40430 is sold as a *dark* burnt sienna. Liquitex's and Golden's
burnt siennas agree with each other (above), and both are lighter and more orange. The
catalog's own burnt sienna (#7c3f24, 33.9/24.1/28.0) has their lightness seen through a
surface, with a chroma between theirs and Kremer's. The engine's tube was never Kremer's
lot in oil: its K was Kremer's thin swatch, read as a film it isn't. Raw umber's lots vary
most (manganese): Golden's is near-neutral, Liquitex's yellower, Kremer's between.

What the films change for the painter: burnt sienna in oil is a deep orange-brown, not the
dull brown §9 gave it. Dry, the pure stick is L\* 43, against §9's 31, the original 61 and
Kremer's dark powder 28. Mars red is an oxide red at last; it had been a pink (its thin
swatch).

### 10.4 Lead white's S per volume

Every earth's measured K and S enter the engine as multiples of lead white's calibrated S,
so lead white's S per volume in oil sets how strongly they tint with it. §9 took it from Mie
at an estimated 1 µm: 0.347 µm⁻¹.

**Searched for and not found open [—].** Measured K–M S of lead white films: Mitton &
Vejnoska, "Hiding power of white pigments: theory and measurement" (Official Digest, 1961),
measured white lead's S in acrylic at seven concentrations and three wavelengths. It is not
accessible here. Berns et al.'s Seurat and Van Gogh studies characterised lead white in
linseed oil with single-constant K–M (K/S only). Conservation work on lead white
transparency (Rees Jones; de Viguerie et al. 2018, *C. R. Physique* 19:543) reports
relative reflectance, not S.

**A measurement-anchored bracket [M/C].** White lead's tinting strength is 1/18.5 of rutile
titanium's by weight (rutile 1850, anatase 1350, zinc oxide 200, white lead 100,
"comparative tinting strengths of white pigments in a standard blue"; the classic table in
Shreve's *Chemical Process Industries*). Tinting strength is S per weight, so
S_lead = S_rutile × (ρ_lead/ρ_rutile)/18.5, with ρ 6.8 (hydrocerussite) and 4.1 (pigment
rutile). LBNL measured rutile's S per volume at four concentrations:

| Rutile | PVC | S at 550 nm (µm⁻¹) | lead white so |
|---|---|---|---|
| W04 | 2 % | 4.12 | 0.369 |
| W02 | 9 % | 4.29 | 0.384 |
| W01 (Tipaque CR-90, 0.25 µm) | 15 % | 3.60 | 0.323 |
| W03 | 22.8 % | 2.76 | 0.247 |

The tinting pastes' crowding isn't stated. A rutile paste at its oil absorption is more
crowded than W03, which would push the figure lower still. Mie at Kremer's Cremnitz white's
measured 3 µm gives 0.50 or less (§9.5). **The bracket, 0.25–0.38, holds the rule's 0.347, so it
stays.** No tint moves for decision 2. Lead white's own tube doesn't change either way: only
the scale the earths enter on (`dry::lead_white_scatter`). Engine 9's lead white S is
calibrated per coat and bumpy across the spectrum (2.51 at 550 nm, 1.68 at 600), so the
scale takes its luminance-weighted mean (1.720 per coat in the dry path's units), as a
tinting strength is judged by eye.

### 10.5 The rule now (`--@ dry-earths`, `--@ oil-earths`)

- **Burnt sienna, raw sienna, raw umber, Mars red, Mars yellow** (`dry::lbnl_film`): in
  oil, an `OIL_EARTHS` slot each, with K and S = the film's per volume × lead white's
  luminance-weighted S per coat / 0.347 (`mixer::lbnl_pigment9`). Dry, the same K and that
  S × the air/oil ratio. `powder_k` is gone for the three earths.
- **Red earth, yellow ochre, viridian:** as §9 (`powder_k` 3.600, 3.137, 4.298, unchanged:
  `dry_earth_fit.py` reproduces them).
- The Mars colours are new to both switches. `OIL_EARTHS` is two slots wider (eight), and
  the checkpoint identity changes again. A checkpoint from before the slots still reads
  (`mixer::identity9_before_oil_earths`). #351 was unmerged, so no checkpoint has its six
  slots.

**K against hematite [C].** The films' K per volume at 550 nm against a pure hematite bed's
4.23 µm⁻¹ (k 0.0925): burnt sienna 1.14 (27 %), raw umber 0.79 (19 %), raw sienna 0.56 (13 %),
Mars red 2.89 (68 %), Mars yellow 0.15 (3 %, goethite). Both iron reds lie under it.

**Tints 1:1 by volume with lead white**, R∞, §9's switch → now (in oil; dry):
- burnt sienna: 131 77 67 (38.9/21.6/16.0) → 172 107 88 (51.9/23.5/21.4); dry 155 97 86 → 192 130 112 (60.2/21.5/19.4)
- raw sienna: 160 116 68 (52.2) → 203 146 102 (65.1/16.6/31.9); dry 187 142 95 → 222 171 130
- raw umber: 128 100 76 (44.5) → 140 117 89 (50.6/4.9/18.7); dry 154 123 99 → 164 140 113
- Mars red (no switch before): 187 131 138 (60.4/22.5/4.7) → 198 94 71 (52.4/40.0/32.5)
- Mars yellow (no switch before): 243 189 127 (80.0) → 232 187 104 (78.3/6.5/47.4)

The earths' tints are lighter than §9's. Their K per volume is the film's, not a factor
fitted to Kremer's dark lot, and burnt sienna's is 2× less.

### 10.6 Estimates left

- Lead white's S per volume: Mie at 1 µm (0.347), inside the bracket 0.25–0.38 (§10.4).
- The Mars colours' air/oil ratio: the rule's at 1 µm. Synthetic oxides are finer (LBNL's R02
  and R03, 0.27–0.30 µm), where the ratio is larger, so their dry sticks may be paler than
  shown.
- Raw sienna below 445 nm (held flat).
- The films are Liquitex's lots. The natural earths vary by lot (§10.3).

Sources for this section: Levinson, Berdahl & Akbari 2005 and the LBNL Pigment Database
spectral charts B13, B14, B15, R04, Y01, W01, W02, W03 and W04 (SHA-256 in
`earth_lbnl_ks.py`); Golden Artist Colors, Burnt Sienna and Raw Umber (as §7); Mitton &
Vejnoska, "Hiding power of white pigments: theory and measurement", *Official Digest* (1961)
(not read); R. N. Shreve, *Chemical Process Industries* (the white pigments' comparative
tinting strengths, as reproduced in KFUPM CHEM 456 lecture notes, ch. 7, table 7.2);
L. de Viguerie et al., "Re-interpretation of the Old Masters' practices through optical and
rheological investigation: the presence of calcite", *C. R. Physique* 19 (2018) 543–552;
R. S. Berns et al., "Rejuvenating the color palette of Georges Seurat's *A Sunday on La
Grande Jatte—1884*: a simulation", *Color Res. Appl.* (2006); Zhang et al. 2015 and Longtin
et al. 1988 (hematite k, as §7.3).

## 11. The earths' tints with lead white: the tube's own pigment (`--@ earth-tints`)

A painter's oil (cassatt8) found yellow ochre and burnt sienna far too strong with lead white:
7 % by volume of ochre a peach, L\* 81.2/10.3/36.4, and of burnt sienna 67.8/13.9/10.5 (a
knifed tiny scene on that day's integration build; lead_white_film.md §1 has the ladders).

**The cause [C].** An oil-earths slot (§8.6, §10.5) holds the earth's K and S per pigment
volume times `mixer::oil_earths_scale`: lead white's calibrated S over its S per pigment volume
(Mie at 1 µm, 0.347 µm⁻¹). That makes the slot the earth's pigment at the volume share lead
white's paint holds. But a part of a tube is the box's tube paint, and the earths are ground
in far more oil than lead white (pigment_oil.md §4–5):

| impressionist tube | oil by weight | density | pigment share by volume | weight |
|---|---|---|---|---|
| lead white | 0.13 | 6.81 | 0.478 | (lead white's own) |
| yellow ochre | 0.43 | 2.8 | 0.306 | 0.494 |
| red earth | 0.45 | 4.0 | 0.221 | 0.358 |
| viridian | 0.50 | 3.2 | 0.225 | 0.364 |
| raw sienna | 0.68 | 3.27 | 0.118 | 0.191 |
| burnt sienna | 0.63 | 3.95 | 0.122 | 0.196 |

And the scale's estimate isn't needed. The Revigo films behind engine 9's units were 32 µm
(lead_white_film.md §4, from Bruce's gauged lead white films), so one engine unit is 32 µm of
paint, and lead white's S per pigment volume there is 0.44 µm⁻¹, not 0.347.

**The rule.** With `--@ earth-tints` (and `--@ oil-earths`), an earth enters a mixture at the
weight (its tube's pigment share) × `REVIGO_FILM_UM` / `oil_earths_scale`
(`Palette::earth_tints_weight`), so a micrometre of its tube paint holds its measured K and S
per pigment volume times its pigment share. Its masstone doesn't change (K and S scale
together); its tints and its films' hiding do. Without the switch nothing moves; without
`--@ oil-earths` it does nothing (the earths' older tables aren't per pigment volume).

**Tints by volume with lead white** (the same scene, `--@ earth-tints` with and without
`--@ tube-volume`): yellow ochre 3 % 86.1/7.3/27.9 → 89.0/4.9/22.0, 7 % 81.2/10.3/36.4 →
85.2/7.7/30.3, 15 % 76.3/12.4/43.8 → 80.6/10.8/37.7; burnt sienna 3 % 74.7/11.4/8.8 →
85.9/7.1/5.4, 7 % 67.8/13.9/10.5 → 81.8/9.2/6.8, 15 % 56.2/17.5/14.1 → 73.1/12.6/9.3.

**A check against a measured ochre ladder [M/C].** No measured tint of a natural ochre with
lead white was found. Okumura (RIT, 2005) measured Golden's acrylic Yellow Ochre (PY 43), Burnt
Sienna and Raw Sienna, each 20/40/60/80/100 % by weight with Golden's titanium white
(GretagMacbeth Color-Eye XTH, specular included). Fitting each ladder by two-constant K–M
(Saunderson k1 0.04, k2 0.6) against the same white, per gram of paint at 450 nm, the ochre
absorbs 0.49 of the white's S, burnt sienna 0.51, raw sienna 0.37. Golden's burnt sienna is
LBNL's (§10.2: its masstone within ΔE 0.3), so it reads as a burnt sienna paint of about 16 %
pigment by volume (LBNL's own film was 13.4 %). If Golden's ochre is ground to that loading
too, its K per pigment volume at 450 nm is about 1.9 µm⁻¹, the engine's 2.29 [E: Golden's
loadings aren't published]. Along the ladder (specular taken out) the ochre's tints run 82.6/
5.2/27.4, 76.8/7.9/35.2, 71.5/10.5/41.5; the engine's with lead white run about 7 b\* more
saturated at the same L\*, about 4 of it lead white's own yellow (b\* 4.6 against titanium
white's 1.4). So the ochre's K is left as §9 has it.

### 11.1 Estimates left

- The ochre's K per pigment volume against Golden's, which rests on Golden's unpublished
  loadings (above). A measured ladder of a natural ochre with lead white in oil would settle it.
- That Revigo's lead white scatters per pigment volume as Bruce's white lead did (the 32 µm).
- The box's tubes' oil (pigment_oil.md: Church's British analyses, the French figures a third
  oilier) and the catalog's pigment densities.

Sources for this section: Y. Okumura, *Developing a Spectral and Colorimetric Database of
Artist Paint Materials*, M.S. thesis, RIT Munsell Color Science Laboratory (2005), and its
data (http://www.rit-mcsl.org/StudentResearch/paint_research.zip: GOLDEN_fluid_matte_all_sci.txt,
Paint_Recipe.xls "Tint Lader"); E. Bruce, *Technologic Papers of the Bureau of Standards* 306
(1926); lead_white_film.md; pigment_oil.md.

---

## Sources

- Feller (ed.) *Artists' Pigments* vol. 1 (1986); Roy vol. 2 (1993); FitzHugh vol. 3
  (1997); Berrie vol. 4 (2007). Eastaugh, Walsh, Chaplin, Siddall, *Pigment Compendium*
  (2004). Gettens & Stout, *Painting Materials* (1942).
- Pigments through the Ages, technical pages (lead white, zinc white, vermilion,
  viridian, emerald green, cadmium, Naples yellow, Indian yellow, red lead, madder,
  carmine, lime white): https://www.webexhibits.org/pigments/indiv/technical/vermilion.html
- Natural Pigments, "Why some paints are transparent and others opaque" and "Gloss, matte
  and the nature of paint surfaces":
  https://www.naturalpigments.com/artist-materials/transparent-opaque-paints ,
  https://www.naturalpigments.com/artist-materials/gloss-matte-paint-surfaces
- Wikipedia, "Hiding power" (extenders white in air, transparent in binder):
  https://en.wikipedia.org/wiki/Hiding_power
- Conservation Wiki, Inpainting: Pigments: https://conservation-wiki.com/wiki/Inpainting:_Pigments
- Linseed-oil index ageing (1.479 → > 1.525 in ~10 yr; ~1.58 in old paintings), via
  Natural Pigments transparency article and de la Rie.
- Berns & de la Rie, varnish refractive index and roughness papers:
  https://www.researchgate.net/publication/272252901 ,
  https://www.researchgate.net/publication/272309272
- NIST, "Relating gloss loss to topographical features of a PVDF coating":
  https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=860584
- Matt PU coating, roughness vs gloss: https://pmc.ncbi.nlm.nih.gov/articles/PMC7077453/
  (Rq preferred over Ra/Rz; not read in full, CAPTCHA)
- Cosentino, FORS database:
  https://www.researchgate.net/publication/266081590 ;
  https://chsopensource.org/new-free-pigments-fors-spectroscopy-database/
- Saunderson 1942, *JOSA* 32:727; Mudgett & Richards 1971, *Appl. Opt.* 10:1485 (k2
  polynomial); colour-science Saunderson implementation:
  https://github.com/colour-science/MunsellAndKubelkaMunkToolbox/blob/master/KubelkaMunk/SaundersonCorrection.m
- Kubelka & Munk 1931 (translation): https://www.graphics.cornell.edu/~westin/pubs/kubelka.pdf
- Ross 1971, theoretical light-scattering power of TiO₂ and microvoids. Bohren & Huffman,
  *Absorption and Scattering of Light by Small Particles* (BHMIE algorithm used here).
- Davies 1954, *Proc. IEE* 101:209; Bennett & Porteus 1961, *JOSA* 51:123; Beckmann &
  Spizzichino 1963.
- Lekner & Dorf 1988, *Appl. Opt.* 27:1278; Twomey, Bohren & Mergenthaler 1986, *Appl.
  Opt.* 25:431.
- Kortüm, *Reflectance Spectroscopy* (1969). Covers KM on powders.
- Computation: `dry_pigment_mie.py` (this folder). BHMIE, lognormal sizes, transport
  scattering σ_s(1 − g) per particle volume, λ = 550 nm.
