# Studio record: what the materials and tools did

## Canvas, ground and chalk
- A two-layer ground (a knifed pale layer under a thin brushed warm layer) gave an even buff field with faint brush striations and a few pale streaks.
- `chalk()` lines on the dry ground came out as grainy gray lines broken by the weave. `sketch` with `wander` 1–2 doubled some contours slightly.
- Body paint laid over the chalk hid it completely; no line showed through the dark lay-in.

## Mixing piles
- A dark pile of earths and black with a little lead white (about one part in eight) and medium 0.2 dried on the canvas as a pale mauve gray, far lighter than its masstone.
- The same kind of dark pile without white, laid over it, came out mid brown, not near black, even after two coats and a blend.
- A pile with a green-earth share, laid as the dark side of a light form, read olive green against the lights.

## Strokes and passes
- `work` with `hand="broad"` and no `fill` (coverage 1.2–1.8) left bright gaps of bare ground in places, as small crescents with crisp edges.
- `broad` strokes stayed visible as long dragged marks after one clipped `blend` at coverage 1.0.
- A thin horizontal band (about 18 units tall) worked with a 16-wide filbert came out as a row of stacked square dabs with a lumpy top edge.
- A wider band (about 100 units) worked with `body`, a 20-wide filbert and `fill=true` came out as a patchwork of separate rectangular dabs in two tones, not a smooth plane.
- Unclipped vertical `body` strokes in a lower block ran up past the mask's top edge as a row of dark notches into the band above.
- `fill=true` inside a clipped shape left many small round dabs that read as a bubbly texture in the lights.
- `work` with `body` and a filbert 9–12, unclipped at coverage 1.4 and no fill, over open pale paint, laid separate short dashes instead of a solid dark shape.
- One flat brush 3 wide, loaded 0.9 at pressure 0.6–0.75, drew a continuous thin pale line across the full 1000 units without running dry.
- A poly about 4 units wide filled with the `detail` hand and a round 2 came out as a single thin line.
- A pointed round 2.5 at pressure 0.15–0.65 laid a hairline highlight that swelled only slightly in the middle.
- A filbert 8 loaded 0.45–0.5 at pressure 0.2–0.5 laid narrow streaks, not soft washes.

## Blending
- A clipped badger `blend` at a shallow angle over three overlapping ramps fused them only partly. Its passes stayed as broad bands at the blend angle, across the underlying strokes.
- A clipped blend over a shape with lobed edges left diagonal light stripes over the form at the blend angle.
- A vertical `blend` over a dark block left vertical streaks.

## Wet into wet
- A lead-white-rich unclipped `broad` pass with a filbert 22 and `fill=true`, then a blend, over a still-open band covered the earlier dark dashes and gave a smoother, cloudier surface.
- The same pass ran past its mask (which excluded shapes shrunk by 2 units) into the open paint of neighboring shapes and veiled their lower parts pale.
- Dark strokes (filbert 8, pressure fading to 0.1) laid into that open pale band afterward came out faint gray, much paler than their pile.
- A later `broad` pass with medium 0.35 over a setting dark layer unified it. The lower layer survived only as scattered streaks where the strokes thinned.

## Glazing and stippling
- Neither `hand="glaze"` nor `stipple` was used.

## Drying and time
- Chunk clock times: a chalk chunk about 1 minute; a full background and shelf lay-in about 73 minutes; a second large lay-in about 50 minutes; blocking in three shapes with blends about 46 minutes.
- At 2 hours in, `drying()` found a dark earth-and-black layer (medium 0.2–0.35, last worked early in the previous chunk) `setting`. A lead-white-rich band laid later in that chunk was still `open`.
- `wait(0)` returns the clock without advancing it; a chunk that only calls it still becomes a logged chunk.

## Masks and edges
- `clip=true` with a `firm` outline gave a perfectly smooth, crisp silhouette, like cut paper.
- `clip=true` with a `soft` outline gave a ragged, lobed and fringed silhouette.
- A `ribbon` along a few points, filled clipped, came out as a hard-edged band with angular joints at the points.
- Two overlapping ramp masks worked with unclipped `broad` strokes, one after the other, did not grade into each other. The second pass's stroke ends formed a ragged feathered seam over the first.
- Shapes reserved from a pass by subtracting a shrunk mask still took unclipped strokes that ran into them, leaving the reserved ground with ragged edges.
- A small clipped ellipse filled with a filbert 6 kept specks of the lighter layer underneath inside it.

## Looking and the easel
- A whole look is 1000×800 pixels for a 1.25 canvas. Crops are 2.4 pixels per unit, so a crop can span at most 500 units a side; a 700-unit crop was refused.
- A `do` whose shell `timeout` (30 s) ran out still ran; `status` then showed the chunk logged.
- Compute times: large broad lay-ins about 27 s, three clipped shapes with blends 16 s, many single strokes 7 s.
- Close crops showed the fill dabs, blend bands and stroke ends that the whole view hid.
