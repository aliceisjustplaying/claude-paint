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

## C (lab round C: the best of A and B)

| | C |
|---|---|
| log | `foliage_C.lua` (same setup chunk as A and B, byte for byte) |
| 1000 px | `foliage_C.jpg` |
| 3200 crop (the same window, canvas 600–920 × 250–490) | `foliage_C_crop.jpg` |
| chunks | 7 (setup, plan + sky, modeled dark, half-light + touches, edges, wood + accents, finish) |
| marks in the crown | one modeled dark pass (filbert 6), one half-light pass (filbert 4), **2,831** of the tree's 38,586 leaf touches (685 mid, 1,383 lit, 235 top, 528 cool), **700** edge leaf dabs + 120 soft ones, the wood of the fork, 79 hanging leaves, **2** hole limbs, **22** top lights |
| clock | 41,194 min at 1000 px (33,643 at 3200: see the wet notes) |

### What C did differently
The panel split 2–2 (A for its leafy lit masses and directional light, B for its grouped
lights and softer edges) and agreed on two faults in both: the shade half has no internal
structure, and the edges have no hierarchy. So C:
1. **Models the crown as ~22 overlapping leaf masses** (chunk 2). The tree's 6,479 clumps
   grouped by k-means, each group an ellipse (unequal: radii × 0.8–1.35) with a height that
   puts the lower masses in front. On a 4-unit grid: which mass is in front, its own light
   (sun 0.7 + sky on its top 0.3) and a **crease** where it overhangs the mass behind.
   A smoother crown-scale light (`BIG`, B's light blurred 28) decides which side is lit,
   and each mass models by its own amount (0.35–1) so they aren't all the same dome.
2. **Sky around the reserved silhouette, no shrink** (B reserved it shrunk and softened, which
   gave every hole the same smoky halo). The sky set 15 h, **then** the dark met it edge to
   edge: no halo, no lace.
3. **The dark in one pass, but modeled** (chunk 3): filbert 6 strokes *wrapping around their
   own mass* (tangent to it) instead of B's noise angle, which made diagonal grooves across
   the shade. Shade half `#1b2217` → `#2f3a2d` at the tops of its masses (cool, the sky),
   sun half `#2a3423` → `#4f5d35`, the crease `#141a11`.
4. **Lights 15 h later** (chunk 4): a thin half-light (filbert 4, `hug=false`, edge broken by
   noise) on the sun-turned sides of the lit masses, then **A's hooked leaf touches, but a
   chosen 7%**: each touch kept with a probability from the plan (crown side × mass turn ×
   no crease × its own light), so the lights come in groups that thin into the dark instead
   of an even carpet. A sparse few cool touches (`#3a4636`) on the shade masses' tops.
5. **Edges in spans** (chunk 5, after the sky dried): a slow noise along the contour and the
   holes picks *found and broken* stretches (blunt filbert 3.4–3.8 leaf dabs pushed 2–6 out,
   lit on the sun side, dark in shade), *soft* stretches in shade and *plain* ones (the lower
   edge against the bright horizon is always left found and clean). Blunt dabs, not flicks:
   a flick that ends in a hairline is B's whisker.
6. **The crown's underside opened** (chunk 6): the trunk runs on up into the crown's shadow
   and forks (the leader and the low left scaffold limb, from `tree.limbs`), darkening as it
   rises, with dark leaf clusters hanging in front. One limb per hole in only two holes, the
   stoutest, its stretch carried 12 units into the leaves either side (two limbs crossing in
   one hole made an X of floating sticks, so one only). 22 top lights. The common finish.

### What I see
- **At 1000 px** C is a lit oak with volume: big light masses on the upper left (leafy, as
  in A, not B's knobby rubber), and a shade half built of overlapping darker masses whose
  tops turn cool, instead of A's slab or B's one flat dark. The trunk goes into the crown
  and forks, so the tree stands on something. The sun side edge is lit leaves against the
  sky; the underside is a clean found edge.
- **At 3200** (`foliage_C_crop.jpg`): holes with mixed edges (a few leaves crossing some,
  clean elsewhere), no halos and no whiskers; a limb that comes out of the leaves into a
  hole. The upper left (not committed; any crop of `out/lab/foliage_C_full.png` at
  1920×384 shows it) has leaf touches in groups over a soft half-light: leafy, not confetti.
- **Against A and B:** I think C is the best of the three. It has A's leafy light and B's
  grouping, and it's the only one with a shade half that has a structure and a trunk that
  goes somewhere.

### C's ceiling
- **The masses are a repeated motif.** 22 k-means domes, each lit on its upper left and dark
  underneath, read a bit as cumulus or broccoli at arm's length. The per-mass contrast helps
  but doesn't break the rhythm. A painter would merge some, lose some and let one or two
  big ones dominate.
- **The shade half at 3200 is still mostly one dark** with vague greener masses in it: the
  structure reads at 1000 px, less up close.
- **No truly lost edge.** The "soft" spans (a gray-green halfway between the dark and the
  sky, dragged across the contour) came out as pale leaf specks, not a soft edge. The edge
  hierarchy is found-and-broken versus found-and-smooth.
- The relief finish embosses a fine grain into the dark and swirls in the sky (all three
  versions).

### Wet-paint misbehavior (C)
1. **Dark leaf dabs over tacky sky (30 h) lie as translucent gray ghosts**
   (`wet/C1_edge_leaves_over_tacky_sky.jpg`, 3200): filbert 3.4–3.8, `#20291b`, load 0.7.
   Over the same sky once dry they are clean dark leaves (`foliage_C_crop.jpg`). This
   agrees with B's finding 6 (a dark over setting sky is translucent gray).
2. **`b:touch` with `drag` over tacky sky** (`wet/C2_touch_drag_over_tacky_sky.jpg`) left
   faint gray scratches in the sky and only a few leaf-shaped marks.
3. **The same log dries on a different clock at 3200 than at 1000.** Chunk 5 waits
   `while drying(760, 445) ~= "dry"` in 12 h steps: the 1000 px replay ended at 41,194 min,
   the 3200 replay at 33,643. The painting still looks the same, but a `drying()`-driven
   wait isn't resolution-independent.
4. What worked: **sky set 15 h, then the dark meeting it edge to edge** (no lace, no halo),
   and lights into the dark 15 h after it was laid (partly still open at 700,450; no plowing).
   A trunk-top glaze (tried first) waited 14 days and laid a black band with a square top:
   dropped for body strokes that fade with height.

### SKETCHBOOK CANDIDATE (C over A and B; confirm on a second crown)
*A crown is a few overlapping leaf masses, each modeled by the sun and the sky; the light is
the tree's own touches, but only where the plan says, and the edge is decided in spans.*
```lua
-- plan: B's silhouette (lv:blur(2.5) + lv:blur(8):shrink(14) - drawn holes)
-- masses: k-means the clumps (K ~ 22, y weighted 1.25), radii 1.9/1.7 x spread (x0.8-1.35),
--   height 0.35*depth + 0.25*(y - mid)/250; on a 4-unit grid store the front mass's light
--   0.7*max(0, n.sun) + 0.3*sky(top) and a crease where it overhangs the one behind
-- BIG = (LB*MASS):blur(28) / MASS:blur(28)   (which side of the crown is lit)
-- 1. sky around MASS:soften(0.8) (no shrink); wait 15 h
-- 2. dark: filbert 6, length 8-18, coverage 2.8, medium 0.3, load 0.6, angle tangent to its mass,
--    color mix(shade #1b2217->#2f3a2d, lit #2a3423->#4f5d35, BIG) darkened to #141a11 by the crease
-- 3. +15 h: half-light filbert 4 hug=false on s*FORM > 0.42-0.7; then tree touches kept with
--    p = s * smoothstep(0.45, 0.85, form) * (1 - crease) * smoothstep(0.25, 0.6, t.lit), x1.4:
--    #46532e / #6d7b43 / #9aa35e; a few #3a4636 on shade tops
-- 4. sky DRY: edge spans by noise(period 55): blunt filbert 3.4-3.8 dabs, pressure {0.75, 0.55},
--    2-6 out; keep the underside against the horizon clean
-- 5. the trunk on into the crown's shadow, forking, fading with height; one limb in two holes
```
Pitfalls: never lay dark leaves over tacky sky (gray ghosts); a flick ending at pressure 0
is a whisker at 3200; two limbs in one hole make an X.
