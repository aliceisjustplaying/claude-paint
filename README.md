# claude-paint

A small procedural painting engine in Rust. Paintings are programs; there is
no image model and no reference imagery.

The rule: work only from what is known about each painter (materials, working
method, habits of hand, recurring motifs), never from pictures. Existing works
may be studied as exercises, but the goal is new paintings in each painter's
manner, not copies of old ones.

```
cargo paint friedrich_monk              # 1000px preview → out/friedrich_monk.png
cargo paint friedrich_monk -- --full    # 3200px        → out/friedrich_monk_full.png
cargo test -p paint
```

Engine (`crates/paint`):
- `pigment` – Kubelka–Munk glazes (Curtis et al. 1997)
- `color` – Mixbox pigment mixing (CC BY-NC 4.0), OKLab light mixing
- `canvas` – paint / glaze / veil ops, canvas weave, dithered PNG out
- `brush` – bristle strokes with paint load and dry-brush breakup
- `mask`, `shape` – coverage masks from functions or tiny-skia paths
- `noise` – fbm in canvas units

Canvas coordinates are units: always 1000 wide, `1000 / aspect` tall.

Viewing renders
---------------
Canvas texture makes PNGs compress badly (1–20 MB), which floods an agent's
context when read as an image. View through `scripts/peek`, which writes a
≤1000px JPEG (~100–250 KB), optionally cropping first:

    scripts/peek out/foo_full.png $TMPDIR/foo.jpg            # whole image
    scripts/peek out/foo_full.png $TMPDIR/crop.jpg 700 1000 300 1700   # H W offsetY offsetX
