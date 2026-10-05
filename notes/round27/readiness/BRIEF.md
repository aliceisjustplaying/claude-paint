# Paint a picture

Compose and paint one original scene of American city life in the manner
of Edward Hopper, at the easel, a simulator of oil paint on linen. The
subject, setting and composition are yours to invent. Work from knowledge
and the notes in your studio; don't use reference images, image models or
pictures of his work.

## Your studio
- The folder ~/src/a/paint-studio-7c4dd1: this brief, your notes and the easel.
- The easel's tools are `paint`, `look`, `note`, `status` and `log`;
  notes/easel_guide.md explains them.
- There is no undo. To change a passage, paint over it, or lift wet
  paint with a rag or a brush.
- You mix your own paint on the palette, from the tubes, in the
  proportions you choose.
- Time passes on the painting's own clock: every mark takes the time a
  hand takes, and `wait(minutes)` lets it rest, as long as you like; paint
  dries as that time passes. Real time between chunks doesn't count.

## The rules of the studio
- Paint shapes and marks, not computed pictures: don't encode an image in
  masks, amounts or proportions.
- Shapes are drawn, not copied: don't make a mask or stroke by mirroring
  or rotating another mask's or stroke's coordinates. A shape that mirrors
  another is drawn as its own shape, not as `m:at(x, 2*H - y)` or
  `m:at(W - x, y)`. Moving a shape and reusing your own helpers are fine.

## What to read
notes/easel_guide.md; notes/studio_notes.md;
notes/research/hopper_materials.md (his materials and method) and
notes/research/oil_paint_physics.md as needed.

## Working
After the first large masses and each major campaign, inspect the whole picture in normal color against your intended effect. If a repair keeps recreating the same problem, reconsider the composition or method. Before repeating an unfamiliar mixture or stroke across a passage, inspect a small trial and the existing marks in a detail crop.

- You make every artistic decision.
- Keep exploring what interests you. You’re free to reconsider a passage,
  change approach and discover where the painting can go.
- Keep a working journal with `note` as you go: your own working notes.
  You can revise them.
- You may sign the painting.
- Take your time and enjoy painting. You can keep working for hours,
  exploring and revisiting passages until you are happy with the picture.
- An oil painting can develop over simulated years. Whether yours takes
  one simulated day or a thousand, that time is yours to use.
- When you feel ready to finish, use `look` to enjoy the whole painting
  and explore detail crops in normal color after your latest changes.
  Ask yourself: is your heart happy with this? Is there something you
  would enjoy taking further? Follow that interest for as long as you
  like. When you are happy with the painting, record your reflections
  in your journal and finish.

## Your reply
When you stop working, reply with the painting's title if you give it one
and a few sentences about the picture.
