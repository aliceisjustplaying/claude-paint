#!/usr/bin/env bash
# Replay the small before-change scenes (notes/thinner/baseline/scenes/*.lua)
# at 128 px with an easel, writing per scene: the PNG, the per-chunk state
# digests (`--state-digest`), what the chunks printed, and with --dump the
# full state after every chunk (`--dump-state`: chunk-NNN.state, final.ckpt).
#
#   run_scenes.sh <easel> <out dir> [--dump]
#
# <easel> is a release build (`cargo build --release -p easel`; the iter
# profile can move the last digits). Each scene takes well under a second;
# all six about 0.1 s. Run it as one heavy job:
#   scripts/lockrun --timeout 60 -- notes/thinner/baseline/tools/run_scenes.sh ...
# A digest line's secs= field is wall time; compare digests without it
# (README.md).
set -euo pipefail
[ -n "${EPOCHREALTIME:-}" ] || { echo "run_scenes: needs bash 5 (EPOCHREALTIME)" >&2; exit 2; }
[ $# -ge 2 ] || { echo "usage: run_scenes.sh <easel> <out dir> [--dump]" >&2; exit 2; }
easel=$1
out=$2
dump=${3:-}
here=$(cd "$(dirname "$0")/.." && pwd)
unset EASEL_BOX
mkdir -p "$out/png" "$out/digest" "$out/out" "$out/err"
: >"$out/times.tsv"
for lua in "$here"/scenes/*.lua; do
  s=$(basename "$lua" .lua)
  extra=()
  [ "$dump" = --dump ] && extra=(--dump-state "$out/state/$s")
  t0=$EPOCHREALTIME
  "$easel" run "$lua" --width 128 --out "$out/png/$s.png" --state-digest "$out/digest/$s.txt" "${extra[@]}" \
    >"$out/out/$s.txt" 2>"$out/err/$s.txt" || { echo "run_scenes: $s failed (see $out/err/$s.txt)" >&2; exit 1; }
  t1=$EPOCHREALTIME
  printf '%s\t%s\n' "$s" "$(awk -v a="$t0" -v b="$t1" 'BEGIN { printf "%.3f", b - a }')" >>"$out/times.tsv"
done
echo "run_scenes: $(wc -l <"$out/times.tsv" | tr -d ' ') scenes -> $out"
