# Winter A/B: which of today's engine changes alter round 2's winter picture

The port (notes/round7/winter_port.md) renders round 2's
`fresh2_winter` on today's engine. Alice preferred the original: today's
picture "shows the tell-tale too-perfect lighting" and its trees are
thinner. This branch (r7-winter-ab, not for merge) switches single engine
changes back for this one picture, renders it, and measures each change
against the original. Renders only. No engine code is changed on the
branch.

## The switches (temporary, in `paintings/src/bin/fresh2_winter.rs`)

All are off by default. With none set the render is byte-identical to the
port's `today` render (checked at 1000 with `cmp`).

- `WINTER_POINT0=1`: every `Tool::round_sable`, `Tool::rigger` and
  `st.line_tool` the painter makes is built with `point: 0.0`. This undoes
  the pointed-tip default of 2ce7ee0 for those brushes. It is the port's
  point-0 diagnostic, done the same way (helpers `rs_`, `rg_`, `lt_`).
  The render is byte-identical to the port's
  `winter_port/diagnostic_today_point0_1000.png`.
- `WINTER_POINT0=2`: the same, plus the style's `detail` sable (a pointed
  `round_sable(2.2)` used by `st.detail()` and `st.hatch()` for the ruin
  hatching and the brook ice and water). The port's diagnostic left that
  one pointed. It makes little difference (whole-picture mean diff to the
  original: 4.46 with 1, 4.48 with 2, at 1000), so the variants below use 1.
- `WINTER_RELIEF=1`: the finish's relief is round 2's `(0.2, 0.02)`
  instead of today's Style default `(0.06, 0.006)` (fc5e9f6).

Engine switches used only in scratch builds, never committed:

- glaze floor off: `formed_film` in crates/paint/src/canvas.rs returns the
  request unchanged (`if true || um >= MIN_FILM_UM`). This undoes the
  thin-film part of 1742baa.
- stipple fade off: `Stipple` default `fade: 0.0` instead of 1.0
  (f3f0fda).

Variants: **a** = `WINTER_POINT0=1`; **b** = `WINTER_RELIEF=1`; **c** = a
and b; **d** = c with the glaze floor off (a scratch engine build).

## Files (notes/round7/winter_ab/)

- `sheet_whole_1000.png`: original, today, a, b, c, d, whole, at 1000
  (3 × 2)
- `crop_oak_crown_3200.png`, `crop_ruin_path_3200.png`,
  `crop_fir_group_3200.png`: 1:1 from the 3200 renders, the port's windows
  (oak 1088×1024+384+640, ruin 960×1024+1158+1088, firs
  768×544+2208+1280), same six panels
- plain: `a_1000.png`, `b_1000.png`, `c_1000.png`, `d_1000.png` and the
  same at `_3200`. The original and today plain renders are the port's
  (`winter_port/original_{1000,3200}.png`, `winter_port/today_{1000,3200}.png`).
- `diagnostic_luma_c_vs_original_1000.png`,
  `diagnostic_luma_d_vs_original_1000.png`: signed luma difference,
  smoothed over 9 px, over a dimmed picture. Red is lighter than the
  original and blue is darker, full at ±12 levels. These are for reading
  the numbers, not views of the painting.

## Measured

Same conventions as the port: |diff| is the per-pixel absolute difference
averaged over RGB (0–255). L is the mean signed luma change (variant minus
original). "tex" is the standard deviation of luma minus its Gaussian blur
(sigma 1 picture unit), a fine-texture measure: lower is smoother. "dark"
is the share of pixels with luma < 80. Regions are in picture units, as in
the port. "Open sky" is x 0–600, y 0–200 and "open snow" is x 30–230,
y 630–710: no motifs in either.

### 1000 px, against the original

| variant | whole mean / p99 / L | sky y<380 L | far/mist L | snow y>460 L | oak dark % | firs dark % | fir window mean |
|---|---|---|---|---|---|---|---|
| original | – | – | – | – | 10.21 | 16.42 | – |
| today | 6.18 / 86.0 / +3.80 | +2.78 | +7.49 | +4.16 | 6.05 | 10.65 | 15.63 |
| a | 4.46 / 29.0 / +1.93 | +1.88 | +4.97 | +1.08 | 10.38 | 16.78 | 7.92 |
| b | 6.21 / 86.0 / +3.78 | +2.77 | +7.42 | +4.14 | 6.06 | 10.65 | 15.68 |
| c | 4.49 / 29.0 / +1.91 | +1.87 | +4.89 | +1.06 | 10.38 | 16.78 | 7.96 |
| d | 3.01 / 28.3 / −0.54 | −0.47 | +0.42 | −0.94 | 10.51 | 16.91 | 5.91 |

### 3200 px, against the original

| variant | whole mean / p99 / L | whole tex (orig 8.15) | sky L | far/mist L | snow L | oak dark % (8.02) | firs dark % (15.50) |
|---|---|---|---|---|---|---|---|
| today | 5.18 / 46.0 / +2.51 | 8.81 | +2.00 | +4.27 | +2.71 | 6.78 | 12.89 |
| a | 4.54 / 29.7 / +1.69 | 8.37 | +1.73 | +3.39 | +1.12 | 8.13 | 16.11 |
| b | 5.21 / 46.0 / +2.47 | 8.88 | +1.97 | +4.12 | +2.69 | 6.78 | 12.89 |
| c | 4.56 / 29.7 / +1.65 | 8.45 | +1.70 | +3.25 | +1.09 | 8.12 | 16.12 |
| d | 3.20 / 29.3 / −0.83 | 8.19 | −0.67 | −1.28 | −0.92 | 8.19 | 16.21 |

Today against b directly at 3200: mean 0.77, p99 3.7. Relief 0.2 changes
this picture very little: its paint is thin.

### The lit oval

The painter's last stage, "veil", is a cool-brown glaze over everything
but the moon, centered at (520, 400) and darker toward the edges:
thickness `(0.06 + 0.5·smoothstep(0.45, 1.3, r) + …)·(0.75 + 0.25·away)`
coats. The painter's comment says "never zero anywhere". At the center it
is 0.045–0.06 coats, which is a film of 0.34–0.45 µm (`COAT_UM` 25 ×
`GLAZE_FILM` 0.3). Since 1742baa a glaze film under `MIN_FILM_UM` (1 µm)
fades out, and it is gone entirely at 0.5 µm. So today the veil has a
bare center, an ellipse reaching about r 0.6 (x about 150–890). The
center is lighter and a soft rim is left where the film starts to form.
The luma map (`diagnostic_luma_c_vs_original_1000.png`) shows it as one
even oval of +5 to +7 levels around the ruin and the oak.

Mean luma of the sky inside r < 0.45 (y < 380) and in a ring
0.75 < r < 0.95:

| | 1000 inside | 1000 ring | 3200 inside | 3200 ring | inside − ring at 3200 |
|---|---|---|---|---|---|
| original | 170.0 | 123.0 | 171.7 | 122.6 | 49.1 |
| today | +8.1 | −0.4 | +6.6 | −0.6 | 56.4 |
| c | +5.8 | −0.4 | +5.9 | −0.6 | 55.7 |
| d | −0.5 | −0.4 | −0.5 | −0.6 | 49.3 |

With the floor off (d), the center-to-edge contrast of the light returns
to the original's. The oval is gone from the luma map
(`diagnostic_luma_d_vs_original_1000.png`).

## Bisect

Method: the (c + glaze floor off) program, adapted only where the API
forced it (the three `Cracks` literal shapes; `pt_` is the identity before
`Tool::point` exists), was built and rendered at commits between round 2's
engine and main. The merge base of origin/amnesia-winter and main is
503be2f. Round 2's branch changes no engine file after it. At 503be2f the
render is byte-identical to the original (mean 0.00). Each merge's commits
were rendered on their own branch, whose fork point has round 2's engine
(except the surface branch, measured from its fork 82179cc).

### First-parent merges, 1000, whole mean diff to the original

| commit | what | whole mean / p99 / L |
|---|---|---|
| 503be2f | round 2's engine | 0.00 |
| 20e6cb1 | cracks (vary 0, veil 0) | 0.16 / 1.0 / −0.16 |
| e82a594 | fixes-ux | 0.61 / 11.0 / −0.25 |
| 1258135 | drying (glaze floor off) | 0.61 (no change) |
| 87c324e | tip (point 0) | 1.12 / 18.3 / −0.08 |
| 182b62d | fixes-paint | 2.44 / 24.7 / −0.14 |
| 072e86c | atmosphere | 2.44 (no change) |
| 8cfecc0 | surface | 3.05 / 28.3 / −0.55 |
| b5a5f1a, 088b51c, 60071f0, fc5e9f6, 6bd8253 | fix3-physics, grain, r6-time, relief, varnish | 3.05 → 3.01 |
| aa9abf1 | main | 3.01 / 28.3 / −0.54 (= d) |

### Single commits, 1000, each against its predecessor

| commit | change | whole mean | where, what |
|---|---|---|---|
| 1742baa | glaze film floor (switched off here) | – | the veil's bare center: whole L +2.45 when on (c vs d), inner sky +6.3 |
| 1742baa, 9a9ebf4, 25b24e2 | settle guards, drying model, setting paint | 0.00 | nothing, with the floor off |
| 6e249f0 (in e82a594) | `Mask::roughen` in canvas units | 0.46 (its parent: 0.00) | the painter roughens only the two brook masks; changed pixels (> 2 levels, 3.7%) lie below y 209, median (553, 597): the brook and the snow painted after it |
| 2ce7ee0 | pointed-tip model (at point 0) | 0.45 | far/mist band 3.22, firs 1.74; open snow tex 4.87 → 4.59 |
| 09380cb | pointed tip: tracks tile the tuft | 0.72 | marks in the band and snow, placement |
| adb4e69 | aim over contrasting paint | 0.78, p99 3.3 | an even small color shift everywhere (open sky +0.25 L, tex 1.16 → 1.09) |
| fddcb76 | stroke centers hug a region's edge | 2.05 | placement at mask edges: horizon band 4.32, firs 4.78, snow 2.97; tone ±0.1 |
| fa26d6a, b421c72 | blender clipped, color_over | ≈0 | – |
| f3f0fda | stipple fade | 0.85 | far/mist L −0.73; texture unchanged. Switched off on today's engine (c vs c + fade off) it changes nothing measurable (whole 4.49 → 4.50) |
| 83633fc | look-and-fill pass | 2.67 | fills flecks between strokes: whole tex 5.64 → 5.56, open sky 1.20 → 1.04, firs 12.45 → 11.84 (L −1.33); snow L −0.47 |
| 5be65e6 | drying rates per physical patch | 1.00 | placement, tone 0 |
| 26974f4 | aim at a mark's mean look | 0.75, p99 3.7 | sky darker (L −0.47, open sky −0.70), far band −0.64 |
| 39e8aea | fill spots, aim weights | 0.25 | – |
| f3f0fda → 82179cc | the fixes-paint and atmosphere merges into main | 1.13 | far/mist band 3.92 |

### Later merges at 3200 (with c's switches and the floor off)

| step | whole mean | notes |
|---|---|---|
| original → 072e86c | 2.62 | round 3 up to atmosphere (as above) |
| 072e86c → 8cfecc0 (surface) | 3.16 | L −0.48; open sky L −0.81 |
| 8cfecc0 → 5d845c6 (halo) | 0.18 | – |
| 5d845c6 → 088b51c (ground grain) | 1.76 | the brushed ground; open sky tex 2.27 → 2.11 |
| 088b51c → 8198d95 (includes relief default; relief held at 0.2) | 0.00 | – |
| 8198d95 → 6bd8253 (varnish levels its own film) | 0.09 | open snow tex 8.79 → 7.04, L −0.62: the snow's ridges lose the pooled varnish at their feet |
| 6bd8253 → 5a89df6 → fa1862b (pinhole fix) | 0.00, 0.01 | – |
| fa1862b → main (d) | 0.00 | – |

### Candidate engine commits not singled out above

All of these are in the ranges measured: sky (34668f2, f832804, 050573b,
a3d0256: the painter paints no atmos sky, so no change at 072e86c),
ground (de1b0c5, 4bf6cb5, 68d6c04: in 088b51c), varnish (1beff29 in
6bd8253), relief (fc5e9f6, held at 0.2), glaze (1742baa; c3ac0b6 touches
`Paint::with_hiding`, not the veil's `Pigment::with_hiding`), cracks
(bcc3481, 29b0503, 0c269f8: 0.16 with vary and veil at 0; ddc7c8a,
0c8c939, 1d943d3 later, with their new fields at 0).

## What each change does to this picture

- **Pointed-brush default (2ce7ee0):** thins every tapered mark: the
  oak's dark share 10.2% → 6.1% at 1000, the firs' 16.4% → 10.7%. Point 0
  restores them (10.4%, 16.8%). It also accounts for part of the
  lightening in the motif windows.
- **Glaze film floor (1742baa):** removes the painter's thinnest veil, so
  there is a bare, lighter ellipse around the ruin and oak (inner sky +6
  to +8 levels, rim unchanged). It is the largest single tonal difference
  left once the brushes are blunt: whole L +1.9 in c, −0.5 in d. With it
  off, the center-to-rim contrast of the sky matches the original's (49.3
  against 49.1 at 3200).
- **Relief 0.2 → 0.06 (fc5e9f6):** small here (0.77 mean between today
  and b at 3200, p99 3.7).
- **Stipple fade (f3f0fda):** no measurable effect on this picture now.
- **What remains in d (whole 3.0 at 1000, 3.2 at 3200):** mostly marks
  placed a little differently: edge hugging (fddcb76), the look-and-fill
  pass (83633fc), the tip's track tiling, the brook's roughen units and the
  brushed ground. Plus two smaller tonal shifts: aim at mean look
  (26974f4), sky −0.5, and look-and-fill, which fills gaps in the firs and
  lowers fine texture in open sky. At 3200 the varnish fix smooths the
  open snow (tex 8.79 → 7.04). d's texture over the whole picture is the
  original's (8.19 against 8.15 at 3200; today 8.81).

## Reproducing

    WINTER_POINT0=1 WINTER_RELIEF=1 cargo paint fresh2_winter -- --full

The scratch builds (the glaze floor and fade switches, and the bisect
worktrees with an adapter for the `Cracks` literal) are not committed.
The diff and bisect scripts are in the task's scratch directory.

Render times: 1000 in 20–25 s, 3200 in about 105 s on today's engine.
