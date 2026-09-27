# Round 19: prompting fixes from the two audits of 2026-09-27

Branch `r19-base`, cut from `r17-base` (40db9c4); worktree
`~/src/a/claude-paint-r19-base`. Round 19's runner
(`~/tmp/gallery-fcf9c110/r19/r19_chains.py`) exports its studios from this
branch and runs its painters in this worktree's harness. Rounds 17 and 18
keep running from `r17-base`. The full list of changes, with before and
after quotes, is `~/tmp/gallery-fcf9c110/r19/CHANGES.md`; every text a
painter can read, old against new, is `r19/painter_text.diff`.

## What changed here

- **`check` is the runner's.** The painter build has no `check` (usage,
  dispatch, server). It answers `easel: no command "check"`. The replay
  build keeps it. `scripts/check_painting <studio> <workdir>` runs it after
  a painter's last sitting: it reopens a copy of the log (one replay), runs
  `check` (a second replay, compared bit for bit), then compares the
  replayed PNG with the painter's last save. The export's probe no longer
  runs `check`, and it fails if the exported easel has one.
- **Errors a painter can see:** no color recipe in the pile and canvas
  help (placeholders instead). The integrity refusals state the fact
  ("differs from the log the session wrote", "can't be read"), not an
  accusation ("was edited outside the session").
- **Guide:** no `check` row, no "## The log" section and no "bit for bit"
  line. Examples are subject-neutral: `canvas{...}` at the start,
  placeholder size and aspect, diagonal stroke/curve/ruler, terrain on a
  small patch. World is a reference with placeholder eye height. "The tube
  box:" loses "(all made by the 1820s)".
- **Research:** `oil_paint_physics.md` drops its developer framing and its
  craquelure and aging-optics (varnish) sections. `friedrich_materials.md`
  drops its varnish section; that commit is separate because it goes
  beyond the approved list.
- **Studio layout:** the license notices ship as `bin/THIRD_PARTY_NOTICES.md`,
  next to the binary they concern, not at the studio root where painters
  browse (a round 17 painter opened the root copy while exploring). The
  export's "nothing but those" check allows only that path.
- **Harness:** the compaction summary opens "Earlier parts of this session
  were condensed." `studio-settings.json`, which named two models, is gone.

## Verification (2026-09-27)

- `cargo test --release -p easel` passed: 27 unit tests plus delivery 2,
  determinism 2, session_integrity 5 and smoke 1 (219 s).
- `cargo test --release -p easel --no-default-features --features replay
  --target-dir target/replayonly` passed: 26 unit tests plus the same
  integration tests (241 s).
- `cargo test --release -p easel --no-default-features` (the painter
  build) passed: 25 unit tests and `tests/painter.rs`, which now asserts
  the usage has no `easel check` and `check` fails with `no command
  "check"` (45 s).
  - Use a short `--target-dir` (e.g. under `~/tmp`): with
    `target/painter` in this worktree, the test's socket path is longer
    than the Unix socket limit (`path must be shorter than SUN_LEN`).
- `node --test harness/painter/test/*.test.ts` passed: 12 tests.
- `scripts/export_r16_studio friedrich|blank` from r19-base (4bee9f6, with
  the probe fix in df127dd) exported both studios. Their text files have
  no `check`, "in one session", `timeout`, "1820s" (outside Friedrich's
  own materials note) or model names. `strings bin/easel` finds none of
  them either.
- A probe session in the exported blank studio ran the guide's new
  examples: diagonal stroke, `h:rule`, `below`, form with the terrain
  patch, world with a block.
- `scripts/check_painting` on that studio:
  - "check: ok: 6 chunks replay to the same canvas; the replay's PNG
    equals the painter's last save".
  - After one more chunk without a save, the PNG was reported as "differs
    ... (saved before the log's last change)".
  - With a failing chunk appended to a copy of the log: "check: FAILED:
    the log doesn't reopen", exit 1.
- After the notices move: `R16_BRANCH=r19-base scripts/export_r16_studio
  friedrich|blank` exported both studios (8fc975e's easel). Each has exactly
  `bin/easel`, `bin/THIRD_PARTY_NOTICES.md` (identical to the repo's) and
  its notes; a studio with a notice at the root fails the check.
