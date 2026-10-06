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

## Pencil

52. Graphite doesn't take on wet paint, and paint laid over it seals it.
    Thin paint lets a line show and body color hides it. Under a thin first
    layer a firm line reads as a thin dark line until a fuller layer covers
    it.



## More notes from the studio

# Studio record: what the paint and tools did
## Mixing piles
- In a pale mix, lead white overwhelms the weak blues: pale smalt added little, and a second graded layer came out far paler than planned. Matching a mid blue took nearly as much blue as white.
- Fresh lead-white paint reads paler than aged paint of the same mix. Patches that should have matched showed as pale ghosts and were still paler five days later.
- Correcting patches with deeper mixes gave dark, smoky spots. A single film doesn't fully hide what is under it, so every patch came out differently.
- Earth darks that contain a fair share of white read reddish or pinkish tan, not grey. To darken them, add more bone black and less white.
## Strokes and passes
- `broad` passes over stacked rectangles ran past each rectangle by a stroke length and left scalloped seams. The lowest band ran down over the drawing, and a band laid below a wet field ran up into it and left a lumpy edge.
- `body` strokes on a shrunk mask escaped past the mask's edge as short dash tails.
- `poly(pts, true)` smooths every corner, so points and gables came out rounded. Plain `poly` with `detail`, `clip=true` and `edge="found"` gives crisp corners and thin points.
- `edge="soft"` let paint run past the mask even with `clip=true` and ate into thin shapes beside it.
- Bands painted from separate zone masks meet as hard stripes. A long field painted around rectangular exclusions shows the rectangles as hard boundaries in its texture.
- `hatch` with a pointed round aimed upward, lengths 4–18, coverage about 1.3, reads as grass. In a receding field, strokes that are too long read as tall stalks, so shorten them with distance.
- A thin clipped ribbon of pale paint reads as a bright, even line. Hatching over it through a noise mask left only glints.
- `hatch` rejected a `coverage` function (error converting a function to a number). Pass it a number.
## Blending
- The badger only moves wet paint, but unclipped it drags that paint across mask edges. A 40-wide pass broke a dark silhouette into separate lumps and nearly erased a soft form next to it. An earlier pass smeared pale paint over thin shapes that weren't dry yet. Use `clip=true` on every blend near anything else.
- Over dry surroundings, a clipped, low-pressure small badger (pressure 0.2–0.35) softens only the edge of a fresh patch.
- A vertical badger pass through a narrow band that held dark marks smeared the dark through the whole band, leaving it mottled.
- Small blends inside ellipse zones left the ellipses' edges visible. One light pass along the full length of the band (coverage 1) smoothed it into streaks.
- Blending along the edge of a darker zone pulled it into horizontal streaks. Lit rims that were badgered downward turned into dragged white streaks.
- One badger pass barely touched a blotchy glaze. Three crossing clipped passes (coverage 2.2, 1.5, 1) while it was wet turned it into a soft, mottled mass.
## Wet into wet
- Dark paint laid into a wet pale passage picks up the pale and comes out lighter. Lay the darks after the pale has set.
- A dark pass laid into a wet pale form barely darkened it, and a repair painted into wet pale left a translucent pale veil.
## Glazing and scumbling
- `glaze` with a high-medium pile (0.5–0.6) over dry paint went on as blotchy, worm-like marks. The same happened with a soft filbert (stiffness 0.3) in 25–60-unit strokes.
- A glaze that holds much white lightens: over darker paint it laid pale streaks instead of darkening.
- A thin glaze (medium 0.7) over a small dry dark shape left vertical streaks. A clipped horizontal badger smoothed them, and the shape ended half a step darker.
- A dark glaze laid into wet pale ghost shapes turned them into dark ghosts. A low-load `scumble` (lost edge) over small patches made them larger, soft and pale.
## Stippling
- Clustered stipple in a bright light pile looked like scattered confetti. Lighting each small lobe separately made them all read alike, as repeated balls. One large form driving the shadow stipple unified them.
- Closing a clumped silhouette with `m:grow(14):shrink(14)` filled the gaps between lobes. Holes carved back into it with roughened polys read as punched out until leaf sprays were stippled in from their rims (`drag` and `twist`).
- A sparse dull stipple (coverage under 1, feather 0.8) over a dry dark mass shows as faint, subtle clusters. A stippled ring around the foot of a form looks too regular.
## Drying and time
- `wait()` takes minutes: `wait(60*30)` passes 30 hours.
- A thin lean layer rich in lead white (medium 0.12) was dry in 20 hours. The thick body over it and an oily umber wash (medium 0.45) were still tacky at 24 hours.
- A lead-white-rich layer laid over set paint stayed tacky for more than three days. A dark body with lead white stayed tacky about four days.
- Over tacky paint, new strokes sit on top and don't take up more paint: darkening a line took two full loads after another day.
- A chunk that only prints is still logged as a chunk.
## Masks and edges
- Subtract whatever stands in front from a mask before painting behind it, or the paint goes over it.
- Every repaint cut around a grown or older, wider exclusion mask left a pale ring. Patching the ring never matched. Repainting the whole passage, cut only by the current shape grown by 0.2, removed it.
- In a large even passage, local corrections kept failing: repaint the whole passage instead.
- Restating a shape in pieces leaves seams and mismatched parts, and the old outline shows past the new one. Repaint it as one shape, then paint the surroundings over the old outline.
- A keep test that only checks each stroke's start lets strokes run across the kept shape.
- `mask + number` and `mask:bounds()` are errors (use `:map`). `form:field` takes only `"fall"`, `"across"` or `"edge"`.
- `local`s are gone in the next chunk, but globals persist, even through close and reopen.
## Brushes
- A rigger (point 1) with pressure falling to 0 tapers to a hair and suits tufts and stems. At width 1.2 it read thin, and a round 1.8 with point 0.8 at pressure 0.8→0 was bolder.
- Pale stems against a dark ground read as lollipops. Dark blades woven over them set them into the ground.
- Short vertical flat strokes clipped to a very narrow band were cut into a row of beads.
## Looking and the session
- A crop can be at most 500 units wide (1200 px at 2400 px). `--grid` helps with placing coordinates, and `--mode squint` helps with judging the whole.
- Streaks that show in a close crop can pass in the whole view, so judge both.
- Shell exports don't persist between commands, so pass chunk files by full path. An unset variable in a `-f` path pointed the easel at the wrong file.
- Replay time grows with the log: `check` took 574 s at 147 chunks and 787 s at 208. `open` at 208 took over 900 s. A `timeout` that kills `open` leaves no session, so run it with a long limit.
