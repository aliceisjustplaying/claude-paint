# Pastel

Added on the `degas` branch (from `split-6-hold-knife`) for a painting in
Degas's manner of the mid-1880s: pastel over peinture à l'essence on toned
paper. The materials it models are in notes/degas-materials.md (a
materials-only research note). The painter's side is in the guide, under
"Drawing: pencil, chalk, pastel and eraser".

## What it is

Pastel is a new `graphite::Medium`, `Pastel`, on the drawing that pencils
and chalk already lay (`graphite::Drawing`): a dry deposit written into the
dry picture, so paint laid later composites over it and seals it.

- **Color.** A drawing cell's flake reflectance is now RGB (`Cell::r`,
  `Lead::flake`). Graphite and chalk set all three channels to their old gray
  and do the same arithmetic per channel, so their results are unchanged.
  `Lead::pastel(masstone, soft)` takes a pile's masstone and makes it dry
  (`dry_color`: paler and slightly grayer than in oil).
- **Layering.** Graphite only fills what is still bare, up to a cap. A
  pastel stroke covers a share `q` of the whole pixel, earlier pastel
  included, so colors laid across each other mix by coverage. A stick drags
  over each point along its contact, so one stroke deposits
  `1 - (1 - dep)^2`. The stick crumbles less than its grain suggests.
- **Tooth.** `Cell::fill` (0..1) is how full the tooth is. A stroke lays
  `dep * (1 - fill)^1.5` and adds `(0.05 + 0.06 soft) * q * (0.6 + 0.6 p)`
  to the fill. `fix` multiplies the fill by 0.4 (it gives back tooth),
  darkens the layer by 3% and still binds it against the eraser.
- **Edges.** The stick presses less toward the edge of its contact
  (`rim = 1 - 0.9 e²`, with `e` 0 on the line's middle and 1 at its edge),
  so a mark's edge rides on the tops of the tooth and breaks up.
  Graphite is unchanged.
- **Side.** `Lead::side(width_mm)` is the stick laid flat: that wide,
  biting 0.45 as deep, laying 0.85 as much, and not wearing.
- **Smudge.** `Canvas::smudge(mask, strength, radius)` drags loose pastel
  within `radius` units together: its colors average, weighted by the loose
  coverage. It also presses the pastel into the hollows (up to 0.97 coverage)
  and packs the tooth. Fixed, painted-over and wet pastel doesn't move.
- **Serialization.** A drawing without pastel keeps its old layout: five
  floats a cell, checkpoint flag 1. With pastel (`Drawing::color`) it is eight
  floats a cell (`r, g, b` and `fill`), flag 2.

## Lua

- `pastel(pile, {soft=0.7, point=})`: `point` (mm) makes a pastel pencil.
- `p:line`, `p:sketch`, `p:hatch`, `p:rule`, as for a pencil.
- `p:side(pts, {width=, pressure=})`.
- `smudge(mask or pts, {strength=, width=, reach=})`.

Also new, and for pencils as well: `hatch{graded=true}` (in the engine,
`graphite::hatch_marks_graded`). The mask becomes a weight: strokes run
wherever it is above 0.04, and each stroke is pressed at every point in
proportion to the mask's value there. A hatched passage then fades into the
next instead of stopping at a line. `hatch_marks` (ungraded) is unchanged.

## What the painting taught (tuning)

- The first numbers laid black as mid-gray: one deposit per pixel, with
  heavy crumbling. Raising the bite and rate, the two-pass drag and the
  higher cap fixed that.
- With strong bite, marks were smooth round-ended tubes. The edge falloff
  breaks them up.
- The darkest a matte pastel black reaches is about sRGB 0.27. That is the
  4% first-surface veil (`canvas::haze`) and the right physics. A black
  passage reads by its neighbours, not by being darker.
- The tooth filled too fast at first (0.10 + 0.14 soft per stroke, fixative
  keeping 45% of the fill). After a few dense passes nothing more took: eyes,
  nostril and lids drawn late didn't show. The current numbers allow several
  layers, and fixative restores more of the tooth.
- Pastel skips wet paint. Thick touches of blotted essence (bone black,
  vermilion) were still "setting" two painted days later and showed through
  as specks. Let essence dry about three weeks before pastel.
- A paper with tooth: a knifed board ground with a rolled absorbent layer on
  top (texture 1.0). A brushed layer, or a roller over bare linen, is too
  smooth or shows the weave's grid.
