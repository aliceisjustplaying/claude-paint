# Watercolor Lab comparison app

Pinned upstream: `uxpreview/watercolor-lab` at `a19bfa2a026856a98ceb400a8eec41f091f04a11`, copyright 2026 Ryan McCarty, MIT. See `LICENSE` and `ASSETS.md`.

This is the pinned UI with a CPU Rust/WASM simulation adapter and a blind Engine A / Engine B switch. The original GPU simulator remains at `src/engine/simulation.ts`; browser drivers retain `window.__wash`. All original controls have a ported counterpart. No scenes, painters or new tests are included.

## Build

Install Node 20+, Rust with `rustup target add wasm32-unknown-unknown` and `cargo install wasm-bindgen-cli --locked --version 0.2.104`. Then:

```sh
npm ci
CARGO_TARGET_DIR="$HOME/.capy/work/watercolor-lab-port/target" bash build.sh
```

`lab-wasm` is a member of the parent watercolor workspace and depends on `../lab-core`. It uses no WASM threads. The shared target path above is deliberate for this approved port.

## Serve locally

```sh
python3 -m http.server 8000 --bind 127.0.0.1 --directory dist
```

Open `http://127.0.0.1:8000`. The build is static, below 1 MB and not deployed publicly. A built handoff can be served by substituting its directory for `dist`; source tools are not needed to serve it.

The default simulation grid is 132×88 on desktop and 88×132 on phone, with the upstream physical sheet and brush footprint unchanged. Both engines use the same grid and display resolution. Existing `?sim=WxH&seed=N` overrides remain available for the parity/capture harness. The UI discloses the coarse grid, fixed three-substep frame clock and speed differences. Changing engines clears wet and dried paint but keeps paper kind/seed, paint, brush and display settings; wet-state transfer is not implemented. Reveal prints the random per-load mapping only to the console.

Simulation passes run on the CPU in WASM. State fields upload to the unchanged upstream WebGL display shader, so both engines retain the exact same display-resolution sampling, optical shading, backlight and X-ray views. This avoids running costly full-screen optics on the CPU while keeping the solver mechanical.
