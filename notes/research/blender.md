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
