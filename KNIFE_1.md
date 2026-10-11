# Knife round 1 experiment

Default-off. Build with `cargo build -p easel --features knife-1`, then explicitly select `knife{width=30, model="knife-1", flex=0.65}` in the Lua log. A feature-enabled binary still uses the unchanged legacy kernel for every ordinary `knife{width=30}`. Neither `paint::ENGINE` nor the checkpoint format changes. Logs that explicitly request this experimental model need that feature to replay; the ordinary build rejects the new constructor keys. This is a pilot, not a new production replay edition.

The constructor adds only `model` and `flex`. `flex` is compliance (0 rigid, 1 flexible, default 0.65): pressing a flexible blade brings it closer to the broad local support while its finite span bridges narrow valleys. The existing `width` is the blade's length in canvas units. A loaded blade has one finite, volume-weighted paint/solvent reservoir; pickup, release and side displacement transfer material between that reservoir and the active film. The baked color, set height and accumulated dry film are never cut into.

`lay` and `scrape` add `tilt`, in degrees above the canvas (0 flat, 90 upright). Defaults are 20 for laying and 70 for scraping. Low tilt gives a broad trailing contact and a thicker available gap. Upright contact narrows the working face and closes that gap. `angle` remains the blade's in-plane direction in radians; when absent the blade lies across the path. Aligning it with the path and raising the tilt produces an edge stroke.

`pressure` remains a number or a start/end pair, clamped to 0–1. More pressure closes the gap and makes the flexible blade follow broad relief more closely, leaving a thin film or picking up paint above its plane. Light pressure bridges texture and leaves thicker, broken slabs. Fluid, medium-rich paint can wet deeper tooth with a thin coating instead of filling every hollow to a slab's plane. `lift` is the share of the remaining load that tries to part from the blade at the end (default 0.1 for laying). Terminal release is limited by the contact area and a 450 µm peel thickness so a huge remaining reservoir is not dumped into a few pixels. Undelivered paint stays on the blade. `load`, `fullness` and `wipe` retain their previous meanings.

Contact has coherent, stroke-seeded variation across the face and along its travel: low-frequency flex/release, smaller tears and irregular end ridges. A spent load catches unevenly before it runs out. The deterministic noise changes contact and material transfer, never rendered colors or the final height field. Strongly pressed wet paint is sheared into the finite reservoir and carried downstream, with a share displaced around the blade ends. Pressure bends the contact down into the tooth while only open film can be removed. Fluidity comes from the existing cure model; set paint is protected. Transfers accumulate blade volume in f64 during a pass before committing its f32 persistent reservoir. This round has no removable dry strata, local blade-zone reservoirs, metal elasticity solver or continuously relaxing wet body. Mixing within a pixel and within the blade is homogeneous.

## Reproduce the probes

```sh
CARGO_TARGET_DIR=<private-target> cargo run --profile test -p paint --features knife-1 --example knife_probe -- <output-directory>
uv run --script scripts/knife_probe_sheet.py <output-directory>
CARGO_TARGET_DIR=<private-target> cargo test --workspace --features knife-1 knife
```

Each of six initial swatches is constructed once and cloned for baseline/knife-1. Color, Gallery 115/55/1 and Relief 135/25/1 are read from each final state without repainting. Sheets crop native pixels without resampling. `volume.csv` includes canvas wet paint, unchanged dry film and blade paint; pigment/binder and explicit thinner are separate inventories in mm³. The conversion is one 25 µm coat times squared physical canvas units (0.3 mm per unit). Ground is included equally in before/after totals. Tool wipes or drying during fixture setup happen before those ledgers begin.

Targeted tests cover closed paint/solvent ledgers, clipping, protected set color/height/film, wet contamination, spent blades, deterministic contact, pressure/tilt/flex effects and Lua chunk rollback. Existing historical golden and replay tests are retained. Linux's cross-architecture baseline-state and kernel hash failures must be reported separately, not re-recorded.
