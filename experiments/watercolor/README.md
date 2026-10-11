# Watercolor 1: offline material pilot

This standalone crate is outside the main Cargo workspace. No feature, painter API, oil solver or saved-painting format imports it. Run it explicitly:

```sh
CARGO_TARGET_DIR="$HOME/.capy/work/wc-pilot-target" cargo test --manifest-path experiments/watercolor/Cargo.toml
CARGO_TARGET_DIR="$HOME/.capy/work/wc-pilot-target" cargo run --release --manifest-path experiments/watercolor/Cargo.toml --bin watercolor-probes -- "$HOME/.capy/work/watercolor-1/renders"
python3 experiments/watercolor/sheet.py "$HOME/.capy/work/watercolor-1/renders"
```

The sheet assembler requires Pillow and DejaVu Sans. The Rust harness writes PPM, conservation CSV and 129 intermediate bloom frames. The Python assembler emits PNG sheets, a Markdown conservation table and a slow GIF. The GIF maps each 1/16 physical second to 120 ms of presentation time (1.92 times slower, using GIF's 10 ms time quantum). Changing that duration changes no material state. This is an empirical Curtis et al. 1997-inspired model, not a replication of their full shallow-water solver or a calibrated pigment catalog.

## State and units

At the fixed prototype grid spacing of 0.2 mm, surface water and capillary occupancy are depths in mm. Totals in the CSV are pixel-mm; multiply by 0.04 mm² to get mm³. Pigment amounts are arbitrary areal mass units, with independent identity for an ultramarine-like, phthalo-like and earth pigment. They are not calibrated grams. The three coupled reservoirs are flowing water with suspended pigment, deposited/bound pigment and pore water. Capacity, sizing and height are static support channels. Pore saturation is occupancy divided by capacity, never oil uptake.

`fiber_paper.rs` and its small RNG are copied from `capy/pr13-paper-ref`, commit `454446b8`, with unused oil uptake and compression fields removed. Fiber laying, flocculation, felt imprint, grammage and micro-roughness are preserved. The pilot chooses 300 g/m² cotton-like cold-press paper, a 0.85 mm felt cell and 32 µm felt depth. Its normalized tooth is a nonlinear projection of the source height; the pressure head uses 12 µm of that projected relief. Sizing varies coherently from 0.3 to 0.88. Clump affinity varies spatially and multiplies low-tooth settling; there is no explicit particle aggregation process yet.

## Deterministic clock

One physics tick is exactly 62,500,000 ns (1/16 s). `wait_ns` accumulates a persisted integer residual and runs only complete ticks. `wait_seconds` rounds its argument once to the nearest ns. Split waits equal one long wait bit-for-bit in the same build/target when their rounded ns totals agree and no intervention occurs between them. This includes full serialized state, both image projections and save/resume across a fractional tick. There is no claim of bit equality across compilers or CPU targets.

`wait_ns_observed` emits immutable states on an absolute tick cadence. The tick number supplies the frame timestamp; the residual at each observed tick is zero. An observer cannot choose substeps or mutate the solver. Splitting an observed wait, including save/resume, produces the same frame states and final state. Requested presentation intervals below 1/16 s must repeat or interpolate images; they cannot manufacture finer physics. Observer callbacks are synchronous and should not panic.

## Canonical phases

1. Snapshot the wet front. Update damped oriented face flux from depth/height differences. Surface pinning releases only above 0.22 mm with a neighboring pore saturation above 0.001. This is an empirical contact-line rule.
2. Budget each donor's combined water outflow. Transfer water and donor-concentration suspended pigments simultaneously. Diffuse each pigment using a separate donor budget.
3. Diffuse pore water with donor and receiver capacity limits. Pore transport does not move pigment in this prototype.
4. Take up surface water into remaining pore capacity. Evaporate surface water with an enhanced exposed-edge sink and capillary water with a saturation-dependent sink. Every lost amount enters the ledger.
5. Adsorb pigment with low-tooth/granulation/clump affinity. Bind only up to the pigment's staining fraction of local settled mass. Desorb loose active pigment under wet water. Rewet archived loose pigment through a separate explicit release law. Bound pigment stays in its stratum unless lifted.

There is one active wet glaze and an ordered set of dry glazes. `finish_glaze` refuses wet state. Damp lifting requires water, removes suspension and loose deposit and removes much less bound staining pigment. Removed water and each pigment go to brush sinks. All exchanges retain pigment identity. Closed boundaries have zero outflow; the only material sinks are named evaporation and lifting. The tests exercise positivity, capacity, conservation, extreme face flux, edge accumulation, pore fronts, dry contact, staining, optical order and clock/resume/frame equality.

## Optical projection and limits

Per-pigment three-channel absorption/scattering coefficients compose each stratum with the finite-thickness Kubelka–Munk R/T formula over paper. The renderer applies ordered layer composition, not RGB alpha mixing. Suspension is projected as part of the active glaze. Wet refractive/gloss effects and spectral calibration are absent. The diffuse and raking views are standalone diagnostic lights, not the oil renderer's `seen_lit` gallery implementation.

The pilot's dense f64 grids and full JSON snapshots prioritize transparent conservation and deterministic testing. Snapshots validate channel dimensions, positivity, capacity and schema 1, but are not integrity-checked production checkpoints and have no untrusted-input allocation budget. Donor limits guarantee numerical nonnegativity, not timestep/grid convergence. Constants are tuned only at this fixed grid; there is no adaptive stability claim. Surface tension, explicit flocculation, individual fibers carrying pigment, binder/surfactant chemistry, brush mechanics and calibrated rewetting remain round-2 work.

Reference: Curtis, Anderson, Seims, Fleischer and Salesin, *Computer-Generated Watercolor*, SIGGRAPH 1997, §§3–5, DOI 10.1145/258734.258896.
