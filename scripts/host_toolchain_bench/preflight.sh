#!/usr/bin/env bash
set -euo pipefail
repo=$1
result=$2
cpus=$3
candidate=$4
cd "$repo"
for tool in cargo rustc uv taskset strace readelf perl pkill; do
  command -v "$tool"
done
taskset -c "$cpus" uv run --no-project python -c 'import os; a=sorted(os.sched_getaffinity(0)); print("affinity=" + str(a)); assert len(a)==4, "exactly four CPUs required"' | tee "$result/preflight-affinity.txt"
taskset -c "$cpus" cargo fetch --locked
uv run --no-project --with pytest --with 'pillow>=11,<13' python -c 'import pytest; from PIL import Image; Image.new("RGBA", (1, 1)).tobytes()'
rustc --version
cargo --version
printf 'candidate=%s\nlinker=%s\nrustflags=%s\nwrapper=%s\nworkspace_wrapper=%s\n' "$candidate" "$CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER" "${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS:-}" "$RUSTC_WRAPPER" "$RUSTC_WORKSPACE_WRAPPER"
