# Lab study: a still water edge (lab2)

Setup (`notes/lab/water_setup.lua`, shared by both logs byte for byte): Friedrich style with
the 1820 greens palette, 280 mm panel at 4:3 (1000 × 750 units), seed 73. Geometry: a
quiet late-afternoon sky; a far shore strip on the water at y = 352 with its reflection
(0.85 of its height); a near bank from the left (a lumpy bush mass, a shoulder at x ≈ 500
and a low earth spit to a tip at x = 650) meeting the water along a waterline `WL(x)`; its
reflection mirrored about `WL` and shortened to 0.7 (we look down a little). Color fields:
the water is the sky mirrored and darkened toward you, and the reflection is the bank's
color at the mirrored point, darker and cooler.

| | log | render | 3200 crop (units 380–700 × 520–660: the spit, waterline, reflection edge) |
|---|---|---|---|
| A, the old way | `water_A.lua` (8 chunks) | `water_A.jpg` | `water_A_crop.jpg` |
| B, the new way | `water_B.lua` (8 chunks) | `water_B.jpg` | `water_B_crop.jpg` |

`easel status` doesn't report stroke counts; the chunk counts and the moves below are exact.

## A: the sketchbook as written
1. The sky a shade dull, `broad` coverage 4.2, medium 0.3, blended.
2. `dry()`, then the far shore in level body strokes, clipped.
3. `dry()`, then the water as whole level strokes (`round 6`, not a flat, sketchbook §3),
   coverage 4.2, and a level blend (`angle_jitter=0.003`). The first try unclipped threw
   white slashes into the far shore (undone); the kept one passes `clip=water`.
4. `dry()`, then the bank inside its masks: a filbert body underlayer, small turning round
   strokes, hatched lights top left, the earth face level (§5 recipe style). The first try
   without the underlayer was peppered with red ground (undone).
5. `dry()`, then the reflections inside their masks: level body strokes of `reflsrc`,
   blended level, clipped.
6. `wait(240)`, then 26 level rigger/round lines, pale and dark, closer toward the horizon
   (§3 glints), clipped to the water. The first try crossed the bush (undone).
7. The finishing chunk. Five `dry()` calls; 47 days on the clock.

**What I see.** A clean, legible picture that reads as a render. Every edge is found: the
bank is a cut-out silhouette, and the reflection is its exact mirror with a knife-sharp
outline (the **hard cut-out** failure). The water is smooth and even. At 3200
(`water_A_crop.jpg`) the bush and earth are an embossed, pitted crust with dark pepper
specks. The contact itself is clean, with no halo: a thin dark seam.

## B: the Round 6 principles
1. **Masses first, one sitting (clock 0 throughout).** The bank, its earth and its
   reflection as one dark shape, laid together and unclipped: the bush in a few big turning
   filbert 6 strokes (length 10–26, coverage 2.6, medium 0.3), the earth level (filbert 5,
   `hug=false`), and the reflection **pulled down** in vertical filbert 6 strokes (angle
   π/2, length 12–40, coverage 2.8, medium 0.35). The far shore and its reflection as one
   band. The first band pass with a filbert left gaps and scribbles, so chunk 3 relays it
   with the broad preset.
2. **The lights around the darks, fenced softly** (chunk 4). Sky and water in long level
   strokes of thin paint (length 120–320, coverage 3.8, medium 0.4), shifted by a slow
   band noise so neither is an even ramp. Each is clipped to its own region
   `:grow(3 or 4):blur(2)`, so the light meets the wet dark over a few units and the edge
   goes soft without being swept. The unclipped first try smeared the far shore and the
   bush (below).
3. **The reflection fused into the water** (chunk 5). The earth relaid fuller, then a level
   blend over `(refl:grow(8) + farrefl:grow(5)) * water`, clipped to the water: the vertical
   pulls fuse and the reflection's outline goes soft.
4. **A few lights** (chunk 6): three or four lit masses top left of the bush, gated by a
   coarse noise, filbert 4 in a muted olive, coverage 1.3, laid into the wet dark.
5. **One accent** (chunk 7): a broken sheen along the waterline, three round-3 strokes in
   the mirrored low sky, softened by a short level blend. An earlier round 2.4 version with
   no blend was a hard white wire at 3200; I replaced it with `easel edit 7`.
6. The finishing chunk. No `dry()`, no `wait()` before it: the whole picture is one wet
   sitting.

**What I see.** Water. The bank's top is feathery where it meets the sky, the reflection
dissolves into the water toward its outer edge, and the far shore and its reflection are
one soft band. The bank, its earth and its reflection read as one dark shape with the
faint sheen as the one found line at the contact. At 3200 (`water_B_crop.jpg`) the paint is
smooth with visible brushing, the spit ends in a reedy fringe (bristle marks past the tip)
and the reflection shows its vertical drags under the level blend.

## Verdict
**B looks better**, clearly at 1000 px. A is tidier, but it looks like a render of a
landscape. B looks like a painting of one: the contact isn't a pasted edge, and the
reflection is in the water rather than stamped on it.

## B's ceiling (what the new way still gets wrong)
- **A pale rim inside the reflection's outer edge** (`water_B_crop.jpg`, the diagonal from
  the spit down to the left). Where the water's light overlapped the wet reflection by 3
  units, the level blend spread it into a lighter band 3–8 units wide: a halo, milder than
  the old ones, but there.
- **Drips.** The vertical reflection strokes under the spit end in detached dark blobs
  below the reflection (x ≈ 560–720 in the crop). They read as reflected reeds at 1000 px
  and as beads at 3200.
- **The sheen** is still two faint parallel lines at 3200 (below), not one soft line.
- **Ground flecks.** Thin long strokes leave warm red-ground dashes in the sky and water.
  At 1000 px they pass as cloud reflections, but they're a coverage accident.
- **The bush's lights** read warmer at 1000 px than at 3200 (`aim="laid"` over the dark
  green). A 3200 check showed muted olive, as intended.

## Wet-paint misbehavior (for tonight's wet fixes)
1. **Long light strokes around wet darks sweep the dark away**
   (`water_wet1_unclipped_lights.jpg`, 1000 px; the first try at chunk 4, undone). Sky and
   water in `broad`, length 120–320, coverage 3.8, unclipped around open darks. The far
   band (20–30 units tall) came out milky, pale streaks crossed the whole bush, and the
   reflection got pale overlays. Strokes overshooting the mask is by design, but they
   carry the pickup hundreds of units at close to full strength (the same as sky item 2 in
   `sky.md`): a real brush dirties and gives up paint far sooner.
2. **A thin stroke into open paint ploughs tramlines** (`water_wet2_sheen_tramlines.jpg`,
   3200, units 380–700 × 520–660). A round 3 stroke (`pressure={0.1, 0.5, 0.3, 0.55, 0.05}`)
   laid into open water (medium 0.4, clock 0), then a short level blend, gives two parallel
   pale lines along the stroke's edges with a darker trough between them. This is the
   "ploughed river over wet paint" failure in miniature. Without the blend (round 2.4) the
   stroke is a single uniform hard wire.
3. **The filbert plough on thin open paint** reported in `sky.md` (item 1) didn't recur
   here: B's filbert strokes went onto the bare ground, not into wet paint.

## SKETCHBOOK CANDIDATE (§3 Water)
**Principle: the bank and its reflection are one dark shape. Lay them first and together,
then bring the water to them while they're wet, then fuse the reflection level.** Cut-out
reflections come from painting the water dry and then the reflection inside its own mask.

```lua
-- one sitting, clock 0, no dry()
-- 1. the darks: bank, earth and reflection unclipped; the reflection pulled DOWN
work(bush,  {hand="body", tool="filbert 6", color=bushcol, angle=function(x, y) return 1.2 + 1.2*turn(x, y) end,
  length={10, 26}, coverage=2.6, medium=0.3, load=0.8})
work(earth, {hand="body", tool="filbert 5", color=earthcol, angle=0.03, length={20, 60}, coverage=2.6, medium=0.3, load=0.8, hug=false})
work(refl,  {hand="body", tool="filbert 6", color=reflsrc, angle=math.pi/2, angle_jitter=0.05, length={12, 40},
  coverage=2.8, medium=0.35, load=0.8})
work(far + farrefl, {hand="broad", color=farband_col, angle=0, angle_jitter=0.01, curve={0, 0}, length={80, 240},
  coverage=3.6, medium=0.3, load=0.9})                 -- a far shore and its reflection: one band
-- 2. the lights around them, fenced by their own region grown a few units and softened
local skyr, openw = skym - bank - far - farrefl, water - refl - farrefl
work(skyr,  {hand="broad", color=sky_bands,   angle=0, length={120, 320}, coverage=3.8, medium=0.3, load=0.7, clip=skyr:grow(4):blur(2)})
work(openw, {hand="broad", color=water_bands, angle=0, angle_jitter=0.004, curve={0, 0}, length={120, 320},
  coverage=3.8, medium=0.3, load=0.7, clip=openw:grow(3):blur(2)})
-- 3. fuse the reflection level, fenced to the water so the bank stays
blend((refl:grow(8) + farrefl:grow(5)) * water, {angle=0, angle_jitter=0.004, length={40, 160}, coverage=2.0, clip=water})
```
(`sky_bands` and `water_bands` are the sky and water fields shifted ±0.018 and ±0.012 in L
by `noise{period=200, stretch={0, 7}}`; `water_B.lua` chunks 2–5 have them in full.
I used medium 0.4 for the lights; 0.3 should be quieter.)
*Ceiling:* a pale rim 3–8 units inside the reflection's edge, dark drips at the ends of
the vertical pulls, and no good way yet to lay a single soft light line into open water.
