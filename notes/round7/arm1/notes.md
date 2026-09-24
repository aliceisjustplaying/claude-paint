# Arm 1: The Old Willow at Evening

Source: `paintings/lua/willows.lua` (32 chunks, painted live at the easel with
`EASEL_WITHOUT=procedural` and `canvas{style="friedrich", aspect=1.4, seed=23, hand=true}`).
Renders: `painting_1000.png`, `painting_3200.png` (plain `easel run` output).

## What it is
A flooded meadow in late autumn, after sunset. One old pollard willow stands on the
axis on a low dyke in the foreground, its bole dark against the water and its rods
against the glow. Behind it is still water mirroring the sky, and a thin far shore
on the horizon (y 452): a low wood on the left and a row of small pollards on the
right that fades into haze. The sky is in horizontal bands, from gray-blue above to
a lemon glow at the horizon, with two thin evening cloud streaks low down. No figure
and no moon.

## Order of work (clock in painting time; 14 sittings, about 48 days)
1. Geometry by hand in Lua: the bank line, the bole's outline, seven knobs on the head
   and about 75 rods grown from them by my own function (a direction and length per
   rod, a small bend, outer rods arching, some older rods forking). Pencil: a 2H
   searching pass, then a 3B pass for the bole, bank and alternate rods. Then fixative.
2. A brown lay-in in raw umber (`hand="glaze"`, medium 0.6, two passes at coverage 1.4)
   on the bank and bole.
3. Sky, first layer: five piles mixed from a restricted palette (no greens, no red
   earth), laid band by band in broad strokes that overlap at the joins, then a blend.
   Water, first layer: four piles mirroring the sky bands, a little darker, then a blend.
4. Rest overnight. Sky, second layer: stippled (width 3.2, coverage 1.7, medium 0.62) in
   nine bands over five sittings of up to 8 h, about 154,000 touches. Each touch's
   color came from what lay under it, lifted 0.012 in L and pulled 15% toward its
   band's pile (piles for the joins mixed from their neighbors).
5. Two cloud streaks: tapered ribbon masks (blurred), brushed thin with `color_over`
   and blended into the wet stipple.
6. Far shore: `hand="detail"` with a round brush under a soft `outline{}` line, then
   upward touches along the wood's top. Eleven small pollards by hand (a bole stroke,
   touches for the head and 14 to 22 rigger rods each). The shore's reflection is one
   transparent `glaze()` through a mask built from the wood's height.
7. Bank in body color from three piles (top lit by the sky, middle, darkest front),
   then a horizontal blend.
8. The bole: body strokes up the form, then about 40 wavering furrows with a
   semi-pointed round, the cleft, lit ridges and a few broken rim strokes of cool sky
   light on the upper left. The waist was later filled out (a second outline) to make
   the bole more post-like. More furrows went across a seam, and the foot was seated
   in a low mound of bank paint with grass over it.
9. The rods: every rod stroked with a pointed round sized to it, from three piles. The
   pressure holds, then falls off near the tip. About 70 young shoots were added,
   then about 80 more rods for a fuller crown (234 in all).
10. Grass by hand: tussocks along the crest against the water, four rush clumps and a
    patch-clustered scatter of blades on the bank face (about 2,900 blades, longer
    toward me). Later came a low fringe along the crest, three big clumps and 22 tall
    seed stalks with touched heads.
11. Faint level lines of light and darker water lines on the water with a rigger.
12. A transparent umber glaze deepening the foreground toward the bottom edge (uneven,
    by noise), and a small one under the willow's foot.
13. `wait(24*60); varnish{}; cracks{}; relief()`.

## Tools and techniques used
`pencil`, `fix`, `work` (glaze, broad, body, detail and blend hands; `color_over`;
`edge=`), `stipple` with `color_over`, `glaze()`, `brush`/`stroke`/`touch` for every
motif, `outline{}` for the far shore and the bole's second line, `ribbon`, masks,
`noise`, `uneven`, `sample`, `rest`/`sitting` (hand time on). No generators: the
willow, far pollards, grass and cloud streaks are my own Lua loops of strokes.

## Engine problems and what happened
- `hand="glaze"` at medium 0.8 and `hand="broad"` with a flat 14 at load 0.45 laid
  strokes with dark pooled rims ("pebbles" and outlined sausages). Coverage of 1.5 or
  more also adds round fill dabs. The workaround was medium 0.6 and two passes at
  coverage 1.4.
- `work` strokes overrun their mask by much more than a narrow motif. A far-shore mask
  6 units tall got a band about 30 units tall, and sky and water strokes crossed the
  bole's lay-in. Narrow passages needed explicit small tools and `edge=`.
- Stippling the sky costs about 61,000 touches and more than 8 h of hand time per
  band at width 3.2 with the default palette trips. A whole sky can't fit in one
  sitting (the maximum is 8 h), so it went band by band over five sittings, with
  `dips` raised to 40.
- The first stipple (full cover from a pile per band) gave nine flat stripes with pale
  seams where soft band masks overlapped. It was redone relative to the paint under it
  (`color_over`), with band masks that sum to one.
- A `while` loop of mine that never ended ran the sitting's clock to 8 h and was
  refused as a whole. Hand time made the bug visible.
- `sample()` inside a `work` color function read differently at 3200 than at 1000 near
  the bole's edge. A pale streak appeared only in the 3200 window. It was fixed by
  sampling toward the bole's middle (`easel edit 27`).
- Round touches over paint of the same dark color change almost nothing: 16 pixels
  for about 25 "bosses" on the head's contour. I left that chunk out.
