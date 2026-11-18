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
- **The pigment's share φ and its limit φm [D].** A pile's tubes give their
  oil by volume v (`Paint::oil_volume`) and the oil left when the pigment
  packs, `packed`, relative to the tube's (palette::tube_packing,
  notes/research/pigment_oil.md). So φ_tube = 1 − v, φm = (1 − v)/((1 − v)
  + packed·v). The medium share m comes back from the paint's `oil`
  (1 + 1.5 m as `Mixture::paint` sets it; 1 + m for blotting):
  φ = φ_tube (1 − m), or φ_tube/(φ_tube + v(1 + m)) blotted. Solvent (turps
  in the paint, an engine-3 thinner beside it) dilutes φ by its share.
- **The measured paint [M].** Lead white ground in linseed oil, measured
  for both its yield stress and its flow curve (O. Ranquet, *Rheology,
  Chemistry and Microstructure of Oil and Tempera Paints*, Dr.-Ing. thesis,
  KIT 2023, publikationen.bibliothek.kit.edu/1000162025; ch. 3 is Ranquet et
  al., Nat. Commun. 14, 1534, 2023):
  - "the LW oil paint at φ = 42 vol% is a paint with a high yield stress
    (σy ≈ 440 Pa)" (printed p. 114; vane-and-cup creep, tangent
    intersection, p. 36). Page numbers here are the thesis's printed ones. 42 vol% lead white is 16 % oil by weight, inside the 12–16 % of
    tube lead whites (pigment_oil.md).
  - Its flow curve (Fig. 3.11, read off the log–log plot) [M, read from a
    figure]: 36 Pa·s at 1000 Pa, 19 Pa·s at 1600 Pa, 10 Pa·s at 2500 Pa,
    7.8 Pa·s at 3100 Pa (84, 250 and 400 s⁻¹), then the paint spilled out
    of the plate–plate gap. Its slope over the last two points, 4.0 Pa·s,
    is the plastic viscosity at the highest rate measured (`TUBE_PA_S`);
    the paint still thins beyond it, so at a brush's 10³–10⁴ s⁻¹ it may be
    less.
  - The same lab's lead white paints at 18, 32 and 42 vol% measure 0.38,
    2.4 and 36 Pa·s at 1000 Pa (Fig. 3.12): about e^(11φ) times the oil's.
    Hard spheres (Krieger–Dougherty, [η] = 2.5, φm from the tube packing,
    which the law used before) give 1.7, 3.2 and 6.5 times: a flocculated
    pigment with its occluded oil is 3–15 times as viscous as that.
  - Its ultramarine paints at 32 vol% have a limiting high-shear viscosity
    "≈ 0.7 Pa.s" and yield stresses of 10 Pa (Abralux pigment) to over
    100 Pa (Kremer pigments) (pp. 126–127): the yield stress depends on the
    pigment's surface far more than on its size.
- **Other measurements of lead white in oil [M]** give far lower yield
  stresses: 2–10 Pa at 35–45 vol% and 27–30 Pa at 51 vol%, τ ∝ φ^7.3
  (Salvant Plisson, de Viguerie, Tahroucht, Menu, Ducouret, Colloids Surf. A
  458, 134, 2014, Fig. 5; Natural Pigments stack lead white); 19 Pa at
  37 vol%, Herschel–Bulkley n = 0.5, about 0.6 Pa·s at 100 s⁻¹ (Cooper et
  al., J. Cult. Herit. 61, 238, 2023; Kremer lead white); about 6 Pa at
  15.7 % oil by weight (de Viguerie et al., C. R. Physique 19, 543, 2018).
  Ranquet shows why they differ: lead white takes up water from the air,
  and 0.6 wt% of it forms capillary bridges between the grains that raise
  the paint's yield stress by up to two orders of magnitude (pigment kept
  at 76 % RH against < 1 %; her 440 Pa paint was ground from pigment kept
  at about 50 % RH, which agreed with pigment kept at 40 %, pp. 49–50). A
  colourman or a painter grinds lead white that has stood in a room's air,
  so the law takes the ambient-humidity paint. The spread matters less than
  it seems: a slider's film goes as √η (§3), and the yield stress mostly
  decides whether the hairs' meniscus holds the film (§3).
- **The 100 Pa the law took before** was "the middle of the 40–200 Pa
  measured for lead-oil paints" (pigment_oil.md, citing de Viguerie et al.
  2009). Those are oil media cooked with litharge, with no pigment in them
  (C. R. Physique 10, 612, 2009, Table 1: "Oil + litharge ≥ 20 %: … yield
  stress ≈ 200 Pa"; the 40 Pa is a megilp jelly, p. 618). No paint was
  measured there.
- **Viscosity [D].** η = η_vehicle (η_tube/η_oil)^(φ/φ_tube), with η_tube =
  `TUBE_PA_S`: the vehicle's with no pigment, the measured paint's at the
  tube's share, exponential in φ between, as the measured series is. For
  lead white this gives 0.27, 1.0 and 2.3 Pa·s at 18, 32 and 42 vol% of a
  46 vol% tube, within a factor 2.5 of the measured (`rheology_follows_the_
  medium`). Before, Krieger–Dougherty for hard spheres gave a tube paint
  0.6 Pa·s.
- **Yield stress.** τ ∝ φ²(φ − φ₀)/(φm(φm − φ)) (YODEL: Flatt & Bowen,
  J. Am. Ceram. Soc. 89, 1244, 2006) [S], normalized so that paint at its
  tube's φ has **[M] 440 Pa** (`TUBE_YIELD_PA`, the measured paint above).
  **[E] φ₀ = 0.15**, the share below which no network spans the paint:
  flocculated suspensions fit 0.1–0.2, and no fit for oil paint was found.
  With `medium=0.6` the yield stress is about 2 Pa; blotted 0.3 it is about
  6 kPa (Ranquet's capillary-suspension paints, "strong impasto", measure
  over 1000 Pa and 3075 ± 392 Pa).
- **Every tube** takes these two numbers at its own tube share, as a
  colourman grinds each pigment to the same consistency. Only lead white's
  were measured for both.

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
- **[M] A hog round's free length.** Bristle rounds are stubbier than
  sable. Rosemary & Co's Ultimate bristle pointed rounds (the maker's size
  chart, rosemaryandco.com/oil-brushes/ultimate/ultimate-pointed-round,
  "All measurements shown in mm"), diameter/hair: 2/0 2.3/8.8, 0 2.8/11.2,
  1 3.0/13.3, 2 3.6/16.0, 3 4.4/18.5, 4 5.4/21.1, 5 6.6/24.2, 6 7.1/26.5,
  7 7.9/28.8, 8 9.3/32.4, 10 11.8/35.5, 12 12.5/40.7, 14 14.6/44.5,
  16 16.7/46.7 mm. The least-squares line is ℓ = 6.0 mm + 2.63 d (within
  3.3 mm of every size), `HOG_ROUND_HAIR_MM`, `HOG_ROUND_HAIR_PER_D`: out
  3.8–4.4 diameters when small and 2.8 when large, where a sable round is
  out 4.6–7. A second maker runs a little longer: Escoda's Clasico
  Chungking white bristle round, series 5131 (retailers' listings of the
  maker's figures, dickblick.com items 06610-1012 and size 4, artlounge.in,
  jacksonsart.com): 2/0 0.8/7.9, 4 2.8/16, 8 4.4/21–24, 10 5.7/27,
  12 6.9–7.0/27–30, 14 8.2/33 mm, 0–6 mm over the Rosemary line. **[E]** A
  round takes its hair between the sable and the bristle line by its
  stiffness, from the sable round preset's (0.2) to the hog preset's (0.85),
  as it takes its hair's diameter between sable and hog.
- **[M] Stipplers.** A stippling brush is a short, full, blunt round. Lynne
  Andrews's stippler series (goat hair; brushesandmore.com), diameter/hair:
  2 3.4/7, 4 5.2/10, 6 6.4/12, 8 7.9/15, 10 9.3/18 mm, ℓ = 0.42 mm + 1.86 d
  (within 0.33 mm). Rosemary & Co's Series 142 hog domed rounds
  (rosemaryandco.com/hog-bristle-domed-round): 7.0/12.6, 9.2/18.0,
  15.1/20.8 mm, 1.4–2 diameters. Conservators' soft hog stipplers
  (preservationequipment.com): 6/10 and 7/15 mm. A stippler's hair is out
  about twice its diameter, a third to a half of a round's. The engine's
  stippler (`Tool::stippler`) is a `Kind::Round` with nothing in its `Tool`
  that tells it apart from a round (its fields are serialized in replay
  effects, so a new field is not added here), so it takes a soft round's
  line. It is used for touches (`stipple`), which take no bed; only a
  stippler dragged as a stroke would press its bed with too long a hair.
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
law holds for Ca ≪ 1, so for a fluid paint the film is taken as never more
than R [E]. The stroke's film is the thinner of the outlet film and this. A
brush barely pressed with fluid paint is held by the meniscus.

**A paste is not held by a meniscus [D].** Surface tension shapes the paint
only where its capillary pressure σ/R beats the paint's yield stress, J =
τR/σ < 1 (the plastocapillary number). A yield-stress paste withdrawn on a
plate leaves a coat "almost independent of the velocity but proportional to
the material yield stress" (Maillard, Boujlel & Coussot, PRL 112, 068304,
2014) [M]: its coat is set by its yield stress, not by capillarity. So for
J ≥ 1 the meniscus sets no bound and the film is the bed's. Tube lead white
(440 Pa) on a hog hair (R ≈ 93–125 µm) has J ≈ 1.2–1.7; with medium 0.25
(72 Pa), J ≈ 0.2. **[E]** Between the two regimes the bound is R/(1 − J),
which is R for a fluid paint and grows without limit as J → 1. Before,
every stroke was held to R, so no stroke of a hog brush laid more than
93 µm and no stroke of a soft one more than 57 µm, whatever its paint.

The measured stroke agrees. An automated 15 mm flat drawn at 20 mm/s at 60°
over paper (Ranquet 2023, Fig. 3.3, read off the profile) leaves about
100 µm of ultramarine oil paint (τ = 10 Pa, η∞ 0.7 Pa·s, J ≈ 0.03): the
meniscus regime's 0.75R–R for a hog hair at Ca = 0.42. A paste of 3075 Pa
(J ≈ 10) leaves 100–450 µm (Rz 350 ± 29 µm), well past R. (The brush's
force was not reported, so this tests the regimes, not the bed's pressure.)

**What it gives.** h ∝ √(U/p) for a fluid paint: pressing harder thins the
film, and drawing faster thickens it. (Both Reynolds's slider and
Landau–Levich give a thicker film at a faster drag; nothing here thins it.)
More medium lowers η and τ and thins it. A yield stress keeps it from
thinning as the drag slows. For a hog flat 7 mm wide with tube paint at 0.7
pressure and 200 mm/s: about 30 µm (12 µm before the measured rheology).
With `medium=0.5`: about 10 µm. A 25 mm flat leaves about 220 µm, 67 µm
with `medium=0.5` (80 µm before, held to the hair's radius). Ordinary
brushing of trade paint leaves about 50 µm wet films (Whittingstall) [M];
Pfund's brushed leaded-zinc house paints 28–30 µm a coat (A. H. Pfund,
*The Hiding Power of White Pigments and Paints*, 1926, p. 10: 6 and 7 coats
for 225 and 204 sq ft/gal) [M, derived].

**Squeeze-out [D].** A slider of finite width leaks paint at its sides as
well as through its outlet (Hamrock, Schmid & Jacobson ch. 8, side
leakage). The engine's hairs plough the share `push` of the paint in their
track outward and ahead, and that paint lands beside the bed, at most the
stroke's film there. The film is what the bed leaves: paint ploughed out of
a pixel under the bed is no longer there, so the bed lays it again up to
the film (the stroke's share of what left the pixel is taken off what it
has laid there). Before this, the share ploughed out of a stroke's middle
was still counted as laid, so the middle stayed below the film while the
pixels beside the bed filled to it: a 9 mm filbert's white stroke lay
16 µm in its middle and 30 µm at its edges, hollow with bright rims over
dark paint. Now it lies ~30 µm across, its edges up to 1.25 times that
(the side leakage's bead), never past the film.

**Into the tooth.** How far a stroke's paint reaches into the weave's
hollows below the points its hairs touch is §5's weave fill: as deep as
the paint its bed passes on per area fills the relief's voids. Before the
switch this reach was a fixed estimate (`WET_REACH`, half the range,
scaled by the exchange's load ramp).

## 4. A press

A touch (`touch_on`) presses the tip down and lifts it. No bed slides, so
no slider film forms and the stroke film's ceiling doesn't apply to it. (As
the slider's speed goes to 0, Reynolds's film tends to the yield-stress
plug, a few µm; a lift-off is not that.)

**Film splitting [M].** A liquid film pulled apart between two surfaces
splits inside the liquid, each surface keeping part of it. Printing
measures the share: in Walker and Fetsko's ink transfer equation the free
film splits in a fraction f "usually 50 %" (W. C. Walker and J. M. Fetsko,
"A concept of ink transfer in printing", TAGA Proc. 1955, 139; S. Abbott,
*Practical Coatings*, "Walker-Fetsko", stevenabbott.co.uk). Roll coating
shows the same split at a nip's exit, at a stagnation line inside the film
(D. J. Coyle, C. W. Macosko and L. E. Scriven, J. Fluid Mech. 171, 183,
1986).

**Over wet paint [D].** Pressed into wet paint of film W, the gap under a
hair holds the wet film against the canvas and the brush's paint F against
the hair, one on the other: a press doesn't shear them together. The film
splits at the middle of the gap, (F + W)/2 from the canvas. Where F > W the
split runs through the brush's paint: the canvas keeps all of its film and
(F − W)/2 of the brush's paint lies on it, and the brush takes back only
its own paint, clean. Where F < W the split runs through the wet film: the
canvas keeps (F + W)/2 of its own paint and none of the brush's, and the
brush takes up (W − F)/2. (Printers see the second case as back-trapping,
a wet first ink lifted onto a later plate's blanket.) On dry paint or bare
ground (W = 0) the canvas keeps F/2. That is what a touch already lays (its
law, with or without the touch split, gives each hair's share as the half
of its face film that a split leaves), so F is twice that, and a touch on
dry ground lays as before but for one thing: its hairs no longer lift back
paint the touch's other hairs have just laid (the split happens once, at
the lift, as the stroke split has it for a stroke).

In the exchange: each pixel starts the touch owing W/2 (`Surf::laid`,
negative). Paint the hairs press onto it pays that first and stays on
them; the rest lands. Where the debt is not paid when the tip has lifted,
the brush takes up what is left of it, (W − F)/2, shared among the hairs by
how long and how hard each was down (`press_lift`). The touch lifts no wet
paint otherwise. The paint the brush gives is what the canvas gains.

Scarlet dabbed with a 9 mm filbert into wet green (some 30 µm) took the
stroke film's ceiling before this (an integration estimate: the stroke's
film at the hand's stroke speed), laid ~55 µm into the wet film, picked the
green up as a stroke does and read brick-brown. With the split it lays
(F − W)/2 ≈ 80 µm of scarlet on the green and the brush stays clean.

## 5. A stroke's start and lift [D]

A brush comes down loaded. At touch-down the gap between the hairs and the
canvas holds the press film F a touch would press out there (twice what a
touch lays: `TOUCH_FILM`·`lay`·(0.4 + 0.6p)·fill coats, with p the hand's
firmest pressure along the stroke, its attack being the press). As the bed
moves, the squeeze drives that paint out and the gap relaxes toward the
slider's film h₀ (§3). Laying a short stroke only the slider's film from
its first step was wrong: a 1.2 mm dab of red with an impasto filbert into
a thin wet green laid 16 µm over 22 µm of green, picked the green up and
read brown, where a touch at the same spot lays its own paint on top.

**The gap's relaxation.** The bed's load per width is carried by the
slider's lift (§3, with its yield stress) and by the squeeze of the film
between the bed and the canvas. Reynolds's squeeze between parallel plates
of length L (a strip, infinitely wide, as the slider is taken; Hamrock,
Schmid & Jacobson, the squeeze-film chapter [S]): W' = ηL³(−ḣ)/h³. So
p L = 0.16 L²(ηU + τh)/h² + ηL³(−ḣ)/h³. In z = 1/h² and the travel s = Ut:

  dz/ds = A − B z − C √z, A = 2p/(ηUL²), B = 0.32/L, C = 0.32 τ/(ηUL).

Its fixed point is the slider's h₀ (p = 0.16L(ηU/h₀² + τ/h₀)). From a
thick press film z grows at A, the squeeze alone (z ≈ 2pt/(ηL²), the
squeeze time), then settles on z₀ over 1/B = L/0.32. For a fluid paint
that is exact: z = z₀ + (z_F − z₀)e^(−Bs). The relaxation length is about
three bed depths whatever the hand's speed and pressure, since both the
squeeze time and h₀² scale as η/p (h₀² ∝ ηUL/p, t ∝ ηL²/(ph₀²) ∝ L/U).
A yield stress shortens it (the C term). The yield stress enters through
the slider's lift, which holds the film at its plug at U → 0; the squeeze
uses the plastic viscosity η (a Bingham squeeze's own yield term would
count the same plug twice). A finite bed leaks at its sides too, which
would shorten the relaxation further (not modeled, as in §3).

For a 9 mm filbert (bed 3 mm deep) the gap is at twice h₀ about 0.9 L
(2.7 mm) along the stroke and within 10 % of it by 5.5 L (16 mm): at
200 mm/s some 0.08 s (an earlier estimate put it at about 0.4 s). The
relaxation is a length, not a time. `relax_m` integrates it per hair and
step (fourth-order Runge–Kutta in z), with each hair's own paint.

**What each spot takes.** A spot the bed's rear leaves during the stroke
takes the outlet film of the gap as it leaves (`OUTLET`·h, never more than
the hairs' meniscus film, as in §3; the bed's rear is the hairs' rearmost
contact). With it goes the paint the squeeze drives out past the slider's
film, `OUTLET`·(h − h₀) over the strip each hair stands for, besides what
the hair gives by its run. The spots still under the bed when the hand
lifts (ahead of where the bed's rear will be when it lifts, its hairs
straightened) take no part in the slide. Each hair over them waits, and
presses its share of the gap's film where it leaves the canvas, as firmly
as it was down while it waited: a touch's split (§4), the canvas keeping
half of the gap's film over the wet film and the brush lifting half the
difference where the wet film is the thicker. So the last bed length of a
stroke does lay a lift split.

**The lift's patch [D].** The paint between the bed's hairs and the canvas
is one film over the bed's footprint, and a film pulled apart splits over
the whole contact, as it does in probe-tack and at a forward-roll nip
(§4), not hair by hair. So each spot under the bed when it lifts keeps
half of the gap's film there, whichever hair is over it. A waiting hair
slides on over such spots until it lifts, and as the hand lifts at the
path's end every hair straightens as it leaves (`reach_end`). So each
hair presses along the whole track since it last laid the slider's film
(its last step, for a hair that didn't wait) to where it lifts: half its
gap's film over the strip it stands for, spacing wide. The canvas keeps
half the gap's film there, this stroke's paint already on it included (so
a press over the slider's own film adds nothing), and what can't land
stays on the hair. A lifting hair is leaving the canvas, so it doesn't
plough. Before this each hair pressed one disc where it lifted, and three
faults showed at a painting's pitch: a disc per hair with bare canvas
between them where the hairs lift one by one in the hand's release; a
lone disc past the mark's end where a long hair's straightened tip
pressed; and a thin band across the end (some 15 µm in a 50 µm stroke),
the track of the sliding hairs' last step, which no hair laid.

**The weave's hollows [D].** Where the bed slides or presses over a
weave, its paint fills the hollows below the crowns the bed rests on as
deep as two bounds allow (`stroke_film::fill_m`, `bristle::weave_fill`).

- *The yield stress.* A Bingham paste in a slot of width w moves once the
  pressure across it beats the yield stress on its two walls: p·w = 2τ·δ
  for a plug δ deep (the force balance behind a yield-stress fluid's flow
  in a channel, and behind the perfectly plastic squeeze flow of a paste
  between plates: N. Roussel and C. Lanos, "Plastic fluid flow parameters
  identification using a simple squeezing test", Applied Rheology 13,
  132–141, 2003; P. Coussot, *Rheometry of Pastes, Suspensions, and
  Granular Materials*, Wiley 2005) [S]. The hollows' width w is the
  relief's own: the mean distance between its crossings of its mean
  level, π·σ/σ′ (S. O. Rice, "Mathematical analysis of random noise",
  Bell Syst. Tech. J. 23, 282, 1944; for a weave of pitch λ, λ/2), over
  the footprint of the stroke or touch.
- *The supply.* The paint fills the relief from the crowns down, as deep
  as the voids above that depth hold the paint the bed passes on per area
  (`bristle::Relief::voids`, from the footprint's relief). A sliding bed
  passes on what its hairs give: each hair's share of what it carries
  over its step (the exchange's run, `dep_total` before its contact's
  share), laid over its share of the area the bed sweeps (the hairs one
  behind another along a track, the bed's length over their spacing),
  and never more than the slider's film: h₀ is a ceiling, and a bed fed
  less than it lays a thinner film (starved lubrication: the film follows
  the lubricant available at the inlet, F. Chevalier, A. A. Lubrecht,
  P. M. E. Cann, F. Colin and G. Dalmaz, "Film thickness in starved EHL
  point contacts", J. Tribol. 120, 126–132, 1998) [M]. A press doesn't
  move: for as long as it presses it squeezes the tuft's paint into its
  patch, so its supply is what the hair holds over its share of the bed.

The crowns are the relief's mean level and √2 its spread, the crests of
a sinusoidal relief of that spread [D]: a bed's face spans many hair
spacings and rests on the weave's crowns, not on its highest fibre. Where
the paint reaches, it wets the surface, crown or hollow alike: a pixel
takes it by the share of its own relief (σ′ either side, planar) above
the paint's floor, not by the hair's contact ramp, which made it lie in
proportion to the relief (bright crowns, thin hollows). The paint the
hairs push aside is the bed's too, so it lands only where the bed's paint
reaches; what can't land stays where it was.

*Which bound acts.* The bed presses at kPa: a 6 mm hog flat at pressure
0.15 at 35 kPa, a 12 mm filbert releasing at 0.03 at 0.6 kPa (`pad`).
Paint yield stresses are tens to hundreds of Pa (§1). So p·w/2τ is
millimetres against hollows of tens of µm, and no bed the hand presses
leaves a hollow bare for its paint's stiffness. A stiffer paint, or a
lighter bed, lifts on a thicker film (h₀ → 0.16τL/p, §3), which fills
more. What leaves the hollows bare is the supply. A 6 mm hog flat dragged
lightly at 200 mm/s over 12 threads/cm unground linen (crowns 80 µm, voids
26 µm deep on average below them) with lean, blotted paint (τ ≈ 400 Pa)
lays a 39 µm slider film; loaded at 0.3 it passes on far more than that and covers the band,
while rubbed nearly dry (a twentieth of a full load) it passes on less
than the voids hold and covers 0.41 of it, the hollows bare and the
crowns painted: a scumble. A 12 mm filbert two thirds spent at the end of
a long stroke still passes on more than its weave's voids hold, so it no
longer breaks into crown dots there (the exchange's load ramp, an
estimate that is 0 at a tenth of a full load and 1 at eight tenths,
called its hairs mostly dry).

*A press levels to the bed.* The gap over a hollow is deeper than over a
crown by the hollow's depth below the crowns (to the fill's depth), so it
holds that much more paint: a press lays its paint in proportion to the
local gap h + d, and where it splits on the lift the canvas keeps half of
it, (h + d)/2 (`Window::lay`, the per-pixel cap). A full press of a
loaded 9 mm filbert on that linen leaves some 30 µm over the hollows and
25 µm on the crowns (laid evenly, within 6 % of each other). Before this,
a press laid one even film over its contact, an estimate.

**The start on tooth [D].** At touch-down the gap holds the press film F,
which may be thinner than h₀. For a 12 mm filbert of tube paint at the
steering law's 400 mm/s, F ≈ 39 µm against h₀ ≈ 105 µm at pressure 0.7
and 325 µm at the attack's 0.08. The start then lays the outlet of a thin
gap (15–27 µm). §3's tooth reach (the paint reaches as deep into the
relief as the film it leaves) left a lightly pressed attack on the
weave's crowns alone: a grid of dots at the thread pitch, 0.67 mm on a
15 threads/cm linen. The moving bed's inlet floods the relief as the
slider's does once the gap has relaxed, so at the start the paint reaches
as deep as the deeper of the gap's outlet film and the slider's.

**At a painting's pitch.** Engine 7 runs a contact larger than its window
(`window::CAPACITY`, 1024 pixels with the plough's margin) through engine
5's kernel, which has neither the press's split nor the gap. A 12 mm
brush's hairs at 12 px/mm (2400 px across a 200 mm canvas) are some 5 to 8
pixels in radius, so at a painting's pitch every slide and press of the
stroke film ran that kernel. A stroke laid about a third of its law's film
(20 µm of 72 µm for a 12 mm filbert of tube paint; at 1 to 6 px/mm it laid
the law's), and a touch ran without its split: no debt over the wet
film, its hairs taking up wet paint as a slide's do. A contact of the press or the gap now runs engine 7 at any
size, its window's scratch growing to fit. A stroke shorter than `TOUCH_STEPS` sweep
steps is sampled in that many, so its attack and release press and lift it
(sampled only at its two ends, a dab's ramps left it barely pressed).

**Pickup.** The split runs at the middle of the gap: the brush takes up
wet paint only where its gap's film h is thinner than the wet film W the
stroke found, and then (W − h)/2. In the slide the hairs take the share
(W − h)/W of what they take scraping the canvas bare. This holds along the
whole stroke. A stroke pressed hard into thick wet paint (h₀ < W) still
works it up. A light loaded stroke over a thinner wet film lays on top,
and a stroke no longer lifts back its own fresh paint.

**A clean blender.** The gap law is a loaded bed's: its press film is the
brush's own paint, squeezed out toward h₀. A blender (`b:blend`) is wiped
on the rag before each stroke, so its press film is next to nothing. Under
the law that gap stays shut for the whole stroke (in z = 1/h² a gap of
zero is a fixed point: the lift and the squeeze both grow without bound
as h → 0, and from a small h the gap opens only as e^(Bs/2)). The hairs
then laid nothing (the outlet film 0.69 h ≈ 0) and took up everything
they passed (the share (W − h)/W ≈ 1), so the blender moved no paint and
the rag took half of it. That is the smooth, sealed plate the squeeze
model assumes, not a dry tuft. Dry hairs dragged into wet paint wet in it
(the paint wicks between them in a contact) and carry it on. So a blender
stroke has no press film to relax: its hairs pick up and lay back as the
slide has it from their first step (the slider's or meniscus film, §3),
laying what they carry over their contact. Nor do they lift as a press:
the lift's split is of the film the bed brought down, and a wiped bed
brought none. Lifting as a press, a clean hair's empty gap took up half
the wet film under the bed at each stroke's end, a pale patch, and its
haul went down at the next stroke's start (blender.md, "The ladder").

**One law.** As the stroke's length goes to 0 the slid strip, no longer
than the stroke, goes to nothing, and the stroke is the lift's split over
the bed: on bare ground a stroke of no length lays 22 µm where a touch
lays 24 µm (the bed's hairs sit as a stroke places them, not as a touch
spreads them). For a long stroke the gap is the slider's film past its
first few relaxation lengths, and the stroke lays the slider's film as
before. Between them the film falls along the stroke from its start. On
bare ground a long stroke lays the slider's film where before its hairs
lifted back some of their own paint (a 9 mm filbert's 15 µm of a
23 µm ceiling is now 22 µm).

**Open.** A slide levels to its bed as a press does (its outlet film over
the crowns, deeper over the hollows); a slide lays its film evenly over
where its paint reaches. The relief's statistics are the footprint's,
not each spot's. The paint the bed passes on per area takes the
exchange's run (`Tool::run`, an estimate of how far a full load lasts)
for what a hair gives.

The press film at touch-down is a touch's (`TOUCH_FILM`·`lay`,
an estimate), not the squeeze's. A press squeezes the bed's paint for the
attack's time (Stefan's squeeze) and no thinner than the plug its yield
stress holds (a Bingham strip's τL/2p). Where it is thinner than h₀ the
start lays thinner than the middle; with the squeeze it would set that
physically.

Spots the slide leaves take the slider's outlet film on top of
the wet film they had (the ceiling counts what the stroke adds, as in §3),
while the lift's split keeps (h + W)/2 in all. By mass balance the outlet
carries the wet film in with it, so it should keep `OUTLET`·h in all. Over
wet paint the two laws then differ by the strip a stroke slides. That is
left as §3 has it.

## 6. Not modeled

The film levelling after the stroke is the drying model's (`drying`,
Orchard). The brush's own reservoir draining (Washburn flow along the
hairs) is not here: the deposit along a stroke stays the exchange's (its
`run`). The stroke film is a ceiling on what one stroke lays, not a supply
law. Shear thinning beyond Bingham (Herschel–Bulkley n < 1), and stand oil
as a medium (above), are not modeled either.

## 7. What one loaded stroke hides

A painter's loaded brush of stiff lead white lays an opaque light over a
dark ground in one stroke. On the easel it covered about a fifth per coat
(a painting begun with the switch, at 2400 px over 610 mm).

**Measured on the engine.** One stroke each, 75 mm long, load 0.85,
pressure 0.5, the hand's own speed, over knifed bone black and a knifed
mid-tone set for 30 days and over the bare off-white ground; 3.9 px/mm.
Film is the wet film the stroke added (median over its middle); contrast
ratio is Y over black ÷ Y over the ground.

| stroke | before: film, CR, L* over black | after |
|---|---|---|
| tube lead white, 6 mm hog filbert | 24 µm, 0.58, 72 | 61 µm, 0.81, 84 |
| tube lead white, 16 mm hog filbert | 86 µm, 0.87, 87 | 101 µm, 0.90, 89 |
| lead white blotted 0.3, 6 mm | 87 µm, 0.91, 90 | 100 µm, 0.92, 90 |
| lead white blotted 0.3, 16 mm | 90 µm, 0.92, 90 | 123 µm, 0.94, 92 |
| flesh half-tone (white, ochre, vermilion, sienna, black), 6 mm | 19 µm, 0.70, 62 | 60 µm, 1.00, 68 |
| face coat (white + `medium=0.25`), soft 7 mm filbert, pressure 0.3 | 36 µm, 0.68, 70 | 82 µm, 0.88, 76 |

The black under them is L* 3; the ground L* 87.5; lead white knifed thick
L* 94. Every "before" film of a hog brush stops at 86–93 µm, the hairs'
radius (93 µm), and the 6 mm brush's tube white lays the slider's 24 µm
under 39 kPa of bed. The 16 mm strokes "after" are short of their law's
film (200–900 µm): the brush gives no more than it carries (its `run`).

**What a stroke lays, measured.** A single oil brushstroke of a fluid
paint from an automated 15 mm flat: about 100 µm (Ranquet 2023, Fig. 3.3);
of a stiff paste, 100–450 µm. Van Gogh's paint layers range "between 20
and 600 µm with an average value at about 100 µm", and his sky's
brushmarks are 85 µm deep on average (Salvant Plisson et al. 2014, p. 140,
confocal profilometry) [M]. Brushed trade paints, spread thin on purpose,
are 28–50 µm a coat (§3).

**How lead white hides, measured.**
- Revigo 08A (Geldof et al., Herit. Sci. 6:17, 2018; lead white in
  poppyseed oil drawn down "using a block operator to create a relatively
  uniform layer of 100 µm thickness", dried in air): 76.8 % over the
  chart's black, 89.4 % over its white at 550 nm, contrast ratio 0.86
  (0.92 at 450 nm, 0.81 at 650 nm) [M]. The engine's lead white is fitted
  to these spectra as a 100 µm film (notes/pigment-mixer.md). A Kubelka–
  Munk fit with the paper's Saunderson constants gives S ≈ 62 mm⁻¹,
  K ≈ 0.03 mm⁻¹ at 550 nm for 100 µm [D].
- Pfund's cryptometer: basic carbonate white lead, 7:3 by weight in oil,
  hides a black plate completely (wet, by eye) at 36 cm² per gram of
  pigment, 243 sq ft/gal of paint (A. H. Pfund 1926, p. 8; H. A. Gardner,
  *Physical and Chemical Examination of Paints*, 1927, p. 23: 36 cm²/g,
  242 sq ft/gal) [M]: about 168 µm of wet paint at 30 % oil, or, at the same
  hiding per gram, about 97 µm at the 16 % oil of a stiff paint [D].
- de Viguerie et al. 2018: lead white at 10.8 % oil drawn with a 30 µm
  applicator over optical black reaches L* 82.5 after a week [M]: S ≈
  50 mm⁻¹ if the film is 30 µm [D].

**Which link was wrong.**
1. *The yield stress* — its source was not a paint (§1). Measured lead
   white paints span 2–440 Pa with the pigment's water; the law takes the
   ambient-humidity paint's 440 Pa.
2. *The viscosity* — hard spheres gave tube paint 0.6 Pa·s; the measured
   lead white paint is 4 Pa·s at the highest rate measured, and its
   dilution series 3–15 times the hard-sphere law. The slider's film goes
   as √η, so a 6 mm filbert's tube white went from 24 to 61 µm.
3. *The hairs' radius* as a cap on every stroke (an estimate) — wrong for
   a paste whose yield stress beats its hairs' capillary pressure (§3).
   That held a stiff white to 93 µm whatever the brush.
4. *The bed's pressure and length* — not changed. The 6 mm filbert presses
   0.48 N on 6.1 × 2.0 mm at pressure 0.5 (39 kPa). No measurement of a
   painter's brush force was found to test it; Ranquet's automated stroke
   gives no force.
5. *The supply* — not changed; it now limits the 16 mm strokes, as it
   should.
6. *Lead white's scattering* — not changed here, and open. The engine's
   lead white hides as Revigo's 100 µm film does (CR 0.92 at 100 µm over
   a blacker black than Revigo's chart). If "100 µm" was the block
   applicator's gap, its film was 50–90 % of that (BYK: the wet film is
   about half the gap clearance; 50–90 % with the paint), and lead white's
   S per µm would be up to twice the engine's. Pfund's complete hiding at
   about 97 µm of stiff paint points the same way (contrast ratio about
   0.95–0.98 by eye, S ≈ 90–160 mm⁻¹ for a nearly white film [D]). Neither
   source gives the film's measured thickness, so the data stay as fitted.

**The test** (`stroke_film_hiding_tests` in easel's api.rs): one loaded
stroke of lead white blotted stiff with a 16 mm hog filbert lays at least
100 µm and hides set black at a contrast ratio above Revigo's 0.86 at
100 µm; so does a 6 mm filbert's (its contrast ratio). Before, the 16 mm
stroke stopped at 91 µm.
