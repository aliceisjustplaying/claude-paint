# Physics review — round 3

1. **Medium — Splitting a wait changes when the same paint dries.**
   - **Location:** `crates/paint/src/drying.rs:222-229,283-288,425,503`.
   - `film_thickness` averages only currently wet pixels. Each `bake` clears gelled neighbors from that field, so later waits recompute a different thickness for the remaining paint. A single long wait instead uses the initial thickness throughout. Merely checking back more often changes the physics, including the tack a subsequent brush feels.
   - **Evidence:** A controlled 20×20 film with alternating 0.2-coat fast-drying and 4-coat slow-drying columns reaches **Dry** after `wait(7000)` but **Tacky** after 70 calls to `wait(100)`. The slow pixel's set-film cure is 1 versus 0.7466623. The regression fails in [striped-wait.log](~/tmp/review-physics-600dda80/striped-wait.log); [test source](~/tmp/review-physics-600dda80/paint/src/review_wait.rs).
   - A public-API reproduction with two ordinary filbert strokes also finds **145 different stages** after 3000 minutes. Calling `dry()` afterward gives total clocks of **8846.31 versus 12053.87 minutes**. [Source](~/tmp/review-physics-600dda80/wait_partition.rs), [output](~/tmp/review-physics-600dda80/wait-partition.log).
   - **Suggested fix:** Keep the physical film neighborhood available after its members gel, or evolve drying through consistent gel events within each wait. Add a partition-invariance regression with heterogeneous thicknesses and drying rates. The existing `waiting_it_out_equals_dry` test at `drying.rs:598-615` jumps from 10 minutes to three days rather than exercising repeated partial gelation.

2. **Medium — Cropping changes local crack visibility.**
   - **Location:** `crates/paint/src/crack.rs:1212-1228,1244`; visibility at `1138-1145`.
   - `crack_local` groups pixels into averaging cells starting at the window's origin, not a whole-canvas grid. A non-cell-aligned crop changes the underlying field even away from the crop edge. Boundary clamping additionally omits neighboring source cells.
   - **Evidence:** An extracted-source numerical probe uses identical global pixels in alternating four-pixel light/dark strips, 240 µm film and ground, `vary=1` and 0.25 mm pixels. Moving the buffer origin from x=0 to x=1 changes reach at the same physical point from **3.75 to 4.493053**. The production visibility equation consequently changes a generation-4 crack from absent to **49.3% visible**. This is an isolated field/visibility test, not a full-render comparison. [Probe and failing test output](~/tmp/review-physics-tip-1e437b57/regression.out), [source](~/tmp/review-physics-tip-1e437b57/regression.rs).
   - **Suggested fix:** Anchor averaging cells to whole-canvas coordinates and retain complete source cells plus an interpolation halo. Test spatially varying paint under non-cell-aligned crops.

3. **Medium — Diagonal pointed-hair coverage doubles under a half-pixel translation.**
   - **Location:** `crates/paint/src/bristle.rs:934-942,1162-1163`.
   - `fine_cover` rotates the pixel center into track coordinates and multiplies two unit-width box overlaps. That treats the axis-aligned canvas pixel as a square rotated with the hair. The resulting coverage weights do not conserve area on the pixel lattice.
   - **Evidence:** The exact source functions give total coverage **3.9999974 pixel²** for a radius-0.02 track from `(10,10)` to `(110,110)`, versus **7.999994 pixel²** after translating both endpoints by `(0,0.5)`. Geometric strip area remains **5.656854 pixel²**. Both tracks are fully inside the probe grid. [Failing regression](~/tmp/review-physics-tip-1e437b57/regression.out), [source](~/tmp/review-physics-tip-1e437b57/regression.rs).
   - This is an optical-coverage defect, **not doubled paint volume**: deposition is normalized at `bristle.rs:1111`, while the coverage update is not. Coverage drives compositing at `wet.rs:247`.
   - **Suggested fix:** Intersect the swept strip with axis-aligned pixels or use an area-conserving filter. Test coverage and reflectance for translated diagonal and oblique marks, not only deposited volume.

4. **Medium — `with_hiding` resets an unrelated drying rate.**
   - **Location:** `crates/paint/src/wet.rs:85-86`; default at `71`.
   - The builder recreates the paint with `Paint::new`, silently replacing a previously chosen drying rate with 1.0. `.with_drying(0.3).with_hiding(0.5)` therefore loses the slow pigment behavior, while reversing the builders preserves it.
   - **Evidence:** `review_hiding_preserves_drying` prints **original drying=0.3 adjusted drying=1** and fails. [Test source](~/tmp/review-physics-checkpoint-771a286f/paint/src/review_checkpoint.rs:33), [output](~/tmp/review-physics-checkpoint-771a286f/checkpoint.log).
   - **Suggested fix:** Change only scattering and preserve other fields with `..self`; add a builder-order regression.

5. **Medium — Sub-1% coverage is enlarged to 1% during compositing.**
   - **Location:** `crates/paint/src/wet.rs:233-235`; wet preview at `247`, baking at `drying.rs:501`.
   - `over_share` uses `cover.max(0.01)` as both the thickness divisor and the blend weight. The numerical guard therefore enlarges the area of tiny hair edges instead of preserving their actual area.
   - **Evidence:** Opaque dark paint with coverage 0.001 over white should produce `[0.99901; 3]` under the documented area-weighted equation. It produces **`[0.9901; 3]`**, ten times the darkening. An ordinary 15-unit round-sable stroke on a 100px canvas contains **92 pixels** with nontrivial wet volume and coverage below 0.01. [Regression source](~/tmp/review-physics-checkpoint-771a286f/paint/src/review_checkpoint.rs:41), [output](~/tmp/review-physics-checkpoint-771a286f/checkpoint.log).
   - **Suggested fix:** Keep actual positive coverage as the blend weight; handle zero and division overflow separately. Add sub-1% coverage tests.

## Reproduction commands

Primary scratch directory: `~/tmp/review-physics-600dda80`. Production builds used `CARGO_TARGET_DIR=target-physics`. Only scratch copies were instrumented.

```sh
export TMPDIR=~/tmp/review-physics-600dda80
export TMP="$TMPDIR" TEMP="$TMPDIR" RAYON_NUM_THREADS=2
export CARGO_TARGET_DIR=~/src/a/claude-paint-review3/target-physics
# Finding 1: expected regression failure
timeout 90 cargo test --manifest-path "$TMPDIR/paint/Cargo.toml" review_wait_partition -- --nocapture
# Public-API check, after building the checkout's library:
timeout 60 cargo build -p paint
timeout 30 rustc --edition=2024 "$TMPDIR/wait_partition.rs" \
  --extern paint=target-physics/debug/libpaint.rlib \
  -L dependency=target-physics/debug/deps -o "$TMPDIR/wait_partition"
timeout 30 "$TMPDIR/wait_partition"
```

Findings 2–3: extracted-source standalone tests, no Cargo build:

```sh
export TMPDIR=~/tmp/review-physics-tip-1e437b57
export TMP="$TMPDIR" TEMP="$TMPDIR"
timeout 10 node "$TMPDIR/build-probes.mjs"
timeout 30 rustc --edition=2024 --test "$TMPDIR/regression.rs" -o "$TMPDIR/regression"
timeout 10 "$TMPDIR/regression" --nocapture
```

Findings 4–5 and checkpoint checks ran in an independently instrumented scratch copy with a separate target directory:

```sh
export TMPDIR=~/tmp/review-physics-checkpoint-771a286f
export TMP="$TMPDIR" TEMP="$TMPDIR"
timeout 120 cargo test --manifest-path "$TMPDIR/paint/Cargo.toml" \
  --target-dir "$TMPDIR/target" checkpoint -- --nocapture
```

## Verification and limits

- **PAINTCK4:** No roundtrip defect reproduced. Eight full/cropped resume scenarios at waits of 0, 10, 300 and 2000 minutes passed serialized-byte equality immediately after restore and after another stroke, wait and dry. Existing checkpoint tests also passed. This covers between-stroke canvas state, not external live brush objects. [Test source](~/tmp/review-physics-checkpoint-771a286f/paint/src/review_checkpoint.rs), [log](~/tmp/review-physics-checkpoint-771a286f/checkpoint.log).
- **Existing suite:** Both bounded full-library runs timed out, at 240 and 300 seconds. No existing test failure appeared before either timeout; this is **not a full-suite pass**. The second run records passes for drying tests, loaded-passage filling, crack fitting and palette mean-look tests. [First log](~/tmp/review-physics-600dda80/paint-tests.log), [second log](~/tmp/review-physics-600dda80/paint-tests-limited.log).
- The reported regression failures are intentional assertions of the violated invariants. No additional finding is asserted for handling or stipple.
- No tracked files were edited: final `timeout 10 git diff --exit-code` succeeded. `git status --short` contained only the preexisting untracked `target-easel/`, `target-geometry/` and `target-physics/` directories.
