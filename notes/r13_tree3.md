# r13_tree3: one tree in winter

Program: `paintings/src/bin/r13_tree3.rs`. Render: `out/r13_tree3_full.png`
(2400 px wide, 2400 × 3000, `cargo paint r13_tree3 -- --full --width 2400`).

## The subject and why

An old pedunculate oak, alone in a snowfield under a low overcast winter
sky. Friedrich gave no tree as much attention as the oak; his oaks are old,
storm-marked, often winter, with dead branches showing their age
[trees.md §5.1, CDF-EICH], and from 1806/10 his old oaks often have dead
tops [ROL]. So: a **stag-headed** oak, its leader dead above the middle of
the crown and standing silver-gray above a live, round lower crown
[trees.md §1, §3.1, ATF, HTC], an old limb broken off the bole long ago (a
stub), water shoots (epicormic rods) on the bole and old limbs, a fallen
limb half-buried in the snow at its foot.

Viewpoint: low, as he drew trees sitting on the ground, so the horizon runs
across the lower trunk [BUSCH-V pp.77–78]. Portrait 4:5 (1000 × 1250
units), horizon at y 1012, the trunk's foot at y 1112.

## Working method

1. **The tree is designed, then grown.** First attempt: a purely
   recursive grower (trunk forks, side axes, Leonardo's rule d² = Σ dᵢ²,
   tropisms). It made a tall pole with a small crown, then a squat bush.
   What worked was the way Friedrich worked from a study: I set the
   *scaffold* by hand (the bole as seven points with its widths, flared at
   the foot; five scaffold limbs with start, angle, diameter and a target
   point, their squares summing to about the bole's, 58²), and let the
   grower make everything below that: side branches at nodes (Leonardo's
   rule on each split), zigzag bends that grow with thinness (crooked oak,
   sympodial), taper between forks (shed twigs), tropism outward from the
   crown's middle and up so the crown fills a dome, and thin wood past the
   dome stops growing. Dead wood gets no fine twigs and breaks off after a
   random length. I checked the skeleton alone first (`TREE_PREVIEW=out.pgm`
   writes it at 0.5 px/unit in a fraction of a second), looked at 8 seeds
   side by side and chose seed 104: a round live crown and a dead leader
   standing above it.
2. **Painting order** (Friedrich's): sky thin (broad lay-in, badger, two
   stipple passes, the second finer and lighter toward the light on the
   horizon), then the snow plain and a far hedge in stipple, then the tree
   over the finished sky [ALF p.346], thick to thin; then the snow lying on
   the limbs; last the grass through the snow [NG p.56].
3. **The tree is all hand gestures, no masks.** The bole and every limb
   thicker than 4 units are strokes along the wood (filbert or round, width
   ≈ a third of the limb), laid side by side across it from the shaded to
   the lit side, broken into strokes of 6–22 points; the outer strokes make
   the silhouette, so the edge has the hand's unsteadiness. Bark: short
   dark fissure lines with a pointed round, and lit ridges dragged lightly
   beside them (dry-brush on the tooth). Branches 1.2–4 units: a pointed
   round pressed to the width where the branch leaves its parent, the
   pressure following the branch's taper (`swell` knots from
   `Tool::pressure_for`). Twigs < 1.2: a pointed rigger lifted off to the
   bud.
4. **Snow on the tree** follows the snow-interception notes: a ridge on the
   upper side of near-level limbs thick enough to hold it, in broken bands
   (heaps on rough bark), heaps in the forks, nothing on steep or thin wood
   [MIL64 pp.5–7].

## Log

- 18:14 start. Read brief, research, engine notes.
- 18:16–18:25 tree grower: pole → bush → loops (limbs steering for their
  targets circled round them; fixed by dropping the target within 70
  units) → dead leader shooting off the canvas (dead axes now break off) →
  seed survey, chose 104.
- 18:22 first whole render at 2400 with checkpoints (2 min: sky 110 s,
  everything else 13 s). Read: tree good; trunk banded horizontally (all
  the strokes across it broke at the same points), snow on the limbs a
  string of white dashes, crown twigs sparse, snow field bland, a row of
  cotton-ball dabs at the foot.
- 18:26 staggered breaks and shuffled stroke order across each limb;
  knobby bole (fbm on the width); many more fissures, lit ridges, lichen;
  drift at the foot as a mask worked with body strokes (first try was a
  hard white box: the mask's sides were cut and its color was not the
  field's; second try takes its color *from* the field color and only
  lightens/cools it near the trunk); far stipple kept below the horizon.
- 18:28 outer strokes of each limb run its whole length (the edge had a
  row of rounded stroke ends); strokes converge and lighten in pressure
  as the wood thins; wood thinner than 7 units is a single pointed round;
  snow as longer lumpy ridges (random swell knots) on level wood.
- 18:29 twiglets clustered at each live tip (oak buds cluster), a few
  marcescent leaves low in the crown (first pass: far too many and orange,
  read as berries; cut to ~7% of low tips, dull brown, hanging).
- 18:30 sky: a third stipple pass over the upper third (rusty flecks of
  the red-brown ground broke through the thin lay-in); snow foreground:
  longer calmer strokes and a light badger (was mottled like camouflage);
  fallen limb with a cool hollow under it and a lit upper edge.
- 18:33 horizon: the far hedge broken into runs with gaps, a few far bare
  trees (rigger trunk and twigs, a stipple haze for the crown; first try
  read as pylons, crowns added); a drift crest across the foreground with
  a cool lee.
- 18:35 the bole now tapers into the crotch (it ended square where the
  limbs left it); fork snow dabs removed (round white polka dots).
- 18:38 side twiglets along all thin wood (13 000 hairline drags; the
  crown's edge net was too thin), keeping the chosen skeleton rather than
  regrowing it; a darker sky-light line on the upper side of level
  branches thicker than 3.5 (on thinner ones it made hollow tubes).
- 18:39 fallen limb crooked and tapering, a fork, a splintered butt, snow
  in broken bands (it was a flat plank).
- 18:40 clean whole run from scratch (kept as a fallback).
- 18:42 last round on the critique's three worst points: the sky gets an
  overcast deck (soft darker masses lying level in the upper sky, paler
  lanes between, thinning toward the horizon); each live limb its own
  tone, grayer-green or browner (a hash of where it starts); snow held on
  wood from 1.8 units and up to ~65° from level (wet snow near 0 °C
  sticks more [MIL64 p.5]). Clean whole run: 2 min 16 s. This is the
  final `out/r13_tree3_full.png`.

## FRICTION

1. **No way to paint a tapering limb as a stroke that tapers.** A filbert
   or blunt round has a mark width that barely changes with pressure
   (0.45 + 0.55p), so a limb painted as strokes along it ends in a blunt
   rounded cap where the next, thinner brush takes over. Workaround: the
   stroke centers converge with the local width (offset by the width left
   after the brush's own), pressure follows √(w/wmax) through `swell`
   knots, and wood thinner than 7 units is handed to a *pointed* round
   (`point: 1.0`), whose width does follow pressure. A "flat laid on
   edge" or a brush whose footprint can be set per point would make this
   one gesture.
2. **Parallel strokes across a form band into seams.** Nothing in
   `Canvas::drag` knows that ten strokes belong to one passage, so if I
   break them at the same points, the canvas shows a row of stroke ends
   across the trunk (horizontal banding), and the outer strokes' ends make
   a lobed silhouette. `Handling` solves this for masks (passages,
   staggered lattices), but there is no "handling along a path": a
   limb-shaped passage with its own direction field. Workaround: shuffle
   the stroke order, stagger breaks per stroke, run the two edge strokes
   the whole length. A `work_along(path, widths, handling)` would be the
   engine-level answer.
3. **Checkpoint staleness counts helper code above `main` for every
   stage.** The tree grower (`build_tree`, above `main`) is only used from
   the "limbs" stage on, but any edit to it made the 110 s "sky"
   checkpoint stale. Workaround: `--stale-ok --ckpt` after checking by
   eye that the sky code didn't change, and a clean whole run at the end.
   A `// ckpt: from limbs` tag on a whole fn item (or moving it below
   `main`) is the fix on the painter's side; I learned it too late.
4. **Sky time dominates.** Sky 110 s of a 123 s render; the tree with
   ~20 000 hand drags takes under 10 s. The loop for tree work is fast
   (resume from "snow": 5 s), but any sky edit costs two minutes. Fine,
   but it means the sky got the fewest iterations.
5. **A mask-bounded passage has a cut edge unless its color matches what
   is around it.** The drift at the foot as a mask came out as a white
   box: its sides were where the mask stopped. Workaround: soft mask and a
   color closure built from the neighboring field's color, only departing
   from it near the trunk. `color_over` would do this too, but I wanted a
   fixed target.
6. **No skeleton preview in the engine.** For a tree the drawing is the
   hard part; I wrote a 25-line PGM rasterizer of the axes to look at the
   skeleton (and 8 seeds) in a second without painting. An underdrawing
   preview (`graphite` onto a blank canvas at low res) would be the
   in-engine version.
7. **Palette mixes drift warm in tiny marks.** The first leaves and grass
   mixed from `#8a7050` came out orange-straw; small touches of a mixed
   earth read more saturated than the asked color over the pale snow. I
   darkened and grayed the targets by eye.

## Critique

What works. The tree reads as an old oak and not a generic tree: crooked
limbs that zigzag and thin as they go, a short massive bole that gives
itself up into five limbs, a round live lower crown with a dead,
silver-gray leader standing above it with its sawn-looking broken stubs,
and bud clusters at the twig tips. It stays whole from the snow to the
hairlines. The limbs look painted, not drawn: strokes along the wood, the
edge held by one long stroke, bark as short fissures and dry-dragged
ridges. The crown's fine net against the pale sky is the best passage.
Snow on the limbs sits where the snow research says it should, as ridges
on the level wood and nothing on steep or thin wood. The picture's
quiet, a single tree under a low overcast with the horizon across its
trunk, is the right mood.

What reads as digital, harshly:
- **The sky is only a little more than a gradient.** The last round gave
  it a level overcast deck, which helps the mood, but the bands are
  regular and ruled-looking at the whole-picture view (alternating
  darker and lighter stripes). The sky got the fewest iterations because
  each costs two minutes.
- **The wood is nearly one color.** A tone per limb now varies it a
  little, but there is no difference between near and far limbs, no
  moss-green on the north side worth seeing, and the lit side of the
  bole is only slightly lighter. Friedrich would have made the bole more particular: moss,
  a hollow, bark plates.
- **The branch drawing is too even in density.** The grower fills its
  dome uniformly; a real old oak has clumps, gaps and a few big
  characterful dead branches hanging across the crown. The dead top is
  the only strong accident.
- **The snow field is empty and smooth.** It is believable snow, but the
  foreground has no particular detail except the tufts and the fallen
  limb, and the drift crest barely registers.
- **Snow on the limbs** now reads as winter, but some of it lies on wood
  steeper than it would really hold, and it is all the same bright
  ridge; no hoarfrost on the twigs.
- **The foot of the drift** is a little lumpy (a row of round stroke
  ends along its top) and the stub on the bole looks like a sawn log
  end glued on.
- **Horizon** dotted and slightly mechanical at full size; acceptable at
  the whole-picture view.

If I had another hour: build the sky with the `atmos` fields (a real
overcast deck with a paler break low on the left where the sun is), vary
the wood color by limb and by lit side, clump the crown (a density field
on the grower), paint a hollow and moss on the bole, and more snow.

## Top five friction points

1. No stroke that tapers from thick to a point with a blunt brush; limbs
   end in caps unless handed to a pointed round (FRICTION 1).
2. No "handling along a path": parallel drags across a limb band into
   seams and lobed edges (FRICTION 2).
3. Checkpoint staleness counts helper code above `main` for every stage
   (FRICTION 3).
4. The sky dominates render time, so it gets the fewest looks
   (FRICTION 4).
5. A mask-bounded passage has a cut edge unless its color is made from
   its surroundings (FRICTION 5).
