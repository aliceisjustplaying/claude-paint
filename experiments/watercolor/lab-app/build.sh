#!/usr/bin/env bash
# MIT License. Copyright (c) 2026 Ryan McCarty.
# Build adapter for Watercolor Lab at a19bfa2a026856a98ceb400a8eec41f091f04a11. See LICENSE.
set -euo pipefail
cd "$(dirname "$0")"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.capy/work/wlgpu/target}"
cargo build --manifest-path ../lab-wasm/Cargo.toml --release --target wasm32-unknown-unknown
wasm-bindgen "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/watercolor_lab_wasm.wasm" --target web --out-dir wasm --out-name watercolor_lab_wasm
npm run build
cp LICENSE ASSETS.md dist/
