# Scheduling, determinism, crop and checkpoint findings

1. **High — safe public brush parameters can invalidate the rectangles protecting parallel raw-pointer access.** `crates/paint/src/bristle.rs:48,530–535,570,606,621–628`; parallel caller: `crates/paint/src/handling.rs:623–628`.
   - `Tool.length` is public and unvalidated. A negative length makes the bend relaxation rate negative, so tips diverge instead of staying within the bound assumed by `footprint`. The footprint remains finite and is accepted. `run_ordered` can consequently run supposedly disjoint tiles whose actual pixel accesses overlap. Physically invalid parameters must still be rejected before safe Rust APIs enter this unsafe path.
   - **Evidence:** a scratch-copy test uses `Tool::round_sable(10.0)` with `length = -1.0`, a 300×300 canvas and a drag from `(400,500)` to `(500,500)`. The planned rectangle is `(108,138,163,163)`, but **9,299 pixels outside it** receive stroke-touch IDs, beginning at `(0,50)`. A second test invokes public `Canvas::work` with that tool on a **one-thread** Rayon pool and checks each returned bound against its scheduled tile. It reports, for example, `tile=28 scheduled=(211,207,300,279) actual=Some((255,0,300,300))`. These prove the safety precondition fails; no multithreaded race was deliberately executed. Sources: `instrumented-paint/src/review_sched.rs`; outputs: `footprint-repro.log`, `tile-repro.log` under the scratch directory below.
   - **Fix:** validate finite, physically permitted tool parameters at painting entry points, including positive dimensions and the documented stiffness range. Ensure the bend integrator is bounded for every accepted value. Public mutable fields mean constructor-only validation is insufficient. Add rejected-input and footprint-containment tests.

2. **High — stages inside loops can silently resume stale paint.** `paintings/src/run.rs:168–175,259–261,286,321`; existing caller: `paintings/src/bin/study_form.rs:218–232`.
   - The source hash stops before the *next stage call's source line*. When a loop calls `stage` repeatedly at the same line, ending one iteration hashes only code before that call: none of the stage body is covered. `study_form` uses this pattern. Changing its dispatch/body after line 223 does not invalidate earlier loop-iteration checkpoints. This is separate from the documented limitation about helpers below `main`.
   - **Evidence:** scratch `loop.rs` includes the real `run.rs`. Save two loop stages with body `c.apply(|_, _, _| [0.2; 3]);`, then change only that body to `c.apply(|_, _, p| [p[0] + 0.3; 3]);`, recompile and resume `first` without `--stale-ok`. It accepts the checkpoint and produces `[0.5,0.5,0.5]`; a fresh run produces `[0.70000005,0.70000005,0.70000005]`. Logs: `loop-save.log`, `loop-resume.log`, `loop-fresh.log`.
   - **Fix:** use explicit stage fingerprints/dependencies or capture the stage body in the API. At minimum, hash the entire painting source for repeated/dynamic call sites rather than treating the next call's line as the previous body's end.

3. **Medium — final-stage checkpoints drop the painter's saved state; appending a stage makes resume panic.** `paintings/src/run.rs:359–360,324,303–304,69–70`.
   - `end` and `finish` close the last stage with `None` instead of its `Keep` state (usually the RNG), writing empty `state=`. Resuming that final stage also omits restoration. After appending a stage, the new `stage` call closes the skipped former last stage with `Some(&mut rng)` and tries to restore eight bytes from an empty state.
   - **Evidence:** scratch `resume.rs` has two stages that advance an RNG, followed by `o.end(&mut c)`. Saving ends with RNG state `15755400384260043838`; unchanged `--resume last` ends with initial state `11400714819323198484`. Replacing the `o.end` line with a third stage followed by `o.end` passes staleness validation but panics at `run.rs:70`: `range end index 8 out of range for slice of length 0`. Logs: `last-save.log`, `last-resume.log`, `appended.log`. The retained source is the appended-stage variant.
   - **Fix:** finalization must receive the final `Keep` value, or stage completion must capture it after each body. Restore independently of whether another stage exists. Validate serialized state lengths instead of slicing blindly.

4. **Medium — stipple coverage can disappear entirely between the coarse maximum probes.** `crates/paint/src/stipple.rs:304–324`.
   - `dmax` is estimated on a grid spaced by half of `max(3 × tool.width, 4)` units. If a nonzero coverage band falls between those samples, `dmax` is zero and stippling returns before planning any touches. For partly sampled peaks, the same estimate also caps candidate density below the requested coverage. This affects valid spatial coverage functions, not just crop margins or brush history.
   - **Evidence:** `stipple_probe.rs` uses a 400px-wide canvas, a full mask and default `Tool::stippler(2.0)`, asking for coverage 10 in a two-unit horizontal band. With the same seed and all other parameters fixed, `[100,102)` paints **0 pixels**; shifting to `[102,104)` paints **2,393 pixels**. Output: `stipple_probe.log`. A narrow mist band can therefore vanish solely because of its alignment to the probe lattice.
   - **Fix:** provide a conservative user-supplied coverage bound or derive density from an adequately sampled/adaptive coverage field. Do not interpret a sparse probe miss as proof that coverage is zero. Add narrow-band and shifted-band regressions.

5. **Medium — checkpoint loading accepts invalid crop-save bounds and can panic during frame validation.** `crates/paint/src/checkpoint.rs:147–155`; downstream failure: `crates/paint/src/canvas.rs:408–415`.
   - `read_state` uses unchecked dimension arithmetic and installs serialized `keep` bounds without checking that they fit the buffer. A corrupt checkpoint can load successfully and crash in normal saving rather than returning `InvalidData`.
   - **Evidence:** `invalid.rs` writes a valid 2×2 state, changes only `keep.x1` from 2 to 3 and reloads it successfully. Saving then panics at `canvas.rs:415`: `len is 4 but index is 4`. Another mutation sets `w=u64::MAX, x0=1`; the debug library panics at `checkpoint.rs:147` with `attempt to add with overflow`. Both are recorded in `invalid.log`.
   - **Fix:** use checked dimension arithmetic before allocating and validate ordered, in-buffer `keep` bounds. Reject malformed geometry with `InvalidData`; also validate dirty bounds and finite positive scales.

## Reproduction and verification

All harnesses and logs are retained in **`~/tmp/review-sched-7b06f4b4`**. Tracked source was not edited. The added footprint assertions exist only in the scratch crate copy.

From `~/src/a/claude-paint-review`:

```sh
export TMPDIR=~/tmp/review-sched-7b06f4b4
export TMP="$TMPDIR" TEMP="$TMPDIR"
export CARGO_TARGET_DIR=~/src/a/claude-paint-review/target-sched

# Existing suite: 50 passed, 0 failed, 2 ignored (paint-tests.log).
timeout 240 cargo test -p paint --lib

# Expected failures proving the footprint/tile violations, not fixes.
timeout 120 cargo test --manifest-path "$TMPDIR/instrumented-paint/Cargo.toml" \
  finite_tool_parameters_must_not_escape_scheduled_rect -- --nocapture
timeout 120 cargo test --manifest-path "$TMPDIR/instrumented-paint/Cargo.toml" \
  work_must_respect_tile_rect -- --nocapture

# Public-API stipple repro.
timeout 180 cargo build -p paint -j 2
timeout 30 rustc --edition=2024 "$TMPDIR/stipple_probe.rs" \
  --extern paint=target-sched/debug/libpaint.rlib \
  -L dependency=target-sched/debug/deps -o "$TMPDIR/stipple_probe"
timeout 40 "$TMPDIR/stipple_probe"

# Checkpoint harnesses include the real run.rs. This manifest location
# keeps their output in scratch. Substitute loop, resume or invalid.
export CARGO_MANIFEST_DIR="$TMPDIR/paintings"
timeout 30 rustc --edition=2024 -Awarnings "$TMPDIR/loop.rs" \
  --extern paint=target-sched/debug/libpaint.rlib \
  -L dependency=target-sched/debug/deps -o "$TMPDIR/loop"
timeout 10 "$TMPDIR/loop" --width 2 --resume first
timeout 10 "$TMPDIR/loop" --width 2
```

The baseline includes scheduler-order, thread-count determinism, footprint and crop tests; their passing results are in `paint-tests.log`. No additional defect was established in ordinary-parameter scheduling or crack-window rasterization. The crop approximation limits already documented in `notes/workflow.md` are not counted as findings.
