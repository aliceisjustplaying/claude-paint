# The easel

The easel is a live oil painting. Its `paint` tool runs Lua chunks one at
a time; `look` shows the canvas. Under it is a physical paint simulator: simulated
bristles carry wet paint over a primed linen canvas, the paint levels and
dries on a clock, and layers combine by Kubelka–Munk optics.

Three things hold for every session:

- **Every chunk that runs stays on the canvas.** There is no undo. To
  change something, paint over it, lift wet paint off with a brush or
  a rag or scrape it off with a knife.
- **A chunk that stops with an error changes nothing.** The canvas, your
  variables, the paint on your brushes and the clock are as they were
  before it, and it isn't written to the log.
- **The log is the painting.** Every chunk that ran is appended to
  `paintings/lua/painting.lua`, and replaying it paints the same canvas.

## Starting

The studio holds one painting, and the easel is open on it. Its tools:

| tool | what it does |
|---|---|
| `paint` | runs a chunk of Lua. The reply is what the chunk printed, the painting's current clock, then `ok` |
| `look` | shows you the canvas as it is now, or your palette (see [Looking](#looking)) |
| `note` | adds an entry to your journal (see [The journal](#the-journal)) |
| `status` | the canvas's setup |
| `log` | the painting so far: every chunk that ran, each after a line `--@ chunk` |

The first chunk is `canvas{}` (see [The canvas](#the-canvas)). `read` reads the files in this folder: your brief and
your notes.

Beside the painting is a scratch canvas to try things on (see [The scratch canvas](#the-scratch-canvas)): `paint`,
`look`, `status` and `log` work on it with `scratch: true`.

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
- `raw` (engine 3), instead of `ground`: leave the canvas raw, the bare
  cloth unprimed: `"cotton duck"` (creamy and absorbent) or `"linen"`
  (browner, it holds less), as Frankenthaler and Morris Louis left theirs
  for soak-staining (see `notes/research/soak_stain.md`). The cloth shows
  its colour and its weave. A brush works on it as on a primed canvas: its
  paint lies on the cloth. `soaked(x, y)` says in words what is in the
  cloth there (`"raw"`), and whether paint lies on it.
- A ground layer's `absorbent=true` (or 0..1) makes it a chalk and glue
  ground: it draws oil out of the paint laid straight on it until its pores
  are full, so thin paint there goes lean, quick to set and matte, while
  thick paint barely notices. The pigment packs from the ground up: the
  paint above keeps its oil, its flow and its gloss until the packed layer
  reaches the surface, and how far a thin film drains is its pigment's
  (fine pigments keep the oil they pack with and dry semi-matte; coarse
  ones, such as smalt, drain further and dry matte). Each box's tubes hold
  as much oil as its period ground them with. Paint that has dried over it
  seals it. An oil ground (the default) absorbs nothing and is semi-matte.

**Gloss.** Every dry surface is more or less glossy: oily paint (medium)
dries glossy, lean paint (blotted, or drawn out by an absorbent ground)
matte, a thin film shows the surface under it, a varnish makes all of it
glossy. A matte surface scatters the light its first surface reflects back
toward you, a faint veil of white over the colors that lifts the darks; a
glossy one sends it away. The looks and the saved picture show it.

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

`turps` thins the pile with that share of turpentine (0 to 0.9):
`pile({{"<tube>", <parts>}, turps=0.6})`. It flows on the brush and
evaporates as the paint is laid, so it leaves a film that much thinner, of
the paint's own body: the lean, quick lay-in and wash. (`thinner`, below,
is the other way to thin a pile: its solvent stays in the film for a while
and leaves over painting time.) Your tubes come ground in the oils your
box's colourmen used, some with a little wax (it dries slower and more
matte). `oil` grinds the pile in another: `"linseed"`, `"walnut"` (dries a
little slower) or `"poppy"` (dries slower still, yellows least).

`blot` is the opposite of medium: the paint laid out on blotting paper
first, which draws out that share of its own oil (0 to 0.5):
`pile({{"<tube>", <parts>}, blot=0.3})`. Blotted paint is leaner, a little
more opaque and much stiffer: it holds the ridges and furrows of the brush
as it dries. The paper draws no more than the pigment lets go: a paint
already ground stiff, or of a fine pigment that holds its oil, gives up
less than you ask. A pile takes medium or blot, not both.

**The palette board.** Every pile is a heap on the board beside the easel.

- `pile{..., name="sky"}` names it.
- `look --palette` (the `look` tool's `palette: true`) shows the board: each
  heap knifed out thick (its masstone and body) with a smear dragged from
  thick to thin across a black stripe (its tint as it thins, how much it
  hides), lit as in the gallery view. Judge a mix there before it touches the
  canvas.
- `p:add{{"<tube>", <parts>}, ..., medium=}` knifes more tube paint into a
  heap, in the units its recipe was given in (a pile knifed as
  `{"lead white", 4}, {"cerulean blue", 0.35}` takes `{"cerulean blue", 0.1}`
  as a tenth of a part more): look, adjust, look again. It returns the pile,
  its recipe with the added parts. The added paint is tube paint as the tubes
  come (in their oil, unthinned), so it dilutes the heap's turpentine, its
  `thinner` and any oil it was reground in;
  a scraped heap is knifed fresh first.
- `mix{{p1, <share>}, {p2, <share>}, ..., name=}` knifes heaps together into a
  new heap, as they are now.
- `palette{set_out={"<tube>", ...}}` sets out only those tubes, a limited
  palette: a pile of any other is an error.
- `palette{dirty=<0..1>}` keeps the board as dirty as painters do. A brush
  that comes to a heap carrying paint leaves a little of it there, more the
  fuller it is (a `b:reload` wipes it first), and the smears of the mixing
  area seep into each new heap, so the heaps drift toward each other through
  a sitting. `palette{clean=true}` scrapes the mixing area and skims the
  heaps. 0, the default, is a clean board: every heap stays as knifed.
- `palette()` returns a line about each heap: its recipe now, and how much of
  it is other paint.

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
| bone black | charred bone (carbon, calcium phosphate) | 0.9 | 0.7 | 1.1 | 0.9 |
| cobalt blue | cobalt aluminate | 0.55 | 0.6 | 0.8 | 2.2 |
| chrome yellow | lead chromate | 0.9 | 0.7 | 1.0 | 1.8 |
| Prussian blue | iron ferrocyanide | 0.35 | 0.45 | 3.0 | 2.4 |
| green earth | celadonite and glauconite clay | 0.2 | 0.35 | 0.3 | 0.8 |
| Rinmann's green | cobalt-zinc oxide | 0.35 | 0.5 | 0.4 | 1.4 |
| copper green | verdigris ground in oil | 0.25 | 0.4 | 1.0 | 1.6 |

Hiding is how much one coat of the tube paint hides what is under it (0
transparent, 1 opaque). Stiffness is the paint's body as it comes from the
tube (0 fluid, 1 stiff). Tinting strength is relative to an average
pigment. Drying is the rate relative to average paint (higher dries
faster); a pile dries at its tubes' rates mixed by volume. These numbers
are estimates from the pigment literature, not measurements.

### Thinner

```lua
w = pile{{"<tube>", <parts>}, thinner=0.5}          -- 0 (none) to 0.9
print(w)                         -- pile(<tube> <parts>; medium 0, thinner 0.5)
print(w.thinner)                 -- 0.5 (0.0 for a pile without it)
```

`thinner` is the share of solvent (turpentine, spirits) by volume knifed
into the pile, 0 to 0.9. It is not `medium`: medium is oil and stays in
the film; the solvent leaves it. What a pile is made of doesn't change
with thinner: its pigments, oil, body and drying rate are the paint's own.

What it does:

- A brushload of thinned paint is liquid, part paint and part solvent.
  One stroke of it lays only a thin film: each spot on the canvas takes
  at most a set thickness of liquid from one stroke, however many hairs
  or how many times the stroke passes over it. The more solvent, the
  thinner that film (about 6 µm at thinner 0.5). What the stroke can't
  lay stays in the brush, so a loaded brush goes a long way; another
  stroke over the same spot lays more.
- The solvent leaves over painting time, whether you wait or keep
  painting: in a thin film a few minutes, longer in a thicker one. The
  paint stays. Once it has gone, what is left is a thinner film
  of the paint itself, and more of what is under it shows through.
- While it is there the wet paint is more fluid: it levels and spreads a
  little, carrying its solvent with it. As the solvent goes, the paint
  firms up to its own body. The oil dries at its own rate all the while.
- Solvent has no color: the picture shows the paint alone.
- A brush or rag that lifts solvent-wet paint takes the solvent with it.

What it doesn't do (left out on purpose): the solvent doesn't evaporate
from the brush or the pile on the palette (a pile keeps its thinner), it
doesn't soak into the ground, and it doesn't dissolve paint that has set
or dried underneath. Solvent-wet paint comes up on a brush or rag no more
readily than the same paint without it. The thickness limits and the
evaporation times are estimates, not measurements.

Thinner is new with this engine: a painting started on an older one
doesn't have it.

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
to 1 (fully pressed); a stroke's `pressure` takes a number for constant
pressure or `{start, end}`. A touch's `pressure` takes a number.
`ramps` gives the press-down and lift-off fractions, `swell` pressure factors
along the stroke, `orient` `"across"`, `"along"` or a fixed angle.

Brushes are blunt unless given a `point` (0 blunt, 1 a full point). A
pointed round or rigger lays a hairline at light pressure and spreads to
its belly when pressed, so its width follows the pressure and a stroke
whose pressure falls to 0 ends in a point.

**Loading part of the brush.** `b:load(p, amount, {side=, share=, streak=})`
dips only part of it: `side` (-1 or 1) and `share` (0..1) put one edge of
the brush in the pile (a double-loaded brush: two paints side by side in
one stroke), `streak` (0..1) takes it up unevenly in bands a few hairs
wide. In `work`, `streak=` does this on every dip and
`second={pile=, load=, side=, share=, streak=}` dips part of the brush in a
second pile after the first.

**Gestures.** `b:gesture({{x, y, p}, ...}, {wobble=, orient=, ramps=, shake=, clip=})` is one
deliberate stroke along a smooth curve through the points, its pressure
following each point's `p` (0..1; a point without one takes its
neighbors'): pressed hard at a root and lifted to nothing at a tip, swelling
through a turn. A pointed brush (`point=`) widens as it is pressed, so its
mark swells from a hairline and tapers back to one with the pressure; a
blunt one narrows at a light touch too (to about a third), but has no point.
`wobble` (units, up to 100) lets the hand drift sideways. Use gestures for the marks
that carry the picture: the few decisive strokes a passage needs.

**Thick paint.** `lay` (a brush option) sets how much paint a full brush
lays down: 1 for an ordinary load, 4 to 16 for impasto. Stiff paint
(blotted, or a stiff tube with no medium) holds what the brush leaves: in
it the hairs gather into clumps that lay the stroke in ridges and furrows
and push walls up along its edges, the coarser the hair (`hair`) the
coarser the clumps. Fluid paint levels out.

**The knife.** `k = knife{width=<units>}` is a painting knife, its blade
that long. `k:load(p, amount)` picks up paint, `k:wipe()` cleans it.
`k:lay(points, {pressure={a, b}, angle=, lift=})` drags it along the points
with the blade across the path (or at a fixed `angle`): it rests on the
surface's high points and stands off them by less the harder it is pressed,
filling what lies under it to the blade's level in a slab with a flat top,
or over dry impasto catching only the ridges; wet paint standing above the
blade is cut off into its bead or pressed out at its ends in ridges, and
the share `lift` of the bead (0.1) stays where it lifts.
`k:scrape(points, {pressure=, angle=, lift=})` scrapes wet paint off down
to the blade (at full pressure, down to the dry paint) and keeps it on the
blade, but for the share `lift` (0) it leaves where it lifts.

**Spatter.** `b:spatter{at={x, y}, toward={dx, dy}, spread=, force=, clip=}`
flicks the loaded brush: the paint its hairs can't hold flies off in droplets
toward `toward` (its length is how far the paint carries, in units), in a cone
`spread` radians either side (0.45 by default), `force` 0..1 hard (0.6). Fluid
paint (medium, turpentine) flies readily; blotted or stiff tube paint barely
leaves the brush. A hard flick throws many small droplets, a gentle one fewer
and bigger; heavier droplets carry farther, and those that land at a slant
stretch along their flight. Each comes off one hair with that hair's paint (a
double-loaded brush spatters both colors), and lands in the wet layer like
any paint. It returns how many droplets landed; `b:fullness()` shows what is
left on the brush.

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
| `blend` | badger 40 | a clean blender, crossing passes top to bottom, inside the mask. It is wiped clean as it goes, so it lifts paint as well as moving it: one pass takes up about 15% of a thin wet film, three passes about 40% |

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
| `piles` | graded color: `{{p1, w1}, {p2, w2}, ...}`, each weight a number or `function(x, y)`; each dip takes a mix of the piles by their weights at the stroke (the brush dipped into neighboring piles), so color changes continuously across one passage, with no seams between masks |
| `scale_at` | the size of the marks across the area: a number or `function(x, y)` multiplying stroke length and brush width (smaller where things are far, larger near); the pass lays more strokes where they are smaller, so its coverage holds |
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
`below` selects larger y values, toward the bottom of the canvas;
`above` selects smaller y values, toward the top.

Numbers and randomness: `rand(a, b)`, `randn(mean, sd)`, `math.random`,
`lerp(a, b, t)`, `clamp(x, lo, hi)`, `smoothstep(a, b, x)`;
`noise{seed=, octaves=4, period=200, persistence=0.5, kind="fbm"|"ridged"|"billow", warp={period, amount, twice}, stretch={angle, k}}`
(call it as `n(x, y)` for about -1..1, `n:at01(x, y)` for 0..1; it can be
passed to `coverage=` or `load_at=`); `worley{seed=, period=, jitter=}`
(`c:at(x, y)` gives the distances to the nearest two cell points, how near
a cell wall, and a stable 0..1 per cell); `uneven(n, lo, hi, irregular,
clump, seed)` (n positions from lo to hi with uneven, clumped gaps).

The nested `warp` and `stretch` tables use positional values, not named
fields: for example, `noise{period=180, stretch={0, 3}}` stretches along
angle 0 by a factor of 3. `stretch={angle=0, k=3}` is not accepted.

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

## Drawing: pencil, chalk, pastel and eraser

```lua
h = pencil("2H")                  -- or pencil{grade="2H"}: 9H..H, F, HB, B..9B
c = chalk()                       -- black chalk
h:sketch(pts, {pressure=0.3})     -- a few light passes (passes=3, wander= units, smooth=true)
h:line(pts, {pressure={0.5, 0.7, 0.4}})    -- one line through the points (smooth=false keeps corners)
h:rule({120, 700}, {860, 180}, {pressure=0.3})  -- straight, against a ruler
h:hatch(m, {angle=-1.1, pressure=0.35})         -- short parallel strokes (spacing=, length=)
h:hatch(m, {pressure=0.6, graded=true})         -- the mask as a weight: each stroke pressed by its value there
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

**Pastel.** A pastel is a stick of pigment with a little gum and chalk:
colored, dry and soft. In a painting begun before engine 6:

```lua
p = pastel(pile{{"vermilion", 1}, {"lead white", 2}}, {soft=0.7})  -- the pile's color; soft 0 (hard) .. 1 (very soft)
p:line(pts, {pressure={0.6, 0.1}})    -- with the end of the stick (also sketch, hatch, rule, as a pencil)
p:side(pts, {width=20, pressure=0.4}) -- laid on its side: a broad band (width in units, about 12 mm by default)
q = pastel(pile{{"lead white", 1}}, {soft=0.2, point=0.6})  -- a pastel pencil: a fine point (mm) that keeps its point
smudge(mask or pts, {strength=0.6, width=, reach=})   -- a finger or stump rubbed over it
fix(mask)                             -- fixative between layers
```

- The stick's color is the pile's pigments as they look dry, a little
  paler and grayer than the same pile in oil; the pile's medium and
  thinner don't matter. Tints are made the way the sticks were, with
  white in the pile (lead white stands in for the chalk).
- Pastel lays on itself. A stroke covers a share of what is under it,
  earlier pastel included, so colors laid across each other mix in the
  eye, stroke by stroke, and the last one laid is on top. A soft stick
  pressed hard covers almost completely; a light touch catches only the
  tops of the tooth and leaves the hollows; the edge of a mark, where the
  stick presses least, breaks up in the tooth.
- Each stroke also fills the tooth, and a full tooth takes little more: the
  paper refuses more pastel. `fix` binds what is there (the eraser no longer
  lifts it, the stump hardly moves it), gives back more than half the tooth so more can
  go on top, and darkens the layer a little.
- `p:side` lays the stick flat: wide, riding on the tops of the tooth
  (speckled at a light touch), and it doesn't wear the end.
- `smudge` drags loose pastel within `reach` units (about 2 mm) together,
  averaging its colors by how much of each is there, presses it into the
  hollows (it covers more) and packs it, so it takes less pastel afterwards.
  Fixed pastel moves a quarter as readily as loose; pastel painted over or
  under wet paint doesn't move.
- Pastel skips wet paint, as graphite does; let the paint set first. Paint
  laid over pastel seals it, as it seals a drawing.

The numbers (how much a stroke lays, how fast the tooth fills, what dry
pigment looks like) are estimates, not measurements.

**Pastel in engine 6.** From engine 6 (a painting begun now) a pastel is a
stick: a solid that rests on the tooth, wears and sheds what it abrades.
An older painting's pastel draws as above.

```lua
p = pastel(pile{{"vermilion", 1}, {"lead white", 2}}, {kind="soft"})   -- a round stick, Ø12 mm (diameter=)
h = pastel(pile{{"bone black", 1}}, {kind="hard"})                     -- a square stick, 6.35 mm
q = pastel(pile{{"lead white", 1}}, {kind="pencil"})                   -- a pastel pencil's core, sharpened
p:stroke(pts, {force=2, alt=60, azimuth=45, roll=0, speed=80})          -- one stroke; each may be a list along pts
p:side(pts, {force=1.5})       -- laid flat across the stroke: a piece as wide as it is long (p:snap(mm) breaks one)
p:roll(30)                     -- turn it in the fingers: the next stroke meets a fresh edge
p:line(pts, {pressure=0.5})    -- a pencil's call: pressure as force (5 N at 1), held at 60°
smudge(pts, {force=1, pad=})   -- a finger drawn along a path (pad: a stump's few mm²)
fix(mask)                      -- binds what is there, and darkens it a little
feel(x, y)                     -- what a fingertip feels there
```

- **One stroke at a time.** `stroke` is a single movement of the hand:
  `force` in newtons (a feathered touch 0.1–0.5, normal 0.5–2, heavy 2–5),
  `alt` the stick's angle to the paper in degrees (upright 90, on its side 0),
  `azimuth` where its upper end points (degrees: 0 to the right, 90 down the
  canvas; a right hand holds it at about 45), `roll` turned about its axis,
  `speed` in mm/s. Each is a number or a list along the points. The hand
  lands and lifts as a hand does: its force comes up from nothing over about
  50 ms of the stroke and falls back over about 40 ms.
- **The stick rests on the tooth.** Under the force it sinks into the paper's
  micro-relief until the material it meets carries the force: the paper gives
  a little (it is compressible), and the soft pastel yields on the tops it
  touches. A light stroke touches only the highest fibres and leaves the
  paper's grain broken through the mark; a heavy one reaches into the pores.
- **It wears.** It lays what it abrades, in proportion to the force and the
  distance (Archard's law), and wears flat against the paper: a stick held
  one way grows a facet and draws wider as it does; rolled or tilted it cuts a
  new, sharp-edged one; a new stick's first strokes are narrow. `print(p)`
  says how many facets it has and how long it is.
- **The tooth fills.** Loose pastel in the pores is a third body the stick
  slides on: the more there is, the less a stroke lays, until the paper
  refuses it. Fixative binds it into a rough crust that files the stick again,
  so a fixed passage takes more pastel; and darkens it a little, as the resin
  wets the particles.
- **The finger** picks up loose pastel and lays it down along its path as fine
  grains pressed into the pores: colours drag into each other and the passage
  covers more for its thickness. It leaves fixed pastel where it is.
- **Colour.** A stick is its pigments as they look dry: absorption as in oil,
  scattering raised by the contrast between each pigment and air (much for
  ultramarine, Prussian blue, the lakes and the earths' clays; little for
  vermilion and the chromes). Dry blacks are greyer than in oil.
- **Feel.** `feel(x, y)` answers in words: the surface (paper, paint and how
  far it has set) and the pastel in the tooth (loose or fixed; a little, the
  tooth taking it, half full and more, full).

**Pastel tools in engine 7.** Six more, each a physical process
(paint's sheet.rs; the research behind them in notes/research/):

```lua
lay_sheet(m, {grammage=120, tone={{"lead white", 3}}})   -- a sheet of paper laid over the mask
lift_sheet()                                              -- taken away, with what it caught
b:dust(pts, {pressure=0.6, tip=50})   -- a dry brush over pastel: lifts what its tips reach
k:scrape(pts, {pressure=, edge=100})  -- a knife over pastel: sinks into the paper, takes the loose
blow(x, y, {distance=50, speed=12})   -- a puff of air (mm from the paper; m/s from the lips)
tap({g=100})                          -- the board's edge struck on the table (an upright sheet: g=1)
```

- **The sheet** keeps what it covers clean. It lies a paper's thickness
  above the picture, so a stick rests on it like any surface; near its
  edge the stick bridges from sheet to picture, and the edge left behind is
  as sharp as the stick's angle and force make it (a stick laid flat leaves
  a wider margin than its point). Strokes and crumbs that land on the sheet
  stay on it; the finger, the eraser and fixative don't reach under it. Lay
  and lift it in one chunk. While it is down, only pastel, the finger, a dry
  brush, the eraser and fixative work: wet paint and graphite aren't modelled
  under a sheet and refuse.
- **A dry brush** (`b:dust`, the brush wiped clean) lifts loose pastel as
  far as its bristles' tips reach into the tooth and no deeper: a hog's tip
  (about 50 µm) reaches only the top micrometres of the pores, so what fills
  them stays as a ghost; a finer `tip` reaches further. Fixed pastel stays.
  The bristles keep what they lift until they are full (a brush a centimetre
  wide holds a few milligrams; `b:wipe(1)` empties it); then the rest is pushed
  ahead and left as a ridge of crumbs where the brush lifts. Over a
  speckled passage it smears more than it cleans.
- **The knife** (`k:scrape`) over pastel: the hand's force (1–5 N by
  `pressure`) bears along the blade's edge, so a wide palette knife presses
  lightly and a short edge hard. The edge sinks into the paper as far as
  the sheet's compression and its surface's stiffness allow, and takes the
  loose pastel above that depth; a stiff blade rides the paper's high spots
  and bridges its low ones, so a scrape is mottled. Pressed hard enough it
  sets the paper: burnished, flatter, the pores shallower (less tooth). Its
  drag can tear fibres loose: raised nap, more tooth there. `edge` is the
  edge's radius in µm (a painting knife 20–500; a scalpel about 1): a
  scalpel on a short edge cuts, shaving a fibre layer and taking even fixed
  pastel. Otherwise fixed pastel stays.
- **A puff** (`blow`) shears the surface in a ring around where it is aimed
  (nothing at the very centre): loose crumbs heaped on the tooth go easily;
  the pores' fine grains only under a hard blow close in, and only their
  top few micrometres. Most crumbs it lifts roll outward and settle in a
  broad ring past where it stops lifting them; some hop further, up to a
  few centimetres; a little of the fine grains settles thinly further out,
  and most are carried off. Crumbs landing on a paper mask go with it.
- **A tap** (`tap`) shakes off the crumbs heavy enough to beat their hold:
  heaps and ridges go, the grains in the pores stay, and fixed pastel stays.
- **Holding a stick up to the picture:** `look` with `hold: "<stick>"` and
  `at: "x,y"` (see Looking).

**Paper (engine 6).** `canvas{size=, aspect=, paper={...}}` instead of
`linen=`: a sheet laid from its fibres (a random fibre network, with flocs),
pressed on a felt (its grain) and calendered. Every key has a default (a
160 g/m² cotton drawing paper with a felt grain):

| key | |
|---|---|
| `tone` | the sheet's colour: parts of tubes, as a ground's paste (`{{"cobalt blue", 1}, {"lead white", 3}}`), seen dry |
| `grammage` | g/m² (90–250 for drawing papers) |
| `fibre`, `fibre_width`, `thickness`, `coarseness` | fibre length (mm), width (µm), collapsed thickness (µm), mass per length (mg/m) |
| `porosity`, `floc`, `floc_size`, `press`, `calender` | the sheet's pore share; the share of fibres in flocs and their size (mm); how much of the thickness variation the press turns into density (0.5–0.8); calendering 0..1 |
| `felt` | `{cell=mm, depth=µm}`, or `false` for none |
| `laid` | `{per_cm=, chain=mm, deficit=}`: a laid mould's wires |
| `absorbent`, `stiffness` | how much of the pores take oil (0..1); the sheet's give under a stick (MPa) |

A ground is optional on paper. Thin oil paint on it goes lean and matte, as
on an absorbent ground. The numbers come from paper physics and, where no
measurement of artists' papers exists, are estimates
(notes/research/paper_surface.md).

## The rag

```lua
r = rag()                                -- a clean cotton rag bunched into a pad, about 40 mm across
r = rag{width=<units>}
r:wipe(m, {pressure=0.5, angle=<radians>, passes=1, refold=<load>})   -- wiped over a mask
r:wipe(pts, {pressure={0.4, 0.8}})       -- one wipe along a path
r:blot(x, y, {pressure=0.6})             -- pressed straight down and lifted off
r:refold()                               -- a cleaner part of the cloth turned outward; retains dampness
r:dip(0.5)                               -- the part in use dipped into spirits, 0..1
print(r)                                 -- rag(width <units>, load <0..1>); also r.load, r.soaked, r.damp, r.fold
```

A rag lifts open paint. The cloth rests on the tops of the weave and of
the paint, and reaches into the hollows as it is pressed harder
(`pressure` 0..1): paint comes off the tops first and stays longer in the
hollows. The last of a film comes away more and more slowly, so a dry rag
leaves a pale tint of the color that more wiping thins. Toward its rim the
pad presses lightly and reaches only the tops of the weave, so the edge of
a wipe shows the weave. Its folds and creases touch unevenly, so a wipe
leaves streaks along its path and a blot a crumpled patch. During a wipe,
a little freshly lifted paint smears back along the lightly pressed edges
and trailing end.

Fresh paint at the face's surface can smear into the next wipe too. It
gradually soaks into the cloth; refolding turns that paint inward.

A rag dipped in spirits (`r:dip`, 0..1) lifts wet paint more readily and
reaches farther into the hollows. It works gradually: wipe after wipe
takes the paint nearly to the ground. Its folds still leave streaks.
Dampness (`r.damp`) halves every three minutes of painting time. Refolding
retains the cloth's dampness; another dip adds spirits.

What it lifts soaks into the cloth. `r.load` is how loaded the part in use
is (0 clean, 1 full): the more loaded, the less it lifts. `r:refold()`
turns a cleaner part outward, but no part is cleaner than what has soaked
through the whole cloth (`r.soaked`, 0..1) leaves it; `rag()` takes a
fresh one. These numbers can be read, not set. Over a mask, `refold=`
refolds whenever the load passes that value.

Over a mask, `wipe` lays wipes side by side across it in `angle`'s
direction, back and forth, each starting and stopping a little inside the
mask. The pad's rim reaches the mask's edge, and in places past it:
the rag isn't clipped to the mask. `passes` goes over it again, each time
a little turned. `seed=` fixes the cloth's folds.

**What it can't reach.** It lifts only open paint, and less the further
the paint has set: paint that is `"setting"` comes away slowly, and paint
past its gel point (`"tacky"` or `"dry"` under `drying(x, y)`) doesn't come
off at all. Paint laid over a set film lifts down to that film.

Hand time: a wipe takes the time to bring the pad down and drag it, at
about 150 mm a second; a blot about a second; a refold 3 seconds; a dip
2.5 seconds; a fresh rag 5 seconds.

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

Every successful paint reply reports the current painting time automatically,
before `ok`.

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
  It reads the top film at that point: where no wet paint lies there (a
  gap between strokes, a wiped spot), it reads the layer beneath.

Each film dries at its own pace, set by its pigments, its thickness and
its oil. It is open for the first 30% of its time to touch-dry, setting
until 60%, then tacky until it is touch-dry. A stroke of lead white or raw
umber from a loaded broad brush is open for 12 to 14 hours, tacky after
about a day and touch-dry in under two days. Cobalt blue, Prussian blue
and burnt sienna are about as fast. Raw sienna, ochres, earths, cadmiums,
ultramarine and bone black are touch-dry in 3½ to 5 days, madder in about 11.
Thinner paint dries sooner and thicker paint later: twice a stroke's
thickness takes 1.6 times as long. Thick, oily paint of slow pigments can stay open for weeks. Wet paint
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
| `mode: "relief"` | under a raking light from the upper left, so the paint's ridges, furrows and slabs show (wet paint shines) |
| `mode: "gallery"` | as the picture hangs: lit from above and a little left at 55°, so impasto models softly |
| `mode: "relief"`, `light: "45,15"` | the light from that azimuth (degrees: 0 from the right, 90 from the top) and elevation (0 to 90); the lower the light, the harsher; at 0 the lamp lies in the canvas's plane: flat paint gets only the room's light, ridges are lit on the lamp side; at 90 it is overhead and casts no shadow |
| `mode: "value,squint"`, `size: 600` | modes combine; `size` sets the long side |
| `grid: true` | a squared grid in canvas units, labeled along the edges |
| `crop: "300,200,500,350"`, `grid: 10` | a window with a grid every 10 units |

The grid is drawn on the PNG only, never on the canvas, like the squares
ruled over a drawing to transfer it.

`look` with `palette: true` shows the palette board instead of the canvas:
each live heap's masstone and body paint, with a smear dragged across a
black stripe. It shows the heap's paint before the unevenness of a
brushload. It takes no
other option, and nothing on the canvas or the clock changes.

| `look` with | shows |
|---|---|
| `survey: true` | the whole canvas at full detail, as several tiles of at most 500 units (2 × 2 for most canvases, 2 × 1 for one twice as wide as high), each a separate image; modes apply (`mode: "gallery"`) |
| `compare: "<an earlier look's path>"` | that earlier image on the left and the current view on the right, at the same height. The right is the view the other options ask for (`size: 800` without `crop` or `size`); the earlier picture is only resized to its height, so repeat the earlier look's crop, modes, light and grid options to compare the same view |
| `ref: "<a picture in the studio>"` | the motif (a photograph, a study) pinned beside the easel: fitted to the canvas's shape, on the left, beside the same view of the canvas: the same crop, size, mode and grid |
| `hold: "<a knife's or a pile's name>"`, `at: "x,y"` | (speculative) the loaded knife held up to the canvas: the passage around the point (240 units square, clipped at the canvas's edges, or your `crop`) with the blade's end at it, the paint thick on the steel, seen in the same light and mode as the passage (`mode` value, squint, relief or gallery, and `light`; not mirror, `grid`, `size`, `palette`, `survey` or `compare`). A knife shows what is on it, as full as it is up to a full load; a pile, a fresh full load. It shows the paint on the knife, not how it would look laid |
| `hold: "<a pastel stick>"`, `at: "x,y"`, `pose: "force,alt,azimuth"` or `side: "<direction>"` | (engine 6 on) the stick held there, seen from above over the passage (120 units around the point, or your `crop`): its low end as a light shadow, where it rests on the tooth at that force in its colour, and (engine 7) the band where its crumbs would settle, tinted; `side` lays it flat across a stroke going that way. The reply says how much it touches and how much its crumbs would reach, in mm². The stick can be a field of a table (`P.glow`). It shows where a stroke from there would lay, not what it would leave |

A survey can arrive in a partial batch to fit the painter's image budget.
The reply lists the remaining tile paths; read them one at a time to inspect
the rest of the canvas.

A whole view defaults to 1000 pixels on its long side (`size` up to 1600),
about 42% of a 2400-pixel canvas's width: small marks, beads of paint and
stray strokes can be hard to see in it. Survey the canvas after
each campaign, and compare before and after.

## How chunks behave

- **Option types.** Each option takes the type shown in this guide.
  Options documented as `function(x, y)` accept a Lua function; numeric
  options take the computed number.
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
  declarations (one switches its chunk to strict mode). There is no
  `math.atan2`: `math.atan(y, x)` takes two arguments. `%d` in
  `string.format` needs an integer (`7.5` is an error): use `%.0f` or
  `math.floor(x)`. Lists and tables are written with braces:
  `{1, 2}`, `{x = 1}`, never `[1, 2]` or `{x: 1}`.
- **`pairs` walks a table in the same order in every session and replay.**
  Every table walks in a fixed order: booleans, numbers, strings (each
  ascending), then other keys in the order they were made. (Paintings
  begun before engine 3 walk tables keyed by strings, numbers and booleans
  in Lua's order.) For a big list, `ipairs` is faster.
- **No memory addresses.** A table, function or userdata without
  `__tostring` prints as `table: (hidden)` (its `__name` for the type),
  in `print`, `tostring`, `string.format`'s `%s` and errors alike, and
  `%p` is refused: an address differs from run to run, so a replay would
  print or choose differently.
- **Memory.** Masks and forms are large: keep big ones `local` when
  later chunks don't need them.

## The scratch canvas

Beside the painting stands a scratch canvas for trying out a mix, a stroke, a glaze or a
drying time before it goes on the painting. `paint`, `look`, `status` and `log` work on it
with `scratch: true`; without it they work on the painting.

- The first chunk painted on it sets it up as the painting's canvas was set up: the same
  size, linen and ground. The painting needs its own `canvas{}` first.
- It is a separate canvas: its own palette, brushes, knives, rags and variables (a pile
  knifed for the painting has to be knifed again here), and its own log,
  `paintings/lua/scratch.lua`. Nothing done on it reaches the painting.
- It keeps its own clock. Time spent or waited there (`wait`, `dry()`, `glaze()`) doesn't
  pass for the painting, and the painting's time doesn't pass for it.
- To use what worked, paint it on the painting: the same pile, the same handling.
- `new_scratch: true` (with `scratch: true`) puts the scratch canvas aside and starts a
  fresh one before the chunk runs. The old log stays, as `paintings/lua/scratch-1.lua`, and so on.
- A trial judges a passage only as far as it repeats that passage's conditions: lay the
  underlayer you mean to work over, and let it dry as long as it has on the painting.

## The journal

`notes/journal.md` is your working journal. `note` appends an entry
stamped with the painting's time, such as `day 2, 09:40`. To revise what
is already there, call `note` with `replaces`, the exact passage to
change, and `text`, what takes its place. Writing a note doesn't advance
painting time.
