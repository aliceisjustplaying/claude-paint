# The knife's ridges: a blade's load and how tall paste stands

Why `--@ knife-slump` (`Canvas::with_knife_slump`) exists, and where its
numbers come from. Found on cassatt7's cloth, whose gallery-light save
showed diagonal "rain": cast shadows from knife ridges 10–80 mm tall
(up to 3300 coats in one 0.25 mm pixel at 2400 px).

## What went wrong

- **The blade's load.** `Knife::full` is one bead: a strip as deep as twice
  the blade's length and 12 coats (300 µm) thick, about a millilitre on a
  4 cm blade. `k:load` added its amount on top of whatever the blade still
  carried, so a painter loading before each pull (as painters do) carried
  several full loads after a few pulls (9 on cassatt7's cloth knife). A
  blade can't hold more than its bead: what it would take beyond that stays
  in the pile. With the switch a load tops the blade up to `full` and no
  more.
- **Ridges a pixel wide.** The paint a pull presses out past the blade's
  ends goes into one pixel beside each end, and the share `lift` of the bead
  into a strip three pixels deep where it lifts: pixels, not millimetres, so
  at 2400 px the same paint stood three times as tall as at 800 px, and a
  knife pulled over its own ridges stacked them further.

## How tall paste stands

A paste with a yield stress τ holds its shape until the shear stress in it
passes τ. Under its own weight, the shear at the bottom of a layer h deep
whose surface slopes by |∇s| is ρ·g·h·|∇s|, so a heap at rest has
ρ·g·h·|∇s| ≤ τ everywhere: the perfectly plastic profile, Nye's for an ice
sheet, h(x) = √(2τ(L − x)/(ρg)) across a ridge of half-width L [N51]. The
slump test of a yield-stress paste is the same balance: a paste stops
spreading where its weight's shear falls to its yield stress, and its
final height measures τ [P96; RC05]. The axisymmetric heap of volume V
stands √(2τR/(ρg)) tall, V = (8π/15)·√(2τ/(ρg))·R^(5/2) (integrating Nye's
profile around its axis).

`slump` (crates/paint/src/bristle.rs) applies this to what a knife pass
touched: paint moves from a pixel to a neighbor whose surface it overtops
by more than τ·dx/(ρ·g·h), half the excess at a time, until none does
(within 1 µm). It works in metres, so a heap settles to the same shape at
any pixel pitch once the pixel is small against the heap (0.25 mm and
0.125 mm pixels agree within 1% on a 50 mm³ heap of τ = 830 Pa; 0.5 mm
pixels, a sixth of the heap, stand it a fifth lower).

- **τ**: each pixel's paint's, from its stiffness (`surface::rheology_at`,
  engine 4 and later: 5 Pa for medium-rich paint to some 3000 Pa for stiff
  paste, notes/research/oil_paint_physics.md), divided by how fluid its film
  still is (`drying::fluid`: setting paint holds).
- **ρ**: oil at 0.93 g/cm³ (notes/research/pigment_oil.md §2) by the
  paint's oil share of its volume (`Prop`'s 8), the rest pigment at
  **2.5 g/cm³ (estimate)**: a pixel's paint doesn't carry its pigments'
  densities. 2.5 is about ultramarine's (2.35) and yellow ochre's (2.8)
  (pigment_oil.md §3); lead white (6.81) and vermilion (8.1) are heavier
  and would stand a little lower. The estimate is light, so a heap stands
  at most as tall as it may.
- A knife's slab, a few hundred µm thick, overtops its neighbors by far
  less than it may (tens of mm, at the slab's depth): it stays as laid.
  Only ridges and beads move.

## Sources

- [N51] J. F. Nye, "The flow of glaciers and ice-sheets as a problem in
  plasticity", Proc. R. Soc. Lond. A 207, 554–572 (1951).
- [P96] N. Pashias, D. V. Boger, J. Summers, D. J. Glenister, "A fifty cent
  rheometer for yield stress measurement", J. Rheol. 40, 1179–1189 (1996).
- [RC05] N. Roussel, P. Coussot, "'Fifty-cent rheometer' for yield stress
  measurements: from slump to spreading flow", J. Rheol. 49, 705–718 (2005).
