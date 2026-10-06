# Techniques at the easel

How the oil painter's techniques are done with this easel's tools, and the
traps that cost a painting time. `notes/easel_guide.md` says what each tool
does; this says what to do with them. None of it is a recipe for a picture:
the picture is yours.

## Plan before you paint

- **Try it on the scratch canvas** (`scratch: true`): a mix, a stroke, a
  glaze, a wipe. Lay the underlayer it will go over, let it dry as long as
  it has on the painting, and compare the trial with the passage in a detail
  crop. Then paint it on the painting with the same pile and the same
  handling.
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
- **Values.** Check `mode: "value,squint"`: do the large lights, middles
  and darks give the effect you want? A picture of light needs darks for the
  light to read against; a misty or high-key picture can keep a narrow range
  on purpose.
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
  small flowers, lichen, the speckle of an old wall. `b:spatter{}` throws
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
  meets the sky's light. Grade it with separate passes using different
  `pile=` values, don't fill it with one violet, and keep it a different
  value and temperature from the object casting it.
- **Distance lightens and cools**: far hills and trees paler and bluer than
  near ones, all of a range alike.

## The palette

- **Look at the board before the canvas.** `look --palette` shows each heap
  thick and thinned over black: a chalky mix, a violet "grey", a glaze that
  stains or one that hides show there, before they cost a passage.
- **Steer a mix by eye**: `p:add{}` a touch at a time, looking between,
  rather than knifing a new recipe blind.
- **Matching a passage** for a repair: start from its recipe in the log and
  lay a trial beside it. The layers since may have shifted the passage, so
  judge the laid trial in a detail crop.
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
- **Two colors on one brush.** `b:load(p, 0.8); b:load(p2, 0.5, {side=1, share=0.4})`, or
  `work(m, {pile=p, second={pile=p2, side=1, share=0.45}})`: each stroke carries both,
  side by side, mingling as it goes. `streak=` loads unevenly, in bands.
- **Wet into wet.** A stroke dragged through wet paint picks some of it up
  and carries it along: color changes within the stroke. Work an area while
  it is open (`drying(x, y)` says "open") for soft transitions; wait for it
  to set for crisp, separate strokes on top.
- **Watch the white.** Every pile with much lead white is chalky. Keep white
  for the lights; let the darks and middles be pigment.
- **Corrections stay close in value.** Keep a correction within a value step
  of what surrounds it, and small accents near their surroundings' value: a
  pale blade or rim light against a dark reads as a crack.
- **Vary the touch.** One stroke length and one brush over a whole canvas
  reads as a mechanical texture, however good the colors. Change brush size,
  stroke length, direction and pressure from passage to passage; leave some
  passages quiet.

## Impasto

- **Thick where it counts.** Most of a canvas is thin: lay-in and shadows
  lean (turpentine), middle tones as from the tube. Keep the thick, blotted
  paint for the lights and the accents. Impasto everywhere is as flat as
  impasto nowhere. `load_at=` varies the load across a pass; `scale_at` gives
  ridges of different sizes.

- Stiff paint holds the brush's marks: blot it (`blot=0.3`) or use it as it
  comes from the tube, with no medium. A full brush lays more with `lay=4`
  to `lay=16`; a coarse hog brush (larger `hair`) leaves coarser ridges.
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

## Blending

- **Blend inside one passage.** Shape the mask like the passage, with its
  edge where the paint already matches. Keep everything you want to keep
  outside the mask, dry shapes included: the blender carries wet paint onto
  them.
- **Look after each pass.** The blender lifts paint as it moves it. When
  darks go pale, texture flattens or the weave shows through, stop.
- **A seam that keeps coming back** needs another method. A smaller patch
  blended again rebuilds the box. Try something else across the whole
  transition on the scratch canvas: a thin tint of the original recipe,
  interlocking strokes of the two neighboring colors over dry paint, or the
  connected passage repainted up to its natural edges.

## Glazing and scumbling (over dry paint)

- **Glaze**: a transparent paint (lakes, viridian, ultramarine, a little of
  anything) with plenty of medium (0.7 to 0.85), `hand="glaze"`, over a dry
  passage. It deepens and saturates without hiding the light underneath; it
  rescues a chalky passage. Over impasto it pools in the valleys and thins
  on the ridges, which then shine through. Too much pigment, or any white,
  lays an opaque veil: try it small first. A glaze over the whole picture
  to pull it together has gone wrong more often than right.
- **Scumble**: light paint dragged thin over a dry darker passage, broken
  by the surface: air, haze, light on water. A light load and
  `hand="scumble"` often lay opaque dabs. The veils that blended in came
  from paint thinned with `thinner`, laid in several light passes. Try it
  on the scratch canvas first.
- Glazes and scumbles want the passage under them **dry** (`wait` until
  `drying` says so): over open paint they just mix.

## Keeping control

- **Clip** a pass near anything it must not touch (`clip=` a mask). The body
  hands carry paint past their mask's edge by as much as a stroke's length:
  without a clip they paint over what is next to them.
- **Protect what is there now.** Paint fine things (twigs, grass tips,
  figures) after the passages behind them where you can. In every later
  pass over that area, subtract each of them from the mask (`m - twigs`),
  and subtract again after `grow` or `blur`, which spread the mask back
  over them.
- **Check a mask before painting through it**: `print(m:at(x, y))` at a few
  points inside the target and just outside it, past each end of a band.
- **Masks from earlier chunks**: a `local` mask is gone in the next chunk.
  Keep what later chunks need as globals: paths, recipes, protection masks.
  Before redrawing something by eye, look for its saved path. Keep big
  temporary masks local (memory).
- **Small things** (a blossom, a figure, a sail) drawn as a neat diagram of
  petals or parts read as icons. A few thick, irregular touches of two or
  three colors read as the thing.
- **Taking paint off**: wet paint comes off with the rag (below) or the
  knife, down to the first set layer. Set paint stays: paint over it or
  glaze it. Either way it costs painted time, not the picture.

## The rag

- **It lifts wet paint down to the first set layer.** Before a wipe, check
  `drying(x, y)` under the paint as well as in it: where the layer beneath
  is still open, the rag takes that too, to the ground.
- **Dip it for a lift.** A dry rag mostly moves paint and leaves its color
  in the weave: useful for wiping lights out of a wet glaze or lay-in. To
  take a passage off, dip well (`r:dip(0.9)`), press firmly, give it two or
  three passes, then go over it again across the first direction with a
  fresh, dipped rag.
- **The spirits go in minutes.** Dampness halves every three minutes of
  painting time, and a long mask wipe takes minutes. Dip just before each
  wipe; `print(r.damp)` says how much is left.
- **A loaded face puts paint back.** `print(r.load)`: near 1, the face
  lifts little and smears. Refold (`refold=` in a mask wipe) or take a
  fresh rag.
- **The rag isn't clipped.** Its rim reaches past the mask and lays lifted
  paint on what is next to it, dry shapes included. Keep the mask clear of
  edges you need crisp. Along an edge, wipe a path parallel to it with a
  rag narrower than the strip.
- **Shape the mask like the passage.** Mask wipes start and stop a little
  inside the mask, so a rectangle leaves a frame of the wiped color. If a
  frame shows, wipe along its sides with a path.
- **Expect a trace.** Even a good lift leaves a tint, streaks along the
  wipe or a rim where it stopped. Look at a detail crop before you call it
  clean, and before you paint over it. Thick paint takes many fresh, dipped
  wipes.
- **One lift, then a new plan.** If the same passage needs lifting twice,
  change what goes on it before painting it again.

## Time

- Time passes only as you paint or `wait`. Paint dries on its own clock:
  lead white and umber fast, lakes and vermilion slow, thin paint faster than
  thick. Added medium and the slower oils (walnut, poppy) slow it; an
  absorbent ground sets thin paint laid straight on it quickly.
- **Check `drying(x, y)` at several points** along the next pass, its edges
  included: thick and thin parts of one passage dry at different rates.
- A painting can take months of painted time. Let layers dry between
  campaigns; that is what lets the next one sit on top.

## Finishing

- `save --gallery` (or `save --light az,el`) saves the picture lit on its
  relief, as it would be seen on a wall; plain `save` is color only.

- An unvarnished picture keeps its matte and glossy passages as painted.
  A varnish (where the easel has one) makes it all glossy: darks deepen.
