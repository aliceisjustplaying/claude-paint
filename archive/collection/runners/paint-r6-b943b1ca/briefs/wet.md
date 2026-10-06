# Stream: real wet-on-wet (worktree ~/src/a/claude-paint-r6-wet, branch r6-wet)

Read ~/tmp/paint-r6-b943b1ca/briefs/common.md first (rules, deliverable).

## The problem
The owner and both advisors think paintings look "pasted on": objects are
finished alone inside their own masks, over paint dried first. The
sketchbook (notes/sketchbook.md line ~25) prescribes "`dry()` before any
passage that goes over earlier work", and its pitfalls table (line ~542)
says paint laid over open paint gives "translucent rock, fog band, plowed
river". So painters avoid wet-on-wet because it misbehaves. Real oil paint
worked into open paint blends, drags and softens at contours; a loaded
brush laid into wet paint picks some up and deposits a broken, mixed edge;
a light stiff stroke over a wet dark sits on it with only a little
pickup ("wet into wet" skills: laying lights into darks, lost-and-found
edges, softening a contour with a clean dry brush).

## What to do
1. **Audit first (measure, don't guess).** Build a study (a study binary in
   paintings/src/bin or Lua under notes/wet/) that paints the same gestures
   over OPEN, SETTING, TACKY and DRY paint with identical brush, load and
   colors:
   - a light foliage dab/touch laid into a dark wet mass;
   - a sky color brought down across a still-wet hill edge, then the edge
     softened with a clean dry brush / badger (a lost edge);
   - a dark shadow stroke dragged along the base of a light form into the
     ground color (contact);
   - a thick light stroke over a thin wet dark (does it stay clean on top
     or turn to mud, and is that right for its load and stiffness?).
   Measure edge width (10–90% transition), how much underlayer is picked up
   and carried, color shift, coverage, halos, ploughing. Compare with what
   oil paint does (notes/research/oil_paint_physics.md and your knowledge;
   cite). Write the findings down before changing code.
2. **Fix what's wrong in the engine** (bristle.rs exchange, wet.rs,
   drying.rs, handling.rs) so wet edges soften and drag like paint and
   lights laid into wet darks stay clean in proportion to load and
   stiffness. Also check the specific failures named in the pitfalls table
   (translucent rock, fog band, plowed river) and whether they are physics
   bugs or painter misuse. Known limitation from notes/drying.md: "wet-in-
   wet within a pixel is still one mixture"; decide whether that matters
   here and fix only if it does.
3. **Evidence**: the audit study before and after (one image per gesture
   row), plus the two benchmark logs (notes/loops/l5_near.lua and
   notes/loops/l3_green.lua) at 1000px before and after, and a 3200px crop
   of a wet-worked passage in each if they change. Engine fixes here will
   change output: that's expected on this branch (re-record the golden,
   say so). The owner decides on merging in the morning, so the
   before/after images must make the change easy to judge.
4. **Sketchbook**: propose (on the branch) replacing the "dry() before any
   passage" default with guidance on when to work wet, tacky or dry, each
   with its ceiling.

Another stream (r6-time) makes painting verbs advance the clock (hand time,
sittings) in the easel and may add small hooks in drying.rs. Keep your
changes to shared files focused to limit merge conflicts.
