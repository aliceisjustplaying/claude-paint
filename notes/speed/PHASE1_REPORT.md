# Speed agent, phase 1: lockrun, the release build and the baseline

Branch `codex/speed` in `~/src/a/claude-paint-speed`, from
`af49348239421791509f7b7c36e9dc39305b26fe`. Plan SHA-256
`382fc2debb7a56505f9b4723c6591f22644feccb330fef62dc723b635f098e30`, with the
decisions in `~/src/a/claude-paint-overnight-decisions.md`. Nothing has been
pushed. No commits went to main and no tags were made. Nothing painted and
nothing replayed beyond the six 128 px scenes below.

| commit | what |
|---|---|
| `c362a1f` | `scripts/lockrun` and `scripts/tests/lockrun.sh` (first version) |
| `8b71762` | lockrun fix: the trampoline holds the lock (see "The sccache incident") |
| `a97c3a6` | **the baseline**: `notes/thinner/baseline/` and the state dumper (`crates/*/src/state_dump.rs`, the hook in `main.rs` and `lib.rs`) |
| this report's commit | `notes/speed/PHASE1_REPORT.md` and `notes/speed/logs/` |

## A. scripts/lockrun

`scripts/lockrun --timeout SECONDS [--owner NAME] [--log FILE] [--lock-timeout S] [--grace S] [--quiet] -- CMD...`
and `scripts/lockrun --status`. It is Python 3 standard library only, with
the full design in its docstring. sha256 `cfcc8e51…5185` at `8b71762`.

- **Lock:** `fcntl.flock` on `/tmp/lockrun-<uid>/lock`, one per user
  (`LOCKRUN_DIR` overrides it; the tests use their own). It uses `/tmp`, not
  `$TMPDIR`, because agents set `TMPDIR` differently and must share one
  lock. lockrun never writes the lock file.
- **Record:** `record.json` is written only after the lock is held,
  atomically (temporary file, fsync, rename). It holds the owner, command,
  process group, pid, helper pid, a uuid4 token, start time, timeout and
  log. A job waiting for the lock never touches it. `history.jsonl` gets
  start, finish and abandoned events.
- **Permission pipe:** the command starts through a trampoline (the group
  leader, `start_new_session`) that blocks on a pipe. The helper writes the
  record, then sends the go byte. If the helper dies first, the pipe gives
  end of file and the trampoline exits 125 without running the command.
- **Crash safety:** the trampoline inherits the locked descriptor and runs
  the command as its child with every other descriptor closed. If the helper
  is killed, the lock stays held until the command exits. As a second guard,
  a lockrun that gets the lock still waits while the previous record says
  `running` and its process group has live, non-zombie members (`ps`).
- **Stopping:** `--timeout` sends SIGTERM to the group, then SIGKILL after
  `--grace` (default 5 s). It keeps the log, marks the record `timeout`,
  prints `UNFINISHED (timeout)` and exits 124. SIGINT, SIGTERM or SIGHUP to
  the helper does the same with `cancelled` and exit 130. When the command
  exits on its own, leftovers in its group are stopped too. The exited
  leader stays unreaped meanwhile (`waitid(WNOWAIT)`), so its group id can't
  be reused. lockrun signals only that group.
- **Nesting:** the job's environment gets `LOCKRUN_TOKEN`. An inner lockrun
  whose token is the running record's (and the lock is held) runs its
  command directly in the outer group, so the outer timeout and
  cancellation still reach it. A stale token waits like anyone else.
- **Exit codes:** the command's own code, 124 timeout, 130 cancelled, 75
  lock not free within `--lock-timeout` (nothing ran), 125 lockrun's error.
- **Output:** stdout and stderr go to the log (default
  `/tmp/lockrun-<uid>/logs/<time>-<token>.log`). The helper copies it to
  its stdout unless `--quiet`.

**Tests:** `scripts/tests/lockrun.sh` (sha256 `8726f1cd…528c`), 17 checks
with dummy jobs (`sleep`, `sh`, Python one-liners) in a private
`LOCKRUN_DIR`. The last run at `8b71762` took 21.3 s, all 17 passed
(`notes/speed/logs/a_lockrun_tests.log`):

1. normal completion: exit code, stdout, log and record
2. the record is on disk before work begins (the job reads its own token, group and owner)
3. timeout: exit 124, the leader and a background child stopped, record `timeout`, log kept
4. a job that ignores SIGTERM is SIGKILLed after the grace period
5. and 6. cancellation by SIGTERM and SIGINT: group stopped, `cancelled`, 130
7. leftovers stopped when the command exits
8. a waiting job leaves the record byte-identical; B starts only after A ends; history in order
9. `--lock-timeout`: exit 75, nothing ran, record untouched
10. helper SIGKILLed with its job alive: a second job is refused because the flock itself is held
11. helper and trampoline both SIGKILLed, command alive: the group guard refuses a second job
12. a daemon that leaves the job's group (setsid) doesn't hold the lock and isn't stopped
13. helper SIGKILLed before permission: the job exits without working; the lock frees
14. nesting: inner calls run at once; an inner `--timeout 1` gives 124 inside the outer job
15. a stale `LOCKRUN_TOKEN` waits like anyone else
16. usage: `--timeout` required; `--status`
17. an unrelated `sleep 300` outside lockrun survives every case

**Mutation checks** (run by hand, then reverted): if the trampoline
doesn't hold the lock, check 10 fails. If the command inherits the lock,
check 11 fails, and by hand a setsid daemon then blocks the next job (exit
75). Without the group guard, check 11 fails. Writing the record after the
go byte fails check 2.

### The sccache incident

The first version (`c362a1f`) let the job inherit the locked descriptor.
The release build in B started sccache's server (`~/.cargo/config.toml`
sets `rustc-wrapper = "sccache"`). The server detaches with setsid, so it
had left the process group, but it kept the lock. The next lockrun waited
on a build that had finished; `lsof` showed `sccache 32336` holding
`/private/tmp/lockrun-501/lock`. I stopped the server
(`sccache --stop-server`; clients restart it on demand) and changed the
design so the command never gets the descriptor (`8b71762`). Any long-lived
daemon a build starts would have done the same.

### Limits

- Tested on macOS only (Python 3.13.15). I did not run it on Linux (the
  VPS). It uses only `fcntl.flock`, `os.waitid(WNOWAIT)`, `killpg` and
  `ps -A -o pid=,pgid=,stat=`, which Linux also provides, but that is
  untested.
- A process that leaves the job's group (setsid or setpgid) is neither
  waited for nor stopped. That covers daemons, and also a job that
  deliberately escapes.
- An inner (nested) `--timeout` signals only the inner command's own
  process, not its children. The outer job's timeout covers the rest.
- `LOCKRUN_TEST_HOLD_BEFORE_GO` is a test hook that delays the go byte. It
  is the only way to test "helper dies before permission" deterministically.
  It is read from the helper's environment and removed from the job's.
- lockrun is not yet approved by the lead. Until it is,
  `~/src/a/claude-paint-heavy-turn.md` still governs. I used it for every
  heavy command in B and C anyway.

## B. Release build of unchanged af49348

`/usr/bin/time -p scripts/lockrun --timeout 600 --owner speed:initial-release-build --log notes/speed/logs/b_release_build.log -- cargo build --release -p easel`
ran with no prior `target/`. It compiled 55 crates and finished in
**43.85 s** (cargo), 44.04 s wall. sccache may have served cached
dependencies; its stats were reset when I stopped the server, so that is
unverified. The binary is `target/release/easel`, sha256
`4201cec1f9ddcc8bcd48106ab5b36a3e337bb9619f138f384eb0e81b1ae040f6`, kept
as `$TMPDIR/cp-speed/easel-af49348` for the baseline. The crates were
af49348's: the lockrun commits touch only `scripts/`.

The rebuild after adding the dumper (paint and easel only) took 43.11 s,
binary `a5f37758…b009`. `cargo check --release -p easel --no-default-features`
and the same with `--features box-inness` passed with no warnings in 6.4 s.
Logs are in `notes/speed/logs/`, with home and temp paths redacted.

## C. The baseline (`a97c3a6`, `notes/thinner/baseline/`)

The details are in `notes/thinner/baseline/README.md` (format, comparison
commands) and `MANIFEST.md` (commands, toolchain, settings, hashes, times).
`SHA256SUMS` covers all 85 other files; its own sha256 is
`07101461…5d35`. The package is 4.0 MB.

- **Scenes:** six engine-3 logs (`--@ box inness`, `--@ engine 3`) replayed
  at 128×64 px: `stroke`, `body` (one `hand="body"` pass), `overlap`,
  `pickup` (a clean brush through wet paint, then onto clean ground, and
  `b:wipe`), `rag` (a dried control, a dry wipe, a blot, a spirits-damp
  wipe, a refold and a hard wipe over dry paint, a damp mask wipe, a stroke
  over the wiped areas) and `wait` (15 min; fifteen 1-min waits; 7.3 + 7.7
  min; a day; four days). That makes 27 chunks.
- **From the unchanged binary (`ref/`):** PNGs, per-chunk `--state-digest`
  lines and printed output. The six scenes take 0.018 to 0.102 s each.
- **Full state (`state/`):** after every chunk, every canvas state value as
  a named field. That covers color, height, film, wet volume, pigment mix,
  hide, stroke ids, floor, cover, the per-pixel drying state, the clock,
  dirty and tacky boxes, tally, hand slice and drawing. It also has the
  studio's seed, counters, clocks and piles, and every brush's bristles and
  every rag as exact `Debug` text. The format is CPSTATE1: named, typed,
  shaped arrays with a JSON header. A later engine adds fields under new
  names. `tools/state_compare.py` compares all baseline fields bit for bit
  and lists the additions; `--added-zero` requires them to be zero.
- **Comparison command:** `scripts/lockrun --timeout 60 -- notes/thinner/baseline/tools/compare_build.sh <release easel> --added-zero`
- **Proof that the dumper changes nothing:** in one lockrun job (1.8 s) I ran
  the unchanged binary and the dumper binary with `--dump-state`, without
  it, and with `RAYON_NUM_THREADS=1`. All digests (with `secs=` dropped),
  PNG bytes and printed output are equal, and the 1-thread dumps are
  identical byte for byte to the 12-thread ones. Each of the 27 dumps also
  rebuilds the PAINTCK8 checkpoint bytes, whose FNV-1a equals af49348's
  `canvas=` digest (`tools/checkpoint_from_dump.py`). So the dump holds
  every checkpointed value exactly.
- **The comparer's own checks:** `tools/test_state_compare.py` (16 cases,
  0.7 s) catches a one-ulp change, 0.0 against −0.0, a removed field,
  nonzero added solvent (as a field and as a new bristle key), a bristle's
  load, a rag's dampness, the studio clock and a missing chunk. It passes
  added zero fields. `tools/verify_package.sh` runs the hashes, these
  checks and the 27 rebuilds (about 2 s). It passed on the committed tree
  exported with `git archive a97c3a6`.
- **Old-file samples (`old_files/`):** a real PAINTCK8 easel save of
  `stroke`, the same file's first 280 bytes (magic and header) and two
  engine-3 logs written by af49348's live session (no canvas made).
- **Cherry-pick:** `git merge-tree --merge-base 8b71762 thinner2 a97c3a6`
  reports a clean apply onto `thinner2`.

## Odd things

- `thinner2` is at `e39da645092f18d70bd593e1cc82bbcd8de7a281`, not the
  plan's `c692eae`. The thinner agent has moved it; I did not inspect it.
- Several `claude-paint-*` worktrees exist (r21 to r21.5, rag, drying,
  repaints, studio, thinner, brief1). I touched none of them.
- At 128 px the default `body` filbert 9 is about 1 px wide and the rag's
  lifts are faint in the PNG. The scenes catch regressions in the state
  values; they are not pictures to judge paint by.
- The baseline commit contains engine-side code: the dumper, which only
  reads. The thinner branch needs it to make comparable dumps and must add
  its new fields (solvent) under new names.

## Open questions for the lead

1. Approve lockrun, and retire the manual turn file?
2. Should the golden-approval scope (decision 1) also cover
   `crates/paint/src/state_dump.rs` and `crates/easel/src/state_dump.rs`?
   They are the code that measures the baseline. An edit that renames or
   redefines an existing field would weaken the comparison.
3. The thinner's check 2 compares "existing paint-state values". I suggest
   `compare_build.sh <easel> --added-zero` as its command, plus equal PNGs
   (the script reports them). The `canvas=` digest will change with
   PAINTCK9 by itself, so it can't be the criterion. Does this satisfy the
   reviewers?
4. Is the inness box enough? It has raw sienna, lead white, raw umber and
   bone black. No scene uses the default tube box.
