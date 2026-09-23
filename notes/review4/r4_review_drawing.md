# Drawing engine: correctness findings

The largest verified problems are crop-dependent painting into `drawing_mask()` and loss of drawing state across disk checkpoints. Ten findings follow, most severe first. Runtime evidence and reproductions are in `~/tmp/review-drawing-3f8bb975` (`$S` below). The lobe-join finding is static analysis; the other nine have executed probes.

1. **High — `drawing_mask()` changes subsequent painting when rendering a crop.**

   **Location:** `crates/paint/src/graphite.rs:590–598`; downstream `crates/paint/src/handling.rs:514–547`.

   The method allocates a whole-canvas mask but fills only the drawing cells held by the crop. Everything outside the window becomes zero, unlike a normal whole-canvas planning mask. `work()` uses mask membership to decide which strokes consume its sequential random stream. Missing off-window drawing therefore changes strokes inside the crop, not just at its boundary.

   **Evidence:** `crop_probe.rs` draws the same chalk lines on a whole 500px canvas and a crop `[400,250,600,450]` with an 80-unit margin, then paints into each drawing mask with the same seed. Before painting, the retained crop is pixel-identical. After painting and drying, **9,981 of 10,000 retained pixels differ**, with maximum channel difference **0.7298738**. A control giving the cropped canvas the full drawing mask has maximum difference only **0.00002387166** (6,571 non-bit-identical pixels after drying; this control is not perfectly exact). See `$S/crop_probe.log`.

   **Suggested fix:** Preserve or reconstruct a whole-canvas drawing guide for stroke planning, independently of the cropped optical buffers. Do not claim that a mask with zero-filled off-window data is the whole drawing. Add a full/crop regression for the documented drawing-mask painting workflow.

2. **High — Disk checkpoints discard the entire drawing bookkeeping, not only eraser state.**

   **Location:** `crates/paint/src/checkpoint.rs:93–151,207–258`; `crates/paint/src/canvas.rs:239`; consumers at `crates/paint/src/graphite.rs:542–544,574–598`.

   `write_state()` never serializes `Drawing`. `read_state()` creates a canvas with `drawing: None` and never restores it. The pixels look right immediately after loading, but `drawing_mask()` becomes empty, `drawing_view()` loses the drawing and erase/fix no longer affect it. A resumed program that paints into its hidden drawing consequently omits that paint. This confirms the known issue in `notes/pencil.md:136–138` and extends its stated impact beyond erase/fix. It concerns disk `Canvas` checkpoints, not the Lua session's in-memory canvas clones.

   **Evidence:** `$S/probe.log` reports:
   ```text
   checkpoint pixels_equal=true drawing_before=true drawing_after=false mask_area_before=1129.2108 mask_area_after=0
   erase changed live=644 resumed=0
   ```
   The mask areas are square canvas units. The probe saves and reloads a single ruled 2B line, then erases both copies.

   **Suggested fix:** Serialize the optional drawing layer and every cell field (`a`, `r`, `lift`, `floor`, `film`), version the format and test continuation through erase, redraw and drawing-mask-guided painting.

3. **Medium — `front(selection)` ignores selected coverage and hidden farther members.**

   **Location:** `crates/paint/src/scene.rs:1380–1395`.

   The implementation finds the nearest selected depth and measures opacity in front of that depth. It does not weight by the selected object's coverage and stops before considering farther selected objects. This contradicts its contract of returning the selected object's hidden parts, where it exists.

   **Evidence:** `$S/depth_probe.log` contains two constant-layer counterexamples, sampled above the horizon to avoid ground occlusion:
   - An opaque layer at depth 5 hides a selected 0.25-coverage layer at depth 10. `front` returns **1**, although only **0.25** coverage exists to hide.
   - A selected 0.25 layer at depth 5 and selected opaque layer at depth 10 surround an unselected opaque layer at depth 7. Selected visible coverage is **0.25**, but `front` returns **0**, missing the hidden **0.75** share of the selected union.

   **Suggested fix:** Define multi-object selection semantics explicitly, then compute the selected coverage without unselected occluders minus the selected visible share with them. Add fractional-coverage and interleaved-depth tests.

4. **Medium — Depth masks treat pixels outside a world's panel as opaque sky.**

   **Location:** `crates/paint/src/scene.rs:819–824,1315–1322`; contrast `scene.rs:883–884`.

   An infinite surface depth means either sky or outside `world.view`, but the depth stack converts both to `Thing::Sky`. A visibility-limited sky pass can paint an unrelated panel. `at_depth()` also permits painting there. The existing view API correctly distinguishes `What::Off`.

   **Evidence:** `$S/depth_probe.rs` creates a world occupying the left 500 units of a 1000-unit canvas. At `(750,100)`, `$S/depth_probe.log` reports:
   ```text
   off panel existing_view=Off visible_sky=1 seen=[(Sky, inf, 1.0)] at_depth=1
   ```

   **Suggested fix:** Retain in-view membership in `Depths`, omit off-panel stacks and explicitly zero pass-permission masks outside the view. Test `visible`, `behind`, `at_depth` and `seen_at` in a two-panel canvas.

5. **Medium — A valid inset that consumes its outline cannot produce an empty Lua mask.**

   **Location:** `crates/easel/src/draw_outline.rs:216–220,235`; `crates/paint/src/outline.rs:709–757`.

   The Lua `:mask()` guard rejects an empty outline as an open line. A closed shape inset beyond its thickness should become an empty mask, allowing the documented rim operation `o:mask() - o:inset(d):mask()` to keep the original shape. The Rust mask method already handles no lines.

   **Evidence:** `$S/outline_probe.lua` makes a clean 10×10 square and insets it by 7. The output in `$S/outline_probe.log` is:
   ```text
   empty inset paths  0  mask success  false
   runtime error: outline:mask(): this line is open; use :below() or :above() (or closed=true)
   ```

   **Suggested fix:** Return an empty mask for an empty outline; retain the error for a nonempty open-only outline. Test both cases.

6. **Medium — Explicit `lobe=` defeats `amount=0`.**

   **Location:** `crates/easel/src/draw_outline.rs:48–58`; `crates/paint/src/outline.rs:191–198`.

   The binding applies the amount multiplier first, then restores a zero lobe height to 0.35 when `lobe` is positive. Thus `amount=0` no longer means a clean curve, as promised in `notes/outline.md:89–90`. For characters without default lobes, an explicit lobe likewise bypasses the amount multiplier on its height.

   **Evidence:** `$S/outline_probe.lua` compares horizontal soft outlines with `amount=0`, identical points and seed. Without explicit lobes the maximum y deviation is **0.0000305176** (floating-point interpolation); with `lobe=24` it is **7.2095795 units**. See `$S/outline_probe.log`.

   **Suggested fix:** Apply explicit character options before multiplying their amplitudes by `amount`. Test zero and nonzero amounts with both soft and firm custom lobes.

7. **Medium — Zero pencil pressure still deposits a conspicuous line.**

   **Location:** `crates/paint/src/graphite.rs:446,514–521`.

   At `p=0`, width remains 80% of the point width, bite remains 3 µm and the coverage cap remains 55% of the lead cap. Deposition has no pressure factor or zero-pressure exit. On a flat ground, contact is 1, so a lifted point deposits graphite instead of producing a gap. This affects both zero-valued Lua profiles and pressure profiles intended to lift off.

   **Evidence:** On a flat 1000px, 440mm canvas with initial channel value 0.8, an 800-unit 2B line at pressure zero produces mask area **472.65216 square units** and darkens a channel to **0.6799099** (`$S/probe.log`). The input passes through `hand_line()`'s normal pressure clamping.

   **Suggested fix:** Make deposition vanish at zero pressure, preferably continuously as pressure approaches zero, while retaining the tooth/contact model for positive pressure. Add zero-pressure and lifted-profile tests.

8. **Medium — Lobe displacement is discontinuous at joins and the closed seam.**

   **Location:** `crates/paint/src/outline.rs:1065–1072,1086–1092`.

   **Static evidence:** Each interval has a different generated height `h`, but its displacement is `h * (sin(pi*u)^0.6 - 0.55)`. At both endpoints that tends to `-0.55*h`. Adjacent heights 2 and 4 therefore approach -1.1 and -2.2 at their shared knot: a 1.1-unit jump for equal local fit. `partition_point` switches heights at the knot. The last and first lobe heights are not matched for a closed line, unlike facet endpoints at `outline.rs:1052–1055`. Sampling connects these jumps with sharp segments rather than continuous rounded troughs. This undermines the rounded-lobe/no-seam claim in `notes/outline.md:22–24`.

   **Suggested fix:** Use shared trough values at neighboring knots, interpolate the baseline continuously and add a bulge that vanishes at either end. Make the baseline periodic on closed lines. Add unequal-height endpoint-limit and closed-wrap tests. No runtime visual reproduction is claimed for this finding.

9. **Low — Invalid non-ASCII pencil grades panic instead of returning an error.**

   **Location:** `crates/paint/src/graphite.rs:70–78`; Lua caller `crates/easel/src/draw_pencil.rs:27–28`.

   `softness()` splits at `g.len()-1`, a byte index that need not be a UTF-8 boundary. The normal invalid-grade path should return `None`, allowing the Lua binding to issue its helpful grade error, but inputs such as `é` panic first.

   **Evidence:** `$S/probe.rs` catches the panic from `softness("é")`; `$S/probe.log` reports `unicode grade panicked=true` and `end byte index 1 is not a char boundary`.

   **Suggested fix:** Recognize ASCII `H`/`B` with `strip_suffix` or reject non-ASCII grades before splitting. Test accented characters and emoji alongside ordinary invalid grades.

10. **Low — Documented `visible=mask` is rejected by the pass API.**

    **Location:** `crates/easel/src/depth.rs:235–237,91–96`; claim `notes/depth.md:61–62`.

    `visible` always parses its argument as a world selection, whose parser rejects mask userdata. Only `behind` has a mask-specific branch. The advertised mask input therefore does not work, including inside a mixed list.

    **Evidence:** `$S/visible_probe.lua` creates a view and executes `glaze(nil,{color='#402010',visible=rect(100,100,50,50)})`. `$S/visible_probe.log` reports `visible mask accepted false` with `want a body number, a layer name, "ground", "water", "sky" or a list, got userdata`.

    **Suggested fix:** Resolve masks explicitly for `visible` and define mixed-list composition, or narrow the documented API if this support was not intended.

## Claim checks and limits

- **Beaded tree lead:** The supplied `notes/pencil/painted_1000.jpg` visibly has separated dots along upper limbs. The recipe thresholds the physical deposit with `drawing_mask():band(0.55,1,0.1):grow(1.2)` (`paintings/lua/pencil.lua:111`). `drawing_mask` reflects tooth- and grain-modulated coverage, not continuous path membership (`graphite.rs:492–520,590–598`). A representative limb probe has several zero band samples along the path at both 1000px and 3200px (`$S/probe.log`), so gaps can already exist before painting. This supports thresholding a textured deposit as a cause, but does not isolate every cause in the complete painting. No separate numerical engine defect is asserted solely from the image. A geometric drawing-guide mask, or a recipe that preserves weak connecting deposits, would avoid relying on strong-deposit islands.
- **Kubelka–Munk:** Graphite is written into the dry pixels (`graphite.rs:526–530`), which ordinary glaze and wet-paint compositing use as the substrate (`canvas.rs:413`, `wet.rs:232–253`, `drying.rs:517`). No graphite-specific paint-opacity branch was found. The existing thin-paint/body-color test passes (`graphite.rs:747–767`; `$S/graphite-tests.log`). This verifies its tested glaze case, not every possible brush/medium combination.
- **Outline determinism/resolution:** Existing outline tests pass. An additional draw/body/offset probe hashes the geometry and stroke data identically with one and four Rayon threads: `16a7848f00577432` (`$S/outline_determinism.log`). Geometry construction does not take a raster Frame (`outline.rs:499,574,709`), while masking does (`outline.rs:751`); no concrete outline geometry resolution-dependence defect was established. This is not a claim that raster masks are identical at different resolutions.
- **Limit masks:** Inspected the `limit` routing in `handling.rs:595–612,763` and `stipple.rs:509–518,555–559`, including fill and cut-in paths. The three existing easel depth tests pass (`$S/easel-depth-tests.log`). No additional limit-specific failure was established; this is not exhaustive verification of every brush footprint or concurrent write boundary.
- **Contact shadows:** `notes/depth.md:42–43` promises ground-only shadows that never darken foreground figures. In contrast, `scene.rs:1491–1495,1517–1521` explicitly documents and implements ground occlusion on visible body surfaces near their feet, even if the body-caster filter selects nothing. This is a contract/documentation mismatch, not established here as a numerical defect: body-ground contact shading is explicitly part of the engine method's contract. Clarify the notes and `from` semantics rather than blindly removing physical body shading. The existing shadow falloff tests pass (`$S/scene-tests.log`, `$S/easel-depth-tests.log`).

## Verification and reproduction

All work used the detached checkout without tracked edits; both tracked diff checks exited 0 (`$S/tracked-diff.log`). Tests and builds used `CARGO_TARGET_DIR=target-drawing`. Probes and outputs were kept in the persistent scratch directory.

Executed existing suites: **28 passed, 0 failed, 1 ignored**, across these four commands:

```sh
export S=~/tmp/review-drawing-3f8bb975
export TMPDIR="$S" TMP="$S" TEMP="$S" CARGO_TARGET_DIR=target-drawing
timeout 180 cargo test -p paint graphite -- --nocapture  # 7 passed, 1 ignored
timeout 180 cargo test -p paint outline -- --nocapture   # 5 passed
timeout 180 cargo test -p paint scene -- --nocapture     # 13 passed
timeout 180 cargo test -p easel depth -- --nocapture     # 3 passed
```

Logs: `$S/graphite-tests.log`, `$S/outline-tests.log`, `$S/scene-tests.log` and `$S/easel-depth-tests.log`. The scene selection includes the golden-image and cross-thread-count scene tests. Passing these existing tests does not negate the counterexamples above.

Standalone Rust probe commands, using the paint artifact built in this checkout:

```sh
timeout 120 cargo build -p paint
for p in probe depth_probe crop_probe outline_determinism; do
  timeout 60 rustc --edition=2024 "$S/$p.rs" \
    --extern paint=target-drawing/debug/deps/libpaint-a8ab5859f2abbd0f.rlib \
    -L dependency=target-drawing/debug/deps -o "$S/$p"
done
timeout 90 "$S/probe"
timeout 45 "$S/depth_probe"
timeout 90 "$S/crop_probe"
RAYON_NUM_THREADS=1 timeout 60 "$S/outline_determinism"
RAYON_NUM_THREADS=4 timeout 60 "$S/outline_determinism"
```

The `.rlib` suffix is the artifact used here; use the corresponding newly built artifact if rebuilding with a different toolchain or feature set. These probes print observed counterexamples rather than failing their process exit status; `probe` intentionally catches the Unicode panic so later checks continue.

Lua probes:

```sh
timeout 120 cargo build -p easel
timeout 60 target-drawing/debug/easel run "$S/outline_probe.lua" \
  --width 100 --out "$S/outline_probe.png"
timeout 60 target-drawing/debug/easel run "$S/visible_probe.lua" \
  --width 100 --out "$S/visible_probe.png"
```

Both Lua probes use `pcall` to print the unexpected errors and continue; their successful CLI exit is not an assertion that the probed behavior is correct.
