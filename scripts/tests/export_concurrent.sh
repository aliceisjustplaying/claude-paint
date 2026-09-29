#!/usr/bin/env bash
# Exports of different profiles at once, from an empty build directory
# (nothing extracted, no easel cached): each studio gets its own box's easel.
# The sargent, inness and blank exports start together, so they extract the
# archive and build at the same time; then each studio's easel must know
# exactly its own box (the list its refusal of an unknown box gives) and
# its strings must hold no other box's name.
#
#   R16_BRANCH=<branch> scripts/tests/export_concurrent.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR). Builds
# three painter easels from nothing (several minutes).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
: "${R16_BRANCH:?set R16_BRANCH to the branch to export}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
work=$(mktemp -d "$TMPDIR/export-concurrent.XXXXXX")
unset EASEL_BOX
export STUDIO_BUILDS=$work/builds
profiles=(sargent inness blank)
pids=()
for p in "${profiles[@]}"; do
  "$repo/scripts/export_r16_studio" "$p" "$work/$p" >"$work/$p.log" 2>&1 &
  pids+=($!)
done
failed=0
for i in "${!profiles[@]}"; do
  wait "${pids[$i]}" || { echo "${profiles[$i]}: the export failed:" >&2; cat "$work/${profiles[$i]}.log" >&2; failed=1; }
done
[ $failed = 0 ] || exit 1
for p in "${profiles[@]}"; do
  case $p in blank) want='"tube box"' ;; *) want="\"$p\"" ;; esac
  e=$work/$p/bin/easel
  # the boxes this easel knows: its refusal of a box no easel has lists them
  probe=$work/$p-probe
  mkdir -p "$probe/bin"
  cp "$e" "$probe/bin/easel"
  echo "no-such-box" > "$probe/bin/box"
  err=$(cd / && "$probe/bin/easel" open 2>&1 || true)
  known=$(sed -n 's/.*no box "no-such-box" (its boxes: \(.*\)).*/\1/p' <<<"$err")
  [ "$known" = "$want" ] || { echo "$p: its easel knows the boxes [$known], not [$want] ($err)" >&2; exit 1; }
  # and its strings name no other box (inside words too: they run together)
  others=()
  for b in sargent inness tadema tonn "tube box"; do
    [ "$b" = "$p" ] || { [ "$b" = "tube box" ] && [ "$p" = blank ]; } || others+=("$b")
  done
  hit=$(grep -ioE "$(IFS='|'; echo "${others[*]}")" <<<"$(strings "$e")" | sort -u | tr '\n' ' ' || true)
  [ -z "$hit" ] || { echo "$p: its easel's strings name other boxes: $hit" >&2; exit 1; }
  echo "$p: ok (boxes: $known)"
done
echo "export_concurrent: three exports at once from an empty build directory each got their own box's easel ($work)"
