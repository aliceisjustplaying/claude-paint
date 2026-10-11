# Knife round 2 developer experiment

Default-off, branched from `capy/knife-1`. Build with `--features knife-2` and explicitly request `knife{width=30, model="knife-2", flex=0.65}`. The feature includes knife-1, which remains separately selectable. Ordinary knives still use the historical kernel. Neither the engine number nor the canvas checkpoint format changes. Experimental logs require their feature-enabled binary.

The painter API is unchanged from round 1: width, compliance, in-plane angle, pressure, tilt and terminal lift. No texture, ridge or compartment controls were added. Round 2 changes contact/material transfer for body paint at lower blade tilt; fresh fluid veils use the round-1 path. A veil regression test compares every canvas field and remaining blade quantity exactly.

## Material mechanisms

- A 1.2 mm physical contact footprint averages the dry support for cohesive body transfer. The actual support height and set color remain untouched. This removes the per-pixel thread/reach gate that made spent marks look like a dotted weave. Coherent millimeter-scale contact islands and a small adhesion film replace that lattice response. Deep set valleys still exceed the blade's reach and remain unpainted.
- Near the end of a body pass, a transverse parting front retreats by a varying physical distance. Some paint stays with the blade where the meniscus tears. Terminal release follows that front with a broken, rounded thickness profile, instead of three uniform rows at the exact final point.
- Body paint released near the blade ends can be divided between the face and a two-sample side lip. The side flow varies coherently along the pass. This moves paint from the finite bead; it does not add a height effect. Terminal flux is limited by contact area and a 1.1 mm peel thickness, with all undelivered paint retained on the blade.
- Body loads and scraped paint occupy eight persistent blade-face reservoirs. Each stores f64 paint/thinner volume, its latent mixture, material properties and cure. Only the contacted face exchanges paint; reloading adds to those local mixtures and wiping clears them. The old aggregate fields are a derived preview cache, not a second inventory. `paint_volume()` and `solvent_volume()` count the authoritative faces. Fresh fluid paint stays pooled to preserve the round-1 veil. Empty reservoirs are removed when they carry no liquid.

All new future-affecting tool state is cloned by the existing session rollback and included in knife debug digests. Contact noise is counter-based on stroke/physical position. Looking at a blade or rendering a view does not advance the material. These are bounded pilot approximations, not a beam elasticity model, a continuous wet-body solver or removable dry stratigraphy. The compartments are piecewise homogeneous and can still leave broad color bands.

## Reproduce

```sh
CARGO_TARGET_DIR=<private-target> cargo test --workspace --lib --bins --features knife-2 knife
CARGO_TARGET_DIR=<private-target> cargo run --profile test -p paint --features knife-2 --example knife_probe -- <probe-directory>
uv run --script scripts/knife_probe_sheet.py <probe-directory>
CARGO_TARGET_DIR=<private-target> cargo run --profile test -p paint --features knife-2 --example knife_demo -- <demo-directory>
uv run --script scripts/knife_demo_sheet.py <demo-directory>
```

The probe now shows baseline, knife-1 and knife-2 from cloned initial states, with three views read from each final state. Existing round-1 modes and tests are retained. The volume CSV records all three models. The morning demo executes one six-pass material-study function with identical points, loads, colors and linen/primer seeds for baseline and knife-2; only the selected knife model differs. Each pass uses a fresh knife, so its remaining paint is deliberately left off the canvas. Its native views are 1200 pixels wide. Whole comparisons use 600-pixel panels; the two Gallery details are unscaled crops. This is an engine demo for Alice, never painter guidance.

Validation covers local dirty-face continuation/reload, finite paint/thinner, protected set layers, an unchanged fluid veil, fewer thread-scale volume jumps, deep-valley skips and nonstraight terminal silhouettes with genuine side/end thickness gains. Linux's baseline-state failure, slow gate timeouts and static-export fixture/account guard failure remain separate existing issues; no golden or guard is weakened.
