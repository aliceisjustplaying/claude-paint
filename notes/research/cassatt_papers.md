# Cassatt's papers and mounts: the give of a mounted sheet, and named sheets

Research notes for `paper={mount=, preset=}` (crates/paint/src/paper.rs: `Mount`, `Drum`, `PRESETS`). Compiled 2026-11-15. It builds on:
- `paper_mechanics_and_transport.md` §1: z-compression of paper, and the Winkler bed the pastel contact uses.
- `paper_surface.md`: the fiber-network dials.
- notes/degas-materials.md §1: papers and mounting in her circle.
- The Cassatt research on branch `notes/cassatt-materials` (`notes/research/cassatt_materials.md` §2).

[M] marks a measured or catalogued value, [E] an estimate, and [D] a value derived here from [M] and [E] values.

## 1. Mounts: how much a laid-down sheet gives under a stick

### What she used [M]
Museum records give the mounts, and sometimes say they have changed:
- **Board.**
  - "Pastel on green wove paper, mounted on illustration board" (c. 1881) [Met 10424].
  - "Pastel on wove paper, mounted on cardboard" (c. 1897) [Met 10404].
  - "Blue-gray wove paper (faded to tan), mounted on board" [AIC 1968.81].
- **Canvas.** "Pastel on wove paper, mounted on canvas" (1895–1914) [Met 13533, 10403, 10387, 10396, 10397]. Some were taken off their strainers later [Met M&C].
- **Strainer.**
  - "Pastel on blue-gray wove paper (faded to tan), mounted on canvas, on a strainer" (1888) [AIC 1922.421].
  - "Originally mounted on a strainer" (1899) [Met 10389, 13532].
  - "Cassatt typically executed her pastels on paper that was adhered to canvas and tacked to a wood strainer", whereas most artists by then used millboard [Met M&C].
  - On one strainer-mounted Cassatt, the paper's edges were glued firmly to the strainer and the free bay moved with the climate until it tore at its weakest points [Choi & Makin].
- **Degas, for comparison.** He usually wrapped the sheet over 5–7 mm paperboard, and occasionally stretched it on a stretcher (notes/degas-materials.md §1).

### The model
The pastel contact (crates/paint/src/pastel.rs `seat`) treats the surface as a Winkler bed of stiffness k (MPa per µm). For a sheet alone on a hard drawing board, k = E_z / caliper (`Paper::z_mpa`; 39 MPa measured on board [M, Mann, Baum & Habeger]). A mount adds its own spring in series:

  1/k = caliper / E_z + 1/k_backing (+ the bay's drum on a strainer)

`Paper::mounted` folds the backing into `z_mpa`, so the sheet still gives one number per pixel. The strainer's drum depends on where the stick is (`Paper::give_at`). With no mount, the sheet behaves exactly as it did before mounts existed.

### Board: k_backing ≈ 0.1 MPa/µm [E]
- Pasteboard, millboard and illustration board are 2–7 mm thick (Degas's 5–7 mm [M]).
- Their through-thickness modulus is of the order of paper's: 20–40 MPa [M, paperboard plies: Mann et al.; Kananen et al.].
- Under the stick's patch (radius a ≈ 2–4 mm), the board's top plies (in-plane modulus 2–8 GPa) spread the load over a region a few times the patch's width. The board then compresses over its thickness across about ten times the patch's area.
- So k ≈ (E_z / t_board) × 10 ≈ (30 MPa / 3000 µm) × 10 ≈ **0.1 MPa/µm** [E].
- For the default drawing sheet (caliper 300 µm, k_sheet 0.13), board gives k ≈ 0.057, an equivalent z of **17 MPa** [D].

### Canvas: k_backing ≈ 0.015 MPa/µm [E]
- Linen or cotton canvas is 0.3–0.6 mm thick.
- A woven fabric is very soft at first touch: apparel cottons lose 30–50% of their thickness by 5 kPa in the Kawabata compression test (EMC) [M, range; Kawabata-system reviews].
- It stiffens steeply once the yarns pack. A stick bears about 0.05–0.3 MPa (pastel_stick_tribology.md), far into that packed regime. The paste and the sheet also bridge the weave.
- Take the packed cloth's tangent modulus as 5–10 MPa [E] over about 0.4 mm. That gives k ≈ 0.012–0.025; use **0.015 MPa/µm** [E].
- For the default sheet: k ≈ 0.0134, equivalent z **4.0 MPa** [D].

### Strainer: the canvas, and the bay's drum [E]
- **Tension.** A well-tensioned canvas painting carries about **0.12–0.24 kN/m**; 0.2 kN/m is the usual working value [M, Heritage Science 2022]. "Optimal" is 1–2 N/cm, and up to 2–2.5 N/cm [M, Academia.edu]. A canvas loses 30–50% of its tension in the first 48 hours [M, Berger & Russell].
- **Bending.** The sheet pasted to canvas is a thin plate. Take its bending stiffness as D ≈ 40 N·mm [E]: paper of in-plane modulus about 3 GPa, about 0.3 mm thick, laminated to the cloth.
  - Over lengths below √(D/T) ≈ √(40 / 0.2) ≈ **14 mm**, the plate bends rather than stretches [D].
- **Global sink.** A 2 N press in the middle of a 60 cm bay deflects it by about F/(2πT)·ln(R/ℓ) ≈ 2/(2π·0.2)·ln(300/14) ≈ **5 mm** [D]. That is the felt give, the "drum".
  - The Winkler contact doesn't need it: the whole patch goes down together.
- **What the contact does need** is how much the surface conforms across the stick's patch. For a plate under a patch of radius a, the deflection varies across the patch by about F·a²/(16πD) ≈ 2 × 9 / (16π × 40) ≈ **9 µm** [D].
  - That is an equivalent bed of k ≈ (2 N / 28 mm²) / 9 µm ≈ **0.008 MPa/µm** in the middle of the bay (`STRAINER_BAY_MPA_PER_UM`) [E].
- **Bars.** The bars are about **40 mm** wide under the picture (`STRAINER_BAR_MM`) [E: 3–5 cm bars on 60–80 cm strainers]. Over them the wood backs the canvas, so the strainer gives as canvas does.
- **Between the bars and the middle.** The point compliance of a tensioned membrane grows as the log of the distance from its support, so the drum's share is s(d) = ln(1 + d/ℓ) / ln(1 + d_max/ℓ), with ℓ = 14 mm. Here d is the distance in from a bar, and d_max is the distance from the bar to the middle of the bay [E: the shape].
- **Crossbars.** Larger strainers had them; they are not modeled.
- **For the default sheet** in the middle of the bay: k ≈ 0.0050, equivalent z about **1.5 MPa** [D]. The stick sinks deepest there.
- **Effect on the picture** (the engine's result, not a source). In the contact model, a softer bed lets the sheet give around the stick, so more of the stick's end bears on the tooth and the same stroke lays more.
  - One 12 mm round soft stick drawn 30 mm at 3 N and 60° on the default sheet laid:
    - 0.57 mm³ with no mount
    - 0.70 on board
    - 0.78 on canvas
    - 0.86 in the middle of a strainer's bay
    - 0.76 over a strainer's bar, the same as on canvas
  - So a strainer-mounted pastel would take a little less pastel over its bars than in the bay: a faint stretcher-bar mark.
  - Whether practice agrees (a firm board is often said to "bite") is unverified. No source describes bar marks in her pastels.

### Order
For any sheet: no mount (a hard drawing board) > board > canvas = strainer over its bars > strainer in its bay, and the bay gives more at every step inward. The tests are `paper::tests::mounts_order_the_give` and `pastel::tests::a_stick_sinks_further_on_a_strainer_than_on_canvas_or_board`.

## 2. Named sheets

### Her papers [M]
- **Tones.** Tan, buff, brown, green and blue are recorded, and late in life she preferred blue [cassatt_materials.md §2; Met M&C].
- **Fading.** Several blue and blue-gray sheets have faded to tan:
  - "wove paper (originally blue)" [Met 10405]
  - "blue-gray wove paper (faded to tan)" [AIC 1922.421, 1968.81]

  So some sheets catalogued as tan today may once have been blue. Degas's aniline-dyed papers faded the same way (notes/degas-materials.md §1).
- **Wove and laid.**
  - Wove sheets are the rule.
  - A laid sheet [Met 13532].
  - Off-white and ivory laid papers for the color prints, watermarked "ED&Cie" and "PL BAS" [cassatt_materials.md §5].
- **Japan paper** for the counterproofs. One NGA counterproof is "pastel over pastel counterproof on chine collé" [NGA 1980.45.9], and a dealer's sheet of the same subject is "pastel counterproof on Japan paper" [Cavalier].
  - Kozo fibers average about **10 mm** (10–15) by **30 µm** [M, CAMEO]. Gampi is about 3 mm by 12 µm [M, Melbourne fibre atlas].
  - Printmaking gampi is 28–42.5 g/m², and lining tissues 10–20 g/m² [M, Japanese Paper Place; Ozu].

### The dials [E]
- **The colored wove sheets.** No measurements of these sheets were found. They were commercial pastel papers, machine-made from wood pulp with some rag. The preset uses estimates for such a sheet:
  - 130 g/m²
  - mean fiber 1.6 mm (softwood 2–3.5 mm with hardwood and groundwood fines; paper.rs)
  - a light felt mark (0.7 mm cells, 14 µm deep)
  - calender 0.25
  - otherwise the drawing paper's values
- **Brown wove.** An unbleached wood-pulp sheet: heavier (150 g/m²), coarser (0.2 mg/m), shorter fibers (1.4 mm), cloudier (floc 0.6), a deeper felt mark.
- **Cream laid.** A rag laid sheet:
  - 110 g/m² (Ingres-type papers about 100–120)
  - fiber 2.2 mm
  - laid lines 9 per cm, chain lines 25 mm apart, a 0.15 fiber deficit over the wires (paper.rs's handmade range)
  - a felt mark
- **Japan paper.** Long fibers, thin and smooth:
  - 45 g/m² (the dial's floor is 40)
  - fiber 10 mm (the dial's ceiling) by 30 µm
  - coarseness 0.1 mg/m
  - porosity 0.6 (an open, bulky sheet)
  - well formed (floc 0.15)
  - no felt (the sheets are brushed onto drying boards)
  - calender 0.6 (smooth)
- **Not set.** Every preset keeps the drawing paper's absorbency (0.8) and give (39 MPa), because no source gives either for these sheets. A mount changes the give.

### Tones: measured from museum images, then fitted to tubes [E]
- **Method.** I took Wikimedia Commons reproductions (960 px) of museum photographs. For each, I took the median sRGB over patches where the paper is bare. Medians drop the pastel flecks.
- **Caveats.** These are photographs of sheets as they are now:
  - white balance and the photographer's color management are unknown
  - colored papers fade (above)
  - Japan paper yellows with age

  Every tone here is therefore an estimate.

| sheet | target sRGB | from |
|---|---|---|
| tan wove | #918262 | measured: NGA 1985.64.81 (*The Black Hat*, "pastel on tan wove paper"). Lower-right reserve (145, 130, 98); the edge patches carry gray pastel, (131–141, 126–131, 112–113) |
| green wove | #9ca296 | measured: Met 10424 (*Woman on a Bench*, "green wove paper"), four reserves: #97a095, #9ca296, #9fa194, #a0a699. It reads gray-green now; it may have faded, as Degas's greens did |
| blue wove | #6b86a8 | estimate: an unfaded mid blue. I found no bare reserve of a blue sheet in an image I could measure |
| gray-blue wove | #8a96a0 | estimate: AIC's "blue-gray" before fading |
| buff wove | #c9b68f | estimate |
| brown wove | #7f6248 | estimate. A High Museum photograph of a sketch "on brown paper" measured #a36a6f (pink): an unreliable color photograph, not used |
| cream laid | #e4dac2 | estimate: ivory laid |
| japan | #ebe3cc | estimate of the sheet as made. The NGA counterproof's sheet measures #dbc79a today (three reserves: #dbc79a, #dbc698, #ddc99c), aged and yellowed |

**Fitting the tones.** Each target was fitted to parts of the catalog's tubes, as the engine dries them (engine 9's `Palette::dry_color` of the "every" box). The fit was a grid search over up to four tubes, with ΔE in CIELAB and a small penalty for each extra tube. A preset's tone is computed from the catalog whatever the painting's box, since a dyed sheet isn't made from the painter's tubes. An engine 5–8 painting dries the same parts a little differently.

| sheet | tone (parts) | engine 9 dry color | ΔE |
|---|---|---|---|
| blue wove | lead white 1, ultramarine blue 1, bone black 0.45 | #6d86a7 | 0.9 |
| gray-blue wove | lead white 1, ultramarine blue 1, viridian 1.5, red earth 1.5 | #8995a0 | 0.3 |
| tan wove | yellow ochre 0.2, bitumen 1, vine black 0.23 | #9d866c | 5.2 |
| buff wove | lead white 1, yellow ochre 2.2, raw umber 3, Prussian blue 0.02 | #cdb891 | 1.2 |
| green wove | lead white 1, yellow ochre 0.65, viridian 0.45, bone black 0.45 | #9ba296 | 0.3 |
| brown wove | raw sienna 1, bitumen 3 | #7d5f48 | 2.0 |
| cream laid | lead white 1, yellow ochre 0.15, raw sienna 0.02, green earth 3 | #e5dac3 | 0.9 |
| japan | lead white 1, yellow ochre 0.1, raw sienna 0.1, viridian 0.02 | #eae1ca | 0.6 |

The tan sits 5 ΔE lighter and less yellow than the photograph. The dry earths can't reach that chroma at that lightness, so it is the closest the tubes give.

## Gaps
- No measured grammage, fiber or surface data for her colored papers; the dials are estimates.
- No measurement of a mounted sheet's give: no indentation data on paper laid on board or canvas, and none on a tensioned canvas under a point load at a stick's scale.
- Blue tones are unmeasured. The best next step is a bare reserve in a high-quality museum image of a late blue sheet (the Met's 1914 *Mother and Child* is mostly covered).
- Crossbars, keys and slack canvas (tension loss) are not modeled.

## Sources
- [Met 10389, 10403, 10404, 10405, 10387, 10396, 10397, 10424, 13532, 13533] The Met Collection API object records (medium lines), https://collectionapi.metmuseum.org/public/collection/v1/objects/{id}, read 2026-11-15.
- [Met M&C] "Mary Cassatt in a Modernist Light: A Close Look at Mother and Child", The Met, *Perspectives*: https://www.metmuseum.org/perspectives/mary-cassatt-modernist-mother-child (via search summary).
- [AIC 1922.421] *Mother's Goodnight Kiss*: https://www.artic.edu/artworks/81527/mother-s-goodnight-kiss (via search summary); [AIC 1968.81] *Sleepy Nicolle*: https://www.artic.edu/artworks/28826/sleepy-nicolle (via search summary).
- [NGA 1985.64.81] *The Black Hat*: https://www.nga.gov/artworks/66479-black-hat; image: https://commons.wikimedia.org/wiki/File:Mary_Cassatt,_The_Black_Hat,_c._1890,_NGA_66479.jpg
- [NGA 1980.45.9] *Sara Wearing a Bonnet and Coat*: https://www.nga.gov/artworks/57658-sara-wearing-bonnet-and-coat; image: https://commons.wikimedia.org/wiki/File:Mary_Cassatt,_Sara_Wearing_a_Bonnet_and_Coat,_c._1904-1906,_NGA_57658.jpg
- [Cavalier] Cavalier Galleries listing, "pastel counterproof on Japan paper", 62.2 × 45.7 cm (via search summary).
- Met 10424 image: https://commons.wikimedia.org/wiki/File:Woman_on_a_Bench_MET_DP-13450-001.jpg
- High Museum sketch on brown paper (not used): https://commons.wikimedia.org/wiki/File:Sketch_of_a_Mother_Looking_Down_at_Thomas_by_Mary_Cassatt,_c._1893,_pastel_on_brown_paper,_High_Museum_of_Art.jpg
- [Choi & Makin] "Treatment and Housing Techniques for Pastel Paintings on Paper", AIC Book and Paper Group Annual 32: https://cool.culturalheritage.org/coolaic/sg/bpg/annual/v32/bpga32-06.pdf (via search summary).
- [Heritage Science 2022] "Mechanical properties and moisture-related dimensional change of canvas paintings – canvas and glue sizing": https://www.nature.com/articles/s40494-022-00794-3 (0.12–0.24 kN/m; 0.2 kN/m working value).
- [Academia.edu] "About the choice of tension for canvas paintings": https://www.academia.edu/8773879/about_the_choiche_of_tension_for_canvas_paintings (1–2 N/cm).
- [Berger & Russell] JAIC 29(1), 1990: https://cool.culturalheritage.org/jaic/articles/jaic29-01-004.html (30–50% tension loss in 48 h).
- [Kawabata-system reviews] KES-FB3 compression (EMC, LC): https://www.sciencedirect.com/topics/engineering/kawabata-evaluation-system; Utkun 2021, cotton fabrics on KES (ResearchGate 357374410).
- [Mann et al.; Kananen et al.] as cited in paper_mechanics_and_transport.md §1.
- [CAMEO] Kozo: https://cameo.mfa.org/wiki/Kozo; [Melbourne fibre atlas] Gampi: https://cultural-conservation.unimelb.edu.au/gampi.html; [Japanese Paper Place] https://www.japanesepaperplace.com/news/washi-and-printmaking/; [Ozu] https://ozuwashi.com/pages/faq
