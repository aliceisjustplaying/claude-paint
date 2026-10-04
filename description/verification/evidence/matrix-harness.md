# Remaining harness matrix

Source `4e525e50897807e9b5f734071dfeb330f1a393d3`, 2026-10-04. Actual installed pi loaded the unchanged painter extension and an explicit local recording provider. The studio used the stable `--no-default-features` painter executable. No external model request or production source change was made.

## Actual painting and image history

The launch restricted tools to `paint,look,note,status,log,read`. A studio AGENTS.md supplied `CONTEXT_SENTINEL_MUST_BE_REMOVED`; the launch supplied `--append-system-prompt APPEND_SENTINEL` and a custom prompt beginning with `<!-- COMMENT_SENTINEL -->`. The stored system message contained only preamble `Only fixture prompt.\n` and cwd. It recorded exactly those six tool schemas. The deliberately supplied context and append sentinels were absent.

Relative, absolute and `file://` reads of BRIEF.md all returned the same fixture text. An outside path was refused and a missing path reported missing. The separate earlier symlink check remains in [runtime evidence](runtime-harness.md).

One real paint tool chunk successfully executed canvas setup, brush loading and touch, rect mask creation, work, HB pencil line, rag dip/blot, wait(0), form construction and parts_mask. It printed `day 1, 09:03` and `1255.5554557729711`, then `ok`. The log contained that exact successful chunk. This establishes actual structured tool admission and shared Lua execution, not every visual claim for these tools.

The same provider response submitted append, exact revision, status and log in order. The appended multiline note was present for the immediately following revision; the status reported the configured setup and log contained one painting chunk. Later tools returned these actual images:

| Options | Returned dimensions |
|---|---|
| size 80, value+mirror, grid 50 | 80×16 |
| crop 0,0,100,100 with size 80 | 240×240 (crop retains full resolution) |
| palette true | 1124×96 |
| 21 successive looks, size 20 | 20×4 each |

The final provider context held 19 images. At the 21st image, the request count dropped from 20 to 16: five oldest images were replaced by text. The stored session retained all 24 images. Final `painter-request-images` metadata was `{images:24,kept:19,dropped:5,keptChars:4940,maxImages:20,maxImageChars:12000000}`. Thus the default count ceiling and batch removal were exercised through pi. The 12 MB byte ceiling was not reached by these tiny images.

## Recovery input

Actual pi RPC received a first input with text `FIRST INPUT` and one image with `PAINTER_SITTING_RECOVERY=1`, then `SECOND INPUT` in the same process. The first recorded user message contained the current brief, multiline journal quoted line by line, actual live globals, journal clock and fresh canvas image. It had two image blocks: the original input plus the new canvas. The second input remained exactly `SECOND INPUT` with zero images; recovery did not repeat. Startup environment variables were removed before provider requests.

## Concurrent journal boundary

The unchanged production `reviseJournal` ran with a filesystem read interposer: after the first revision read `A unique B second\n`, a second complete revision changed `second` to `NEW`, then an external append added `EXTERNAL APPEND\n`. The first revision resumed and changed `unique` to `FIRST` using its already-read text. Actual final file: `A FIRST B second\n`. Both revision records retained the same original full journal. The second revision and append were lost from the current file. This is a controlled filesystem interleaving, not a claim of a naturally scheduled race or cross-process locking. It directly demonstrates the stale-read rewrite behavior described in J18/J24 and the journal portion of J28.

## Real provider transports, local endpoints

Actual pi OpenAI Responses and Google Generative AI adapters sent HTTP requests to disposable loopback recording endpoints with dummy credentials. Each endpoint returned two structured look calls, then a final reply; the real painter look tool executed both. All three requests were captured for each adapter. OpenAI `/responses` payloads marked every user/tool-result `input_image` as `detail: high`. Google `/models/gemini-3.1-pro:streamGenerateContent?alt=sse` payloads marked every image with `media_resolution_ultra_high`; tool-result images moved into the supported image parts. Request image counts grew 1→2→3. These are actual serialized wire payloads, not pure transformer calls. They establish H41/H42 without contacting external providers or asserting model acuity.

## Actual pacing and setting boundaries

An actual pi RPC session reported usage `input=2, cacheRead=9, cacheWrite=1` with input-token budget 10. After the first reply, the fixture changed the process environment budget to 999999. The second input still waited for the original 60-second window: first settled at 831 ms, second started at 832 ms and settled at 60833 ms. No wait text appeared in the model conversation. The oversized next request proceeded when the window emptied. A separate identical run aborted during that wait: second start at 827 ms, settlement at 1031 ms after abort at approximately 1027 ms.

With probe 0.4 seconds and give-up 0.00005 hours, the first limit response waited 408 ms; the second limit settled immediately because give-up had elapsed. Changing the process probe environment to 100 seconds inside the first provider response did not change the loaded 0.4-second setting. These are real wall-clock waits using existing diagnostic settings.

Invalid settings `PAINTER_MAX_IMAGES=-1`, `PAINTER_INPUT_TPM=-1`, `PAINTER_LIMIT_PROBE_S=-1` and `PAINTER_COMPACT_RESERVE=-1` each made pi exit 1 with their specific validation error.

**New defect:** `PAINTER_LIMIT_PROBE_S=Infinity` was accepted. Two limit errors produced stderr `asking again in Infinity min`, but retried after approximately 18 and 5 ms, then settled successfully 24 ms after first start. The parser checks nonnegative values without finiteness, so the timer cannot represent the advertised wait. Source: `harness/painter/limits.ts:45–46`.

## Summary bounds and failures

The production summary builder ran against the real easel with 159 live globals, a brief longer than 60000 characters and journal longer than 120000 quoted characters. It retained their end sentinels, dropped beginning sentinels, emitted truncation paths and retained exactly 150 globals. Successful paint-result evidence `day 3, 12:00` won over a later failed result `day 9, 12:00`; journal `day 2, 10:00` was separately identified. The full input files remained readable. An empty studio reported missing brief, no journal entries and explicit unavailable-easel errors for globals and canvas. This is the summary builder boundary, not automatic pi compaction.

## Equivalent CLI and tool view

The actual tool's 80×16 value+mirror/grid50 PNG and the subsequent equivalent CLI look had identical SHA-256 `8b31006d8dda548e21cf97af4aaf50844d899daff18b27ca38ddf433a610e357`.

## Compaction through actual pi

Two 60000-word local responses were followed by RPC `compact`. The actual compaction entry recorded `enabled:true,reserveTokens:100000,keepRecentTokens:20000` and `harnessSettings:true`. The summary quoted the multiline journal and included real globals and a saved canvas path. No provider request occurred for summarization: the recording provider received only the two ordinary turns and the post-compaction turn.

After compaction, a CLI brush touch changed the canvas before the next model request. That request inserted the summary-time image immediately after the summary. Its decoded PNG SHA-256 exactly matched the saved summary image (`1de7a48b8190c5c3159fb9a0dfef2cc96713868f1e2f2689dc96c0b0797d1dd4`), establishing that it used the saved image rather than a fresh look.

## Concurrent caller and tool abort

Actual pi emitted two paint calls in one response. The first ran an 800-million-iteration bounded Lua sum then set `slow_marker=123`. After tool_execution_start plus 200 ms, a terminal caller submitted `cli_marker=789; print(slow_marker)` to the same easel. The CLI waited and printed 123. The second tool printed 123 and 789, then assigned later_marker=456. A subsequent independent query returned all three exact marker values. The accepted first tool source remained unchanged; all callers shared one serialized painting.

A separate actual paint tool ran the same bounded sum and assigned abort_marker=999. Aborting pi 200 ms after tool start settled the agent at 1034 ms from launch, approximately 216 ms after agent_start. A later independent easel query returned abort_marker=999: the existing server completed its accepted chunk despite tool cancellation. This confirms cancellation does not promise rollback. The process-group kill is covered by the existing client-boundary test; this probe observed pi settlement and committed server state rather than inspecting every OS descendant.

## Default usage delays with controlled time

An external timer/Date.now adapter recorded the unchanged production harness's requested waits while advancing a virtual wall clock. Actual pi processed reset-one-second, no-reset and reset-one-hour errors. Requested delays were exactly 61000, 1800000 and 3660000 ms. The third error arrived just under the default 24-hour give-up threshold, so its full 61-minute wait was scheduled; the following error settled after the threshold. These are scheduled-duration and control-flow checks under a controlled clock, **not** elapsed 30-minute or 24-hour wall-clock runs. The earlier short waits and 60-second token window used actual time.

## Failure and cache boundaries

Actual pi compaction with a filesystem interposer throwing while the production summary builder checked BRIEF.md returned the short fallback `BRIEF.md and notes/journal.md are in the studio.` The compaction metadata recorded the injected error. There was no model summarizer call.

In another actual pi session, the fixture changed `PAINTER_COMPACT_RESERVE` to 1 after extension startup. Manual compaction still recorded the original 100000 reserve. Its saved image was then deleted before the first post-compaction request and restored before the next. Both requests contained zero summary images: the missing image remained cached unavailable for that process.

A production client call used an existing mutable `WAIT_MS.do` budget of 1000 ms, with a wrapper that ran the real easel chunk then delayed only its reply for two seconds. It reported `the easel didn't answer; log shows whether the chunk ran`. A fresh real caller printed timeout_marker=321, confirming the committed chunk despite the timeout. A first 150 ms attempt expired during initial status and did not commit; the second deliberately allowed admission and commit before reply delay.

A separate process ran the unchanged journal revision helper on a 16 MB disposable journal. The observer sent SIGKILL after the revision-history file became nonempty. The process died by SIGKILL; its full 16000091-byte revision record was valid JSON, while the journal still began with the original `unique\n`. The history write had completed and journal rewrite had not. This records actual interrupted filesystem work rather than promising transactional rollback.

## Remaining controlled boundaries

Four actual pi input images exceeded the default byte ceiling after pi's image encoding: 15673584 base64 characters before pruning. The production context hook kept one image (3918396 characters) and dropped three, stopping before discarding the newest image. Metadata retained the default 20-image/12000000-character limits. The separate 24-look run above establishes five-at-a-time removal when enough older images exist.

A filesystem interleaving during production summary assembly returned the old brief, then changed the on-disk brief, journal and actual easel global before subsequent reads. The resulting summary contained OLD_BRIEF, NEW_JOURNAL and mixed_marker=42, not NEW_BRIEF. This records the lack of an atomic cross-file/easel snapshot.

The compaction registration function was exercised with one SettingsManager getter temporarily absent in the fixture process, then immediately restored. It emitted `pi's SettingsManager has no compaction getters to override; pi's own settings apply`. This covers the registration fallback warning under the specified controlled interface, not compatibility with an uninstalled historical pi version.

## Final tool and setting probes

A recorded executable wrapper observed zero easel calls while actual pi read missing and outside files. Its next status tool, with an existing painting initially closed, produced exactly `status`, `open`, `status`; the open replayed the existing painting before admission.

With a one-second delay around the actual look executable, pi accepted size80/value and size40/mirror as separate tools. A concurrent terminal requested size60/grid50 while the first tool was pending. The two tool replies returned separate 80×16 and 40×8 images; the terminal returned 60×12. Later arguments did not replace the earlier accepted arguments.

Starting with image limit 7, the provider fixture changed the process environment to limit 1 and emitted three real look calls. The next request retained all three and metadata still reported maxImages 7; startup settings remained fixed. An actual pi recovery input in an empty studio stated missing brief, no journal entries and unavailable globals/canvas. A paint tool in that studio returned ENOENT; a subsequent local-provider network-error response remained an error rather than painting progress.

A real OpenAI adapter was also pointed at a loopback endpoint closed before the request. It produced no model completion during the observation interval and was terminated; a local easel status remained available. This is a local transport-unavailability probe, not an external-provider outage claim.

Terminal `TERMINAL_UNIQUE` was revised through the actual pi note tool while a separate terminal `PENDING_APPEND` waited behind an accepted slow chunk. Final journal retained REVISED_TERMINAL, PENDING_APPEND and the later TOOL_APPEND. The pending append remained an append and the revision retained its supplied exact-match text. The same provider message's revision and append were serialized.

## Retained raw receipts

[Compact harness receipts](matrix-harness-receipts.json) retain provider requests, timed agent/tool events, actual transport payload structure, client test output and canvas-state observations from the disposable fixtures. Image bodies are replaced by character counts and SHA-256; long repeated strings retain bounded excerpts. Timestamps are elapsed fixture milliseconds. The controlled-clock runs above remain controlled-clock evidence.

J28 also uses root's external-before/during checks in `matrix-concurrent/root.json` and `matrix-transport.json`; these complement the exact-text revision race and ordered append/revision observations above.
