#!/usr/bin/env bash
# Every Rust test binary of the workspace (unit and integration tests, test
# profile), several binaries at a time, longest first, with the output of
# each binary kept whole. Prints each binary's output and its "test result:"
# line as cargo does; exits nonzero if a binary fails or none ran.
#
#   scripts/tests/cargo_tests.sh [-j N] [-- <test binary args>]
#
# `cargo test --workspace` runs the binaries one after another and the
# doctests (all four are ignored) after them; this runs the same tests
# (`--lib --bins --tests`: no doctests) in about half the time. -j: binaries
# at once (default 4); each still runs its tests on all cores.
set -euo pipefail
repo=$(cd "$(dirname "$0")/../.." && pwd)
cd "$repo"
jobs=4
[ "${1:-}" = -j ] && { jobs=$2; shift 2; }
[ "${1:-}" = -- ] && shift
work=$(mktemp -d "${TMPDIR:-/tmp}/cargo-tests.XXXXXX")
trap 'rm -rf "$work"' EXIT
cargo test --workspace --lib --bins --tests --no-run --message-format=json 2>"$work/build.err" >"$work/build.json" || { cat "$work/build.err" >&2; exit 1; }
# name, crate dir, executable; the slowest binaries first (notes/speed/timing)
python3 - "$work/build.json" >"$work/bins" <<'EOF'
import json, os, sys
order = ["paint/lib", "easel/bin", "session_integrity", "smoke", "determinism", "delivery", "ground_grain", "boxes"]
bins = []
for line in open(sys.argv[1]):
    try:
        m = json.loads(line)
    except ValueError:
        continue
    if m.get("reason") == "compiler-artifact" and m.get("executable") and m["profile"]["test"]:
        t = m["target"]
        key = t["name"] + "/" + t["kind"][0] if t["kind"][0] in ("lib", "bin") else t["name"]
        bins.append((order.index(key) if key in order else len(order), key, os.path.dirname(m["manifest_path"]), m["executable"]))
for _, key, d, exe in sorted(bins):
    print("%s\t%s\t%s" % (key.replace("/", "-"), d, exe))
EOF
[ -s "$work/bins" ] || { echo "cargo_tests: no test binaries" >&2; exit 1; }
export work
printf '%s\0' "$@" >"$work/args"
run() {
  local name=$1 dir=$2 exe=$3 args=()
  [ -s "$work/args" ] && mapfile -d '' args <"$work/args"
  local t0=$EPOCHREALTIME
  if (cd "$dir" && "$exe" "${args[@]}") >"$work/$name.out" 2>&1; then echo 0 >"$work/$name.rc"; else echo $? >"$work/$name.rc"; fi
  awk -v a="$t0" -v b="$EPOCHREALTIME" 'BEGIN { printf "%.2f\n", b - a }' >"$work/$name.secs"
}
export -f run
tr '\t' '\n' <"$work/bins" | tr '\n' '\0' | xargs -0 -n 3 -P "$jobs" bash -c 'run "$0" "$1" "$2"'
failed=0
while IFS=$'\t' read -r name dir exe; do
  echo "     Running $name ($(cat "$work/$name.secs") s wall)"
  cat "$work/$name.out"
  [ "$(cat "$work/$name.rc")" = 0 ] || { failed=1; echo "cargo_tests: $name FAILED (exit $(cat "$work/$name.rc"))"; }
done <"$work/bins"
[ $failed = 0 ] || exit 1
