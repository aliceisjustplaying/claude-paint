# History of the engine-3 work

Working notes from October 3 and 4, 2026, kept for the record. They were
written outside the repository; home paths are shortened to `~`. The current
state is in `../HANDOVER.md` and `../../speed/FINAL_REPORT.md`.

- `overnight-plan.md`, `overnight-decisions.md`, `overnight-log.md`,
  `overnight-report.md`: the overnight run (thinner, rag check, faster tests).
- `heavy-turn.md`: the retired heavy-job turn note (lockrun approval).
- `round24-review.md`, `round24-adversarial-review.md`, `round24-open-questions.md`: round 24 reviews.
- `overnight-speed-brief.md`, `thinner-spec.md`: stubs pointing to the plan.
- `thinner-v1-branch.patch`: the abandoned first thinner, branch `thinner-v1`
  (`d3f7144c`, `94712a42`) as `git format-patch` from `3d0ea660`; `git am`
  on that commit reproduces the branch exactly. The branch was deleted.
- `stash-thinner-v1.patch`: applies on top of `thinner-v1-branch.patch`; stash of the abandoned first thinner (raw sienna
  hiding 0.2 in engine 3; not adopted: check 13 (b) keeps pigment values).
- `stash-drying-all.patch`: an earlier draft of engine-3 drying (one global
  model); the `drying` branch's per-tube rates replaced it.
- `wt-exp-uncommitted.patch`: `--state-digest` instrumentation, later merged
  as `feat/state-digest`.
