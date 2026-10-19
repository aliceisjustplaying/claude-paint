# Exact batch replay

`easel run` can keep a resumable checkpoint after every successful chunk, and
`scripts/finish_painting` and `scripts/replay_clip` always do: a replay that is
killed or interrupted picks up from its last finished chunk when the same
command runs again (`<out>.replay.ckpt`, `<frames dir>.replay.ckpt`), and the
checkpoint is deleted once the replay finishes. Live sessions and their
finish-only `live.ckpt` files are unchanged.

Hand-time frames (`--frames-dir`) resume too. After each chunk the recorder
appends its state to `frames.states` in the frames folder; a resumed replay
takes the state after the checkpoint's last chunk, cuts `frames.tsv` and the
frames back to it and carries on, so the frames come out as an uninterrupted
replay writes them. `--frames-every`, `--frame-width` and the light must match.

```
easel run painting.lua --replay-checkpoint replay.ckpt --until-chunk 137 --out final.png
easel run painting.lua --replay-checkpoint replay.ckpt --for-minutes 50 --out final.png
easel run painting.lua --replay-checkpoint replay.ckpt --out final.png --checkpoint-out final.ckpt
```

`--until-chunk` is an absolute, inclusive chunk number. `--for-minutes` stops
at the first successful chunk boundary after its wall-time budget; it cannot
interrupt a chunk. Repeating the command resumes the existing checkpoint,
not a new replay. A partial batch saves its checkpoint and exits successfully
without exporting a partial final picture. The complete batch exports normally,
including the existing finish-only `--checkpoint-out` format. An already-complete
checkpoint can export again without painting its chunks again.

The checkpoint is replaced atomically after every chunk and contains its source
SHA-256, executable SHA-256, width, engine environment, cursor, elapsed time,
recorded engine effects and exact canvas state. Corruption, incompatible builds,
changed sources, changed widths, changed `PAINT_*` settings and divergent
reconstruction are errors. The previous checkpoint survives an interrupted
write. Rebuilding the executable requires starting a new checkpoint. A failed
engine effect with an unsupported error type makes checkpointing fail even if
Lua catches its error. Ordinary runtime errors are recorded and reproduced.

## Reconstructing Lua without serializing Lua internals

The checkpoint does not serialize Lua's heap. It reexecutes every completed Lua
chunk, including closures, table mutations, geometry, field callbacks and random
draws, at the original dimensions. Expensive engine operations do not run during
this reconstruction: `time::verb` and chunk-end flushes use a recorded result and
post-operation state. Held brushes, knives and rags are restored in place so Lua
aliases still share the same objects. Their complete state is stored, including
pickup, previous contacts, pigment, solvent and cure. The trace also restores
the studio RNG, clock, hand, palette board, call counters and chunk output.

Canvas-dependent reads outside verbs have their own typed trace records:
rag dampness time, drawing-guide masks, finishing dry checks, legacy color
samples and per-grid-node `color_over` input, sitting tally and timesheet data.
Lua callbacks still execute at each original grid node. Pencil operations record
their final wear as well as their return value, preserving the original sequence
of f32 additions. Rag registration runs before its traced verb.

Each traced operation validates its type, position and a SHA-256 of its incoming
studio/tool state. At the end of the prefix, all trace entries must be consumed
and the reconstructed studio/tool state must match the saved state bit for bit.
The saved canvas then replaces the disposable reconstruction canvas. Lua's fixed
hash seed and allocation serials preserve the original program's table traversal
and object identity. Consequently, the next chunk sees the original Lua state,
native objects and exact canvas, rather than a lower-resolution approximation.
Neither tracing nor reconstruction retains strong references to Lua's tools.

This boundary is part of the checkpoint format: new native operations must record
every canvas-dependent result and any closure-local mutation that escapes into
Lua. Build binding prevents an older trace from silently crossing such a change.

Resumable replay currently refuses combination with hand-time frames, per-chunk
state dumps or a painting state store, since those exporters have separate
continuation state. Plain, uninterrupted `run` still supports them as before.

## Progress

Batch replay atomically updates `painting.replay-progress.json` beside
`painting.lua` before and after each chunk. Plain `run` writes it beside its
output image instead, so replaying a read-only source remains supported.
It contains `version`, `status`, `chunks_done`,
`chunks_total`, `current_chunk`, `elapsed_secs`, `chunks_per_second` and
`eta_secs`. Unknown rates and ETAs are null. Paused and complete runs have no
current chunk. A one-line equivalent goes to stderr. Elapsed time and the
average rate include reconstruction and checkpoint work; ETA is an average
estimate, not a prediction of an individual chunk's cost.

Only one replay of a particular log should own its progress file at a time.
The progress file reports the last published state after a crash; the checkpoint
is the authoritative continuation cursor.
