# Brush simulation 2

This is a default-off experiment based on `capy/brush-sim-1`. Build with `--features brush-sim-2` and choose `EASEL_BRUSH_SIM=body2` for a new experimental session. An absent variable or `off` still uses engine 5. Existing logs and checkpoints keep their recorded engines. A build without this feature refuses engines 9–11. Iteration 1's engines 6–8 retain their deposition equations.

| Setting | Engine | Release | Entropy |
|---|---:|---:|---:|
| lean2 | 9 | 1.05 | 2.2 |
| body2 | 10 | 1.20 | 2.8 |
| wild2 | 11 | 1.25 | 4.0 |

`body2` is the selected experiment, not a production default. Travel-correlated hair groups vary their spacing and branch along the mark. Each held brush retains its cumulative physical travel and bend between strokes; dips retain the exact historical input budget while redistributing it among finite reservoirs. Pressure, body, depletion, motion direction and existing tack response affect contact and transfer. Loaded hairs wet the valleys; depleted groups skip them. Local pickup changes with bundle contact and travel, and carries pigment, properties, cure and solvent through the ordinary finite reservoirs. The shared splitting allowance stays bounded. There is no pigment recoloring or renderer change.

The travel-correlated `tempo` factor changes residence-dependent release and pickup. The gesture API has no physical velocity samples, so this is a deterministic quasistatic speed-variation proxy. It does not alter the hand-time clock or claim calibrated flow dynamics. Group convergence/divergence comes from an evolving displacement field, not explicit hair-to-hair adhesion. A shallow working layer and local rag reservoirs remain future schema work.

The inherited rollback regression exposed three transient session fields omitted from snapshots: chunk number, call count and RNG state. This branch restores them only in experimental sessions. Engines 1–5 retain their prior transaction behavior and replay bytes.

## Reproduce

```sh
export CARGO_TARGET_DIR=/path/to/private-target CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=2
cargo test -p paint --features brush-sim-2 --test brush_sim_1 --test brush_sim_2
cargo test -p easel --features brush-sim-2 --bin easel brush_sim
cargo run --profile test -p paint --features brush-sim-2 --example brush_sim_2 -- /path/to/equal-input
cargo run --profile test -p paint --features brush-sim-2 --example brush_sim_2 -- /path/to/matched-mass --matched
python3 notes/brush-sim-2/compare.py /path/to/equal-input
python3 notes/brush-sim-2/compare.py /path/to/matched-mass
```

The comparison needs Pillow. Four fixtures use 640 × 320 native pixels on a 160 × 80 mm fine-linen support. Crossing exactly repeats iteration 1's crossing. Dark is one 90%-loaded bone-black tube pass over a pale ground, with 5% medium. Wet crosses a fresh blue patch with a lightly loaded warm brush, then uses that brush for another mark. Spent uses a 7% load. Every color/gallery pair comes from the same final canvas. Gallery is `seen_lit(115,55,1)`.

Equal-input figures expose paint-budget changes. Matched-mass figures use 13 bisection steps to choose a common multiplier for all requested loads in each scene/variant. Initial ground is identical; net final canvas paint is matched to engine 5. This controls total deposited mass, not each pixel or each stroke's separate deposition/pickup. Load factors, residual brush inventory and conservation errors remain in `metrics.csv`. Timing excludes calibration, support preparation and rendering. Three rotated-order rounds must produce identical material statistics. `luminance.csv` measures unquantized linear Rec.709 Y in fixed pixel rectangles; gallery highlights are retained. `transport.csv` samples raw local linear color and B−R at the drag tail and following mark, not a pigment-fraction estimate.

The sheet generator preserves native pixels without sharpening or contrast changes. `alice-comparison.png` compares baseline, iteration 1 body and iteration 2 body2 with gallery/color side by side; `entropy-gallery.png` includes lean2 and the higher-entropy wild2. Full-scene sheets retain all five variants.

For iteration 3, prioritize a shallow partly unmixed working layer, genuinely local rag/solvent exchange and a stronger coupled bundle/body model. Keep the explicit edition boundary and compare matched mass under unchanged gallery light. Current dark recipes can already reach bone black's own value; mixtures containing white require honest finite-film previews, not a global contrast curve.
