# r11_study3: one trunk against sky and snow

Program: `paintings/src/bin/r11_study3.rs`. Renders: `out/r11_study3.png`
(1000px), `out/r11_study3_full.png` (3200px).

## The study
A small portrait canvas (260 mm wide, aspect 0.75, so about 26 × 35 cm,
on `Style::friedrich()`'s Dresden ground). One old oak trunk, nearly a
silhouette, rises out of evening snow and leaves the top edge. A heavy
limb goes off to the right with twigs into the sky, and a broken stub
points left. The sky runs from a cool gray-blue through a pale greenish
band to a warm glow just above the far snow line. Two low copses sit on
the horizon. The sun is low behind the tree, so the trunk throws its
shadow toward us across the snow.

The subject is the two meetings:
- **Trunk against sky**: crisp, slightly ragged bark edges. The left edge
  takes a cool rim of sky light. The limb's top is lit and its underside
  dark. A thin crust of snow lies on the limb and the stub. Twigs taper
  to hairlines.
- **Trunk into snow**: the root flare swells at the foot. The contact
  line is irregular: a drift on the windward left and a scoop in the lee
  on the right. There is a crease of cool shade right at the bark, and
  the snow's lip is pushed over the dark in places. The shadow comes
  toward us, and dry grass stalks come up through the snow, laid last.

## Working method (in stage order, as he worked)
1. `drawing`: a pencil drawing (HB for the trunk edges, 2H for the limb,
   stub, horizon and contact line), two passes via `sketch_marks`
   [CATS pp.128, 131].
2. `sky`: a thin lay-in in elbow strokes from a family palette (lead
   white, cobalt, ochre, red earth, umber), a shade duller than the goal.
   Then a badger pass top to bottom, a stipple into the wet paint, a
   dry, and a finer, lighter stipple dense toward the glow [NG p.56;
   CATS p.127].
3. `snow`: body color along the surface. The color field carries the
   modeling: a drift height field in perspective (longer and flatter
   far off), lit where it rises away from us toward the glow and shaded
   where it falls toward us. **The cast shadow is mixed into the snow
   as it is laid** (shadowed snow is its own pile, as a painter mixes
   it). The far copses are stippled small and cool. Everything dries
   before the tree.
4. `trunk`: body strokes up the trunk (aimed at a bark color field with a
   cool left rim and the darkest dark on the right), then the limb and
   stub, with strokes along them and the top lit. The mask stops at the
   snow contact line.
5. `bark`: into the soft paint, a pointed round draws short wavering
   fissures and faint ridges, plus short cross cracks (oak bark is
   blocky). Next come a sparse stipple of pale lichen on the sky side, a
   stipple of snow light bounced up onto the foot, a broken cool rim down
   the left edge, and pale split wood on the stub's break.
6. `twigs`: recursive hand gestures (a pointed round, then a rigger),
   each pressed where it leaves and lifted off to its tip.
7. `foot`: along the contact line, a crease of cool shade, then short,
   broken, flat strokes of lead white for the lip, and rounded strokes
   for the drift on the left. A snow crust goes along the top of the limb
   and the stub.
8. `shadow`: a bluer scoop in the lee, stippled. Then long low drift crests
   in the foreground, each with its furrow, clipped off the trunk. A thin
   warm line of glow runs along the far snow's edge.
9. `grass`: dry stalks through the snow, fine upturning rigger flicks
   with a few seed-head touches [NG p.56].
No varnish and no cracks: `c.dry()` and `c.relief(st.relief)`, then save.

## What I tried and changed (log)
- v1: the snow was pale blue-white, lighter than the glow. That reads as
  a digital gradient, not evening. I darkened and warmed it so the land
  sits below the sky in value.
- v1: the trunk's lower third came out gray and speckled because I laid
  the trunk into snow that was still wet (`wait(90)`), so the brush
  dragged lead white up the bark. That's physically right and a
  painter's mistake. Fix: `c.dry()` the snow first.
- v1: lit snow "crests" as a masked body pass gave cut-out white blobs.
  Dropped them. The modeling went into the snow's color field, plus a
  few hand-laid crest and furrow strokes.
- v2–v3: the cast shadow as a body pass with `color_over`, then as a
  `st.glaze()` over dry snow. Both read as a separate pasted-on path
  (blocky patches, and a saturated blue that pooled into dark crescents
  in the snow's impasto hollows). Fix: mix the shadow into the snow
  lay-in.
- v3: "snow brought back over the trunk's foot" as a masked body pass
  gave a crinkled rectangle, because the mask's soft x-falloff became
  hard with `clip(true)` and the load was heavy. I replaced it with hand
  strokes along the contact curve.
- v4: the lip strokes all followed the same curve with the same load and
  made a beaded white lace line (each stroke's end ridge lit by the
  relief). Now they are broken (30% skipped), flatter (more medium, less
  load) and vary in pressure.
- The limb mask began inside the trunk, so the limb was painted across
  the trunk as a lighter band. Now it starts at the trunk's edge, with a
  fillet in the crotch.

## FRICTION
(See the end of this file for the ranked top five.)

1. **Geometry helpers invalidate every checkpoint.** A painting's shared
   geometry (`axis`, `half_w`, `contact`) lives in top-level functions
   above `main`. Every stage uses them, so any change to the trunk's
   shape stales `drawing` and `sky`, even though the sky doesn't depend
   on it (only the pencil drawing does). The `// ckpt: from <stage>` tag
   works on lines, not on functions used by several stages. Workaround:
   `--resume sky --stale-ok --ckpt`, knowing the pencil drawing under
   the resumed canvas is slightly out of date.
2. **Checkpoint stem ignores `--out`**: a second crop window (`--crop
   ... --out out/limb.png`) writes to the same
   `out/r11_study3_full_crop.<stage>.ckpt` files as the first crop, so
   working on two passages (the foot and the crotch) means repainting
   from scratch each time you switch. Workaround: none; I re-ran with
   `--ckpt` after each switch.
3. **Soft masks + `clip(true)` give hard edges.** A mask whose soft
   falloff is meant as "fewer strokes here" (feathering a passage) is
   treated as a boundary: a clipped handling paints up to it fully
   and stops sharply, so a feathered patch turns into a rectangle.
   There's no "feather the passage by coverage" knob on `Handling`
   (Stipple has `fade`). Workaround: hand strokes along the curve.
4. **No brushed-glaze shadow over impasto that stays a glaze.** A
   `st.glaze()` over the dry, lightly impasted snow pooled into the
   hollows of every snow stroke, which made a shadow of dark crescents.
   That's physically right for a glaze (thin fluid paint pools), but
   Friedrich's thin, mostly single layer wouldn't have had the impasto
   under it. Workaround: mix the shadow into the snow lay-in.
5. **Hand strokes have no aiming.** `Held` + `pal.paint(col, medium)`
   mixes the masstone, and a thin lip stroke of `#cbc6be` over snow of
   about `#bdb9b6` dried far brighter than its neighbors. `Canvas::aim`
   exists, but it's per mark: every lip, crest and crust stroke needs its
   own aim call and radius, so hand-laid passages get chosen by trial.
6. **`pal.paint` is a mix search per call**, and I call it inside loops
   with fixed colors. Hoisting fixes it, but nothing warns you.
7. **Wet-into-wet surprise.** `c.wait(90.0)` after a body-color snow
   left it wet enough that the trunk strokes plowed white into the bark.
   That's correct physics, but there's no cheap way to ask "is this
   passage still open?" before painting over it. `drying_at(x, y)`
   exists per point; a per-mask summary would help.
8. **Relief lights every stroke end as a bead.** Short strokes of stiff
   white over a mid-gray (the lip) each leave a lit ridge at the release,
   and a row of them reads as a chain of beads. Workaround: more medium,
   lighter loads, varied lengths.
9. `Rng` has `f`, `range`, `chance`, `normal`, but no integer pick
   (`below(n)`). Trivial, but I reached for it.

## Critique
(in progress; see the final section)
