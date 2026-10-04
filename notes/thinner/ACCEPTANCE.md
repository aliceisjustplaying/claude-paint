# Thinner acceptance checks

The 19 thinner checks and the rag study from the overnight plan
(`~/src/a/claude-paint-overnight-plan.md`, sha256
`382fc2debb7a56505f9b4723c6591f22644feccb330fef62dc723b635f098e30`), as
named tests, with commands, expected results and error limits. Written
before the thinner exists, for review. After approval these files are
frozen: the program changes to pass them, not the other way round.

Branch `thinner2`: e39da64 (af49348 merged into the phase-1 diagnosis
c692eae), then the before-change baseline a97c3a6 merged as 6cf9384. Round
1 of review (2171f3c) asked for changes R1-R15
(`~/src/a/claude-paint-reviews/thinner-tests-required-changes-round1.md`);
this is round 2. The tests commit is named in
`notes/thinner/TESTS_PHASE_REPORT.md`; the file hashes are at the end.

## How to run

All in release builds (the baseline was made in release, and the iter
build can change the last digits). One heavy job at a time, through the
approved lock, with these limits:

| command | runs | limit |
|---|---|---|
| `~/src/a/claude-paint-tools/lockrun --timeout 60 --owner <you> -- scripts/test_thinner_acceptance --quick` | check 2, check 13 (a), check 13 (b) as an expected failure, and every test that isn't slow | 60 s once built |
| `~/src/a/claude-paint-tools/lockrun --timeout 300 --owner <you> -- scripts/test_thinner_acceptance --card` | check 1, and check 13 (b) as an expected failure | 300 s |
| `~/src/a/claude-paint-tools/lockrun --timeout 600 --owner <you> -- scripts/test_thinner_acceptance --all` | everything: the slow tests, the rag study's sheet and the card | 600 s (also a first build) |

**What counts.** A test passes only if cargo exited 0 and printed
`test NAME ... ok` for it. Each of these fails the run:

- a test that is missing, filtered out, ignored or failed;
- a cargo run that exits nonzero.

So a skipped test never counts as passed, and a run that matches nothing
fails. The slow tests are `#[ignore = "slow"]`, so an ordinary
`cargo test` doesn't run them; the script runs them by exact name with
`--ignored`. They are check 1, the check 8 card sweep, the check 10 gel
and touch-dry comparison and the rag study. `--all` writes the rag
study's sheet to `notes/thinner/rag_study.png` and fails if the file isn't
written.

**Check 13 (b)** fails on unchanged code (below) and awaits the user.
Every mode runs it and expects it to fail. It is reported on its own line
as `EXPECTED FAIL (pre-existing; user decision, ACCEPTANCE.md check 13)`.
If it passes or doesn't run, the run fails.

**Exit codes:**

- 1: anything else failed;
- 3: every other required test passed and only 13 (b)'s expected failure
  remains, printed as `NOT ALL GREEN`;
- 2: a usage error.

While 13 (b) stands, 0 is never returned.

**Dependencies:** cargo (rustc 1.97.1 here), bash 5 (the baseline's
`run_scenes.sh` reads `EPOCHREALTIME`), python3 with only the standard
library (the baseline's `state_compare.py`, and `scripts/thinner_dump_fields.py`)
and xz (`old_files/paintck8_save_stroke.ckpt.xz`). Nothing comes from the
speed branch beyond what the baseline commit brought; no inner lock is
taken, so the outer lockrun can't deadlock.

**The runner's own test:** `scripts/tests/thinner_acceptance_runner.sh`.
It uses a fake cargo and a stub check 2 and builds nothing, about 1 s.
These cases must exit 1:

- ok lines from a cargo that exits nonzero;
- a name missing, ignored or failed;
- empty output;
- 13 (b) passing or not running;
- check 2 failing;
- no rag sheet.

Everything passing with 13 (b) failing must exit 3 in all three modes,
and a usage error exits 2. It passes all 15 cases.

Single tests, exactly:

- check 2: `cargo build --release -p easel && scripts/thinner_check2 target/release/easel`
- check 13: `cargo test --release -p paint --test thinner_pigments -- --exact <name>`
- paint: `cargo test --release -p paint --test thinner_physics -- --exact <name>` (slow: add `--ignored`)
- easel: `cargo test --release -p easel --bin easel -- --exact thinner_tests::<name>` (slow: add `--ignored`)

## Files

| file | what |
|---|---|
| `crates/paint/tests/thinner_physics.rs` | checks 4, 5, 6, 8 (small thinner), 9, 10, 14, 15, 16, 17, 19 against the paint engine's public API |
| `crates/paint/tests/thinner_support/mod.rs` | their measuring helpers, and the list of the interface they need |
| `crates/paint/tests/thinner_pigments.rs` | check 13, on af49348's API only (it runs today) |
| `crates/paint/tests/thinner/tubes_af49348.txt` | check 13's answer: the tube table printed by unchanged af49348 |
| `crates/easel/src/thinner_tests.rs` | checks 1, 3, 7, 8 (the card sweep), 11, 12, 18 and the rag study, at the easel (Lua, sessions, saves, logs) |
| `crates/easel/src/thinner_measure.rs` | their helpers: the card, sums, session saves, af49348's old files, pictures |
| `crates/easel/tests/thinner/*.lua` | tiny inputs: a canvas, thinned strokes, a rag loaded before a failure (`cloth.lua`), a next chunk, a failing chunk, a chunk reusing the rag (`cloth_again.lua`), a four-chunk log |
| `crates/easel/src/main.rs` | four test-only lines that compile the easel modules (`#[cfg(all(test, feature = "replay"))]`) |
| `scripts/thinner_check2` | check 2, on the baseline's own tools |
| `scripts/thinner_dump_fields.py` | check 2's field half: the dumps declare the solvent |
| `scripts/test_thinner_acceptance` | the commands above |
| `scripts/tests/thinner_acceptance_runner.sh` | the runner's self-test |

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
- `paint::thinner::evaporation_tau_min(paint_um) -> f64`: the evaporation time (minutes) of the solvent at a pixel holding `paint_um` of paint (solvent-free). The pixel's own paint, not a neighborhood's; positive, finite, never shorter for more paint.
- Evaporation over one minute leaves `exp(-1 / τ(h))` of a pixel's solvent at fixed paint h (the plan's law).
- **PAINTCK9, a deliberate format contract.** The engine-3 save starts `PAINTCK9`, keeps every byte of the PAINTCK8 layout after the magic and ends with the solvent: one little-endian f32 (µm) per buffer pixel, row major, as the file's last 4 × pixels bytes. Anything else new goes between the two. It is part of the format, documented with it (checkpoint.rs), not a test seam.
  - The tests use it to make "the same paint with a different amount of solvent" through the save, without a setter only tests would use (checks 10, 15, 17, 19).
  - `with_solvent_scaled` checks that the copy saves back to exactly the same bytes outside that block: pigment, stiffness, cure, clock, everything. It also checks that the block holds the scaled solvent.
- **State dumper.** `easel run --dump-state` (`crates/{paint,easel}/src/state_dump.rs`, part of the protected baseline) gains the solvent under new names, and no existing field is renamed or redefined:
  - the canvas field `wet.solvent`, f32, shape `[h, w, 1]` like `wet.vol`;
  - `solvent` in each bristle's `Debug` (`brushes[i].bristles[j].solvent`);
  - `solvent_mm3` in each rag's (`rags[i].solvent_mm3`).

  Any new clock state also gets a new field.
- Lua, engine 3: `pile{..., thinner = t}`, `p.thinner` (0.0 when not given), and `print(p)` showing `, thinner t` after the medium only when `t > 0`. Engines 1 and 2: the key is an error, `p.thinner` is nil and `print(p)` is unchanged.
- `save::read` refuses a PAINTCK8 save of an engine-3 canvas from its header, naming af49348; `Canvas::read_state` refuses a PAINTCK8 checkpoint of an engine-3 canvas, naming af49348.

## The checks

"Rounding" below means float error, not tolerance for different behavior.
Every limit was chosen before any thinner code exists, so none was fitted
to a run. Where a limit could be wrong for today's code, the test carries
an unthinned control on today's code path that must pass the same limit.
The leveling sizes (checks 10 and 16) are labeled model estimates, not
measurements.

### 1. The card (locked target)

`thinner_tests::c01_the_card_raw_sienna_thinned_half_keeps_half_the_contrast_at_2400px`, `#[ignore = "slow"]`, `--card`.

**Setup.** Raw sienna, `thinner=0.5`, one `body` pass (`clip=true`) at load
0.3 and one at 0.6, in separate strips over the card at 2400 px. The card
is the Inness toned ground with a bone-black band and a lead-white band
dried 60 days. After the passes the test waits `ceil(10 τ) + 1` minutes,
with τ the evaporation time of the thickest paint on the card.

**Diagnostic guards.** In each strip the card's contrast before the pass
is finite and positive, and the pass laid paint and solvent.

**Expected, in this order:**

- The wait is at least 10 τ, for τ both after the passes and at the
  measurement.
- The solvent left in each strip is under 0.001 × what was there right
  after the passes. The plan says "of the added solvent". Some solvent
  evaporates during the pass, so what is there afterward is no more than
  what was added, and this test is the stricter one.
- The same holds for all the solvent on the canvas, so solvent that
  spread out of a strip can't pass for evaporated.
- Each strip shows ≥ 50% of the card's black/white contrast: the
  luminance of the white band's mean color minus the black band's, after
  the pass over before it.

Measured on the open paint once the solvent has gone, not after drying.
The target is as the user locked it.

### 2. No thinner

`scripts/thinner_check2 <release easel>`, which `--quick` and `--all` run
after `cargo build --release -p easel`.

**The answers** are the before-change baseline, `notes/thinner/baseline/`
(commit a97c3a6). The speed agent made it from unchanged af49348 in a
release build, and the lead checked it independently. Its README gives
the format. Nothing in it is edited.

Three parts, all required:

1. `notes/thinner/baseline/tools/compare_build.sh <easel> --added-zero`.
   It replays the six scenes (stroke, body, overlap, pickup, rag, wait;
   128×64 px, 27 chunks). It compares the full state after every chunk
   bit for bit: canvas per pixel, clock, brushes' bristles, rags, studio.
   Every field the new engine adds must be zero. Expected: it exits 0 and
   all six PNGs are af49348's.
2. The same six scenes with `thinner=0` written into every `pile{`. The
   same state comparison (`state_compare.py --added-zero`) and the same
   PNGs.
3. `scripts/thinner_dump_fields.py` on the dumps of both runs:
   - every chunk has `wet.solvent` (f32, the shape of `wet.vol`);
   - every bristle has `solvent`;
   - every rag has `solvent_mm3`;
   - at least one chunk holds a brush and one a rag.

   Without this, an unchanged dumper could leave the solvent out and pass
   `--added-zero` with nothing to hold to zero. The baseline's own dumps
   fail this part (no `wet.solvent` in 12 chunks of `pickup` and `rag`).

Not criteria, as the baseline's README says: the `canvas=` digest (PAINTCK9
changes it by itself) and the printed output (reported).

### 3. Old files

Inputs: the baseline's `old_files/`, made by unchanged af49348, and tiny
logs.

- `c03_af49348_paintck8_saves_are_refused_naming_the_old_version`. The
  inputs are the genuine PAINTCK8 save of an engine-3 canvas
  (`paintck8_save_stroke.ckpt.xz`, through `xz -dc`) and its first 280
  bytes alone (`paintck8_save_header.bin`). Expected:
  - `save::read` refuses both with an error that contains `af49348`, not
    the "failed to fill whole buffer" of running out of file, so the
    refusal comes from the header, before anything is painted;
  - `Canvas::read_state` refuses the save's checkpoint, naming `af49348`.
- `c03_af49348_engine_3_logs_still_replay`. The two engine-3 logs that
  af49348's live session wrote, with and without a box line, replay every
  chunk as engine 3. They are compatible: with no thinner, engine 3 paints
  as before (check 2).
- `c03_a_log_keeps_its_engine`. Tiny logs with no engine line, `--@ engine 2`
  and `--@ engine 3` (a 64 px canvas each) replay as engines 1, 2 and 3,
  both the studio and the canvas. Old logs aren't replayed as engine 3.
- `c03_engines_1_and_2_have_no_thinner_and_engine_3_has`.
  - On engines 1 and 2, `thinner=` is an error, and a pile prints
    `nil	pile(raw sienna 1; medium 0)`.
  - On engine 3 a pile prints `0.0	pile(raw sienna 1; medium 0)` without
    it and `0.5	pile(raw sienna 1; medium 0, thinner 0.5)` with it.
  - 0.95, -0.1 and a string are errors.

Policy (lead-approved, flagged to the user): engine-1/2 logs are compatible
and keep their engine; engine-3 PAINTCK8 saves are refused, naming
af49348. The real engine-1/2 painting logs aren't replayed here.

### 4. Nothing vanishes

- `c04_a_stroke_and_a_wipe_account_for_all_paint_and_solvent`.
  - **Setup.** A thinned underlayer, then four strokes across it: raw
    sienna thinned 0.5, lead white thinned 0.2 and 0.9, and unthinned raw
    sienna. A stroke puts no time on the clock (asserted), so nothing
    evaporates during it.
  - **Expected, per stroke:** canvas + brush paint is the same before and
    after, and so is canvas + brush solvent. The unthinned brush comes
    away carrying solvent: the pickup moved solvent with the paint.
  - **The rag**, dry and then dipped in spirits. Expected: canvas paint
    before = after + what `rag_wipe` returned, and canvas solvent before
    = after + `Rag::solvent_mm3`.
- `c04_control_a_wholly_unthinned_scene_balances`. The control: no thinner
  anywhere, so `with_thinner` is never called and this is today's path.
  The same kinds of stroke over wet paint and the same wipe must balance
  paint within the same limit, with no solvent at all.

**Rounding: 1e-4 of the total counted** (`BALANCE_REL`), lead-approved.
The totals are f32 per-pixel amounts summed in f64. A stroke changes
10^4-10^5 pixels, each with a relative error of about 6e-8. A random walk
would give about 2e-5 of the total; 1e-4 allows five times that. This is a
heuristic, not a bound, and the control is what tests it on today's code.
Real loss is larger: a single bristle's share is about 1/120 of the
brush.

### 5. The brush runs out

`c05_an_emptying_brush_lays_less_and_a_fuller_load_lasts_farther`.

**Setup.** A filbert 8, raw sienna thinned 0.5, one 960-unit stroke at
load 0.3 and one at 0.6. Paint is measured along the stroke's middle in
10-unit bins. θ = `0.25 × stroke_limit_um(0.5) × 0.5`.

**Expected:**

- the paint starts above θ;
- the last 100 units average under half the start;
- the paint falls below θ before the stroke's end;
- at load 0.6 it stays above θ at least 30 units farther than at 0.3.

θ is an absolute level, so the test holds whether or not the limit binds
at load 0.3.

### 6. Pressure

`c06_more_pressure_lays_more_paint_up_to_the_stroke_limit`.

The same thinned load at pressure 0.3, 0.6 and 0.9. Expected:

- each step lays at least 5% more paint, measured off the brush;
- no pixel gains more wet film than `stroke_limit_um(0.5)`, allowing for
  rounding of 1e-3 relative + 1e-3 µm.

5% sits well below the mark-width growth between steps, about 30%.

### 7. Overlap

`thinner_tests::c07_two_overlapping_thinned_passes_leave_more_paint_than_one`.

Two thinned `body` passes over the same rectangle. Expected: at least 20%
more paint after the second than after the first. Each stroke gets a new
limit, and a pass's pickup takes much less than it lays.

### 8. Smooth changes

- `c08_a_hundredth_of_thinner_is_a_small_change` (paint). The same five
  strokes with no thinner, with thinner 0 and with thinner 0.01.
  Expected:
  - thinner 0 gives the same save bytes as no thinner;
  - at 0.01, total paint is within 3%;
  - the mean per-pixel difference is within 3% of the mean film;
  - solvent is above 0 and at most 2% of the paint.

  The 3%: a hundredth of the load is solvent, and dips and pickups may
  triple that, no more.
- `thinner_tests::c08_more_thinner_never_hides_the_card_more`, slow.
  Raw sienna at loads 0.3 and 0.6 and thinner 0, 0.1, …, 0.9. Each setting
  gets its own card at 480 px, with the strip in the same place and the
  same seed, so only the thinner changes. Each is measured after its
  solvent has gone, with guards for paint laid and contrast present.
  Expected:
  - the contrast showing never falls from one step to the next by more
    than 0.005;
  - 0.9 shows more than 0.

  The 0.005 is a measurement allowance the plan doesn't grant in words
  ("must not make the card less visible"). The lead approved it and it is
  flagged to the user.

### 9. Evaporation

`c09_the_solvent_evaporates_and_the_film_loses_its_volume`. The
exponential law is tested directly, with the expected values computed in
the test.

**Setup.** Two films on a flat ground: one thinned pass, and five passes
(asserted thicker: median ≥ 1.5×). Every minute, for each pixel that
qualifies:

- its paint changed by at most 1e-3 of itself during the minute;
- it holds ≥ 1 µm of paint;
- it holds ≥ 0.01 µm of solvent.

**Expected:**

- The share of its solvent left equals `exp(-1 / τ(h))` within
  1e-4 + 2δ, where δ is the paint's relative change: liquid moving with
  δ of the paint can move about that much solvent again. The same share
  every minute is the "equal intervals, equal fractions" rule.
- Each film has at least 200 such pixel-minutes.
- The thick film loses a smaller share per minute than the thin one.
- τ is positive and finite and never shorter for more paint (0.5 to 200 µm).
- Total solvent falls every minute.
- Paint balances within 1e-4, so the wet film loses exactly the
  solvent's volume.
- After ten times the slowest τ, under 0.001 of the solvent is left.

Immediate loss, linear loss and a wrong rate fail the per-pixel law.
Check 19 owns the "no solvent color" contract.

### 10. Wet handling and drying

- `c10_a_solvent_wet_film_spreads_more_than_after_the_solvent_has_gone`.
  - **Setup.** Two copies of one thinned patch on a flat ground: the same
    paint geometry and cure, one with its solvent and one without (taken
    out through the save). Both wait the same span, one τ of the median
    film.
  - **Expected:** the paint surface's roughness (std of relief + paint)
    falls by more than 2% in the copy with solvent, and by more than twice
    what it falls in the copy without. Paint balances in both.
  - 2% is a model estimate.
- `c10_thinned_and_unthinned_paint_of_equal_thickness_gel_and_dry_together`, slow.
  - **Setup.** Film A is a thinned patch. Film B is A with its solvent
    taken out through the save: the same paint in the same places, at
    equal remaining thickness. Every pixel of film is checked every
    30 minutes for 20 days.
  - **Limit:** the median gel time and the median touch-dry time agree
    within 3% or 30 min, whichever is longer. Lead-approved.
  - Why: 30 min is the check interval, and 3% allows solvent-driven
    leveling to move local thickness by a few percent (drying time ∝
    thickness^0.7).
- Extra pickup strength from solvent is omitted in this version
  (lead-approved), so it has no test. The guide and report say so.

### 11. Save and reopen

`thinner_tests::c11_a_save_mid_evaporation_reopens_to_the_same_state_and_goes_on_the_same`.

**Inputs:** `canvas.lua`, `thinned.lua`, then `wait(m)`, with m = τ(median
film) × ln 2, rounded, + 0.37 min. Asserted before saving: 10-90% of the
solvent is left, and the clock is between whole minutes.

**"Save"** means the session's whole save (`session_save`, through
`save::write`): the canvas, seed, chunk and call counters, studio clocks,
piles, setup, style, box and engine. A save holds no Lua globals and no
held brushes or rags, by design (save.rs:7-12), so the next chunk makes
its own. The header's `chunks=` counts the session's own log (save.rs:47),
which a reopened session restarts. It is written as `chunks_before` + the
log in both sessions; nothing else is changed.

**Expected:**

- the reopened session saves exactly as the one saved;
- after `next.lua` (a thinned stroke, a wait, a rag) in both, the saves
  are identical;
- a replay of the whole log from scratch saves the same.

### 12. Repeatable results

`thinner_tests::c12_the_same_log_saves_the_same_bytes_on_one_thread_and_four`.

`determinism.lua` is four chunks: thinned strokes and passes, waits of
3.3 and 1 min, and a damp rag's wipe and blot. It is replayed in a
one-thread and a four-thread rayon pool. Expected: the whole session
saves are identical (`session_save`), with solvent on the canvas at the
end.

### 13. Pigment ordering

`crates/paint/tests/thinner_pigments.rs`. It uses only af49348's API, so it
runs on today's code.

**(a) `c13_every_pigment_value_is_af49348s`.** The tube table must equal
`crates/paint/tests/thinner/tubes_af49348.txt` byte for byte:

- the catalog: each tube's name, pigment, masstone, hiding, stiffness,
  strength and both drying rates, floats in shortest exact form;
- every box, with its tubes and engine.

The file is the output of the ignored `print_tube_table` test. It was run
on a detached worktree of af49348 (`af49348239421791509f7b7c36e9dc39305b26fe`)
with only this test file added:
`TUBE_TABLE_OUT=<file> cargo test --release -p paint --test thinner_pigments -- --exact --ignored print_tube_table`
(rustc 1.97.1). The reviewer cross-checked it against
`git show af49348:crates/paint/src/palette.rs`: 48 tubes and 6 boxes
match. **Passes today.**

**(b) `c13_burnt_sienna_shows_the_card_at_least_as_well_as_raw_sienna`.**
Measured the card's way:

- a bone-black band and a lead-white band, laid thick and dried;
- the same 20 filbert strokes of unthinned raw sienna, then of burnt
  sienna (Sargent box, load 0.5), across both, with the films equal pixel
  for pixel.

Expected: burnt sienna lets at least as much of the black/white contrast
show as raw sienna. Field/Salter 1869 §155 calls burnt "more transparent
than the raw earth".

**PRE-EXISTING FINDING, for the user's decision: (b) fails on unchanged
code** (paint sources equal af49348's, per (a); measured 2026-10-04):

| | hiding (one coat's contrast ratio) | scattering per coat | masstone luminance | card contrast showing, equal 10.07 µm film |
|---|---|---|---|---|
| raw sienna | 0.40 | 0.296 | 0.174 | **17.91%** |
| burnt sienna | 0.45 | 0.199 | 0.080 | **9.85%** |

Burnt sienna scatters less, but its darker masstone absorbs much more, so
on the card it hides more than raw sienna. The tube table agrees: hiding
0.45 against 0.40. The test body stays as written. The runner reports it
as an expected failure and exits 3, never 0, while it stands. Pigments
are not changed.

### 14. Stroke points

`c14_more_points_on_the_same_path_lay_the_same_paint_and_one_stroke_shares_one_limit`.

**Setup.** The same straight path given as 5, 10 and 50 points.

**Expected:**

- the stroke laid paint, and solvent exactly when thinned;
- paint, solvent and their sum each agree at every pixel within 2% of
  the stroke limit (thinned) or of the thickest film (the unthinned
  control);
- total paint and total solvent each agree within 0.5%.

**Why 2%:** `drag_on` (bristle.rs) resamples every path to its own step,
so extra collinear points change only float rounding of arc length. A
limit that reset per point or per segment would multiply the film by the
number of points added, at least 2×.

**One stroke shares one limit.** A brush of 400 overlapping hairs
(filbert 20, `hair` 2.5), and one stroke that goes over its own path four
times, each add no more than `stroke_limit_um(0.5)` to any pixel
(rounding 1e-3 relative + 1e-3 µm) and lay something.

### 15. Drying inputs

`c15_solvent_does_not_slow_the_oil_cure`.

**Setup.** A film of five thinned passes, measured where it holds ≥ 12 µm
of paint and some solvent (at least 100 pixels). Below 0.37 coats
(9.3 µm), `drying::rate` clamps its thickness factor (drying.rs:251), so
it can't tell inputs apart; 12 µm stays above that. The same film, with
and without its solvent (through the save), waits one τ while most of
the solvent is still there.

**Expected:** the 10th, 50th and 90th percentiles of cure with solvent ÷
cure without are all within 2% of 1. The copies start with identical cure
and thickness; only solvent-driven leveling can move them, a little.

**Why wrong inputs fail:**

- If the thickness counted the solvent (thinner 0.5 roughly doubles it),
  the rate would fall by 2^0.7 ≈ 1.6× while the solvent is there.
- If solvent-softened stiffness went in, the `FAT` term adds up to 1.6×
  (stiffness 0.5 → 0.2 alone is +14%).

Over one τ (about 63% of the solvent still present on average) either is
well beyond 2%.

### 16. Moving paint

`c16_brush_rag_and_spreading_carry_solvent_in_the_local_ratio`.

**Setup.** Two fresh films sit side by side: raw sienna thinned 0.5
(solvent ÷ paint = 1) and lead white thinned 0.2 (0.25). Every pixel holds
its film's ratio within 1e-3.

**Expected:**

1. **Brush.** A clean brush through the first film carries ratio 1
   within 1e-3.
2. **Rag.** A rag over each film takes that film's ratio within 1e-3, and
   leaves every pixel's ratio within 1e-4 of what it was.
3. **Spreading.** On a flat ground, a thick film of ratio 1 (four
   thinned-0.5 passes) sits beside a thin film of ratio 1/9 (one light
   pass of thinner 0.1). After one minute:
   - **Balance.** The solvent left equals what the law leaves of the
     solvent before, computed per pixel, within 1e-4: either
     Σ s0·exp(-1/τ(h0)) (evaporation first) or Σ s1·exp(+1/τ(h1)) = S0
     (spreading first). Paint balances within 1e-4.
   - **Local ratios.** Every pixel with ≥ 1 µm of paint (at least 1000
     of them) ends with a ratio inside its 4 px neighborhood's ratios
     before, times that neighborhood's evaporation factors at its
     thicknesses before and after (1e-4). Spreading mixes liquids; it
     doesn't separate paint from solvent. A uniform multiplier on the
     solvent fails this.
   - **Redistribution across the two ratios.** At least 20 thin-film
     pixels beside the thick film gained ≥ 1% paint. Their mean ratio,
     with evaporation taken back, rose by more than 1%: they took in the
     thick film's liquid, solvent and all. Paint moving without its
     solvent would make it fall.

The movement guards (≥ 20 pixels gaining ≥ 1%) are model estimates.

### 17. Waits

`c17_a_wait_split_on_the_minute_grid_is_exact_and_off_it_within_1e4`.

The clock counts one-minute steps from the canvas's start. Two thinned
patches, starting on a whole minute. Expected:

- `wait(15)` and 15 × `wait(1)` give the same save, byte for byte, with
  solvent present throughout.
- With a stroke between waits on whole minutes (wait 3, stroke, wait 12,
  against 3 × 1, stroke, 12 × 1), the same save, byte for byte.
- `wait(7.3)` then `wait(7.7)` (the same clock: 7.3f32 + 7.7f32 = 15
  exactly) agree with `wait(15)` at every pixel within 1e-4 relative in
  paint µm, solvent µm and cure.
- From a fractional start (`wait(0.4)` first), `wait(15)` and 15 ×
  `wait(1)` agree within the same 1e-4: each crosses the minute grid at
  different points, so exact equality isn't required off the grid.

**Zero rule:** two values agree if they are equal, or both are no
further from zero than a floor, or they are within 1e-4 of the larger in
magnitude. The floors:

- paint and solvent: 1e-5 coats = 2.5e-4 µm (the engine's bare-canvas
  threshold);
- cure: 1.5e-7, a millionth of the way to the gel point.

### 18. Undo after failure

`thinner_tests::c18_a_failed_chunk_after_thinned_paint_takes_everything_back`.

**Setup.** Two sessions run `canvas.lua`, `thinned.lua` and `cloth.lua`.
`cloth.lua` is a rag the painter keeps, dipped and wiped through the
thinned strokes; asserted: it holds paint (`load` > 0) and solvent
(`solvent_mm3` > 0) and is damp. One session then runs `failing.lua`:

- a thinned stroke;
- a thinned pass;
- `wait(1.5)`;
- a dip, wipe, refold and wipe of the same rag;
- `error("stop")`.

**Expected:**

- the canvas bytes (paint, solvent, clock), the studio clock and the
  brushes' and the rag's state are as before;
- `next.lua` and then `cloth_again.lua` (the same rag and brush, reused)
  leave the same canvas, brushes and rag in both sessions.

Leftover per-stroke limits would change the next strokes and fail this.

### 19. No solvent color

`c19_the_same_paint_with_more_or_less_solvent_looks_the_same`.

The same paint, with all, none and half of its solvent (through the save).
Expected: `seen()` gives the same bits.

### The rag study

`thinner_tests::rag_study`, slow. `--all` runs it with `THINNER_RAG_STUDY`
set to `notes/thinner/rag_study.png` and fails if the sheet isn't written.

**Panels.** Eight, 256 px each, before and after:

- a dry cloth wipe, a blot and a damp cloth wipe, on wet paint;
- the same three on thinned paint;
- a dry-paint control (60 days);
- a brush stroke over a wiped area.

Printed per panel: paint removed, tone left, change outside the cloth,
the lifted patch's size, spread and elongation, paint and luminance.

**Expected:**

- **Wipes:** remove ≥ 25% of the paint along the wipe's middle; change
  the paint well outside the cloth by ≤ 1%; leave < 90% of the tone (the
  ground shows).
- **Spirits:** lift more than a dry cloth, on both paints.
- **Shape**, judged only once the lifted patch has ≥ 100 pixels and a
  spread of ≥ 4 units² both ways: a wipe's elongation is ≥ 2.5 and a
  blot's ≤ 1.6.
- **Dry control:** the swatch is there before the wipe (its band at most
  80% of the ground's luminance); it has no open paint; its luminance
  changes by ≤ 1e-6.
- **A stroke over the wiped area:** adds ≥ 5 µm on average where it
  crosses the wipe.

**Why these limits:**

- 25% and 90%: the rag's notes say one pass lifts about half of a thin
  fresh film.
- 2.5 and 1.6: a 600-unit wipe from a pad about 130 units across, against
  a lumpy disc.

The earlier report's 14% stain is printed as "tone left" for the damp
wipe, not asserted: it is an open limitation. The picture's look is
approved by the reviewers and the user, separately; the test doesn't
claim it.

## Decisions and open questions

Settled in round 1 (lead and reviewer):

- Extra solvent pickup is omitted. The guide and report say so.
- Check 3's policy: engine-1/2 logs are compatible and keep their engine;
  engine-3 PAINTCK8 saves are refused, naming af49348. Flagged to the user.
- The leveling and movement sizes in checks 10 and 16 are model estimates.
- Check 8's 0.005 allowance. Flagged to the user.
- The PAINTCK9 layout is a deliberate format contract (above).

For the user:

1. **Check 13 (b) fails today.** The sienna order on the card is the
   reverse of Field/Salter's. Is that a defect for a later pigment fix
   (not tonight), or should the check be restated?
2. Check 3's policy and check 8's allowance (above).

Risks the builder may hit; a fix goes back to review, never into a weaker
test:

- Check 9 needs at least 200 pixel-minutes per film where spreading moved
  the paint by ≤ 1e-3 in the minute. If the model's leveling moves every
  pixel more than that, the fixture needs review.
- Check 16 needs spreading that is visible within one minute (≥ 20 pixels
  gaining ≥ 1%).

## File hashes (sha256)

| sha256 | file |
|---|---|
| `fa1bf2299b9307ceb0d74209c8a27772d75af93deb9681af240687db98612db5` | `crates/paint/tests/thinner_physics.rs` |
| `fe7d4e25e0e03a094d3c534dc2f8728c6ef4482e29f38a5b0b3ce01042d728ff` | `crates/paint/tests/thinner_support/mod.rs` |
| `53924605cde088b16056fd31d69d57f96c5eae8d5c4674858326823bb0ca06cc` | `crates/paint/tests/thinner_pigments.rs` |
| `542e25ab395446ea79c893640e5515702309b145f99a5da5ff7e617ea83e15c2` | `crates/paint/tests/thinner/tubes_af49348.txt` |
| `77d987f32935818cca1468d0f93bcd2d5b59f8a99ee272609164e8390f69133c` | `crates/easel/src/thinner_tests.rs` |
| `f94b50a428ef2e05285f2f4f6c634662eff85d945d2c0ab0c1033c06fd7cefd1` | `crates/easel/src/thinner_measure.rs` |
| `c3c9c40a29b19a5c4a2e42b6dd85871d1429160abea221000460d95f1caa73d9` | `crates/easel/tests/thinner/canvas.lua` |
| `40674b768e1f79347749a09957d00fb74f57b207557aee2d919fe5f389e14796` | `crates/easel/tests/thinner/cloth_again.lua` |
| `436d362cffcb882ad8abad6e587e593ff833eb25e2a34abaf789fa4d07c4b084` | `crates/easel/tests/thinner/cloth.lua` |
| `55ad4b849994870ef8c14a46e1c655bd60b32c4860e7d96916437331c5e89623` | `crates/easel/tests/thinner/determinism.lua` |
| `fc674bf8a45732cf52e56976279e2ecfd24ca2bba9fcecd43d7f31d3ef137a96` | `crates/easel/tests/thinner/failing.lua` |
| `83a9244e38ef0eca10903a4d7a4761ac3f8dd6861d564526cfe6c2f8c9325259` | `crates/easel/tests/thinner/next.lua` |
| `31e301b3f309913d55147c7c2a626bf3a8bb8ca57c8859b2884403e68d30919d` | `crates/easel/tests/thinner/thinned.lua` |
| `447618bca8d6725db9497c304f0eaa2f8db20635dad5822b4a5b9ae914b8c0e9` | `scripts/test_thinner_acceptance` |
| `f3e2bc44ee07005ff92c445728c8efb4ea241d6f7ed58ba34c16addbf2a3ce4b` | `scripts/tests/thinner_acceptance_runner.sh` |
| `33000f5b75fd4cc175aab7a2cab8e19b7f450916f622326898e3076282811cff` | `scripts/thinner_check2` |
| `991de2d0a61a076a304cc036127ee3ed70ac80c13ef29a5cb2c421cbbeee0030` | `scripts/thinner_dump_fields.py` |

`crates/easel/src/main.rs` gains only four test-only lines. The baseline's own files are listed in `notes/thinner/baseline/SHA256SUMS` (commit a97c3a6), unchanged.
