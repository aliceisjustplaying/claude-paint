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
- Lead white dominates a mix. Even with four parts smalt to one of white, the paint was only a mid value. Deep or mid tones need little white.
- New paint mixed to match an older passage that had been glazed over came out too light at three parts white to one of blue. One and a half to two parts white matched. Small test swatches on a dry spot found the mix.
- `print(pile)` shows a pile's parts and medium. Piles kept as globals were still usable 100 painting days later.
## Strokes and fills
- On dry paint, `work` with the detail hand, `load=1`, `pressure={0.6,0.9}` and coverage 3 to 4 laid a fully opaque fill every time.
- The same dark pile at ordinary load and pressure came out pale and mauve-gray. Its thin film looked lighter than the pile.
- Thin lines of an earth-rich dark (rigger and pointed round) came out tan, lighter than a mid blue-gray field around them. Restroking the same paths with a pile rich in bone black made them read dark.
- A broad-hand wash at coverage 2 with `fill=true` came out lighter and more opaque than planned and hid the pencil drawing. Where the bands overlapped it left a row of lumps.
- Thin pale strokes laid along the upper edge of dark lines read gray and metallic, and some looked detached from the line, with pointed ends.
- Pale ridges drawn with a pointed round (pressure 0.35 to 0.15) over a dark mass came out too bright and scratchy.
- Pairs of short dragged filbert marks along a path read as a row of pills. A light rim on the far edge made each hollow look raised.
- Short horizontal scumbled dabs (filbert 3, pressure 0.35 to 0.15) laid lighter than the pile promised.
- The body hand with `edge="soft"` over a small patch laid a flat, opaque slab with a hard lower edge, even at medium 0.55.
- Opaque rectangular repair patches showed as rectangles. A patch whose edges ran along existing shape edges left no seam.
## Wet into wet
- A dark full-load pass into a wet mid tone only lifted it, even at coverage 5 and `load=1`. The result stayed mid-gray.
- Pale over wet dark went gray and dirty. Dark over wet pale went gray, streaky and ragged-edged. Both happened again every time the underlayer was still wet.
- New paint laid over test swatches that were 20 minutes old came out lighter where it crossed them.
- A full-load dark pass over tacky paint gave a solid, continuous dark mass.
- Once the underlayer was dry, both light over dark and dark over light sat cleanly on top.
## Blending
- Two blending passes over a wet banded layer softened it but left faint bands. A stipple and then a blend left speckles in the transition, and one more blend fused them.
- A clipped blend at a mask's edge left the hard edge. Two unclipped blends across a 50-unit band dragged the wet paint over the line and softened it.
- Blending a rectangle that held a wet patch among older dry paint smeared the patch's paint into pale blurs around it.
- A blend ring around a dry patch's edge only partly hid the edge.
## Glazing
- The glaze hand isn't clipped. On a band about 50 units tall its long horizontal strokes ran well past the mask on both sides and filled the neighboring passages.
- A dark pile at medium 0.65, brushed on with the detail hand and a filbert 5 at coverage 2, was nearly opaque and flattened the texture under it. Where its mask was cut by a `rect` it left a lighter seam.
- A glaze at medium 0.75 and coverage 1.2 over a broad dry field, clipped below a curve, left a hard straight top edge. It went down in cool swirls, and it was dry within 5 days.
## Stippling and texture
- Coverage driven by noise (`mask:times(noise)` and then `work`) laid a mottle far stronger and more even than intended, like a leopard skin.
- Stippling into a wet layer and then blending fused the touches.
## Drying and time
- Lead-white-rich layers at medium 0.15 to 0.2 were mostly touch-dry in 20 hours and fully dry in 26. Smalt-rich strips were still tacky after 22 hours.
- Dark piles with little white (umber, bone black, medium 0.05 to 0.1) laid thick were setting at 6 hours and still tacky at 16 and 26 hours.
- Stacked dark passes stayed tacky for 5 days and were dry by about 10. Pale caps over dark and repeated repairs often needed 7 to 9 days.
- `drying(x,y)` at several points before crossing a passage prevented the muddying. After a wait, check again: different spots dried days apart.
- Varnish needed every point to be touch-dry. `varnish{coats=0.3}` warmed the pale passages slightly.
## Masks and edges
- `ribbon(pts, w)` takes the full width: a 100-unit ribbon of width 10 has an area of 1078.
- `m:grow()` on a shape's mask spread paint onto an adjacent shape. It had to be painted back.
- `noise{stretch=...}` takes positional values `{angle, k}`. Named keys failed with "error converting Lua nil to f32".
- A chunk that stopped with a runtime or syntax error changed nothing, as the guide says.
- `clip=true` on a body pass kept a narrow band's edge crisp.
## Brushes
- A round 4 with `point=1` stays at 0.58 wide up to pressure 0.2. It is 1.2 at 0.4, 3.0 at 0.7 and 4.9 at 1.
- A rigger 1.5 stays at 0.56 up to pressure 0.4, then 1.16 at 0.7 and 1.95 at 1. `pressure_for` a width below that minimum returns about 0.
- Short strokes with pressure falling to 0 and ramps of 0.6 drew noticeably shorter marks than their paths.
## Looking at the canvas
- A crop may be at most 1200 px per side, which is about 500 canvas units. A 1000-unit crop was refused, so use `--size 1000` for a whole view.
- `--mode squint` showed that the lower field was as light as the lightest band, which the color view had hidden. `--mode value` confirmed the dark and light masses.
- Close crops at 1:1 caught seams, streaks and wet-mixing damage that a whole view didn't show.
- Compute time: 6,200 thin strokes took 23 s, and one blend over the upper half of the canvas took 47 s.
