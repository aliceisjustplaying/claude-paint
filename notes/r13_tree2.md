# r13_tree2: one oak in winter

Program: `paintings/src/bin/r13_tree2.rs`. Render: `out/r13_tree2_full.png`
(2400 px wide, 1000 × 1250 units, portrait 4:5).

## The study

An old pedunculate oak standing alone on a low rise of snow, under a pale
overcast winter sky. The horizon is low and crosses the lower bole, as if
drawn sitting on the ground (Friedrich marked the horizon even on the lower
trunk and kept that viewpoint in the painting: trees.md §5.2, BUSCH-V
pp.77–78). The tree is characterful in the ways the sources say his oaks were
(trees.md §5.3–5.5):

- **Stag-headed.** The centre-left leader is alive low down and dead above:
  bare, silver-gray wood with a few broken stubs. A centre-right riser is dead
  and broken off short. (ATF/HTC: retrenchment; CDF-EICH: dead branches as
  signs of great age.)
- **A sawn-looking stump** of a lost limb on the left of the bole, cut clean,
  with a pale face and growth rings (ROL: his "mutilated" oaks with smooth cuts).
- **An old wound**: a dark hollow with rolled callus lips low on the bole.
- **Short-shoot crown**: the live crown is a crooked net of short shoots. Some
  forks abort (the old tree sheds twigs, cladoptosis), buds cluster at tips,
  and a few epicormic sprouts grow on the bole.
- **Snow near 0 °C.** Wet snow lies as a ridge on the upper side of
  near-level limbs thick enough to hold it and is missing where it slid off.
  None lies on steep wood. It is lodged in lumps in the main fork. (MIL64,
  MIL66: interception rises toward freezing, and snow slides off slanting
  supports.)
- **Marcescent leaves**: a few brown dead leaves still on low young twigs.
- Dry grass through the snow as fine upturning strokes laid last, over the
  finished snow (NG p.56). There is a fallen dead branch half sunk in the
  snow and a thin strip of distant wood on the horizon.

## Working method

Order, from the sources: pencil underdrawing, then sky, snowfield, the tree
over the finished sky (ALF p.346), thick to thin, snow on the limbs, and
small things last.

1. **drawing**: HB outline of the trunk and the scaffold limbs' sides, the
   horizon ruled in 2B, and the line of the rise.
2. **sky**: broad level lay-in (`st.broad()`), fused with the badger,
   dried, then stippled over with the same gradation
   (friedrich_materials.md §6: stippled skies). The color is a gray
   overcast, faintly violet overhead, warming to a pale ochre band at the
   horizon, with soft darker stratus drifts in the upper sky.
3. **snow**: body strokes, nearly level, in lead-white mixes that reflect
   the gray sky. The plain beyond the rise is grayer, the rise's flank away
   from the light bluer, and there is a light crest line. The distant wood
   is stippled. The snow is stippled again, lightly.
4. **trunk**: all the wood thicker than 2.4 units is one mask (trunk
   outline polygon with a concave root flare, plus a ribbon per limb). A
   dark umber underpaint is dried. Then each limb is modeled with strokes
   *along* it, laid side by side across its width in lanes, with color
   from a diffuse light at upper left (lit gray, mid umber, shadow dark;
   the dead wood silver). Bark fissures are dark pointed strokes along the
   big wood, broken, with a lit ridge beside some and an occasional
   cross-break. Then the stump face, the hollow and lichen touches.
5. **twigs**: every grown branch past the body width is one or more
   pointed strokes (`point: 1.0`), pressure from the branch width along
   its length (Gesture swell knots from `Tool::pressure_for`). Tips lift
   off. Thicker ones go down first, then the fine net.
6. **limb snow**: runs along the upper side of each branch where it is
   near level, in two strokes: a cooler body straddling the top edge,
   then the lit crest above it.
7. **foot**: the drift against the trunk in rows of strokes following
   its surface, lifting off into the snow already there. Then the weeds,
   the leaves and the fallen branch.

**The tree** is a "study from nature" for the trunk and seven scaffold
pieces (placed by hand: the great left bough, the stag-headed leader, the
level right elbow that holds most snow, the dead riser, a lower right
limb). Everything finer is grown by `grow()`:
- Internode length grows with width. At each fork the child takes a share
  of the parent's cross-section and the parent continues at
  `(w^Δ − wc^Δ)^(1/Δ)`, with Δ = 2 (Leonardo; trees.md §2).
- Sympodial kinks: the parent turns away from the child at every fork, so
  the line goes in elbows (trees.md §1). There are occasional random
  elbows and little wander.
- Thin shoots turn up and heavy level limbs sag. Near the canvas edges,
  shoots turn upward (keeps the whole tree in the picture).
- Side shoots along the scaffolds alternate and are acrotonic (stronger
  toward the outer end). Dead limbs keep only a few short stubs.

The fast loop: `--resume snow --stale-ok` takes 10–15 s at 2400 px (the
tree stages cost about 10 s in all). A whole render from scratch takes
about 125 s: drawing 43 s, sky 61 s. Crops of about 140 × 120 units take
9–25 s.

## Friction

1. **The ground's knife texture printed through lighter paint as a
   crackle net** (looked like craquelure, which the brief forbids). I
   modeled the lit side of the trunk with a soft round at light pressure
   over a wet dark underpaint. The hairs touched only the tops of the
   textured ground and left the dark in the hollows, which gave a network
   of dark lines. It survived relief off (a color effect, not lighting).
   Diagnosing it took four crop renders with env-var switches
   (`NORELIEF`, `SKIPF`, `SKIPL`) that I had to add to my own program.
   There is no way to ask the engine "show me what this pass alone laid",
   or to view the canvas without the ground. Workaround: dry the
   underpaint, less medium, full load, filberts for the wide lanes, and a
   pressure floor of 0.45. Physically plausible, but nothing warned me,
   and a painter would have seen it at once on the easel.
2. **Staleness is keyed to source position, not to what a stage uses.**
   The tree geometry (`build_tree`) and colors sit above `main`, so every
   tree edit makes every checkpoint stale, including "sky" (which doesn't
   use the tree). The real fix would be moving the tree code, but the
   pencil stage *does* use the tree. Workaround: `--resume snow
   --stale-ok` all through, with a clean whole render at the end. Tagging
   with `// ckpt: from` doesn't fit a helper function above `main` used by
   several stages.
3. **The pencil underdrawing is slow at 2400 px**: about 43 s for roughly
   12 lines (the trunk outline, 14 limb sides, the horizon and the rise).
   It is the single most expensive stage after the sky, for something a
   painter does in minutes. It's affordable only because of checkpoints.
4. **Checkpoints at 2400 × 3000 are 720–890 MB each**; with `--ckpt` on
   the whole run that's 6+ GB per render on a disk at 86%. I deleted old
   ones by hand before each fresh run.
5. **No tapered or pressure-profiled stroke along a path in one call.** A
   branch is a polyline with a width at each point. To draw it I had to
   convert widths to pressures myself (`Tool::pressure_for`) and feed them
   as `swell` knots, which are spaced evenly in *parameter*, not at my
   points. The widths only match if my points are evenly spaced. A
   `Gesture::widths(Vec<f32>)` per point would say what a painter means
   ("this thick here").
6. **Ribbon masks put a round cap at every point**, including the first.
   A 124-unit-wide trunk start became a disc hanging below the ground,
   twice (once in the first pass, once when I reordered the bodies and
   `skip(1)` silently skipped the wrong one). I built the trunk from an
   outline polygon instead. A ribbon with flat or no end caps would help.
7. **`Mask::from_shape` has no soft edge**, so a snow drift painted
   through `work` inside a shape mask came out as a hard white slab with
   rectangular ends. I repainted it as rows of drags that follow the
   surface and lift off at their ends, which reads much better, and
   honestly is how a painter would do it.
8. **Limb modeling by hand-rolled "lanes".** There is no handling that
   paints *along* a curved body (an angle field from a polyline). `work`
   takes one angle function of (x, y). For hundreds of curved limbs I
   would have needed a per-pixel direction field, so I hand-wrote lanes of
   drags clipped to the wood mask. It works, but the lane structure
   shows as parallel bands when the lane count is low.
9. **`Paint` from `pal.paint(white, 0.05)` still came out translucent**
   in small round-sable dabs over dark wood (the first fork heaps looked
   like glass beads). Load and pressure matter more than the mix. I
   couldn't see what a single dab would lay without rendering.
10. **The stroke-order/visibility of a body inside another body.** A
    stump whose base sits inside the trunk is painted over by the trunk's
    lanes if it comes first, and paints over the trunk if it comes after.
    There is no "behind" for gestures, only order and masks.

11. **Thick wood joins are hard with strokes that run along each branch.**
    A side branch starts on its parent's axis, so its modeling strokes
    streak across the parent. Starting them where the branch leaves the
    parent's side fixed the crown. For the scaffold limbs at the main fork
    it exposed the trunk outline's flat top and bare underpaint, so they
    still start inside. There is no notion of one body growing out of
    another (a union with a blended seam). The fork is still a bit of a
    jumble of crossing strokes.
12. **Keeping the whole tree in the frame** needed a hack in the growth
    rule (shoots turn upward near the side edges) plus a hard stop. The
    engine has no help for "compose this inside the canvas"; that is fair
    (it's the painter's job), but the upturned twigs at the left and right
    ends read a little like brooms.

Things that worked well: resuming is exact and fast (1.4 s to repaint
only the last stage at 2400 px); crops at 2400 px take 9–25 s; drawing
about 19,000 pointed twig strokes takes only about 2.5 s; `pressure_for`
made width-driven strokes possible at all.

## Critique (honest)

What reads as painting: the twig net. The fine pointed strokes lifting
off at the tips give a real filigree against the sky, and at 2400 px the
finest twigs are hairlines that thicken toward the limbs in steps. The
stag-head reads: a pale dead leader and a broken dead riser stand above a
living crown that stops short of them. The trunk's bark, with dark
broken fissures, lit ridges and cross-breaks, reads as brushwork in the
spirit of his hatched pencil studies. The snow along the tops of the level
limbs is the most "winter" thing in it. The sky is a quiet, believable
overcast with a warm band low down. The low horizon crossing the bole
puts us on the ground, looking up.

What reads as digital or weak:
- **Symmetry and roundness of the crown.** It is a well-behaved dome.
  Friedrich's oaks are wilder: limbs that go out, turn back, break off.
  Mine has the right parts (dead top, stump, hollow) but the silhouette is
  too even, and the brooms at the side edges come from my edge rule, not
  from a tree.
- **The scaffold limbs are smooth Catmull-Rom curves**, too clean for
  "gnarled, bent branches"; they needed elbows and thickened knuckles
  where limbs were lost.
- **The limbs' modeling** is lanes of parallel strokes. At full size they
  read as painted, but uniformly so: the same stroke length and spacing
  everywhere, with no passages where the hand slowed for a knot or a
  scar.
- **The snowfield is nearly empty.** The drifts are faint horizontal
  lines; the drift at the foot is better but still a little like a
  cushion. The weeds help, but the foreground lacks the small particular
  things Friedrich would put there (a track, a stone, a stake).
- **The dead wood** is a flat silver-gray with grain lines. It lacks the
  cracks and the broken, splintered ends a dead oak limb has (the top ends
  are round-capped strokes).
- **The main fork** is a crossing of strokes and not the swelling collars
  and bark ridges the sources describe (trees.md §2).
- I left out craquelure and varnish as the brief asked. The ground shows
  through only faintly. I lost the "paint pooling in the texture" look on
  the trunk when I fixed the crackle net, and the sky and snow have it
  only slightly.

If I had another hour: gnarl the scaffolds (elbows, collars, a torn limb
with splinters), break the crown's silhouette with one long limb reaching
out and one gap where a limb fell, and give the foreground one particular
object.
