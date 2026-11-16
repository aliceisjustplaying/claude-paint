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
  before it, and it isn't written to the log. The one exception is a fresh
  scratch canvas: see [The scratch canvas](#the-scratch-canvas).
- **The log is the painting.** Every chunk that ran is appended to
  `paintings/lua/painting.lua`, and replaying it paints the same canvas.

## Starting

The studio holds one painting, and the easel is open on it. When the easel
opens on a painting that has a log, it rebuilds the painting by replaying
the log; until it has finished, `status` answers how many of its chunks are
done. Its tools:

| tool | what it does |
|---|---|
| `paint` | runs a chunk of Lua. The reply is what the chunk printed, the painting's current clock, then `ok` |
| `look` | shows you the canvas as it is now, or your palette (see [Looking](#looking)) |
| `probe` | gives the paint at a point (`at: "x,y"`) or averaged over an area (`crop: "x0,y0,x1,y1"`) in numbers: its color and value from 0 to 255 without the gallery's light, the color as a look without a mode shows it; the same two in the gallery's light, the color as mode `gallery` shows it and the value as mode `value` shows it; and whether it is open, setting, tacky or dry; it can't tell which piles the paint came from |
| `note` | adds an entry to your journal (see [The journal](#the-journal)) |
| `status` | the canvas's setup; while a chunk is still running, how far it has got; while the easel rebuilds the painting from its log, how many of its chunks are done |
| `log` | the painting so far: every chunk that ran, each after a line `--@ chunk`; with `find: "<text>"`, only the lines that hold that text, each with its line number and the two lines before and after it |
| `read` | reads a file in this folder, such as your brief or your notes; on a folder, lists what is in it |
| `reference` | searches the reference library or reads from it: `query: "<words>"` gives the `id` and title of each entry that holds all the words, in any case; `id: "<id>"` gives that entry's text a page at a time, with `page` to choose one and `next_page` naming the next (null on the last) |
| `reference_list` | lists the reference library: each entry's `id` and title |

The first chunk is `canvas{}` (see [The canvas](#the-canvas)). `read` reads the files in this folder: your brief and
your notes. `read` on a folder lists what is in it.

Beside the painting is a scratch canvas to try things on (see [The scratch canvas](#the-scratch-canvas)): `paint`,
`look`, `probe`, `status` and `log` work on it with `scratch: true`.

In the examples below, `<tube>` stands for a name from the tube box and
`<parts>` for a number of parts you choose. Other numbers in the examples
show how a call is written; the values are yours to choose.

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
- `paper`, instead of `linen`: a sheet of paper (below).
- `ground`: the preparation layers, bottom first. Each is a paste mixed
  from tubes in parts (as a pile is, below), a thickness in µm (5 to 400)
  and how it is put on: `"knife"` (levels the weave; `texture` 0..1 is the
  knife's waviness), `"roller"` (a fine even texture) or `"brush"` (laid in
  crossing strokes, its striations stay). The ground is dry when painting
  starts.
- `seed`: the randomness of the linen, the ground and everything after.
- `raw` (engine 3), instead of `ground`: leave the canvas raw, the bare
  cloth unprimed: `"cotton duck"` (creamy and absorbent) or `"linen"`
  (browner, it holds less). The cloth shows
  its color and its weave. A brush works on it as on a primed canvas: its
  paint lies on the cloth. `soaked(x, y)` says in words what is in the
  cloth there (`"raw"`), and whether paint lies on it.
- A ground layer's `absorbent=true` (or 0..1) makes it a chalk and glue
  ground: it draws oil out of the paint laid straight on it until its pores
  are full, so thin paint there goes lean, quick to set and matte, while
  thick paint barely notices. The pigment packs from the ground up: the
  paint above keeps its oil, its flow and its gloss until the packed layer
  reaches the surface. How far a thin film drains depends on its pigment:
  fine pigments keep the oil they pack with and dry semi-matte; coarse
  ones, such as smalt, drain further and dry matte. Each tube holds as
  much oil as paint of its day was ground with. Paint that has dried over
  the ground seals it. An oil ground (the default) absorbs nothing and is
  semi-matte. (A painting whose log's head has no `--@ drain` line keeps
  the older ground: it draws oil from the paint just laid, leaving it
  stiffer.)

**Paper.** `canvas{size=, aspect=, paper={...}}` instead of `linen=`: a
sheet laid from its fibers (a random fiber network, with flocs), pressed
on a felt (its grain) and calendered. Every key has a default (a 160 g/m²
cotton drawing paper with a felt grain):

| key | |
|---|---|
| `tone` | the sheet's color: parts of tubes, as a ground's paste (`{{"cobalt blue", 1}, {"lead white", 3}}`), seen dry |
| `grammage` | g/m² (90–250 for drawing papers) |
| `fiber`, `fiber_width`, `thickness`, `coarseness` | fiber length (mm), width (µm), collapsed thickness (µm), mass per length (mg/m) |
| `porosity`, `floc`, `floc_size`, `press`, `calender` | the sheet's pore share; the share of fibers in flocs and their size (mm); how much of the thickness variation the press turns into density (0.5–0.8); calendering 0..1 |
| `felt` | `{cell=mm, depth=µm}`, or `false` for none |
| `laid` | `{per_cm=, chain=mm, deficit=}`: a laid mold's wires |
| `absorbent`, `stiffness` | how much of the pores take a liquid, oil or water (0..1); the sheet's give under a stick (MPa) |
| `grit` | a gritted ground: `{size=µm, cover=0.3..1, binder="acrylic"\|"glue"}`, `true` for a P400 sanded paper (35 µm, cover 0.6, acrylic), or `false` for none |

**Sanded paper.** `grit=` coats the sheet with grains held in a binder,
as a sanded pastel paper or board is, or a ground of pumice in glue or
gesso painted onto paper. `size` is the grains' median size (by grade:
P240 58 µm, P320 46, P400 35, P600 26, P800 22). `cover` is the share of the
sheet the grains cover, seen from above (an open coat 0.5–0.7; a closed one
holds a little less). The binder is `"acrylic"` (modern sanded papers, or
pumice in acrylic gesso) or `"glue"` (glue or starch, the older pumice
grounds; leaner, so more of the coat is open). The tooth is then the open
room between the grains, under the tallest ones the stick rests on: at P400
4 times a drawing paper's in acrylic and 5.5 times in glue, more on the
coarse grades (P240 7–9 times), less on the fine (P800 2.5–3.5). The hard,
angular grains also wear a stick about three times as fast ("sanded paper
eats pastel"), so at the same force it fills in a few more strokes than
paper does. Laid with a lighter hand, a pass at a time as thick as one on
fresh paper, it takes 10–15 layers where drawing paper goes slick after
two or three. The sheet's grain shows through the coat. An acrylic coat seals the paper
against oil, so only the coat's own pores take it; a glue coat lets oil
through to the paper. A ground laid over the grit fills its tooth as thick
paint would. A knife rides the grains: it neither burnishes the tooth nor
raises a nap. Pumice grounds are 18th century and on; pumice papers were
sold by about 1890; acrylic sanded papers are late 20th century. Sources,
and which numbers are estimates: notes/research/sanded_paper.md.
| `cobb` | with pastel and water (below): the sheet's sizing, the g/m² of water it takes up in its first minute (its Cobb60, 5 to 200; 25 by default, a sized drawing paper), or `false` for an unsized sheet, which drinks water at once |

A ground is optional on paper. Thin oil paint on it goes lean and matte, as
on an absorbent ground. The numbers come from paper physics and, where no
measurement of artists' papers exists, are estimates. Paper is there in a painting begun since
it came (its log's head says `--@ paper`); an older painting replays as it
was painted, and `paper=` in it is refused. Grit is there in a painting
begun since it came (its log's head also says `--@ grit`); in an older
painting `grit=` is refused as an unknown key.

**Named sheets and mounts.** A sheet may be one of Cassatt's and her
circle's by name: `paper="tan wove"`, or `paper={preset="tan wove", ...}`
with keys of your own, which override the preset's (`laid=false` takes a
laid sheet's wires away). A preset sets the dials below and the tone; a
preset's tone is mixed from the catalog's tubes, whatever your box. The
colored papers have faded in museums (many blue sheets are tan now), so
these are the sheets as estimated new; sources and estimates in
notes/research/cassatt_papers.md.

| `preset` | grammage | fiber (mm) | felt | laid | calender | tone (parts) |
|---|---|---|---|---|---|---|
| `"blue wove"` | 130 | 1.6 | 0.7 mm, 14 µm | | 0.25 | lead white 1, ultramarine blue 1, bone black 0.45 |
| `"gray-blue wove"` | 130 | 1.6 | 0.7 mm, 14 µm | | 0.25 | lead white 1, ultramarine blue 1, viridian 1.5, red earth 1.5 |
| `"tan wove"` | 130 | 1.6 | 0.7 mm, 14 µm | | 0.25 | yellow ochre 0.2, bitumen 1, vine black 0.23 |
| `"buff wove"` | 130 | 1.6 | 0.7 mm, 14 µm | | 0.25 | lead white 1, yellow ochre 2.2, raw umber 3, Prussian blue 0.02 |
| `"green wove"` | 130 | 1.6 | 0.7 mm, 14 µm | | 0.25 | lead white 1, yellow ochre 0.65, viridian 0.45, bone black 0.45 |
| `"brown wove"` | 150 | 1.4 (coarseness 0.2, floc 0.6) | 0.8 mm, 18 µm | | 0.25 | raw sienna 1, bitumen 3 |
| `"cream laid"` | 110 | 2.2 | 0.8 mm, 15 µm | 9 per cm, chains 25 mm, deficit 0.15 | 0.2 | lead white 1, yellow ochre 0.15, raw sienna 0.02, green earth 3 |
| `"japan paper"` | 45 | 10 (width 30 µm, coarseness 0.1, porosity 0.6, floc 0.15) | none | | 0.6 | lead white 1, yellow ochre 0.1, raw sienna 0.1, viridian 0.02 |

Every other dial is the drawing paper's (fiber width 25 µm, thickness
6 µm, coarseness 0.16, porosity 0.5, floc 0.5 of 1.5 mm, press 0.65,
absorbent 0.8, stiffness 39). Japan paper is thin, long-fibered and
smooth: the sheet for counterproofs.

`mount=` lays the sheet down on something, which gives under a stick with
the sheet (`stiffness` stays the sheet's own; the mount's give adds to it):

- no mount: a sheet pinned on a hard drawing board, all but rigid behind it
- `"board"`: laid down on pasteboard or millboard; about ten times softer
  than the sheet alone
- `"canvas"`: laid down on canvas on a firm bed; the cloth compacts under
  the stick, softer still
- `"strainer"`: on canvas tacked to a wooden strainer. Over the bars (the
  outer 50 mm of the picture) it is as firm as on canvas. Between them the
  sheet and canvas drum: they soften over the first centimeter or so in
  from a bar, then give the same over the rest of the bay, whatever its
  size (the whole bay sinks millimeters under the hand, but that carries
  the stick and the surface round it down together).
  `mount={"strainer", bar=30, cross=1}` sets the bars' width (mm, 5–150)
  and a number of crossbars (0–8), as wide as the bars, which divide the
  longer side into equal bays and are as firm as the bars

Each give is derived from measured moduli under a worn stick's full facet
(notes/research/cassatt_papers.md §1). A softer bed spreads the stick's
contact wider, so less of it rides on pastel the stroke has just laid in
the tooth and the same stroke lays a little more: on board about 6% more
than on a hard drawing board, on canvas about 12%, and in a strainer's bay
about as on canvas (past canvas it levels off). It never lays more than the
stick wears, which is the same on any bed
(Archard). A firm bed's marks are crisper (a narrower contact), which may
be the "bite" painters describe.

Named sheets and mounts are there in a painting begun since they came (its
log's head says `--@ paper-presets`); in an older painting `preset=` and
`mount=` are unknown keys and a sheet's name alone is refused.

**Gloss.** Every dry surface is more or less glossy: oily paint (medium)
dries glossy, lean paint (blotted, or drawn out by an absorbent ground)
matte, a thin film shows the surface under it, a varnish makes all of it
glossy. A matte surface scatters the light its first surface reflects back
toward you, a faint veil of white over the colors that lifts the darks; a
glossy one sends it away. The looks show it.

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
medium mixed in (0, as from the tube, to 0.95). Only the parts'
proportions count: `{{"<tube A>", 4}, {"<tube B>", 1}}` and
`{{"<tube A>", 0.4}, {"<tube B>", 0.1}}` knife the same paint. To thin it,
raise `medium`, `turps` or `thinner`; to lay less of it, lower the `load`.
Medium makes the paint more
transparent and more fluid, and slower to dry. It is added oil only. What a pile looks like is
what its pigments make together, thick or thin, over what is already on
the canvas; you find out by painting with it and looking. A pile mixed by
hand is a little uneven: each brushload takes slightly different
proportions (about 6%). The palette has room for 16 piles. A 17th is an
error that lists the piles on the palette, oldest first; nothing is scraped
off to make room. (A painting whose log has no `--@ full-palette-refuses`
line scrapes the oldest off instead.)

`turps` thins the pile with that share of turpentine (0 to 0.9):
`pile({{"<tube>", <parts>}, turps=0.6})`. It flows on the brush and
evaporates as the paint is laid, so it leaves a film that much thinner, of
the paint's own body: the lean, quick lay-in and wash. (`thinner`, below,
is the other way to thin a pile: its solvent stays in the film for a while
and leaves over painting time.) Your tubes come ground in the oils your
box's colormen used, some with a little wax (it dries slower and more
matte). `oil` grinds the pile's pigments again in another, without wax:
`"linseed"`, `"walnut"` (dries a little slower) or `"poppy"` (dries slower
still, yellows least).

`blot` is the opposite of medium: the paint laid out on blotting paper
first, which draws out that share of its own oil (0 to 0.5):
`pile({{"<tube>", <parts>}, blot=0.3})`. Blotted paint is leaner, a little
more opaque and much stiffer: it holds the ridges and furrows of the brush
as it dries. The paper draws no more than the pigment lets go: a paint
already ground stiff, or of a fine pigment that holds its oil, gives up
less than you ask. (On a painting whose log's head has no `--@ drain`
line, the paper draws the full share.) A pile takes either medium or blot.

**The palette board.** Every pile is a heap on the board beside the easel.

- `pile{..., name="cool_mix"}` names it.
- `look --palette` (the `look` tool's `palette: true`) shows the board: each
  heap knifed out thick (its masstone and body) with a smear dragged from
  thick to thin across a black stripe (its tint as it thins, how much it
  hides), lit as in the gallery view. Judge a mix there before it touches the
  canvas.
- `p:add{{"<tube>", <parts>}, ..., medium=}` knifes more tube paint into a
  heap, in the units its recipe was given in (a pile knifed as
  `{"<tube A>", 4}, {"<tube B>", 0.35}` takes `{"<tube B>", 0.1}`
  as a tenth of a part more): look, adjust, look again. It returns the pile,
  its recipe with the added parts. The added paint is tube paint (in its
  tube's own oil, unthinned), so it dilutes the heap's turpentine, its
  `thinner` and its oil;
  a scraped heap is knifed fresh first.
- `mix{{p1, <share>}, {p2, <share>}, ..., name=}` knifes heaps together into a
  new heap, as they are now.
- `palette{scrape={p1, p2, ...}}` scrapes those piles off the board. A pile
  whose variable is gone goes by the name `palette()` lists it under:
  `palette{scrape={"sky", "pile 7"}}` scrapes every heap listed as "sky" and
  the one listed as "pile 7". A pile scraped off is knifed fresh the next
  time it is used, if the palette has room for it.
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
| lead white | basic lead carbonate | 0.82 | 0.8 | 0.6537508 | 2.0 |
| smalt | cobalt potash glass, coarse | 0.3 | 0.55 | 0.45 | 1.6 |
| pale smalt | a paler grade of smalt | 0.35 | 0.55 | 0.35 | 1.6 |
| yellow ochre | hydrated iron oxide earth | 0.8 | 0.7 | 0.8 | 0.8 |
| red earth | iron oxide earth | 0.85 | 0.7 | 0.9 | 1.0 |
| vermilion | mercuric sulfide | 0.9 | 0.75 | 0.989209 | 0.4 |
| raw umber | iron and manganese oxide earth | 0.8 | 0.65 | 0.9 | 2.4 |
| bone black | charred bone (carbon, calcium phosphate) | 0.9 | 0.7 | 1.0000027 | 0.9 |
| cobalt blue | cobalt aluminate | 0.55 | 0.6 | 1.0028677 | 2.2 |
| chrome yellow | lead chromate | 0.9 | 0.7 | 0.8445485 | 1.8 |
| Prussian blue | iron ferrocyanide | 0.35 | 0.45 | 1.0 | 2.4 |
| green earth | celadonite and glauconite clay | 0.2 | 0.35 | 0.3 | 0.8 |
| Rinmann's green | cobalt-zinc oxide | 0.35 | 0.5 | 0.4 | 1.4 |
| copper green | verdigris ground in oil | 0.25 | 0.4 | 1.0 | 1.6 |

Hiding is the tube's hiding as the pigment literature gives it (0
transparent, 1 opaque); in a painting, how much a coat hides follows from
what its pigments absorb and scatter, so a strong tube hides far more
than its number. Stiffness is the paint's body as it comes from the tube
(0 fluid, 1 stiff). Tinting strength is relative to an average pigment.
Some tubes mix from measurements of real oil paint: for them the
measurement carries the strength and this number is only a calibration
weight, so a tube at 1.0 can tint white more than ten times as strongly as
an earth at 0.8. Drying is the rate relative to average paint (higher dries
faster); a pile dries at its tubes' rates mixed by volume. The other
numbers, and the colors and strengths of the tubes not measured, are
estimates from the pigment literature.

### Thinner

```lua
w = pile{{"<tube>", <parts>}, thinner=0.5}          -- 0 (none) to 0.9
print(w)                         -- pile(<tube> <parts>; medium 0, thinner 0.5)
print(w.thinner)                 -- 0.5 (0.0 for a pile without it)
```

`thinner` is the share of solvent (turpentine, spirits) by volume knifed
into the pile, 0 to 0.9. Medium is oil and stays in the film; the solvent
leaves it. With thinner, the pile's pigments, oil, body and drying rate
stay the paint's own.

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
evaporation times are estimates.

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
| `glaze` | soft filbert 26 | 120–300 units, light pressure; a thin film (below) |
| `scumble` | filbert 9 | 10–20 units, worked back and forth; a thin, broken veil (below) |
| `blend` | badger 40 | a clean blender, crossing passes top to bottom, inside the mask. It is wiped clean as it goes, so it lifts paint as well as moving it: one pass takes up about 15% of a thin wet film, three passes about 40%: paint taken off, not a set change in value; what the value does depends on what the lift uncovers |

| option | meaning |
|---|---|
| `pile` | the pile every stroke dips into (every hand but `blend`) |
| `tool` | `"filbert 8"`, `{kind=, width=, ...}` or a brush |
| `length` | `{min, max}` stroke length in units |
| `coverage` | layers of strokes over each point on average; for `glaze` and `scumble`, the share of the mask their film reaches (below) |
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

**Glaze and scumble.** A `glaze` pass lays a thin film, about 7.5 µm,
wherever its strokes reach. Where strokes overlap, the film gets no
thicker. What shows through it depends on the pile: the more medium, the
more transparent the film; transparent tubes tint what is under it, and
white veils it. How strong the paint is counts as much as its hiding: the
strongest tubes lay a dark, nearly opaque film even with much medium.
Each further pass adds another film. A `scumble` pass
lays a thinner film, about 5 µm, as a broken veil; laid with body paint,
it hides part of what it crosses. For both hands, `coverage` is the share
of the mask the film reaches: 1 reaches all of it, 0.5 about half, and the
rest is left as it was. A coverage above 1 reaches no further and makes
the film no stronger. The other hands lay as much paint as the brush
carries, so they cover rather than tint.

**What stays inside the mask.** `work` plans its strokes from the mask:
each stroke is anchored in it, but its path can begin outside and cross
the edge. By default `detail` and `blend` keep their paint inside the
mask, soft where the mask is soft. `glaze`, and `blend` with
`clip=false`, carry paint past the edge by half a brush at most, fading
out. The other hands (`body`, `broad`, `hatch`, `scumble`) can carry
paint past the edge onto whatever is there, by as much as a stroke's
length. `clip=true` keeps
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

Masks are maps of the whole canvas, and operations return new masks. Shapes
and `mask(fn)` give coverage (0..1). `m:distance()` gives signed distance in
units (positive inside the edge, negative outside), and `m:map(fn)` keeps
whatever `fn` returns, so both can run past 0..1.

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

To turn a field back into coverage, band it, clamp it or ramp it:

```lua
d = m:distance()
d:band(0, 40, 4)                                       -- the 40 units just inside the edge
d:map(function(v) return clamp(v / 40, 0, 1) end)      -- 0 at the edge, 1 from 40 units in
d:map(function(v) return smoothstep(-20, 20, v) end)   -- a soft edge 40 units wide
```

`m:soften(units)` gives a hard edge a ramp that many units wide and leaves
a mask that is already graded (a function, a blur) at least as soft as it
was.

Points are `{{x, y}, ...}` or a flat `{x1, y1, x2, y2, ...}`.
`below` selects larger y values, toward the bottom of the canvas;
`above` selects smaller y values, toward the top.

Numbers and randomness: `rand(a, b)`, `randn(mean, sd)`, `math.random`,
`lerp(a, b, t)`, `clamp(x, lo, hi)`, `smoothstep(a, b, x)`;
`noise{seed=, octaves=4, period=200, persistence=0.5, kind="fbm"|"ridged"|"billow"|"plain", warp={period, amount, twice}, stretch={angle, k}}`
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

An outline of three or more points is closed (its line returns from the
last point to the first) unless it is given `open=true`; two points make an
open line. On a closed outline `o:below()` and `o:above()` fill the shape
itself and ignore `bottom`, and `o:band()` and `o:paint()` include the
closing stretch. For a horizon or shoreline you mean to fill below or
above, draw it with `open=true`:

```lua
shore = outline{{0, 500}, {500, 480}, {1000, 550}, open=true}
land = shore:below()
```

## Drawing: pencil, chalk, pastel, charcoal and eraser

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
a solid that rests on the tooth, wears and sheds what it abrades.

```lua
p = pastel(pile{{"vermilion", 1}, {"lead white", 2}}, {kind="soft"})   -- a round stick, Ø12 mm (diameter=)
h = pastel(pile{{"bone black", 1}}, {kind="hard"})                     -- a square stick, 6.35 mm
q = pastel(pile{{"lead white", 1}}, {kind="pencil"})                   -- a pastel pencil's core, Ø4.4 mm, 175 mm long, sharpened
q:sharpen()                    -- a pencil only: cut a fresh point; returns the mm of core it took off
p:stroke(pts, {force=2, alt=60, azimuth=45, roll=0, speed=80})          -- one stroke; each may be a list along pts
p:side(pts, {force=1.5})       -- laid flat across the stroke: a piece as wide as it is long (p:snap(mm) breaks one)
p:roll(30)                     -- turn it in the fingers: the next stroke meets a fresh edge
p:line(pts, {pressure=0.5})    -- a pencil's call: pressure as force (5 N at 1), held at 60°
smudge(pts, {force=1, pad=})   -- a finger drawn along a path (pad: a stump's few mm²)
smudge(mask, {angle=30, passes=2, pattern=})      -- a fingertip in loose circles over an area (pad<=60: a stump, strokes)
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
  touches. A light stroke touches only the highest fibers and leaves the
  paper's grain broken through the mark; a heavy one reaches into the pores.
- **It wears.** It lays what it abrades, in proportion to the force and the
  distance (Archard's law), and wears flat against the paper: a stick held
  one way grows a facet and draws wider as it does; rolled or tilted it cuts a
  new, sharp-edged one; a new stick's first strokes are narrow. `print(p)`
  says how many facets it has and how long it is.
- **Sharpening a pencil.** A new painting's pastel pencil is a Pitt
  Pastel's: 175 mm long on a 4.4 mm core, its point eight cuts at 0.20 rad
  (11.5°) that bare about 11 mm of core, the very tip crushed flat until it
  bears a 0.1 N touch at the core's hardness (0.25 mm up at the default
  `soft`). It wears flat as it draws, and its line goes wider and paler.
  `q:sharpen()` cuts a fresh point, as a blade does: the core gets shorter
  by what the cut takes off (as far back as the worn end sits behind a fresh
  point, measured along the taper; a few mm after twenty lines) and the call
  returns that in mm. It takes 16 s of hand time (MTM-1's motion times for
  the knife, eight wood cuts, eight core cuts and a touch-up on sandpaper). A
  pencil shorter than 75 mm once cut no longer reaches the web of a small
  hand and is refused; take a fresh one. A flat worn along the core with the
  pencil laid on its side (`q:side`), lying flatter than the cuts, stays. A
  soft or hard stick has no point and refuses `sharpen`. All this needs the
  switch `--@ pastel-pencil-sharpen`, which every new painting has; an older
  log keeps the pencil it was painted with (120 mm, Ø4.5 mm, cut at 0.22
  rad and blunted 0.15 mm), and its `sharpen` replays as it was painted
  (refused). The sources are in notes/research/pastel_stick_tribology.md §1.
- **Crumbs.** Most of what the stick abrades comes away as crumbs that ride
  under its face and settle into the tooth along and beside the stroke, the
  hollows taking more: a mark covers several times what the stick touches,
  as discrete specks. Crumbs that fall on wet paint are lost in it.
- **Keeping pastel inside a shape.** A stick has no `clip=`: it lays where
  it touches, and its crumbs land beside the stroke, so pastel worked up to
  a shape's edge spills past it. To keep a passage clean, cover it first:
  `lay_sheet(mask)` lays a sheet of paper over the mask, you work the
  neighbouring area up to and over its edge, and `lift_sheet()` takes it
  away with what landed on it, leaving an edge as sharp as the stick's angle
  and force make it. Lay and lift it in the same chunk (see **The sheet**
  below).
- **The tooth fills.** Loose pastel in the pores is a third body the stick
  slides on: the more there is, the less a stroke lays, until the paper
  refuses it. A drawing paper takes two or three layers. A sanded sheet
  (`paper={grit=...}`) holds four to five and a half times as much at
  P400, and its grains file the stick about three times as fast, so a light
  hand lays layer on layer there.
  Fixative binds it into a rough crust that files the stick again,
  so a fixed passage takes more pastel; and darkens it a little, as the resin
  wets the particles.
  The eraser lifts loose pastel out of the tooth with the coverage it takes,
  so an erased passage takes pastel again.
- **The finger** picks up loose pastel and lays it down along its path as fine
  grains pressed into the pores: colors drag into each other and the passage
  covers more for its thickness. It leaves fixed pastel where it is.
  Given a mask, `smudge` works the finger over the area as a pastellist
  does: a fingertip at 0.6 N in loose circles 15 mm across drifting along
  lanes at `angle` (degrees, as an azimuth), a stump (`pad` up to 60 mm²) at
  1 N in straight strokes there and back along each lane
  (`pattern="circles"|"strokes"` overrides), two passes, the second across
  the first. It presses `force` × the mask and lifts where the mask fades
  out; in circles a soft-edged mask softens the blend only through the
  contact's size, which changes little, so for a blend that fades out use
  `pattern="strokes"` or a smaller mask. It is the same finger as along a
  path, not a blur: what it picks up it carries on. Sources and which
  numbers are estimates: notes/research/pastel_blending.md.
- **Color.** A stick is its pigments as they look dry: absorption as in oil,
  scattering raised by the contrast between each pigment and air (much for
  ultramarine, Prussian blue, the lakes and the earths' clays; little for
  vermilion and the chromes). Dry blacks are a little grayer than in oil.
  A dye lake (rose madder, carmine, yellow lake, magenta) is its dye on a
  white alumina, as its measured powder is: rose madder a muted rose,
  carmine a deep crimson, and with white they make clear pinks. Permanent
  alizarin (a quinacridone, not a lake) is a dusky violet-rose dry, and green
  earth, verdigris, lead-tin yellow and transparent oxide yellow scatter by
  their measured indices.
- **Feel.** `feel(x, y)` answers in words: the surface (paper or sanded paper, paint and how
  far it has set) and the pastel in the tooth (loose or fixed; a little, the
  tooth taking it, half full and more, full).
- Pastel skips wet paint, as graphite does; let the paint set first. Paint
  laid over pastel seals it, as it seals a drawing.

**Strokes over an area.** A stick's `hatch` covers a passage as a
pastellist does, each mark one stroke of the stick (it wears and sheds as
`stroke` does). By default its marks are Cassatt's open strokes, 8 mm apart
and 50 mm long; a pencil's `hatch` keeps its 1.5 mm and 12 mm:

```lua
p:hatch(m, {angle=function(x, y) return -0.9 - x / 2000 end})  -- strokes along a direction field (radians), or a form's field
p:hatch(m, {angle=-1.0, zigzag=true})                           -- each mark one back-and-forth stroke
p:hatch(m, {zigzag={amplitude=30, period=50, round=0}})         -- its swing to either side and per swing there and back (units), sharp corners
p:hatch(m, {angle=-1.0, length=600, spacing=120, jitter=true, open=0.3})  -- slashing diagonals, loose, the paper between
p:hatch(m, {open=bare})                                         -- open in proportion to a mask: 1 bare paper, 0 covered
p:hatch(m, {alt=30, azimuth=45, speed=250})                     -- how the hand holds and moves the stick
```

- `angle` is radians (as for a pencil: 0 along x, positive turning down the
  canvas), or a `function(x, y)` returning them, or a form's field, as
  `work` takes. Along a field the strokes follow it: lines a `spacing` apart
  traced through the mask, each cut into strokes of up to `length`, so the
  hatching turns with the form and stays evenly spaced as the field fans.
- `zigzag` turns each mark into one continuous stroke swinging back and forth
  across its line: `amplitude` to either side, `period` along the line per
  swing there and back, `round` 0 for sharp corners to 1 for a sine (the
  default: a hand oscillating across the line; at 5 mm and 8 mm its turns are
  0.3 mm across, sharp under a stick's line). The
  marks run along `angle` and the legs cross it, the more squarely the larger
  `amplitude` is against `period` (by default at about 68° to it): for legs
  slanting one way, run the zigzag across the slant. A swing that would leave
  the mask is cut short at its edge.
  With a zigzag, marks are 40 mm long and bands sit a swing's width apart
  unless `length` and `spacing` say otherwise; `zigzag=true` swings 5 mm to
  either side and 8 mm per swing.
- `jitter={length=, angle=}` stretches each mark by up to ± the share
  `length` (a mark that would grow out of the mask keeps its length) and turns
  it by up to ± `angle` radians about its middle; `jitter=true` is ±30% and
  ±0.17 rad. Seeded: the same `seed=` lays the same strokes.
- `open` leaves that share of the marks out, the paper (or what is under)
  showing between the rest: a number 0..1, or a mask whose value at each
  mark's middle is its chance to be left out. The same marks are left out
  whatever the jitter.
- `alt`, `azimuth` and `speed` hold the stick for every mark as `stroke`
  takes them. By default it is held at 50° (a held pen's measured altitude)
  with its upper end to the lower right (45°), and each stroke goes as fast as
  its length asks: 80 mm/s for 12 mm (150 ms), as L^0.53 for others (171 mm/s
  for 50 mm; a straight line's drawing time goes as L^0.469), a zigzag at its
  legs' speed. A `speed=` holds every mark at that.
- Each mark bows as the hand swings it: an arc about the wrist (the hand's
  189 mm), its middle away from the hand, L²/(8R) for a mark across the
  wrist's swing (1.7 mm for 50 mm), less as the mark turns toward the hand,
  none along it; a sweep longer than the wrist turns (132 mm) swings from the
  elbow (467 mm). The hand is where `azimuth` points.
- The defaults for zigzags and jitter are estimates from Cassatt's pastels
  of 1895–1908, as are the lengths and angles of her passages:
  backgrounds of diagonals 30–80 mm long at 40–65°, 5–15 mm apart; dresses
  of long strokes along the folds; zigzag legs 6–20 mm
  (notes/research/cassatt_pastel_strokes.md, which also sources the hold,
  speed and bow). In a new painting a plain `p:hatch` with a stick (none of
  these options) lays these strokes, not the pencil's as before.
- A painting begun before these came (its log's head has no
  `--@ pastel-area-strokes`) refuses them, as it did, and replays as painted:
  its plain stick hatching lays the pencil's marks, 1.5 mm apart, 12 mm long,
  at 60° and 80 mm/s, as it did. A pencil's `hatch` doesn't take them.

**Pastel with water.** A stick dipped in water, a wet brush through laid pastel, ground pastel as a paste:

```lua
p:wet(1)                                  -- dip the tip (share 0..1 of what it soaks up); returns how wet it is
p:stroke(pts, {force=2})                  -- a creamy line: lays more, darker while wet, fewer crumbs
b = brush("round", 20)
b:dip(0.8)                                -- water only (a clean brush: no oil paint on it)
b:wash(pts, {pressure=0.5})               -- through laid pastel: the loose pastel turns to paste and moves along
b:dip(1, {pastel=p, amount=0.5})          -- ground pastel in the water: a paste (or, with little of it, a wash)
b:wash(pts)                               -- lays it, wet; returns the pastel laid, mm³
```

- **While wet** the water fills the air between the particles and the
  pastel darkens, by how much of its pores the water fills and by its
  pigments' refractive index: an ultramarine or a lake goes nearly as deep
  as in oil, a vermilion or an earth hardly changes. `feel` says `, wet`.
- **It dries on the painting's clock** (strokes' hand time and `wait`).
  The pastel's fine pores hold their water; what stands over them sinks
  into the sheet as its sizing lets it (`paper={cobb=25}`, the default:
  about 3 µm in the first second, 25 µm in a minute; `cobb=false`, an
  unsized sheet, at once), until the sheet's pores are full (`absorbent`).
  The studio (20 °C, 50 % RH) evaporates about 0.7 µm of water a minute,
  first what stands, then what the sheet took in, up through the pastel,
  which stays dark until the sheet under it is dry: a wet stick's line
  dries in minutes, a brushed wash in about an hour.
- **Dry again** its gum has set: the pastel is bound where it kept enough
  of its gum to hold against a firm finger (about half a percent of it).
  Water that sank into the sheet took its share of the gum with it, so a
  thin veil flooded with many times its own water on a sheet that drinks
  it stays loose; a wetted passage that kept its pores' water binds.
  Wetting it again binds no more. Bound,
  the finger, a dry brush and the eraser move it no more than fixed
  pastel, and a stick takes to it as to a fixed crust. Its color comes
  back almost to dry: the gum in its pores darkens it a little.
- **A dipped tip** has lost its gum's hold and is a wet chalk: barely
  damp (under about half full) it draws as dry; wetter, it is plastic,
  far softer, laying more for the same force and smearing instead of
  crumbling; soaked (past about two thirds full) it is a slurry the face
  presses into the tooth as a paste, a creamy line with no crumbs. A dip
  soaks about 3 mm of the end, and its water spreads up the dry stick (a
  third of it left at the tip after ten seconds), so a dip serves a few
  strokes; dip it again (`p:wet`, a reach to the jar's hand time).
- **A wet brush** holds water in its belly (a round about 0.16 mL for a
  6 mm brush), lays it as it lays paint (more for a harder press), and picks up the wetted paste as it picks up wet paint,
  laying it on along its run, with what it already held: drawn out of a
  passage, it drags its color across the paper. Bound and fixed pastel
  stays. Grinding a stick into the water (`pastel=`) shortens it; a full
  paste (`amount=1`) fills the space between the hairs, and its solids
  push out the water. `b:wipe(1)` empties the brush. Not under a sheet,
  and not on wet paint.
- **Water over charcoal.** Charcoal brings no gum: vine charcoal wetted
  dries as loose as it was, and over pastel it is bound only by the
  pastel's gum. A wet brush carries charcoal on as charcoal, and grinding a
  charcoal stick into the water (`pastel=`) makes a charcoal wash. A
  charcoal stick's own tip can't be wetted (`p:wet` refuses it): it is no
  chalk soil, and its wet tip isn't modeled.
- **On a sanded sheet** (`grit=`) an acrylic coat keeps the water off the
  paper: a wash stands in the tooth until it dries, and binds. A glue coat
  lets it through to the sheet. Japan paper (`paper="japan paper"`) is
  unsized and drinks a wash at once.
- Not modeled: water running sideways (backruns, blooms), the paper itself
  darkening while wet, cockling, steam.
- Pastel with water is there in a painting begun since it came (its log's
  head says `--@ wet-pastel`, with `--@ paper` and `--@ drawing-optics`);
  an older painting replays as it was painted and has no `p:wet`, `b:dip`
  or `b:wash`. The numbers and the few estimates left are in
  notes/research/wet_pastel.md.

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
  brush, a puff, a tap, the eraser and fixative work: wet paint and graphite
  aren't modeled under a sheet and refuse.
- **A dry brush** (`b:dust`, the brush wiped clean) lifts loose pastel as
  far as its bristles' tips reach into the tooth and no deeper: a hog's tip
  (about 50 µm) reaches only the top micrometers of the pores, so what fills
  them stays as a ghost; a finer `tip` reaches further. Fixed pastel stays.
  The bristles keep what they lift until they are full (a brush a centimeter
  wide holds a few milligrams; `b:wipe(1)` empties it); then the rest is pushed
  ahead and left as a ridge of crumbs where the brush lifts. Over a
  speckled passage it smears more than it cleans.
- **The knife** (`k:scrape`) over pastel on paper: the hand's force (1–5 N by
  `pressure`) bears along the blade's edge, so a wide palette knife presses
  lightly and a short edge hard. The edge sinks into the paper as far as
  the sheet's compression and its surface's stiffness allow, and takes the
  loose pastel above that depth; a stiff blade rides the paper's high spots
  and bridges its low ones, so a scrape is mottled. Pressed hard enough it
  sets the paper: burnished, flatter, the pores shallower (less tooth). Its
  drag can tear fibers loose: raised nap, more tooth there. `edge` is the
  edge's radius in µm (a painting knife 20–500; a scalpel about 1): a
  scalpel on a short edge cuts, shaving a fiber layer and taking even fixed
  pastel. Otherwise fixed pastel stays.
- **A puff** (`blow`) shears the surface in a ring around where it is aimed
  (nothing at the very center): loose crumbs heaped on the tooth go easily;
  the pores' fine grains only under a hard blow close in, and only their
  top few micrometers. Most crumbs it lifts roll outward and settle in a
  broad ring past where it stops lifting them; some hop further, a few
  centimeters, and over ten from a hard blow close in; a little of the fine
  grains settles thinly further out, and most are carried off. Crumbs
  landing on a paper mask go with it.
- **A tap** (`tap`) shakes off the crumbs heavy enough to beat their hold:
  heaps and ridges go, the grains in the pores stay, and fixed pastel stays.
- **Holding a stick up to the picture:** `look` with `hold: "<stick>"` and
  `at: "x,y"` (see Looking).

Pastel is there in a painting begun since it came, with paper (its log's
head says `--@ paper`); an older painting replays as it was painted, and
has no `pastel`, `smudge`, `feel`, `lay_sheet`, `lift_sheet`, `blow` or
`tap`, no `b:dust`, no `edge=` on `k:scrape`, and no stick to hold up. A painting
begun before the lakes' dry colors came (its head has no `--@ dry-lakes`)
replays its lake sticks and paper tones, and those of the pigments whose
index was corrected, in the colors it was painted with. The numbers (how hard a stick is, how fast it wears, what dry pigment
looks like) are estimates, not measurements (notes/pastel.md); the lakes'
dry colors are fitted to measured powders
(notes/research/dry_pigment_optics.md §6).

**Charcoal.** Vine (willow) charcoal is a charred twig with no binder;
compressed charcoal is charcoal powder pressed with a gum binder.
Both are sticks in the tooth, as pastel is: they draw with a stick's calls
and a pencil's, and the finger, the eraser, a dry brush, a puff and fixative
work on them as on pastel.

```lua
v = charcoal()                          -- vine, a round stick Ø8 mm × 140 (charcoal{kind="vine", diameter=5, length=}: thin 5, medium 8, thick 10)
k = charcoal("compressed")              -- compressed, a square stick 6.35 mm × 76 (charcoal{kind="compressed", diameter=})
v:sketch(pts, {pressure=0.3})           -- a pencil's calls; v:stroke, v:side, v:roll, v:snap as a pastel's
```

- **Vine** is soft and black, its splinters lying loose on the tooth: the
  kneaded eraser takes it a little more readily than graphite, a finger
  smears it far, a dry brush or a puff lifts it. The stick is mostly pores,
  so a stroke lays little: it reads grey where the line is broken and the
  paper shows. It is for laying in and correcting.
- **Compressed** is darker (half its carbon is fine soot packed with gum,
  so its facets reflect less), covers far more per stroke, and lays finer
  particles, which hold to the fibres about as hard as the eraser pulls:
  the eraser lightens it and leaves a stain. Its gum binds nothing once
  dry (at the studio's 50 % humidity), so the finger and a puff move it as
  they move vine.
- Charcoal is matte: no graphite sheen, and no veil of reflected light over
  it either (a bed of carbon particles shows only what its facets reflect):
  it is a deep black. **Fixative** (`fix()`) binds it and darkens it a
  little, the more the thinner it lies; the eraser and the finger then
  leave it. Paint over it seals it.
- **Pastel over charcoal.** Pastel laid over loose charcoal mixes with it in
  the tooth and comes out greyer; over fixed charcoal it lies clean. Fix a
  charcoal drawing before laying pastel on it. (The stick itself doesn't
  pick up charcoal and dirty: only the tooth mixes.)
- Graphite doesn't smear with the finger here at all; charcoal does.

Charcoal is there in a painting begun since it came, with paper (its log's
head says `--@ charcoal` with `--@ paper`); an older painting replays as it
was painted and has no `charcoal`. Its color, how much a stroke covers and
how the eraser takes it come from carbon's optics, the stick's porosity
and the particles' adhesion; vine's hardness is wood charcoal's measured
one; compressed's hardness and both wear rates are estimates
(notes/research/charcoal.md).

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
leaves streaks along its path and a blot a crumpled patch. What the rag
lifts stays in the cloth: none of it goes back onto the canvas.

A rag dipped in spirits (`r:dip`, 0..1) lifts wet paint more readily and
reaches farther into the hollows. It works gradually: wipe after wipe
takes the paint nearly to the ground. Its folds still leave streaks.
Dampness (`r.damp`) halves every three minutes of painting time. Refolding
retains the cloth's dampness; another dip adds spirits.

The part of the cloth in use is its face. `r.load` is how full the face
is (0 clean, 1 full): the fuller, the less it lifts. One face holds about
as much paint as a 20 µm wash covering twenty squares the width of the
pad; thicker paint fills it over a smaller area, so a wipe over a big wet
area can fill it in its first few strokes. A wipe that runs on a full face
lifts almost nothing, and the chunk's reply says so
(`rag: the face in use is full: ...`). `r:refold()` turns a cleaner face
outward, though every face holds at least what has soaked through the
whole cloth (`r.soaked`, 0..1); `rag()` takes a fresh one. These numbers
can be read, not set. Over a mask, `refold=` refolds whenever the load
passes that value.

Over a mask, `wipe` lays wipes side by side across it in `angle`'s
direction, back and forth, each starting and stopping a little inside the
mask. The rag lifts only inside the mask, as much as the mask's value
there: paint outside it stays as it was, even where the pad reaches past
the edge. `passes` goes over it again, each time a little turned. `seed=`
fixes the cloth's folds.

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
space above it are seen). `w:proxy(s, body)` places an unseen body that
casts a shadow. `w:aerial(Z)` and `aerial(dist, visibility)` give
how much air lies between the eye and a distance (0..1).

**Depth.** A view knows what lies behind what. `w = w:layer(name, mask,
depth)` registers a shape you paint by hand at a depth (meters, a spot, a
canvas point `{x, y}` where it stands on the ground, or `"ground"`); make
the view after it. Then `v:visible(x)`, `v:front(x)`, `v:behind(x)`,
`v:at_depth(m)`, `v:between(a, b)` and `v:seen(x, y)` are masks and
answers, and `v:cast_shadow{soft=, from=}` and `v:contact_shadow{reach=,
from=}` are shadows that fall off with distance from what casts them.
Things are named by body number, layer name, `"ground"`, `"water"`,
`"surface"`, `"sky"`, `"bodies"`, `"layers"`, a mask or a list. Only
bodies cast `v:cast_shadow` and `v:contact_shadow`: `from=` takes any of
these names, but only the bodies among them cast, so a layer named there
casts nothing. Passes take
`visible=` (only where that is seen), `behind=` (only where nothing in
front of it is) and `at=` (a depth in meters): strokes still overrun the
mask's own edges and stop at what is in front.

## Time

Painting takes time, and paint dries on the painting's clock. Only
painting operations (strokes, touches, passes, trips to the palette) and
`wait(minutes)` advance it. Real time between chunks does not: paint
doesn't dry while you think. `wait(minutes)` passes that much painting
time at once.

Every successful paint reply reports the current painting time, before
`ok`.

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
  gap between strokes, a wiped spot), it reads the layer beneath. It only
  reports: a pass later in the same chunk runs whatever it returns, unless
  your code tests the answer (`if drying(x, y) ~= "open" then ... end`).

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
canvas at its full detail, 2.4 pixels to a canvas unit, when it fits within
1200 pixels a side; a wider crop is scaled down to fit, and `size` sets a
smaller limit. The reply gives the scale in pixels per canvas unit.
`crop: "x0,y0,x1,y1"` gives two opposite corners in canvas units, not a
position and width/height.

| `look` with | shows |
|---|---|
| nothing | the whole canvas, scaled down |
| `crop: "300,200,500,350"` | a window in canvas units, at full detail |
| `mode: "value"` | in grays |
| `mode: "squint"` | blurred, as through half-closed eyes |
| `mode: "edges"` | the picture's edges drawn over a dim gray of it: orange where an edge is hard, blue where it is soft, brighter the more the lightness changes across it; hard and soft as they read at the size of the image the look returns |
| `mode: "mirror"` | flipped left to right |
| `mode: "relief"` | under a raking light from the upper left, so the paint's ridges, furrows and slabs show (wet paint shines) |
| `mode: "gallery"` | as the picture hangs: lit from above and a little left at 55°, so impasto models softly |
| `mode: "relief"`, `light: "45,15"` | the light from that azimuth (degrees: 0 from the right, 90 from the top) and elevation (0 to 90); the lower the light, the harsher; at 0 the lamp lies in the canvas's plane: flat paint gets only the room's light, ridges are lit on the lamp side; at 90 it is overhead and casts no shadow |
| `mode: "value,squint"`, `size: 600` | modes combine; `size` sets the long side |
| `grid: true` | a squared grid in canvas units, labeled along the edges |
| `crop: "300,200,500,350"`, `grid: 10` | a window with a grid every 10 units |
| `mask: "<name>"` | the named global mask's outline and a light fill over the view; several names may be comma-separated |

The grid is drawn on the PNG only, like the squares ruled over a drawing
to transfer it.

`look` with `palette: true` shows the palette board instead of the canvas:
each live heap's masstone and body paint, with a smear dragged across a
black stripe. It shows the heap's paint before the unevenness of a
brushload. It takes no
other option, and nothing on the canvas or the clock changes.

| `look` with | shows |
|---|---|
| `survey: true` | the whole canvas at full detail, as several tiles of at most 500 units (2 × 2 for most canvases, 2 × 1 for one twice as wide as high), each a separate image; modes apply (`mode: "gallery"`) |
| `compare: "<an image's path>"` | an image in this studio, usually an earlier look, on the left and the current view on the right, at the same height. The right is the view the other options ask for (`size: 800` without `crop` or `size`); the image on the left is only resized to that height, so to compare an earlier look with the same view, repeat its crop, modes, light and grid options |
| `hold: "<a knife's or a pile's name>"`, `at: "x,y"` | (speculative) the loaded knife held up to the canvas: the passage around the point (240 units square, clipped at the canvas's edges, or your `crop`) with the blade's end at it, the paint thick on the steel, seen in the same light and mode as the passage (`mode` value, squint, relief or gallery, `light` and `mask`; not mirror, edges, `grid`, `size`, `palette`, `survey` or `compare`). A knife shows what is on it, as full as it is up to a full load; a pile, a fresh full load. It shows the paint on the knife, not how it would look laid |
| `hold: "<a pastel stick>"`, `at: "x,y"`, `pose: "force,alt,azimuth"` or `side: "<direction>"` | (with pastel) the stick held there, seen from above over the passage (120 units around the point, or your `crop`): its low end as a light shadow, where it rests on the tooth at that force in its color, and the band where its crumbs would settle, tinted; `side` lays it flat across a stroke going that way; with neither, 1.5 N at 60°. The reply says how much it touches and how much its crumbs would reach, in mm². The stick can be a field of a table (`P.glow`). It shows where a stroke from there would lay, not what it would leave |

A survey can arrive in a partial batch to fit the painter's image budget.
The reply lists the remaining tile paths; read them one at a time to inspect
the rest of the canvas.

A whole view defaults to 1000 pixels on its long side (`size` up to 1600),
about 42% of a 2400-pixel canvas's width: small marks, beads of paint and
stray strokes can be hard to see in it. Survey the canvas after
each campaign, and compare before and after.

## How chunks behave

- **A long chunk.** If `paint` stops waiting for the easel's answer before
  a chunk finishes, its reply says whether the chunk is still running, with
  the painting time it has taken so far, or whether it is in the log. While
  a chunk runs, `status` says so, with the painting time it has taken so
  far and the painting's clock. Once the chunk is in the log and the easel
  is still finishing its reply, `status` says that instead.
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
- It keeps its own clock. Time spent or waited there (`wait`) doesn't
  pass for the painting, and the painting's time doesn't pass for it.
- To use what worked, paint it on the painting: the same pile, the same handling.
- `new_scratch: true` (with `scratch: true`) puts the scratch canvas aside and starts a
  fresh one before the chunk runs. The old log stays, as `paintings/lua/scratch-1.lua`, and so on.
  The fresh canvas stays even if the chunk stops with an error: the old trial's marks,
  variables and clock don't come back, though its log is still there to read.
- A scratch canvas the easel can't reopen is put aside the same way the first time it is
  used after the easel opens, and the reply says so.
- A trial judges a passage only as far as it repeats that passage's conditions: lay the
  underlayer you mean to work over, and let it dry as long as it has on the painting.

## Surfaces: sheets beside the painting

A painting begun with named surfaces (its log's head says `--@ surfaces`, beside
`--@ paper`) may keep sheets of paper beside its canvas, in the same session: a study to
trace from, a proof to print on. An older painting replays as it was painted, and none of
this is there: `sheet`, `use`, `surfaces`, `handoff` and `receive` don't exist in it.

```lua
st = sheet{name="study", size=300, aspect=4/3, paper={grammage=60, calender=0.6}}
use(st)                       -- draw on the study: every drawing verb acts on it now
h = pencil("2B"); h:line({{100,100},{800,700}})
use("painting")               -- back to the canvas canvas{} made
names, current = surfaces()   -- the names, the painting first, and the current one's
```

- **`sheet{name=, size=, aspect=, paper={...}, ground=, seed=}`** takes a sheet from the
  drawer: a surface of its own, with its own paper (the keys `canvas{paper=}` takes) and
  size in mm. It is laid out at the painting's pixel pitch, so a line keeps its width from
  one sheet to another. Its width is 1000 units, as the painting's is (`st.H` is its
  height). It is not in hand until `use(st)`.
- **One surface is current.** Every drawing verb (pencil, pastel, brushes, knives, the rag,
  `wait`'s ageing, `drying`) acts on it. `use(s)` or `use("name")` takes another up and
  returns the name of the one put down. A sheet laid on the picture is lifted first.
- **What each keeps.** A surface keeps its own paint, drawing, pastel and paper. The hand
  is shared: the same brushes, knives, rags, palette and piles go from one to the other,
  carrying what is on them.
- **One clock.** Time passes for every sheet: hand time spent on the study and `wait`
  there age the painting's paint too. A surface put down is aged all at once when it is
  taken up, or looked at, to exactly what it would be had the painter waited beside it.
- **A failed chunk** puts back every surface it touched, and which one was current.
- **Looking.** `look`, `probe`, `save` and `status` take `--surface <name>`. Without it
  `look`, `probe` and `save` show the painting, as before, and `status` the current
  surface (its line names them all). A look changes nothing.
- **Saves and checkpoints** keep every surface. A painting that never takes a sheet saves
  exactly as before.

**Handing a surface to another session.** An artist and a printer are two sessions, each
seeing only what it is handed. A surface goes from one to the other as a file in the
studio:

```lua
-- the artist's session
sha = handoff(pl, "handoff/key.surface")   -- pl leaves this session; the reply gives its sha256
-- the printer's session
k = receive("handoff/key.surface", {after=90, sha256="<the hash handoff gave>"})
use(k)
```

- **`handoff(s, path)`** takes the surface out of the session, with its whole state and
  the session's clock, into a `.surface` file in the studio (never outside it, never over
  a file already there). The painting itself stays. It happens only if the chunk is kept.
- **`receive(path, {after=, sha256=, name=})`** takes it in. `after=` is the minutes it
  spent on the way (the log names them: the wall clock can't be replayed), and it arrives
  aged by exactly that, through the same drying as `wait`: ink on a plate keeps drying,
  paint keeps setting. `after=0` brings it as it left. `sha256=` names the file's content:
  a receive without it says the hash, and a replay that finds another file there refuses
  it. `name=` renames it on arrival.

## Plates: copper and zinc beside the painting

A painting whose log's head also says `--@ plate` (new paintings do; it needs
`--@ surfaces`) may keep intaglio plates among its surfaces and work them as a drypoint
artist does. Without the switch none of this exists (`plate`, `needle` and the plate's tools
are not there), and a painting with named surfaces that makes no plate saves and replays as
before. Ink and wiping come with `--@ press` (below, after damp paper).

```lua
k = plate{name="key", metal="copper", temper="half-hard", size={346, 211}, thickness=1.3, bevel=45}
n = needle{tip=25, cone=30}       -- point radius µm, half-angle of its cone, degrees
use(k)                            -- in hand: only the plate's tools mark it
r = k:drypoint(n, {{100,200},{800,260}}, {force=2, alt=55, azimuth=0, speed=30})
k:burnish({{300,300},{600,300}}, {force=30, radius=10, passes=3})
k:scrape({{650,400},{800,400}}, {force=5, width=8, passes=1})
k:roulette({{100,500},{900,500}}, {pitch=0.2, tooth=30, force=3})
print(k:feel(450, 230).words)     -- "a line about … µm wide and … µm deep, burr standing … µm"
use("painting")
```

- **`plate{name=, metal=, temper=, size={w, h}, thickness=, bevel=}`.** The period box
  (an 1890s Paris print box) has `"copper"` and `"zinc"`; another metal is refused.
  - Copper comes `"annealed"` (45 HV), `"eighth-hard"` (65 HV, an estimate between the two
    measured tempers) or `"half-hard"` (90 HV, the default). Zinc is `"rolled"` (45 HV).
  - Size is in mm; thickness (default 1.3 mm) and bevel (45°) are for the plate mark later.
  - It is laid out at the painting's pitch, as the sheet it prints on will be. Its width is
    1000 units (`k.H` its height).
  - Steel facing comes later (`--@ edition`); the plate keeps the hook.
- **A statistical plate.** A pixel is wider than a drypoint line. So each cell keeps its
  openings binned by width, each bin's share, depth and width, and its burr's volume,
  height and side, not a picture of the lines. Those are what wiping and the press will
  read.
  - The tools' paths are kept too, so a crop look draws the lines again at its own scale.
- **`needle{tip=, cone=}`** and **`k:drypoint(n, pts, {force=, alt=, azimuth=, speed=})`.**
  Each option is a number or a list along the points, as for a pastel stroke: force in N
  (a hand: 0.5–5), alt and azimuth as pastel takes them, speed in mm/s.
  - **Width from scratch hardness:** w = √(8F/πH). Four times the force gives twice the
    width, and zinc is √2 wider than half-hard copper.
  - **Depth** follows the point: its rounded tip, then its cone.
  - **Plough or cut.** The needle's lean toward the way it goes (pulled, top ahead) lowers
    its attack angle, 90° − cone − lean.
    - Below copper's critical 60° (measured 45–85°) it ploughs: all of the groove's metal
      goes into burr, 0.75 × the depth high (an estimate: no burr has been measured).
    - Pushed or held upright, it cuts a chip and leaves a clean line with no burr. The verb
      says so.
  - **The burr's side.** Leaning to one side throws the burr to the other; leaning past
    its half-angle, all of it.
  - **The result** is a table: `width`, `depth`, `burr`, `beta`, `cut`, `ploughed`,
    volumes in µm³ (`groove`, `burr_volume`, `chip`), `length`, `seconds`.
- **`k:burnish(pts, {force=, radius=, passes=, speed=})`.** A steel burnisher (a sphere of
  `radius` mm).
  - **Burr first.** It sits on the burr and presses it flat where the burr can't bear its
    force at the metal's hardness. The fin folds onto the land beside the line.
  - **Then the line's rims.** Once no burr holds it off, the rims flow into the line where
    its Hertz pressure passes what a rim bears, by the contact's elastic give each pass.
  - A drypoint line goes after a dozen passes or so. Metal is conserved (`flattened`,
    `pressure` in MPa).
- **`k:scrape(pts, {force=, width=, passes=, speed=})`.** A three-sided scraper held to cut.
  - It takes the burr off, then shaves t = F/(H·width·tan 30°) a stroke (about 1 µm for
    5 N on 8 mm of copper).
  - The lines go with it, and it leaves a shallow dish (`scraped`, `chip`).
- **`k:roulette(pts, {pitch=, tooth=, cone=, force=, speed=})`.** A toothed wheel: a pit
  every `pitch` mm, each indented at the hardness, with a ring of burr.
- **`k:feel(x, y)`** answers as a fingernail would: `words` ("a line about 50 µm wide and
  40 µm deep, burr standing 30 µm", "a shallow dish 4 µm down", "smooth") and the numbers
  (`depth`, `width`, `open`, `burr`, `level`).
- **Hand time.** A tool's path at its speed (default 30 mm/s, an estimate: drypoint is hard
  work) goes on the clock, which every surface shares.
- **Only its tools mark it.** Paint, pencil and pastel are refused while a plate is in hand.
- **Looking at it.** `look --surface key` shows polished metal.
  - **The colour** is Fresnel reflection from the metal's measured optical constants:
    Johnson & Christy for copper, Werner et al. for zinc.
  - **Lines read dark** because a V-groove sends the light from wall to wall before it
    climbs out. The burr catches light at its faces' slope.
  - **Light.** `--mode relief` or `--light az,el` lights the relief (lines, burr, a scraped
    dish) with the canvas's own raking light; `--mode mirror` previews the print's way
    round.
  - **A crop** is drawn again from the tools' paths at the crop's scale.
  - `probe --surface key --at x,y` (or `--crop`) gives the open share, the lines' width
    and depth, the burr and the level. `save` refuses a plate.
- **Saves, checkpoints, replay and handoff** keep the plate's whole state (its cells and
  its tools' paths) in its surface's entry. A plate handed off arrives exactly as it left,
  and a session without the switch refuses to receive one.
- **Physics and estimates:** notes/research/intaglio_physics.md, notes/research/plate_physics.md.

### Etching: grounds, acid and aquatint

A painting whose log's head also says `--@ etch` (new paintings do; it needs `--@ plate`) may
ground its plates, draw through the ground, bite them in acid and lay an aquatint. Without the
switch these tools refuse ("this painting doesn't etch"), and a plate no etching tool has
touched saves and replays byte for byte as before. Ink and wiping come with `--@ press`.

```lua
use(k)
k:ground("hard", {um=4})                        -- rolled over the face
n = needle{tip=40, cone=30}
r = k:needle(n, {{100,200},{900,200}}, {force=0.3})   -- through the ground: r.touched, r.width (the window)
k:immerse{acid="ferric", baume=36}             -- into the bath (face down by default): it bites on the clock
use(study)                                      -- ... while you draw on another surface, or wait
use(k)
r = k:lift()                                    -- out, rinsed and dried: r.minutes it lay there, r.depth
k:stop_out(rect(0, 400, 1000, 300))             -- varnish: no acid there
k:bite{acid="ferric", baume=36, minutes=30, rock=true}   -- in, 30 minutes, out, in one
k:clean()                                       -- solvent: ground, varnish, rosin off
k:aquatint{box=0.6, wait=30, minutes=1}         -- the dust box: wait s after stirring, minutes in it
k:melt()
k:bite{acid="nitric", parts=8, minutes=2, feather=true}   -- on zinc: sweep the bubbles
print(k:feel(500, 200).words)
use("painting")
```

- **`k:ground("hard", {um=})`.** A hard ground (beeswax, bitumen, rosin) rolled over the
  whole face, 4 µm by default (an estimate: 2–8). It covers whatever lines are already
  there. It goes on a clean plate (`k:clean()` first), and takes 10 minutes of hand time.
  Soft ground is `--@ soft-ground`, later.
- **`k:needle(n, pts, {force=, alt=, azimuth=, speed=})`** draws an etching needle
  (`needle{}`) through the ground.
  - **The window.** It parts the ground where its point is wider than the ground is thick.
    The window it leaves (`width`) is the point's width at the ground's top.
  - **The metal.** The rest of the force falls on the metal, which it marks lightly by the
    same scratch law as drypoint. The default 0.3 N with a 40 µm point is about 3 µm.
  - **A line too light** to get through the ground (the ground carries a few mN) bares
    nothing, and the verb says so (`touched`).
  - The default speed is 40 mm/s. An etching needle needs a ground: without one, use
    `k:drypoint`.
- **`k:stop_out(mask)`.** Stopping-out varnish brushed where the mask (made while the plate
  is in hand) is at least half. No acid reaches there until `k:clean()`. Hand time is its
  area at 160 mm²/s (an estimate).
- **`k:immerse{acid=, …}`** puts the plate into its tray, and **`k:lift()`** takes it out.
  - **The bath is on the session's clock.** While the plate lies in it, it bites through
    whatever the session does meanwhile: work on another surface, `wait`, paint drying.
  - **The bite** is worked out when the plate comes out, from the minutes it lay there, on
    the plate's own clock (the session's clock, which a resting surface, a save and a
    handoff keep). The minutes are read to the millisecond.
  - **Hand time.** Lowering the plate in takes seconds, which count as bath time. Lifting it
    out, rinsing and drying it takes 2 minutes of hand time, after the bite.
  - **While it is in,** the plate can't be worked or felt: its tools refuse with "the plate
    "key" is in its ferric bath (12 min so far): lift it out first (k:lift())".
    - A second `k:immerse` is refused the same way: lift it out first, even to change
      baths. `k:lift()` on a plate that isn't in a bath is refused too.
    - `look` and `probe` show the plate as it would come out now, and leave it in. `probe`
      adds the bath and its minutes so far. `status` lists every plate in a bath.
  - **Rocking and feathering** are not counted as hand time: a rocked tray stands for a
    rocker, and the feather's sweeps are short. Working elsewhere during a feathered bite
    is allowed.
  - **Out, stopped out, back in.** Taken out, stopped out and put back into the same acid at
    the same strength, the plate goes back into the tray it came out of, still loaded with
    metal. Lines under the varnish keep the depth of the first bath.
  - **A plate left in.** A plate left in its bath keeps biting for as long as the session's
    clock runs, through saves, replays and handoffs. A session's log can always be taken up
    again, so the plate stays in until it is lifted out, and then it has bitten the whole
    time, as a plate forgotten in a tray has.
    - Past a day the steps lengthen. A bath stops biting once its oxidant is spent (under a
      thousandth of what it was made up with), and the lift says so.
    - A bite past the plate's thickness is reported as holed: the model doesn't open the
      back.
  - **`k:bite{…, minutes=}`** is in, `minutes`, out, in one. It is the same as
    `k:immerse{…}`, `wait(minutes)`, `k:lift()`, bit for bit, and at most a day; for longer,
    use immerse and lift. It is refused while the plate is in a bath.
  - **`acid`:**
    - `"ferric"`: ferric chloride, strength `baume=` (default 38 °Bé) or `molar=`
    - `"nitric"`: nitric acid, `parts=` of water to one of acid (default 8) or `molar=`
    - `"dutch"`: Dutch mordant, by the 1911 Britannica's recipe
  - **`temp=`** °C (default 20).
  - **`face=`:** `"down"` (held off the tray's floor, ferric's default), `"upright"` (in
    a tank) or `"up"` (in a tray, nitric's and Dutch's default).
  - **`rock=true`** or `{tilt=, period=}` rocks the tray. **`feather=true`** or seconds
    between sweeps (nitric only, face up) sweeps the bubbles.
  - **`fresh=true`** makes a fresh bath. Otherwise, going back into the same acid at the
    same strength, the plate's tray keeps its acid between bites and slows as it takes up
    metal.
  - `k:immerse` returns `molar` (the oxidant left in the tray), `loaded` (g/L), `litres`,
    `since` (the plate's clock), `fresh` and `washed`.
  - **The rate** is the oxidant's mass transfer, worked out from the CRC viscosities and
    standard convection correlations, with no fitted constant.
    - Ferric chloride face down at 20 °C bites about 1 µm/min. It is fastest near 2 M and
      1.4× faster per 10 °C (its measured 24.9 kJ/mol).
    - Face up and still, the spent acid lies on the plate and the bite slows as √t.
    - Nitric acid on zinc is several times faster, slowed where its bubbles sit.
    - Dutch mordant is slow and even (its rate is an estimate).
  - **Lines** bite down and sideways under the ground (etch factor 1.2 for ferric), so a
    line widens by 2·depth/K. A narrow line bites slower as it deepens.
  - **Bitten lines have no burr:** the acid takes the needle's.
  - Loose (unmelted) rosin floats off as the plate goes in.
  - **The result** of `k:lift()` and `k:bite`: `rate` (µm/min at the start), `layer` (µm),
    `bubbles`, `depth`, `metal` (µm³ taken), `loaded` (g/L of metal in the bath), `molar`,
    `minutes` (in the bath), `washed`, `spent`.
- **`k:aquatint{box=, wait=, minutes=}`** puts the plate in the rosin dust box: `box` m
  tall (0.6), `wait` s after the cloud was stirred (30), `minutes` in it (1).
  - **Grain size.** Grains settle at their Stokes speed, so the largest still falling
    after the wait is √(18µH/ρgt): 61 µm after 5 s, 18 µm after 60 s.
  - **Cover** comes from the cloud's density (an estimate, set to give the workshops' 50 %
    on these defaults).
  - **Speckle.** Each cell takes its own random share of grains.
  - The wait and the minutes pass on the clock. The verb returns `largest` and `cover`.
- **`k:melt()`** heats the plate until the grains melt on, each spreading to 1.3 times its
  width.
  - A bite then eats the bare metal between the dots and under them. The pits deepen and
    the tone rises.
  - When the undercut reaches a dot's radius the dot is lost. The pits merge into an open
    bite and the tone falls: an over-bitten aquatint prints lighter.
  - Stop out between bites for stepped tones.
- **`k:clean()`** takes ground, varnish and rosin off with solvent (5 minutes). What the acid
  opened stays as the plate's lines and pits. The drypoint tools want a clean plate.
- **What the acid opens joins the plate's cells,** in the same width bins as drypoint lines,
  so wiping and the press will read a bitten line as they read a drypoint one. The metal the
  acid takes is counted, and the tray holds it.
- **The look.**
  - A grounded plate is dark brown: a film of bitumen over the metal, from its optics.
  - Needle lines show bright metal through it, and bitten lines their walls.
  - Varnish is near black. Rosin speckles the face.
  - `k:feel` adds the ground, the varnish and the rosin (`ground`, `rosin`). `probe` adds the
    coats, the deepest bite, the ink a wipe's yarns would leave (a guide to the tone), and
    the tray.
- **Saves, checkpoints, replay and handoff** keep the coats and the tray in the plate's
  section (`PLAT` version 2; version 1 plates still read). They also keep the bath a plate
  lies in and when it went in, so a plate saved or handed off in its bath comes out with
  the bite it would have had without the save.
  - A plate handed off in its bath travels in its tray. It bites through the `after=`
    minutes on the way, and `receive` says so.
  - A save keeps no Lua heap. After `easel finish` reads one, take the plate up by name
    (`use("key")`).
  - A session that doesn't etch refuses to receive an etched plate.
- **Physics and estimates:** notes/research/etch_physics.md.

## Damp paper: a printer's sheets

A painting begun with damp paper (its log's head says `--@ damp-paper`, beside `--@ paper`)
keeps a sheet's own water, as a printer damps a sheet to print on. An older painting
replays as it was painted, and `soak`, `blot`, `pack`, `cover`, `uncover` and `damp_of`
don't exist in it. The numbers and their sources are in notes/research/damp_paper.md.

```lua
pr = sheet{name="proof", size=430, aspect=4/3, paper={grammage=250}}
soak(pr, {minutes=20, drain=30})  -- in the tray 20 min, held up by an edge to drip 30 s
blot(pr)                          -- between two fresh blotters: no longer shiny
pack({pr, pr2}, {hours=10, dry=4})  -- overnight under a board with 4 dry sheets: one dampness
uncover(pr)                       -- out of the pack, lying on the bench: it dries now
mc, word, t = damp_of(pr)         -- 0.4 (g water per g fiber), "damp", {width=, height=, ...}
```

- **Every verb takes a surface** (what `sheet{}` or `receive` returned, or its name). It
  must be a blank sheet of paper: a drawn or painted sheet, or the linen painting, is
  refused. A sheet put aside is taken up for the verb and put aside again (as `use` does
  it), so it is aged to the clock first; the current surface stays current.
- **`soak(s, {minutes=, drain=})`** lays it in a tray: water goes in through both faces as
  fast as its sizing lets it (`paper={cobb=}`: Cobb 25 g/m² takes 50 g/m² in a minute, in
  √time), up to what its fibers and pores hold. Lifted out, it keeps a film of water on
  each face, thinning the longer it drips (30 s if `drain` isn't given). The minutes pass
  on the clock. **`soak(s, {mist=<g/m²>})`** sprays water on instead: unsized and Japan
  papers are only misted.
- **`blot(s, {passes=, blotter={mm=, grammage=}})`** presses it between two fresh blotters
  (a 0.8 mm cotton blotter if not named). They take the standing water and, as they draw,
  some of what is in its pores: a soaked 250 g/m² sheet comes to about 0.4 g/g with one
  pass. Each pass takes 20 s of hand time.
- **`pack({s1, s2, ...}, {hours=, dry=})`** interleaves the sheets (with `dry` dry sheets of
  the first's make) under a board: they come to one dampness, the water shared between
  them, and stay covered while the hours pass. A printer's pack of a pint of water to 35
  sheets of 250 g/m² BFK gives each about 0.56 g of water per g of paper.
- **`cover(s)`** puts it under plastic or blotters: it keeps its water. **`uncover(s,
  {faces=1})`** lays it on the bench (one face open) or hangs it up (`faces=2`). A soaked,
  blotted or misted sheet lies on the bench, one face open, until covered.
- **Drying.** An open face loses water as a wet surface does in the studio's air (about
  0.7 g/m² a minute): a blotted 250 g/m² sheet lying on the bench stays damp for about half
  an hour, one fresh from the tray for hours. Once its fibers start to give up their own water it dries more slowly, toward the
  studio's moisture (about 0.08 g/g).
- **`damp_of(s)`** feels it: its moisture content (g of water per g of dry fiber), a word
  (`"shiny"`: water standing on it; `"damp"`: soft through, ready to print; `"drying"`:
  stiffening and shrinking; `"dry"`), and a table: `mc`, `water` (g/m²), `standing`,
  `width` and `height` now and `made_width`, `made_height` as made (mm), `stiffness`
  (against the dry sheet's, about 0.22 when damp), `z_mpa`, `faces`, and `plastic` (true
  while damp: a shape pressed into it now sets as it dries). A look changes nothing.
- **Size.** A sheet grows as its fibers take water and shrinks as they give it up: across
  the machine direction (its width) about four times as much as along it (its height) for
  a machine-made wove sheet, equally for a laid or Japan sheet. A damp machine-made sheet
  is about 2.5 % wider than it was dry. Water past the fibers' saturation, in the pores,
  grows it no more, so a sheet keeps its size while it is damp and starts to shrink as it
  dries.
- **Between sessions.** A damp sheet handed off (`handoff`) arrives drier by its time on
  the way (`receive{after=}`) if it was left open, as it left if covered. Saves and
  checkpoints keep its water.


## Ink and wiping a plate

A painting whose log's head also says `--@ press` (new paintings do; it needs `--@ plate` and
`--@ damp-paper`, and without them it is accepted and installs nothing) may mix etching inks,
ink its plates and wipe them as a printer does. Without the switch `ink` and `tarlatan` don't
exist and a plate's printer's verbs refuse ("no ink or press"); a plate never inked saves and
replays byte for byte as before. The press itself comes next.

```lua
black = ink{{"bone black", 3}, {"raw umber", 1}, oil=0.1, stiff=1}  -- tubes from the box, in burnt plate oil
use(k)
k:warm(40)                                   -- on the hot plate at 40 °C; it cools in air from then
r = k:ink(black)                             -- by card; {by="dabber"}, {mask=m}
t = tarlatan{stiff=0.7}                      -- starched tarlatan bunched to a pad (width= mm)
w = t:wipe(k, nil, {pressure=0.5, passes=3}) -- in circles; {angle=90}: back and forth across
k:hand_wipe(nil, {passes=2})                 -- the palm, with whiting ({whiting=false}: without)
k:retrousse(rect(0, 250, 1000, 100))         -- a soft muslin drawing ink up beside the lines
print(k:feel(500, 300).words)                -- "wet ink, 12 µm in the lines and a film of 0.02 µm; …"
k:clean_ink()                                -- solvent and a rag: all the ink off
use("painting")
```

- **`ink{{tube, parts}, ..., oil=, stiff=, name=}`.** Pigment from the box's tubes in burnt
  plate oil. `oil` (0 to 0.6, default 0) is plate oil added, its share of the ink: it thins
  the pigment's colour per µm and makes the ink fatter (slower to dry). `stiff` is the oil's
  grade, 0 light (#00) to 1 heavy (#3, the default). The ink's viscosity is the oil's raised
  by its pigment (Krieger–Dougherty) and halving with every 17.5 °C of warmth: `black.viscosity`
  (Pa·s at 20 °C), `black.pigment` (its share of the ink).
- **`k:warm(t)`** puts the plate on the hot plate (2 minutes of hand time); it comes off at
  `t` °C and cools in air with a time constant from its metal and thickness (copper 1.3 mm:
  7.5 minutes). A warm plate's ink is thinner: it fills fine lines more fully and
  retroussage draws up more.
- **`k:ink(black, {mask=, by=})`** pushes ink into every line and pit (to 90 % of it, the
  air trapped at the bottom, unless the ink is too stiff to be pushed into a very fine line in
  the time) and leaves a film on the flat: the card's edge drags a film by Reynolds' slider
  law, some tens of µm of a stiff ink. Reports `added` (mm³), `film` (µm), `fill`, `temp`,
  `viscosity`, `area` (mm²), `seconds`. A coated plate (ground, rosin) refuses ink. A plate
  still in its acid (`k:immerse{}`) refuses ink, warmth and the wipes until `k:lift()`, and
  an inked plate refuses `k:immerse` and `k:bite` until `k:clean_ink()`.
- **`t:wipe(k, mask, {pressure=, passes=, angle=})`.** A tarlatan is a rag: its face loads,
  takes less as it fills, smears a little back (the plate tone), and is refolded with
  `t:refold()`. It reaches a line's ink by its yarns (a 75 µm yarn dips 4 µm into a 50 µm
  line, 19 µm into a 100 µm one, its radius into a wider one) and by its sag across a wide
  opening, which goes as the width to the fourth: lines under half a millimetre keep their ink
  below the yarns' reach, an open bite of several millimetres is wiped clean but for a band at
  its rim (none if it is shallower than a yarn's radius). Past a burr the cloth is held off
  the plate for (B·h/p)^¼, half a millimetre or so either side: wiping across a drypoint
  line keeps that ink, the velvet of a drypoint; wiping along it doesn't. More `stiff`
  starch holds the cloth stiffer: less from wide lines. Reports `before`, `after`, `taken`,
  `laid`, `kept` (mm³: `kept` is what the cloth now holds, `before − after`), `film` and
  `lines` (µm left over the area), `seconds`, `temp`; `t.load`, `t.ink` (mm³ held).
- **`k:hand_wipe(mask, {pressure=, passes=, whiting=, angle=})`.** The palm: soft skin bulges
  only a few µm into a fine line and right into an open bite, parts from the film it touches
  taking half (with whiting nearly all), and is held off past a burr further than the cloth.
  Two passes with whiting take a tarlatan's tone to almost nothing: the bare metal shines.
- **`k:retrousse(mask, {passes=})`.** A soft cloth's fibre ends hang into the lines and draw
  ink up by capillarity (more from a warm plate), laying it on the plate within half a
  millimetre around: lines soften and spread.
- **Drying.** The ink dries as an oil film of its thickness, on the clock: it wipes for hours,
  and a day or two later it has set and nothing moves it. A plate handed off inked
  (`handoff`) arrives aged by its time on the way: received three days later, its ink has
  dried in.
- **`k:feel`** adds the ink (`ink`, `film`, `cure` and words: wet, tacky or set); `probe` adds
  the ink in the lines and the film over the area, and the plate's temperature. `look --surface`
  shows the ink as a Kubelka–Munk film over the metal, black in the lines.
- **Hand time.** From Cassatt's account of printing with Leroy (15–20 minutes to ink and wipe
  each plate of a proof, plates about 35 × 21 cm): the card 277 mm² a second, a tarlatan pass
  416, a palm pass 555; a small plate takes proportionally less.
- **Saves, checkpoints, replay and handoff** keep the ink in the plate's section (`PLAT`
  version 3: version 2's layout, the coats with their bath slot, then the ink; versions 1 and
  2 still read and are still written for plates never inked). The
  tarlatans are held with the rags: a failed chunk puts them back. A session without the switch
  refuses to receive an inked plate.
- **Physics and estimates:** notes/research/intaglio_physics.md §5.6.

## The journal

`notes/journal.md` is your working journal. `note` appends an entry
stamped with the painting's time, such as `day 2, 09:40`. To revise what
is already there, call `note` with `replaces`, the exact passage to
change, and `text`, what takes its place. Writing a note doesn't advance
painting time.

The journal comes back to you as written, in later sittings and after a
condensed conversation, so a lesson in it is only as good as what
supports it. Keep what you saw apart from why you think it happened. When
later evidence contradicts an entry, revise it; earlier versions are kept.
