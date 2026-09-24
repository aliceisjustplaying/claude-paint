# Handoff (2026-09-23 evening): moving to the M3 Pro

**Update 2026-09-24 morning: Round 6 night 1 ran (steps 1–4 of the plan
below, plus critic panels). Read `notes/round6.md` first: what landed on
main, what waits on branches and the decisions for the owner.**

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

## Update: second opinions (read notes/advice/astra.md and gemini.md)
The owner felt progress went sideways since round 2–3. Both advisors
(gpt-6-astra, Gemini 3.8 Flash) agree: (1) A/B the paint relief lighting on
identical paintings first (cheap, the owner judges); (2) mark economy:
masses, edges and a few accents instead of painting every generated detail
(keep structure for placement and branching: the bare oak is the evidence);
(3) test wet/tacky/open interaction: the sketchbook's "dry() before any
passage" likely causes the pasted-on cutout look; (4) no full Bob Ross
sprint: bounded single-subject mark-making studies instead; (5) the owner's
eye decides, critics are diagnostic. Rejected: Gemini's idea to benchmark
Friedrich's actual masterpieces (copying). The next plan is those three
experiments, in order, before more loops.

## Round 6 plan: the mark-making lab (high level, agreed with the owner)

**The measuring stick is the owner's eye: does it look good?** Not "does it
look like a Friedrich", not a critic total. Critics (panel: Gemini 3.8 Flash
+ gpt-6-astra) are diagnostic only: they name defects, they don't decide.

**Principles** (from the owner's review and both advisors, notes/advice/):
- *Mark economy*: say as much as possible with as few marks as possible.
  A crown is a dark mass, a few unequal lights, a couple of branches and
  sky holes, not 30,000 touches ("confetti").
- *Edges and value families across objects*: decide edges between things
  (found, soft, lost); group darks with darks and lights with lights across
  objects (the rock's shadow side, its cast shadow and the wood behind read
  as one dark shape). Objects must meet their surroundings, not be finished
  alone inside their own masks ("pasted on", halos, the rock's pale base
  strip, the stump's rectangle).
- *A hand in time*: a stroke costs the time a hand takes to make it; a
  painter works in sessions (a few hours, a couple of times a day) and paint
  sets between them. Economy and wet/dry timing then come from physics, not
  rules. Friedrich over days or weeks is fine; so is a fast alla prima day.
- *Real wet-on-wet*: open paint must blend, drag and soften at contours;
  the sketchbook's "dry() before any passage" default goes (it likely makes
  the cutout look). Dry and tacky stay tools for when they're wanted.
- *Quiet surface*: the relief lighting currently embosses every stroke
  ("embossed plastic", "grooves"); Friedrich's surface is thin and smooth.
- Structure tools (fir, tree_in, rock) stay as SCAFFOLDS (placement,
  silhouette, major branching, light), not as things to trace in full. The
  bare oak is promising but "too computationally fractal" with twigs
  floating in the air: fix connectivity, fewer and more deliberate twigs.
- Never copy existing paintings or benchmark against them.



Note: a mirror of the PRE-scrub history is kept on the old machine at `~/tmp/paint-overnight-*/scrub/backup.git` (owner: keep it; never push from it).
