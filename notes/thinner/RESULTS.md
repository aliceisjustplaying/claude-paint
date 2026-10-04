# Thinner: results

The thinner is implemented in engine 3 on branch `thinner2` (not pushed).

**Final state.** The final acceptance run (`--all`, below) passes all 27
required tests, the card included. Its one failure is check 13 (b), the
pre-existing finding left to the user, so it exits 3 ("NOT ALL GREEN").
The tests are the frozen set at 48be56a: the round-3 approval (b2a0e14)
plus the corrections approved in rounds 4 and 5
(`~/src/a/claude-paint-reviews/thinner-tests-review-astra-r5.md`;
TESTS_PHASE_REPORT.md). The protected baseline (`notes/thinner/baseline/`)
is untouched since a97c3a6. The dumpers gained one field, under a new
name.

**The card target is met at 2400 px**: thinned raw sienna keeps 84.4% of
the card's contrast at load 0.3 and 81.4% at load 0.6. Today, unthinned,
it keeps 13% and 6% (DIAGNOSIS.md).

| commit | what |
|---|---|
| df9058c | the thinner: solvent beside the paint, the per-stroke ceiling, evaporation on the minute grid, the flow, PAINTCK9, Lua, dumper field |
| 18548cf | τ doubles at 100 µm, not 20 (the card's wait guard); the guide's Thinner section |
| afc7898 | the flow leaves a wetting film (the 2400 px lattice defect); `scripts/thinner_sheet` |
| 1e27682 | the first RESULTS.md, sheets, logs |
| 0a661f6 | a brush's `Debug` text (the `brushes=` digest, the dump's `brushes.debug`) is af49348's byte for byte before engine 3 |
| e135129 | round 4 test corrections: c03 ×2, c18, c04, c05 (approved) |
| 4c15082, ffef02e | the rag study re-rendered after afc7898; RESULTS.md |
| 48be56a | round 5 test corrections: c16, the rag sheet's directory (approved; the frozen set) |
| 7687ad3 | τ disclosure; `weave_or_brush.jpg` |
| (this commit) | the final run; `single_stroke_edge.jpg`; the mutation sources; this file |

The thinner's code last changed in 0a661f6. Every commit after it
changes only tests, scripts or notes.

## The final runs

Every command ran through `~/src/a/claude-paint-tools/lockrun` on this
machine (rustc 1.97.1, release), at 7687ad3's code and tests.

| command | result | exit | time | log (sha256) |
|---|---|---|---|---|
| `THINNER_RAG_STUDY_DIR=<temp dir> scripts/test_thinner_acceptance --all` (lockrun `--timeout 600`) | 27 of 27 required tests pass; 13 (b) the expected failure | **3** (NOT ALL GREEN) | 146 s in lockrun (431 s with waiting for the lock) | `logs/all_final.txt` (`6a88606e…aab1`) |
| `cargo test --release -p paint --lib` (existing tests; lockrun `--timeout 600`, as are the next two) | 180 passed, 0 failed, 8 ignored | 0 | 146 s | `logs/paint_lib_final.txt` (`fd19bb27…c1eb`) |
| `cargo test --release -p easel --bin easel -- --skip thinner_tests::` (existing tests) | 71 passed, 0 failed, 1 ignored | 0 | 49 s | `logs/easel_bin_final.txt` (`004914bc…d95c`) |
| `cargo test --release -p easel --test determinism --test session_integrity`, then `-p paint --test curved_drag_nan --test ground_grain` | 4 + 10 + 2 + 1 passed, 0 failed (ground_grain's 3 ignored as before) | 0, 0 | 82 s + 5 s | `logs/integration_final.txt` (`00f49129…e0e9`) |

`--all` runs the card. The earlier separate `--card` run
(`logs/card_18548cf.txt`, exit 3) predates afc7898 and is superseded by
this one.

## Each check (from `logs/all_final.txt`)

Each test runs in one of `--all`'s cargo sections, so the time is the
section's. Every check exits as its section did: 0, except 13 (b)'s (101).

| check | test | result | section, time |
|---|---|---|---|
| 1 | `c01_the_card_…_2400px` (slow) | **pass** (the card, below) | card, 23 s |
| 2 | `scripts/thinner_check2` | **pass**: all 4 parts. Every scene and chunk is equal to af49348 on every baseline field, with and without `thinner=0`. The dumps declare the solvent, and the thinned scene dumps nonzero solvent | check 2, 36 s (with the build) |
| 3 | `c03_af49348_paintck8_saves_are_refused_…`, `c03_engines_1_and_2_have_no_thinner_…`, `c03_af49348_engine_3_logs_still_replay`, `c03_a_log_keeps_its_engine` | **pass** (4 of 4) | easel, 37 s |
| 4 | `c04_a_stroke_and_a_wipe_…`, `c04_control_…` | **pass** (2 of 2) | paint, 3 s |
| 5 | `c05_an_emptying_brush_…` | **pass** | paint, 3 s |
| 6 | `c06_more_pressure_…` | **pass** | paint, 3 s |
| 7 | `c07_two_overlapping_thinned_passes_…` | **pass** | easel, 37 s |
| 8 | `c08_a_hundredth_of_thinner_is_a_small_change` | **pass** | paint, 3 s |
| 8 | `c08_more_thinner_never_hides_the_card_more` (slow) | **pass** (the sweep, below) | easel-slow, 45 s |
| 9 | `c09_the_solvent_evaporates_…` | **pass** | paint, 3 s |
| 10 | `c10_a_solvent_wet_film_spreads_more_…` | **pass** | paint, 3 s |
| 10 | `c10_thinned_and_unthinned_paint_…_gel_and_dry_together` (slow) | **pass** | paint-slow, under 1 s |
| 11 | `c11_a_save_mid_evaporation_…` | **pass** | easel, 37 s |
| 12 | `c12_the_same_log_saves_the_same_bytes_on_one_thread_and_four` | **pass** | easel, 37 s |
| 13 (a) | `c13_every_pigment_value_is_af49348s` | **pass** | pigments, 1 s |
| 13 (b) | `c13_burnt_sienna_shows_the_card_…` | **expected fail** (pre-existing; the user decides) | check13b, 1 s, cargo exit 101 |
| 14 | `c14_more_points_on_the_same_path_…` | **pass** | paint, 3 s |
| 15 | `c15_solvent_does_not_slow_the_oil_cure` | **pass** | paint, 3 s |
| 16 | `c16_brush_rag_and_spreading_…` | **pass** | paint, 3 s |
| 17 | `c17_a_wait_split_on_the_minute_grid_…` | **pass** | paint, 3 s |
| 18 | `c18_a_failed_chunk_after_thinned_paint_…` | **pass** | easel, 37 s |
| 19 | `c19_the_same_paint_with_more_or_less_solvent_looks_the_same` | **pass** | paint, 3 s |
| rag | `rag_study` (slow) | **pass**; sheet written (below) | easel-slow, 45 s |

Unfinished: none.

## The card (check 1, 2400 px, raw sienna thinned 0.5)

The wait is ceil(10 τ) + 1 minutes, τ of the thickest paint on the card.

| load | contrast kept | waited | τ after the pass / at the measurement | solvent in the strip: after the pass → at the measurement (µm-pixels) | whole canvas: left at the measurement, of what was there after the pass |
|---|---|---|---|---|---|
| 0.3 | **84.4%** | 35 min | 3.328 / 3.328 min | 0.0022 → 0 | 7.0e-7 (1.24e-5 of 17.78) |
| 0.6 | **81.4%** | 35 min | 3.328 / 3.328 min | 2549 → 0.0018 | 7.0e-7 |

The load-0.3 strip was painted first, so its solvent had mostly gone
during the rest of the pass.

Earlier runs:

- **At df9058c** (τ doubling at 20 µm), load 0.3 kept 88.9%. The run
  failed its own guard: τ grew from 9.72 to 10.07 min as paint pooled, and
  99 min is less than 10 × 10.07 (`logs/card_first_d20.txt`).
- **At 18548cf** (no wetting film), the card kept 89.0% and 86.3%.

## Thinner settings swept (the check 8 test, 480 px card, each setting its own card)

Raw sienna, contrast kept after the solvent has gone (`logs/all_final.txt`; the same at afc7898):

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
(`logs/ceiling_sweep.txt`; τ doubling 100, before the wetting film,
and the tests as approved in round 3, before their corrections):

| ceiling | 6 µm | 8 | 9 | 10 | 12 | 24 | 36 |
|---|---|---|---|---|---|---|---|
| card load 0.3 / 0.6 | 89% / 86% | | | | 83% / 79% | 75% / 69% | 70% / 62% |
| c05 (brush runs out) | fail | fail | fail | fail | pass | pass | fail |
| c09 (evaporation law) | pass | pass | fail | fail | fail | fail | fail |
| c04, c16 | fail | fail | fail | fail | fail | fail | fail |

No value passed both the round-3 c05 and c09. I kept the value set before
measuring (6 µm) and didn't tune it to either test. A thinned brush keeps
what it can't lay, so it still held most of its liquid at the end of
c05's original 960-unit stroke. Round 4 lengthened the stroke
(TESTS_PHASE_REPORT.md), and at 6 µm every check now passes.

## The test corrections (rounds 4 and 5, approved)

Six required tests failed at afc7898 against the round-3 tests
(`logs/all_afc7898.txt`). None of them was edited until a review
approved its change. Details are in TESTS_PHASE_REPORT.md.

- **Three bugs in the test code (round 4):**
  - c03 counted the log's header comment as a chunk;
  - c03's tiny log lacked the `linen=` that `canvas{}` requires;
  - c18 held a `RefCell` borrow across `borrow_mut`.
- **Three setups that conflicted with the plan's physics:**
  - c04 (round 4): a solvent-heavy brush can end a stroke fuller. Its
    guard is now that the stroke moved paint; the balances are
    unchanged.
  - c05 (round 4): a thinned brush keeps what it can't lay. The stroke is
    now a 7680-unit zigzag with no reload.
  - c16 (3) (round 5): the low-ratio film is laid first, so the ratio-1
    film ends in an edge beside it, and the measured pixels are chosen
    from the state before the minute. The mutation demos are in
    `logs/round5_mutations.txt`, with each mutation's diff and the probe
    to rerun them: paint moving without its solvent fails the rise
    assertion itself.

No assertion or limit changed.

Check 9 passes at the chosen settings. Its allowance, though, assumes
neighboring pixels share a solvent ratio. A probe at τ doubling 20 µm
found 5 of 1,661 pixels over the limit in minute 1. Each had unchanged
paint, between rows 3 and 12 µm thick, whose ratios had drifted apart.
Noted for the reviewers, not blocking.

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
    against 0a661f6's build: `scripts/tests/old_logs.sh` replays all 11 cases
    (engine 1, legacy and round 19) exactly as af49348 did, PNGs and
    per-chunk digests. Check 2 still passes.
- **The limits, left out on purpose** (said in the guide's Thinner
  section):
  - no solvent is lost from the palette pile or the brush (it evaporates
    only from the canvas);
  - no soaking into the ground;
  - thinner doesn't dissolve set or dry paint;
  - no extra pickup of solvent-wet paint (the brush and rag pick up
    thinned paint as they do unthinned).
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
  test writes a 1 MB PNG; not committed). Rendered in round 4 with the
  current code. The final run's sheet is the same: the code hasn't
  changed since, the run printed the same numbers, and it differs from
  the JPEG only by the JPEG's compression (PSNR 35.3 dB). So it was not
  copied in again.
- **`notes/thinner/weave_or_brush.jpg`**: the same thinned broad pass with
  linen and on a plain ground (256 px).
- **`notes/thinner/single_stroke_edge.jpg`**: one filbert stroke on a plain
  ground (256 px), thinned 0.5 just after and 30 min after, and unthinned
  (`logs/single_stroke_edge.txt`).

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
  they look alike.
- **A thinned stroke has a dark rim at its free edge, which the flow
  doesn't level.** In `notes/thinner/single_stroke_edge.jpg` a single
  thinned stroke on a plain ground holds 2.3 and 2.1 µm of paint in its
  two edge rows against about 0.9 µm inside. After 30 min it still holds
  2.0 and 1.9 µm (`logs/single_stroke_edge.txt`). So the hatch's outlines
  are each stroke's own edges, not only the overlaps. The unthinned
  stroke (17-49 µm inside) shows no such rim. Diagnostic only; nothing
  was tuned, and I haven't traced the rim's cause.
- Also open: check 13 (b), check 3's old-file policy and check 8's 0.005
  allowance.

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

## Existing tests (final run)

- `cargo test --release -p paint --lib`: 180 passed, 0 failed (8 ignored, as before).
- The easel binary's own tests (all but the thinner's): 71 passed, 0 failed, 1 ignored. These include the engine-1 and engine-2 fixture replays at 320 px and the checkpoint tests.
- Integration tests: easel `determinism` (4) and `session_integrity` (10), paint `curved_drag_nan` (2) and `ground_grain` (1): all pass.
- Not run, because they replay paintings or are the speed branch's: the easel `boxes` (a round-19 log), `painter`, `delivery` and `smoke` tests, and `scripts/tests/*.sh`. The plan forbids expensive replays. The speed agent's `scripts/test` is the reduced suite for the merged branch.
