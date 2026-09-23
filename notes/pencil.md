# Pencil underdrawing (round 4, branch `pencil`)

The painters in amnesia round 3 said they couldn't draw: "A painter draws
it; I compiled it" (`notes/amnesia3/easel3_near.md`). This stream adds
drawing as a medium of its own. You sketch the composition on the ground in
graphite or black chalk, look at it, erase and redraw, and then paint over it,
using the drawing as your guide. Friedrich worked this way:
graphite pencils of different hardness and black chalk, sometimes in two
passes (faint, then bolder), straights ruled. In the early works the drawing
stays visible through the very thin paint
(`notes/research/friedrich_materials.md` §3).

## What changed

- `crates/paint/src/graphite.rs` (new): the medium.
  - `Lead::pencil("2B")`, `Lead::graphite(softness)` and `Lead::chalk()`.
    Hardness (the share of clay in the lead) sets the flake
    reflectance (2H pale silver gray, 4B dark gray; graphite is never black),
    how much one pass lays (`rate`), the most it can cover (`cap`), how far
    pressure lets it bite into the tooth (`bite_um`), the grain (`crumble`)
    and how fast the point blunts (`blunt_mm`, width toward 3× the sharp
    point). Chalk is carbon black in clay: flake reflectance 0.022, matte,
    broad (0.9 mm), crumbly, and it wears fast.
  - `Canvas::draw(lead, mark, worn_mm, seed)` rasterizes a dense path with
    per-point pressure. At each pixel the point rides on the local tops of
    the surface height field (the weave through the ground, max over
    ±0.35 mm). The deposit falls off with depth below those tops as
    `exp(-depth / bite)`, with `bite = bite_um · p^1.2 + 3 µm`, so light lines
    catch only the crowns of the weave and heavy lines fill in. A pixel
    coarser than 0.15 mm counts its depth and grain as averages
    (so 1000px previews aren't beaded dots).
  - Optics: the deposit covers a fraction `a` of the pixel with flakes of
    reflectance `r`, written straight into the dry picture (`px`). Every
    later layer composites over it by Kubelka–Munk (`Pigment::over`), so
    thin paint shows it and body color hides it, with no special case.
    Graphite doesn't take on wet paint (pixels with wet paint are skipped).
  - A sparse bookkeeping layer, `Drawing` (20 bytes per pixel, created only
    when something is drawn), records `a`, `r`, how readily it lifts, a
    fixed floor and the film thickness when it was drawn. `Canvas::erase(mask,
    strength)` is a kneaded eraser. It lifts graphite up to 90% per pass and
    chalk up to 72%, both scaled from 55% in the hollows to 95% at the tops,
    so a ghost stays. It recovers the ground under the deposit exactly by
    inverting the mix. `fix_drawing(mask?)` sets the floor. Once the film is
    thicker than when the line was drawn, it is sealed: the eraser leaves
    it alone.
  - `drawing_mask()` (1 on a firm line, also under paint) and
    `drawing_view()` (the drawing alone on white, like an infrared
    reflectogram; no caller yet, see Known issues).
  - Mark builders: `hand_line` (Catmull–Rom or straight, arm sway and
    finger tremor, none against a ruler), `sketch_marks` (a searching line:
    n passes, each offset and bowed its own way, starting and ending early
    or late, lifting off now and then, the first pass lightest) and
    `hatch_marks` (short parallel strokes inside a mask, bowed by the wrist,
    heavy at the start and lifting off at the end).
- `crates/paint/src/canvas.rs`: one field (`drawing: Option<Box<Drawing>>`,
  `None` by default) and its initializer. `lib.rs`: `pub mod graphite` and
  `Lead` and `Medium` re-exported.
- `crates/easel/src/draw_pencil.rs` (new): the Lua bindings. It is declared
  from `api.rs` with `#[path]` and installed with one line, so `main.rs` is
  untouched.
  A pencil is a plain Lua table (`{grade=, kind=, worn=}`) with shared
  methods, so the wear of its point is on the Lua heap: undo, rollback and
  replay restore it (`easel check` passes on the study).
- `crates/easel/src/api.rs`: `canvas{size=mm}` sets the painting's width in
  mm. A pencil line is 0.3–0.9 mm. On the 1714 mm early style at 1000px it is
  a quarter of a pixel, so on a small canvas the drawing reads in a
  preview. The size also goes in the log's setup line.
- `crates/easel/README.md`: a "Drawing: pencil, chalk and eraser" section
  with a worked example. `README.md`: one bullet.

Without a drawing, nothing changes. The golden scene isn't affected, since
no code path runs unless `draw` is called.

## The painter's API (easel)

```lua
canvas{style="friedrich_early", size=440, aspect=1.4, seed=11}
h = pencil("2H")                 -- 9H..H, F, HB, B..9B; pencil{grade="2B"}; chalk()
h:rule({0, 432}, {1000, 432}, {pressure=0.3})      -- against a ruler
h:sketch(pts, {pressure=0.3})    -- searching: passes=3, wander=(units, ~2 mm), smooth=true
b = pencil("2B")
b:line(pts, {pressure={0.55, 0.7, 0.5}, smooth=false})   -- a firm line; corners kept
b:hatch(poly(shadow), {angle=-1.1, pressure=0.35})       -- spacing=, length= (units)
erase(pts, {strength=0.9, width=9})   -- or erase(mask, {strength=})
fix()                                 -- or fix(mask)
drawing_guide()                       -- the drawn lines (unbroken, whole canvas): paint into this
drawing_mask()                        -- the graphite deposit itself (grainy; window only in a crop)
b.worn  b:width()  b:sharpen()
```

## Evidence

`paintings/lua/pencil.lua` (9 chunks; `easel check`: "replay matches the
live canvas exactly"):

1. canvas: early grounds, 440 mm
2. first pass, 2H: ruled horizon, searching sketches of the ground, a rock
   and a tree
3. second pass, 2B: firm lines; the rock's top is drawn too high
4. the wrong top is lifted with the kneaded eraser (twice) and redrawn lower
   with corners, a ledge and a crack, the shadow side hatched, HB twigs
5. fixative; masks for the paint
6. left half: thin, translucent paint (`paint={0.3, 0.3}`, medium 0.6,
   load 0.3), then 7. blended
8. right half: body color (medium 0.15, load 0.9, coverage 4)
9. dry; the tree painted over its hidden drawing: trunk along the drawn
   coordinates, limbs into `drawing_guide():band(0.7, 1, 0.1):grow(1.2)` (was `drawing_mask():band(0.55, 1, 0.1)`, which beaded)

Images (`notes/pencil/`):
- `drawing_1000.jpg`, `drawing_3200_crop.jpg`: the drawing alone (chunks
  1–4). At 3200px: faint searching 2H lines under the firm 2B contour, the
  eraser's ghost of the first, too-high dome, hatching that catches the
  crowns of the weave.
- `painted_1000.jpg`, `painted_3200_crop.jpg` (window 330,400–650,580): on
  the left, the rock contour, ledge, crack and the ghost dome show through
  the thin paint ("still partly shimmer through"). On the right, the same
  rock's hatching is gone under body color.
- `chart_1000.jpg` (`paintings/lua/pencil_chart.lua`): rows 2H, HB, 2B, 4B
  and chalk at pressures 0.15, 0.35, 0.6 and 0.9 (three lines and a sketch
  each). Below: 2B hatching, 2B crosshatching, chalk hatching and a 4B
  block with a rectangle and a stroke erased.

Tests (`cargo test -p paint graphite`): grades parse; softer and firmer is
darker and chalk darkest; a light line favors the tops of the tooth; the
eraser lifts most but leaves a ghost; fixed drawing can't be lifted; a glaze
seals it; thin paint shows a line while body color hides it; no drawing
means no change; points wear, soft ones faster.

## Review 4 fixes (branch `fix4-engine`)

- **The drawing guide (#1, the beaded tree).** `Canvas::drawing_guide()`
  (Lua `drawing_guide()`) is the drawing as geometry. Every `draw` also
  lays its line into a whole-canvas buffer, and a crop render gets all of
  it too. The value is the coverage the lead would lay on a perfectly
  smooth ground in one pass (its rate and cap at the pressure), with no
  tooth, no grain and no wet paint to skip. It reads on the same scale
  as `drawing_mask`: 1 on a firm line, about 0.5 on a light 2H line, 0
  at zero pressure. The line is at least two pixels wide, so it samples
  unbroken. The eraser lifts it (by up to 85% per pass at full strength)
  and fixative sets its floor. Paint over the drawing leaves it alone.
  `drawing_mask()` is still the physical deposit, broken by the tooth and
  the grain, and known only in the window of a crop. Its doc now says so
  and points to the guide for planning. `guide_is_the_same_in_a_crop`
  paints into the guide on a whole canvas and on a crop: the retained
  pixels differ by less than 1e-3. With `drawing_mask` the same test
  differs by 0.73 over all 10,000 pixels.
- `paintings/lua/pencil.lua` chunk 9 paints the limbs into
  `drawing_guide():band(0.7, 1, 0.1):grow(1.2)`. The band keeps the firm
  2B and HB lines and drops the light 2H search. The beads are gone:
  compare `notes/pencil/tree_beaded_before_1000.jpg` with
  `tree_guide_1000.jpg`. `painted_1000.jpg` is re-rendered.
- **Checkpoints (#2).** Format `PAINTCK6` stores the drawing: every cell
  (`a`, `r`, `lift`, `floor`, `film`), the guide and the guide's floor.
  `PAINTCK5` files are refused (re-run to checkpoint again).
  `drawing_survives_a_checkpoint` erases, redraws and paints into the
  guide after resuming. The result is bit-identical to the run without
  the checkpoint.
- **Zero pressure (#7).** The deposit is multiplied by
  `touch(p) = smoothstep(0, 0.12, p)`: nothing at 0, rising continuously,
  unchanged from 0.12 up, so existing drawings at ordinary pressure are
  identical. A lift-off profile (`{0.8, 0}`) fades out to nothing.
- **Grades (#9).** `softness` takes ASCII digits then `H`/`B`. Anything
  else (`é`, emoji, `1.5B`, `+2B`) is `None`, not a panic.

## Known issues and next steps

- The guide ignores sealing. After paint has gone over a line, the eraser
  can't lift it from the picture, but it still lifts it from the guide.
  Sealing depends on the paint film, which a crop render doesn't hold
  outside its window, and the guide has to be the same in a crop.
- The guide is as wide as the point (at least two pixels). It doesn't
  taper as the line lifts off, only fades, so limbs painted into it end
  bluntly. A painter who wants tapered ends can multiply by a ribbon.
- Memory: the guide is 4 bytes per whole-canvas pixel (8 once fixative
  is used), on top of the 20 bytes per window pixel of the cells.

- **No `look --mode drawing`.** `look.rs` belongs to the lookaid stream.
  `Canvas::drawing_view()` is ready. The hook is about six lines: a
  `drawing` flag in `View`, `"drawing" | "irr" => v.drawing = true` in
  `parse`, and `if v.drawing { c.drawing_view() } else ...` where `look`
  picks its pixels. Until then, `drawing_mask()` and `look --crop` do the
  job.
- **Sheen** is only in the flake reflectance. `relief()` doesn't give soft
  graphite its raking-light shine.
- **Wet paint doesn't pick up loose graphite.** Real unfixed graphite grays
  and smears into oil paint, which is why painters fix it. `fix()` only
  stops the eraser for now.
- **The constants are tuned by eye** against the grade chart, not measured.
  The tooth depths come from this engine's height field: 30–48 µm median
  below the local tops (`graphite::probe::tooth_depths`, ignored test).
- Hatching is even in rhythm; no zigzag hatching yet. Pencil lines have no
  `clip=` option.
- Memory: 20 bytes per pixel once something is drawn (about 15 MB at
  1000px, 146 MB at 3200px, 7:5).
- Seen in passing (other streams): `hand="glaze"` with `medium=0.85` laid
  puddled, oversized strokes that ran past the mask (chunk 6, first try).
  Default `paint=` hiding is effectively opaque at a lean film, so thin paint
  needs `paint={0.3, 0.3}` to let a drawing show.
