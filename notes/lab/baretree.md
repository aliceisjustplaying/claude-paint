# Lab: a bare winter oak (baretree), old way vs new way

One bare oak against a pale winter sky over a low ground, 300 mm panel, 4:3, seed 61.
Shared setup: `notes/lab/baretree_setup.lua` (canvas, sky and ground color fields, a
lopsided crown `OAKCROWN`, trunk `OAKTRUNK`, `tree_in{species="oak", season="winter",
seed=11, girth=0.06}`: 5,525 limbs, of which 4,474 are twigs). Logs:
`baretree_A.lua` (9 chunks), `baretree_B.lua` (12 chunks). Chunk 1 of both is the setup,
byte for byte.

- Images: `baretree_A.jpg`, `baretree_B.jpg` (1000 px); `baretree_A_crop.jpg`,
  `baretree_B_crop.jpg` (3200 px, the crown's right edge, canvas 640–940 × 180–400).
- Wet-paint failures: `baretree_wet/*.jpg` (live-session crops at 1000 px, described below).

## A, the old way (sketchbook as written)

1. Sky, first thin layer a shade duller (broad, coverage 4.2, medium 0.3, blend).
2. `wait(24*60)`, stippled second layer (§2).
3. `dry()`, the ground as tone (body, lengths 20–60).
4. `dry()`, sky cut back over the crenellated crest and blended (§4).
5. `dry()`, the oak by §5: `wood(3.5)` in body paint, lit flank on `wood(7)`,
   `paint_wood` round 2.4 for 1.2–3.5, **every twig** with a rigger 0.55 at pressure
   0.04, dead leaves `share=0.4`.
6. `dry()`, cast shadow as a blurred glaze (0.3 coats).
7. About 900 rigger grass blades (§8): 320 in a tuft to bury the trunk's round foot (§5
   "Roots"), the rest over the near ground.
8. Finish. Marks: every limb under 3.5 wide stroked once (about 5,500), ~200 dead-leaf touches, ~900 blades,
   plus the area passes.

What I see: at 1000 px a lacy, even "spiderweb" crown. The fishbone twig sprays at
every limb end read as little stars across the crown, and the lace is the same density
everywhere, so the crown has no mass. At 3200 (`baretree_A_crop.jpg`) the sprays float:
many clusters sit in the air with no visible parent, a thick limb ends bluntly
mid-crown, and the dead leaves are orange flecks. The foot needed a straw tuft to hide
the trunk's round flare, and it reads as a broom; the foreground blades read as a
barcode. The lit flank (`#7d7566` on dry paint) barely shows. Credit where due: the
sky's two layers are the best sky of the pair, and at 3200 the lace has real drawing in
it.

## B, the new way

1. Sky in one thin layer at the target color, a second soft broad pass laid into it
   wet, blended (no stipple).
2. Ground laid **into the open sky** (no `dry()`), `clip=land` so strokes don't start in
   the sky; a clean blender along the crest while both were open → a soft far edge with
   no cut-in. A second firm body pass restated the band just under the crest (see
   failure 1).
3. **The twig mass as a tone**, straight into the open sky: the thin wood's density
   `DEN = oak:wood(0, 1.2):blur(20)` mapped by `smoothstep(0.03, 0.22)`, filled with
   lean radial strokes (round 2, lengths 8–20, coverage 1.6, load 0.35, pressure
   {0.45, 0.05}) that pull the sky 40% toward `#5a5048` by density. No twig is traced.
4. `wait(24*60)` (sky tacky; `drying()` printed): the wood as one connected dark:
   `wood(3.5)` body, `paint_wood` round 2.4 for 1.2–3.5 wide, rigger 0.9 for 0.7–1.2
   pressed to the limb's own width. That's 261 limbs, and since a child is never wider
   than its parent, nothing painted floats.
5. The same wood restated at once into its own wet dark (the first pass skipped on the
   tacky sky).
6. Into the wet dark: the lit flank (the low sun from the left) on `wood(6)`'s sun side,
   plus 45 twigs at the crown's edge, each one attached to painted wood.
7. The ground's own color (sampled beside the trunk) dragged over the wet foot in a
   small roughened ellipse, so the trunk goes into the ground.
8. The cast shadow as a glaze starting inside the foot (so trunk, contact and shadow are
   one dark shape).
9. After the ground had set, the twig tone a second time, weighted to the crown's outer
   part: `OUTER = DEN * (0.2 + 0.8*(1 - smoothstep(25, 110, crowndist)))`, minus the
   exact `wood(1.2)` mask, mix 0.45 toward `#554a42`.
10. Finish. Marks: about 570 limb strokes (261 limbs × 2 plus 45 edge twigs) against
    A's ~5,500, with no blades and no dead leaves. The two tone passes are area passes.

What I see: at 1000 px it reads as a winter oak. The crown has a mass: a warm, soft
fringe where the fine twigs crowd at the rim, with sky through the middle. The big
limbs are clear and dark at the fork and run out into the fringe. The trunk has a lit
flank, it goes into the ground, and its dark continues into the cast shadow. The near
ground is quiet (no barcode).

## Verdict

**B looks better to me at the size the picture is seen.** It says "bare oak in
winter" with a tenth of the wood marks. The crown has a shape and a value, while A's
reads as a uniform web of stars. A is better only in the 3200 crop, where its lace has
real linear drawing and B's tone turns to fur.

## B's ceiling (what the new way still gets wrong)

- **At 3200 the twig tone is fur or cotton wool** (`baretree_B_crop.jpg`): soft gray
  tufts made of short hairs, not a mass of twigs. The strokes don't branch, and the
  tone's outline follows blurred blobs of the density (lumpy lobes with round holes).
- **The 0.7–1.2 limbs are wiry noodles.** They are evenly thick, curl a lot and stop
  in mid-air at the tone. This comes from the scaffold's crook and zigzag, and B paints
  it as generated. A painter would straighten and drop half of them.
- The first tone is barely visible after it dries; it took a second pass to register.
- The foot is still a little abrupt, and the sky shows faint warm wisps where the thin
  single layer left the ground breathing through (the stippled sky of A is smoother).
- The relief lighting (default, both versions) embosses the sky's strokes.

## Wet-paint misbehavior (for the engine fix), crops in `baretree_wet/`

1. **Body strokes laid into open sky plow it up** (`crest_no_blender.jpg`). The ground
   strokes that started over the open sky, or crossed the band of sky paint under the
   crest, dragged pale smears a few units down into the ground. `clip=land` plus a firm
   restatement of the band fixed it (`crest_restated.jpg`). This is plausible as paint;
   the size of the smears is on the heavy side.
2. **A glaze-hand stroke over the OPEN thin sky wipes it to the ground**
   (`tone_try4_glaze_lifts_crop.jpg`): `hand="glaze"`, filbert 6, medium 0.85, over a
   ~40 µm sky on the red ground. The probe in a stroke read `#aa8b7c`, open, wet 15 µm:
   the stroke tracks are salmon-colored ground. A thin glaze shouldn't strip a sky like a
   rag. **Engine suspect.**
3. **A clean blender over the open thin sky also lifts it to the red ground**
   (`tone_try6_blender_lifts.jpg`: salmon dashes in the crown after `blend` at coverage
   1.2 over a lean filbert veil). **Engine suspect.**
4. **Glaze-hand strokes over tacky sky craze into a mosaic**
   (`tone_try5_glaze_on_tacky_mosaic.jpg`): after `wait(600)`, where `drying()` said
   "tacky", the veil came out as pale cells with dark borders, like crackle; where it
   said "setting" it was a soft transparent dab. **Engine suspect.**
5. **Glaze-hand strokes lift each other even on a dry sky**
   (`tone_try7_glaze_on_dry_rims.jpg`, after `wait(3*24*60)`): each new stroke picks up
   the previous fresh glaze stroke, so overlaps leave a net of darker rims ("brain
   coral"). The brush-free `glaze()` verb doesn't do this. **Engine suspect.**
6. **Dark wood laid into open or setting sky turns mottled gray**
   (`wood_into_open_sky_crop.jpg`: 0 h, trunk L 0.51 with pale flecks; the same after 8 h).
   On the **tacky** sky a day later it skips and leaves pale flecks instead
   (`wood_on_tacky_flecks.jpg`). A second pass into its own wet dark closed them. Real
   paint does gray a dark laid into wet white, but this much churn from a loaded round
   brush at medium 0.15 looks too strong.
7. **A light laid into the wet dark gets swallowed** (the flank at `#8a8072`, load 0.6
   disappeared entirely). It needed `#aa9e8b`, load 0.85, medium 0.1.
8. **Long shadow strokes into open ground came out as streaky blue-gray smears with
   pale rims** (`shadow_strokes_into_open_ground.jpg`): filbert 6, `color_over` darkened
   relative to what's under it. B went back to a glaze for the shadow.

Timings seen: the thin sky (medium 0.3) stays "open" for about 6 h, is "setting" at
8–10 h, "tacky" at 1–2 days and mostly "dry" at 3 days. The body-paint ground was
still "open" after a day.

## SKETCHBOOK CANDIDATE: a bare tree is its connected wood plus ONE twig tone

*Principle.* Paint the wood that carries the tree and say the twigs as a tone. Don't
trace the twigs. The fine twigs of a winter crown read as a warm, translucent fringe
thickest at the rim, and they must never float. So paint every limb down to a width you
choose, pressed to its own width (a child is never wider than its parent, so the painted
wood is connected by construction), and lay the rest as tone from the twigs' own
density.

```lua
-- after the sky and ground lay-in, into the OPEN sky (no wait):
DEN = t:wood(0, 1.2):blur(20):map(function(v) return smoothstep(0.03, 0.22, v) end)
FX, FY = <the fork>;  RADIAL = function(x, y) return math.atan(y - FY, x - FX) end
work(DEN, {hand="body", tool="round 2", length={8, 20}, coverage=1.6, load=0.35, medium=0.25, hug=false,
  angle=RADIAL, angle_jitter=0.3, pressure={0.45, 0.05}, ramps={0.1, 0.8},
  color_over=function(x, y, under) return mix(under, "#5a5048", 0.4 * DEN:at(x, y)) end})
-- next day (sky tacky): the wood, then the same again at once into its own wet (closes the skips)
work(t:wood(3.5), {hand="body", tool="round 2", length={4, 12}, coverage=3, angle=1.5, clip=t:wood(3.5), color=dark, medium=0.15})
t:paint_wood(brush("round", 2.4), {color=dark, min=1.2, max=3.5})
t:paint_wood(brush("rigger", 0.9), {color=dark, min=0.7, max=1.2})   -- NOT below 0.7
-- ...restate the three, then the lit flank into the wet dark, clipped to itself:
local st = t:wood(6); local fl = st * mask(function(x, y) return 1 - st:at(x - 4.5, y + 1.8) end)  -- offset toward the sun
work(fl, {hand="body", tool="round 1.6", length={4, 10}, coverage=3, angle=1.5, clip=fl:grow(1):soften(1),
  color="#aa9e8b", medium=0.1, load=0.85})
-- once the rest has set: the tone again, weighted to the rim, cut by the EXACT stout-wood mask
local cd = t:crown_mask():distance()
OUTER = mask(function(x, y) return DEN:at(x, y) * (0.2 + 0.8 * (1 - smoothstep(25, 110, cd:at(x, y)))) end)
work(OUTER - t:wood(1.2), {<the same strokes>, color_over=function(x, y, u) return mix(u, "#554a42", 0.45 * OUTER:at(x, y)) end})
```

*Pitfalls.* Cutting the tone off the wood with a **grown** mask left pale halos beside
every limb; not cutting it at all grayed the fork. Cut the stout wood only, with its
exact mask. A glaze-hand veil or a blender over a thin open sky lifts to the ground.
`glaze()` through the density reads as a smoke cloud. Scumble (fan 5–7) gives a
cauliflower or an opaque wreath. *Ceiling:* at 3200 the tone is fur, and the 0.7-wide
limbs are wiry and curly as grown. Next try: fewer, straighter limbs chosen by hand, and
a tone whose strokes fork.
