# Runtime verification results

The remaining matrix exposed five additional product defects: default studio export failure, picker focus dismissal, an infinite retry-delay setting that retries immediately, opening stuck after a replay-time integrity error and invalid viewer position after history replacement. A separate parser limitation leaves nonshrinking in-place history rewrites stale; whether that edit path is supported remains a product call. The aging-slice wording in this description was also corrected. [Reproductions and causes](../bug-triage.md#b16--default-studio-export-builds-an-incompatible-historical-easel).

Source was pinned to `4e525e50897807e9b5f734071dfeb330f1a393d3`. Product source and tests were unchanged. These are agent-run results; the documents remain drafted until a person completes their P1/P2 rows.

A subsequent [real Space Bunny provider run](evidence/live-bunny.md) completed compaction, fresh-session recovery and exact replay. It exposed the missing painting clock (B23); its fix is preserved separately and is not integrated or deployed.

## Results

<!-- matrix-counts:start -->
| Checklist cluster | Pass | Fail | Blocked or partial |
|---|---:|---:|---:|
| Canvas and views | 160 | 0 | 0 |
| Mark making | 260 | 0 | 0 |
| Passages and time | 143 | 1 | 0 |
| Commands, sessions, journal and harness | 175 | 0 | 0 |
| Viewer and space | 161 | 5 | 0 |
| Delivery | 92 | 1 | 0 |
| Intended behavior in bug triage | 7 | 15 | 0 |
| Total | 998 | 22 | 0 |
<!-- matrix-counts:end -->

These counts cover the original matrix, excluding the later B23 follow-up row. Rows are assertions, not independent test executions. Several rows can share one observed scenario. A descriptive row can pass while describing a defect; its intended-behavior triage row fails. “Fail-document” records an inaccurate original description even when corrected here. This finite matrix does not prove that every possible combination is bug-free.

## Evidence

| Area | Executed checks |
|---|---|
| [Canvas, looks and sessions](evidence/matrix-root.md) | Live parameter extremes, decoded image transforms, tool/clock nonmutation, real concurrent rendering, accepted-input capture, rollback, process termination, filesystem faults, root/box isolation and offline operation under OS network denial |
| [Four input channels](evidence/matrix-channels/root.json) | Identical actual painter-tool source through inline CLI, file and stdin; all four produced the same PNG |
| [Mark making](evidence/matrix-marks.md) | Brushes, masks, drawing and rags; numerical boundaries, visual comparisons, retained state and wet/dry contact |
| [Passages and time](evidence/matrix-passages.md) | Hand presets and modifiers, clipping/depth, material stages, actual intermediate aging, palette eviction and exact clock-owner flushing |
| [Space](evidence/matrix-space.md) | Solids, worlds, projection, references, views, visibility/depth clipping and numerical limits |
| [Harness and journal](evidence/matrix-harness.md) | Actual pi tools, compaction/recovery, image limits, token pacing, cancellation, journal write races and real OpenAI/Gemini adapter HTTP payloads sent to local recording endpoints |
| [Viewer](evidence/matrix-viewer.md) | Actual browser controls, playback cadence, polling, focus, delayed/failed images, late results, rebuilt history and actual static export |
| [Delivery](evidence/matrix-delivery.md) | All seven current export profiles, default historical export, finish/video options, concurrent checks, cancellation and actual disk exhaustion |
| [Startup timing](evidence/matrix-timers.json) | Real stopped-server and periodically advancing replay runs; recorded elapsed times distinguish inactivity timeout from total duration |

Shared material interruption probes operate after the native material call returns and before its containing chunk commits. They verify the submitted-chunk contract, not interruption at every instruction within every native algorithm. Browser protocol fixtures exercise the unchanged UI; actual static export and production parser tests are identified separately. Local recording endpoints exercise provider serialization without claiming external model behavior.

## Existing tests

| Test group | Result | Receipt |
|---|---|---|
| Default Rust unit/integration targets | 267 passed after clean session-integrity rerun; 11 explicitly ignored | [Full target output](evidence/rust-tests-canonical.txt), [10-test clean rerun](evidence/rust-integrity-final.txt) |
| Painter-build integration | 3 passed | [Output](evidence/rust-painter.txt) |
| Harness | 51 passed across initial suite plus real-easel case | [Evidence](evidence/runtime-harness.md#existing-harness-tests) |
| Studio server/export | 23 passed, no skips | [Evidence](evidence/runtime-viewer.md#existing-server-and-export-tests) |

The Rust default invocation used `cargo test --workspace --lib --bins --tests --no-fail-fast`. Its recorded exit was nonzero because two integration tests retained obsolete embedded temporary paths after the checkout moved to avoid Unix socket length limits. Cleaning this task's easel test artifacts and rerunning session_integrity at the short canonical path passed all ten tests. No failing product assertion was removed or changed. The table counts each test once using its final applicable result; it does not claim one uninterrupted green invocation. Painter tests used `--no-default-features`. Repository maintenance-script gates and ignored tests were not run.

## Coverage limits

One external-provider run is recorded in the [Space Bunny report](evidence/live-bunny.md). Other providers, a human P1/P2 pass and access from the other Mac remain unclaimed. Provider default 30-minute/24-hour scheduling uses a controlled clock; the actual 60-second token window, configured short retry waits and real startup timing are identified separately. A ten-year simulated wait is not ten years of real elapsed time or physical pigment calibration. Fault interleavings establish the recorded boundaries, not every possible crash instruction.

The [private preview](http://<tailscale-host>:18765/?p=paint-studio-decafe) contains synthetic conversation records and real probe images. It is not a live model painter. Known product findings remain in [triage](../bug-triage.md); the baseline matrix made no source fixes. The later clock fix remains isolated as described in the provider report.
