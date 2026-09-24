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
1 light touches: clean, first of a load          0.57     0.38     0.99     0.89
  clean, third of a load                         0.36     0.27     0.99     0.89
  area reading light, mm² per touch              0.88     0.10     7.99     6.72
  soft fringe share of the touch                 0.83     0.97     0.25     0.32
  short strokes, clean                           0.97     0.88     1.00     1.00
2 sky over hill: edge 10–90% before badger       9.24     9.24     0.88     0.88
  edge 10–90% after badger (dried)              11.44    10.12     1.32     1.32
  sky 6 mm above the join, pulled to hill (0..1) 0.10     0.09     0.05     0.06
  hill 6 mm below the join, pulled to sky        0.13     0.08     0.67     0.71
3 contact shadow: depth reached (1 = paint)      0.93     0.95     1.00     1.00
  edge to the form above, 10–90%                 2.64     2.20     1.32     0.88
  edge to the ground below, 10–90%               3.08     1.76     0.88     0.44
4 loaded stiff light, clean (start/end)      0.94/0.75 0.86/0.22 0.94/0.37 0.92/0.77
  thinned light pressed, clean (start/end)   0.72/0.58 0.62/0.20 0.66/0.11 0.57/0.26
pitfalls (the later passage 30 min after, or after dry()):
  translucent rock: the rock gets 0.60 of the way to its look over a dried floor
  fog band: snow over the wood's foot gets 0.73 of the way
  plowed river: 90% of the river's track shows the ground (dried first: 0%)
```

(The pitfall lines are the three failures of the sketchbook's pitfalls
table, rebuilt in painter's terms with the style's handlings: a body rock
over a thick body floor, snow brought up over a near-black wood's foot, a
river laid with a pressed hog flat into valley paint.)

Images: the BEFORE halves of `notes/wet/row1_before_after.jpg` …
`row4_before_after.jpg` (1000 px, one row each, columns open · setting ·
tacky · dry), `row1_zoom_before_after.jpg` (row 1 at 2000 px),
`pitfalls_before_after.jpg`.

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
   their cores sit halfway to the dark (clean 0.57, 0.36 for the third
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
   stiff light's end drops to 0.22 (0.75 over open paint). Follows from 1.
3. **A sky brought over a wet hill can't cut the hill.** Over dry paint the
   sky brought 3.5 mm into the hill moves the hill 6 mm below the join about
   two thirds of the way to the sky (0.67–0.71); over open paint by
   0.08–0.13: the sky dissolves into the hill as a 9 mm soft gray-blue band
   before any blending (`row2_before_after.jpg`, left two cells), the "fog
   band" again. The badger then spreads it to 10–11 mm. The lost edge
   exists, but it is made of mud, and there is no found edge in wet paint.
4. **Pressing through a thin wet dark exposes the ground.** The thinned
   light pressed over a thin open dark turns pink (`row4_before_after.jpg`,
   lower left stroke; `plough_before.jpg`): every bristle pass pushes `push` (0.3 for a hog flat)
   of the film sideways, whatever its thickness, including paint the same
   stroke just laid. Across the ~20 bristle passes a pixel gets, the middle
   of the track empties to the ground and the paint piles at the flanks.
   This is the **plowed river** ("piled 800–950 µm of paint and showed red
   ground through it in streaks", `notes/amnesia4/easel4_green.md`). Physics
   bug: there is no film a bristle can't push below.
5. **What works.** Contact shadows reach their value in every column. They
   are softer in open paint (2.2–3.1 mm) than dry (0.4–0.9 mm), but only
   because the shadow mixes into what it touches. A loaded stiff light over a *thin* dark stays clean at its
   start (0.94) even in open paint, because the dark is only a fifth of the
   mixture. Tacky and dry behave as the drying notes describe.

So the pitfalls table's three failures are engine behavior, made worse by
painter choices that are ordinary for a painter (a thick floor, hatching
into it, a flat pressed along a river).

## 2. What changed (engine)

**A surface film per wet pixel** (`wet::Wet::top`, `tlat`, `thide`). Each
pixel's wet paint is now two parts: the **body** (paint laid on a dry
surface, and everything brushes have worked in) and the **surface film**
(the newest stroke's paint laid into wet paint, not yet worked in). This is
the drying notes' open issue "wet-in-wet within a pixel is still one
mixture", and the audit shows it matters: it was the main cause of all
three pitfalls. The rest of the engine sees one film: `vol` is still the
whole thickness (drying, leveling and the brushes' contact read it), and
drying rates and leveling use the volume-mixed properties of both parts
(`Wet::prop`, `wet::whole`).

What brushes do with it (`bristle::exchange`, `Surf::lay/add/stir/take/
take_column/add_body`):
- **Laying.** On a dry pixel paint becomes the body, as before. Into wet
  paint it goes on the surface film. When a *different* stroke lays paint
  there, the old surface film is now underneath and joins the body first,
  so the surface is always the newest stroke's paint. (Without this, a
  field's own overlapping strokes built a thick dark "surface" and the
  next light mixed into it.)
- **Stirring.** Each bristle pass works a share of the surface film into
  the body: `STIR` (0.12) × how far the bristle moved (shear; a tip
  pressed straight down barely stirs) × its reach (pressure) × `0.3 + 0.7
  × stiffness` × the cushion (`1 − 0.8 × load`: a loaded bristle rides on
  its own paint) × the surface paint's own stiffness factor (`1.25 − 0.5 ×
  stiff`: stiff paint holds its place). So a loaded stiff light laid
  lightly stays on top; a lean hog pressed and dragged back and forth
  works it in; a clean badger mixes and redistributes.
- **Pickup** takes the surface film first. It reaches the body only
  through the cushion: a loaded brush lifts about a fifth of the body a
  spent one does.
- **The plough** moves the whole column of paint in a bristle's way,
  surface and body alike, and each part lands where it belongs on the
  neighbor (body to body, surface to surface). Moving the surface first
  made a pixel-scale speckle of dark over light. It no longer pushes a
  film below what a bristle rides on: `PLOUGH_KEEP` (0.6 coats, 15 µm) ×
  `0.3 + 0.7 × stiffness` × `1 − 0.4 × reach`. Coarse stiff hog rides on
  more paint than fine soft hair, and pressure gets closer to the canvas.
  So a thin wet film is smeared, not scraped to the ground; thick paint
  still ploughs into ridges above that floor.
- **Seeing and drying.** `look_px` and the bake composite the body, then
  the surface film over it, with Kubelka–Munk, each its share of the
  leveled thickness (`wet::film_over`). Without a surface film this is
  exactly the old `over_share` path.

Tried and dropped: limiting pickup to about a coat of a thick film. It
doubled every brushed ground (30 µm asked, 64 laid), because handlings
rely on a brush taking up excess paint as it spreads it. The stale
surface film, not pickup depth, was what made touches dirty at high
resolution.

Costs: 44 bytes more per pixel (about 300 MB at 3200 × 2240). Checkpoints
are `PAINTCK7` (they store the surface film); `PAINTCK6` files are refused.
No new painter API: the same verbs behave like paint.

### How a painter uses it (easel)

No new verbs: the same brushes behave like paint in open paint. Choose
the stage by the edge you want (`notes/sketchbook.md` §1).

```lua
-- lights into a dark mass, while the mass is still wet: a full brush, one
-- touch each, light pressure. They stay clean; reload every few touches.
work(crown, {hand="body", color="#26301f"})
b = brush{kind="filbert", width=8}
for i, p in ipairs(lights) do
  if i % 3 == 1 then b:load("#b7bd72", 0.9) end
  b:touch(p[1], p[2], {pressure=0.7, drag={1.5, 0.5}})
end

-- a sky brought down over a hill's top while the hill is wet: a found edge
-- where the brush stops; then lose it where the hill should melt into air
h = brush{kind="flat", width=12}
h:load(sky, 0.8)
h:stroke(ridge_pts, {pressure={0.7, 0.6}})
soft = brush("badger", 22)
soft:stroke(ridge_pts, {pressure={0.35, 0.3}})

-- a contact shadow dragged along a form's foot into the wet ground: it
-- sits at full value; blend only where the contact should soften
s = brush{kind="filbert", width=6}
s:load("#2f2a26", 0.8)
s:stroke(foot_pts, {pressure={0.7, 0.5}})

print(drying(500, 300))   -- "open", "setting", "tacky" or "dry"
```

What mixes: pressure, a lean brush (`load` 0.2–0.4), a stiff hog, dragging
back and forth, `blend`, `scrub`. What stays on top: a full load
(0.8–1.0), a soft brush, a light touch, one pass.

## 3. Evidence (after)

`cargo paint study_wet` (1000 px):

```
(units: mm at the canvas's 440 mm width)         open  setting    tacky      dry
1 light touches: clean, first of a load          0.88     0.93     0.99     0.94
  clean, third of a load                         0.88     0.91     0.99     0.94
  area reading light, mm² per touch              4.99     4.91     7.36     5.67
  soft fringe share of the touch                 0.37     0.33     0.25     0.28
  short strokes, clean                           1.00     1.00     1.00     1.00
2 sky over hill: edge 10–90% before badger       2.64     1.32     0.88     0.88
  edge 10–90% after badger (dried)              11.44    10.56     1.32     1.32
  sky 6 mm above the join, pulled to hill (0..1) 0.05     0.05     0.02     0.03
  hill 6 mm below the join, pulled to sky        0.14     0.10     0.62     0.65
3 contact shadow: depth reached (1 = paint)      0.99     1.00     1.00     1.00
  edge to the form above, 10–90%                 1.32     0.88     1.32     0.88
  edge to the ground below, 10–90%               0.88     0.88     0.88     0.88
4 loaded stiff light, clean (start/end)      0.98/0.93 1.00/0.61 0.98/0.89 0.98/0.95
  thinned light pressed, clean (start/end)   0.82/0.65 0.87/0.53 0.81/0.62 0.76/0.69
pitfalls (the later passage 30 min after, or after dry()):
  translucent rock: the rock gets 0.82 of the way to its look over a dried floor
  fog band: snow over the wood's foot gets 0.87 of the way
  plowed river: 18% of the river's track shows the ground (dried first: 0%)
```

What I see (images in `notes/wet/`, BEFORE above AFTER in each):
- `row1_before_after.jpg`, `row1_zoom_before_after.jpg`: touches of a
  loaded light into an open or setting dark mass now read as lights (0.88
  and 0.93 clean, about 5 mm² each; before 0.57 and 0.38, under 1 mm²),
  with slightly broken edges where the pressed tip pushed the wet dark
  aside. At 2000 px they are the same (0.89 open, 0.95 setting; before
  0.35 at 2000 px, so the old behavior was also resolution-dependent).
- `row2_before_after.jpg`: over open paint the sky now makes a found but
  soft edge (2.6 mm against 0.9 over dry), with some hill dragged into it,
  instead of a 9 mm gray-blue band. The clean badger then loses it (11 mm).
  Found by default, lost when worked: the painter decides. The firm loaded
  hog still stirs part of its sky into the wet hill, so it covers the hill
  less than over dry paint (0.14 vs 0.62). That is what a pressed hog does
  in wet paint; a lighter touch covers more.
- `row3_before_after.jpg`: the contact shadow sits at full value in wet
  paint with an edge as crisp as the brush made it (0.9–1.3 mm). Oil
  doesn't bleed like watercolor: a wet contour softens when it's worked
  (blend, badger, a dry brush), not by itself.
- `row4_before_after.jpg`, `plough_before.jpg` / `plough_after.jpg`: a
  pressed stroke through a thin wet dark no longer scrapes to the pink
  ground (81% of the tracks showed the ground before, 3% after, in
  `wet::tests::a_pressed_stroke_does_not_plough_a_thin_film_to_the_ground`);
  the field's own strokes no longer leave ground-colored furrows either.
- `pitfalls_before_after.jpg`: the rock over an open floor gets 0.82 of the
  way to its look over a dried floor (was 0.60), with streaks of the floor
  worked up into it rather than an even translucency. The snow over the
  wood's foot gets 0.87 (was 0.73), with dark streaks instead of a gray
  band. The river shows the ground in 18% of its track (was 90%), only at
  its spent tail (see known issues).

**Benchmarks** (`easel run <log> --width 1000`, old binary vs new): they do
**not** render identically; this branch changes output on purpose.
- `l5_near.lua`: mean difference 4.5 of 255, 15% of pixels over 8, max 176.
  Images: `l5_near_before.jpg`, `l5_near_after.jpg`, and a 3200 px crop of
  the wood's foot (x 60–360, y 250–400) in
  `l5_near_3200crop_before_after.jpg`: the pale veil blob over the wood is
  gone and the trunks and hatched needles read through; the veil that sank
  into the wet wood as fog now lies where it was laid.
- `l3_green.lua`: mean 2.4, 4% over 8, max 121. `l3_green_before.jpg`,
  `l3_green_after.jpg`, and the hedge on the hill's edge at 3200 px
  (x 440–760, y 430–580) in `l3_green_3200crop_before_after.jpg`: the
  hedge's and oak's light clumps, laid into their wet dark masses, now stay
  light instead of sinking. They read more as separate lights, and in the
  hedge rather evenly sized and spaced (the log's recipe was tuned while
  lights sank; a painter would now use fewer).

The golden scene is re-recorded (`UPDATE_GOLDEN=1`).

Tests (`wet::tests::wet_on_wet`): a loaded light laid lightly into open
dark keeps over 0.85 of its lift over the dark dried, and a lean pressed
brush works in more; a pressed stroke doesn't plough a thin film to the
ground; the surface film dries over the body (Kubelka–Munk), not mixed;
the surface film stays finite and within the film. `probe_touch` (ignored)
prints a touch's lift at 500–3200 px.

## 4. Known issues and next steps

- **The brush is still one mixture per bristle.** A loaded bristle holds
  about 50 times what a touch lays, so a few touches into wet dark barely
  dirty it (third touch as clean as the first). A real tip dirties faster
  than its belly. Next: a surface layer on the bristle (as in dAb), fed from
  the reservoir.
- **Two layers, not a stack.** A third wet stroke folds the second into the
  body, which is then a mixture. That is enough for "light into dark" and
  "sky over hill"; glazing wet over wet over wet would need more.
- **A spent hog pressed hard still squeegees** wet paint down to its plough
  floor (0.3 coats), and the ground shows through that thin a film at the
  river's spent tail. Physically plausible for a nearly dry, stiff brush at
  pressure 0.9; with a load it covers.
- **Setting paint grabs** (tack from half the gel point, `drying::feel`), so
  a brush empties early over it (stiff light ends at 0.61 against 0.93
  open). That is the drying model's design, not this change.
- **Edge widths depend on resolution** for the badger (sky edge after the
  badger: 11.4 mm at 1000 px, 7.5 mm at 2000 px; the old engine was similar,
  11.4 and 6.6). Touches and stroke cleanliness now agree across resolutions.
- The stir and cushion constants are estimates (`[E]`), set so the study's
  gestures behave as the alla prima rules describe; there are no
  measurements of mixing depth to calibrate them against.
