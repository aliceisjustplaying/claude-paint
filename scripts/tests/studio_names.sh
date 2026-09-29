#!/usr/bin/env bash
# scripts/check_studio_names on fixture studios:
# - a name early in a long bin/easel is found (under pipefail a streaming
#   `strings | grep -q` misses it: grep's early exit kills strings with
#   SIGPIPE and the pipeline fails as if nothing matched);
# - names match case-insensitively as whole words, with a space, a hyphen or
#   nothing between their parts ("da-Vinci", "alma tadema", "WINSOR"), and
#   not inside other words ("convincing");
# - the studio's own artist is exempt, and only that artist;
# - bin/easel is read as bytes: an accented name ("C\xc3\xa9zanne" in UTF-8,
#   which `strings` cuts down to "zanne") is found, and so is a name of two
#   words run into a longer token ("bonedaVincilead"), which prose may hold.
#
#   scripts/tests/studio_names.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
check=$repo/scripts/check_studio_names
work=$(mktemp -d "$TMPDIR/studio-names.XXXXXX")

# studio <name> <note text>: a studio whose easel is a clean fixture binary
studio() {
  local d=$work/$1
  mkdir -p "$d/bin" "$d/notes"
  printf '%s\n' "$2" > "$d/notes/note.md"
  printf 'lead white\nbasic lead carbonate\n' > "$d/bin/easel"
  echo "$d"
}
# expect <pass|fail> <studio> [own ...]
expect() {
  local want=$1 d=$2 got
  shift 2
  if "$check" "$d" "$@" >"$d.out" 2>&1; then got=pass; else got=fail; fi
  [ "$got" = "$want" ] || { echo "studio_names: $(basename "$d") with own=($*): expected $want, got $got" >&2; cat "$d.out" >&2; exit 1; }
}

# a forbidden name early in a long binary
d=$(studio early "Nothing here.")
{ printf 'tube box\nsargent\nInness\n'; for i in $(seq 1 40000); do printf 'filler string number %06d for the pipe\n' "$i"; done; } > "$d/bin/easel"
set +e
strings "$d/bin/easel" | grep -qiwE 'Inness'
rc=$?
set -e
[ $rc -ne 0 ] || { echo "studio_names: the fixture didn't make the streaming grep fail (rc 0); make it longer" >&2; exit 1; }
expect fail "$d" Sargent
grep -q 'bin/easel: Inness' "$d.out" || { echo "studio_names: the early name isn't reported" >&2; cat "$d.out" >&2; exit 1; }

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
# a box name run together with a tube name in the binary
d=$(studio run-together "Nothing here.")
printf 'bone blacktonnlead white\n' > "$d/bin/easel"
expect fail "$d" Sargent
expect pass "$d" Tonn
# an accented name in the binary's bytes, between NULs
d=$(studio accented "Nothing here.")
printf 'lead white\0C\303\251zanne\0bone black\0' > "$d/bin/easel"
[ -z "$(strings "$d/bin/easel" | grep -i 'c.*zanne' || true)" ] || { echo "studio_names: strings kept the accented name; the fixture tests nothing" >&2; exit 1; }
expect fail "$d" Tonn
grep -q 'bin/easel: Cézanne' "$d.out" || { echo "studio_names: the accented name isn't reported" >&2; cat "$d.out" >&2; exit 1; }
d=$(studio accented-velazquez "Nothing here.")
printf 'lead white\0Vel\303\241zquez\0' > "$d/bin/easel"
expect fail "$d" Tonn
# a name of two words run into a longer token, and its other spellings
for token in bonedaVincilead "boneda Vincilead" leadvangoghwhite BobRossblack blackalmatademawhite; do
  d=$(studio "joined-$(tr -c 'A-Za-z0-9' _ <<<"$token")" "Nothing here.")
  printf 'lead white\n%s\n' "$token" > "$d/bin/easel"
  expect fail "$d" Sargent
done
# but not in the notes' prose, where names are whole words
d=$(studio joined-prose "bonedaVincilead")
expect pass "$d" Sargent
echo "studio_names: all fixture studios checked as expected ($work)"
