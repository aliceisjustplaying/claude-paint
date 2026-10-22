# Accepted round-5 mixer baseline

Alice accepted the round-5 mixer and authorized refreshing its rendered and mixer-dependent captures. The mixer, its fit, all tables, scene inputs and numeric tolerances are unchanged.

## Producing build

The producer is a frozen test-profile easel whose mixer and tables are byte-identical to the accepted round-5 source (ec2a760, also unchanged in a6a597f). Its SHA-256 is `c2d80a62fb8007c63091320731ec0108957e8a5cc8969cbd4c013037cf9292cf`. Recording ran on Linux x86_64 with four Rayon threads and a one-thread control. No host was deployed.

The mixer identity is `17375460472093594666`. It covers the decoder schema, measured coefficients/constants, reference substrate and inverse:
- measured.bin: `5be1924f293ee0fcf0517b5575f3b6275c5ef36b046543edfc11341664e95e15`
- reference.bin: `f9920b80d2b2df1e9286a75d3ab5868d4177883cb23b2ca58339d0a974554211`
- inverse.bin: `9c5688169627e3b07b5593ee5dd644fdebe672e3d5305ce1384e0244d6ae217e`

## Frozen physics and refreshed optics

The reference before this refresh is the package at a6a597f, retaining its earlier main-physics expectations. All 27 stored dumps retain every non-mixer field byte-for-byte from that reference. The refreshed fields are color, 21-channel wet latents, pigment pile colors, bristle/knife latents and the producing mixer identity.

Scattering is also refreshed: channel 0 of wet.hide and tool hide arrays is derived from a tube's masstone and its catalog hiding. Measured masstones change it. Channels 1 and 2 (stiffness and drying) and every remaining tool property stay frozen. Rag loads, wet volumes, floors, clocks, geometry and all unrelated debug values are not re-recorded.

`tools/refresh_mixer.py` performs this projection and asserts the preserved fields. `proof/refresh-field-audit.json` records all 27 reference/capture hashes and preservation results. These are deliberately mixed-source expectations, not untouched replay dumps. Their canvas hashes are reconstructed with `checkpoint_from_dump.py`; tool/studio digests and proof images describe the real fresh replay.

Main's existing rag mismatch remains a registered known failure, with exactly the same scalar signature and bounds. Its current image passes; its physical state is not silently blessed. The classifier now derives latent width from the stored wet.lat shape rather than assuming seven channels.

## Recording and verification

Run the unchanged six scenes at width 128:

```sh
notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/dump" --dump
notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/nodump"
RAYON_NUM_THREADS=1 notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/t1" --dump
uv run --no-project python notes/thinner/baseline/tools/refresh_mixer.py \
  --reference "$FROZEN_STATE" --capture "$CAPTURE/dump/state" \
  --package notes/thinner/baseline
```

The three image sets match byte-for-byte, printed output matches and one/four-thread dumps meet the unchanged comparer. ref/ contains fresh images, printed output, times and reconstructed canvas digests. proof/ contains actual replay digests and timings. SHA256SUMS covers every package file except itself; interpreter caches are excluded from the package, not the inventory check.

The authorized recorders also refresh the paint fixture, the three engine-2 drying-sheet views, all twelve old-log image/digest pairs, the default-box image and the two tube tables. The legacy tube table preserves its tube ordering and all non-color/non-strength bytes; only mixer-dependent colors and scalar weights change. The expanded current table is recorded in full. No assertion, scene recipe or threshold is weakened by refreshing these expected data.

Historical files in old_files/ remain unchanged compatibility inputs. The checkpoint reader now consumes their original seven-channel payload width, rejects occupied obsolete paint and imports empty historical canvases without stale latents.

```sh
notes/thinner/baseline/tools/verify_package.sh
scripts/tests/baseline_state.sh "$EASEL"
scripts/thinner_check2 "$EASEL"
```

The last two retain the existing exit-4 known-rag classification when its exact scalar signature is the only state mismatch; the full gate recognizes that marker. It is not reported as a raw state pass.
