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
