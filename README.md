# Claude Paint product description

This describes Claude Paint from a painter's, runner's and viewer's point of view: what an action changes, what survives failure and what can be observed afterward. The source is read-only at commit `4e525e50897807e9b5f734071dfeb330f1a393d3` in `../claude-paint`.

## Scope

The primary surface is the current tube-based easel. The default developer build includes replay and finishing; the exported painter build has one painting and omits those capabilities. The painter harness and studio browser are included because they are the other user-facing surfaces in this project. Historical scene helpers are covered as replay compatibility, not as current painting tools. Research notes, archived rounds, engine internals without a user action and the separately maintained stillwet gallery are outside this description.

The source map is [the guide](../claude-paint/notes/easel_guide.md), [command handling](../claude-paint/crates/easel/src/main.rs), [chunk state](../claude-paint/crates/easel/src/session.rs), [painting operations](../claude-paint/crates/easel/src/api.rs), [harness](../claude-paint/harness/painter/painter.ts) and [viewer](../claude-paint/studio/index.html). Behavior evidence lives in `crates/easel/tests`, `crates/paint/tests`, `harness/painter/test`, `studio/test_studio.py` and `scripts/tests`. Defaults live beside their operations, especially `look.rs`, `time.rs`, `paint/src/style.rs` and `paint/src/handling.rs`.

## Shared skeleton

Every feature has Summary; The simple case; The interaction, event by event; Modifiers; Cancel and interrupt; Interactions with other systems; Edge cases; Open questions and verification. Its diagram describes visible states, including commit and discard boundaries.

The unit of interaction is a submitted command or Lua chunk. Its phases are Starting, Ending at once, Becoming extended, While extended and Finishing. Browser actions use the same phases for selection, loading and display. Extended means computation or loading is underway, not that the painter can edit a running chunk.

The variant axes are build/surface, input options, existing painting state and selected viewing mode. The fixed interrupt rows are Explicit abort; Another action; Environment failure; Target changed externally; Input channel changed. The fixed cross-cutting order is Access; History; Containers; Restricted state; Offline; Collaboration; Notifications; Preferences. No effect means the action does not affect that concern, not that the whole product is immune to failure.

## Structure and coverage

`drafted` means source-backed prose exists. Runtime checks are recorded individually. `verified` requires a person to run every P1 and P2 checklist row for the document; automated probes alone do not earn it.

| Document | Purpose | Coverage |
|---|---|---|
| [Looking](painting/looking.md) | Pilot: canvas views, crops, modes and palette | drafted |
| [Commands and chunks](foundations/commands.md) | Invocation, commit, rollback and interruption | drafted |
| [Canvas and materials](foundations/canvas.md) | Coordinates, setup, paint and persistent objects | drafted |
| [Sessions and surfaces](foundations/sessions.md) | Build differences, opening, closing and session selection | drafted |
| [Painting passages](painting/passages.md) | Area work, blending, stippling and edge handling | drafted |
| [Brushes](painting/brushes.md) | Loading, wiping, strokes and touches | not started |
| [Shapes](painting/shapes.md) | Masks, outlines and controlled variation | not started |
| [Drawing](painting/drawing.md) | Pencil, chalk, erasing and fixing | not started |
| [Rags](painting/rags.md) | Lifting paint, refolding and spirits | not started |
| [Space and light](painting/space.md) | Forms, worlds, views and depth restrictions | not started |
| [Painting time](painting/time.md) | Hand time, waiting and drying | not started |
| [Journal and inspection](session/journal.md) | Notes, status, globals and log | drafted |
| [Painter harness](session/harness.md) | Tools, image memory, pacing and compaction | not started |
| [Replay and delivery](delivery/replay.md) | Save, frames, checks, replay, finishing and export scripts | drafted |
| [Studio viewer](watching/studio.md) | Painter selection, live watching, replay and static export | not started |

[goal.md](goal.md) is the drafting contract. [glossary.md](glossary.md) owns terms. [verification](verification/README.md) records checks and their limits. [bug-triage.md](bug-triage.md) collects suspected defects without modifying the product.
