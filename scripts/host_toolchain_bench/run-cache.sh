#!/usr/bin/env bash
set -euo pipefail
preflight=0
if [ "${1:-}" = --preflight ]; then
  preflight=1
  shift
fi
[ "$#" -ge 2 ] && [ "$#" -le 3 ] || { echo 'usage: bash run-cache.sh [--preflight] CHECKOUT EXISTING_OUTPUT_DIRECTORY [CPU_LIST]' >&2; exit 2; }
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
case "$output/" in "$repo/"*) echo 'output must be outside checkout' >&2; exit 2 ;; esac
result=$(mktemp -d "$output/cache-bench.XXXXXX")
cp flake.nix "$result/flake.original.nix"
cp crates/easel/src/main.rs "$result/main.original.rs"
trap 'cp "$result/flake.original.nix" "$repo/flake.nix"; cp "$result/main.original.rs" "$repo/crates/easel/src/main.rs"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
cp "$bundle/flake.gnu.nix" flake.nix
printf 'results=%s\nrevision=%s\nrequested_cpus=%s\n' "$result" "$(git rev-parse HEAD)" "$cpus" | tee "$result/run.txt"
nix develop --no-write-lock-file --command env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_RUSTFLAGS RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= bash "$bundle/preflight.sh" "$repo" "$result" "$cpus" cache >"$result/preflight.log" 2>&1
nix develop --no-write-lock-file --command kache --version >>"$result/preflight.log" 2>&1
if [ "$preflight" = 1 ]; then
  echo 'cache preflight: ready; no builds or tests ran'
  exit 0
fi
nix develop --no-write-lock-file --command bash "$bundle/cache.sh" "$repo" "$result" "$cpus" >"$result/cache.log" 2>&1
cat "$result/cache.log"
