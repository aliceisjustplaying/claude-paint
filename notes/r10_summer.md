# r10_summer: "Summer Afternoon: the Lime Tree on the Rise"

Program: `paintings/src/bin/r10_summer.rs`. Renders: `out/r10_summer.png`
(1000px), `out/r10_summer_full.png` (3200px). Scratch:
`~/tmp/r10-summer-9120a7f9/`.

## Composition and why

A flat, wide summer land under a high sky, the horizon a little below the
middle (0.635 h), the picture built in level bands the way Friedrich builds
his flat-land pictures (the Greifswald meadows, the plain near Dresden):
sky, a thin blue-grey strip of far woods and a town, the plain of meadows,
the rise in front.

- **One tree as the figure of the picture.** A single lime (linden) in
  full summer leaf, whole, on a low rise left of center. Friedrich paints
  single trees as presences (the lone oaks, the solitary tree in the flat
  land); the lime is the village tree, home and summer, where the oak in
  his winters is death.
- **Rückenfiguren.** Two small figures seen from behind at the edge of
  the lime's shade, a man in a dark green coat and a woman in a dark red
  dress with a pale shawl, her hand on his shoulder, looking over the
  plain. Friedrich's couples look *out* (moon, sea, sunset); here they look
  at the far town.
- **The town with church towers on the horizon.** Three spires and a
  windmill, tiny and blue-grey: the far goal the looking goes to
  (Greifswald's towers recur in his work). Kept small and low so it is
  found, not shown.
- **Particular things of the day.** A ditch with pollard willows running
  into the plain; haycocks on a mown strip; hedgerows and a copse on the
  right; a footpath worn up the rise to where the two stand; grass and
  meadow flowers in front, laid last in upturning strokes [NG p.56].
- **Light.** Afternoon sun behind the viewer's left shoulder: the lime's
  lit side is the left, its shadow falls away to the right across the rise
  and the meadow behind. Level banks of fair-weather cloud lie one above
  another, heaped on top, flat below, smaller toward the horizon.

## Working method, stage by stage

(Stages are the program's `o.stage(..)` blocks.)

1. **ground**: the Friedrich style's bought ground (red earth, light brown,
   brushed top); a thin warm brown underpainting glazed over the whole
   canvas for the values, heavier over the land, fused with the badger and
   dried.
2. **sky**: laid in lean with a broad filbert from the sky's own palette
   (lead white, cobalt, pale smalt, ochre, red earth), cool blue above to
   warm cream at the horizon, warmer on the sun's side; fused and dried.
   Clouds stippled onto the dry sky: grey-violet bellies, warm white tops,
   fused lightly in the cloud area only, dried, crests re-stippled; then a
   fine stipple over the whole sky for the air's grain.
3. **distance**: the far woods (short level strokes + stipple, cool); the
   town painted stroke by stroke (roofs, towers with a lit left side,
   spires lifted to a point, windmill sails).
4. **plain**: meadows in level strokes; mown strips in perspective
   (equally spaced on the ground, so widening toward us), fading into one
   tone near the horizon; the far meadow stippled into the air.
5. **middle**: copse and hedgerow in small touches, dark then lit; the
   ditch with a glint of sky; haycocks; pollard willows far to near.
6. **bank**: the rise and foreground; the footpath; the lime's shadow as a
   glaze, softened with the badger.
7. **lime**: the grown wood (engine `broadleaf` growth into my drawn
   crown), stout wood in pointed-sable strokes; lit bark as lean dry
   strokes; the leaf mass laid dark in short hatching; then every hooked
   leaf touch from the grown tree, back to front, colored by the light it
   catches (my four greens).
8. **figures**: the couple, stroke by stroke.
9. **grass**: `Sward` tufts; blades as upturning rigger strokes, deeper in
   the shade, yellower where the wind turns them; flowers a touch each.
10. finish: aged varnish, cracks, raking light (`Finish::aged`).

## FRICTION (running list)

1. **Clouds laid into a wet sky vanish.** Stippling cloud bellies and tops
   into the wet sky lay and then running the badger over the sky fused them
   into it: only a thin "worm" of the brightest crest survived. Workaround:
   lay the sky, fuse, `dry()`, then stipple the clouds on the dry sky and
   badger only a cloud-area mask at low pressure. It costs a full dry and
   the wet-into-wet softness of the cloud edges is lost.
2. **Grown trees at small sizes don't work.** `broadleaf::Tree::grow` with
   a crown of 5–20 units gives few or no usable touches; a pollard willow
   grown from `Species::willow()` at 20–70 units painted as a bare trunk
   stub (touches too small/few for the pointed sable). A hedgerow of grown
   bushes painted as a row of black trunks with a few dots ("hearts").
   Workaround: my own small-tree and willow motifs (touches placed in an
   ellipse, dark pass then lit pass; willow rods and leaf touches along
   them). There is no "far tree" level of detail in the engine.
3. **A hatched lay-in clipped to the leaf mask gives a cut-paper edge.**
   `work(..).clip(true)` over `Tree::leaves` gives a hard, broccoli-like
   silhouette with the holes looking cut out. Workaround: no clip and a high
   threshold so the lay-in stays inside and the leaf touches make the edge.
4. **The grass sward reads as a lawn.** `Sward` tufts painted with the
   rigger make an even, bright, evenly spaced grass texture across the whole
   foreground (a digital grass brush look), even with per-tuft color
   variation.
5. **Glazes are streaky as a shadow.** A glaze `work` with `load_at` from a
   soft shadow mask left a hard dark slab with visible stroke ends;
   needed a lower load and a badger pass over the shadow area.
