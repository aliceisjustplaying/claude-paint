# Stream 4: form and light for solids

Branch `form`. This stream answers fresh-painter findings 5 and 10: rocks read
as "macarons, loaves, haystacks, beetles and soap bars", mountains as "weird
sea with hard edges", and painters had to build their own height fields and
shading. The engine now gives a painting program a solid to reason about.
Each point of it has planes turned toward or away from one light, a core
shadow, reflected light, cast shadows and a distance for aerial perspective.
It also gives stroke-direction fields and masks derived from that solid.
The engine still paints nothing: the rock and range motifs live in
`paintings/src/rocks.rs` as one painter's example.

## What changed

- `crates/paint/src/form.rs` (new)
  - **`Form`** is a depth buffer over the canvas. Each pixel stores the nearest solid's `z` (units toward the viewer), its exact normal, a part id, a facet id and a distance.
  - **`Sdf`** describes 3-D bodies as signed distance functions, sphere-traced orthographically:
    - masses: `ellipsoid` and `block` (a rounded box whose faces are facets 1–6);
    - operations: `.turn(c, yaw, pitch, roll)`, `.cut(at, n, facet, round)` (a fracture plane that becomes its own facet), `.rough(amp, period, seed, ridged)` (weathering, soft lumps or pitted grain), `union` and `subtract`.
  - **`Ridge`** is a mountain face or cliff below a crest line.
    - The face leans back (`lean`) and flattens at the foot.
    - Gullies start just under the crest and follow the fall lines back to it, so they fan out from the peaks. Downhill, fine gullies merge into wider ones.
    - Spurs are rounded; strata (`strata`) and a base line (`base`: beach, valley floor) are optional.
  - **`Relief`** takes any height function.
  - **`Light`** sets the direction and how far it comes from the viewer's side. A negative `front` is contre-jour. It also sets ambient sky light, reflected light (default: from below and the far side), penumbra, reach, occluder thickness and `across_parts` (whether one part shadows another).
  - **`Shade`** gives `turn` (n·L), `direct`, `cast`, `bounce`, `sky` and a combined `value`. `lit(soft)` sorts a point into the light or shadow family with a halftone band. The core shadow falls out of the model: just past the terminator, neither direct nor reflected light reaches.
  - **Fields:**
    - `fall` (gravity projected onto the plane: the way water runs);
    - `across` (around the form);
    - `edge_angle` (along a plane break or an overlap);
    - `bend` (+ convex arris or spur, − concave joint or gully);
    - `dist`, `part`, `sample`.
  - **Masks:**
    - `mask(|s| …)`, any rule over samples;
    - `silhouette(parts, |s| soft)`, an edge whose softness varies per point (crisp near, lost in haze, softer where the form turns away);
    - `edges(turn, step, span)`, plane breaks and overlaps. The turn is measured per unit of 3-D surface distance, so the limb of a rounded form isn't taken for a break.
  - **`aerial(dist, visibility)`** gives the aerial-perspective fraction.
- `crates/paint/src/mask.rs` (additions)
  - An exact Euclidean distance transform (Felzenszwalb & Huttenlocher).
  - `distance()` (signed, units), `offset`, `dilate`, `erode`, `rim(width, soft)`, `band(lo, hi, soft)`, `soften(|x, y| width)` (re-edge with varying softness) and bilinear `sample(x, y)`.
- `lib.rs`: `pub mod form` and re-exports of `Form, Light, Ridge, Relief, Sdf, Shade, Solid`. `paintings/src/lib.rs`: `pub mod rocks`.
- `paintings/src/rocks.rs` (new, the painter's side)
  - Geometry: `ground`, `boulder` (a lumpy granite mass with five fracture planes and ridged grain), `outcrop` (stacked Elbe-sandstone beds with a recessed core showing in the joints) and `chalk_cliff` (Rügen: a near-vertical fluted face, pinnacles, flint bands, a scree foot and a farther cliff). It also has `ranges` (three receding ranges: rounder and finer in form the farther they are) and `skyline`.
  - `Stone`: colors per family (light, half, shadow, core, bounce, crevice), with presets for granite, sandstone, chalk and mountain. `Stone::at(&shade)`.
  - `Air { color, visibility }`.
  - `paint_solid` paints dark planes first and lights last, in this order:
    1. block in the whole mass in its shadow colors, thin, strokes down the planes (across on tops that face the sky);
    2. the shadow planes again, with core and reflected light;
    3. the lit planes in stiffer paint, each stroked its own way, so the breaks come out abrupt;
    4. a soft clean brush fuses each plane and the turning halftones, stopping at the breaks;
    5. dark accents along the big concave breaks (`edges` × `bend` < 0), strokes along `edge_angle`;
    6. last, a few touches of the brightest lights.
  - `paint_ranges` works far to near:
    - silhouette softness grows with aerial distance;
    - a body pass runs down the fall lines, then the lit spurs, then dark strokes in the deepest gullies (near ranges only);
    - a level mist veil at each foot is fused with the badger.
- `paintings/src/bin/study_form.rs`: the evidence sheet (below). `--only boulder|outcrop|cliff|ranges` paints one motif over the whole canvas. `--grisaille` shows the light model's values unpainted (a diagnostic).

## API, briefly

```rust
use paint::{Form, Light, Ridge, Sdf};
use paint::form::aerial;

let mut form = Form::new(c.frame());
// a boulder: a mass, turned, weathered, then broken by fracture planes
let rock = Sdf::ellipsoid(c3, [150.0, 110.0, 120.0])
    .turn(c3, 0.3, 0.1, -0.12)
    .rough(17.0, 150.0, 1, false)               // soft lumps
    .cut(top_pt, [-0.35, -1.0, 0.45], 10, 4.0)   // a sloping top: facet 10
    .cut(side_pt, [1.0, -0.15, 0.3], 11, 3.0)    // the flank away from the light
    .rough(0.9, 25.0, 2, true);                  // pitted grain
let id = form.add(&rock, 0.5);                   // dist 0.5 (aerial perspective)
let range = Ridge::new(0.0, 1000.0, |x| crest(x), 400.0, 7)
    .lean(0.9, 0.7).gullies(45.0, 0.5).base(valley_y);
let far = form.add_at(&range, &|_, _, z| 3.0 - 0.002 * z);
form.light(Light::new((-1.0, -0.7), 0.5).ambient(0.2).penumbra(0.05));

// then paint with fields from the form
let lit = form.mask(|s| if s.part == id { s.shade.lit(0.12) } else { 0.0 });
let edge = form.silhouette(&[id], |s| 0.4 + 5.0 * aerial(s.dist, 6.0).powi(2));
let hd = st.body()
    .color(|x, y| form.sample(x, y).map_or(sky(x, y), |s| stone.at(&s.shade)))
    .angle(|x, y| form.fall(x, y))
    .clip(true);
c.work(&lit.mul(&edge), &hd, 1);
let joints = form.edges(0.8, 3.0, 2.5).mul(&Mask::from_fn(f, |x, y| smoothstep(0.3, 0.7, -form.bend(x, y, 2.5))));
```

## Evidence

- `out/study_form.png` (1000px sheet, ~20 s), `out/study_form_full.png` (3200px, ~2 min), `out/study_form_gris.png` (grisaille, ~4 s). The PNGs are git-ignored; regenerate with `cargo paint study_form [-- --full | --grisaille]`.
- JPEG previews in `~/tmp/form-2c21ec6a/`: `sheet_1000.jpg`, `gris_final.jpg`, and the 3200 crops `f_boulder.jpg` and `f_ranges.jpg`.

What I saw, judged as a painter:
- **Ranges (bottom right):** the clearest win. They read as mountains lit from the left. Lit spurs alternate with shadowed gullies that fan down from the peaks. There are three layers, each paler and softer-edged in the air, with mist lying at the feet between them. There are no horizontal bands. The near range is heavy and dark, and the peaks are still a little too regular.
- **Boulder (top left):** a solid lit from the upper left, with a lit top plane (lichen), a light front, a dark flank and a soft cast shadow on the ground to the right. Its foot is sunk in the grass. It reads as a rock, not a loaf.
- **Sandstone outcrop (top right):** reads as stacked, weathered beds with dark joints, lit tops and shadowed undersides, casting shadows on the ground. It is a little too tidy and too yellow.
- **Chalk cliff (bottom left):** reads as a white fluted wall with a green brink over the sea, with a farther cliff beyond. The light and shadow on the face are subtle; the fluting could be stronger.
- **At 3200px** the big light structure holds, but the handling doesn't. Each plane is a field of uniform short dashes, and the ground specks through in the shadows (white flecks). These are fresh-painter issues 7 and 8 (coverage and "beads at 3200") showing up here. The motif passes use short strokes to follow the planes; they need longer, more varied strokes and better coverage at the edges.

## Known issues and next steps

- Handling at full size: uniform dash texture and ground specks in dark passages (see above). This belongs partly to stream 2 (stroke entropy) and partly to `paint_solid`'s pass settings.
- Cast shadows are traced in screen space over the depth buffer. Occluders more than `Light::thickness` in front are ignored, but a thin solid far in front can still shadow things far behind it. Use `across_parts(false)` for distant layers.
- `Ridge` is a bas-relief: it has no overhangs. Its fall lines are estimated from the smoothed crest, not integrated, so gullies on complex crests only roughly follow the true fall lines.
- There is no hydraulic erosion. Gullies are merged fixed-scale noise, which reads well at a distance, but a close range would benefit from a real drainage network.
- `Form` stores ~28 bytes per pixel (≈190 MB at 3200px). Build one form per motif and drop it after painting.
- Next: a `Form::simplify(radius)` normal blur (the painter's squint) so color fields can follow the big planes while texture is added on purpose; lit-rim helpers for contre-jour; snow and vegetation lying on up-facing planes (`s.shade.sky`, `s.n[1]`) as a documented pattern; and use the rocks in a real composition (the Cross in the Mountains rock, the Wanderer's crags).
- `mask.rs:106` (`mul`) and `box_rows` clippy warnings predate this stream.

## Commits

- `fa669e7` mask: signed distance (exact EDT), offset/dilate/erode, rim, band, soften, sample
- `7ea2b63` form: depth buffer of solids, Light/Shade, fields and masks; tests
- `566e039` rocks: stone and range painting passes; study_form sheet + grisaille; Ridge fixes
- `50c3dc0` rocks: plane fusing, accents only at big breaks, seated boulder, ranges lighting and mist
- (final) clippy fixes, notes
