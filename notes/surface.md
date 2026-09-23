# Surface fixes (branch `surface`, round 3)

This branch fixes three open physics issues from the round 3 integration:
bare ground flecking through dark passages, light marks over a dark
drying darker than aimed, and drying stages that changed with resolution.

## 1. Bare flecks inside dark passages

### What causes them (measured)
`bristle::cover_tests::probe_bare` paints a dark square (`#2c2925`) on
the `friedrich_early` ground and dries it. It then counts the interior
pixels that read as ground, meaning closer to the ground's value than to
the paint's. It averages three seeds at 1000px. Run it with
`cargo test --release -p paint probe_bare -- --ignored --nocapture`
(`PROBE=body,broad`, `PROBE_COV=2.5`, `PROBE_VARS=base,nofill,...`).

A switch-one-thing-off run on the base code (e506951) ruled out the
deposit model:

| body, coverage 2.5 | bare |
|---|---|
| base | 1.97% |
| plough off (`push` 0) | 5.05% (worse: ploughing spreads paint into gaps) |
| pickup off | 1.94% |
| no broken strokes | 1.28% |
| full load every stroke | 1.74% |

Next I marked every pixel that any bristle's capsule reached. Of the 2.0%
bare pixels in the body passage, 1.85% had never been under a touching
bristle. In the broad passage it was 6.4% of 11%, plus 1.3% under a
bristle with zero contact, which was the weave skipped at light pressure.
So the flecks are the **gaps between strokes**, not paint missing inside
marks. The notes had blamed the bristle model's ploughing
(`notes/fixes_paint.md`); that was wrong. The images agree: the holes are
5–20 px patches between marks (`notes/surface/color_dark_band_before.jpg`),
not weave-sized specks.

Two things make the gaps:
- **The marks are smaller than the plan assumes.** A single broad stroke
  covers 63% of its nominal width × length with at least 0.15 coat
  (`probe_mark_area`): width falls with pressure (`mark_width` 19 of 22),
  the 0.4 release ramp lifts off, and the load runs down. Body strokes
  are short, so their end caps make up for it (1.15).
- **Placement along a row was uniform.** Each cell's stroke could fall
  anywhere in its cell. About a quarter of neighboring pairs left an end
  gap, and holes stayed open wherever gaps in two rows lined up
  (Poisson-like tails: broad at coverage 2.5 left 11% bare, about
  e^−2.5 = 8%).

### The fix
- **Look and fill** (`handling.rs`, `Canvas::fill_gaps`). After a pass, the
  painter looks at the passage and dabs paint into what the strokes
  missed. The pass finds bare pixels inside the region (no deposit from
  this pass, or a film under 0.04 coat) on a grid of cells half a brush
  wide, in units. A cell whose bare area is at least 0.5% of a brush
  width squared (and at least 1.5 px) gets one short stroke through the
  centroid of its bare pixels, along the local direction, with a dab's
  share of the load. This is one look, not a loop. **Default on** when
  the pass means to cover: `coverage ≥ FILL_FROM` (1.5), load ≥ 0.25,
  not a blender and not a scrub. `Handling::fill(false)` turns it off
  (broken color, a lay-in that lets the ground breathe) and `fill(true)`
  forces it on.
  - Crops and replay stay exact. Every dab follows from its cell alone:
    its own random stream, a fresh brush seeded by where it starts, a
    stroke id reserved per cell of the region's bounding box (always
    reserved, so later ids don't shift) and tiles sized for any dab.
    `crop_matches_whole` passes: mean color difference 0.0015 at margin
    20 and 0.00027 at margin 120. With the first, shared-brush version it
    was 0.034.
- **Rows are stratified along** (`place`): each stroke falls in the middle
  70% of its cell, as rows already did across.
- **Loaded hairs wet the shallow hollows** (`bristle.rs`, `WET_REACH` =
  0.5). A hair carries paint proud of itself, so a well-loaded blunt hair
  reaches half the tooth's range even when pressed lightly. A nearly dry
  one skims the peaks (`smoothstep(0.1, 0.8, vol/full)`). This removes the
  speckled fringes at loaded stroke ends. It changes nothing at normal
  pressure, where the reach is already larger.

### Numbers (probe_bare, 1000px, 3 seeds, share of the interior bare)

| pass | before | after | after, `fill(false)` |
|---|---|---|---|
| body, coverage 1 | 19.0% | 19.6% | 19.6% |
| body, coverage 2.5 | 1.49% | **0.00%** | 1.06% |
| body, coverage 4 | 0.10% | 0.00% | 0.03% |
| broad, coverage 1 | 47.2% | 36.6% | 36.6% |
| broad, coverage 2.5 | 10.96% | **0.01%** | 6.06% |
| broad, coverage 4 | 1.85% | 0.01% | 1.05% |
| detail, coverage 2.5 | 0.34% | 0.22% | 0.26% |

Coverage 1 still leaves the ground between strokes, as asked. Broken
color is still possible. At coverage 2.5, a body pass with load 0.1
leaves 2.1% bare and load 0.2 leaves 1.6% (neither is filled). A light
scumble (pressure 0.15–0.3, load 0.1) leaves 44% bare in the body and 47%
in the broad.

Regression test: `bristle::cover_tests::loaded_passage_covers`. At 500px
the body and broad passages at coverage 2.5 must be under 0.2% bare
(0.01% and 0.04%). Broad without looking must leave gaps (1.2%), and the
light dry-brush scumble must stay broken (38%).

### Evidence
- `cargo paint study_surface` (new): dark body paint, then broad, at
  coverage 1, 2.5 `fill(false)`, 2.5 and a dry brush.
  `notes/surface/study_surface.jpg`.
- study_color's dark band at 1000px: `notes/surface/color_dark_band_{before,after}.jpg`.
  The ground patches are gone.
- study_scene's sand and dusk sky at 1000px: `notes/surface/scene_1000_{before,after}.jpg`.
  The orange flecks in the sand and sky are gone. The trodden path keeps
  its texture.
- study_scene's dark dusk sand at 3200px (`--full --crop 50,1180,450,1310`):
  `notes/surface/scene_sand_3200_{before,after}.jpg`. There were about
  30 orange flecks; one faint speck remains.
- I didn't render the paintings in `paintings/fresh2` (the coast,
  mountains and winter friction items). They use the same `work` passes,
  so they should improve the same way, but that isn't checked.

## 2. Aim at a mark's mean look

`Palette::aim_for(want, under, medium, coats, Marks)` makes the mark's
**mean look** equal the look wanted. The mean is taken over the area of
a mark at each thickness and averaged in linear light, as the eye
averages it at viewing distance. The pile used to be judged at one
expected thickness (plus a spread). `Marks::of(tool)` picks the spread:

- `Marks::Blunt` (filbert, flat, stippler) and `Marks::Pointed` (round
  sable, rigger) hold the shares of the mark's area at 1/8…4× the
  expected thickness. I measured them with `probe_mark_thickness`
  (handling.rs, sparse marks, 1000px). A blunt brush lays about 75% of
  its mark at 1–2×. A pointed one lays a thick core and puts about half
  its area at ½× or less.
- The score is the mean-look miss, plus `AIM_ROBUST` (0.5) × the old
  per-thickness spread (so a pile can't be right on average while being
  wrong everywhere), plus the thin-edge term, the family term and
  parsimony. `AIM_FAMILY` rose from 0.12 to 0.28: the mean look over a
  contrasting underlayer tempts hue compensation (a yellowed grey over
  slate), and the family term holds it back.
  `contrasting_aims_stay_in_family` and `color_over_sees_the_canvas`
  both pass.
- `Palette::aim` is `aim_for(.., Marks::Blunt)`, as used by stipple and
  `Canvas::aim`. Handlings call `aim_for` with their own tool. The kind
  is part of the cache key, so results stay deterministic and cached.

**What it bought, honestly.** At the thicknesses the detail handling
lays (3.3 coats expected), light lead-white piles over a dark hide almost
fully even at 1/8×. The model's look spans L 0.633–0.658 against 0.655
wanted, so the pointed and blunt piles barely differ at that thickness.
They differ at thin marks: at 0.84 coats a pointed pile is 0.03 lighter
in masstone. The test miss for pointed marks turned out to be **partial
pixel coverage** of fine marks: pixels a hairline only partly covers show
the dark beside it. Its body L miss falls with resolution: −0.043 at
500px, −0.027 at 750, −0.020 at 1000 and −0.009 at 2000. That is
sampling, not the pile. `light_marks_over_a_dark_stay_in_hue` now judges
detail marks at 750px with a bound of **0.035** (was 0.05 at 500px).
Body marks keep 0.025. At 1000px detail marks come in at 0.024, but
that run takes over 3 min in a debug build. The test's dark lay-in keeps
its flecks with `fill(false)`, since that is what it tests against.

## 3. Drying stage independent of resolution

A film's drying rate now comes from its thickness **averaged over a
fixed physical patch**: `drying::FILM_MM` = 1.25 mm (sd, a double box
filter whose radius is picked for the pixel size). Bare pixels are
excluded from the average, so a film's edge isn't judged thinner than
its body. The rationale is that a film skins over as a whole, so the
bristle ridges and furrows within a stroke don't dry on separate clocks,
and a coarse pixel and a fine one are judged over the same millimeters.
`wait`, `dry` (time left) and `bake` (the tack rate of set films) all
use it (`Canvas::film_thickness`).

- `study_time` now reports the same stages at 400, 1000 and 2000px:
  light open at 30 min; at 3 h light tacky, umber tacky, slate open; the
  next day light and umber dry, slate tacky. Before, 400px had light
  setting and slate setting at 3 h, and 1000px had light setting at
  30 min. Its assertions are back and stricter than before (all three
  checkpoints).
- Test `drying::tests::stages_dont_depend_on_resolution`: the same
  physical film (lead white in furrows 0.6 mm apart, 1 ± 0.8 coats) at
  400 and 1200px. The stage shares at 30, 90 and 150 min are identical.
  Judged per pixel they differed: 13% of the fine render was tacky at
  90 min and 17% was still setting at 150 min.

**Remaining:** the blunt brushes' deposit itself depends on resolution. A
hog field laid at 400px is about 23% thicker, and more uneven at the
stroke scale, than at 1200px (mean 1.12 vs 0.91 coats). Where a film
sits near a stage threshold, the stage can still differ. The total time
`dry()` reports (the slowest film) also still varies with width: 14113,
8790 and 6512 min at 400, 1000 and 2000px. Both come from the deposit
(`notes/tip.md`, "Blunt tools still depend on resolution"), not from
drying.

## Changed output
- **Golden re-recorded** (`e930669`, debug profile). The change is
  intended: the fill pass, stratified rows, the wet reach of loaded hairs
  and the mean-look aim all change painted passages.
- Every `work` pass with coverage ≥ 1.5 and load ≥ 0.25 now paints extra
  fill dabs. Their number is small next to the pass (the brushed ground:
  92 dabs after 701 strokes). A painting that relied on ground showing
  through a coverage 2–3 passage should say `.fill(false)`.
- `Handling` has a new public field, `fill: Option<bool>`. Struct literals
  with `..h` are unaffected.
- `Palette` aims differ a little everywhere: blunt marks are judged by
  mean look and `AIM_FAMILY` is 0.28. In study_color's bottom skies the
  aimed sky misses by ΔE 0.002–0.018. The family sky misses by
  0.002–0.023 (before: 0.005–0.019); its top strip got worse, 0.011 →
  0.023.
- Checkpoint format is unchanged. The drying state has the same fields;
  only how rates are computed changed.

## API
```rust
// a lay-in that lets the ground breathe between strokes
c.work(&m, &st.broad().color(sky).coverage(2.5).fill(false), 1);
// fill even a sparse pass's gaps
c.work(&m, &st.body().color(rock).coverage(1.2).fill(true), 2);
// aim a pile for a particular brush's marks
let m = pal.aim_for(want, under, 0.2, 1.5, paint::Marks::of(&Tool::round_sable(2.0)));
// drying: nothing new to call; stages no longer depend on the width
c.wait(180.0);
```

## Next
- Make blunt deposits resolution-independent (the coverage-aware tracks
  that `tip` gave pointed tools). It would close the remaining drying
  difference and change every painting.
- Count coverage by a mark's real footprint (`probe_mark_area`: broad
  0.63 of nominal). Filling covers the gaps, but "coverage 2.5" still
  means about 1.7 real layers for broad strokes, so `laid_coats` would
  need re-measuring together with it.
- Render `paintings/fresh2` and the Moonrise at 3200px to check fills in
  masked motifs (thin clipped bands and spires).
