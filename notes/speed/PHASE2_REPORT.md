# Speed agent, phase 2: smaller and faster tests, test commands, safeguards

Branch `codex/speed` in `~/src/a/claude-paint-speed` (helper branch
`codex/safeguards` merged in). Plan SHA-256 `382fc2de…8e30`; decisions file
as of phase 1. Nothing pushed, nothing on main, no tags, no painting run or
replayed. Every heavy command ran under `scripts/lockrun`.

## Results at a glance

| command | result | time |
|---|---|---|
| `scripts/test` (working copy, after changes) | **pass**: 6 steps, 287 tests | 75 s of checks once built (cargo tests 42.8 s) + build steps (0.1 s and 22 s when already built) |
| `scripts/test --all` (working copy, at `342b1f4`) | **fail**: 18 of 20 steps passed; `painter-build-inness` (a real, pre-existing compile error) and `peek` (not executable) failed; both fixed in `4356dbc` and `f3cec91` | 493.5 s |
| `scripts/test --all --candidate f3cec91` | **fail**: a fresh worktree under macOS's deep `$TMPDIR` made 15 easel tests fail binding sockets past 104 bytes; `test_candidate` also counted build steps as zero-test steps. Both fixed in `63f6b5c` | 562 s in a 600 s lock |
| `scripts/test --all --candidate 63f6b5c` | **fail**: one test, `session_integrity a_look_never_overwrites_an_earlier_observation`, compares the path the easel prints (resolved, `/private/tmp/…`) with `CARGO_TARGET_TMPDIR` (`/tmp/…`); fixed in the tool (`c53d40a`: resolved worktree path). (I meant to test `2f3f03a`; the job read `HEAD` before that commit landed.) | 571.8 s |
| **`scripts/test --all --candidate c53d40a`** | **PASS**: 19 steps, 669 tests, fresh full worktree, removed afterward; receipt `git notes --ref=test-receipts show c53d40a` (local, not pushed), files in `~/src/a/claude-paint-receipts/c53d40a…/20261004T035939-34555/` | 540.9 s of the 600 s limit (build 99 s, tests 441 s), after waiting for a thinner build |

The user's "6 to 10 minutes": I did not rerun the old suite to time it (the
plan forbids running old expensive tests for a before-time, and it replayed
seven paintings). What was measured before any change, one test per process
(`notes/speed/timing/before_*.tsv`): the paint library's 180 tests 148 s
serial (one at more than 60 s, stopped), the easel binary's 64 tests (7
painting replays not run) 67 s, the integration tests 238 s (one more than
60 s, stopped). `cargo test --workspace` after the changes took 102.5 s
(binaries one after another plus doctests); `scripts/test`'s cargo step runs
the same 270 tests in 42.8 s.

## 1. Inventory

- Rust: 10 test binaries, 288 tests (276 run by default, 12 ignored) at the
  start. List per binary: `cargo test --workspace --no-run`, then `--list`
  (`notes/speed/logs/p2_test_build.log`; test build 28.4 s).
- Scripts: 11 in `scripts/tests/` before this work (lockrun.sh is phase 1).
  Each read before running (`notes/speed/SKIPPED.md`: none replays a
  painting; `box_notes`, `export_*` need `R16_BRANCH` and painter builds and
  are left out).
- Python: the round 24 runner (`notes/round24/runner`, 254 tests), the studio
  viewer (`studio/test_studio.py`, 18 run, 5 skipped). The runner suites of
  rounds 17 to 23 are older copies, not run. The round 24 suite needs Pillow:
  without it 7 tests fail (`ModuleNotFoundError: No module named 'PIL'`,
  `p2_python_tests.log`); `--all` runs it with `uv run --with pytest --with pillow`.
- Earlier timing records: `notes/engine-map-2026-10-02.md` §D2 ("the whole
  suite runs in about a minute … plus about 50 s of build", quoting
  `notes/workflow.md`, out of date).

## 2. Tests made smaller or faster

Times: one test alone, test profile (`notes/speed/timing/before_*.tsv` and
`after_*.tsv`). No assertion was removed or loosened; no expected number of
an existing test changed.

| test | guards against | change | why it still catches it | before | after |
|---|---|---|---|---|---|
| `paint rag::tests::nothing_is_lifted_past_the_gel_point` | a rag lifting paint past its gel point | ages the canvas in one-hour waits tried on a copy (kept while paint stays open), then two-minute waits, instead of two-minute waits all the way | the same state: no open paint, stopped within two minutes of the last pixel setting (as before), and the same assertions | > 60 s (stopped) | 9.6 s |
| `easel tests::the_thick_swatch_matches_paint_laid_thick` | the palette's thick swatch differing from paint laid thick | split in three tests, one per pile (`…`, `…_dark`, `…_earths`), sharing a helper | the same three piles and assertions; they now run side by side | 11.3 s | 3.8 s each |
| `easel --test smoke a_short_session_at_the_easel` | the easel's commands (open, do, look, check, save, run, reopen, journal) and errors | the second ground is rolled, not brushed: a brushed ground is a whole-canvas `work` pass at 2400 px, and the test replays the log four times | every command and assertion is the same; brushed grounds are tested in paint (`tests::brushed_ground_honors_thickness`, `ground_grain`) and through the easel in `old_logs.sh` (engine1_tiny, r19) | 58.8 s | 7.7 s |
| `easel --test delivery save_delivers_the_wet_canvas_as_seen_and_replay_matches` | a saved PNG that isn't the wet canvas as seen, or a replay delivering other bytes | the wet passage covers the window the look reads plus a margin (`rect(40, 0, 520, 250)`, coverage 2) instead of the whole width at coverage 4 | the compared window is all wet paint; mutation: delivering the dry picture (`c.pixels()` for `c.seen()`) fails it ("max difference 107 levels", `p2_mutation_deliver.log`) | 19.7 s | 7.0 s |
| `easel --test determinism hand_time_is_deterministic_across_thread_counts` | hand-timed replays differing with the thread count | replays at 480 px (`easel run --width`); the 2400 px form is `…_at_the_live_width`, `#[ignore = "slow"]`, run by `--all` | work tiles are sized in canvas units, so the same tiles run on the same threads; mutation: seeding each tile's brush with the rayon thread index fails all four quick tests (`p2_mutation_thread_seed.log`) | > 60 s (stopped) | 6.7 s |
| `easel --test determinism state_digests_are_the_same_in_every_replay` | state digests differing between replays | 480 px; live form in `--all` | as above | 7.95 s | 0.63 s |
| `easel --test determinism the_rag_replays_the_same` | rag replays differing | 480 px; live form in `--all` (`rag(width 91, load 0.` still asserted) | as above | 38.6 s | 2.9 s |
| `easel --test boxes a_round_19_log_replays_as_before` | the round 19 log not replaying to round 19's picture | `#[ignore = "slow"]`, run by name in `--all`; `scripts/test` runs the same log at 320 px against af49348 (`old_logs.sh`, case r19) | the 2400 px golden is still checked in `--all`; the 320 px case failed under the engine-gate mutation | 26.6 s | 0 in `scripts/test` (39 to 46 s in `--all`) |
| the whole Rust suite | | `scripts/tests/cargo_tests.sh -j 4`: four test binaries at a time, no doctests (all four are ignored) | the same 270 tests (`scripts/test` requires at least 270) | 102.5 s (`cargo test --workspace`) | 42.8 s |

Not mutation-tested: smoke (only made cheaper), the rag test (its
assertions unchanged; reaching the same "nothing open" state), the
swatch split (mechanical).

## 3. Replacements for the whole-painting replays (lead's decision)

The seven tests that replayed whole paintings are
`#[ignore = "replays whole paintings: never run"]` (kept in the code; no
command runs them). `scripts/tests/old_logs.sh` (in `scripts/test`, 4.8 s)
replays ten tiny cases and compares PNG sha256 and per-chunk state digests
with goldens from af49348's unchanged release easel (option B;
`crates/easel/tests/old_logs/golden/README.md` has the command, binary
hash and file hashes).

| old test | failure it guards | replacement | catches it? |
|---|---|---|---|
| `session::tests::logs_without_an_engine_line_replay_as_before` | engine gating: a log with no `--@ engine` line stops replaying as engine 1 | `engine1_tiny` (synthetic engine-1 log, 5 chunks, 160 px), `studio_6399ad` and `studio_db6324` (first 3 chunks, 128 px) | engine-gate mutation (`logged_engine` defaults to 3): all three fail, as do the other seven (no engine line either) (`p2_mutation_engine_gate.log`) |
| `legacy::tests::easel3_free_replays` | the legacy API (styles, palettes, `world`, views, sun, spots) stops running these logs | `easel3_free` (first 3 chunks) | legacy canvas-seed mutation fails all six legacy cases and none of the others (`p2_mutation_legacy_canvas.log`); legacy palette mutation fails it (`p2_mutation_legacy.log`) |
| `legacy::tests::easel3_green_replays` | same, greens palette, sky, clouds | `easel3_green` (first 3) | both mutations |
| `legacy::tests::easel3_near_replays` | same, stipple, wait | `easel3_near` (first 2) | both mutations |
| `legacy::tests::easel4_free_replays` | same, trees, mounds, pencil, outlines | `easel4_free` (first 3) | both mutations |
| `legacy::tests::easel4_green_replays` | same, pencil rule and sketch | `easel4_green` (first 3) | seed mutation only (its first chunks don't use the palette) |
| `legacy::tests::easel4_near_replays` | same, bodies | `easel4_near` (first 3) | seed mutation only |

**Missing coverage:** the legacy logs' later chunks (verbs such as `frond`,
`stone`, `crack`, `sward`, `tuft_band`, `sk:airlight`, `w:to_ground`,
`glaze`, `dry`) and the engine-1 studios' later chunks are not reached. A
synthetic legacy log calling each verb would close the first gap; not
written.

## 4. Option B goldens and answers waiting for the user

- **Option B (from unchanged af49348, for a reviewer to check and record in
  `refs/notes/golden-approvals`):** `crates/easel/tests/old_logs/golden/*.txt`
  (10 files) with their inputs (`crates/easel/tests/old_logs/*.lua`,
  `cases.tsv`) and `scripts/tests/old_logs.sh`. Recorded by
  `scripts/lockrun --timeout 60 -- scripts/tests/old_logs.sh <af49348 release easel> --record crates/easel/tests/old_logs/golden`,
  easel sha256 `4201cec1…40f6` (`p2_old_logs_record.log`). Added to
  `notes/golden_paths.txt` (`91d5b62`). Suggested record (lead or reviewer):

  ```sh
  scripts/golden_approve record --commit 91d5b62 --approver <reviewer> --builder speed \
    --paths crates/easel/tests/old_logs/ scripts/tests/old_logs.sh \
    --reason "old-log goldens, computed by af49348's unchanged release easel" \
    --method "option B: crates/easel/tests/old_logs/golden/README.md"
  ```
  (`--dry-run` of this command at 91d5b62 lists the 22 files and records nothing.)
- **Waiting for the user:** nothing. No expected number was computed by changed code.

## 5. Test commands

- `scripts/test` and `scripts/test --all` (one script; lists in
  `notes/speed/test_lists/fast.tsv` and `all.tsv`). Each step has a time
  limit and a least number of tests; a step that runs fewer fails, so a test
  name that matches nothing can't pass. The run holds one outer lockrun (15
  min) and gives scripts a short `/tmp/cpt.*` `TMPDIR`. `--only` runs
  chosen steps and is never a pass. JSON summary for `test_candidate`.
- `scripts/test` (fast): release easel build, test build, the 270 Rust
  tests, `old_logs.sh`, the baseline package check and the engine against
  the baseline (`baseline_state.sh`: every state field and PNG equal to
  af49348).
- `scripts/test --all`: the fast steps, then the four slow tests by exact
  name, the painter build's tests (`--no-default-features --features
  box-inness`), `box_features.sh` (test profile), `lockrun.sh`,
  `safeguards.sh`, `replay_env.sh`, `check_live.sh`, `replay_clip.sh`,
  `box_tubes.sh`, `studio_names.sh`, `peek.sh`, the round 24 runner tests and
  the studio viewer tests.
- `notes/speed/SKIPPED.md`: every ignored test (name, reason, correctness or
  experiment, time) and the checks neither command runs.

Found on the way:

- `replay_env.sh` (failing since 7b80cb0): a small case showed it was
  `replay_clip --length 5` on a 2-chunk painting, refused by `--max-hold`
  (1 s default): "raise --max-hold to at least 2.00". The box handling it
  tests worked. Now `--max-hold 2`; passes in 5 s. It also needs a short
  `TMPDIR` (socket paths).
- The painter build's unit tests didn't compile (`api.rs`, `form.rs` test
  modules used default-box sessions). Gated to `cfg(all(test, tube_box))`
  as `Session::new` is (`4356dbc`). `box_features.sh` only compiled the
  painter integration test, so it never saw this.

## 6. Safeguards (helper agent, `codex/safeguards`, merged)

`scripts/test_candidate`, `scripts/golden_approve`, `scripts/merge_candidate`,
`notes/golden_paths.txt`, `scripts/tests/safeguards.sh` (53 checks on dummy
repositories, all pass, about 53 s) and `notes/speed/SAFEGUARDS.md` (formats,
the lead's commands including the a97c3a6 baseline approval, limits). After
the merge I fixed two things the first real run exposed (`63f6b5c`): the
candidate worktree now lives under `/tmp` (socket path limit), and build
steps need no tests while a build-only summary fails (two new checks). No
receipt or approval was recorded except the candidate receipts below, which
are local notes (`refs/notes/test-receipts`), not pushed. merge_candidate
was tested on dummy repositories only. No independent review yet.

## Commits (on `af5beb3`)

| commit | what |
|---|---|
| `dc8659c` | smaller and faster tests (rag, swatch, smoke, delivery, determinism, r19) |
| `99accaa` | painting replays never run; tiny old-log replacements with af49348 goldens |
| `7c2f57a` | `replay_env.sh` fix; `box_features.sh` profile option |
| `86d7281` | `scripts/test`, lists, `cargo_tests.sh`, `baseline_state.sh`, SKIPPED.md, timings |
| `8e89e4d`…`7be7442` | safeguards (helper), merged in `0348095` |
| `342b1f4` | `notes/agent_brief_template.md`, linked from the README |
| `4356dbc` | painter build's unit tests compile again |
| `f3cec91` | `--all` list fixes (peek, painter count) |
| `91d5b62` | old-log goldens added to the protected list |
| `63f6b5c` | `test_candidate` fixes from its first real run |
| `2f3f03a` | the four slow tests run side by side in `--all` |
| `c53d40a` | `test_candidate` resolves its worktree path; **the commit that passed** |

## Unfinished or needing a decision

- `paint --test ground_grain thin_blend_bares_ground` is an asserting
  correctness test (3200 px, "~1 min in release, far longer in debug") that
  no command runs. Should `--all` run it, or is it covered enough?
- The legacy and engine-1 later chunks: missing coverage (section 3).
- `--all` from a fresh worktree is close to its 10-minute limit (540.9 s in
  the passing run, 63.6 s of it the release build). `box_features.sh` (130 s, 12 builds) and the slow
  determinism test (88 s) are the largest steps.
- No independent review of the test changes or the safeguards yet.
