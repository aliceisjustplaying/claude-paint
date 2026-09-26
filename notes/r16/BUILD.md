# Round 16: the studio build

Branch `r16-base` (from main `4abe5b4`), worktree `~/src/a/claude-paint-r16-base`.
This note lists every change, what was removed, what was kept and why,
the Lua API before and after, the test results and the questions left for
the owner. The painter briefs, the chain runner and the handoff reader
are not part of this build.

## In short

- **The engine** is Round 15's cleaned `crates/paint` (from `r15-base`),
  plus three small additions made test-first: piles laid as knifed
  (`Handling::piled`, `Stipple::piled`), tubes that dry at their own rates
  (`Tube::drying`, `Mixture::laid`) and the tube box (`Palette::tube_box`).
  Palettes and styles are renamed to material names; no painter's name is
  left in the code.
- **The easel** has no undo, edit, try, previews, probes, overlays, color
  functions, canvas sampling, `dry()`, sittings or subject generators.
  Hand time is always on; `wait(minutes)` takes any length and returns the
  time of day. Paint reaches the canvas only as piles knifed from named
  tubes (`pile{}`), and `canvas{}` takes the painter's size, linen and
  ground. `easel note` appends to `notes/journal.md`.
- **The painter's guide** is `notes/easel_guide.md` (467 lines); the old
  1437-line `crates/easel/README.md` is now a pointer to it.
- **The export**: `scripts/export_r16_studio friedrich|blank <dest>`.
- **Tests**: `cargo test --workspace` passes (198 tests, 17 ignored);
  `cargo build --release --workspace` passes.

## Commits

```
fdfb0ff r16 base: Round 15's cleaned engine (crates/paint from r15-base: no rock, atmos, Ridge, growth, fir or broadleaf; history out of comments); the Rust paintings crate out of the workspace
f933116 Engine: piles laid as knifed (Handling::piled, Stipple::piled ...); tubes carry drying rates (Mixture::laid); the tube box; palettes and styles renamed to material names
bf8183e Easel: no undo, edit, try or previews; hand time always on, wait(minutes) returns the time of day, no dry(); piles knifed from the tube box ...; a smoke test
3cd457e Easel and engine: neutral examples and fixture names; outline chars without subject aliases
d61e735 Engine comments: no references to the painter-specific research note
83961b0 The painter's guide: notes/easel_guide.md; README and the easel README trimmed to point at it
fda040c Research notes for the studio: the materials note without named paintings, motif methods or aesthetic advice; the physics note without artists, paintings or national traditions
a8926bd scripts/export_r16_studio
4aa0d41 Export leaves out the build's smoke test; the hand-time test shows paint setting under handwork
```

(The workspace doesn't build at `fdfb0ff` alone: the easel still called
the removed engine modules until `bf8183e`.)

## 0. The base: Round 15's engine, no Rust paintings

- `crates/paint` is taken whole from `r15-base` (`git checkout r15-base --
  crates/paint`). Main hasn't touched `crates/` since the two branches'
  merge-base `af51374` (`git diff --shortstat af51374 main -- crates` is
  empty), so this adds Round 15's cleanup and loses nothing: no `rock`,
  `atmos`, `form::Ridge`, `growth`, `fir`, `broadleaf` or `crack_lab`;
  history, people and verdicts out of the comments (astra_review findings
  1, 3, 9, 15–24 as Round 15 applied them). The engine's tests pass
  unchanged (golden scene included).
- The `paintings` crate (the Rust runner, studies, past paintings, motif
  modules, the Lua example logs in `paintings/lua/`) is removed from this
  branch and the workspace (`Cargo.toml` members are `crates/paint` and
  `crates/easel`), and the `cargo paint` alias is gone from
  `.cargo/config.toml`. Painters paint at the easel only. Main keeps all of
  it.

## 1. The easel API

### a. No undo

Removed: `easel undo`, `undone`, `redo`, `edit` (replace, insert, drop a
chunk and replay), `show N`, the `--undo` and `--checkpoints` snapshots,
`crates/easel/src/edit.rs` and `Session::undo`/`splice`. A chunk that fails
still changes nothing: the session takes one snapshot before each chunk
and restores it on an error (`crates/easel/src/session.rs`, `Session::run`).
A chunk that succeeds is in the log for good. Reopening a session replays
its log.

### b. No previews

Removed: `easel try`, `show()`, `probe()` and their overlays, `look --probe`,
`--show`, `--dried` (the canvas dried on a copy: a preview of the future),
`--relief` (it dried the copy first), `--scale` with its background
3200 px crop sessions (`crates/easel/src/crop.rs`: another resolution of
the program, not the canvas) and `--wait`.

Kept, and why:
- `look` whole and `--crop` (enlarged by whole pixels), `--size`.
- `--mode wet`: where the paint is open, setting, tacky or dry. It shows
  the real canvas's state, as a knuckle feels it.
- `--mode value`, `squint`, `mirror`: the paint as a painter looks at it
  (a monochrome viewing glass, half-closed eyes, a mirror). They transform
  the view of the real canvas; they show nothing that isn't there.
- `--grid [step]`: drawn on the JPEG, never on the canvas, labeled in
  canvas units. It is the painter's squared-up drawing: the squares ruled
  over a sketch to transfer it. Painters place every mark by coordinates,
  so without it they would read positions off pixels. Kept.

### c. Time

- `dry()` is gone. So are `clock()`, `hand_time()`, `sitting{}`, `rest()`,
  `timesheet()`, the sitting limit that refused marks and the log header
  that enforced it (`time.rs` rewritten).
- Hand time is always on from `canvas{}` (`time::start`): every stroke,
  touch, pass and trip to the palette costs the time a hand takes
  (`crates/paint/src/tally.rs`), and the paint ages while the hand works.
- `wait(minutes)` takes any length and returns the time of day, e.g.
  `day 3, 14:20`; the painting begins at 09:00 on day 1
  (`time::time_of_day`). Why: a painter at the easel knows what day it is
  and roughly the hour, and needs that to judge drying ("I laid that two
  days ago"); the hand's own time is otherwise invisible to them. It is
  not a score: nothing else prints the clock. The chunk reply is `ok ·
  chunk N (x s to compute)` (machine seconds, for planning chunk sizes),
  `status` is chunks, width and canvas setup, and the log's chunk markers
  are `--@ chunk N` (no clock).
- `drying(x, y)` stays: the paint's stage at a point, as touch tells a
  painter.
- `glaze()`, `varnish{}`, `cracks{}` and `relief()` used to dry the whole
  canvas first (the engine's `Canvas::glaze` and `relief` call `dry()`),
  which was a `dry()` shortcut. Now each refuses unless every film on the
  canvas is touch-dry (`needs_dry`, `crates/easel/src/api.rs`); the painter
  waits. A glaze into wet paint is brushed: `work(m, {hand="glaze",
  pile=...})`.

### d. Hand mixing

Removed: `pal`, `palette()`, `pal:with/only/mix/aim/paint`, `paint()`
(including `{raw=true}`), `b:load(color, amount, {medium=, at=, coats=,
pal=, raw=, hiding=, stiff=})`, and `color=`, `color_over=`, `jitter=`,
`medium=`, `pal=`, `aim=`, `paint=` on `work`; `color=`, `color_over=`,
`medium=`, `pal=`, `aim=`, `paint=`, `jitter=`, `fade=` on `stipple`;
`color=` and `pigment=` on `glaze`; `color=` on `varnish`; `pal=`,
`medium=` and `mix=` (and the canvas sampling) on `lose`.

Added:
- `pile{{"lead white", 6}, {"smalt", 1}, {"yellow ochre", 0.5}, medium=0.2}`:
  parts by volume of named tubes, normalized, and a share of oil medium
  (0 to 0.95). It is the engine's `Palette::pile` on the tube box; knifing
  it costs 20 s of hand time (`Piles::knife`, `Tally::knife`, new) and puts
  it on the palette, so later dips into it are reloads. `print(p)` shows
  its parts, never a color.
- `b:load(p, amount)`, `b:reload(p, amount)`; `work{pile=p}`,
  `stipple{pile=p}`, `glaze(m, {pile=p})`, `lose(m, {pile=p})`,
  `o:paint(b, {dip={p, amount}})`. Every dip is the pile remixed by the
  style's mixing jitter (6% relative sd of the proportions: a pile mixed
  by hand is uneven), nothing aimed or matched.
- `tubes()`: the names in the box.
- Engine, test-first: `Handling::piled` and `Stipple::piled` (every dip
  from the given pile, the color field, `palette` and `aim` unused;
  `handling::tests::a_pass_from_a_pile_lays_the_pile`,
  `stipple::tests::stippling_from_a_pile_lays_the_pile`), `Tube::drying`
  with `Mixture::drying` and `Mixture::laid` (a pile dries at its tubes'
  rates mixed by volume; before this, every paint the easel laid dried at
  the average rate 1; `palette::tests::a_pile_dries_at_its_tubes_rate`).
  `Mixture::paint` is unchanged, so the golden scene is unchanged.
- The tube box (`Palette::tube_box`, test
  `the_tube_box_holds_every_tube_once`): the union of the tubes the engine
  knows, 14: lead white, smalt, pale smalt, yellow ochre, red earth,
  vermilion, raw umber, bone black, cobalt blue, chrome yellow, Prussian
  blue, green earth, Rinmann's green, copper green. All were made by the
  1820s. The guide lists each with its pigment, hiding, stiffness, tinting
  strength and drying rate, and no color or use.
- `canvas{size=, aspect=, linen=, ground=, seed=}`: the painter chooses the
  width in mm, the aspect, the linen's threads per cm and the ground layers,
  each a paste of tubes in parts with a thickness and how it is put on
  (knife, roller, brush). All but `seed` are required; the error shows the
  form. The style is `Style { name: "oil", .. }` built on `Style::oil()`
  (tools, handling presets, blending); no painter's name in the API.

### e. No per-pixel color

Removed: `color()`, `rgb()`, `mix()`, `gradient()`, `shift()`, `sample()`,
color values and their arithmetic (`Col`: `.r .g .b .L`, `:mix`, `:hex`),
color fields (`color=function(x, y)`, a sky or clouds as a color),
`color_over`. Functions of `(x, y)` remain for angles, coverage, load,
edge quality and glaze thickness (numbers, not colors), and for masks.

### f. No subject generators

Removed with their Lua bindings: `draw_firs.rs` (`fir{}`, `fir_wood{}`),
`draw_trees.rs` (`tree_in{}`, `tree_group{}`), `draw_rocks.rs` (`rock{}`),
`tree{}` and `t:foliage{}`, `sward{}`, `ridge{}`, `w:sky{}`, `w:clouds{}`,
`w:ranges{}`, `haze{}`, and the `EASEL_WITHOUT` switch (nothing left to
switch off). The engine modules behind them were already gone with Round
15's engine. Outline characters lost their subject aliases (`"rock"`,
`"foliage"`, `crates/paint/src/outline.rs`).

Kept, as scaffolds on shapes the painter supplies (the line of astra_review
findings 1, 2 and 7):
- `body.ellipsoid/block/half_space` with turn, cut, rough, union,
  subtract; `terrain{}` (the painter's height function); `form{}` and its
  light, masks and fields (light and shadow on the painter's solids).
- `world{}`: camera, horizon, eye height, field of view, the painter's
  ground function, water level, one sun; `w:place`, `w:proxy`, `w:layer`,
  spots and projection, `w:ribbon`, `w:recede`; the view's masks (where
  land, water and sky are seen, cast shadows, contact, reflections) and the
  depth options (`visible=`, `behind=`, `at=`). These say where a placed
  shape stands, how big it looks and where its light falls; they invent no
  shape.
- `aerial(dist, visibility)` and `w:aerial(Z)`: how much air lies between
  the eye and a distance, a number. With no color arithmetic left it only
  informs.
- `outline{}` and `body_of{}`: a line drawn through the painter's points,
  a silhouette from the painter's spine and widths.
- `noise`, `worley`, `uneven`: generic randomness.
- The handling presets (`hand=`), `edge=`, `lose()`, the pencil, eraser and
  fixative, `varnish{}`, `cracks{}`, `relief()`: physical, and each the
  painter's explicit choice.

### g. Cleanups from astra_review in the easel

- History, people, rounds and review labels out of the easel's comments
  and tests (`review 3/4, finding ...`, `Round 7`, `Alice`, old/new
  narratives); fixture names made neutral (a "figure", "boat", "stone",
  "sea", "hill", "rock" and "sheep" became an upright, a block, water, a
  shape and plain names).
- Test programs paint swatches, strokes and bands, not landscape parts.
- No negative lists in anything a painter reads. The build's smoke test
  does list removed commands and verbs (the brief asks for it), so the
  export leaves `crates/easel/tests/smoke.rs` out.

## 2. What the painter sees

- `notes/easel_guide.md`, new: opening a session, the canvas, tubes and
  piles, brushes and strokes, covering an area (presets and options,
  edges, stipple, glaze, lose), masks and geometry, drawing, solids and
  space (form, world, depth), time, finishing, looking, how chunks behave,
  the journal, replay, costs. Examples are swatches, a stroke and piles.
  No recipes, subjects, taste, history or people. Every example in it was
  run (a scratch log replayed with `easel run`: all eight chunks ran).
- `README.md` rewritten: what the simulator is, how to build and test,
  where things are. `crates/easel/README.md`: a pointer to the guide.
- Left out of the export: the dev notes, studies, past paintings and their
  programs, the Rust runner, `scripts/` (including `peek`: `look` already
  writes JPEGs of at most 1000 px), `studio/`, and research beyond the
  profile's notes.

## 3. The export

```sh
scripts/export_r16_studio friedrich <dest>
scripts/export_r16_studio blank <dest>
```

`<dest>` must not exist or be empty. The script `git archive`s `r16-base`
(override with `R16_BRANCH`) through a fixed list (`.cargo/config.toml`,
`.gitignore`, `Cargo.toml`, `Cargo.lock`, `README.md`,
`THIRD_PARTY_NOTICES.md`, `crates/easel`, `crates/paint`,
`notes/easel_guide.md`, `notes/research/oil_paint_physics.md`, and for
friedrich `notes/research/friedrich_materials.md`), drops the smoke test,
creates an empty `notes/journal.md` and `paintings/lua/`, greps for
painters' names (blank: none outside `THIRD_PARTY_NOTICES.md`; friedrich:
none but Friedrich's) and fails if it finds any, then builds the easel
fresh (`cargo build --release -p easel`, with `CARGO_TARGET_DIR` unset;
it retries with `DEVELOPER_DIR=/Library/Developer/CommandLineTools` if the
Xcode license blocks linking).

The research notes in this branch are the cleaned versions:
- `notes/research/friedrich_materials.md` (findings 3 and 4): supports,
  grounds, underdrawing, pigments, media, layers, varnish, condition and
  greens, with sources. No painting titles (objects are described by date,
  size or place), no motif-specific methods (firs, grass, foliage
  drawings), no aesthetic advice (the moonlit glaze, Field's harmony rule,
  Carus, Goethe), no list of missing evidence by motif. Source titles that
  name a motif are cited by author, venue and year.
- `notes/research/oil_paint_physics.md` (finding 20): the same numbers
  with no artists, paintings or national traditions; "Typical canvas work
  should be far lower" became "That is one panel measurement, not an
  estimate for this simulator's canvas."

Checked: both exports built; `grep -rIniwE` for the list of names in the
script finds nothing in the blank export (source, notes and the built
binary's strings); a trial session in the friedrich export (open, canvas,
piles, a pass and a stroke, wait, look with a grid, a removed verb and
command refused, a note, check, close) behaved as documented.

## 4. The journal

`easel note '<text>'` (or `easel note -` from stdin) appends a dated line,
`- 2026-09-26 21:43 UTC: <text>`, further lines indented under it, at the
end of `notes/journal.md` (`main.rs`, `note` and `journal_entry`). It
opens the file for append only; nothing in the easel rewrites it. The
guide calls it "your working journal" and says entries stay as written.

## 5. Tests

`cargo test --workspace` (debug-profile tests, optimized):

| suite | passed | ignored |
|---|---|---|
| paint (lib) | 171 | 10 |
| paint `curved_drag_nan` | 2 | 0 |
| paint `ground_grain` | 1 | 3 |
| paint doctests | 0 | 4 |
| easel (bin) | 21 | 0 |
| easel `determinism` | 2 | 0 |
| easel `smoke` | 1 | 0 |

New: the palette's pile drying and tube box tests, the pile tests for
passes and stipple, `a_knifed_pile_is_on_the_palette` (tally), the easel's
time tests (hand time of known verbs, hand time always on, `wait` returns
the time of day), `a_failed_chunk_leaves_brush_and_clock_alone`, the
journal's date test, `hand_time_is_deterministic_across_thread_counts`
and the smoke test. The smoke test (`crates/easel/tests/smoke.rs`): opens
a session, sets up a canvas, knifes two piles, lays a pass and strokes,
waits a day, looks whole and cropped with a grid and modes; confirms that
`undo`, `try`, `show`, `edit`, `undone` and `redo` are not commands, that
`look --dried/--probe/--show/--scale` are refused, and that `dry()`,
`mix()`, `sample()`, `probe()`, `show()`, `b:load("#hex")`, `color=` and a
color function fail and change nothing; checks that the replay matches
exactly (`easel check`), that `easel run` of the log gives the same PNG
bytes as `easel save`, that reopening resumes the two chunks and that the
journal only grows. Removed with the features they tested: the undo, edit,
try, overlay, crop-session and sitting tests, and the old
`hand_time.rs` tripwires (they replayed logs that use removed verbs).

`cargo build --release --workspace` passes.

## API before and after

| before | after |
|---|---|
| `canvas{style="friedrich", palette=, aspect=, seed=, size=, hand=}` | `canvas{size=, aspect=, linen=, ground={...}, seed=}` |
| `pal`, `palette()`, `pal:with/only/mix/aim/paint`, `paint()` | `tubes()`, `pile{{tube, parts}, ..., medium=}` |
| `b:load(color, amount, {...})` | `b:load(pile, amount)` |
| `work{color=, color_over=, pal=, aim=, medium=, paint=, jitter=, ...}` | `work{pile=, fill=, ...}` |
| `stipple{color=, aim=, fade=, pal=, medium=, ...}` | `stipple{pile=, feather=0, ...}` |
| `glaze(m, {color=, pigment=, coats=})` (dried the canvas first) | `glaze(m, {pile=, coats=})` (needs dry paint) |
| `varnish{color=, ...}`, `cracks{}`, `relief()` (dried first) | the same without `color=`, needing dry paint |
| `lose(m, {pal=, medium=, mix=, ...})` | `lose(m, {pile=, ...})` |
| `color`, `rgb`, `mix`, `gradient`, `shift`, `sample` | (none) |
| `show`, `probe` | (none) |
| `wait` (returned the clock), `dry`, `clock`, `hand_time`, `sitting`, `rest`, `timesheet` | `wait(minutes)` returns the time of day; `drying(x, y)` |
| `tree`, `sward`, `tree_in`, `tree_group`, `fir`, `fir_wood`, `rock`, `ridge`, `w:sky`, `w:clouds`, `w:ranges`, `haze` | (none) |
| `easel try/undo/undone/redo/show/edit`, `look --probe/--show/--dried/--relief/--scale` | (none); `easel note` added |

**The Lua API that remains:** `canvas`, `tubes`, `pile`, `brush` (`load`,
`reload`, `wipe`, `fullness`, `stroke`, `touch`, `mark_width`,
`pressure_for`), `work`, `blend`, `stipple`, `glaze`, `lose`, masks
(`everywhere`, `rect`, `ellipse`, `poly`, `below`, `above`, `ribbon`,
`mask` and their operations), `noise`, `worley`, `uneven`, `rand`,
`randn`, `lerp`, `clamp`, `smoothstep`, `outline`, `body_of`, `pencil`,
`chalk`, `erase`, `fix`, `drawing_mask`, `drawing_guide`, `body`,
`terrain`, `form`, `aerial`, `world` (spots, place, proxy, layer, view,
projection, depth masks, shadows), `wait`, `drying`, `varnish`, `cracks`,
`relief`, `print`, `W`, `H`.

## Exact replay

A log replays byte-identically: `easel check` in the smoke test,
`easel run` of the log against `easel save` (same PNG bytes), the session
tests (`replay_is_exact_and_failures_roll_back`,
`rollback_restores_tables_and_upvalues_exactly`) and the determinism tests
(object-keyed tables across processes; hand time at 1 and 4 threads).
Logs written by the easel before this branch (`paintings/lua/*.lua` on
main, `notes/loops/*.lua`) use removed verbs and don't run here.

## Open questions for the owner

1. **Sittings are gone.** The brief says the clock is never budgeted, so
   there is no sitting length and nothing refuses marks until a rest.
   `notes/principles.md` ("Against reward hacking" 4) asked for enforced
   sittings. Is removing them right?
2. **`wait` returns the time of day** (`day 3, 14:20`, from 09:00 on day
   1), and nothing else shows the clock. Keep it, or have `wait` return
   nothing?
3. **Glazing, varnishing, cracks and relief need the whole canvas dry.**
   The engine's glaze dries every film on the canvas, so the easel refuses
   while anything is wet anywhere: one slow patch of bone black holds up a
   glaze elsewhere for weeks of painting time. A local check would need an
   engine change to `Canvas::glaze`.
4. **The engine still has the inverse color search.** `Palette::mix`,
   `Palette::aim`, `Aim` and the presets' `.mixed(palette)` are in
   `crates/paint` (engine tests and the golden scene use them), where a
   painter can read them, though nothing in Lua reaches them. Remove them
   from the studio's engine?
5. **Automatic parts of the handlings stay** (astra_review 7): the
   presets' stroke character (bow, drift, broken strokes, swell, clump),
   look-and-fill (`fill`, on by default at coverage 1.5 or more with a
   loaded brush) and `edge={found=, soft=, lost=}` laid out by noise. I
   set stipple's `feather` to 0 by default in the easel (the engine's
   default is 0.6); should `fill` also default to off?
6. **Pile physics.** Each dip remixes the pile's proportions by 6%; the
   palette holds 16 piles; piles never skin over on the palette, however
   many days pass. Drying rates for green earth, Rinmann's green and
   copper green are estimates.
7. **The tube box.** 14 tubes, all the engine knows; no Naples yellow,
   ultramarine or madder lake (the engine has no data for them). "Tube" is
   a misnomer for the 1820s (collapsible tubes date from 1841; colors came
   in bladders). Rename?
8. **Varnish** is a fixed aged mastic tone (the painter chooses coats, not
   the resin); `cracks{}` ages the picture by centuries. Keep both in the
   studio?
9. **Crack walls show a fixed ground color**
   (`crates/paint/src/crack.rs`, `GROUND_WALL`), not the painter's chosen
   ground (astra_review 9). An engine change, left alone.
10. **The log can still be edited by hand.** With the session closed, a
    painter can delete or change chunks in `paintings/lua/<name>.lua` and
    reopen: an undo by the back door. The easel only keeps a copy of the
    file when it was changed while the session was open. A hash chain in
    the chunk markers would catch edits (not truncation).
11. **`easel run --width N`** renders the program at another width, which
    a painter could use as a higher-resolution preview. Kept for replay and
    delivery; the guide says a replay at another width is the same program
    at another grain. Which width is delivered?
12. **The journal's dates are UTC wall-clock time**, which shows the painter
    how long they have worked in real time. Use the painting's time of day
    instead?
13. **`body_of{}`** (a silhouette from a spine and widths) is generic, but
    its old examples were a sheep and figures. Kept.
14. **The materials note still identifies objects** by date, size and
    place ("two large canvases of 1808–10"), and some source URLs contain
    place or project names (e.g. `cdfriedrich.de`). The physics note's
    source 10 lost its URL (the URL named a painter).
15. **The names check is a fixed list** in the export script; a name not
    on it would pass.
16. **The `studio/` viewer** reads pi session logs, not easel logs, so it
    is unaffected; it isn't in the export.
