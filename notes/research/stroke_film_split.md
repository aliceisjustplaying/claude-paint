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

**Clump width before this change.** The engine's clump was
`2·clump_u = 2·r_h·(1 + 4·stiffen²)` wide. Here `r_h` is the modelled
hair's share of the brush's area (`Tool::hair_radius`). For the 8 mm
filbert, the clump was 0.57 mm in barely stiff paint and 2.8 mm in stiff
body paint, with 14 and 2.8 bands across the brush. So the width
depended on the paint's stiffness and on the number of modelled hairs,
which is a drawing resolution, not a property of the brush.

## Clump width from elastocapillary coalescence

With the switch, a clump is as wide as elastocapillary coalescence
makes it (`bristle::coalesce`). The paint's stiffness no longer enters.
Whether the ridges a clump leaves survive is left to the levelling
physics the engine already has (below).

**The law for a pair.** Two flexible plates clamped a gap d apart, with
a wetting liquid between them, zip together from their free ends. They
stop at the distance `L_stick` from the clamps where the plates' bending
balances the surface energy the zipping gives up [RB10, eq. 6]:

  `L_stick⁴ = 9·d²·L_EC² / 2`,  `L_EC = (B/γ)^{1/2}`,

with B the plates' bending stiffness per width. The energy balance
gives the same prefactor. Each plate bends in an S over ℓ, clamped at
the base and lying flat along the other beyond, and stores
`6·B·(d/2)²/ℓ³`. Zipping gives up `2γ` per area. The minimum of
`3·B·d²/ℓ³ − 2γ·(L − ℓ)` lies at `ℓ⁴ = 9·d²·B/(2γ)`. This check fixes
the S-bend's factor 6, which the previous section's estimate also used.

For fibres, Roman & Bico replace B by `B_rod = π·E·r⁴/4` and the
capillary term by `γ·r` [RB10, section 3]. The prefactor of that γ·r
term follows from the menisci. Take two touching wet fibres of radius
r, with zero contact angle, and menisci of radius R filling the two
wedges between them. Joining them gives up the arc of each fibre that
the menisci cover, less the menisci themselves. Per length and per
wedge, that is `2r·φ − 2R·α`, with `cos φ = r/(r + R)` and
`α = π/2 − φ`. For both wedges together the gain is:

| R | 0.5 r | r | ∞ |
|---|---|---|---|
| gain / (γ·r) | 1.90 | 2π/3 = 2.09 | 2(π − 2) = 2.28 |

The engine takes R = r, which is an **[est]**: the gain varies by ±10 %
with how much paint the wedge holds. It ignores the pressure in the
paint, which is near the reservoir's. So for two fibres with a gap g
between their surfaces,

  `L_pair⁴ = 9·EI·g² / (c·γ·r)`,  `c = 2π/3`.

This is the previous section's balance with c = 2 and δ = W/2.

**Clumps of clumps.** A brush withdrawn from the liquid bundles in a
cascade: pairs of clumps merge into larger clumps [BRMB04, PBBRB07].
A clump of n hairs is n times as stiff as one hair, because the liquid
lets its hairs slide [RB10]. Two of them merge where

  `ℓ⁴ = 9·(n·EI)·g_n² / (c·γ·r_n)`.

Here `r_n = r·√(n/φ_b)` is the clump's radius, with its hairs packed at
φ_b = π/(2√3) = 0.907. The gap between neighbouring clumps is
`g_n = √n·(d − 2r/√φ_b)`, with d the hairs' spacing in the ferrule. So
`ℓ ∝ n^{3/8}`, the law Roman & Bico give for fibres (`N^{3/8}`, from the
2D lattice) [RB10]. The largest clump joins right at the tips, ℓ = L,
with L the hair out of the ferrule. It holds N = 2n hairs gathered from
a patch `d·√N` across, which is the spacing of the clumps:

  `N = 2·(L⁴·c·γ·r / (9·EI·g₁²·√φ_b))^{2/3}`,  `W = d·√N`,
  `g₁ = d − 2r/√φ_b`.

The clumps form only if a pair can, `L_pair < L`. Otherwise the hairs
stay apart and the brush lays no clump bands. The law doesn't depend on
how far up the hairs the paint reaches, as long as their tips are wet.
The largest clump zips over no length at all. Boudaoud, Bico & Roman
find the clumps spread over `0.3·N_max < N < N_max` [BBR07]. The
engine's bands use `N_max`, the law's value. The engine's bands also
already vary in level from band to band. With W ∝ √N, the smallest
clumps are 0.55 W.

So W ∝ L^{4/3}·γ^{1/3}·E^{-1/3}·r^{-2/3}, given the packing.

**The brushes' data.**

- γ = 35 mN/m (`surface::SIGMA`; linseed oil measures 33–36 mN/m,
  oil_paint_physics.md). Solvent lowers it, to about 25 mN/m for mineral
  spirit. That narrows W by (25/35)^{1/3} = 0.89, which the engine
  doesn't take.
- E: natural hair bristle measures 680×10³ psi = 4.69 GPa at 50 % RH
  [DuP, Table 1; the table's heading says "Mpsi" but its graphs give
  psi×10³]. Water drops it to 410×10³ psi. Oil doesn't swell keratin,
  so the engine takes the dry value. It takes the same value for sable
  and badger, which is an **[est]** since they are keratin like the
  bristle. For synthetics, Tynex 612 nylon measures 580×10³ psi =
  4.0 GPa dry and 420×10³ psi wet; nylon 66 650/230; nylon 6 280/140
  [DuP]. The brochure gives no modulus for PBT (Orel). No preset is
  synthetic, so these aren't used.
- r: whole pig bristles measure 268 µm across mid-shaft and 319 µm at
  the root [M14]. A brush's hair out of the ferrule is the tapered,
  flagged tip end. The engine takes 180 µm across (r = 90 µm), an
  **[est]** between the mid-shaft and the flag. Siberian weasel
  (kolinsky) guard hairs measure 108–152 µm at their widest [Z, in MS].
  A sable brush hair tapers from its belly to a point, so the engine
  takes 90 µm (r = 45 µm), an **[est]**. Badger hair is taken at 100 µm,
  an **[est]**.
- The hairs' spacing: d = r·√(π/φ_t), where φ_t is the share of the
  tuft the hairs fill at the ferrule. The engine takes φ_t = 0.45, an
  **[est]** between loose (0.3) and close-packed (0.6). For the fan,
  whose hairs spread in one thin layer, it takes 0.2, also an **[est]**.
- L: makers publish each size's width and hair length. The engine fits
  a line to them, `L = a + b·width`:

| kind | fitted to | a (mm) | b |
|---|---|---|---|
| flat (hog) | Princeton Umbria flats 7–24 mm (synthetic hog) | 12.18 | 0.763 |
| filbert (hog) | six hog filberts No. 8 (14–17 mm, 23–33 mm long) and da Vinci Forte filberts | 4.95 | 1.562 |
| round (sable, also the stippler) | Escoda Óptimo kolinsky rounds 0–20 | 8.86 | 2.529 |
| rigger | da Vinci 673, Escoda, Princeton Aqua Elite riggers | 23.27 | 2.069 |
| blender (badger) | da Vinci 5080 mottlers 20–80 mm | 26.35 | 0.144 |
| fan | none found: **[est]** | 15 | 0.25 |

The widths and lengths come from the makers' tables collected in [AG],
plus Utrecht, Trekell, da Vinci, Silver Brush and Pro Arte listings for
the hog filberts. `width` is the engine's footprint at full pressure, a
little wider than the ferrule. Tool kinds don't name a hair, so each
kind takes one: hog for flats, filberts and fans, sable for rounds and
riggers, badger for the blender.

**What the law gives** (`coalesce`; the test
`with_the_switch_a_clump_is_as_wide_as_coalescence_makes_it` checks the
first two rows against an independent working of the law):

| brush | L | L_pair | hairs to a clump | W | bands across |
|---|---|---|---|---|---|
| hog flat 8 mm | 18.3 mm | 5.8 mm | 56 | 1.79 mm | 4.5 |
| sable round 4.4 mm | 20.0 mm | 2.4 mm | 720 | 3.19 mm | 1.4 |
| hog filbert 8 mm | 17.4 mm | 5.8 mm | 50 | 1.68 mm | 4.8 |
| hog flat 14 mm | 22.9 mm | 5.8 mm | 102 | 2.41 mm | 5.8 |
| hog fan 8 mm | 17.0 mm | 10.1 mm | 9 | 1.07 mm | 7.5 |
| badger 8 mm | 27.5 mm | 2.8 mm | 1190 | 4.55 mm | 1.8 |
| sable round 1.5 mm | 12.7 mm | 2.4 mm | 210 | 1.74 mm | one clump |

The hog brushes' clumps of 1–2.4 mm, some 50–100 hairs each, sit
between the two old widths for the filbert (0.57 and 2.8 mm). The sable
round's fine, soft hair gathers into one or two clumps across the brush,
the way a wet round draws to a point. How sensitive the hog flat 8 mm's
W is to the estimates:

| φ_t 0.3 / 0.45 / 0.6 | r 60 / 90 / 134 µm |
|---|---|
| 1.06 / 1.42 / 2.49 mm (at L = 15.4 mm) | 2.72 / 2.08 / 1.59 mm (at L = 20.5 mm) |

**In the engine.** With the switch, `clump_factor` lays `width / W`
bands across the brush (`clump_span`), with no clamp. A brush narrower
than one clump lays evenly across. The bands' width depends on neither
the paint's stiffness nor the number of modelled hairs
(`with_the_switch_the_clumps_do_not_depend_on_the_modelled_hair_count`).
The plough that throws paint aside at the brush's sides throws it a
clump's width, still capped at 3 px. `clump_step_mean` still makes the
factors average to 1 over the hairs touching at each step. Measured as
in the section above (12 stroke ids, load 0.6), with the new widths:

| | filbert 8 mm | hog flat 8 mm |
|---|---|---|
| share of load laid, default hand | 15.2 % | 25.7 % |
| spread over the ids, default hand | 0.6 % | 2.1 % |
| spread, steady hand | 0.08 % | 0.5 % |
| marks apart, steady hand (old widths with the switch; before the switch) | 0.098 (0.107; 0.120) | 0.106 (0.082; 0.092) |

The share laid is the same as with the old widths, so the tables
above still hold. The marks differ from id to id about as much as
before.

**Contrast stays an estimate.** A clump lays `1 ± 0.6·stiffen` times the
mean, as before. A clump/gap geometry could set it. The clumps of N
hairs, packed at φ_b and spaced W apart, cover `2r/(d·√φ_b)` = 0.80 of
the brush's width at φ_t = 0.45, whatever N is. In the limit of no
spreading, a clump lays 1.26 times the mean and the gaps lay nothing.
A pressed brush's clumps splay on the canvas, and the paint film under
the brush's belly joins across the gaps. Neither is modelled, so the
bound can't be turned into a contrast. The contrast stays the old
**[est]**, including no clumps in fluid paint (stiffen = 0).

**Survival is the levelling's.** The engine levels open wet paint
(`drying::level_open` → `surface::settle_with_transport`). A ridge at the
bristle scale (λ ≥ 0.25 mm) decays with Orchard's
`τ = 3η(λ/2π)⁴/(σh³)` and freezes below Hester's yield floor
`a_c = τ_y·λ³/(8π³σh)` (surface.rs). For clump ridges 1.7 mm apart in a
25 µm film:

- fluid paint (η = 2 Pa·s): τ ≈ 60 s, so the ridges level well within
  the engine's 900 s set time;
- stiff paint (τ_y = 3000 Pa): a_c ≈ 7 cm, so the ridges freeze.

The levelling therefore already does what the contrast's tie to
`stiffen` does. That tie could go once a derived contrast exists (left
open).

## Left open

- The clumps' contrast (`0.6·stiffen`, no clumps in fluid paint) is
  still an estimate tied to the paint's stiffness ("Contrast stays an
  estimate"). Their width now comes from coalescence.
- The fibre radii, the tuft packing and the fan's hair length are
  estimates. No measured hair count or packing for an artist's brush was
  found.
- Solvent lowers the paint's surface tension, and with it W
  (by (γ/35 mN/m)^{1/3}). The engine takes the oil's γ.

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
- [G14] K. Singh, J. R. Lister and D. Vella, "A fluid-mechanical model of
  elastocapillary coalescence", *J. Fluid Mech.* 745 (2014) 621–646,
  arXiv:1310.4530: states ℓ_ec = (B/γ)^{1/2} after [BRMB04], and the
  "collaborative stiffening" of a bundle (stiffness N·K) that limits its
  size; their spring-block arrays give a largest cluster ∝ K^{-1/2}.
- [RB10] B. Roman and J. Bico, "Elasto-capillarity: deforming an elastic
  structure with a liquid droplet", *J. Phys.: Condens. Matter* 22
  (2010) 493101, section 3 "Wet hairs": the pair law
  `L_stick⁴ = 9d²L_EC²/2` (eq. 6), bundles of N lamellae N/2 times as
  stiff as one, sliding in the liquid, separated by Nd/2 (eq. 8); for
  rods `B_rod = πEr⁴/4` against `γr`, and `L_stick(N) ∼ N^{3/8}`.
- [W14] Z. Wei, T. M. Schneider, J. Kim, H.-Y. Kim, J. Aizenberg and
  L. Mahadevan, "Elastocapillary coalescence of plates and pillars",
  *Proc. R. Soc. A* 471 (2015) 20140593, arXiv:1408.0748: drying arrays,
  hierarchical clusters and their arrest (consulted; no figure here
  depends on it).
- [Z, in MS] Zhang et al., winter guard hairs of Siberian weasels (Tonghe
  forest farm), as quoted in the Mammalian Species account of *Mustela
  sibirica* (vol. 50, no. 966, 2018): mid-back guard hair 108–152 µm
  across (males, mean 126.6 µm).
- [AG] "Paint brush size chart: measured dimensions by shape & brand",
  austingallery.org (2026): makers' widths and hair lengths per size
  (Escoda, Princeton, da Vinci, Rosemary & Co); hog filbert No. 8
  listings at Utrecht (14 × 23 mm), Blick/Trekell (15.9 × 31.8 mm),
  Jackson's (da Vinci 5423 17 × 25, Silver Brush 1103 15 × 33, Pro Arte
  B 14 × 32 mm).
- [DuP] DuPont Filaments, "Filament Performance in Brushes" (brochure,
  2014), Table 1 "Brush Moduli" (50 % RH dry / 100 % RH wet, psi×10³):
  natural hair bristle (horse hair and hog bristle) 680 / 410, Tynex 612
  nylon 580 / 420, Herox 610 nylon 530 / 333, nylon 66 650 / 230, nylon 6
  280 / 140; online at brushtec.com (Filaments_Brochure_web.pdf).
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
