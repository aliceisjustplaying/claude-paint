# Engine 3: handover 2, October 4, 2026 (evening)

Follows HANDOVER.md section 6. Work is on branch `rag` (local, not pushed),
11 commits on `main` (`cad5db0d`). Nothing here has been merged.

## Technical scope
  
  Behavior-changing defaults require review of the exact candidate.
  The deposition experiment is finite-rate exchange between finite brush
  and canvas reservoirs, specified in AGENT_BRIEF_V2.md section 4c and
  REVIEW_RESPONSE.md section 5.
  
  ## Health

- `scripts/test --all` at `6f31f790`: red only on the three known steps
  (check 13 (b); the `rag` scene of `baseline-state` and thinner check 2).
- `scripts/test_thinner_acceptance --quick` at `b72c3978`: 21 of 22, only
  13 (b) (expected) and check 2 (the `rag` scene) not passed. The full
  `scripts/test --all` has not been run after `6f31f790`.
- sccache: start it from a normal shell with `SCCACHE_IDLE_TIMEOUT=0`; the
  server quit while idle once and the next build hit the trap in
  HANDOVER 5.0.

## Rag (HANDOVER 6.1): done, chosen by the owner

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
- (3) Thin films: `b72c3978` replaced the 2 µm floor with Orchard's h³
  slowing, mobility × h³ / (h³ + `WET_FILM_UM`³). One stroke at thinner
  0.75 now flows 0.87% in 5 minutes (was 0), 0.9 flows 0.036%
  (`thinner::tests::single_strokes`). Visual difference small (mean 0.35,
  max 10 grey levels; `notes/thinner/wash-experiment/flow.jpg`). Committed
  pending visual acceptance; selection or rollback remains open.
- (2) A pass over a wet wash: not done. `0acf8cfe` holds a test-only
  switch (`thinner::set_wet_rule`, `Surf::used` in `bristle.rs`; default
  0 = today) with strict and soft stand-ins and pictures
  (`notes/thinner/wash-experiment/index.html`). These candidates are not selected. Next: build the finite-rate exchange of
  `AGENT_BRIEF_V2.md` §4c, compare it against today's rule on the same
  fixtures, show the owner, then restate protected checks 7, 9, 14, 15 and
  16 with approval. Remove the switch when done.

## Then

- Sienna 13 (b) (HANDOVER 6.3), unchanged.
- A real painter: `notes/round24/runner/` is the Inness lane on engine 3,
  never launched. Needs a tag on the final commit, a detached checkout at
  `~/src/a/claude-paint-r24run`, `BRANCH`/`BASE`/`H` in its
  `r21_chains.py` (lines 146-155; lane thinking is `xhigh`, line 367), a
  read of the guide's "The rag" against the new rag, and a launch with the
  studio viewer on the tailnet. The owner wants this once 6.2 (2) is done.
- `description/` waits until after engine 3.

## Probes and switches added (all `#[ignore]`)

`rag::tests::thin::table`; `thinner::tests::{short_waits, wait_cost,
single_strokes, wet_rules, flow_pictures}`. Sheet scripts:
`notes/rag/thin-experiment/*.py`, `notes/thinner/wash-experiment/sheets.py`
(`uv run`).
