# Engine 3 review response

This package responds to the supplied review of the earlier audit. It changes the implementation priorities, withdraws default adoption of thinner variant (b), recommends a defined sienna requirement, and adds two source/algebra findings.

Start with **REVIEW_RESPONSE.md**. **AGENT_BRIEF_V2.md** replaces the previous brief's order and unconditional (b) integration instruction. **SOURCE_RECEIPTS.md** contains exact original-packet excerpts and hashes. **SUPPLIED_COUNTER_REVIEW.txt** is the user's supplied report, preserved unchanged.

## Scope

No Rust engine fix, new rendering, production merge, or physical validation is included. The counter-review's new Rust measurements are attributed to it; its raw scratch probe logs were not supplied. No usable cargo/rustc was found on PATH in the local environment. `environment.json` records that boundary and input hashes.

## Reproduce the isolated calculations

From this directory, using Python 3.10 or newer:

```sh
python -m unittest -v test_model_probes
python model_probes.py > probe_results.json
```

The 12 tests check equations, units, a constructed limiter example and the existing optical formulas. They do not execute the Rust engine. Floating-point calculations use Python's usual binary64, not Rust f32.

The new rag/ceiling threshold assumes a fresh ceiling-limited film of unchanged composition, with no added lateral or subsequent-pass material. The claim about a measured ≤1 µm paint pixel follows directly from the minimum rag floor. The mixed-mobility example isolates the shared flow schedule, not an entire evolving transport simulation.

Verify source provenance against the original archive with:

```sh
python verify_source_manifest.py /path/to/engine3-rag-review-2026-10-04.zip
```

A changed source hash means these exact-source observations need review before reuse.
