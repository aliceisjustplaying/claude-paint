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

**Into the tooth.** A loaded hair's paint reaches below the points the hair
itself touches only as far as the film it leaves. In the exchange's contact
units (±60 µm of relief, `bristle::TOOTH_UM`, scaled by its 1.6) that is
h/(3.2 × 60 µm). Before the switch this reach was a fixed estimate
(`WET_REACH`, half the range). A light stroke of little, stiff paint on a
coarse weave leaves its film on the weave's crowns (the scumble test: 52%
of a band covered against 100% pressed). The contact model gives most of
that already; the film's reach adds a little.

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

**A press on tooth [D].** Where the patch's film lies over a weave, it
fills the hollows under the bed as it covers the crowns, so a press lays
over its hair's whole contact (`exchange`'s `Window::lay`), not only
where the hair itself reaches into the relief. The weight there is
w + (1 − w)·c, with c the hair's contact on that spot and w how wet the
hair is (the exchange's 0.1–0.8 load ramp); how much the hair gives stays
its contact's. Only a hair nearly run dry has too little paint to fill
the gap and lays on the crowns its tip touches. The hollows should hold
more than the crowns, the local gap being deeper by the hollow's depth;
that is not modeled (the film is laid evenly over the contact: an
estimate, the simplest that doesn't break the film).

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
laying what they carry over their contact. They still lift as a press
does where the hand leaves the canvas. A clean hair's gap holds no paint
of its own, so the lift takes up half the wet film there, as a clean
brush's touch does (§4).

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

**Open.** The press film at touch-down is a touch's (`TOUCH_FILM`·`lay`,
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
