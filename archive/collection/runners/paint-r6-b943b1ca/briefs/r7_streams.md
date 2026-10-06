# Round 7 streams: the four tasks

## edges (worktree ~/src/a/claude-paint-r7-edges, branch r7-edges)
The top complaint from Alice and both critics: **boundaries**. "The mountain
and its reflection resemble a filled selection", firs repeat one crisp
silhouette with pinholes, the figure is "an icon", grass is "wire", every
edge is equally crisp. Painters work through masks (`clip=`, `mask`,
shapes), and a stroke clipped hard to a mask makes a hard, even edge that
no brush makes. Real painters decide edges: found (crisp where form turns
against light), soft (blended), lost (the shape dissolves into its
neighbor), and vary them along one contour.
1. Measure first: how do clip masks, `hug`, cut-in and blur(`:blur`)
   shape the edges strokes make today? Profile edges across the mountain,
   firs and figure in evening_lake at 3200 (render its log from branch
   r7-paint-wet: build that branch's easel in a scratch worktree; it's the
   wet engine) and in a lab study on main.
2. Give painters real edge control, physically: e.g. a clip that lets
   bristles overrun a mask's edge by a varying amount (a brush doesn't stop
   at a line), an edge-quality field along a contour (found/soft/lost
   varying with light and a painter's choice), a "lose this edge" verb that
   drags the neighbor's paint across a contour with a dry brush. Choose
   what's most useful and physical; keep old behavior the default where
   existing logs depend on it.
3. A study: one silhouette against glow (a mountain ridge and its
   reflection, or a fir group) painted the old way and with the new edges;
   a lossless sheet for Alice (notes/edges/), a sketchbook entry with a
   ceiling, easel guide docs, notes/edges.md.

## piles (worktree ~/src/a/claude-paint-r7-piles, branch r7-piles)
Alice on the evening_lake sky: "the lighting is maybe a little too
perfect." The painter wrote its light as smooth math (evening_lake.lua
lines ~108–115: a 6-stop `gradient` plus a Gaussian glow), so every stroke
got a mathematically perfect color. A painter mixes a handful of piles on
the palette (with knife-mixing variation), reloads from them, and makes
transitions by blending on the canvas, so no two passages are the same
mixture and gradients are stepped then softened.
1. Read how color functions flow into strokes today (easel `color=`
   functions, `Aim`, `Palette::aim_for`, `palette.rs` piles, the hand
   ledger's `Piles` in tally.rs).
2. Build a way for painters to paint light the way painters mix: e.g. a
   `piles{...}` helper or a `color=` option that snaps a color field to N
   hand-mixed piles (chosen like a painter would: along the field's range,
   with mixing variation and the palette's real pigments), picks the pile
   per stroke or passage (not per pixel), and relies on `blend` and wet
   paint for the transitions. Deterministic. Old behavior stays default.
3. A study: the evening_lake sky and lake painted with its formula vs with
   piles (same composition); lossless sheet for Alice (notes/piles/),
   sketchbook entry (principle first: "mix piles, don't paint a formula"),
   easel guide docs, notes/piles.md.

## cracks (worktree ~/src/a/claude-paint-r7-cracks, branch r7-cracks)
Alice: "we do want the age stuff", but the craquelure is "still too neat,
still too digital". See notes/paint1/evening_lake_fresh_vs_aged.png and
evening_lake_aged_3200.png: a fine, even polygon network everywhere.
Engine: crates/paint/src/crack.rs, notes/cracks.md.
1. Read what's known (from text sources: conservation literature on
   craquelure: age cracks vs drying cracks, ground-dependent patterns, how
   Friedrich's thin grounds and paint crack, cupping, dirt, varnish
   cracks; cite; never look at pictures) and compare with the current
   algorithm.
2. Make it less neat, physically: varied island size by paint thickness
   and ground, dominant directions (canvas weave, stretcher bars and
   corners, roll damage), hierarchy (few long primary cracks, secondary
   ones meeting them at right angles), width and depth that vary along a
   crack, cracks that fade out, dirt and varnish discoloration in some and
   not others, cupping with light catching the island edges, passages
   that barely crack (thin glazes) next to ones that do (thick lead white).
3. Evidence: before/after on evening_lake (the aged log: add `cracks{}`
   before `relief()` in the last chunk) at 1:1 3200 crops of sky, water
   and dark foliage, lossless for Alice (notes/cracks_r7/), plus
   notes/cracks.md updated.

## friedrich (worktree ~/src/a/claude-paint-r7-friedrich, branch r7-friedrich)
Alice: "not very Caspar". Research only (no code): from written sources
(art history texts, museum and conservation publications; cite them; never
look at or describe pictures you view: text only), what makes a picture
Friedrich's beyond motifs: composition habits (symmetry, emptiness, the
horizon's placement, a foreground cut off from the distance, figure scale
and placement, the repoussoir, the vertical axis), his treatment of light
and sky (no visible sun, the glow at the horizon, bands), his palette and
layering, what he left out, how his skies and water relate, his mood and
how restraint produces it. Also the common mistakes of imitations. Update
notes/friedrich.md (keep what's there; add a clearly sourced section) and
write notes/briefs/friedrich_painter.md: a one-page guide a painter reads
before composing (principles, not recipes; no copying of known works).
