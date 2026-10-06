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
- `medium` above 0.95 is an error, and the chunk changes nothing. There's no color sampling: `print(pile)` lists a pile, and matches are made by eye (one patch took three tries: too dark, too light, close, with a faint edge left).
- Repainting a built-up passage with its first lay-in piles came out paler and warmer than the passage.
- A pale fill laid against a wet strong-colored neighbor picked up that color and came out blotched.
## Strokes
- `ribbon` on straight point lists gives stiff shapes. Densifying points on a spline, varying width with noise and roughening gives an irregular edge.
- A pointed round from `pressure_for(w)` down to 0 tapers blades and twigs: 900 short blades took about 1 s, 1,450 clumps about 3 s.
- Separate light blades on a dark field read as contrasty cutouts. Many fine dark and mid blades over a dry dark field unify it. Blades read best crossing a dark edge against a lighter field.
- A `detail` ribbon tapering from 3 to 0.7 units reads as a ruled rope.
- Unclipped `body` work (filbert 5) threw strokes well past a band's mask across its neighbors.
- A lower shape painted first showed through a dark shape laid over it as a pale band. Restating at coverage 4.5 covered it.
- Brushes have no `:dab`; `:touch` lays a single mark.
## Blending
- Badger passes over freshly laid bands, all still open, softened the seams but left the strongest one. It took several passes plus an intermediate band.
- A blend whose mask takes in an unpainted gap drags paint into the gap. With `clip=false` it drags one shape into its neighbor.
- Blends bounded by `rect` masks leave seams at the rect edges. Crossing blends leave swirls.
- Long level blends (`ruler=true`, lengths 120 to 400) over open paint give a smooth, even passage.
- Blending wet light blades into a wet dark field smeared them into gray and pale blotches.
- Blending a wet dark coat laid over lighter paint lightened it each time: the blender lifted the lighter paint into it.
- Blending fresh touches over a dry layer barely changed them. Blending a fresh glaze over dry paint lifted the glaze and didn't even it out.
- Blending narrow wet strokes on a grown, softened mask turned them into soft bands with blunt, rounded ends.
## Wet into wet
- Stipple touches onto a wet dark fill of the same area vanished into it. They held once it was dry.
- A darker second pass over a wet light shape still came out light, mixed with the paint under it.
- Blades laid on tacky paint held.
## Glazing
- `hand="glaze"` (a 26-unit brush) isn't clipped. On a rect it spilled over a dark shape as a pale band, left halos and ended in a hard edge. With `clip=true` it stayed inside.
- A thin glaze (medium 0.9, `load_at` fading) went on as visible streaks. Three rounds of level and vertical badger passes while wet made it smooth (30 s of compute), with a faint seam at its boundary.
- A clipped dark glaze (medium 0.75) over dry, textured blades, its load graded down the passage, darkened them without streaks. Over a dry smooth area (medium 0.55) it went on blotchy.
## Stippling
- Clustered stipple on a union of ellipses gives solid, scalloped, blobby masses. A sparse ring outside them (feather 0.5) reads as a fuzzy halo of dots.
- Sparse pale stipple (coverage 0.18, cluster 0.9) in a dry dark mass reads as bright specks, not openings. A day later, dark stipple at coverage 1.3 covered most of them.
- Coarse stipple (width 4.5) over a dry light layer leaves pale gaps where the layer shows through. From close up, the specks later called for a smooth repaint.
- Stipple on a ring minus a shrunken interior leaves a dark ring around a lighter center.
- Dense stipple over dry dark round patches absorbed their outlines.
- Sparse fine stipple (width 1.3, coverage 0.25) along a thin band read as a row of marks like lettering.
## Drying and time
- A lean, lead-white-rich layer (medium 0.25) was dry at two heights after 22 h and still tacky between them.
- Green earth, umber and ochre paint (medium 0.15) was tacky at 30 and 50 h. At 86 h one point was dry and others were still tacky.
- Dark earth and bone-black paint (medium 0.08 to 0.1) was tacky at 40 h and dry in places at 88 h. Dark stippled masses were still tacky after 3 days.
- A thick dark repaint (coverage 4, then blended) was still tacky 3.5 days later.
- `varnish` needs everything touch-dry, so it was left off while passages were tacky.
- A canvas with two ground layers took 35 s to compute. Big repaint and blend chunks took 30 to 35 s.
## Masks and edges
- `above(f, true)` came out nearly empty inside the intended region, and a blend over it did nothing. `m - above(f)` worked.
- `m:offset` exists; there's no translate.
- A rim made as `(m - m:shrink(2.5)) * rect` also traces the rect's cut edges and any holes. Intersecting with `m:shrink` keeps only the true contour.
- Subtracting a protecting mask also shuts out patches inside it that need paint.
- Masks split by a y threshold paint a form as stacked horizontal bands. Diagonal facet polygons plus a crack stroke broke it up.
- Noise-threshold masks make hard-edged islands. A clipped fill cut by a rect leaves a straight edge.
- Repainting a strip beside a dark shape left a pale line at its very edge. Growing the dark shape 1.6 units past its mask covered it.
- An edge mask built from a shape's own mask landed mid-band, because the paint had spread past that mask.
## Looking
- `--crop` is 1:1 at 2.4 px a unit, up to 1,200 px a side (about 500 units). A bigger crop is an error, reported after the chunk has run.
- Specks, seams and halos showed only in crops. Flat or busy passages showed in the whole view at 1600.
