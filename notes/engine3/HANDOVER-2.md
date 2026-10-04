# Engine 3: handover 2, October 4, 2026 (evening)

Historical status on branch `rag`. HANDOVER-3.md is the current parallel
plan and done condition; its state supersedes this document. The selected
rag settings below remain reference material.

## Goal and done condition

The current milestone is **the tools ready for an Inness painting**.
Then run the painting, inspect how it turns out and use that evidence to
choose what needs tweaking. This is not a requirement to finish every
engine-3 release task or perfect the simulation before a painter uses it.

Ready means the selected rag and completed thinner work run together,
relevant correctness checks and a short save/reopen/replay smoke sequence
pass without unexplained failures, and the Inness runner and viewer are
configured for the exact candidate. Known limitations must be explicit;
they are not automatically launch blockers. The scope is the specified
mechanics and the checks needed for painting readiness.

## Remaining route to the painting

Use `reviews/engine3-review-response/AGENT_BRIEF_V2.md` as the technical
scope, retaining the selected appearance rather than restarting its search.

1. Resolve the preserved uncommitted thinner work below. The strict/soft
   experiment is already removed.
2. Complete thinner flow and finite-supply brush/canvas exchange (§4).
   Verify elapsed-time and mixed-field behavior; the historical timing and
   substep claims below are not proof that all review checks are satisfied.
   Show the exact behavior-changing candidate before selecting defaults.
3. Finish rag path consistency (§3) and check bounded pickup, dirty
   carryover and refolding (§2) on the integrated candidate. Fix demonstrated
   correctness defects without reopening a general cloth redesign.
4. Address the documented sienna requirement (§5), with approval for any
   protected changes. Keep pigment constants unchanged initially.
5. Run relevant checks and a short connected tool/save/reopen/replay
   sequence (§6). Distinguish known baseline/approval issues from actual
   tool failures; do not silently waive either or make the whole release
   process a prerequisite for an experimental painting.
6. Freeze the candidate, configure the existing round-24 Inness runner and
   tailnet viewer, then launch. Judge further appearance work from the
   painting.

Full release closure and the product-description refresh remain separate
from this painting-readiness milestone (brief §6).

## Current scope and starting point

- Selected rag appearance: `6f31f790`; retain these choices.
- Thin-film flow experiment `b72c3978` was reverted in `f69b7967`.
  The 2 µm floor is back; its replacement remains open.
- The strict/soft deposition code, probe and comparison choices from
  `0acf8cfe` have been removed. The separate historical flow comparison
  and `flow_pictures` probe remain. Default deposition is unchanged.
- Next deposition work: finite-rate exchange between finite brush and
  canvas reservoirs, per `AGENT_BRIEF_V2.md` §4c and
  `REVIEW_RESPONSE.md` §5 under `notes/engine3/reviews/engine3-review-response/`.
- Behavior-changing defaults require review of the exact candidate.

### Existing uncommitted work (preserved, not part of this cleanup)

`crates/paint/src/thinner.rs` already contained removal of `FLOW_MIN`
and its early-return check, plus an ignored `fingerprint` diagnostic.
These edits remain uncommitted and unvalidated here. Establish their
status before further thinner edits; timings below predate their removal
of the cutoff.

## Health

- `scripts/test --all` at `6f31f790`: red only on the three known steps
  (check 13 (b); the `rag` scene of `baseline-state` and thinner check 2).
- `scripts/test_thinner_acceptance --quick` at `b72c3978`: 21 of 22, only
  13 (b) (expected) and check 2 (the `rag` scene) not passed. The full
  `scripts/test --all` has not been run after `6f31f790`.
- sccache: start it from a normal shell with `SCCACHE_IDLE_TIMEOUT=0`; the
  server quit while idle once and the next build hit the trap in
  HANDOVER 5.0.

## Rag (HANDOVER 6.1): appearance selected; correctness work remains

`6f31f790`, engine 3 only (`crates/paint/src/rag.rs`):

- No fixed stain. The last of a film comes away more and more slowly:
  take × v / (v + h), h = `SLOW_COATS` (1 µm) × (2 − reach share),
  shrunk by spirits (`SLOW_DAMP`). Owner's pick "version 4".
- Spirits work gradually: `DAMP_LIFT3` 1, `DAMP_REACH` 0.3, `DAMP_WICK`
  0.3. One damp wipe of a thin wash takes about 80%, three about 93%.
  Owner's pick "B (mild)".
- The pad presses less toward its rim (`RIM_PRESS` 0.3, outline fade
  `RIM_EDGE` 0.85), so the edge shows the weave instead of a haze.
  Owner's pick, round 3.
- Spirits running into the paint beside a damp wipe: tried, invisible
  under the old flow stepping, not kept.
- Two unprotected tests in `rag.rs` restated (no fixed 0.5 µm stain;
  spirits clear 90% instead of 95% over three passes).
- Record: `notes/rag/thin-experiment/` (pages `first.html`,
  `round2.html`, `round3.html`, `index.html` = round 4, `results.txt`).

Pending: the `rag` baseline scene (`notes/thinner/baseline/`, protected)
needs rebaselining and the owner's approval. HANDOVER 6.1 (5), the
vertex-count dependence (`rag.rs`, the per-segment `ceil` in `rag_wipe`),
is not done. 6.1 (2)-(4) were not started.

## Thinner (HANDOVER 6.2)

- (1) Elapsed time: done (`84740a0c`, `a05c94a2`). Evaporation and flow
  step on a grid of `FLOW_TICKS` = 64 a minute counted from the canvas's
  start; drying keeps its whole-minute steps. A 0.02-minute wait now
  moves 0.0064-0.0107% of the paint from any phase, against 0% or 0.49%
  before (`thinner::tests::short_waits`). Cost: `wait(30)` after a thinned
  patch at 2400 px 10.2 s against 0.79 s (`thinner::tests::wait_cost`).
  The owner chose 64 ticks over 16 (2.85 s) knowing the cost; optimize if
  painters thin a lot.
- Protected check 16's solvent balance loosened from 1e-4 to 5e-4 by the
  owner's decision (`ddc154f3`, recorded in `ACCEPTANCE.md` "Decisions"
  and at the assertion; may be revisited). Approval of
  `crates/paint/tests/thinner_physics.rs` and `notes/thinner/ACCEPTANCE.md`
  is not recorded yet.
- (4) The substep cap: no longer binds at 64 ticks (a tick needs a few
  substeps); not otherwise changed.
- (3) Thin films: the experimental slowing law from `b72c3978` was
  reverted in `f69b7967`. The hard 2 µm floor is current again. A reviewed
  replacement is still needed. The historical flow images remain in
  `notes/thinner/wash-experiment/`; they do not describe current behavior.
- (2) A pass over a wet wash: not done. The strict/soft experiment from
  `0acf8cfe` has been removed without changing default deposition. Next:
  implement the finite-rate exchange experiment in `AGENT_BRIEF_V2.md`
  §4c, compare against the required controls and show the exact candidate
  before selecting default behavior. Protected expectation changes need
  approval; do not assume every existing check needs rewriting.

## Then

- Sienna 13 (b) (HANDOVER 6.3), unchanged.
- A real painter: `notes/round24/runner/` is the Inness lane on engine 3,
  never launched. Needs a tag on the final commit, a detached checkout at
  `~/src/a/claude-paint-r24run`, `BRANCH`/`BASE`/`H` in its
  `r21_chains.py` (lines 146-155; lane thinking is `xhigh`, line 367), a
  read of the guide's "The rag" against the new rag, and a launch with the
  studio viewer on the tailnet. Launch follows the painting-readiness condition above.
- `description/` waits until after engine 3.

## Retained diagnostic probes (all `#[ignore]`)

`rag::tests::thin::table`; `thinner::tests::{short_waits, wait_cost,
single_strokes, flow_pictures}`. Sheet scripts:
`notes/rag/thin-experiment/*.py`, `notes/thinner/wash-experiment/sheets.py`
(`uv run`).
