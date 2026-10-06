# Studio record: what the materials and tools did

## Canvas, ground and chalk
- A ground with a knifed pale layer (80 µm, texture 0.25) under a thin brushed warm layer (35 µm) gave an even buff field with visible tooth. `canvas{}` took 41 s to compute.
- `chalk()` `sketch` and `line` over the dry ground were quick: a whole drawing chunk took under 0.5 s to compute and about 2 minutes of clock time.
- The first body passes hid the chalk completely.

## Mixing piles
- Near-masstone strokes vanish: filbert strokes from a pile close to the passage beneath (the mid tones) barely showed. Only the lead-white-rich and near-black strokes read.
- About half a part of Prussian blue in roughly a dozen parts of an earth mix pushed the whole mix visibly toward blue-green.
- A dark earth-and-black pile laid in a thin dark stroke over a dark field came out as a pale gray hairline, lighter than the field.

## Strokes and passes
- `work` with `hand="body"`, unclipped, with `fill=true` at coverage 1.5–1.6 laid a shape as many short, separate dashes. It read as a stippled or pixelated patch with a ragged outline of stroke ends, not a solid mass.
- The same hand over a ribbon about 14 units wide scattered stubby dashes across and past the band, so it read as a ring of bristles.
- `body` at coverage 1.4 with no `fill` laid a dark oval as a heap of separate short dark strokes. A later medium-rich pass and a blend darkened it but didn't fuse the strokes.
- Single filbert 10–12 strokes of lead-white-rich paint over setting paint stayed as crisp, separate pale bars with square ends.
- A filbert 10 stroke of a dark pile over a light passage that was setting, at pressure 0.7–0.9, came out as a thin, translucent streak narrower than the brush.
- A filbert 5 stroke about 50 units long, with pressure rising then falling (0.5, 0.7, 0.2), laid a narrow lens-shaped mark that tapered at both ends and had crisp edges. A second stroke offset by about 4 units gave a two-tone version with a dark lower edge.
- A pointed round 2.2–3 laid pale hairlines at pressure 0.2–0.7, and a pressed `touch` laid a small round dot.
- A curved band drawn with a round 6 (dark, pressure 0.7–0.85) plus pointed-round strokes read only as thin pale arcs, not as a band of that width. The dark stroke disappeared into the dark field under it.
- Two long horizontal strokes about 1000 units long, a pale one from a pointed round 2.5 and a dark one from a round 4 just below it, didn't read as a distinct crisp line in a close crop.
- Flat 8 strokes at pressure 0.5 over a pale band left only thin streaks.

## Blending
- A clipped `blend` at coverage 0.8–1.0 over open `body` dashes didn't fuse them. The dashes stayed.
- A `blend` at coverage 1.8 over a whole field, right after a medium-rich (0.35) broad pass on a sine ramp, removed a hard vertical seam between two open passages and left a broad soft transition. Stroke streaks survived at the far end.
- A medium-rich (0.25) filbert glaze over separate pale bars, followed by a `blend` in an ellipse in the same chunk, covered the bars and gave a smoother passage.

## Wet into wet
- An unclipped `broad` pass with `fill=true` over a patch next to an open, freshly painted shape ran over it. The pass buried about a third of the shape, including its whole thin projecting loop.
- An unclipped `body` fill of a rectangle, using the same pile as the field under it, came out as a lighter patch with hard edges. By then the field had been changed by a darker medium-rich pass and blend.

## Glazing and stippling
- A flat 12 loaded 0.6 with a lean-dark pile at medium 0.4, at pressure 0.4–0.5 over a dark stroked patch, made no visible change.
- Neither `hand="glaze"` nor `stipple` was used.

## Drying and time
- With medium 0.15, the dark passages were still `open` after about 75 minutes and again at 2.5 hours. Paint at medium 0.1 laid 1–1.5 hours earlier was also still `open`.
- After `wait(180)`, three passages (medium 0.1–0.35, the oldest about 3 hours old, the newest about 20 minutes) were all `setting`.
- Right after that wait, the middle of the field showed the weave as a fine pale speckle, most visible in value mode. That part had taken several broad passes and blends of up to 1.8 coverage. The speckle hadn't shown before the wait.
- Clock times: a broad lay-in of three large bands with blends took about 76 minutes; a single broad ramp pass with a whole-field blend took 19; blocking in two shapes with `body` passes and blends took 53; a chunk of fills, ribbons and blends took 35. A chunk of about 20 single strokes, one clipped fill and one blend took 21 minutes, and about 30 strokes and touches took 4.
- `wait(0)` returns the clock without advancing it, and a chunk that only queries `drying` still becomes a logged chunk.

## Masks and edges
- Two broad passes on ramp masks (0 to 1 over 200 units) laid one after the other met in a hard vertical seam. The later pass's stroke ends formed a lobed, ragged column along it, because the ramp only decided where strokes started.
- Unclipped broad bands laid under and over a straight horizontal edge left it lumpy, with stroke ends running across it both ways.
- A clipped `body` or `detail` fill of an ellipse or a smoothed `poly` gave a solid shape with a crisp, smooth outline. A small clipped `detail` fill in a thin ellipse showed fine parallel stroke lines inside it.
- A rotated oval with a soft ramp was built with `mask(function)`.
- A shape split into two halves by complementary ramp masks, filled one half after the other, showed a straight seam where they met.

## Looking and the easel
- Whole looks are 1000×800. Crops of about 200–340 units a side came back at 2.4 pixels per unit. In the whole view the value and squint modes showed the tonal masses, and value mode showed the weave speckle clearly.
- Compute times: large broad passes 33–58 s, `body` block-ins 11–17 s, stroke-only chunks 1–2 s. `check` replayed 14 chunks in 158 s and matched exactly.
