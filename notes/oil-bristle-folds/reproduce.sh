#!/usr/bin/env bash
# Reproduce the exact small visual study, not a painting replay.
set -euo pipefail
cd "$(dirname "$0")/../.."
[ -d .venv ] || uv venv .venv
uv run --no-project scripts/lockrun --timeout 600 --owner oil-bristle-folds-build -- \
  cargo build --profile iter -p paint --example bristle_folds
uv run --no-project scripts/lockrun --timeout 60 --owner oil-bristle-folds-study -- \
  sh -c 'target/iter/examples/bristle_folds notes/oil-bristle-folds > notes/oil-bristle-folds/metrics.csv 2> notes/oil-bristle-folds/checks.txt && uv run --no-project python notes/oil-bristle-folds/check_threads.py'
