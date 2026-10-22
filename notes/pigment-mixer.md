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
bands, yellow a rising edge and white a nearly flat reflector. White
scatters more strongly than the chromatics; blue's scattering varies by
about a factor of two, not an extreme wavelength-dependent multiplier.
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

`uv run --script scripts/build_pigment_tables.py` regenerates both binary
tables and `crates/paint/src/mixer/primaries.json`. The script pins its
NumPy/SciPy build dependencies; neither is an engine runtime dependency.
The JSON records the generated K/S arrays, wavelength grid and table
SHA-256 values. No old mixer code, coefficients, tables or sampled outputs
are inputs to this generator.

The implementation is project MIT code. The method's paper is cited, not
redistributed. The public patent-record search found no matching pigment
mixing patent by either author or Secret Weapons; that is a search result,
not a patent-clearance opinion. The PR's evidence records the queries,
license audit, mixing comparison and painting replay.

## Checkpoints and compatibility

New canvas checkpoints use `PAINTC19`, identifying mixer 1 independently
of the engine number and retaining the preceding material layout inside
the envelope. Older checkpoints with wet paint are explicitly refused:
their seven floats describe a different latent space. Empty older wet
layers can be imported with their stale latents cleared. Old resumable
replay checkpoints are also refused, because recorded brushes and tool
events contain the old representation. New replay checkpoints use
`EASLRP02`. Replaying an old log is supported; interpreting an old wet
latent as a new one is not.

## Fixture and baseline policy

The mixer-dependent expected images and state captures are re-recorded
with explicit authorization for this change. All existing state and image
tolerances remain. The thinner's old rag exception is retired: every state
or image mismatch now fails, and existing zero-solvent fields must remain
exactly zero. The package still reconstructs every saved canvas checkpoint
from its state dump and checks its FNV digest.

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
