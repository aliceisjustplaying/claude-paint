#!/usr/bin/env bash
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
work=$(mktemp -d "$TMPDIR/finish-policy.XXXXXX")
easel=$("$repo/scripts/replay_easel")

for box in hopper friedrich default; do
  studio=$work/$box
  mkdir -p "$studio/paintings/lua"
  E() {
    (cd "$studio"
     if [ "$box" = default ]; then unset EASEL_BOX; else export EASEL_BOX="$box"; fi
     EASEL_ROOT="$studio" EASEL_SESSION=painting "$easel" "$@")
  }
  trap 'E close >/dev/null 2>&1 || true' EXIT
  E open painting >/dev/null
  E do 'canvas{size=300, aspect=1, seed=3, linen=15, ground={{pile={{"lead white", 4}, {"yellow ochre", 1}}, um=80, apply="knife"}}}' >/dev/null
  E do 'b = brush("round", 4); b:load(pile{{"bone black", 1}}, 0.8); b:stroke({{200, 300}, {800, 320}})' >/dev/null
  E close >/dev/null
  trap - EXIT
  log=$studio/paintings/lua/painting.lua
  cp "$studio/out/easel/painting/live.png" "$studio/plain.png"
  save=$studio/out/easel/painting/live.ckpt
  cp "$save" "$studio/end.ckpt"
  for source in save gzip zstd replay; do
    rm -f "$save" "$save.gz" "$save.zst"
    case "$source" in
      save) cp "$studio/end.ckpt" "$save" ;;
      gzip) gzip -c "$studio/end.ckpt" > "$save.gz"; printf 'invalid save' > "$save" ;;
      zstd) command -v zstd >/dev/null || continue; zstd -q -c "$studio/end.ckpt" > "$save.zst" ;;
    esac
    out=$studio/${source}_FRDC1_finished.png
    opts=(--coats 0.4)
    [ "$box" = friedrich ] || opts+=(--relief)
    "$repo/scripts/finish_painting" "$log" "$out" "${opts[@]}" >"$studio/$source.log" 2>&1
    if [ "$box" = friedrich ]; then
      grep -q 'when_dry(function() varnish{coats=0.4} end)' "${out%.png}.lua"
      grep -q 'when_dry(function() cracks{} end)' "${out%.png}.lua"
      if cmp -s "$studio/plain.png" "$out"; then
        echo "finish_policy: Friedrich finishing did not change the picture" >&2
        exit 1
      fi
    else
      cmp "$log" "${out%.png}.lua"
      cmp "$studio/plain.png" "$out"
    fi
    if [ "$source" = replay ]; then
      grep -q 'no usable matching saved end state; replaying the full log' "$studio/$source.log"
    else
      grep -q 'finished from the save:' "$studio/$source.log"
    fi
    cmp "$studio/save_FRDC1_finished.png" "$out"
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

studio=$work/default
log=$studio/paintings/lua/painting.lua
awk '/^--@ chunk 1$/ { print "  " $0; print "--@ box friedrich"; next } { print }' "$log" > "$log.part"
mv "$log.part" "$log"
"$repo/scripts/finish_painting" "$log" "$studio/indented.png" >"$studio/indented.log" 2>&1
cmp "$log" "$studio/indented.lua"
cmp "$studio/plain.png" "$studio/indented.png"
echo "finish_policy: indented chunk boundary passed"
