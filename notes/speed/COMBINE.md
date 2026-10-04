# Combining engine 3: codex/speed and thinner2

Branch `engine3-overnight`, worktree `~/src/a/claude-paint-engine3` (full
checkout: `git sparse-checkout disable` after the hook made it sparse; 1564
files in index and tree). Started from `codex/speed` at `2b85ff6`.

## Merge 1: thinner2 at ffef02e

`git merge --no-ff --no-edit thinner2` (thinner2 =
`ffef02ee1c…`), commit `6917a91`. **No conflicts.** Merge base `a97c3a6`
(the baseline commit, which thinner2 had merged).

Files changed on both sides, merged by Git without conflict, hunks not
overlapping:

| file | thinner2 | codex/speed |
|---|---|---|
| `crates/easel/src/api.rs` | pile `thinner`, the pile's paint, the thinner's API | `cfg(all(test, tube_box))` on the test module |
| `crates/easel/src/main.rs` | the `thinner_measure` and `thinner_tests` test modules | the state dumper hook (in a97c3a6, shared), the thick-swatch test split |
| `crates/easel/src/session.rs` | `piles()` reads the pile's paint | `#[ignore]` on the engine-1 painting replay |
| `crates/paint/src/rag.rs` | the rag's solvent (production code) | `aged_until_set` in the test module |

Checks after the merge:

- Every thinner2 file is byte for byte as at ffef02e, except
  `crates/paint/src/rag.rs`, where the only differences are speed's two
  hunks inside `mod tests` (from line 616 on: lines 723 and 912).
- `crates/paint/src/state_dump.rs`: as on thinner2 (its `wet.solvent` field
  added); `notes/thinner/baseline/`: unchanged.
- The 22 frozen files (`thinner-tests-APPROVED.md`): 20 match the approved
  b2a0e14 hashes. Two differ, exactly as they are on thinner2's head (its
  round-4 corrections, not yet reviewed):
  `crates/easel/src/thinner_tests.rs` (`e167e7f16bb193420b991033feb3299d0e26619897dc3ee1f1aadf3c10508f99`)
  and `crates/paint/tests/thinner_physics.rs` (`57371c2e8ddc9d5471f3ea87d623f91c48fb99264a80ebcfe85814a9f3b5cad6`).
- Nothing of the thinner's was edited.
