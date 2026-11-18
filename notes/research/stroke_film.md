# The film one brush stroke leaves

The numbers and laws behind paint's `stroke_film.rs` (the stroke film of a
painting begun with `--@ gestures-only`; engine-word bit 58). Tags: [M]
measured or from a datasheet, read in the source; [S] a standard handbook or
textbook value, not re-read for this note; [D] derived here; [E] an
estimate, with the reason.

## 1. The paint's rheology

Oil paint is a suspension of pigment in oil with a yield stress, fitted by
Herschel–Bulkley (notes/research/oil_paint_physics.md §1). The stroke film
takes two numbers from it: the viscosity at brushing shear η, and the yield
stress τ.

- **[M] Linseed oil** η = 0.048 Pa·s, σ = 33–36 mN/m at 20 °C
  (oil_paint_physics.md, its source [6]). `LINSEED_PA_S`, `SIGMA_N_M`.
- **[S] Gum turpentine** η = 1.375 mPa·s at 25 °C (CRC Handbook of
  Chemistry and Physics, viscosity of liquids). `TURPENTINE_PA_S`.
- **[M] Stand oil** η = 0.5–25 Pa·s by grade (oil_paint_physics.md). The
  engine has no stand-oil medium yet (a pile's `medium=` is linseed), so the
  law doesn't use it. A medium kind would enter as the vehicle's viscosity
  below.
- **[S] The vehicle.** Oil and solvent mix by Arrhenius's log rule,
  ln η = x ln η_solvent + (1 − x) ln η_oil, x the solvent's share of the
  vehicle (Arrhenius 1887; Grunberg & Nissan, Nature 164, 799, 1949).
- **[S] Crowding.** η = η_vehicle (1 − φ/φm)^(−[η]φm), [η] = 2.5 for hard
  spheres (Krieger & Dougherty, Trans. Soc. Rheol. 3, 137, 1959).
- **The pigment's share φ and its limit φm [D].** A pile's tubes give their
  oil by volume v (`Paint::oil_volume`) and the oil left when the pigment
  packs, `packed`, relative to the tube's (palette::tube_packing,
  notes/research/pigment_oil.md). So φ_tube = 1 − v, φm = (1 − v)/((1 − v)
  + packed·v). The medium share m comes back from the paint's `oil`
  (1 + 1.5 m as `Mixture::paint` sets it; 1 + m for blotting):
  φ = φ_tube (1 − m), or φ_tube/(φ_tube + v(1 + m)) blotted. Solvent (turps
  in the paint, an engine-3 thinner beside it) dilutes φ by its share.
  For paint naming no tubes (v = 0.54, packed 0.7): φ = 0.46, φm = 0.55,
  η ≈ 0.6 Pa·s, the plastic viscosity at high shear. Measured apparent
  viscosities of 10–1000 Pa·s at 10–100 s⁻¹ include the yield stress's τ/γ̇,
  which the law adds itself (§3).
- **Yield stress.** τ ∝ φ²(φ − φ₀)/(φm(φm − φ)) (YODEL: Flatt & Bowen,
  J. Am. Ceram. Soc. 89, 1244, 2006) [S], normalized so that paint at its
  tube's φ has **[M] 100 Pa**, the middle of the 40–200 Pa measured for
  lead-oil paints (pigment_oil.md; tube paints span 10–1000 Pa,
  oil_paint_physics.md). **[E] φ₀ = 0.15**, the share below which no network
  spans the paint: flocculated suspensions fit 0.1–0.2, and no fit for oil
  paint was found. With `medium=0.6` the yield stress is under 1 Pa; blotted
  0.2 it is several times the tube's.

## 2. The bristle bed

- **[S] Hair modulus** 3.5 GPa: α-keratin fibres (human hair) measure
  3–4.5 GPa in tension (C. R. Robbins, *Chemical and Physical Behavior of
  Human Hair*, 5th ed., 2012, ch. 9). Hog bristle and sable are keratin.
  `HAIR_E_PA`.
- **[M] Hair diameters**: sable 60–120 µm, hog 200–300 µm
  (oil_paint_physics.md §2). **[E]** A tool's `stiffness` (0 soft hair, 1
  hog) takes its hair between 90 and 250 µm.
- **[E] Free length.** At full pressure the tips trail by the tool's
  `length` (as `drag_on` bends them). That is taken as half the hair's free
  length (`BEND` = 0.5): a flat's hair is out about 1.3–1.5 widths and
  trails about half of that.
- **[M] A round's free length.** Rounds are not made in a flat's
  proportions. Rosemary & Co's Series 33 kolinsky rounds (the maker's size
  chart; the same figures on its pocket-brush pages and at retailers):
  size 0 1.5 mm across the belly, 9.8 mm of hair out; size 2 2.0/14.1;
  size 4 3.4/15.7; size 6 3.9/20.0; size 8 5.2/24.2 mm
  (rosemaryandco.com, "R2 pocket pure kolinsky sable pointed size 8" and
  "size 4"; hobbyheaven.org and artsavingsclub.co.za Series 33 charts). The
  least-squares line is ℓ = 5.2 mm + 3.61 d (within 1.7 mm of every size),
  `ROUND_HAIR_MM`, `ROUND_HAIR_PER_D`. A round's hair is out 4.6 (size 8)
  to 7 (size 2) diameters, where the flat's proportion (`length / BEND`,
  1.4 widths with the default `length`) gives 1.4. With the flat's free
  length (2.8 mm) a 2 mm sable round's ~130 hairs, bent 0.7 mm at pressure
  0.5, pressed 0.49 N on its 2 mm disc (122 kPa) and the film fell to 7 µm;
  a 1.5 mm round's to 4 µm, and a dark line read as a faint trace. With the
  measured 12.4 mm the same tuft presses 6 mN (1.4 kPa): the outlet film is
  over the hair's radius, so the meniscus holds it (61 µm), and the line
  lays as much as the brush gives, about 35 µm on bare ground, as it did
  before the switch. A round given a `length=` longer than the maker's hair
  keeps it (the hair is as long as asked).
- **[S] Each hair** is a cantilever: F = 3EIδ/ℓ³, I = πd⁴/64.
- **[E] Tuft shape**: a flat's ferrule is about a quarter as deep as it is
  wide, a filbert's a third. Rounds, riggers and blenders are round, and a
  fan is a layer or two of hairs. **[E] Packing** 0.5: random packings of
  parallel fibres fill 0.5–0.8, and a loaded, splayed tuft is at the low
  end.
- The bed is the tuft's footprint: its depth along the travel (L) by its
  width. The tuft's force over it is the mean pressure p. A hog flat 7 mm
  wide at 0.7 pressure gives about 1 N, a sable a tenth of that.
- **The hand's speed.** `speed=` (mm/s), or without it the hand-time
  ledger's pace: the steering law through a tunnel four mark-widths wide
  (Accot & Zhai 1997), at most 400 mm/s (tally.rs, [E] there). Architectural
  brushing runs at about 0.5 m/s (Whittingstall, TA Instruments RH059,
  "Paint evaluation using rheology") [M].

## 3. The film [D]

**The squeeze film.** Reynolds's fixed-incline slider of length L at
speed U over a film of viscosity η carries the greatest load per width
W = 0.16 ηUL²/h₀² at inlet/outlet 2.2 (Reynolds, Phil. Trans. 177, 157,
1886; e.g. Hamrock, Schmid & Jacobson, *Fundamentals of Fluid Film
Lubrication*, ch. 8: W·h₀²/(ηUL²) = 6[ln(1+K)/K² − 2/(K(2+K))], 0.0267·6 at
K = 1.2) [S]. With p = W/L, h₀ = 0.4√(ηUL/p).

**The yield stress.** At the film's shear γ̇ = U/h the paint's effective
viscosity is η + τ/γ̇, so h₀² = 0.16 L (ηU + τh₀)/p. Solved,
h₀ = ½(b + √(b² + 4c)) with b = 0.16 τL/p and c = 0.16 ηUL/p. As U → 0
the film tends to 0.16 τL/p, where the bed's pressure gradient p/L just
balances τ across it. A yield-stress fluid leaves a plug of finite
thickness, independent of speed and proportional to its yield stress. This
is what Maillard, Boujlel & Coussot measured withdrawing a plate from a
yield-stress bath: a coated layer "almost independent of the velocity but
proportional to the material yield stress" (PRL 112, 068304, 2014,
https://enpc.hal.science/hal-01117227/document) [M]. There gravity sets the
balance; here the bed's pressure does.

**The outlet.** At K = 1.2 the flux is U·h*/2 with h* = 2(1 + K)/(2 + K)·h₀
= 1.375 h₀ at the pressure peak. All of it is carried off on the moving
canvas: a film of 0.69 h₀ (`OUTLET`).

**The meniscus.** Behind each hair the paint is drawn out of a meniscus of
the hair's radius R: h = 1.34 R Ca^(2/3), Ca = ηU/σ (Landau & Levich, Acta
Physicochim. URSS 17, 42, 1942; Bretherton, J. Fluid Mech. 10, 166, 1961)
[S]. The yield stress's share of the drag goes in Ca as (ηU + τh)/σ. The
law holds for Ca ≪ 1, so the film is taken as never more than R [E]. The
stroke's film is the thinner of the outlet film and this. A brush barely
pressed is held by the meniscus.

**What it gives.** h ∝ √(U/p) for a fluid paint: pressing harder thins the
film, and drawing faster thickens it. (Both Reynolds's slider and
Landau–Levich give a thicker film at a faster drag; nothing here thins it.)
More medium lowers η and τ and thins it. A yield stress keeps it from
thinning as the drag slows. For a hog flat 7 mm wide with tube paint at 0.7
pressure and 200 mm/s: about 12 µm. With `medium=0.5`: about 5 µm. A 25 mm
flat leaves about 80 µm. Ordinary brushing leaves about 50 µm wet films
(Whittingstall) [M].

**Into the tooth.** A loaded hair's paint reaches below the points the hair
itself touches only as far as the film it leaves. In the exchange's contact
units (±60 µm of relief, `bristle::TOOTH_UM`, scaled by its 1.6) that is
h/(3.2 × 60 µm). Before the switch this reach was a fixed estimate
(`WET_REACH`, half the range). A light stroke of little, stiff paint on a
coarse weave leaves its film on the weave's crowns (the scumble test: 52%
of a band covered against 100% pressed). The contact model gives most of
that already; the film's reach adds a little.

## 4. Not modeled

The film levelling after the stroke is the drying model's (`drying`,
Orchard). The brush's own reservoir draining (Washburn flow along the
hairs) is not here: the deposit along a stroke stays the exchange's (its
`run`). The stroke film is a ceiling on what one stroke lays, not a supply
law. Shear thinning beyond Bingham (Herschel–Bulkley n < 1), and stand oil
as a medium (above), are not modeled either.
