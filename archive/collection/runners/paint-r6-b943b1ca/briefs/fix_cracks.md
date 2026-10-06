# Bug investigation: the craquelure's variation doesn't reach the picture

Worktree: ~/src/a/claude-paint-fix-cracks (branch fix-cracks, from main).
Work only there; commit often; do not push or merge. Scratch: run
`~/.local/bin/agent-tmp fixcracks` once and export TMPDIR=TMP=TEMP to it.
Other agents may share the machine: wrap renders and tests in `timeout`.

## Evidence (read first)
- notes/cracks_lab/README.md (the whole file) and
  notes/cracks_lab/round2/critics/fable.md and astra.md: blind judgments
  of one painting finished with five crack settings. Fable MEASURED:
  coverage 0.7-1.1% everywhere (sky, snow, tree alike); every crack a
  one-pixel hairline ~5 gray levels darker, end to end; primaries 18-25 px
  apart; identical concentric arcs hugging all four corners.
- crates/paint/src/crack.rs module doc and `Cracks` fields: the design
  promises the opposite: `vary` (local paint decides subdivision and
  opening), `hierarchy` (first cracks open widest, swell and pinch),
  `patchy` (some passages split finely, others keep a few long cracks),
  `grime`, `dirt`, `depth_um`, `cupping_um`, `corners` (cracks
  perpendicular to the diagonal near the corners).
- A painter reported (branch r10-arm1, notes/r10_winter_a.md, friction 5):
  only `width_um` made a visible difference; `depth_um`, `dirt`, `grime`
  and `cupping_um` made none.
- The lab program: branch r10-arm2, paintings/src/bin/r10_crackslab.rs
  (uncommitted in ~/src/a/claude-paint-r10-arm2; copy what you need), with
  3200 checkpoints in that worktree's out/ (resume from `glaze` with
  `--stale-ok`; the output lands in out/r10_winter_b_full.png). Its finish:
  `Cracks { width_um: Some(16.0), dirt: 0.25, depth_um: 14.0, ..aged(0) }`.

## How to work
1. Measure, don't guess: for each setting above, does changing it change
   the rendered picture, and how (coverage, width distribution, contrast,
   per-region density: pale thin sky vs dark paint vs thick white)? Also
   render the crack network alone if you can, to see its geometry.
2. For every setting that promises an effect it doesn't deliver (or a
   geometry that contradicts its doc, like rings at the corners instead
   of diagonal cracks), write a failing test FIRST that expresses the
   documented physical expectation (e.g. with `hierarchy` 1 the widest
   cracks open several times wider than the narrowest; with `vary` 1,
   density differs between a thin pale region and a thick dark one;
   corner cracks run roughly perpendicular to the diagonal). See it fail
   on main.
3. Diagnose the cause (file:line) and fix the cause. This is a bug fix
   under a feature freeze: make the existing settings do what their docs
   say; no new settings, no noise added to hide uniformity. Principles:
   notes/principles.md (entropy comes from the process).
4. Note: Alice (the user) says "anything that reads like repetition reads
   digital"; both judges prefer a slight direction (`grain` ~0.25-0.35);
   dark paint cracks too (visibility is contrast, not absence). Don't
   change `grain`'s default; Alice decides defaults.
5. Before finishing: `cargo test --workspace` and
   `cargo test --release -p easel --test hand_time` green (re-record
   golden hashes only where the fix intends a pixel change, and say which);
   clippy clean.
6. For Alice (lossless PNG, notes/fixes/cracks/): the lab painting
   finished with its own settings on main vs your branch: the same two
   1:1 crops the lab uses (pale sky; dark tree against warm sky) plus a
   corner, side by side, labeled, and a `README.md` whose first section
   says in plain words what to look at. Also round 2's winter (branch
   amnesia-winter) before/after at one crop.
7. notes/fixes/cracks/README.md: each setting, measured before/after;
   the tests; the causes (file:line); what's left.

Final reply: per setting: worked / was broken (cause, fix) / left; test
names and results; image paths. US English, no Oxford comma. Never write
the user's real name or an absolute home path into committed files; the
user is "Alice".
