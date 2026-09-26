# claude-paint

A physical oil-paint simulator in Rust. A painting is a program: it primes a
linen canvas, mixes piles from tube paints on a palette and moves simulated
bristle brushes through wet paint, which levels, dries on a clock and
composites by Kubelka–Munk optics. There is no image model and no reference
imagery.

## Layout

- `crates/paint/src`: the engine. The module docs at the top of each file
  are the reference.
- `paintings/src/run.rs`: the stage runner (options, stages, crops,
  checkpoints, the finish).
- `paintings/src/bin/`: one program per painting. The `study_*.rs`
  programs exercise one part of the engine each.
- `notes/guide.md`: the engine as a painter uses it.
- `notes/research/`: `friedrich_materials.md` (materials and working
  methods) and `oil_paint_physics.md` (the physics the engine models).
- `scripts/peek`: small JPEG previews of renders.

## Running

A painting `paintings/src/bin/<name>.rs` runs with `cargo paint <name>`
(an alias in `.cargo/config.toml`). Everything after `--` goes to the
program. Render at 2400 px wide, and give every run of a painting the same
width and seed: crops and checkpoints are tied to them.

```
cargo paint <name> -- --width 2400                            # → out/<name>.png
cargo paint <name> -- --width 2400 --seed 7 --out path.png
cargo paint <name> -- --width 2400 --crop 280,440,460,580     # only that window, in units
                                                              #   → out/<name>_crop.png (--margin 40)
cargo paint <name> -- --width 2400 --ckpt                     # checkpoint after every stage
cargo paint <name> -- --width 2400 --resume <stage>           # start after that stage
cargo paint <name> -- --width 2400 --stop <stage>             # save right after a stage
cargo paint <name> -- --width 2400 --resume <stage> --stale-ok --ckpt   # use a stale checkpoint and adopt it
cargo paint <name> -- --width 2400 --no-cracks                # finish without craquelure
cargo paint study_stipple -- --width 2400                     # a study
```

Canvas coordinates are units: 1000 wide and `1000 / aspect` tall, at any
pixel width.

A painting is written in stages (`if o.stage("name", &mut c, &mut rng) {
... }`). With `--ckpt` each stage's end is saved to
`out/<name>.<stage>.ckpt`, and `--resume <stage>` starts from there,
painting exactly what a full run paints. A checkpoint is refused as stale
once the engine, the helpers in `paintings/src` or the painting's code up
to the end of that stage's block has changed. A `--crop` render paints
only its window (plus a margin) at full resolution and keeps its own
checkpoints; it matches the same region of a whole render closely, not
exactly. For detail work: crop the passage, `--ckpt` once, then iterate on
the late stages with `--resume`. `notes/guide.md` section 12 has the rules.

## Viewing renders

Renders are PNGs of several MB. View them through `scripts/peek`, which
writes a JPEG at most 1000 px on a side:

```
scripts/peek out/<name>.png $TMPDIR/view.jpg                       # whole image
scripts/peek out/<name>.png $TMPDIR/detail.jpg 700 1000 300 1700   # height width y-offset x-offset, px
```
