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
wind-thinned spruces (one dead). More spruces go down the flank into the
mist, which climbs toward them. Four birds cross the glow toward the sun.

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
2. **underpainting**: a thin brushed wash, umber under the knoll and
   blue-gray under the near range and valley. This came out of a problem
   (see FRICTION 3): without it, the pale top ground flashed through the
   dark foreground as white specks.
3. **sky**: a lay-in in long level elbow arcs, a shade duller than the
   target, fused top to bottom with the badger, then stippled into the wet
   lay-in (stippler 3 units, coverage 2) to break the strokes.
4. **sky light**: once dry, a second, finer and paler stipple, denser toward
   the glow. Then the clouds are stippled with a slight horizontal drag:
   violet-gray bodies with rose-gold lower edges near the sun.
5. **far range**: one `Form` holds three `Ridge`s, lit from behind and
   above right (contre-jour, `front` −0.35). Each is painted with my own
   passes: body strokes down the fall lines, colored by shade and aerial
   perspective, then a stipple aimed at the same colors so the strokes
   dissolve.
6. **far mist**: a stipple veil (`aim(false)`, density = tone) at the foot of
   the far range, thinning upward in noise banks.
7. **mid range**: the same method, darker and cooler, then mist at its foot.
8. **near range**: body strokes, then spruce woods hatched upright where a
   noise field says forest, then single spruce-tip flicks along the crest
   where the wood reaches it, then mist rising from the valley at its foot.
9. **valley mist**: the sea of fog. Long level strokes of thin paint with
   `load_at` giving the density, fused with the badger, then stippled. Its
   color has lit billow tops and cool hollows, and it is grayer lower down,
   toward the knoll's shadow.
10. **knoll**: body color in strokes following the swell of the ground.
11. **heather**: rust-brown patches hatched upright, a thin band of dead
    grass along the crown's edge, and about 20 stones (a dark underside
    stroke and a pale upper-edge stroke each).
12. **tor**: a `Form` of four rounded blocks (sunk into the turf) plus a
    fracture plane and grain. My own passes: a dark lay-in down the planes,
    stiffer paint on the lit planes, dark joints where `bend` < 0, then the
    turf color tucked back over its foot.
13. **knieholz**: dark mats hatched in crossing strokes on the knoll's
    shoulders.
14. **spruces**: my own spruce. The stem is one stroke lifted toward the
    top. Tiers go out and down, each with a few hanging needle hatches.
    The left side is thinned by the west wind. The dead one is a gray stem
    with alternate bare claws.
15. **slope trees**: smaller spruces going down the flank, then mist
    stippled up over their feet.
16. **figure**: the wanderer, stroke by stroke in a local frame:
    - legs, boots, the coat in five overlapping strokes, shoulders, arms;
    - the stick;
    - neck, hair and a glimpse of cheek;
    - a thin warm rim of dawn light down his right side.
17. **grass**: about 2,000 rigger flicks, curving upward from the knoll's
    edge downward. Pale ones only right at the crest, against the mist.
18. **birds**: four small V-strokes over the glow.
19. **finish**: varnish, craquelure (finer than the aged preset: FRICTION 1)
    and relief light.

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
5. **`--stop X` does nothing when resuming from X.** `--resume knoll --stop
   knoll` ran to the end: the stage is "in checkpoint", so its end never
   fires the stop. Wasted a render; I used `--stop` on the next stage
   instead.
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

## Critique (to be updated)

(after the full render)
