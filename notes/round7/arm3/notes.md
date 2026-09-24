# Round 7, arm 3 (the easel): "Evening on the Bodden"

Source: `paintings/lua/bodden.lua` (17 chunks, hand time on, sittings
enforced). Renders: `painting_1000.png`, `painting_3200.png`
(`easel run paintings/lua/bodden.lua --width 1000|3200`). `easel check`
confirmed that the replay matches the live canvas.

## What it shows
A calm lagoon after sunset, landscape format (aspect 1.45). The horizon
sits at y 432, so the sky takes 63% of the canvas. A small town lies on the
far shore with a Gothic tower on the vertical axis, a second church with a
turret, chimneys, a few trees and a windmill on each side (the left one
larger). Groves stand low along the shore. The sky runs from a deep gray
blue at the top to a yellow glow at the horizon. A pink-gray stratum hangs
high on the left, with a fainter one on the right. The water mirrors the
glow and darkens toward the viewer, and it holds a soft reflection of the
town. The foreground is a dark grassy bank with a few small groups of reeds
at each side. There is no figure and no sun or moon.

## Order of work (sittings)
1. Sitting 1: a pencil drawing (a 2H ruled horizon, the shore and the bank;
   the town, mills and axis in 3B). A thin raw-umber lay-in on the bank,
   blended. Then the first sky layer: seven piles laid in bands from the top
   down with the default broad brush, wet into wet, and a blend. Then the
   first water layer: seven piles in long level strokes and a level blend.
2. Sitting 2 (rest 16 h): the glow zone of the sky stippled (width 4,
   coverage 1.5). Each touch comes from the pile for its band, mixed 45%
   with what is already there, with a fade. It took about 4.8 h of hand
   time.
3. Sitting 3: the upper sky stippled the same way (width 4.5, coverage 1.35),
   overlapping the glow zone. About 4.8 h.
4. Sitting 4: two strata. Their masks are ribbons, laid with the broad brush
   through `color_over`, warmer on their undersides, then blended. Then the
   far shore, groves and town in body color with small brushes, with
   `edge=` (the town found against the light) and a clipped level blend.
5. Sitting 5: the spire, the pinnacles and the turret drawn to a point with
   a pointed round (point=1). The mills (a body, then four sails as round
   strokes). Chimneys and trees in the town. The second water layer, whose
   color comes from sampling the painted sky at its mirror height, darkened
   toward the viewer. The town's reflection was dragged down into the wet
   water with vertical strokes, and the whole water got a long level blend.
   Then the bank's body color, laid wet against the water's edge.
6. Sitting 6 (rest 40 h): calm lines on the water (horizontal round-brush
   strokes in the mirrored-sky color, closer together toward the horizon)
   and a few dark ripples near the bank. 672 grass blades along the brow
   and more through the bank's body, sparser and larger toward the viewer
   (rigger, point=1, set down at the root and lifted off). Six
   groups of reeds with leaves and hanging panicles. `lose()` on the
   strata's edges.
7. Finish: `wait(24*60); varnish{}; cracks{}; relief()` (the varnish waited
   26 days for the paint to dry).

## Tools used
pencil (2H, 3B, rule, sketch, line), `work` (broad, body, detail, hatch),
`blend`, `stipple` with `color_over` and `fade`, `edge=`, `lose`,
`outline{}` (groves, town trees), `ribbon`, `poly`, `sample` (the water
reflects the painted sky), brushes with `point=1`, `rest`, `sitting`,
`timesheet`, `look --scale 3.2`, `--dried` and `--mode value,squint|mirror`.
I did not use the world, sky, clouds, form, rock or tree generators.

## Engine problems hit
- A **flat 12** or **filbert 10** in `hand="broad"` with long strokes (70 to
  600 units) laid paint mostly at the stroke rims, and the ground showed
  through the middles. It read as outlined "fish scales" in the sky, and
  in the water as horizontal stripes of orange ground. The same thing
  happened with a filbert 5 in `hand="body"` at medium 0.5 and load 0.45.
  Workaround: the style's default broad tool (a lab test on a scratch
  canvas compared eight variants).
- A thin, fluid wash (medium 0.7, load 0.4 to 0.5) left each stroke with a
  dark rim. A `blend` while wet fixed it on the bank, but not in a pale
  sepia over the water, so I dropped the water lay-in.
- The first `blend` of the sky lifted paint and opened patches of red
  ground, because the flat-brush strokes under it were thin.
- The first level-water attempt came out as cloud-like blotches, because
  the broad preset curves and drifts its strokes. `curve={0.01, 0},
  cross=0, drift={0, 1}` made them level.
- Under hand time, the sketchbook's sky stipple (width 2.6, coverage 2.6)
  would take about 23 h for this sky. Stippling the glow zone at width 3.2
  and coverage 2 ran past a 6 h sitting and the chunk was refused. So I
  stippled coarser (width 4 to 4.5, coverage about 1.4) over two sittings.
- A stipple laid band by band in pile colors, each lifted +0.012 L, made
  hard stripes. Mixing each touch with the color under it
  (`color_over`) fixed that.
- Cloud strata painted as filbert strands read as scratchy lines. Painted
  as `body` masses with `edge="soft"` and underside strokes, they read as
  outlined cut-outs. A low stratum over the glow, done with the broad
  brush, read as a row of round blobs, so I dropped it.
- The first reed beds were masks hatched in vertical strokes. Their mask
  ran to the bottom of the canvas, and a 4 h sitting was refused. Once
  bounded, they read as domes, so I replaced them with small groups of
  stems.
- Panicles placed at a stem's geometric tip floated above the stem, whose
  last 60% had faded (`ramps={0.03, 0.6}` with pressure falling to 0.02).
  Shortening the release to 0.25 fixed it.
- `easel try --look` shows the canvas after the rollback, so previews of
  paint needed `do` and then `undo`.
