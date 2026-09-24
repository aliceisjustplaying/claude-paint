# Lab 2: oakleaf (hierarchical detail at a middle distance)

A broad summer oak in the middle distance (the crown 370 of 1000 units wide, about a third of
the canvas), standing in a meadow, a low line of distant trees behind with three breaks, soft
daylight from the upper left. Canvas 1000 × 667 (3:2), 440 mm, Friedrich ground,
`friedrich_1820_greens`, seed 71. Shared setup: `oakleaf_setup.lua` (sky gradient, the far
tree line as a soft open `outline`, the meadow colors, the crown drawn lopsided and lobed,
`tree_in{species="oak"}`, the cast shadow). Both logs start with it byte for byte (chunk 1).

**Sheet:** `oakleaf_sheet.jpg` (whole pictures above, the same 3200 crop below).
**Key: P = H (hierarchical detail), Q = A (round 1's old recipe).**

| | A (old recipe) | H (hierarchical) |
|---|---|---|
| log | `oakleaf_A.lua` | `oakleaf_H.lua` |
| 1000 px | `oakleaf_A.jpg` | `oakleaf_H.jpg` |
| 3200 crop (canvas 270–590 × 150–390: the sun side, the gap, the lower edge) | `oakleaf_A_crop.jpg` | `oakleaf_H_crop.jpg` |
| renders (gitignored, as in `out/lab/`) | `out/lab2/oakleaf_A.png`, `_full.png` | `out/lab2/oakleaf_H.png`, `_full.png` |
| crown marks | a hatch pass over all leaves, sky into all `t:gaps()`, the full wood (4,731 limbs painted), **34,418 touches** | one lay-in and one modeled mass pass, **249 lit leaf groups + 88 cool ones (1,361 dabs)**, **171 edge dabs**, **1 limb** in **2 gaps**, **14 top lights** |
| far trees | hatched (round 1.4, 1.5–5 long), a lit layer | one soft mass: a thin lay-in with the sky, one body pass, a cross pass |
| meadow | tone, a sward of **5,996 tufts**, **1,400** long rigger blades across the near band | tone with big patches from a slow noise, the shadow worked in wet; **3 plants** in front (74 leaf strokes, 115 blades, 13 seed stalks, 6 flowers) |
| clock | 59,082 min (three `dry()`s) | 58,089 min (most of it waiting for the sky to dry before the edges, then the finish) |

## What A did
Round 1 A's recipe, as written: sky two layers (broad + blend, the next day a stipple),
the meadow as tone, `dry()`; the far trees hatched per sketchbook §4 "hedgerows and groves",
the shadow hatched, `dry()` again (a trunk laid over the open far paint churned it into a pale
band, so I added it); the wood (thick body, a lit flank, `paint_wood` thick to thin), `dry()`;
the leaves exactly as round 1 A (hatch body from `t:light()`, sky into `t:gaps()`, five
touch layers); then the §8 foreground: a sward over a noise patch field, tinted from under
each tuft, grass over the trunk's foot and the 1,400-blade near band. Adapted to this size:
the lit flank a step darker (at full light it read as a white bandage on a 14-unit trunk),
the fine wood split between two riggers (`rigger 0.9` above 0.5, `0.55` below).

## What H did (the hierarchy)
1. **Big shapes and value families** (chunk 2, t = 0). The silhouette from the grown
   leaves' own lobes, filled solid (`lv:blur(14)` core) so that only **two drawn gaps** are
   left. The crown split by hand into **five unequal masses** (the big sun mass top left, a
   top right mass turning away, three lower ones hanging in front, the center one rising
   high), each a dome over its own irregular outline (normal from its distance field) and
   each given its share of the sun (`MSIDE`, a painter's call over the measured light). The
   value is in two families: shade (recess 0.1, body 0.2, tops that see the sky 0.35, a
   little bounce on undersides) and sun (recess 0.3, body 0.5, turned to the sun 0.85),
   stepped (`steps()`, 0.12 apart) so they read as families, not a gradient. The recess
   above a mass that hangs in front is darkened but broken by a noise along its length.
   The sky alla prima around the reserved crown, trunk and far trees; then at once, edge to
   edge with it, a thin lay-in of the crown, trunk and far trees (bare reserved ground showed
   through later passes as orange flecks), and the crown's masses wet into the lay-in
   (filbert 6, strokes wrapping around their own mass). The meadow in big patches (a
   stretched slow noise, ±0.035 L), the shadow worked into it wet.
2. **The far trees as one soft mass** (chunk 3, +15 h): one body pass and one short cross
   pass over the lay-in, a restricted palette, close to the sky in value, bluer at the top,
   the three breaks left as they fall. The trunk.
3. **The middle scale, only where the light is** (chunk 4, +25 h, the masses tacky): each
   chosen clump of the grown tree becomes one leaf group, a crescent of 3–6 blunt filbert
   dabs on its sun side, a step lighter than the mass under it. Kept with a probability from
   the planned value, times a patch noise so the groups gather and leave quiet stretches:
   249 groups on the sun side, 88 sparse cool ones on the shade masses' tops.
4. **A few particulars** (chunk 5, once the sky was dry): the sun-side edge broken in spans
   by 171 dabs carrying the mass's own color out past the contour (the shade side and the
   underside left as found masses); one limb through the big gap; the leader lost into the
   crown's shadow; 14 top lights; the trunk's lit strip; a few blades at the foot.
5. **The meadow in front** (chunk 6): three unequal plants, each a dark clump of arching
   leaves, blades through it and stalks with seed heads, flowers only in the biggest.

## What I see
- **At 1000 px** H (P) is a lit tree with volume: the sun catches the upper left of the
  big masses, the lower tier hangs in front with lighter tops and dark undersides, the right
  side turns away in cool steps, and the light breaks into groups only where it falls. A (Q)
  is a convincing dark leafy oak, but its light is spread evenly in small flecks: it reads
  as texture more than as a lit volume, and its many small holes speckle the silhouette.
  H's far wood is one soft band with breaks; A's is a hatched hedge of the same value but
  busier. H's meadow is quiet with three particulars; A's near band is an even lawn of
  hairline blades.
- **At 3200** A is round 1's known failure again: confetti touches over the whole crown
  and dozens of torn-paper holes. H is quiet: masses you can read, leaf groups gathered on
  the lit tops, two gaps.
- **Which looks better to me:** H, clearly at 3200 and, I think, at 1000 too. A is leafier
  at arm's length; H has the light.

## H's ceiling
- **The silhouette is smooth-lobed**: a soft cumulus edge with few real breaks. The edge
  dabs help on the sun side, but the crown still reads a little like a cauliflower. It needs
  a few deeper notches and one or two lobes that stand clear of the mass.
- **The lit groups at 3200 read as flat pale patches** (laid over tacky paint they keep
  poster edges), a bit like lichen up close. At 25 h (paint only setting) they sank into
  the masses and vanished; I didn't find a stage in between.
- **The lower half is still mostly one dark.** Its undersides have a step (the bounce) and
  the lower tier has sky-lit tops, but under the tier line it's one family.
- **The meadow is plain.** Massed is right, but the middle distance of the field has no
  incident at all, and the three plants are small at 1000 px.
- **Setup:** the cast shadow is a flat roughened ellipse and the trunk's foot is a bulb
  (from `tree:wood(7)`'s root flare) in both versions.

## What went wrong along the way (for the next painter and the engineer)
1. **The varnish drew brown worm lines at 3200** around every dab and stroke edge on
   H's smooth crown (`varnish{coats=0.3, vary=0.1}`, round 1's finish). Varnish alone
   gave the worms; `relief()` alone didn't; `vary=0` and a `dry()` first didn't help;
   `coats=0.12` nearly clears them. Both versions use the 0.12 finish. In A the worms hide
   in the touches. (I first blamed crazing from painting over the setting lay-in and moved
   the masses into chunk 2. They stayed there, but that wasn't the cause.)
2. **Reserved areas are bare ground**: a mass painted later into a reservation (the far
   trees, the crown, the trunk) showed the orange ground in flecks through the weave, even
   at coverage 4.6 plus a cross pass. A thin lay-in with the sky fixed it.
3. **Dark dabs over tacky paint are gray ghosts** (round 1's pitfall, again): a shadow dab
   under each leaf group came out as gray-blue holes; dropped.
4. **Dark accents at this distance read as black dots**: 8–16 notches in the recesses,
   even a step off the local dark, read as polka dots against the half-lights; dropped.
   The recesses carry the darks.
5. **A band of taller grass across the meadow** read as a hedge stripe with specks (with
   lit up-strokes on top), then as flat hay bales (soft, broken); dropped.
6. **Edge dabs in a fixed dark on a lit top** read as black knobs at 3200: the edge dab
   takes the mass's own color at its root.

## SKETCHBOOK CANDIDATE (a tree in the middle distance)
*At this distance a crown is a few big masses, each lit as a volume, in two stepped value
families; the leaves show only as groups where the light falls, and there are almost no
particulars.*
```lua
-- silhouette: lv:blur(2) lobes + lv:blur(14):shrink(14) solid core - 2 drawn gaps
-- masses: 4-6 drawn by hand (unequal; the lower ones in front), poly(pts,true):roughen(6,22):grow(6)
--   normal: u = 1 - d/R from m:distance() (R = 1.1 x the depth at its center), n = (outward*u, sqrt(1-u^2))
--   side: a hand-set share of the sun per mass (0.08..1), 0.7 of it + 0.3 of the crown-scale light
--   value: shade 0.11 + 0.24*skyfacing - 0.08*recess + 0.05*bounce; sun 0.28 + 0.6*turn - 0.16*recess
--   stepped 0.12 apart; the recess broken by noise{period=40}
-- 1. t=0: sky around the reserved crown; at once a thin lay-in (medium 0.32), then the masses
--    wet into it: filbert 6, 8-18 long, coverage 3.6, load 0.8, strokes tangent to their mass
-- 2. +40 h (tacky): leaf groups = chosen clumps, p = smoothstep(0.4,0.72,value)*(1-recess)
--    *(0.55+0.45*lit) * 0.8 * patch noise(period 34); 3-6 filbert 3.2-4.4 dabs in a crescent
--    on the sun side, one step lighter. ~250 groups for a crown 370 wide. A few cool ones on shade tops.
-- 3. sky DRY: ~170 edge dabs in spans on the sun side only, the mass's own color; one limb in
--    one gap; ~14 top lights; no dark accents
-- far trees: one soft mass, laid in with the sky, one body pass + a cross pass, a restricted palette
-- meadow: tone in big noise patches; 2-3 particulars in front; nothing in between
-- finish: varnish coats 0.12 (0.3 pools into worms on smooth paint)
```
Pitfalls: no dark dabs over tacky paint; no dark accents at this distance; never leave a
reservation as bare ground.
