# r12_tree2: one oak in winter

Program: `paintings/src/bin/r12_tree2.rs`. Render: `out/r12_tree2_full.png`
(2400 × 3000 px, `cargo paint r12_tree2 -- --full --width 2400`, about
2.5 minutes whole on the shared machine, about 100 s of that in the sky).

## The picture and why

An old pedunculate oak alone on a snowfield under a low winter sky, late
afternoon, portrait format (1000 × 1250 units). I chose it from what the
research says Friedrich gave most attention to: "no tree as much attention
as the oak", old oaks marked by storms with dead tops [CDF-EICH, trees.md
§5.1]; *Oak Tree in Snow* (1829) as "an old oak, crownless and probably dead
at the top… filigree twigs, and remains of a fallen trunk" [CDF-EICH §5.4].
My tree is my own, not that one:

- **Stag-headed, not dead.** `Habit::oak()` with `decline 0.18`, so some
  limbs are dead grey wood among the live crown (retrenchment, [ATF;
  HTC]); the biggest dead one reaches out low on the left, carrying snow.
- **Open-grown, crowded buds.** `node_buds: 2`, `tip_buds: (2, 2)` (oak's
  clustered buds, trees.md §3.1), `shed: 0.06` (little shade on a lone
  tree), `apical: 0.58` and `trunk: 0.07` for a clear bole: a broad,
  crooked crown on a short thick trunk. Seed 9.
- **A storm break.** The broken limb lies on the snow to the right, half
  sunk, pale splintered face toward the tree, snow along its top, a cool
  shadow under it.
- **Snow where physics puts it** [MIL64; MIL66]: ridges on the upper side
  of limbs that are near level and thick enough (`level > 0.45`, broken by
  noise where it slid off); none on steep or thin wood; driven snow caught
  in the rough bark on the windward (left) side of the bole, dry-brushed
  so it skips; a drift banked highest against the windward side of the
  foot, cool in the lee.
- **Marcescent leaves** on the low young twigs only (a few rust touches),
  none in the dead top [CDF-EICH; trees.md §3.1].
- **Grass last**, upturning flicks through the finished snow [NG p.56], in
  patches where the wind blew the snow thin, not sprinkled.
- **Sky** stippled over a thin lay-in [NG p.56; CATS p.127]: leaden
  grey-blue above to a thin cold yellow at the horizon. A far, low band of
  land at the horizon, kept faint so the tree stays the whole subject.
- **Order of work** as documented: primed ground (Style::friedrich),
  sky, land, the tree painted on the finished sky [ALF p.346], details
  last. No aged finish: `o.end` and the style's relief only.

## Working method

1. Grew candidate trees and looked at them as a contact sheet of
   silhouettes before painting (`--probe <seed>` writes
   `out/r12_tree2_probe.pgm`, 18 skeletons as masks, no paint). This is my
   stand-in for Friedrich's pencil tree studies. Three rounds: the stock
   oak habit (sparse, antler-like limbs; I first painted seed 12 of it),
   then denser habits (two axil buds, clustered tip buds, little
   shedding), then a clearer bole. Final: seed 9 of the last.
2. Stages: sky, sky stipple, far, snow, tree, twigs, bark, snow on the
   tree, foot, fallen limb, grass, leaves. The tree is grown *after* the
   snow stage block, so re-growing or re-painting it never stales the
   100 s sky.
3. Looked whole and in 2400px crops after every change (foot, crown, a
   limb).

What changed after looking:
- v1: one round sable dragged per limb, including the 50–60 unit bole.
  The bole broke into streaks (hairs running dry) and the roots, painted
  as limbs, hung under the snow line like hair. Roots dropped (snow
  covers them).
- v3: bole and big limbs laid in as a *form* instead: a mask from the
  skeleton's ribbons of everything wider than 6 units, worked with body
  strokes whose angle follows the nearest limb segment, lighter on the
  side toward the light, dead wood grey. Thinner wood dragged from where
  it leaves that form. Result: solid wood, but the stroke ends notched the
  silhouette into a cut-paper staircase.
- v6: the fill is clipped to an eroded mask, and each side of each big
  limb is cut with a sable stroke along the edge. Clean silhouette.
- The skeleton's crown was sparse (each modeled tip "stands for a small
  cluster of real twigs", notes/motifs.md), so I paint the cluster: 2–4
  crooked shoots with sympodial elbows from every live tip, side
  twiglets on them, and side shoots along the outer limbs. At 2400 these
  read as a fine net, the best part of the picture.
- Fork cushions of snow as round touches read as white polka dots on the
  bole; removed. Windward snow as short vertical flecks read as drips;
  replaced with a broken dry-brush plaster.
- Grass evenly scattered read as a digital sprinkle; changed to patches.
- v7–v8: switched to the denser habit. With ~6700 limbs the skeleton ends
  in its own bud clusters, so my added tip shoots made brooms; I cut them
  to one shoot on a quarter of the tips, and cut the marcescent leaves to
  a sprinkle on the low crown.
- v7: stratus banks stippled in the upper sky came out as hard grey
  smudges; toned down to a faint layering (contrast 0.1–0.2).
- v9: dead stretches of a limb were outlined in live dark bark (the edge
  stroke took the limb's first color); now each stretch is cut in its own
  paint. Broken ends and thick dead stubs get splinters and a pale torn
  face instead of the brush's round end.
- v10–v11: stippled modeling of the snow drifts. At coverage 2.2 the cool
  hollows became blue puddles; at 0.8 in a paler grey they're a breath,
  which is closer to right.

## FRICTION

1. **One brush can't paint a trunk.** Following a limb with a single
   `drag` works for wood up to a few units wide. At 50+ units (the bole)
   a round sable of that size streaks and runs dry partway, and
   `Tool::run` can't be set long enough to matter. Workaround: build my
   own mask from `Shape::ribbon` of the thick segments and `work` it with
   a handling whose angle comes from a nearest-segment search I wrote
   (`Segs::near`, linear over a few hundred segments; fine because angle
   and color closures are only called at stroke centers). A "direction
   field along a skeleton" helper would save every tree painter this.
2. **`work` notches silhouettes.** Unclipped body strokes overshoot the
   mask and their square ends make a stepped edge; `.clip(true)` on an
   `erode`d mask plus an edge-cutting sable pass on each side fixes it,
   but the clipped fill then leaves a faint pale rim between fill and
   edge stroke that reads like an outline. There is no "cut the
   silhouette" handling for a ribbon (`cut_in` exists for masks, but its
   edge tracing doesn't know a limb's direction).
3. **No way to change a Handling's tool.** `Style::body()`/`detail()`
   come with their tool fixed and `Handling` has no `.tool()` setter, so
   to use a smaller filbert I had to rebuild the whole chain from
   `Handling::new` and guess which of the preset's settings mattered
   (pressure, dips, curve, tail, broken, swell). Easy to lose part of the
   preset's character without noticing.
4. **Skeletons are sparse and there's no preview of them.** The grown
   oak's limbs are long with few side branches (an antler look), and
   tufts at the tips; I had to invent the twig net myself. Choosing a
   tree meant writing a PGM contact-sheet probe (the paintings crate has
   no image dependency, so I wrote the P5 header by hand and converted with
   `magick`). `Skeleton` also has many 0-width epicormic limbs on the
   bole (`w[0]` < 1) that are painted but invisible, and the limb list
   doesn't mark them.
5. **Staleness from items above `main`.** Adding a helper function
   (`habit()`, `probe()`) above `main` staled every checkpoint, including
   the 100 s sky, though the sky's code was unchanged. I used
   `--stale-ok --ckpt` each time. The model counts "every line up to the
   closing brace" of a stage, including unrelated items before `main`;
   helpers are only exempt when they sit *below* `main`.
6. **Thin shoots came out as hairlines.** A pointed rigger at `0.9` wide
   at pressure 0.8 with lean paint is a near-invisible hairline over a
   grey sky; the side branches only read after I gave them their own
   width (1.4–2.3) and fuller loads. Width and pressure interact through
   `mark_width` in ways I could only learn by looking; `pressure_for`
   helps once you know the width you want.
7. **Snow on limbs by hand.** Deciding where snow lies needs the upward
   normal, levelness and width per point; I computed it myself from
   `Limb::dir`. Straightforward, but it means every winter painter
   rewrites it. (Fine by the principles: it's the painter's decision.)
8. **Sky time.** The sky lay-in plus badger plus stipple is 100 s of the
   150 s whole render (3000 px tall); everything after it is 5–25 s. Crop
   renders didn't help for whole-picture judgment of a single tree.

9. **Stipple coverage is hard to judge in advance.** The same coverage
   number that gives a veil in a sky (over a mid tone) gave solid blue
   patches on white snow: the contrast of the touch against what's under
   it decides, and `aim(false)` masstone color at coverage > 1 hides.
   Two full re-renders to find the range.
10. **Checkpoints are large.** 547 MB per stage at 2400 × 3000; with 15
    stages `--ckpt` writes ~8 GB into `out/`. I kept them for resuming
    but a painter on a small disk can't.

## Critique (honest)

- The oak reads as an old open-grown oak in snow: short thick bole,
  crooked limbs, a dead grey limb reaching out low with snow along it,
  filigree at the crown edge against a grey sky, a drift banked on the
  windward side of the foot, the storm-broken limb lying in the snow.
  The twig net and snow ridges at full size are the best passages.
- Brown bud clusters at the ends of the limbs still read a little as
  tufts (brooms) at viewing distance; Friedrich's crowns end in finer,
  more even lace.
- The big limbs are smooth tubes, too even in width between forks,
  and the edge-cutting sable leaves a faint rim; the bark pass is
  timid away from the bole. They look drawn with a marker in places.
- The sky is a correct, quiet gradient with faint stratus, but the
  stipple is only visible up close; it's close to a digital gradient at
  viewing distance.
- The snowfield is quiet but nearly blank; the far band at the horizon
  is a hard dark line with bumps, more diagram than distance.
- The fallen limb is a single smooth tapered stroke, a stick rather than
  a broken oak limb.
- Composition: the tree sits a little left and low, with a lot of sky
  above; I kept it (the sky is part of the subject) but a Friedrich
  would place it with more intent.

### Earlier critique (first version, seed 12 of the stock habit)

- The tree reads as an old oak in snow at a distance, and the twig net
  and dead grey limbs against the grey sky are the most Friedrich-like
  passages. The fallen limb and the drift at the foot tell the story.
- The main limbs are still too long and too bare between the trunk and
  their tips: an antler or candelabra shape more than an oak's crooked,
  much-branched crown. The side shoots I added read as short spikes, not
  branches. A painter would have drawn more mid-sized branches.
- The big limbs are too smooth and even, tube-like after the silhouette
  fix; the bark pass is faint away from the bole.
- The sky is a correct gradient but too even: the stipple is only visible
  up close, and there's no cloud structure.
- The snowfield is quiet but a bit blank and flat; the drifts are too
  subtle, and the horizon line is weak (the far woods almost vanish).
- Digital tells: the edge-sable rim along the big limbs; very regular
  twig clusters at every tip (brooms).
