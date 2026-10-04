# Engine 3: independent review and implementation brief

**Reviewed packet:** `engine3-rag-review-2026-10-04.zip`, October 4, 2026.

Start with **REVIEW.md** for the diagnosis and proposed architecture, then **AGENT_BRIEF.md** for the ordered implementation work. **EVIDENCE.md** contains line-numbered excerpts and SHA-256 hashes of the uploaded source files. **SOURCES.md** records the external primary references and what each does—and does not—support.

The original Rust source was not modified. The source snapshots were inspected and supplied images examined. A Rust toolchain and working dependency-download access were unavailable here, so **the Rust suites and original renders were not rerun**. Recorded engine measurements are explicitly distinguished from independent calculations. This is not an approved or completed rag implementation.

## Runnable material

Python 3.10 or later; standard library only:

```sh
cd engine3-independent-review
python -m unittest -v test_probes
python probes.py --output my_probe_results.json
# Optional: additionally verify pigment source hashes against your extracted packet:
python probes.py --root /path/to/engine3-rag-review --output my_probe_results.json
```

The delivered `test_results.txt` records **18 passing tests**, including 500 randomized budget/conservation cases in one test. These validate the independent probes and the small budget reference, not the Rust renderer or real oil-paint behavior.

`bounded_transfer.py` is a reference implementation of donor/receiver capacity limiting and component-conserving transfer. It intentionally contains **no cloth geometry, paint calibration, optical rendering, or historical material claim**. It is suitable as a small algorithm to port and test—not as a replacement engine.

`images/` contains three unaltered images from the supplied packet, copied for convenient reading. They are prior, unaccepted engine results, not new renders or real painting references.
