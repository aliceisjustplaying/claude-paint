# Bug triage

These findings describe the inspected commit, not proposed source changes. “Confirmed” means the narrow observation was reproduced; it does not settle intended behavior. “Suspected” means source evidence needs runtime confirmation. Entries with a product call can be resolved by clarifying the contract rather than changing behavior.

| ID | Severity | Finding | State | Decision |
|---|---|---|---|---|
| B01 | high | Post-execution persistence failure can leave the session refusing normal commands | confirmed failure path | product call |
| B02 | medium | Initial studio-list failure prevents polling startup | confirmed | fix |
| B03 | medium | Studio connection failures can look like no painter or stale LIVE | confirmed | fix |
| B04 | medium | Provider usage-limit wait lacks prompt abort handling | confirmed | fix |
| B05 | medium | Eraser can change a hidden drawing guide without changing sealed visible drawing | confirmed | product call |
| B06 | medium | Journal revision history and rewrite are not one atomic update | confirmed | product call |
| B14 | medium | Failed chunks change checkpoint counters and produce a false replay mismatch | confirmed | fix |
| B16 | medium | Default studio export builds an incompatible historical easel | confirmed | fix |
| B18 | medium | Infinite retry delay becomes immediate retries | confirmed | fix |
| B19 | medium | Opening keeps polling after an integrity error during replay | confirmed bounded reproduction | fix |
| B20 | medium | Rewound viewer position is invalid after history replacement | confirmed | fix |
| B07 | low | Brush fullness can exceed the guide's stated 0–1 range | confirmed mismatch | product call |
| B08 | low | Misspelled frame-capture option silently disables capture | confirmed | fix |
| B09 | low | Empty picker keyboard navigation dereferences an absent card | confirmed for six keys | fix |
| B10 | low | Nested mask lists differ between visible and behind restrictions | confirmed | product call |
| B11 | low | General mask description omits signed and unclamped fields | runtime-confirmed wording gap | fix documentation |
| B12 | low | Drawing-guide comment promises full coverage on light lines | runtime-confirmed wording gap | fix documentation |
| B13 | low | Palette-capacity wording obscures retained pile lifetime | runtime-confirmed wording gap | fix documentation |
| B15 | low | Broadcast stream header overflows at phone width | confirmed layout; support scope open | product call |
| B17 | low | Picker stays open when Tab leaves the document | confirmed browser boundary | fix or contract clarification |
| B21 | low | Description treated aging threshold as exact scheduling | description corrected | resolved documentation |
| B22 | low | Nonshrinking history rewrite can retain stale parsed prefix | confirmed; support scope unspecified | product call |
| B23 | low | Painting clock stays absent without explicit timestamp output | isolated fix verified; not integrated or deployed | fix |

## B01 — Persistence failure can restrict every ordinary command

The painter encounters this after a chunk computes successfully but its log or witness cannot be written. Instead of a normal success or a clean Lua rollback, the server can retain state that fails its readiness check. Expected recovery behavior is not specified; the current fail-closed behavior protects history but can trap the session. Reproduction requires an isolated session and a controlled failure between log append and witness rename, followed by status, do and close. The [runtime probe](verification/evidence/runtime-cli.json) made the pending witness path a directory. The log advanced, the witness did not and status/do/close refused. Restoring baseline log bytes still left uncommitted memory; restarting from the preserved original files recovered the baseline. This is a disposable recovery experiment, not a supported recovery procedure.

> Technical note: Cause: `crates/easel/src/main.rs:837` appends and syncs the log, then writes and renames the witness; `main.rs:913` rejects uncommitted state, and `main.rs:924` gates all ordinary commands. Process-crash timing can similarly leave inconsistent files. This is a deliberate integrity design with an unresolved recovery contract, not evidence that normal failed Lua loses work.

Raised by [commands](foundations/commands.md#finishing) and [sessions](foundations/sessions.md#cancel-and-interrupt). Severity high because ordinary commands can become unavailable. Decision: product call on recoverability while preserving integrity.

## B02 — Initial studio-list failure prevents automatic recovery

When the initial sessions response fails or is invalid, the viewer's startup promise rejects before polling timers are installed. The page may remain empty after the server recovers. Expected: an explicit loading error and retry or a documented reload requirement. Reproduction: make the first sessions request fail, restore it and observe whether the page starts without a reload. The [browser probe](verification/evidence/runtime-viewer.md#startup-failure-bug-02-reproduced) observed no retry or recovery for 23 seconds after service restoration; reload recovered it.

> Technical note: Cause: `studio/index.html:376` awaits fetch/JSON without recovery; `studio/index.html:824` starts polling only in the success continuation of Promise.all.

Raised by [studio](watching/studio.md#open-questions-and-verification). Severity medium; reloading is a recovery route. Decision: fix.

## B03 — Connection failures have misleading viewer states

Before any events, any event-fetch failure can say the painter has not started, even if the actual problem is the connection. After events exist, failure leaves prior content without a disconnection indicator, potentially still labeled LIVE. Expected: distinguish unavailable data from a known empty history and connected live watching. Reproduction: interrupt event responses before first load and after loading a painter; compare the visible message and badge. The [browser probe](verification/evidence/runtime-viewer.md#event-failure-bug-03-reproduced) reproduced the false initial waiting message and a stale FINISHED badge after later failures; stale LIVE specifically remains untested.

> Technical note: Cause: `studio/index.html:409` treats every fetch/parse exception as the waiting-for-painter case when empty, and otherwise returns without connection state.

Raised by [studio](watching/studio.md#cancel-and-interrupt). Severity medium because the user can misread stale state. Decision: fix. This is separate from B02: established polling can continue here.

## B04 — Usage-limit waiting may ignore abort until the timer ends

A painter run waiting after a recognized provider limit can remain inside its wait despite an abort. Expected: cancellation of the wait without needing to kill the whole process. Reproduction: induce a recognized temporary limit with a long wait, abort the pi run and observe when the wait ends. The [actual pi RPC probe](verification/evidence/runtime-harness.md#usage-limit-abort-bug04) used a local provider and a three-second wait: abort was acknowledged only when the timer ended, about three seconds after the request. No external provider was contacted.

> Technical note: Cause: `harness/painter/painter.ts:140` uses an unconditional setTimeout promise at line 151. The pacing wait at line 111 has explicit abort handling; the usage-limit wait does not.

Raised by [harness](session/harness.md#cancel-and-interrupt). Severity medium; a process stop is recoverable but the run may otherwise wait for a long interval. Decision: fix; delayed outer-lifecycle cancellation was reproduced.

## B05 — Eraser can reduce a guide beneath sealed paint

After drawing and painting over an unfixed line, erasing can reduce the line's guide mask even where visible graphite is protected by paint. Expected is a product choice: either the guide tracks the sealed drawing or it intentionally remains separately erasable. Reproduction: record guide samples, paint over the line, erase the same area, then compare guide samples and image. The [runtime probe](verification/evidence/runtime-paint.md) reduced guide area from 4003.0022 to 3133.6022 while the PNG remained byte-identical.

> Technical note: Cause: `crates/paint/src/graphite.rs:623` updates the guide before the paint-sealing checks used for visible drawing cells at line 642.

Raised by [drawing](painting/drawing.md#open-questions-and-verification). Severity medium because later painting using the guide can differ without a visible drawing change. Decision: product call.

## B06 — Journal revision can record an edit that did not finish

A harness revision records the old journal and intended replacement before overwriting the journal. Storage failure can leave the history record without the rewrite; a concurrent file edit after the read can be overwritten. Expected: clarify whether revisions are best-effort audit attempts or completed atomic edits. Reproduction requires an isolated journal with a controlled rewrite failure or concurrent edit. The [runtime probe](verification/evidence/runtime-harness.md#journal-history-before-denied-rewrite-bug06) denied the journal rewrite: EACCES left original bytes while the intended revision had already been appended. The [controlled production-helper interleaving](verification/evidence/matrix-harness.md#concurrent-journal-boundary) also lost a second revision and external append when the first revision rewrote its stale snapshot.

> Technical note: Cause: `harness/painter/journal.ts:17` reads and matches, then appends history at line 29 and writes the journal at line 30, without an atomic cross-file transaction or conflict check.

Raised by [journal](session/journal.md#finishing). Severity medium because notes can diverge from revision intent. Decision: product call; the denied-write case is confirmed; ordinary successful edits also passed.

## B07 — Fullness exceeds the documented range

Loading a fresh brush twice produced fullness 1.928667426109314 in the isolated probe. The guide describes fullness as 0–1. Expected: either a nominal capacity description that permits overfilling or enforced capacity. [Additional evidence](verification/evidence/additional.md) records the exact submitted chunk and values.

> Technical note: Cause: `crates/paint/src/bristle.rs:429` adds the new load to existing paint; `bristle.rs:456` divides total paint by nominal capacity without clamping. `notes/easel_guide.md:119` describes a 0–1 return. `crates/easel/src/api.rs:385` also lacks explicit finite/range validation for load amount, a related untested boundary concern.

Raised by [brushes](painting/brushes.md#open-questions-and-verification). Severity low for the confirmed reporting-contract mismatch; no corruption from invalid amounts is claimed. Decision: product call.

## B08 — Unknown frame option means off

`easel frames typo` returned success and `frames off`. Expected: an unknown option error preserving the previous setting. Reproduction and output are in [additional evidence](verification/evidence/additional.md).

> Technical note: Cause: `crates/easel/src/main.rs:968` treats only `on` or no argument as enabled and all other first arguments as disabled.

Raised by [delivery](delivery/replay.md#edge-cases). Severity low because painting history remains intact, though automatic images can be missed. Decision: fix.

## B09 — Empty picker has no keyboard target

With no painter cards, Arrow, Home or End handling attempts to focus an absent element. Expected: an empty picker remains operable without a JavaScript exception. Reproduction: serve an empty session directory, open the picker, focus its checkbox and press Home. This produced an uncaught exception in [the browser probe](verification/evidence/browser.md#empty-picker-probe). The [broader browser pass](verification/evidence/runtime-viewer.md#empty-picker-bug-09-reproduced-for-every-navigation-key) reproduced all four arrows, Home and End with focus inside the empty picker.

> Technical note: Cause: `studio/index.html:365` builds the card list, then lines 371–372 call focus without checking that it is nonempty.

Raised by [studio](watching/studio.md#open-questions-and-verification). Severity low; closing the dialog is still separately handled. Decision: fix.

## B10 — Nested behind-mask list behaves differently

A nested mixed list accepted for visible restrictions is not accepted the same way for behind restrictions. Expected: either consistent nested-list support or explicit documentation of the distinction. Reproduction: submit equivalent restrictions using a top-level mask and the same mask nested inside another list. The [runtime probe](verification/evidence/runtime-paint.md) accepted top-level behind and nested visible masks but rejected the nested behind mask.

> Technical note: Cause: `crates/easel/src/depth.rs:105` recursively extracts visible masks, while line 272 partitions only the outer behind list and nested masks reach a names-only parser.

Raised by [space](painting/space.md#open-questions-and-verification). Severity low; flattening the mask list is a recoverable alternative. Decision: product call.

## B11 — Mask description omits signed and unclamped fields

The guide describes masks as 0–1 coverage, while distance returns signed values and map does not clamp its result. Expected: distinguish a distance/value field from ordinary coverage so the painter knows whether it is ready to use for paint.

> Technical note: `notes/easel_guide.md:227`, `crates/paint/src/mask.rs:162` and `crates/easel/src/api.rs:662` establish the mismatch.

Raised by [shapes](painting/shapes.md#open-questions-and-verification). Severity low. Decision: fix documentation; field operations can remain intentional. The [runtime probe](verification/evidence/runtime-paint.md) measured positive/negative distance and map output 2.

## B12 — Drawing-guide comment overstates light-line coverage

The wrapper describes the drawing guide as 1 on a line; the engine retains lower coverage for light lines. Expected: describe continuous coverage rather than a guaranteed full-strength path.

> Technical note: `crates/easel/src/draw_pencil.rs:318` and `crates/paint/src/graphite.rs:700` provide the conflicting descriptions.

Raised by [drawing](painting/drawing.md#open-questions-and-verification). Severity low. Decision: fix documentation. The [runtime probe](verification/evidence/runtime-paint.md) sampled light guide coverage 0.65536 and firm coverage 0.97792.

## B13 — Palette-capacity wording obscures retained pile lifetime

“Oldest pile scraped off” can be read as deletion of a retained Lua pile. The 16-entry limit belongs to the recent palette-trip ledger; retained pile values still exist, and a later load can incur new mixing time. Expected: explain palette-trip cost separately from variable lifetime.

> Technical note: `notes/easel_guide.md:78`, `crates/paint/src/tally.rs:103`, `crates/easel/src/time.rs:104` and `crates/easel/src/api.rs:493` establish the two representations.

Raised by [canvas](foundations/canvas.md#edge-cases) and [time](painting/time.md#edge-cases). Severity low. Decision: fix documentation. The [runtime probe](verification/evidence/runtime-paint.md) retained and loaded the first pile after 18 later mixture trips; the additional [production ledger probe](verification/evidence/matrix-ledger.txt) measured2.5seconds for retained-color reload before eviction and22.5seconds after the17th distinct color (20seconds mixing plus2.5seconds reload).

The viewer's separate “working now” and resting/finished timing thresholds remain an open product terminology question in its document, not a confirmed defect. No upstream issue or source change was made.

## B14 — Failed chunks cause checkpoint comparison failures

A failed second chunk leaves the painting PNG and successful source log unchanged but changes internal checkpoint counters. The runner then reports `check: DIFFERS from the live save` even though the painting pixels match. Expected: a rolled-back attempt should not make the saved state falsely disagree with replay. In five identical one-chunk studios, the no-failure control passed; separate failed varnish, cracks, relief and plain-error attempts all produced the mismatch. [Controlled evidence](verification/evidence/runtime-delivery.md#isolated-failed-chunk-checkpoint-discrepancy).

> Technical note: `crates/easel/src/save.rs:52–53` serializes chunk/call counters; `session.rs:280` resets them before execution, but `session.rs:202–222` omits them from snapshot/restore. The plain-error checkpoint differed only at bytes 110 and 118: chunk/calls 1/1 became 2/0. Canvas payload bytes were identical. No lost-paint claim follows from this result.

Raised by [commands](foundations/commands.md#finishing) and [delivery](delivery/replay.md#finishing). Severity medium because the runner reports a reproducibility failure after an ordinary rejected command. Decision: fix counter restoration or explicitly exclude nonsemantic counters from the comparison contract.

## B15 — Stream header overflows at phone width

At a 390-pixel viewport with `stream=1`, the header's scroll width was 459 pixels and the painter picker shrank to 18 pixels. The ordinary public page did not overflow at that width. Expected behavior depends on whether the broadcast-specific layout supports phone viewing; that scope is not established. [Browser evidence and screenshot](verification/evidence/runtime-viewer.md#expanded-static-and-public-browser-pass).

> Technical note: `studio/stream.css:34–38` enlarges clock/painter text and adds a nonshrinking site label to the header. This is source support for the width pressure; no single-rule repair has been isolated.

Raised by [studio](watching/studio.md#open-questions-and-verification). Severity low. Decision: product call on supported stream viewport widths.

## B16 — Default studio export builds an incompatible historical easel

The current export script without `R16_BRANCH` selects `round-16`, builds it then fails with `easel: no command "tubes"` and `the exported easel can't name its box`. It leaves a partial destination. Expected: the default invocation delivers a runnable studio or rejects an incompatible revision before assembling it. The actual default run resolved `round-16` to `f48e722f8c9a916d3d91b54bc8be0320d0bfd16f`; all seven explicit-current profiles exported successfully. [Runtime reproduction](verification/evidence/matrix-delivery.md#default-historical-export).

> Technical note: `scripts/export_r16_studio:77` selects the historical default; line155 requires `tubes --markdown`, which that historical executable lacks.

Raised by [delivery verification](verification/delivery.md). Severity medium: default export fails, with an explicit compatible revision working. Decision: fix the default revision or compatibility requirement.

## B17 — Picker can stay open when Tab leaves the document

With the picker open and the last painter card focused, native Tab moved focus out of the document. Subsequent Tab reached header and previous-look controls while the picker remained open with `aria-expanded=true`. Expected under the documented Tab-away behavior: dismissal when focus leaves the picker. Direct outside pointer dismissal worked. [Native browser reproduction](verification/evidence/matrix-viewer.md#picker-focus-boundary).

> Technical note: `studio/index.html:375` checks a non-null `relatedTarget` before dismissing. Document-boundary focus loss can supply null, so this path remains open. This observation is specific to the exercised browser traversal, not all browsers.

Raised by [studio verification](verification/viewer-space.md). Severity low. Decision: fix dismissal handling or narrow the documented keyboard contract.

## B18 — Infinite retry delay becomes immediate retries

`PAINTER_LIMIT_PROBE_S=Infinity` passes startup validation and prints “asking again in Infinity min”, but two real local-provider limit responses retried after approximately 18 and 5 ms. Expected: reject a nonfinite delay rather than silently make repeated requests. A successful third response ended this bounded probe; no unbounded request storm was run. [Actual pi reproduction](verification/evidence/matrix-harness.md#actual-pacing-and-setting-boundaries).

> Technical note: `harness/painter/limits.ts:44–50` validates nonnegative numbers without checking finiteness. Passing the resulting infinite delay to the runtime timer does not preserve the advertised delay.

Raised by [harness verification](verification/session.md). Severity medium because malformed configuration can repeatedly retry against a limit. Decision: fix finite-number validation.

## B19 — Opening can keep polling after replay becomes integrity-restricted

An external edit after replay captured the log allowed replay to finish, but the initial `open` did not return the resulting integrity error. The server kept answering unsuccessful status requests while the opening client waited. Expected: return the actionable readiness error rather than keep treating it as startup progress. The bounded reproduction observed 4.04 seconds, replay completion at 1.22 seconds and 27 unsuccessful status lines before terminating the opening client. [Raw reproduction](verification/evidence/matrix-open-error/root.json).

> Technical note: `crates/easel/src/main.rs:498` accepts only successful status responses in the startup loop, discarding error bodies. Lines489–496 reset `last_progress` on any newly logged line, including the status failures generated by this polling. Thus the code's stall timer does not bound this repeating-error path. That unbounded implication is from source; the runtime observation was four seconds.

Raised by [session verification](verification/session.md). Severity medium: the opening caller waits without the error already available to a separate status caller. Decision: return readiness errors and count replay progress separately from diagnostic traffic.

## B20 — Rewound viewer position becomes stale when history changes

While rewound to event 14 of 16, replacing the server history with a new two-event history cleared the image and changed the scrubber maximum to 1, but retained the old position text and threw while reading an absent event's timestamp. Expected: a valid position in the replacement history and a coherent rendered view. A simpler append while rewound updated the code/result and scrubber range but retained the old total in `2/15` after history grew to 16. [Actual browser receipts](verification/evidence/matrix-viewer.md#late-result-and-rebuilt-history).

> Technical note: `studio/index.html:396` resets history, line415 handles the changed epoch and line432 only calls `show` when live-follow is enabled. The retained position reaches `events[pos].ts` at line521 without being reconciled to the shorter history.

Raised by [studio verification](verification/viewer-space.md). Severity medium because a supported history rebuild while rewound produces an exception and an invalid display. Decision: reconcile the selection and counter on history replacement and append.

## B21 — Description overstated exact aging-slice duration

The product description originally said long passes age in15-minute slices. Actual intermediate frames occurred after about 950–978 seconds of additional handwork, because the 15-minute threshold is checked at completed work boundaries. Expected documentation: a threshold rather than exact 900-second scheduling. [Observed slice times](verification/evidence/matrix-passages.md#intermediate-aging-slices).

> Technical note: `crates/easel/src/time.rs:21` sets the 15-minute threshold; production work reaches the threshold at mark boundaries. This is a defect in this description's precision, not an established product failure.

Raised by [time row T058](verification/passages-time.md). Severity low. Decision: documentation corrected in [time](painting/time.md#edge-cases); the failed original claim remains recorded as provenance.

## B22 — Equal-size or growing history rewrites can stay stale

After a production studio server parsed a history, replacing an earlier result with longer text left that server returning the old result. A newly started production server reading the same file returned the replacement. [Recorded file/server comparison](verification/evidence/matrix-viewer-parser.json). Expected behavior depends on whether in-place history editing is supported; normal append-only history is unaffected by this finding.

> Technical note: `studio/studio.py:259–264` resets the parser only when file size falls below its saved offset. Equal-size or larger replacement files retain the old parsed prefix and are read only from the previous offset.

Raised by [viewer verification](verification/viewer-space.md). Severity low for this unsupported-or-unspecified edit path. Decision: product call on supported history rewrites, with documentation that distinguishes appending, truncation and in-place replacement.

## B23 — Painting clock can stay absent during painting

A viewer can watch successful painting without seeing the painting's own clock. Expected: recorded painting time is visible during painting without requiring the model to print it explicitly. Reproduction: create a canvas and advance time without printing a timestamp, then inspect the viewer clock. The real Space Bunny run reproduced the absence; the follow-up probe verified an isolated fix. [Report and receipts](verification/evidence/live-bunny.md).

> Technical note: At baseline `4e525e5`, `crates/easel/src/main.rs:928` returns successful Lua output without an automatic timestamp. `studio/index.html:449` searches successful paint output for a timestamp and `studio/index.html:524` hides the clock when none exists. The parser also did not expose clock metadata from compaction/recovery summaries. The [patch](verification/evidence/live-bunny/painting-clock.patch) changes these boundaries.

Raised by the live viewer observation and [studio description](watching/studio.md#open-questions-and-verification). Severity low: missing status display, with no lost painting state observed. Decision: fix. Local commit `2a705f4` is preserved separately; no integration or deployment occurred. [Preservation record](verification/evidence/live-bunny/preservation.json).
