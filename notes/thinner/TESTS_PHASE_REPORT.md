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
