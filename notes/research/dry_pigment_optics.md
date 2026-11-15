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
~0.8–1 for particles ≥ 2 µm. I found no published per-pigment value. For a lake's
alumina hydrate (n 1.57) at 1 µm it is computed in §6.3: 0.97 (Mie × Percus–Yevick).

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
  so it is worth a retry by hand. (Later reached with a browser's request headers: its
  powder spectra are used in §6.)
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

## 6. Dye lakes dry: measured powders and the dye loading

**The problem.** At engine 9 a pure rose madder pastel stick came out sRGB 242 218 223
(L\*a\*b\* 89/9/0), a near-white tint with no white in it, while carmine lake came out
171 58 87. Real madder lake powder, and the madder pastels made from it, are a rose to
crimson (below).

**Diagnosis [C].** `dry::spectral_color` gives a pigment under n = 1.9 the dry scattering
of lead white per volume, scaled by the Mie ratio at its index (×0.82 at n = 1.6), and its
oil K per coat. The scattering half is right for a lake. Its substrate, alumina hydrate
(gibbsite, n 1.57), scatters in air at 0.71 µm⁻¹ per volume at 1 µm (table 2.2), the same
as the table's 1.6 to within 1 %. An alumina hydrate powder is white and opaque in air,
like kaolin. The **absorption** half is what fails. The engine-9 tables fix a lake's K/S
in oil, not its K per volume of pigment:
- rose madder and permanent alizarin are researched tubes. Their K comes from a Kremer gum
  swatch fitted as an assumed 100 µm oil film, with S set by an estimated hiding
  (`scripts/pigment_data/researched-tubes.json`: "No new oil K/S or tint strength is
  measured").
- magenta and yellow lake are proxies: the catalog masstone at the catalog scattering.
- carmine is the only measured one (Revigo cochineal).

Rose madder's K at 510 nm is 0.19 per coat, against carmine's 6.7. Dry, against a
white-like S of 2–4 per coat, madder's K/S is about 0.05, so R∞ ≈ 0.73: the near-white.
(The same weak K makes engine 9's rose madder oil masstone pale: 211 126 125 decoded. That
is not changed here.)

### 6.1 Measured lake powders

**Cosentino 2014 FORS powders** (`CSV/powder.csv` of the CHSOS download, SHA-256
86bb05f0…ae45; Kremer pigments, 360–1000 nm). This is the "pure powder" set §5 could
not download.

**The instrument** (Cosentino 2014, *e-conservation* 2:54–65, §2.1; read from the CHSOS
copy of the paper): an Ocean Optics USB4000 spectrometer (360–1000 nm, 1.5–2.3 nm FWHM)
and an HL-2000-FHSA halogen lamp. Every spectrum in the database was taken with an
**ISP-R integrating sphere**, the powders included. The sphere collects the diffuse and
the specular reflection together, so the readings are total (specular-included)
reflectance. The gum-arabic swatches were also measured with an **R400-7 reflection
probe** (six illuminating fibers around one reading fiber) tilted to 45/45, which
excludes the specular. The paper does not name the white standard or describe the
reference. The raw powder spectra are not on an absolute scale: the four carbon blacks
read 0.16–0.17 flat, and the whites read 1.10–1.31. So the reference was not taken in
the samples' geometry (sample distance or seating).

**The correction from physics** [C]. `dry_lake_fit.py`:
- *The sphere is linear.* On the 54 gum-arabic swatches, each measured both ways,
  sphere = 0.178 + 1.015 × probe (400–700 nm, rms 0.065). A quadratic term comes out at
  −0.05, below the scatter. Chalk and gypsum are left out because, in gum (n 1.52–1.6),
  their swatches are the paper seen through a clear film. A single-beam sphere's
  substitution error would bend the curve upward, and it doesn't. So the raw reading is
  offset + gain × reflectance.
- *The offset* is set by the blacks. A carbon black powder's total reflectance is
  0.03–0.05: its grains' facet reflections (§3), and carbon-based pigments measured at
  3–5 % in the visible (SPIE *Photonics Focus*, May/June 2026, "In pursuit of the perfect
  black"). Midpoint 0.04.
- *The gain* is set by the whites, and **no powder can reflect more than 1**. The
  earlier correction mapped chalk and gypsum to 0.93. That puts titanium white at 1.085,
  so it is out. Rutile titanium white is the brightest powder and is flat from 550 to
  700 nm. A dry rutile powder reflects 0.90–0.99 there: bulk rutile 0.90–0.91, and fine
  pigment rutile 0.995 against a Spectralon WS-1-SL (Sanjeev et al., *Chalcogenide Lett.*
  — chalcogen.ro/607_Sanjeev.pdf). Midpoint 0.945. Lead white has no visible absorption,
  so it gives the gain's spectral shape. Its raw spectrum and titanium white's fall alike
  from 430 to 700 nm (lead 1.36 → 1.27): that is the lamp and reference, not the
  pigments. Corrected, lead white comes out at 0.92 (0.88–0.96), zinc white at L\* 96,
  chalk 92 and gypsum 91. Natural chalk and gypsum are off-white, so these fit.

Corrected L\*a\*b\* (D65) at black 0.04 and titanium white 0.945, with L\* over the bounds
(black 0.03–0.05 × titanium white 0.90–0.99) [M/C]:

| Powder | sRGB | L\* (range) | a\* | b\* |
|---|---|---|---|---|
| madder lake | 175 114 116 | 54.5 (52.6–56.2) | 24.6 | 8.8 |
| carmine lake | 130 39 62 | 30.7 (28.0–33.2) | 41.0 | 7.7 |
| yellow lake (reseda, weld) | 189 157 84 | 66.1 (64.3–67.7) | 2.9 | 41.9 |
| alizarin (synthetic, for context) | 193 38 48 | 42.6 (40.5–44.6) | 59.8 | 33.9 |
| lac dye lake (context) | 162 46 51 | 37.6 (35.2–39.7) | 47.5 | 24.7 |
| vermilion (check) | 180 91 90 | 49.4 (47.5–51.3) | 36.1 | 17.4 |

The earlier two-reference estimate gave madder 54.7–57.7. The correction now has its
bounds from physics, and the energy check removed its lighter half.

**Lab-made madder lakes measured as powder [M]:**
- Sattar, "Lake Pigments and Thin-Layer Chromatography of Natural Dyes", *J. Chem. Educ.*
  101 (2024) 4983–4988: madder (Rubia tinctorum, Kremer 37201) dye from dyed silk, laked
  with alum. L\*a\*b\* 66.1 ± 0.4 / 25.5 ± 0.3 / 9.4 ± 0.3 (n = 5). Madder's red reaches into
  the yellow (an orange cast); cochineal's has a blue cast (purple).
- "Pigment Synthesis and Analysis of Color in Art", *J. Chem. Educ.* 2020,
  doi:10.1021/acs.jchemed.9b01093: madder lake 62.6 / 23.9 / 16.7.

**Cross-check.** Their a\* (23.9–25.5) and Sattar's b\* (9.4) match the corrected Kremer
powder (24.6, 8.8). Their L\* is 8–12 higher (ΔE76 11.4–11.7). Both are classroom lakes:
dye from silk shearings or a single extraction, laked once. That carries less dye than a
commercial lake, and a lighter powder is what less dye gives (§6.3). So the lab lakes
bracket the measured span from the light side and don't contradict the FORS powder. The
fit stays on the Kremer powder, the commercial pigment the tube stands for.

**"Seeing red"** (Fonseca et al., *Herit. Sci.* 7:92, 2019) measured madder and
cochineal lakes made to the National Gallery's standardized recipes (Kirby et al.). It
used an ASD FieldSpec 4 at 45/0 with a 4 mm spot, against a 99 % Spectralon. The samples
were paint-outs in four binders, not powders. It confirms the band positions the fit
relies on: madder λ_abs,1 500–509 nm with a shoulder at 538–548 nm; alumina-substrate
cochineal lakes at 528–531 and 557–561 nm. The madder lake in gum arabic (M-Std-Al ex.,
Fig. 1b) reflects about 0.14 at 500 nm and 0.75 at 700 nm. CHSOS's madder in gum arabic,
by the probe, reads 0.03 at 510 nm and 0.68 at 700 nm. That is a deeper swatch, but
these are different lakes and film thicknesses, so it checks the shape, not the level.

No measured powder was found for a fuchsine (aniline, magenta) lake. CHSOS has a
fuchsine dye (Basic Red 9), not a lake.

### 6.2 The engine's rule (switch `--@ dry-lakes`)

For a lake, K_dry(λ) = a · K_oil(λ), and S_dry is the rule for n < 1.9 at the substrate's
1.57. The factor a is the dye loading the oil tables leave unidentified. It is fitted per
lake so the pure dry color matches the corrected powder's L\*a\*b\* [C]:

| Lake | a (earlier) | ΔE76 to powder after (before) | Range of a over the correction's bounds |
|---|---|---|---|
| rose madder | **35.7** (31.9) | 5.0 (38.9) | 31.5–40.8 |
| carmine lake | **3.01** (2.57) | 9.9 (13.5) | 1.9–4.1 |
| yellow lake | **12.2** (10.6) | 3.5 (29.2) | 10.6–14.0 |
| magenta | **6.26** from dye chemistry, K/S 5.7 at 543 nm (§6.3; was yellow lake's 12.2, an estimate) | no powder | 5.35–6.34 over 3–20 wt % dye |

The physical correction moved the loadings up 12–17 %, because the energy check removed
the lighter reference. Madder still needs about thirty-five times the absorption the oil
table gives it, however the powder is corrected. Magenta has no powder, so its loading
is set by its dye (§6.3): a fuchsine lake's K/S at 543 nm over the engine's K/S there
with no loading.
Carmine's fit stops at ΔE 9.9 because of its oil K's own shape: b\* 13.9 against 7.7.

A flat substrate S (lead white's mean, not its spectrum) fits madder and yellow lake as
well, but carmine only to ΔE 16. Revigo's carmine K and lead-white S share a dip near
510 nm (the fitting's, not the pigments'), and dividing by the white-shaped S cancels
it. So the rule keeps that shape.

Before → after, engine 9, sRGB, pure and 1:1 by volume with lead white (the same in every
box that has the tube):

| Tube (boxes) | pure | 1:1 lead white |
|---|---|---|
| rose madder (friedrich, sargent, alma-tadema, giverny, impressionist, seurat, cezanne, every) | 242 218 223 → 174 112 124 | 247 230 233 → 200 143 154 |
| carmine lake (giverny, impressionist, every) | 171 58 87 → 139 22 52 | 190 88 117 → 163 47 77 |
| yellow lake (impressionist, every) | 232 221 175 → 183 157 80 | 239 232 198 → 205 184 112 |
| magenta (sargent, every) | 239 142 211 → 198 71 159 | 248 170 224 → 220 101 185 |
| permanent alizarin (tonn, every) | 213 200 219 → 165 129 153 | 225 217 230 → 218 207 223 |
| green earth | 213 205 184 → 228 225 218 | 233 231 223 → 236 234 229 |
| copper green | 181 223 213 → 197 234 230 | 211 239 234 → 215 241 238 |
| lead-tin yellow | 241 239 188 → 239 235 178 | 243 241 198 → 241 239 192 |
| transparent oxide yellow | 227 196 154 → 222 189 143 | 242 226 208 → 242 225 207 |
| Mars yellow | 244 189 123 → 244 189 123 | 248 202 150 → 248 202 151 |
| Rinmann's green | 112 199 192 → 101 194 187 | 167 224 223 → 165 223 223 |

### 6.3 The loading against dye chemistry

A loading is physical if the K it gives is what the lake's dye can absorb. Per volume of
pigment, K = 2μ_a and S = ¾σ′ (§2.1), and μ_a = ln 10 · ε · c, with c = w ρ / M (dye
mass fraction w, lake density ρ, dye molar mass M). The dye is packed in grains of about
1 µm, so a strongly dyed grain shades its own interior. This package effect is taken
from van de Hulst's absorption efficiency for a sphere,
Q = 1 + 2e^(−x)/x + 2(e^(−x) − 1)/x² with x = μ_a d, which tends to the thin limit for
small x. S is the Mie value for n 1.57 at 1 µm (0.71 µm⁻¹, table 2.2) times a packing
factor for dependent scattering, computed below (0.968–0.972). The inputs are:
- **madder**: alizarin, M 240.2. ε of its Al chelate at 510 nm is 5×10³–1.7×10⁴ L mol⁻¹ cm⁻¹.
  The upper value is alizarin red S–Al at 510 nm (aluminium spectrophotometry with PVP,
  1.70×10⁴). The lower is neutral alizarin's visible band, log ε ≈ 3.7 [E].
- **carmine**: carminic acid, M 492.4. ε is 6.84×10³ L mol⁻¹ cm⁻¹ at 494 nm, from
  E¹%₁cm = 139 in dilute HCl (EU Reg. 231/2012, E 120), up to 1.0×10⁴ for the Al
  chelate's bathochromic band [E].
- ρ 2.0–2.4 g/cm³ (gibbsite 2.42, less organic).
- dye content: lakes carry a few to about 20 wt % dye. Cochineal itself holds 10–19 wt %
  carminic acid (Kirby, Spring & Higgitt, *NG Tech. Bull.* 26, 2005, and 28, 2007), and
  carmine proper, the Al–Ca chelate, is ≥ 50 % carminic acid (E 120).

**The packing factor, computed [A].** A powder's grains sit close, so the waves they
scatter interfere and S per grain falls. In the quasi-crystalline approximation the
transport scattering of a grain in the powder is its Mie phase function, weighted by
(1 − cos θ), times the structure factor S(q) of the grains' arrangement, at
q = 2k sin(θ/2). For hard spheres S(q) is the Percus–Yevick closed form (Wertheim 1963;
Thiele 1963). `dry_lake_fit.py` computes it for gibbsite (n 1.57) in air at 545 nm, over
table 2.2's lognormal (σ_ln 0.5) about 1 µm, at a loose powder's 40–55 % solids by volume.
Its Mie code gives σ′ = 0.706 µm⁻¹ at 1 µm, the table's 0.71.
- φ 0.40 → 0.972, 0.45 → 0.970, 0.50 → 0.968, 0.55 → 0.969.
- The factor is near 1 because at x = πd/λ ≈ 5.8 the grain scatters forward. S(q) is
  small only for q d below about 5, angles under about 25°, which the (1 − cos θ) weight
  hardly counts.
- Finer grains feel it more: 0.87 at 0.5 µm (lognormal, φ 0.45), and 0.61 for a single
  0.5 µm size. §2.3's 0.5–0.7 for sub-micron pigments is of that order.
- The estimate of 0.5–1 used before is replaced by 0.968–0.972. The audit expected 0.6–0.9.
  It took x = π d n/λ ≈ 9, but x is set by the medium's index, air's 1, not the grain's.

K/S at the band [C]:

| | madder, 510 nm | carmine, 530 nm |
|---|---|---|
| corrected powder (bounds) | 2.42 (2.13–2.76) | 15.9 (11.5–25.4) |
| engine before (oil K) | 0.112 | 5.56 |
| engine with the loading | 4.05 | 16.9 |
| dye chemistry, 3 wt % | 1.0–3.0 | 0.69–1.2 |
| dye chemistry, 20 wt % | 4.0–5.6 | 3.2–4.3 |
| dye chemistry, pure dye | 5.7–5.8 | 5.5–5.7 |
| **dye content the powder needs** | **2.1–8.8 wt %** | **beyond pure dye at d = 1 µm** |
| dye content the oil K alone gives | 0.08–0.3 wt % | ≥ 62 wt % |
| dye content the loading gives | 5–21 wt % | beyond pure dye |

**Madder: consistent.** The powder's K/S needs 2–9 wt % alizarin on the alumina, a
normal madder lake. The loaded engine needs 5–21 wt %, which is still a lake. The oil K
alone would mean 0.08–0.3 wt %, a hundredth of any lake. That is the pale stick this
switch fixes. The ×36 is the oil table's missing dye, not a fudge.

**Carmine: K/S bounded only from below.** At 530 nm the corrected powder reflects about
0.03, under the black anchor itself, so its K/S (11–25) is fixed by the floor more than
by the dye. Even pure dye in 1 µm grains reaches only K/S ≈ 5.7, because the package
effect caps it. Finer grains lift that cap but scatter more (σ′ 2.7 µm⁻¹ at 0.3 µm), so
they gain little. What the numbers do say: Kremer's "carmine lake" is dark as a near-pure
carmine is (≥ 50 % chelate), not as a 3 % lake is, and the engine's oil K already sits
there (≥ 62 wt %). The loading of 3 is a small step and is fixed by the red side of the
spectrum, which the floor doesn't touch.

Yellow lake: no ε for the luteolin–Al chelate in the visible was found, so it is not
checked.

**Magenta: the loading from its dye [B/A].** The tube is "fuchsine (aniline) lake on
alumina" (`palette.rs`), and no fuchsine-lake powder was found (§6.1). Its loading is set
so the pure lake has the K/S its dye gives at its band:
- **Dye.** Basic fuchsin, rosaniline hydrochloride (CAS 632-99-5, M 337.8 g/mol):
  ε = 8.06×10⁴ L mol⁻¹ cm⁻¹ at λmax 543 nm in PBS (PhotochemCAD L04; Taniguchi & Lindsey,
  *Photochem. Photobiol.* 94 (2018) 290, Agilent 8453, Alfa Aesar A12952). A practical-grade
  dye is not pure, so the true ε may be up to ~15 % higher. That doesn't matter here, below.
  HiMedia's specific absorbance (7.97–9.16×10⁴, quoted by the audit) could not be reached
  and is not used.
- **Formula.** The same as above: μ_a = ln 10 · ε · w ρ / M, K = 2 μ_a × the package effect
  of a 1 µm grain, S = ¾ σ′ × packing (σ′ 0.712 µm⁻¹ at n 1.57; packing 0.968–0.972; ρ 2.0–2.4).
  This assumes fuchsine keeps its solution band on alumina. As a cationic dye it is
  precipitated, not chelated, so no large shift is expected.
- **Result, K/S at 543 nm:**

  | dye content | 0.5 wt % | 1 | 3 | 5 | 10 | 20 | pure |
  |---|---|---|---|---|---|---|---|
  | μ_a d | 0.6 | 1.2 | 3.6 | 6.0 | 12 | 24 | 121 |
  | K/S | 1.7–2.0 | 2.9–3.3 | 4.9–5.1 | 5.4–5.5 | 5.7 | 5.8 | 5.8 |

- **The dye is so strong that a grain is opaque from about 3 wt %.** There K/S stops
  depending on ε or dye content. It reaches the 1 µm grain's package limit,
  K/S = 4/(d σ′ p) = 5.79.
  - Over grain sizes: 6.8 at 0.3 µm, 5.0 at 0.5–0.7 µm and 5.85 at 1 µm (lognormals,
    with each size's own σ′ and packing).
  - So K/S is 5–7 for any normal lake of any fineness.
- **Taken: K/S 5.71**, at 10 wt % (bounds 4.9–5.8 over 3–20 wt % at 1 µm).
  - The engine's dry K/S for magenta at 543 nm with no loading is 0.912. That is linear
    between the 540 and 550 nm bands, with the substrate's S at n 1.57.
  - **The loading is therefore 6.26** (5.35–6.34 over 3–20 wt %).
  - Yellow lake's 12.2, used before, gave K/S 11.1. That is twice what fuchsine in a 1 µm
    grain can absorb.
- **Pure dry magenta:** 175 50 134 (L\*a\*b\* 43.0/58.4/−19.2 at 12.2) → **198 71 159**
  (50.8/59.5/−21.7). 1:1 with lead white: 220 101 185.
- **Estimate left:** the dye content (10 wt %, taken as a lake's middle). It moves K/S by
  only −15 % at 3 wt % and +1 % at 20 wt %.

### 6.4 Pigments whose index the table had wrong

`dry::refractive_index` falls back to 1.9 for any tube it doesn't list. Permanent
alizarin it lists as a lake. With the switch, `dry::measured_index` gives these their
measured index:

| Tube | Pigment | Table | Measured (source) |
|---|---|---|---|
| permanent alizarin | quinacridone violet PV19 (catalog: "a quinacridone"; data Kremer PV19) | 1.6 | **2.03**: 2.02–2.04 (CAMEO "Quinacridone dyes"; Natural Pigments; β 2.02, γ 2.04) |
| green earth | celadonite, glauconite | 1.9 | **1.62** (Gettens & Stout 1966, via CAMEO "Glauconite"; ranges 1.59–1.66) |
| copper green | verdigris | 1.9 | **1.545**: α 1.53, γ 1.56 (CAMEO "Verdigris"; *Pigments through the Ages*) |
| lead-tin yellow | Pb₂SnO₄ (type I) | 1.9 | **2.30**: 2.29/2.31 (CAMEO "Lead-tin yellow"; Natural Pigments 2.29–2.32) |
| transparent oxide yellow | goethite (synthetic) | 1.9 | **2.35**: α 2.26, β 2.39, γ 2.40 (SSHADE; Natural Pigments) |
| Mars yellow | goethite (synthetic, PY42 α-FeOOH) | 2.2 | **2.35**: the same mineral. Handbook of Mineralogy: α 2.260–2.275, β 2.393–2.409, γ 2.398–2.515; the mean of the lower ends is 2.35. Natural Pigments' datasheet gives 2.42 for its Mars yellow, the upper bound. |
| Rinmann's green | cobalt in a zinc oxide host | 1.9 (fallback) | **2.02**: zincite nω 2.013, nε 2.029 (Handbook of Mineralogy; O'Donoghue, *Gems*), 2.018 averaged over orientation, as the table's zinc white (2.02). Cobalt (CoO, n 2.33) can only raise it a little. The audit proposed 2.0 (CAMEO 2.00–2.02). |

Left as it is: bitumen (not a particulate pigment; no reliable index).

**The check against powders** (CHSOS powders, corrected as above; dry engine
L\*a\*b\* before → after; ΔE76 to the powder) [M/C]:

| Tube | Powder | Engine before → after | ΔE before → after |
|---|---|---|---|
| lead-tin yellow (powder: lead-tin yellow I) | 86.6/−8.8/36.1 | 93.4/−6.9/24.7 → 92.2/−7.6/28.2 | 13.3 → **9.8** |
| green earth | 56.1/−1.9/11.3 | 82.4/−0.5/11.2 → 89.6/−0.3/3.9 | 26.3 → 34.4 |
| copper green (powder: verdigris) | 33.6/−27.2/−13.6 | 85.8/−15.9/0.5 → 90.1/−12.5/−2.2 | 55.2 → 59.4 |
| permanent alizarin | no PV19 powder spectrum found | 82.1/7.8/−7.7 → 58.2/17.8/−7.6 | — |
| transparent oxide yellow | none | 80.9/4.7/25.3 → 78.5/5.4/27.6 | — |
| Mars yellow | none | 80.2/11.9/40.9 → 80.3/11.8/40.7 | — |
| Rinmann's green | none (CHSOS "cobalt green" is not identified as Co–Zn, and was not used) | 74.8/−27.7/−4.7 → 72.9/−29.4/−4.8 | — |

Lead-tin yellow comes nearer its powder. Green earth and verdigris move further away,
and they were already 26 and 55 ΔE off. Their index is now right, and what remains is
their K per volume. Both oil tables are made the way the lakes' were, and dry each
powder is 30–50 L\* deeper than the engine. Coarse grains add to it: green earth and
verdigris run 1–30 µm, and σ′ at 3 µm is a quarter of its 1 µm value (table 2.2). That
is the lakes' problem again, for a later PR: a loading per pigment fitted to these same
powders, or a size-aware S. PV19 has no powder spectrum. CHSOS shows a PV19 spectrum
only as an image (Pigments Checker, Kremer 23402), and no CSV was found. Its pure dry
color, L\* 58 and a dusky violet-rose, is the high-index rule over its estimated oil
hiding. Its tint with white barely moves (225 217 230 → 218 207 223), because its K per
volume is unidentified, as madder's was. Mars yellow barely moves: its Mie air/oil ratio
is 1.67 at 2.2 and 1.70 at 2.35.

### 6.5 Rose madder in oil (not changed here)

Engine 9's oil table gives rose madder a K/S whose R∞ is sRGB 205 141 142 (L\* 65; the
engine's decoded masstone is 211 126 125). Its 1:1 with lead white in oil is 240 212 217
(L\* 87), a near-white. The cause is the one in §6.2:
its K comes from a gum swatch read as an assumed 100 µm film. A physically grounded fix
for a later PR [C]:
- **K:** absorption per volume is the dye's in any medium (§2.4), so oil K takes the
  dry loading, ×35.7. This is checked against dye chemistry in §6.3.
- **S:** the substrate's S in oil is the dry S divided by the Mie air/oil ratio at n 1.57
  (×43, table 2.2). That gives 0.107 per coat at 550 nm against the table's 0.159. They
  agree to 1.5×, so the oil S is roughly physical and stays.
- **Result:** masstone R∞ 80 29 30 (L\*a\*b\* 18.6/24.2/11.1), a deep crimson-brown as a
  thick madder glaze is. 1:1 with lead white 163 100 110 (49.4/26.9/5.2). With the Mie S
  as well, the masstone is 48 18 23 and the tint is unchanged (160 98 109).
- **Check needed first:** a measured madder-in-oil tint with lead white. CHSOS's
  linseed-oil swatches are on paper, not opaque, and come from another sphere session,
  so they can't pin it. The fix would also move every oil painting's madder, so it needs
  its own switch. Permanent alizarin has the same weak K in oil.

### 6.6 Green earth and verdigris: for the later oil-K PR (not changed here)

With their measured index (§6.4) they move further from their powders:
- green earth: ΔE 26 → 34;
- verdigris: ΔE 55 → 59.

Their oil K per volume is too small, as the lakes' was. The estimates audit (row L11,
class C) proposes, for that PR:
- **A loading on each one's oil K, fitted exactly as for the lakes,** with
  `dry_lake_fit.py`'s `fit()`, to the corrected CHSOS powders already in this set:
  green earth 56.1/−1.9/11.3, verdigris 33.6/−27.2/−13.6.
  - The script prints what that would give (not applied): green earth ×31.4, ΔE 0.5;
    verdigris ×189, ΔE 10.0.
  - Verdigris's very large factor, and its leftover ΔE, say its S is too high as well as
    its K too low.
- **Grain size is the other half.** Both run 1–30 µm, and σ′ at 3 µm is a quarter of its
  1 µm value (table 2.2). The audit marks a size-aware S (table 2.2's σ′ against d) as
  derivable [A]. Fitting K alone at 1 µm's S would hide a size effect in the loading.
  - The later PR should fit S from a measured grain size first, then K.
  - Measured size distributions (SEM or laser diffraction) of Kremer's green earth and
    verdigris are the inputs to find.
- **Its switch.** It moves every new painting's green earth and copper green, so it needs
  its own switch, or a revision of `--@ dry-lakes` before that line ships.

Sources for this section: Cosentino 2014, *e-conservation* 2:54–65 (CHSOS copy of the
paper; data download page
https://chsopensource.org/new-free-pigments-fors-spectroscopy-database/) ; Sattar 2024,
https://doi.org/10.1021/acs.jchemed.4c00752 ; J. Chem. Educ. 2020,
https://doi.org/10.1021/acs.jchemed.9b01093 ; Fonseca, Schmidt Patterson, Ganio,
MacLennan & Trentelman, "Seeing red", *Herit. Sci.* 7:92 (2019),
https://doi.org/10.1186/s40494-019-0335-1 ; Kirby, Spring & Higgitt, *NG Tech. Bull.*
26 (2005) 71–87 and 28 (2007) 69–95 (nationalgallery.org.uk) ; EU Reg. 231/2012, E 120
carmines ; aluminium–alizarin red S spectrophotometry with PVP (ε 1.70×10⁴ at 510 nm) ;
PhotochemCAD, p-Fuchsin (L04),
https://www.photochemcad.com/databases/common-compounds/arylmethane-dyes/p-fuchsin, and
Taniguchi & Lindsey, *Photochem. Photobiol.* 94 (2018) 290–327 ; Wertheim, *Phys. Rev.
Lett.* 10 (1963) 321 and Thiele, *J. Chem. Phys.* 39 (1963) 474 (Percus–Yevick hard
spheres) ; Bohren & Huffman, *Absorption and Scattering of Light by Small Particles*
(1983), BHMIE ; Anthony, Bideaux, Bladh & Nichols, *Handbook of Mineralogy*: goethite,
https://www.rruff.net/doclib/hom/goethite.pdf, and zincite ; Natural Pigments, Mars yellow
(PY42) dry pigment datasheet ;
Sanjeev et al., size-dependent reflectance of TiO₂, chalcogen.ro/607_Sanjeev.pdf ;
SPIE Photonics Focus May/June 2026, "In pursuit of the perfect black" ; Coatings 2018,
8:171, https://doi.org/10.3390/coatings8050171 ; Heritage 2023, 6:192,
https://www.mdpi.com/2571-9408/6/4/192 ; CAMEO (MFA Boston): Quinacridone dyes,
Glauconite, Verdigris, Lead-tin yellow ; SSHADE, goethite ; CHSOS fuchsine,
https://chsopensource.org/products/pigments-checker/pigments-checker-modern-and-contemporary-art-pigments-list/basic-red-9-fuchsine/ .
Computation: `dry_lake_fit.py` (this folder).

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
