#!/usr/bin/env bash
# scripts/check_box_tubes on fixture easels (text files standing in for the
# binary's strings) and fixture boxes:
# - a box that holds vermilion: an "orange vermilion" or a "Chinese
#   vermilion" in the easel is an outside tube, though it holds an inside
#   name (the check matches outside names against the strings as they are);
# - a box that holds orange vermilion: the vermilion inside its own record
#   isn't one, a vermilion on its own is;
# - an outside name inside an inside tube's pigment (Prussian blue in
#   Antwerp blue's) isn't one, the same name on its own is;
# - a clean easel passes.
#
#   scripts/tests/box_tubes.sh
#
# Requires a caller-provided persistent scratch directory (TMPDIR).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
check=$repo/scripts/check_box_tubes
work=$(mktemp -d "$TMPDIR/box-tubes.XXXXXX")

printf '%s\n' "lead white" vermilion "orange vermilion" "Chinese vermilion" "Prussian blue" "Antwerp blue" "bone black" > "$work/catalog"
head='| tube | pigment | hiding | stiffness | tinting strength | drying |
|---|---|---|---|---|---|'
# a box with vermilion (and not the other two vermilions)
printf '%s\n' "$head" '| lead white | basic lead carbonate | 0.82 | 0.8 | 1 | fast |' '| vermilion | mercuric sulfide | 0.9 | 0.6 | 0.8 | slow |' > "$work/vermilion.md"
# a box with orange vermilion (and not plain vermilion)
printf '%s\n' "$head" '| lead white | basic lead carbonate | 0.82 | 0.8 | 1 | fast |' '| orange vermilion | mercuric sulfide, orange shade | 0.9 | 0.6 | 0.8 | slow |' > "$work/orange.md"
# a box with Antwerp blue, whose pigment names Prussian blue
printf '%s\n' "$head" '| lead white | basic lead carbonate | 0.82 | 0.8 | 1 | fast |' '| Antwerp blue | Prussian blue extended with alumina | 0.3 | 0.5 | 0.7 | medium |' > "$work/antwerp.md"

# expect <pass|fail> <table> <strings> [the outside names reported]
n=0
expect() {
  local want=$1 table=$2 text=$3 got out
  shift 3
  n=$((n + 1))
  printf '%s\n' "$text" > "$work/easel-$n"
  if out=$("$check" "$work/easel-$n" "$work/$table.md" "$work/catalog" 2>&1); then got=pass; else got=fail; fi
  [ "$got" = "$want" ] || { echo "box_tubes: $table box, easel strings [$text]: expected $want, got $got ($out)" >&2; exit 1; }
  local reported
  reported=$(sort <<<"$out" | tr '\n' '|')
  if [ $# -gt 0 ]; then
    [ "$reported" = "$(printf '%s\n' "$@" | sort | tr '\n' '|')" ] || { echo "box_tubes: $table box, easel strings [$text]: reported [$out], not [$*]" >&2; exit 1; }
  fi
}

# outside names that hold an inside one
expect fail vermilion $'lead white\nvermilion\nbasic lead carbonate\nmercuric sulfide\norange vermilion' "orange vermilion"
expect fail vermilion $'lead white\nvermilion\nmercuric sulfide\nChinese vermilion' "Chinese vermilion"
expect fail vermilion $'lead whitevermilionorange vermilionChinese vermilion' "orange vermilion" "Chinese vermilion"
expect pass vermilion $'lead white\nbasic lead carbonate\nvermilion\nmercuric sulfide'
# an inside name that holds an outside one
expect pass orange $'lead white\norange vermilion\nmercuric sulfide, orange shade'
expect fail orange $'lead white\norange vermilion\nmercuric sulfide, orange shade\nvermilion' vermilion
expect fail orange $'lead whitevermilionorange vermilion' vermilion
# an outside name in an inside tube's pigment
expect pass antwerp $'lead white\nAntwerp blue\nPrussian blue extended with alumina'
expect fail antwerp $'lead white\nAntwerp blue\nPrussian blue extended with alumina\nPrussian blue' "Prussian blue"
expect fail antwerp $'Antwerp blue\nbone black' "bone black"
echo "box_tubes: $n fixture easels checked as expected ($work)"
