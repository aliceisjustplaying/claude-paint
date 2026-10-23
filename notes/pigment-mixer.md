# Independent pigment mixing (engine 9)

All engine numbers use the independent mixer and new easel paintings record
engine 9. Legacy tables remain frozen for older replays. The engine-9 fit
uses the unchanged Revigo measurements and the physical prior described below.

## Engine 9: physical volumes and dark mixtures

Engine 9 uses `calibrated.bin`: additive K/S per 100 µm of paint volume,
with measured tube weights exactly one. Optical strength is carried in
the coefficients rather than normalized fitted recipe weights. The
original `measured.bin`, its metadata and inverse stay unchanged for
engines 8 and earlier. Engine 9 rebuilds RGB residuals against its own
decoder; the inverse is not regenerated.

Source fractions are masses of finished paints. Fitting and scoring both
convert them using `v_i = (m_i/rho_i)/sum(m_j/rho_j)`, then add physical
coefficients as `K=sum(v_i K_i)` and `S=sum(v_i S_i)`. The runtime accepts
volumes, so it does not apply another density multiplier. Every wet-paint
density is an **estimate**, not a Revigo density measurement. Table 5
supplies eight measured average oil mass fractions; other oil fractions
and lake/wax substrate densities are labeled estimates. Component-density
sources and the additive-volume model are in
[`pigment-density9.md`](pigment-density9.md) and
`scripts/pigment_densities.json`. Air, contraction and drying shrinkage are
not measured. This fixes unit consistency within a documented model, not
the missing physical density evidence.

The bounded fitter takes ten coordinate passes, at most 120 evaluations
per paint, followed by one sparse joint solve, at most 100 evaluations.
It uses color residuals over fixed 07A white and raw reflectance residuals
over both measured backings. A fixed log-K/S prior, initialized from the
legacy coefficients scaled by mass weight and relative density, never
resets. A wavelength second-difference penalty prevents isolated spectral
spikes. The three audited dark cases have twice the color-residual weight.
No revised12 spectrum enters the objective.

All 27 black-containing source mixtures are fitted, including the 18
previously omitted non-primary dark mixtures and trace-black/lead-white
08B. There is no independent dark holdout. The 65 fitted rows and revised12
are disjoint. Four historically selection-touched rows and unused pure
Prussian 13A remain separate. The round-6 declaration records adaptive
model selection after the audit; these are not newly blind results.

The compiled decoder gives these OKLab distances and agrees with the
packed-table oracle across all 82 rows within 1.69e-7 OKLab. The dark-case
regression also checks actual 100 µm `Paint::over` films, and the density
regression checks the equivalent mass/volume descriptions of 13G.

| source row | status | before | calibrated |
|---|---|---:|---:|
| 11E ultramarine/black | fitted | 0.100332 | 0.000973 |
| 10D emerald/black | fitted | 0.076014 | 0.019594 |
| 04E carmine/black | fitted | 0.063038 | 0.003621 |
| revised12 mean | held out of fitting | 0.023212 | 0.024218 |
| revised12 worst | held out of fitting | 0.041499 | 0.049032 (04C) |

The worst-case tradeoff stays visible: 04C gets worse rather than being
moved into fitting. Fixed-prior and smoothness alternatives were checked
without fitting revised12. The smoother physical-volume model improves
the audited dark cases but does not reproduce every source ladder exactly.
No threshold is relaxed.

```sh
uv run --script scripts/calibrate_pigments9.py
cargo build --release --locked -p paint --example mixpairs
uv run --script scripts/pigment_strength_fit.py target/release/examples/mixpairs --measured
uv run --script scripts/pigment_mixer_score.py --engine9 target/release/examples/mixpairs \
  --strengths scripts/pigment_strengths.json --out score9.md --csv score9.csv --json score9.json
```

The scorer reports all 82 modeled source rows with fitted, heldout,
selection-touched or unused status, plus complete black, old17 and
revised12 groups. `--before` accepts the audit's all-row CSV to include
before colors and errors. [`pigment-calibration9.csv`](pigment-calibration9.csv)
records all 82 rows, both fit statuses and before/after colors and errors.
`--python-only` labels a coefficient-oracle
check, not runtime validation. Alternate-oil 07B and 14F have no modeled
identity and are excluded. Raw measurements stay unchanged. The historical
score remains available without `--engine9` using its historical strength
file. The rest of this note records that historical round-5 model.

## Historical round 5: finite measured films and constant paint weights

The fifteen measured catalog tubes retain their own wavelength-dependent
K and S. The other 39 still enter through their catalog RGB and five-primary
inverse reconstruction, with their scalar strengths unchanged. Measured
tubes have no RGB residual. The three vermilion grades share the measured
Vermillion paint.

The latent remains 21 floats: eighteen pigment concentrations and three
linear RGB residuals. Piles, brush paint, knives, medium and wet exchange
retain that latent. Baking still renders an appearance to RGB; this is not
a new spectral layering renderer.

For each wavelength K = ΣcᵢKᵢ and S = ΣcᵢSᵢ. Round 5 evaluates the source
paper's finite, non-hiding KM film at its measured 100 µm reference coat,
over the fixed measured white field `07A_w`, instead of comparing an
infinite-thickness prediction with a finite measured film. It applies the
same Saunderson constants (k₁ 0.04, k₂ 0.49) and fixed specular-excluded
display conversion used in earlier scores. The measured color targets do
not change. The white field is one global reference, not a sample-specific
lookup or a recipe-dependent correction.

Each measured paint also has one calibrated constant linear recipe weight.
The palette uses its existing rule: normalized weights cᵢ proportional to
the supplied share times that constant. There is no exponent, concentration
law, pair-dependent parameter or target-color lookup. The 39 RGB-only tube
strengths are not fitted or changed. The measured constants and K/S arrays
live together in `measured.bin`; the reference substrate is in
`reference.bin`. The inverse is regenerated against this same decoder.
RGB residuals remain exact against the runtime decoder.

## Source and precommitted splits

The unchanged CC0 measurements in `scripts/pigment_data/revigo` come from
https://github.com/fligt/revigo-spectra, documented in Geldof, Proaño Gaibor,
Ligterink, Hendriks and Kirchner (2018),
https://doi.org/10.1186/s40494-018-0181-6. Pure and mixed 100 µm films were
measured over black and white at 400–700 nm. The existing MIT
`spectral.rs` D65/CIE conversion uses endpoint extension to 380–750 nm.

`scripts/pigment_fit_split.json` is the round-5 declaration. Commit
`7c541ca` moved 16D and 17D to fit before either extender mixture was
fitted, leaving 16F and 17F as transfer tests to cobalt. Commit `758beb1`
moved 13F to fit before fitting it and reported pure 13A apart. Commit
`546deb3` then added 16F, 17F and previously selection-touched 16E before
fitting them: cochineal-only extender calibration did not transfer
consistently to cobalt. No independent extender test remains. The nearly
pure Prussian 13F and 50.7% tint 13C reflect less red light than nominally
pure 13A, despite all being opaque; the original pure-film anchor is not
consistent with that mixture ladder. The new Prussian fit uses 13B and
13F, leaving 13C and 13G as tests.

The old test still has seventeen rows and is always reported, explicitly
including the five samples now fitted. The revised test has twelve.
The six added fits include 16E, leaving four selection-touched rows
reported apart. The original eleven fit samples remain fitted. 08E
already had round-2 selection exposure. These are
partitions used to compare numerical source iterations, not a newly blind
holdout or independent validation after model selection.

Coefficient calibration starts from the original five-primary fits. Other
paints use their pure film and original fit tint, except Prussian's
declared replacement anchor. Extenders use their pure film and all their
declared chromatic mixtures. Constant weights minimize raw reflectance
error over both black and white, using the same fit samples and complete
primary ladders, with zinc white fixed at weight 1. Three local
coefficient/weight alternations initialize one final joint K/S fit over
all declared fit spectra. That final solve includes the five primary
paints, so their coefficients and RGB inverse change too. The calibrated
weights stay fixed during it. No revised-test row is a coefficient or
constant-weight fitting input.

The joint solve improves the worst old-test rows and the revised-test
worst case, with a mean-error tradeoff against a sequential-only fit.
More local iterations, another weight calibration and a shared global
weight/K/S alternation did not improve both old and revised mean errors;
the shared surface-reflection experiment was also rejected. The iteration
ledger records the tradeoffs rather than claiming a global optimum or
that plain KM can reproduce every measured row exactly.

The generator asserts that the revised test and fitted sample sets are
disjoint. `measured.json` records all samples, both partitions, the
coefficient-fit weights, final weights, reference substrate and residuals.
The primary provenance preserves the original basis split for historical
scoring and separately lists the final joint-fit inputs and residuals.
The original `split_for_strengths` function is retained to reproduce the
round-3 partition. Main's old RGB calibration is reference evidence only.

## Reproduction

```sh
OPENBLAS_NUM_THREADS=1 uv run --script scripts/build_pigment_tables.py
cargo build --release --locked -p paint --example mixpairs
cargo build --release --locked -p easel
uv run --script scripts/pigment_strength_fit.py target/release/examples/mixpairs --measured
uv run --script scripts/pigment_mixer_score.py target/release/examples/mixpairs \
  --strengths scripts/pigment_strengths.json --out score.md --sheet real-paint.png
```

The generator pins NumPy/SciPy and one BLAS thread. It derives K/S from the
raw spectra, calibrates the constant weights and builds the inverse; the
strength script's `--measured` mode writes those generated constants, not
a second fit through an old binary. Both old and revised scores are
reported. The real-paint sheet shows revised test, added fit, original fit,
selection-touched and unused pure-film groups separately.
Scoring normalizes the raw recipe fractions for both engines: 08F sums
to 100.3% in the unchanged source specifications. No measurement is edited.

`notes/pigment-mixer-comparison.csv` contains only the current independent
mixer's 1,045 results over the original 95 ramps. Main's outputs remain
reference evidence, never generation inputs. Round-5 images, iteration
records, scores, guards and replay receipts are in
`renders/STIL-259/` with round-5 filenames. No color judgment or visual acceptance is
claimed and no guard threshold, golden or capture is changed.

## Earlier constant edits

The superseded basis was hand-edited during the implementation review.
Its recorded checks were:

| Revision | What was checked |
|---|---|
| `8190bef` | White-tint brightness and the teal cast in the original tint chart; the white-scattering increase and flatter blue scattering addressed those defects. |
| `68bd70b` | Dark-pigment coverage, blue-tint chroma and the unchanged blend-reach/glaze tests; the earlier decoder could not reach bone black's dark end. |
| `a6e03e2` | Blue-tint chroma and the unchanged reach/glaze checks. The later full gate still exposed the white-over-dark knife-skim failure. |
| `7a03bc4` | Middle-gray chroma, blue tints, knife skim and glaze. The unchanged blend-reach test remained red. |
| `18584b1` | The unchanged knife-skim, blend-reach and glaze tests, followed by the full gate. |

Old-engine charts and numeric comparisons were also present during those
reviews, and an old-engine green output was incorrectly used as a test
target. That target and the sampled comparison columns have been removed.
The earlier sequence is not evidence that the superseded choices were
made without seeing those comparisons. That superseded generator read
only its synthetic basis and the credited CIE conversion matrix; the
current generator uses the CC0 measurements.

## Checkpoints and licensing

`PAINTC19` and `EASLRP04` check the mixer fingerprint before interpreting
latents or tool snapshots. It covers the decoder schema, K/S and constant
weights, reference substrate and inverse bytes. Old identities are refused;
Lua logs remain replayable. State dumps preserve their producing identity.

The implementation and K/S derivation are project MIT; source measurements
are CC0. The method is credited to Sochorová and Jamriška,
*Practical Pigment Mixing for Digital Painting* (2021),
https://doi.org/10.1145/3478513.3480549. The source paper is linked, not
redistributed. The earlier patent-record search is not legal clearance.
Fixture sources are in `notes/pigment-mixer-assets.md`; licenses and
package inventory are in `notes/pigment-mixer-licenses.md` and
`notes/pigment-mixer-dependencies.json`.
