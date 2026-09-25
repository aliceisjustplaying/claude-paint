# Round 9, arm 2: "Winter Morning on the Ryck"

Source: `paintings/src/bin/r9_ryck_winter.rs` (`cargo paint r9_ryck_winter`).
Renders: `painting_1000.png`, `painting_3200.png` (this folder).

## The composition, and why

A still, veiled winter morning on the flat coast of Pomerania below
Greifswald, Friedrich's home town. The horizon sits a little below the
middle; more than half the picture is sky, pale and nearly empty, as in his
flat-land pictures (Greifswald meadows, the Monk's empty sky). The land is a
table of snow cut by two lines that meet on the horizon:

- **the frozen Ryck** coming out from under the low sun on the left and
  winding toward us; the veiled sun lies in the far ice as a warm sheen
  (no disc: the sun is behind haze, only its glow in the sky);
- **a path lined with pollard willows** running straight to the town,
  the trees shrinking in regular perspective (the kind of row Friedrich
  drew in his tree studies, and a rhythm of repeated verticals like his
  rows of firs or the posts on his shores). Sledge ruts run along it.
- **the town on the horizon**: three towers, the tall spire of St. Nikolai,
  the blunt tower of St. Marien, the slimmer St. Jakobi, over a low band
  of roofs; far, pale, in the air (Friedrich painted Greifswald's
  silhouette across the meadows more than once). A post mill on the left.
- **one man seen from behind** (Rückenfigur) on the path, walking toward
  the town, dark against the snow: the figure the viewer stands in for.
- **an old pollard close on the left bank**, cut by the frame, its crown
  of rods dark against the pale sky: a repoussoir, and the near end of the
  same kind of tree the row repeats into the distance.
- small particulars: a flat boat frozen in at the bank with snow in it, its
  mooring post and rope; crows in the air; dry grass and reeds flicked up
  through the snow along the banks, the verges and the near corner (NG:
  grass laid last, "fine upturning strokes" over finished snow); the
  willows' long shadows thrown toward us by the low sun.

Hour and light: morning, the sun low in the east (left), just above the
land and veiled. Everything on the right bank is backlit and nearly a
silhouette; shadows run from each foot away from the sun's point on the
horizon, toward the viewer's right.

Palette: `Style::friedrich()` (the post-1820 palette: lead white, cobalt
blue, smalt, ochres, vermilion, bone black, umber, chrome yellow); every
color is a pile mixed on the palette (`Palette::mix`) or aimed by the
handling. Ground: Friedrich's reddish ocher stack with a brushed top.

## Working method, stage by stage

1. **underpainting**: a thin warm-brown glaze over everything, heavier on
   the land and in the right-hand corner where the willows stand; badger;
   dry.
2. **sky**: two passes of long broad strokes wet into wet (a loaded first
   lay, a leaner second), the color running from a cool grey-blue zenith to
   pale straw near the land, a wide warm glow round the veiled sun. Long
   stratus bands dragged into the wet sky with a filbert; pale curling
   touches round the sun's place; three badger passes; dry. No stipple.
3. **distance**: the far snow in level strokes; the horizon's tree lines and
   hedges as short broken level dabs of grey violet with a round sable,
   copses as small upright touches, paler toward the sun.
4. **town**: gables one by one, each tower as three shaft strokes and a
   spire pulled up off a pointed sable; the post mill.
5. **fields**: body-color snow in strokes following the gentle roll of the
   ground; warmer on the crests, cool blue-grey in troughs and in the drift
   shade along the bottom edge; a light badger pass.
6. **ice**: a darker glassy base (sky color in the ice), badger; then snow
   blown over the ice dragged out by hand from both banks in long tongues
   trailing downwind, softened wet with the badger; lean warm strokes of
   the sun's sheen on the far reach; dry; a few crack hairlines.
7. **banks**: the dark lip where the field breaks over the ice, drawn down
   each bank in runs of strokes, set down and lifted, with gaps; snow
   dragged back over it from the field side so the edge is lost and found.
8. **boat**: hull side in a few strokes, the gunwale light, snow heaped
   inside and drifted against it, the post with its snow cap and the rope.
9. **shadows**: the willows' shadows and the sledge ruts as a thin cool
   glaze through a mask (built from the sun's direction), before the trees
   go in.
10. **willows**: each tree far to near: the trunk in side-by-side strokes
    following a flared foot, a waist and the swollen head, a slight kink;
    the knuckled head in dabs; bark fissures and lights in short broken
    strokes; the rods pulled up out of the head with a pointed rigger in a
    fan, arching outward, reloaded every six rods with a slightly different
    brown; snow on the head; snow dragged across the foot.
11. **details**: grass and reeds in clumps, flicked up with the pointed
    rigger; crows.
12. **figure**: legs, the coat in five overlapping strokes flaring below the
    waist, arms, a fold and belt, collar, head, a tall hat, a stick; snow
    dragged across the feet. Painted after the grass so the grass doesn't
    cross him.
13. **finish**: aged varnish, craquelure, relief light (`Finish::aged`).

## FRICTION

(Running list, most painful first at the end; see the final summary.)

1. **Editing a helper function below `main` stales every checkpoint.** My
   willow and figure are helper functions (as they should be: one tree
   motif called eleven times). Per the staleness model, "everything after
   the top-level item that holds it" counts for every stage, so tweaking
   the figure made "fields" stale. Workaround: `--stale-ok --ckpt` on almost
   every iteration, which trains the painter to ignore the staleness check
   entirely. A per-stage "which helpers does this block call" or a
   `// ckpt: from <stage>` tag for fn items would fix it.
2. **Geometry at the top stales early stages even when they don't use it.**
   The river masks live near the top (the ice and the fields need them);
   changing the river's width staled the sky. Workaround again `--stale-ok`
   (sound, since the sky doesn't read those masks, but the tool can't know).
3. **Strokes laid as shading read as glassy tubes.** Cast shadows and
   sledge ruts painted as dragged filbert strokes of a darker, low-hiding
   pile came out *lighter* than the snow at their centers with dark ridged
   edges, like plastic tubing (3200px crop), and the ruts as beaded dashes.
   The mean-look aim plus relief lighting of the stroke ridges seems to make
   any thin dark-over-light stroke look embossed. Workaround: shadows and
   ruts as a transparent glaze (`st.glaze(0.85)`) through a mask built from
   the sun's direction. That's closer to how a shadow on snow was glazed,
   but a painter would just brush it: it should work as a stroke too.
4. **A glaze covers whatever is under its mask, including trunks already
   painted.** My first shadow glaze ran across the trunks of nearer willows
   and lightened them into pale bands (a bluish scattering glaze over a dark
   is lighter). Workaround: reorder, glaze the shadows before the willows.
   Fine for a painter, but surprising: a transparent glaze was expected
   only to darken.
5. **Snow on the tops of things (willow heads, the boat, the post) comes out
   as cotton balls.** A `touch` of stiff white with a round sable gives a
   round fuzzy blob; a thin lying cap of snow needs a flat, dragged,
   pressed-off mark. I made the touches smaller and leaner; still blobby at
   3200px.
6. **The ice first read as a road.** A river painted with one handling over
   one mask is one even band with a perfectly continuous edge; the
   `color(x, y)` closure varied the color but the aimed, blended result
   flattened it again (the planned sheen and snow patches barely showed).
   Workaround: paint the variation as separate gestures (snow tongues
   dragged by hand from the banks) over a darker base. The lesson matches
   principles.md: variation from a function gets averaged away; variation
   from gestures survives.
7. **No `Clone` on `Handling`.** To run the badger three times with an
   angle I call `st.blend()` inside the loop. Minor.
8. **No integer draw on `Rng`** (`below`, `pick`): I wrote
   `(rng.f() * n as f32) as usize` with a clamp. Minor.
9. **The mask edge is the edge.** The ice/field boundary is a hard,
   perfectly continuous curve even with a soft ramp; the bank strokes on
   top only partly hide it. Roughening would help (`Mask::roughen`), but a
   painter would find the edge by painting the two sides against each
   other; there's no easy way to say "let the field strokes overlap the ice
   strokes irregularly".
10. **Craquelure is strong at 1000px**: long straight crack lines cross the
    quiet sky; `Finish::aged` gives no easy knob for a younger, finer net
    other than building my own `Finish`.
