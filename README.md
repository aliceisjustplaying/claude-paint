# claude-paint

A small procedural painting engine in Rust. Paintings are programs; there is
no image model and no reference imagery.

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
