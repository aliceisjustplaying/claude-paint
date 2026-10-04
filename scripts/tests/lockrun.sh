#!/usr/bin/env bash
# scripts/lockrun with tiny dummy jobs (sleep, echo, python one-liners):
# normal completion, the record written before work begins, timeout,
# cancellation (SIGINT, SIGTERM), leftovers, waiting without erasing the
# previous record, --lock-timeout, a helper killed while its job lives (and
# with it the trampoline that holds the lock), a daemon leaving the job, a
# helper killed before it gives the job permission, nesting, a stale token, and an unrelated process
# that must survive all of it.
#
#   scripts/tests/lockrun.sh
#
# Each case uses its own lock directory (LOCKRUN_DIR) under a temporary
# directory, never the machine's real lock. Takes about 20 s.
set -euo pipefail
repo=$(cd "$(dirname "$0")/../.." && pwd)
L=$repo/scripts/lockrun
unset LOCKRUN_TOKEN LOCKRUN_DIR LOCKRUN_TEST_HOLD_BEFORE_GO LOCKRUN_OWNER
work=$(mktemp -d "${TMPDIR:-/tmp}/lockrun-test.XXXXXX")
bystander=
cleanup() {
  [ -n "$bystander" ] && kill "$bystander" 2>/dev/null || true
  # stop anything a failed case left behind (only pids this script recorded)
  for f in "$work"/*/pid.*; do [ -f "$f" ] && kill -9 "$(cat "$f")" 2>/dev/null || true; done
  rm -rf "$work"
}
trap cleanup EXIT
passed=0
fail() { echo "lockrun test FAILED: $*" >&2; exit 1; }
ok() { passed=$((passed + 1)); echo "ok $passed - $*"; }
alive() { kill -0 "$1" 2>/dev/null && [ "$(ps -o stat= -p "$1" | cut -c1)" != Z ]; }
# wait_until SECONDS COMMAND...: bounded wait for a condition (test helper only)
wait_until() {
  local end=$((SECONDS + $1)); shift
  until "$@"; do [ $SECONDS -lt $end ] || return 1; sleep 0.1; done
}
rec() { python3 -c 'import json,sys; r=json.load(open(sys.argv[1]+"/record.json")); print(r.get(sys.argv[2], ""))' "$1" "$2"; }
running() { [ -f "$1/record.json" ] && [ "$(rec "$1" state)" = running ]; }
case_dir() { local d=$work/$1; mkdir -p "$d"; echo "$d"; }

# A process outside lockrun that must survive every case.
sleep 300 &
bystander=$!

# 1. Normal completion: the exit code passes through, output reaches stdout and the log.
D=$(case_dir normal)
set +e
out=$(LOCKRUN_DIR=$D "$L" --timeout 10 --owner t1 -- sh -c 'echo hello; echo oops >&2; exit 3' 2>"$D/err")
code=$?
set -e
[ $code = 3 ] || fail "normal: exit $code, wanted 3"
printf '%s\n' "$out" | grep -qx hello || fail "normal: stdout lacks the job's output"
log=$(rec "$D" log)
grep -qx hello "$log" && grep -qx oops "$log" || fail "normal: the log lacks stdout or stderr"
[ "$(rec "$D" state)" = finished ] && [ "$(rec "$D" exit)" = 3 ] && [ "$(rec "$D" owner)" = t1 ] || fail "normal: record $(cat "$D/record.json")"
[ "$(grep -c '"event": "start"' "$D/history.jsonl")" = 1 ] && [ "$(grep -c '"event": "finish"' "$D/history.jsonl")" = 1 ] || fail "normal: history"
ok "normal completion: exit code, stdout, log and record"

# 2. The record (owner, command, group, token, start) is on disk before the job's work begins.
D=$(case_dir record_first)
LOCKRUN_DIR=$D "$L" --timeout 10 --owner t2 --quiet -- python3 -c '
import json, os
r = json.load(open(os.environ["LOCKRUN_DIR"] + "/record.json"))
assert r["state"] == "running", r
assert r["token"] == os.environ["LOCKRUN_TOKEN"], r
assert r["pgid"] == os.getpgid(0), (r, os.getpgid(0))
assert r["owner"] == "t2" and r["command"][0] == "python3" and r["start"], r
open(os.environ["LOCKRUN_DIR"] + "/checked", "w").write("yes")
' 2>/dev/null || fail "record_first: the job did not see its own record"
[ -f "$D/checked" ] || fail "record_first: the job did not run"
ok "record written before work: the job reads its own token, group and owner"

# 3. Timeout: the whole group (leader and a background child) stops; exit 124; log kept.
D=$(case_dir timeout)
set +e
LOCKRUN_DIR=$D "$L" --timeout 1 --grace 1 --quiet -- sh -c "echo before; sleep 30 & echo \$! > '$D/pid.bg'; echo \$\$ > '$D/pid.leader'; sleep 30; echo after" 2>"$D/err"
code=$?
set -e
[ $code = 124 ] || fail "timeout: exit $code, wanted 124"
! alive "$(cat "$D/pid.bg")" && ! alive "$(cat "$D/pid.leader")" || fail "timeout: a process of the job survived"
[ "$(rec "$D" state)" = timeout ] || fail "timeout: record state $(rec "$D" state)"
grep -qx before "$(rec "$D" log)" && ! grep -qx after "$(rec "$D" log)" || fail "timeout: log"
grep -q 'UNFINISHED (timeout)' "$D/err" || fail "timeout: no UNFINISHED message"
ok "timeout: exit 124, group stopped, record 'timeout', log kept"

# 4. A job that ignores SIGTERM gets SIGKILL after the grace period.
D=$(case_dir stubborn)
set +e
LOCKRUN_DIR=$D "$L" --timeout 1 --grace 1 --quiet -- python3 -c "
import os, signal, time
signal.signal(signal.SIGTERM, signal.SIG_IGN)
open('$D/pid.job', 'w').write(str(os.getpid()))
time.sleep(30)" 2>"$D/err"
code=$?
set -e
[ $code = 124 ] || fail "stubborn: exit $code"
! alive "$(cat "$D/pid.job")" || fail "stubborn: the job survived SIGKILL"
grep -q SIGKILL "$D/err" || fail "stubborn: no SIGKILL message"
ok "timeout with a job ignoring SIGTERM: SIGKILL after the grace period"

# 5. Cancellation: SIGTERM and SIGINT to the helper stop the group; exit 130.
for sig in TERM INT; do
  D=$(case_dir "cancel_$sig")
  LOCKRUN_DIR=$D "$L" --timeout 60 --grace 1 --quiet -- sh -c "sleep 30 & echo \$! > '$D/pid.bg'; sleep 30" 2>"$D/err" &
  helper=$!
  wait_until 10 running "$D" && wait_until 10 test -s "$D/pid.bg" || fail "cancel $sig: the job did not start"
  kill -s "$sig" "$helper"
  set +e; wait "$helper"; code=$?; set -e
  [ $code = 130 ] || fail "cancel $sig: exit $code"
  ! alive "$(cat "$D/pid.bg")" && ! alive "$(rec "$D" pid)" || fail "cancel $sig: the job survived"
  [ "$(rec "$D" state)" = cancelled ] || fail "cancel $sig: record state $(rec "$D" state)"
  ok "cancellation by SIG$sig: group stopped, record 'cancelled', exit 130"
done

# 6. Leftovers: a job that exits leaving a background process in its group.
D=$(case_dir leftovers)
LOCKRUN_DIR=$D "$L" --timeout 10 --grace 1 --quiet -- sh -c "sleep 30 & echo \$! > '$D/pid.bg'; exit 0" 2>/dev/null || fail "leftovers: exit"
! alive "$(cat "$D/pid.bg")" || fail "leftovers: the background process survived"
[ "$(rec "$D" leftovers_stopped)" = True ] || fail "leftovers: not recorded"
ok "leftovers stopped when the command exits"

# 7. A waiting job never erases the running job's record; jobs run one at a time, in order.
D=$(case_dir queue)
LOCKRUN_DIR=$D "$L" --timeout 20 --owner A --quiet -- sh -c "touch '$D/a.start'; sleep 2; touch '$D/a.end'" 2>/dev/null &
a=$!
wait_until 10 running "$D" || fail "queue: A did not start"
before=$(shasum -a 256 <"$D/record.json")
LOCKRUN_DIR=$D "$L" --timeout 20 --owner B --quiet -- sh -c "[ -f '$D/a.end' ] && touch '$D/b.ran'" 2>"$D/b.err" &
b=$!
wait_until 10 grep -q 'waiting for the lock held by A' "$D/b.err" || fail "queue: B did not report waiting for A"
[ "$(shasum -a 256 <"$D/record.json")" = "$before" ] || fail "queue: the record changed while B waited"
[ ! -f "$D/b.ran" ] || fail "queue: B ran while A held the lock"
wait "$a" || fail "queue: A failed"
wait "$b" || fail "queue: B failed (or ran before A ended)"
[ -f "$D/b.ran" ] && [ "$(rec "$D" owner)" = B ] || fail "queue: B did not run"
events=$(python3 -c 'import json,sys; print(" ".join(r["owner"]+":"+r["event"] for r in map(json.loads, open(sys.argv[1]))))' "$D/history.jsonl")
[ "$events" = "A:start A:finish B:start B:finish" ] || fail "queue: history $events"
ok "waiting job leaves the record alone; B starts only after A ends; history kept"

# 8. --lock-timeout: give up with 75, nothing runs, record untouched.
D=$(case_dir busy)
LOCKRUN_DIR=$D "$L" --timeout 20 --owner A --quiet -- sleep 2 2>/dev/null &
a=$!
wait_until 10 running "$D" || fail "busy: A did not start"
before=$(shasum -a 256 <"$D/record.json")
set +e
LOCKRUN_DIR=$D "$L" --timeout 20 --lock-timeout 0.5 --quiet -- touch "$D/b.ran" 2>/dev/null
code=$?
set -e
[ $code = 75 ] && [ ! -f "$D/b.ran" ] || fail "busy: exit $code"
[ "$(shasum -a 256 <"$D/record.json")" = "$before" ] || fail "busy: record changed"
wait "$a"
ok "--lock-timeout: exit 75, nothing ran, record untouched"

# 9. The helper is killed (SIGKILL) while its job lives: no second job starts until the job ends
#    (the trampoline still holds the lock).
D=$(case_dir crash)
LOCKRUN_DIR=$D "$L" --timeout 60 --owner A --quiet -- sleep 4 2>/dev/null &
helper=$!
wait_until 10 running "$D" || fail "crash: A did not start"
job=$(rec "$D" pid)
kill -9 "$helper"; wait "$helper" 2>/dev/null || true
alive "$job" || fail "crash: the job died with its helper (the case needs it alive)"
set +e
LOCKRUN_DIR=$D "$L" --timeout 20 --lock-timeout 1 --quiet -- touch "$D/b.ran" 2>"$D/b.err"
code=$?
set -e
[ $code = 75 ] && [ ! -f "$D/b.ran" ] || fail "crash: a second job ran while the orphaned job lived (exit $code)"
grep -q 'waiting for the lock held by A' "$D/b.err" || fail "crash: the flock itself was not held by the orphaned job's trampoline"
wait_until 15 sh -c "! kill -0 $job 2>/dev/null" || fail "crash: the job did not end"
LOCKRUN_DIR=$D "$L" --timeout 20 --lock-timeout 5 --quiet -- touch "$D/c.ran" 2>/dev/null || fail "crash: the lock stayed held after the job ended"
[ -f "$D/c.ran" ] || fail "crash: C did not run"
grep -q '"event": "abandoned"' "$D/history.jsonl" || fail "crash: the abandoned job was not noted"
ok "helper killed with its job alive: the trampoline's lock keeps others out until the job ends"

# 10. The helper and the trampoline (the lock holder) are both killed while the command
#     lives on in the group: the record's process-group guard keeps others out.
D=$(case_dir crash_both)
LOCKRUN_DIR=$D "$L" --timeout 60 --owner A --quiet -- sh -c "echo \$\$ > '$D/pid.cmd'; exec sleep 4" 2>/dev/null &
helper=$!
wait_until 10 running "$D" && wait_until 10 test -s "$D/pid.cmd" || fail "crash_both: A did not start"
tramp=$(rec "$D" pid)
cmd=$(cat "$D/pid.cmd")
kill -9 "$helper" "$tramp"; wait "$helper" 2>/dev/null || true
alive "$cmd" || fail "crash_both: the command died with its helper"
set +e
LOCKRUN_DIR=$D "$L" --timeout 20 --lock-timeout 1 --quiet -- touch "$D/b.ran" 2>"$D/b.err"
code=$?
set -e
[ $code = 75 ] && [ ! -f "$D/b.ran" ] || fail "crash_both: a second job ran (exit $code)"
grep -q "helper is gone but its process group lives" "$D/b.err" || fail "crash_both: no guard message"
wait_until 15 sh -c "! kill -0 $cmd 2>/dev/null" || fail "crash_both: the command did not end"
LOCKRUN_DIR=$D "$L" --timeout 20 --lock-timeout 5 --quiet -- touch "$D/c.ran" 2>/dev/null || fail "crash_both: lock not free after the command ended"
ok "helper and lock holder killed, command alive: the process-group guard keeps others out"

# 10b. A daemon the job starts (it leaves the group, as sccache's server does) neither
#      keeps the lock after the job nor is stopped by lockrun.
D=$(case_dir daemon)
LOCKRUN_DIR=$D "$L" --timeout 20 --quiet -- sh -c "python3 -c '
import os, sys, time
os.setsid()
open(sys.argv[1] + \".tmp\", \"w\").write(str(os.getpid()))
os.rename(sys.argv[1] + \".tmp\", sys.argv[1])
time.sleep(30)' '$D/pid.daemon' & while [ ! -s '$D/pid.daemon' ]; do sleep 0.05; done" 2>/dev/null || fail "daemon: the job failed"
daemon=$(cat "$D/pid.daemon")
alive "$daemon" || fail "daemon: lockrun stopped a process outside the job's group"
LOCKRUN_DIR=$D "$L" --timeout 20 --lock-timeout 2 --quiet -- touch "$D/b.ran" 2>/dev/null || fail "daemon: the daemon kept the lock"
[ -f "$D/b.ran" ] || fail "daemon: B did not run"
alive "$daemon" || fail "daemon: the daemon was stopped"
kill "$daemon"
ok "a daemon that leaves the job's group doesn't hold the lock and isn't stopped"

# 11. The helper dies before giving permission: the job exits without working; the lock frees.
D=$(case_dir before_go)
LOCKRUN_DIR=$D LOCKRUN_TEST_HOLD_BEFORE_GO=30 "$L" --timeout 60 --quiet -- touch "$D/worked" 2>/dev/null &
helper=$!
wait_until 10 running "$D" || fail "before_go: no record"
job=$(rec "$D" pid)
alive "$job" || fail "before_go: the trampoline is not waiting"
kill -9 "$helper"; wait "$helper" 2>/dev/null || true
wait_until 5 sh -c "! kill -0 $job 2>/dev/null" || fail "before_go: the job kept waiting after the helper died"
[ ! -f "$D/worked" ] || fail "before_go: the job worked without permission"
grep -q 'helper died before giving the job permission' "$(rec "$D" log)" || fail "before_go: no message in the log"
LOCKRUN_DIR=$D "$L" --timeout 20 --lock-timeout 2 --quiet -- true 2>/dev/null || fail "before_go: lock not free"
ok "helper killed before permission: the job exits without working; lock free"

# 12. Nesting: an inner lockrun in the same job runs at once; its own timeout still applies.
D=$(case_dir nested)
LOCKRUN_DIR=$D "$L" --timeout 20 --quiet -- sh -c "
  '$L' --timeout 5 -- sh -c 'echo inner > \"$D/inner\"' &&
  { '$L' --timeout 1 --grace 1 -- sleep 30; echo \$? > '$D/inner_timeout'; } &&
  touch '$D/outer_done'" 2>/dev/null || fail "nested: outer failed"
[ "$(cat "$D/inner")" = inner ] && [ -f "$D/outer_done" ] || fail "nested: inner did not run"
[ "$(cat "$D/inner_timeout")" = 124 ] || fail "nested: inner timeout gave $(cat "$D/inner_timeout")"
[ "$(grep -c '"event": "start"' "$D/history.jsonl")" = 1 ] || fail "nested: the inner calls took the lock"
ok "nesting: inner calls run directly (no deadlock), inner timeout 124"

# 13. A stale LOCKRUN_TOKEN (not the running job's) does not bypass the lock.
D=$(case_dir stale_token)
LOCKRUN_DIR=$D "$L" --timeout 20 --owner A --quiet -- sleep 2 2>/dev/null &
a=$!
wait_until 10 running "$D" || fail "stale_token: A did not start"
set +e
LOCKRUN_DIR=$D LOCKRUN_TOKEN=0123456789abcdef "$L" --timeout 20 --lock-timeout 0.5 --quiet -- touch "$D/b.ran" 2>"$D/b.err"
code=$?
set -e
[ $code = 75 ] && [ ! -f "$D/b.ran" ] || fail "stale_token: bypassed the lock (exit $code)"
grep -q 'stale LOCKRUN_TOKEN' "$D/b.err" || fail "stale_token: no warning"
wait "$a"
ok "a stale LOCKRUN_TOKEN waits like anyone else"

# 14. Usage: --timeout is required; --status reports.
set +e
LOCKRUN_DIR=$(case_dir usage) "$L" -- true 2>/dev/null; code=$?
set -e
[ $code = 2 ] || fail "usage: a missing --timeout gave $code"
LOCKRUN_DIR=$work/usage "$L" --status | grep -q '^lock: free' || fail "usage: --status"
ok "usage: --timeout required; --status"

alive "$bystander" || fail "the unrelated process was killed"
ok "an unrelated process outside lockrun survived every case"
echo "lockrun: all $passed checks passed"
