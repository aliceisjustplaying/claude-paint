# l1_near: loop 1 rework of the round-4 near winter (erratic, spruce, birch stump)

Critique being answered (P5, total 22): worst = embossed foam snow rims around both
boulders; next = the wood as one flat black mass; next = the foreground shadow band flat
and even.

## What I changed and why

1. **Snow rims → drifts (chunk 11 rewritten, chunk 19 rewritten).** The rims were a
   16–24-unit ribbon painted twice in thick body (chunk 11: filbert 3, coverage 3.0,
   medium 0.15; chunk 19 again at coverage 3.4), which with `relief()` read as
   foam/stickers. Now the drift is built from the stone's real foot line
   (`FOOT[x]`: the lowest y where `v:visible("bodies")` > 0.5, every 2 units) and is
   only the snow that creeps *up* the stone: height `max(0, 3 + 26*hn + 4*hn2)` (hn
   period 70, hn2 period 11), so along the base it comes and goes in tongues and
   vanishes in places. It stops 1–5 units below the foot (no apron), is colored 55% from
   the field just below (`sample(x, b+9, 3)` +0.015 L) so it merges into the ground, and
   is laid thin (flat 6, coverage 3.6, medium 0.3, load 0.8, hug=false, clipped only on
   its top edge) and blended twice. Chunk 19 is now only a thin veil (coverage 2.2,
   medium 0.4, load 0.5, 50% mix) over the hollow in front, not a second pass over the drift.
2. **Snow cap on the boulder (chunk 11).** Lower noise threshold (0.18–0.42, period 22)
   so the cap is a continuous sheet, not lichen dabs; filbert 4, coverage 2.6, medium
   0.32, load 0.6, then `blend(cap, {clip=cap})`.
3. **Contact shadow (chunk 14).** `contact_shadow` now minus `footd:grow(2)`: it had put
   a dark crease under the drift, so the stones floated.
4. **Foreground shadow band (chunk 14).** The cast-shadow glaze is multiplied by a mask
   that follows the snow's own light: `0.35 + 0.5*smoothstep(0.3,0.5,lit) +
   0.3*ripple + 0.2*slow`, clamped 0.15–1.1 (ripple: noise period 22, stretch {0.02, 4};
   slow: period 110), coats 0.36. It now pales and deepens along its length.
5. **Wood interior (new chunk 20, before the varnish).** A back row of 22 spruces
   (`uneven(22, 10, 560, ...)`, tops `70 + 0.42x ± 35`) drawn only as rigger
   branches in #2d3834 to #333e3a (a step grayer than the #1f2824 mass), 60% of the
   branches, pressure 0.45 → 0.05, clipped to `woodm:shrink(5)` above y 338, minus the
   stone. Plus 26 trunks (round 1.6, #2a2622/#3b3630) standing at the wood's foot,
   y ≈ 318 → 349. The effect is subtle at 1000 px. The mass is still mostly black.

Renders: `out/l1_near.png` (1000) and `out/l1_near_full.png` (3200).

## SKETCHBOOK CANDIDATES (only if the critic's scores improve)

- **Snow against a stone's foot, drawn from the real foot line (replaces the drift
  ribbon in §7).** Scan the body mask for its lowest point per x (`FOOT[x]`, step 2),
  make the drift the band from `foot - h(x)` down to `foot + 1..5`, with
  `h = max(0, 3 + 26*noise(period 70) + 4*noise(period 11))` so it vanishes in places.
  Color it about 55% from the field just below the foot. Lay it thin (flat 6,
  coverage 3.6, medium 0.3, load 0.8, hug=false, clip only on the top edge) and blend
  twice. A ribbon of even width, painted twice in thick body, read as foam/stickers (P5
  worst defect).
- **Pitfall: `roughen()` on a soft shadow mask (cast_shadow) hardens the whole penumbra**
  into blocks with hard edges, because it re-edges at the 0.5 level. Modulate a soft
  mask by multiplying it, never roughen it.
- **Pitfall: noise stretched perpendicular to a long shadow gives vertical "curtain"
  streaks.** On a ground plane, stretch the ripple near horizontal (`stretch={0.02, 4}`),
  as undulations read in perspective.
- **Pitfall: subtract drifts and snow lips from `contact_shadow`,** or the crease lands
  under the drift and the stone floats.
- **Pitfall: an under-hiding pass over a rough stone speckles.** Thin paint (filbert,
  medium 0.4 to 0.45) over the stone's pitted relief showed the dark grain through as
  white lace. The flat 6 at load 0.8 with two blends covered it.
