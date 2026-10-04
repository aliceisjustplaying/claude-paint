# Verification

Checklists are grouped by feature. P1 covers established facts and suspected defects, P2 ordinary behavior and P3 exact numeric or visual details. Run P1 before P2 before P3. Result is pass, fail or blocked, with evidence and limits. A fail can mean a document is wrong; it is not automatically a product defect. Every suspected product defect belongs in [triage](../bug-triage.md).

The source commit is `4e525e50897807e9b5f734071dfeb330f1a393d3`; source HEAD and binary build provenance must be checked separately. The documented default build is `cargo build --release -p easel`; the painter variant uses `--no-default-features`. These are maintainer runbook details, not a request to the reader. An isolated EASEL_ROOT prevents probes from altering existing paintings.

The browser surface is `studio/studio.py`; it accepts a session log directory, host and port. Python execution uses uv and a virtual environment. Preview access remains private to the tailnet. Existing public services are outside this pass.

A document remains drafted until a person has run all its P1 and P2 rows. Automated and agent-driven results are recorded without claiming that human verification happened.
