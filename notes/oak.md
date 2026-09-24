# Bare trees that hold together (branch `r6-oak`)

The owner called the bare oak from `tree_in{}` promising but "too
computationally fractal", with "twigs floating in the air". This stream
found why the twigs float, fixed it in the painting code, and made the
crown more economical: a few big limbs and fewer twigs with character.
A second pass (below, "Pass 2") dropped the twig tone after a critic
panel rejected it, and made the oak's wood angular and tapering instead
of rope-like.

## Why the twigs floated (measured)

The geometry was sound. Every limb in `broadleaf.rs` starts at its
parent's node, and every twig starts on a point of its limb. The floating
came from how the wood was painted. I painted the trees_in bare oak on a
plain ground and followed each floating piece up its parent chain
(scratch scripts: `notes/oak/floating.py` counts dark pieces not joined to
the tree and unpainted gaps along each limb's path; it needs numpy, scipy
and pillow in a uv venv).

1. **Wood painted by no pass (the main cause).** The recipe painted
   `t:wood(3.5)` as body paint and then `t:paint_wood(..., {min=1.2,
   max=3.5})` and `{max=1.2}`. But `wood(lo)` selects by *local* width,
   while `paint_wood` selected limbs by their *base* width. A stout limb
   (base 3.5 or more) that tapers to a twig had its thin outer run painted
   by nothing. In the trees_in oak that was 9 limbs and 897 units of wood,
   including the top of the leader, and 188 limbs and twigs left from those
   unpainted runs. Example: limb 248 is 7.6 wide at its base. Its run from
   2.9 wide down to its tip read as bare ground in the render, so limb 260
   and all its twigs floated.
2. **Lift-off before the joints.** Strokes used `ramps={0.03, 0.5}`, and
   `Gesture::pressure_at` fades the pressure to 5% over the release share.
   So a limb was at its width for only the first half of its length, while
   its side twigs leave along the faded half and its tip twigs from the
   point where the brush had lifted. The starbursts at the crown's edge
   were clusters of twigs hanging on nothing.
3. **Joints laid thin.** A stroke that starts exactly at the fork lays
   little paint in its first step, which left a 1–2 px gap at 3200.
4. **Paint too thin to register (not fixed; a resolution limit).** At
   1000 px the finest twigs (0.12–0.3 units wide) are sub-pixel hairlines.
   Their darkness dips in stretches, so at a strict threshold they break
   into dashes.

Reloading the brush on every stroke or ending the pressure at the width
changed little (measured: 115 and 120 floating pieces against 112).
Cause 1 was the big one.

## What changed

- `crates/paint/src/broadleaf.rs` (geometry, additive; the grown tree is
  unchanged in every season):
  - `Limb.lead` marks the twig that leads on from a limb's tip.
    `Tree.twig_w` is the finest wood the model grows.
  - `Tree::drawn(detail)` says which limbs a painter draws. All the stout
    wood is drawn. Of the fine wood (twigs, and limbs under two twig
    widths), a share `detail` of its length is drawn, picked for character:
    long pieces, a limb's leading twig, and pieces toward the crown's edge
    where they show against the sky, with a little hash jitter. Each is
    always drawn with the wood it leaves from, and only one side twig per
    spot (no starbursts from one point).
  - `Tree::wood_strokes(lo, hi, detail)` plans the strokes for a band of
    *local* widths. A run starts one point back in thicker wood. A limb
    starts half a segment back along its parent (pulled out of it). A run
    that goes on into thinner wood overlaps it and doesn't lift. A drawn
    leading twig is painted as the end of its limb's stroke, when both are
    in the band. Parents come before children.
  - `Tree::twig_mass(frame, detail)` returns the fine wood as a soft
    mask: the undrawn twigs at full weight and the drawn ones at 0.35,
    blurred over half a twig's length and scaled so the densest 2% is 1.
- `crates/easel/src/draw_trees.rs`:
  - `t:paint_wood` now strokes `wood_strokes`. The pressure follows the
    wood's width through `swell` knots at even steps of the stroke. There
    is no attack, and a short lift (0.12) only at real tips. `pressure=`
    is the tip pressure.
  - `detail=` on `tree_in`, `tree_group` and `paint_wood`; `t.detail`;
    `t:twig_mass(detail)`. The default is `0.35 + 0.65 * leaf`: 0.35 bare,
    1 (everything, as before) in full leaf.
- Tests: `bare_wood_is_drawn_connected_at_any_detail` (drawn wood is
  closed under parents, the stout wood is always drawn, detail thins the
  fine wood, the three usual bands paint every point of every drawn limb,
  and every stroke starts on wood painted before it);
  `twig_mass_is_a_soft_tone_where_the_undrawn_twigs_are`; and in easel,
  `a_bare_oak_paints_without_floating_twigs`, which paints a bare oak at
  600 px on a plain ground and counts dark pieces off the tree. None may
  be bigger than a speck (6 px). The base code left 14 pieces over 3 px
  there, the largest 16.
- Study: `paintings/lua/bare_trees.lua` has a lime, the trees_in oak
  (same crown, trunk and seed), a beech and a birch, all bare.
- Docs: `crates/easel/README.md` ("A bare tree holds together", "Fewer,
  more deliberate twigs: `detail`", the example) and `notes/sketchbook.md`
  §5 (the bare-tree recipe, with its ceiling).

## The API

```lua
bare = tree_in{crown=C, trunk=T, species="oak", season="winter", sun=WORLD, seed=7}   -- detail 0.35
-- the wood thick to thin, each band with a brush that can lay its widths
work(bare:wood(3.5), {hand="body", tool="round 2", coverage=3.5, angle=1.5, clip=bare:wood(3.5), color="#3b342c"})
bare:paint_wood(brush("round", 2.4), {color="#3b342c", min=1.2, max=3.5})
bare:paint_wood(brush("rigger", 0.9), {color="#554e45", min=0.5, max=1.2, every=2})
bare:paint_wood(brush("rigger", 0.55), {color="#554e45", max=0.5, pressure=0.04, every=2})
```

`detail=1` draws every twig (connected now too); `detail=0` draws only the
stout wood. `angular=` (a species number) sets how angular the wood is.
`t:wood_strokes{min=, max=, detail=}` returns the planned strokes.
`t:twig_mass()` still exists, but a tone laid from it is not recommended
(Pass 2).

## Evidence, pass 1 (notes/oak/1_* to 5_*)

All the "before" images were rendered by the base commit's binary (built
from `git archive 2c5a658`) with the trees_in.lua recipe. All the "after"
images use this branch and `paintings/lua/bare_trees.lua`.

- `1_whole_before_above_after_below.jpg`: the whole study at 1000 px.
  Before, the limbs thin out and break into loose twig clusters, the lime
  and beech worst. After, every limb runs out to its twigs, and each crown
  has a soft edge of twig haze with a few drawn twigs.
- `2_oak_1000_before_lines_tone.jpg`: the oak at 1000 px, three ways:
  before | lines only (`detail` 0.35, no tone) | lines and tone. Lines
  only is clean and calligraphic. With the tone, it reads as a winter
  oak's crown. The owner should pick: the tone is one `work` pass in the
  recipe and easy to drop.
- `3_crown_edge_3200_before_lines_tone.jpg`: a 3200 crop of the crown edge
  (140 units square), the same three. Before: a starburst of look-alike
  twigs, with pieces floating. After: longer twigs with character, all
  connected. The tone reads as fine twig haze at a painter's distance.
- `4_lime_beech_birch_before_after.jpg`: before/after pairs at 1000 px.
  The habits differ as they should (lime dense and upright, beech rising,
  birch hanging). The beech's and birch's leaders still run up as poles
  (a known trees_in issue, not touched here).
- `5_fir_old_3200_bough_press_ab.jpg`: the old fir from `firs.lua` at
  3200, as it is (left), and with the boughs pressed longer (right,
  release 0.15, end 0.3). Almost no difference (see the fir, below).

Numbers (the trees_in bare oak on a plain ground, the same recipe calls;
dark pieces not joined to the tree; darkness over 0.15 of the ground-to-
wood contrast):

| | before | after |
|---|---|---|
| 1000 px, pieces > 3 px | 97 (1216 px, largest 199) | 0 |
| 3200 crop, pieces > 31 px (not cut by the crop) | 6 (664 px, largest 188) | 0 |
| 3200 crop, same at darkness > 0.3 | 11 (639 px) | 0 |
| 1000 px, same at darkness > 0.3 | 112 (1337 px) | 86 (1215 px) |

The last row is cause 4: stretches of sub-pixel hairlines that dip under
the stricter threshold. They are faint lines, not detached twigs, and the
tone covers them in the recipe.

**Benchmark logs:** `notes/loops/l5_near.lua` and `notes/loops/l3_green.lua`
render **byte-identical** at 1000 px before and after (`cmp` on the PNGs
from the base binary and this branch's). Neither uses `tree_in`: l3_green
uses `tree{}` (growth.rs) and l5_near uses `fir{}`/`fir_wood{}`, which this
stream did not touch. The golden scene is unchanged.

`paintings/lua/trees_in.lua` renders differently now: `paint_wood` paints
by local width. Its bare oak gets the default `detail` 0.35 (lines only,
since its recipe has no tone), and the in-leaf trees get the thin runs of
their stout limbs painted where they show through the gaps.

## Pass 2: no tone; angular, tapering wood

### The tone is out
A blind panel of four critics (two Gemini, two gpt-6-astra), judging a
lab study where the fine twigs were indicated as a tone, all preferred
drawn twigs, strongly ("steel wool", "fur", "gray scribble cushions").
The owner agreed: in `4_lime_beech_birch_before_after.jpg` the toned
trees look heavier and smeared. The recipe (the easel guide, sketchbook
§5, `bare_trees.lua`) is lines only now. `t:twig_mass()` stays in the
API, documented as not recommended, with this evidence.

### Why the limbs read as rope
The owner saw curly vines: long smooth S-curves of nearly even width,
with loops at the tips. I measured the strokes as the brush draws them
(`notes/oak/shape.py`, on `t:wood_strokes{}` dumps). I found four causes.
1. **The brush's spline.** `Canvas::drag` runs every stroke through
   `densify` (`crates/paint/src/path.rs`), a Catmull-Rom spline through
   the given points. Stroked through the limb's nodes, every elbow became
   a smooth curve. Only 43% of the turning was near a node (within 0.12
   of a model step).
2. **A random walk in the growth.** Each step turns by a random `crook`
   (0.45 for an oak), and a smoothing pass followed. So the limbs meander
   at every step (about 33° per step), with no straight runs. Path over
   chord was 1.45, and 89 strokes hooked back (net turning over three
   steps beyond 120°).
3. **Brushes that couldn't lay the widths.** A pointed brush lays from
   about two hairs to a little over its size: `rigger 0.55` lays 0.21 to
   0.71 (`b:mark_width`). `paint_wood` pressed it to at most its nominal
   0.55, and the recipe used it for all wood under 1.2. So everything from
   a twig to a small limb came out about one width. On a plain ground at
   3200, wood 0.3–0.6 and 0.6–1.2 wide both painted 0.70 wide.
4. **Long, wavy twigs.** An oak's twigs were 1.8 model steps long, and a
   side twig's zigzag started toward its limb, so some ran alongside it
   like a doubled line.

The pipe model does thin the wood at forks: a limb's width drops to a
median 0.71 of itself past a fork. The brushes hid it.

### What changed
- `Species::angular` (oak 1, lime 0.5, beech, birch and willow 0), with
  `tree_in{angular=}` to override it. For an angular species:
  - each run of nodes between forks is straightened with Douglas-Peucker
    (tolerance `angular` × half a model step); the kept nodes are the
    elbows;
  - a shoot keeps a heading (a running mean of its direction) and no step
    turns more than about 70° off it, so it can zigzag but not hook back
    (a side shoot starts a new heading);
  - `wood_strokes` subdivides each straight segment every 0.5 units, so the
    brush's spline follows the straight runs and keeps the corners;
  - a side twig stands at least 0.7 × `twig_spread` off its limb, and its
    zigzag starts away from it.
  The oak's `twig_len` is 1.1 (was 1.8): short, stiff twigs.
- `paint_wood` presses a brush up to the widest mark it lays
  (`mark_width(1)`), not its nominal size. The recipe splits the fine band
  in two: `rigger 0.9` for 0.5–1.2, and `rigger 0.55` below 0.5 with a lift
  to a point.
- Test `an_oak_is_angular_a_beech_smooth`. It checks that 70% of an oak's
  interior nodes barely turn, against 14% with `angular=0`; that the oak
  has fewer hooks; that its strokes lie on its segments; that a beech keeps
  its curves; and that twigs end thinner than they start.

This changes the grown wood of oaks and limes in every season: the in-leaf
oak of trees_in.lua grows a different crown of the same outline
(`p2_5_oak_in_leaf_before_after.jpg`), with elbows showing in the gaps.
Beeches, birches and willows grow as before; only their brush pressure
changed. The benchmark logs don't use `tree_in` and still render
**byte-identical** (`cmp` against the base binary, after this pass). The
golden scene is unchanged.

### Measured (the trees_in bare oak; pass-1 geometry reproduced with `angular=0, twig_len=1.8`)

| limb strokes | before | after |
|---|---|---|
| path length / chord | 1.45 | 1.25 |
| turning within 0.12 step of a node | 43% | 80% |
| hooks (net turn over 3 steps > 120°) | 89 of 474 | 69 of 538 |
| turning per model step | 0.57 rad | 0.54 rad |

About 40 of the remaining hooks are the turn where a limb leaves its
parent (the stroke starts half a segment back on the parent), which is a
real fork angle. Lime: node share 47% → 87%, path/chord 1.31 → 1.24. Beech
and birch: unchanged, by design (beech path/chord 1.21, its turning spread
along the curves, 45% at nodes).

Painted width on a plain ground at 3200 (median across the wood, binned by
the model's width; the twig bin is too thin to measure this way):

| model width | pass 1 | pass 2 |
|---|---|---|
| 0.3–0.6 | 0.70 | 1.00 |
| 0.6–1.2 | 0.70 | 1.30 |
| 1.2–2.4 | 1.95 | 2.15 |
| 2.4–3.5 | (none measured) | 2.90 |

### Evidence, pass 2 (before = pass 1 lines only, pass-1 binary)
- `p2_1_whole_before_above_after_below.jpg`: the study at 1000 px.
- `p2_2_oak_1000_before_after.jpg`: the oak at 1000. Before, it reads as
  rope, all smooth S-curves. After, it's a crooked oak: elbows, limbs
  thinning at each fork, short stiff twigs.
- `p2_3_crown_edge_3200_before_after.jpg`: the crown edge at 3200. After,
  straight runs change direction at nodes, twigs end in points, and no
  loops at the tips.
- `p2_4_lime_beech_birch_before_after.jpg`: the lime straighter, the beech
  still smooth and rising (its fine wood a little stronger), the birch
  with its hanging twigs as before.
- `p2_5_oak_in_leaf_before_after.jpg`: the in-leaf oak of trees_in.lua,
  which changes with the wood.

## Maintenance (round 6 review, S3)

No change to any tree: the five-species bare fixture (painted wood in three
bands plus a dump of every `wood_strokes` point) and both benchmark logs
render byte-identical.

- The painter's layer moved out of the growth model into
  `crates/paint/src/broadleaf/wood.rs`: `WoodStroke`, `subdivide`,
  `Tree::{is_fine, drawn, wood_strokes, twig_mass}` and their tests
  (`broadleaf.rs` 2000 → 1683 lines; `wood.rs` 351). It is a child module,
  so it reads the growth model's private helpers (`inside`, `edge_dist`)
  without widening them, and `paint::broadleaf::WoodStroke` still works.
  `straighten` stays in `broadleaf.rs`: it shapes the grown nodes (it runs
  inside `grow`), so it is growth, not the painter's selection.
- `angular` is a switch plus a size, and now says so. Four of its five uses
  are `> 0` (no turn past 70° off the heading; twigs 0.7 × spread off the
  limb; twig zigzag away from the limb; strokes subdivided along straight
  runs) and one scales (the straightening tolerance, `angular` × half a
  step). Making the four continuous would have moved lime (0.5) and costs
  an arbitrary blend for the zigzag sign, so I named the switch instead:
  `Species::is_angular()`, documented on the field. **Lime** keeps its
  output: the whole angular habit, with half the oak's straightening.
  `Tree.angular` is now a bool (the wood was grown angular), not a copied
  number. `Tree.twig_w` was never a copy: it is the species' `twig_w` at
  the tree's size (documented).
- One polyline helper: `paint::path::{dist, length, arclen}` (same float
  operations in the same order as each copy it replaces: broadleaf
  `polylen`, graphite `arclen`, rock `cumlen`, outline `dist`/`line_len`
  and two cumulative loops, and `plan_stroke`'s arc table in
  draw_trees.rs). `tally::path_len` is the same sum and is left for its
  owner to switch.
- `paint_wood`'s pressure profile goes through one named helper,
  `set_pressure_profile` (swell knots over a flat pressure of 1, which is
  exactly the profile). A direct `pressure=` knot list needs `b:stroke` in
  `api.rs` to take one; that's the place to fix it.

## The fir

I looked at the ragged `old` fir at 3200. Its boughs do run from the stem
into their pads. The "branches floating in the air" read comes from the
pads: discrete, leaf-like packets of hatch with sky between them and at
the stem (`bare_inner`), hung on a thin rigger line. Pressing the bough
strokes longer (image 5) barely changes it. Fixing it means changing how
the pads are shaped and painted (fir.rs, draw_firs.rs), which changes
l5_near's spruce. I left it for a stream that can re-judge that painting.

## Known issues and next

- At 1000 px the drawn twigs are sub-pixel and read as faint lines. A
  `detail` that falls with the tree's size on the canvas (fewer, stouter
  twigs on a small tree) would help.
- The crown is still a model's. Limbs cross each other more than a real
  crown's do, and a few sibling limbs run side by side like doubled lines.
  A space-colonization step that keeps siblings apart would help.
- Some elbows are sharper than an oak's (a turn kept at one node can reach
  about 90°). Rounding kept nodes over a hair's width would soften them
  without bringing back the rope.
- `twig_mass` is kept but not recommended.
- Leaders as poles (beech, birch) and the in-leaf torn-paper holes are
  still open (notes/trees_in.md).
- The fir's pads (above).
