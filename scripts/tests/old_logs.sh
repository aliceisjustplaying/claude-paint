#!/usr/bin/env bash
# Old logs match their recorded current-mixer results: tiny cases in crates/easel/tests/old_logs/cases.tsv (a
# synthetic engine-1 log, the first chunks of two engine-1 studio logs and of the six
# legacy easel3/easel4 logs, a synthetic log of the legacy verbs past those chunks, the
# round 19 log at 320 px), replayed by a release easel at
# a small width. Each decoded PNG must meet the image tolerance against
# its golden; legacy_tiny also checks a same-binary repeat. State digests are diagnostic. References came
# from the generator recorded in crates/easel/tests/old_logs/golden/README.md.
# These replace the tests that replayed
# whole paintings (notes/speed/SKIPPED.md); no whole painting is replayed here.
#
#   scripts/tests/old_logs.sh [<release easel>]          (default $CARGO_TARGET_DIR or target, /release/easel)
#   scripts/tests/old_logs.sh <easel> --record <dir>     write explicitly authorized goldens
#
# About 5 s (four cases at a time). A heavy job: run it inside scripts/lockrun.
set -euo pipefail
repo=$(cd "$(dirname "$0")/../.." && pwd)
easel=${1:-${CARGO_TARGET_DIR:-$repo/target}/release/easel}
record=
[ "${2:-}" = --record ] && record=${3:?--record needs a directory}
cases=$repo/crates/easel/tests/old_logs
[ -x "$easel" ] || { echo "old_logs: no easel at $easel (cargo build --release -p easel)" >&2; exit 1; }
work=$(mktemp -d "${TMPDIR:-/tmp}/old-logs.XXXXXX")
trap 'rm -rf "$work"' EXIT
unset EASEL_BOX
grep -v '^#' "$cases/cases.tsv" | awk -F'\t' 'NF >= 3 { print $1 "\t" $2 "\t" $3 }' >"$work/list"
[ -s "$work/list" ] || { echo "old_logs: no cases" >&2; exit 1; }
one() {
  local name=$1 log=$2 width=$3
  if ! "$easel" run "$cases/$log" --width "$width" --out "$work/$name.png" --state-digest "$work/$name.dig" >"$work/$name.out" 2>"$work/$name.err"; then
    echo "FAILED to replay"; return
  fi
  { echo "png_sha256 $(shasum -a 256 <"$work/$name.png" | cut -d' ' -f1)"; sed 's/ secs=[^ ]*//' "$work/$name.dig"; } >"$work/$name.got"
  echo 'done'
}
export -f one
export easel cases work
cut -f1-3 "$work/list" | tr '\t' ' ' | xargs -P 4 -L 1 bash -c "one \"\$0\" \"\$1\" \"\$2\" >\"\$work/\$0.status\""
failed=0 n=0
while IFS=$'\t' read -r name log width; do
  n=$((n + 1))
  if [ "$(cat "$work/$name.status")" != 'done' ]; then
    echo "old_logs: $name: the replay failed:"; tail -5 "$work/$name.err" | sed 's/^/  /'; failed=1; continue
  fi
  if [ -n "$record" ]; then
    mkdir -p "$record"; cp "$work/$name.got" "$record/$name.txt"; cp "$work/$name.png" "$record/$name.png"; uv run --script "$repo/scripts/compare_images.py" --print-platform >"$record/platform.txt"; echo "old_logs: $name recorded"; continue
  fi
  want=$cases/golden/$name.txt
  if [ ! -f "$want" ]; then echo "old_logs: $name: no golden ($want)"; failed=1; continue; fi
  if [ "$name" = r19 ]; then
    IFS= read -r palette <"$work/$name.out"
    if [ "$palette" != "lead white, smalt, pale smalt, yellow ochre, red earth, vermilion, raw umber, bone black, cobalt blue, chrome yellow, Prussian blue, green earth, Rinmann's green, copper green" ]; then
      echo "old_logs: r19 default tubes differ"; failed=1
    fi
  fi
  if uv run --script "$repo/scripts/compare_images.py" "$cases/golden/$name.png" "$work/$name.png" --expected-platform "$(cat "$cases/golden/platform.txt" 2>/dev/null || echo unknown)" --actual-platform current; then
    echo "old_logs: $name ok ($log at $width px)"
    if ! cmp -s "$want" "$work/$name.got"; then
      echo "old_logs: $name cross-build digests differ (image is within tolerance)"
    fi
    if [ "$name" = legacy_tiny ]; then
      "$easel" run "$cases/$log" --width "$width" --out "$work/$name.again.png" --state-digest "$work/$name.again.dig" >"$work/$name.again.out" 2>"$work/$name.again.err"
      if ! uv run --script "$repo/scripts/compare_images.py" "$work/$name.png" "$work/$name.again.png"; then
        echo "old_logs: $name repeat replay exceeds image tolerance"; failed=1
      fi
      if ! cmp -s <(sed 's/ secs=[^ ]*//' "$work/$name.dig") <(sed 's/ secs=[^ ]*//' "$work/$name.again.dig"); then
        echo "old_logs: $name repeat state digests differ (diagnostic only)"
      fi
    fi
  else
    echo "old_logs: $name exceeds image tolerance"; failed=1
  fi
done <"$work/list"
[ $failed = 0 ] || { echo "old_logs: FAILED" >&2; exit 1; }
if [ -n "$record" ]; then echo "old_logs: all $n cases recorded in $record"; else echo "old_logs: all $n cases match recorded baselines"; fi
