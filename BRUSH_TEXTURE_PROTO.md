# Brush texture prototype — scratch experiment only

This branch starts at `67c7dff0`, the Cézanne painter's build. It is not a production engine change. Nothing is enabled by default. Do not install either experiment binary into a running studio.

Enable with `cargo build --release -p easel --features brush-texture-proto -j 2`. Build without that feature to retain the historical deposition code. The feature is deliberately not persisted in logs or checkpoints; a production version would need a recorded engine selection before anyone uses it for new paintings.

The prototype redistributes each dip's existing paint among transverse bundles, preserving the dip's total. Stiff, full-bodied paint bunches the modeled hairs and varies their effective tip length. Low loads catch the existing linen/ground/paint high points, leaving the valleys bare; the per-capsule deposition budget is redistributed onto surviving contacts, with a concentration bound. Every deposited unit still leaves a finite bristle reservoir, and wet-paint pickup remains in the ordinary exchange. No noise is applied to images or completed height fields.

All three brush routes share the mechanism: `work` uses `Held::load` and `drag_on`, ordinary strokes use the same calls, and stipple uses `Held::load` and `touch_on`. Partial loads also redistribute within the dipped part without changing its total. Soft brushes and fluid paint reduce the effect continuously rather than through brush-width exclusions.

Reproducible probes:

```sh
RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=../target nice -n 15 cargo build --release -p paint --examples --features brush-texture-proto -j 2
RAYON_NUM_THREADS=2 nice -n 15 ../target/release/examples/brush_texture_card ../renders/card-proto
RAYON_NUM_THREADS=2 nice -n 15 ../target/release/examples/brush_texture_marks ../renders/marks-proto
RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=../target nice -n 15 cargo test -p paint --features brush-texture-proto texture_proto_tests -j 2
```

The card reports initial paint, deposited paint, remaining paint, conservation error and peak wet thickness. The marks probe adds short strokes, scheduled work, scheduled stipple, and a spent brush pulled out of a wet blue underlayer. Repeat without the feature for the control.

The scratch replay renderer is selected only by `BRUSH_PROTO_RENDER_DIR`. It writes normal and historical gallery-lit native/full/crop PNGs and a raw physical height field after the replay. It does not change the canvas. The frozen 125-chunk log and the image-sheet scripts stay outside the repository, in `~/.capy/work/brush-texture-proto/source/`.

The delivered comparison, measurements, limits and production follow-up are in `~/.capy/drive/captain-stillwet/reports/brush-texture-proto.md`.
