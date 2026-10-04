#!/usr/bin/env bash
# Check the baseline package itself (no build needed, about 2 s):
# 1. every file's SHA-256 against SHA256SUMS (and no file missing from it);
# 2. state_compare.py's own checks (test_state_compare.py);
# 3. every stored dump rebuilds the PAINTCK8 checkpoint whose FNV-1a is the
#    canvas= digest af49348's unchanged easel wrote for that chunk (ref/digest).
#
#   notes/thinner/baseline/tools/verify_package.sh
set -euo pipefail
cd "$(dirname "$0")/.."
shasum -a 256 -c SHA256SUMS --quiet || { echo "verify_package: a file differs from SHA256SUMS" >&2; exit 1; }
listed=$(awk '{print $2}' SHA256SUMS | sort)
present=$(find . -type f ! -name SHA256SUMS | sed 's|^\./||' | sort)
[ "$listed" = "$present" ] || { echo "verify_package: files and SHA256SUMS disagree:" >&2; diff <(echo "$listed") <(echo "$present") >&2; exit 1; }
python3 tools/test_state_compare.py >/dev/null || { echo "verify_package: test_state_compare.py failed" >&2; exit 1; }
n=0
for d in state/*/; do
  s=$(basename "$d")
  for f in "$d"chunk-*.state.xz; do
    python3 tools/checkpoint_from_dump.py "$f" --digest "ref/digest/$s.txt" >/dev/null || { echo "verify_package: $f doesn't rebuild ref/digest/$s.txt's checkpoint" >&2; exit 1; }
    n=$((n + 1))
  done
done
echo "verify_package: hashes ok; state_compare checks pass; $n dumps rebuild af49348's checkpoints"
