#!/usr/bin/env bash
# The engine against the small before-change results (notes/thinner/baseline):
# every state field af49348 had, after every chunk of the six scenes, equal
# bit for bit; fields a newer engine adds zero (--added-zero); and every
# scene's PNG the same bytes as af49348's. (The thinner's check 2: "no
# thinner" leaves engine 3 as it was.)
#
#   scripts/tests/baseline_state.sh [<release easel>]   (default target/release/easel)
set -euo pipefail
repo=$(cd "$(dirname "$0")/../.." && pwd)
easel=${1:-$repo/target/release/easel}
out=$("$repo/notes/thinner/baseline/tools/compare_build.sh" "$easel" --added-zero) || { echo "$out"; exit 1; }
echo "$out"
scenes=$(ls "$repo/notes/thinner/baseline/scenes" | grep -c '\.lua$')
same=$(grep -c '^  png same as af49348' <<<"$out" || true)
[ "$same" = "$scenes" ] || { echo "baseline_state: $same of $scenes PNGs equal af49348's" >&2; exit 1; }
echo "baseline_state: all $scenes scenes equal af49348 in state and PNG"
