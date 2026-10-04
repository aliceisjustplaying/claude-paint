#!/usr/bin/env bash
# Tests for scripts/test_candidate, scripts/golden_approve and
# scripts/merge_candidate on tiny dummy Git repositories under $TMPDIR:
# a bare "origin", a published local clone (PUB) and a dev clone (DEV). The
# candidate commits carry a fake scripts/test that follows the summary contract
# (its behavior comes from the committed file fake_mode) and a copy of
# scripts/lockrun (run with an isolated LOCKRUN_DIR). No cargo, no real repo.
#
#     scripts/tests/safeguards.sh
#
# Prints "ok N - ..." or "not ok N - ..." per check and a count; exits 1 on
# any failure. Everything is removed afterward.
set -u
S=$(cd "$(dirname "$0")/.." && pwd)
T=$(mktemp -d "${TMPDIR:-/tmp}/safeguards-test.XXXXXX")
trap 'rm -rf "$T"' EXIT
export LOCKRUN_DIR=$T/lock TEST_RECEIPT_DIR=$T/receipts
unset LOCKRUN_TOKEN LOCKRUN TEST_LIST_FILE
ORIGIN=$T/origin.git PUB=$T/pub DEV=$T/dev CT=$T/cand-tmp
mkdir -p "$CT"
n=0 fails=0 OUT= RC=

ok() { n=$((n + 1)); echo "ok $n - $1"; }
bad() { n=$((n + 1)); fails=$((fails + 1)); echo "not ok $n - $1"; printf '%s\n' "$OUT" | tail -25 | sed 's/^/#   /'; }
check() { local d=$1; shift; if "$@"; then ok "$d"; else bad "$d"; fi; }
has() { grep -qF -- "$1" <<<"$OUT"; }
rc() { [ "$RC" = "$1" ]; }
g() { git -C "$DEV" "$@"; }

tc() { OUT=$(cd "$DEV" && TMPDIR=$CT "$S/test_candidate" "$@" 2>&1); RC=$?; }
mc() { OUT=$(cd "$DEV" && "$S/merge_candidate" "$@" --local "$PUB" 2>&1); RC=$?; }
ga() { OUT=$(cd "$DEV" && "$S/golden_approve" "$@" 2>&1); RC=$?; }
receipt() { g notes --ref=test-receipts show "$1" 2>/dev/null; }
# rget COMMIT PYEXPR: evaluate PYEXPR with r = the receipt JSON
rget() { receipt "$1" | python3 -c 'import json,sys; r=json.load(sys.stdin); print(eval(sys.argv[1]))' "$2"; }
cleaned() { [ -z "$(ls -A "$CT")" ] && [ "$(g worktree list | wc -l | tr -d ' ')" = 1 ]; }
rtrue() { [ "$(rget "$1" "bool($2)")" = True ]; }
verdict_is() { [ "$(rget "$1" 'r["verdict"]')" = "$2" ]; }
problem_has() { rget "$1" '"\n".join(r["problems"])' | grep -qF -- "$2"; }
one_problem() { has "REFUSED: 1 problem(s)"; }
origin_main() { git --git-dir="$ORIGIN" rev-parse refs/heads/main; }
set_origin_main() { git --git-dir="$ORIGIN" update-ref refs/heads/main "$1"; }

# mkc PARENT [path=content | path=<delete>]...: a commit without touching any working copy
mkc() {
  local parent=$1 idx=$T/idx kv p v b t; shift
  rm -f "$idx"
  GIT_INDEX_FILE=$idx g read-tree "$parent"
  for kv in "$@"; do
    p=${kv%%=*} v=${kv#*=}
    if [ "$v" = "<delete>" ]; then GIT_INDEX_FILE=$idx g update-index --force-remove -- "$p"
    else
      b=$(printf '%s\n' "$v" | g hash-object -w --stdin)
      GIT_INDEX_FILE=$idx g update-index --add --cacheinfo "100644,$b,$p"
    fi
  done
  t=$(GIT_INDEX_FILE=$idx g write-tree); rm -f "$idx"
  g commit-tree "$t" -p "$parent" -m "test commit: $*"
}

# forge COMMIT FROM [PY]: copy FROM's receipt onto COMMIT with commit/tree rewritten, then PY edits r
forge() {
  local tree; tree=$(g rev-parse "$1^{tree}")
  receipt "$2" | python3 -c '
import json, sys
r = json.load(sys.stdin)
r["candidate"]["commit"], r["candidate"]["tree"] = sys.argv[1], sys.argv[2]
exec(sys.argv[3])
print(json.dumps(r, indent=1))' "$1" "$tree" "${3:-pass}" >"$T/forged.json"
  g notes --ref=test-receipts add -f -F "$T/forged.json" "$1" 2>/dev/null
}

# ---------------------------------------------------------------- dummy repos
SEED=$T/seed
git init -q -b main "$SEED"
mkdir -p "$SEED/scripts" "$SEED/answers" "$SEED/tests" "$SEED/notes"
cp "$S/lockrun" "$SEED/scripts/lockrun"
cat >"$SEED/scripts/test" <<'EOF'
#!/usr/bin/env python3
# Fake scripts/test following the summary contract; behavior from ./fake_mode.
import hashlib, json, os, sys, time
assert sys.argv[1:3] == ["--all", "--summary"], sys.argv
out, mode = sys.argv[3], open("fake_mode").read().strip()
tgt = os.environ["CARGO_TARGET_DIR"]
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
bad = []
if not os.environ.get("LOCKRUN_TOKEN"): bad.append("not run under lockrun")
if not os.path.realpath(tgt).startswith(os.path.realpath(os.getcwd()) + "/"): bad.append("target dir outside")
if not os.path.isfile("notes/pic.png"): bad.append("notes/pic.png missing: sparse checkout")
if os.path.exists("leak.txt") or open("README").read() != "readme v2\n": bad.append("dev changes leaked")
if os.environ.get("FAKE_TEST_STARTED"): open(os.environ["FAKE_TEST_STARTED"], "w").write("started\n")
if mode in ("timeout", "slow"): time.sleep(60)
os.makedirs(os.path.join(tgt, "test-logs"), exist_ok=True)
log = os.path.join(tgt, "test-logs", "unit.log")
open(log, "w").write("running 3 tests\ntest result: ok\n" + "".join(b + "\n" for b in bad))
step = dict(name="unit", command="cargo test --release -p paint", exit=0, seconds=0.1, tests_run=3,
            passed=3, failed=0, ignored=0, timed_out=False, log=log, log_sha256=sha(log))
s = dict(mode="all", verdict="pass", list="tests.list", list_sha256=sha("tests.list"), steps=[step])
rc = 0
if bad or mode == "fail": step.update(exit=1, passed=2, failed=1); s["verdict"] = "fail"; rc = 1
if mode == "zero": step.update(tests_run=0, passed=0)
if mode == "exit1pass": rc = 1
if mode == "lie_timedout": step["timed_out"] = True
if mode == "nosteps": s["steps"] = []
if mode == "badlist": s["list_sha256"] = "0" * 64
if mode == "badlog": step["log_sha256"] = "f" * 64
if mode in ("withbuild", "onlybuild"):
    build = dict(name="build", kind="build", command="cargo build --release", exit=0, seconds=0.1, tests_run=None,
                 passed=0, failed=0, ignored=0, timed_out=False, log=log, log_sha256=sha(log))
    s["steps"] = [build, step] if mode == "withbuild" else [build]
if mode != "nosummary": json.dump(s, open(out, "w"))
sys.exit(rc)
EOF
chmod +x "$SEED/scripts/test" "$SEED/scripts/lockrun"
printf 'unit: cargo test --release -p paint\n' >"$SEED/tests.list"
printf 'pass\n' >"$SEED/fake_mode"
printf 'answer v1\n' >"$SEED/answers/a.txt"
printf '#!/bin/sh\necho golden test v1\n' >"$SEED/tests/golden_test.sh"
printf '# protected\nanswers/\ntests/golden_test.sh\nnotes/golden_paths.txt\n' >"$SEED/notes/golden_paths.txt"
printf 'PNG-ish\n' >"$SEED/notes/pic.png"
printf 'build.out\n' >"$SEED/.gitignore"
printf 'readme v1\n' >"$SEED/README"
git -C "$SEED" add -A && git -C "$SEED" commit -qm "seed M0"
printf 'readme v2\n' >"$SEED/README"
git -C "$SEED" commit -qam "seed M"
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

# ---------------------------------------------------------------- test_candidate
tc "$P"
pass_receipt() {
  rc 0 && rtrue "$P" "r['verdict']=='pass' and r['candidate']['commit']=='$P' and
    r['candidate']['tree']=='$(g rev-parse "$P^{tree}")' and r['base_main']['commit']=='$M' and
    r['base_main']['is_ancestor'] is True and r['exit']==0"
}
log_copied() {
  local d f; d=$(rget "$P" 'r["receipt_dir"]'); f=$(rget "$P" 'r["logs"][0]["file"]')
  d=${d/#\~/$HOME}
  [ -f "$d/receipt.json" ] && [ "$(shasum -a 256 "$d/$f" | cut -d' ' -f1)" = "$(rget "$P" 'r["logs"][0]["sha256"]')" ]
}
check "pass: exit 0 and a pass receipt for the exact commit, tree and base main (an ancestor)" pass_receipt
check "pass: the step log was copied out before cleanup and its sha256 recorded" log_copied
check "pass: receipt has build settings, list hash and tool hash" rtrue "$P" \
  "r['build']['profiles_used']==['release'] and r['test_list']['sha256']==r['summary']['list_sha256']
   and len(r['tool']['sha256'])==64 and r['test_list']['file']=='tests.list'"
check "pass: temporary worktree and its build dir removed" cleaned
check "pass: the sparse post-checkout hook ran and the checkout was made full (fake test saw notes/pic.png)" \
  has "post-checkout: sparse worktree"
check "dirty dev clone (modified README, untracked leak.txt): the candidate was tested clean" verdict_is "$P" pass

for spec in "fail|step unit exited 1" "zero|ran no tests" "nosummary|wrote no summary" \
  "exit1pass|scripts/test (under lockrun) exited 1" "lie_timedout|timed out (timed_out=True)" \
  "nosteps|an empty test selection never passes" "badlist|list_sha256 is not the hash" \
  "badlog|differs from the summary's" "onlybuild|only build steps"; do
  mode=${spec%%|*} want=${spec#*|}
  C=$(mkc "$M" fake_mode="$mode")
  eval "C_$mode=$C"
  tc "$C"
  check "$mode: exit 1, fail receipt saying \"$want\", worktree removed" \
    eval 'rc 1 && verdict_is $C fail && problem_has $C "$want" && cleaned'
done

C=$(mkc "$M" fake_mode=withbuild)
tc "$C"
check "a build step (kind build, no tests) before a step with tests: pass" eval 'rc 0 && verdict_is $C pass'

C=$(mkc "$M" fake_mode=timeout)
tc "$C" --timeout 2
check "timeout: unfinished receipt (exit 3), never a pass, worktree removed" \
  eval 'rc 3 && verdict_is $C unfinished && problem_has $C "timed out after 2 s" && cleaned'

C=$(mkc "$M" fake_mode=slow)
rm -f "$T/started"
(cd "$DEV" && TMPDIR=$CT FAKE_TEST_STARTED=$T/started exec "$S/test_candidate" "$C" >"$T/sig.out" 2>&1) &
pid=$!
for _ in $(seq 100); do [ -f "$T/started" ] && break; sleep 0.1; done
kill -TERM "$pid"; wait "$pid"; RC=$?; OUT=$(cat "$T/sig.out")
check "SIGTERM mid-test: unfinished receipt, test stopped, worktree removed" \
  eval 'rc 3 && verdict_is $C unfinished && problem_has $C "cancelled (SIGTERM)" && cleaned'

tc main
check "refuses a branch name (it can move)" eval 'rc 2 && has "give an exact commit id" && cleaned'
tc deadbeefdeadbeef
check "refuses an unknown commit id" eval 'rc 2 && has "no such commit"'
g branch "${P:0:10}" "$M" 2>/dev/null
tc "${P:0:10}"
check "refuses an id that is also a branch name" eval 'rc 2 && has "is also a ref name"'
g branch -D "${P:0:10}" >/dev/null 2>&1

printf '%s\necho stray >>README\n' "$SPARSE_HOOK" | sed '/^exit 0$/d' >"$DEV/.git/hooks/post-checkout"
tc "$P"
check "refuses a checkout that is not clean (a hook edited README); no receipt written; worktree removed" \
  eval 'rc 2 && has "not clean after checkout" && verdict_is $P pass && cleaned'
printf '%s\n' "$SPARSE_HOOK" >"$DEV/.git/hooks/post-checkout"

# ---------------------------------------------------------------- golden_approve
G1=$(mkc "$M" tests/golden_test.sh=changed)
G2=$(mkc "$M" answers/a.txt=v2)
GX=$(mkc "$M" answers/a.txt=v-other)
G3=$(mkc "$M" answers/a.txt="<delete>")
GL=$(mkc "$M" notes/golden_paths.txt=tests/golden_test.sh answers/a.txt=v3)
ga check --candidate "$G1" --base "$M"
check "changed approved test without approval: refused" eval 'rc 1 && has "UNAPPROVED M" && has "tests/golden_test.sh"'
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
check "that approval does cover its own blob" eval 'rc 0 && has "approved   M"'
ga check --candidate "$G3" --base "$M"
check "deletion of a protected file needs approval" eval 'rc 1 && has "UNAPPROVED D deleted"'
ga record --commit "$G3" --approver lead --builder speed --reason "drop it" --paths answers/a.txt
ga check --candidate "$G3" --base "$M"
check "an approved deletion passes" eval 'rc 0'
ga check --candidate "$GL" --base "$M"
check "dropping a path from the list does not unprotect it; the list change itself needs approval" \
  eval 'rc 1 && has "notes/golden_paths.txt" && has "answers/a.txt"'
ga record --commit "$M" --approver lead --builder speed --reason baseline --dry-run
check "default record covers every protected file at the commit (baseline style)" \
  eval 'rc 0 && has "3 path(s)" && has "answers/a.txt" && has "tests/golden_test.sh" && has "notes/golden_paths.txt"'

# ---------------------------------------------------------------- merge_candidate refusals
mc "$G1"
check "merge: missing receipt refused" eval 'rc 1 && has "no test receipt"'
mc "$C_fail"
check "merge: failed-test receipt refused" eval 'rc 1 && has "verdict is '"'"'fail'"'"'"'
Q=$(mkc "$M" extra.txt=from-Q)
g notes --ref=test-receipts copy "$P" "$Q" 2>/dev/null
mc "$Q"
check "merge: a receipt for another commit (proof for another version) refused" eval 'rc 1 && has "receipt is for another commit"'
R=$(mkc "$M" extra.txt=from-R)
forge "$R" "$P" 'r["candidate"]["tree"] = "'"$(g rev-parse "$P^{tree}")"'"'
mc "$R"
check "merge: receipt tree mismatch refused" eval 'rc 1 && one_problem && has "tree"'
P2=$(mkc "$P" extra.txt=branch-moved)
mc "$P2"
check "merge: branch moved after the test (new tip has no receipt) refused" eval 'rc 1 && has "no test receipt"'
mc main
check "merge: a branch name instead of an exact id refused" eval 'rc 2 && has "give an exact commit id"'
L=$(mkc "$M" tests.list=swapped-list)
forge "$L" "$P"
mc "$L"
check "merge: test list in the candidate differs from the tested list: refused" eval 'rc 1 && one_problem && has "test list mismatch"'
tc "$G2"
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
git -C "$PUB" checkout -q --detach "$M" 2>/dev/null
mc "$P"
check "merge: published copy not on branch main refused" eval 'rc 1 && one_problem && has "not on branch main"'
git -C "$PUB" checkout -q main 2>/dev/null
I=$(mkc "$M" build.out=tracked-now)
forge "$I" "$P"
echo mine >"$PUB/build.out"
mc "$I"
check "merge: an ignored file the candidate would overwrite refused" eval 'rc 1 && one_problem && has "ignored files"'
rm "$PUB/build.out"
N=$(mkc "$M0" extra.txt=old-base README="readme v2")
tc "$N" --base-main "$M"
mc "$N"
check "merge: candidate that does not contain main refused (tests passed, is_ancestor false)" \
  eval 'verdict_is $N pass && [ "$(rget $N "r[\"base_main\"][\"is_ancestor\"]")" = False ] && rc 1 && one_problem && has "does not contain the recorded main"'
check "after all refusals: origin main and the published copy unchanged" \
  eval '[ "$(origin_main)" = "$M" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ] && [ -z "$(git -C "$PUB" status --porcelain)" ]'

# ---------------------------------------------------------------- races and recovery
# Main moves to MID, which the candidate PL contains: a plain push would be a
# fast-forward and succeed, so only the lease can stop it.
MID=$(mkc "$M" mid.txt=mid)
g push -q origin "$MID:refs/heads/side2" 2>/dev/null
PL=$(mkc "$MID" extra.txt=from-PL)
tc "$PL" --base-main "$M"
# Between merge_candidate's checks and `git push`: a git wrapper first on PATH
# (test fixture only) moves main just before the push runs. The lease rejects it.
REAL_GIT=$(command -v git)
mkdir -p "$T/bin"
printf '#!/bin/sh\n[ "$3" = push ] && "%s" --git-dir="%s" update-ref refs/heads/main %s\nexec "%s" "$@"\n' \
  "$REAL_GIT" "$ORIGIN" "$MID" "$REAL_GIT" >"$T/bin/git"
chmod +x "$T/bin/git"
OUT=$(cd "$DEV" && PATH=$T/bin:$PATH "$S/merge_candidate" "$PL" --local "$PUB" 2>&1); RC=$?
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
check "local update fails after the remote update: exit 4, recovery steps printed, local files untouched" \
  eval 'rc 4 && has "REMOTE UPDATED, LOCAL COPY NOT UPDATED" && has "git merge --ff-only $P" && [ "$(origin_main)" = "$P" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$M" ] && [ ! -e "$PUB/extra.txt" ]'
rm -f "$PUB/.git/index.lock" "$DEV/.git/hooks/pre-push"
set_origin_main "$M"

# ---------------------------------------------------------------- end to end
mc "$P"
check "end to end: pushed with the lease, published copy fast-forwarded with its files updated" \
  eval 'rc 0 && [ "$(origin_main)" = "$P" ] && [ "$(git -C "$PUB" rev-parse HEAD)" = "$P" ] && [ "$(cat "$PUB/extra.txt")" = from-P ] && [ -z "$(git -C "$PUB" status --porcelain)" ]'
check "end to end: the exact guarded push command was used" has "running: git push --force-with-lease=main:$M origin $P:main"

echo "# $n checks, $((n - fails)) passed, $fails failed"
[ "$fails" = 0 ]
