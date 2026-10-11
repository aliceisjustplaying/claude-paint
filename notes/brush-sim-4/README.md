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

The new quantitative set uses the same 18 strokes, seed 1906, 1200 × 800 native pixels and original ground. Calibration changes one common multiplier for all brush loads, using finite dips of at most one. It matches total final canvas `material_total` minus the primed initial canvas value within 0.03%. The common target is the lowest equal-input deposition among the five presets: lean4 could not reach the original baseline's deposition at eightfold loading, so baseline and body3 are calibrated down too. `--target baseline` retains the original-target diagnostic. `--resume` reuses measured candidates from an interrupted calibration with the exact same binary and source. This is a paint-volume proxy in coats × square canvas units, not density-weighted physical mass. It does not match individual strokes or local pixels. The receipt exposes every factor and measured deposition, including the equal-input budget differences.

Every distinct calibration candidate is run once. The selected candidate's already exported Gallery 115/55/1, Color and final Canvas are retained without a second replay. Checkpoints stay outside Drive. The state analysis example reads those checkpoints without painting again. Its dark-wall ROI is native `(160,180)–(680,460)`; the transverse gradient is a one-pixel forward height difference, including linen/ground plus wet paint and solvent. Profile averages are analysis only. Crops and sheets use untouched native PNG pixels.

The old `renders/morning/brush/` study remains an equal-requested-input reference and is not overwritten. New files use the `iteration-4-` prefix.

## Strict default-off proof

Build feature-off and feature-on binaries using the same optimized test profile as the retained main proof, then run:

```
python3 notes/brush-sim-4/prove_off.py /path/to/brush-sim-3/default-off-proof.json /path/to/proof /path/to/feature-off-easel /path/to/feature-on-easel
```

The proof verifies all 13 input-source hashes and both builds' PNG, stdout and normalized per-chunk state hashes directly against the retained main receipts. Only timing fields are excluded. Do not substitute release-profile float-state hashes or relax a failed comparison. Existing dry-rag, historical rag and thinner checks remain unchanged.
