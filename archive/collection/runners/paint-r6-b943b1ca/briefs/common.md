# Round 6 overnight: common brief (claude-paint)

You are one of three engineers working in parallel overnight on claude-paint,
a physical oil-paint simulator in Rust where paintings are programs (no image
model, no reference images). The owner is asleep; the integrator session
(me) reviews and merges your branch. Work autonomously; do not ask questions.

Read first, in this order:
1. README.md (the two rules: work only from knowledge, never pictures; first
   principles: every mark made by a simulated brush carrying paint).
2. notes/HANDOFF.md, especially "Round 6 plan: the mark-making lab". The
   owner's measuring stick is "does it look good?" Principles there: mark
   economy, edges and value families across objects, a hand in time, real
   wet-on-wet, quiet surface, structure tools as scaffolds.
3. notes/advice/astra.md (a second opinion the owner accepted).
4. crates/easel/README.md (the Lua easel painters use) and notes/workflow.md.
Then whatever your stream needs (notes/drying.md, notes/surface.md,
notes/trees_in.md, notes/motifs.md, notes/research/*.md).

## Setup and rules
- Your worktree and branch are named in your task. Work ONLY there. Never
  touch ~/src/a/claude-paint (main) or the other worktrees. Do not push.
- Commit early and often on your branch with clear messages.
- target/ is pre-seeded; builds are incremental. Use `--release` builds for
  renders (target/release/easel, `cargo paint`).
- Scratch: run `~/.local/bin/agent-tmp <stream-name>` once and export
  TMPDIR=TMP=TEMP to that dir. Never write to /tmp or OS temp dirs.
- The machine (12 cores) is shared with two other agents and the
  integrator's renders. Wrap every render and test in `timeout` (e.g.
  `timeout 900 ...`). Prefer 1000px renders and crops (`--crop`) over full
  3200px renders; run the whole workspace test suite only at the end.
- View PNGs only through `scripts/peek SRC OUT.jpg [H W Y X]`. Look at every
  result like a painter would before you call it done. Quality over speed.
- `cargo test --workspace` must pass at the end. The golden scene
  (crates/paint/tests/golden_scene.txt, debug profile) may change only if
  your stream intends to change output: re-record with UPDATE_GOLDEN=1 and
  say so in the commit.
- Replay compatibility matters: the benchmark logs notes/loops/l5_near.lua
  and notes/loops/l3_green.lua are the owner's current best paintings. Say
  explicitly in your report whether they render identically after your
  change (compare PNGs of `target/release/easel run <log> --width 1000`
  before and after, byte-for-byte or max pixel difference).
- Keep `cargo clippy` free of new warnings.
- Python only via `uv` with a virtualenv (rarely needed).
- US English, no Oxford comma.
- Anonymity: the project is published under the pseudonym "alice". Never
  write the owner's real name or an absolute home path into any committed
  file: write `~/...`. A pre-commit hook rejects violations; never bypass it.
- Never chain commit/push after a merge in one command.

## Deliverable
- Code committed on your branch, tests passing.
- A notes file `notes/<stream>.md`: what changed and why, the API a painter
  uses (short examples), evidence (image paths committed under
  notes/<stream>/ as ≤1000px JPEGs made with scripts/peek, and what you
  saw), known issues and what you would do next.
- If it changes what painters do, update crates/easel/README.md and
  notes/sketchbook.md (sketchbook rules: every entry states its ceiling;
  principles before recipes).
- Final message: a concise report (what landed, commits, API, evidence
  image paths, whether the benchmark logs render identically, open issues).
  Budget: about 2–3 hours. Land something solid and committed rather than
  something ambitious and half-done.
