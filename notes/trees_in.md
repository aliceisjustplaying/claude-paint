# Broadleaved trees grown into a drawn crown (branch `tool-oak`)

## Why

Trees were a defect the critics repeated in every round
(notes/review_scores_loop0_raw.md l.15, loop3_raw.md l.96, 115, loop4_raw.md
l.4, 50, 92):
- "an oak crown that is one cloud-shaped mass ... no limbs carrying separate
  leaf masses"
- "an Italian umbrella-pine silhouette made of stamped, lumpy foliage blobs"
- "a topiary of round green cloud-blobs on a bare skeleton of limbs, not an
  oak grown from its trunk"
- "the trees on the plain are identical stamped dots" / "lollipop dots"

`tree{habit="oak"}` grows from buds, and a painter can't choose its shape.
Then `t:foliage{}` puts clumps on the ends of limbs, which is where the
blob-on-skeleton look comes from. Firs were fixed by letting the painter
draw the silhouette (notes/firs.md). This does the same for broadleaves.

## What landed

- **`crates/paint/src/broadleaf.rs`** (new, geometry only):
  - `Tree::grow(crown, trunk, &Species, &Season, sun, seed)` uses space
    colonization (Runions, Lane and Prusinkiewicz 2007).
    - Attraction points fill the drawn crown in depth, crowding toward the
      shell. A low 3D noise thins them, so the crown has its own gaps where
      limb masses part.
    - The drawn trunk runs on into the crown as a crooked leader
      (`leader`). Only 2–7 nodes on it may sprout (`scaffold`), plus its
      top, so limbs leave the trunk; they don't sprout everywhere.
    - Each step heads toward its attraction points, bent by the species:
      `up`, `out`, `crook`, `inertia` and an oak's sympodial `kink` (a
      lasting turn that decays down the limb). No step turns more than
      about 55°, which prevents curls.
    - Widths follow the pipe model (exponent `pipe`), remapped so the
      trunk has the species' girth. The trunk tapers up from a flared
      foot. Limbs follow the thickest child through each fork.
    - Fine twigs grow at the tips on alternating sides, like a fishbone,
      and along the thin wood. Birch twigs hang; oak twigs zigzag.
    - Leaf clumps sit on the leafy wood (thinner than `leafy_w` twig
      widths). The season sets their share, their size, and whether leaves
      fall in patches and turn. Winter keeps a few dead leaves low in oak
      and beech.
    - Each clump is lit by where it faces on the crown and dimmed by the
      clumps between it and the sun (a grid across the sun's direction).
    - Each clump gets hooked touches for a pointed brush. They lie by
      species: oak any way with a hook, beech level, birch and willow
      hanging.
    - The tree grows in the crown's own quantized frame. So the same crown,
      trunk and seed grow the same wood anywhere and in every season, and
      a bare oak can stand beside the same oak in leaf.
  - Masks: `leaves_where(pick)` (by depth, light, turn or dead),
    `leaves`, `light` (0..1), `wood(lo, hi)` (by local width: the runs of
    each limb in that width range), `trunk`, `mask`, `crown_mask`.
  - `Group::grow` keeps the drawn crowns in front and adds `extra` trees
    made from them (flipped, stretched, reshaped, some narrow). The added
    trees stand `1 + u*recede` farther off, smaller, higher toward the
    horizon and hazier. `shadow()` gives one ellipse per tree, thrown
    away from the sun.
  - Species: `oak`, `beech`, `lime`, `birch`, `willow` (a pollard).
    Seasons: `spring`, `summer`, `autumn`, `late_autumn`, `winter`.
  - 3 tests: an oak fills its crown from its trunk (tips inside, sunward
    side lighter, trunk widest); winter is bare with the same wood as
    summer, and a moved crown grows the same tree; species differ (birch
    twigs hang) and a group recedes.
- **`crates/easel/src/draw_trees.rs`** (new):
  - `tree_in{crown=, trunk=, species=, season=, sun=, seed=, leaf=, turn=,
    ...species numbers}`.
  - `tree_group{crowns=, trunks=, species=, season=, foot=, count=,
    horizon=, recede=, air=, spread=, narrow=, sun=, seed=}`.
  - `sun=` takes `{x, y, z}` or a `world{}` (its sun turned into canvas
    axes).
  - Tree values: `t.limbs`, `t.clumps`, `t:leaves{depth=, lit=, turn=,
    dead=}`, `t:light()`, `t:lit()`, `t:shade()`, `t:gaps()`,
    `t:wood(lo, hi)`, `t:trunk()`, `t:touches{...}`, `t:paint(brush,
    {color=, lit=, depth=, share=, ...})` and `t:paint_wood(brush, {color=,
    min=, max=, ...})`.
  - Group values: `g:trees()` (far to near, each with `haze` and `scale`),
    `g:shadow()`, `g:haze(i)` and `g:mask()`.
  - 2 tests.
- **Small shared touches:** `lib.rs` `pub mod broadleaf`, `main.rs` one
  `mod` line, `api.rs` one install line. Nothing in form.rs, the paint
  tests or any existing painting.
- **README** (crates/easel/README.md, "Broadleaved trees grown into a
  drawn crown"): the API and a worked example painting an oak in leaf
  and the same oak bare, plus a field group.
- **Sketchbook** section 5, at its top: the recipe, marked as the tool
  to use.
- **Study:** `paintings/lua/trees_in.lua`, 8 chunks: sky, ground, the
  field group, the oak in leaf, the same oak bare, a beech and a birch.

`cargo test --workspace` passes. The golden fingerprint did not change:
`git diff 862fe6a -- crates/paint/tests` is empty. Clippy shows no warnings
in the new files.

## Evidence

- `notes/trees_in/trees_in_1000.jpg`: the whole study at 1000 px.
- `notes/trees_in/trees_in_3200_crop.jpg`: the 3200 render of the crop
  150,140–510,480 (the oak in leaf's shade side, the bare oak, three field
  trees). `notes/trees_in/trees_in_3200_detail.jpg` is a detail inside
  the leafy crown.
- Replay: `easel run paintings/lua/trees_in.lua` (about 45–60 s at
  1000 px), or `--width 3200 --crop 150,140,510,480` (about 135 s).

Judged against the critics' words:
- **"One cloud-shaped mass, no limbs carrying separate leaf masses":**
  mostly answered. The oak's crown is lobed. Sky comes through where limb
  masses part, and limbs show in the gaps, rising from a leaning trunk
  with one level limb. At 3200 the lit side is laid in single hooked leaf
  touches, not stipple. The shade side is still very dark at 1000 px and
  reads as one mass there. A skylit touch pass gives it some form at 3200.
- **"Topiary of blobs on a bare skeleton, not grown from its trunk":**
  answered in the geometry. Leaves sit on grown twigs and the limbs come
  from the trunk. The bare oak (same wood) shows it: a crooked sympodial
  crown with a level limb and fine fishbone twigs, clearly the same tree.
  It is the best passage in the study.
- **"Umbrella pine":** not reproduced. A drawn crown can't grow a flat
  parasol top unless someone draws one.
- **"Field trees identical round balls":** partly answered. The group
  has broad low oaks, a tall narrow lime and small far trees in haze,
  each with a trunk and one faint shadow. At 1000 px the smallest are
  still dome-on-stalk shapes. A drawn crown that is round grows a round
  tree.
- **Beech and birch:** they read as their species at 1000 px (a gray
  smooth stem and a white stem with black marks, the birch airy). But
  each has a pole problem: see below.

## Known issues and next steps

- **Hard edges on the gaps.** Sky holes are crisp, torn-paper shapes at
  3200 even with the touches laid after the sky. Next: a `gaps()` that
  is softened by the leaf noise, and a pass of rim touches along
  `t:gaps():rim()` that crosses into the holes.
- **Poles.** A beech's leader, and a birch's white stem, run straight up
  through the crown. Next: fork the leader when it reaches the crown for
  species that fork (beech), and end the birch's white bark where the
  stem thins (the study uses `wood(2.6)`; try 4).
- **Shade side too dark at 1000.** The recipe's colors are the fix. The
  tool could also offer a `t:skylight()` mask (clumps' upper faces on
  the shade side).
- **Winter twig tips.** They are better than the first starbursts, but
  the fine lace at the crown's edge is sparse. Next: a second generation
  of twigs, shorter and more numerous, at the outer shell.
- **Unused in a painting:** `lime` alone, `willow` (pollard), `spring`,
  `autumn` and `late_autumn`. They have unit coverage only for thinning
  and turn. An autumn color function would mix by `touch.turn`.
- **Cost.** A 286-tall oak in leaf grows 3000 limbs, 5000 clumps and
  30,000 touches in about 11 s of painting at 1000 px. `share=` thins
  the touches.
