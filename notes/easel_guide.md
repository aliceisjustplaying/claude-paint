# The easel

The easel is a live oil painting. You send it Lua chunks with its `paint`
tool, one at a time, and look at the canvas between them with `look`. Under it is a physical paint simulator: simulated
bristles carry wet paint over a primed linen canvas, the paint levels and
dries on a clock, and layers combine by Kubelka–Munk optics.

Three things hold for every session:

- **Every chunk that runs stays on the canvas.** There is no undo. To
  change something, paint over it, or lift wet paint off with a brush.
- **A chunk that stops with an error changes nothing.** The canvas, your
  variables, the paint on your brushes and the clock are as they were
  before it, and it isn't written to the log.
- **The log is the painting.** Every chunk that ran is appended to
  `paintings/lua/painting.lua`, and replaying it paints the same canvas.

## Starting

The studio holds one painting, and the easel is open on it. Its tools:

| tool | what it does |
|---|---|
| `paint` | runs a chunk of Lua. The reply is what the chunk printed, then `ok` |
| `look` | shows you the canvas as it is now (see [Looking](#looking)) |
| `note` | adds an entry to your journal (see [The journal](#the-journal)) |
| `status` | the canvas's setup |
| `log` | the painting so far: every chunk that ran, each after a line `--@ chunk` |

The first chunk is `canvas{}` (see [The canvas](#the-canvas)). `read` reads the files in this folder: your brief and
your notes.

In the examples below, `<tube>` stands for a name from the tube box and
`<parts>` for a number of parts you choose. Other numbers in the examples
only show how a call is written; they aren't recommended values.

## The canvas

```lua
canvas{size=<mm>, aspect=<width / height>, linen={<warp>, <weft>}, seed=<seed>,
       ground={{pile={{"<tube>", <parts>}, {"<tube>", <parts>}}, um=<µm>, apply="<how>", texture=<0..1>},
               ...}}
```

- `size`: the canvas's width in mm (50 to 5000).
- `aspect`: width / height.
- `linen`: plain-weave linen, threads per cm, one number or `{warp, weft}`
  (4 to 60).
- `ground`: the preparation layers, bottom first. Each is a paste mixed
  from tubes in parts (as a pile is, below), a thickness in µm (5 to 400)
  and how it is put on: `"knife"` (levels the weave; `texture` 0..1 is the
  knife's waviness), `"roller"` (a fine even texture) or `"brush"` (laid in
  crossing strokes, its striations stay). The ground is dry when painting
  starts.
- `seed`: the randomness of the linen, the ground and everything after.

It sets `W` (1000) and `H` (`1000 / aspect`). The canvas is always 1000
units wide, whatever its pixel width. The origin `(0, 0)` is the upper
left corner; x increases to the right and y downward. Angles are radians,
0 left to right, π/2 downward.

## Tubes and piles

Paint reaches the canvas only from piles you knife together on the
palette from the tubes in the box:

```lua
p = pile{{"<tube>", <parts>}, {"<tube>", <parts>}}
q = pile{{"<tube>", <parts>}, medium=0.3}
print(p)                         -- pile(<tube> <parts>, <tube> <parts>; medium 0)
print(table.concat(tubes(), ", "))   -- the names in the box
```

A pile is parts by volume of named tubes, plus `medium`: the share of oil
medium mixed in (0, as from the tube, to 0.95). Medium makes the paint more
transparent and more fluid, and slower to dry. It is added oil only. What a pile looks like is
what its pigments make together, thick or thin, over what is already on
the canvas; you find out by painting with it and looking. A pile mixed by
hand is a little uneven: each brushload takes slightly different
proportions (about 6%). The palette has room for 16 piles; the oldest is
scraped off to make room.

The tube box:

| tube | pigment | hiding | stiffness | tinting strength | drying |
|---|---|---|---|---|---|
| lead white | basic lead carbonate | 0.82 | 0.8 | 1.0 | 2.0 |
| smalt | cobalt potash glass, coarse | 0.3 | 0.55 | 0.45 | 1.6 |
| pale smalt | a paler grade of smalt | 0.35 | 0.55 | 0.35 | 1.6 |
| yellow ochre | hydrated iron oxide earth | 0.8 | 0.7 | 0.8 | 0.8 |
| red earth | iron oxide earth | 0.85 | 0.7 | 0.9 | 1.0 |
| vermilion | mercuric sulfide | 0.9 | 0.75 | 1.0 | 0.4 |
| raw umber | iron and manganese oxide earth | 0.8 | 0.65 | 0.9 | 2.4 |
| bone black | charred bone (carbon, calcium phosphate) | 0.9 | 0.7 | 1.1 | 0.4 |
| cobalt blue | cobalt aluminate | 0.55 | 0.6 | 0.8 | 1.4 |
| chrome yellow | lead chromate | 0.9 | 0.7 | 1.0 | 1.8 |
| Prussian blue | iron ferrocyanide | 0.35 | 0.45 | 3.0 | 1.8 |
| green earth | celadonite and glauconite clay | 0.2 | 0.35 | 0.3 | 0.8 |
| Rinmann's green | cobalt-zinc oxide | 0.35 | 0.5 | 0.4 | 1.4 |
| copper green | verdigris ground in oil | 0.25 | 0.4 | 1.0 | 1.6 |

Hiding is how much one coat of the tube paint hides what is under it (0
transparent, 1 opaque). Stiffness is the paint's body as it comes from the
tube (0 fluid, 1 stiff). Tinting strength is relative to an average
pigment. Drying is the rate relative to average paint (higher dries
faster); a pile dries at its tubes' rates mixed by volume. These numbers
are estimates from the pigment literature, not measurements.

## Brushes and strokes

```lua
b = brush("filbert", 8)                  -- kinds: round, flat, filbert, fan, rigger, badger, stippler
b = brush{kind="round", width=3, point=1, stiffness=0.5}   -- also length, hair, run, lay, pickup,
                                                           -- push, splay, ragged, bristles
b:load(p, 0.8)                           -- dip into a pile: 0..1 of a full load
b:reload(q, 0.8)                         -- wipe most of the old paint off, then load
b:wipe(0.85)                             -- remove 85% of the paint onto the rag
b:fullness()                             -- paint left, 0..1
b:stroke({{120, 640}, {260, 470}, {430, 420}},
  {pressure={0.9, 0.3}, ramps={0.05, 0.4}, orient="across", shake=1, swell={1, 1.3, 0.8}, clip=m})
b:touch(400, 300, {pressure=0.6, drag={1, 0}, twist=0.2, angle=0.3, clip=m})
b:mark_width(0.4)                        -- the width of a mark at this pressure (units)
b:pressure_for(0.5)                      -- the pressure for a 0.5-unit line
```

Widths are in canvas units. A brush keeps its paint across strokes and
chunks, so strokes from one load run dry, and a brush that has been
through wet paint carries some of it. Pressure ranges from 0 (lifted)
to 1 (fully pressed); a stroke's `pressure` is `{start, end}`.
`ramps` gives the press-down and lift-off fractions, `swell` pressure factors
along the stroke, `orient` `"across"`, `"along"` or a fixed angle.

Brushes are blunt unless given a `point` (0 blunt, 1 a full point). A
pointed round or rigger lays a hairline at light pressure and spreads to
its belly when pressed, so its width follows the pressure and a stroke
whose pressure falls to 0 ends in a point.

## Covering an area

```lua
work(m, {hand="<hand>", pile=p, angle=<radians>, coverage=<layers>})
blend(m, {angle=<radians>})              -- a clean blender over wet paint (= work with hand="blend")
stipple(m, {pile=p, width=<units>, coverage=<layers>})
work(m, {hand="glaze", pile=q})          -- a thin layer brushed on with a soft brush
lose(m, {pile=p, where=0.5})
```

`work` covers a mask with strokes of a real brush, planned the way a hand
lays them. `hand` picks a starting handling, which the options below
change:

| hand | tool | strokes |
|---|---|---|
| `broad` | filbert 22 | 80–220 units, bowing in long arcs, direction wandering over 350-unit patches |
| `body` (default) | filbert 9 | 20–60 units |
| `detail` | round 2.2 | 4–14 units, clipped to the mask |
| `hatch` | round 2.6 | 5–12 units, nearly straight, side by side, clumped |
| `glaze` | soft filbert 26 | 120–300 units, light pressure |
| `scumble` | filbert 9 | 10–20 units, worked back and forth |
| `blend` | badger 40 | a clean blender, crossing passes top to bottom, inside the mask |

| option | meaning |
|---|---|
| `pile` | the pile every stroke dips into (every hand but `blend`) |
| `tool` | `"filbert 8"`, `{kind=, width=, ...}` or a brush |
| `length` | `{min, max}` stroke length in units |
| `coverage` | layers of strokes over each point on average |
| `angle` | stroke direction: a number or `function(x, y)` |
| `pressure`, `ramps` | `{min, max}` pressure; attack and release fractions |
| `dips` | `{strokes per trip to the palette, load, wipe before dipping}` |
| `load`, `load_at` | load per dip; a number or `function(x, y)` that varies it |
| `edge` | how the passage meets the mask's edge (below) |
| `clip` | `true`: every bristle stops on the mask's edge; or a mask to clip to |
| `hug` | `true` (default): strokes reach the mask's edges; `false` lets coverage thin there |
| `fill` | `false` by default: gaps between strokes stay. Set `true` to follow the strokes with dabs into the gaps they left |
| `order` | `"passages"` (default), `"scatter"`, `"down"`, `"across"` or a sweep angle |
| `angle_jitter`, `curve` (`{bow, wave}`), `cross`, `drift` (`{amount, scale}`), `tail`, `broken`, `swell`, `clump`, `ruler` | how far the strokes depart from even ruler lines (`ruler=true` sets them straight and even) |
| `orient`, `shake`, `threshold`, `cut_in` (a tool), `scrub`, `blender`, `mix_jitter`, `seed` | the brush's orientation, the hand's unsteadiness, the mask level strokes are anchored at, cutting in the edge with a second tool, back-and-forth strokes, a clean brush, how uneven each dip of the pile is, the randomness |

Where an option takes `function(x, y)`, it is sampled every 2 units over
the area and interpolated. A mistyped option is an error that lists the
valid ones.

**What stays inside the mask.** `work` plans its strokes from the mask:
each stroke is anchored in it, but its path can begin outside and cross
the edge. By default `detail` and `blend` keep their paint inside the
mask. The other hands (`body`, `broad`, `hatch`, `glaze`, `scumble`) can
carry paint past the edge onto whatever is there, by as much as a stroke's
length (a `glaze` stroke is 120–300 units by default). `clip=true` keeps
every bristle inside the mask; `clip=` another mask keeps them inside that
one instead. `edge=` (below) shapes how the passage meets the edge, with
an overrun that varies and can reach beyond it.

**Edges.** `edge=` carries the passage up to the mask's edge the way a
brush does: each stroke stops by its own amount, and past the line its
film thins out over the weave. It takes a number from 0 (found: crisp) to
1 (lost: the passage runs well past the line and dissolves), a name
(`"found"`, `"firm"`, `"soft"`, `"loose"`, `"lost"`), a function of
`(x, y)`, a mask, or a table: `{found=, soft=, lost=, period=40, seed=}`
lays those shares out along the contour in runs about `period` units long;
`waver=` and `reach=` scale how far the line wanders and how far strokes
run over. It can't be combined with `cut_in`.

**`stipple(m, {...})`** lays many small touches of a tip, each through the
bristles: `pile`, `width` (a stippler's width, 2 by default) or `tool`,
`coverage` (touches per point; a number or `function(x, y)`), `pressure`
`{min, max}`, `dips` `{touches per dip, load, wipe}`, `drag` (how far the
tip moves while down: a number or `{length, angle}`), `twist`, `cluster`
(0 even .. 1 in clumps, or `{amount, size}`), `feather` (0 by default;
above 0, touches get lighter where coverage is below 1), `clip`,
`mix_jitter`, `seed`.

**`lose(m, {pile=, where=, ...})`** drags a lightly loaded brush across
the mask's edge from the outside in, where `where` (as `edge=`; 1 by
default) is high: short strokes start outside, cross the edge at a slant
(or at `angle`) and lift off inside. Also `tool` (`"filbert 4"`), `reach`
(`{out, in}` units), `load` (0.2), `pressure` (`{0.35, 0.02}`), `every`
(a stroke every 1.2 brush widths), `seed`. It returns the number of
strokes.

`work`, `blend` and `stipple` also take `visible=`, `behind=`,
`at=` and `view=` (see [Depth](#depth)).

## Masks and geometry

Masks are coverage maps (0..1) of the whole canvas. Operations return new
masks.

```lua
everywhere()   rect(x, y, w, h)   ellipse(cx, cy, rx, ry)
poly({{x, y}, ...})   poly(pts, true)                  -- true: smoothed
below(function(x) return 150 + 0.6 * x end)   -- under a curve (or a point list)
above(curve)
ribbon(points, widths)   ribbon(points, 3)             -- a band along a line
mask(function(x, y) return x < 500 and 1 or 0 end)    -- any function, at every pixel
m + n   m * n   m - n   -m                             -- union, intersection, difference, inverse
m:roughen(units, period, seed, edge)   m:soften(units)   m:blur(units)
m:grow(units)   m:shrink(units)   m:offset(units)   m:rim(width, soft)
m:distance()   m:band(lo, hi, soft)   m:times(fn or mask)   m:map(function(v) return v * v end)
m:at(x, y)   m:area()
```

Points are `{{x, y}, ...}` or a flat `{x1, y1, x2, y2, ...}`.

Numbers and randomness: `rand(a, b)`, `randn(mean, sd)`, `math.random`,
`lerp(a, b, t)`, `clamp(x, lo, hi)`, `smoothstep(a, b, x)`;
`noise{seed=, octaves=4, period=200, persistence=0.5, kind="fbm"|"ridged"|"billow", warp={period, amount, twice}, stretch={angle, k}}`
(call it as `n(x, y)` for about -1..1, `n:at01(x, y)` for 0..1; it can be
passed to `coverage=` or `load_at=`); `worley{seed=, period=, jitter=}`
(`c:at(x, y)` gives the distances to the nearest two cell points, how near
a cell wall, and a stable 0..1 per cell); `uneven(n, lo, hi, irregular,
clump, seed)` (n positions from lo to hi with uneven, clumped gaps).

**Drawn lines.** `outline{pts... or pts=, char=, seed=, closed=, open=,
corners=, corner_angle=60, size=, amount=, lobe=, edge=}` turns a few
points into a line drawn by hand (a smooth curve through them, broken at
corners marked `"c"`, with a hand's irregularity) and its mask:

```lua
o = outline{{300, 600, "c"}, {320, 450}, {420, 380, "c"}, {560, 400}, {650, 480, "c"}, {500, 620}, char="firm", seed=3}
work(o:mask(), {hand="body", pile=p})
o:paint(b, {pressure=0.8, dip={p, 0.6}, every=3})   -- stroke the line itself
```

| char | the line |
|---|---|
| `firm` (default for `outline`) | long strokes, slight overshoots, a crisp mask |
| `searching` | short strokes restated a little off each other, running past corners |
| `broken` | straight facets and chips in broken stretches between quiet ones |
| `soft` (default for `body_of`) | lobes of two sizes, light broken strokes, a mask edge lost in places |

`amount` scales the irregularity (0 is a clean curve), `lobe` sets the
lobe width in units, `edge` the mask's soft edge in units. An open line's
outside is on its left. `body_of{spine=, widths=, limbs={{pts..., widths=}, ...}, blend=0.8, char=}`
is the silhouette of a skeleton of points and widths. Methods: `o:mask()`,
`o:below(bottom)`, `o:above()`, `o:band(width, taper)`, `o:inset(d)`,
`o:offset(d)`, `o:paint(brush, {...})`, `o:path(i)`, `o:paths()`,
`o:strokes()`, `o:at(t)`, `o:length()`, `o:corners()`.

## Drawing: pencil, chalk and eraser

```lua
h = pencil("2H")                  -- or pencil{grade="2H"}: 9H..H, F, HB, B..9B
c = chalk()                       -- black chalk
h:sketch(pts, {pressure=0.3})     -- a few light passes (passes=3, wander= units, smooth=true)
h:line(pts, {pressure={0.5, 0.7, 0.4}})    -- one line through the points (smooth=false keeps corners)
h:rule({120, 700}, {860, 180}, {pressure=0.3})  -- straight, against a ruler
h:hatch(m, {angle=-1.1, pressure=0.35})         -- short parallel strokes (spacing=, length=)
h:width()   h.worn   h:sharpen()  -- the point blunts as you draw
erase(pts, {strength=0.9, width=9})  -- a kneaded eraser along a path, or erase(mask, {strength=})
fix()                                -- fixative (or fix(mask)): the eraser no longer lifts it
drawing_guide()                      -- the drawn lines themselves, as a continuous mask
```

The point rides on the tops of the weave; pressure lets it reach into the
hollows. Soft leads lay darker, glossier lines, hard ones pale silver
lines. The eraser lifts most of a line, more from the tops than the
hollows, and leaves a ghost. Once paint has gone over the drawing it is
sealed: thin paint lets it show through, body paint hides it.

## Solids, light and space

These are scaffolds for shapes you give them: they answer where light
and shadow fall and what lies in front of what.

**Form.** Solids in canvas units, z toward you, lit by one light:

```lua
s = body.ellipsoid({400, 500, 0}, {120, 90, 80}):turn({400, 500, 0}, 0.3, 0.1, 0)
      :cut({400, 430, 0}, {-0.3, -1, 0.4}, 1, 3):rough(6, 120, 1)
k = body.block({650, 520, 0}, {160, 60, 90}, 3)       -- center, size, rounding
t = terrain{area={250, 200, 650, 500}, height=function(x, y) return 10 * math.sin(x / 60) end}
f = form{ {s, dist=0.3}, {k}, light={from={-1, -0.7}, front=0.5, ambient=0.2} }
f:value(x, y)   f:lit_at(x, y, soft)   f:part(x, y)   f:sample(x, y)   f:fall(x, y)   f:across(x, y)
f:lit{parts={1}, soft=0.12}   f:shadow{parts={1}}   f:silhouette{parts={1}}   f:edges{turn=0.8}   f:parts_mask{1}
work(f:lit{parts={1}}, {pile=p, angle=f:field("fall")})   -- field("fall"|"across"|"edge") for angle=
```

Solids combine with `s:union(o)` and `s:subtract(o)`; `body.half_space(at,
normal)` cuts. The light also takes `bounce`, `bounce_from`, `penumbra`,
`reach`, `thickness` and `across_parts`.

**World (reference).** A space in meters seen in perspective: a camera
over a supporting surface, one directional light, and the bodies you
place there. Its calls:

```lua
w = world{eye=<m>, fov=<degrees>}        -- camera height and field of view
s = w:spot(x, y)                         -- the surface seen at a canvas point (or w:spot_at(X, Z))
w, n = w:place(s, body.block(s:p(0, 0.5, 0), s:size(1, 1, 1), s:m(0.05)))   -- s:p, s:size, s:m: meters to units
v = w:view()                             -- trace once and keep it
v:bodies_mask{n}   v:shadows()   v:contact(0.25)   v:at(x, y)   v.form
w:to_ground(x, y)   w:project(X, Y, Z)   w:scale_at(Z)   w:height(x, y, meters)
w:shadow_angle(x, y)   w:sun_canvas()   w:ribbon(pts, width)   w:recede({X, Z}, {dX, dZ}, n)
```

`world{}` options, all optional: `view` (the canvas rectangle it
covers), `horizon` (the canvas y of eye level), `eye`, `fov`, `ground` (a
function `(X, Z)` giving the surface's height in meters; flat by
default), `water` (`{level=, ripple=}`: a level reflecting surface),
`sun` (`{azimuth=, elevation=}` in degrees: the light's direction;
azimuth 0 is straight ahead, -90 to the left, 180 behind the eye),
`visibility` and `backdrop`. View queries: `v:water()` and `v:mirror(x,
y)` (with `water`), `v:land()` and `v:sky()` (where the surface and the
space above it are seen). `w:proxy(s, body)` places a body that casts a
shadow but isn't seen. `w:aerial(Z)` and `aerial(dist, visibility)` give
how much air lies between the eye and a distance (0..1).

**Depth.** A view knows what lies behind what. `w = w:layer(name, mask,
depth)` registers a shape you paint by hand at a depth (meters, a spot, a
canvas point `{x, y}` where it stands on the ground, or `"ground"`); make
the view after it. Then `v:visible(x)`, `v:front(x)`, `v:behind(x)`,
`v:at_depth(m)`, `v:between(a, b)` and `v:seen(x, y)` are masks and
answers, and `v:cast_shadow{soft=, from=}` and `v:contact_shadow{reach=,
from=}` are shadows that fall off with distance from what casts them.
Things are named by body number, layer name, `"ground"`, `"water"`,
`"surface"`, `"sky"`, `"bodies"`, `"layers"`, a mask or a list. Passes take
`visible=` (only where that is seen), `behind=` (only where nothing in
front of it is) and `at=` (a depth in meters): strokes still overrun the
mask's own edges, never what is in front.

## Time

Painting takes time, and paint dries on the painting's clock. Only
painting operations (strokes, touches, passes, trips to the palette) and
`wait(minutes)` advance it. Real time between chunks does not: paint
doesn't dry while you think. `wait(minutes)` passes painting time at once;
it doesn't make you wait that many real minutes.

- **Hand time.** Every stroke, touch, pass and trip to the palette takes
  the time a hand takes to make it: a stroke by its length and the
  brush's width (broad sweeps are fast, fine lines slow per mm), a
  stipple touch about a third of a second, a dip into a pile on the
  palette 2.5 s, knifing a new pile 20 s. The paint ages while the hand
  works: a long pass is painted in slices of 15 minutes, and its first
  strokes are setting by the time the last go on.
- **`wait(minutes)`** lets time pass with your hand away from the canvas:
  minutes, hours or days (`wait(3 * 24 * 60)`), up to 10 years
  (5,259,600 minutes). It returns the time of day, e.g. `day 3, 14:20`
  (the painting was begun at 09:00 on day 1).
- **`drying(x, y)`** tells you what the paint there is like to the touch:
  `"open"` (workable: it blends and lifts), `"setting"` (stiff, barely
  blends), `"tacky"` (set; it grabs the brush) or `"dry"` (touch-dry).

Each film dries at its own pace, set by its pigments, its thickness and
its oil. Thin lean paint of fast-drying pigments is touch-dry in a day;
thick, oily paint of slow pigments can stay open for weeks. Wet paint
under a new stroke comes up into it; paint laid over dry paint sits on
top of it.

## Looking

`look` shows you the canvas as it is now, with wet paint as laid. A whole
view shows all of it, scaled down, like stepping back. A crop shows the
canvas at its full detail, 2.4 pixels to a canvas unit, and may be at most
500 units on either side. `crop: "x0,y0,x1,y1"` gives two opposite corners
in canvas units, not a position and width/height.

| `look` with | shows |
|---|---|
| nothing | the whole canvas, scaled down |
| `crop: "300,200,500,350"` | a window in canvas units, at full detail |
| `mode: "value"` | in grays |
| `mode: "squint"` | blurred, as through half-closed eyes |
| `mode: "mirror"` | flipped left to right |
| `mode: "value,squint"`, `size: 600` | modes combine; `size` sets the long side |
| `grid: true` | a squared grid in canvas units, labeled along the edges |
| `crop: "300,200,500,350"`, `grid: 10` | a window with a grid every 10 units |

The grid is drawn on the PNG only, never on the canvas, like the squares
ruled over a drawing to transfer it.

## How chunks behave

- **Globals persist, locals don't.** Each chunk is its own Lua chunk:
  `p = pile{...}` is there in later chunks, `local p = ...` is not.
- **Randomness is deterministic.** `math.random`, `rand` and `randn` are
  reseeded at the start of every chunk from the canvas's seed and the
  chunk's number, and passes pick their own seeds the same way (pass
  `seed=` to fix one). A replay paints the same thing, and a failed chunk
  doesn't shift the next one's randomness.
- **No OS access.** `io`, `os`, `debug`, `require`, `dofile`, `loadfile`
  and `collectgarbage` aren't there. `print` goes to the reply.
- **Lua 5.5.** Numbers are integers or floats (`7 // 2` is 3, `7 / 2` is
  3.5). Bitwise operators are built in; there is no `unpack` (use
  `table.unpack`). Loop variables are read-only. Don't write `global`
  declarations (one switches its chunk to strict mode).
- **`pairs` walks a table in the same order in every session and replay.**
  Tables keyed by strings, numbers and booleans walk in Lua's order; a
  table with any other key walks in a fixed order (booleans, numbers,
  strings, then other keys in the order they were made). For a big list,
  `ipairs` is faster.
- **No memory addresses.** A table, function or userdata without
  `__tostring` prints as `table: (hidden)` (its `__name` for the type),
  in `print`, `tostring`, `string.format`'s `%s` and errors alike, and
  `%p` is refused: an address differs from run to run, so a replay would
  print or choose differently.
- **Memory.** Masks and forms are large: keep big ones `local` when
  later chunks don't need them.

## The journal

`notes/journal.md` is your working journal. `note` appends an entry
stamped with the painting's time, such as `day 2, 09:40`. To revise what
is already there, call `note` with `replaces`, the exact passage to
change, and `text`, what takes its place. Writing a note doesn't advance
painting time.
