# Round 7, arm 2 (a Rust program): *Two Poplars at a Pond, Evening*

Source: `paintings/src/bin/pond_poplars.rs` (`cargo paint pond_poplars`,
`-- --full` for 3200). Renders: `painting_1000.png`, `painting_3200.png`.

## What it is

Dusk after sunset, landscape format (1.4:1). The horizon is ruled level at
61% of the height, so the sky takes most of the canvas. It is clear, deep
blue-gray above and warm near the horizon, with the glow a little right of
center. There are three thin strands of evening cloud at different heights
and a small young moon high on the left. A low dark far bank runs across the
whole width, with a few low bushes near both edges and a faint far range
showing over it at the left. On the bank near the vertical axis stand two
Lombardy poplars, one whole (x 492) and one shorter with its top broken off
and a dead leader standing out of it (x 561). A still pond reflects the bank,
the trees and the sky. The near shore is dark, rises a little to both corners
and has reeds and sedge, thick at the corners and sparse in the middle. There
is no figure. Kinds of things: sky (with moon), trees, bank, water, reeds.

## Order of work (stages)

1. **drawing**: 2H pencil for the search (the horizon ruled in two pulls, the
   water's edge, the shore, a faint axis, both poplar outlines, the
   reflections' axes, the moon's place). The poplar flanks were restated in 3B,
   broken where unsure (`Canvas::draw` with `graphite::Lead`).
2. **lay-in**: one thin brown wash (`Style::glaze(0.88)`) for the values,
   loaded by a hand-written depth function: light in the sky, heavier on the
   bank, water, shore and trees. Then a darker brown body inside the trees.
3. **sky**: a lean broad lay-in a shade duller than the target, in level
   arcs, fused with the badger. After a 4 h wait, a stipple aimed at the
   sky's own tones. After drying, a finer stipple building the glow toward
   the horizon. The palette was restricted to lead white, pale smalt, cobalt,
   ochre, umber, red earth and bone black.
4. **moon**: a crescent mask filled with ~150 short touches of a small round.
   Painted before the clouds.
5. **clouds**: three or four strands with hand-set heights, lengths and
   thicknesses. They were stippled (`drag` level), fused by a badger pass
   while wet and let dry. A few lean warm touches mark the lit lower lip of
   the long strand.
6. **hills**: a stipple a step darker and cooler than the glow, left and
   right only.
7. **water**: broad level filbert strokes in the mirrored-sky color field
   (the sky sampled above the horizon, stretched, darkened toward the
   viewer), fused level. After a 5 h wait, a stipple of the same field and a
   light level blend.
8. **bank** (after the water, see below): the body in level filbert strokes
   laid inside its line. The sky was then cut back over the crest with level
   pulls of a round in a color sampled just above (`Canvas::sample`). Then the
   top line itself in two rows of level pulls, the bushes hatched upright,
   broken lighter grass tips on the crest and a dark lip at the water. Last
   came the bank's reflection, row by row in level pulls, the lower rows
   thinner and more broken.
9. **poplars** (my own function, `poplar`): the stem, then a dead-color body
   in the inner ~70% of the crown, tapered toward the top and the crown's
   base. Then ~100 branches rising from the stem (quadratic curves outward
   and up to the flank), each carrying sprays of short upswept strokes from
   three dark piles. How leafy a branch is varies, so the crown has hollows
   and bulges and the silhouette comes from where the sprays end. Fine flicks
   go past each branch end, some sprays hang at the crown's base and the
   leader is drawn into a point. On the broken tree: a thin gray dead leader,
   a splinter and two dead shoots. Finally a few lean cool touches on the
   left flank and warm ones on the right, then the trunk and some suckers at
   the foot. Waits of 60–90 min between these passes.
10. **reflections** (my own function, `reflection`): the trunk's image as one
    downward pull wiggled by the ripple noise, cut by 2–3 thin level touches
    of water. The crown's image went down row by row in level pulls of a
    round (1–2 pieces per row, ragged ends), each row shifted sideways by the
    ripples, some rows skipped, the color fading into the water toward the
    viewer. Then 90 faint level drags over the open water (a few percent
    lighter or darker than the color sampled there, kept off the trees'
    images) and a few pale wind lines.
11. **shore**: a body layer laid inside the shore line, then short upright
    hatching of grass over it. After drying, the water was cut back down over
    the shore's top in level pulls mixed from the water sampled just above.
    Then low sedge blade by blade with a rigger all along the shore (thin in
    the middle, thick at the corners) and ~170 nearer tufts scattered through
    the band in drifts, larger toward the bottom, a shade lighter than the
    ground, some with straw tips.
12. **reeds** (my own function, `reeds`, from its own seed 23): 52 clumps
    alternating left and right, none within 90 units of the axis, blades as
    four-point rigger strokes set down and lifted off, a few lit straw tips.
    Some clumps stand in the shallows, and their blades get short, broken,
    paler images. There are two broken stems bent over the water.
13. **veil**: one thin cool-brown `Canvas::glaze`, heavier toward the corners
    and the bottom, lighter over the glow, never zero.
14. **finish**: `Finish::aged` (varnish, craquelure, relief) unchanged.

## Tools used

`Style::friedrich()` (its ground, palette `friedrich_1820`, broad, body,
hatch, detail, glaze and badger handlings), `Stipple`, `Held` and `Gesture`
for every hand-drawn stroke, `Canvas::draw` (graphite), `Canvas::sample`
(to match colors by eye for cut-ins and drags), `Canvas::glaze`, `wait` and
`dry`, `Mask` and `Shape` for regions, `Fbm` noise for the lobes, ripples and
drifts. No motif or scene generators (no `Habit`, `tree_in`, `fir`, `rock`,
`Form`, `World` or `atmos`).

## Engine problems and what I did

- `Palette::only` panics on a tube the palette doesn't have: there is no
  "ultramarine" or "smalt" in `friedrich_1820`. I used pale smalt and cobalt.
- The water's broad strokes, painted after the bank, overran up into it by
  about 5 units and left a pale strip between the bank and its reflection.
  This is by design (strokes overshoot their mask). I painted the water first
  and the bank over it.
- The filbert stroke ends of the bank body and the near shore crenellated
  their top edges (boxy lumps). I laid the bodies inside the line and cut
  the neighbor's color back over it, sampled from the canvas.
- A thin brush dragged over the wet sprays (the branch wood, drawn after
  the leaves) ploughed light channels through the paint. At 1000 px they read
  as herringbone chevrons in the lower crowns. I removed those strokes.
- A badger pass over the wet cloud stipple spread it until the strands
  vanished. I made the stipple stronger before blending. A level badger
  pass over the wet tree reflections left a ghostly rectangle in the water,
  so I removed it.
- A stippled "veil" over the moon and a brushed mist over the dark bank both
  read as pale specks or a speckled band (as the sketchbook warns). I
  dropped both.
- `Handling` has no builder for its tool; I set the `tool` field directly.
- Editing helper functions or moving stage blocks made later checkpoints
  stale. I used `--stale-ok` while iterating. Both final renders ran from
  scratch.
- The 1000 px render takes ~25 s and the 3200 px render ~2 min on the shared
  machine.
