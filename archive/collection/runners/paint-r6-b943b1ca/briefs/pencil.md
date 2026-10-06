# Pencil check (claude-paint), worktree ~/src/a/claude-paint-r6-pencil, branch r6-pencil

House rules: ~/tmp/paint-r6-b943b1ca/briefs/common.md. Tests:
`cargo test --workspace` (~1 min); before finishing also `cargo test
--release -p easel --test hand_time`.

Alice (the project's owner; call her Alice) saw "an outline" around the
rocks in several 3200px crops and asked: "is that like remnants of the
pencil?" The engine has an underdrawing (crates/paint/src/graphite.rs,
notes/pencil.md): graphite and chalk dragged over the ground, in the
picture under the paint ("thin paint shows it, body color hides it"),
with a kneaded eraser and fixative. Another agent (r6-glitch) is
cataloging all small artifacts; your question is only the pencil.

1. Find which of today's logs draw an underdrawing (grep for the pencil
   verbs in notes/lab/*.lua, notes/lab2/*.lua, notes/loops/*.lua) and
   render the rock passages at 3200 (`easel run <log> --width 3200 --crop
   ...`) with and without the pencil chunks (a copy of the log with them
   removed). Is the outline the pencil? Measure (a line profile across
   the edge) and show crops.
2. If it is: is it physically right (graphite under thin oil does show,
   and Friedrich's underdrawings show in thin passages), or too strong,
   too crisp, too dark, too continuous, or visible through paint that
   should hide it (check hiding/coverage math, the fixative, how many
   coats cover it)? Fix what's wrong in the engine with a test that fails
   first; if it's right but the painters draw too hard, write a
   sketchbook pitfall with a recipe (grade, pressure, erasing back,
   drawing only the big shapes).
3. If it isn't the pencil: say what it is if you can find out (varnish
   residue? the relief? a dark contact stroke in the recipe?) and hand
   the evidence to the glitch stream by writing it in notes/pencil_check.md.
Evidence for Alice (she views on another machine; commit it):
notes/pencil_check/alice_sheet.png (lossless, 1:1 crops, with and without
the pencil, and after any fix) plus notes/pencil_check.md. Benchmarks
byte-identical unless a fix means to change them. Budget about 1–1.5 h.
Final message: concise report with the answer, commits and image paths.
