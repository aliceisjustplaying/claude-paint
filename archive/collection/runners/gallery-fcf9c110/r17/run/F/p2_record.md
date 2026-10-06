# Studio record: what the paint and tools did
## Mixing piles
- A glaze (medium 0.6) that still held some lead white greyed and lightened dry saturated shapes into pale ghosts with darker rims. A solid repaint with no white fixed them.
- A shadow mix with little white and much Prussian blue and umber went on far too dark and crisp over a pale field. The shapes read as holes.
- Marks from a pile close in value to what lies under them vanish. Short strokes over a dark band of the same value disappeared, and small stippled shapes at the field's value merged into it. A darker pile and a lighter pile laid together made the texture read.
- A lighter pile with extra white, hatched over a dry mid passage, came out frosty and too pale.
## Strokes and passes
- `broad` in six soft-edged horizontal bands (coverage 2.2, fill, edge soft) gave stripes with scalloped seams.
- Zones filled with `body` from separate masks, even clipped and filled, met as hard stripes. A repaint bounded by a rectangle left a straight seam at its edge. One full-width pass graded by smoothstep masks (hatch, coverage 1.3, threshold 0.5) removed the seams.
- A repaint around subtracted shapes left haloes of the old color round each one. A clipped ring pass (coverage 2, fill) closed them but also covered their cast shadows and edges, which then had to be restated.
- A repaint mask that subtracted a thin band (3–5 units) shrunk by 0.8 let `body` strokes run over most of the band.
- An unclipped `glaze` pass along a narrow shape threw long smears into the fields beside it.
- `detail` with `clip=true` and `edge="found"` gave crisp small silhouettes. A row of near-identical small polys read as repeats, and a row of rect blocks each in a different pile read as a barcode.
- A `ribbon` stays thick along its whole length, so a receding band read as a tube. A pointed round (point 0.2–0.3), with pressure falling from about 0.55 to 0.15 over several reloads, drew a tapering thin line. Each reload showed as a slight break.
- Hatch at coverage 0.15–0.3 with a round 1.0 read as a fine, short texture. Rigger hatch 16–34 long, ramped to the tip, read as tall upright strokes.
- Hatch with an angle function fanning out from a base point, clipped to a lobed mask, built a broom-shaped mass. A mask with one sine-shaped peak made pointed flame shapes; two smaller off-center fans broadened the top. `stipple` also takes a coverage function.
## Blending
- Two clipped horizontal badger passes over wet bands gave a soft, slightly mottled gradient.
- Crossing clipped badger passes (coverage 2.2, 1.2, 1.2) over wet opaque blocks turned them into soft forms.
- 16-wide badgers crossing along wet seams left smeary zigzags. A badger at angle 0.12 left diagonal smears.
- Hard badgering of wet dark crisp shapes spread them out and left them rounded and blunt-ended.
- A width-6 clipped badger over a wet row of contrasting blocks fused it into a hazy mottle.
- A blend across a seam between wet and set paint softened the edge but kept the color step.
## Wet into wet
- Dark shapes and dark hatch laid into a wet pale passage dissolved and barely showed. The same pass a day later, on tacky paint, darkened it.
- Paint laid against a wet neighbor ate into it: a cover pass beside a wet narrow band narrowed it more than planned. A pale band painted over a wet dark edge dragged the dark into a grey marble.
- Thin dark strokes laid into wet pale patches dissolved. A light stipple went over a dark one only after the dark had rested 8 hours.
## Glazing and scumbling
- A semi-transparent blue glaze (medium 0.6, detail hand, load 0.35, two passes, then a width-6 clipped badger) over a dry pale band darkened it, more on one side where the second pass went.
- A glaze (medium 0.7, load 0.35, coverage 0.8) over a patch of over-bright marks knocked it down into one dark mass.
- Scumble at load 0.4–0.5 on thin ellipses laid opaque blocks, not veils. A seam scumble came out darker than the band above it and made a new edge. A scumble with a fading mask and threshold 0.25 left a scalloped edge.
## Stippling
- Dense stipple (width 3.2, coverage 4.5) over a union of ellipses with punched holes read as a flat blob with round holes. Clumps set along drawn branching strokes read as structure.
- Lighting a whole mass with one mask left a hard diagonal split. A clustered stipple of a middle tone across the split softened it.
- A darker stipple at normal pressure barely changed a pale one. With less white, width 2.6, coverage 3, pressure 0.55–0.8 and dips {8, 0.8, 0.5} it darkened.
- A rim stipple with drag, twist and feather read as a fuzzy halo. Pairs of dragged touches along a light, low-pressure stroke read as feathery sprays.
- Six small touches per spot read as specks. Small flat ellipses stippled in two tones read as flat clusters.
- A feathered, clustered stipple laid across the seam between two stippled masses hid the seam.
- A dense dark stipple over a region buried the small marks laid there earlier.
## Drying and time
- Lead-white-rich bands (medium 0.08) were dry after 22 hours; lower passages were tacky at 22 hours and mostly dry at 46.
- Thicker lead-white passages were tacky at 30 and 50 hours, and dry in places by 98. A lead-white patch was still tacky at 3 days and dry at 5.
- A dark passage with bone black was still tacky 5 days after it was laid.
- Light touches on tacky paint left only tiny dots. On dry paint, a wider brush laid visible marks.
- Glazes were held back until the paint under them was dry, not just tacky.
## Masks and edges
- A noise mask scaled for perspective turned too fine toward the far band and read as camouflage.
- A mask helper with a slope error put a pass in the wrong place, so the pass barely changed anything. An explicit `below()` curve fixed it.
- A keep-out mask built from the shapes in front and subtracted from each later pass protected them.
## Brushes
- A 2H pencil at pressure 0.2–0.3 barely showed in the whole view and needed a crop to see.
- A pale-loaded rigger 0.9 over a pale field laid stick-thin lines too faint to read.
- A rigger 1.1, with two strokes ramped in pressure from 0.1 to 0.55 and back, made small V marks.
- An unclipped thick vertical stroke overshot onto a neighboring shape and left a dark streak.
## Looking and the session
- Crops over 1200 px (500 units) are refused. A grid crop helped with placing coordinates, and `--mode value` and `squint` helped judge the whole.
- A line that looked zigzag and too white in a 1:1 crop read correctly in the whole view.
- Globals persisted through close and reopen. At 184 chunks, an `open` under a 600 s timeout left no session; one run with 900 s resumed.
