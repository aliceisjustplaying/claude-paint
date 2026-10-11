# Watercolor 5: matched wash-cleanup swatches

## Watercolor Lab port

The accepted solver is still the default. Select the pinned Watercolor Lab solver when creating a new sheet:

```lua
paper{solver="lab",kind="cold-press",size=52.8,aspect=1.5,cell=0.05,seed=71}
local b=brush{name="round",width=2}
b:load{water=2,pigments={ultramarine=0.1,earth=0.02}}
b:stroke({{8,12,0.2,0},{20,13,0.8,0.3},{40,12,0.4,0.8}},{duration=0.8})
```

The added point fields are pressure and elapsed stroke time in seconds. They are optional; pressure defaults to the stroke's `pressure` option and timing defaults to 40 mm/s. A moving Lab stroke requires positive time. Width follows pressure, stamps follow Watercolor Lab's distance-carry resampler and water/pigment release stops when the named brush's finite inventory runs out. The reference brush's 0.15 reservoir floor remains inside the port; the finite inventory cap is outside it. Lab washes use moving rows. Existing strokes with the default solver retain their accepted behavior.

`lab-core/` is a mechanical Rust port of Watercolor Lab at `a19bfa2a026856a98ceb400a8eec41f091f04a11`. Its passes use reference cell units, f32 shader arithmetic, LINEAR sampling, CLAMP_TO_EDGE boundaries and RGBA16F storage/blend quantization. Raster vertex precision is an adapter input: native software reference runs use four subpixel bits; the browser reads its GL backend's value. Paper and brush generation preserve the original JavaScript double precision before texture/attribute uploads. All 36 pigments and the MIT notice are included.

The native map is 0.05 mm/cell, so the reference 1056×704 grid represents 52.8×35.2 mm. One unit of standing-water depth maps to 1 mm only at the adapter boundary. Lua time maps to 180 reference substeps per second (three steps per 60 Hz frame). This is an explicit playback map, not a new physical time constant inside the core. The existing 0.0625-second clock, pending residual, saved state, named brushes, checkpoint restore, frame hook and Color/Gallery/Relief exports remain. Legacy sheets remain schema 3; Lab sheets are schema 4 so older runners refuse them instead of silently losing the port's texture state. Native views use the existing studio light presets over the port's KM color; comparisons and the app use the unchanged reference display.

Lab's `ultramarine`, `phthalo` and `earth` API inventories use the library's French ultramarine, phthalo blue and burnt sienna. The browser exposes all library pans. Lab glazes fold into the reference's single dried-reflectance layer automatically; `glaze()` checks the reference dry condition. Lab lifting uses the reference scrub field. `blot{}` has no reference counterpart and returns an explicit unsupported-operation error for Lab sheets. The current solver's blot and glaze stack are unchanged.

The reference does not conserve pigment during semi-Lagrangian sampling. Its dried reflectance also cannot recover per-pigment mass. `balance()` for Lab sheets therefore reports `conserved=false`, current water/live pigment and finite brush-release inventory; it does not pretend the reference satisfies the current solver's strict conservation invariant. Checkpoint validation still rejects malformed textures, parameters and brush-release ledgers. The existing solver keeps its strict conservation checks.

The browser fork and local build instructions are in `lab-app/README.md`. It keeps the pinned UI, with a blind Engine A/B switch that clears paint while retaining sheet settings. Salt is hidden in both options: its amplified sine hash is backend-sensitive and fails the reference numeric tolerance even though the formula is unchanged. No salt workaround is applied to the core.

The per-pass readPixels comparison and matched capture commands are in `lab-validation/`. The only new numeric verification is that required parity/conservation diagnostic. No new unit tests are added. The original full-resolution capture exceeded the five-minute software-GPU limit; use hardware Chrome for the complete comparison. See the task report for numeric differences, timing, the app's lower-resolution setting and visual limitations.

This crate lives outside the main workspace. The oil executable, solver, boxes and checkpoint formats do not import it. It builds a separate `watercolor-studio` binary that requires both `--box watercolor` and a studio whose `box` file says `watercolor`. This is an empirical Curtis-inspired pilot, not a production watercolor edition.

```sh
export CARGO_TARGET_DIR="$HOME/.capy/work/wc4-target"
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
- `b:stroke({{X_MM,Y_MM},...},{dry=BOOL,pressure=0_TO_1,grade=0_TO_1})` uses rounded segments, tooth-caught outlines and subcell area coverage. `b:wash{x=MM,y=MM,width=MM,height=MM,grade=0_TO_1}` fills a rectangle; grade is the bottom/top pigment ratio. Wet deposition draws from the finite brush inventory. Dry contact combines tooth skips with irregular filaments around a mean 0.14 mm spacing, variable strand width/load and subcell-integrated contact.
- `drop{x=MM,y=MM,water=MM3,radius=MM}` adds a peaked Gaussian clean-water deposit. `b:lift(PATH)` supplies damp-brush water, lifts loose pigment and retains the lifted water/pigment in that brush. `blot{RECTANGLE_FIELDS}` transfers removed material to a separately counted blot sink.
- `wait(SECONDS)` advances 0–600 seconds per call. Stroke/wash actions themselves take no hand time. `clock()` includes the pending nanosecond residual. `balance()` checks the whole studio inventory and reports surface/pore water, `max_surface_depth_mm` (peak free-water depth in mm) and the dry guard. `glaze()` archives a dry optical stratum, refuses wet state and limits a studio to 12 archived glazes.
- `look{mode="color"|"gallery"|"relief",path=NAME_PNG,width=PIXELS,crop={X,Y,W,H}}` writes a PNG. Painter-facing `look` defaults to Color. Gallery uses 115°/55°/1 and Relief uses 135°/25°/1 in the watercolor paper-light projector. A crop is in output pixels and is saved without further resizing.
- `frames{every=SECONDS,width=PIXELS,prefix=NAME}` emits Gallery frames while waiting. Intervals must be multiples of 0.0625 s, with at most 1,024 frames per chunk. `frames(false)` disables it. Filenames contain absolute physics tick numbers; presentation settings are re-enabled explicitly in a new chunk.
- `save("NAME.json")` stages a named material/tool snapshot at that point. Successful chunks automatically publish `checkpoint.json`. `watercolor-studio --box watercolor restore DIR NAME.json` restores a named snapshot; `render DIR OUTPUT.png` renders the current checkpoint in Gallery at 1200 px.

Every run starts a fresh Lua VM. It restores paper, all material reservoirs/glazes, the clock/residual, the ledger and named brush inventories. It does **not** restore arbitrary Lua locals, closures, coroutines or `math.random` continuation. A continuation chunk redeclares its Lua handles by name and executes new work. Exact repeated source chunks are refused. Failed Lua chunks leave the committed checkpoint, named saves and output images unchanged; staged output is removed. There is one writer lock per studio. After a forcibly killed process, inspect any remaining lock/pending files before removing them.

The VM exposes math, table, string and UTF-8 libraries with a 128 MB Lua allocation limit and about two million interpreted instructions. File loading, OS, package and debug libraries are absent. Native simulation buffers have the separate paper-cell limit. Native waits can be CPU-heavy; use a bounded background job for longer runs. The runner is intended for trusted private studios, not hostile checkpoint uploads or a public service.

## Medium and optics

Schema 3 persists cell size and refuses earlier snapshots. Continue older studios using their pinned earlier runner. The standard probe harness simulates 720×448 cells at 0.05 mm over the same physical sheet as rounds 1 and 2. Water depths are mm; multiply summed pixel-mm by the saved cell size squared for mm³. Pigment channels store areal mass density. The studio accounts for canvas inventory, unused/dirty brushes, evaporation and the blot sink in physical area-integrated units.

Paper retains pr13's fiber/floc/felt geometry at commit `454446b8`. Fine tooth is the relative height between a 0.08 mm correlation and a 0.5 mm local sheet datum. Dry contact and optical normals retain this tooth unchanged. Wet head, permeability and settling use an additional triangular correlation with a 0.15 mm radius, computed once per wait from the saved paper height. Correlated grammage still supplies pore capacity and smoothly varying sizing supplies permeability. The serialized clump field remains for schema compatibility but no longer modulates wet settling. Wet surface head retains its 25 µm projection.

One public tick is exactly 62,500,000 ns. At its canonical boundary a wet tick chooses a power-of-two substep count from cell size and the maximum water head, with at least four substeps. Deep drops and finer grids get shorter steps to damp grid waves. Caller wait partitions and frame cadence cannot choose those steps; pore-only evolution uses one bounded tick. Fully dry unobserved waits fast-forward the same clock, including drying reached midway through a wait. Integer residuals are persisted. Wait partitions with equal rounded ns totals, no intervention and the same build/target produce bit-identical material state and projections. Absolute-tick frame sampling cannot choose solver steps. Lua source history may differ between equivalent chunk partitions; it is bookkeeping, not material state. Transient solver buffers are reused within a wait, do not enter snapshots and do not affect arithmetic order.

Face flow includes thin-film mobility, viscous damping, pigment-dependent viscosity and sizing-dependent permeability. Thin films retain a weaker valley permeability bias on the wet-scale field. Transport coefficients scale with cell size; water transfer is capped at 35% of donor water per substep and pigment transfers retain donor limits. Pore transfers also retain capacity limits. An empirical contact-line head pins to the applied footprint; interior pressure does not feed back from neighboring liquid depth. The existing wet-front mask also retains capillary moisture. Edge-enhanced evaporation samples a 0.2 mm neighborhood, changes continuously with its water-depth contrast and fades below 4 µm. Fine-film suspension is captured continuously as free water is taken up/evaporated; the remaining pigment deposits when that film vanishes. This avoids whole-cell capture and evaporation jumps that left faint interior seams. Staining affinity adsorbs smooth pigments early. Granulating pigments settle preferentially into wet-scale valleys, with bounded relief that fades with free-water depth and the total deposited/bound coating across glazes. The independent tooth-gradient pigment drift is removed; concentration diffusion is retained. No flocculation field is introduced. A finite binding fraction preserves lifting differences. Speed-dependent rewetting releases loose pigment under a clean-water backrun. Evaporation/lifting remain explicit sinks or typed transfers to tools.

Each glaze keeps its coverage and per-pigment deposited/bound amounts. Coverage reconstructs subcell painted area optically with finite-thickness Kubelka–Munk composition over the visible paper. It does not erase pigment inventory. Rendering uses the same saved final state for Color, Gallery, Relief and crops. When export width equals the simulation width, output bytes come directly from the simulation projection with no resizing. Other widths use Lanczos reconstruction. Wet gloss, calibrated spectra and cross-grid/time convergence remain unverified.

Pin the runner commit/build used for a painting when resuming it. These JSON snapshots validate channel shapes, finiteness and balances but have no production integrity/tile-store/runtime-registry guarantees. The oil CPSTATE1 path stays independent.

## Native-resolution demo

Run `demo/native-start.lua` then `demo/native-finish.lua` in a fresh explicit watercolor studio. These scale the engine study to a real 60×40 mm, 1200×800-cell sheet, with the clean-water drop moved farther inside the wash to expose its whole rim. Their 1200×800 Gallery/Color finals and detail crops are 1:1 simulation pixels. Intermediate frame-hook exports are 480×320 diagnostic views. Assemble the bloom strip/slow loop with `python3 experiments/watercolor/demo/morning_sheet.py DIR native-`.

State writes stream through a buffered writer and continuation validates the decoded state without making a second JSON/state copy. Snapshots remain dense JSON and can exceed 200 MB at native resolution; keep them in private scratch, never in Capy Drive. Each successful chunk writes one automatic checkpoint. The native demo does not create duplicate named checkpoints.

`demo/round4-start.lua` and `demo/round4-finish.lua` use that same native sheet, with 28 seconds of drying before the clean-water drop. The finishing chunk restores named tools in a fresh VM, completes the glaze/dry contact/lift and writes `watercolor-4-gallery.png`, Color and three 1:1 output crops for the dried edge, bloom and dry brush. Assemble its frame-hook strip/loop with `python3 experiments/watercolor/demo/morning_sheet.py DIR round4-`. The assembler selects quarter-second frames without assuming an absolute starting tick.

Round 4 adds only `balance().max_surface_depth_mm` to the Lua result table; existing signatures and limits are unchanged from round 3. Reviewed notes written against round 2 also need the prior round-3 paper delta: optional `cell`, default 0.05 mm, accepted 0.025–0.1 mm and the 1,500,000-cell limit. Exports at native width bypass resizing. Keep those notes separately reviewed before distribution.

## Probe harness

### Round-5 matched swatches

`demo/round5-swatches.lua` makes one 16×24 mm sheet at 0.05 mm per cell (320×480), seed 71. Its six rows are staining phthalo, pale ultramarine, dense ultramarine, earth, a wet color join and unchanged dry contact. Run this exact source once with a runner built from `d5c6a41` and once with the round-5 runner in separate fresh studios. Both Color and Gallery exports come from each run's final state. No simulation is rerun for a view or crop.

```sh
python3 experiments/watercolor/demo/round5_sheet.py CURRENT_STUDIO FIXED_STUDIO OUTPUT_DIR
```

The assembler produces a labeled two-column Gallery sheet, its Color version and six pixel-exact 1:1 Gallery row crops. It requires Pillow and DejaVu Sans. There are no Lua API or schema changes in round 5. Pin the runner when continuing a studio: old snapshots remain readable, but a new physics build changes their future trajectory.

```sh
"$CARGO_TARGET_DIR/release/watercolor-probes" "$HOME/.capy/work/watercolor-3/probes"
python3 experiments/watercolor/sheet.py "$HOME/.capy/work/watercolor-3/probes" "$HOME/.capy/drive/captain-stillwet/renders/watercolor-2"
```

An optional final argument selects `flat`, `bloom`, `graded`, `granulation`, `dry-brush`, `glaze` or `lifting` for targeted iteration. The full assembler requires Pillow and DejaVu Sans. It produces the seven-probe pair sheet, a round comparison, conservation tables and the bloom strip/GIF. The GIF holds each 1/16 physical second for 120 ms (1.92× slower, respecting GIF's 10 ms time quantum). It is a diagnostic loop, not a website clip.

Reference: Curtis et al., *Computer-Generated Watercolor*, SIGGRAPH 1997, §§3–5, DOI 10.1145/258734.258896.
