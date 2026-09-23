# Depth masks and editing a chunk in place

Branch `depth`. This stream answers two complaints from amnesia round 3
(`notes/amnesia3/easel3_*.md`):

- Free #3: "keeping things behind other things is all manual masking, and
  that caused most of my errors". The sea veil painted over the figure
  and glints crossed the stones. Near #2: shadow masks made with
  `(m:grow(n) - m):soften(k)` came out as hard rings.
- Free #7, near #9 and green #10: fixing an early chunk meant close, `sed`
  the log, reopen and replay (27–110 s), and undo threw the chunk's code
  away.

## Part 1: depth from the world

### Engine (`crates/paint/src/scene.rs`, additive)

- **`Depths`** (`view.depths()`, built on first use): at every pixel, a
  stack of what is seen along the line of sight, nearest first. The stack
  holds the ground or water (or sky), every visible body and every layer,
  each with its coverage and distance in meters. A body's coverage comes
  from one ray per pixel and four at its silhouette, so its edges are soft
  over a pixel. Proxies are left out: they are never seen. What is seen of
  a figure written with gestures is its layer. Masks are front-to-back
  composites, so a soft edge hides exactly as much as it covers:
  - `visible(sel)`: coverage less whatever is in front
  - `front(sel)`: what hides it, where it is
  - `behind(sel)`: where a pass lying just behind it shows
  - `at_depth(z)`: where a pass `z` m off shows
  - `between(a, b)`: whatever is seen between `a` and `b` m
  - `seen_at(x, y)`: each thing's share of the pixel
- **Layers**: `World::layer(name, mask, LayerDepth::At(m) | Ground)`. A
  motif painted by hand is registered at a depth, so the world knows what
  it hides and what hides it.
- **Soft shadows**:
  - `View::soft_shadows(soft, casters)` traces the world's sun with
    `penumbra × soft`. The edge widens with distance from the caster.
  - `View::occlusion(reach, by)` is a contact shadow from
    `World::sky_occlusion`: a 16-direction, cosine-weighted cone trace of
    the sky hidden within `reach`. Occluders fade out toward `reach`, so
    it falls off smoothly with distance and has no edge.
  - Both are weighted by how much of the ground is seen. They never
    darken the sky (no ring above a stone) or a figure standing in front.
  - `World::cast_soft` is `cast` with the penumbra and casters as
    parameters (`cast` itself is unchanged).
- **`Handling::limit` and `Stipple::limit`** (additive, in handling.rs and
  stipple.rs): a mask no bristle crosses, whatever `clip` says. Without
  it, `work` strokes overshoot their mask (`hug`) straight into the figure.
  With it they still overshoot the region's own edges, but never into
  what is in front. `None` by default: no output changes (golden
  unchanged).

### Easel (`crates/easel/src/depth.rs`, new)

- `w:layer(name, mask, depth)`: the depth is meters, a spot, a canvas
  point `{x, y}` (the ground seen there, meaning the figure's feet) or
  `"ground"`.
- View masks: `v:visible`, `v:front`, `v:behind`, `v:at_depth`,
  `v:between`, `v:seen(x, y)`, `v:layers()`,
  `v:cast_shadow{soft=, from=}` and `v:contact_shadow{reach=, from=}`.
- Pass options on `work`, `blend`, `stipple` and `glaze`: `visible=`,
  `behind=` (names, body numbers, masks or a list) and `at=`. They use the
  last view made (`w:view()`, kept in the `Studio` and rolled back with
  it) or `view=v`.
- Edits to shared files, all small: world.rs (the recipe carries layers,
  `w:layer`, and the view registers itself and adds the depth methods);
  api.rs (a `Studio.view` field, the option keys, the `restrict` calls);
  session.rs (the snapshot holds the view); main.rs (`mod depth`).

```lua
w = w:proxy(fs, body.ellipsoid(fs:p(0, 0.85, 0), fs:size(0.26, 0.88, 0.18)))  -- his shadow
w = w:layer("figure", coatm + headm, fs)                                     -- what is seen of him
v = w:view()
stipple(seaband, {width=2.6, color=veil, coverage=3, behind={"figure", "bodies"}})
work(zone, {hand="detail", tool="round 1", coverage=0.18, angle=0, color=glint, visible="water"})
glaze(v:cast_shadow{soft=1.6}, {color="#4a4c60", coats=0.55})
glaze(v:contact_shadow{reach=0.35}, {color="#2a2420", coats=0.9})
```

### Evidence

`paintings/lua/depth.lua` (10 chunks, about 50 s at 1000px): a man seen
from behind on a beach in front of a calm sea, three stones at the
water's edge (one behind him, one sunk in the sea), a low sun from the
left. The sky, the sea band and the beach are painted first. Then come
the stones from `v.form` (`v:visible("bodies")`, which stops at his
outline), the man, and the shadows from the world. After that, a stippled
sea veil (`behind={"figure", "bodies"}`) and sparse glints
(`visible="water"`). `paintings/lua/depth_hand.lua` is the same log with
chunks 8–10 redone the round-3 way (made with `easel edit`): the veil and
glints on the hand-drawn sea mask with nothing subtracted, ring contact
shadows and a hand-drawn soft band for the man's shadow.

- `notes/depth/figure_hand_vs_depth.jpg`: hand masks on the left. The
  veil is stippled over the man's coat and the stone behind him, exactly
  free #3's accident. On the right, depth options: the veil and glints
  stop at his outline, and the stone behind him stops at his edge.
- `notes/depth/shadows_hand_vs_depth.jpg`: hand masks on top. A black
  ring runs around each stone's foot and the veil crosses every stone.
  Depth options below: the cast shadow is crisp at the foot and widens far
  out, and the contact shadow is a dark seam under each stone that fades
  into the sand.
- `notes/depth/depth.jpg` and `notes/depth/depth_hand.jpg`: the whole
  pictures.

As a painting it is a demonstration, not a Friedrich. The man is a stiff
silhouette, the sea is plain and the far stone sits on the water rather
than in it.

Tests:
- `scene::tests::depth_masks_know_what_is_in_front`: a figure layer over
  a stone and the sea, covering visible, front, behind, at_depth, between
  and soft silhouettes.
- `scene::tests::soft_shadows_fall_off_without_rings`: the penumbra widens
  with distance and with `soft`. The contact profile is monotone with no
  step, zero a reach away, in the sky and up the pole.
- `depth::tests`: the options through Lua. The figure's pixels are
  untouched by a veil `behind="figure"`, by glints `visible="water"` and
  by a glaze. An unknown layer gets a helpful error, the replay is exact
  and the current view rolls back with a failed chunk and with undo.

## Part 2: editing a chunk in place

- `easel edit N -f chunk.lua | '<lua>' | -` replaces chunk N. `--insert`
  puts it before N, `--drop` removes N and `--undone K` uses undone code.
  `easel show N` prints a chunk. `easel undone [K]` lists or prints what
  was undone or replaced. `easel redo [K]` runs it again at the end.
- `Session::splice(at, remove, insert)` takes a snapshot of the current
  state, restores the nearest snapshot at or before `at` and runs the
  chunks from there. If any chunk fails, it restores the snapshot, the
  log and the snapshots exactly as they were ("nothing changed").
- **Checkpoints are in memory**, because a snapshot is the Lua heap
  (heap.lua's copy of every reachable table and upvalue) plus canvas,
  brushes and clock, and the heap can't be written to disk. Snapshots are
  kept by chunk count: the last `--undo` (8) plus `--checkpoints` (default
  6) older ones on a grid whose spacing doubles as the log grows (0, 4, 8,
  12, 16, 20 in a 30-chunk log). Each costs about 50 MB at 1000px, so the
  default adds about 300 MB. heap.lua's restore never changes its copy,
  so one snapshot can be restored many times.
- `undo n` past the undo snapshots replays from a checkpoint. The code of
  undone and replaced chunks goes to `out/easel/<name>/undone.lua`. The
  log stays the single source of truth: an edit rewrites it, and
  `easel check` confirms the live canvas equals a fresh replay.
- Test: `session::tests::edit_a_chunk_in_place_from_a_checkpoint`
  covers replace, a failed edit (its own error, or a later chunk's
  assert), undo after it, insert and drop, and an undo of 5 past a ring
  of 2. Each result equals a fresh replay of the new log, bit for bit,
  with the same clock.

### Timing: edit at chunk 5 of 30

This was measured on the first 30 chunks of `easel3_free.lua` at 1000px,
on the shared 10-core machine with `cargo test` running beside it:

| what | time |
|---|---|
| `easel run` (full replay, fresh process) | 62.1 s |
| `easel open` (replay into a live session: the old close, sed, reopen path) | 47.4 s |
| `easel edit 5 -f chunk5.lua` (from the checkpoint after chunk 4; 26 chunks) | **30.0 s** |
| `easel check` afterward | 43.8 s, exact |
| `easel undo 12` (past the 8-deep ring: replays from checkpoint 16) | 3.9 s |
| `easel edit 8` in depth.lua (3 chunks from the undo snapshot) | 1.0–9.6 s |

The saving is the chunks before the edit. Here those are the canvas and
the stippled sky, the slowest chunks of the painting. Everything after
the edit has to be painted again: paint is physical, and a later stroke
picks up what an edited one left. Honest summary: an edit at chunk 5 of
30 costs about two-thirds of reopening, and an edit near the end costs
seconds.

## Known issues and next

- A layer is flat at one depth (or on the ground). A long motif that runs
  into depth (a fence, a wall) needs several layers or bodies.
- Depth options use the last view made. A chunk that makes a scratch view
  (for a probe) changes the default. Pass `view=v` to be explicit.
- `v:behind` with several names multiplies them one by one ("behind each
  of them"), which is what painters meant in every case I found.
- An edit renumbers nothing unless it inserts or drops. Inserting changes
  the later chunks' random seeds (they depend on the chunk number).
- Faster edits need dependency tracking: skip later chunks whose pixels
  and Lua state the edit can't reach. That can be made exact but is a
  project of its own.
- Pairs order after a rollback (README, Limits) applies to edits too.
  `easel check` catches it.
