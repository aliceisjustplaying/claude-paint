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
- Piles mostly of lead white, laid in broad bands, came out paler and higher in key than wanted. Deeper passages needed far less white.
- A dark earth pile with a little white, laid as body and then blended, came out lighter and redder than the same pile left unblended beside it.
- A dark body pile with some white over a dry dark field read too light. The same pile with no white, loaded 0.9 at coverage 3.5, gave a solid dark mass.
- Small shapes filled with a pile heavy in lead white stayed the brightest things in their area and read as cut out.
## Strokes and brushes
- Body strokes planned inside a noise-shaped mask came out stroky and wormy until blended.
- `detail` work along a band 4 to 7 units wide came out as a row of dots or beads, not a line.
- A filbert 7 dragged at pressure 0.35 falling to 0.05 left thin, even, mechanical streaks.
- Short filbert 4 dashes stacked in a narrow band looked choppy and regular. Long thin strokes (60–220 units, pressure 0.15–0.45) crossing a passage read as quiet horizontal texture and hid hard seams under them.
- A rigger 2.2–2.4 with a full point, load 0.7–0.8 and pressure 0.65 falling to 0.4, laid a clean dark line over dry paint.
- A rigger 1.4 flicked up from a line at pressure 0.4–0.7 to 0 made tufts too small to read, and dark ones vanished on a dark field. A rigger 1.8 with 14–40 unit strokes, after a 3-hour wait, read.
- A flat 4.5 (load 0.9, pressure 0.9 to 0.8, `orient="across"`) laid solid narrow verticals over a tacky field.
- A rigger 1.5 in two short curved strokes, pressure rising then falling, made a small clean V-shaped mark.
- A clipped `detail` fill of thin tapering shapes 3–5 units wide came out as spikes, too thin.
- Two flat 5 strokes (load 0.6–0.8, pressure 0.5–0.7) clipped to a shape covered an unwanted pale patch cleanly.
- A pointed round 1.6 at load 0.3–0.35 and pressure 0.35 falling to 0.15, clipped, laid a faint hairline.
## Masks and edges
- Subtracting a shape from a mask doesn't protect it: the unclipped strokes of the next pass ran over it and buried it. Passing the same mask as `clip=` kept them off.
- `edge={found=0.5, soft=0.5}` on a body pass gave a serrated, spiky edge. Its strokes also stopped short of the canvas's bottom edge and left pale bumps until a clipped band was laid there.
- A rim band made as `m - m:shrink(2.2)`, limited by a side mask, also caught the shape's bottom edge and laid a pale hatched patch there.
- A pale rigger line clipped to `m:shrink(0.3)` landed as a faint stripe inside the edge, not on it.
- `above(446)` with a bare number is an error ("points: want ..."). A function or a point list works.
- A chunk that errors changes nothing and can be rerun as is.
## Blending
- Blending a whole shape after accents spread the accents across it and dragged fresh dark lines into streaks. Blending only a narrow seam with `tool="filbert 8"` kept most of a line.
- `blend` with `clip=false` on a grown mask dragged wet paint far sideways. It pulled a dark edge into the light field beside it as a fuzzy lump and lifted a dark shape.
- Clipped blends of horizontal bands left hard straight seams where the blend masks ended.
- Dotted light touches along a wet edge fused into a soft edge when a band across that edge was blended.
- Thin light streaks blended within a shrunk mask faded almost to nothing.
- A patch repainted over dry paint stayed visible as a lighter disc. Blending its edge and later a thin scumble softened it but left a halo.
## Wet into wet
- Dark hatched dashes (`hatch`, clipped) laid into a wet pale passage and then blended came out lighter and pinker than their pile, as a flat block. The same kind of dashes, unclipped and unblended with ragged ends, in a freshly laid wet passage read as broken horizontal marks.
- A white-heavy scumble (filbert 6, load 0.25, pressure 0.12–0.3, coverage 0.5) over a dry dark field went on as opaque pale blobs. Two blends spread the pale over the whole field.
- A light opaque pile scumbled with a filbert 3 went on as hot opaque patches. The same color at medium 0.3, in a pointed round 3 at load 0.35, laid a subtle line.
## Stippling
- A width-3 stipple into a wet passage came out lighter than its pile. Four hours later (setting to tacky) it was still lighter. `detail` strokes over it a day later, when it was tacky, gave the dark.
## Glazing
- Glazes at medium 0.6–0.7 over tacky or dry mid tones barely darkened them. Clipped over tacky paint, the glaze came out streaky with a lighter fringe along the clip. Blending it while wet softened the streaks.
- A glaze at medium 0.45 over a dry passage went on near-black and streaky, far stronger than those, with a hard top edge.
- A glaze at medium 0.72 over dry paint left dark blobs. Two horizontal blends while it was wet turned them into soft smudges.
- A glaze at medium 0.75 over a dry pale passage sank into the weave as crossing streaks. Two blends and then a light one (pressure 0.2–0.35) evened it without clearing it.
- A glaze at medium 0.55 over light shapes that were still setting didn't dull them. Once they were dry, a clipped `detail` fill (round 2.5, medium 0.6) changed their tone fully.
## Drying and time
- White-rich body passages were tacky about 22 hours after laying and dry a day later.
- Earth-and-umber body on a shape was tacky at 23 and 43 hours and dry by about 3.3 days.
- Broad body bands were dry after 32 hours. Dashes worked wet into them and blended were still tacky then and dry a day later.
- A dark black-and-umber body at coverage 3.5 was still tacky after 52 hours.
- Dark earth body was tacky 36 hours later. A second coat laid on the tacky one was still tacky 30 hours after that.
- Small, thick white-heavy fills were still setting or tacky about 1.3 days later and dry after 3 more days.
- `wait()` prints the new time, and `drying(x, y)` reports open, setting, tacky or dry.
## Looking and the session
- A crop can be at most 500 units wide at 2400 px. Crops 560–600 units wide were refused.
- Value mode showed values bunched in the light-to-mid range when the color view hid it. Squint and mirror views were used to check the masses.
- `bin/easel check` took about 300 s for 77–81 chunks and needs the session open. After `close` it replies "no painting open".
- `open` replays the log, so globals from earlier chunks (masks, piles) are there again in later sittings.
- Shell state doesn't carry between commands. `cd` to the studio and export the scratch variables in every call.
