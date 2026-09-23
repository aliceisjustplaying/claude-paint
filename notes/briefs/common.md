# Common brief for all overnight workstreams (claude-paint)

You are one of five engineers working in parallel overnight on claude-paint, a
physical oil-paint simulator in Rust where paintings are programs (no image
model, no reference images). The user is asleep; an integrator session
(me) will merge your branch into main. Work autonomously; do not ask questions.

## North star
"A painting app for agents." The engine provides physics and tools (brushes,
paint, canvas, optics, geometry primitives). Motifs belong to the painter.
Goal: agents composing new, original Caspar David Friedrich paintings from
first principles, from what is known about his materials and method. Read
README.md (the two rules: work only from knowledge, never pictures; first
principles: every mark made by a simulated brush carrying paint, KM optics,
no flat fills or optical blends pretending to be paint).

Read first: README.md, notes/fresh_painters.md (what three fresh painters
struggled with, plus the user's review; this is your requirements list),
notes/overnight_plan.md, notes/friedrich.md, notes/research/*.md as needed.
The fresh painters' programs are archived in paintings/fresh/*.rs (not built;
they show the workarounds painters hand-rolled, which is useful evidence).

User's review, condensed: everything is too straight, too neat, too
horizontal ("digital"); not enough entropy; the three skies look the same
(style defaults paint the sky, not the painter); the oak is tree-shaped but
does not grow like a tree (twigs scattered, disjointed); mountains look like
"weird sea with hard edges"; Friedrich's work is full of tiny particular
details while agents paint broad strokes because the API makes broad easy
and detail laborious.

## Your setup
- Your worktree and branch are given below. Work ONLY there. Never touch
  ~/src/a/claude-paint (main) or other worktrees. Do not push.
- Commit early and often on your branch with clear messages (the last
  session died in a power loss; uncommitted work is lost work).
- The target/ dir is pre-seeded; builds are incremental.
- Scratch files: run `~/.local/bin/agent-tmp <stream-name>` once,
  export TMPDIR=TMP=TEMP to that dir. Never write to /tmp or OS temp dirs.
- Four other agents share this 10-core machine. Wrap every render and test in
  `timeout` (e.g. `timeout 600 cargo paint ...`). Prefer 1000px previews and
  small study binaries over --full renders; a hung render is killed, not waited on.
- View PNGs only through `scripts/peek SRC OUT.jpg [H W Y X]` (PNG renders
  are huge). Look at your results; judge them like a painter would.
- `cargo test -p paint` must pass. The golden scene fingerprint
  (crates/paint/tests/golden_scene.txt, debug profile) may change when you
  intentionally change output: re-record with UPDATE_GOLDEN=1 and say so in
  the commit. Keep existing paintings (friedrich_moonrise_valley,
  friedrich_monk2, study_*) building and rendering; update them to new APIs.
- Keep `cargo clippy` clean-ish; no new warnings.
- For Python (if ever needed) use `uv` with a virtualenv.
- Write US English, no Oxford comma.

## Conflicts
Stay inside the files your stream owns where you can. If you must touch a
shared file (lib.rs exports, tests.rs, a painting binary), keep the change
small and local. Other streams:
1 color (pigment/palette/wet/color.rs, paint choice in handling finish_plan),
2 strokes (handling.rs planning: centers, trace, mark kinds; style.rs presets),
3 workflow (canvas.rs Frame origin/crop, run.rs, checkpoints),
4 form (new form.rs, rocks),
5 motifs (tree.rs → growth primitive, paintings/src motifs).

## Deliverable
- Code committed on your branch, tests passing.
- A notes file `notes/<stream>.md`: what changed and why, the API a painter
  uses (short examples), evidence (render paths + what you saw), known issues
  and what you would do next.
- A study binary or updated study sheet in paintings/src/bin that shows the
  feature (so the integrator can look at it).
- Final message: a concise report (what landed, commits, API, evidence
  image paths, open issues). Budget: about 2 hours of work; land something
  solid and committed rather than something ambitious and half-done.

## Anonymity (required)
This project is published under the pseudonym "alice". Never write the owner's real name or an absolute home path into any committed file: write paths as `~/...` (e.g. `~/tmp/...`, `~/src/a/...`). A pre-commit hook rejects violations.
