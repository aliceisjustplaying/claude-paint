# The Chapel Gable at Evening (python_friedrich control)

A control experiment: one plain Python program (numpy, Pillow only to write the
PNG) that paints one new picture in Friedrich's manner, look-first, with no
paint physics, no reference images and nothing from the Rust engine.

    uv run paint.py --width 1000 --out out_1000.png
    uv run paint.py --width 3200 --out out_3200.png   # about 40–60 s

Output is deterministic (fixed seeds; two 1000px runs are byte-identical).
Geometry, stroke placement and noise all live in width units, so the
1000px and 3200px renders are the same picture at two resolutions.

## What I painted

A ruined chapel gable stands on a bare ledge above a calm sea, dead on the
central axis. It has one tall pointed arch and a round oculus above it. A man in
a dark blue-green coat and a beret stands still inside the arch with his back to
us. His head sits on the ruled horizon, against the brightest band of light.
The sun has set. The light is one band of pale lemon where the dark land meets
the sky, and above it the sky runs through rose and gray-violet to dusk blue in
horizontal bands. There is one long low cloud stratum, a few thin parallel strata
and a pale wedge that cuts down from the upper left. A young crescent moon sits
small and veiled on the right. The sea repeats the sky's light in paler lanes.
There's no middle ground: ledge, then water, then sky.

Choices taken from `notes/briefs/friedrich_painter.md`: an axis first (gable
apex, oculus, arch and figure all on x = 0.5), symmetry in the whole with
irregularity in the parts (the right shoulder is broken and the stepped
remnants differ left and right), a sky share of 58% above a ruled horizon, one
figure standing still on the axis with his head over the horizon, the light as a
band with no sun disk, a veiled moon, calm water, one key (dusk violet and rust)
with one small accent (the coat), and a bare foreground. The kinds of things are
ledge, ruin, figure, sea and sky: five. The ruin is "the one object of
contemplation". I left out a self-seeded birch on the broken shoulder, gulls and
stones on the ledge.

## How the program works (paint.py, 990 lines)

1. **Design.** Analytic, resolution-independent functions give the look of each
   zone: sky bands and clouds (value noise and fbm warped horizontally), sea,
   land, the wall (masonry built from course lines, voussoirs round the arch and
   a ring of stones round the oculus, lost joints, damp and lichen patches) and
   the figure (a polygon in meters plus ellipses). These are rasterized once to a
   full-resolution "design" image plus zone masks. A second copy without the
   wall and figure is used by the sky and sea strokes, so they don't drag dark
   stone into the air.
2. **Ground.** Warm red-ocher lower priming under a patchy lead-white top layer.
   A texture height map holds the top layer's horizontal brush striations
   plus a fine linen weave, which fades out when threads would drop below
   about 1.6 px.
3. **Underdrawing.** Graphite lines: the horizon ruled twice, the axis and
   golden-section verticals, and the gable, arch, oculus and ledge outlines. They
   stay faintly visible under thin paint.
4. **Brush strokes** (`Canvas.stroke`). Each stroke is a slightly bent path
   rasterized in its own bounding box. Its width wobbles along its length, it
   has round caps and a tapered option, and it carries a row of 5–90 bristles.
   Each bristle has its own strength, its own dry-out point (dry-brush breaks up
   toward the end of the stroke) and its own slight tone. Dry paint catches only
   on the peaks of the ground texture. Thin paint pools in the valleys, which
   gives Friedrich's dotted, strokeless gradations. Color is the stroke's own
   color (sampled at its center, with luminance jitter) mixed with the design
   under each pixel, so it partly follows the design and partly stays flat. Every
   stroke also adds to a paint height map.
5. **Layers, in his order.** Dead color in umber-tinted strokes, a thin even sky
   and sea layer, then sky strokes in three sizes. Next come stippling (hundreds
   of thousands of tiny dabs, splatted and softened with a hand-written
   cumulative-sum box blur, settling in the ground's valleys), cloud strokes
   along the strata and the wedge, the moon, the sea (strokes, then pale glints,
   then the horizon softened by mist only at the sides) and the land. The wall
   gets a flat dead color, horizontal lay-in strokes along the courses, dabbed
   stone faces, dark rain streaks, a warm reveal band inside the arch that
   catches the glow and a faint lit gable edge. The figure gets a flat silhouette,
   long coat-fold strokes, hair, and a warm rim on the shoulders and head. Grass
   is flicked up last along the crest and on the broken steps.
6. **Finishing.** A dark glaze toward the edges (Carus's advice for twilight
   pictures), a warm, slightly milky varnish, relief shading from the height
   map, and craquelure. The cracks come from a jittered Voronoi network at two
   scales (about 6 mm and 2.5 mm cells if the canvas is 1 m wide), with uneven
   strength, lost stretches and clustering.

No drawing library, image filter or scipy is used. Every blur, noise, mask and
stroke is numpy code in the file.

## What works

- At full view it reads as one picture with one light and one order: the axis
  holds, the head lands on the horizon, and the arch frames the brightest band.
- The sky gradation from lemon through rose to dusk blue is smooth without
  being a clean gradient: thin layer, strokes and stipple over a warm ground.
- The sea at 3200px is the best passage: long broken lanes of paler paint,
  glints and a horizon that is crisp on the axis and softer at the sides.
- The veiled crescent, with thin cloud strokes dragged across it, is small and
  unemphatic.
- The wall reads as weathered coursed stone at 1000px, with joints that come and
  go instead of a brick grid.

## What doesn't

- **Edges are the most "digital" thing left.** The gable and figure silhouettes
  are mask-clipped. They're exact (which Friedrich's are against the light),
  but they're exact the way a vector shape is exact, not the way a pointed sable
  is. A painter's cut-in edge overlaps and wanders by a hair. I added only a
  few pixels of wobble.
- **The upper sky is streaky.** Horizontal dry-brush hairlines cover it evenly.
  At 3200px this reads as brushwork, but it's more uniform than his stippled
  skies, and it borders on "uniform noise".
- **The foreground is dull.** It's bare, as it should be, but its combed
  strokes have no form. A person would model the ledge's lip and a few hollows.
- **The figure is a cutout.** The coat folds are just vertical strokes. The beret
  reads as a flat cap. There's no sense of weight or of the head's slight bow
  beyond the outline.
- **The masonry is baked into the design.** The strokes follow it rather
  than build it, which is the most "rendered" part of the method: the joints
  exist before the brush does.
- The motif (a figure framed in a ruined window) sits close to his ruin
  pictures, though it isn't a copy of any one. The oculus-and-arch gable is
  maybe too tidy a symbol.

## Time and size

- About 30 minutes of wall-clock time for this session, by the system clock:
  roughly 20:45 to 21:15, including 13 look-and-fix iterations with renders.
  That's well under the 2-hour budget.
- `paint.py`: 990 lines, including comments and blank lines.
