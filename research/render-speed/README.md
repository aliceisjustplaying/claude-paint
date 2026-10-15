# Oil render-speed research

Research only, based on main `73f5f9cb4c01a7e82f6b4d41d9aaa5bfe0347209`. This branch is not a production change or a request to merge. The final report is `reports/STIL-45.md` in the Stillwet captain Drive.

## Reproduction

Build with an isolated target:

```sh
CARGO_TARGET_DIR="$HOME/.capy/work/render-speed/target" CARGO_PROFILE_RELEASE_DEBUG=1 cargo build --release --locked -p easel
```

The copied inputs are r40, r39.1 and r34 engine-5 logs from `stillwet-runs/gallery-fcf9c110`. Plain versions stop before `--@ chunk (finishing)`. The r40 scaling prefix contains the original first 12 chunks. No log instructions, canvas geometry or brush work inside those chunks are rewritten. Inputs and rendered images stay outside this repository, under `~/.capy/work/render-speed/`.

`measure.py --result PATH.json --timeout SECONDS -- COMMAND...` records wall time, child CPU time and peak RSS along with stdout/stderr. Invoke it with `uv run`. Linux peak RSS is KiB; macOS peak RSS is bytes.

The research binary adds `easel render-speed CHECKPOINT OUTPUT_DIR [1,2,4]`. It restores once and measures color, Gallery and Relief, RGB conversion, thumbnails, PNG encoding and file writes. Each scene/variant retains a PNG. It checks the exact parallel conversion against the serial path and reports pixel differences for the striped thumbnail and 16-bit sRGB lookup prototypes.

Environment switches in this branch:

- `RENDER_PROFILE=1`: emit `PROFILE_LIGHT`, `PROFILE_FRAME` and live rollback timing rows to stderr.
- `RENDER_SCHED_PROFILE=1`: emit per-batch tile count, dependency edge count, initial ready count, peak active tiles, summed tile work time, preparation time and elapsed time. Tile work seconds overlap and are not wall time.
- `RENDER_LIVE=1`: run accepted log chunks with the real live-session rollback snapshot path enabled. This is a controlled live-compute experiment, not a full studio/tool benchmark.
- `RENDER_CHECKPOINT_CHUNK=N` with `RENDER_CHECKPOINT_PATH=...`: retain one intermediate finishable checkpoint.
- `RENDER_DRY_FAST=1`: experimental dry-surface shortcut. It bypasses the two bridging blurs only if every computed wet/solvent height is nonpositive. Disabled by default.

The timers and extra commands are research instrumentation. No crack/traction implementation, engine version, painter text or production clip script is changed.

## Interpretation

Keep CPU samples, wall-phase timers and whole-process times separate. A 30 fps, 600-frame video can use fewer than 600 distinct engine captures; the hand-time recorder also captures chunk ends. The output width does not change simulation resolution. The raw checkpoint is finishable/displayable, not a resumable Lua heap and brush state.

For timing comparisons, run one compute workload per machine. No benchmark is allowed to consume the Mac's final 20 GiB of free disk. Preserve images but reclaim owned build caches and raw checkpoint copies when needed. The 16/32-vCPU recommendations in the report are extrapolations, not measurements on nonexistent cores.
