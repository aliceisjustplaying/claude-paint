# l1_green: loop 1 rework (Friedrich, the wanderer over the Saxon plain)

Starting point: the round-4 green session, critic P4 score 19 (friedrich 4, paint 4,
drawing 3, light 4, detail 4). I worked in critique order and made four changes. All but
the first went in as new chunks before the varnish.

## What changed and why

1. **Field trees (critique "worst": stamped round shrub balls with identical drop
   shadows).** Rewrote chunk 17. The 14 identical balls, each with its own ellipse
   shadow, became 11 loose groups of 1 to 5 trees. Each crown is its own
   `outline{char="soft"}` with 7 jittered points. About 30% are narrow and tall
   (poplar-like, ry = 1.5 to 2.1 r) and the rest broad; size is `base*rand(0.55, 1.35)`.
   Broad crowns over r 3 get a short trunk. Each group gets one roughened shadow, laid
   thin (`hug=false`, coverage 1.6) with a shift that fades with aerial distance. Dark
   and lit colors are muted olive (`#2d3627` to `#3a3d2b`, lit `#58603e` to `#646a45`)
   and hazed by `w:aerial(Z)`. The global `FIELDTREES` holds the crowns so later passes
   can avoid them.
2. **Plain (critique "worst": dash-row fields, and "sugary" greens).** New chunk 23: a
   thin glaze-hand veil over the plain, clipped to it. Each stroke mixes what's under it
   40% toward its own gray, plus 12 to 18% of an ochre-gray earth (`#7b7a5a`). This takes
   the saturation out and fuses the dash rows a little without losing the fields. The
   river goes grayer under the same veil.
3. **Hill and sky (critique "next": saturated yellow-green, puffy cumulus).** New chunk
   24. `glaze(hill, {color="#6d6a4c", coats=0.32, pigment="semi"})`, noise-modulated
   from 0.75 to 1, with the figure, stone and oak cut out, turns the lit lawn to an olive
   earth green. `glaze(cloudmask:blur(8), {color="#aeb6bd", coats=0.4,
   pigment="semi"})` pushes the cumulus back into the sky.
4. **Figure and boulder (critique "next": a hooded black blob and a sliced loaf).** New
   chunk 25. The figure gets a hat brim, hair at the nape, a lit collar, a lit left flank
   (`fig:band(1.4, 0.5)`), two back folds and a staff. The boulder gets a transparent dark
   glaze on its lower plane (a wavy smoothstep 478 to 498), streaked weathering glazes,
   three more fractures and moss touches at its foot.

Not done (next session): the river is still a ribbon with lumpy edges, and it needs
banks and a darker near edge. Hedges are still hatch dashes, not drawn hedgerows. The
cumulus shapes are still puffy (only their value was quieted), and the boulder is still
loaf-shaped in silhouette. The hedge mass on the right is still one dark mass.

## SKETCHBOOK CANDIDATES (add only if the critic's scores improve)

- **Desaturating veil over dry paint (a new recipe).** Take chroma out of a passage
  without flattening it:
  ```lua
  work(M, {hand="glaze", tool="filbert 4", length={6, 16}, coverage=1.6, medium=0.75, load=0.35, clip=M,
    color_over=function(x, y, under)
      local s = 255 * (under.value ^ (1/2.2))
      return mix(mix(under, rgb(s, s, s), 0.4), "#7b7a5a", 0.12)
    end})
  ```
  Fixed: "saturated yellow-green, sugary palette" on the plain. For a big lit lawn,
  `glaze(M, {color="#6d6a4c", coats=0.32, pigment="semi"})` did the same job more
  simply.
- **Pitfall: a body veil with long strokes erases a whole passage.** The same
  `color_over` veil with `hand="body"`, filbert 5, lengths 18 to 50, load 0.5 and no
  `clip=` wiped out the fields, the river and the field trees. It also crossed into the
  figure, the stone and the oak's trunk, though they weren't in the mask (the strokes
  overshoot). For a veil, use `hand="glaze"`, short strokes, low load, `clip=` and a mask
  minus every motif, grown.
- **Field trees as groups (beats the "identical balls" ceiling in §4/§5).** Use 1 to 5
  trees per group, and give each crown its own soft `outline` from 7 jittered points.
  Make 30% of them tall and narrow, give broad ones over r 3 a short trunk, and lay one
  roughened shadow per group rather than one ellipse per tree.
- **Quieting bright cumulus after the fact:** `glaze((cl:mask{alpha={0.25, 0.9}} *
  above(HZ-30) - crown):blur(8), {color="#aeb6bd", coats=0.4, pigment="semi"})`. This
  changes their value, not their shape.
