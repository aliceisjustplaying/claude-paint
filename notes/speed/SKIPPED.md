# What the gate runs, and what it doesn't

`scripts/test` is the one gate, run the same way by hand and by CI
(`.woodpecker/test.yaml`). It builds the workspace's test binaries once
(`cargo test --workspace --no-run`, test profile), then runs the checks below,
at most four at a time. A check passes only if it exits 0 and shows its tests
ran: each Rust test named here prints `ok`, and every other check prints a
count of at least one. On a 4-vCPU cloud machine it takes TIME_COLD from an
empty target and TIME_WARM once built.

## What it runs

- paint, by exact name: `tests::{fixture_matches_golden, drag_conserves_paint,
  a_stroke_reaches_its_end, dry_is_idempotent_and_clears_wet, fixture_is_deterministic_across_thread_counts}`,
  `drying::tests::checkpoint_mid_drying_resumes_exactly`,
  `noise::tests::worley_finds_the_true_nearest_features`,
  `bristle::knife_skim::a_skimming_knife_keeps_its_light_on_wet_dark_paint`, and
  the blender's feather and the brush skim: `handling::wet_edges::{a_blend_feathers_a_wet_edge_onto_dry_paint_and_stops,
  a_thin_stroke_on_wet_paint_keeps_its_value_under_a_gentle_blend,
  a_blend_still_works_a_thin_wet_film_on_dry_paint}`.
- rag, by exact name: `rag::rag_lift::{a_wipe_lifts_a_wet_passage_and_lays_none_of_it_further_along, a_mask_wipe_lifts_nothing_outside_its_mask}`.
- easel, by exact name: `session::tests::engine_2_logs_replay_as_before`,
  `session::tests::{new_paintings_record_the_glaze_film_and_older_logs_replay_without_it,
  a_glaze_tints_the_drawing_under_it_and_keeps_it_legible, a_scumble_at_half_coverage_leaves_gaps}`,
  `session::tests::new_paintings_record_the_rag_lift_and_older_logs_replay_without_it`,
  `api::draw_rag::tests::a_wipe_on_a_full_face_says_so`,
  `session::tests::a_gesture_keeps_its_width_to_its_last_point`,
  `session::tests::new_paintings_record_the_knife_skim_and_older_logs_replay_without_it`,
  `session::tests::new_paintings_record_the_blend_feather_and_brush_skim_and_older_logs_replay_without_them`,
  `api::tests::math_random_keeps_old_draws_and_refuses_empty_ranges`,
  `api::draw_rag::tests::an_older_log_sees_the_globals_it_saw`,
  `api::blend_at_masks_tests::soften_keeps_a_graded_mask_graded`,
  `api::blend_at_masks_tests::an_unclipped_blend_or_glaze_reaches_half_a_brush_past_its_mask` and
  `boxes::a_round_19_log_replays_as_before` (`#[ignore = "slow"]`, run by name:
  the round 19 log at 2400 px against `r19_default_box.png`).
- easel integration files, whole: `smoke.rs`, `session_integrity.rs`,
  `checkpoint_paths.rs`, `save_confined.rs` and `ck7.rs`.
- The thinner, by exact name: `c13_every_pigment_value_is_af49348s`,
  `c04_a_stroke_and_a_wipe_account_for_all_paint_and_solvent` and
  `c09_the_solvent_evaporates_and_the_film_loses_its_volume`.
- The stored baselines' scripts: `scripts/tests/old_logs.sh`,
  `scripts/tests/baseline_state.sh`, `scripts/thinner_check2`,
  `notes/thinner/baseline/tools/verify_package.sh`,
  `scripts/tests/image_tolerance.py` and the comparators' pytest guards
  (`baseline_state_compare_test.py`, `known_rag_baseline_test.py`).
- The Python suites production code depends on: the round 24 runner
  (`notes/round24/runner`), `studio/test_studio.py` and the easel MCP tests
  (`harness/easel_mcp/test_protocol.py`, `test_remote.py`).

**The known failure.** The thinner baseline's rag scene differs from
af49348's (`scripts/known_rag_baseline.py` holds its signature).
`baseline_state.sh` and `thinner_check2` exit 4 with
`KNOWN_FAILURE: rag-historical-baseline-main` for it, and the gate names it
as a known failure on every run. Any other failure fails the gate.

## What it doesn't run

- Every Rust test not named above.
- `session::tests::logs_without_an_engine_line_replay_as_before`: a stored
  engine-1 baseline kept as code, `#[ignore]`d because it replays whole
  paintings. `old_logs.sh` replays the first chunks of its two studios.
- `scripts/tests/box_notes.sh`, `export_profiles.sh` and
  `export_concurrent.sh` (they need `R16_BRANCH` and build painter easels),
  `box_tubes.sh`, `studio_names.sh`, `test_strip_sources.py`,
  `thinner_dump_fields_test.py`, and the runner tests of rounds 17 to 23.

What that gives up: worker-count equality and footprint and cache bounds of
engines 5 and 7, the painter build's box and command exposure (no feature
build matrix, no painter or replay-without-finish build), and finish routing
and replay fallback (`finish_painting`, `check_painting`, `replay_clip`,
`replay_batch` and `peek` have no tests in the gate).
