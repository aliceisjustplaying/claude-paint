#!/usr/bin/env bash
# The runner's replay scripts take a painting's box from its log, whatever
# EASEL_BOX the caller has set: a sargent-box painting goes through
# scripts/check_painting, scripts/finish_painting and scripts/replay_clip
# with EASEL_BOX=inness in their environment.
#
#   scripts/tests/replay_env.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR) and ffmpeg
# (for replay_clip; without it the test fails before it starts). Uses the
# replay build of this checkout, built if needed in the target directory
# cargo builds into (CARGO_TARGET_DIR too; scripts/replay_easel), the easel
# the three scripts use.
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
command -v ffmpeg >/dev/null || { echo "replay_env: needs ffmpeg (replay_clip is one of the three scripts it tests); nothing tested" >&2; exit 1; }
work=$(mktemp -d "$TMPDIR/replay-env.XXXXXX")
easel=$("$repo/scripts/replay_easel")
[ -x "$easel" ] || { echo "replay_env: no easel at $easel" >&2; exit 1; }

# a two-chunk painting from the sargent box, with tubes the default box doesn't have
studio=$work/studio
mkdir -p "$studio/paintings/lua" "$studio/notes"
E() { (cd "$studio" && EASEL_ROOT="$studio" EASEL_SESSION=painting EASEL_BOX=sargent "$easel" "$@"); }
trap 'E close >/dev/null 2>&1 || true' EXIT
E open painting >/dev/null
E do 'canvas{size=300, aspect=1.25, seed=3, linen=15, ground={{pile={{"lead white", 4}, {"cadmium red", 1}}, um=80, apply="knife"}}}' >/dev/null
E do 'b = brush("round", 4); b:load(pile{{"emerald green", 1}}, 0.8); b:stroke({{200, 300}, {800, 320}})' >/dev/null
E save >/dev/null
E close >/dev/null
trap - EXIT
log=$studio/paintings/lua/painting.lua
[ "$(sed -n 3p "$log")" = "--@ box sargent" ] || { echo "the painting's log doesn't name the sargent box" >&2; exit 1; }

export EASEL_BOX=inness
fail() { echo "replay_env: $1 (EASEL_BOX=inness in its environment); see $2" >&2; exit 1; }
"$repo/scripts/check_painting" "$studio" "$work/check" >"$work/check.log" 2>&1 || fail "check_painting failed" "$work/check.log"
grep -q '^check: ok' "$work/check.log" || fail "check_painting didn't pass" "$work/check.log"
grep -q "the replay's PNG equals the painter's last save" "$work/check.log" || fail "check_painting's replay differs from the save" "$work/check.log"
"$repo/scripts/finish_painting" "$log" "$work/finished.png" --no-cracks >"$work/finish.log" 2>&1 || fail "finish_painting failed" "$work/finish.log"
[ -s "$work/finished.png" ] || fail "finish_painting wrote no picture" "$work/finish.log"
"$repo/scripts/replay_clip" "$log" "$work/clip.mp4" --every 1 --length 5 --width 300 >"$work/clip.log" 2>&1 || fail "replay_clip failed" "$work/clip.log"
[ -s "$work/clip.mp4" ] || fail "replay_clip wrote no clip" "$work/clip.log"
echo "replay_env: check_painting, finish_painting and replay_clip replayed a sargent-box log with EASEL_BOX=inness set ($work)"
