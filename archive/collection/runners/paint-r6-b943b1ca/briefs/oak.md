# Stream: bare trees that hold together (worktree ~/src/a/claude-paint-r6-oak, branch r6-oak)

Read ~/tmp/paint-r6-b943b1ca/briefs/common.md first (rules, deliverable).

## The problem
The bare oak from `tree_in{}` (notes/trees_in.md, notes/showcase/7_tool_trees.jpg,
center tree) is the most promising structure tool: the owner called it
promising but "too computationally fractal" with "twigs floating in the
air". She said the same about the fir tool: "branches floating in the air".
Look at the image: the outer crown dissolves into fuzzy starbursts of
look-alike twigs, and at full size some twigs don't visibly connect to
anything.

## What to do
1. **Find the causes of floating.** Render the bare oak (and the fir) at
   1000px and 3200px crops of the crown edge. For each floating twig, find
   why: geometry (a twig not attached to its parent, a gap at the fork), a
   width falling below what the brush can lay (a rigger lifting off before
   the join), stroke start/end taper at the joint, drawing order, or paint
   too thin/light to register. Measure it: e.g. a test that paints a bare
   tree on a plain ground and counts dark connected components not
   connected to the trunk (target: none bigger than a speck), and gaps
   along each limb's painted path.
2. **Fix connectivity** in growth/broadleaf/fir and their painting code
   (crates/paint/src/growth.rs, broadleaf.rs, fir.rs,
   crates/easel/src/draw_trees.rs, draw_firs.rs): every painted twig leaves
   from painted wood.
3. **Fewer, more deliberate twigs.** Mark economy: a painter doesn't paint
   every twig. Make the hierarchy read (a few big limbs, fewer longer
   twigs with character, the finest twigs merged into a soft tone at the
   crown edge rather than a starburst of hairlines: think of how a painter
   indicates a winter crown's fine twig mass with a dry-brush or scumbled
   haze and a few selected twigs on top). Offer a knob for how much fine
   twig detail is drawn vs indicated as a tone, with a sensible default.
4. **Evidence**: before/after of the bare oak (whole and a 3200px crown-edge
   crop), a bare beech/birch/lime if the habits differ, and the fir if you
   touched it. Say whether notes/loops/l5_near.lua and
   notes/loops/l3_green.lua render identically (they may not use these
   tools). The owner decides on merging, so make the before/after easy to
   judge.
5. Keep the in-leaf trees unchanged unless a fix is shared and clearly
   better; don't take on the leafy "torn-paper holes" problem here.
