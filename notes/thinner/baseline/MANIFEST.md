# MANIFEST: how the baseline was made

Made 2026-10-04 02:28 to 02:45 BST by the speed agent (branch `codex/speed`,
working copy `~/src/a/claude-paint-speed`) under the overnight plan whose
SHA-256 is `382fc2debb7a56505f9b4723c6591f22644feccb330fef62dc723b635f098e30`.
Every heavy command ran as one `scripts/lockrun` job at a time.

## Code

- **Reference results** (`ref/`): engine-3 code at
  `af49348239421791509f7b7c36e9dc39305b26fe`, unchanged. The working copy
  was af49348 plus the `scripts/lockrun` commits (`c362a1f`, `8b71762`),
  which touch no file under `crates/`, `Cargo.*` or `.cargo/`
  (`git diff af49348 8b71762 --stat`: `scripts/lockrun`,
  `scripts/tests/lockrun.sh` only).
- **State dumps** (`state/`, `proof/`, `old_files/paintck8_save_stroke.ckpt.xz`):
  the same code plus the dumper, which is in this baseline's commit: the new
  files `crates/paint/src/state_dump.rs` (sha256 `59688af0…b816`) and
  `crates/easel/src/state_dump.rs` (`68a8500c…2720`), `pub mod state_dump;` in
  `crates/paint/src/lib.rs`, and in `crates/easel/src/main.rs` the
  `mod state_dump;` line, the `--dump-state` flag and two calls in `run`.
  Nothing else changed (`git diff af49348 -- crates Cargo.lock Cargo.toml .cargo`:
  11 insertions, 1 deletion in main.rs and lib.rs, plus the two new files).

## Toolchain and build

| | |
|---|---|
| rustc | 1.97.1 (8bab26f4f 2026-07-14), LLVM 22.1.6, host aarch64-apple-darwin |
| cargo | 1.97.1 (c980f4866 2026-06-30) |
| profile | `release`: opt-level 3, debug false, incremental false, codegen-units 1 (Cargo.toml) |
| build settings | `.cargo/config.toml` `[env] CFLAGS = "-Dluai_makeseed()=0x5eedu"` (Lua's fixed hash seed); `~/.cargo/config.toml` sets `rustc-wrapper = "sccache"` (sccache 0.17.0, a compile cache) |
| features | easel defaults (`replay`, `finish`; `replay` turns on `all-boxes` and `paint/hand-hook`) |
| Cargo.lock | sha256 `568ddfcc23523c2c6c97fb54e0b1c79eb0609d5df2767bbc16456df0700de016` (unchanged from af49348) |
| machine | Apple M3 Pro, 12 cores (6 performance, 6 efficiency), macOS 27.0.1 (26A434) |
| threads | rayon default (12) for `ref/` and `state/`; `RAYON_NUM_THREADS=1` for `proof/digest_t1` |
| other tools | Python 3.13.15 (standard library only), xz 5.8.4 (`xz -9e -T1`), GNU bash 5.3.15 |

| build | command | time | binary sha256 |
|---|---|---|---|
| af49348, empty target dir (55 crates; sccache may have served cached dependencies, unverified) | `/usr/bin/time -p scripts/lockrun --timeout 600 --owner speed:initial-release-build --log notes/speed/logs/b_release_build.log -- cargo build --release -p easel` | 43.85 s (cargo), 44.04 s wall | `4201cec1f9ddcc8bcd48106ab5b36a3e337bb9619f138f384eb0e81b1ae040f6` |
| with the dumper (paint and easel rebuilt) | `/usr/bin/time -p scripts/lockrun --timeout 600 --lock-timeout 5 --owner speed:dumper-release-build --log notes/speed/logs/c_dumper_release_build.log --quiet -- cargo build --release -p easel` | 43.11 s (cargo), 43.25 s wall | `a5f3775843fa006a36417d3cfb315746bada0d2ffd8746279105a12c3b63b009` |

The first binary was copied to `$TMPDIR/cp-speed/easel-af49348` before the
dumper was added. The second was copied to `$TMPDIR/cp-speed/easel-dumper`.

## Canvas

Every scene: 128×64 px (`easel run --width 128`, `canvas{aspect=2}`),
1000×500 canvas units, `size=200` mm (0.2 mm a unit, 1.5625 mm a pixel),
`--@ box inness`, `--@ engine 3`.

## Commands

One lockrun job (1.8 s wall; `W=$TMPDIR/cp-speed/runs`, `T=notes/thinner/baseline/tools/run_scenes.sh`):

```sh
/usr/bin/time -p scripts/lockrun --timeout 60 --lock-timeout 5 --owner speed:baseline-scenes --log notes/speed/logs/c_baseline_runs.log --quiet -- bash -c "set -e;
  $T $TMPDIR/cp-speed/easel-af49348 $W/ref;
  $T $TMPDIR/cp-speed/easel-dumper $W/dump --dump;
  $T $TMPDIR/cp-speed/easel-dumper $W/nodump;
  RAYON_NUM_THREADS=1 $T $TMPDIR/cp-speed/easel-dumper $W/t1 --dump"
```

Then, without the engine:

- Each scene in `dump`, `nodump` and `t1` against `ref`: digest lines
  without `secs=` (`cmp`), PNG bytes (`cmp`), printed output (`cmp`), all
  equal; each of the 27 dumps in `t1` against `dump` byte for byte, all equal.
- Each dump's embedded `digest` against `ref/digest`'s line for its chunk:
  27 of 27 equal.
- `tools/checkpoint_from_dump.py <dump> --digest ref/digest/<scene>.txt`
  for all 27 dumps: every rebuilt PAINTCK8 checkpoint's FNV-1a equals
  af49348's `canvas=` digest.
- `tools/compare_build.sh $TMPDIR/cp-speed/easel-dumper --added-zero` (under
  lockrun, 1.0 s): all six scenes equal on every field, with the same PNG,
  output and digests.
- `ref/` = `$W/ref/{png,digest,out,times.tsv}`; `state/<scene>/` =
  `xz -9e -T1` of `$W/dump/state/<scene>/chunk-*.state`; `proof/` =
  `$W/{dump,nodump,t1}/digest` and `times.tsv`.
- `old_files/paintck8_save_stroke.ckpt.xz` = `xz -9e -T1` of
  `$W/dump/state/stroke/final.ckpt`, written by `save::write` (unchanged
  af49348 code) from the state shown equal above.
  `old_files/paintck8_save_header.bin` is its first 280 bytes: `PAINTCK8`,
  the header length and the `easel save 1` header (box inness, engine 3,
  width 128).
- `old_files/engine3_log_*.lua`: af49348's live session (the unchanged
  binary, `EASEL_ROOT` a temporary directory, no canvas made): `easel open
  old; easel -s old do 'x = 1'; easel -s old close`, and the same with
  `EASEL_BOX=inness` for session `oldbox`.
- A painter build check after adding the dumper: `cargo check --release -p
  easel --no-default-features` and `... --features box-inness` (6.4 s, no
  warnings; `notes/speed/logs/c_check_no_default.log`).

## Times (wall seconds per scene, one run each)

| scene | af49348 (`ref`) | dumper, no dump | dumper, dumping | dumping, 1 thread |
|---|---|---|---|---|
| body | 0.102 | 0.097 | 0.347 | 0.120 |
| overlap | 0.021 | 0.020 | 0.031 | 0.041 |
| pickup | 0.020 | 0.019 | 0.031 | 0.041 |
| rag | 0.030 | 0.030 | 0.046 | 0.055 |
| stroke | 0.018 | 0.018 | 0.027 | 0.037 |
| wait | 0.027 | 0.026 | 0.041 | 0.051 |

## Files

`SHA256SUMS` lists every file in this directory with its SHA-256
(`shasum -a 256 -c SHA256SUMS` from here). The whole package is about 4 MB.
