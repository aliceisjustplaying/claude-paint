# The geometry of a plain weave (`--@ relief-prefilter`, `weave.rs`)

## The fault

`surface::linen_um` builds each crossing from sines and steps the weft's
height where a weft passes from under one warp to over the next: from
0.55 of the weft's height to nothing across a cell edge. A thread can't do
that. It runs over and under the threads it crosses on a smooth, crimped
path, set by the threads' sizes and spacing and the tension it was woven
at, and the surface paint meets is the top of those threads.

## Peirce's geometry

F. T. Peirce, "The geometry of cloth structure", *J. Textile Inst.* 28,
T45–T96 (1937). A thread bends round each thread it crosses at the
distance their heights make together, half of D = g₁ + g₂ (g the threads'
heights; diameters for round threads), and runs straight between, tangent
to the arc over one crossing thread and the arc under the next. For a
thread crossing threads spaced p apart, with crimp height h (how far its
axis rises and falls) and length l between two crossings:

- p = (l − Dθ) cos θ + D sin θ
- h = (l − Dθ) sin θ + D (1 − cos θ)
- h₁ + h₂ = D (warp and weft together)
- crimp c = l/p − 1

Eliminating l gives the angle of the straight run directly,
p sin θ + (D − h) cos θ = D: the tangent to the two arcs. Given both
threads' crimps and spacings, the equations fix D and h₁: the warp's h₁
for its crimp past wefts s_we apart, plus the weft's h₂ for its crimp past
warps s_wa apart, must equal D (`weave::peirce_weave`). The equations are
unchanged by scale, so D and h are in proportion to the spacing for given
crimps.

`weave.rs` solves the tangent for each pair of crossings with each
crossing's own D (thick and thin threads, slubs: D grows with the two
threads' thickness there, and h₁ = f₁D with f₁ the warp's share). A
thread's path is then the arc round each crossing thread's axis, the same
arc from either side, so its height and slope are continuous everywhere.
The thread's top is that path offset by its section's half-height. An arc
offsets to an arc about the same center and a straight run to a straight
run q/cos θ higher, so the offset is as smooth as the path. The paint
meets the highest thread at each point, or the cloth's underside in a
hole.

Later models refine the section: Kemp's racetrack (A. Kemp, "An extension
of Peirce's cloth geometry to the treatment of non-circular threads",
*J. Textile Inst.* 49, T44–T48, 1958) and elastica paths for threads
whose bending stiffness sets the crimp. Peirce's arcs and lines, with
measured crimps, give the measured yarn heights within 20% and cloth
thickness within 35% (below), so they are used as they are.

## The canvases measured

**Nineteenth-century painting canvases.** A. Iaccarino Idelson,
O. K. Bergsma and R. M. Groves, "Quantitative morphological analysis of
warp and weft yarns in historical woven structures", *Studies in
Conservation* (2025, vol. 71 no. 2, 2026), doi:10.1080/00393630.2025.2491250;
open preprint at doi:10.5281/zenodo.14836221. Across 26 textiles, crimp
was measured along the neutral axis of drawn yarns, yarn width on the
cloth's face and yarn thickness on drawn yarns seen edge-on. The eight
linen painting canvases of 1845 to the late 19th century (Tables 2–4, 7):

| painting | warp × weft /cm | crimp warp / weft | width warp / weft, mm | thickness warp / weft, mm | cloth, mm |
|---|---|---|---|---|---|
| Postiglione 1845 | 8 × 8 | 5.16 / 3.27% | 0.60 / 0.96 | 0.38 / 0.37 | 0.66 |
| Dehodencq 1870 | 17 × 11 | 15.29 / 2.34% | 0.71 / 0.51 | 0.38 / 0.32 | 0.32 |
| Gélibert 1881 | 24 × 18.7 | 11.92 / 3.56% | 0.49 / 0.41 | 0.20 / 0.23 | 0.45 |
| Auguin 1885 | 25.3 × 25 | 12.96 / 4.65% | 0.39 / 0.33 | 0.24 / 0.29 | 0.22 |
| Alleaume 1887 | 24 × 21.7 | 11.36 / 6.76% | 0.44 / 0.35 | 0.16 / 0.17 | 0.44 |
| Sauzeau 1893 | 22 × 20 | 13.65 / 5.73% | 0.48 / 0.40 | 0.23 / 0.23 | 0.37 |
| Müller, late 19th c. | 31.3 × 29 | 10.26 / 5.70% | 0.35 / 0.30 | 0.14 / 0.14 | 0.26 |
| Furcy de Lavault, late 19th c. | 23.5 × 22.3 | 14.21 / 3.03% | 0.48 / 0.36 | 0.17 / 0.22 | 0.38 |

**Modern artists' linen.** P. Chiriboga, *Finite element modeling of
vibrations in canvas paintings*, PhD thesis, TU Delft, 2013, Tables
3.1–3.3 and 4.1. Sections were cut through resin-embedded cloth, crimp
measured by ASTM D3883, thickness by caliper:

| linen | warp × weft /cm | crimp warp / weft | warp section e × g, mm | weft section e × g, mm | cloth |
|---|---|---|---|---|---|
| Rijn (Claessens 066) | 17 × 14.5 | 19.60 / 0.92% | 0.574 × 0.196 | 0.352 × 0.250 | 0.562 mm |
| Waal (Claessens 013) | 23 × 23 | 18.88 / 1.68% | 0.406 × 0.115 | 0.278 × 0.167 | 0.396 mm |
| Amazone | 20 × 16 | 15.04 / 0.96% | 0.500 × 0.167 | 0.373 × 0.262 | 0.369 mm |

The cut sections are lenticular, wider than tall: the warp about three to
one, the weft one and a half. Warp crimp above weft crimp is the rule. The
historical canvases had it in 24 of 26, by 50% on average. The modern
linens are far more unbalanced (weft crimp about 1%).

## What the weave takes from them

- **Crimp** (`WARP_CRIMP`, `WEFT_CRIMP`): the eight historical linens'
  means, warp 11.85% and weft 4.38%. The engine paints pictures of that
  century.
- **D and the crimp split**: Peirce's equations for those crimps at the
  canvas's own spacings. For 16 × 15 threads/cm, D = 0.493 mm,
  h₁ = 0.313 mm and h₂ = 0.179 mm (f₁ = 0.64).
- **The heights' share** (`WARP_SHARE`): the warp's share of the two yarn
  heights on the eight, 0.44–0.54, mean 0.486.
- **Widths** (`WARP_WIDTH`, `WEFT_WIDTH`): yarn width on the cloth's face
  over spacing on the eight. Warp 0.48–1.21, mean 1.023: the warps lie
  edge to edge and press into each other. Weft 0.56–0.87, mean 0.769.
- **Section shape**: a lens of that width and height. A lens as wide as it
  is tall is a circle, so Peirce's round thread is the special case.
  Round threads, as the task first named them, can't carry the measured
  sections. With the modern linens' mean diameters (e + g)/2, Peirce gives
  cloth 0.92–0.99 mm thick (measured 0.37–0.56) and warp crimp of 19–45%,
  and Waal can't be woven with them.

D taken from the drawn yarns' measured heights instead of from the crimps
fails for the historical canvases. Their sum often exceeds the cloth's
thickness (Dehodencq 0.70 mm in 0.32, Auguin 0.53 in 0.22): drawn yarns
spring back from the flattening the cloth gave them. The crimps survive
drawing (the drawn yarn keeps its waves), so the weave takes D from them.

What Peirce gives each canvas from its own counts and crimps alone, set
against what was measured on it (`peirce_matches_the_measured_canvases`;
the threads share D as that canvas's measured heights do):

| canvas | D, Peirce | yarn heights | cloth, Peirce | measured |
|---|---|---|---|---|
| Postiglione | 0.708 | 0.75 | 0.75 | 0.66 |
| Gélibert | 0.361 | 0.43 | 0.42 | 0.45 |
| Alleaume | 0.358 | 0.33 | 0.39 | 0.44 |
| Sauzeau | 0.398 | 0.46 | 0.45 | 0.37 |
| Müller | 0.255 | 0.28 | 0.28 | 0.26 |
| Furcy de Lavault | 0.333 | 0.39 | 0.38 | 0.38 |
| Rijn | 0.497 | 0.446 | 0.64 | 0.562 |
| Waal | 0.335 | 0.282 | 0.39 | 0.396 |
| Amazone | 0.403 | 0.429 | 0.49 | 0.369 |

(mm.) D comes within 20% of the measured heights on all nine, and the cloth
within 35%. The caliper presses the cloth, which may be part of Amazone's
gap. Dehodencq and Auguin are left out because their drawn yarns can't be
compared (above). Across all eleven canvases with crimps, D is 0.72–0.83
of the mean thread spacing, except the hand-woven Postiglione (0.57).

Taking the eight canvases' mean crimps and proportions for every thread
count is an assumption. It holds across 8 to 31 threads/cm, but each
canvas has its own crimp (the warp's runs 5–15%).

## What the relief is now

At 16 × 15 threads/cm (the Marly canvas), read finely, the relief varies
by 68 µm rms, from 0 to 588 µm above the cloth's underside. Its mean is
435 µm. The warp crowns stand highest, and the warp threads lie edge to
edge, so open holes are 0.3% of the area. Averaged over 0.38 mm pixels it
varies by 23 µm rms. The old weave varied by 6.4 µm: its crossings with
the warp on top and with the weft on top stood about equally high, so its
over and under (a period of two threads, which the pixels resolve) nearly
cancelled. The warp knuckles are what show now.

`crimped_weave_is_continuous` reads the relief along lines at a
two-thousandth of a thread. Along a thread, and across a crease where two
threads meet, the surface moves less than a slope of 4 allows, and eight
times less at eight times finer. It drops only at a thread's edge where
the thread lies over a lower one or a hole, and by no more than the
thread's axis stands above the floor.

## Fibre fuzz

Loose fibres on the threads roughen their tops by about one elementary
flax fibre's diameter, 10–25 µm (H. L. Bos, *The potential of flax fibres
as reinforcement for composite materials*, PhD thesis, TU Eindhoven,
2004). The relief takes 15 µm from lowest to highest, on value noise of
about a third of a thread. This is an **estimate**: no measurement of a
linen yarn's surface roughness was found, and the fibre's diameter only
bounds it.

## Holes

Where no thread lies, the paint meets the cloth's underside: the lowest
any thread's top can be, or the nominal cloth's underside if that is
lower. A hole goes through the cloth. Size or a ground fills it in a real
painting, and the engine's ground and paint level over it as over any
hollow.

## Thread counts far from the measured ones

A cloth can't be woven tighter than its threads allow (Peirce's jammed
limit: the straight run between two arcs has no room). Where the thickest
crossing doesn't fit, `Weave::new` makes the threads thinner until it
does. The measured counts never reach this. Where the crimps can't be met
at all (counts far apart, such as 4 × 60 per cm, which the easel allows),
D is taken as 0.78 of the mean spacing, as measured.

## Cotton duck

The cotton in Chiriboga's table is a rib weave (41 × 20.5 threads/cm), not
the plain-woven duck Frankenthaler and Louis used. Two of the historical
canvases are partly cotton or hemp. No measured sections or crimp for
numbered cotton duck were found. The US standard numbers duck by weight
(number = 19 − ounces per linear yard 22 inches wide; #10 ≈ 15 oz/yd²,
#12 ≈ 11.5 oz/yd²), and vendors give no counts or crimp. The raw canvas
(`raw="cotton duck"`) lays the same linen geometry under its cloth. An
open question.

## Cost

Each point needs the four crossings round it. Reading their thicknesses
(value noise) was most of the cost, so `weave::CrossTable` reads each
crossing once for the window being laid, with the same values bit for bit.
A point then costs 104 ns on one core, against 85 ns for `linen_um` (best
of three runs of 2 million points; 224 ns without the table). The
footprint still reads 8 points a thread, stratified
(`surface::SAMPLES_PER_THREAD`). An exact or analytic footprint average
isn't available, because the relief drops at the threads' edges.

At 8 a thread the footprint averages come within 2.5 µm rms of the weave
read 48 times a thread. That is 11% of the 23 µm the averaged relief
varies by, where #350 had 1.5 µm in 6.4 (23%). At 4 a thread the error is
5.9 µm, at 12 it is 1.1 µm. The linen is about 1.2 times as costly to lay
as before, paid once when the canvas is laid.

Pixels wider than a thread use a table of the weave's mean against its
threads' thicknesses (`weave::WeaveMean`, 7 × 13 entries, each the mean of
a period read 48 × 48 points a thread), built in about 10 ms.
