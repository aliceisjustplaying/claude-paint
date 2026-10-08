# Irregular bristle groups and folds: isolated candidate

**Implemented, opt-in only. No default promotion.** The candidate adds coherent tip grouping and local ridge/hollow pairs through brush contact and finite deposition, under unchanged current lighting. The complete repository gate is **not passed**; verification failures and omitted checks are below.

Code commit: `9e28e343e666b13d018fb0d4137b156cf4cfe0c6`.
Baseline: `8eeb9bec3ce146bcd15c37db447665391fb95bdd`.
Worktree retained: `~/src/a/claude-paint-oil-bristle-folds`, branch `experiment/oil-bristle-folds`.

## Exact candidate and visual judgment

[Private runnable gallery](http://m3p.tailec2dc.ts.net:8773/index.html) compares the unchanged baseline on the left with the candidate on the right. It includes loaded, nearly spent, curved/pressure-varying and crossing-wet marks at **160 × 80** and **320 × 160** pixels. Every scene is 100 × 50 mm. Current-lighting, flat-color and wet-height PNGs are included. [Deliverable hashes](SHA256SUMS) identify the exact images, sources and receipts. Source: [small study](../../crates/paint/examples/bristle_folds.rs).

Saved review PNGs:

- [Current lighting, 320 px](review-lit-320.png)
- [Current lighting, 160 px](review-lit-160.png)
- [Flat color, 320 px](review-flat-320.png)
- [Wet height, 320 px](review-wet-height-320.png)

My aesthetic reading of these exact images: the candidate's broad mark has less uniform track density, short uneven crests and wider neighboring grooves. The curved mark makes the locally gathered tracks easiest to see. The nearly spent brush remains thin and streaky, not an impasto stamp. The wet crossing still mixes orange into blue and carries dirt into the separate blue continuation.

Limitations visible in the images: fine parallel tracks still dominate much of the stroke. The candidate does not create the reference's rounded paint-body appearance. Some bunches become overly tall local piles, particularly at the coarser resolution. This is a usable comparison candidate, not evidence of calibrated brush physics or a conclusion that it looks better. **Exact user visual approval remains pending before any default promotion.**

The gallery returned HTTP 200 through the full Tailscale hostname; the collaborative browser loaded all comparison PNGs. The server binds only to `100.82.115.34:8773`. Another machine's connectivity was not tested. The static server and its small worktree venv remain for review; the generated `target/` (including the test runner's nested feature-build directories) is removed after verification. The example is rebuilt by `reproduce.sh`; the static gallery does not need build binaries.

## Selection, compatibility and implementation

The explicit experiment path is:

```rust
canvas.drag_bristle_folds(&mut held, &gesture, clip);
```

Selection is **per gesture**, not a global switch or serialized brush setting. Ordinary `drag`, scheduled work, touches and knives obtain a `Surf` with no fold mode. The new method participates only with engine 4+ and a blunt brush wider than or equal to 6 mm with tool stiffness above 0.6. Engine defaults remain 5. Soft, narrow and pointed brushes and historical engines are unchanged by the opt-in call. See [contact implementation and its two contract tests](../../crates/paint/src/bristle.rs).

The candidate replaces the participating stroke's old stationary release bands with irregular, distance-correlated groups. A compact ridge/hollow pair occasionally changes contact reach and release. Neighboring hairs deflect toward the shared bunch/crest field before their existing swept capsules exchange paint. Deflection stays within the existing root bound; the scheduler footprint and path resampling are unchanged. Pressure still brings a hair down before variation is applied. Loaded-tip wicking, support relief, wet pickup, transport, ploughing, brush dirt and drying use the existing mechanics.

Every deposit is still debited from that hair's liquid reservoir. Release is positive and capped below the available liquid. Solvent ceilings retain paint that cannot be laid. No final-height noise, stamp raster, reference source or reference asset was added. The knife's mechanics, the lighting implementation, pigment mixing and material/checkpoint schemas were not edited. The [baseline source hashes](unchanged-source.json) confirm `canvas.rs`, `checkpoint.rs`, `wet.rs`, `surface.rs` and the knife implementation region are unchanged.

**Replay boundary:** easel logs do not expose this method. Existing logs and defaults keep their ordinary behavior. This is a Rust-program experiment; reproducing it requires the explicit calls and this source revision. Checkpoints retain the existing physical canvas state and engine number, not the caller's choice of its next operation. Continuation after loading a checkpoint must select the same candidate method explicitly. Promotion would require a separate approved engine/log integration, not silently replacing engine 5.

## Chosen values, not physical measurements

All new geometry constants are aesthetic estimates in the nominal full-width brush frame:

| Control | Chosen value |
|---|---:|
| Group transverse coordinate spacing | 1.1 mm |
| Group distance coherence | 7 mm |
| Position warp scales | 3 mm across, 11 mm along |
| Fold opportunity interval | 14 mm |
| Fold interval activation threshold | hash > 0.35 |
| Fold ridge half-span | 0.9–1.7 mm |
| Adjacent hollow center offset | 1.3 × half-span |
| Fold longitudinal envelope support | ±3 mm, with ±1 mm plateau |
| Center wander | up to ±0.5 mm on a 9 mm coordinate |
| Tip-deflection bound | ±0.65 mm, tapered to zero at brush edges |
| Release multiplier clamp | 0.15–2.5, followed by 95% liquid cap |

Pressure compresses these brush-frame coordinates through the existing footprint model. A brush seed combined with its stroke ID determines the fields; resampling-step index and pixel coordinates do not seed them. Fields reset between separately selected gestures. This does not model persistent bundle deformation across gestures. Exact formulas: `FoldContact`, `fold_noise` and `fold_strength` in the contact implementation.

The study uses tool widths 14 mm and 11 mm, paint stiffness 0.95, linen crown 12 µm, brush seeds 19/71 and support seed 23. Nearly spent means wiping the initial 0.9 load to 4% of that initial amount, not 4% of a nominal full load. The lamp is fixed at azimuth 315°, elevation 30° and strength 0.9; neither lighting adaptation 1 nor material-highlight adaptation 2 is implemented here. Wet-height PNGs map 0–250 µm into linear grayscale before ordinary sRGB encoding and clip taller peaks to white.

## Measured results

[Paint budgets](metrics.csv) use coats × square canvas units, the existing `wet_total`/`Held::carried` units. Each row includes initial paint, final canvas paint, paint left in brushes, coverage above 0.01 coat and peak wet height.

- Maximum absolute relative error in canvas + carried paint: **7.3 × 10⁻⁸**, from the rounded CSV. The chosen assertion limit is 2 × 10⁻⁵. New tests also balance solvent and require finite nonnegative reservoirs at 64, 160 and 320 px, with thinner 0 and 0.5. Pickup is checked by orange/red entering previously blue bristles, not by requiring net brush volume to rise: a brush can pick up paint while releasing more than it gains.
- [48-hour wait measurements](drying.csv): dry-film + wet-paint accounting error is at most **4 × 10⁻⁸**, against a chosen 10⁻⁴ assertion bound. Nearly spent marks dry almost completely while the much thicker loaded marks remain substantially wet. This is simulated behavior, not measured drying physics.
- Repeat all canvas state fields and brush debug state: exact. Views leave state unchanged. Checkpoint continuation reproduces exact saved-state bytes, flat view and brush state. A loaded-stroke crop with 150 units of context has **zero** measured interior wet-volume difference. Receipt: [study checks](checks.txt). That does not establish exact crops without sufficient support/paint context; off-window contact retains the engine's ghost-contact approximation, with the candidate's release gain included.
- One versus four Rayon threads: all **48 scene PNGs**, all 16 budget rows and all 16 drying rows match exactly, excluding elapsed times. Receipt: [thread proof](thread-proof.json).
- Baseline production brush source restored from the exact baseline commit, in this worktree only: **24 ordinary-drag PNGs and eight numeric budget rows match exactly**. The helper restores candidate source even on failure and rebuilds its binary in the same `target/`. Receipt: [baseline proof](baseline-proof.json); procedure: [baseline comparison helper](compare_baseline.py).

The candidate releases more of the finite initial load than the baseline; it does not merely rearrange a matched deposited mass:

| Scene | Candidate canvas paint increase at 320 px | Candidate canvas paint drift, 160 → 320 px |
|---|---:|---:|
| Loaded | 23.40% | 2.91% |
| Nearly spent | 27.69% | 3.70% |
| Curved/pressure | 23.66% | 6.67% |
| Wet crossing | 20.11% | 3.37% |

Those percentages are calculated from `metrics.csv`. In the loaded candidate, peak wet height is about **958 µm at 160 px** and **707 µm at 320 px**. Millimeter-based control coordinates do not eliminate the engine's resolution-dependent sampling/contact behavior. Images at two resolutions are supplied; physical accuracy and resolution-invariant peak height are **not** established.

## Verification and remaining gate limits

[New candidate test receipt](candidate-tests.json): two tests passed, zero failed, zero ignored in 0.19 s; the locked small-check/render batch took 1.9 s. Existing tests and protected expected outputs were not weakened or edited.

The initial unrestricted `scripts/test` attempt was **UNFINISHED**, not passed: [fast receipt](fast-tests.json). Builds took 145.8 s, checks 117.4 s, total 264.0 s. `cargo-test` timed out at its unchanged 60 s limit before the batched script published test counts. It was not blindly retried or given more time.

Other fast failures:

- `baseline-state`: body, overlap and pickup matched; the rag case differed. The exact baseline source reproduces the same rag wet-volume differences. [Baseline contract receipt](baseline-contracts.json).
- `thinner-quick`: check 2 fails on the rag scene (including `thinner=0`) with the same wet-volume differences as `baseline-state`; only five of six PNGs match its historical reference. The expanded pigment-table assertion fails with the exact baseline too. Its runner also requests `c17_a_wait_split_on_the_minute_grid_is_exact_and_off_it_within_1e4`, but the current test is named `c17_a_wait_split_on_the_grid_is_exact_and_off_it_is_bounded`. The missing name is not counted as a pass. The runner reported 21 of 24 required tests passed. No runner or golden was changed to hide these failures. [Saved failure excerpts](fast-failure-excerpts.txt) include all three thinner-quick failures; the rag and pigment failures are reproduced against baseline source, while the missing test name is verified in the unchanged source.

The initial selected `scripts/test --all` invocation ran **zero checks**: its both-phase runner validates all `--only` names separately in each phase and rejects check names inside the build phase. Receipt: [selection failure](all-selection-failure.json). The experiment helper splits the original build and check phases without changing the runner.

The final `scripts/test --all` selection and its result are recorded by [applicable-check commands](applicable_checks.sh) and the [all-build receipt](all-build.json) / [all-check receipt](all-check.json). All three builds passed (126.4 s of build time, 126.8 s wall time). **12 of 14 selected check steps passed; the selection failed**, in 171.7 s wall time (420.8 s summed check time because group `w` runs concurrently). Combined phase wall time was 298.5 s. **This is a partial selection, not the complete gate.**

Final selected-check results:

| Step | Result | Seconds |
|---|---|---:|
| image-tolerance | passed | 0.76 |
| old-logs | passed | 24.76 |
| baseline-package | passed | 2.87 |
| baseline-state | FAILED | 1.26 |
| lockrun | passed | 22.59 |
| box-features | FAILED | 118.67 |
| test-runner | passed | 74.4 |
| safeguards | passed | 106.44 |
| runner-round24 | passed | 34.81 |
| studio-viewer | passed | 22.35 |
| replay-clip | passed | 5.55 |
| box-tubes | passed | 0.92 |
| studio-names | passed | 1.91 |
| peek | passed | 3.55 |

The final failures are `baseline-state` (the same rag differences described above) and `box-features`: its replay-box test expects nine box names, but the current catalog includes a tenth, `every`. [Exact box failure](logs/all/box-features.log), [unchanged test](../../crates/easel/tests/boxes.rs#L204) and [baseline source equality](unchanged-source.json) show the mismatch is outside the edited brush path; this box test was not rerun against a separately built baseline. Its 21 successful configuration markers do not make the overall step pass.

The receipts retain every original step command and timeout, plus sanitized logs. Original log hashes are preserved separately from hashes of the redacted deliverables. The following original all-suite steps are not certified:

- `cargo-test`: previous 60 s timeout; its default selection also contains live-width simulations.
- `slow-tests-by-name`: 2400 px simulations.
- `slow-thin-blend`: 3200 px simulation/crops.
- `thinner-all`: includes the 2400 px card and larger studies, plus the already observed runner/golden failures.
- `painter-build-inness`, `smoke-replay-without-finish`, `replay-env` and `check-live`: fixture sessions default to live-width painting rather than this task's 64–320 px scene limit. Box-feature compile/table checks are selected separately.

No whole painting or reference meadow was replayed. No exact-commit fresh-worktree gate receipt was created: the task permits only this existing worktree, and `scripts/test --all --candidate` creates another worktree.

## Reproduction commands

Commands are run from `~/src/a/claude-paint-oil-bristle-folds`; long operations are launched with `job_run`. No alternate target directory is used. Direct candidate checks use 60 s limits and builds use 600 s limits. `scripts/test` uses its unchanged list limits, including 120 s for `test-runner` and 180 s for `safeguards`; no listed timeout was increased.

```sh
bash notes/oil-bristle-folds/reproduce.sh

uv run --no-project scripts/lockrun --timeout 600 -- \
  uv run --no-project python notes/oil-bristle-folds/compare_baseline.py --contracts

uv run --no-project scripts/lockrun --timeout 600 -- \
  cargo test -p paint --lib --no-run
uv run --no-project scripts/lockrun --timeout 60 -- \
  cargo test -p paint --lib fold_candidate_tests -- --nocapture

uv run --no-project scripts/test --summary notes/oil-bristle-folds/fast-tests.local.json \
  --logs target/folds-fast-logs
bash notes/oil-bristle-folds/applicable_checks.sh
```

The first fast command is the recorded unsuccessful full attempt, not a recommendation to rerun its slow selection. `reproduce.sh` creates a worktree venv if absent, builds the example through a 600 s lock and runs the 160/320 px study plus thread comparison through a 60 s lock. The baseline helper temporarily changes only this worktree's brush implementation and example; it must run alone under the lock, not while tests are active.

Recorded private gallery command, retained with `job_run(service=true, wake_on_output="Serving HTTP")`:

```sh
PYTHONUNBUFFERED=1 uv run --no-project python -m http.server 8773 \
  --bind 100.82.115.34 --directory notes/oil-bristle-folds
```

Not done yet: user visual approval, complete repository gate and any approved default/replay integration. No push, PR, merge or configuration change was made. No other agents were spawned; implementation and review were performed locally.
