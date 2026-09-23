# Loop 2: the horizontal wood-grain (tool defect)

Branch `loop2-grain`. The critic's defect, on the loop-1 near painting and
`notes/amnesia4/easel4_near.lua`: "a horizontal wood-grain streaking covers
the sky and snow, which looks like the tool rather than a painting choice"
(also in `notes/review_scores_loop0_raw.md` and
`notes/review_scores_loop1_raw.md`, line 7).

## Cause

The Friedrich top ground (`Apply::Brush` in `Style::prepare`,
`crates/paint/src/style.rs`) was laid with a 40-unit (17.6 mm) hog in long
strokes (250–600 units) at `angle 0`, `angle_jitter 0.04`, `cross 0.1` and
coverage 3.5. The overlapping stroke edges set as parallel ridges a few mm
apart across the whole canvas. `relief()` lit them through every thin sky
and snow passage. It wasn't the linen: the weave alone measures isotropic,
and the knife layers level it.

Measured on the ground's height field (`crates/paint/tests/ground_grain.rs`,
3200 px, a 400 × 300 unit window, seeds 1, 23 and 5). *Fine* is the mean
squared slope down the columns over along the rows (1 = no preferred
direction). *Coarse* is the same after a ~1 mm blur: the stroke-edge ridges
that make the wood-grain.

| ground | fine | coarse |
|---|---|---|
| linen + the two knife layers | 1.5 | 2.5 |
| old brushed top ground | 3.6–4.1 | **7.4–8.5** |
| new brushed top ground | 2.0–2.4 | 2.2–2.8 |
| `friedrich_early` (roller) | 0.8 | 1.2 |

## Fix

`brush_ground` in `style.rs` brushes the top ground the way a primer brushes
out a priming, in two passes over the wet paste:

1. **Spread**: the same hog and paste, in crossing strokes (`cross 0.5`) of
   150–400 units. Their direction drifts patch by patch: ±1.1 rad on a
   ~150-unit (66 mm) noise field, across on average.
2. **Lay off**: the clean hog (`blender()`), drawn lightly (pressure
   0.35–0.55, coverage 3.5) through the wet paste along the same drifting
   direction. This levels the spreading's ridges and leaves fine, broken
   striations.

The layer still has a direction locally (the source says striations show),
but no canvas-wide parallel ridges. The thickness calibration changed with
the handling: the load is now `(um / 296)^(1/1.25)`, fit to measurements at
30, 60 and 90 µm. `brushed_ground_honors_thickness` asks 30/60/90 µm and gets
29.9/59.8/90.6.

**Rationale and sources.**
- The top ground was brushed and its bristle striations "shine through the
  thin upper paint layers" [KÖR p.284, in `notes/research/friedrich_materials.md`
  line 25]. So the texture stays; only its mechanical regularity goes.
- A period recipe for priming canvas lays the priming "up and down the canvas
  with as long a stretch as possible, then horizontally, and afterwards
  perpendicularly again … finishing off horizontally", all while wet
  (*How To Prepare The Canvas*,
  https://chestofbooks.com/home-improvement/repairs/painting/Cyclopedia/How-To-Prepare-The-Canvas.html).
  This is an analog, not Friedrich evidence: it describes a scene painter's
  distemper priming. It supports cross-brushing first and a final lay-off
  pass. The drifting direction is my assumption (a hand-brushed priming,
  worked patch by patch).
- The roller texture stays the `friedrich_early` (Berlin) default, where
  CATS p.127 describes it. For Dresden, KÖR describes brushing, so the brush
  stays the Friedrich default.

## API

No new API. `Style::friedrich()` and any custom `Ground { apply: Apply::Brush, .. }`
(e.g. `paintings/fresh2/fresh2_winter.rs`) get the new ground. Every easel
session with `style="friedrich"` gets it too.

## Tests

- `friedrich_ground_has_no_horizontal_grain` (in `crates/paint/tests/ground_grain.rs`,
  1200 px, debug profile ~30–60 s): fine < 2.2, coarse < 3.0, and the
  brushed ground's RMS slope must stay > 1.2× the knife layers' (the
  striations must not be leveled away). New: 1.53 / 1.80. The old ground
  failed it (2.63 / 5.6).
- Diagnostics (ignored): `print_ground_grain` (the table above),
  `save_ground_crops` (lit bare-ground crops, `GRAIN_OUT=dir`) and
  `sky_bares_ground` (see Known issues).
- `cargo test --workspace` passes.
- **Golden re-recorded** (`crates/paint/tests/golden_scene.txt`, commit
  f3fd2cb): the golden scene is prepared with `Style::friedrich()`, so its
  top ground changed on purpose.

## Evidence (`notes/loop2_grain/`)

Replays of `notes/amnesia4/easel4_near.lua` at `--width 3200`: sky crop
`820,20,1000,200`, snow crop `820,310,1000,450`.
- `before_bare_ground_lit.jpg` / `after_bare_ground_lit.jpg`: the bare
  ground lit by `relief(0.35)`. Before: long wavy parallel ridges across the
  whole width. After: arcs in varied directions, a hand-brushed layer.
- `before_near_snow_3200.jpg` / `after_near_snow_3200.jpg`: the wood-grain
  in the snow is gone. What shows now is a faint criss-cross of brushing.
- `before_near_sky_3200.jpg` / `after_near_sky_3200.jpg`: the sky no longer
  sits on parallel streaks. It shows arcs of brushing (partly the sky's own
  strokes). See Known issues.
- Study sheet: `cargo paint study_ground` (bare ground band on top, then
  thin blue, stiff white and a glaze over it) shows the new ground unchanged
  in code.
- `after_near_1000.jpg`: the whole painting. Compare
  `notes/amnesia4/easel4_near.jpg`: the horizontal streaks in the sky are
  gone at picture scale.

## Known issues

- **Bare-ground contour dashes in thin blended skies at 3200 px.** In the
  sky crop, 574 px (0.18%) of saturated ground color show as one-pixel
  dotted contours along some ridges. The old ground had 1 px
  (`after_sky_rings_zoom.jpg`; counted as r − b > 45 and r > 120). The snow
  crop is unchanged (192 → 193 px). They are invisible at 1000 px.
  `sky_bares_ground` isolates it. A thin broad sky over the old brushed
  ground leaves 532 bare px, and the badger blend closes nearly all of them
  (→ 2). Over the new ground the blend closes few (392 → 347). The blender
  runs across the canvas. The old ridges ran with it, so it smeared paint
  along them, while the new ones cross it and keep bare crests. Things I
  tried that didn't fix it: a residual film on crests in `settle` (reverted,
  no effect), softer paste, lighter spreading, and a firmer or longer lay-off
  at the same thickness (a firmer lay-off only helped by laying less paste).
  Next: find where the thin film goes to zero along a contour (bristle
  pickup on crests across the stroke, `bristle.rs` around line 1186, or the
  wet bake). The rings' one-pixel width suggests a threshold, not a physical
  thinning, and it probably also makes the dotted arcs already present in the
  old snow crop.
- The loop-1 near session isn't in this repo, so I checked only
  `easel4_near.lua`. The fix is in the style, so it applies to both.
- The drifting direction field is visible as swirling arcs in the bare
  ground at close range. If a critic reads that as a pattern, lower the
  drift amplitude (2.2 → 1.6 in `brush_ground`) at the cost of a little more
  horizontal bias.
