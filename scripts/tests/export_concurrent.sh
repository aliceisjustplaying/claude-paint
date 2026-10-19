#!/usr/bin/env bash
# Exports of different profiles at once, from an empty build directory
# (nothing extracted, no easel cached): one build serves all studios,
# while each copy exposes only its configured runtime box.
#
#   R16_BRANCH=<branch> scripts/tests/export_concurrent.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR). Builds
# one shared easel from nothing (several minutes).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
: "${R16_BRANCH:?set R16_BRANCH to the branch to export}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
work=$(mktemp -d "$TMPDIR/export-concurrent.XXXXXX")
unset EASEL_BOX
export STUDIO_BUILDS=$work/builds
hook=$work/hook
cat > "$hook" <<HOOK
#!/usr/bin/env bash
set -eu
echo "\$1" >> "$work/builds-called"
sleep 1
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
[ "$(cat "$work/builds-called")" = universal ] || { echo "expected exactly one universal build" >&2; exit 1; }
for p in "${profiles[@]}"; do
  case $p in blank) box=none ;; *) box=$p ;; esac
  e=$work/$p/bin/easel
  cmp -s "$e" "$work/sargent/bin/easel" || { echo "$p: binary differs from sargent's" >&2; exit 1; }
  [ "$(cat "$work/$p/bin/studio")" = "painter $box" ] || { echo "$p: wrong runtime selection" >&2; exit 1; }
  table=$(cd / && "$e" tubes --markdown)
  probe=$work/$p-probe
  mkdir -p "$probe/bin"
  cp "$e" "$probe/bin/easel"
  cp "$work/$p/bin/studio" "$probe/bin/studio"
  echo no-such-box > "$probe/bin/box"
  isolated=$(cd / && EASEL_BOX=no-such-box "$probe/bin/easel" tubes --markdown)
  [ "$isolated" = "$table" ] || { echo "$p: metadata or environment changed its runtime box" >&2; exit 1; }
  cp "$work/inness/bin/studio" "$probe/bin/studio"
  selected=$(cd / && "$probe/bin/easel" tubes --markdown)
  inness=$(cd / && "$work/inness/bin/easel" tubes --markdown)
  [ "$selected" = "$inness" ] || { echo "$p: adjacent selection didn't select inness" >&2; exit 1; }
  echo "$p: ok (shared binary, isolated $box selection)"
done
sargent=$(cd / && "$work/sargent/bin/easel" tubes --markdown)
blank=$(cd / && "$work/blank/bin/easel" tubes --markdown)
[[ "$sargent" != "$inness" && "$blank" != "$inness" && "$sargent" != "$blank" ]] || { echo "runtime box tables aren't distinct" >&2; exit 1; }
echo "export_concurrent: three exports shared one build with isolated runtime selections ($work)"
