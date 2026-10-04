# Round 24: code review guide

Current direction (2026-10-04): round 24 has not painted. The better thinner remains ENGINE 3 and is the next implementation priority; painting is deferred. Follow `~/src/a/claude-paint-thinner-spec.md` draft 7 and the updated handover. Old paintings retain their pinned runtime; no engine-1/2 compatibility replay gate applies to the new candidate. No expensive replays, ever. The required new evidence is tiny before/after fixtures, the exact thinner card target and a small inspected rag/thinner study. The sections below record the pre-thinner integration and historical evidence; they are not commands to rerun those checks.

Integration branch: `round-24` in `~/src/a/claude-paint-r24` = af49348 (main + drying + repaints + rag + r24-fixes c4bb131 + sienna c5963f8 + prompt edits af49348). This pinned commit does not yet contain the better thinner; the upcoming thinner candidate is still engine 3. Full suite on c5963f8: 276 passed, 0 failed.
Primary integration review: `git diff 7b80cb0 af49348`. Branch-tip diffs below are historical context and omit later integration fixes. Review the future thinner candidate against af49348 separately.
Open design questions: `~/src/a/claude-paint-round24-open-questions.md`.

## The one rule everything rests on
Historical round-24 requirement: changes were engine-gated with engine-1/2 replay identity. Current thinner policy: engine 3 can evolve; old paintings use preserved old code. Tiny engine-3 baseline comparisons and the new acceptance gates replace historical replay checks.
Gate points to check:
- `crates/paint/src/lib.rs` `ENGINE = 3` (merge conflict resolved: both doc lines kept).
- drying: `drying::Pace::of(engine)` (drying.rs ~198-208) is the only engine switch for timing; tube rates via `Palette::drying_of`, `Tube::drying_3`.
- repaints: `session.rs` `canonical_tables(engine)` (~:35) is the only switch for table order.

## repaints (`origin/repaints`, 7d2127e) - 8 files, +358/-62. Read first: smallest, highest value
Where to look:
1. `crates/easel/src/prelude.lua` (~:54, :149): under engine 3 every table walks in the fixed order; `pairs` on non-tables never exposes Lua's `next`.
2. `crates/easel/src/heap.lua` (:47, :71, :93-142): snapshot avoids `rawlen`; "stale" only for a changed table with holes.
3. `session.rs`: the gate and the new tests (`under_engine_3_a_failed_chunk_needs_no_rebuild`, engine-2 rebuild still happens, order pinned).
Existing tests it changed (check the reasons hold):
- `plain_tables_keep_lua_order` now pinned to engines 1/2 (its intent: old logs keep Lua order).
- `session_integrity` forces its rebuild via a table with a hole (growing a table no longer rebuilds); key count 30 -> 31.
Evidence: 8 real rebuilds (Sonnet 3a958c, Space Bunny 35c488) all rebuild on engine 2, none on 3; recovery 885 s -> 0.09 s at 320 px.
Known leftover: a changed table left with holes still rebuilds (`#` depends on layout).

## drying (`origin/drying`, 56c5470) - 13 files, +575/-81
Test sheet: `notes/drying/test_sheet.jpg` (lead white + bone black lifted at 1 h/4 h/12 h/24 h/3 d, engine 2 vs 3).
Where to look:
1. `crates/paint/src/drying.rs`: `TOUCH_DRY_MIN_3` 60 h (:58), `OPEN_SHARE` 0.6 (:76, estimate), `STROKE` 1.5 coats (:62), `Pace::of`.
2. `crates/paint/src/palette.rs`: per-tube engine-3 rates, `Palette::with` keeps the box's engine.
3. Tests: `strokes_dry_within_the_sources_ranges` (:959), `fast_pigments_dry_before_slow_ones` (:999), `thick_films_dry_as_much_slower_as_goldens` (:1013).
Existing tests it changed (check):
- three band tests in drying.rs retimed to engine 3, sample times picked by scanning ("earlier times catch parts of the band still open").
- `waiting_it_out_equals_dry`, `splitting_a_wait_changes_nothing`: longer waits.
- two tests in handling.rs and time.rs pinned to engine 2.
- removed asserts include "lead white sets within 3 h" / "touch-dry the next day" (engine-2 facts, now wrong for engine 3).
Estimates / open: OPEN_SHARE 0.6, FAT, 1.5-coat stroke, titanium 0.9 (no tube); Golden's absolute times vs the artist guides left unresolved; no source range for vermilion, zinc white, smalt, Naples yellow, Antwerp blue, viridian, green earth, magenta, bone brown, bitumen (kept old rates).
Pre-existing failure (not this branch): `scripts/tests/replay_env.sh` (replay_clip `--length`), fails on 7b80cb0 too.

## rag (`origin/rag`, b2be109)
Pictures: `notes/rag/rag_wipe_out.jpg` (compare with the Palesca / Downing-White handouts), `notes/rag/rag_vs_brush_wipe.jpg`, `notes/rag/rag_passes_and_drying.jpg`.
API (engine 3 only): `r = rag()`, `r:wipe(mask|pts, {pressure, angle, passes, refold})`, `r:blot`, `r:refold()`, `r:dip(x)`; `r.load/.soaked/.damp/.fold/.width` read-only (writes are errors).
Where to look:
1. `crates/paint/src/rag.rs`: the lift model; every parameter marked [E] (estimate). Spirits: lift x(1 + 8*amount), new field `damp`.
2. `crates/easel/src/draw_rag.rs`: verb, cloth state held by the session (rolled back on a failed chunk; not in the save file, like brushes).
3. Engine gate: `rag` global registered only for engine 3 (test `an_older_log_sees_the_globals_it_saw`: 69 globals, same order).
Honest shortfall: a spirits rag removes 97-99% of the film but leaves 14% of the tone's color (the thin stain the cloth can't take). Not loosened; reported. Design question: accept, or let spirits take the stain too.
Tests changed: two header tests expecting `--@ engine 2` -> 3 (same as drying). A socket race in session_integrity seen once (unrelated, passed on rerun).
blend (finding 5): confirmed it removes 15/29/40% in 1/2/3 passes; unchanged.
## thinner - current work
Use the dedicated thinner spec draft 7, not the historical open-questions targets. Do not weaken its locked card target. Rag appearance remains to be inspected in the small interaction study; no engine-3 painting has validated it.

## r24-fixes (c4bb131): the external review's findings 1, 6, 7
- heap.lua: under engine 3 a failed chunk that touched a table whose `#` border could move is detected (repro: t[2]=2; failed t[1]=1;t[3]=3;t[1]=nil;t[3]=nil; then #t). Test added.
- rag.rs: `damp` evaporates, half every 3 min of painting time (estimate; Jennings 1902). Test added.
- drying.rs: range/order tests use the tubes in the build (pass with --no-default-features).
- Replays: 33a5bf and 3f29bf identical vs 3d0ea66 (digests minus secs=, output, PNG); new build slightly faster.

## sienna (c5963f8): user decision, 100% historical
- burnt sienna engine-3 rate 2.3 (~45 h), raw sienna keeps 1.2 (~86 h); Field/Salter 1869 §§50, 155. Test burnt_sienna_dries_before_raw_sienna. Modern W&N orders them the other way (noted in the code).

## prompt edits (af49348): user-approved
- notes/round24/runner/brief_template.md undo line names the rag; guide `blend` row says it lifts paint; three Lua 5.5 notes. Look at the wording.
