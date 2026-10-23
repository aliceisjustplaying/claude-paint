# Conservative adaptive chemistry depth (STIL-322)

## Method and acceptance

The old estimator advanced a globally doubled reference grid but refined only cells whose local coarse/fine differences failed. Diffusion couples those cells to their coarse neighbors. Repeatedly splitting the affected cells did not remove the error introduced by the untouched neighbors. The primary diagnostic reached 8,197 cells in one S3LWLO column near 130 hours before its diagnostic stop. Mesh grading alone did not resolve that mismatch.

`oil/depth.rs` now advances the proposed selectively refined mesh and compares its original-cell means with the same globally refined reference. Trial/reference agreement must be within one quarter of the state tolerance, leaving headroom below the outer acceptance threshold. Using the full tolerance for this support check still allowed repeated refinement at 3e-7 in Golden254; the stricter support check resolves that case. A failing region expands to adjacent cells. If errors remain inside an already marked region, its refinement support expands rather than repeatedly subdividing just the receiving cells. A disconnected zero-thickness barrier falls back to the full positive-thickness reference mesh. There is no history limit, cell-count limit or prescribed physical cell width.

Splitting reconstructs oxygen, oxidation, network and inhibitor with a monotonicity-limited linear slope from cell centers on the nonuniform mesh. The stencil stays inside one packet and recipe. Interior slopes use the monotonized-central limiter; a material-boundary cell uses its available one-sided slope. The child means are the parent mean plus or minus one quarter of the parent width times the limited slope. The slope also respects [0, 1] and, where initially valid, network <= oxidation.

Pigment, binder and solvent split componentwise into halves and residuals. Thus oxygen capacity (binder times binder solubility plus solvent times solvent solubility) and binder weight split with the same fractions. The reconstructed state means preserve their respective inventories to single-precision rounding. Merging retains the existing capacity-weighted oxygen and binder-weighted chemical means. Cell and packet layouts remain 32 and 40 bytes. Regridding does not alter packet tension, moment, damage, provenance or coverage.

Local relative rigidity errors near the zero-stiffness gel boundary are used to locate refinement only when whole-column rigidity actually fails. Otherwise a vanishingly small local stiffness could demand indefinitely fine cells despite agreement in the column's extension, coupling and bending. Acceptance still requires the existing whole-column 1% rigidity test. A supplied `CoupledAccuracy` still requires no threshold sign change and at most 1% maximum relative error. Its explicit optional interface is unchanged.

Coarsening is restricted to equal-width, same-recipe cells within an unstressed packet and maintains 2:1 grading within a material run. A candidate merge must reconstruct the original child states within one quarter of the state tolerance, including composition. This permits a resolved linear gradient without treating it as a flat state. The entire coarsened column must also pass the rigidity test and any supplied coupled-threshold callback. Failed coarsening is discarded. This avoids accumulating individually acceptable rigidity changes or silently crossing a threshold after acceptance.

The state tolerance is tightened from 1e-3 to 1e-6. The front-variation tolerance remains 0.05, rigidity remains 0.01 and calendar acceptance remains one 600-second sample. No tolerance is loosened. At 1e-5 the 254 µm Golden fixture satisfies its broad source interval but is still 80 minutes later than both fixed-grid predictions; that is not accepted as fixed-grid agreement.

Solvent-free columns borrow the tile's kinetics table directly. Solvent-bearing columns adjust only the recipes actually present in the column. Recipe IDs are remapped only during chemistry calls and restored before rigidity or coupled callbacks. This retains the original evaporation equation and chemistry step limits without copying every unused tile recipe for each column.

## Primary measurements

Measured on primary `machine_01M5M4F0WEBPVQ6BSF48E8QTYN`, using standalone optimized `rustc`, not Cargo. The linked primary debug rlib SHA-256 is `87d5aedbd0eddf83c83ec160b9e0e2a84d3f6787fd489613ceee332b2d959711`. The measured depth source SHA-256 is `468107c464066017ea304698c7ced87a0c97b02b8deced34bfbedf1276b28e09`. The primary live checkout was not edited.

All 14 endpoints agree with both 96- and 192-cell references within 600 seconds and satisfy the source targets within that same sampling tolerance. Both reference grids give the same sampled endpoints. Times below are hours; a triple is set-to-touch / dry-to-touch / dry-hard.

| Fixture | Fixed 96 and 192 | Adaptive | Peak / final cells | Fixed 96 / 192 / adaptive runtime, seconds |
| --- | --- | --- | --- | --- |
| S3LWLO 450 µm | 63 / 90 / 134 | 63 / 90 / 134 | 386 / 386 | 0.430 / 0.859 / 3.770 |
| S3UBLO 450 µm | 213 / 242 / 284 | 213 / 242 / 284 | 256 / 256 | 0.482 / 0.897 / 1.462 |
| S3LWSO 450 µm | 112 / 157 / 207 | 112.166667 / 157 / 207 | 256 / 256 | 0.332 / 0.663 / 1.593 |
| S3UBSO 450 µm | 234 / 261 / 305 | 234 / 261 / 305 | 259 / 259 | 0.484 / 0.966 / 1.561 |
| Golden 76 µm | 47.833333 | 47.833333 | 16 / 16 | 0.903 / 1.802 / 0.297 |
| Golden 254 µm | 99 | 99.166667 | 37 / 37 | 2.005 / 4.009 / 1.463 |

The runtime horizon is the first dry-hard observation, matching the existing predictor, even for the Golden touch endpoint. The complete fixed-grid comparisons plus adaptive runs take 23.97 seconds and peak at 3,044 KiB RSS. These adaptive runs are not faster than a fixed 192-cell solve for every fixture: the S3LWLO case spends extra work resolving the gel front and checking selective trials. They do eliminate the runaway refinement without a numerical cap.

The grids are not prescribed uniform 2.5 µm grids. At the final observation S3LWLO cells span 0.220–1.758 µm and Golden254 spans 3.969–7.938 µm. Some fixtures do select uniform grids when their errors demand it. The primary mesh receipts record intermediate growth; the 1e-5 sweep also shows coarsening, including S3UBLO's peak of 81 cells and final 75 cells.

## Endpoint refinement convergence

Only the state tolerance varies in this sweep. The 0.05 front and 0.01 rigidity tolerances stay fixed. Differences are measured against both fixed grids, which agree at the sampling resolution.

| State tolerance | Largest endpoint difference | 450 µm peak cells, LWLO / UBLO / LWSO / UBSO |
| --- | --- | --- |
| 1e-5 | 4,800 seconds, Golden254 | 171 / 81 / 92 / 102 |
| 1e-6, selected | 600 seconds | 386 / 256 / 256 / 259 |
| 3e-7 | 600 seconds, Golden254 only | 643 / 512 / 256 / 512 |

At 3e-7 the LWSO endpoint also matches exactly; Golden254 stays one sample later. This demonstrates convergence of the sampled endpoints, not a universal pointwise state-error bound or a validated full nonlinear wrinkle trajectory. Refinement at the gel boundary can be strongly nonuniform and the number of cells need not change monotonically at every tolerance.

## Tests and reproduction

Four inline regressions cover conservative linear prolongation and bounds, the nonlocal S3LWLO calendar plus thick Golden calendar, real coupled growth-threshold acceptance and sparse solvent-adjustment equivalence on the accepted mesh. The focused calendars assert the unchanged 600-second agreement with both fixed grids. The separate primary calendar executable asserts all 14 endpoints. The four tests pass in 12.68 seconds.

The threshold test supplies a real callback that computes extension/coupling/bending and evaluates `mechanics::coupled_linear_growth_oracle`. Its initial coarse and reconstructed grids have opposite growth signs at a fixed compressive load. After refinement the callback reports the same sign and <=1% relative error. Force and moment are unchanged. The source-calendar runs pass `None`: those runs verify chemistry and rigidity, not a missing coupled callback. No coupled threshold is claimed for them.

All build inputs and raw receipts are under primary scratch `/home/user/.capy/work/STIL-322-depth-debug/`:

- `depth-original.rs`, `depth.rs` and `calendar.rs` are the original source, patched source and 14-endpoint runner.
- `final-manifest.log` records compiler, linked rlib and source hashes.
- `final-tests.log` and `final-test-build.log` record the focused regressions.
- `final-calendar.log`, `final-loose-calendar.log` and `final-finer-calendar.log` contain raw endpoints, intermediate cell counts, final widths, runtimes and `/usr/bin/time -v` resources.
- `depth-final-loose.rs`, `depth-final-finer.rs` and their calendar wrappers retain the convergence inputs.

The standalone calendar command, run inside that scratch directory, is:

```sh
LIB=$(ls -t /workspace/claude-paint/target/debug/deps/libpaint-*.rlib | head -1)
rustc --edition=2024 -O calendar.rs --extern paint="$LIB" \
  -L dependency=/workspace/claude-paint/target/debug/deps -o calendar
/usr/bin/time -v ./calendar --assert
```

The standalone test wrapper reexports the primary library's chemistry, constitutive and store modules and includes the primary mechanics source for the real growth oracle. It compiles `depth.rs` with `rustc --test`; no primary source is rewritten and no fresh-machine build is needed.
