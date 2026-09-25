# Amnesia round 2: paint an original Friedrich

You are a painter who has never seen this project's earlier paintings. Using
claude-paint, a physical oil-paint simulator in Rust where paintings are
programs, compose and paint ONE new, original painting in the manner of
Caspar David Friedrich: not a copy of any existing Friedrich, but a picture
he could have painted, made from what is known about his materials, method,
habits and motifs. You work from knowledge only: no reference images, no
image models, never look at pictures of his work. (The project's starting
point: models painting through programs, pixel by pixel, with no image model
and no off-the-shelf art software.)

## Your theme (loose; the composition is entirely yours)
Winter. Any time of day, any subject Friedrich might have chosen in a winter landscape.

## Setup
- Worktree: ~/src/a/claude-paint-r11-flash (branch r11-flash). Work only there. Do not push.
- Your painting: paintings/src/bin/r11_winter_flash.rs. Run: `cargo paint r11_winter_flash`
  (1000px preview), `-- --full` (3200px), `-- --full --crop x0,y0,x1,y1`
  (full-res window, fast), `-- --ckpt` then `-- --resume <stage>` (skip
  finished stages). Read README.md for all of it.
- Do NOT modify crates/paint (the engine). If the engine is in your way,
  work around it in your program and write down the friction (below). That
  friction is the most valuable thing you produce besides the painting.
- Scratch: run `~/.local/bin/agent-tmp r11_winter_flash` once and export
  TMPDIR=TMP=TEMP to it. Never write to /tmp.
- Other painters share this 10-core machine: wrap renders in `timeout`
  (e.g. `timeout 900`); prefer previews and crops; never wait on a hung render.
- View PNGs only via `scripts/peek SRC OUT.jpg [H W Y X]` (renders are huge).
- Commit your program and notes often (`git add` only your files).

## What to read
README.md; notes/research/friedrich_materials.md (his materials and method,
sourced); notes/research/oil_paint_physics.md as needed; the engine docs in
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
- Look at your painting often, at 1000px and in 3200px crops, and judge it
  harshly: what reads as digital (too straight, too even, too neat) and
  what reads as paint?

## Deliverables (keep them updated as you go; sessions can die)
1. paintings/src/bin/r11_winter_flash.rs and final renders out/r11_winter_flash.png (1000px) and
   out/r11_winter_flash_full.png (3200px).
2. notes/r11_winter_flash.md, written continuously: the composition and why (what in
   Friedrich it draws on); your working method stage by stage; FRICTION: a
   running list of every place the engine made something hard, wrong,
   slow or impossible, with the workaround you used; your honest critique of
   the result.
3. Final reply: a short summary with the render paths and your top five
   friction points.

Budget: about 90 minutes; hard stop at 2 hours. Have a complete painting
with a full render by 75 minutes, then refine.
