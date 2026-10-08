# Oil-lighting candidate: separate body and grooves

Display-only candidate, not a default. The inspected brush interiors are much less embossed but approach the flat-color view; this is not an established visual improvement. User approval is still required before any default change.

- Baseline: `8eeb9bec3ce146bcd15c37db447665391fb95bdd`.
- Branch: `experiment/oil-lighting`, retained worktree `~/src/a/claude-paint-oil-lighting`.
- Implementation and PNG evidence commit: `90b7ee449db73e8e45b8fd1c0abe98bc193fb226`. Report/runner receipts are a subsequent documentation commit on the same branch.
- Private preview: <http://m3p.tailec2dc.ts.net:8771/index.html>. HTTP 200 and all six images loaded in the browser from the serving machine. Another machine's connectivity is not verified.

## Implementation boundary

`Canvas::seen_lit_body_candidate` is the explicit experiment entry point in `crates/paint/src/canvas.rs`. `seen_lit` still selects the historical branch of one shared renderer. No renderer copy, configuration framework, engine bump, checkpoint field, Lua view selection or default was added. `crates/paint/examples/oil_lighting.rs` is the experiment caller.

The broad normal comes from two box-blur passes over the existing dry-plus-wet height field. Each radius is `round(0.6 / px_mm())` pixels. Only this smooth normal controls diffuse illumination and the broad highlight. The original fine surface normal contributes a narrow highlight, not diffuse light. Both normals retain the existing soft slope limit, 1.2. All math remains linear-light; conversion to sRGB happens only when writing the PNG.

Chosen study constants, not measured optical properties:

| Constant | Candidate | Historical |
|---|---:|---:|
| Body filter | Two 0.6 mm-radius box passes | None |
| Broad highlight exponent | 24 | 40, single lobe |
| Narrow highlight exponent | 120 | No separate lobe |
| Broad/narrow weights | 0.85 / 0.15 | Single lobe |

The weights sum to one. The sheen ceiling remains `(0.02 + 0.10 * min(4 * wet_volume, 1)) * lit`, at most 0.12 linear RGB. No oil-content or dry-gloss response was introduced. The current ambient light, diffuse ceiling of 1.6, lamp/view vectors and actual-height shadow ray calculation are shared unchanged, including the existing one-pixel shadow blur. Mixing, pigment, deposition, brush/knife geometry and drying are untouched. No reference bundle code or assets were copied; the original brush's unresolved license therefore does not enter this candidate.

## Reproducible marks and images

The complete recipe is `crates/paint/examples/oil_lighting.rs::scene`:

- Engine 5, smooth neutral support `#746b60`, physical width 24 mm, aspect 1.5.
- Hog flat brush, width 110 canvas units (2.64 mm), seed 73, full load of body paint `#b56b32`.
- First pull `(140,145)` → `(800,170)`, pressure 0.75 → 0.55.
- Second pull from the same partly spent reservoir `(160,325)` → `(800,295)`, pressure 0.6 → 0.35.
- Knife width 130 units (3.12 mm), full load of body paint `#436b91`; pull `(160,490)` → `(760,490)`, pressure 0.45 → 0.7, default angle, lay=true, lift=0.12.
- Fixed lamp azimuth 135°, elevation 25°, strength 0.8, straight-on view. No wait, dry, relief edit, crop, dither or tone mapping between views. Canvas clock remains zero.

Evidence: `current-160.png`, `candidate-160.png`, `flat-160.png` (160×107) and the corresponding `*-320.png` (320×213). `index.html` compares them, displaying the 160 px row at 2× with nearest-neighbor scaling. Each row uses exactly one material state; rows are independent runs of the same recipe, not identical material across resolutions.

Reproduction, from the assigned worktree only:

```sh
cd ~/src/a/claude-paint-oil-lighting
uv venv .venv
source .venv/bin/activate
export CARGO_TARGET_DIR="$PWD/target"
notes/oil-lighting/run.sh
notes/oil-lighting/check-baseline.sh
```

`run.sh` locks the build at 600 seconds and render at 60 seconds. `check-baseline.sh` temporarily restores only this worktree's baseline `canvas.rs`, substitutes the historical view for the example's candidate call, compares outputs then restores candidate sources through an EXIT trap and rebuilds the candidate executable. It must run serially, without another build/edit in this worktree. The baseline receipt labels that substituted view `baseline-alias`; it is not candidate output. No main checkout or other worktree is touched.

## Unchanged material and historical output

The example writes the full checkpoint to memory before rendering and compares its bytes after current, candidate and flat views. Both resolutions passed every exact comparison. Checkpoint sizes are 1,643,838 and 6,543,678 bytes. `measurements.txt` records those assertions and the unchanged clock.

`baseline-proof.txt` establishes byte-identical current/flat PNGs against the unchanged baseline renderer and equal material SHA-256 hashes:

- 160 px: `6bd330a4ac2089ee554a349cc01cc745b708d14460031d5ee7201adfc6ae4528`.
- 320 px: `e1047daf8158233eb093a10d2f146b7ffa9ecbcbdd0f1448da90fa3689af09d4`.

`SHA256SUMS` records the six PNGs and both material hashes. Temporary checkpoint files were deleted after comparison; the example regenerates them. The opt-in path takes `&self` and introduces no saved state.

## Physical scale and bounds

At 160 and 320 px, pixel widths are 0.15 and 0.075 mm. The filter radii are 4 and 8 pixels, both 0.6 mm. Their two-pass standard deviations are approximately 0.548 and 0.520 mm, calculated from the discrete box variance, not measured physical roughness.

The separate analytic test samples one fixed 16 mm height profile at 160 and 320 px: a 350 µm mound with 4 mm characteristic width plus 15 µm grooves of 0.4 mm wavelength. Comparing the low-resolution linear render with 2×2 averages of the high-resolution render, excluding boundary padding, measured mean absolute channel error **0.00016227**, maximum **0.00049414**. Chosen acceptance bounds are 0.001 and 0.004; this checks rendering scale, not physical accuracy or resolution-independent brush geometry.

Other owner-boundary tests enforce finite nonnegative output, the existing diffuse/specular ceiling and unchanged checkpoint bytes across engines 1–5. A 1 mm-high, 1 mm-wide ridge preserves the historical shadow away from the body filter at a 10° lamp. The first shadow fixture was only one pixel wide; the retained shadow blur reduced its height to one-third, invalidating the assumed shadow reach. Widening the fixture corrected the test, not the renderer.

At pixel sizes too coarse to resolve the chosen filter, its radius may round to zero. The candidate does not invent a larger physical filter. Cropped candidate renders need enough material margin for two filter passes; exact crop equivalence was not tested.

## Visual inspection and timings

All six PNGs were inspected together in the browser, with the 320 px images also opened individually. Subjective observations:

- Current lighting strongly emphasizes each brush furrow. Candidate lighting removes most of that repeated light/dark relief, leaving subtle broad variation and a few dark slit shadows.
- At this lamp angle, broad paint-body rounding and narrow glints are not strongly visible. The candidate brush interiors look close to flat, especially at 160 px.
- The knife retains its silhouette, tears and right-side physical shadow but loses much of the thin bright edge present in current lighting.

The 0.6 mm smoothing choice may be too aggressive for these marks. These observations support showing the candidate, not promoting it or claiming physical realism. Geometry limitations and different lamp angles remain unmeasured.

Single observations on this host with four Rayon threads, excluding PNG encoding/checkpoint writing:

| Width | Scene construction | Current render | Candidate render | Current / candidate max linear channel |
|---|---:|---:|---:|---:|
| 160 | 19.338 ms | 0.987 ms | 1.245 ms | 0.610563 / 0.485750 |
| 320 | 37.052 ms | 3.351 ms | 3.554 ms | 0.768301 / 0.503363 |

These are not a benchmark or a speedup claim. The initial candidate release build took 17.91 s; baseline renderer build 16.17 s; restored candidate build 16.33 s. The recorded focused tests took 0.09 s in the test binary (`focused-tests.txt`); a final repeated check passed the same three tests in 0.11 s.

## Tests and remaining limits

`focused-tests.txt`: **3 passed**, zero failed/ignored. No protected expected outputs or goldens changed.

The fast and all lists were run with explicit build/check phases through `check-bounded.sh`. The subset verdicts are **unfinished** for builds (intentional filtering) and **fail** for checks; neither is a full-suite pass. All three build steps succeeded: release easel 54.69 s, test binaries 38.70 s and thinner release test binaries 41.83 s in the fast run; the all run reused them.

Passed check steps in the bounded runs:

| Step | Count reported by runner | All-list wall time |
|---|---:|---:|
| Old-log replay/image checks | 11 | 25.46 s |
| Image-tolerance script (eight internal cases) | 1 | 0.80 s |
| Lockrun checks | 17 | 23.84 s |
| Test runner checks | 18 | 72.96 s |
| Safeguards checks | 140 | 94.07 s |
| Replay clip checks (64 px synthetic frames) | 1 | 5.51 s |
| Box-tube checks | 1 | 0.74 s |
| Studio-name checks | 1 | 1.58 s |

The all check phase took 145.5 s, with grouped steps overlapping. No step timed out. Selected runner values are consolidated in `receipts/checks.json`: exits, counts, commands, times and original log hashes. The two relevant failure logs are retained with redacted paths; redundant passing logs are omitted. The initial phases tested the dirty candidate over baseline HEAD; the all fixture recheck used source commit `90b7ee44`. There is no fresh-worktree `--candidate` receipt.

Initially `baseline-package` exited 1 and `baseline-state` exited 2 because sparse checkout excluded the six tracked baseline PNGs. They were restored byte-for-byte from the baseline Git blobs, without changing sparse configuration or expected values. The package check then passed in 2.72 s in the fast recheck and 2.76 s in all: hashes valid, comparator checks passed and 27 dumps reconstruct the original checkpoints.

After fixture restoration, `baseline-state` still exits 1: body, overlap and pickup match every state field and PNG, then rag differs in chunks 4–8. The first changed chunk has 382 differing wet-volume values (maximum difference 2.96356 coats), changed rag loading and nonzero added `rags.mobile` fields. Stroke and wait comparisons are not reached. The rag behavior is outside this lighting-only change; no protected answers or material implementation were changed to make that check pass. The failure log is byte-identical with the unchanged `8eeb9bec` canvas renderer (`baseline-state-unmodified.txt`, exit 1), establishing that this failure predates the candidate. That probe temporarily restored baseline `canvas.rs`, built release easel under a 600 s lock, ran `scripts/tests/baseline_state.sh` under a 60 s lock then restored candidate source and binaries. No other material source changed.

The feature-table step also ran separately: 21 successful configuration reports, then exit 1 after 102.88 s. `the_replay_build_holds_every_box` expects nine box names but the catalog returns ten, including `every` (`crates/easel/tests/boxes.rs:206`, `receipts/box-features.log`). Both the test and palette source are unchanged from baseline: this is an existing expectation mismatch, not a lighting result. It was not repaired in this task.

The bounded commands were:

```sh
notes/oil-lighting/check-bounded.sh fast
notes/oil-lighting/check-bounded.sh all
uv run --no-project python scripts/test --phase check --only baseline-package --only baseline-state
uv run --no-project python scripts/test --all --phase check --only baseline-package --only baseline-state
uv run --no-project python scripts/test --all --phase check --only box-features
```

Each `scripts/test` invocation takes its own lock. On this sparse worktree, the setup preceding fixture-dependent checks was `mkdir -p notes/thinner/baseline/ref/png` followed by `git show 8eeb9bec3ce146bcd15c37db447665391fb95bdd:notes/thinner/baseline/ref/png/<name>.png > notes/thinner/baseline/ref/png/<name>.png` for body, overlap, pickup, rag, stroke and wait. These are existing expected blobs, not new answers. A full-suite pass cannot be claimed: `cargo-test`, `slow-tests-by-name`, `slow-thin-blend` and the thinner acceptance steps include scenes above the requested 320 px limit (`notes/speed/SKIPPED.md`, `crates/easel/tests/determinism.rs`, `crates/paint/tests/thinner_physics.rs`). The painter/smoke session tests also render live-width views; the image utility/website suites include larger images. Specifically omitted: `cargo-test`, `slow-tests-by-name`, `slow-thin-blend`, `painter-build-inness`, `thinner-quick`, `thinner-all`, `replay-env`, `check-live`, `smoke-replay-without-finish`, `runner-round24`, `studio-viewer` and `peek`. Those steps were not run as substitutes at a smaller width.

An initial attempt to select both build/check names through `scripts/test --only ...` and its `--all` variant ran zero steps: the runner validates all selected names against the already phase-filtered list. `check-bounded.sh` uses separate explicit phases without changing the protected runner. The failed invocations count as unfinished, not tests passed.

Generated `target/` build outputs (7.3 GiB) and temporary material checkpoints were removed. The worktree, committed PNGs, source, receipts and private preview remain. The small `.venv` is retained for that running preview and reproducible checks. No push, PR, merge, configuration change or default promotion occurred.

Not done yet: user visual approval, the unrelated rag baseline-state failure and box-inventory expectation mismatch, a complete required-suite run under permission to exceed the 320 px cap and any default promotion.
