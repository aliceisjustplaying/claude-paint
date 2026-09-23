# Looking by eye (round 4, stream `lookaid`)

The round-3 painters placed every mark by coordinates guessed from a JPEG
("every mark was a coordinate I estimated from a JPEG; I never touched the
picture"), undid probe chunks to keep them out of the log, and waited for
2-minute 3200 px renders to judge small details. This stream gives the
easel ways to read positions off the picture, ask what is at a point, see
geometry before painting it and look at a window at full resolution while
painting.

## What landed

- **`look --grid [step]`**: major and minor lines in canvas units, with the
  major values along the top and left edges and the steps in the corner.
  The step is picked from the zoom (about 90 output px per major step:
  100/20 on the whole canvas, 20/4 on a 160-unit crop) or given. Lines
  darken light paint and lighten dark paint, so they read over sky and
  shadow alike. It works on crops, `--mirror` (the labels still give true
  x) and `--scale` looks.
- **`look --probe x,y;x,y`** and Lua **`probe(x, y [, view])`**: the color
  (hex, OKLab, value), the drying stage, the wet film in µm (new engine
  accessor `Canvas::wet_um`) and, with a view or world in a global (or
  passed), what the eye sees there: sky, ground, water or body N, its
  distance and the point in meters. The CLI prints one line per point and
  draws numbered crosses. `probe()` returns a table and marks P1, P2...
- **Lua `show(...)`**: an overlay for the next looks. It takes a mask, a
  point list (a polyline; a polygon with `closed=true`; the vertices are
  numbered), a path with `width=` or `brush=` (the band that stroke will
  cover) or `x, y, label`. It never paints and returns its first argument.
  The first `show()`/`probe()` of a chunk replaces the overlay. `look
  --show on|off|clear` (bare: toggle) controls it.
- **`easel try '<lua>'`**: runs a chunk as `do` would (same chunk number,
  so the same seeds), keeps its prints and overlay, then rolls it all back.
  It isn't logged and keeps every undo snapshot (it runs at undo depth + 1
  and undoes itself). This is the preview loop: try a shape, look, adjust
  the numbers, try again, then `do` the painting chunk. It also answers
  the round-3 request for an unlogged probe chunk (easel3_free friction 8).
- **`look --crop x0,y0,x1,y1 --scale 3.2`**: the window as it looks at
  3200 px, live. `crop.rs` keeps a background thread per window (two at
  most) that owns a session painting a slightly larger window (30 units
  more on each side, a 40-unit margin painted and not shown) at that
  width. It follows the live log chunk by chunk, and when the live session
  undoes, it undoes too (undo depth 8; past that it rebuilds). A look waits
  up to `--wait 90` s, then shows the latest finished state (saying how far
  behind it is) or says to look again. Any crop inside a kept window
  reuses it.
- Looks enlarge crops by a whole factor rounded up, as long as the image
  stays within 1.6× `--size`: a 600-unit crop now comes back at 1200 px
  instead of 1:1 (easel3_free friction 14).

## Design decisions

- **show() in the log: logged, a no-op in replay.** Overlays live in Lua
  app data (`look::Marks`), outside the heap snapshots, and only a live
  session records them (`look::begin` marks a session live before each
  `do`/`try`; `easel run`, `easel check` and crop sessions never call it).
  In a replay `show()` validates its arguments and returns the first one,
  and `probe()` returns its table. Neither touches the canvas, the RNG or
  the automatic seeds, so a chunk with shows replays exactly as without
  them. Validation is the same live and in replay, so a chunk can't pass
  live and fail on replay. For previews that shouldn't be in the log at
  all there is `try`.
- **Crop sessions without the global crop.** `paint::set_crop` is
  process-wide and read when a canvas is made, which would race between
  the live session, crop threads and parallel tests. So `Studio` got a
  `crop` field, and `canvas{}` uses the new `Style::prepare_window` (the
  same priming on `Canvas::new_window`). `prepare` is unchanged
  byte-for-byte.
- **Why a follower and not a one-off crop replay.** A one-off crop replay
  is not faster: masks are whole-canvas at 3200, so replaying the round-3
  "near" log with `easel run --width 3200 --crop 400,300,600,450` took
  226 s, against 205 s for the whole canvas at 1000 px (same busy machine).
  Following the log pays that once per window; after that a look costs
  only the new chunks at crop width.
- **Depth from the globals.** Worlds and views are Lua values, not studio
  state, so `probe` reads the first global holding a view (then a world),
  sorted by name so replays pick the same one. A painter keeping `local v`
  gets no depth until they pass it: `probe(x, y, v)`.
- **Drawing.** Overlays are drawn in output pixels after the view modes,
  with a 3×5 bitmap font on dark plates (2× at ≤1500 px, 3× above). Masks
  are sampled at each output pixel (tint by coverage, outline at 0.5).
  Paths get a dark underlay so they read on any paint.

## Files

- `crates/easel/src/look.rs`: view flags, overlay store, `show`/`probe`,
  `try_chunk`, rendering (grid, marks, font), tests.
- `crates/easel/src/crop.rs` (new): crop sessions that follow the log.
- `crates/easel/src/main.rs`: the look command (probes, overlay,
  `--scale`), `easel try`, crop sync after `do`/`undo`.
- `crates/easel/src/api.rs`: `Studio.crop`, `canvas{}` on a window, one
  line registering `show`/`probe`.
- `crates/paint/src/style.rs` (`prepare_window`), `crates/paint/src/wet.rs`
  (`wet_um`).

## Tests (`cargo test --release -p easel look::`)

- `show_and_probe_leave_the_canvas_and_the_replay_alone`: a chunk with
  every kind of show() and two probes leaves the canvas bits unchanged;
  the overlay rules (replace, keep, off, toggle, clear); the log replays to
  the same bits and the replay keeps no overlay; bad arguments fail both
  live and in replay.
- `try_rolls_back_and_keeps_the_overlay`: a tried chunk that painted,
  waited and set a global leaves the canvas, clock, log and globals as
  they were, keeps its overlay and both undo snapshots; a failing try
  changes nothing.
- `looks_draw_the_aids_without_touching_the_canvas`: grid, probes, marks
  and mirror on a crop, the canvas bits unchanged; flag parsing.
- `a_crop_session_follows_the_log_through_undo`: forward, undo (bits equal
  the earlier state) and a crop inside the kept window equal a fresh crop
  replay bit for bit.

## Study

`paintings/lua/lookaid.lua`: an evening field with a world, then a chunk
that shows a sheep polygon, a pond ellipse and a brush path, probes the
pond (ground, 20.6 m away) and paints the path in the probed color. It
replays in about 23 s at 1000 px (`easel run paintings/lua/lookaid.lua
--look`), the shows doing nothing. Run the last chunk with `easel try` in
a session to see them. A live session over it passed `easel check` after
CLI probes and a chunk that painted from `probe()`.

## Evidence

(`out/` is not committed; these come from `paintings/lua/lookaid_demo.lua`
and a live session on `notes/amnesia3/easel3_near.lua`.)

- `out/easel/lookaid_demo/look-0001.jpg`: whole-canvas grid 100/20.
- `out/easel/lookaid_demo/look-0002.jpg`: a try's overlay: sheep polygon,
  pond ellipse, a brush path band, a labeled point, a Lua probe and three
  CLI probes (sky, ground 20.6 m away, sky).
- `out/easel/lookaid_demo/look-0003.jpg` / `-0005.jpg`: the sheep at
  1000 px (grid 20/4) and the same window at 3200 px.
- `out/easel/lookaid_near/look-0001.jpg`: the near painting with a grid
  and three probes.
- `out/easel/lookaid_near/look-0002.jpg`: a 600-unit crop at 1200 px with
  grid 50/10, a previewed crack path (rigger band, numbered) and a
  fallen-block polygon, not painted.
- `target/easel-look-test/aided.jpg`: the test's mirrored crop with
  every aid.
- `out/easel/lookaid_near/look-0003.jpg`: the birch at 3200 px from the
  live session (window 400,20,560,250). First look at the window: 560 s of
  background painting for 26 chunks on the busy machine (the look itself
  waited 316 s after two earlier short looks started it).
- `out/easel/lookaid_near/look-0004.jpg` → `-0005.jpg`: a twig previewed
  with `try` + `show(twig, {brush=b})` on a 10/2 grid at 3200 px, then
  painted with `do` (chunk 27, 0.07 s). The next `--scale 3.2` look was
  current in 0.21 s: the crop session had painted chunk 27 in the
  background. The twig lies on the previewed points.

## Known issues

- The first `--scale` look at a window costs a full replay (minutes for
  a long log on a busy machine); keep windows inside one you already have.
  Every crop session also holds its own Lua state and canvas, and the
  background thread competes with the live session for cores while it
  catches up.
- Crop sessions see only their window: `sample()`/`probe()` outside it
  read less than the live session, as with `easel run --crop`, so a chunk
  that paints from them can differ there.
- The overlay lives in the server: `easel close` drops it (the log keeps
  the shows, which replay as no-ops).
- `show(mask)` outlines the 0.5 level only; a very soft mask shows mostly
  as tint.
- `show(points)` draws straight segments; a brush stroke through the
  same points is smoothed by the engine, so it curves a little between
  them (see look-0005).

## Next

- `look --show` for a mask by name from the globals without a chunk
  (`look --show-mask sky`).
- Crop sessions resumed from engine checkpoints (`checkpoint.rs`) instead
  of a full first replay.
- A `--mode drying` map (open/tacky/dry) now that probes report the stage.
