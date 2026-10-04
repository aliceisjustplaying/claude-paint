# Commands, canvas and views runtime verification

Agent-run on 2026-10-04 against a clean checkout of `4e525e50897807e9b5f734071dfeb330f1a393d3`, using its freshly built default release binary. All roots were disposable. The original working checkout was not modified. [Raw command replies and assertions](runtime-cli.json) retain inputs, exit codes and observations; repeated identifiers distinguish successive probe attempts by their order in that array.

## Commands and lifecycle

- Empty and whitespace-only chunks are rejected; the empty-source description was corrected. Nonempty print-only and assignment chunks succeed. Globals survive later chunks; locals do not. Failed assignments restore the previous global.
- Lua table/string/math/utf8 facilities are available; io, os, require, dofile, loadfile, debug and collectgarbage print as nil.
- Inline, file and stdin submission each returned the same marker value. A file changed while its accepted chunk ran did not change the accepted assignment.
- A bounded slow chunk printed before and after its work. No reply bytes arrived during computation; both printed lines arrived with completion.
- A disconnected queued request was skipped. A separately disconnected running request committed. For the latter, the server's CPU time advanced from 0:02.02 to 0:02.32 while no reply was available; the socket then closed and another caller observed its retained assignment. This establishes cancellation after execution began, not merely after sending input.
- Explicit session selection beat the environment, which beat the remembered session. Invalid names were refused. Reopening an existing session replayed its retained chunks; opening an already-running one reattached.
- Two marks followed by a Lua error left an identical PNG. Successful replay matched the live PNG exactly. Close produced live PNG, metadata, checkpoint and witness files and removed the socket.

The actual infinite-loop check ran for **600.010844 seconds**, returned the ten-minute-limit error and reported rollback. A subsequent command succeeded as chunk 1. [Deadline receipt](runtime-deadline.json). This measures Lua-hook interruption, not a deadline inside arbitrary native operations.

## Persistence and external edits

For B01, the isolated session's `committed.pending` path was made a directory. The next chunk computed, appended its source log and failed to write the witness. Log and witness diverged. Status, painting, log and close all refused ordinary operation with integrity errors. Restoring the original log bytes still left the running session reporting uncommitted state. Stopping only that owned server and reopening the preserved baseline files recovered the original assignment. This demonstrates a recovery experiment; it is not a supported user recovery procedure.

A separate external-log edit before invocation caused status and close to refuse. Restoring identical original bytes let that otherwise unchanged server continue. These are distinct from ordinary Lua failure and its rollback.

## Canvas and materials

Missing size/aspect/linen/ground, size 49/5001, aspect 0.19/5.01, linen 3/61, unknown setup keys, ground thickness 4/401 and unknown tube names all failed. Status remained at zero chunks with no canvas. Valid setup followed by an explicit error also left no canvas and no W/H globals; a later valid two-coat setup succeeded.

The two-coat case used size 100, aspect 2, linen 12/16, red-earth knife ground then lead-white brush ground and seed 7. Its status and look succeeded. A fresh aspect-2 probe printed W=1000, H=499.99996948242188 and day 1,09:00. The previously documented floating-point limitation applies here too: an exact H==500 assertion failed, while a 0.001-unit tolerance passed. Aspect 5 printed approximately 200 for H; live output remained 2400 pixels wide.

Pile parts 0, negative, infinite and NaN were rejected, as were missing tube names and medium outside 0–0.95. Endpoint medium values succeeded. A local pile disappeared from the next chunk's scope; a global pile remained. A failed reassignment restored the old red-earth pile and removed the new marker.

## Views and image inspection

The ordinary, value, mirror, grid, palette and crop requests returned valid eight-bit RGB PNGs. [Independent pixel checks](runtime-look-pixels.json) decoded the generated images using Pillow:

| Observation | Result |
|---|---|
| Default whole image | 1000×200 |
| Requested size 64 | 64×13 |
| Requested size 0 | 1×1 |
| Requested sizes 2000 and 100000 | Clamped to 1600×320 |
| Crop corners 100,50,300,150 | 480×240 native pixels |
| Reversed crop corners | Identical pixels |
| Crop with size 200 | Still 480×240 |
| 500-unit-wide crop | 1200×360 |
| 501-unit-wide crop | Rejected |
| Mirror | Exact horizontal reversal of ordinary image |
| Value | Every decoded pixel has equal R,G,B |
| Value plus normal | Identical to value alone |
| Grid | Visible lines/labels; image differs from ordinary view |

The [grid image](runtime-look-grid.png) and [palette](runtime-look-palette.png) were visually inspected. Grid labels follow canvas coordinates. The palette contains the red-earth global pile and labeled thick, 12 µm, 4 µm and striped-card panels. [Ordinary image](runtime-look-plain.png), [value image](runtime-look-value.png).

Palette combined with size, crop, mode or grid was rejected. Unknown options/modes, malformed crops, excessive crops and a subpixel grid were rejected. Successful and rejected looks left painting-log bytes unchanged; viewing also left the saved painting PNG unchanged. Removing an earlier generated look did not replace a surviving later look. Denying writes to the output directory made plain look report Permission denied; restoring permission and saving confirmed unchanged painting pixels.

A successful no-canvas assignment with follow-up look returned chunk success plus the missing-canvas warning. This checks the commit-before-look contract; a file-write failure specifically after commit was not injected. Frame capture produced files in its separate frames directory.

## Probe corrections and limits

The raw records retain two corrected probe expectations: empty source was wrongly expected to succeed, and H was wrongly compared to an exact decimal. The first was a documentation error; the second is the already documented floating-point representation. A first CPU-time observation called a different `ps` implementation and failed to establish execution; the repeated case used `/bin/ps`, observed increasing CPU time and passed. These are not three product defects.

The initial long macOS temporary path exceeded Unix socket limits; successful probes used a short isolated root. Rust test receipts separately describe stale embedded paths after relocating the checkout. No source workaround was introduced.

Not established here: all setup textures and physical calibration, every rendering-mode combination, precise native-operation deadlines, every acceptance/interruption boundary, disk exhaustion or simultaneous external writers. Checklist rows retain partial/blocked status where the complete scenario was not exercised.
