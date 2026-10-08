#!/usr/bin/env bash
# Worktree-only tiny sweep; the original gallery is read-only evidence.
set -euo pipefail
cd "$(dirname "$0")/../../.."
[ -d .venv ] || uv --offline venv .venv
out=notes/oil-bristle-folds/release
uv --offline run --no-project --python .venv/bin/python scripts/lockrun --timeout 600 --owner folds-release-build --log "$out/build.log" -- \
  sh -c 'cargo build --profile iter -p paint --example bristle_folds && cargo test -p paint --lib --no-run'
uv --offline run --no-project --python .venv/bin/python scripts/lockrun --timeout 60 --owner folds-release-tests --log "$out/tests.log" -- \
  cargo test -p paint --lib fold_candidate_tests -- --nocapture
uv --offline run --no-project --python .venv/bin/python scripts/lockrun --timeout 60 --owner folds-release-render --log "$out/render.log" -- \
  sh -c 'target/iter/examples/bristle_folds notes/oil-bristle-folds/release --release-sweep > notes/oil-bristle-folds/release/metrics.csv 2> notes/oil-bristle-folds/release/checks.txt && uv --offline run --no-project --python .venv/bin/python python notes/oil-bristle-folds/release/check_sweep.py'
