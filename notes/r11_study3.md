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
  fillet in the crotch. The first fillet sat under the limb as a bulb; it
  belongs in the acute angle above it.
- The broken stub ended in a rounded sausage cap: a capsule mask always
  rounds its end. I cut the last segment square in `limb_side` and drew
  four splinters off the break, one longer than the rest, plus a touch of
  pale wood.
- The far copses came out milky below their dark tops: stippled into the
  still-wet snow, the touches picked up lead white. I dry the snow first.
- Cloud streaks: first stippled on thin masks, they came out as ruled
  lines, then as dotted smears (the stippler grain shows in a thin band).
  Now a small soft filbert draws them in broken, swelling strokes of thin
  paint, with no fill.
- The limb was a straight rod and the upper trunk a bare pole. The limb is
  crooked now (eight points, zig-zag, as sympodial oak limbs grow), with a
  dead branch high on the left that ends in a break.
- Final pass at 3200px: the crust on the dead branch and the stub landed
  on the **underside**. My "up" normal flipped for limbs drawn right to
  left. Fixed by choosing the normal with negative y.
- The foot still had bright white "claws". Removing relief lighting
  (`NO_RELIEF=1`, a debug switch left in the program) didn't change
  them, and neither did aiming the lip paint. Tinting each stroke family
  in a debug run (crease red, lip green, drift blue) showed the whole
  story at once. The "drift" arcs were claws out in the open snow, 30
  units left of the trunk. The crease sat 5 units below the real edge,
  and the lip strokes sat on the bark. Before the `foot` stage, the
  snow field's own edge (the trunk mask cut at `contact`) already made a
  clean mound against the trunk. So the foot is now just a thin crease of
  shade on that edge and a few lip touches, each aimed with
  `Canvas::aim` a shade lighter than the snow it lands on. Lesson: debug
  colors per stroke family should be a one-line switch.
- The 3200px crops caught most of the above (bark "rain streaks", the
  beaded lip, the limb band across the trunk, milky copses). At 1000px
  they read only as vague wrongness.

## FRICTION

### Top five
1. **No false-color or "which code made this mark" view.** It took three
   blind experiments at 3200px to find that the white claws at the foot
   were my own drift strokes in the wrong place (item 14).
2. **Checkpoint staleness follows lines, not what a stage uses.** Shared
   geometry functions above `main` stale every stage, including the sky,
   so `--stale-ok` became routine. And crop checkpoints share one stem,
   so switching between two crop windows repaints from scratch
   (items 1, 2 and 13).
3. **Wet-into-wet pickup is silent and easy to trigger.** Twice a later
   pass went milky over an open passage (trunk into snow, copses into
   snow). It's physically right, but there's no warning and no cheap
   "is this mask dry?" query (items 7 and 12).
4. **Masks are hard boundaries for handlings.** Feathered passages turn
   into rectangles with `clip(true)`, and there's no tapered-path mask
   for limbs (capsules round every end), so a painter's own limbs need a
   hand-written coverage function, a square cut and a crotch fillet
   (items 3 and 10).
5. **Hand strokes and thin passages don't aim by default.**
   `pal.paint(hex, medium)` is a masstone, so thin hand strokes (the
   lip, crust and crests) dry off their intended look. Thin stipple bands
   show their grain as dotted lines, and a glaze over impasto pools into
   crescents. Each needed a different workaround: `Canvas::aim` per mark,
   a brush instead of the stippler, or mixing the shadow into the lay-in
   (items 4, 5, 11 and 15).

### All of it

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
10. **A capsule is the only limb shape at hand.** `Mask` has no "tapered
    polyline" or "stroke of a path with widths", so I wrote my own
    `limb_side` (coverage, direction, and which side is up). It needed a
    square cut for broken ends and a hand-placed ellipse for the crotch
    fillet. `growth::Skeleton` has `mask`, but a skeleton you draw by
    hand (a painter's own limb) can't use it.
11. **Thin stipple bands show their grain.** A stipple in a band a few
    units wide reads as a dotted line, not a streak. `fade` helps at the
    edges, but the dots along the band's axis stay visible. A brush was
    the right tool.
12. **Wet-into-wet pickup is silent.** Twice (trunk into snow, copses into
    snow) a later pass went milky because the passage under it was still
    open. Nothing in the run output says "stroke picked up X% foreign
    paint". The 1000px preview just looks gray, and it takes a 3200px
    crop to see why.
14. **No way to see which marks came from which code.** The white claws
    at the foot took three blind experiments (thinner paint, aimed
    paint, no relief) before I tinted each stroke family by hand. A
    debug mode that colors marks by call site or stage (a "false-color"
    view) would have found it in one render.
15. **`pal.paint(target, medium)` means "masstone"** (`Palette::mix`:
    "the pile that, laid thick … looks `target`"). For thin hand
    strokes, that's easy to misread as "looks like this where I put
    it". `Canvas::aim` is the right call; the notes say so, but hand-mark
    examples (study_tip) all use `Paint::body(hex)` or `pal.paint`.
13. **Crop checkpoints are per stem, and 3200px crops cost 40–95 s from
    scratch** on the shared machine (mask building over the whole canvas
    is most of it). Switching between two crop windows means a full
    re-run each time (friction 2).

## Critique (honest)
What works:
- The value structure is Friedrich's: a dark, nearly silhouetted tree
  against a sky that is the lightest thing in the picture, with snow a
  step below the glow. The sky passes from cool gray-blue through a pale
  green band to the warm glow without a seam. It is thin paint, stippled,
  and the ground's grain shows through it.
- Trunk against sky: the silhouette is ragged at the bark's scale, not
  smooth. The limb springs from a fillet, lit on top and dark
  underneath, with a broken crust of snow on it. The twigs lift off to
  hairlines. The dead branch and the stub's torn end give the tree a
  history.
- Trunk into snow: the trunk stops at an irregular contact line. A drift
  piles on the windward side, with a crease of shade at the bark and a
  scoop of bluer shadow in the lee. The cast shadow comes toward us and
  widens, mixed into the snow rather than laid on it, so the drifts show
  through. The grass stalks, laid last, sit in the snow.

What reads as digital or weak:
- The trunk is still too even in tone along its length. At 1000px it is
  one dark shape. The bark (fissures, cross cracks, lichen) only shows in
  3200px crops, and the lichen barely at all. A backlit oak is dark, but
  Friedrich's trunks have more going on in their lower third.
- The root flare is a smooth bell. There are no roots going into the snow
  and no humps of snow over buried roots.
- The contact at the foot is clean now, but it's plain: one mound and
  one crease. There are no crumbs of snow on the bark, no roots, and no
  hollow where the snow has melted back from the dark wood.
- The foreground snow modeling is a noise field in perspective. It reads
  as painted drifts at 1000px, but its strokes are the same everywhere.
  The crest strokes are generic, not drawn to particular drifts.
- The twigs are recursive and too regular: forks at the same fractions
  and similar angles. The claw-like, hooked twig ends of an old oak
  aren't there.
- The far copses are flat dark strips with ragged tops. They pass at
  1000px but have no inner structure (no trunks, no gaps of sky).
- The cloud streaks are good in the 3200px crop but faint at 1000px.

As a study of the two meetings, it succeeds at the sky edge more than at
the snow edge: the sky edge is decided, and the foot is still an
arrangement of marks that almost convinces.
