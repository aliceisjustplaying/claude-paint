# Variant d as the engine (branch r7-d)

Alice on the winter A/B (notes/round6/alice_review.md, notes/round7/winter_ab.md):
"So we want d": today's engine with the pointed-brush default off and the
glaze min-film floor off. Two focused reverts, each with a test that fails
first:

1. **Pointed tips opt-in** (reverts the default of 2ce7ee0). `Tool::round_sable`,
   `Tool::rigger`, `Style::detail` and `Style::line_tool` are blunt
   (`point: 0.0`) and lay the width asked for. The pointed model is
   unchanged: `Tool { point: 1.0, .. }`, Lua `brush{kind="round", width=w, point=1}`.
   Test `bristle::tip_tests::presets_are_blunt_and_lay_their_width`.
   notes/tip.md, "Round 7: opt-in".
2. **Glaze film floor 1 → 0.05 µm** (reverts the thin-film part of 1742baa).
   `canvas::MIN_FILM_UM` = 0.05 µm with the same C¹ fade (zero at
   0.025 µm), a numerical floor. The edge bugs it also guarded against
   were the NaN in `settle`, guarded there: with no floor at all
   `glaze_long_falloff_has_no_edge` and
   `glaze_through_blurred_mask_leaves_no_rectangle` still pass. The varnish
   and glaze peak film of `settle_film` borrowed the constant; it is now
   its own `surface::PEAK_FILM_UM`, still 1 µm (as in d). Test
   `canvas::tests::a_thin_veil_is_laid`. notes/drying.md.

`fresh2_winter`'s temporary A/B switches (`WINTER_POINT0`, `WINTER_RELIEF`)
are gone. The program is otherwise the port's, so it uses the Style's
relief (0.06, 0.006).

## Winter, 1000 px (`cargo paint fresh2_winter`, no env)

Per-pixel |diff| averaged over RGB, 0–255; L is the mean signed luma change.

| against | mean | p99 | max | L |
|---|---|---|---|---|
| d (`winter_ab/d_1000.png`) | 1.19 | 16.7 | 152 | −0.05 |
| original (round 2) | 2.96 | 29.3 | 164 | −0.60 |
| today before this branch (`winter_port/today_1000.png`) | 5.25 | 91.0 | 178 | −4.40 |

d itself is 3.01 from the original. The 1.19 to d is all program, not
engine: with the finish's relief at d's (0.2, 0.02) it is 0.97, and with
the style's detail sable also given `point: 1.0` (d blunted only the
brushes the painter made, not `st.detail()`, used by the ruin hatching and
the brook) the render is **byte-identical to d** (mean 0.000). So the
engine is d exactly, and the rest is 0.22 relief plus 0.97 from the
detail sable now being blunt too. (Checked in scratch builds, not
committed.) The lit oval is gone: in 100-unit tiles the mean luma difference to
d is within ±1.5 everywhere (the sky tiles within ±0.1), where the
stale build of this branch (below) showed c's +5 to +7 over the sky and
motifs.

## Benchmarks, 1000 px (main → this branch)

| | mean | max | p99 | L | pixels > 2 levels |
|---|---|---|---|---|---|
| l5_near, both | 7.57 | 196 | 116.0 | −5.24 | 47.5% |
| l5_near, tip only | 5.70 | 196 | 116.0 | −2.85 | 26.1% |
| l5_near, floor only (after the tip) | 2.19 | 30 | 11.3 | −2.39 | 26.2% |
| l3_green, both | 2.47 | 169 | 23.3 | −1.04 | 27.8% |
| l3_green, tip only | 2.16 | 182 | 22.7 | −0.52 | 22.7% |
| l3_green, floor only (after the tip) | 0.50 | 82 | 6.7 | −0.52 | 8.2% |

l5_near is darker all over: the fir wood, the lone spruce, the boulder's
lichen speckle and the grass tufts are heavier with blunt brushes (they
were painted with `brush("round")` and riggers and `fit` pressures). The
floor-only row is its three shadow glazes (0.22–0.42 coats through soft
masks and views): only glazes changed there, so it is their thin parts,
under 1 µm of film, now laid. l3_green
changes less: fewer small pointed marks.

## Files (notes/round7/winter_d/)

- `winter_1000.png`: winter, this branch, no switches.
- `sheet_winter_d_vs_now_1000.png`: d (left) and this branch (right).
- `l5_near_{before,after}_1000.png`, `l3_green_{before,after}_1000.png`
  and `sheet_*_before_after_1000.png` (before left).

## Hashes re-recorded on purpose

- Golden scene (commit 1): it drags a round sable and a rigger, now blunt.
- Replay hashes (`crates/easel/tests/hand_time.rs`): example.lua,
  l5_near.lua and overran.lua in commit 1 (they paint with
  `brush("round")` and riggers and never set `point`); l5_near again in
  commit 2 (glazes with films under 1 µm). The golden, example and overran
  don't glaze that thin and are unchanged by commit 2.

## A trap met on the way

Restoring a source file with `mv file.bak file` after a fail-first check
gives it the backup's older mtime, so cargo kept the release build of the
temporary edit (the 1 µm floor) and the first winter render still had the
oval. Everything above is from clean rebuilds (sources touched), and both
commits' release replay hashes were checked in a clean worktree. Restore
with `cp` (new mtime) or `touch` afterward.
