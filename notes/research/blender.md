# A clean blender over wet strokes (`b:blend`, gestures-only)

## The fault

`b:blend` (gestures-only, #348) wipes the brush (nine tenths of what it
holds) and drags it through the wet paint (`Canvas::blend_stroke`). Over
fresh wet strokes on dry paint it barely softened them, while a rag in
spirits lifted them cleanly. A painter's soft blender (badger, or a clean
dry hog) drawn lightly across wet strokes picks paint up off the ridges and
lays it down again a little further on, softening the edges within a pass or
two.

Measured on a tiny scene (`a_blender_softens_wet_strokes_on_dry_paint_and_keeps_their_paint`:
a dark band four filbert strokes deep over a dried light underlayer, on
linen at 0.375 mm a pixel, the stroke film on), with two passes of strokes
3 mm apart across the band at pressure 0.4:

| | edge after pass 1, 2 (from 4.8 mm) | canvas keeps | rag took |
|---|---|---|---|
| filbert 9 mm, before | 6.8, 0.8 mm | 38% | 61% |
| badger 9 mm, before | 9.5, 10.2 mm | 46% | 53% |
| filbert 9 mm, after | 11.0, 17.6 mm | 70% | 29% |
| badger 9 mm, after | 9.8, 15.0 mm | 61% | 38% |

The blender lifted like a rag, and the filbert's edge hardened again.

## Why

Of the four causes the brief named:

- **Wiped hairs taking almost nothing:** no. A wiped hair is hungry: it
  picks up `pickup · (0.35 + 0.65 · spent^1.5)` of the wet paint it
  touches, about 0.9 of the tool's `pickup` (0.18 filbert, 0.3 badger), up
  to the stroke's floor.
- **The pressure, the contact on a smooth dry layer:** at pressure 0.4 the
  hairs touch a smooth dry layer at about half weight. That slows what they
  lay there but doesn't cause the loss.
- **The film-law cap:** no. With the stroke film off the blender lost as
  much (40–75% in a probe of the same scene).
- **What causes it:** a hair lays what it carries at the rate a full load
  runs down, `1 − exp(−travel / run)`, where `Tool::run` is how far a
  brushload lasts: 66 mm for this filbert (`40 + 6 · width` units), 4.5 mm
  for the badger (30 units). Paint the blender picked up stayed on the hairs
  for the rest of the stroke, and the wipe before the next stroke took it.

## The law

Paint a clean hair picks up sits on the part of the hair lying on the
canvas. It doesn't soak up into the tuft during a stroke: by Lucas and
Washburn (E. W. Washburn, Phys. Rev. 17, 273, 1921), paint wicks up the
gaps between hairs a length L = √(γ r t cos θ / 2η). With γ = 0.033 N/m
(linseed oil), r ≈ 45 µm (gaps about a hair's radius: an estimate), a
contact of t ≈ 12 ms (2.5 mm of hair at 200 mm/s) and η ≈ 1 Pa·s (tube paint
at brushing shear, `stroke_film::rheology`), L ≈ 0.1 mm, against hairs
10–20 mm long. Its yield stress holds it back further. So the hairs lay what
they picked up over the length of hair lying on the canvas, not over a
brushload's run. `blend_contact` takes that length as the hairs' trail,
`length · p` (the geometry `drag_on` already bends them by), which is about
2.5 mm for these 9 mm brushes at pressure 0.4. It is an estimate: an
elastica pressed against the canvas would give the contact from the hair's
stiffness and the force.

Only `b:blend` changes, under the gestures-only switch. The blender pass
(`blend(m)`, `Handling::blender`) and every loaded stroke keep the run of a
full load.

## What stays

The blender still loses some paint: what its hairs hold at the end of each
stroke, which the next wipe takes (29–38% over two passes here, about six
strokes over each point). The guide's blender pass takes about 15% in one
pass and 40% in three, the same order. No paint is made: what the canvas
loses is what the rag and the brush hold.

## The ladder (cassatt8)

### The fault

In cassatt8 (*Children on the Sands*, chunk 22) the painter drew fourteen
short `b:blend` strokes of a clean 8-unit filbert (5.2 mm on the 650 mm
canvas) at pressure 0.4 across a wet edge between a white and a lavender
down the far child's back: each 35 units long, 9.9 apart. They left a
regular ladder of dashes, which the painter painted over.

Reproduced on a 130 mm canvas at the same scale in mm (strokes, brushes and
spacing five times as many units; 480 px, about the painting's 3.7 px/mm):
dense tube whites, about 90 µm wet, laid edge to edge, then the same
strokes. Measured down columns across the edge, the component of the wet
film at the strokes' spacing went from under 1 µm to 34 µm. Under each
stroke the film fell to 50 µm, a rim of 144 µm stood along each side, and
the paint between the strokes stayed at 97 µm. A 5 mm badger in place of
the filbert did the same (22 µm, rims of 131 µm). In the colour, each
stroke's end was a pale patch and each start a spot of lavender on the
white.

### Why

Switched off one at a time (a probe, not kept):

- **The plough:** the cause of the rims. Off, the component at the
  spacing fell from 34 to 9 µm. The hairs pushed paint aside wherever they
  touched it, however thin the film and however light the bed.
- **Each stroke's lift** (#348's stroke end, still acting on a blender):
  the pale ends. A loaded brush's lift splits the film in the gap under its
  bed, the brush's own paint. Each pixel under the bed owes half its film
  and the hairs' paint pays it. A wiped blender's gap holds next to none, so
  the lift took up half the wet film under the bed at every stroke's end.
- **The wipe's residue:** the spots at the starts. The wipe leaves a tenth
  of what the brush held, mostly what the last lift took up, and the hairs
  laid it over their contact, a few mm, where the next stroke started.
- **The weave fill and the stroke start's press:** not causes. A blender's
  stroke start already takes no press film (above).
- **The spacing:** part of what the painter saw, and not the engine's. At
  pressure 0.4 a filbert's mark is 0.67 of its width, 5.4 units, so strokes
  9.9 apart leave 45% of the edge untouched between them.

### The law

**What the bed pushes aside.** A blender's bed is a slider like any brush's
(§3 of stroke_film.md). It brings no paint, so the film under it is the
canvas's own paint. Its rheology, from the pixel's paint and solvent, sets
the film the bed leaves, the outlet film 0.69 h₀ with
h₀² = 0.16 L(ηU + τh₀)/p. Paint thinner than that passes under the bed: the
bed rides on it and its hairs push none of it aside. Paint thicker than that
can't pass, and the hairs push out the excess and no more
(`stroke_film::blend_plough`). For tube lead white (440 Pa, 4 Pa·s) at the
steering law's 189 mm/s:

| bed, pressure 0.4 | p | film it leaves |
|---|---|---|
| badger 5.2 mm | 17 kPa | 139 µm |
| hog filbert 5.2 mm | 43 kPa | 49 µm |
| badger 9.1 mm | 5.6 kPa | 238 µm (the hairs' meniscus) |

So a soft blender drawn lightly through a dense coat furrows none of it,
and a hog pressed into it furrows it: brush marks show where the paint is
thick and the hairs stiff.

**The stroke's end.** The lift's split is of the paint the bed brought down.
A wiped bed brought none: its hairs lie in the canvas's own paint and leave
it carrying what they picked up, as they slid. A blender stroke therefore
has no lift press. Its hairs slide to the path's end, where the hand lifts
and they straighten (`reach_end`).

**The contact as the hand eases.** What a hair picked up lies along the part
of it on the canvas. As the pressure ramps down at the stroke's end, the
part that lifts takes its share up with it. So the part still down lays its
share at the same rate per distance as before: over the contact at the
stroke's firmest, not the shrinking contact. Over the shrinking contact,
each stroke's last few mm took all the hairs held, a ridge at every end
(33.6 µm at the spacing near the ends in the test below, 4.6 µm after).

**What the rag leaves.** A rag wipes the outside of the tuft and its tips.
What it leaves, a tenth here, is held between the hairs near the ferrule.
By the Washburn length above (about 0.1 mm in a stroke's contact time),
paint doesn't wick out of the tuft onto the contact within a stroke. A
blender stroke holds that paint back and feeds it to the hairs over a
brushload's run (`Tool::run`, which sets how a loaded brush's tip is fed),
not over their contact. Taking `Tool::run` as the feed's length is an
estimate: it is the engine's measure of how fast a load reaches the tip.
Nothing is lost: what is still held back at the stroke's end stays in the
brush for the next wipe.

### Measured

`short_blender_strokes_soften_a_wet_edge_without_a_ladder` (bristle.rs) sets
up a wet edge of two dense paints laid by a 9 mm hog filbert at pressure
0.8 (about 100–140 µm wet). It then draws short strokes of a 6 mm badger at
0.4 across the edge, half a mark width apart, so every spot is worked
twice. Components at the strokes' spacing, columns x = 426–576:

| | film | luminance (edge contrast 0.513) | edge 25–75% |
|---|---|---|---|
| before | 8.7–43.5 µm; a 171 µm pile at the ends | 0.003–0.006 | 45 → 138 units |
| after | 1.0–6.5 µm; 114 µm at the ends | 0.003–0.011 | 45 → 117 units |

The test asks for under a tenth of the film and under 3% of the edge's
contrast at the spacing, a wider edge, and no paint made (what the canvas
loses, the rag and the brush hold, to 1e-3). With strokes 0.7 of a mark
apart, so some spots are worked once and some twice, the luminance
component is 0.015, from the coverage, not from the strokes' ends.
`a_stiff_blender_pressed_into_dense_paint_furrows_it`: a 5 mm hog filbert
at 0.8, strokes a third overlapping, leaves 44–47 µm at the spacing in the
same paint (its bed leaves about 40 µm).

The older test, `a_blender_softens_wet_strokes_on_dry_paint_and_keeps_their_paint`,
draws a 9 mm filbert at 0.4 over a 40 µm band. The bed leaves 118 µm there,
so the filbert now pushes none of the band aside. Its first pass moves the
band's toe out but leaves the rim of the band's first stroke, which sets the
edge's 90% point: 4.5 → 5.0 mm. Its second pass softens the edge to 14.3 mm.
Before, its first pass "softened" the edge to 10.2 mm by ploughing the band
down into a pile 210–316 µm thick at the strokes' ends. The test now asks
the filbert's first pass to widen the edge at all, and the badger's to
widen it 1.5×, as before. Both must still more than double it in two passes.

Only `b:blend` changes, and only under the gestures-only switch.
