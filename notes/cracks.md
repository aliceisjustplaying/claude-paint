# Craquelure revamp (branch `cracks`)

## Round 7: less neat (branch `r7-cracks`)

Alice on the aged *Evening at a Mountain Lake*: "we do want the age stuff",
but the craquelure is "still too neat, still too digital"
(`notes/round6/alice_review.md`, last section). The old render
(`notes/paint1/evening_lake_aged_3200.png`) showed why. One fine polygon
web covered the whole picture. Every crack had about the same width, the
islands had about the same size everywhere, and the density was the same in
the sky and the water. In the dark fir wood the cracks simply stopped at
the silhouette, because the paint's lightness decided whether they formed,
so they looked cut by a mask.

### What the literature says (text sources only)

- **The network lives in the ground; paints differ in how they carry it.**
  In the classic model the glue-bound ground is the most brittle layer
  (strain at break about 0.002) and cracks first. Janas et al. found that
  some aged paints are more brittle than that. Lead white paint "continues
  to get stiffer and stronger" but keeps "considerable flexibility" and is
  "resistant to cracking" from its own shrinkage. Zinc white and verdigris
  become "extremely brittle". Umbers lose strength and grow more plastic
  [JANAS22]. Kim et al. found that canvas-aging cracks appear "regardless of
  the paint layer colors" and form more easily where the paint over the
  ground is thin [KIM22].
- **Width and depth.** By OCT a canvas-aging crack measured 70 µm wide and
  370 µm deep in thin paint, and 50 µm by 236 µm in thicker paint. Drying
  cracks are wider and shallower (89 × 181 µm), and in thicker paint wider
  and deeper (123 × 219 µm) [KIM22]. A saturated fine network in a lead-white
  paint layer measured 20 ± 8 µm wide, with islands about 6 paint
  thicknesses apart [JANAS22]. Real cracks vary. Most knife-made fakes share
  "a similar inverted triangle shape" and come out "wide/deep or
  narrow/shallow" [KIM22]. Artificial craquelure
  generally comes out "uniform in appearance, while genuine craquelure has
  cracks with irregular patterns" [WIKI].
- **Hierarchy.** "The top bar of each T formed first", which gives primary
  and secondary generations (Bucklow, in the research notes). Italian panels
  show "distinct secondary networks of thin cracks" [WIKI, after Bucklow
  1997]. New cracks form midway between old ones until the spacing
  saturates [JANAS22].
- **Thin films don't crack by themselves.** Below a critical thickness a
  drying film stays whole. Near it, cracks form as "isolated star-shaped
  crack junctions", and thick films form complete networks. Stiffer paints
  crack "more spread out" [WIKI, after Giorgiutti-Dauphiné 2016].
- **Direction.** Dutch canvases crack "perpendicular to major axis of
  painting". French canvases, on thicker grounds, show "non-directional
  cracks with smooth, curved lines" [WIKI, after Bucklow 1997].
- **The stretcher.** "Most old cracked paintings have cracks corresponding
  to the stretcher inside edges" [HACK04]. Conservators name the stretcher
  bar crease (cracks along the inner edges of the bars), draw crackle
  ("parallel cracks emanating from the corners … creating a ripple
  effect"), pinwheel cracks from impacts, traction crackle and varnish
  blanching from "very minute cracking" [HART].
- **Cupping.** Most of the tension is carried in the paint and ground, "but
  where the paint cracks, all the tension is taken by the canvas", and the
  "re-alignment of forces causes the islands of paint to cup" [HACK04].
  Raking light shows cupping that frontal light nearly hides (research
  notes, §5).
- **Friedrich.** A thin paint over a two-to-four-layer ground whose top is
  "a patchy whitish" lead white [KÖR p.284], and "one to two very thin
  layers" of paint [SMB-proj]. *The Sea of Ice* has an "ear-of-grain"
  craquelure [HH]. No quantitative crack study of a Friedrich exists
  (`notes/research/friedrich_materials.md` §2, §8).

### Against the algorithm

The Round 3 engine already had the right skeleton: a sequential network
(nucleate at the highest stress/strength, run across σ1, relax, T-junctions),
spacing fitted to the ground, corners, a stretcher band and a weave prior
for thin grounds. What made it neat:

| Real craquelure | Round 6 engine | Round 7 |
|---|---|---|
| a few long first cracks dominate; later ones are finer | opening ≈ released stress: 1.0, 1.0, 0.8, 0.6, 0.4 by generation | `hierarchy`: 2.2, 1.1, 0.5, 0.2, 0.1 (≈ 73 µm down to hairlines) |
| width swells and pinches; cracks fade out | ±30% over 0.7 mm; free ends taper | also ×e^±0.55 over 3.5 mm; later cracks close shut for stretches |
| passages differ a lot | ±30% stress, lightness and film prune 0–3 generations | `patchy`: a strength field over ~11 cm (a patchy ground) grows a different network: some passages keep only their first cracks |
| a direction on some canvases | none (thick ground) | `grain`: tension along the length, drifting ±25°; a tendency (0.15) |
| grime differs; varnish residues | the same soot in every crack | `grime`: per crack amount (width, a patchy cleaning), 18% amber |
| cracks in darks show (pale ground, dust) | dark slot on dark paint, invisible, so cracks end at the dark's edge | walls show the ground: a faint light line in darks |
| thin paint: hairlines; thick: wider | opening from the film, saturated in the wet engine (film reads 250–1000 µm) | recalibrated (below) |

### What changed (`crates/paint/src/crack.rs`)

1. **`hierarchy`** (0..1, default 1). A crack's opening is
   `1.7 × 0.68^generation` × a per-crack log-normal (σ ≈ 0.45) × its
   released stress, blended with the old opening by `hierarchy`. Along the
   crack a slow swelling (e^±0.55 over 3.5 mm) multiplies the old fine
   variation. Later generations close to nothing over stretches (roughly
   12%, 25%, 38% and 50% of their length for generations 1–4, in pieces of
   a few millimeters). The
   cupping scales with √(generation factor), so primaries cup most.
   Physically: a crack keeps opening while the islands on either side
   shrink into it, so the first cracks, which split the largest islands,
   open most.
2. **`patchy`** (0..1, default 1). The film's strength is multiplied by up
   to e^1.1 ≈ 3 by a broad field (110 mm features, skewed so most of the
   canvas cracks well). There the network stops about three generations
   early. It is grown that way, not pruned, so its ends are real
   T-junctions or free ends, and it is canvas-wide, so crops match.
3. **`grain`** (0..1, default 0.15). ±`0.25 × grain` of the mean stress added
   along the canvas's longer side, its direction drifting ±25°. With it,
   cracks start off the ideal heading by up to ±50° where the stress is
   nearly isotropic, so the direction stays a tendency. At 0.5 it looked
   like rain.
4. **`grime`** (0..1, default 1). The crack's color is its shadowed walls
   (`SLOT` × paint over pale ground, 50% ground under thin paint, 15% under
   thick; the ground's color `GROUND_WALL` is an assumed yellowed lead white
   over ocher). Over them lies a gray-brown grime whose amount differs per
   crack: it grows with the opening, varies ×0.4–1.6 per crack and drops by
   up to 80% where a patchy past cleaning reached. 18% of cracks hold amber
   old varnish instead. Over lights a crack reads dark, and over darks a
   faint light line (test: 2× the paint, the old soot 0.8×).
5. **Thickness recalibrated.** The film bookkeeping counts every coat laid,
   also paint blended or wiped away, so in the wet engine it reads far above
   a real film: the lake's sky about 250 µm and the fir wood 500–1000 µm
   (median 330, 90th percentile 670 over the whole canvas). `THICK_UM = 150..700` now marks thin
   to thick (it was 30..150, which saturated everywhere). The opening uses a
   physical film of 20–300 µm across that range.
6. `Cracks::even(seed)`: the Round 6 recipe (all four at 0), for
   comparisons and for the tests that measure the network.

The easel's `cracks{}` accepts `hierarchy`, `patchy`, `grain` and `grime`
(`crates/easel/src/api.rs`, one line each in the key list and the override
macro). Nothing else in the shared files (`handling.rs`, `api.rs`,
`bristle.rs`) changed.

### API

```lua
-- the last chunk, as before: now uneven by default
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; cracks{}; relief()
cracks{grain=0.4}                        -- a canvas that cracked more across its length
cracks{patchy=0.3, hierarchy=0.6}        -- a more even, finer web
cracks{hierarchy=0, patchy=0, grain=0, grime=0}   -- the Round 6 look
relief(0.3)                              -- raking light: the cupped islands show
```

```rust
o.finish(&mut c, &mut rng, &Finish::aged(st.relief)); // Cracks::aged: all four on
let k = Cracks { grain: 0.4, ..Cracks::aged(seed) };
let old = Cracks::even(seed);
```

### Evidence (`notes/cracks_r7/`, lossless, built from the renders)

Made from the log on `r7-paint-wet` with `cracks{}` added before `relief()`
in the last chunk. That branch's easel (the wet engine) rendered the
painting at 3200 and at 1000 once. A scratch hook saved the canvas just
before `cracks{}`, and a harness cracked and lit it with the old and the
new code (its old output is byte-identical to
`notes/paint1/evening_lake_aged_3200.png`).

- `evening_lake_cracks_before_after_3200.png`: 1:1 crops of the 3200 render,
  before | after: the sky with the moon, the lake's glow and mirror, the
  dark fir wood against the glow. Plain crops: `before_*.png`,
  `after_*.png`.
- `evening_lake_cracks_before_after_1000.png`: the whole at 1000, each
  cracked at that resolution.
- `evening_lake_aged_r7_1000.png`, `evening_lake_aged_r7_3200.png`: the
  plain aged painting.
- `raking_light_water_3200.png`: the lake crop under `relief(0.3)`, the
  Round 6 recipe | Round 7: the cupped islands along the first cracks.

What I see. **Sky:** before, a dark, even web of 3–4 mm islands everywhere.
After, a few long wandering cracks that open and pinch, finer ones between
them that fade out, larger quiet stretches in the dark upper sky and a
denser web low in the glow. **Lake:** a well-developed passage, but with
visibly different islands, some cracks bold and many hairlines. **Firs:**
before, cracks stopped at the silhouette. After, they run on into the wood
as faint warm lines, sparser there (dark, oily, thick paint). **At 1000:**
before, a fine mesh over the whole picture. After, it is barely there: a
few long lines, a denser patch low right and draw crackle at the corners,
about what one sees standing back from an old varnished canvas.

### Tests (`crack::tests`)

- `first_cracks_open_widest`: opening by generation 1.0 … 0.4 without
  hierarchy, 2.2 … 0.08 with it; each generation narrower.
- `patchy_development_leaves_quiet_passages`: crack length per 20 mm cell,
  coefficient of variation 0.04 → 0.25, quiet cells 0 → 7%.
- `grain_turns_the_first_cracks_across`: first cracks within 30° of
  vertical on a landscape canvas: 0.34 (none), 0.56 (0.15), 0.94 (1).
- `cracks_in_darks_read_light`: over paint of 0.020 the old soot gives
  0.016 and the new walls and grime 0.041 (it fails with `grime: 0`).
- `crop_cracks_like_the_whole`: the whole `crack()` output of a crop
  matches the whole canvas within 1e-4 away from its edge.
- The older network tests pin `Cracks::even`.

### Open issues / next

- **Paint identity.** Lightness and film still stand in for pigment:
  "dark" means an oily earth or black and "light" means lead white. The wet
  engine knows the pigments while the paint is wet. A per-pixel record of
  lead white, zinc and verdigris content would let brittle paints crack
  finely and tough ones hold (Janas).
- **Own drying cracks in thick paint** (star junctions near the critical
  thickness, full networks above it) are not modeled. The film bookkeeping
  would first need a physical thickness.
- **The ground's color** in the crack walls is a constant. The canvas
  doesn't store its ground color, and storing it means a checkpoint format
  change.
- **Not done:** pinwheel impact cracks, roll damage (no solid text source
  found), the *Sea of Ice* "ear-of-grain" pattern and traction crackle.
- The factors (1.7, 0.68, the closing fractions, e^1.1, 0.25, grime ×0.4–1.6,
  18% amber, the thickness range) are tuned by eye on this painting within
  the measured widths. They are assumptions.
- The corners' draw crackle shows at 1000 as faint arcs. That is physical
  (Hartmann), but it may read as a pattern.

### Sources (Round 7)

- [JANAS22] A. Janas, M. F. Mecklenburg, L. Fuster-López, R. Kozłowski,
  P. Kékicheff, D. Favier, C. K. Andersen, M. Scharff, Ł. Bratasz,
  "Shrinkage and mechanical properties of drying oil paints", *Heritage
  Science* 10, 181 (2022). https://www.nature.com/articles/s40494-022-00814-2
- [KIM22] S. Kim, S. M. Park, S. Bak, G. H. Kim, C.-S. Kim, J. Jun,
  C. E. Kim, K. Kim, "Investigation of craquelure patterns in oil paintings
  using precise 3D morphological analysis for art authentication",
  *PLoS One* 17(7): e0272078 (2022).
  https://pmc.ncbi.nlm.nih.gov/articles/PMC9333328/
- [HACK04] S. Hackney, "Paintings on Canvas: Lining and Alternatives",
  *Tate Papers* 2 (2004).
  https://www.tate.org.uk/research/tate-papers/02/paintings-on-canvas-lining-and-alternatives
- [WIKI] "Craquelure", Wikipedia (after S. Bucklow, "The Description of
  Craquelure Patterns", *Studies in Conservation* 42 (1997), and
  F. Giorgiutti-Dauphiné, "Painting cracks", *J. Appl. Phys.* 120, 065107
  (2016)). https://en.wikipedia.org/wiki/Craquelure
- [HART] Hartmann Fine Art Conservation, "Conservation damage terms".
  https://www.hartmannconservation.com/damage-terms
- AIC Conservation Wiki, "Crackle" (Stout's 1974 terms, Bucklow's features).
  https://www.conservation-wiki.com/wiki/Craquelure
- [KÖR], [SMB-proj], [HH]: `notes/research/friedrich_materials.md`.

---

## Round 3: the preset fits the canvas

The problem, from all three amnesia-2 painters and the user: "the crack reads
as a grid", "a little too digital and aggressive". `Finish::aged` used
`Cracks::aged`, which assumed a 60 µm ground (partly weave-bound, so the
cracks followed warp and weft in a rectangular grid). `Style::friedrich`
primes 240 µm (110 + 70 + 60) and the winter painter primed 140 µm. The
preset also drew 70 µm cracks with dirt 0.6, so at 1000px the network
darkened every ~8 px island edge and the preview looked like crazed glass
(`notes/amnesia2/fresh2_mountains.md` friction 1 and 11,
`fresh2_winter.md` 1, `fresh2_coast.md` 8). Each painter hand-tuned a
`Cracks` to work around it.

## What changed

1. **The preset fits the canvas.** `Cracks::aged(seed)` now leaves
   `ground_um`, `island_mm` and `width_um` as `None`. `Canvas::crack` fills
   them in (`Cracks::fit`):
   - **Ground:** from the canvas. `Canvas::prime` and brushed style grounds
     add their thickness to `Canvas::ground_um()`, and the value is kept in
     checkpoints (appended at the end of the state, format bumped to `PAINTCK2`; after merging with drying it is `PAINTCK3`). With 240 µm the weave
     coupling is 0 (de Willigen: thick grounds give smooth, curved cracks),
     so there is no grid.
   - **Island size:** proportional to the layer thickness. Channel cracks
     saturate at a spacing of ten to twenty film thicknesses:
     `island_for(g) = 14e-3 × (g + 20 µm)`, clamped to 1.2–7 mm. That gives
     3.6 mm for Friedrich's 240 µm and 2.2 mm for the winter's 140 µm. The
     winter painter picked 2.6 mm by eye. The factor 14 is an assumption,
     calibrated to the 2–6 mm range in the research notes.
   - **Opening:** strain × island size, with `STRAIN = 0.9%` (assumption),
     so 33 µm on a Friedrich ground instead of 70 µm. Depth is 20 µm,
     cupping 15 µm, dirt 0.4.
2. **Resolution-aware rendering.** Crack coverage was already area-based
   (tent filter). What made the preview dark was the width, the dirt and
   point-sampled cupping. Now:
   - The cupping profile is averaged over the pixel analytically, so the
     sharp 1/d lift no longer puts a full-height ridge on one pixel at 1000px.
   - The open crack is a shadowed slot (`SLOT = 0.35` of the paint's
     reflectance) with grime over it, mixed in by the covered area. A 33 µm
     crack covers ~7% of a 0.44 mm pixel, which is a faint web at 1000px.
     At 3200px (0.14 mm) the same crack covers ~25% of a pixel and reads as
     a fine, particular hairline.
   - Worn shoulders shrank from 0.6 to 0.4 of the width on each side.
3. **Non-uniform aging** (`Cracks::vary`, 1 in `aged`, 0 gives the old
   even network):
   - **Stress field:** ±30% broad variation over a few centimeters
     (uneven drying and humidity), plus a **stretcher**. The canvas held
     over a bar cracks 15% less. Along a bar's inner edge the canvas flexes,
     which adds a stress across the edge, so a wandering band of cracks runs
     parallel to it. Bars are 30–70 mm wide, with a cross bar on canvases
     over 900 mm. The network still grows sequentially from this field
     (crack.rs: nucleate at the highest σ/strength, run perpendicular to σ1,
     relax, T-junctions).
   - **Local paint** (`Local`, a ~1 mm grid over the rendered window,
     built from the canvas's film thickness and lightness):
     - Lead-white-rich lights are brittle and go through all 5 generations
       of the network. Dark, oily glazes stop up to 2 generations earlier,
       so their islands are larger and more of their cracks end free.
     - Paint over 30–150 µm thick loses up to one more generation.
     - Cracks open wider where the layer is thicker:
       √((ground + paint) / (ground + 20 µm)), clamped to 0.7–1.8.
     - A crack fades out over one generation, so there are no hard cutoffs.

     This is local (only the rasterizing reads it), so a `--crop` render
     still cracks exactly like the whole canvas.
4. **Milky varnish veil** (`Cracks::veil`, 0.5 in `aged`): patches of old
   varnish crazed into microcracks about 0.3 mm apart (jittered Voronoi
   edges), whose lit edges scatter light "like a milky veil" [SMB-blog,
   `notes/research/friedrich_materials.md` §7]. Where a pixel is finer than
   the crazing it draws the edges as faint light lines. Where it is coarser
   it draws their mean haze, ~1.3% toward milky gray in a full patch. The
   haze shows over darks and disappears over lights. My first try at ~7%
   made gray clouds, so it stays deliberately small.

## API

```rust
// the default: fits itself to the canvas (ground, islands, opening), uneven,
// a little milky varnish
o.finish(&mut c, &mut rng, &Finish::aged(st.relief));

// override any part; None fields still come from the canvas
let k = Cracks { dirt: 0.2, veil: 0.0, ..Cracks::aged(0) };
let k = Cracks { island_mm: Some(5.0), vary: 0.3, ..Cracks::aged(0) };
o.finish(&mut c, &mut rng, &Finish { cracks: Some(k), ..Finish::aged(st.relief) });

c.ground_um();                 // what the canvas was primed with, µm
paint::crack::island_for(240.0) // 3.6 mm
Cracks::aged(0).fit(240.0)       // the recipe with every None filled in
```

Breaking change: `island_mm`, `ground_um` and `width_um` are now
`Option<f32>`, and there are new fields `vary` and `veil`. I updated
`friedrich_moonrise_valley` (now plain `Finish::aged`), `study_workflow`
and `study_cracks` (its calibration panels pin `vary: 0`, `veil: 0`). The archived `paintings/fresh2/*.rs`
set these fields by hand and would need `Some(..)`, but they are not built.

## Evidence

- `cargo paint study_aging` → `out/study_aging.png` (new study). One
  Friedrich canvas (440 mm, 240 µm) with a smalt sky glaze, stiff lead
  white and an umber glaze, varnished. Left: the old numbers. Right: the
  new preset. Top: 1000px whole. Bottom: the same 3200px window.
  `notes/cracks/study_aging.jpg`, `study_aging_new_3200.jpg`.
  - Old: at 1000px a visible rectangular mesh over the sky and white. At
    3200px a weave-bound grid of rectangles, the "brick wall" look.
  - New: at 1000px the web is barely there. It is faint in the lights and
    absent in the darks, about as much as you'd see standing back from a
    44 cm picture. At 3200px there are curved, particular hairlines with
    T-junctions and varying width. The islands in the thick lead white
    are larger and their cracks longer than in the sky.
- `fresh2_winter` (copied temporarily into `paintings/src/bin`, finish set
  to plain `Finish::aged(st.relief)`), before (old code) and after:
  - `notes/cracks/winter_before.jpg` vs `winter_after.jpg`. Before, a dark
    grid covers the whole picture and competes with the oak and the ruin.
    After, the picture reads as the painter meant it. The surface is aged
    but quiet.
  - `winter_before_sky_zoom.jpg` vs `winter_after_sky_zoom.jpg` (2.5×
    zoom of 1000px sky). The rectangular mesh became an isotropic, uneven
    web, denser in some places than others, with a wandering
    stretcher-bar line near the right edge.
  - `winter_before_3200_crop.jpg` vs `winter_after_3200_crop.jpg`
    (`--full --crop 300,120,600,380`). Before: bricks. After: a fine
    network of different-sized islands, which is plausible for a small
    varnished canvas. The snow in shadow gets sparser cracks than the lit
    snow.
  - My judgment: the change fixes the complaint. The preset no longer
    fights the painting, and nobody needs a hand-tuned workaround.
- Renders are fast: the network and raster for a 1000px canvas take well
  under a second, and 0.3 s for a 3200px window.
- `cargo test -p paint` passes. The crack tests pin `vary: 0` or explicit
  sizes where they measure the network. The golden scene doesn't crack and
  is unchanged.

## Round 3 review fix: crop origin (branch `fix3-physics`)

`crack_local` grouped pixels into ~1 mm averaging cells starting at the
window's corner, not on a whole-canvas grid. A crop whose corner didn't
fall on a cell boundary averaged different pixels into each cell, so its
local reach and opening changed away from the crop's edge too. The
review's probe used light and dark 4-px strips: reach 3.75 in the whole
render against 4.49 in a crop starting one pixel over, so a generation-4
crack went from absent to 49% visible. Now the cells sit on the canvas's
grid (edges at multiples of `n` canvas pixels) and `Local`'s origin is the
first cell's canvas corner. Cells cut by the buffer's edge average the part
inside. That and the bilinear interpolation reach about two cells (~2 mm)
in, which lies inside a crop's unsaved margin (`run::DEFAULT_MARGIN`, 40
units, ~28 mm). Whole renders are unchanged, since their grid already
started at 0. `study_aging` changes by at most 4 of 255 levels, and those
changes come from the paint fixes, not the cracks.

Test: `crop_origin_doesnt_move_the_local_field` builds a crop at canvas
pixel (41, 42), which cuts the 4-px cells, over light and dark strips and a
film that thickens in steps. It matches the whole render's reach and
opening within 1e-4 everywhere two cells or more from the crop's edge.
Before the fix it failed: reach 2.98 whole against 3.86 cropped.

## Known issues / next

- **Cracks in darks are nearly invisible.** A dark slot on dark paint has
  no contrast. On real paintings, cracks in darks often show *light*: the
  paler ground shows in the gap, or the cupped edges catch the light. The
  crack goes through the ground, so the ground's color could show at the
  bottom of wider cracks. Relief light (strength 0.2 in the Friedrich
  style) only hints at the cupping.
- "Paint type" uses lightness as a proxy for lead-white content. It
  doesn't know which pigments are there. A per-pixel binder/pigment record
  from the wet layer would be better (stream 1 territory).
- Film-driven density prunes late generations while rasterizing instead
  of growing a different network. It's cheap and crop-safe, but a pruned
  crack's T-junction partners become free ends. The alternative is to
  record film thickness canvas-wide (which a crop doesn't have).
- The island factor (14 × thickness), the strain (0.9%), the stretcher
  width and the veil strength are assumptions tuned by eye within the
  research ranges. No quantitative crack study of a Friedrich exists
  (research notes §8).
- Not done: drying cracks (wide, early cracks in bitumen or oily darks),
  impact spirals, the "ear-of-grain" pressure cracks of *The Sea of Ice*.
