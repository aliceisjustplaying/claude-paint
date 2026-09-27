#!/usr/bin/env bash
# painter_text.diff: every text a round 19 painter can read, round 17/18 (old) against round 19 (new).
# Studio names in the rendered briefs are normalized to paint-studio-000000 so only the text differs.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
g=$(dirname "$here")
old_repo=~/src/a/claude-paint-r17-base
new_repo=~/src/a/claude-paint-r19-base
w=$(mktemp -d "${TMPDIR:?}/painter-text.XXXXXX")
mkdir -p "$w/old" "$w/new"

norm() { sed -E 's/paint-studio-[0-9a-f]{6}/paint-studio-000000/g; s/studio-[0-9a-f]{6}/studio-000000/g'; }

# briefs, as rendered
norm < "$g/r17/run/F/p1_brief.md" > "$w/old/BRIEF.md (Friedrich lane; r17 lane F, June)"
norm < "$here/briefs/friedrich.md" > "$w/new/BRIEF.md (Friedrich lane; r17 lane F, June)"
norm < "$g/r17/run/O/p1_brief.md" > "$w/old/BRIEF.md (blank lane; r17 lane O, r18, r18g)"
norm < "$here/briefs/blank.md" > "$w/new/BRIEF.md (blank lane; r17 lane O, r18, r18g)"

# the runner's messages to the painter
py() { python3 -c "import ast,sys; t=ast.parse(open(sys.argv[1]).read()); print(next(ast.literal_eval(n.value) for n in t.body if isinstance(n, ast.Assign) and n.targets[0].id == sys.argv[2]))" "$@"; }
py "$g/r17/r17_chains.py" PAINTER_MSG > "$w/old/message: sitting 1 (launch)"
py "$here/r19_chains.py" PAINTER_MSG > "$w/new/message: sitting 1 (launch)"
py "$g/r17/r17_chains.py" SITTING_MESSAGE > "$w/old/message: sittings 2 and later"
py "$here/r19_chains.py" SITTING_MESSAGE > "$w/new/message: sittings 2 and later"

# the compaction summary's header
hdr() { node -e '
  const s = require("fs").readFileSync(process.argv[1], "utf8");
  const m = s.match(/const HEADER =\s*"([^"]*)"/);
  console.log(m[1]);' "$1"; }
hdr "$old_repo/harness/painter/compaction.ts" > "$w/old/compaction summary: header line"
hdr "$new_repo/harness/painter/compaction.ts" > "$w/new/compaction summary: header line"

# notes in the studio
for f in notes/easel_guide.md notes/research/oil_paint_physics.md notes/research/friedrich_materials.md; do
  git -C "$old_repo" show "r17-base:$f" > "$w/old/$(basename "$f")"
  git -C "$new_repo" show "r19-base:$f" > "$w/new/$(basename "$f")"
done
cp "$g/r17/trees.md" "$w/old/trees.md (Friedrich studio)"
cp "$here/trees.md" "$w/new/trees.md (Friedrich studio)"
cp "$g/r17/studio_notes.md" "$w/old/studio_notes.md (base)"
cp "$here/studio_notes.md" "$w/new/studio_notes.md (base)"
# what the reader is asked for (its record goes into the next painter's studio_notes.md)
cp "$g/r17/reader_brief.md" "$w/old/reader_brief.md (reader; its record reaches the next painter)"
cp "$here/reader_brief.md" "$w/new/reader_brief.md (reader; its record reaches the next painter)"

# the system prompt, and the tools as the model is told about them (name and description)
cp "$old_repo/harness/painter/system_prompt.md" "$w/old/system_prompt.md"
cp "$new_repo/harness/painter/system_prompt.md" "$w/new/system_prompt.md"
printf 'bash: pi'"'"'s built-in shell tool\nread: pi'"'"'s built-in read tool (any file)\n' > "$w/old/tools (names and descriptions)"
node -e '
  const s = require("fs").readFileSync(process.argv[1], "utf8");
  for (const m of s.matchAll(/name: "(\w+)",[\s\S]*?description:\s*([\s\S]*?),\n\t\t\tparameters/g))
    console.log(m[1] + ": " + eval(m[2]));
  console.log("read: pi\x27s built-in read tool, refused outside the studio");
' "$new_repo/harness/painter/easel-tools.ts" > "$w/new/tools (names and descriptions)"

# the painter build's help and the replies round 19 changed, as the painter build prints them
# (capture_easel_replies.sh; r17's painter easel is target/studio-build/5656ef2: crates/ unchanged
# from there to r17-base's tip). EASEL_NEW: the r19-base painter build.
tm='s/\([0-9.]* s to compute\)/(… s to compute)/'
"$here/capture_easel_replies.sh" "$old_repo/target/studio-build/5656ef2/target/release/easel" | sed -E "$tm" > "$w/old/bin-easel replies (painter build)"
"$here/capture_easel_replies.sh" "${EASEL_NEW:?the r19-base painter build}" | sed -E "$tm" > "$w/new/bin-easel replies (painter build)"

cd "$w"
: > "$here/painter_text.diff"
for f in "system_prompt.md" "tools (names and descriptions)" "BRIEF.md (Friedrich lane; r17 lane F, June)" "BRIEF.md (blank lane; r17 lane O, r18, r18g)" \
         "message: sitting 1 (launch)" "message: sittings 2 and later" "compaction summary: header line" \
         "easel_guide.md" "oil_paint_physics.md" "friedrich_materials.md" "trees.md (Friedrich studio)" \
         "studio_notes.md (base)" "bin-easel replies (painter build)" \
         "reader_brief.md (reader; its record reaches the next painter)"; do
  diff -u --label "old/$f" --label "new/$f" "old/$f" "new/$f" >> "$here/painter_text.diff" || true
done
echo "$here/painter_text.diff ($(wc -l < "$here/painter_text.diff") lines; files in $w)"
