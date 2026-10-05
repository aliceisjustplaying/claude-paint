# The scratch canvas

A second canvas beside the painting for trials, so painters stop testing on the painting itself.
Agreed with Alice on 2026-10-05: a separate canvas with its own state, set up like the painting,
reached through the existing tools with `scratch: true`, at the full 2400px. It keeps **its own clock**
(a shared clock was considered and dropped: drying tests there, `glaze()`'s 8 to 29 days included,
would have aged the painting).

## What a painter gets

- `paint`, `look`, `status`, `log` take `scratch: true`; `paint` also takes `new_scratch: true`
  (with `scratch`). Replies on the scratch canvas start with `[scratch canvas]`.
- The first scratch chunk sets the scratch canvas up with the painting's own `canvas{...}`, written
  back by the easel (`Studio::canvas_src`, `api.rs` `lua_src`) and logged as the scratch log's
  chunk 1. Before the painting has a canvas, a scratch chunk is refused with a message.
- Its own Lua state, palette, brushes, knives, rags and log: `paintings/lua/scratch.lua`
  (session dir `out/easel/scratch/`). `new_scratch` renames the log to `scratch-N.lua` and starts again.
- Guide section: `notes/easel_guide.md`, "The scratch canvas".

## How it is built

- `crates/easel/src/main.rs`: the painting's server holds the scratch canvas as a second
  `Server` (`Server::scratch`), reopened from its log at startup (`resume_scratch`; a log that
  fails to reopen is put aside, never stopping the painting). `--scratch` routes `do`, `look`,
  `status`, `log`, `globals` and `save`; `do --scratch --new` starts a fresh one. A stale
  scratch state (failed chunk, see `session.rs` `stale`) is rebuilt at once.
- `harness/painter/easel-tools.ts`: the tool options.
- `studio/studio.py`, `studio/index.html`: paint events carry `scratch` (and `new_scratch`);
  `is_whole` excludes scratch looks (so the picker, replay steps and the gallery export,
  `export_static.py`, stay on the painting). While the painter's newest chunk or look is on the
  scratch canvas, the scratch canvas is the big picture and the painting a small one in the
  corner; otherwise the reverse. The code pane shows `scratch.lua` then; the clock and the palette
  panel follow the canvas in use.

## Checked

- Painter build (`--no-default-features`) in a throwaway studio: refusal before a canvas; the
  scratch ground pixel-identical to the painting's in an untouched crop; `wait(24*60)` on scratch
  left the painting at `day 1, 09:00` and `painting.lua` unchanged; status, log, palette look;
  `--new` put `scratch-1.lua` aside; a failed scratch chunk changed nothing; close and reopen
  replayed `scratch.lua` to a pixel-identical look.
- Harness tools loaded against pi's packages: the options, the `[scratch canvas]` tag, the
  `new_scratch` refusal, a scratch crop look.
- Viewer with a synthetic session (headless Chromium): the swap, captions, code file, clock.
- Test: `session.rs` `a_canvas_written_back_sets_up_the_same_canvas`.

## Before a round uses it

- The painter-facing text (tool descriptions in `easel-tools.ts`, the guide section) needs
  Alice's approval.
- The round's runner: `session_reviewed` (`notes/round29/runner/r21_chains.py:561`) resets on
  every `paint` result and counts every `look`. A scratch paint must not reset it and a scratch
  look must not count: skip calls whose arguments have `scratch: true`.
