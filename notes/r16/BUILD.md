# Round 16: the studio build

Branch `r16-base` (from main `4abe5b4`), worktree `~/src/a/claude-paint-r16-base`.
This note lists every change, what was removed, what was kept and why,
the Lua API before and after, the test results and the owner's follow-up
decisions. The owner's task messages are authoritative; agent-authored
project notes are not additional owner rules. The painter briefs, the chain runner and the handoff reader
are not part of this build.

## In short

> The last section, **Binary-only studio**, replaces earlier text about
> the export, `easel run`, sessions, poured `glaze()`, `look --mode wet`
> and `drawing_mask()`.

- **The engine** is Round 15's cleaned `crates/paint` (from `r15-base`),
  plus three small additions made test-first: piles laid as knifed
  (`Handling::piled`, `Stipple::piled`), tubes that dry at their own rates
  (`Tube::drying`, `Mixture::laid`) and the tube box (`Palette::tube_box`).
  The follow-up removes automatic palette matching and presets' matching
  paths from the engine source, makes fill opt-in and makes glaze drying
  checks local to its mask. Palettes and styles have material names.
- **The easel** has no undo, edit, try, previews, probes, overlays, color
  functions, canvas sampling, `dry()`, sittings or subject generators.
  Hand time is always on; `wait(minutes)` takes any length and returns the
  time of day. Paint reaches the canvas only as piles knifed from named
  tubes (`pile{}`), and `canvas{}` takes the painter's size, linen and
  ground. Session logs append in place and are checked against a separate
  committed record before every request and reopen. Live painting and
  delivery use 2400 pixels; PNG looks are bounded whole views or native
  crops. Save encodes the live wet state rather than drying a copy.
  `easel note` appends painting-time entries to `notes/journal.md`.
- **The painter's guide** is `notes/easel_guide.md`; the old
  1437-line `crates/easel/README.md` is now a pointer to it.
- **The export**: `scripts/export_r16_studio friedrich|blank <dest>`.
- **Verified code revision `72f25e5`**: `cargo test --workspace` passes
  (197 tests, 14 ignored); `cargo build --release --workspace` passes.
  Ten developer-only removed-API compile checks and the standalone PNG
  viewer checks also pass. Subsequent changes are documentation only.
  Exact logs are listed under final verification below.

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
- `look` whole and `--crop` (source pixels at 1:1), `--size` for whole views.
- `--mode wet`: where the paint is open, setting, tacky or dry. It shows
  the real canvas's state, as a knuckle feels it.
- `--mode value`, `squint`, `mirror`: the paint as a painter looks at it
  (a monochrome viewing glass, half-closed eyes, a mirror). They transform
  the view of the real canvas; they show nothing that isn't there.
- `--grid [step]`: drawn on the PNG, never on the canvas, labeled in
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
  not a score; journal dates also use this painting time. The chunk reply is `ok ·
  chunk N (x s to compute)` (machine seconds, for planning chunk sizes),
  `status` is chunks, width and canvas setup, and the log's chunk markers
  are `--@ chunk N` (no clock).
- `drying(x, y)` stays: the paint's stage at a point, as touch tells a
  painter.
- `glaze()`, `varnish{}`, `cracks{}` and `relief()` used to dry the whole
  canvas first (the engine's `Canvas::glaze` and `relief` call `dry()`),
  which was a `dry()` shortcut. Glaze now checks only the covered area
  and identifies a wet spot by canvas coordinates. Varnish, cracks and
  relief still require the whole canvas touch-dry (`crates/easel/src/api.rs`).
  A glaze into wet paint is brushed: `work(m, {hand="glaze", pile=...})`.

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
  from the given pile, with the raw color field unused;
  `handling::tests::a_pass_from_a_pile_lays_the_pile`,
  `stipple::tests::stippling_from_a_pile_lays_the_pile`), `Tube::drying`
  with `Mixture::drying` and `Mixture::laid` (a pile dries at its tubes'
  rates mixed by volume; before this, every paint the easel laid dried at
  the average rate 1; `palette::tests::a_pile_dries_at_its_tubes_rate`).
  `Mixture::paint` was unchanged in the initial build. The follow-up
  migrates the golden scene to explicit piles, changing that fixture.
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
  writes bounded PNG views), `studio/` and research beyond the
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

`easel note '<text>'` (or `easel note -` from stdin) appends a line stamped
with the session's painting time, such as `- day 2, 09:40: <text>`, with
further lines indented under it, at the end of `notes/journal.md`
(`crates/easel/src/main.rs`). It opens the file for append only; nothing
in the easel rewrites it. The guide calls it "your working journal" and
says entries stay as written.

## 5. Tests

`cargo test --workspace` (debug-profile tests, optimized):

| suite | passed | ignored |
|---|---|---|
| paint (lib) | 159 | 7 |
| paint `curved_drag_nan` | 2 | 0 |
| paint `ground_grain` | 1 | 3 |
| paint doctests | 0 | 4 |
| easel (bin) | 25 | 0 |
| easel `delivery` | 2 | 0 |
| easel `session_integrity` | 5 | 0 |
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
| `work{color=, color_over=, pal=, aim=, medium=, paint=, jitter=, ...}` | `work{pile=, fill=false, ...}` |
| `stipple{color=, aim=, fade=, pal=, medium=, ...}` | `stipple{pile=, feather=0, ...}` |
| `glaze(m, {color=, pigment=, coats=})` (dried the canvas first) | `glaze(m, {pile=, coats=})` (needs touch-dry paint only beneath its coverage) |
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

## Owner decisions applied

The owner's follow-up resolves the earlier policy questions:

1. No enforced sittings. The clock measures physical drying and handwork,
   not a budget. The sitting section of `notes/principles.md` was written
   by an agent and is not an owner's rule.
2. `wait` keeps returning the painting's day and time of day.
3. Glaze needs touch-dry paint only beneath the area it covers. A wet
   patch elsewhere must remain wet and must not block that glaze.
   Varnish, cracks and relief keep the whole-canvas dry requirement.
4. Automatic tube selection and proportion matching must be absent from
   the exported engine source, not merely inaccessible through Lua.
5. `fill` defaults to off; explicit `fill=true` remains. Stipple feather
   stays at 0 by default in the easel.
6. A session log must detect hand edits and truncation on reopen and while
   live. Failed replay cannot silently discard later successful chunks.
7. Live painting and delivery use the same 2400-pixel-wide canvas. Looks
   are PNG: whole views at most 1600 pixels on the long side and below
   3 MB, crops at source resolution with each side at most 1200 pixels.
   The separate, unexported `scripts/peek` now also writes only PNG,
   bounds whole views and preserves 1:1 crops (`scripts/tests/peek.sh`).
8. Journal entries use painting time, not UTC wall time.
9. The small physics items remain as implemented: pile drying estimates,
   palette pile behavior, tube names, aged mastic, cracks and the fixed
   crack-wall ground color. No new physical model was requested here.

### Retained limits

- Local files are not a security boundary against someone who rewrites
  both the log and its trusted integrity state or changes the executable.
  Integrity checks are intended to refuse ordinary hand-edited session
  logs, not to police a hostile machine owner.
- `body_of{}` remains generic geometry from supplied points and widths.
- The materials note identifies some objects by date, dimensions or
  place. Source 10 in the physics note has no URL because its earlier URL
  included an artist's name. Research and small physics items are unchanged
  in this follow-up.
- The export's artist-name check is a fixed list. Build paths can appear
  in binaries, so blank export destinations should have neutral names.
- `studio/` is not exported and is unaffected.

## Follow-up implementation receipts

### Session integrity and journal

`Server::validate`, `Server::resume` and `Server::save_log` in
`crates/easel/src/main.rs` implement an equivalent integrity record rather
than a hash chain: the exact committed log is mirrored in
`out/easel/<name>/committed.lua`. Every live request compares both files
with the in-memory committed bytes. Reopening requires matching files,
a successful replay of every chunk and canonical serialization. Missing,
edited and shortened logs are refused explicitly. A failed replay never
becomes a partial session that can overwrite later chunks.

Successful chunks append only their new suffix to the Lua log, sync it,
then sync and rename the pending integrity record. A crash between the
two writes fails closed; there is no automatic recovery. Directory
metadata is not explicitly synced. Earlier logs without an integrity
record cannot reopen. The record detects edits to the log alone, not
coordinated changes to both local files or whole-directory rollback.

`easel note` now goes through the selected open session and uses
`time::time_of_day(clock)`. Notes neither advance painting time nor enter
the replay program. An integrity refusal also blocks notes.

Regression ownership: `crates/easel/tests/session_integrity.rs` covers
live tampering, closed-session edits, truncated logs, failed replay,
fixed live width and virtual journal timestamps. The initial live-edit
and width regressions were authored before implementation. Additional
lifecycle regressions were then demonstrated against the unchanged
pre-change binary, not all authored before implementation. Their red
and green receipts are `~/tmp/r16-build-247d8e71/integrity-red.log`,
`session-baseline-red.log` and `session-fixed-green.log` in that same
folder.

### Viewing and delivery

`LIVE_WIDTH` is 2400. Neither `open` nor `run` accepts another width;
`run` also no longer has a crop-render or margin option. `look --crop`
is a window of the actual pixels, never a second render. Whole looks
are RGB PNGs at most 1600 pixels on the long side (1000 by default),
reduced further until at most 3,000,000 bytes. Crops keep 1:1 pixels and
refuse sides over 1200 pixels. Their PNGs can exceed 3 MB; the owner's
explicit byte bound applies to whole views. Frames and `run --look`
also write PNG. JPEG encoding dependencies were removed.

**Changed judgment:** merely switching JPEG to PNG was insufficient.
The old `save` called `Canvas::save` on a clone, which dried it before
encoding. A wet-paint regression showed a difference of up to 106
8-bit channel levels against a native live look. `deliver` now encodes
`Canvas::seen()` without drying; save and replay share that writer.
Dithering is omitted so native look pixels and delivery pixels agree.
`crates/easel/tests/delivery.rs` owns that contract and byte-identical
saved/replayed PNGs. `crates/easel/src/look.rs` tests dimensions, native
crop pixels and the whole-view byte cap.

The diagnostic `run --dump-surface` remains for the cross-process exact
replay test; it writes physical surface heights, not a painted preview.

### Workflow limitations

The delivery subagent validated an intermediate compiling revision in a
registered scratch worktree at
`~/tmp/r16-build-247d8e71/prefix-worktree`. This violated the
instruction to work only in the designated worktree. It was reported,
not silently removed; no main branch or pre-existing worktree was edited.
The final integrated verification below is run in the designated
`claude-paint-r16-base` worktree and in the requested fresh exports.

### Engine removals and local glazing

`725ca09` deletes `Palette::mix`, `Palette::aim`, `aim_for`, `paint` and
`paint_for`, `Canvas::aim`, `Aim`, handling and stipple matching options,
`Handling::mixed` and the handling presets' palette-search paths. The
spectral target-search path is also removed. Explicit proportions via
`Palette::pile`, pigment optics, forward spectral mixing and ordinary
physical color compositing remain. Those calculate what supplied paint
makes, not which paint a painter should choose.

Rust callers and the golden fixture now supply explicit piles. The
old golden image changed with that fixture migration; the earlier
base-build statement that the golden was unchanged no longer describes
this follow-up. `Style::glaze()` has no medium argument; the pile supplies
its medium. `Handling::fill` with no override is now off; explicit true
still runs the gap-filling dabs. The defaults regression compares the
actual painted result with explicit false and true.

`e7dae39` removes the implicit `Canvas::glaze` dry operation. The Lua
boundary checks every mask-covered pixel for touch-dry substrate before
any palette trip or glaze. A refusal names a coordinate in canvas units.
Wet paint outside that mask is not cured. The regression exercises local
glazing beside wet paint, refusal over wet paint, no-op rollback and the
unchanged full-canvas finishing gate.

The ten removed-API compile-failure cases live in
`notes/r16/removed_palette_api.md`, outside the painter export, rather
than in engine rustdoc that would advertise removed matching APIs.
These are run separately with rustdoc against the built paint library.
The painter-facing palette documentation describes only explicit mixing.

## Final verification of the follow-up

The final code revision is `72f25e5`; `f4bc387` updates only the guide.
Validation ran in `~/src/a/claude-paint-r16-base` with persistent scratch
`~/tmp/r16-build-247d8e71` and Command Line Tools selected.

- `cargo test --workspace`: **197 passed, 14 ignored, 0 failed**.
  Suite counts are in the table above. Receipt:
  `~/tmp/r16-build-247d8e71/final-tests.log`.
- `cargo build --release --workspace`: passed. Receipt:
  `~/tmp/r16-build-247d8e71/final-build.log`.
- `rustdoc --test notes/r16/removed_palette_api.md --edition 2024
  --extern paint=target/debug/deps/libpaint-4089c38b608b7177.rlib
  -L dependency=target/debug/deps`: **10 passed**, verifying removed APIs
  against the library built by the workspace tests. Receipt:
  `~/tmp/r16-build-247d8e71/removed-api-tests.log`.
- `sh scripts/tests/peek.sh`: passed. Whole views, the byte cap under
  high-entropy input, 1:1 crops and invalid output/crop rejection are
  covered. The high-entropy whole view was 2,799,553 bytes. Artifacts:
  `~/tmp/r16-build-247d8e71/peek-test-33805`.

Follow-up commits (after the previous build's `2a78d0f`):

```
7727075 Make the standalone image viewer write bounded PNG views
464e34c Exercise the PNG byte cap with a high-entropy image
f3acf20 Bound look PNGs and preserve native crop pixels
341056a Reject altered session logs and partial replays; fix live size and journal time
725ca09 Remove engine color matching and make passage filling opt-in
fcdc0d9 Deliver the canvas as seen: save and run write live pixels at 2400px only
e7dae39 Check glaze drying only beneath its mask without curing other paint
669209c Keep removed-API checks outside painter source and correct replay header
72f25e5 Move remaining removed matching API checks out of exported engine docs
f4bc387 Document fixed-resolution PNG painting, local glazing and protected logs
```

Both fresh exports of `f4bc387` passed, including a fresh
`cargo build --release -p easel` in each destination:

| profile | destination | receipt in `~/tmp/r16-build-247d8e71` |
|---|---|---|
| blank | `~/tmp/r16-build-247d8e71/blank-followup-final` | `blank-followup-export.log` |
| friedrich | `~/tmp/r16-build-247d8e71/materials-followup-final` | `materials-followup-export.log` |

Both have an empty journal, no `.git` or `out`, no removed matching API
references in `crates/` and only the intended notes. `target/` contains
only the export's fresh build artifacts. The blank binary's strings also
passed the script's artist-name pattern check; the zero-hit receipt is
`~/tmp/r16-build-247d8e71/blank-binary-name-audit.log`.

Export usage is unchanged:

```sh
scripts/export_r16_studio blank <empty-or-new-destination>
scripts/export_r16_studio friedrich <empty-or-new-destination>
```

There are no unanswered policy questions from the owner's follow-up.
The local-integrity trust limit, fail-closed crash recovery, native crop
byte size and scratch-worktree deviation are documented above rather
than treated as new policy decisions. All later commits update only this
unexported build record; the tested export contents are unchanged.

## Binary-only studio (after the overnight review)

The owner took six cheap fixes from `~/tmp/gallery-fcf9c110/r16/review/REVIEW.md`
and left the rest (palette-time flush, masks as images, relief, service
isolation) alone. Nothing here is a sandbox: a painter with a shell can
still copy files, decode PNGs or restore the log and its record together.

1. **Export ships no source.** `scripts/export_r16_studio` builds the
   painter's easel from a `git archive` of the branch in
   `target/studio-build/<commit>` (`cargo build --release -p easel
   --no-default-features`) and ships `bin/easel`, `notes/` (guide, research
   per profile, empty `journal.md`), an empty `paintings/lua/` and
   `THIRD_PARTY_NOTICES.md`. No crates, Cargo files, tests or README. The
   script fails on any other file, checks the notes and the binary's
   strings for painters' names, then runs a session in a throwaway copy
   from `/` (open, canvas, stroke, look, save, check, close) and confirms
   `run` is refused.
2. **Two builds from one crate** (`crates/easel/Cargo.toml`, feature
   `replay`, on by default). The **replay build** (`cargo build --release
   -p easel`, `target/release/easel` in r16-base) keeps `run`,
   `--dump-surface`, `hash-probe`, `EASEL_ROOT`, `-s`/`EASEL_SESSION` and
   named sessions; delivery renders and tests use it. The **painter build**
   (`--no-default-features`) compiles those out: one session, fixed name
   `painting`; `open` takes no name; `-s` is an error; the studio root is
   the parent of the executable's folder (`current_exe`, canonicalized),
   whatever the cwd or environment. The log header no longer names
   `easel run` (`f225d32`).
3. **No poured glaze.** The Lua `glaze()` is gone; `work(m, {hand="glaze",
   pile=p})` brushes a glaze into the wet-paint model. Varnish, cracks and
   relief are unchanged and need the whole canvas dry.
4. **`wait` limits.** `wait(minutes)` refuses non-finite, negative and
   anything above 5,259,600 minutes (10 years) before time passes.
   `Canvas::wait` panics on a non-finite value instead of calling
   `Canvas::dry` (the easel reports a failed chunk that changed nothing).
5. **Removed views.** `look --mode wet` (and `drying`/`stages`) and
   `drawing_mask()`. Value, squint, mirror, grid, crop and `drying(x, y)`
   stay.
6. **Guide.** Commands use `bin/easel` from the studio folder; the Starting
   block ends with `note`, `save` and `close` and names
   `out/easel/painting/painting.png` and `paintings/lua/painting.lua`.
   Ground and pile examples are `<tube>`/`<parts>` placeholders (the one
   runnable canvas uses a single tube, lead white); the world example is a
   camera over a flat plane with one block, other `world{}` options only
   listed; finishing is titled optional; the Time section says only
   painting operations and `wait` advance the clock, not real time.

Left as they are: the hidden `serve` command (painter build refuses any
name but `painting`); `save <relative path>` resolves against the cwd of
`open`; a symlink to `bin/easel` uses the target's studio.

### Commits

```
40ab8a3 Limit wait to 0..10 years and stop an infinite engine wait from drying
c917ef3 Remove the poured glaze() global; glazes are brushed
9e6327a Remove look --mode wet and drawing_mask() from the easel
7b7cfa8 Build a painter easel without the replay feature
f225d32 Log header without the replay command the painter build lacks
afda9b3 Binary-only studio export; guide for one painting at bin/easel
```

### Verification (at `afda9b3`)

- `cargo test --workspace`: 198 passed, 14 ignored (easel unit 25,
  delivery 2, determinism 2, session_integrity 5, smoke 1, painter 0
  compiled out; paint lib 160 + 7 ignored, curved_drag_nan 2, ground_grain
  1 + 3 ignored, doc-tests 4 ignored). Log: `~/tmp/r16-build-247d8e71/cheap-tests.log`.
- `cargo test -p easel --no-default-features`: easel unit 25, painter 1
  (the others are gated to `replay`). Log: `cheap-painter-tests.log`.
- `cargo build --release --workspace`: passes (`cheap-build.log`).
- Exports: `~/tmp/r16-build-247d8e71/studio-blank-binonly` and
  `studio-friedrich-binonly`, each `bin/easel` (5.3 MB), the notes, the
  notice and an empty `paintings/lua/`. The guide's Starting commands ran
  as printed in a copy of the blank studio.
- Red logs for the new behavior: `r16cheap-wait-red.log`,
  `r16cheap-removed-api-smoke-red.log`, `r16cheap-painter-red.log`.
