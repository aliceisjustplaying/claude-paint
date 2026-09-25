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
   soft shadow mask left a hard dark slab with visible stroke ends (the
   strokes are planned where the mask passes the threshold and end
   there); a badger pass after it did not soften it. Workaround: the
   shadow *stippled*, with density following the shade mask and a
   darker version of the ground's own color: the edge becomes a thinning
   of touches. The same trick (a stipple with `.paint(0.25, 0.3)`, low
   hiding) worked as a cool shade veil over the lime's far side. A
   "stipple glaze" is the most useful tool I found this round.
6. **Thick limbs paint as striped planks.** `wood_strokes` painted with a
   round sable at the limb's width show bristle stripes and pale, square
   ends at 3200px (the "ladder" issue in notes/motifs.md). Workaround: each
   stroke twice, the second narrower with more medium (0.4). Better, not
   gone.
7. **Sky clouds took four tries.** Cloud density fields are mine to
   write (Fbm + domes), and the look depends on them completely:
   continuous banks came out as white "worms" (only the crest showed),
   heaped banks as cigars, separate heaps as cartoon sprites. What worked:
   soft level streaks, Gaussian across, heaped above and flat below,
   gated by a low noise so they break. `atmos::Clouds` exists but needs a
   `World` camera and meters; I didn't have time to learn it for a
   painting with no other use for `scene`.
8. **The small things of a foreground are all hand-written.** Nothing in
   the engine helps with a dock, a thistle, yarrow, pebbles or a cart
   track, which is right by the principles, but each one took a function
   of gestures (100+ lines together), and at 1000px most of them
   read as specks. Yarrow heads at the first size read as scraps of white
   paper.
9. **`Tree::bounds` includes the trunk.** I wanted the crown's extent to
   shade the crown's far side and break its edge; I had to compute it from
   `Tree::crown` (the drawn outline) myself.
10. **Crops don't share checkpoints with the whole render.** A crop's
    `--ckpt` goes to `<name>_full_crop.*`, so every different crop window
    paints from the ground up (25–60 s at 3200px here because the whole-
    canvas masks and the lime's 23k touches are planned regardless). The
    lime crop took 59 s.
11. **The perspective of flat land is mine to derive.** Mown strips equally
    spaced on the ground, cloud shadows in perspective, the cart track
    narrowing: all from a hand-made `depth` and `1/(d+k)` distance. The
    `scene` module would give this, but it's a big API to learn for
    one painting; a small "ground plane" helper (y ↔ distance) would do.
