# Verification

Checklists are grouped by feature. P1 covers established facts and suspected defects, P2 ordinary behavior and P3 exact numeric or visual details. Run P1 before P2 before P3. Result is pass, fail or blocked, with evidence and limits. A fail can mean a document is wrong; it is not automatically a product defect. Every suspected product defect belongs in [triage](../bug-triage.md).

The source commit is `4e525e50897807e9b5f734071dfeb330f1a393d3`; source HEAD and binary build provenance must be checked separately. The documented default build is `cargo build --release -p easel`; the painter variant uses `--no-default-features`. These are maintainer runbook details, not a request to the reader. An isolated EASEL_ROOT prevents probes from altering existing paintings.

The browser surface is `studio/studio.py`; it accepts a session log directory, host and port. Python execution uses uv and a virtual environment. Preview access remains private to the tailnet. Existing public services are outside this pass.

A document remains drafted until a person has run all its P1 and P2 rows. Automated and agent-driven results are recorded without claiming that human verification happened.

## Checklists and evidence

The [broad runtime pass](runtime-pass.md) records the follow-up checks, test receipts, confirmed defects and remaining coverage. Its totals are 254 pass, 9 fail and 750 blocked or partial checklist rows.

| Checklist | Documents |
|---|---|
| [Canvas and views](canvas-views.md) | Canvas, looking |
| [Passages and time](passages-time.md) | Area work, painting clock |
| [Mark making](mark-making.md) | Brushes, shapes, drawing, rags |
| [Sessions](session.md) | Commands, session lifecycle, journal, harness |
| [Viewer and space](viewer-space.md) | Browser viewer, geometric references |
| [Delivery](delivery.md) | Saving, replay, finishing and export |
| [Triage](triage.md) | One P1 reproduction per suspected defect |

The [CLI observations](evidence/cli.md), [additional probes](evidence/additional.md) and [browser observations](evidence/browser.md) contain actual results. Checklist scenarios can be broader than those probes and stay blocked when the full expected result was not checked. `blocked — unrun` means this pass did not execute the scenario; it is not a discovered product failure. A known bad behavior can pass a descriptive row while failing the intended behavior in triage.

The release binary was built with `cargo build --release -p easel` in the source checkout at the recorded commit. Source HEAD stayed at that commit. This documentation task made no source edits. Concurrent changes later appeared in session and rag source files, examples and review folders; they were left untouched and are outside the pinned description. The follow-up pass ran the documented Rust, harness and studio test groups and actual local workflows. External-provider runs, the complete historical export matrix and the human P1/P2 pass remain unclaimed.

The [private fixture preview](http://<tailscale-host>:18765/?p=paint-studio-decafe) uses the current studio source with the committed synthetic records in `fixtures/sessions`. Its images come from the CLI probe; its abbreviated display code is not a runnable painting. The actual program is [probe.lua](evidence/probe.lua). The server uses `.preview-venv` and the persistent exec session recorded in browser evidence. Access from the serving machine through the Tailscale IP was checked; access from the other Mac remains untested.

The structural checker `check-docs.rb` checks feature skeletons, interrupt rows, cross-cutting order, source footers, coverage links, glossary duplicates and relative links/anchors. It does not prove product behavior. Output is recorded in [structure evidence](evidence/structure.txt).
