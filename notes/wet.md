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

Costs: 44 bytes more per pixel (about 315 MB at 3200 × 2240; the surface
film's volume, pigment and properties). Pass 2's set-aside film added 8
more (52, about 373 MB) until the maintenance round (§9) moved them out
of the canvas; they are now 4 bytes of stroke scratch that copies of the
canvas and checkpoints don't carry. Checkpoints
are `PAINTCK8` (they store the surface film, after main's hand-time block
`PAINTCK7`); older files are refused.
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

## 3. Evidence (after the first pass; see §5 for the current engine)

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

## 5. Second pass: the lab painters' failures, veins, blending

Main was merged first (r6-time's hand time and the easel fix). The lab
painters had worked wet on purpose on the old engine and reported
failures (`notes/lab/*.md` on their branches). I rebuilt each mechanism as
a panel in `paintings/src/bin/study_wet_lab.rs` (`cargo paint
study_wet_lab`), with the handling they used, measured on main's engine
and this branch's, and replayed `notes/lab/water_B.lua` for the tramlines.

### What changed in the engine

- **The earlier surface film is settled at the stroke's end** (`Surf::lay`,
  `Surf::settle_mid`). The first pass buried an earlier stroke's surface
  film in the body as soon as a new stroke touched the pixel. A new
  stroke's thin edge over an earlier light stroke then showed the dark body
  through it: the **web of dark veins** on the rock and the snow in the
  first pass's `pitfalls_before_after.jpg`. Now the old film is set aside
  and, when the stroke ends, buried in proportion to how much of the new
  paint covers it (`BURY` = 0.6 coats hides it all). The rest stays in the
  surface film, mixed with the new paint. A film like the body under it is
  always buried; that changes nothing but frees the surface.
- **The plough goes with the paint's stiffness** (`PLOUGH_STIFF`). Only
  paint with a yield stress keeps a bow wave; medium-rich paint flows round
  the bristle and back into its furrow (a glaze levels bristle-scale
  ridges in about 0.06 s: `notes/research/oil_paint_physics.md` §1). Paint
  from medium 0.85 (stiffness about 0.02) is pushed at a tenth of the old
  rate, a thin sky at medium 0.3 at about 0.6, stiff paint as before.
- **A blender or a nearly spent bristle drags what it carries into the
  film** instead of laying it on top (`drag_in`). A clean blender (`lay`
  0) holds no charge; the paint it moves was picked up a moment before.
  In the first pass it laid that paint as a surface film, and across a wet
  seam it left a pale lace of sky on the dark and no blend. The cushion
  also scales with how much paint the tool is made to carry.
- **Stirring** follows the bristle's deposit (so what a bristle lays is
  worked by the same pass). It is stronger for thin surface films: a film
  much thinner than a coat is no layer of its own once it is sheared. It
  depends less on bristle stiffness and pressure: any hair shears a surface
  film.

### The lab failures, one by one

Measured in `study_wet_lab` (1000 px; ground = share of the worked area
within ΔE 0.06 of the ground color; texture = mean |L − L blurred over 2
units| ×1000; the brush-free `glaze()` verb gives about 1):

```
                                     main engine          this branch
0 blend across a wet seam            ground 0.0% tex  4.8  ground 0.0% tex  4.8
1 glaze hand over open thin sky      ground 42%  tex 20.4  ground 0.1% tex 11.9
2 glaze hand over tacky sky          ground 0.0% tex 32.3  ground 0.0% tex  4.0
3 glaze hand over dry sky            ground 0.0% tex 42.2  ground 0.0% tex  6.8
4 glaze() verb over dry sky (ref.)   ground 0.2% tex  1.4  ground 0.0% tex  1.0
5 thin stroke into open water        ground 0.0% tex  6.2  ground 0.0% tex 12.1
6 blender over a lean veil, open     ground 0.8% tex  5.2  ground 0.0% tex  6.1
```

Images: `lab_failures_before_after.jpg` (all panels),
`lab_glaze_zoom_before_after.jpg` (panels 1–3 at 2000 px),
`lab_water_B_3200crop_before_after.jpg` (the replayed water log).

**Lifting or ploughing to the ground**
- *A glaze-hand stroke over thin open sky lifts it to the ground* (lab 3):
  reproduced (42% of the area). **Fixed** (0.1%): the plough no longer
  bulldozes fluid glaze paint, and the thin sky under it stays.
- *A clean blender or badger along a wet seam, or over a lean veil,
  exposes the ground* (labs 1 and 3): reproduced only weakly (0.8% over a
  lean veil). **Fixed** (0%), by the same fluid plough and by blenders
  dragging their paint in. A blender across a seam fuses it again (texture
  4.8, as on main) where the first pass left a lace.
- *A filbert into thin wet sky scrapes it to the ground* (lab 2): the same
  mechanism as the plowed river and row 4 of `study_wet` (a bristle
  pushing a thin film aside), fixed in the first pass by the plough floor
  and now also by the fluid plough. Not replayed separately.
- *A light into an open dark ploughs a camouflage mottle with ground
  showing* (lab 1): **fixed in the first pass** (lights stay on top, clean:
  `row1_zoom_before_after.jpg`). *Heaps of 2.8–3.5 mm* at one spot: **not
  fixed**. Found the mechanism: a bristle's contact rises with the wet
  film's thickness (`base + 0.35 × vol`), so it lays more where paint is
  already thick. A clean blender over a seam heaped the dark side from 2.2
  to 4–6 coats while conserving paint overall. Blenders now mix what they
  heap, so it doesn't show there. A deposit that falls as the wet film
  under it thickens (film splitting) would fix it, but every handling's
  thickness is calibrated on the present deposit (the brushed-ground test
  doubled when I tried limiting pickup instead). Next step.

**Pushed aside instead of mixed**
- *Tramlines from a thin stroke into wet water* (lab 2): replayed
  `water_B.lua` at 3200 px (units 380–700 × 520–660). **Fixed**: the sheen is
  one faint line where main has two parallel pale lines. The fluid plough
  no longer piles the water into two ridges beside the stroke. (The reeds'
  dark touches on the spit now stay where they were laid, too.)
- *A dark over open light pushes the light to the stroke's edges as a pale
  lace ridge* (labs 1 and 3): the same plough, now weaker in fluid paint.
  Not measured separately.
- *A dark stroke through wet light drags the light 40–60 units at nearly
  full strength* (lab 2): **not addressed**. That is how far a brush
  carries what it picked up (its `run`), not the wet film. Open.

**Setting and tacky**
- *Glaze-hand strokes over tacky sky craze into a mosaic* (lab 3):
  reproduced (texture 32). **Fixed** (4.0): the cells were fresh glaze
  ploughed into rims around stick-slip patches.
- *Glaze-hand strokes on dry sky lift each other into a net of dark rims*
  (lab 3): reproduced (42). **Fixed** (6.8; the `glaze()` verb gives 1).
  Each new stroke ploughed the previous, still-fluid glaze stroke into a
  rim.
- *A dark over setting light comes out a translucent gray streak* (labs 1
  and 2): **not fixed**. Setting paint already feels tacky
  (`drying::feel` ramps tack from half the gel point), so the brush
  empties within a few units, and a spent brush's last paint is dragged
  in. The loaded stiff light over setting dark in `study_wet` row 4 ends
  at 0.09 clean (0.88 over open paint). The suspect is the tack ramp in
  the drying model; I left it, since the time stream owns drying.
- *Sky laid into setting dark: lumpy puffs 246–625 µm* (lab 1): the heaping
  above, plus the fast emptying. Open.
- *A thin veil over tacky paint lies as a flat, hard-edged slab the shape of
  its mask* (lab 1): not addressed. Over tacky paint nothing mixes, so a
  veil's edge is where its mask put it, as over dry paint. That is the
  mask's edge, which the painter controls (`hug=false`, a softened mask).
- *A dark on tacky paint skips with pale flecks* (lab 3): stick and slip,
  as designed in `notes/drying.md`. Left.

### The veins (my first pass's own artifact)

`veins_before_after.jpg`: the rock over an open floor. The first pass
shows a web of sharp dark lines along the edges of the rock's strokes;
now the dark comes up as soft, broken grays where the brush thinned or
dragged. The pitfall scene now reads: rock 0.80 of the way to its look
over a dried floor, snow 0.78, river 25% of its track showing the ground
at the spent tail (main: 0.60, 0.73 and 90%; first pass: 0.82, 0.87 and
18%, but with the veins). `pitfalls_before_after.jpg` is updated.

### The blocky patches at l5_near's wood foot

They are paint the log lays. Chunk 7 paints the wood's floor between tree
rows 3 and 2 as `work(fl, {hand="body", coverage=3, angle=1.5, ...})`:
vertical body-color strokes, into the still-wet rows. On main's engine
they mixed into the wet needles as a muddy veil; now they stay where they
were laid and read as the flat-brush marks they are. Rendered without that
one `work` call, the patches are gone
(`l5_near_3200crop_floor_pass_check.jpg`: the log as it is above, the
floor pass removed below). It's the log's problem. A painter would now lay
the wood's floor as a thin, soft tone, or before the rows. The blocky top
edge of the snow bank below is chunk 8's own body strokes, the same on
both engines.

### Current numbers (`study_wet`, 1000 px)

```
(units: mm at the canvas's 440 mm width)         open  setting    tacky      dry
1 light touches: clean, first of a load          0.87     0.93     0.99     0.94
  clean, third of a load                         0.87     0.90     0.99     0.94
  area reading light, mm² per touch              5.32     5.10     7.37     5.70
  short strokes, clean                           1.00     1.00     1.00     1.00
2 sky over hill: edge 10–90% before badger       4.40     2.20     0.88     0.88
  edge 10–90% after badger (dried)              11.44    11.00     1.32     1.32
  hill 6 mm below the join, pulled to sky        0.10     0.06     0.63     0.66
3 contact shadow: depth reached (1 = paint)      0.98     1.00     1.00     1.00
  edge to the form above, 10–90%                 1.32     1.32     0.88     0.88
4 loaded stiff light, clean (start/end)      0.99/0.88 0.99/0.09 0.98/0.88 0.98/0.94
  thinned light pressed, clean (start/end)   0.61/0.44 0.86/0.26 0.91/0.70 0.85/0.74
```

At 2000 px the touches read 0.89/0.95 clean (open/setting).

### Benchmarks and tests

`l5_near.lua` and `l3_green.lua` at 1000 px do **not** render identically
to main (on purpose). l5_near: mean difference 4.5 of 255, 14% of pixels
over 8, max 176. l3_green: mean 3.1, 7% over 8, max 121. Images:
`l5_near_after.jpg`, `l3_green_after.jpg` (with the `_before.jpg`s),
`l5_near_3200crop_before_after.jpg`, `l3_green_3200crop_before_after.jpg`.
Re-recorded because output changes on purpose: the golden scene, and the
replay hashes in `crates/easel/tests/hand_time.rs` (both profiles;
`PRINT_HASHES=1` prints them). `tally`'s aging test now fingerprints the
wet film too: with the new engine nothing in its 160 minutes of hand time
gels, so the dry picture alone no longer differs.

## 6. Third pass: the brush's tip

A blind panel (2 × Gemini 3.8 Flash, 2 × gpt-6-astra) judged main against
pass 2 on six pairs from identical logs. Pass 2 won foliage_C and l5_near
4–0 (lights don't sink, the wood reads); main won rock_B 4–0 and sky_B,
water_B, l3_green 3–1. The panel's read: on pass 2 later marks stay
discrete, firm and opaque; where passages should melt (cloud undersides,
reflections, the rock's terminator and base, the hedge) they read as
stepped, stamped stroke ends.

### What changed

- **A tip layer on each bristle** (`Bristle::tip`, part of its load).
  What a bristle picks up from the wet film stays on its surface and is
  laid first; it works into the reservoir as the brush travels
  (`TIP_RUN`, 15 units e-folding). A dip in the pile coats the tip
  afresh; wiping takes the tip too. So a loaded stiff light touched into a
  wet dark is clean on the first touch and dirtier on the following ones
  (`wet::tests::wet_on_wet::touches_of_one_load_dirty_as_they_go`: the
  first touch is as light as a fresh load's at the same spot, the next
  four average under 0.96 of it, a reload is clean again). A stroke
  dragged along a wet contour lays the under-paint it picked up just
  behind where it picked it up.
- **A pressed touch is cushioned less** when it picks up (the tip is
  pushed into the wet paint and splits off it as it lifts).
- **Fluid paint on the brush picks up more** (pickup × `1.3 − 0.6 ×
  stiffness` of the bristle's paint): a medium-rich brush integrates more
  than a stiff full one.
- **Glancing bristles in a moving stroke drag their paint in**
  (`GLANCE`): a bristle that barely touches (the lift-off, the edges) lays
  its thin film into the wet surface instead of on it, so stroke ends
  feather into wet paint. Touches are exempt (pressed straight down).

### Evidence

`notes/wet/pass3_<pair>.jpg`: the six panel crops (3200 px, the panel's
windows), stacked main / pass 2 / pass 3.

Mean difference between pass 2 and pass 3 per crop (of 255): sky_B 2.5,
water_B 2.0, rock_B 1.2, foliage_C 0.8, l5_near 1.9, l3_green 0.9
(main to pass 3: 4.5, 3.6, 3.1, 2.2, 8.3, 3.6). **Pass 3 moves the
pictures only a little.** The cloud bellies in sky_B feather slightly
more; l5_near's wood and foliage_C's lights are unchanged (the wins hold).
The rock's scalloped terminator and blunt stroke ends and the hedge's
separate lights are about as they were.

Why so little: what a bristle picks up is a few percent of what a loaded
bristle lays per step (its pickup is cushioned by its own load), so the
tip colors the stroke only where the load runs low. The blunt ends at the
rock's terminator are the flat brush's own footprint ending. Main softened
them by mixing every mark into the wet paint, which is also what sank the
lights and muddied the wood. Getting both needs the stroke's end itself to
change: a flat lifting off rolls onto its edge and drags. That is a change
to the gesture (`Gesture` ramps and the flat's orientation as pressure
falls), not to the wet film, and I didn't attempt it in this pass.

`study_wet` after pass 3: touches 0.88 clean first, 0.83 third (open);
rock 0.69 and snow 0.74 of the way to their look over dried paint (more
integrated than pass 2's 0.80/0.78); the river's spent tail shows the
ground in 35% of its track (pass 2: 25%). The lab failures stay fixed
(`study_wet_lab`: glaze hand over open sky 0% ground, tacky texture 4.2,
dry 7.7).

Tests: the golden scene and the hand-time replay hashes are re-recorded
(output changes on purpose).

## 7. Review fix: paint conservation through a set-aside film

A code review found that the surface film an earlier stroke left, set
aside when a new stroke first lays paint on it (`Surf::lay`, pass 2), was
treated as body paint until the stroke ended. Its volume stayed in `vol`
with no pigment of its own, so:
- stirring and `add_body` mixed against an inflated body;
- pickup and the plough could carry it away with the *body's* pigment;
- `settle_mid` then restored the saved film at its full volume, however
  much of it had gone.

Total volume could stay right while the pigment amounts drifted.

**Fix** (`bristle.rs`, `wet.rs`). While a stroke runs, the set-aside film
is an explicit layer: `Wet::mid` holds its remaining volume per pixel and
`Wet::midx` points to its pigment and properties, kept with the stroke.
- Body volume is `vol − top − mid` everywhere (`Surf::body`).
- Pickup takes the surface film, then the set-aside film, then the body,
  each with its own pigment.
- The plough moves all three layers in proportion; the set-aside part
  lands on the neighbor's surface.
- Settlement uses what is left of the set-aside film and computes the body
  before clearing it. My first version of the fix made that last mistake
  itself, and the new test caught it.

`mid` is always 0 between strokes, so checkpoints don't store it.

**Test** (`bristle::conservation::a_third_stroke_over_two_wet_colors_conserves_paint`).
Every moment is summed over the canvas's wet paint and every bristle's
reservoir and tip, before and after each stroke, touch and clean-brush
pass: the volume, and the volume times each latent pigment component,
scattering, stiffness and drying rate. Each must hold within 2e-4 of its
own total; f32 mixing rounds at about 1e-6 a step.

The scene:
- a dark field;
- a light field over half of it;
- a third pigment dragged back and forth across both, loaded and lean;
- a clean hog through it after each stroke;
- a touch into it after each stroke.

The test counts the pickups and plough moves that take set-aside paint
(more than 100 of each) and checks that every set-aside film is settled
when its stroke ends. On the reviewed code it fails at the first light
stroke over the wet dark: pigment 0 is off by 2.6e-3 of its total.

**Effect on pictures** (3200 px crops of the panel's windows, pass 3
against the fix; mean difference out of 255):

| crop | mean | max |
|---|---|---|
| rock_B | 1.6 | 131 |
| foliage_C | 0.7 | 157 |
| l5_near | 1.0 | 182 |
| l5_near, whole at 1000 px | 0.4 | 138 |

None of these is visible at viewing size: the rock's terminator, the
lights and the wood are as they were. Stacked pass 3 / fix:
`notes/wet/reviewfix_rock_B.jpg`, `reviewfix_foliage_C.jpg` and
`reviewfix_l5_near.jpg`. The golden scene and the hand-time replay hashes
(both profiles) are re-recorded because the fix changes output.

**Checkpoint format.** Merged with main's hand-time checkpoint
(`PAINTCK7`, r6-time's review fix): the surface film is now `PAINTCK8`,
appended after the hand-time block; older files are refused.

## 8. Wet control experiment (Alice's decision: r6-wet stays unmerged; run the experiment)

Main was merged first (relief default 0.06, r6-oak, r6-time's review
fixes). The film format stays `PAINTCK8`. The replay hashes are
re-recorded for the combination.

### The lift-off

`bristle::lift_off`, applied over a stroke's release ramp (the last
`ramps.1` of it, where the pressure falls):
- a **flat** rolls onto its chisel edge (its wide axis turns toward the
  travel);
- a **round** draws in to its point (blunt rounds; pointed ones already
  do);
- a **filbert** does half of each;
- the trailing hairs drag longer as the handle rises (`LIFT_DRAG`).

It is geometry and pressure, not a fade. **It is the default**, because
it is how a brush that lifts off moves, and it is tied to the release
ramp the handlings already have. A painter who wants a square, stamped
end sets the release to 0 (`ramps={a, 0}`): pressing to the end and
lifting straight off.

### The controlled study

`paintings/src/bin/study_wet_control.rs`, built unchanged on three
engines:
- **main** at `dc79ebb` (a worktree in scratch);
- **r6-wet as it was** (this branch without the lift-off);
- **r6-wet + lift-off**.

Rows:
- **A.** A loaded stiff light accent (a filbert touch and a short stroke)
  into a dark.
- **B.** A hog flat with the light dragged along a light/dark contour.
- **C.** A flat's light stroke ending in a dark.
- **D.** A glaze-hand veil over a thin sky.

Columns: stage (o open, s setting, t tacky, d dry) × load (F 0.9, L 0.3)
× pressure (l 0.45, p 0.9). The metrics:
- retained light: 1 is the light's own paint;
- exposed ground: % of the track within ΔE 0.06 of the ground;
- edge or end width: 10–90%, mm on the 440 mm canvas.

Rows with no exposed ground anywhere are left out below. Row A, B and C
never showed ground on any engine.

### main, 1000 px
```
                                   oFl oFp oLl oLp sFl sFp sLl sLp tFl tFp tLl tLp dFl dFp dLl dLp
A accent: retained light           0.62 0.65 0.49 0.52 0.68 0.58 0.52 0.43 0.99 0.99 0.78 0.92 0.84 0.96 0.53 0.75
B contour: edge 10-90% mm          0.88 0.88 3.96 5.28 1.32 1.32 3.96 4.84 0.44 0.44 0.44 0.88 0.44 0.44 0.44 0.44
C flat end: end 10-90% mm          1.76 1.32 3.96 3.52 3.08 2.20 4.40 9.24 0.44 0.88 0.44 0.88 0.44 0.88 0.88 0.88
C flat end: retained light         0.95 0.94 0.83 0.83 0.88 0.83 0.80 0.82 0.98 0.98 0.83 0.84 0.94 0.93 0.90 0.72
D glaze veil: ground %               17   13   41   36    1    0   12    6    0    0    0    0    0    0    0    0
```
### main, 3200 px
```
                                   oFl oFp oLl oLp sFl sFp sLl sLp tFl tFp tLl tLp dFl dFp dLl dLp
A accent: retained light           0.51 0.56 0.47 0.38 0.63 0.46 0.37 0.39 1.00 1.00 0.95 0.98 0.96 0.99 0.80 0.90
B contour: edge 10-90% mm          0.41 2.47 1.10 2.47 0.55 2.47 1.92 2.34 0.14 0.14 0.14 0.28 0.14 0.14 0.14 0.14
C flat end: end 10-90% mm          1.92 0.82 3.03 0.69 1.51 2.06 2.20 7.56 0.28 0.28 0.28 0.28 0.28 0.28 0.14 0.28
C flat end: retained light         0.79 0.77 0.81 0.34 0.76 0.73 0.42 0.75 1.00 1.00 1.00 1.00 1.00 1.00 0.99 0.99
D glaze veil: ground %                0    0    6    1    0    0    0    0    0    0    0    0    0    0    0    0
```
### wet, 1000 px
```
                                   oFl oFp oLl oLp sFl sFp sLl sLp tFl tFp tLl tLp dFl dFp dLl dLp
A accent: retained light           0.82 0.94 0.53 0.65 0.99 0.96 0.74 0.70 0.99 1.00 0.83 0.96 0.89 0.98 0.60 0.87
B contour: edge 10-90% mm          0.44 0.44 1.32 1.32 0.88 0.44 1.32 1.76 0.44 0.44 0.44 0.44 0.44 0.44 0.44 0.44
C flat end: end 10-90% mm          1.76 0.88 3.08 4.84 1.76 1.32 3.96 4.84 0.44 0.88 0.44 0.88 0.44 0.88 0.88 0.44
C flat end: retained light         0.99 0.98 0.94 0.85 0.99 0.99 0.90 0.99 1.00 0.99 0.96 0.95 0.98 0.98 0.95 0.92
```
### wet, 3200 px
```
                                   oFl oFp oLl oLp sFl sFp sLl sLp tFl tFp tLl tLp dFl dFp dLl dLp
A accent: retained light           0.70 0.91 0.41 0.48 0.98 0.89 0.73 0.63 1.00 1.00 0.96 0.98 0.97 0.99 0.81 0.91
B contour: edge 10-90% mm          0.28 0.28 1.92 2.34 0.28 0.28 1.79 1.92 0.14 0.14 0.14 0.28 0.14 0.28 0.28 0.14
C flat end: end 10-90% mm          3.30 1.38 4.68 7.15 3.71 2.89 3.44 6.46 0.28 0.28 0.28 0.14 0.28 0.28 0.28 0.28
C flat end: retained light         0.98 0.91 0.75 0.56 0.99 0.93 0.79 1.00 1.00 1.00 1.00 1.00 1.00 1.00 1.00 0.99
```
### lift, 1000 px
```
                                   oFl oFp oLl oLp sFl sFp sLl sLp tFl tFp tLl tLp dFl dFp dLl dLp
A accent: retained light           0.82 0.95 0.52 0.65 0.99 0.97 0.76 0.71 0.99 1.00 0.86 0.96 0.91 0.98 0.52 0.88
B contour: edge 10-90% mm          0.44 0.44 1.32 1.32 0.44 0.44 1.32 1.76 0.44 0.44 0.44 0.44 0.44 0.44 0.44 0.44
C flat end: end 10-90% mm          2.20 1.76 3.96 4.40 2.20 1.76 2.20 4.84 1.32 1.32 1.32 1.32 0.88 1.32 0.88 1.32
C flat end: retained light         0.99 0.98 0.93 0.85 0.99 0.98 0.90 1.00 1.00 0.99 0.96 0.95 0.98 0.98 0.96 0.92
```
### lift, 3200 px
```
                                   oFl oFp oLl oLp sFl sFp sLl sLp tFl tFp tLl tLp dFl dFp dLl dLp
A accent: retained light           0.70 0.91 0.40 0.48 0.98 0.89 0.72 0.64 1.00 1.00 0.96 0.98 0.97 0.99 0.86 0.90
B contour: edge 10-90% mm          0.28 0.28 1.92 2.34 0.28 0.28 1.65 1.92 0.14 0.14 0.14 0.28 0.14 0.28 0.28 0.14
C flat end: end 10-90% mm          1.65 0.55 1.51 0.82 2.61 0.82 1.10 5.91 0.28 0.41 0.41 1.24 0.28 0.14 0.28 0.14
C flat end: retained light         0.98 0.91 0.75 0.55 0.99 0.93 0.78 1.00 1.00 1.00 1.00 1.00 1.00 1.00 1.00 0.99
```

### What it shows

- **Clean accents: pass.** A loaded stiff light into open paint keeps 0.82
  (light) and 0.94 (firm) of its light at 1000 px, 0.70 and 0.91 at 3200 px.
  Main keeps 0.62/0.65 and 0.51/0.56. Into setting paint: 0.99/0.96 against
  main's 0.68/0.58. A lean accent integrates on every engine (0.4–0.75).
- **Exposed ground: pass.** Main's glaze-hand veil over open sky shows the
  ground in 13–41% of its track at 1000 px (0–6% at 3200). Both wet
  engines show none, anywhere.
- **Deliberately softened edges: partial pass.**
  - On the wet engines the load controls it. A full flat dragged along a
    wet contour keeps it crisp (0.44 mm at 1000 px, 0.28 at 3200); a lean
    one softens it (1.3–1.8 mm, 1.7–2.3 at 3200).
  - Main softens more (4–5 mm at 1000 px) and softens even a full load
    (0.9–1.3 mm).
  - So the wet engine softens only what the painter works lean. Its
    softest deliberate edge is about a third of main's accidental one at
    1000 px, and about the same at 3200 (2.3 against 2.5).
- **The lift-off changes the shape of a stroke's end, not its softness.**
  - A flat ending in wet paint now ends tapered to its chisel edge
    instead of square (`owner_control_detail_open.jpg`, row C).
  - Measured along the centerline, its 10–90% end is shorter at 3200 px
    (open: 1.65/0.55 mm against 3.30/1.38 without it), because the end
    narrows instead of fading.
  - At 1000 px over tacky and dry paint the ends read a little longer
    (1.3 mm against 0.4–0.9), for the same reason.

### Repaints

The logs were adapted, not replayed: each copy changes only the passage
that relied on paint sinking, with the same number of chunks
(`notes/wet/repaints/*_wet.lua`, each saying what changed):
- **sky_B:** chunk 3 blends the cloud banks' edges into the open sky.
- **water_B:** chunk 5 blends the reflections level more (coverage 3.5
  for 2.0).
- **rock_B:** chunk 3 blends the lit face into the setting shadow along
  the terminator.
- **l3_green:** chunk 11 lays the wood's lights lean and fewer (load 0.3,
  coverage 1.6 for 2.2).

Rendered on r6-wet + lift-off against main's own log on main (at
`dc79ebb`). The sheets `notes/wet/owner_repaint_<S>.jpg` are labeled A
and B only.

**Key** (don't read before judging):
- sky_B: A = r6-wet repaint, B = main
- water_B: A = main, B = r6-wet repaint
- rock_B: A = r6-wet repaint, B = main
- l3_green: A = main, B = r6-wet repaint

My own look, as a reading rather than a verdict:
- **rock_B:** the adapted terminator is softer than the plain replay's
  and closer to main's.
- **sky_B:** the bank edges are softer, but main's banks still melt more
  at the bellies.
- **water_B:** the reflection's teeth under the spit remain; more
  blending didn't remove them.
- **l3_green:** the wood's lights are smaller and lower in contrast, but
  still read as separate dots against main's merged texture.

**The wins** (`liftoff_regression_foliage_C.jpg`,
`liftoff_regression_l5_near.jpg`; main / r6-wet as it was / r6-wet +
lift-off, 3200 px crops):
- The lift-off leaves them nearly untouched: a mean difference of 0.6
  (foliage_C) and 1.4 (l5_near) out of 255.
- Main's r6-oak merge regrew foliage_C's crowns. On the new geometry main
  and r6-wet differ little there (mean 1.6); l5_near still differs
  clearly (8.1).

### Recommendation

- **Keep the two-layer film and don't merge it as the default yet.**
  Accents and ground exposure pass clearly. Soft edges are controllable
  but modest, and only the painter's lean or blended strokes make them.
  Passages that main softened for free (cloud bellies, reflections, a
  hedge's lights) need deliberate work on this engine and still don't
  fully melt with one extra blend.
- **The lift-off is a reasonable default for stroke shape**, but it
  doesn't close the softness gap.
- **The next thing to try** is a real softening mechanism for fluid paint:
  the leveling of a wet stroke's thin end into the paint around it (the
  Orchard leveling the physics notes describe, applied laterally at a
  wet-into-wet stroke end), not more geometry.
- **Or offer the engine as an opt-in** ("wet control" as a style or
  canvas setting), so paintings that want main's melting keep it, as
  the advice suggests: capability first, default later.

## 9. Maintenance round (the thermos review: S2, B3, B7, B8, B9, S6)

The eight-reviewer review (`notes/round6/thermos.md` on main) asked for
the film to have one owner before more work lands on it. What changed:

**Structure (S2), behavior-identical.** `l5_near` and `l3_green` at 1000
px `cmp` byte-identical to `7a1340f` after it; the golden scene and the
replay hashes passed unchanged.
- `wet::Layer { v, lat, hide }`: one part of a film. The surface film
  (`Wet::top`, was `top/tlat/thide`), a bristle's tip (`Bristle::tip`),
  a film set aside under a stroke, and the `film::Parcel`s that pickup
  and the plough move (surface, set aside, body) are layers.
- `vol` stays the stored whole and the body its remainder, computed in
  one place (`film::Stroke::body`). Storing the body as its own layer
  would have been cleaner, but every stroke would then round differently
  (`(b + t) + v` against `b + (t + v)`), so it wasn't done in a
  behavior-preserving pass. Same for the bristle (`vol` plus `tip`).
- `film.rs` (new): the raw view `Surf` and `Stroke`, one stroke's hold on
  the film. Every change a brush makes to the film goes through it
  (`lay`, `add`, `add_body`, `stir`, `take`, `take_column`, `land`). The
  set-aside film is a reservation the stroke owns and settles when it is
  dropped, so drags and touches (and any later stroke path) settle alike.
  Debug builds assert that nothing is set aside when time passes (`wait`)
  or a checkpoint is written.
- `exchange.rs` (new): `exchange`, and `contact()`, a pure function
  computing a bristle's physics once per step (contact threshold,
  hunger, the pickup's reach through the cushion, drag-in, stir, the
  plough's floor and its stiffness gate). `enum Mode { Drag, Touch { dep } }`
  replaces `dep: Option<f32>`.
- The test-only global `MID_MOVES` is gone: a stroke reports how often
  its pickup and plough took set-aside paint (`Canvas::drag_counted`,
  `touch_counted`), and the conservation test sums its own strokes.
- Lines: `bristle.rs` 2357 → 1638; `exchange.rs` 576 and `film.rs` 516
  are new; `wet.rs` 716 → 747.

**Memory (B9).** The set-aside film's volume and pigment now live with
the stroke. The canvas keeps only a per-pixel slot index (4 bytes) as
scratch: it is empty between strokes, and copies of the canvas (undo
snapshots) and checkpoints don't carry it. The film costs 44 bytes a
pixel (about 315 MB at 3200 × 2240; it was 52, about 373 MB, all of it
copied into every snapshot).

**B3: the plough's stiffness.** It now counts a film set aside under the
stroke with that film's own stiffness (body, surface and set-aside
mixed by volume). Before, set-aside coats counted as body paint: in
Astra's reproduction ten coats of whole-column stiffness 0.9 moved 2.64
coats as one body and 0.26 as a fluid coat under nine stiff set-aside
ones (`exchange::tests::the_plough_feels_the_set_aside_films_stiffness`).
Effect: 1000 px l5_near mean 0.00 of 255 (max 18), l3_green 0.00 (23);
the six 3200 px panel crops mean ≤ 0.02 (max 7–166 at a few pixels).
The wet studies print the same numbers.

**B7: a round's dirty tip.** Capillary feed (`feed`, pointed tools)
folded every bristle's tip into its reservoir on every step, so rounds
and riggers never had a dirty tip. The feed runs the belly's paint down
the tuft; what a hair picked up sits on its tip and works in over
`TIP_RUN`, as on any brush. Feed now pools reservoirs only
(`bristle::tip_tests::feed_keeps_the_tips`: a round dragged through a
wet dark ends with 0.46 of 147 in its tips, before 0). Effect against
B3: 1000 px l5_near mean 0.02 (max 59), l3_green 0.01 (58); panel crops
l5_near 0.12 (46), l3_green 0.03 (50), the others ≤ 0.00 (≤ 4). A faint
speckle along rigger and round marks, invisible at viewing size. The
wet studies print the same numbers.

The golden scene and the replay hashes (both profiles) are re-recorded
for B3 and B7. Merging main needs its own deliberate re-record (B10).

**B8: checkpoints.** `read_state` refuses a PAINTCK8 surface film that
isn't finite or lies outside `0 ≤ top ≤ vol` (with float slack), as it
refuses a bad hand-time ledger
(`checkpoint::tests::a_corrupt_surface_film_is_refused`).

**S6: the studies.** `paintings/src/study.rs` holds the measuring code
the three wet studies had copied (`Img`, `width`, `median`, `rect`,
`field`). Where the copies had drifted, each study keeps its own
semantics explicitly (`width(.., to_ends)`). All three print the same
numbers and write the same images as before.

Left: the checkpoint still writes the surface film after the hand-time
block (moving it means a new format version for no gain now), and the
`#[ignore]` probes in `wet.rs` stay.

## 10. Review of the maintenance round (finding 2 and 3)
- **Checkpoint volume** (finding 2): loading now refuses a film whose
  total volume isn't finite or is negative; before, `vol = +inf` or a tiny
  negative total under an empty surface layer passed the surface check.
  Test `a_corrupt_film_volume_is_refused` (fails without the check).
- **The studies' line measure** (finding 3): `study::Img::line` offset its
  averaging band by the per-sample step times `j/s`, so the band shrank as
  the resolution grew (0.20 units instead of 0.63 for a 32-unit vertical
  line at 3200). It now uses a unit normal. `study_wet_control`'s numbers
  on this branch, before → after: row B (contour softening) is unchanged
  at 1000 and moves in two cells at 3200 (2.34 → 2.47 and 1.92 → 2.06 mm);
  row C (a flat's stroke end) changes in most cells at both sizes (at
  1000, e.g. 2.20 → 1.32 mm open/full/light). The §8 comparison with
  main's engine used the old measure for both engines; its conclusion
  (the load controls softening; full loads crisp, lean ones soft) holds
  on the new numbers for this branch, but main's column wasn't re-measured.
