# Current-mixer references for old logs

The 11 unchanged cases in `../cases.tsv` were explicitly re-recorded for
STIL-212 with the independent pigment mixer. Their older engine headers
retain the corresponding handling behavior, not the retired pigment
implementation. These are current-mixer expectations, not af49348 or
historical-library output.

Producing source: `d3c9d5940e0d9a7caba0f057461611e9abd53c57`.
The workspace test-profile easel was built with `profile.test.debug=0`,
Rust 1.99.0, CPU affinity 0–3 and four build/test/Rayon threads. Its SHA-256
is `b811186de2267f5d0fd5b3f20799da9adb2d4095d05f1f58c29fe6ab4ddd7395`.
Full build provenance is in `notes/thinner/baseline/MANIFEST.md`.

Recorded on 2026-10-22 with:

```sh
scripts/tests/old_logs.sh target/debug/easel --record crates/easel/tests/old_logs/golden
```

The recorder received the absolute executable path. It uses the existing
width for each case in `cases.tsv` and writes a PNG, a `.txt` file with
its SHA-256 and per-chunk state digests (`secs=` removed), plus the
measured producing platform in `platform.txt` (`linux-x86_64`).

All platforms retain the maximum decoded RGBA channel error of 2/255
from `notes/workflow.md`. State digests are diagnostic. The same-binary
`legacy_tiny` repeat remains checked. No tolerance or input changed as
part of this recording.
