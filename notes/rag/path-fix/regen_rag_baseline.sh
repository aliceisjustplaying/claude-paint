#!/usr/bin/env bash
# PROPOSED, NOT RUN on the protected baseline: regenerate the `rag` scene of
# notes/thinner/baseline/ (protected, notes/golden_paths.txt) from a
# candidate's release easel, for the owner's approval. The other five scenes
# are untouched. The easel must have `run --dump-state` (every build since
# a97c3a6 does).
#
#   notes/rag/path-fix/regen_rag_baseline.sh <release easel> [--apply]
#
# Without --apply it writes the new files to a temporary directory and
# prints what would change (state fields, PNG, printed output), leaving the
# baseline alone. With --apply it replaces ref/png/rag.png,
# ref/digest/rag.txt, ref/out/rag.txt, the rag row of ref/times.tsv and
# state/rag/chunk-*.state.xz, and rewrites SHA256SUMS. Run under lockrun:
#   ~/src/a/claude-paint-tools/lockrun --timeout 120 --owner <who> -- \
#     notes/rag/path-fix/regen_rag_baseline.sh target/release/easel
# The README/MANIFEST text that says every scene is af49348's needs a line
# saying which revision the rag scene now comes from (owner's wording).
set -euo pipefail
[ $# -ge 1 ] || { echo "usage: regen_rag_baseline.sh <release easel> [--apply]" >&2; exit 2; }
easel=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
apply=${2:-}
repo=$(cd "$(dirname "$0")/../../.." && pwd)
base=$repo/notes/thinner/baseline
work=$(mktemp -d "${TMPDIR:-/tmp}/rag-regen.XXXXXX")
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/scenes"
cp "$base/scenes/rag.lua" "$work/scenes/"
# run_scenes.sh replays every scene in its own scenes/; give it only this one
mkdir -p "$work/tools"
cp "$base/tools/run_scenes.sh" "$work/tools/"
"$work/tools/run_scenes.sh" "$easel" "$work/new" --dump >/dev/null
python3 "$base/tools/state_compare.py" "$base/state/rag" "$work/new/state/rag" --added-zero || true
cmp -s "$base/ref/png/rag.png" "$work/new/png/rag.png" && echo "png: same" || echo "png: DIFFERENT"
diff "$base/ref/out/rag.txt" "$work/new/out/rag.txt" && echo "printed output: same" || true
if [ "$apply" = --apply ]; then
  cp "$work/new/png/rag.png" "$base/ref/png/rag.png"
  cp "$work/new/digest/rag.txt" "$base/ref/digest/rag.txt"
  cp "$work/new/out/rag.txt" "$base/ref/out/rag.txt"
  awk -F'\t' -v OFS='\t' -v t="$(cut -f2 "$work/new/times.tsv")" '$1 == "rag" { $2 = t } { print }' "$base/ref/times.tsv" >"$work/times.tsv"
  cp "$work/times.tsv" "$base/ref/times.tsv"
  rm -f "$base"/state/rag/chunk-*.state.xz
  for f in "$work"/new/state/rag/chunk-*.state; do
    xz -9e -T1 -c "$f" >"$base/state/rag/$(basename "$f").xz"
  done
  # same files, same order: only the hashes change
  (cd "$base" && awk '{ print $2 }' SHA256SUMS | xargs shasum -a 256 >"$work/SHA256SUMS" && cp "$work/SHA256SUMS" SHA256SUMS)
  echo "applied: rag scene regenerated; SHA256SUMS rewritten"
fi
