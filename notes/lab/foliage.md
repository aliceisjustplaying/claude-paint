# Lab pair: foliage (lab1)

One oak crown in summer leaf against a clear afternoon sky, the trunk showing only
below the crown. Canvas 1000 × 750 (4:3), a 320 mm panel, Friedrich ground,
`friedrich_1820_greens`, seed 61. Shared setup: `foliage_setup.lua` (crown drawn as a
lopsided, lobed `outline{}`, trunk, `tree_in{species="oak"}`, sun azimuth −120°,
elevation 42°). Both logs start with it byte for byte.

| | A (old way) | B (new way) |
|---|---|---|
| log | `foliage_A.lua` | `foliage_B.lua` |
| 1000 px | `foliage_A.jpg` | `foliage_B.jpg` |
| 3200 crop (canvas 600–920 × 250–490, right crown: holes and edge) | `foliage_A_crop.jpg` | `foliage_B_crop.jpg` |
| chunks | 6 (setup, sky, stipple, wood, leaves, finish) | 9 (setup, sky around a reserved crown, mass, lights, edge, limbs, top lights, trunk glaze, finish) |
| marks in the crown | wood fill + ~4,000 limb strokes, a hatch pass over all leaves, sky into all gaps, **41,421 touches** | one dark-mass pass (filbert 7), two light passes (filbert 4 and 3), **460** edge flicks, **7** limb strokes, **28** top touches |
| clock | 54,178 min (two `dry()`s, 12 days each) | 31,577 min (most of it one deliberate wait for the hole sky to dry before the limbs, and the trunk glaze) |

## What A did
Sketchbook §2 sky (broad + blend, next day stipple, clipped off the land), `dry()`,
the tree_in wood recipe (§5 step 1), `dry()`, then §5 steps 2–4 in full: the leaf body
hatched from `t:light()`, sky into `t:gaps()`, five touch layers. An honest best effort.
The first stipple spilled speckle over the land edge, so I clipped it.

## What B did
1. **Planned the masses first** (chunk 2): the leaves blurred into one silhouette
   (the fine lobes kept at blur 2.5, one solid inside at blur 8), and the light grouped
   into families (`t:light()` blurred and normalized). Seven sky holes were **drawn**
   where the limb masses part and a limb crosses, not taken from `t:gaps()`, whose
   holes at this scale are specks.
2. **The sky alla prima AROUND the reserved crown and trunk** (two thin broad
   layers wet into wet, one light blend, no stipple).
3. **The crown as one dark mass** (chunk 3, 1.5 h later, sky still open): filbert 7,
   thin (load 0.55, medium 0.3), shadow colors only, meeting the wet sky edge to edge.
4. **Lights into the SETTING dark** (chunk 4, 15 h later): the half-light masses
   (filbert 4, medium 0.12, `hug=false`) and a smaller top light, only where the
   grouped light says so.
5. **The edge decided by hand** (chunk 5): 460 flicks pulled from inside the setting
   mass out past the contour, each in its own light family.
6. **One limb per hole** (chunk 6), a single round-brush stroke on the stretch of the
   stoutest limb crossing it, after the hole sky had dried.
7. 28 top touches; a transparent glaze on the trunk; the common finish.

## What I see
- **A at 1000 px** reads as a convincing, leafy oak. It is the more "finished" image
  at arm's length. Its light is spread evenly over the crown in small flecks, so the
  crown reads as textured more than as a lit volume. **At 3200** it's the known
  failures: an even carpet of little touches (confetti) and torn-paper sky holes with
  crisp, pale rims (`foliage_A_crop.jpg`).
- **B at 1000 px** reads as one tree in sunlight: a few big unequal light masses on
  the sun side, a dark body, a handful of holes with branches in them. It's broader
  and more painterly. **At 3200** the surface is quiet, the holes have soft edges and
  hold one drawn limb each, and there's no confetti (`foliage_B_crop.jpg`).
- **Which looks better:** B, narrowly. Its big light-and-shade makes a volume and
  its surface holds up close. A wins on leafiness at arm's length. It isn't a
  knockout.

## B's ceiling (what the new way still gets wrong)
- **The shade half is one flat dark.** A cool veil to turn the front shade masses
  (tried twice) laid flat gray slabs over the tacky dark (see wet notes), so I dropped
  it. The right half has no volume at all.
- **The lit masses have a spongy mottle** (the light paint breaking over the setting
  dark). It reads as leaves at 1000 px but as moss or sponge close up.
- **The silhouette is soft and a bit blobby** (cumulus). The edge flicks help on
  the sun side but come out even around the contour.
- **Every hole has a smoky soft halo**: the dark mass thinned where it met the wet
  sky. It's softer than A's torn paper but reads as a blur.
- The relief lighting embosses the smooth sky into swirls in both versions (not a
  painting difference).

## Wet-paint misbehavior (for the engineer fixing wet behavior)
Crops are in `wet/`. The clock is "minutes after the paint under it was laid".
1. **Dark over open light paint plows into a pale lace** (`wet/1_dark_into_wet_sky.jpg`).
   A filbert 7 dark (#1f281b–#3e4a2c) laid over the open two-layer sky (~30–90 µm) left
   a pale ridge around every stroke: the sky paint is pushed to the stroke edges instead
   of mixing into a muddy green. It's the same at 1.5 h and 7.5 h (open) and only
   milkier at 15.5 h (setting). The trunk showed it too. A real brush would pick the
   light up and gray the dark. Workaround: reserve the area and meet edge to edge.
2. **A clean blender along a wet dark/wet sky seam exposes the ground**
   (`wet/3_blender_on_seam.jpg`): orange ground specks all along the seam. The same
   thing happened on the rock (a terminator blend reopened the crack with orange
   specks). A blender should fuse the two films, not lift them to the ground.
3. **Light into open dark plows too** (`wet/2_lights_into_dark.jpg`, left): a camouflage
   mottle with orange ground in it, even over a thin dark (18–35 µm). Probes then read
   **2,774–3,477 µm** of wet film at one spot (it was 500–585 µm over the thicker
   first dark): paint is heaped up, not spread. Into **setting** dark (15 h) the same
   pass is soft and leafy (used). Into **tacky** dark (24 h) it lies as flat poster
   slabs with mask-shaped edges.
4. **Sky laid into the setting dark gives white cotton puffs**
   (`wet/4_light_into_dark_and_tacky_veil.jpg`, left): lumpy, whitish, 246–625 µm thick
   (the default `aim` loads heavily over a dark; masstone aim at load 0.5 barely
   helped). Workaround: reserve the holes before the dark.
5. **A thin veil over tacky paint makes a flat slab in the exact shape of its mask**
   (same image, right), even at coverage 0.9, load 0.35, `hug=false` and a color only a
   step off the dark (L 0.39 over 0.33). The strokes don't read as strokes.
6. **A dark stroke over setting sky paint is translucent gray** (`wet/5_limb_over_sky.jpg`,
   3200): a loaded round 3 laid a streaky gray line over a hole painted a day
   earlier. Over the same sky once dry it's a clean dark limb. This agrees with the
   sketchbook's "fine lines only on dry paint".
7. Minor: land strokes laid at the horizon picked up the open sky and dragged pale
   streaks into the land's top edge (in both versions).

What worked wet: **lights into setting dark**, and **reserving** where a dark meets a
light (sky, holes, trunk) so the two only touch at a line.

## SKETCHBOOK CANDIDATE (narrow win; confirm on a second crown)
*A crown is a reserved dark mass, a few lights laid into it while it sets, drawn holes
and one limb per hole. Paint the masses, not the touches.*
```lua
-- plan: silhouette with its lobes, one solid inside; light grouped; holes drawn where limbs cross
local lv = t:leaves()
local edge = lv:blur(2.5):map(function(v) return smoothstep(0.3, 0.55, v) end)
local core = lv:blur(8):map(function(v) return smoothstep(0.3, 0.55, v) end):shrink(14)
HOLES = (union of 5-8 ellipses 10-22 x 7-12 units on limb crossings):roughen(5, 9, seed, 1.5)
MASS = edge + core - HOLES
-- LB = t:light():blur(10) / t:leaves():blur(10)   (light, grouped)
-- 1. sky around the crown: sk = skym - MASS:shrink(1.5):soften(1.5) - trunk; clip=sk, no stipple
-- 2. same session +1.5 h: the dark, edge to edge
work(MASS, {hand="body", tool="filbert 7", length={10, 24}, coverage=2.6, medium=0.3, load=0.55, clip=MASS,
  color=deep/body/sunny-shade by LB})
-- 3. +15 h, the dark SETTING: lights (medium 0.12, hug=false) on LB 0.52-0.68, top on LB > 0.72
-- 4. same session: ~450 flicks from 3-6 inside the contour to 3-9 outside, round 2.2-2.6,
--    pressure {0.7, 0}, colored by LB at the root
-- 5. hole sky DRY: per hole, one stroke along the stretch of the stoutest limb crossing it
```
Pitfalls: never lay the dark over wet sky, sky into the dark, or a veil over tacky
dark (see above). Ceiling: the shade half is flat and the lit masses are spongy.
