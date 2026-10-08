# Release-only bristle comparison

**Experiment only. No candidate was promoted.** [Private comparison page](http://m3p.tailec2dc.ts.net:8773/release/index.html) shows the existing 1.0× candidate beside 0.9× and 0.8× release, with the old ordinary-drag baseline as a fourth column. Loaded, nearly spent, curved/pressure-varying and wet-crossing scenes are available at 160 × 80 and 320 × 160 px. All 98 original evidence files remain byte-identical to report commit `78b3dd5e`: [identity proof](proof.json), [original hashes](original-evidence.sha256). The [original gallery](../index.html) is untouched.

Implementation starts from approved experiment `9e28e343`; original report `78b3dd5e` remains the record of its full-gate limits. This follow-up changes only the experiment's release multiplier, its tiny example and the separate `release/` evidence. Worktree: `~/src/a/claude-paint-oil-bristle-folds`, branch `experiment/oil-bristle-folds`.

## What is held fixed

The [doc-hidden study call](../../../crates/paint/src/bristle.rs) accepts only 1.0, 0.9 or 0.8. It multiplies the existing candidate release factor after its original clamp. Real and off-window ghost contact use that same factor. Per-hair finite reservoirs, solvent handling and the 95% release cap remain in place. Ordinary `drag` still selects no fold mode; `drag_bristle_folds` still selects 1.0.

No geometry field formula, seed, gesture, starting load, support or lighting value changed. The [example](../../../crates/paint/examples/bristle_folds.rs) uses brush seeds 19/71, support seed 23 and the original 315°/30°/0.9 lamp. Actual contact and pickup can respond to retained paint; identical geometry inputs do not promise identical final paint-dependent hair trajectories. No public configuration framework or persisted setting was added.

## Measured paint amounts

Canvas amount is the **net paint remaining on the canvas after pickup**, not cumulative gross release. Brush amount is paint remaining in all participating brushes. Units are coats × square canvas units. Crossing includes both brushes and the dirty continuation. Every percentage below is an actual amount divided by its reference amount, not the chosen release coefficient.

At 320 px, 0.9× leaves **92.92–94.06%** of the 1.0× candidate's canvas paint and **107.37–108.78%** of its remaining brush paint. At 0.8× these ranges are **85.31–87.49%** and **115.28–118.44%**. The reduced-release candidates still leave more canvas paint than the old baseline: **12.65–18.64% more** for 0.9× and **4.45–8.92% more** for 0.8×.

The following 320 px values are rounded for reading. [All 32 rows at both resolutions](comparison.csv) retain full recorded precision and baseline amounts; [raw budgets](metrics.csv) retain elapsed times.

| Scene | Release | Net canvas paint | Brush paint left | Canvas % of 1.0× / old baseline | Brush % of 1.0× / old baseline |
|---|---:|---:|---:|---:|---:|
| Loaded | 1.0× | 383359.687 | 265836.056 | 100.00% / 123.40% | 100.00% / 78.52% |
| Loaded | 0.9× | 360016.713 | 289179.018 | 93.91% / 115.89% | 108.78% / 85.42% |
| Loaded | 0.8× | 334328.235 | 314867.505 | 87.21% / 107.62% | 118.44% / 93.01% |
| Nearly spent | 1.0× | 13238.595 | 12729.248 | 100.00% / 127.69% | 100.00% / 81.60% |
| Nearly spent | 0.9× | 12300.971 | 13666.871 | 92.92% / 118.64% | 107.37% / 87.61% |
| Nearly spent | 0.8× | 11293.413 | 14674.430 | 85.31% / 108.92% | 115.28% / 94.07% |
| Curve / pressure | 1.0× | 383499.711 | 265696.052 | 100.00% / 123.66% | 100.00% / 78.36% |
| Curve / pressure | 0.9× | 360705.145 | 288490.574 | 94.06% / 116.31% | 108.58% / 85.08% |
| Curve / pressure | 0.8× | 335531.404 | 313664.306 | 87.49% / 108.19% | 118.05% / 92.51% |
| Wet crossing | 1.0× | 506596.922 | 367848.859 | 100.00% / 120.11% | 100.00% / 81.26% |
| Wet crossing | 0.9× | 475103.686 | 399342.096 | 93.78% / 112.65% | 108.56% / 88.22% |
| Wet crossing | 0.8× | 440548.224 | 433897.553 | 86.96% / 104.45% | 117.96% / 95.85% |

At 160 px, canvas paint is 92.88–93.94% of 1.0× for 0.9× and 85.23–87.29% for 0.8×. Resolution-dependent deposition and peak height remain. These are simulation measurements, not calibrated physical quantities. Wet-height PNGs clip above 250 µm; equally white peaks need not have equal height.

## Verification receipts

- [Focused tests](tests.log): both `fold_candidate_tests` passed, zero failed or ignored, 0.56 s test time; locked command exited 0 in 1.30 s. The existing finite-budget/pickup test now covers all three releases, at 64/160/320 px with thinner 0/0.5. Paint and solvent balance and finite nonnegative hair reservoirs are asserted. No expected outputs or protected tests were weakened.
- [Exact comparison proof](proof.json): 1.0× exactly reproduces all 24 original candidate PNGs; all 24 original baseline PNGs and all 16 original budget rows also match. Every one of the 98 tracked original evidence files matches `78b3dd5e`, independently of the missing temporary recovery manifest.
- [Render/repeat receipt](render.log): locked sweep exited 0 in 10.19 s. One-thread and four-thread repeat runs each match all 96 PNGs, all 32 budget rows excluding elapsed time and all 32 drying rows. Maximum absolute relative canvas-plus-brush paint error is **7.3 × 10⁻⁸** from the recorded CSV, below the existing 2 × 10⁻⁵ assertion limit.
- [State checks](checks.txt): same-input repeats match every canvas state field and brush debug state. Views leave state unchanged. Checkpoint continuation matches saved-state bytes, flat view and brush state. All three release values have zero measured loaded-stroke interior crop difference with 150 units of context. Ghost contact still approximates unseen canvas; this does not prove arbitrary crops exact.
- [Build recovery](build-recovery.json), [compiler log](build.log): the canceled predecessor's build completed both compiler phases (12.56 s and 15.30 s) and left both executables. Its helper and process group were absent and the lock was free. Artifacts were reused, not rebuilt. The stale running record does **not** constitute an outer successful-exit receipt. Offline `uv` with the explicit existing venv completed the 20 s launch diagnostic immediately.
- [Command receipts](run-receipts.json) record commands, exits, durations and unchanged 60 s limits. Builds use a 600 s lock. No whole painting, broad test suite or additional agent was run. The previous full-gate rag/box failures and timeout remain uncertified, as recorded in the [original report](../REPORT.md).
- [Browser check](browser-check.json): both full Tailscale gallery URLs returned HTTP 200. All 96 comparison PNGs returned 200 and decoded at their expected dimensions; lit images displayed at native 160/320 px widths. Saved review PNGs: [160 px](review-lit-160.png), [320 px](review-lit-320.png). The retained server binds only to `100.82.115.34:8773`. Connectivity from another machine was not tested.

## Reproduction and retained state

[Reproduction script](reproduce.sh) uses offline `uv` and the existing worktree venv, a 600 s locked build and separate 60 s locked test/render commands. Long runs are launched asynchronously; a build launch has a 660 s outer job limit. [Verification/page helper](check_sweep.py) compares against the saved original evidence and writes only this follow-up directory. [Follow-up hashes](SHA256SUMS) identify the exact source and deliverables.

The worktree, private server, browser tab, venv and deliverable PNGs are retained. Generated `target/` is removed after verification; temporary repeat directories remove themselves. No merge, push, PR, default change or lighting change was made.

Not done yet: visual approval of a release choice, the complete repository gate and any separately approved promotion.
