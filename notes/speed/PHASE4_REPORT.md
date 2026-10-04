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
