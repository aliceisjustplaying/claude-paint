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
- **It stays down** across chunks, waits and saves (the checkpoint's tape
  section, version 23). Paint on it dries where it lies. Lifting it parts
  the paint across its edges as §4 says.
- **Not modeled:** paint creeping under the edge (the tape is taken as
  pressed down along its edge), and the tape lifting a weak ground.

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

## 4. Lifting tape across paint

When tape comes up, the paint laid across its edge since it went down must
part there. How it parts depends on the film's state (paint's `tape::part`).

- **A wet film is a liquid** and parts at the edge.
- **A set film is a solid.** Its modulus and work of fracture are taken to
  grow linearly from nothing at the gel point (`drying::GEL`) to their
  touch-dry values, by its cure `sub` [E: no measurement of a film between
  gel and touch-dry was found].
  - **[M] Modulus** 33 ± 3 MPa: an unaged oil paint film in micro-tension,
    rising to 270 ± 19 MPa after 16 weeks of ageing, with strain at break
    falling from 0.077 to 0.019 ("A multiscale in-situ experimental method
    for characterizing the mechanical response of oil paint", Experimental
    Mechanics, 2026, https://link.springer.com/article/10.1007/s11340-026-01297-z;
    from the abstract). `FILM_E_PA`. **[E] Poisson ratio** 0.4.
  - **[M] Work of fracture** 10–40 J/m² at first crack for oil paints aged
    about 30 years, the lowest (zinc white, raw umber, lead white with
    litharge) about 15 J/m². An animal-glue ground is about 100 J/m², an
    alkyd 250 J/m² (Janas, Avgerou, Charalambides, Fuster-López & Bratasz,
    "Fracture toughness of aged oil paints", npj Herit. Sci. 12, 103, 2024,
    doi 10.1186/s40494-024-01224-2). **[E]** A young film is taken at
    15 J/m². `FILM_FRACTURE_J_M2`.
  - **[E] Adhesion** of the new film to the touch-dry surface beside the
    tape: 2 J/m² on dry paint or a sealed ground, 10 J/m² on a ground still
    absorbent (the new film's oil keyed into it). Coating interfaces measure
    1–300 J/m² (reviews of coating delamination). No measurement for oil on
    oil was found.
- **[S] Tear or turn.** A crack running down through a film meets the
  interface at 90°. It turns into the interface when the interface's
  toughness is under about 0.26 of the film's, the ratio of the energy
  release rates of the deflected and the penetrating crack for elastically
  like layers (He & Hutchinson, "Crack deflection at an interface between
  dissimilar elastic materials", Int. J. Solids Struct. 25, 1053, 1989).
  `DEFLECT`. Otherwise it tears the film at the edge.
- **[M] The flap.** A film that turns into the interface is peeled with the
  tape as a flap. Its two tears converge at the angle θ with
  sin θ = α√(2Bτ)/(2γt) for a wide flap (Kruglova, Brau, Villers & Damman,
  "How geometry controls the tearing of adhesive thin films on curved
  surfaces", PRL 107, 164303, 2011, eq. 6, arXiv:1106.5366; the flat case
  of Hamm, Reis, LeBlanc, Roman & Cerda, "Tearing as a test for mechanical
  characterization of thin adhesive films", Nature Materials 7, 386, 2008).
  B = Et³/12(1 − ν²) is the bending stiffness, τ the adhesion, γt the work
  of fracture times thickness, and α the peel angle (**[E] 90°**: tape
  pulled up and back, about square to the picture). The ridge joining flap
  and film is L = α√(BW/2(2γt + τW)), which tends to α√(B/2τ) for a wide
  flap (their eq. 3). That is taken as the width the film lifts off beside
  the tape. Where sin θ ≥ 1 the flap tears off at once, at the edge.
- **What it gives [D].** A touch-dry film on dry paint (τ/Γ = 0.13 < 0.26)
  turns into the interface. Up to about 25 µm thick the flap converges, and
  the film lifts off over about 0.1–0.2 mm beside the edge (a few pixels on
  a fine canvas, none on a coarse one). Thicker, sin θ ≥ 1 and it tears at
  the edge. A still-tacky film (weak) and a film on an absorbent ground
  (keyed in) tear. Each edge point is judged from the film's thickness
  there.
- **[E] Kept beside the tape**: the touch-dry picture within 10 mm of it,
  as it was when the tape went down (`APRON_MM`), so a lifted film can show
  it again. This bounds the computation; it is not a physical constant.
- **Not modeled:** the tear's wander finer than a pixel, and the film
  lifting beyond the ridge where the adhesion is locally weak.
