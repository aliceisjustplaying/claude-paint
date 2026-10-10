#!/usr/bin/env bash
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
work=$(mktemp -d "$TMPDIR/finish-policy.XXXXXX")
easel=$("$repo/scripts/replay_easel")

for box in hopper friedrich default; do
  studio=$work/$box
  mkdir -p "$studio/paintings/lua"
  E() { (cd "$studio" && EASEL_ROOT="$studio" EASEL_SESSION=painting EASEL_BOX="$box" "$easel" "$@"); }
  trap 'E close >/dev/null 2>&1 || true' EXIT
  E open painting >/dev/null
  E do 'canvas{size=300, aspect=1, seed=3, linen=15, ground={{pile={{"lead white", 4}, {"yellow ochre", 1}}, um=80, apply="knife"}}}' >/dev/null
  E do 'b = brush("round", 4); b:load(pile{{"bone black", 1}}, 0.8); b:stroke({{200, 300}, {800, 320}})' >/dev/null
  E close >/dev/null
  trap - EXIT
  log=$studio/paintings/lua/painting.lua
  cp "$studio/out/easel/painting/live.png" "$studio/plain.png"
  for source in save replay; do
    out=$studio/${source}_FRDC1_finished.png
    "$repo/scripts/finish_painting" "$log" "$out" >"$studio/$source.log" 2>&1
    if [ "$box" = friedrich ]; then
      grep -q 'when_dry(function() varnish{coats=0.4} end)' "${out%.png}.lua"
      grep -q 'when_dry(function() cracks{} end)' "${out%.png}.lua"
      ! cmp -s "$studio/plain.png" "$out"
    else
      cmp "$log" "${out%.png}.lua"
      cmp "$studio/plain.png" "$out"
    fi
  done
  grep -q 'finished from the save:' "$studio/save.log"
  grep -q '^finished:' "$studio/replay.log"
  cmp "$studio/save_FRDC1_finished.png" "$studio/replay_FRDC1_finished.png"
  echo "finish_policy: $box save and replay passed"
done

studio=$work/friedrich
"$repo/scripts/finish_painting" "$studio/paintings/lua/painting.lua" "$studio/plain-finished.png" --no-varnish --no-cracks >"$studio/plain-finished.log" 2>&1
cmp "$studio/paintings/lua/painting.lua" "$studio/plain-finished.lua"
cmp "$studio/plain.png" "$studio/plain-finished.png"
echo "finish_policy: Friedrich opt-out passed"
