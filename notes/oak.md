# Bare trees that hold together (branch `r6-oak`)

The owner called the bare oak from `tree_in{}` promising but "too
computationally fractal", with "twigs floating in the air". This stream
found why the twigs float, fixed it in the painting code, and made the
crown more economical: a few big limbs, fewer twigs with character, and
the rest of the fine twig mass indicated as a tone.

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
-- 1. the fine twig mass as a tone: a dry rigger dragged out along the twigs, thin and pale
local tm, cx, cy = bare:twig_mass(), bare.fork[1], bare.fork[2]
work(tm, {hand="body", tool="rigger 0.7", length={6, 14}, coverage=2, pressure={0.5, 0.2}, load=0.25,
  medium=0.35, threshold=0.05, hug=false, clip=tm:map(function(v) return 0.3 * v end), angle_jitter=0.3,
  angle=function(x, y) return math.atan(y - cy, x - cx) end,
  color=function(x, y) return mix("#554e45", sky(x, y), 0.55) end})
-- 2. the wood thick to thin; the bands are local widths, so nothing is left out
work(bare:wood(3.5), {hand="body", tool="round 2", coverage=3.5, angle=1.5, clip=bare:wood(3.5), color="#3b342c"})
bare:paint_wood(brush("round", 2.4), {color="#3b342c", min=1.2, max=3.5})
bare:paint_wood(brush("rigger", 0.55), {color="#554e45", max=1.2, pressure=0.04})
```

`detail=1` draws every twig (connected now too); `detail=0` draws only the
stout wood. A birch's tone hangs: `angle` about 1.45 instead of outward.

## Evidence (notes/oak/)

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

## The fir

I looked at the ragged `old` fir at 3200. Its boughs do run from the stem
into their pads. The "branches floating in the air" read comes from the
pads: discrete, leaf-like packets of hatch with sky between them and at
the stem (`bare_inner`), hung on a thin rigger line. Pressing the bough
strokes longer (image 5) barely changes it. Fixing it means changing how
the pads are shaped and painted (fir.rs, draw_firs.rs), which changes
l5_near's spruce. I left it for a stream that can re-judge that painting.

## Known issues and next

- At 1000 px the drawn twigs are sub-pixel and read as faint, broken
  lines. The tone hides this; a `detail` that falls with the tree's size
  on the canvas (fewer, stouter twigs on a small tree) would help.
- The tone's rigger strokes can read as a crosshatch at 3200 if pressed
  harder or with more jitter. At 0.4 of the mask it fogged the crown.
  Blotchy alternatives failed: a scumble made one gray blob and a fan
  brush made confetti.
- `twig_mass` is scaled by its own 98th percentile, so a tree with few
  twigs gets as strong a tone as a dense one. Scale by density per unit
  area if that matters.
- Leaders as poles (beech, birch) and the in-leaf torn-paper holes are
  still open (notes/trees_in.md).
- The fir's pads (above).
