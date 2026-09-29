#!/usr/bin/env bash
# Exports of different profiles at once, from an empty build directory
# (nothing extracted, no easel cached): each studio gets its own box's easel.
# The sargent, inness and blank exports start together, so they extract the
# archive and build at the same time; then each studio's easel must know
# exactly its own box (the list its refusal of an unknown box gives) and
# its strings must hold no other box's name. The sargent export is held
# between its build and its copy (STUDIO_BUILD_HOOK) until the inness and
# blank builds have finished, so an export that copied from a build output
# the others share would copy another box's easel every time.
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
# the barrier: each build says it's done; sargent's waits for the other two
hook=$work/hook
cat > "$hook" <<HOOK
#!/usr/bin/env bash
set -eu
touch "$work/built-\$1"
[ "\$1" = sargent ] || exit 0
for i in \$(seq 1 1800); do
  [ -e "$work/built-inness" ] && [ -e "$work/built-default" ] && exit 0
  sleep 1
done
echo "the inness and blank builds didn't finish in 30 minutes" >&2
exit 1
HOOK
chmod +x "$hook"
export STUDIO_BUILD_HOOK=$hook
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
for v in sargent inness default; do [ -e "$work/built-$v" ] || { echo "the $v build never reached the hook" >&2; exit 1; }; done
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
  for b in sargent inness tadema tonn hopper "tube box"; do
    [ "$b" = "$p" ] || { [ "$b" = "tube box" ] && [ "$p" = blank ]; } || others+=("$b")
  done
  hit=$(grep -ioE "$(IFS='|'; echo "${others[*]}")" <<<"$(strings "$e")" | sort -u | tr '\n' ' ' || true)
  [ -z "$hit" ] || { echo "$p: its easel's strings name other boxes: $hit" >&2; exit 1; }
  echo "$p: ok (boxes: $known)"
done
echo "export_concurrent: three exports at once from an empty build directory each got their own box's easel ($work)"
