# Friedrich notes

Working method (no reference images): light ground → brown umber underpainting
for values → body-color sky (Mixbox pigment gradient) → thin KM glazes → very
soft horizontal strokes → figures last → warm varnish + canvas weave.

## Monk by the Sea study (`paintings/src/bin/friedrich_monk.rs`)

What worked
- Pigment-mixed sky gradient warped by horizontally stretched fbm
  (x * 0.35–0.45, y * 1.4–1.6) reads as layered cloud banks, not blobs.
- Mist bank = semi-opaque dark glaze fading up from the horizon, max ~0.6.
  At 0.8+ it swallows the horizon line.
- Pale veil band (gaussian in y around 0.47h) gives the luminous middle sky.
- Sky strokes: Body medium, opacity ~0.12, streak ≤ 0.2, load 1.0. Lower load
  (dry brush) makes scan-line streaks across the whole sky.
- The figure must overlap the horizon so the head reads against the mist;
  near-black #0d0c0b; add a faint contact shadow glaze or it floats.

What didn't
- Canvas weave below ~3px/thread aliases into a grid (now auto-faded).
- Saturated ochre dune looked like a beach resort. Keep sand gray-ochre.
- Scattered single grass blades read as hairs; clump them.
- Gulls as bright V strokes read as checkmarks; keep them soft, ~0.5 opacity.
- Evenly spread whitecaps read as dashes; concentrate near the shore.

Ideas for next
- A proper Rückenfigur generator (coat, hat, stick, poses) instead of
  hand-placed polygon points.
- Oak generator (gnarled, recursive, with twig density falloff).
- Fog that fills valleys by depth, not just by y.
