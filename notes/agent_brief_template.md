# Brief template for a builder agent

Copy this into a builder's brief and fill in the bracketed parts. The rules
below hold for every builder, whatever the task.

## The task

- Goal: [what to build or fix, and how we will know it works]
- Working copy: [`~/src/a/claude-paint-<name>`, branch `<branch>`, from commit `<sha>`].
  Work only there. Never touch `~/src/a/claude-paint` (main and the live
  site) or another agent's worktree. No commits to main, no tags, no push
  unless the lead says so.
- Report: save [`notes/<area>/REPORT.md`] and commit it before announcing
  that you are done.

## Running tests

- **Fast checks:** `scripts/test`. It builds what it needs and runs
  `notes/speed/test_lists/fast.tsv`, about 75 s once built (build about
  20 s on top; each test step's limit is 60 s). Use it after every change.
- **All required checks:** `scripts/test --all`
  (`notes/speed/test_lists/all.tsv`): the fast checks, the slow tests by
  exact name, the scripts' tests, the painter build and the Python tests.
  Run it before you hand work over.
- **An exact commit, for the record:** `scripts/test --all --candidate <sha>`
  tests that commit in a fresh clean worktree and records a receipt in
  `refs/notes/test-receipts` (`notes/speed/SAFEGUARDS.md`).
- A step passes only if its command succeeds within its time limit and runs
  at least its listed number of tests. A test name that matches nothing is a
  failure, and a skipped test is not a passed test.
- `notes/speed/SKIPPED.md` lists every ignored test and why. Run a slow test
  only by its exact name (`cargo test ... -- --ignored --exact <name>`);
  never run all ignored tests.

## The saved comparison results

- `notes/thinner/baseline/` holds small before-change results from the
  unchanged engine-3 code (af49348): six tiny scenes with their full state
  after every chunk. `scripts/tests/baseline_state.sh` (in `scripts/test`)
  checks the engine against them field by field, with `--added-zero`: an
  added numeric field must be all zero, and an added value inside the
  brushes', rags' or studio's text must be 0, `false`, `None` or empty. Not
  checked: a whole added text field, and the length of an added list (its
  elements are). The tools are protected; a stricter check is a reviewed
  change.
- `crates/easel/tests/old_logs/` holds tiny old-log cases with goldens from
  the same code (`scripts/tests/old_logs.sh`).
- Expected answers are never computed by the code under test. A new or
  changed expected number is either computed by the unchanged af49348
  release build (commands and hashes recorded) or waits for the user. The
  protected files are listed in `notes/golden_paths.txt`. A change to them
  needs an approval recorded by someone other than its builder
  (`scripts/golden_approve`). Passing tests are not permission to change
  their answers.

## One heavy job at a time

Every build, test run or other multi-second CPU job goes through
`scripts/lockrun`, which allows one such job machine-wide:

    scripts/lockrun --timeout 60 -- <command>      ordinary checks, once built
    scripts/lockrun --timeout 600 -- <command>     builds and batches

`scripts/test` takes the lock itself. A job that runs out of time is
stopped, its log is kept and it counts as unfinished. Don't extend the limit
or retry the same slow command: investigate with a smaller case. Start long
commands in the background (`job_run`) and let their completion wake you.
Don't wait in sleep or polling loops.

## Never replay paintings

Don't replay a whole painting, and don't run expensive replays, not even
inside a test, a timing run or a benchmark. Read a test or script before
running it. If a check needs a painting's behavior, use a tiny scene: a few
chunks at 64 to 320 px, or the first one to three chunks of a log.

## Privacy and Git

Keep the privacy hooks on; never use `--no-verify`. Don't write the user's
real name, private email or home paths into committed files, logs or
messages (use `~` or `$TMPDIR`). Commit in small logical commits and never
rewrite history you didn't create in this task.

## Before you finish

- `scripts/test --all` passes, or every failing step is in your report with
  its cause.
- Your report lists commits, commands with their results and times, what
  you could not do, and questions for the user.
- Temporary files, worktrees and build directories you made are gone (see
  `~/.pi/agent/AGENTS.md`, disk hygiene).
