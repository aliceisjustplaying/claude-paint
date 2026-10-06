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
