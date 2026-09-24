# Evening at a Mountain Lake (`evening_lake`)

Round 7, painted on the wet-on-wet engine (branch `r7-paint-wet`, from `r6-wet`).

- Log: `paintings/lua/evening_lake.lua` (21 chunks; `easel check` replays it exactly:
  "replay matches the live canvas exactly (21 chunks, 72.0s)").
- Renders: `out/evening_lake.png` (1000) and `out/evening_lake_full.png` (3200, 418 s).
  `out/*.png` is gitignored, so they stay local; `easel run paintings/lua/evening_lake.lua
  --width 3200` remakes the full render.
- Sheet for Alice (lossless, built from the PNGs): `notes/paint1/evening_lake_alice.png`. It has
  the whole at 1000 px over three 1:1 crops of the 3200 render: the fir promontory against
  the glow, the far range with its mist and mirror, and the man on the bank.
- Evidence crops for the engine notes (JPEG looks at `--scale 3.2`):
  `evening_lake_crest1..4_*.jpg`.

## The idea
A still mountain lake just after sunset. The sun has gone down behind the far range, a
little right of center (`SUNX = 600`). The afterglow lies low over the lake and the water
gives it back. A thin young moon hangs above it in the blue. A dark fir wood on a
headland comes in from the left, and its spires break the glow. A long low range fades into
a band of mist on the far water. On the near bank a man in a long coat stands with a stick
on a small rise, seen from behind and looking out. Two stones lie at the water's edge.

The Friedrich parts are the contre-jour twilight, the empty middle of water and sky, the
spruces in short hatched strokes against the light, the stippled-looking mist, the
Rückenfigur and the moon (from his moonwatchers). I also followed his advice to Carus: one
dark glaze over the whole picture except the moon, growing darker toward the edges
(`notes/research/friedrich_materials.md` line 70, [MET p.35]). I didn't copy a known picture.
There is no rock on a summit, no sea and no pair of figures.

## Composition and palette
- Canvas 1.4:1 (1000 × 714), `friedrich_1820_greens` (the fir darks want Prussian blue),
  seed 47, hand time on. The sky and water use a restricted sky palette (lead white, smalt,
  cobalt, ochre, umber, red earth, chrome yellow and vermilion; no greens).
- The far water's edge is at `HZ = 432`, a little under the middle. The headland's skyline
  falls on a diagonal from the upper left (y ≈ 190) to a low spit at x ≈ 450. The range
  rises to a dome right of center (crest ≈ 366 at x 790). The near bank rises on the right
  to the man (x 802, feet at y ≈ 601). Moon at (676, 158).
- Three value families: the near-black headland, its reflection and the bank; the
  mid-value upper sky and range; and the light glow, which lies low in the sky and again
  in the water. The man stands in that light, above a band of the mirrored cloud.

## Order of work (7 sittings, hand time on)
1. **Drawing** (chunk 2): a 2H search, then 3B for the headland, range and bank, with the
   far water's edge ruled, then `fix()`.
2. **Dead color** (chunk 3): a thin, lean, one-color-per-region underpainting over the
   whole canvas. It is clipped to each region grown 1.5–3 units, because the unclipped
   first try threw dark stubs into the sky. Then `dry()` (21 days). The reason: Alice
   called the orange ground flecks in round 6's thin skies and water "horrible", and a
   covered, cool dead color gives thin sky paint nothing orange to show. The 3200 sky crop
   shows no ground flecks.
3. **Sitting 2, the distance wet** (chunks 5–9): the range first as one mass (filbert 6,
   clipped), with its mirror pulled down in vertical strokes. Then the sky brought down over
   the wet range top in banded long strokes, then a level blend. The cloud bank went into
   the open sky unclipped, with pale lights under it toward the glow and its top lost in a
   blend (sketchbook §2, the lab's sky B). Then the lake: the headland's reflection as one
   dark pulled-down mass, the open water around it (the sky mirrored, the bank included,
   darkening toward me) fenced by its own region grown and blurred, and a level blend
   (the lab's water B). Last, the range's mirror restated lighter and one calm band of mist
   along the far edge, both laid wet.
4. **Sitting 3, the headland over the dry sky** (chunks 11–13): the wood as one hatched
   near-black mass, 15 hand-placed `fir{}` spires poking above its skyline by 14–48 units,
   a small spit with reeds, four small firs down the steep flank and a front row of
   `fir_wood{}` standing at the waterline with bare trunks. The firs' geometry was grown
   in sitting 2, before any of it was painted, so the lake could mirror the spires wet
   (below).
5. **Sitting 4, the near bank** (chunks 15–19): the bank and two stones (`rock{}`, lit only
   from the sky) laid wet together. Then the man, grass from 5,200 candidate blades culled by a
   patch noise (denser along the bank's lip), the moon over the dry sky and the far crest restated over the dry range.
6. The glaze (chunk 20) and the varnish with relief at its default (chunk 21).

Hand time: 1,545 minutes (25.75 h), 89,723 strokes, 7 sittings. Sitting 3 ran to 17.8 h of
hand time against 3 h planned: the hatching of one fir wood is most of the painting's
hand time. I finished that passage in one sitting and then rested, rather than resting in
the middle of the wood.

## What I think works
- **At 1000 px it reads as a painting of a place at a time of day**, with a clear value
  plan. The eye goes glow → man → moon.
- **The fir headland.** The spires break the glow as a painter's firs do, in short
  hatches with bare trunks at the water, and the wood's own edge is soft, not cut. At 3200
  (sheet crop 1) the spires still hold.
- **The lake.** The glow and the cloud bank mirrored in the water, and the headland and
  its reflection as one dark shape, both came straight from laying the darks first and
  bringing the lights to them wet.
- **The man.** At 3200 he reads as a man in a long coat and cap with a stick, not a
  peg (sheet crop 3). It took four goes (below).

## What doesn't
- **The range's mirror** is blotchy. At 3200 it has pale, lacy, scraped-looking patches
  (sheet crop 2, lower right of the reflection) left by restating it lighter into the wet
  reflection, and the range and its mirror still read a little as one symmetric lozenge.
- **Pinholes of sky in the wood mass**: small pale specks across its upper half at
  3200 (sheet crop 1). They are gaps in the hatching over the sky strip I left inside
  the headland's edge.
- **The front row of firs is fairly evenly spaced** along the water, a little like a
  fence.
- **Two sky strokes read digital at 3200**: a short greenish vertical streak near
  (509, 163), and a hard-edged, striped lozenge in the low cloud bank near (581, 306).
- **The stones are dark lumps.** Their lit tops are still a patch of paler dabs at 3200,
  though much quieter than the first try.
- **The spit's reflection** is a smudged gray tongue.
- **The near bank** is a smooth, dark, low-incident band. The grass carries it at 1000 px
  but not much more.

## Engine problems hit (with crops)
1. **A sky brought down over a wet range left a double contour** at 3200: a dark line a few
   units above the ridge with a pale band under it (`evening_lake_crest1_double_contour_3200.jpg`,
   canvas 640–900 × 330–430). It looks like the range's wet paint was pushed up to the sky
   stroke's edge.
2. **Two wet fixes both failed.** Sky restated over the band, fully loaded and laid lightly,
   left a gray rim (`..._crest3_sky_restated_wet_3200.jpg`). Range color brought up over
   the pale band (dark into wet light, full load, pressure 0.3–0.45) broke into a lace of
   pale beads (`..._crest2_dark_into_wet_light_lace_3200.jpg`). The sketchbook says a
   loaded brush laid lightly stays clean for *lights into wet darks*, and in this picture
   it did. The reverse, a dark into wet light, still laces. That's the lab sky study's
   item 2/3 (`notes/lab/sky.md`), still present on this engine.
3. **Over dry paint the same move worked** (`..._crest4_range_up_over_dry_3200.jpg`): a
   clean found ridge. So on this engine a dark into light needs a set or dry light.
4. A `rect`-like region, even built from a function and blurred 3 units, printed a straight
   edge in the water (the first spit fix). Smoothstep fades on both axes fixed it: the
   sketchbook's §11 pitfall, confirmed.
5. `easel try` doesn't run before `canvas{}` exists, so the first chunk's geometry can't be
   previewed. I painted chunk 1 without overlays and tried the overlays after.
6. `body_of`: a short two- or three-point limb across the top of the head (a hat brim)
   curls into a hook with `char="firm"` and a raised `amount`. A wide first spine point
   makes the cap without it.

## SKETCHBOOK CANDIDATES
**1. A dead color stops the ground flecks (§1 or §2).** *Principle:* thin sky and water
paint shows whatever is under it where the strokes thin. Over the bare Friedrich ground
that is orange, and Alice called it "horrible". Lay a thin, lean dead color over the
whole canvas first, each region in its own dull value, and let it dry.
```lua
local dead = pal:only{"lead white","raw umber","bone black","yellow ochre","cobalt blue"}
work(skyM, {hand="body", tool="filbert 10", color=dull_sky, angle=0, angle_jitter=0.15, length={40, 120},
  coverage=3.2, medium=0.4, load=0.6, pal=dead, clip=skyM:grow(3)})   -- likewise water, range, land, bank
dry()                                                                  -- ~21 days of clock at medium 0.4
```
Clip each region (grown 1.5–3): unclipped, the darks throw stubs into the sky that ghost
through thin paint later. *Ceiling:* one more sitting and three weeks of clock. I've
only checked the flecks in this picture's sky, at 3200.

**2. A far crest: dark into light only when the light is dry (§4).** *Principle:* on the
wet engine a light laid into a wet dark stays clean, but a dark laid into wet light breaks
into a lace of pale beads. So restate a range's ridge after the sky has dried. If a
sky brought down over a wet range leaves a double contour, bring the range up over it
the next day:
```lua
crest2 = function(x) return crest(x) - 5.5 end            -- the ridge raised over the dark line
band = mask(function(x, y) local c = crest2(x)
  return smoothstep(c - 1, c + 0.5, y) * (1 - smoothstep(c + 10, c + 16, y)) end)
work(band, {hand="body", tool="filbert 4", color=function(x, y) return rangecol(x, y + 5.5) end,
  angle=function(x, y) return math.atan(crest(x + 4) - crest(x - 4), 8) end, length={20, 60}, coverage=2.8,
  medium=0.2, load=1.0, pressure={0.35, 0.5}, ramps={0.3, 0.3},
  clip=mask(function(x, y) return smoothstep(crest2(x) - 0.7, crest2(x) + 0.7, y) end)})
```

**3. Mirror a wood before you paint it (§3 water).** *Principle:* the reflection belongs
to the wet water sitting, but the wood belongs over the dry sky. So grow the trees'
geometry first (`fir{}` gives masks without painting), mirror their mask about the
waterline and paint that reflection with the water. The lake then shows the spires, and
not the flat polygon of the headland.
```lua
landM = woodM + firM
reflM = mask(function(x, y) local wl = PWL(x); if y <= wl then return 0 end
  return landM:at(x, wl - (y - wl)/0.78) end):soften(1)                -- 0.78: we look down a little
work(reflM, {hand="body", tool="filbert 6", color=dark_to_cooler, angle=math.pi/2, length={12, 40},
  coverage=2.8, medium=0.3, load=0.85, clip=reflM:grow(2)})
-- then the open water around it (fenced by openW:grow(3):blur(2)) and a level blend, as the lab's water B
```

**4. A small man from behind that holds at 3200 (§9).** *Principle:* build the cap into
the spine as its top point, not as a limb: a wide top over a smaller head and a short neck.
Bend the arms, curve the spine a little, flare the hem and give him a stick. Skip the rim
light: a rim offset by 1.3 units came out as a wide pale band down his side.
```lua
FIG = body_of{spine={{fx-0.3,fy-55.4},{fx,fy-52.8},{fx+0.2,fy-49.6},{fx+0.6,fy-46.2},{fx+1.7,fy-30},{fx+2.2,fy-13}},
  widths={6.8, 5.0, 3.9, 10.8, 9.0, 13.2},                                -- cap, head, neck, shoulders, waist, hem
  limbs={{{fx-1.6,fy-14},{fx-2.6,fy}, widths={2.8,2.0}}, {{fx+4.6,fy-14},{fx+5.4,fy+0.5}, widths={2.8,2.0}},
         {{fx-4.2,fy-45.2},{fx-5.8,fy-37},{fx-5.0,fy-28.5}, widths={3.0,2.7,2.2}},
         {{fx+6.2,fy-44.8},{fx+8.2,fy-35},{fx+8.8,fy-27.5}, widths={3.0,2.7,2.2}}},
  blend=0.4, char="firm", amount=1.1, seed=91}
work(FIG:mask(), {hand="detail", tool="round 1.6", color=dark, angle=1.57, length={2, 6}, coverage=5, clip=FIG:mask():grow(0.3)})
-- the stick: rigger 0.9 from the right hand to the ground, pressure {0.8, 0.6}
```
(At 56 units tall. A hat as a limb across the head curled into a feather. With a wide
first spine point of 8.2 the head came out as a bulb on a long neck.)

**5. Carus's glaze (§12 or §1 finishing).** One transparent dark glaze over everything
but the moon, falling off smoothly from the light:
```lua
local vig = mask(function(x, y) local dx, dy = (x - 610)/640, (y - 440)/470
  return smoothstep(0.45, 1.25, math.sqrt(dx*dx + dy*dy)) end) * (-moonM)   -- the moon's exact mask, not grown
glaze(vig, {color="#22232b", coats=0.32, pigment="transparent"})
```
It gathered the picture onto the glow and made the thin moon stand out.
