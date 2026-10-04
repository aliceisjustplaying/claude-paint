# Start here (October 4, 2026)

**Newer: [HANDOVER-2.md](HANDOVER-2.md) (October 4, evening; branch `rag`). Read it first.**

The consolidation is done. The work is in [HANDOVER.md](HANDOVER.md), section 6
(rag, then thinner, then sienna, then the first engine-3 painting). Sections 2
to 5 there describe the state before the consolidation; skip them.

## State

- One checkout, `~/src/a/claude-paint`, on `main`, pushed. No other worktrees or
  branches, local or remote. Everything from the engine-3 line is on main.
- `scripts/test --all` is red in exactly three steps, all expected:
  `thinner-all` (check 13 (b), fixed by section 6.3), and `baseline-state` and
  thinner check 2 on the `rag` scene only (the textured rag; left red until the
  rag work). The verdict is a plain FAIL. Anything else red is new.
- Main moves by fast-forward, not through the gate. Changes to protected files
  (`notes/golden_paths.txt`) still need the owner's say.
- `description/` (the product description) still describes main before engine
  3, at `4e525e5`. It needs a revision pass later; it is not part of section 6.
- Outside reviews: `notes/engine3/reviews/`. History of how this got here:
  `notes/engine3/history/`.

## Before building

- Start sccache from a normal shell first (`sccache --start-server`); see
  HANDOVER.md 5.0 for why.
- Builds and tests go through `~/src/a/claude-paint-tools/lockrun`;
  `scripts/test` takes the lock itself. A full `scripts/test --all` takes about
  8 minutes.
- New worktrees are sparse (a local post-checkout hook skips `notes/` images
  except the thinner baseline). The main checkout is full.
- Keep `~/src/a/claude-paint-tools`, `-receipts` and `-reviews`: the gate
  scripts and notes refer to them by path.
- The repository is public. Never write the owner's real name or a home path
  that contains it; write `~/...`.
