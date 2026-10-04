# Thinner: results

The thinner is implemented in engine 3 on branch `thinner2` (not pushed).
The approved tests are unchanged. They are frozen at b2a0e14
(`~/src/a/claude-paint-reviews/thinner-tests-APPROVED.md`), and all 22
frozen files still match their recorded sha256. The protected baseline
and dumper are untouched, except for one field added under a new name.

**The card target is met at 2400 px**: thinned raw sienna keeps 84% of the
card's contrast at load 0.3 and 81% at load 0.6. Six required tests fail:

- Three are mistakes in the frozen test code. Fixed copies pass.
- Three conflict with the plan's own physics: a brush keeps what it can't
  lay, and a thinned stroke lays a thin film.

They are listed under [Needs re-review](#needs-re-review). Check 13 (b)
is the known pre-existing finding.

| commit | what |
|---|---|
| df9058c | the thinner: solvent beside the paint, the per-stroke ceiling, evaporation on the minute grid, the flow, PAINTCK9, Lua, dumper field |
| 18548cf | τ doubles at 100 µm, not 20 (the card's wait guard); the guide's Thinner section |
| afc7898 | the flow leaves a wetting film (the 2400 px lattice defect); `scripts/thinner_sheet` |
| 1e27682 | RESULTS.md, the two sheets, logs |
| 0a661f6 | round 4: a brush's `Debug` text (the `brushes=` digest, the dump's `brushes.debug`) is af49348's byte for byte before engine 3 |
| e135129 | round 4: PROPOSED test corrections (c03 ×2, c18, c04, c05 approved in round-4 review) |
| 48be56a | round 5: PROPOSED c16 correction and the rag sheet's directory, for re-review (`TESTS_PHASE_REPORT.md`, round 5) |
| 4c15082, ffef02e | round 4: the rag study re-rendered after afc7898; this file |
| (this commit) | round 5: τ disclosure; `weave_or_brush.jpg` |

## Commands, exit codes, logs

Every command ran through `~/src/a/claude-paint-tools/lockrun`, on this
machine (rustc 1.97.1, release).

| command | code | result | exit | wall time | log (sha256) |
|---|---|---|---|---|---|
| `scripts/test_thinner_acceptance --all` | afc7898 | FAILED: 6 required tests fail (below); every other test passes, including all four slow ones and the card | **1** | 173 s in lockrun | `logs/all_afc7898.txt` (2753d020…09ce) |
| `scripts/test_thinner_acceptance --card` | 18548cf | the card passes; 13 (b) is the expected failure | **3** (NOT ALL GREEN) | 26 s | `logs/card_18548cf.txt` (40a2a3d7…e14b) |
| `scripts/test_thinner_acceptance --quick` | 18548cf | FAILED: the same 6 | **1** | 9 s | `logs/quick_18548cf.txt` (b998c8c6…ec3c) |
| `cargo test --release -p paint --lib` (existing tests) | afc7898 | 180 passed, 0 failed, 8 ignored | 0 | 122 s of tests | `logs/paint_lib_afc7898.txt` (140fc205…2185) |
| `cargo test --release -p easel --bin easel -- --skip thinner_tests::` (existing tests) | afc7898 | 71 passed, 0 failed, 1 ignored | 0 | 50 s | `logs/easel_bin_afc7898.txt` (ccfbf88d…ad67) |
| `cargo test --release -p easel --test determinism --test session_integrity`, `-p paint --test curved_drag_nan --test ground_grain` | afc7898 | 4 + 10 + 2 + 1 passed, 0 failed | 0 | 145 s | `logs/integration_afc7898.txt` (b30d66e4…4c09) |

The runner fails every test of a cargo run that exits nonzero (review R1),
so its summary lists all 12 quick paint tests and all 8 easel tests as
"NOT PASSED". The per-test lines in the log show each one's own result.
The table below is from those lines. Afc7898 changed only the flow, after
18548cf's `--card` and `--quick`. Its `--all` reran the card and every
quick test, with the same pass and fail set.

## Each check (at afc7898, `logs/all_afc7898.txt`)

| check | test | result |
|---|---|---|
| 1 | `c01_the_card_…_2400px` | **pass** (numbers below) |
| 2 | `scripts/thinner_check2` | **pass**. All 6 scenes and 27 chunks are equal to af49348 on every baseline field, with af49348's PNGs, with and without `thinner=0`. 54 chunks declare the solvent; the thinned scene dumps nonzero solvent |
| 3 | `c03_af49348_paintck8_saves_are_refused_…`, `c03_engines_1_and_2_have_no_thinner_…` | **pass** |
| 3 | `c03_af49348_engine_3_logs_still_replay`, `c03_a_log_keeps_its_engine` | **fail: bugs in the frozen tests** (re-review 1 and 2). A fixed copy passes |
| 4 | `c04_a_stroke_and_a_wipe_…` | **fail** (re-review 4) |
| 4 | `c04_control_a_wholly_unthinned_scene_balances` | pass |
| 5 | `c05_an_emptying_brush_…` | **fail** (re-review 5) |
| 6 | `c06_more_pressure_…` | pass |
| 7 | `c07_two_overlapping_thinned_passes_…` | pass |
| 8 | `c08_a_hundredth_of_thinner_is_a_small_change` | pass |
| 8 | `c08_more_thinner_never_hides_the_card_more` (slow) | pass (sweep below) |
| 9 | `c09_the_solvent_evaporates_…` | pass |
| 10 | `c10_a_solvent_wet_film_spreads_more_…` | pass (roughness falls 3.5% in 3 min with solvent, 0 without) |
| 10 | `c10_thinned_and_unthinned_paint_of_equal_thickness_gel_and_dry_together` (slow) | pass |
| 11 | `c11_a_save_mid_evaporation_…` | pass |
| 12 | `c12_the_same_log_saves_the_same_bytes_on_one_thread_and_four` | pass |
| 13 (a) | `c13_every_pigment_value_is_af49348s` | pass |
| 13 (b) | `c13_burnt_sienna_shows_the_card_…` | **expected fail** (pre-existing; user decision) |
| 14 | `c14_more_points_on_the_same_path_…` | pass |
| 15 | `c15_solvent_does_not_slow_the_oil_cure` | pass |
| 16 | `c16_brush_rag_and_spreading_…` | **fail** (re-review 6) |
| 17 | `c17_a_wait_split_on_the_minute_grid_…` | pass |
| 18 | `c18_a_failed_chunk_after_thinned_paint_…` | **fail: a bug in the frozen test** (re-review 3). A fixed copy passes |
| 19 | `c19_the_same_paint_with_more_or_less_solvent_looks_the_same` | pass |
| rag | `rag_study` (slow) | pass; sheet written (below) |

## The card (check 1, 2400 px, raw sienna thinned 0.5)

At afc7898 (`--all`). The wait is ceil(10 τ) + 1 minutes, τ of the
thickest paint on the card.

| load | contrast kept | waited | τ after the pass / at the measurement | solvent left: strip / whole canvas (of what was there after the pass) |
|---|---|---|---|---|
| 0.3 | **84.4%** | 35 min | 3.328 / 3.328 min | 0 of 0.0022 / 7.0e-7 |
| 0.6 | **81.4%** | 35 min | 3.328 / 3.328 min | 7.0e-7 / 7.0e-7 |

The load-0.3 strip was painted first. Its solvent was mostly gone during
the rest of the pass, so the strip held only 0.0022 µm-pixels right after
it. Today, unthinned, the same card keeps 13% (load 0.3) and 6%
(load 0.6) (DIAGNOSIS.md).

Earlier runs:

- **At df9058c** (τ doubling at 20 µm), load 0.3 kept 88.9%. The run
  failed its own guard: τ grew from 9.72 to 10.07 min as paint pooled, and
  99 min is less than 10 × 10.07 (`logs/card_first_d20.txt`).
- **At 18548cf** (no wetting film), the card kept 89.0% and 86.3%.

## Thinner settings swept (the check 8 test, 480 px card, each setting its own card)

Raw sienna, contrast kept after the solvent has gone (`logs/all_afc7898.txt`):

| thinner | 0 | 0.1 | 0.2 | 0.3 | 0.4 | 0.5 | 0.6 | 0.7 | 0.8 | 0.9 |
|---|---|---|---|---|---|---|---|---|---|---|
| load 0.3 | 19% | 26% | 48% | 64% | 74% | 82% | 89% | 94% | 97% | 99% |
| load 0.6 | 5% | 15% | 35% | 53% | 68% | 78% | 86% | 92% | 97% | 99% |

At 480 px the card at thinner 0 keeps more (19% and 5%) than at 2400 px.
DIAGNOSIS.md found the same width dependence.

## The estimates and their sweep

| quantity | value | status | source |
|---|---|---|---|
| ceiling of wet film one stroke adds to a pixel | 6 µm at thinner 0.5; `6 × (1 − t)/t` µm, unbounded at t = 0 | ESTIMATE, set before measuring | qualitative: a thinner liquid leaves a thinner film (Landau and Levich 1942, film ∝ (ηU)^(2/3)) |
| evaporation time τ | 2 min × (1 + h / 100 µm), h the pixel's paint | ESTIMATE; doubling changed from 20 to 100 µm (above) | "a few minutes" on paper (W. J. Pearce [Jennings], *Paint & Colour Mixing*, 1902, https://www.gutenberg.org/cache/epub/56738/pg56738-images.html); slower in thicker films (C. M. Hansen 1967, solvent retention). No source gives numbers for oil paint |
| mobility of the flow | 0.06 mm²/min at half solvent; scales as φ/(1 − φ); slowed by the oil's cure | ESTIMATE | Orchard 1962 leveling rate, with σ, h and η assumed (thinner.rs) |
| wetting film the flow leaves | 2 µm of liquid | ESTIMATE, added for the lattice defect | Orchard's h³: the last of a film barely moves |

The ceiling was swept from 6 to 36 µm. Card at 2400 px, contrast kept
at loads 0.3 / 0.6, with the paint tests that change with it
(`logs/ceiling_sweep.txt`; τ doubling 100, before the wetting film):

| ceiling | 6 µm | 8 | 9 | 10 | 12 | 24 | 36 |
|---|---|---|---|---|---|---|---|
| card load 0.3 / 0.6 | 89% / 86% | | | | 83% / 79% | 75% / 69% | 70% / 62% |
| c05 (brush runs out) | fail | fail | fail | fail | pass | pass | fail |
| c09 (evaporation law) | pass | pass | fail | fail | fail | fail | fail |
| c04, c16 | fail | fail | fail | fail | fail | fail | fail |

No value passes both c05 and c09. I kept the value set before measuring
(6 µm) and didn't tune it to either test. Re-review 5 below explains why.

## The rag (rag study, `notes/thinner/rag_study.jpg`)

The sheet has 8 rows. Each row is one panel (256 px) before and after:

1. dry wipe, wet paint
2. blot, wet paint
3. damp wipe, wet paint
4. dry wipe, thinned paint
5. blot, thinned paint
6. damp wipe, thinned paint
7. dry-paint control
8. stroke over a wiped area

| panel | paint removed | tone left | wipe or blot shape (elongation) |
|---|---|---|---|
| dry cloth wipe, wet paint | 90.5% | 47.6% | 5.6 |
| blot, wet paint | 23.6% | 87.9% | 1.1 |
| damp cloth wipe, wet paint | 97.2% | 20.8% | 5.3 |
| dry cloth wipe, thinned paint | 68.1% | 44.3% | 5.7 |
| blot, thinned paint | 21.2% | 83.0% | 1.1 |
| damp cloth wipe, thinned paint | 68.3% | 44.1% | 5.4 |
| dry paint (control) | 0% | 100% | (none lifted) |
| brush stroke over the wiped area | adds 3.3 → 8.5 µm where it crosses | | |

**The stain (the earlier 14% report).** A damp wipe over wet paint still
leaves **20.8%** of the tone's darkening, and a dry wipe 47.6%. That is
the rag's `STAIN_COATS` floor plus what lies in the hollows (rag.rs).
Unresolved: the rag can't wipe back to the ground in one pass, and this
study doesn't show whether more passes would.

**Damp against dry on thinned paint: 68.3% against 68.1%.** The spirits
make almost no difference there. A thinned film is only ~3 µm, and most
of it is within the cloth's reach either way. This passes the test's
"more" by 0.2 points; it is a question for the user (appearance 3).

## Needs re-review

None of the frozen files was edited. Each item gives the evidence and
the proposed change.

1. **`c03_af49348_engine_3_logs_still_replay`: a test bug.**
   - It counts chunks with `log.matches("--@ chunk").count()`. The log's
     own header comment ("Each "--@ chunk" line starts one chunk") also
     matches, so it expects 2 chunks where there is 1.
   - Fix: count lines that start with `--@ chunk`.
   - With only that change, the test passes. A temporary copy was run at
     afc7898's code and then deleted.
2. **`c03_a_log_keeps_its_engine`: a test bug.**
   - Its tiny log's `canvas{}` has no `linen=`, which `canvas{}`
     requires: "canvas: linen= (threads per cm…)".
   - Fix: add `linen=15`. With that, it passes: engines 1, 2 and 3 are
     kept.
3. **`c18_a_failed_chunk_after_thinned_paint_takes_everything_back`: a
   test bug.**
   - `let (bytes0, clock0, hand0) = (state(&s.canvas().unwrap()), s.st.borrow().clock, held(&s));`
     keeps `s.canvas()`'s and `s.st.borrow()`'s temporaries borrowed
     while `held` takes `borrow_mut`, so it fails with "RefCell already
     borrowed".
   - Fix: three separate `let`s. With that, it passes: the failed chunk's
     canvas, solvent, clock, brushes and the held rag are all restored,
     and the next two chunks match the clean session.
4. **`c04_a_stroke_and_a_wipe_…`: the 0.9 case's guard conflicts with the
   ceiling.**
   - The guard "the brush laid paint" means the brush's paint fell over
     the stroke.
   - Lead white thinned 0.9 lays at most 0.67 µm of wet film per pixel
     (0.07 µm of paint). Crossing the thinned underlayer, it picks up up
     to the filbert's 18% of that film's paint.
   - So it ends the stroke carrying more paint than it started with, as a
     solvent-heavy brush dragged through wet paint would.
   - The balances (the check itself) aren't reached. Proposal: guard on
     what the stroke laid (paint on the canvas outside the underlayer
     rises), or move the 0.9 stroke off the underlayer.
5. **`c05_an_emptying_brush_…`: the stroke is too short for the ceiling.**
   - The plan requires that paint not laid stays on the brush. So with a
     6 µm ceiling, a filbert 8 thinned 0.5 lays about 90-115 volume units
     per 120 units of travel and still holds 61% (load 0.3) and 74%
     (load 0.6) of its liquid after 960 units. That was measured by a
     scratch probe, since deleted.
   - It doesn't run out within the test's one 960-unit stroke. It would
     over about 3000.
   - Ceilings of 12-24 µm pass it, but then check 9 fails (above).
   - Proposal: the same assertions along a longer path without reloading,
     for example several 960-unit rows.
6. **`c16_brush_rag_and_spreading_…`, part (3): the fixture's assumption
   fails under the ceiling.**
   - The test assumes the ratio-1 film (four passes at thinner 0.5) is
     thicker than the ratio-1/9 film (one light pass at thinner 0.1).
   - With the ceiling it isn't. The four passes reach about 9.6 µm (each
     stroke adds ≤ 6 µm and picks some up). The thinner-0.1 pass,
     ceiling 54 µm, lays about 12 µm.
   - So the liquid flows from the 1/9 side into the ratio-1 side, and the
     measured ratio barely moves (0.3215 → 0.3219).
   - The other spreading assertions pass: the solvent balance to 1e-4,
     every pixel's ratio within its neighborhood, and the movement guard.
   - Proposal: make the ratio-1 film the higher one (more passes, or a
     thinner ceiling for the low-ratio film), or take the inflow side
     from the data.

Check 9 passes at the chosen settings. Its allowance, though, assumes
neighboring pixels share a solvent ratio. A probe at τ doubling 20 µm
found 5 of 1,661 pixels over the limit in minute 1. Each had unchanged
paint, between rows 3 and 12 µm thick, whose ratios had drifted apart.
Raising the ceiling brings that back. Noted for the reviewers, not
blocking.

## What was decided, and the limits (guide: `notes/easel_guide.md`, "Thinner")

- **Engine 3 only.** On engines 1 and 2, `thinner=` is an error, and
  `p.thinner` is nil.
- **Unthinned paint takes today's path** (check 2), digests included
  since 0a661f6.
  - Before engine 3, a brush's `Debug` text, which `--state-digest` hashes
    as `brushes=`, is af49348's byte for byte.
  - An engine-3 brush also names each bristle's `solvent`. The protected
    dumper needs it (check 2 part 3).
  - Checked with the speed branch's approved old-log answers, read-only,
    against this build: `scripts/tests/old_logs.sh` replays all 11 cases
    (engine 1, legacy and round 19) exactly as af49348 did, PNGs and
    per-chunk digests. Check 2 still passes.
- **Left out on purpose** (said in the guide):
  - solvent evaporating from the brush or the palette pile;
  - soaking into the ground;
  - dissolving set or dry paint;
  - extra pickup of solvent-wet paint.
- **Brush and rag pickup** move solvent in the local ratio.
- **The flow runs once per whole minute of the grid**, and evaporation
  and drying over every step. That is why a split wait is exact on the
  grid (check 17).
- **Old files:** PAINTCK8 saves of engine-3 canvases are refused from
  the header, naming af49348. Engine-1/2 logs and saves (PAINTCK8) work
  as before.
- **The palette look** shows a thinned pile's swatches as the paint left
  after the solvent: each coat scaled by 1 − thinner.

## Pictures

- **`notes/thinner/thinner_sheet.jpg`** (`scripts/thinner_sheet`, 256 px
  panels). The rows:
  1. raw sienna, unthinned;
  2. raw sienna, thinner 0.5;
  3. lead white, unthinned;
  4. lead white, thinner 0.5 (each at loads 0.1, 0.3 and 0.6, 40 min after
     the pass);
  5. raw sienna thinner 0.5 at load 0.3, 0, 2, 5 and 15 min after the pass.

  The real post-evaporation measurement is the card above (35 min).
- **`notes/thinner/rag_study.jpg`**: the rag study's sheet, as JPEG (the
  test writes a 1 MB PNG; not committed). Re-rendered in round 4 with the
  current code (after afc7898's wetting film); the numbers are those in
  the table above, which `--all` at afc7898 printed too.

## For the user

- **Brush capacity.** A thinned brush lasts much longer than a loaded
  one: it lays at most a thin film per stroke and keeps the rest.
  - On its 7680-unit zigzag (round-4 c05), a filbert 8 thinned 0.5 keeps
    laying paint above a quarter of the stroke's ceiling for about 2,150
    units at load 0.3 (about two canvas widths) and 4,820 at load 0.6
    (about five), then runs out.
  - Unthinned, the same brush is nearly empty after one width.
  - That follows from the plan's rule that unlaid paint stays on the
    brush. Is it how a thinned brush should behave?
- **The fine grain in thinned paint**, below (appearance 1).
- **Damp rag about the same as a dry rag on thinned paint:** 68.3%
  against 68.1% lifted (appearance 3).
- **The stain:** a damp wipe over wet paint leaves 20.8% of the tone's
  darkening (the earlier report said about 14%), and a dry wipe 47.6%.
  The rag can't wipe back to the ground in one pass.
- **τ's thickness dependence was changed after a run: an estimate,
  adjusted.** `TAU_DOUBLING_UM` went from 20 µm to 100 µm (18548cf).
  - **Why:** check 1's guard. As thinned paint pooled in the weave's
    hollows during the wait, the deepest film grew from 77 to 81 µm, so τ
    at the measurement (10.07 min) exceeded a tenth of the 99 min waited.
    At 20 µm, check 9 also had 5 of 1,661 qualifying pixels over its
    limit in minute 1.
  - **Effect on the card:** it barely moves, 88.9% kept at load 0.3 with
    20 µm against 89.0% with 100 µm. A thicker film still holds its
    solvent longer, just less so.
  - Neither number has a source.
- **The cross-hatch in thinned paint** (rag study rows 4-6) is the brush
  marks, not the weave. `notes/thinner/weave_or_brush.jpg` shows the same
  thinned broad pass with linen (left) and on a plain ground (right), and
  they look alike: each stroke lays only up to its own thin ceiling, so
  the overlaps of the strokes read as a hatch.
- Also open: check 13 (b), check 3's old-file policy, check 8's 0.005
  allowance, and the c16 correction (TESTS_PHASE_REPORT.md, rounds 4 and
  5).

## Appearance questions for the user

1. **Thinned paint has a fine grain at every size.** At 256 px it is a
   speckle. At 2400 px it is a faint trace of the weave in the hollows,
   after the fix below. Is that the texture of a scrubbed-in lay-in, or
   too much?
2. **The lattice defect, fixed.** Before afc7898, at 2400 px, thinned
   lead white over black was a lattice of white threads on bare black:
   the flow drained the weave's tops. It now leaves a 2 µm wetting film
   and reads as a grey veil. The fix is an estimate.
3. **The damp rag on thinned paint** lifts barely more than a dry one
   (68.3% against 68.1%).
4. **The 0, 2, 5 and 15 minute row looks the same.** Solvent has no
   color, and at 256 px the flow (about 0.25 mm a minute) is below a
   pixel. The change is the solvent leaving, which isn't visible.

## Existing tests

- `cargo test --release -p paint --lib`: 180 passed, 0 failed (8 ignored, as before).
- The easel binary's own tests (all but the thinner's): 71 passed, 0 failed, 1 ignored. These include the engine-1 and engine-2 fixture replays at 320 px and the checkpoint tests.
- Integration tests: easel `determinism` (4) and `session_integrity` (10), paint `curved_drag_nan` (2) and `ground_grain` (1): all pass.
- Not run, because they replay paintings or are the speed branch's: the easel `boxes` (a round-19 log), `painter`, `delivery` and `smoke` tests, and `scripts/tests/*.sh`. The plan forbids expensive replays. The speed agent's `scripts/test` is the reduced suite for the merged branch.
