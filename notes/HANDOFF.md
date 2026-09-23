# Handoff (2026-09-23 evening): moving to the M3 Pro

Read this first in a new session. Then `notes/round5_plan.md` (the current
round, its rules and a status log) and `notes/scores.md` (every critic
batch so far).

## Where we are
Round 5: OODA loops toward better paintings (plan: notes/round5_plan.md).
A painter reworks one benchmark painting at the easel for ~25 min, reading
`notes/sketchbook.md` first; blind critics score it against its previous
version and an anchor; techniques that raised the score go into the
sketchbook; the worst TOOL defect gets a small engine fix.

- **near** (erratic in snow): best is loop 5, `notes/loops/l5_near.lua`
  (median 22 vs 21 for loop 3 in the same batch; loop 4 was rejected).
- **green** (summer valley): best is loop 3, `notes/loops/l3_green.lua`
  (median 20 vs 18).
- **anchor:** round 2's coast (critics' images in scratch; any fixed image
  of it will do: it's a Rust program under `paintings/fresh2/`, not built).

Tool work merged in round 5 (all with tests, in the easel guide
`crates/easel/README.md` and the sketchbook):
- halos fixed (contact level by running median);
- Friedrich top ground brushed in crossing strokes (no horizontal
  wood-grain, no ploughed-ridge dashes);
- `fir{}` / `fir_wood{}`: firs and woods grown into a drawn envelope;
- `tree_in{}` / `tree_group{}`: broadleaf trees grown into a drawn crown;
- `rock{}`: rocks inferred from a drawn outline, lit by the world's sun;
- spectral.js port as an optional module (not integrated; notes/spectral.md).

## What's next (the rest of round 5)
1. One more loop per painting with the new tools: **near** uses `rock{}` for
   the boulder (seam, flat shadow flank) and fixes the stump's rect()
   snow patch (all three critics named it); **green** uses `tree_in{}` for
   the oak and `tree_group{}` for the field trees.
2. Judge with the **panel** the user asked for: 2 × `google/gemini-3.8-flash`
   and 2 × `openai-codex/gpt-6-astra` (better vision than Claude), blind,
   shuffled, previous versions RE-RENDERED with the current engine (the
   grain fix changed every ground). Report medians overall and per family.
   Critic brief: `notes/briefs/critic_brief.md`; loop painter brief:
   `notes/briefs/loop_painter_brief.md`.
3. Show the user before/after (round 4 originals in notes/amnesia4/ vs the
   final loops) with the score history. The user is "of two minds" about
   structure tools (they grow shapes; they don't paint): judge by output.
4. **Scrub the git history** of the owner's real name with
   `git filter-repo --replace-text` on all branches, verify
   `git log --all -p | grep -ci <name>` is 0, force-push (user approved; do
   it when no agents run). The pre-commit hook in `.git/hooks/pre-commit`
   blocks new occurrences; hooks are NOT cloned: install it on the new
   machine with `cp scripts/pre-commit-anonymity .git/hooks/pre-commit`.

## Setting up the new machine
```sh
git clone https://github.com/aliceisjustplaying/claude-paint && cd claude-paint
cargo build --release --workspace          # Lua 5.5 is vendored; CFLAGS seed in .cargo/config.toml
cargo test --workspace                     # ~10 min; all pass at the handoff
target/release/easel run notes/loops/l5_near.lua --width 3200 --out out/l5_near_full.png
```
Full renders aren't committed (out/*.png is ignored): every painting is a
replayable log, byte-identical to its live session.

## Rules the user set (keep them)
- Anonymity: the project is under the pseudonym "alice"; never write the
  owner's real name or an absolute home path into the repo (write `~/...`).
- US English, no Oxford comma; each message to the user starts with a
  kaomoji; back claims with receipts.
- Python only via `uv` with a virtualenv.
- Scratch in `~/tmp/<task>-<hex>` via `~/.local/bin/agent-tmp`; never /tmp.
- Reviews: `openai-codex/gpt-6-astra`, thinking medium, fast off; neutral
  wording (the word "adversarial" and security-flavored phrasing tripped a
  content filter once).
- Create git worktrees in a step BEFORE spawning subagents into them (a
  parallel batch starts the agent before its cwd exists: exit 1).
- Never chain commit/push after a merge in one command (a failed merge was
  once committed with conflict markers and pushed).
- The user's standing instruction: take your time; quality over speed;
  look at every render before merging; tight OODA loops.

## Branches
All work is merged into main except the per-loop painter branches
(`loop*-near`, `loop*-green`), whose logs and notes are copied into
`notes/loops/`, and the amnesia branches (`amnesia-*`, `easel3-*`,
`easel4-*`), archived in `notes/amnesia2/3/4`.
