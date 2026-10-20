#!/usr/bin/env bash
set -euo pipefail
preflight=0
if [ "${1:-}" = --preflight ]; then
  preflight=1
  shift
fi
[ "$#" -ge 2 ] && [ "$#" -le 3 ] || { echo "usage: bash run.sh [--preflight] CHECKOUT EXISTING_OUTPUT_DIRECTORY [CPU_LIST]" >&2; exit 2; }
bundle=$(cd "$(dirname "$0")" && pwd -P)
repo=$(cd "$1" && pwd -P)
output=$(cd "$2" && pwd -P)
measure=$(cd "$HOME/capy-measure" && pwd -P)
case "$output/" in "$measure/"*) ;; *) echo 'output must be under ~/capy-measure' >&2; exit 2 ;; esac
cpus=${3:-0-3}
cd "$repo"
git merge-base --is-ancestor 8272ecb28bed5fb5315497520011cc4a8098dc4e HEAD
git diff --quiet 8272ecb28bed5fb5315497520011cc4a8098dc4e -- . ':(exclude)scripts/host_toolchain_bench' || { echo 'production files must match 8272ecb2' >&2; exit 2; }
[ -z "$(git status --porcelain -- . ':(exclude)scripts/host_toolchain_bench')" ] || { echo 'production checkout must be clean' >&2; exit 2; }
case "$output/" in "$repo/"*) echo 'output must be outside the checkout' >&2; exit 2 ;; esac
command -v nix >/dev/null
result=$(mktemp -d "$output/linker-bench.XXXXXX")
cp flake.nix "$result/flake.original.nix"
trap 'cp "$result/flake.original.nix" "$repo/flake.nix"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
printf 'results=%s\nrevision=%s\nrequested_cpus=%s\n' "$result" "$(git rev-parse HEAD)" "$cpus" | tee "$result/run.txt"
failed=0
for candidate in gnu mold wild lld; do
  mkdir "$result/$candidate"
  cp "$bundle/flake.$candidate.nix" flake.nix
  cp flake.nix "$result/$candidate/flake.nix"
  if ! nix develop --no-write-lock-file --command env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_RUSTFLAGS RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= bash "$bundle/preflight.sh" "$repo" "$result/$candidate" "$cpus" "$candidate" >"$result/$candidate/preflight.log" 2>&1; then
    cat "$result/$candidate/preflight.log" >&2
    exit 1
  fi
done
if [ "$preflight" = 1 ]; then
  echo 'preflight: all four candidates ready; no builds or tests ran'
  exit 0
fi
for candidate in gnu mold wild lld; do
  cp "$bundle/flake.$candidate.nix" flake.nix
  if nix develop --no-write-lock-file --command env RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= bash "$bundle/candidate.sh" "$repo" "$result/$candidate" "$cpus" "$candidate" >"$result/$candidate/run.log" 2>&1; then
    echo "$candidate: completed"
  else
    failed=1
    echo "$candidate: FAILED (see $result/$candidate/run.log)" >&2
  fi
done
if ! nix develop --no-write-lock-file --command uv run --no-project --with 'pillow>=11,<13' python "$bundle/compare_pixels.py" "$result" >"$result/pixel-comparison.log" 2>&1; then
  failed=1
fi
cat "$result/pixel-comparison.log"
exit "$failed"
