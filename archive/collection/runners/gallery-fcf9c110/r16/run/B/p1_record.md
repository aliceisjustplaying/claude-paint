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
