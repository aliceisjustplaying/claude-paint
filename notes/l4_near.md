# l4_near: loop 4 rework of the near winter (erratic, spruce, birch stump)

Critique being answered (three critics, totals 21/22/22). The shared complaint, "worst" for one
critic and "next" for two: the wood at upper left is a near-black slab with no aerial
perspective, ruled trunks and white specks, and its hard contour against the snow makes it a
stage flat. Other complaints: the boulder's ruled lit/shadow seam and the white "icing" rim at its
foot (worst for one critic), and the young spruce's sagging limbs and missing snow.

Session: about 25 minutes. The varnish chunk was taken off with `undo 1`, the fixes went in as
chunks 23–25 and the varnish came back as chunk 26.

## What I changed and why

1. **The wood recedes into air (chunk 23, new).** After `dry()`, a veil goes over the whole wood
   (`woodm * -ysm * -v:visible("bodies")`) with `hand="body"`, round 2, lengths 4–11, vertical
   (`angle=π/2`, jitter 0.3), `medium=0.45`, `load=0.55`, `coverage=2.6`, `hug=false` and
   `clip=` the same mask. `color_over` mixes what's there toward a haze of
   `#67717a → #8d959c` (by y, 150 → 350). The mix fraction is
   `wveil = (0.06 + 0.46·far^1.3 + 0.32·foot) · (0.8 + 0.4·noise)`, clamped to 0.7, where
   `far = smoothstep(120, 640, x)` is distance along the edge toward the clearing and
   `foot = smoothstep(270, 350, y)` is a low mist among the trunks. The near corner (top left)
   keeps its near-black, the far end at the clearing goes gray-blue, and the foot dissolves into
   mist instead of a hard dark line on the snow. The ruled trunks and the stripe at the foot go
   soft in it. Measured with `sample().L`: (560, 250) went 0.31 → 0.41 at the first strength.
2. **The veil blended while wet (chunk 24, new).** Without it, the veil's dry-brush breaks
   left the dark wood showing through as dark flecks, peppering the wood. `blend` over the same
   mask, vertical, clipped, fused it into one film.
3. **The foot-bank glazed cooler (chunk 25, new).** `glaze(footd:blur(2.5), {color="#8a8fa8",
   coats=0.26})` over the drift at the boulder's foot, stone included, to take down the white
   rim. **At 1000 px the effect is small; the rim is still there.**

## Tried and undone

- A veil only over the wood interior (`woodm - union of edge.mask`). The edge-spruce masks cover
  nearly the whole wood (the interior was only 15.8k units²), so it changed nothing.
- The same veil with `hand="glaze", medium=0.8`: no visible lift on the near-black.
- The veil without `dry()` first: the pale paint sank into the still-open hatching (chunk 22),
  nothing visible.
- Restating the edge spruces' boughs through the mist with a rigger 1.0, each a step darker
  than the veil under it (`shift(sample, -0.07..-0.12)`), 608 boughs: they came out as crisp,
  even ruled lines in chevrons, and where the sample under was pale mist they made light
  chevrons at the foot. Undone.
- Softening the boulder's seam with a Gaussian band (width 12) of half-tone (`mix(under,
  "#8a8078", 0.34)`, filbert 3 along `fall`) and a `blend`. First placed on the pencil line
  (x ≈ 541 + …), but **the painted seam is at x ≈ 494**, not on the drawing, so it lit a stripe
  in the shadow flank. Moved to 494, it read as a smooth vertical pillar, a new ruled shape.
  Undone. The seam is still open.

## SKETCHBOOK CANDIDATES (only if the critic's scores improve)

- **Aerial recession for a near-black wood** (§5, "wood interiors as flat black masses"; §4):
  a veil over the whole wood, graded by *distance along the edge* and by a *foot mist*, not a
  flat haze. Recipe as in (1) above: body hand, round 2, vertical short strokes, medium 0.45,
  load 0.55, coverage 2.6, `color_over = mix(under, haze, k)`, with k from 0.06 at the near
  corner to ~0.5 at the far end plus 0.32 at the foot. Then `blend` the same mask, vertical and
  clipped, while wet.
- *Pitfall:* **`hand="glaze"` with `color_over` can't lift a near-black passage.** To lighten
  a dark mass you need body paint (thin, `load` about 0.55).
- *Pitfall:* **a pale veil over a wood needs `dry()` first,** even when it's a day after the
  last hatch: without it, the veil vanished into the open dark paint.
- *Pitfall:* **`easel try --look` didn't show this body-veil's change,** though `sample().L`
  inside the try did (0.31 → 0.41). Measure inside the try, or `do` it and `undo`.
- *Pitfall:* **the painted seam of a body isn't the pencil line.** The boulder's lit/shadow
  seam is from the form's light (x ≈ 494), 45 units from the drawn joint (x ≈ 541). Find it
  with a crop and grid before painting to it.
- *Pitfall:* **a Gaussian half-tone band over a ruled seam, blended, becomes a smooth vertical
  pillar,** another ruled shape. A seam needs a turned plane in the form (a `cut` or a rounded
  shoulder in the body), not paint over it.
- *Pitfall:* **restated boughs through mist as thin rigger lines, darker than the sampled
  veil, read as ruled chevrons,** and pale ones where the veil is light.

## Still weak

- The boulder's seam and the lace-like white rim at its foot (the rim is the drift paint caught
  in the stone's grain inside `stonem`). The glaze in chunk 25 barely touched it.
- The spruce: limbs sag and there's little snow on it (untouched this session).
- A few dark specks at y ≈ 300 across the wood, left from the veil's broken strokes.
- The misty wood interior is soft now: at 3200 it may read as airbrush rather than trees.
- **Seen at 3200 (out/l4_near_full.png, wood crop x 300–650, y 150–370):** the veiled far
  spires carry a thin pale fringe along their boughs. Their rigger bough tips from chunk 7 ran
  past `woodm`, so the veil missed them, and they stay near-black on gray trees. The misty
  interior is smooth and flat, with no trees in it. *Pitfall candidate:* a veil clipped to a
  silhouette mask misses the rigger strokes drawn past it; grow the clip a little
  (`woodm:grow(1.5)`, intersected with the non-sky area) or veil those tips separately.
