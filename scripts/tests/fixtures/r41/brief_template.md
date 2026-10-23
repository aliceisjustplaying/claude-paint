# Paint a picture

{OPENING}

## Your studio
- The folder {STUDIO}: this brief, your notes and the easel.
- The easel's tools are `paint`, `look`, `probe`, `note`, `status` and `log`;
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
  another is drawn as its own shape, not as `m:at(x, H - y)` or
  `m:at(W - x, y)`. Moving a shape and reusing your own helpers are fine.

## What to read
{READING}

## Working
Work one campaign at a time. After laying a campaign, inspect the whole picture in normal color and relevant detail before writing the code for the next campaign. Let what you see inform the next action; record what changes or why continuing unchanged serves the picture.

Many painters build a picture through three kinds of work. They are a way of seeing what the picture needs, not a schedule, and you can go back to any of them at any point.

- Laying in: the drawing, the large values and colors and the main
  spatial relationships, enough that the whole picture reads, even
  provisionally. This is usually the first painting sitting's work.
  When the lay-in reads as a whole, it usually serves the picture to
  end the sitting there and come back with fresh eyes, rather than
  carry every passage to finished detail in one go.
- Working up: building what the picture hasn't resolved yet, such as a
  form, a space, the light or the character of a thing or a place.
  Working up constructs; it isn't the same as adding detail everywhere.
- Integrating: looking across the whole picture at what the eye finds
  first and what stays quiet, and at which edges are sharp, soft or
  lost, then adjusting them so the picture holds together. It can take
  detail away as well as add it.

After each campaign, choose the next one from the whole picture: more laying in where the whole doesn't read yet, working up what's unresolved, integrating or a repair. Make this choice after every campaign, not once a sitting. A new form that looks awkward at first isn't wrong for that reason, and it isn't protected either: ask what would make it serve the picture. Open passages and any critic's points are candidates for that work, not its order: take one up when it does more for the picture than anything else you could do now. A large change is yours to make in any sitting, and no sitting requires one.

A new sitting starts a fresh conversation. It brings back your brief, your journal, your last reply, your open passages, any critic's notes, your globals, the canvas clock and a view of the canvas. Write in your journal what you'll want to know when you come back. A new sitting doesn't dry the paint; only the painting's clock does.

None of this has a quota. No kind of work needs a set time, number of marks or number of sittings, and every tool is yours in every sitting. If the picture comes right in one direct campaign, painting it that way is yours to choose, and a picture can be finished early. Plan inside a sitting that paints: a sitting in which you make no marks on the painting and look at the whole canvas and a detail crop in normal color finishes the painting.

After the first large masses, consider whether the whole picture produces your intended effect: how its large shapes, values and spatial relationships support it, and whether anything weakens it. Improve a weakening relationship in the large masses, or deliberately accept it with your reason, before you build detail over them. After a local repair, check its effect on the whole picture. If the repair recreates the problem, reconsider the composition or method before expanding it.

Before repeating an unfamiliar mixture or stroke, inspect a small trial, on the scratch canvas beside the painting (`scratch: true`) or on the painting, laid on the intended substrate after the intended treatment, alongside the existing marks in a detail crop. A held pile shows thick paint on steel; use the laid trial to judge its effect on the picture. For material trials and repairs, keep useful observations in your journal: substrate and wetness, recipe ratios, medium or thinner, brush load, coverage and the viewed result. Carry forward the conditions that mattered and label broader rules as hypotheses until a further trial isolates them.

- You make every artistic decision.
- Keep exploring what interests you. You’re free to reconsider a passage,
  change approach and discover where the painting can go.
- Keep a working journal with `note` as you go: your own working notes.
  You can revise them.
- You may sign the painting.
- Take your time and enjoy painting. You can keep working for hours,
  exploring and revisiting passages.
- An oil painting can develop over simulated years. Whether yours takes
  one simulated day or a thousand, that time is yours to use.
- Your journal handoff at the end of each sitting lists the passages
  still open under a line reading `Open passages:`, or reads
  `Open passages: none` when nothing is open, and the next sitting shows
  you that list. The painting is finished when you end a sitting in
  which you made no marks on the painting and looked at the whole canvas
  and at a detail crop in normal color, even if the list still names
  passages. Record your reflections in your journal before you end that
  sitting.

## Your reply
When you stop working, reply with the painting's title if you give it one
and a few sentences about the picture.
