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
# The schedule module's numeric contract: stdlib only, the last frame at
# each hand time, unquantized durations, quoted names and a final-first
# prefix. Dynamic-only opening flags must not affect numeric pace.
mkdir "$work/schedule"
printf 'file\thand_secs\tticks\tchunks_done\tkind\n0001.png\t0\t1\t0\ttick\n0002.png\t60\t2\t0\ttick\nit\047s red.png\t60\t3\t0\tchunk\n0003.png\t120\t4\t0\ttick\nit\047s blue.png\t120\t5\t0\tfinal\n' > "$work/schedule/frames.tsv"
uv venv -q "$work/stdlib"
uv run --no-project --python "$work/stdlib/bin/python" python -S "$repo/scripts/replay_pace.py" "$work/schedule" \
  --pace 1 --length 8.12346 --max-hold 10 --final-first 1.25 --ramp ignored --ramp-span ignored --open-hold ignored --floor ignored --gamma ignored \
  >"$work/schedule.log" 2>&1 || fail "stdlib numeric schedule: failed" "$work/schedule.log"
cat > "$work/expected.txt" <<'EOF_CONCAT'
file 'it'\''s blue.png'
duration 1.25000
file '0001.png'
duration 1.93673
file 'it'\''s red.png'
duration 1.93673
file 'it'\''s blue.png'
duration 2.91667
file 'it'\''s blue.png'
duration 0.04167
file 'it'\''s blue.png'
EOF_CONCAT
cmp "$work/expected.txt" "$work/schedule/concat.txt" || fail "stdlib numeric schedule: concat bytes differ" "$work/schedule/concat.txt"
# Numeric refusals keep awk's conversion/domain behavior and exit status,
# including negative scientific exponents passed through replay_clip.
while IFS='|' read -r name pace length cap message; do
  rc=0
  "$repo/scripts/replay_clip" none.lua "$work/$name.mp4" --reuse --frames-dir "$work/rel-0.6/clip.mp4.frames" \
    --pace "$pace" --length "$length" --max-hold "$cap" --width 64 >"$work/$name.log" 2>&1 || rc=$?
  [ "$rc" = 1 ] || fail "$name: exit $rc, not refusal exit 1" "$work/$name.log"
  printf '%s\n' "$message" > "$work/refusal.txt"
  cmp "$work/refusal.txt" "$work/$name.log" || fail "$name: refusal text changed" "$work/$name.log"
  [ ! -e "$work/$name.mp4" ] || fail "$name: a clip was cut" "$work/$name.log"
done <<'EOF_REFUSALS'
negative-pace|-1|8|10|replay_clip: --length 8 is longer than the clip can run: 0 moments held at most 10 s each (--max-hold) plus the 3 s final hold come to at most 3.00 s; lower --length
scientific-pace|-1e-3|8|10|replay_clip: --length 8 is longer than the clip can run: 0 moments held at most 10 s each (--max-hold) plus the 3 s final hold come to at most 3.00 s; lower --length
invalid-length|0.6|bogus|10|replay_clip: --length must exceed the 3 s final hold
invalid-cap|0.6|8|bogus|replay_clip: --length 8 is longer than the clip can run: 2 moments held at most bogus s each (--max-hold) plus the 3 s final hold come to at most 3.00 s; raise --max-hold to at least 2.50 or lower --length
overflow-pace|10000|8|10|replay_clip: the holds come to 0.000 s, not 5.000
EOF_REFUSALS

# CR and Unicode line separators are filename bytes, not TSV records.
printf 'file\thand_secs\nwhite\342\200\250\r.png\t0\nit\047s red.png\t60\nblue.png\t120\n' > "$work/schedule/frames.tsv"
uv run --no-project --python "$work/stdlib/bin/python" python -S "$repo/scripts/replay_pace.py" "$work/schedule" \
  --pace=1 --length=8 --max-hold=10 >"$work/lf-records.log" 2>&1 || fail "LF-only records: failed" "$work/lf-records.log"
printf "file 'white\342\200\250\r.png'\nduration 2.50000\nfile 'it'\\\\''s red.png'\nduration 2.50000\nfile 'blue.png'\nduration 2.91667\nfile 'blue.png'\nduration 0.04167\nfile 'blue.png'\n" > "$work/lf-expected.txt"
cmp "$work/lf-expected.txt" "$work/schedule/concat.txt" || fail "LF-only records: concat bytes differ" "$work/schedule/concat.txt"
echo "replay_clip test: relative and absolute frames folders at numeric and dynamic pace, each clip 8 s to a frame; 75 s at --max-hold 1 refused; stdlib numeric schedule, refusal semantics and LF-only records exact ($work)"
