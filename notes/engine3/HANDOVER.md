# Engine 3: handover, October 4, 2026

For the agent who picks this up. It covers where the code is, what the owner has
decided, how to consolidate everything on main, and what to build after that.
Nothing here has been merged yet. Every number below comes from a run or a file
named beside it.

## 1. What the owner wants

1. One folder with the code (`~/src/a/claude-paint`), no worktrees.
2. Everything that is needed merged on main, work in progress included.
3. The stale worktrees removed.
4. Main in a state where the rag and thinner work can start.
5. Then the coding, ending in the first full engine-3 painting. None exists yet
   (the owner confirmed it; a search of the worktrees found only test fixtures
   and demo scripts marked `--@ engine 3`).

The owner asked for this to be done carefully.

## Status, October 4, later

The consolidation below is done: `engine3-consolidation` merged `fix/painting-clock` and `engine3-overnight`, took the sienna measurement and `notes/look/`, and imported `description/`; main was fast-forwarded to it. Sections 2 to 5 describe the state before that. As expected, `scripts/test --all` is red only on 13 (b) and the `rag` scene of `baseline-state` and thinner check 2. The rag scene is not listed as a known failure, so the verdict is a plain FAIL (exit 1), not NOT ALL GREEN; the owner chose to leave it red until the rag work (6.1).

## 2. State at handover

Created today by the previous agent, all local, nothing pushed:

| What | Where |
|---|---|
| This branch and worktree | `engine3-consolidation` in `~/src/a/claude-paint-consolidate` = main (`4e525e5`) + the backup commit + this file |
| Backup of the untracked rag review material | branch `backup/rag-cloth-untracked-2026-10-04` (`e788e4d`), 161 files, each verified against the working file |
| The reviewer packet and both external reviews (zips) | `~/src/a/claude-paint-reviews/engine3-rag-review-2026-10-04/` |
| Probe patches and logs behind the numbers in this file | `out/reviews/engine3-review-check/` (in this branch) |

A trial merge of `engine3-overnight` into this branch was started and aborted.
The worktree is clean.

The untracked copies of `notes/rag/*-review/`, `notes/rag/reviewer-package/` and
`out/` are still in `~/src/a/claude-paint`. They are identical to the backup
commit except for the 88 MB packet zip, which is not in git.

## 3. Decisions already made (do not ask again)

| Decision | Source |
|---|---|
| Keep Rust. No Python rewrite. | Both external reviews |
| Consolidate on main; WIP on main is acceptable. | Owner |
| **Check 13 (b)**: restate it as a contrast ratio (burnt sienna's luminance contrast ratio, over black divided by over white, lower than raw sienna's at equal film on the same substrates). Pigment values unchanged. The old "difference kept" measure stays as a diagnostic. | Owner delegated it; decided by the previous agent |
| **Thinned deposition** follows this rule: "a pass over wet wash mostly redistributes, and a pass over dry paint stacks." | Owner |
| Look variant (b) is not adopted. | Both reviews |
| The retired-face ledger (R07) and overlap work (R08) are deferred. | Both reviews |

Neither decision is implemented. Both change protected tests (section 7).

Why the ratio for 13 (b): Field calls burnt sienna "richer, deeper, and more
transparent than the raw earth" (§155, gutenberg.org/files/20915). A darker
paint always loses the current measure, so the sentence cannot mean it. On the
card's own bands the ratio ranks burnt lower at every thickness (0.166 against
0.182 at 10 µm; `out/reviews/engine3-review-check/look_probe.log`). The margin
at one coat is narrow: 0.438 against 0.452.

## 4. The branches

There are two lines and one set of loose patches.

**Line A, main** (`4e525e5`, pushed, same as `origin/rag-release`). The fork
point `3379b9f` plus four commits:

- `02975ee`: the rag. The same patch as `b2be109` on line B (identical
  `git patch-id`).
- `b612503`: the textured rag, ported from `look-experiments` without solvent
  handling, plus the test tooling copied from the speed work.
- `d18168f`: a rag bounds fix (4 lines in `rag_wipe`) and docs. Main only.
- `4e525e5`: docs and a test count.

On main, engine 3 means engine 2 plus the rag (`crates/paint/src/lib.rs:74-76`).
Main's head has a passing gate receipt
(`git notes --ref=test-receipts show 4e525e5`).

**Line B, the engine-3 line.** One chain; each branch contains the one before
it (checked with `git merge-base --is-ancestor`):

`drying`, `rag`, `repaints` → `round-24` (`af49348`) → `codex/safeguards` →
`codex/safeguards2` → `codex/safeguards3` → `codex/speed` (`2b85ff6`) →
`codex/gate-next` (`eafb6d0`), with `thinner2` (`995fbab`) merged →
`engine3-overnight` (`d6318d1`, pushed) → `look-experiments` (`196a673`,
**on no remote**).

On line B, engine 3 means slower drying, new tube drying rates, rollback
without a rebuild, the rag, the thinner and the faster tests
(`round-24:crates/paint/src/lib.rs:74-86`).

`look-experiments` adds 11 commits to `engine3-overnight`:

- `1efff68`, `957d87e`: the sienna measurement (`crates/paint/tests/look_sienna.rs`).
- `b194901` (a), `bd5addc` (b), `abb5f75` (B), `0ce3dc4` (revert of B): thinned
  ceiling experiments. The branch ends with (b) in place.
  - (a): a thinned load ploughs less.
  - (b): a stroke leaves a pixel holding at most its ceiling more than it held
    when the stroke reached it. Passes stack.
  - (B): a pixel holds at most the larger of the ceiling and what it held
    before. Wet passes do not stack.
- `d6bbd5e`, `8add36d`, `55b1ec0`, `41680fe`: the textured rag, solvent-aware.
- `196a673`: `notes/look/REPORT.md`.

**The cloth prototype.** Patches against main `4e525e5`, never committed as
code: `notes/rag/{cloth,grip,material}-review/prototype.patch` and
`notes/rag/contact-review/research.patch`. They are alternatives, not a stack.
`material-review/prototype.patch` is the most developed.

**Everything else:**

- 42 local branches are already merged into main.
- `thinner` / `thinner-v1` (`94712a4`): the abandoned first thinner. Two commits
  not on line B, plus `stash@{0}`.
- `feat/records-*`, `fix/N11`, `fix/N12`, `fix/N14`, `feat/r21-pause-between`:
  September 30 records work, unrelated to engine 3, on no remote.
- Four stashes (`git stash list`).
- **`fix/painting-clock` (`2a705f4`) is another agent's work**, committed at
  14:59 today from a temp worktree that was gone twenty minutes later. It is one
  commit ahead of main ("report painting time without explicit Lua prints"),
  on no remote and not merged. Ask the owner about it before merging or
  deleting it. Main may move under you.

## 5. Consolidation

Work in `~/src/a/claude-paint-consolidate` on `engine3-consolidation`. Do not
touch `~/src/a/claude-paint` until section 5.6.

### 5.0 Before any build

- **One heavy job at a time.** Every build or test run goes through
  `~/src/a/claude-paint-tools/lockrun`. `scripts/test` takes the lock itself.
- **sccache trap.** `~/.cargo/config.toml` sets `rustc-wrapper = "sccache"`.
  `scripts/test` gives each step its own `TMPDIR` under `/tmp/cpt.*` and deletes
  it. If the sccache server starts inside a step, it keeps that deleted `TMPDIR`
  and every later compile on the machine fails with
  `sccache: error: Failed to create temp dir`. This happened today. Start the
  server first from a normal shell (`sccache --start-server`), and if the error
  appears run `sccache --stop-server` and start it again.
- **Starting health is not re-measured.** Today's run of `scripts/test` on main
  and on `engine3-overnight` was invalidated by the trap above: the steps that
  compiled first passed (`paint` 176 tests, `old-logs` 11, `test-runner` 17,
  `safeguards` 140, `lockrun` 17) and the rest failed on the sccache error. The
  last valid records are main's passing receipt and `engine3-overnight`'s
  `notes/speed/FINAL_REPORT.md` (not all green, only because of 13 (b)).
  Rerun both before merging.
- **The repository is public.** Never publish the owner's real name or a home
  path that contains it; write paths as `~/...`. Do not push. The owner pushes.
- Read `notes/agent_brief_template.md`, `notes/speed/SAFEGUARDS.md` and
  `notes/thinner/ACCEPTANCE.md` on `engine3-overnight` first.

### 5.1 Merge line B

`git merge --no-ff engine3-overnight`

It conflicts in 11 files, one block each (verified with `git merge-tree`).
Most are add/add: main's side is a cut-down copy of line B's file.

| File | Resolution |
|---|---|
| `crates/paint/src/lib.rs` | Take line B's `ENGINE` note (the full engine-3 definition). |
| `crates/paint/src/rag.rs` | **Do not hand-merge.** Start from `look-experiments:crates/paint/src/rag.rs`, which is main's textured rag with solvent handling. Then re-apply `d18168f`'s bounds change in `rag_wipe` and main's test `repeated_wipes_conserve_paint_and_a_second_damp_wipe_clears_more`. Check with `git diff main -- crates/paint/src/rag.rs`: what remains should be solvent handling and the helpers `aged_until_set` and `any_open`. |
| `crates/easel/src/draw_rag.rs` | The sides differ in one comment line. Take either. |
| `crates/easel/tests/determinism.rs` | Take line B's block (480 px test plus the live-width one). |
| `notes/golden_paths.txt` | Take line B's (a superset); confirm every line of main's is present. |
| `notes/speed/test_lists/all.tsv`, `fast.tsv` | Take line B's lists. Main's run each test binary as its own step; line B's run one `cargo-test` step with a minimum of 270 tests. Raise that minimum for tests main added. |
| `notes/easel_guide.md`, `notes/rag/README.md`, `notes/speed/SKIPPED.md` | Docs. Keep both sides' content. |
| `scripts/safeguards_lib.py` | Same content, different mode (main 100755, line B 100644). Keep main's mode so there is no protected difference against main. |

`session.rs`, `api.rs`, `main.rs` and `tally.rs` changed on both sides and
merge without conflict. That merge is unverified: build and test.

### 5.2 Bring the rest of `look-experiments`

- Cherry-pick `1efff68` and `957d87e` (the sienna measurement). The 13 (b)
  restatement uses it.
- Copy `notes/look/` and `crates/paint/tests/look_rim.rs` for the record.
- Do **not** bring (b) (`bd5addc`). Keep `look-experiments` as a branch: `abb5f75`
  is the reference for (B), the variant closest to the owner's rule for wet
  passes.
- `notes/look/REPORT.md`'s thinned sienna card is stale: it records 1.64 µm and
  the branch's final code lays 5.36 µm (`look_probe.log`).

### 5.3 The cloth

The patches and their notes are already in this branch as files. Porting the
cloth onto the merged rag is rag work (section 6), not consolidation:

- `material-review/prototype.patch` applies cleanly to main `4e525e5`.
- It fails 4 of 19 `rag.rs` hunks on `look-experiments` and 10 on
  `engine3-overnight`.
- The cloth path has no solvent handling at all; the merged rag does.

### 5.4 What will be red

- **13 (b)**: by design until it is restated.
- **`baseline-state` and thinner check 2, `rag` scene.** The saved baseline is
  af49348's rag; the textured rag differs from chunk 4 on
  (`notes/look/REPORT.md`, "The baseline's rag scene now differs"). The other
  five scenes should stay equal.
- Anything else red is a merge problem. Find it before going on.

### 5.5 Protected files and approvals

`notes/golden_paths.txt` lists the protected set. A change to it needs an
approval recorded with `scripts/golden_approve record` by someone other than
the builder. **Only the owner or the lead records approvals. Never record one
yourself.** `scripts/golden_approve check --candidate <id> --base <id>` shows
what is unapproved; today it passes for `engine3-overnight` against main.

The consolidation will need approvals for at least: the merged test lists, the
merged `golden_paths.txt`, the 13 (b) restatement and whatever is done about
the baseline's `rag` scene.

### 5.6 Moving main

The gate (`~/src/a/claude-paint-tools/gate/`: `test_candidate`,
`golden_approve check`, `merge_candidate`) adds a commit to main only on a
passing receipt with approvals. It will refuse a candidate with a red check.

The owner accepts WIP on main. Whether main moves through the gate once green
or by a direct fast-forward is the owner's call. **Ask in one line before
moving main outside the gate.**

Before the fast-forward, remove the untracked `notes/rag/*-review/` copies from
`~/src/a/claude-paint` (they would block the checkout); confirm first that they
still match this branch. Move the packet zip out of the repo folder; a copy is
already in `~/src/a/claude-paint-reviews/`.

Not verified: whether anything serves the website from the working files in
`~/src/a/claude-paint`. `notes/agent_brief_template.md` calls that folder "main
and the live site".

### 5.7 Cleanup, after main holds everything

| Worktree | State today | Note |
|---|---|---|
| `~/src/a/claude-paint-engine3`, `-speed`, `-thinner2`, `-r24` | clean | Their branches are on line B. |
| `~/src/a/claude-paint-look` | 1 untracked file (`notes/look/index.html`) | Save it first. Branch is on no remote. |
| `~/src/a/claude-paint-brief1` | 1 changed or untracked file | Look before removing. `brief1` is merged into main. |
| `~/src/a/claude-paint-r23` | clean, detached at `7b80cb0` | Contained in main. |
| a temp worktree under `$TMPDIR/pr4-revert.*` | clean, detached at `4e525e5` | Not the previous agent's. |
| `~/src/a/claude-paint-consolidate` | this one | Remove last. |

Use `git worktree remove`. Delete branches only with `git branch -d`, which
refuses unmerged ones. Ask the owner before dropping the stashes, `thinner-v1`
or the September 30 branches, and before pushing `look-experiments` or the
backup branch.

## 6. The coding, after consolidation

Two external reviews and the previous agent's checks agree on this order.
The reviews are `REVIEW.md`, `AGENT_BRIEF.md`, `REVIEW_RESPONSE.md` and
`AGENT_BRIEF_V2.md`, unpacked in `notes/engine3/reviews/` (the zips are in
`~/src/a/claude-paint-reviews/engine3-rag-review-2026-10-04/`).

### 6.1 Rag

The owner rejected the cloth prototype's look twice. Measured causes:

1. **The stain floor decides thin wipes.** `STAIN_COATS` leaves 1 to 2 µm
   whatever the cloth does.
   - Films of 1 µm or less lose 0.0% to a wipe. The damp wipe sits exactly on
     the floor's bound: 33.2%, 66.5% and 90.0% removed at 1.5, 3 and 10 µm
     (`rag_probe3.log`).
   - A brush-laid stroke is 1.51 µm at thinner 0.5 and 0.53 µm at 2/3, so two
     damp wipes remove 33.9% and 0.0% (`thinner_probe3.log`).
   - With the floor off, a damp wipe removes 100.0% and a dry wipe 78% at every
     thickness. Neither state has cloth texture.
   - Replacement, from sources: a retained capacity per ground (near zero on a
     cured oil ground, larger on gesso or a young oil ground); paint fills it
     first; solvent on the cloth lifts it; finer pigments retain more. Golden:
     "the white of both grounds could be regained with a little mineral spirits
     or oil on the wiping cloth"
     (justpaint.org/differentiating-between-acrylic-gesso-and-williamsburg-oil-ground).
     No source gives a thickness. `prime` has no absorbency parameter
     (`crates/paint/src/canvas.rs:281`).
2. **Contact never leaves a gap.** No pixel in the wipe's middle has
   accumulated contact under 0.35; the lowest lane is 0.55 (`rag_probe.log`,
   `rag_probe2.log`). Contact is a gap mask over 0.18 mm, not pressure
   (`rag/cloth.rs:186` in the patch). The owner's direction is a deforming
   cloth (`OWNER_FEEDBACK.md` in the packet); the second review proposes
   prescribed contact shapes first. Try both on the same thin film and show
   the owner.
3. **The 69 µm fixture mostly shows the cloth filling up.** At a quarter of
   the prototype's lift rate (`LIFT` 1 instead of 4) the film left runs from 2.3 µm at the start to 20.2 µm at the end. Only 17%
   of it came off the cloth (tracer, `rag_probe3.log`). Pickup is throttled
   twice, by whole-face load and by cell; removing the whole-face factor drops
   the film left from 8.62 to 6.54 µm. Use thin films (0.5 to 10 µm) as the
   main fixtures and keep 69 µm as a stress case.
4. **Cells overfill.** A damp wipe at the prototype's rate leaves 100 of 361
   cells over capacity, the worst at 1.26 times (`rag_probe.log`). Enforce receiver capacity inside the transfer.
5. **Main's rag depends on vertex count.** The same dry line leaves 31.2% with
   up to 17 points and 23.8% with 1,761; damp 6.8% and 3.9% (`ship_probe.log`).
   Segments shorter than a quarter pad add stamps, and the per-stamp shares
   compound. A small separate fix: one arclength sampler across vertices.
   In the cloth prototype the same defect moves the film by about 1%.

Small and real, lower priority: blot absorption uses a made-up distance
(`rag/face.rs:54`); the unseen sideways wander is 3.5% of travel.

### 6.2 Thinner

1. **Flow runs only on whole-minute boundaries.** A 0.02 min wait moves no
   paint from clock phases .10, .50 and .97 and 0.97% of all paint from .99.
   End states differ by up to 60% of everything the flow moves
   (`thinner_probe.log`).
2. **Deposition by the owner's rule.** Today's ceiling and (b) both give every
   stroke a fresh allowance. Checks 7, 9, 15 and 16 and the second half of
   check 14 (`crates/paint/tests/thinner_physics.rs:344-353`) assert that
   allowance. Restating them is a protected change.
3. **A single stroke at thinner 0.75 or more never flows.** Its ceiling is at
   or below the 2 µm wetting film (`WET_FILM_UM`): the flow moved 0.0000% of
   the paint, against 16.2% at 0.5 (`thinner_probe2.log`).
4. **The 64-substep cap bites in mixed scenes.** A thinner-0.95 stroke
   elsewhere in the dirty region changes a thinner-0.5 patch's flow by 16.1%
   after one minute (`thinner_probe3.log`).

### 6.3 Sienna

Implement the 13 (b) decision. It touches the test
(`crates/paint/tests/thinner_pigments.rs:107`), `notes/thinner/ACCEPTANCE.md`,
`scripts/test_thinner_acceptance` and its runner test (both expect 13 (b) to
fail at line 119), and the known-failure columns of the test lists. All
protected.

### 6.4 Then

One small complete painting on the consolidated main: thin underpainting,
wiped lights, a dirty rag and a refold, a thin pass, body paint, a wait, save,
reopen and replay.

## 7. Not known

- `description/` (the product description, imported from its own repo) still describes main at `4e525e5`, before the engine-3 merge (`description/goal.md` pins that commit). It needs a revision pass against the new main: drying, repaints, thinner, the textured rag and the painting clock.
- The size of the retained capacity for any ground, and how a real wash builds
  in thickness. Neither has a measured source.
- Whether (B) meets the owner's rule once passes dry. It was not rerun.
- All probes used one rag fixture and seed, a flat ground for the thinner runs,
  macOS and the release profile.
