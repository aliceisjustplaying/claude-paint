# The varnish at 3200: worm lines along paint edges (round 6)

**Symptom.** The standard finish `wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()`
drew thin brown or dark lines along paint edges at 3200px: worms around every dab in the
lab2 oak's smooth crown (`notes/lab2/oakleaf.md`, failure 1), a dark-orange hairline along
the rock's lit base (`notes/lab2/rock.md`). `relief()` alone was clean; `vary=0` and
`dry()` first didn't help; `coats=0.12` nearly cleared it. At 1000 the finish was fine.

**Owner sheet:** `notes/varnish/owner_sheet.jpg` (four 3200 crops at 1:1 pixels, before
over after, labeled).

## Cause (measured)

`Canvas::glaze` (which `varnish{}` calls) laid its film with `surface::settle`, which
levels the *whole surface* (the dry relief plus the new film) as if all of it were
fluid, and then gives each pixel `(level − old).max(0)`. With a 2 µm film:

| scale | bristle band λ₁ | decay over 900 s | yield floor a_c |
|---|---|---|---|
| 3200, rock (0.094 mm/px) | 0.70 mm | 0.76 | 109 µm |
| 3200, oak and l3 (0.138 mm/px) | 0.62 mm | 0.64 | 74 µm |
| 1000, rock (0.30 mm/px) | 1.35 mm | 0.98 | 773 µm |
| 1000, oak (0.44 mm/px) | 1.98 mm | 1.00 | 2438 µm |

At 1000 nothing is above the yield floor, so nothing moves. At 3200 a dry stroke edge of
150–400 µm (steep at 0.1 mm per pixel) stands above the floor and gets "leveled" by a
quarter to a third: the level falls tens of µm below the shoulder (no film there) and
rises tens of µm above the foot. The local volume rescale keeps that shape, so the
varnish taken off the shoulders sits in a line at the foot of every step.

Measured on the rock_H finish at 3200 (crop 360–680 × 380–620; I dumped the old height,
the film laid and the film settled from `glaze` and analyzed them with
`notes/varnish/scripts/an.py` and `an2.py`):

- the film asked for was 2.12 µm; the median pixel got 1.01× that;
- **0.9% of pixels got more than 3×** (up to 38×): median 14.7 µm and max 79.8 µm of
  varnish, holding 7.2% of all the varnish. They sit at the foot of steps (median band
  coefficient d₁ = −168 µm);
- **1.5% got almost none** (under 0.1×): the shoulders (d₁ = +151 µm);
- ordinary pixels have |d₁| ≈ 20 µm, below the floor.

`notes/varnish/rock_film_map_before.jpg` marks the deep pixels red and the bare ones blue
over the picture. The red marks are exactly the brown dashes in
`rock_3200_before.jpg`, each with a blue shoulder beside it. Depth of color is
proportional to film thickness, so 15–80 µm reads as 2–10 coats of varnish: brown.

So the open issue in `notes/drying.md` had the right cause (a thin glaze leveling below
tall ridges) but the wrong side: the bare tops are there, but they're pale. **The dark
lines are the pooled volume at the steps' feet.** A floor on the peaks alone would not
have been enough without bounding the pool.

## Fix

A new `Canvas::settle_film` (`crates/paint/src/surface.rs`) for a thin fluid film over a
**dry** surface; `glaze` (and so `varnish{}`) uses it. Grounds and wet paint still use
`settle`.

- The film follows the relief; only its own thickness flows. In lubrication theory a
  film h on a convex spot of band amplitude A thins as dh/dt = −h³σAk⁴/3η, which
  integrates to **h = h₀ / √(1 + 2 (A/h₀)(T/τ₀))** (τ₀ is Orchard's τ at h₀, from the same
  `level_band`; A counts only above the same yield floor). Drainage slows as the film
  thins, so the peaks keep a film.
- Every peak keeps at least `MIN_FILM_UM` (1 µm, or all of it when less was laid): a
  wetting film; the resin has set before it drains thinner.
- What drains moves downhill about one bristle band (two box blurs of radius 2·r₁) and
  gathers in the concave spots there, weighted by how concave they are, **at most 2× the
  film laid** (`POOL_MAX`). What finds no place stays where it was. Volume is conserved
  locally, then exactly.

The varnish now reads as what it should be: a gentle, even, warm layer. It is a little
deeper in the weave's hollows and there are no lines.

Tests (`crates/paint/src/tests.rs`):
- `thin_film_over_dry_impasto_coats_peaks_and_pools_a_little`: 2.25 µm over 100–300 µm
  dry dabs at 0.094 mm/px. Volume is kept to 1e-4, every pixel gets ≥ `MIN_FILM_UM` and
  ≤ 2×, and over 80% get the film laid. It also checks that the old fluid `settle` still
  pools there (>10×), so the case stays covered.
- `varnish_over_impasto_is_even`: `glaze(varnish, 0.3)` over a 250 µm dab. Every pixel's
  warming lies within 0.3–2.2× the flat surface's. The old path fails it (0 to 2.8×).
- The golden scene was re-recorded (`UPDATE_GOLDEN=1`): its glazes now settle by the new
  rule (intended).

## Evidence

All 3200 crops, varnish `coats=0.3, vary=0.1`, then `relief()`. "Before" is the code
before this change and "after" is this branch. The oak is `notes/lab2/oakleaf_H.lua` with
the painter's `coats=0.12` workaround set back to 0.3 (the original problem).

| study | crop (units) | before | after |
|---|---|---|---|
| rock_H | 360–680 × 380–620 | `notes/varnish/rock_3200_before.jpg` | `rock_3200_after.jpg` |
| oakleaf_H (coats 0.3) | 270–590 × 150–390 | `oakleaf_3200_before.jpg` | `oakleaf_3200_after.jpg` |
| l5_near (boulder edge) | 440–640 × 300–460 | `l5_near_3200_before.jpg` | `l5_near_3200_after.jpg` |
| l3_green (rock, figure) | 320–500 × 420–520 | `l3_green_3200_before.jpg` | `l3_green_3200_after.jpg` |

What I saw: before, brown worms outline every dab in the oak crown, and a thin dark
outline runs around the crown against the sky. Every paint edge on both boulders is
hatched and flecked, and there's a dark-orange line along the rock's lit base. After, all
of it is gone, and the crops look like the relief-only renders
(`oakleaf_3200_relief_only.jpg`), only warmer. The soft dark patch low on the l3 rock is
the painter's (it is there with relief alone); the old varnish's flecks broke it up.

**How even the varnish is.** I compared each render to the same log with the varnish
removed (relief only), as a per-pixel transmittance in linear light
(`scripts/even2.py`):

| 3200 study | mean transmittance R, G, B: before / after | std of blue transmittance: before / after | pixels darkened >2× the mean: before / after |
|---|---|---|---|
| rock_H | 0.947, 0.897, 0.776 / 0.946, 0.894, 0.767 | 0.072 / 0.020 | 0.98% / 0.00% |
| oakleaf_H (0.3) | 0.962, 0.913, 0.827 / 0.957, 0.896, 0.770 | 0.178 / 0.044 | 8.56% / 0.00% |
| l5_near crop | 0.944, 0.896, 0.788 / 0.936, 0.877, 0.728 | 0.185 / 0.038 | 6.94% / 0.00% |
| l3_green crop | 1.037, 0.994, 0.887 / 0.945, 0.887, 0.758 | 0.368 / 0.021 | 41.87% / 0.00% |

The mean warming is now slightly stronger at 3200, because the film laid is now on every
pixel instead of being piled into a few saturated ones. It also matches 1000 exactly: for
l5_near at 1000 over the same window the transmittance is 0.936, 0.877, 0.728, before
and after this change.

**Benchmarks at 1000** (`easel run <log> --width 1000`, before vs after, 8-bit PNGs):
- `notes/loops/l5_near.lua`: not byte-identical. Mean abs difference 0.0002/255, max 13;
  339 pixels differ at all and 4 by more than 4 (a small cluster at pixel 549–560,
  362–381).
- `notes/loops/l3_green.lua`: not byte-identical. Mean 0.0001/255, max 1 (292 pixels
  differ by 1).

The replay hashes in `crates/easel/tests/hand_time.rs` (example.lua and l5_near at
160px) were re-recorded: 1 and 8 pixels moved by 1/255.

At 3200 they change where the old varnish drew lines (the crops above: mean abs
difference 4.7/255 for l5_near and 4.8/255 for l3_green, max 164 and 117).

## Using it

Nothing changes for painters. The finish is as before:
```lua
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
```
`coats` means the same thing and can go back to 0.3 where a painter thinned it to 0.12
to hide the worms (the lab2 oakleaf logs; I left those records as they are).

## Open issues

- The bristle band of `settle` is tied to the pixel (λ₁ = 3 pixels × 1.5), so its yield
  floor falls with resolution: that is why the old varnish went wrong only at 3200.
  Wet paint still levels with `settle` and may show milder versions of the same
  resolution dependence where a thin fluid stroke crosses tall dry impasto. `settle_film`
  is the model for that case if it turns up (split the wet layer's own leveling from the
  dry relief under it).
- Pooling is capped by a constant (2×), not derived from the hollow's shape and volume.
  It reads right. A fuller model would fill each hollow to its level and stop.
- The peak floor is `MIN_FILM_UM` for every glaze. A glaze that is wiped (a rag over
  the peaks) would want less, and that could be an option on `glaze`.
- Crops: like `settle`, `settle_film` ends with an exact volume rescale over the whole
  buffer, so a crop can differ very slightly from a whole render (as before). I did
  not measure it.
