# Measured pigment mixer, round 5

The source model and tables are pushed at [ec2a760](https://github.com/aliceisjustplaying/claude-paint/commit/ec2a760646454d5579dac5823b4648ec00d585d8) on [#244](https://github.com/aliceisjustplaying/claude-paint/pull/244). This is numerical evidence, not visual acceptance or a green gate. Nothing was merged or deployed.

The decoder keeps linear two-constant KM and the 21-float latent. It evaluates a finite 100 µm film over one fixed measured white field and uses one calibrated constant recipe weight per modeled paint. K/S are jointly fitted to the declared source spectra, with no concentration law, changed measurement, per-pair correction or picture/main fitting input. The 39 RGB-only strengths are unchanged.

The revised test has 12 rows. Five old test rows are now fitted: 13F, 16D, 16F, 17D and 17F. Previously selection-touched 16E is also fitted. No independent extender test remains. The old 17 are still scored in full, the six added fits are labeled and unused pure 13A is separate. The revised rows were viewed during model selection; 08E also had earlier selection exposure.

| Group | round 4 median / mean / worst | round 5 median / mean / worst |
|---|---|---|
| Same old 17, including five now fitted | 0.027 / 0.062 / 0.240 | 0.023 / 0.022 / 0.046 |
| Same revised 12, used for model selection | 0.023 / 0.025 / 0.050 | 0.023 / 0.023 / 0.041 |
| All 33 displayed mixtures, including fit rows | 0.021 / 0.041 / 0.240 | 0.021 / 0.019 / 0.066 |

The revised median is slightly higher at full precision (0.023190 versus 0.022956), while its mean and worst improve. Main is a reference at the still-current `95dba3f`: its revised-12 median / mean / worst are 0.077 / 0.102 / 0.263. New is closer on 11/12 revised rows and 16/17 old rows; main remains closer on 08F. The round-4 comparison is freshly rescored from its frozen binary with the same normalized recipe shares (`round4-control-round5.json`).

The real-paint sheet is `real-paint-sheet-round5.png`, measured | new | main. `score-round5.md` and `.json` contain every row. The strength column shows the chromatic paint's constant; partner weights also apply and are recorded in the generated calibration. The unchanged 08F source percentages sum to 100.3%, so both engines receive their normalized ratio.

The selected joint solve lowers the worst errors and closes the priority cochineal/extender rows, with a revised-mean tradeoff against sequential-only fitting. Later joint initialization, weight recalibration and a shared global refinement do not improve both old and revised means. `iteration-ledger-round5.md`, `iteration-metrics-round5.json` and the compressed model snapshots record those tradeoffs. This is stopping evidence for the tried changes, not proof of a global optimum.

08F and 13G remain above 0.03 in the revised test. Old-test 16F, now fitted, also remains above it. `remaining-errors-round5.md` gives one-line causes and includes the other displayed residuals; `.json` records their lightness/chroma components and fixed-versus-actual-backing counterfactuals without any new fitting.

`guards-vs-round4-round5.md` puts every unchanged guard measurement beside round 4. The original seven tests are now 5 pass / 2 fail, versus 4 / 3. The paint library is 107 pass / 4 fail / 8 ignored. All 9 ck7, 20 replay-checkpoint and 4 Python reconstruction tests pass. All 12 old logs replay; ten still exceed unchanged goldens. Smoothness is 189 endpoint / 139 interior anomaly ramps, versus 300 / 193 in round 4. The full method and red [CI 774](https://ci.mosphere.at/repos/7/pipeline/774/2) are disclosed in `verification-round5.md`.

The same eight ramps and ten recipes are in `swatches-main-vs-new-round5.png`, with all 1,045 independent ramp values and ten recipe values adjacent. Actual-palette trace strengths are in `strength-comparison-round5.md` and `.csv`; measured Prussian at 1% still exceeds the raw-RGB guard's 0.33 bound on that separate path.

`f0dc5e-main-vs-new-round5.png` shows *Last Light on the Tarn* (Claude Opus 5.5, round unknown), [studio f0dc5e](https://stillwet.art/studio/?p=paint-studio-f0dc5e), in gallery light. Main is above and round 5 below. New was replayed with the rebuilt, frozen committed binary through all 108 chunks, 640 × 479. Only the original log's engine header changes from 8 to 9. The current-main pane is reused from round 4 and its pixels and still-current commit are verified. The new replay took 594.5 seconds including checkpoint writes, not a speed comparison. Binary, input and output hashes are in `gallery-provenance-round5.json`.

All five generated files reproduce byte-for-byte from the current generator. `provenance-round5.json` audits source-data preservation, declared fit/test separation, runtime order and packed values, unchanged original split/guards and reproduction. `manifest-round5.json` lists and hashes every other round-5 file. Earlier round-4 files are not overwritten. Large replay checkpoints stay in scratch and are not Drive deliverables.
