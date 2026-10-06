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
