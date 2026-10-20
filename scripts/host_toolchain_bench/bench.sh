#!/usr/bin/env bash
set -euo pipefail
bundle=$(cd "$(dirname "$0")" && pwd -P)
repo=$(git -C "$bundle" rev-parse --show-toplevel)
mkdir -p "$HOME/capy-measure"
case "${1:-}" in
  --cache-only) shift; exec bash "$bundle/run-cache.sh" "$repo" "$HOME/capy-measure" "${1:-0-3}" ;;
  --linkers-only) shift; exec bash "$bundle/run.sh" "$repo" "$HOME/capy-measure" "${1:-0-3}" ;;
  --preflight)
    shift
    bash "$bundle/run.sh" --preflight "$repo" "$HOME/capy-measure" "${1:-0-3}"
    exec bash "$bundle/run-cache.sh" --preflight "$repo" "$HOME/capy-measure" "${1:-0-3}"
    ;;
  --selftest) shift; exec bash "$bundle/selftest.sh" "${1:-0-3}" ;;
  '') ;;
  *) echo 'usage: bash scripts/host_toolchain_bench/bench.sh [--cache-only|--linkers-only|--preflight|--selftest] [CPU_LIST]' >&2; exit 2 ;;
esac
bash "$bundle/run.sh" "$repo" "$HOME/capy-measure" 0-3
exec bash "$bundle/run-cache.sh" "$repo" "$HOME/capy-measure" 0-3
