# Texture forensics: what in the paint could read as "JPEG artifacts"? (Round 7)

Alice sees something like JPEG artifacts in the texture of lossless renders
("the entropy still reads like JPEG artifacts"; "not the crackle at all,
it's something about texture"). This note lists every source of pixel-scale
texture in a render, where it lives in the code, a switch that takes each
out on its own, and what each switch does to a set of measurements on four
fixed windows. **No verdict:** the variants are in `notes/round7/texture/`
under neutral numbers (v01–v28) so they can be judged blind; the key is
`notes/round7/texture/key.md`. Judges: look at the images before reading on.

## How it was made

- Engine: this branch = main (variant d) plus `crates/paint/src/texoff.rs`:
  switches read from the environment variable `PAINT_TEXOFF`
  (comma-separated), **default off**. With it unset, the benchmarks
  `notes/loops/l5_near.lua` and `l3_green.lua` at 1000 px are
  byte-identical to main (checked, see the end).
- Paintings (logs unchanged, scratch copies): arm 1
  (`git show r7-arm1:paintings/lua/willows.lua`), arm 2
  (`git show r7-arm2:paintings/src/bin/pond_poplars.rs`, built as a
  scratch binary) and the Round 6 Lab sky A (`notes/lab/sky_A.lua`).
- Windows (units; 3200 px renders with `--crop`, default margin 40), each
  saved at 1:1 as a lossless PNG:
  - `Asky`   willows 650,60–850,200 (640×448 px): the upper right sky
  - `Apond`  willows 650,465–850,545 (640×256 px): the water right of the trunk
  - `Bsky`   pond_poplars 650,150–850,290 (640×448 px): the glow band
  - `labsky` sky_A 350,260–550,400 (640×448 px): between the two cloud banks
  (willows was rendered as one crop 650,60–850,545 and cut into the two windows.)
- Two families: **as painted** (one switch off; cracks as the painter
  finished) and **no cracks** (craquelure off plus one switch), because in
  A and B the cracks dominate every fine-scale number and hide the rest.
- Measurements: `scripts/texture_metrics.py` (committed; its docstring
  defines every number). Run with
  `uv run scripts/texture_metrics.py file.png ...`.
- Calibration: the Round 6 lossless crop `notes/round6/jpeg_check/sky_A_full_crop.png`
  and its q80 JPEG `sky_A_full_crop_q80.jpg` score **block8 0.94 % vs 68.1 %**
  and b8/b7,9 1.33 vs 58.4. Everything else changes little between them
  (L_8-16 0.762 vs 0.766), so the texture numbers describe the paint, not
  compression.

## The suspects and where they live (line numbers on main)

| # | suspect | where | what it does at pixel scale | switch |
|---|---|---|---|---|
| 1 | 8-bit save dither | `crates/paint/src/canvas.rs:497` (`save`) | triangular dither of ±1 LSB, **independent per channel** (hash per pixel and channel): per-pixel luma *and* chroma noise | `dither` (round only), `dither_mono` (one dither value for all three channels) |
| 2 | relief lighting | `canvas.rs:439` (`relief`); strength `style.rs:136` `(0.06, 0.006)`; called by `relief()` / `Finish` | shades the height field (weave, ridges, crack cupping) from the upper left | `relief` |
| 3 | linen weave | `surface.rs:466` (`linen_um`), laid by `build_support` `surface.rs:216` | a periodic height field: 15 warp / 13 weft threads per cm; felt by the bristles, pools thin paint, lit by relief | `weave` (height 0) |
| 4 | grounds | `canvas.rs:271` (`prime`, knife texture: value noise at 0.3/0.9 mm); `style.rs:203` (`brush_ground`, the brushed top ground) | the ground's own relief (striations, knife noise) under thin paint | `ground` (all grounds primed flat) |
| 5 | bristle footprints | `bristle.rs:846` (drag hair radius floor 0.55 px), `bristle.rs:1454` (touch 0.75 px), `bristle.rs:987` (smoothstep coverage per pixel center) | how a hair's contact is rasterized: aliasing on the pixel grid | `hairsoft` (floors 1.2 / 1.5 px) |
| 6 | stipple touches | `stipple.rs` (`stipple_with`, `touch_on` in `bristle.rs`) | many small round touches with a sharp rim; the layer under shows in the gaps | `stipple` (passes skipped) |
| 7 | stipple patches ("a load serves about one patch") | `stipple.rs:443` `cell = g·√dips` (at least 2 widths), in passages `stipple.rs:425` (≥ 12 widths) | the touches of one dip land in one square patch of a grid: pile-to-pile color differences can tile the passage | `dipcells` (a pile's touches scattered over the passage) |
| 8 | per-pile mix jitter | `palette.rs:478` (`remix`), `mix_jitter: 0.06` `style.rs:132`; OKLab jitter for unmixed paint (`handling.rs`, `stipple.rs`) | every dip's proportions vary by 6 % (relative sd) | `jitter` |
| 9 | aiming over the underlayer (color_over/aim) | strokes: `handling.rs:943` (`stroke_under`); stipple: `stipple.rs:497` (`judge_under` per dip); marks: `palette.rs:542` | each pile is mixed for what it will sit on, judged from the canvas at that spot: the under-texture feeds into the pile | `aim` (piles by masstone) |
| 10 | quantized aim recipes | `palette.rs:103` `Q_UNDER = 100` (underlayer keyed on 0.01 OKLab steps); stipple memo `stipple.rs:267` (`seen` on 1/120 steps) | neighboring dips over slightly different underlayers share or flip recipes: small tone steps | `aimfine` (4× finer keys) |
| 11 | look-and-fill dabs | `handling.rs:665` (`fill_gaps`), on by `handling.rs:445` | dabs into bare spots after a covering pass | `fill` |
| 12 | varnish | `crates/easel/src/api.rs:1808`, `paintings/src/run.rs:607` (a glaze, `canvas.rs:396`, pools in hollows) | a warm film, thicker in the hollows; lowers contrast | `varnish` |
| 13 | craquelure | `crack.rs:1329` (`crack`), its milky varnish veil `crack.rs:1443` | cracks, cupping, grime | `cracks` |
| – | drying noise | `drying.rs:127` (value noise in the drying rate, per mm) | only when the paint dries, not a pixel texture itself | not switched |

Periods these predict at 3200 px (arm paintings are 440 mm wide, 0.1375
mm/px; the Lab sky is 300 mm, 0.094 mm/px): the weave at 4.85 px (warp)
and 5.6 px (weft) in A and B, 7.1 and 8.2 px in the Lab sky. Stipple
patch sizes from the formulas (estimates, clustering ignored): Lab sky
(width 2.6, coverage 2.6, 18 touches a dip) about 5.2 units = 17 px; A's
second sky (width 3.2, coverage 1.7, 40 a dip) about 10 units = 33 px;
B's sky passes (2.4 / 2.4 / 18 and 1.5 / ≤2.1 / 22) about 15 and 10 px.
Passages (tiles, one brush each) are 30–40 units, about 100–125 px.

## Headline measurements

**No block grid.** In every window and variant, blockiness at 8 and 16 px
is at noise level: 0.6–3.0 % at 8 px in the three skies (block16 up to
9 % in the as-painted A sky, where cracks cross it) against 68 % for
the q80 JPEG of the Lab sky. The pond's 8–11 % is striation, not a grid: the
same measure gives 13.7 % at 6 px, 13.0 % at 9 and 39 % at 17 (as
painted). b8/b7,9 only means something when block8 is large; here it
wanders between 0.5 and 2.7 with the noise. JPEG's own signature, 8×8
blocking, is not in the renders.

**Regular periods that are there** (grid, peak/median of the edge-profile
spectrum at 4–40 px): the weave. The Lab sky peaks at 8.13 px (the weft)
at 50.9; with `weave` off the peak falls 64 % (to 18.1) and with
`relief` off 34 % (the weave shows through the relief lighting and the
paint it pools). Without cracks, A's sky peaks at 4.84 px (the warp,
32.7) and B's at 5.59 px (the weft, 30.7); `weave` halves A's peak (-48 %).

**What changes the texture most (changes of 10 % or more; the tables
below have all of it):**

| switch | Lab sky | A sky (no cracks) | B sky (no cracks) | A pond (no cracks) |
|---|---|---|---|---|
| cracks | (no cracks in it) | as painted → no cracks: L_2-4 -70 %, L_4-8 -54 %, L_8-16 -27 % | L_2-4 -65 %, L_4-8 -52 %, L_8-16 -41 % | L_2-4 -53 %, L_8-16 -40 % |
| stipple | L_2-4 -61 %, L_4-8 -75 %, L_8-16 -82 %, L_16-32 -76 %; flat% 0.06 → 29.7 | L_2-4 -36 %, L_4-8 -54 %, L_8-16 -73 %, L_16-32 -70 %; flat% 2.8 → 42 | L_4-8 -35 %, L_8-16 -53 %; b* hf -30 % | (no stipple in it) |
| aim | L_2-4 … L_16-32 -23 to -27 %; flat% ×8.5 | L_2-4 -16 %, L_8-16 -27 %; a* hf +29 %; flat% ×4 | L_4-8 -14 %; b* hf -23 %; flat% +26 % | – |
| dipcells | grid -21 % | grid +72 % | L_4-8 +34 %, L_8-16 +40 %, L_16-32 +48 %; flat% -38 % | (no stipple) |
| dither / dither_mono | a* hf -17 %, b* hf -12 % (both) | a* hf -33 %, b* hf -17 % (both) | a* hf -23 %, b* hf -12 % (both) | a* hf -22 % |
| relief | grid -34 %; flat% +29 % | flat% +80 %; grid -31 % | flat% +26 % | flat% +72 % |
| weave | grid -64 % | grid -48 % | – | – |
| varnish | flat% -26 % (dE 7.3: color) | flat% -22 % (dE 11) | flat% -11 % (dE 7.9) | dE 8.0 |
| ground | grid +38 % | flat% +32 % | grid ×2 | flat% +33 % |
| jitter, aimfine, fill, hairsoft | under 10 % on the texture bands; aimfine flat% -45 % in A | | | |

(Band = the standard deviation of L* carried by texture of that period in
px; "hf" = the 1–2 px noise of that channel; "flat%" = pixels in flat
5×5 patches; dE = mean ΔE76 against the family's baseline.)

**Per-pixel chroma noise.** a*/b* noise at 1–2 px is 0.23–0.30 / 0.30–0.47
in every window. The independent-per-channel dither is a steady share of
it: taking it out lowers a* by 17–33 % and b* by 6–17 %; `dither_mono`
does the same to chroma and keeps the luma dither (L_2-4 unchanged or up).
Without stipple the chroma noise stays while the lightness texture falls,
so the chroma/luma ratio rises to 2.8 in the Lab sky and the A and B skies.
That is the dither's chroma, alone on a smooth passage.

## What each switch changes (by eye, 2× nearest, crack-free A sky and the Lab sky)

- **stipple**: the sky becomes smooth horizontal strokes. The cellular
  mottle, the blotches with sharp scalloped rims and (Lab sky) warm
  specks where the underlayer shows between touches all go. By far the
  largest change in the 4–32 px bands.
- **aim**: the stipple's patches grow into larger, flatter tiles with
  steps between them (some near-square in the A sky); the tone drifts
  (dE 2–4) because unaimed piles don't compensate for the underlayer.
- **dipcells**: the blotches break into a finer, more even speckle; in B's
  sky the texture gets stronger in every band (the piles' differences are
  scattered touch by touch instead of patch by patch).
- **cracks**: removes the crack network. In A and B it is the main 2–8 px
  texture as painted.
- **dither / dither_mono**: no visible change at 1:1; chroma noise down.
- **relief, weave, ground**: small; the weave's period drops out of the
  spectrum, relief off flattens the finest shading (flat% up).
- **varnish**: without the warm film (`#e6d3a4`, 0.4 coats) the color
  changes (dE 7–11) and the texture bands are 10–14 % stronger in the A
  and B skies (as painted).
- **jitter, aimfine, fill, hairsoft**: hard to see; measurements within a
  few %.

## Tables

### As painted (cracks on unless switched off)

**A sky (willows, 650,60-850,200)**

| switch | dE | block8 | block16 | grid | gridP | L_2-4 | L_4-8 | L_8-16 | L_16-32 | L_32-64 | slope | Lhf | ahf | bhf | flat% | step10% | axis% | cells |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | 0 | 2.3 | 6.31 | 18.4 | 29.8 | 0.645 | 0.563 | 0.543 | 0.464 | 0.409 | -1.65 | 0.615 | 0.24 | 0.323 | 1.62 | 31.6 | 28.6 | 0.922 |
| dither | 0.47 | 2.08 | 7.38 | 19.2 | 29.8 | 0.639 | 0.56 | 0.542 | 0.465 | 0.407 | -1.65 | 0.611 | 0.167 | 0.275 | 1.83 | 31.7 | 28.5 | 0.937 |
| dither_mono | 0.462 | 2.12 | 6.21 | 18 | 29.8 | 0.65 | 0.565 | 0.543 | 0.465 | 0.408 | -1.64 | 0.618 | 0.166 | 0.275 | 1.45 | 31.5 | 28.5 | 0.913 |
| relief | 0.319 | 2.06 | 6.44 | 18.5 | 29.8 | 0.643 | 0.554 | 0.532 | 0.459 | 0.407 | -1.65 | 0.608 | 0.239 | 0.323 | 3.01 | 32.4 | 28.7 | 0.939 |
| weave | 0.382 | 2.37 | 6.25 | 18.1 | 29.8 | 0.644 | 0.559 | 0.538 | 0.459 | 0.4 | -1.63 | 0.613 | 0.238 | 0.322 | 1.76 | 31.7 | 28.6 | 0.926 |
| ground | 0.835 | 2.2 | 6.55 | 19.4 | 29.8 | 0.644 | 0.561 | 0.529 | 0.451 | 0.396 | -1.64 | 0.607 | 0.238 | 0.323 | 2.16 | 32.3 | 28.9 | 0.931 |
| varnish | 11.1 | 2.11 | 6.96 | 18.5 | 29.8 | 0.733 | 0.634 | 0.598 | 0.507 | 0.44 | -1.61 | 0.691 | 0.231 | 0.403 | 1.25 | 32.3 | 28.8 | 0.904 |
| cracks | 0.248 | 0.894 | 1.49 | 32.7 | 4.84 | 0.194 | 0.258 | 0.396 | 0.38 | 0.376 | -2.44 | 0.299 | 0.232 | 0.298 | 2.79 | 26.6 | 25.9 | 1.35 |
| stipple | 1.53 | 2.97 | 8.97 | 26.4 | 37.2 | 0.608 | 0.499 | 0.376 | 0.292 | 0.361 | -1.34 | 0.536 | 0.234 | 0.334 | 25.3 | 43.3 | 35.8 | 1.28 |
| dipcells | 1.01 | 2.25 | 7 | 19.6 | 29.8 | 0.642 | 0.554 | 0.523 | 0.437 | 0.349 | -1.55 | 0.612 | 0.241 | 0.343 | 1.73 | 31.4 | 28.9 | 0.921 |
| jitter | 0.662 | 1.91 | 6.61 | 17.9 | 37.2 | 0.647 | 0.564 | 0.539 | 0.453 | 0.388 | -1.6 | 0.614 | 0.238 | 0.326 | 2.06 | 31.8 | 28.7 | 0.915 |
| aim | 4.07 | 2.74 | 7.51 | 34.7 | 29.8 | 0.614 | 0.521 | 0.461 | 0.42 | 0.412 | -1.69 | 0.557 | 0.302 | 0.342 | 6.72 | 36.3 | 30.6 | 1.1 |
| aimfine | 0.699 | 1.96 | 6.16 | 15.3 | 29.8 | 0.649 | 0.567 | 0.551 | 0.459 | 0.397 | -1.59 | 0.612 | 0.243 | 0.315 | 0.905 | 31.1 | 28.5 | 0.93 |
| fill | 0.857 | 2.31 | 5.91 | 20.3 | 29.8 | 0.645 | 0.563 | 0.541 | 0.461 | 0.42 | -1.65 | 0.613 | 0.244 | 0.334 | 1.62 | 31.6 | 28.6 | 0.927 |
| hairsoft | 0.0564 | 2.01 | 6.65 | 17.1 | 29.8 | 0.645 | 0.563 | 0.542 | 0.463 | 0.409 | -1.64 | 0.615 | 0.239 | 0.323 | 1.64 | 31.6 | 28.5 | 0.923 |

**A pond (willows, 650,465-850,545)**

| switch | dE | block8 | block16 | grid | gridP | L_2-4 | L_4-8 | L_8-16 | L_16-32 | L_32-64 | slope | Lhf | ahf | bhf | flat% | step10% | axis% | cells |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | 0 | 11.3 | 24 | 18.6 | 33.6 | 0.788 | 0.674 | 0.525 | 0.381 | 0.41 | -1.34 | 0.935 | 0.266 | 0.47 | 10.2 | 47.2 | 52.4 | 1.1 |
| dither | 0.458 | 13.4 | 27.4 | 19.1 | 33.6 | 0.784 | 0.672 | 0.524 | 0.381 | 0.41 | -1.34 | 0.932 | 0.206 | 0.442 | 11.3 | 47.5 | 52.8 | 1.12 |
| dither_mono | 0.444 | 11 | 23.5 | 18.5 | 33.6 | 0.793 | 0.675 | 0.525 | 0.382 | 0.41 | -1.34 | 0.937 | 0.205 | 0.442 | 9.15 | 47 | 52.3 | 1.08 |
| relief | 0.319 | 11.6 | 26.1 | 18.7 | 33.6 | 0.787 | 0.669 | 0.517 | 0.385 | 0.408 | -1.35 | 0.929 | 0.266 | 0.469 | 17.3 | 49.1 | 53 | 1.13 |
| weave | 0.26 | 11.3 | 25.2 | 17.8 | 33.6 | 0.788 | 0.672 | 0.525 | 0.383 | 0.408 | -1.34 | 0.932 | 0.265 | 0.47 | 11.3 | 47.5 | 52.5 | 1.11 |
| ground | 0.741 | 11.7 | 24.5 | 19.8 | 33.6 | 0.79 | 0.676 | 0.513 | 0.391 | 0.404 | -1.36 | 0.93 | 0.259 | 0.467 | 13.2 | 48.8 | 52.5 | 1.09 |
| varnish | 7.96 | 11.6 | 25 | 18.9 | 33.6 | 0.85 | 0.723 | 0.562 | 0.408 | 0.431 | -1.33 | 1 | 0.273 | 0.447 | 9.09 | 47.5 | 51.8 | 1.09 |
| cracks | 0.262 | 8.4 | 11.9 | 12.5 | 33.6 | 0.368 | 0.383 | 0.313 | 0.255 | 0.376 | -1.69 | 0.632 | 0.266 | 0.425 | 16.2 | 45.6 | 78.5 | 1.9 |
| stipple | 0.0711 | 13 | 24.2 | 19.6 | 33.6 | 0.794 | 0.678 | 0.527 | 0.382 | 0.412 | -1.34 | 0.936 | 0.265 | 0.474 | 10.1 | 47.2 | 52.7 | 1.1 |
| dipcells | 3.09e-05 | 11.3 | 24 | 18.6 | 33.6 | 0.788 | 0.674 | 0.525 | 0.381 | 0.41 | -1.34 | 0.935 | 0.266 | 0.47 | 10.2 | 47.2 | 52.4 | 1.1 |
| jitter | 0.502 | 11.2 | 24.4 | 18.9 | 33.6 | 0.792 | 0.675 | 0.526 | 0.379 | 0.374 | -1.29 | 0.932 | 0.263 | 0.474 | 9.84 | 47.1 | 51.8 | 1.1 |
| aim | 1.1 | 11.4 | 24.5 | 18.3 | 33.6 | 0.787 | 0.674 | 0.527 | 0.383 | 0.387 | -1.34 | 0.935 | 0.287 | 0.473 | 10.1 | 47.4 | 52.8 | 1.05 |
| aimfine | 0.386 | 11.2 | 24.5 | 17.9 | 33.6 | 0.79 | 0.674 | 0.525 | 0.378 | 0.378 | -1.31 | 0.934 | 0.27 | 0.472 | 9.48 | 47.1 | 51.8 | 1.07 |
| fill | 0.822 | 9.38 | 21.7 | 18.9 | 33.6 | 0.782 | 0.67 | 0.517 | 0.396 | 0.439 | -1.37 | 0.932 | 0.265 | 0.461 | 10.9 | 47.6 | 51.6 | 1.09 |
| hairsoft | 0.00564 | 11.2 | 24.2 | 18.5 | 33.6 | 0.788 | 0.674 | 0.524 | 0.381 | 0.41 | -1.34 | 0.934 | 0.265 | 0.467 | 10.1 | 47.2 | 52.4 | 1.1 |

**B sky (pond_poplars, 650,150-850,290)**

| switch | dE | block8 | block16 | grid | gridP | L_2-4 | L_4-8 | L_8-16 | L_16-32 | L_32-64 | slope | Lhf | ahf | bhf | flat% | step10% | axis% | cells |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | 0 | 1.56 | 3.44 | 12.8 | 37.2 | 0.406 | 0.358 | 0.295 | 0.228 | 0.187 | -1.43 | 0.461 | 0.275 | 0.364 | 10.6 | 35.1 | 27.5 | 0.732 |
| dither | 0.479 | 2.19 | 4.37 | 12.8 | 37.2 | 0.396 | 0.354 | 0.294 | 0.228 | 0.188 | -1.45 | 0.455 | 0.211 | 0.323 | 12 | 35.5 | 27.8 | 0.764 |
| dither_mono | 0.468 | 1.67 | 2.84 | 10.5 | 37.2 | 0.415 | 0.36 | 0.296 | 0.229 | 0.188 | -1.43 | 0.465 | 0.211 | 0.323 | 9.49 | 34.9 | 27.5 | 0.711 |
| relief | 0.252 | 1.67 | 3.25 | 11 | 37.2 | 0.405 | 0.349 | 0.291 | 0.225 | 0.188 | -1.44 | 0.456 | 0.275 | 0.364 | 13.2 | 36 | 27.6 | 0.745 |
| weave | 0.26 | 1.69 | 3.12 | 12.5 | 37.2 | 0.406 | 0.354 | 0.296 | 0.227 | 0.187 | -1.44 | 0.459 | 0.274 | 0.362 | 11.2 | 35.4 | 27.6 | 0.735 |
| ground | 0.447 | 1.83 | 3.56 | 11.3 | 37.2 | 0.408 | 0.361 | 0.298 | 0.228 | 0.193 | -1.43 | 0.462 | 0.276 | 0.363 | 11 | 35.2 | 27.5 | 0.73 |
| varnish | 7.82 | 1.94 | 3.39 | 12.5 | 37.2 | 0.45 | 0.396 | 0.325 | 0.25 | 0.204 | -1.42 | 0.503 | 0.271 | 0.387 | 9.17 | 35.7 | 27.5 | 0.73 |
| cracks | 0.276 | 1.59 | 2.71 | 30.7 | 5.59 | 0.144 | 0.171 | 0.174 | 0.144 | 0.144 | -1.91 | 0.238 | 0.275 | 0.356 | 13.8 | 28.1 | 29.5 | 0.95 |
| stipple | 1.71 | 2.32 | 4.78 | 17.3 | 5.59 | 0.394 | 0.328 | 0.246 | 0.207 | 0.27 | -1.53 | 0.403 | 0.233 | 0.261 | 28.5 | 41.8 | 34.2 | 1.03 |
| dipcells | 1.1 | 1.74 | 3.53 | 31.2 | 31.9 | 0.415 | 0.387 | 0.338 | 0.278 | 0.281 | -1.58 | 0.486 | 0.29 | 0.394 | 6.96 | 33.6 | 28.8 | 0.92 |
| jitter | 0.493 | 1.94 | 4.21 | 8.29 | 21.3 | 0.406 | 0.356 | 0.289 | 0.22 | 0.179 | -1.37 | 0.443 | 0.279 | 0.37 | 11.4 | 36 | 28.7 | 0.73 |
| aim | 1.44 | 1.6 | 3.93 | 15.1 | 37.2 | 0.397 | 0.341 | 0.277 | 0.215 | 0.19 | -1.42 | 0.434 | 0.25 | 0.283 | 13.7 | 36.3 | 28.6 | 0.757 |
| aimfine | 0.674 | 1.52 | 3.07 | 15.2 | 37.2 | 0.404 | 0.352 | 0.29 | 0.22 | 0.179 | -1.38 | 0.46 | 0.246 | 0.347 | 10.8 | 35.5 | 27.7 | 0.714 |
| fill | 0.468 | 1.72 | 3.71 | 14.4 | 37.2 | 0.408 | 0.36 | 0.3 | 0.233 | 0.204 | -1.47 | 0.462 | 0.288 | 0.378 | 10.6 | 35.2 | 27.5 | 0.745 |
| hairsoft | 0.23 | 1.96 | 4.13 | 10.9 | 37.2 | 0.403 | 0.351 | 0.292 | 0.225 | 0.189 | -1.45 | 0.446 | 0.26 | 0.336 | 11.6 | 35.5 | 27.8 | 0.756 |

**Lab sky A (sky_A.lua, 350,260-550,400)**

| switch | dE | block8 | block16 | grid | gridP | L_2-4 | L_4-8 | L_8-16 | L_16-32 | L_32-64 | slope | Lhf | ahf | bhf | flat% | step10% | axis% | cells |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | 0 | 1.61 | 2.32 | 50.9 | 8.13 | 0.349 | 0.513 | 0.75 | 0.599 | 0.42 | -2.16 | 0.543 | 0.3 | 0.347 | 0.0614 | 23.8 | 25.2 | 0.781 |
| dither | 0.457 | 1.55 | 2.46 | 54.4 | 8.13 | 0.337 | 0.511 | 0.749 | 0.6 | 0.42 | -2.17 | 0.539 | 0.25 | 0.307 | 0.0614 | 23.8 | 25.1 | 0.788 |
| dither_mono | 0.448 | 1.97 | 2.97 | 45.6 | 8.13 | 0.357 | 0.515 | 0.75 | 0.599 | 0.421 | -2.15 | 0.547 | 0.25 | 0.307 | 0.061 | 23.7 | 25.1 | 0.776 |
| relief | 0.337 | 1.09 | 1.93 | 33.7 | 8.13 | 0.342 | 0.505 | 0.744 | 0.599 | 0.42 | -2.18 | 0.534 | 0.301 | 0.347 | 0.0792 | 23.9 | 25.3 | 0.787 |
| weave | 0.411 | 1.43 | 2.29 | 18.1 | 37.2 | 0.342 | 0.497 | 0.74 | 0.587 | 0.411 | -2.16 | 0.534 | 0.298 | 0.344 | 0.0582 | 23.7 | 24.8 | 0.779 |
| ground | 0.73 | 1.79 | 3 | 70.3 | 8.13 | 0.343 | 0.499 | 0.734 | 0.581 | 0.4 | -2.13 | 0.532 | 0.299 | 0.346 | 0.0509 | 23.6 | 25.1 | 0.774 |
| varnish | 7.27 | 1.68 | 2.62 | 52 | 8.13 | 0.363 | 0.537 | 0.785 | 0.627 | 0.441 | -2.16 | 0.568 | 0.291 | 0.368 | 0.0453 | 23.8 | 25.1 | 0.782 |
| cracks | 0 | 1.61 | 2.32 | 50.9 | 8.13 | 0.349 | 0.513 | 0.75 | 0.599 | 0.42 | -2.16 | 0.543 | 0.3 | 0.347 | 0.0614 | 23.8 | 25.2 | 0.781 |
| stipple | 5.02 | 1.05 | 1.54 | 103 | 8.28 | 0.135 | 0.13 | 0.137 | 0.145 | 0.191 | -2.18 | 0.147 | 0.258 | 0.326 | 29.7 | 23.5 | 39.9 | 1.57 |
| dipcells | 1.61 | 0.676 | 2.17 | 40.3 | 8.13 | 0.35 | 0.518 | 0.788 | 0.619 | 0.431 | -2.16 | 0.562 | 0.305 | 0.357 | 0.0628 | 23.8 | 25.7 | 0.827 |
| jitter | 0.491 | 1.71 | 2.72 | 62.4 | 8.13 | 0.343 | 0.504 | 0.732 | 0.579 | 0.409 | -2.13 | 0.54 | 0.302 | 0.35 | 0.0596 | 23.7 | 25.2 | 0.773 |
| aim | 2.31 | 2.24 | 2.45 | 46.6 | 8.13 | 0.265 | 0.377 | 0.547 | 0.463 | 0.364 | -2.25 | 0.391 | 0.281 | 0.292 | 0.521 | 24.1 | 25.4 | 0.92 |
| aimfine | 0.769 | 1.8 | 2.77 | 49.2 | 8.13 | 0.354 | 0.523 | 0.762 | 0.609 | 0.409 | -2.14 | 0.547 | 0.294 | 0.342 | 0.0698 | 24 | 25 | 0.768 |
| fill | 0.617 | 1.5 | 2.38 | 65.6 | 8.13 | 0.354 | 0.521 | 0.76 | 0.603 | 0.421 | -2.14 | 0.558 | 0.314 | 0.37 | 0.045 | 23.9 | 25.1 | 0.783 |
| hairsoft | 0.178 | 1.72 | 2.56 | 50.9 | 8.13 | 0.325 | 0.493 | 0.745 | 0.599 | 0.42 | -2.2 | 0.527 | 0.297 | 0.341 | 0.0607 | 23.7 | 25.1 | 0.797 |

### No cracks (craquelure off plus the switch; base = cracks off only)

**A sky (willows, 650,60-850,200)**

| switch | dE | block8 | block16 | grid | gridP | L_2-4 | L_4-8 | L_8-16 | L_16-32 | L_32-64 | slope | Lhf | ahf | bhf | flat% | step10% | axis% | cells |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | 0 | 0.894 | 1.49 | 32.7 | 4.84 | 0.194 | 0.258 | 0.396 | 0.38 | 0.376 | -2.44 | 0.299 | 0.232 | 0.298 | 2.79 | 26.6 | 25.9 | 1.35 |
| dither | 0.468 | 0.85 | 1.74 | 48.6 | 4.84 | 0.17 | 0.254 | 0.396 | 0.381 | 0.375 | -2.48 | 0.291 | 0.156 | 0.246 | 3.16 | 26.7 | 25.7 | 1.39 |
| dither_mono | 0.462 | 0.886 | 2.06 | 41.7 | 4.84 | 0.21 | 0.261 | 0.397 | 0.382 | 0.375 | -2.42 | 0.306 | 0.156 | 0.246 | 2.55 | 26.5 | 25.7 | 1.32 |
| relief | 0.319 | 0.712 | 1.42 | 22.7 | 4.84 | 0.188 | 0.24 | 0.38 | 0.374 | 0.375 | -2.5 | 0.285 | 0.232 | 0.298 | 5.03 | 27.3 | 26 | 1.4 |
| weave | 0.384 | 0.598 | 1.14 | 17.1 | 31.9 | 0.195 | 0.251 | 0.39 | 0.375 | 0.366 | -2.43 | 0.296 | 0.231 | 0.297 | 3.03 | 26.6 | 25.5 | 1.36 |
| ground | 0.839 | 0.959 | 1.5 | 50.3 | 4.84 | 0.192 | 0.257 | 0.381 | 0.362 | 0.361 | -2.43 | 0.287 | 0.231 | 0.297 | 3.68 | 26.8 | 25.7 | 1.38 |
| varnish | 11.2 | 0.872 | 1.4 | 35.8 | 4.84 | 0.201 | 0.275 | 0.423 | 0.405 | 0.4 | -2.45 | 0.317 | 0.229 | 0.319 | 2.19 | 26.5 | 25.8 | 1.36 |
| stipple | 1.54 | 0.933 | 1.73 | 34.5 | 5.59 | 0.123 | 0.118 | 0.106 | 0.113 | 0.33 | -2.35 | 0.135 | 0.225 | 0.309 | 42 | 28.6 | 52.5 | 2.89 |
| dipcells | 1.01 | 0.646 | 1.34 | 56.3 | 4.84 | 0.186 | 0.241 | 0.369 | 0.348 | 0.323 | -2.37 | 0.295 | 0.234 | 0.32 | 3.12 | 26.1 | 26.4 | 1.35 |
| jitter | 0.665 | 1.31 | 1.47 | 29.6 | 4.84 | 0.192 | 0.256 | 0.393 | 0.362 | 0.359 | -2.4 | 0.294 | 0.23 | 0.301 | 3.62 | 26.7 | 25.7 | 1.35 |
| aim | 4.09 | 0.69 | 1.95 | 64.8 | 29.8 | 0.162 | 0.196 | 0.288 | 0.324 | 0.368 | -2.64 | 0.218 | 0.299 | 0.325 | 11.2 | 28.5 | 28.2 | 1.86 |
| aimfine | 0.702 | 0.948 | 1.35 | 40.1 | 5.59 | 0.197 | 0.263 | 0.408 | 0.369 | 0.37 | -2.36 | 0.293 | 0.236 | 0.292 | 1.54 | 24.9 | 26 | 1.35 |
| fill | 0.861 | 0.601 | 2.05 | 39.7 | 4.84 | 0.194 | 0.258 | 0.397 | 0.375 | 0.384 | -2.44 | 0.298 | 0.237 | 0.31 | 2.76 | 26.7 | 25.6 | 1.35 |
| hairsoft | 0.0561 | 0.875 | 1.49 | 34.2 | 4.84 | 0.193 | 0.258 | 0.396 | 0.379 | 0.376 | -2.44 | 0.299 | 0.232 | 0.298 | 2.79 | 26.6 | 25.9 | 1.35 |

**A pond (willows, 650,465-850,545)**

| switch | dE | block8 | block16 | grid | gridP | L_2-4 | L_4-8 | L_8-16 | L_16-32 | L_32-64 | slope | Lhf | ahf | bhf | flat% | step10% | axis% | cells |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | 0 | 8.4 | 11.9 | 12.5 | 33.6 | 0.368 | 0.383 | 0.313 | 0.255 | 0.376 | -1.69 | 0.632 | 0.266 | 0.425 | 16.2 | 45.6 | 78.5 | 1.9 |
| dither | 0.457 | 11.9 | 14.8 | 12.3 | 31.9 | 0.357 | 0.381 | 0.311 | 0.255 | 0.376 | -1.7 | 0.628 | 0.206 | 0.393 | 18.3 | 46.1 | 78.8 | 2.01 |
| dither_mono | 0.445 | 7.78 | 10.1 | 14.2 | 33.6 | 0.376 | 0.385 | 0.312 | 0.256 | 0.376 | -1.68 | 0.635 | 0.205 | 0.393 | 14.7 | 45.3 | 78.2 | 1.84 |
| relief | 0.318 | 8.33 | 11.7 | 12.3 | 31.9 | 0.365 | 0.37 | 0.3 | 0.254 | 0.373 | -1.71 | 0.622 | 0.266 | 0.423 | 27.9 | 49 | 80.6 | 2.06 |
| weave | 0.261 | 8.67 | 12.1 | 12.8 | 35.5 | 0.368 | 0.376 | 0.315 | 0.256 | 0.375 | -1.69 | 0.627 | 0.265 | 0.424 | 18.3 | 46.2 | 78.7 | 1.94 |
| ground | 0.743 | 10.3 | 13.6 | 14.1 | 33.6 | 0.376 | 0.386 | 0.296 | 0.265 | 0.368 | -1.67 | 0.626 | 0.259 | 0.421 | 21.5 | 47.6 | 79.9 | 1.93 |
| varnish | 8 | 8.72 | 11.8 | 10.2 | 35.5 | 0.385 | 0.402 | 0.329 | 0.268 | 0.392 | -1.69 | 0.667 | 0.273 | 0.429 | 14.6 | 45.9 | 78.3 | 1.91 |
| stipple | 0.0714 | 11.1 | 15.4 | 10.1 | 33.6 | 0.379 | 0.391 | 0.318 | 0.257 | 0.377 | -1.68 | 0.634 | 0.265 | 0.429 | 16.2 | 45.7 | 78.8 | 1.9 |
| dipcells | 3.2e-05 | 8.39 | 11.8 | 12.7 | 33.6 | 0.368 | 0.383 | 0.313 | 0.255 | 0.376 | -1.69 | 0.632 | 0.266 | 0.425 | 16.2 | 45.6 | 78.5 | 1.9 |
| jitter | 0.503 | 7.99 | 10.8 | 10.5 | 37.6 | 0.365 | 0.379 | 0.309 | 0.248 | 0.341 | -1.62 | 0.624 | 0.263 | 0.43 | 16 | 45.4 | 77.8 | 1.92 |
| aim | 1.1 | 8.55 | 11.9 | 10.2 | 37.6 | 0.373 | 0.39 | 0.321 | 0.259 | 0.346 | -1.67 | 0.637 | 0.287 | 0.429 | 16.2 | 45.8 | 78.6 | 1.8 |
| aimfine | 0.387 | 8.15 | 10.6 | 12.2 | 33.6 | 0.368 | 0.383 | 0.313 | 0.249 | 0.336 | -1.63 | 0.63 | 0.27 | 0.426 | 15.2 | 45 | 77.6 | 1.85 |
| fill | 0.825 | 6.85 | 8.86 | 11.5 | 35.5 | 0.356 | 0.38 | 0.31 | 0.266 | 0.417 | -1.71 | 0.628 | 0.265 | 0.416 | 17.4 | 45.7 | 77.8 | 1.89 |
| hairsoft | 0.00571 | 7.78 | 10.8 | 11.4 | 33.6 | 0.368 | 0.383 | 0.313 | 0.255 | 0.376 | -1.69 | 0.63 | 0.265 | 0.421 | 16.2 | 45.5 | 78.7 | 1.9 |

**B sky (pond_poplars, 650,150-850,290)**

| switch | dE | block8 | block16 | grid | gridP | L_2-4 | L_4-8 | L_8-16 | L_16-32 | L_32-64 | slope | Lhf | ahf | bhf | flat% | step10% | axis% | cells |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| base | 0 | 1.59 | 2.71 | 30.7 | 5.59 | 0.144 | 0.171 | 0.174 | 0.144 | 0.144 | -1.91 | 0.238 | 0.275 | 0.356 | 13.8 | 28.1 | 29.5 | 0.95 |
| dither | 0.479 | 2.21 | 5.04 | 53.6 | 5.59 | 0.11 | 0.165 | 0.173 | 0.145 | 0.143 | -2 | 0.227 | 0.213 | 0.314 | 15.3 | 28.5 | 29.9 | 1.03 |
| dither_mono | 0.468 | 1.09 | 2.39 | 30.7 | 15.4 | 0.166 | 0.175 | 0.176 | 0.145 | 0.144 | -1.85 | 0.247 | 0.211 | 0.313 | 12.3 | 27.8 | 29.4 | 0.901 |
| relief | 0.251 | 1.29 | 2.18 | 29.8 | 15.4 | 0.14 | 0.152 | 0.168 | 0.142 | 0.145 | -1.98 | 0.228 | 0.275 | 0.355 | 17.3 | 28.8 | 29.4 | 0.988 |
| weave | 0.261 | 1.11 | 2.7 | 37 | 15.4 | 0.145 | 0.163 | 0.174 | 0.144 | 0.144 | -1.92 | 0.235 | 0.275 | 0.353 | 14.6 | 28.3 | 29.3 | 0.958 |
| ground | 0.449 | 1.19 | 2.32 | 61.3 | 5.59 | 0.147 | 0.178 | 0.18 | 0.144 | 0.146 | -1.85 | 0.241 | 0.276 | 0.355 | 14.5 | 28.2 | 29.3 | 0.95 |
| varnish | 7.87 | 1.31 | 2.33 | 21 | 5.59 | 0.146 | 0.179 | 0.185 | 0.154 | 0.153 | -1.93 | 0.249 | 0.271 | 0.38 | 12.3 | 28.3 | 29.3 | 0.965 |
| stipple | 1.73 | 0.754 | 1.7 | 133 | 5.59 | 0.122 | 0.11 | 0.0824 | 0.121 | 0.243 | -2.41 | 0.123 | 0.233 | 0.25 | 40 | 24.3 | 56.5 | 1.86 |
| dipcells | 1.1 | 1.33 | 2.33 | 20.4 | 31.9 | 0.166 | 0.228 | 0.243 | 0.212 | 0.256 | -2.1 | 0.285 | 0.291 | 0.388 | 8.6 | 28.7 | 31.3 | 1.2 |
| jitter | 0.495 | 1.27 | 3.05 | 24.8 | 5.59 | 0.143 | 0.165 | 0.168 | 0.133 | 0.132 | -1.8 | 0.201 | 0.28 | 0.362 | 14.7 | 25.7 | 31.8 | 0.984 |
| aim | 1.45 | 0.755 | 2.11 | 43.6 | 5.59 | 0.134 | 0.146 | 0.153 | 0.13 | 0.146 | -1.98 | 0.194 | 0.25 | 0.273 | 17.3 | 25.9 | 31.9 | 1.04 |
| aimfine | 0.677 | 1.38 | 2.34 | 32.8 | 5.59 | 0.139 | 0.158 | 0.166 | 0.133 | 0.128 | -1.84 | 0.235 | 0.246 | 0.338 | 14.4 | 28.4 | 29.7 | 0.921 |
| fill | 0.47 | 1.73 | 2.97 | 30.1 | 5.59 | 0.149 | 0.176 | 0.181 | 0.151 | 0.16 | -1.93 | 0.241 | 0.288 | 0.37 | 13.9 | 28.3 | 29.4 | 0.972 |
| hairsoft | 0.232 | 1.4 | 2.57 | 35.6 | 5.59 | 0.131 | 0.156 | 0.17 | 0.142 | 0.144 | -2.02 | 0.208 | 0.26 | 0.327 | 15.2 | 27 | 30.3 | 1.02 |

## Reproducing, and checks

```sh
cargo build --release -p easel
PAINT_TEXOFF=stipple target/release/easel run notes/lab/sky_A.lua --width 3200 --crop 350,260,550,400 --out out/v.png
PAINT_TEXOFF=cracks,aim target/release/easel run willows.lua --width 3200 --crop 650,60,850,545 --out out/w.png
uv run scripts/texture_metrics.py out/v.png
```

(`willows.lua` = `git show r7-arm1:paintings/lua/willows.lua`; for B, copy
`pond_poplars.rs` from r7-arm2 into `paintings/src/bin/` and run
`cargo paint pond_poplars -- --full --crop 650,150,850,290 --out out/b.png`.)

- With `PAINT_TEXOFF` unset the engine is main's: `notes/loops/l5_near.lua`
  and `l3_green.lua` at 1000 px byte-identical to a build of main
  (sha256 `cb7e2f7e…a2159fe` and `581c167d…be05883`).
- `cargo test --workspace`: all pass; `cargo test --release -p easel --test hand_time`: 5 passed.
- The switches are experiment scaffolding (`crates/paint/src/texoff.rs`,
  one guarded line or two at each suspect, listed above). They read the
  environment once per process; remove them when the question is settled.
