#!/usr/bin/env bash
# What a painter's easel prints for the replies round 19 changed: run a painter build in a
# throwaway studio and trigger each one. Usage: capture_easel_replies.sh <painter easel binary>
set -uo pipefail
bin=$1
s=$(mktemp -d "${TMPDIR:?}/replies.XXXXXX")
mkdir -p "$s/bin" "$s/notes" "$s/paintings/lua"
cp "$bin" "$s/bin/easel"
E="$s/bin/easel"
say() { printf '$ bin/easel %s\n' "$*"; "$E" "$@" 2>&1 | sed "s|$s|<studio>|g"; echo; }
"$E" open >/dev/null
echo "## bin/easel help"; "$E" help | sed "s|$s|<studio>|g"; echo
say check
say do 'canvas{}'
say do 'canvas{size=300, aspect=1.25, linen=15, ground={{pile={{"lead white", 1}}, um=80, apply="knife"}}}'
say do 'p = pile{}'
say do 'p = pile{"lead white"}'
say do 'work(everywhere(), {})'
log=$s/paintings/lua/painting.lua
cp "$log" "$s/keep.lua"
printf -- '-- edited\n' >> "$log"
echo "(the log edited)"; say status
rm "$log"
echo "(the log removed)"; say status
cp "$s/keep.lua" "$log"
"$E" close >/dev/null 2>&1
