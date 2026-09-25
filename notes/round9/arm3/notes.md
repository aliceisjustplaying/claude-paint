# Round 9, arm 3: a winter landscape at the easel

Source: `paintings/lua/r9a3_winter.lua` (easel session log).
Renders: `painting_1000.png`, `painting_3200.png` (this folder).

## Composition and why

A winter evening just after sunset. A low horizon (y 468 of 714) under a big
quiet sky, gray-violet above and a lemon-rose glow low on the left where the
sun went down. On a snow mound left of center stands an old dead oak, the
picture's main vertical, its crooked limbs against the glow (Friedrich's
*Oak Tree in the Snow*, the dead oaks of *Abbey in the Oakwood*: the tree as
a figure). To the right, lower on the slope, a leaning wooden wayside cross
half buried in snow, and a lone walker in a dark coat with a stick, seen
from behind, going toward it (the Rückenfigur; the cross in winter recalls
*Winter Landscape with Church* and *Morning in the Riesengebirge* without
copying either). Far off, a thin dark band of fir wood on the plain, hazed by
the evening air. Near foreground: snow with a few dark stones, dry grass and
bramble stems poking through (grass laid over finished snow, as NG p.56
reports for *Winter Landscape*), and the walker's footprints.

## Method, stage by stage

1. Canvas (friedrich style, after-1820 palette), aspect 1.4.
2. Underdrawing: horizon ruled in 2H, mound and oak trunk sketched, cross
   and walker drawn in HB.
3. Sky: two broad lay-ins wet into wet (long sweeps, then shorter fuller
   ones to close the gaps), then one blend. No stipple.
4. Snow in one wet lay-in with the mound modeled inside the color (face
   toward the glow light and warm, the plain in its lee cooler, the slope
   turning away blue-violet, drifts), then a blend.
5. Far fir wood (`fir_wood`, two rows, hatched and hazed), snow pulled
   across its feet.
6. The oak (`tree_in`, winter, a drawn ragged crown, three tries to get
   away from a lollipop), nearly silhouette against the glow.
7. The wayside cross by hand with flats (post, bar, small gable roof),
   darkened with a glaze limited to the wood; snow on the bar and roof.
8. The walker: coat as a drawn outline filled with short vertical strokes,
   hat and head as small masks, legs, arm and stick as single strokes, a
   rim of glow on the left shoulder.
9. Snow drifted over every foot, each pull loaded with the snow sampled
   beside it.
10. Two stones by hand (the `rock` tool's versions looked worse, below).
11. The walker's way back: a broken trodden furrow and dents of his steps,
    spaced in perspective, each a cool filbert dab with a lit far lip.
12. Dry grass over the finished snow (NG p.56's "fine upturning" strokes):
    small clumps along the mound, by the stones and the cross, then a
    denser stubble and two bramble arcs in the bottom corners.
13. The far left under the glow: a broken line of hedges and a village
    church (tower, nave, spire), nearly lost in the air.
14. A thin young moon high right (its lit limb toward the set sun), the
    evening star low in the glow, three birds going over toward the wood,
    two crows hunched in the oak (set on real limb points read from
    `oak.limbs`).
15. A faint dark glaze growing toward the edges (his advice to Carus, MET
    PDF p.35), then varnish, cracks and relief.

16. Refinement after the first full render: the far hedge line broken into
    copses with lobed tops (it was a flat lavender stripe at 3200), the
    stone's drift lowered to cover a dotted sliver of rock under it, a
    drift carried round the right side of the oak's foot (the trunk ended
    on a straight cut), a few tiny bare trees out of the far copses, the
    walker's hair darkened (it read as a brown face patch at 3200), and
    the hedge kept off the oak's trunk.

Title: **Winter Evening with a Wayside Cross**.

## FRICTION

1. **Long broad sky strokes leave the ground showing.** The first
   `hand="broad"` pass (lengths 120 to 320, coverage 4.5) left dozens of
   orange-ground holes in the upper sky, and with `tool="flat 14"` and a
   `badger 18` blend, diagonal drag streaks too. Workaround: a second,
   shorter (60 to 170), fully loaded pass wet into the first, then a plain
   `blend`. Coverage 4.5 doesn't mean covered when the strokes run dry.
2. **`easel try --look` shows the rolled-back canvas**, so it can't preview
   paint, only overlays. Workaround: `do`, look, `undo`.
3. **`fill` is in the README but not in the Lua `work` options.** The
   README says `.fill(false)` lets the ground show between strokes; `work`
   rejects `fill=`. Workaround: coverage under 1.5.
4. **Wet over dry keeps every stroke's outline.** Shadows laid over dry
   snow with `color_over={shift=...}` came out as crisp blue outlined
   scribbles ("wire"); with explicit colors and a `blend` they still had
   hard, flame-like tongues past the region, since a blender can't move
   dry paint. Overrun strokes past a soft region (hug=false) are visible
   as separate tongues. Workaround: model the whole snow field inside the
   first wet lay-in's color function (edit chunk 5) and only then paint
   over it.
5. **The oak comes out a lollipop or a baobab.** `tree_in` with a round
   drawn crown gave an evenly filled broccoli on a thin stem; raising
   `girth` gave a thick column with a round bulb *below* the foot (the
   flare is painted as a disc past the foot point). Workaround: a wider,
   lower, notched crown, a short trunk, `girth=0.12`, and the stout wood
   clipped to `above(foot line)` so the bulb never shows.
6. **Flat brush `orient=<angle>` turned the flat edge-on**, a thin line,
   when I meant the stroke direction; `orient="across"` is what lays a
   broad flat mark. The first try also tapered the post to a point at its
   bottom, like a sword.
7. **A light shift in `load` plus `at=` aiming made the snow drifts glare
   white.** `b:load(color, 0.6, {at=...})` aims at the laid look and, with
   a partial load, mixes lighter to make up for thin cover; with my small
   lightness shift on top the drifts were whiter than any snow. Workaround:
   masstone loads (no `at=`), full load, the snow color sampled beside.
8. **The `rock` tool on small stones in snow.** Its per-plane stroke
   direction left vertical streaks, `blend` on its shadow family made a
   flat black void, and `r:snow{}` read as cotton wool at 3200. Workaround:
   painted the stones by hand (a poly mask, a light-to-dark color across,
   one long filbert pull for the snow cap, one for the drifted base).
9. **`glaze` waits for all paint to dry** (6.5 days on the clock) even
   when I only glaze the cross: fine for the physics, but it means any
   glaze forces the whole canvas dry, so wet-into-wet work elsewhere can't
   continue after it.
10. **Editing an early motif moves everything placed on it.** Raising the
    oak's `detail` in chunk 8 regrew different limbs, so the crows placed
    by eye in chunk 20 floated in the air. Workaround: read limb points
    from `oak.limbs` and seat the crows on them.
11. **A far horizon lost in the glow has no edge to paint up to.** At the
    left, the snow and the lemon sky sampled the same color (#dccdaa) for
    20 units around HZ; the far wood placed at the nominal foot floated in
    snow. Workaround: probe with `sample()` down a column, then paint the
    hedge line as its own mask.
12. **`easel do` versus `edit` numbering.** After `edit 8` the old chunk 9
    was replayed on the new tree, and my new chunk went in as chunk 10 on
    top of it. Easy to lose track; I had to undo and `edit 9`.
13. **`--scale 3.2` looks cost 15 to 25 s for each new window** (a whole
    replay) while two other painters share the machine; I kept windows few
    and reused them.
14. **Passes don't know what I painted in front.** The far hedge, laid
    after the oak, crossed its trunk as pale dashes at 1000 px. The depth
    tools (`w:layer`, `behind=`) would handle it, but only for motifs
    registered in a world; a motif painted from `tree_in` alone isn't.
    Workaround: multiply the hedge mask by `-oak:mask():grow(0.6)`.
15. **1000 px and 3200 px disagree on fine motifs.** The oak's fine twigs
    look like fuzzy "caterpillars" at 1000 and a clean lace at 3200; the
    far hedge was a smooth stripe at 1000 and a row of hard dashes at
    3200. Judging at one width misleads about the other; I had to look at
    both before each decision.

## Top five friction points

1. Wet over dry keeps every stroke's outline (shadows became wire or
   flame tongues); the only way to model the snow softly was inside the
   first wet lay-in's color function.
2. `tree_in` oaks come out as a lollipop or a baobab (a round bulb painted
   below the foot); it took a drawn, notched, low crown, a tuned girth and
   a clip at the foot line.
3. The `rock` tool on small stones in snow: vertical streaks, a black
   void after `blend`, cotton-wool snow caps; hand-painted stones were
   better.
4. Editing an early motif silently moves things placed on it (crows
   floating after the oak regrew); edits and chunk numbers are easy to
   lose track of.
5. Fine motifs look different at 1000 and 3200 (twigs, hedges), and each
   new `--scale 3.2` window costs a whole replay (15 to 30 s on a shared
   machine).
