# Mixer round 4: measured tubes keep their own pigment

Code: [5fa01c3](https://github.com/aliceisjustplaying/claude-paint/commit/5fa01c3219de3503c71ab2a7bc3f103cd2a2489d), on [the existing mixer PR](https://github.com/aliceisjustplaying/claude-paint/pull/244). These are measurements and comparison images, not a visual judgment or an acceptance decision. No fifth round was started.

Fifteen catalog tubes enter as their own measured K and S. The other 39 still use RGB reconstruction through the same five-primary inverse table and keep their original scalar strengths. The 21-float latent carries eighteen pigment concentrations and three RGB residuals. Direct per-wavelength two-constant KM replaces the fixed-dimensional forward lookup. Piles, brush paint, knives and wet exchange retain that latent instead of reconstructing it from RGB. The inverse table is byte-identical to round 3; replacing forward interpolation with exact evaluation means RGB mixing outputs are not claimed byte-identical to round 3.

New nonprimary coefficients use each pure film and its single fixed round-3 fit sample, or only the pure film for the two extenders. Primary coefficients and their own ladders are unchanged. No test sample, main output or picture calibrates the new coefficients. The split function is unchanged. The fixed test retains 08E, which round 2 had read during primary selection; it is not labeled wholly untouched.

| group | mixtures | round 3 new median / mean / worst | round 4 new median / mean / worst | main median / mean / worst |
|---|---|---|---|---|
| Fixed round-3 test | 17 | 0.065 / 0.082 / 0.156 | 0.027 / 0.062 / 0.240 | 0.117 / 0.136 / 0.397 |
| Fit, not independent | 11 | 0.036 / 0.048 / 0.165 | 0.006 / 0.007 / 0.024 | 0.034 / 0.040 / 0.140 |
| Previously selection-touched | 5 | 0.056 / 0.082 / 0.212 | 0.031 / 0.046 / 0.114 | 0.085 / 0.103 / 0.213 |

Distances are Euclidean OKLab. New is closer than main on 15/17 test mixtures; main is closer on the two cochineal/extender mixtures 16D and 17D. The new worst case is 17D. Main uses measured masstone RGBs and its own saved round-3 fit-set strengths; new uses each paint's own K and S at the measured shares with scalar strength 1.0. See `score.md` for every row.

## Images

- `real-paint-sheet.png`: measured | new | main for all 17 fixed test mixtures, followed by clearly labeled fit and selection-touched groups.
- `swatches-main-vs-new.png`: exactly the earlier sheet's eight ramps and ten probe recipes. Main is above in ramps and left in recipes, new below and right.
- `f0dc5e-main-vs-new.png`: *Last Light on the Tarn* (Claude Opus 5.5, round unknown), [studio f0dc5e](https://stillwet.art/studio/?p=paint-studio-f0dc5e), gallery light, main above and new below, each 640 × 479.

The gallery source is the complete 108-chunk [archived Lua log](https://github.com/aliceisjustplaying/stillwet-paintings-public/blob/main/studios/paint-studio-f0dc5e/lua/archive/committed%20-%20version%2002.lua), SHA-256 `78c30fa4e6a036f28565c718bad1356491970d99f2aaec4c19050f2dd6096307`. The new input changes only its engine header from 8 to 9, SHA-256 `0dd737da63a5f390fabbab8d5e86a339ae71d3abfce56913781ff688f9b893c7`. Main uses its original header. The current-main renderer is pinned to `95dba3ff937a96d2fad23dda578ab6efe4a7b4c8`; its stopped replay resumes from its saved checkpoint. The new identity has no compatible older checkpoint, so its first replay rebuilds all chunks and saves new resumable checkpoints.

The candidate easel was rebuilt from the committed code and generated tables before freezing the render binary: SHA-256 `e6b689a0a09c5ce6e67a75eeb48b00eb81519b450abfdcd08bc653ffcf2d3dcd`. Source and table hashes are in `final-inputs.sha256`; gallery input/output receipts are in `gallery-provenance.json`. This is not the stale-binary workflow from round 1.
The fresh current-main output is pixel-identical to the earlier fixed main reference. New's full replay took 500.62 seconds including checkpoint writes; main's resumed run took 323.87 seconds. The runs overlap and cover different replay work, so these are not a speed comparison.

## Guards, strength and replay checks

The original seven mixer guards are 4 pass / 3 fail, versus round 3's 3 pass / 4 fail, with every threshold unchanged. All values are beside their reproduced round-3 values in `guards-vs-round3.md`. The actual-palette Prussian 1% trace moves lead white by 0.409105 OKLab, above 0.33; the existing trace test passes because it exercises RGB reconstruction, not measured palette latents. Both paths are reported.

All 39 RGB-only scalar strengths remain 0.3–1.6. The measured tubes' scalars are 1.0, with relative strength carried by measured absorption and scattering, so the scalars do not imply equal tinting power. At 1% in lead white, the 39 RGB-only tubes have median distance 0.069257 and range 0.025715–0.165670; the 13 measured tubes other than the two whites have median 0.136172 and range 0.055162–0.409105. Measured emerald is weaker in this probe than many RGB-only tubes. Per-tube results and the cream recipe's changed weights are in `strength-comparison.md` and `.csv`.

The final old-log run replays all 12 cases, with 10 image mismatches and two passes against unchanged goldens. Smoothness counts are 300 endpoint / 193 interior anomaly ramps, versus round 3's reproduced 186 / 172 and main's 0 / 0. Checkpoint/plumbing tests pass. The five paint-library failures and failing CI groups are described in `verification.md`; no golden, capture or threshold was updated.

## Evidence files

`manifest.json` gives the SHA-256 of every file in this directory except itself.

- `README.md`, `score.md`, `real-paint-sheet.png`, `swatches-main-vs-new.png`, `f0dc5e-main-vs-new.png`.
- `guards-round3.md`, `guards-round4.md`, `guards-vs-round3.md`, `smoothness.txt`.
- `strength-comparison.md`, `strength-comparison.csv`, `mixes-new.csv`, `recipes-new.tsv`.
- `verification.md`, `paint-tests.log`, `ck7-tests.log`, `replay-tests.log`, `reconstruction-tests.log`, `old-logs.log`.
- `provenance.json`, `gallery-provenance.json`, `final-inputs.sha256`, `tables-after.sha256`, `main-strengths.json`, `main-reference-pin.txt`, `round3-control.json`, `manifest.json`.
