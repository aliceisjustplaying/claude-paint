# Watercolor reference pilot

This is an offline comparison harness. It adds no solver, renderer or Lua API changes. The Rust runner is the accepted `f75ed9aa57d7e9a5050914221dec777a1cf50cbe` baseline. The reference is MIT Wet Edge at `84aa409d5032a6fc1b613665b4626be1cf410eee`.

`marks.json` defines six marks in seven setups. Each is on 19.2 × 12.8 mm cold-press paper, seed 71. Ours keeps the accepted 0.05 mm cells. Wet Edge keeps its native 0.2 mm cells. The harness intercepts only three sheet-dimension constants in upstream `main.js` and `minds.js` to run a 96 × 64 sheet. It does not copy or modify upstream shaders or spectra.

Ours uses 24 capsule segments per stroke, widths following the same pressure envelope as Wet Edge, finite loads proportional to swept area and 1/60-second waits between segments. Its public API cannot change a wet brush's width continuously or assign hand time to a stroke. Wet Edge uses the moving brush API. Its Wash tool calls a deterministic headless adapter for `sim.path`; its row and cut-in decisions remain upstream's. The flat wash explicitly disables upstream's optional instantaneous prewet.

Pigment units are not comparable. Ours supplies 0.0015 areal pigment and a nominal 0.12 mm water dose per swept area. Wet Edge uses paint strength 0.2 and its round brush's normal 0.3 mm water level. These are visible loaded washes, not calibrated equal concentrations or colors. Ours' generic warm earth is paired with upstream Transparent Red Oxide (PR101). Upstream has no pan named Burnt Sienna. A preliminary 0.015-strength Wet Edge loading check was almost invisible and was discarded before the final reference pass. No coefficient was tuned to make an image pass.

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

On Linux the script defaults to Chromium with SwiftShader/Vulkan and the GPU watchdog disabled. Software WebGPU works but the native gum-setting wait made one setup take 29 minutes. The delivered reference instead ran in system Chrome on an Apple M1 Pro GPU (`apple/metal-3`), arranged by the Captain with Alice's approval. On macOS the script selects system Chrome and hardware WebGPU automatically; `GPU_MODE=hardware` selects those flags elsewhere. `CASE_IDS=wash` captures only that setup. There are no CPU-transliterated outputs in this pilot.

The initial six successful Metal setups use this harness at `79bf4c9` with native browser flags and PR101 substituted. The original Wash capture was a no-op because the adapter missed `washAround`'s local width constant. Only that setup was recaptured with the harness correction, with the same paper, loads, tool and stage settings. The updated script checks pigment names before running and refuses a mark that leaves no pigment. The native gum-setting wait remains intact.

The review page is self-contained with inline images, CSS and JavaScript. Gallery is the default for ours; Color is available. Feedback has pass/fail for each engine, notes, local browser persistence and a copy-to-clipboard action with a selected-text fallback.
