# Independent-mixer baseline provenance

This package records the current color candidate, not visual acceptance.
The candidate's four color guards still fail; its exact source and mixer
table hashes identify the capture without treating it as finished.

## Producing build

Recorded on 2026-10-22 from source
`0c1872d6af305f982c4bb424bef015133c75bfcc` on Linux x86_64.
The test-profile easel was built with `profile.test.debug=0`.
Its SHA-256 is
`42c9caa49b5479abe865441ffe6ec9760cabdb910ce82ce8891fbafada3d4867`.

The producing mixer identity is `5477582544128984857`
(`0x4c04484d6ab04719`). It fingerprints the two tables and the decoder
schema, independently of the engine number. Each dump preserves it in
the scalar `u64` field `mixer.identity`; reconstruction copies this value
instead of labeling an old dump with the current checkout's tables.

The Nix development shell supplied the toolchain. The isolated sakaki job
used `MemoryMax=6G`, `CPUQuota=400%`, `nice -n 10`,
`CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=4` and
`RAYON_NUM_THREADS=4`. CPU quota is not CPU affinity. No live painting's
studio or OBS was changed.

The generating script and `crates/paint/src/mixer/primaries.json` record
the independently authored basis, concentration power and table hashes.

## Frozen physics and current mixer fields

All 27 stored dumps combine main `d94eafd60` physics with this capture's
`color`, `wet.lat`, bristle `lat` arrays and piles (both
`studio.piles` and the redundant `studio.debug` pile colors).
Every other pre-existing value is retained exactly from main, including
rag loads, wet volumes, hiding, floors, drying clocks and bristle volumes.
The field audit checks both halves exactly, not only within tolerance.
New solvent fields are zero. The new identity metadata equals the
producing capture exactly; it is not a physics-field exception or an
ignored comparison field.

These are field-transplanted expectations, not untouched replay dumps.
The `canvas=` hashes in state headers and `ref/digest/` are reconstructed
from the stored expectations with `tools/checkpoint_from_dump.py`.
The tool/studio digests, PNGs, printed output, times and `proof/` files
come from the fresh replay. Proof canvas digests therefore describe
actual replay state, not the transplanted rag expectation.

Main's known rag physics mismatch is still diagnosed, not accepted as a
baseline pass. Its existing scalar signature and unrelated-state checks
remain. The current-mixer rag PNG must pass its image comparison instead
of reproducing an obsolete image-failure signature.

## Record commands

`EASEL` names the absolute path of that build's `target/debug/easel`;
`CAPTURE` is scratch outside this package.

```sh
notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/dump" --dump
notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/nodump"
RAYON_NUM_THREADS=1 notes/thinner/baseline/tools/run_scenes.sh "$EASEL" "$CAPTURE/t1" --dump
```

All runs use the unchanged six scene logs at width 128. Each scene's PNG
is byte-identical across these three runs; the four-thread and one-thread
state dumps pass the existing state comparer. The dump run supplies
`ref/png/`, `ref/out/` and `ref/times.tsv`. The three runs supply
`proof/digest_{dump,nodump,t1}/` and the corresponding times.

After capture, only the mixer fields listed above are transplanted into
main's state expectations. The package reconstructs all 27 canvas
checkpoint digests. `ref/png/platform.txt` is the measured output of
`scripts/compare_images.py --print-platform`.

## Other mixer-dependent references

The exact existing recorder tests ran with `UPDATE_GOLDEN=1`:

```text
tests::fixture_matches_golden
session::tests::engine_2_logs_replay_as_before
```

They record the paint scene and its platform and the three engine-2
drying-sheet views. The numerical plough reference was not re-recorded;
its unchanged test passes.

The same build also supplied:

```sh
scripts/tests/old_logs.sh "$EASEL" --record crates/easel/tests/old_logs/golden
"$EASEL" run crates/easel/tests/r19_default_box.lua --out crates/easel/tests/r19_default_box.png
uv run --script scripts/compare_images.py --print-platform > crates/easel/tests/r19_default_box.platform
```

The old-log set has 12 cases, including the original ck7 Lua log at
128 px. Its PNG and text digest are newly recorded. The separate r19
image uses its unchanged default width 2400; the old-log r19 case stays
at width 320. No scene input, recipe, numeric assertion or image/state
tolerance was changed by recording.

## Inventory

`SHA256SUMS` contains every intended package file except itself, with no
interpreter cache or temporary output. `tools/verify_package.sh` checks
the inventory, comparer self-tests and all reconstructed checkpoints.
`old_files/` remains unchanged historical compatibility input, not an
active expectation or a way to forgive a new mismatch.
