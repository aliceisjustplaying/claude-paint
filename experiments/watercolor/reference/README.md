# Watercolor reference pilot

This is an offline comparison harness. It adds no solver, renderer or Lua API changes. The Rust runner is the accepted `f75ed9aa57d7e9a5050914221dec777a1cf50cbe` baseline. The reference is MIT Wet Edge at `84aa409d5032a6fc1b613665b4626be1cf410eee`.

`marks.json` defines six marks in seven setups. Each is on 19.2 × 12.8 mm cold-press paper, seed 71. Ours keeps the accepted 0.05 mm cells. Wet Edge keeps its native 0.2 mm cells. The harness intercepts only the two sheet-dimension constants in upstream `main.js` and `minds.js` to run a 96 × 64 sheet in software. It does not copy or modify upstream shaders or spectra.

Ours uses 24 capsule segments per stroke, widths following the same pressure envelope as Wet Edge, finite loads proportional to swept area and 1/60-second waits between segments. Its public API cannot change a wet brush's width continuously or assign hand time to a stroke. Wet Edge uses the moving brush API. Its Wash tool calls a deterministic headless adapter for `sim.path`; its row and cut-in decisions remain upstream's. The flat wash explicitly disables upstream's optional instantaneous prewet.

Pigment units are not comparable. Ours supplies 0.0015 areal pigment and a nominal 0.12 mm water dose per swept area. Wet Edge uses paint strength 0.2 and its round brush's normal 0.3 mm water level. These are visible loaded washes, not calibrated equal concentrations or colors. A preliminary 0.015-strength Wet Edge loading check was almost invisible and was discarded before the final reference pass. No coefficient was tuned to make an image pass.

Stage checkpoints use center averages. Ours uses a 1.2 × 1.2 mm square from its automatic checkpoint. Wet Edge uses a 0.6 mm-radius disk through `sense`. Common descriptive thresholds are 0.04 mm standing water for shiny, 0.004 for satin and pore depths of 0.05/0.0125 for moist/damp/dry. Ours has no official stage API; these labels are harness diagnostics. Each receipt also records whole-sheet water maxima. Ours ends with its exact dry guard. Wet Edge ends with its native dry skip and gum-setting wait. The two clocks are not real-paper seconds with the same physical meaning.

## Reproduce

Use fresh output directories. Keep binaries and state outside Drive.

```sh
export WORK="$HOME/.capy/work/watercolor-reference"
export CARGO_TARGET_DIR="$WORK/target"
cargo build --release --locked --manifest-path experiments/watercolor/Cargo.toml
python3 experiments/watercolor/reference/ours.py \
  "$CARGO_TARGET_DIR/release/watercolor-studio" "$WORK/output/ours"
git clone https://github.com/lambentLogic/wet-edge.git "$WORK/wet-edge"
git -C "$WORK/wet-edge" checkout 84aa409d5032a6fc1b613665b4626be1cf410eee
```

Start `python3 tools/serve.py 8765` in the upstream checkout, in a separate terminal. Then:

```sh
cd experiments/watercolor/reference
npm ci
CHROME_PATH=/usr/local/bin/chromium node wet-edge.mjs "$WORK/output/wet-edge"
python3 review.py "$WORK/output" "$WORK/watercolor-reference.html"
```

Chromium needs WebGPU, SwiftShader/Vulkan and the GPU watchdog disabled for this software-GPU workload. The script supplies those flags. A normal hardware adapter may not need them. There are no CPU-transliterated outputs in this pilot.

The review page is self-contained with inline images, CSS and JavaScript. Gallery is the default for ours; Color is available. Feedback has pass/fail for each engine, notes, local browser persistence and a copy-to-clipboard action with a selected-text fallback.
