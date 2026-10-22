# Independent pigment mixing (engine 9)

New easel paintings record engine 9. Every engine number now uses the same
independent pigment mixer; there is no compatibility feature or restricted
mixing dependency. Existing logs remain replayable, with different colors.

## Model and provenance

The implementation follows Sochorová and Jamriška, *Practical Pigment Mixing
for Digital Painting* (2021), equations 1–14:
https://doi.org/10.1145/3478513.3480549.

Five real oil paints are the primaries: cobalt blue, chrome yellow
lemon, eosin lake (geranium lake), bone black and zinc white, from the Revigo
project's reconstruction of Van Gogh's palette (Geldof, Proaño Gaibor,
Ligterink, Hendriks and Kirchner 2018, https://doi.org/10.1186/s40494-018-0181-6).
Their reflectance measurements are published under CC0 at
https://github.com/fligt/revigo-spectra and vendored unchanged in
`scripts/pigment_data/revigo` (credited in `THIRD_PARTY_NOTICES.md`).
Each paint was drawn down 100 µm thick over the black and the white field
of an opacity chart, pure and mixed by weight with zinc white and bone
black, and measured 400–700 nm with specular included. The generator
solves, per wavelength, the non-hiding Kubelka–Munk film equation with the
source's Saunderson constants (k₁ 0.04, k₂ 0.49) for each paint's
absorption K and scattering S, zinc white and bone black first from their
own series, then each chromatic from its pure film, tints and tones. That
is the paper's own method; the fit residuals per sample are recorded in
`primaries.json`. The data's 400–700 nm grid is extended to the
conversion's 380–750 nm by holding the end values.

For concentrations c, the mixture uses K = ΣcᵢKᵢ and S = ΣcᵢSᵢ (plain
linear two-constant KM on weight shares; there is no concentration law),
then the infinite-thickness reflectance 1 / (1 + K/S + √((K/S)(K/S + 2)))
shown specular-excluded through the same Saunderson constants, the matte
paint colour the catalog's masstones describe. D65/CIE integration gives
linear RGB; the color-matching matrix comes from the existing MIT
`spectral.rs` port. Both K and S are wavelength-dependent as measured:
the lake and bone black scatter almost nothing, chrome yellow and zinc
white scatter strongly, cobalt blue in between, and bone black's
absorption is about ten times the lake's at 550 nm. These are the mixer's
primaries, not the palette's separate tube hiding, tinting-strength or
scattering parameters. Those rendering properties are unchanged.

Which five paints, out of the three blues, two chrome yellows, three
reds, bone black and two whites the data measures, is decided by
`scripts/pigment_basis_select.py` on half of the measured two-paint
mixtures only. The mixtures are split by paint before any score is read:
the paints of odd-numbered series (eosin on wax, red lead, viridian,
ultramarine) select, those of even-numbered series (eosin, cochineal,
vermilion, emerald green, cobalt blue, Kremer chrome yellow) test. Each
candidate basis encodes the selection paints from their measured masstone
colours, mixes them at the measured weights and is scored by the mean
OKLab distance to the measured mixture, leaving out its own fit data; the
lowest score wins, with ties within 0.005 broken by the residual the
catalog's tube colours need. `scripts/pigment_mixer_score.py` then scores
the built tables on the test half, which no choice ever read, with main's
mixer on the same inputs for reference, and reports the selection half
separately.

The palette's tinting strengths are the one free parameter calibrated to
the measurements. `scripts/pigment_strength_fit.py` fixes its own split
first: the fit set is each measured paint's zinc-white tint nearest 10 %
(the usual tinting-strength ratio) and, for lead white, its 10 % bone
black tone; the test set is every other measured two-paint mixture (the
50 % and 1 % tints, every lead-white and extender mixture), minus the
primaries' own ladders and the mixtures the primary selection read. For
each paint the strength is the weight factor that makes the tube's latent,
averaged with the white's as `Palette::pile` does, land nearest the
measured fit tint; the same fit with the catalog tube's own masstone in
place of the measured one gives the fifteen catalog tubes with a measured
counterpart their strengths (`scripts/pigment_strengths.json`). A
strength scales how much of a tube's recipe enters a mix; it cannot change
the recipe, so a tube whose masstone RGB encodes as black (cochineal,
Prussian blue) tints in the recipe's hue, not the paint's, whatever its
strength. `scripts/pigment_mixer_score.py` reports the test, fit and
selection-touched sets apart.

Nothing in the basis is fitted to a look or to the display gamut. The
forward table is the KM mixture clipped to the sRGB
cube. The inverse solves a constrained least-squares fit of that clipped
output, five nonnegative concentrations summing to one, plus 0.05 times
the raw RGB fit error. This regularizer can trade clipped fit error for
raw fit error; it is not a lexicographic tie-break or a guarantee of a
unique or smooth inverse. An additive RGB
residual stores the difference between the requested RGB and the decoded
pigment mixture. Thus the latent is eight floats: five concentrations
and three residual channels. Weighted averages operate directly on those
eight channels. Bone black is a primary because without it the inverse
has to build every dark from the three chromatic absorbers, which is black
in masstone but a navy in a tint; with it, grays and dark tints have a
neutral absorber to use.

The inverse and forward functions use independently generated tables:
the inverse 33³ nodes on an sRGB grid (four u16 concentrations each,
288 KiB), the forward 21⁴ nodes over the four square-root stick
coordinates (2.3 MiB, quadrilinear), with endpoint-dense, squared-ratio
axes to resolve steep KM changes near a pure pigment. 21 nodes per axis
keep the earlier 33³ table's interpolation error on catalog ramps (mean
0.006 against 0.009 OKLab, 99th percentile 0.048 against 0.055). The residual is calculated against the same
forward decoder used at runtime, so table interpolation does not introduce
a color error in an encode/decode round trip.

Mixing already encoded latents is linear. Decoding to RGB and encoding
again can change the recipe and later mixing behavior, especially for
colors outside the primaries' gamut. The existing palette/load and
drying paths retain that approximation. Neither the coarse inverse nor
the surrogate fit is a claim of measured physical pigment accuracy.

## Reproduction

`OPENBLAS_NUM_THREADS=1 uv run --script scripts/build_pigment_tables.py` regenerates both binary
tables and `crates/paint/src/mixer/primaries.json`. The script pins its
NumPy/SciPy build dependencies; neither is an engine runtime dependency.
One BLAS thread fixes the optimizer's reduction order for reproduction.
The JSON records the derived reflectance and K/S arrays, the samples each
fit used and their residuals, the wavelength grid, the near-white node
count, the clipped share of the forward grid and table SHA-256 values. No old mixer code, coefficients, tables or sampled outputs
are inputs to this generator.

This round was checked against the independent mixer guards (the trace
guard's bound now follows the measured strongest tinter), actual
table/runtime diagnostics, the test-half score against measured paint
mixtures and the main-vs-new sheet by eye. Passing the guards does not
close remaining visual differences.

The implementation is project MIT code. The method's paper is cited, not
redistributed. The public patent-record search found no matching pigment
mixing patent by either author or Secret Weapons; that is a search result,
not a patent-clearance opinion. The PR's evidence records the queries,
license audit, mixing comparison and painting replay.

## Validation and visual comparison

`notes/pigment-mixer-comparison.csv` contains only the independent mixer's
linear RGB results for 1,045 mixtures across 95 ramps: all 91 pairs in the
14-tube default box, three direct RGB pairs and lead white/ultramarine
from the full catalog, each in 10% steps. Tube mixtures use the actual
palette path and retain its tinting strengths. Old-engine samples and
their comparison columns are retained outside the repository as review
evidence, not as generator, fit or test inputs.

The independent mixer tests constrain hue, neutral grays, trace sensitivity,
blue chroma, lightness and round trips without pinning any old-engine
output. Passing them does not establish visual acceptance. The same eight
ramps and ten recipe probes are rendered side by side with main, along
with the gallery pair for *Last Light on the Tarn* (Claude Opus 5.5,
round unknown), [studio f0dc5e](https://stillwet.art/studio/?p=paint-studio-f0dc5e).
Those images are judged by eye only; no old-engine numeric output enters
the color-basis fit.

## Earlier constant edits

The superseded basis was hand-edited during the implementation review.
Its recorded checks were:

| Revision | What was checked |
|---|---|
| `8190bef` | White-tint brightness and the teal cast in the original tint chart; the white-scattering increase and flatter blue scattering addressed those defects. |
| `68bd70b` | Dark-pigment coverage, blue-tint chroma and the unchanged blend-reach/glaze tests; the earlier decoder could not reach bone black's dark end. |
| `a6e03e2` | Blue-tint chroma and the unchanged reach/glaze checks. The later full gate still exposed the white-over-dark knife-skim failure. |
| `7a03bc4` | Middle-gray chroma, blue tints, knife skim and glaze. The unchanged blend-reach test remained red. |
| `18584b1` | The unchanged knife-skim, blend-reach and glaze tests, followed by the full gate. |

Old-engine charts and numeric comparisons were also present during those
reviews, and an old-engine green output was incorrectly used as a test
target. That target and the sampled comparison columns have been removed.
The earlier sequence is not evidence that the superseded choices were
made without seeing those comparisons. The generator itself reads only
the synthetic basis and the credited CIE conversion matrix.

## Checkpoints and compatibility

New canvas checkpoints use `PAINTC19`, identifying the mixer independently
of the engine number and retaining the preceding material layout inside
the envelope. Older checkpoints with wet paint are explicitly refused:
their seven floats describe a different latent space. Empty older wet
layers can be imported with their stale latents cleared. Old resumable
replay checkpoints are also refused, because recorded brushes and tool
events contain the old representation. New replay checkpoints use
`EASLRP03`. Its compressed payload starts with the mixer identity before
any tool state, even when no canvas exists. Replaying an old log is supported; interpreting an old wet
latent as a new one is not.

The identity is the first eight SHA-256 digest bytes interpreted as a
little-endian u64. The hash covers the exact schema tag in `mixer::identity`,
then the forward table and the inverse table. The tag names the latent
layout, concentration coordinates, additive residual and interpolation;
a change to those semantics requires a new tag. Table changes alter the
identity automatically. The hash is cached outside the pixel path. This
is a compatibility fingerprint, not authentication. State dumps preserve
the producing identity as `mixer.identity`, so reconstructing a historical
dump never labels it with whichever tables happen to be checked out.

## Fixture and baseline policy

The mixer-dependent expected images and state captures are re-recorded
with explicit authorization for this change. All existing state and image
tolerances remain. Nonmixer state expectations retain main's values and
the thinner's known rag physics mismatch remains explicitly diagnosed,
not accepted as a new baseline pass. Only mixer-dependent colors, latents
and piles are refreshed. Every image must meet its existing tolerance,
including rag, and existing zero-solvent fields remain exactly zero.
The package reconstructs every saved canvas checkpoint from its state
dump and checks its FNV digest.

`UPDATE_GOLDEN=1` enables the existing scene recorder, the seeded numerical
plough recorder and the engine-2 drying-sheet recorder. The old-log recorder
is `scripts/tests/old_logs.sh <easel> --record <directory>`. The six thinner
scenes are captured with `notes/thinner/baseline/tools/run_scenes.sh`; its
manifest records the actual producing build and commands. Synthetic
obsolete-checkpoint tests generate their own input instead of distributing
the historical binary fixture.

Other generated fixture sources are in `notes/pigment-mixer-assets.md`.
The complete named license/action table and retained reference source list
are in `notes/pigment-mixer-licenses.md`; the portable per-package inventory
is `notes/pigment-mixer-dependencies.json`.
