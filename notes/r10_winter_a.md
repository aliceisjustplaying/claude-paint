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
4. **moon**: the crescent in three passes of a round sable along an arc,
   each set in from the last, with pressure swelling in the middle. The
   evening star is a single touch.
5. **far**: the far woods band and the far snow plain in hatching. Single
   far trees as tiny pointed flicks, grouped and uneven. A church (nave and
   tower in `detail` strokes, the spire a rigger flick).
6. **field**: the snow in long broad strokes following the lie of the
   ground (the angle field is the knoll's slope, fading out downhill), then
   the badger, then shorter body strokes in the foreground. The color field
   `snow()` models drifts as a stretched height field lit from the west
   (the glow side). The hollows go bluer, and a bank across the middle
   ground has a lit lip and a shadowed face. The far slope is warmer and
   lighter; the near snow is cooler and darker.
7. **dolmen**: the dark hollow under the capstone; the far upright in
   shadow; the uprights and the capstone in `detail` strokes. The stones'
   color is a function of place: cool sky-light on top, dark undersides, a
   warm west end, reflected snow-light low down, and mottling at two
   scales. The capstone's strokes follow its dome. A lighter broken pass on
   the upper planes (a fbm facet mask with `color_over`), fissures, lichen
   touches, then snow on the capstone's top (thicker in the middle, patchy)
   and on the uprights' tops and the half-buried boulders.
8. **oaks**: my own oak habit (see friction #1), painted limb by limb from
   the trunk up. Then crooked claw twigs along every limb (two levels);
   bark furrows on the trunk; a dull warm rim on the glow side of the big
   wood; snow on the upper edge of level limbs; snow drifted against the
   foot.
9. **figure**: the coat as a small mask worked in dark vertical strokes;
   legs, head, a low hat, a stick; a cool rim of sky-light on the
   shoulders; footprints as blue-gray touches dragged along.
10. **crows**: perched ones found on level stretches of upper limbs (a
    body stroke, a head touch, a rigger beak); flying ones as two wing
    flicks and a body touch.
11. **grass**: tufts along the bank's lip, in the near snow and at the
    dolmen's foot, as rigger and small-sable flicks pressed at the root and
    lifted off. A few tall weed stalks with side shoots and seed heads.
12. **veil**: the transparent edge glaze. Then `finish` (varnish, softened
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
   3200px crop of a light passage (the snow, the glow). Halving `dirt` and
   `depth_um` barely changed it; I also halved `cupping_um`. It is hard to
   know which knob controls the visible weight of a crack.
7. **Stages can't tell a crop render where to look.** A full crop still
   pays for every whole-canvas mask and the sky stipple (33–45 s for a
   crop). Resuming the crop from a checkpoint (`--resume field`) brings it
   to about 8 s, which is what made the detail work possible.
8. **A limb's "up" side.** Putting snow on top of a limb needs the normal
   that points up. My first version offset in y by the width, which lands
   inside a diagonal limb as a pale stripe down its middle. Not an engine
   bug, but a `Limb::normal(i)` or `upper_edge(i)` would help every painter
   who puts snow or light on branches.
9. **`Palette::friedrich_1820` has no "smalt"**, so a family written for
   the early palette panics at run time ("no tube smalt"). Fine, but a
   compile-time or clearer message listing the palette's tube names would
   save a cycle.

## Critique

(Written after the full render; see the end.)
