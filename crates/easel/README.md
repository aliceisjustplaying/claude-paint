# The easel

A live painting session for agents. The canvas stays alive in a background
process; you send it Lua 5.5 chunks one shell command at a time, look at the
result as a small JPEG and carry on. Every chunk that succeeds is appended to
`paintings/lua/<name>.lua`. That file is the painting: `easel run` repaints it
from scratch, byte for byte the same at the same width, or at 3200px for the
full render.

The engine underneath is the claude-paint oil-paint simulator (see the
repository README): every mark is made by simulated bristles carrying paint,
and layers combine by Kubelka–Munk optics. The easel paints nothing on its own.
You choose the motifs, colors, brushes and order.

## Quick start

```sh
cargo build --release -p easel          # once; the binary is target/release/easel
E=target/release/easel

$E open dusk                            # start a session (1000px wide by default)
$E do 'canvas{style="friedrich", aspect=1.4, seed=7}'
$E do --look '
HZ = 470
sky = function(x, y) return gradient({{0,"#5d7396"},{0.6,"#9fabb8"},{1,"#e9d6a6"}}, y/HZ) end
work(above(function(x) return HZ + 15 end), {hand="broad", color=sky, angle=0, coverage=4.5})'
# prints: ok · chunk 2 · 5.98s · ... and the path of a JPEG: read it
$E look --mode value,squint             # judge the values
$E undo                                 # didn't like it
$E close                                # the session stays in paintings/lua/dusk.lua
$E run paintings/lua/dusk.lua --width 3200    # the full render → out/lua/dusk_3200.png
```

`paintings/lua/example.lua` is a whole small study made this way: an evening
sky, a distant ridge, mist and a spruce on a knoll, in ten chunks.
`paintings/lua/rocks.lua` models a boulder and a mountain range as solids
and paints them from their light and shadow. Read both before you start.

## The loop

1. **Open.** `easel open <name> [--width 1000] [--undo 8]` starts a
   background session, or reattaches if it's already running. If
   `paintings/lua/<name>.lua` exists, the session replays it first, so you
   can close, come back and go on painting. Other commands use the session you
   opened last (or `-s <name>`, or `EASEL_SESSION`).
2. **Paint a chunk.** `easel do '<lua>'`, `easel do -f chunk.lua` or
   `easel do -` (stdin). The reply is whatever the chunk `print`ed, then
   `ok · chunk N · seconds · clock · wet/dry`. Add `--look` to also get a
   look in the same command.
3. **Look.** `easel look` prints the path of a ≤1000px JPEG (100–250 KB):
   - `--crop x0,y0,x1,y1` a window in canvas units, enlarged by whole
     pixels so you see the real grain;
   - `--mode value` (grayscale), `squint` (blurred: big shapes and values
     only), `mirror` (flipped: fresh eyes on the drawing); comma-separate
     to combine, e.g. `--mode value,squint`;
   - `--dried` (alias `--wet`): wet paint as it will look once it has
     leveled and dried; `--relief` also lights the brushwork from the upper
     left; `--size N` for the longest side.

   The plain look shows what is on the canvas now: the dry picture with wet
   paint on it as laid.
4. **Keep or undo.** A chunk that fails changes nothing: the canvas, your
   variables (including anything it changed inside tables and closures
   from earlier chunks), the paint in your brushes and the clock go back to
   how they were before it, and it isn't logged. `easel undo [n]` takes back the last
   n successful chunks (up to `--undo`, 8 by default; each snapshot costs
   about 50 MB at 1000px).
5. **Step back.** `easel log` prints the program so far. `easel status`
   gives a one-line summary. `easel save [path]` writes a PNG. `easel check`
   replays the log in a fresh session and confirms it matches the live
   canvas exactly.
6. **Time-lapse.** `easel frames on` saves a JPEG after every chunk in
   `out/easel/<name>/frames/`.

## How chunks behave

- **Globals persist, locals don't.** Each chunk is its own Lua chunk. Write
  `sky = ...` to use `sky` later; `local sky = ...` lives only in that chunk.
- **Units.** The canvas is 1000 units wide and `1000 / aspect` tall (`W`
  and `H` after `canvas{}`), y pointing down, whatever the pixel width.
  Angles are radians, 0 = left to right, π/2 = downward.
- **Randomness is deterministic.** `math.random`, `rand(a, b)` and
  `randn(mean, sd)` are reseeded at the start of every chunk from the canvas
  seed and the chunk number. `work`, `stipple`, `brush` and `tree` pick their
  own seeds the same way (pass `seed=` to fix one). A replay therefore paints
  the same thing, and a failed chunk doesn't shift the randomness of the next.
- **No OS access.** `io`, `os`, `debug`, `require`, `dofile`, `loadfile`
  and `collectgarbage` are not available. `print` goes to the reply.
- **Lua 5.5.** Numbers are integers or floats (`7 // 2` is 3, `7 / 2` is
  3.5; `math.type` tells them apart), and whole numbers from the easel (`W`,
  `H`) are integers. Bitwise operators are built in (`a & b`, `1 << 4`);
  there is no `bit` library and no global `unpack` (use `table.unpack`).
  Loop variables are read-only: `for i = 1, 3 do i = i + 1 end` is an
  error. Don't write `global` declarations: one `global x` switches its
  chunk to strict mode, and then even `print` must be declared. Plain
  assignment (`x = 1`) makes a global, as in older Lua. `pairs` walks a
  table in the same order in every session and every replay.
- **Colors** are `"#rrggbb"` strings, `rgb(r, g, b)` (0–255 sRGB) or color
  values from `mix`, `gradient`, `sample` and `pal:mix`. A color value has
  `.r .g .b` (linear), `.value` (luminance), `.L` (OKLab lightness),
  `:mix(other, t, mode)` and `:hex()`.
- **Fields.** Wherever an option takes a function of `(x, y)` (a color, an
  angle, a coverage), it's sampled every 2 units over the area being painted
  and interpolated. That's fine for gradients and noise but not for
  single-pixel detail; use masks for hard edges. Mask functions
  (`mask(fn)`) run at every pixel. Both cost about 0.1 s at 1000px.
- **Engine errors are rolled back too.** A mistyped option name is an
  error that lists the valid options.

## API reference

### Canvas and palette

```lua
canvas{style="friedrich", aspect=1.4, seed=7}   -- the first chunk; returns H
-- styles: "friedrich" (after 1820), "friedrich_early". Sets W, H and pal.
print(pal)                                   -- the tubes
pal:tubes()                                  -- list of tube names
local blues = pal:only{"lead white", "pale smalt", "cobalt blue"}
local c, recipe, err = pal:mix("#6f84a8")    -- nearest masstone the tubes reach
local c2 = pal:aim("#9fb0c0", sample(500, 200), 0.3, 1.0)  -- want, over, medium, coats
local p = pal:paint("#445566", 0.2)          -- a Paint (color, medium)
paint("#445566", {raw=true, hiding=0.5, stiff=0.3})        -- a paint not mixed from tubes
```

### Colors and helpers

```lua
mix("#334455", "#887766", t, "light")        -- modes: "light" (OKLab, default), "pigment", "linear"
gradient({{0, "#5d7396"}, {0.6, "#9fabb8"}, {1, "#e9d6a6"}}, t)
rgb(120, 130, 140)   color("#aabbcc")
smoothstep(a, b, x)  lerp(a, b, t)  clamp(x, lo, hi)
n = noise{seed=3, octaves=5, period=260, persistence=0.5}
n(x, y)              -- about -1..1        n:at01(x, y)  -- 0..1
sample(x, y, r)      -- what's on the canvas there (wet paint included)
```

### Masks

Masks are coverage maps of the whole canvas. Operations return new masks.

```lua
everywhere()
mask(function(x, y) return y < 300 and 1 or 0 end)    -- any function, 0..1
ellipse(cx, cy, rx, ry)   rect(x, y, w, h)
poly({{x, y}, ...})       poly(pts, true)              -- true: smoothed
below(function(x) return 420 + 20*math.sin(x/90) end)  -- under a curve (or a point list)
above(curve)                                           -- over it
ribbon(points, widths)    ribbon(points, 3)            -- a band along a line
m + n   m * n   m - n   -m                             -- union, intersect, subtract, invert
m:roughen(units, period, seed, edge)  -- push the edge in and out by about `units`
                                      -- (noise of `period` units), new edge `edge` units soft
m:soften(units)   m:blur(units)
m:grow(units)     m:shrink(units)   m:offset(units)
m:rim(width, soft)                -- the inside strip along the edge
m:distance()                      -- signed distance in units (+ inside): a field
m:band(lo, hi, soft)              -- turn a field back into a mask
m:times(fn or mask)  m:map(function(v) return v^2 end)
m:at(x, y)   m:area()
```

Points are `{{x, y}, {x, y}, ...}` or a flat `{x1, y1, x2, y2, ...}`.

### The hand: brushes and strokes

```lua
b = brush("round", 3)        -- kinds: round, flat, filbert, fan, rigger, badger, stippler
b = brush{kind="filbert", width=8, stiffness=0.5}      -- also: length, hair, run, lay,
                                                       -- pickup, push, splay, ragged, bristles
b:load("#2a3040", 0.9)       -- mix it from pal (masstone) and dip: amount 0..1 of a full load
b:load("#9fb0c0", 0.6, {at={500, 300}, coats=0.8})     -- aim at the look over what's there
b:load(color, 0.8, {medium=0.4, pal=blues})
b:reload(color, amount)      -- wipe most of the old paint, then load
b:wipe(0.85)                 -- on the rag
b:fullness()                 -- paint left, 0..1
b:stroke({{100, 500}, {300, 520}, {500, 510}},
  {pressure={0.9, 0.3}, ramps={0.05, 0.4}, orient="across", shake=1, swell={1, 1.3, 0.8}, clip=m})
b:touch(x, y, {pressure=0.6, drag={1, 0}, twist=0.2, angle=0.3, clip=m})
```

A brush keeps its paint across strokes and chunks: several strokes from one
load run dry naturally. `orient` is `"across"`, `"along"` or a fixed angle.

### Covering areas

```lua
work(mask, {hand="body", color=..., angle=0, ...})
```

`hand` picks a preset from the style, which you then adjust:
`broad` (long soft passes: skies, fog, water), `body` (form in body color,
the default), `detail` (small, cut in), `hatch` (short strokes side by side:
conifers, grass), `glaze` (thin veils; `medium` 0.6–0.95), `scumble`,
`blend` (a clean blender fusing wet paint). Options:

| option | meaning |
|---|---|
| `color` | required: a color or `function(x, y)` returning one: the look you want on the canvas |
| `angle` | stroke direction, a number or `function(x, y)` |
| `tool` | `"filbert 8"`, `{kind=, width=}` or a brush |
| `length` | `{min, max}` stroke length in units |
| `coverage` | layers of strokes over each point (2–5) |
| `medium` | oil medium in the paint, 0..1 (thin 0.25, body 0.15) |
| `pal` | a palette (e.g. `pal:only{...}`), or `false` for unmixed paint |
| `aim` | `"laid"` (default: aim at the look over what's there), `"masstone"`, or a number of coats |
| `pressure`, `ramps` | `{start, end}` pressure; attack and release fractions |
| `dips` | `{every, load, wipe}`: strokes per trip to the palette |
| `load`, `load_at` | load per dip; a field that varies it |
| `angle_jitter`, `curve` (`{bow, wave}`), `cross`, `drift` (`{amount, scale}`), `tail`, `broken`, `swell`, `clump` | the hand's irregularity |
| `order` | `"passages"`, `"scatter"`, `"down"`, `"across"` or a sweep angle |
| `orient`, `shake`, `clip`, `threshold`, `cut_in` (a tool), `scrub`, `blender`, `ruler`, `jitter`, `mix_jitter`, `paint` (`{hiding, stiff}`), `seed` | as in the engine's `Handling` |

```lua
blend(mask, {angle=0})                    -- = work(mask, {hand="blend", ...})
stipple(mask, {width=2.4, color="#cfccc2", coverage=function(x, y) ... end,
  pressure={0.5, 0.9}, dips={16, 0.35, 0.7}, aim=false, medium=0.6,
  drag={1, 0}, twist=0.3, cluster={0.2, 5}, feather=0.6, clip=false})
glaze(mask_or_nil, {color="#8a6a3a", coats=0.4, pigment="transparent"})   -- or semi, opaque, varnish
```

### Trees

```lua
t = tree{habit="spruce", x=310, y=500, height=210, seed=4}  -- oak, dead_oak, birch, spruce
for _, l in ipairs(t.limbs) do
  -- l.pts {{x,y},...}, l.w (width per point), l.z, l.order (0 trunk, 1 limbs, ...),
  -- l.parent (index), l.at, l.dead, l.dead_from, l.broken, l.root
end
t.tips   t.bounds   t:mask()
```

The skeleton says how the tree grew. Painting it is up to you: stroke
the limbs with brushes sized by `l.w`, build foliage masks from
`ribbon(l.pts, widths)`, and so on.

### Form: solids, light and shade

Model what you paint as solids, light them, then let their planes decide
your colors, stroke directions and edges. Form paints nothing itself.
Coordinates are canvas units, with z pointing toward you.

```lua
-- bodies: a mass, turned, weathered, broken by fracture planes; + and - combine
rock = body.ellipsoid({330, 560, 60}, {150, 105, 110})      -- center, radii
  :turn({330, 560, 60}, 0.3, 0.1, -0.12)                    -- yaw (right side toward you), pitch (top toward you), roll
  :rough(15, 150, 1)                                        -- amp, period, seed, ridged?
  :cut({330, 480, 60}, {-0.35, -1, 0.45}, 10, 4)            -- at, outward normal, facet id, round
  :rough(0.9, 25, 2, true)                                  -- pitted grain
slab = body.block({600, 600, 0}, {200, 40, 80}, 3)          -- center, size, round
-- a mountain face below a crest line (a function of x, or points)
range = ridge{crest=function(x) return 330 - 60*math.sin(x/170) end, depth=320, seed=7,
  lean={0.9, 0.7}, gullies={45, 0.5}, fan=1, base=560, z0=-600}   -- also strata={spacing, step, tilt}
-- any height field: z (toward you) or nil where there's no surface
dune = terrain{area={0, 500, 1000, 714}, height=function(x, y) return 20*math.sin(x/60) end}

-- the lit depth buffer: parts are numbered in order (1, 2, ...)
f = form{ {range, dist={2.0, 0}}, {rock, dist=0.3},          -- dist: number or {at, per_z} (aerial perspective)
  light={from={-1, -0.7}, front=0.5, ambient=0.2, penumbra=0.05} }  -- front < 0: contre-jour
  -- light also takes bounce, bounce_from={x,y,z}, reach, thickness, across_parts

f:sample(x, y)      -- nil off the form, else {part, facet, z, n, dist, fall, across, lit, shade={turn, direct, cast, bounce, sky, value}}
f:shade(x, y)  f:value(x, y)  f:lit_at(x, y, soft)  f:part(x, y)  f:dist(x, y, far)
f:fall(x, y)  f:across(x, y)  f:bend(x, y, span)  f:edge_angle(x, y, span)
aerial(dist, visibility)     -- how much of a color the air replaces, 0..1

-- masks
f:parts_mask{2}                                   -- where a part is in front
f:lit{parts={2}, soft=0.12}   f:shadow{parts={2}} -- the light and shadow families
f:silhouette{parts={1}, soft=0.6, haze={3, 4}}    -- edge soft + 3·aerial(dist, 4)² units wide
f:edges{turn=0.8, step=3, span=2.5, concave=true} -- plane breaks and overlaps (concave: only hollows)
f:mask(function(s) return s.shade.sky end)        -- any rule (serial, ~0.5 s at 1000px; s is reused)

-- stroke directions straight from the form (no grid): down the planes, around them, along breaks
work(f:silhouette{parts={2}} * f:shadow{parts={2}}, {hand="body", color=function(x, y)
  return mix("#3e3a36", "#8a8070", f:value(x, y)) end, angle=f:field("fall")})
f:field("across")   f:field("edge", 2.5)
```

Place solids in depth with z: a ridge's face leans toward you at its foot,
so set its `z0` back (e.g. `z0=-600`) or it will hide the rocks in front of
it. A form costs 28 bytes per pixel (20 MB at 1000px, 190 MB at 3200px).

### Time and finishing

```lua
wait(minutes)         -- time passes: the paint ages where it lies; returns the clock
dry()                 -- wait until every film is touch-dry; returns the clock
clock()               -- painting minutes since canvas{}
drying(x, y)          -- "open", "setting", "tacky" or "dry" there
varnish{color="#e6d3a4", coats=0.4, vary=0.12}
cracks{dirt=0.4, vary=1, veil=0.5}          -- craquelure, fitted to this canvas's ground;
                                             -- also island_mm, ground_um, width_um (default:
                                             -- from the ground), depth_um, cupping_um, corners
relief(strength, gloss)                      -- light the surface relief (style default)
```

`wait` runs the engine's drying model: each pixel's paint goes from open
(workable, blends and lifts) through setting (stiff, barely blends) to
tacky (set, grabs the brush) and touch-dry, at a pace set by its pigments,
film thickness and oil. So a chunk can work wet into wet (no wait), come
back to tacky paint (`wait(180)`) or paint over a dry layer
(`wait(24*60)`). The clock is written into the log at every chunk, so time
is part of the program.

## Replay

```sh
easel run paintings/lua/<name>.lua [--width 3200] [--out path.png]
          [--crop x0,y0,x1,y1] [--margin 40] [--look]
```

This runs the chunks in order in a fresh session and writes
`out/lua/<name>_<width>.png`; `--look` also writes a JPEG next to it. At the
width you painted at, the PNG is byte-identical to `easel save` from the live
session. At 3200px it is the full render: the same program at a finer grain.
The file is plain Lua with chunk markers (`--@ chunk N · clock M`), so you
can edit it by hand and replay. If you do, keep the markers.

## Costs (1000px, busy 10-core machine)

- `canvas{}` (the primed linen) ≈ 3 s; a broad sky pass with a blend
  ≈ 6 s; a body passage ≈ 1–1.5 s; a stipple pass ≈ 0.2 s; hundreds of
  brush strokes along tree limbs ≈ 0.05 s
- `look` ≈ 0.04 s, `look --dried` ≈ 0.3 s
- replaying the example: 18–29 s at 1000px (see notes/easel.md for 3200px)
- rollback bookkeeping ≈ 1 µs per live Lua table per chunk (a tree's 4,000
  tables: under 5 ms)

## Limits

- Rollback restores everything reachable from your globals: tables, their
  metatables and the variables your functions close over. A coroutine
  suspended across chunks is not restored. After a rollback that restored a
  table the failed chunk had changed, `pairs` over that table may visit keys
  in a different order than a replay would; `easel check` detects this, and
  `ipairs` and numeric loops are unaffected.
- Sessions paint the whole canvas. `easel run --crop` renders a window
  of a finished program.
