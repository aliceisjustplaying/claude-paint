# A stroke over its own paint: does the brush lift back what it just laid?

**Scope.** `b:stroke` drags a loaded brush. The engine draws a stroke as many
hairs, each laying the share `1 − exp(−x/run)` of its paint over its travel x,
and each lifting part of the wet paint it passes over (`bristle::exchange`,
`engine7::exchange`). The hairs behind pass over the paint the hairs ahead
have just laid. This note answers whether they should lift any of it back. It
explains the `--@ stroke-split` switch and gives its numbers. Values tagged
**[est]** are estimates.

## What the engine did (before the switch)

Each hair contact lifts `v · pickup · wt · own · fl` of the wet column `v`
under it. It never takes the column below the stroke's floor, which is
`v · (1 − pickup)` as the stroke first touches the pixel. A pixel the same
stroke has already laid on takes the rate `own = 0.15`. That factor was an
unsourced constant from the first physical-paint commit. On bare ground the
floor is 0, so every hair that passed lifted about 1 % of the film the
stroke itself had laid there, back into the brush. Paint a hair ploughed
aside landed on pixels not marked as the stroke's own, and there the rate
was the full one.

Measured on a tiny scene: a 200 mm canvas at 400 px, engine 9, brush skim,
blue body paint, load 0.6, pressure 0.8, a straight stroke with the default
ramps. The table gives the share of the load the brush gave the canvas
(laid less lifted). The column "no pickup" is the same stroke with
`pickup = 0`, which is the hairs' law with nothing lifted.

| brush, stroke | before | no pickup | lifted back |
|---|---|---|---|
| filbert 8 mm, 12 mm stroke, bare ground | 7.7 % | 9.7 % | 21 % of what it laid |
| filbert 8 mm, 40 mm stroke, bare ground | 24.4 % | 32.5 % | 25 % |
| hog flat 8 mm, 12 mm, bare ground | 15.2 % | 18.6 % | 18 % |
| round sable 4 mm, 12 mm, bare ground | 15.9 % | 17.5 % | 9 % |

The lifted share grows with the number of hairs, because every extra hair
is one more pass. With the stroke unramped, the filbert gives 10.5 % of its
load with 120 hairs and 9.4 % with 240.

### What made a dragged touch lay about twice a stroke

The touch-split note (touch_film_split.md) found that a dragged touch laid
1.4–2.4 times what a stroke of the same length laid, and put it down to this
lift. The lift accounts for about a fifth of the difference. The larger
parts come from two other places:

- **Clumps.** With stiff paint, a hog brush's hairs gather into clumps, and
  each clump lays more or less than the mean (`sf.clump`,
  `1 + 0.6·stiffen·(2·band − 1)`, one random value per band for each stroke
  id). A brush 8 mm wide has only a few bands, so their mean differs from
  1 from stroke to stroke. The same 12 mm stroke, drawn as the first,
  second ... twelfth stroke of a canvas, gives 7.7, 10.7, 11.1, 11.8, 8.4,
  14.8, 13.1, 11.4, 8.8, 13.0, 14.7 and 11.6 % of its load. The earlier
  note measured the first of these, the lowest. The switch now makes the
  clumps average to 1 over the brush ("Clumps move paint across the
  width", below).
- **Ramps.** A stroke presses down over its first 8 % and lifts off over its
  last 15 %. Over a 12 mm stroke that is a quarter of its length at lower
  pressure, so fewer hairs touch.

Averaged over the twelve stroke ids and drawn without ramps, a 60-unit
(12 mm) filbert stroke and a touch dragged 60 units compare like this:

| | stroke | dragged touch | touch / stroke |
|---|---|---|---|
| before the switch | 15.6 % | 18.6 % | 1.20 |
| with the switch, clumps as before | 19.6 % | 21.9 % | 1.11 |
| with the switch, clumps averaging 1 | 20.3 % | 21.9 % | 1.08 |
| the split law, `(1 − e^{−(ℓ+d)/run}) / (1 − e^{−d/run})` | | | 1.13 |

`ℓ` is the pressed patch's mean length (touch_film_split.md). With the
clumps averaging 1, the stroke lays a little more than its hairs' law
over 60 units, `1 − e^{−60/run}` = 19.3 %. Its hairs travel a little
farther than the hand: the first step counts at least half a hair's
width, and the last step draws the trailing tips to the end. So the
ratio falls 5 % short of the law. With the default ramps, the stroke
lays 15.2 % and the ratio is 1.44 (1.63 before the switch). The law has
no ramps, because a touch has none, so this ratio is not expected to
match it.

## Is the lift physical?

**One contact, one split.** A brush on the canvas is one wetted bed of
hairs. The paint fills the gaps between the hairs and the gap between the
hair tips and the canvas as one continuous liquid. The engine says the same
about a wet tuft: "the paint between the hairs bridges" (`drag_on`). Paint
leaves this bed for the canvas where the bed lifts away behind the brush,
at one splitting meniscus. Film-splitting flows are understood from roll
coating. Coyle, Macosko and Scriven [CMS86] solved the flow between two
surfaces that diverge after a nip. The film divides once, at the meniscus
where the surfaces part, in a ratio set by the flow there: the ratio of the
two films equals the speed ratio to the 0.65 power, confirmed by experiment.
In the limit where one surface stands still (a canvas under a moving brush),
the splitting stagnation line becomes a static contact line [CMS86]. Nothing
upstream of that meniscus is "laid" yet. Paint between the leading hairs and
the canvas is still inside the bed, and it is divided only where the bed
leaves.

**What the engine's per-hair law already is.** The hairs' law
`1 − exp(−x/run)` is the engine's account of the net film a stroke leaves
behind: what the canvas keeps after the split, distributed hair by hair so
that the mark carries the hairs' streaks. If the trailing hairs then lift
part of it back at each pass, the split is applied once at the law and
again at every hair row. The second application double-counts it. The
signature of the double count is that the net deposit falls as hairs are
added (120 → 240 hairs: 10.5 → 9.4 %). A real split at one meniscus does not
change with how many hairs feed it.

**What stays physical.** Wet paint that was on the canvas before the stroke
is a different case. It joins the liquid in the bed, and at the meniscus
part of it goes with the brush. This is the back-transfer printers measure
as imperfect wet trapping [P53], and the "brush picks up the underpainting"
that every painter knows. The engine keeps it. A stroke lifts up to the
tool's `pickup` share of the paint that was under it before the stroke
(the floor `v·(1 − pickup)`, unchanged). The rate per pass no longer needs
the 0.15 that spared the stroke's own paint, because the floor now holds
that paint. The share lifted therefore depends on the cap, not on how many
hairs pass.

**Ploughed paint.** Paint the hairs push aside (`push`) is still inside the
bed until the bed leaves. It carries its share of the floor of the pixel it
came from to the pixel it lands on. So neither the stroke's own paint nor
the protected part of the earlier paint becomes liftable by being pushed.

## Measured deposits to compare against

No study measures the split per hair row. The measurements that exist are
whole-brush deposits, mostly for house paints:

- **First stroke of a freshly dipped brush.** US 3,706,111 [US706] describes
  a manufacturer's "dump" test. A 4-inch brush is dipped to half its hair
  length for 3 min and drained for 60 s. Its first single stroke over a form
  4 in × 4½ in delivers 4.5 g of a soya-alkyd, solvent-borne house paint
  (35 % vehicle: 27 % alkyd, 70 % aliphatic hydrocarbons, 3 % drier; 65 %
  pigment: TiO₂, ZnO, CaCO₃, silica). From the stated composition and handbook
  densities (TiO₂ 4.2, ZnO 5.6, CaCO₃ 2.7, silica 2.2, hydrocarbon 0.77,
  alkyd ≈ 1.05 g/cm³ [est]), the paint is about 1.56 g/cm³. That makes the
  dump 2.9 cm³ over 116 cm², a wet film of about 250 µm laid in about one
  brush-width of travel.
- **What one dip releases.** US 6,367,114 [US114] reports a manufacturers'
  standard test of a 3-inch brush: 48.0 g picked up per dip, 9.7 g released,
  covering 204 in². About 20 % of the paint picked up goes onto the surface
  per dip, as a film of about 57 µm wet (latex density ≈ 1.3 g/cm³ [est]).
  US 3,706,111 gives the release over 60 strokes of 35 in for its alkyd paint
  (22.9 g), a total spread over repeated strokes rather than a per-stroke
  film.
- **Brushed coats.** Manufacturers' data sheets for brush-applied alkyd
  enamels give 60–125 µm (2.5–5 mils) wet per coat [SW, BM]. Sherwin-Williams
  notes that brushing may need more than one coat to reach it.

The engine's stroke lays the film `lay` coats where it starts with a full
load. For the filbert that is 1.2 × 25 µm = 30 µm, falling along the stroke
as the run law gives. At load 0.6 the touch-split note measured a mean
stroke film of about 38 µm. With the switch, a stroke's laid paint rises by
the 9–25 % it used to lift back. The film stays below the house-paint
figures. Those are for heavily loaded wide brushes and fluid paint, so they
bound an artist's stroke from above rather than match it. They support the
direction of the change: the old stroke kept less paint than a brush's
whole-contact deposit, and the fix brings it closer without exceeding it.
The data do not fix `run` or `lay`. Those remain the engine's existing
estimates and are not changed here.

## The law with the switch

- Each paint a hair lays on pixel i raises the stroke's floor there by what
  the pixel actually gained (`Surf::stroke_split`, both exchange paths).
- Pickup's own-paint rate 0.15 is gone. Pickup is
  `min(v · pickup · wt · fl, v − floor)`.
- Ploughed paint m off pixel i carries `m · floor_i / v_i` of the floor to
  its target (`split_floor_take`, `split_floor_land`). A target the stroke
  hasn't touched first takes its floor from its own paint, as a hair's
  first touch would give.

With the switch, a stroke on bare ground gives the canvas exactly what the
same stroke gives with `pickup = 0`, for every brush. The deposit no longer
depends on the hair count (120 and 240 hairs within 1 %). A touch dragged d
and a stroke of length d lay in the ratio the split law gives (1.11 measured
against 1.13). On wet paint the stroke still lifts some of the paint under
it, at most its pickup share, and it gives more than before.

| brush, 12 mm stroke, share of load the canvas kept | before | with the switch, clumps as before | with the switch |
|---|---|---|---|
| filbert, bare ground | 7.7 % | 9.7 % | 15.3 % |
| filbert, wet white | 5.6 % | 9.6 % | 11.5 % |
| hog flat, bare ground | 15.2 % | 18.6 % | 25.9 % |
| hog flat, wet white | 12.8 % | 17.8 % | 20.4 % |
| round sable 4 mm, bare | 15.9 % | 17.5 % | 26.1 % |

(On the wet white the stroke lifted 2.1 mm³ with the switch against 4.3 mm³
before, out of a filbert load of 51 mm³. That was with the clumps as
before. The knife's mark takes a stroke id, so in the first two columns
the strokes on bare ground and on the wet white had different clumps.
With the clumps averaging 1, the two compare directly.)

The stroke ids in this table are the first ones on their canvases. Those
ids happened to lay the least before (7.7 % against a mean over twelve ids
of 11.4 % for the filbert), so the jump in the last column is mostly the
clumps. With 120 and 240 hairs the filbert lays 20.4 % and 20.3 %
(unramped).

## Clumps move paint across the width

**What they are.** A wet brush's hairs pull together into bundles. The
liquid's surface tension draws neighbouring fibres together, and their
bending stiffness resists. Bico, Roman, Moulin and Boudaoud showed this
with a model brush of elastic lamellae dipped in a wetting liquid
[BRMB04]. Pairs of bundles merge in turn into a hierarchical pattern.
Wet fibres do the same in three dimensions [PBBRB07]. Bundles merge only
up to a largest size set by the balance of the two forces [BBR07]. The
deformation depends on the system's size against the elastocapillary
length `ℓ_ec = (B/γ)^{1/2}`, with B the bending stiffness and γ the
surface tension [BRMB04, as given by later work in G14].

**What that conserves.** Coalescence moves hairs, and the paint between
them, across the brush's width. It neither makes paint nor holds any
back, and it doesn't change the force the hand presses with. So the
engine's per-hair clump factor should change where a stroke lays its
paint, not how much it lays in total. Before, the factor's mean over a
narrow brush's few bands was a random number per stroke id. On the
8 mm filbert it ranged from about 0.7 to 1.3, so the same stroke laid
7.7–14.8 % of its load depending only on its id.

**The law with the switch.** At each step of a stroke, `drag_on` takes
the mean of the clump factor over the hairs touching at that step. Each
hair is weighted by what its law would lay over the step without clumps:
its paint times `1 − e^{−travel/run}`, the same as `exchange`. Each
hair's factor is divided by that mean (`Surf::clump_mean`,
`clump_step_mean`). The bands, their levels and their contrast stay as
they were. Only their mean over the brush is now 1. The weighting leaves
out the parts of a hair's deposit that only its pixel pass knows (how much
of its track touches the weave, and the tack). Those differ little from
hair to hair.

Measured on the same scene (12 stroke ids, filbert and hog flat 8 mm,
load 0.6). With the default hand, the filbert's stroke lays 15.2 % of
its load and the hog flat's 25.7 %, with spreads (largest less smallest,
over the mean) of 0.6 % and 2.3 %. Before the switch they laid 11.4 % and
20.9 %, with spreads of 62 % and 53 %. The remaining spread
is the hand's own wander in pressure and path. With a steady hand
(`shake = 0`), the twelve ids lay within 0.06 % (filbert) and 0.5 % (hog
flat) of each other.

The marks still differ. Across the stroke's middle, each id's mark
(normalised to what it laid) differs from the ids' mean mark by 0.107 of
its paint for the filbert and 0.082 for the hog flat (summed |difference|
over rows). Before the switch these were 0.120 and 0.092. So the clumps
keep about 90 % of their across-width effect.

**Clump width against elastocapillary theory.** The engine's clump is
`2·clump_u = 2·r_h·(1 + 4·stiffen²)` wide. Here `r_h` is the modelled
hair's share of the brush's area (`Tool::hair_radius`). For the 8 mm
filbert, the clump is 0.57 mm in barely stiff paint and 2.8 mm in stiff
body paint, with 14 and 2.8 bands across the brush.

An estimate from the energy balance follows. Take a hair of radius a,
clamped in the ferrule, with a wetted free length L. It joins a clump
whose axis is δ = W/2 away. It bends over a length ℓ in an S shape
(clamped at the ferrule, lying along the clump beyond it), which stores
`6·EI·δ²/ℓ³`. Along the remaining `L − ℓ` it lies wetted against the
clump, which gives up a surface energy of about `2γa` per length. That
prefactor is an **[est]** of the meniscus between touching fibres.
Minimising over ℓ gives `ℓ = (3·δ·ℓ_ec)^{1/2}`, where
`ℓ_ec = (EI/(γa))^{1/2}` is the fibre form of the elastocapillary length.
A clump holds while ℓ < L, so

  `W_max ≈ 2·L² / (3·ℓ_ec)`.

The input values:

- γ: 35 mN/m for the oil (`surface::SIGMA`; 33–36 mN/m measured,
  oil_paint_physics.md).
- E: natural hair bristle's 680×10³ psi = 4.7 GPa dry (410×10³ psi wet
  with water, which oil is not) [DuP].
- a: pig bristle measures 160–380 µm across, mid-shaft 268 µm and root
  319 µm [M14, ICAR]. An artist's hog brush is made of the tapered,
  flagged tip ends, so a = 50–80 µm near the paint is an **[est]** at
  the fine end of that range.
- L: the wetted free length, 10–15 mm on a No. 8 filbert, is an **[est]**.

| a | ℓ_ec | W at L = 10 mm | W at L = 15 mm |
|---|---|---|---|
| 50 µm | 115 mm | 0.58 mm | 1.3 mm |
| 80 µm | 232 mm | 0.29 mm | 0.65 mm |
| 134 µm (mid-shaft) | 504 mm | 0.13 mm | 0.30 mm |

So the theory puts a hog brush's clumps in oil at about 0.1–1.3 mm. The
engine's 0.57 mm, in barely stiff paint, lies inside that range. Its
2.8 mm, in stiff paint, is about twice the theory's upper end. Hierarchical
merging [BRMB04] and the uncertainty in L, which enters squared, leave
room for a factor of two, but not a factor of ten. The engine's widths
are left as they were and remain **[est]**, for three reasons:

- In the theory the width depends on the fibres (a, E), on L and on γ.
  γ is about the same for fluid and stiff oil paint. The engine instead
  ties the width and the contrast (`0.6·stiffen`) to the paint's
  stiffness. Stiffness does decide whether the ridges a clump leaves
  survive levelling (the yield-stress floor in oil_paint_physics.md), so
  `stiffen` is a stand-in for levelling at a hair's scale, not for
  coalescence.
- `r_h` is the modelled hair's share of the area, so the clump width
  changes with the number of modelled hairs. That number is a drawing
  resolution, not a property of the brush.
- The tool has no fibre radius or wetted length to compute W from.

Setting W from the formula above would need per-brush fibre data and its
own switch. That is left open.

## Left open

- The clump width and contrast (above): unsourced constants, the width
  of the same order as the elastocapillary estimate but tied to the
  paint's stiffness and to the modelled hair count rather than to the
  fibres and γ.

## Sources

- [BRMB04] J. Bico, B. Roman, L. Moulin and A. Boudaoud, "Elastocapillary
  coalescence in wet hair", *Nature* 432 (2004) 690, doi:10.1038/432690a:
  a model brush of elastic lamellae withdrawn from a wetting liquid
  bundles hierarchically; the elastocapillary length (B/γ)^{1/2}.
- [PBBRB07] C. Py, R. Bastien, J. Bico, B. Roman and A. Boudaoud, "3D
  aggregation of wet fibers", *EPL* 77 (2007) 44005,
  doi:10.1209/0295-5075/77/44005.
- [BBR07] A. Boudaoud, J. Bico and B. Roman, "Elastocapillary
  coalescence: aggregation and fragmentation with a maximal size",
  *Phys. Rev. E* 76 (2007) 060102.
- [G14] A fluid-mechanical model of elastocapillary coalescence, arXiv:1310.4530
  (2014): states ℓ_ec = (B/γ)^{1/2} after [BRMB04], and the "collaborative
  stiffening" of a bundle that limits its size.
- [DuP] DuPont Filaments, "Filament Performance in Brushes" (brochure,
  2014), Table 1: tangent modulus of natural hair bristle 680×10³ psi at
  50 % RH, 410×10³ psi wet.
- [M14] Mohan et al., "Tensile properties of hair fibres obtained from
  different breeds of pigs", *Biosystems Engineering* (2014),
  sciencedirect.com/science/article/pii/S153751101400004X (abstract):
  mid-shaft 267.9 ± 8.1 µm, root 319.2 ± 6.5 µm.
- [ICAR] ICAR, "Value added products from pig bristles": bristles
  210–320 µm across; a review of Indian breeds gives 185–380 µm.

- [CMS86] D. J. Coyle, C. W. Macosko and L. E. Scriven, "Film-splitting
  flows in forward roll coating", *J. Fluid Mech.* 171 (1986) 183–207,
  doi:10.1017/S0022112086001416.
- [CMS87] D. J. Coyle, C. W. Macosko and L. E. Scriven, "Film-splitting
  flows of shear-thinning liquids in forward roll coating", *AIChE J.* 33
  (1987) 741–746, doi:10.1002/aic.690330506: the more shear-thinning the
  liquid, the farther out of the gap the splitting meniscus and the more
  symmetric the split.
- [P53] F. Preucil, "Color and tone errors of multicolor presses", *TAGA
  Proceedings* (1953): apparent trap of an ink printed onto wet ink.
- [US706] US Patent 3,706,111, "Brush bristles": paint release, dump and
  coverage tests of a 4-inch brush in oil-base (soya alkyd) and latex paint.
- [US114] US Patent 6,367,114, "Paint brush having crinkle filaments and
  natural bristles": pick-up 48.0 g and release 9.7 g per dip, coverage
  204 in², 3-inch brush.
- [SW] Sherwin-Williams product data sheets: Industrial Enamel B54-100
  (4.5–9.0 wet mils), Pro Industrial Waterbased Alkyd Urethane 113.12
  (4.0–5.0 wet mils), Product Finishes Alkyd Enamel F75AL0005 (2.5–3.5 wet
  mils).
- [BM] Benjamin Moore V201 Alkyd Urethane Enamel data sheet (3.5–4.0 wet
  mils).
- W. S. Overdiep, "The levelling of paints", *Prog. Org. Coat.* 14 (1986)
  159–175, doi:10.1016/0033-0655(86)80010-3: brushed alkyd films and their
  levelling (background on brush marks; no deposit figures).
- T. C. Patton, *Paint Flow and Pigment Dispersion*, 2nd ed., Wiley (1979),
  application rheology chapter: brushing as flow through the bristle bed.
  Consulted only through secondary citations; no figure here depends on it.
