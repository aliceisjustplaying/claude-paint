# Lab 2: the bare oak on the angular tool, and bare oaks at a distance

Two subjects, each with its own owner sheet. The letters on the sheets are
neutral and were drawn at random.

## Key (don't read this before you look at the sheets)

| sheet | A | B |
|---|---|---|
| `bare_sheet.jpg` | **C**: round 1's `baretree_C.lua` replayed as is on the current engine | **C2**: C's recipe on the new tool (`wood_strokes`) |
| `barerow_sheet.jpg` | **T**: near tree in lines; middle and far crowns as a dry-brushed haze with a few lines on top | **L**: every crown in selected drawn lines only |

Each sheet has the whole pictures (1000 px) on top and a 3200 crop of the same
window below. `bare`: canvas 640–940 × 180–400, the same window as round 1's crops.
`barerow`: canvas 540–910 × 300–485, the middle and far trees.
The renders are in `out/lab2/` (`*.png` at 1000 px, `*_full.png` at 3200 px). The
repo ignores `out/**/*.png`, as it did for round 1's lab, so they aren't committed.
`easel run notes/lab2/<log>.lua [--width 3200]` repaints each one.

## Subject 1: `bare` (C against C2)

Setup: `bare_setup.lua`, round 1's `baretree_setup.lua` byte for byte (one oak,
`tree_in{... seed=11, girth=0.06}`). On the current engine that setup grows the
angular oak: 5,252 limbs, 4,242 of them twigs. Logs: `bare_C.lua`, `bare_C2.lua`.

### C: round 1's log as is (13 chunks)
The same code as round 1, now running on the angular limbs: C's own loop picks and
strokes `oak.limbs`. It keeps everything down to 0.7 wide, samples the finer wood
with a clustered noise and drops the limbs that meander, then strokes each limb
through its nodes, lightly smoothed. It also paints 374 chosen twigs, a faint tier
of 427 more, the trunk turned into its own wet dark, the foot patch, the glaze
shadow and the grass band. Marks: 602 limb pieces, 374 twigs and 427 faint twigs,
about 1,400 wood strokes. Relief is 0.06 now, down from 0.2.

What I see: the angular growth shows through even though C smooths the thin wood and
the brush's spline rounds every node. The crown is more crooked than round 1's, with
flatter runs and kinks, and fewer S-curves. At 3200 the lines are still the most
delicate of the pair: varied weights, long tapering twigs, faint far wood.

### C2: C's recipe on the new tool (12 chunks)
Chunks 2–5 (sky, ground, crest), the trunk-turning chunk, the foot, the shadow, the
band and the varnish are C's code, copied unchanged. What changed is how the wood is
painted (chunks 6–7):
- **The strokes come from `oak:wood_strokes{min, max, detail=0.5}`**, not from
  `oak.limbs`. The tool subdivides each straight run, so the brush keeps the elbows.
  Each stroke starts inside the wood it leaves from, and a leading twig is drawn on
  in the same stroke. I stroke them by hand so I could keep C's recipe:
  - *Depth color* as in C: `REC = smoothstep(40, -260, mean z)`, eased from the
    parent by at most ±0.3, mixed `0.06 + 0.5*RE + 0.2*thin*(0.4 + 0.6*RE)` toward
    `sky(x, y)`. Twigs get another 18% toward the sky, and far wood is 25% thinner
    (`kk = 0.85*(1 - 0.25*RE)`).
  - *Taper*: pressure follows the model's width through `swell` knots (up to 24 per
    piece, as `paint_wood` does). The brush lifts to a point only at a real tip
    (tip pressure 0.2 / 0.06 / 0.04 by band, lift 0.12).
  - *Selection*: C's clustered noise (`period=140`, keep `0.25 + 0.7*smoothstep(-0.25,
    0.35, n)`) runs on top of the tool's `detail=0.5` choice. A dropped fine piece
    takes everything that grows from it along, so nothing floats. 831 of 2,006 fine
    strokes kept.
  - Bands: round 2.4 for 1.2–3.5, rigger 0.9 for 0.5–1.2, rigger 0.55 below.
  - Long strokes go in pieces of ≤ 60 units, each on its own load (0.85). **The next
    piece starts 3 units back.** With a one-point overlap, every join left a pale nick
    at 3200. The round 2.4 limbs are **laid twice, the second pass straight into the
    first's wet paint**, because the first pass skipped on the dry sky's tooth.
- The faint tier (C's chunk 9): planned fine strokes at `detail=0.8` that aren't in
  the main set, weighted to the rim and clustered as in C, at 0.65 width and 40%
  toward the sky. 201 strokes (C had 427: C2 skips any whose parent was dropped).
- Marks: 1,287 wood pieces, counting the second pass on the limbs, plus 201 faint
  ones.

What I see: at 1000 px **C2 reads more like an oak**. It has elbows, zigzag limbs
that thin at each fork, a crown that holds its own near the rim, and fewer rope
curves. It is also heavier and busier. The 0.5–1.2 wood comes out at nearly one
width (about 1.3 at 3200) and darker than C's, so the crown's interior reads as a
wire tangle. At 3200 the crop shows the tool's known faults: short stiff twig stubs
that look thorny, a few right-angle hooks near the top, and sibling limbs that run
side by side. C's crop is more delicate.

**Verdict.** B (C2) is better at the size the picture is seen, by a little, because
the angular growth is what an oak looks like. A (C) is better at 3200. **C2's
ceiling:** the middle wood is too even and too dark. It needs thinner pressure for
0.5–1.2 (about 0.7 of the model width, not 0.85), more depth lightening in that band,
and fewer fine strokes near the top (the hooks). The elbows and taper are right.

## Subject 2: `barerow` (the distance test)

Why: round 1 concluded "selection beats tone" from one foreground tree. Here are three
bare oaks in a row going away over a low winter field. Setup: `barerow_setup.lua`,
3:2, 300 mm, seed 62. **Near**: crown 396 tall, feet 170 below the horizon.
**Middle**: 155 tall, 66 below, haze 0.38. **Far**: 61 tall, 24 below, haze 0.66.
Each crown is drawn differently and lopsided. The setup also holds the shared
helpers: `depths(t)` (C's eased depth, scaled to the crown), `woodcolor` (C's color,
then toward `AIRAT(x, y)`, which is the air against the sky and a slightly darker
field below the horizon, so a far trunk doesn't glow pale against the ground) and
`lay` (C2's stroker). Chunks 2–7 are the same in both versions: C's sky, ground and
crest, then the near oak by C2's recipe (1,328 pieces: 694 fine kept, 164 faint) with
the trunk turned. Chunks 9–11 of L and 10–12 of T are also shared: the feet, the three
shadows (fainter with distance) and grass bands at the near and middle feet, all C's recipe scaled by trunk
width.

- **L** (chunk 8): the middle and far oaks in selected lines only. `detail` is 0.35
  for the middle and 0.25 for the far, thinned again by a clustered noise sized to the
  crown. Rigger 0.9 for 1.2–3.5, rigger 0.55 below, all pulled toward the air.
  Middle: 349 pieces (230 fine). Far: 211 (131 fine).
- **T** (chunks 8–9): chunk 8 lays the haze. It goes through
  `t:twig_mass(0.1):blur(height/30)`, smoothstep 0.05–0.5, as lean stiff paint
  (`hand="body"`, filbert 3 for the middle and filbert 1.4 for the far, lengths
  10–24 and 4–10, coverage 2, load 0.3, medium 0.05, `paint={0.5, 0.95}`, pressure
  0.45→0.15). The strokes run the way the wood runs there: a direction field
  splatted from the limb segments, bent outward from the fork (1.2 weight). Each
  pulls what's under it 50% of the way to `mix("#665d58", AIR, 0.35…0.5)` at the
  density. Chunk 9 adds **a few lines**: the trunk and every limb at least 1.4 (middle)
  or 0.6 (far) wide at its base, plus a clustered 20% of the thinner wood. Middle: 73
  pieces. Far: 51.

What I tried for the tone before this (logs undone, not kept):
- A scumble filbert at mix 0.35 gave opaque blotches.
- A fan brush at load 0.12 was invisible and paler than the sky.
- A brush-free `glaze()` through a streaked mask gave a starburst, then sponge
  patches, then a cotton cloud. **Engine note:** a glaze thinner than about 1 µm
  doesn't form (`formed_film`, `MIN_FILM_UM` in `crates/paint/src/canvas.rs`). So
  `coats × mask` under about 0.27 cuts off, and a soft mask turns into hard-edged
  patches. Vary a glaze by its color, not by thin coats.
- The dry-brush that worked has direction: strokes along the wood, not radial from
  one point (firework) and not a random scribble (steel wool).

What I see:
- At 1000 px, **T's middle tree reads as a distant bare crown**: a warm, transparent
  mass, thickest at the rim, with its structure drawn over it. L's middle tree reads
  as a smaller copy of the near wire tree. Its lines are too heavy for the distance,
  because a rigger can't lay a line finer than about 0.2 units. A distant tree's
  twigs are much finer than that, so every line drawn out there is a thick line.
- The far trees are close: both read as little trees. T's is softer and sits
  further back.
- At 3200 (the crops) L is clean drawing. T's haze shows as separate pale filbert
  dabs with faint paint rims, more like a mist of fine leaves than twigs. That is
  its ceiling.

**Verdict.** A (T) looks better to me at the size the picture is seen. The middle
tree carries air and mass without fur. Selection wins in the foreground and at 3200.
**Past the middle distance, a restrained directional tone under a few lines beats
lines alone**, as long as the tone stays a veil and not a patch.
**T's ceiling:** the dabs register one by one at 3200. Next try: a finer tool
(filbert 2) with softer paint (stiff about 0.6) and a lower mix per stroke at a
higher coverage, so it reads as grain and not as capsules. Or lay the haze into the
sky while it's still setting, so its edges melt.

## SKETCHBOOK CANDIDATE: select near, veil far

*Principle.* A bare crown's twigs are drawn only where a line can be as fine as
they are. In the foreground, draw the wood and a selection of twigs
(`wood_strokes`, depth color, taper, clustered windows). Past the middle distance a
brushed line is already thicker than any twig out there, so say the twig mass as a
restrained, transparent tone that runs the way the wood runs. Then draw only the
trunk and the carrying limbs over it.

```lua
-- far/middle crown t, haze h (0.4 middle .. 0.7 far); sky and ground dry
local tm = t:twig_mass(0.1):blur(t.height / 30)
local m = tm:map(function(v) return smoothstep(0.05, 0.5, v) end)
local dir = grainfield(t, t.height / 40)       -- doubled-angle sum of limb segments per cell, 5x5 weighted
local FX, FY = t.fork[1], t.fork[2]
work(m, {hand="body", tool="filbert 3" --[[1.4 far]], length={10, 24} --[[4..10 far]], coverage=2.0,
  load=0.3, medium=0.05, paint={0.5, 0.95}, pressure={0.45, 0.15}, ramps={0.1, 0.6}, hug=false,
  angle=function(x, y) local a, o = dir(x, y), math.atan(y - FY, x - FX)
    if math.cos(a - o) < 0 then a = a + math.pi end
    return math.atan(math.sin(a) + 1.2 * math.sin(o), math.cos(a) + 1.2 * math.cos(o)) end, angle_jitter=0.15,
  color_over=function(x, y, u) return mix(u, mix("#665d58", AIR, 0.35), 0.5 * m:at(x, y)) end})
-- then a few lines: trunk + limbs >= ~0.1x trunk width at their base + 20% of the rest, clustered,
-- colored mix(C's depth color, AIRAT(x, y), h): the air against the sky, a darker field below
```
*Pitfalls.* A thin `glaze()` varied by `coats` breaks into patches, since a film
under about 1 µm doesn't form. Radial strokes from the fork read as a firework. A
random direction reads as steel wool. Opaque scumbles read as blotches. A distant
limb pulled toward a pale air shows up pale against the field, so pull it toward
what's behind it. *Ceiling:* at 3200 the veil's dabs register one by one.
