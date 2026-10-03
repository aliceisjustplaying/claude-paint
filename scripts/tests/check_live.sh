#!/usr/bin/env bash
# scripts/check_painting judges a painting's replay against the live canvas
# and the save file its session wrote when it closed: equal is "check: ok",
# either different fails, and with neither it only claims "replay only".
#
#   scripts/tests/check_live.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR). Uses the
# replay build of this checkout, built if needed (scripts/replay_easel).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
# short: the session sockets under it must fit a socket path (104 bytes on macOS)
work=$(mktemp -d "$TMPDIR/cl.XXXXXX")
easel=$("$repo/scripts/replay_easel")
[ -x "$easel" ] || { echo "check_live: no easel at $easel" >&2; exit 1; }
fail() { echo "check_live: $1; see $2" >&2; exit 1; }

# a two-chunk painting, closed so its session saves the live canvas
paint() {
  local studio=$work/$1
  mkdir -p "$studio/paintings/lua" "$studio/notes"
  E() { (cd "$studio" && EASEL_ROOT="$studio" EASEL_SESSION=painting "$easel" "$@"); }
  trap 'E close >/dev/null 2>&1 || true' EXIT
  E open painting >/dev/null
  E do 'canvas{size=300, aspect=1.25, seed=3, linen=15, ground={{pile={{"lead white", 4}, {"red earth", 1}}, um=80, apply="knife"}}}' >/dev/null
  E do "b = brush('round', 4); b:load(pile{{'bone black', 1}}, 0.8); b:stroke({{200, $2}, {800, 320}})" >/dev/null
  E close >/dev/null
  trap - EXIT
}
paint a 300
paint b 500
live=$work/a/out/easel/painting/live.png
[ -s "$live" ] || fail "closing the session saved no live canvas" "$work/a"
save=$work/a/out/easel/painting/live.ckpt
[ -s "$save" ] || fail "closing the session wrote no save file" "$work/a"

check() { # <tag>: runs check_painting on studio a; prints its exit code
  local rc=0
  "$repo/scripts/check_painting" "$work/a" "$work/c-$1" >"$work/c-$1.log" 2>&1 || rc=$?
  echo $rc
}
[ "$(check same)" = 0 ] && grep -q '^check: ok' "$work/c-same.log" || fail "a replay equal to the live canvas didn't pass" "$work/c-same.log"

cp "$live" "$work/a-live.png"
cp "$work/b/out/easel/painting/live.png" "$live"
[ "$(check other)" = 1 ] && grep -q '^check: DIFFERS from the live canvas' "$work/c-other.log" || fail "a replay unlike the live canvas passed" "$work/c-other.log"

cp "$work/a-live.png" "$live"
cp "$save" "$work/a-live.ckpt"
cp "$work/b/out/easel/painting/live.ckpt" "$save"
[ "$(check othersave)" = 1 ] && grep -q '^check: DIFFERS from the live save' "$work/c-othersave.log" || fail "a replay unlike the live save passed" "$work/c-othersave.log"

zstd -q --rm "$work/a-live.ckpt" -o "$save.zst" 2>/dev/null || gzip -c "$work/a-live.ckpt" > "$save.gz"
rm -f "$save" "$work/a-live.ckpt"
[ "$(check compressed)" = 0 ] && grep -q "the replay's canvas state equals the save" "$work/c-compressed.log" || fail "a replay equal to the compressed live save didn't pass" "$work/c-compressed.log"

rm "$live" "$save".*
[ "$(check none)" = 0 ] && grep -q '^check: replay only' "$work/c-none.log" || fail "without a live canvas or save the check didn't say replay only" "$work/c-none.log"
echo "check_live: check_painting passed the live canvas and save (also compressed), failed another canvas and another save, and said replay only without either ($work)"
