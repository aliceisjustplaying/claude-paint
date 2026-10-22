# MANIFEST: STIL-212 current-mixer baseline

Recorded on 2026-10-22 on the approved cloud verification machine. These
active expectations were explicitly authorized for the independent mixer.
They are not historical Mixbox/af49348 results. Scene inputs and tolerance
code were not changed by the capture.

## Producing build

| Item | Value |
|---|---|
| Source | `d9b1d7ded2ae87480d9d58ef2872c2c2675b3734` |
| Easel SHA-256 | `bab36684b9d9dfeb3eb79adc36bd166bf315922c65691da2bd6989eaf8140eb3` |
| Cargo.lock SHA-256 | `8511bd710df7e47ca3f1c8754de74c89dc5f5d74ce0c2e7d1fa3359eddf44cb8` |
| rustc | `1.99.0 (b940084d7 2026-09-28)`, LLVM 23.1.1 |
| cargo | `1.99.0 (5f94df478 2026-08-27)` |
| Host/platform | `x86_64-unknown-linux-gnu`, `linux-x86_64`, Ubuntu 24.04 |
| CPU | AMD EPYC, 16 logical CPUs available, affinity restricted to IDs 0–3 |
| Profile | Workspace `test`: opt-level 2, codegen-units 256, incremental, debug assertions and overflow checks enabled; `profile.test.debug=0` |
| Features | Workspace defaults |
| Threads | `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=4`, `RAYON_NUM_THREADS=4`; only the explicit one-thread proof overrides Rayon to 1 |
| Other tools | Python 3.12.3 via uv 0.13.4, GNU bash 5.2.21, xz 5.4.5 |
| Canvas checkpoint | PAINTC19 envelope, base edition 9, mixer identity 1 |

The working checkout was clean when the producing build started. All
commands below inherited `taskset -c 0,1,2,3` and the three thread/job
variables above. The same settings were used for the main benchmark.

```sh
cargo test --workspace --no-run --config profile.test.debug=0 --lib --bin easel \
  --test smoke --test look --test session_integrity --test live_frames \
  --test live_replay_checkpoint --test replay_frames_resume --test painter_signals \
  --test studio_profiles --test checkpoint_paths --test save_confined --test ck7 \
  --test thinner_physics --test thinner_pigments --test engine9_plumbing \
  --message-format=json-render-diagnostics
```

## Record commands

`EASEL` below is the absolute path to `target/debug/easel` from that build.
`CAPTURE` is a scratch directory outside the package. No release-profile
or separately rebuilt executable supplied these expectations.

```sh
notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/dump" --dump
notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/nodump"
RAYON_NUM_THREADS=1 notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/t1" --dump
```

Each run uses the unchanged six `scenes/*.lua` inputs at width 128.
The dump run supplies `ref/png`, `ref/digest`, `ref/out` and
`ref/times.tsv`. Each `dump/state/<scene>/chunk-NNN.state` supplies its
existing `state/<scene>/chunk-NNN.state.xz` through `xz -9e -T1 -c`.
The three runs supply `proof/digest_{dump,nodump,t1}` and
`proof/times_{dump,nodump,t1}.tsv`. `ref/png/platform.txt` is the measured
output of `uv run --script scripts/compare_images.py --print-platform`.

Before copying, each scene's PNG bytes and printed output were compared
with `cmp` across all three runs. Digest lines were compared with
`cmp` after `sed 's/ secs=[^ ]*//'`. Four-thread and one-thread dumps
were compared using the unchanged `state_compare.py --quiet --added-zero`.
Package verification reconstructs all 27 canvas checkpoints and checks
their FNV-1a against `ref/digest` without an exception.

## Other active fixtures recorded with the same build

The producing Rust test executables were selected from Cargo's artifact
stream. From their crate directories, with `UPDATE_GOLDEN=1`, the exact
test names were:

```text
tests::fixture_matches_golden
bristle::kernel_traps::ploughing_films_at_the_threshold_leaves_the_recorded_cure
session::tests::engine_2_logs_replay_as_before
```

The first two record `crates/paint/tests/golden_scene.png` (and platform)
and `plough_volume_cure.png`. The session test records all three
`crates/easel/tests/engine2/drying_sheet_{normal,gallery,relief}.png` views.
The same build also supplied:

```sh
scripts/tests/old_logs.sh "$EASEL" --record crates/easel/tests/old_logs/golden
"$EASEL" run crates/easel/tests/r19_default_box.lua --out crates/easel/tests/r19_default_box.png
uv run --script scripts/compare_images.py --print-platform > crates/easel/tests/r19_default_box.platform
```

The separate r19 image uses its default width 2400. The old-log recorder
uses each unchanged case's width from `cases.tsv`, including r19 at 320.

## Historical files and checksums

`old_files/` is retained unchanged: its PAINTCK8 save/header and original
logs remain historical compatibility inputs. They are not the producer
or expected state of the active regression baseline.

`SHA256SUMS` is refreshed after recording and documentation updates with
all package files except itself. `tools/verify_package.sh` checks this
complete inventory, the state comparer's tests and all 27 reconstructed
checkpoint digests. Raw execution logs and intermediate uncompressed
captures remain in the verification scratch, outside this package.
