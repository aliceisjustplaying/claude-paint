#!/usr/bin/env bash
# scripts/test itself (the real runner), with tiny dummy steps (sleep, echo, bash
# subshells): every process a step starts is stopped before the run ends and the
# lock is released, whatever ends the run, and the runner refuses lists that
# can't test anything.
#
#   scripts/tests/test_runner.sh [case number...]     (default: all)
#
# Cases: a step's leftovers (a TERM-resistant child, an orphaned grandchild) on
# normal completion; a step past its limit whose leader exits on SIGTERM before
# its TERM-resistant child; the outer timeout; lockrun cancelled; SIGTERM to the
# coordinator; the coordinator killed (SIGKILL); a stale LOCKRUN_TOKEN; nesting
# inside a real job; build-only and zero-minimum lists; Cargo's target directory
# (two distinguishable dummy easels); a group running at the same time. Each case
# uses its own lock directory (LOCKRUN_DIR), never the machine's lock, and
# records the pid of every process it starts so it can check they are gone.
# About 40 s.
set -euo pipefail
repo=$(cd "$(dirname "$0")/../.." && pwd)
R=$repo/scripts/test
unset LOCKRUN_TOKEN LOCKRUN_DIR SCRIPTS_TEST_OUTER_LIMIT
W=$(mktemp -d /tmp/cptr.XXXXXX)
cleanup() {
  for f in "$W"/*/pid.*; do [ -f "$f" ] && kill -9 "$(cat "$f")" 2>/dev/null || true; done
  rm -rf "$W"
}
trap cleanup EXIT
T=$'\t'
passed=0
want() { [ $# -gt 0 ] && { [ -z "$ONLY" ] || [[ " $ONLY " == *" $1 "* ]]; }; }
ONLY="$*"
fail() { echo "test_runner FAILED: $*" >&2; exit 1; }
ok() { passed=$((passed + 1)); echo "ok $passed - $*"; }
alive() { kill -0 "$1" 2>/dev/null && [ "$(ps -o stat= -p "$1" | cut -c1)" != Z ]; }
all_dead() { local f; for f in "$1"/pid.*; do [ -f "$f" ] || continue; alive "$(cat "$f")" && { echo "alive: $f $(cat "$f")" >&2; return 1; }; done; return 0; }
wait_until() { local end=$((SECONDS + $1)); shift; until "$@"; do [ $SECONDS -lt $end ] || return 1; sleep 0.1; done; }
lock_free() { LOCKRUN_DIR=$1/lk "$repo/scripts/lockrun" --timeout 5 --lock-timeout 0.5 --quiet -- true 2>/dev/null; }
case_dir() { local d=$W/$1; mkdir -p "$d"; echo "$d"; }
# step <name> <limit> <least> <regex> <command> [group]
step() { printf 'script%s%s%s%s%s%s%s%s%s%s%s%s\n' "$T" "$1" "$T" "$2" "$T" "$3" "$T" "$4" "$T" "$5" "${6:+$T}" "${6:-}"; }
summary() { python3 -c 'import json,sys; s=json.load(open(sys.argv[1])); print(eval(sys.argv[2]))' "$1" "$2"; }
# a TERM-resistant process that records its pid in <file>
stubborn() { echo "( trap '' TERM; echo \$BASHPID > $1; exec sleep 60 )"; }

# 1. Leftovers on normal completion: a TERM-resistant child and an orphaned
if want 1; then
#    grandchild (its parent has exited) are stopped; the run passes.
D=$(case_dir leftovers)
{ step left 20 1 '^done' "$(stubborn "$D/pid.child") & ( $(stubborn "$D/pid.orphan") & ) ; wait_for() { while [ ! -s \$1 ]; do sleep 0.05; done; }; wait_for $D/pid.child; wait_for $D/pid.orphan; echo done"; } >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1 || fail "leftovers: the run failed: $(cat "$D/out")"
all_dead "$D" || fail "leftovers: a process the step left is alive"
grep -Eq 'stopped [0-9]+ leftover processes' "$D/out" || fail "leftovers: not reported: $(cat "$D/out")"
lock_free "$D" || fail "leftovers: the lock is held"
ok "normal completion: a TERM-resistant child and an orphaned grandchild are stopped"

fi

# 2. A step past its limit whose leader exits on SIGTERM before its TERM-resistant child.
if want 2; then
D=$(case_dir step_timeout)
# (the next step checks the child is gone when it starts: stopped by the step's timeout,
# not only by lockrun when the whole run ends)
{ step slow 1 1 '^never' "$(stubborn "$D/pid.child") & echo \$\$ > $D/pid.leader; sleep 60"; step after 10 1 '^after' "kill -0 \$(cat $D/pid.child) 2>/dev/null && echo still-alive || echo after"; } >"$D/l.tsv"
set +e; LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1; code=$?; set -e
[ $code = 1 ] || fail "step timeout: exit $code: $(cat "$D/out")"
all_dead "$D" || fail "step timeout: a process of the step is alive"
[ "$(summary "$D/s.json" 's["verdict"], s["steps"][0]["timed_out"], s["steps"][1]["ok"]')" = "('unfinished', True, True)" ] || fail "step timeout: summary $(cat "$D/s.json")"
lock_free "$D" || fail "step timeout: the lock is held"
ok "a step past its limit: its leader and its TERM-resistant child are stopped before the next step; the run is unfinished"

fi

# 3. The outer timeout (lowered for the test) stops the run and every step process.
if want 3; then
D=$(case_dir outer_timeout)
step slow 60 1 '^never' "$(stubborn "$D/pid.child") & echo \$\$ > $D/pid.leader; sleep 60" >"$D/l.tsv"
set +e; SCRIPTS_TEST_OUTER_LIMIT=2 LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --logs "$D/logs" >"$D/out" 2>&1; code=$?; set -e
[ $code = 124 ] || fail "outer timeout: exit $code: $(cat "$D/out")"
all_dead "$D" || fail "outer timeout: a step process outlived the run"
lock_free "$D" || fail "outer timeout: the lock is held"
ok "the outer timeout (exit 124) stops every step process; the lock is free"

fi

# 4. Cancelling lockrun (SIGTERM to the helper) stops every step process.
if want 4; then
D=$(case_dir cancel)
step slow 60 1 '^never' "$(stubborn "$D/pid.child") & echo \$\$ > $D/pid.leader; sleep 60" >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --logs "$D/logs" >"$D/out" 2>&1 &
helper=$!
wait_until 15 test -s "$D/pid.child" || fail "cancel: the step did not start"
kill -TERM "$helper"
set +e; wait "$helper"; code=$?; set -e
[ $code = 130 ] || fail "cancel: exit $code"
all_dead "$D" || fail "cancel: a step process outlived the run"
lock_free "$D" || fail "cancel: the lock is held"
ok "cancelling lockrun (exit 130) stops every step process; the lock is free"

fi

# 5. SIGTERM to the coordinator itself: it stops the steps and ends unfinished.
if want 5; then
D=$(case_dir term_coordinator)
step slow 60 1 '^never' "$(stubborn "$D/pid.child") & echo \$PPID > $D/coordinator; sleep 60 & echo \$! > $D/pid.sleep; wait" >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1 &
helper=$!
wait_until 15 test -s "$D/pid.child" -a -s "$D/coordinator" || fail "term: the step did not start"
kill -TERM "$(cat "$D/coordinator")"
set +e; wait "$helper"; code=$?; set -e
all_dead "$D" || fail "term: a step process outlived the coordinator"
[ "$(summary "$D/s.json" 's["verdict"]')" = unfinished ] || fail "term: verdict $(cat "$D/s.json")"
[ $code = 130 ] || fail "term: exit $code"
lock_free "$D" || fail "term: the lock is held"
ok "SIGTERM to the coordinator stops the steps; summary unfinished, exit 130; the lock is free"

fi

# 6. The coordinator killed (SIGKILL): lockrun stops the steps it left before releasing the lock.
if want 6; then
D=$(case_dir crash)
step slow 60 1 '^never' "$(stubborn "$D/pid.child") & echo \$PPID > $D/coordinator; sleep 60 & echo \$! > $D/pid.sleep; wait" >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --logs "$D/logs" >"$D/out" 2>&1 &
helper=$!
wait_until 15 test -s "$D/pid.child" -a -s "$D/coordinator" || fail "crash: the step did not start"
kill -9 "$(cat "$D/coordinator")"
set +e; wait "$helper"; set -e
all_dead "$D" || fail "crash: a step process outlived the killed coordinator"
lock_free "$D" || fail "crash: the lock is held"
ok "the coordinator killed: lockrun stops its steps before releasing the lock"

fi

# 7. A stale LOCKRUN_TOKEN doesn't bypass the lock: the run waits for the holder.
if want 7; then
D=$(case_dir stale_token)
step when 20 1 '^started' "python3 -c 'import time; print(\"started\", time.time())'" >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$repo/scripts/lockrun" --timeout 20 --quiet --owner holder -- sleep 3 2>/dev/null &
holder=$!
wait_until 10 test -f "$D/lk/record.json" || fail "stale token: the holder didn't start"
t0=$(python3 -c 'import time; print(time.time())')
LOCKRUN_DIR=$D/lk LOCKRUN_TOKEN=0123456789abcdef0123456789abcdef "$R" --list "$D/l.tsv" --logs "$D/logs" >"$D/out" 2>&1 || fail "stale token: the run failed: $(cat "$D/out")"
wait "$holder"
start=$(sed -n 's/^started //p' "$D/logs/when.log")
python3 -c "import sys; sys.exit(0 if float('$start') - float('$t0') > 2.0 else 1)" || fail "stale token: the step ran while the lock was held"
grep -q 'stale LOCKRUN_TOKEN' "$D/out" || fail "stale token: lockrun didn't say so"
ok "a stale LOCKRUN_TOKEN waits for the lock"

fi

# 8. Nested in a real job (scripts/test --candidate runs so): no deadlock, one lock job.
if want 8; then
D=$(case_dir nested)
step one 10 1 '^one' 'echo one' >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$repo/scripts/lockrun" --timeout 30 --quiet -- "$R" --list "$D/l.tsv" --logs "$D/logs" >"$D/out" 2>&1 || fail "nested: $(cat "$D/out")"
[ "$(grep -c '"event": "start"' "$D/lk/history.jsonl")" = 1 ] || fail "nested: the inner run took the lock again"
ok "inside a real lockrun job: runs at once, without taking the lock again"

fi

# 9. Lists that can't test anything are refused before anything runs.
if want 9; then
D=$(case_dir lists)
printf 'build%sb%s10%s0%s-%strue\n' "$T" "$T" "$T" "$T" "$T" >"$D/build.tsv"
step zero 10 0 '^x' 'echo x' >"$D/zero.tsv"
printf 'script%sx%s10%s1%s-%secho x\n' "$T" "$T" "$T" "$T" "$T" >"$D/noregex.tsv"
step long 601 1 '^x' 'echo x' >"$D/long.tsv"
for l in build zero noregex long; do
  set +e; LOCKRUN_DIR=$D/lk "$R" --list "$D/$l.tsv" >"$D/$l.out" 2>&1; code=$?; set -e
  [ $code != 0 ] || fail "lists: $l was accepted"
  [ ! -f "$D/lk/record.json" ] || fail "lists: $l took the lock"
done
grep -q 'lists no test step' "$D/build.out" && grep -q 'must run at least one test' "$D/zero.out" && grep -q 'needs a regex' "$D/noregex.out" && grep -q 'at most 600 s' "$D/long.out" || fail "lists: messages"
ok "build-only, zero-minimum, uncountable and over-600-s lists are refused before taking the lock"

fi

# 10. Cargo's target directory: the steps get the one cargo resolves, so a stale
if want 10; then
#     executable elsewhere is never the one run.
D=$(case_dir target)
for t in stale fresh; do mkdir -p "$D/$t/release"; printf '#!/bin/sh\necho %s easel\n' "$t" >"$D/$t/release/easel"; chmod +x "$D/$t/release/easel"; done
step which 10 1 '^fresh easel$' '"$CARGO_TARGET_DIR/release/easel"' >"$D/l.tsv"
CARGO_TARGET_DIR=$D/fresh LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1 || fail "target: $(cat "$D/out")"
[ "$(summary "$D/s.json" 's["target_dir"]')" = "$D/fresh" ] || fail "target: summary $(summary "$D/s.json" 's["target_dir"]')"
want=$(cd "$repo" && env -u CARGO_TARGET_DIR cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
env -u CARGO_TARGET_DIR LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s2.json" --logs "$D/logs2" >/dev/null 2>&1 || true
[ "$(summary "$D/s2.json" 's["target_dir"]')" = "$want" ] || fail "target: unset CARGO_TARGET_DIR resolved to $(summary "$D/s2.json" 's["target_dir"]'), not $want"
ok "steps run \$CARGO_TARGET_DIR/release/easel from the directory cargo resolves (fresh, not stale)"

fi

# 11. A group runs at the same time; the summary carries each step's list fields.
if want 11; then
D=$(case_dir group)
{ step a 10 1 '^a' 'sleep 2; echo a' g; step b 10 1 '^b' 'sleep 2; echo b' g; } >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1 || fail "group: $(cat "$D/out")"
[ "$(summary "$D/s.json" 's["wall_seconds"] < 3.5')" = True ] || fail "group: wall $(summary "$D/s.json" 's["wall_seconds"]')"
[ "$(summary "$D/s.json" '[(x["name"], x["limit"], x["least"], x["group"]) for x in s["steps"]]')" = "[('a', 10.0, 1, 'g'), ('b', 10.0, 1, 'g')]" ] || fail "group: step fields"
[ "$(summary "$D/s.json" 'len(s["runner_sha256"])')" = 64 ] || fail "group: runner_sha256"
ok "a group's steps run at the same time; the summary holds limit, least, group and the runner's hash"

fi
echo "test_runner: all $passed checks passed"
