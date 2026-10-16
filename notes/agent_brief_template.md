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

- **The gate:** `scripts/test`, about 4 minutes from an empty target on
  4 vCPUs, less once built. CI runs the same command on every pull request.
- A check passes only if it exits 0 and shows its tests ran: a test name that
  matches nothing is a failure, and a skipped test is not a passed test.
- `notes/speed/SKIPPED.md` says what the gate runs and what it doesn't. Run
  an ignored test only by its exact name
  (`cargo test ... -- --ignored --exact <name>`); never run all ignored tests.

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

Run one build, test run or other multi-second CPU job at a time. Don't
retry the same slow command: investigate with a smaller case. Start long
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

- `scripts/test` passes, or every failing check is in your report with its
  cause.
- Your report lists commits, commands with their results and times, what
  you could not do, and questions for the user.
- Temporary files, worktrees and build directories you made are gone (see
  `~/.pi/agent/AGENTS.md`, disk hygiene).
