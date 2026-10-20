#!/usr/bin/env bash
set -euo pipefail
bundle=$(cd "$(dirname "$0")" && pwd -P)
repo=$(git -C "$bundle" rev-parse --show-toplevel)
mkdir -p "$HOME/capy-measure"
result=$(mktemp -d "$HOME/capy-measure/parser-check.XXXXXX")
cd "$repo"
cp flake.nix "$result/flake.original.nix"
trap 'cp "$result/flake.original.nix" "$repo/flake.nix"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
cp "$bundle/flake.gnu.nix" flake.nix
nix develop --no-write-lock-file --command bash "$bundle/selftest-inside.sh" "$result" "${1:-0-3}"
