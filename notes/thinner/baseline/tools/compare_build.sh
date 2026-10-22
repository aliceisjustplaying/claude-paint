#!/usr/bin/env bash
# Replay the scenes with a release easel that has `run --dump-state` and
# compare its state after every chunk with the baseline, field by field.
#
#   compare_build.sh <easel> [state_compare.py flags, e.g. --added-zero]
#
# Fails (exit 1) if any field the baseline has exceeds numeric tolerance, or a
# chunk is missing. Fields a newer engine adds are listed; --added-zero
# requires them to be zero (thinner check 2: no thinner, no solvent). Also
# compares PNGs within image tolerance and reports state digests without
# failing on them (a new checkpoint format changes the canvas= digest by itself).
# Run as one heavy job: scripts/lockrun --timeout 60 -- .../compare_build.sh ...
set -euo pipefail
[ $# -ge 1 ] || { echo "usage: compare_build.sh <easel> [state_compare flags]" >&2; exit 2; }
easel=$1; shift
here=$(cd "$(dirname "$0")/.." && pwd)
repo=$(cd "$here/../../.." && pwd)
work=$(mktemp -d "${TMPDIR:-/tmp}/baseline-compare.XXXXXX")
trap 'rm -rf "$work"' EXIT
"$here/tools/run_scenes.sh" "$easel" "$work" --dump >/dev/null
failed=0
scenes=0
state_result='state within tolerance on every baseline field'
for d in "$here"/state/*/; do
  s=$(basename "$d")
  state_failed=0
  image_failed=0
  if uv run --no-project python "$here/tools/state_compare.py" "$d" "$work/state/$s" --quiet "$@" >"$work/$s.cmp"; then
    echo "$s: $state_result ($(tail -1 "$work/$s.cmp" | sed 's/state_compare: //'))"
    cat "$work/$s.cmp"
  else
    state_failed=1
    echo "$s: STATE DIFFERS"; sed 's/^/  /' "$work/$s.cmp"
  fi
  if uv run --script "$repo/scripts/compare_images.py" "$here/ref/png/$s.png" "$work/png/$s.png" --expected-platform "$(cat "$here/ref/png/platform.txt" 2>/dev/null || echo unknown)" --actual-platform current >"$work/$s.image-report"; then
    png=same
  else
    png=DIFFERENT
    image_failed=1
  fi
  cat "$work/$s.image-report"
  if [ "$state_failed" = 1 ] || [ "$image_failed" = 1 ]; then
    failed=1
  fi
  cmp -s <(sed 's/ secs=[^ ]*//' "$here/ref/digest/$s.txt") <(sed 's/ secs=[^ ]*//' "$work/digest/$s.txt") && dg=same || dg=different
  cmp -s "$here/ref/out/$s.txt" "$work/out/$s.txt" && out=same || out=DIFFERENT
  echo "  png $png as recorded baseline; printed output $out; state digests $dg"
  scenes=$((scenes + 1))
done
[ "$scenes" = 6 ] || { echo "compare_build: expected all six scenes" >&2; exit 1; }
[ "$failed" = 0 ] || { echo "compare_build: DIFFERENT from the recorded baseline" >&2; exit 1; }
echo "compare_build: all baseline fields meet their state and image criteria"
