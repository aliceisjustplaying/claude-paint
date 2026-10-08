#!/usr/bin/env bash
# Rebuild the unchanged renderer in this worktree only, compare PNGs and
# checkpoint hashes then restore the candidate sources and executable.
set -euo pipefail
cd "$(dirname "$0")/../.."
work=$(mktemp -d)
cp crates/paint/src/canvas.rs "$work/canvas.rs"
cp crates/paint/examples/oil_lighting.rs "$work/oil_lighting.rs"
restore() {
  cp "$work/canvas.rs" crates/paint/src/canvas.rs
  cp "$work/oil_lighting.rs" crates/paint/examples/oil_lighting.rs
}
trap 'restore; rm -rf "$work"' EXIT
git show 8eeb9bec3ce146bcd15c37db447665391fb95bdd:crates/paint/src/canvas.rs > crates/paint/src/canvas.rs
sed 's/Canvas::seen_lit_body_candidate/Canvas::seen_lit/' "$work/oil_lighting.rs" > crates/paint/examples/oil_lighting.rs
scripts/lockrun --timeout 600 -- cargo build --release -p paint --example oil_lighting
scripts/lockrun --timeout 60 -- env RAYON_NUM_THREADS=4 target/release/examples/oil_lighting "$work/baseline" > "$work/baseline-run.txt"
{
  echo 'renderer baseline=8eeb9bec3ce146bcd15c37db447665391fb95bdd'
  for width in 160 320; do
    cmp "notes/oil-lighting/current-$width.png" "$work/baseline/current-$width.png"
    cmp "notes/oil-lighting/flat-$width.png" "$work/baseline/flat-$width.png"
    hash=$(shasum -a 256 < "$work/baseline/material-$width.ckpt" | cut -d' ' -f1)
    grep -q "^$hash  notes/oil-lighting/material-$width.ckpt$" notes/oil-lighting/SHA256SUMS
    echo "width=$width current_png_equal=true flat_png_equal=true material_sha256_equal=true sha256=$hash"
  done
  sed 's/view=candidate/view=baseline-alias/' "$work/baseline-run.txt"
} > notes/oil-lighting/baseline-proof.txt
restore
scripts/lockrun --timeout 600 -- cargo build --release -p paint --example oil_lighting
