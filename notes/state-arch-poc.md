# Optical state without replay: isolated prototype

This branch tests whether the existing lighting renderer can consume a compact
optical snapshot without reconstructing a `Canvas` or executing Lua. It does not
change production rendering or material equations and is not a production file
format.

`paint::render_snapshot::RenderSnapshot` stores linear seen color (12 bytes per
pixel), wet-inclusive physical height (4), wet paint volume (4) and a 24-byte
header with dimensions, engine and physical scale. The snapshot is sufficient
for the current `Canvas::seen_lit` formula. The formula is deliberately copied
into the experiment so the legacy implementation remains untouched; production
integration would need an explicitly frozen renderer identity and parity gate.

The writer uses a 64 KiB staging buffer. The reader caps optical files at 16
million pixels before allocation and rejects invalid scale, unknown engine,
non-finite values, negative wet volume, truncation and trailing bytes. Geometry
is private after construction. Only whole-canvas input is supported. The source
material checkpoint decoder is unchanged: capture accepts a trusted existing
checkpoint and must run under an external resource budget.

## Run

Use the machine-wide heavy-job lock and the current disk/CPU limits. Check free
space before each command. On the shared Mac, use a job-private `CARGO_TARGET_DIR`
and `RAYON_NUM_THREADS=2`; do not replay an existing painting to create another
copy of a checkpoint that already exists.

```sh
cargo test --release -p paint --lib render_snapshot
cargo build --release -p paint --example state_arch
state_arch capture existing.ckpt optical.view
state_arch render optical.view output-directory
```

The example executable is under `$CARGO_TARGET_DIR/release/examples/` (or the
usual `target/release/examples/`). Capture writes and reloads the optical file,
then requires exact f32-channel equality with the material canvas for Color,
Gallery (115/55/1) and Relief (135/25/1). Render is a separate process that takes
only the optical file and writes those three views plus an alternate light.
No replay, material-state restoration or simulation time advancement occurs in
render mode. Compare `output-directory/color.png` with the checkpoint's original
`live.png` as a separate archived-output check.

Unit tests cover engines 1–5 with dry gloss, wet volume and solvent, three light
directions, exact field roundtrip, cropped-canvas refusal, oversized dimensions,
bad magic, truncation and trailing data. Their claim is same-binary parity, not
proof of cross-build or cross-platform identity.

## Deliberate limits

There is no pigment/layer reconstruction, Lua continuation, atomic publication,
checksum, content-addressed tile store, compression or paged working set here.
Render memory is bounded by the admitted pixel count and a fixed number of
arrays, not independent of image dimensions. Higher-resolution material physics
cannot be recovered by enlarging this snapshot. A future renderer needing new
optical inputs requires a new schema or a projection from richer material state.

The architecture report and measured receipt live in the requested shared
report, `reports/astra-state-architecture.md`, rather than being claimed as a
production performance guarantee in this repository.
