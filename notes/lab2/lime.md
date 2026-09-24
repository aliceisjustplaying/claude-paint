# Lab 2: lime (hierarchical detail on a foliage crown)

One tall summer lime (linden) with a dense oval crown against a pale afternoon sky, the
whole tree in the frame with a meadow under it. Canvas 1000 × 1333 (3:4), a 320 mm panel,
Friedrich ground, `friedrich_1820_greens`, seed 71. Shared setup: `lime_setup.lua` (a tall
lobed egg drawn as an `outline{}`, `tree_in{species="lime"}` made denser: `voids=0.06,
shell=0.45, clump=0.03, leafiness=1.6, girth=0.04`; sun azimuth −120°, elevation 42°). Both
logs start with it byte for byte, and both lay the same meadow and foot shadow.

**Key (the sheet is blind): P = A (round 1's old recipe), Q = H (hierarchical detail).**
The letters were drawn at random.

| | A (old recipe) | H (hierarchical) |
|---|---|---|
| log | `lime_A.lua` | `lime_H.lua` |
| 1000 px | `lime_A.jpg` | `lime_H.jpg` |
| 3200 crop (canvas 500–820 × 250–490: the terminator, the upper shade masses, a hole, the right edge) | `lime_A_crop.jpg` | `lime_H_crop.jpg` |
| sheet | `lime_sheet.jpg` (P left, Q right, crops under) | |
| chunks | 6 (setup, sky, stipple, wood, leaves, finish) | 7 (setup + plan + sky, big lay-in, middle scale, edges, particulars, finish) |
| marks in the crown | wood fill + `paint_wood`, one hatch pass over every leaf, sky into all `t:gaps()`, **16,369** touches | one lay-in pass (filbert 7), **2,738** clump dabs, **1,403** leaf touches (371 mid, 350 lit, 610 high, 72 cool), **581** edge flicks + 11 free clusters, **133** lost-span dabs, 142 hanging leaves, 1 limb in a hole, **25** top lights |
| clock | 48,495 min | 35,899 min |

## What A did
round 1's `foliage_A.lua` adapted faithfully to this tree: sketchbook §2 sky (broad + blend,
next day a stipple clipped off the land), `dry()`, the `tree_in` wood recipe, `dry()`, then
the leaf body hatched from `t:light()`, sky into `t:gaps()` and the five touch layers
(shade, sky-shade, mid, lit, top every 6). Only the palette moved a step yellower for a lime.

## What H did
1. **Plan (chunk 2).** Three scales, each deciding less than the one above:
   - *Big:* the silhouette (`lv:blur(2.5)` + a filled core + the crown mask shrunk 40, so
     the tree's own gap by the leader doesn't show) less six drawn holes of unequal size.
     The crown as one egg lit from the upper left (`EGG`) decides the two **value families**.
     Eleven hand-placed lobes of unequal size, each with its own modeling amount, its own
     overhang crease and a hand-set share of sky on its top (`LSK`, 0.15–0.95). The
     terminator is pushed about by the lobes (`FAM = smoothstep(0.38, 0.56, EGG + 0.3(LOBE −
     0.5) − 0.1 CREASE)`). The lit value is `0.45 + 0.35 EGG + 0.18 LOBE − 0.25 CREASE`; the
     shade value is `0.07 + 0.34 SKY(1 − CREASE) + 0.08 LOBE − 0.06 CREASE`, so the shade is
     several masses, each deepest under the lobe above it and lifting to a cool top.
   - *Middle:* the tree's 2,331 clumps gathered into 70 groups by a **weighted** Voronoi
     (seed weights 0.55–1.7, so sizes vary about 3×). A group's outline is the union of its
     clumps (nearest clump on a 4-unit grid), its light a dome turned to the sun by its own
     amount (0.4–1).
   - The sky alla prima around the reserved silhouette (two broad layers, one blend).
2. **Big shapes (15 h later):** the whole crown laid in by families only, filbert 7, strokes
   wrapping each lobe. Shade `#141c12 → #3a4739` (the top of the range is cool), lit
   `#2c3822 → #6a7a42`.
3. **Middle scale (25 h later, the lay-in setting):** for each clump on a group's sun side,
   3–12 short filbert 4 dabs on the clump's own sun side, colored **a step above the lay-in
   under it** (`mix(BIGCOL, #8d9a55, 0.22–0.52)`). Then the tree's own leaf touches, kept with
   a probability from family × group light × the touch's light, and a few cool touches on
   the shade tops.
4. **Edges (sky dry):** spans by a slow noise: found-and-broken (leaf dabs pushed 2–6 out,
   colored by their family, a few free clusters just outside), lost (sky-colored dabs biting
   in, alternating with pale leaf dabs reaching out, at close value), and the underside
   against the horizon left plain.
5. **Particulars (dry):** the trunk carried up into the crown's shadow, taking the leaves'
   dark as it rises, with 142 dark leaves hanging in front; a limb tried in three holes (one
   was stout enough); 25 top lights on the most lit groups.

Tried and dropped in H (each would have gone into the log as a failure otherwise):
- a group **seam** term (the group id changing within 5 units) drew thin dark cracks: cracked
  mud;
- dark group undersides as near-black mask passes: camouflage mottle in the open lay-in;
  as dabs: polka dots;
- half-lights as mask passes: flat poster slabs with mask edges; a fixed half-light color:
  darker than the lit lobes, read as dots;
- lights at 15 h: the lay-in (medium 0.3) was still open here and they sank; at 25 h it was
  setting and they held;
- lost edges as a halfway tone straddling the contour: a frosty rim; as a thin sky glaze
  (`hand="glaze"`, medium 0.8): white frost patches;
- the trunk's top and two scaffold limbs drawn into the crown: a post with stick arms; three
  dark pockets with a lit branch in the lit side: black specks.

## What I see
- **A (P) at 1000 px** is a leafy, busy tree: little domes of lit leaves all over the left,
  a right half that is one near-black slab, and the holes (the tree's own gaps plus the
  sky dabbed into `t:gaps()`) look punched, with torn-paper edges and a trunk showing
  through the middle. **At 3200** the shade is a dark carpet flecked with pale sky specks
  (the gaps at touch scale) and the light is an even confetti of hooks.
- **H (Q) at 1000 px** reads as one lit volume: a clear sun side with leaf groups of
  different sizes, a terminator that steps with the lobes, and a shade half of five or six
  separate masses (the upper right, a cool middle-right shelf, the low left, the underside)
  with cool tops and deep seams. The trunk goes into the crown's shadow. **At 3200** the
  shade masses keep their values and edges between them; the lit groups are dabs over a
  smooth half-tone.
- **Which looks better:** H to my eye, clearly at arm's length (light and volume, a shade
  half that isn't a slab), narrowly up close. A still has more leaf at arm's length.

## H's ceiling
- **The lower crown stacks as horizontal shelves.** The lobes are ellipses and the lower
  ones sit side by side; the cool tops make bands. Lobes drawn as irregular outlines
  (or a lobe laid diagonally) would break it.
- **The lit side is soft:** at 3200 the group lights are round dabs over a smooth,
  airbrushed lay-in: mossy, not leafy. The lay-in could carry texture (shorter strokes,
  broken) or the touches could be denser on the group tops.
- **The shade contour is smooth, with dark beads on it** (the found flicks in shade read as
  dots at 3200), and the holes are still small clean ovals: only one of three limbs was
  stout enough to draw, and the edges of the holes aren't varied enough.
- The relief finish embosses a faint grain (both versions).

## SKETCHBOOK CANDIDATE (H over A on one crown; confirm on a second subject)
*A crown is painted at three scales, each deciding less than the one above: the families
(lit side, shade side as several masses with their own tops and seams), then irregular leaf
groups made of the tree's own clumps, lit a step above whatever is under them, then a
handful of particulars.*
```lua
-- BIG: EGG = crown ellipsoid lit by the sun; ~10 hand-placed unequal lobes (own amount 0.6-1,
--   own sky share 0.15-0.95, crease where one overhangs another)
-- FAM = smoothstep(0.38, 0.56, EGG + 0.3*(LOBE - 0.5) - 0.1*CREASE)     -- a stepped terminator
-- lit = 0.45 + 0.35*EGG + 0.18*LOBE - 0.25*CREASE;  shade = 0.07 + 0.34*SKY*(1 - CREASE) + 0.08*LOBE
-- 1. sky around the silhouette; +15 h the lay-in, filbert 7, coverage 2.8, medium 0.3, wrapped
--    around each lobe: shade #141c12 -> #3a4739 (cool top), lit #2c3822 -> #6a7a42
-- MIDDLE: ~70 groups by weighted Voronoi of the clumps (weights 0.55-1.7), dome light by own amount
-- 2. when the lay-in SETS (+25 h here): per clump on a group's sun side, 3-12 filbert 4 dabs
--    colored mix(lay-in color, #8d9a55, 0.22-0.52); then the tree's touches kept with
--    p = FAM * smoothstep(0.38, 0.72, group) * (1 - crease) * smoothstep(0.15, 0.55, t.lit)
-- 3. sky dry: edges in spans (found-broken flicks / sky dabs interlocked with pale leaves / plain
--    underside); particulars: the trunk into the crown's shadow, one limb per big hole, ~25 top lights
```
Pitfalls: never lay a light at a fixed color (it must be above what's under it); dark
passes into an open lay-in mottle; group seams drawn as lines read as cracked mud.
