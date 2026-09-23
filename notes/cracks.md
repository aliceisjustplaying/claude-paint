# Craquelure revamp (branch `cracks`)

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
     checkpoints (appended at the end of the state, format bumped to `PAINTCK2`). With 240 µm the weave
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
