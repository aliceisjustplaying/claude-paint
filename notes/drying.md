# Time and drying (branch `drying`)

The painting now has a clock. `c.wait(minutes)` lets time pass, and each
pixel's wet paint ages according to its own paint and history. Brushes
behave differently on open, setting, tacky and touch-dry paint. `c.dry()` is
still there. It now means "wait until everything is touch-dry," and a
painting that never calls `wait` renders bit for bit as before (the golden
scene is unchanged).

This branch also fixes amnesia friction item 2: near-zero glaze amounts
that drew hard edges and rectangles.

## API

```rust
use paint::drying::drier;
use paint::{Paint, Stage};

let light = Paint::body(hex("#e2cfa6")).with_drying(drier::LEAD_WHITE); // dries fast
let black = Paint::body(hex("#303338")).with_drying(drier::BONE_BLACK); // dries slow
// ... paint the sky ...
c.wait(0.0);            // nothing happens: keep working wet into wet
c.wait(30.0);           // half an hour: still open, a little stiffer
c.wait(180.0);          // come back after lunch: lead-white passages are tacky
c.wait(24.0 * 60.0);    // next day: they are touch-dry, new paint sits on top
c.dry();                // wait until every film is touch-dry (as before)

c.clock();              // minutes since the canvas was made (f64)
c.drying_at(x, y);      // Stage::Open | Setting | Tacky | Dry
```

- `Paint::drying` is a new field: the drying rate relative to average paint
  (1.0 by default in every constructor). `Paint::with_drying(rate)` sets it.
  `paint::drying::drier` holds the rates for the period pigments.
- `Canvas::wait(minutes: f32)` is the call the Lua easel's `wait(minutes)`
  should make. Negative waits are clamped to 0, and an infinite wait is
  `dry()`.
- `c.clock()` starts weeks in, because `Style::prepare` dries the ground.
  Print times relative to where you start.

## The model

Oil paint dries by oxidation. The oil takes up oxygen and cross-links,
going from a liquid to a gel to a solid film. Each pixel's open (wet) film
carries a cure value (`drying::Px::cure`): 0 is fresh, `GEL` = 0.15 is the
gel point and 1 is touch-dry.

**Rate.** Cure per minute = `drying / (TOUCH_DRY_MIN × thick × fat)`:
- `TOUCH_DRY_MIN` = 24 h for one lean 25 µm coat of average paint.
- `thick` = (film thickness in coats)^0.7, at least 0.5. Thick films dry
  slower.
- `fat` = 1 + 0.6 × (1 − stiffness). Medium-rich (fat) paint dries slower
  than stiff tube paint.
- `drying` is the pigment's rate, mixed by volume with the paint like
  scattering and stiffness. It rides along as the third component of
  `wet::Prop`, so brushes carry it and mix it.

**Stages** (`drying.rs`):

| stage | cure | what brushes do |
|---|---|---|
| open | < 0.075 | as before: blends wet into wet, lifts, ploughs, levels |
| setting | 0.075–0.15 | viscosity climbs: lift and plough scale with fluidity `1 − smoothstep(0, GEL, cure)`, and the surface starts to get sticky |
| tacky | 0.15 to 1 | the film has set. It no longer flows, mixes or lifts. It grabs: brushes empty up to 3× faster, in stick-slip patches |
| touch-dry | ≥ 1 | new paint sits on top without mixing (the old dry-paint behavior) |

**Leveling happens only while the film is fluid.** A film levels for
`SET_TIME` (900 s) after it is last worked, which is thixotropic recovery.
If it is worked while setting, it levels only for
`SET_TIME × fluid(cure)`. So a mark made in setting paint keeps its
bristle ridges.

**New wet paint over a set layer.** When a film reaches the gel point during
a `wait`, it levels, as it would have, and bakes into the dry picture
(Kubelka–Munk color, height, film). It can't flow or mix any more, so
nothing is lost. Its tack lives on per pixel (`Px::sub`, the set film's
cure, and `Px::srate`, its rate) until it is touch-dry. A pixel can
therefore hold new wet paint over a tacky or dry layer. The new paint never
mixes with the layer under it, and the brush feels the tack.

**Fresh paint into older paint.** At each `wait`, pixels touched by a stroke
since the last wait (stroke ids above a watermark) dilute their cure by the
fresh volume, `cure × seen / vol`, and level again at the fluidity they have
now.

**Brush hook** (`bristle.rs` `exchange`, small and additive). `Surf`
carries a pointer to the drying state (null until the first `wait`). Three
things change:
- The per-stroke lift floor and the per-bristle lift scale with fluidity.
- The plough scales with fluidity.
- The deposit over a tacky surface goes up by `1 + 2 × tack`, times a
  stick-slip patch factor (`drying::stick`).

With a null pointer every factor is exactly 1.0, so the output is
unchanged.

**Cost.** The drying state (5 floats per pixel) is allocated on the first
`wait`, so paintings that don't wait pay nothing. A wait is a pass over the
dirty box plus, when films set, one `settle` of that box (as `dry()` does).

## Parameters and sources

Tags: [S] means from a search summary of the source (I did not read the
full text); [E] means my own estimate.

| parameter | value | basis |
|---|---|---|
| relative pigment rates | lead white 2, umber 2.4, chrome yellow 1.8, Prussian blue 1.8, smalt 1.6, cobalt 1.4, sienna 1.2, red earth 1, ochre 0.8, ultramarine 0.8, vermilion 0.4, bone black 0.4, lamp black 0.35, madder 0.3, zinc white 0.35 | Ranking from Mayer, *The Artist's Handbook*, pp. 301–304: fast are chrome yellow, Naples yellow, raw and burnt umber, lead white and Prussian blue; slow to very slow are vermilion, lamp, vine and ivory black and natural madder [S] (https://pdfcoffee.com/the-artistx27s-handbook-7-pdf-free.html). The number for each class is [E]. Smalt as a drier: cobalt is a siccative; Mayer ranks cobalt blue fast to medium [S] |
| `TOUCH_DRY_MIN` | 24 h for 25 µm of average paint | Thin-film touch-dry runs 1–2 days for raw umber, 1–3 for lead white, 2–5 for bone black and 7–14+ for alizarin [S]. Raw and burnt umber were touch-dry after about 24 h in controlled tests [S] (https://www.nature.com/articles/s41529-024-00472-8). One coat here (25 µm) is thinner than typical test films, hence 1 day [E] |
| thickness exponent | 0.7 | [E] Skinning is reaction-limited in thin films and increasingly oxygen-limited in thick ones. A 250 µm titanium film is touch-dry in 4–6 days (Golden, "Weighing In on the Drying of Oils", https://justpaint.org/weighing-in-on-the-drying-of-oils/, cited in notes/research/oil_paint_physics.md §3) |
| fat factor | 1 + 0.6(1 − stiff) | [E] More oil per pigment means slower drying |
| `GEL` | 0.15 of touch-dry | [E] Chosen so a lead-white-rich coat is tacky after about 2 h and an average one after about 3.6 h. Real open time varies from hours to a day |
| grab, stick-slip | up to 3× faster deposit, patch scale 3 bristle radii | [E] |
| `SET_TIME` | 900 s | unchanged (surface.rs: thixotropic recovery; oil_paint_physics.md "Implications" A) |

The time constants are the least certain part. They are calibrated so that
thin Friedrich-like coats of lead-white mixtures behave the way the user
described (open at 30 min, tacky at 3 h, touch-dry the next day). The
pigment ranking has the best basis. The absolute times should be treated as
adjustable.

## Glaze near-zero fix (amnesia friction 2)

**The mechanism** (reproduced, not guessed). A glaze of
`0.35·exp(−(d/38)²)` requests thicknesses of 1e-12 down to 1e-43 coats far
from its center. In `surface::settle`, the local conservation ratio
`laid / kept` divides one blur of denormal residue by another. That
overflows to infinity, and `0 × ∞` gives NaN. A NaN in the total makes
`so > 0.0` false, so the global rescale is silently skipped. Then
`Pigment::over(p, NaN)` paints the glaze's **full masstone**, because
`(NaN).min(40.0)` returns 40 in `layer`. On a 400 px Friedrich canvas the
ring 200–400 units out changed by 0.25 (about 64 of 255 levels), ending
exactly where `exp` underflows. NaN heights explain the later "black dots"
and thrown strokes the coast painter saw.

**The fix:**
- `settle` zeroes deposits below `ADD_EPS_UM` = 1e-4 µm, which is
  numerically nothing and below f32 resolution on ~100 µm heights. It also
  accepts the local ratio only when it is finite, and it asserts (in debug
  builds) and guards the global total.
- `Canvas::glaze` gives films a physical minimum:
  `canvas::MIN_FILM_UM` = 1 µm, about the size of glazing-pigment
  particles. Below it, the film fades out smoothly (C¹, zero at 0.5 µm), so
  the cutoff itself draws no edge. Blurred-mask residue and long tails end
  softly where the film gives out. Glazes of about 0.13 coats or more (1 µm of
  film) are unchanged bit for bit, which covers every glaze in the golden
  scene.
- Regression tests (`canvas::tests`):
  - `glaze_long_falloff_has_no_edge`: the winter moon glow.
  - `glaze_through_blurred_mask_leaves_no_rectangle`: the coast ribbons.
  - `formed_film_is_smooth_and_monotone`.

  They also assert that every pixel and height is finite.

## Evidence

`cargo paint study_time` (1000 px, about 14 s; `--width 2000`, about 17 s):
- `notes/drying/study_time.jpg`: the whole sheet.
- `notes/drying/study_time_blends_2000.jpg`: the same blend (dark blue
  brought up into a warm lead-white field, then worked across the join with
  a clean flat and a badger) started at 0 min, 30 min, 3 h and the next
  day. At 0 and 30 min there is a soft mixed band of light blue. At 3 h the
  blue drags into the tacky light field in broken, crisper streaks and
  nothing of the light comes up. The next day the blue strokes sit on top,
  crisp and unmixed.
- `notes/drying/study_time_scumble_glaze_2000.jpg`: a light scumble over
  umber that is tacky (3 h) and touch-dry (next day). Over tacky paint it
  is heavy where the stroke starts, then breaks up and gives out early as
  the tack strips the brush. Over dry paint it skims evenly along the whole
  stroke. Next to them is a pale glaze with a long Gaussian falloff through
  a blurred mask: no edge, no rectangle.

The program prints the stages it reaches: at 3 h, light Tacky, umber Tacky,
slate (bone black) Open; the next day, light Dry, umber Dry, slate Tacky.

Tests (`cargo test -p paint`: 75 pass, golden unchanged). `drying::tests`
covers:
- stages by clock and pigment (lead white tacky at 3 h and dry the next
  day; bone black still wet)
- thick and fat films dry slower
- waiting it out in steps bakes bit for bit what `dry()` bakes
- a clean brush lifts less from setting paint and nothing from set paint
- tack lays more paint early in a stroke
- a checkpoint written mid-drying resumes exactly
- 1 vs. 4 threads give identical results

## For the integrator

- **`Paint` gained a public field (`drying`).** Struct literals must add it.
  `paintings/src/figures.rs::mix` is updated. Branches that build `Paint {
  .. }` literally need `drying: 1.0` (or a mix).
- **`wet::Prop` is `[f32; 3]`.** In bristle.rs: the initial bristle
  `hide`, `load` passing `paint.drying`, and loops in `Surf::add` and the
  `got_h` pickup sums. Plus the `dry` pointer in `Surf` and about 20 lines
  in `exchange`. All additive, near the `tip` stream's area but not in its
  geometry code.
- **Checkpoint format v2 (`PAINTCK2`; `PAINTCK3` after merging with cracks, which appends the ground thickness).** It adds the drying rate in the
  wet props, the per-pixel stroke ids (`wait` reads them), the clock, the
  watermark, the tacky box and the drying state when allocated. v1 files
  are refused with a clear error; re-run with `--ckpt`.
- `dry()` moved from wet.rs to drying.rs. `surface::settle` gained
  `settle_for` (per-pixel leveling time).
- Crops. Everything is per pixel except the leveling `settle`, which
  already differs slightly in a crop, so crop behavior is as before.
  Parallel tiles: `wait` never runs during strokes, and brushes only read
  the drying state.

## Open issues

- **Wet-in-wet within a pixel is still one mixture.** Setting (not yet
  gelled) paint still mixes with new paint laid into it; only its lift and
  plough fall. A two-film wet pixel would need a second latent, which costs
  memory.
- **Through-drying and fat over lean** are not modeled after touch-dry.
  `Px` could keep a film age for `crack.rs` (drying cracks when a
  fast-drying layer sits on a slow one, bitumen-like).
- **Palette tubes don't carry drying rates yet.** `Palette` mixes give
  `drying` 1.0. Adding a rate to `Tube` and mixing it by volume is a small
  change in palette.rs (owned by other streams). Until then, set it with
  `with_drying`.
- **Tacky films can't be torn.** A brush dragged hard through tacky paint
  lifts strings of it. Here set paint never lifts.
- **Glazes over heavy impasto leave the ridge tops bare** (pre-existing
  `settle` pooling). A thin fluid glaze levels below tall stroke ridges and
  `(lev − old).max(0)` gives those pixels no film, which shows as crisp
  dark dashes. Real glazes leave a wetting film on peaks. A floor of about
  `MIN_FILM_UM` on the peaks, taken from the pooled volume, would fix it.
- `Canvas::glaze` still dries everything first (it is a finished glaze,
  not a wet film). A wet glaze that ages would be a `Handling` with
  `st.glaze()` plus `wait`.
- Stroke ids are compared against a watermark without wraparound handling
  (4 billion strokes).

## Known issue: drying stage is resolution-dependent at low widths
Found during integration (round 3): `study_time` at 400px reports "light
Setting, slate Setting" at 3 h where 1000px and 2000px both report "light
Tacky, slate Open". Pre-existing in the drying merge (checked at cbe0cd8).
Likely cause: a pixel's film thickness (which sets its drying rate) is the
average over the pixel, so at coarse widths thin and thick paint blend into
one intermediate thickness. The study now reports the difference instead of
asserting. For the review: drying rates should be computed from thickness at
a fixed physical scale, not per pixel.

**Fixed** (branch `surface`, notes/surface.md §3): rates come from the film
thickness averaged over `drying::FILM_MM` (1.25 mm), so `study_time` gives
the same stages at 400, 1000 and 2000px and asserts them again. What
remains is the blunt deposit's own resolution dependence (a hog field is
~23% thicker at 400px than at 1200px).

## Round 3 review fixes (branch `fix3-physics`)

**Splitting a wait no longer changes the drying** (review3 physics #1).
`film_thickness` averages only the paint that is still wet, and each `bake`
removes the films that gelled. So every later `wait` judged the remaining
paint's thickness afresh, and checking back often changed the physics. A
striped film of thin fast stripes (0.2 coats, drying 2) and thick slow ones
(4 coats, drying 0.4) reached Dry after `wait(7000)` but stayed Tacky after
70 × `wait(100)`: the thick stripe lost its thin neighbors from its average
and dried slower (set-film cure 1 vs 0.747).

Now each open film's neighborhood thickness is judged when the film is
worked (`absorb`, the start of the `wait` after a stroke laid or touched it)
and stored in `drying::Px::th`. Waits, gel events (the set film's `srate`)
and `dry()` use the stored value, and it changes only when a brush works
that pixel again. Fresh paint laid next to an older film doesn't change
the older film's rate; the fresh paint's own thickness is judged over
everything wet around it, the older film included. Checkpoints are now
`PAINTCK5`, which stores `th`, so `PAINTCK4` files are refused.

Regressions in `drying.rs`:
- `splitting_a_wait_changes_nothing`: the striped film, one wait vs 70.
  Every pixel reaches the same stage, and the slow stripe's cure matches
  within 1e-3.
- `splitting_a_wait_changes_nothing_under_the_brush`: two filbert strokes
  (lead white, drying 2, and slate, drying 0.4) on linen, `wait(3000)` vs
  100 × `wait(30)`. Every pixel reaches the same stage, the time to dry
  out matches within 0.1% and the picture after `dry()` matches within
  1e-3. Before the fix, 149 pixels differed in stage.

What remains: films that gel at different times inside one long wait still
bake together at its end, while split waits bake them in separate groups.
Leveling and pinhole closing see different neighbors, so the pictures
differ by rounding-level amounts (under 1e-3 reflectance in the test above).
Stages and times match.

Evidence: `study_time` barely changes. Its largest change is at the tops
of the rigger hooks in the tacky panel, and most of it comes from the tip
fixes (notes/tip.md). `notes/drying/r3fix_hooks_before_after.jpg` shows
them at 5× (before above, after below).
