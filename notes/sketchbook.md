# The painter's sketchbook

Read this before you paint at the easel (`crates/easel/README.md`). It holds craft
(techniques with the numbers that worked, and the mistakes that cost painters the most
time), never finished pictures to copy. Credits in brackets: [r2 winter] is round 2 (Rust,
older API, `notes/amnesia2/`, only its lessons that translate to the easel), [r3 ...]
round 3 (`notes/amnesia3/`), [r4 ...] round 4 with the drawing tools (`easel4_green`,
`easel4_near`). Add what you learn in the same form: what to do, the recipe, what goes
wrong.

**Read notes/principles.md first:** the tools give you physics and constraints; you make the decisions, the way a person painting would (piles mixed on a palette, not color formulas; gestures, not fills; structure tools as scaffolds, not things to trace).

**These are the best techniques so far, not the best possible.** Every
recipe is a floor to beat. Each section names its **ceiling**: where the
best recipe still reads as schematic or digital. If a passage of yours still
reads digital, don't settle for the recipe: try something new, and if the
critic scores it better, replace the entry (don't keep both). Pitfalls stay:
they only prevent bad results.

**Hierarchical detail, not maximal subtraction** (Alice, Round 6).
Economy doesn't mean less detail everywhere; it means detail placed where
the eye goes and massed where it doesn't. Big shapes and value families
first, then a middle scale of irregular groups, then a few selected
particulars. The Round 6 lab (`notes/lab/`, `notes/round6.md`) found both
failures: too little (foliage as blobs or domes, a bare tree's twigs as a
tone that read as fur) lost to the old detailed recipes, and too much (an
even carpet of 40,000 touches, a starburst at every twig tip) read as
procedural noise. What won kept the middle scale: bare tree C (fewer,
varied, tapered twigs, all connected), water B, sky B, rock B.

---

## 1. Working order and time

- **Order.** Draw; the sky thin; the distance far to near; the ground as tone; the motifs;
  small particulars last with pointed brushes; grass last of all in fine upturning strokes
  over finished ground or snow.
- **Working in sittings (hand time) [r6 time].** *Principle:* a stroke costs the time a
  hand takes to make it, and a painter works in sittings of a few hours with the paint
  setting between them. Then what is wet when a passage meets its neighbor follows from
  the work, not from a rule. `canvas{..., hand=true}`, `sitting{hours=3}`, `rest(16)`,
  `timesheet()`, `look --mode wet` (`crates/easel/README.md`, notes/time.md). What
  follows from it:
  - In one sitting the paint under a later passage is still open and comes up into it.
    Lay each passage only where it shows, a little (about 10 units) past where its
    neighbor will meet it. A ridge painted to the bottom of the canvas and then a dark
    knoll over it in the same sitting came out cobbled with churned-up light paint. Laid
    only where it shows, the knoll stayed solid, with a soft shoulder against the ridge,
    and the ridge took 42 minutes of hand time instead of 90
    (`notes/time/example_three_ways.jpg`).
  - The time of an edge decides its character: painted into open paint it is lost (a
    hill's top drags the sky down), after lunch (`rest(4)`) it drags less, and the next
    day (`rest(16)`) it is crisp like `dry()` (`notes/time/edge_timing.jpg`). Put the rest
    where you want a found edge.
  - Read the timesheet as the price of detail: the fir wood in `l5_near` is 53 hours of
    hand time (211,000 hatched strokes), the whole sky 32 minutes.
  - *Ceiling:* the times are estimates (Fitts and the steering law are sourced; the
    palette trips, which are most of the time, are guesses), a pass ages in 15-minute
    slices from the top down rather than along a painter's path, and how open paint
    looks where two passages meet (a pale, streaky drag band) is still unresolved. The
    `dry()` rule below stands until the wet-on-wet work decides it.
- **`dry()` before any passage that goes over earlier work.** `wait(24*60)` is often not
  enough. A thick body floor (coverage 4.5) was still "open" after a day, and the rock
  laid over it went semi-transparent [r3 near]. Snow over a day-old near-black wood
  smeared into a gray fog band [r4 near]. A river over wet valley paint plowed through
  to the red ground [r4 green]. If you aren't sure, `print(drying(x, y))` at the spot
  first.
- **Fine lines only on dry paint.** Cracks drawn into open paint come out dashed like
  stitching [r3 near]. Stones painted 4 h after the grass came out green-streaked; a day's
  wait fixed it [r3 green].
- **Waits that worked.** Sky, then `wait(24*60)` and the second sky layer or stipple [r3
  green, r3 near]. `wait(180)` was enough for a sky stipple over a thin (medium 0.3)
  lay-in [r3 free]. `wait(120)` to `wait(3*60)` before hatching a wood or a tree over a
  fresh hill worked [r4 green].
- **Glaze timing.** `glaze()` waits until everything under it is touch-dry (8 to 29 days
  of clock each) [r4 green], so never put one inside a wet-into-wet sequence. A veil that
  doesn't wait: `work(m, {hand="glaze", medium=0.8, color_over={shift={...}}})`.
- **Finish old.** The standard last chunk is `wait(24*60); varnish{...}; cracks{}; relief()`:
  a Friedrich is two centuries old and cracked, and Alice wants the craquelure as part of
  the final look (Round 6). Judge glitches with and without it: cracks are marks too.
- **Retouch before the varnish.** Keep `wait(24*60); varnish{...}; cracks{}; relief()` as the last
  chunk and fix things with `easel edit N --insert` before it: seconds, not a replay [r4
  near, r4 green]. Since round 6 the varnish (and any `glaze()`) coats every stroke ridge
  and pools only a little in the hollows, the same at 1000 and 3200: no need to thin
  `coats` to 0.12 against brown worm lines at paint edges (`notes/varnish.md`) [r6 varnish].
- **Keep `relief()` at its default.** The Friedrich default is now `(0.06, 0.006)`, the
  Alice's pick from the Round 6 A/B (`notes/round6/relief/`): the old 0.2 embossed every
  stroke into creases ("grooves") and raised outlines around sky holes. `relief(0.6)`
  turned a sky into swirling impasto [r3 free]. Relief lighting can't fix pale rims,
  halos or cut-out edges: those are in the paint.

## 2. Skies and air

*Ceiling:* skies read as smooth, even gradients ("too neat") and impasto cumulus reads lumpy; nobody has yet painted a Friedrich sky with real structure (banks, breaks, an uneven glow) that also reads as paint.

- **The sky is two thin layers, the second stippled over the first once it's set.** This
  is the move that most often made a sky read as Friedrich's rather than as scrubbed paint
  [r3 free, r3 near, r3 green, r2 all].
  ```lua
  skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth"}
  work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=skypal})
  blend(skym, {angle=0})
  -- next chunk
  wait(24*60)
  stipple(skym, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55})
  ```
  With a clouds object, a second broad pass works too: `wait(24*60)`, then `work(...,
  {hand="broad", color=cl, coverage=3.5, medium=0.35, aim=1.6})` and `blend` [r3 green].
- **Restrict the sky palette.** Leave out the greens. Add chrome yellow and vermilion only
  for a warm glow [r3 free]. A full palette flecks skies and ranges orange where the mix
  jitters [r2 mountains].
- **Lay the first sky a shade duller than the target;** the stipple lifts it [r2 winter,
  r2 coast].
- **A stipple lighter than the field reads as salt where it thins.** Lift only about 0.006
  to 0.028 in OKLab L above the field and keep coverage at 0.9 or more everywhere [r2
  coast].
- **Never stipple a pale veil over a dark passage.** Every dot stands alone (static,
  "frogspawn" at 3200). Mist over dark hills is a brushed veil instead: `hand="broad"`,
  medium 0.6, density through `load_at`, then a clipped `blend`. Keep stipple for the sky
  and already-light fog [r2 mountains].
- **Clouds.** Light on the lit tops a day later: `hand="scumble"`, warm white,
  `coverage=2.2`, `medium=0.35`, `hug=false`, through `cl:mask{alpha={0.45,0.85},
  lit={0.5,0.9}}:blur(3)`. Darken the bellies with `hand="glaze"`, `medium=0.8`,
  `color_over={shift={-0.05, 0, -0.012}}` through a `:blur(4)` shade mask. Then blend the
  grown cloud mask. A `stipple` on cloud edges gave white speckle, and a hard-masked
  `glaze()` gave gray slabs [r4 green].
- **An afterglow** that `w:sky` got wrong (it read as noon): your own gradient with a
  Gaussian `math.exp(-((x - SUNX)/520)^2)` added to its t [r3 free].
  Then paint it from piles, not from the formula (section 12, "Mix piles, don't paint a
  formula") [r7 piles].
- **Haze at the foot of a range:** a graded `glaze(hazem, {coats=0.55, pigment="semi"})`
  and then a fine, low-contrast stipple (`width=1.5`, `aim=false`, `medium=0.6`,
  `fade=1`). A coarse stipple there read as frost [r3 green].
- **Moon.** The crescent is a mask (disc minus an offset disc) filled with short touches
  clipped to it, so the horns come to points. A dragged arc gives a blunt banana. A glow
  is stippled, not glazed [r2 winter, r2 mountains].

## 3. Water

- **Calm sea = the mirrored sky, darkened toward you.** Color field: `refl = skycol(x, HZ
  - d*2.2 - 10)` (d = y - HZ), mixed into a dark that deepens over 120 units, plus a
  stretched noise (`stretch={0, 30}`) for faint current lines [r3 free].
- **Smooth a blotchy sea the same way as a sky:** stipple a veil of its own gradient
  (`width=2.2`, `coverage=2.6`, `medium=0.5`), then while wet `blend(veil, {angle=0,
  angle_jitter=0.003, length={80, 200}, coverage=2})` [r3 free].
- **Glints after `wait(240)`:** horizontal rigger (0.9) strokes, pale ones mixed toward
  the mirrored sky and darker ones in the troughs (round 1.6), spaced closer near the
  horizon (`y += 1.2 + d*0.055 + rand(0, 1)`), longer and more numerous toward you,
  gathered under the glow [r3 free].
- **No flat brush in `broad` over water:** brick marks, and it dragged wet paint from
  neighbors [r3 free]. Calm water wants whole level strokes, no `broken` or `tail`, over
  an underpainting in its own tones (a flat one showed through where strokes ran dry) [r2
  coast].
- **A river:** `w:ribbon(pts, width_m)` in meters, laid after the valley has set. Author
  canvas points and convert each with `w:to_ground` (keep `{p[1], p[3]}`) so it narrows by
  itself [r4 green].

## 4. Distance and ranges

*Ceiling:* ranges still read as parallel bands or boxy crests; far woods as hedges of repeated round blobs.

- **`w:ranges` can come out boxy** (crenellated crests) [r3 green]. Hand crests are
  reliable: noise plus a Gaussian dome or two, e.g. `HZ - 16 - 10*n:at01(x,0) -
  44*math.exp(-((x-770)/105)^2)`, masked with `below(crest):roughen(0.8, 9)`, body strokes
  25 to 70 long at a near-level angle, color from crest to foot [r3 green].
- **Stroke ends crenellate any hard crest.** Cut the sky back over it: sample the sky 26
  units above the crest, lay it over the edge with a slightly noisy line and blend. Fade
  the cut-in in with `smoothstep`, never stop it at a vertical seam (visible at 3200).
  Keep it away from wet motifs, which it smears along the horizon [r3 green].
- **Each range a clear step darker and cooler than the one behind.** In contre-jour, near
  forms go darker than far ones [r2 mountains].
- **Fields in perspective:** color the plain per point from ground coordinates. `worley`
  cells in rotated meters (`U/55, V/95`) gave fields of the right size; 140 m cells were
  giant blotches. Try the scale with `easel try` and a probe first. Haze each field with
  `mix(c, sk:airlight(x), w:aerial(Z)*0.95)` [r4 green, r3 green].
- **Hedgerows and groves:** hatched (round 1.2 to 1.6, lengths 1.5 to 5) with turning
  angles, sized by eye (by `w:height` at a high eye they were pillars). Rows of round
  crowns, identical field trees and thin ruled hedge masks read as stamps and lines: vary
  size, spacing (`uneven`), shape and shadow, and break hedges with noise [r3 green, r4
  green].

## 5. Trees

*Ceiling:* oaks come out as umbrella/savanna crowns or "broccoli"; spruce tiers as regular chevrons; field trees as identical balls; wood interiors as flat black masses. No painter has yet drawn a tree with its structure, only built one from parameters.

**Broadleaves from a drawing: `tree_in{}` and `tree_group{}` (THE TOOL TO USE for oak,
beech, lime, birch and willow; use it before `tree{}`/`t:foliage{}` or a hand-built crown).**
Draw the crown as an `outline{}`, lopsided and lobed (a symmetric blob grows a
lollipop), and the trunk from the foot up. Then
`t = tree_in{crown=o, trunk={{x,foot},...}, species="oak"|"beech"|"lime"|"birch"|"willow",
season="summer"|"spring"|"autumn"|"late_autumn"|"winter", sun=WORLD, seed=}` grows the limbs
from the trunk into your crown, with the crown's own gaps, leaf clumps on the twigs and hooked
touches on the clumps [trees_in study, `paintings/lua/trees_in.lua`]. Paint it in this order:
1. Wood: `work(t:wood(3.5), {hand="body", tool="round 2", ...})`. Put the lit flank only on
   `t:wood(7)`, since a flank on thin limbs turns them into pale lines. Then
   `t:paint_wood(brush("round", 2.4), {min=1.2, max=3.5})` and `brush("rigger", 0.9), {max=1.2}`.
2. The leaves' body: `work(t:leaves(), {hand="hatch", tool="round 1.6", hug=false, clip=...})`,
   colored from `L = t:light()`: `mix(mix("#222b1d", "#35412a", smoothstep(0.1, 0.45, v)),
   "#5c6a3a", smoothstep(0.5, 0.85, v))`. A single body color gives one dark blob.
3. Sky into `t:gaps()` **before** the touches, or the holes keep hard cut-out edges.
4. Touches, `b = brush("round", t.touch_w)`: `lit={0, 0.4}` `#263020`; `lit={0.15, 0.4},
   depth={0.25, 1}, share=0.5` in a cool `#4a5446` (the front shade masses catch the sky);
   `lit={0.4, 0.6}` `#46522e`; `lit={0.6, 0.78}` `#6f7c45`; `lit={0.78, 1}` `#98a060`, `every=6`.

A bare tree: the same crown, trunk and seed with `season="winter"` grow the same wood (even
moved across the canvas). *Principle: draw the structure, select the twigs, leave the sky.* A
painter draws the big limbs whole and a few twigs with character, and leaves the rest out.
`tree_in` does the picking: `detail=` (default 0.35 bare) is the share of the fine wood drawn,
always with the wood it leaves from. An oak's wood is angular (straight runs, elbows at the
nodes, short stiff twigs; `angular`, on by default for oaks) [bare_trees study,
`paintings/lua/bare_trees.lua`, notes/oak.md]. Paint the wood thick to thin, each band with a
brush that can lay its widths:
`t:wood(3.5)` as body paint (lit flank on `t:wood(7)`), then
`t:paint_wood(brush("round", 2.4), {min=1.2, max=3.5})`, then
`t:paint_wood(brush("rigger", 0.9), {min=0.5, max=1.2, every=2})` and
`t:paint_wood(brush("rigger", 0.55), {max=0.5, pressure=0.04, every=2})`, the fine wood a shade
lighter than the limbs. The bands are local widths, so nothing floats. A single `rigger 0.55` for
everything under 1.2 paints it all one width (it lays 0.21 to 0.71): no taper at the forks.
*Don't* lay a tone for the undrawn twigs (`t:twig_mass()`): a blind panel of four critics called
it "steel wool", "fur" and "gray scribble cushions" and preferred the drawn twigs, and toned trees
look heavier and smeared.
Paint only `share=0.4` of the dead leaves an oak keeps, or they read as burrs.
*Ceiling:* at 1000 px the drawn twigs are sub-pixel hairlines and read as faint lines; the
oak's crown is still a model's (limbs cross each other more than a real crown's do), and a few
sibling limbs run side by side like doubled lines.
Field trees: draw 2–3 different crowns (a broad low one, a tall narrow one) with short
trunks, then `tree_group{crowns=, trunks=, species={...}, count=6, horizon=HZ, spread=0.8}`.
Paint far to near with `t.haze`, and lay one `g:shadow():blur(1.5)` glaze at 0.25. Don't
`roughen` the shadow: it breaks into speckles.
*Still weak* [trees_in study]: sky holes have crisp torn-paper edges at 3200; a beech's
leader reads as a gray pole through its crown; a birch's white stem runs high into the crown.

**Firs from a drawing: `fir{}` and `fir_wood{}` (use these before hand-building a spruce).**
Draw the silhouette as an `outline{}` (apex, flanks, the crown's base) and
`fir{envelope=o, foot={x, y}, habit="spire"|"old"|"young"|"storm", seed=}` grows
a spruce into it: uneven whorls, gaps in runs, dead and broken lower boughs, tips
on your line. Paint it in this order [firs study, `paintings/lua/firs.lua`]:
1. `work(f:needles(), {hand="hatch", tool="round 1.8", length={3, 7}, coverage=2.4,
   clip=f:needles(), angle=...})`, dark `#1c2621`→`#25302a` down the tree. Use a
   **continuous** angle, `1.57 + 0.5*clamp((ax - x)/12, -1, 1)`. An angle that flips
   at the stem leaves a pale seam down the tree at 3200.
2. Boughs with a rigger 1.2 (`f.boughs`, dead ones `#6a655d`) and
   `f:paint(brush("rigger", 0.6), {color="#77716a", kind="twig"})`. Stroke the stem
   only below `f.crown_base - f.tier`; a full-length stem stroke reads as a pale line.
3. `f:paint(brush("round", f.hatch), {color="#18211d", lit={0, 0.5}})`, then
   `#34402f` for `lit={0.5, 0.7}`, then `#667052` with a brush 0.8x for
   `lit={0.7, 1}, every=6`. These are the short hatched strokes, hung from each
   bough, lit by the sun you passed (`sun=`).

Draw each envelope yourself: lopsided, a torn flank, a notch for a lost top. A
symmetric envelope still gives a Christmas tree. `old` at 400 units reads as
curtain branches hanging off a crooked stem, `young` as a dense cone to the snow.

A wood: `wood = fir_wood{skyline=outline{pts=TOP, open=true, char="soft",
lobe=14}, foot=516, depth=4, count=13}`. Paint rows far to near. Row r's needles
are `mix(dark, air, 0.62*wood:haze(r))` with air a shade darker than the low sky
(`#a4a7a1`); 0.95 went milky. Paint trunks with a rigger (`wood:trees(r)`,
`f.leader.pts`) and the hatched strokes (`wood:paint(b, r, {...})`) on rows 1–2
only. Between rows 3 and 2, lay the floor: `wood:floor():roughen(7, 22, seed,
3)` in `#2a302d`→`#555c5a` down to the foot, `hug=false`. Use `wood:haze(r)`,
not `wood.rows[r].haze`, inside color functions: `wood.rows` builds every tree
(14 s).

**Spruces by hand (the older recipe; `tree{habit="spruce"}` gives a forest spruce with a bare trunk).**
`tree{habit="spruce"}` is a tall forest tree; `years=14` gave a lollipop [r4 near]. What
worked is a spruce hand of your own: an axis, then per tier a left and a right branch that
droops and lifts at the tip, longer toward the foot, plus a shorter foreshortened branch
toward you about 70% of the time. The numbers from [r4 near] chunk 7:
- tier = height/22; branch length `L = halfw * (0.12 + 0.88*t^0.85)` with a ±15% jitter;
  droop `(0.18 + 0.2*t)*L`; tip lift `0.08*L`; ribbon width `tier*(0.42 + 0.3*t)`;
- the mask is the union of ribbons, `:roughen(max(0.8, tier*0.12), max(3, tier*0.4), seed,
  0.5)`;
- fill it with `hand="hatch"`, `round 2`, lengths 3 to 9, coverage 3.2, near-black green
  (`#1f2824` to `#2c3630`), angle along the branches (on the young spruce: 2.75 left of
  the axis, 0.4 right of it);
- then draw every branch with a rigger (0.9 to 1.1), `pressure={0.8, 0.05}, ramps={0.05,
  0.6}`, so the tiers read against the sky.

Place wood-edge spruces with `uneven()`, tips on a drawn line, heights varied and
overlapping, over one dark interior mass. Even spacing reads as a row of Christmas trees
and an even crest as a clipped hedge. A transparent gray-green glaze over the interior
tones sky holes that read as bright paper at 3200 [r3 near]. Still weak: spires from one
function share a shape, and tiers come out as symmetric chevrons [r4 near]. Vary `halfw`,
`droop` and `lean` per tree.

**Oaks and broadleaves.**
- **Fuller crowns:** `t:foliage{sun=..., clump=0.04, spray=2}` gives a broad, dense,
  rounded crown in place of the clumpy savanna default. `clump` is a fraction of tree
  height, and 0.1 is enormous [r3 green]. `years` is hard to predict (80 to 400 gave 6 to
  10 limbs); leave it out or try a few values with `easel try` [r3 free, r3 green].
- **Preview seeds** with `easel try` and `show(t:mask())` [r4 green, r3 green].
- **Limbs and trunk first,** leaves over them, so the wood shows only in the holes [r3
  green].
- **Trunk and thick limbs as filled ribbons**, not single strokes (a long stroke from one
  load runs dry and fades out). For limbs with `l.w[1] >= 3`, build `ribbon(l.pts, l.w)`,
  then `work` in body color (round 2, lengths 4 to 12, coverage 3.5, `angle=1.5`,
  `clip=`). The lit flank is `thick * mask(function(x,y) return 1 - thick:at(x - 3.5, y +
  1.5) end)` in a gray `#8a8270` [r4 green]. Thin limbs as strokes by width: round 1.8
  above width 1, rigger 0.6 below, reload every 5 strokes or under 0.3 fullness.
- **Leaves in three hatch layers,** `hug=false` so edges thin out, angles from a noise
  (`2.6*turn(x,y)`, period 9, `angle_jitter=0.8`): a mid green over the whole crown (round
  1.8), then the shadow masses (round 1.6), then the lit masses (round 1.4) [r3 green].
- **Break the silhouette** with about 700 small hooked strokes along `crown:rim(5, 1)`:
  round 1.4, length 2.5 to 5, a hook of `randn(0, 0.8)`, `pressure={0.7, 0.05}`, light
  ones where lit and dark in shade [r3 green].
- **Sky back into the holes:** `work(leaves:gaps(6) * crown:grow(2) - wood:grow(2),
  {hand="detail", tool="round 1.4", color=cl})`. With the clouds object as the color, the
  holes match the sky behind [r3 green].
- **Dead snags above the leaves:** tapered ribbons (4.2 to 1.4 wide), dark `#4f4a42` with
  a silver lit flank `#a29b8d`, `medium=0.1` [r3 green].
- **Roots:** `tree` root limbs flare as straight spikes. Bury the foot in grass or snow.
  For snow, clip the whole tree to a wavy "above the snow line" mask, which reads better
  than painting snow over a stub [r3 free, r2 winter].
- **What fails:** sparse leaves by `stipple` read as pollen. Use a few dabs on a random
  quarter of `foliage().clumps` [r3 near]. At 3200, hatch dabs inside the crown still read
  as stippling. Bigger, hooked leaf strokes are the open problem [r3 green].
- **Birch:** twigs first, then the white trunk, a gray shadow side, black marks. Place the
  black patches irregularly with knots where limbs leave; an even ladder of lenticels
  reads as dashes [r3 near].

- **Field trees as groups** (beat "identical balls"): 1–5 trees per group, each crown
  drawn separately, ~30% tall and narrow, one faint uneven shadow per group, muted
  olive fading with distance. [loop 1 green]

- **Opening a black wood at its foot** (loop 2 near): a low horizontal band of dim,
  shaded floor (`#323b38`→`#6b706f`) at the wood's foot, near-black trunks (round 3.2)
  with low boughs (round 1.4) over it, and a thin veil in vertical columns higher up
  for air between the back trees. *Pitfall:* pale vertical patches read as ghosts.
  Paint trunks with pressure rising toward the TOP, or they end in points.
- **Breaking a wood-edge comb** (loop 2 near): treetops `randn(0, 30)` around the drawn
  line (not ±12), one tree ~55 units above the rest, a few young ones 25–60 lower,
  crown width 0.18–0.32 of height, some twin spires. *Pitfall (critics):* symmetric
  stacked tiers with a white edge on every tier read as a Christmas card; Friedrich's
  firs are short hatched strokes. [loop 2 near, loop 2 critics]
- **Replacing a hand-made wood with `fir_wood{}` in an existing log** (loop 5 near, +1):
  reuse the old skyline, give it a wavy foot line and keep the old mask's name
  (`woodm = wood:mask()`) so later chunks still run; then drop every chunk that patched
  the old interior. Snow at a wood's foot without a ledge: the foot outline's
  `below(H)` with a noise-varied fade depth (`392 + 26*n`, ±18), colored with the wood's
  cool cast shade mixed into the sampled field; snow on the wood only on sun-side
  strokes of the front row: `wood:paint(b, 1, {kind="top", lit={0.62, 1}, every=5})`.
  [loop 5 near]

## 6. Rocks and stones

*Ceiling:* stones read as loaves, eggs or stacked masonry; snow on stone reads as blotches or lichen.

**Rocks from a drawing: `rock{}` (use this before building a rock from ellipsoids and cuts).**
Draw the silhouette as `outline{pts=..., char="broken"}` ("firm" for a sure contour).
Mark corners with `"c"`. Draw the main cracks as point lists. Then
`r = rock{outline=o, cracks={...}, kind="granite"|"sandstone"|"chalk", sun=..., seed=}`.
It infers the planes, lights them and gives masks and fields [rocks_in study,
`paintings/lua/rocks_in.lua`, `paint_rock` in chunk 2]:
1. Take the color from `r:value()` through a five-stop gradient anchored on
   `r:levels(0.03, 0.97)`. Mix in `P.bounce` by `r:reflected()` and `P.core` by
   `r:core()`.
2. Lay the whole mass (filbert 4, coverage 3.2, medium 0.25) and then `r:lit(0.2)`
   stiffer (medium 0.12). Both go `angle=r:field("plane")`, so the strokes change
   direction at every plane break.
3. `blend(r:shadow(0.3), ...)` only. Blending the whole rock gave back the pale
   smooth loaf.
4. Stroke every line in `r.seams` once with a round brush (`pressure={0.8, 0.2}`,
   `clip=r:mask()`). Short `detail` strokes over `r:cracks()` came out as dotted
   lines, and the sandstone read as a crate.
5. Glaze `r:cast()` (0.3 coats) and `r:contact()` (0.35), each times a mask of
   your panel or ground, because the cast shadow reaches far under a low sun.

What decided the look:
- **Draw the outline lopsided.** My first erratic, drawn as a symmetric dome with
  a crack down the middle, was a loaf with a seam. A high shoulder, a broken back
  and a crack running off at an angle made it a stone.
- **Draw a sandstone ledge with a flat top and a stepped flank.** A domed outline
  gives a bedded loaf.
- **Under a low sun over snow, give the light a strong bounce from below.** Use
  `sun={from={-1, -0.14}, front=0.3, bounce=0.55, bounce_from={0.3, 1, 0.45}}`.
  With the default bounce the shadow side was the critics' "flat purple half".
- **Snow:** `r:snow{amount=0.7, depth=3}:blur(2):map(...)`, painted in masstone.
  It still shows dark flecks at 1000 px (unsolved).

- **An ellipsoid alone is an egg or a loaf.** What reads as stone: one mass, turned,
  roughened at two scales, cut by two or three fracture planes, e.g.
  ```lua
  body.ellipsoid(c, r):turn(c, 0.35, 0.08, -0.07)
    :rough(0.08*R, 0.9*R, 3)                 -- R = the radius
    :cut(top, {-0.25, -1, 0.35}, 10, 0.05*R):cut(side, {0.9, -0.35, 0.3}, 11, 0.04*R)
    :rough(0.017*R, 0.17*R, 5, true)          -- fine pitted grain
  ```
  [r4 near]. The first round-3 rough (`0.1*r, 1.4*r`) gave eggs, and `rough(1.2, 14, s,
  true)` gave cork. `rough(0.16*r, 0.7*r)` plus a finer one worked [r3 free]. A rounded
  block fused with a flat ellipsoid on top reads as an erratic [r2 coast]. Big `round`
  values on blocks give pillows, and stacked hard blocks give masonry [r3 near].
- **`size` and `s:size` take radii.** Two painters' first boulders were twice the size
  they meant [r4 near, r3 green].
- **Paint from the form:** body color from `f:value` through a four- or five-stop granite
  gradient, OKLab mottling with `shift`, strokes along `f:field("fall")` in shadow and
  `f:field("across")` on the lit face (filbert 4, lengths 6 to 18, coverage 3.6, `clip=`
  the stone) [r4 near, r4 green]. Then `blend` so the wet paint fuses into a hard surface.
  Without the blend, filbert strokes at 3200 looked like a bristly haystack [r3 green].
- **Fissures:** `f:edges{turn=0.7, step=3, span=2.5, concave=true} * stone:shrink(3)`,
  detail round 1.4 along `f:field("edge")`, plus two or three drawn fractures with a round
  1.6 and `shake=1.2` [r4 near]. A fine default span makes pumice. Keep edges to strong
  turns over a wider span [r2 coast]. Pure-black fissure specks read as pepper [r4 near],
  so use a dark gray `#2e2c2f`, not black.
- **Lichen:** a `worley` (period 9) mask gated by a slow noise, stippled `width=1.4`,
  `coverage=1.1`, `aim=false`, `fade=0.5`, pale gray-green and ochre [r4 near]. Keep it on
  the sky-facing lit planes.
- **Weathering** turned camouflage mottle into old stone: dark streaks glazed down from
  each ledge with a stretched noise, lichen rosettes clustered by worley, pits in the
  undercut [r3 near].
- **A straight base line** makes a rock architecture. Sink the foot in grass, bracken or
  drifted snow; fallen blocks from polygons read as sugar cubes [r3 near].

## 7. Snow

*Ceiling:* snow fields are flat and empty; lit snow under a low sun came out lilac until overruled by hand.

- **Snow on the ground from the view's light, remapped.** At an 11° sun, lit flat snow is
  only 0.37 to 0.49 in `v:at(x,y).shade.value`. Probe first, then `mix(blue_gray,
  warm_white, smoothstep(0.3, 0.5, lit))`, graying with distance. Use only `p.what ==
  "ground"`. The first pass came out lilac everywhere [r4 near].
- **Snow on the upper faces of a shape** (boughs, ledges): `mask(function(x, y) return
  m:at(x, y) * (1 - m:at(x, y - 2.5)) end)`, gated by noise, then `stipple(tops,
  {width=1.5, coverage=2.2, pressure={0.4, 0.8}, drag={1, 0}, aim=false, medium=0.15,
  fade=0})`, heaviest and warmest on the sun side. `dry()` first [r4 near]. Strokes there
  made a chevron "rain" pattern. On limbs, drop snow runs shorter than three widths, or
  they bead [r2 winter].
- **Snow cap on a stone:** `f:mask(function(s) return smoothstep(-0.55, -0.8, s.n[2])
  end)` (normal facing up), broken by noise, `:roughen(2, 8, seed, 0.6)`, filbert 3 in
  body, color from `f:value` [r4 near].
- **Snow drifted against a foot:** a soft open `outline` along the foot, its path shifted
  down and turned into a `ribbon` of varying width, `:roughen(2.5, 10, seed, 1.5)`. Color
  it from the snow just below (`sample(x, y + 26, 4)`, lightened a touch), then `blend` it
  [r4 near].
- **Cast shadows on snow:** `glaze(vs:cast_shadow{soft=2.2}, {color="#6a7090",
  coats=0.32})`. At 0.5 coats they were too heavy [r4 near]. Filbert strokes on snow look
  like boards [r2 winter]. A low sun gives a ruler-straight band, so break it with grass
  laid over it.

- **Snow banked against a stone** (beat loop 1's drift; loop 2 near +3). Shape ONE continuous
  bank with a hand-drawn envelope where the wind would pile it (sums of gaussians, e.g.
  `4 + 13*G(222,48) + 6*G(430,60) + 5*G(590,16)`, minus where the lee shades it), plus small
  noise at periods ~95, 19, 6; deep on the windward side, almost gone in the shade. Color
  it from the snow field itself (`sample(x, base+12, 3)`), with a slightly darker, cooler
  line where it tucks under the rock. *Pitfall:* noise clipped at zero (`3 + 26*noise`)
  gives evenly spaced puffs ("cotton balls"). *Ceiling:* the lip is still fairly even
  along the front; let the rock meet the snow directly in places. [loop 2 near]
- **Contact shade runs over the snow at the stone's foot, not around it.** Cutting the
  drift out of `contact_shadow` leaves a lit gap above a floating dark stripe; instead
  `v:contact_shadow{reach=0.14}:blur(1.5) * -stone` at ~0.22 coats. [loop 2 near]
- *Pitfall (critics):* a long shadow band across the foreground needs a visible caster
  and must agree with the other shadows; an unexplained band "contradicts the boulder's
  own shadow". [loop 2 critics]
- *Pitfall:* `roughen()` on a soft cast-shadow mask turns its soft edge into hard
  blocks: vary a soft mask by multiplying, not roughening. Noise stretched along a long
  ground shadow makes vertical curtains: stretch near horizontal (`stretch={0.02, 4}`).
  Thin filbert paint (medium 0.4–0.45) over a rough stone lets the dark grain show
  through as white lace. [loop 1 near]

## 8. Grass and foreground particulars

*Ceiling:* foregrounds are the dullest passages in every round (a barcode of upright dashes, a smooth band with sparse incident); Friedrich's are dark but drawn, every tuft particular.

The foreground is where every round fell shortest of Friedrich. Budget real time for it.

- **`sward` defaults cover everything** (69,000 tufts, a uniform lawn) [r3 near]. Give it
  a noise patch mask as `region`, `spacing=2.6`, `thin=0.5`. Then tint each tuft from
  what's under it: `g:reload(mix(sample(t.x, t.y, 2), straw, 0.12 + 0.35*rand()*rand()),
  0.7)` [r3 near]. For winter stubble, cull tufts further with a second noise [r4 near].
- **A sward starts on a visible line.** Add a sparse second sward (`thin=0.75`) in front
  of it, or roughen the region [r3 green].
- **The single most effective foreground chunk** [r4 green]: about 1,400 long curving
  rigger (0.8) blades through the dark foreground band, height `(8 + 34*depth) * rand(0.5,
  1.6)`, a lean from a slow noise plus `randn(0, 0.22)`, a curl of `randn(0, 0.25)`, six
  colors alternating sunlit tips and dark stalks, reloaded every 7 blades, `pressure={0.35
  + 0.5*depth, 0}, ramps={0.05, 0.75}`. It turned a barcode of short uprights into a
  meadow.
- **Set each blade down lightly at the root and lift it off** (`pressure= {p, 0}`, a long
  release). Pressed at once, blades leave a row of dark beads [r2 mountains].
- **Particulars need size and value to show at 1000 px.** A 1 px thistle over 1 px grass
  of the same value is invisible [r3 green]. Flowers need pressure 0.6 to 1.0 and a 2.4
  brush [r3 green]. Transparent mixes (Prussian blue, green earth) vanish on green; push
  the thistle toward near-black (`#2f3a24`) [r4 green].
- **Seed heads:** a rigger stalk, then 6 or 7 small `touch`es down from the tip with
  `drag={0.2, 1.4}`, in pale straw against the dark band [r4 green].
- **Bracken:** an arching rigger rachis and paired round-brush pinnae that shorten toward
  the tip, in muted rusts. The first fronds were too small and bright ("orange fern
  icons"). Lay dark masses under them, then lighter lit fronds over every group, or they
  read as flat blobs at 3200 [r3 near, r4 near].
- **Too even, too dense, too bright.** Nearly every first try was all three (lawn, hedge,
  orange ferns, green moss outline, polka-dot flowers) [r3 near, r3 green]. Cluster
  particulars with a noise patch field, scale them with depth and make the first try
  sparser and duller than you think.

## 9. Figures

*Ceiling:* figures are stiff, symmetric silhouettes, sometimes cut-out looking at 3200.

- **Size figures to the motifs around them.** A world-correct 1.75 m man near the viewer
  was taller than the stones [r3 free].
- **`body_of` for a figure from behind** worked [r4 green]: a four-point spine (head to
  hem) with widths about `{4.2, 7.5, 7.2, 9.5}*k`, two legs, two arms, a hat as a short
  wide limb, `blend=0.5`, `char="firm"`. Fill it dark (`#2e3136` to `#23211f`) with round
  1 in short strokes, then a lit edge from `fig:band(1.2, 0.5)` on the sun side only.
- **Judge figures at 3200:** `look --crop ... --scale 3.2`. At 1000 px a 50-unit figure is
  50 px, and square shoulders, a lost cap or a pale coat don't show [r3 free, r4 green].
- **Every later pass must leave the figure alone.** A sea veil, its blend and new glints
  painted over a figure because the mask never excluded him [r3 free]. Register him as a
  layer (`w:layer("figure", mask, spot)`) and pass `behind={"figure"}`, or subtract
  `figure:grow(1.5)` from every later mask.
- **Birds:** small V-strokes, each wing a rigger flick pulled out from the body and lifted
  at the tip. Heavier brushes gave hearts and beans [r2 winter, r2 mountains].

## 10. Drawing

- **Pencil grades on the Friedrich ground.** 2H is invisible at 1000 px on the mid-toned
  warm ground, even in a 2x crop; `drawing_mask():area()` shows it's there [r4 near, r4
  green]. 2B is faint. A 3B firm pass (`pressure={0.6, 0.75, 0.7, 0.55}`) reads [r4 near].
  Draw the search in 2H anyway (it's the right method, and it shows through thin paint at
  3200), then restate what you're sure of in 3B. A smaller canvas (`canvas{size=...}` in
  mm) makes pencil lines read larger.
- **Read the points off the picture:** `look --grid` (and `--grid 10` on a crop) for
  coordinates. Keep the point lists in globals (`BOULDER`, `WOODTOP`, `BROW`) so the
  pencil, the masks and the brush use the same line [r4 near, r4 green].
- **Paint into your drawing:** `drawing_guide():band(0.7, 1, 0.1)` is the drawn line as a
  continuous mask (not the grainy `drawing_mask`).
- **`show()` is the drawing you actually see.** Overlay point lists, masks and brush bands
  (`show(pts, {width=8})`) before you paint. Round 4 green drew mostly with `show()`
  because the pencil was too faint.
- **`easel try` for everything exploratory** (seed searches, `v:at` and `w:spot`
  printouts, previews). It rolls itself back; round 3's probe chunks had to be undone by
  hand.
- **`outline{}` characters:** `soft` for a far wood line (`lobe=9` to `11`), a hill brow
  (`lobe=5, amount=0.6`) or drift edges (`lobe=6` to `12`, `amount=1.2` to `1.4`);
  `broken` with `"c"` corners for a snapped stump or rock; `firm` for figures [r4 near, r4
  green]. An open outline's `:below(H)` is the land under a line. Its lobes point up.

## 11. Masks

- **`work` strokes overshoot the mask by design.** `clip=true` stops every bristle on the
  mask's edge [r3 free]; `edge=` (below) carries the passage to it the way a brush does. Use
  `hug=false` where an edge should thin out (foliage, drifts, veils).
- **`clip=` is true or false.** A mask there (`clip=m:grow(3):blur(2)`) has always been read
  as `true`: the strokes clip to the pass's own region, hard, and the grown, softened mask
  is never used. The reply now says so. For a softer or looser edge use `edge=` [r7 edges].
- **Never put a `rect()` inside a painted area.** It prints ruled edges, even softened.
  Snow banks, drifts and the stump's snow all showed straight sides [r4 near]. Build fades
  from `mask(function(x, y) return smoothstep(a, b, x) * smoothstep(c, d, y) end)` and add
  `hug=false`.
- **Rings make outlines.** `(m:grow(n) - m):soften(k)` for a contact shadow reads as a
  cartoon outline [r3 near], and a ring glaze to kill a halo doubled the shadow into a
  dark outline [r4 near]. Limit the falloff to the base side (y below the foot line) or
  use `v:contact_shadow{}`.
- **Hard masks read as digital.** Blur or roughen them before a glaze and lower the coats.
  A cloud shadow on a plain came out as ponds, then strips; what worked was
  `glaze(cs:blur(18), {coats=0.16, pigment="transparent"})` [r4 green, r3 green].
- **Subtract what's in front, or use depth.** Every round lost time to a pass that painted
  over an earlier motif (glints over stones and a barrow, a veil over a figure, a range
  painted under a knoll) [r3 free, r2 mountains]. With a world, `behind=`, `visible=` and
  layers do this for you.
- **`blend` smears everything inside its region,** small details included (bracken turned
  into orange streaks across a stone's foot) [r4 near]. Blend only the passage you just
  laid, clipped to it.
- **`f:part(x, y)` returns 0 off the form, not nil.** A test for nil painted nothing,
  silently [r3 green].

### Edges: found, soft and lost [r7 edges]

**Decide every edge; vary it along one contour.** A stencil (`clip=true`) makes the edge no
brush makes: one crisp, even line the length of a ridge, the same for every stroke. That is
what Alice and both critics called "a filled selection" in Evening at a Mountain Lake
(measured: the ridge's edge 0.38 units wide all along, 100% crisp, contrast varying 3%,
wandering 0.1 unit off a perfect curve; `notes/edges.md`). A painter makes an edge
**found** where a form turns against the light (the peak against the glow, a figure's
lit shoulder), **soft** along most of a contour and **lost** where a shape runs into its
neighbor at a close value (a ridge running down into haze, a reflection's lower edge in
the water).

- `work(m, {..., edge=...})` instead of `clip=`: each stroke carries its paint up to the
  region's edge and a little over it, by its own amount, and past the line the brush lifts
  and its film thins out. `edge="soft"` for one quality; a function of `(x, y)` (0 found ..
  1 lost) to decide it by the light; `edge={found=0.3, soft=0.5, lost=0.2, period=50}` for
  stretches along the contour.
  ```lua
  ridgeQ = function(x, y)                       -- found against the glow, lost low in the haze
    local glow = G(x, SUNX, 150)
    local low = smoothstep(HZ - 60, HZ - 10, crest(x))
    return clamp(0.5 - 0.5*glow + 0.5*low + 0.25*sn(x, y), 0, 1)
  end
  work(rangeM, {hand="body", tool="filbert 6", color=rangecol, ..., edge=ridgeQ})
  ```
- `lose(m, {where=..., angle=0})` after the passage is **dry**: short strokes of the
  neighbor's color (dirtied a little with the region's) dragged back across the edge where
  `where` says. Level strokes (`angle=0`) break a reflection's lower edge into the water.
- Lay the dead color a few units **inside** the line (`m:shrink(3)`) so the last pass
  decides the edge; a stencilled dead color shows through a soft edge as a hard one.
- Soft edges soften most into **wet** neighbors (the overrun picks up and mixes); over dry
  paint they are a thinning film and a broken fringe.

*Goes wrong:* `lose` into **wet** paint with its default lean brush lifts the wet film and
shows what is under it (pale "teeth" along a ridge in the lab); lose over dry paint, or
load more. On a ridge that `edge=` already carried outward, `lose` works along the mask's
line, now inside the dark, and leaves gray patches there: use it on edges a pass stencilled
or held found. Large overruns only move the edge out as a staircase of opaque stroke ends;
the fence keeps them short on purpose.

*Ceiling:* the lab ridge (`notes/edges/alice_edges.png`) is brushed and varied but still
reads mostly crisp over dry sky (86% of its length found at 3200, from 100%): a truly soft
dark-on-light edge over dry paint needs a blend of the two in the wet or a scumble,
which `lose` does only roughly. Not yet tried on firs, the figure or grass, or on the
wet engine (`r7-paint-wet`).

## 12. Color

*Ceiling* (piles): the sky and lake from piles (`notes/piles/`) read as
stepped paint rather than a formula, but the steps are still laid by the
formula's strokes, and a pile over a whole cloud body goes flat. Nobody has
yet painted a sky from piles from the start (short strokes laid into each
other at each step, a wet blend per step).

- **Mix piles, don't paint a formula.** A `color=function(x, y)` gives
  every stroke the formula's exact color; a gradient plus a Gaussian glow
  comes out "a little too perfect" (Alice, Round 7). Decide your piles
  the way a painter does: name the handful of mixtures you'd knife on the
  palette (`colors=`), and use a field only as a map of where each goes.
  Make the transitions on the canvas (`blend`, wet into wet).
  ```lua
  -- the piles you'd mix for this sky, dusk blue down to the warm band
  SKYP = piles(where, {colors={"#47536c", "#626b82", "#8b8d9c", "#a8999c", "#c9b49c", "#e3cb9a"},
    over=skyS, pal=skypal, medium=0.3, coverage=3.8, load=0.75, seed=601})
  print(SKYP)                                   -- read the recipes: are they paint you'd mix?
  work(skyS, {hand="broad", color=SKYP, coverage=3.8, medium=0.3, load=0.75, pal=skypal})
  blend(skyS, {angle=0, coverage=1.2, length={150, 400}})
  ```
  (`where` is any rough color map saying which pile belongs where; the
  pile nearest to it wins, with ragged, wandering boundaries.) Letting
  `piles` choose the piles for you from a formula (`n=7` without
  `colors=`) works, but it's the formula in disguise: the machine decides
  the mixtures. Prefer naming them (notes/principles.md).
  5–7 piles for a sky, 3–4 for a cloud bank or its lights, 5–6 for water.
  Call `piles` after the underpainting it covers (the piles are mixed
  looking at the canvas) and give it the pass's `coverage` and `load` (it
  aims them at the thickness that pass lays). Give each `piles` a `seed`:
  without one it takes the painting's next auto seed and shifts every later
  one. Reuse the same pile set for touch-ups. [r7 piles]
  *Pitfall:* few piles over a small shape paint it flat (the bank's body in
  4 piles went one taupe); a straight broad stroke's square end shows more
  against a neighboring pile: blend the steps, or shorten the strokes.
- **Aim at what's there.** `aim="laid"` (default) aims each stroke at the look over what's
  under it. After a glaze, old colors are wrong: grass loaded with the pre-glaze colors
  stood out pale. Sample the glazed ground and mix from it [r3 free].
- **Relative color for shadows:** `color_over={shift={-0.06, -0.004, -0.017}}` (darker, a
  little bluer) is the ground's own shadow, stroked the way the ground goes [r3 green, r4
  green]. Mixing shadows from your own color field instead made them *lighter* than the
  real, darker paint [r2 winter].
- **Transparent darks for shadows and glazes:** raw umber and bone black,
  `pigment="transparent"`. Thin darks with lead white in them go milky and bluish over a
  darker field [r2 coast].
- **Earth-only palettes for small motifs** (`pal:only{"red earth", "raw umber", "bone
  black"}`, or lead white, smalt, ochre and umber for sails and stone). Full-palette mixes
  show their strongest tube as colored rims where a mark thins [r2 coast], and weathering
  came out rusty [r2 winter].
- **A color field applies per stroke,** sampled every 2 units. Fine structure (drift
  shading) blurs into blotches unless the field is smooth or stretched along the strokes
  [r2 winter].
- **Greens:** meadows came out too uniformly yellow-green. Friedrich's are cooler and
  varied, with blue-green darks. Mix the shadow greens toward the sky's blues [r3 green].
- **`hand="scumble"` is a dry brush.** It lays opaque blobs or a lattice of broken
  strokes, not a veil [r3 near, r4 near]. For "a veil over what's there," use
  `hand="body"` with `color_over=function(x, y, under) return mix(under, c, 0.55) end` and
  then `blend`. Scumble works for lit cloud tops and a stone's lit top (coverage 2, medium
  0.35) [r4 green].
- **A brush glaze with `color_over` is still laid everywhere.** Returning `under` doesn't
  skip the stroke; it drags and flattens texture beneath [r3 green]. Restrict the mask
  instead.
- **Narrow body passes build impasto.** Two body passes on a 20-unit trunk reached 1.5 mm
  of paint. Friedrich's films are thin. Lower `load` and `coverage` on small areas [r4
  green].

- **Veil that takes color out without flattening texture** (loop 1 green +2, light 4→5).
  Thin glaze strokes, fenced to the area, pulling what's under them 40% toward its own
  gray plus ~12% `#7b7a5a`; for a big lit lawn one glaze `#6d6a4c` at 0.32 coats.
  *Pitfall:* the same veil in heavy long strokes erased the plain and spilled onto
  motifs: keep veils thin, short-stroked, and cut every motif out. [loop 1 green]

## 13. Pitfalls that cost the most time

| Pitfall | Fix |
|---|---|
| Paint laid over paint that's still open (translucent rock, fog band, plowed river) | `dry()` first, or check `drying(x, y)` [r3 near, r4 near, r4 green] |
| A later pass paints over an earlier motif | Depth layers and `behind=`, or subtract the motif (grown) from every later mask; crop the motif after every big pass [r3 free] |
| `clip=<mask>` taken for a softer fence (it clips hard to the region itself) | `clip=true` or `edge=` [r7 edges] |
| `rect()` fades and ring masks leave ruled edges and outlines | `smoothstep` fades in `mask(fn)`, `hug=false`, `v:contact_shadow` [r4 near, r3 near] |
| Fixing an early chunk by hand-editing the log (close, sed, 30 to 100 s replay; `close` once overwrote the edit) | `easel edit N -f`, `edit N --insert`, `edit N --drop` in the live session [r3 green, r3 free] |
| Probe chunks cluttering the log | `easel try` [r3 free] |
| Body radii mistaken for full sizes | `size` and `s:size` are radii [r4 near, r3 green] |
| `eye=` is absolute, not above the ground: the camera sat inside a knoll | Make the ground about 0 at the camera [r3 free] |
| Judging detail at 1000 px (figure ruined under a veil, grain in a stipple) | `look --crop ... --scale 3.2` on the motif after each pass that touches it [r3 free] |
| A generator's defaults (sward, spruce, oak crown) taken as the answer | Treat each as a first try: thin, cull, vary, restate by hand |
| A chunk that silently painted nothing | Print an area or count at the end of each chunk (`print(m:area())`, blade counts) [r3 green] |
| The live 3200 crop timing out under load | Retry later, or `easel run --crop` in the background [r4 near] |
| Scratch study sessions write `paintings/lua/<name>.lua` | Move study logs to your scratch dir afterward [r3 green] |

- **Never cut motifs out of a veil or glaze with a grown mask** (`-(fig:grow(1.5))`): the
  ring between the grown mask and the painted motif keeps the old paint and reads as a
  glowing halo (loop 2 green: paint score 2 of 10). Use the world's depth instead:
  `glaze(area, {..., behind={"figure", "bodies"}})` or `visible="ground"` (register
  hand-painted motifs with `w:layer(name, mask, depth)`). If you must cut, cut with the
  motif's exact mask, not a grown one. [loop 2 green, loop 3 critics]
- **Hundreds of short broken strokes along an edge read as embossed scribbles** at 3200
  (loop 2 green's ~900 crown-edge strokes and grass loops); fewer, drawn, varied marks
  beat many generated ones. [loop 3 critics]
- **Check a proxy's size against the painted motif**: proxy/body sizes are radii
  (`s:size` takes half-sizes), so a proxy written at full size casts a shadow twice as
  long and wide; check with `w:height(x, y, 1)`. [loop 3 near]
- **A small caster's shadow at a low sun, drawn by hand**, when `cast_shadow` starts too
  far from a small proxy: `w:project` the sun direction from the foot, glaze a mask with
  half-width `6.5 + 7t`, edge `1 + 9t`, fade `1 - smoothstep(0.08, 0.62, t)` along it.
  An ellipsoid boulder shades the ground under its own lit belly: subtract below its foot
  line there. [loop 3 near, +1]

- **Removing halos from an existing log** (loop 3 green +2): replace every `- X:grow(n)`
  in veil and glaze masks with `- X` (the exact motif mask); blur a veil's mask first and
  cut the motifs after the blur, never before. `oak:mask()` is larger than the painted
  wood: cut with the painted limbs' own mask. Close a remaining fringe with a small-brush
  repaint just outside the motif (round 1.6, lengths 2–6, coverage 3, medium 0.25,
  `clip=-M`), each stroke's color sampled ~11 units farther out. Check halos at 3200: they
  can be clean at 1000. [loop 3 green]
- *Pitfall:* a brush veil with `color_over` over a detailed area
  blurs the detail away. [loop 3 green]
- *Tried and rejected (loop 4 near, 0):* a distance-graded veil + wet blend over a black
  wood gave air but smeared it into vertical streaks and lost the trunks. Use `fir_wood{}`
  instead (section 5).

- **Glitches at 3200 that are the recipe's, not the engine's** (Round 6 glitch census,
  `notes/glitch.md`, sheet `notes/glitch/alice_census.png`). Principle: whatever you
  leave uncovered over a strong underlayer shows at full size as sharp, bright,
  digital-looking specks and slivers, even when the 1000 px preview reads as a mass.
  Check at `look --crop ... --scale 3.2` and run `uv run scripts/glitch.py` on a
  3200 crop. Ceiling: these fixes remove the slivers; they don't make the passage good.
  - *Thin long level strokes over the orange Friedrich ground* leave slivers of
    ground between them (lab 1 water B's open water and sky: the "orangey lines" the
    owner called horrible). Fix: a second pass of the same color at a different
    stroke length, or a `blend()` while open, or tone the ground there first; judge
    `coverage` at 3200, not 1000. [lab 1 water B]
  - *A dense small-brush hatch over a dry pale layer* (a dark crown hatched with
    `round 1.6` over a dry sky) leaves pinholes of sky between its ridges. The engine
    no longer leaves paint on a sliver (glitch P1), but the bare crevices between
    200 µm ridges of stiff paint stay open. Fix: lay the crown's darkest mass as one
    body layer first (dead coloring), then hatch into it; or hatch with more medium.
    [lab 2 lime P]
  - *A crack left as a masked channel* (the lit pass clipped off `rk:cracks()`) reads as
    an even, ruled line of the shadow paint. Fix: paint the crack as a stroke with a
    pointed round, varying pressure, a light lip on the sunny side, lost halfway (lab 2
    rock H does this). [lab 1 rock B]
  - *Thin snow dragged over a dry, ridged rock* catches in the hollows and leaves the
    ridges' crests bare: at 1000 it reads as snow torn by the grain (as meant), at 3200
    as dark outlines round the rock's old strokes. Fix: lay the cap fuller (`load` 0.9,
    less medium) or before the rock dries, or accept it only where it reads as grain.
    [l5_near]

## 14. Habits of looking

- **Look after every chunk** (`easel do --look`). A third of round 3's chunks were undone;
  that's normal [r3 free].
- **Value and squint at each stage change:** `look --mode value,squint`. It showed a rock,
  a sky and a middle floor all at one value. The fix was a painter's fix: darken
  everything around the rock with transparent glazes toward the bottom and sides [r3
  near]. Do it after the lay-in, after the motifs and before the particulars.
- **`--dried`** before judging a wet passage's color. Wet paint levels and shifts as it
  dries.
- **3200 crops for handling.** 1000 px is for composition and value only. Specks, beads,
  dotted edges, the sameness of repeated marks and figure drawing show only at 3200 [r2
  mountains, r3 free, r4 near]. Crop each motif at `--scale 3.2` after the pass that made
  it and after any later pass that crosses it.
- **`--probe`** for actual values when a value relation looks off. A "pale" rock that
  seemed an engine bug was simply lighter than the turf around it [r2 mountains].
- **A scratch study** for a hard motif: three versions side by side on a flat ground, then
  carried into the painting (the round 3 oak's crown) [r3 green].
- **Before the varnish, look for regularity.** Every place a round-4 picture looked
  digital was a place where a loop or mask went down faster than the painter looked:
  identical trees, barcode grass, ruled hedges, flat shadow bands [r4 green].

