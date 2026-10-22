# Accepted-mixer regression baseline

These six unchanged engine-3 logs exercise the accepted round-5 mixer through the universal paint path. Engine headers retain their original handling behavior. Alice authorized refreshing old-mixer expectations; the mixer and all fit/table bytes remain fixed.

The 27 dumps are field-transplanted expectations. Color, current-width latents, pile colors, producing identity and color-derived scattering come from the fresh replay. All other physics bytes, including stiffness, drying, volumes, rag loads, floors and clocks, remain frozen from the preceding package. MANIFEST.md gives hashes and exact commands; proof/refresh-field-audit.json records the preservation audit.

| Path | Contents |
|---|---|
| scenes/ | Six unchanged 128 × 64 engine-3 scene inputs |
| state/ | All 27 compressed state expectations |
| ref/ | Fresh PNGs, printed output, timings and reconstructed canvas digests |
| proof/ | Actual replay digests/times with dumping, without dumping and with one thread, plus the field audit |
| tools/ | State comparison, checkpoint reconstruction, safe mixer-field refresh and package verification |
| old_files/ | Unchanged genuine historical compatibility inputs |
| SHA256SUMS | Every intended package file except the checksum list itself |

The scenes are stroke (3 chunks), body (2), overlap (3), pickup (4), rag (8) and wait (7). All use the existing 200 mm canvas, aspect 2, seed 21 and 80 µm lead-white/ochre ground. No input, recipe or tolerance changes.

The existing finite field limits, exact integer/shape checks and nonfinite rejection are unchanged. Current images retain maximum decoded RGBA error 2/255. Main's known rag physical mismatch remains diagnosed by its unchanged scalar signature and unrelated-state checks; it is not re-recorded as success. Its current-mixer PNG passes and its latent width is now checked against the stored wet.lat shape.

CPSTATE1 dumps retain the producing mixer identity. tools/checkpoint_from_dump.py reconstructs PAINTC19 canvas checkpoints and verifies their recorded canvas digests. Old PAINTCK8 files are not regenerated. No old dump is merely relabeled with the current identity.

```sh
notes/thinner/baseline/tools/verify_package.sh
scripts/tests/baseline_state.sh target/debug/easel
```
