# Handoff (2026-09-23 evening): moving to the M3 Pro

**Update 2026-09-24 night (read first): Round 7.** Principles: `notes/principles.md`
(tools give physics and constraints, not answers; entropy; feature freeze; against
reward hacking). Alice's reviews: `notes/round6/alice_review.md` (last sections).
The engine is variant d (`notes/round7/winter_ab.md`, `winter_d.md`): round 2's
winter program on it is "remarkably close" to the original. Tonight: three free
paintings on engine d (`notes/round7/arms/`, key there): none moves Alice yet;
the three converged on nearly the same picture; the Lua easel with everything
(C) "has the best vibe somehow". A second opinion from Claude Fable 5.1:
`notes/advice/fable_r7.md`. Open: the wet engine (branch r6-wet, unmerged; needs
main merged: edges/clip fix vs its exchange rewrite); piles (branch r7-piles,
parked, Alice wants to keep it); the rendering bug Alice saw (a whitish
horizontal line through a grass patch in painting A = arm 1); git identity: this
repo sets `alice` locally (the per-directory git identity include for ~/src/a is missing on
this machine). Next: decide the direction with Alice after Fable's advice.

**Day of 2026-09-25 (with Alice):**
- **Round 10 = the breakthrough** (`notes/round10/`, `notes/round2_magic.md`):
  round 2's brief verbatim, Rust, Opus at thinking high, no recipe book.
  Alice: "this feels like progress". The person whose post started the
  project: "The trees are amazing ... the second one especially" (the
  summer lime). Model judges ranked that one last: humans and models
  diverge there.
- **Round 11** (`notes/round11/`): a trunk study (3x Opus) and whole winters
  by Astra, Gemini Flash (ran out of Google credits at the end) and Fable
  5.1; blind cross-critique: every model ranked its own painting 5th-6th;
  round 2 still first for all; round 10's Opus paintings next.
- **Fixed and merged:** dry rims (strokes over dry paint kept their outlines,
  `notes/fixes/dry_rims/`, start with `LOOK_HERE_far_hills_4x.png`); the
  test audit (13 items, -291 lines, release hand_time 2 min -> 8 s); the
  pollard willow removed from the docs; a clippy error on main.
- **Decided:** one resolution for painting, looking and delivering: ~0.2 mm
  per pixel (2250-2400 px for a small Friedrich canvas), from the next round.
- **Cracks** (`notes/cracks_lab/README.md`): real craquelure has direction
  (the Monk: broadly down-right; other paintings other patterns), clusters
  and varying amounts; "anything that reads like repetition reads digital".
  Ours: one-pixel hairlines, even coverage, rings at the corners.
  **Running:** `fix-cracks` (branch fix-cracks): make the existing crack
  settings reach the picture, test-first; merge only after Alice sees its
  before/after (`notes/fixes/cracks/`). Then: direction ~0.3, diagonal
  down-right, per painting.
- **Open:** the sky's "JPEG effect" (not the stipple alone: round 2
  stippled too); floating twigs (3 painters: a branch should start at its
  parent's width); checkpoint staleness by line (6 painters); whether to
  keep Lua (a clean test is proposed); more models (Gemini needs credits).

**Overnight (2026-09-25, after Alice slept):**
- **Round 8** (`notes/round8/blind/`, key there): round-2-style brief, commission "a winter
  landscape", three arms. Every painting has figures, a route and hand-written motifs.
  Blind critics: Gemini ranks round 2's winter first, Astra ranks arm 1 (the cross) first
  ("moves me most") and round 2 as the most painted.
- **The "JPEG artifact" look is the stipple layer** (`notes/round7/texture/README.md`):
  forensics plus a truly blind judge (no key on disk) picked stipple-off in both passages.
  (A Gemini judgment read the key file and was discarded.)
- **Round 9** (`notes/round9/`, justification in its README): Round 8 plus two brief lines
  (skies in broad blended strokes, no stipple veil; bury the feet of things). Blind critics:
  Gemini ranks round 2 first, then arm 2 (the Ryck); Astra ranks arm 3 (the wayside cross)
  first. **Both put the new craquelure first in their advice** ("antique skin", "cracked
  glass"); round 2's older cracks aren't blamed. With vs without cracks:
  `notes/round9/nocracks/`. Candidate regression: the Round 7 craquelure (merge 112ed6b).
- **Recurring engine friction** (four painters): strokes laid over dry paint keep their
  outlines (a "lacy net", "glass tubing"; one measured 399 um at the rims vs 11 um inside):
  the top bug to investigate. Also `edge="lost"` overpaints small holes.

**Morning summary (2026-09-25):** Fable (`notes/advice/fable_r7.md`): "the engine isn't what's
stopping you. The brief is." The composition guide `notes/briefs/friedrich_painter.md`
(required reading tonight) says "You can have no figure at all", "Keep the foreground
bare", "Leave out, then leave out more" (lines 34, 58, 60): it bans the winter picture;
round 2's brief said the opposite ("full of tiny particular details",
`notes/amnesia_brief.md:49`); tonight's painters obeyed ("There is no figure"). It
proposes: round 2's short brief back, assigned themes (winter, coast, mountains) as a
patron would, hand time off for painters, the winter program as a golden picture gate,
round 8 = six painters (each theme in Rust and at the easel without procedural tools),
blind against round 2. Critics on the three arms: `notes/round7/arms/critics/` (C
strongest; "a relationship, not just a motif"; the "JPEG" look = patchy fine-scale
mottling over smooth fields). Texture forensics: branch r7-texture.

**Update 2026-09-24 morning: Round 6 night 1 ran (steps 1–4 of the plan
below, plus critic panels). Read `notes/round6.md` first: what landed on
main, what waits on branches and the decisions for Alice.**

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
4. ~~Scrub the git history~~: done (see Round 6, step 0). Hooks are NOT
   cloned: install `scripts/pre-commit-anonymity` on every new clone.

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
  the user's real name or an absolute home path into the repo (write `~/...`).
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
Alice felt progress went sideways since round 2–3. Both advisors
(gpt-6-astra, Gemini 3.8 Flash) agree: (1) A/B the paint relief lighting on
identical paintings first (cheap, Alice judges); (2) mark economy:
masses, edges and a few accents instead of painting every generated detail
(keep structure for placement and branching: the bare oak is the evidence);
(3) test wet/tacky/open interaction: the sketchbook's "dry() before any
passage" likely causes the pasted-on cutout look; (4) no full Bob Ross
sprint: bounded single-subject mark-making studies instead; (5) Alice's
eye decides, critics are diagnostic. Rejected: Gemini's idea to benchmark
Friedrich's actual masterpieces (copying). The next plan is those three
experiments, in order, before more loops.

## Round 6 plan: the mark-making lab (high level, agreed with Alice)

**The measuring stick is Alice's eye: does it look good?** Not "does it
look like a Friedrich", not a critic total. Critics (panel: Gemini 3.8 Flash
+ gpt-6-astra) are diagnostic only: they name defects, they don't decide.

**Principles** (from Alice's review and both advisors, notes/advice/):
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

**Order of work** (small steps, Alice looks at each):
0. On the new machine: clone, build, install `scripts/pre-commit-anonymity`
   (`cp scripts/pre-commit-anonymity .git/hooks/pre-commit`). The history
   scrub is DONE (2026-09-23 evening): all 18 branches rewritten with
   `git filter-repo --replace-text`, 0 matches in a fresh mirror clone of
   GitHub, main's tree unchanged; any clone made before that date has the
   old history: re-clone.
1. **Relief A/B** (no repainting): replay the best near and green logs with
   relief off / low / current; Alice picks. Make the winner the default.
2. **Hand in time**: each stroke/touch/stipple advances the easel clock by a
   plausible hand time (from length, size, brush); painting sessions with
   rests between them; show open/tacky/dry in `look --mode wet`. Tests;
   sketchbook note on working in sessions.
3. **Real wet-on-wet**: audit what open paint does at contours today;
   studies of the same gesture over open, tacky and dry paint; engine work
   where wet edges don't soften or drag like paint; drop the dry() default.
4. **The lab**: single-subject studies, each done the old way and the new
   way (economy, lost edges, value families, wet interaction, hand in
   time): a foliage mass; a rock touching the ground; a quiet sky; a bare
   tree silhouette; a water edge. Alice judges each pair; winners go
   into the sketchbook with their ceilings.
5. **Back to paintings**: rework the near and green benchmarks (and a fresh
   free painting) with what the lab found; Alice judges; the critic
   panel names the next defects. Repeat in short loops.
Bob Ross stays parked; his documented techniques (dark-before-light masses,
tapping, knife deposits) may be borrowed as lab exercises.

Note: a mirror of the PRE-scrub history is kept on the old machine at `~/tmp/paint-overnight-*/scrub/backup.git` (Alice: keep it; never push from it).
