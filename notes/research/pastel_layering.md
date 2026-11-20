# Pastel over pastel: what a stroke lays on worked, loose, smudged and fixed pastel

Five behaviours a painter found while painting a Cassatt pastel live (engine 9, the 2026-11-16 integration build ea2690aa, with the pastel K–M, veil, fixative index and oil-K switches), checked against physics and practice. One was a bug; it is fixed behind `--@ pastel-crumb-layer` (paint's `Canvas::with_pastel_crumb_layer`). The other four are right; the numbers below say why. [M] measured, [D] derived, [P] practitioners' reports, [E] estimate.

## 1. The scene

A 160 px, 150 mm window of `Paper::drawing(4)` (mean pore depth µ = 6.0 µm), all of a new painting's drawing switches on. The ground is three passes of a near-black soft stick (R 0.05/0.03/0.02, soft 0.75: σp 3.9 MPa, K 0.048) at 2 N and 30°, leaving 3.3 µm of loose pastel in the band. "Smudged" is that ground worked over by six finger passes at 1 N. "Fixed" is that ground after `fix()`. Over each, four passes of a pale soft stick (0.85/0.80/0.75, 2 N, 30°) or a pastel pencil (soft 0.35: σp 11.6 MPa, K 0.028, 2.5 N, 60°), each after one stroke off the band to wear a facet. The volume each pass lays (mm³ over an 80 mm stroke) and the band's luminance Y after each pass:

| stick | over | mm³ per pass | Y: ground → after 4 passes, as built | with crumbs layered |
|---|---|---|---|---|
| soft | bare | 2.39, 1.12, 0.50, 0.31 | 0.550 → 0.636 | 0.550 → 0.636 |
| soft | loose | 0.66, 0.40, 0.25, 0.19 | 0.387 → **0.382** | 0.387 → 0.442 |
| soft | smudged | 1.56, 0.40, 0.21, 0.13 | 0.322 → **0.312** | 0.322 → 0.419 |
| soft | fixed | 1.94, 1.43, 1.23, 1.09 | 0.382 → 0.582 | 0.382 → 0.582 |
| pencil | bare | 0.11, 0.62, 0.62, 0.51 | 0.550 → 0.584 | same |
| pencil | loose | 0.075, 0.058, 0.044, 0.038 | 0.387 → 0.405 | same |
| pencil | smudged | 0.10, 0.09, 0.41, 0.39 | 0.322 → 0.372 | 0.322 → 0.395 |
| pencil | fixed | 0.10, 0.10, 0.53, 0.47 | 0.382 → 0.446 | same |

(The pencil's first pass is light everywhere: its fresh point bears on a few pixels until it wears a flat.) The volumes are the same with and without the fix: it changes only how the crumbs show.

## 2. The bug: a stroke's crumbs were mixed into the loose pastel [D]

With the pastel K–M switch (#328, branch fix/pastel-km-mixing) a stroke's **face deposit** lies over loose pastel as a layer (Kubelka's two-layer formula, pastel_mixing.md §3), but its **crumbs**, 1 − 1/6.6 ≈ 85 % of all it lays (`SHED`), were mixed into the loose pastel intimately, as the finger's paste is. In Kubelka–Munk a little dark swamps a lot of light (pastel_mixing.md §1), so 1.4 µm of pale pastel mixed into 3.3 µm of near-black is still near-black, and the extra coverage of that mixture over the paper between the marks made the band *darker*: Y 0.387 → 0.382 over loose, 0.322 → 0.312 over smudged. That is behaviours (b), (c) and (d) as seen.

The crumbs settle in the bed of pixels under the face that it does **not** touch (`lay_crumbs_of`'s bed: 0 < gap < a crumb's size, `CRUMB_UM`). The face is above them; nothing shears them into what is there. Intimate mixing needs the particles worked together (the finger, a brush, a blade, a paste); crumbs dropped on a bed lie on it. So the crumbs take the same two-layer rule as the face deposit: `layered`, with the new particles over the old where they land, beside them where the old leaves gaps. On bare paper the two rules agree (nothing under them; checked to 10⁻⁴).

With the switch: four pale passes over the loose dark lighten it by 0.055 in Y (about 3 L* there), over the smudged one by 0.097, monotonically; the same over a fixed ground lightens it by 0.20, as it lays 3× the volume (§4). Pastel over loose charcoal still comes out greyer than over fixed: a stroke's layer is thin and leaves gaps (charcoal.md §6).

Practice agrees that a light laid over a loose dark shows: the usual order in soft pastel is dark to light, the lights last and unfixed (ArtTechniques; Newberry, "a one-way trip from dark towards the light", 15 or so tones; HubPages: the final light colours left completely unfixed to keep them bright) [P]. Lights "struggle" over heavy darks (ArtTechniques), which is the full tooth (§3), not the color.

## 3. (b) and (c): a stroke lays less over loose or smudged pastel — right

**Loose.** Loose pastel in the pores is a third body: where the stick meets it rather than paper, nothing abrades the stick and nothing lodges. The first pale pass over the three dark passes lays 0.28× a pass on bare paper. Counted in layers of a first pass on bare paper, the ground (1.68) plus four pale passes (0.63) is 2.3 layers, inside the 2–3 layers painters claim for a drawing paper (sanded_paper.md §2; the engine's `a_sanded_sheet_takes_layer_on_layer` holds it under 5) [P]. "Eventually you're painting pastel over pastel and it won't take any more" (WetCanvas) [P].

*Is a loose bed really no abrasive?* Its chalk and pigment grains (calcite 1.3 GPa) are far harder than the compact (2–30 MPa), so rolling grains do abrade it, three-body: Rabinowicz's K 10⁻³–10⁻² against two-body 10⁻²–10⁻¹, about a tenth (sanded_paper.md §3) [M]. Tried at a tenth of paper's rate, a drawing paper never refuses: the same stroke keeps laying 0.3× a fresh pass after the tooth is full. But what a full tooth refuses is *retention*, not wear: over a filled pore the abraded particles have nowhere to lodge, ride on as dust and fall off [P]. The engine's zero net deposit over the filled pores is that limit. (The crumbs, which do lodge in the hollows under the face, are kept whatever the fill.)

**Smudged.** The finger presses the loose grains into the pores and spreads them evenly, the tops' pores too: it fills the tooth, as the tribology note's packing says ("it pushes grains into valleys, taking coverage toward 100 % and flattening the tooth", pastel_stick_tribology.md §3) and as painters say of blending [P]. It doesn't seal: the first pass over the smudged passage lays 1.56 mm³, more than over the unsmudged one (0.66; the finger moved some pastel out of the band), and `fix()` gives it tooth again (§4). What looked like refusal was the color (§2).

## 4. (d): a fixed passage takes more — right

`fix()` binds the loose pastel into a rough crust that files the stick again (`CRUST_WEAR` 0.5 of paper, two-body), while loose pastel files it not at all (§3). Over the fixed ground the pale stick lays 1.94, 1.43, 1.23, 1.09 mm³ against 0.66, 0.40, 0.25, 0.19 over the loose one: 2.9× on the first pass, 5.7× by the fourth. Practice: fixative between layers restores tooth, "the first layer acts as a tooth for the subsequent layers" (HubPages); Degas fixed between layers and later layers sat on top unmixed (notes/degas-materials.md) [P]. What was wrong was only that a light didn't show *before* fixing (§2).

## 5. (a): a pencil lays almost nothing over worked pastel — right

A pastel pencil is three times harder than a soft stick (σp 11.6 against 3.9 MPa) with K 0.028 against 0.048: Archard's K/σp is 0.20 of the soft stick's. Its small point carries its load on few pixels, so it presses deep into the micro-relief, and over a filled tooth most of that is on the loose fill, which bears load but doesn't abrade it. Over the loose ground it lays 0.075 → 0.038 mm³ a pass, an eighth to a sixteenth of what it lays on bare paper once its point has a flat (0.62), and because it barely wears, its point stays sharp and keeps laying little. Over the smudged or fixed ground it wears a flat by the third pass and lays 0.4–0.5. The crumb fix changes almost nothing here (a pencil sheds few crumbs). This is the pastellist's rule "hard before soft": pencils and hard pastels go under soft pastel or on a fixed layer, not over loose soft pastel [P].

## 6. (e): dry browns with white read pink — right for the earths that are

The engine's dry tints (engine 9 with the oil-K and dry-lakes switches, `Palette::dry_color`), CIELAB:

| tube + lead white | L* | a* | b* | C | h |
|---|---|---|---|---|---|
| burnt sienna 50 % | 79.5 | 13.9 | 9.4 | 16.8 | 34.0 |
| burnt sienna 90 % white | 91.7 | 6.3 | 3.7 | 7.3 | 30.4 |
| raw umber 50 % | 77.3 | 6.7 | 13.6 | 15.2 | 63.7 |
| raw umber 90 % white | 89.9 | 3.5 | 5.8 | 6.8 | 58.8 |
| red earth 90 % white | 87.0 | 12.0 | 6.7 | 13.7 | 29.3 |
| Mars brown 90 % white | 87.8 | 4.6 | 2.4 | 5.1 | 27.2 |
| Indian red 90 % white | 86.0 | 6.9 | −1.3 | 7.0 | −11.0 |
| bone brown 90 % white | 87.4 | 2.2 | 3.7 | 4.3 | 59.4 |

**Burnt sienna** (calcined, hematite-rich) is a salmon tint: CSS salmon (#FA8072) is h 32.7°, dark salmon 44.5°. **Raw umber** (goethite with manganese oxides) and bone brown are grey-browns, h 59–66° at chroma under 16. Both as expected.

The pinks are the hematite browns, and hematite tints are pink. By Mie (Bohren & Huffman, the repository's `dry_pigment_mie.py`) with hematite's measured optical constants (Querry 1985, ordinary ray), in oil, against lead white (`earth_tint_mie.py`) [D]:

| hematite d | masstone h | tint 10 % | 3 % | 1 % (h, C) |
|---|---|---|---|---|
| 0.2 µm | 35.5 | 31.2, 18.1 | 29.4, 12.2 | 28.7, 8.0 |
| 0.4 µm | 22.0 | 21.7, 7.5 | 22.4, 4.7 | 22.7, 3.0 |

A fine hematite's tint sits at h ≈ 29–31°, which is where the engine's burnt sienna tints are (30–34°). A coarser one goes rosier (22°): Mars brown, red earth and Indian red tints reading pink to mauve is what the mineral does. (Pure Querry hematite absorbs more than a natural earth, but the hue of the tint is set by the shape of its K over S, which the earth's hematite shares.)

**A caveat, not a bug here.** The researched tubes' K and S were fitted to their masstones with one wavelength-flat S (`mixer/researched.json`, "flat_s_limitation"). Hematite's S is not flat: Mie gives S(650)/S(550) = 2.7 at 0.2 µm, 2.9 at 0.4 µm, 1.6 at 1 µm. For fine hematite the flat-S rule makes the tint yellower and more chromatic than Mie (+3 to +15° in h), the opposite of pink. Only for coarse hematite (1 µm) does it push the tint toward violet (−16 to −31°), at chroma under 6, nearly grey. So the flat-S fit cannot be what makes burnt sienna or umber tints pink; it may make **Indian red**'s tint too violet (−11° in the engine). Measured tint ladders of the earths in oil would settle it: none was found openly (Okumura 2005's acrylic K and S, the RIT database and the MPI oil dataset were checked: the first two aren't openly downloadable here, the third has no earths).

## 7. The switch

`--@ pastel-crumb-layer` in a log's head; paint's `Canvas::with_pastel_crumb_layer`, riding on the pastel K–M switch (`with_pastel_km_mix`; without it, off). A bit of the checkpoint's engine word (1 << 51), no section; a checkpoint with the bit and without the K–M switch is refused. New paintings have it; a log without the line replays as painted.

## Sources

- Rabinowicz, *Friction and Wear of Materials* (1995): two- and three-body abrasive wear coefficients, as cited in sanded_paper.md §3.
- Querry, M. R., *Optical constants*, CRDC-CR-85034 (1985), https://apps.dtic.mil/sti/citations/ADA158623; via Polyanskiy, refractiveindex.info, *Sci. Data* 11, 94 (2024), Fe2O3/Querry-o (CC0).
- Bohren & Huffman, *Absorption and Scattering of Light by Small Particles* (1983), BHMIE (dry_pigment_mie.py).
- Wyman, Sloan & Shirley, "Simple analytic approximations to the CIE XYZ color matching functions", *JCGT* 2(2), 2013.
- Handprint, "colormaking attributes" (https://www.handprint.com/HP/WCL/watere.html): burnt sienna watercolor masstone LCh 45/46/42; burnt umber 36/24/48.
- ArtTechniques, "Pastel Painting: How the Medium Works" (https://arttechniques.com/techniques/pastel); M. Newberry, *Art Companion* ch. 30 (https://michaelnewberry.substack.com/p/art-companion-chapter-30-pastel-technique); WetCanvas, "layering pastels problem" (https://wetcanvas.com/forums/archive/index.php/t-349683.html); HubPages, "How to Create Texture With Artists' Soft Pastels" (https://discover.hubpages.com/art/texture-with-soft-pastels).
- Checked for earth K and S: Okumura, *Developing a Spectral and Colorimetric Database of Artist Paint Materials*, MS thesis, RIT (2005); Berns, *Artist Paint Spectral Database* (RIT); Asadi Shahmirzadi, Babaei & Seidel, "A Multispectral Dataset of Oil and Watercolor Paints", *EI* 2020 (no earths).
- In this repository: pastel_mixing.md, pastel_stick_tribology.md §3, sanded_paper.md §2–3, charcoal.md §6, notes/degas-materials.md.
