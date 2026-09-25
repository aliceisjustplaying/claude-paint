# r12_tree3: Old Oak in Snow (a study)

Program: `paintings/src/bin/r12_tree3.rs`. Render:
`cargo paint r12_tree3 -- --full --width 2400` → `out/r12_tree3_full.png`
(2400 × 3053 px, a 440 × 560 mm canvas, about 0.18 mm per pixel).

## The picture

One ancient pedunculate oak standing alone on a low rise of snow under a
still, clouded late-afternoon winter sky. Its top has died back: the
leader and two upper limbs are grey dead wood, broken off short with
splinters (stag-headed, retrenching; trees.md §1, §3.1 [ATF; HTC]). The lower
crown is still alive and spreads wide, ending in dense sprays of young
shoots. Wet snow near 0 °C lies in heaps along the upper sides of the
near-level limbs ("heaps on rough bark" [MIL64 p.6]). A few curled brown
dead leaves hang on the low live shoots (marcescence [CDF-EICH]). At the
foot the bole swells into buttress roots that dive under a drift. Twigs the
oak shed (cladoptosis [RUST]) lie on the snow, and dry grass stalks stand up
through it, laid over the finished snow as fine upturned strokes [NG p.56].
The sky is heavy grey-violet above, clearing low down to a pale
yellowish band with the glow on the left. A far wood shows as a faint blue
strip on the right horizon. No varnish, no cracks: `Finish` with 0 coats and
`cracks: None`, only the raking-light relief.

## Working method

Stages, in Friedrich's order (materials §6; trees §5.5, where the trees
go "on the already painted sky" [ALF p.346]):
1. `drawing`: graphite on a light two-layer ground (warm ochre and chalk
   under a brushed lead-white layer, after the London *Winter Landscape*
   [NG p.55]). A 2H horizon is ruled; both contours of the bole and every
   limb wider than 3 units are drawn with HB. The twigs are left to the brush.
2. `sky`: a lean lay-in with level strokes, badger-blended, then stippled
   wet with the sky's own tones. A darker bank of cloud is stippled with a
   horizontal drag, then dry, finer and lighter stippling builds the low
   glow.
3. `far`: the distant wood, stippled.
4. `snow`: lay-in, then lead-white body strokes following the lie of the
   rise, clipped to the snow; the sky seam is stippled over.
5. `wood`: the bole and big limbs as several round-sable strokes laid side
   by side along the wood (`broad_limb`). Then every limb is pulled from
   where it springs to its tip, handed down to finer brushes as it thins
   (`limb`, pointed rounds, then a pointed rigger lifted off to a hairline).
   Then sprays of the youngest shoots at every live tip (`sprays`), short
   stubby shoots all along the limbs (`short_shoots`) and splinters at
   the breaks.
6. `bark`: once dry, broken lean strokes in lighter grey where the bark
   ridges catch the sky, more on the side toward the glow, and a broken rim
   down the lit edge. Then dark rigger fissures up the bole.
7. `snow on wood`, `leaves`, `foot` (buttress roots, drift, fallen twigs,
   grass).

The tree is grown by `paint::Habit` (oak, 34 years, `trunk 0.045`, `shed
0.06`, `node_buds 2`, `decline 0.22`, `decay 0.35`, seed 5; 1560 limbs). I
bend it with my own `gnarl` field (a short-period displacement, the same for
every point, so twigs stay attached). Everything painted is my own code in
the program. I didn't call `paintings::trees`.

Loop: one full render with `--ckpt` (about 135 s: the drawing 44 s, the
sky 66 s), then `--resume far --stale-ok` for tree work (about 27 s) and
`scripts/peek` crops. To choose the tree I wrote a diagnostic: with
`SKEL_ONLY=path` the program saves the grown skeleton's silhouette at 700 px
and exits. Environment variables (`OAK_SEED`, `OAK_TRUNK`, `OAK_SHED`...)
let me sweep 6 to 10 trees in seconds and paste them into a contact sheet.
That silhouette is not the painting. It is the botanist's sketch I choose
from.

## What I tried, in order

- v1: the stock oak habit, decline 0.32, seed 7. **Base placed wrong**: I
  set the tree's base y by hand, 90 units below the snow surface I'd
  computed. The `above` clip hid the lower bole, so the tree stood cut off
  in the snow. Fixed by deriving `base.y` from the `rise` function. The
  crown was also a thin skeleton of 10 visible limbs, and the knoll-shade and
  tree-shadow stipples read as purple slabs.
- v2: seed 4 at 34 years, sprays added. It was a candelabra: very thick
  limbs up to blunt dead tops, with the fine wood barely visible.
- Skeleton sweep (contact sheets): `trunk 0.06` gives limbs far too thick
  for the tip count. `trunk 0.045 + shed 0.06 + node_buds 2` gives a real
  open-grown oak with 1400 to 1700 limbs. I picked seed 5: broad, with a dead leader.
- v3/v4: dead wood darkened, brown halos cut by keeping the outer lanes
  inside the contour and making the bark colour cooler, snow clipped at the
  horizon, buttress roots and drift at the foot, shadow removed (overcast).
- v5: short shoots along the limbs, a broken rim, more grass.

## FRICTION (engine)

1. **Growth widths vs. tip count.** `Habit::trunk` is the trunk's width,
   and the pipe exponent is solved from the tip count. On a skeleton with
   few tips (seed 7: 88–400 limbs), a realistic `trunk 0.06` makes every
   limb stay fat right up to its end (20+ units at a dead top). The
   result looks like a candelabra or coral, not an oak. Nothing tells you
   this before painting. *Workaround:* lower `trunk` (0.045) and `shed`,
   and raise `node_buds`, to get more tips; sweep seeds with a silhouette
   dump.
2. **No way to see a skeleton cheaply.** There is no fast "show me the
   tree" path. Each look at a candidate tree means a full paint pass, or
   writing your own silhouette dump (`Skeleton::mask` → glaze → save on
   a throwaway 700 px canvas). The seed-to-seed variance is huge (42 to
   5121 limbs with the same habit), so choosing a tree *is* the first
   design act. *Workaround:* the `SKEL_ONLY` env hook plus a PIL contact
   sheet via `uv`.
3. **Handlings overshoot their mask by default.** The snow's `broad()` and
   `body()` strokes ran past the snow mask into the sky and left grey
   humps above the horizon. That silently ruins a horizon, the most
   important line in a Friedrich. *Workaround:* `.clip(true)`, then
   stipple the seam. The default feels backward for a mask named after the
   region.
4. **Thin dark paint at stroke edges reads as a warm brown halo.** A big
   round sable's ragged edge lays thin paint. Over the light sky, Kubelka–
   Munk makes a thin umber-grey go orange-brown, so every limb had a
   brownish fringe. Physically plausible, but it reads as a digital glow.
   *Workaround:* a cooler bark mix (#2a2826), outer lanes kept inside the
   contour, less shake, less ragged.
5. **Setup before the first stage makes every checkpoint stale.** The oak
   must be grown before the `drawing` stage (the pencil needs it), so any
   change to the tree's growth numbers makes the sky and snow stale (the
   sky alone is 66 s at 2400). *Workaround:* `--resume far --stale-ok`
   while iterating, with a note to myself that the pencil drawing under
   the sky is the old tree's until a clean render. A `// ckpt: from
   drawing` tag doesn't help, because the drawing is the first stage.
6. **Pencil is slow at 2400:** 44 s to draw about 150 contour lines.
7. **The model's tips are clumps.** Growth tips end in dense "broom"
   tufts (noted in motifs.md). I added sprays and short shoots by hand to
   put a network of fine shoots along the limbs, not only at their
   ends.
8. **Rng has no integer range.** It's a small thing, but every painter writes one
   (`ri` in my program).
10. **Editing a helper below `main` makes every stage stale.** Changing
    `limb_snow` or `short_shoots` (used only by late stages) refused
    `--resume snow`, because helpers after the item count for every stage.
    The painter's own motif functions are exactly what you edit most.
    *Workaround:* `--stale-ok` when I know the early stages don't call
    them. A per-function "used from stage X" tag would fix it.
11. **Dead wood dragged through wet dark goes muddy.** My broad lanes ran
    in live bark colour past `dead_from`; the grey dead top laid over wet
    dark came out streaked and translucent, with the pencil contours
    showing as outlines. This is correct wet-in-wet physics, and my own
    mistake. *Fix:* stop the lanes at `dead_from`. Worth a line in
    motifs.md: `dead_from` matters for any multi-pass limb painting.
12. **A glaze is a smooth function.** The final cool veil over the near
    snow reads a little like a digital gradient. The noise I multiplied in
    barely shows through 0.03-hiding paint. A painter would glaze with a
    brush, unevenly; `glaze` has no hand.
9. **Big round sables plough ladder-like pale lines** along long thick
   limbs under the relief light (known, motifs.md). I cut the long bark
   streaks to 1–4 nodes so the pale lines don't read as plank grain.

- v6–v7: dead wood made opaque, and lanes stop at `dead_from`. Short shoots
  are fewer and closer to the limb angle (they read as thorns), and the snow heaps are
  smaller and greyer (they read as white commas). A clean full render.
- v8: a `veil` stage glaze (cool over the near snow, faint in the sky's
  upper corners), resumed from `foot` (exact).

## Critique (honest)

What works: the tree's architecture. It is an open-grown oak that has
visibly lived a long time. A live, spreading lower crown ends in sprays
of fine shoots that lift off to hairlines, and the leader and upper
limbs are dead and silvered, broken off with splinters. At 2400 px the
twig filigree against the pale sky reads as painted with a pointed brush,
not rasterized. The dead grey against the live near-black gives the tree a
story. The foot has buttress roots going under a drift, which is better
than the flat-cut stump of v4. The sky's warm low band behind the bole puts
the darkest dark against the lightest light, where the eye should go.

What doesn't:
- **Too centered and too symmetric.** A candelabra silhouette in the middle
  of the canvas, with the horizon ruled level. Friedrich would place it
  deliberately and give the land some line. My rise barely reads.
- **The snow is empty and too even.** It has no drift lie and no tracks; the
  scattered grass tufts are too small and too evenly spread to carry it,
  and the final glaze reads as a gradient.
- **The twig tufts are brooms.** The growth model's tip clusters plus my
  sprays give dense round tufts at limb ends, with bare wood between.
  Real oak crowns are a more even net. The short shoots still have a
  whiff of thorns.
- **Snow on the limbs** now reads as snow, but it is too uniform in size and
  spacing: little white dashes on every level limb. Forks hold no
  heaps and no snow bridges between twigs.
- **The sky's cloud bank** is blotchy stipple in patches, and the lay-in
  leaves horizontal banding in the upper sky: painted, but not
  Friedrich's seamless stipple.
- The bole's lanes show as long parallel strokes. They read as bark but
  are a little too tidy, and the rim light is still thin.

In short, a believable old oak, painted with a hand, standing in a
placeholder landscape.
