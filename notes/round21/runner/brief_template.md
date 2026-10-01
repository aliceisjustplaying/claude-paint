# Paint a picture

{OPENING}

## Your studio
- The folder {STUDIO}: this brief, your notes and the easel.
- You paint at the easel: paint a chunk, look at the canvas, paint the
  next. Its tools are `paint`, `look`, `note`, `status` and `log`;
  notes/easel_guide.md explains them.
- There is no undo. To change a passage, paint over it, or lift wet
  paint off with a brush.
- You mix your own paint on the palette, from the tubes, in the
  proportions you choose.
- Time passes on the painting's own clock: every mark takes the time a
  hand takes, and `wait(minutes)` lets it rest, as long as you like; paint
  dries as that time passes. Real time between chunks doesn't count. The
  clock isn't a budget or a target.

## The rules of the studio
- Paint shapes and marks, not computed pictures: don't encode an image in
  masks, amounts or proportions.
- Shapes are drawn, not copied: don't make a mask or stroke by mirroring
  or rotating another mask's or stroke's coordinates. A shape that mirrors
  another is drawn as its own shape, not as `m:at(x, 2*H - y)` or
  `m:at(W - x, y)`. Moving a shape and reusing your own helpers are fine.

## What to read
{READING}

## Working
- You make every artistic decision.
- Look at your painting often, whole and close up.
- Keep a working journal with `note` as you go: your own working notes.
  You can revise them.
- Develop the painting until you judge it complete.

## Your reply
When you stop working, reply with the painting's title if you give it one
and a few sentences about the picture.
