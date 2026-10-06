# Wet control experiment (branch r6-wet), per the owner's decision

The owner decided: r6-wet is not merged yet; run the experiment. It's the
first next step from the second opinion (notes/advice/astra_r6.md §5.1,
on main): "Settle wet control. Compare current main, wet and
wet-plus-lift-off using identical strokes across load, pressure and drying
stages. Measure retained light, exposed ground and edge width at
1000/3200. Pass: both clean accents and deliberately softened edges, then
owner preference on repainted sky/water/rock without losing foliage/near
gains." Also read its §2 on r6-wet (it doubts "the physics is right" while
the stirring constants are estimates, and notes pass 3 regressed some
numbers vs pass 2).

First: main has moved (review fixes, relief default 0.06, r6-oak merged).
Merge main into r6-wet (`git merge main`; check for conflicts and markers
before committing; never chain commit after a merge); keep PAINTCK8.

## 1. The lift-off (the missing piece you named)
Implement how a stroke ends: a flat lifting off rolls onto its edge and
drags (the footprint narrows to the chisel edge, pressure falls, the
trailing bristles drag the wet film), a round lifts to its point, a filbert
between. Keep it physical (bristle geometry and pressure over the last part
of the stroke), not a cosmetic fade. It should soften stroke ends in wet
paint and stay crisp enough on dry paint. Decide whether it's a default or
a handling option; justify.

## 2. The controlled experiment
A study binary (paintings/src/bin/study_wet_control.rs or similar) with a
grid: rows = gestures (a loaded stiff light accent into a wet dark; a
stroke dragged along a wet contour to soften it; a flat's stroke ending in
wet paint; a thin glaze-hand veil over open paint), columns = drying stage
(open, setting, tacky, dry) × load (full, lean) × pressure (light, firm).
Render it on three engines: main (build it from a main worktree you create
under your scratch dir), r6-wet as it is, r6-wet + lift-off. Measure per
cell: retained light (clean fraction of accents), exposed ground (% of
track), edge width 10–90% (mm) at 1000 and 3200. A table in notes/wet.md
§8, and one sheet image per engine.

## 3. The repaints (only if 2 passes: clean accents AND controllable soft edges)
Repaint the panel's losing studies on r6-wet + lift-off: sky_B, water_B,
rock_B from notes/lab/ (on main) and the l3_green hedge: not by replaying
the old logs (they were tuned on main's engine; the advisor flagged that
as unfair), but by adapting them: copy each log and change only the
passages that depended on paint sinking (the lights, the soft edges), with
the same budget of chunks. Also re-render foliage_C and l5_near (the
unanimous wins) to check they didn't regress.

## Owner sheets (she views on another machine: everything must be committed)
- notes/wet/owner_control_sheet.jpg: the three engines' grids side by side
  (or stacked), labeled.
- notes/wet/owner_repaint_<S>.jpg for each repaint: main's version vs the
  repaint on r6-wet, whole (≤1600px wide) over matched 3200 crops, labeled
  with neutral letters only; the key in notes/wet.md.
Same rules as before (commit often, don't push, tests pass in debug, the
conservation test stays green, golden re-recorded only with a stated
reason). Budget about 2 hours. Concise report at the end: the table's
headline numbers, sheet paths, keys, and your recommendation.
