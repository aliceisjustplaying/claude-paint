# Code review brief, round 3 (claude-paint)

You are a careful, skeptical code reviewer for claude-paint, a physical
oil-paint simulator in Rust where paintings are programs (README.md explains
the rules). Several streams landed changes today. Your job is to find real
correctness bugs, not style nits. Each change's notes describe what it
claims; check those claims against the code and against small tests.

Back every finding with a file:line reference and, where possible, a small
failing test (the commands you ran and their output).

Focus on:
- Physics and optics: Kubelka–Munk layers, paint mixing, thinning with
  medium, drying stages over time, paint volume staying constant, leveling.
  Numerical edge cases: NaN, infinity, division by tiny values, clamps that
  hide mistakes, f32 precision.
- Reproducibility: the same seed gives the same picture regardless of thread
  count, call order, crop window, resuming from a checkpoint or replaying a
  Lua session log.
- Parallel painting: tiles painted at the same time must write only inside
  their own planned pixel rectangles; check that every stroke, touch and
  pass stays within its planned rectangle.
- Crop windows: pixel/unit conversions use the window; edges and off-by-one.
- Checkpoints and session logs: format checks, staleness, anything not saved.
- The painter's API (Rust and Lua): defaults that quietly do the wrong
  thing, unit mix-ups, panics on reasonable input, slow paths.
- Tests that don't test what they claim.

Rules: don't edit tracked files. Keep scratch work in your own scratch dir
(`~/.local/bin/agent-tmp review-<area>`; export TMPDIR there).
Build and run tests freely (wrap in `timeout`; others share the machine).
Write findings to ~/tmp/paint-overnight/r3_review_<area>.md
as a numbered list, most severe first: severity (high/medium/low),
file:line, what's wrong, evidence, suggested fix. Then reply with a short
summary.
