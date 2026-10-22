# Current-mixer regression baseline

These six small engine-3 logs were captured for STIL-219's current
independent-mixer candidate. Only the mixer fields come from that
capture: color, wet.lat, bristle latents and studio piles.
All other existing state values come from main `d94eafd60`, including
the rag physics that the original capture inadvertently replaced.
The engine headers and
scene inputs are unchanged. They select the older handling behavior, not
the retired pigment implementation. These are current-mixer expectations,
not a claim that the old library's output is preserved. `MANIFEST.md`
records the producing build and commands.

## Contents

| Path | Contents |
|---|---|
| `scenes/*.lua` | Six unchanged engine-3/inness logs, replayed at width 128 (128×64 pixels) |
| `ref/png/` | Current-mixer rendered images and the producing platform |
| `ref/digest/` | Reconstructed canvas digests for the mixed-source expectations; tool and studio digests from the capture |
| `ref/out/` | Printed output from each scene |
| `ref/times.tsv` | Wall seconds per scene, measured with state dumping enabled |
| `state/<scene>/chunk-NNN.state.xz` | All 27 CPSTATE1 state dumps, compressed with `xz -9e -T1` |
| `proof/` | Digests and wall times with dumping, without dumping and with one Rayon thread |
| `old_files/` | Historical PAINTCK8 save/header and original engine-3 logs retained as old-file compatibility inputs, not current expectations |
| `tools/` | Replay, state comparison and checkpoint reconstruction tools |
| `SHA256SUMS` | Every package file except the checksum list itself |

| Scene | Chunks | Operations |
|---|---:|---|
| `stroke` | 3 | A raw sienna filbert stroke and a second stroke from the remaining load |
| `body` | 2 | A body pass of raw sienna and lead white |
| `overlap` | 3 | Two lead-white passes and a wet raw-umber stroke |
| `pickup` | 4 | Bone black, clean-brush pickup, redeposition and wiping |
| `rag` | 8 | Dry and damp wiping, blotting, refolding, masking and a final brush stroke |
| `wait` | 7 | Mixed paint followed by short and multiday drying intervals |

All scenes use a 200 mm canvas, aspect 2, linen 15, seed 21 and an
80 µm lead-white/yellow-ochre ground. The small width is for state
regressions, not a visual assessment of paint.

## State and checkpoint format

`easel run <log> --dump-state <dir>` writes one `chunk-NNN.state` after
each chunk and a final session save. CPSTATE1 contains an eight-byte magic,
a little-endian u64 JSON-header length, the JSON header and field bytes.
The header records each field's name, dtype, shape, offset and length.
Fields include the canvas, wet paint, drying clock, solvent, held tools
and studio values. Lua globals and derived caches are not represented.

`tools/checkpoint_from_dump.py` reconstructs the engine-3 PAINTC19 canvas
checkpoint with base edition 9 and mixer identity 1, including solvent.
`tools/verify_package.sh` requires its FNV-1a to match the recorded
`canvas=` digest for every one of the 27 chunks. The historical PAINTCK8
files in `old_files/` are deliberately not regenerated.

## Acceptance and proof

The existing finite numeric field limits in `tools/state_compare.py`
are unchanged. Integers, shapes, dtypes, missing fields and enum values
remain exact. Nonfinite values fail. `--added-zero` requires added state
to be zero. Rendered images retain the maximum decoded RGBA channel
error of 2/255 documented in `notes/workflow.md`. Re-recording changes
the expected data, not these comparisons.

All six scenes are checked. Main's known rag physics mismatch remains a
known failure: only its measured scalar signature with no unrelated
state failure receives exit 4 and `KNOWN_FAILURE: rag-historical-baseline-main`.
Its PNG must pass against the current-mixer image capture. The historical
707-pixel image-failure signature is not allowed against this new image.
Other image failures and state regressions still fail with exit 1.

The original capture checks that PNG bytes, printed output and digests with
`secs=` removed match across dumping, no dumping and a one-thread
dumping replay. The one-thread dumps also pass the unchanged state
comparison against the four-thread dumps. `proof/` keeps those original
capture digests, not a claim that the mixed-source expectations replay.
Only the canvas hashes in `ref/digest/` and dump headers are recomputed
from the expectations. Tool and studio hashes remain capture diagnostics.
`secs=` is diagnostic wall time and is not an equality criterion.

From the repository root, using the producing test-profile easel:

```sh
notes/thinner/baseline/tools/verify_package.sh
scripts/tests/baseline_state.sh target/debug/easel
```

The full gate builds its own test-profile executable and checks the same
recorded fields and images. The original source commit and full commands
for this recording are in `MANIFEST.md`.
