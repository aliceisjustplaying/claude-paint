# l3_near: loop 3 rework of the near winter (erratic, spruce, birch stump)

Critique being answered (three critics, totals 22/23/20): worst = the long, straight,
even shadow band across the whole foreground "cast by nothing", while the boulder's own
shadow falls toward the viewer (two critics); worst = the stamped Christmas-card spruces
with a white edge on every tier (one critic, "next" for the others); next = the carrot-orange
stump top and the white specks over the dark wood "like screen noise".

## What I changed and why

1. **The shadow band (chunk 14, rewritten).** It *was* the stump's shadow, but from the
   wrong caster: the proxy was `body.block(st:p(0, 0.55, 0), st:size(0.36, 1.1, 0.36))`.
   `size` takes radii, so the block was 2.2 m tall and 0.72 m wide (the sketchbook's "radii"
   pitfall again). At 275 units/m the painted stump is 167 units, 0.6 m. At the 11° sun, that
   2.2 m block threw an 8.5 m shadow 40 units wide across the whole foreground.
   - The correct 0.6 m proxy still gave a 3 m shadow (to x 810), and `cast_shadow` started it
     60 units right of the stump's foot, so it looked detached. So I dropped the stump proxy
     and **drew the shadow by hand from the projected sun direction**: the centerline is
     `w:project` of points 0 to 3.1 m along azimuth 68° from the stump (`STSH`). The shadow is
     dark and 13 units wide at the foot. The half-width grows `6.5 + 7t`, the edge softness
     `1 + 9t`, and it fades out by `1 - smoothstep(0.08, 0.62, t)`, scalloped by noise
     (periods 28 and 9). Glaze `#62688a`, 0.42 coats, times the existing snow `mod`. That's
     physically fair: under the veiled sun (`overcast=0.35`), the penumbra outgrows a 23 cm
     stump within a couple of meters. The grass (chunk 15) is laid over it.
   - **The belly shadow in front of the boulder removed.** The ellipsoid boulder shaded the
     ground under its own bulge in front of its lit face, a 70-unit band under the stone that
     read as "its shadow falls toward the viewer". An erratic sits sunk in the ground, so
     `cast_shadow * -belly`, where `belly` is everything below `footat(x)` left of x 575–606.
     The boulder's real shadow to the right, under and past the spruce, is kept, and so is the
     contact shade.
2. **The young spruce de-stamped (chunk 12, rewritten).** After `spruce{}`, every bough is
   rescaled about its root by `f = clamp(1 + 0.3·noise(period 70; by y and side) + N(0, 0.2),
   0.5, 1.4)`, and 10% of them are stunted to ×0.35–0.55. Each gets a random sag
   (`randn(0, 0.12)·|dx|`). The mask is rebuilt from the new ribbons with a rougher edge
   (`roughen(3.2, 6)`, was about 2.4/7.9). Then a **curtain of short hanging branchlets**
   (round 1.2, one per 3.2 units of bough, length `tier·(0.2–0.55)·(0.6 + 0.6t)`,
   `pressure={0.75, 0.05}`) breaks the clean underside of every chevron. Ragged needle tips
   along the rim follow: `work(ysm:rim(4, 1), {hand="hatch", round 1.2, hug=false,
   coverage 1.3})`. The tree is now lopsided, and its whorls differ.
3. **Snow on the spruce only in lumps (chunk 18).** It was a stipple on every upper edge.
   Now that stipple is gated by a second, stretched patch noise (period 26, `stretch={0.05,
   3}`, `smoothstep(0.4, 0.56)`) and weighted to the sun side. A first try at
   `smoothstep(0.5, 0.68)` left almost nothing.
4. **The stump's break weathered (chunk 13).** The raw `#9c7a52` became silver-gray
   `#a39a8a → #857d70 → #5f5a54` from the lit side to the shaded side, as a snag looks after a
   winter or two.
5. **The specks in the wood hatched out (new chunk 22, before the varnish).** Pale specks
   (sample L > 0.34 to 0.46, grown 1.8) inside `woodm:shrink(7)`, above y 306 and away from the
   spruce, got a near-black hatch (round 1.6, the wood's own colors). The edge-tree
   silhouettes against the sky aren't touched.

Renders: `out/l3_near.png` (1000) and `out/l3_near_full.png` (3200).

## SKETCHBOOK CANDIDATES (only if the critic's scores improve)

- **Check proxy sizes against the painted motif** (§7 or §13 pitfall): `size` takes radii,
  so a proxy block written as the motif's full size casts a shadow 2× too long and 2× too wide.
  At a low sun that's a band across the whole picture. Measure first:
  `w:height(x, y, 1)` gives units per meter at the foot.
- **A small caster's shadow at a low sun, drawn by hand** (§7): project the sun direction
  from the foot with `w:project(X + L·sin(az+180°), Y, Z + L·cos(az+180°))` for L = 0…h/tan(el),
  then glaze a mask that is dark and crisp at the foot and widens (half-width
  `6.5 + 7t`, edge `1 + 9t`) and fades out (`1 - smoothstep(0.08, 0.62, t)`), with a little
  noise wobble. It reads as cast by the stump and doesn't cut the picture in two.
  `cast_shadow` of a small proxy can start detached from the foot.
- **Sink an ellipsoid boulder's shadow** (§6/§7 pitfall): an ellipsoid shades the ground under
  its own belly, in front of its lit face, and critics read that as a shadow falling toward the
  viewer. Subtract everything below the foot line on the lit side from `cast_shadow`.
- **De-stamping a spruce** (§5, beats "symmetric chevrons"): rescale every bough of the
  spruce hand about its root by a slow noise (by height and side) plus `N(0, 0.2)`, clamped
  to 0.5–1.4, and stunt 10% to ×0.35–0.55. Rebuild the mask, then hang short branchlets
  (round 1.2, one per 3.2 units, `tier·0.2–0.55` long) under each bough and hatch the rim
  with `hug=false`. Snow in lumps: gate the tops stipple by a second patch noise so most
  tiers carry none.

## Still weak

- A faint vertical seam in the snow at x ≈ 187, y 520–769 (0.01–0.04 L). Its source isn't
  found; it isn't in chunks 14, 16, 17 or 19's masks as far as I checked.
- The edge spruces of the wood (chunk 7) are still symmetric chevrons with lit tier edges.
  The bough-rescale recipe above should go into `spruce()` itself, but that means a replay
  from chunk 7.
- The belly-free snow in front of the boulder is now an even light field. It could use a
  faint cool undulation.
- The sky's horizontal "wood grain" striation is untouched (probably the tool).
