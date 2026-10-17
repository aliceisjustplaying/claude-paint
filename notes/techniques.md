# Techniques at the easel

How the oil painter's techniques are done with this easel's tools, and the
traps that cost a painting time. `notes/easel_guide.md` says what each tool
does; this says how to use them. What you paint is yours to choose.

## Plan before you paint

- **Try it on the scratch canvas** (`scratch: true`): a mix, a stroke, a
  glaze, a wipe. Lay the underlayer it will go over, let it dry as long as
  it has on the painting, and compare the trial with the passage in a detail
  crop. Then paint it on the painting with the same pile and the same
  handling.
- **Plan in values.** `look` with `mode: "value,squint"` shows the picture as
  big masses of light and dark. Make the masses hold there before you add
  detail.
- **Coordinates.** A look's pixels are not canvas units. Use `grid: true` (or
  a crop with `grid: 10`) to read positions off a look before you aim a mask
  at something you saw.

## Before another pass: a decision checklist

1. **Check the paint state.** Use `drying(x, y)` along the next pass and
   at its edges: `"open"` blends and lifts; `"setting"` is stiff and barely
   blends; `"tacky"` has set and grabs the brush; `"dry"` is touch-dry.
   The call reads the top film. Where no wet paint lies, as in a gap
   between strokes, it reads the set layer beneath. A set layer under wet
   paint can still be tacky and grab the brush, so read a nearby gap too.
   Lay into open paint when you want strokes to fuse; wait for it to set
   when you want them separate. If tacky paint breaks the stroke, wait
   longer. See the guide's [Time](easel_guide.md#time).
2. **Diagnose a failed deposit in a detail crop.** Check the load, the
   paint the brush passed through and the layer beneath.
   - Gaps between marks: if the brush runs dry, use shorter strokes and
     reload more often. If gaps remain, check brush contact and coverage;
     extra coverage can add dabs and relief.
   - A continuous thin film showing the undercolor: test less thinning,
     a fuller deposit or a mixture with stronger hiding. Evidence for
     these fixes is limited, so compare them on the scratch canvas first.
   - A mixture changed by paint carried on the brush: use a fresh brush.
     If open underpaint caused the pickup and you want separate color,
     wait for it to set. Check the pile itself before changing its recipe.
3. **Prepare a transition.** Mix a few in-between tones, lay them and
   look. Blend while the paint is open, as [Blending](#blending) describes,
   and stop at the signs listed there. In dry paint, build the transition
   with new paint.
4. **Choose a repair.** If further handling keeps mixing an unsuccessful
   wet deposit, scrape or lift it before restating it. The knife scrapes
   wet paint down to set paint; the rag lifts it and leaves a thin
   tint. Inspect what remains before repainting. See
   [The knife](#the-knife) and [The rag](#the-rag) for handling and limits.
5. **Test a doubtful change.** Use the scratch procedure in
   [Plan before you paint](#plan-before-you-paint), matching the underpaint
   and its drying state. Change one condition, compare the results and
   carry the handling that worked over to the painting. Leave the passage
   alone if another pass has no clear benefit.

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

- **Step back after every campaign** and ask whether the whole picture still
  does what you meant it to, as well as whether the last thing you fixed is
  fixed. Then `survey` it at full detail: what you can't see in a whole view
  is still on the canvas.
- **Values.** Check `mode: "value,squint"`: do the large lights, middles
  and darks give the effect you want? Light reads against darks; a high-key
  picture can keep a narrow range on purpose.
- **Light relationships.** Relate the light and shadow of each passage to
  the effect you want. A uniform band or a firm boundary can serve the
  picture; judge it against the whole.
- **Forms meet through relationships.** Overlap, color and edge handling
  separate or connect adjacent planes. A boundary that is only a mask's
  edge leaves their spatial relationship unstated.
- **Composition**: judge how the masses work together for the effect you
  chose. Where the emphasis sits, central or off-center, is yours to
  choose.

## Marks

- **A hierarchy of marks.** A few large decisive strokes (`b:gesture`), many
  middle ones, some small accents. One stroke size and one shape over a whole
  area reads as texture.
- **Mark scale and distance.** Marks that shrink through a passage
  (`scale_at`) can suggest recession; marks of similar scale throughout
  can compress depth.
- **Mark direction and form.** Direction can turn with a plane or bind
  neighboring planes through a shared rhythm. Strokes that wander in
  every direction read as noise.
- **Light and texture.** Bright light reads as light with soft edges and
  little texture. Busy or outlined strokes inside it, or strokes circling
  a center, turn it into an object.

## Spatter

- **Flicked paint makes scattered marks.**
  `b:spatter{at={x, y}, toward={dx, dy}}` throws droplets whose sizes,
  spacing and colors vary by themselves: aim the flick and let the droplets
  fall.
- **Thin the paint to make it fly**: a pile with medium or turpentine
  spatters; blotted paint hardly does. A hard flick gives a fine spray, a
  gentle one fat drops.
- **Mask what must stay clean** with `clip=`: droplets fly past any edge.
- A few flicks over a passage, then look.

## Forms and shadows

- **A form can read as one mass.** Relate its parts through color and
  overlap so they hold together. `form{}` gives a lit-volume guide, and
  color modulation builds solidity with or without that lighting.
- **Shadows take color from surrounding light.** With warm light reflected
  up near its base and cooler light from above, a shadow is warmer near its
  base and cooler above. Grade it in separate passes with different `pile=`
  values, and judge its value and temperature against the object casting
  it and the passages around it. One violet for every shadow is the usual
  trap.
- **Distance and color.** Distance lightens and cools: far forms are paler
  and bluer than near ones and closer to each other in tone. Similar
  intensity in near and far planes can compress depth.

## The palette

- **Look at the board before the canvas.** `look` with `palette: true` shows
  each heap thick and thinned over black: a chalky mix, a violet "gray", a
  glaze that stains or one that hides show there, before they cost a passage.
- **Steer a mix by eye**: `p:add{}` a touch at a time, looking between
  additions.
- **Matching a passage** for a repair: start from its recipe in the log and
  lay a trial beside it. The layers since may have shifted the passage, so
  judge the laid trial in a detail crop.
- **Set out a limited palette** for a picture that should hang together; a
  few tubes mixed every way give related colors by themselves.
- **A dirty board unifies.** `palette{dirty=0.2}` to `0.4` lets each heap
  pick up a trace of the others through a sitting, as a working palette does;
  clean it (`palette{clean=true}`) when a passage needs pure color.

## Broken color

- **Many close piles.** Knife three to five piles of neighboring colors
  (warmer, cooler, lighter, darker) and lay them as separate strokes over
  the same area; the eye mixes them. `mix_jitter=` in `work` varies each
  dip; every pile is a little uneven.
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
  thin pale accent against a dark reads as a crack.
- **Vary the touch.** One stroke length and one brush over a whole canvas
  reads as a mechanical texture. Change brush size, stroke length, direction
  and pressure from passage to passage; leave some passages quiet.

## Impasto

- **Thick paint for lights and accents.** Most of a canvas is thin: lay-in and shadows
  lean (turpentine), middle tones as from the tube. Keep the thick, blotted
  paint for the lights and the accents. Impasto everywhere reads as flat.
  `load_at=` varies the load across a pass; `scale_at` gives ridges of
  different sizes.

- Stiff paint holds the brush's marks: blot it (`pile{..., blot=0.3}`) or
  use it as it comes from the tube, with no medium. A full brush lays more with `lay=4`
  to `lay=16`; a coarse hog brush (larger `hair`) leaves coarser ridges.
- In stiff paint the hairs gather into clumps: strokes lie in ridges and
  furrows, walls rise along their sides, a bead where they end. Fluid paint
  levels flat, which suits a glaze.
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
  passage. It deepens and saturates and lets the light underneath through;
  it rescues a chalky passage. Over impasto it pools in the valleys and
  thins on the ridges, which then shine through. Too much pigment, or any
  white, lays an opaque veil: try it small first. A glaze over the whole
  picture to pull it together has failed more often than it has worked.
- **Scumble**: light paint dragged thin over a dry darker passage, broken
  by the surface. A light load and
  `hand="scumble"` often lay opaque dabs. The veils that blended in came
  from paint thinned with `thinner`, laid in several light passes. Try it
  on the scratch canvas first.
- Glazes and scumbles want the passage under them **dry** (`wait` until
  `drying` says so): over open paint they just mix.

## Keeping control

- **Clip** a pass near anything it must not touch (`clip=` a mask). The body
  hands carry paint past their mask's edge by as much as a stroke's length:
  without a clip they paint over what is next to them.
- **Protect what is there now.** Paint fine marks after the passages behind
  them where you can. In every later pass over that area, subtract each of
  them from the mask (`m - protected`), and subtract again after `grow` or
  `blur`, which spread the mask back over them.
- **Check a mask before painting through it**: `print(m:at(x, y))` at a few
  points inside the target and just outside it, past each end of a band.
- **Masks from earlier chunks**: a `local` mask is gone in the next chunk.
  Keep what later chunks need as globals: paths, recipes, protection masks.
  Before redrawing something by eye, look for its saved path. Keep big
  temporary masks local (memory).
- **Small forms** drawn as a neat diagram of separate parts can read as
  icons. A few thick, irregular touches of two or three colors read as the
  thing.
- **Taking paint off**: wet paint comes off with the rag (below) or the
  knife, down to the first set layer. Set paint stays: paint over it or
  glaze it. Either way it costs only painted time.

## The rag

- **It lifts wet paint down to the first set layer.** Earlier passes that
  haven't set are part of the same wet film, so a firm lift can take them
  too, down to set paint or the ground. Before a wipe, check
  `drying(x, y)` in the paint: `"open"` paint lifts, `"setting"` paint
  comes away slowly, and `"tacky"` or `"dry"` paint has set and stays.
  A wipe usually leaves a thin wet tint, and `drying(x, y)` reads that
  tint as `"open"` or `"setting"`. If the tint is `"setting"`, or the set
  layer under it is still tacky, it grabs the brush when you repaint.
  Check it at a nearby gap in the paint, where `drying(x, y)` reads the
  set layer.
- **Dip it for a lift.** A dry rag lifts less and leaves a pale tint of
  the color in the weave: useful for wiping lights out of a wet glaze or
  lay-in. To take a passage off, dip well (`r:dip(0.9)`), press firmly, give it two or
  three passes, then go over it again across the first direction with a
  fresh, dipped rag.
- **The spirits go in minutes.** Dampness halves every three minutes of
  painting time, and a long mask wipe takes minutes. Dip just before each
  wipe; `print(r.damp)` says how much is left.
- **A face fills up.** `print(r.load)`: near 1, the face lifts almost
  nothing, and the reply says so when a wipe ran on a full face. Refold
  (`refold=` in a mask wipe); when `r.soaked` nears 1 the whole cloth is
  full, so take a fresh rag. A wide wet area fills a face fast.
- **A mask wipe stays inside its mask.** Paint outside the mask is left as
  it was. A path wipe has no mask: the whole pad lifts, so along an edge
  you need crisp, use a rag narrower than the strip.
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

- An unvarnished picture keeps its matte and glossy passages as painted.
  A varnish (where the easel has one) makes it all glossy: darks deepen.
