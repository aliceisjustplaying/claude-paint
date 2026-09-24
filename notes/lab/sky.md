# Lab study: a quiet evening sky over a low dark horizon strip (lab2)

Setup (`notes/lab/sky_setup.lua`, shared by both logs byte for byte): Friedrich style, 300 mm
panel at 3:2 (1000 × 667 units), seed 61, sky palette without greens (plus chrome yellow and
vermilion for the glow). Geometry: a low strip under a noisy crest at y ≈ 550–566; a glow
centered at x = 650 whose width wanders with a noise; two cloud banks (a thin, broken high
band at y ≈ 196 and a long low bank at y ≈ 452 that thins and parts over the glow); one color
field each for the sky, the clouds (gray-violet bodies, undersides lit warm toward the glow)
and the strip.

| | log | render | 3200 crop (units 380–780 × 400–590) |
|---|---|---|---|
| A, the old way | `sky_A.lua` (7 chunks) | `sky_A.jpg` | `sky_A_crop.jpg` |
| B, the new way | `sky_B.lua` (8 chunks) | `sky_B.jpg` | `sky_B_crop.jpg` |

`easel status` doesn't report stroke counts; the chunk counts and the moves below are exact.

## A: the sketchbook as written
1. The first sky layer a shade duller than the target: `broad`, coverage 4.2, medium 0.3,
   then `blend` (sketchbook §2).
2. `wait(24*60)`: `drying` said `tacky`, so `dry()` (the clock jumped 5.7 days) and the
   second layer stippled over it: width 2.6, coverage 2.6, lifted L +0.015 (inside the
   0.006–0.028 the sketchbook allows).
3. `dry()`, then the banks inside their masks: `broad`, coverage 3.5, `aim=1.6`,
   `clip=clouds`, then `blend(clouds, {clip=clouds})`.
4. `dry()`, then the lit undersides scumbled warm and the tops veiled darker with a glaze
   brush (`color_over={shift={-0.04, 0, -0.012}}`), then blended. The first try (coverage
   2.2, `#efc890`) gave opaque cream slabs and was undone; the kept one is coverage 1.4 in
   `#d7ae86`.
5. `dry()`, then the strip in near-level body strokes, clipped. The crest came out clean,
   so the sketchbook's sky cut-in (a fix for crenellated crests) wasn't needed.
6. The finishing chunk. Five `dry()` calls; 22 days on the clock.

**What I see.** An evenly graded sky, "too neat", with two cigar-shaped cloud cutouts
pasted on it. Each bank is a finished object in its own silhouette. The lit undersides
are bright shapes with hard lower edges, and the right bank's light breaks into three
round blobs. At 3200 (`sky_A_crop.jpg`) the stipple layer is **salt-and-pepper**: gray
specks over the whole glow, even at L +0.015. The dark strip is the one clean passage: a
crisp, flat, believable land silhouette.

## B: the Round 6 principles
1. **Masses, one sitting.** The sky as unequal horizontal bands of thin paint: `skycol`
   shifted by a slow stretched noise (`period=180, stretch={0, 7}`, ±0.022 L), `broad`,
   length 120–320, coverage 3.8, medium 0.4, load 0.7, then a light level blend
   (coverage 1.2, lengths 150–400). No second layer, no stipple.
2. **Banks into the open sky, no wait, unclipped.** `work(clouds:blur(3), {hand="broad",
   hug=false, curve={0, 0}, length={90, 260}, coverage=2.2, medium=0.35, load=0.9,
   pressure={0.45, 0.7}, ramps={0.25, 0.35}})`. The edges pick up the wet sky, so the banks
   sit *in* it.
3. **A few unequal lights, then lost tops.** Warm light only along the low bank's bottom
   contour, a band straddling its edge (`low:grow(4) - low:shrink(12)`), fading away from the
   glow (`exp(-((x-650)/380)^2)`), broken into 3–4 runs by a coarse noise: `broad`,
   length 70–180, coverage 1.5, pressure {0.35, 0.5}. The high band keeps no lights. Then
   a level blend over the upper half of each bank (coverage 2.4) loses their tops into the
   sky. The undersides stay found where they face the glow.
4. **The strip in a second session.** `wait(480)` (the fat sky was still `open` at 5 h and
   `setting` at 8 h), then one dark body pass, clipped so the crest is found against the
   glow, and a level blend along the crest away from the glow (lost at the ends).
5. **Closing the crest.** A fuller, stiffer pass over the top 22–34 units of the strip.
6. **Next morning** (`wait(24*60)`): a thin `color_over` veil (70% toward the strip color)
   over that band to knock the pale streaks down.
7. The finishing chunk. Clock before finishing: 32 hours, no `dry()`.

**What I see.** A sky that reads as air and paint, not a ramp: the banks belong to it, the
glow is uneven, and the high band is barely there, as it should be. The low bank has lost
tops and a warm, found underside only where the glow hits it. At 3200 (`sky_B_crop.jpg`)
the paint is smooth, with no stipple speckle. The strip is B's weakest passage: its top
20–30 units are streaked with pale gray strata, which read as mist at 1000 px but look
accidental. There's also a thin pale seam and a few dark flecks along the crest at 3200.

## Verdict
**B looks better.** The sky, which is the subject, is clearly better: no cutouts, no
speckle, a glow with structure. A's only better passage is the strip, and only because
it went down over a dry sky. If I were to pick one picture to hang, it would be B.

## B's ceiling (what the new way still gets wrong)
- **The dark into the light.** Any dark laid over a wet light sky fought it (see below). B
  paid for it with three chunks and still has streaky strata. Next time: stop the sky short
  of the crest (or lay the strip first and bring the sky down to it), so the dark never has
  to cover open light paint.
- **Relief ripples.** Fat sky paint (medium 0.4, coverage 3.8) shows wavy ridges under
  `relief()` at 3200 (`sky_B_crop.jpg`, across the glow). "Quiet surface" wants thinner,
  leaner paint (medium ≤ 0.3, lower coverage) than I used. The Friedrich ground's crossing
  strokes also show through both skies as curly swirls at 1000 px (upper right in both
  JPEGs).
- **Stroke ends.** A few bank strokes still end in rounded blobs, and one pale lozenge in
  the high band (x ≈ 620, y ≈ 215) reads as a stamp.
- **Ground flecks.** Thin broad strokes leave small warm specks of the red ground in the
  upper sky. At 1000 px they read as warm cloud bits; they're a coverage accident, not a
  decision.

## Wet-paint misbehavior (for tonight's wet fixes)
1. **A filbert ploughs thin open sky down to the ground** (`sky_wet1_filbert_plough.jpg`,
   3200, units 380–620 × 160–240). Hand strokes with `brush{kind="filbert", width=6}`,
   `reload(c, 0.55)`, `pressure={0.25, 0.6, 0.55, 0.15}` along a bank, laid straight into
   the just-painted sky (medium 0.4, clock 0): each stroke left a thin pale ribbon with
   bright orange ground exposed along both sides. The mark is 2–3 units wide, not 6. The
   same bank laid with `work(..., {hand="broad"})` didn't plough. (This attempt was undone;
   it isn't in the log.)
2. **A dark body stroke through open light paint carries the light far and at full
   strength** (`sky_wet2_strip_into_open.jpg`, 3200, units 120–520 × 520–620). The open
   sky reaches only 10 units under the crest (`skym` = above crest + 10), but pale wedges
   the color of the sky reach 40–60 units down into the dark strip (body, coverage 3,
   medium 0.3, clip=land, no wait). A real brush would mix the pickup to a mid gray
   within a stroke length; here it's redeposited nearly unmixed. (Undone; not in the log.)
3. **Dark over *setting* light paint fails to cover** (`sky_wet3_strip_into_setting.jpg`,
   3200, same window). After `wait(480)` (`drying` = `setting` at the crest), the same
   dark pass left the top 20–25 units as pale streaks where the sky shows through. Over
   the dry sky in A the identical pass (`work(land, {hand="body", ...})`) covered fully.
   Setting paint should take a new layer (maybe with a little drag), not repel it. This
   one is in B's log (chunk 5); chunks 6–7 repair it.

## SKETCHBOOK CANDIDATE (§2 Skies)
**Principle: a sky is one wet sitting. Lay the banks into the open sky, not onto a dry one;
light them only where they face the glow; lose their tops.** Cutout clouds come from
finishing each bank inside its own mask over dry paint.

```lua
-- one sitting, clock 0, no dry()
sn = noise{seed=31, octaves=3, period=180, stretch={0, 7}}     -- bands, not a ramp
work(skym, {hand="broad", color=function(x, y) local v = sn(x, y); return shift(skycol(x, y), 0.022*v, 0.004*v, 0.008*v) end,
  angle=0, angle_jitter=0.02, length={120, 320}, coverage=3.8, medium=0.3, load=0.7, pal=skypal})   -- 0.3, not B's 0.4
blend(skym, {angle=0, coverage=1.2, length={150, 400}})
cm = clouds:blur(3)                                              -- banks INTO the wet sky, unclipped
work(cm, {hand="broad", color=cloudcol, hug=false, curve={0, 0}, angle=0, angle_jitter=0.02, length={90, 260},
  coverage=2.2, medium=0.35, load=0.9, pressure={0.45, 0.7}, ramps={0.25, 0.35}, pal=skypal})
-- lights: a band straddling the low bank's bottom contour (its center line c2), fading away
-- from the glow, broken into 3-4 runs by a coarse noise; the high band gets none
local pn = noise{seed=44, octaves=2, period=160, stretch={0, 3}}
local low = bank2:blur(3)
lightm = ((low:grow(4) - low:shrink(12)) * mask(function(x, y)
  return smoothstep(c2(x) - 2, c2(x) + 8, y) * math.exp(-((x - SUNX)/380)^2) * smoothstep(-0.2, 0.2, pn(x, 0))
end)):blur(2)
work(lightm, {hand="broad", color=function(x, y) return mix("#c9a58e", "#ecc48e", math.exp(-((x - SUNX)/220)^2)) end,
  hug=false, angle=0, length={70, 180}, coverage=1.5, medium=0.3, load=0.85, pressure={0.35, 0.5}, ramps={0.35, 0.4}, pal=skypal})
-- lost tops: a level blend over the upper half of each bank (c = that bank's center line)
topsm = (cm:grow(6) * mask(function(x, y) local c = y > 320 and c2(x) or c1(x)
  return 1 - smoothstep(c - 2, c + 8, y) end)):blur(5)
blend(topsm, {angle=0, coverage=2.4, length={60, 200}, hug=false})
```
Keep the dark land off open sky paint: stop the sky at the crest, or lay the land first.
*Ceiling:* the land strip over the wet sky (B chunks 5–7), relief ripples in fat sky paint,
the occasional blob at a stroke end.
