# Spectral paint mixing (branch `spectral`)

## Verdict
**Not integrated.** Spectral Kubelka–Munk doesn't clearly help the painters,
so it stays an optional module (`paint::spectral`) with a study sheet. The
engine's mixing (Mixbox) and layering (KM per RGB channel) are unchanged.
The golden was not re-recorded, and checkpoints and replay are untouched.

Why, in three numbers (OKLab ΔE; a just-noticeable difference is about 0.02):
- **Spectral KM vs KM per RGB channel, same concentration model:** at most
  0.040 in mixing (Prussian blue + chrome yellow), 0.003–0.023 elsewhere, and
  at most 0.023 in layering (glazes and varnish). spectral.js builds a
  spectrum from seven broad basis spectra that are nearly flat within the
  blue, green and red bands (their edges fall near 490 and 590 nm). Per
  wavelength, KM on those spectra is close to KM per channel.
- **Pigment-shaped spectra** (smalt's cobalt bands, chrome yellow's sharp
  edge and so on, fitted to each tube's masstone) move mixes a further
  0.024 at most and glazes 0.017 at most. The shifts go the way painters
  describe (smalt + ochre greens warmer and duller, Prussian blue + ochre
  grayer), but they are about one JND.
- **Cost:** spectral layering is 17× the per-channel cost (439 vs 26
  ns/call). Routing all of the engine's layering through it made
  `study_color` 1.8× slower (11.4 → 20.6 s wall, 13.5 → 24.9 s user), and
  the aimed dabs 8.5× slower (0.76 → 6.48 s). The picture changed by mean
  ΔE 0.0010, with 0.08% of pixels over 0.02. The two renders look the same.

The difference that *is* visible (0.05–0.14) is between Mixbox and any KM
concentration model, RGB or spectral. See "What does matter" below.

## What landed
- `crates/paint/src/spectral.rs`: a port of spectral.js 3.0
  (https://github.com/rvanwijnen/spectral.js, commit `bb2b05c`, MIT,
  Copyright (c) 2025 Ronald van Wijnen; full license in the module header
  and `THIRD_PARTY_NOTICES.md`). Its tables (basis spectra, D65-weighted
  CIE 1931 color matching functions, matrices) were generated from the JS
  source, not typed by hand.
  - `from_rgb`, `to_xyz`, `to_rgb`: linear RGB ↔ 38-sample reflectance
    spectrum, 380–750 nm (round trip exact to 2e-5).
  - `ks`, `km`: K/S ↔ R∞. `km` uses the reciprocal form
    1 / (1 + q + √(q(q+2))), which stays stable in f32 for deep absorbers.
  - `mix_js`: spectral.js `mix`, matching its output to 2e-4
    (`spectral::tests::matches_spectral_js`, reference values from node).
  - `mix_ks`: single-constant K/S mixing with plain weights.
  - `SpectralPigment { k, s }`: the engine's `Pigment` per wavelength.
    `masstone(rgb, s)`, `from_masstone_spectrum`, `from_appearance`,
    `from_appearance_spectra`, `with_hiding`, `varnish`, `mix`
    (two-constant: K and S add by concentration), `scaled` (medium),
    `masstone_color`, `over(rgb, coats)` and `over_spectrum`.
  - `curve(&[(nm, value)])` and `fit_shape(rgb, &shape)`: a spectrum
    with a pigment's known shape whose color is exactly `rgb`. The shape's
    K/S is scaled by exp(a + b·u + c·u²) and solved by damped Newton, so
    narrow bands stay where the shape puts them.
- `pigment.rs`: the per-channel KM math is factored into
  `ks_from_appearance` and `layer1` (pub(crate)), so spectral layering
  runs exactly the same formulas once per wavelength. Output is unchanged:
  every test passes, including the golden.
- `Palette::pile(parts)`: the mixture the palette makes from given parts
  (a public wrapper, used by the study).
- `paintings/src/bin/study_spectral.rs` → `out/study_spectral.png`, with
  the numbers printed to stderr.

```rust
use paint::spectral::{self, SpectralPigment};
let blue = SpectralPigment::masstone(hex("#172440"), 0.06);
let yellow = SpectralPigment::masstone(hex("#e8b21c"), 1.98);
let green = SpectralPigment::mix(&[(blue, 0.5 * 3.0), (yellow, 0.5)]).masstone_color();
let glazed = blue.scaled(0.2).over(hex("#e8b21c"), 1.0);          // a glaze over yellow
let js = spectral::mix_js(&[(hex("#002185"), 1.0, 1.0), (hex("#fcd200"), 1.0, 1.0)]);
```

## Design
The engine describes a paint by its masstone (RGB) and a flat KM scattering
S per coat. Absorption is K = S·(1−R∞)²/2R∞ per channel. Mixbox mixes
masstones, weighted by volume × tinting strength, and S mixes by volume.
The spectral port keeps those semantics per wavelength: the masstone
becomes a spectrum by `from_rgb` (or `fit_shape`), S stays flat, K(λ)
follows, and mixing adds K and S by concentration. With weights w = volume
× strength, the mixture's K/S(λ) is Σ w S (K/S)ᵢ / Σ w S. Layering uses the
engine's KM layer formulas at each wavelength and composites over the
substrate's spectrum. A dry picture stored as RGB is converted with
`from_rgb` at each composite.

spectral.js's own `mix` is a different model: single-constant KM with
heuristic weights (factor² × strength² × luminance). It is ported as
`mix_js` for reference and shown as strip 4 in the study.

## Measurements (`cargo paint study_spectral`, stderr)
Tube S per coat, from the palette's hiding: lead white 3.27, smalt 0.22,
Prussian blue 0.06, yellow ochre 0.89, chrome yellow 1.98, raw umber 0.31.

(a) Greens, 50/50 by volume (Mixbox | KM per RGB | spectral KM | spectral.js):
| pair | Mixbox | RGB-KM | spectral KM | spectral.js | RGB vs spectral KM, max ΔE over the ramp |
|---|---|---|---|---|---|
| Prussian + ochre | #223e43 | #374938 | #364c42 | #233641 | 0.016 |
| Prussian + chrome | #1f4938 | #52681d | #516d3a | #2f4b3c | 0.040 |
| smalt + ochre | #8e884a | #998539 | #9c873e | #97863f | 0.016 |
| smalt + chrome | #a1a432 | #c0aa1d | #c4ac28 | #beaa2a | 0.023 |

Max ΔE from Mixbox along the ramps: KM models 0.06–0.14, spectral.js
0.035–0.13.

(b) Tints with lead white: RGB vs spectral KM differ by at most 0.003
(smalt) and 0.005 (Prussian). Both KM models make paler tints than Mixbox
(smalt with 20% white: L 0.78 vs 0.68). Lead white's S is 15× smalt's, so
white dominates the mixture.

(c) Glazes, RGB-KM vs spectral, at 0.5–4 coats: umber over sky blue
≤ 0.002; madder over yellow ochre ≤ 0.019; Prussian blue glaze over chrome
yellow ≤ 0.023 (spectral a little greener); umber over madder over sky
≤ 0.002. Glaze order (madder then umber vs umber then madder) changes the
color by 0.019 in both models. KM per channel already captures order.

(d) Aged varnish (#e6d3a4) over darks and lights: ≤ 0.004 between models
up to 2 coats, and 0.010 at 4 coats over lead white. Both warm the darks
and yellow the lights by the same amount (4 coats over lead white: shift
0.376 vs 0.385).

(e) Cost per call on this machine, under load from the other agents:
Mixbox mix of three tubes 298 ns; KM per RGB 45 ns; spectral KM 335 ns;
spectral.js mix 418 ns; layer per RGB 26 ns; spectral layer 439 ns (343 ns
already in spectra); `SpectralPigment::masstone` 54 ns. Mixing would cost
about the same as today, but layering runs everywhere: in `bake`,
`look_px` and `Canvas::under`, and hundreds of times per aim search. The
render measurement is in the verdict. To take it, I temporarily routed
`Pigment::over` through spectral behind an env var. That change was never
committed.

(F) Pigment-shaped spectra (the study's `shape_of`): written from pigment
knowledge, not measured curves. They are qualitative features, fitted to
each tube's masstone:
- smalt: Co²⁺ bands near 540/590/640 nm, blue and deep-red reflectance;
- Prussian blue: a maximum near 450–490 nm and broad absorption toward
  700 nm;
- chrome yellow: an edge near 510 nm;
- goethite ochre: a gradual edge and a shoulder near 650 nm;
- vermilion: an edge near 600 nm;
- umber: rising slowly toward the red;
- green earth: a broad maximum near 500–550 nm;
- lead white: flat.

Against the basis spectra, mixes move by at most 0.024 (smalt + ochre at
20% ochre: h 111 → 94, C 0.074 → 0.063; Prussian + ochre at 50%: C 0.032 →
0.020). Glazes move by at most 0.017 (madder over ochre) and varnish by at
most 0.015 (over a Prussian-ochre dark green).

## What does matter: the concentration model, not the spectrum
The visible gap is Mixbox vs KM, 0.05–0.14 on the same piles, and it comes
from how much each paint counts in a mixture. Judged against what is known
of the paints:
- **Prussian blue + chrome yellow.** Commercial chrome greens were
  mixtures of the two. A web search summary, citing the NBS circular
  "Paint pigments: yellow, brown, blue, green and bronze"
  (https://www.govinfo.gov/content/pkg/GOVPUB-C13-09cd7735b9b31a49e23a94b4e65bef89/pdf/GOVPUB-C13-09cd7735b9b31a49e23a94b4e65bef89.pdf),
  puts yellow-greens at 10–15% Prussian blue and medium greens at 15–25%
  (I didn't check the circular itself). Mixbox gets this: 20% blue gives
  #4a7634, a medium green. KM with S derived from hiding
  underrates Prussian blue, because its tiny S (0.06) means its K is small
  next to chrome yellow's. The same pile comes out yellow-olive (#828f1c).
  Mixbox is more paint-like here.
- **Smalt + lead white.** KM makes smalt vanish into white faster than
  Mixbox does. That fits smalt's known weakness, but the palette already
  models weakness through `strength` (0.45).

So a switch to KM mixing, spectral or not, would first need a better
concentration model: tinting strength acting on K, and an S per tube
measured rather than derived from hiding. That is a palette question, and
it can be answered in RGB.

## Evidence
- `out/study_spectral.png` (1000 px), its layout mapped in the binary's
  header comment. In sections A and B, strips 2 (RGB-KM) and 3 (spectral KM)
  can't be told apart, while strips 1 (Mixbox) and 4 (spectral.js) visibly
  differ. In C and D, each pair of strips matches except for a slight
  separation in Prussian blue over chrome yellow. In F, the basis and
  shaped strips are nearly identical.
- Scratch, not committed: `~/tmp/spectral-a05621c9/`,
  `color_rgb.png` vs `color_spec.png` (study_color with the engine's layering
  vs spectral layering), crops `crop_rgb.jpg` and `crop_spec.jpg`, and
  `imgdiff.py` (OKLab ΔE per pixel).

## If someone revisits
- Measured reflectance spectra for the historical pigments (for example,
  the open FORS databases of artists' pigments) would replace `shape_of`.
  My guess: still about one JND in mixes, more in glazes that sit exactly
  on a band edge.
- A spectral engine would store spectra (or a few principal components)
  in the wet layer and the dry picture, not RGB, and convert only for
  display. Otherwise every composite pays for `from_rgb`/`to_rgb`, and each
  one projects back onto the broad basis.
- The concentration model (above) is the cheaper and more visible thing to
  work on.
