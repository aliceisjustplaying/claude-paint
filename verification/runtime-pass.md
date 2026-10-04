# Broad runtime pass

The 2026-10-04 agent pass exercised every documented area through selected CLI, browser, harness and delivery scenarios. It found a new failed-chunk checkpoint defect, reproduced existing suspicions and corrected inaccurate documentation expectations. It did not complete every checklist scenario. Source was pinned to `4e525e50897807e9b5f734071dfeb330f1a393d3` in a clean, disposable checkout; no product source or tests were changed.

## Results

| Checklist cluster | Pass | Fail | Blocked or partial |
|---|---:|---:|---:|
| Canvas and views | 38 | 0 | 122 |
| Mark making | 40 | 0 | 220 |
| Passages and time | 41 | 0 | 103 |
| Commands, sessions, journal and harness | 51 | 0 | 124 |
| Viewer and space | 65 | 0 | 101 |
| Delivery | 15 | 0 | 78 |
| Intended behavior in bug triage | 4 | 9 | 2 |
| Total | 254 | 9 | 750 |

A feature row can pass because the observed product matches its description of a defect; the intended-behavior row in triage then fails. These are checklist assertions, not counts of independent test executions. Blocked includes unrun scenarios and partially checked scenarios; it does not mean every row needs unavailable access. Many remaining rows require additional controlled comparisons, interruption timing, extreme inputs or option combinations. The pass does not relabel any feature as human-verified.

## Executed evidence

- [Commands, canvas and views](evidence/runtime-cli.md): setup validation, material bounds, whole-chunk rollback, session selection, queued versus running disconnect, accepted-file capture, post-execution persistence failure, image transforms, crop limits and output-write failure. The real [ten-minute Lua deadline](evidence/runtime-deadline.json) stopped at 600.010844 seconds and the next command succeeded.
- [Mark making](evidence/runtime-paint.md): tools, masks, drawing and cloth, sampled values, retained state, image rollback and visually inspected marks.
- [Passages and time](evidence/runtime-passages.md): all hand presets, seeded output, clipping pixel counts, wet/dry blending, simulated waiting and independent clocks.
- [Space](evidence/runtime-space.md): solids/worlds, projection, references, explicit/default views, visibility/depth clipping and replay.
- [Harness and journal](evidence/runtime-harness.md): installed pi with local recording providers, all six configured tools, real painter executable, compaction/recovery, usage-limit abort and denied journal rewrite. No external model provider was contacted.
- [Studio](evidence/runtime-viewer.md): actual production browser with controlled HTTP faults, keyboard navigation, late records, stale responses, zoom, static export refresh, public mode and stream layouts.
- [Delivery](evidence/runtime-delivery.md): runner comparison, checkpoint finishing, replay fallback, companion equivalence, H.264 video checked with ffprobe and a pinned blank-studio export. [Video artifact](evidence/runtime-delivery.mp4).

## Existing tests

| Test group | Result | Receipt |
|---|---|---|
| Default Rust unit/integration targets | 267 passed after clean session-integrity rerun; 11 explicitly ignored | [Full target output](evidence/rust-tests-canonical.txt), [10-test clean rerun](evidence/rust-integrity-final.txt) |
| Painter-build integration | 3 passed | [Output](evidence/rust-painter.txt) |
| Harness | 51 passed across initial suite plus real-easel case | [Evidence](evidence/runtime-harness.md#existing-harness-tests) |
| Studio server/export | 23 passed, no skips | [Evidence](evidence/runtime-viewer.md#existing-server-and-export-tests) |

The Rust default invocation used `cargo test --workspace --lib --bins --tests --no-fail-fast`. Its recorded exit was nonzero because two integration tests retained obsolete embedded temporary paths after the checkout moved to avoid Unix socket length limits. Cleaning this task's easel test artifacts and rerunning session_integrity at the short canonical path passed all ten tests. No failing product assertion was removed or changed. The table counts each test once using its final applicable result; it does not claim one uninterrupted green invocation. Painter tests used `--no-default-features`. Repository maintenance-script gates and ignored tests were not run.

## Findings and corrections

[B14](../bug-triage.md#b14--failed-chunks-cause-checkpoint-comparison-failures) is new: a failed chunk leaves internal chunk/call counters changed, causing runner checkpoint comparison to report DIFFERS despite identical pixels and successful source. The controlled plain-error checkpoint differed at only two bytes; the canvas payload was identical.

The pass also reproduced restricted persistence recovery, startup/event-fetch failures, delayed usage-limit abort, sealed drawing/guide divergence, journal revision-before-failure, all empty-picker navigation keys and nested behind-mask behavior. [Triage](../bug-triage.md) retains product decisions separately from observations. [B15](../bug-triage.md#b15--stream-header-overflows-at-phone-width) records stream-header overflow at 390 pixels; whether broadcast layout supports that width remains open.

Descriptions/checklists were corrected for empty chunks being rejected, permissive unknown brush-constructor fields, floating-point dimensions and six-tool restriction being supplied by the runner rather than the painter extension alone.

## Remaining limits

The full checklist matrix remains incomplete: 750 rows are partial or unrun. Examples include every option combination, native-operation deadline boundaries, long provider/rebuild budgets, simultaneous journal writers, disk exhaustion, all export profiles/historical refs and exact timing/appearance boundaries. No external-provider session, full human P1/P2 pass or connection from the other Mac was tested. The retained private preview uses synthetic conversation records with actual probe images, not a running model painter.
