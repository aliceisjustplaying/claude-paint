# Watercolor 3: native-resolution experimental studio

This crate lives outside the main workspace. The oil executable, solver, boxes and checkpoint formats do not import it. It builds a separate `watercolor-studio` binary that requires both `--box watercolor` and a studio whose `box` file says `watercolor`. This is an empirical Curtis-inspired pilot, not a production watercolor edition.

```sh
export CARGO_TARGET_DIR="$HOME/.capy/work/wc3-target"
cargo test --locked --manifest-path experiments/watercolor/Cargo.toml
cargo build --release --locked --manifest-path experiments/watercolor/Cargo.toml
"$CARGO_TARGET_DIR/release/watercolor-studio" --box watercolor init "$HOME/.capy/work/watercolor-demo"
"$CARGO_TARGET_DIR/release/watercolor-studio" --box watercolor run "$HOME/.capy/work/watercolor-demo" experiments/watercolor/demo/smoke-start.lua
"$CARGO_TARGET_DIR/release/watercolor-studio" --box watercolor run "$HOME/.capy/work/watercolor-demo" experiments/watercolor/demo/smoke-finish.lua
```

The two smoke chunks demonstrate saved continuation through the public Lua API. Their scene is an engine demo for people reviewing the implementation. Painter notes are a separate draft in the Captain's drive and require approval before use.

## Developer API inventory

- `paper{kind="cold-press"|"hot-press", size=WIDTH_MM, aspect=WIDTH_OVER_HEIGHT, cell=CELL_MM, seed=N}` initializes once. The prototype accepts widths of 2–60 mm, aspects of 0.5–3, cell sizes of 0.025–0.1 mm and at most 1,500,000 simulation cells. The default cell is 0.05 mm. A 60 mm sheet with aspect 1.5 has 1200×800 real simulation cells. A finer grid must fit the cell-count limit.
- `brush{name=NAME, width=MM}` creates a named brush; the same name retrieves its saved width and load. Changing its width requires a new name. `b:status()` reports water, pigment and width.
- `b:load{water=MM3, pigments={ultramarine=MASS, phthalo=MASS, earth=MASS}}` adds to the brush. There are three real pigment identities; no aliases pretend to be additional pigments. Water uses mm³ (µL); pigment uses arbitrary integrated areal mass units.
- `b:stroke({{X_MM,Y_MM},...},{dry=BOOL,pressure=0_TO_1,grade=0_TO_1})` uses rounded segments, tooth-caught outlines and subcell area coverage. `b:wash{x=MM,y=MM,width=MM,height=MM,grade=0_TO_1}` fills a rectangle; grade is the bottom/top pigment ratio. Wet deposition draws from the finite brush inventory. Dry contact combines tooth skips with coherent 0.14 mm filament tracks and a tangential footprint.
- `drop{x=MM,y=MM,water=MM3,radius=MM}` adds a peaked Gaussian clean-water deposit. `b:lift(PATH)` supplies damp-brush water, lifts loose pigment and retains the lifted water/pigment in that brush. `blot{RECTANGLE_FIELDS}` transfers removed material to a separately counted blot sink.
- `wait(SECONDS)` advances 0–600 seconds per call. Stroke/wash actions themselves take no hand time. `clock()` includes the pending nanosecond residual. `balance()` checks the whole studio inventory and reports surface/pore water and the dry guard. `glaze()` archives a dry optical stratum, refuses wet state and limits a studio to 12 archived glazes.
- `look{mode="color"|"gallery"|"relief",path=NAME_PNG,width=PIXELS,crop={X,Y,W,H}}` writes a PNG. Painter-facing `look` defaults to Color. Gallery uses 115°/55°/1 and Relief uses 135°/25°/1 in the watercolor paper-light projector. A crop is in output pixels and is saved without further resizing.
- `frames{every=SECONDS,width=PIXELS,prefix=NAME}` emits Gallery frames while waiting. Intervals must be multiples of 0.0625 s, with at most 1,024 frames per chunk. `frames(false)` disables it. Filenames contain absolute physics tick numbers; presentation settings are re-enabled explicitly in a new chunk.
- `save("NAME.json")` stages a named material/tool snapshot at that point. Successful chunks automatically publish `checkpoint.json`. `watercolor-studio --box watercolor restore DIR NAME.json` restores a named snapshot; `render DIR OUTPUT.png` renders the current checkpoint in Gallery at 1200 px.

Every run starts a fresh Lua VM. It restores paper, all material reservoirs/glazes, the clock/residual, the ledger and named brush inventories. It does **not** restore arbitrary Lua locals, closures, coroutines or `math.random` continuation. A continuation chunk redeclares its Lua handles by name and executes new work. Exact repeated source chunks are refused. Failed Lua chunks leave the committed checkpoint, named saves and output images unchanged; staged output is removed. There is one writer lock per studio. After a forcibly killed process, inspect any remaining lock/pending files before removing them.

The VM exposes math, table, string and UTF-8 libraries with a 128 MB Lua allocation limit and about two million interpreted instructions. File loading, OS, package and debug libraries are absent. Native simulation buffers have the separate paper-cell limit. Native waits can be CPU-heavy; use a bounded background job for longer runs. The runner is intended for trusted private studios, not hostile checkpoint uploads or a public service.

## Medium and optics

Schema 3 persists cell size and refuses earlier snapshots. Continue older studios using their pinned earlier runner. The standard probe harness simulates 720×448 cells at 0.05 mm over the same physical sheet as rounds 1 and 2. Water depths are mm; multiply summed pixel-mm by the saved cell size squared for mm³. Pigment channels store areal mass density. The studio accounts for canvas inventory, unused/dirty brushes, evaporation and the blot sink in physical area-integrated units.

Paper retains pr13's fiber/floc/felt geometry at commit `454446b8`. Two physical-scale correlations form the tooth, correlated grammage supplies pore capacity and smoothly varying sizing supplies permeability. The old independent Gamma pore scatter is not used as visible pigment clumping. Clump affinity varies coherently over 0.3 mm and has a smaller influence than paper valleys. Surface head and optical normals share a 25 µm projection of normalized tooth.

One public tick is exactly 62,500,000 ns. A wet tick has four deterministic 1/64-second transport/reaction substeps; pore-only evolution uses one bounded tick. Fully dry unobserved waits fast-forward the same clock, including drying reached midway through a wait. Integer residuals are persisted. Wait partitions with equal rounded ns totals, no intervention and the same build/target produce bit-identical material state and projections. Absolute-tick frame sampling cannot choose solver steps. Lua source history may differ between equivalent chunk partitions; it is bookkeeping, not material state. Transient solver buffers are reused within a wait, do not enter snapshots and do not affect arithmetic order.

Face flow includes thin-film mobility, viscous damping, pigment-dependent viscosity and sizing-dependent permeability. Transport coefficients scale with cell size; water transfer is capped at 35% of donor water per substep and pigment transfers retain donor limits. Pore transfers also retain capacity limits. Edge-enhanced evaporation samples a 0.2 mm neighborhood. Fine-film suspension is deposited before numerically negligible water becomes a point carrier. Staining affinity adsorbs smooth pigments early; granulating pigments drift down tooth gradients and settle preferentially into correlated valleys. A finite binding fraction preserves lifting differences. Speed-dependent rewetting releases loose pigment under a clean-water backrun. Evaporation/lifting remain explicit sinks or typed transfers to tools.

Each glaze keeps its coverage and per-pigment deposited/bound amounts. Coverage reconstructs subcell painted area optically with finite-thickness Kubelka–Munk composition over the visible paper. It does not erase pigment inventory. Rendering uses the same saved final state for Color, Gallery, Relief and crops. When export width equals the simulation width, output bytes come directly from the simulation projection with no resizing. Other widths use Lanczos reconstruction. Wet gloss, calibrated spectra and cross-grid/time convergence remain unverified.

Pin the runner commit/build used for a painting when resuming it. These JSON snapshots validate channel shapes, finiteness and balances but have no production integrity/tile-store/runtime-registry guarantees. The oil CPSTATE1 path stays independent.

## Native-resolution demo

Run `demo/native-start.lua` then `demo/native-finish.lua` in a fresh explicit watercolor studio. These are the same logical engine study scaled to a real 60×40 mm, 1200×800-cell sheet. Their 1200×800 Gallery/Color finals and detail crops are 1:1 simulation pixels. Intermediate frame-hook exports are 480×320 diagnostic views. Assemble the bloom strip/slow loop with `python3 experiments/watercolor/demo/morning_sheet.py DIR native-`.

State writes stream through a buffered writer and continuation validates the decoded state without making a second JSON/state copy. Snapshots remain dense JSON and can exceed 200 MB at native resolution; keep them in private scratch, never in Capy Drive. Each successful chunk writes one automatic checkpoint. The native demo does not create duplicate named checkpoints.

## Probe harness

```sh
"$CARGO_TARGET_DIR/release/watercolor-probes" "$HOME/.capy/work/watercolor-3/probes"
python3 experiments/watercolor/sheet.py "$HOME/.capy/work/watercolor-3/probes" "$HOME/.capy/drive/captain-stillwet/renders/watercolor-2"
```

An optional final argument selects `flat`, `bloom`, `graded`, `granulation`, `dry-brush`, `glaze` or `lifting` for targeted iteration. The full assembler requires Pillow and DejaVu Sans. It produces the seven-probe pair sheet, a round comparison, conservation tables and the bloom strip/GIF. The GIF holds each 1/16 physical second for 120 ms (1.92× slower, respecting GIF's 10 ms time quantum). It is a diagnostic loop, not a website clip.

Reference: Curtis et al., *Computer-Generated Watercolor*, SIGGRAPH 1997, §§3–5, DOI 10.1145/258734.258896.
