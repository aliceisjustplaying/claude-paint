1. **Medium — thin ellipsoids silently lose visible surface pixels.** `crates/paint/src/form.rs:428–442` stops tracing after 160 steps and reports `None`, without distinguishing exhaustion from a miss. The ellipsoid distance estimate (`form.rs:294–298`) converges too slowly for elongated solids. **Evidence:** `geometry.rs` scans interior points satisfying `(x/rx)² + (y/ry)² < 0.95`: radii `[10,100,100]` lose 144 hits; `[5,100,100]` lose 2,990. For center `[500,350,0]`, radii `[5,100,100]` and pixel `(495.25,329)`, the analytic front is `z=23.108446`; iteration 159 is still at `z=29.600828`, distance `0.090795666`, then `hit` returns `None` (`trace.log`). Narrow stones or transformed thin masses develop missing regions rather than merely inaccurate shading. **Fix:** use an analytic ray/ellipsoid intersection for the primitive and a convergence-safe fallback for composed SDFs; test aspect ratios 10:1 and 20:1, not only nearly spherical masses.

2. **Medium — rotating about an off-center pivot can clip real geometry.** `crates/paint/src/form.rs:385–391` estimates the rotated bounds from only four of eight box corners. It omits both mixed x/z corners, so its supposedly generous enclosing sphere can be too small. `Form::add_at` trusts these bounds (`form.rs:750–760`). **Evidence:** `block([500,350,0], [20,20,20], 0).turn([600,350,-100], PI/4, 0, 0)` reports minimum x `457.5219`, but directly hits `(450.5,350.5)` at `z=-93.91011`. Adding it to a 1000×700 Form leaves that pixel absent (`geometry.log`). **Fix:** transform all eight corners and bound them, or compute the maximum pivot distance using all eight. Test rotation around a pivot outside the body.

3. **Medium — `.coverage(0.0)` still paints.** `crates/paint/src/handling.rs:400` silently clamps coverage to at least `0.05`; there is no zero-coverage early return. This makes disabling a pass with a coverage parameter destructive. **Evidence:** a 100×100 canvas with full mask, `Handling::new(Tool::filbert(22.0)).coverage(0.0).color(|_,_|[0.05;3])`, seed 1 changes **1,549 pixels** after drying (`probe.log`). **Fix:** return before planning when coverage is zero; reject negative/nonfinite coverage explicitly. Add a no-op test for zero on both painting and blending handlings.

4. **Medium — exported tree limbs erase dead sections of the leader.** `crates/paint/src/growth.rs:804–816,841` follows a continuous limb through live/dead transitions but sets the entire limb's `dead` flag from its start. Decline kills upper leader nodes (`growth.rs:700–704`); the root remains live. `paintings/src/trees.rs:64` therefore uses live bark for the dead upper trunk, and callers cannot recover the missing state. **Evidence:** current-source internal probe, `Habit::dead_oak()`, seed 7: the trunk path has **20 dead nodes out of 55**, but `limbs[0].dead == false` (`motif_probe_output.txt`). **Fix:** split exported limbs at state changes while preserving parent connections, or export and paint per-segment dead state. Test node-to-skeleton state preservation.

5. **Medium — dead-oak decay leaves sibling subtrees longer than its promised claws.** `crates/paint/src/growth.rs:722–730` keeps the strongest child for a two-internode claw and deletes that child's children, but retains the first node's other children and their subtrees. Later pruning skips those children because their parent is already thin and dead (`growth.rs:714–715`). This contradicts the one- or two-internode decay described in `notes/motifs.md`. **Evidence:** the current-source probe finds surviving thin-dead subtrees of depth 3 for seed 2, depth 4 for seed 7 and two depth-3 subtrees for seed 11, all with default `Habit::dead_oak()` (`motif_probe_output.txt`). **Fix:** remove every sibling except the chosen continuation at each retained claw node, then remove all children at its terminal node. Assert maximum surviving thin-dead subtree depth ≤2.

6. **Medium — positive pitch tips the top away from the viewer, opposite the API contract.** `crates/paint/src/form.rs:252–254` promises that positive pitch brings the top toward the viewer, but the matrix at `form.rs:261` maps negative y to negative z. The module defines positive z toward the viewer. **Evidence:** a unit sphere centered at `[0,-10,0]`, turned around the origin with pitch `PI/2`, has its front hit near `z=-9` (`probe.log`), placing its center at `-10` instead of `+10`. `paintings/src/rocks.rs:169–170` describes outcrop tops seen from above, then uses positive pitch `0.32` at line 192. **Fix:** reverse the pitch matrix's sine signs to match the public contract, then review affected motif renders; add a basis-vector rotation test.

7. **Low — the documented Form storage budget omits large temporary allocations.** `notes/form.md:108` correctly states ~190 MB of persistent storage at 3200px for a landscape canvas, but is insufficient as a working-memory budget. `crates/paint/src/form.rs:759–766` collects a full rectangular `Vec<Option<(Hit,f32)>>` before installing any hits; `mask.rs:157–167,239–256` allocates multiple full-frame distance-transform buffers during a silhouette. **Evidence:** an instrumented allocator on a **3200×2133** Form with one full-frame flat relief measures **191.1 MB** live after construction, **382.3 MB** peak during `add` and **423.4 MB** peak during `silhouette`. This excludes any Canvas, painter masks or allocator/RSS overhead (`memory.log`). Cropping does not reduce these allocations when following `Form::new(c.frame())`, because `c.frame()` is the whole frame (`canvas.rs:245–247`). **Fix:** process hits by row/tile, reuse distance-transform buffers and document peak working memory plus aspect ratio. Until then, budget well above 190 MB per active motif, even in a crop.

## Reproduction receipts

Scratch sources and logs: `~/tmp/review-api-42caee19/`. No tracked files changed (`git diff --stat` was empty).

From the review checkout:

```sh
export TMPDIR=~/tmp/review-api-42caee19
export TMP="$TMPDIR" TEMP="$TMPDIR" CARGO_TARGET_DIR=target-api
# Baseline checks: four form tests pass, three mask tests pass, all workspace targets check.
timeout 180 cargo test -p paint --lib form::tests -- --nocapture
timeout 180 cargo test -p paint --lib mask::tests
timeout 180 cargo check --workspace --all-targets
# Build a library at a stable, hash-independent path for all probes.
timeout 180 cargo build -p paint --lib
for name in probe geometry trace memory; do
  timeout 30 rustc --edition=2024 "$TMPDIR/$name.rs" \
    --extern paint=target-api/debug/libpaint.rlib \
    -L dependency=target-api/debug/deps -o "$TMPDIR/$name"
  RAYON_NUM_THREADS=2 timeout 60 "$TMPDIR/$name"
done
# Internal growth probe: current growth.rs copied into scratch, with inspection code appended.
timeout 40 rustc --edition=2021 -O "$TMPDIR/motif_probe.rs" \
  --extern paint=target-api/debug/libpaint.rlib \
  -L dependency=target-api/debug/deps -o "$TMPDIR/motif_probe"
timeout 30 "$TMPDIR/motif_probe"
```

Saved baseline output: `form-tests.log`, `mask-tests.log`, `check.log` and `build.log`. The growth probe was also rerun against the freshly built debug library with identical output (`motif_probe_debugdep.log`). The adversarial probes report incorrect results despite those passing checks; they are not added regression tests.

## README assessment

The stage example in `README.md:34` uses the current `Run::stage(name, canvas, state)` signature (`paintings/src/run.rs:259`), and `cargo check --workspace --all-targets` succeeds. This is not a claim that every documented CLI invocation was rendered or that checkpoint exactness was independently tested here.

For the incoming painter agent, README is incomplete onboarding: its engine map (`README.md:38–59`) omits `form`, and its coordinate paragraph (`README.md:63–64`) does not distinguish `c.frame()` from `c.window()` or explain that masks and Forms still use the whole canvas during crops. Add direct links to `notes/form.md` and `notes/color.md`, plus the whole-frame and peak-memory warning above. The color link matters because palette handlings default to aimed results while glaze presets explicitly use masstone (`handling.rs:63–66`, `style.rs:278–290`); the README currently links strokes/workflow but not these semantics. No additional README correctness defect is asserted without a repro.
