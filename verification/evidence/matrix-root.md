# Remaining canvas, look and lifecycle matrix

Source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. Actual release binaries, disposable roots and real Unix sockets. Fixtures are in [root.rb](../fixtures/matrix/root.rb), [canvas.rb](../fixtures/matrix/canvas.rb), [views-concurrent.rb](../fixtures/matrix/views-concurrent.rb), [lifecycle.rb](../fixtures/matrix/lifecycle.rb), [sessions.rb](../fixtures/matrix/sessions.rb) and [offline.rb](../fixtures/matrix/offline.rb). No production source changes or fake clocks.

## Views and read-only state

[Raw results](matrix-root/root.json) and [decoded pixels](matrix-root/pixels.json). Ordinary and palette looks returned PNG paths and dimensions. Global red and blue piles appeared in the palette. Value/gray and squint/blur aliases were pixel-identical. Mirror was the exact horizontal reflection; combined value/mirror and squint/mirror exactly reflected their respective transformed originals. Value RGB channels were equal. Blur changed pixels. Grid images differed while the following plain image exactly matched the preceding plain image. The [mirrored grid](matrix-root/mirror-grid.png) was visually inspected: labels decrease from950 to50 left-to-right while y increases downward. The [plain image](matrix-root/plain.png) retains red paint and the graphite line over the textured ground.

Saved full-resolution PNG, brush fullness, rag load/soaked/damp/fold and painting clock matched before and after14 look variants. The successful log did not change during those views. A subsequent brush touch succeeded. Invalid mode and crop left status/chunk count unchanged. Undo and look-restore were rejected. A successful `do --look` produced its next image; failed Lua did not produce a follow-up image. A later plain look used ordinary settings.

For the postcommit filesystem fault, the fixture waited until `committed.lua` changed, then removed directory write permission while the follow-up image was rendering. The CLI returned success for chunk6 and `(no look: ... Permission denied)`. The committed marker remained available; restoring permission allowed the next look. This is a real postcommit write failure, not a failure to persist the painting.

## Concurrent looks

[Raw results](matrix-concurrent/root.json). Each accepted-render probe observed server CPU advance while the first socket had no response. A value look remained distinct from the mirror request queued behind it. A paint command queued behind a plain render completed after that render; the later image contained the new mark. The renderer remained usable after the active client disconnected. A look disconnected while still queued behind a slow chunk produced no image.

Editing the log before a look rejected both look and paint. Editing during an observed render allowed that captured render to finish, then the next status refused the mismatched log. Terminating the server during a render returned an empty response, preserved the committed log and permitted reopening from it. These receipts cover actual render execution, not only slow Lua around an already completed look.

## Canvas boundaries

[Raw results](matrix-canvas/root.json) and [image comparison](matrix-canvas/pixels.json). Actual live sessions accepted size 50/5000, aspect0.2/1/2/5, linen4/60 and warp/weft pairs. Immediately out-of-range values rejected without a canvas. Every live session remained2400px wide; saved look dimensions followed the aspect. Preparation plus material setup and a mark committed as one chunk.

Omitted seed and explicit seed1 produced identical images. Seed2 differed. Knife, roller, brush, weave variants,5/400µm coats and reversed coat order produced11 distinct inspected image hashes. Thickness4/401 rejected. Medium0/0.5/0.95 accepted, negative and0.96 rejected. Printed duplicate tube entries combined into the same pile mixture. New piles did not mutate existing ones. An attempted replacement canvas rejected and the saved image stayed identical; Lua reseeding remained available.

Piles, brush, mask, outline, rag and geometric form survived globally and were used in a later chunk. Canvas-dependent operations before setup rejected. Unsupported image input rejected. Current Lua exposed no resize, clear, undo or layer global. The running sandbox exposed no `os`, `io` or `require`.

## Submitted chunks and setup interruption

[Raw lifecycle results](matrix-lifecycle/root.json), [shared material transport](matrix-transport.json) and [transport fixture](../fixtures/matrix/transport.rb). Queued setup disconnected before execution left no canvas. Disconnecting after preparation began did not prevent the accepted chunk from finishing. A second changed setup stayed pending then rejected as already initialized. File input retained the captured source after the file was overwritten.

Server termination before chunk commit lost the uncommitted setup and reopening had no canvas. An external log edit before setup rejected admission; an edit during the accepted chunk prevented commit and restricted readiness. A real witness temporary-path fault left the appended log different from its old witness and restricted status. These probes act at the submitted-chunk boundary. Material transport probes interrupt after the native operation returns but before the containing chunk commits; they do not claim instruction-level interruption inside each native algorithm.

Lost reply followed by resubmission incremented the counter twice. Failed material changes restored saved PNG, held brush/rag state and clock; no failed chunk entered the successful log. Identical sessions produced identical random values and final PNGs after one inserted a failed random chunk. A caught invalid touch reported its error and the enclosing valid chunk completed.

## Journal and inspection

[Raw journal receipts](matrix-root/root.json). Status, globals, log and notes preserved painting PNG and successful log. Multiline stdin was stamped and indented; whitespace-only notes rejected without modifying the file. A journal missing its final newline received a separator. Denying journal writes returned a permission error without changing its text. A queued note whose socket disconnected was skipped. Notes containing painting-like text did not change setup.

Status, globals, log and note requests queued behind observed running Lua and returned after its marker committed. Mutating a global table's contents did not change its assignment-chunk stamp. Two sessions shared the root journal while retaining different globals, logs and painted images. Missing-session inspection and note requests returned errors.

## Sessions and closure

[Raw session receipts](matrix-sessions/root.json). The painter binary ignored `EASEL_SESSION`, rejected named open/session flags and used its studio's one `painting` log. Developer sessions in two roots stayed independent. Selecting session b while a ran did not redirect a's accepted chunk. Changing client box configuration did not alter the already running server's capabilities.

Adjacent box and environment configuration must agree: conflicting names rejected, rather than the adjacent file silently winning. An existing recorded box also rejected a conflicting new configuration; removing the conflict reopened the recorded painting. This corrects the old checklist's precedence wording.

Terminating the opening client during replay left the background server running. A second open waited and reattached. The ordinary request before readiness failed to connect. A dead socket did not imply readiness and open recovered it. A denied root prevented launch. Open-width flags rejected. A closed log modified without its witness refused reopening.

Checkpoint temporary-path failure was reported on close while the live image, metadata and Lua log survived. Reopening succeeded from the log, including after deleting the checkpoint. The separate [historical replay results](matrix-oldlogs.txt) compare all 11 fixtures with checked-in golden PNG hashes and per-chunk digests, including headerless engine1 and legacy scene helpers. Current setup rejected the old style/palette vocabulary.

A further defect was reproduced: editing the log after replay captured it leaves the initial `open` polling an integrity-restricted server. [Four-second reproduction](matrix-open-error/root.json) records repeated `status ... err`, a completed replay and an opening client still running until the fixture terminates it. The loop ignores unsuccessful status responses and resets its no-progress clock on each new server-log line; see main.rs489–506. No thirty-minute duration is claimed for this defect probe.

## Offline

[Actual OS sandbox receipts](matrix-offline.json). A loopback TCP control succeeded outside the sandbox and failed inside it. The same IP-outbound-denied sandbox opened the easel, prepared and painted the canvas, rendered a look, appended a journal entry and closed successfully using the local Unix socket. Provider availability is not required for those local commands.

## Additional variants and all input channels

[Extra receipts](matrix-extra/root.json) show a local-only pile does not become a palette global, while a subsequent global pile appears. The accepted file-based `do --look` retained its source after the file changed and produced its follow-up image. A separate offline replay completed while an observed live chunk was pending; the live marker committed to its original session. Intersecting wet strokes and a week-dried underpainting produced different overlap images. The [coordinate fixture](matrix-extra/coordinates.png) places marks at(20,20),(120,20),(20,120) and uses0/π/2 passage angles with a labeled grid.

[Channel receipts](matrix-channels/root.json) reproduce the actual painter tool's exact source through inline CLI, file and stdin. All four generated the same80×16 value/mirror/grid PNG SHA256 `8b31006d8dda548e21cf97af4aaf50844d899daff18b27ca38ddf433a610e357`. The source includes canvas setup, brush, rectangular passage, pencil, rag, wait and a geometric reference; [harness receipt](matrix-harness.md#equivalent-cli-and-tool-view). The [durable source](../fixtures/matrix/channel-input.lua) is included.

## Harness client budgets

[Actual production client with an external accelerated clock](matrix-client-budgets.json) scheduled180000ms for ordinary calls and720000ms for paint, including the initial status/open chain. The rebuild response changed the deadline to the1800000ms inactivity budget; after its100ms poll interval under the1000× clock, the next subprocess received1699000ms remaining and ultimately reported stalled progress. A delayed opening used only the remaining original budget and reported that it did not open in time. No `WAIT_MS` constant was changed. [Fixture](../fixtures/matrix/client-budgets.mjs).

These are default scheduled-duration and control-flow checks against real child processes with controlled replies, not three-, twelve- or thirty-minute wall-clock waits. Existing production client tests separately exercise advancing rebuilds beyond the ordinary budget and a fresh command budget afterward. The session startup timer below is a separate actual wall-clock run.

## Per-option render concurrency and CLI output

[Separate variant receipts](matrix-render-variants/root.json) exercise crop, squint, mirror, combined value/mirror, grid50, size500 and palette while each render is actually pending. Each first request showed server CPU advance with no response; a differently configured second request remained queued. Every first PNG matched the same-options baseline exactly. The palette contained27 globals so that its render was observable; the canvas used the accepted0.2 aspect to make even size500 rendering observable. [Fixture](../fixtures/matrix/render-variants.rb).

A real CLI stdout pipe contained no bytes at7.9ms while server rendering was observed; the single path/dimensions reply completed at83.0ms. This is separate from the raw-socket observations. In the painter build, both `-s another status` and `open another` rejected while a1.19-second chunk was pending; the original chunk still committed its marker to `painting`.

## Real startup inactivity limit

[Wall-clock transcript](matrix-timers.json), [fixture](../fixtures/matrix/timers.rb). An actual server stopped during replay made no progress and the opening client returned the30-minute error after1800.184seconds. A separate19-chunk replay was paused105seconds per chunk, with actual completed-chunk progress between pauses. Its opening client succeeded after2011.715seconds (33minutes31.715seconds). This demonstrates an inactivity limit rather than a30-minute total cap. No virtual clock or modified timeout was used for these two runs.

## CLI render followed by harness client

A real CLI value render was observed pending with server CPU advance. A preloaded Node process then invoked the unchanged production `atEasel` helper for a mirror look while the CLI still had no output and remained running. Both completed through the actual painter server with distinct paths. Decoded first-image pixels were achromatic and the second retained color. [Receipt](matrix-channel-concurrent.json), [fixture](../fixtures/matrix/channel-concurrent.rb). This is the production harness-client boundary; actual pi orchestration is covered separately, not claimed for this particular ordering.

## Journal revision preserves painting

The actual production revision helper replaced one unique passage in a real painter studio. Its history record contained the exact previous full journal, replaced/new text and a valid wall-clock timestamp. Before/after full PNG hashes, successful Lua log hash and status matched. [Raw receipt](matrix-journal-pixels.json), [fixture](../fixtures/matrix/journal-pixels.mjs).
