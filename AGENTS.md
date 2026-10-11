# Development checks

Run `scripts/test-changed` after each fix round. It covers committed changes
against `origin/main`, staged and unstaged edits and untracked files. Use
`scripts/test-changed --dry-run` to inspect the selection, or pass explicit
repository-relative paths to check just the files changed in the current round.
Unknown code paths and build configuration changes use the broad iteration tier.
The ignored live-width checks and full thinner card stay in the final gate.

Run `scripts/test --all` once on the final head before reporting ready. Run it
alongside independent review, then use targeted checks for any review fixes.
If a review fix changes code, the final head still needs its own full-gate pass.
The iteration tier is never a candidate receipt or evidence that the full gate
passed. See `notes/testing.md` for the tiers and coverage inventory.

On the live-stream Mac, run heavy checks with `nice -n 15`,
`RAYON_NUM_THREADS=2`, `RUST_TEST_THREADS=2` and `CARGO_BUILD_JOBS=2`.
Keep build output in this worktree or a private target directory. Never use a
live painter's target directory or studio. The test runner retains the shared
heavy-job lock and cleans up only its own test processes.

Before each build or test on the live-stream Mac, check `/bin/df -h /`. If less
than 15 GB is free while a painter is running, stop and tell the Captain. Use one isolated target directory
for this job, such as the worktree's `target/` or a private scratch directory.
Different branches must not share a target directory: they can overwrite the
CLI between build and test. The compiler cache can share compiled dependencies.
Keep private scratch outputs below about 4 GB and remove your own intermediate
build outputs when done. Never delete another job's cache, worktree or studio.
