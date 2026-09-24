# Edges (Round 7, stream "edges")

The top complaint on Evening at a Mountain Lake, from Alice and both blind
critics: boundaries. "The mountain and its reflection resemble a filled
selection"; firs repeat one crisp silhouette; the figure is "an icon";
every edge is equally crisp. This stream measured where those edges come
from and gave painters physical edge control: found, soft and lost, varying
along one contour.

## 1. Measurement

### The tool: `scripts/edges.py`

`uv run scripts/edges.py RENDER.png --box x0,y0,x1,y1 [--mode column|all] [--plot out.png]`
profiles the edges in a box of a render (pixels; OKLab L). `column` mode is
for a level edge (a ridge): per pixel column, the strongest vertical step,
its contrast and its width (10% to 90% of the step, in canvas units). It
reports width percentiles, the shares of the contour that are found (width
≤ 1.2 units), soft (1.2–4) and lost (contrast under a third of the median,
or wider), how much the width and contrast vary along the edge (CV) and
the waver (RMS of the edge's y about its own 8-unit running mean: 0 is a
ruler-perfect curve). `all` mode profiles every edge pixel across its
gradient (silhouettes). One unit is 0.44 mm on a 440 mm canvas.

### Evening at a Mountain Lake at 3200 (`notes/paint1/evening_lake_3200.png`)

| edge | width p10 / p50 / p90 (units) | found / soft / lost | contrast CV | waver |
|---|---|---|---|---|
| the ridge (x 719–919 units, 640 columns) | 0.25 / 0.38 / 0.49 | 1.00 / 0 / 0 | 0.027 | 0.10 units |
| the figure (all edges in its box) | 0.68 / 0.85 / 3.06 | 0.59 / 0.40 / – | – | – |
| the fir spires (all edges in their box) | 0.49 / 1.20 / 2.79 | 0.50 / 0.50 / – | – | – |

The ridge is one crisp step, 0.17 mm wide, of nearly constant contrast, on
a curve that wanders a tenth of a unit off its own smooth line. No brush
lays that; a selection filled with paint does. The profile plot is
`notes/edges/evening_lake_ridge_profile.png` (every column's edge tinted
green: found).

### Where it comes from: the stencil, and a trap in `clip=`

- **`clip=true` is a stencil.** In `bristle::exchange` every bristle's
  contact weight is multiplied by the clip mask at each pixel, and its
  deposit is renormalized over the pixels it still touches. So every
  bristle stops exactly on the mask's 0.5 level, and a bristle half over
  the line lays its whole share inside it. The edge is the mask's edge,
  the same for every stroke. The ridge in chunk 19 was restated with a
  clip that is a pure `smoothstep(crest2(x) - 0.7, crest2(x) + 0.7, y)`
  along a noise-plus-Gaussians function: the measured edge.
- **`clip=<mask>` never used the mask.** `work` and `stipple` read `clip`
  as `Option<bool>`, and mlua turns any non-nil, non-boolean value into
  `true` (mlua 0.12.1 `src/conversion.rs`, `impl FromLua for bool`: `_ =>
  Ok(true)`). So `clip=skyS:grow(2):blur(1.5)` clipped the sky pass hard to
  `skyS` itself. The grown, softened fence the painter wrote was never
  read. Evening at a Mountain Lake has 31 `clip=` options; about two thirds
  pass a grown, blurred or different mask (`skyM:grow(3)`,
  `rangeRefl:grow(2)`, `wood:grow(1.5)`, `(-landM)`...). The wood's hatch
  (chunk 11: "the edge is the strokes' own fringe", `hug=false`,
  `clip=wood:grow(1.5)`) was in fact stencilled to the wood's skyline.
  The same trap is in every benchmark log (`l5_near` 23 uses, `l3_green`
  17). Honoring the masks now would change every existing log, so the
  meaning stays (`clip=<mask>` = `clip=true`) and the reply now says so
  once per chunk: `work: clip= is true or false; the mask given was not
  used...`.
- **`hug`** (default on) moves stroke centers just outside the region onto
  its edge, so coverage doesn't thin there. Unclipped, strokes overshoot by
  up to half a brush. It governs coverage, not the edge's quality.
- **`cut_in=`** stops the body strokes short and runs short strokes of a
  second brush along the outline just inside (`cut_in_edges`): a hand
  drawing the edge, but also the same crisp line everywhere, found.
- **`m:blur(r)`** softens the mask, but under `clip=true` the renormalized
  deposit lays full paint wherever the mask is above zero near a bristle:
  it moves the stencil's line rather than softening the edge. Blurred masks
  only soften as a glaze's coats (`glaze(m:blur(..))`).

A lab study on main (§3) painted the same way measures the same: the
stencilled ridge is 100% found, width p50 0.50 units, waver 0.10.

## 2. What changed

### `edge=`: a fence instead of a stencil (`crates/paint/src/fence.rs`)

A `Fence` is one pass's edge: the region's signed distance to its edge in
units, moved by a slow waver (the painter's line isn't the mask's line:
`waver` × (0.3 + 0.1·W) × (1 + 2q) units, over 6–40 units), and a quality
field `q` (0 found, 0.5 soft, 1 lost). Each stroke draws its own overrun
`u` (0..1, hashed from where it starts and the seed, so crops paint the
same). At a pixel at signed distance `d` a bristle's contact is scaled by

```
o = W·reach·(a(q) + b(q)·u)     how far past the edge this stroke reaches
s = px + W·c(q)                 over how far it lifts off
f = smoothstep(-o - s, -o, d)
```

with `(a, b, c)` found `(0, 0.12, 0.06)`, soft `(0.02, 0.25, 0.8)`, lost
`(0.1, 0.4, 2.2)` brush widths, interpolated. Past the fence the hairs are
held off the weave's hollows (the contact threshold rises by `0.7·(1 - f)`:
a broken, dry fringe), and the bristle's deposit is scaled by its mean
`f²` over its contact, so the film thins to nothing: a lifting brush lays
less paint. Paint a bristle can't lay outside stays on it, as with a clip.

What didn't work, on the way (lab crops in the scratch notes, described
here): the first numbers (lost overrun up to 2.3 brush widths, fade 1.3)
moved the edge out as a **staircase of opaque stroke ends** (horizontal
strokes each ending at its own place past a slanted ridge), and soft edges
over dry sky still measured 93% found, because the renormalized deposit
laid a full film on the weave tops the lifted hairs still touched. Short
overruns, long fades and the `f²` thinning fixed both: over dry sky a
ridge painted all "soft" measures width p50 0.99 units, 31% soft (from
0.50 and 0%), all "lost" p50 1.63, 48% soft and 17% lost.

Shared files, kept small:
- `bristle.rs`: `Clip` enum (`Mask` or `Fence { fence, u, limit }`)
  through `drag_on`, `touch_on` and `exchange` (the public `drag` and
  `touch` still take `Option<&Mask>`); the mask path computes exactly as
  before; the fence path adds the lift and the thinning.
- `handling.rs`: `Handling::fence` and `.fence(Arc<Fence>)` (sets
  `clip`), the pass's clip becomes a `Clip`, `run_plans` draws each
  stroke's `u`.
- `stipple.rs`: one call wraps its mask clip.
- `easel/src/api.rs`: `edge=` parsing, the `clip=` notice, the pass seed
  drawn once (so `edge=` doesn't change which strokes a pass plans), a few
  helpers `pub(crate)`.

### `lose(region, {...})` (`crates/easel/src/draw_edges.rs`)

Loses an edge after the passage is laid: along the region's contour
(`edge::contours`), where `where` is high, short strokes start out in the
neighbor, cross at a shallow slant (or at `angle`, pointed inward) and lift
off inside, each loaded lightly with what lies in the neighbor, dirtied
with `mix` (0.35) of the region's color and aimed at the look where it
crosses. Brush strokes through `Canvas::drag`, with trips and hand time
like `b:stroke`.

What went wrong in the lab, in order: strokes square across the edge,
evenly spaced, read as a **comb** of pale teeth; slanted along the edge
they read as snow on a crest; the pale teeth were not paint at all but the
**wet** dark film lifted by the lean brush (the ridge's pass was minutes
old), showing the sky under it (probed: `sample` above and below the
line). Over dry paint, pressed (`pressure={0.75, 0.25}`) and lean
(`medium=0.6`), it lays a scumble. On the study's ridge, which `edge=` had
already carried outward, it worked along the mask's line, now inside the
dark, and left gray patches; it is used there only on the mirror's lower
edge, with level strokes, where it reads as the water carried across.

### The API a painter uses

```lua
work(rangeM, {hand="body", tool="filbert 6", color=rangecol, edge="soft"})
work(rangeM, {..., edge=function(x, y) return 1 - glow(x, y) end})           -- 0 found .. 1 lost
work(reflM,  {..., edge={found=0.2, soft=0.5, lost=0.3, period=50}})          -- stretches
work(figM,   {..., edge={quality=0.2, waver=0.5, reach=0.8}})
dry()
lose(reflM, {where=function(x, y) return y > HZ + 6 and 0.7 or 0 end, angle=0, pressure={0.75, 0.25}, medium=0.6})
```

Easel guide: "Edges: found, soft and lost" (`crates/easel/README.md`).
Sketchbook: §11 "Edges" with its ceiling, and the `clip=<mask>` pitfall.

## 3. The study

`paintings/lua/edges_old.lua` and `paintings/lua/edges_new.lua`: a ridge
and its mirror in still water against an evening glow, identical but for
chunks 3–4. The old way stencils the range and its mirror (`clip=true`).
The new way decides the ridge's edges by the light (found where it turns
against the glow, soft on the flanks, lost where it runs low into the
haze, plus a noise), gives the mirror's edges soft and lost stretches, and
once dry drags the water back across the mirror's lower edge with level
strokes (`lose`).

For Alice (lossless): `notes/edges/alice_edges.png` (the whole of each at
1000 over a 1:1 3200 crop of each), and the plain images
`edges_old_1000.png`, `edges_new_1000.png`, `edges_old_3200_crop.png`,
`edges_new_3200_crop.png` (units 170–650 × 250–560).

Measured at 3200 on the crop's ridge (1536 columns):

| | width p10 / p50 / p90 | found / soft / lost | width CV | waver |
|---|---|---|---|---|
| old (stencil) | 0.25 / 0.50 / 0.79 | 1.00 / 0 / 0 | 0.35 | 0.10 units |
| new (`edge=`) | 0.46 / 0.70 / 1.40 | 0.86 / 0.14 / 0.00 | 0.54 | 0.35 units |

What I see: the old ridge is the filled selection, a vector curve. The new
one is laid by a brush: small steps and lumps where strokes ran over, the
peak crisp against the glow, the low left flank dissolving into the sky,
and the mirror's lower edge broken by level water strokes. It is still
mostly crisp over the dry sky (86% found): a soft dark-on-light edge over
dry paint is a thinning, broken fringe here, not a fused one. Some pale
bits along the mirror's lower edge (the lost stretches' lifted fringe
beside `lose`'s strokes) may read as glints or as chips.

## 4. Tests

- `fence::tests::fences_carry_paint_past_the_edge_by_quality`: a dark disk
  over a light ground; darkening outside the edge (0–3 units, 8–16 units):
  stencil 0.000 / 0.000, found 0.245 / 0.000, lost 0.355 / 0.189; the rim
  inside covered in all three.
- `fence::tests` (shape monotone in q; stroke draws spread over 0..1).
- `draw_edges::tests::edges_lose_and_the_clip_notice`: the notice appears
  for a mask in `clip=`; a stencil lays nothing 5 units out, `edge="lost"`
  darkens more than 10 of 60 points there; every form of `edge=` parses, a
  bad name and `edge=` with `cut_in=` fail; `lose` strokes where asked and
  none where `where=0`.
- Benchmarks `notes/loops/l5_near.lua` and `l3_green.lua` at 1000:
  byte-identical to main (`cmp`). Golden scene unchanged (nothing changes
  unless `edge=` or `lose` is written).

## 5. Open issues and next

- **Not applied to Evening at a Mountain Lake.** Its log needs the wet
  engine (`r7-paint-wet`); this branch is on main. The fence touches
  `bristle.rs` where the wet engine rewrote `exchange` (`film.rs`,
  `exchange.rs`), so merging needs the same few lines there: the `Clip`
  enum, the lift in the contact threshold and the `f²` thinning of the
  deposit.
- **Firs, figure, grass.** Their edges come from other code: fir needles
  are painted with `clip=nd` (a stencil) and with pointed-brush strokes
  (`f:paint`), the figure with `clip=figM:grow(0.3)` (in fact `true`).
  `edge=` works on their `work` passes; the stroke methods (`b:stroke`,
  `f:paint`, `t:paint`) still take only a mask clip.
- **`stipple` has no `edge=` yet**, and `blend` gets it through `work`
  but untested.
- **Soft over dry paint** stays a thinning fringe, not a fusion. A real
  soft edge is usually made wet into wet: the overrun picks up the
  neighbor, and the lab's wet variant (ridge painted into the wet sky, an earlier setting of the fence) measured 35% found, 54% soft, 11% lost. The
  better recipe may be to lay the neighbor wet and lose into it.
- **`lose`** follows the mask's line even where an `edge=` pass already
  moved the visible edge; it could follow the painted edge instead (a
  contour of the canvas's value near the mask's line).
- **Honoring `clip=<mask>`**: a real fence mask would be the fix painters
  expected, but it would change every existing log; left as a notice. If
  Alice wants it, it is two lines in `work` and `stipple` plus
  re-recorded benchmarks.
