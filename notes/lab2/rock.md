# Lab 2: rock (hierarchical detail)

A new boulder: a weathered granite erratic in a summer meadow, afternoon light from the
left and a little in front, its shadow falling right. Canvas 1000 × 667 (3:2), a 300 mm
panel, Friedrich ground, seed 81. The shared setup `rock_setup.lua` (sky and meadow
colors, the erratic drawn as a broken `outline{}` with a rounded crown, a tucked-in left
foot and a fracture face on the right, one arris line, one brow line and one crack,
`rock{kind="granite"}`) is chunk 1 of both logs, byte for byte. Its planes: crown top
(plane 4, faces the sky, value 0.68), shoulder (plane 28, faces the sun, 0.82), lower face
(plane 29, turned down to the meadow, 0.52), fracture face in shadow (planes 21/33/36,
0.23–0.30).

## Key (the owner judges blind)
**P = H** (hierarchical detail), **Q = B** (round 1's rock B recipe). Drawn with
`random.SystemRandom().choice`.

| | B | H |
|---|---|---|
| log | `rock_B.lua` | `rock_H.lua` |
| 1000 px | `rock_B.jpg` | `rock_H.jpg` |
| 3200 crop (canvas 360–680 × 380–620: the lower lit face, the terminator, the foot, the start of the cast shadow) | `rock_B_crop.jpg` | `rock_H_crop.jpg` |
| sheet | `rock_sheet.jpg` (P left, Q right) | |
| chunks | 5 (setup, sky + meadow + dark family, lit face, foot, finish) | 6 (setup, the same chunk 2, lit side at the middle scale, particulars, contact, finish) |
| marks after chunk 2 | one lit pass (filbert 5), **321** scrubbed meadow strokes at the foot | 4 plane passes + 1 halftone band + 3 patches (filbert 4–8); **2** crack strokes; **45** tuft blades in 4 tufts, 12 lap strokes, 1 pocket pass; in the shadow 6 straddling strokes and 1 tuft of 8 blades |
| clock | 29,663 min (wait 1800 + 1800 + finish) | 29,663 min (wait 1800 + 1800 + 1800 + finish) |

Hand time was not used (the recipe B being tested has no sittings). Both 3200 renders need
a guard in chunk 2 (`rk:sample` returns nil on a few edge pixels at 3200; round 1's NY
function had no guard), added to both logs identically.

## What B did (round 1's rock B, adapted faithfully)
The rock B log from `notes/lab/rock_B.lua`, changed only where the drawing changed: the
cast shadow is kept off the lit front by `smoothstep(435, 575, x)` (the terminator here is at
x ~530–575, B's was ~540), the foot loop runs over this rock's base (x 272–750), noise seeds
moved. Chunk 2: sky, sunlit meadow with the rock and cast reserved, the dark family (shadow
side + foot + cast) edge to edge. Chunk 3 (30 h): the lit face in one pass, filbert 5,
value from `rk:value()` through B's four-stop gradient `#6a645a → #ada28c`. Chunk 4 (30 h):
321 short upward filbert-3 strokes of the meadow's own colors over the base.

## What H did
1. **Chunk 2 = B's chunk 2** word for word: the grounding (the dark family across the
   rock/ground edge) is B's.
2. **The middle scale** (chunk 3, 30 h, shadow setting). The lit side as its three planes,
   each one value and temperature, each with its own brush, stroke size and direction:
   - crown top (plane 4): cool gray `#a3a39c → #bdbbb0`, filbert 6, strokes `across`;
   - shoulder (plane 28): the lightest and warm, `#b4a58c → #cbb99a`, filbert 6, down the plane;
   - lower face (plane 29): a full step darker and warmer, `#85796a → #9a8c76`, greened by
     0.35 toward the foot (the meadow's bounce), filbert 8, strokes 18–44 long down the fall;
   - left rim (plane 37): a cool halftone `#7f7b72 → #8e897d`.

   Inside a plane the value moves only a little: 40% the pixel's light, 60% the plane's
   own, plus a slow noise (period 110). The plane borders are pushed around by a noise (up
   to 16 units) and blurred 5, so two planes meet along an irregular, mostly lost border.
   Below the brow the lower face darkens toward the shadow over its last 32 units
   (`mix(c, "#5f5950", 0.7*turn)`), and a halftone band (filbert 4, `mix(lower, shadow,
   0.6)`) is laid along the terminator, straddling it, while both sides are wet. That
   stretch of the terminator is lost; above the brow the shoulder meets the shadow crisply.
   Then three weathered patches, placed by eye with noisy borders, wet into wet: a
   gray-green lichen bloom on the crown, one rust stain running down the lower face and one
   dark weather streak from the brow. The shoulder is left alone.
3. **Selected particulars** (chunk 4, 30 h, lit side setting): one crack from the brow down
   the lower face, dark with a light lip on its sunny side, sharp at the top and thinning
   out to nothing halfway down. That's all.
4. **The contact** (chunk 5, 30 h): varied along its length. There's a small soft pocket of
   cast color under the tucked-in left foot. On the lit foot, four tufts of different size
   (7–16 blades, 20–38 tall, round 2.8, dark stalks with a few sunlit, set down at the root and
   lifted off) lap over the rock at irregular places. Low meadow strokes sampled from the
   canvas under them are laid along the ground, riding 0–4 units over the base in four spans,
   and the gaps are left as the plain meeting of rock and meadow. In the shadow, six broad
   strokes of the rock-foot/cast mix straddle the base, plus one dark tuft.

## What I see
- **P (H)** turns. The crown, the shoulder and the lower face are three different lights,
  so the lit side reads as a solid with a top and a front, not one slab. The lower face
  darkening into the shadow gives the rock a belly. The crack is the one drawn thing on the
  lit side and the eye goes to it. At 3200 the lit side is quiet: big soft strokes and
  patches, no even texture. The foot sits: dark into the shadow on the right, tufts and
  plain meeting on the left.
- **Q (B)** is grounded exactly as in round 1, but its lit face is again one light slab
  (crown and face barely separate: B's gradient spans only `#6a645a → #ada28c` by pixel
  light) covered in an even stroke texture. At 3200 the fringe of 321 short upright strokes
  is a comb along the lit foot, and a pale sliver (a thin lit arris that `rk:lit()` finds
  inside the shadow) shows at the base of the terminator.
- **Which looks better:** P, to my eye, clearly at 3200 and still at 1000.

## H's ceiling
- **The plane border under the shoulder is nearly horizontal** across the whole face, so the
  rock leans toward a block with a lid. That's the drawing (the tool's shoulder plane ends
  on a level), and the noise only breaks it a little.
- **The crown's lichen patch** is lighter and more mottled than I'd like at 1000; it reads a
  little as a snow or frost cap.
- **At 3200 the finish draws a thin dark-orange line along the lit base** and around every
  stroke that ends on the meadow: the relief and varnish crease at a step in paint
  thickness (absent before chunk 6 and present after it: `look --scale 3.2` before and
  after the finish). B hides it under its fringe. It's the same crease that peppers both
  rocks with dark flecks at 3200.
- **The lost terminator** still shows some broken paint (a lace of shadow through the
  halftone) at 3200; laid a day later (my first try) it was a pale torn strip.
- **The tufts are small at 1000**; they tell only at 3200.
- The cast shadow is still B's crisp-edged parallelogram.

## Wet-paint notes
- **A halftone laid across a setting terminator** (short strokes across it, the next day)
  came out as a row of pale beads. Laid *along* it as a band the next day, it broke into a
  pale lace. Laid along it while the lit paint was still wet (in the same chunk as the
  lower face, shadow setting), it fused. Lose an edge while both sides are open.
- **A thin dark line on setting paint** (the brow accent, round 1.2–1.3) came out dashed
  like stitching; a filbert 3 stroke there read as a ruled stick. The value step between
  the planes carried the brow on its own.
- **Pale tuft blades over a pale lit rock vanish**; grass is darker than a lit rock, so
  the blades have to be dark stalks with only a few lit.
- **A mask of "the meadow rising up the foot"** (two soft mounds) read as pasted green
  cushions; low strokes along the ground in the canvas's own sampled color didn't.

## SKETCHBOOK CANDIDATE
*A rock's lit side is its planes, not a texture: give each lit plane one value and one
temperature (a top turned to the sky cool, a face to the sun warmest, a face turned down to
the ground a full step darker and warmer), its own stroke size and direction, then two or
three large weathered patches wet into them and one found particular where the light
turns. Keep round 1's dark family for the grounding.*
```lua
-- chunk 2 as rock B (sky; meadow reserved; SHADEM + CASTM edge to edge)
-- chunk 3, ~30 h (shadow setting): group the lit planes by rk:sample(x+16*dx, y+16*dy).plane
--   (noise period 45 pushes the borders), :blur(5), * LITALL, where
--   LITALL = rk:lit(0.25) * ROCKM * (-SHADEM:shrink(3))   -- no light on thin arrises inside the shadow
-- color per plane: mix(lo, hi, 0.5 + 0.4*(t - 0.6) + 0.25*slow(x, y))   -- t = smoothstep(levels) of v
--   crown  #a3a39c-#bdbbb0 filbert 6 "across"; shoulder #b4a58c-#cbb99a filbert 6 "plane";
--   lower  #85796a-#9a8c76 filbert 8 "fall", length 18-44, + 0.35 green bounce near the foot,
--          -> #5f5950 by 0.7 over the last 32 units before the terminator (below the brow)
-- the lost terminator: a band |x - tx(y)| < 12 in the same chunk, mix(lower, shadow, 0.6), filbert 4
-- 3 patches by eye (noisy ellipses, blur 3), medium 0.2, coverage 2.2-2.4, wet into wet
-- chunk 4, ~30 h: one crack, round 1.4 dark + round 1.0 light lip, pressure 0.75 -> 0 down its length
-- chunk 5, ~30 h: 3-4 tufts (7-16 blades, 20-38 tall, round 2.8, dark stalks), low lap strokes
--   in the canvas's sampled color 0-4 units over the base in spans; gaps left plain
-- never: one gradient over the whole lit face, drawn accents along a plane border, an even fringe
```
Ceiling: the finish's relief crease draws a line at every thickness step, so the plain
spans of the contact show a hairline at 3200.
