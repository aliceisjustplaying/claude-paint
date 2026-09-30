#!/usr/bin/env bash
# scripts/replay_clip cuts a clip from a frames folder given relative (the
# default <out>.frames beside a relative --out) or absolute (a folder name
# with a quote and a space), at numeric and dynamic pace, and the clip runs
# the --length asked for, to within a video frame (24 fps); a --length the
# holds can't fill under --max-hold is refused.
#
#   scripts/tests/replay_clip.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR), ffmpeg,
# ffprobe and uv (dynamic pace). No easel: the clips are cut with --reuse
# from three flat-color frames a minute of hand time apart (so dynamic pace
# gives the last moment, which the final hold shows, a large share).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
for t in ffmpeg ffprobe uv; do
  command -v "$t" >/dev/null || { echo "replay_clip test: needs $t; nothing tested" >&2; exit 1; }
done
work=$(mktemp -d "$TMPDIR/replay-clip.XXXXXX")

# frames.tsv as the easel writes it, and a PNG per row
frames() {
  mkdir -p "$1"
  printf 'file\thand_secs\tticks\tchunks_done\tkind\n' > "$1/frames.tsv"
  local i=0 c
  for c in white red blue; do
    i=$((i + 1))
    ffmpeg -loglevel error -f lavfi -i "color=c=$c:s=64x48" -frames:v 1 "$1/$(printf %04d "$i").png"
    printf '%04d.png\t%d\t%d\t0\ttick\n' "$i" $(( (i - 1) * 60 )) "$i" >> "$1/frames.tsv"
  done
}
fail() { echo "replay_clip test: $1; see $2" >&2; exit 1; }
# the clip runs 8 s, give or take a frame
eight() {
  local d
  d=$(ffprobe -v error -show_entries format=duration -of csv=p=0 "$1")
  awk -v d="$d" 'BEGIN { exit !(d - 8 <= 1 / 24 + 1e-6 && 8 - d <= 1 / 24 + 1e-6) }' || fail "$3: the clip runs $d s, not 8" "$2"
}

for pace in 0.6 dynamic; do
  # relative: the default frames folder beside a relative --out
  mkdir -p "$work/rel-$pace"
  frames "$work/rel-$pace/clip.mp4.frames"
  (cd "$work/rel-$pace" && "$repo/scripts/replay_clip" none.lua clip.mp4 --reuse --pace "$pace" --length 8 --max-hold 10 --width 64) \
    >"$work/rel-$pace.log" 2>&1 || fail "relative frames folder, pace $pace: failed" "$work/rel-$pace.log"
  eight "$work/rel-$pace/clip.mp4" "$work/rel-$pace.log" "relative frames folder, pace $pace"
  # absolute, with a quote and a space in the folder's name
  abs="$work/it's $pace"
  frames "$abs/frames"
  "$repo/scripts/replay_clip" none.lua "$abs/clip.mp4" --reuse --frames-dir "$abs/frames" --pace "$pace" --length 8 --max-hold 10 --width 64 \
    >"$work/abs-$pace.log" 2>&1 || fail "absolute frames folder, pace $pace: failed" "$work/abs-$pace.log"
  eight "$abs/clip.mp4" "$work/abs-$pace.log" "absolute frames folder, pace $pace"
  # a --length the moments can't fill at --max-hold 1 is refused, with the
  # longest the cap allows, and no clip is cut
  if (cd "$work/rel-$pace" && "$repo/scripts/replay_clip" none.lua long.mp4 --reuse --frames-dir clip.mp4.frames --pace "$pace" --length 75 --max-hold 1 --width 64) \
    >"$work/long-$pace.log" 2>&1; then fail "--length 75 at --max-hold 1, pace $pace: not refused" "$work/long-$pace.log"; fi
  grep -q 'is longer than the clip can run: .* come to at most [0-9.]* s; raise --max-hold to at least [0-9.]* or lower --length' "$work/long-$pace.log" \
    || fail "--length 75 at --max-hold 1, pace $pace: no message with the longest length and the options" "$work/long-$pace.log"
  [ ! -e "$work/rel-$pace/long.mp4" ] || fail "--length 75 at --max-hold 1, pace $pace: a clip was cut" "$work/long-$pace.log"
done
echo "replay_clip test: relative and absolute frames folders at numeric and dynamic pace, each clip 8 s to a frame; 75 s at --max-hold 1 refused ($work)"
