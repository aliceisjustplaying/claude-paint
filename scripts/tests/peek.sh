#!/bin/sh
# Requires ImageMagick and a caller-provided persistent scratch directory.
set -eu
if command -v magick >/dev/null 2>&1; then
  image_tool=magick
elif command -v convert >/dev/null 2>&1; then
  image_tool=convert
else
  echo 'peek test requires ImageMagick 6 or 7' >&2
  exit 127
fi
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
work="$TMPDIR/peek-test-$$"
mkdir -p "$work"
repo=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
"$image_tool" -size 1800x1700 xc:gray "$work/source.png"
"$repo/scripts/peek" "$work/source.png" "$work/whole.png"
[ "$("$image_tool" "$work/whole.png" -format '%m %w' info:)" = 'PNG 1600' ]
[ "$(wc -c < "$work/whole.png")" -lt 3000000 ]
"$image_tool" -size 1800x1700 xc:gray -seed 1 +noise Random "$work/noise.png"
"$repo/scripts/peek" "$work/noise.png" "$work/noise-view.png"
[ "$("$image_tool" "$work/noise-view.png" -format '%w' info:)" -lt 1600 ]
[ "$(wc -c < "$work/noise-view.png")" -lt 3000000 ]
"$repo/scripts/peek" "$work/source.png" "$work/crop.png" 300 400 100 200
[ "$("$image_tool" "$work/crop.png" -format '%m %w %h' info:)" = 'PNG 400 300' ]
if "$repo/scripts/peek" "$work/source.png" "$work/bad.jpg"; then exit 1; fi
if "$repo/scripts/peek" "$work/source.png" "$work/large.png" 1201 400 0 0; then exit 1; fi
printf 'peek: whole PNG, byte limit, 1:1 crop and invalid-output checks passed\n'
