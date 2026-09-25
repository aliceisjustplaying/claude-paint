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
- 18:26 first whole render at 2400 with checkpoints.

## FRICTION

(see below; kept as I go)

## Critique

(after the render)
