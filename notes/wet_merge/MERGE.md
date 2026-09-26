# Merging main into the wet-on-wet engine (branch `wet-merge`)

Main at `4abe5b4` merged into `r6-wet` at `62ace25` (their merge base is
`2159c6c`; main is 153 commits past it). Nothing is pushed and main is
untouched. Paintings rendered on both engines: `README.md` here.

**Bottom line.** Main's fixes are all in, and the wet engine's two layers,
pickup, plough floor, stirring, tip, lift-off and blender spread are all
kept. It took two engine decisions and four test changes:

- **Main's dry-rims fix, re-expressed in the wet model.** Each hair now
  ploughs by its own size (`crates/paint/src/exchange.rs`, `contact` and
  `exchange`). The branch's plough floor alone doesn't stop the rims: with
  the branch's plough, a filbert 5 loaded full still ploughs its mark into
  rims over dry paint. It lays 0.36 coats in the middle of its mark and
  0.68 at the edges. The same failure shows as a lacy net of stroke
  outlines on l5_near's snowy rock cap.
- **One change to the branch's lift-off.** The tip hair, which main's twigs
  fix keeps down until the hand leaves the canvas, no longer gets the extra
  lift drag, so a lifted stroke reaches the end of its path.
- **Four tests whose thresholds were set on one parent's side effects.**
  They now state what holds on the merge, with the numbers in their
  comments (§3). One of them matters for how the branch was judged: the
  branch's "a loaded light stays on top" held at 500 px only because the
  hairs there were finer than a pixel and over-ploughed. At 2400 px the
  branch kept 0.58 of the light's lift; the merge keeps 0.63 and main 0.57.

Tests: `cargo test --workspace` passes 269, fails 0, ignores 25.
`cargo test --release -p easel --test hand_time -- --include-ignored`
passes 5 of 5. `cargo build --release --workspace` passes, with one
warning in main's own `fresh2_winter.rs`. Clippy shows no errors.

## 1. The conflicts

| file | hunks | resolution |
|---|---|---|
| `crates/paint/src/bristle.rs` | 5 | the branch's structure (exchange in `exchange.rs`, `drag_stroke`/`touch_stroke`), with main's `Clip` enum (fence) in every signature and call; main's in-file `exchange` (the ~590-line block) dropped *after* porting its three changes into `exchange.rs` (§2); `probe_patch` dropped (main's test audit deleted it); the branch's `feed_keeps_the_tips` and `a_lifting_flat_does_not_heap_its_stroke_end` kept |
| `crates/easel/tests/hand_time.rs` | 2 | main's structure from the test audit (one example tripwire, `l5_near` opt-in with `#[ignore]`, no `PRINT_HASHES`); hashes re-recorded (§4) |
| `notes/time.md` | 1 | main's text (it describes main's test names) |
| `crates/paint/tests/golden_scene.txt` | 1 | re-recorded, not merged (§4) |

Git merged everything else on its own. Two of those automatic merges
needed checking:
- **Main's twigs fix** (`017113a`, the tip hair stays down at any positive
  pressure) landed in the branch's `drag_stroke` on its own. It then
  collided with the lift-off (§2.3).
- **Main's fence** (`r7-edges`, `Clip::Fence`) reaches the brushes through
  `handling.rs` and `stipple.rs`, which merged cleanly. The branch's
  `film::Stroke` held `Option<&Mask>`; it now holds `Option<Clip>`
  (`crates/paint/src/film.rs`).

The checkpoint format stays at the branch's `PAINTCK8` (main is at
`PAINTCK7`).

## 2. What main's `exchange` changes became in the wet model

Main changed its old one-file `exchange` in three ways after the branch
split off. The branch had rewritten that function into `exchange.rs`, so
git couldn't carry the changes across. I ported each one by hand.

### 2.1 The fence (`r7-edges`)
Main's code, moved over as it was:
- the contact weights read `Clip::at`: past the fence the hairs lift off
  the weave's hollows (`ct.th + lift`);
- a stroke past its fence lays a thinner film (`sum_k / sum_w`), applied
  where main applies it, before the tack;
- the plough's target checks `c.at(..).0` instead of the mask.

Main's fence tests pass (`fence::tests::found_is_near_the_line_and_lost_runs_far`,
`fences_carry_paint_past_the_edge_by_quality`).

### 2.2 Dry rims (`84befc5`, `notes/fixes/dry_rims/`)
Main's fix: a hair finer than a pixel is drawn at least 0.55 px wide
(`rb`), so each pixel of its drawn track now gives up only its share of
what the hair ploughs (`hair / rb`), and the paint lands a hair's width
away (`2 · hair`), shared bilinearly. On the wet engine the same three
pieces go into the branch's plough, which moves only the paint above the
film a hair rides on (`PLOUGH_KEEP`):
- `contact(.., rb, hair, mode)` computes
  `push_k = push · (seg / 2·hair) · (hair / rb)` (`exchange.rs`, `contact`);
- `hair` is `rb` for a pointed tool's hair and for a pressed touch, as on
  main (`exchange.rs`, `exchange`);
- `off = 2 · hair` for a moving hair, `rb + 1` for a touch;
- a moving hair's ploughed paint is shared bilinearly. The branch's soft
  blender keeps its own spread along the flank (glitch T), out to the new
  `off`.

Stirring still works per drawn pixel (`seg / 2·rb`). A fine hair stirs by
its drawn track, not its own. Neither side changed that, and I left it.
It's a candidate for the same argument as the plough.

**Why it's needed.** The branch's floor keeps a *thin* film from being
ploughed, and main's test (`a_stroke_over_dry_paint_covers_its_middle`,
loaded 0.8) passes with or without the per-hair plough. A full brush lays
paint well above the floor, though. At 1000 px, main's measure (the film
in the middle half of the mark against its thickest) gives:

| over dry paint, 1000 px | the merge | the merge with the branch's plough |
|---|---|---|
| filbert 5, load 1.0, stiff paint | 1.00 | 0.53 |
| filbert 5, load 1.0, medium 0.5 | 1.00 | 0.57 |
| filbert 4, load 1.0 | 0.99 | 0.79 |
| style body, load 1.0 | 0.98 | 0.98 |

l5_near shows it in its own picture. The rock's snow cap is a filbert 4
pass (load 0.6) over the dried stone, then a blend (`notes/loops/l5_near.lua`,
chunk 11). With the branch's plough the cap carries a net of dark stroke
outlines at 1000 px; with main's per-hair plough it's a clean sheet, like
main's. Crop: `~/tmp/wet-merge-9d4eadac/check/l5near_rims.png` (main, the
merge with the branch's plough, the merge).

**Test.** Main's test now also runs every tool loaded full
(`crates/paint/src/bristle.rs`, `a_stroke_over_dry_paint_covers_its_middle`).
It fails with the branch's plough (filbert 5: 0.36 coats in the middle,
0.68 at its thickest) and passes on the merge and on main.

### 2.3 Twigs meet the lift-off
Main's `a_lifted_stroke_paints_to_the_end_of_its_path` failed on the
merge: a blunt rigger's flick stopped at 178 on a path ending at 180.
Main's fix keeps the tip hair down to the end. The branch's lift-off
(`LIFT_DRAG`, §8 of `notes/wet.md`) bends every hair back behind the hand
by up to 0.6 of its length as the handle rises, and that includes the tip.
So the tip was down, but trailing 2 units behind the hand.

Fix (`bristle.rs`, `drag_stroke`): the tip hair (the one main keeps down)
doesn't get the extra lift drag; the trailing hairs still do. After the
fix, every tool and stroke in the test ends at 178-180 with no bare
column. That covers blunt and pointed sables and riggers, a hog flat 6 and
a filbert 4, each with a flick and a limb lifted at a fork. Before, four
of the twelve left 1-2 bare columns. The branch's lift-off test
(`a_lifting_flat_does_not_heap_its_stroke_end`) still passes.

### 2.4 Blunt presets meet the branch's tests
Main made `round_sable` and `rigger` blunt again (`5e63e1b`). Tests
written on the branch with a pointed `round_sable` now ask for the point
explicitly, as main's own tip tests do:
- `feed_keeps_the_tips` (`sable(2.0)`, `sable(8.0)`): capillary feed runs
  only on pointed tools, so the test needs a point to test anything;
- `a_blender_along_a_wet_line_does_not_split_it`
  (`Tool { point: 1.0, ..Tool::round_sable(3.0) }`), to keep its scene;
- `a_lifting_flat_does_not_heap_its_stroke_end`'s "blunt round" is now
  just `Tool::round_sable(12.0)`.

## 3. Tests whose thresholds changed

All four fail without the merge's changes for reasons I traced, not
because the behavior they guard broke. Each keeps its intent, and each
comment gives the numbers.

**`wet::tests::wet_on_wet::a_loaded_light_laid_lightly_stays_on_top`**
(branch). Threshold 0.85 → 0.62 on the share of its lift that a loaded
light laid lightly into wet dark keeps, against the same stroke over
the dark dried. The test's 500 px canvas and the same scene at 1000 and
2400 px:

| engine | 500 px | 1000 px | 2400 px |
|---|---|---|---|
| main (one mixture per pixel) | 0.55 | 0.56 | 0.57 |
| r6-wet (the branch's plough) | 0.97 | 0.85 | 0.58 |
| the merge | 0.68 | 0.65 | 0.63 |

At 500 px the hog flat's hairs are a third of a pixel. The branch's
plough moved paint by the drawn track, several times what the hairs move,
and cleared the wet dark out of the light's path. At 2400 px, where the
hairs are wider than a pixel, the branch kept 0.58, about main's 0.57.
The merge is about the same at every size and ahead of both parents at
2400. The new threshold, 0.62, still fails main's one-mixture engine,
which is the bug the test was written against. Its second assertion (a
lean brush pressed hard works the dark in more) is unchanged and passes
(0.35 against 0.68). I also switched stirring and pickup off one at a
time: without stirring the light keeps less (0.39-0.50), and pickup
barely matters (0.65-0.73). So the loss at high resolution isn't pickup.
**This bears on the branch's evidence:** its tests ran at 500-1000 px,
where the over-plough helped the lights.

**`drying::tests::stages_follow_the_clock_and_the_pigment`** (branch's
version of main's test). "Lead white sets within 3 h" → "within 3.5 h";
bone black is still checked as wet at the same moment, and the day's
clock still totals 24 h. The filbert 40 band is now 1.79 coats where
the test samples it; with the branch's plough it was 1.23. Its middle no
longer ploughs out to its edges, which is main's dry-rims fix. Thicker
paint sets later: tacky at 185 min, where it used to be 145.

**`bristle::tip_tests::presets_are_blunt_and_lay_their_width`** (main).
Before, the blunt sable 1.6's ink width at pressure 0.4 had to be above
0.8 and above 1.5 × the pointed one's. Now it has to be at least 0.9 of
the width its hairs span (`mark_width(0.4)` = 1.02) and above 1.3 × the
pointed one's. The blunt mark is 0.97 on the merge, 1.10 on main and
1.17 with the branch's plough. The pointed one is 0.66 on all three. The
merge's blunt mark is its hairs' width, because its plough doesn't push
paint out past them. The test's regression (the old pointed default made
the two equal) still fails it. The new form is closer to the test's name
than the old ratio was.

**`crates/easel/tests/hand_time.rs`, `an_old_overrunning_log_replays_unchanged`**
(main). Its printed ledger moved: sitting 1 at 0.341 h, where main's
engine gives 0.337 and r6-wet 0.338. The clock moves to 247.99 and 368.44
min. A pass fills the gaps its strokes leave on the canvas
(`crates/paint/src/handling.rs`, `fill_gaps`; it counts bare pixels), so
the planned strokes and their hand time follow where the paint landed.
The test's own doc says pixels aren't its job. The ledger isn't
engine-free either, and the comment now says so.

## 4. Re-recorded

I re-recorded these only after the checks above, and after comparing
renders on the three engines at 1000 px (OKLab ΔE × 100, mean / 95th
percentile):

| log | merge vs main | merge vs r6-wet | r6-wet vs main | merge vs merge with the branch's plough |
|---|---|---|---|---|
| `paintings/lua/example.lua` | 0.44 / 1.29 | 0.27 / 0.42 | 0.56 / 1.38 | 0.21 / 0.42 |
| `notes/loops/l5_near.lua` | 1.06 / 4.39 | 2.98 / 10.8 | 3.07 / 9.59 | 1.60 / 7.26 |

r6-wet sits far from main on l5_near partly because of main's own changes
since the split: blunt presets (l5_near paints its rock's lichen and
touches with `brush("round")`, which was pointed on the branch), the
`clip=<mask>` fix and dry rims. Main's per-hair plough moves l5_near toward
main at 1000 px (1.74 → 1.06 against main). At 2400 px fewer hairs are
finer than a pixel, so it should matter less there; `README.md` has the
2400 px renders.

- golden scene `2e3c999c0d769397` (`crates/paint/tests/golden_scene.txt`);
- `example.lua` at 160 px `0x9611_4321_481b_9b00`, `l5_near.lua`
  `0xa215_ef39_8e57_b9f3` (both release; `cargo test --workspace` checks the
  example in the test profile too, and passes);
- the overran ledger (§3).

## 5. Open

- Stirring by the drawn track (§2.2).
- The loaded-light result at painting resolution (0.63 of the dried
  lift, §3) is lower than the branch's notes suggest (0.88 clean touches in
  `study_wet`, a different gesture and measure). I didn't rerun the wet
  studies on the merge.
- The blender's plough now spreads out to `2 · hair` (main's distance), not
  `rb + 1`. At 3200 px a Friedrich badger's hair is about 6.6 px, so the
  spread is 13 px where it was 7.6. The tramline test passes.
