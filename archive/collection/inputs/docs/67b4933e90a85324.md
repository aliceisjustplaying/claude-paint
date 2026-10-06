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


## More notes from the studio

# Studio record: what the paint and tools did
## Mixing piles
- A mid-dark pile with about as much lead white as umber, laid on a dark passage, came out as a pale disc, far lighter than the dark around it.
- A pile for shallow hollows with about 2.6 parts white barely differed from a pale field mixed at 3.6 to 5 parts white. Cutting the white to 1 part made it read darker.
- Piles that were mostly lead white kept the whole lay-in pale and close in value. The value and squint views showed the upper and lower fields at nearly the same value.
- A pile defined as `local` in one chunk was gone in the next: `b:load` failed with "needs a pile", and the chunk changed nothing. `rng()` doesn't exist, and that chunk failed cleanly too.
## Strokes and fills
- A broad-hand pass (coverage 2, `fill=true`) in horizontal bands left half of one band thin, with the pencil showing, and the other half blotchy. A body-hand pass over it (strokes 40 to 90 long, coverage 2.5) evened it.
- Unclipped body strokes near a mask's edge threw a thin stray arm of paint out past the shape.
- The detail hand at `load=1`, pressure 0.6 to 0.9, coverage 3.5 to 4 and `fill=true` laid opaque fills of small shapes every time.
- A band built from half-ellipse lumps read as regular domes with hard square bottoms. Many small overlapping ellipses of mixed sizes, roughened, gave a broken, irregular top edge.
- A pointed round (width 3, point 1), with pressure running from `pressure_for(width)` to near 0, laid about 4,000 tapering lines in 2.3 s of compute. Lines grown long this way read as long and flowing.
- About 1,500 short two-part strokes (2.5 to 6 units, then 2 to 5), with pressure falling to 0.02, read as short crooked spurs.
- Dark strokes that began clear of the lines around them looked like they were floating. A ribbon joining each one to the nearest thicker line fixed that.
- Thin dark lines drawn at load 0.5 across a dark shape came out lighter than the shape. Going over them with the shape's own pile at coverage 5 and load 1 hid them.
- Pale ribbons along the upper edges of dark lines became the lightest marks on the canvas.
- Pale strokes along the tops of 70% of the tiers in a small stacked shape swamped its lower half. Dark strokes along the undersides, laid once the pale strokes had dried, brought the contrast back.
- Closely spaced small ellipses along a path read as a chain of pills. A roughened ribbon filled over them read as a pale streak.
- Rigger clumps (width 1.6, pressure 0.45 to 0.7 falling to 0) scattered evenly over a field read as a spotted pattern.
## Wet into wet
- Pale paint laid over a dark passage that was still setting or tacky came out dirty gray. Laid again once the dark was dry, it stayed clean.
- A stipple of lighter grains into a wet dark body mostly disappeared. Only a few light blobs were left.
- A dark pile over wet mid-tone hollows partly lifted the paint under it, so they darkened only a little.
- Thin dark crescents covered with a mid pile and blended with a badger 6 left a light lower lip and a gradient. The hollows read as raised.
## Blending
- Four wet horizontal bands and then two badger passes at angle 0 blended smoothly, with a slight mottle left. The two passes over about 60% of the canvas took 46 s.
- A clipped body pass with `fill=true` ended in a hard straight lower edge. A stipple with coverage ramped down by a function and `feather=0.8`, then a denser stipple (up to 2.0) and a blend while wet, softened the edge.
- Wide, flat ellipses of pale paint blended with a badger 8 dried as flat discs, lighter than the field around them.
- Three fresh bands laid across the full width and blended covered an unwanted patch without a seam.
## Glazing
- A clipped glaze-hand pass (medium 0.75, coverage 1.2) over a dry pale field went down as an opaque blue-violet block with swirls.
- At medium 0.92, `load=0.25` and pressure 0.2 to 0.35, the same hand left only a few starved streaks.
- A pale semi-transparent blue pile at medium 0.85, `load=0.5` and pressure 0.3 to 0.5, blended afterward, laid an even cool veil that was a little blotchy.
- Over a whole lower field, a `load_at` ramp from 0.04 to about 0.64 with depth, then a blend, darkened the field gradually with no visible top edge. It left soft cool swirls. A second pass after drying deepened the bottom band.
## Lifting
- A clean flat 8 with `wipe(1)` before every stroke (pressure 0.8, three horizontal passes) lifted most of a wet glaze. Streaks stayed in the weave hollows until crossing vertical and horizontal passes with a flat 6 cleared nearly all of them. The brush's `fullness()` afterward was 0.015. Lifting also took color from thin lines under the glaze, and they turned pale.
- Lifting a wet mid patch off dry dark paint left a lighter rectangle, which had to be covered once dry.
## Stippling
- A stipple (width 2) along the rim of a band read as speckles, like bare ground showing through.
- A pale stipple at medium 0.3 (width 2.2, `feather=0.8`, coverage falling off over an ellipse and scaled 1.4 to 2.5) over dry thin lines veiled them to faint tips.
- A light stipple at coverage 0.35 and width 1.4 on a dark field read as a uniform dust.
## Drying and time
- A lead-white-rich lay-in at medium 0.12 to 0.15 was dry in thin areas at 30 hours and still tacky in fuller ones. All of it was dry by 44 hours.
- Dark piles (umber and black, medium 0.06 to 0.1): setting at 16 hours and tacky at 5 days. The trunk-sized passes were dry by about 9 days. Stacked dark passes were still tacky in places at 8 to 10 days, while points a few units away were already dry.
- A light body pass of mostly white was setting after a few hours and dry after 30.
- The veil glazes at medium 0.85 were dry within 5 days.
- `varnish{coats=0.3}` after a 10-day wait warmed the whole canvas and pulled it together.
## Masks and edges
- A curve passed to `above()`/`below()` that stopped at x=620 painted a rectangle beyond its end, out to x 700. Every curve must span the full canvas width.
- A mask that wasn't intersected with the lower field ran above the dividing edge into the upper field. A small wedge restated the edge.
- Subtracting grown masks (`m:grow(0.5 to 1)`) of the finished shapes kept later passes off them.
- `mask(function)` comparing `S:at(x, y)` with `S:at(x, y - d)` picked out a thin top crescent of each shape in a mask.
## Looking at the canvas
- A crop 640 units wide (1536 px) was refused. 500 units is the limit.
- Value plus squint showed only the darkest masses standing out from a field of nearly equal lights.
- A whole view caught the stray rectangle. A close crop of a distant band looked rocky, but it read correctly at full size.
- `bin/easel check` replayed 97 chunks in 293 s and matched the canvas exactly.
- A 2H pencil at pressure 0.2 to 0.35 drew very faint lines, which showed through wherever the first paint was thin.
