# A study: one tree in winter

You are a painter who has never seen this project's earlier paintings. Using
claude-paint, a physical oil-paint simulator in Rust where paintings are
programs, paint ONE study in the manner of Caspar David Friedrich, made from
what is known about his materials, method and habits. You work from
knowledge only: no reference images, no image models, never look at
pictures of his work. (The project's starting point: models painting
through programs, pixel by pixel, with no image model and no off-the-shelf
art software.)

## Your study
One tree, not a whole landscape: a single bare tree in winter, whole,
from where it stands in the snow to its finest twigs, against a winter
sky. The tree is the whole subject; which tree, and how old or damaged,
is yours to choose. Leave off the aged finish (no craquelure, no aged
varnish): the paint alone has to carry it. Make it look deliberately
painted, even if the drawing is clumsier.

## Setup
- Worktree: ~/src/a/claude-paint-r12-tree3 (branch r12-tree3). Work only there. Do not push.
- Your painting: paintings/src/bin/r12_tree3.rs. Render at one size only,
  2400 px wide: `cargo paint r12_tree3 -- --full --width 2400` (the whole,
  -> out/r12_tree3_full.png), `-- --full --width 2400 --crop x0,y0,x1,y1`
  (a window at that resolution, fast), `-- --ckpt` then `-- --resume
  <stage>` (skip finished stages). There is no smaller preview: the
  painting you look at is the painting. Read README.md for all of it.
- Do NOT modify crates/paint (the engine). If the engine is in your way,
  work around it in your program and write down the friction (below). That
  friction is the most valuable thing you produce besides the painting.
- Scratch: run `~/.local/bin/agent-tmp r12_tree3` once and export
  TMPDIR=TMP=TEMP to it. Never write to /tmp.
- Other painters share this 10-core machine: wrap renders in `timeout`
  (e.g. `timeout 900`); prefer previews and crops; never wait on a hung render.
- View PNGs only via `scripts/peek SRC OUT.jpg [H W Y X]` (renders are huge).
- Commit your program and notes often (`git add` only your files).

## What to read
README.md; notes/research/friedrich_materials.md (his materials and method,
sourced); notes/research/trees.md (how trees are built and how he studied
them, sourced); notes/research/oil_paint_physics.md as needed; the engine docs in
crates/paint/src (module docs); the engine feature notes notes/color.md,
strokes.md, stipple.md, form.md, workflow.md, motifs.md (API examples);
the study programs in paintings/src/bin/study_*.rs. paintings/src/trees.rs
and rocks.rs are another painter's habits for trees and rocks: you may read
them to learn the growth and form primitives, but motifs belong to the
painter: paint your own trees, rocks, figures and everything else in your
program. Do not read git history or any other notes.

## Principles
- The engine provides physics and tools (brushes, paint, canvas, optics,
  growth skeletons, solid forms, masks). You make every artistic decision.
- Don't let presets paint for you: Style::broad() etc. are a starting
  vocabulary, not a finished look. Choose stroke direction, curvature,
  order, stippling, glazing and every color like a painter would.
- Friedrich's pictures are full of tiny particular details; don't stop at
  broad passages. Use crops at full resolution to work on details.
- Look at your painting often, whole and in crops at 2400px, and judge it
  harshly: what reads as digital (too straight, too even, too neat) and
  what reads as paint?

## Deliverables (keep them updated as you go; sessions can die)
1. paintings/src/bin/r12_tree3.rs and the final render out/r12_tree3_full.png
   (2400px).
2. notes/r12_tree3.md, written continuously: what you tried and why; your
   working method; FRICTION: every place the engine made something hard,
   wrong, slow or impossible, with the workaround you used; your honest
   critique of the result.
3. Final reply: a short summary with the render paths and your top five
   friction points.

Budget: about 60 minutes; hard stop at 90.
