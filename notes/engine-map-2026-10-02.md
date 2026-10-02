# Engine map: claude-paint, for the refactor plan (2026-10-02)

Read-only study of `~/src/a/claude-paint` at `c4a558c`. Receipts are `file:line` (paths under `crates/` unless noted), commands run or quoted docs. Scratch evidence (scripts, digests) is in `~/tmp/sw-scope/`. The raw `sample` dumps the profile receipts name (`s_*_*.txt`, `sample_frdc2_run.txt`, `s_b08dc3.txt`, ~150 MB) were deleted after summarizing; `sample <pid> 10 -file <f>` and the scripts there reproduce them.

**Caveat on every timing.** All timings were taken on a loaded machine: `uptime` said `load average: 35.02` on 12 cores (`sysctl hw.ncpu` → 12). Three check/finish replays and two live painters were running.

---

## A. How the engine is cut today

### A1. Modules (line counts from `wc -l`)

**`crates/paint/src` (engine, ~20k lines)**

| module | lines | owns |
|---|---|---|
| `canvas.rs` | 645 | `Frame` (units↔pixels, crop windows), `Canvas` struct (all per-pixel buffers, `canvas.rs:171-207`), `prime` (ground layer), `glaze` (instant KM film), `relief`, `save` |
| `surface.rs` | 558 | `Linen`, `build_support` (weave height, `:215`), leveling `settle`/`settle_for` (`:243-320`), `settle_film` for glaze/varnish (`:357`), `box_blur` (`:158`), `COAT_UM`=25 µm |
| `wet.rs` | 324 | `Paint` (masstone, scatter, stiff, drying; `:49-64`), `Wet` (the one wet layer per pixel, `:120-142`), `seen()`/`look_px` (wet over dry, `:217-232`) |
| `drying.rs` | 932 | clock, per-pixel `Px` drying state (`:189-210`), `wait` (`:253`), `dry` (`:392`), `absorb` (`:441`), `bake` (`:486`), stages (`:376`), drier constants |
| `bristle.rs` | 2239 | `Tool`, `Held`, `Bristle`, `Gesture`, `Touch`; `drag`/`drag_on` (`:679`, `:861`), `touch`/`touch_on` (`:1548`, `:1568`), the per-bristle kernel `exchange` (`:1131`), contact surface `contact_level` (`:38`), `Surf` raw-pointer view (`:542-572`) |
| `handling.rs` | 1579 | `Handling` (a `work` pass's options), `work_with` (`:410`): place → plan strokes → tiles (`run_plans` `:718`) → `fill_gaps` (`:562`), `cut_in_edges` (`:651`) |
| `sched.rs` | 248 | `run_ordered` (tile DAG, bit-identical to serial order, `:49`), `paint_pass` (hand-time slices with `wait` between, `:152`) |
| `stipple.rs` | 774 | stipple passes (touches) through `paint_pass` |
| `fence.rs`, `edge.rs` | 274, 109 | `edge=` fences (found/soft/lost), contour tracing |
| `graphite.rs` | 1065 | pencil/chalk `Drawing` (per-pixel `Cell`, `:135-163`), `draw` (`:515`), `erase` (`:623`), `fix_drawing` (`:663`) |
| `palette.rs` | 655 | tube catalog (cfg-gated per box, `:62+`), `Palette::pile`, `Mixture::paint/laid` (medium, `:478-492`) |
| `pigment.rs` | 235 | RGB Kubelka–Munk `Pigment` (K, S per channel), `over` (`:177`) |
| `spectral.rs` | 387 | 38-band spectral KM. **Unused**: `grep -rn spectral crates` finds only `lib.rs:30 pub mod spectral;` |
| `color.rs`, `noise.rs`, `rng.rs`, `path.rs`, `shape.rs`, `mask.rs` | 116–529 | helpers. `Mask` = whole-canvas `Vec<f32>` (`mask.rs:9-12`) |
| `crack.rs` | 2030 | craquelure network in mm, rasterized into height/px (`crack` `:1346`) |
| `tally.rs` | 587 | hand-time ledger, `set_hand_time`, `hand_pass` → `wait` (`:346-351`) |
| `checkpoint.rs` | 435 | exact canvas save/restore `write_state`/`read_state` (`:101`, `:182`) |
| `form.rs`, `scene.rs`, `outline.rs`, `hand.rs`, `style.rs` | 73–2005 | solids/lighting, world/perspective, drawn outlines, local gesture frames, `Style::prepare` (support + grounds, `style.rs:160-192`, `brush_ground` `:199`) |
| `tests.rs` | 676 | characterization tests, golden fixture fingerprint |

**`crates/easel/src` (Lua session, ~19k lines incl. legacy)**

| module | lines | owns |
|---|---|---|
| `main.rs` | 1184 | CLI, unix-socket server, `run` (replay, `:978`), `deliver` (PNG of `seen()`, `:965`), `--state-digest` (`:1100`), `LIVE_WIDTH = 2400` (`:44`) |
| `session.rs` | 1460 | Lua state, the log, `run(chunk)` with snapshot/rollback (`:253-333`), `snap` clones the whole canvas (`:182-193`), `CHUNK_LIMIT` 600 s (`:33`), engine/box marks (`:23`, `:27`, `:697`) |
| `api.rs` | 1541 | the Lua verbs: `canvas{}` (`:1197`), `pile{}` (`:1261`), `brush`, `Brush:stroke/touch/load/wipe` (`:362-460`), `work` (`:805`), `blend` (`:1412`), `stipple` (`:1022`), `wait` (`:1426`), `drying` (`:1438`), masks, noise; per-chunk reseeding `begin` (`:65-71`) |
| `time.rs` | 225 | `verb()` hand-time rules; `SLICE_MIN` 15 (`:22`), `GRAIN_MIN` 1 min (`:25`) |
| `draw_pencil.rs`, `draw_outline.rs`, `draw_edges.rs` | 311, 423, 184 | pencil/chalk/erase/fix, `outline{}`, `lose()` |
| `form.rs`, `world.rs`, `depth.rs` | 513, 417, 440 | Lua wrappers for solids, world and depth |
| `look.rs` | 611 | bounded PNG views of `seen()` |
| `finish.rs` | 123 | `varnish{}`, `cracks{}`, `relief()` (feature `finish`, not in painter builds) |
| `check.rs` | 191 | `easel check`: a second full replay on its own thread (`:105-171`) |
| `frames.rs` | 234 | hand-time frames for replay clips |
| `legacy/` | ~9,000 | old API (`canvas{style=}`): trees, rocks, firs, sky. Its own doc says "Replays are not pixel-identical to the old renders" (`legacy/mod.rs:34-35`) |
| `prelude.lua` | 383 | deterministic `pairs`/`next`, hidden addresses |
| `heap.lua` | 121 | Lua heap snapshot/restore for rollback |

### A2. How a painter action travels

| verb | path |
|---|---|
| `canvas{}` | `api.rs:1197` → `Style::prepare` (`style.rs:160`) → `with_size_mm`/`with_linen` → `build_support` (`surface.rs:215`, weave height) → per ground `prime` (`canvas.rs:281`, knife/roller) or `brush_ground` (`style.rs:199`: a whole-canvas `work` pass, coverage 3.5, `fill`) → `time::start` sets 15-min hand slices |
| `pile{}` | `api.rs:1261` → `Palette::pile` (Mixbox latent mix by parts × strength) → `Mixture`; `time::knife` charges hand time |
| `work` / `blend` / glaze hand | `api.rs:1410/1412` → `api::work` (`:805`, parse into `Handling`) → `time::verb(Pass)` (`api.rs:973-976`) → `Canvas::work_with` (`handling.rs:410`): `place` centers, `hand_trace` paths, `finish_plan` (footprints) → `run_plans` (`:718`): tiles of 2× max reach → `paint_pass` (`sched.rs:152`): batches of 15 min hand time; each batch `surf()` (rebuilds the contact surface if the height changed, `bristle.rs:636-649`) → `run_ordered` → per tile `Held::new(... seed ^ tile index)` (`handling.rs:781`) → `drag_on` → per step × per bristle `exchange` (deposit, pickup, plough on the wet buffers) → between batches `hand_pass` → `wait` (`tally.rs:346`). Then `fill_gaps`, `cut_in_edges` |
| `stipple` | `api.rs:1022` → `Canvas::stipple_with` (`stipple.rs:221`) → `paint_pass` → `touch_on` |
| `b:stroke` / `b:touch` | `api.rs:401-460` → `time::verb(Marks)` → `Canvas::drag` (`bristle.rs:679`) / `touch` (`:1548`): **one brush on the calling thread**; a `wait` every 1 min of hand time (`time.rs:25`) |
| `wait` | `api.rs:1426` → `Canvas::wait` (`drying.rs:253`): `absorb` (film thickness by two box blurs over the dirty box), age cures, `bake` (level with `settle_for`, composite into `px`, set tack) |
| `drying(x,y)` | `api.rs:1438` → flush hand time → `drying_at` (`drying.rs:350`) |
| `look` | `main.rs:855` → `look::render` (`look.rs:338`) of `seen()` (wet over dry, `wet.rs:229`) |
| pencil/chalk | `draw_pencil.rs:60` (`Verb::Pass`) → `Canvas::draw` (`graphite.rs:515`): coverage per pixel by depth below local tops of `height` |
| `erase` / `fix` | `draw_pencil.rs:257-282` → ribbon mask, blur → `Canvas::erase` (`graphite.rs:623`, serial whole-canvas loop) / `fix_drawing` (`:663`) |

### A3. Per-pixel state (all at 2400 px wide, the whole canvas, `main.rs:44`, `:988`)

| buffer | type | bytes/px | receipt |
|---|---|---|---|
| dry color `px` | `[f32;3]` linear RGB | 12 | `canvas.rs:179` |
| surface `height` (weave + grounds + films), µm | f32 | 4 | `canvas.rs:181` |
| `film` (coats laid, bookkeeping) | f32 | 4 | `canvas.rs:183` |
| wet `vol`, `lat` (Mixbox latent, 7 floats), `hide` (scatter, stiff, drying), `stroke`, `touched`, `floor`, `cover` | | 60 | `wet.rs:120-135`; `LATENT_SIZE = 7` (mixbox-2.0.0 `lib.rs:53`) |
| drying `Px` (cure, lev, seen, sub, srate, th) | 6×f32 | 24 | `drying.rs:189-210`, allocated at first `wait` |
| contact surface `base` (derived) | f32 | 4 | `canvas.rs:195`, `bristle.rs:636-649` |
| **canvas total** | | **108** | sum of the above |
| graphite `Cell` (a, r, lift, floor, film) + guide (+floor) | | 20 + 4 (+4) | `graphite.rs:135-163`, only once something is drawn |
| each Lua mask | f32 | 4 | `mask.rs:9-12` |

Scalars: `ground_um`, `linen`, `mm_per_unit`, `engine` (`canvas.rs:186-206`). **No paint layer history is kept**: a film that sets is composited into `px` (`drying.rs:570-596`) and its wet record is zeroed.

Sizes: FRDC1 (`aspect=0.72`) is 2400×3333 = 8.0 M px → ~864 MB canvas. INNS (`aspect=1.5`) is 3.84 M px → ~415 MB. A live session clones the canvas per chunk for rollback (`session.rs:192`), which doubles that. The FRDC2 finish replay's sampled footprint was 4.5 GB (`sample` header: "Physical footprint: 4.5G"). I did not check the breakdown (masks and Lua tables are the likely remainder).

Physical radii are in mm, so their pixel radii depend on canvas size: contact level 1.5 mm (`bristle.rs:643`), film thickness `FILM_MM` 1.25 (`drying.rs:63`), leveling bands (`surface.rs:92-113`). FRDC1 (480 mm wide, 0.2 mm/px) gets ~2.4× larger kernels than INNS (1140 mm).

---

## B. The three seams

### B1. Support and texture: **narrow, one height field**
- Lives in: `Linen` + `build_support` (`surface.rs:37-55`, `:215-237`, `linen_um` `:464`) → `Canvas.height`. Grounds go through `prime` (`canvas.rs:281`, noise + `settle` + KM over `px`) or `brush_ground` (`style.rs:199`). `ground_um` is a scalar (`canvas.rs:186`).
- Tops/hollows reach the tools only through `height`:
  - brush deposit/pickup/dry brush: `base = 0.5 + (height − running-median level)/(2·TOOTH_UM)` (`bristle.rs:636-649`, `TOOTH_UM` 60 `:29`). In `exchange` a bristle touches where `smoothstep(th − give, th + 0.2, base + 0.35·vol)` (`bristle.rs:1240-1250`). A loaded hair reaches deeper (`wick`, `:1205-1206`). Fences lift hairs off the hollows (`:1248-1253`).
  - pencil: depth below the local max of `height` in a tooth window (`graphite.rs:561-572`). Eraser: the same "top" test with a 0.35 mm window (`graphite.rs:635-651`).
  - leveling: `settle_for` levels relative to `height` (`surface.rs:250-320`). `settle_film` lets glazes pool in hollows (`:357-440`).
  - cracks: weave pitch from `linen` (default 14×12 if none) and stretcher corners (`crack.rs:1351`, header `:28-36`).
- Entanglement: writers of `height` are `build_support`, `raise` (settle), `crack` (`crack.rs:1358`). Every writer bumps `surf_gen` (`surface.rs:236`, `:333`, `crack.rs:1357`). Readers are listed above. `Style.linen` is not optional, and `canvas{}` requires `linen=` (`api.rs:1205-1220`).

### B2. Paint film and medium: **wide; spread over wet, drying, surface, palette and bristle**
- Amounts: one wet layer per pixel, `vol` in coats of 25 µm (`surface.rs:24`). Pigment is a Mixbox latent mixed by volume (`wet.rs:157-170`, `bristle.rs:588-610`). There is **no per-pigment identity and no oil or solvent amount**.
- `medium` = scatter × (1−m) and stiffness × (1−m)², drying rate unchanged (`palette.rs:478-492`). "Fat" exists only as low stiffness: the drying-rate factor is `1 + FAT·(1−stiff)` (`drying.rs:127-131`). Rheology comes from stiffness alone (`surface.rs:59`).
- Drying: `rate(vol_thickness, stiff, drying)` (`drying.rs:127`). `GEL` = 0.15 (`:46`). Stage (`drying.rs:376-388`): vol ≥ 1e-3 and cure < 0.075 → open; vol ≥ 1e-3 and cure ≥ 0.075 → setting; else `sub` < 1 → tacky; else dry. A film past `GEL` levels for `lev` seconds and **bakes into `px`** (`bake` `:486-631`). Its tack survives only as `sub`/`srate`.
- Engine-2 behavior: paint laid mixes into the cure at once (`bristle.rs:604-608`). Version branches: `drying.rs:451`, `bristle.rs:668`, `scene.rs:522`, `scene.rs:636`.
- Optics: RGB KM per channel, after Curtis et al. 1997 (`pigment.rs:1-3`, `:177-187`). Wet-over-dry with sub-pixel cover (`wet.rs:179-189`, `bead_cover` `:207`). `spectral.rs` is unused.
- Cracks: `crack.rs` grows a network in mm from `ground_um` and the weave pitch, then edits `height` and `px` (`:1346-1370`).
- Entanglement: `Surf` (`bristle.rs:542-572`) holds raw pointers to 10 wet/drying buffers, and `surf()` asserts their lengths (`:631-635`). `drying.rs` reads `wet.stroke/touched` ids to tell which films were worked (`:451-453`). `graphite.rs` reads `film` and `wet.vol` to decide sealing (`graphite.rs:579-590`). The checkpoint format pins this layout (`checkpoint.rs:17-34`, `MAGIC "PAINTCK8"`).

### B3. Instruments: **one kernel, `exchange`, shared by every brush verb**
- Bristle brush: `Tool` presets with 14–160 bristles (`bristle.rs:130-235`; badger 160, hog flat 140, filbert 120). The step is `max(rb, 1.25)` px (`:888`). Per step, every touching bristle runs `exchange` over its capsule. Pass 1 computes contact weights; pass 2 does pickup (`pickup·hunger·fluid`, capped by a per-stroke `floor`), then deposit (load share by distance traveled, ×`grab`/`stick` on tacky paint), then plough to neighbors (`:1314-1422`).
- Lifting with a clean brush = the same pickup, with `hunger` up to 1 for an empty bristle (`:1296`). Blender = `Style::blend` (badger, `dips(3, 0.0, 0.9)`, `style.rs:354-363`). Stippling = `touch_on` with a fixed film-splitting deposit (`bristle.rs:1568`).
- Graphite eraser: the only removal tool. It works because graphite keeps its own per-pixel `Cell` above `px` and can un-composite it (`uncover`/`cover`, `graphite.rs:606`, `:655-657`). Paint has no such layer.

### B4. The planned features against today's structure

| feature | would reuse | would change | in the way |
|---|---|---|---|
| **Rag** (wipe wet) | `Surf::take` (`bristle.rs:616`), `base` contact (tops wiped, hollows keep paint), mask footprint | a new verb with its own contact kernel (cloth, not bristles), `tally` pricing | works on open/setting paint only. Tacky paint is already baked into `px` (`drying.rs:486+`), so a rag can't smear it |
| **Knife, scraping wet/tacky** | `Surf::take` + plough code (`bristle.rs:1363-1420`) | a rigid-blade kernel (flat contact to a height, not bristle capsules) | tacky films are not in the wet layer (baked). The per-stroke `floor` (`bristle.rs:1330-1333`) stops one pass lifting everything |
| **Blade/abrasive, dry paint** | `height` edits (as `crack.rs:1358`) | needs the color under the removed paint | **no layer history**: dry paint is RGB + height + film count only (`canvas.rs:179-183`). Needs a per-pixel record of at least the top dry film (pigment + thickness) to un-composite KM, like graphite's `Cell` |
| **Scratch through wet** (handle, thumbnail) | `exchange` pickup/plough, a 1-hair `Tool` | contact must reach the ground regardless of `base` | `exchange` only touches where reach clears the relief (`bristle.rs:1240-1250`) and lifts at most `pickup` per pass |
| **Painting knife (laying)** | `Surf::add` (`bristle.rs:588`), `rheology` of stiff paint (keeps a flat top) | a slab deposit (flat top relative to `height`, ridges at edges) | the deposit model is per-bristle, proportional to distance (`bristle.rs:1272-1290`) |
| **Thinner (turps) / drier** | drier: `Mixture.drying` already exists (`palette.rs:454`), so a multiplier is a palette change. Thinner: `Prop` mixing is per-component (`wet.rs:36-40`) | a solvent fraction per pixel (and per bristle) that evaporates in `wait`; lean vs fat in `rate`/`rheology` | `medium` is not separable into oil and solvent (`palette.rs:478-485`). Adding a `Prop` component changes the checkpoint and `--state-digest` bytes (`main.rs:1096-1098`: "a field added … changes the digests") |
| **Per-studio tools** | box mechanism: cargo features + `cfg` (`crates/paint/build.rs:1-24`, `palette.rs:62+`), `scripts/export_r16_studio` builds per profile | `cfg` on Lua verb registration in `api.rs` `install` | none found |
| **Panel** (rigid, chalk or oil ground) | `Canvas` with `linen: None` is already flat (`build_support` returns early, `surface.rs:216`); `prime` for grounds | a support enum (linen / panel + wood-grain or smooth height); chalk ground absorbency (oil sinking in) | `Style.linen` and `canvas{linen=}` are mandatory (`api.rs:1217`). Cracks assume weave pitch and stretcher corners (`crack.rs:1351`, `:34-36`). No absorbency term in drying |
| **Watercolor on paper** (2+ papers) | `height` + `base` for paper tooth; KM with low S; `Tool::round_sable` (`bristle.rs:151`); hand-time clock | a water layer with flow/diffusion (time-stepped, as in Curtis 1997), pigment in suspension vs deposited, per-pigment granulation/staining, evaporation clock, rewetting of dry pigment | single wet layer with mixed latents (no pigment identity). Dry paint baked into `px` (lifting/rewetting impossible). Drying is oxidation (`drying.rs:1-30`), and hand slices are 15 min (`time.rs:22`), too coarse for minutes-scale drying. Linen mandatory |

Adding any of these as **new verbs or options** need not change old pixels, provided existing code paths keep their float order and rectangles (see E).

---

## C. Where the time goes

### C1. Recorded finish replays (`notes/round*/runner/run/*/p*_finish_err.txt`)

| painting | chunks | total | median/chunk | top 50 chunks | by quarter of log |
|---|---|---|---|---|---|
| FRDC1 (round21) | 572 | 12,837 s (3.57 h) | 8.2 s | 40% | 11/15/37/38% |
| FRDC2 (round21) | 324 | 2,340 s (0.65 h) | 3.0 s | 56% | 32/27/36/6% |
| SONF1 (round21.5) | 511 | 5,861 s (1.63 h) | 3.5 s | 59% | 34/29/24/13% |
| INNS1 (round22.1) | 312 | 1,977 s (0.55 h) | 1.6 s | 69% | 30/22/31/17% |

(`~/tmp/sw-scope/chunks.py`; FRDC2's run finished during this study.)

- **The finishing chunk itself is cheap**: the last chunk (dry, `varnish`, `cracks`) took 2.33 s (FRDC1), 4.32 s (FRDC2), 4.09 s (SONF1) and 8.75 s (INNS1). The hours are the replay.
- **Chunk 1** (`canvas{}` with a brushed ground) takes 57.8 s (FRDC1), 43.9 s (FRDC2) and 43.7 s (INNS1); SONF1, with knife and roller grounds only, takes 0.56 s. 38 of the 49 studio logs use `apply="brush"` in chunk 1 (grep over `~/src/a/paint-studio-*/paintings/lua/painting.lua`).
- **Replay = live speed.** For FRDC1's last 68 live chunks, live `do` total was 1,793 s and the replay of the same chunks was 1,625 s (ratio 0.91, `live.py` on `~/src/a/paint-studio-35c488/out/easel/painting/server.log`). The chunks are intrinsically slow, not slower on replay.
- **Slow chunks are `work`/`blend` passes.** Time share of chunks containing each verb (static census, `census.py`): FRDC1 `work` 92%, `blend` 20%, `stroke` 8%. SONF1 `blend` 73%, `work` 57%. The slowest are e.g. FRDC1 #394 (177 s: `work(..., hand="hatch", coverage=4.0)` ×2 over a grown mask), #462 (163 s: one `stipple`, coverage 3) and SONF1 #177 (145 s: three `blend`s).

### C2. Profiles (`sample`, 10 s windows; 3 check replays × 3 windows, the FRDC2 finish and my own 35-chunk replay)

| question | answer | receipt |
|---|---|---|
| what the Lua thread does | 91.9% inside `api::work`; 85.6% of all Lua-thread samples are `work` waiting on the tile scheduler (`run_ordered`) | `verbs.py s_*_*.txt sample_frdc2_run.txt`; b08dc3 whole run: 81.5% |
| how busy the 12 workers are | 2–4 cores busy on average: busy share 14–29% of 13 threads (excluding the check servers' socket thread, idle in `accept`); one window 47% during a `contact_level` rebuild. `sample` also counts runnable threads the loaded OS had descheduled | `prof.py` per sample |
| what busy workers compute | `bristle::exchange` plus tile code inlined into `run_ordered` (the `exchange` kernel and `Fence::at`). `contact_level` 1–27% (running-median `memmove`). `box_blur` 2–5% | `leaf.py`, `prof.py` |
| main thread in late FRDC chunks | 73% waiting on tiles, 15% in `hand_pass`→`wait`→`bake`/`settle_for`, 8.7% in `contact_level` | `tree.py sample_frdc2_run.txt 0` |
| single-threaded? | **Largely.** Lua is one thread. A `work` pass parallelizes only over non-overlapping tiles, and typical passes have **1–3 non-empty tiles** (`PAINT_DEBUG=1`: e.g. `work: 31 strokes, reach 75x36, tiles 7x11` → `2 tiles`). `b:stroke`/`b:touch` are serial. Per-canvas ops (blurs, `wait` loops, `seen`) use rayon | `handling.rs:718-730`, `sched.rs:173-176`, `bristle.rs:679-693` |
| measured parallel speedup | 5-chunk replay (`paint-studio-125996`): 26.1 s with 12 threads vs 43.4 s with `RAYON_NUM_THREADS=1` (**1.65×**). PNGs and per-chunk state digests are identical | `~/tmp/sw-scope/out/s125996_{a,t1}.dig` |

### C3. Per stroke vs per canvas

| work | scope | receipt |
|---|---|---|
| `exchange` per bristle per 1.25-px step | per stroke: ∝ path length (px) × bristles × capsule pixels; linear in resolution | `bristle.rs:888-889`, `:1131+` |
| stroke planning (place, trace, footprints) | per pass, serial, small (<1% in samples) | `handling.rs:410-520` |
| `contact_level` (row and column running medians, transposes, box blur) | **whole canvas**, at every `surf()` after any height change: every batch of a pass and every stroke after a bake | `bristle.rs:636-649`, `surface.rs:333` |
| `wait` (`absorb` film thickness, aging, `bake`/`settle_for`, cover fix-up, composite) | the **dirty box**, which only grows until `dry()` (`wet.dirty = None` only at `drying.rs:519`), so late in a painting it is ~the whole canvas. Runs every 15 min of hand time in a pass, every 1 min of single strokes, and at the end of every pass and chunk | `drying.rs:253-307`, `time.rs:22-25`; serial loops at `drying.rs:325-333`, `:500-516`, `:539-560`, `surface.rs:265-272`, `:322-334` |
| live rollback snapshot | whole canvas clone per live chunk (not in replays) | `session.rs:264`, `:192` |
| `--state-digest` | whole checkpoint per chunk; 11.7% of my b08dc3 replay | `main.rs:1100-1106`, `verbs.py s_b08dc3.txt` |

### C4. What is replayed more than once

| replay | when | cost seen |
|---|---|---|
| 1. live painting | during sittings | FRDC1 live `do` sum 12,156 s |
| 2. **live rebuilds** after a failed chunk whose rollback left Lua tables different (`session.rs:300-319`, `main.rs:634-640`) | during sittings | FRDC1 studio: 7 rebuilds, **22,152 s (6.15 h)**. f99282: 6,722 s. 3a958c: 4,635 s. ba1453: 2,531 s (`server.log` `rebuilt` lines). Triggered by quick errors (0.1–13 s `do … err` just before each `rebuilding`) |
| 3. reopen at a new sitting (`resumed chunk` lines) | sittings | FRDC1 `live.txt`: "replayed 503" of 571 |
| 4. `check_painting` step 1: `open` replays the log | after the painting | FRDC1 12,829 s; SONF1 5,855 s |
| 5. `check_painting` step 2: `easel check` replays it **again** and compares with replay 4 | after | SONF1 9,130 s (`p1_check.log`: "times: reopen 5855 s, check 9130 s"); FRDC1's has run from 20:01 to past 23:45 |
| 6. `finish_painting` replays it once more, plus the finishing chunk | after, **concurrently with 4–5** (`notes/round22.1/runner/r21_chains.py:1269-1270`) | FRDC1 12,837 s |

Replays 4 and 6 compute the same thing. Step 5 only tests run-to-run determinism, which `crates/easel/tests/determinism.rs` already tests.

### C5. Could the live state be reused instead of replayed?
**Yes for the finish.** `Canvas::write_state`/`read_state` restore exactly. The doc says "Restoring is exact: a resumed run paints bit-for-bit what an uninterrupted one does" (`checkpoint.rs:13-14`), and `drying.rs:777` tests it (`checkpoint_mid_drying_resumes_exactly`). The finishing chunk needs only the canvas, the studio seed (`Cracks::aged(seed)`, `finish.rs`; varnish noise from `s.seed`) and the clock/hand state (in the checkpoint's tally). It needs no Lua state. Today nothing writes a checkpoint to disk: the only caller is the digest (`main.rs:1105`). The live session writes only `live.png`/`live.txt` at close (`main.rs:839-852`). Confidence: high that the state restores exactly; medium that nothing else in the finishing chunk reads Lua or studio state (I read `finish.rs` and `time.rs` but did not test it).

### C6. Resolution of each step
Everything runs on the full 2400-px canvas: painting, replays (`main.rs:988`), `open` ("live sessions are fixed at 2400px", `main.rs:387`), check (`check.rs:149`) and finishing (`finish_painting`: "Writes <out.png> (2400 px"). Masks are whole-canvas (`canvas.rs:307-311`). Looks and frames are downscaled copies (`look.rs`, `frames.rs` default 1000 px). Cracks are grown on an mm grid (cells of S/6) and rasterized per pixel (`crack.rs:37-41`).

### C7. Speed-ups, in order of expected gain

| # | change | gain (est.) | pixels | confidence |
|---|---|---|---|---|
| 1 | Finish from a canvas checkpoint (written by the live session at close, or by the check replay) instead of replaying | removes a 0.5–3.6 h replay per painting; the finish becomes seconds | pixel-identical | high |
| 2 | Drop `check_painting`'s second replay (`easel check`), or replace it with a bitwise compare of the one replay against the live checkpoint | removes another full replay (SONF1: 2.5 h) | pixel-identical (no pixels made) | high |
| 3 | Make live rollback exact (or rebuild from a per-chunk canvas checkpoint) so a failed chunk doesn't replay the log | up to 6 h of painter wall time (FRDC1) | pixel-identical | medium (did not check why tables mismatch) |
| 4 | Exact incremental `contact_level`: running medians are bit-local over a window (`bristle.rs:2193-2205`); recompute the box blur's full rows and columns through the changed band (`bristle.rs:2207-2215` says crops are not exact) | 5–25% of late chunks | pixel-identical if done as the test says | medium |
| 5 | Parallelize the serial gather loops in `absorb`/`film_thickness`/`bake`/`settle_for`/`raise` without changing rects or blur order | up to ~10% of late chunks (main thread 15% in `wait`) | pixel-identical (per-pixel independent copies) | medium-high |
| 6 | Cache `canvas{}` + grounds by arguments (checkpoint after chunk 1) | 44–58 s per replay with a brushed ground; large for a test gate | pixel-identical | high |
| 7 | `exchange` micro-optimizations that keep the float operation order (pass 1 weights are per-pixel independent; pass 2 has neighbor writes, so it is ordered) | 1.2–2× on the kernel (guess) | pixel-identical only with care | low |
| 8 | Smaller or independent tiles, more concurrent brushes | 2–4× on `work` (12 cores, 1–3 tiles today) | **changes pixels**: each tile's brush is seeded by tile index (`handling.rs:781`), so the tile grid is part of the picture | medium |
| 9 | Blur only local rects (dirty box per film, crop windows) instead of the ever-growing dirty box / whole canvas | large in late chunks | **changes pixels** (running sums restart, `bristle.rs:2207-2239`) | medium |
| 10 | Coarser physics grids (drying, leveling, contact in mm cells), fewer bristles, larger steps | large | **changes pixels** | medium |

---

## D. The replay test

### D1. What makes a replay identical today
- **Randomness**: own RNG and hashes (`rng.rs`). Lua `math.random`/`rand` reseeded per chunk from seed and chunk number (`api.rs:65-71`, `mixseed` `:84-89`). Brush seeds from `auto_seed` (`:73-76`). Pass seeds via `seed_of` (`:1011`).
- **Ordering**: deterministic `pairs`/`next` and hidden addresses (`prelude.lua:18-60`; guide: "`pairs` walks a table in the same order in every session and replay").
- **Threads**: `run_ordered` gives the serial result ("bit-for-bit that of running them one by one in order", `sched.rs:7-9`). Rayon loops are per-pixel independent, and the one reduction is `f32::max` (`drying.rs:419`). Tests: `drying_is_deterministic_across_thread_counts` (`drying.rs:847`), `fixture_is_deterministic_across_thread_counts` (`tests.rs:139`), `determinism.rs` (processes, thread counts). My 1-thread vs 12-thread run: identical PNG and digests.
- **Floating point**: release `codegen-units = 1`, `incremental = false`, because "several codegen units can partition and optimize the same code differently … moving a few pixels by 1/255" (`Cargo.toml:8-13`). Test and release profiles give different floats (commit `41c30dc`: "PNG hashes per build profile (engine floats differ)"). f32 running sums in `box_blur` make results depend on the rectangle (`bristle.rs:2207-2239`).
- **Version marks**: `--@ engine N` (absent = 1) selects behavior (`lib.rs:64-73`, `ENGINE = 2`; branches listed in B2). `--@ box <name>` selects tubes (`session.rs:23`, `:350`).
- **Comparisons**: `easel check` compares `seen()` bits and `surface_um` bits (`check.rs:166-170`). `check_painting` compares PNG bytes with the live `live.png` (`cmp`, `scripts/check_painting`). `easel run --state-digest` logs FNV-1a of the full checkpoint, brushes and studio state per chunk (`main.rs:1072-1115`). Its format is fixed: "a field added to `Held`, `Hand` or the checkpoint changes the digests" (`:1096-1098`).

### D2. Existing tests
| test | what | receipt |
|---|---|---|
| `paint` golden fixture | fingerprint of a multi-pass fixture, test profile | `tests.rs:147-158`, `crates/paint/tests/golden_scene.txt` |
| r19 log | `r19_default_box.lua` (21 lines) replays to a recorded PNG+surface FNV, release-built easel | `crates/easel/tests/boxes.rs:139-155` |
| engine-1 logs | studios 6399ad (7 chunks) and db6324 (18) at **320 px**, hashed | `session.rs:1440-1458` |
| determinism | object-keyed tables across processes; hand time across thread counts | `crates/easel/tests/determinism.rs` |
| scripts | `scripts/tests/check_live.sh` (check verdicts), `replay_env.sh` (box from log), box/export checks | `scripts/tests/*.sh` headers |
| duration | "the whole suite runs in about a minute on the M3 Pro … plus about 50 s of build" | `notes/workflow.md:455-461`. That note's release-tier `hand_time` test no longer exists (`ls crates/easel/tests`) |

### D3. Proposed gate (record goldens with the pre-refactor commit, release profile)
- **Tier 1, per-verb goldens (~minutes).** One small Lua program per verb at a reduced width (as `session.rs:1447` does at 320 px), each with `--state-digest` per chunk plus PNG and surface FNV. Use a knife-only ground to skip the 45-s brushed ground, with one brushed-ground case. Cover:
  - `work` for each hand: broad, body, detail, hatch, glaze, scumble, blend;
  - `work` options: `fill`, `clip`, `edge=`/fence, `cut_in`, `order`;
  - `stipple`, `lose`;
  - `b:stroke` (blunt and pointed, tacky substrate), `b:touch`, `b:wipe`/clean lift;
  - `wait` across open→setting→tacky→dry, `drying`;
  - pencil/chalk `sketch`/`line`/`rule`/`hatch`, `erase`, `fix`, `outline:paint`;
  - `varnish`/`cracks`/`relief`;
  - engine 1 and 2; each box; a legacy `canvas{style=}` case.
- **Tier 2, short real logs at 2400 px (~10–15 min with 3–4 at once).** Candidates (chunks; recorded time):

| log | chunks | engine/box | time |
|---|---|---|---|
| `paint-studio-125996` | 5 | 2 | 26 s (my replay) |
| `paint-studio-6f0308` | 4 | 2 | 7 s live |
| `paint-studio-482665` | 2 | 2 | 54 s live |
| `paint-studio-6399ad` | 7 | 1 | 121 s live |
| `paint-studio-b5323f` | 10 | 1 | 157 s live |
| `paint-studio-db6324` | 18 | 1 | 92 s live |
| `paint-studio-106bf6` | 17 | 2 | 223 s live |
| `paint-studio-3cab71` | 17 | 1, hopper | 199 s live |
| `paint-studio-b08dc3` | 35 | 2 | 257 s (my replay) |
| `notes/time/edge_instant.lua` | 3 | legacy | 115 s (my replay) |
| `notes/lab/rock_A.lua` | 7 | legacy | 87 s (my replay) |

  Compare per-chunk digests (drop `secs=`), so a failure names its chunk. Live times are from each studio's `server.log` `do … ok` lines.
- **Tier 3, everything (overnight).** Unique logs: **49 studio logs** (6,514 chunks) and **47 legacy logs** under `notes/` (597 chunks). The `run/` and `~/tmp/gallery-fcf9c110` `.lua` files are copies plus a finishing chunk, and `archive/sources/` holds 21 Rust programs, not logs (`find archive/sources -type f` → 21 `.rs`). Estimated single-replay time: studios ~14.2 h for the 5,115 chunks with recorded times plus ~2.7 h for 1,399 untimed chunks at the 7 s/chunk median ≈ **17 h** (`fullcost.py`). Legacy logs: ~12–38 s/chunk in my two runs → ~2–6 h. At 3–4 concurrent replays (each uses 2–4 cores) that is ~6–8 h of wall time (estimate). Add the 4 finished PNGs on disk (`notes/round*/runner/run/*_finished.png`) as finishing goldens.

---

## E. Risks and unknowns

**Hardest to move**
1. `bristle.rs::exchange` + `Surf`. Unsafe raw pointers into 10 buffers (`:542-572`). The plough writes to neighbor pixels (`bristle.rs:1363-1420`), so results depend on visit order. Known "traps" for exact speedups are pinned by tests (`:2122-2239`, commit `64f6f4b`).
2. `drying.rs` `absorb`/`bake`. They depend on stroke ids, the engine version, the ever-growing dirty box and blurs whose rectangle is part of the result.
3. `session.rs` rollback with `heap.lua`. Exact restore of Lua state, with full rebuilds when it fails.

**Constraints that contradict a naive plan**
4. **The rectangles are numerics.** `box_blur` running sums restart at a rect's edge, so blurring a different rect changes bits (`bristle.rs:2207-2215`). A refactor that changes the dirty box, crop margins or tile footprints changes pixels.
5. **The tile partition is part of the picture.** There is one brush per tile, seeded by tile index (`handling.rs:781`), with tiles sized from stroke reach (`:724-726`). The large in-replay parallel gains need a new engine version (C7 #8–10). The pixel-identical gains of size are in the pipeline (#1–3).
6. **Codegen can move pixels.** The repo says codegen partitioning moved pixels by 1/255 (`Cargo.toml:8-11`) and that the profiles differ (`41c30dc`). Moving code between modules or crates, or changing inlining, may change floats without any semantic change. The gate will show it, and the owner must decide whether such drift fails the refactor. I did not find the mechanism.
7. **Dry paint is baked into RGB.** A blade or abrasive on dry paint, scraping tacky paint and watercolor lifting or rewetting all need per-pixel layer history (at least the top dry film's pigment and thickness, as graphite's `Cell` keeps). Recording it costs memory on top of ~108 B/px (~864 MB for FRDC1's canvas). It is pixel-neutral only if no existing path reads it, and it should be gated to a new engine version or to the new verbs.
8. **No pigment identity, no oil/solvent amounts.** Granulation, staining, turpentine and lean/fat all need new per-pixel (and per-bristle) state. That changes the checkpoint and the `--state-digest` format (`main.rs:1096-1098`), so the gate should hash the old fields or PNG+surface, not the raw digest.
9. **Watercolor's clock.** Hand time ages paint in 15-min slices (`time.rs:22`). Watercolor dries in minutes, so a per-medium slice is needed. Changing the global one changes old logs.
10. **Linen is mandatory.** It is required in `Style` and `canvas{}` (`api.rs:1205-1220`), and cracks assume weave pitch and stretcher corners (`crack.rs:1351`). Panel and paper need a support type without disturbing the linen path.

**Not determined**
- Why live rollbacks report mismatched tables (did not check `heap.lua` restore details).
- The breakdown of the 4.5 GB footprint (did not check).
- Per-verb CPU over a whole long painting: I sampled 10-s windows of late chunks plus one whole 35-chunk run, not a full 572-chunk replay.
- Unloaded timings: every number here was taken at load ~35.
- Why `easel check`'s replay ran 56% slower than the reopen for SONF1 (9,130 s vs 5,855 s). Possibly contention, or its Lua hook every 1M instructions (`check.rs:39`); did not check.
- Whether `exchange` pass 1 can be vectorized with bit-identical results (did not try).
- Internals of `form.rs`, `scene.rs`, `outline.rs` and `stipple.rs` beyond their entry points (did not check).
