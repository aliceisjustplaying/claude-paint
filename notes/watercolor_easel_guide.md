# Watercolor easel

The studio has five MCP tools: `paint`, `look`, `note`, `status` and `log`.
`paint` runs a Lua chunk. A successful chunk keeps the paper, named brush loads
and material clock for the next chunk. A failed chunk leaves the committed
painting unchanged. Each chunk starts a fresh Lua VM; retrieve saved brushes
by name and define any Lua variables and functions again.

## Paper and coordinates

`paper{width=1056, height=704, kind="cold-press", seed=71}` creates the paper.
Those values are the defaults, so `paper{}` creates the same sheet. Width and
height are simulation cells. Width accepts integers from 2 to 2,400 and height
from 2 to 1,600. Paper is created once. A continuation chunk uses the existing
sheet. `kind` can be `"cold-press"`, `"hot-press"` or `"rough"`.
Seed accepts an integer from 0 to 4,294,967,295.

Coordinates and brush widths are cells, with the origin at the upper-left
corner, x increasing to the right and y downward. Fractional coordinates are
allowed. The sheet's kind and seed determine its tooth. A hardware GPU runs
the Watercolor Lab simulation.

## Palette and brushes

`palette()` returns all 36 pigment library ids. Use those exact strings in
`b:load`. Pigments have their library absorption, scattering, granulation,
staining and density values. Loads mix their optical coefficients and material
properties in proportion to the amounts still in the brush.

`b=brush{name="br1", width=24}` creates a named round brush. Width accepts
1 to 2,400 cells and defaults to 24. Pressure changes the contact radius around
that nominal width. Calling `brush{name="br1"}` again retrieves its saved
width and load. To change its width, use a new name. Names contain up to 80
letters, digits, `-`, `_` and `.`, and cannot start with `.`.

`b:load{water=10, pigments={["french-ultramarine"]=1}}` adds to the existing
load. Each call accepts 0 to 100 microliters of water and 0 to 10 arbitrary
mass units of each pigment. Several library ids can be loaded together.
The brush has a finite inventory and retains unused load between strokes and
chunks. `b:status()` reports its `width`, remaining `water` and `pigments`.
A fresh name gives a clean, empty brush.

## Marks and water

`b:stroke({{x,y}, {x,y}}, {pressure=0.35, duration=seconds})` takes a path of
1 to 2,000 points. A one-point path makes a round contact. Pressure accepts
0 to 1. Each point can also be `{x,y,pressure,time_seconds}`; point times start
at zero and increase along a moving path. Duration accepts up to 600 seconds
and must be positive for a moving stroke. By default, duration is path length
divided by 800 cells per second, with a minimum of 0.0625 seconds.

Water evolves during a stroke as its contacts are laid. `dry=true` makes
dry contact and retains the brush's water. A brush with less than 0.001
microliters of water also makes dry contact. `grade=ratio`, from 0 to 1,
reduces pigment from the top of a stroke to the bottom; 1 is an even load.

`b:wash{x=x, y=y, width=w, height=h, grade=ratio}` lays a rectangle as rows
of brush strokes. Coordinates and dimensions are cells. It accepts the same
pressure, dry-contact, grade and duration options. The brush can run out of
water or pigment before the whole area is covered.

`drop{x=x, y=y, water=amount, radius=r}` adds clean water. Water accepts
0 to 100 microliters and radius accepts 1 to 1,200 cells. Its default radius
is 12 cells. Water can move workable paint and make a backrun.

`b:lift(path)` uses the round lift contact on the same path form. It supplies
water from the brush and loosens workable pigment on the sheet. Cured paint
remains part of the paper. Lifted pigment stays in the simulated sheet;
the lift contact does not collect it into the brush. `blot` and salt are
unavailable in this API.

## Time and drying

`wait(seconds)` advances the material clock by 0 to 600 seconds per call
and returns the clock reading. `clock()` includes any fraction waiting for
the next 0.0625-second tick. Each tick advances 11 or 12 Lab substeps,
averaging 180 per painter-clock second. Looking and wall-clock time between
calls do not advance the material.

Drying depends on the amount of water, the paper and the surrounding wet
area. The measured wet-to-damp-to-dry timing for this draft will be filled
in from the Mac GPU smoke run before review.

`balance()` reports `dry`, `max_surface_water` and `max_wetness`, along with
water and brush-release inventory diagnostics. Standing surface water at or
below 0.002 with remaining wetness is damp. `dry` becomes true when wetness
is below 0.02 everywhere and suspended pigment is at or below 0.0001 per cell.
The Lab reference uses half-float fields and advection; `conserved` is false.
The diagnostic totals do not promise exact per-pigment conservation.

`glaze()` checks that the sheet is dry. The Lab engine already cures dry
pigment into the paper's reflectance; this call adds no layer and refuses
while the sheet is wet. Cured paint cannot be lifted.

## Looking and the journal

The MCP `look` tool returns a PNG of the current paper. The painter sees
Color by default. `mode="gallery"` uses Gallery light at 115/55/1 and
`mode="relief"` uses raking light at 135/25/1. A Color look also writes
display-only Gallery and Relief companions for the studio viewer, whose
default is Gallery light. Looking changes no material or clock.

`size` sets the long side of a whole-sheet look, from 16 to 2,400 pixels.
Without it, the view has one pixel per cell. `crop="x0,y0,x1,y1"` selects
two opposite corners in integer cells and returns that region at 1:1.
Supported MCP look options are `mode`, `size`, `crop` and `scratch`.

Within Lua, `look{path="look.png", mode="color", width=1200}` writes a PNG
into the studio, with width from 1 to 2,400 pixels. Lua crops use
`crop={x,y,width,height}` in output pixels. A named `save("name.json")`
captures the sheet, clock and brush loads. Successful chunks also save an
automatic checkpoint; the next chunk loads it. An identical completed
source chunk is refused, so submit new continuation code.

`note{text="..."}` appends to `notes/journal.md`, stamped with painting
time. `replaces` changes one exact, unambiguous passage already there.
`status` reports the setup and clock. `log` returns the successful Lua
chunks in order, also kept in `paintings/lua/painting.lua`.

## The scratch paper

Beside the painting stands scratch paper for trying out a mix, a stroke,
a glaze or a drying time before it goes on the painting. `paint`, `look`,
`status` and `log` work on it with `scratch: true`; without it they work on
the painting.

- The first use sets it up as the painting's paper was set up: the same
  size, kind and seed. The painting needs its own `paper{}` first.
- It is a separate sheet with its own brushes, loads, clock and log,
  `paintings/lua/scratch.lua`. Load a brush again here. Nothing done on
  it reaches the painting.
- It keeps its own clock. Time spent or waited there does not pass for
  the painting, and the painting's time does not pass for it.
- To use what worked, paint it on the painting with the same handling.
- `new_scratch: true`, with `scratch: true`, puts the current sheet aside
  and starts a fresh sheet before the chunk runs. The old log stays as
  `paintings/lua/scratch-1.lua`, then `scratch-2.lua` and so on. Each has
  its final checkpoint beside it as `scratch-1.json`, `scratch-2.json` and
  so on, so an operator can render that sheet later.
- A trial judges a passage only as far as it repeats that passage's
  conditions. Lay the underlayer you mean to work over and let it dry
  as long as it has on the painting.

Lua has math, table, string and utf8 libraries, a 128 MB memory limit and
a budget of about two million instructions per chunk. Native painting and
waiting run outside that instruction budget. Keep the same runner build
when continuing a painting.
