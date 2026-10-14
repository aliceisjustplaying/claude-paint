# Watercolor timeout research

Pinned source: `8ee1cdd67f510a55eb6dc279e7113c593e1b282a`. These scripts reproduce saved Lua, count planned stamps and collect timing receipts. They do not change the watercolor engine's behavior. `instrumentation.patch` adds counters and timers to an isolated checkout only.

Research directory on alice-m3p: `/Users/alice/.capy/work/wc-timeouts/`. The original studios and sessions are read-only inputs. Every generated studio, checkpoint, log and build target stays in that directory. Scripts refuse to reuse output directories, and experiment scripts check the 15 GiB free-space floor.

The original binary is retained as `watercolor-studio`; the timer-only binary is retained as `profile-watercolor-studio`. The build target was removed after the investigation. The source checkout's instrumentation is local-only, and no engine edits are included on this branch.

Run scripts with the research virtual environment:

```sh
cd /Users/alice/.capy/work/wc-timeouts
uv venv .venv
uv run --no-project --python .venv/bin/python extract.py
uv run --no-project --python .venv/bin/python census.py
```

`extract.py` extracts the seven exact timeout chunks and inventories retained checkpoints. It parses large JSON states and takes several minutes. `census.py` exports paint calls and runs `trace.lua` with the installed Lua 5.5.1. The tracer replaces paint methods with counters, so its counts describe planned work rather than executed GPU work. Its ordinary Round stamp-spacing calculation matches the pinned Rust source. The engine profiler independently verified the 21,021-stamp count for RW1 L628.

```sh
uv run --no-project --python .venv/bin/python reproduce.py rw3-L452 --label another-rw3-L452 --limit 900 --sample
uv run --no-project --python .venv/bin/python replay-history.py
uv run --no-project --python .venv/bin/python launch-bench.py --profile --checkpoint /Users/alice/.capy/work/wc-timeouts/history/before-L628.json --limit 1200
uv run --no-project --python .venv/bin/python launch-bench.py --profile --app --checkpoint /Users/alice/.capy/work/wc-timeouts/history/before-L628.json --limit 1200
uv run --no-project --python .venv/bin/python bench.py --suite all
uv run --no-project --python .venv/bin/python analyze.py
```

The launch tests create only the owned `art.stillwet.wc-timeouts-reproduction` LaunchAgent, with its plist and output under the research directory. They remove that registration on exit. `--app` changes only its `ProcessType` to `Interactive`; it is an experimental control, not a production scheduling recommendation. The commands document the recorded runs: their named outputs already exist, so repeating them is rejected. `reproduce.py --label` supports a fresh replay name; another history or benchmark run requires fresh output paths in an isolated script copy. The scripts deliberately do not delete previous results.

To regenerate the profiler from a clean, isolated checkout of the pinned source:

```sh
git apply /path/to/research/wc-timeouts/instrumentation.patch
CARGO_TARGET_DIR=/Users/alice/.capy/work/wc-timeouts/target CARGO_BUILD_JOBS=4 cargo build --release --locked --manifest-path experiments/watercolor/Cargo.toml --bin watercolor-studio
```

`analyze.py` exports `measurements.json` and an exact-command timing table. Its least-squares model is an inference from successful historical calls, not a measured completion time for canceled calls. The final report and 58 timing receipts are in the Stillwet Captain's Drive, under `reports/STIL-40.md` and `handoffs/wc-timeouts/`.
