#!/usr/bin/env bash
# Explicit partial --all selection. Does NOT claim the complete gate passed.
# The omitted simulation steps exceed the task's 320 px cap. cargo-test
# already timed out at its original 60 s limit; it is not blindly retried.
set -euo pipefail
cd "$(dirname "$0")/../.."
# The existing both-phase runner validates every --only name inside each phase.
# Split phases to avoid passing check names to its build-phase validation.
uv run --no-project scripts/test --all --phase build \
  --summary notes/oil-bristle-folds/all-build.local.json --logs target/folds-all-logs
uv run --no-project scripts/test --all --phase check \
  --summary notes/oil-bristle-folds/all-check.local.json --logs target/folds-all-logs \
  --only image-tolerance --only old-logs --only baseline-package --only baseline-state \
  --only lockrun --only box-features --only test-runner --only safeguards \
  --only runner-round24 --only studio-viewer --only replay-clip --only box-tubes \
  --only studio-names --only peek
