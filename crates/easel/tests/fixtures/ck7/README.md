# Native PAINTCK7 fixture

This is a real version-7 checkpoint written by `Canvas::write_state` from historical commit `38a990d2857eb7642bdc5d62efd9f0f930caef48`, the source of the largest pre-save studio group. The current candidate did not generate the expected PNG or checkpoint.

`generate.patch` adds an isolated developer probe and a write-only `easel save 1` envelope to that commit. The envelope records engine 1 because the historical palette has no engine field, matching the backfill's save-on-close adapter. Native painting, optical rendering and checkpoint serialization are unchanged. The probe paints three synthetic chunks at 32×26: a ground, a blue wet passage and a dark passage after a short wait. It uses the old session, writes the old checkpoint and calls the old `deliver` for `color.png` from that exact state, without another replay.

The only version-7 layout difference from version 8 is that the stream ends immediately after the nine-word hand-time ledger; version 8 adds an engine u64. Version 7 therefore imports as engine 1. The test checks every material byte by re-exporting as version 8 and comparing against the original version-7 bytes with only the magic changed and an engine-1 word appended. Writers in the current engine remain unchanged.

## Generating runtime

- Rust: `1.97.1`, compiler commit `8bab26f4f68e0e26f0bb7960be334d5b520ea452`.
- Target: `x86_64-unknown-linux-gnu`, Ubuntu 24.04.
- Source: `38a990d2857eb7642bdc5d62efd9f0f930caef48` plus the included probe patch.
- Dependencies: that commit's unmodified `Cargo.lock` (`--locked`).
- Build: historical release profile, opt-level 3, one codegen unit, non-incremental. Historical `.cargo/config.toml` fixes the Lua hash seed.
- Runtime: `RAYON_NUM_THREADS=2`; default historical tube box.

```sh
git checkout --detach 38a990d2857eb7642bdc5d62efd9f0f930caef48
git apply /path/to/generate.patch
CARGO_TARGET_DIR=/private/ck7-target cargo build --locked --release -p easel
RAYON_NUM_THREADS=2 /private/ck7-target/release/easel ck7-fixture /private/fixture
zstd -q -3 /private/fixture/live.ckpt
```

## Frozen hashes

| Artifact | SHA-256 |
|---|---|
| `color.png` | `49d56389b9a8682e10b41d25409f9df98eb672504c85f8edd5be818487120722` |
| Raw `live.ckpt` | `43fa81f77fc1ddf2874ed63296220d78d335c842738c8b2d58d757bf3e2d1a88` |
| `live.ckpt.zst` | `884af9d50829062bfb4a16ee05d66ce98b148be40fbffa44e7cceb6c7fce5499` |
| `painting.lua` | `7027570fc057198c104e7b511da431d2648de3e9d36cfe0e68a2ae2a1a3b203b` |

The Color test requires PNG **byte equality**, for both the compressed and raw save, with and without the original source available. It does not substitute a tolerance or claim that historical painting replay is cross-platform exact. Malformed cases cover truncation, a trailing engine word, contradictory engine metadata and unknown older magic.
