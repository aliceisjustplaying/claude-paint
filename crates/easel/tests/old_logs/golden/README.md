# Current-mixer references for old logs

The 12 cases in `../cases.tsv` were explicitly recorded for STIL-219's
current independent-mixer candidate, including the original ck7 Lua log
at 128 px. The other inputs and all widths are unchanged. Older engine headers
retain the corresponding handling behavior, not the retired pigment
implementation. These are current-mixer expectations, not af49348 or
historical-library output.

Producing source: `116943558560499a8950e0cb99eb91b477d3ae09`.
The workspace test-profile easel was built with `profile.test.debug=0`,
four build/test/Rayon workers, CPU quota 400%, memory cap 6 GiB and nice 10.
Its SHA-256 is `7f5c05849e74d3236e5caa22ab052430b27e5481d32f11a881406bbea21bfa6d`.
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
