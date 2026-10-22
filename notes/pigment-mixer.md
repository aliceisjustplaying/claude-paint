# Independent pigment mixing (engine 9)

New easel paintings record engine 9. Every engine number uses the same
independent pigment mixer. Existing logs remain replayable with different
colors. There is no Mixbox runtime, table, coefficient or fitting input.

## Current design: measured tubes keep their pigment

Fifteen of the 54 catalog tubes have a Revigo counterpart. They enter the
mixer as that paint's own wavelength-dependent absorption K and scattering
S, not as an RGB reconstruction. Their displayed masstones come from the
same coefficients, without an RGB residual. All fifteen use scalar strength
1.0: their measured K and S already carry their relative tinting strength.
The three vermilion grades share the single measured Vermillion paint.
Their separate hiding, stiffness, drying and oil properties remain catalog
properties, not new spectral fits.

The other 39 tubes still enter through RGB: the same 33³ inverse table
represents their catalog colors with five primary concentrations, plus an
additive linear RGB residual. Their scalar strengths are unchanged,
0.3–1.6. Those numbers weight the recipe; they do not make a measured and an
RGB-only pigment equally strong. For an RGB-only tube, the five measured
primaries in its recipe determine its wavelength-dependent tinting behavior.
No missing pigment spectra or strength measurements are invented.

The latent has 21 floats: eighteen measured paint concentrations, followed
by three linear RGB residual channels. Its first five paints remain cobalt
blue, lemon chrome, eosin lake, bone black and zinc white. Thirteen more
paints supply the remaining catalog counterparts and the measured-paint
evaluation, including its white extenders. The order and all coefficients
are recorded in `crates/paint/src/mixer/measured.json`.

Mixing is an ordinary weighted latent average. For every wavelength,
K = ΣcᵢKᵢ and S = ΣcᵢSᵢ, then
R∞ = 1 / (1 + K/S + √((K/S)(K/S + 2))).
There is no concentration law. The decoder evaluates this equation directly
at 38 wavelengths, applies the source's specular-excluded Saunderson
conversion and integrates through D65/CIE to linear RGB. The old 21⁴ forward
table is removed: a fixed-dimensional table cannot decode arbitrary
combinations of eighteen pigments. RGB residuals are computed against this
same exact decoder, so RGB encode/decode round trips remain exact.

`Palette::pile` retains the mixed latent. `Mixture::paint`, medium,
drying-rate changes, brush loads, knife loads and the wet layer carry it
without decoding to RGB and reconstructing another pigment. A knife's
reported paint also retains its existing latent. RGB-only piles retain
their original RGB-derived recipes through those handoffs too. Baking a
layer still renders its appearance to RGB; this change does not turn the
layering optics into a spectral renderer.

## Measurement source and calibration split

The Revigo reconstruction of Van Gogh's palette is documented in Geldof,
Proaño Gaibor, Ligterink, Hendriks and Kirchner (2018),
https://doi.org/10.1186/s40494-018-0181-6. Unchanged reflectance measurements
and their CC0 license are vendored in `scripts/pigment_data/revigo` from
https://github.com/fligt/revigo-spectra and credited in
`THIRD_PARTY_NOTICES.md`.

The paints were drawn down 100 µm over black and white fields, pure and in
weight mixtures, measured at 400–700 nm with specular included. Per
wavelength, the generator solves the non-hiding KM film equation with the
paper's Saunderson constants k₁ = 0.04 and k₂ = 0.49. Values outside the
measured range hold the nearest endpoint to cover the existing MIT
`spectral.rs` conversion's 380–750 nm range.

The five RGB reconstruction primaries and their fits are unchanged from
round 2. Their selection used the split committed in
`scripts/pigment_basis_select.py`; their own measurement ladders are not
test data. The RGB inverse table is byte-identical to round 3.

Every additional paint's K and S are derived from its pure film and, where
available, its single round-3 fit tint. Lead white uses its pure film and
08C. The extenders use only their pure films. There is no second scalar
strength fit and no calibration on the remaining tint levels, mixtures with
other whites or selection-touched mixtures. The generator asserts that no
test mixture appears in its coefficient fit data.

`split_for_strengths` in `scripts/pigment_strength_fit.py` is unchanged.
The eleven fit mixtures remain 01B 03B 04B 05B 06B 08C 09B 10B 11B 13B 14B.
The seventeen test mixtures remain 03C 04C 04F 06C 08D 08E 08F 08G 10C 13C
13F 13G 14C 16D 16F 17D 17F. The five previously selection-touched mixtures
are reported separately. The test is not used to choose coefficients or
strengths. No main output, picture or look target is a generation input.
This preserves the committed split rather than claiming a fresh holdout:
08E was previously read during round 2's primary selection.

## Reproduction and evaluation

```sh
OPENBLAS_NUM_THREADS=1 uv run --script scripts/build_pigment_tables.py
cargo build --release --locked -p paint --example mixpairs
cargo build --release --locked -p easel
uv run --script scripts/pigment_strength_fit.py target/release/examples/mixpairs --measured
uv run --script scripts/pigment_mixer_score.py target/release/examples/mixpairs \
  --strengths scripts/pigment_strengths.json --out score.md --sheet real-paint.png
```

Generation pins NumPy and SciPy and one BLAS thread. `primaries.json`
records the original basis fits and inverse hash. `measured.json` records
every coefficient, source sample, fit residual and coefficient-table hash.
`pigment_strengths.json` records the unchanged evaluation split and unit
measured strengths; `--measured` does not optimize a scalar.

The scoring example accepts named measured paints as well as RGB inputs.
The new mixer is scored using each paint's own K and S at the measured
weight share. Main is scored only as a reference, from measured masstone
RGBs and its own round-3 fit-set-calibrated strengths. Test, fit and
selection-touched scores are labeled separately. The fit score is not an
independent test. Guard thresholds and existing goldens are unchanged.

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

`PAINTC19` checkpoints and `EASLRP03` replay envelopes check the mixer
identity before interpreting current latents. The first eight SHA-256 bytes
of the exact decoder schema tag, measured coefficient bytes and inverse
table identify this representation. Round-3 occupied checkpoints and tool
snapshots are incompatible and explicitly refused; old Lua logs remain
supported. Empty obsolete canvas wet layers can still be imported with
their old latents cleared.

State dumps retain their producing `mixer.identity`, so reconstruction
never relabels historical dumps with current tables. The new representation
is covered by RGB round trips, mixed measured/RGB associativity, catalog
pigment preservation through pile/paint/knife handoffs and occupied
measured-pigment checkpoint round trips.

## Fixture and licensing policy

No old-log golden or thinner capture is re-recorded in round 4. Expected
images from a different table fingerprint remain diagnosed as failures,
with unchanged tolerances. No noncolor transport parameter is calibrated.

The model follows Sochorová and Jamriška, *Practical Pigment Mixing for
Digital Painting* (2021), equations 1–14,
https://doi.org/10.1145/3478513.3480549. The implementation is project MIT
code. The method's paper is cited, not redistributed. The earlier public
patent-record search found no matching pigment mixing patent by either
author or Secret Weapons; that is a search result, not a patent-clearance
opinion.

Generated fixture sources are in `notes/pigment-mixer-assets.md`. The
license/action table and retained reference source list are in
`notes/pigment-mixer-licenses.md`; the per-package inventory is
`notes/pigment-mixer-dependencies.json`.
