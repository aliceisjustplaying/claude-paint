# Speed agent, phase 4: round-2 review fixes

Branch `codex/speed`, from `0d802c2` (phase 3). Inputs: the lead's phase 4
brief, `~/src/a/claude-paint-reviews/safeguards-review-astra-r2.md` (APPROVE
WITH REQUIRED CHANGES) and `speed-tests-review-astra-r2.md` (APPROVE, with
SHOULD-FIX items). Nothing pushed, no main, no tags, no painting replayed.
Heavy jobs went through `scripts/lockrun`, started with `job_run`. Item 2 (S2)
was made by a helper agent on `codex/safeguards3` and merged (`f8c3427`).

| item | finding | fix | commit | evidence |
|---|---|---|---|---|
| 1 | S1 / R1: a nested run's leftover sweep killed the outer batch's other processes | the runner sweeps the whole process group only when lockrun started it as its own job (lockrun's record: its leader is our parent and its command is this runner with `--locked`). Otherwise, as nested in a batch or in `test_candidate`'s lockrun, it stops only its own steps' processes: those carrying its marker (below) and descendants seen with its steps that are still in the group | `120b0aa` | `test_runner.sh` case 12: inside an outer lockrun job, a sibling `sleep` survives while the step's own TERM-resistant leftover is stopped. Mutation (treat every run as owning the group): case 12 fails with the sibling killed. The candidate path keeps its guarantees: the runner still stops every process of its steps, and `test_candidate`'s lockrun still stops the group at its end (cases 3, 4, 6 and the candidate run below) |
| 2 | S2: `test_candidate` ran the candidate's own lockrun | the approved lockrun is pinned by SHA-256 (`cfcc8e51…105185`) in `scripts/safeguards_lib.py`. `test_candidate` takes `$LOCKRUN` (refused unless pinned), else `~/src/a/claude-paint-tools/lockrun`, else the `scripts/lockrun` beside itself, each only if its hash is the pin. It copies the checked bytes outside the worktree and runs that copy. The receipt records the path, source and hash; `merge_candidate` refuses a receipt whose lockrun hash isn't the pin or is missing. `scripts/lockrun` and `scripts/tests/lockrun.sh` are protected | helper `d4351ac`, `d4c77ad`, `8620b48` | `safeguards.sh` 121 checks (was 111): a forging candidate lockrun is never run (its marker file stays absent, the receipt's lockrun is the pin, from outside the candidate); a non-pinned `$LOCKRUN` refused; merge refuses an edited or removed lockrun hash. With `0d802c2`'s `test_candidate`, exactly the new checks 41–43 and 51–54 fail |
| 3 | S3: a step child that leaves the group survives (the easel's session server, `main.rs:478`) | every step runs with `CLAUDE_PAINT_TEST_RUN=<run id>:<run id>.<step>` in its environment. Its processes keep it when they leave the group, and no unrelated process has it, so pid reuse can't confuse it. The runner reads environments with `ps -E` on macOS and `/proc/*/environ` on Linux. A step's timeout, the leftover sweep and a cancel stop the marked processes whatever their group. sccache's server (shared with other jobs) is never stopped | `120b0aa` | case 13: a TERM-resistant child that calls `setpgid(0, 0)` is stopped both when its step ends normally and when it times out; a detached daemon named sccache is left alone. Mutation (ignore marks): case 13 fails. With the real easel, a step that ran `easel open` left its session server (own process group) running; the runner stopped it ("stopped 1 leftover processes"; none left) |
| 4 | R2: case 2 passed with only the leader stopped | case 2 now runs the timed-out step inside a group beside a watcher step, which checks the TERM-resistant child is gone 8 s in, before the group ends and any sweep could run | `120b0aa` | mutation (SIGTERM only the leader): case 2 fails ("child-alive") |
| 4 | R3: nothing tested the `--locked` validation | case 14: `--locked` with no token, another token or another group's record is refused before any step runs | `120b0aa` | mutation (validation off): case 14 fails |
| 4 | R4: case 10 couldn't fail | case 10 sets the target dir only in a Cargo `config.toml` (CARGO_HOME, CARGO_TARGET_DIR unset); the step falls back to the stale dummy easel if the variable is missing | `120b0aa` | mutation (no export): case 10 fails |
| 4 | R5: a script step's count could match the log's `$ <command>` header | counts skip the header line the runner writes | `120b0aa` | case 15: a step whose regex matches only its command text runs 0 tests and fails. Mutation (count the header): case 15 fails |

`scripts/tests/test_runner.sh`: 15 checks (was 11), 54 s, all pass. Each
mutation above was applied to `scripts/test` alone, the named case run, and
the file restored (`cmp`-checked).

What the runner still does not stop (its docstring says so): a shared daemon
(sccache), a process that clears its environment and leaves the group, and,
when the coordinator itself is killed with SIGKILL, a marked process outside
the group (lockrun stops only the group). The scratch directories a killed
coordinator leaves (`/tmp/cpt.*`, N1/N4) are unchanged.

## The candidate run

`scripts/test --all --candidate a6805e4c56284c9af8e45c2509148fb6d3fe0771`
(the normal path): **PASS**, 22 steps, 760 counted tests, 423.4 s of the 600 s
lock (build 100 s), fresh full worktree under `/tmp`, removed afterward.
Receipt: `git notes --ref=test-receipts show a6805e4` (local, not pushed),
files in `~/src/a/claude-paint-receipts/a6805e4…/20261004T053806-70247/`.
Format v2, no problems; lockrun from `~/src/a/claude-paint-tools/lockrun`,
sha256 equal to the pin; manifest `all.tsv` sha256 `1e00527e8057…`, runner
`scripts/test` `35fc0d9c3df6…`; checkout after the run unflagged and equal to
the commit.

Step times (s): release build 63.6, test build 36.4, cargo-test 41.4, old-logs
5.9, baseline 3.2 + 1.3; group s: slow tests 125.9, thin_blend 110.7, painter
33.0; lockrun 21.6; replay-env 6.0; check-live 22.0; group w: box-features
95.0, replay-only smoke 56.2, test-runner 57.2 (15), safeguards 90.8 (121),
runner 33.2 (254), studio 8.9 (23), replay-clip 6.0, box-tubes 1.3,
studio-names 3.5, peek 7.9.

`scripts/golden_approve check --candidate a6805e4 --base 3379b9f` (read
only): the old-log package and the baseline are approved; 7 protected changes
wait: `notes/golden_paths.txt`, `notes/speed/test_lists/all.tsv` and
`fast.tsv`, `scripts/lockrun`, `scripts/test`,
`scripts/tests/baseline_state.sh`, `scripts/tests/lockrun.sh`.

## Approvals still needed (lead)

The seven above. Integration must add the thinner's checks to `all.tsv`
(review N2) through the same approval.

Commits in this phase: `120b0aa` (runner: items 1, 3, 4), helper `d4351ac`,
`d4c77ad`, `8620b48` merged in `f8c3427` (item 2), `a6805e4` (safeguards step
minimum 121; the tested commit), and this report.

# Phase 5: the gate protects itself

Commit `ace4a41`. The review round 3 (APPROVE) found the gate's own tools
unprotected, so a candidate could supply its own gate.

- `notes/golden_paths.txt` now lists `scripts/test_candidate`,
  `scripts/merge_candidate`, `scripts/golden_approve`,
  `scripts/safeguards_lib.py` (it holds the lockrun pin) and
  `scripts/tests/safeguards.sh`. They import nothing else from the
  repository (each imports only `safeguards_lib.py` beside it). They run only
  the pinned lockrun and the candidate's `scripts/test`, both already
  protected.
- `notes/speed/SAFEGUARDS.md`, new section "Run the gate from the approved
  copy": merges run the gate from `~/src/a/claude-paint-tools/gate/`, never
  from a candidate's or a working copy's `scripts/`.
- The copy is at `~/src/a/claude-paint-tools/gate/`: the four files from
  `ace4a41` (unchanged since `800aa60`, the approved version), a README with
  the source commit and sha256s, and `SHA256SUMS`. `shasum -a 256 -c
  SHA256SUMS` reports OK for all four, and
  `~/src/a/claude-paint-tools/gate/golden_approve check --candidate 800aa60
  --base 3379b9f` run from the repository passes.
  - test_candidate `7904db98b0157c2f2377e47bf01ac4ebdc993052a10e187ce1156b324de210a7`
  - merge_candidate `a23e2bfe8542037d3bde864ada9428ce66d0e56f45c8cfcf6e8049b59f9c6755`
  - golden_approve `e283ff70dd6d0b607baf2ef33c91bfa1015566655644affd7eab1be5c66a8891`
  - safeguards_lib.py `7616dd6128e519fe486c44986f076970ffd0beb3b2800aab12ea0d6bcdcb199f`
- `scripts/tests/safeguards.sh`: 123 checks (was 121). Checks 77 and 78: on a
  dummy base that protects the gate files, a candidate that edits
  `merge_candidate`, or `safeguards_lib.py`'s pin, is reported as an
  unapproved protected change. Result `# 123 checks, 123 passed, 0 failed`,
  70.7 s under lockrun. The `safeguards` step in `all.tsv` still requires at
  least 121 checks: still true, and the list is approved, so it isn't
  changed.
- Needs approval (lead): the changed `notes/golden_paths.txt` and
  `scripts/tests/safeguards.sh`.

## Looking ahead to the combine (read only, `thinner2` at `ffef02e`)

`git merge-tree` of `thinner2` and `codex/speed` (base `a97c3a6`) is
textually clean. Four files are changed on both sides: `crates/easel/src/api.rs`,
`main.rs`, `session.rs` and `crates/paint/src/rag.rs`. The hunks don't
overlap: the thinner's pile and rag fields and its test modules; speed's
test-module gates, swatch tests, ignore attributes and the rag test helper.
`README`, `Cargo.toml` and `Cargo.lock` are changed on one side only, or not
at all. The conflicts to expect are semantic:

1. **Check 13 (b) fails by design.** `c13_burnt_sienna_shows_the_card_at_least_as_well_as_raw_sienna`
   (`crates/paint/tests/thinner_pigments.rs:107`) is a plain `#[test]` that
   fails until the user decides. `scripts/test`'s cargo step runs every
   non-ignored test, so the fast and `--all` runs fail on it, and so does
   `test_thinner_acceptance` (exit 3, "NOT ALL GREEN"). No candidate can
   pass until the user decides. That is correct; I would not skip it.
2. **The thinner's tests would run twice, once in the wrong profile.**
   `cargo_tests.sh` runs every test binary in the test profile: `thinner_physics`,
   `thinner_pigments` and the easel binary's `thinner_tests` module. Their
   approved command is release (`test_thinner_acceptance`). I propose
   `cargo_tests.sh` leaves them out (the two binaries by name,
   `--skip thinner_tests::` for the easel binary), with an entry in
   SKIPPED.md, and that they run only through their own runner.
3. **Protected changes on the thinner side.** `crates/paint/src/state_dump.rs`
   (+3 lines: `wet.solvent`, an added field, as the baseline rules allow)
   and the 22 frozen files appear, with the round-4 corrections if those
   change them. All need approval at the combine.
4. **Old logs are probably unaffected.** Engines 1 and 2 keep `PAINTCK8`, and
   the thinner keeps a bristle's `Debug` text as before for engines below 3
   (`BristleText`), so the eleven old-log digests should hold. `old_logs.sh`
   will show it. `baseline_state.sh` is check 2's command.
5. **`--all` writes into the repository.** `test_thinner_acceptance --all`
   writes `notes/thinner/rag_study.png`. In a candidate worktree that's an
   untracked file (test_candidate refuses tracked changes, not untracked);
   in a working copy it's litter. Better under `$TMPDIR` (a protected file:
   its own review).
6. **Lists and limits.** `fast.tsv` and `all.tsv` need thinner steps (a
   protected change; review N2), and the card test needs 300 s.

How `scripts/test` should include the acceptance checks (a proposal, not done):

- `fast.tsv`: a build step `cargo test --release -p paint --test thinner_physics
  --test thinner_pigments --no-run` plus the easel binary's release tests
  (`--no-run`). Then a step `scripts/test_thinner_acceptance --quick` (60 s;
  its runs took a few seconds once built).
- `--all`: `scripts/test_thinner_acceptance --all`, which includes the card
  (173 s inside lockrun once built, at `afc7898`; the card alone 24.6 s). As
  one step it gets 300 s, so it fits the card's 300 s limit.
- **It doesn't fit one 600 s batch.** Today's `--all` takes 423 s from a
  fresh worktree, 100 s of it builds. The thinner adds release test builds
  (about 35 s warm at `afc7898`; a fresh release build of the easel test
  binary is likely 60–120 s) and about 170 s of checks, so roughly 650–700 s.
  I propose splitting the candidate run into two separately limited
  batches, each 600 s, matching the plan's "initial builds and final check
  batches: ten minutes each": first every build step of the list, then every
  check. Both run in one worktree, both limits and both results go in the
  receipt, and it passes only if both pass. The check batch would be about
  323 + 170 ≈ 490 s, less if the thinner's step joins group "s". The
  alternative, a separately receipted thinner batch, needs the same gate
  change and builds twice. Either way this changes `scripts/test` and the
  gate tools (protected): it needs review, and then the approved gate copy
  must be updated.
