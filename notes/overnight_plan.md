# Overnight plan (session 2026-09-23)

North star: a painting app for agents. The engine provides physics and
tools (brushes, paint, canvas, optics, geometry primitives); motifs belong to
the painter. Target: agents composing new, "original" Friedrichs by Claude
from first principles, no reference images. (Starting tweet: models painting
via programs, pixel by pixel, no image model, no off-the-shelf art software.)

## Phase 1: six parallel workstreams (worktrees ../claude-paint-<name>, branch <name>)

| stream | branch | owns (to limit conflicts) |
|---|---|---|
| 1 color semantics | `color` | pigment.rs, palette.rs, wet.rs, color.rs; paint choice in handling.rs `finish_plan` |
| 2 strokes: entropy, coverage, presets | `strokes` | handling.rs stroke planning (centers, trace, curvature, order), style.rs presets |
| 6 stipple | `stipple` | new stipple.rs (Canvas::stipple), bristle.rs dab/tip behavior |
| 3 workflow: crop renders, checkpoints | `workflow` | canvas.rs Frame (origin offset), run.rs, new checkpoint module |
| 4 form and light for solids | `form` | new form.rs, paintings/src/rocks.rs, study bin |
| 5 motifs out of the engine | `motifs` | tree.rs → growth primitive; paintings/src/trees.rs, figures.rs |

Integrator (main session) merges into main in order color, strokes, stipple, workflow, form, motifs, re-records
the golden, pushes to github.com/aliceisjustplaying/claude-paint.

## Decontamination (user, before sleep)
monk2 (a study of Monk by the Sea) was painted by copying one known picture
too closely, and Moonrise leans on Two Men Contemplating the Moon. Neither
may steer the engine. At merge time: move monk2 out of the build into
paintings/archive/ (not built, not a regression target); the monk figure
leaves the shared motifs. Amnesia painters get a worktree with no existing
painting programs (monk2, moonrise, fresh/ removed) and read only README,
research notes and engine docs, not notes/friedrich.md or git history.
Engine quality is judged on study sheets and new compositions, never on
likeness to a known painting.

## Phase 2: adversarial code review
openai-codex/gpt-6-astra subagents, medium effort, fast off. Fix findings.

## Phase 3: amnesia round 2
Three fresh painters, original Friedrich compositions, 1–2 h budget each.
Then stop and evaluate with the user.

## Status log
- Six subagents launched (color, strokes, stipple, workflow, form, motifs); shared brief copied to notes/overnight_brief.md.
- motifs merged (5ec3808): growth.rs skeletons; open: spruce reads as araucaria, not Friedrich's drooping spire; oak twig tips tuft into pom-poms at 1000px; ridge bands on thick limbs.
- form merged: ranges read well (gullies, layered haze); rocks solid but covered in uniform crumpled-foil dab texture, outcrop still loaf-like; orange ground specks in skies (coverage); 3200px dashes.
- (updated as work lands)
