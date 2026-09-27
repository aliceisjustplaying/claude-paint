# Paint a picture

{OPENING}

## Your studio
- The folder {STUDIO}. Work only there, and in your scratch folder.
- You paint at the easel: open it, paint a chunk, look at the canvas,
  paint the next. {EASEL_HOWTO}
- What is painted stays painted. There is no undo: paint over what you
  don't want.
- You mix your own paint on the palette, from the tubes, in the
  proportions you choose.
- Time passes on the painting's own clock: every mark takes the time a
  hand takes, and `wait(minutes)` lets it rest, as long as you like; paint
  dries as that time passes. Real time between commands doesn't count. The
  clock isn't a budget or a target.
- Scratch: run `~/.local/bin/agent-tmp {SLUG}`, then
  `export TMPDIR="<that folder>" TMP="<that folder>" TEMP="<that folder>"`;
  write temporary files only there.

## The rules of the studio
- The easel is the only way to paint and to see the painting. Don't replay
  or copy the session yourself, open a second one, or edit or restore its
  files.
- Look at the canvas with the easel's views. Don't read pixel values with
  other programs.
- Paint shapes and marks, not computed pictures: don't encode an image in
  masks, amounts or proportions.
- Shapes are drawn, not copied: don't make a mask or stroke by reflecting,
  flipping, rotating or translating another mask's or stroke's coordinates.
  A shape that mirrors another is drawn as its own shape, not as
  `m:at(x, 2*H - y)` or `m:at(W - x, y)`.

## What to read
{READING}

## Working
- You make every artistic decision.
- Look at your painting often, whole and close up.
- Keep a working journal with `bin/easel note "..."` as you go: your own
  working notes. Add to it; don't rewrite earlier entries.
- Develop the painting until you judge it complete. Then save it
  ({EASEL_SAVE}) and close the session.

## Your reply
When you stop working, reply with the paths of the saved painting and its
log, the painting's title if you give it one and, if you like, a few
sentences about the picture.
