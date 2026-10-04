# Techniques at the easel

How the oil painter's techniques are done with this easel's tools, and the
traps that cost a painting time. `notes/easel_guide.md` says what each tool
does; this says what to do with them. None of it is a recipe for a picture:
the picture is yours.

## Plan before you paint

- **Sketch first** (for whoever runs the replay build of the easel; a
  painter's studio has one painting and its `easel open` takes no name). A
  session named `sketch…` (`easel open sketch-1`) paints at a quarter of the
  width, about 16 times faster. Try three or four
  compositions as small, quick sketches before the painting: the big shapes,
  the values, where the light is. A chunk of the painting that took two
  minutes takes seconds there.
- **Plan in values.** `look` with `mode: "value,squint"` shows the picture as
  big masses of light and dark. If the masses don't hold there, no detail
  will save them.
- **Coordinates.** A look's pixels are not canvas units. Use `grid: true` (or
  a crop with `grid: 10`) to read positions off a look before you aim a mask
  at something you saw.

## Ground

- An **oil ground** (the default) absorbs nothing: paint stays open and
  glossy longer. An **absorbent ground** (`absorbent=true`) drinks the oil of
  thin paint: matte, lean, quick to set, colors that sit on the surface;
  thick paint is barely touched. Choose before you start: it is the
  canvas's character.
- A **toned ground** (a little ochre, umber or black in the white) sets the
  middle value from the first stroke; a white ground keeps everything high
  in key and lets thin paint glow.

## Lay-in: lean, quick, thin

- Thin the paint with turpentine (`pile{..., turps=0.6}`) for the first
  drawing and masses: it flows, covers fast and leaves a thin film that
  dries before the next sitting.
- **Fat over lean.** Each layer should be at least as oily as the one under
  it: lay-in lean (turpentine, blotted), body as from the tube, glazes rich
  (medium). Lean over fat dries matte and patchy over a still-soft layer.

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

## Marks

- **A hierarchy of marks.** A few large decisive strokes (`b:gesture`), many
  middle ones, some small accents. One stroke size and one shape over a whole
  area reads as texture, not as the thing.
- **Marks get smaller with distance.** Sea, field, flower beds: give the pass
  a `scale_at` that shrinks toward the horizon or the far end, so the
  brushwork itself recedes.
- **Marks follow the form.** Leaves along the branch, water level, grass up
  the slope. Strokes that wander in every direction read as noise.
- **Light is soft and quiet.** Glare and sunlit openings have soft edges and
  little texture; busy, outlined strokes turn light into an object (a disc,
  a moon). Don't paint strokes circling a center inside the light.
- **Glitter** on water is small, sharp and dense far off, breaking into
  scattered flecks near you; big uniform dabs read as beads. Tie it to a
  light in the sky.

## Spatter

- **Flicked paint for what is scattered by nature**: spray, gravel, a field of
  small flowers, lichen, the speckle of an old wall.
  `b:spatter{at={x, y}, toward={dx, dy}}` throws
  droplets whose sizes, spacing and colors vary by themselves; aim it, don't
  place each one.
- **Thin the paint to make it fly**: a pile with medium or turpentine
  spatters; blotted paint hardly does. A hard flick gives a fine spray, a
  gentle one fat drops.
- **Mask what must stay clean** with `clip=`: droplets fly past any edge.
- A little goes a long way: a few flicks over a passage, then look.

## Forms and shadows

- **A form is one mass.** Build a rounded thing (a stack, a tree, a figure)
  as one shape whose parts flow into each other, modeled from light to
  shadow, not as parts placed on parts (a body with a roof on top reads as a
  hut). `form{}` and its solids give a single lit volume to paint by.
- **Shadows take color from around them**: a shadow on a sunlit field is
  warmed near its base by light thrown up from the field, cooler where it
  meets the sky's light. Grade it (`piles=`), don't fill it with one violet,
  and keep it a different value and temperature from the object casting it.
- **Distance lightens and cools**: far hills and trees paler and bluer than
  near ones, all of a range alike.

## The palette

- **Look at the board before the canvas.** `look --palette` shows each heap
  thick and thinned over black: a chalky mix, a violet "grey", a glaze that
  stains or one that hides show there, before they cost a passage.
- **Steer a mix by eye**: `p:add{}` a touch at a time, looking between,
  rather than knifing a new recipe blind.
- **Set out a limited palette** for a picture that should hang together; a
  few tubes mixed every way give related colors by themselves.
- **A dirty board unifies.** `palette{dirty=0.2}` to `0.4` lets each heap
  pick up a trace of the others through a sitting, as a working palette does;
  clean it (`palette{clean=true}`) when a passage needs pure color.

## Color: broken, not mixed flat

- **Many close piles.** Knife three to five piles of neighboring colors
  (warmer, cooler, lighter, darker) and lay them as separate strokes over
  the same area; the eye mixes them. `mix_jitter=` in `work` varies each
  dip; a pile is never perfectly even.
- **Two colors on one brush.** `b:load(p, 0.8)` then
  `b:load(p2, 0.5, {side=1, share=0.4})`, or
  `work(m, {pile=p, second={pile=p2, side=1, share=0.45}})`: each stroke carries both,
  side by side, mingling as it goes. `streak=` loads unevenly, in bands.
- **Wet into wet.** A stroke dragged through wet paint picks some of it up
  and carries it along: color changes within the stroke. Work an area while
  it is open (`drying(x, y)` says "open") for soft transitions; wait for it
  to set for crisp, separate strokes on top.
- **Watch the white.** Every pile with much lead white is chalky. Keep white
  for the lights; let the darks and middles be pigment.
- **Vary the touch.** One stroke length and one brush over a whole canvas
  reads as a mechanical texture, however good the colors. Change brush size,
  stroke length, direction and pressure from passage to passage; leave some
  passages quiet.

## Impasto

- **Thick where it counts.** Most of a canvas is thin: the lay-in and the
  shadows thin, the middle tones as from the tube. Keep the thick paint for
  the lights and the accents. Impasto everywhere is as flat as impasto
  nowhere. `load_at=` varies the load across a pass; `scale_at` gives
  ridges of different sizes.

- Stiff paint holds the brush's marks: use it as it comes from the tube,
  with no medium. A full brush lays more with `lay=4` to `lay=16`; a coarse
  hog brush (larger `hair`) leaves coarser ridges.
- In stiff paint the hairs gather into clumps: strokes lie in ridges and
  furrows, walls rise along their sides, a bead where they end. Fluid paint
  levels flat: that is a glaze's job, not impasto's.
- Check it with `look` and `mode: "relief"` (raking light from the upper
  left at 25°: it exaggerates, as a raking lamp does) and with
  `mode: "gallery"` (55°, as the picture is seen on a wall). Judge the
  picture under the gallery light; use raking light to inspect the surface.
- Let impasto set before painting into it, unless you want the new stroke
  to drag the old one along.

## The knife

- `k = knife{width=30}; k:load(p, 1.0); k:lay(points, {pressure={0.5, 0.5}})`
  lays a slab with a flat top, torn where it parts from the blade (its ends,
  its leading edge): light pressure lays it thick, and the paint runs out
  sooner; firm pressure thin, and it goes further. The points set its path.
- Pulled lightly over **dry** impasto, it catches only the ridges: broken
  color over a textured underlayer, a sparkle of light paint on dark.
- `k:scrape(points, {pressure=1})` takes wet paint off down to what is dry:
  to restate a passage, or to leave a ghost of it.

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
  lead white and umber fast, lakes and vermilion slow, thin paint faster
  than thick.
- A painting can take months of painted time. Let layers dry between
  campaigns; that is what lets the next one sit on top.

## Finishing

- `save --gallery` (or `save --light az,el`) saves the picture lit on its
  relief, as it would be seen on a wall; plain `save` is color only.

- An unvarnished picture keeps its matte and glossy passages as painted.
  A varnish (where the easel has one) makes it all glossy: darks deepen.
