# r10_winter_a: "Dolmen in the Snow at Dusk"

Program: `paintings/src/bin/r10_winter_a.rs`. Renders: `out/r10_winter_a.png`
(1000px), `out/r10_winter_a_full.png` (3200px).

## Composition and why

A winter evening just after sunset on the flat Pomeranian land. A Hünengrab
(a passage grave: a great domed capstone on three uprights) stands on the
crown of a low snow rise, right on the skyline. A big bare oak rises beside
it and spreads over it. A smaller, broken-topped oak stands behind on the
left, against the brightest part of the afterglow. On the flat horizon lies
a thin blue band of far woods, with a church spire far off on the right.
Above is a young crescent moon, and low in the glow the evening star. The
sky runs from a cool cobalt-gray at the top down to a pale lemon glow. In
the foreground a lone man in a dark coat stands in the snow with his back
to us, leaning on a stick. His footprints lead in from the lower left.
Crows sit in the oak and two fly toward the woods. Dry grass and a few tall
dead weed stalks stand up through the snow.

What it draws on (from knowledge, not pictures):
- **Dolmens and oaks in snow.** Megalithic graves with oaks around them are
  one of Friedrich's recurring winter subjects (the Hünengrab pictures of
  c.1807 and after). The oak is his emblem of age and death, usually bare
  and stag-headed.
- **The Rückenfigur.** A small figure seen from behind, dwarfed by the
  landscape, stands in for the viewer. I placed him low and left of center,
  facing the grave: the living man before the ancient dead.
- **A low horizon and a big stippled sky.** Friedrich's skies are stippled
  thin over a warm ground (NG p.56; CATS p.127). The dusk gradient, with its
  lemon glow at the horizon, is his most typical evening light.
- **Moon and evening star.** A young crescent in the west after sunset,
  with its lit limb turned toward the sun below the horizon, as it should
  be. He painted such moons again and again.
- **Distant spire.** A church on the horizon of a flat land, as in his
  Greifswald views.
- **Materials.** The `Style::friedrich()` ground (a warm red-ocher lower
  ground under a brushed lighter top ground). The post-1820 palette
  (`Palette::friedrich_1820`: lead white, cobalt, pale smalt, ochre, red
  earth, vermilion, umber, bone black, chrome yellow), so the picture is
  dated c.1822. Families are set out per passage (`pal.only`) as a painter
  sets out the colors for a sky or for snow. There is a graphite
  underdrawing: the horizon ruled against a straightedge, then the rise,
  the capstone and the trunks drawn freehand. Grass goes on last over the
  finished snow, in "fine upturning strokes" (NG p.56). The whole is
  darkened a little toward the edges and foreground with a transparent
  glaze, after Friedrich's advice to Carus (MET PDF p.35), weighted to the
  bottom so the sky stays clear. Varnish and craquelure are softened: a
  well-kept picture.

## Working method, stage by stage

1. **drawing**: a 2H horizon against a ruler; the knoll line, the
   capstone's top and the oak trunks in HB.
2. **sky**: laid in thin with the broad filbert in long, nearly level arcs,
   a shade darker than wanted (a lay-in dries into the ground). Fused with
   the badger, then stippled into the wet paint with a 3-unit stippler aimed
   at the sky's own tones. Dried.
3. **glow**: a finer, lighter stipple (1.6 units), denser toward the
   horizon and toward the glow. (Cloud streaks were tried here and removed:
   they read as pink smudges, and a clear evening is more Friedrich.)
4. **moon**: first three curved drags of a round sable (a blunt banana of
   even width at full size, rejected). Now the crescent is a mask: the
   disk beyond a terminator ellipse, with its lit limb turned toward the
   sun below the horizon. It is thick in the middle and sharp at the horns,
   worked in small strokes around the curve. The rest of the disk is a
   breath lighter than the sky (earthshine, "the old moon in the new
   moon's arms"). The evening star is a single touch.
5. **far**: the far woods band and the far snow plain in hatching. Single
   far trees as tiny pointed flicks, grouped and uneven. A church (nave and
   tower in `detail` strokes, the spire a rigger flick).
6. **field**: the snow in long broad strokes following the lie of the
   ground (the angle field is the knoll's slope, fading out downhill), then
   the badger, then shorter body strokes in the foreground. The color field
   `snow()` models drifts as a stretched height field lit from the west
   (the glow side). The hollows go bluer, and a bank across the middle
   ground has a lit lip and a shadowed face. The far slope is warmer and
   lighter; the near snow is cooler and darker. The foreground body
   strokes are fused with the badger (without it each stroke read as a
   gray lozenge at 3200px), and the palette jitter is turned down for the
   snow.
7. **drifts**: wind-carved crests across the near snow: thin soft-mask
   bands under each crest worked in `detail` strokes a little darker and
   bluer than what is there (`color_over`), then a lit lip along each
   crest. (Two failed tries: hand drags with a lean filbert gave rows of
   blue ovals; a `body` pass in the band spilled the 9-unit filbert far
   past it into flat blue smears.)
8. **dolmen**: the dark hollow under the capstone; the far upright in
   shadow; the uprights and the capstone in `detail` strokes. The stones'
   color is a function of place: cool sky-light on top, dark undersides, a
   warm west end, reflected snow-light low down, and mottling at two
   scales. The capstone's strokes follow its dome. A lighter broken pass on
   the upper planes (a fbm facet mask with `color_over`), fissures, lichen
   touches, then snow on the capstone's top (thicker in the middle, patchy)
   and on the uprights' tops and the half-buried boulders. The capstone
   went through three shapes (a plank, a mushroom, a pointed lens) before
   the final one: a superellipse profile, domed above and flatter below,
   with blunt round ends, tilted a little.
9. **oaks**: my own oak habit (see friction #1), painted limb by limb from
   the trunk up. Then crooked claw twigs along every limb (two levels);
   bark furrows on the trunk; a dull warm rim on the glow side of the big
   wood; snow on the upper edge of level limbs; snow drifted against the
   foot.
10. **figure**: first the footprints (heel and toe touches, aimed a
   little darker and bluer than the snow, larger as they come nearer). Then
   a greatcoat mask with shoulders, a waist and a flared hem, worked in
   dark vertical strokes. Trousers and boots, the right arm bent to the
   stick, three cool folds down the back, a collar and head, a top hat (a
   mask plus a rigger brim), the stick, and a faint warm rim on the glow
   side. (The first version was a bell-shaped sack in a cowboy hat.)
11. **crows**: perched ones found on level stretches of upper limbs (a
    body stroke, a head touch, a rigger beak); flying ones as two pointed
    wing flicks and a body touch.
12. **grass**: tufts along the bank's lip, in the near snow and at the
    dolmen's foot. Flicks of a *pointed* rigger and small sable, pressed
    lightly at the root and lifted off; heights skewed (most short, a few
    long); four muted straw and umber paints. A few tall weed stalks with side shoots and seed heads.
13. **veil**: the transparent edge glaze. Then `finish` (varnish, softened
    cracks, relief).

## FRICTION

(Running list; most important first at the end in the summary.)

1. **The growth model didn't give me an oak I could use.** `Habit::dead_oak`
   and `Habit::oak` at 370 units gave either a pole with a few tufts at the
   tips or a narrow two-pronged tree. The limb count swung from 56 to 398
   between seeds with the same habit. `decline: 0.15` still left 49 of 56
   limbs dead. `abort: 0.7` produced a limb that looped back on itself and
   a bare pole. The pipe-model widths keep limbs thick almost to their tips
   and end them in a tuft ("noodles"; motifs.md already lists it).
   Workaround: I wrote my own recursive oak (`my_oak`: a short heavy bole
   splitting low into crooked, elbowed limbs, with broken and dead ones) and
   emit it as a `Skeleton`, so I keep the engine's data type and paint it
   with my own hand. I also tried a `taper()` remap of the widths
   (w' = W(w/W)^k), which helped the look but not the structure.
2. **Helper functions invalidate every checkpoint, and I missed the fix.**
   Any edit to a helper above `main` (a color function like `snow()`, or
   `cap_top()`) makes every stage stale, including the sky, which never
   calls it. For the first half hour I used `--stale-ok` constantly, which
   defeats the safety check. The fix exists: wrapping the helpers in
   `// ckpt: from field` … `// ckpt: end` (workflow.md, the staleness
   model). But README and run.rs describe the tag as being for "setup
   written after the block" and for lines "between blocks", so I read it as
   not applying to functions. A sentence saying "wrap helper functions
   too" would have saved many stale-ok runs. The tag is also unchecked: if
   I tag a helper the sky does use, the sky's checkpoint silently looks
   fresh.
3. **Pale "confetti" from lean dry-brush strokes.** A few dozen lean filbert
   strokes of a slightly lighter gray over a dark stone came out as small,
   bright, warm dashes, much lighter and yellower than the paint I mixed.
   Workaround: a masked `work` pass with `color_over(|u| shift(u, +0.06))`,
   which reads as planes.
4. **`Gesture::pressure(a, b)` and taper.** To paint a limb whose width is
   known at every node, I had to call `Tool::pressure_for` at each end and
   break the limb into sections, because pressure is only linear from start
   to end (or `swell` knots, which are relative). A per-point pressure (or
   per-point width) on `Gesture` would let one stroke follow a known taper.
5. **No way to swap a handling's tool.** `st.detail()` bundles tool,
   palette, medium and hand settings. To use a smaller filbert for the
   stones I'd have had to rebuild the whole handling with `Handling::new`
   and re-set a dozen knobs. A `.tool(Tool)` builder would do it.
6. **Crack visibility at full size.** `Finish::aged` craquelure dominates a
   3200px crop of a light passage (the snow, the glow). See #14 for the
   knob that worked.
7. **A fresh crop pays for the whole canvas.** A 3200px crop still
   builds every whole-canvas mask and plans the whole sky stipple (15–45 s
   per crop here). Resuming the crop from a checkpoint (`--resume field`)
   brings it to 2–10 s, which is what made the detail work possible. (This
   is documented; it's still the main cost of looking closely.)
8. **A limb's "up" side.** Putting snow on top of a limb needs the normal
   that points up. My first version offset in y by the width, which lands
   inside a diagonal limb as a pale stripe down its middle. Not an engine
   bug, but a `Limb::normal(i)` or `upper_edge(i)` would help every painter
   who puts snow or light on branches.
9. **`Palette::friedrich_1820` has no "smalt"**, so a family written for
   the early palette panics at run time ("no tube smalt"). Fine, but a
   compile-time or clearer message listing the palette's tube names would
   save a cycle.

10. **Pointed tips are opt-in, and I only found out halfway.** Every
    `Tool` preset, `rigger` and `round_sable` included, has `point: 0.0`
    (tip.md). So my grass, twigs and wing flicks were blunt tufts: every
    blade a thick wedge, like a thorn. A painter picking up "a rigger" for
    grass expects a point. Workaround: `Tool { point: 1.0, ..Tool::rigger(w) }`
    everywhere fine work happens. A `Tool::rigger` that comes pointed (and a
    `blunt()` to opt out) would match what the name promises.
11. **A brush running dry makes "ladder" bands.** A thick limb painted in
    one long drag (up to 10 nodes, ~80 units) ran out of paint partway, and
    under relief lighting the starved bristle tracks came out as regular
    light bands across the limb. This looks like the "ladder ridges"
    motifs.md describes. More medium (0.3) didn't fix it; reloading every
    ~35 units did. `Held::fullness()` exists, but nothing tells the painter
    during a drag that the brush has gone dry. A drag option to reload when
    fullness falls below x, or a warning, would make this easy.
12. **Aiming against the wrong underlayer.** `c.sample()` + `c.aim()` for a
    contact shadow under the man's boots sampled the boots I'd just painted,
    so "a bit darker than what's here" was aimed against black and came out
    as a *light* blue oval. My mistake, but `Canvas::aim` has no way to say
    "judge against the snow, not against the last marks". An explicit
    `under` color parameter (it exists on `Palette::aim`) is the workaround.
13. **A handling spills past a narrow mask.** A `body()` pass (9-unit
    filbert, unclipped by default) over a 4–8-unit band laid broad flat
    smears far outside it. Obvious in hindsight: `clip(true)` or a tool
    sized to the band. The presets don't warn when the tool is wider than
    the mask's features.
14. **Crack weight is set by `width_um`, not by `depth_um`/`dirt`/`grime`.**
    I turned down depth, dirt, grime and cupping without visible effect.
    Setting `width_um: Some(12.0)` (the default is strain × island size,
    much wider) quieted the network. The doc comments are accurate, but
    "which knob makes cracks less visible" isn't answered anywhere.

## Critique

Judged at 1000px and in 3200px crops, harshly.

**What works.**
- The big shape is Friedrich's: a low horizon, a quiet stippled sky from
  cobalt-gray to lemon, three dark silhouettes on the skyline (the broken
  oak, the grave, the great oak) and one small dark figure below. The
  values hold. Everything important is dark against the lightest band of
  sky, and the snow sits between the two in value.
- The great oak is the best passage. It is broad, elbowed and crooked,
  with broken limbs ending in splinters. Its twigs are a fine crooked lace
  at 3200px rather than "foliage", and it has no ladder bands or joins
  since the reload fix.
- The crescent: thick in the middle, sharp horns, lit limb toward the
  sunken sun, earthshine in the dark part. The evening star in the glow.
- The man is legible at 3200px as a man of the 1820s (greatcoat, top hat,
  stick). He is small enough to be Friedrich's, not an illustration.
- Evening light through the dolmen, under the capstone and between the
  uprights. This is the one moment of drama in the picture, and it reads.

**What reads as digital, or simply weak.**
- **The dolmen is still too neat.** The uprights are near-rectangular
  slabs with straight sides. The capstone is a smooth symmetric bun whose
  snow sits on it like icing with a hard lower edge. Real megaliths are
  lumpy, split and lichened. The stone texture at 3200px is flat,
  stroke-smoothed paint with almost no grain; every attempt at grain
  (dry-brush lights) came out as confetti, and I removed it. It's the
  focal point and the weakest-drawn thing in the picture.
- **The snow is smooth but empty.** After fusing the lay-in it reads as
  snow at 1000px, but at 3200px it's a soft haze with very little paint
  character. Friedrich's foreground snow has a slight impasto and a
  crisp, dense surface. The drift crests help but are too regular (all
  horizontal, similar lengths).
- **The grass is scattered evenly.** Clumped by noise and along the
  bank, but it still reads as sprinkled rather than growing where the
  ground would hold it (along the bank, in hollows, around the stones).
- **The small oak** is my big oak's habit at a smaller size, so the two
  trees rhyme too closely. Friedrich would make the second tree a
  different character (a stump, a pollard, a dead snag).
- **The far woods** are a flat dark hump at left and right, with no
  gradation into the haze. The spire is nice but tiny, and the church body
  is a lump.
- **Crows** are readable only in crops.
- The underdrawing is fully hidden: honest for a late picture, but it
  means that stage contributes nothing you can see.

**If I had another hour.** Build the stones with `Form` (an `Sdf` capstone
turned and cut, weathered, lit by sky light from above and glow from
behind) instead of hand color fields; give the second tree a different
habit; move the grass to where the drifts thin; add a thin impasto of
lead white on the foreground drift lips.
