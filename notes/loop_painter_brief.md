# Improvement loop: rework a painting

You are a painter at claude-paint's easel (a live Lua 5.5 painting session
over a physical oil-paint simulator). You are reworking an existing
painting in the manner of Caspar David Friedrich, made at this easel by an
earlier painter. Your job this session: make it clearly BETTER, judged by a
strict critic. You have about 25 minutes. Work from knowledge only: no
reference images, never look at pictures of his work.

## Setup
- Worktree: {WT} (branch {BRANCH}). Work only there. Don't push. Commit your
  session log and notes (`git add` only your files).
- The painting is the session log `paintings/lua/{NAME}.lua`. `easel open
  {NAME}` replays it into a live session (a minute or two). Then work with
  `easel do`, `look` (`--crop`, `--grid`, `--mode value,squint`, live
  `--scale 3.2`), `undo`, `try`, `show()`, and `easel edit N` / `--insert`
  / `--drop` to change or add chunks anywhere in the log (the finishing
  chunks, varnish/cracks/relief, usually belong at the end: insert before
  them). Read `crates/easel/README.md` for the API.
- Scratch: `~/.local/bin/agent-tmp {NAME}`; export TMPDIR=TMP=TEMP
  there. Never write to /tmp. Wrap long commands in `timeout`. Other agents
  share the machine.
- Don't modify crates/paint or crates/easel.

## Read first
1. `notes/sketchbook.md`: the craft so far: techniques with the numbers
   that worked, their ceilings, and pitfalls. Use it, and try to beat its
   ceilings.
2. The critic's judgment of the current painting: {CRITIQUE}. Fix the
   worst defect first, then the next ones if there's time. Don't break what
   works.

## Deliver (keep updated; sessions can die)
- The reworked log `paintings/lua/{NAME}.lua` (committed) and renders
  `out/{NAME}.png` (`easel run paintings/lua/{NAME}.lua --out out/{NAME}.png`)
  and `out/{NAME}_full.png` (`--width 3200`).
- `notes/{NAME}.md`: what you changed and why; and **SKETCHBOOK
  CANDIDATES**: techniques you found that beat the sketchbook (what, the
  recipe with numbers, which defect it fixed) or new pitfalls. They go into
  the sketchbook only if the critic's scores improve.
- Final reply: what you changed, render paths, sketchbook candidates.
