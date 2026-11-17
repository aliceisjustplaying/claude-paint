# Bone black's absorption in engine 9

A painter found that thin films of bone black read teal, in printing ink and in brush paint
alike. Real bone (ivory) black is a warm-to-neutral black, and its thin glazes are a brownish
to neutral grey. The fault was in engine 9's pigment table. The fix is paint's
`mixer::BONE_BLACK_K` behind the easel's `--@ bone-black` switch (every new painting has it).
The fit is notes/research/bone_black_fit.py.

## 1. The fault, reproduced

The colours below come through the engine's own path (`Palette::pile`, `mixer::over9` over an
off-white of linear RGB 0.92, 0.91, 0.88, and `decode9` for tints), in CIELAB (D65). A film is
given as µm of tube paint. A tint is given as the black's volume fraction.

| | before (engine 9's table) | after (`--@ bone-black`) |
|---|---|---|
| film 1 µm over white | L 66.7 a +2.1 b +6.3 h 72 | L 69.1 a +1.3 b +9.4 h 82 |
| film 3 µm | L 40.2 **a −4.6** b +13.5 h 109 | L 39.9 a +2.9 b +13.4 h 78 |
| film 10 µm | L 7.8 **a −15.0** b +9.9 h 147 (teal) | L 2.6 a +2.0 b +3.3 h 58 |
| 0.03 in lead white | L 70.5 a −0.1 b −2.1 h 266 | L 71.5 a −0.1 b −0.7 h 259 |
| 0.10 in lead white | L 55.4 a −0.2 b −3.0 h 266 | L 56.9 a −0.5 b −1.1 h 245 |
| 0.30 in lead white | L 37.3 a −0.6 b −3.1 h 259 | L 38.9 a −0.8 b −1.0 h 229 |
| 0.03 in zinc white | L 67.4 a −0.1 b −5.5 h 269 | L 68.5 a −0.0 b −3.9 h 269 |
| 0.10 in zinc white | L 51.5 a +0.1 b −8.0 h 271 | L 53.0 a +0.0 b −6.0 h 270 |
| 0.30 in zinc white | L 33.4 a +0.0 b −8.2 h 270 | L 34.9 a +0.2 b −6.4 h 272 |

Revigo's measured tints, specular excluded, over the white field:
- zinc white: 07C (2.9 wt % black) a 0.0 b −5.8, h 270; 07D (9.6 %) a 0.3 b −7.2, h 273
- lead white: 08B (0.9 %) a −0.3 b −1.8, h 262; 08C (10.2 %) a −0.2 b −3.2, h 267

The blue-grey tint is real and measured: it is the white's scattering, which falls faster
with wavelength than the black's absorption. The teal is not. It appears only where the
black's own transmission shows, in thin films.

## 2. Where the data came from

- **Source.** Revigo's reflectance of bone black drawn down 100 µm over an opacity chart's
  black and white fields, mixed with zinc white (07C–07F) and lead white (08B, 08C) and into
  chromatic paints. Geldof, Proaño Gaibor, Ligterink, Hendriks and Kirchner, *Heritage
  Science* 6:17 (2018), CC0 data vendored in scripts/pigment_data/revigo. There is no pure
  bone-black sample.
- **First fit** (scripts/build_pigment_tables.py, measured.bin): per-wavelength
  Kubelka–Munk K and S, the paper's own method. It gives a smooth K falling from 98
  (400 nm) to 33 (650 nm), with S at the solver's floor (1.2e-4). Its thin films are brown.
- **Engine 9's calibration** (scripts/calibrate_pigments9.py, calibrated.bin): every paint
  is refitted jointly per volume. The objective is the samples' OkLab colour, with the raw
  spectra weighted only 0.05 and a weak (0.01) log-curvature prior. A colour residual pins
  three numbers per sample, so spectral shape could trade between paints:
  - the black's K came out 33 at 450 nm, **11.5 at 510 nm**, **61 at 550 nm** and 18 at
    610 nm;
  - zinc white's and lead white's S each gained a matching spike at 550 nm (zinc
    3.7 → 7.8 → 4.4; lead 4.6 → 10.0 → 6.7).

  Tints, where black K over white S is what counts, kept their colour. A thin film transmits
  exp(−K x) and so shows the 500–520 nm window: teal.
- **A free refit doesn't escape it.** Even a free per-wavelength spectral refit on the
  black-and-white ladders alone, with engine 9's whites, keeps a peak at 550 nm (41 vs 18
  at 520 nm): it is pinned by the whites' spike. Carbon has no absorption band there.

## 3. The physics, and the fit

- **Composition.** Bone black is about 10 % carbon and 84 % calcium phosphate
  (hydroxyapatite), with a little magnesium phosphate and calcium carbonate (Gettens and
  Stout 1966, via MFA CAMEO). Retail grades give 10–20 % carbon (Natural Pigments), and bone
  char 7–10 % (Wikipedia, *Bone char*).
- **The matrix doesn't absorb.** Hydroxyapatite (nω 1.651, nε 1.644) is colourless, and in
  linseed oil (n 1.48) it scatters weakly. The colour is the carbon's.
- **Carbon's index is nearly flat.** Strongly absorbing carbon has m ≈ 1.95 − 0.79i, with
  almost no spectral dependence across the visible (Bond and Bergstrom 2006). For particles
  small against the wavelength, absorption per volume is (6π/λ)·Im[(m²−1)/(m²+2)], so K goes
  as λ^−1: black carbon's absorption Ångström exponent of about 1. Charred organic matter
  ("brown carbon") steepens it; large opaque grains flatten it.
- **The shape.** So the fit is K(λ) = K550 (λ/550 nm)^−α. It is fitted to the measured
  Revigo reflectance (both fields, specular included, Saunderson k1 0.04 and k2 0.49) of the
  six samples where bone black is the only absorber: 07C–07F and 08B, 08C. Recipes are
  converted to wet-paint volume as in calibrate_pigments9.py, and the whites keep engine 9's
  K and S.
  - **K550 = 26.28 per 100 µm of tube paint per volume, α = 1.132 ± 0.081**
  - zinc ladders alone: α 1.26; lead ladders alone: α 1.02
  - the first Revigo fit's slope (measured.bin) is steeper, about 2, but its zinc white
    differs
- **S** stays the table's, about 1e-4 per 100 µm. No fit resolves it: K/S is about 10⁵.
- **Magnitude check** (estimate):
  - Bone black paint is 48 wt % oil (pigment_densities.json; pigment density 2.29, oil
    0.93), so the pigment's volume fraction is φ ≈ 0.31.
  - Freely dispersed soot at a mass absorption cross section of 7.5 m²/g (Bond and
    Bergstrom 2006) and 10–20 % carbon would give K ≈ 100–200 per 100 µm.
  - The fitted 26 is what grains about 3.5 µm across give if they absorb near their
    surface: K ≈ 2 · 1.5 φ / d. That matches bone black's few-micrometre grains.
  - So the measured K is consistent with its carbon being held in grains, not dispersed.
    This is a check, not an input.

## 4. The cost: colour error against Revigo

CIELAB ΔE76 over the white field, specular excluded (the calibration's convention):

| sample set | engine 9's table | power law |
|---|---|---|
| zinc ladders 07C, 07D, 07E, 07F | 3.18, 1.80, 2.40, 4.06 | 2.21, 0.97, 3.65, 4.96 |
| lead ladders 08B, 08C | 0.73, 0.72 | 2.11, 2.45 (a little less blue than measured) |
| mean of the six fitted ladders | 2.15 | 2.72 |
| mean of all 27 bone-black samples | 6.85 | 8.11 |

The black's mixtures with chromatic paints get worse, by up to about 5. Those paints were
co-fitted with the dipped black, so their mixtures with it carry the trade. Examples:
- 06E vermilion: 0.59 → 2.47
- 05E red lead: 1.13 → 3.94
- 12D cobalt: 6.05 → 10.10
- 14E chrome yellow: 19.9 → 24.6

The same goes for barium sulphate 16C: 1.30 → 6.88.

A fit that is right for both would recalibrate every engine-9 paint with spectral-shape
constraints. That changes every new colour, so it is left as an open question. The switch
changes bone black only, in new paintings.

## 5. Other measured data looked at

- **CHSOS FORS database** (Cosentino 2014, *e-conservation* 2:54–65; the CSV download with
  the SHA-256s that oil_k_fit.py checks). Bone black, ivory black, vine black and lamp black
  are each measured as powder, in linseed oil and in gum arabic.
  - The swatches are opaque, and their reflectance carries an uncorrected offset of about
    15 % (stray light and surface), so they don't constrain K's shape.
  - Their masstones are flat and neutral: in oil, bone black C* 0.4 and ivory black 0.3;
    lamp black 1.2 at h 272 and vine black 1.1 at h 303, both slightly bluish. None is teal.
- **Not obtained:** the Berns/RIT artist-paint spectral database (Okumura 2005) and any
  Kremer reflectance data. No bone-black tint spectra beyond Revigo's were found.

## 6. Other carbon blacks in the engine

- **Vine black** (researched table, product 12015, a grape-seed black). Its K is flat
  (α 0.07) over a flat S fitted to one masstone. Films are neutral (3 µm a −0.5 b +1.9;
  10 µm a +0.1 b +0.9) and tints bluish (0.1 in lead white h 260). No teal fault, so it is
  unchanged. Its flat K is that table's own flat-S assumption.
- **Bitumen** (a proxy tube built from RGB) holds 4.6 % of the measured BoneBlack primary.
  Its films are warm brown (3 µm h 58, 10 µm h 53), so it is unchanged.
- **Ivory black, lamp black:** not tubes in the catalog. Ivory black is historically the
  same pigment as bone black. **Black chalk** (graphite.rs) is a coverage model with no
  spectra.

## Sources

- Geldof, Proaño Gaibor, Ligterink, Hendriks, Kirchner, "Reconstructing Van Gogh's palette to
  determine the optical characteristics of his paints", *Heritage Science* 6:17 (2018),
  https://doi.org/10.1186/s40494-018-0181-6 ; data https://github.com/fligt/revigo-spectra (CC0).
- T. C. Bond and R. W. Bergstrom, "Light Absorption by Carbonaceous Particles: An Investigative
  Review", *Aerosol Sci. Technol.* 40:27–67 (2006), https://doi.org/10.1080/02786820500421521
  (m = 1.95 − 0.79i, nearly flat in the visible; MAC 7.5 ± 1.2 m²/g at 550 nm).
- R. J. Gettens and G. L. Stout, *Painting Materials: A Short Encyclopaedia*, Dover 1966, as
  cited by MFA CAMEO, https://cameo.mfa.org/wiki/Bone_black (about 10 % carbon, 84 % calcium
  hydroxyapatite).
- Natural Pigments, Bone Black, https://www.naturalpigments.com/bone-black-pigment.html
  (10–20 % carbon); Wikipedia, *Bone char*, https://en.wikipedia.org/wiki/Bone_char.
- Hydroxylapatite, nω 1.651, nε 1.644: ICDD card 00-009-0432; Mindat,
  https://www.mindat.org/min-1992.html (after the *Handbook of Mineralogy*).
- A. Cosentino, "FORS spectral database of historical pigments in different binders",
  *e-conservation* 2:54–65 (2014), https://chsopensource.org/new-free-pigments-fors-spectroscopy-database/.
