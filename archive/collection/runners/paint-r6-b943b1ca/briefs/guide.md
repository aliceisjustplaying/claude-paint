# Write the painter's guide for the paint engine

Worktree: ~/src/a/claude-paint-r15-base (branch r15-base). Write two files:
README.md (replace it) and notes/guide.md. Commit them; do not push. Do not
change any other file. Scratch: run `~/.local/bin/agent-tmp guide` once and
export TMPDIR=TMP=TEMP.

## Who reads them
A painter (an AI model) who will write one painting as a Rust program with
the `paint` crate and render it with `cargo paint`. It has only this
folder: the engine (crates/paint/src, whose module docs are the reference),
paintings/src/run.rs (stages, crops, checkpoints), the study programs in
paintings/src/bin/study_*.rs, notes/research/ (Friedrich's materials and
oil paint physics) and scripts/peek. It renders at one size, 2400 px wide.

## What to write
- README.md: short. What the project is (a physical oil-paint simulator;
  paintings are programs; no image model, no reference imagery), how to
  run, render, crop, checkpoint and resume, and where to read further
  (notes/guide.md, the module docs, notes/research/, the studies). Every
  command and file named must exist in this folder: check each one.
- notes/guide.md: the engine as a painter uses it, in the order a painter
  needs it: canvas, ground and style; brushes and gestures; handlings
  (covering a passage); palette, color and aiming (masstone vs the look on
  the canvas); stipple; glazes and veils; drying and time; masks, forms and
  scene helpers; pencil; finish (varnish, cracks, relief) as options; the
  stage runner, crops, checkpoints and what makes a checkpoint stale;
  viewing renders; what is slow. Explain what each thing does physically
  and how to call it, with short API examples.

## Sources
Read the engine source and module docs in this worktree, run.rs and the
studies. You may also read, for API facts only, the old developer notes on
branch main: `git show main:notes/color.md`, strokes.md, stipple.md,
form.md, workflow.md, surface.md, tip.md, drying.md, pencil.md, time.md,
edges.md, scene.md, atmosphere.md. Check every API fact against the source
here (some modules exist on main that do not exist here).

## Rules (the reason this file exists)
The painter must meet the engine, not the project's history or taste.
- No history: no rounds, branches, commits, fixes, reviews, "used to",
  "now", "was changed".
- No people: no users, owners, viewers, critics, reviewers, judges, no
  quotes from anyone, no opinions of how something looked to someone.
- No past paintings or painters: no painting names, no "a painter found",
  no program names except the study programs that exist here.
- No motifs and no recipes for subjects: nothing about how to paint a sky,
  a tree, snow, a figure, a rock, water or anything else as a subject; no
  suggested compositions, palettes for scenes, or "what works". Describe
  tools and physics only; API examples must be neutral (a mask, a region, a
  stroke), not a scene.
- No aesthetic verdicts ("reads as digital", "looks natural", "better").
- No references to files or tools that are not in this folder.
- Don't mention any of these rules or what the guide leaves out.
Plain, dense, accurate. US English, no Oxford comma. Never write an
absolute home path or anyone's real name.

Final reply: a list of the sections and any API facts you couldn't verify.
