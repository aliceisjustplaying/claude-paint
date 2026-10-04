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
checks every field and value the baseline has, bit for bit, and lists what
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
- Release profile only (`codegen-units = 1`, not incremental). The iter
  profile can move the last digits.
