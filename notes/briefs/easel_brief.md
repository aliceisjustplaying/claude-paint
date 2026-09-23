# Easel: a live Lua painting session for agents

## Why
claude-paint is "a painting app for agents": the engine (Rust, crates/paint)
provides physics and tools; paintings today are Rust programs
(paintings/src/bin/*.rs) that are compiled and run top to bottom (or resumed
at a stage). Three painters last night lost time to Rust itself (closures,
ownership, `per_column` not Copy; notes/amnesia2.md friction 10) and to the
write-compile-run loop. The user wants the painting experience to feel more
like painting: an easel where the canvas stays alive, the agent makes marks,
steps back and looks, makes more. Think Shader Showdown live coding.

Decision (user): **Lua**. A REPL-like live session, and the final painting is
a Lua file that replays the session. The most important question is how it
FEELS FOR THE AGENT, so optimize for an AI agent driving it through a shell
tool (one bash command per action; it reads images with a file-read tool).

## What to build (new crate `crates/easel`, binary `easel`)
1. **Lua embedding** with mlua 0.12: **Lua 5.5** (`mlua = { version = "0.12",
   features = ["lua55", "vendored"] }`; Lua 5.5.1 is the latest release, user's
   choice). Painter callbacks are sampled onto rasters anyway (below), so
   LuaJIT's speed isn't needed. Deterministic: replace/seed `math.random` from
   the engine Rng; no wall-clock or OS access needed by paintings.
2. **A painter-shaped Lua API** over the engine. Idiomatic Lua: tables for
   options, short verbs, units = canvas units (1000 wide). Cover what a
   painter needs; read notes/{color,strokes,stipple,form,motifs,workflow}.md
   and README.md for the engine API. At minimum:
   - canvas: `canvas{style="friedrich", aspect=1.4, seed=1}` (Style::prepare),
     palettes (`pal:mix`, `pal:aim`, `pal:only{...}`, medium), `Paint`.
   - the hand: take a brush (kinds and widths), load/wipe/reload, a stroke
     along points with pressure/ramps, a touch (dab), several strokes in a
     row with the same load (the brush runs dry naturally).
   - covering areas: `work(mask, {tool=..., length=..., curve=..., cross=...,
     order=..., color=function(x,y) ... end, ...})` mapping onto Handling;
     `stipple(mask, {...})`; `glaze`; `blend`.
   - masks: from function, from shapes (polygon, ellipse, ribbon), ops
     (blur, offset, signed distance, band, union/intersect/subtract).
   - noise (Fbm), growth skeletons (grow a tree, get limbs as Lua tables:
     the painter paints them), form (Sdf/Ridge solids, Light, Shade queries).
   - `dry()`; finishing (varnish, cracks, relief) as explicit verbs.
   Lua callbacks can't be called from rayon threads (Lua state isn't Sync):
   evaluate painter functions (color fields, angle fields, mask functions)
   serially or sample them onto rasters/grids first; measure the cost at 1000px.
3. **Session mode** that feels like an easel:
   - `easel open <name> [--width 1000]` starts (or reattaches to) a
     background session holding the live canvas in memory (unix socket or
     similar; the agent launches it once).
   - `easel do '<lua>'` / `easel do -f chunk.lua` runs a chunk in the live
     session and prints its output/errors (errors don't corrupt the canvas:
     a failed chunk is rolled back).
   - `easel look [--crop x0,y0,x1,y1] [--mode normal|value|squint|mirror]
     [--wet]` writes a small JPEG (≤1000px, ~100–250 KB) and prints its path,
     so the agent can view it. Studio tools: value (grayscale) view, squint
     (blur) view, mirror view; showing wet paint as it would look.
   - `easel undo [n]` reverts the last chunk(s) (snapshots in memory, bounded).
   - `easel log` shows the session so far; the session log is always saved to
     `paintings/lua/<name>.lua`: exactly the successful chunks, in order, as a
     replayable program (with the canvas/seed header).
   - `easel close`.
4. **Replay**: `easel run paintings/lua/<name>.lua [--width 3200] [--out path]
   [--crop ...]` repaints the file from scratch, byte-identical to the live
   session at the same width. At 3200px it's the full render.
5. **Time** (stretch, design for it): the user loves "let it dry for 10
   minutes". Add a `wait(minutes)` verb to the API now; for this prototype it
   can map onto the existing all-or-nothing `dry()` above a threshold and be
   a no-op below it, clearly documented as a placeholder: a real
   open/tacky/touch-dry model is a later engine stream. Design the session so
   time is a first-class part of the log.
6. **Frames** (stretch): `easel frames on` makes every chunk also save a
   frame, so a session can become a time-lapse.

## Docs
Write `crates/easel/README.md`: the painter's guide to the easel (how to open
a session, the loop, the API reference with short examples). It must be
self-sufficient for a fresh agent with no other context besides the
research notes. Add a sample session `paintings/lua/example.lua` (a small
study, not a Friedrich copy) that replays.

## Then test the feel yourself
Once it works, paint a small study live through the easel (a sky, a
distance, a foreground detail), the way an agent would: one command at a
time, looking often. Write `notes/easel.md`: design, API decisions, what
felt good and what felt awkward as the agent at the easel, speed numbers
(chunk latency, look latency, replay time at 1000/3200), and what to do next.
