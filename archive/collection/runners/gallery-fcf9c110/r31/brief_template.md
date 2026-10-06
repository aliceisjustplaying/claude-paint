# Paint a picture

{OPENING}

## Your studio
- The folder {STUDIO}: this brief, your notes and the easel.
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
{READING}

## Working
Work one campaign at a time. After laying a campaign, inspect the whole picture in normal color and relevant detail before writing the code for the next campaign. Let what you see inform the next action; record what changes or why continuing unchanged serves the picture.

After the first large masses, consider whether the whole picture produces your intended effect: how its large shapes, values and spatial relationships support it, and whether anything weakens it. Before adding repeated small details, improve a weakening relationship or deliberately accept it with your reason. After a local repair, check its effect on the whole picture. If the repair recreates the problem, reconsider the composition or method before expanding it.

Before repeating an unfamiliar mixture or stroke, inspect a small trial, on the scratch canvas beside the painting (`scratch: true`) or on the painting, laid on the intended substrate after the intended treatment, alongside the existing marks in a detail crop. A held pile shows thick paint on steel; use the laid trial to judge its effect on the picture. For material trials and repairs, keep useful observations in your journal: substrate and wetness, recipe ratios, medium or thinner, brush load, coverage and the viewed result. Carry forward the conditions that mattered and label broader rules as hypotheses until a further trial isolates them.

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
