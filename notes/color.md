# Color semantics (stream 1, branch `color`)

## The problem
A paint's color used to mean "how one coat looks over white":
`Pigment::with_hiding(color, hiding)` set the over-white look to `color` and
the over-black look to `color × hiding`. For any paint that doesn't fully hide,
the masstone (the paint laid thick) is darker than that. So a mark mixed to
match a field dried darker than the field, and a thin passage dried darker than
asked. The fresh painters hit this as dark stipple specks, darkened glazes and
recipe seams in the coast sky. They worked around it by sampling the canvas by
hand (`aim()` in `paintings/fresh/fresh_coast.rs`).

## What color means now
- **`Paint::color` and `Tube::color` are the masstone.** That is the paint
  laid thick, which is also how it looks laid over paint of the same color, at
  any thickness. In KM the masstone R∞ is the fixed point of compositing
  (`pigment::tests::masstone_is_a_fixed_point`). So a mark mixed to the field
  it sits in disappears into it, body color or thin.
- **Hiding is the contrast ratio of one coat** (look over black ÷ look over
  white; `pigment::hiding_of`). The same numbers as before (0.07 glaze, 0.5
  scumble, 0.92 body) now convert to one KM scattering value S per coat
  (`pigment::scatter_for`). S is flat across channels; absorption is
  `K = S·(1−R∞)²/(2R∞)` per channel, so it carries the hue
  (`Pigment::masstone`).
- **Mixing is two-constant KM with Mixbox for hue.** The masstone mixes in
  Mixbox latent space: in the wet layer by volume, on the palette by volume ×
  tinting strength (as before). S mixes linearly by volume (true KM). K follows
  from the mixed masstone and S. Before, hiding was mixed linearly and K/S was
  rebuilt from an over-white color. That under-weighted white: in real paint a
  little white makes a glaze turbid fast because its S is large, and linear
  S-mixing captures that. The wet layer's `Prop[0]` is now S, not hiding.
- **`Paint` carries S.** `Paint { color, scatter, stiff }`: the masstone and
  the KM scattering per coat. Name paint by hiding with
  `Paint::new(color, hiding, stiff)` or `p.with_hiding(h)`; `Paint::km`
  takes S directly; `p.hiding()` derives the contrast ratio for reporting.
  Hiding saturates near 1 (`scatter_for` caps it at 0.9995), so a paint that
  stored only hiding lost the S of any strongly scattering mixture: an
  opaque black + white pile mixed to 0.1 has S ≈ 20 and came back as S ≈ 1,
  and a 0.1-coat film of it over white read 0.44 instead of 0.10. Now the
  paint the brush loads is the paint the palette scored
  (`palette::canvas_tests::mixture_to_paint_preserves_scattering`).
- **Medium thins the pigment.** It multiplies K and S by (1 − medium) and
  leaves the masstone alone (`Mixture::paint`). The old rule was
  `hiding × k^0.6`.
- **Paints named by their tint** (glazes, as in "tints white to X") use
  `Paint::tint(tint, hiding, stiff)`. `Paint::glaze(tint)` calls it. The
  `Pigment::{transparent, semi, opaque, varnish, with_hiding}` constructors
  keep the over-white meaning for `Canvas::glaze` films. Grounds
  (`Canvas::prime`) now use masstone semantics like all other paint.

## Aim at the result
A painter judges a pile on the canvas: "what will this look like here, over
what is already painted, laid as thick as I lay it?" That question now has an
answer at three levels:

```rust
// what is on the canvas at a point (dry picture + wet paint, disc of radius r)
let under = c.under(x, y, r);
// preview: how a paint looks laid `coats` thick over it
let look = paint.over(under, coats);

// per mark (stipple, dabs, hand-made strokes): the pile from the palette
// that, thinned with `medium` and laid `coats` thick here, looks `want`
let p: Paint = c.aim(&pal, want, (x, y), r, medium, coats);
let m: Mixture = pal.aim(want, under, medium, coats);   // same, with the recipe
let p = pal.paint_for(want, under, medium, coats);       // same, as Paint
// no palette: solve the masstone for a fixed hiding
let p = Paint::aimed(want, under, coats, hiding, stiff);
// name paint by hiding, override on a mixed paint
let p = pal.paint(col, 0.0).with_hiding(0.97).with_stiff(1.0);

// handlings: palette-mixed handlings aim by default
st.broad().color(sky)                  // Aim::Laid: expects the thickness it lays
st.broad().color(sky).aim(0.8)         // expect 0.8 coats
st.body().color(rock).by_masstone()    // color = masstone, don't look
st.glaze(0.9).color(umber)             // glazes mix by masstone (depth is load_at's job)
st.broad().palette(&family)            // swap in a paint family, keep the medium
```

- `Aim::Laid` (the default with a palette) expects `Handling::laid_coats()`
  × `load_at` coats: one stroke's film per unit of load (1.3 for filberts,
  2.2 for rounds and riggers) times how many strokes overlap at a point that
  gets paint, `c / (1 − e^−c)` for coverage `c`. The constants come from
  `handling::tests::probe_laid_by_coverage` (median film where paint
  landed, coverage 0.2–4, load 0.3 and 0.7). The old estimate,
  `1.1 × coverage × load` floored at 0.3, expected 0.3 coats from sparse
  marks that lay 1–1.7, so aimed piles overshot (see "Aim over contrasting
  paint" below). Fixed-paint handlings (`.paint(hiding, stiff)`, e.g. the
  brushed ground) stay `Aim::Masstone` unless you call `.aim(coats)`.
- `Style::glaze` is `by_masstone()`. When a veil was aimed, aim compensated
  away the depth the painter varies with `load_at`, and the moonrise mist
  covered its spruces. Add `.aim(coats)` if you want a glaze aimed at a look.
- **Robust judging.** A pile is scored over 0.5×, 1×, 2× and 4× the
  expected thickness (weights 0.2, 0.45, 0.25, 0.1; `AIM_SPREAD`), because a
  stroke lays paint thin at its edges and thick where it starts. The 4× point
  stands for the pile's own color where it lands thick. Two more terms: the
  thin edge (0.25×) should lie on the way from the underlayer to the look
  wanted (`AIM_THIN`: no red rim, no milky veil), and the pile's masstone
  pays 0.12 per unit of a/b distance from the look wanted (`AIM_FAMILY`:
  keep the pile in the family of the color asked for).
- **Out-of-reach targets** come out as the nearest the painter can get, and
  `Mixture::error` reports the miss at the expected thickness. A transparent
  glaze cannot bring a dark up to a pale target. A lead-white or ochre veil
  *does* lift a dark: those paints scatter, and a 90%-medium lead white is a
  hiding ≈ 0.3 milky scumble. That is real paint behavior, not a bug.
- **Continuity.** Each tube in a pile has a small cost that ramps in
  continuously (`PARSIMONY` 0.004 OKLab over the first 10% of the pile). This
  stops a touch of black flicking in and out between neighboring piles.
  Refinement can now add a tube, which removes a search artifact where a
  single-tube start could never gain a second paint. For long smooth passages,
  set out a family, as a painter does:
  `let sky_pal = st.palette.only(&["lead white", "smalt", "yellow ochre"]);`.
- **Cost.** An uncached aim search takes ~0.1 ms. It scans a precomputed grid
  of all single tubes, pairs and triples at 1/12 steps (built once per
  palette), takes the 24 best at the expected thickness, scores them over the
  spread, then refines. Results are cached by quantized (want, under, coats,
  medium). The search runs from the quantization cell's center, so results
  don't depend on call order and threads can share a palette.
  `Palette::cache_sizes()` reports counts: the study sheet does 638 aimed
  searches in total (≈ 64 ms at 0.1 ms each). `friedrich_moonrise_valley` took 30.0 s CPU vs
  28.0 s before (user time; wall time is meaningless under the night's load
  average of ~190).

### For the stipple stream
Call `c.aim(&pal, want, (x, y), r, medium, coats)` per dot, with `r` about
the dot radius and `coats` about 0.5–1 for a dot (the spread covers 0.5×–2×).
To stipple a sky lighter by a touch, aim at the sampled look plus a bit of
OKLab L:

```rust
let mut l = to_oklab(c.under(x, y, r)); l[0] += 0.05;
let p = c.aim(&pal, from_oklab(l), (x, y), r, 0.6, 0.6);
```

`under` includes wet paint, so dots over a wet passage are judged against it.
`Canvas::aim` takes `&self` and the palette caches behind a `Mutex`, so it is
safe to call when planning marks in parallel (results are order-independent).

## Aim over contrasting paint (round 3, branch `fixes-paint`)
Amnesia round 2 (coast #1, #2, #13, #15; winter #14): thin light marks over
cool darks dried orange or salmon; a fleck of bare ground under a stroke
skewed its pile; thin edges showed their strongest tube; lead-white darks
went milky. Four changes:
- **The thickness model matches what marks lay** (`Handling::laid_coats`,
  above). This was the largest error. Sparse marks were aimed as 0.3-coat
  films and then laid 1–1.7 coats, so the pile over-compensated for the
  dark.
- **Judged along the stroke, robustly.** `stroke_under` samples nine discs
  evenly along the path, weights them toward the start (where a loaded
  brush lays most) and takes a weighted median per OKLab channel. The old
  one averaged a few points in linear light, where one light fleck among
  dark samples pulls the mean far toward it. `Canvas::aim` (hand marks) now
  uses `Canvas::judge_under(x, y, r)`: the median of nine sub-discs across
  the mark. `Canvas::under` is still the plain mean.
- **Judged thick and at the thin edge** (`AIM_SPREAD`, `AIM_THIN`, above).
- **Kept in the family of the color wanted** (`AIM_FAMILY`). Example from
  `palette::tests::probe_contrast_aims`: a light touch `#9a8f80` over dark
  sand `#3a3128` at 0.3 coats was lead white + red earth, masstone a +0.053
  (salmon); now it is lead white + raw umber, a +0.011.

Tests: `palette::tests::contrasting_aims_stay_in_family` (three light-over-dark
cases at 0.3–1 coats: masstone and 4× look within 0.025 a/b of the target)
and `handling::tests::light_marks_over_a_dark_stay_in_hue` (sparse detail and
body marks over a dark sand lay-in with ground flecks: mean L miss 0.008–0.010,
down from 0.049–0.050 under the old estimate; a/b miss ≤ 0.0054).

Relative colors: `Handling::color_over` and `Stipple::color_over` hand the
closure what is under the stroke. See `notes/strokes.md` and `notes/fixes_paint.md`.

## Tests (crates/paint)
- `palette::canvas_tests::matched_marks_disappear` (a): dabs aimed at the
  field under them, over a brushed sky. Worst mean ΔE of the mark region
  before vs after is 0.0037 (body, 0.15 medium), 0.0027 (semi, 0.55) and
  0.0016 (thin, 0.8). The old reading of the same piles gives 0.045, 0.046
  and 0.043. (An OKLab JND is about 0.02.) The test asserts < 0.008, and
  asserts the old reading is > 3× worse for the semi and thin paints.
- `palette::tests::smooth_targets_make_no_seams` (b): a smalt → lead-white
  ramp in 81 steps over a warm gradient underlayer. The target moves at most
  0.0044 per step. Worst neighbor step in the look at 0.3, 1 and 2.5 coats:
  masstone mixing 0.023, 0.034 and 0.022; aimed, full palette 0.0097, 0.0087
  and 0.016; aimed family (lead white, smalt, ochre) 0.0060, 0.0086 and
  0.011.
- `palette::canvas_tests::glazes_stay_glazes` (c): a dark umber glaze (0.9
  medium) brushed over light and dark lowers the light passage's L by 0.15
  and moves the dark by +0.027. It deepens with thickness. A pale
  hiding-0.07 glaze aimed at a dark gets less than halfway. Body color
  reaches the same target (error < 0.05).
- `wet::tests::aimed_reaches_targets_made_by_the_same_model`: `Paint::aimed`
  (and `Paint::tint`) bisect on the masstone's luminance, whose residual
  `luminance(m(l)) − l` changes sign across 0.002–0.995, so the solve always
  converges. The old fixed-point iteration could stop far off. Targets made
  by 4 hidings × 4 underlayers × 5 thicknesses (0.1–2.5 coats) × 5 paints
  come back within 0.002 (it missed a 0.1-coat, hiding 0.92 target over
  black by 0.26).
- `pigment::tests::{masstone_is_a_fixed_point, scatter_inverts_hiding}`,
  `palette::tests::{aim_hits_reachable_targets, tint_round_trips_over_white}`.
- Golden re-recorded (debug profile): paint color is now masstone and
  palette-mixed passages are aimed.

## Evidence
`cargo paint study_color` → `out/study_color.png`
(`paintings/src/bin/study_color.rs`; the header comment maps the panels).
- Top band, sky with 900 dabs mixed to match it. In the x 0–500 half (the old
  reading) the dabs dry as dark specks, the "digital rain". In x 500–750
  (`Canvas::aim`) they vanish. In x 750–1000 (aimed 0.06 L lighter) they read
  as a deliberate pale stipple. Best seen at `--width 1600`, cropped.
- Middle band, a glaze wedge of 0–5 coats (umber + black, 0.9 medium, an
  even `Canvas::glaze` film of the palette paint's own pigment). Over light,
  L goes from 0.857 to 0.335; over dark, from 0.284 to 0.253.
- Bottom band, smalt → lead-white skies over the ground: `by_masstone`,
  aimed (full palette) and aimed family. Mean ΔE from the target by height
  (printed by the study): masstone up to 0.054 (top), aimed ≤ 0.030,
  family ≤ 0.034. No seams in any of them at this coverage.
- `friedrich_moonrise_valley` and `friedrich_monk2` build unchanged and
  render lighter in the semi-transparent passages (mist, sky veils). The old
  reading darkened any paint that doesn't fully hide (test (a) measures
  this), so these passages now sit nearer the colors their programs ask
  for. monk2's later `Canvas::glaze` films were tuned by eye against the old
  renders, so its upper sky may want retuning. Before/after previews are in
  `~/tmp/color-05d324b4/{base,aim}_{moonrise,monk2}.jpg`.

## Shared files touched (small, local)
- `bristle.rs` `Held::load`: loads S (`paint.scatter()`) instead of hiding
  into the bristle reservoir (one line).
- `canvas.rs` `prime`: `Pigment::masstone_hiding` instead of `with_hiding`
  (one line).
- `style.rs` `glaze()`: `.by_masstone()` plus doc (two lines).
- `lib.rs`: exports `Aim`.
- `handling.rs`: the `aim` field, the builders (`aim`, `aim_laid`,
  `by_masstone`, `palette`), `finish_plan` (now takes `&Canvas`; its two call
  sites pass `self`) and `stroke_under`. Stroke planning is untouched.

## Known issues and next steps
- The full palette can still trade recipes near out-of-reach targets (the
  0.016 step at 2.5 coats). It shows up at the light end, where thinned lead
  white can't beat a warm ground, and at the deep end, where aimed skies reach
  for bone black. A family (`only`) fixes it. A stronger fix is a per-passage
  pile the painter modifies: a `Handling` hook that aims only the proportions
  of a fixed recipe.
- Mixing is inconsistent between palette and wet layer. The palette weights
  Mixbox by tinting strength; the wet layer (brush ↔ canvas) weights by
  volume. A strength per wet pixel would make them agree.
- `Aim::Laid`'s thickness estimate is a constant per brush kind (see above).
  The glaze tool lays about half of the filbert figure; glazes mix by
  masstone, so it only matters with `.aim(coats)`. Round brushes lay more
  per load as coverage rises (2.0 → 3.4). If the pointed-tip brush changes
  deposit, rerun `probe_laid_by_coverage`.
- `Canvas::under` samples before a pass. Strokes in a pass don't see each
  other's wet paint while being planned; the thickness estimate covers the
  stacking.
- The palette has no truly transparent glazing pigment (a madder or lake).
  Umber + black stands in.
- `paintings/fresh` and `paintings/archive` are not built and still use
  `Paint { hiding, .. }` literals. Use `Paint::new` or `.with_hiding` if
  they are revived.
- The fresh painters' archived programs (`paintings/fresh`) use
  `Pigment::with_hiding(pt.color, pt.hiding)` to preview a paint. That is
  now `pt.over(under, coats)`; their hand-rolled `aim()` is `Canvas::aim`.
