# Stream 2: strokes (entropy, coverage, presets)

Branch `strokes`. The user's complaint: "too straight, too neat, too
horizontal = digital; we need so much more entropy", plus "the same sky three
times" (the style presets painted the sky, not the painters), findings item 7
(ground shows through, strokes not seeded past edges) and item 8 (the blender
in shuffled tile order drags paint across gradients).

## What changed

### Stroke geometry (`handling.rs`, `hand_trace`, `break_stroke`)
Every stroke used to be a 7-point streamline of the angle field with a tiny
constant bend, uniform length, constant pressure. Now each stroke is drawn
the way a hand makes it:

- **Arcs and S-curves** (`curve(bow, wave)`): curvature per stroke, sagitta
  ≈ `bow`·length (sd). Arcs mostly bulge away from the pivot (a right hand
  below and right of the brush), like wrist and elbow swings; `wave` of the
  curved strokes are S-shaped (curvature changing sign along the stroke).
- **Criss-cross** (`cross(angle)`): two stroke families at ± angle to the
  direction field.
- **Drift** (`drift(amount, scale)`): the direction wanders smoothly across
  a passage (a noise field added to the angle field), so neighboring strokes
  agree but the passage isn't ruled.
- **Length tails** (`tail(share)`): besides the uniform range, short dabs
  (0.2–0.7 × min, loaded with a proportionally smaller touch of paint) and long
  sweeps (1–1.7 × max).
- **Broken strokes** (`broken(share)`): the brush lifts partway and comes
  down again a little off the line (offset, gap or overlap, small turn),
  carrying on with the paint left on the brush.
- **Pressure swell** (`swell(sd)`): a few pressure knots along each stroke.
  This needed a small additive change in `bristle.rs`: `Gesture::swell(knots)`
  (empty = unchanged behavior, so direct drags and motifs are unaffected).

### Placement and order (`place`, `Order`, `tile_order`, `levelize`)
- Centers were an even jittered grid, globally shuffled. Now the area is cut
  into **passages** (≈1.6 stroke lengths square). Each passage gets its own
  lattice aligned with its stroke direction: rows of strokes laid side by
  side, jittered along the row freely and across the row by ±35% of the row
  spacing (so neighbors always overlap and no ground shows between rows).
  Lattices of adjacent passages don't line up, so passages meet raggedly.
- **Clump** (`clump(amount)`): a slow density field crowds strokes in some
  places (it only adds strokes; thinning opened gaps).
- **Order** (`order(Order::…)`, `sweep(angle)`):
  - `Passages` (default): passage by passage in random order, the hand moving
    across each passage laying strokes side by side. Moving on to a new
    passage is always a trip to the palette (the brush no longer carries dark
    paint from one passage into a light one).
  - `Sweep(angle)`: the whole area in one sweep, band by band (`FRAC_PI_2`
    = top to bottom). Tiles are painted in that order too.
  - `Scatter`: anywhere, random order (the old behavior).
- **Parallel safety.** The four fixed checkerboard phases with random order
  are replaced by `levelize`: given any total order of tiles, each tile goes
  into the batch after the latest earlier tile whose footprint it overlaps.
  So every pair of overlapping tiles is painted in the requested order, and
  a parallel run equals a serial run in that order. Footprints stay exact
  (`finish_plan`/`footprint` unchanged); the neighbor window searched is
  measured from the footprints. Unit test `levelize_keeps_overlapping_tiles_in_order`;
  `scene_is_deterministic_across_thread_counts` still passes.
- `run_plans` and `finish_plan` keep their signatures. `Plan` gained two
  fields (`swell`, `passage`), set to defaults in `finish_plan`'s last line:
  the only line in `finish_plan` I touched (a trivial merge with the color
  stream).

### Coverage (findings item 7)
- Centers are seeded half a stroke length past the canvas edges (the mask is
  read clamped, so a region touching the edge continues past it). Test
  `work_covers_the_edges`: >97% of a 2.5% border covered at coverage 4.
- With `clip(true)`, strokes may start outside the region and brush into it
  (accepted if any part of the path is in the region; color and passage are
  taken at the entry point). Measured on a clipped panel (probe, coverage
  2.5, `broad()`): 92.3% → 96.2% covered. On a whole canvas 96% → 96.7%.
- With the moonrise sky recipe (two lay-in passes), no ground shows at 1000px
  or 3200px apart from a few isolated flecks (see known issues).

### Style presets (`style.rs`)
The presets are now a vocabulary with a hand but no look of their own:
- `broad()`: long elbow arcs (`curve 0.06`), drift 0.22 rad over 350 units,
  tails, some broken strokes, swell. No longer ruler-horizontal; the
  painter sets the direction.
- `body()`: wrist strokes, more bowed and broken, drift over 150 units.
- `detail()`: nearly straight, small drift, no breaks.
- **new `hatch()`**: short hatched strokes side by side ("like a closely woven
  textile", NG pp.49–50), clumped, for conifers, grass, far slopes.
- `glaze()`: like broad, softer.
- `blend()`: light crossing strokes (±0.2 rad), worked **top to bottom**
  (`sweep(FRAC_PI_2)`) so the badger never drags paint back up a gradient
  (findings item 8). Override with `.order(Order::Passages)` etc.
- Brushed ground in `prepare`: the priming hog still runs mostly across the
  canvas but wanders (0.25 rad over 450 units), bows, crosses ±0.1 rad and
  breaks off. `BRUSHED_UM_PER_LOAD` recalibrated 195 → 318 (more strokes now
  reach the edges): asking 30 µm lays 28, asking 90 lays 97.
- `Handling::ruler()` switches all of it off (straight, even, scattered), for
  mechanical work or comparison.

## API (painter's view)

```rust
use paint::{Order, Style};
use std::f32::consts::FRAC_PI_2;
let st = Style::friedrich();

// a sky laid in with elbow arcs, direction wandering, then a second pass
// in crossing strokes, fused top to bottom
c.work(&sky, &st.broad().color(sky_col).coverage(4.0), 1);
c.work(&sky, &st.broad().color(sky_col).cross(0.2).length(60.0, 150.0), 2);
c.work(&sky, &st.blend().unwrap(), 3);                // sweeps top to bottom

// a slope in body color, strokes following the fall line, strongly bowed
c.work(&hill, &st.body().color(earth).angle(|x, _| fall(x)).curve(0.12, 0.4), 4);

// firs: short hatching, steep
c.work(&firs, &st.hatch().color(dark).angle(|_, _| 1.4).cross(0.25), 5);

// a gradient worked left to right, band by band
c.work(&sea, &st.broad().color(sea_col).sweep(0.0), 6);

// the old ruler look
c.work(&area, &st.broad().ruler(), 7);
```

Knobs on `Handling` (all builder methods): `curve(bow, wave)`, `cross(angle)`,
`drift(amount, scale)`, `tail(share)`, `broken(share)`, `swell(sd)`,
`clump(amount)`, `order(Order)`, `sweep(angle)`, `ruler()`. Defaults in
`Handling::new` are hand-like (curve 0.05, drift 0.12 rad / 300 units, tail
0.12, broken 0.06, swell 0.15, clump 0.3, `Order::Passages`).

## Evidence
- `cargo paint study_strokes` (`paintings/src/bin/study_strokes.rs`): the same
  twilight gradient, the moonrise sky recipe, five ways: ruler (old), `broad()`
  now, arcs, criss-cross, drift. Top row lay-in, bottom row fused with the
  badger. Renders: `~/tmp/strokes-e17f4445/study5.png` (1000px),
  `study_3200.png` (3200px; crops `s32_top.jpg`, `s32_bot.jpg`).
  - Lay-in: ruler reads as horizontal bands. The new panels read as hand
    work: broad as soft wandering arcs, arcs as cloud-like puffs, criss-cross
    as a diagonal weave, drift as bands that wander.
  - Fused: the old random-order blender (panel 1) leaves horizontal smears
    and drags dark paint down the edge (finding 8 reproduced). The swept
    blender gives smooth, thin gradients with quiet life. At 3200px the
    new panels show bristle striations running in gentle arcs. It still
    reads as thin smooth paint, not noise.
- `friedrich_moonrise_valley` (unchanged program) before/after:
  `~/tmp/strokes-e17f4445/moonrise_before.png`,
  `moonrise_after.png` (1000px), `moonrise_after_full.png` (3200px). The sky
  lost its ruled horizontal banding and is a softer, mottled twilight; the
  mist, ridges and ledge are otherwise the same picture.
- `study_ground` after: `ground_after.png`. The top ground's striations
  wander and bow, and thin paint pools in them in wavy rather than ruled lines.

## Edges, blending and relative color (round 3, branch `fixes-paint`)
- **`hug` (default on):** stroke centers seeded just outside a region
  (within half a brush, across the stroke direction) move onto its edge,
  plus a quarter brush further in when unclipped. The edge gets as many
  strokes as the inside. Before, it got half, and a thin clipped band
  (coast #3's horizon) kept pale slivers of what was under it.
- **Carried in (unclipped, not blenders):** a stroke seeded outside the
  region that crosses its edge at more than 30° is kept. It is trimmed to
  the part inside plus a quarter brush and pulled from the edge inward, so
  the brush goes down at the edge fully loaded.
  `.hug(false)` turns both off (the old placement).
- Test `edges_are_covered_like_the_inside`; over six seeds
  (`probe_edges_over_seeds`), the share of a 5-unit edge band still reading
  as ground: body pass, 60-unit band unclipped 0.012 → 0.002; disk
  unclipped 0.012 → 0.0008; clipped cases unchanged (≈ 0.002); a 15-unit
  clipped band under the broad brush 0.13 → 0.02 (unclipped 0.29 → 0.015).
  Paint reaching past an unclipped edge is denser within a brush width;
  beyond it (12–30 units) it is about the same (disk 0.14 → 0.18 share of
  pixels darkened, band 0.002 → 0.008).
- **`Style::blend()` is clipped** to the mask it works (soft where the mask
  is soft). A badger no longer drags wet paint across a passage's edge
  (coast #5, mountains #14), where later passages showed it in their gaps
  (mountains #9). To fuse across an edge on purpose, give it a mask that
  spans the edge, or `.clip(false)`. Test `blender_stays_in_its_region`:
  the light passage above a blended dark one darkens by 0.0000 L (0.58
  with `clip(false)`).
- **`color_over(|x, y, under| ..)`:** a color field relative to the
  canvas. `under` is what is under the stroke before the pass (judged along
  it, as the aim does), so it's deterministic and in a crop judged by what
  the crop holds. `paint::shift(c, dl, da, db)` moves a color in OKLab:
  `st.body().color_over(|_, _, u| shift(u, -0.08, 0.0, -0.03))` is "the snow
  here, darker and bluer". Test `color_over_sees_the_canvas` checks this over
  a snow field from bright to dull: ΔL −0.073 to −0.077 (strokes) and
  −0.050 to −0.063 (stipple), bluer at both ends.
- Ground flecks *inside* dark passages (mountains #3) come from the bristle
  model ploughing paint off the weave's peaks, not from placement. They
  vanish at coverage ≈ 4 and under a dark underpainting (bristle.rs; not
  changed here).

## Known issues / next
- Isolated bare-ground flecks and short scraped "comets" remain in lay-ins;
  they show in the old ruler panel too and come from the bristle model (a
  brush ploughing wet paint off the peaks), not placement.
- An occasional stroke's palette pile lands far from its target (seen as a
  grey stroke in the light part of the drift panel: its pile was
  `[0.40, 0.19, 0.11]` where neighbors got ≈`[0.5, 0.42, 0.33]`). That is
  `Palette::remix` jitter in `finish_plan`'s paint choice (color stream).
- Criss-cross at large angles across a steep gradient drags dark paint down
  (a real brush would too): use small angles on gradients, or `sweep`.
- `coverage` still means nominal layers of the full brush width; lean paint
  at light pressure hides less, so thin passages want coverage ≈ 4 (as the
  moonrise sky uses) or a second pass.
- `coverage(0.0)` disables a pass: `work` returns before planning, for a
  painting or a blending handling. Negative or non-finite coverage panics
  (it used to be clamped up to 0.05, so a pass "turned off" still painted).
- `Sweep` serializes tile rows (two phases per band), so swept passes run
  with less parallelism; fine for blending, slower for big lay-ins.
- Next: per-passage variation of the knobs themselves (a painter's hand
  changes as it tires or speeds up); a `Hand` profile per painter in `Style`
  (handedness, pivot); tie the dab share to the stipple stream's marks.
