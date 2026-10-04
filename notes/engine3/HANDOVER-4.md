# Engine 3: handover 4, October 4, 2026 (~20:50 UTC)

**Superseded status:** the lane work is consolidated and the old agents
are stopped. Current state and the specific verification blocker are in
[the launch note](../round24/LAUNCH.md#current-state-october-4-2026-2200-bst-onward).
The owner subsequently authorized finishing rag carryover and concurrent
checks, with all development and launch work on the M3.

Status of the parallel plan in [HANDOVER-3.md](HANDOVER-3.md), which still
defines the goal and done condition (tools ready for one Inness painting,
then launch). Branch `rag`. This file records where every lane stands, the
owner's decisions taken during the run, and what remains.

## Where things are

- `rag` at `a98d8ca` (local; not pushed; approval notes in
  `refs/notes/golden-approvals` not pushed either).
- Shared starting revision for all lanes: `d54b423` (the exchange
  experiment checkpoint, default off).
- Lane worktrees (each on branch `e3/<lane>` from `d54b423`):
  `~/src/a/claude-paint-wt/{a-exchange,b-flow,c-rag}`. D and E are merged
  and their worktrees removed.
- Lane rules given to every subagent: `~/src/a/claude-paint-wt/LANE_RULES.md`
  (outside the repo).
- Heads-up notes collected while the you-should-know relay is broken:
  `~/src/a/claude-paint-wt/YSK_ISSUES.md`. Source log:
  `~/.pi/agent/you-should-know/checks.jsonl` (one-line summaries only).
- Leftover worktrees not made by the integration owner:
  `$TMPDIR/tmp.izVlOmdwv0/old` (84740a0) predates this run; lane B's
  `$TMPDIR/laneb/wt-old` (73b8129) and `wt-prev` (88bfb7e) are B's scratch
  comparison checkouts. Remove them (`git worktree remove`) once B is done.

## Owner decisions taken in this run

1. **Sienna 13 (b) restated** (approved): lower RGB luminance contrast
   ratio at equal direct films; the old absolute-difference quantity kept
   as a diagnostic. Applied in `6307ad8`, approval recorded with
   `scripts/golden_approve record --approver user --builder sienna`;
   `golden_approve check --base d54b423` passes.
2. **Guide wording for the rag** (approved): `notes/easel_guide.md` "The
   rag" as merged in `d6a7669`.
3. **Thin-film flow** (approved, conditional): lane B's bounded h³ slowing
   becomes the engine-3 default **once check 17 passes** with nothing else
   broken in `--quick`.
4. **Rag behavior is frozen for this milestone.** Cross-wipe dirty
   carryover (a rag holding the color it picked up into the next wipe) is a
   real gap (below) but must not be implemented now. Lane C was told to
   remove it and report it as a finding only.
5. **Lane A wrap-up**: no more experiments. Pick the most sensible measured
   setting that keeps checks 7 and 9 passing, keeps one thinned stroke
   thin and agrees at 1200/2400 px; otherwise pick "today" (exchange off).
6. Painter runtime: Opus 5.5, thinking high (set in the runner).

## Lanes

### D — sienna: DONE, merged (`da5c56b`, `6307ad8`)

Evidence and approval note: `notes/thinner/sienna-13b/`. With the patch,
`--quick` was 23/24, the only failure check 2's `rag` scene (known pending
rebaseline). Still owed: regenerate the brush-made sienna card on the final
A+B candidate (`scripts/test_thinner_acceptance --card`).

### E — painter setup: DONE, merged (`0319909`, `2bb7057`, merge `d6a7669`)

- `notes/round24/runner/r21_chains.py`: thinking `high`; `TAG = "round-24"`
  placeholder; `BASE` = `~/src/a/claude-paint-r24run`, `H` =
  `BASE/harness/painter`; a real run refuses unless `BASE` is a clean
  checkout of the tag. Runner tests 255 passed (lane E's run).
- `notes/round24/LAUNCH.md`: freeze and launch steps 1-8. Step 6 (added by
  the integration owner): stop `art.stillwet.studio.sync` before launch, add
  the new studio to the `--skip` list in `~/src/a/stillwet/sync-studio.sh`,
  verify with a trial export, then restart the sync. Without this the
  painter is published to the public stillwet site.
- Tailnet viewer `http://m3p.tailec2dc.ts.net:8765/` returned 200 (lane E
  restarted the LaunchAgent with bootout/bootstrap).
- Guide statements to re-verify after integration are listed in LAUNCH.md
  (thinner ceiling lines 126-131, flow 132-138, 145-147, brushes 169-171,
  blending numbers line 203, rag 357-379, drying line 490).

### B — thin-film flow: IN PROGRESS (fixing check 17)

- Committed: `73b8129` (diagnostics, no behavior change), `88bfb7e` (the
  candidate, marked unapproved; owner has since approved it conditionally).
- Candidate: mobility × h³/(h³+2³) replaces the hard 2 µm donor floor
  (`THIN_FILM_UM` 2 µm, ESTIMATE); `spread` recomputes its stability bound
  every substep; `wait_on_grid` flows partial ticks.
- B's measurements: films ≤ 2 µm move 0.017-5.0% in a minute (old 0);
  one stroke at thinner .75 moves 5.64% in 5 min (old 0); short-wait phase
  variation ×1.98 → ×1.002; mixed-field truncation fixed; conservation
  within 6e-7; `wait(30)` at 2400 px 23.2 s → 31.4 s. Images:
  `notes/thinner/wash-experiment/lane-b-flow*.png` (visually near
  identical; mean difference 0.4 grey levels). Details:
  `notes/thinner/wash-experiment/lane-b-flow.md`.
- Blocker: protected check 17 fails, case "from 0.4 min: 15 vs 15 × 1", cure
  at 6 of 25,600 near-bare pixels (`age` resets a pixel under 1e-5 coats to
  fresh at each drying step, so off-grid splits differ).
- Working (uncommitted, ~100 lines in `drying.rs`, `thinner.rs`,
  `flow_probes.rs`): inside a wait, paint the flow carries onto a near-bare
  pixel keeps its cure (`age_from`, `absorb_with`). Not yet verified.
- To merge: c17 passes in all cases; `--quick` otherwise only the known
  check 2 `rag` scene; `cargo test --release -p paint --lib` green;
  engine 1/2 unaffected.

### A — brush exchange: WRAPPING UP

- Committed: `efa1527` (measurement fixes, ledger, reconstructed (b)/(B)
  controls, probes), `0ff4f00` (`set_exchange_floor`, default off),
  `71fc4b6` (round 2 sheets and `notes/thinner/wash-experiment/exchange-r2.md`).
  Default behavior unchanged; default-build fingerprint bit-identical.
- The four HANDOVER-3 measurement limitations are addressed in
  exchange-r2.md (paint above ground incl. dried film; ground response
  with exchange on; path consistency criterion vs a 0.001-unit nudge;
  coupled load effects).
- Round 2 result: no exchange setting passes the current protected thinner
  contract. Its pick for visual review, `fk4j1`, fails checks 5, 7 (+7.9%
  vs ≥ 20% required), 8, 14, 15, 16. Checks 6, 7 and 14 assert the
  per-stroke ceiling itself, so any exchange default needs an approved
  contract change.
- Under decision 5 (checks 7 and 9 must pass) the expected outcome is
  **"today" (exchange off)**. Await A's final report to confirm.
- The original (b)/(B) code is in
  `~/src/a/claude-paint-reviews/engine3-rag-review-2026-10-04/engine3-rag-review-2026-10-04.zip`
  (`engine3-rag-review/wip/look/`, look-experiments head 196a673, (b) in
  place); A's controls are reconstructions from the report.

### C — rag correctness: IN PROGRESS (separating the path fix)

- Scope: fix path-sampling dependence (the per-segment `ceil` in
  `rag_wipe`: the same line drawn with 2 vs many points lifts different
  amounts), plus correctness checks on controlled films and an engine-1/2
  rag guard (no old log uses a rag; the protected `rag` baseline scene is
  engine 3).
- Committed: `b1a5cd5` (ignored probes, no behavior change).
- Uncommitted: a ~650-line `rag.rs` change that contains both the path
  fix (`rag_wipe3`, `Seg`, `pad3`, `Touch`) and cross-wipe carryover (a
  `mobile` field on `Rag`, `rag_pool`, `rag_keep`). Carryover was briefly
  requested by the integration owner and then withdrawn (decision 4). C
  must remove carryover and commit the path fix alone; `notes/rag/path-fix/`
  holds probe output, sheets and `regen_rag_baseline.sh` (uncommitted).
- To merge, verify: `Rag` has exactly its old fields (no `mobile`);
  engine-1/2 rag case bit-identical to `d54b423`; the only behavior change
  is path sampling; the protected `rag` baseline rebaseline is prepared as
  a command for owner approval, not committed.
- If separation fails, merge without C's fix and record the path
  dependence as a known limitation.

## Known findings (not blockers for this painting)

- **Rag carries no paint color between wipes.** `Rag` (`rag.rs:198`) has
  no color; the smear pool is created per `rag_wipe` call (`rag.rs:634`),
  and a mask wipe calls `rag_wipe` per stroke (`rag.rs:759`). A dirty rag
  wiping a clean area leaves it clean. Owner agrees a rag should hold
  color; deferred until after the painting.
- **Lua table layout read** (`session.rs` `TableHead`, commit `c4bb131`):
  documented in `notes/easel.md` "Lua internals read directly" and at the
  `mlua` line of `crates/easel/Cargo.toml` (`a98d8ca`). Recheck before any
  mlua/lua-src/Lua upgrade.
- **Exchange pickup per step** (heads-up): under the exchange, pickup per
  drag step doesn't scale with step length; only relevant if an exchange
  default is chosen.
- **Unsigned receipt/approval notes**: any agent could forge them
  (release-gate limitation, outside this milestone).

## Remaining route to launch

1. Receive A's final report (expected: "today"). Merge A's branch (probes,
   notes, default-off switches) or only its notes, whichever is clean.
2. Receive B's c17 fix; verify; merge (decision 3).
3. Receive C's separated path fix; verify per the C section; merge, or
   skip it as a known limitation.
4. Rerun on the combined candidate: `scripts/test_thinner_acceptance
   --quick`, then `scripts/test --all` (~8 min). Classify every failure.
   Expected known item: check 2 `rag` scene (needs owner-approved
   rebaseline; if C's fix lands, regenerate with C's script after
   approval).
5. Regenerate the brush-made sienna card (`--card`) on the candidate.
6. Connected sequence (HANDOVER-3): underpainting → wiped lights → dirty
   carryover (within one wipe only, see findings) → refold → thin/body
   overpaint → wait → save/reopen → replay, images at working and detail
   size.
7. Re-verify the guide statements listed in LAUNCH.md against the build.
8. Freeze: tag `round-24`, follow `notes/round24/LAUNCH.md` steps 1-8
   (including the stillwet skip step), confirm the painter is using the
   easel, and give the viewer URL.
9. Clean up: remove `~/src/a/claude-paint-wt/*` worktrees and branches,
   lane B's `$TMPDIR/laneb/` worktrees; push `rag` and
   `refs/notes/golden-approvals` when the owner wants them published.
