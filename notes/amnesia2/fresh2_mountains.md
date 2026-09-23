# fresh2_mountains: Daybreak in the Riesengebirge (a wanderer by a granite tor)

Program: `paintings/src/bin/fresh2_mountains.rs`. Renders: `out/fresh2_mountains.png`
(1000px) and `out/fresh2_mountains_full.png` (3200px). Scratch and review
JPEGs: `~/tmp/fresh2-mountains-02270ee4/`.

## Composition and why

The time is dawn, before sunrise, on the Silesian (north) side of the
Riesengebirge. We stand on a dark granite knoll. The sky is cool gray-blue
at the top, warms through a faint rose to a pale lemon glow behind the far
range, and is crossed by two thin bands of stratus whose undersides catch
the light. The Schneekoppe rises as a blunt cone right of center, in the
glow. Two receding ranges below it get grayer and cooler toward us, with
mist lying at the foot of each. The nearest range is wooded and dark, with a
serrated edge of spruce tips where the wood reaches the crest. Its foot is
lost in a sea of valley mist that fills the right half of the lower picture.
The knoll takes the lower left: a dark hump of turf, heather and stones. On
its crown sits a tor of weathered granite "woolsack" blocks. Right of the
tor stands a lone wanderer, seen from behind, bareheaded, with a stick,
looking east toward the light. At the knoll's right shoulder stand a few
wind-thinned spruces (one dead). Smaller spruces go down the flank toward
the fog. A low, dark granite outcrop breaks through the turf at the bottom
left, cut by the frame. Four birds cross the glow toward the sun. A thin
waning crescent, the old moon rising ahead of the sun, hangs over the
wanderer with its lit limb turned down toward the hidden light. The St.
Laurentius chapel is a nub on the Schneekoppe's summit.

What it draws on in Friedrich (from knowledge, no pictures):
- The Riesengebirge is his real motif. He walked it in 1810 and painted
  wide, banded panoramas of its ranges from memory and drawings for years
  afterward, often at morning and often with mist in the valleys.
- The *Rückenfigur*: one small figure seen from behind at the edge of a
  height, standing in for the viewer. He is the one sharp silhouette in the
  picture.
- Contre-jour dawn: the light source is behind the motif and hidden, so
  near things are dark masses and far things dissolve into light.
- Strict layering: a dark foreground stage, then an empty middle (the mist)
  to jump across, then the far ranges. There is no path from here to there.
- A granite tor (the Riesengebirge's woolsack weathering), plus dwarf pine
  (Knieholz) and spruces as the particular local flora: exact, unheroic
  details.
- The dead spruce among the living ones, and birds flying toward the light,
  are his kind of quiet symbolism.
- Technique, from notes/research/friedrich_materials.md:
  - a warm ready-primed ground (the Style's Dresden ground);
  - a pencil underdrawing that shows through the thin paint [CATS p.132];
  - "a very thin underpainting" [CATS p.127];
  - one or two thin layers, with sky, mist and far hills stippled
    [NG p.56] and firs hatched in short strokes [NG pp.49–50];
  - grass flicked upward last, over the finished ground [NG p.56];
  - small details (figure, birds) at the very end [CATS p.130].
- Format 1000×690 units (1.45:1), about 44 cm wide per the Style: a modest
  cabinet picture, as most of his mountain views were.

## Working method, stage by stage

1. **drawing**: pencil lines (a thin round with gray, lean, low-hiding
   paint) along each crest, the knoll's edge, the tor and the figure. The
   pencil lifts every 40–110 units. The lines stay faintly visible under the
   thin sky and mist.
2. **underpainting**: a thin brushed wash. Umber goes under the knoll
   (starting 3 units inside its edge), blue-gray under the near range and
   valley, and a cool violet-gray under the far and mid ranges. This came
   out of a problem (FRICTION 3): without it, the pale pink top ground
   flashed through as white specks in the dark and orange specks in the
   ranges.
3. **sky**: a lay-in in long level elbow arcs, a shade duller than the
   target, fused top to bottom with the badger, then stippled into the wet
   lay-in (stippler 3 units, coverage 2) to break the strokes.
4. **sky light**: once dry, a second, finer and paler stipple, denser toward
   the glow. Then the clouds are stippled with a slight horizontal drag:
   violet-gray bodies with rose-gold lower edges near the sun.
5. **far range**: one `Form` holds three `Ridge`s, lit from behind and
   above right (contre-jour, `front` −0.35). Each is painted with my own
   passes: body strokes down the fall lines, colored by shade and aerial
   perspective, then a stipple aimed at the same colors (coverage 2.6) so
   the strokes dissolve. The ranges and mists are mixed from a family set
   out on its own: lead white, pale smalt, cobalt, yellow ochre, raw umber,
   bone black. No vermilion or red earth, which had flecked them orange.
6. **far mist**: a stipple veil (`aim(false)`, density = tone) at the foot of
   the far range, thinning upward in noise banks.
7. **mid range**: the same method, a clear step darker and cooler than the
   far range. Spurs (`bend` > 0) catch sky light and gullies (`bend` < 0)
   stay dark. Then a short mist at its foot, stippled and fused with a
   clipped badger pass.
8. **near range** (masked off the knoll): body strokes, then spruce woods
   hatched upright near the crest only. Then tiny spruces along the crest
   where the wood reaches it, in groups with gaps: each is a hair-thin
   rigger stem lifted to a point plus 3–6 level boughs, longer toward the
   foot. Then a brushed, fused veil of mist rising from the valley at its
   foot.
9. **valley mist**: the sea of fog. Long level strokes of thin paint with
   `load_at` giving the density, fused with the badger, then stippled. Its
   color has lit billow tops and cool hollows, and it is grayer lower down,
   toward the knoll's shadow.
10. **knoll**: body color in strokes following the swell of the ground,
    darker toward us.
11. **heather**: rust-brown patches hatched upright, a thin band of dead
    grass along the crown's edge, and about 20 stones (a dark underside
    stroke and a pale upper-edge stroke each).
12. **tor** and **outcrop**: `Form`s of rounded blocks sunk into the turf,
    with a fracture plane and grain. My own `paint_granite` passes:
    - a dark lay-in down the planes;
    - stiffer paint on the lit planes;
    - dark joints where `bend` < 0;
    - the turf color tucked back over the foot along its seat line.

    The near outcrop is pushed toward near-black (`sheen` 0): in
    contre-jour a near rock is darker than a far one.
13. **knieholz**: dark mats hatched in crossing strokes on the knoll's
    shoulders.
14. **spruces**: my own spruce. The stem is one stroke lifted toward the
    top. Tiers go out and down, each with a few hanging needle hatches.
    The left side is thinned by the west wind. The dead one is a gray stem
    with alternate bare claws.
15. **slope trees**: smaller spruces going down the flank. I tried mist
    lapping up over their feet, first stippled, then brushed; both turned
    the dark flank into mottled lichen, so the knoll stays crisp against the
    fog.
16. **figure**: the wanderer, stroke by stroke in a local frame:
    - legs, boots, the coat in five overlapping strokes, shoulders, arms;
    - the stick;
    - neck, hair and a glimpse of cheek;
    - a thin warm rim of dawn light down his right side.
17. **grass**: about 2,000 rigger flicks, curving upward, spread from the
    knoll's edge downward. Pale ones only right at the crest, against the
    mist. Then backlit tufts in clumps on the knoll's skyline, breaking its
    edge. Each flick is set down lightly at the root, pressed into the
    blade and lifted off (`swell` + a slow attack); pressed at once, they
    left a row of dark beads along the edge.
18. **moon**: the crescent is a lune mask (the disc less the same disc
    shifted away from the sun), filled with short sable strokes running
    round the limb and clipped, so the horns come to points. A dragged
    arc with swelling pressure gave a blunt banana.
19. **birds**: four small V-strokes over the glow, each wing a rigger
    flick lifted off. Also the chapel on the Schneekoppe: a dab and a
    lantern flick.
20. **finish**: varnish, craquelure (finer than the aged preset, over this
    canvas's real 240 µm ground: FRICTION 1 and 11) and relief light.

## FRICTION (running list)

1. **The aged craquelure is resolution-blind.** `Cracks::aged` draws
   70 µm hairlines. At 1000px, one pixel is 0.44 mm, so every crack becomes
   a full-contrast dark pixel line and the whole preview looks like crazed
   glass (first preview). At 3200px (0.14 mm/px) it is plausible. The
   preview is supposed to judge the painting, and here it misjudges the
   surface. *Workaround:* `Cracks { width_um: 40, depth_um: 20, dirt: 0.3,
   ..aged }` in my own `Finish`. The rasterizer should use sub-pixel
   coverage: a crack narrower than a pixel should darken it by its area,
   not paint it.
2. **No builder for a handling's brush size.** Presets fix the width
   (`body()` is a 9-unit filbert, `hatch()` 2.6), and there is no
   `.width()`. *Workaround:* a small `ToolWidth` extension trait in my
   program that edits the public `tool` field (and scales the hair length
   with it).
3. **Bare-ground flecks in dark body passages.** At 1000px the dark knoll
   was sprinkled with single white pixels and short pale "comets": the pale
   top ground showing where stiff strokes ploughed paint off the texture's
   peaks. Stopping with `--stop heather` showed they were there before the
   grass. The engine knows about this (notes/strokes.md, known issues).
   *Workaround:* a brushed thin underpainting (umber, blue-gray) before
   anything else, so exposed peaks show a dark color. That is historically
   right for Friedrich anyway, but I only did it because the flecks forced
   me to.
4. **Checkpoints go stale for edits that can't matter.** The stale check
   hashes the source prefix up to a stage's end. Moving a constant near the
   top (the tor's position, the near ridge's depth) invalidates every
   checkpoint, even for stages that never read it. *Workaround:*
   `--stale-ok`, judging by hand whether the edit could affect earlier
   stages. That's error-prone: I had to reason about whether a changed
   `Ridge` depth could change another ridge's silhouette in the shared
   `Form` (it could, in principle).
5. **`--stop` silently doesn't stop.** Two ways, and both cost me renders:
   - `--resume knoll --stop knoll` runs to the end: the stage is "in
     checkpoint", so its end never fires the stop.
   - `--resume` takes the file slug (`far_range`) but `--stop` compares to
     the stage's real name (`far range`). `--stop far_range` never matches
     and the run goes to the end without a word. While bisecting I took
     three of these "stopped" renders for real intermediate states before
     I noticed they were identical to the final.

   *Workaround:* quote the real name (`--stop "valley mist"`) and always
   stop on a stage after the one resumed. An unknown `--stop` name should
   be an error, like an unknown `--resume` is.
6. **Closures and ownership.** Every noise field (`Fbm`) and `per_column`
   profile is non-`Copy`. Each color or mask closure that uses it is `move`
   (it has to be, for `Sync` and lifetimes), so the second closure that
   wants the same field fails to compile. *Workaround:* rebinding
   everything as a reference (`let n = &n;`) right after it is made. This
   cost two compile rounds and is pure boilerplate: 30 lines of `let x =
   &x;`.
7. **`Sdf::block` size is whole extents, and nothing says so where you
   read it.** I took it for half-extents and the tor floated above the
   ground with mist showing underneath. You only find out by rendering.
8. **The engine's light model gives contre-jour faces too much value on
   up-facing planes.** With the light behind the motif, blocks' tops came
   out pale beige: loaves, not a dark tor against the dawn. I had to crush
   the lit color and use `direct^1.5`. That's fine as a painter's choice,
   but `Shade::value` mixes in ambient sky light and doesn't expose "rim"
   (grazing light on a silhouette edge) as its own term. Contre-jour
   painting is all rim.

9. **Hidden layers leak through every later passage, and the only way to
   find out is a 3200px render.** The pale chips in the dark knoll at
   3200px were fog: my valley-mist mask (a density field) was nonzero under
   the whole lower left, so the fog was painted under the knoll, and every
   gap in the knoll's body color showed it. At 1000px they were a few white
   pixels that I took for ground flecks (item 3). The painter's rule
   "don't lay light paint where dark will go" has to be enforced by hand in
   every mask (`off_knoll`). Finding the stage took `--resume X --stop Y`
   bisection on crop checkpoints, which worked well.
   It happened twice. The second time (3200px only, 2,600 pale pixels in a
   200×80-unit patch of turf) the culprit was the near range itself: a
   `Ridge` reaches `depth` below its crest across the whole width, so it was
   painted gray under the knoll. Bisecting stage by stage with
   `--resume A --stop B` and counting pixels over 70 found it, and masking the
   range with `off_knoll` took the count to 204. A `Form` has no notion of
   what stands in front of it unless that's in the same form.
10. **Stipple as a veil over dark reads as static at 3200px.** A mist
    stippled with `aim(false)` over a dark range is a field of pale dots and
    hollow rings on the dark ("frogspawn" at the fringe). Stippling breaks
    strokes over *light* passages beautifully, but over dark ones every dot
    stands alone. A badger pass over the wet stipple barely helped.
    *Workaround:* mist over dark passages is a brushed veil (broad,
    `by_masstone`, medium 0.6, density through `load_at`), badger-fused.
    Stipple stays in the sky and over already-light fog.
11. **The aged crack preset contradicts the style's ground.**
    `Cracks::aged` assumes a 60 µm ground, so the cracks follow the weave
    and form a rectilinear grid. `Style::friedrich` primes 240 µm (110 +
    70 + 60). The finish should take `ground_um` from the style. 
    *Workaround:* `ground_um: st.ground.iter().map(|g| g.um).sum()`. The
    network became an isotropic web.
12. **Small marks become blobs at 3200px.** A spruce tip or a bird made
    with a 0.7-unit round (2 px at 3200) comes out as a rounded peg or a
    bean, not a point or a wing: the round splays under pressure and ends
    in a blunt cap. *Workaround:* riggers (0.3–0.4) lifted to zero
    pressure with long release ramps. There is no pointed-tip model; a
    real sable round comes to a point when lifted.
13. **Preview and full render are different paintings.** Masks at a
    different resolution accept different stroke centers, so the RNG and
    every later mark diverge (notes/workflow.md). In practice the
    composition holds, but the look of every detail (specks, tick
    fields, dot textures) only shows at 3200px. The 1000px preview is
    useful for composition and value only; I judged all handling in
    3200px crops.

14. **`Style::blend()` isn't clipped, so fusing a masked passage smears
    it across the mask's edge.** Every badger pass over the fog (valley
    mist, near-range veil) dragged wet pale paint below the fog's mask onto
    the bare area where the knoll would go, and the knoll's body color
    later showed it in its gaps as pale blue flakes. That's physically what
    a badger does, but a painter fusing a cut passage keeps the blender
    inside it. *Workaround:* `.clip(true)` on every blend pass (5 of them).
    The preset should inherit the passage's clipping, or at least say it
    doesn't in its doc.
15. **Judging values is harder than it should be.** I spent a while
    debugging a "pale" foreground rock that was sRGB 45–98 painted over
    turf of about 30. The engine was right; the value relation was wrong.
    Probing `c.under()` after each pass and reading the PNG pixels found
    it. A tiny `Canvas::probe(label, pts)` that logs the look at a few
    points after every `work` in debug runs would have saved this.

16. **Palette families cut both ways, and nothing tells you which tube a
    speck came from.** Orange specks in the ranges at 3200px could have
    been bare pink ground or a vermilion-heavy pile from mixing jitter. I
    couldn't tell which: `Canvas` keeps no record of which pile laid a
    pixel. I did both fixes (a fuller cool underpainting and a red-free
    family), and the specks went. But the red-free family also made the
    far range a neutral gray, reading as snow: Friedrich's violets need
    smalt plus cinnabar [KÖR fig. 6]. The far range now has its own
    "violet" family with vermilion back in, and no specks came back, so
    the culprit was probably the bare ground all along. A debug layer
    ("which stroke/pile made this pixel") would settle such questions in
    one look.
17. **Timings** on the shared machine: 1000px whole ≈ 25–30 s. 3200px
    whole ≈ 95–125 s. 3200px crop of 350×120 units ≈ 30 s, and 4–10 s
    resumed from a late stage. The crop plus resume loop is what made
    detail work possible. Checkpoints for whole 3200px renders would be
    about 440 MB per stage, so I only checkpointed crops and previews.

18. **A `--stale-ok` resume doesn't refresh the checkpoint it resumed
    from.** Once the prefix of stage X has changed, every later `--resume
    X` needs `--stale-ok` again, even with `--ckpt`, until a full run
    repaints X. Harmless, but it trains you to pass `--stale-ok` by reflex,
    which defeats the check that exists to protect you (item 4).

## Critique

Judged at 1000px and in 3200px crops (the last full render is
`out/fresh2_mountains_full.png`).

What works:
- **The value structure and the dawn.** A dark near stage, a bright
  empty middle (the fog), then three ranges stepping back: dark wooded
  near range, cool blue-gray mid range, grayed violet far range and
  Schneekoppe against a lemon glow that cools through rose into gray-blue.
  It reads as contre-jour before sunrise, the air thickening with
  distance. That is the Friedrich structure, and it came from values, not
  from detail.
- **The fog sea.** Brushed thin, fused, then stippled, with lit billow
  tops and cool hollows. It lies flat, glows and has a torn upper edge
  against the dark woods. At 3200px it is paint: soft stipple over a
  smooth lay-in.
- **The Rückenfigur.** The wanderer by the tor is small, dark and sharp,
  with a thin warm rim on his right side. He is the one hard silhouette,
  which is how Friedrich uses his figures.
- **The particular details:** the crescent with pointed horns turned
  toward the hidden sun, the chapel nub on the Schneekoppe, the four birds,
  the spruce tips along the near crest, the backlit tufts breaking the
  knoll's skyline, the dead spruce among the living. They carry the "no
  gradation of detail by significance" feeling at 3200px.
- **Surface.** The craquelure is now an isotropic web of the right scale,
  the varnish warms everything a little, and the stippled sky has the
  grainy-smooth, strokeless look that Friedrich's pooled thin paint
  should have.

What doesn't (harshly):
- **The mid range was a curtain; now it's quieter but generic.** Strokes
  down the fall lines plus regular `Ridge` gullies made even vertical
  streaks: a flat-topped wall hung with drapery. I made the gullies fewer
  and shallower (46 units, carve 0.28), halved the spur/gully contrast and
  added large irregular buttresses (noise, widening downhill). It now
  reads as a soft, distant ridge, but a generic one. The gullies are
  still noise, not a drainage pattern.
- **The knoll is a big, quiet, slightly too smooth dome.** The lower left
  third is mostly dark olive with ragged mats, heather and a few stones.
  It's dark enough to be right, but its silhouette is a clean curve
  except where the tufts break it, and its surface lacks the
  particularity (bilberry, a gray lichened stone, a bent grass stem
  lit by the sky) that Friedrich puts right under our feet. The
  foreground outcrop helps weight the corner but is barely legible.
- **The tor and the figure are small** for the size of the dark stage.
  They're right for Friedrich's scale, but the tor is still a slightly
  soft, loaf-edged lump at 3200px. Real woolsack granite has sharper
  horizontal joints and a flatter bed-and-cushion rhythm.
- **Some handling still reads digital at 3200px:**
  - the woods' hatching along the near range (a regular field of
    dashes where the veil thins);
  - the needle hatches of the spruces bead into rounded dabs;
  - the crest tufts are short, blunt marks rather than hair-fine blades;
  - the stippled mist at the far range's foot is granular ("granite")
    at full size.
- **The clouds are good but minimal.** Two thin stratus bands and a wisp.
  A Friedrich dawn would have more considered cloud drawing.
- **The foreground's orange-brown touches** (a few bare-ground or umber
  flecks) read at 3200px as dead leaves. That's fine, but it's luck, not
  intent.

If I had another session: gullies on the mid range drawn from a real
drainage pattern (my own fall-line strokes, not noise), a
bilberry-and-lichen pass on the knoll, sharper horizontal joints on the
tor, and needle strokes for the spruces that overlap and droop rather
than dab.
