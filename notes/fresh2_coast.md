# fresh2_coast: *Morning on the Shore at Arkona*

An original picture in the manner of Caspar David Friedrich, painted from
knowledge only (no reference images). Program:
`paintings/src/bin/fresh2_coast.rs`. Renders: `out/fresh2_coast.png`
(1000px) and `out/fresh2_coast_full.png` (3200px).

## The picture and why

The Baltic shore on Rügen just before sunrise. The horizon is ruled a
little above the middle (y 408 of 714). The sea is flat and dark under a sky
that climbs from a pale lemon glow at the horizon, through rose and pearl,
to a cool slate blue at the top. A waning crescent hangs over the place where
the sun will rise. Its lit limb is turned down toward the sun below the
horizon, which is where it would be at that hour. Low bars of violet-gray
stratus lie over the glow, and their undersides catch the light. On the
beach, left of center, stand three fishermen's poles; a net hangs drying on
a rope between two of them. A big glacial erratic lies in the left
foreground, with a smaller stone beside it. A woman in a dark high-waisted
gown and a dull red shawl stands at the water's edge, right of center, with
her back to us, looking toward the glow and a brig hull-down on the horizon.
A far sail and a few gulls are the only other life. Tide pools in the damp
sand hold the sky. Marram grass, flat stones and pebbles fill the dark
foreground.

What it draws on in Friedrich (from general knowledge of his work and
`notes/research/friedrich_materials.md`):
- **Emptiness and horizontals.** A huge sky over a thin band of sea and
  shore, as in the Baltic pictures. Nearly everything is level, and the few
  verticals (poles, figure, masts) are cut sharply against it.
- **The Rückenfigur.** A single small figure seen from behind, a stand-in
  for the viewer, looking at something we can't fully see: the sun that
  hasn't risen yet.
- **The moon and ships as quiet symbols.** A crescent and a departing or
  arriving sail. Friedrich's coast pictures often pair a figure with ships.
- **Poles and nets.** Fishing gear on the shore gives a humble, particular
  subject and a strong vertical rhythm. It is not a copy of any
  composition I know.
- **The erratic.** Rügen's beaches are strewn with glacial boulders, and
  Friedrich drew them often.
- **Technique:** a warm, multi-layer ground; a graphite underdrawing (horizon
  ruled, poles, boulder and figure outlined) left to shimmer through; a very
  thin underpainting [CATS p.127]; a sky laid in thin and then stippled in
  two passes, wet and dry [NG p.56]; thin paint elsewhere; details last:
  grass in "fine upturning" strokes over the finished sand [NG p.56] and the
  gulls added at the end (like the gulls added to the finished *Monk*
  [CATS p.130]); a dark glaze toward the bottom edge and corners (his advice
  to Carus [MET p.35]); aged varnish and craquelure. Palette: the post-1820
  one (lead white, pale smalt, cobalt, yellow ochre, red earth, vermilion,
  raw umber, bone black, chrome yellow), used as **families**. The sky,
  sea and earth passages each mix only from their own subset.

## Working method, stage by stage

1. **drawing**: a rigger with a lean gray "graphite" paint: the ruled
   horizon, the poles, the boulder's contour and the figure.
2. **underpainting**: the sea in a dull blue-gray and the shore in umber,
   thin and by masstone. Added after the first renders showed the orange
   ground flickering through every gap (see FRICTION).
3. **sky**: `broad()` lay-in (coverage 4.2, medium 0.4) with long,
   near-level arcs, a shade duller than the target; a light badger pass;
   then a stipple (stippler 3.2) into the wet paint, aimed at the sky's own
   colors.
4. **sky stipple**: dry; a finer stipple (1.7), barely lighter than the
   field and denser toward the glow.
5. **clouds**: stratus bars as hand-placed filbert drags aimed at a
   violet-gray over the sky, and a warm round-sable line on each underside.
   Softened along their length with a small badger.
6. **moon**: a halo stippled a touch lighter, then the crescent mask worked
   with a small round in stiff lead white, by masstone.
7. **sea**: a masstone `broad()` lay-in, dark at the horizon and lighter
   and warmer toward the shore, with a path of light under the sun. The far
   band is laid by hand in level rows. A narrow badger fuses it inside the
   water only. Then the swell: faint darker and lighter strokes that crowd
   together toward the horizon. Glitter under the glow: thin, lean
   horizontal dashes. Last, the horizon ruled in short sable strokes.
8. **shore**: a masstone body lay-in (damp gray sand near the water, drier
   ochre, the foreground sinking into shadow) and a light fusing; tide pools
   holding the low sky; ripple marks (light crests, dark troughs, bowed and
   in perspective); a broken lip of foam; the wrack line; pebbles, each a
   dark touch with a light touch on the top.
9. **rocks**: a `Form` with the erratic (a rounded `block` fused with an
   ellipsoid, turned, roughened, cut at one end), a low stone and a group in
   the shallows. The light is low and from behind to the right
   (`Light::new((0.85, -0.35), -0.25)`). Order: shadows on the sand first;
   then per stone a dark block-in down the planes (coverage 4.5), the lit
   planes in stiffer lighter paint, a light fuse and accents only in the
   big concave breaks; then lichen touches on the sky-facing top and
   reflections of the far stones.
10. **ships**: a brig hull-down (hull, two masts, stacked sails shaded on
    the left and warm on the right) and a far sail.
11. **poles**: the net first: a thin masstone veil over a mask hung from a
    sagging rope, lean fold lines, and two families of rigger mesh lines.
    Then the poles in two loads each (the second set down in the wet end of
    the first), a lit edge on the sunward side, the rope, lashings, cork
    floats and long faint shadows toward us.
12. **figure**: gestures in a `Hand` frame: skirt strokes flaring to the
    hem, bodice, sleeves, shawl to a point down the back, neck, head and
    hair knot; a small rim of light on the head and right shoulder; her
    shadow.
13. **foreground**: flat half-sunk stones (dark wide strokes, a light top,
    a shadow) and marram tufts in upturned rigger strokes (dark, olive, a
    few pale).
14. **gulls**: bent two-stroke wings, aimed darker than the sky under them.
15. **glaze**: a thin umber veil deepening toward the bottom edge and
    corners; then `finish` with varnish, a finer and cleaner craquelure
    than the stock one, and raking light.

## FRICTION

(Workaround in each item.)

1. **Aimed light touches over a cool dark turn saturated orange or salmon.**
   `Canvas::aim` for a small light mark over the blue-gray sea (swell
   highlights, glitter) or the dark sand (pebble tops) picked piles heavy
   in ochre or red. Thin lead white over dark reads blue (turbid-medium
   effect), and the search "fixes" that with the complement. Where the
   stroke then lays thicker than expected it dries as an orange fleck. At
   1000px the sea and beach were dotted with them. *Workaround:* mix those
   marks by masstone (`Palette::paint`), and use palette families without
   reds for sea and earth (`Palette::only`).
2. **Bare ground shows through lay-ins, and aim then overcompensates.**
   With the Friedrich ground (warm orange-brown), a `broad()` lay-in at
   coverage 4 still leaves gaps and scraped places. Worse, an aimed stroke
   is judged by the canvas under its **center**: a stroke centered on a
   bare fleck mixes a blue-white pile to cancel the orange, and that pile
   dries pale where it runs on over covered paint. *Workaround:* a thin
   underpainting stage (historically right anyway) and masstone lay-ins for
   sea and beach.
3. **Coverage thins out at a mask's edge, so a hard edge needs hand
   work.** The sea lay-in, and even a dedicated narrow `Handling` band
   along the horizon, left pale slivers of the sky paint (laid a few units
   past the horizon) just under the horizon. Stroke centers don't reach the
   top edge of a thin band, and with `clip(true)` the strokes don't spill
   into it. It took three renders to see that the "pale smears" were sky
   paint and not stray piles. *Workaround:* stop the sky mask at the
   horizon and lay the top 15 units of the sea by hand in level rows of
   explicit drags. An "edge-hugging" option for `work` (seed centers on
   the edge contour, as `cut_in` does for a separate tool) would help.
4. **`tail` and `broken` in a dark broad lay-in.** With them switched off
   the sea came out clean in one test. They also change every random draw,
   so I couldn't separate them from item 3 in the time. Switching off
   either one alone moved the pale slivers rather than removing them.
   Changing any knob reshuffles the whole pass, which makes A/B debugging
   of a handling hard. *Workaround:* `tail(0.0).broken(0.0)` for calm
   water (it wants whole level strokes anyway).
5. **A blender drags paint across a boundary it isn't clipped to.** The
   stock `blend()` badger (40 units wide) swept level over the sea dragged
   the dark horizon band up into the dry sky as smudges. *Workaround:* a
   16-unit badger with `clip(true)` over a mask that starts 5 units below
   the horizon.
6. **Handling color fields can't read the canvas.** `c.work` borrows the
   canvas mutably, so a `color` closure can't call `c.under` (e.g. "darken
   whatever sand is here" for cast shadows). *Workaround:* hoist the
   beach's color function out of its stage and reuse it, letting
   `Aim::Laid` judge against the canvas. A `color_over(|x, y, under| …)`
   field would be natural.
7. **Wide variable-closure capture.** `Frame::per_column` returns a
   non-`Copy` closure, and every `move` color closure then consumes it.
   *Workaround:* `let shore = &shore;` (minor, but every painting will hit
   it).
8. **Stock craquelure at 1000px reads as a grid laid over the picture.**
   `Cracks::aged` hairlines are sub-pixel at 1000px but drawn a full pixel
   dark. *Workaround:* a custom `Cracks` (4.5 mm islands, 45 µm, dirt 0.3).
9. **Stipple lighter than the field reads as salt where it thins.** This
   matches the stipple notes. *Workaround:* lift only 0.006–0.028 L above
   the field and keep a coverage floor of 0.9 everywhere.
10. **Small motifs by gesture are fiddly without feedback.** The net took
    three tries: a solid veil reads as a flag; dark round-sable fold lines
    bead into tassels at 3200px; a dark net over the dark sea vanishes.
    What worked: a short net hung against the sky, lean rigger folds, and
    mesh lines started above the rope so none begin in the open. No
    engine fix is implied; these were painter's lessons, but each cost a
    full render cycle.
11. **Form: an ellipsoid boulder reads as an egg or potato, and a
    union with a small lobe reads as a turtle's head.** A rounded `block`
    fused with a flat ellipsoid on top, turned and roughened at three
    scales, reads as a granite erratic. `form.edges` at the default fine
    span turned the ridged grain into hundreds of black pits (pumice).
    *Workaround:* accents only for turns ≥ 1.2 over a 6-unit span.
12. **Dark block-in on a silhouette left pale holes** (sand showing through)
    at `coverage 3, threshold 0.2`. *Workaround:* coverage 4.5, threshold
    0.1.

## Critique

(Written after the full render; see the end of this file.)
