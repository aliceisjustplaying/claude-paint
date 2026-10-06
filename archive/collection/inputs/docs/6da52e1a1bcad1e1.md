# Workflow: crop renders, checkpoints, speed (stream 3, branch `workflow`)

Goal: a fast feedback loop, so a painter can afford tiny details. Before this
branch, the only way to see a detail at full resolution was a whole 3200px
render (friedrich_moonrise_valley: 107–145 s here, under a load average of
20–130 from the other streams), and every run repainted every stage.

What landed:

1. **Crop renders**: `--crop x0,y0,x1,y1` (units) paints only that window,
   at the run's full resolution. They are fast, and close to a whole render
   (not exact; the reasons are below).
2. **Stages and checkpoints**: `--ckpt` saves the complete canvas state after
   every stage and `--resume <stage>` starts from one. Resuming is
   **bit-exact**, and stale checkpoints are refused.
3. **Speed**, with the output unchanged: an exact per-column memo for 1-D
   profiles (cuts mask cost in crops) and a dependency-ordered tile scheduler
   in place of phase barriers.
4. Not done: `--frames` (the "watch Claude paint" time-lapse).

All timings below were taken on the shared 10-core machine with the other
four streams running (load average 17–130). They are wall-clock times and
noisy; compare within a row, not across rows.

## The fast loop

```
cargo paint friedrich_moonrise_valley -- --full --crop 280,440,460,580 --ckpt   # once, 14 s
cargo paint friedrich_moonrise_valley -- --full --crop 280,440,460,580 --resume mist
                                                                          # 1.6 s per iteration
```

| moonrise, the two figures on the ledge | time |
|---|---|
| whole canvas, 3200px | 86–145 s (load 14–110) |
| crop 180×140 units (576×448 px) at 3200px, margin 40 | 13–14 s |
| the same crop, resumed from "mist" | 1.6 s |
| whole canvas, 1000px preview | 16–21 s |
| 1000px preview, resumed from "mist" | 1.1–2.7 s |

## Painter API

`paintings/src/run.rs` (module docs have the full list):

```rust
let o = Run::new("my_painting");
let mut rng = Rng::new(o.seed);
let mut c = o.canvas(|| st.prepare(o.width, 1.4, o.seed)); // loaded instead on --resume
let f = c.frame();                                          // whole-canvas frame for masks
let ridge = f.per_column(|x| h * 0.6 + 12.0 * n.get(x, 0.0)); // optional: exact memo
let sky = Mask::from_fn(f, |x, y| ...);                     // outside stages: always runs
if o.stage("sky", &mut c, &mut rng) {
    c.work(&sky, ...);                                      // skipped when resuming later
}
if o.stage("land", &mut c, &mut rng) { ... }
o.finish(&mut c, &mut rng, &Finish::aged(st.relief)); // or o.end(&mut c, &mut rng)
```

- `stage(name, c, state)` begins a stage and ends the previous one. Ending a
  stage prints its time, writes its checkpoint with `--ckpt` and stops the
  run with `--stop name`. It returns false when a resumed run should skip
  the stage.
- Rules: paint only inside stage blocks. Code between blocks runs on every
  run, resumed or not, so keep it to masks, fields and constants. Anything a
  stage hands to later stages must travel in the canvas or in the `Keep`
  state (usually the painting's `Rng`; `()` and pairs also implement `Keep`).
  Pass the same `Keep` value to `end` or `finish`: they close the last stage,
  and its checkpoint saves that state like any other (they used to save it
  empty, so resuming the last stage lost the RNG and appending a stage after
  it panicked). `Keep::restore` checks the saved length and returns an error;
  a checkpoint whose state doesn't fit is refused with a message.
- **Migration** for other streams' paintings: the old
  `if o.stage(&mut c, "x") { return; }` came after a stage's code. Now put
  `if o.stage("x", &mut c, &mut rng) {` before that code and `}` where the old
  marker was, and hoist any value a later stage uses (moonrise hoists the
  moon's position). `friedrich_moonrise_valley` and `friedrich_monk2` are
  converted. Their output is unchanged by the conversion, but see the
  palette fix below.
- `--crop` needs no painting changes. `Run::new` calls `paint::set_crop`,
  and `Canvas::new` picks it up. An explicit form is
  `Canvas::new_window(px, aspect, ground, Some(Crop { units, margin }))`.

## Crop renders: how they work

`Frame` gained a window: `w × h` are the pixels a buffer holds, at origin
`(x0, y0)` in the whole `full_w × full_h` canvas. `width()` and `height()`
stay whole-canvas units, and all px↔unit conversions go through
`Frame::{index, ux, uy, whole, clip, whole_index}`. For a whole canvas every
formula reduces to the old arithmetic, so the golden fingerprint did not move.

- **Masks stay whole-canvas**: `c.frame()` returns the whole frame and
  `c.window()` the buffer's. So stroke planning runs over the whole canvas
  with the same RNG sequence, the same tiles and the same stroke ids. Tiles
  whose footprint misses the window are skipped. The cost is that mask
  building and planning scale with the canvas, not the crop. See speed below.
- Brush geometry stays in whole-canvas pixels (`bristle::exchange`), and
  only buffer indices are translated. A crop therefore does the same float
  arithmetic as a whole render for every pixel it holds.
- Where a bristle's contact lies outside the window, there is no canvas to
  feel. The bristle is assumed to lay paint as usual (`GHOST_TOUCH` 0.8) and
  to lift none, so it enters the window about as spent as in a whole render.
  A contact cut by the window edge lays only the window's share there.
- Linen, grounds (`prime`), glazes, `apply`, relief and the save dither are
  pixel-local, so they are exact. Cracks grow over the whole canvas (the
  network is in mm) and only the window is rasterized, so they are exact too.
- The **margin** (default 40 units, `--margin`) is painted but not saved. It
  gives paint leveling (`settle` blurs), the brushes' contact surface and
  strokes that enter the window their context.

### Why a crop is close but not exact

A mark's look depends on the brush's history along its whole path: paint it
picked up and ridges it ploughed ahead of itself before entering the window,
and earlier strokes of the same tile's brush. That history depends on the
canvas outside the window, which a crop doesn't have. Long strokes (grounds
of 250–600 units, mist of 80–200, badger fusing) carry the most history.
Short strokes, dabs, the rigger, figures, trees and cracks match well.
Resolution of the numbers below: max/mean over RGB, in 8-bit levels.

| moonrise figures crop vs the same region of a whole render | mean | pixels > 8 levels | max |
|---|---|---|---|
| 1000px, margin 12 | 1.47 | 4.5% | 39 |
| 1000px, margin 40 | 1.11 | 2.5% | 40 |
| 1000px, margin 80 | 0.86 | 1.8% | 42 |
| 1000px, margin 160 | 0.56 | 0.9% | 41 |
| 3200px, margin 40 | 1.34 | 4.5% | 56 |
| 3200px, margin 80 | 1.01 | 3.0% | 48 |

Engine test `crop_matches_whole` checks this: linen and ground are exact,
brushwork is close, and the difference falls as the margin grows. For
scale: the `palette` history bug (below) alone moved the whole painting by a
mean of 1.6 levels. The 1000px preview and the 3200px render differ in kind:
`work` accepts a stroke center by reading the mask at that point, masks at
another resolution accept slightly different centers, and from there the
RNG sequence and every later mark differ.

Evidence (all in `~/tmp/workflow-75d81b41/`, viewed through
`scripts/peek`):
- `evidence_crop3200.png`: whole-render region | 3200px crop (margin 40) |
  difference ×8. The figures, grass, cracks and spruces are the same marks.
  The difference is low-amplitude and in the mist (long strokes), plus one
  faint mist band at the top.
- `cmp_m80.png`: the same at 1000px, margin 80.
- `cargo paint study_workflow` writes `out/study_workflow.png`, the same
  sheet for a small scene, and prints the times and the difference (here:
  whole 6.4 s, crop 1.8 s, mean 1.37 and max 35 levels; the error is in the
  badger-fused sky).

In a crop, `c.sample()`, `c.pixels()` and `c.surface_um()` see only the
window (`sample` clamps to it).

## Checkpoints

- File: `out/<stem>.<stage>.ckpt`. The stem is `<name>[_full][_crop]`,
  regardless of `--out`, and spaces in stage names become `_`. Contents:
  magic, a key=value header (name, stage, width, seed, crop and margin, the
  `Keep` state, a timestamp and three source hashes), then the canvas.
  The canvas part is the frame, crop window, color, relief, film, linen,
  size, the wet layer (volume, Mixbox latents, hiding and stiffness, dirty
  box) and the stroke counter. Per-pixel stroke ids and film floors are not
  saved, because every later stroke has a new id and never reads them. The
  brushes' contact surface is rebuilt from the relief. Sizes: 43 MB at
  1000px, about 440 MB at 3200px whole, 35 MB for the 3200px figures crop.
  Writing takes 0.02–0.05 s at 1000px.
- **Exact**: resuming from any stage, including one that ends with wet paint
  ("sky lay"), gives a byte-identical PNG. Checked for mist and "sky lay" at
  1000px and for mist in the 3200px crop.
- **Validation**: a checkpoint must match name, width, seed and crop.
  `--resume` also refuses it if the code that produced it has changed since
  it was saved. The error names what changed. The staleness model (since
  the `fixes-ux` round; before it, a stage hashed every line before the
  *next* stage call, so setup for a later stage staled the earlier one):
  - the engine (`crates/paint/src`) and `paintings/src` helpers (recursive,
    without `bin`), whole;
  - in the painting's file: every line up to the closing brace of the
    stage's own `if o.stage(..) { .. }` block (found by a small lexer that
    skips comments and literals), plus everything after the top-level item
    that holds it (helper functions below `main`, which the old prefix
    missed). Code between blocks after the stage's block is setup for later
    stages and doesn't count;
  - lines tagged for a later stage are left out: a line ending in
    `// ckpt: from <stage>`, or the lines between a `// ckpt: from <stage>`
    line and a `// ckpt: end` line. Stages before `<stage>` ignore them;
    `<stage>` and later ones count them. A tag naming no stage is an error.
    Only plain `//` comments are tags (not doc comments or strings).
  - A stage call not directly followed by its block falls back to the old
    rule (the lines before the ending call, or both files whole).

  Sound as far as the stage rules hold: code between blocks may build
  masks, fields and geometry but must not paint or draw from the `Keep`
  state (a resumed run draws from a fresh one there, so that was never
  byte-exact anyway). A tag is the painter's word; it is not checked.
  `--stale-ok` uses a stale checkpoint anyway; `--stale-ok --ckpt` also
  rewrites its fingerprints for the current code (header key `adopted`
  holds the original save time), so the next `--resume` needs no flag.
  Without `--ckpt` it says it didn't refresh and how to.
- **Loading** checks the geometry before allocating: frame arithmetic is
  checked for overflow, the crop (`keep`) and dirty boxes must be ordered and
  inside the buffer, and the scale and mm per unit finite and positive. A
  corrupt file is an `InvalidData` error, not a panic when saving later.

### A bug found by exactness: `Palette::mix` depended on history

The first resume from "sky lay" differed from the straight run: 0.02% of
pixels, by 1 level. The cause was that `Palette::mix` cached mixtures under
a quantized OKLab key but searched for whichever color reached the bucket
first. So a pile's mixture depended on which colors had been mixed before,
and a resumed run with an empty cache mixed differently. It now searches
the bucket's center, which makes the result a pure function of the key.
This changes output: per-stroke piles move within one 1/400 OKLab bucket,
moonrise moves by a mean of 1.6 levels, and it looks the same (compared
side by side). The golden was re-recorded (commit 3ca1c69). **palette.rs is
stream 1's file.** The change is one line in `mix`, and stream 1 should keep
the property "mix is a pure function of its input".

## Speed

Profile (macOS `sample`; moonrise at 1000px, and the 3200px ground stage):
`bristle::drag_on`/`exchange` is about 85% of busy CPU. The rest is dry and
settle, the Mixbox conversion and crack rastering. Rayon workers are idle
most of the time. Phases hold only 1–6 tiles (long strokes make big tiles),
the badger passes over the sky run 1–3 tiles at a time (the sky stage takes
as long on 1 thread as on 10), and the single drags (clouds, wisps, 100+
spruces) are sequential. In a crop at 3200px, the whole-canvas masks
dominated instead: Perlin noise in the painting's mask closures.

Wins taken, all with bit-identical output (golden unchanged; moonrise at
1000px byte-identical to the post-palette-fix render):
- `Frame::per_column(g)` tabulates a function of x at every pixel-column
  center. A mask calling `ridge(x)` per pixel then evaluates it once per
  column, and any other x falls through to `g`. Moonrise uses it for its 4
  profile lines. The 3200px crop's ground stage dropped from 13.2 s to 4.5 s
  and the whole crop from 24 s to 13 s.
- `sched::run_ordered`: `Canvas::work` used to wait at each of the four
  phase barriers. Now a tile starts as soon as every earlier tile whose
  footprint overlaps it has finished. Tiles with disjoint footprints commute
  exactly, so the result is the one-by-one order's. That rests on every
  stroke staying inside its footprint, which holds only for physically
  possible tools (a negative `length` made the bristle bend diverge far
  outside it). So `drag`, `touch`, `work` (and its cut-in tool) and `stipple`
  first check `Tool::validate`: finite fields, positive width, hair and run,
  at least one bristle, non-negative length, lay, splay and raggedness, and
  stiffness, pickup and push within 0..1. The bend relaxation rate is clamped
  to 0..1, and `drag_on`/`touch_on` clamp every pixel access to the stroke's
  own footprint as a second line of defense; debug builds assert the clamp
  never cuts anything. The mist stage (resumed,
  3 runs alternating with the old binary): 3.63 s against 3.9 s. The gain is
  limited by how few tiles there are and by this machine's load. I expect,
  but didn't measure, a larger effect on an idle machine and for passes of
  short strokes (more tiles).

Before/after, moonrise:

| | before (main) | after |
|---|---|---|
| 1000px whole | 18.4 s (load ~20) | 15.9–16.7 s (load ~46) |
| 3200px whole | 107 s (load ~40); 145 s after the palette fix (load ~110) | 86 s (load ~14), byte-identical to the post-fix render |
| 3200px crop, figures | — (not possible) | 13.3 s |
| 3200px crop, resumed from mist | — | 1.6 s |

What I'd do next for speed, in order: (1) exact micro-optimizations in
`exchange`, which needs line-level profiling; I didn't have line tables.
(2) Masks that evaluate lazily outside a crop window, so crops stop paying
for whole-canvas masks. (3) Anything faster for long badger passes changes
the physics order, so it is a design decision, not a safe win.

## Merge notes for the integrator

- `canvas.rs`: `Frame` has new fields (`x0, y0, full_w, full_h`). Use
  `Frame::new(w, h, scale)` instead of struct literals. `Canvas.f` is now
  the **buffer** (window) frame. Code that builds masks inside the crate
  must use `c.frame()`, not `c.f`; `check_mask` panics loudly otherwise. I
  changed `style.rs` (one line) and `tree.rs` (one line, `from_shape`).
  Buffer code that indexes `px/height/film/wet` by `self.f.w` keeps working
  in both modes. New buffer code that converts px↔units should use
  `f.ux/uy/index`.
- `handling.rs` (stream 2's file): `work` and `cut_in_edges` plan in
  `mask.f` (2 lines). The execution loop at the end of `run_plans` is now a
  closure passed to `sched::run_ordered` (the tile body is unchanged). If
  stream 2 changed that loop, apply their change inside `paint_tile`.
- `palette.rs` (stream 1's file): the one-line cache fix above.
- `rng.rs`: `Rng::state` / `Rng::from_state`.
- `lib.rs`: `pub mod checkpoint; mod sched;` and exports `Crop` and `set_crop`.
- `bristle.rs`: `Surf` carries the window; `exchange` clips to it and
  estimates contacts outside it. For whole canvases the arithmetic is
  unchanged.
- `crack.rs`: `raster_window`; `Canvas::crack` grows the network over the
  whole canvas.
- If you add state to `Canvas` or `Wet`, add it to `checkpoint.rs` and bump
  `MAGIC`. Otherwise resumes silently lose it.
- `.gitignore`: `out/*.ckpt`.

## Known issues

- A crop is close, not exact (see above), and its mask and planning cost
  scale with the whole canvas.
- Checkpoints of whole 3200px canvases are about 440 MB each. `--ckpt`
  writes one per stage, so use it with crops or at preview size.
- The `Run` stage API changed. Paintings on other branches that still use
  `o.stage(&mut c, "x")` won't compile until migrated (mechanical, above).
- `study_cracks` builds several canvases in one process; `--crop` applies
  to all of them, so don't crop it.
- `--frames` (time-lapse) is not implemented. The natural hook is
  `Run::stage` plus a `Canvas` callback after each `work` phase or every N
  strokes, saving a copy with the wet layer composited (a `dry()` on a
  clone).

## After merging main (color, strokes, stipple, form, motifs)

- **One scheduler.** `sched::run_ordered` now runs all tile work: `work` in
  the strokes stream's requested tile order (`tile_order`: Passages, Sweep,
  Scatter), which replaces `levelize`, and `stipple` in its phase order,
  which replaces its own phase runner. Both skip tiles that miss a crop
  window. The semantics are the strokes stream's (notes/strokes.md): every
  pair of overlapping tiles runs in the requested order, so parallel equals
  serial. Tiles no longer wait for a whole batch, and the dependency build
  uses a spatial index. `handling::tests::tiles_keep_their_order_where_they_overlap`
  (the strokes stream's test, ported) and `sched::tests` check it.
  Stipple output is the same on 2 and 10 threads.
- **Random draws no longer depend on the canvas.** Aiming (the color
  stream) makes a pile's recipe depend on what is under the stroke, and
  `Palette::remix` draws a varying number of values per recipe. A crop sees
  a different canvas outside its window, so its later strokes drew
  different random values: moonrise planned 571 strokes whole and 560
  cropped in one pass. Each pile's mixing jitter now uses its own generator
  forked with one draw, in `finish_plan` and in stipple's `paint_for`.
  Planning is identical again (every pass plans the same number of
  strokes). Output changes, so the golden was re-recorded for this reason
  only.
- **Stipple recipe memo.** It stored the recipe for whichever exact color
  first hit a coarse key; it is now computed from the key's center (the
  same fix as `Palette::mix`). `Palette::mix` and `Palette::aim` on main
  already compute from their keys, so they are order-independent.
- **Crop-aware new code.** `stipple` plans in the mask's whole frame with
  whole-canvas footprints; `touch_on` returns buffer bounds and, outside
  the window, estimates the deposit. `Canvas::under` looks only at pixels
  the window holds, falling back to the nearest one. `stroke_under` ignores
  points that are on the canvas but outside the window. Form fields and
  the new mask operations work on whole-canvas frames, like masks. The
  `Frame { .. }` literals in form.rs and mask.rs tests became `Frame::new`.
- **Stage API.** moonrise, monk2, study_form and study_stipple use stage
  blocks. `Run::end(&mut c)` closes the last stage for programs that don't
  call `finish`. The other studies have no stages and run unchanged; every
  bin runs whole and with `--crop` (except study_cracks, which builds
  several canvases).

Verification after the merge (1000px, margin 40, diff in 8-bit levels):

| crop vs the same region of a whole render | mean | > 8 levels | max |
|---|---|---|---|
| moonrise figures 280,440,460,580 | 1.43 | 5.1% | 96 |
| study_stipple sky, both passes 300,150,500,300 | 0.59 | 0.003% | 10 |
| study_stipple mist over the ridge 300,560,500,625 | 1.98 | 6.0% | 28 |
| study_stipple same, margin 250 (before the rng fork) | 0.64 | 0.3% | 13 |
| study_stipple ridge only (`--stop ridge`) | 0.005 | 0% | 1 |
| study_form boulder 150,100,350,250 | 0.63 | 2.1% | 61 |
| study_form ranges 650,450,850,600 | 0.60 | 0.35% | 33 |
| study_form cliff 100,450,300,600 | 0.19 | 0.007% | 18 |

The mist stipple is margin-limited: a dip serves 16 touches, so touches
outside the window affect the load of those inside it. study_form crops
take 11–12 s against 31 s whole, because its form fields (SDFs per pixel)
are computed over the whole canvas, like masks.

Resuming stays byte-identical: moonrise from "sky lay" (wet), "mist" and
"figures", and study_stipple from "pass 1".

## Fixes from amnesia round 2 (branch `fixes-ux`)

From the painters' friction lists (notes/amnesia2.md items 9, 10, 11, 12).
Resume is still byte-exact (study_stipple at 400px: whole vs resumed from
"pass 1", `cmp` identical); golden unchanged.

**Stage names and flags** (`paintings/src/run.rs`):
- Names match by `run::key`: case, spaces, underscores, hyphens and
  slashes don't matter. `--stop far_range` stops at "far range" (it
  used to run to the end without a word), and `--resume "Far Range"`
  finds `out/<stem>.far_range.ckpt`.
- `Run::new` is `#[track_caller]` and reads the painting's own file for its
  `.stage("literal", ..)` calls. An unknown `--stop` or `--resume` is an
  error before anything is painted, and it lists the stages. With a stage
  named by a variable (study_form's loop), the check happens when the run
  ends instead: `end`/`finish` refuse to finish a run whose `--stop`
  never matched. A missing checkpoint lists the ones saved for this run.
- `--resume X --stop X` saves X's checkpoint as an image and stops
  ("nothing painted"). It used to repaint to the end. `--stop` at a stage
  before the resume point is an error.
- Staleness: see the model under Checkpoints above. Painter's view: build
  a later stage's geometry right before that stage (after the earlier
  block), or tag it `// ckpt: from <stage>` where it has to sit at the top.
  `--stale-ok --ckpt` adopts the checkpoint.
- Tests: `run::tests::{stop_and_resume_names, unknown_stage_fails_early,
  stale_ok_refreshes_with_ckpt, stage_fingerprint_model}` and
  `run::source::tests` (lexer, blocks, tags). Under `cfg(test)`, `die`
  and `--stop` panic instead of exiting, so the flows are testable.

**Closures** (winter #13, coast #7, mountains #6): `Fbm` is `Copy`. Its
octave tables are built once per (seed, octaves, persistence) and shared
(leaked, about 1 KB per octave), and its output is unchanged (tested against
the noise crate). `Frame::per_column` returns `&'a impl Fn(f32) -> f32`, a
`Copy` reference: it is still called `ridge(x)`, and its table lives for the
rest of the program. A `move` closure that captures only these is `Copy`
too, so one profile or noise can feed a mask and any number of color
closures with no `let n = &n;`. `Mask` and `Form` are still not `Copy`
(capture `&mask`).

**`Mask::roughen`** (winter #10, #16): `amount` and `edge` are now canvas
units. The contour moves by about `amount` units (fbm, `period` units), and
the new edge ramps over `edge` units. It works through the signed distance
to the 0.5 contour, so a hard `Shape` mask roughens like a soft one. Pixels
farther than `amount + edge` from the edge keep their value. The old
behavior (mask-value units, a no-op on hard masks, `edge = 1` turned 0 into
0.16 everywhere) is gone. No built painting used it. Test:
`mask::tests::roughen_moves_hard_edges_in_units` (at two resolutions).

**form.rs docs**: `Sdf::block`'s `size` is the whole extent (it spans
`c ± size/2`; an example seats a tor on the turf line), and
`Sdf::ellipsoid`'s `r` is radii. Every constructor and `Ridge` builder now
states its units. The docs also say `Ridge` reaches `depth` below its crest
whatever stands in front of it (mountains #9).

## Open bugs

### Fixed: a gesture with a NaN point laid one dab and said nothing (coast #17)

Fixed after merging `tip`: `Gesture::validate` and `Touch::validate` reject
non-finite numbers, and `drag_on`/`touch_on` (every painting path) panic
with e.g. "Gesture point 10 is not finite: (505.5, NaN)". The history:

**Symptom** (coast painter): an 11-point U-shaped `drag` (a coil of rope,
`Tool::rigger(0.5)`) laid nothing, while straight strokes nearby did.

**Cause**: the painter's points. `paintings/fresh2/fresh2_coast.rs` builds
`y = py + 1.3 + drop * a.sin().powf(0.8)` for `a = PI * k / 10`. In f32,
`PI.sin()` is −8.7e-8, so `powf(0.8)` is NaN at k = 10. The only half
that painted (bottom to left end) was the only one without the last point.
**Engine side** (bristle.rs, `drag_on`): the arc length `total` is NaN,
`nsteps = ((total / step).ceil() as usize).max(1)` is 1 (NaN casts to 0),
and `(k * step).min(total)` ignores the NaN. So the brush takes a single
step at the start and lifts: 12 pixels at 1000px, against 106 for the same
U with a finite last point. `footprint` doesn't catch it either, since
`f32::min`/`max` skip NaN. `Tool::validate` checks the tool but nothing
checks the gesture.

**Fix to apply after merging `tip`** (bristle.rs is that stream's file):
in `Canvas::drag` (and `touch`/`Touch`, and in `Canvas::work`'s planned
gestures via `footprint_checked`), reject non-finite points the way an
invalid `Tool` is rejected. Panic with the point's index and value, e.g.
`"Gesture point 10 is not finite: (505.5, NaN)"`. Also check that
`pressure`, `attack`, `release` and `swell` are finite. Repro:
`crates/paint/tests/curved_drag_nan.rs`. Three tests pin the diagnosis,
and `a_nan_point_is_an_error` is `#[ignore]`d (it expects the panic to
mention "point 10"). Un-ignore it with the fix.

## Tests: two tiers (Round 6)
- **Everyday:** `cargo test --workspace`. The test profile is optimized
  (`[profile.test]` in Cargo.toml: opt-level 2, debug assertions and
  overflow checks on, not incremental), so the whole suite runs in about a
  minute on the M3 Pro (it took ~10 minutes unoptimized), plus about 50 s
  of build when the engine changed. The golden scene is recorded with this
  profile (`UPDATE_GOLDEN=1 cargo test -p paint`).
- **Before a merge:** also `cargo test --release -p easel --test hand_time`
  (Alice's logs replayed in the release build, hashed). The release
  profile is deterministic (`incremental = false`, `codegen-units = 1`):
  two clean builds give byte-identical binaries and renders, so a hash
  mismatch is a real change, not build noise.
