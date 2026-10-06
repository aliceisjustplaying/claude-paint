# Paint a picture

Paint one picture of your choosing in oil, at the easel, a simulator of oil
paint on linen. Subject, composition and manner are yours. Work from what you
know; don't use reference images or image models.

## Your studio
- The folder ~/src/a/paint-studio-db6324: this brief, your notes and the easel.
- You paint at the easel: paint a chunk, look at the canvas, paint the
  next. Its tools are `paint`, `look`, `note`, `status` and `log`;
  notes/easel_guide.md explains them.
- What is painted stays painted. There is no undo: paint over what you
  don't want.
- You mix your own paint on the palette, from the tubes, in the
  proportions you choose.
- Time passes on the painting's own clock: every mark takes the time a
  hand takes, and `wait(minutes)` lets it rest, as long as you like; paint
  dries as that time passes. Real time between chunks doesn't count. The
  clock isn't a budget or a target.

## The rules of the studio
- Paint shapes and marks, not computed pictures: don't encode an image in
  masks, amounts or proportions.
- Shapes are drawn, not copied: don't make a mask or stroke by reflecting,
  flipping, rotating or translating another mask's or stroke's coordinates.
  A shape that mirrors another is drawn as its own shape, not as
  `m:at(x, 2*H - y)` or `m:at(W - x, y)`.

## What to read
notes/easel_guide.md; notes/studio_notes.md;
notes/research/oil_paint_physics.md as needed.

## Working
- You make every artistic decision.
- Look at your painting often, whole and close up.
- Keep a working journal with `note` as you go: your own working notes.
  Entries stay as written.
- Develop the painting until you judge it complete.

## Your reply
When you stop working, reply with the painting's title if you give it one
and a few sentences about the picture.
