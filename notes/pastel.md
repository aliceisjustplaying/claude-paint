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


## Engine 7 tools (2026-10-05, after the dandiya)

Tools only where each is a physical process modelled from the research,
no undo-like conveniences. Paint's `sheet.rs`:

- **Stick hold** (`look --hold <stick> --at x,y [--pose f,alt,az[,roll] |
  --side dir]`, main.rs `stick_hold_look`). Reads only. It calls the stroke's
  own seating (`Canvas::seat_stick`, factored out of `stick_stroke` with the
  arithmetic unchanged, so replays are identical) through
  `Canvas::stick_contact`: where the stick touches at that force, and where
  its face lies within a crumb's size of the surface.
- **Paper mask** (`lay_sheet(mask, {grammage=, tone=})`, `lift_sheet()`). A
  raised surface `caliper` µm thick (the paper physics' caliper for that
  grammage) with the support's mean pore depth: `Canvas::surface_point`
  feeds the contact solver, so the stick bridges its edge by its own geometry.
  What lands on it (contact deposit and crumbs, by the crumb bed's depths) is
  the sheet's (`sheet_catch`), shown in the look and gone at lift. The
  finger, eraser and fixative skip it. Wet-paint and graphite verbs refuse
  while it is down (`time::verb_dry` marks the verbs modelled for it); a
  chunk must lift what it laid (session.rs).
- **Blade on pastel** (`k:scrape`, `Canvas::scrape_pastel`). The knife's
  geometry: the blade rests on the highest point within 4 mm of steel flex
  and stands off by 300 µm·(1−p)^1.5. Pastel fills the pores (mean depth µ)
  from the bottom; v − µ is heaped above the envelope. The blade takes what
  stands above its plane, loose first, then the crust; coverage follows the
  volume as a deposit's (Poisson, PARTICLE_UM). No paper damage modelled.
- **Dry brush** (`b:dust`, `Canvas::dust_pastel`;
  notes/research/pastel_brushing.md). Force is not the limit (a tip pushes
  10³–10⁶ times the rolling force of a particle): reach is. Tip radius r
  (hog 50 µm, the research's 25–75) over inter-fibre gaps of width w (the
  paper's fibre width) sinks h = r − √(r² − w²/4); with pores of exponential
  depth (mean µ) filled from the bottom, the pastel deeper than h is
  µ·e^(−h/µ), and that stays, with all fixed pastel. Lifted pastel: half
  carried and let go over 3 mm downstream, half pushed ahead and dropped
  within 0.5 mm; the brush meets it again and sweeps it on, so it ends as a
  darker lip where the brush lifts, the ghost behind (estimates: 30–70 %, 1–5 mm, 0.1–1 mm; no measurement exists;
  the research note's bench test would set them). The tips' band is the
  brush's mark width across and 2 mm along.

Not built, and why: wet pastel and steam (the deposit and consolidation of a
water-softened pastel compact and its gum are unmeasured; only the water
transport could be modelled), pastel paste/gouache (a water-borne binder
drying by evaporation to a porous film: physics known, a large job).

### Engine 7 tools, second pass (2026-10-06; notes/research/pastel_removal_air_blade.md)

The brush, the blade and the loose dust made more physical. Paint's
sheet.rs:

- **The knife sinks** (`edge_sink_um`): a cylindrical edge (radius `edge`,
  100 µm by default) on the sheet as a bed of springs with Chen et al.'s
  (2020) compression curve σ = 0.636(e^(13.54ε) − 1) MPa, thickness the
  paper's caliper; the hand's line load 0.05–0.5 N/mm by pressure. It takes
  the loose pastel above that depth (35 % dropped where it lifts, an
  estimate within the research's 20–50 %; the rest carried off), and
  burnishes: the residual strain 0.49ε − 0.027 lowers the pixel's height and
  its pore depth µ by the dent (the voids it closed), once per pixel. The
  Winkler depth ignores the fibres' in-plane stiffness, so it is an upper
  bound: a firm scrape strips the loose pastel to the paper. Fixed pastel
  stays; nap raising and cutting (a scalpel) are not modelled.
- **The brush fills** (`Held::pastel_mg`): it keeps what it lifts in
  proportion to its room (capacity 0.06 mg per mm³ of a 2 mm loaded tip,
  width × 2 × 0.3·width: a fur brush's pile holds 5–7 % of its volume in
  toner); the rest is pushed 0.5 mm ahead as crumbs (lay_crumbs, up to
  100 µm) and met again, so it ends as a ridge at the lift. `b:wipe` empties it.
- **blow(x, y, {distance, speed, nozzle})**: Phares et al.'s peak wall shear
  of an impinging jet, τ = 44.6ρU²Re^(−½)(h/d)^(−2) (h/d ≥ 6), on a ring at
  0.09h, rising linearly inside it and falling as r^(−2.3) outside (a radial
  wall jet). Heaped loose crumbs go past Shao & Lu's threshold for 100 µm
  crumbs of 1250 kg/m³ (~0.21 m/s), on a ramp (the shear fluctuates); in the
  pores the shear dies as exp(−4.21 z/w) (Moffatt), w the fibre width, so
  the fine grains (Shao & Lu at 5 µm, ~0.8 m/s) go down to z = (w/2.1)·ln(u*/u*t).
  Redeposition (0–20 %) left out.
- **tap({g})**: crumbs heaped on the tooth with m·a > F_adh fall (F_adh 50
  nN, crumbs 1250 kg/m³: at 100 g, d_c ≈ 33 µm; at 1 g, ~150 µm); with
  crumb sizes P(A) ∝ A^(−3/2) up to 300 µm the falling share of the heap is
  (A_max − A_c)/(A_max − A_min). Pore-held grains and fixed pastel stay.

Still flat: a brushed ghost is an even tone, because the paper model gives
every pixel the same pore depth (µ = the fibre thickness); letting µ vary
with the local fibre count would need its own derivation.

### Third pass (2026-10-06; notes/research/paper_mechanics_and_transport.md)

The brief: "make it right, more physical"; and replay exactness only matters
for master, so engine 7 changed in place.

- **Pores per pixel** (paper.rs): Dodson's mean surface pore height
  t·ε/(1−ε) at the local porosity (the solid share following the fibre count
  to the power 1 − press: denser spots shallower), times a Gamma(6)/6
  scatter for the few dozen pores a pixel holds (k ≈ 2–12, the middle).
  The sheet's mean stays the fibre thickness at ε = 0.5. A brushed ghost
  now has the paper's grain.
- **The knife's dent** (`edge_sink_um`): a Pasternak layer, p = σ(w/T) −
  (G_xz·T/3)w″ with Chen's σ and G_xz = 30 MPa (15–50 [E]; 16–127 measured
  in board), solved on a grid with an active set for the contact and a
  bisection on the depth: 0.17–0.8 of the bed of springs at a knife's light
  loads, as the research's numerical solution. The load is the hand's 1–5 N
  over the edge's length (no longer a fixed N/mm), so a 3 cm palette knife
  presses lightly (no permanent dent: Chen's residual strain is 0 below
  ε ≈ 0.055) and a 2–3 mm edge hard (burnishes).
- **Nap**: drag per fibre crossed, μ·q·w_f (μ 0.3, w_f the fibre width),
  past a joint's strength (3 mN; 1.1–6.5 measured) tears ends up; the tooth
  there deepens up to 1.9× (1.3–2.5 [E]) with the excess.
- **The scalpel**: past q = H·2R (fibre wall 350 MPa) the edge cuts,
  shaving 12 µm (about a fibre layer, 10–15 [D]) a pass and taking the
  pastel in it, fixed too.
- **Blown crumbs**: lifted heaped crumbs roll out in their direction (48
  sectors) and settle between 1.15 and 1.56 times the radius where the shear
  stops lifting them (u* ∝ r^(−1.15); impact threshold 0.81–0.86 of the
  fluid one; a puff's few cm too short for saltation): a ring. 5 % of the
  fine grains settle within 10 cm (Wood's fit: 1–9 % for 5 µm); the rest are
  carried off. `blow` returns what left the picture.
