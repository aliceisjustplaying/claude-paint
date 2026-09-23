# Firs and fir woods grown into a drawn envelope (branch `tool-firs`)

## Why

Conifers were the most repeated defect in every critic round. Here are the
critics' words (notes/review_scores_loop2_raw.md, loop3_raw.md):
- "stamped spruces: symmetric chevron tiers with a white edge on every
  branch"
- "Friedrich built firs from short hatched strokes with irregular, specific
  growth"
- "the fir wood at upper left is a near-black, unmodulated silhouette
  block ... no aerial recession"
- "no trunks, gaps or tonal steps"

Painters hand-built a spruce every round (notes/amnesia4/easel4_near.lua
chunk 7, `function spruce(o)`), because `tree{habit="spruce"}` grows a forest
tree whose shape you can't choose. Friedrich painted firs in "short, hatched
strokes" [NG pp.49–50] (notes/research/friedrich_materials.md l.67, 118).

## What landed

- **`crates/paint/src/fir.rs`** (new, geometry only):
  - `Fir::grow(envelope, foot, &FirHabit, sun, seed)`. The leader runs
    foot to apex with some wander and sometimes an elbow (a lost top) or a
    dead spike.
  - Whorls of 3–6 boughs a year sit at random azimuths, seen in projection
    (a bough toward you is short). There are interwhorl boughs too. Each
    bough's reach is measured against the envelope at its own height, and
    the tip is refit to the envelope at the tip's height, so the tree grows
    *into* the drawn silhouette.
  - Boughs rise near the top, sag lower down and lift at the tip. Missing
    boughs come in runs (a noise down the crown), and some are broken. The
    lower boughs are dead, and stubs stay on the bare trunk.
  - Each live bough carries a needle pad, thin above and deep below, with
    tufts on old trees. Along each pad `lay_strokes` hangs short
    pointed-brush strokes, each with a `lit` value (sunward side, upper
    face, height, tip). These are the "short hatched strokes".
  - Masks: `needles` (the pads plus the strokes' fringe, so the edge is
    hatched), `wood`, `trunk`, `lit(from)`, `mask`.
  - Habits: `spire`, `old` (ragged), `young`, `storm`.
  - `Wood::grow(skyline, foot, &WoodSpec, seed)` places firs in rows. The
    front row's tops sit on the drawn skyline: most within a few percent,
    a quarter short, one in ten above. Each row back is `1 + r*recede`
    farther: smaller, feet converging on the horizon, more trees and
    `haze = 1 - exp(-r*air)`. Trunks stand bare under the crowns. Masks
    per row, plus `floor` (the dim band under the front crowns).
  - 3 unit tests: tips inside the envelope, deterministic, old vs young,
    and the wood recedes.
- **`crates/easel/src/draw_firs.rs`** (new): `fir{envelope=, foot=, habit=,
  seed=, sun=, ...overrides}` and `fir_wood{skyline=, foot=, depth=, count=,
  horizon=, recede=, air=, width=, mix=, bare=, sun=, seed=}`.
  - The envelope and skyline take an `outline{}` or points.
  - Fir values: `f.boughs`, `f.leader`, `f:needles()`, `f:lit()`,
    `f:shade()`, `f:gaps()`, `f:strokes{kind=, lit=, z=}` and
    `f:paint(brush, {color=, lit=, kind=, ...})`, which lays the strokes
    with the pointed brush pressed to each stroke's width.
  - Wood values: `wood:needles(r)`, `wood:all(r)`, `wood:visible(r)`,
    `wood:lit(r)`, `wood:floor()`, `wood:haze(r)`, `wood:scale(r)`,
    `wood:trees(r)` and `wood:paint(b, r, {...})`.
  - 2 tests.
  - Small shared touches: `draw_outline.rs` gains `outline_path()`,
    `api.rs` one install line, `main.rs` one `mod` line and `lib.rs`
    `pub mod fir`.
- **README** (crates/easel/README.md, "Firs and fir woods"): the API and a
  worked example that paints a fir and a wood with hatched strokes.
- **Sketchbook** section 5: a recipe ahead of the old hand-built spruce.
- **Study**: `paintings/lua/firs.lua` (6 chunks: sky, snow, a lone spire, a
  ragged old fir and a young fir, a wood edge of 4 rows).

`cargo test --workspace` passes. The golden fingerprint did not change
(crates/paint/tests untouched; nothing existing calls the new code).
Clippy shows no warnings in the new files.

## Evidence

- `notes/firs/firs_1000.jpg`: the whole study at 1000 px.
- `notes/firs/firs_3200_crop.jpg`: the crop 240,120–600,540 at 3200 (the
  wood's right half and the spire). `notes/firs/firs_3200_detail.jpg` is a
  detail of the wood's recession.
- Replay: `easel run paintings/lua/firs.lua` (about 25 s at 1000 px), or
  `--width 3200 --crop 240,120,600,540` (50–115 s).

Judged against the critics' words:
- **"Symmetric stacked tiers, a white edge on every tier, Christmas card":**
  mostly answered. The spire's tiers are uneven: a gap run a third of the
  way down, boughs of different reach, lower ones hanging with lifted tips.
  Its mass is hanging hatched sprays, not chevrons. At 3200 (the stem
  detail) the masses read as pendant shoots laid with a pointed brush.
  There is no white edge. The lit strokes are a few olive-gray ones on the
  sunward side.
- **"Short hatched strokes, each fir individual":** yes for the three
  single trees. The spire, the old fir (curtain branches off a crooked
  stem, dead twigs at the foot, a visible trunk) and the young fir (a dense
  cone to the snow) are clearly three different trees.
- **"One near-black block, no recession, no trunks or gaps":** partly
  answered. The wood now has a dark front row of individual hatched firs,
  paler grayer rows behind them seen in the gaps and the far rows as
  hatched tone. The crest is a varied line of tops, not a comb. The floor
  at the foot is still weak: a dim roughened band that reads more like a
  low hedge than trunks standing in shade. The front row's trunks barely
  show against it.

## Known issues and next steps

- **Sky holes along the stem.** Left and right pads don't always meet at
  the stem, so a column of small sky holes runs up the spire. At 1000 px
  it can read as a thin pale line. Next: put a narrow needle core along
  the leader inside the crown, broken by the gap runs.
- **Wood floor and trunks.** The floor mask is one band. Next: make it
  the region between trunks (front-row `f:trunk()` plus back-row feet),
  darken it upward under the crowns and let a few front trunks catch light.
- **Wood-tree tops.** At 3200 the tops of wood trees look a little
  spiky and leaf-like (the top bough's strokes fan out like a star). Next:
  shorter, more vertical strokes in the top two tiers.
- **Made-up envelopes all look alike.** `envelope_for` gives spires of one
  family. A painter-drawn skyline plus `width=` varies them, but a
  per-tree lean and notch would help more.
- **Snow.** Nothing puts snow on the boughs yet. The pads' upper faces
  (`b.pad[k][1]` above the bough line) are where it would lie. A
  `f:tops()` mask would make that one line.
- **Wood cost.** `wood.rows` builds every tree's Lua table. Inside a color
  function it cost 14.6 s of Lua fields in the first draft. Use
  `wood:haze(r)` and `wood:trees(r)` (documented in the README and the
  sketchbook).
- **Storm habit untested in a painting.** It exists with unit coverage of
  the shared code only.
