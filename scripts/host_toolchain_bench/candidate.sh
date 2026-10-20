#!/usr/bin/env bash
set -euo pipefail
repo=$1
result=$2
cpus=$3
candidate=$4
bundle=$(cd "$(dirname "$0")" && pwd -P)
if [ "${LINKER_BENCH_PINNED:-}" != yes ]; then
  export LINKER_BENCH_PINNED=yes
  exec taskset -c "$cpus" bash "$0" "$@"
fi
cd "$repo"
unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS
export RUSTC_WRAPPER='' RUSTC_WORKSPACE_WRAPPER=''
export CFLAGS='-Dluai_makeseed()=0x5eedu'
export CARGO_TARGET_DIR="$result/target"
[ ! -e "$CARGO_TARGET_DIR" ] || { echo 'target already exists; refusing to reuse it' >&2; exit 2; }
uv run --no-project python -c 'import os; a=sorted(os.sched_getaffinity(0)); print("affinity=" + str(a)); assert len(a)==4' >"$result/affinity.txt"
grep Cpus_allowed_list /proc/self/status >>"$result/affinity.txt"
printf 'candidate=%s\nlinker=%s\nrustflags=%s\nwrapper=%s\nworkspace_wrapper=%s\n' "$candidate" "$CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER" "${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS:-}" "$RUSTC_WRAPPER" "$RUSTC_WORKSPACE_WRAPPER" >"$result/env.txt"
printf 'CFLAGS=%s\n' "$CFLAGS" >>"$result/env.txt"
build=$(sed -n 's/^launch build "\(.*\)"/\1/p' scripts/test)
[ -n "$build" ] || { echo 'missing exact gate build command' >&2; exit 2; }
printf '%s\n' "$build" >"$result/build-command.txt"
export work="$result"
if { time -p bash -c "$build" >"$result/build.log" 2>&1; } 2>"$result/build.time"; then
  echo 0 >"$result/build.rc"
else
  rc=$?
  echo "$rc" >"$result/build.rc"
  exit "$rc"
fi
if { time -p bash scripts/test >"$result/gate.log" 2>&1; } 2>"$result/gate.time"; then
  rc=0
else
  rc=$?
fi
echo "$rc" >"$result/gate.rc"
exe=$(uv run --no-project python "$bundle/artifact_executable.py" "$result/build.json" "$result/build.diagnostics.log")
readelf -p .comment "$exe" >"$result/comment.txt"
readelf --version-info "$exe" >"$result/abi.txt"
unset EASEL_BOX
export PYTHONDONTWRITEBYTECODE=1
uv run --no-project --with 'pillow>=11,<13' python "$bundle/render_pixels.py" "$repo" "$result" "$exe" >"$result/pixels.log" 2>&1
find "$CARGO_TARGET_DIR/debug" -samefile "$exe" -type f -delete
strace -f -e trace=execve -s 4096 -o "$result/exec.trace" bash -c "$build" >"$result/proof-build.log" 2>&1
grep -E 'execve\("[^"]*/(ld[^/"]*|mold|wild|collect2)"' "$result/exec.trace" >"$result/linker-proof.txt"
cat "$result/affinity.txt" "$result/build.time" "$result/gate.time"
tail -1 "$result/gate.log"
exit "$rc"
