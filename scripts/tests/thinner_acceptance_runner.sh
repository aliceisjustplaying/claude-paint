#!/usr/bin/env bash
# Self-test of scripts/test_thinner_acceptance: a fake `cargo` and a stub
# check 2 in a scratch copy, so nothing is built or painted (a second or
# so). Each case sets what the fake prints and how it exits, and checks the
# runner's exit code and summary:
#
#   every test ok, check 13 (b) failing as expected   -> 3, NOT ALL GREEN (--quick, --card, --all)
#   ok lines but cargo exits nonzero                  -> 1
#   a name missing / ignored / failed                 -> 1
#   no output at all                                  -> 1
#   check 13 (b) passing, or not running              -> 1
#   check 13 (b) failing somewhere else (another panic) -> 1
#   check 2 failing                                   -> 1
#   the rag study's sheet not written (--all)         -> 1
#
#   scripts/tests/thinner_acceptance_runner.sh
set -euo pipefail
src=$(cd "$(dirname "$0")/../.." && pwd)
work=$(mktemp -d "${TMPDIR:-/tmp}/thinner-runner-test.XXXXXX")
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/repo/scripts" "$work/repo/notes/thinner" "$work/bin" "$work/tmp"
cp "$src/scripts/test_thinner_acceptance" "$work/repo/scripts/"
cat >"$work/repo/scripts/thinner_check2" <<'EOF'
#!/usr/bin/env bash
if [ -n "${FAKE_CHECK2_FAIL:-}" ]; then echo "check 2: FAIL"; exit 1; fi
echo "check 2: PASS"
EOF
cat >"$work/bin/cargo" <<'EOF'
#!/usr/bin/env bash
# fake cargo: `build` does nothing; `test` prints a libtest line for each
# name after `--` as the FAKE_* variables say
[ "$1" = build ] && exit 0
names=(); seen=0
for a in "$@"; do
  if [ "$a" = "--" ]; then seen=1; continue; fi
  [ $seen = 1 ] || continue
  case "$a" in --*) continue ;; esac
  names+=("$a")
done
has() { [[ " $1 " == *" $2 "* ]]; }
c13b=c13_burnt_sienna_shows_the_card_at_least_as_well_as_raw_sienna
failed=0; code=0
[ -n "${FAKE_EMPTY:-}" ] && exit 101
for n in "${names[@]}"; do
  if [ "$n" = $c13b ] && [ -z "${FAKE_13B_PASS:-}" ]; then
    [ -n "${FAKE_13B_DROP:-}" ] && continue
    echo "test $n ... FAILED"; failed=1
    echo
    echo "failures:"
    echo
    echo "---- $n stdout ----"
    if [ -n "${FAKE_13B_OTHER:-}" ]; then
      echo "thread '$n' panicked at crates/paint/tests/thinner_pigments.rs:116:5:"
      echo "the strokes laid paint"
    else
      echo "thread '$n' panicked at crates/paint/tests/thinner_pigments.rs:119:5:"
      echo "burnt sienna shows 9.85% of the card, raw sienna 17.91%: Field/Salter (§155) has burnt the more transparent"
    fi
    continue
  fi
  if has "${FAKE_FAIL:-}" "$n"; then echo "test $n ... FAILED"; failed=1
  elif has "${FAKE_SKIP:-}" "$n"; then echo "test $n ... ignored, slow"
  elif has "${FAKE_DROP:-}" "$n"; then :
  else echo "test $n ... ok"; fi
  has "${FAKE_EXIT_ON:-}" "$n" && code=101
  if [ "$n" = thinner_tests::rag_study ] && [ -n "${THINNER_RAG_STUDY:-}" ] && [ -z "${FAKE_NO_SHEET:-}" ]; then echo png >"$THINNER_RAG_STUDY"; fi
done
[ $failed = 1 ] && exit 101
exit $code
EOF
chmod +x "$work/repo/scripts/"* "$work/bin/cargo"

fails=0
# expect <exit> <mode> <summary text or -> [VAR=value...]
expect() {
  local want=$1 mode=$2 text=$3; shift 3
  local out got=0
  out=$(env "$@" TMPDIR="$work/tmp" PATH="$work/bin:$PATH" "$work/repo/scripts/test_thinner_acceptance" "$mode" 2>&1) || got=$?
  if [ "$got" != "$want" ] || { [ "$text" != - ] && ! grep -qF -- "$text" <<<"$out"; }; then
    echo "FAIL: $mode $* -> exit $got (want $want${text:+, text \"$text\"})"
    tail -8 <<<"$out" | sed 's/^/    /'
    fails=$((fails + 1))
  else
    echo "ok:   $mode ${*:-(all pass)} -> $got"
  fi
}

for m in --quick --card --all; do
  expect 3 $m "NOT ALL GREEN" FAKE_=
done
expect 3 --all "EXPECTED FAIL (pre-existing; user decision, ACCEPTANCE.md check 13)" FAKE_=
expect 1 --quick "cargo exited 101" FAKE_EXIT_ON=c05_an_emptying_brush_lays_less_and_a_fuller_load_lasts_farther
expect 1 --card "printed ok, but cargo exited 101" FAKE_EXIT_ON=thinner_tests::c01_the_card_raw_sienna_thinned_half_keeps_half_the_contrast_at_2400px
expect 1 --all "(did not run" FAKE_DROP=c06_more_pressure_lays_more_paint_up_to_the_stroke_limit
expect 1 --all "(skipped)" FAKE_SKIP=thinner_tests::c01_the_card_raw_sienna_thinned_half_keeps_half_the_contrast_at_2400px
expect 1 --all "(failed)" FAKE_FAIL=c09_the_solvent_evaporates_and_the_film_loses_its_volume
expect 1 --all "FAILED" FAKE_EMPTY=1
expect 1 --quick "passed unexpectedly" FAKE_13B_PASS=1
expect 1 --card "the expected failure must run and fail" FAKE_13B_DROP=1
expect 1 --quick "not at its ordering assertion" FAKE_13B_OTHER=1
expect 1 --quick "check 2 (scripts/thinner_check2" FAKE_CHECK2_FAIL=1
expect 1 --all "rag study's sheet was not written" FAKE_NO_SHEET=1
expect 2 --bogus - FAKE_=

if [ $fails = 0 ]; then echo "thinner_acceptance_runner: all cases pass"; else echo "thinner_acceptance_runner: $fails cases FAILED"; exit 1; fi
