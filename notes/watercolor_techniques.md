# Watercolor techniques at this easel

How the watercolor painter's techniques are done with this easel's tools, and
where the engine differs from real paper. `notes/watercolor_easel_guide.md`
says what each call does; `notes/research/watercolor_physics.md` says how real
watercolor behaves. What you paint is yours to choose. In the examples,
`<pigment>` stands for any id that `palette()` returns.

## Plan before you paint

- **Try it on the scratch paper** (`scratch: true`): a load, a stroke, a wash
  into wet paper, a lift. The scratch sheet has the painting's size, kind and
  seed, and its own brushes and clock. Lay the underlayer the trial will go
  over and give it the same waiting time as on the painting, then compare
  the two in a 1:1 crop. Paint what worked on the painting with the same
  load, path, pressure and timing.
- **Start a clean sheet** with `new_scratch: true` when the scratch paper is
  full or when a trial needs dry, untouched paper.
- **Look at 1:1.** `look` with `crop="x0,y0,x1,y1"` returns cells at 1:1,
  which shows edges, granulation and dry-brush breaks. `mode="relief"` rakes
  light across the tooth. Looking does not advance the clock.

## Paper

- `paper{kind="hot-press"}`, `"cold-press"` (the default) or `"rough"` is
  chosen once, before the first mark. In this engine the kind sets the
  height of the tooth (relative amplitude 0.3 hot-press, 0.72 cold-press,
  1.0 rough) and its fiber texture. Absorption, evaporation and capillary
  spread use the same settings on all three kinds; there is no sizing
  setting.
- `seed=` changes the pattern of the tooth. The scratch paper copies it.

## Time and the stages of wetness

- **The clock is fast.** Real paper takes minutes to pass from shiny to dry
  (the physics note's table). Here a measured stroke went from wet to damp in
  about **1 second** of painter-clock `wait` and to dry in about **2.5
  seconds**, sampled every 0.25 seconds. That stroke was a 150-cell round
  stroke on the default paper, width 42, pressure 0.35, loaded with
  10 microliters of water and 10 mass units of one pigment. A wetter or
  larger area stays workable longer.
- **The stroke dries as it is laid.** Point times and `duration` are painter
  clock too. In a slow stroke the first contacts have begun drying before
  the last ones land. Give a stroke that should stay wet along its length a
  short duration.
- **Reading the stage.** `balance()` returns `max_surface_water`,
  `max_wetness` and `dry`. Standing water above 0.002 means some of the
  sheet is still wet; 0.002 or less with wetness left is damp; `dry` is true when wetness
  is below 0.02 everywhere. These are maxima over the whole sheet, so any
  wet passage elsewhere sets them. To time one passage, lay it alone on the
  scratch paper and step the clock:

  ```lua
  for i = 1, 16 do
    local t = wait(0.25)
    local s = balance()
    print(t, s.max_surface_water, s.max_wetness, s.dry)
  end
  ```

- Wall-clock time between calls and looking leave the sheet as it was.

## Loading and laying paint

- `b = brush{name="<name>", width=<cells>}` makes a round brush; the same name
  later returns it with its remaining load. A new name is a clean brush.
- `b:load{water=<µL>, pigments={["<pigment>"]=<mass>}}` adds to the load.
  The brush releases water and pigment from what it holds, and `b:status()`
  shows what is left. The brush runs dry, and a
  stroke or wash can end before its path does.
- `b:stroke(path, {pressure=..., duration=...})`. Each point can carry its own
  pressure and time, `{x, y, pressure, t}`. Pressure widens or narrows the
  contact around the nominal width.
- `b:wash{x=, y=, width=, height=}` lays a rectangle as rows of strokes;
  `grade=` thins the pigment from top to bottom (`1` is even).

## Wet-in-wet

- **Wet the area first.** Load a brush with water and no pigment
  (`b:load{water=<µL>}`) and lay a wash or stroke, or put down clean water with
  `drop{x=, y=, water=<µL>, radius=<cells>}`. Then lay the pigment while the
  area is still wet: for a stroke-sized area that is within about a second
  of painter clock, longer for a flooded one.
- **Charge a wet stroke.** A second loaded brush laid into a stroke that is
  still wet joins its water, and the two loads flow together where they
  meet.
- **Mixtures.** A brush carrying several pigments lays one blended color with
  the averaged granulation, staining and density of its load. Where two
  colors meet wet, each cell carries one blended set as well. Pigments in a
  mixture do not separate by density here.

## Edges and backruns

- **Hard edges.** Water evaporates faster at the boundary of a wet area, and
  pigment there settles out as it dries, so a stroke laid on dry paper dries
  with a darker rim.
- **Soft edges.** Lay the stroke into wet paper, or into the still-wet edge of
  the passage beside it, before either reaches damp.
- **Backruns (blooms).** Clean water or a wetter load added to paint that has
  lost its standing water but is still damp spreads into it and pushes
  pigment to a darker, lobed rim. `drop{}` is the direct way. The wetness of
  damp paper fades over time, so the window for a backrun is short at this
  clock. Find it on the scratch paper with the same load and wait.
- Water dropped on dry paint finds nothing to move (see Glazing).

## Granulation

- Granulating pigments settle into the valleys of the tooth, most in slow,
  shallow water. The texture is fixed into the paper when the cell dries and
  stays visible under later glazes. A taller tooth (rough) gives stronger
  settling. The engine has no separate flocculation; clumping shows only as
  this settling.
- The engine's library values, with its own thresholds for these words:

| Property | Pigment ids |
|---|---|
| Granulating (0.5 or more) | `yellow-ochre`, `cobalt-violet`, `ultramarine-violet`, `french-ultramarine`, `cobalt-blue`, `cerulean-blue`, `viridian`, `terre-verte`, `raw-sienna`, `burnt-sienna`, `raw-umber`, `burnt-umber`, `indian-red`, `buff-titanium` |
| Staining (0.65 or more) | `indian-yellow`, `pyrrol-scarlet`, `quinacridone-rose`, `alizarin-crimson`, `quinacridone-burnt-orange`, `dioxazine-violet`, `phthalo-blue`, `prussian-blue`, `indanthrone-blue`, `phthalo-green` |
| Opaque (scattering 1.2 or more): hides the paper and lightens what it goes over | `cadmium-yellow`, `naples-yellow`, `cadmium-orange`, `cadmium-red`, `cerulean-blue`, `indian-red`, `buff-titanium` |
| Light (density 0.3 or less): stays in the water and travels far in blooms | `hansa-yellow`, `indian-yellow`, `green-gold`, `pyrrol-scarlet`, `quinacridone-rose`, `alizarin-crimson`, `quinacridone-burnt-orange`, `dioxazine-violet`, `phthalo-blue`, `prussian-blue`, `indanthrone-blue`, `phthalo-green`, `sap-green`, `hookers-green`, `neutral-tint` |

## Lifting

- `b:lift(path)` lays water from the brush along the path and loosens
  pigment that has settled but not dried. The lift loosens pigment only
  where there is water on the paper, so load the brush with water and it
  brings its own. Its scrub effect fades within a fraction of a second of
  painter clock.
- Staining pigment resists the lift and gives up only part of its deposit;
  non-staining pigment lifts nearly clean.
- **Lifted pigment stays on the sheet.** The brush does not take it up, and
  there is no blotting (`blot` is unavailable), so loosened pigment moves
  with the water and settles again nearby.
- **Dry paint cannot be lifted.** Once a cell is dry its paint is part of the
  paper. Lift before the area dries.

## Glazing

- Wait until `balance().dry` is true (or `glaze()` succeeds; it refuses while
  the sheet is wet). A stroke over dry paint then lays a new
  layer whose color combines with what is under it.
- Dried paint here does not redissolve: water and new paint over it leave it
  as it is, so a glaze never stirs the layer beneath. Real paint can
  redissolve and tint the new layer (the physics note's staining section).
- Opaque pigments hide the paper and lighten what they cover; transparent
  ones let what is under them show through. Try a glaze over the same
  underlayer on the scratch paper first.

## Dry brush

- `b:stroke(path, {dry=true})`, or a brush with almost no water left, makes
  dry contact: pigment lands on the high points of the tooth and skips the
  valleys. The brush keeps its water. The paper's kind and seed decide
  where the stroke breaks.
- Pressure changes the contact width. Check the mark in a 1:1 crop and in
  `relief`, on the scratch paper first.

## Limits of this engine

- **One dried layer.** Every dried pass folds into a single paper
  reflectance. The engine keeps no separate glaze layers and no record of
  which pigment lies where once it has dried.
- **No blotting** and no salt. Neither call is available.
- **Pigment is not conserved while it flows.** The amount of pigment can
  drift as paint moves with the water; `balance()` reports
  `conserved=false`.
- **No sizing or tilt.** Paper kinds differ in tooth and fiber texture, and
  no call tilts the sheet or changes its sizing.
- **Fast clock.** Stages that take minutes on real paper pass in seconds
  here, so time wet work by `balance()` and scratch trials.

## Finishing

- Let the sheet dry (`balance().dry`) before the last look. The studio viewer
  shows the painting in Gallery light by default; `look` with
  `mode="gallery"` shows the same.
