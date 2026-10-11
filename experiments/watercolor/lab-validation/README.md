# Watercolor Lab parity and matched marks

Pinned source: `uxpreview/watercolor-lab` commit
`a19bfa2a026856a98ceb400a8eec41f091f04a11`, copyright 2026 Ryan McCarty, MIT.
The copied original modules differ only by attribution comments. Their full
license is in `experiments/watercolor/THIRD_PARTY_NOTICES.md`.

## Numeric parity

Install the pinned upstream's dependencies with `npm ci`. Chromium must have
WebGL2 and float render targets. These commands use that checkout's esbuild and
Playwright dependencies; they do not use the comparison app.

```sh
export UPSTREAM="$HOME/.capy/work/watercolor-lab-port/upstream"
export PW_CHROMIUM=/path/to/Chrome
export CARGO_TARGET_DIR="$HOME/.capy/work/watercolor-lab-port/target"
node experiments/watercolor/lab-validation/parity-original.mjs /path/to/original-parity.json
cargo run --release --manifest-path experiments/watercolor/lab-core/Cargo.toml \
  --bin lab-parity -- /path/to/original-parity.json /path/to/parity.json
```

The 16×12 reference reads RGBA/FLOAT pixels after every shader pass. Cases cover
wet transport, forced dry-fold, a closed-system diagnostic and moving brush
inputs with ultramarine, burnt sienna, salt and scrub. Every pass records all
logical water, flux, suspension, deposit, capillary, dried, paper and wetness-blur
fields. Current pass outputs replace the corresponding read fields before
swaps. Transfer pairs still read identical pre-swap inputs.

Rust starts from the original's readback values, not a separately generated
approximation. Every pass reports max/mean absolute differences, maximum
component half-float ULPs and up to four tolerance exceedances. `fields` measures
the accumulating replay; `isolated_fields` reruns that pass from its exact
original input textures. The declared component tolerance is
`max(1e-5 absolute, 4 half-float ULPs)`. The executable fails if isolated
comparisons exceed it. Accumulated exceedances remain in the report and are
never hidden or reclassified as isolated matches.

The closed-system diagnostic disables evaporation, absorption and capillary
drying, removes salt/scrub and starts every cell wet enough to prevent dry-fold.
It reports original and Rust standing-plus-capillary water and suspended-plus-
deposited pigment totals. Upstream's semi-Lagrangian gather is not conservative;
the diagnostic reports that behavior without changing it.

## Seven matched marks

`marks.json` is the single input for both capture runners. It pins a 1056×704
cold-press sheet, seed 71, French ultramarine PB29 and burnt sienna PR101. Paths
use the upstream brush's finite reservoir, pressure 0.55 and two physics
substeps per path segment. Stage programs are applied to one continuing
simulation, never four independent replays. Both views use the source render
with backlight off and X-ray off.

Build the pinned original with `npm ci && npm run build`, then run this on a
hardware-GPU machine:

```sh
UPSTREAM=/path/to/pinned/watercolor-lab PW_CHROMIUM=/path/to/Chrome \
  node experiments/watercolor/lab-validation/capture-marks.mjs /path/to/captures/original
```

The original's background RAF physics updates are disabled during scripted
capture, so only the recipe advances its state. The PNG comes directly from a 1056×704 canvas backing bitmap after an explicit
source render, so DOM borders, scaling and overlays cannot enter it. Each mark has a five-minute budget. Stop original
normal-size captures if a software renderer exceeds it; do not rerun the full
sequence on that renderer.

Run the native port with the identical recipe:

```sh
node experiments/watercolor/lab-validation/capture-native.mjs /path/to/captures/native
```

Pillow converts its PPM output to PNG without resampling. Either runner accepts
optional mark names after the output directory, for example `dry-stroke`.
Both save timing JSON incrementally and per-stage statistics over the painted
footprint: surface-depth mean/max, flow.a mean/max, sheen fraction, damp fraction
and cured fraction. Source thresholds are in `shaders.ts:396,623–625,650–652`.
The named stage files are fixed-step samples, not a claim that their wetness
classifications have been verified. If statistics do not support the requested
names, present them as Stage 1–4 until the hardware reference is measured.

The native runner also saves live pigment totals at every stage and a cloned
one-pass advection diagnostic; that extra probe never advances the captured
simulation. Dried pigment folds into optical reflectance, so the live pigment
total after dry-fold is not a full material inventory.

For a bounded normal-size original mass diagnostic without the full drying
sequence:

```sh
node experiments/watercolor/lab-validation/parity-original.mjs --normal-drift \
  /path/to/normal-original-drift.json
```

It lays the dry-stroke recipe, then measures the next single advection pass.
All numeric captures and PNGs belong in private scratch, not in the source tree.
