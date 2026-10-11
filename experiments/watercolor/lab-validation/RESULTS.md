# Validation results

Pinned upstream: `a19bfa2a026856a98ceb400a8eec41f091f04a11`.
The reference used Chromium ANGLE Vulkan SwiftShader, `SUBPIXEL_BITS=4`,
16×12 RGBA16F textures, LINEAR filtering and CLAMP_TO_EDGE wrapping.
Four cases exercised 148 passes, including six pigment splats, salt and scrub.
The numeric files are under `~/.capy/work/watercolor-lab-port/captures/`.

## Supported passes

Every isolated supported pass is within `max(1e-5, 4 component half-float ULPs)`.
The accumulating replay has 37 exceeded samples before salt: 32 are four
closed-system velocity differences repeated in later unchanged-field captures;
five are accumulated brush-input differences. These are reported, not erased.
The largest supported accumulated suspension difference is 0.00390625.

The table excludes the salt pass and subsequent salt-contaminated replay
states. Max is the largest component difference; mean averages every component
over all captures of that pass and field. Scrub's isolated difference is zero.

| Pass | Output | Isolated max | Isolated mean | Replay max | Replay mean |
|---|---|---:|---:|---:|---:|
| blur_wet | blur_w | 0.0009765625 | 2.20627930e-7 | 0.00048828125 | 2.81790749e-7 |
| flux | flux | 0.0000152587891 | 2.51337695e-9 | 0.0000152587891 | 1.48772811e-8 |
| water | flow | 0.000244140625 | 2.50883974e-7 | 0.0009765625 | 6.10971489e-7 |
| move_pigment | sus_k | 0.0000152587891 | 2.45964680e-9 | 0.00390625 | 3.56932362e-6 |
| move_pigment | sus_s | 0.0000152587891 | 2.37009655e-9 | 0.000122070312 | 4.99600831e-7 |
| move_pigment | sus_p | 0.00000762939453 | 1.03281286e-9 | 0.000244140625 | 3.14810910e-7 |
| transfer_dep | dep_k | 0.00000762939453 | 2.13129589e-9 | 0.0000686645508 | 1.11734829e-7 |
| transfer_dep | dep_s | 0.00000190734863 | 5.25361452e-10 | 0.00000381469727 | 1.82801905e-8 |
| transfer_sus | sus_k | 0.0000610351562 | 8.28041289e-9 | 0.0029296875 | 3.33302439e-6 |
| transfer_sus | sus_s | 0.0000610351562 | 6.32821749e-9 | 0.000122070312 | 5.06729031e-7 |
| transfer_sus | sus_p | 0.000000238418579 | 5.67151568e-10 | 0.000244140625 | 3.38302925e-7 |
| capillary | cap | 0.0000610351562 | 3.66917214e-8 | 0.0000610351562 | 3.68350018e-8 |
| dry_fold | dried | 0 | 0 | 0.00048828125 | 5.19630237e-8 |
| dry_clear_sus | sus_k | 0.000000953674316 | 1.99398551e-9 | 0.0029296875 | 3.32355595e-6 |
| dry_clear_sus | sus_s | 0.000000119209290 | 1.79100495e-10 | 0.000122070312 | 5.04997726e-7 |
| dry_clear_sus | sus_p | 0.000000238418579 | 5.61181551e-10 | 0.000244140625 | 3.36380580e-7 |
| dry_clear_dep | dep_k | 0 | 0 | 0.0000686645508 | 1.11722889e-7 |
| dry_clear_dep | dep_s | 0 | 0 | 0.00000381469727 | 1.82742205e-8 |
| splat | flow | 0 | 0 | 0.000244140625 | 2.01773623e-7 |
| splat | sus_k | 0 | 0 | 0.0009765625 | 4.09175280e-6 |
| splat | sus_s | 0 | 0 | 0.000122070312 | 5.47979855e-7 |
| splat | sus_p | 0 | 0 | 0.000244140625 | 4.46142318e-7 |

Source color, backlight and six X-ray renders from identical final textures
have zero max/mean byte differences across all four cases. Those isolated
render matches do not claim the accumulated states are identical.

## Salt remains a failure

The isolated salt channel has max difference 4.6953125 and mean 0.313096710,
with 184 exceeded components. The full replay report has 9,414 exceeded
components because salt's different output propagates into later passes.
The parity executable deliberately exits nonzero. Salt is hidden in both
comparison UIs; it is not presented as a verified feature. No physics was tuned
to conceal this difference.

## Conservation and source drift

With evaporation, absorption and capillary drying off, no salt/scrub and no
dry-fold, the three-step diagnostic reads:

| Quantity | Before | Original after | Rust after |
|---|---:|---:|---:|
| Standing + capillary water | 19.1286010742 | 19.1491866708 | 19.1492171884 |
| Suspended + deposited pigment | 5.8754234314 | 6.5248356462 | 6.5248708725 |

Original pigment gain occurs at `MOVE_PIGMENT`: +0.140772820, +0.239760339
and +0.268634081 on its three passes. The cause is upstream
`src/engine/shaders.ts:188–194`: receiver cells backtrace and gather suspension
with `texture(uSusK, at)`, without conservative donor normalization. The source
and port both retain this behavior. Total pigment gain is about 11.053%.

At normal 1056×704 dry-stroke size, the original's next single advection pass
takes live pigment from 12,073.4237619 to 12,273.0595021, a 1.65351390% gain.
The port takes 12,073.4123037 to 12,273.1854005, a 1.65465315% gain. The bounded
original diagnostic completed in 12.55 seconds; it did not rerun full drying.

## Capture status

The original full-size software mark was stopped at 300.2 seconds before its
second stage completed. Its pre-fix DOM screenshot is not comparison evidence.
The corrected runner disables RAF physics and exports an explicit 1056×704
canvas bitmap, excluding DOM borders and overlays. Hardware-original captures
and the other six native marks are coordinated by the parent.

The authoritative native dry-stroke run completed in 186.958 seconds. All four
PNGs were checked at exactly 1056×704. Stage 2 still has source sheen on 91.962%
of its painted footprint, so it cannot be asserted to be satin without the
hardware reference. Stage 3 has zero surface water and 99.605% damp footprint;
stage 4 has 100% cured footprint and max flow.a 0.00133324. Neutral Stage 1–4
labels remain honest when a wetness name is unverified.

The full native dry sequence ends with live pigment 2.55180365. That is not
the full material inventory: dry-fold converts pigment into the dried optical
reflectance field. No dry-fold inventory claim is inferred from that number.

See `README.md` for exact commands. Raw artifacts: `original-parity.json`,
`parity.json`, `normal-original-drift.json`, `parity-summary.md` and
`native/dry-stroke-{timing,drift}.json` in the scratch captures directory.
