# Sanded pastel paper: a gritted ground

These notes back `paper::Grit` (the easel's `paper={grit=}`). They cover the
coat of grit that makes a sanded pastel paper, a pastel board or a homemade
pumice ground, and what it changes for a pastel stick. Compiled 2026-11-15.
It extends paper_surface.md (the fiber sheet) and pastel_stick_tribology.md
(the stick, Archard wear, the tooth filling).

**Tags:** **[M]** measured or stated by a cited source. **[D]** derived here
from cited values, with the arithmetic shown. **[E]** an estimate with no
direct measurement; treat it as a tunable range.

**The main gap.** I found no profilometry, pore volume or stick-wear
measurement of any artists' sanded paper. The makers grade their grit with
abrasive grit numbers but don't say which standard they use. The layer
counts below are makers' and painters' claims. The coat's geometry is
worked out from the coated-abrasive literature, which describes
sandpaper, not pastel paper.

---

## 1. What a gritted ground is

### Grit, binder and coat
- **[M] Grit materials.** Pumice, marble dust or rottenstone (Wikipedia
  "Pastel"); aluminium oxide on modern papers (UART: "aluminum oxide
  particles uniformly" deposited, per a retailer; Pastel Premier: "double
  primed and coated with aluminum oxide", then "a final seal coat").
- **[M] Binders.**
  - **Acrylic.** Art Spectrum Colourfix is an acrylic primer with grit,
    "waterproof when dry". It can be brushed or rolled onto paper, board,
    canvas, metal or glass. Colourfix papers are 300 g/m² hot-pressed
    watercolor paper screen-printed with that primer.
  - **Starch.** Church (1915) gives a homemade ground: "a thin and even
    coat of powdered pumice mixed with liquefied starch" on Bristol board
    or stout drawing paper.
  - **Glue.** Liotard's paper preparation used "ground pumice stone and
    fish-glue" (a manuscript, via Jeffares). In Rome some pastellists coated
    canvas with parchment glue mixed with sieved marble dust and pumice
    (Chaperon 1788 §323, via Jeffares).
- **[M] Grit size by grade.** These are the FEPA P (ISO 6344) median sizes:

  | grade | P240 | P320 | P400 | P600 | P800 |
  |---|---|---|---|---|---|
  | median size, µm | 58.5 | 46.2 | 35.0 | 25.8 | 21.8 |

  The CAMI grades of the same number are finer (320 ≈ 35 µm, 400 ≈ 22 µm,
  600 ≈ 15 µm; metallography.org).
- **[M] Makers' grades.**
  - UART: 240 ("the largest tooth"), 320 ("extra layering"), 400 ("a
    favorite"), 500, 600 ("detailed work") and 800 ("finest").
  - Pastel Premier: 320 and 400, plus a 600 extra fine.
  - Neither maker says which standard the numbers follow. **[E]** The
    easel takes FEPA P, which gives the coarser reading.
- **[M] Open and closed coats.** In a closed coat the grain covers 100% of
  the backing. An open coat covers 50–70%, which "leaves significant
  valleys between the grains". Closed coats "clog or load quickly" on soft
  materials (Benchmark Abrasives).
- **[M] How far the grain sits in the binder.** In one sandpaper patent
  the size coat is 55–90% of the grain's length. The make and size coats
  together are under 90% of it, so at least about 10% of the grain stands
  proud (US 7,294,048).
- **[E] Make coat alone.** 20–50% of the grain's height. This is an
  engineering rule of thumb; no source here confirms it.
- **[E] For a pastel paper,** take the binder to hold 0.35 of a grain's
  height (`GRIT_EMBED`). A pastel coat needs open valleys, so it is closer
  to a make coat than a full size coat.
- **[E] Coat thickness.** About one to two grain sizes over the sheet:
  some 30–100 µm for 240–600 grit. The coat is thin compared with the
  felt grain's millimeter cells, so the sheet's grain at the mm scale
  shows through. The easel keeps the sheet's relief under the coat.

### Pore volume against drawing paper
- **[D] Drawing paper.** The easel's 160 g/m² sheet has pores whose mean
  depth below the fiber tops is about 6 µm (paper_surface.md;
  `Paper::drawing`). That is all the tooth the pastel model gives it.
- **[D] The open volume between grains.** Treat each grain as a sphere of
  diameter d, radius r = d/2, standing h = (1 − 0.35)·d above the binder.
  - A grain cap of height h has volume V = πh²(3r − h)/3.
  - There are cover/(πr²) grains per unit area.
  - The open volume per unit area above the binder is then
    h − cover · h²(3r − h)/(3r²).
  - At P400 (d = 35 µm, h = 22.75 µm) and an open coat's cover of 0.6, this
    is 22.75 − 0.6 · 16.8 = **12.7 µm**.
  - A closed coat (cover 1) gives 6.0 µm, the same as drawing paper. That
    fits the abrasive trade: a closed coat loads sooner.
- **[E] The binder's own pores.** The binder is lean and porous below the
  valleys, and fine pastel grains (1–5 µm) can enter its interstices.
  - Take acrylic as 0.2 open of the binder's height and glue or starch as
    0.35.
  - Pumice grains are vesicular themselves.
  - This adds 0.2 · 0.35 · 35 = 2.5 µm at P400 in acrylic, and 4.3 µm in
    glue.
- **[D] The tooth (`Grit::tooth_um`).** With cover 0.6:

  | grade | acrylic, µm | glue, µm | × drawing paper |
  |---|---|---|---|
  | P320 | 20 | 23 | 3.4–3.8 |
  | P400 | 15 | 17 | 2.5–2.8 |
  | P600 | 11 | 13 | 1.9–2.1 |

  **[E]** So the tooth is 2–4× drawing paper's: more for the coarse grades,
  less for the fine. The makers put it the same way round: the coarse
  grades for layering, the fine ones for detail.
- **[E] Bearing near the top.** For sphere caps the material ratio just
  below the tops is about 2·cover·z/r. The model's exponential bearing
  curve with mean depth µ gives z/µ there, so µ ≈ d/(4·cover), which is
  14.6 µm at P400. That is within 20% of the open volume above, so the
  model's single pore depth, which fixes both the bearing and what the
  tooth holds, fits a grit coat as it fits paper.
- **[D] Scatter.** A 0.25 mm pixel holds cover·px²/(πr²), about 39 grains
  at P400. A pixel's tooth scatters as Gamma(k)/k over its k grains, as the
  paper's pores do (paper_surface.md §4).

## 2. How many layers it holds
- **[M] Makers' claims.**
  - Pastel Premier 320 and 400 "accept as many as 25 layers of pastel"
    (Jerry's Artarama, Art Department LLC and others).
  - Colourfix primers are "ideal for artists who are looking to do
    multiple layers of pastel without the need for fixative".
- **[M] Painters and reviewers.**
  - Sanded papers hold "10–20+ layers of pastel compared to 2–3 layers on
    regular pastel paper" (Paul Rubens Shop guide).
  - Smoother papers "fill up quickly. Once the shallow valleys are full,
    the paper goes slick" (same guide).
  - A UART reviewer found it "took in quite a few layers, even after
    fixative" (The Artistic Gnome).
  - Jackson's compared 400–800 grades with 1 and 4 layers, in photographs
    only.
- **[E] A layer is a covering application of a few particle thicknesses**
  (6 µm particles; pastel_stick_tribology.md §3). So the layers a sheet
  holds go roughly as its tooth. The geometry above gives 2–4× paper's.
  The claims (10–25 against 2–3) are 4–8×. **The model sits at the low end
  of what painters report.** A thicker, leaner coat, or the pores inside
  pumice grains, could close the gap. Nothing I found measures them.
- **Test thresholds** (pastel.rs `a_sanded_sheet_takes_layer_on_layer`).
  They come from these sources, not from the model:
  - Drawing paper refuses after 2–3 layers: by its fifth pass it lays less
    than a quarter of its first.
  - The coat's tooth is 2–4× and painters report many more layers: over
    twelve passes the sanded sheet holds at least 2× the pastel.
  - Its later passes lay at least 2.5× the paper's, and its sixth still
    lays above that quarter.

## 3. How it wears a stick
- **[M] Painters.** Sanded papers "eat up pastels and coloured pencils
  fast" (Fine Art Tutorials). They "eat pastel faster" and "can chew up
  soft sticks and fingers" (Paul Rubens Shop). The surface "will also
  consume your pastels very quickly" (Ellen Questel). One painter thinks
  grit in an acrylic base eats pastel less (WetCanvas). That is a single
  anecdote, and the easel doesn't use it.
- **[M] Abrasive wear.** In the abrasive model the wear coefficient is the
  asperities' mean tan θ over π. Two-body abrasion runs K = 10⁻²–10⁻¹, and
  three-body abrasion, where loose grains roll, 10⁻³–10⁻² (Rabinowicz 1995,
  via ScienceDirect Topics). The stick's own K against paper (0.01–0.06,
  pastel_stick_tribology.md) is already in the two-body range.
- **[M] Hardness.** An abrasive scratches once it is about 1.2× harder
  than the worn surface. Beyond that the wear rate depends only weakly on
  the hardness ratio (Tabor; Hutchings 2004; Pintaude). Alumina, pumice
  and marble dust are all far harder than a pastel compact, whose σp is
  2–30 MPa. So the grit material barely matters, and the easel doesn't
  ask for it. Its shape does matter.
- **[E] `GRIT_WEAR` = 2.5.** A crushed grain has sharp facets (tan θ about
  0.5–1). A paper fiber's flank is rounded (about 0.2–0.3: the pores'
  6 µm depth over a fiber's 25 µm width). The ratio is 2–4.
  - In the model, 2.5 multiplies the stick's wear where it touches the
    grit above the fill.
  - A fixed crust over the grit is pastel and files the stick as it does
    on paper (`CRUST_WEAR`).
  - The model's first stroke on fresh P400 lays about 2.5× what it does on
    drawing paper. The test `grit_wears_the_stick_faster` asks for at
    least 2×, the low end of that range.

## 4. Fixative
- **[M]** Colourfix is meant to take many layers "without the need for
  fixative". UART takes layers "even after fixative". Pastel Premier
  withstands "scrubbing or reworking", and its 320 and 400 take water,
  mineral spirits and alcohol (heavy alcohol "may soften the surface").
- **[E] The easel doesn't change fixative for grit.**
  - Fixative binds the loose pastel in the tooth, as on paper.
  - The bound crust is filed again at `CRUST_WEAR`, and the grit tips
    above the fill go on abrading at `GRIT_WEAR`.
  - A fixed sanded passage therefore takes pastel again, as a fixed paper
    one does. Because the grit's tooth is larger, it is seldom needed.

## 5. Oil, water and the ground on paper
- **[M]** Colourfix primer takes "oils, gouache, watercolours". For oil on
  paper one tip is to size the paper first with acrylic medium or gesso.
  The primed surface is waterproof (Jerry's Artarama; Delta Art; Jackson's).
- **[E] How the easel takes oil on a gritted sheet** (`Paper::oil_capacity`):
  - The coat's open volume takes oil, as an absorbent ground's pores do.
  - An acrylic coat seals the fibers under it: the sheet's own pores take
    no oil.
  - A glue or starch coat is lean and lets oil through to the sheet, so
    its pores take oil as plain paper's do.
- **A ground laid over a gritted sheet** (`canvas{ground=}`) is a paint
  film. A thin one leaves the grit's tooth open, and one as thick as the
  tooth fills it (`Canvas::micro_um`). Gesso brushed over sanded paper
  does fill it.
- **[E] The knife.** On a gritted sheet a blade rides hard grains. It
  doesn't flatten the tooth or tear fibers loose (no fiber is bare), and
  a scalpel shaves no fiber layer. It still takes loose pastel.

## 6. When gritted grounds were available
- **18th century: pumice on the support.**
  - **[M]** Pastellists drew on coarse blue wrapping paper strong enough
    "to withstand rubbing with pumice, a technique that artists used to
    produce … tooth" (Metropolitan Museum, "The Rise of Pastel in the
    Eighteenth Century").
  - **[M]** Liotard's recipe used ground pumice in fish-glue. Rome's
    pastellists used marble dust and pumice in parchment glue on canvas
    (Chaperon, *Traité de la peinture au pastel*, 1788). Both are via
    Jeffares's *Prolegomena*.
- **Late 19th century: pumice paper and sandpaper as named supports.**
  - **[M]** John Henry Twachtman, *Landscape*, after 1890: "pastel on pumice
    paper, mounted on cardboard" (Metropolitan Museum, Rogers Fund 1925; catalogued in
    Bolger & Shelley, *American Pastels in the Metropolitan Museum of
    Art*, 1989).
  - **[M]** Twachtman's *Pastures, Branchville, Connecticut*, c. 1888–91,
    is on "gray-green sandpaper … (paper: P. L. Breveté)" (Cleveland
    Museum of Art 1946.72).
  - **[M]** Elmer MacRae's *Schooner at Dock*, 1911 or 1914, is pastel on
    pumice paper mounted on board (listed on the same Metropolitan Museum
    page; the page could not be re-read for this note, HTTP 429, so this
    and the Twachtman record are as a search quoted them).
- **Early 20th century.** **[M]** "For pastel work a specially prepared
  paper is now generally employed. This has a surface of finely-powdered
  pumice" (Church, *The Chemistry of Paints and Painting*, 4th ed., 1915,
  pp. 322–323).
- **Modern.** **[M]** Colourfix (Art Spectrum, Australia), UART, Pastel
  Premier (USA) and others make acrylic or aluminium-oxide sanded papers and
  boards. **[E]** I couldn't confirm when each product was introduced.
- **So, for the easel:**
  - A glue or starch pumice ground fits any date from the 18th century on.
  - A commercial pumice paper fits from at least about 1890 (Twachtman).
  - An acrylic sanded paper is a late-20th-century support.

---

## Sources

- FEPA P sizes: Universal Photonics, grit size comparison chart (https://universalphotonics.com/Content/refdocs/_upiRef10b.pdf); HLH, blasting media grit chart (https://hlh-js.com/resource/blog/blasting-media-grit-size-chart-grit-mesh-microns-inches-conversion/); Fine Tools, abrasive grit conversion (https://www.fine-tools.com/blog/conversion-chart-abrasives-grit-sizes); metallography.org grit size chart (https://www.metallography.org/resources/grit-size-chart).
- Open and closed coats: Benchmark Abrasives, "A Complete Guide To Coated Abrasives" (https://benchmarkabrasives.com/blogs/abrasives/what-are-coated-abrasives). Size coat and protrusion: US 7,294,048, "Abrasive article" (https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/7294048).
- UART grades: UART (https://www.uartpastelpaper.com/); Texas Art Supply (https://www.texasart.com/products/uart-premium-sanded-pastel-paper); Jackson's Art Blog, "UArt Pastel Paper" (https://www.jacksonsart.com/blog/2015/12/04/uart-pastel-paper/); The Artistic Gnome, "UART Sanded Paper Review" (https://theartisticgnome.com/uart-sanded-paper-review/).
- Pastel Premier: Jerry's Artarama (https://www.jerrysartarama.com/global-arts-premier-sanded-paper-320-grit-italian-clay-sanded-paper-sheets-10-pack-20x26-quot); Art Department LLC (https://artdepartmentsupply.com/products/pastel-premier-sanded-pastel-paper-12x16-white); FLAX (https://flaxart.com/pastel-premier-sanded-pastel-paper/).
- Colourfix: Art Spectrum, Colourfix brochure (https://artspectrum.com.au/wp-content/uploads/2017/10/Colourfix-DL-Web.pdf); Delta Art technical sheet (https://www.deltaart.ca/images/uploads/file/coloufixprimer[1].pdf); Jerry's Artarama (https://www.jerrysartarama.com/art-spectrum-colourfix-pastel-primer); Jackson's (https://www.jacksonsart.com/en-ca/art-spectrum-colourfix-pastel-primer).
- Painters on layers and wear: Paul Rubens Shop, "Soft Pastel Paper" (https://www.paulrubensshop.com/blogs/knowledge/soft-pastel-paper); Fine Art Tutorials, "The Best Pastel Paper" (https://finearttutorials.com/guide/best-pastel-paper/); Ellen Questel (https://www.ellenquestel.com/blog/161250/introduction-to-soft-pastels-and-some-tips-on-how-to-work-with-them); WetCanvas, "Wallis Sanded Paper" (https://www.wetcanvas.com/forums/topic/wallis-sanded-paper/).
- Abrasive wear coefficients: ScienceDirect Topics, "Abrasion" (https://www.sciencedirect.com/topics/materials-science/abrasion), citing Rabinowicz, *Friction and Wear of Materials*, 2nd ed. (1995), and Rabinowicz, Dunn & Russell, *Wear* 4, 345 (1961).
- Hardness ratio: Hutchings, "Modelling abrasive wear", *Wear* (2004) (https://www.sciencedirect.com/science/article/abs/pii/S0043164804002595); Pintaude, "An Overview of the Hardness Differential Required for Abrasion" (https://www.researchgate.net/publication/245371525); ScienceDirect Topics, "Hutchings" (https://www.sciencedirect.com/topics/engineering/hutchings).
- History: Metropolitan Museum, "The Rise of Pastel in the Eighteenth Century" (https://www.metmuseum.org/perspectives/the-rise-of-pastel-in-the-eighteenth-century); Jeffares, *Prolegomena to Pastels & pastellists* (https://pastellists.com/Misc/Prolegomena.pdf); Metropolitan Museum, Twachtman, *Landscape* (https://www.metmuseum.org/art/collection/search/12844) and Bolger & Shelley, *American Pastels in the Metropolitan Museum of Art* (1989; https://www.metmuseum.org/met-publications/american-pastels-in-the-metropolitan-museum-of-art); Cleveland Museum of Art 1946.72 (https://www.clevelandart.org/art/1946.72); Church, *The Chemistry of Paints and Painting*, 4th ed. (Seeley, Service & Co., 1915), pp. 322–323, quoted at https://noteaccess.com/MATERIALS/PPS.htm; Wikipedia, "Pastel" (https://en.wikipedia.org/wiki/Pastel).
