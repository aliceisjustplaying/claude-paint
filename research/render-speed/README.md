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
- `RENDER_INTEGER_POWER=1`: display-only multiplication chain for the fixed 40th-power sheen.
- `RENDER_STEP_SCALE=2` or `4`: increase the swept-capsule segment length; changes discrete brush interaction and requires visual comparison.
- `RENDER_TILE_SCALE=0.5` or `0.25`: smaller handling tiles, changing brush identities and load grouping.
- `RENDER_SLICE_FACTOR=4` or `64`: coarser covering-pass aging batches, preserving total hand time but changing when paint ages.
- `RENDER_REORDER=1`: greedily group nonoverlapping tile footprints before the existing ordered scheduler. It never permits concurrent overlapping writes, but changes paint order and appearance.

The timers and extra commands are research instrumentation. No crack/traction implementation, engine version, painter text or production clip script is changed.

## Interpretation

Keep CPU samples, wall-phase timers and whole-process times separate. A 30 fps, 600-frame video can use fewer than 600 distinct engine captures; the hand-time recorder also captures chunk ends. The output width does not change simulation resolution. The raw checkpoint is finishable/displayable, not a resumable Lua heap and brush state.

For timing comparisons, run one compute workload per machine. No benchmark is allowed to consume the Mac's final 20 GiB of free disk. Preserve images but reclaim owned build caches and raw checkpoint copies when needed. The 16/32-vCPU recommendations in the report are extrapolations, not measurements on nonexistent cores.

The later hyper runner measured 16 vCPUs directly; only 32-worker results remain extrapolated. It confirmed the replay plateau, the swept-step prefix gain, output-row scaling and independent-job throughput. Its full receipt is `reports/STIL-45-bigbox.md` in the captain Drive.

`clip.py` reuses the existing dynamic pacing calculations to prepare exactly 600 frames at 30 fps, with 15/495/90 opening/process/hold frames. `compare.py` retains full and native-detail comparison sheets plus numeric differences. These are research artifacts, not a change to the production clip wrapper or an automated visual-acceptance gate.

The final report distinguishes complete-painting results from prefix tests, stage medians from process wall time and measured throughput from estimates. The branch is a collection of research probes; none of its approximation flags is a production recommendation on its own.
