# Safeguards that protect main

Three tools check a version before it reaches main:

- `scripts/test_candidate` tests one exact commit and records a receipt.
- `scripts/golden_approve` records and checks approvals of protected test answers.
- `scripts/merge_candidate` adds a commit to main only if its receipt, approvals and main's position check out.

`scripts/safeguards_lib.py` holds their shared code. `scripts/tests/safeguards.sh` tests them on dummy repositories. Branch `codex/safeguards`, built on `codex/speed` at `af5beb3`. None of these tools has been run on the real repository. No receipt or approval has been recorded there, and main and the website are unchanged.

## Commands for the lead

Notes do not travel with branches. Fetch them before checking and push them after recording:

```sh
git fetch origin refs/notes/test-receipts:refs/notes/test-receipts
git fetch origin refs/notes/golden-approvals:refs/notes/golden-approvals
git push origin refs/notes/test-receipts
git push origin refs/notes/golden-approvals
```

Both pushes go through the normal privacy hooks. A fetch fails if the remote doesn't have the ref yet.

Record the existing baseline approval (the 88 protected files at `a97c3a6`, built by the speed agent). Run this only as the lead or the user:

```sh
cd ~/src/a/claude-paint-safeguards
scripts/golden_approve record \
  --commit a97c3a659f7f4c5848be59c611fdcf68e98f6281 \
  --approver lead --builder speed \
  --list-from 870a2d397a5e9319e230b1183299b206a356248c \
  --reason "Baseline answers for the thinner work, computed by the unchanged engine 3 code (af49348); first small comparison verified by the lead" \
  --method "Option B: unchanged af49348 release build plus the read-only state dumper; commands, toolchain, binary and output hashes in notes/thinner/baseline/MANIFEST.md and SHA256SUMS at a97c3a6"
git push origin refs/notes/golden-approvals
```

`--list-from` names the commit whose `notes/golden_paths.txt` defines the protected set (`a97c3a6` has no list yet). A dry run (`--dry-run`) on the real repository listed 88 paths: everything under `notes/thinner/baseline/` plus both `state_dump.rs` files. A dry run records nothing.

The list file itself also needs approval, because it is new relative to main. Approve it once the thinner agent has named its paths in it:

```sh
scripts/golden_approve record --commit <commit with the final list> --approver lead \
  --builder <who edited it> --reason "..." --paths notes/golden_paths.txt
```

Today `golden_approve check --candidate 9d6de58… --base 3379b9f…` reports 89 unapproved protected changes. The baseline approval above clears 88 of them. The list approval clears the 89th.

Test a candidate, then merge it. Merging is not for tonight: test it only on dummy repositories.

```sh
scripts/test_candidate <full commit id> [--base-main <main commit id>]
scripts/golden_approve check --candidate <id> --base <main id>
scripts/merge_candidate <id> --local ~/src/a/claude-paint
git push origin refs/notes/test-receipts
```

`scripts/test --all --candidate <id>` will exec `scripts/test_candidate <id>`. The speed agent is wiring that.

## scripts/test_candidate

`scripts/test_candidate COMMIT [--base-main COMMIT] [--remote origin] [--timeout 600] [--lock-timeout S] [--list-file PATH]`

1. COMMIT must be a commit id of 7 to 64 hex digits. Branch names, `HEAD`, tags, unknown ids and ambiguous prefixes are refused, as is an id that is also a ref name. A name can move between testing and merging. The receipt records the full id and the tree hash.
2. The base main is `--base-main`, else `refs/heads/main`, else `refs/remotes/origin/main`. The receipt records which source it used and whether that commit is an ancestor of the candidate. In the real repository every worktree shares one repository, so `refs/heads/main` is the published working copy's branch.
3. It runs `git worktree add --detach` into a new directory under `$TMPDIR`. The repository's post-checkout hook makes that worktree sparse, so `git sparse-checkout disable` follows. Then the checkout is verified: HEAD and tree are the commit's, no entry is skip-worktree or assume-unchanged, `git ls-files` counts as many files as `git ls-tree -r COMMIT`, and `git status --porcelain --untracked-files=all --ignored` is empty. Otherwise it refuses (exit 2) and writes no receipt. A full checkout includes the notes images, about 610 MB according to the hook's comment. They are deleted with the worktree.
4. One outer `lockrun --timeout 600` (`--timeout` changes it) runs `scripts/test --all --summary <file outside the worktree>` in the worktree, with `CARGO_TARGET_DIR=<worktree>/target`. The lockrun used is `$LOCKRUN`, else the candidate's own `scripts/lockrun`, else `~/src/a/claude-paint-tools/lockrun`. Every candidate builds from an empty target directory, which sccache may speed up. 600 s has to cover that build plus the tests.
5. A pass needs all of the following:
   - exit 0;
   - a summary that parses, with `mode` "all", `verdict` "pass", a 64-hex `list_sha256` and at least one step;
   - for every step: `exit` 0, `timed_out` false, `tests_run` ≥ 1, `passed` ≥ 1 and `failed` 0;
   - every step's `log` exists and hashes to its `log_sha256`;
   - the test list file in the commit hashes to the summary's `list_sha256`.

   The list file is the summary's `list` field (a repository-relative path), else `--list-file`, else `$TEST_LIST_FILE`. With none of these the verdict is fail. A timeout (lockrun 124), a cancellation (130, or SIGINT/SIGTERM/SIGHUP to test_candidate) or a busy lock (75, with `--lock-timeout`) is `unfinished`. Everything else is `fail`.
6. It copies the logs out of the worktree first. Then it always runs `git worktree remove --force --force`, `rm -rf` of the temporary directory and `git worktree prune`, after a pass, a failure, a timeout or a signal. Signals are ignored during cleanup. A cleanup error is printed and stored in the receipt.
7. Exit codes: 0 pass, 1 fail, 3 unfinished, 2 refused before testing.

### Receipt format

The receipt is written by `git notes --ref=test-receipts add -f` on the candidate commit, so it lives outside the commit. A newer run's note replaces the older one. Every run's files stay in `$TEST_RECEIPT_DIR/<commit>/<time>-<pid>/` (default `~/src/a/claude-paint-receipts`): `receipt.json`, `summary.json`, `lockrun.log` and `logs/NN-<step>.log`.

```text
format            "claude-paint test receipt v1"
verdict           "pass" | "fail" | "unfinished";  problems: [reasons]
candidate         {commit, tree, requested (the argument)}
base_main         {commit, source, is_ancestor}
exit, timeout_s, started_at, ended_at, seconds
tool              {name, sha256 over test_candidate + safeguards_lib.py}
lockrun           {path (which one), sha256, command}
checkout          {full, files, clean_before, head_after, tracked_changes_after}
cleanup_errors    []
build             {rustc, cargo, cflags (from [env] in the candidate's .cargo/config.toml),
                   cflags_env_override, rustflags_env, rustc_wrapper, rustc_wrapper_source,
                   configs [{path, sha256}] (candidate and ~/.cargo), profiles_declared,
                   profiles_used (from the steps' cargo commands), cargo_target_dir}
test_list         {file, sha256 (of the file in the commit), summary_list_sha256}
summary           the summary JSON, copied;  summary_sha256
logs              [{step, file, sha256, summary_log_sha256}];  lockrun_log_sha256
receipt_dir       local path, written with ~
```

Receipts are pushed to a public repository. The home directory is written as `~`, and a receipt containing text the privacy hooks protect is not recorded. Only log hashes go into the note. The logs themselves stay on the machine.

## scripts/golden_approve

The protected set is listed in `notes/golden_paths.txt`. Each line is a path, or a directory prefix ending in `/`. `#` starts a comment. The list file itself is always protected. The initial list: `notes/thinner/baseline/`, `crates/paint/src/state_dump.rs`, `crates/easel/src/state_dump.rs` and `notes/golden_paths.txt`, plus a placeholder comment for the thinner acceptance tests, helpers and answers.

`record --commit X --approver ROLE --builder ROLE --reason TEXT [--method TEXT] [--paths P...] [--list-from C] [--dry-run]`
appends one JSON line to the note on X in `refs/notes/golden-approvals`:

```text
{"format": "claude-paint golden approval v1", "commit": X, "approver", "builder",
 "reason", "method", "approved_at",
 "paths": [{"path", "blob": "<blob id at X>" | "deleted"}],
 "list_from": {"commit", "sha256"}, "tool_sha256"}
```

By default it approves every protected file at X. `--paths` takes files or `dir/` prefixes, and a path absent at X approves its deletion. It refuses when approver equals builder, ignoring case. It uses `git notes append`, so several approvals can share one commit.

`check --candidate C --base B` checks every protected path that differs between B and C. Added, modified and deleted paths all count, and a rename counts as a deletion plus an addition. The protected set is the union of B's and C's lists, so dropping a path from the list does not unprotect it. Each change needs its exact blob at C, or "deleted", in a valid approval. A valid approval sits on the commit it names, has approver ≠ builder, and each blob it lists really is that path's blob at that commit. It prints approved and UNAPPROVED changes and exits 1 if any change is unapproved. `show` prints every approval.

## scripts/merge_candidate

`scripts/merge_candidate COMMIT --local PUBLISHED_COPY [--remote origin] [--branch main]`

Run it from a working copy of the repository. Before it changes anything, it lists every failed check:

1. A receipt exists on COMMIT. It names exactly this commit and tree, with verdict pass and exit 0. Its summary passes the same rules as above.
2. The receipt's test list file in COMMIT hashes to the receipt's `test_list.sha256` and to the summary's `list_sha256`, so the list was not swapped.
3. `golden_approve check` passes between the receipt's base main and COMMIT.
4. The base main is in COMMIT's history.
5. `git ls-remote` shows the remote's main at the recorded base main.
6. The published copy:
   - its HEAD is the recorded base main and it is on branch main;
   - its remote URL is the same;
   - no merge, rebase, cherry-pick or revert is in progress;
   - `git status --porcelain --untracked-files=all` is empty;
   - no ignored file sits at a path COMMIT adds (git would overwrite it silently).

Then it runs exactly `git push --force-with-lease=main:<recorded main> origin <COMMIT>:main`. Next it re-checks the published copy, then runs `git -C <local> fetch origin main` and `git -C <local> merge --ff-only <COMMIT>`. Last it verifies that the published copy's HEAD is COMMIT and its status is clean. Hooks stay on; it never uses `--no-verify`.

Exit codes:

- 0: merged and the published copy updated.
- 1: refused, nothing changed.
- 2: usage error, for example a name instead of an exact id.
- 3: the push failed. If main moved, the message says so and the remote does not have COMMIT.
- 4: the remote was updated but the published copy was not. It prints the remote's new and old commits, says nothing was overwritten, and gives the manual steps: `git status`, `git fetch`, `git merge --ff-only <COMMIT>`, no `reset --hard`. Undoing the remote update is a separate, deliberate choice.

## Tests

```sh
scripts/tests/safeguards.sh
```

It builds a bare origin, a published clone and a dev clone under `$TMPDIR`. The dev clone has the same sparse post-checkout hook as the real repository, and its working copy stays dirty throughout. The candidate commits are made with plumbing. They carry a fake `scripts/test` that follows the summary contract and a copy of `scripts/lockrun`, which runs with an isolated `LOCKRUN_DIR`. Every run passes `TMPDIR` to test_candidate and then checks that the directory is empty and `git worktree list` shows only the dev clone. The fake test fails unless it runs under lockrun, with its target directory inside the worktree, with `notes/pic.png` present (a full checkout) and without the dev clone's changes. Receipts are "forged" (copied and edited) only to test merge_candidate's refusals. Every pass comes from a real test_candidate run.

Result on 2026-10-04 (macOS, Python 3.13): `# 51 checks, 51 passed, 0 failed`, exit 0, 53 to 56 s wall over the last two runs. The hooks run on every Git command, and that accounts for most of the time.

The 51 checks:

- **test_candidate pass path (6):**
  - the receipt has the exact commit, tree, base and ancestry;
  - the log was copied out with a matching hash;
  - the receipt has build settings, the list hash and the tool hash;
  - the worktree was removed;
  - the sparse hook ran and the checkout was made full;
  - the dirty dev clone did not leak in.
- **test_candidate failures (8), each a fail receipt with the specific reason and the worktree removed:** a failing step, zero tests, a missing summary, exit 1 with a pass summary, a step marked timed out under a pass verdict, no steps, a wrong list hash and a wrong log hash.
- **unfinished:** a timeout and SIGTERM mid-test (both unfinished, worktree removed).
- **refusals:** a branch name, an unknown id, an id that is also a branch name, and an unclean checkout (a hook edits README). The unclean checkout writes no receipt, so the earlier pass stays, and the worktree is removed.
- **golden_approve (10):**
  - changing an approved test is refused, and so is changing an answer;
  - self-approval is refused, ignoring case, and records nothing;
  - a hand-written self-approval note is ignored;
  - an approval for a different blob is refused, and that approval does cover its own blob;
  - deleting a protected file needs approval, and an approved deletion passes;
  - dropping a path from the list doesn't unprotect it;
  - the default record covers every protected file.
- **merge_candidate refusals (16):**
  - missing receipt;
  - failed receipt;
  - a receipt for another commit;
  - a tree mismatch;
  - the branch moved after the test;
  - a branch name instead of an id;
  - a swapped test list;
  - an unapproved answer change with passing tests;
  - remote main moved;
  - published main moved;
  - published copy with a modified file;
  - published copy with an untracked file;
  - published copy not on main;
  - an ignored file the candidate would overwrite;
  - a candidate that doesn't contain main;
  - afterward, origin and the published copy are unchanged.
- **races (3):**
  - main moves after the checks and before the push, to a commit the candidate contains. A plain push would fast-forward, and the lease rejects it as "stale info". A `git` wrapper put first on `PATH` by the test moves main; the tool has no test hook.
  - main moves during the push (a pre-push hook). The server's compare-and-swap rejects it.
  - the local update fails after the remote update (a pre-push hook leaves `index.lock` in the published copy). The result is exit 4 with recovery steps, and the files are untouched.
- **end to end (2):** the push used the exact guarded command, and the published copy was fast-forwarded with its files updated and its status clean.

Mutation runs: I removed one guard at a time from a copy of the tools and re-ran the suite. Each time the intended checks failed:

- without the lease: check 47. Checks 48 to 51 also failed as a knock-on effect, because that push then succeeded and moved the published copy.
- without the sparse fix: 31 checks, starting with every pass.
- accepting self-approved notes: checks 24, 25 and 38. Check 39 also failed as a knock-on effect, because the G2 merge then went through.
- accepting zero tests: check 8.

## Known limits

- **One Git identity.** Every agent commits and writes notes as the same Git user. "Approver" and "builder" are declared role names, and approver ≠ builder is a procedural check. The rule is that only the lead or the user runs `golden_approve record` and `merge_candidate`. Nothing stops an agent from writing a note by hand.
- **Receipts are not signed.** merge_candidate trusts the receipt note's content. The tests forge receipts to show it checks consistency (commit, tree, list), not authenticity. The receipt directory's logs and hashes let a reviewer cross-check a receipt. Re-running test_candidate is the real proof.
- **Notes must be fetched and pushed explicitly** (see the commands above). An approval or receipt recorded on another machine is invisible until fetched. merge_candidate reads the local notes only.
- **The list hash needs scripts/test's cooperation.** test_candidate can check the list only if the summary names its list file in `list`, or if `--list-file` or `TEST_LIST_FILE` is given. Otherwise every run fails. Whether every reviewed check actually ran is scripts/test's job: test_candidate checks only the steps the summary reports.
- **Protected-set gaps.** The dumper's wiring lines in `crates/easel/src/main.rs` and `crates/paint/src/lib.rs` are not protected; listing those whole files would be too broad. The thinner paths are still a placeholder. An approval covers an exact blob anywhere and never expires.
- **Latest receipt wins.** A later run's note replaces the earlier one, so the latest result counts. Older receipts survive only in the local receipt directory.
- **Tests that edit tracked files** are recorded (`checkout.tracked_changes_after`) but do not fail the run.
- **SIGINT from a terminal was not tested.** Background jobs in a non-interactive shell ignore SIGINT, so the suite tests SIGTERM. SIGINT takes the same cleanup path, and lockrun stops the job's process group itself.
- **Not yet run for real.** Nothing has been run against the speed agent's real `scripts/test`, which does not exist yet, or with a Rust build.
