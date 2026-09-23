# Amnesia round 3: paint at the easel

You are a painter who has never seen this project's earlier paintings. Using
claude-paint's **easel** (a live painting session in Lua 5.5 over a
physical oil-paint simulator: bristle brushes carrying paint, Kubelka–Munk
optics, drying over time), paint ONE new, original painting in the manner of
Caspar David Friedrich: a picture he could have painted, made from what is
known about his materials, method, habits and motifs, not a copy of any
existing work. Work from knowledge only: no reference images, no image
models, never look at pictures of his work. (The project's starting point:
models painting through programs, with no image model and no off-the-shelf
art software.)

## Your brief
{THEME}

## The easel
- Worktree: {WT} (branch {BRANCH}). Work only there. Don't push.
- Read `crates/easel/README.md` first: it's the painter's guide (opening a
  session, the loop, the whole Lua API with examples). Build once with
  `cargo build --release -p easel`; the binary is `target/release/easel`.
- Paint live, one chunk at a time: `easel open {NAME}`, then `easel do '...'`
  (or `do --look`), `easel look` (with `--crop`, `--mode value|squint|mirror`),
  `easel undo`. Time is real: `wait(minutes)` lets paint set and dry, and the
  paint behaves differently open, tacky and dry.
- Your painting IS the session log, `paintings/lua/{NAME}.lua`: it replays.
  Final renders: `easel run paintings/lua/{NAME}.lua --out out/{NAME}.png`
  (1000px) and `--width 3200 --out out/{NAME}_full.png`. Crops at 3200 are
  cheap: use them to work on details.
- Do NOT modify crates/paint or crates/easel. If something is in your way,
  work around it in Lua and write down the friction (below).
- Scratch: run `~/.local/bin/agent-tmp {NAME}` once and export
  TMPDIR=TMP=TEMP to it. Never write to /tmp. Two other painters share this
  10-core machine: wrap long commands in `timeout`; don't wait on a hung
  render. View PNGs only via `scripts/peek SRC OUT.jpg [H W Y X]` or the
  easel's `look` JPEGs.
- Commit your session log and notes often (`git add` only your files).

## What to read
crates/easel/README.md; notes/research/friedrich_materials.md (his materials
and method, sourced); notes/research/oil_paint_physics.md as needed. You may
read the engine feature notes (notes/*.md) for depth and the sample sessions
in paintings/lua/ to learn the API. Don't read git history or any other notes.

## Principles
- The engine and easel provide physics and tools. Every artistic decision is
  yours: composition, light, every color, stroke direction, order, drying
  times, where to stipple or glaze.
- Friedrich's pictures are full of tiny particular details, especially in
  foregrounds. Don't stop at broad passages; use 3200 crops.
- Look often (value and squint views too) and judge harshly: what reads as
  digital (too straight, too even, too neat, too regular) and what as paint?

## Deliverables (keep them updated; sessions can die)
1. paintings/lua/{NAME}.lua and renders out/{NAME}.png, out/{NAME}_full.png.
2. notes/{NAME}.md, written continuously: the picture and why (what in
   Friedrich it draws on); how you worked, stage by stage; **HOW THE EASEL
   FELT**: honestly, as the agent at the easel: what felt like painting,
   what felt like programming, what was slow, confusing or missing, and how
   it compares with what you'd expect from writing a program; FRICTION: every
   place the engine or easel made something hard, wrong or impossible, with
   your workaround; an honest critique of the result.
3. Final reply: a short summary with the render paths, your top five
   friction points and three sentences on how the easel felt.

Budget: about 90 minutes; hard stop at 2 hours. Have a complete painting
with a full render by 75 minutes, then refine.
