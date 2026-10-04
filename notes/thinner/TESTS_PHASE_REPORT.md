# Thinner: test-writing phase report

2026-10-04, thinner agent. The plan's sha256 is
`382fc2debb7a56505f9b4723c6591f22644feccb330fef62dc723b635f098e30`. User
decisions: `claude-paint-overnight-decisions.md`.

## Commits on `thinner2` (not pushed)

| commit | what |
|---|---|
| e39da64 | af49348 merged into the phase-1 diagnosis (the lead's merge) |
| f45a184 | the first test draft (19 checks and the rag study) |
| 6cf9384 | the before-change baseline a97c3a6 merged (`--no-ff`; it brings lockrun and the state dumper). No conflicts |
| **2171f3c** | **the tests commit for review**: check 2 on the baseline's tools, check 3 on af49348's old files, check 13 (a) and (b). Hashes in `ACCEPTANCE.md` |

No thinner code exists. The tests, helpers, inputs, commands, limits and
hashes are in `notes/thinner/ACCEPTANCE.md`.

## What ran (all through `~/src/a/claude-paint-tools/lockrun`)

| job | result |
|---|---|
| The af49348 tube table (check 13's answer). A detached worktree of af49348 with only `thinner_pigments.rs` added, `print_tube_table`, release | done in 13.9 s. The worktree was removed afterward |
| Check 13 on this branch (paint sources equal af49348's) | (a) **passes**. (b) **fails**: raw sienna shows 17.91% of the card and burnt sienna 9.85% at an equal 10.07 µm film. A pre-existing finding (below) |
| Compiling `thinner_physics` and the easel test binary (`--no-run`) | **They don't compile, as expected**: 49 errors in paint and 7 in easel. Every one is missing thinner API: `paint::thinner`, `Paint::with_thinner`, `Canvas::{solvent_um, solvent_total, cure_at}`, `Held::carried`, `Rag::solvent_mm3`. The 3 E0689 errors (`ceil` on an ambiguous float) come from the unresolved `paint::thinner` import. Because the build stops at these errors, type errors behind them can only surface once the API exists |

Not run: check 2. It needs a build with the thinner, and on today's code it
would be the baseline's own self-check. Also not run: the script's real
runs. Its pass/skip/missing logic was checked with a fake `cargo` that
printed chosen results: all ok → exit 0; one skipped, one failed and one
missing → exit 1, each named.

## The tests

| check | test | file |
|---|---|---|
| 1 | `c01_the_card_raw_sienna_thinned_half_keeps_half_the_contrast_at_2400px` (`#[ignore = "slow"]`) | easel `thinner_tests` |
| 2 | `scripts/thinner_check2` (`compare_build.sh --added-zero` + PNGs; the same with `thinner=0`) | script |
| 3 | `c03_af49348_paintck8_saves_are_refused_naming_the_old_version`, `c03_af49348_engine_3_logs_still_replay`, `c03_engines_1_and_2_have_no_thinner_and_engine_3_has` | easel |
| 4 | `c04_a_stroke_and_a_wipe_account_for_all_paint_and_solvent` | paint `thinner_physics` |
| 5 | `c05_an_emptying_brush_lays_less_and_a_fuller_load_lasts_farther` | paint |
| 6 | `c06_more_pressure_lays_more_paint_up_to_the_stroke_limit` | paint |
| 7 | `c07_two_overlapping_thinned_passes_leave_more_paint_than_one` | easel |
| 8 | `c08_a_hundredth_of_thinner_is_a_small_change` (paint), `c08_more_thinner_never_hides_the_card_more` (easel, slow list) | both |
| 9 | `c09_the_solvent_evaporates_and_the_film_loses_its_volume` | paint |
| 10 | `c10_a_solvent_wet_film_spreads_more_than_after_the_solvent_has_gone`, `c10_thinned_and_unthinned_paint_of_equal_thickness_gel_and_dry_together` (slow list) | paint |
| 11 | `c11_a_save_mid_evaporation_reopens_to_the_same_state_and_goes_on_the_same` | easel |
| 12 | `c12_the_same_log_saves_the_same_bytes_on_one_thread_and_four` | easel |
| 13 | `c13_every_pigment_value_is_af49348s`, `c13_burnt_sienna_shows_the_card_at_least_as_well_as_raw_sienna` | paint `thinner_pigments` |
| 14 | `c14_more_points_on_the_same_path_lay_the_same_paint_and_one_stroke_shares_one_limit` | paint |
| 15 | `c15_solvent_does_not_slow_the_oil_cure` | paint |
| 16 | `c16_brush_rag_and_spreading_carry_solvent_in_the_local_ratio` | paint |
| 17 | `c17_a_wait_split_on_the_minute_grid_is_exact_and_off_it_within_1e4` | paint |
| 18 | `c18_a_failed_chunk_after_thinned_paint_takes_everything_back` | easel |
| 19 | `c19_the_same_paint_with_more_or_less_solvent_looks_the_same` | paint |
| rag | `rag_study` (slow list; `THINNER_RAG_STUDY=<png>` writes the sheet) | easel |

Commands: `scripts/test_thinner_acceptance --quick | --card | --all`.

## Needs the user's judgment

1. **Check 13 (b) fails on unchanged code.** At an equal film over the
   card, burnt sienna hides more than raw sienna (9.85% against 17.91% of
   the contrast showing). The tube table agrees: burnt sienna's hiding is
   0.45 and raw's 0.40. Burnt scatters less (0.199 against 0.296 per coat)
   but its darker masstone absorbs more. Field/Salter §155 has burnt the
   more transparent. The test is kept as an expected failure, and no
   pigment was changed.
2. **"Incompatible old logs" (check 3).** The tests treat engine-1/2 logs as
   compatible (they replay bit for bit today) and refuse only engine-3
   PAINTCK8 saves, naming af49348. If the user meant refusing engine-1/2
   logs too, check 3 needs another test.

## Open questions for the reviewers

These are also in `ACCEPTANCE.md`:

- The tests impose a save layout: the solvent is the last 4 × pixels bytes
  of the PAINTCK9 save.
- Extra pickup from solvent-wet paint is proposed as omitted, so there is
  no test.
- Checks 10 and 16 commit the model to visible leveling within one τ.
- No limit has been tried against a run. Unthinned controls (checks 4, 8
  and 14) hold today's code to the same limits.

## Left on disk

This branch's `target/`, about the size of one release build. Phase 3
needs it, and it gets deleted at the end. The scratch logs in `$TMPDIR`
are deleted.

## Round 2 (review round 1: APPROVE WITH REQUIRED CHANGES, R1-R15)

**Tests commit: 55ef93a.** It is on top of 5ce3989 (the round-1 report).
Required changes: `~/src/a/claude-paint-reviews/thinner-tests-required-changes-round1.md`.
`notes/thinner/ACCEPTANCE.md` has the new limits, dependencies, decisions
and hashes. The protected baseline and the dumper (`notes/thinner/baseline/`,
`crates/{paint,easel}/src/state_dump.rs`) are untouched since a97c3a6:
`git diff --quiet a97c3a6 HEAD -- …` is clean.

What ran, all through lockrun:

- **Compile, paint:** `thinner_physics` fails on 53 errors, every one
  missing thinner API: `paint::thinner`, `with_thinner`, `solvent_total`,
  `solvent_um`, `cure_at`, `carried`, `solvent_mm3`.
- **Compile, easel:** the test binary fails on 10 errors, of the same kind.
- **Check 13 (a)** still passes.
- **The runner's self-test**
  (`scripts/tests/thinner_acceptance_runner.sh`, a fake cargo, no build)
  passes all 15 cases.
- **`thinner_dump_fields.py`** rejects the baseline's own af49348 dumps,
  as it must: no `wet.solvent` in 12 chunks, and bristles and rags without
  solvent.

| R | change | where (at 55ef93a) |
|---|---|---|
| R1 | A test counts as passed only if cargo exited 0 as well as printing `ok` for it; a nonzero exit fails the run. Self-test of the runner: ok lines with a nonzero exit, missing, ignored, failed, empty output, and more | `scripts/test_thinner_acceptance:116`, `:121`; `scripts/tests/thinner_acceptance_runner.sh` |
| R2 | `#[ignore = "slow"]` on the check 8 card sweep, the rag study and the check 10 gel/dry comparison; `--all` runs them by exact name with `--ignored` | `crates/easel/src/thinner_tests.rs:146`, `:371`; `crates/paint/tests/thinner_physics.rs:277`; runner `:181`, `:184` |
| R3 | 13 (b) runs in every mode as an expected failure, on its own line as `EXPECTED FAIL (pre-existing; user decision, ACCEPTANCE.md check 13)`; passing or not running fails the run; exit 3 = `NOT ALL GREEN`, never 0 while it stands. The test body is unchanged | runner `:133`, `:204` |
| R4 | Check 9 tests the law: per pixel and per minute at fixed paint thickness, the share of solvent left = `exp(-1/τ(h))` within 1e-4 + 2δ; two films, the thick one asserted ≥ 1.5× thicker and asserted to lose a smaller share; ≥ 200 pixel-minutes each | `thinner_physics.rs:189`, law at `:222` |
| R5 | Check 16 spreading: a thick ratio-1 film beside a thin ratio-1/9 film. Total solvent balance computed per pixel from the law (1e-4); every pixel's ratio within its neighborhood's ratios × evaporation (1e-4); the thin film's ratio rises where the thick film's liquid came in; movement guard kept (≥ 20 px gaining ≥ 1%) | `thinner_physics.rs:459`, `:485`, `:501`, `:516` |
| R6 | Check 10 leveling: two copies with the same paint and cure, with and without solvent, the same span; the solvent copy levels > 2% and > 2× the other; paint balances in both | `thinner_physics.rs:248` |
| R7 | Checks 11 and 12 compare whole session saves (`save::write`: canvas, seed, counters, studio clocks, piles, setup), and check 11 compares them right after reopening too. Boundary stated: no Lua globals or held tools in a save (save.rs:7-12) | `crates/easel/src/thinner_measure.rs:62`; `thinner_tests.rs:171`, `:216` |
| R8 | Check 18: `cloth.lua` makes a rag holding paint and solvent (asserted) before the snapshot; the failed chunk dips, wipes and refolds it; `cloth_again.lua` reuses it, and both sessions must match | `thinner_tests.rs:237`; `crates/easel/tests/thinner/{cloth,failing,cloth_again}.lua` |
| R9 | `--all` writes `notes/thinner/rag_study.png` and fails without it; ≥ 100 lifted pixels and a spread of ≥ 4 units² both ways before any shape is judged; the dry swatch must be there before its wipe. Visual approval is separate | `thinner_tests.rs:409`, `:414`; runner `:184` |
| R10 | `scripts/thinner_dump_fields.py`: `wet.solvent` (f32, the shape of `wet.vol`) in every chunk, `solvent` on every bristle, `solvent_mm3` on every rag, at least one brush and one rag seen; check 2 part 3 runs it on both runs' dumps | `scripts/thinner_dump_fields.py`; `scripts/thinner_check2:61` |
| R11 | ACCEPTANCE.md names cargo, bash 5, python3 and xz, the lockrun commands with 60/300/600 s, and this report | `notes/thinner/ACCEPTANCE.md`, "How to run" |
| R12 | The PAINTCK9 layout is stated as a deliberate format contract; `with_solvent_scaled` checks that the copy saves back identical outside the solvent block, so pigment, stiffness, cure, clock and everything else are unchanged | ACCEPTANCE "The interface"; `crates/paint/tests/thinner_support/mod.rs:198`, `:214` |
| R13 | Check 15: ≥ 12 µm (above the clamp, drying.rs:251), positive solvent, 10th/50th/90th percentiles. Check 4: a wholly unthinned control scene. Check 14: paint and solvent compared separately plus the total, positive deposits. Check 8: one card per setting, same place and seed. Check 17: a fractional start, and a stroke between waits | `thinner_physics.rs:381`, `:73`, `:327`; `thinner_tests.rs:148`; `thinner_physics.rs:579`, `:587` |
| R14 | Check 1: guards for a finite, positive contrast before and for paint laid; also < 0.001 solvent left on the whole canvas, so spreading out of a strip can't pass for evaporating. Target unchanged | `thinner_tests.rs:38`, `:43` |
| R15 | Tiny logs with no engine line, engine 2 and engine 3 replay as engines 1, 2 and 3 (studio and canvas); af49348's engine-3 logs replay as engine 3 | `thinner_tests.rs:92`, `:85` |

### Choices the reviewers should check

None of these weakens a requirement. Each is a choice the required change
left open.

1. **R7, `chunks=`.** A save's header counts the session's own log
   (save.rs:47), which a reopened session restarts. So the save of a
   reopened session differs from the original's in that one line by
   design, thinner or not. `session_save` rewrites it to all the chunks
   the painting has had (`chunks_before` + the log) in both sessions; every
   other byte is compared as written. Without that, check 11 would fail on
   existing save behavior unrelated to the thinner.
2. **R5, the order within a minute.** The balance accepts either order of
   evaporation and spreading within one step: Σ s0·e(h0), or Σ s1/e(h1) =
   S0. Each is computed independently to 1e-4. The plan doesn't fix the
   order. Unlike the round-1 slope test, a uniform error on the solvent
   fails both, and so does the per-pixel bound.
3. **R4, τ per pixel.** The law is tested at the pixel's own paint
   thickness, and ACCEPTANCE.md now says τ takes the pixel's own paint. A
   model that sets τ from a neighborhood's thickness (as drying does for
   cure, `film_thickness`) would fail it. If the builder wants that, it
   goes back to review.
4. **Time budget, unknown.** `--all` now runs 20 cards at 480 px (R13),
   the 20-day gel comparison, the rag study and the 2400 px card in one
   600 s batch. The time is unknown until the thinner exists. If it runs
   over, it is reported as unfinished and investigated small; the limit
   isn't raised.
5. **Risks in fixtures**, noted in ACCEPTANCE.md. Check 9 needs ≥ 200
   pixel-minutes where the paint moved ≤ 1e-3 in a minute. Check 16 needs
   visible spreading within one minute. If the model's leveling makes
   either impossible, the fixture goes back to review; the test isn't
   weakened.

I disagree with none of R1-R15.
