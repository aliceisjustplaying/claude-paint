# Checks run against the external Engine 3 review (October 4, 2026)

Inputs: `notes/rag/reviewer-package/engine3-rag-review-2026-10-04.zip` (the packet) and
`~/engine3-independent-review.zip` (the review). Nothing in the repository's source was changed.

- `verify_evidence.py`: every hash in the review's `source_manifest.json` and every quoted line in its
  `EVIDENCE.md`, checked against the extracted packet (17 hashes, 1,597 lines, 0 mismatches).
- `rag-probes-on-research.patch`: instrumentation and two ignored tests (`review_probe`, `review_probe2`)
  applied to the packet's `research/` snapshot. Output: `rag_probe.log`, `rag_probe2.log`.
- `thinner-probes-on-wip-thinner.patch`: counters in `thinner.rs` and `tests/review_thinner.rs`, applied to the
  packet's `wip/thinner/` snapshot. Output: `thinner_probe.log`, `thinner_probe2.log`.
- `main-rag-probe-on-shipping.patch`: one test file, public API only, on the packet's `shipping/` snapshot
  (its `rag.rs` is byte-identical to main's at 4e525e5). Output: `ship_probe.log`.
- `look_probe.log`: the packet's own `look_rim` and `look_sienna` measurements rerun on its unmodified
  `wip/look/` snapshot.

All runs: `cargo test --release`, rustc 1.97.1, macOS, through `lockrun`.
To rerun: extract the packet, apply a patch with `patch -p1` inside the named snapshot, then e.g.
`cargo test --release -p paint --lib review_probe -- --ignored --nocapture`.

## Round 2: checks on the reviewer's response (`~/engine3-review-response.zip`)

The two patches above now also carry the round-2 probes.

- `rag_probe3.log` (`review_probe3` on `research/`): named ablations on the 69 um fixture (no whole-face thirst,
  no laying back, no stain floor), a tracer for how much of the film left came off the cloth, and thin uniform
  films (0.5 to 10 um) wiped with the floor on and off.
- `thinner_probe3.log` (`review_rag_floor`, `review_mixed_cap` on `wip/thinner/`): brush-laid thinned strokes wiped
  with a clean damp rag against the 1 um floor's bound; a thinner-0.5 patch flowing with and without an unrelated
  thinner-0.95 stroke in the same dirty region.

The response package's 16 source hashes verify against the packet and its 12 Python tests pass.
