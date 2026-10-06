# Studio record: what the paint and tools did

## Mixing piles
- Neighboring piles of 3 to 4 parts lead white with small amounts of blue and earth laid down as nearly the same flat pale gray.
- A glaze pile of umber, bone black and red earth at medium 0.7 over a dry dark layer went on too warm and too saturated. The same glaze with cobalt in place of red earth (medium 0.65 to 0.7) darkened the layer and stayed cool.
- A patch of a mid pile painted over and beside partly wet dark paint came out paler and whiter than planned.
## Strokes and passes
- `work` in wide horizontal bands (filbert 14, coverage 2.5, `fill=true`) left the bands as separate stripes with the ground showing between strokes. Only blending fused them.
- `detail` strokes (round 4, 6 to 16 units, steep angle) along a mask with a jagged top turned every bump into a sharp spike.
- A `body` pass with vertical strokes on a soft mask (`threshold=0.4`) laid crude vertical streaks.
- Low-load dabs from a flat 3 to 5 (`load_at` 0.25 to 0.65, coverage 0.9) in three piles over a dry layer read as a cobbled patchwork until a horizontal blend fused them.
- Hand-placed horizontal flat strokes of one width across a field read as stacked slabs. Widths and spacing that grew toward the bottom read as a flat plane receding.
- Thin pale strokes (flat 0.9 to 3, load 0.2 to 0.6, pressure falling to 0.05) on a dry dark or mid field stayed crisp and stark, like dashes or painted lines.
- A flat stroke pressed to 0.8 to 0.9 with default ramps gave a vertical bar with rounded ends. `ramps={0.02, 0.02}` on a stiffer flat (0.7) squared the ends, and a light dragged touch across the top flattened it further.
- Reflection strokes along sine-wave paths read as wriggles.
- A rigger (point 1) pressed 0.9 to 0.4 drew a clean thin line. At 0.1 to 0.45 in two short strokes it made small V marks.
- Strokes offset along both sides of a polyline edge (a dark band outside, a pale line inside) looked piped, like icing.
- `work` rejects a function for `coverage` ("expected number"). `load_at` takes one.
## Blending
- Two full-field blends at crossing angles (0 and 0.12 to 0.25) of a wet banded layer gave a soft gradient and dissolved narrow lobed shapes laid wet into it into soft streaks.
- A blend inside a rectangle over fresh vertical strokes smeared them into a hard-edged block that also spread over dry paint beside them.
- Blending a wet dark to light gradient with a pale rim modelled a smooth rounded form.
- A blend mask grown and softened around shapes on a dry field softened their edges but not their outlines, and pulled wet paint in from a tacky neighbor.
- A narrow badger (14) in a ring along an edge softened it but smeared wet marks nearby and left a patch of pale lattice.
- A soft rectangle blend (badger 20) over thin wet strokes on a dry field took most of them away.
- A blend mask shaped like the passage itself (grown 4, softened 4) fused broken strokes into a mottled mass with no rectangle edge. Paint lifted off the weave tops left a pale lattice.
- A narrow badger (16) over crisp strokes on a dry field sank them into the layer.
## Wet into wet
- Dark strokes into a setting or tacky pale field came out lighter and streaky, left gray lifted patches and once a ring where an edge was disturbed. A second dark pass did not cover them.
- A pale fill painted up against a dry edge left a paler strip along the join.
- Dark horizontal strokes into a fresh pale band, then one horizontal blend, gave a soft hazy dark mass.
## Glazing and scumbling
- `hand="glaze"` clipped to its mask, then two blends, darkened a dry area and left a fine crackle-like texture visible close up.
- A `scumble` (medium 0.3 to 0.35, load 0.35, coverage 1) went on as an opaque block with a scalloped edge. Three blends over the whole area dragged it into a light-to-dark gradient.
- A `body` pass of a white-rich pile at medium 0.35 (load 0.5, coverage 1.5) then two badger 30 blends in a softened ellipse gave a subtle soft brightening with a faint edge at one end.
## Stippling and touches
- A stipple (width 2.4, coverage 0.9, feather 0.4) on a mask of a shape's upper-left rim showed as a warmer broken edge.
- A stipple on `ellipse:rim(4, 2) * mask:grow(3)` made no visible change. About 40 round-brush touches placed around the ellipse did reshape its edge into small lobes.
- Small stippled clumps of one size read as uniform specks. Larger ones with `drag` read as blotches.
## Drying and time
- A lead-white-rich lay-in was setting 4 h after it went on, mostly tacky at 6 h, still tacky at 25 h and dry at about 55 h.
- Umber and bone black paint was still tacky 3 days after it was laid and dry by 5.5 days. A bone black mix laid in body was tacky at 36 h and dry at 84 h.
- Small dark detail touches were tacky at 40 and 61 h and dry by 110 h. A glaze at medium 0.7 was dry within 72 h.
- After 120 days every point sampled on a grid was dry. `varnish{coats=0.3, vary=0.08}` then warmed and unified the canvas.
## Masks and edges
- A union of roughened ellipses gave a lobed silhouette. Where the ellipses and a base strip failed to overlap, a gap stayed unpainted and later passes clipped around it showed through.
- A smoothed `poly` made rounded domes. Straight `poly` edges read as ruled lines.
- Lens masks built from sine profiles came out as sausage shapes with even hard edges. Clipped fills of them (`clip=true`) looked cut out and pasted on.
- Thresholded stretched noise showed only where the noise crossed the threshold, sometimes on one side of the canvas only.
- `noise{stretch={angle=, k=}}` errors ("nil to f32"). `stretch={angle, k}` works.
- Clipping a pass to a field minus a dark shape (`- mask:grow(0.5)`) kept that shape's edge crisp.
## Looking
- Crops are 2.4 px per unit, so a crop wider than 500 units fails. A 500-unit crop showed smears, rings and lattice patches the whole view hid.
- `--mode mirror` showed a hard edge the normal view hid. `--mode value,squint` showed a value range too narrow to see in color.
- Two whole-field blends took 39 to 45 s. `check` replayed 80 chunks in 399 s.
