# Watercolor easel

The studio has seven MCP tools: `paint`, `look`, `note`, `status`, `log`, `reference` and `reference_list`.
`reference_list` lists the reference library's entries by `id` and title; `reference` with `query` finds the entries that hold all its words, and with `id` reads one a page at a time, following `next_page` until it is null.
`paint` runs a chunk of Lua. A chunk that succeeds keeps the paper, your
named brush loads and the material clock for the next chunk. A chunk that
fails leaves the committed painting as it was. Each chunk starts in a fresh
Lua VM: get your saved brushes back by name, and define your Lua variables
and functions again.
Lua's `io` library is unavailable; output uses `print`.

In the examples below, `<pigment>` stands for an id from `palette()`. Other
numbers in the examples show how a call is written; the values are yours to
choose.

## Paper and coordinates

```lua
paper{width=<cells>, height=<cells>, kind="<kind>", seed=<seed>}
```

- `width` and `height`: the sheet's size in simulation cells. Width accepts
  integers from 2 to 2,400 and height from 2 to 1,600.
- `kind`: `"cold-press"`, `"hot-press"` or `"rough"`.
- `seed`: an integer from 0 to 4,294,967,295.

The defaults are width 1,056, height 704, `"cold-press"` and seed 71, so
`paper{}` makes that sheet. The paper is created once; a continuation chunk
paints on the existing sheet. `paper{}` on a sheet that already has paper
stops the chunk with "paper already exists". A chunk can run before the
painting has paper: one that only prints, such as `print(1)` or a chunk
that prints the ids `palette()` returns, succeeds and goes into the log like
any other successful chunk. `brush{}` and its marks, `drop{}`, `wait()`,
`clock()`, `glaze()` and Lua `look{}` need paper, so call `paper{}` before
them. The scratch paper takes its size, kind and seed from the painting's
paper, so it can be used once the painting has paper.

Coordinates and brush widths are in cells. The origin is the upper-left
corner; x increases to the right and y downward. Fractional coordinates are
allowed. The sheet's kind and seed set its tooth. The Watercolor Lab
simulation runs on a hardware GPU.

## Palette and brushes

`palette()` returns the ids of all 36 pigments in the library. Use those
exact strings in `b:load`. Each pigment carries its library values for
absorption, scattering, granulation, staining and density. A load mixes the
optical coefficients and material properties of its pigments in proportion
to the amounts still in the brush.

```lua
b = brush{name="<name>", width=<cells>}
```

This makes a named round brush. Width accepts 1 to 2,400 cells and defaults
to 24. Pressure changes the contact radius around that nominal width.
Calling `brush{name="<name>"}` again gets back its saved width and load; to
change the width, use a new name. A name has up to 80 letters, digits, `-`,
`_` and `.`. It can't start with `.`.

```lua
b:load{water=<microliters>, pigments={["<pigment>"]=<amount>}}
```

A load adds to what the brush already holds. Each call takes 0 to 100
microliters of water and 0 to 10 arbitrary mass units of each pigment, and
can load several pigments together. There is no cap on what a brush can
hold: five loads of 100 microliters leave it holding 500 microliters.
Strokes use the load up as they lay it, and the brush keeps what it hasn't
laid between strokes and chunks. `b:status()` reports its `width` and its
remaining `water` and `pigments`. A new name gives a clean, empty brush.
With a width-24 round at pressure 0.35 on cold-press paper, 40 µL water/1 French Ultramarine unit and 80 µL/2 units each carried a fading 936-cell stroke, leaving 3.30 and 9.01 µL.

## Marks and water

```lua
b:stroke({{x, y}, {x, y}, ...}, {pressure=<0..1>, duration=<seconds>})
```

A stroke takes a path of 1 to 2,000 points; a one-point path makes a round
contact. Pressure takes 0 to 1 and defaults to 0.35. A point can also be
`{x, y, pressure, time_seconds}`; point times start at zero and increase
along a moving path. Duration takes up to 600 seconds and must be positive
for a moving stroke. By default a stroke lasts its path length divided by
800 cells per second, with a minimum of 0.0625 seconds.

Water moves and dries during a stroke as its contacts are laid. `dry=true`
makes a dry contact and keeps the brush's water. A brush holding less than
0.001 microliters of water also makes a dry contact. `grade=<ratio>`, from
0 to 1, reduces the pigment from the top of a stroke to the bottom; 1 is an
even load.

`b:wash{x=, y=, width=, height=, grade=}` lays a rectangle as rows of brush
strokes. Its coordinates and dimensions are in cells. Row centers are at
most 0.7 brush widths apart. Each row's path runs from the rectangle's left
side to its right, so the brush's contact reaches past both sides. It
takes the same pressure, dry-contact, grade and duration options. Each row
starts with the brush's starting water and pigment load. That load is
gradually replenished along the row so a loaded brush can cover the
rectangle. By default a row lasts its length divided by 2,800 cells per
second, with a minimum of 0.015625 seconds; `duration` sets the time of
each row. A painting begun on an earlier version of the easel keeps the
wash timing it started with.

`drop{x=, y=, water=<microliters>, radius=<cells>}` adds clean water: 0 to
100 microliters. Radius accepts 1 to 1,200 cells, 12 by default. The water
lands as a soft patch that fades out 1.8 times the radius from the center.
A radius-40 drop's water lands up to 72 cells from its center. For a
drop's patch to land inside a shape, place its center at least 1.8 radii in
from each edge: a radius-40 drop centered 40 cells inside an edge lands
water up to 32 cells past that edge. Water can move workable paint and make a
backrun. A drop lands at once and takes no painter-clock time. A drop
whose patch misses the sheet entirely stops the chunk with "drop is outside
paper". Stroke paths can run past the edges. A painting begun on an earlier
version of the easel keeps the drop it started with, whose water stops at
the radius.

`b:lift(path)` lays the round lift contact along a path of the same form.
It supplies water from the brush, loosens workable pigment on the sheet and
removes some suspended pigment from the sheet. Removed pigment is not added
to the brush's inventory. Cured paint has become part of the paper and
can't be lifted. `blot` and salt are unavailable here.

## Time and drying

`wait(seconds)` advances the material clock by 0 to 600 seconds a call and
returns the clock reading. `clock()` includes any fraction still waiting for
the next 0.0625-second tick. Each tick runs 11 or 12 Lab substeps, 180 per
painter-clock second on average. Looking and wall-clock time between calls
leave the material alone: the sheet stays as it was at the end of the last
call.

On the default paper, a test stroke on the Mac GPU became damp after about
1 second of further `wait` and dry after about 2.5 seconds, sampled every
0.25 seconds. It was a 150-cell stroke with a round brush of width 42 at
the default pressure 0.35, loaded with 10 microliters of water and 10 mass
units of French Ultramarine. The brush still held pigment when the stroke
ended. For a stroke like that, the wet-into-wet window is roughly the
first second of further painter-clock time. Earlier contacts in a stroke
have begun drying while the later ones are being laid.

Drying depends on the amount of water, the paper and the surrounding wet
area. A wetter area can stay workable longer.

`balance()` reports `dry`, `max_surface_water` and `max_wetness`, along with
diagnostics for the water and brush-release inventory. Standing surface
water at or below 0.002, with wetness remaining, counts as damp. `dry`
becomes true when wetness is below 0.02 everywhere and suspended pigment is at or
below 0.0001 per cell. The Lab reference uses half-float fields and
advection; `conserved` is false. The diagnostic totals don't promise
exact per-pigment conservation.

`glaze()` checks that the sheet is dry and refuses while it is wet. The
Lab engine already cures dry pigment into the paper's reflectance, so the
call adds no layer.

## Looking and the journal

The `look` tool returns a PNG of the paper as it is now. You see Color by
default. `mode: "gallery"` shows it in Gallery light (azimuth 115°,
elevation 55°, intensity 1) and `mode: "relief"` in raking light (135°,
25°, 1). A Color look also writes display-only Gallery and Relief
companions for the studio viewer, which shows Gallery light by default.
Looking changes neither the material nor the clock.

`size` sets the long side of a whole-sheet look, from 16 to 2,400 pixels.
Without it, the view has one pixel per cell. `crop: "x0,y0,x1,y1"` gives two
opposite corners in integer cells and returns that region at 1:1; `size`
has no effect on a crop. The `look` tool takes `mode`, `size`, `crop` and
`scratch`.

Within Lua, `look{path="<name>.png", mode="color", width=<pixels>}` writes a
PNG into the studio, from 1 to 2,400 pixels wide. A Lua crop is
`crop={x, y, width, height}` in output pixels. `save("<name>.json")`
captures the sheet, the clock and the brush loads under that name. Every
successful chunk also saves an automatic checkpoint, which the next chunk
loads. A chunk whose exact source has already run is refused, so each
continuation needs new code.

`note` appends an entry to `notes/journal.md`, stamped with the painting's
time. To revise an entry, call `note` with `replaces`, the exact passage
to change, which must appear there once, and `text`, what takes its
place. `status` reports the setup and the clock. `log` returns the
successful Lua chunks in order; they are also kept in
`paintings/lua/painting.lua`.

## The scratch paper

Beside the painting stands scratch paper for trying out a mix, a stroke,
a glaze or a drying time before it goes on the painting. `paint`, `look`,
`status` and `log` work on it with `scratch: true`; without it they work on
the painting.

- The first use sets it up as the painting's paper was set up: the same
  size, kind and seed. Each scratch sheet, the first and every fresh one,
  comes with its paper, so scratch chunks start without `paper{}`.
- It is a separate sheet with its own brushes, loads, clock and log,
  `paintings/lua/scratch.lua`. Load a brush again here. Nothing done on
  it reaches the painting.
- It keeps its own clock. Time spent or waited there doesn't pass for
  the painting, and the painting's time doesn't pass for it.
- To use what worked, paint it on the painting with the same handling.
- `new_scratch: true`, with `scratch: true`, puts the current sheet aside
  and starts a fresh one before the chunk runs. The old log stays as
  `paintings/lua/scratch-1.lua`, then `scratch-2.lua` and so on. Each has
  its final checkpoint beside it as `scratch-1.json`, `scratch-2.json` and
  so on, so an operator can render that sheet later.
- A trial judges a passage only as far as it repeats that passage's
  conditions: lay the underlayer you mean to work over, and let it dry
  as long as it has on the painting. For a wash or glaze, those conditions
  also include the passage's whole area and wetness, with the same rows,
  overlaps and top-ups. One stroke shows the load laid once; where rows or
  passes overlap, the paper takes it two or more times. A larger area also
  takes longer to lay, so its first rows have longer to dry before the
  last ones land.

## Lua

Lua has the math, table, string and utf8 libraries, a 128 MB memory limit
and a budget of about two million instructions per chunk. Native painting
and waiting run outside that instruction budget. Keep the same runner build
when continuing a painting.

### The 12-minute limit

A `paint` call has 12 minutes of wall time, on either sheet. A chunk still
running at 12 minutes is stopped, and the call replies "the easel didn't
answer; `log` shows whether the chunk ran". No number of strokes is known
to be safe: in earlier sittings, some chunks of a few hundred strokes or
fewer were stopped, while larger ones finished. The sheet stays as it was
between chunks, so a long pass can continue in the next chunk while its
paint is still wet. Lay a long wet pass over several chunks, each ending
after a whole row or wash, and start the next chunk at the row that
follows. After a stopped chunk, the pass picks up from the end of the last
chunk that finished.
