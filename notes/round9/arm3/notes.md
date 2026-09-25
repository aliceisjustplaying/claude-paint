# Round 9, arm 3: a winter landscape at the easel

Source: `paintings/lua/r9a3_winter.lua` (easel session log).
Renders: `painting_1000.png`, `painting_3200.png` (this folder).

## Composition and why

A winter evening just after sunset. A low horizon (y 468 of 714) under a big
quiet sky, gray-violet above and a lemon-rose glow low on the left where the
sun went down. On a snow mound left of center stands an old dead oak, the
picture's main vertical, its crooked limbs against the glow (Friedrich's
*Oak Tree in the Snow*, the dead oaks of *Abbey in the Oakwood*: the tree as
a figure). To the right, lower on the slope, a leaning wooden wayside cross
half buried in snow, and a lone walker in a dark coat with a stick, seen
from behind, going toward it (the Rückenfigur; the cross in winter recalls
*Winter Landscape with Church* and *Morning in the Riesengebirge* without
copying either). Far off, a thin dark band of fir wood on the plain, hazed by
the evening air. Near foreground: snow with a few dark stones, dry grass and
bramble stems poking through (grass laid over finished snow, as NG p.56
reports for *Winter Landscape*), and the walker's footprints.

## Method, stage by stage

1. Canvas (friedrich style, after-1820 palette), aspect 1.4.
2. Underdrawing: horizon ruled in 2H, mound and oak trunk sketched, cross
   and walker drawn in HB.
3. Sky: two broad lay-ins wet into wet (long sweeps, then shorter fuller
   ones to close the gaps), then one blend. No stipple.

## FRICTION

1. **Long broad sky strokes leave the ground showing.** The first
   `hand="broad"` pass (lengths 120 to 320, coverage 4.5) left dozens of
   orange-ground holes in the upper sky, and with `tool="flat 14"` and a
   `badger 18` blend, diagonal drag streaks too. Workaround: a second,
   shorter (60 to 170), fully loaded pass wet into the first, then a plain
   `blend`. Coverage 4.5 doesn't mean covered when the strokes run dry.
2. **`easel try --look` shows the rolled-back canvas**, so it can't preview
   paint, only overlays. Workaround: `do`, look, `undo`.
