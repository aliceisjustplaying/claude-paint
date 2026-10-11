# Fresh paint-body relaxation study

Developer research only. No painter API, engine default, historical rendering formula or checkpoint schema is changed. Select the experiment by calling `Canvas::relax_fresh_body(seconds)` explicitly. Ordinary painting never calls it.

## Question

Can early movement of a fresh film round the narrow deposition ridges without adding paint, changing the lamp or flattening every material equally? Existing paint levels in `drying::bake` through `surface::settle_for`; an open film's visible relief comes directly from its deposited volume. The study isolates an earlier conservative transport step. It does not diagnose every finished painting as wet: the earlier 125-chunk Cézanne snapshot analyzed by the texture report had already aged for over 165 simulated days.

## Approximation

The free surface is dry height plus wet-film thickness. A five-point discrete Laplacian gives capillary pressure `p = -0.035 * laplacian(height)` in pascals. Face flux uses `h^3 / (3 * viscosity)` mobility and the pressure gradient. A yield factor suppresses flow below the donor's yield stress. The time step is bounded by the explicit fourth-order stability scale and an inventory limiter prevents a donor from losing over 20% in one substep. Face transfers debit the donor and credit the receiver, carrying the original donor's seven Mixbox latent components and five material properties. They do not move the dry support or sample the rendered image.

Stiffness uses the existing engine's approximate viscosity/yield range. Immediately after shear, viscosity is multiplied by 0.05 and yield stress by 0.02; both recover exponentially with an 8-second time constant. These three numbers are chosen experimental controls, not measured values for the pigment catalog. The 2-second comparison is an illustrative dose, not a calibrated brush-to-photo timeline. The lubrication approximation is least reliable at steep, thick stroke edges. Contact-angle/wetting physics, elastic skin and large-deformation flow are absent.

The implementation is intentionally limited to a whole canvas with fresh, solvent-free paint and full pixel coverage, without a drawing or raw-cloth soaking state. It rejects the other cases before mutation. The integration runs on a private wet-state copy and a budget failure leaves the caller unchanged. It has no persistent structure channel: every explicit call restarts shear recovery, so split calls are not equivalent to one long call. Production integration must persist a recoverable-structure field and solver phase in material checkpoints and couple these to the real clock.

## Reproduce

Run from this worktree with the shared heavy-job lock and two workers while the stream is live:

```sh
/bin/df -g /
# Continue only with at least 15 GiB available, including after acquiring the lock.
export CARGO_TARGET_DIR="$PWD/target"
RAYON_NUM_THREADS=2 CARGO_BUILD_JOBS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 \
  nice -n 15 scripts/lockrun --timeout 600 -- cargo test --release -p paint --lib body_study --no-run
RAYON_NUM_THREADS=2 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 \
  nice -n 15 scripts/lockrun --timeout 60 -- cargo test --release -p paint --lib body_study
RAYON_NUM_THREADS=2 CARGO_BUILD_JOBS=2 CARGO_PROFILE_RELEASE_DEBUG=0 \
  nice -n 15 scripts/lockrun --timeout 600 -- cargo build --release -p paint --example body_relaxation
RAYON_NUM_THREADS=2 nice -n 15 scripts/lockrun --timeout 60 -- \
  "$CARGO_TARGET_DIR/release/examples/body_relaxation" "$HOME/.capy/work/astra-surface/results"
```

The example paints each scene once, clones that exact state and runs the candidate only on the clone. Both branches retain identical initial pigment, support and strokes. At 160 and 320 pixels, four scenes cover a loaded medium-body brush, a stiffer load, a nearly spent brush and a two-color crossing. Each physical support is 100 × 50 mm, with 12 µm linen crowns. The existing renderer supplies Color, Gallery (115/55/1) and Raking (135/25/1). Forced-dry views additionally apply the unchanged `dry()` to each branch; this checks retained geometry, not a new calibrated aging model.

Height files are little-endian f32 micrometers: dry height plus wet volume × 25 µm. This is the solver's free surface, not the renderer's separate substrate-bridging projection. They are numerical evidence, not photographs. CSV mass checks compare the same deposited mass; no different brush-release gain is allowed to confound the comparison.

## Acceptance boundary

This is a default-off research candidate, not a completed production solver. Focused tests cover paint inventory, optical/material moments, nonnegative finite volume, immutable support, exact repeat, checkpoint continuation of the same explicit sequence, heterogeneous-mobility energy dissipation and no-op/rejection behavior. This work does not certify physical resolution convergence, realistic contact-line behavior, historical replays or full repository compatibility. The two grids independently repaint their initial scenes. The coarse grid undersamples the linen's approximately 0.7–0.8 mm thread pitches, and the finer one has only 2.3–2.7 samples per thread; comparison between them combines deposition, support sampling and solver effects. Keep it isolated until the research report's visual judgment and verification receipts are reviewed.

## Recorded result, October 11, 2026

Five focused tests passed on Ubuntu 24.04.4 x86_64 with Rust 1.97.1, two build/Rayon workers and a private target. Exact modified Rust-source hashes match the reviewed Mac worktree. No macOS painting replay identity or complete repository gate is claimed. The separate reviewer identified a face-mobility timestep defect and missing finite checks; both were corrected before these runs, alongside crop-output rejection. No existing test or golden changed.

The surface example took 1.55 seconds. Maximum matched baseline/candidate paint-inventory error was 5.44e-8 relative, including forced-dry stages. At 320 pixels, fresh loaded peak wet height fell from 309.15 to 253.12 micrometers; crossing paint fell from 345.87 to 276.83. Stiff paint changed only 1.83% and spent paint stayed unchanged.

Visual inspection of the unchanged Color/Gallery/Raking views finds only modest rounding: the long combed ridges remain. Whole-swatch mean gallery differences are only 0.0366 and 0.0687 of an 8-bit channel level for loaded/crossing paint. This demonstrates conservative, material-dependent transport, not a convincing standalone cure for a digital appearance. Do not promote the experiment alone.

`dark_mixtures` is a separate analytical example for the recorded dark recipes. It prints masstone, finite-film substrate transmission and the existing dry-haze response with and without white. It does not replay or alter a painting. Research outputs, source receipts and the broader surface/rag architecture are published in the requested captain-scope paint-surface report and render folder.
