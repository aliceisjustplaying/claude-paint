# Bug triage

These findings describe the inspected commit, not proposed source changes. “Confirmed” means the narrow observation was reproduced; it does not settle intended behavior. “Suspected” means source evidence needs runtime confirmation. Entries with a product call can be resolved by clarifying the contract rather than changing behavior.

| ID | Severity | Finding | State | Decision |
|---|---|---|---|---|
| B01 | high | Post-execution persistence failure can leave the session refusing normal commands | suspected failure-path limitation | product call |
| B02 | medium | Initial studio-list failure prevents polling startup | suspected | fix |
| B03 | medium | Studio connection failures can look like no painter or stale LIVE | suspected | fix |
| B04 | medium | Provider usage-limit wait lacks prompt abort handling | suspected | fix |
| B05 | medium | Eraser can change a hidden drawing guide without changing sealed visible drawing | suspected | product call |
| B06 | medium | Journal revision history and rewrite are not one atomic update | suspected | product call |
| B07 | low | Brush fullness can exceed the guide's stated 0–1 range | confirmed mismatch | product call |
| B08 | low | Misspelled frame-capture option silently disables capture | confirmed | fix |
| B09 | low | Empty picker keyboard navigation dereferences an absent card | confirmed for Home | fix |
| B10 | low | Nested mask lists differ between visible and behind restrictions | suspected | product call |
| B11 | low | General mask description omits signed and unclamped fields | source-confirmed wording gap | fix documentation |
| B12 | low | Drawing-guide comment promises full coverage on light lines | source-confirmed wording gap | fix documentation |
| B13 | low | Palette-capacity wording obscures retained pile lifetime | source-confirmed wording gap | fix documentation |

## B01 — Persistence failure can restrict every ordinary command

The painter encounters this after a chunk computes successfully but its log or witness cannot be written. Instead of a normal success or a clean Lua rollback, the server can retain state that fails its readiness check. Expected recovery behavior is not specified; the current fail-closed behavior protects history but can trap the session. Reproduction requires an isolated session and a controlled failure between log append and witness rename, followed by status, do and close. It was not induced in this pass.

> Technical note: Cause: `crates/easel/src/main.rs:837` appends and syncs the log, then writes and renames the witness; `main.rs:913` rejects uncommitted state, and `main.rs:924` gates all ordinary commands. Process-crash timing can similarly leave inconsistent files. This is a deliberate integrity design with an unresolved recovery contract, not evidence that normal failed Lua loses work.

Raised by [commands](foundations/commands.md#finishing) and [sessions](foundations/sessions.md#cancel-and-interrupt). Severity high because ordinary commands can become unavailable. Decision: product call on recoverability while preserving integrity.

## B02 — Initial studio-list failure prevents automatic recovery

When the initial sessions response fails or is invalid, the viewer's startup promise rejects before polling timers are installed. The page may remain empty after the server recovers. Expected: an explicit loading error and retry or a documented reload requirement. Reproduction: make the first sessions request fail, restore it and observe whether the page starts without a reload. Unrun.

> Technical note: Cause: `studio/index.html:376` awaits fetch/JSON without recovery; `studio/index.html:824` starts polling only in the success continuation of Promise.all.

Raised by [studio](watching/studio.md#open-questions-and-verification). Severity medium; reloading is a recovery route. Decision: fix.

## B03 — Connection failures have misleading viewer states

Before any events, any event-fetch failure can say the painter has not started, even if the actual problem is the connection. After events exist, failure leaves prior content without a disconnection indicator, potentially still labeled LIVE. Expected: distinguish unavailable data from a known empty history and connected live watching. Reproduction: interrupt event responses before first load and after loading a painter; compare the visible message and badge. Unrun.

> Technical note: Cause: `studio/index.html:409` treats every fetch/parse exception as the waiting-for-painter case when empty, and otherwise returns without connection state.

Raised by [studio](watching/studio.md#cancel-and-interrupt). Severity medium because the user can misread stale state. Decision: fix. This is separate from B02: established polling can continue here.

## B04 — Usage-limit waiting may ignore abort until the timer ends

A painter run waiting after a recognized provider limit can remain inside its wait despite an abort. Expected: cancellation of the wait without needing to kill the whole process. Reproduction: induce a recognized temporary limit with a long wait, abort the pi run and observe when the wait ends. No real provider request was made for this check.

> Technical note: Cause: `harness/painter/painter.ts:140` uses an unconditional setTimeout promise at line 151. The pacing wait at line 111 has explicit abort handling; the usage-limit wait does not.

Raised by [harness](session/harness.md#cancel-and-interrupt). Severity medium; a process stop is recoverable but the run may otherwise wait for a long interval. Decision: fix, subject to runtime confirmation of pi's outer lifecycle.

## B05 — Eraser can reduce a guide beneath sealed paint

After drawing and painting over an unfixed line, erasing can reduce the line's guide mask even where visible graphite is protected by paint. Expected is a product choice: either the guide tracks the sealed drawing or it intentionally remains separately erasable. Reproduction: record guide samples, paint over the line, erase the same area, then compare guide samples and image. Unrun.

> Technical note: Cause: `crates/paint/src/graphite.rs:623` updates the guide before the paint-sealing checks used for visible drawing cells at line 642.

Raised by [drawing](painting/drawing.md#open-questions-and-verification). Severity medium because later painting using the guide can differ without a visible drawing change. Decision: product call.

## B06 — Journal revision can record an edit that did not finish

A harness revision records the old journal and intended replacement before overwriting the journal. Storage failure can leave the history record without the rewrite; a concurrent file edit after the read can be overwritten. Expected: clarify whether revisions are best-effort audit attempts or completed atomic edits. Reproduction requires an isolated journal with a controlled rewrite failure or concurrent edit. Unrun.

> Technical note: Cause: `harness/painter/journal.ts:17` reads and matches, then appends history at line 29 and writes the journal at line 30, without an atomic cross-file transaction or conflict check.

Raised by [journal](session/journal.md#finishing). Severity medium because notes can diverge from revision intent. Decision: product call; source evidence alone does not prove normal revision failure.

## B07 — Fullness exceeds the documented range

Loading a fresh brush twice produced fullness 1.928667426109314 in the isolated probe. The guide describes fullness as 0–1. Expected: either a nominal capacity description that permits overfilling or enforced capacity. [Additional evidence](verification/evidence/additional.md) records the exact submitted chunk and values.

> Technical note: Cause: `crates/paint/src/bristle.rs:429` adds the new load to existing paint; `bristle.rs:456` divides total paint by nominal capacity without clamping. `notes/easel_guide.md:119` describes a 0–1 return. `crates/easel/src/api.rs:385` also lacks explicit finite/range validation for load amount, a related untested boundary concern.

Raised by [brushes](painting/brushes.md#open-questions-and-verification). Severity low for the confirmed reporting-contract mismatch; no corruption from invalid amounts is claimed. Decision: product call.

## B08 — Unknown frame option means off

`easel frames typo` returned success and `frames off`. Expected: an unknown option error preserving the previous setting. Reproduction and output are in [additional evidence](verification/evidence/additional.md).

> Technical note: Cause: `crates/easel/src/main.rs:968` treats only `on` or no argument as enabled and all other first arguments as disabled.

Raised by [delivery](delivery/replay.md#edge-cases). Severity low because painting history remains intact, though automatic images can be missed. Decision: fix.

## B09 — Empty picker has no keyboard target

With no painter cards, Arrow, Home or End handling attempts to focus an absent element. Expected: an empty picker remains operable without a JavaScript exception. Reproduction: serve an empty session directory, open the picker, focus its checkbox and press Home. This produced an uncaught exception in [the browser probe](verification/evidence/browser.md#empty-picker-probe). Arrow and End remain unrun variants.

> Technical note: Cause: `studio/index.html:365` builds the card list, then lines 371–372 call focus without checking that it is nonempty.

Raised by [studio](watching/studio.md#open-questions-and-verification). Severity low; closing the dialog is still separately handled. Decision: fix.

## B10 — Nested behind-mask list behaves differently

A nested mixed list accepted for visible restrictions is not accepted the same way for behind restrictions. Expected: either consistent nested-list support or explicit documentation of the distinction. Reproduction: submit equivalent restrictions using a top-level mask and the same mask nested inside another list. Unrun.

> Technical note: Cause: `crates/easel/src/depth.rs:105` recursively extracts visible masks, while line 272 partitions only the outer behind list and nested masks reach a names-only parser.

Raised by [space](painting/space.md#open-questions-and-verification). Severity low; flattening the mask list is a recoverable alternative. Decision: product call.

## B11 — Mask description omits signed and unclamped fields

The guide describes masks as 0–1 coverage, while distance returns signed values and map does not clamp its result. Expected: distinguish a distance/value field from ordinary coverage so the painter knows whether it is ready to use for paint.

> Technical note: `notes/easel_guide.md:227`, `crates/paint/src/mask.rs:162` and `crates/easel/src/api.rs:662` establish the mismatch.

Raised by [shapes](painting/shapes.md#open-questions-and-verification). Severity low. Decision: fix documentation; field operations can remain intentional.

## B12 — Drawing-guide comment overstates light-line coverage

The wrapper describes the drawing guide as 1 on a line; the engine retains lower coverage for light lines. Expected: describe continuous coverage rather than a guaranteed full-strength path.

> Technical note: `crates/easel/src/draw_pencil.rs:318` and `crates/paint/src/graphite.rs:700` provide the conflicting descriptions.

Raised by [drawing](painting/drawing.md#open-questions-and-verification). Severity low. Decision: fix documentation. Runtime intensity sampling remains unrun.

## B13 — Palette-capacity wording obscures retained pile lifetime

“Oldest pile scraped off” can be read as deletion of a retained Lua pile. The 16-entry limit belongs to the recent palette-trip ledger; retained pile values still exist, and a later load can incur new mixing time. Expected: explain palette-trip cost separately from variable lifetime.

> Technical note: `notes/easel_guide.md:78`, `crates/paint/src/tally.rs:103`, `crates/easel/src/time.rs:104` and `crates/easel/src/api.rs:493` establish the two representations.

Raised by [canvas](foundations/canvas.md#edge-cases) and [time](painting/time.md#edge-cases). Severity low. Decision: fix documentation. A 17-pile lifetime probe remains unrun.

The viewer's separate “working now” and resting/finished timing thresholds remain an open product terminology question in its document, not a confirmed defect. No upstream issue or source change was made.
