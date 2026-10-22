#!/usr/bin/env bash
# The engine against the recorded results (notes/thinner/baseline):
# every recorded state field, after every chunk of the six scenes, within
# numeric float limits on every platform;
# fields a newer engine adds zero (--added-zero); and every
# scene's PNG under the image tolerance policy. (The thinner's check 2: "no
# thinner" matches the recorded engine-3 state.)
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
failed=0
compare_flags=()
state_result="state within tolerance on every baseline field"
state_summary="within tolerance on every baseline field"
for dir in "$baseline"/state/*/; do
  name=$(basename "$dir")
  state_failed=0
  image_failed=0
  if uv run --no-project python "$baseline/tools/state_compare.py" "$dir" "$work/state/$name" --quiet --added-zero "${compare_flags[@]}"; then
    echo "$name: $state_result"
  else
    state_failed=1
  fi
  if ! uv run --script "$repo/scripts/compare_images.py" "$baseline/ref/png/$name.png" "$work/png/$name.png" --expected-platform "$(cat "$baseline/ref/png/platform.txt" 2>/dev/null || echo unknown)" --actual-platform current >"$work/$name.image-report"; then
    image_failed=1
  fi
  cat "$work/$name.image-report"
  if [ "$state_failed" = 1 ] || [ "$image_failed" = 1 ]; then
    failed=1
  fi
  scenes=$((scenes + 1))
done
[ "$scenes" = 6 ] || { echo "baseline_state: expected all six scenes" >&2; exit 1; }
[ "$failed" = 0 ] || { echo "baseline_state: FAILED (a different or unrelated regression)" >&2; exit 1; }
echo "baseline_state: all $scenes scenes $state_summary; PNGs meet the image tolerance"
