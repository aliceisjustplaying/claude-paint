# Optics and paint: three reproduced defects

1. **High — `Paint::aimed` returns substantially wrong paint for reachable targets.**
   - **Location:** `crates/paint/src/wet.rs:81–103`, especially the 24-iteration limit at line 84.
   - **Wrong:** The solver alternates between scattering derived from the previous masstone and channel inversion at that fixed scattering. It returns after 24 iterations without verifying the resulting appearance. This contradicts the reachable-target behavior in `notes/color.md` and affects fixed-paint aiming in `handling.rs:693` and `stipple.rs:254`.
   - **Evidence:** Generate a target with `Paint::new([0.8;3], 0.92, 0.5).over([0.0;3], 0.1)`, then call `Paint::aimed` with the same substrate, thickness and hiding. The target is necessarily reachable, but the solver produces `[0.116949104;3]` instead of `[0.33077282;3]`. Its returned masstone is `[0.34768784;3]`, not the known solution `[0.8;3]`. `repros.rs::aimed_paint_must_reach_a_target_generated_by_the_same_model` fails. This is not limited to 0.1 coats: at hiding 0.5 and 0.6 coats over black, the target is 0.35811543 and the result is 0.32539618 (`aim_probe.log`).
   - **Fix:** Solve the coupled scattering/masstone problem with a bracketed or safeguarded numerical method and check the final appearance residual. Add round-trip tests generated from known valid paints across lightening targets and thin films; do not silently treat lack of convergence as an unreachable target.

2. **High — Converting a mixture to brush paint destroys high scattering.**
   - **Location:** `crates/paint/src/palette.rs:366–372`, `crates/paint/src/wet.rs:109–110` and `crates/paint/src/pigment.rs:41–42`.
   - **Wrong:** `Mixture::paint` discards scattering S and stores only its one-coat hiding ratio. `Paint::scatter` inverts that ratio, but `scatter_for` caps hiding at 0.9995. Highly scattering mixtures round to hiding 1, so the conversion cannot preserve their S. Palette aiming evaluates the original S (`palette.rs:261`), while brush loading uses the lossy reconstruction (`bristle.rs:269`): the paint applied is not the paint scored.
   - **Evidence:** A valid custom two-tube palette has white `[0.99;3]` and black `[0.01;3]`, both hiding 0.99, stiffness 0.5 and strength 1. Mix target `[0.1;3]` with no medium. The mixture has S=20.254013 and hiding=1; `m.paint(0.0).scatter()` is only 1.0127121. At 0.1 coats over white, the original mixture yields `[0.09827301,0.100028776,0.11042471]`, but the converted paint yields `[0.44297823,0.45052582,0.492608]`. `repros.rs::converting_a_mixture_to_paint_must_preserve_scattering` fails. This repro uses custom tubes, not the stock Friedrich palette.
   - **Fix:** Preserve S directly in the paint passed to the brush and used by `Paint::over`; derive hiding only for reporting. Raising the hiding cap alone cannot recover information already rounded away. Test mixture → paint → brush scattering preservation, including nearly opaque mixtures.

3. **Medium — Stippling silently skips nonempty small regions.**
   - **Location:** `crates/paint/src/stipple.rs:304–319`.
   - **Wrong:** Maximum coverage is estimated only on a grid with spacing `3 × tool.width` and half-grid offsets. All probes can miss a valid mask, leaving `dmax=0` and returning before any touches are planned. Increasing coverage cannot help because every sampled mask value is zero.
   - **Evidence:** On a 1000×1000 canvas, mask a radius-10 disk centered at (500,500), use `Tool::stippler(20.0)`, constant coverage 10, `.aim(false)` and seed 123. `wet_total()` is exactly zero. The bounding box starts at (490,490); its first probe is outside the disk and the next coordinates are 520, beyond it. The same mask/seed/coverage with tool widths 10 and 2 deposits 945.4666 and 761.9851 units²·coats respectively (`probe.log`). `repros.rs::stipple_must_not_discard_a_nonempty_disk_at_high_coverage` fails.
   - **Fix:** Estimate maximum coverage from actual nonzero mask pixels, or use adaptive sampling with a guaranteed nonempty-region fallback. Test compact disks, thin regions and separated mask islands rather than only broad rectangular bands.

## Reproduction

Scratch sources and full outputs: `~/tmp/review-optics-acdb2c8c/` (`repros.rs`, `repros.log`, `probe.log`, `aim_probe.log`). From `~/src/a/claude-paint-review`:

```sh
export TMPDIR=~/tmp/review-optics-acdb2c8c
export TMP="$TMPDIR" TEMP="$TMPDIR" CARGO_TARGET_DIR=target-optics
timeout 90 cargo build -p paint
timeout 30 rustc --edition=2024 --test "$TMPDIR/repros.rs" \
  --extern paint=target-optics/debug/deps/libpaint-a8ab5859f2abbd0f.rlib \
  -L dependency=target-optics/debug/deps -o "$TMPDIR/repros"
timeout 60 "$TMPDIR/repros" --nocapture --test-threads=1
```

Observed: **0 passed, 3 failed**, exit 101, in 0.04 seconds (`repros.log`). The library filename is the artifact from this checkout/build configuration.

Baseline command: `timeout 180 cargo test -p paint --lib -- --test-threads=2` with the same environment. Observed: **50 passed, 0 failed, 2 ignored** (`baseline.log`). No tracked files were changed (`git diff --exit-code`, exit 0).
