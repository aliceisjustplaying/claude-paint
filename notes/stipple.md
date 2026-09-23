# Stream 6: stippling

Friedrich stippled his skies, mist and distant hills, which "enhance[s] the
transparency and light scattering" [NG p.56]. His smooth gradations come from
stippling and from thin paint pooling in the ground texture, not from thick
blending [CATS p.127] (notes/research/friedrich_materials.md §6, table line
~105). All three fresh painters tried stippling and dropped it. This stream
makes it a real technique: many small touches of a brush tip, each simulated
through the bristles.

## What changed

**`bristle.rs`: a touch primitive (`Touch`, `Canvas::touch`, `touch_on`,
`touch_footprint`) and `Tool::stippler`.**
Before this, a "dab" was a tiny `Gesture` drag. That had four problems:
- Stationary tips still trailed in +x, because the travel direction defaulted
  to (1, 0).
- Deposit scaled with the distance traveled, so a dab laid almost nothing.
  At 1000px, dabs of every size were invisible.
- Only the central hairs of a pointed round touched.
- At 3200px, each hair of a big round stamped its own disk ("frogspawn") and
  small dabs came out as hard beads.

A touch is a vertical press/hold/lift (4 steps, more if the hand drifts):
- The tips splay radially as the belly flattens, so the patch grows with
  pressure. The outer hairs of a pointed round land only when pressed.
- Each hair deposits by film splitting: `TOUCH_FILM · lay · fill` coats over
  the patch area that hair stands for (spacing²), scaled by `0.4 + 0.6·p`
  (a lighter press squeezes out less).
- Each hair still lifts wet paint (pickup, dirty brush), so touches into wet
  paint fuse.
- In a pressed tip, paint wicks between the hairs. Each hair's contact is a
  bell of radius `max(hair, 1.4·spacing, 0.75 px)` with no pixel skirt. The
  hairs therefore sum to one continuous patch that is fullest in the middle
  (no frogspawn, no hollow ring). Coarse renders don't swell small marks,
  because the skirt was what made 1000px dots 2× wider and fainter than
  3200px ones.

`exchange` gained a `dep: Option<f32>` (fixed deposit volume). Drags pass
`None`, so stroke behavior and the golden are unchanged.

**`stipple.rs` (new): `Stipple` builder + `Canvas::stipple(mask, &Stipple, seed)`.**
- *Placement:* a jittered grid sized so that the densest coverage keeps every
  cell, thinned by `coverage(x, y) × mask` and optionally clumped (fbm).
  The densest coverage is sampled at mask pixels no more than a unit (or
  half a tool width) apart, and at every mask pixel if that finds nothing.
  It used to be probed on a lattice 3 tool widths apart, and a small disk
  or a narrow band of coverage could fall between the probes and be skipped
  (`small_and_narrow_regions_are_not_skipped` below).
- *Order:* each passage (tile) is worked in small patches, row by row and
  back and forth, dabbing at random within a patch. One load serves about one
  patch, and each dip's paint is aimed at the centroid of the touches it
  serves. (Shuffling a whole tile spread every pile across it and made
  tile-row seams in gradients.)
- *Feathering:* where coverage < 1 the touches also press lighter, so they
  are smaller and fainter.
- *Runner:* its own tile-parallel runner (checkerboard phases; tile ≥ 2 ×
  touch reach). I did not reuse `handling.rs::run_plans`, which is driven by
  `Plan`/`Gesture`/`Handling`, all of which the strokes stream is editing.
  So `handling.rs` is untouched: no merge conflict.
- *Determinism:* geometry is planned in parallel, and dip paints are chosen
  serially in tile order. Serial matters because `Palette::mix`'s cache
  returns whichever target first filled a quantized key, so parallel mixing
  differed across thread counts (caught by a test).

**Color: one integration point.** `Stipple::paint_for(want, seen, coverage,
…)` is the only place stippling chooses paint. When the color stream's
substrate-aware Palette API lands, call it there (with
`coats = touch_coats() × max(coverage, 1)`). Until then a clearly marked
WORKAROUND does the following:
- Invert Kubelka–Munk per channel (`aim_km`, bisection) so that a film of the
  passage's thickness over what the canvas shows (`seen`: wet paint over dry,
  averaged at 5 points) dries to `want`.
- Mix that on the palette and correct once for the mixture's real hiding.
- If thinned paint can't get there, retry with half and then no medium (the
  painter thins it less).
- Memoize recipes (OKLab steps ≈ 0.5–0.8 %).
- `.aim(false)` just mixes `want`, which is right for a veil built by
  density.

## API

```rust
use paint::{Stipple, Tool, Touch};

// one touch by hand
let mut b = Held::new(Tool::stippler(2.0), 1);
b.load(pal.paint(hex("#c8d0dc"), 0.5), 0.5);
c.touch(&mut b, &Touch::at(400.0, 120.0).pressure(0.6).drag(0.3, 0.0).twist(0.4), None);

// a sky: lay-in, then two stipple passes
let s1 = Stipple::new(Tool::stippler(3.0))      // ~1 mm marks on a 440 mm canvas
    .mixed(pal, 0.45).color(sky)                  // aimed at the result on the canvas
    .coverage(|_, _| 2.2)                         // touches per point (tone by density)
    .pressure(0.5, 0.85)                          // mark-size variation
    .dips(20, 0.4, 0.5);                          // touches per dip, load, wipe
c.stipple(&sky_mask, &s1, 12);                    // into the wet lay-in: fuses
c.dry();
let s2 = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.5).color(glow)
    .coverage(|_, y| 2.2 * smoothstep(100.0, 390.0, y)).dips(24, 0.35, 0.6);
c.stipple(&sky_mask, &s2, 13);

// mist over a dry ridge: a veil built by density, not aimed per dot
let mist = Stipple::new(Tool::stippler(2.2)).mixed(pal, 0.7).color(|_, _| hex("#c3c4bf"))
    .coverage(mist_cov).aim(false).dips(16, 0.3, 0.7);
```

Other knobs:
- `.drag(len, Some(angle) | None)`: hand drift while the tip is down; None
  means any direction.
- `.twist(sd)`
- `.cluster(0..1, size)`
- `.feather(k)`
- `.clip(true)`
- `.paint(hiding, stiff)` and `.jitter(l, hue)`: without a palette.

Mark size ≈ 0.8 × tool width at mid pressure.

## Evidence

`cargo paint study_stipple` (1000px) and `-- --full` (3200px) write
`out/study_stipple.png` and `out/study_stipple_full.png`. JPEG views and
crops are in `~/tmp/stipple-18244a99/evidence/`:
`study_1000.jpg`, `study_3200.jpg`, `crop1000_sky.jpg` vs `crop3200_sky.jpg`
(the same 200-unit square), `crop3200_strips.jpg`, `crop3200_loupe.jpg` and
`crop3200_mist.jpg`.

The sheet has four parts:
- Far left strip: the lay-in alone.
- Next strip: stipple pass 1 only.
- The rest: both passes.
- Bottom: a ridge (dry) with mist stippled over its foot, denser in the
  valley and thinning upward in fbm banks.

Timings (release, shared 10-core machine under load from 4 other agents):

| pass | touches | 1000px total (aim part) | 3200px total (aim part) |
|---|---|---|---|
| sky pass 1 (stippler 3, cov 2.2) | 176,897 | 2.8 s (1.9 s) | 5.2 s (2.5 s) |
| sky pass 2 (stippler 1.6, cov ≤ 2.2) | 212,743 | 1.6 s (0.8 s) | 4.4 s (1.8 s) |
| mist (stippler 2.2, cov ≤ 3, no aim) | 160,377 | 0.7 s | 2.4 s |

Timings vary about 2× with the load from the other agents.
The "aim part" is the serial KM-aim palette search (900–1,050 recipes)
inside the workaround. The touches themselves take about 2–3 s for a full
sky pass at 3200px. The whole 3200px study runs in 42–58 s,
most of it the `Handling` lay-in and ridge.

What I saw, judged as a painter:
- **Sky:** the stippled strips read as a luminous, grainy-smooth sky with no
  stroke direction. Next to them, the lay-in strip is visibly streaky. The
  gradient (cool blue → pale → warm glow) survives the stippling. At
  1000px and in the 3200px image scaled down, the two look alike. The same
  200-unit crop has the same grain scale at both resolutions, with no beads.
  Under a loupe at 3200px (`crop3200_loupe.jpg`) the marks are soft round
  patches with slightly irregular edges: no rings, no bristle dots.
- **Ribbing:** the brushed top ground's horizontal striations show through
  the thin stipple as fine ribbing. That is physical and documented ("its
  striations showing through the thin paint" [KÖR p.284]), but it keeps a
  horizontal cast the user dislikes. A sky stippled over a rolled ground
  would not have it.
- **Where it still reads as specks:** where the lighter pass 2 thins out
  (mid sky), isolated pale dots sit on the blue like snow. Each dot aims at
  the lighter glow over a lay-in that dried darker than intended (the
  color-semantics bug). The mist's upper fringe is a scatter of faint dots,
  and its body is mottled ("granite") at 3200px, though acceptable at
  1000px. The mist is the weaker half.

## Tests (`cargo test -p paint`, all pass; golden unchanged)

The new tests live in `stipple.rs` (the shared `tests.rs` is untouched):
- `touch_conserves_paint`: all tools, drift and twist, wet canvas.
- `touch_footprint_bounds_every_touched_pixel`: 3 scales, all tools.
- `touch_is_one_solid_patch`: round 8 and stippler at 3200px. The inner half
  is filled everywhere and the middle is at least as full as the rim, so no
  frogspawn and no hollow ring.
- `touch_is_resolution_independent`: volume within 15% and half-paint area
  within 35% + 1 px between 1000px and 3200px.
- `aim_km_reaches_reachable_targets`
- `stipple_is_deterministic_across_thread_counts`
- `stipple_follows_coverage_and_mask`
- `small_and_narrow_regions_are_not_skipped`: a radius-10 disk with a
  20-unit stippler, 2-unit coverage bands at several offsets and separated
  radius-4 islands all get touches

## Fade and relative color (round 3, branch `fixes-paint`)
Amnesia round 2 (coast #9, mountains #10): a stipple lighter than its field,
or a mist over a dark, read as salt or static where it thinned.
- **`fade(k)`, default 1.** Where coverage `c` < 1, each load of touches is
  aimed `min(1, c)^k` of the way from what it sits on to `color`, so the
  touches fade into the field as they thin out. This also applies with
  `aim(false)`. `fade(0.0)` restores constant contrast, for deliberate
  specks. Test `thin_stipple_fades_instead_of_salt` measures the sd of L
  where coverage is 0.18–0.48: a pale stipple over a mid-blue sky drops from
  0.0149 to 0.0058, and a mist fringe over a dark (`aim(false)`) from 0.0433
  to 0.0120. Where coverage is ≥ 1 the tone is unchanged.
- **`color_over(|x, y, under| ..)`**: the look wanted, relative to what the
  canvas shows where a load of touches lands (judged before the pass), e.g.
  a shadow on snow: `.color_over(|_, _, u| shift(u, -0.06, 0.0, -0.03))`.
- Dips judge the underlayer with `Canvas::judge_under` (a median), not a
  mean, so a speck under a dip's centroid doesn't decide its pile.

The `paint_for` workaround described above was replaced earlier by
`Palette::aim` (the code calls it; the old text is kept for history).

Evidence: `study_stipple` at 1000px, base vs this branch:
`~/tmp/fixes-paint-3f815331/study/study_stipple_{base,new}.jpg`
and the mist fringe crop `fringe_cmp.jpg` (base on top). There are fewer
isolated pale specks in the dark above the mist, and the tone thins more
gradually. The dense body of the mist is still grainy up close; stipple
over a dark still reads best as a light veil laid by brush and fused.

## Known issues / next

- **Color:** replace the `paint_for` workaround with the color stream's API.
  Lay-ins that dry too dark force the stipple to lift the tone, which makes
  pale specks. The palette's recipe switches make visible seams in a
  stippled gradient (I saw one in the mist when its color varied with y; the
  study now uses one mist color).
- **Coupling tone to density:** built in since round 3 as `fade` (see
  "Fade and relative color" below). A true veil mode (each touch carrying a
  share of the step) was tried and dropped: thin touches stack with Poisson
  noise and read *noisier* than opaque ones at coverage 3.
- **Stipple size in mm:** mark sizes are in canvas units, so a 1714 mm Monk
  needs a smaller `width` than a 440 mm canvas. A `Stipple::mark_mm` helper
  would help.
- **Short drags:** `Gesture` drags much shorter than the brush width (what
  painters did before) are still the old, nearly invisible dab. Routing
  `drag_on` to `touch_on` when the path is shorter than ~½ width would fix
  it everywhere. That changes `footprint`, tile sizes and the golden, so I
  left it for the integrator's call.
- **Performance:** the serial palette search dominates the aimed passes.
  Parallel mixing needs a palette cache that doesn't depend on the order of
  requests (color stream).
- **Ribbing at 1000px:** at 1000px the mist shows heavier horizontal ribbing
  than at 3200px, because the weave/striation contact is sampled at 0.44 mm
  per pixel.
