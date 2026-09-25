# Round 9, arm 1: a winter landscape at the easel, without the procedural tools

Painting: `paintings/lua/winter_willows.lua` (easel session, `EASEL_WITHOUT=procedural`, no hand time).

## Composition and why

*Winter Evening with Pollard Willows* (working title). A flat, snow-covered
lowland just after sunset. A frozen brook winds out of the lower left
foreground and back into the plain; a row of pollard willows stands along its
bank, diminishing toward a far village whose church spire is the only
vertical on the horizon. A lone walker in a dark coat goes along a trodden
track toward the village. The sky is two thirds of the picture: slate blue
above, going through pale gray to a lemon and peach afterglow low on the
left, where the sun went down, with a thin crescent moon over it.

What it draws on in Friedrich (from notes/research, not from pictures):
- the low horizon and the huge quiet sky, laid thin over a warm ground that
  glows through (warm ground layers, "one to two very thin layers");
- winter and dusk as his subjects: snow in lead white, cool smalt/cobalt
  shadows, the sky's lemon-mauve twilight;
- a single figure seen from behind or at a distance, walking toward a church:
  the Rückenfigur and the church as a goal on the horizon;
- a row receding into depth with exact diminution (his ruler-and-square
  studio, precise underdrawing);
- tiny particulars: rods on each pollard, crows, dry grass stalks through the
  snow ("fine upturning strokes" laid last over the finished snow).

## Working method, stage by stage

1. Canvas (`friedrich`, after 1820: reddish ocher ground), aspect 1.4.
2. Underdrawing in 2H: horizon ruled, far land line, stream banks, willows,
   church, figure; fixed.
3. Sky: two broad passes of long horizontal strokes wet into wet from a
   color field (slate at top, pale gray, peach and lemon glow centered low
   left), each blended. The warm ground glints through in places as lit
   wisps; kept.
4. Far land: a hazed band (filbert, horizontal), blended; after it dried,
   the village gables and church tower as small firm-edged touches, the
   spire drawn down to a point with a pointed round.
5. Snow: a thin underpainting (broad flat, medium 0.35) over the whole land,
   blended, then after a day the body snow (filbert 6, lead white mixes,
   coverage 5.5), strokes following gentle drifts.
6. Frozen brook: meandering ribbon (my own points), slate ice lighter and
   warmer toward the vanishing point, dusted with snow; a dark hole of open
   water; then the far bank's face as a cool strip and the near lip as white
   overhanging snow.
7. Pollard willows (my own function `willow`: a gnarled trunk outline in
   `char="broken"`, painted dark in vertical strokes, a broken glint on the
   left flank, then rods from five knobs with a pointed rigger in two
   passes, outer rods leaning out and curving back up). Sized by hand in
   perspective: eye at 1.7 m, so 1 m = (y − horizon)/1.7 units.
8. The near willow, cut by the top edge and old: a split in the trunk.
   Then per trunk: a second dark coat after drying (the first dried thin),
   a thicket of thin whips added to every crown (medium 0.4 in the load so
   the hairlines don't break on the weave), then after drying the light bark
   ridges dragged dry, patchy snow on the knuckles, and snow dragged across
   each foot: first the trunk below an irregular snow line filled with the
   field's own paint sampled beside it, then horizontal strokes across the
   base, each loaded from the field at that height.
9. The walker's track: footprints only, spaced by the ground's
   foreshortening (distinct near, a furrow far off). The walker: greatcoat,
   tall hat and stick, drawn as small polygons and painted with small rounds,
   one leg in stride, a faint glint of the glow on his left shoulder.
10. A cool transparent glaze over the land, darker toward the bottom and
    sides and lighter under the glow (Friedrich to Carus: a dark glaze
    "growing darker toward the picture's edges"), a lighter slate glaze in
    the upper sky corners. The trunks and walker take the land glaze at one
    even strength along their whole height.
11. Long wind-drift swells: a cool lee under each crest and a lighter lip,
    as two passes whose region and load come from a swell field, with lost
    edges.
12. Particulars: a thin waxing crescent over the afterglow (lit limb toward
    the set sun, a faint ring of earthshine), dry grass and reeds with seed
    heads along the brook and in the corners as upturning flicks, crows
    perched in the rods and a few flying.
13. Finish: a day's wait, varnish (0.3 coats), craquelure, relief.

## FRICTION

1. **Glazes cross the motifs.** A glaze over the land (to darken the snow
   toward the edges, Friedrich's advice to Carus) drew a visible horizon
   line straight across the dark willow trunks: cooler below, warmer
   above. A glaze over a rectangle around each knuckle (to sink the snow
   caps) left dark rectangles over the rods, like a filled selection. Without
   `world`/`view` (removed with the procedural tools) there is no depth
   system to say "behind the trees", so every glaze needs the motif masks
   subtracted by hand (`LANDG - KEEP`, the union of every trunk and the
   figure). Workaround: keep a global union of motif masks and subtract it
   from every glaze.
2. **Dark paint over light dries into slivers.** The willow trunks, painted
   dark in one coat over dry snow, looked solid while wet. After `dry()`
   they showed rows of horizontal light slivers at 1000px: the film leveled
   off the weave's crests and the snow showed through. It looked like a
   glitch, not paint. The plain look before drying gave no hint (`--dried`
   would have). Workaround: a second coat of the dark after drying, as a
   painter would, then dry again.
3. **Snow body left the red ground in scallops.** A body pass of snow
   (filbert 5, coverage 4) left torn holes of the red-ocher ground between
   strokes. The `friedrich` ground is uniformly reddish; the research
   describes a patchy lead-white top ground, which would have forgiven gaps.
   Workaround: a thin broad underpainting over all the land first, then body
   snow at coverage 5.5, load 0.95.
4. **Thin dark strokes go golden.** Dried grass and the willow rods, loaded
   with dark umber-gray masstones, came out golden-brown and transparent
   over the snow and the light sky: the thin film of a rigger is mostly
   the paint's transparent side. Workaround: `b:load(color, amount, {at=...,
   coats=0.9})`, aiming at the look over what is there; for the trunks
   `aim="masstone"` and a second coat.
5. **Small strokes over open paint lift it.** The village gables and church
   tower, built from many small overlapping strokes of a round over the
   still-open far band, came out pale, rounded and stacked like pagodas
   (each stroke lifted the band paint). Workaround: let the band dry, then
   lay each roof as a small polygon with `work{hand="detail", edge=...}`
   and the spire as three pointed strokes.
6. **Perspective by hand.** With `world` gone there is no ground-plane
   scale. I derived it: 1 m = (y − horizon)/1.7 units at the foot of a thing
   (eye at 1.7 m), and a step on the ground is dy ≈ 0.36·(y − HZ)²/k with
   k = 1.7·f. Before I did that, the footprints came out first as a flat
   pale ribbon (a second river) and then as evenly spaced blue polka dots.
   This is knowledge a painter carries, so it's fair, but it cost three
   tries.
7. **`edge="lost"` on a narrow band.** A 0.3 m-wide trodden furrow painted
   with `edge={lost=0.6, soft=0.4}` still read as a crisp, even ribbon. I
   dropped the band and let the footprints make the track.
8. **Option names differ between verbs.** `work` rejects `fill` (documented
   in the engine README as `Handling::fill`); `b:stroke` rejects `broken`.
   The error lists the valid options, which helps, but each costs a round
   trip.
9. **No mask translate.** A "top rim" (snow caps, the far bank's face) needs
   `m * (1 − m shifted down by k)`, written as a per-pixel `mask(function)`
   closure calling `m:at(x, y − k)`. It works (≈0.1 s), but a
   `m:shift(dx, dy)` would be simpler and cheaper.
10. **Pencil invisible at working size.** A 2H underdrawing at pressure
    0.25–0.3 on the 440 mm canvas barely shows at 1000px even in a crop, so
    I couldn't use it to check placement; I placed everything by
    coordinates instead.
11. **The thinnest whips read as dotted wire** at 1000px (a rigger 0.45
    with a point, lean load): broken runs of single pixels. At 3200px they
    are continuous hairlines.
13. **Drift patches never matched the field.** Snow laid later over a
    trunk's foot came out as a pale oval, then (darker) a blue puddle, then
    a pale rectangle: `rect()` in the zone left straight sides, the land
    glaze skipped the part of the drift inside the trunk mask I had excluded,
    and new opaque paint lies smoother and lighter than the textured,
    glazed field. What finally worked was the painter's move: fill below an
    irregular snow line with paint *sampled from the field beside the
    trunk* (`sample()` in the color function) and drag single strokes
    across, each loaded from the field at its height.
14. **`work{coverage=}` takes no function.** I wanted the swells as a
    coverage field fading in and out; `coverage` takes a number or a noise,
    not a function. Workaround: the field became the region's mask values and
    `load_at`, with `edge="lost"`.
15. **Lost edges reach into motifs.** A lost edge on a broad filbert runs
    up to ~2 brush widths past its region and picks up wet paint there: the
    first swell pass laid light bands straight across two trunks. The trunk
    masks had to be grown by 10 units before subtracting.
16. **Relief makes the snow look embossed.** At 3200 the body snow's stroke
    ridges, lit by the default relief (0.06), read as crumpled paper or
    etched scribbles over the whole field. I left Alice's default in place;
    thinner snow paint (more medium) from the start would have been
    smoother and closer to Friedrich's "slight impasto".
17. **The 1000px and 3200px pictures differ where it matters.** Slivers,
    specks and dotted hairlines appear at 1000 and not at 3200 (or the
    reverse), so a fix judged at one width can be wrong at the other.

12. **Crops at 3200 cost a replay after an early edit.** Each `easel edit`
    of an early chunk (the willows are chunk 11) makes the next `--scale
    3.2` look replay the whole log (50–90 s on this busy machine).
