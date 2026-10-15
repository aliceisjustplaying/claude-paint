# Engine 7 evidence tools

These tools keep source logs read-only and never change the normal replay CLI. `replay.py replay --engine 7` changes only the engine header in a scratch input. Engine 5 remains the application default. The standard replay binary must already support engine 7.

Python uses `uv`; the replay, matrix and trace tools have no third-party dependencies. Image pairs use Pillow (`uv run --with pillow scripts/engine7/compare.py ...`). No script changes simulation resolution implicitly. The default is the original 2400-pixel width, Gallery light `seen_lit(115,55,1)`, no frames and the prepared-ground cache disabled.

## Replays

```sh
W="$HOME/.capy/work/engine7/evidence"
CARGO_TARGET_DIR="$W/target" CARGO_PROFILE_RELEASE_DEBUG=1 cargo build --release --locked -p easel
uv run scripts/engine7/replay.py prepare "$W/inputs/HOPR1_finished.lua" "$W/inputs/r40-12.lua" --chunks 12
uv run scripts/engine7/replay.py prepare "$W/inputs/HOPR1_finished.lua" "$W/inputs/r40-full.lua"
uv run scripts/engine7/replay.py prepare "$W/inputs/CEZN1_finished.lua" "$W/inputs/cezanne-12.lua" --chunks 12
uv run scripts/engine7/replay.py prepare "$W/inputs/CEZN1_finished.lua" "$W/inputs/cezanne-full.lua"
uv run scripts/engine7/replay.py replay "$W/inputs/r40-12.lua" --engine 7 --workers 4 --easel "$W/target/release/easel" --output "$W/runs/r40-12-e7-w4" --revision "$(git rev-parse HEAD)"
```

Plain logs exclude the appended `--@ chunk (finishing)` only. `--finishing` includes it explicitly. Prefixes retain the original first N chunks, including their comments and whitespace. Output directories must not already exist, so a rerun cannot silently replace a receipt.

The source receipts from `gallery-fcf9c110` are:

| Input | Bytes | SHA256 |
|---|---:|---|
| r40 first 12 | 8,119 | `5eeaa3e4e6e0118db6a8c0bbe94ac972822e8f6cc1de607e61e410fa22c44b12` |
| r40 full plain, 90 chunks | 53,923 | `23bab861c0e73d157af482959071aa4ba405798653b0aecbac880b72263d39c0` |
| Cézanne first 12 | 12,234 | `6ab9aeb02789e1d13914af8158380cbe3f1907345846a74af6a3c630f7acbead` |
| Cézanne full plain, 176 chunks | 122,481 | `f3664d6d1d94e503485cb6437cd21175da994b5040713f72e84371b64f1063b9` |

Each output holds the selected Lua input, stdout, stderr, native Gallery PNG and a JSON receipt matching `engine7-bench-v1`. Process wall includes startup, painting, Gallery rendering and PNG output. CPU/wall and peak RSS come from `wait4` for that child only; RSS is normalized to bytes on Linux and macOS. Chunk wall times come from the existing `Session::run` stderr timings, rounded to 0.01 seconds. They are not derived by subtracting process times. The binary SHA and explicit build revision identify the executable. Unmeasured ready width remains null.

`--checkpoint` or `--digest` adds diagnostic output cost; do not compare those runs to plain runs as if their boundaries matched. Numerical differences are not visual acceptance.

## Hyper matrix

The Captain runs the matrix on the approved hyper machine. No builds, other benchmarks or encoders should overlap its timed interval.

```sh
uv run scripts/engine7/matrix.py run --inputs "$W/inputs" --easel "$W/target/release/easel" --output "$W/hyper-1" --revision "$(git rev-parse HEAD)"
uv run scripts/engine7/matrix.py report "$W/hyper-1"
```

The default is one sequential trial for engine 5 then 7 at 1, 4, 8 and 16 workers on r40 first 12, Cézanne first 12 and full r40. `--cases r40-12 cezanne-12 r40-full cezanne-full` also includes full Cézanne. Single trials do not establish small percentage gains.

## Scratch trace

`trace.py instrument SOURCE SCRATCH [--footprint FILE]` creates a disposable source copy. Only that copy gets tracing edits in central files. The real checkout is never rewritten. Build the scratch easel with a lane-specific target directory, run ordinary engine-5 replays and feed its stderr into `trace.py analyze STDERR OUTPUT.json`.

The tracer records whole synchronous verbs, persistent brush identities, palette trips, observed hand-clock changes, explicit waits, queries, prints and chunk boundaries. With `--footprint`, it copies the footprint lane's module into scratch and records exact/conservative envelopes at `drag_on`. Pointed tools explicitly keep the conservative fallback. The geometry trace has real overhead and must not supply performance speedups.

The graph preserves existing global aging joins. It includes exact spatial overlap where available and omits unknown spatial edges. Its maximum antichain is computed from reachability and a bipartite maximum matching; that is an optimistic width bound, not measured CPU occupancy. Weighted span uses instrumented durations. Missing knife/rag tool identity or unsupported spatial closures can only narrow the true opportunity. This trace does not implement a recorder or authorize deferred aging.

## Visual fixtures and checks

`fixtures/` contains a clean badger pull across wet red/blue, a split-loaded two-color drag and a lean scumble over a dry dark passage. All use deterministic seeds and the same short physical canvas. Replay each at engine 5 and 7, then use `compare.py` for a small overview or `--crop X Y SIZE` for a native-pixel pair. Gallery pairs label engine 5 on the left and engine 7 on the right.

```sh
uv run scripts/engine7/test_tools.py
uv run --with pillow scripts/engine7/compare.py e5/gallery.png e7/gallery.png overview.png
uv run --with pillow scripts/engine7/compare.py e5/gallery.png e7/gallery.png detail.png --crop 600 300 420
```
