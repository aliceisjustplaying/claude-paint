# Techniques at the easel

How the oil painter's techniques are done with this easel's tools, and the
traps that cost a painting time. `notes/easel_guide.md` says what each tool
does; this says what to do with them. None of it is a recipe for a picture:
the picture is yours.

## Plan before you paint

- **Sketch sessions (developer replay build).** A session named `sketch…`
  (`easel open sketch-1`) paints at a quarter of the width, with one
  sixteenth as many pixels. The painter's studio has one painting and does
  not expose separate sketch sessions.
- **Plan in values.** `look` with `mode: "value,squint"` shows the picture as
  big masses of light and dark. If the masses don't hold there, no detail
  will save them.
- **Coordinates.** A look's pixels are not canvas units. Use `grid: true` (or
  a crop with `grid: 10`) to read positions off a look before you aim a mask
  at something you saw.

## The whole picture, every time

- **Step back after every campaign** and ask whether the picture still says
  what it is for (height, glare, distance, weather), not only whether the
  last thing you fixed is fixed. Then `survey` it at full detail: what you
  can't see in a whole view is still on the canvas.
- **The full value range.** Check `mode: "value,squint"`: there should be
  real darks and real lights, not everything in the middle. A picture of
  light needs darks for the light to read against.
- **The sky is a light source.** It has a gradient (warmer, lighter toward
  the light) and it explains every light and shadow below it. A flat band of
  one tone is dead; a ruler-straight horizon is too hard: lose it in places.
- **Forms meet each other physically.** Where a slope ends at water there is
  a drop, a face, shadow and foam; where a path meets grass there is an edge
  that belongs to the ground. A boundary drawn as a mask's edge and nothing
  else flattens the space.
- **Composition**: avoid a centered target. Unequal masses, a dominant side,
  one place that is clearly the most important.

## Forms and shadows

- **A form is one mass.** Build a rounded thing (a stack, a tree, a figure)
  as one shape whose parts flow into each other, modeled from light to
  shadow, not as parts placed on parts (a body with a roof on top reads as a
  hut). `form{}` and its solids give a single lit volume to paint by.
- **Shadows take color from around them**: a shadow on a sunlit field is
  warmed near its base by light thrown up from the field, cooler where it
  meets the sky's light. Grade it with separate passes using different
  `pile=` values, don't fill it with one violet, and keep it a different
  value and temperature from the object casting it.
- **Distance lightens and cools**: far hills and trees paler and bluer than
  near ones, all of a range alike.

## Glazing and scumbling (over dry paint)

- **Glaze**: a transparent paint (lakes, viridian, ultramarine, a little of
  anything) with plenty of medium (0.7 to 0.85), `hand="glaze"`, over a dry
  passage. It deepens and saturates without hiding the light underneath; it
  rescues a chalky passage. Over impasto it pools in the valleys and thins
  on the ridges, which then shine through.
- **Scumble**: an opaque light paint dragged thin (`hand="scumble"`, little
  load) over a dry darker one, broken by the surface: air, haze, light on
  water.
- Glazes and scumbles want the passage under them **dry** (`wait` until
  `drying` says so): over open paint they just mix.

## Keeping control

- **Clip** a pass near anything it must not touch (`clip=` a mask). The body
  hands carry paint past their mask's edge by as much as a stroke's length:
  without a clip they paint over what is next to them.
- **Masks from earlier chunks**: a `local` mask is gone in the next chunk.
  Keep what later chunks need as globals; keep big temporary masks local
  (memory).
- **Small things** (a blossom, a figure, a sail) drawn as a neat diagram of
  petals or parts read as icons. A few thick, irregular touches of two or
  three colors read as the thing.
- **No undo**: a mistake is painted over, glazed, scraped or let dry and
  restated, as on any canvas. It costs painted time, not the picture.

## Time

- Time passes only as you paint or `wait`. Paint dries on its own clock:
  lead white and umber fast, lakes and vermilion slow, thin paint faster than
  thick. Added medium slows drying; oil species and ground absorbency are
  not drying controls in this easel.
- A painting can take months of painted time. Let layers dry between
  campaigns; that is what lets the next one sit on top.

## Finishing

- `save --gallery` (or `save --light az,el`) saves the picture lit on its
  relief, as it would be seen on a wall; plain `save` is color only.

- An unvarnished picture keeps its matte and glossy passages as painted.
  A varnish (where the easel has one) makes it all glossy: darks deepen.
