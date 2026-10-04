# Harness and journal runtime verification

Source commit: `4e525e50897807e9b5f734071dfeb330f1a393d3`. Run on 2026-10-04 against a clean source checkout, using installed Node and pi. No production source or tests were changed. All studios, sessions and journal files were disposable. Providers in these checks were local recording fixtures; no external model request was made.

## Existing harness tests

Executed `TMPDIR=<scratch> node --test harness/painter/test/*.test.ts`.

Result: 51 tests, 50 passed, zero failures and one skipped, in 6.043 seconds. The skipped case was `paint, look, note and status at a real easel`, because its `EASEL_BIN` painter-build prerequisite was not supplied in this initial run.

The passing production-module checks covered:

- Studio path resolution, including outside paths, symlinks, file URLs and names beginning with dots.
- Client process timeout, abort, listener removal, rebuild progress admission, decreasing/invalid progress, stalled progress, fresh ordinary budget after advancing rebuild and early executable exit.
- Child environment isolation and forwarding of `RAYON_NUM_THREADS`.
- Removal of numbered/compute-time presentation and random naming of look files.
- Exact journal replacement, complete revision history and rejection of absent, duplicated, empty or missing-file targets.
- Image count and size pruning, five-image batches, oversized newest images, path/generic placeholders and unchanged input message objects.
- Token pacing calculations, cached-input accounting and per-minute quota classification.
- Summary quoting, live globals, unavailable-easel reporting and valid clock evidence.
- OpenAI and Gemini image payload transformations.

The suite also launched the actual installed pi with local faux-provider fixtures: two usage-limit failures followed by success preserved the original model conversation while retaining errors in the stored session; later-sitting recovery inserted studio material and a canvas image; a resumed session compacted and inserted the summary-time image. These use executable easel fixtures and do not establish actual model-provider availability or painting appearance.

Receipts: [client tests](../../../claude-paint/harness/painter/test/easel-client.test.ts), [image tests](../../../claude-paint/harness/painter/test/context-images.test.ts), [journal tests](../../../claude-paint/harness/painter/test/journal.test.ts), [limit tests](../../../claude-paint/harness/painter/test/limits.test.ts), [recovery tests](../../../claude-paint/harness/painter/test/recovery.test.ts), [summary tests](../../../claude-paint/harness/painter/test/summary.test.ts), [pacing tests](../../../claude-paint/harness/painter/test/pace.test.ts), [vision tests](../../../claude-paint/harness/painter/test/vision-payload.test.ts).

## Usage-limit abort: BUG04

Launched actual pi RPC with the production `painter.ts`, existing `faux-limit.ts` provider and `PAINTER_LIMIT_PROBE_S=3`. After stderr announced the first usage-limit wait, sent `{"id":"abort","type":"abort"}`. Queried state 200 ms later. The observation clock begins at child launch:

| Time | Observation |
|---|---|
| 298 ms | Usage-limit wait announced; abort sent immediately |
| 501 ms | `get_state` returned `isStreaming: true` |
| 3300 ms | `agent_settled` emitted; abort response acknowledged |

The run remained unsettled for approximately 3.002 seconds after abort, matching the configured wait. No second provider request occurred after the abort. This confirms delayed cancellation of the usage-limit wait, not continued model work after cancellation. The check used real wall time, not an overridden timer. The 30-minute default was not waited out.

Production receipt: [usage-limit settle hook](../../../claude-paint/harness/painter/painter.ts:134).

## Journal history before denied rewrite: BUG06

Created `notes/journal.md` with bytes `original unique passage\n`, then made only that file read-only (mode `0444`). Called the production `reviseJournal(studio, "unique", "changed")` with its revision-history directory writable.

Observed:

```text
error code: EACCES
journal bytes: "original unique passage\n"
revision record: {"at":"2026-10-04T11:36:28.955Z","replaced":"unique","with":"changed","before":"original unique passage\n"}
```

Thus history recorded the attempted replacement even though the journal rewrite failed. Permissions were restored after inspection. This confirms the split-write observation, without deciding whether recording an attempted revision is intended product behavior. No disk-full or mid-write truncation was simulated.

Production receipt: [revision append followed by rewrite](../../../claude-paint/harness/painter/journal.ts:29).

## Additional journal cases

Direct calls to the unchanged production `reviseJournal` used separate disposable files:

| Operation | Result |
|---|---|
| Replace `unique` with itself in `one unique passage\n` | Journal unchanged; revision record added |
| Replace `unique ` with empty text | Exact deletion produced `one passage\n` |
| Externally rewrite file to `externally changed passage\n`, then replace `externally changed` with `current` | Current file read; output `current passage\n` |

The three successful operations produced three history records. These establish helper behavior; they do not exercise simultaneous writers or interruption between filesystem operations.

## Limits

Full-duration 12-minute/3-minute client budgets, 30-minute rebuild stalls, 24-hour give-up and default usage waits were not elapsed. Existing tests cover bounded versions and pure delay calculations. Simultaneous journal writers, disk exhaustion, a provider network outage and model response quality remain untested. No claim here establishes live external-provider credentials or access.

## Actual painter tools through pi

Built painter executable (`cargo build --release -p easel --no-default-features`) was copied to a disposable short-path studio's `bin/easel`. The installed pi loaded the unchanged painter extension and a local provider that emitted structured tool calls. The launch used `--tools paint,look,note,status,log,read`, matching [the runner](../../../claude-paint/notes/round21.1/runner/r21_chains.py:122). This restriction is a launcher requirement: an initial probe without `--tools` also exposed pi's default bash, edit and write tools. The extension alone does not remove them.

The stored system message in the correctly restricted run had exactly the six requested tool schemas. Its sections were `preamble` containing `Only the fixture prompt.\n` and `cwd` containing the disposable studio path. The leading HTML comment was absent. Global context injection was disabled with pi's launch flags; the probe did not deliberately configure extra global context to test filtering independently.

The real workflow produced these observations:

| Action | Observed result |
|---|---|
| Read `BRIEF.md` | Studio text returned |
| Read absolute outside path or inside symlink to outside file | `is outside the studio` errors |
| Read nonexistent studio file | `no such file in the studio` error |
| First paint: create canvas, assign marker 42 and print `first` | `first\nok` |
| Second paint in same model response: print marker and assign 43 | `42\nok`, proving ordered shared-state execution |
| Two `look` calls with size 80 | Two different UUID PNG paths, each `80x80`, each with an image block; no elapsed-time presentation |
| Append `first note`, then replace it with `revised note` | Journal `- day 1, 09:00: revised note\n`; revision history preserved original bytes and real-time timestamp |
| Status and log | Setup-only tool status; log contained exactly the two successful paint chunks |
| Model emits final `done`; inspect easel from terminal | Server still answered `2 chunks · 2400px` and setup |

All six tools were exercised, including the replacement variant of note. Inspection, reads, looks and note operations did not increase the two-chunk count. This sequence did not compare canvas pixels before and after notes. The server was explicitly closed after recording the result. [Sanitized tool replies](runtime-harness-tools.txt).

The initially skipped test was then run with `EASEL_BIN` set to the painter binary and a short temporary path:

```text
node --test --test-name-pattern='paint, look, note and status at a real easel' harness/painter/test/easel-client.test.ts
✔ paint, look, note and status at a real easel (1050.146417ms)
tests 1; pass 1; fail 0; skipped 0
```

It additionally exercised automatic opening from a closed studio, failed-paint rollback reporting and a `480x480` crop through the real client. Together with the [initial suite output](runtime-harness-tests.txt), all 51 existing harness tests passed across the two runs. They were not all rerun together with `EASEL_BIN` enabled.
