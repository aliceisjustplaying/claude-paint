# Check 13 (b) restatement: evidence

All runs are new. They were made on 2026-10-04 on the M3 Pro (macOS, aarch64) with rustc 1.97.1 in release builds, through `~/src/a/claude-paint-tools/lockrun`, against `d54b423` with the palette doc comment applied. `sienna-13b.patch` was applied uncommitted for the "after" runs and reverted afterward. The working-tree files carry no change. Logs from the runs are in this directory.

## Before (unchanged test)

`cargo test --release -p paint --test thinner_pigments -- --exact c13_burnt_sienna_shows_the_card_at_least_as_well_as_raw_sienna c13_every_pigment_value_is_af49348s --show-output`
→ (a) ok. Old (b) FAILED at `thinner_pigments.rs:119` with "burnt sienna shows 9.85% of the card, raw sienna 17.91%". Cargo exited 101 (`before_thinner_pigments.log`).

`cargo test --release -p paint --test look_sienna -- --exact --ignored --nocapture siennas_by_contrast_ratio` reproduces `notes/look/logs/sienna.txt` digit for digit, the thinner-0.5 rows included (mean paint 1.64 µm). These rows come from this lane's base, `d54b423`, not from A+B. The final card is still owed on the A+B candidate.

## After (patch applied)

`cargo test --release -p paint --test thinner_pigments -- --show-output`
→ 3 passed, 1 ignored (`print_tube_table`). Cargo exited 0 (`after_thinner_pigments.log`).

### Direct films (new (b)), luminance contrast ratio Y = Y(over black) / Y(over white)

Substrates as measured:

- black/80% white chart: black RGB 0, 0, 0 (Y 0); white RGB 0.8, 0.8, 0.8 (Y 0.8)
- bone black/lead white masstones: black RGB 0.0130, 0.0110, 0.0097 (Y 0.0113); white RGB 0.8632, 0.8148, 0.7157 (Y 0.8180)

| film | substrate | raw Y | burnt Y | burnt − raw | raw R/G/B | burnt R/G/B | retained abs. diff raw / burnt |
|---|---|---|---|---|---|---|---|
| 3 µm | chart | 0.0471 | 0.0352 | −0.0119 | .0439/.0469/.0907 | .0302/.0363/.0589 | 78.2% / 65.9% |
| 10 µm | chart | 0.1695 | 0.1537 | −0.0157 | .1429/.1771/.7980 | .1040/.1921/.6612 | 49.0% / 29.9% |
| 25 µm | chart | 0.4516 | 0.4378 | −0.0138 | .3414/.5219/.9998 | .2765/.7612/.9994 | 20.0% / 10.1% |
| 30 µm | chart | 0.5365 | 0.5117 | −0.0248 | .4029/.6307/1.0000 | .3368/.8773/.9999 | 15.2% / 7.9% |
| 3 µm | masstones | 0.0586 | 0.0474 | −0.0113 | .0548/.0586/.1125 | .0424/.0485/.0780 | 78.9% / 66.6% |
| 10 µm | masstones | 0.1748 | 0.1590 | −0.0158 | .1451/.1847/.8181 | .1096/.1998/.6903 | 50.0% / 30.8% |
| 25 µm | masstones | 0.4450 | 0.4270 | −0.0180 | .3300/.5227/.9999 | .2698/.7608/.9995 | 20.7% / 10.6% |
| 30 µm | masstones | 0.5273 | 0.4983 | −0.0291 | .3886/.6304/1.0000 | .3269/.8768/.9999 | 15.8% / 8.4% |

Luminance and retained absolute difference rank the siennas oppositely at every film, as the review response predicted. Green ranks with the retained difference from 10 µm up. Red and blue rank with luminance at every film, but blue only barely at 25 and 30 µm, where both ratios are near 1. The log has the RGB over each substrate.

Catalog against rendered values (one coat, 25 µm):

| | catalog hiding (grayscale surrogate) | rendered Y ratio, black/white 1.0 | rendered Y ratio, black/80% white |
|---|---|---|---|
| raw sienna | 0.40 | 0.3884 | 0.4516 |
| burnt sienna | 0.45 | 0.3778 | 0.4378 |

### Card diagnostic (old (b) quantity, brush-made, base code)

Card bands before the strokes: black RGB 0.0152, 0.0121, 0.0102 (Y 0.0126); white RGB 0.8286, 0.7891, 0.7022 (Y 0.7912). The film has equal pixels, with a mean of 37.92 µm over the bands and 10.07 µm over the canvas.

| | retained absolute substrate difference | contrast ratio Y (R/G/B) |
|---|---|---|
| raw sienna | 17.91% | 0.4998 (.4131/.5532/.9841) |
| burnt sienna | 9.85% | 0.4642 (.3431/.6545/.9680) |

These numbers depend on brush deposition and belong to `d54b423`. They are not the final A+B card.

### Tolerance check (after the margin was fixed)

The scratch probe was deleted after it ran (`f64_probe.log`). An f64 port of `ks_of`, `layer1` and `over` was compared with the engine's f32 `Paint::over` for all 16 film/substrate/tube cases. The largest difference in contrast ratio was 9.18e-8. The predeclared margin of 1e-4 is about 1000 times that. The smallest measured gap, 0.0113, is about 110 times the margin.

### Runner

- `bash scripts/tests/thinner_acceptance_runner.sh` passed 19 of 19 cases (`selftest.log`).
- `lockrun --timeout 120 -- scripts/test_thinner_acceptance --quick` passed 23 of 24 and exited 1 (`after_quick_summary.log`). Pigments, sienna (both new tests), paint (12) and easel (8) passed. The one failure was check 2, in the `rag` scene with thinner=0 (5 of 8 chunks differ). That failure predates this patch: HANDOVER-3 "Recorded baseline status" lists "check 2 `rag` scene (pending rebaseline)". The patch touches neither check 2 nor engine code. Without that pre-existing failure, the patched runner would exit 0.
