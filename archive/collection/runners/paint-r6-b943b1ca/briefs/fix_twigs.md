# Bug investigation: twigs that float off their limbs

Worktree: ~/src/a/claude-paint-fix-twigs (branch fix-twigs, from main).
Work only there; commit often; do not push or merge. Scratch: run
`~/.local/bin/agent-tmp fixtwigs` once and export TMPDIR=TMP=TEMP to it.
Wrap renders and tests in `timeout`; other agents share the machine.

## Evidence (read first; branch:path via `git show`)
- r11-study1:notes/r11_study1.md, friction 4: "Gestures don't report
  their width along the way. `Gesture::ramps` fades the pressure, but
  nothing tells you how wide the stroke is at a given point. Twigs
  branched off parent twigs that had already faded to nothing, so they
  floated in the sky."
- r11-astra:notes/r11_winter_astra.md, friction 1: "Brush taper left
  disconnected branch tips."
- r10-arm1:notes/r10_winter_a.md (it wrote its own recursive oak): Alice
  saw "twigs just sort of hanging in the air"; judges saw "detached"
  twig marks. Also its friction 2: pointed tips are off by default.
- Round 2's painter avoided it by hand: "brush handed down as the limb
  thins, set down in the last one's wet end"
  (amnesia-winter:notes/fresh2_winter.md, stage 8).
- Engine: crates/paint/src/hand.rs (Gesture, ramps), bristle.rs (tips,
  lift-off, `Tool::point`), growth.rs (Skeleton, limb widths),
  paintings/src (any limb painting helpers).

## How to work
1. Reproduce minimally: a parent stroke and a child stroke that starts on
   the parent's path near its end, painted the way these painters did
   (Gesture with ramps; the engine's own skeleton painting if there is
   one). Measure: is there a gap of unpainted canvas between parent and
   child? Where does the parent's mark actually end compared with its
   path's end? What width does a stroke have at a given point vs. what the
   painter asked?
2. Decide honestly which it is:
   a) a BUG: the engine paints something other than documented (e.g. a
      stroke's visible mark ends well before its path ends; the pressure
      ramp reaches zero width before the documented point; lift-off
      leaves a gap; a skeleton helper starts children thinner than the
      pipe model says) -> failing test first, fix the cause, no new knobs.
   b) a missing capability (e.g. no way to ask a gesture its width at a
      point) -> do NOT build it (feature freeze); write it down precisely
      with the evidence, for Alice to decide.
3. Before finishing: `cargo test --workspace` and
   `cargo test --release -p easel --test hand_time` green; clippy clean;
   re-record golden hashes only where the fix intends a pixel change.
4. For Alice (lossless PNG, notes/fixes/twigs/): before/after 1:1 of a
   parent-child junction and of a whole small bare tree painted the same
   way on main vs the branch, labeled, and a README whose first section
   says in plain words what to look at. If nothing was fixed, show the
   reproduction instead.

Final reply: bug or missing capability (with evidence), cause (file:line),
fix, tests and results, image paths. US English, no Oxford comma. Never
write the user's real name or an absolute home path into committed files;
the user is "Alice".
