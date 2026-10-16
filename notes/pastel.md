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
  `Canvas::fix_pastel`, `Canvas::rub` (the finger) and `Canvas::feel`.
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
  begun since has `pastel`, `smudge` and `feel`; an older one replays with
  none of them, its globals as they were.

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

Learned while painting:

- Pastel skips wet paint. Thick touches of blotted essence were still
  "setting" two painted days later and showed through as specks. Let
  essence dry about three weeks before pastel goes over it.
- A paint film thicker than the paper's pore depth has the paint's
  micro-roughness (0.3–2 µm for lean paint, as the research gives), so
  pastel over a heavy essence lay-in fills its tooth in one light pass.
  Keep essence thin where pastel will go, or leave the paper bare.
- Everything a contact abrades stays on the pixels in contact, so the
  paper's hollows take none and two heavy passes of a soft stick cover only
  about 30–60 %; measured strokes cover about 6.6 times their real contact
  area (pastel_stick_tribology.md). The crumbs that spread it are the next
  step.
