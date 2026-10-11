# Brush simulation iteration 4

This default-off experiment is stacked on `capy/brush-sim-3` at `ee84159c`. Build with `--features brush-sim-4`; new experimental sessions select `EASEL_BRUSH_SIM=lean4`, `body4` or `wild4` (engines 15, 16 and 17). Unset/off stays engine 5. Ordinary builds reject the new engines. Earlier experimental engines retain their recorded equations.

Loaded roots converge toward 3–5 irregular moving groups. Paint bridges a loaded hair's contact more broadly, while depletion reduces that bridge and retains broken contact. Contact width and release still depend on the finite hair reservoir, pressure, travel and the seeded group field.

Within actual positive contact weights, neighboring wet pixels conservatively exchange paint from the thicker film into the thinner film. The transfer carries latent pigment, material properties, cure and proportional solvent. It does not reapply absorption or evaporation. Mobility vanishes at gel cure, respects zero mask/fence contact and is stronger for paint from an earlier stroke than the current stroke. No image filter, display contrast change, palette change, state schema change or rag change is included. This is a local quasistatic proxy, not a solved capillary fluid model. It has a native-pixel discretization and needs broader scale/resolution validation before adoption.

The initial broad-contact WIP exposed an inherited fixed per-stroke ceiling: several grouped hairs competed for the same pixel's allowance, rejecting additional paint into their finite reservoirs. Iteration 4 now scales that unthinned ceiling with a hair's available load above its nominal full load. Depleted hairs retain the old bound. Thinned paint retains its existing solvent-dependent ceiling, and engines 1–14 keep their old equations. The loaded-volume contract checks that eight finite dips can exceed baseline's one-dip deposition while preserving the input-plus-residual paint ledger.

| Preset | Engine | Release | Entropy | Leveling |
|---|---:|---:|---:|---:|
| lean4 | 15 | 1.05 | 3.0 | 0.65 |
| body4 | 16 | 1.15 | 3.6 | 1.00 |
| wild4 | 17 | 1.05 | 4.8 | 1.25 |

## Quantitative morning comparison

```
CARGO_TARGET_DIR="$HOME/.capy/work/brush-sim-4/target" cargo build --profile test -p easel --features brush-sim-4
python3 notes/brush-sim-4/morning.py "$HOME/.capy/work/brush-sim-4/target/debug/easel" "$HOME/.capy/work/brush-sim-4/morning"
CARGO_TARGET_DIR="$HOME/.capy/work/brush-sim-4/target" cargo build --profile test -p paint --features brush-sim-4 --example brush_sim_4_state
```

The new quantitative set uses the same 18 strokes, seed 1906, 1200 × 800 native pixels and original ground. Calibration changes one common multiplier for all brush loads, using finite dips of at most one. It matches total final canvas `material_total` minus the primed initial canvas value to baseline's equal-input deposition within 0.03%. `--target lowest-equal-input` is available as a separate lower-volume control. The initial fixed-ceiling WIP could not reach the original baseline even at eightfold loading; its saturation motivated the finite-load capacity fix. `--resume` reuses measured candidates from an interrupted calibration with the exact same binary and source. `--controls <earlier-output>` reuses saved engine-5 and engine-13 measurements and selected final states across an iteration-4-only model revision, recording the earlier binary and commit explicitly. It never repaints those controls. This is a paint-volume proxy in coats × square canvas units, not density-weighted physical mass. It does not match individual strokes or local pixels. The receipt exposes every factor and measured deposition, including the equal-input budget differences.

Every distinct calibration candidate is run once. The selected candidate's already exported Gallery 115/55/1, Color and final Canvas are retained without a second replay. Checkpoints stay outside Drive. The state analysis example reads those checkpoints without painting again. Its dark-wall ROI is native `(160,180)–(680,460)`; the transverse gradient is a one-pixel forward height difference, including linen/ground plus wet paint and solvent. Profile averages are analysis only. Crops and sheets use untouched native PNG pixels.

The old `renders/morning/brush/` study remains an equal-requested-input reference and is not overwritten. New files use the `iteration-4-` prefix.

## Strict default-off proof

Build feature-off and feature-on binaries using the same optimized test profile as the retained main proof, then run:

```
python3 notes/brush-sim-4/prove_off.py /path/to/brush-sim-3/default-off-proof.json /path/to/proof /path/to/feature-off-easel /path/to/feature-on-easel
```

The proof verifies all 13 input-source hashes and both builds' PNG, stdout and normalized per-chunk state hashes directly against the retained main receipts. Only timing fields are excluded. Do not substitute release-profile float-state hashes or relax a failed comparison. Existing dry-rag, historical rag and thinner checks remain unchanged.

## Wind-down on October 11

The Captain stopped the round on the Codex reset. Runtime code is `9f9428e`; the original-volume painting calibration and full gate are incomplete. The closest completed lean4 painting uses factor 1.546875 and deposits 7,253,594.240 versus baseline's 7,249,498.889, an error of +0.056492%, outside the planned 0.03% tolerance. Body4 and wild4 only have equal-input previews and must not be called volume-matched. Lean4 at factor 2 reaches 9,367,701.332, so the earlier loaded-volume ceiling defect is fixed in the actual study as well as the targeted finite-ledger contract.

All 32 targeted contracts, the experimental all-targets check and both Playwright suites passed. The final default-off proof has all 26 comparisons byte-exact. The full gate was stopped after eight completed steps, 376 tests and 730 seconds; its completed Rust batch has the same three retained-main rag/plough/engine-2 failures, and its historical float-state comparison has the same existing delta. No threshold or golden was changed. Keep this off.

Completed native Gallery/Color paintings, dark-wall crops and an original-volume near-match sheet are retained in the Captain's Drive under `renders/morning/brush/iteration-4-winddown-*`, with `reports/brush-sim-4.md` documenting the limitations. Sheets are assembled from existing untouched PNGs; no painting was replayed at wind-down. Private targets, binaries and checkpoints were removed. A future authorized round needs a fresh quantitative final-state set, completion of all three preset calibrations and the interrupted gate steps; the removed scratch checkpoints are not a resume source.
