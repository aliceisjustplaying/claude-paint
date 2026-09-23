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

## Fix

`brush_ground` in `style.rs` brushes the top ground the way a primer brushes
out a priming, in two passes over the wet paste:

1. **Spread**: the hog in crossing strokes (`cross 0.5`) of 100–250 units.
   Their direction drifts patch by patch: ±0.7 rad on a ~90-unit (40 mm)
   noise field, across on average. The paste is pushed a little ahead of the
   bristles (`push 0.15`), not ploughed into ridges.
2. **Lay off**: the clean hog (`blender()`) drawn lightly (pressure
   0.35–0.55, coverage 3.5) through the wet paste along the same direction.
   It is held low, so it skims the paste (`push 0.03`).

The thickness calibration is refit to the new handling: load =
`(um / 364)^(1/1.24)`. `brushed_ground_honors_thickness` asks 30/60/90 µm
and gets 30.5/59.8/89.1.

### Round 2: the dotted contours (integrator review)

Round 1 (commit f3fd2cb) removed the grain but left one-pixel orange dotted
contours in thin skies at 3200 px. It also drifted 2.2 rad over 150 units,
which read as swirling arcs (`round1_near_sky_3200.jpg`,
`round1_sky_dashes_zoom.jpg`). The root cause was **ploughing**. The hog's
own `push 0.3`, on both passes (the clean lay-off brush at 3.5× coverage
too), pushed the wet priming into ridges at the bristle and stroke edges. At
one bare pixel the ground was 520 µm thick against the 240 µm the three
layers ask for, rising ~280 µm within 0.7 mm (`tests::diag_sky_bare_pixels`,
`DIAG_PX=217895`). A thin sky over such a crest levels off it (the fine band
levels in ~7 s, Orchard's τ = 3η(λ/2π)⁴/(σh³)), so the crest keeps 0–9 µm.
That draws a one-pixel line of ground.

I checked and ruled out:
- the look-and-fill pass: every bare pixel had wet paint and full cover;
- sharing the ploughed paint bilinearly: bare pixels 347 → 328;
- a residual film on crests in `settle`, as the h³ drainage stall
  ∛(3η(λ/2π)⁴/(σ t_set)) ≈ 6 µm. It avoids exact zeros, but 6 µm of pale
  paint over red ground still reads as bare, so I dropped it.

Laying off is done with a light, low brush that skims the paste, so the
lay-off push went to 0.03 and the spread's to 0.15. Measured over seeds 23,
1 and 5 at 3200 px (`sky_bares_ground`), a blended thin sky leaves this many
bare pixels:

| ground | bare pixels |
|---|---|
| old brushed ground | 98 |
| round 1 | 883 |
| round 2 | 164 |

The remaining ones are thin spots in the sky's own strokes, not crests. They
average 0.57 coats laid against 2.1 elsewhere, at average relief, and the
knife-only ground has them too.

## Rationale and sources

- The top ground was brushed and its bristle striations "shine through the
  thin upper paint layers" [KÖR p.284, in `notes/research/friedrich_materials.md`
  line 25]. So a texture stays; its mechanical regularity goes.
- A period recipe for priming canvas lays the priming "up and down the canvas
  with as long a stretch as possible, then horizontally, and afterwards
  perpendicularly again … finishing off horizontally", all while wet
  (*How To Prepare The Canvas*,
  https://chestofbooks.com/home-improvement/repairs/painting/Cyclopedia/How-To-Prepare-The-Canvas.html).
  This is an analog: it describes a scene painter's distemper priming, not
  Friedrich's oil ground. It supports cross-brushing and a final lay-off
  pass. The drifting direction and the low-push lay-off are my assumptions.
- The roller texture stays the `friedrich_early` (Berlin) default, where
  CATS p.127 describes it. For Dresden, KÖR describes brushing, so the brush
  stays the Friedrich default.

## Measurements

Height field, 3200 px, a 400 × 300 unit window, seeds 1, 23 and 5
(`print_ground_grain`). *Fine* is the mean squared slope down the columns
over along the rows (1 = no preferred direction); *coarse* is the same after
a ~1 mm blur (the wood-grain scale). *RMS* is the RMS slope in µm/mm.

| ground | fine | coarse | RMS |
|---|---|---|---|
| linen + the two knife layers | 1.5 | 2.5 | 54 |
| old brushed ground | 3.6–4.1 | **7.4–8.5** | n/a |
| round 1 | 2.0–2.4 | 2.2–2.8 | 99–107 |
| round 2 | 1.73–1.76 | 2.04–2.50 | 63–68 |
| `friedrich_early` (roller) | 0.8 | 1.2 | 26 |

## API

No new API. `Style::friedrich()` and any custom `Ground { apply: Apply::Brush, .. }`
(e.g. `paintings/fresh2/fresh2_winter.rs`) get the new ground. Every easel
session with `style="friedrich"` gets it too.

## Tests

- `friedrich_ground_has_no_horizontal_grain` (in `crates/paint/tests/ground_grain.rs`,
  1200 px, debug profile): fine < 2.2, coarse < 3.0, and the RMS slope must
  stay > 1.2× the knife layers' (striations kept). Round 2 gives 1.29 /
  1.61, RMS 1.5×. The old ground fails (2.63 / 5.6).
- `sky_bares_ground` (ignored: ~1 min in release, far longer in debug):
  asserts new ≤ 2 × old + 20 bare pixels against the old recipe, rebuilt in
  the test (`old_brushed`). Run it with
  `cargo test --release -p paint --test ground_grain -- --ignored sky_bares --nocapture`.
- Diagnostics (ignored): `print_ground_grain`, `save_ground_crops`
  (`GRAIN_OUT=dir`) and `tests::diag_sky_bare_pixels` (the wet state of
  bare pixels before the bake; `DIAG_PX=i` prints the ground around a
  pixel).
- `cargo test --workspace` passes.
- **Golden re-recorded** in commits f3fd2cb and 985b948: the golden scene is
  prepared with `Style::friedrich()`, so its top ground changed on purpose.

## Evidence (`notes/loop2_grain/`)

Replays of `notes/amnesia4/easel4_near.lua` at `--width 3200`: sky crop
`820,20,1000,200`, snow crop `820,310,1000,450`. Counts are orange
bare-ground pixels (r − b > 45 and r > 120, the canvas edge excluded).
- `before_near_sky_3200.jpg` → `round1_near_sky_3200.jpg` →
  `after_near_sky_3200.jpg`: horizontal streaks, then swirls and dashes
  (574 px), then a soft sky (14 px; the old ground had 1).
- `before_near_snow_3200.jpg` → `after_near_snow_3200.jpg`: the wood-grain
  becomes a faint texture in mixed directions (192 → 166 px, all in the
  dotted arcs at the top that were already there before).
- `before_bare_ground_lit.jpg` → `after_bare_ground_lit.jpg`: the bare
  ground lit by `relief(0.35)`.
- `after_near_1000.jpg`: the whole painting; compare
  `notes/amnesia4/easel4_near.jpg`.
- Study sheet: `cargo paint study_ground` (bare ground band on top).

## Known issues

- The dotted arcs along the top of the snow crop predate this branch
  (they are in the before render). They're probably the same crest drain
  over the snow's own impasto ridges; I didn't touch them.
- A thin blended sky still leaves ~1.7× the old ground's bare pixels. These
  come from thin spots in the sky strokes, not from the ground.
- The loop-1 near session isn't in this repo, so I checked only
  `easel4_near.lua`. The fix is in the style, so it applies to both.
