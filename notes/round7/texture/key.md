# Key: texture variants (open after judging)

Each number is one render of the same four windows (see texture.md).
Family "as painted": one switch off, everything else as rendered for Alice (cracks included).
Family "no cracks": craquelure off *and* one switch off (so the texture under the cracks can be judged).
The Lab sky has no cracks, so the no-cracks family has no `labsky` file: its Lab sky is the as-painted variant with the same switch.

| number | family | switch (`PAINT_TEXOFF`) | what it takes away |
|---|---|---|---|
| v01 | as painted | `jitter` | no mix jitter (every pile mixed the same) |
| v02 | as painted | `ground` | grounds laid flat (no knife texture, no brushed top ground) |
| v03 | no cracks | `cracks,jitter` | no mix jitter (every pile mixed the same) |
| v04 | as painted | `stipple` | stipple passes skipped |
| v05 | as painted | `cracks` | no craquelure |
| v06 | no cracks | `cracks,hairsoft` | softer bristle contacts (radius floor 1.2 / 1.5 px) |
| v07 | as painted | `dipcells` | stipple without patches (a pile's touches scattered over its passage) |
| v08 | no cracks | `cracks,ground` | grounds laid flat (no knife texture, no brushed top ground) |
| v09 | no cracks | `cracks,dipcells` | stipple without patches (a pile's touches scattered over its passage) |
| v10 | no cracks | `cracks,aimfine` | aimed recipes on a 4x finer grid of the underlayer |
| v11 | as painted | `hairsoft` | softer bristle contacts (radius floor 1.2 / 1.5 px) |
| v12 | no cracks | `cracks,fill` | no look-and-fill dabs |
| v13 | no cracks | `cracks,dither` | 8-bit save rounds without dither |
| v14 | no cracks | `cracks,stipple` | stipple passes skipped |
| v15 | as painted | (none) | baseline (the engine as on main; cracks as the painter finished) |
| v16 | as painted | `varnish` | no varnish |
| v17 | no cracks | `cracks,dither_mono` | dither one value for all three channels (luma only) |
| v18 | no cracks | `cracks,aim` | no aiming over the underlayer (piles mixed by masstone) |
| v19 | no cracks | `cracks,weave` | flat support (no linen weave in the height field) |
| v20 | as painted | `aimfine` | aimed recipes on a 4x finer grid of the underlayer |
| v21 | as painted | `relief` | no relief lighting |
| v22 | no cracks | `cracks,relief` | no relief lighting |
| v23 | as painted | `dither_mono` | dither one value for all three channels (luma only) |
| v24 | as painted | `dither` | 8-bit save rounds without dither |
| v25 | no cracks | `cracks,varnish` | no varnish |
| v26 | as painted | `fill` | no look-and-fill dabs |
| v27 | as painted | `weave` | flat support (no linen weave in the height field) |
| v28 | as painted | `aim` | no aiming over the underlayer (piles mixed by masstone) |

v05 (no cracks) is also the baseline of the no-cracks family: compare the no-cracks numbers with it.
