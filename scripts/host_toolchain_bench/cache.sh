#!/usr/bin/env bash
set -euo pipefail
repo=$1
result=$2
cpus=$3
if [ "${LINKER_BENCH_PINNED:-}" != yes ]; then
  export LINKER_BENCH_PINNED=yes
  exec taskset -c "$cpus" bash "$0" "$@"
fi
bundle=$(cd "$(dirname "$0")" && pwd -P)
cd "$repo"
unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS KACHE_DISABLED EASEL_BOX
export RUSTC_WRAPPER=kache RUSTC_WORKSPACE_WRAPPER=''
export CFLAGS='-Dluai_makeseed()=0x5eedu'
export CC_ENABLE_DEBUG_OUTPUT=1 PYTHONDONTWRITEBYTECODE=1
. "$bundle/isolate-cache.sh" "$result"
[ ! -e "$KACHE_CACHE_DIR" ] || { echo 'cache store already exists' >&2; exit 2; }
uv run --no-project python -c 'import os; a=sorted(os.sched_getaffinity(0)); print("affinity=" + str(a)); assert len(a)==4' >"$result/affinity.txt"
grep Cpus_allowed_list /proc/self/status >>"$result/affinity.txt"
printf 'linker=%s\nrustflags=%s\nwrapper=%s\nworkspace_wrapper=%s\nCFLAGS=%s\nKACHE_SEED_NEW_TARGETS=%s\n' "$CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER" "${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS:-}" "$RUSTC_WRAPPER" "$RUSTC_WORKSPACE_WRAPPER" "$CFLAGS" "$KACHE_SEED_NEW_TARGETS" >"$result/env.txt"
build_one() {
  local mode=$1 source=$2 build exe
  cd "$source"
  mkdir "$result/$mode"
  export CARGO_TARGET_DIR="$result/$mode/target" work="$result/$mode"
  [ ! -e "$CARGO_TARGET_DIR" ] || { echo 'refusing existing target' >&2; return 2; }
  build=$(sed -n 's/^launch build "\(.*\)"/\1/p' scripts/test)
  [ -n "$build" ] || return 2
  build=${build/ --message-format=json-render-diagnostics/ --message-format=json-render-diagnostics -vv}
  printf '%s\n' "$build" >"$work/build-command.txt"
  if [ -f "$KACHE_RUNTIME_DIR/events.jsonl" ]; then cp "$KACHE_RUNTIME_DIR/events.jsonl" "$work/events.before.jsonl"; else : >"$work/events.before.jsonl"; fi
  { time -p bash -c "$build" >"$work/build.log" 2>&1; } 2>"$work/build.time"
  if [ "$mode" != off ]; then kache --json stats >"$work/stats.json" 2>"$work/stats.diagnostics.log"; fi
  if [ -f "$KACHE_RUNTIME_DIR/events.jsonl" ]; then cp "$KACHE_RUNTIME_DIR/events.jsonl" "$work/events.after.jsonl"; else : >"$work/events.after.jsonl"; fi
  if [ "$mode" = off ] || [ "$mode" = warm ]; then
    if { time -p bash scripts/test >"$work/gate.log" 2>&1; } 2>"$work/gate.time"; then
      echo 0 >"$work/gate.rc"
    else
      echo "$?" >"$work/gate.rc"
      return 1
    fi
  fi
  exe=$(uv run --no-project python "$bundle/artifact_executable.py" "$work/build.json" "$work/build.diagnostics.log")
  uv run --no-project --with 'pillow>=11,<13' python "$bundle/render_pixels.py" "$source" "$work" "$exe" >"$work/pixels.log" 2>&1
  printf '%s build timing:\n' "$mode"
  cat "$work/build.time"
}
export KACHE_DISABLED=1 RUSTC_WRAPPER=''
build_one off "$repo"
unset KACHE_DISABLED
export RUSTC_WRAPPER=kache
build_one cold "$repo"
build_one warm "$repo"
printf '\n' >>"$repo/crates/easel/src/main.rs"
build_one touched "$repo"
cp "$result/main.original.rs" "$repo/crates/easel/src/main.rs"
git -C "$repo" diff --exit-code -- crates/easel/src/main.rs
git clone --no-hardlinks --no-checkout "$repo" "$result/second-source" >"$result/clone.log" 2>&1
git -C "$result/second-source" checkout --detach 8272ecb28bed5fb5315497520011cc4a8098dc4e >>"$result/clone.log" 2>&1
build_one second-checkout "$result/second-source"
cd "$repo"
for mode in off cold; do
  grep -F -- 'running:' "$result/$mode/build.log" | grep -F -- '-Dluai_makeseed()=0x5eedu' >"$result/$mode/lua-fixed-seed-proof.txt"
done
uv run --no-project --with 'pillow>=11,<13' python "$bundle/compare_pixels.py" "$result" off cold warm touched second-checkout >"$result/pixel-comparison.log" 2>&1
cat "$result/pixel-comparison.log"
cp "$result/second-source/crates/easel/src/main.rs" "$result/second-main.original.rs"
printf '\ncompile_error!("intentional cache stale-result guard");\n' >>"$result/second-source/crates/easel/src/main.rs"
export CARGO_TARGET_DIR="$result/negative-target" work="$result/negative"
mkdir "$work"
cd "$result/second-source"
build=$(sed -n 's/^launch build "\(.*\)"/\1/p' scripts/test)
if bash -c "$build" >"$work/build.log" 2>&1; then rc=0; else rc=$?; fi
cp "$result/second-main.original.rs" crates/easel/src/main.rs
echo "$rc" >"$work/build.rc"
[ "$rc" -ne 0 ] && grep -qF 'intentional cache stale-result guard' "$work/build.log"
echo 'cache: intentional compile error was not hidden by the warm store'
