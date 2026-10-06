# Studio notes

Facts about the canvas, the paint and the tools at this easel, by operation:
what you do, what the paint does and, where it's known, why. Widths and
positions are in canvas units (the canvas is 1000 units wide).

## The canvas and ground

1. The linen's vertical threads are closer-spaced and more even than the
   horizontal ones, which carry thick slubs a few millimeters long.
2. The ground chosen in `canvas{}` shows through thin paint; body color
   hides it.
3. Light paint laid at light pressure over a wet dark catches only the high
   points of the knife-textured ground. The dark stays in the hollows as a
   net of fine dark lines that looks like cracking.
4. Bristles plough paint off the tops of the weave, so a lay-in can keep
   small specks of bare ground. A denser pass covers them.

## Paint in thin films

5. A coat of paint over paint of its own masstone looks the same at any
   thickness, so a mark from a pile matching what it lies on disappears
   into it.
6. A thin film doesn't look like its pile. Thin dark or earth mixtures over
   a pale passage come out lighter, warmer and more saturated than the
   masstone on the palette.
7. The thin fringe of a dark stroke over a light field dries as a warm
   brown halo.
8. Lead white hides strongly (about 0.82 of a full hide per coat); smalt and
   pale smalt are semi-transparent (about 0.3). A thin veil of a pale
   lead-white mixture veils and lightens what is under it, like a scumble,
   rather than tinting it.

## Brushes

9. Every brush is blunt unless given a `point`. A blunt brush's mark runs
   from about half its width at the lightest touch to its full width pressed
   (a little more with splay), so it can't taper to a point.
10. A pointed brush at light pressure lays a hairline, and its width grows
    with pressure. A band a few units wide needs real pressure or a blunt
    brush of that width. A stroke whose pressure falls off at the end draws
    down to the tip and still reaches the end of its path.
11. A brush keeps its paint across strokes. Each stroke lays most at its
    start, and the load runs down along the stroke, so a long drag with a
    wide brush runs dry partway and leaves starved streaks and bands.
12. A well-loaded hair wets the hollows of the weave. A nearly empty one
    skims the tops (dry brush, broken color). Soft hair bends down into the
    hollows; stiff bristle rides on the tops.
13. Dry brush breaks up only at very low loads (well under 0.1 of a full
    load). At loads that still seem light it lays continuous lines or flat
    patches with clean edges. (reported, not verified)
14. A pointed brush loaded below about a quarter stops wetting the hollows,
    and its thin line breaks up along the weave. Run dry, the tip loses its
    point and splits.
15. A pressed touch lays one continuous, nearly round patch (the paint
    bridges the hairs) that grows with pressure.

## Strokes, passes and clipping

16. `clip=true` stops every bristle on the mask's edge, weighted by the
    mask's value. A clipped fill reproduces the mask's outline exactly, as
    one even line, steps and corners included. The `detail` and `blend`
    hands are clipped by default. `broad`, `body`, `hatch`, `glaze` and
    `scumble` are not.
17. Unclipped strokes run past the mask's edge by their own length and
    width. Along a long straight edge they throw lumps into the field
    beyond, and a brush wider than a band spreads well outside it.
18. The square ends of unclipped strokes notch a shape's edge into steps.
19. In `work`, the mask's value only decides where strokes start (at or
    above `threshold`: 0.3, or 0.1 for `detail`). Unclipped, a soft mask
    edge doesn't make a soft paint edge; strokes end where their lengths
    end.
20. `work` leaves the ground showing between its strokes. With `fill=true`
    it looks at its passage afterward and dabs paint into the bare spots its
    strokes left. Single `stroke` and `touch` calls don't fill in.
21. Short strokes planned along a band about one unit wide come out as a
    dashed line: the planner lays them separately, with gaps.
22. Parallel strokes that all break at the same places leave a row of stroke
    ends across the passage, and the outer strokes' ends make a lobed edge.
23. `cut_in` with a round along a curved edge lays short strokes that follow
    it as a string of small bumps.
24. In a smooth lean passage, the loaded starts of broad strokes stay as
    lighter blotches. Blending reduces them but doesn't remove them.

## Wet into wet

25. A brush over open paint lifts some of it and mixes it with its own. A
    clean brush lifts wet paint, less as it sets, and nothing from set or
    dry paint.
26. Light touches into a wet dark dissolve into it. The same touches on
    paint that has set stay where they land.
27. A pale stroke dragged through a wet dark picks up the dark and comes out
    streaked, dirtier and less opaque, so what is underneath shows.
28. A dark stroke laid into a wet pale passage lifts the pale and comes out
    paler than its pile.
29. A passage laid against a wet neighbor drags the neighbor into its edge.
30. Crossing strokes in wet paint thin each other at the crossings, and the
    layer below shows there.
31. A tacky surface grabs: a brush dragged over it leaves its paint sooner
    and in broken patches (stick and slip).

## Stippling

32. Touches into a wet layer lift some of it and fuse. On a dry layer each
    touch stays separate.
33. One dip serves one small patch of touches (24 by default), and each
    dip's mixture varies a little. On a dry layer the passage can dry as a
    patchwork of slightly different patches; fewer touches per dip make the
    patches smaller.
34. Coverage is how many touches fall on each point; one touch covers about
    π(0.4 × width)².
35. Contrast with what is underneath decides how a stipple reads. A coverage
    that gives a veil over a mid tone gives solid patches over a pale field.
36. With `feather` above 0, touches press lighter where coverage falls
    below 1, so a thinning stipple ends in smaller, fainter marks; at 0 they
    keep their pressure.
37. A row of separate touches along a thin line reads as a string of beads.
    A short dragged stroke along it reads as one continuous ridge.

## Blending

38. The blender fuses open paint by lifting it and moving it. It can't move
    set or dry paint.
39. `blend` stays inside its mask by default. Unclipped, it drags wet paint
    across the mask's edge.
40. Touches stippled into a wet layer and then blended fuse into it. Only
    the strongest contrasts survive, as thin traces. Touches laid after the
    blending pass stay on top as a lacy texture.
41. Repeated blending of one lean layer lifts paint off the tops of the
    weave, and the ground shows through as a pale lattice.
42. Blending spreads each thin spot in a layer into a soft blotch.

## Glazing

43. A brushed glaze (`hand="glaze"`) isn't clipped.
    Over a dry pale passage it lays its thin film mostly on the tops of the
    weave, speckles and, once blended, reads lighter. (reported, not
    verified)

## Masks

44. Every mask covers the whole canvas. A mask function that isn't zero far
    from the spot you mean paints there too.
45. Masks from shapes (`poly`, `rect`, `ellipse`, `ribbon`) are crisp.
    `soften` and `blur` give them a ramp. `roughen` replaces the ramp with a
    new edge at the 0.5 level, so a long soft fade comes back as a hard
    edge.
46. `ribbon` puts a round disc at every point, both ends included, so it
    reaches half a width past its end points.
47. Two separately softened masks that meet leave a light gap or a dark
    overlap along the seam. A mask and its inverse (`m` and `-m`) meet
    without one.
48. A passage bounded by a mask shows a cut edge wherever its paint differs
    from its neighbor's at the mask's edge.

## Drying and time

49. Paint goes from open (workable) to setting (stiff, barely blends) to
    tacky to touch-dry. Lead-white-rich paint gels in about 2 hours and
    average paint in about 3.5. A lean coat is touch-dry in about a day;
    bone black and lakes take days. Thick or oily paint dries slower.
50. `wait` ages every point of the canvas at once. Which passages have set
    depends on their pigment, thickness and oil, not on where you worked.
51. A long pass is painted in 15-minute slices that age
    between them, so its first part is older than its last. While the work
    goes on, the paint under a later passage is still open and comes up
    into it.

## Varnish and cracks

52. `varnish` lays a yellowish film (0.4 coats by default) that only
    absorbs. It warms the darks without veiling them, grays the blues and
    turns pale cool grays warm.
53. In `cracks`, `width_um` sets how strongly the network reads. `dirt` and
    `grime` move a crack's tone a little. `depth_um` and `cupping_um` only
    shape the relief, which shows in raking light.
54. A crack reads by contrast: a dark line in lights and a faint light line
    in darks (its walls show the pale ground). In a mid tone about as dark
    as its fill, it barely shows.

## Pencil

55. Graphite doesn't take on wet paint, and paint laid over it seals it.
    Thin paint lets a line show and body color hides it. Under a thin first
    layer a firm line reads as a thin dark line until a fuller layer covers
    it.



## More notes from the studio

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
