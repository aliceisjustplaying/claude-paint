#!/usr/bin/env bash
set -euo pipefail
result=$1
bundle=$(cd "$(dirname "$0")" && pwd -P)
repo=$PWD
unset KACHE_DISABLED EASEL_BOX
. "$bundle/isolate-cache.sh" "$result"
export RUSTC_WRAPPER=kache RUSTC_WORKSPACE_WRAPPER=''
export CFLAGS='-Dluai_makeseed()=0x5eedu' PYTHONDONTWRITEBYTECODE=1
[ -z "${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS:-}" ] || { echo 'acceptance requires default GNU linker flags' >&2; exit 2; }
uv run --no-project python -c 'import os; a=sorted(os.sched_getaffinity(0)); print("affinity=" + str(a)); assert len(a)==4' >"$result/affinity.txt"
grep Cpus_allowed_list /proc/self/status >>"$result/affinity.txt"
printf 'speed_sha=%s\nlinker=%s\nrustflags=%s\nRUST_TEST_THREADS=%s\nRAYON_NUM_THREADS=%s\nCARGO_BUILD_JOBS=%s\ncache=isolated local-only, target seeding disabled\n' "$(git rev-parse HEAD)" "$CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER" "${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS:-}" "${RUST_TEST_THREADS:-unset}" "${RAYON_NUM_THREADS:-unset}" "${CARGO_BUILD_JOBS:-unset}" >"$result/env.txt"
printf 'int main(void) { return 0; }\n' >"$result/linker-probe.c"
"$CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER" -Wl,--version "$result/linker-probe.c" -o "$result/linker-probe" >"$result/linker-version.txt" 2>&1
grep -q 'GNU ld' "$result/linker-version.txt"
RUSTC_WRAPPER='' KACHE_DISABLED=1 cargo fetch --locked >"$result/fetch.log" 2>&1
RUSTC_WRAPPER='' KACHE_DISABLED=1 uv run --no-project --with pytest --with 'pillow>=11,<13' python -c 'import pytest; from PIL import Image; Image.new("RGBA", (1, 1)).tobytes()'
[ ! -e "$KACHE_CACHE_DIR" ] || { echo 'refusing nonempty acceptance cache' >&2; exit 2; }
if [ "${2:-0}" = 1 ]; then
  cat "$result/affinity.txt" "$result/env.txt" "$result/linker-version.txt"
  echo 'acceptance preflight: requested SHA, four CPUs, GNU linker and isolated fresh cache verified; no gate ran'
  exit 0
fi
for mode in cold warm; do
  mkdir "$result/$mode"
  export CARGO_TARGET_DIR="$result/$mode/target"
  [ ! -e "$CARGO_TARGET_DIR" ] || exit 2
  if [ -f "$KACHE_RUNTIME_DIR/events.jsonl" ]; then cp "$KACHE_RUNTIME_DIR/events.jsonl" "$result/$mode/events.before.jsonl"; else : >"$result/$mode/events.before.jsonl"; fi
  if { time -p bash scripts/test >"$result/$mode/gate.log" 2>&1; } 2>"$result/$mode/gate.time"; then rc=0; else rc=$?; fi
  echo "$rc" >"$result/$mode/gate.rc"
  grep '^gate: built in ' "$result/$mode/gate.log" >"$result/$mode/build.time" || true
  kache --json stats >"$result/$mode/stats.json" 2>"$result/$mode/stats.diagnostics.log"
  if [ -f "$KACHE_RUNTIME_DIR/events.jsonl" ]; then cp "$KACHE_RUNTIME_DIR/events.jsonl" "$result/$mode/events.after.jsonl"; else : >"$result/$mode/events.after.jsonl"; fi
  printf '%s gate:\n' "$mode"
  cat "$result/$mode/gate.time"
  tail -1 "$result/$mode/gate.log"
  [ "$rc" = 0 ] || exit "$rc"
  uv run --no-project --with 'pillow>=11,<13' python "$bundle/render_pixels.py" "$repo" "$result/$mode" "$CARGO_TARGET_DIR/debug/easel" >"$result/$mode/pixels.log" 2>&1
done
uv run --no-project --with 'pillow>=11,<13' python "$bundle/compare_pixels.py" "$result" cold warm >"$result/pixel-comparison.log" 2>&1
cat "$result/pixel-comparison.log"
git diff --exit-code
echo 'acceptance: cold and warm fresh-target full gates passed with identical historical pixels'
