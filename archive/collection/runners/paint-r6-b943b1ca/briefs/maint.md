# Maintenance round: fix the thermos findings (claude-paint)

Read notes/round6/thermos.md on main first: the deduplicated findings of
eight reviewers (bugs B1–B10, structure S1–S6). The full reviewer reports
(with exact file:line evidence and proposed designs) are in
~/tmp/paint-r6-b943b1ca/thermos/*.md: read the ones relevant to
your items. Also read ~/tmp/paint-r6-b943b1ca/briefs/common.md
for the house rules (worktree only, commit often, don't push, scratch via
~/.local/bin/agent-tmp, timeout long commands, tests in debug, anonymity,
US English no Oxford comma).

## The bar
- **Refactors preserve behavior exactly.** notes/loops/l5_near.lua and
  notes/loops/l3_green.lua at 1000px must be byte-identical (`cmp`) to a
  render from your starting commit, the golden scene and the replay hashes
  in crates/easel/tests/hand_time.rs unchanged, unless an item is a bug
  fix that is meant to change output, and then only where it's meant to
  (say so in the commit, with the measured difference).
- **Bugs get a test that fails first**, then passes.
- Delete complexity rather than move it: that's what the reviewers asked
  for. But measure twice: this engine's floats, RNG consumption and tile
  order are replay-critical.
- Keep clippy free of new warnings; run `cargo fmt` only on code you
  touch (don't reformat whole files you don't otherwise change).
- Full `cargo test --workspace --no-fail-fast` (debug) and the release
  replay test (`cargo test --release -p easel --test hand_time`) pass at
  the end.
- Update the notes that describe what you changed (notes/time.md,
  notes/oak.md, notes/varnish.md, notes/wet.md) briefly.
- Three agents work in parallel on disjoint files (see your task); stay
  inside yours. If you must touch another's file, keep it to a line and
  say so in your report.
- Final message: concise report: what you fixed (by id), commits, tests,
  byte-identity checks, line counts before/after of the files you
  restructured, anything you deliberately left and why.
