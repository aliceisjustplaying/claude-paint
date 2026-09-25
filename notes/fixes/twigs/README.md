# Twigs that float off their limbs (branch fix-twigs)

## What to look at

All images are lossless PNGs. Look at them at 100%: main on the left and
this branch on the right. The painting program is the same on both sides.
Only the engine differs.

- `2_whole_bare_tree.png`: a small bare tree at 3200 px, painted the way
  the round 10 and 11 painters painted theirs. Each limb and twig is one
  stroke that lifts off at its end, and the next twig is set on that
  stroke's path, often right on its end. On main the crown is full of
  loose dashes: twigs hanging in the air with a bare gap between them and
  the wood they grow from. On the branch every twig is joined to its limb.
- `1_junctions_in_the_tree.png`: two of those joints, at 1:1 and 4x. On
  main the parent stroke stops short and the child starts a few pixels
  away on bare ground. On the branch the parent draws down to a fine tip
  that runs into the child.
- `3_one_junction_study1_settings.png`: one parent and one child, with the
  r11-study1 painter's exact brush and ramps. Main leaves 4.7 units
  (15 px at 3200) of bare canvas between them. The branch leaves none.
- `4_side_effect_l5_near_grass.png`: what else changes. Every lifted stroke
  now reaches the end of its path, so the grass flicks in the l5_near
  benchmark are taller, with fine tips. On main they stopped short as
  stubs. Nothing else in that painting moved visibly.

## Verdict: a bug

The engine painted something other than what it documents. A stroke that
lifts off (`ramps` with a release) stopped laying paint before the end of
its path. It stopped where the pressure fell below the touch threshold of
the brush's first hair. So the end of a lifted stroke was bare canvas (in
the new test, 3–18 units of an 80-unit stroke, up to a fifth of it), and a
twig set on a limb's end started on nothing.

The docs say the tip stays down to the end:

- `Tool::point` (crates/paint/src/bristle.rs:124): "pressed lightly only
  the point touches (a hairline) … on the lift the mark draws down to a
  point".
- The blunt brush's hairs (bristle.rs:382): "a light touch or a lift-off
  gives just the tip".
- The easel guide (crates/easel/README.md:313–314): "a flick that ends in a
  hairline is a stroke whose pressure falls to 0".
- `Tool::mark_width` (bristle.rs:288): the mark has "a lower bound of
  about two hairs: the finest line the point draws".

## Cause (line numbers on main)

- `Gesture::pressure_at` (bristle.rs:531) fades the pressure over the
  release share, down to 5% of the stroke's pressure at the end.
- Every hair needs a minimum pressure before it touches (`thresh`,
  bristle.rs:387–400). No hair has a threshold of 0, not even the tip.
  The innermost hair of a round tuft sits at a small radius, and its
  length varies a little. In the probes here the marks stopped at a
  pressure of about 0.04 (pointed rigger 1.4: no ink at 0.04, 0.44 units
  of ink at 0.05) to about 0.08 (blunt sable 3: 0.04 units of ink at
  0.10, 0.02 at 0.08, none at 0.05).
- `drag_on` (bristle.rs:904–905) skipped every hair with `reach <= 0`.
  Once the lifting pressure fell under the lowest threshold, no hair
  touched and the stroke laid nothing more, although the hand hadn't
  reached the end of the path.

r11-study1's branching was sound: each twig starts on a point of its
parent's path (`bare_tree.rs` copies that code). Its workaround and
r11-astra's were ways around this bug. r11-study1 ended a parent at 45%
of its pressure and set each child at 90% of the parent's linear
pressure there.
r11-astra cut its release to 0.025 (`ramps(0.0, 0.025)` in its `line`).

## The fix

In `drag_on` (crates/paint/src/bristle.rs, at `let tip = …` and the
`reach` test just below it), the hair that touches first (the lowest
threshold) stays on the canvas at any pressure above 0. It touches with
zero reach, so it lays one hair's line. How deeply it wets the weave
still follows its load (`wick`, and `WET_REACH` for a blunt brush).
Nothing changes where that hair was already down, and no new knob is
added. A stroke is only different from main on the steps where it used
to have left the canvas.

## Measured

A small bare tree at 3200 px (`bare_tree.rs`), with the limbs in
r11-astra's pointed sable and the twigs in r11-study1's pointed rigger
and ramps. Dark pieces not joined to the tree (darkness over 0.15 of
ground to bark, bigger than 6 px), and lifted ends that have a child set
on them and touch the tree (`measure.py`):

| | main | fix-twigs |
|---|---|---|
| pieces off the tree | 10 (1571 px, largest 525) | 0 |
| lifted ends joined to the tree | 9 of 21 | 21 of 21 |

One straight parent and a child set on its end, 3200 px: bare canvas
along the joint, in units (1 unit = 3.2 px):

| settings | main | fix-twigs |
|---|---|---|
| r11-study1 final (pointed rigger 1.4, pressure 0.55→0.25, ramps 0.03/0.35) | 4.7 | 0 |
| r11-study1 first try (pressure 0.55→0, ramps 0.03/0.75) | 15.0 | 0 |
| blunt sable 3, ramps 0/0.35 | 1.9 | 0 |
| blunt rigger 1, ramps 0/0.35 | 3.4 | 0 |

## Tests

- New: `bristle::tip_tests::a_lifted_stroke_paints_to_the_end_of_its_path`.
  It uses four brushes (blunt sable, blunt rigger, pointed rigger and
  pointed sable) and two strokes (the guide's flick, and a limb lifted at
  a fork), each 80 units long. Every pixel column along the path must
  have paint. On main it fails in 7 of the 8 cases, with 3–18 units of
  the path left bare. On the branch it passes.
- Re-recorded, because the fix is meant to change their pixels (all
  three have lifted strokes): the golden scene
  (`crates/paint/tests/golden_scene.txt`), and the hand_time replay
  hashes for `paintings/lua/example.lua` (975 of 1.8 million pixels move
  at 1600 px, none in a patch bigger than 8 px) and `notes/loops/l5_near.lua`
  (the grass, image 4).
- `cargo test --workspace`: all pass. `cargo test --release -p easel
  --test hand_time -- --include-ignored`: 5 of 5 pass, l5_near included.
  `cargo clippy --workspace --all-targets`: nothing new. This toolchain
  (clippy 0.1.97) reports the same 58 warnings on main, none of them in
  the lines changed here.

## Not fixed, for Alice to decide

1. **Missing capability: a stroke's width at a point.** r11-study1's
   friction 4 asks for `Gesture::width_at(s)`. `Gesture::pressure_at`
   (bristle.rs:528) exists but is private, and nothing in the API gives
   the pressure or the width at a point along a gesture. Painters work
   out the linear pressure themselves, and they have to know about the
   ramps. That's a new API, and the feature freeze rules it out here.
   With this fix, a twig set anywhere on its parent's path does land on
   paint, so the feature is no longer needed for connection. It would
   still help match a child's width to its parent.
2. **A stroke that ends pressed, with no release, ends short by the
   trail of its hairs.** The hairs trail behind the hand
   (`Tool::length`: "How far tips trail behind the contact"). With no
   lift at the end they are still bent back when the stroke stops. A
   rigger, whose hairs are 5 times its width, ends about 0.6 × length ×
   pressure short: 1.3–2.2 units for a rigger 1.4 at 3200 px. A child set
   there shows a gap of 4–7 px. That is what `length` documents, and none
   of the painters' twigs ended pressed. A short release closes it.
3. **A blunt brush run dry has a pale tip.** Only a pointed tuft feeds
   paint from its belly to its tip. A blunt round painted past its load
   ends its lift in a faint dry line. For example, a blunt sable 4 over
   120 units ends at 25% fullness and 6.9 units short. Over 90 units it
   reaches the end. That is the documented dry-brush behavior. The
   painters' remedy of a fresh load or a smaller brush where the limb
   thins is right.
4. **`mark_width` overstates a blunt round's mark at light pressure.**
   The pressure's effect on which hairs touch is left out of it for a
   blunt tool. At 3200 px, a blunt sable 3 at pressure 0.5 asks for 2.18
   and lays 2.11. At 0.28 it asks for 1.63 and lays 0.99. At 0.19 it asks
   for 1.43 and lays 0.31. The blunt rigger stays close. This is a real
   mismatch, but it doesn't open gaps. Fixing it would change
   `pressure_for` and so every blunt `paint_wood` and fir stroke, so it
   needs its own decision.

## Reproduce

`bare_tree.rs` is a standalone program: a scratch cargo project with
`paint = { path = "<worktree>/crates/paint" }`, the release profile of
the workspace (`codegen-units = 1`), and `cargo run --release -- <dir>`.
It writes `tree.png` and `junctions.json` (the lifted ends that have a
child set on them). `measure.py` (numpy, scipy and pillow in a uv venv)
counts pieces and joints. `sbs.py` makes the side-by-sides. For main,
build the same program against `git archive main crates`.
