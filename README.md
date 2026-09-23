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
cargo paint <name> -- --full --crop 280,440,460,580 # just that window (units) at 3200px
                                                    #   → out/<name>_full_crop.png (--margin 40)
cargo paint <name> -- --ckpt                        # checkpoint after every stage
cargo paint <name> -- --resume mist                 # start from the "mist" checkpoint
cargo paint <name> -- --stop sky                    # save right after a stage
cargo paint <name> -- --no-cracks
cargo test -p paint                                 # UPDATE_GOLDEN=1 to re-record the golden scene
```

The fast loop for detail work: render the passage you're working on with
`--full --crop`, add `--ckpt` once, then iterate on the late stages with
`--resume <stage>`. The moonrise figures at full resolution on a busy
machine: 86–145 s for the whole canvas, 14 s cropped, 1.6 s cropped and
resumed from "mist". Paintings
are written in stages (`if o.stage("sky", &mut c, &mut rng) { ... }`, see
`paintings/src/run.rs`). A crop closely matches the same region of a whole
render; a resume is exact. Details and limits: `notes/workflow.md`.

Engine (`crates/paint`):
- `surface` – linen weave and ground layers as a height field in µm; wet
  layers level as they dry (Orchard's law with a yield-stress floor), thin
  fluid paint pools in the hollows, volume is conserved
- `wet` – the wet paint layer (volume, Mixbox pigment mix, hiding, stiffness)
- `bristle` – simulated brushes (round, flat, filbert, fan, rigger, badger):
  per-bristle reservoirs, bend and splay, contact with the surface relief,
  deposit, pickup and ploughing
- `handling` – how a painter covers an area: hand-like stroke planning (arcs,
  criss-cross, drift, dabs, broken strokes, pressure swell; see
  `notes/strokes.md`), passage-by-passage or swept order, trips to the
  palette, parallel tiles with exact pixel footprints
- `style` – painter profiles: support, grounds, tools, handling (from sourced
  knowledge, see `notes/research/`)
- `hand` – writing small motifs as brush gestures in a local frame
- `pigment` – Kubelka–Munk layers (Curtis et al. 1997); `color` – Mixbox, OKLab
- `crack` – craquelure grown crack by crack from film stress (T-junctions,
  weave-following on thin grounds, cupping, grime)
- `palette` – the painter's tubes and mixing on the palette; what a paint's
  color means (masstone vs. the look on the canvas): `notes/color.md`
- `stipple` – many small touches of a brush tip (Friedrich's skies, mist,
  distant hills), each simulated through the bristles: `notes/stipple.md`
- `form` – the painter's model of a solid: a depth buffer of bodies (`Sdf`
  ellipsoids and blocks, turned, cut and weathered), reliefs and mountain
  faces (`Ridge`), lit with cast shadows. It gives per-point planes, light
  and shadow families, fall lines and masks for parts, facets, silhouettes
  and edges. It paints nothing: `notes/form.md`
- `scene` – one world, one sun: a camera (horizon, ground plane, scale at
  depth), ground and water, bodies placed on the ground, cast shadows traced
  along the sun, contact seams, mirror images in still or rippled water,
  ribbons and spacing that recede. Every `Form` in it is lit alike. It
  paints nothing: `notes/scene.md`
- `canvas` – glazes, relief lighting, dithered PNG out
- `growth` – how trees grow (buds, light, vigor, pipe-model widths, decline):
  returns a skeleton of limbs, including which parts are dead wood;
  painting it is the painter's job: `notes/motifs.md`
- `mask`, `edge`, `shape`, `path`, `noise`, `rng` – geometry and randomness
  (`edge` traces a mask's outline for cutting in)
- `sched` (internal) – runs tiles of strokes in parallel without changing
  what gets painted, and skips tiles outside a crop
- `checkpoint` – the complete canvas state to a file and back (resuming)

Motifs written as gestures and the `Run`/`Finish` helpers live in `paintings/src`.

Canvas coordinates are units: always 1000 wide, `1000 / aspect` tall; the
physical size in mm comes from the style.

Two frames: `c.frame()` is the **whole canvas**. Build every mask and
`Form` on it (painting with a mask of any other size panics).
`c.window()` is the pixels the canvas actually holds, which is the whole
canvas or a `--crop` window. Masks and Forms stay whole-canvas in a crop,
so stroke planning and randomness match a whole render. The cost is that their memory and build time scale with the
canvas, not the crop. A 3200px Form costs ≈190 MB to keep and ≈310 MB at
peak (3:2); build one per motif and drop it after painting
(`notes/form.md`, Memory).

Handlings mixed from a palette aim at the *look* on the canvas by default
(`Aim::Laid`), judged over what is already there. Glazes (`Style::glaze`)
and fixed paints use the paint's *masstone* instead (`notes/color.md`).

Notes for the painter (read before writing a painting):
- `notes/workflow.md` – stages, crops, checkpoints, resuming, speed
- `notes/strokes.md` – how handlings plan strokes
- `notes/color.md` – what a color means: masstone, aimed mixing, hiding
- `notes/stipple.md` – stippling
- `notes/form.md` – solids, light, shadow and the masks they give
- `notes/scene.md` – one world and one sun: placing things, shadows, contact, reflections, perspective
- `notes/motifs.md` – trees, spruces, figures

Viewing renders
---------------
Canvas texture makes PNGs compress badly (1–20 MB), which floods an agent's
context when read as an image. View through `scripts/peek`, which writes a
≤1000px JPEG (~100–250 KB), optionally cropping first:

    scripts/peek out/foo_full.png $TMPDIR/foo.jpg            # whole image
    scripts/peek out/foo_full.png $TMPDIR/crop.jpg 700 1000 300 1700   # H W offsetY offsetX
