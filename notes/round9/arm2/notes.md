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
8. **boat**: the hull in side view, laid in level strokes from the sheer
   line (rising to both ends, bow raked, stern square) down to the ice,
   darker below; the gunwale light; snow heaped inside and drifted against
   it; the post with its snow cap and the rope. (First version, strokes
   along a single curve, read as a puck; the side-view rows fixed it.)
9. **shadows**: the willows' shadows and the sledge ruts as a thin cool
   glaze through a mask (built from the sun's direction), before the trees
   go in.
10. **willows**: each tree far to near: the trunk in side-by-side strokes
    following a flared foot, a waist and the swollen head, a slight kink,
    the silhouette strokes blunt and cooler (see friction 5); the knuckled
    head as stumps of old cuts pushed up out of it; on the big tree a dark
    split cleft with a lit lip; bark fissures and lights in short broken
    strokes; the rods pulled up out of the head with a pointed rigger in a
    fan, arching outward, reloaded every six rods with a slightly different
    brown; snow on the head; a drift heaped against the foot in short level
    strokes, lit on top.
11. **details**: an old fence of weathered posts across the field from the
    river to the path (two fallen), snow-capped, drifted at the feet; reeds
    standing out of the ice by the left bank; grass and reeds in clumps
    along the banks and in a few wind-scoured patches, flicked up with the
    pointed rigger; crows.
12. **figure**: legs, the coat in five overlapping strokes flaring below the
    waist, arms, a fold and belt, collar, head, a tall hat, a stick; snow
    dragged across the feet. Painted after the grass so the grass doesn't
    cross him.
13. **finish**: aged varnish, craquelure made finer and cleaner than
    `Cracks::aged` (depth 12 µm, dirt 0.2, veil 0.3: a picture kept well;
    the default network dominated the quiet sky), relief light.

## FRICTION

Top five first, then the rest in the order I hit them.

1. **A dark stroke laid with a pointed tip draws a warm tan outline
   around itself.** Every trunk had a thin light-brown rim a few pixels
   outside its dark silhouette, on both sides, at 3200px: an outline, the
   most "digital" thing in the picture. It is in the paint (present at
   `--stop willows`, before varnish and relief), and it went away when the
   silhouette strokes used `point: 0.0`. My reading: the few outer hairs of a
   pointed tuft lay a very thin film of the brown beyond the body of the
   mark, and a thin brown film over white snow is, optically, a tan glaze.
   Physically honest, maybe, but no painter sees it, because a real loaded
   sable edge is crisp. Workaround: blunt tips and a cooler grey mixed
   into the edge strokes, so any thin film reads as a soft grey edge.
2. **Editing a helper function below `main` stales every checkpoint.** My
   willow and figure are helper functions (as they
   should be: one tree motif called eleven times). Per the staleness model,
   "everything after the top-level item that holds it" counts for every
   stage, so tweaking the figure made "fields" stale. Workaround:
   `--stale-ok --ckpt` on almost every iteration, which trains the painter
   to ignore the staleness check entirely (I then resumed *after* the stage
   I had changed once and looked at a stale boat, which is exactly the
   mistake the check exists to prevent). A per-stage "which helpers does this
   block call" or a `// ckpt: from <stage>` tag for fn items would fix it.
3. **Thin dark strokes over light snow looked like glass tubing.** Cast
   shadows and sledge ruts painted as dragged filbert strokes of a darker,
   low-hiding pile came out *lighter* than the snow at their centers with
   dark ridged edges (3200px crop), and the ruts as beaded dashes.
   Workaround: shadows and ruts as a transparent glaze (`st.glaze(0.85)`)
   through a mask built from the sun's direction. That's closer to how a
   shadow on snow was glazed, but a painter would just brush it: it should
   work as a stroke too.
4. **Variation written into a `color(x, y)` closure gets averaged away.** The
   river, painted with one handling over one mask, came out as one even
   band with a perfectly continuous edge; the planned sheen and snow
   patches in the color function barely showed after aiming and the
   badger. It read as a road in three iterations. Workaround: a darker base,
   then the variation painted as separate gestures (snow tongues dragged by
   hand from the banks, lean sheen strokes). Matches principles.md, but
   the handling API invites the function route first.
5. **Snow lying on top of things comes out as cotton balls.** A `touch` of
   stiff white with a round sable is a round fuzzy blob; willow heads, the
   boat, posts and the foot drifts all looked like wool or fog at 3200px
   (the first foot drift of the big tree, a 40-unit filbert, was a grey
   bow-tie smear). A lying cap or a heaped drift needs flat, pressed,
   level marks; I rebuilt drifts as rows of short level filbert strokes,
   narrower, lit on top. Still no easy mark for "a thin crisp cap of snow".

6. **Geometry at the top stales early stages even when they don't use it.**
   The river masks live near the top (the ice and the fields need them);
   changing the river's width staled the sky. `--stale-ok` again (sound
   here, but the tool can't know).
7. **A glaze covers whatever is under its mask, including trunks already
   painted**, and a bluish scattering glaze over a dark *lightens* it: my
   first shadow glaze ran across the nearer willows' trunks as pale bands.
   Workaround: glaze the shadows before the willows. Fine for a painter,
   but surprising.
8. **The mask edge is the edge.** The ice/field boundary is a continuous
   curve even with a soft ramp; the bank strokes on top only partly hide
   it. A painter finds that edge by painting the two sides against each
   other; there is no easy way to say "let the field strokes overlap the
   ice irregularly".
9. **`Cracks::aged` dominates a quiet sky** at both 1000 and 3200px (long,
   dark, dense network). Knobs exist (`depth_um`, `dirt`, `veil`) but only
   by reading crack.rs; `Finish::aged` has no gentler sibling.
10. **Stroke shapes are points, not outlines.** Everything with a
    silhouette (hull, coat, trunk) needed me to invent a raster of rows of
    strokes between two curves; the first boat (strokes along one curve)
    was a puck. Writing "fill this outline with level strokes, bow raked"
    by hand is where most of my code went.
11. **Pollard rods, crows and grass worked well** with the pointed rigger
    and a reload every few marks: not friction, noted for balance.
12. Minor API: no `Clone` on `Handling` (I call `st.blend()` inside the
    loop to run the badger three times); no integer draw on `Rng`.
13. **Speed was fine**: 1000px whole in 28 s, resumed late stages in about
    1 s, a 3200px crop in 8 to 40 s, the whole 3200px in about 3 min.
