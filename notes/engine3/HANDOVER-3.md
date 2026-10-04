# Engine 3: parallel route to the first Inness painting

Current plan on branch `rag`. This supersedes the sequential route and
status in HANDOVER-2.md; that file retains the selected rag settings and
historical measurements. Technical requirements come from
[AGENT_BRIEF_V2.md](reviews/engine3-review-response/AGENT_BRIEF_V2.md) and
[REVIEW_RESPONSE.md](reviews/engine3-review-response/REVIEW_RESPONSE.md).

## Goal and done condition

**Tools ready for one Inness painting. Then launch it and use the painting
to decide what needs tweaking.** This is an experimental painting milestone,
not a full engine-3 release or a demand for calibrated real-world physics.

Painting-ready means all of the following hold on one frozen candidate:

- The selected rag appearance, finite-supply brush/canvas exchange and
  reviewed thin-film flow replacement work together. Required visual
  selections and approvals for changed protected expectations are recorded.
- Relevant checks cover material accounting, path consistency, wet/dry
  buildup, elapsed-time flow and preservation of engines 1/2. No unexplained
  failure or unresolved defect that invalidates these comparisons remains.
- A short sequence works: underpainting → wiped lights → dirty carryover →
  refold → thin/body overpaint → wait → save/reopen → replay. Its images
  and state results come from that exact candidate.
- The existing Inness runner, guide and private tailnet viewer are ready
  for that candidate. Runtime: Opus 5.5, high thinking, as requested.

Once these conditions hold, freeze/tag the candidate and launch. Confirm
that the painter has successfully used the easel and provide the reachable
viewer URL. Do not continue isolated aesthetic tuning before seeing the
painting. If blocked, name the specific unmet condition and evidence.

Known baseline differences and incomplete release paperwork are not
silently treated as passes, but are not automatically painting blockers.
Distinguish them from actual tool failures. Full release closure, PR #3,
the product-description refresh and general cloth/solver redesign are
outside this milestone (brief §6).

## Parallel work and dependencies

Start the independent lanes below together when execution capacity and
agent permissions allow. This plan does not override a no-subagents rule.
A single agent can prepare independent work while background checks run.
Do not create extra review or coordination layers.

One integration owner preserves the existing uncommitted experiment,
records the shared starting revision and owns combined changes to
`notes/thinner/ACCEPTANCE.md`, runner manifests and protected baselines.
Use isolated branches/worktrees for concurrent edits, especially A and B,
which both touch `thinner.rs`. Transfer the existing experiment explicitly;
a worktree from HEAD alone does not contain it. Experimental checkpoints
must remain default-off and must not imply visual acceptance.

| Lane | Start now; owned work | Handoff / dependency |
| --- | --- | --- |
| **A — Brush exchange** | `bristle.rs`, exchange probes in `thinner.rs`, exchange comparisons. Correct the measurement limitations below before evaluating k/j. Implement finite-rate, finite-supply exchange with no fixed target layer; compare the review's baseline/(b)/(B) controls. Cover brush depletion, wet/dry buildup, linen/smooth grounds, uninterrupted/partitioned motion, lifts and reloads (§4c). | Reliable candidate, images, material accounting and path results. Coordinate any shared `drying.rs` edits with B. Final combined comparison waits for B; initial experiments do not. |
| **B — Thin-film flow** | Flow code/probes in `thinner.rs` and timing in `drying.rs`, isolated from A. Replace the hard 2 µm floor with the reviewed bounded slowing/retention approach; label estimates. Use direct films and current brush strokes. Verify short waits from different clock phases, split waits, save/reload and the mixed-field substep case (§4a–b). | Candidate with transport, conservation and timing results. Direct-film work needs no exchange candidate; rerun brush-dependent checks after A integrates. No general solver rewrite. |
| **C — Rag correctness** | `rag.rs` and rag diagnostics. Retain the selected appearance from `6f31f790`. Fix path sampling; check reversal/corners, bounded pickup/return, dry underlayer protection, dirty carryover, refolding and blot behavior (§2–3). | Focused checks and images on controlled films. Final brush-made sequence waits for A+B, not the path fix. Change only mechanisms needed for demonstrated defects or the stated contracts. |
| **D — Sienna** | `thinner_pigments.rs` and direct optical fixtures. Prepare the contrast-ratio requirement for 13(b), retaining the old diagnostic and leaving pigment constants and 13(a) unchanged (§5). | Approval-ready protected changes and direct-film results can finish independently. Final brush-made card waits for A+B. Coordinate acceptance-text/test-runner edits through the integration owner. |
| **E — Painter setup** | `notes/round24/runner/`, guide check and viewer preparation. Reuse the existing Inness lane; prepare runtime/path settings and private tailnet access now. No PR #3 integration or runner redesign. | Setup is ready with final revision/tag explicitly pending. Export/build the final studio and verify behavior-specific guide text after integration; launch after the readiness checks. |

Dependencies: A+B → final thinner/brush comparisons; A+B+C → connected
painting-tool smoke sequence; A+B+D → final sienna card. E proceeds
throughout. Integration and the final checks wait for the required lane
results, not for unrelated release work.

Each lane returns its patch, focused check results, candidate images where
needed and specific limitations. Routine engineering choices belong to the
implementer; scope changes and visual/protected approvals go to the owner
with a recommendation. No new approval checkpoint for routine coding.

### Integration and launch

Integrate completed independent patches as they become ready. Resolve A/B's
shared-file changes explicitly; do not replace one lane's file wholesale.
Run focused checks during development and `scripts/test --all` on the final
combined candidate. Classify every failure. Regenerate only approved
baselines; changing an expectation to obtain a pass is not a fix.

Builds/tests use `~/src/a/claude-paint-tools/lockrun` (`scripts/test` already
locks). Parallel implementation does not mean overlapping heavy builds or
editing a checkout during its test run. Remove completed worktrees.

After the connected sequence and final brush-dependent checks, bind E's
runner to the frozen candidate and launch. Further tuning follows painting
evidence.

## Recorded baseline status

- `f6411630`: `FLOW_MIN` cutoff removed (owner's call); check 9's per-pixel
  bound loosened 1e-4 → 2e-4 + 2δ (owner's decision, in ACCEPTANCE.md
  "Decisions", hash updated). `--quick`: 21/22, only 13 (b) (expected) and
  check 2 `rag` scene (pending rebaseline). `wait(30)` at 2400 px: 21.8 s
  (was 9.3 s).
- PR #3 (flakypuff, impasto/knife/lit looks): reviewed, deferred until after
  the Inness painting by the owner. Doesn't touch thinner.rs or rag.rs; merges
  into `rag` without textual conflicts.

## In progress (uncommitted, default behavior unchanged)

Finite-supply exchange experiment (AGENT_BRIEF_V2 §4c), owner-approved
direction: **no fixed layer / no ceiling**. Film thickness should come from
brush supply and contact, not a target height.

- `thinner.rs`: thread-local switches `set_exchange(bool)` and
  `set_exchange_kj(k, j)`. Ignored probes: `exchange_scenes` (env
  `EXCH_DIR`, `EXCH_SETS="today;k,j;..."`, `EXCH_LINEN_ONLY`),
  `exchange_side`, `c09_probe`, `fingerprint`.
- `bristle.rs`, when exchange is on for a thinned hair: no per-stroke
  ceiling, pickup without the per-stroke floor/own-stroke discount, release
  rate × (1−t)^k, thinned load × (1−t)^j (also in the crop-window ghost path).
- Round 1 (k=j=0): far too heavy. One stroke 23.9 µm vs 1.6 today; brush
  dumps 62%. Sheet: `notes/thinner/wash-experiment/exchange-r1.png`.
- Planned comparison: **both** mechanisms (k and j). First correct the
  measurement limitations below. The grid run
  `EXCH_LINEN_ONLY=1 EXCH_SETS="today;2,0;4,0;0,1;0,2;2,1;4,1"` was started
  but aborted before output. After the measurement corrections, rerun
  and build a sheet (crop units x 200-800, y 200-500). Review the exact
  candidate before selecting defaults. Temp scripts are not durable inputs.

## Lane A: measurement limitations before tuning

- **Dry-pass buildup:** `exchange_scenes` currently measures only
  `wet.vol`. It omits earlier coats transferred to `film` by drying.
  Measure total deposited paint above the original ground before using
  the wet/dry-pass table to assess buildup.
- **Ground response:** the initial full-load/high-pressure scene gives
  similar linen and smooth results. The follow-up pressure/load sweep in
  `exchange_side` runs before enabling exchange, so its differences
  describe the baseline. Repeat with exchange explicitly enabled before
  drawing conclusions about the candidate's response to the ground.
- **Path consistency:** two drag calls differ from one downstream of the
  cut (largest pixel difference: baseline 4.9 µm, exchange 36 µm).
  Restarting bristle wander is a candidate explanation, not validation of
  equivalent motion. Even within one call, 2 versus 51 points differs by
  0.045 µm in the baseline and 3.4 µm with exchange. Resolve the discrepancy
  or establish an explicit, justified acceptance criterion before tuning.
- **Coupled load effects:** the smaller-load setting does not isolate
  deposition. Lower fullness also raises pickup through `hunger` and
  changes contact with the weave through `wet`/`wick` in `bristle.rs`.
  Report these coupled effects rather than attributing the result solely
  to less paint being laid. The chosen exponents are estimates, not
  calibrated physical constants.

The immediate task is to make these comparisons reliable, then evaluate
k/j candidates. Preserve the experimental default-off boundary.
