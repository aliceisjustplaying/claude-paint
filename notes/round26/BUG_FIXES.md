(｀・ω・´) The two approved Monet retrospective bugs are fixed in the current source.

- The maintained classifiers in `notes/round24/runner/`, `notes/round25/runner/` and
  `notes/round26/runner/` now recognize `gesture`, `wipe`, `blot`, `spatter`, `lay`
  and `scrape`. Existing helper and alias propagation uses these names too.
  The classifier remains conservative: a brush or knife cleaning call named `wipe`
  also counts as painting. [Classifier](runner/painting_chunks.py:21),
  [stop-decision regressions](../round24/runner/test_painting_chunks.py:24).
- `pile` captures all trailing Lua arguments and rejects them, including `nil`,
  with an error explaining that tube parts and options belong in one table.
  The single-table parsing is unchanged. mlua 0.12.1's `FromLuaMulti for T`
  reads only the first value; `(Table, Variadic<Value>)` captures the remainder.
  [Binding](../../crates/easel/src/api.rs:1752),
  [Lua regression](../../crates/easel/src/api.rs:2174).

The test-audit authoring gate was applied before adding regressions. On the
pre-fix code, all 27 runner cases failed because drawing did not increase the
painting count. The Lua regression failed because the invalid call returned
success. After repair:

| Check actually run | Result |
|---|---|
| `uv run --no-project --with pytest --with pillow python -m pytest -q -p no:cacheprovider notes/round24/runner/` | 291 passed in 24.35 s, including 27 new cases across rounds 24–26 |
| `cargo test -p easel --bin easel -- --skip thinner_tests::` | 102 passed, 8 ignored, 11 filtered out in 45.63 s |
| `cargo check -p easel --no-default-features --features box-giverny` | Passed |
| `git diff --check` | Passed |

Rust commands ran through `uv run --no-project python scripts/lockrun --timeout 600 --` in
the existing `target/`. Existing Rust tests executed actual brush gestures and rag
wipes. The new Lua test checks extra arguments and valid single-table recipes on
engines 1–5. The full repository suite and thinner acceptance tests were not run.

Read-only classification of the original Monet log changed its painting-chunk
count from 73 to 99: chunks 142–146 now count as painting and waiting chunk 147
does not. No saved counts or completion records were rewritten. The log's chunk 2
contains the invalid two-table `pile` call, so the stricter current binary rejects
that historical log. Its frozen source remains the replay source. [Original call](../../../paint-studio-12e572/paintings/lua/painting.lua:13),
[frozen source and launch copy](LAUNCH.md:3).

No launch copies, frozen runtime worktrees, painters, prompts or viewer services
were changed. Earlier round runners were left as historical sources. The existing
retrospective and evidence files and the other agent's studio work were not staged.
