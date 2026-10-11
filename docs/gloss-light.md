# Display look 2: local gloss and wet sheen

Display look **2** is opt-in for Gallery, Relief and the three Museum modes. Every existing display/export path defaults to look **1** until Alice chooses a migration. Color and the painter's ordinary unlit look retain their exact previous transform. Paint deposition, curing, RNG, checkpoints and engine editions are unchanged.

Look 1 remains available:

```sh
easel look --mode gallery --look-version 1
easel render-checkpoint live.ckpt.zst before.png --mode gallery --look-version 1
easel render-checkpoint live.ckpt.zst after.png --mode gallery --look-version 2
easel render-checkpoint live.ckpt.zst museum.png --mode museum --look-version 2
```

`legacy` and `gloss` are aliases for 1 and 2. The public `Canvas::seen_lit` and `Canvas::seen_museum` methods retain look 1 for historical callers; the new `*_version` methods select explicitly. `look`, `render-checkpoint`, `run`, `save` and `finish` accept `--look-version`; lit process frames inherit `run`'s selection. Companion views and held-knife passage/blade views inherit the requested version. Existing companion PNGs must be explicitly regenerated from saved state to adopt look 2. Cached display images must include the look version alongside source revision, light and size. The export log records the version and light; `frames.tsv` adds a trailing `look_version` column without moving its existing fields.

```sh
easel run painting.lua --gallery --look-version 2 --out gallery.png
easel save gallery.png --gallery --look-version 2
easel finish live.ckpt final.png --gallery --look-version 2
```

`finish --relief` remains the historical destructive finishing operation. Its equations and output are unchanged; `--look-version` selects only the optional display illumination added by `--gallery` or `--light`. For display-only Relief without finishing or aging, use `render-checkpoint --mode relief --look-version 2`.

Material-backed `render-state` and atomic committed-store `save` exports also preserve explicit look selection. Compact stage-4 snapshots retain their existing legacy renderer and cache identity. Their format does not store local gloss, so `render-snapshot` and `snapshot-companions` cannot select look 2; use the material-backed checkpoint/store routes for that view. This change does not reinterpret existing optical snapshots or expand their format.

## Optical approximation

The old display began with Color's 4% × (1 − gloss) white veil and added an exponent-40 highlight independent of local dry gloss. Look 2 retains Color as its calibrated diffuse baseline and replaces only the added highlight with a directional, neutral dielectric sheen. A flat matte surface matches Color exactly. Removing Color's entire haze was tested and rejected: it exposed the underlying pigment chroma and collapsed the value difference between matte grounds and fresh paint. Recalibrating that empirical material term requires a separate Color/material version and measured controls.

Stored dry gloss is blended toward wet oil using the same coverage-aware, thin-film transition as Color: wet-covered area × smoothstep(0, 0.1 coats, volume/covered area). A thin or partly covered wet mark cannot polish the entire pixel. Gloss activates smoothly between 0.25 and 0.95. The sheen narrows from exponent 128 to 512 as polish increases, centered on the light/view half-vector. At Gallery's 55° elevation, a flat glossy surface is far outside this narrow reflection.

The old broad highlight contained a flat-surface lift and useful normal-driven variation. Gallery and Relief retain that variation as rough-surface diffuse modulation: subtract the old lobe's flat-surface response at the same light, volume and shadow visibility, then add the signed normal-dependent difference to diffuse luminance. This adds no lift on a flat surface. The correction scales the diffuse RGB together, preserving its chromaticity, and keeps at least 10% of the original diffuse shade to avoid negative reflectance on deep wet troughs. The legacy directional diffuse multiplier, normals and shadows themselves remain unchanged. This is an empirical surface-contrast calibration, not extra height or image sharpening.

The narrow polished peak follows a Schlick dielectric factor with normal-incidence reflectance 0.04 (oil's approximate refractive index 1.5). Wet coverage raises peak strength from one-half to the full reflection factor. The visible sheen replaces the same fraction of the modulated diffuse color: output = diffuse × (1 − sheen) + sheen. Flat glossy paint away from the reflection angle retains Color. Museum retains its existing multi-source diffuse modulation and averages the narrow sheen over its eight sources at low contrast; its legacy model had no broad highlight to decompose.

These are bounded display lobes, not a radiometrically calibrated microfacet BRDF or measured oil roughness. The gloss activation thresholds, exponents and wet peak scale are artistic model controls. The renderer has no measured lamp size, subpixel slope distribution or polarization model. Museum remains a low-glare approximation. No global contrast curve, pigment-dependent darkening or image grain is added.

The flat dark matte regression requires exact linear Color values and exact Color PNG channel values, with at most 2/255 per-channel difference from look 1. Its flat glossy counterpart is also byte-identical to Color in look 2; only look 1's spurious broad lift disappears. Raised matte surfaces retain the same diffuse shading response, and fresh paint keeps its contrast against the matte ground.

Saved-state Gallery controls require at least 85% of look 1's surface-relief contrast. On the retained 320-pixel probes and native r40 crop, the measured ratios are 99.94% for raised-matte, 97.66% for the fresh-stroke interior and 103.03% for the r40 wall. The metric is the standard deviation of the 5×5 high-pass of linear luminance `(Gallery − Color)` on fixed regions. Subtracting Color and excluding design boundaries isolates paint-surface texture; whole-image standard deviation can hide its loss behind the painting's color design. Total visible high-pass contrast is also required to retain 85%, and passes on all three regions. Both metrics and region definitions are in `renders/gloss-light/relief-retention.json` and `relief_metrics.py` in the Captain's Drive.

`gloss_light_probe <output-directory> <saved-state-directory>` relights the five retained probe checkpoints without depositing paint again. With no saved-state argument it creates the deterministic controls once and saves them before rendering.

## Compatibility

`run --checkpoint` remains an alias for the durable `--checkpoint-out` export introduced by state-stage1. Saved-state rendering accepts compressed studio saves, Museum modes and both look versions, and never executes the painting log. Re-rendering the same saved state in Color is byte-identical between look versions. Look 1 retains the previous Gallery, Relief and Museum arithmetic for historical exports.
