# Thermos review of Round 6 (synthesis)

Eight reviewers, blind to each other: four model families (Opus 5.5 high,
gpt-6-astra low, Muse Spark 1.3 xhigh, GLM 5.3 Flash high) × two kinds
(correctness and breakage; code quality). Scope: `2c5a658..main` (at
`d5c53f0`) and the unmerged `r6-wet`. Their full reports are in the
integrator's scratch folder; this is the deduplicated list, weighted by how
many reviewers found each item independently. "✓" means the integrator
checked it in the code.

## Bugs

| # | bug | set | where | found by |
|---|---|---|---|---|
| B1 | A long hand-timed pass paints over its own set paint: early slices gel during the pass, and the gap-filler treats gelled paint as bare (a 502 min pass: 1 fill dab with hand time off, 353 on) | main | `handling.rs:644`, `drying.rs:435` | Astra, Opus (probe) |
| B2 | `ruler()` sets Scatter without `order_set`, so hand time still overrides it ✓ | main | `handling.rs:215` | Opus ×2, GLM (measured) |
| B3 | The plough's stiffness counts the set-aside film as body paint (10× wrong movement in a test) | r6-wet | `bristle.rs:1601` | Astra (reproduced), Muse |
| B4 | `varnish()` skips the hand clock: owed time lands after the varnish dried the canvas, and varnishing costs no hand time | main | `api.rs` varnish | Muse, Opus |
| B5 | Every `work`/`stipple` starts with an empty palette: each pass's first dip is billed as a new mix (1–2 h over a few hundred calls) | main | `handling.rs:812`, `stipple.rs:505` | Opus ×2, Muse, GLM |
| B6 | The hand ledger differs in a crop (fill dabs and aimed dips judge the window) | main | `tally.rs:14` doc | Opus (probe), GLM |
| B7 | The dirty brush tip is folded back every step for rounds and riggers, so it does nothing there | r6-wet | `bristle.rs:930` | Opus |
| B8 | PAINTCK8's film block isn't validated on read (NaN, `top > vol` load silently) | r6-wet | `checkpoint.rs` | Muse |
| B9 | Memory: 52 bytes/pixel, not 44 (~373 MB at 3200); `mid`/`midx` (always 0 between strokes) are copied into every undo snapshot | r6-wet | `wet.rs` | Opus ×2, Muse, GLM |
| B10 | Merging r6-wet needs a deliberate re-record of the golden and replay hashes (both sides re-recorded against different engines) | r6-wet | tests | Opus, GLM |

## Structure (all four quality reviewers unless noted)

| # | problem | fix |
|---|---|---|
| S1 | The timed-pass batching loop is duplicated in `handling.rs` and `stipple.rs`, the copies already disagree on "sweep down", and the new loop was pasted in without reindenting | one runner owning batches, stroke ids, crop filter, dirty bounds and the waits between slices; one `sweep_down` order; `Option<Order>` instead of the `order_set` flag (fixes B2 by construction) |
| S2 | The wet film is parallel "total + part" arrays edited through raw pointers; conservation is a protocol callers must remember; `bristle.rs` grows 1,898 → 2,357 | a `Layer { v, lat, hide }` type owned by `wet.rs`; typed parcels from pickup; a stroke-scoped reservation that settles itself; `exchange`'s per-bristle physics in one pure `contact()` (fixes B3, B9 by construction) |
| S3 | `broadleaf.rs` 1,510 → 2,000: tree growth fused with the painter's wood selection | move `drawn`, `wood_strokes`, `twig_mass` to their own module; one `polyline` length/arclength helper (currently written 3–6 times) |
| S4 | Hand-time `flush` calls scattered over 13–16 easel verbs, with ordering rules kept by habit (Opus, GLM, Muse) | one wrapper that owns the rule; ideally the engine owns the hand/idle split |
| S5 | `settle` and `settle_film` duplicate the band setup and the conservation epilogue (Opus, GLM, Muse) | shared helpers |
| S6 | Small: `touch_secs` ignores its argument; the drying-stage rule copied into `tally.rs`; mixed seconds/minutes names; hand-written Tally serialization; the wet studies copy their measuring code (and the copies drifted) | as named |

## Clean (checked by several reviewers)
Determinism (thread count, crop, replay with hand time off), wet-film
conservation per moment, the varnish fix, the relief change, the easel
root fix, the bare-tree changes, PAINTCK7.

## Verdicts
main: sound but needs B1, B2, B4, B5 fixed and S1 before more work lands
on the timed passes. r6-wet: the physics is careful and well tested, but
restructure around a `Layer` type (S2) before merging, which also fixes
B3 and B9; then B7, B8, B10.

## Status (end of the maintenance round)
All fixed, each bug with a test that failed first; with hand time off the
benchmarks are byte-identical on main.
- main (merged `55a85df` maint-b, `8c5bf96` maint-a): B1, B2, B4, B5, B6
  (documented), S1 (`Canvas::paint_pass` in sched.rs, `TileOrder`), S3
  (`broadleaf/wood.rs`, `path::{length, arclen}`), S4 (`time::verb`), S5
  (shared settle helpers; settle_film's bounds now exact, GLM's suspicion
  was real), S6 (timing items).
- r6-wet (unmerged, `accfd87`): S2 (`Layer`, `film.rs` Stroke reservation,
  `exchange.rs` `contact()`; bristle.rs 2,357 → 1,638), B3, B7, B8, B9 (44
  B/px again: the set-aside film is stroke-local), the studies' shared
  module, B10 (main merged, golden and replay hashes re-recorded for the
  merged engine).
- Also found during the round, by two agents independently: the release
  profile's incremental codegen moved a few pixels by 1/255 between
  rebuilds. Fixed in `3060281` (incremental = false, codegen-units = 1):
  two clean builds now give byte-identical binaries and renders.
- Left: the engine owning the hand/idle split (S4's "ideally"); storing
  the wet film's parts disjointly (changes float rounding); `tally::path_len`
  → `path::length`; the `Sweep(π/2)` float wobble in `tile_order`.
