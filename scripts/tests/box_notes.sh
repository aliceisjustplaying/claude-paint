#!/usr/bin/env bash
# Check each box studio's materials note (the painter's copy, stripped of
# its sources) against the studio's box:
# (a) every tube `bin/easel tubes` prints is named in the note's pigment
#     table (§4) or in "the tubes here" (§9);
# (b) every tube §9 names is in the box: each mapping's tubes (after "→")
#     and any catalog tube named in §9;
# (c) the guide's tube table is `bin/easel tubes --markdown`.
#
#   scripts/tests/box_notes.sh [<studio>...]
#
# With no studios, exports sargent, inness, alma-tadema, tonn and hopper from
# R16_BRANCH (scripts/export_r16_studio) and checks those. The catalog's
# tube names come from R16_BRANCH's palette.rs (HEAD if unset).
#
# Requires a caller-provided persistent scratch directory (TMPDIR).
set -euo pipefail
: "${TMPDIR:?set TMPDIR to persistent scratch storage}"
repo=$(cd "$(dirname "$0")/../.." && pwd)
unset EASEL_BOX
work=$(mktemp -d "$TMPDIR/box-notes.XXXXXX")
if [ $# -eq 0 ]; then
  : "${R16_BRANCH:?set R16_BRANCH to the branch to export, or name exported studios}"
  for profile in sargent inness alma-tadema tonn hopper; do
    "$repo/scripts/export_r16_studio" "$profile" "$work/$profile" >"$work/$profile.log" 2>&1 || { cat "$work/$profile.log" >&2; echo "$profile: the export failed" >&2; exit 1; }
    set -- "$@" "$work/$profile"
  done
fi
git -C "$repo" show "${R16_BRANCH:-HEAD}:crates/paint/src/palette.rs" | sed -n 's/^ *tube("\([^"]*\)".*/\1/p' >"$work/catalog"
[ "$(wc -l <"$work/catalog")" -ge 14 ] || { echo "box_notes: can't read the tube catalog from palette.rs" >&2; exit 1; }

head='| tube | pigment | hiding | stiffness | tinting strength | drying |'
table_of() { awk -v head="$head" '$0 == head { on = 1 } on && !/^\|/ { exit } on { print }' "$1"; }
failed=0
for dest; do
  bad=0
  box=$(cat "$dest/bin/box")
  note=$dest/notes/research/${box//-/_}_materials.md
  [ -s "$note" ] || { echo "$box: no materials note at $note" >&2; failed=1; continue; }
  (cd / && env -u EASEL_BOX "$dest/bin/easel" tubes) >"$work/$box.tubes"
  # (a) and (b)
  perl -CSD -Mutf8 -e '
    my ($box, $note, $tubes, $catalog) = @ARGV;
    sub slurp { open my $f, "<:encoding(UTF-8)", $_[0] or die "$_[0]: $!"; local $/; <$f> }
    sub lines { grep { length } split /\n/, slurp($_[0]) }
    sub section { my ($text, $n) = @_; $text =~ /^## \Q$n\E\.[^\n]*\n(.*?)(?=^## |\z)/ms or die "$box: no section $n in the note\n"; $1 }
    my $text = slurp($note);
    my $table = join "\n", grep { /^\|/ } split /\n/, section($text, 4);
    (my $s9 = section($text, 9)) =~ s/\s+/ /g;
    my @box = lines($tubes);
    my %in = map { lc($_) => 1 } @box;
    my $named = sub { my ($name, $hay) = @_; $hay =~ /(?<![\w-])\Q$name\E(?![\w-])/i };
    my @bad;
    # (a) each tube in the box is named in the note (§4 table or §9)
    for my $t (@box) {
      push @bad, "the box holds \"$t\", which the note names neither in its pigment table nor in §9" unless $named->($t, "$table\n$s9");
    }
    # (b) the tubes §9 maps to are in the box ("a → x, y and z (remark);")
    while ($s9 =~ /→\s*([^;.]+)/g) {
      (my $to = $1) =~ s/\([^)]*\)//g;
      for my $t (split /\s*,\s*|\s+and\s+/, $to) {
        $t =~ s/^\s+|\s+$//g;
        push @bad, "§9 maps to \"$t\", which is not in the box" unless $in{lc $t};
      }
    }
    # ... and so is any catalog tube §9 names (other than inside a longer name in the box)
    for my $t (lines($catalog)) {
      next if $in{lc $t};
      my $rest = $s9;
      $rest =~ s/(?<![\w-])\Q$_\E(?![\w-])/ /gi for grep { length($_) > length($t) && /\Q$t\E/i } @box;
      push @bad, "§9 names \"$t\", which is not in the box" if $named->($t, $rest);
    }
    print STDERR "$box: $_\n" for @bad;
    exit(@bad ? 1 : 0);
  ' "$box" "$note" "$work/$box.tubes" "$work/catalog" || bad=1
  # (c)
  shown=$(table_of "$dest/notes/easel_guide.md")
  printed=$(cd / && env -u EASEL_BOX "$dest/bin/easel" tubes --markdown)
  [ "$shown" = "$printed" ] || { echo "$box: the guide's tube table isn't \`easel tubes --markdown\`" >&2; bad=1; }
  [ $bad = 0 ] || { failed=1; continue; }
  echo "$box: ok ($(wc -l <"$work/$box.tubes" | tr -d ' ') tubes, each named in the note; §9 names only the box's)"
done
[ $failed = 0 ] || { echo "box_notes: a note and its box disagree" >&2; exit 1; }
echo "box_notes: every note agrees with its box"
