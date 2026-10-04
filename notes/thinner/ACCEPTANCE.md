# Thinner acceptance checks

The 19 thinner checks and the rag study from the overnight plan
(`~/src/a/claude-paint-overnight-plan.md`, sha256
`382fc2debb7a56505f9b4723c6591f22644feccb330fef62dc723b635f098e30`), as
named tests, with commands, expected results and error limits. Written
before the thinner exists, for review. After approval these files are
frozen: the program changes to pass them, not the other way round.

Branch `thinner2`, on top of e39da64 (af49348 merged into the phase-1
diagnosis c692eae). The file hashes are at the end.

## How to run

All in release builds (the baseline was made in release, and the iter
build can change the last digits). One heavy job at a time: wrap each
command in `scripts/lockrun` once the lead approves it.

| command | runs | time limit |
|---|---|---|
| `scripts/test_thinner_acceptance --quick` | every check below except the four slow ones | 60 s once built |
| `scripts/test_thinner_acceptance --card` | check 1 only | 300 s |
| `scripts/test_thinner_acceptance --all` | everything, check 1 included | 600 s |

The script passes only if every required test ran and printed `ok`. A test
that is missing, filtered out, ignored or failed fails the run, so a
skipped test never counts as passed and a run that matches nothing fails.
It needs only cargo and bash (nothing from `codex/speed`). Check 1 is
`#[ignore = "slow"]` and runs by exact name with `--ignored`.

Single tests, exactly:

- paint: `cargo test --release -p paint --test thinner_physics -- --exact <name>`
- easel: `cargo test --release -p easel --bin easel -- --exact thinner_tests::<name>`
- check 1: `cargo test --release -p easel --bin easel -- --exact --ignored --nocapture thinner_tests::c01_the_card_raw_sienna_thinned_half_keeps_half_the_contrast_at_2400px`

## Files

| file | what |
|---|---|
| `crates/paint/tests/thinner_physics.rs` | checks 4, 5, 6, 8 (small thinner), 9, 10, 13, 14, 15, 16, 17, 19 against the paint engine's public API |
| `crates/paint/tests/thinner_support/mod.rs` | their measuring helpers, and the list of the interface they need |
| `crates/easel/src/thinner_tests.rs` | checks 1, 2, 3, 7, 8 (the card), 11, 12, 18 and the rag study, at the easel (Lua, sessions, saves, logs) |
| `crates/easel/src/thinner_measure.rs` | their helpers: the card, sums, the baseline reader, pictures |
| `crates/easel/tests/thinner/*.lua` | tiny inputs: a canvas, thinned strokes, a next chunk, a failing chunk, a four-chunk log |
| `crates/easel/src/main.rs` | two test-only lines that compile the easel modules (`#[cfg(all(test, feature = "replay"))]`) |
| `scripts/test_thinner_acceptance` | the commands above |

## The interface the tests need

What exists at af49348 is used as it is. New, for the builder to provide
(also listed in `thinner_support/mod.rs`):

- `Paint::with_thinner(t)`: the paint thinned with solvent share `t` (0 to 0.9).
- `Canvas::solvent_um(x, y)`: solvent in the open film at a point, µm. `Canvas::wet_um` stays the paint, solvent-free.
- `Canvas::solvent_total()`: all the solvent, in the units of `Canvas::wet_total` (coats × square units).
- `Canvas::cure_at(x, y)`: the open film's cure at a point (0 fresh).
- `Held::carried() -> (f64, f64)`: paint and solvent on the brush, in `wet_total`'s units.
- `Rag::solvent_mm3: f64` (a public field): solvent the cloth has taken off the canvas, mm³, cumulative. `rag_wipe` and `rag_blot` still return the paint lifted, mm³.
- `paint::thinner::stroke_limit_um(t) -> f32`: the most wet film (paint + solvent, µm) one stroke may add to a pixel; `f32::INFINITY` at `t = 0`.
- `paint::thinner::evaporation_tau_min(paint_um) -> f64`: the solvent's evaporation time (minutes) in a film holding `paint_um` of paint.
- The engine-3 save starts `PAINTCK9`, keeps every byte of the PAINTCK8 layout after the magic, and ends with the solvent: one little-endian f32 (µm) per buffer pixel, row major, as the file's last 4 × pixels bytes. Anything else new goes between the two.
- Lua, engine 3: `pile{..., thinner = t}`, `p.thinner` (0.0 when not given), and `print(p)` showing `, thinner t` after the medium only when `t > 0`. Engines 1 and 2: the key is an error, `p.thinner` is nil and `print(p)` is unchanged.
- `save::read` refuses a PAINTCK8 save of an engine-3 canvas from its header, naming af49348; `Canvas::read_state` refuses a PAINTCK8 checkpoint of an engine-3 canvas, naming af49348.

The last-bytes rule lets four tests (10 drying, 15, 17, 19) make "the same
paint with a different amount of solvent" through the save, without a
setter only tests would use.

## The checks

"Rounding" below means float error, not tolerance for different behavior.
Every limit was chosen before any thinner code exists and without running
the tests (no heavy-job turn yet); where a limit could be wrong for
today's code, the test carries an unthinned control on today's code path
that must pass the same limit.

### 1. The card (locked target)

`thinner_tests::c01_the_card_raw_sienna_thinned_half_keeps_half_the_contrast_at_2400px`, `#[ignore = "slow"]`, `--card`.

Raw sienna, `thinner=0.5`, one `body` pass (`clip=true`) at load 0.3 and one
at 0.6, side by side over the card (Inness toned ground, a bone-black and a
lead-white band dried 60 days), 2400 px. After the passes the test waits
`ceil(10 τ) + 1` minutes, with τ the evaporation time of the thickest paint
on the card. Expected, in this order:

- the wait is at least 10 τ, for τ after the passes and τ at the measurement;
- solvent left in each strip < 0.001 × the solvent there right after its
  pass. The plan says "of the added solvent". Some solvent evaporates during
  the pass, so what is there after it is no more than what was added, and
  this is the stricter test;
- each strip shows ≥ 50% of the card's black/white contrast. Contrast is the
  luminance of the white band's mean color minus the black band's, after
  the pass over before it.

Measured on the open paint once the solvent has gone, not after drying.

### 2. No thinner

`thinner_tests::c02_no_thinner_or_thinner_0_leaves_the_saved_paint_state_unchanged`.

Each before-change scene in `notes/thinner/baseline/` is replayed twice:
its log as it is, and with `thinner=0` added to every `pile{`. Expected:
the new save starts `PAINTCK9`; bytes 8 to the end of the old save are the
same as the old PAINTCK8 save's (every existing paint-state value,
exactly); the solvent section is all zero; `solvent_total() == 0`.

**TODO (baseline).** The baseline is the speed agent's, from unchanged
af49348 on `codex/speed`; it isn't in this branch yet. The reader,
`thinner_measure::baseline_scenes`, assumes `scenes.txt` with one scene a
line (`<name> <width px> <log file> <state file>`), the state file being
`Canvas::write_state(.., "")` bytes. When the lead names the baseline
commit and it is merged, the reader is adapted to its real format and
reviewed again; the test and its rule stay. Until then the test fails with
"no before-change baseline in this branch yet".

### 3. Old files

- `thinner_tests::c03_a_paintck8_save_of_an_engine_3_canvas_is_refused_by_its_header_naming_the_old_version`.
  The input is a tiny file: `PAINTCK8`, a save header saying `engine=3`, and
  no canvas after it. `save::read` must return an error that contains
  `af49348`, not the "failed to fill whole buffer" of running out of file.
- `thinner_tests::c03_a_baseline_paintck8_checkpoint_is_refused_naming_the_old_version`:
  the baseline's real PAINTCK8 checkpoints. `Canvas::read_state` must
  refuse each one with an error that contains `af49348` (same TODO as check 2).
- `thinner_tests::c03_engines_1_and_2_have_no_thinner_and_engine_3_has`:
  on engines 1 and 2, `thinner=` is an error and a pile prints
  `nil	pile(raw sienna 1; medium 0)`. On engine 3: `0.0	pile(raw sienna 1; medium 0)`
  without it and `0.5	pile(raw sienna 1; medium 0, thinner 0.5)` with it;
  0.95, -0.1 and a string are errors.

The engine-1/2 replays themselves stay covered by the existing tests
(`engine_2_logs_replay_as_before`, `logs_without_an_engine_line_replay_as_before`).
See open question 1.

### 4. Nothing vanishes

`c04_a_stroke_and_a_wipe_account_for_all_paint_and_solvent`.

The test lays a thinned underlayer, then draws four strokes across it:
raw sienna thinned 0.5, lead white thinned 0.2 and 0.9, and unthinned
raw sienna as the control. A stroke puts no time on the clock (asserted),
so nothing evaporates during it. Expected, per stroke:
canvas + brush paint before = after, and canvas + brush solvent before =
after. The unthinned brush comes away carrying solvent, which proves the
pickup moved solvent with the paint. The rag is wiped dry and then dipped
in spirits. Its expected balance: canvas paint before = after + what
`rag_wipe` returned, and canvas solvent before = after + `Rag::solvent_mm3`.

**Rounding: 1e-4 of the total counted** (`BALANCE_REL`). The totals are
f32 per-pixel amounts summed in f64. A stroke changes about 10^4-10^5
pixels, each by f32 arithmetic with a relative error of 6e-8. Random-walk
accumulation gives about 2e-5 of the total; 1e-4 allows five times that.
Real loss is larger: one bristle's share is about 1/120 of the brush, and
a missed deposit or pickup is a few per mille or more. The unthinned
control holds today's code to the same 1e-4.

### 5. The brush runs out

`c05_an_emptying_brush_lays_less_and_a_fuller_load_lasts_farther`.

A filbert 8, raw sienna thinned 0.5, one 960-unit stroke, at load 0.3 and
then at 0.6. Paint is measured along the stroke's middle in 10-unit bins.
θ is a quarter of the stroke limit's paint, `0.25 × stroke_limit_um(0.5) × 0.5`.
Expected:

- it starts above θ;
- the last 100 units average under half the start;
- it falls below θ before the stroke's end;
- at load 0.6 it stays above θ at least 30 units farther than at 0.3.

θ is an absolute level, so the test holds whether or not the limit binds
at load 0.3.

### 6. Pressure

`c06_more_pressure_lays_more_paint_up_to_the_stroke_limit`.

The same thinned load at pressure 0.3, 0.6 and 0.9. Expected:

- each step lays at least 5% more paint, measured off the brush;
- no pixel gains more wet film than `stroke_limit_um(0.5)`, allowing for
  rounding of 1e-3 relative plus 1e-3 µm.

5% sits below the mark-width growth between steps, which is about 30%.

### 7. Overlap

`thinner_tests::c07_two_overlapping_thinned_passes_leave_more_paint_than_one`.

Two thinned `body` passes go over the same rectangle. Expected: at least
20% more paint after the second than after the first. Each stroke gets a
new limit, and a pass's pickup takes much less than it lays.

### 8. Smooth changes

- `c08_a_hundredth_of_thinner_is_a_small_change` (paint). The same five
  strokes are drawn with no thinner, with thinner 0 and with thinner 0.01.
  Expected:
  - thinner 0 gives the same save bytes as no thinner;
  - at 0.01, total paint stays within 3%;
  - the mean per-pixel difference stays within 3% of the mean film;
  - solvent is above 0 and at most 2% of the paint.

  3%: one hundredth of the load is solvent, and dips and pickups may
  triple that, not more.
- `thinner_tests::c08_more_thinner_never_hides_the_card_more` (easel, slow).
  The card at 960 px (the plan fixes no width for this check), raw sienna,
  loads 0.3 and 0.6, thinner 0, 0.1, …, 0.9, each measured after its
  solvent has gone. Expected:
  - the contrast showing never falls from one step to the next by more
    than 0.005 (half a percentage point, measurement noise from different
    deposits sampling the weave differently);
  - 0.9 shows more of the card than 0.

### 9. Evaporation

`c09_the_solvent_evaporates_and_the_film_loses_its_volume`. Expected:

- τ(h) is finite and positive and never shorter for a thicker film, at
  0.5 to 200 µm;
- in a thinned patch, total solvent falls every minute;
- total paint stays the same (rounding 1e-4), so the wet film loses
  exactly the solvent's volume;
- after `ceil(10 τ_max) + 1` minutes less than 0.001 of the solvent is left.

That solvent adds no color is check 19's: one test owns that contract.

### 10. Wet handling and drying

- `c10_a_solvent_wet_film_spreads_more_than_after_the_solvent_has_gone`.
  The patch is thinned, on a flat ground, so the paint's own ridges are
  the only relief. "Roughness" is the standard deviation of relief + paint.
  Expected:
  - over one τ of the median film, with solvent present, roughness falls
    by more than 2% of itself;
  - over the same span after the solvent has gone (film still open), it
    falls by less than half as much.

  This requires the model to level solvent-wet paint visibly within
  minutes (open question 4).
- `c10_thinned_and_unthinned_paint_of_equal_thickness_gel_and_dry_together`.
  Film A is a thinned patch. Film B is A with its solvent taken out
  through the save: the same paint in the same places, at equal remaining
  thickness. Each pixel of film is checked every 30 minutes for 20 days.
  **Limit: the median gel time and the median touch-dry time agree within
  3% or 30 min, whichever is longer.** The reasoning:
  - 30 min is the check interval.
  - 3% allows for solvent-driven leveling moving local thickness by a few
    percent (drying time ∝ thickness^0.7, `drying::THICK`).
  - Wrong inputs miss by 14% or more. If the wet thickness (paint +
    solvent) went into drying, thinner 0.5 doubles it and the time rises
    2^0.7 ≈ 1.6×. If solvent-softened stiffness went in, the `FAT` term
    (`drying.rs`) adds up to 1.6× (e.g. stiffness 0.5 → 0.2: 1.3 → 1.48,
    +14%).
- Extra pickup strength from solvent: **proposed as omitted** in this
  version. With it omitted there is nothing to test. If the builder adds
  it, a test is written and reviewed first.

### 11. Save and reopen

`thinner_tests::c11_a_save_mid_evaporation_reopens_to_the_same_state_and_goes_on_the_same`.

Inputs: `canvas.lua`, `thinned.lua`, then `wait(m)`, with m = τ(median
film) × ln 2, rounded, + 0.37 min. Asserted before saving: 10-90% of the
solvent is left, and the clock is between whole minutes. Expected:

- `save::write` then `save::read` give the same canvas bytes;
- `next.lua` (a thinned stroke, a wait, a rag) run in both gives the
  same bytes;
- a replay of the whole log from scratch gives the same bytes.

### 12. Repeatable results

`thinner_tests::c12_the_same_log_saves_the_same_bytes_on_one_thread_and_four`.

`determinism.lua` is four chunks: thinned strokes and passes, waits of
3.3 and 1 min, and a damp rag's wipe and blot. It is replayed in a
one-thread and a four-thread rayon pool. Expected: identical save bytes,
with solvent on the canvas at the end.

### 13. Pigment ordering

`c13_pigment_values_are_unchanged_and_burnt_sienna_is_the_more_transparent`.

Expected:

- raw sienna (`#9a6a2b`, hiding 0.4, stiffness 0.5, strength 0.7) and
  burnt sienna (`#7c3f24`, 0.45, 0.55, 0.9) keep af49348's values bit for bit;
- burnt sienna scatters less per coat (Field/Salter 1869 §155).

### 14. Stroke points

`c14_more_points_on_the_same_path_lay_the_same_paint_and_one_stroke_shares_one_limit`.

The same straight path is given as 5, 10 and 50 points.

**"Meaningfully": every pixel's wet film within 2% of the stroke limit,
and the total within 0.5%.** For the unthinned control the 2% is of the
thickest film. The reasoning:

- `drag_on` (bristle.rs) resamples every path to its own step, so extra
  collinear points change only float rounding of arc length.
- 2% leaves room for that.
- A limit that resets per point or per segment multiplies the film by
  the points added, at least 2×.

One stroke shares one limit. A brush of 400 overlapping hairs (filbert
20, `hair` 2.5), and one stroke that goes over its own path four times,
must add no more than `stroke_limit_um(0.5)` to any pixel (rounding
1e-3 relative + 1e-3 µm).

### 15. Drying inputs

`c15_solvent_does_not_slow_the_oil_cure`.

Film A (with solvent) and film B (A with its solvent taken out through
the save) wait three τ. Expected: the median over the film of A's cure ÷
B's is within 2% of 1. The wrong inputs fall at least 14% short (see 10).

### 16. Moving paint

`c16_brush_rag_and_spreading_carry_solvent_in_the_local_ratio`.

Two fresh films sit side by side: raw sienna thinned 0.5 (solvent ÷ paint
= 1) and lead white thinned 0.2 (0.25). Expected:

- every pixel holds its film's ratio within 1e-3;
- a clean brush through the first film carries ratio 1 within 1e-3;
- a rag wiped over each film takes that film's ratio within 1e-3, and
  leaves every pixel's ratio within 1e-4 of what it was (the rag scales a
  pixel's paint and solvent by the same factor, so only f32 rounding
  differs);
- spreading: after one minute on a fresh film of one ratio, each pixel's
  change of ln(ratio) is corrected for evaporation alone,
  +1/τ(h) at the thickness before or after, whichever explains it better.
  It is then regressed on ln(paint after ÷ paint before). Expected slope
  between -0.5 and 0.5, with at least 50 pixels changing thickness by
  more than 1%. The slope separates the right model from the wrong ones:
  - solvent that moves with its paint gives about 0;
  - solvent left behind gives about -1: a pixel that gains paint and no
    solvent loses ratio in proportion;
  - the rest of the band allows for evaporation measured at a thickness
    between the two and for mixing with neighbors.

### 17. Waits

`c17_a_wait_split_on_the_minute_grid_is_exact_and_off_it_within_1e4`.

Two thinned patches are made, starting on a whole minute. Expected:

- `wait(15)` and 15 × `wait(1)` give the same save, byte for byte, with
  solvent present throughout;
- `wait(7.3)` then `wait(7.7)` (the same clock: 7.3f32 + 7.7f32 = 15
  exactly) agree with `wait(15)` at every pixel within 1e-4 relative in
  paint µm, solvent µm and cure.

**Zero rule:** two values agree if they are equal, or both are no further
from zero than a floor, or they are within 1e-4 of the larger in
magnitude. The floors:

- paint and solvent: 1e-5 coats = 2.5e-4 µm, the engine's own threshold
  for bare canvas;
- cure: 1.5e-7, a millionth of the way to the gel point.

### 18. Undo after failure

`thinner_tests::c18_a_failed_chunk_after_thinned_paint_takes_everything_back`.

Two sessions run `canvas.lua` and `thinned.lua`. One then runs
`failing.lua`: a thinned stroke, a thinned pass, `wait(1.5)`, a damp rag,
then `error("stop")`. Expected:

- the canvas bytes (paint, solvent, clock), the studio clock and the
  brushes' and rags' state are as before;
- `next.lua` in both sessions gives the same canvas, brushes and rags.

Leftover per-stroke limits would change the next strokes and fail this.

### 19. No solvent color

`c19_the_same_paint_with_more_or_less_solvent_looks_the_same`.

The same paint is copied with all, none and half of its solvent (through
the save). Expected: `seen()` is the same bits.

### The rag study

`thinner_tests::rag_study` (slow list).

Eight 256 px panels, each before and after:

- a dry cloth wipe, a blot and a damp cloth wipe, on wet paint;
- the same three on thinned paint;
- a dry-paint control (60 days);
- a brush stroke over a wiped area.

With `THINNER_RAG_STUDY=<path.png>` it writes the sheet. It prints, per
panel: paint removed, tone left, change outside, the elongation of the
lifted patch, paint and luminance. Expected:

- **Wipes** (dry and damp, on wet and on thinned paint):
  - remove ≥ 25% of the paint along the wipe's middle;
  - change the paint well outside the cloth by ≤ 1%;
  - leave < 90% of the tone (the ground shows).
- **Spirits** lift more than a dry cloth, on both paints.
- **Shape:** a wipe's lift has elongation ≥ 2.5 and a blot's ≤ 1.6, and a
  blot lifts something.
- **Dry paint** has no open paint, and its luminance changes by ≤ 1e-6.
- **A stroke over the wiped area** adds ≥ 5 µm on average where it
  crosses the wipe.

Why these limits:

- 25% and 90%: the rag's own notes say one pass lifts about half of a
  thin fresh film (`rag.rs` `LIFT`).
- 2.5 and 1.6: the wipe path is 600 units long and the pad about
  130 units across; the blot is a lumpy disc.

The earlier report's 14% stain is printed as "tone left" for the damp
wipe. It isn't asserted: it is an open limitation, not a pass. The picture
needs the reviewer's eyes as well.

## Open questions for review

1. **Which old logs are "incompatible" (check 3)?** Engines 1 and 2 replay
   bit for bit today (existing golden tests), and the thinner is engine 3
   only, so these tests treat engine-1/2 logs as compatible. They refuse
   only engine-3 PAINTCK8 saves, naming af49348. If the user meant that
   the new program should refuse engine-1/2 logs and send them to
   7b80cb0, check 3 needs another test, and the existing replay tests
   would change.
2. **The save layout rule** (solvent as the last 4 × pixels bytes) is
   imposed by the tests. Acceptable?
3. **Extra pickup from solvent-wet paint:** proposed omitted, so there is
   no test.
4. **Open-time leveling:** checks 10 and 16 require solvent-wet paint to
   level measurably within one τ, and to change thickness by >1% at 50+
   pixels within one minute. That is a modeling commitment ("solvent
   makes wet paint flow more"); its size is an estimate.
5. **Limits chosen without a run.** No heavy-job turn was available. The
   unthinned controls (checks 4, 8, 14) hold today's code to the same
   limits; if a control fails, the limit is wrong and goes back to review.
6. **Check 1 measures open paint** after the solvent has gone, as the
   plan says, not dried paint.

## File hashes (sha256)

Recorded at the tests commit; see `notes/thinner/TESTS_PHASE_REPORT.md`
for the commit.

| sha256 | file |
|---|---|
| `416736d1dbc5e2e8c907922f1127565d1cfa781b39d09ade1e506cd3778ee5f0` | `crates/paint/tests/thinner_physics.rs` |
| `8a2477dd336aa76918b8ed617caf8fe8bd2ef48dcb1778117696807d12941a7e` | `crates/paint/tests/thinner_support/mod.rs` |
| `9c45436c95853bff24146e24f9985fbb9f66d7ac79f7fbbb64ac632a90e3c3f1` | `crates/easel/src/thinner_tests.rs` |
| `a75ef7bdac6a359e7d0aee3524130410e39851e72a2495041f31f8009ea25beb` | `crates/easel/src/thinner_measure.rs` |
| `c3c9c40a29b19a5c4a2e42b6dd85871d1429160abea221000460d95f1caa73d9` | `crates/easel/tests/thinner/canvas.lua` |
| `31e301b3f309913d55147c7c2a626bf3a8bb8ca57c8859b2884403e68d30919d` | `crates/easel/tests/thinner/thinned.lua` |
| `83a9244e38ef0eca10903a4d7a4761ac3f8dd6861d564526cfe6c2f8c9325259` | `crates/easel/tests/thinner/next.lua` |
| `e140d15e4e0d93f9e522aa85a10cece5dcbe66ba8d4d16ea9bd6ddc4649b5527` | `crates/easel/tests/thinner/failing.lua` |
| `55ad4b849994870ef8c14a46e1c655bd60b32c4860e7d96916437331c5e89623` | `crates/easel/tests/thinner/determinism.lua` |
| `2d8c04f031a0d3fec77a69ba50a22f2b7584123363617ffc193d3dc03c35071f` | `scripts/test_thinner_acceptance` |

`crates/easel/src/main.rs` gains only the four test-only lines shown in the tests commit's diff (it changes for other reasons, so its hash isn't recorded).
