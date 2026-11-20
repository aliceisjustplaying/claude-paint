# The veil over paper, black chalk and graphite

The pastel veil switch (`--@ pastel-veil`; paint's `Canvas::with_pastel_veil`, `Canvas::unveiled`, `Canvas::paper_open`), extended from pastel_surface.md: with it, the matte first-surface veil is shown over a paint film only. Compiled 2026-11-16. [M] measured, [D] derived, [E] estimate. The numbers in §§3–4 come from `black_chalk_optics.py` (`uv run notes/research/black_chalk_optics.py`).

## 1. What the engine did

The engine shows every matte surface with a veil (`canvas::haze`): k = 0.04 × (1 − gloss), an oil film's Fresnel reflection at n ≈ 1.5, scattered back by a matte face and added over its colour as k(1 − R). Bare paper has gloss 0.05 (`Canvas::with_paper`), so k = 0.038 over every sheet.

pastel_surface.md (#329) took the veil off every open particle bed. It kept it over paint, the paper, graphite and black chalk, and left the paper as an open question.

## 2. Paper [D]

**Where the paper's colour comes from.** A preset's tone (`Preset::tone`) is parts of tubes (`Palette::dry_color`), fitted to a target sRGB. Two targets were measured, as median patches of bare paper in museum photographs; the others are estimates of the sheet's appearance (cassatt_papers.md, "Tones"). A plain sheet's default tone, `#e8e0cf`, is also an appearance. So every tone is what a viewer sees of the whole sheet, with its surface's reflection already in it. Showing it under the veil counts that reflection twice. Each colored sheet came out 1–5 L* lighter than the photograph it was fitted to (§5).

**Why a sheet has no separate first surface.** Physics gives the same answer as the tones' origin.
- *A sheet is a scattering bed of fibres and fillers.* Its Kubelka–Munk R∞ is "the total diffuse reflectance", measured d/0° with the specular port trapped (TAPPI T 525 and the like) [M: Farnood 2009, Optical properties of paper: theory and practice, 14th FRS, pp. 275 and 283]. Each fibre's Fresnel reflection (cellulose n ≈ 1.53: 4.4 % at normal incidence) is one of the scattering events inside that R∞, exactly as a pigment particle's is inside a pastel's.
- *Paper is rough on the scale of light.* "Roughness of paper is two orders of magnitude larger than the wavelength of light" [M: Farnood 2009, p. 307]. Parker Print-Surf roughness runs from 0.9 µm (glossy coated) to 5.9 µm (uncoated) over a set of European papers [M: TU Dortmund thesis]. Uncoated sheets in a surface-sizing study measured 6.3–8.2 µm (PPS S10) [M: "Effect of the paper structure and composition on the surface sizing pick-up"]. The coherent (mirror) share of a rough surface's Fresnel reflection is exp(−(4πσ cos θ/λ)²) (Beckmann–Spizzichino, as Farnood gives it) [D]:
  - at σ = 1 µm and λ = 0.55 µm it is 10⁻²²⁷ at normal incidence, and 10⁻¹⁵ even at the glossmeter's grazing 75°;
  - at σ = 0.5 µm it is 10⁻⁵⁷ and 1.6 × 10⁻⁴.

  An uncoated sheet's surface reflection is therefore all incoherent: light scattered off tilted fibre facets. That is part of the diffuse reflectance a spectrophotometer, a camera or a viewer records. An uncoated sheet's measurable 75° gloss comes from facets tilted to the mirror angle (Farnood's samples start near 6 GU; uncoated offset papers are commonly 5–15 GU at 75°, an industry figure no source here confirms).

**The size film.** A gelatine-sized sheet does carry gelatine at its faces.
- *How much.* Historical European papers hold 0–7 % gelatine by weight [M: Barrett & Mosier 1995, JAIC 34; Ormsby, Barrett & Hojjatie]. Ten 17th–18th-century writing papers held 2.6–5.8 % [M: National Bureau of Standards 1943, as cited by Barrett & Mosier]. Northern papers fell to 1–2 % after 1600 [M: Paper Through Time, University of Iowa].
- *How thick.* At 3.4 % (a well-preserved 1483 sheet), a 110 g/m² sheet holds 3.7 g/m², or 2.8 µm at 1.35 g/cm³. That is 1.4 µm per face if all of it lay on the faces, and it doesn't: a tub size soaks into the sheet [D]. At 7 % it would still be under 3 µm per face. The machine-made wove sheets were rosin-alum sized in the pulp, which leaves no surface film at all (cassatt_papers.md; wet_pastel.md §4.2). A Cobb60 of 25 g/m² (`Paper::cobb60`) shows the surface pores still open to water.
- *Why it adds nothing.* A film 1–3 µm thick over a surface several µm rough, its fibres 25 µm wide, follows the fibres. Its air face is as rough as they are and scatters as incoherently. Gelatine's index (≈ 1.54) is cellulose's to within 0.01, so the film's air face reflects what the fibres' own faces did (4.4 %), and the buried gelatine–cellulose face reflects 10⁻⁵ [D]. Size replaces the fibres' first surface; it doesn't add a second one. Whatever it does is in the sheet's measured tone.

**Calender.** Calendering lowers the roughness and raises the gloss. An uncoated, calendered sheet is still microns rough, so at a viewer's near-normal angles its reflection stays incoherent [D, by the factor above].
- *What it moves.* Going from a matte to a glossy uncoated finish (≈ 5 to ≈ 15 GU) moves at most about a tenth of a 4.4 % surface reflection out of the diffuse light, ≤ 0.5 % absolute [E: GU taken as the share of a smooth surface's mirror reflection]. Calendering also densifies the sheet and lowers its scattering ("calender blackening").
- *Why it isn't modelled.* Each preset's tone is its sheet as it is, at its own finish. The effect of the `calender` dial on the tone is real but small (≤ 0.5 L* on a mid tone), below the tones' own uncertainty (fits of 0.3–5 ΔE to estimated targets). It isn't separable from them without gloss and scattering data on these sheets.

**So: no veil over bare paper.** A sheet is seen as its tone, whatever its size or calender. The veil comes back only where a film covers the paper's pores. A paint film (or a ground) gives its own smooth face once it is thicker than the paper's micro-roughness. Thinner, it follows the fibres and leaves them open, by the rule the engine already uses for a film over paper's pores (`Canvas::micro_um`). The veil grows with film / micro and is whole at 1 (`Canvas::paper_open`) [E: the linear share is that rule's; a film's levelling is the physics behind it]. So there is no edge where a stroke of paint thins out onto bare paper. A sheet laid over the picture (`lay_sheet`) is unveiled too, its paper and the pastel it caught.

## 3. Black chalk [D, its R∞ E]

**What it is.** Natural black chalk (pierre d'Italie, pierre noire) is a carbonaceous shale or schist: "about 10-15% carbon diffused with fine clay so that it would be soft enough for drawing" [M: historyofdrawing.com, "Drawing Materials"; the Met, "Chalk"; the National Gallery glossary, "Chalk": carbonaceous shale from northern Italy and France]. It has no binder; the paper's tooth abrades crumbs off the lump. Its mark is matte: "Graphite produces a gray-black mark that has a slight sheen to it — black chalk is duller and blacker" [historyofdrawing.com]. Conté's manufactured pierre noire is called "matt... without the sheen of graphite" [P: artists' accounts].

**So it is an open particle bed, like pastel and charcoal.** Its crumbs' facets scatter inside the bed's R∞, and there is no smooth face for a veil. The engine had it veiled only because #329 grouped it with graphite as `non_pastel`. With the switch it is unveiled.

**Its R∞.** The engine's `Lead::chalk` flake, 0.022, is an estimate from the first graphite commit (eb10a22c). Mie and Kubelka–Munk from the composition (black_chalk_optics.py) bound it widely, because the answer depends on how the carbon is spread through the clay:
- *Crumbs of shale,* the carbon finer than light within the clay (an effective medium, 10 % porosity): R∞ **0.010–0.014** (Y; 10–15 wt% carbon, its index a 400–600 °C char's or lamp black's).
- *Clay platelets and 1 µm carbon particles as separate scatterers:* **0.25–0.34**.

Shale's organic matter is largely dispersed finely through the clay, which favours the dark bound, but the crumbs' structure isn't measured. The engine's 0.022 lies inside, near the dark end, and is kept [E]. Without the veil it is seen at L* 16.5, as dark as vine charcoal (0.021). With the veil it was 29.2. The sources call a black chalk line "often less intense than that of charcoal" and "grayish black, because of impurities". That fits a broken line and a grayer stone better than a veil. A reflectance spectrum (FORS) of a solid black chalk passage would set R∞.

## 4. Graphite [D]

**Graphite is opaque.** At its basal plane (Djurišić & Li 1999, J. Appl. Phys. 85:7404, ordinary ray, via refractiveindex.info) [M]:
- n = 2.72 + 1.49i at 555 nm, so normal reflectance 0.32 (0.29 at 450 nm, 0.34 at 609 nm);
- an absorption length λ/4πk of about 30 nm.

The light that enters a flake goes no further than 30 nm. Graphite has no body colour under a surface: **all it returns is its own first-surface reflection**. A pencil's pressure lays the platelets flat along the paper into a near-continuous, slightly greasy layer [P: Smartermarx, "When dry media collide"]. Their mirror reflection gives the "silvered metallic brightness" seen under side light [M: forensic microscopy of pencil strokes, Northwestern J. Crim. Law & Criminology]. A copying pencil, with almost no graphite, lacks it.

**How the engine models it, and why the veil doesn't belong.**
- *What `flake` is.* `Lead::flake` (0.30 for 9H down to 0.05 for 9B, estimates) is the gray a diffuse viewer sees of the laid flakes. That is the off-specular share of their surface reflection, plus the clay of a hard lead.
- *The veil models something graphite doesn't have.* The veil is a dielectric film's Fresnel reflection (n 1.5) scattered back by a matte face. Graphite's surface is graphite's: its reflection is already all of `flake`.
- *The engine's own rule agrees.* A glossy surface sends its first-surface reflection away from the viewer, k(1 − gloss) → 0 (`haze`). A specular film is the extreme of that.

Both readings give one answer: no veil over graphite, which is seen at `flake` over its coverage. Before the switch it carried k(1 − R) more, 3.5 L* for a 2H and 8.5 for a 9B at full cover.

**Not modelled.** The glint itself: the mirror lobe toward a raking lamp, `seen_lit` included, is not modelled. The engine has one diffuse reflectance per pixel. A soft lead's "glossier" line is not raised in the canvas's gloss either.

## 5. What it changes [D]

**A bare sheet**, the whole canvas (an easel test prints these: `paper_veil_tests`):

| sheet | Y veiled → bare | L* veiled → bare | ΔL* |
|---|---|---|---|
| drawing paper (#e8e0cf) | 0.759 → 0.750 | 89.8 → 89.4 | 0.4 |
| blue wove | 0.260 → 0.231 | 58.0 → 55.1 | 2.9 |
| gray-blue wove | 0.322 → 0.295 | 63.5 → 61.2 | 2.3 |
| tan wove | 0.281 → 0.253 | 60.0 → 57.3 | 2.7 |
| buff wove | 0.513 → 0.494 | 76.9 → 75.7 | 1.2 |
| green wove | 0.376 → 0.352 | 67.7 → 65.9 | 1.9 |
| brown wove | 0.163 → 0.130 | 47.4 → 42.8 | 4.6 |
| cream laid | 0.719 → 0.708 | 87.9 → 87.4 | 0.5 |
| japan paper | 0.768 → 0.758 | 90.2 → 89.8 | 0.4 |

With the switch each sheet is seen as the color it was fitted to (cassatt_papers.md's "engine 9 dry color").

**Black chalk and graphite at full cover** (flake alone):

| lead | R | L* bare | L* veiled | ΔL* |
|---|---|---|---|---|
| black chalk | 0.022 | 16.5 | 29.2 | 12.7 |
| 2H | 0.183 | 49.8 | 53.3 | 3.5 |
| HB | 0.156 | 46.5 | 50.5 | 4.0 |
| 4B | 0.107 | 39.1 | 44.4 | 5.3 |
| 9B | 0.050 | 26.7 | 35.2 | 8.5 |

**A hatched swatch** (three passes at full pressure, 4-unit spacing, on the drawing paper; the paper between the strokes counts):

| lead | Y veiled → bare | L* veiled → bare |
|---|---|---|
| black chalk | 0.111 → 0.076 | 39.7 → 33.1 |
| 4B | 0.283 → 0.255 | 60.2 → 57.5 |
| HB | 0.409 → 0.385 | 70.1 → 68.4 |
| 2H | 0.469 → 0.448 | 74.1 → 72.7 |

**Old logs** (no `--@ pastel-veil`) replay as painted: everything veiled but charcoal (`Cell::bare_carbon`), the code path as it was before #329.

## Sources

- R. Farnood, "Optical properties of paper: theory and practice", 14th Fundamental Research Symposium, Oxford 2009, pp. 273–352: https://bioresources.cnr.ncsu.edu/wp-content/uploads/2019/04/2009.1.273.pdf
- PPS roughness: TU Dortmund thesis (Eldorado repository): https://eldorado.tu-dortmund.de/server/api/core/bitstreams/58f856f9-88f4-4f14-b519-c0cc57a568d8/content; "Effect of the paper structure and composition on the surface sizing pick-up": https://www.researchgate.net/publication/273381283
- T. Barrett and C. Mosier, "The role of gelatin in paper permanence", JAIC 34 (1995): https://cool.culturalheritage.org/jaic/articles/jaic34-03-002.html; Paper Through Time (University of Iowa): http://paper.lib.uiowa.edu/
- Black chalk: historyofdrawing.com, "Drawing Materials": https://historyofdrawing.com/?page_id=5; the Met, "Chalk": https://www.metmuseum.org/about-the-met/collection-areas/drawings-and-prints/materials-and-techniques/drawing/chalk; National Gallery glossary, "Chalk": https://www.nationalgallery.org.uk/paintings/glossary/chalk; Clark Art Institute, "Drawing Materials": https://www.clarkart.edu/microsites/lines-from-life/about/exhibition-(1)
- Graphite: A. B. Djurišić and E. H. Li, "Optical properties of graphite", J. Appl. Phys. 85, 7404 (1999), tables at https://refractiveindex.info/?shelf=main&book=C&page=Djurisic-o; Smartermarx, "When dry media collide: graphite with white pastel": https://smartermarxmagazine.com/when-dry-media-collide-graphite-with-white-pastel-charcoal-white/; "Sequence of pencil strokes", J. Crim. Law & Criminology: https://scholarlycommons.law.northwestern.edu/cgi/viewcontent.cgi?article=5165&context=jclc
- Carbon and clay indices, and the Mie and Kubelka–Munk recipe: charcoal.md §2, charcoal_optics.py, dry_pigment_optics.md §2.
