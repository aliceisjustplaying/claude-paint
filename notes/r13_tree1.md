# r13_tree1: one tree in winter

Program: `paintings/src/bin/r13_tree1.rs`. Render: `out/r13_tree1_full.png`
(2400 px wide, 2400 × 3000; `cargo paint r13_tree1 -- --full --width 2400`).

## The picture

An old pedunculate oak standing alone on a low swell of snow, the horizon
low (y = 1010 of 1250). It is stag-headed: the old leader and a second
limb beside it are dead, bleached gray and broken off, and they stand above
the living crown ("dead limbs standing above the living foliage", trees.md
§1, [ATF; HTC]). One low right limb is sawn off to a stub (Roloff's
"sawn-off" limbs in Friedrich's oaks, trees.md §5.5). A few epicormic shoots
sprout from the bole. The sky is a cold gray-blue, paler and warmer down to
the horizon, over a low band of far woods. The snow is lead white in body.
Dry grass stands through it in patches. A fallen limb from the dead crown
lies half-buried at the right.

## Working method

1. **Geometry first, as a drawing would come first.** The tree is a
   skeleton the program grows (`Tree::grow`): each limb is a polyline with a
   width at each point. The rules come from trees.md:
   - Leonardo's rule at every side limb: the mother keeps
     `w^1.9 − wc^1.9` (exponent inside the measured 1.8–2.3 range). A side
     limb is thickest where it leaves its parent, and widths only ever fall
     outward.
   - Acrotony and sympodial kinks: the mother turns away from each child
     it gives off, which is what makes oak crooked.
   - Heading: each limb holds to its own heading, which turns up slowly as
     it thins. The big boughs keep their spread, and the twigs rise.
   - Stag head: dead limbs don't thin into twigs. They end broken at 25–40%
     of their base width, keep only 40% of their side limbs (as stubs), and
     kink at every old node.
   - Twig tips sometimes end in a small cluster of short shoots (oak buds
     cluster at the tip).
   - Buttress roots go down into the snow. The bole has burrs (a width
     wobble), not a column.
   The result is about 5,400 limbs and 94,000 points. I checked the
   silhouette with a 1-px-per-unit PGM thumbnail (`TREE_PGM=path`). That is
   the painter's thumbnail sketch, not part of the painting.
2. **Drawing**: an HB contour of the trunk and boughs, and an H line for the
   axes of the thinner limbs and the snow line (`Canvas::draw`).
3. **Sky**: a thin broad lay-in with a gradient and long, low strata of
   darker cloud from fbm stretched across. Badger-blended, then stippled
   into the wet paint and let dry. Then a finer, lighter stipple that
   thickens toward the pale band at the horizon.
4. **Land**: far woods hatched in a pale blue-gray band that breaks off in
   places. Snow in body lead white, laid in long strokes with the ground,
   cooler toward the right. Then, wet-in-wet, long cool hollows between
   low waves of snow.
5. **Trunk**: every limb wider than 3.2 units is one mask (a spatial hash of
   tapered capsules, edges roughened). It is painted with `work` and a small
   filbert. The stroke angle is read from the nearest limb segment, so the
   strokes run along the limb. The color is lit on a cylinder from the upper
   left, with gray-green lichen on the lit side of the living wood, and gray
   for the dead limbs. Bark fissures come next: short dark (sometimes light)
   pointed-round strokes along the limb, broken into blocks, into the wet
   paint.
6. **Limbs** (0.95–3.2 units): each limb is painted from base to tip in
   strokes of about 45–65 units. Each stroke starts where the limb leaves its
   parent, pressed to the width there and lifted toward the width at its end
   (`pressure_for`), with a reload between strokes. The pointed round is
   picked from a set of brush sizes, the smallest that makes the mark.
7. **Twigs** (< 0.95 units): the same with a pointed rigger. The finest end
   in a lift-off to the point.
8. **Snow on the tree**: bands of snow on the upper side of limbs that are
   within ~55° of level and at least 1.6 units thick. A band runs 2–8 points,
   then a gap where the snow slid off. Some bands are cool gray. Snow is
   lodged in the great crotch.
9. **Foot**: a drift mound worked with contour strokes, cooler on the right;
   the trunk's shadow where it meets the drift; grass in patches, flicked
   up with a rigger, last (NG p.56); the fallen limb with its shadow, the
   snow on its top and snow drifted over it in two places.
10. **Finish**: no varnish, no cracks. The relief is lit only (`Finish`
    with `varnish_coats: 0`, `cracks: None`).

## Log

- v0: a Y-shaped, vase-like crown with the dead leader buried inside. The
  boughs sagged, because my "want" pulled thick limbs outward. Replaced it
  with a per-limb heading that turns up only slowly (scaled by 1/(1+0.5w)).
  Dead boughs made longer and slower to thin, so they stand above the crown.
- v1: the first paint. The tree reads. Faults: white dots in every fork
  (snow `touch`es, round and digital), arrowhead-like tip clusters on every
  twig, the base cut flat by the mask and a flat white slab of "drift", a
  dark cut-out band of woods, and grass spread evenly like a pattern.
- v2–v4: fork dots replaced by a lump of short strokes in the great crotch
  only. Tip clusters now on 45% of tips, shorter and narrower. Roots added
  as steep buttresses. A contour-stroked mound replaced the slab. Woods made
  paler and broken. Grass put in patches. Cool hollows added. Bark
  darkened and the bole given burrs.
- v5: the sky at full size read as flat gray with scanline stripes.
  The lay-in now wanders (`angle` varying, `cross(0.12)`, `drift`),
  pass 1 of the stipple went to coverage 3 and pass 2 covers the upper sky
  too. It reads as stippled paint now. Added last year's leaves on low
  twigs near the trunk (marcescence; the *Oak Tree in Snow* text mentions
  brown leaves kept through winter [CDF-EICH]). The first try put leaves on
  565 twigs, like confetti. Now they are on about 60 twigs in sheltered
  patches, hanging from their stalks, a darker brown.
- v6–v8: far woods hatched with a small round (1.3) instead of the hatch
  preset's 2.6 brush, so the edge is a serration of tree tops, not cotton
  lumps. Cool hollows lighter (they had read as dirty stains). Fallen limb
  made crooked, with a softer shadow and without its cotton-ball snow. Weed
  stalks with dark seed heads added among the grass. The drift now
  overlaps the foot in low, uneven lumps: a pointed round made loops and
  a filbert made snowballs, until the lumps were flattened.

## FRICTION

1. **A tree has no engine support: the painter builds its whole
   geometry.** There is `rock.rs`, `form`, `scene` and `hand`, but nothing for a
   branching structure. That is correct by the principles (motifs belong to
   the painter), but it means about 300 lines of skeleton, spatial hash,
   capsule distance and PGM thumbnail before the first mark. The
   costliest part to write was the *area* treatment of thick limbs: I
   needed a mask plus an angle field plus a lighting field, all from the
   same geometry, and `Mask::from_fn` + `angle(|x, y|)` + `color(|x, y|)`
   each call my `nearest()` separately (three queries per pixel or stroke).
   A way to hand `work` a per-point *frame* (direction, across coordinate,
   width) once would help any painter who paints along a form (limbs,
   rivers, roads, ropes).
2. **`Handling` can't take another brush.** `st.body()` fixes the filbert
   width at 9 units. There is no `.tool(t)` builder, so to work a trunk with
   a 5-unit filbert I rebuilt the whole handling from `Handling::new` and
   copied the preset's knobs by hand (pressure, dips, drift, tail, broken,
   swell, mix jitter). Workaround: the copy in the "trunk" stage. It is
   fragile, because it drifts from the preset.
3. **A held brush can't change size.** `Held.tool` is public, but the
   bristles are built for the old tool in `Held::new`, so assigning a new
   tool silently mismatches them. I create a new `Held` per limb from a set
   of sizes (`round_for`). That is fine as a painter's habit (a jar of
   brushes), but the public field invites the bug. Either make it private
   or rebuild the bristles on assignment.
4. **Checkpoint staleness vs. helper code below `main`.** All my tree code
   lives in helper items after `main`, and the staleness model counts those
   items for *every* stage. So any change to the tree generator stales the
   sky, the sky stipple and the land (about 2 min of painting at 2400 px).
   `--resume "sky light" --stale-ok` was the workaround every iteration.
   It is safe here because the sky doesn't read the tree (the drawing does,
   but it is faint), yet it is exactly the kind of override the model is
   meant to avoid. A way to say "this helper is read from stage X on" for
   items, not just lines, would fix it.
5. **Checkpoints at 2400 px are 893 MB each.** `--ckpt` on a whole render
   wrote about 7 GB for 8 stages on a disk with 100 GB free. I kept them
   because resuming from "sky light" is what made iteration possible
   (13 s instead of 130 s). Whole-render timings at 2400 px: underdrawing
   plus priming 45 s, sky lay-in, blend and stipple 72 s, everything
   else 18 s. The ~5,400 limbs' drags (limbs and twigs) take under 1 s,
   and the priming and the sky take 90% of the time.
6. **No round-dab primitive that isn't round.** `Touch` at a fork made a
   perfect disk of white, which reads as digital at once. Snow in a crotch
   has to be several short drags on top of each other. That is fine as a
   method, but the touch's footprint is too regular to use alone for
   anything natural.
7. **`Frame::per_column` + `Mask::from_fn` with captured masks.** Building a
   mask that reads another mask (`snow_m.sample`) moves it into a `move`
   closure. It worked here because `snow_m` wasn't needed later, but
   `Mask` isn't `Copy`, which forces an ordering on the program.
8. **No way to preview geometry.** Before painting, I wanted to see the
   skeleton. There is no cheap "draw these polylines flat" diagnostic in
   the engine (the principles rightly forbid flat fills in paintings), so
   I wrote a PGM writer and converted it with `sips`. A diagnostic
   rasterizer outside the painting path, like `study_sky --fields`, would
   save every motif painter this.
9. **Graphite pressure is hard to judge.** With `hand_line` profiles of
   0.25–0.6, the HB contour was nearly invisible on the reddish ground at
   2400 px, even before paint. There is no feedback short of rendering.

10. **Every motif ends up hand-tuned against its own failure mode.** The
   first try of nearly every small detail (fork snow, tip clusters, leaves,
   knuckle snow, far woods) came out as a regular, repeated shape: dots,
   arrowheads, confetti, scallops, lumps. The fix was always the same: fewer,
   irregular spacing, varied size, patchy by a noise field. The engine
   gives `uneven` spacing in `noise` for this, but the drags and touches a
   painter places by hand get no help, so each motif re-derives it.

## Critique

What works. At full size the tree reads as an oak and as old: a short,
burred bole, boughs that spread and twist, a living crown ending in a fine
net of twigs, and above it two bleached dead limbs, crooked at their old
nodes. The whole is painted from the snow to the finest twigs. The
twigs are the best passage: pointed-rigger strokes lifted off to the tip,
warm gray-brown against the cold sky, overlapping into a filigree that
looks painted, not rendered. The trunk's body color, run along the form,
with fissures drawn into the wet paint, reads as bark. The stippled sky
has the granular, strokeless look of his skies, and the far woods are a
believable serration.

What reads as digital or weak:
- The composition is safe: the tree is centered, the horizon straight and
  low, the sky empty. It is a Friedrich *arrangement*, but without his
  tension (no second element, no figure, no dolmen, no ruin, which the
  brief ruled out anyway).
- The crown silhouette is too even. The living crown is a fairly
  uniform haze of twigs of one density and one warm color. Friedrich's
  oaks have gaps, holes, clumps and some limbs bare to the tip. The generator's
  randomness is statistically even, so the crown has no "events".
- Snow on the limbs is thin and timid. Only a few bands show at viewing
  distance, so the tree barely says "snow has fallen". It is physically
  argued (steep limbs shed snow), but the picture wants more.
- The dead limbs' crookedness is sinuous rather than angular. A
  snake-like wobble instead of sharp breaks.
- The snow field is correct but inert. The cool hollows are very quiet
  now, the grass patches still look placed, and the fallen limb still reads
  a little like a stick laid on a tablecloth.
- The foot: the drift's top is still mostly one straight line across the
  root flare.
- The underdrawing is invisible. The HB lines were too light to show even
  under the thin sky.
