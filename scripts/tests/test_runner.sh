#!/usr/bin/env bash
# scripts/test itself (the real runner), with tiny dummy steps (sleep, echo, bash
# subshells): every process a step starts is stopped before the run ends and the
# lock is released, whatever ends the run, and the runner refuses lists that
# can't test anything.
#
#   scripts/tests/test_runner.sh [case number...]     (default: all)
#
# Cases: a step's leftovers (a TERM-resistant child, an orphaned grandchild) on
# normal completion; a step past its limit inside a group (its TERM-resistant
# child stopped at its own timeout while the other step goes on); the outer
# timeout; lockrun cancelled; SIGTERM to the coordinator; the coordinator killed
# (SIGKILL); a stale LOCKRUN_TOKEN; nesting inside a real job; build-only and
# zero-minimum lists; a target dir from Cargo's configuration (two
# distinguishable dummy easels); a group running at the same time; a batch's
# sibling process left alone; a step child that left the group stopped (sccache
# left alone); --locked validation; the log header not counted; a known
# failure; the build and check phases. Each case
# uses its own lock directory (LOCKRUN_DIR), never the machine's lock, and
# records the pid of every process it starts so it can check they are gone.
# About 70 s.
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

# 2. A step past its limit, inside a group: its leader exits on SIGTERM, its
#    TERM-resistant child must be stopped at the step's own timeout, while the
#    other step of the group goes on (it checks the child is gone at about 8 s:
#    after the 1 s limit and the 5 s grace before SIGKILL, before the group ends
#    and any sweep could run).
if want 2; then
D=$(case_dir step_timeout)
{ step slow 1 1 '^never' "$(stubborn "$D/pid.child") & echo \$\$ > $D/pid.leader; sleep 60" g
  step watch 20 1 '^child-gone$' "while [ ! -s $D/pid.child ]; do sleep 0.05; done; sleep 8; if kill -0 \$(cat $D/pid.child) 2>/dev/null; then echo child-alive; else echo child-gone; fi; echo \$\$ > $D/pid.watch" g
  step after 10 1 '^after' 'echo after'; } >"$D/l.tsv"
set +e; LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1; code=$?; set -e
[ $code = 3 ] || fail "step timeout: exit $code: $(cat "$D/out")"
all_dead "$D" || fail "step timeout: a process of the step is alive"
got=$(summary "$D/s.json" '[(x["name"], x["timed_out"], x["ok"]) for x in s["steps"]] + [s["verdict"]]')
[ "$got" = "[('slow', True, False), ('watch', False, True), ('after', False, True), 'unfinished']" ] || fail "step timeout: $got; watch said $(tail -1 "$D/logs/watch.log")"
lock_free "$D" || fail "step timeout: the lock is held"
ok "a step past its limit inside a group: its TERM-resistant child is stopped at its timeout while the group's other step runs on; the run is unfinished"
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

# 10. Cargo's target directory comes from Cargo's configuration alone (a
#     config.toml in CARGO_HOME, CARGO_TARGET_DIR unset): the steps get it, so the
#     step runs the "fresh" dummy easel there, not the "stale" one in target/.
if want 10; then
D=$(case_dir target)
for t in stale fresh; do mkdir -p "$D/$t/release"; printf '#!/bin/sh\necho %s easel\n' "$t" >"$D/$t/release/easel"; chmod +x "$D/$t/release/easel"; done
mkdir -p "$D/cargohome"; printf '[build]\ntarget-dir = "%s"\n' "$D/fresh" >"$D/cargohome/config.toml"
step which 10 1 '^fresh easel$' '"${CARGO_TARGET_DIR:-'"$D"'/stale}/release/easel"' >"$D/l.tsv"
env -u CARGO_TARGET_DIR CARGO_HOME=$D/cargohome LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1 || fail "target: $(cat "$D/out")"
[ "$(summary "$D/s.json" 's["target_dir"]')" = "$D/fresh" ] || fail "target: summary $(summary "$D/s.json" 's["target_dir"]')"
ok "a target dir set only in Cargo's configuration reaches the steps (fresh easel, not stale)"
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
# 12. Nested in a batch (an outer lockrun job): the run stops its own steps'
#     leftovers but not the batch's other processes (review S1).
if want 12; then
D=$(case_dir nested_sibling)
step one 10 1 '^one' "( $(stubborn "$D/pid.left") & ); while [ ! -s $D/pid.left ]; do sleep 0.05; done; echo one" >"$D/l.tsv"
LOCKRUN_DIR=$D/lk "$repo/scripts/lockrun" --timeout 60 --quiet -- bash -c "sleep 30 & echo \$! > $D/sibling; '$R' --list '$D/l.tsv' --logs '$D/logs' >'$D/out' 2>&1; echo runner=\$? > $D/result; if kill -0 \$(cat $D/sibling) 2>/dev/null; then echo sibling=alive >> $D/result; else echo sibling=killed >> $D/result; fi; kill \$(cat $D/sibling)" 2>/dev/null || true
[ "$(cat "$D/result" | tr '\n' ' ')" = "runner=0 sibling=alive " ] || fail "nested sibling: $(cat "$D/result") $(cat "$D/out")"
alive "$(cat "$D/pid.left")" && fail "nested sibling: the step's own leftover survived"
ok "nested in a batch: the step's leftover is stopped, the batch's sibling process is not"
fi

# 13. A step's child that leaves the process group (setpgid, as the easel's
#     session server does), TERM-resistant: stopped when the step ends normally,
#     and when a step times out; a shared daemon (sccache: it detaches as the real
#     server does) is left alone (review S3).
if want 13; then
D=$(case_dir detached)
detach() { echo "python3 -c 'import os, signal, sys, time; os.setpgid(0, 0); signal.signal(signal.SIGTERM, signal.SIG_IGN); open(sys.argv[1], \"w\").write(str(os.getpid())); time.sleep(60)' $1 & while [ ! -s $1 ]; do sleep 0.05; done"; }
ln -s /bin/sleep "$D/sccache"  # (named sccache; a copy of a system binary would not run)
{ step ends 20 1 '^ends' "$(detach "$D/pid.ends"); python3 -c 'import os, sys; os.setsid(); open(sys.argv[1], \"w\").write(str(os.getpid())); os.execv(sys.argv[2], sys.argv[2:])' $D/daemon '$D/sccache' 60 & while [ ! -s $D/daemon ]; do sleep 0.05; done; echo ends"
  step times 1 1 '^never' "$(detach "$D/pid.times"); sleep 60"; } >"$D/l.tsv"
set +e; LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1; code=$?; set -e
for f in ends times; do alive "$(cat "$D/pid.$f")" && fail "detached: the $f step's detached child survived"; done
alive "$(cat "$D/daemon")" || fail "detached: the shared daemon (sccache) was stopped"
kill "$(cat "$D/daemon")"
[ "$(summary "$D/s.json" '[(x["name"], x["ok"], x["timed_out"]) for x in s["steps"]]')" = "[('ends', True, False), ('times', False, True)]" ] || fail "detached: summary"
ok "a child that left the group is stopped (normal end and timeout); sccache is left alone"
fi

# 14. --locked is accepted only inside the lockrun job whose record names this
#     group and token (review R3): no token, a stale token, a record of another
#     group are all refused before anything runs.
if want 14; then
D=$(case_dir locked)
step x 10 1 '^ran' "echo ran > $D/ran" >"$D/l.tsv"
mkdir -p "$D/lk"
pg=$(ps -o pgid= -p $$ | tr -d ' ')
rec() { printf '{"token": "%s", "state": "running", "pgid": %s, "pid": 1, "command": []}\n' "$1" "$2" >"$D/lk/record.json"; }
rec aaaa "$pg"
for env in "LOCKRUN_TOKEN=" "LOCKRUN_TOKEN=bbbb" "LOCKRUN_TOKEN=aaaa PGID_OTHER=1"; do
  [ "$env" = "LOCKRUN_TOKEN=aaaa PGID_OTHER=1" ] && rec aaaa 1
  set +e; env $env LOCKRUN_DIR=$D/lk python3 "$R" --locked --list "$D/l.tsv" >"$D/out" 2>&1; code=$?; set -e
  [ $code != 0 ] && grep -q -- '--locked is for the lockrun job' "$D/out" || fail "locked: $env was accepted: $(cat "$D/out")"
  [ ! -f "$D/ran" ] || fail "locked: a step ran ($env)"
done
ok "--locked without the job's token, with another token or with another group's record is refused"
fi

# 15. A script step's count never includes the runner's "$ <command>" header line (review R5).
if want 15; then
D=$(case_dir header)
step quiet 10 1 'marker' ': marker' >"$D/l.tsv"
set +e; LOCKRUN_DIR=$D/lk "$R" --list "$D/l.tsv" --summary "$D/s.json" --logs "$D/logs" >"$D/out" 2>&1; code=$?; set -e
[ $code = 1 ] && [ "$(summary "$D/s.json" 's["steps"][0]["tests_run"]')" = 0 ] || fail "header: the command line was counted: $(cat "$D/out")"
ok "the header line naming the command doesn't count as a test"
fi

# 16. A known failure (columns 8 to 10): exactly the listed exit and line, with
#     nothing else wrong, makes the verdict known_failure, exit 4, never pass;
#     another exit, the exit without the line, or another failing step: fail.
if want 16; then
D=$(case_dir known)
kstep() { printf 'script%s%s%s10%s1%s^ran%s%s%s-%s3%s^ONLY KNOWN LEFT$%sthe known one\n' "$T" "$1" "$T" "$T" "$T" "$T" "$2" "$T" "$T" "$T" "$T"; }
kstep k 'echo ran; echo ONLY KNOWN LEFT; exit 3' >"$D/known.tsv"
kstep k 'echo ran; exit 3' >"$D/noline.tsv"
kstep k 'echo ran; echo ONLY KNOWN LEFT; exit 5' >"$D/otherexit.tsv"
{ kstep k 'echo ran; echo ONLY KNOWN LEFT; exit 3'; step bad 10 1 '^never' 'exit 1'; } >"$D/plusfail.tsv"
for l in known noline otherexit plusfail; do
  set +e; LOCKRUN_DIR=$D/lk "$R" --list "$D/$l.tsv" --summary "$D/$l.json" --logs "$D/logs-$l" >"$D/$l.out" 2>&1; eval "code_$l=\$?"; set -e
done
[ "$code_known" = 4 ] && [ "$(summary "$D/known.json" '(s["verdict"], s["verdict_text"], s["steps"][0]["known_failure"], s["steps"][0]["ok"])')" = "('known_failure', 'NOT ALL GREEN (known pre-existing failure: the known one)', True, False)" ] || fail "known: $code_known $(cat "$D/known.out")"
grep -q 'NOT ALL GREEN (known pre-existing failure: the known one)' "$D/known.out" || fail "known: not printed"
for l in noline otherexit plusfail; do
  eval "c=\$code_$l"
  [ "$c" = 1 ] && [ "$(summary "$D/$l.json" 's["verdict"]')" = fail ] || fail "known: $l gave exit $c, $(summary "$D/$l.json" 's["verdict"]')"
done
ok "a known failure: verdict known_failure (NOT ALL GREEN), exit 4; without its line, with another exit or beside a failure: fail"
fi

# 17. Two phases: the build steps as one lockrun job, the others as a second; the
#     check phase doesn't run after a failed build.
if want 17; then
D=$(case_dir phases)
{ printf 'build%sb%s10%s0%s-%secho built\n' "$T" "$T" "$T" "$T" "$T"; step t 10 1 '^tested' 'echo tested'; } >"$D/ok.tsv"
{ printf 'build%sb%s10%s0%s-%sexit 1\n' "$T" "$T" "$T" "$T" "$T"; step t 10 1 '^tested' "echo tested > $D/tested; echo tested"; } >"$D/badbuild.tsv"
LOCKRUN_DIR=$D/lk "$R" --list "$D/ok.tsv" --summary "$D/ok.json" --logs "$D/logs" >"$D/ok.out" 2>&1 || fail "phases: $(cat "$D/ok.out")"
[ "$(summary "$D/ok.json" '(s["verdict"], sorted(s["phases"]), s["phases"]["build"]["limit"], s["phases"]["check"]["verdict"], [x["name"] for x in s["steps"]])')" = "('pass', ['build', 'check'], 600.0, 'pass', ['b', 't'])" ] || fail "phases: summary"
[ "$(grep -c '"event": "start"' "$D/lk/history.jsonl")" = 2 ] || fail "phases: not two lockrun jobs"
set +e; LOCKRUN_DIR=$D/lk2 "$R" --list "$D/badbuild.tsv" --summary "$D/bad.json" --logs "$D/logs2" >"$D/bad.out" 2>&1; code=$?; set -e
[ $code = 1 ] && [ ! -f "$D/tested" ] && [ "$(summary "$D/bad.json" '(s["verdict"], sorted(s["phases"]))')" = "('fail', ['build'])" ] || fail "phases: a failed build: exit $code $(cat "$D/bad.out")"
ok "two phases, two lockrun jobs; after a failed build the check phase doesn't run"
fi

echo "test_runner: all $passed checks passed"
