# A touch of a loaded brush: how much paint a press and lift lays

**Scope.** `b:touch` presses a loaded brush tip onto the canvas and lifts it,
optionally dragging it a little while down (`bristle::touch_on`). This note
explains why the `--@ touch-split` switch sets what a touch lays, and where
its numbers come from. Values tagged **[est]** are estimates.

## The fault it fixes

Before the switch, each hair of a touch laid a fixed film: `TOUCH_FILM · lay`
coats (1 coat = 25 µm) over the patch it stands for, times the load's
fullness and `0.4 + 0.6·p`. That amount does not depend on how much paint
the brush holds, and it does not grow with the drag. With an 8 mm filbert
(`width=40` on a 200 mm canvas), loaded to 0.6 and pressed at 0.8, on main
4bbbbec3 (`easel run --dump-state`, 400 px):

| mark | dry ground: paint laid | on ~9 coats of wet white | on ~1 coat of wet white |
|---|---|---|---|
| touch | 0.16 mm³ (0.6 % of the load) | 0.16 mm³ laid, net −0.24 mm³ (it lifts more than it lays) | grey-blue, sRGB 126,153,168 |
| touch, drag 60 units (12 mm) | 0.15 mm³ | net −0.28 mm³ | 131,159,167 |
| stroke, 2 points 60 units apart | 5.4 mm³ (~10 % of the load) | +3.4 mm³ | 37,75,166 |

The touch also lifts wet paint as a stroke does. A filbert's `pickup` is
0.18, so it can lift up to 18 % of the wet column under it, which is far
more than its fixed coat. On wet paint it mostly lays back the underpaint
it has just picked up. That is the reported "a touch lays almost nothing on
wet paint".

## Physics

**Film splitting.** When a wet film between two surfaces is pulled apart,
it splits inside the liquid, and each surface keeps part of it. Printing
ink transfer measures this. In Walker and Fetsko's transfer equation, the
excess ink beyond what the receiving surface immobilizes splits in a
constant fraction `f`, usually about 0.5 [WF55; TUD]. A rough surface puts
`f` a little below 0.5 [ABB]. Roll coating shows the same split at the exit
of a nip, where the film divides between the two surfaces at a stagnation
line [CMS86].

**A slide and a lift split the same face.** A brush moving along the canvas
leaves paint by the same split, made continuously where its contact face
leaves the canvas behind it. A press-and-lift makes the split once, over the
whole pressed patch. In both cases the canvas receives the split share of
the film on the same contact face. So a pressed patch receives the film a
stroke leaves at each point it passes, which is the film the stroke lays
where it starts.

**Contact time doesn't change the split.** A press holds the tip down for
some tenths of a second. A sliding brush covers a point only for the time
it takes to pass one patch length, some 10 ms at 100 mm/s. The film between
a pressed hair and the canvas thins by squeeze flow. For a Newtonian film
of viscosity μ between discs of radius R under pressure P, Stefan's law
gives the time to thin to h as about `3μR²/(4Ph²)` [ESB05]. With μ ≈ 20 Pa·s
for oil paint at brushing shear rates [est, from the range in
oil_paint_physics.md §1], R ≈ 50 µm (a hair's contact), P ≈ 10 kPa [est]
and h ≈ 100 µm, that is about 0.4 ms. Both contacts are long compared with
that, so the split happens on a film that has already squeezed out, and a
longer touch lays no more and no less. A paint with a yield stress stops
thinning at a finite thickness [ESB05], which again does not depend on
dwell time.

**Wet on wet.** Pressed into wet paint, the brush's paint and the wet film
join in the contact. On the lift, the film splits inside the combined
layer, so the brush takes some of the wet paint with it. This is the
back-transfer that printers measure as imperfect wet trapping (the second
ink takes less well on a wet first ink than on paper; Preucil's apparent
trap measures it) [P53]. The engine already models this as pickup, by the same law for a
touch and a stroke. The switch changes only what the touch lays.

## The law (`touch_on`, with the switch)

The stroke's law (`exchange`) says that a hair moving a distance x lays the
share `1 − exp(−x/run)` of its paint. `run` is the tool's e-folding length,
which is the engine's own existing estimate and is not changed here.

A touch lays the film a stroke lays at each point it passes. The stroke's
film over a swath of width W is `N·liquid/(run·W)`, from N hairs each laying
`liquid/run` per unit of travel. Spread over the pressed patch of area A,
that film is `N·liquid·(A/W)/run`. So each hair lays the share a stroke's
hair lays while moving `ℓ = A/W`, the patch's mean length. For a footprint
of half-width `half` and the brush kind's root area,
`ℓ = root_area · half / 2`. A drag of length d slides the patch further, so:

    laid per hair = load at the touch's start × (1 − exp(−(ℓ + d)/run))

This is spread over the press/hold/lift steps by how far the hair is
pressed at each step, as before. The law needs no new constant. The old
`0.4 + 0.6·p` factor (an unsourced estimate) is gone. Pressure now acts
through the patch: a lighter press makes a smaller `half`, a shorter ℓ and
less paint.

## Results (same scene, with the switch)

| mark | dry: laid | dry: mean film | wet (~1 coat of white): sRGB |
|---|---|---|---|
| touch | 1.22 mm³ | ~30 µm | 56,94,168 |
| touch, drag 60 | 7.65 mm³ | | 28,62,162 |
| stroke 60 | 5.2 mm³ | ~38 µm | 37,76,166 |

A pressed touch now lays about the film a stroke starts with. A dragged
touch lays more than a stroke of the same length, between 1.4 and 2.4 times
depending on orientation and the stroke's ramps. The stroke falls short of
its own law's share (about 10 % of the load over 60 units, where the law
gives 19 %) because its trailing hairs pass over the film the leading hairs
have just laid and lift part of it. The hairs of a pressed touch barely
pass over each other's paint. Whether a stroke's trailing hairs should lift
that much is a question about the stroke, not the touch. It is left open
here.

## Sources

- [WF55] W. C. Walker and J. M. Fetsko, "A concept of ink transfer in
  printing", *TAGA Proceedings* (1955) 139; also *Am. Ink Maker* 33 (1955) 38.
- [TUD] Dissertation, Lehrstuhl für Werkstoffkunde, Universität Dortmund
  (eldorado.tu-dortmund.de): the Walker–Fetsko parameters, with `f` "usually
  50 %".
- [ABB] S. Abbott, *Practical Coatings*, "Walker-Fetsko"
  (stevenabbott.co.uk/practical-coatings/Walker-Fetsko.php): `f` is the
  split ratio for thick films, below 0.5 on rough surfaces.
- [P53] F. Preucil, "Color and tone errors of multicolor presses", *TAGA
  Proceedings* (1953): the apparent trap of an ink printed onto wet ink.
- [CMS86] D. J. Coyle, C. W. Macosko and L. E. Scriven, "Film-splitting
  flows in forward roll coating", *J. Fluid Mech.* 171 (1986) 183–207,
  doi:10.1017/S0022112086001416.
- [ESB05] J. Engmann, C. Servais and A. S. Burbidge, "Squeeze flow theory
  and applications to rheometry: a review", *J. Non-Newtonian Fluid Mech.*
  132 (2005) 1–27.
