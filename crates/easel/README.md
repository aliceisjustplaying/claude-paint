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
and paints them from their light and shadow. `paintings/lua/meadow.lua` is
a daylight landscape built on a world: a sky and clouds for its sun, hazed
ranges, a beech in leaf casting its shadow and a meadow of grass tufts.
Read all three before you start.

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
   how they were before it, and it isn't logged. This holds at any
   `--undo`, including `--undo 0`. `easel undo [n]` takes back the last
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
  assignment (`x = 1`) makes a global, as in older Lua.
- **`pairs` and `next` walk a table in the same order in every session and
  every replay.** A table keyed only by strings, numbers and booleans walks
  in Lua's own order. A table with any other key (a table, function, mask
  or other easel value) walks in a fixed order instead: booleans, numbers,
  strings (each ascending), then the other keys in the order your program
  created them. That costs a sort per walk, so for a big collection of
  objects a list and `ipairs` is still the better choice.
- **`string.gmatch` iterators survive rollback.** An iterator kept in a
  global resumes where it was before a failed or undone chunk.
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
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.5, seed=11}
canvas{style="friedrich_early", size=440, aspect=1.4, seed=11}   -- size: the width in mm
-- (the style's by default: 440 after 1820, 1714 for the early grounds). Pencil
-- lines are fractions of a mm, so they read at 1000px on a small canvas.
-- palettes: friedrich_1820, friedrich_early, and the same with _greens (adds
-- Prussian blue, green earth and, after 1820, Rinmann's green) for summer
palette("friedrich_early_greens")            -- any palette by name
pal:with{"copper green"}                     -- a rare tube for one passage
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
shift(c, dL, da, db) -- the same color moved in OKLab: shift(c, -0.06, 0, -0.02) is darker, bluer
```

### Noise and irregularity

```lua
noise{kind="ridged", seed=3, period=120}        -- kinds: fbm (default), ridged (crests), billow (heaps)
noise{period=200, warp={80, 25}}                -- domain warp: {period, amount, twice?}: folds, wisps
noise{period=150, stretch={0.2, 4}}             -- stretched 4x along angle 0.2: wind, bedding
cells = worley{seed=2, period=30}               -- cells: stones, cracked mud, clumps
local f1, f2, edge, r = cells:at(x, y)          -- edge is 0 on a cell wall; r is 0..1 per cell
uneven(9, 120, 860, 0.6, 0.4, 5)                -- 9 positions from 120 to 860, spaced by hand:
                                                -- irregular gaps, clumped (posts, trees, boats)
```

A noise can be passed straight to `coverage=` or `load_at=` (as 0..1).

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
b:mark_width(0.4)            -- round and rigger tips are pointed: width of a mark at this pressure
b:pressure_for(0.5)          -- the pressure for a 0.5-unit line
brush{kind="round", width=3, point=0.5}                -- a blunter point (1 = sharp, default)
```

A flick that ends in a hairline is a stroke whose pressure falls to 0:
`b:stroke({root, mid, tip}, {pressure={0.75, 0}, ramps={0.1, 0.75}})`.

A brush keeps its paint across strokes and chunks: several strokes from one
load run dry naturally. `orient` is `"across"`, `"along"` or a fixed angle.

### Drawing: pencil, chalk and eraser

Draw the composition on the ground before you paint, the way Friedrich did:
graphite pencils of different hardness and black chalk, often a faint first
pass and then a bolder one, the straights against a ruler. The drawing lies
in the picture under the paint: thin paint lets it show through (as in his
early works), and body color hides it.

```lua
h = pencil("2H")                  -- or pencil{grade="2H"}: 9H..H, F, HB, B..9B
b = pencil{grade="2B"}
c = chalk()                       -- black chalk: deep, matte, broad, crumbly
h:sketch(pts, {pressure=0.3})     -- a searching line: a few light passes, each its own guess
                                  -- (passes=3, wander= units, smooth=true)
b:line(pts, {pressure={0.5, 0.7, 0.4}})   -- one firm line through the points (smooth=false keeps corners)
h:rule({0, 432}, {1000, 432}, {pressure=0.3})   -- straight, against a ruler
b:hatch(mask, {angle=-1.1, pressure=0.35})     -- short parallel strokes (spacing=, length= in units)
b:width()   b.worn   b:sharpen()  -- the point blunts as you draw (soft leads fast); lines widen
erase(pts, {strength=0.9, width=9}) -- a kneaded eraser along a path, or erase(mask, {strength=})
fix()                             -- fixative (or fix(mask)): the eraser no longer lifts it
drawing_mask()                    -- where the drawing is, 1 on a firm line, also under paint
```

How it behaves: the point rides on the tops of the canvas weave, and
pressure lets it reach into the hollows, so a light line is a broken line
of grain and a heavy one fills in. Soft leads lay darker, glossier gray;
hard ones lay a pale silver gray. Graphite doesn't take on wet paint. The
eraser lifts most of a line, and better from the tops than the hollows. A
ghost stays behind, as on a real canvas. Once paint has gone over the
drawing, it is sealed. Look at the drawing with `look --crop` (it is fine
work).

A worked example (from `paintings/lua/pencil.lua`):

```lua
h2 = pencil("2H")                                  -- first pass: light and searching
h2:rule({0, 432}, {1000, 432}, {pressure=0.3})
h2:sketch(ROCK, {pressure=0.3})
b = pencil("2B")                                   -- second pass: firm (the top came out too high)
b:line(WRONG, {pressure={0.55, 0.7, 0.6, 0.5}})
-- next chunk, after a look: lift the wrong top and redraw it
erase(top_of_wrong, {strength=0.9, width=9})
b:line(ROCK, {pressure={0.55, 0.7, 0.6, 0.5}, smooth=false})
b:hatch(poly(shadow_side), {angle=-1.1, pressure=0.35})
fix()
-- thin, translucent paint, blended: the drawing shimmers through
work(rock * LEFT, {hand="body", color="#8d8a80", medium=0.6, load=0.3, coverage=2.2, paint={0.3, 0.3}, clip=LEFT})
blend(LEFT, {angle=0.04, clip=LEFT})
-- body color hides it; paint the next thing into the hidden drawing
work(rock * RIGHT, {hand="body", color="#8d8a80", medium=0.15, load=0.9, coverage=4, clip=RIGHT})
dry()
local firm = drawing_mask():band(0.55, 1, 0.1):grow(1.2) * rect(660, 290, 240, 190)
work(firm, {hand="detail", tool="round 2", color="#3b342c", coverage=3, length={4, 10}})
```

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
| `color` | a color, `function(x, y)` returning one, or a sky or clouds (`color=s`): the look you want on the canvas |
| `color_over` | instead of `color`: relative to what's under each stroke. `{shift={dL, da, db}}` (e.g. a shadow: `{shift={-0.06, 0, -0.012}}`), or `function(x, y, under)` sampled every 2 units with `under` = the canvas there before the pass |
| `hug` | `true` (default): strokes reach a mask's edges; `false` lets edges thin out |
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
  drag={1, 0}, twist=0.3, cluster={0.2, 5}, feather=0.6, clip=false,
  fade=1})                                -- fade: contrast falls where coverage thins; 0 for specks
                                          -- (stars, snowflakes); color_over works here too
glaze(mask_or_nil, {color="#8a6a3a", coats=0.4, pigment="transparent"})   -- or semi, opaque, varnish
```

### Trees and foliage

```lua
t = tree{habit="beech", x=700, y=336, height=236, seed=5}
-- habits: oak, dead_oak, birch, spruce, beech, alder, willow; years= to grow older or younger
for _, l in ipairs(t.limbs) do
  -- l.pts {{x,y},...}, l.w (width per point), l.z, l.order (0 trunk, 1 limbs, ...),
  -- l.parent (index), l.at, l.dead, l.dead_from, l.broken, l.root
end
t.tips   t.bounds   t:mask()
leaves = t:foliage{sun={-0.6, -0.7, 0.35}, seed=5}   -- sun: toward it (x right, y down, z to you)
-- winter=true: bare; also years, clump, spacing, squash, droop, fill, ragged, bare, tip, spray
leaves:mask()   leaves:lit()   leaves:gaps(6)   leaves:envelope(6)   -- masks
leaves.clumps   -- back to front: {x, y, z, r, squash, tilt, fill, lit, shade, limb, mass}
```

The skeleton says how the tree grew, and the foliage where its leaves are
and how the sun reaches them. Painting them is up to you. A worked example
(from meadow.lua):

```lua
local trunk, limb, twig = brush("round", 3.5), brush("round", 1.4), brush("rigger", 0.6)
for i, l in ipairs(t.limbs) do
  local b = (l.order == 0) and trunk or ((l.w[1] > 0.9) and limb or twig)
  if i % 5 == 1 or b:fullness() < 0.3 then b:reload("#3e372f", 0.9) end
  b:stroke(l.pts, {pressure={0.85, 0.2}, ramps={0.03, 0.5}})
end
local turn = noise{seed=7, period=12}      -- leaf strokes turn every which way, not in rows
local way = function(x, y) return 2.2 * turn(x, y) end
work(leaves:mask(), {hand="hatch", tool="round 1.6", length={3, 7}, coverage=2.6, angle=way, color="#2c3a22"})
work(leaves:lit() * leaves:mask(), {hand="hatch", tool="round 1.3", length={2, 5}, coverage=2.2, angle=way, color="#6a843b"})
```

### Meadows

```lua
tufts = sward{region=below(function(x) return HZ + 40 end), horizon=HZ, near=H, height=30,
  flowers=0.05, seed=4, wind={lean=0.15, gust=0.2, period=160, seed=2}}
-- also spacing, thin, smallest, blades={lo, hi}, fan, curl, kinds, patch, patch_size
-- tufts far first: {x, y, scale, height, lean, lush, blades={{foot, ctrl, tip}, ...}, flower={x, y, r, kind}}
local g = brush("rigger", 0.7)
for i, t in ipairs(tufts) do
  if i % 4 == 1 then g:reload(i % 8 == 1 and "#7a8f3c" or "#5e7a30", 0.7) end
  for _, bl in ipairs(t.blades) do
    g:stroke(bl, {pressure={clamp(0.25 + 0.5 * t.scale, 0.2, 0.9), 0}, ramps={0.05, 0.7}})
  end
end
```

Tufts shrink and thin toward the horizon and stop where they'd be smaller
than `smallest`: paint the far meadow as tone underneath first.

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

### The world: camera, ground, sun, shadows, water

A world is one picture's space in meters: a camera (eye height, horizon,
field of view), the ground, water, one sun and the bodies standing there.
Build it once. `w:place` returns a new world (keep the result).

```lua
HZ = H * 0.46
w = world{horizon=HZ, eye=1.7, fov=50,                 -- horizon: canvas y at eye level
  sun={azimuth=-125, elevation=38},                    -- degrees: 0 ahead (contre-jour), -90 left,
                                                       -- 180 behind you; below 0: twilight
  ground=function(X, Z) return 1.2*math.sin(X/40) * math.min(1, Z/60) end,   -- meters, gentle
  water={level=-0.2, ripple={0.03, 1.4, 0.3, 7}}}      -- fills hollows below the level
-- also view={x, y, w, h} (a panel), visibility (m), backdrop (m)
local s = w:spot(700, 336)             -- the ground seen at a canvas point (or w:spot_at(X, Z))
print(s)                               -- spot(x, y, units per meter, at X, Y, Z m)
local tall = w:height(s.x, s.y, 18)    -- how tall 18 m looks standing there (units)
local rock = body.block(s:p(0, 0.4, 0), s:size(1.3, 1.0, 1.1), s:m(0.3))   -- meters -> units
w, stone = w:place(s, rock)            -- a body in the world (lit, shadowed, reflected)
w = w:proxy(s, body.ellipsoid(s:p(0, 11, 0), s:size(7, 6, 6)))   -- casts a shadow but isn't painted
                                       -- as a form: stand-in for a tree crown or a figure
v = w:view()                           -- trace once and keep it
v:sky()  v:land()  v:water()  v:shadows()  v:contact(0.25)  v:reflections()  v:bodies_mask{stone}
v:at(x, y)      -- {what="sky"|"ground"|"water"|"body", body, at={X, Y, Z}, n, dist, shade, lit}
v:mirror(x, y)  -- what calm water shows there: {body, src={x, y}, shade, fresnel, travel} or nil
v.form          -- the bodies as a form: v.form:lit{parts={v:part(stone)}}, v.form:field("fall"), ...
                -- (v.form.parts counts visible bodies only; v:part(proxy) is 0)
w:to_ground(x, y)  w:project(X, Y, Z)  w:scale_at(Z)  w:aerial(Z)  w:shadow_angle(x, y)  w:sun_canvas()
w:ribbon({{-2.6, 4.5}, {-1.6, 8.6}, {-0.4, 9.7}}, 1.1)   -- a path on the ground (width m) as a mask
w:recede({2, 6}, {0.3, 4}, 8)                             -- spots stepping away: posts, footprints
```

A shadow is the ground a little darker and bluer, stroked the way it falls:

```lua
work(v:shadows() * v:land(), {hand="body", tool="filbert 3", length={6, 16}, coverage=3,
  angle=w:shadow_angle(s.x, s.y + 2), color_over={shift={-0.06, -0.004, -0.012}}})
```

### Sky, clouds, haze and distant ranges

```lua
sk = w:sky{haze=2.2, uneven={0.5, 30000, 5}}   -- the sky for this world's sun, as paint colors
-- also layer={alt, thick, density, uneven, seed}, overcast=0..1, exposure, balance (adapt to the sun's color)
cl = w:clouds{sky=sk, cell=3,                  -- meters; x right, z away, base/top altitude
  {kind="cumulus", x=-3000, z=9000, base=1300, width=3000, height=1500, seed=4},
  {kind="bank", x0=-2000, x1=30000, z=38000, depth=9000, base=700, top=2600, seed=4},
  {kind="stratus", base=3500, thick=500, cover=0.35, seed=2, wind={-0.2, 2}, breaks={14000, 0.2}}}
work(above(function(x) return HZ + 12 end), {hand="broad", color=cl, angle=0, coverage=4.5})
cl:mask{alpha={0.3, 0.8}, lit={0.4, 0.9}}      -- lit cloud edges; shade={..} for the bellies
sk:at(x, y)  sk:airlight(x)  cl:alpha(x, y)  cl:lit(x, y)

air = haze{visibility=18000, height=800, mist={140, 6, 90, 7}}   -- mist: {top m, density, uneven m, seed}
rs = w:ranges{near=6000, far=22000, count=2, seed=8, heights={180, 700}, kinds={"dome", "saddle"}}
for i = #rs, 1, -1 do                          -- far to near; kinds: peak, dome, plateau, saddle, cliff
  local l = rs[i]
  work(l:mask() * above(function(x) return HZ + 3 end), {hand="body", length={20, 60}, angle=0.05,
    color=function(x, y) return mix("#3f4c44", sk:airlight(x), 0.75 * l:haze(air, x, y)) end})
end
-- l:crest(x), l:z_at(x), l:ridge{depth=200, gully=260} (a solid for form{} to light)
```

A sky or clouds passed as `color=` is read on the engine's threads, so it
costs nothing extra. Twilight is the same code with the sun below the
horizon (`elevation=-4`): the sky turns to its afterglow, the Earth's
shadow and the Belt of Venus, and the clouds catch the light from below.

### A whole painting, in order

1. `canvas{}`, then `w = world{...}` and `sk = w:sky{...}` (and clouds).
2. The sky, thin, in long strokes, then `blend`; `wait(24*60)`.
3. The distance: ranges far to near, each hazed by `l:haze`; mist
   stippled over their feet.
4. The ground as tone, colored by distance (`w:to_ground`, `w:aerial`).
5. The motifs: bodies and proxies placed in the world, `v = w:view()`,
   then shadows and contact, trees (limbs, then foliage dark to light),
   rocks from `v.form`, meadows from `sward`.
6. The small particulars last, with pointed brushes: twigs, blades,
   flowers, figures.
7. `wait(24*60); varnish(); relief()` (and `cracks{}` for an old picture).

For winter, grow trees bare (`t:foliage{winter=true}` or no foliage), and
paint snow as the ground's color with `color_over` for its blue shadows.

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

A day isn't always enough: thick, oily or slow-drying paint (bone black,
lakes, heavy body color) can stay open for weeks of painting time. Check
with `drying(x, y)` before painting over a passage, or call `dry()`.

`glaze` goes over dry paint, so it first waits until everything under it is
touch-dry; that time passes on the clock and the easel says so
(`glaze: waited 9.8 days for the paint under it to dry`). Any other time the
canvas spends (a finishing verb drying the paint first) is also reported at
the end of the chunk, so the clock you see is always the canvas's.

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
can edit it by hand and replay. If you do, keep the markers, and edit it
with the session closed: a live session writes the log after every chunk.
If it finds the file was edited while it was open, it keeps your version as
`<name>.edited-N.lua` next to it (and says so) instead of overwriting it;
close, copy it back and open again to paint on from your edit.

## Costs (1000px, busy 10-core machine)

- `canvas{}` (the primed linen) ≈ 3 s; a broad sky pass with a blend
  ≈ 6 s; a body passage ≈ 1–1.5 s; a stipple pass ≈ 0.2 s; hundreds of
  brush strokes along tree limbs ≈ 0.05 s
- `look` ≈ 0.04 s, `look --dried` ≈ 0.3 s
- replaying the example: 18–29 s at 1000px (see notes/easel.md for 3200px)
- creating Lua tables and closures costs about 15% more than in plain Lua
  (the easel numbers them for `pairs`); arithmetic is unaffected
- rollback bookkeeping ≈ 1 µs per live Lua table per chunk (a tree's 4,000
  tables: under 5 ms; a meadow of 9,000 tufts kept in a global: 0.1 s). Keep
  big lists `local` when later chunks don't need them.
- `w:view()` ≈ 0.3 s, `w:sky{}` ≈ 0.2 s, `w:clouds{}` 1–4 s (cell 2–3)

## Limits

- Rollback restores everything reachable from your globals: tables, their
  metatables, the variables your functions close over and `gmatch`
  iterators. A coroutine suspended across chunks is not restored. After a
  rollback that restored a string-keyed table the failed chunk had changed,
  `pairs` over that table may visit keys in a different order than a replay
  would; `easel check` detects this, and `ipairs`, numeric loops and tables
  with object keys are unaffected.
- A table that once held object keys and then lost them all may keep an
  order that depends on the process until it grows again. Build a fresh
  table if you rely on its `pairs` order.
- Sessions paint the whole canvas. `easel run --crop` renders a window
  of a finished program.
