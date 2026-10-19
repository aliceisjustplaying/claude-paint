# Small before-change results (engine 3, af49348)

Six tiny engine-3 scenes, replayed by the unchanged engine-3 code
(`af49348239421791509f7b7c36e9dc39305b26fe`) in a release build. Saved: their
inputs, their pictures, and every state value the session holds after every
chunk. The thinner work compares against these files. It must not make its
own expected answers from the code it is testing (overnight plan, "First,
save small before-change results"). `MANIFEST.md` records how they were made.

## What is here

| path | what |
|---|---|
| `scenes/*.lua` | the inputs: easel logs (`--@ box inness`, `--@ engine 3`), replayed at 128 px wide (`easel run --width 128`), 128×64 px |
| `ref/png/<scene>.png` | the picture af49348's unchanged easel delivered (8-bit sRGB, wet paint as laid) |
| `ref/digest/<scene>.txt` | af49348's `--state-digest` lines, one per chunk: FNV-1a-64 of the canvas checkpoint, the brushes and rags, and the studio (main.rs `state_digest_line`) |
| `ref/out/<scene>.txt` | what the chunks printed (brush fullness, rag load and dampness, times of day, `drying()`) |
| `ref/times.tsv` | wall seconds per scene (unchanged easel) |
| `state/<scene>/chunk-NNN.state.xz` | the full state after chunk NNN (format below), xz-compressed |
| `proof/` | digests and times of the dumping easel: with `--dump-state` (`digest_dump`), without it (`digest_nodump`) and with one rayon thread (`digest_t1`) |
| `old_files/` | samples for "the new program refuses old files": a genuine PAINTCK8 easel save of the `stroke` scene (`paintck8_save_stroke.ckpt.xz`), its first 280 bytes alone (magic, length and header: `paintck8_save_header.bin`), and two engine-3 logs written by af49348's live session (`engine3_log_default_box.lua`, `engine3_log_inness.lua`). The scene logs are af49348 engine-3 logs too |
| `tools/` | `run_scenes.sh` (replay the scenes), `state_compare.py` (compare dumps), `compare_build.sh` (both, against this baseline), `checkpoint_from_dump.py` and `verify_package.sh` (check this package), `test_state_compare.py` (the comparer's own checks) |
| `SHA256SUMS` | every file's SHA-256 |

The scenes:

| scene | chunks | what it does |
|---|---|---|
| `stroke` | 3 | a raw sienna stroke from a filbert 60 at load 0.6, then a second stroke from what is left on the brush |
| `body` | 2 | one `work{hand="body"}` pass of raw sienna and lead white over a 400×260-unit rectangle |
| `overlap` | 3 | a lead white stroke, a second pass over it, then a raw umber flat across both, wet into wet |
| `pickup` | 4 | a bone black stroke; a clean flat drawn through it (picks paint up); that flat on clean ground (lays it down); `b:wipe(0.85)` |
| `rag` | 8 | a raw umber control left 5 days to dry; four raw sienna patches; a dry-cloth wipe, a blot, a spirits-damp wipe (`dip(0.6)`), a refold and a hard wipe over the dry control, a damp wipe over a mask, a brush stroke across the wiped areas |
| `wait` | 7 | lead white and raw sienna (medium 0.2) strokes; `wait(15)`; fifteen `wait(1)`; `wait(7.3)` then `wait(7.7)`; a day; four days (`drying()` printed along the way) |

All use the same canvas: `size=200` mm, `aspect=2`, linen 15, seed 21, a
knifed lead-white/yellow-ochre ground 80 µm thick.

## State dump format (CPSTATE1)

`easel run <log> --dump-state <dir>` writes `<dir>/chunk-NNN.state` after
each chunk and `<dir>/final.ckpt` (the session's save, as `easel close`
writes it) after the last. The code is in two new files,
`crates/paint/src/state_dump.rs` (the canvas's fields) and
`crates/easel/src/state_dump.rs` (the file, brushes, rags and studio). The
hook in `main.rs` adds the flag and two calls; `lib.rs` gets a `mod` line.

```
8 bytes   "CPSTATE1"
u64 LE    header length
header    JSON: {"format", "chunk", "digest", "fields": [{"name", "dtype", "shape", "offset", "nbytes"}, ...]}
data      each field's values, little-endian, row major (offsets count from the end of the header)
```

`dtype` is `f32`, `f64`, `u32`, `u64` or `utf8`. `digest` is that chunk's
`--state-digest` line. The fields:

- Canvas, per pixel (`[h, w]` or `[h, w, k]`): `color` (linear RGB of the
  dry picture), `height` (surface µm), `film` (coats), `wet.vol`, `wet.lat`
  (pigment mix, 7 latent values), `wet.hide` (scattering, stiffness, drying
  rate), `wet.stroke`, `wet.touched`, `wet.floor`, `wet.cover`, and the
  drying state `clock.cure`, `clock.lev`, `clock.seen`, `clock.sub`,
  `clock.srate`, `clock.th` (shape `[0]` before the first wait).
- Canvas scalars: `frame` (w, h, x0, y0, full_w, full_h and the kept box),
  `scale`, `mm_per_unit`, `linen` (warp, weft, crown, slubs, present),
  `linen_seed`, `surf_gen`, `engine`, `ground_um`, `wet.current`,
  `wet.dirty`, `clock.now` (minutes, f64), `clock.mark`, `clock.tacky`,
  `drawing`, `hand_slice`, `tally`.
- Studio: `studio.seed`, `studio.chunk`, `studio.calls`, `studio.clock`,
  `studio.clock0` (f64) and `studio.piles` (`[n, 3]`).
- Text: `brushes.debug` (each held brush's `Debug`, one a line: the tool and
  every bristle's position, bend, load `vol`, pigment mix `lat`, `hide` and
  `cure`), `rags.debug` (each rag's width, load, soaked, damp, wet_at, fold
  and seed) and `studio.debug` (the same text `--state-digest` hashes).
  Rust's `Debug` prints a float in its shortest round-trip form, so the text
  holds the exact value. `state_compare.py` parses it into named values such
  as `brushes[0].bristles[3].vol` and `rags[1].damp`.

Everything the PAINTCK8 checkpoint stores is in the dump, plus `wet.floor`,
which the checkpoint leaves out. `checkpoint_from_dump.py` rebuilds the
checkpoint bytes from a dump, and their FNV-1a equals af49348's `canvas=`
digest for all 27 chunks. Not in the dump, as in the digests: the Lua
globals, the style, the world view and derived caches.

**A later engine that adds state** (solvent per pixel, in the brushes or
rags, more clock fields) adds fields under new names, or new keys in a
struct's `Debug`. It keeps the existing names and meanings. A comparison
checks every field and value the baseline has within numeric tolerance, and lists what
only the new dump has. `--added-zero` requires the additions to be zero
(numbers 0, `false`, `None`).

## Compare a build with this baseline

Build a release easel with `--dump-state` (any branch that has this
commit's dumper), then run, as one heavy job:

```sh
cargo build --release -p easel
scripts/lockrun --timeout 60 -- notes/thinner/baseline/tools/compare_build.sh target/release/easel --added-zero
```

It replays the six scenes into a temporary directory and fails unless every
baseline field of every chunk is equal. It also reports whether the PNG,
the printed output and the digests are af49348's. A new checkpoint format
changes the `canvas=` digest by itself, so only the field comparison and
the PNG decide. To compare two dump directories or files directly:

```sh
notes/thinner/baseline/tools/state_compare.py notes/thinner/baseline/state/rag <new>/state/rag [--added-zero]
```

To check the package itself (hashes, the comparer's own checks, the 27
checkpoints rebuilt): `notes/thinner/baseline/tools/verify_package.sh`.

## How the dumper was shown not to change results

The dumper only reads. On these scenes the binary with the dumper gives
the same results with and without `--dump-state` and with one rayon thread
or the default 12. Each time, the digest lines (secs aside), PNG bytes and
printed output equal those of af49348's unchanged binary, built before the
dumper was added. The dumps from 1 and 12 threads are identical byte for
byte. `MANIFEST.md` lists the commands and binary hashes.

## Notes

- 128 px is a development width: the brushes' widths are in canvas units
  (1000 across), so a `body` pass's filbert 9 is about 1 px here. These
  scenes catch regressions in the state. They are not pictures to judge
  paint by.
- The `secs=` field of a digest line is wall time. Drop it before comparing
  (`sed 's/ secs=[^ ]*//'`).
- These reference measurements used the release profile. All build profiles
  now use the same bounded image acceptance policy in notes/workflow.md.

## Numeric state comparison

`scripts/tests/baseline_state.sh` uses finite numeric tolerances on every
platform. `--linux-tolerance` remains a compatibility alias, not an opt-in.
Rendered PNGs use maximum decoded RGBA channel error at most 2/255,
regardless of build or platform. Only macOS arm64 and Ubuntu x86_64 with
Rust 1.97.1 supplied the historical measurements below.

The numeric limits below were measured against all 27 stored chunks using
the release easel at `908d0db`, the committed lockfile and Rust 1.97.1 on
Ubuntu 24.04 x86_64. They are absolute limits, with no relative allowance
that grows with a field's magnitude. Other finite floats use the small
default absolute tolerance documented by `state_compare.py`; signed zeros
are equivalent and nonfinite values are rejected. Every integer, dtype,
shape, missing value and enum remains exact. Added state still must satisfy
`--added-zero` without any tolerance. The comparer prints the
maximum allowed and observed absolute error for each bounded field,
including on failure.

The small-error measurements used to set the bounds include all five
non-rag scenes and rag chunks 1–3 (before the first wipe). Rag's later
semantic differences are measured separately below and are rejected.

| Field | Largest small error observed | Absolute limit | Units |
|---|---:|---:|---|
| `color` | 4.27663326e-6 | 5e-6 | linear RGB channel (0–1) |
| `height` | 1.220703125e-4 | 1.3e-4 | µm |
| `film` | 5.24520874e-6 | 6e-6 | coats |
| `wet.vol`, `clock.seen` | 7.15255737e-6 | 8e-6 | coats |
| `wet.floor` | 7.39097595e-6 | 8e-6 | coats |
| `wet.lat` | 1.49011612e-7 | 1.6e-7 | pigment latent coordinate |
| `wet.hide` | 4.76837158e-7 | 5e-7 | scattering, stiffness or relative drying rate |
| `clock.cure` | 3.72529030e-7 | 4e-7 | oxidation progress (1 = touch-dry) |
| `clock.lev` | 6.103515625e-5 | 7e-5 | seconds |
| `clock.th` | 5.72204590e-6 | 6e-6 | coats |
| `clock.sub` | 1.52587891e-5 | 1.6e-5 | set-film oxidation progress |
| `clock.srate` | 2.09547579e-9 | 2.2e-9 | oxidation progress per minute |
| bristle `vol` | 1.00000000e-3 | 1.1e-3 | coats × canvas-unit² |
| bristle `rx` | 6.0e-8 | 6.1e-8 | normalized root offset |
| bristle `ry` | 2.0e-8 | 2.1e-8 | normalized root offset |
| bristle `thresh` | 3.0e-8 | 3.1e-8 | pressure (0–1) |
| bristle `bend` | 1.0e-7 | 1.1e-7 | canvas units |
| bristle `lat` | 1.0e-7 | 1.1e-7 | pigment latent coordinate |
| bristle `hide` | 3.0e-7 | 3.1e-7 | paint property |
| bristle `cure` | 6.0e-12 | 6.1e-12 | oxidation progress |

A coat is 25 µm, so the largest allowed wet-volume/floor error is
0.0002 µm of paint, and the height allowance is 0.00013 µm. At these
scenes' 0.2 mm per canvas unit, the bristle-volume allowance of 1.1e-3
coats × canvas-unit² is 1.1e-6 mm³. The leveling allowance is 70
microseconds. These limits admit the
measured rounding without admitting meaningful changes in film,
drying time or pigment mix.

Full-scene maximum absolute differences (including rag's failures):

| Scene | Chunks | Color | Height (µm) | Film (coats) | Wet volume (coats) | PNG changed pixels / 8192 | PNG max channel delta |
|---|---:|---:|---:|---:|---:|---:|---:|
| body | 2 | 1.78813934e-7 | 9.15527344e-5 | 3.09944153e-6 | 3.57627869e-7 | 1 | 1/255 |
| overlap | 3 | 1.78813934e-7 | 9.15527344e-5 | 3.09944153e-6 | 2.95042992e-6 | 1 | 1/255 |
| pickup | 4 | 1.78813934e-7 | 9.15527344e-5 | 3.09944153e-6 | 1.90734863e-6 | 1 | 1/255 |
| rag | 8 | 1.78813934e-6 | 1.220703125e-4 | 5.24520874e-6 | 7.03836143 | 707 | 109/255 |
| stroke | 3 | 1.78813934e-7 | 9.15527344e-5 | 3.09944153e-6 | 1.74343586e-6 | 1 | 1/255 |
| wait | 7 | 4.27663326e-6 | 1.220703125e-4 | 4.76837158e-6 | 3.09944153e-6 | 1 | 1/255 |

Five scenes pass the numeric limits. Rag fails at chunk 4, the first dry
wipe: `wet.vol[1310]` is 0.9863235950469971 in the baseline and
3.949877977371216 on Linux, while `rags[0].load` changes from 0.78475475
to 0.5218438. All binary integer fields and existing Debug integers/enums
still match. An independent macOS arm64 replay of the same main code
also gives load 0.5218438 and wet volume 3.9498796463012695 at that index,
confirming a shared historical-baseline mismatch. Current `rag.rs` has
engine-3 cloth, smear and wipe branches
absent from af49348, and the new nonzero mobile rag state also fails
`--added-zero`. The 707 changed pixels and 109/255 channel difference
are material; neither the numeric limits nor a platform golden accepts
them. This is tracked explicitly as a known historical failure, without
changing physics or re-recording the stored baseline.

`baseline_state.sh` checks all six scenes even after a failure and prints
the full mismatch. It exits 4 with the single marker
`KNOWN_FAILURE: rag-historical-baseline-main` only when the other five
scenes pass both their state and image criteria and the rag matches
`scripts/known_rag_baseline.py`'s diagnostic signature. Any other failure
exits 1. This is a known failure, never a baseline pass.

The shared diagnostic interface is:

```sh
python3 scripts/known_rag_baseline.py <rag-dump-directory> --image-report <compare_images-stdout-file>
```

Exit 0 means the known diagnostic matches; 1 means a different/unrelated
failure and 2 means malformed input. The helper emits no known marker.
It checks eight scalars across the dry wipe (chunk 4), damp wipe (chunk
6) and refold/brush operation (chunk 8), with narrow rounding bands. All
pre-wipe chunks and unaffected state fields still use the ordinary bounded
numeric comparison. Only the identified wet-array, carried-rag
paint and final-brush mismatch categories may differ; dtype/shape,
integers, nonfinite changes and nonzero added solvent are rejected.
The image report must be the measured Linux and macOS result (707/8192
changed pixels, max 109/255), including the tolerance provenance prefix.
State-only callers may omit
`--image-report`; historical image checks must supply it.

Targeted checks:

```sh
python3 scripts/tests/baseline_state_compare_test.py
python3 scripts/tests/known_rag_baseline_test.py
python3 notes/thinner/baseline/tools/test_state_compare.py
```
