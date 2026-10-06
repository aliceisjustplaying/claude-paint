#!/bin/bash
# end_first.sh <in.mp4> <out.mp4>: the clip with its own last frame held for its first 0.5 s
# (24 fps, H.264 yuv420p, faststart, as the clips are), then the clip as it was.
set -euo pipefail
in=$1; out=$2; t=$(mktemp -d)
ffmpeg -v error -y -sseof -0.05 -i "$in" -frames:v 1 -update 1 "$t/last.png"
ffmpeg -v error -y -loop 1 -framerate 24 -t 0.5 -i "$t/last.png" -i "$in" \
  -filter_complex "[0:v]format=yuv420p,setsar=1[a];[1:v]format=yuv420p,setsar=1[b];[a][b]concat=n=2:v=1[v]" \
  -map "[v]" -r 24 -c:v libx264 -crf 18 -pix_fmt yuv420p -movflags +faststart "$out"
rm -rf "$t"
