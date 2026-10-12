# Watercolor Lab GPU app

Pinned upstream: `uxpreview/watercolor-lab` at `a19bfa2a026856a98ceb400a8eec41f091f04a11`, copyright 2026 Ryan McCarty, MIT. See `LICENSE` and `ASSETS.md`.

The original UI runs the Rust `lab-core` wgpu backend through WASM and WebGPU. Simulation and display stay on the GPU. The desktop grid is 1056×704, the phone grid is 704×1056 and the original clock runs three substeps per frame. Existing `?sim=WxH&seed=N` capture overrides remain available.

The Engine A/B switch is removed because a canvas cannot hold both WebGL and WebGPU contexts. Compare with the original website at https://watercolor.ryankm.com/. Salt remains hidden because its sine hash is backend-sensitive. Brush controls, backlight, X-ray, three-level undo and saved dried sheets remain. Probes and saved-sheet readbacks are asynchronous.

## Build

Install Node 20+, Rust with `rustup target add wasm32-unknown-unknown` and `cargo install wasm-bindgen-cli --locked --version 0.2.104`.

```sh
npm ci
CARGO_TARGET_DIR="$HOME/.capy/work/wlgpu/target" bash build.sh
```

`lab-wasm` depends on the same `lab-core` crate as the native Metal backend. The shader formulas are copied from the pinned GLSL passes, with explicit bindings and WebGPU coordinate conversions. No WASM threads or CPU field uploads are used while painting.

## Serve locally

```sh
python3 -m http.server 8000 --bind 127.0.0.1 --directory dist
```

Open `http://127.0.0.1:8000` in a browser with WebGPU support. This is a static local build, not a public deployment.
