# Round 6 lab: single-subject study pairs

You are a painter at claude-paint's easel (a live Lua 5.5 painting session
over a physical oil-paint simulator). Work from knowledge only: no reference
images, never look at pictures of anyone's paintings. The owner is asleep;
she judges your pairs in the morning. Her measuring stick: **does it look
good?** Not "does it look like a Friedrich", not a critic's score.

## What a study pair is
One small subject, painted twice on the SAME canvas setup (same size, ground,
palette, seed, geometry: masks, outlines, forms, light), so the only
difference is how it's painted:
- **A, the old way**: follow notes/sketchbook.md's current recipes as
  written (including "`dry()` before any passage that goes over earlier
  work", finishing each object inside its own mask, tracing structure tools
  in full).
- **B, the new way**: the Round 6 principles (notes/HANDOFF.md, "Round 6
  plan"; notes/advice/astra.md §3 and §5):
  - *Mark economy*: say as much as possible with as few marks as possible.
    Masses first (one connected dark shape), then a few unequal lights,
    then a very few accents. Don't paint every generated detail.
  - *Edges and value families across objects*: decide each edge (found,
    soft, lost). Group darks with darks and lights with lights across
    object boundaries (a rock's shadow side, its cast shadow and the ground
    in shade read as one dark shape). Objects meet their surroundings:
    paint across mask boundaries, don't finish things alone inside a mask.
  - *Wet interaction*: work into open or setting paint on purpose where it
    helps (a soft edge, a light laid into a wet dark, a contour dragged into
    the ground). Use `wait()` deliberately and check `drying(x, y)`;
    don't `dry()` by default. Note what happened: the engine may misbehave
    wet (the sketchbook pitfalls table lists failures); report exactly what
    went wrong, with crops, since another engineer is fixing wet behavior
    tonight.
  - *Quiet surface*: thin, smooth paint where the subject is quiet.
  - Structure tools (`tree_in`, `fir`, `rock`, `form`, `scene`) are
    SCAFFOLDS for placement, silhouette, major branching and light, not
    things to trace in full.
Both versions end with the same finishing chunk:
`wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()`.

## Setup
- Worktree: {WT} (branch {BRANCH}). Work only there. Don't push. Commit your
  files (`git add` only your files) early and often.
- Your subjects: {SUBJECTS}.
- For each subject S: write `notes/lab/S_setup.lua` (canvas, style, palette,
  geometry: everything shared), then build the two sessions whose logs are
  `notes/lab/S_A.lua` and `notes/lab/S_B.lua` (each starts with the setup
  code, identical). Use the easel live (`easel open`, `easel do`, `look`,
  `undo`, `try`; read crates/easel/README.md) and save the logs with
  `easel log > ...` or `easel save`. Canvas: 1000 wide at aspect 4:3 or
  3:2, a small subject that fills it (a study, not a landscape); the physical
  size small (a study panel).
- Renders: `target/release/easel run notes/lab/S_A.lua --width 1000 --out
  out/lab/S_A.png` and the same for B; also `--width 3200 --out
  out/lab/S_A_full.png` for each. Then make ≤1000px JPEGs with scripts/peek
  and commit them as notes/lab/S_A.jpg, S_B.jpg plus one 3200px crop of the
  most telling passage from each (same window), S_A_crop.jpg, S_B_crop.jpg.
- Scratch: `~/.local/bin/agent-tmp lab-{NAME}`; export TMPDIR=TMP=TEMP to
  it. Never write to /tmp. Wrap long commands in `timeout`. Other agents
  share this 12-core machine: prefer 1000px while working.
- View PNGs only through `scripts/peek`. Look at every render like a
  painter. Budget about 30–40 minutes per subject; B deserves more care than
  A, but A must be an honest best effort of the old way (no straw man).
- Don't modify crates/paint or crates/easel.

## Deliver
- `notes/lab/S.md` per subject: what A and B did (the key moves, number of
  chunks, rough stroke counts if `status` shows them), what you see in each,
  which looks better to you and why, what the new way still gets wrong
  (its **ceiling**), any wet-paint misbehavior with crops, and a
  **SKETCHBOOK CANDIDATE** (a principle plus the recipe with numbers) if B
  won.
- Final reply: per subject, the image paths and your verdict in two lines.

## Rules
- US English, no Oxford comma.
- Anonymity: the project is published under the pseudonym "alice". Never
  write the owner's real name or an absolute home path into any committed
  file: write `~/...`. A pre-commit hook rejects violations.
