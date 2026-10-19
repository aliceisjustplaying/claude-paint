# Committed finishing state (stage 3b)

This layer publishes a finishable material revision after each successful chunk, independent of `look`. It uses the existing lossless checkpoint format and preserves old execution and rendering formulas. Compact optical snapshots, spatial tiles and copy-on-write rollback are later layers.

## Opt in

The `state-store` Cargo feature is off by default. The shared default-feature easel does not enable it, and enabling the feature alone does not change session persistence. Studio runtime selection is documented in [easel.md](easel.md#studio-selection).

```sh
CARGO_TARGET_DIR=/private/target cargo build -p easel --features state-store
easel run painting.lua --state-store /private/painting-state --out color.png
easel render-state /private/painting-state gallery.png
easel render-state /private/painting-state color.png --mode color
easel render-state /private/painting-state relief.png --mode relief
```

`run --state-store` requires an empty store. It publishes a canonical accepted journal and material checkpoint after each successful chunk, including before the final PNG. A failed later chunk leaves the previous successful revision available. It never replays a log to render a committed revision. `--look` on `render-state` uses the existing bounded look renderer instead of native resolution. Gallery is the display default; painter look stays unchanged.

For a live painting, compile the shared default-feature binary with the additional `state-store` feature and set `EASEL_STATE_SESSION` to that exact session name before opening it. A one-painting studio uses `EASEL_STATE_SESSION=painting`; a named developer sketch might use `EASEL_STATE_SESSION=sketch-study`. The store lives in `out/easel/<name>/state`. Only that selected painting opts in; scratch sessions keep their existing separate journal behavior. Do not set this environment variable on running painters until the rollout is approved.

The disk quota defaults to 2048 MiB. Override it with `--state-budget-mib N` for `run` or `EASEL_STATE_DISK_MIB=N` for a live selected painting. Values are 1–65536 MiB.

## Authority and capabilities

`CURRENT` contains one revision ID, or an explicit empty-root sentinel before the first publication. Losing the pointer while objects exist is refused without collecting those objects. Its immutable JSON manifest names the parent, the accepted source's length and SHA-256, chunk count, material object, schema, native geometry, physical pixel scale, pigment box, engine and actual executable SHA-256. Capability flags distinguish renderable and finishable state from resumability. Before a canvas exists, a source-only revision is valid but cannot render.

Object IDs hash canonical uncompressed bytes with their object type and schema. Source and checkpoint objects are independently compressed with lossless zstd, including a frame checksum. Readers verify the strong hash and exact byte count, then check that the checkpoint's envelope agrees with the accepted journal. SHA-256 provides local integrity, not authentication against someone who can replace every object and root.

When opted in, the manifest is the commit authority. The `.lua` and `committed.lua` files become source exports. They are updated only after root publication; an interrupted export may be missing or a prefix of accepted source. Startup verifies the committed objects first, repairs those known exports and rejects unrelated source replacements. Lua session reopen still replays the verified source: this artifact does not contain the Lua heap, held tools, palette board or world view.

Continuing a committed store requires the exact recorded executable. A changed binary is refused instead of silently reconstructing accepted material with a different runtime. Read-only `render-state` verifies the saved state and uses the current identified legacy renderer; it does not continue a painter session. Runtime migration and a portable edition registry remain later work.

## Publication and recovery

One writer holds `writer.lock` for its lifetime. Read leases share `readers.lock`; publication and garbage collection take it exclusively so a reader cannot lose its material objects while rendering. The expected parent is checked before staging and again before publishing.

1. Compute bounded source and material fingerprints without collecting serialized material in memory, and admit the whole transaction against the disk quota.
2. Exclusively create temporary objects, stream their bytes through the lossless encoder and verify that the written bytes match the precomputed ID. Sync each file, install the immutable object and sync the object directory.
3. Write and sync the immutable manifest, install it and sync the manifest directory.
4. Write and sync a temporary root pointer, atomically rename it to `CURRENT` and sync the directory before acknowledgment.
5. Retain the current revision and its immediate predecessor and collect other owned objects and manifests while no reader can lose them.

A crash before pointer installation exposes the complete old revision (or no revision on the first commit). A crash after installation exposes the complete new revision. Startup removes unreachable objects and abandoned staging files only after validating the accepted root. Corruption is refused, not repaired by replay or silently replaced with an older painting.

An I/O failure before publication leaves the accepted root unchanged. If root installation has happened but its sync fails, commitment is indeterminate: reopen and inspect the accepted chunk count before retrying. Live publication failures discard the unaccepted in-memory session by terminating its server after the error reply. An export failure after a completed commit acknowledges the accepted chunk and stops the server so startup can repair the exports. No retry should assume that an error after publication means the chunk was never accepted.

## Explicit limits

- At most two retained manifests and their objects remain after collection. Parent IDs are provenance links, not instructions to retain an unbounded ancestor chain.
- The logical-file-byte disk quota includes retained state and worst-case pending compressed objects, with a 64 KiB reserve for manifests, root staging and small metadata. Admission is conservative even when real compression would fit. Filesystem allocation overhead is not included in that logical count. An external ENOSPC still rejects publication before installing incomplete state.
- A material payload is at most 1 GiB uncompressed, source at most 16 MiB and a manifest at most 16 KiB. Serialization and hashing stream through bounded buffers. Store decoders limit the zstd window to 8 MiB.
- These are storage and I/O bounds, not a 3 GiB process-memory promise. The legacy active canvas, Lua heap and existing rollback clones still require their normal memory; the tile working set and transaction-delta admission belong to later stages. The existing canvas parser remains a trusted-data decoder behind object-integrity checks.

Sources and outputs must stay outside the reserved store namespace. Immutable objects and manifests are read-only. The first tests cover abrupt process exit at each publication phase, old/new completeness, lost updates, writer races, corruption, cross-object source agreement, retention, quota failures, reader leases and byte-identical replay with publication enabled.
