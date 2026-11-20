# Pastel's surface: the veil, and fixative

The pastel veil switch (`--@ pastel-veil`; paint's `Canvas::with_pastel_veil`, `Canvas::unveiled`). Compiled 2026-11-16 from estimates-audit-2 (C1 and engine-wide fault 4) and charcoal.md §2. [M] measured, [D] derived, [E] estimate.

## 1. No veil over a particle bed [D]

The engine shows a matte surface with a first-surface veil (`canvas::haze`): 4 % of the light, an oil film's Fresnel reflection at n ≈ 1.5, scattered back by a matte surface and added over its color, k(1 − R). That is right for a continuous film whose body color sits under a smooth interface: paint, a varnish.

A bed of particles has no such interface. Mie's scattering of each particle already holds its surface's (facet) reflection, and Kubelka–Munk's R∞ is everything the bed returns. A veil over it counts the particles' surfaces twice. #321 found this for charcoal (vine at Y 0.060, L* 29.5 with the veil, above measured charcoal powder's < 5 %: arXiv 2110.05832) and took the veil off carbon only. The same holds for every pastel: chalk and pigment beds are particle beds too, their dry colors built by the same Mie and Kubelka–Munk (`dry::dry_color`, dry_pigment_optics.md §2). So with the switch the rule is one: **no veil over any open particle bed, loose or fixed** (`Canvas::unveiled`, over the drawing's share of the pixel at its reflectance as it shows, water's darkening with it); #321's carbon-only rule (`Cell::bare_carbon`) is that rule restricted to carbon, kept for logs without the switch.

What keeps the veil: **paint and wet paint** over the pastel, their own film's surface (the cell is sealed, `film`).

The switch first left the veil over the paper and over graphite and black chalk. paper_veil.md took it off them too: a sheet's tone is its whole reflectance, a size film follows the fibres and adds no surface, and the veil comes back over paper only as a film fills its pores; black chalk is a bed of clay and carbon crumbs; graphite is opaque, all its light its flakes' own surface reflection, which `Lead::flake` already is. With the switch only a film is veiled.

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

- The paper itself is a bed of fibres, and the veil over it double-counted in the same way: answered in paper_veil.md (no veil over bare paper with the switch).
- Standing water over wet pastel (`crate::wet_pastel`) is a continuous film while it stands, and glossy; it is treated as part of the bed, as #322 did, with no veil. Its specular reflection goes away from a diffuse viewer, so this is near enough.

## Sources

- charcoal.md §2 and charcoal_optics.py (the veil over carbon, the resin, the light spray's resin); estimates-audit-2.md C1 and fault 4; dry_pigment_optics.md §2 (Mie and Kubelka–Munk of dry pigments).
- Measured charcoal powder below 5 % in the visible: arXiv 2110.05832 (as cited in charcoal.md).
