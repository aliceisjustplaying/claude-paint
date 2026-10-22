# Independent pigment mixing (engine 9)

New easel paintings record engine 9. Every engine number now uses the same
independent pigment mixer; there is no compatibility feature or restricted
mixing dependency. Existing logs remain replayable, with different colors.

## Model and provenance

The implementation follows Sochorová and Jamriška, *Practical Pigment Mixing
for Digital Painting* (2021), equations 1–14:
https://doi.org/10.1145/3478513.3480549.

Four synthetic primary pigments contribute absorption K and scattering S
at 38 wavelengths, 380–750 nm. For concentrations c, the mixture uses
K = ΣcᵢKᵢ and S = ΣcᵢSᵢ, then the infinite-thickness Kubelka–Munk
reflectance 1 / (1 + K/S + √((K/S)(K/S + 2))). D65/CIE integration gives
linear RGB. The color-matching matrix comes from the existing MIT
`spectral.rs` port, credited in `THIRD_PARTY_NOTICES.md`.

The primary reflectance curves and scattering functions are independently
authored, not measured pigment data. Blue and magenta use broad smooth
bands, yellow a two-step rising edge and white a nearly flat reflector.
The blue and magenta are dark, weakly scattering pigments, so dark tube
colors can be represented mostly by pigment rather than large negative
RGB corrections. Scattering is wavelength-independent for every primary;
white scatters more strongly than the chromatics.
These are the mixer's virtual primaries, not the palette's separate tube
hiding, tinting-strength or scattering parameters. Those rendering
properties are unchanged.

The generator fits the primary reflectances to keep sampled mixtures in
the display gamut. It solves a constrained RGB least-squares inverse to
find four nonnegative concentrations summing to one. An additive RGB
residual stores the difference between the requested RGB and the decoded
pigment mixture. Thus the latent remains seven floats: four concentrations
and three residual channels. Weighted averages operate directly on those
seven channels.

The inverse and forward functions use independently generated 33³ tables
(632 KiB combined). The inverse is sampled on an sRGB grid. The forward
table uses endpoint-dense, squared-ratio coordinates to resolve steep KM
changes near a pure pigment. The residual is calculated against the same
forward decoder used at runtime, so table interpolation does not introduce
a color error in an encode/decode round trip.

Mixing already encoded latents is linear. Decoding to RGB and encoding
again can change the recipe and later mixing behavior, especially for
colors outside the virtual pigments' gamut. The existing palette/load and
drying paths retain that approximation. Neither the coarse inverse nor
the surrogate fit is a claim of measured physical pigment accuracy.

## Reproduction

`OPENBLAS_NUM_THREADS=1 uv run --script scripts/build_pigment_tables.py` regenerates both binary
tables and `crates/paint/src/mixer/primaries.json`. The script pins its
NumPy/SciPy build dependencies; neither is an engine runtime dependency.
One BLAS thread fixes the optimizer's reduction order for reproduction.
The JSON records the generated K/S arrays, wavelength grid and table
SHA-256 values. No old mixer code, coefficients, tables or sampled outputs
are inputs to this generator.

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
