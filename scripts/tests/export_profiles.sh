#!/usr/bin/env bash
# Export every studio profile from a branch and check each from outside:
# the export's own checks pass, the studio holds its box (bin/box) or none,
# the guide's tube table is what the studio's easel prints for its box, the
# guides match the committed guide after the studio's tool rewrite, and a
# box studio's guide also differs in the tube table.
#
#   R16_BRANCH=<branch> scripts/tests/export_profiles.sh [profile...]
#
# By default every profile; giverny and impressionist only if R16_BRANCH has
# their materials notes (an older branch lacks them).
#
# Requires a caller-provided persistent scratch directory (TMPDIR). Builds
# one painter easel per box (a few minutes the first time).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
: "${R16_BRANCH:?set R16_BRANCH to the branch to export}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
work=$(mktemp -d "$TMPDIR/export-profiles.XXXXXX")
unset EASEL_BOX
head='| tube | pigment | hiding | stiffness | tinting strength | drying |'
table_of() { awk -v head="$head" '$0 == head { on = 1 } on && !/^\|/ { exit } on { print }' "$1"; }
# the profiles named on the command line, else all of them (the two newest only where R16_BRANCH has them)
profiles=("$@")
if [ ${#profiles[@]} -eq 0 ]; then
  for p in blank friedrich sargent inness alma-tadema tonn hopper giverny impressionist seurat cezanne; do
    case $p in
      giverny|impressionist|seurat|cezanne) git -C "$repo" cat-file -e "$R16_BRANCH:notes/research/${p}_materials.md" 2>/dev/null || continue ;;
    esac
    profiles+=("$p")
  done
fi
for profile in "${profiles[@]}"; do
  dest=$work/$profile
  "$repo/scripts/export_r16_studio" "$profile" "$dest" >"$work/$profile.log" 2>&1 || { cat "$work/$profile.log" >&2; echo "$profile: the export failed" >&2; exit 1; }
  git -C "$repo" show "$R16_BRANCH:notes/easel_guide.md" > "$work/$profile.guide.md"
  uv run --no-project python "$repo/scripts/studio_guide.py" "$work/$profile.guide.md" --studio "$dest"
  committed=$(cat "$work/$profile.guide.md")
  case $profile in blank) box= ;; *) box=$profile ;; esac
  if [ -z "$box" ]; then
    [ ! -e "$dest/bin/box" ] || { echo "$profile: has a bin/box" >&2; exit 1; }
    [ "$(cat "$dest/notes/easel_guide.md")" = "$committed" ] || { echo "$profile: the guide isn't the committed one" >&2; exit 1; }
  else
    [ "$(cat "$dest/bin/box")" = "$box" ] || { echo "$profile: bin/box isn't $box" >&2; exit 1; }
    if [ "$profile" = tonn ] && ! git -C "$repo" cat-file -e "$R16_BRANCH:notes/research/tonn_materials.md" 2>/dev/null; then
      [ ! -e "$dest/notes/research/tonn_materials.md" ] || { echo "$profile: unexpected materials note" >&2; exit 1; }
    else
      [ -s "$dest/notes/research/${profile//-/_}_materials.md" ] || { echo "$profile: no materials note" >&2; exit 1; }
    fi
    ! grep -lE '^## Sources|\[[A-Z]{2}[^]]*\]' "$dest"/notes/research/*.md || { echo "$profile: sources left in a painter's note" >&2; exit 1; }
    # the guide outside the table is the committed guide's
    strip() { awk -v head="$head" '$0 == head { skip = 1; next } skip && /^\|/ { next } { skip = 0; print }'; }
    [ "$(strip < "$dest/notes/easel_guide.md")" = "$(strip <<<"$committed")" ] || { echo "$profile: the guide differs outside its tube table" >&2; exit 1; }
  fi
  shown=$(table_of "$dest/notes/easel_guide.md")
  printed=$(cd / && env -u EASEL_BOX "$dest/bin/easel" tubes --markdown)
  [ "$shown" = "$printed" ] || { echo "$profile: the guide's table isn't the easel's box" >&2; exit 1; }
  case $profile in
    cezanne) techniques=notes/cezanne_techniques.md; physics=notes/research/cezanne_oil_paint_physics.md ;;
    *) techniques=notes/techniques.md; physics=notes/research/oil_paint_physics.md ;;
  esac
  if git -C "$repo" cat-file -e "$R16_BRANCH:$techniques" 2>/dev/null; then
    git -C "$repo" show "$R16_BRANCH:$techniques" | cmp -s - "$dest/notes/techniques.md" || { echo "$profile: the techniques aren't the profile's committed copy" >&2; exit 1; }
  fi
  git -C "$repo" show "$R16_BRANCH:$physics" > "$work/$profile.physics.md"
  "$repo/scripts/strip_sources" "$work/$profile.physics.md"
  cmp -s "$work/$profile.physics.md" "$dest/notes/research/oil_paint_physics.md" || { echo "$profile: the physics note isn't the profile's stripped committed copy" >&2; exit 1; }
  [ ! -e "$dest/notes/cezanne_techniques.md" ] && [ ! -e "$dest/notes/research/cezanne_oil_paint_physics.md" ] || { echo "$profile: private source filenames leaked into the studio" >&2; exit 1; }
  echo "$profile: ok (${box:-default box}, $(($(wc -l <<<"$printed") - 2)) tubes)"
done
echo "export_profiles: every profile exported and checked in $work"
