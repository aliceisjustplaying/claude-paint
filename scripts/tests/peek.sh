#!/bin/sh
# Requires ImageMagick and a caller-provided persistent scratch directory.
set -eu
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
work="$TMPDIR/peek-test-$$"
mkdir -p "$work"
repo=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
magick -size 1800x1700 xc:gray "$work/source.png"
"$repo/scripts/peek" "$work/source.png" "$work/whole.png"
[ "$(magick identify -format '%m %w' "$work/whole.png")" = 'PNG 1600' ]
[ "$(wc -c < "$work/whole.png")" -lt 3000000 ]
magick -size 1800x1700 xc:gray -seed 1 +noise Random "$work/noise.png"
"$repo/scripts/peek" "$work/noise.png" "$work/noise-view.png"
[ "$(magick identify -format '%w' "$work/noise-view.png")" -lt 1600 ]
[ "$(wc -c < "$work/noise-view.png")" -lt 3000000 ]
"$repo/scripts/peek" "$work/source.png" "$work/crop.png" 300 400 100 200
[ "$(magick identify -format '%m %w %h' "$work/crop.png")" = 'PNG 400 300' ]
if "$repo/scripts/peek" "$work/source.png" "$work/bad.jpg"; then exit 1; fi
if "$repo/scripts/peek" "$work/source.png" "$work/large.png" 1201 400 0 0; then exit 1; fi
printf 'peek: whole PNG, byte limit, 1:1 crop and invalid-output checks passed\n'
