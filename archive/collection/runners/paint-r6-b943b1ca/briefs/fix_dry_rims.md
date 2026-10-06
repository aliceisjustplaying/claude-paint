# Bug fix: strokes over tacky or dry paint keep their outlines

Worktree: ~/src/a/claude-paint-fix-dry-rims (branch fix-dry-rims, from main).
Work only there; commit often; do not push. Scratch: run
`~/.local/bin/agent-tmp dryrims` once and export TMPDIR=TMP=TEMP to it.

## The bug (four painters, independently)
Read these first (branch:path, via `git show`):
- r8-arm1:notes/round8/arm1/notes.md, friction item 2: "A filbert ploughs
  its paint into rims": 399 µm of paint on the stroke's edges, 11 µm inside,
  so the sky showed through each stroke as a net.
- r8-arm3:notes/round8/arm3/notes.md, friction item 2: body strokes over
  tacky or dry paint make a lacy net of loop outlines instead of a covering film.
- r9-arm3:notes/round9/arm3/notes.md, friction item 4: wet over dry keeps
  every stroke's outline.
- r9-arm2:notes/round9/arm2/notes.md, friction items 1 and 3: thin dark
  strokes over light snow look like glass tubing (lighter middle, dark ridged
  edges); a pointed dark stroke draws a warm tan rim around itself.
A painter laying an opaque body stroke over dry paint expects a covering
film, thickest roughly where the brush pressed, not a ring.

## How to work
1. Reproduce minimally in a test in crates/paint (or easel, whichever is
   closer to the cause): one body stroke over a dried layer; measure film
   thickness across the stroke. Write the test FIRST and see it FAIL on main.
   The assertion must express the physical expectation (interior coverage not
   far below the rims), not a snapshot of numbers.
2. Diagnose the cause in the engine (deposit, pickup, exchange with the
   surface, drying state, bristle footprint...). Write the diagnosis down.
3. Fix the cause, not the symptom. This is a bug fix under a feature
   freeze: no new capabilities, no new knobs. Don't add noise or smoothing
   to hide it. Principles: notes/principles.md.
4. Check the related reports: are the tan rim and glass tubing the same
   cause? Fix only what shares the cause; list the rest.
5. Before finishing: `cargo test --workspace` and
   `cargo test --release -p easel --test hand_time` all green;
   clippy clean.
6. Visual before/after for Alice, lossless PNG: re-render one affected
   passage (e.g. r8-arm3's far hills or r9-arm3's snow shadows) on main vs
   your branch, same window, 1:1 crop, side by side, labeled, in
   notes/fixes/dry_rims/. Also check the round 2 winter
   (branch amnesia-winter) doesn't change for the worse; include it.
7. notes/fixes/dry_rims/README.md: the bug, the minimal test, the cause
   (with file:line), the fix, the before/after, anything left.

Final reply: the cause in two sentences, the fix, the test names, test
results, image paths. US English, no Oxford comma. Never write the user's
real name or an absolute home path into committed files; the user is
"Alice".
