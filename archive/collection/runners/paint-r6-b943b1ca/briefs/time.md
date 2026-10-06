# Stream: hand in time (worktree ~/src/a/claude-paint-r6-time, branch r6-time)

Read ~/tmp/paint-r6-b943b1ca/briefs/common.md first (rules, deliverable).

## The problem
Paint dries on a clock (notes/drying.md: open → setting → tacky → dry,
`wait(minutes)` in the easel), but painting itself takes no time: a `work{}`
of 20,000 strokes or a 30,000-touch stipple happens in zero minutes. So the
timing of wet and dry is entirely whatever `wait()` calls the painter
writes, and the sketchbook's habit is `dry()` before every passage, which
likely causes the pasted-on cutout look (objects finished alone over dry
paint, never meeting their surroundings wet).

The owner's principle: "a stroke costs the time a hand takes to make it; a
painter works in sessions (a few hours, a couple of times a day) and paint
sets between them. Economy and wet/dry timing then come from physics, not
rules. Friedrich over days or weeks is fine; so is a fast alla prima day."

## What to build
1. **Hand time per mark.** Every painting verb in the easel (work, stipple,
   blend, glaze, the hand/motif verbs, the tree/fir/rock painting verbs:
   audit crates/easel/src/api.rs and friends) advances the easel clock by a
   plausible time for the marks it actually made: from stroke count, path
   length, brush size and kind, touches, and trips to the palette to reload
   or remix. Find the numbers from knowledge and sources where you can
   (brush speeds, touches per second for stippling, time per palette
   trip); tag estimates as estimates. The engine may need to report what a
   handling actually did (strokes, length, reloads): keep engine changes
   additive.
2. **The paint ages while the hand works.** Apply the elapsed time to the
   canvas with the existing drying (`Canvas::wait`) at a sensible grain (at
   least per verb; finer, e.g. per tile batch or per N strokes, if cheap and
   deterministic). Keep it deterministic and thread-count independent.
3. **Sittings.** A painter works in sittings of a few hours and rests
   between them. Design a small API, e.g. `sitting{hours=3}` / `rest(hours)`
   or a style/profile setting, and have `status` report the clock, the
   current sitting's elapsed time and how much of the canvas is open,
   setting, tacky or dry. Decide whether a sitting that runs long warns or
   rests automatically; explain the choice.
4. **See it.** `easel look --mode wet` shows open, setting, tacky and dry
   areas (a clear false-color overlay on a dimmed picture).
5. **Opt-in and replay safe.** Existing logs (notes/loops/l5_near.lua,
   notes/loops/l3_green.lua, paintings/lua/*) must replay byte-identically:
   hand time is off unless a painting turns it on (e.g. `hand_time(true)` or
   `canvas{..., hand=true}`). Test that.
6. **Tests**: clock advances by the expected amount for known verbs,
   determinism across thread counts, replay of an existing log unchanged,
   `--mode wet` renders.
7. **Evidence**: a small study (Lua under notes/time/ or a study binary)
   showing one passage painted twice, with hand time off and on, e.g. a
   sky then a hill edge painted into it within a sitting vs the next day:
   show the edge where they meet. Plus a `look --mode wet` image mid-sitting.
8. **Sketchbook**: a short note on working in sittings (principle first,
   with its ceiling), replacing nothing yet: the wet-on-wet stream decides
   the fate of the "dry() before any passage" rule.

Another stream (r6-wet) is auditing and fixing what open paint does at
contours (bristle.rs / wet.rs / drying.rs). Avoid restructuring those files;
keep your drying-side changes small and additive to limit merge conflicts.
