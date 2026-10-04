# Faster tests, the test commands and the merge safeguards: the final report

This covers the "faster tests" half of the overnight plan, the job lock and
the checks that protect main. The thinner's own results are in
`notes/thinner/RESULTS.md`. Everything is on branch `engine3-overnight`
(this worktree, `~/src/a/claude-paint-engine3`): engine 3 at af49348, the
thinner (final at 995fbab) and the speed work, combined without conflicts
(`notes/speed/COMBINE.md`). Main and the website are unchanged, nothing was
painted or replayed, and the lead pushes.

## Where it stands

The final combined commit, **`0abf7929c44159475e3fcb327329c7b932477aa6`**,
was tested by `scripts/test --all --candidate` in a fresh full checkout, with
the pinned job lock:

- Verdict: **NOT ALL GREEN (known pre-existing failure: thinner check 13(b),
  user decision)**. Every other check passed. Check 13 (b) waits for your
  decision (burnt sienna versus raw sienna on the card; `notes/thinner/ACCEPTANCE.md`
  check 13). Until then no candidate can pass and the merge tool refuses it,
  as designed.
- Build phase: 3 builds in 149 s (of a 600 s limit). Check phase: 21 steps,
  785 counted tests, in 348 s (of 600).
- Receipt: `git notes --ref=test-receipts show 0abf792` (local until the lead
  pushes `refs/notes/test-receipts`), files in
  `~/src/a/claude-paint-receipts/0abf792…/20261004T071745-1604/`.

## Tests made smaller or faster

No assertion was loosened and no expected answer of an existing test changed.
Times are one test alone in the test profile, before and after.

| test | the bug it still catches | before | after |
|---|---|---|---|
| `smoke a_short_session_at_the_easel` | the easel's commands and error messages (open, do, look, check, save, run, reopen, journal). The second ground is rolled instead of brushed; brushed grounds are tested elsewhere | 58.8 s | 7.7 s |
| `delivery save_delivers_the_wet_canvas_as_seen…` | a saved picture that isn't the wet canvas as seen, or a replay that delivers different bytes. It now paints only the area it checks; delivering the dry picture instead fails it (107 levels off) | 19.7 s | 7.0 s |
| `determinism` hand time, state digests, rag | a replay that changes with the thread count. Now at 480 px (seeding brushes by thread fails all of them); the 2400 px versions run in `--all` | over 60 s, 8.0 s, 38.6 s | 6.7 s, 0.6 s, 2.9 s |
| `rag nothing_is_lifted_past_the_gel_point` | a rag lifting paint that has set. It ages the canvas in hour steps, then two-minute steps, to the same "nothing open" point | over 60 s | 9.6 s |
| the thick-swatch test (split in three) | the palette's thick swatch differing from paint laid thick | 11.3 s | 3.8 s each, side by side |
| `boxes a_round_19_log_replays_as_before` | the round 19 log no longer replaying to its picture. It moved to `--all`; a 320 px version runs in the fast checks | 26.6 s | 0 in fast (about 40 s in `--all`) |
| the whole Rust suite | | 102.5 s (`cargo test --workspace`) | 42 to 46 s (four test binaries at a time, no doctests) |

Two existing tests had bugs, which I fixed without changing what they assert:
- `replay_env.sh` had failed since round 23. Its 5-second clip couldn't be
  filled under the clip tool's 1-second hold limit; it now passes
  `--max-hold 2` and passes.
- `session_integrity rebuilding_serves_progress…` treated "the socket file
  exists" as "the server is ready". In the final full run, under load, it
  connected before the server listened (twice in two runs). It now waits for
  a connection; two full runs then passed.

## The test commands

- **`scripts/test`** (fast): builds, then the Rust tests, the old-log
  replacements, the before-change baseline (state and pictures equal to
  af49348) and the thinner's quick acceptance checks. On the final head:
  builds 91 s, checks 63 s, about **2.5 minutes** in all when the builds are
  needed; the checks alone about a minute. It ends NOT ALL GREEN (13 (b)),
  exit 4.
- **`scripts/test --all`**: everything required, in two phases of at most 600 s each
  (builds; checks). Times are above. The slow tests, the 3200 px ground
  check and the thinner's `--all` (with the 2400 px card) run side by side,
  and so do the build-feature checks and the script checks that mostly wait.
- Every step has a time limit and a minimum number of tests. A run that
  finds no tests, skips its checks or runs out of time can't pass. A
  known failure has to be named in the list with its exact exit and message.
- `notes/agent_brief_template.md` tells future builders how to use them.

## What isn't run, and why

`notes/speed/SKIPPED.md` lists every skipped test. In short:

- Seven tests that **replayed whole paintings** (engine-1 studios and six
  legacy logs) never run. The plan forbids replays. They are replaced by
  `scripts/tests/old_logs.sh`: eleven tiny cases (the first chunks of those
  logs, a synthetic engine-1 log, a synthetic log of the legacy verbs, the
  round 19 log), all compared with af49348's unchanged release build. Breaking the
  engine choice or the legacy code fails them. **Missing coverage:** the later
  chunks' own arguments and interactions, and the engine-1 studios' later
  chunks.
- Twelve diagnostics and experiments stay ignored (probes, timings, tables).
- Four documentation examples aren't compiled (they're snippets).
- The thinner's acceptance tests run through their own release runner, not
  the regular Rust step.
- Not run anywhere: the studio export scripts that need a round branch and
  painter builds, and the runner tests of rounds 17 to 23 (round 24's run).

## Script and build-setting checks

All pass in `--all`: the painter build for one box
(`--no-default-features --features box-inness`, 14 tests; it didn't even
compile before), every box combination (`box_features.sh`), the replay build
without the finishing verbs (the smoke test), `replay_env.sh`,
`check_live.sh`, `replay_clip.sh`, `box_tubes.sh`, `studio_names.sh`,
`peek.sh`, the round 24 runner tests (254) and the studio viewer tests (23,
none skipped).

## Small full-resolution checks

All of these passed in the final run. "Full resolution" means the painting's
own pixel density (2400 px across, or more), on a small window or a few strokes.

- **Laying paint:** the thinner's 2400 px card, the fixed target. Thinned raw
  sienna keeps 84.4% of the card's contrast at load 0.3 and 81.4% at load
  0.6, where at least 50% was required (thinner RESULTS). Also the 3200 px
  pointed-hatch check in `crates/paint/src/tests.rs` and the 3200 px ground
  check (below).
- **Rag lifting:** the rag tests at 2400 px in a crop window: lifting from the
  tops first, the pale tint of a dry rag, spirits lifting nearly to the
  ground, nothing lifted past the gel point. The thinner's rag study (its
  picture `notes/thinner/rag_study.jpg`) was reviewed separately.
- **Drying:** checked across resolutions rather than at 2400 px.
  `stages_dont_depend_on_resolution` compares 400 and 1200 px of the same
  physical film. The other drying tests (against the sources' ranges, fast
  and slow pigments, thick films, split waits) use small canvases with real
  physical sizes.
- **Solvent evaporation:** the thinner's checks 9 and 17 and the card's wait
  (at least ten evaporation times, under one-thousandth of the solvent left).
- **The 3200 px ground check** (`thin_blend_bares_ground`) runs in `--all`
  and passes. See the open items.

## Job lock, saved answers and merge safeguards

- **`scripts/lockrun`** allows one heavy job at a time on the machine; the
  lead's approved copy is `~/src/a/claude-paint-tools/lockrun`. 17 checks
  cover crashes, timeouts, cancellation, nesting, stale tokens and daemons;
  all pass. Every heavy job tonight ran through it.
- **`scripts/test`'s own process handling:** 17 checks
  (`scripts/tests/test_runner.sh`), all pass. They cover timeouts, cancels, a
  killed coordinator, leftover and detached processes (the easel's session
  server), nesting inside a batch, known failures and the two phases.
- **Saved answers** (`notes/golden_paths.txt`): the baseline, the old-log
  answers and inputs, the thinner's frozen tests, the test lists, the runner,
  the gate tools and the lock. A change to any of them needs an approval
  recorded by someone other than its builder (`scripts/golden_approve`).
- **The merge gate** (`test_candidate`, `merge_candidate`): 140 checks on
  dummy repositories, all pass. Four review rounds; round 4 approved them,
  and 11 of 12 deliberate breakages were caught (the 12th couldn't be made).
  A receipt binds the exact commit, list, runner, lock and checkout; the
  merge refuses anything but a pass, refuses if main moved, and updates the
  published copy's files too. **Tested only on dummy repositories**; it
  has never touched the real main.
- **Approval status** (`golden_approve check --candidate 0abf792 --base 3379b9f`):
  147 protected changes, 137 approved, **10 waiting for the lead**:
  `crates/paint/src/state_dump.rs`, `notes/golden_paths.txt`, both test
  lists, `scripts/test`, `scripts/test_candidate`, `scripts/merge_candidate`,
  `scripts/safeguards_lib.py`, `scripts/tests/safeguards.sh` and
  `scripts/tests/test_runner.sh`.

## Open items for you

1. **Check 13 (b)**, the thinner's: burnt sienna's transparency against the
   period source. Until it is decided, nothing can pass the gate.
2. **`thin_blend_bares_ground`'s bound.** It allows twice the reference's
   bare ground, plus 20 pixels. It didn't catch the brushed-ground crest bug its own
   comment describes (raising the brush's push to 0.3 or 0.9 still passed:
   7476 bare pixels against a limit of 9570). Tightening it is a change to an
   approved answer, so it is your call.
3. **The new gate copy.** `~/src/a/claude-paint-tools/gate-next/` (two
   phases, known failures) was reviewed and approved in round 4 but not yet
   promoted: the approved `gate/` copy can't test this branch (it doesn't
   read the new list format). Promote it after the lead records the 10
   approvals.
4. **Linux is untested.** Everything ran on macOS. The lock and the runner
   use portable calls (and `/proc` for environments on Linux), but nothing
   was run there.
5. **Smaller things.** A test that kills the runner's coordinator leaves an
   empty `/tmp/cpt.*` directory. The existing `frames.rs` test leaves an empty
   `out/test/`. The legacy logs' later chunks aren't covered (above).
