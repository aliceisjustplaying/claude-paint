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
- **Particular things of the day.** A ditch with rushes and pollard
  willows running into the plain; haycocks on a mown strip; a hedgerow
  and a copse on the right, a lone field tree left of the town; two horses
  at grass (the horses of the Greifswald meadows); a cart track worn up
  the rise to where the two stand, with pebbles in its ruts; in front,
  grasses gone to seed, dock, yarrow and a thistle; three swallows high
  up. Grass and plants laid last in upturning strokes [NG p.56].
- **Light.** Afternoon sun behind the viewer's left shoulder: the lime's
  lit side is the left, its shadow falls away to the right across the rise
  and the meadow behind, and cloud shadows lie across the plain and the
  near foreground. Soft level streaks of fair-weather cloud lie one above
  another, thinner toward the horizon.

## Working method, stage by stage

(Stages are the program's `o.stage(..)` blocks.)

1. **ground**: the Friedrich style's bought ground (red earth, light brown,
   brushed top); a thin warm brown underpainting glazed over the land for
   the values (barely any in the sky), fused with the badger and dried.
2. **sky**: laid in lean with a broad filbert from the sky's own palette
   (lead white, cobalt, pale smalt, ochre, red earth), cool blue above to
   warm cream at the horizon, warmer on the sun's side; fused and dried.
   Clouds stippled onto the dry sky from a cloud palette without red earth
   (grey-blue bellies, warm white tops), fused lightly in the cloud area
   only, dried, lit crests re-stippled; then a fine stipple over the whole
   sky for the air's grain.
3. **swallows**: three, a rigger each wing and tail, on the dry sky.
4. **distance**: the far woods (short level strokes + stipple, cool); the
   town stroke by stroke (houses as blocks of short upright strokes, some
   gabled; towers with a lit left side; spires lifted to a point; a
   windmill).
5. **plain**: meadows in level strokes, their color already carrying the
   mown strips in perspective (equally spaced on the ground, so widening
   toward us, fading into one tone near the horizon), the cloud shadows
   and the lime's shadow; the far meadow stippled into the air.
6. **middle**: copse, lone tree and hedgerow in small hooked touches, dark
   then lit (my `small_tree`); the ditch as a dark band with rushes and
   three glints; haycocks (strokes lit left to right, a shadow to the
   right); pollard willows far to near (my `willow`: trunk, knuckled head,
   rods, narrow leaves dark then silver).
7. **bank**: the rise and foreground in strokes following the slope, the
   shadows in the color; the cart track's ruts laid with a small brush
   along them, then a stipple of grit.
8. **lime**: the grown wood (engine `broadleaf` growth into my drawn,
   lobed, ovate crown), stout wood in pointed-sable strokes twice; the
   trunk repainted as a column (11 filbert strokes up the grain, lit
   grey-green left to dark right, aimed at the look over the dark wood),
   fissures, lichen flecks, the foot flaring into grass; the leaf mass
   laid dark in short hatching; every hooked leaf touch of the grown tree,
   back to front, colored by its light in my four greens; a lean,
   low-hiding stipple of cool shade over the side away from the sun; sprays
   of leaves out past the edge.
9. **horses**, then **figures**: stroke by stroke, rims of light on the
   sun's side.
10. **grass**: `Sward` tufts, patchy; blades as upturning rigger strokes,
    deeper in the shade, yellower where the wind turns them; flowers a
    touch each. **plants**: pebbles, tall grasses, dock, yarrow, thistle.
11. finish: aged varnish, cracks, raking light (`Finish::aged`).

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
   there); a badger pass after it did not soften it. Second try: the
   shadow *stippled*, density following the shade: better edges, but at
   3200px it read as a strip of dark clover laid on the grass. What worked
   in the end: no separate shadow at all, the shade folded into the
   meadow's own color function, so the lay-in strokes carry it (the way a
   painter mixes the shaded grass, not shades the grass afterwards). The
   stipple did work as a lean, low-hiding (`.paint(0.25, 0.3)`) cool veil
   over the lime's far side, where the leaf touches show through it.
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
12. **Editing a helper function stales every checkpoint.** Helpers below
    `main` count for every stage, so changing `paint_lime` (used only in
    the "lime" stage) made "bank" stale, and so did moving the lime's
    crown geometry, which sits between blocks above "bank". I used
    `--stale-ok` knowingly each time; `// ckpt: from lime` tags would
    cover the geometry but not the helper. A per-helper attribution (hash
    a helper only for stages that call it) would make resume useful for
    motif work, which is exactly where it's needed.
13. **`Palette::paint(target, ..)` is a masstone target, and I forgot.**
    Bark bands mixed that way from dark browns came out pale grey over the
    dark wood (the masstone of a thin, lean mix isn't the look). `c.aim(..)`
    at a point on the trunk fixed it at once. Two calls that both take "a
    color" and give a paint, with very different meanings, is an easy
    trap; a name like `paint_masstone` would help.
14. **A grown crown depends on everything around it.** Moving the lime's
    foot 10 units (to stand it on the bank, not its edge) regrew a
    different tree, since the trunk line is an input. I added an env var
    for the growth seed and compared three at 1000px side by side (peek +
    `magick +append`) to pick one. A small "grow N variants and show them"
    helper would save a round trip.
15. **Preview and full render differ in more than resolution.** The lime's
    holes and lobes land differently at 1000px and 3200px (stroke centers
    are accepted on masks at another resolution; notes/workflow.md). The
    sky holes I judged at 1000px moved in the full render. Judging needs
    both, which doubles the looking time.
16. **Isolated heavy stipple touches read as white dust at 3200px.** In
    the fine sky stipple a few touches land fresh from the dip and sit as
    specks over the smooth sky. I let them stand (they read as texture at
    arm's length).

## Critique (honest)

What works: the sky, a luminous cool-to-cream gradient with soft level
cloud streaks and the ground's warmth coming through in places, reads as
thin stippled paint. The lime at 3200px is the best passage: thousands of
hooked leaf touches, lit shoulders on the sun's side, a cool shade veil on
the far side, sky holes with limbs in them; it has the "no gradation of
detail according to significance" quality. The middle distance holds up at
full size: pollard willows, the hedgerow, the haycocks with their small
shadows, two horses, the town's spires. The value plan (light sky, middle
land, one dark tree, the dark foreground as a frame) is Friedrich's.

What reads as digital or weak:
- The crown's silhouette on the shaded right is still too even an arc; a
  lime's crown breaks into separate masses more than mine does.
- The foreground is a field of evenly distributed grass marks. The tufts
  are good one by one, but their distribution is statistical, not looked
  at; Friedrich's foregrounds are more particular and more sparing. The
  plants (dock, yarrow, thistle) are too small to carry at 1000px.
- The line where the far grass stops (the sward's smallest mark) reads as
  a faint boundary across the rise.
- The couple is legible but slight; the heads are dabs. At 40 units they
  are right for the picture's scale but could carry more drawing.
- The trunk is a smooth modeled column; the bark wants more character.
- The composition is safe: a big tree left, a town right. It is closer to
  a pleasant summer view than to Friedrich's charged emptiness; a lower
  tree, more sky and a wider plain would be braver.
