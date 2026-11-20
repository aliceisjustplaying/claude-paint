# How pastels mix in the tooth

The pastel K–M switch (`--@ pastel-km-mix`; paint's `Canvas::with_pastel_km_mix`, `Cell::s`, `Cell::add_grains`). Compiled 2026-11-16 from estimates-audit-2's engine-wide fault 1 and charcoal.md §6. [M] measured, [D] derived from measured values, [E] an estimate with the reason it is one.

## 1. The fault

A stick's own color is built from its tubes by Kubelka–Munk (`dry::dry_color`), but before this switch two sticks' loose pastel in the same tooth was mixed by averaging its reflectance by volume (`Cell::add_grains`). Averaging reflectance is right only where the two lie side by side (an area mix) or the eye sees one through gaps in the other. Particles rubbed together are an intimate mixture, and intimate mixtures darken far beyond their share:

- "A few percent of dark grains can drastically reduce the reflectance, much more than their weight fraction"; in an intimate mixture "the darker material dominates", in an areal mixture the brighter (Clark 1983, montmorillonite with charcoal and carbon grains, measured 0.4–3 µm; restated with its Figure 20 in Clark 1999) [M]. Clark 1983's tabulated values are not freely available (no full text found), so the engine is checked against the trend, not a number.
- Charcoal *spread over* a soil mixes linearly (USDA Forest Service, *Spectral analysis of charcoal on soils*) [M]: the side-by-side case, which the engine keeps for coverage below 1 (the paper between the particles).

## 2. Intimate mixing [D]

In Kubelka–Munk, K and S of a mixture add by volume, so

  (K/S)_mix = Σ vᵢ Sᵢ (K/S)ᵢ / Σ vᵢ Sᵢ,  R∞ = 1 + K/S − √((K/S)² + 2 K/S),

each component's K/S from its own R∞. So a cell needs one more number besides its color: the loose layer's scattering S per µm (as it lies), the volume-weighted mean of what was laid (`Cell::s`, an f32). Pastel's S is nearly flat across the visible (chalk and pigment in air scatter by index contrast, not color), and so is carbon's, so one gray value serves.

Where each stick's S comes from:

- **A pastel stick:** the dry scattering `dry_color` already gives its parts relative to lead white dry, put on lead white's absolute scale, S = 0.75 σ′ with σ′ = 0.88 µm⁻¹ (`mie_air` at n 2.0, d 1 µm; S = 0.75 σs(1 − g), dry_pigment_optics.md §2.1): `dry::dry_scatter` and, for engine 9's spectral tubes, `dry::spectral_scatter` (the mean over the spectrum). Times the stick's solids, 1 − ε = 0.45 (`wet_pastel::POROSITY`), it is S per µm of stick (`Stick::scatter`). Whiting alone, 0.75 × 0.331 = 0.248 µm⁻¹ per particle volume, stands in for a stick with none recorded.
- **Vine charcoal:** `CarbonBed` S (Mie of charred cellulose, charcoal_optics.py), its channel mean 0.0240 µm⁻¹ per splinter volume, times the stick's char share, 0.30/1.44 = 0.21 (apparent over wall density, charcoal.md §7): 0.0050 per µm of stick.
- **Compressed charcoal:** `COMPRESSED_CHARCOAL` counts K and S per crumb of the compact (its pores within the crumb), so all of the stick: 0.0179.
- In the tooth a stick's worn volume lies at `PARTICLE_UM / cover_um` of itself (`Grains::lies`), so its S per µm as it lies is `scatter × cover_um / PARTICLE_UM` (`Stick::lying_scatter`): pastel 0.45 S_particle, vine 0.49, compressed 0.55, the same packing of about one half.

The intimate paths are the finger (`Canvas::rub`), the crumbs under a stick's face (`pastel::lay_crumbs_of`; with the crumb layer switch they lie over the loose pastel instead, §3a), a dry brush, a knife and a puff (`sheet.rs`), a wet brush's paste (`wet_pastel::wash_pastel`) and sticks ground together into a brush's paste (the easel's `b:dip`). Each tool carries its load's S (`pastel::Carried::mixed`, `WashLoad::paste_s`, `Held::paste_s`), so what it picks up mixes on it by Kubelka–Munk too. A paste that packs as it dries (`wet_pastel::dried_share`) keeps its particles, so its S per volume rises by the same factor.

**Check against the audit's figure** (a yellow pastel (0.75, 0.6, 0.1) whose particles scatter as chalk filler, S = 0.75 × 0.73 µm⁻¹, with vine charcoal; charcoal.md §6):

| charcoal by volume | averaged | Kubelka–Munk |
|---|---|---|
| 2 % | (0.735, 0.588, 0.098) | (0.704, 0.578, 0.100) |
| 5 % | (0.714, 0.571, 0.096) | (0.649, 0.549, 0.099) |
| 10 % | (0.677, 0.542, 0.092) | (0.578, 0.504, 0.098) |

The engine with the switch gives the right-hand column within 2.5 × 10⁻³ (the table takes carbon's S per channel, the cell its gray mean; `graphite::tests::pastel_mixed_in_the_tooth_darkens_as_powders_do`). The loss from the yellow is 1.7–3.1× the average's in red and green, and about none in blue, where the yellow already absorbs: Clark's "darkest substance dominates at a given wavelength".

## 3. A stroke over loose pastel: layering [D]

A stick's face deposit lands on top of what is in the tooth: a layer, not a mixture. Kubelka's two-layer formula, R = R₁ + T₁² R₂ / (1 − R₁ R₂), with R₁ and T₁ of the new layer from Kubelka–Munk's hyperbolic solution (a = 1 + K/S, b = √(a² − 1); R₁ = sinh(bSX)/(a sinh bSX + b cosh bSX), T₁ = b/(a sinh bSX + b cosh bSX)) (`dry::km_layer`; checked against Kubelka's exponential closed form and the non-absorbing limit [R₂ + SX(1 − R₂)]/[1 + SX(1 − R₂)]).

The tooth's layers cover by area as `Cell::reflectance` counts them, l = 1 − e^(−v/6 µm). New particles fall uniformly, so of the loose layer's area after the stroke:
- l_old (1 − q) is the old pastel, still uncovered;
- l_old q is the new particles over the old, by the two-layer formula, their thickness where they cover being v/q µm;
- (1 − l_old) q is the new particles beside the old: their own color, an area mix.

So pastel over loose charcoal keeps more of its color than the same pastel rubbed in: a single stroke's 6 µm of chalk (SX ≈ 0.7) lets the charcoal show; a heavy one hides it. Over fixed pastel nothing changes (the fixed crust is its own stratum, as before).

## 3a. A stroke's crumbs: a layer too (the crumb layer switch) [D]

This switch (`--@ pastel-crumb-layer`) mixed a stroke's crumbs (`pastel::lay_crumbs_of`) into the loose pastel intimately, with the finger's paste. But the crumbs settle in the bed of pixels under the face *not* in contact with it (0 < gap < a crumb's size): the face is above them, so nothing shears them into the pastel already there. They lie over it, as the face deposit does, and need the same two-layer rule. The crumbs are 1 − 1/6.6 ≈ 85 % of what a stroke lays (`SHED`), so mixing them made a light over a loose dark all but invisible: three passes of a near-black (3.3 µm loose in the tooth), then four passes of a pale stick laying 1.4 µm more, moved the band's Y from 0.387 to 0.382 (darker), and over the same passage smudged from 0.322 to 0.311. With the crumbs layered: 0.387 → 0.442 and 0.322 → 0.419. Pastel over loose charcoal still comes out greyer than over fixed (the layer is thin and leaves gaps). Details and the other behaviours checked with it: pastel_layering.md.

## 4. Memory

`Cell` grows from 96 to 100 bytes (one f32). A drawing exists only on a canvas drawn on, one cell per window pixel: 4 bytes more per pixel there, 1 MB on a 512 × 512 window. Saved in the checkpoint's own section (`PASTEL_KM`, "PKMX"), written only with the switch, and in a resting canvas's spill.

## 5. Left as they are, and why

- **A sheet laid over the picture** (`sheet.rs`, `lay_sheet`) catches pastel in its own arrays, still mixed by reflectance: it keeps no S per pixel. It is a mask, rarely drawn on twice.
- **Loose to bound and back** (fixative, the gum setting, a scalpel freeing bound pastel) moves pastel between the loose and bound strata by volume, as before: those are the same particles changing state in place, not new ones mixing. A freed bound share takes the loose layer's S.
- **A layered stroke later rubbed** is taken as one loose layer of its seen color: the cell keeps one color and one S, not the stack. The finger then mixes that color by Kubelka–Munk, which is slightly paler than mixing the two components afresh would be. Exact would need the stack's K/S per channel per cell (12 more bytes).
- **Gray S:** carbon's S varies ±2 % across the channels and the spectral tubes' a little more; a gray mean is used.
- **Pastel's porosity 0.55** [D, bounds 0.50–0.61] and **vine's char share 0.21** (0.19–0.24) come from the stick densities in wet_pastel.md §1 and charcoal.md §7. Over that range they change a stick's S by ±10 %, and a mixture's color less.

## Sources

- Clark, R. N. 1983, "Spectral properties of mixtures of montmorillonite and dark carbon grains", *JGR* 88(B12):10635–10644, doi 10.1029/JB088iB12p10635.
- Clark, R. N. 1999, *Spectroscopy of Rocks and Minerals, and Principles of Spectroscopy*, clarkvision.com (Figures 19–20: areal against intimate mixtures).
- USDA Forest Service, *Spectral analysis of charcoal on soils: implications for wildland fire severity mapping methods* (treesearch 37497): linear mixing for charcoal spread over soil.
- Kubelka, P. & Munk, F. 1931, *Z. Tech. Phys.* 12:593; Kubelka, P. 1948, "New contributions to the optics of intensely light-scattering materials, part I", *JOSA* 38:448 (the hyperbolic solution and the layer over a ground).
- In this repository: estimates-audit-2.md (engine-wide fault 1), notes/research/charcoal.md §6–7, charcoal_optics.py §4, dry_pigment_optics.md §2.1, wet_pastel.md §1.
