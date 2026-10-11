# Compact committed render artifacts (stage 4)

The default-off `render-snapshot` Cargo feature adds explicit optical export and companion generation. It includes `state-store`. Successful chunks still publish only their material state; no snapshot or companion is automatically written, and painter `look` behavior is unchanged.

```sh
cargo build -p easel --features render-snapshot
easel snapshot-state /private/painting-state /private/painting.view
easel render-snapshot /private/painting.view /private/gallery.png
easel render-snapshot /private/painting.view /private/color.png --mode color
easel render-snapshot /private/painting.view /private/relief.png --mode relief
easel snapshot-companions /private/painting.view /private/companion-cache
```

Export pins and verifies the accepted store revision and decodes its trusted material checkpoint. It never executes the journal. The file records the committed revision and the actual exporting renderer executable's SHA-256; this renderer may differ from the executable that originally executed the material. Rendering requires that exact recorded renderer binary. Unsupported snapshot or renderer versions are refused. A rebuilt exporter can derive a new optical artifact from the same material; runtime migration and cross-build certification are later work.

## Format and crop contract

`CPVIEW02` is a fixed little-endian format: 180 bytes of header plus exactly 20 bytes per held pixel. The header includes format and renderer versions, the renderer ID, whole-canvas dimensions, held window origin and dimensions, pixels-per-unit scale, the buffer-relative visible `keep` rectangle, engine, micrometers per pixel, committed revision ID and renderer binary SHA-256. A SHA-256 digest covers all canonical metadata and the optical payload. A separate content ID hashes every encoded byte, including the digest. Identity hashes detect local corruption; they do not authenticate against someone who can rewrite the entire file.

The payload contains linear RGB appearance, wet-inclusive surface height and wet volume. It preserves their native f32 bits and the legacy lighting formula. It does not carry pigments, material layers, Lua state or editable wet paint, so it is renderable and cannot continue painting or simulate finishing.

All captured window pixels, including the lighting context around a crop, remain in the artifact. Lighting always evaluates the entire held buffer. PNG extraction happens afterward. Default output uses the visible `keep` rectangle without its padding. `--crop x0,y0,x1,y1` selects an end-exclusive rectangle in whole-canvas pixels and rejects requests not fully contained in that visible region. It never silently clamps a missing crop or relights a cropped subset without its context. This does not invent context beyond what a cropped material checkpoint actually stored.

The reader validates dimensions and offsets before allocating: at most 16 million held pixels, whole dimensions at most 9600×9600, positive finite scales, engine 1–5 and valid visible bounds. It rejects unknown versions, non-finite optical values, negative wet volume, digest mismatch, every truncation and trailing bytes. Serialization and decoding use blocks at most 64 KiB; there is no full serialized payload clone. The optical arrays require 20 bytes per pixel; shading and PNG encoding allocate additional working buffers. Material capture still incurs the existing trusted checkpoint decoder's memory cost.

Exports and standalone PNGs use exclusively created temporary files, file sync, atomic rename and parent-directory sync. Failure before rename preserves the old output and removes the partial temporary file. Snapshot output must be outside the store and cannot alias any of its files. PNG output cannot alias the snapshot input.

## Companions and bounded cache

`snapshot-companions` renders Color, Gallery (115/55/1) and Relief (135/25/1), without any material checkpoint, source log or `look` event. Its JSON stdout supplies the three PNG paths, snapshot content ID, revision and accepted cache generation. These are native-resolution PNGs with the same encoder and sRGB conversion as direct delivery.

Use a dedicated cache directory. Cache identity includes the complete snapshot content ID, committed revision, actual renderer binary, renderer edition, PNG encoder identity, complete output crop and exact f32 bits of both lights. `--gallery-light az,el,strength` and `--relief-light az,el,strength` explicitly change those keys. Known alternate cache renderer/encoder identities invalidate reuse; an unsupported optical renderer is refused at decode. Reusing a generation first verifies its manifest digest and every PNG's SHA-256 and byte count, so corruption cannot return stale companions.

One exclusive writer lease covers the transaction. All three content-addressed PNGs and their immutable manifest are synced before the cache's `CURRENT` pointer is atomically installed. The current and immediately previous generations are retained, with deduplicated PNGs. Missing or corrupt pointers are refused when retained files exist. Corrupt cache files fail closed; discard a damaged derived cache and regenerate it from the preserved snapshot. This cache has no effect on the authoritative material root. Returned paths are generation-specific and can be collected after two later generations; they are not an archival file-retention API.

The logical-file-byte cache quota defaults to 512 MiB and can be set with `--cache-budget-mib N` (1–4096). Admission includes existing retained files, a conservative bound of `4×visible pixels + 1 MiB` per pending PNG and 64 KiB for metadata/staging. Per-file writers enforce that bound. An oversized next generation preserves the accepted one. Filesystem allocation overhead is excluded, and external ENOSPC remains an I/O failure. Standalone snapshot export has a hard bound of 320,000,180 bytes; PNG bounds depend on the requested visible rectangle. These are explicit file budgets, not an enforced process RSS limit.

Automatic optical export after accepted chunks is an option for a later layer behind a separately admitted budget. This layer does not add that work to a live painter's critical path.
