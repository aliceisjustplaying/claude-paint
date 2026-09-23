# The easel: live Lua painting sessions

Stream: easel (branch `easel`). The painter's guide is `crates/easel/README.md`;
this note covers the design, how it felt to paint with it and what to do next.

## What landed

- `crates/easel`, binary `easel`: first LuaJIT 2.1 with mlua 0.10; since
  round 2, Lua 5.5.1 with mlua 0.12 (`features = ["lua55", "vendored"]`),
  built with a fixed hash seed (see Round 2).
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

## Round 2 (integrator follow-ups)

**Lua 5.5.1.** The easel now embeds Lua 5.5.1 through mlua 0.12
(`features = ["lua55", "vendored"]`, lua-src 551.0.2). What changed for
painters:
- integer and float subtypes (`W` and `H` are integers now, so `print(H)`
  says 714);
- no `bit` library (the operators are built in) and no global `unpack`;
- loop variables are read-only;
- one `global` declaration makes its whole chunk strict. The guide says to
  use plain assignment.

The port found a determinism trap. Lua 5.4+ seeds its string hash per
state, and on macOS mlua 0.12 passes `arc4random()` as that seed
(`mlua-sys/src/lua55/lauxlib.rs`, `luaL_makeseed`). So `pairs` order
changed from process to process: the same probe table walked in three
different orders in three runs. `table.sort` also randomizes its pivots
from Lua's own `luaL_makeseed`. The fix has two parts:
- Lua is built with `CFLAGS=-Dluai_makeseed()=0x5eedu` (`.cargo/config.toml`;
  only lua-src compiles C in this workspace);
- the easel creates its states with C's `luaL_newstate` and hands them to
  mlua (`Lua::get_or_init_from_ptr`), closing them itself on drop.

`Session::new` compares a probe order against the recorded one and refuses
to run if the build lost the flag. With the fix, a session resumed from the
log in one process and `easel run` in another wrote byte-identical PNGs of
the example, and `easel check` matched.

Speed at 1000px (same machine, both binaries run back to back):

| | LuaJIT 2.1 | Lua 5.5.1 |
|---|---|---|
| per-pixel mask calling a noise userdata | 0.12–0.23 s | 0.24 s |
| per-pixel plain mask | 0.06–0.09 s | 0.06 s |
| whole-canvas field grid (glaze thickness) | 0.03–0.04 s | 0.03 s |
| 10⁷ iterations of `s = s + math.sin(i*0.001)*0.5` | 0.04 s | 0.59–0.72 s |

Callbacks cost about the same under both, because the Rust/Lua boundary
dominates. Pure Lua arithmetic is about 15× slower without the JIT, which
matters only for painters who compute heavily in Lua.

**Merged main.** The engine's `Mask::roughen` is now in units, so the
easel's workaround is gone and `m:roughen(units, period, seed, edge)` calls
it. `cracks{}` follows `Cracks::aged`: the island size, ground and opening
are fitted to the canvas unless given, and `vary` and `veil` are new.

**Form.** The Lua surface is described in the README; everything in it is
immutable, so rollback never needs to repair it:
- `body.ellipsoid`/`block`/`half_space` with `:turn :cut :rough :facet`,
  `+` and `-`;
- `ridge{}`, over a crest function or points;
- `terrain{}` (the engine's `Relief`), a Lua height function sampled on a grid;
- `form{ {solid, dist=}, ..., light={...} }` builds and lights the depth
  buffer in one call;
- queries: `sample`, `shade`, `value`, `lit_at`, `part`, `dist`, `fall`,
  `across`, `bend`, `edge_angle`;
- masks: `parts_mask`, `lit`, `shadow`, `silhouette{soft, haze}`,
  `edges{concave}` and `mask(fn(s))`;
- `f:field("fall"|"across"|"edge")`, which `work{angle=}` reads natively on
  the engine's threads, with no grid.

`paintings/lua/rocks.lua` is the demo: a boulder and a range painted by
light and shadow families, strokes down the fall lines and dark accents in
the boulder's concave breaks. One thing a painter will trip on: a `ridge`
leans toward the viewer at its foot, so it hides a rock in front of it
unless its `z0` is set back. My first try did exactly that; the README
now says so.

**Exact rollback.** The shallow copy of the globals is replaced by
`heap.lua`. Before each chunk it walks everything reachable from `_G` and
the string metatable: tables (keys, values, metatables) and the upvalues of
Lua functions. It records each table's contents and each function's
upvalues. On failure or undo, it writes back only the tables whose
contents changed, in place (so object identity holds), and resets the
upvalues. Anything the failed chunk created becomes unreachable. It uses a
private copy of the `debug` library, taken out of the globals. Replaying
the log into a fresh VM with painting as no-ops was not sound: verbs return
values chunks branch on (`sample`, `b:fullness()`, trees).

Cost, measured with `status`: about 1 µs per live table per chunk. The
example session (a tree's roughly 4,000 limb tables) spent 0.02 s on
bookkeeping over its ten chunks, and rolling back a chunk that edited a
limb took about 0.01 s. With 100,000 small tables, a snapshot takes about
0.09 s, and a snapshot plus restore about 0.13 s. Memory is one shallow
copy of each table per undo level.

Tests cover a failing chunk that mutates old tables, an upvalue, a
metatable and `string`; undo of a table edit and an upvalue bump; and exact
replay afterwards. The remaining limits are coroutines suspended across
chunks, and `pairs` order over a table that was rewritten during a rollback.

**Coverage (not this stream).** The example and rocks renders show bare
ground flecks: orange in the skies, pale in the dark knoll. There are more
of them at 3200px, and the ridge strokes look blocky. The fixes-paint
stream is working on this; the easel only passes `coverage` through.

**Second merge of main (drying, scene).** It was clean apart from one import
my `Canvas::seen` needed. `wait(minutes)` is now the engine's
`Canvas::wait`, so it's no longer a placeholder. `dry()` and `wait` return
the clock, and `clock()` counts from `canvas{}` (the canvas's own clock
also includes the weeks its grounds dried). The new verb `drying(x, y)`
reports the stage. The merge also exposed a name clash I had made: the form
module's `relief{}` solid builder had replaced the finishing verb
`relief()`, and the example's last chunk failed on replay. The builder is
now `terrain{}`, and a test runs the finishing verbs. The example and rocks
renders change with the new engine (drying levels films differently), but
each still replays byte-identically: live `save` and `easel run` gave
identical PNGs, and `check` matched.

## Round 3: the whole engine in Lua

After merging main (tip, fixes-paint, green, atmosphere, scene, drying),
the easel binds every new stream, keeping everything painter-shaped and
immutable:
- **Scene** (`crates/easel/src/world.rs`):
  - `world{horizon, eye, fov, sun={azimuth, elevation}, ground=fn(X, Z),
    water={level, ripple}}`, with spots (`w:spot`, `w:spot_at`,
    `s:p/m/size`) and perspective helpers (`height`, `to_ground`,
    `project`, `scale_at`, `aerial`, `shadow_angle`, `sun_canvas`,
    `ribbon`, `line`, `recede`);
  - `w:place` and `w:proxy` return a new world (it is kept as a recipe and
    rebuilt, which is cheap until `w:view()` traces it);
  - `v = w:view()` gives `at`, `mirror`, `sky`/`land`/`water`/`shadows`/
    `contact`/`reflections`/`bodies_mask` masks, and `v.form`, which is
    the whole form API over the view's bodies. `FormU` now holds any
    "form holder", so a view's form reuses every form method.
- **Air:**
  - `w:sky{}` and `w:clouds{}` can be passed straight to `color=`: the
    engine's threads read them natively, with no grid;
  - clouds also give `alpha`, `lit`, `glow`, `soft` and
    `mask{alpha, lit, shade}`;
  - `haze{}`, and `w:ranges{}` layers with `crest`, `mask`, `haze`, `z_at`
    and `ridge` (a solid for `form{}`).
- **Noise:** `noise{kind=ridged|billow, warp, stretch}` (the plain
  `noise{}` stays the same `Fbm`, so older sessions replay unchanged),
  `worley{}` and `uneven()`. A noise passed to `coverage=`/`load_at=` is
  read natively.
- **Growth:** beech, alder and willow habits, `years`, and
  `t:foliage{sun, winter, leafing overrides}`, which returns clumps back
  to front plus `mask`, `lit`, `gaps` and `envelope`; `sward{}` returns
  tufts with their blades and flowers.
- **Color:**
  - `canvas{palette=...}` and `palette(name)` for the greens palettes, and
    `pal:with{...}`;
  - `shift()`;
  - `color_over` in `work` and `stipple`: `{shift={dL, da, db}}` is native
    (relative to whatever each stroke lands on), and `function(x, y,
    under)` is sampled on the 2-unit grid with `under` read from the
    canvas before the pass;
  - `hug`, stipple `fade`.
- **Pointed tips:** the `point=` tool option, `b:mark_width(p)` and
  `b:pressure_for(w)`.

One fix on the way: the field grid used to ask painter functions about
the canvas's own edge (x = 1000). A world's `to_ground` returns nil there,
so strokes centered at the edge took the "far away" color, which showed as
a pale band along the meadow's right side and bottom. Edge nodes now
sample just inside the canvas.

**The meadow (`paintings/lua/meadow.lua`), judged.** I painted it live in
7 chunks. What works:
- the sky is the world's own (Rayleigh blue over a paler horizon);
- the ranges are hazed from their distance;
- the ground pales into air toward the horizon;
- the beech is grown and leafed with its lit side toward the sun, sits in
  the world's perspective (18 m tall at 82 m) and casts a proxy shadow on
  the grass;
- about 36,000 grass blades are stroked with a pointed rigger and recede
  in scale.

What doesn't, yet:
- the clouds are soft veils, because broad strokes and the blender melt
  them; they need a second pass of light touches on `cl:mask{lit=...}`;
- the grass is even, with no lush and thin patches in light and shade (use
  the sward's `patch` and a value pass);
- flowers are too small to read at 1000px;
- the trunk barely shows;
- the lower crown shows the beech's level spray layering a little too
  regularly;
- bare-ground flecks remain (the coverage issue).

**At 3200px** (`easel run paintings/lua/meadow.lua --width 3200`, 114 s)
the meadow reads much sparser than in the 1000px preview. Tufts are laid
out in canvas units, so there are as many at both sizes, but a
pointed-rigger blade is a hairline a few pixels wide at 3200. The preview
promises a denser sward than the full render delivers. A painter should
check a `--crop` at 3200 before trusting the preview; the motif side
could scale `spacing` or blade width with resolution. The hazed horizon
band also shows dotted strokes at full size, and the bare-ground flecks
are larger.

**Replay exactness after the merge:** example, rocks and meadow are each
byte-identical between a live session resumed from its log and
`easel run` in another process, at 1000px and at 3200px. At 3200px the
live resumes took 81, 60 and 208 s and the runs 87, 92 and 114 s.

Two feel notes. The perspective helpers catch scale mistakes at once: my
first beech spot was 11.5 m away, where 18 m is 1,671 units, and `print(s)`
showed it before anything was painted. And `color_over` with a b shift
turns a green shadow teal quickly; −0.012 is plenty.

**Costs:**
- `w:view()` 0.3–0.5 s, `w:sky{}` about 0.2 s, clouds 1–4 s;
- the meadow's tufts (about 45,000 tables in globals) cost 0.1 s of
  rollback bookkeeping per chunk, reaching 0.7 s by the last chunk; keep
  big lists `local` when later chunks don't need them.

## Round 3 review fixes

The four findings in notes/review3/r3_review_easel.md are fixed. Each one has
a regression test that failed before its fix and passes after it.

1. **`--undo 0` disabled failure rollback.** `Session::run` used undo depth 0
   to recognize a disposable replay, so a live `--undo 0` session kept a
   failed chunk's paint and globals. Replays are now `Session::replay(width)`
   (used by `easel run` and `check`). A live session always snapshots before a
   chunk and keeps that snapshot only if `undo_depth > 0`. Test:
   `live_session_without_undo_still_rolls_back`.
2. **Object-keyed `pairs` differed between processes.** Lua hashes tables,
   functions and userdata by address. That order changes from process to
   process, and it also moves string keys that collide with those objects.
   The fix has two parts:
   - The easel creates its Lua state with its own allocator
     (`lua_newstate(counting_alloc, …)`), which writes a 16-byte header before
     every block. For tables and closures, the header holds a creation serial.
     Userdata and threads also get a serial, and because their pointers lie
     inside the block, they are also kept in a small address map.
   - `prelude.lua` replaces `pairs` and `next`. A table keyed only by strings,
     numbers and booleans still walks in Lua's own order (the C `next`), so
     existing paintings are unchanged. A table with any other key walks in a
     fixed order: booleans, numbers, strings, objects by serial, then library
     C functions by name. `next` caches each table's order for the length of
     a traversal, and the cache is cleared before every chunk. Heap snapshots
     skip the cache.

   Only relative serial order matters. Objects created by failed chunks, by
   snapshots or by mlua shift the numbers but never reorder the objects a
   successful chunk created. Tests: `object_keys_walk_in_creation_order`
   (two sessions in one process) and `tests/determinism.rs` (three `easel
   run` processes, identical output and PNG bytes).
   `plain_tables_keep_lua_order` checks the fast path against stock Lua
   with the same seed.
   Cost: an allocation-only microbenchmark (200,000 kept tables, 1 million
   temporary tables and closures, 300,000 strings) took 0.29–0.33 s against
   0.25–0.27 s with Lua's allocator. My first version kept every object in a
   `BTreeMap` and took 0.6 s.
3. **`string.gmatch` state lived in C.** `prelude.lua` reimplements it on top
   of `string.find`, with its position in upvalues, which the heap snapshot
   restores. It follows 5.5's `gmatch_aux`: a leading `^` is literal, an
   empty match can't end where the last match ended, and `init` is handled
   the way `posrelatI` handles it, including values past the end. Tests:
   `gmatch_iterators_roll_back` (a failure and an undo) and `gmatch_matches_lua`
   (15 cases against stock Lua). No other library in the sandbox keeps state
   in C that a painter can hold onto: `ipairs`, `next` and `utf8.codes` are
   stateless, `math.random` is reseeded every chunk, and coroutine, io and os
   aren't loaded.
4. **A view's form counted proxies.** `ViewBox` now counts visible bodies,
   matching `View::new`, which only numbers those. Test:
   `view_form_counts_visible_parts_only` (a proxy-only world and a
   visible/proxy/visible world).

Evidence (scratch dir `~/tmp/fix3-easel-0f194e93`): `repros.log`
reruns the reviewer's `--undo 0` and gmatch sessions. The failed glaze is gone,
`print(a)` gives 1, the iterator gives `red` again and `check` matches. In
`check1000.log` (first allocator) and `check1000b.log` (final), example, rocks and meadow are resumed live at 1000px and
`easel check` reports "replay matches the live canvas exactly" for all three.
Each live `save` is byte-identical to `easel run`.

Remaining limits: coroutines; a string-keyed table rewritten by a rollback;
a table that held object keys and lost them (its layout keeps the old
collisions until it rehashes).

## Next

1. **Rollback cost for big heaps.** Snapshot only tables that changed since the last chunk (a write barrier via proxies), or treat tables produced by `sward{}`/`tree{}` as frozen.
2. **Form, deeper.** `Form::simplify` (the painter's squint over normals),
   lit-rim helpers for contre-jour, and a gallery of `rocks.rs`-style
   motifs rewritten in Lua.
3. **Drying, felt.** `wait` now runs the engine's drying model (merged
   from main at the end of round 2) and `drying(x, y)` reports the stage;
   the looks could show it (a `--mode drying` map of open/tacky/dry).
4. **Detail at full resolution, live.** A session holding a `--crop`
   window at 3200px, resumed from a checkpoint of the chunks so far,
   would let a painter work Friedrich's small particulars at their real
   grain. The engine's Frame and crop machinery already supports it.
5. **Rollback, last gaps.** Coroutines; a restore that rebuilds changed
   string-keyed tables in their original insertion order, so `pairs` order
   holds (object-keyed tables are already sorted).
6. **Motif helpers in Lua.** Tufts, needles and grass as small gesture
   libraries in Lua (`paintings/lua/lib/`), loaded by a sandboxed
   `use "trees"` whose text is inlined into the log so replay stays
   self-contained.
7. **Time-lapse.** `frames on` already writes one JPEG per chunk; add
   `easel frames --video` via ffmpeg.
