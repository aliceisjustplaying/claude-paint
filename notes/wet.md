# Real wet-on-wet (branch `r6-wet`)

Round 6, step 3 of the mark-making lab (`notes/HANDOFF.md`): open paint
should blend, drag and soften at contours; a loaded light laid into a wet
dark should stay clean in proportion to its load and stiffness; the
sketchbook's "`dry()` before any passage" default should go.

## 1. Audit (before any engine change)

### The study

`paintings/src/bin/study_wet.rs` (`cargo paint study_wet`, 1000 px square,
about 15 s). Four gestures, each in four columns: over paint that is
**open** (laid seconds before), **setting** (near its gel point: the study
waits in 5-minute steps until each row's underlayer reads `Setting`, then
paints that row), **tacky** (set, sticky) and **touch-dry**. Brush, load,
colors, seed and gesture are identical across a row. The underlayers are
laid with the style's `body` handling, as a painter at the easel would lay
them. The program prints a table of measurements (lightness in OKLab, from
the dried picture unless marked):

1. **Lights into a dark mass.** Six touches (filbert 10, loaded 0.9 with a
   stiff light green, reloaded every third) and three short strokes into a
   dark green mass. *Clean* is where the touch's core sits between the dark
   (0) and the light paint's masstone (1). *Area* is how much of each touch
   reads lighter than halfway.
2. **Sky down across a hill.** The sky laid in above, then brought down in
   level hog-flat strokes 8 units (3.5 mm) into the hill's top edge, then a
   clean badger worked three times along the join. *Edge* is the 10–90%
   width of the lightness step across the silhouette (median of 20
   profiles), before the badger (wet look) and after it (dried). *Pulled*
   is how far the sky 6 mm above the join moved toward the hill value, and
   the hill 6 mm below toward the sky value.
3. **Contact shadow.** Two strokes of a dark (filbert 6) dragged along the
   base of a light form where it meets the ground color. *Depth* is how
   close the stroke's darkest point gets to the dark paint; edges are the
   10–90% widths up into the form and down into the ground.
4. **Thick light over thin dark.** A hog flat loaded full with stiff light
   body color laid lightly (pressure 0.45) over a thin medium-rich dark, and
   the same color thinned (stiff 0.3, hiding 0.6, load 0.45) and pressed
   (0.85). *Clean* at the stroke's start and end.

### What it measured (engine at `2c5a658`)

```
(units: mm at the canvas's 440 mm width)         open  setting    tacky      dry
1 light touches: clean, first of a load          0.56     0.38     0.99     0.89
  clean, third of a load                         0.35     0.27     0.99     0.89
  area reading light, mm² per touch              0.90     0.10     7.99     6.72
  soft fringe share of the touch                 0.83     0.97     0.25     0.32
  short strokes, clean                           0.98     0.89     1.00     1.00
2 sky over hill: edge 10–90% before badger       3.08     2.64     1.32     3.08
  edge 10–90% after badger (dried)               7.92     7.92     1.32     1.32
  sky 6 mm above the join, pulled to hill (0..1) 0.05     0.05     0.04     0.05
  hill 6 mm below the join, pulled to sky        0.08     0.05     0.53     0.45
3 contact shadow: depth reached (1 = paint)      0.95     0.96     1.00     1.00
  edge to the form above, 10–90%                 2.64     2.20     0.88     0.44
  edge to the ground below, 10–90%               3.52     1.76     0.44     0.88
4 loaded stiff light, clean (start/end)      0.95/0.72 0.86/0.23 0.94/0.32 0.93/0.76
  thinned light pressed, clean (start/end)   0.72/0.58 0.66/0.21 0.67/0.13 0.58/0.25
```

Images: `notes/wet/row1_before.jpg` … `row4_before.jpg` (1000 px render,
one row each, columns open · setting · tacky · dry).

### What oil paint does

- **Wet over wet is the normal case, not an accident.** Alla prima is
  defined by new paint going into paint that is still wet
  (https://en.wikipedia.org/wiki/Wet-on-wet). How much the two mix is the
  painter's choice: a loaded brush laid with a light touch places the new
  paint *on top* of the wet layer; pressure, a stiff brush, a lean
  (underloaded) brush and going back and forth work it in. Mud comes from
  working, not from contact.
- **Lights into darks.** The rule of the alla prima painter: the light goes
  in last, thick, with a loaded brush and one touch. A fully loaded brush
  rides on a cushion of its own paint; the wet dark under it comes up only
  where the bristles reach through that cushion (the mark's broken,
  slightly soiled edges, and the tail of a stroke where the load gives
  out). A second touch from the same load is dirtier than the first,
  because the brush has picked up some dark.
- **Edges in open paint.** A color brought across a wet contour drags a
  little of the other paint with it and the contour softens by a few
  millimeters; a clean dry brush or badger worked along it loses the edge
  further. Over tacky paint the new paint grabs and breaks; over dry paint
  the edge stays exactly where the brush put it.
- **Thin films don't plough.** A bristle (hog 0.2–0.3 mm, sable 0.06–0.12
  mm: `notes/research/oil_paint_physics.md` §2) moving through a film much
  thinner than itself parts and smears it; it can push a bow wave only in
  paint thicker than it rides in. A loaded brush lays its own paint behind
  itself, so it cannot scrape a wet passage down to the ground; only a
  stiff, lean brush pressed hard does that. Stroke-edge ridges come from
  thick paint.
- Simulators: IMPaSTo keeps one volume-mixed wet layer per cell over
  accumulated dry layers (Baxter, Wendt and Lin; Baxter, Liu and Lin 2004,
  https://onlinelibrary.wiley.com/doi/pdf/10.1002/cav.47), which is the
  model this engine had. dAb's brush has a separate *surface* layer over its
  reservoir: the paint that touches is not the paint that is held.

### Findings

1. **A light laid into open paint vanishes.** Touches into an open dark
   mass read light over 0.9 mm² each against 6.7 mm² over dry paint, and
   their cores sit halfway to the dark (clean 0.56, 0.35 for the third
   touch of a load). Into *setting* paint they disappear (0.10 mm²).
   Cause: `Surf::add` mixes every deposit into the pixel's one wet mixture
   by volume. A touch lays about one coat into a body underlayer of 1.5–2.5
   coats, so it becomes a third of a dark mixture wherever it lands. The
   brush's pressure and load play no part. This is also the
   **translucent rock** (body color over a thick open floor became a third
   of the floor's color: "the far wood showed straight through it",
   `notes/amnesia3/easel3_near.md`) and the **fog band** (snow into open
   near-black: `notes/amnesia4/easel4_near.md`). Physics bug. The drying
   notes' open issue ("wet-in-wet within a pixel is still one mixture")
   matters here: it is the main cause.
2. **Setting paint is worse than open.** Setting paint already feels tacky
   (`drying::feel` ramps tack from half the gel point), so the brush
   empties up to 3× faster, and the paint it lays still mixes in fully. The
   stiff light's end drops to 0.23 (0.72 over open paint). Follows from 1.
3. **A sky brought over a wet hill can't cut the hill.** Over dry paint the
   sky laid 3.5 mm into the hill moves the hill 6 mm below the join about
   halfway to the sky (0.45–0.53); over open paint by 0.05–0.08: the sky
   dissolves into the hill. The badger then spreads the mixed band to an
   8 mm soft gray-blue ribbon (`row2_before.jpg`, left two cells): the
   "fog band" again. Edges do soften in open paint (3 → 8 mm), so the lost
   edge exists, but it is made of mud.
4. **Pressing through a thin wet dark exposes the ground.** The thinned
   light pressed over a thin open dark turns pink (`row4_before.jpg`,
   lower left stroke): every bristle pass pushes `push` (0.3 for a hog flat)
   of the film sideways, whatever its thickness, including paint the same
   stroke just laid. Across the ~20 bristle passes a pixel gets, the middle
   of the track empties to the ground and the paint piles at the flanks.
   This is the **plowed river** ("piled 800–950 µm of paint and showed red
   ground through it in streaks", `notes/amnesia4/easel4_green.md`). Physics
   bug: there is no film a bristle can't push below.
5. **What works.** Contact shadows reach their value in every column and
   are softer in open paint (2.2–3.5 mm) than dry (0.4–0.9 mm), as they
   should be. A loaded stiff light over a *thin* dark stays clean at its
   start (0.95) even in open paint, because the dark is only a fifth of the
   mixture. Tacky and dry behave as the drying notes describe.

So the pitfalls table's three failures are engine behavior, made worse by
painter choices that are ordinary for a painter (a thick floor, hatching
into it, a flat pressed along a river).
