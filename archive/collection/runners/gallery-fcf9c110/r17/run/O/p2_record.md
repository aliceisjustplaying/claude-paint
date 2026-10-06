# Studio record: what the paint and tools did
## Mixing piles
- Body piles with as much lead white as all their other tubes together read pale, even where the pile was meant as the dark of a passage. Only a pile with a trace of white (about 2%) at coverage 3.5 and load 0.9 gave a solid dark.
- A narrow band filled with an earth pile holding about 12% white came out lighter and pinker than the dry dark around it. A second coat at 15% white laid while the first was wet was still light. Once the band was tacky, a coat with less white and more black matched the dark.
- A thin line from a dark pile with some white (0.6 part in 3) came out too pale over dry dark paint. A pile of black and umber alone laid a dark line.
## Strokes and brushes
- `broad` at coverage 2.5, clipped, left gaps with the ground showing in blotches. `body` with a filbert 14 at coverage 3 and `fill=true` covered.
- Single flat 3.5–5 strokes of a pale pile at low load (0.3–0.6) and light pressure (0.15–0.45) came out as even, ruled stripes.
- A flat pressed at 0.9 with `orient="across"` laid a short solid bar with rounded ends.
- A flat on a wavering path, pressure 0.7 falling to 0.1 (ramps 0.05, 0.6), laid a tapering bar. Two such strokes end to end over dry paint (load 0.75, then 0.45) left a lighter gap between them.
- A rigger 1.4 at pressure 0.3 laid a hairline. A rigger 1.5 in two short curved strokes (0.1 to 0.6, then 0.6 to 0.1) made a small clean V.
- A round 2.2 (point 0.6) at pressure 0.7 laid clean dark lines over tacky paint.
- Short flat 2.2 strokes of a white pile, pressure 0.25–0.45 falling to 0.1, made small lozenge-shaped marks.
- A round 2.5 of a pale pile (medium 0.2, load 0.35, pressure 0.1 to 0.35) laid a crisp, too-white line over dry dark. One pressed flat 4 stroke of the dark over it hid it.
- A clipped `detail` fill (flat 4, coverage 4) of a dark pile with a little white came out mid-grey over dry paint. A second coat with no white at load 1 went darker.
## Blending
- Three clipped band passes followed by a clipped `blend` left hard seams where the bands met. While the paint was wet, unclipped blends near vertical (angle about 1.5, strokes 30–70 units) across the seams removed them.
- A blend at angle 1.3 (and later 0.9) over a wet body passage combed it into parallel streaks. A horizontal blend over it while wet took them out.
- Thin roughened ellipses filled clipped read as crisp cut-out bars. A horizontal blend over the wet area fused them into the field.
- A dark pass blended over a wider wet area lost most of its dark into the lighter paint around it.
- `blend(m:grow(15), {clip=true})` left the lower edge of a wet mass hard and dripping. Blends along a rim band softened it only partly.
- An unclipped badger 24 over an area part wet and part dry dragged the wet paint into swirls and washed out small light accents.
- An unclipped badger across the line between two wet passages smeared each into the other. An unclipped horizontal blend whose area reached a nearby dark shape smeared its end.
- A blend can't move dry paint: where it met dry paint a hard edge stayed. A band of an in-between pile laid across the edge and blended while wet hid it.
- A blend mask's edge inside a wet passage left a faint line. A second, narrower blend over the line removed it.
- A patch repainted next to dry paint and blended clipped around itself stayed visible as a lighter rectangle.
## Wet into wet
- A body pass into a wet pale field came out paler and pinker than its pile. Small dark `detail` strokes into a wet pale field also came out pale.
- A strip laid wet against a wet neighbor came up dirty and streaked.
- A dark coat over a wet layer lifted it and came out mid-dark grey. A pale coat over a wet dark came out streaky.
- Filbert 3 strokes of a dark pile dragged into a wet pale shape came out lighter than their pile.
- A darker `detail` fill into a wet pale shape, then a badger 6 blend, barely darkened it. The same fill laid once the pale had set (5 hours) held its value.
## Glazing
- `hand="glaze"` at medium 0.7 (coverage 1.6, pressure 0.2–0.35, clipped) over a dry dark area darkened it slightly, with faint streaks.
- Two clipped glaze passes with a filbert 6 at medium 0.8 over dry pale shapes darkened them unevenly, in streaks.
- A glaze with a soft filbert 10 (medium 0.75, `edge="lost"`) over paint laid minutes before dragged it into horizontal streaks.
- A `detail` pass at medium 0.3 over dry dark, then blended, came out lighter than the dark around it.
## Masks and edges
- Small separate lobes painted along an edge read as detached islands. One continuous new edge painted over them joined them.
- Clipped fills on the two sides of a redrawn boundary left lighter patchy triangles along it.
- Small smoothed `poly` lens shapes filled clipped with a pale pile read as bright pills with hard edges.
- `lose` with a filbert 3 (reach {4,5}, load 0.3, where 0.6) laid 109 strokes, covered much of the small shapes and left a lighter halo around them against a glazed surround.
- `edge="soft"` on a clipped band made its lower edge dissolve into the dark below it.
- A mask term multiplied by 0 gives a number, not a mask: "want a mask, got integer". The chunk changed nothing.
## Drying and time
- White-rich bands were still open about 4 hours after laying. One band was setting.
- The first body layer was tacky at 36 hours and dry at 66. A second layer was tacky at 26 hours and dry at 50.
- Passages laid on one day were all tacky at 30 hours. At 60 hours some were dry and others tacky. At 96 hours only an umber and black body (coverage 3.2, load 0.9) was still tacky.
- A clipped `detail` fill of black and umber with no white (coverage 4, load 1) laid over dry paint was tacky at 50 and 90 hours and dry at 162.
- A white-rich scumble with a body pass over it was still tacky at 40 and 70 hours.
- Small pale body fills were tacky at 30 and 54 hours and dry at 126. A narrow band of two wet coats of umber-rich paint was tacky at 24 and 48 hours.
## Looking at the canvas
- A 500-unit crop is 1200 px and works. Crops 560–720 units wide were refused. `--size 1400` gives a larger whole view.
- Value mode showed values bunched pale. Squint showed the masses. `--grid` gave coordinates.
- `bin/easel check` took 338 s for 95 chunks and 420 s for 120. `open` replays the log, so earlier globals come back.
