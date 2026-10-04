#!/usr/bin/env bash
# Tests for scripts/test_candidate, scripts/golden_approve and
# scripts/merge_candidate on tiny dummy Git repositories under $TMPDIR:
# a bare "origin", a published local clone (PUB) and a dev clone (DEV). The
# candidate commits carry a fake scripts/test that follows the summary contract
# (it reads the committed list notes/speed/test_lists/all.tsv; its behavior
# comes from the committed file fake_mode) and a copy of scripts/lockrun (run
# with isolated LOCKRUN_DIRs). No cargo, no real repo. Most test_candidate runs
# go in parallel, each with its own lock directory.
#
#     scripts/tests/safeguards.sh
#
# Prints "ok N - ..." or "not ok N - ..." per check and a count; exits 1 on
# any failure. Everything is removed afterward.
set -u
S=$(cd "$(dirname "$0")/.." && pwd)
T=$(mktemp -d "${TMPDIR:-/tmp}/safeguards-test.XXXXXX")
trap 'chmod -R u+w "$T" 2>/dev/null; rm -rf "$T"' EXIT
export LOCKRUN_DIR=$T/lock TEST_RECEIPT_DIR=$T/receipts
unset LOCKRUN_TOKEN LOCKRUN TEST_LIST_FILE
ORIGIN=$T/origin.git PUB=$T/pub DEV=$T/dev
LIST=notes/speed/test_lists/all.tsv
n=0 fails=0 OUT= RC=
: >"$T/dirs"

ok() { n=$((n + 1)); echo "ok $n - $1"; }
bad() { n=$((n + 1)); fails=$((fails + 1)); echo "not ok $n - $1"; printf '%s\n' "$OUT" | tail -25 | sed 's/^/#   /'; }
check() { local d=$1; shift; if "$@"; then ok "$d"; else bad "$d"; fi; }
has() { grep -qF -- "$1" <<<"$OUT"; }
hasnt() { ! grep -qF -- "$1" <<<"$OUT"; }
rc() { [ "$RC" = "$1" ]; }
g() { git -C "$DEV" "$@"; }

# the candidate's temporary directory, as test_candidate printed it (/tmp/cpc.*)
cdir() { sed -n 's|^test_candidate: temporary working copy: \(.*\)/wt$|\1|p' <<<"$OUT"; }
note_dir() { local d; d=$(cdir); [ -n "$d" ] && echo "$d" >>"$T/dirs"; }
tc() { OUT=$(cd "$DEV" && "$S/test_candidate" "$@" 2>&1); RC=$?; note_dir; }
# tcbg KEY ARGS...: test_candidate in the background with its own lock dir; tcget KEY afterward
tcbg() {
  local key=$1; shift
  while [ "$(jobs -rp | wc -l)" -ge 8 ]; do sleep 0.05; done
  ( cd "$DEV" && LOCKRUN_DIR=$T/lock-$key "$S/test_candidate" "$@" >"$T/out.$key" 2>&1; echo $? >"$T/rc.$key" ) &
}
tcget() { OUT=$(cat "$T/out.$1"); RC=$(cat "$T/rc.$1"); note_dir; }
mc() { OUT=$(cd "$DEV" && "$S/merge_candidate" "$@" --local "$PUB" 2>&1); RC=$?; }
ga() { OUT=$(cd "$DEV" && "$S/golden_approve" "$@" 2>&1); RC=$?; }
receipt() { g notes --ref=test-receipts show "$1" 2>/dev/null; }
# rget COMMIT PYEXPR: evaluate PYEXPR with r = the receipt JSON
rget() { receipt "$1" | python3 -c 'import json,sys; r=json.load(sys.stdin); print(eval(sys.argv[1]))' "$2"; }
# cleaned: this run's /tmp/cpc.* directory (if it made one) is gone and git lists only the dev clone
cleaned() { local d; d=$(cdir); { [ -z "$d" ] || [ ! -e "$d" ]; } && [ "$(g worktree list | wc -l | tr -d ' ')" = 1 ]; }
nodir() { [ -z "$(cdir)" ]; }
rtrue() { [ "$(rget "$1" "bool($2)")" = True ]; }
verdict_is() { [ "$(rget "$1" 'r["verdict"]')" = "$2" ]; }
problem_has() { rget "$1" '"\n".join(r["problems"])' | grep -qF -- "$2"; }
one_problem() { has "REFUSED: 1 problem(s)"; }
origin_main() { git --git-dir="$ORIGIN" rev-parse refs/heads/main; }
set_origin_main() { git --git-dir="$ORIGIN" update-ref refs/heads/main "$1"; }

# mkc PARENT [path=content | path=<delete> | path=<gitlink:ID> | path=<symlink:TARGET>]...:
# a commit without touching any working copy
mkc() {
  local parent=$1 idx=$T/idx kv p v b t; shift
  rm -f "$idx"
  GIT_INDEX_FILE=$idx g read-tree "$parent"
  for kv in "$@"; do
    p=${kv%%=*} v=${kv#*=}
    case $v in
      "<delete>") GIT_INDEX_FILE=$idx g update-index --force-remove -- "$p" ;;
      "<gitlink:"*) v=${v#<gitlink:}; GIT_INDEX_FILE=$idx g update-index --add --cacheinfo "160000,${v%>},$p" ;;
      "<symlink:"*) v=${v#<symlink:}; b=$(printf '%s' "${v%>}" | g hash-object -w --stdin)
                    GIT_INDEX_FILE=$idx g update-index --add --cacheinfo "120000,$b,$p" ;;
      *) b=$(printf '%s\n' "$v" | g hash-object -w --stdin)
         GIT_INDEX_FILE=$idx g update-index --add --cacheinfo "100644,$b,$p" ;;
    esac
  done
  t=$(GIT_INDEX_FILE=$idx g write-tree); rm -f "$idx"
  g commit-tree "$t" -p "$parent" -m "test commit: $*"
}

# forge COMMIT FROM [PY]: copy FROM's receipt onto COMMIT with its commit and tree (also in the
# checkout evidence) rewritten, then PY edits r
forge() {
  local tree; tree=$(g rev-parse "$1^{tree}")
  receipt "$2" | python3 -c '
import json, sys
r = json.load(sys.stdin)
c, t = sys.argv[1], sys.argv[2]
r["candidate"]["commit"], r["candidate"]["tree"] = c, t
for k in ("before", "after"):
    if isinstance(r.get("checkout", {}).get(k), dict):
        r["checkout"][k]["head"], r["checkout"][k]["tree"] = c, t
exec(sys.argv[3])
print(json.dumps(r, indent=1))' "$1" "$tree" "${3:-pass}" >"$T/forged.json"
  g notes --ref=test-receipts add -f -F "$T/forged.json" "$1" 2>/dev/null
}

# ---------------------------------------------------------------- dummy repos
SEED=$T/seed
git init -q -b main "$SEED"
mkdir -p "$SEED/scripts" "$SEED/answers" "$SEED/tests" "$SEED/notes/speed/test_lists"
cp "$S/lockrun" "$SEED/scripts/lockrun"
cat >"$SEED/scripts/test" <<'EOF'
#!/usr/bin/env python3
# Fake scripts/test following the summary contract; behavior from ./fake_mode.
import hashlib, json, os, subprocess, sys, time
a = sys.argv[1:]
assert a[:2] == ["--all", "--summary"], a
out, mode = a[2], open("fake_mode").read().strip()
lst = a[a.index("--list") + 1] if "--list" in a else "notes/speed/test_lists/all.tsv"
tgt = os.environ["CARGO_TARGET_DIR"]
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
G = lambda *x: subprocess.run(["git", *x], check=True, capture_output=True)
bad = []
if not os.environ.get("LOCKRUN_TOKEN"): bad.append("not run under lockrun")
if not os.path.realpath(tgt).startswith(os.path.realpath(os.getcwd()) + "/"): bad.append("target dir outside")
if not os.path.isfile("notes/pic.png"): bad.append("notes/pic.png missing: sparse checkout")
if os.path.exists("leak.txt") or open("README").read() != "readme v2\n": bad.append("dev changes leaked")
if os.environ.get("FAKE_TEST_STARTED"): open(os.environ["FAKE_TEST_STARTED"], "w").write("started\n")
if mode in ("timeout", "slow"): time.sleep(60)
os.makedirs(os.path.join(tgt, "test-logs"), exist_ok=True)
steps = []
for line in open(lst):
    if not line.strip() or line.startswith("#"): continue
    f = line.rstrip("\n").split("\t")
    n = None if f[0] == "build" else max(int(f[3]), 1)
    log = os.path.join(tgt, "test-logs", f[1] + ".log")
    open(log, "w").write("$ %s\ntest result: ok\n%s" % (f[5], "".join(b + "\n" for b in bad)))
    steps.append(dict(name=f[1], kind=f[0], command=f[5], limit=float(f[2]), least=int(f[3]),
                      group=f[6] if len(f) > 6 and f[6] not in ("", "-") else None, exit=0, seconds=0.1,
                      tests_run=n, passed=n or 0, failed=0, ignored=0, timed_out=False, ok=True, why="",
                      log=log, log_sha256=sha(log)))
s = dict(mode="all", verdict="pass", list=lst, list_sha256=sha(lst), runner_sha256=sha(os.path.abspath(__file__)),
         wall_seconds=0.3, steps=steps)
u = steps[1]
rc = 0
if bad or mode == "fail": u.update(exit=1, passed=2, failed=1, ok=False, why="1 failed"); s["verdict"] = "fail"; rc = 1
if mode == "zero": u.update(tests_run=0, passed=0)
if mode == "exit1pass": rc = 1
if mode == "lie_timedout": u["timed_out"] = True
if mode == "nosteps": s["steps"] = []
if mode == "badlist": s["list_sha256"] = "0" * 64
if mode == "badrunner": s["runner_sha256"] = "0" * 64
if mode == "badlog": u["log_sha256"] = "f" * 64
if mode == "badfield": u["command"] = 5
if mode == "onlybuild": s["steps"] = steps[:1]
if mode == "omit": s["steps"] = steps[:2]
if mode == "extra": s["steps"] = steps + [dict(u, name="bonus")]
if mode == "dup": s["steps"] = steps + [dict(u)]
if mode == "reorder": s["steps"] = [steps[0], steps[2], steps[1]]
if mode == "cmd": u["command"] = "true"
if mode == "limit": u["limit"] = 999.0
if mode == "least": u["least"] = 1
if mode == "least0": steps[2]["least"] = 0
if mode == "group": steps[2]["group"] = None
if mode == "unfinished":
    steps[2].update(exit=-15, timed_out=True, ok=False, why="timed out after 30 s"); s["verdict"] = "unfinished"; rc = 1
if mode == "mutate": open("README", "w").write("changed while testing\n")
if mode == "unstage": G("rm", "--cached", "-q", "fake_mode")
if mode == "headmove": G("checkout", "-q", "--detach", "HEAD^")
if mode == "skipflag": G("update-index", "--skip-worktree", "README"); open("README", "w").write("hidden\n")
if mode == "sparse": G("sparse-checkout", "set", "--no-cone", "/*", "!/notes/")
if mode == "stuck":
    os.makedirs(os.path.join(tgt, "stuck")); open(os.path.join(tgt, "stuck", "f"), "w").write("x\n")
    os.chmod(os.path.join(tgt, "stuck"), 0o500)
if mode != "nosummary": json.dump(s, open(out, "w"))
sys.exit(rc)
EOF
chmod +x "$SEED/scripts/test" "$SEED/scripts/lockrun"
TAB=$'\t'
LIST_V1="# dummy reviewed list
build${TAB}build${TAB}600${TAB}0${TAB}-${TAB}cargo build --release
cargo${TAB}unit${TAB}60${TAB}3${TAB}-${TAB}cargo test --release -p paint
script${TAB}second${TAB}30${TAB}1${TAB}^ok${TAB}scripts/tests/second.sh${TAB}slow"
printf '%s\n' "$LIST_V1" >"$SEED/$LIST"
printf 'pass\n' >"$SEED/fake_mode"
printf 'answer v1\n' >"$SEED/answers/a.txt"
printf '#!/bin/sh\necho golden test v1\n' >"$SEED/tests/golden_test.sh"
GOLDEN0='# protected
answers/
tests/golden_test.sh
notes/golden_paths.txt
future/absent.txt'
printf '%s\n' "$GOLDEN0" >"$SEED/notes/golden_paths.txt"
printf 'PNG-ish\n' >"$SEED/notes/pic.png"
printf 'build.out\n' >"$SEED/.gitignore"
printf 'readme v1\n' >"$SEED/README"
git -C "$SEED" add -A && git -C "$SEED" commit -qm "seed M0"
printf 'readme v2\n' >"$SEED/README"
printf '%s\nnotes/speed/test_lists/\nscripts/test\n' "$GOLDEN0" >"$SEED/notes/golden_paths.txt"
git -C "$SEED" commit -qam "seed M: the check set is protected"
git init -q --bare -b main "$ORIGIN"
git -C "$SEED" push -q "$ORIGIN" main 2>/dev/null
git clone -q "$ORIGIN" "$DEV" 2>/dev/null
git clone -q "$ORIGIN" "$PUB" 2>/dev/null
M=$(g rev-parse main) M0=$(g rev-parse main~1)
# The same sparse post-checkout hook the real repository has.
SPARSE_HOOK='#!/bin/sh
[ "$1" = 0000000000000000000000000000000000000000 ] || exit 0
gd=$(git rev-parse --absolute-git-dir) || exit 0
cd_=$(git rev-parse --path-format=absolute --git-common-dir) || exit 0
[ "$gd" != "$cd_" ] || exit 0
git sparse-checkout set --no-cone "/*" "!/notes/**/*.png" && echo "post-checkout: sparse worktree" >&2
exit 0'
printf '%s\n' "$SPARSE_HOOK" >"$DEV/.git/hooks/post-checkout"
chmod +x "$DEV/.git/hooks/post-checkout"
M2=$(mkc "$M" other.txt=main-moved)
g push -q origin "$M2:refs/heads/side" 2>/dev/null
# The dev clone is dirty throughout: its changes must never reach a candidate.
printf 'dev edit\n' >>"$DEV/README"
printf 'untracked\n' >"$DEV/leak.txt"

P=$(mkc "$M" extra.txt=from-P)
LIST_CHANGED=${LIST_V1/second.sh/second.sh --quick}

# ---------------------------------------------------------------- test_candidate (parallel runs)
MODES_FAIL="fail zero nosummary exit1pass lie_timedout nosteps badlist badlog onlybuild"
MODES_BIND="omit extra dup reorder cmd limit least least0 group badrunner"
MODES_CO="mutate unstage headmove skipflag sparse"
for mode in $MODES_FAIL $MODES_BIND $MODES_CO badfield stuck unfinished timeout; do
  C=$(mkc "$M" fake_mode="$mode"); eval "C_$mode=$C"
done
G2=$(mkc "$M" answers/a.txt=v2)
LC=$(mkc "$M" "$LIST=$LIST_CHANGED")
LU=$(mkc "$M" "$LIST=${LIST_V1/second.sh/second.sh --unapproved}")
FL=$(mkc "$M" "notes/speed/test_lists/fast.tsv=$(head -3 <<<"$LIST_V1")")
N=$(mkc "$M0" extra.txt=old-base README="readme v2" notes/golden_paths.txt="$(g show "$M:notes/golden_paths.txt")")
MID=$(mkc "$M" mid.txt=mid)
g push -q origin "$MID:refs/heads/side2" 2>/dev/null
PL=$(mkc "$MID" extra.txt=from-PL)
C_slow=$(mkc "$M" fake_mode=slow)

# P alone first: in the parallel batch the sparse post-checkout hook can lose the race for
# the shared .git/config lock, and check 8 needs to see that it ran
( cd "$DEV" && "$S/test_candidate" "$P" >"$T/out.P" 2>&1; echo $? >"$T/rc.P" )
rm -f "$T/started"
( cd "$DEV" && LOCKRUN_DIR=$T/lock-sig FAKE_TEST_STARTED=$T/started exec "$S/test_candidate" "$C_slow" >"$T/out.sig" 2>&1 ) &
sigpid=$!
tcbg timeout "$C_timeout" --timeout 2
for mode in $MODES_FAIL $MODES_BIND $MODES_CO badfield stuck unfinished; do eval 'tcbg $mode "$C_'"$mode"'"'; done
tcbg G2 "$G2"
tcbg LC "$LC"
tcbg LU "$LU"
tcbg FL "$FL" --list-file notes/speed/test_lists/fast.tsv
tcbg N "$N" --base-main "$M"
tcbg PL "$PL" --base-main "$M"
for _ in $(seq 200); do [ -f "$T/started" ] && break; sleep 0.05; done
kill -TERM "$sigpid"; wait "$sigpid"; echo $? >"$T/rc.sig"
wait

# the cleanup-failure run first: its directory can't be removed until the test unlocks it
tcget stuck; C=$C_stuck
STUCK_DIR=$(cdir)
check "cleanup failure (a test leaves an undeletable directory): verdict fail (exit 1), never pass; problem and cleanup_errors recorded" \
  eval 'rc 1 && verdict_is $C fail && problem_has $C "cleanup failed" && [ -n "$STUCK_DIR" ] && [ -e "$STUCK_DIR" ] && rtrue $C "r[\"cleanup_errors\"]"'
chmod -R u+w "$STUCK_DIR" && rm -rf "$STUCK_DIR"; g worktree prune

tcget P
pass_receipt() {
  rc 0 && rtrue "$P" "r['verdict']=='pass' and r['candidate']['commit']=='$P' and
    r['candidate']['tree']=='$(g rev-parse "$P^{tree}")' and r['base_main']['commit']=='$M' and
    r['base_main']['is_ancestor'] is True and r['exit']==0 and r['cleanup_errors']==[]"
}
log_copied() {
  local d f; d=$(rget "$P" 'r["receipt_dir"]'); f=$(rget "$P" 'r["logs"][1]["file"]')
  d=${d/#\~/$HOME}
  [ -f "$d/receipt.json" ] && [ "$(shasum -a 256 "$d/$f" | cut -d' ' -f1)" = "$(rget "$P" 'r["logs"][1]["sha256"]')" ]
}
check "pass: exit 0 and a pass receipt for the exact commit, tree and base main (an ancestor)" pass_receipt
check "pass: the step logs were copied out before cleanup and their sha256 recorded" log_copied
check "pass: receipt binds the check set: list, list and runner hashes of the commit's blobs, the 3 parsed steps (limit, least, group)" \
  rtrue "$P" "r['manifest']['list']=='$LIST' and r['manifest']['list_sha256']=='$(g show "$P:$LIST" | shasum -a 256 | cut -d' ' -f1)'
   and r['manifest']['runner_sha256']=='$(g show "$P:scripts/test" | shasum -a 256 | cut -d' ' -f1)'
   and r['manifest']['runner_sha256']==r['summary']['runner_sha256'] and r['manifest']['list_sha256']==r['summary']['list_sha256']
   and [(s['name'], s['limit'], s['least'], s['group']) for s in r['manifest']['steps']]==[('build',600.0,0,None),('unit',60.0,3,None),('second',30.0,1,'slow')]"
check "pass: receipt has checkout evidence before and after (HEAD, tree, index = commit tree, no flags, not sparse, no tracked changes)" \
  rtrue "$P" "all(r['checkout'][k]['head']=='$P' and r['checkout'][k]['index_matches_commit'] is True and r['checkout'][k]['flagged_count']==0
   and r['checkout'][k]['sparse'] is False and r['checkout'][k]['tracked_changed'] is False
   and r['checkout'][k]['files_index']==r['checkout'][k]['files_tree'] for k in ('before','after'))"
check "pass: receipt has build settings and the tool hash" rtrue "$P" "r['build']['profiles_used']==['release'] and len(r['tool']['sha256'])==64"
check "pass: its /tmp/cpc.* directory and worktree removed" cleaned
check "pass: the sparse post-checkout hook ran and the checkout was made full (fake test saw notes/pic.png)" \
  has "post-checkout: sparse worktree"
check "dirty dev clone (modified README, untracked leak.txt): the candidate was tested clean" verdict_is "$P" pass

fail_case() { # MODE WANT DESCRIPTION
  local mode=$1 want=$2; tcget "$mode"; C=$(eval echo "\$C_$mode")
  check "$3: exit 1, fail receipt saying \"$want\", directory removed" \
    eval 'rc 1 && verdict_is $C fail && problem_has $C "$want" && cleaned'
}
fail_case fail "step unit exited 1" "a failing step"
fail_case zero "ran no tests" "a step with zero tests"
fail_case nosummary "wrote no summary" "no summary"
fail_case exit1pass "scripts/test (under lockrun) exited 1" "exit 1 with a pass summary"
fail_case lie_timedout "timed out (timed_out=True)" "a step marked timed out under a pass verdict"
fail_case nosteps "an empty test selection never passes" "no steps"
fail_case badlist "list_sha256 is not the hash" "a wrong list hash"
fail_case badlog "differs from the summary's" "a wrong log hash"
fail_case onlybuild "only build steps" "only the build step reported"
fail_case omit "the summary omits listed step(s): second" "two listed test steps, only one reported (reviewer's case)"
fail_case extra "step(s) the list does not: bonus" "an extra step"
fail_case dup "more than once: unit" "a duplicated step"
fail_case reorder "is not the list's (build, unit, second)" "reordered steps"
fail_case cmd "step unit: the summary's command is 'true'" "a changed step command"
fail_case limit "step unit: the summary's limit is 999.0" "a changed step limit"
fail_case least "step unit: the summary's least is 1" "a changed step minimum (least)"
fail_case least0 "a non-build step must require at least 1" "a non-build step with least 0 in the summary"
fail_case group "step second: the summary's group is None" "a changed step group"
fail_case badrunner "runner_sha256 is not the hash of scripts/test" "a runner hash that is not scripts/test's"
fail_case badfield "reading the build settings failed" "a malformed summary field that crashes result reading (cleanup still runs)"
fail_case mutate "tracked files or the index changed after the test" "the test edits a tracked file (reviewer's case)"
fail_case unstage "the index was not the commit's tree after the test" "the test changes the index"
fail_case headmove "HEAD after the test was" "the test moves HEAD"
fail_case skipflag "skip-worktree/assume-unchanged flags after the test" "the test sets skip-worktree and hides an edit"
fail_case sparse "sparse checkout after the test" "the test makes the checkout sparse"
check "the edited-file receipt records the evidence (tracked_changed true, the changed path)" \
  rtrue "$C_mutate" "r['checkout']['after']['tracked_changed'] is True and any('README' in l for l in r['checkout']['after']['tracked_changes'])"

tcget unfinished; C=$C_unfinished
check "an inner step timeout (summary verdict unfinished, runner exit 1): unfinished receipt (exit 3), not fail, never pass" \
  eval 'rc 3 && verdict_is $C unfinished && problem_has $C "reported unfinished work (timed out: second)" && cleaned'
tcget timeout; C=$C_timeout
check "outer timeout: unfinished receipt (exit 3), never a pass, directory removed" \
  eval 'rc 3 && verdict_is $C unfinished && problem_has $C "timed out after 2 s" && cleaned'
OUT=$(cat "$T/out.sig"); RC=$(cat "$T/rc.sig"); note_dir; C=$C_slow
check "SIGTERM mid-test: unfinished receipt, test stopped, directory removed" \
  eval 'rc 3 && verdict_is $C unfinished && problem_has $C "cancelled (SIGTERM)" && cleaned'
tcget LC
check "a candidate that changes the list is tested on its own list (the binding is to the commit's blobs; approval is merge's job)" \
  eval 'rc 0 && verdict_is $LC pass'
tcget FL
check "--list-file: another list in the commit is run with --list and bound (fast.tsv, 2 steps)" \
  eval 'rc 0 && rtrue $FL "r[\"manifest\"][\"list\"]==\"notes/speed/test_lists/fast.tsv\" and len(r[\"summary\"][\"steps\"])==2"'
for k in G2 N PL; do tcget $k; done
tcget LU
all_receipts() {
  local c k
  for k in P timeout $MODES_FAIL $MODES_BIND $MODES_CO badfield stuck unfinished slow; do
    if [ "$k" = P ]; then c=$P; else eval 'c=$C_'"$k"; fi
    receipt "$c" | grep -q "\"commit\": \"$c\"" || return 1
  done
  for c in $G2 $LC $LU $FL $N $PL; do receipt "$c" | grep -q "\"commit\": \"$c\"" || return 1; done
}
check "the $(set -- P timeout $MODES_FAIL $MODES_BIND $MODES_CO badfield stuck unfinished slow G2 LC LU FL N PL; echo $#) concurrent runs each kept their own receipt note (writes serialized, read back)" all_receipts

# ---------------------------------------------------------------- test_candidate refusals
refused() { # COMMIT WANT DESCRIPTION [ARGS]
  local c=$1 want=$2 d=$3; shift 3
  tc "$c" "$@"
  check "refuses $d (exit 2, no receipt, no directory)" eval 'rc 2 && has "$want" && ! receipt $c >/dev/null && nodir && cleaned'
}
refused "$(mkc "$M" "$LIST=<delete>")" "notes/speed/test_lists/all.tsv is not in" "a commit without the test list"
refused "$(mkc "$M" scripts/test="<delete>")" "scripts/test is not in" "a commit without scripts/test"
refused "$(mkc "$M" "$LIST=${LIST_V1/${TAB}3${TAB}/${TAB}0${TAB}}")" "every non-build step must require at least 1" \
  "a list whose non-build step requires 0 tests"
refused "$(mkc "$M" "$LIST=$(head -2 <<<"$LIST_V1")")" "only build steps" "a list with no non-build step"
refused "$(mkc "$M" "$LIST=${LIST_V1/${TAB}-${TAB}cargo test/ cargo test}")" "want 6 or 7 tab-separated fields" "a malformed list"
refused "$(mkc "$M" "$LIST=<symlink:../../../README>")" "is not a regular file" "a list that is a symlink"
tc main
check "refuses a branch name (it can move)" eval 'rc 2 && has "give an exact commit id" && cleaned'
tc deadbeefdeadbeef
check "refuses an unknown commit id" eval 'rc 2 && has "no such commit"'
g branch "${P:0:10}" "$M" 2>/dev/null
tc "${P:0:10}"
check "refuses an id that is also a branch name" eval 'rc 2 && has "is also a ref name"'
g branch -D "${P:0:10}" >/dev/null 2>&1
g tag "${P:0:12}" "$M"
tc "${P:0:12}"
check "refuses a hex prefix that is also a lightweight tag (reviewer's case)" eval 'rc 2 && has "is also a ref name (refs/tags/${P:0:12})"'
g tag -d "${P:0:12}" >/dev/null

printf '%s\necho stray >>README\n' "$SPARSE_HOOK" | sed '/^exit 0$/d' >"$DEV/.git/hooks/post-checkout"
tc "$P"
check "refuses a checkout that is not clean (a hook edited README); no receipt written; directory removed" \
  eval 'rc 2 && has "not a full checkout of the commit" && verdict_is $P pass && cleaned'
printf '%s\n' "$SPARSE_HOOK" >"$DEV/.git/hooks/post-checkout"

# ---------------------------------------------------------------- golden_approve
G1=$(mkc "$M" tests/golden_test.sh=changed)
GX=$(mkc "$M" answers/a.txt=v-other)
G3=$(mkc "$M" answers/a.txt="<delete>")
GL=$(mkc "$M" notes/golden_paths.txt=tests/golden_test.sh answers/a.txt=v3)
GG=$(mkc "$M" "answers/a.txt=<gitlink:$M0>")
GS=$(mkc "$M" "tests/golden_test.sh=<symlink:/bin/true>")
FA=$(mkc "$M" future/absent.txt=now-here)
RN=$(mkc "$M" scripts/test=changed-runner)
ga check --candidate "$G1" --base "$M"
check "changed approved test without approval: refused" eval 'rc 1 && has "UNAPPROVED M tests/golden_test.sh"'
ga check --candidate "$G2" --base "$M"
check "unapproved answer change: refused" eval 'rc 1 && has "answers/a.txt"'
ga record --commit "$G2" --approver Speed --builder speed --reason "self"
check "self-approval (approver == builder, any case): refused and nothing recorded" \
  eval 'rc 2 && has "cannot approve its own" && ! g rev-parse -q --verify refs/notes/golden-approvals >/dev/null'
g notes --ref=golden-approvals append -m "{\"format\":\"claude-paint golden approval v1\",\"commit\":\"$G2\",\"approver\":\"lead\",\"builder\":\"LEAD\",\"reason\":\"x\",\"paths\":[{\"path\":\"answers/a.txt\",\"blob\":\"$(g rev-parse "$G2:answers/a.txt")\"}]}" "$G2" 2>/dev/null
ga check --candidate "$G2" --base "$M"
check "a hand-written self-approval note is ignored by check" eval 'rc 1 && has "UNAPPROVED"'
ga record --commit "$GX" --approver lead --builder speed --reason "other answer" --paths answers/a.txt
ga check --candidate "$G2" --base "$M"
check "approval for a different blob of the same path: still refused" eval 'rc 1 && has "UNAPPROVED"'
ga check --candidate "$GX" --base "$M"
check "that approval does cover its own blob" eval 'rc 0 && has "approved   M answers/a.txt (mode 100644"'
ga check --candidate "$G3" --base "$M"
check "deletion of a protected file needs approval" eval 'rc 1 && has "UNAPPROVED D answers/a.txt (deleted)"'
ga record --commit "$G3" --approver lead --builder speed --reason "drop it" --paths answers/a.txt
ga check --candidate "$G3" --base "$M"
check "an approved deletion passes" eval 'rc 0'
ga check --candidate "$GL" --base "$M"
check "dropping a path from the list and changing it: both refused; the list does not unprotect (reviewer's case)" \
  eval 'rc 1 && has "UNAPPROVED M notes/golden_paths.txt" && has "UNAPPROVED M answers/a.txt"'
ga check --candidate "$GG" --base "$M"
check "a protected file replaced by a submodule entry (gitlink): listed and refused as an unsupported type (reviewer's case)" \
  eval 'rc 1 && has "protected changes" && has "UNAPPROVED T answers/a.txt (mode 160000, object ${M0:0:12})" && has "unsupported type submodule"'
g notes --ref=golden-approvals append -m "{\"format\":\"claude-paint golden approval v1\",\"commit\":\"$GG\",\"approver\":\"lead\",\"builder\":\"speed\",\"reason\":\"x\",\"paths\":[{\"path\":\"answers/a.txt\",\"blob\":\"$M0\"}]}" "$GG" 2>/dev/null
ga check --candidate "$GG" --base "$M"
check "an approval naming the submodule's object but not its mode does not cover it" eval 'rc 1 && has "UNAPPROVED T answers/a.txt"'
ga record --commit "$GG" --approver lead --builder speed --reason "submodule on purpose" --paths answers/a.txt
ga check --candidate "$GG" --base "$M"
check "an approval naming exactly mode 160000 and that object covers it" eval 'rc 0 && has "approved   T answers/a.txt (mode 160000"'
ga check --candidate "$GS" --base "$M"
check "a protected file replaced by a symlink: refused as an unsupported type" \
  eval 'rc 1 && has "UNAPPROVED T tests/golden_test.sh (mode 120000" && has "unsupported type symlink"'
ga check --candidate "$P" --base "$M"
check "a listed path absent at base and candidate (future/absent.txt) is fine" eval 'rc 0 && has ": 0"'
ga check --candidate "$FA" --base "$M"
check "when that listed path appears, it needs approval" eval 'rc 1 && has "UNAPPROVED A future/absent.txt"'
ga check --candidate "$LC" --base "$M"
check "a change to the test list (protected) needs approval" eval 'rc 1 && has "UNAPPROVED M $LIST"'
ga check --candidate "$RN" --base "$M"
check "a change to scripts/test (protected) needs approval" eval 'rc 1 && has "UNAPPROVED M scripts/test"'
ga record --commit "$LC" --approver lead --builder speed --reason "reviewed list change" --paths "$LIST"
ga check --candidate "$LC" --base "$M"
check "an approval of the exact list blob covers that list" eval 'rc 0'
LC2=$(mkc "$M" "$LIST=${LIST_CHANGED/--quick/--quicker}")
ga check --candidate "$LC2" --base "$M"
check "the approval is tied to the exact blob: another edit of the list is refused" eval 'rc 1 && has "UNAPPROVED M $LIST"'
ga record --commit "$M" --approver lead --builder speed --reason baseline --dry-run
check "default record covers every protected file at the commit (baseline style), with modes" \
  eval 'rc 0 && has "5 path(s)" && has "100644" && has "100755" && has "scripts/test" && has "notes/golden_paths.txt"'

# ---------------------------------------------------------------- merge_candidate refusals
mc "$G1"
check "merge: missing receipt refused" eval 'rc 1 && has "no test receipt"'
mc "$C_fail"
check "merge: failed-test receipt refused" eval 'rc 1 && has "verdict is '"'"'fail'"'"'"'
mc "$C_mutate"
check "merge: the receipt of a run that edited a tracked file refused (verdict and evidence)" \
  eval 'rc 1 && has "verdict is '"'"'fail'"'"'" && has "checkout evidence: tracked files or the index changed after the test"'
Q=$(mkc "$M" extra.txt=from-Q)
g notes --ref=test-receipts copy "$P" "$Q" 2>/dev/null
mc "$Q"
check "merge: a receipt for another commit (proof for another version) refused" eval 'rc 1 && has "receipt is for another commit"'
B=$(g commit-tree "$(g rev-parse "$P^{tree}")" -p "$M" -m "same tree as P")
g notes --ref=test-receipts copy "$P" "$B" 2>/dev/null
mc "$B"
check "merge: an unchanged receipt copied to a commit with the same tree refused (reviewer's case)" \
  eval 'rc 1 && has "receipt is for another commit"'
R=$(mkc "$M" extra.txt=from-R)
forge "$R" "$P" 'r["candidate"]["tree"] = "'"$(g rev-parse "$P^{tree}")"'"'
mc "$R"
check "merge: receipt tree mismatch refused" eval 'rc 1 && one_problem && has "tree"'
E1=$(mkc "$M" extra.txt=from-E1)
forge "$E1" "$P" 'r["checkout"]["after"]["tracked_changed"] = True'
mc "$E1"
check "merge: a pass receipt whose checkout evidence says tracked files changed refused" \
  eval 'rc 1 && one_problem && has "tracked files or the index changed after the test"'
forge "$E1" "$P" 'r["checkout"]["after"].update(flagged_count=1, flagged=["S README"])'
mc "$E1"
check "merge: a pass receipt whose evidence shows skip-worktree flags refused" eval 'rc 1 && one_problem && has "skip-worktree"'
forge "$E1" "$P" 'del r["checkout"]["after"]'
mc "$E1"
check "merge: a pass receipt without checkout evidence after the test refused" eval 'rc 1 && one_problem && has "no checkout evidence after the test"'
forge "$E1" "$P" 'r["cleanup_errors"] = ["/tmp/cpc.x still exists"]'
mc "$E1"
check "merge: a pass receipt that records a cleanup error refused" eval 'rc 1 && one_problem && has "cleanup errors"'
forge "$E1" "$P" 'r["summary"]["steps"] = r["summary"]["steps"][:2]'
mc "$E1"
check "merge: a forged pass receipt whose summary omits a listed step refused (re-verified against the commit's list)" \
  eval 'rc 1 && one_problem && has "omits listed step(s): second"'
forge "$E1" "$P" 'r["manifest"]["steps"][1]["least"] = 1; r["summary"]["steps"][1]["least"] = 1'
mc "$E1"
check "merge: a receipt whose manifest and summary agree on a weaker minimum than the commit's list refused" \
  eval 'rc 1 && has "manifest steps differs" && has "the summary'"'"'s least is 1"'
forge "$RN" "$P"
mc "$RN"
check "merge: scripts/test changed after the test (runner hash mismatch) refused" \
  eval 'rc 1 && has "manifest runner_sha256 differs" && has "runner_sha256 is not the hash of scripts/test"'
mc "$LU"
check "merge: a passing candidate that changed the list without approval refused" \
  eval 'verdict_is $LU pass && rc 1 && one_problem && has "unapproved protected change: M $LIST"'
P2=$(mkc "$P" extra.txt=branch-moved)
mc "$P2"
check "merge: branch moved after the test (new tip has no receipt) refused" eval 'rc 1 && has "no test receipt"'
mc main
check "merge: a branch name instead of an exact id refused" eval 'rc 2 && has "give an exact commit id"'
L=$(mkc "$M" extra.txt=from-L "$LIST=${LIST_V1/${TAB}60${TAB}/${TAB}61${TAB}}")
forge "$L" "$P"
mc "$L"
check "merge: test list in the candidate differs from the tested list: refused" \
  eval 'rc 1 && has "manifest list_sha256 differs" && has "manifest steps differs" && has "the summary'"'"'s limit is 60.0, the list'"'"'s 61.0"'
mc "$FL"
check "merge: a receipt for another list than all.tsv refused" eval 'rc 1 && has "not the reviewed $LIST"'
NB=$(mkc "$M0" extra.txt=from-NB README="readme v2")
forge "$NB" "$P" 'r["base_main"]["commit"] = "'"$M0"'"'
mc "$NB"
check "merge: refused when the list and scripts/test are not protected paths at base or candidate" \
  eval 'rc 1 && has "$LIST is not a protected path" && has "scripts/test is not a protected path"'
mc "$G2"
check "merge: passing tests with an unapproved answer change refused" \
  eval 'rc 1 && one_problem && has "unapproved protected change: M answers/a.txt"'
set_origin_main "$M2"
mc "$P"
check "merge: remote main moved after the test refused" eval 'rc 1 && one_problem && has "main moved since the test"'
set_origin_main "$M"
git -C "$PUB" commit -q --allow-empty -m "local work" 2>/dev/null
mc "$P"
check "merge: published local main moved refused" eval 'rc 1 && one_problem && has "HEAD is"'
git -C "$PUB" reset -q --hard "$M" 2>/dev/null
echo edit >>"$PUB/README"
mc "$P"
check "merge: published copy with a modified file refused" eval 'rc 1 && one_problem && has " M README"'
git -C "$PUB" checkout -q -- README 2>/dev/null
echo new >"$PUB/new.txt"
mc "$P"
check "merge: published copy with an untracked file refused" eval 'rc 1 && one_problem && has "?? new.txt"'
rm "$PUB/new.txt"
git -C "$PUB" update-index --assume-unchanged README
echo hidden >>"$PUB/README"
mc "$P"
check "merge: published copy with an assume-unchanged entry hiding an edit refused" \
  eval 'rc 1 && one_problem && has "skip-worktree/assume-unchanged entries" && has "h README"'
git -C "$PUB" update-index --no-assume-unchanged README; git -C "$PUB" checkout -q -- README
git -C "$PUB" checkout -q --detach "$M" 2>/dev/null
mc "$P"
check "merge: published copy not on branch main refused" eval 'rc 1 && one_problem && has "not on branch main"'
git -C "$PUB" checkout -q main 2>/dev/null
I=$(mkc "$M" build.out=tracked-now)
forge "$I" "$P"
echo mine >"$PUB/build.out"
mc "$I"
check "merge: an ignored file the candidate would overwrite refused" eval 'rc 1 && one_problem && has "ignored files"'
ID=$(mkc "$M" build.out/inner.txt=tracked-now)
forge "$ID" "$P"
mc "$ID"
check "merge: an ignored file where the candidate needs a directory (parent-path collision) refused" \
  eval 'rc 1 && one_problem && has "where the candidate needs a directory: build.out"'
rm "$PUB/build.out"
mc "$N"
check "merge: candidate that does not contain main refused (tests passed, is_ancestor false)" \
  eval 'verdict_is $N pass && [ "$(rget $N "r[\"base_main\"][\"is_ancestor\"]")" = False ] && rc 1 && one_problem && has "does not contain the recorded main"'
check "after all refusals: origin main and the published copy unchanged" \
  eval '[ "$(origin_main)" = "$M" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ] && [ -z "$(git -C "$PUB" status --porcelain)" ]'

# ---------------------------------------------------------------- races and recovery
# Main moves to MID, which the candidate PL contains: a plain push would be a
# fast-forward and succeed, so only the lease can stop it.
# Test fixtures only: a git wrapper first on PATH acts at one step of merge_candidate
# (WRAP_AT: before-push, after-push-move, query-fails, fetch-ignored).
REAL_GIT=$(command -v git)
mkdir -p "$T/bin"
cat >"$T/bin/git" <<EOF
#!/bin/sh
real="$REAL_GIT"
case "\$WRAP_AT:\$3" in
  before-push:push) "\$real" --git-dir="$ORIGIN" update-ref refs/heads/main $MID ;;
  after-push-move:push) "\$real" "\$@"; r=\$?; "\$real" --git-dir="$ORIGIN" update-ref refs/heads/main $MID; exit \$r ;;
  query-fails:push) "\$real" "\$@"; r=\$?; touch "$T/pushed"; exit \$r ;;
  query-fails:ls-remote) [ -e "$T/pushed" ] && { echo "fatal: unable to reach the remote" >&2; exit 128; } ;;
  fetch-ignored:fetch) echo mine >"$PUB/build.out" ;;
esac
exec "\$real" "\$@"
EOF
chmod +x "$T/bin/git"
mcw() { OUT=$(cd "$DEV" && WRAP_AT=$1 PATH=$T/bin:$PATH "$S/merge_candidate" "$2" --local "$PUB" 2>&1); RC=$?; }
mcw before-push "$PL"
check "lease: main moved after the checks, before the push, to a commit the candidate contains (a plain push would fast-forward): rejected" \
  eval 'verdict_is $PL pass && rc 3 && has "stale info" && has "PUSH FAILED" && [ "$(origin_main)" = "$MID" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ]'
set_origin_main "$M"
# During the push itself (pre-push hook moves main): the server's compare-and-swap rejects it.
printf '#!/bin/sh\ngit --git-dir="%s" update-ref refs/heads/main %s\n' "$ORIGIN" "$MID" >"$DEV/.git/hooks/pre-push"
chmod +x "$DEV/.git/hooks/pre-push"
mc "$PL"
check "main moved during the push (pre-push hook): rejected, nothing overwritten" \
  eval 'rc 3 && has "PUSH FAILED" && [ "$(origin_main)" = "$MID" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ]'
set_origin_main "$M"
printf '#!/bin/sh\ntouch "%s/.git/index.lock"\n' "$PUB" >"$DEV/.git/hooks/pre-push"
mc "$P"
check "local update fails after the remote update: exit 4, files untouched, recovery reports what it observed (remote at the candidate, local HEAD, the merge ran and failed, status) and no blanket assurance" \
  eval 'rc 4 && has "Observed just now" && has "remote origin main: $P" && has "the push landed" && has "HEAD $M, the recorded main" && has "merge --ff-only in the published copy: ran and failed" && has "git merge --ff-only --no-overwrite-ignore $P" && hasnt "Nothing in it was overwritten" && [ "$(origin_main)" = "$P" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ] && [ ! -e "$PUB/extra.txt" ]'
rm -f "$PUB/.git/index.lock" "$DEV/.git/hooks/pre-push"
set_origin_main "$M"
mcw fetch-ignored "$I"
check "an ignored file created after the last check (during fetch) is not overwritten: merge --no-overwrite-ignore refuses, exit 4, recovery says the merge ran and failed" \
  eval 'rc 4 && has "merge --ff-only in the published copy: ran and failed" && [ "$(cat "$PUB/build.out")" = mine ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ]'
rm -f "$PUB/build.out"
set_origin_main "$M"
mcw query-fails "$P"
check "the remote query fails after the push: exit 5, recovery printed with the remote state UNKNOWN" \
  eval 'rc 5 && has "STOPPED AFTER THE PUSH" && has "UNKNOWN: the query failed" && has "whether the push landed is not known" && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ]'
rm -f "$T/pushed"
set_origin_main "$M"
mcw after-push-move "$P"
check "main moved right after a successful push: exit 5 (not 'push failed'), recovery reports the remote at neither commit" \
  eval 'rc 5 && has "remote origin main: $MID" && has "at neither commit" && hasnt "PUSH FAILED" && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ]'
set_origin_main "$M"

# ---------------------------------------------------------------- end to end
mc "$P"
check "end to end: pushed with the lease, published copy fast-forwarded with its files updated" \
  eval 'rc 0 && [ "$(origin_main)" = "$P" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$P" ] && [ "$(cat "$PUB/extra.txt")" = from-P ] && [ -z "$(git -C "$PUB" status --porcelain)" ]'
check "end to end: the exact guarded push command was used" has "running: git push --force-with-lease=main:$M origin $P:main"
check "every /tmp/cpc.* directory this suite's runs created is gone, and only the dev clone is a worktree" \
  eval '[ -s "$T/dirs" ] && ! (while read -r d; do [ -e "$d" ] && echo "$d"; done <"$T/dirs" | grep -q .) && [ "$(g worktree list | wc -l | tr -d " ")" = 1 ]'

echo "# $n checks, $((n - fails)) passed, $fails failed"
[ "$fails" = 0 ]
