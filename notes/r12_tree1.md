# r12_tree1: one tree in winter

Program: `paintings/src/bin/r12_tree1.rs`. Render: `out/r12_tree1_full.png`
(`cargo paint r12_tree1 -- --full --width 2400`).

## The study
An old open-grown oak, stag-headed (a dead, broken leader and dead limbs
standing in a live lower crown: retrenchment, trees.md §3.1 [ATF; HTC]),
alone on a snow field under a pale veiled winter sky, low horizon with a
thin band of distant woods. Friedrich's bare oak in snow recurs from 1807
to 1829 (trees.md §5.3), usually aged, storm-marked, with dead branches
[CDF-EICH]. Portrait canvas (aspect 0.78), 440 mm wide (Style default).

## Method (as he worked, from the notes)
1. Bought warm Dresden ground (Style::friedrich: red earth, ochre, brushed
   top).
2. Underdrawing: horizon against a ruler, both outlines of the trunk and
   limbs of order ≤ 2 with an HB pencil (hand_line), faint.
3. Sky: thin horizontal lay-in, badger, a wet stipple pass aimed at the
   sky's tones, then dry a finer lighter pass denser toward the horizon
   [NG p.56; CATS p.127]. Long flat veils of high cloud in the color
   field, a veiled low sun glow at the lower left.
4. Snow field: lead white body, horizontal strokes, cool grey in the
   hollows and on the far side of long drifts; the oak's soft shadow
   thrown right. Far woods: short upright hatching, blue-grey, in
   stretches with open country between.
5. The tree over the finished, dry sky [ALF p.346], trunk first, each limb
   one movement base to tip with a pointed round sable, the next smaller
   brush picking it up inside the wet end of the last; wide wood laid in
   bands side by side along the grain (the grain of oak bark), toned from
   the shadow side to the light. Then back into the wet: a lighter stroke
   down the lit side, a darker down the shadow side. Twig sprays at every
   live tip (crooked, clustered: oak buds crowd at the tip).
6. Dry; bark: fissure strokes along the wood, lean grey on the ridges.
7. Snow on the wood: ridges on near-level limbs thick enough to hold it,
   in heaps (oak bark is rough [MIL64 p.6]); little heaps in forks.
8. The foot: snow banked over the roots; dry grass flicked up over the
   finished snow [NG p.56]; a few of last year's brown leaves on low
   twigs (marcescence, [CDF-EICH]).
No varnish, no cracks: `Run::end` + relief only.

## Log
- v1 (seed 7, decline 0.32): the tree came out nearly all dead wood, 114
  limbs, no twigs: a "Y" of dead poles. Decline is a blunt knob (see
  friction). Horizon band ruler-straight.
- Probe of seeds × (decline, years): limb counts swing 7 → 983 for the
  same numbers (seed 8 with decline 0.15 gives a 7-limb tree). Chose seed
  12, decline 0.15, 30 years: 983 limbs, 50 dead.
- v2: reads as an old oak. At 2400px crops: roots are flat dark boards
  lying on the snow; the foot snow reads as separate pebbles (filbert
  dabs); snow on limbs beads into dots; bark lights are white scribbles;
  twig clusters are thistle-like radiating bursts. Crown too sparse for an
  old oak: the skeleton has long bare stretches.

- Contact sheets (a PROBE mode in the program writes `Skeleton::mask`
  silhouettes of 12 seeds as a PGM: my sketchbook, not a painting): every
  oak from `Habit::oak()` was a lanky young tree. My own habit (weak
  apical control 0.5, branch angle 1.15, flat 0.75, gentler tropism,
  34 years, trunk 0.065) gave broad old crowns. Seed 4 looked best on the
  sheet, but the sheet cropped each cell at the canvas width: its crown
  actually spans x −246..1001 and ran off both edges in v4. Printed
  bounds, took seed 1 (627 limbs, x 123..788): a crooked two-armed oak
  with a dead broken limb at the right and a burl with a snag at the fork.
- v6–v7: far woods lowered, lightened and broken into stretches (it was a
  hedge wall); snow on the wood as overlapping touches along the top edge
  of the limb over a thin cool underside, the noise only thickening and
  thinning the ridge (as an on/off threshold it made white dashes);
  bark: 72% dark fissures, the lights shorter and lean (they read as
  zebra stripes); roots truncated to 1.3× their width.
- v8–v10: crows on the two broken dead ends farthest out against the sky
  (first try put them against the dark trunk at the fork, invisible) and
  two far off at the left; the piece broken from the dead limb lying half
  buried before the tree; twig sprays also along limbs under 5 units
  (a veil through the crown, not only brooms at the tips); dead wood a
  weathered grey instead of dark with white dashes; the oak's shadow
  thrown right across the snow, soft.
- v11 (final): root buttresses that bulge and turn down into the ground,
  let dry, then the bank laid over them with an uneven edge (a tongue of
  snow up the windward buttress) and its shadow tucked under the trunk on
  the shade side. (The first bank dragged the wet root paint into the
  snow in dark smears: true to wet paint, so I let the roots dry first.)

## Working method, as it turned out
Whole render ≈ 133 s (ground prep ≈ 45 s, sky ≈ 65 s, everything else
≈ 20 s). With checkpoints, `--resume land` painted the tree stages in
≈ 6 s, so the tree was worked almost entirely by resuming; crops with
their own small checkpoints (`--crop 330,960,600,1180`) for the foot, at
1 s per iteration. Looked at the whole and at four standing crops
(crown top, right limb, fork/foot, foreground) after every change.

## Critique (honest)
What works: the oak reads as an old, storm-worn open-grown oak, crooked
with sympodial elbows, a dead grey broken limb, a burl, a live crown of
twigs in a fine haze; the twig network at 2400 px looks brushed (pointed
tips, lifts, kinks), not drawn. Snow sits where it would: on the upper
side of the level wood, in forks, banked on the windward side of the
foot. The sky is stippled and has the thin, grainy transparency of paint
pooling in the ground's texture. The composition is Friedrich's: a
single motif on the center line, low horizon, a thin band of distance,
much sky.

What doesn't:
- The crown is still too open for an old oak. His oaks have a dense
  "filigree" net; mine has long bare limbs ending in tufts. The skeleton
  gives ~600 limbs and my sprays add clusters, but a real crown edge is a
  continuous net of long shoots. It reads more like a pruned or
  wind-stripped tree than a full old oak.
- The big wood is a little uniform: a dark body with streaks, lit side
  only slightly lighter. The trunk has no real cylinder (sky light on the
  left, reflected snow light underneath), and the band strokes are too
  parallel.
- The snow field is bland at the whole view: smooth gradients with faint
  drifts. It is Friedrich-like in its emptiness but lacks the small
  particulars (tracks, tussock heads, the blue in the shadow of each
  drift) that would make it a place.
- Small motifs are clumsy at 2400 px: the crows are blobs with beaks,
  the fallen branch is a stiff grey ribbon, and the pale-wood touch on the
  burl's break reads as a white eye.
- The sky's upper half is a very even gradient: the veils I put in the
  color field barely survive the stippling.

## FRICTION
Most important first.

1. **Growth gives little control over the tree you want, and it
   varies wildly by seed.** Same habit, limb counts 7 → 983 by seed (probe
   in the log: seed 8 at decline 0.15 is a 7-limb pole, seed 12 a
   983-limb tree). `decline` is a blunt knob: 0.32 gave a nearly dead
   114-limb "Y" with no twigs (v1). There is no way to ask for "a live
   lower crown, a dead top of this size", nor for a crown spread: `grow`
   takes a height only, and a broad habit's crown ran 1250 units wide off
   a 1000-unit canvas. *Workaround:* my own sketchbook mode (PGM
   silhouettes of 12 seeds from `Skeleton::mask`), printing `bounds()`,
   picking seeds by eye.
2. **Checkpoint staleness counts every helper below `main` against every
   stage** (workflow.md: "plus everything after the top-level item that
   holds it"). All my tree, snow and crow helpers live below `main`, so
   every edit to them staled the sky and land checkpoints, which cost
   2 minutes to repaint. *Workaround:* `--stale-ok` after judging by hand
   that the sky/land code hadn't changed. That's easy to get wrong. A
   stage could hash only the helpers it calls, or a helper could carry the
   same `// ckpt: from <stage>` tag as setup lines.
3. **No per-point pressure along a gesture.** To make one stroke follow a
   limb's taper I resample the limb evenly by arc length, compute
   `tool.pressure_for(w)` at every point and pass those as `swell` knots
   with `pressure(1.0, 1.0)`. It works, but only because knots are
   "evenly spaced from start to end" (`bristle.rs` `swell_at`, a
   fraction of the stroke), and it interpolates between knots with a
   smoothstep, so the width is piecewise flat at every knot. A
   `Gesture::widths(Vec<f32>)` (or pressures per point) would say what a
   painter means. The same for tools: a mark `w` wide needs
   `w / Tool{..round_sable(1.0)}.mark_width(0.85)` (the linear scaling
   breaks at the hair-radius floor); there's no `Tool::for_mark(w, p)`.
4. **Checkpoints are 738 MB each at 2400 px (portrait 0.78) and `--ckpt`
   writes every stage**: 10 stages = 7 GB per run. I deleted them between
   rounds and leaned on crop checkpoints (small). A `--ckpt land,tree` to
   pick stages would do.
5. **Skeleton roots lie in the picture plane**: the buttress roots came
   out as flat dark boards lying on the snow field (v2), not diving into
   the ground. *Workaround:* truncate root limbs to 1.3× their width,
   paint my own buttresses that bulge and turn down, bury them in a snow
   bank.
6. **Sparse crowns.** Each modeled tip stands for a cluster of twigs
   (motifs.md), so the outer crown is thin; the fine net is the painter's
   to invent (fair) but there's no help: my `twigs` sprays are a
   hand-rolled recursive stroke tree.
7. **No "up" or normal along a `Limb`** (only `dir(i)`): snow on the upper
   side, lit side and shadow side all need the normal flipped toward up
   or toward the light by hand, per point. Small, but every painter of
   trees will write it.
8. **Every full render pays ≈ 45 s for the ground and ≈ 65 s for the
   sky.** A full-size look after changing anything before `land` costs
   over 2 minutes; crops are the only fast view (20 s cold).
9. `Handling::fill(false)` with the hatch preset still made the far woods
   a continuous band; the gaps had to come from the mask.
10. Wet pickup is right but global: laying snow against wet roots dragged
    dark into the snow; the only remedy is `c.dry()` (the whole canvas).
