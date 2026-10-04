# Delivery runtime verification

Run 2026-10-04 using source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`, its actual runner scripts and preserved release binaries. All studios, checkpoints, outputs and archive builds were disposable. No source edits or external providers were used. [CLI records](runtime-delivery-cli.json) and [workflow records](runtime-delivery-workflows.json) retain replies and exit codes with temporary paths replaced by `<fixture>`.

## Replay and comparison

An empty default session rejected save with `no canvas yet`. A live canvas with a red-earth stroke saved, returned exact live/replay agreement from `check` and closed with its log and live outputs. Missing input, empty input, unknown run option, unpaired frame arguments and widths 15/9601 were rejected; width 16 rendered successfully. A saved PNG header identified eight-bit RGB (PNG IHDR bit-depth 8, color-type 2); sRGB color interpretation was not separately validated.

`check_painting` exercised three outcomes:

| Fixture | Exit | Final result |
|---|---|---|
| Fresh two-chunk studio, setup then stroke, saved and closed without failures | 0 | `check: ok`; live PNG, checkpoint and painter save matched replay |
| Supplied twelve-chunk studio, whose close metadata said all twelve chunks had been replayed | 0 | `check: replay only`; equal files were explicitly not independent live evidence |
| Fresh one-chunk setup+stroke, followed by three rejected wet finishing calls | 1 | `check: DIFFERS from the live save`; PNG matched but checkpoint differed |

The third outcome is the confirmed counter-restoration defect isolated below. The rejected calls were `varnish{}`, `cracks{}` and `relief()`, each reporting paint not all dry and “the chunk failed and changed nothing.” A live `check` still reported exact canvas equality before close. The control without those attempts passed checkpoint comparison. The controlled follow-up below separates failed operations from chunk layout and identifies the changed fields.

The fresh control source was:

```lua
--@ chunk 1
canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}
--@ chunk 2
p=pile{{"red earth",1}}; b=brush("round",4); b:load(p,0.8); b:stroke({{100,80},{600,80}})
```

Nonempty comparison workdirs were rejected with exit 2. No concurrency or replacement-check cancellation scenario was exercised here.

## Finishing

`finish_painting` finished the twelve-chunk checkpoint, wrote a companion Lua log and compressed its checkpoint. Replaying the companion at normal width produced exactly the same PNG, SHA-256 `11cfd68324beb8bf9d14f1cc9e02a8d50ac1d79bf9b2b12198dcd8ab99b17f70`.

A fresh two-chunk studio also finished from its checkpoint. After compression removed the uncompressed checkpoint, invoking the script again took the replay fallback and succeeded. Decompressing the checkpoint and invoking direct `finish --log` with the matching log produced a PNG byte-identical to the replay fallback. Supplying the deliberately changed three-chunk log rejected the checkpoint with:

```text
is not a save of <fixture>/badlog.lua (2 chunks in the save, 3 in the log, or the text differs)
```

The companion begins with the unchanged original two-chunk source, followed by the finishing chunk. Naming output so its companion would overwrite the input log was rejected. A separate copied log with `error("deliberate replay error")` as chunk three failed to render: its finished companion Lua remained, but no completed PNG existed. The default varnish/cracks path was exercised; alternate coat counts, relief options and the ten-year retry limit were not.

## Replay video

The actual `replay_clip` script replayed the two-chunk control with `--every 0.2 --length 4 --width 160 --max-hold 2 --sheet <fixture>/sheet.jpg`. `ffprobe` reported:

```text
codec_name=h264
width=160 height=32 pix_fmt=yuv420p
r_frame_rate=24/1 nb_frames=96
duration=4.000000
```

The JPEG contact sheet was visually inspected: twelve views in a four-column, three-row arrangement, showing blank ground followed by the stroke. [Encoded clip](runtime-delivery.mp4).

Re-encoding with `--reuse` and a nonexistent input Lua path succeeded from the existing frames directory. An impossible 1000-second duration/max-hold combination was rejected before encoding. The frame manifest contained chunk, tick and final entries; duplicate hand times were collapsed to two moments for encoding. A one-chunk setup+stroke control had only a single final hand-time moment and correctly could not fill the requested four-second clip. Dynamic pacing and drying-wait time exclusion were not exercised.

## Pinned studio export

Executed the unmodified export script with `R16_BRANCH=4e525e50897807e9b5f734071dfeb330f1a393d3`, a dedicated `STUDIO_BUILDS` and an empty destination. The blank profile built from the committed archive and completed its own binary, guide, notices, tube-table, source-exclusion and outside-working-directory painting checks. Reply:

```text
exported the blank studio from 4e525e50897807e9b5f734071dfeb330f1a393d3 (4e525e5)
box: the default tube box
notes: easel_guide.md journal.md research/oil_paint_physics.md
```

The delivered binary was then executed directly: `open` and canvas creation succeeded; `run`, `check` and `finish` were unavailable; `print(type(varnish),type(cracks),type(relief))` printed `nil nil nil`. The session closed normally. Re-exporting into the nonempty destination was refused. Other box profiles, concurrent exports and the default historical `round-16` ref were not tested.

## Remaining limits

No disk-full, mid-write interruption, parallel destination overwrite, ten-year drying loop, 9600-pixel render or complete alternate finishing/video option matrix was run. The scripts' ordinary local workflows were exercised, not every checklist cell. Original source stayed read-only. Owned archive builds, PNG frames, disposable studios and intermediate images were removed after preserving this evidence and the MP4.

## Isolated failed-chunk checkpoint discrepancy

A follow-up controlled the chunk layout: five fresh studios received byte-identical one-chunk setup+stroke source. One closed immediately. The other four each attempted exactly one additional failing chunk before close. All five retained byte-identical successful logs.

| Extra attempted chunk | PNG versus replay | Checkpoint versus replay | check_painting exit |
|---|---|---|---|
| None | Equal | Equal | 0, `check: ok` |
| `varnish{}` | Equal | Different | 1, `check: DIFFERS` |
| `cracks{}` | Equal | Different | 1, `check: DIFFERS` |
| `relief()` | Equal | Different | 1, `check: DIFFERS` |
| `error("plain failure")` | Equal | Different | 1, `check: DIFFERS` |

This isolates a general failed-chunk path rather than finishing alone or different chunk boundaries. The byte comparison below identifies the changed fields; future painting behavior was not separately tested. [Full controlled results](runtime-delivery-rollback.json).

A byte comparison then isolated the discrepancy completely for the plain-error case: only bytes 110 and 118 differed. The no-error checkpoint header has `chunk=1` and `calls=1`; after the failed second chunk it has `chunk=2` and `calls=0`. The rest of the checkpoint, including canvas payload, was byte-identical. [Successful source reproducer](runtime-delivery-rollback.lua); the additional attempted chunk is `error("plain failure")`.

The serializer writes these counters in [save.rs](../../../claude-paint/crates/easel/src/save.rs:52). Chunk entry resets them through `begin(n)` in [session.rs](../../../claude-paint/crates/easel/src/session.rs:280), while the [snapshot](../../../claude-paint/crates/easel/src/session.rs:202) and restoration omit them. This explains the comparison failure as retained failed-attempt counters, not lost painting pixels. No source fix was made.
