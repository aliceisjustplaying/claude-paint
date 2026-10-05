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
  but S rises ~2.4× in air. A dry carbon black is therefore a little greyer, never
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
no single source). The filler dominates the dry S, so dry pastel colours are tints of
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
the surface reflection (see below), or K/S will be wrong for dark colours.

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
  Richards give a polynomial in n. Effect: dark colours get darker and saturate (R_i 0.05
  → R_m 0.021 with k1 = 0, k2 = 0.6).
- **Matte paint in air** (lean oil, gouache): the interface still exists but is rough, so
  k1 ≈ 0.04–0.09 becomes a flat **diffuse** veil added to the colour. Keep k2 ≈ 0.6 for
  the binder-rich skin, or lower it where the surface is porous.
- **Powder or pastel in air:** there is no continuous interface. Diffuse-reflectance
  practice (Kortüm, *Reflectance Spectroscopy*) applies KM directly to R∞ with k2 ≈ 0. The
  first-surface Fresnel reflections from particle facets (~4–20 % per facet for n
  1.5–3.0) are already part of the volume scattering. At the top grains they act as an
  extra spectrally flat diffuse term. **Engine suggestion [E]:** k2 = 0, plus an additive
  flat k1 ≈ 0.02–0.04. This caps the darkest dry blacks at R ≈ 0.03–0.05. Example: R_i
  0.02 gives R_m ≈ 0.05 for powder vs 0.008 for an oil film, roughly 1.5 L* units against
  ~7. That gap is why a black pastel never matches a black oil glaze.

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
- Lekner & Dorf 1988 and Twomey et al. 1986 give measured and modelled wet/dry reflectance
  ratios for sand and soils: wet reflectance is ~0.5–0.8 of dry across the visible.

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
