# Pastel

Added on the `degas` branch (from `split-6-hold-knife`) for a painting in
Degas's manner of the mid-1880s: pastel over peinture à l'essence on toned
paper. The materials it models are in notes/degas-materials.md (a
materials-only research note). The painter's side is in the guide, under
"Drawing: pencil, chalk, pastel and eraser".

## What it is

Pastel is a new `graphite::Medium`, `Pastel`, on the drawing that pencils
and chalk already lay (`graphite::Drawing`): a dry deposit written into the
dry picture, so paint laid later composites over it and seals it.

- **Color.** A drawing cell's flake reflectance is now RGB (`Cell::r`,
  `Lead::flake`). Graphite and chalk set all three channels to their old gray
  and do the same arithmetic per channel, so their results are unchanged.
  `Lead::pastel(masstone, soft)` takes a pile's masstone and makes it dry
  (`dry_color`: paler and slightly grayer than in oil).
- **Layering.** Graphite only fills what is still bare, up to a cap. A
  pastel stroke covers a share `q` of the whole pixel, earlier pastel
  included, so colors laid across each other mix by coverage. A stick drags
  over each point along its contact, so one stroke deposits
  `1 - (1 - dep)^2`. The stick crumbles less than its grain suggests.
- **Tooth.** `Cell::fill` (0..1) is how full the tooth is. A stroke lays
  `dep * (1 - fill)^1.5` and adds `(0.05 + 0.06 soft) * q * (0.6 + 0.6 p)`
  to the fill. `fix` multiplies the fill by 0.4 (it gives back tooth),
  darkens the layer by 3% and still binds it against the eraser.
- **Edges.** The stick presses less toward the edge of its contact
  (`rim = 1 - 0.9 e²`, with `e` 0 on the line's middle and 1 at its edge),
  so a mark's edge rides on the tops of the tooth and breaks up.
  Graphite is unchanged.
- **Side.** `Lead::side(width_mm)` is the stick laid flat: that wide,
  biting 0.45 as deep, laying 0.85 as much, and not wearing.
- **Smudge.** `Canvas::smudge(mask, strength, radius)` drags loose pastel
  within `radius` units together: its colors average, weighted by the loose
  coverage. It also presses the pastel into the hollows (up to 0.97 coverage)
  and packs the tooth. Fixed, painted-over and wet pastel doesn't move.
- **Serialization.** A drawing without pastel keeps its old layout: five
  floats a cell, checkpoint flag 1. With pastel (`Drawing::color`) it is eight
  floats a cell (`r, g, b` and `fill`), flag 2.

## Lua

- `pastel(pile, {soft=0.7, point=})`: `point` (mm) makes a pastel pencil.
- `p:line`, `p:sketch`, `p:hatch`, `p:rule`, as for a pencil.
- `p:side(pts, {width=, pressure=})`.
- `smudge(mask or pts, {strength=, width=, reach=})`.

Also new, and for pencils as well: `hatch{graded=true}` (in the engine,
`graphite::hatch_marks_graded`). The mask becomes a weight: strokes run
wherever it is above 0.04, and each stroke is pressed at every point in
proportion to the mask's value there. A hatched passage then fades into the
next instead of stopping at a line. `hatch_marks` (ungraded) is unchanged.

## What the painting taught (tuning)

- The first numbers laid black as mid-gray: one deposit per pixel, with
  heavy crumbling. Raising the bite and rate, the two-pass drag and the
  higher cap fixed that.
- With strong bite, marks were smooth round-ended tubes. The edge falloff
  breaks them up.
- The darkest a matte pastel black reaches is about sRGB 0.27. That is the
  4% first-surface veil (`canvas::haze`) and the right physics. A black
  passage reads by its neighbours, not by being darker.
- The tooth filled too fast at first (0.10 + 0.14 soft per stroke, fixative
  keeping 45% of the fill). After a few dense passes nothing more took: eyes,
  nostril and lids drawn late didn't show. The current numbers allow several
  layers, and fixative restores more of the tooth.
- Pastel skips wet paint. Thick touches of blotted essence (bone black,
  vermilion) were still "setting" two painted days later and showed through
  as specks. Let essence dry about three weeks before pastel.
- A paper with tooth: a knifed board ground with a rolled absorbent layer on
  top (texture 1.0). A brushed layer, or a roller over bare linen, is too
  smooth or shows the weave's grid.

## Engine 6: the stick (physical)

Sources: notes/research/paper_surface.md, pastel_stick_tribology.md,
dry_pigment_optics.md (materials and physics research; every number there is
tagged measured, derived or estimated). Engine 5's pastel above replays as it
was for older logs; a painting begun with engine 6 gets this one.

- **Paper** (`paper.rs`): fibres laid as a Poisson fibre network (lognormal
  lengths, uniform angles, mass laid along each fibre into the pixels it
  crosses), a share of them in flocs (Neyman–Scott clusters), thinned over a
  laid mould's wires; the surface follows the fibre count to a power below one
  (pressing), plus a felt's cells and calendering. Under each pixel the pores'
  depths below the top envelope are exponential with a mean about the fibre
  thickness (µ, `Canvas::micro`). A paint film thicker than µ brings its own
  micro-roughness, which follows its gloss.
- **Contact** (`pastel.rs`): the stick is a solid (round, square or a
  sharpened core) cut by the facets it has worn; its underside over each
  pixel is a ray cast up through it. It sinks until the force is carried. At
  each pixel the sheet gives as a Winkler spring (z modulus over caliper)
  while the pastel yields plastically at σp on the material ratio of the
  micro-relief it meets (1 − e^(−o/µ)): the two carry the same pressure.
- **Deposit**: Archard, ΔV = K × (material ratio met) × (distance slid) per
  pixel. Only bare paper (or paint) and fixed pastel file the stick; loose
  pastel, which fills the pores from the bottom, is a third body. Volume
  becomes covered area as particles about 6 µm thick (Poisson coverage);
  colours layer by coverage, the last on top.
- **Wear**: the volume abraded comes off the stick flat against the paper,
  on the face within 20 µm of its lowest point: a facet within 4° of one it
  has is worn further, else a new one is cut.
- **Fixative**: loose becomes bound (a crust filed at half the rate of
  paper); the particles' scattering falls by the share the resin wets
  (Kubelka–Munk: K/S rises), the coverage floor is set.
- **The finger** (`Canvas::rub`): a pad (135 mm² × F^0.4) picks up 20% of
  the loose pastel under it per pass and lays 25% of what it carries, as
  grains a third as thick (pressed in, they cover more).
- **Dry colour** (`pastel::dry_color`): two-constant Kubelka–Munk from each
  tube's oil masstone (K/S) and oil scattering, the scattering raised to its
  dry value by the Mie ratio for the pigment's refractive index (n ≥ 1.9), or
  set from the dry Mie scattering itself below that.
- **The hand**: a stroke's force rises from nothing over 50 ms of travel and
  falls over 40 ms (the 2–5 Hz bandwidth of a hand's force); deposit per mm
  doesn't depend on speed (Archard), only hand time does.

Estimates in use (no measurement of pastel itself exists): σp from 30 MPa
(hard) to 2 MPa (very soft), K from 0.01 to 0.06, particle thickness 6 µm,
crust roughness 3 µm, crust wear 0.5, the finger's 20%/25%, the felt's cells
(0.8 mm, 20 µm), the resin wetting 15% of the particles. The research notes
end with bench measurements that would replace them.

## Engine 7: crumbs, dry blacks, the eraser (2026-10-05, the dandiya painting)

Three changes found while painting; each is gated to engine 7, so engine-6
logs replay as they were.

- **Crumbs** (`pastel::SHED`, `lay_crumbs`). Engine 6 laid all the pastel a
  contact abrades on the pixels in contact, so the paper's hollows never took
  any: two heavy passes of a soft stick covered about 30–60 % and the tooth
  then read "full". Measured (Archambault, in pastel_stick_tribology.md): a
  stroke covers about 6.6 times its real contact area, the debris spreading,
  fragmenting and collecting in the texture. Now 1/6.6 of what a contact
  abrades stays there; the rest rides under the face as crumbs and settles
  (1 − e^(−ds/3 mm) a step) into the pixels under the face that lie within a
  crumb's size (150 µm) of it, the deeper ones taking more. Each crumb is one
  agglomerate, its area drawn from the measured P(A) ∝ A^(−3/2) (cut-off 80–300
  µm by softness and force), pressed flat to PARTICLE_UM like the contact's
  own deposit; what is still carried at lift-off drops there. Result: two
  heavy passes cover about 90 %, a light stroke stays broken, and the
  crumbs are discrete specks, not a veil.
- **Dry carbon blacks** (`pastel::carbon_ratio`). For pigments below n 1.9
  engine 6 sets the dry scattering to dry white's, while their absorption
  comes from their small oil scattering: dry bone black came out sRGB ~160,
  as light as a tan paper. A black's colour is its carbon, on the same
  particles: bone black now takes the measured air/oil ratio of its apatite
  matrix (×9 at 1 µm, dry_pigment_optics.md table 2.2) and vine black the
  carbon ratio (×2.4, §2.4: "a little greyer, never pale"); pure dry bone
  black is now about sRGB 90. The other low-index pigments (ultramarine,
  Prussian blue, the lakes, viridian) still dry very pale (the research's own
  worked example for ultramarine agrees); left as they were, an open
  question.
- **The eraser lifts the particles** (`Canvas::erase`). It lowered pastel's
  coverage but left its volume in the tooth, so an erased patch stayed
  "full" and refused new pastel. Now the loose volume falls with the
  coverage taken.

Also learned (no change made): a paint film thicker than the paper's pore
depth has the paint's micro-roughness (0.3–2 µm for lean paint, as the
research gives), so pastel over a heavy essence lay-in fills its tooth in one
light pass. Keep essence thin where pastel will go, or leave the paper bare.
