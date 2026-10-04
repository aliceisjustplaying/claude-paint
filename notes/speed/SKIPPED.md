# What the test commands skip, and why

`scripts/test` runs `notes/speed/test_lists/fast.tsv` and `scripts/test --all`
runs `notes/speed/test_lists/all.tsv`. This file lists everything neither runs:
every ignored Rust test, and the script and Python checks left out. No test
command replays a painting.

Times are wall times of one test alone (`notes/speed/tools/time_tests.py`, test
profile, M3 Pro, `notes/speed/timing/`), unless marked otherwise.

## Ignored Rust tests

### Run by `scripts/test --all`, by exact name (`#[ignore = "slow"]`)

| test | checks | time | why not in `scripts/test` |
|---|---|---|---|
| `easel --test boxes a_round_19_log_replays_as_before` | correctness: the round 19 log replays to round 19's picture and surface at 2400 px (golden from r19-base) | 26.6 s | slow; `scripts/test` runs the same log at 320 px (`old_logs.sh`, case r19) |
| `easel --test determinism hand_time_is_deterministic_across_thread_counts_at_the_live_width` | correctness: hand-timed replays equal at 1 and 4 threads, at 2400 px | over 60 s (stopped at 60 s when timed) | slow; the quick form at 480 px runs in `scripts/test` (6.7 s) |
| `easel --test determinism state_digests_are_the_same_in_every_replay_at_the_live_width` | correctness: state digests equal at 1 and 4 threads, at 2400 px | 7.95 s | the quick form at 480 px runs (0.6 s) |
| `easel --test determinism the_rag_replays_the_same_at_the_live_width` | correctness: rag replays equal at 1 and 4 threads, at 2400 px | 38.6 s | the quick form at 480 px runs (2.9 s) |

### Never run: they replay whole paintings

The overnight plan forbids replaying paintings in tests, at any width. The
tests stay in the code as ignored experiments; nothing runs them.
`scripts/tests/old_logs.sh` (in `scripts/test`) replaces each with a tiny case,
with goldens from af49348's unchanged release easel.

| test | the failure it guards | replacement in `scripts/tests/old_logs.sh` | what the replacement doesn't cover |
|---|---|---|---|
| `easel session::tests::logs_without_an_engine_line_replay_as_before` (studios 6399ad, 7 chunks, and db6324, 18, at 320 px) | engine gating: a log with no `--@ engine` line stops replaying as engine 1, or engine 1's paths (dilution of the cure by fresh paint, hand time, grounds) drift | `engine1_tiny` (a synthetic engine-1 log: knife and brushed grounds, chalk and pencil, broad, glaze, body and detail passes, a stroke into open paint 20 min old, a touch, a blend, an outline painted, waits of 20 min, a day and three days, 160 px); `studio_6399ad` and `studio_db6324`: their first 3 chunks at 128 px | the studios' later chunks (most of their strokes and waits) |
| `easel legacy::tests::easel3_free_replays` (38 chunks at 200 px) | the legacy API (`canvas{style=, palette=}`, `world`, `w:view`, `w:sun_canvas`, `w:spot`, gradients) stops running the easel3 logs | `easel3_free`: first 3 chunks at 128 px | chunks 4 to 38 |
| `easel legacy::tests::easel3_green_replays` (31 chunks) | the same, greens palette, `w:sky`, `w:clouds`, `w:project` | `easel3_green`: first 3 chunks | chunks 4 to 31 |
| `easel legacy::tests::easel3_near_replays` (26 chunks) | the same, `stipple`, `wait` | `easel3_near`: first 2 chunks (chunk 3 alone took 1.8 s) | chunks 3 to 26 |
| `easel legacy::tests::easel4_free_replays` (20 chunks) | the same, `tree`, `mound`, pencil, outlines | `easel4_free`: first 3 chunks | chunks 4 to 20 |
| `easel legacy::tests::easel4_green_replays` (23 chunks) | the same, `world`, pencil `rule` and `sketch`, `show` | `easel4_green`: first 3 chunks | chunks 4 to 23 |
| `easel legacy::tests::easel4_near_replays` (20 chunks) | the same, bodies (`body.ellipsoid`, `cut`, `turn`, `rough`), `v:bodies_mask` | `easel4_near`: first 3 chunks | chunks 4 to 20 |

The legacy logs' later chunks use verbs no tiny case reaches (in the six
logs: `frond`, `stone`, `crack`, `sward`, `tuft_band`, `sk:airlight`,
`w:to_ground`, `glaze`, `dry` and others). That is **missing coverage**: an
API change that breaks only those verbs is not caught. A small synthetic
legacy log calling each would cover it; it isn't written.

How the replacements were shown to catch their failures (each mutation built
in release, `scripts/tests/old_logs.sh` run, the mutation reverted):

- engine gate (`logged_engine` returns 3 for a log with no engine line): all 10 cases fail (`notes/speed/logs/p2_mutation_engine_gate.log`);
- the legacy canvas's seed (`legacy::canvas` prepares with `seed + 1`): the six legacy cases fail, the engine-1 and r19 cases pass (`p2_mutation_legacy_canvas.log`);
- the legacy palette names mapped to the smalt boxes: easel3_free, easel3_green, easel3_near and easel4_free fail (their tubes are gone); easel4_green and easel4_near pass, because their first chunks only draw and set up the world (`p2_mutation_legacy.log`).

### Diagnostics and experiments (plain `#[ignore]`, not run)

| test | what it is | time |
|---|---|---|
| `paint --test ground_grain print_ground_grain` | experiment: prints each ground stack's grain | unknown |
| `paint --test ground_grain save_ground_crops` | experiment: writes lit crops of the bare grounds (`GRAIN_OUT`) | unknown |
| `paint --test ground_grain thin_blend_bares_ground` | **correctness** (asserts): a thin blended broad pass bares at most twice as much ground over the brushed ground as over the reference, three seeds at 3200 px | "~1 min in release, far longer in debug" (its doc); not measured. Left out of `--all`; see the report |
| `paint bristle::cover_tests::probe_bare` | experiment: where bare flecks in a dark passage come from | unknown |
| `paint bristle::cover_tests::probe_mark_area` | experiment: a stroke's covered area against its nominal size | unknown |
| `paint bristle::tip_tests::probe_ink_width` | experiment: ink width at two resolutions, for tuning | unknown |
| `paint crack::tests::full_canvas_speed` | speed measurement: cracks over a 3200 px canvas | unknown |
| `paint graphite::probe::tooth_depths` | experiment: pencil tooth depths | unknown |
| `paint handling::tests::probe_edges_over_seeds` | experiment: coverage at mask edges over seeds | unknown |
| `paint rag::tests::table` | experiment: the rag report's table | unknown |
| `paint tests::diag_blend_bare_pixels` | experiment: the pixels a thin blend leaves bare | unknown |
| `easel session::tests::real_failed_chunks` | experiment: rollback timings on real failed chunks (`EASEL_ROLLBACK_CASES`; replays the logs it names: run only on logs that aren't paintings) | unknown |

## Script and Python checks

In `scripts/test --all`: `box_features.sh` (with `BOX_FEATURES_PROFILE=test`),
`lockrun.sh`, `safeguards.sh`, `replay_env.sh`, `check_live.sh`,
`replay_clip.sh`, `box_tubes.sh`, `studio_names.sh`, `peek.sh`, the round 24
runner's pytest suite and `studio/test_studio.py`, plus the painter build's
tests (`cargo test -p easel --no-default-features --features box-inness`).
In `scripts/test`: `old_logs.sh`, `baseline_state.sh` and the baseline
package's `verify_package.sh`. None replays a painting: `replay_env.sh` and
`check_live.sh` paint a 2-chunk test log (a canvas and one stroke).

Not run by either:

| check | why | time |
|---|---|---|
| `box_features.sh` with release builds (its default) | `--all` runs it in the test profile: which tubes a build holds doesn't depend on the profile | several minutes (its header) |
| `box_notes.sh` | needs `R16_BRANCH` and exports five studios (painter easel builds) | several minutes; unknown |
| `export_profiles.sh`, `export_concurrent.sh` | need `R16_BRANCH`; build painter easels from nothing (one per box) | "a few minutes", "several minutes" (their headers) |
| the runner tests of rounds 17 to 23 (`notes/round*/runner/test_*.py`) | earlier rounds' copies of the runner; round 24's runs | not measured |

`replay_env.sh` failed from 7b80cb0 on (`claude-paint-round24-review.md`). Run
alone on its own 2-chunk log, `replay_clip` said: "--length 5 is longer than
the clip can run: 1 moment held at most 1 s each (--max-hold) plus the 3 s
final hold come to at most 4.00 s; raise --max-hold to at least 2.00 or lower
--length". The box handling it tests worked (check_painting and
finish_painting passed, replay_clip replayed with the log's box). The test
now passes `--max-hold 2` and passes (5.7 s). It also needs a short `TMPDIR`
(the session socket's path must stay under 104 bytes; `scripts/test` gives
the scripts a `/tmp/cpt.*` directory).
