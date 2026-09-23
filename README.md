# claude-paint

A physical oil-paint simulator in Rust. Paintings are programs; there is
no image model and no reference imagery.

The rule: work only from what is known about each painter (materials, working
method, habits of hand, recurring motifs), never from pictures. Existing works
may be studied as exercises, but the goal is new paintings in each painter's
manner, not copies of old ones.

The second rule: first principles. Every mark is made the way a painter makes
it: bristle brushes carrying wet paint over a primed linen surface, paint that
levels and dries, layers composited by Kubelka–Munk optics. No flat fills, no
optical blends pretending to be paint.

```
cargo paint friedrich_moonrise_valley               # 1000px preview → out/<name>.png
cargo paint friedrich_moonrise_valley -- --full     # 3200px         → out/<name>_full.png
cargo paint <name> -- --width 1600 --seed 7 --out path.png
cargo paint <name> -- --stop sky                    # save right after a stage
cargo paint <name> -- --no-cracks
cargo test -p paint                                 # UPDATE_GOLDEN=1 to re-record the golden scene
```

Engine (`crates/paint`):
- `surface` – linen weave and ground layers as a height field in µm; wet
  layers level as they dry (Orchard's law with a yield-stress floor), thin
  fluid paint pools in the hollows, volume is conserved
- `wet` – the wet paint layer (volume, Mixbox pigment mix, hiding, stiffness)
- `bristle` – simulated brushes (round, flat, filbert, fan, rigger, badger):
  per-bristle reservoirs, bend and splay, contact with the surface relief,
  deposit, pickup and ploughing
- `handling` – how a painter covers an area: stroke planning, trips to the
  palette, parallel tiles with exact pixel footprints
- `style` – painter profiles: support, grounds, tools, handling (from sourced
  knowledge, see `notes/research/`)
- `hand` – writing small motifs as brush gestures in a local frame
- `pigment` – Kubelka–Munk layers (Curtis et al. 1997); `color` – Mixbox, OKLab
- `crack` – craquelure grown crack by crack from film stress (T-junctions,
  weave-following on thin grounds, cupping, grime)
- `canvas` – glazes, relief lighting, dithered PNG out
- `growth` – how trees grow (buds, light, vigor, pipe-model widths, decline): returns a skeleton; painting it is the painter's job
- `mask`, `shape`, `path`, `noise`, `rng` – geometry and randomness

Motifs written as gestures and the `Run`/`Finish` helpers live in `paintings/src`.

Canvas coordinates are units: always 1000 wide, `1000 / aspect` tall; the
physical size in mm comes from the style.

Viewing renders
---------------
Canvas texture makes PNGs compress badly (1–20 MB), which floods an agent's
context when read as an image. View through `scripts/peek`, which writes a
≤1000px JPEG (~100–250 KB), optionally cropping first:

    scripts/peek out/foo_full.png $TMPDIR/foo.jpg            # whole image
    scripts/peek out/foo_full.png $TMPDIR/crop.jpg 700 1000 300 1700   # H W offsetY offsetX
