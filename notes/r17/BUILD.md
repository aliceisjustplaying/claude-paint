# Round 17: finishing moves out of the painter's build

Branch `r17-base`, cut from `r16-base`; worktree `~/src/a/claude-paint-r17-base`.

## What changed

- **Moved:** `varnish{}`, `cracks{}` and `relief()` (with their helpers
  `brushed`, `wet_share`, `needs_dry`, the `MASTIC` color and the
  must-be-dry regression test) go from `crates/easel/src/api.rs` into a new
  `crates/easel/src/finish.rs`. The behavior is unchanged.
- **Feature:** `finish` in `crates/easel/Cargo.toml`. Default features are
  now `["replay", "finish"]`. `api::install` calls `finish::install` only
  with `#[cfg(feature = "finish")]`.
- **Painter build** (`--no-default-features`, which is what
  `scripts/export_r16_studio` builds) has no finishing verbs: calling one
  fails as an undefined global and the chunk changes nothing.
  `crates/easel/tests/smoke.rs` checks this under
  `cfg(not(feature = "finish"))`. `strings` finds the "not all dry yet"
  message in the full binary but not in the painter binary.
- **Guide:** `notes/easel_guide.md` loses its "Finishing (optional)"
  section. The other exported notes (`notes/research/oil_paint_physics.md`
  and `notes/research/friedrich_materials.md`) are sourced research, not
  instructions, and are unchanged.
- **Finishing after the session:** `scripts/finish_painting <log> <out.png>
  [--coats C] [--no-varnish] [--no-cracks] [--relief]` builds the default
  (full) easel, appends a finishing chunk to a copy of the painter's log
  (`<out>.lua`) that waits a month at a time until the paint is touch-dry
  (at most ten years), then varnishes (coats 0.4) and cracks, and replays
  it with `easel run` at 2400 px.

## Verification

- `cargo test --release -p easel`: all pass: 25 unit tests plus delivery 2,
  determinism 2, session_integrity 5 and smoke 1 (2 min 17 s).
- `cargo test --release -p easel --no-default-features --features replay
  --target-dir target/replayonly`: all pass: 24 unit tests (the finishing
  test compiles out) plus the same integration tests, with smoke checking
  that the verbs are absent (2 min 19 s).
- `cargo build --release -p easel --no-default-features`: builds.
- **Round 16 B1** (`paint-studio-cd267e`, 137 chunks):
  - `scripts/finish_painting` ran in 429 s wall clock and wrote a
    2400 × 1690 PNG. The paint was already dry, so there was no wait.
  - A plain `easel run` of the same log with this build is pixel-identical
    to `notes/round16/look/B1_2400.png` (0 differing pixels).
  - The finished picture is warmer and slightly darker: mean RGB goes from
    160/158/142 to 153/146/118. A craquelure network covers the whole
    picture and shows most in the sky.
- **Small wet log:** the script waited one month, the paint was dry by
  day 31 and varnish, cracks and relief all ran.
- **Script fixes found while verifying:** the usage text left out the
  synopsis line, and `--coats` with a missing or non-numeric value was
  passed into the Lua call (a word became `nil`, which silently meant the
  default). Both now print the usage or an error and exit 2.
