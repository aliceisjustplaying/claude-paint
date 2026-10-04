#!/usr/bin/env bash
# Replay the scenes with a release easel that has `run --dump-state` and
# compare its state after every chunk with the baseline, field by field.
#
#   compare_build.sh <easel> [state_compare.py flags, e.g. --added-zero]
#
# Fails (exit 1) if any field the baseline has differs, bit for bit, or a
# chunk is missing. Fields a newer engine adds are listed; --added-zero
# requires them to be zero (thinner check 2: no thinner, no solvent). Also
# reports, without failing, whether the PNGs and the state digests equal
# af49348's (a new checkpoint format changes the canvas= digest by itself).
# Run as one heavy job: scripts/lockrun --timeout 60 -- .../compare_build.sh ...
set -euo pipefail
[ $# -ge 1 ] || { echo "usage: compare_build.sh <easel> [state_compare flags]" >&2; exit 2; }
easel=$1; shift
here=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d "${TMPDIR:-/tmp}/baseline-compare.XXXXXX")
trap 'rm -rf "$work"' EXIT
"$here/tools/run_scenes.sh" "$easel" "$work" --dump >/dev/null
failed=0
for d in "$here"/state/*/; do
  s=$(basename "$d")
  if python3 "$here/tools/state_compare.py" "$d" "$work/state/$s" --quiet "$@" >"$work/$s.cmp"; then
    echo "$s: state equal on every baseline field ($(tail -1 "$work/$s.cmp" | sed 's/state_compare: //'))"
  else
    failed=1
    echo "$s: STATE DIFFERS"; sed 's/^/  /' "$work/$s.cmp"
  fi
  cmp -s "$here/ref/png/$s.png" "$work/png/$s.png" && png=same || png=DIFFERENT
  cmp -s <(sed 's/ secs=[^ ]*//' "$here/ref/digest/$s.txt") <(sed 's/ secs=[^ ]*//' "$work/digest/$s.txt") && dg=same || dg=different
  cmp -s "$here/ref/out/$s.txt" "$work/out/$s.txt" && out=same || out=DIFFERENT
  echo "  png $png as af49348's; printed output $out; state digests $dg"
done
[ $failed = 0 ] && echo "compare_build: EQUAL to the af49348 baseline on every field it has" || { echo "compare_build: DIFFERENT from the af49348 baseline" >&2; exit 1; }
