# Pastel

A pastel stick drawn on paper (or on any ground): a solid that rests on
the tooth, wears to facets and lays what it abrades, a finger that rubs
it, and fixative. It was first made for a painting in Degas's manner of
the mid-1880s, pastel over peinture à l'essence on toned paper; the
materials it models are in notes/degas-materials.md (a materials-only
research note), the physics in notes/research/pastel_stick_tribology.md,
paper_surface.md and dry_pigment_optics.md. The painter's side is in the
guide, under "Drawing: pencil, chalk, pastel and eraser".

## What it is

- **The engine** (`paint::pastel`): `Stick` (its section, length, worn
  facets, hardness, wear coefficient and dry color), `Canvas::stick_stroke`
  (a stroke along points, each with its force, pose and speed),
  `Canvas::fix_pastel`, `Canvas::rub` (the finger) and `Canvas::feel`;
  `paint::sheet`, a paper mask (`Canvas::lay_sheet`, `lift_sheet`) and
  where a stick held up would touch (`Canvas::stick_contact`).
- **The drawing it lays in.** Pastel shares the drawing pencils and chalk
  lay (`graphite::Drawing`): a dry deposit written into the dry picture,
  so paint laid later composites over it and seals it. A drawing cell's
  flake reflectance is RGB (`Cell::r`); graphite and chalk set all three
  channels to their gray and do the same arithmetic per channel, so their
  results are unchanged. A cell also keeps the pastel in the tooth, µm
  (volume per area as it lies, packed): `loose`, and `bound` by fixative.
- **Serialization.** A drawing without stick pastel keeps its old layout:
  five floats a cell, checkpoint flag 1. Once pastel lands in the tooth (or
  the finger moves some), `Drawing::sticks`, it is nine floats a cell (`a`,
  `r, g, b`, `lift`, `floor`, `film`, `loose`, `bound`), flag 2.
- **The switch.** The Lua verbs are the paper-and-pastel switch's (a
  `--@ paper` line in the log's head, as for `canvas{paper=}`): a painting
  begun since has `pastel`, `smudge`, `feel`, `lay_sheet` and `lift_sheet`,
  and can hold a stick up in a look; an older one replays with none of
  them, its globals as they were.

## Lua

- `pastel(pile, {kind="soft"|"hard"|"pencil", soft=, diameter=, length=})`:
  a round soft stick (Ø 12 mm), a square hard one (6.35 mm) or a pastel
  pencil's sharpened core (Ø 4.5 mm), of the pile's pigments as they look dry.
- `p:stroke(pts, {force=, alt=, azimuth=, roll=, speed=})`: one stroke,
  each a number or a list along the points.
- `p:side(pts, {force=, speed=, roll=})`: laid flat across the stroke.
- `p:roll(degrees)`, `p:snap(mm)`.
- `p:line`, `p:sketch`, `p:hatch`, `p:rule`, as for a pencil: the pencil's
  pressure as force (5 N at 1), the stick at 60° to the paper, its upper end
  to the lower right, at 80 mm/s.
- `smudge(pts, {force=1, pad=, speed=40})`: a finger (or a stump, `pad`
  mm²) drawn along a path.
- `fix(mask)` binds pastel as well as the drawing.
- `feel(x, y)`: what a fingertip feels there, in words.
- `lay_sheet(mask, {grammage=120, tone=})`, `lift_sheet()`: a sheet of
  paper laid over the mask to keep it clean, and taken away with what it
  caught; laid and lifted within one chunk.
- `easel look --hold <stick> --at x,y [--pose force,alt,azimuth[,roll] |
  --side direction]`: where the stick held there would touch.

Also new, and for pencils as well: `hatch{graded=true}` (in the engine,
`graphite::hatch_marks_graded`). The mask becomes a weight: strokes run
wherever it is above 0.04, and each stroke is pressed at every point in
proportion to the mask's value there. A hatched passage then fades into the
next instead of stopping at a line. `hatch_marks` (ungraded) is unchanged.

## The stick (physical)

Sources: notes/research/paper_surface.md, pastel_stick_tribology.md,
dry_pigment_optics.md (materials and physics research; every number there is
tagged measured, derived or estimated).

- **Paper** (`paper.rs`): fibers laid as a Poisson fiber network (lognormal
  lengths, uniform angles, mass laid along each fiber into the pixels it
  crosses), a share of them in flocs (Neyman–Scott clusters), thinned over a
  laid mold's wires; the surface follows the fiber count to a power below one
  (pressing), plus a felt's cells and calendering. Under each pixel the pores'
  depths below the top envelope are exponential with a mean about the fiber
  thickness (µ, `Canvas::micro`). A paint film thicker than µ brings its own
  micro-roughness, which follows its gloss (`Canvas::micro_um`).
- **Contact** (`pastel.rs`): the stick is a solid (round, square or a
  sharpened core) cut by the facets it has worn; its underside over each
  pixel is a ray cast up through it. It sinks until the force is carried. At
  each pixel the sheet gives as a Winkler spring (z modulus over caliper,
  `Canvas::give_mpa_per_um`) while the pastel yields plastically at σp on the
  material ratio of the micro-relief it meets (1 − e^(−o/µ)): the two carry
  the same pressure.
- **Deposit**: Archard, ΔV = K × (material ratio met) × (distance slid) per
  pixel. Only bare paper (or paint) and fixed pastel file the stick; loose
  pastel, which fills the pores from the bottom, is a third body. Volume
  becomes covered area as particles about 6 µm thick (Poisson coverage);
  colors layer by coverage, the last on top. What falls on wet paint is lost
  in it, not laid.
- **Wear**: the volume abraded comes off the stick flat against the paper,
  on the face within 20 µm of its lowest point: a facet within 4° of one it
  has is worn further, else a new one is cut.
- **Fixative**: loose becomes bound (a crust filed at half the rate of
  paper); the particles' scattering falls by the share the resin wets
  (Kubelka–Munk: K/S rises), the coverage floor is set.
- **The finger** (`Canvas::rub`): a pad (135 mm² × F^0.4) picks up 20% of
  the loose pastel under it per pass and lays 25% of what it carries, as
  grains a third as thick (pressed in, they cover more).
- **Dry color** (`dry::dry_color`, `Palette::dry_color`): two-constant
  Kubelka–Munk from each tube's oil masstone (K/S) and oil scattering, the
  scattering raised to its dry value by the Mie ratio for the pigment's
  refractive index (n ≥ 1.9), or set from the dry Mie scattering itself
  below that; carbon blacks by their measured air/oil ratio.
- **The hand**: a stroke's force rises from nothing over 50 ms of travel and
  falls over 40 ms (the 2–5 Hz bandwidth of a hand's force); deposit per mm
  doesn't depend on speed (Archard), only hand time does.

Estimates in use (no measurement of pastel itself exists): σp from 30 MPa
(hard) to 2 MPa (very soft), K from 0.01 to 0.06, particle thickness 6 µm,
crust roughness 3 µm, crust wear 0.5, the finger's 20%/25%, the felt's cells
(0.8 mm, 20 µm), the resin wetting 15% of the particles. The research notes
end with bench measurements that would replace them.

## Crumbs, and the eraser

Two changes found while painting.

- **Crumbs** (`pastel::SHED`, `lay_crumbs`). Laying all the pastel a
  contact abrades on the pixels in contact left the paper's hollows empty:
  two heavy passes of a soft stick covered about 30–60 % and the tooth then
  read "full". Measured (Archambault, in pastel_stick_tribology.md): a
  stroke covers about 6.6 times its real contact area, the debris spreading,
  fragmenting and collecting in the texture. Now 1/6.6 of what a contact
  abrades stays there; the rest rides under the face as crumbs and settles
  (1 − e^(−ds/3 mm) a step) into the pixels under the face that lie within a
  crumb's size (150 µm) of it, the deeper ones taking more. Each crumb is one
  agglomerate, its area drawn from the measured P(A) ∝ A^(−3/2) (cut-off 80–300
  µm by softness and force), pressed flat to PARTICLE_UM like the contact's
  own deposit. Where no paper lies within a crumb's size of the face, the
  crumbs ride on to the next step; what is still carried at lift-off drops
  where the face last was, or falls clear of the picture if nothing lay
  under it. Crumbs that settle on wet paint are lost in it, as the contact's
  own deposit is. Result: two heavy passes cover about 90 %, a light stroke
  stays broken, and the crumbs are discrete specks, not a veil.
- **The eraser lifts the particles** (`Canvas::erase`). It lowered pastel's
  coverage but left its volume in the tooth, so an erased patch stayed
  "full" and refused new pastel. Now the loose volume falls with the
  coverage taken. Graphite and chalk keep no loose volume, so a pencil
  drawing erases as before.

Estimates in use: the 3 mm a crumb rides, the 150 µm bed, the cut-off's
dependence on softness and force (within the measured 100–300 µm).

## A paper mask, and a stick held up

Tools only where each is a physical process modeled from the research, no
undo-like conveniences. Paint's `sheet.rs`:

- **Stick hold** (`look --hold <stick> --at x,y [--pose f,alt,az[,roll] |
  --side dir]`, main.rs `stick_hold_look`). Reads only: no hand time,
  nothing in the log. It calls the stroke's own seating
  (`Canvas::seat_stick`, the arithmetic `stick_stroke` uses) through
  `Canvas::stick_contact`: where the stick touches at that force (in its
  color), where its face lies within a crumb's size of the surface (where
  its crumbs would settle, tinted), and the rest of its low part as a light
  shadow. Without `--pose` or `--side` it is held at 1.5 N, 60° up, its
  upper end to the lower right; `--side` lays it flat across a stroke going
  that way, at 2 N. Only with the paper-and-pastel switch is there a stick
  to hold: without it `--pose` and `--side` are refused.
- **Paper mask** (`lay_sheet(mask, {grammage=, tone=})`, `lift_sheet()`). A
  raised surface `caliper` µm thick (the paper physics' caliper for that
  grammage) with the support's mean pore depth: `Canvas::surface_point`
  feeds the contact solver, so the stick bridges its edge by its own
  geometry, and the edge it leaves is as sharp as the stick's angle and
  force make it. What lands on it (contact deposit and crumbs, shared with
  the paper beside it by the crumb bed's depths) is the sheet's
  (`sheet_catch`), shown in the look and gone at the lift. The finger, the
  eraser and fixative don't reach under it. Wet-paint and graphite verbs
  refuse while it is down (`time::verb_dry` marks the verbs modeled for
  it); a chunk must lift what it laid (session.rs), so chunk checkpoints
  never hold one (a save in between does: checkpoint version 15).

Learned while painting:

- Pastel skips wet paint. Thick touches of blotted essence were still
  "setting" two painted days later and showed through as specks. Let
  essence dry about three weeks before pastel goes over it.
- A paint film thicker than the paper's pore depth has the paint's
  micro-roughness (0.3–2 µm for lean paint, as the research gives), so
  pastel over a heavy essence lay-in fills its tooth in one light pass.
  Keep essence thin where pastel will go, or leave the paper bare.
