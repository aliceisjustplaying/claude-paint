# Display look 2: local gloss and wet sheen

Gallery, Relief and the three Museum modes now use display look **2** by default. Color and the painter's ordinary unlit look retain their exact previous transform. Paint deposition, curing, RNG, checkpoints and engine editions are unchanged.

Look 1 remains available:

```sh
easel look --mode gallery --look-version 1
easel render-checkpoint live.ckpt.zst before.png --mode gallery --look-version 1
easel render-checkpoint live.ckpt.zst after.png --mode gallery --look-version 2
easel render-checkpoint live.ckpt.zst museum.png --mode museum --look-version 2
```

`legacy` and `gloss` are aliases for 1 and 2. The public `Canvas::seen_lit` and `Canvas::seen_museum` methods retain look 1 for historical callers; the new `*_version` methods select explicitly. The easel's display companions, lit final exports and lit process frames select look 2. Existing companion PNGs must be regenerated from saved state to adopt it. Cached display images must include the look version alongside source revision, light and size.

## Optical approximation

The old display began with Color's 4% × (1 − gloss) white veil and added an exponent-40 highlight independent of local dry gloss. Look 2 retains Color as its calibrated diffuse baseline and replaces only the added highlight with a directional, neutral dielectric sheen. A flat matte surface matches Color exactly. Removing Color's entire haze was tested and rejected: it exposed the underlying pigment chroma and collapsed the value difference between matte grounds and fresh paint. Recalibrating that empirical material term requires a separate Color/material version and measured controls.

Stored dry gloss is blended toward wet oil using the same coverage-aware, thin-film transition as Color: wet-covered area × smoothstep(0, 0.1 coats, volume/covered area). A thin or partly covered wet mark cannot polish the entire pixel. Gloss activates smoothly between 0.25 and 0.95. The sheen narrows from exponent 128 to 512 as polish increases, centered on the light/view half-vector. At Gallery's 55° elevation, a flat glossy surface is far outside this narrow reflection.

The peak follows a Schlick dielectric factor with normal-incidence reflectance 0.04 (oil's approximate refractive index 1.5). Wet coverage raises peak strength from one-half to the full reflection factor. The actual visible sheen replaces the same fraction of diffuse color: output = diffuse × (1 − sheen) + sheen. Away from the reflection angle, the diffuse baseline is unchanged. Gallery and Relief honor their existing cast-shadow visibility; diffuse lighting and softened slopes remain unchanged. Museum averages the sheen over its eight sources and scales it by the selected low surface contrast.

These are bounded display lobes, not a radiometrically calibrated microfacet BRDF or measured oil roughness. The gloss activation thresholds, exponents and wet peak scale are artistic model controls. The renderer has no measured lamp size, subpixel slope distribution or polarization model. Museum remains a low-glare approximation. No global contrast curve, pigment-dependent darkening or image grain is added.

The flat dark matte regression requires exact linear Color values and exact Color PNG channel values, with at most 2/255 per-channel difference from look 1. Its flat glossy counterpart is also byte-identical to Color in look 2; only look 1's spurious broad lift disappears. Raised matte surfaces retain the same diffuse shading response, and fresh paint keeps its contrast against the matte ground.

## Compatibility

`run --checkpoint` remains an alias for the durable `--checkpoint-out` export introduced by state-stage1. Saved-state rendering accepts compressed studio saves, Museum modes and both look versions, and never executes the painting log. Re-rendering the same saved state in Color is byte-identical between look versions. Look 1 retains the previous Gallery, Relief and Museum arithmetic for historical exports.
