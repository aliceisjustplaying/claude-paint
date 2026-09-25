# Paint an original Friedrich

You are a painter who has never seen this project's earlier paintings. Using
claude-paint, a physical oil-paint simulator in Rust, compose and paint ONE
new, original painting in the manner of Caspar David Friedrich: not a copy
of any existing Friedrich, but a picture he could have painted, made from
what is known about his materials, method, habits and motifs. You work from
knowledge only: no reference images, no image models, never look at
pictures of his work. (The project's starting point: models painting
through programs, with no image model and no off-the-shelf art software.)

## Your commission (loose; the composition is entirely yours)
**A winter landscape.** Everything else (place, hour, weather, what is in
it, who is in it) is your choice.

## Your medium: a Rust program
Write your painting as one Rust program, paintings/src/bin/<name>.rs, using
the `paint` crate directly. Run: `cargo paint <name>` (1000px preview),
`-- --full` (3200px), `-- --full --crop x0,y0,x1,y1` (full-res window,
fast), `-- --ckpt` then `-- --resume <stage>` (skip finished stages); the
engine docs are in crates/paint/src (module docs), with examples in
paintings/src/bin/study_*.rs. Motifs belong to you: paint your own trees,
rocks, figures and everything else in your program.

## Setup
- Worktree: ~/src/a/claude-paint-r9-arm2 (branch r9-arm2). Work only there. Do not push. Commit
  your files often (`git add` only your files).
- Do NOT modify crates/paint or crates/easel. If the engine is in your
  way, work around it and write down the friction (below). That friction is
  the most valuable thing you produce besides the painting.
- Scratch: run `~/.local/bin/agent-tmp r9arm2` once and export
  TMPDIR=TMP=TEMP to it. Never write to /tmp.
- Two other painters share this machine: wrap renders in `timeout`
  (e.g. `timeout 900`); prefer previews and crops.
- View PNGs only via `scripts/peek SRC OUT.jpg [H W Y X]` (renders are huge).

## What to read
README.md; notes/research/friedrich_materials.md (his materials and method,
sourced); notes/principles.md (short: how this project thinks about tools);
the guide for your medium (below). Everything else in notes/ is optional:
read what you need, when you need it. Don't read git history.

## Principles
- The engine provides physics and tools (brushes, paint, canvas, optics,
  masks). You make every artistic decision.
- Don't let presets or helpers paint for you: they're a starting
  vocabulary, not a finished look. Choose stroke direction, curvature,
  order, stippling, glazing and every color like a painter would.
- Friedrich's pictures are full of tiny particular details; don't stop at
  broad passages. Use crops at full resolution to work on details.
- Look at your painting often, at 1000px and in 3200px crops, and judge it
  harshly: what reads as digital (too straight, too even, too neat) and
  what reads as paint?
- Paint skies (and other quiet passages) with broad, overlapping strokes
  blended wet into wet, not with a stippled veil of small touches: in
  earlier paintings a stipple layer over the sky read as "JPEG artifacts"
  (a blotchy, clumped mottle) to every viewer.
- Let neighboring paint bury the feet of things: drag snow across the base
  of trunks, crosses, rocks and figures so they stand in the snow, not on it.

## Deliverables (keep them updated as you go; sessions can die)
1. Your painting's source (see your medium) and plain renders copied to
   notes/round9/arm2/painting_1000.png and painting_3200.png (lossless).
2. notes/round9/arm2/notes.md, written continuously: the composition and
   why (what in Friedrich it draws on); your working method stage by stage;
   FRICTION: a running list of every place the engine made something hard,
   wrong, slow or impossible, with the workaround you used.
3. Final reply: the title, the render paths and your top five friction
   points.

Budget: about 90 minutes; hard stop at 2 hours. Have a complete painting
with a full render by 75 minutes, then refine.

US English, no Oxford comma. The project is published under the pseudonym
"alice": never write the user's real name or an absolute home path into
committed files.
