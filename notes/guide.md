# The paint engine, as a painter uses it

This guide covers the `paint` crate (`crates/paint/src`) and the stage
runner (`paintings/src/run.rs`) in the order a painting needs them. The
module docs at the top of each source file are the reference. This guide
says what each part does physically and how to call it. The study programs
in `paintings/src/bin/study_*.rs` call these APIs in working code.

Contents:

1. Units, colors and a program
2. Canvas, ground and style
3. Brushes and gestures
4. Handlings: covering a passage
5. Palette, color and aiming
6. Stipple
7. Glazes and veils
8. Drying and time
9. Masks, forms and scene helpers
10. Pencil
11. Finish (optional): varnish, cracks, relief
12. The stage runner, crops and checkpoints
13. Viewing renders
14. What is slow

## 1. Units, colors and a program

- **Units.** The canvas is always 1000 units wide and `1000 / aspect`
  units tall, whatever the pixel width. Every position, length and brush
  width in the API is in units. At 2400 px wide a unit is 2.4 px. The
  physical size comes from the style: `Style::friedrich()` is 440 mm wide,
  so a unit is 0.44 mm and a pixel about 0.18 mm.
- **Colors** are linear-light RGB reflectances in 0..1 (`paint::Rgb =
  [f32; 3]`). `hex("#rrggbb")` converts sRGB hex to linear.
  `paint::color` has `mix(a, b, t, Mix::Pigment | Mix::Light |
  Mix::Linear)`, `gradient(&[(t, color)], t, mode)`, `shift(c, dl, da, db)`
  (a move in OKLab) and `to_oklab`/`from_oklab`.
- **Randomness** is deterministic: `Rng::new(seed)` and `Fbm::new(seed,
  octaves, period)` are seeded, and every painting call takes its own
  `seed`. The same program with the same seed paints the same picture.
- **Angles** are radians on the canvas, 0 pointing right, y pointing down.

A painting is a binary in `paintings/src/bin/<name>.rs`. Cargo finds it by
its file name, and `cargo paint <name> -- --width 2400` runs it (the
`paint` alias is in `.cargo/config.toml`). A minimal program:

```rust
use paint::{Mask, Rng, Style, hex};
use paintings::run::{Finish, Run};

fn main() {
    let o = Run::new("my_painting");
    let st = Style::friedrich();
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, 1.4, o.seed));
    let f = c.frame();
    let upper = Mask::from_fn(f, |_, y| if y < 300.0 { 1.0 } else { 0.0 });
    if o.stage("first", &mut c, &mut rng) {
        c.work(&upper, &st.broad().color(|_, _| hex("#8a8f96")), 1);
    }
    let lower = upper.clone().invert();
    if o.stage("second", &mut c, &mut rng) {
        c.work(&lower, &st.body().color(|_, _| hex("#5a4a3a")).angle(|_, _| 0.3), 2);
    }
    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}
```

Section 12 explains `Run`, stages and what may go between them.

## 2. Canvas, ground and style

### The physical surface

A `Canvas` holds a dry picture (color per pixel), a surface height field in
µm (woven linen, ground layers and paint films), a wet paint layer on top
and a clock. One coat of paint is `COAT_UM` = 25 µm of wet film.

- **Linen** (`Linen { warp_per_cm, weft_per_cm, crown_um, slubs, seed }`)
  is a plain weave whose thread crowns stand `crown_um` above the
  interstices; `slubs` makes hand-spun threads uneven. `Linen::fine(seed)`
  is 14 × 12 threads/cm with 160 µm crowns.
- **Grounds** are paint layers primed over the whole canvas.
  `c.prime(color, hiding, um, stiff, texture, seed)` spreads `um` µm, lets
  it level for its stiffness (fluid chalk-glue ≈ 0.2, oil lead white ≈ 0.6)
  and sets it; `texture` roughens it before it levels. A thick ground fills
  the weave; a thin one lets it show.
- **Leveling** (`surface`): a wet film levels by surface tension until it
  sets (Orchard's law, with a yield stress that freezes ridges smaller than
  a critical size). Stiffness sets the paint's viscosity (about 2 to 2000
  Pa·s) and yield stress (about 5 to 300 Pa). So thin fluid paint runs off
  the weave's crowns and pools in its hollows, and stiff paint keeps its
  bristle ridges. Volume is conserved.

### Style

A `Style` is data: support, grounds, tools, a palette and handling presets.
`Style::friedrich()` and `Style::friedrich_early()` are built from
`notes/research/friedrich_materials.md`:

| | `friedrich()` | `friedrich_early()` |
|---|---|---|
| width | 440 mm | 1714 mm |
| linen | 15 × 13 threads/cm | 12 × 11 threads/cm |
| grounds (bottom first) | 110 µm knife, 70 µm knife, 60 µm brushed | 110 µm knife, 70 µm knife, 30 µm rolled |
| palette | `Palette::friedrich_1820()` | `Palette::friedrich_early()` |

Both share the tools: `broad` a filbert 22 units wide, `body` a filbert 9,
`detail` a round sable 2.2, `line` a rigger 0.6 and a badger blender 40.
`relief` is `(0.06, 0.006)`, the strength and gloss for the finishing
light. The fields are public, so a variant is a struct update:

```rust
let st = Style { width_mm: 600.0, ..Style::friedrich() };
```

`st.prepare(width_px, aspect, seed)` makes the linen at the style's size
and primes each ground: `Apply::Knife { texture }` and `Apply::Roller` go
through `prime`; `Apply::Brush` spreads the paste with a 40-unit hog in
crossing strokes whose direction drifts, lays it off with the clean brush
and dries it, so fine broken bristle striations stay in the top ground.
`aspect` is width ÷ height.

Without a style: `Canvas::new(width_px, aspect, raw_color)
.with_size_mm(mm).with_linen(linen)`, then `prime` as many times as there
are grounds. The default size is 700 mm.

## 3. Brushes and gestures

### Tools

A `Tool` is a physical brush. `Held` is that brush in the hand: every
bristle has its own reservoir (volume, pigment mix, scattering). As the
handle moves:

- bristles splay with pressure and their tips trail behind the motion,
  lagging on turns;
- a bristle touches the canvas only where its reach clears the weave and
  the paint relief, so light pressure catches only the crowns (dry brush);
- each bristle deposits a share of its load in proportion to the distance
  it travels (`run` is the e-folding length) and picks up wet paint, so
  brushes get dirty, colors mix on the canvas and a clean brush blends;
- moving bristles plow wet paint aside and ahead, so ridges and stroke
  ends build up.

Presets (width in units):

| constructor | kind | character |
|---|---|---|
| `Tool::round_sable(w)` | `Round` | soft, little plowing |
| `Tool::hog_flat(w)` | `Flat` | stiff, square marks, strong ridges, broken edges |
| `Tool::filbert(w)` | `Filbert` | oval, soft-ended marks |
| `Tool::fan(w)` | `Fan` | sparse, spread bristles |
| `Tool::rigger(w)` | `Rigger` | a few long soft hairs, long load (`run` 250) |
| `Tool::badger(w)` | `Blender` | big, soft, lays nothing (`lay` 0) |
| `Tool::stippler(w)` | `Round` | short full tuft for touches |

Fields: `width`, `bristles`, `length` (tip trail), `stiffness` (0 soft ..
1 hog), `hair`, `run`, `lay` (film laid at the start of a full stroke),
`pickup`, `push` (plow), `splay`, `ragged`, `point`. Change one with a
struct update: `Tool { push: 0.02, ..Tool::filbert(9.0) }`. Painting calls
panic on a tool that `Tool::validate` rejects (non-finite fields, width ≤
0, stiffness, pickup or push outside 0..1 and so on).

**Pointed tips.** Every preset has `point: 0.0`: the mark is as wide as the
brush and pressure make it. `point` near 1 makes a cone of graded hairs:
at light pressure only the point touches (a hairline), pressing spreads
the belly, lifting draws the mark to a point, a loaded tip wets the weave's
hollows (a continuous line) and a tip run dry splits.

```rust
let tool = Tool { point: 1.0, ..Tool::round_sable(1.6) };
let w = tool.mark_width(0.4);     // units at pressure 0.4
let p = tool.pressure_for(0.5);   // pressure for a 0.5-unit mark
```

### Gestures and touches

```rust
use paint::{Gesture, Held, Orient, Paint, Tool, Touch, hex};

let mut b = Held::new(Tool::filbert(6.0), 3);          // seed: the brush's hairs
b.load(Paint::body(hex("#7a6a58")), 0.8);             // 0..1 of a full load
let g = Gesture::new(vec![(100.0, 200.0), (140.0, 190.0), (180.0, 205.0)])
    .pressure(0.7, 0.4)                                // start, end
    .ramps(0.1, 0.3)                                   // press-down, lift-off fractions
    .swell(vec![1.0, 1.2, 0.9])                        // pressure along the stroke
    .orient(Orient::Across)                            // or Along, Fixed(angle)
    .shake(1.0);                                       // 0 = mechanically exact
c.drag(&mut b, &g, None);                              // Some(&mask) clips
c.touch(&mut b, &Touch::at(300.0, 220.0).pressure(0.6).drag(0.3, 0.0).twist(0.2), None);
b.wipe(0.8);                                           // rag: remove 80 %
b.reload(Paint::body(hex("#c8c0b0")), 0.6);            // wipe clean and load
```

- `Gesture::line(a, b)` is a two-point gesture. Points are resampled along
  a spline; a non-finite point panics.
- A `Touch` presses the tip straight down and lifts it: the hairs splay
  outward and the patch grows with pressure; each hair deposits by film
  splitting (a set thickness per load, not a share per distance) and lifts
  some wet paint, so touches into wet paint blend.
- `Held::fullness()` is the paint left, relative to a full load.
- A clip mask multiplies each bristle's contact by the mask; paint a
  bristle can't lay stays on it.

**Hand frames.** `Hand::new(at, size, seed)` sets a local frame (origin in
units, units per local unit, `v` up) for writing a small group of gestures
in its own coordinates: `take(tool_fn, width, paint, load)` returns a
`Held`, `line(c, &mut held, pts, p0, p1)`, `dab(c, &mut held, u, v, len,
dir, p)` and `mark(c, &mut held, Mark { .. }, clip)` draw in local
coordinates.

**Drawn outlines.** `paint::outline::Outline::draw(pts, corners, closed,
Character::firm(), seed, None)` turns a few points into a hand-drawn line
(wobble, facets or lobes, lifts, gaps, overshoots, restated stretches) and
gives its `mask(f)`, `band(f, width, taper)`, `below(f, bottom)`,
`above(f)`, `gestures(pressure, shake)` and `paint(c, &mut held, pressure,
clip)`. Characters: `firm`, `searching`, `broken`, `soft`
(`Character::named`), scaled by `.amount(k)`. `Outline::body(&[Bone], ..)`
grows a silhouette from a skeleton of limbs with widths.

## 4. Handlings: covering a passage

`c.work(&mask, &handling, seed)` plans strokes over a region and paints
them one by one through the bristle simulation. A `Handling` says which
tool, how the strokes run and how the brush is loaded.

### Planning

- **Placement.** The region (plus a margin past the canvas edges, so
  strokes brush in from outside) is cut into passages about 1.6 stroke
  lengths square. Each passage has its own lattice of stroke centers aligned
  with its stroke direction, jittered cell by cell; `clump` thins and
  crowds them by a slow density field. Lattices of neighboring passages
  don't line up.
- **Strokes.** Each stroke follows the direction field (`angle`, plus
  `angle_jitter`, wandering by `drift`), bows into an arc or S (`curve`),
  falls into one of two families at ± `cross`, takes a length from the
  range with a tail of dabs and long sweeps (`tail`), sometimes lifts and
  restarts off the line (`broken`) and varies its pressure along its length
  (`swell`). `ruler()` switches all of that off (straight, even, evenly
  spread, random order).
- **Order.** `Order::Passages` (default): passage by passage, a trip to the
  palette at each new passage. `Order::Sweep(angle)` or `.sweep(angle)`:
  band by band across the whole area (`FRAC_PI_2` is top to bottom), so a
  blender never drags paint back across a gradient. `Order::Scatter`:
  random.
- **Edges.** `hug` (default on) moves centers seeded just outside the
  region onto its edge, so the edge gets as many strokes as the middle;
  unclipped strokes reach a little past it. `threshold`
  (default 0.3) is the least mask value for a stroke center.
- **Look and fill.** After its strokes, a pass that means to cover
  (coverage ≥ 1.5, load ≥ 0.25, not a blender or a scrub) finds the
  spots it left bare (under 0.04 coat) on a grid of cells half a brush wide
  and lays one short stroke through each, once. `.fill(false)` leaves the
  ground between strokes; `.fill(true)` fills at any coverage.

### The knobs

| builder | meaning | `Handling::new` default |
|---|---|---|
| `length(a, b)` | stroke length range, units | 30–90 |
| `coverage(c)` | stroke area ÷ region area; 0 disables the pass | 2.0 |
| `angle(\|x, y\| ..)`, `angle_jitter(sd)` | direction field | 0, 0.08 |
| `curve(bow, wave)` | sagitta ÷ length (sd); share of S-curves | 0.05, 0.25 |
| `cross(a)` | two families at ± a | 0 |
| `drift(amount, scale)` | direction wander, radians over units | 0.12, 300 |
| `tail(share)`, `broken(share)`, `swell(sd)`, `clump(k)` | see above | 0.12, 0.06, 0.15, 0.3 |
| `pressure(a, b)` | per-stroke range | 0.6–0.9 |
| `ramps(attack, release)` | press-down and lift-off fractions | 0.08, 0.15 |
| `orient(Orient)` | wide axis across, along or fixed | `Across` |
| `dips(every, load, wipe)` | strokes per palette trip, load, share wiped first | 1, 0.64, 0.6 |
| `blender()` | clean brush, never loads | off |
| `scrub(n)` | short back-and-forth strokes | 0 |
| `shake(k)` | hand unsteadiness | 1 |
| `clip(on)` | stencil each bristle to the mask | off |
| `limit(Arc<Mask>)` | hard mask no bristle paints outside | none |
| `fence(Arc<Fence>)` | per-stroke overrun at the edge (section 9) | none |
| `cut_in(tool)` | body strokes stop short; the tool runs along the outline just inside | none |
| `load_at(\|x, y\| ..)` | multiplies the load at each stroke's center | none |
| `fill(on)`, `hug(on)`, `threshold(t)` | see above | auto, on, 0.3 |

Color and paint: `color(|x, y| rgb)`, `color_over(|x, y, under| rgb)`,
`mixed(&palette, medium)`, `palette(&p)`, `medium(m)`, `mix_jitter(sd)`,
`by_masstone()`, `aim(coats)`, `aim_laid()`, `paint(hiding, stiff)` and
`jitter(l, hue)` (section 5).

### Style presets

Each preset is a `Handling` with the style's tool, palette and medium and
a hand, but no direction of its own; set `angle`, `cross`, `order` and the
rest on it.

| preset | tool | strokes | coverage | medium |
|---|---|---|---|---|
| `st.broad()` | broad filbert | 80–220, long arcs, drift 0.22 over 350 | 2.5 | `thin_medium` 0.45 |
| `st.body()` | body filbert | 20–60, more bowed and broken | 2.5 | `body_medium` 0.2 |
| `st.detail()` | round sable | 4–14, clipped, threshold 0.1 | 3.0 | 0.1 |
| `st.hatch()` | round, 1.2 × detail width | 5–12, nearly straight, clumped | 2.5 | 0.15 |
| `st.glaze(medium)` | soft filbert | 120–300, by masstone | 2.5 | as given |
| `st.blend()` | badger (`Option`) | 120–300, ± 0.2 crossing, clipped, swept top to bottom | 3.0 | none |
| `st.scumble()` | body filbert | scrub 3, 10–20 | default | 0.5 |

`st.line_tool(width)` is the style's rigger at another width.

```rust
use std::f32::consts::FRAC_PI_2;
c.work(&region, &st.broad().color(field).angle(|_, _| 0.1).coverage(3.0), 11);
c.work(&region, &st.blend().unwrap().clip(false), 12);          // fuse across the mask's edge
c.work(&region, &st.body().color(field).cross(0.3).fill(false), 13);
c.work(&region, &st.broad().color(field).sweep(FRAC_PI_2), 14);
```

`work_with(&mut piles, ..)` does the same as `work` but dips into a shared
`tally::Piles` (section 8).

## 5. Palette, color and aiming

### What a color means

- **Masstone.** `Paint::color` and `Tube::color` are the masstone: the
  color of the paint laid thick, which is also how it looks laid over paint
  of the same color at any thickness. A mark mixed to the masstone of the
  field it sits in disappears into it.
- **Hiding** is the contrast ratio of one coat (its look over black ÷ over
  white): about 0.05 a glaze, 0.5 a scumble, 0.92 body color. It converts
  to a Kubelka–Munk scattering S per coat, flat across channels; the
  absorption per channel follows from S and the masstone.
- **Mixing** is two-constant KM with Mixbox for hue: the masstone mixes in
  Mixbox latent space (on the palette by volume × tinting strength, in the
  wet layer by volume); S mixes by volume. **Medium** multiplies K and S by
  (1 − medium) and stiffness by (1 − medium)², leaving the masstone.
- **Layers** composite by KM (`Pigment::over`): a thin coat over a
  different color lands between the two.

`Paint` constructors: `Paint::new(masstone, hiding, stiff)`,
`Paint::body(c)` (0.92, stiff 1), `Paint::scumble(c)` (0.5, 0.6),
`Paint::km(c, scatter, stiff)`, `Paint::tint(tint, hiding, stiff)` and
`Paint::glaze(tint)` (named by the look of one coat over white),
`Paint::aimed(want, under, coats, hiding, stiff)`. Modifiers:
`with_hiding`, `with_stiff`, `with_drying`. `p.over(under, coats)` is how
`coats` of it look over `under`.

### The palette

A `Palette` is a set of `Tube`s (`name`, masstone `color`, `hiding`,
`stiff`, tinting `strength`). Every pile is mixed from at most three tubes.

- `Palette::friedrich_early()`: lead white, smalt, pale smalt, yellow
  ochre, red earth, vermilion, raw umber, bone black.
- `Palette::friedrich_1820()`: the same without smalt, plus cobalt blue and
  chrome yellow.
- `friedrich_early_greens()` and `friedrich_1820_greens()` add Prussian
  blue and green earth (the second also Rinmann's green);
  `Palette::copper_green()` is one extra tube.
- `pal.only(&["lead white", "raw umber"])` restricts to named tubes (panics
  on an unknown name), `pal.with(vec![tube])` adds tubes.

Calls:

```rust
let m = pal.mix(want);                            // masstone nearest `want` (OKLab)
let m = pal.aim(want, under, medium, coats);      // look nearest `want` laid over `under`
let p: Paint = m.paint(medium);                   // thinned, ready to load
let p = pal.paint(want, medium);                  // mix + thin
let p = pal.paint_for(want, under, medium, coats);
let p = c.aim(&pal, want, (x, y), r, medium, coats); // judged on the canvas at (x, y)
println!("{}", pal.recipe(&m));                   // "lead white 0.72 + ..."
```

`Mixture::error` is the OKLab miss. A target out of reach comes out as the
nearest pile the tubes make: a transparent glaze can't lighten a dark; a
scattering paint (lead white, ochre) thinned with medium can.

### Aiming: masstone versus the look on the canvas

A handling with `mixed(&pal, medium)` treats its color field as the look
wanted on the canvas (`Aim::Laid`). For each stroke it samples what is
under the stroke before the pass (nine discs along the path, weighted
toward the loaded start, combined by a median per OKLab channel) and picks
the pile whose mark will look like the color there when laid as thick as
this handling lays paint. That thickness is `Handling::laid_coats()`:
1.3 coats per unit of load for filberts, flats, fans and blenders, 4.0 for
rounds and riggers, times the mean overlap `c / (1 − e^−c)` at coverage
`c`, clamped to 0.2–8. The pile is scored by the mean look over a spread
of thicknesses (a pointed brush lays a thick core and thin edges), with
costs that keep its thin edge on the way from the underlayer to the
target, keep its masstone in the target's hue family and add a small cost
per tube.

- `.aim(coats)`: aim, expecting this thickness.
- `.by_masstone()`: the color field is the masstone; nothing is sampled.
  `st.glaze(..)` uses it, so `load_at` alone sets a glaze's depth.
- `.paint(hiding, stiff)`: a fixed paint whose color is its masstone
  (`.aim(coats)` solves the masstone for its hiding instead).
- `.color_over(|x, y, under| ..)`: the target relative to what is there,
  e.g. `shift(under, -0.06, 0.0, -0.03)` is darker and bluer than the
  canvas under each stroke. It replaces `color`.
- `.mix_jitter(sd)`: each dip remixes the pile with proportions jittered
  by `sd` (relative). Every pile has its own random stream, so planning is
  the same in a crop.
- `Canvas::under(x, y, r)` is the plain mean of dry picture plus wet paint
  in a disc; `Canvas::judge_under(x, y, r)` is the median judge `c.aim`
  uses; `Canvas::seen()` is every pixel with its wet paint on it.

Aim results are cached by quantized target, underlayer, thickness and
medium, and computed from the key, so they don't depend on call order.
`Palette::remix` gives a jittered copy of a mixture.

## 6. Stipple

`c.stipple(&mask, &Stipple, seed)` covers a region with touches of a tip
(each a `Touch` through the bristles). Tone comes from how densely touches
fall and their color; the marks have no direction.

```rust
use paint::{Stipple, Tool, hex};
let s = Stipple::new(Tool::stippler(2.0))
    .mixed(&pal, 0.5)
    .color(|_, _| hex("#b8b4a8"))
    .coverage(|_, y| 1.5 * (y / 400.0).min(1.0))   // touches per point
    .pressure(0.4, 0.75)
    .dips(24, 0.5, 0.5);
c.stipple(&region, &s, 21);
```

- **Placement.** A jittered grid thinned by `coverage(x, y) × mask`,
  clumped by `cluster(amount, size)`. Each tile is worked patch by patch;
  one dip serves about one patch and its pile is aimed at the centroid of
  the touches it serves.
- **Mark size.** A touch covers about π(0.4 · width)² at mid pressure;
  harder touches make bigger, fuller marks.
- **Color.** By default each dip is aimed so the stippled passage (about
  `coverage` touches thick, at least one) dries to `color` over what the
  canvas shows there. `.aim(false)` mixes `color` directly (density builds
  the veil). `color_over` works as in handlings.
- **Thin coverage.** `feather(k)` (default 0.6): where coverage < 1, touches
  press less. `fade(k)` (default 1): where coverage < 1, each touch is
  aimed nearer to what it sits on, `min(1, c)^fade` of the way. 0 turns
  either off.
- **Other knobs.** `drag(len, Some(angle) | None)` (hand drift while the
  tip is down), `twist(sd)`, `clip(on)`, `limit(Arc<Mask>)`,
  `paint(hiding, stiff)` and `jitter(l, hue)` without a palette.
  Defaults: pressure 0.4–0.75, dips every 24 touches at load 0.5, wipe 0.5.
- Touches into wet paint lift and fuse with it; over dry paint they sit on
  top.

## 7. Glazes and veils

Two ways to lay a thin transparent layer.

**`Canvas::glaze(&pigment, mask, |x, y| coats)`** lays a film of
`pigment`, `coats` deep (times the mask), over the whole dry picture in
one step. It first dries every wet film on the canvas (it calls `dry()`).
The film is mostly medium: 0.3 coat of film per coat of color depth. It
levels and pools in the hollows of the surface, so it is deeper there, and
it dries at once. A request thinner than 0.05 µm of film fades out
smoothly, so a long falloff ends without an edge.

Pigments for glazes are named by the look of one unit layer over white:
`Pigment::transparent(c)` (hiding 0.06), `Pigment::semi(c)` (0.45),
`Pigment::opaque(c)` (0.92), `Pigment::varnish(c)` (0.004, absorbs only),
`Pigment::with_hiding(c, h)`, `Pigment::from_appearance(on_white,
on_black)`. `Pigment::masstone(r, s)` and `masstone_hiding(r, h)` name one
by masstone. `pig.over(under, coats)` previews it.

```rust
c.glaze(&Pigment::transparent(hex("#6a5030")), Some(&region), |x, _| 0.2 + 0.6 * (x / 1000.0));
```

**A brushed glaze or scumble** is wet paint: `st.glaze(medium)` is a soft
filbert laying paint that is mostly medium (≈ 0.85–0.95 for a transparent
glaze, ≈ 0.6 for a veiling scumble) in long strokes, mixed by masstone.
Vary its depth with `load_at` and fuse it with `st.blend()`; it dries with
the clock like any paint.

```rust
let g = st.glaze(0.9).color(|_, _| hex("#5a4632")).load_at(|x, _| 0.5 + 0.5 * (x / 1000.0));
c.work(&region, &g, 31);
```

## 8. Drying and time

The canvas has a clock in minutes. Oil paint dries by oxidation, and each
pixel's open film carries its own cure (0 fresh, 0.15 the gel point, 1
touch-dry). What a brush feels depends on the stage (`paint::Stage`):

| stage | cure | what brushes do |
|---|---|---|
| `Open` | < 0.075 | blends, lifts, plows, levels |
| `Setting` | 0.075–0.15 | lift and plow fall with fluidity; marks stop leveling |
| `Tacky` | set film | doesn't flow, mix or lift; grabs the brush: deposit up to 3× faster, in stick-slip patches |
| `Dry` | ≥ 1 | new paint sits on top without mixing |

- **Rate.** Cure per minute is `drying / (1440 × thick × fat)`: one lean
  25 µm coat of average paint is touch-dry in about a day; `thick` =
  (film coats)^0.7, at least 0.5, with the thickness judged over a patch of
  about 1.25 mm; `fat` = 1 + 0.6 (1 − stiffness), so medium-rich paint dries
  slower. `drying` is the paint's rate: `paint::drying::drier` has lead
  white 2.0, umber 2.4, chrome yellow and Prussian blue 1.8, smalt 1.6,
  cobalt 1.4, sienna 1.2, red earth 1.0, ochre and ultramarine 0.8,
  vermilion and bone black 0.4, lamp black and zinc white 0.35, madder 0.3.
  Paint mixed from a palette dries at 1.0; `Paint::with_drying(rate)` sets
  another rate on a paint loaded by hand.
- **Leveling** happens only while a film is fluid: it levels for 900 s
  after it is last worked (less if worked while setting). At the gel point
  it bakes into the dry picture; its tack lives on until it is touch-dry,
  and new wet paint over it never mixes with it. Fresh paint worked into an
  older film dilutes its cure by volume.

Calls:

```rust
c.wait(0.0);          // nothing passes: keep working wet into wet
c.wait(30.0);         // half an hour
c.wait(180.0);
c.wait(24.0 * 60.0);  // a day
c.dry();              // until every film is touch-dry
let t = c.clock();    // minutes (f64)
let s = c.drying_at(x, y);      // Stage at a point
let shares = c.stage_shares();  // [open, setting, tacky, dry] over the kept pixels
let um = c.wet_um(x, y);        // wet film at a point, µm
```

A negative wait counts as 0 and an infinite one is `dry()`. The clock
doesn't start at 0: `Style::prepare` dries its grounds, so read `c.clock()`
and count from there. A program that never waits works wet into wet until
something dries it: `dry`, `glaze`, `prime`, `relief` and `save` all dry
the canvas first.

### Hand time

Every planned stroke, touch, palette trip and wipe is priced in seconds of
hand time and counted in a ledger (`c.tally()`: strokes, touches, path
length in mm, reloads, piles mixed, wipes, pencil lines, seconds). The
prices come from Fitts's law for moving to the next mark, the steering law
for drawing it (`paint::tally::pace`), 2.5 s per reload, 20 s per new pile
and 2 s per wipe. The ledger never changes what is painted.

With `c.set_hand_time(Some(slice_min))` the paint ages while the hand
works: `work` and `stipple` paint in slices of about `slice_min` minutes of
hand time with a `wait` between them, and the default order becomes a
sweep down. The last slice of each pass stays owed; `c.clock_hand_min()`
puts what is owed on the clock (`c.hand_owed_secs()` reports it).
`None` (the default) keeps marks off the clock.

`tally::Piles` is the palette's piles: a dip within 0.035 OKLab of a pile
is a reload, anything else a new pile, 16 at most. `work_with` and
`stipple_with` share one `Piles` across passes; `work` and `stipple` start
a clean one.

## 9. Masks, forms and scene helpers

### Masks

A `Mask` is float coverage the size of the **whole** canvas, built on
`c.frame()` (a mask of another size panics when painted with).

```rust
let f = c.frame();
let m = Mask::from_fn(f, |x, y| if y > 0.4 * x + 200.0 { 1.0 } else { 0.0 });
let s = Mask::from_shape(f, Shape::new().ellipse(500.0, 300.0, 120.0, 80.0));
let soft = s.clone().blur(3.0);                        // radius, units
let rough = s.clone().roughen(7, 25.0, 3.0, 1.5);     // seed, period, amount, edge (units)
let both = m.clone().union(&s);                        // max
let cut = m.clone().subtract(&s);                      // m × (1 − s)
let ring = s.rim(6.0, 2.0);                            // inside, within 6 units of the edge
let grown = s.offset(4.0);                             // dilate(d), erode(d)
let dist = s.distance();                               // signed distance, units (+ inside)
let band = dist.band(2.0, 10.0, 1.0);                  // 1 between two levels of a field
let edge = s.soften(|x, _| if x < 500.0 { 0.0 } else { 6.0 }); // edge ramp width by place
```

Also `map`, `mul`, `mul_fn`, `invert`, `sample(x, y)` and `Mask::full` /
`Mask::empty`. `Shape` builds paths in units (`move_to`, `line_to`,
`quad_to`, `cubic_to`, `rect`, `ellipse`, `circle`, `poly`, `smooth_poly`,
`below(curve, bottom)`, `ribbon(pts, widths)`, `add`).

`f.per_column(|x| ..)` tabulates a function of x once per pixel column and
returns a `Copy` reference, so a mask closure that calls it per pixel
evaluates it once per column. `Fbm` is `Copy` too; both go into any number
of `move` closures.

`paint::edge::contours(&mask, step)` traces the mask's 0.5 line into
polylines with inward normals.

### Edges: clip, fence, limit

- `clip(true)` is a stencil: every bristle's contact is multiplied by the
  mask, so every stroke stops on the same line.
- A **fence** (`paint::fence`) carries the passage to the edge the way a
  brush does. `Fence::new(&region, &quality, width, waver, reach, seed)`:
  the region's edge (moved by a slow waver) and a quality field (0 found,
  0.5 soft, 1 lost). Each stroke overruns the edge by its own amount; past
  the fence the hairs lift off the weave's hollows and the film thins to
  nothing over a distance that grows from found to lost.
  `fence::stretches(&region, (found, soft, lost), period, band, seed)` lays
  the qualities out in runs along the contour.
- `limit(Arc<Mask>)` is a hard mask on top of either: nothing paints
  outside it, while strokes still overshoot the region's own edge.

```rust
use paint::fence::{Fence, stretches};
use std::sync::Arc;
let q = stretches(&region, (0.3, 0.5, 0.2), 60.0, 20.0, 5);
let fence = Arc::new(Fence::new(&region, &q, 9.0, 1.0, 1.0, 5));
c.work(&region, &st.body().color(field).fence(fence), 41);
```

### Noise

`paint::noise`: `Fbm::new(seed, octaves, period)` (`get` ≈ −1..1, `get01`),
`Octaves::{new, ridged, billow}` in 2-D and 3-D with level of detail,
`Warp::new(seed, period, amount)` (domain warping), `Aniso::new(angle,
stretch)`, `Worley::new(seed, period)` (cells: `f1`, `f2`, `edge()`, `id`),
`uneven(n, lo, hi, irregular, clump, seed)` (positions with unequal gaps),
`vary(v, amount, i, seed)` and `rand01(i, j, seed)`.

### Form: a lit depth buffer

`Form` holds, for every pixel of the whole canvas, the nearest solid, its
depth `z` (units toward the eye), normal, part id, facet id and a distance
for aerial perspective. It paints nothing; it gives fields and masks.

```rust
use paint::{Form, Light, Sdf};
let mut form = Form::new(c.frame());
let body = Sdf::block([500.0, 400.0, 0.0], [200.0, 120.0, 150.0], 10.0)  // center, whole size, rounding
    .turn([500.0, 400.0, 0.0], 0.4, 0.2, 0.0)                              // yaw, pitch, roll
    .cut([560.0, 360.0, 60.0], [1.0, -0.5, 0.6], 7, 2.0)                   // plane, facet id, rounding
    .rough(3.0, 50.0, 9, false);
let part = form.add(&body, 10.0);                                          // distance for aerial perspective
form.light(Light::new((-1.0, -0.6), 0.4).ambient(0.15).penumbra(0.06));
let lit = form.mask(|s| if s.part == part { s.shade.lit(0.2) } else { 0.0 });
let edges = form.edges(0.5, 4.0, 3.0);                                     // plane breaks and overlaps
let dir = |x: f32, y: f32| form.fall(x, y);                                // down the planes
```

- Solids: `Sdf::ellipsoid(center, radii)`, `Sdf::block(center, size,
  round)` (size is the whole extent), `half_space`, `cut`, `union`,
  `subtract`, `turn`, `rough`, `facet`; `Ridge::new(x0, x1, crest, depth,
  seed)` (a face with spurs and gullies down from a crest line: `lean`,
  `gullies`, `fan`, `strata`, `z0`, `base`); `Relief::new(area, |x, y| ..)`
  for any height function. Coordinates: x right, y down, z toward the eye.
- `Light::new(from, front)`: `from` is the light's direction in the
  picture, `front` its component toward the eye (negative: from behind the
  solid). `ambient`, `bounce`, `penumbra`, `reach`, `thickness`,
  `across_parts`.
- Per point: `sample(x, y)` (`Sample`: part, facet, z, n, dist, shade),
  `shade(x, y)` (`Shade`: `turn` = n·L, `direct`, `cast`, `bounce`, `sky`,
  `value`; `lit(soft)` is membership of the light family), `fall`,
  `across`, `part`, `dist`, `bend` (convex +, concave −), `edge_angle`.
- Masks: `mask(|s| ..)`, `silhouette(&parts, |s| soft_units)`, `edges(turn,
  step, span)`. `paint::form::aerial(dist, visibility)` is the share of a
  color lost to the air.
- Memory: about 28 bytes per pixel of the whole canvas (≈ 115 MB at
  2400 × 1714 px). Build one, take its masks and fields, then drop it.

`paint::rock::Rock::grow(outline, corners, cracks, plane_lines, &spec,
light, seed)` infers a faceted solid behind a drawn closed outline (optional
crack and plane lines inside it) for `RockSpec::granite()`, `sandstone()`
or `chalk()`, and returns masks for its planes, light and shadow families,
cracks, contact seam and cast shadow, plus stroke directions.

### Scene: one camera, one sun

`paint::scene` places solids on a ground under one sun, so they share
light, scale and shadows. World coordinates are meters: X right, Y up, Z
away; the eye is at (0, eye, 0) looking level, so the horizon is the eye
level line.

```rust
use paint::{Sdf, Sun, Water, World};
let h = c.height();
let mut w = World::new([0.0, 0.0, 1000.0, h], 0.55 * h, 1.6)   // view [x, y, w, h], horizon y, eye m
    .sun(Sun::deg(-40.0, 25.0))                                  // azimuth from straight ahead, elevation
    .water(Water::new(-0.3).ripple(0.02, 3.0, 1.5, 4));
let spot = w.spot(420.0, 0.8 * h).unwrap();                     // the ground seen at a canvas point
let id = w.place(spot, Sdf::ellipsoid(spot.p(0.0, 0.6, 0.0), spot.size(1.2, 0.6, 1.0)));
let view = w.view(c.frame());
let (ground, shadows, contact) = (view.land(), view.shadows(), view.contact(0.1));
let reflected = view.reflections(&[id]);
```

- Camera: `fov(width, deg)`, `project`, `to_ground`, `scale_at(z)`,
  `height(x, y, meters)` (how tall something that size looks standing at a
  canvas point), `aerial(z)`, `shadow_angle(x, y)`, `sun_canvas()`.
- Ground and water: `ground(|x, z| height)`, `Water::new(level)` with
  ripples; `is_water`, `water_depth`.
- Bodies: `place` (visible, in the view's `Form`) and `proxy` (casts
  shadows and shows in water but isn't in the `Form`); `Spot::p(right, up,
  toward)`, `m(meters)`, `size(w, h, d)`; `recede(start, step, n)` for
  spacing that recedes; `ribbon(pts, width)` for a band lying on the ground.
- `View`: `form` (the visible bodies lit by the sun), `at(x, y)` (`Point`:
  what, where, normal, distance, shade), `sky()`, `land()`, `water()`,
  `shadows()`, `contact(reach)`, `mirror(x, y)` (where the reflected thing
  is on the canvas, Fresnel share, ray length), `reflections(&bodies)`,
  `soft_shadows`, `occlusion` and `depths()`: what lies behind what at
  every pixel, with masks `visible`, `front`, `behind`, `at_depth`,
  `between`. `World::layer(name, mask, LayerDepth)` registers a hand-painted
  region at a depth so these masks account for it.

### Atmosphere

`paint::atmos` gives, for the same `Sun`, fields that paint nothing:

- `Sky::new(sun).haze(h).uneven(amount, period, seed).layer(..)
  .overcast(o)`: single scattering by air and haze in a spherical
  atmosphere with ozone. `SkyField::new(sky, &world, cell)` samples it over
  the camera and maps it into paint's range (`at(x, y)`, `value`,
  `airlight(x)`, `exposure(k)`, `balance(light, amount)`).
- `Cloud::cumulus`, `bank`, `stratus` (meters; `density`, `soft`, `wind`,
  `breaks`, `heap`, `reach`), `Clouds::new(vec![..]).field(&sky_field,
  &world, cell)`: a `CloudField` with `alpha`, `lit`, `glow`, `ambient`,
  `soft`, `dist`, `color` and `mask`.
- `Haze::new(visibility_m).height(h).mist(top, density, uneven, seed)` and
  `loss(eye, dist, h, x)`: the share of a color lost to the air along a
  line of sight.
- `Ranges::new(near, far, count, seed).heights(..).irregular(k)
  .oblique(k).build(&world)`: receding crest lines in world meters; each
  `RangeLayer` gives `crest(&world, x)`, `haze(&world, &air, x, y)` and a
  `form::Ridge` for its face.

## 10. Pencil

`paint::graphite` draws on the ground before paint. A point is dragged over
the tooth: it rides on the local tops of the surface and pressure lets it
bite into the hollows, so a light line breaks up in the weave and a heavy
one fills in. The deposit covers a share of each pixel with flakes of a
given reflectance and goes into the dry picture, so later paint composites
over it by KM: a thin layer lets it show, body color hides it. Wet paint
doesn't take graphite.

- Leads: `Lead::pencil("2H")` (9H..H, F, HB, B..9B; `None` for anything
  else), `Lead::graphite(softness)`, `Lead::chalk()`. Harder leads lay less,
  paler and stay sharp; softer ones lay more, darker and blunt faster
  (`width_mm(worn_mm)` grows toward 3× the point). Chalk is matte, near
  black, broad and crumbly.
- Lines: `graphite::hand_line(pts, profile, smooth, ruler, tremor, seed)`,
  `sketch_marks(pts, pressure, passes, wander, smooth, tremor, seed)` (a
  searching line of several light passes), `hatch_marks(&mask, angle,
  spacing, length, pressure, seed)`. `profile` gives the pressure at evenly
  spaced stations along the line.
- `c.draw(&lead, &mark, worn_mm, seed)` returns the mm drawn (the point
  wears by that much). `c.erase(&mask, strength)` lifts loose drawing,
  more from the tops than the hollows, never all of it. `c.fix_drawing(None
  | Some(&mask))` binds it. Paint over it seals it.
- `c.drawing_guide()`: the drawn lines as an unbroken mask over the whole
  canvas (also in a crop), to plan passes along them;
  `c.drawing_mask()`: the grainy deposit itself (window pixels only);
  `c.drawing_view()`: the drawing alone on white.

```rust
use paint::{Lead, graphite};
let lead = Lead::pencil("2H").unwrap();
let mark = graphite::hand_line(&[(120.0, 400.0), (420.0, 380.0), (760.0, 410.0)], &[0.3, 0.5, 0.35], true, false, 0.2, 7);
let mut worn = 0.0;
worn += c.draw(&lead, &mark, worn, 7);
let lines = c.drawing_guide().band(0.7, 1.0, 0.1);
```

## 11. Finish (optional): varnish, cracks, relief

`o.finish(&mut c, &mut keep, &finish)` ends the last stage, dries the
canvas, then applies a `Finish`:

- **Varnish**: `Pigment::varnish(finish.varnish)` glazed
  `varnish_coats ± varnish_vary` coats (varying by an fBm 400 units
  across): a clear film that only absorbs.
- **Cracks** (`finish.cracks: Option<Cracks>`, skipped with `--no-cracks`):
  `c.crack(&cracks)` grows craquelure one crack at a time in a stress field
  sized in mm. Cracks run perpendicular to the largest stress and later
  ones meet earlier ones at T-junctions; islands are about 14 times the
  layer thickness (ground plus 20 µm of paint, kept within 1.2–7 mm:
  ≈ 3.6 mm on a 240 µm ground); thin
  grounds (under 100 µm) pull cracks onto the weave; corners crack across
  the diagonals; island edges cup; grime settles in the cracks.
  `Cracks::aged(seed)` fits islands, weave coupling and openings to the
  canvas's ground and varies them with the paint under each crack;
  `Cracks::even(seed)` is one even network. Fields: `island_mm`,
  `ground_um`, `width_um`, `depth_um`, `cupping_um`, `dirt`, `corners`,
  `vary`, `veil`, `hierarchy`, `patchy`, `grain`, `grime`.
- **Relief**: `c.relief(strength, gloss)` lights the surface height field
  from the upper left at about 35° (weave, ridges, cracks) with a faint
  sheen on the ridges.

`Finish::aged(st.relief)` is varnish `#e6d3a4` at 0.4 ± 0.12 coats,
`Cracks::aged` and the style's relief. Every field is public:

```rust
let fin = Finish { cracks: None, varnish_coats: 0.2, ..Finish::aged(st.relief) };
o.finish(&mut c, &mut rng, &fin);
```

To end without any of it: `o.end(&mut c, &mut rng);` then optionally
`c.relief(..)`, then `o.save(&mut c);`.

## 12. The stage runner, crops and checkpoints

### Run

`Run::new(name)` reads the command line: `--width N` (use 2400), `--seed N`
(default 1), `--out path`, `--crop`, `--margin`, `--stop`, `--ckpt`,
`--resume`, `--stale-ok`, `--no-cracks`. Call it from the painting's own
file: it reads that file's stage names. `o.width`, `o.seed` and `o.crop`
are public.

- `o.canvas(|| make)` returns a fresh canvas from `make`, or on `--resume`
  the canvas from the checkpoint (then `make` doesn't run).
- `o.stage(name, &mut c, &mut keep)` begins a stage and ends the one
  before: it prints the time, writes the checkpoint with `--ckpt` and exits
  with `--stop` at that stage. It returns false when a resumed run skips
  the stage.
- `o.end(&mut c, &mut keep)` or `o.finish(..)` closes the last stage.
- `keep` is the state carried from stage to stage besides the canvas,
  saved in checkpoints: anything implementing `run::Keep` (`Rng`, `()` and
  pairs of `Keep`s implement it). Pass the same value to every `stage` and
  to `end`/`finish`.

Rules:

- Paint only inside stage blocks. Code between blocks runs on every run,
  resumed or not: keep it to masks, fields, geometry and constants.
- Don't draw from the `keep` state between blocks (a resumed run would
  draw from a different state). Geometry built from random draws between
  stages takes its own `Rng::new(o.seed + k)`.
- Stage names match regardless of case, and spaces, underscores and
  hyphens are the same (`--stop first_pass` stops at "First pass"). An
  unknown name is an error that lists the stages. Two stages with the same
  name are an error.

### Commands

All runs of one painting use the same `--width`, `--seed` and crop, since
checkpoints are tied to them.

```
cargo paint <name> -- --width 2400                          # → out/<name>.png
cargo paint <name> -- --width 2400 --stop <stage>           # save right after a stage
cargo paint <name> -- --width 2400 --ckpt                   # checkpoint after every stage
cargo paint <name> -- --width 2400 --resume <stage>         # start after that stage
cargo paint <name> -- --width 2400 --resume <s> --stop <s>  # the checkpoint as an image
cargo paint <name> -- --width 2400 --resume <s> --stale-ok --ckpt
cargo paint <name> -- --width 2400 --crop x0,y0,x1,y1       # → out/<name>_crop.png
cargo paint <name> -- --width 2400 --crop x0,y0,x1,y1 --margin 80 --ckpt
cargo paint <name> -- --width 2400 --no-cracks
```

`--stop` saves the canvas as it is after that stage; saving dries the wet
paint first, and no finish is applied.

### Crops

`--crop x0,y0,x1,y1` (units of the whole canvas) paints only that window,
at the run's resolution, plus a margin (default 40 units, `--margin`)
that is painted but not saved: it gives leveling, the brushes' feel of the
surface and strokes entering the window their context. The program doesn't
change: `c.frame()` is still the whole canvas; masks and forms are still
whole-canvas; strokes are planned over the whole canvas with the same
random draws; tiles that miss the window are skipped. `c.window()` is
the pixels held; in a crop, `sample`, `pixels`, `under` and the aim see
only those.

The linen and grounds laid with `prime` match a whole render exactly.
Brushwork, a brushed ground included, matches closely: outside the window
there is no canvas for a brush to feel, so the paint it lays and picks up
there is estimated, and it enters the window about as loaded as in a whole
render. The difference falls as the margin grows and is largest for long
strokes. The look-and-fill dabs see only the pixels the crop holds.

A crop's checkpoints are its own (`out/<name>_crop.<stage>.ckpt`), and a
crop resumes only from a checkpoint made with the same crop and margin.

### Checkpoints

`--ckpt` writes `out/<stem>.<stage>.ckpt` after each stage (stem
`<name>` or `<name>_crop`; spaces and slashes in the stage name become
`_`). A checkpoint holds the whole canvas state after its stage: dry
picture, surface, film, linen, size, the wet layer, stroke ids, clock and
drying state, the drawing, the hand-time ledger, plus the `keep` state,
width, seed, crop and fingerprints of the code. A resume from it is exact:
it paints bit for bit what an uninterrupted run paints.

`--resume <stage>` refuses a checkpoint whose width, seed, crop or name
differs, and refuses a **stale** one. A checkpoint is stale when any of
this changed since it was saved:

- anything in the engine (`crates/paint/src`) or in the helpers in
  `paintings/src` outside `bin` (every checkpoint goes stale);
- in the painting's file: every line up to the closing brace of that
  stage's `if o.stage(..) { .. }` block (its body and everything before
  it), and everything after the item that holds it (helper functions below
  `main`). Comments count.

Code after the block, between it and the next stage, is setup for later
stages and doesn't count. Setup that has to sit higher (a constant at the
top) can be tagged: a line ending in `// ckpt: from <stage>`, or the lines
between `// ckpt: from <stage>` and `// ckpt: end`, count only for that
stage and later ones. A tag naming no stage is an error. A tag is taken on
trust: a tagged value that an earlier stage does read leaves that stage's
checkpoint looking fresh.

`--stale-ok` uses a stale checkpoint anyway; with `--ckpt` it also rewrites
the checkpoint's fingerprints for the current code, so the next resume
needs no flag.

## 13. Viewing renders

Canvas texture makes PNGs large (a 2400 px render is several MB). View
them through `scripts/peek`, which writes a JPEG at most 1000 px on a side
(quality 95, full-resolution color), optionally cropping first.

```
scripts/peek out/<name>.png $TMPDIR/view.jpg                     # whole image
scripts/peek out/<name>.png $TMPDIR/detail.jpg 600 900 300 1200  # height width y-offset x-offset, px
```

Crop offsets are pixels of the PNG: at 2400 px wide, units × 2.4. A
`--crop` render (`out/<name>_crop.png`) holds only the window, at full
resolution, so it can be peeked whole.

## 14. What is slow

Measured at 2400 px on a 12-core machine under other load
(`Style::friedrich()`, aspect 1.4 unless noted):

| step | time |
|---|---|
| `st.prepare` (linen, two knifed grounds, brushed top ground) | 39 s |
| `broad()` over 30 % of the canvas | 7 s |
| `blend()` over the same region | 16 s |
| `body()` over 70 % of the canvas | 3 s |
| `detail()` or `hatch()` over 200 × 200 units | 0.5–0.9 s |
| `stipple` over 30 % at coverage 1.5 | 1.1 s |
| `wait`, `dry`, `glaze` | 0.05–0.2 s each |
| one `Form` body, lit | 0.1 s |
| varnish, cracks and relief | 0.5 s |
| checkpoint, whole canvas 2400 × 1500 | 274 MB, 0.1 s to write |
| `study_stipple`: whole run / resumed from its third stage / a 200 × 150-unit crop | 78 s / 6.5 s / 11 s |

- **Painting is the bristle simulation.** Its cost grows with the number
  of strokes × their length × bristles. Parallelism comes from tiles
  (square passages larger than twice any stroke's reach) that don't share
  pixels, so long strokes make big tiles and few of them run at once.
- **Swept passes** (`blend()`, any `sweep`) run band by band, and hand time
  sweeps down too: less runs in parallel.
- **The ground** is repainted on every run that doesn't resume, since
  `o.canvas(|| st.prepare(..))` runs only without `--resume`.
- **Whole-canvas work in crops.** Masks (4 bytes per pixel), forms (about
  28 bytes per pixel), scene views and stroke planning cost the same in a
  crop as in a whole render. Tabulate profiles with `f.per_column`, build
  a form once and drop it.
- **Checkpoints** at full size are hundreds of MB each, one per stage.
  They pay off on the stages after a slow one.
- Palette aims are cached; an uncached aim search is about 0.1 ms.

The fast loop for a detail: render its window with `--crop` and `--ckpt`
once, then iterate on the late stages with `--resume <stage>` and the same
crop.
