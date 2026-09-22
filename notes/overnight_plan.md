# Overnight plan (session 2026-09-23)

North star: a painting app for agents. The engine provides physics and
tools (brushes, paint, canvas, optics, geometry primitives); motifs belong to
the painter. Target: agents composing new, "original" Friedrichs by Claude
from first principles, no reference images. (Starting tweet: models painting
via programs, pixel by pixel, no image model, no off-the-shelf art software.)

## Phase 1: five parallel workstreams (worktrees ../claude-paint-<name>, branch <name>)

| stream | branch | owns (to limit conflicts) |
|---|---|---|
| 1 color semantics | `color` | pigment.rs, palette.rs, wet.rs, color.rs; paint choice in handling.rs `finish_plan` |
| 2 strokes: entropy + stippling | `strokes` | handling.rs stroke planning (centers, trace, new mark kinds), style.rs presets |
| 3 workflow: crop renders, checkpoints | `workflow` | canvas.rs Frame (origin offset), run.rs, new checkpoint module |
| 4 form and light for solids | `form` | new form.rs, paintings/src/rocks.rs, study bin |
| 5 motifs out of the engine | `motifs` | tree.rs → growth primitive; paintings/src/trees.rs, figures.rs |

Integrator (main session) merges into main in order 1, 2, 3, 4, 5, re-records
the golden, pushes to github.com/aliceisjustplaying/claude-paint.

## Phase 2: adversarial code review
openai-codex/gpt-6-astra subagents, medium effort, fast off. Fix findings.

## Phase 3: amnesia round 2
Three fresh painters, original Friedrich compositions, 1–2 h budget each.
Then stop and evaluate with the user.

## Status log
- (updated as work lands)
