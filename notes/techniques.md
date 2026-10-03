# Techniques at the easel

How the oil painter's techniques are done with this easel's tools, and the
traps that cost a painting time. `notes/easel_guide.md` says what each tool
does; this says what to do with them. None of it is a recipe for a picture:
the picture is yours.

## Plan before you paint

- **Sketch first.** A session named `sketch…` (`easel open sketch-1`) paints
  at a quarter of the width, about 16 times faster. Try three or four
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

## Color: broken, not mixed flat

- **Many close piles.** Knife three to five piles of neighboring colors
  (warmer, cooler, lighter, darker) and lay them as separate strokes over
  the same area; the eye mixes them. `mix_jitter=` in `work` varies each
  dip; a pile is never perfectly even.
- **Two colors on one brush.** `b:load(p2, 0.5, {side=1, share=0.4})`, or
  `work{second={pile=p2, side=1, share=0.45}}`: each stroke carries both,
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

- Stiff paint holds the brush's marks: blot it (`blot=0.3`) or use it as it
  comes from the tube, with no medium. A full brush lays more with `lay=4`
  to `lay=16`; a coarse hog brush (larger `hair`) leaves coarser ridges.
- In stiff paint the hairs gather into clumps: strokes lie in ridges and
  furrows, walls rise along their sides, a bead where they end. Fluid paint
  levels flat: that is a glaze's job, not impasto's.
- Check it with `look` and `mode: "relief"` (raking light from the upper
  left; `light: "45,15"` for a lower, other light). Relief that only shows
  in raking light is real but quiet; in the plain look the color carries it.
- Let impasto set before painting into it, unless you want the new stroke
  to drag the old one along.

## The knife

- `k = knife{width=30}; k:load(p, 1.0); k:lay(points, {pressure={0.5, 0.5}})`
  lays a slab with a flat top and crisp edges: light pressure thick and
  short, firm pressure thin and long.
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
  lead white and umber fast, lakes and vermilion slow, poppy oil slower than
  linseed, thin paint faster than thick, an absorbent ground faster still.
- A painting can take months of painted time. Let layers dry between
  campaigns; that is what lets the next one sit on top.

## Finishing

- An unvarnished picture keeps its matte and glossy passages as painted.
  A varnish (where the easel has one) makes it all glossy: darks deepen.
