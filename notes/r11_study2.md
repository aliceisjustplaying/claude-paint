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

- (clock check: the times above were my guesses; the real clock said 11:57
  at v8. Real times from here.)
- 11:57 first 3200px render (245 s). Crops: limbs were flat polygonal bands
  pasted on the trunk (a corner at every vertex); snow on the bough read as
  glossy slivers inside the limb; the lip a row of C-shaped curls.
- 12:00 limbs through Catmull–Rom, modeled by where a point lies across the
  limb (upper side catches the glow), lean ridges and cracks along them,
  bark strokes dragged from the trunk out along the limb at the fork. Snow
  on the bough only where it runs level, sitting on the upper edge.
- 12:02 lip as long flat overlapping strokes along the bank line; the dark
  hollow only on the shadow side; little snow pushed up on the windward side
  as pressed touches; grass with bent-over and stubble blades; bark ridges
  toned down. 3200px render of that state (279 s) kept in scratch as
  `r11_study2_full_prev.png`.
- 12:06 last round on the weak points of the critique: the trunk's
  silhouette made uneven (slow swellings per side, a callus round the scar,
  a burl low on the shadow side); a root running out from the foot on the
  shadow side and diving under the snow (first try tapered to a point on
  top of the snow like a horn; now it keeps its girth and the bank line
  cuts it); eight curled oak leaves blown onto the snow, each with a hair of
  blue shadow. Final renders from scratch: 1000px 12:10 (44 s), then 3200px.

## FRICTION
Ranked by how much it cost me.

1. **Nothing is in front of anything; the painter has to cut every overlap
   by hand.** The trunk mask ran down to y 1160 so the snow could cover its
   foot. The mound over it fades into the field (a soft mask), and where the
   mask was weak the handling laid no strokes, so the trunk's paint showed
   through as dark specks in the snow at 3200px (`c1.jpg`). Nothing warned
   me. Workaround: the trunk mask ends exactly at the bank line
   (`* (1.0 - smoothstep(bank(x) + 0.5, bank(x) + 2.0, y))`), and the lip
   strokes cover the seam. The same problem, inverted, cut the cast shadow
   off at the base: the mound was painted over it. Workaround: the mound
   moved into the snow stage, before the shadow. A painter keeps "the snow
   is in front of the trunk" in their head; here it lives in stage order
   plus hand-cut masks, and a soft mask edge means gaps, not thin paint.
2. **The blender's strength is a guess.** `st.blend()` at its default
   (coverage 3, pressure 0.4–0.5) over the wet cast shadow on wet snow
   wiped the shadow almost out (v4). There's no way to ask what share of
   the contrast a fusing pass will keep. I tuned by eye:
   `.coverage(0.9).pressure(0.3, 0.4)`.
3. **No limb primitive for a painter's own tree.** Motifs belong to the
   painter, fine, but the geometry under a limb (a curved, tapering tube, its
   distance field, its direction, which side is up) is not a motif, and I
   wrote it from scratch: distance to a polyline with interpolated radius,
   Catmull–Rom smoothing, the upper normal (`Limb::sd`, `Limb::smooth`).
   It's per-pixel over about 40 segments for every mask, `angle` and `color`
   call, so the 3200px crop over the limbs took 91 s against 41 s for the
   base. `growth::Skeleton` has limbs, but only as the output of growing a
   tree, not as a shape I can draw myself.
4. **Small gestures take a shape of their own at full size.** At 1000px
   they looked fine; at 3200px:
   - lean filbert drags for bark plates came out as dotted blotches
     ("camouflage");
   - a round sable arched over 4 points to heap snow came out as a C-curl
     with a dark rim;
   - a tiny inverted-V drag came out as a hollow ring;
   - snow caps sized to read at 1000px were white pills.

   Workarounds: a nearly dry round sable instead of a filbert, flat
   overlapping strokes instead of arches, `Touch` instead of tiny drags. A
   painter sees the mark as it forms; here I only found out in a 40 s crop.
   The 1000px preview doesn't predict the 3200px mark for blunt tools
   (notes/tip.md says so; it bit me anyway).
5. **Checkpoints go stale from geometry at the top of the program.** The
   limb, twig and bank geometry is shared by the drawing, masks and late
   stages, so it sits above the first stage. Every tweak staled "drawing",
   "sky" and "far", which don't use it. `// ckpt: from twigs` fixed the twig
   list, but not the limbs (the drawing traces them). I used `--stale-ok`
   about eight times and had to reason myself about whether it was safe.
   Once it wasn't quite: a crop resumed from "trunk" showed the old shadow.
6. **Crops pay for the whole canvas.** Sky stipple and whole-canvas masks
   (including my limb distance fields) make a 3200px crop take 40–90 s from
   scratch. After one `--ckpt` crop, resumed crops took about 2 s, which
   is the loop that works.
7. **Pointed tools knot at joins.** Twigs springing from a limb begin at
   `pressure_for(w)` with a pointed sable, and the press leaves a small
   blob at every join (visible in the 3200px twig crop). I left it: it
   reads a little like a node.

## Critique
What works: the sky reads as Friedrich's kind of evening, cool above, pearl
through straw to a faint rose over the snow, with the stipple grain visible
only close up. The twigs are oak-like, angular and crowded at the ends, and
they carry the "crossing the sky" part best. The shadow springs from the
foot of the trunk and runs toward us, and at 3200px it is visibly brushed
and fused. The limbs grow out of the trunk and are lit on top.

What doesn't:
- The trunk is still too even in tone: one mid-dark brown all the way up,
  fissures like combed lines of similar weight. The last round gave its
  silhouette swellings, a callus and a burl, which helped, but the bark is
  strokes, not plates. Next: bark plates as shapes with lit upper edges.
- The junction with the snow, the actual subject, is better but still too
  clean: an almost straight line on the left. The root on the right now
  dives into the snow, but it is a smooth horn-like tube, not a gnarled
  flange. The leaves are right in idea, but at 1000px they read as specks.
  There's no melt ring.
- The snow field is empty and even: fine strokes, soft lavender troughs,
  but the swells don't read as forms. It also sits a little too white next
  to a glowing sky for a sun on the horizon.
- The limbs are rubbery, not angular like oak (Catmull–Rom over wobbled
  points gives S-curves; an oak limb kinks).
- The grass is tidy, in tufts of similar size. The far woods are a thin
  dotted band, barely there, which may be right.
- Digital tells left at 3200px: uniform stroke widths in the shadow, small
  white flecks of crevice snow like raindrops, the scar a clean ring.
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
