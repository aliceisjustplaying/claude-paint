#!/usr/bin/env bash
set -euo pipefail
result=$1
cpus=$2
if [ "${LINKER_BENCH_PINNED:-}" != yes ]; then
  export LINKER_BENCH_PINNED=yes
  exec taskset -c "$cpus" bash "$0" "$@"
fi
bundle=$(cd "$(dirname "$0")" && pwd -P)
unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS KACHE_DISABLED
. "$bundle/isolate-cache.sh" "$result"
export RUSTC_WRAPPER=kache RUSTC_WORKSPACE_WRAPPER=''
mkdir -p "$result/source/src" "$result/source/tests"
printf '[package]\nname = "easel"\nversion = "0.0.0"\nedition = "2024"\n' >"$result/source/Cargo.toml"
printf 'fn main() { println!("tiny cargo proof"); }\n' >"$result/source/src/main.rs"
printf '#[test]\nfn smoke() { assert!(std::path::Path::new(env!("CARGO_BIN_EXE_easel")).is_file()); }\n' >"$result/source/tests/smoke.rs"
cd "$result/source"
uv run --no-project python -c 'import os; a=sorted(os.sched_getaffinity(0)); print(a); assert len(a)==4' >"$result/affinity.txt"
export CARGO_TARGET_DIR="$result/target-good"
cargo test --no-run --test smoke --message-format=json-render-diagnostics >"$result/good.json" 2>"$result/good.log"
{
  printf 'kache: diagnostic reproduction\nbackground daemon does not inherit KACHE_LOCAL_ONLY\n[cache.remote]\n'
  printf '[libc 0.2.189] cargo:rerun-if-changed=build.rs\n[libc 0.2.189] cargo:rustc-cfg=freebsd12\n[libc 0.2.189] cargo:rustc-check-cfg=cfg(freebsd12)\n'
  cat "$result/good.json"
  printf 'kache: trailing diagnostic\n'
} >"$result/mixed.json"
exe=$(uv run --no-project python "$bundle/artifact_executable.py" "$result/mixed.json" "$result/mixed.diagnostics.log")
"$exe" >"$result/executable.log"
grep -qF 'tiny cargo proof' "$result/executable.log"
grep -qF 'background daemon does not inherit' "$result/mixed.diagnostics.log"
grep -qF '[libc 0.2.189] cargo:rerun-if-changed=build.rs' "$result/mixed.diagnostics.log"
grep -qF '[libc 0.2.189] cargo:rustc-cfg=freebsd12' "$result/mixed.diagnostics.log"
grep -qF '[libc 0.2.189] cargo:rustc-check-cfg=cfg(freebsd12)' "$result/mixed.diagnostics.log"
printf '{malformed cargo JSON\n' >"$result/malformed.json"
if uv run --no-project python "$bundle/artifact_executable.py" "$result/malformed.json" "$result/malformed.diagnostics.log" >"$result/malformed.out" 2>&1; then
  echo 'parser accepted malformed Cargo JSON' >&2; exit 1
fi
cp src/main.rs "$result/main.original.rs"
printf '\ncompile_error!("intentional parser compiler-error check");\n' >>src/main.rs
export CARGO_TARGET_DIR="$result/target-bad"
if cargo test --no-run --test smoke --message-format=json-render-diagnostics >"$result/bad.json" 2>"$result/bad.log"; then
  echo 'compiler unexpectedly accepted deliberate error' >&2; exit 1
fi
cp "$result/main.original.rs" src/main.rs
if uv run --no-project python "$bundle/artifact_executable.py" "$result/bad.json" "$result/bad.diagnostics.log" >"$result/bad.out" 2>&1; then
  echo 'parser accepted failed compiler result' >&2; exit 1
fi
grep -qF 'intentional parser compiler-error check' "$result/bad.log"
grep -qF 'Cargo reported build failure' "$result/bad.out"
kache --json stats >"$result/stats.json" 2>"$result/stats.diagnostics.log"
printf 'selftest: real Cargo build, mixed diagnostics, malformed JSON and compiler-error rejection passed\nresults=%s\n' "$result"
