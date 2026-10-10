# Brush simulation 1

Experimental finite-reservoir brush models. Default builds and new sessions use engine 5. Build easel with `--features brush-sim-1` to enable engines 6–8, then set `EASEL_BRUSH_SIM=lean`, `body` or `wild` when opening a new session. `off` or an absent variable retains engine 5. Existing sessions always use their logged engine. The engine number is also stored in checkpoints; builds without the feature refuse experimental logs and checkpoints.

| Setting | Engine | Release multiplier | Entropy |
|---|---:|---:|---:|
| Lean | 6 | 0.85 | 1.0 |
| Body | 7 | 1.05 | 1.0 |
| Wild | 8 | 0.95 | 2.8 |

Each dip redistributes the historical amount between finite hair reservoirs, preserving full and partial dip totals. Its irregular bundles vary with brush seed and dip number. Contact groups wander in physical millimeters and change with stroke identity, pressure, paint body and depletion. A broad intermittent ridge sits beside a hollow. Neighboring hairs deflect toward these groups before ordinary pickup, mixing, release and ploughing happen. No texture is added to the renderer.

Bundle contact averages the dry surface across a 0.45 mm radius before sensing its local relief. The underlying weave remains in the actual material and lighting, while its individual raster peaks no longer gate dry scumble into a checkerboard. A stroke-seeded transverse contact threshold supplies larger interrupted tracks instead of per-pixel grain.

The whole geometric hair contact supplies the deposition denominator, including contact beyond a canvas edge. Missing contact therefore releases less paint. Rejected deposits remain in the hair. A shared per-stroke film-splitting allowance follows the tool's `lay`, bounded between 240 and 700 micrometers. Ploughing accepts only transfers into a neighboring wet film up to 60 micrometers above the source film; rejected paint stays at the source. These are uncalibrated material-transfer limits, not display-height clamps. They do not cap cumulative paint thickness or promise that an arbitrarily layered painting cannot become tall.

The feature reaches ordinary strokes, scheduled `work` and touches/stipple. It changes neither gallery lighting nor the painter's default look. Engines 6–8 are an experimental replay contract: keep this revision with the corresponding logs until an iteration is selected and its parameters are frozen.

## Reproduce the small study

```sh
export RAYON_NUM_THREADS=2 CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=1
nice -n 15 cargo test --release -p paint --features brush-sim-1 --test brush_sim_1 -j 2
nice -n 15 cargo run --release -p paint --features brush-sim-1 --example brush_sim_1 -j 2 -- /path/to/study
nice -n 15 uv run --no-project --with pillow python notes/brush-sim-1/compare.py /path/to/study
```

The six scenes are loaded, nearly spent, curved with varying pressure, wet crossing with a contaminated continuation, scumble over dry paint and a layered painted passage. Every variant receives the same gestures and total dip budget. Scumble's dry base is prepared with the selected model, so the result includes that model's dry surface. Every color/gallery pair comes from one final canvas state. The sheet extracts identical native 1:1 crops, with full native scene comparisons alongside it.

`metrics.csv` contains eight rotated-order timing repetitions per scene and setting. Time per chunk is the draw portion of each synthetic scene, excluding support preparation, rendering and export. `summary.csv` uses median timings, net canvas paint above the scene's starting state and total final physical surface height, including ground and scumble's dry base. Paint is in coats times square canvas units, not grams. The wet crossing's pickup/release and residual brush paint are included in the conservation ledger. Busy-device timings are descriptive and are not an isolated benchmark.

Next knobs: irregular group spacing and coherence lengths, the distribution of per-dip loads, release versus depletion and pressure, intermittent ridge span and frequency, plough leveling allowance and the film-splitting budget. Tune geometry with the high-entropy view beside the lean view, then adjust release to keep the desired paint budget.
