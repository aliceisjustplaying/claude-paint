# Speed agent, phase 3: review fixes, the ground check, headroom, legacy coverage

Branch `codex/speed`, from `c86c48d` (phase 2). Inputs: the lead's phase 3
brief, `~/src/a/claude-paint-reviews/speed-tests-review-astra.md` (APPROVE WITH
REQUIRED CHANGES) and `safeguards-review-astra.md` (REJECT). Nothing pushed, no
main, no tags, no painting replayed. Every heavy job ran through
`scripts/lockrun`, launched with `job_run`. The safeguard fixes (SB2, SB3, SB5,
S5 and the safeguard SHOULD-FIX list) were made by a helper agent on
`codex/safeguards2` and merged (`e093d32`); I own them and checked them by the
real candidate run below.

**`notes/golden_paths.txt` is final at `bb31d5b`** (B5 in `a0b2c25`, SB4 in
`bb31d5b`; no later commit changes it).

## Findings and what was done

| finding | fix | commit | evidence |
|---|---|---|---|
| B1 / SB1: steps escape the outer lockrun group; no SIGTERM handling; escalation stops at the leader; any token bypasses the lock | steps stay in the lockrun job's process group (no new session), so lockrun's own timeout, cancel and crash handling reach them and it holds the lock until none is left. A step past its limit is stopped with every descendant seen with it (repeated `ps` scans: a child that outlives its parent is still known), SIGTERM then SIGKILL until none is left. After each step or group, leftovers in the group are stopped. SIGTERM, SIGHUP, SIGINT stop the steps and end "unfinished" (exit 130). Always runs through lockrun, which validates an inherited token; the internal `--locked` flag is refused unless lockrun's record names this group and token | `a2d7569` | `scripts/tests/test_runner.sh`, 11 checks through the real runner (outer timeout, lockrun cancelled, SIGTERM to the coordinator, coordinator SIGKILLed, stale token, nesting, a leader exiting before a TERM-resistant child, orphaned grandchild leftovers). Mutations: `start_new_session=True` restored fails cases 3, 4, 6; trusting any token fails 7; stopping only the leader (no sweep) fails 2 |
| B2 / SB6: 900 s outer limit | 600 s on both paths; a list step over 600 s is refused | `a2d7569` | test_runner case 9 |
| SB6: fast cargo step 180 s | 60 s (it measured 42 s in every run); every script and Python step of `--all` 60 s too, except `safeguards.sh` (111 checks, 65 s alone, 71 s beside box_features: 180 s). Builds keep 600 s (they are builds, not ordinary checks). The slow tests (240 s, 180 s), the painter and replay-only builds with their tests (300 s) and box_features (12 builds, 600 s) are batch parts, each well under the 600 s batch | `a37d47e`, `4ffd0f5`, `8053fc1` | step times below |
| B3: `thin_blend_bares_ground` absent from `--all` | `#[ignore = "slow"]`, run by exact name in `--all` (180 s step), unchanged at 3200 px with its bound unchanged. No smaller version: see item 1 | `a37d47e` | runs: 103.8 s and 105.0 s, passes |
| B4: golden steps run a literal `target/release/easel` | the runner resolves cargo's target dir (`cargo metadata`, so CARGO_TARGET_DIR and config) and exports it; the steps run `"$CARGO_TARGET_DIR/release/easel"`; `old_logs.sh` and `baseline_state.sh` default to it | `a2d7569`, `4daf619`, `a37d47e` | test_runner case 10: two distinguishable dummy easels (stale and fresh); the step runs the fresh one, and with CARGO_TARGET_DIR unset the summary's target dir is cargo's |
| B5: r19 input unprotected | `crates/easel/tests/r19_default_box.lua` in the protected list | `a0b2c25` | |
| SB4: thinner frozen set unprotected | the 22 paths of `thinner-tests-APPROVED.md` plus `scripts/tests/baseline_state.sh`, `scripts/test` and `notes/speed/test_lists/` added; paths absent on codex/speed are fine (helper's dummy cases: listed-but-absent passes; appearing needs approval) | `bb31d5b` | helper: `golden_approve check --candidate a0b2c25 --base 3379b9f` reads it |
| SB2: pass after tracked files changed | the checkout is rechecked after the run (HEAD, tree, index, sparse, skip-worktree and assume-unchanged flags, tracked changes); any difference fails; merge refuses missing or inconsistent evidence | helper `f722eb0`, `63fbdb1` | safeguards.sh 111 checks |
| SB3: check set not bound | the receipt binds the list and runner of the candidate commit; the summary's steps must equal the list's exactly (name, kind, command, limit, least, group, order); list and runner hashes checked; merge re-reads them and requires them protected. The runner writes `runner_sha256`, `least`, `group` | helper; `a2d7569` | safeguards.sh; the candidate run below exercises it on the real list |
| SB5: gitlink at a protected path skipped | every entry type listed; non-regular types refused unless an approval names the exact mode and object | helper | safeguards.sh |
| S2 / item 2: `--all` headroom | see item 2 | | |
| S3: doc examples and Pillow skips | Pillow installed for the studio tests, all 23 required (none skipped); the four ignored doc examples listed in SKIPPED.md | `a37d47e`, `6f8ba45` | studio-viewer: 23 run |
| S4: equivalence wording | determinism's 480 px tests described as smaller invariance cases (tile margins depend on width; rayon schedules dynamically); the rag shrink as reaching the same no-open-paint precondition, not the same wait integration (this report) | `6f8ba45` | |
| S5: inner timeout recorded as fail | the receipt keeps the runner's "unfinished" (exit 3) | helper | safeguards.sh |
| S6: added-zero reach | not changed (the comparator is protected); the brief now says exactly what `--added-zero` checks: added numeric fields all zero, added values inside the Debug text 0, `false`, `None` or empty; not checked: a whole added text field, an added list's length. A stricter check needs a reviewed change to protected tools; I didn't touch them | `6f8ba45` | |
| S7: replay without finish | `cargo test -p easel --no-default-features --features replay --test smoke` in `--all` (its no-finishing-verbs assertions run only there) | `a37d47e` | 23 to 46 s, passes |
| S8: build-only list passes | a list needs a test step, every test step a minimum of at least one, a script step a counting regex; refused before taking the lock | `a2d7569` | test_runner case 9 |
| safeguard SHOULD-FIX (cleanup finally, stale $CT check, --no-overwrite-ignore, flagged paths, honest recovery, SAFEGUARDS.md) | helper | `f722eb0`…`0db7936` | |

Found on the way: running the painter build beside the determinism tests
replaced `target/debug/easel`, the binary those tests run (3 failed with "no
command run"). Builds of other feature sets now use their own short target
dirs (`$CARGO_TARGET_DIR/p`, `/bf`, `/ro`; `d2fac2d`, `543e305`: a long name
pushed a painter test's socket past 104 bytes). The helper found that parallel
`test_candidate` runs could lose receipts while printing "recorded"; fixed
with locked writes and a 36-run check.

## Item 1: `thin_blend_bares_ground`

The bug it names (`style.rs` `brush_ground`): a brushed ground whose paste is
"ploughed into ridges" (the hog's own push, 0.3) "piles 2x as thick at the
stroke edges: tall, sharp crests that thin paint drains off", so a thin
blended pass leaves bare ground. The test compares bare pixels over the style
ground with a reference ground of parallel strokes: style ≤ 2 × parallel + 20.

Probes (a temporary test, removed; `notes/speed/logs/p3_thin_blend_probe*.log`;
three seeds, test profile):

| ground | 3200 px (style / parallel, limit) | 2400 px | 1600 px |
|---|---|---|---|
| unchanged (push 0.15) | 4145 / 4775, limit 9570: passes | 3196 / 3057 | 496 / 522 |
| push 0.3 (the hog's own, the bug the comment names) | 4302 / 4775: **passes** | 3301 / 3057 | not run |
| push 0.9 | 7476 / 4775: **passes** | 4250 / 3057 | 716 / 522 |

The test does not catch either crest mutation: its bound (2× the reference
plus 20) only catches a regression that more than doubles bare ground. So no
smaller test can be "mutation-proven equivalent" to it on this bug, and I
didn't propose one: a tighter bound would be a new answer, which needs review
(and changing its bound is a weakening or strengthening of an approved test).
It now runs unchanged in `--all` (about 104 s, beside the other slow tests).
**Question for the user:** should the bound be tightened (for example to
1.2 × parallel) to catch the crest mutation? That is a change to an
expected answer.

## Item 2: `--all` headroom

Choice: one batch with groups of steps running at the same time, not two
batches. Two receipted batches would double the fresh build cost (each needs
the release and test builds, about 100 s) and make the receipt logic carry two
verdicts; groups cut the wall time without weakening a step. Each step keeps
its own limit and its own TMPDIR; the batch passes only if every step does.

- Group "s": the four slow tests, `thin_blend_bares_ground` and the painter
  build's tests (mostly single-threaded tests beside a compile).
- Group "w": box_features (compiling, now the dev profile: which tubes a build
  holds depends on its features, not its profile; its tests are table checks)
  beside the steps that mostly wait (git hooks in safeguards.sh, Python,
  ffmpeg) and the replay-only smoke build. lockrun.sh (timing-sensitive) and
  the easel scripts run alone.

| run | wall | |
|---|---|---|
| phase 2 candidate `c53d40a` (fresh worktree, 19 steps) | 540.9 s of 600 | before |
| local, warm, 22 steps (adds thin_blend 104 s, test_runner, replay-only smoke) | 286.7 s | `all4` |
| **candidate `8053fc1` (fresh worktree, 22 steps)** | **419.5 s of 600 (30% headroom)** | PASS |

Step times of the warm run (s): cargo-test 42.2; old-logs 5.5; baseline 3.1 +
0.8; group s: slow tests 99.3, thin_blend 103.8, painter 30.1 (failed then on
the socket path, fixed: 26.2 alone); lockrun 21.2; replay-env 4.9; check-live
21.1; group w: box-features 83.8, replay-only smoke 46.0, test-runner 36.5,
safeguards 71.0, runner 30.2, studio 8.0, the rest under 8.

## Item 3: legacy verbs past the logs' first chunks

`crates/easel/tests/old_logs/legacy_tiny.lua` (synthetic, 8 chunks, 128 px,
1.8 s) calls the legacy API the logs' later chunks use: `w:sky`, `w:clouds`,
`w:ranges` with `l:mask`, `l:haze`, `l:crest`, `haze{}`, `sk:airlight`,
`w:aerial`, `w:to_ground`, `w:scale_at`, `w:height`, `worley`, `glaze`,
`tree{}` with `:foliage`, `:mask`, `:gaps` and its limbs as ribbons, `w:proxy`
with `body.ellipsoid`, `body.block`, `:rough`, `:turn`, `:cut`,
`w:shadow_angle`, `v:shadows`, `v:land`, `v:visible`, `form{}` with
`:silhouette`, `:value`, `:field`, `:lit`, `:edges`, `:part`, `vs:cast_shadow`,
`vs:contact_shadow`, `sward{}` with its blades stroked, `dry`, `varnish`,
`relief`. Names like `frond`, `stone`, `tuft_band` are the painters' own Lua
functions (as the review notes), not easel verbs.

- Golden: af49348's unchanged release easel (sha256 `4201cec1…40f6`), option B,
  recorded with all eleven cases; the ten earlier goldens came out byte for
  byte the same (`p3_old_logs_record.log`). README and hashes in
  `crates/easel/tests/old_logs/golden/README.md`. Golden sha256
  `96f1d2ef477b2c74d8bf7c3e557d17dc83cd8be05572252d89b890de6b17ecb8`, input
  `f016f90473017ec7635e5ab389d120bb77612427d4e2ae200bd7aeac1f3968ca`.
- Mutations (release build, `p3_mutation_legacy_tiny.log`): `w:ranges`
  heights × 1.1 and `sward{}` blade height × 1.1 each fail legacy_tiny alone;
  restored, all eleven pass.
- Still not covered: the later chunks' own arguments and interactions; the
  engine-1 studios' later chunks.
- Needs an approval like the other old-log goldens (the protected directory
  covers it): two new files, `legacy_tiny.lua` and `golden/legacy_tiny.txt`,
  and changed `cases.tsv` and `golden/README.md`.

## Item 4: the thinner's frozen set

Committed to the protected list (`bb31d5b`), per the lead's later instruction
(SB4), not only proposed.

## The candidate run

`scripts/test --all --candidate 8053fc1452f74855714c90505227895e4229dc11`:
**PASS**, 22 steps, 746 counted tests, fresh full worktree under `/tmp`,
removed afterward; 419.5 s inside a 600 s lock (build 100 s). Receipt v2 in
`refs/notes/test-receipts` (local, not pushed; files in
`~/src/a/claude-paint-receipts/8053fc1…/20261004T050152-57706/`). It records
the manifest (the list `notes/speed/test_lists/all.tsv`, sha256
`68122f5c…cc1f`, and the runner `scripts/test`) with every step's fields
matched to the list, and the checkout before and after (1562 files in index
and tree, no flags, HEAD and index the commit's).

Step times (s): release build 63.8, test build 35.8, cargo-test 42.5,
old-logs 5.8, baseline 3.2 + 1.3; group s: slow tests 125.3, thin_blend
111.4, painter 32.6; lockrun 21.4; replay-env 5.7; check-live 21.5; group w:
box-features 92.5, replay-only smoke 54.3, test-runner 37.3, safeguards 86.9
(111 checks), runner 33.4 (254), studio 8.3 (23), replay-clip 5.8, box-tubes
1.4, studio-names 3.3, peek 7.7.

`scripts/golden_approve check --candidate 8053fc1 --base 3379b9f` (read
only): 29 protected changes without approval, as expected before the lead
records them (the old-log package with legacy_tiny, the r19 input,
`notes/golden_paths.txt`, both test lists, `scripts/test`,
`scripts/tests/baseline_state.sh`). The 88 baseline approvals still match.

## Open

- `thin_blend_bares_ground`'s bound (item 1): a user decision.
- S6: the comparator's added-zero stays as documented; a stricter one is a
  reviewed change to protected tools.
- A step's scratch directory (`/tmp/cpt.*`) survives when its coordinator is
  killed (SIGKILL) or stopped by the outer timeout before it can remove it:
  test_runner.sh's crash and timeout cases left six small empty ones (removed
  by hand). Harmless, but not cleaned automatically.
- SIGINT from a terminal to `test_candidate` (helper) is untested; the
  runner's SIGINT path goes through the same handler as SIGTERM (test_runner
  case 5 tests SIGTERM).
- Approvals still needed (lead): the old-log package (now with legacy_tiny and
  the r19 input), `notes/golden_paths.txt` itself, the test lists and runner,
  and the thinner's files when they arrive.
