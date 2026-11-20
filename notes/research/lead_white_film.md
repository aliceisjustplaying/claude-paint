# Lead white's hiding and the tubes' own pigment (`--@ tube-volume`)

Computation: `lead_white_film.py` (this folder). The switch: `Palette::tube_volume`,
`mixer::revigo_oil`, `mixer::REVIGO_FILM_UM`, the easel's `--@ tube-volume`. [M] measured,
[C] computed from measurements, [E] estimate.

## 1. The fault

A painter mixing an oil (cassatt8, 2026-11-18) found tints with lead white far too strong:
7 % of yellow ochre a peach, the sand a saturated orange. Measured on the engine (integration
build of that day, impressionist box, a knifed tiny scene, parts by volume with lead white,
L\*a\*b\* as the plain render shows it):

| tube | 1 % | 3 % | 7 % | 15 % | 30 % |
|---|---|---|---|---|---|
| yellow ochre (oil-earths) | 90.4/3.6/18.2 | 86.1/7.3/27.9 | 81.2/10.3/36.4 | 76.3/12.4/43.8 | 69.6/14.9/51.3 |
| cobalt blue | 82.1/−0.7/−18.8 | 73.5/0.2/−31.0 | 66.4/1.7/−40.1 | 58.3/4.7/−48.4 | 49.7/9.0/−55.4 |
| ultramarine | 85.3/−0.3/−13.4 | 76.3/2.3/−25.6 | 68.6/5.7/−35.5 | 58.8/11.8/−46.5 | 48.3/19.8/−56.6 |
| burnt sienna (oil-earths) | 83.0/8.5/6.5 | 74.7/11.4/8.8 | 67.8/13.9/10.5 | 56.2/17.5/14.1 | 47.3/19.7/16.8 |
| vermilion | 82.1/16.5/7.4 | 72.3/26.2/11.7 | 66.8/30.4/14.5 | 59.4/36.5/18.1 | 50.0/43.8/24.3 |

The open question two earlier builders left was lead white's scattering S per volume: if lead
white scatters twice what the engine has, every tint with it is twice too strong. It is not
that simple, because the measured tubes' tints are measured.

## 2. Cobalt, ultramarine and vermilion tint as Revigo measured [M/C]

Engine 9's measured tubes are the Revigo paints (Geldof et al. 2018), fitted to their
measured tints. Their tints by weight with zinc white and lead white, converted to volume
with the engine's densities, against the engine, 100 µm films over Revigo's white field
(D65, specular excluded):

| sample | by weight / by volume | measured | engine |
|---|---|---|---|
| 08F lead white + cobalt | 89.9/10.4 → 77.3/22.7 % | 55.2/3.5/−46.7 | 52.9/6.9/−53.2 |
| 12B zinc white + cobalt | 90/10 → 87.2/12.8 % | 56.3/3.3/−47.8 | 56.9/2.9/−49.1 |
| 11B zinc white + ultramarine | 89.3/10.7 → 86.0/14.0 % | 55.6/8.6/−43.7 | 56.5/10.9/−48.2 |
| 06B zinc white + vermilion | 90/10 → 95.0/5.0 % | 66.8/32.5/10.8 | 66.4/31.1/10.4 |

So a tint of these with white is as strong as the measured paints' (the ratio of a tube's K
to white's S needs no absolute S). Two things were still wrong, and neither was the S per
volume the earths' tints rest on.

## 3. A tube is its box's tube paint, not Revigo's [M/C]

A pile is parts by volume of the box's tubes. A Revigo table is per volume of Revigo's paint,
which holds its own share of pigment. The box's tubes hold theirs: the impressionist box's
oil by weight is measured (Church's 1901 analyses and the French figures,
pigment_oil.md §4–5), Revigo's from its table 5 where measured (zinc white 0.37, bone black
0.48, cobalt 0.51, ultramarine 0.41, Prussian blue 0.56), else the hand-ground rule
1.45·OA/(100 + 1.45·OA) [E, as `scripts/pigment_densities.json` says]. At one pigment
density, a volume of the box's tube holds (its pigment share)/(Revigo's) of Revigo's pigment:

| tube | box oil, pigment share | Revigo's | weight |
|---|---|---|---|
| lead white | 0.13, 0.478 | 0.127, 0.485 | 0.985 |
| zinc white | 0.19, 0.412 | 0.370, 0.219 | 1.884 |
| cobalt blue | 0.55, 0.153 | 0.510, 0.175 | 0.874 |
| ultramarine | 0.34, 0.434 | 0.410, 0.363 | 1.197 |
| vermilion | 0.22, 0.289 | 0.127, 0.442 | 0.655 |
| viridian | 0.50, 0.225 | 0.537, 0.200 | 1.124 |
| chrome yellow | 0.33, 0.239 | 0.225, 0.348 | 0.687 |
| bone black | 0.55, 0.224 | 0.480, 0.276 | 0.810 |

The French tubes were oilier than Revigo's hand-ground reconstructions in the oil-hungry
pigments and the heavy ones (vermilion), and Revigo's zinc white was very oily. Tints with
the weights (R∞ seen outside, by volume with lead white), before / after:

| tube | 3 % | 7 % | 15 % |
|---|---|---|---|
| cobalt blue | 74.1/0.3/−30.2 / 75.1/0.1/−28.9 | 66.2/2.1/−40.1 / 67.3/1.8/−38.8 | 57.8/5.1/−49.0 / 59.0/4.6/−47.9 |
| ultramarine | 76.6/2.2/−25.5 / 74.9/2.9/−27.6 | 68.4/5.9/−35.8 / 66.3/7.0/−38.3 | 59.0/11.5/−46.4 / 56.7/13.2/−48.8 |
| vermilion | 73.8/24.2/11.0 / 77.1/21.2/9.6 | 65.8/31.1/14.8 / 69.6/27.9/13.0 | 57.9/37.6/19.0 / 61.7/34.6/16.9 |

Cobalt blue at 7 % by volume in lead white is about 3 % by weight: Revigo's own ladder puts
10 % by weight at L\* 55. It is a strong tint because the measured paint is. The earths enter
on a different footing: #351's `--@ oil-earths` slots, which its `--@ earth-tints` follow-up
puts on the same tube-paint footing.

## 4. How thick the Revigo films were

Revigo drew each paint "using a block operator to create a relatively uniform layer of 100 µm
thickness" (Geldof et al. 2018). That is the applicator's gap. Such an applicator lays a wet
film of about half its gap (BYK-Gardner's guidance) [E]. Nothing was gauged. Engine 9 fitted
every coefficient per that nominal 100 µm, so the thickness sets how much every film hides
per micrometre, not the tints.

**Lead white's hiding at gauged thickness [M].** Bruce (NBS Technologic Paper 306, 1926,
tables 2–3) spread white lead paint 5B (68.3 % white lead, 30.6 % non-volatile vehicle,
1.1 % thinner, by weight) on a half-black, half-white plate (0.053 and 0.75 of MgCO₃), and
read the contrast ratio with a Martens photometer against the dry film's thickness by an
Ames dial (±2 µm), 18 films:

| dry film, µm | 14 | 36 | 49 | 60 | 82 | 91 | 113 | 134 | 153 |
|---|---|---|---|---|---|---|---|---|---|
| measured | 0.494 | 0.782 | 0.868 | 0.908 | 0.947 | 0.967 | 0.971 | 0.985 | 0.986 |
| engine, Revigo film 32 µm | 0.546 | 0.779 | 0.839 | 0.873 | 0.914 | 0.926 | 0.946 | 0.958 | 0.966 |
| engine, as fitted (100 µm) | 0.291 | 0.496 | 0.576 | 0.628 | 0.706 | 0.731 | 0.780 | 0.814 | 0.839 |

His paint's dry pigment volume is 0.243 (the oil at 0.95 of its fresh volume as it dries
[E]; 0.234 at 1.0). The engine's lead white at that pigment volume fits his films below his
hiding criterion (0.98) best as a film of **32 µm** per nominal 100 (rms 0.044; 30.5–33.5 µm
over the oil's dry volume and whether the photometer saw the gloss). The engine's curve is
shallower than his: engine 9's lead white S at 600–650 nm is half its S at 550 nm (a feature
of the Revigo fit), so its red end hides
slower. At his mean hiding thickness, 127 µm, he measured 0.98, the engine gives 0.955.

**Checks [M/C].**
- BYK's half gap would be 50 µm: lead white S per pigment volume 0.28 µm⁻¹, against
  Bruce's 0.44. Bruce's white lead (commercial, 1926) and Revigo's (the RCE reference
  collection) are different lots, so the two can both be right; Bruce measured thickness.
- Pfund's cryptometer (1919): basic carbonate white lead hides a black plate completely, by
  eye, at 36 cm² per gram of pigment, 41 µm of pigment. At 32 µm the engine gives a contrast
  ratio of 0.954 there, as fitted to 100 µm 0.809.
- de Viguerie et al. 2018: lead white at 10.8 % oil, drawn with a 30 µm applicator over an
  optical black, L\* 82.5 after a week. At 32 µm the engine gives that at a film of 15 µm,
  half the gap, as BYK says.
- Lead white's S per pigment volume at 32 µm is 0.44 µm⁻¹: Mie at 1 µm gives 0.347, at
  Kremer's Cremnitz white's measured 3 µm about 0.50 (#351's dry_pigment_optics.md §9.5). Shreve's
  tinting strengths (a secondary table, its basis not stated) over LBNL's rutile give
  0.25–0.38 (§10.4).

**The rule.** With the switch a coat's optical thickness is the coefficients' per
`REVIGO_FILM_UM` (32 µm), 3.1× what it was: tube lead white hides at a contrast ratio of 0.84
at 32 µm and 0.98 at 130 µm (as fitted: 0.59 and 0.88), over Revigo's black and white fields.
Every engine-9 paint's films scale alike; opaque tints don't change.

## 5. Estimates left

- Revigo's paints' oil where table 5 didn't measure it (lead white, vermilion, the chromes,
  viridian, emerald green, red lead, cochineal): the hand-ground rule.
- One pigment density for a tube and its Revigo paint (the box's catalog density).
- The dry oil's volume in Bruce's film (0.95–1.0 of its fresh volume: 30.5–32 µm).
- That Revigo's lead white scatters per pigment volume as Bruce's did.
- The researched (non-Revigo) tubes keep their tables, whose per-100-µm reading was itself a
  swatch's read as an oil film; they scale with the rest.

## Sources

- F. Geldof, Proaño Gaibor, Ligterink, Hendriks and Kirchner, "Reconstructing Van Gogh's
  palette to determine the optical characteristics of his paints", *Heritage Science* 6:17
  (2018), doi:10.1186/s40494-018-0181-6; spectra in `scripts/pigment_data/revigo`.
- E. Bruce, "Hiding power of paints", *Technologic Papers of the Bureau of Standards* 20
  (1926) 173–186, no. 306, tables 2–4:
  https://nvlpubs.nist.gov/nistpubs/nbstechnologic/nbstechnologicpaperT306.pdf
- A. H. Pfund, "The hiding power of white pigments and paints", *J. Franklin Inst.* 188
  (1919) 675–681, doi:10.1016/S0016-0032(19)90902-1.
- L. de Viguerie et al., "Re-interpretation of the Old Masters' practices through optical and
  rheological investigation: the presence of calcite", *C. R. Physique* 19 (2018) 543–552.
- BYK-Gardner, film applicator guidance (wet film about half the gap clearance).
- A. H. Church, *The Chemistry of Paints and Painting*, 4th ed. (1915), p. 66; and the French
  tube figures, as pigment_oil.md §4–5.
- `scripts/pigment_densities.json` (Revigo's table 5 and the hand-ground rule).
