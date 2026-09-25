# Fix: strokes over tacky or dry paint kept their outlines

Branch `fix-dry-rims`, from main at d9af250. A bug fix under the feature
freeze: no new capability, no new knob.

## The bug

Four painters in rounds 8 and 9 reported it independently:

- r8-arm1, friction 2: `work{hand="body", tool="filbert 5"}` on the far
  hills "laid every stroke as an outline: 399 µm of paint on the stroke's
  edges, 11 µm inside, so the sky showed through each stroke as a net of
  gray rings."
- r8-arm3, friction 2: body strokes over tacky or dry paint made "a lacy
  net of loop outlines instead of a covering film."
- r9-arm3, friction 4: "wet over dry keeps every stroke's outline."
- r9-arm2, friction 3: thin dark filbert strokes over light snow at
  3200px looked "like glass tubing": dark ridged edges with a pale middle.
  (Friction 1, a tan rim around pointed dark strokes, is a separate
  cause. See below.)

A painter laying an opaque body stroke over dry paint expects a covering
film, thickest about where the brush pressed, not a ring.

## The minimal test

`a_stroke_over_dry_paint_covers_its_middle` (crates/paint/src/bristle.rs:1651,
helper `film_across_over_dry` at :1624). The test lays one stroke over a
light layer that has dried, on a Friedrich-sized strip 1000px wide. It
measures the mean new film across the mark along its middle and asserts
that the middle half of the mark holds at least 0.6 of the thickest film
anywhere across it. It checks three brushes: `filbert 5`, a small filbert
like r9-arm2's shadows (`filbert 2`, lay 0.5, stiffness 0.3) and the
Friedrich body tool.

On main it fails on the first brush:

    filbert 5: 0.06 coats in the middle of the mark, 1.25 at its thickest:
    [... (-3.50, 1.25), (-2.50, 0.21), (-1.50, 0.06), (-0.50, 0.06),
     (0.50, 0.06), (1.50, 0.06), (2.50, 0.21), (3.50, 1.18) ...]

That's r8-arm1's report in miniature: a 20:1 ratio of edge to middle.
With the fix it passes.

## Diagnosis

Measurements come from scratch probes on the Friedrich ground: one stroke
at pressure 0.8 over a dried layer. "Ratio" means the thickest film across
the mark divided by the mean over its middle half.

| brush | render | main | fix |
|---|---|---|---|
| filbert 5 | 1000px | 1.24 / 0.06 coats, ratio 21 | 0.68 / 0.61, ratio 1.1 |
| filbert 5, `push` 0 | 1000px | ratio 1.0 | ratio 1.0 |
| filbert 5 | 3200px | ratio 1.1 | ratio 1.1 |
| body (filbert 9, push 0.06) | 1000px | ratio 1.1 | ratio 1.1 |
| filbert 2 (r9-arm2's shadows) | 3200px, p 0.6 | 0.32 at the edges, 0.02 inside | 0.10 to 0.20 across |

Three things follow:

1. **It's the plough.** With `push` = 0 the rims vanish.
2. **It depends on resolution.** The same brush is fine at 3200px and a
   ring at 1000px. It appears wherever a brush's hairs are finer than a
   pixel: filbert 5 at 1000px (hair radius 0.23 px), filbert 2 even at
   3200px (0.29 px).
3. **The dry layer doesn't cause it.** The stroke rims the same over bare
   ground, wet paint, tacky paint and dry paint. The dry layer is where it
   shows. Over wet paint a stroke mixes with the layer, so its cells show
   the same wet mix, but over tacky or dry paint they show the old,
   differently colored layer. Painters' first lay-ins used the style's
   broad and body brushes (`push` 0.03 and 0.06, hairs near a pixel at
   1000px), where the effect is mild. Their later passes over set paint used
   small tools such as `filbert 5`, or `filbert 2.5` and `round 1.6` in
   r9-arm3's log.

In a whole `work` pass (body hand, coverage 2.5, over dry paint, new film
in coats, 10th / 50th / 90th percentile), `filbert 5` went from
0.24 / 0.85 / 20.2 on main to 2.41 / 3.95 / 7.44 with the fix. The
default body tool went from 1.66 / 2.98 / 6.00 to 1.83 / 3.52 / 5.05.

## The cause

Code as of main d9af250, crates/paint/src/bristle.rs:

- :846: a blunt tool's hair is drawn at a radius of at least 0.55 px:
  `let rb = (tool.hair_radius() * s).max(if tool.point > 0.0 { FINE_RB } else { 0.55 });`
- :1188: its contact reaches `rb + 0.5`:
  `1.0 - smoothstep(rb * 0.5, rb + 0.5, dist)`.
- :1252, :1307: every pixel of that drawn track gives up
  `push_k = tool.push * (seg / (2.0 * rb))` of its paint, per hair, per step:
  `let m = v * push_k * wt * fl;`.
- :1256, :1319: the paint is thrown to the next whole pixel,
  `off = rb + 1.0`, rounded to one target pixel.

So at coarse resolution a hair ploughs the paint under a track several
times wider than itself and throws it more than a pixel. For a `filbert 5`
at 1000px, each hair moves about 5 times the paint its own track holds, 3
times as far: about 18 times the flux. That compounds over the roughly 50
hairs stacked on each pixel, until every stroke's paint (its own deposit
included) ends up just outside its outermost hairs.

`push` means "Fraction of wet paint a moving bristle ploughs aside per
pass" (:113), a physical share. The pointed tool's fine hairs were already
ploughed physically: exact track coverage, a hair's width aside, shared
bilinearly ("the same distance at any resolution"). Blunt tools were not.

## The fix

bristle.rs:1252–1267 and :1330. A moving blunt hair now ploughs by the
same rule as a pointed tool's fine hairs:

    let hair = if fine || dep.is_some() { rb } else { (tool.hair_radius() * s).min(rb) };
    let push_k = tool.push * (seg / (2.0 * hair)).clamp(0.0, 1.0) * (hair / rb);
    let off = if dep.is_some() { rb + 1.0 } else { 2.0 * hair };
    let spread = dep.is_none();   // the target is shared bilinearly

- Each pixel of the drawn track gives up only its share of the hair's own
  track (hair / rb). This is 1 wherever the hair is at least 0.55 px, so
  high-resolution renders barely change.
- The paint lands a hair's width away (2·hair), shared by the four pixels
  around the target, not rounded to one.
- Pointed tools are unchanged (`fine`: hair = rb, and off and the sharing
  were already this). Touches (a pressed tip, `dep`) are unchanged: their
  contact is widened on purpose to bridge the hairs (`touch_rb`).

There's no new parameter, no noise and no smoothing, and the footprint
bound still holds: the plough reaches at most 2·rb, within the 3·(rb + 1)
that `footprint` reserves. Its debug assertions and
`footprint_bounds_every_touched_pixel` pass.

Re-recorded, deliberately:

- the golden scene (`crates/paint/tests/golden_scene.txt`);
- the easel replay hashes in `crates/easel/tests/hand_time.rs`
  (example.lua, l5_near.lua and overran.lua at 160px, where every hair is
  finer than a pixel), with a note in each doc comment;
- overran.lua's printout, whose clock moves by 3 s: after a covering pass,
  `fill_gaps` (handling.rs:665) lays hand-timed strokes through bare spots,
  and the fix leaves different spots bare.

One test changed its measure:
`tally::tests::hand_time_ages_the_paint_as_it_goes_and_is_deterministic`
asserted that a hand-timed pass changes the dry picture. On main, 22
pixels of its sky set within the 160-minute pass, because they were thin
films left between rims (thin films dry fastest). With the fix nothing
sets in 160 minutes, as average paint shouldn't (it gels at about 3.6 h,
drying.rs `GEL`), so the dry pictures match. The paint still aged: the wet
layer differs in 25,714 of 41,040 pixels. The test now compares the wet
paint (tally.rs:389).

## The related reports

- **Glass tubing (r9-arm2, friction 3): same cause, fixed.** Thin
  filberts at 3200px have hairs finer than a pixel. The probe above
  reproduces the profile: dark edges with next to no paint between them,
  and after the fix a filled stroke. I didn't check why their middles read
  lighter than the snow itself.
- **The tan rim of a pointed dark stroke (r9-arm2, friction 1): not the
  same cause, not fixed.** A pointed round 2.6 dragged over dry paint gives
  the same profile with `push` 0 as with its default 0.05, and no film
  outside the mark, at 1000 and 3200px. The pointed branch already ploughed
  physically, and this fix doesn't touch it. The painter's reading (the
  outer hairs of the tuft lay a thin film beyond the mark) remains a
  candidate. Not investigated.
- **"A blender can't move dry paint" (r9-arm3, friction 4, second half):**
  that's physics, not a bug.

## Before and after

All images are lossless PNG: side by side and labeled, the same window on
main (d9af250) and on this branch, 1:1 unless marked. `sbs.py` made them
(`uv run --with pillow sbs.py a.png b.png x0 y0 x1 y1 out.png "a" "b"`).

- `r8arm1_far_hills_1000.png`: r8-arm1's far hills as its notes say it
  first painted them, `hand="body", tool="filbert 5"` over the dry sky
  (`r8arm1_far_hills.lua`: its log, chunks 1 to 5, with the tool put back).
  On main it's a net of gray rings with the sky showing through. With the
  fix it's a covering band.
- `r8arm3_far_hills_1000.png` and `r9arm3_snow_shadows_1000.png`:
  reconstructions of the other two passages with the default body hand
  (`r8arm3_far_hills.lua`, `r9arm3_snow_shadows.lua`). The logs keep only
  the workarounds, and the tool of r9-arm3's shadows wasn't recorded. The
  fix removes a few pale rings and specks, but on main neither is the net
  they describe (see what's left).
- Round 2's winter (`fresh2_winter`, as ported to main in round 7:
  notes/round7/winter_port.md), with round 7's windows:
  `winter_whole_1000.png` (whole, 1000px) and `winter_oak_crown_3200.png`,
  `winter_ruin_path_3200.png` and `winter_fir_group_3200.png` (1:1 at
  3200px). At 3200px it barely changes (mean 1.1/255 per pixel, 1.2% of
  pixels by more than 8/255): the ruin reads a little more solid and
  nothing looks worse. At 1000px it changes more (mean 2.1/255, 4% of
  pixels): the oak's limbs and twigs are thinner and cleaner and the crows
  are no longer blobs. That's closer to the 3200px render. The mean linear
  difference between the 1000px render and the 3200px render box-filtered
  down to 1000px:

  | window | main | fix |
  |---|---|---|
  | whole | 0.0146 | 0.0121 |
  | oak crown | 0.0260 | 0.0179 |
  | fir group | 0.0330 | 0.0243 |
  | ruin with path | 0.0207 | 0.0180 |

## Tests

- `cargo test --workspace`: all pass (paint 194 passed with 14 ignored,
  easel 45, hand_time 5, determinism 1 and the rest).
- `cargo test --release -p easel --test hand_time`: 5 passed.
- `cargo clippy --workspace --all-targets`: no new findings. Main already
  has one error (crates/paint/src/palette.rs, an approximate `FRAC_PI_2`
  in test code) and some 50 warnings, all unchanged.

## What's left

- r8-arm3's and r9-arm3's nets with the **default** body tool aren't
  reproduced. The Friedrich body brush (filbert 9) has hairs of 0.41 px
  at 1000px, so it had only mild rims (a single stroke's ratio was 1.1 on
  main). At 3200px its hairs are 1.3 px and nothing changes. If those
  passes used small tools, this was the cause. If a net still shows with
  the default body tool, it's something else. Tack's stick-and-slip
  (drying.rs `stick`) is one mechanism that makes deposits patchy over
  tacky paint. Not investigated.
- The tan rim of pointed dark strokes (above).
- Touches still plough by their widened contact (`touch_rb`). No report
  points at them, and the stipple was the painters' workaround.
