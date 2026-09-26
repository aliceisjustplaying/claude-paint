# The paintings, ported to today's API (both engines)

Every painting program we have, copied into `paintings/src/bin/ab_*.rs`
(Rust) or `paintings/lua/ab_*.lua` (easel logs). The files are the same on
branch `wet-before` (main's engine, `4abe5b4`) and branch `wet-merge` (the
wet engine merged with main). The painter's choices are unchanged:
composition, colors, tools, strokes, seeds, order and finish. Changes were
made only where today's API refused the program; each one is listed below.
The renders are in `README.md`.

    cargo paint ab_r10_arm3 -- --full --width 2400 --out <file>.png
    easel run paintings/lua/ab_r7_arm1.lua --width 2400 --out <file>.png

## Unchanged (copied byte for byte)

| file | painting | source |
|---|---|---|
| `ab_r7_arm2.rs` | round 7 arm 2 (B), a pond with poplars, Rust | `r7-arm2:paintings/src/bin/pond_poplars.rs` |
| `ab_r8_arm2.rs` | round 8 arm 2 (Z), "Winter Evening, the Way to the Ruined Choir" | `r8-arm2:paintings/src/bin/winter_ruin.rs` |
| `ab_r9_arm2.rs` | round 9 arm 2 (M), "Winter Morning on the Ryck" | `r9-arm2:paintings/src/bin/r9_ryck_winter.rs` |
| `ab_r10_arm1.rs` | round 10 arm 1 (A), "Dolmen in the Snow at Dusk" | `r10-arm1:paintings/src/bin/r10_winter_a.rs` |
| `ab_r10_arm2.rs` | round 10 arm 2 (C), "Winter evening by a frozen pond" | `r10-arm2:paintings/src/bin/r10_winter_b.rs` |
| `ab_r10_arm3.rs` | round 10 arm 3 (B), "Summer Afternoon: the Lime Tree on the Rise" | `r10-arm3:paintings/src/bin/r10_summer.rs` |
| `ab_r11_astra.rs`, `ab_r11_fable.rs`, `ab_r11_flash.rs` | round 11's three winters (Astra, Fable 5.1, Gemini Flash) | `r11-*:paintings/src/bin/r11_winter_*.rs` |
| `ab_r14_p1.rs` | round 14 painter 1, "Morning in the Mountains" | `r14-p1:paintings/src/bin/r14_p1.rs` |
| `ab_r14a_p1.rs` | round 14's first attempt, the ploughed field | `r14a-p1:paintings/src/bin/r14_p1.rs` |
| `ab_r14_p2.rs`, `ab_r14_p3.rs` | round 14 painters 2 (evening town on the coast) and 3 (Baltic shore) | `r14-p2`, `r14-p3` |
| `ab_r15_p1.rs`, `ab_r15_p2.rs`, `ab_r15_p3.rs` | round 15: misty mountains; an oak in snow with a church; a Baltic shore | `r15-p1..3` |
| `ab_easel3_free.lua`, `_green`, `_near` | round 3 at the easel | `easel3-*:paintings/lua/easel3_*.lua` (= `notes/amnesia3/*.lua`) |
| `ab_easel4_free.lua`, `_green`, `_near` | round 4 at the easel | `easel4-*` (= `notes/amnesia4/*.lua`) |
| `ab_r7_arm1.lua` | round 7 arm 1 (A), willows, easel without procedural tools | `r7-arm1:paintings/lua/willows.lua` |
| `ab_r8_arm1.lua`, `ab_r8_arm3.lua` | round 8 arms 1 (X) and 3 (W), frozen ponds | `r8-arm1:paintings/lua/frozen_pond.lua`, `r8-arm3:paintings/lua/pond.lua` |
| `ab_r9_arm1.lua`, `ab_r9_arm3.lua` | round 9 arms 1 (K, pollard willows) and 3 (N, wayside cross) | `r9-arm1:paintings/lua/winter_willows.lua`, `r9-arm3:paintings/lua/r9a3_winter.lua` |
| `ab_l5_near.lua`, `ab_l3_green.lua` | the best loop logs | `notes/loops/l5_near.lua`, `notes/loops/l3_green.lua` |

Every r14 and r15 file equals the one in its painter's folder
(`~/src/a/paint-r14-p1` etc.). The r7-r11 arm worktrees have no
uncommitted changes, so each branch holds the painter's final program.

Main's own `fresh2_winter` and `friedrich_moonrise_valley` are rendered as
they are on main, with no copy. Round 2's winter (branch `amnesia-winter`)
*is* main's `fresh2_winter`: round 7 ported it with one forced change,
the Cracks literal (`notes/round7/winter_port.md`). It is rendered once,
under that name.

## Changed

### `ab_r7_arm3.lua`: round 7 arm 3 (C), the bodden (`r7-arm3:paintings/lua/bodden.lua`)
Removed the header line `-- sittings enforced: ...`. That line makes the
easel strict: once a sitting reaches its length, it refuses marks until a
rest. On today's engine (both of them) the hand's ledger for sitting 4
comes out at 3.2 h of the 3.0 h the painter planned. A pass fills the gaps
its strokes leave, so its hand time follows where the paint landed
(`notes/wet_merge/MERGE.md` §3). The easel then refused chunk 9 and
everything after it. Without the line the log replays as an unenforced one,
every mark as the painter made it. The other five strict logs (r7 arm 1,
r8 arms 1 and 3, r9 arms 1 and 3) replay within their sittings and keep
the line.

### `ab_r2_coast.rs`, `ab_r2_mountains.rs`: round 2's coast and mountains
(branches `amnesia-coast`, `amnesia-mountains`, = `paintings/fresh2/`)
The same forced change as round 2's winter: in `paint::Cracks`,
`island_mm`, `ground_um` and `width_um` became `Option<f32>`. The
painter's values are wrapped in `Some(..)` (the coast: 5.0, 140.0, 32.0;
the mountains: 40.0 and the style's summed ground).

One difference from the winter port: both programs fill the rest of the
literal with `..paint::Cracks::aged(0)`. They therefore get today's
`aged()` for the fields that came after round 2 (uneven aging, varnish
veil, hierarchy, patches, grain, grime). The winter port had to write
every field out and set those six to 0 to keep round 2's one even web.
Here the minimal change is `Some(..)`, so the new fields follow today's
default. Both engines get the same value.

The coast needed a second change: two NaN skips. Its `hem(x)` and the
rope's coil use `sin(..).powf(..)`, and at the end of their range the sine
of an f32 π comes out a hair below zero (−8.7e-8), so `powf` gives NaN.
Round 2's engine took such a gesture and laid nothing for it. Today's
engine panics instead: `Gesture::validate` ("a NaN point would otherwise
make the brush take one step and lift, silently", `crates/paint/src/bristle.rs`).
The port skips exactly those marks, which laid nothing in round 2:

- the rope's last segment (`pts[9]` to `pts[10]`), whose end point is NaN;
- the cork floats whose `y = hem(x) - 3.0` is NaN (from about x = 458 on).

Each skip has a `(port: ..)` comment in the file.

### `ab_r1_coast.rs`, `ab_r1_mountains.rs`, `ab_r1_winter.rs`: round 1
The three paintings of the first amnesia round (worktree
`claude-paint-fresh`, 2026-09-22), archived on main as
`paintings/fresh/fresh_*.rs` (`dddd9af`, not built there). They were
written against a much older API.

1. **Stop points.** Round 1's `run.stage(&mut c, "x")` came *after* a
   stage's painting and returned true only for `--stop x`. It printed the
   time and changed nothing else (`paintings/src/run.rs` at `dddd9af`).
   Today's `stage("x", ..)` *begins* a stage and returns true to paint it.
   Ported as-is, every `if run.stage(..) { return; }` would stop the
   painting. Removed (9, 10 and 9 of them); a whole render is unaffected.
2. **`finish`** takes the checkpoint state: `run.finish(&mut c, &mut (), &fin)`
   (`()` keeps nothing).
3. **`Paint`** has scattering (Kubelka–Munk) where it had a `hiding` field:
   - `Paint { color: c, hiding: h, stiff: s }` → `Paint::new(c, h, s)`;
   - `Paint { hiding: h, stiff: s, ..p }` → `p.with_hiding(h).with_stiff(s)`;
   - `pt.hiding` → `pt.hiding()` on a `Paint`.

   `Mixture::hiding` is still a field and is left alone. `Palette::paint(color,
   medium)` means what it meant then (a mix thinned with `medium`).
4. **Figures take paint** (mountains, `man_in_cape`). Round 1's figures
   took colors and made paint of them themselves: `body` at hiding 0.97,
   stiffness 1.0; `thin` at 0.95/0.45; `lean` for the rim at 0.55/0.4.
   Today's take `Paint` and derive `thin` and `lean` from it. The four
   colors are passed as `Paint::new(color, 0.97, 1.0)`. That makes `body`
   exact; `thin` comes out at hiding 0.97 instead of 0.95 and the rim's
   `lean` at 0.58 instead of 0.55. The figure's strokes are unchanged: the
   two functions differ only in their types (diffed).
5. **The oak** (winter). `paint::Oak`, the engine's gnarled-oak motif,
   was removed in `26bdfb6` (`growth` replaced it). Its code
   (`crates/paint/src/tree.rs`, the same from `dddd9af` to its removal) is
   copied into the program as `mod oak`, unchanged except for its imports,
   `c.f` → `c.frame()` and its two `Paint` literals (as in 3). The
   painter's oak is grown and painted by the code they used.

**The winter was not rendered.** Its snow-impasto loop never ends, on
either engine. Once `xe` reaches `x1` its next step is
`x = xe - r.range(20.0, 45.0)`, which moves `x` back from `x1`; the next
`xe` is `x1` again, and so on forever. Both renders were stopped by
`timeout 1800` (exit 124). The picture Alice saw must have come from a
different version of the file than the one archived at `dddd9af`, so the
port was left as it is rather than guessing a fix.

All three build without warnings. The round 1 engine differed in much more
than the API (paint optics, brushes, drying), so these are round 1's
programs on today's engines, not round 1's pictures.

## Not ported

- **r7-python** (`experiments/python_friedrich/paint.py`): the control
  experiment in plain Python with no engine. It has nothing to run on
  either engine.
- Not in the set, for the record: Evening at a Mountain Lake
  (`paintings/lua/evening_lake.lua` on `r7-paint1` / `r7-paint-wet`), the
  round 11 trunk studies and the round 12-13 tree studies.
