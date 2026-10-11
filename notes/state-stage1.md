# Final checkpoint export and replay-free views

This is PR 1 of the state architecture plan: final-state export. The saved artifact is the existing `easel save 1` header plus the edition's unchanged `Canvas::write_state` bytes. It is renderable and finishable. It does not contain the Lua heap, held tools or arbitrary painter globals and cannot resume the next painter chunk.

## Export a final state

```sh
easel run painting.lua --out color.png --checkpoint-out final.ckpt
```

The checkpoint is written once, after every chunk and requested output succeeds, using a buffered, synced, exclusively created temporary file in the destination directory and rename. Failed writes remove their temporary file. No `--dump-state` is needed and no intermediate states are written. A failed chunk leaves any previous checkpoint at the destination untouched. Exporting does not change the painting's state, RNG, edition, look or log. `--width` still means a development-resolution reconstruction; its checkpoint records that resolution.

The checkpoint must have a separate destination from the source, PNG, digest, surface dump and derived look output. It must also be outside any `--dump-state` or `--frames-dir` directory and must not be hard-linked to their existing outputs. These checks run before any output is opened and resolve symlinks and `..` even when the destination does not yet exist.

## Open an existing studio save

```sh
easel render-checkpoint live.ckpt color.png --mode color
easel render-checkpoint live.ckpt gallery.png
easel render-checkpoint live.ckpt relief.png --mode relief
easel render-checkpoint live.ckpt.zst gallery.png
```

Both uncompressed `live.ckpt` and zstd-compressed `live.ckpt.zst` saves from existing studios open directly. Zstd decoding streams into the existing canvas reader; it does not create an uncompressed disk copy. The original save stays unchanged. Rendering needs no painting log and executes none of its chunks. It uses the existing Color, Gallery (115°/55°/1) and Relief (135°/25°/1) formulas without aging, varnishing or cracking the paint. Gallery is the default for this developer command. Painter-facing look defaults remain unchanged.

Output is native resolution. Add `--look` to use the existing bounded look renderer (default longest side 1000 px), whose bytes match the corresponding Color look or Gallery/Relief companion from the same state and build. Add `--log painting.lua` to require the recorded source hash and chunk count to match; this reads the source for verification only. It never replays it.

There is no conversion step for a supported existing save in this stage: it already is the saved state. Keep it alongside its source and canonical PNG. An older studio without a checkpoint needs one final replay on its recorded runtime to backfill using `--checkpoint-out`, then every view can open that result. Never replay separately for each light.

## Limits and compatibility

The reader supports the canvas checkpoint versions the current engine already reads. An unsupported historical format still needs its historical runtime or a separately verified migration. Saves are trusted local artifacts: the legacy material decoder is not an allocation-bounded hostile-input parser. Compressed saves must still fit the process's canvas memory budget after decompression; compression bounds neither that allocation nor the renderer's scratch.

Same-build native PNG and look-companion bytes are tested for engines 1–5, including wet paint. Historical cross-build or cross-platform byte identity remains unverified without the original runtime's golden. No physics, numeric ordering or checkpoint schema was changed. The replay build alone adds these commands and the zstd dependency. Studio saves on close use the same exclusive temporary-file writer; their saved bytes and painter-facing behavior stay unchanged.

Per-chunk publication, source-prefix manifests, strong integrity hashes, retention/GC, compact optical snapshots and a tiled working set remain later PRs (3b–5 in the architecture report). This final-only export does not add crash durability between accepted chunks or claim resumability. The current temporary-file rename is the existing final-save mechanism, not the future multi-object transaction protocol.
