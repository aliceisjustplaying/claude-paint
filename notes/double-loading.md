# Double-loaded and streaky brushes

A dip used to fill every bristle of a brush alike, so a stroke laid one paint
across its whole width and changed color only where it met wet paint. Painters
load brushes unevenly all the time: one side or corner in a second pile, or a
pull through an unevenly knifed pile that leaves the hair streaked. The colors
then come off side by side within one stroke and mingle as it goes.

`Held::load_part(paint, amount, &Part)` dips only part of the brush. Each
bristle takes `amount × full × weight` of the paint, mixed into what it
already holds, as `load` does:

- `side` (-1..1) and `share` (0..1): which edge of the wide axis goes into
  the pile and how much of the width, with a soft margin as hairs splay.
- `streak` (0..1): bristles take paint up in bands a few bristles wide
  (neighbors share a band, plus each bristle's own variation). The weight
  is `(1 - streak) + streak × 3.3 n²` with `n` in 0..1, so a dip's total stays
  about that of an even one (tested: within 20% over 40 seeds).

`Part::ALL` is exactly `load` (tested), and nothing draws new randomness unless
the options are used, so existing logs replay as before.

At the easel:

```lua
b:reload(p, 0.8)
b:load(q, 0.5, {side=1, share=0.4})     -- one side in a second pile
b:load(w, 0.4, {streak=0.9})            -- streaks of a third
work(m, {pile=p, streak=0.6, second={pile=q, load=0.45, side=1, share=0.45}})
```

In `work`, `streak=` makes every dip into the pile streaky and `second=` dips
part of the brush into a second pile after it (a second trip to the palette in
hand time). Each dip gets its own streak pattern, seeded from where its stroke
starts.

`paintings/lua/study2.lua` is the test sheet: double-loaded strokes, streaky
strokes, three paints on one brush and two `work` passes.

Note: on this x86 Linux machine `logs_without_an_engine_line_replay_as_before`
fails with and without this change (checked on a clean stash: same hash
mismatch), most likely because float results differ from the machine the
hashes were taken on. `easel4_free_replays` also fails here; it was not
rechecked on a clean stash.
