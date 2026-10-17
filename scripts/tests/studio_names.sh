#!/usr/bin/env bash
# scripts/check_studio_names on fixture studios:
# - names match case-insensitively as whole words, with a space, a hyphen or
#   nothing between their parts ("da-Vinci", "alma tadema", "WINSOR"), and
#   not inside other words ("convincing");
# - the studio's own artist is exempt, and only that artist;
# - accented names in notes are checked.
#
#   scripts/tests/studio_names.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
check=$repo/scripts/check_studio_names
work=$(mktemp -d "$TMPDIR/studio-names.XXXXXX")

# studio <name> <note text>: a studio with a fixture note
studio() {
  local d=$work/$1
  mkdir -p "$d/bin" "$d/notes"
  printf '%s\n' "$2" > "$d/notes/note.md"
  echo "$d"
}
# expect <pass|fail> <studio> [own ...]
expect() {
  local want=$1 d=$2 got
  shift 2
  if "$check" "$d" "$@" >"$d.out" 2>&1; then got=pass; else got=fail; fi
  [ "$got" = "$want" ] || { echo "studio_names: $(basename "$d") with own=($*): expected $want, got $got" >&2; cat "$d.out" >&2; exit 1; }
}

# spellings of names in the notes
for text in "a da-Vinci guide" "DaVinci brushes" "da Vinci" "Leonardo DA VINCI" "Alma Tadema's panels" "alma-tadema" "WINSOR & Newton" "winsor-newton" "as Inness did" "Sargent-like"; do
  d=$(studio "note-$(tr -c 'A-Za-z0-9' _ <<<"$text")" "$text")
  expect fail "$d" Tonn
done
# not inside other words
d=$(studio clean "a convincing lace of twigs; tonnage; winsome")
expect pass "$d" Sargent
# the own artist is exempt, and only that one
d=$(studio own-sargent "as Sargent did, in a Sargent-like way")
expect pass "$d" Sargent
d=$(studio own-tadema "Alma-Tadema's and Alma Tadema's panels")
expect pass "$d" Alma-Tadema Tadema
d=$(studio own-tadema-other "Alma-Tadema, not Inness")
expect fail "$d" Alma-Tadema Tadema
# a studio's own painter by the plain spelling of an accented name in a note
d=$(studio own-accented "Cézanne's and Cezanne's whites")
expect pass "$d" Cézanne
expect fail "$d" Tonn
# names in the notes' prose are whole words
d=$(studio joined-prose "bonedaVincilead")
expect pass "$d" Sargent
d=$(studio accented-prose "Cézanne and Velázquez")
expect fail "$d" Tonn
d=$(studio box-name "Nothing here.")
printf 'Inness\n' > "$d/bin/box"
expect fail "$d" Tonn
expect pass "$d" Inness
d=$(studio binary-note "Nothing here.")
printf 'Inness\0Cézanne\0' > "$d/notes/image.bin"
expect pass "$d" Tonn
echo "studio_names: all fixture studios checked as expected ($work)"
