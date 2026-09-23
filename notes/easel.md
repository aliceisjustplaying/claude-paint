# The easel: live Lua painting sessions

Stream: easel (branch `easel`). The painter's guide is `crates/easel/README.md`;
this note covers the design, how it felt to paint with it and what to do next.

## What landed

- `crates/easel`, binary `easel`: LuaJIT 2.1 embedded with mlua 0.10
  (`features = ["luajit", "vendored"]`; builds on macOS arm64 in about 20 s).
- A background session per painting, reached over a unix socket
  (`out/easel/<name>/sock`). The commands are `open`, `do`, `look`, `undo`,
  `log`, `status`, `save`, `frames`, `check` and `close`.
- `paintings/lua/<name>.lua` is always the session so far: the successful
  chunks in order, each after a `--@ chunk N · clock M` marker.
  `easel run <file> [--width 3200] [--crop ...]` replays it. At the session's
  width the PNG is byte-identical to `easel save` from the live session
  (checked with `cmp` on the example; also a unit test).
- A painter's API in Lua: canvas and palette, colors, masks with operators,
  brushes that keep their paint, `work` over every `Handling` option,
  `stipple`, `glaze`, `blend`, trees as Lua tables, noise, `dry`, `wait`,
  `varnish`, `cracks` and `relief`.
- `paintings/lua/example.lua`: a small study painted live (an evening sky, a
  distant ridge, mist and a spruce on a knoll) in ten chunks.
- Engine changes, all additive: `#[derive(Clone)]` on `Canvas`, `Wet` and
  `Held`, and `Canvas::seen()` (the dry picture plus the wet paint as laid,
  public so `look` can show wet paint).

## Design decisions

**A chunk is the unit of everything.** It is the unit of the log, of undo,
of randomness and of rollback. Before each chunk the session snapshots the
canvas (a clone, ≈ 50 MB at 1000px), a shallow copy of the Lua globals, the
contents of every live brush and the clock. A chunk that errors or panics is
restored from its snapshot and never logged. So a typo halfway through a
chunk can't leave half a passage on the canvas or a half-drained brush
behind. Undo pops snapshots (8 by default, `--undo N`).

**Determinism without the painter thinking about it.** `math.random`,
`rand` and `randn` are reseeded at the start of chunk N from
`(canvas seed, N)`. `work`, `stipple`, `brush` and `tree` draw automatic
seeds from `(seed, N, call number)`. A failed chunk therefore can't shift
later randomness, and replay needs nothing but the file. The width is not
in the file: the same program renders at 1000 or 3200.

**Lua closures stay off rayon threads.** Painter fields (color, angle,
coverage, `load_at`, glaze thickness) are sampled serially every 2 units
over the mask's bounding box, grown by the longest stroke. The engine
threads then read them bilinearly. Angles are interpolated as (cos, sin),
so a field can wrap through ±π. Mask functions run at every pixel. Measured
at 1000px: a per-pixel noise mask takes 0.10 s and a trivial one 0.03 s;
field grids take 0.01–0.28 s per call (the reply prints it as "Lua fields").
At 3200px masks cost about ten times as much, and the grids cost the same.

**Short verbs, tables for options, strict keys.** `work(mask, {...})`
takes every `Handling` option by its engine name, and presets come from the
style (`hand="broad"`). Unknown option names are errors that list the valid
ones. That caught my own `colr=` typo in the first minute.

**Masks in units.** The engine's `Mask::roughen` pushes mask *values*.
My first ridge used `roughen(1.5, ...)` meaning 1.5 units, and the mask
spread over the whole sky. At the easel `m:roughen(units, period, seed)`
now moves the edge by signed distance, in units. `m:soften(units)` was
added too.

**Time.** `wait(minutes)` advances a painting clock that is written into
every chunk marker. At 60 minutes or more it dries the canvas; below that
it only moves the clock. This is a placeholder until a real
open/tacky/touch-dry model exists. The log already carries the times, so
that model can be dropped in later.

**Memory.** Lua can't see how big a mask is (29 MB at 3200px), so a loop
building a foliage mask from 109 ribbons piled up gigabytes before the
garbage collector ran. The easel now counts the mask bytes it makes and
collects every 400 MB, and after every chunk. Peak RSS for the example at
3200px went from 6.8 GB to 1.9 GB. Replays keep no snapshots.

## How it felt at the easel

I painted `example.lua` as an agent would: one bash command per chunk,
`--look` on most of them, then reading the JPEG.

What felt good:
- **The loop is short enough to feel like painting.** Lay the sky
  (6 s), look, `wait(24*60)`, lay the far ridge (1.2 s), look, and so on.
  `do --look` makes painting and looking a single command. The look itself
  takes 0.04 s.
- **Mistakes are cheap.** I undid three times. The first time the ridge
  mask was wrong and the ridge covered the sky. The second time the mist
  left a gray fringe on the knoll's wet edge, which I fixed with
  `ridge - ground:grow(4)`. The third time the foliage was too blobby. Each
  undo and retry took about a second, with no recompiling or repainting of
  the sky.
- **Globals are the sketchbook.** I defined `knoll(x)`, `ridge` and
  `ground` once and reused them for the mist mask, the tree's footing and
  the grass. In Rust, closures and ownership made this kind of reuse
  painful (friction 10 in notes/amnesia2.md).
- **Mask arithmetic reads like intent.** `ridge - ground:grow(4)` says
  exactly what I meant.
- **The studio views pay off.** `--mode value,squint` showed the distant
  ridge almost the same value as the sky, which the color view hid.
  `--crop` enlarges by whole pixels, so I could see that the 1000px stipple
  is pixel grain.
- **The hand is physical.** Six strokes from one load of a round visibly
  run dry. A tree's 567 limbs were stroked one by one with dips, in 0.03 s.
- **Resume works.** `close` and `open` replay the log, which I needed after
  rebuilding the binary.

What felt awkward:
- **Presets don't size themselves to the motif.** The `hatch` preset's
  brush is about 5 units wide, too wide for spruce tiers, even with
  `tool="round 1.4"`. My spruce ended up a dark mass (fine for Friedrich at
  this scale, but not by choice). Detail still takes more words than broad
  work, as the fresh painters found.
- **The plain look shows wet paint as laid.** Thick wet stipple looks
  grainy and bright until it levels. `--dried` shows the truth but takes
  0.3 s. The better default is unclear.
- **The preview doesn't predict holes at 3200px.** Body passages at
  coverage 3 leave specks of ground in both renders, but at 3200px there
  are noticeably more of them (compare `out/lua/example_3200.png` with the
  1000px look). A painter at 1000px doesn't see what the full render will
  show.
- **Shell quoting.** Lua in single quotes means no `'` inside the chunk.
  Double quotes work everywhere, and `easel do - <<'EOF'` handles long
  chunks.
- **`local` in a chunk disappears after it,** which is Lua semantics.
  The README says so, but a painter may trip on it once.
- **Rollback is shallow for tables.** If a chunk edits a table created by
  an earlier chunk and then fails, the edit stays. `easel check` detects the
  divergence; I didn't hit it.

## Speed (release, 1000px unless noted; 10-core machine shared with four other agents)

| action | time |
|---|---|
| `easel open` (new session) | 0.1 s |
| `canvas{}` (priming the linen) | 3.1–3.3 s |
| broad sky, coverage 4.5, plus a blend | 6.0 s |
| body passage (ridge, knoll) | 1.2–1.6 s |
| stipple pass (mist) | 0.14–0.29 s |
| 567 limb strokes with dips | 0.03 s |
| hatch foliage over 109 ribbons | 0.8–1.0 s |
| `varnish(); relief()` | 0.30 s |
| `look`, plain | 0.02–0.06 s |
| `look --mode value,squint` | 0.08 s |
| `look --dried` | 0.30 s |
| `easel check` (replay and compare, 10 chunks) | 18 s |
| `easel run` at 1000px | 29 s and 38 s (two runs; the machine was busy) |
| `easel run --width 3200` | 106 s and 152 s; the sky chunk alone is 65 s, `canvas{}` 25 s |
| peak memory, `run --width 3200` | 1.9 GB (6.8 GB before the GC fix) |
| peak memory, `run` at 1000px | 0.5 GB |

Evidence: `out/lua/example_3200.png` (full render),
`out/easel/example/look-*.jpg` (the session's looks in order),
`paintings/lua/example.lua` (the session itself). `out/` is not committed:
rerun `easel run paintings/lua/example.lua --width 3200 --look`.

## Next

1. **Form.** Expose `Form`, `Sdf`, `Ridge`, `Light` and `Shade` (build a
   solid, light it, then query `shade(x, y)`, `fall(x, y)` and part
   masks). I left this out to land the rest.
2. **A drying model behind `wait`.** Open, tacky and touch-dry states per
   pixel from the film's age and medium, so wet-into-wet, scumbling on
   tacky paint and glazing on touch-dry paint follow from the clock.
3. **Detail at full resolution, live.** A session holding a `--crop`
   window at 3200px, resumed from a checkpoint of the chunks so far,
   would let a painter work Friedrich's small particulars at their real
   grain. The engine's Frame and crop machinery already supports it.
4. **Complete rollback.** Deep-snapshot tables reachable from globals,
   or rebuild the Lua state by replaying in the background after an undo.
5. **Motif helpers in Lua.** Tufts, needles and grass as small gesture
   libraries in Lua (`paintings/lua/lib/`), loaded by a sandboxed
   `use "trees"` whose text is inlined into the log so replay stays
   self-contained.
6. **Time-lapse.** `frames on` already writes one JPEG per chunk; add
   `easel frames --video` via ffmpeg.
