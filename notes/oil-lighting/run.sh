#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
# Build separately from the ordinary render time limit. Both take lockrun.
scripts/lockrun --timeout 600 -- cargo build --release -p paint --example oil_lighting
scripts/lockrun --timeout 60 -- bash -c '
  RAYON_NUM_THREADS=4 target/release/examples/oil_lighting notes/oil-lighting | tee notes/oil-lighting/measurements.txt
  shasum -a 256 notes/oil-lighting/material-*.ckpt notes/oil-lighting/*.png > notes/oil-lighting/SHA256SUMS
  rm notes/oil-lighting/material-*.ckpt
'
