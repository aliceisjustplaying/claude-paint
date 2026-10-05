#!/usr/bin/env bash
# The engine against the small before-change results (notes/thinner/baseline):
# every state field af49348 had, after every chunk of the six scenes, equal
# bit for bit; fields a newer engine adds zero (--added-zero); and every
# scene's PNG within the cross-build image tolerance. (The thinner's check 2: "no
# thinner" leaves engine 3 as it was.)
#
#   scripts/tests/baseline_state.sh [<release easel>]   (default $CARGO_TARGET_DIR or target, /release/easel)
set -euo pipefail
repo=$(cd "$(dirname "$0")/../.." && pwd)
easel=${1:-${CARGO_TARGET_DIR:-$repo/target}/release/easel}
baseline=$repo/notes/thinner/baseline
work=$(mktemp -d "${TMPDIR:-/tmp}/baseline-state.XXXXXX")
trap 'rm -rf "$work"' EXIT
"$baseline/tools/run_scenes.sh" "$easel" "$work" --dump >/dev/null
scenes=0
for dir in "$baseline"/state/*/; do
  name=$(basename "$dir")
  uv run --no-project python "$baseline/tools/state_compare.py" "$dir" "$work/state/$name" --quiet --added-zero
  echo "$name: state equal on every baseline field"
  uv run --script "$repo/scripts/compare_images.py" "$baseline/ref/png/$name.png" "$work/png/$name.png"
  scenes=$((scenes + 1))
done
[ "$scenes" -gt 0 ] || { echo "baseline_state: no scenes" >&2; exit 1; }
echo "baseline_state: all $scenes scenes equal af49348 in state; PNGs meet the cross-build tolerance"
