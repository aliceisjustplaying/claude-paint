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

# Studio record: what the paint and tools did

## Mixing piles
- A wet mix rich in lead white takes over. Opaque shadow mixes with 1 to 3 parts white came out far paler than planned once laid or blended, even on a dry layer.
## Strokes
- Hand-planned strips painted side by side from a graded row of 5 or 7 piles read as separate stripes or stepped bands.
- Seven piles with curved strokes 0.075 of the form's height apart, overlapping and painted dark to light with no blending, gave a continuous turn. The stroke ends left a scalloped texture.
- Low-load strips (load 0.35 to 0.45) over a dry mottled layer read as light sausage shapes, not as a darkening.
- Strokes of field color clipped to run along the outside of a contour left a scalloped edge and dried darker than the field.
- A dark `broad` pass with `edge="firm"` around an excluded shape left a warm orange halo along that shape's edge.
## Passes with soft masks
- A `work` pass on a soft value-derived mask with `threshold=0.4` came out as separate short dabs, a scaly look.
- Scumble and glaze passes clipped to a smoothstep mask did not fade at the ramp. They cut hard bands or blotches at the threshold.
- A scumble on a soft radial mask (`threshold=0.25`, `edge="soft"`) came out as a hard-edged, scalloped patch.
- A `body` pass on a band softened by 20 units with `threshold=0.35` left blotchy stripes.
## Glazing
- With a white-containing glaze (medium 0.45, coverage 1.6), the strokes stayed too opaque and blotchy.
- A glaze of umber, green earth and black at medium 0.6 and coverage 3 went on far too dark.
- An umber glaze at medium 0.85 to 0.88 and coverage 1.5 to 2 went on as separate strokes with a hard edge. Two or three badger passes then turned it into a mottled half tone.
- A glaze of ochre, umber and green earth with a soft value mask covered nearly the whole small shape, and blending smeared it into an olive mottle.
## Wet into wet
- A dark strip laid into a wet pale layer lifted the pale and came out mid-light.
## Blending
- Blending a zone with a soft edge left the zone's outline as a hard seam, twice. Blending the whole field twice at crossing angles dissolved the seam.
- A 40-unit badger over a whole small shape averaged its modeling to one flat mid value.
- A blend over a layer of strips rich in lead white pulled the whole shape to its palest value.
- A 10-unit badger at one slant over small shapes dragged them into diagonal streaks.
- A 12-unit badger along a narrow band lifted paint rather than fusing it.
- A blend mask that reached still-wet neighboring paint dragged that paint out as a large blot.
- A thin band excluded from a field blend by a 2-unit margin still got smeared.
- A horizontal blend band across fresh light strokes dragged the light over the adjacent dark and left a line at the band's top edge.
- A freshly painted band blended with the field around it pulled out into flame-like streaks. Three more passes with a 40-unit badger at three angles melted them into a soft glow.
- A shadow painted wet into a re-laid field, followed by one horizontal blend of the whole field (clipped), came out soft and clean.
## Lifting
- A clean flat (stiffness 0.6) wiped fully after every stroke lifted most of a wet glaze. It left streaks along the stroke direction, or a faint grid when the strokes were short.
- A badger pass afterward evened those streaks.
- A wider, softer flat (14 units, stiffness 0.4) in four passes lifted a fresh dark line where an 8-unit flat had only streaked it.
## Stippling
- A 2-unit stipple at coverage 0.12 over a dry layer left separate dark dots, strongest on the light areas.
## Drying and time
- Lean mixes rich in lead white, and dark field mixes with medium 0.1 to 0.2, were all tacky after 24 h.
- After 42 h a twice-laid lean band was dry and the rest still tacky. After 66 h only the most-layered part of the dark field was tacky.
- A second layer rich in lead white over dry paint was tacky at 40 h and 60 h and dry at 96 h.
- A chrome yellow layer was tacky at 48 h and dry by 84 h. Umber glazes at medium 0.85 to 0.88 were dry by 72 to 96 h.
- A few thick strokes of umber, black and red earth were still tacky at 60 h while the lean paint beside them was dry.
## Masks and scaffolds
- `form:value` is low off the solid, so a mask built from "value below x" spills past the solid's outline onto everything around it. `f:silhouette{}` gives the solid's own outline.
## Finishing
- `varnish{coats=0.3, vary=0.08}` on a touch-dry canvas warmed the picture slightly and brought neighboring areas together.
## The easel
- A chunk with an endless loop hung the easel, and `status` and `close` then timed out as well. Stopping the process and running `open` replayed the log with globals intact, and the hung chunk left nothing.
- Chunk times seen: the canvas with two ground layers took 41 s, a whole-field blend at 1000 x 520 units took 32 to 55 s, and `check` replayed 94 chunks in 529 s.
## Looking
- A crop is shown at 2.4 px per canvas unit. Close crops showed faults that the whole view hid: halos, specks and lines left at a band's edge.


## More notes from the studio

# Studio record: what the paint and tools did

## Mixing piles
- Neighboring piles of 3 to 4 parts lead white with small amounts of blue and earth laid down as nearly the same flat pale gray.
- A glaze pile of umber, bone black and red earth at medium 0.7 over a dry dark layer went on too warm and too saturated. The same glaze with cobalt in place of red earth (medium 0.65 to 0.7) darkened the layer and stayed cool.
- A patch of a mid pile painted over and beside partly wet dark paint came out paler and whiter than planned.
## Strokes and passes
- `work` in wide horizontal bands (filbert 14, coverage 2.5, `fill=true`) left the bands as separate stripes with the ground showing between strokes. Only blending fused them.
- `detail` strokes (round 4, 6 to 16 units, steep angle) along a mask with a jagged top turned every bump into a sharp spike.
- A `body` pass with vertical strokes on a soft mask (`threshold=0.4`) laid crude vertical streaks.
- Low-load dabs from a flat 3 to 5 (`load_at` 0.25 to 0.65, coverage 0.9) in three piles over a dry layer read as a cobbled patchwork until a horizontal blend fused them.
- Hand-placed horizontal flat strokes of one width across a field read as stacked slabs. Widths and spacing that grew toward the bottom read as a flat plane receding.
- Thin pale strokes (flat 0.9 to 3, load 0.2 to 0.6, pressure falling to 0.05) on a dry dark or mid field stayed crisp and stark, like dashes or painted lines.
- A flat stroke pressed to 0.8 to 0.9 with default ramps gave a vertical bar with rounded ends. `ramps={0.02, 0.02}` on a stiffer flat (0.7) squared the ends, and a light dragged touch across the top flattened it further.
- Reflection strokes along sine-wave paths read as wriggles.
- A rigger (point 1) pressed 0.9 to 0.4 drew a clean thin line. At 0.1 to 0.45 in two short strokes it made small V marks.
- Strokes offset along both sides of a polyline edge (a dark band outside, a pale line inside) looked piped, like icing.
- `work` rejects a function for `coverage` ("expected number"). `load_at` takes one.
## Blending
- Two full-field blends at crossing angles (0 and 0.12 to 0.25) of a wet banded layer gave a soft gradient and dissolved narrow lobed shapes laid wet into it into soft streaks.
- A blend inside a rectangle over fresh vertical strokes smeared them into a hard-edged block that also spread over dry paint beside them.
- Blending a wet dark to light gradient with a pale rim modelled a smooth rounded form.
- A blend mask grown and softened around shapes on a dry field softened their edges but not their outlines, and pulled wet paint in from a tacky neighbor.
- A narrow badger (14) in a ring along an edge softened it but smeared wet marks nearby and left a patch of pale lattice.
- A soft rectangle blend (badger 20) over thin wet strokes on a dry field took most of them away.
- A blend mask shaped like the passage itself (grown 4, softened 4) fused broken strokes into a mottled mass with no rectangle edge. Paint lifted off the weave tops left a pale lattice.
- A narrow badger (16) over crisp strokes on a dry field sank them into the layer.
## Wet into wet
- Dark strokes into a setting or tacky pale field came out lighter and streaky, left gray lifted patches and once a ring where an edge was disturbed. A second dark pass did not cover them.
- A pale fill painted up against a dry edge left a paler strip along the join.
- Dark horizontal strokes into a fresh pale band, then one horizontal blend, gave a soft hazy dark mass.
## Glazing and scumbling
- `hand="glaze"` clipped to its mask, then two blends, darkened a dry area and left a fine crackle-like texture visible close up.
- A `scumble` (medium 0.3 to 0.35, load 0.35, coverage 1) went on as an opaque block with a scalloped edge. Three blends over the whole area dragged it into a light-to-dark gradient.
- A `body` pass of a white-rich pile at medium 0.35 (load 0.5, coverage 1.5) then two badger 30 blends in a softened ellipse gave a subtle soft brightening with a faint edge at one end.
## Stippling and touches
- A stipple (width 2.4, coverage 0.9, feather 0.4) on a mask of a shape's upper-left rim showed as a warmer broken edge.
- A stipple on `ellipse:rim(4, 2) * mask:grow(3)` made no visible change. About 40 round-brush touches placed around the ellipse did reshape its edge into small lobes.
- Small stippled clumps of one size read as uniform specks. Larger ones with `drag` read as blotches.
## Drying and time
- A lead-white-rich lay-in was setting 4 h after it went on, mostly tacky at 6 h, still tacky at 25 h and dry at about 55 h.
- Umber and bone black paint was still tacky 3 days after it was laid and dry by 5.5 days. A bone black mix laid in body was tacky at 36 h and dry at 84 h.
- Small dark detail touches were tacky at 40 and 61 h and dry by 110 h. A glaze at medium 0.7 was dry within 72 h.
- After 120 days every point sampled on a grid was dry. `varnish{coats=0.3, vary=0.08}` then warmed and unified the canvas.
## Masks and edges
- A union of roughened ellipses gave a lobed silhouette. Where the ellipses and a base strip failed to overlap, a gap stayed unpainted and later passes clipped around it showed through.
- A smoothed `poly` made rounded domes. Straight `poly` edges read as ruled lines.
- Lens masks built from sine profiles came out as sausage shapes with even hard edges. Clipped fills of them (`clip=true`) looked cut out and pasted on.
- Thresholded stretched noise showed only where the noise crossed the threshold, sometimes on one side of the canvas only.
- `noise{stretch={angle=, k=}}` errors ("nil to f32"). `stretch={angle, k}` works.
- Clipping a pass to a field minus a dark shape (`- mask:grow(0.5)`) kept that shape's edge crisp.
## Looking
- Crops are 2.4 px per unit, so a crop wider than 500 units fails. A 500-unit crop showed smears, rings and lattice patches the whole view hid.
- `--mode mirror` showed a hard edge the normal view hid. `--mode value,squint` showed a value range too narrow to see in color.
- Two whole-field blends took 39 to 45 s. `check` replayed 80 chunks in 399 s.
