# Masking tape, stencil film and spray fixative

The numbers behind paint's `tape.rs` and `spray.rs` (the `tape`, `stencil`
and `fix(points)` verbs). Tags: [M] measured or a manufacturer's datasheet,
[D] derived here from measured numbers, [E] estimate (with the reason).

## 1. Masking tape

- **[M] Thickness.** 3M ScotchBlue 2090 (crepe-paper backing, acrylic
  adhesive) is 5.4 mil (0.137 mm) total thickness, ASTM D-3652 (3M product
  information sheet, January 2010,
  https://multimedia.3m.com/mws/media/1160635O/external-release.pdf). Some
  distributors list 4.9 mil (0.124 mm). The engine uses 140 µm (`TAPE_UM`).
- **[M] Widths.** It is sold as 0.94 in (24 mm) and 1.88 in (48 mm) rolls,
  among others (same sheet; distributor listings). Fine-line masking tapes
  go down to about 1 mm and wide rolls to about 100 mm. The engine takes
  1–100 mm (`TAPE_WIDTH_MM`), with 24 mm when no width is given.
- **[M] It holds only on a sound, dry surface.** The sheet sells 2090 as
  removable cleanly for up to 14 days. Laid on wet or tacky paint, a
  pressure-sensitive adhesive bonds into the film. The engine refuses tape
  wherever the paint under it is not touch-dry.
- **[E] Its color** is only how it looks while it is down. Nothing about it
  stays once it is lifted.
- **Not modeled:** paint creeping under the edge (the tape is taken as
  pressed down along its edge), wet impasto torn along the edge as the tape
  comes away, and the tape lifting a weak ground.

## 2. Stencil film

- **[E] Thickness, 190 µm.** Polyester stencil blanks are sold at 5, 7.5
  and 10 mil (0.125–0.25 mm). This comes from retail listings, not checked
  against one datasheet, so the engine takes the middle weight as an
  estimate. As with tape, the thickness raises the relief the tools meet.

## 3. Spray fixative

- **[M] Distance.** Fixative cans are used "in light, even coats from
  12–16 inches away" (Krylon Workable Fixatif, retailer listing:
  https://www.texasart.com/products/krylon-spray-workable-fixatif). Artists
  report 12–18 in. The engine uses 300 mm (`DISTANCE_MM`) and takes 50 to
  1000 mm.
- **[M] Cone.** Consumer aerosol actuators throw a full cone of about
  20–30° (table of typical aerosol products in "Study on characteristics
  behavior of developing nozzle for aerosol spray", core.ac.uk 42953868; the
  extracted table is garbled, so only the range is used). The engine uses
  25° (`CONE_DEG`).
- **[M] Mass rate.** Spray cans measured 0.6 g/s while spraying (TNO,
  "Aerosols from spray cans and trigger sprays", cited by the RIVM ConsExpo
  Paint Products Fact Sheet, https://www.ncbi.nlm.nih.gov/books/NBK563050/).
  `CAN_G_S`.
- **[E] Propellant, 0.4 of that mass.** Aerosol paints carry roughly a third
  to a half propellant, which flashes off before the spray lands. No figure
  for a fixative was found.
- **[M] Droplets.** Consumer aerosol actuators give a Sauter mean diameter
  of preferably 20–70 µm, measured 15 cm downstream (US 5,323,935). The
  engine uses 50 µm.
- **[E] Spread, 2× the droplet's width.** A slow, low-viscosity droplet
  landing on a porous sheet wets out to a spot a few times its width. No
  measurement for fixative was found.
- **[M] Light coat, 20 g/m².** Conservation sprays lay 20–50 g/m² of a
  1–2 % resin solution (paper_surface.md §5, AIC wiki). The engine takes the
  low end as `fix()`'s light coat, which wets 0.15 of the particles
  (pastel.rs `fix_pastel`).
- **[M] Sweep speed, 300 mm/s.** This is the spray painter's rule of about
  a foot a second.

### The deposit [D]

Take the cone as even in solid angle, with half angle α, and the nozzle h
above the sheet. The flux at a point θ off the axis is (Q/Ω)·cos³θ/h², with
Ω = 2π(1 − cos α). Swept at speed v along a straight line, the mass laid at
distance d from the line is

  m(d) = (Q / vΩ) · 2 S cos α / (h² + d²),  S = √(h² tan²α − d²),

and nothing beyond d = h tan α. With the defaults (Q = 0.36 g/s of solution,
v = 300 mm/s, h = 300 mm, α = 12.5°), one pass lays about 12 g/m² under the
nozzle, falling to nothing 67 mm either side: about half a light coat.

### What it binds [D]

The share of the surface the spots reach is 1 − exp(−1.5 β² V / d), where
V = m/ρ is the volume of solution per area (ρ = 0.8 g/cm³), d is the droplet
diameter and β its spread. Under the defaults' peak this is about 0.8, and
it falls off toward the cone's edge. That share of the loose drawing and
pastel is bound. The resin wets 0.15·m/20 of the bound particles (capped at
1), darkening them by index matching as `fix_pastel` does.

**Not modeled:** droplets drifting past the cone (overspray), runs from a
spray held too close, and the spray's own nonuniformity within the cone.
