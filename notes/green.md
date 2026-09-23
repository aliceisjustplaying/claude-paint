# Stream: green (sourced greens, foliage, meadows)

The user noticed that last night's paintings all share one palette and one
mood, while Friedrich "made many paintings that were very green." The
palette had no green tube, and `growth` grew only bare skeletons. This
stream adds the greens he is documented to have used, leaves on grown
trees and a ground-cover primitive for meadows, plus a study that uses them.

## 1. Research (notes/research/friedrich_materials.md §9)

The key new source is the Dresden technical study of 14 paintings, Mäder
et al., *METALLA* Sonderheft 13 (2025), p.102 [MÄD]. I checked the quotes
against the text myself:
- Given how sparse his palette was, his greens are an "üppige Vielfalt"
  (a lush variety).
- The true green pigments were "kupferhaltige Pigmente sowie Grüne Erde"
  (copper greens and green earth).
- He mixed greens from blue and yellow, "sehr häufig" (very often) with
  Prussian blue and also with smalt and cobalt. The yellows were yellow
  ochre, Naples yellow and chrome yellow.
- Green passages often stack "bis zu vier" (up to four) layers of
  different greens.
- Rinmann's green is rare: three paintings, c.1819–23.
- One painting (1808) has copper plus arsenic, a degraded copper-arsenite
  green. My inference: it must be Scheele's green, since Schweinfurt green
  dates from 1814.
- The London *Winter Landscape* grass has no green pigment at all: smalt,
  Naples yellow, bone black, ochre and possibly Prussian blue [NG p.56].
  §4 had left out the smalt; I've fixed that.

Dates come from the NGA *Artists' Pigments* volumes and Field 1835.
Viridian, chromium oxide and emerald green are too late or too doubtful
for him. No green analyses were found for *Hill and Ploughed Field*,
*Meadows near Greifswald*, *Morning in the Riesengebirge* or *Chalk
Cliffs*; that is a gap, stated in §9.

## 2. Palette (`crates/paint/src/palette.rs`)

New tubes, with masstone, hiding, stiffness and strength as documented
approximations (sources in the doc comments):

| tube | masstone | hiding | stiff | strength | basis |
|---|---|---|---|---|---|
| Prussian blue | #172440 | 0.35 | 0.45 | 3.0 | transparent, very strong [AP3 pp.196–197]; strength tempered (A) |
| green earth | #3a4843 | 0.2 | 0.35 | 0.3 | translucent, weak, little body [AP1 p.146; FIELD p.129]; Munsell 7.5G/2.9/1.5 converted |
| Rinmann's green | #5f8f76 | 0.35 | 0.5 | 0.4 | semi-transparent, weak (A for the numbers) |
| copper green | #3f7f6a | 0.25 | 0.4 | 1.0 | "poor hiding power in oil" [AP2 p.132]; numbers A |

API:
```rust
Palette::friedrich_early_greens()   // early + Prussian blue + green earth
Palette::friedrich_1820_greens()    // 1820 + Prussian blue + green earth + Rinmann's green
Palette::friedrich_1820().with(vec![Palette::copper_green()])  // a rare tube for one passage
Palette::green_tubes()              // the two green-making tubes, to add anywhere
let st = Style { palette: Palette::friedrich_1820_greens(), ..Style::friedrich() };
let sky_pal = st.palette.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth"]);
st.broad().mixed(&sky_pal, 0.45)    // paint the sky from its own family
```

**The base palettes are unchanged, on purpose.** I first added the greens
to `friedrich_early`/`friedrich_1820`. With them, the aimed search picked
the very strong Prussian blue for the smalt/cobalt sky ramp. The miss
halved (0.049 → 0.023 OKLab), but recipe switches made seams at thin coats,
so `palette::tests::smooth_targets_make_no_seams` failed (0.028 against a
0.018 limit). It would also have changed every existing sky, and his skies
are smalt or cobalt [ALF; NPJ25]. So the greens live in separate palettes,
and existing paintings and the golden are unchanged. The golden passed
without being re-recorded.

Finding: for muted summer greens, the base palette already mixes well
(cobalt blue + chrome yellow + bone black, ΔE ≈ 0.003–0.006). The greens
palette adds green earth to those piles (for example "bone black 0.08 +
chrome yellow 0.17 + green earth 0.75"). Prussian blue seldom wins for
these targets (test `green_tests::greens_are_mixed_closer`). The aim code
is untouched (it belongs to fixes-paint).

## 3. Geometry (`crates/paint/src/growth.rs`)

- **`Habit::leaf: Leafing`**: how a species carries leaves, as data.
  Presets: `oak`, `birch`, `beech`, `alder`, `willow`, `spruce`, `none`.
  Fields:
  - `years`: wood this young carries leaves
  - `clump`: clump radius / tree height
  - `spacing`
  - `squash`: flat beech sprays < 1 < hanging birch/willow
  - `droop`
  - `fill`: how much sky shows through a clump
  - `ragged`: lobed outline
  - `bare`: twigs with no leaves, the sky holes
  - `tip`: leaves crowd toward shoot ends
  - `inner`/`inner_w`: short leafy shoots along older thin wood, so the
    crown's shell fills in
  - `spray`: extra clumps around each live tip for the real twigs the
    model does not grow
- **New habits:** `Habit::beech()` (distichous, level sprays, shade
  tolerant), `alder()` (a clear stem through a narrow oval crown) and
  `willow()` (steep rising limbs, arching shoots, a leaning bole).
- **`Limb::age`**: the age in years at each point, so leaves go on young wood.
- **`Skeleton::foliage(sun, seed) -> Foliage`** (or `foliage_with(&Leafing, ..)`):
  - `clumps: Vec<Clump>`, where each `Clump` has `at`, `z`, `r`, `squash`,
    `tilt`, `fill`, `lit`, `shade`, `limb` and `mass` (its main limb: the
    masses the eye groups)
  - live wood only: a dead oak's dead limbs stay bare
  - `lit` is the clump's facing on the crown's envelope times the shadow
    cast by the clumps between it and the sun
  - methods:
    - `mask(frame)`: leaves = 1, with sky holes; a leaf-sized noise breaks
      every partly covered place into leaves and sky
    - `lit(frame)`: front clump over back, each slightly rounded; a clump
      covers what is behind it by its `fill` (the same share as in `mask`),
      so an airy clump in front lets the light of the leaves behind show
      through instead of overwriting it (tested:
      `transparent_foliage_keeps_the_light_behind`)
    - `envelope(frame, reach)` and `gaps(frame, reach)`: the holes inside
      the crown
    - `back_to_front()`, `bounds()`, `grain()`
- **`Sward { horizon, near, height, spacing, thin, smallest, blades, fan, curl, flowers, kinds, patch, patch_size, wind: Wind }`**:
  - `grow(&region_mask, seed) -> Vec<Tuft>` scatters tufts with a minimum
    distance, so there are no rows. They crowd in lush patches, lean with
    `Wind { lean, gust, period, seed }` and shrink toward the horizon.
  - Spacing grows as `scale^-thin`, so there are fewer marks per area far
    away. Tufts shorter than `smallest` are dropped: that's where the
    painter paints tone instead.
  - Each `Tuft` has `at`, `scale`, `height`, `lean`, `lush`, `blades`
    (quadratic curves foot → control → tip) and `flower: Option<Flower { at, r, kind }>`.
    Tufts come far first.
- Tests: `foliage_on_live_young_wood` covers every habit. It checks that
  clumps are only on live wood, that the sunward side is lit more, that
  sky shows through the crown (2–60%), that lit ≤ mask and that winter and
  dead wood are bare. `sward_recedes` checks far-first order, that far
  tufts are shorter and sparser, the mean lean against the wind, flowers
  present and no marks below `smallest`.

```rust
let oak = Habit::oak().grow((690.0, 570.0), 360.0, seed);
let leaves = oak.foliage((-0.55, -0.75, 0.35), seed);   // toward the sun
let mask = leaves.mask(c.frame());
let light = leaves.lit(c.frame());
for cl in leaves.back_to_front() { /* touches at cl.at, sized by cl.r, colored by cl.lit */ }
let hedge = Habit { years: 15, ..Habit::alder() }.grow(..)
    .foliage_with(&Leafing { clump: 0.05, spray: 0.5, ..Leafing::alder() }, sun, seed);
let tufts = Sward { horizon, near: h, height: 26.0, wind: Wind { lean: 0.12, gust: 0.18, period: 140.0, seed }, ..Sward::default() }
    .grow(&meadow_mask, seed);
```

## 4. Study (`paintings/src/bin/study_green.rs`)

A summer meadow in daylight, 1.4:1: a leafy oak, a hedgerow of young
alders and beeches, far fields, a clear sky. The study's own habits:
- The sky is painted from a sky family of tubes and fused.
- Far woods are stippled.
- The meadow is laid in lean, with the oak's shadow and soft cloud shadows
  glazed over it.
- Leaf masses are laid in with short hatched strokes cut to
  `Foliage::mask` and colored by `lit`, then broken with dark touches,
  mid touches on the sunward side of lit clumps and a few crisp
  yellow-green touches where `lit > 0.45`.
- Grass goes last, as fine upturning rigger strokes, far first [NG p.56],
  with a sheen field that follows the gusts; a flower is one touch.

`--geom` shows the geometry flat (diagnostic only).

Evidence (in `~/tmp/green-ee8db531/`):
- `p1.jpg`: the first painted pass. The hedge was lollipops on stems,
  the clouds cotton spots, the far woods a straight strip.
- `geom.jpg` (whole) and `geom4.jpg`/`geom1.jpg` (the oak): geometry.
  Clumps only at limb ends first ("lollipops"), then with `inner` and
  `spray`, a filled crown shell with sky holes.
- `p3.png`/`p3.jpg`: the 1000px study as committed (also `out/study_green.png`).
- `oak3200.png`/`.jpg` and `oak3200_detail.jpg`: `--full --crop
  540,170,960,600` at 3200px, 3m22s on a busy machine. The 1000px render
  takes about 115 s from scratch; the hedge (47 shrubs, 21k clumps) is the
  slowest stage at about 41 s.

## Honest judgment

At 1000px it reads as a green summer landscape: meadow, hedgerow, a tree
in full leaf, cloud shadows. It is a clear step away from last night's
twilight palette. It does **not** yet read as a Friedrich summer passage.
- **The oak reads as a savanna tree.** This skeleton (oak habit, 24 years)
  is a flat umbrella on long bare limbs. The model grows an open young
  oak, not a mature, rounded, dense-crowned one.
- **At 3200px the leaf masses are cut-outs.** The hatched lay-in is
  clipped to the mask, so each mass is flat dark green with a crisp
  digital edge. The light touches are uniform round dabs, like polka dots.
  This is closer to a video-game tree than to Friedrich's foliage, whose
  drawings use "small strokes and hooks" and "jagged lines" [KH].
- **The meadow works best.** It has varied texture and upturning blades
  at the front, but the far half is a smooth lay-in, and the cloud shadows
  are a bit blotchy.

## Next

1. **Foliage painting:**
   - Leaf-shaped marks at the silhouette (hooked, jagged strokes following
     `tilt`/droop) instead of clipping.
   - Layered greens (MÄD's "up to four layers"): a warm mid layer under
     the dark, with sky scumbled back into `gaps`.
   - Light touches varied in size and shape, oriented along the twig.
2. **Growth:** a mature oak habit (more years with less self-thinning,
   and reiteration from dormant buds), so crowns are rounder and denser.
   Pollard willows are worth doing for Greifswald meadows.
3. **Performance:** the hedge's 21k clumps cost ~40 s at 1000px. Paint far
   hedges with a coarser `Leafing` (bigger clumps, `spray` 0) or a stipple
   over `mask`.
4. **Easel bindings:** `Skeleton::foliage`, `Foliage::{mask, lit, gaps}`,
   `Sward::grow` and the greens palettes.
