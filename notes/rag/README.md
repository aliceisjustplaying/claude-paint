# The rag

A verb for lifting open paint with a soft cloth bunched into a pad
(`crates/paint/src/rag.rs`, Lua in `crates/easel/src/draw_rag.rs`, the
guide's "The rag"). From the review's tools table: "Rag on open paint: one
broad soft contact on `Surf::take`; hand time in `tally`; replaces the 29
striped brush-wipes" of the round 22.1 Inness painter.

```lua
r = rag()                                -- pad about 40 mm across; rag{width=<units>, seed=}
r:wipe(m, {pressure=0.5, angle=0, passes=1, refold=0.5})   -- over a mask
r:wipe(pts, {pressure={0.4, 0.8}})       -- one wipe along a path
r:blot(x, y, {pressure=0.6})
r:refold()                               -- a cleaner, dry face
r:dip(0.5)                               -- the face in use dipped in spirits
r.load, r.soaked, r.damp, r.fold, r.width   -- read only (a write is an error)
```

## How it works

- One contact per step of the pad along its path (a quarter of its width):
  the cloth bridges the surface's local peaks (ground, set paint and the wet
  film on it, within 0.7 mm) and sags 15 to 105 µm below them with pressure.
  Wet paint above that level is in reach; a quarter of what is below it
  wicks up anyway. So the tops lose more than the hollows, and pressing
  harder reaches deeper.
- The cloth's creases (3 mm apart across the path, shifting every 25 mm
  along it) press from 0.4 to 1 of full contact; the pad's edge is soft and
  its width and line wander. A blot is a crumpled patch with a lumpy edge.
- Lift is a share of the paint in reach, scaled by how fluid the paint is:
  the square root of `drying::fluid` (the brush's own curve). It is 1 for
  fresh paint and 0 at the gel point. Past the gel point the film is no
  longer in the wet layer (drying.rs bakes it), so the rag can't touch it.
  A stain of 0.04 to 0.08 coats always stays.
- What it lifts loads the face in use (one face holds a 400 µm film over
  the pad's area); a loaded face lifts less (`1 - load²`). A refold turns a
  cleaner face out, but no cleaner than the whole cloth's soak (12 faces).
  Nothing goes back on the canvas.
- A face dipped in spirits (`r:dip(amount)`) lifts wet paint more
  readily: its lift rate is `1 + 8 × amount` times a dry face's. That is
  its only effect. It stays damp until a refold turns out a dry face or
  the spirits evaporate: half of what is left goes every 3 minutes of
  painting time (the clock, hand time included), dry some 17 minutes after
  a dip at 0.5 (an estimate; Jennings, Paint & Colour Mixing, 1902, "To
  Test the Purity of Turpentine": a few drops on paper "evaporate in a few
  minutes", https://www.gutenberg.org/cache/epub/56738/pg56738-images.html).
  Added
  because both sources wipe the lights with a rag dipped in spirits. It
  needs one more number on the rag (`damp`, the face in use): `soaked` is
  the whole cloth's soak and sets how clean a refolded face can be, so it
  can't also say that this face is wet.
- The rag is a userdata over state the session holds, rolled back with the
  brushes after a failed chunk; its numbers can be read, not set. No canvas
  state, checkpoint field or save-file field is new (the save file,
  save.rs, holds no brushes either, so no rags); the replay state digest
  (`easel run --state-digest`) hashes held rags after the brushes, and logs
  without a rag hash as before.
- Hand time (`paint::rag::pace`): a wipe is a stroke's Fitts aim (the
  sourced constants in `tally::pace`) plus its length at 150 mm/s; a blot
  a touch plus 0.6 s; a refold 3 s; a dip 2.5 s; a fresh rag 5 s. A wipe
  over a mask ages the paint in 15-minute slices, as `work` does.

## Every parameter is an estimate

No number in the rag is measured or taken from a source. Each is marked
[E] in `crates/paint/src/rag.rs`: the pad width (40 mm), the lift rate,
the blot's share, the stain (0.04 coats), the bridge (0.7 mm), the sag
(60 µm), the wick (a quarter), a face's capacity (400 µm over the pad),
the 12 faces, the crease and shift spacings, the damp face's lift factor
(8, set so the spirits check below passes),
the drag speed (150 mm/s), and the blot,
refold, dip and fresh-rag times. The only sourced constants it uses are
the Fitts and steering numbers already in `tally::pace`, and the drying
model's curve (`drying::fluid`, itself built on estimates).

The tables below show what those estimates give. They don't show that
the estimates are right.

## Checks against what painters describe

Sources: R. Palesca, "How to create a wipe-out underpainting in oil"
(https://rpalescafineart.com/blogs/blog/how-to-create-a-wipe-out-underpainting-in-oil),
and S. Downing-White, 3rd handout, underpainting method 2
(https://susandowningwhiteclasses.com/3rd-handout-underpainting-method-2.html).
Only search excerpts of the pages were read, not their photos.

| what painters describe | test | result with the estimates |
|---|---|---|
| A clean dry rag "should wipe away the paint and leave a pale tint on the canvas" (Downing-White) | `a_dry_rag_leaves_a_pale_tint_and_spirits_lift_nearly_to_the_ground` | over a thin umber tone, 3 times by hand at pressure 0.8 (a cleaner face whenever one is half loaded): 26% of the tone's color stays (OKLab distance from the bare ground); film off the tops 98%, off the hollows 88% |
| A rag dipped in spirits lifts the lights nearly to the ground: most of the film gone from the weave tops, a faint stain in the hollows (Palesca: "dip the rag into OMS"; Downing-White: "a light dip in OMS to reveal the lightest values") | the same | dipped at 0.5 and re-dipped at each refold, 3 times: film off the tops 99%, off the hollows 97% (asked: 95% of each, and more than the dry rag from the hollows) |
| More comes off the weave tops than the hollows | `the_rag_lifts_open_paint_from_the_tops_first` | thin film, 1 pass: 62% off the tops, 38% from the hollows |
| A loaded cloth lifts less until refolded | the same | a loaded face's second pass lifts less than a refolded face's |
| Nothing lifts past the gel point | `nothing_is_lifted_past_the_gel_point` | dry and damp rag, wipes, blots and pressure 1: the canvas's state is bit for bit unchanged |

The thresholds are my reading of the sources' words ("a pale tint", "a
faint stain"), not numbers from them. The tests age paint by its state
(its cure as a share of the gel point, or until no wet paint is left),
never by hours, so they hold if the drying model's times change.

**What the spirits don't reach: the color.** With spirits the film is
nearly all gone, but what the eye sees falls only from 26% of the tone's
color (dry) to 14% (spirits). Raising the spirits' lift factor doesn't
help: 1 gives 20%, 4 gives 15%, 12 gives 14%. The cause is the stain the
cloth can't take (`STAIN_COATS`, 0.04 coats on the tops and 0.08 in the
hollows), which is the same dry or damp. A film of raw umber that thin
still shows. The spirits rag may only change lift strength, so it can't
take the stain. Making it do so would take a second effect (spirits
dissolving the stain), and it wasn't added. The real calibration is by
eye: `rag_wipe_out.jpg` beside the handouts' photos.

## Numbers

`cargo test -p paint --lib rag::tests::table -- --ignored --nocapture`: a
440 mm canvas on 15-thread linen with one 25 µm brushed ground (the weave
shows), at the live width (2400 px; a crop window around the patch). A sky
film is laid with a filbert 22, then a 176 × 97 mm patch is wiped, pressure
0.5, a clean face each pass (refolding inside a pass at load 0.5). The
share of the film removed is measured over the patch's middle. "Tops" and
"hollows" are the upper and lower thirds of the surface's contact level.

Thin sky film (2.7 coats, about 70 µm), share removed: tops / hollows / all.

| paint | 1 pass | 2 passes | 3 passes |
|---|---|---|---|
| fresh | 62 / 38 / 48% | 92 / 65 / 78% | 97 / 79 / 88% |
| setting (60% of the way to the gel point) | 47 / 28 / 36% | 82 / 51 / 65% | 94 / 66 / 79% |
| near the gel point (90%) | 7 / 7 / 7% | 16 / 15 / 15% | 23 / 21 / 22% |
| past the gel point | 0 / 0 / 0% | 0 / 0 / 0% | 0 / 0 / 0% |

Thick film (12.9 coats, about 320 µm):

| paint | 1 pass | 2 passes | 3 passes |
|---|---|---|---|
| fresh | 37 / 35 / 36% | 66 / 61 / 63% | 83 / 75 / 79% |
| setting (60%) | 26 / 25 / 25% | 52 / 49 / 50% | 69 / 63 / 65% |
| near the gel point (90%) | 3 / 7 / 6% | 6 / 14 / 11% | 9 / 19 / 16% |
| past the gel point | 0 / 0 / 0% | 0 / 0 / 0% | 0 / 0 / 0% |

Near the gel point of the thick film, the hollows lose more than the tops:
the film over the thread tops is thinner, so it gels first.

Pressure, one pass over fresh thin film: 0.2 gives 42 / 35 / 37%; 0.8 gives
83 / 52 / 68%.

Today's brush-wipe (a flat 16 wiped before each of 900 short strokes, the
round 22.1 painter's loop) over fresh paint: 96 / 94 / 95% (thin), 95 / 95 /
95% (thick), in 36 minutes of hand time. The rag: 4 to 35 s a pass.

## Pictures

- `rag_wipe_out.jpg` (`rag_wipe_out.lua`): a wipe-out on a thin raw umber
  tone, as the two sources describe it: the tone evened with a light rag;
  test panels wiped by hand in rows, a cleaner part turned out whenever
  one is half loaded: a clean dry rag (3 times) and a rag dipped in spirits
  (3 times, re-dipped at each refold) then a dry part; a ball with the half-lights wiped dry, the light wiped
  with spirits and a blot. The tone is oil paint with medium (the easel
  has no solvent in paint yet), so it is fatter and blotchier than the
  handouts' tone thinned with spirits.
- `rag_vs_brush_wipe.jpg` (`rag_vs_brush_wipe.lua`): a wet sky with one
  cloud wiped out by the rag (left) and one by the brush-wipe loop (right),
  whole and at full detail.
- `rag_passes_and_drying.jpg` (`rag_passes_and_drying.lua`): 1, 2 and 3
  passes over three bands painted at different times. The log waits by
  state, not by hours (band 1 until `drying()` says tacky or dry at every
  test patch; band 2 until most are setting or any passes the gel point),
  and the caption gives each patch's `drying()` as the run printed it.
  This render: band 1 tacky at all three, band 2 setting, setting and
  open, band 3 open.

Rendered with `easel run <file> --out <png>` at 2400 px, then scaled.

## Against notes/principles.md

- A physical capability, not a result: the painter chooses where, how hard,
  how often and when to refold; what comes off follows from the paint's
  thickness, drying and the weave.
- A person has it: Friedrich's process note lists "rag or finger lifting
  open paint" (notes/research/friedrich_process.md), and principles.md's
  backlog has it as P3.
- Defaults to the painter deciding: nothing runs unless called.
- Deterministic and physical: seeded from the chunk like every verb; it
  removes paint (into the cloth) and takes hand time; it can't reach set
  paint, and a clean cloth costs a refold or a fresh rag.

## Finding 5 (`blend` is a remover)

Confirmed in the code: `Style::blend()` dips every 3 strokes with load 0
and wipe 0.9 (`crates/paint/src/style.rs:361`), and on each dip
`handling.rs:795` runs `held.wipe(0.9)`. Measured with the same patch:
`blend()` removes 15 / 29 / 40% of fresh thin film in 1 / 2 / 3 passes (9 /
19 / 26% of the thick film), evenly from tops and hollows. Blend wasn't
changed.

## Old logs: engine 3

The rag exists only in engine-3 sessions (`paint::ENGINE` is now 3; a log
records its engine as `--@ engine N`, and a log without the line is
engine 1). An older log replays without the global `rag`, so it sees
exactly the globals it saw. Engine 3 changes nothing else here (the
physics checks are all `engine >= 2`). The drying and repaints branches
also raise the engine to 3, so the merges meet on the same number.
The `.lua` sheets here start with `--@ engine 3`.

Why: before the gate, two old paintings printed something different on
replay, though their pictures were byte for byte the same. Both walk
`pairs(_G)`: paint-studio-58dfc6 (chunk 288) printed 358 globals instead
of 357, and paint-studio-520026 (line 2526) printed its piles in a
different order (one more key in `_G` changed Lua's hash order). Three
other logs define their own global named `rag` (7ff345 and 496bb9 a
brush, 3a958c a mask). Assigning a global replaces the easel's, so they
were never affected.

Checked: six old paintings (paint-studio-7ff345, -496bb9, -33a5bf (the
round 22.1 Inness painting), -520026, -58dfc6 and -3a958c) replayed with
the round-23 easel and with this branch's (`easel run <log> --width 300`):
the same PNG bytes and the same printed output. The test
`an_older_log_sees_the_globals_it_saw` pins an engine-2 session's
`pairs(_G)` (69 names and their order hash, printed by the round-23 easel)
and fails without the gate.
