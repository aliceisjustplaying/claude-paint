# r11_study2: one trunk against sky and snow

Program: `paintings/src/bin/r11_study2.rs`. Renders: `out/r11_study2.png`
(1000px), `out/r11_study2_full.png` (3200px). Scratch: `~/tmp/r11-study2-74938f1e/`.

## Plan and why
- Portrait 24 × 30 cm (`width_mm: 240`, aspect 0.8): a small study.
- Ground: Friedrich's warm knifed lower layers kept, but the top ground made
  a lighter, patchy cream (#c9b597) because the picture is pale sky and snow
  and he "painted mostly on a patchy whitish ground" (KÖR p.284).
- Order as documented: graphite drawing (2H searching, HB firmer, horizon
  ruled) → thin sky lay-in, badger, stipple wet, dry, finer stipple toward
  the glow → far woods hatched → lead-white snow, slight foreground impasto
  (NG p.50), cast shadow wet into wet → trunk over the dry sky (ALF p.346):
  umber underpainting, body, fissures, lit rim → twigs → the base (snow
  banked against the trunk, hollow, snow caps) → dead grass flicked up last
  over the finished snow (NG p.56).
- Light: sun just set behind and left of the tree, so the trunk is nearly a
  silhouette with a warm rim on the left and its shadow comes toward us.

## Log
- 11:52 v1 (1000px, 34 s). Sky gradient good. Bad: snow meets trunk in a
  ruled horizontal line; cast shadow a hard slab; bark "plates" (lean dabs)
  read as camouflage spots; twigs thin and spidery; stub a stick poked in.

## FRICTION
(log continues)
- 12:03 v2: bank made irregular (drift tongue on the windward left, root
  hump on the right, lobes), a separate mound pass, snow on the stub/bough
  as broken lines instead of caps, denser oak twigs (depth 3, angular
  zig-zag), bark "plates" replaced by long lean ridges. The mound came out
  a pasted blob with a hard lower edge and its own color.
- 12:08 v3: mound fades into the field (its color is the field's color
  where it thins). 3200 crop of the base (`c1.jpg`): dark specks in the
  mound (trunk paint showing through: the trunk mask ran to y 1160 under a
  thin mound), a scalloped cotton-ball lip, the shadow as lozenge dabs, the
  bark ridges as dotted blotches.
- 12:20 v4–v5: trunk mask now ends at the snow line; lip irregular; shadow
  in longer strokes fused with the badger (the badger at its default
  coverage erased it: down to coverage 0.9, pressure 0.3–0.4); bark ridges
  with a nearly dry round sable instead of a filbert; snow darker and cooler
  (it was paper white against a glowing sky, which is wrong for a sun on
  the horizon), with warm crests and lavender troughs.
- 12:28 v6: the mound moved into the snow stage *before* the cast shadow,
  so one shadow crosses mound and field and springs from the foot of the
  trunk (before, the mound was painted over the shadow and cut it off).
- 12:36 v7–v8: lip heaps (snow piled a little against the bark), toned down
  from white "popcorn"; the broken stub (it read as a stick glued on, with a
  white pill of snow) replaced by a healed branch scar: dark hollow, callus
  lip lit on its upper left.

## Working method
Stages with `--ckpt` at 1000px; iterate late stages with `--resume` (≈1–8 s
per look against 35 s whole); `scripts/peek` crops of the 1000px render for
the base; one 3200px `--crop` of the base (40 s) to judge marks at size.
Geometry (trunk, bank, shadow) as closures shared by masks, color fields
and hand gestures, so the drawing, the paint and the details agree.
