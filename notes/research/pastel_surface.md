# Pastel's surface: the veil, and fixative

The pastel veil switch (`--@ pastel-veil`; paint's `Canvas::with_pastel_veil`, `Cell::bare_bed`, `Canvas::unveiled`) and the fixative index switch (`--@ fixative-index`; `Canvas::with_fixative_index`, `Cell::bind_by_index`, §4). Compiled 2026-11-16 from estimates-audit-2 (C1 and engine-wide fault 4) and charcoal.md §2. [M] measured, [D] derived, [E] estimate.

## 1. No veil over a particle bed [D]

The engine shows a matte surface with a first-surface veil (`canvas::haze`): 4 % of the light, an oil film's Fresnel reflection at n ≈ 1.5, scattered back by a matte surface and added over its color, k(1 − R). That is right for a continuous film whose body color sits under a smooth interface: paint, a varnish.

A bed of particles has no such interface. Mie's scattering of each particle already holds its surface's (facet) reflection, and Kubelka–Munk's R∞ is everything the bed returns. A veil over it counts the particles' surfaces twice. #321 found this for charcoal (vine at Y 0.060, L* 29.5 with the veil, above measured charcoal powder's < 5 %: arXiv 2110.05832) and took the veil off carbon only. The same holds for every pastel: chalk and pigment beds are particle beds too, their dry colors built by the same Mie and Kubelka–Munk (`dry::dry_color`, dry_pigment_optics.md §2). So with the switch the rule is one: **no veil over any open particle bed, loose or fixed** (`Cell::bare_bed`, the bed's share of the pixel and its reflectance as it shows, water's darkening with it); #321's carbon-only rule (`Cell::bare_carbon`) is that rule restricted to carbon, kept for logs without the switch.

What keeps the veil:
- **paint and wet paint** over the pastel: their own film's surface (the cell is sealed, `film`);
- **paper** between the marks, and a sheet laid over the picture (only the pastel it caught is unveiled);
- **graphite and black chalk** (`non_pastel`): graphite's flakes are smooth platelets with a sheen of their own, not a Mie bed; the leads' reflectance (`Lead::flake`) is a seen value, not a bed's R∞.

**Fixative.** A workable fixative's light spray lays about 1.5 µm of resin (charcoal.md §2: Krylon's SDS and coverage). That fills a share of the pores of the pastel under it; where a thin line's pores (porosity 0.55) hold less than the resin, the rest soaks into the sheet's own pores (about half of a 250 µm caliper). The resin's menisci in the pores face every way, so no continuous film forms and the bed stays matte: **a fixed bed is unveiled too**. A continuous resin film would need more resin than the pastel's and the sheet surface's pores together hold, tens of µm, many heavy coats; the easel's `fix()` is a light spray, so it has no such case.

## 2. What it changes [D]

The veil adds k(1 − R) with k = 0.04 (× (1 − gloss)) over the bed's share of the pixel. Over a whole bed:

| pastel | Y | L* unveiled | L* with the veil | ΔL* |
|---|---|---|---|---|
| a whiting tint | 0.80 | 91.7 | 92.0 | 0.4 |
| a pale tint | 0.60 | 81.8 | 82.7 | 0.9 |
| a middle value | 0.30 | 61.7 | 64.0 | 2.3 |
| a dark (umber, ultramarine) | 0.08 | 34.0 | 40.7 | 6.7 |
| a near-black pastel | 0.04 | 23.7 | 33.6 | 10.0 |
| vine charcoal (as before) | 0.021 | 16.0 | 29.5 | 13.4 |

So the pale pastels barely move and the darks deepen a great deal: the veil was washing out exactly the deep darks that make pastel's reputation. Pastel is often described as the most saturated dry medium for this reason: its particles show their own color with no film over them.

## 3. Left, and why

- The paper itself is a bed of fibres, and the veil over it may double-count in the same way. Its color (`Paper::tone`) comes from its pulp's dry pigments and is shown with the veil. Changing that changes every sheet, so it is left to its own question.
- Standing water over wet pastel (`crate::wet_pastel`) is a continuous film while it stands, and glossy; it is treated as part of the bed, as #322 did, with no veil. Its specular reflection goes away from a diffuse viewer, so this is near enough.

## 4. Fixative darkening by each particle's index [D]

**The fault** (estimates-audit-2, fault 3). Fixative divided every pastel's K/S by 1 − w + w/5 over a fixed share w = 0.15: "scattering falls to about a fifth, for particles of n ≈ 2 in resin". By the repository's own Mie that is wrong for n ≈ 2. The ratio S(resin)/S(air), resin poly(n-butyl methacrylate) at n 1.49 (charcoal.md §2), at a pastel's particle size (whiting's d₃₂ 1.7 µm, the size `dry::scatter_in` is tabled at), size-averaged as `scatter_in` is (dry_pigment_mie.py `resin`):

| n | 1.50 | 1.56 | 1.59 | 1.65 | 1.70 | 1.74 | 1.84 | 2.00 | 2.15 | 2.30 | 2.40 | 2.45 | 3.00 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| S_resin/S_air | 0.0005 | 0.027 | 0.056 | 0.143 | 0.238 | 0.304 | 0.449 | 0.492 | 0.557 | 0.600 | 0.610 | 0.605 | 0.719 |

(`dry::scatter_in_resin`.) One fifth is right only near n 1.7. The old rule over-darkened every high-index pigment (lead and zinc whites, the cadmiums and chromes, the earths, vermilion) and under-darkened the low-index ones (ultramarine, Prussian blue, the lakes, and the chalk a tint is let down with).

**The rule.** Over the share of its pores the resin fills, a bed's K and S are those in resin, mixed with those in air by that share, as `CarbonBed::color` does for charcoal:
- a pigment's or chalk's absorption stays and its S falls by `scatter_in_resin` of the cell's particle index (`Cell::index`, the one #322's wet pastel switch records, by volume; carbon's share, recorded at whiting's index, taken out);
- carbon's K and S in resin are its beds' own (`charcoal::resin_ratios`, the mean of vine's and compressed's; K falls ×0.91–0.95, S ×0.63–0.96);
- in a cell of both, K and S are split by each one's own (with the pastel K–M switch: the cell's S, `Cell::s`, and carbon's K and S per µm as it lies, `charcoal::lying_ks`; without it by volume, as #321 did).

So K/S′ = K/S × [1 − w_K,c (1 − f_K,c)] / [(1 − w_S,c) f_S,p + w_S,c f_S,c], with f = 1 − ws + ws × (the ratio in resin). For pure charcoal this is exactly `CarbonBed::color(ws)`, in place of #321's linear scaling of K/S (`WET_KS`), which it approximated; the code has one rule.

**The share wetted** is #321's: the resin a light spray lays (`charcoal::LIGHT_SPRAY_UM`, 1.5 µm: Krylon Workable Fixatif's SDS and its labelled 15–20 ft² a coat, less the estimated half that lands and third of a coat a light spray is) over the pores of the loose pastel under it (porosity 0.55, `wet_pastel::POROSITY`): 0.27 of 10 µm, 0.14 of 20 µm, 0.09 of 30 µm, all of a thin line under 2.7 µm (`charcoal::wet_share`). The old 0.15 was right for 18 µm.

**What it changes** (ΔL* of a fixed bed, old rule against the new one at 20 µm and at 5 µm of loose pastel):

| pastel | n | dry L* | old | new, 20 µm | new, 5 µm |
|---|---|---|---|---|---|
| lead white tint | 2.0 | 93.9 | −0.39 | −0.22 | −1.04 |
| vermilion | 3.0 | 61.7 | −1.80 | −0.55 | −2.35 |
| yellow ochre | 2.1 | 72.9 | −1.46 | −0.74 | −3.43 |
| ultramarine | 1.5 | 57.1 | −1.88 | −2.15 | −11.8 |
| whiting tint | 1.59 | 93.9 | −0.39 | −0.42 | −2.55 |
| bone black | 1.65 | 23.7 | −1.54 | −1.49 | −7.11 |

By this rule the blues and the chalk darken most, and the earths and vermilion barely move. Conservators and practitioners report that fixative darkens pastel and that heavy spraying darkens more ("saturation changes", AIC Paper Conservation Catalog chs. 3 and 23; Fine Art Tutorials, as cited in charcoal.md §2) [P]; no measured ΔE of fixed pastels by pigment was found (estimates-audit-2's searches), so the order by pigment is derived here, not checked against a measurement.

**Left, and why.**
- *The light spray's resin* (1.5 µm): that half of it lands and a light spray is a third of a coat are #321's estimates. Weighing a sprayed sheet would set them.
- *Calcite is birefringent.* Whiting is taken at its mean index, 1.59 (ratio 0.056); averaged over its ω 1.658 and ε 1.486 the ratio is 0.109 (dry_pigment_mie.py `resin`, "calcite"). One index per cell can't carry that, so a whiting-rich tint darkens somewhat too much (a whiting tint's ΔL* at 20 µm is −0.42 by its mean index against −0.40 birefringence-averaged; at 5 µm −2.55 against −2.32).
- *The index is a mean.* A cell's index is the volume mean of what landed (#322's bookkeeping). The ratio is concave over 1.59–2.0, so a mixture's ratio at its mean index is a little high, and it darkens slightly too little. The error is under 0.1 in the ratio for a tint, about 2 % in K/S at a 0.15 share.
- *Finger-moved pastel* keeps no index (#322's tools record none), so a cell that pastel was rubbed into, and that held none before, is taken as whiting.

## Sources

- charcoal.md §2 and charcoal_optics.py (the veil over carbon, the resin, the light spray's resin); estimates-audit-2.md C1 and fault 4; dry_pigment_optics.md §2 (Mie and Kubelka–Munk of dry pigments).
- Measured charcoal powder below 5 % in the visible: arXiv 2110.05832 (as cited in charcoal.md).
