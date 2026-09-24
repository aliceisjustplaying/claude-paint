# Round 2's winter painting on today's engine

Round 2's painter wrote `paintings/src/bin/fresh2_winter.rs` on branch
`amnesia-winter` (fa772d2): its own wood, prune, gnarl, limb_snow, spruce,
walker_fig and crow. It is ported here, to main (234d972, branch
r7-winter), as `paintings/src/bin/fresh2_winter.rs`, and in a scratch
worktree of r6-wet (62ace25) for the wet engine. Everything the painter
chose is unchanged: composition, colors, tools, stroke plans, seeds, the
order of work, the motif functions and the finish values.

    cargo paint fresh2_winter              1000 px
    cargo paint fresh2_winter -- --full    3200 px

## API changes made

One, the same on both engines: the `paint::Cracks` literal in the finish.

- `island_mm`, `ground_um` and `width_um` became `Option<f32>` (`None` now
  means "fit to the canvas"). The painter's values are wrapped:
  `Some(2.6)`, `Some(140.0)`, `Some(18.0)`.
- New fields, all set to the values that reproduce round 2's single even
  web:
  - main: `vary: 0.0, veil: 0.0, hierarchy: 0.0, patchy: 0.0, grain: 0.0,
    grime: 0.0`
  - r6-wet has only `vary` and `veil`: `vary: 0.0, veil: 0.0`

  `Cracks::even` sets hierarchy, patchy, grain and grime to 0 but leaves
  `vary: 1.0, veil: 0.5` from `Cracks::aged`. Those two fields came after
  round 2, so they are set to 0 here too: `vary <= 0` returns
  `Local::uniform()` (crack.rs, `crack_local`) and `veil > 0` is the only
  path to `varnish_veil`.

Other than that, the file is round 2's text. The compiler still warns about an unused `y` at
line 496; it's the painter's own code and left as is. On r6-wet only the
comment above the `Cracks` literal differs from the main file. `r6-wet` was
already checked out in another worktree, so the scratch worktree was
added `--detach` at r6-wet's head.

## Engine defaults that change the look (kept, not undone)

Beyond relief 0.06, the varnish and pinhole fixes, clip masks honored and
`[profile.test]`, one default changes this picture more than the others:
**`Tool::point`** (2ce7ee0, "Pointed-tip brush model"). `Tool::round_sable`
and `Tool::rigger` now default to `point: 1.0`, and `st.line_tool` (from
the style's line brush) is also affected. With a point, the footprint's
half-width gets an extra factor `cone(p)` = `sqrt(min(p / 0.85, 1))`
(bristle.rs). Below pressure 0.85 a stroke is narrower than it was. The
painter's own `spread()` / `pressure_for()` invert round 2's footprint
formula (`(0.45 + 0.55 p)(1 + splay (p − 0.5))`) and don't know the cone,
so every tapered limb, spruce tier, crow, fence rail, grass blade and the
figure paint thinner than planned. The painter's code isn't changed for this.

As a diagnostic only (not a version, not on the sheets), the same port
was rendered at 1000 with every round, rigger and line tool set to
`point: 0.0`: `winter_port/diagnostic_today_point0_1000.png`. The numbers
below separate the two causes.

## Files (notes/round7/winter_port/)

- `sheet_whole_1000.png`: original, today, today-wet, whole, at 1000, side
  by side
- `crop_oak_crown_3200.png`, `crop_ruin_path_3200.png`,
  `crop_fir_group_3200.png`: 1:1 from the 3200 renders, same window in all
  three
- plain: `original_1000.png`, `today_1000.png`, `today_wet_1000.png`,
  `original_3200.png`, `today_3200.png`, `today_wet_3200.png`

The original renders were built today from round 2's branch
(`r2winter/out/fresh2_winter{,_full}.png`), not re-rendered here.

Crop windows, in picture units (1000 wide, ×3.2 at 3200). Each is the
motif's geometry plus a margin:

| window | picture units x, y | 3200 px (w×h+x+y) | from |
|---|---|---|---|
| oak crown | 120–460, 200–520 | 1088×1024+384+640 | oak base (262, 598), height 390 → top 208; dark-pixel extent x 130–450 |
| ruin with path | 362–662, 340–660 | 960×1024+1158+1088 | ruin polygon x 462–562, y 351–462, centered at x 512; brook's upper reaches and the walker (590, 592) below |
| fir group | 690–930, 400–570 | 768×544+2208+1280 | spruce stems x 712–889, tallest top y 414, feet y 541–553, reach ≤ 0.24 h |

## What differs, measured

|diff| is the per-pixel absolute difference averaged over RGB, 0–255;
"L" is mean signed luma change (second minus first). Regions are in
picture units. The script is not committed; these are its outputs.

### 3200 px

| region | orig → today mean / p99 / max | orig → wet mean / p99 / max | today → wet mean / p99 / max |
|---|---|---|---|
| whole | 5.18 / 46.0 / 165.7 (L +2.51) | 5.23 / 46.3 / 166.0 (L +2.45) | 0.88 / 5.0 / 160.0 |
| sky, y < 380 | 3.71 / 11.3 / 162.3 (L +2.00) | 3.74 / 11.3 / 161.3 | 0.85 / 3.3 / 108.3 |
| far range, ridge, mist, y 380–460 | 8.57 / 65.0 / 164.3 | 8.65 / 65.3 / 164.7 | 0.98 / 10.7 / 160.0 |
| snow field, y > 460 | 6.29 / 72.0 / 165.7 | 6.34 / 73.0 / 166.0 | 0.91 / 6.0 / 155.0 |
| oak crown window | 8.21 / 95.0 / 164.0 (L +6.14) | 8.24 / 95.3 / 163.3 | 0.72 / 5.7 / 139.0 |
| ruin with path window | 7.42 / 48.0 / 164.0 (L +4.87) | 7.42 / 48.0 / 163.3 | 0.86 / 6.0 / 136.7 |
| fir group window | 12.58 / 148.3 / 165.7 (L +7.49) | 12.66 / 149.7 / 166.0 | 1.35 / 15.3 / 160.0 |
| moon, 618–718 × 68–168 | 2.10 / 6.7 / 53.7 | 2.11 / 6.3 / 54.0 | 0.99 / 4.0 / 11.3 |
| foreground, y > 600 | 4.56 / 42.3 / 123.3 | 4.60 / 43.0 / 120.3 | 0.95 / 5.7 / 95.0 |

### 1000 px

| region | orig → today | orig → wet | today → wet | orig → point-0 diagnostic |
|---|---|---|---|---|
| whole | 6.18 / 86.0 / 162.3 | 6.26 / 87.0 / 162.3 | 1.11 / 6.0 / 97.0 | 4.46 / 29.0 / 153.3 |
| sky | 4.27 / 34.3 / 161.3 | 4.41 / 34.3 / 161.7 | 1.28 / 5.3 / 42.0 | 3.43 / 10.3 / 131.7 |
| far range, ridge, mist | 10.50 / 116.3 / 160.7 | 10.40 / 116.0 / 160.7 | 1.11 / 10.3 / 44.7 | 7.53 / 44.7 / 150.3 |
| snow field | 7.63 / 100.3 / 162.3 | 7.68 / 102.0 / 162.3 | 0.86 / 6.0 / 97.0 | 5.02 / 35.0 / 153.3 |
| oak crown window | 12.19 / 135.7 / 161.3 | 12.07 / 135.7 / 161.7 | 0.86 / 6.0 / 44.7 | 6.42 / 41.0 / 142.0 |
| ruin with path window | 8.21 / 58.0 / 156.3 | 8.12 / 58.0 / 156.0 | 0.88 / 6.0 / 42.7 | 7.12 / 40.0 / 135.0 |
| fir group window | 15.63 / 143.3 / 162.3 | 15.85 / 146.0 / 162.3 | 1.29 / 12.0 / 63.3 | 7.92 / 62.3 / 153.3 |
| foreground, y > 600 | 5.43 / 57.0 / 106.7 | 5.43 / 57.7 / 108.0 | 0.92 / 6.0 / 94.0 | 4.05 / 32.3 / 94.7 |

### Dark marks: share of pixels with luma < 80 in each window

| | oak crown | fir group | ruin with path | fence (700–1000, 560–722) |
|---|---|---|---|---|
| 1000 original | 10.17% | 16.37% | 1.66% | 7.26% |
| 1000 today | 6.03% | 10.63% | 0.79% | 3.73% |
| 1000 today-wet | 6.03% | 10.52% | 0.81% | 3.72% |
| 1000 today, point 0 (diagnostic) | 10.34% | 16.73% | 1.65% | 7.92% |
| 3200 original | 7.99% | 15.47% | 1.41% | 5.48% |
| 3200 today | 6.77% | 12.87% | 1.01% | 4.61% |
| 3200 today-wet | 6.76% | 12.73% | 1.01% | 4.60% |

### Where the differences come from

- **Dark marks (wood, firs, crows, fence, grasses, figure):** thinner and
  lighter on today's engine and on the wet one. At 1000 the dark area in
  the oak's window falls from 10.2% to 6.0% and in the firs from 16.4% to
  10.6%. With `point: 0` (diagnostic) the dark shares come back to the
  original's (10.3%, 16.7%), and in those windows the mean diff to the
  original halves (oak 12.2 → 6.4, firs 15.6 → 7.9). Most of the change
  in the marks is the pointed-tip default. At 3200 the dark share shrinks
  less (oak 8.0% → 6.8%, firs 15.5% → 12.9%); the cause of that was not
  measured.
- **Everything else (sky, far range, mist, snow):** a smaller, wide
  difference that remains in the point-0 diagnostic (whole 4.46 mean, p99
  29). Today's picture is a little lighter overall (mean luma +2.5 at 3200,
  +2.0 in the sky, +4 to +7 in the motif windows, where the thinner dark
  marks add to it). The engine changes since round 2 (relief, varnish, the
  pinhole fix, clip masks) are the candidates. The diff wasn't broken down
  further by cause.
- **Moon:** nearly unchanged (mean 2.1 at 3200).
- **Grasses:** the same stroke plan. The original places 328 tufts at
  1000 (a re-render from round 2's branch here printed 328 and is
  byte-identical to the provided original); main 328 at 1000; wet 328 at
  1000 and 329 at 3200.
- **Today vs today-wet:** small everywhere: mean 0.88 (3200) and 1.11
  (1000), p99 5–6, isolated maxima up to 160 where a mark lands a pixel or
  two apart. The largest regional mean is the fir group (1.35 at 3200).
  In the moon window the wet render is slightly darker (L −1.9 at 1000).

## Render times (this machine, while other renders ran)

main: 1000 in 23 s, 3200 in 107 s. r6-wet: 1000 in 52 s, 3200 in 340 s.
