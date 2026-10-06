(•̀ᴗ•́)و Recommended drop-in: [painter/bin/easel](painter/bin/easel), built from fix commit `ff730e4997a2c66d97282c78abe18c317da8d3e3` on `research/wait-solvent`. It is an arm64 painter build with `--no-default-features --features box-every`; help and tubes passed. SHA-256: `051a5cdff3fbbe583a9a7ddee8d5c27d28f1f72d3ab432dc457e1547f5c8bf43`. Left uninstalled. Receipts: [commit](commit.log), [build](committed-build.log), [help](final-painter-help.log), [tubes](final-painter-tubes.log).

The fix tracks solvent pixels and processes flow around liquid-carrying pixels and their neighbors. It skips drying-buffer allocation when no wet paint is ready to join the set paint layer. Engine 5 uses aligned quarter-minute steps when flow is slow (`max_mobility * 0.25 / pixel_mm² <= 0.0025`) and discards solvent below `max(1e-8, 1e-5 * (paint_um + 2))` µm. The existing no-solvent shortcut ages the remaining interval, grouping paint that sets at different times into one update. Earlier trace removal and this grouping are numerical approximations, not physical calibration. Earlier engines retain the fine grid and original cutoff. Receipts: [drying](../../crates/paint/src/drying.rs), [flow](../../crates/paint/src/thinner.rs), [contract](../../notes/thinner/ACCEPTANCE.md).

| Measurement | Wall time | Receipt |
|---|---:|---|
| Unmodified stock, first 49.993 simulated minutes | 378.476 s | [baseline](baseline-wait.log) |
| Original 64-tick grid and cutoff, with exact processing optimizations, full 4,320-minute wait | 1,693.247 s | [reference](exact-wait.log) |
| Release fix, full wait from the original checkpoint, default threading | **373.557 s** | [release](release-wait.log) |
| Fix replay, all 34 chunks, 2,400 px, four Rayon threads | **1,015.01 s** | [replay](fast-replay.log) |
| Release fix, full wait from that fixed replay's checkpoint | **286.970 s** | [replayed checkpoint](replay-release-wait.log) |

Full unmodified-stock wall time was not measured. Extrapolating its 0.1183 s/tick sample over 173,547 reference ticks gives **5.70 hours**, an estimate. The stock-grid reference supplied the after-wait image; it matched stock byte-for-byte on the one-minute painting checkpoint probe and sparse 15-minute probe. Release and instrumented fixes matched byte-for-byte after the full wait. Receipts: [baseline](baseline-wait.log), [reference](exact-wait.log), [checkpoint equivalence](exact-equivalence.json), [sparse equivalence](sparse-equivalence.log), [release equivalence](release-equivalence.json).

| Standard unlit RGB comparison, 2,400 × 1,800 | Changed pixels | Maximum channel change | Mean absolute RGB change |
|---|---:|---:|---:|
| Stock versus fix, 34-chunk replay before wait | 21 | 7 levels | 0.000004012 levels |
| Stock versus fix, wait from the same original checkpoint | 82,694 | 49 levels | 0.01742708 levels |
| Stock replay plus wait versus fixed replay plus wait | 82,700 (1.914%) | 49 levels | 0.01742863 levels |

Receipts and side-by-side PNG crops: [before wait numbers](prefix-comparison.json), [before wait crop](prefix-comparison-crop.png), [checkpoint wait numbers](checkpoint-wait-comparison.json), [checkpoint wait crop](checkpoint-wait-comparison-crop.png), [replay plus wait numbers](replay-wait-comparison.json), [replay plus wait crop](replay-wait-comparison-crop.png). Checkpoint field differences are included in the JSON; RGB agreement does not establish relief or physical accuracy.

**Stock-versus-fix comparisons fail the cross-build policy**: at most one RGB level in at most 432 pixels. Before the wait the maximum exceeds one; afterward both limits fail. Moving `check_painting` to the fix commit aligns its model with the new binary; it does not preserve historical stock images. The live post-check was not run. Receipt: [policy](../../notes/workflow.md), comparison JSON above.

Tests: **31 passed, zero failed, nine ignored** across [thinner](final-thinner-tests.log), [drying](final-drying-tests.log) and [physics](final-physics-tests.log). The engine-3 split-wait contract remains 1e-4. Engine 5 keeps the tested byte-exact minute/quarter-minute split cases with solvent throughout; fractional splits deliberately allow 1e-3 relative difference in paint, solvent and cure, or 0.001 µm absolute difference in paint/solvent. Receipts: [test](../../crates/paint/tests/thinner_physics.rs), [contract](../../notes/thinner/ACCEPTANCE.md).

The fix remains off main. Main independently advanced to `efa2b0b7` through an unrelated Studio commit during this run ([main reference receipt](main-ref.log)). No live-studio operation, process intervention or push was performed in this task. Generated build directories, scratch checkpoints and temporary benchmark executables were removed ([cleanup receipt](cleanup.log)); the requested worktree, drop-in package, source, comparison PNGs, JSON and measurement logs remain.

Not done yet: a completed unmodified-stock full-wait wall-clock measurement.
