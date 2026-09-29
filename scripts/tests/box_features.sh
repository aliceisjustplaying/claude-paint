#!/usr/bin/env bash
# The box features of paint and the easel, configuration by configuration:
# - no box-* feature: the default tube box only;
# - one box-* feature (a painter's build): that box only, no default box, and
#   the catalog holds that box's tubes and no other;
# - two box-* features without all-boxes: no build (a compile error);
# - the replay build (easel's default, paint's all-boxes): the default box
#   and all five named boxes.
# paint's catalog tests (palette::tests) run in each, and the easel's
# painter test the_build_holds_only_its_box_s_tubes in each painter build.
#
#   scripts/tests/box_features.sh
#
# Builds paint and the easel once per configuration (several minutes); uses
# CARGO_TARGET_DIR if set.
set -euo pipefail
repo=$(cd "$(dirname "$0")/../.." && pwd)
cd "$repo"
unset EASEL_BOX

# passes <n> <what> <cargo test args...>: the tests run and at least n pass
passes() {
  local n=$1 what=$2 out
  shift 2
  out=$(cargo test --release "$@" 2>&1) || { echo "box_features: $what failed:" >&2; echo "$out" >&2; exit 1; }
  local ran
  ran=$(sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p' <<<"$out" | awk '{s += $1} END {print s + 0}')
  [ "$ran" -ge "$n" ] || { echo "box_features: $what ran $ran tests, not $n or more:" >&2; echo "$out" >&2; exit 1; }
  echo "box_features: $what: $ran passed"
}
# refused <what> <cargo check args...>: the build fails with the box error
refused() {
  local what=$1 out
  shift
  if out=$(cargo check "$@" 2>&1); then
    echo "box_features: $what built; it must not" >&2
    exit 1
  fi
  grep -q 'more than one box-\* feature without all-boxes' <<<"$out" || { echo "box_features: $what failed, but not with the box error:" >&2; echo "$out" >&2; exit 1; }
  echo "box_features: $what: refused"
}

painter_test=(--test painter the_build_holds_only_its_box_s_tubes -- --exact)
for f in "" box-sargent box-inness box-alma-tadema box-tonn box-hopper; do
  features=(--no-default-features)
  [ -z "$f" ] || features+=(--features "$f")
  passes 3 "paint ${f:-no box}" -p paint "${features[@]}" --lib -- palette::tests::the_catalog palette::tests::every_box
  passes 1 "easel ${f:-no box} (painter)" -p easel "${features[@]}" "${painter_test[@]}"
done
refused "paint box-sargent,box-inness" -p paint --no-default-features --features box-sargent,box-inness
refused "easel box-inness,box-tonn (painter)" -p easel --no-default-features --features box-inness,box-tonn
passes 3 "paint all-boxes (default)" -p paint --lib -- palette::tests::the_catalog palette::tests::every_box
passes 1 "easel replay (default)" -p easel --test boxes the_replay_build_holds_every_box -- --exact
echo "box_features: no box, each box alone, two boxes and the replay build hold the boxes they should"
