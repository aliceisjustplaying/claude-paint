#!/usr/bin/env bash
set -euo pipefail
bundle=$(cd "$(dirname "$0")" && pwd -P)
repo=$(git -C "$bundle" rev-parse --show-toplevel)
preflight=0
if [ "${1:-}" = --preflight ]; then preflight=1; shift; fi
if [ "${1:-}" != --inside ]; then
  [ "$#" -ge 1 ] && [ "$#" -le 2 ] || { echo 'usage: bash bench.sh --acceptance FULL_SPEED_SHA [CPU_LIST]' >&2; exit 2; }
  [[ $1 =~ ^[0-9a-f]{40}$ ]] || { echo 'a full commit SHA is required' >&2; exit 2; }
  mkdir -p "$HOME/capy-measure"
  result=$(mktemp -d "$HOME/capy-measure/acceptance.XXXXXX")
  cd "$repo"
  cp flake.nix "$result/flake.original.nix"
  trap 'cp "$result/flake.original.nix" "$repo/flake.nix"' EXIT
  trap 'exit 130' INT
  trap 'exit 143' TERM
  cp "$bundle/flake.gnu.nix" flake.nix
  nix develop --no-write-lock-file --command bash "$0" --inside "$result" "$1" "${2:-0-3}" "$preflight"
  exit
fi
result=$2
sha=$3
cpus=$4
preflight=$5
taskset=$(command -v taskset)
remote=$(git -C "$repo" remote get-url origin)
printf 'results=%s\nrequested_speed_sha=%s\nrequested_cpus=%s\n' "$result" "$sha" "$cpus" | tee "$result/run.txt"
git clone --filter=blob:none --no-checkout "$remote" "$result/source" >"$result/clone.log" 2>&1
git -C "$result/source" fetch origin "$sha" >>"$result/clone.log" 2>&1
git -C "$result/source" checkout --detach "$sha" >>"$result/clone.log" 2>&1
[ "$(git -C "$result/source" rev-parse HEAD)" = "$sha" ]
cd "$result/source"
nix develop --no-write-lock-file --command env -u RUST_TEST_THREADS -u RAYON_NUM_THREADS -u CARGO_BUILD_JOBS -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_RUSTFLAGS "$taskset" -c "$cpus" bash "$bundle/acceptance-run.sh" "$result" "$preflight" >"$result/acceptance.log" 2>&1
cat "$result/acceptance.log"
