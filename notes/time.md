# A hand in time (round 6, branch `r6-time`)

Paint dried on a clock (`notes/drying.md`: open, setting, tacky, dry;
`wait(minutes)` at the easel), but painting took no time. A `work{}` of
200,000 hatched strokes happened in zero minutes, so what was wet or dry
depended only on the `wait()` calls a painter wrote. The sketchbook's habit
of `dry()` before every passage then finishes each object alone over dry
paint.

The owner's principle: *a stroke costs the time a hand takes to make it; a
painter works in sittings of a few hours and paint sets between them.*
Economy and wet/dry timing should come from that physics, not from rules.

This stream adds:
- **the hand's ledger** (`paint::tally`): every mark the engine makes is
  counted as it is planned and priced in seconds of hand time;
- **hand time on the clock** (opt-in): the paint ages while the hand works,
  and a long pass ages in slices as it goes;
- **sittings**: `sitting{hours=3}`, `rest(hours)`, `timesheet()`, and a
  status line with the clock, the sitting and how much of the canvas is
  open, setting, tacky or dry;
- **`look --mode wet`**: the drying stages in false color over the picture.

Hand time is **off** unless a painting turns it on, and every existing log
replays byte for byte as before (tested).

## The API (easel)

```lua
canvas{style="friedrich", aspect=1.4, seed=7, hand=true}   -- hand time on from the start
hand_time(true)            -- or turn it on (or off) later; returns whether it was on
sitting{hours=3}           -- a new sitting starts now (a clean palette); sitting(3) too
rest(16)                   -- step away: the paint sets; the next mark starts a new sitting
rest()                     -- overnight (16 h)
t = timesheet()            -- {clock=, sitting=, sittings=, hours=, hand=, open=, setting=,
                           --  tacky=, dry=, strokes=, touches=, reloads=, piles=, hand_min=}
print(string.format("%.0f min at the easel this sitting, %.0f%% of the canvas open", t.sitting, 100 * t.open))
```

- With hand time on, every painting verb puts its hand time on the clock
  and the drying model ages the paint by it (`Canvas::wait`). `work`,
  `blend`, `stipple`, `glaze` and the pencil lines do so when they finish.
  Strokes and touches made one at a time (`b:stroke`, `b:touch` and every
  motif verb built on them: `f:paint`, `t:paint`, `t:paint_wood`,
  `o:paint`, the rock seams) do so each time a minute has piled up. Every
  chunk ends with the clock up to date. `clock()`, `drying(x, y)` and
  `timesheet()` bring it up to date first.
- A long pass is painted in **slices of 15 minutes of hand time**, and the
  paint ages between slices. A sky that takes an hour has begun to set at
  its first passages by the time the hand reaches the last. With the paint
  ageing, the default passage order becomes a sweep down the region (the
  engine's checkerboard phases are a device for threads, not a hand's
  path). An `order=` you set is kept.
- `wait(minutes)` stays as it was. A wait of 2 hours or more is a rest (the
  next mark starts a new sitting). So is `dry()`, and so is the time a
  finishing verb spends drying the paint (`varnish`, `glaze`).
- Palette trips: a load of a held brush (`b:load`, `b:reload`) is a reload
  if a pile of that color is already on the palette this sitting, or a new
  pile to mix if not. The palette holds 16 piles, and a new sitting starts
  with a clean palette. Within a pass, the piles are judged by the color
  asked for (not the aimed result, so a crop prices the same).
- The reply after every chunk and `easel status` read:
  `clock 1175.6 min · sitting 2: 2.9 h of 3.0 h · hand time on · open 46% setting 0% tacky 54% dry <1%`.

**A sitting that runs long is reported, not ended.** Once a sitting passes
its hours, the chunk's reply says so, once for every hour over:
`sitting 2: 3.8 h at the easel, 3.0 h planned; finish the passage while it
is open, then rest(hours)`. An automatic rest would fall wherever the hand
happened to be, say halfway through a sky, and silently put the second
half on set paint. A painter finishes the passage while it is wet and then
stops. So the painter chooses where the break goes, and the easel keeps
the count honest.

```sh
easel look --mode wet            # open (blue), setting (green), tacky (orange), dry (gray)
easel look --mode wet --crop 400,280,800,480
```

`--mode wet` dims the picture to gray (its values still read) and tints
each pixel by where its paint is in drying, with a legend in the corner.
It combines with the other modes (`--mode wet,squint`). The older `--wet`
flag is still an alias of `--dried` (the paint as it will look once dry),
as before.

## The engine

- `paint::tally` (new): `Tally` (strokes, touches, path mm, reloads, piles
  mixed, wipes, pencil lines, seconds), the pace constants, `Piles` (the
  palette) and `batches` (slices of a pass). `Canvas::tally()`,
  `tally_mut()`, `stages()`, `stage_shares()`, `set_hand_time(slice)`,
  `hand_owed()`, `clock_hand()`, `mm_per_unit()`.
- Counting is additive and never changes what is painted:
  - `run_plans` (handling.rs) counts every planned stroke and dip;
  - `stipple` counts every planned touch and dip;
  - `Canvas::drag` and `Canvas::touch` (bristle.rs) count one stroke or
    touch each (one line apiece).

  The count is taken on the whole canvas, before the crop filter and the
  split among threads. So the ledger is the same at any thread count and
  in a crop, except for the look-and-fill dabs, which look at the pixels a
  crop holds.
- Slicing (hand time on only): the pass's tile order is cut into batches of
  about 15 minutes of hand time, and each batch runs as before, followed by
  `wait` (`Canvas::hand_pass`). Stroke ids are now allocated per batch.
  With one batch (hand time off) that gives exactly the ids of the old
  code. A mid-pass `wait` sets the drying watermark (`absorb`) at the
  newest id, so the strokes after it must take later ids. Otherwise their
  fresh paint inherits the cure and thickness of the film it went over
  (the first version did this: setting streaks along the slices' seams,
  regression test `the_slices_after_a_wait_are_fresh_work`). The fill and
  cut-in sub-passes use ids reserved in advance for crop determinism, so
  they aren't sliced; their time goes on the clock when the verb ends.
- `drying.rs`, `wet.rs` and the bristle physics are untouched (the r6-wet
  stream owns them). `bristle.rs` gains one counting line in `drag` and one
  in `touch`.

## How long a mark takes

Tags: [S] from a source (search summaries; I did not read the full
papers), [E] my estimate. All constants are in `paint::tally::pace`.

| move | model | numbers | basis |
|---|---|---|---|
| bringing the brush to the next stroke | Fitts: `a + b·log2(1 + D/W)`, D = max(stroke length, 2 widths), W = 2 mark widths | a = 0.1 s, b = 0.122 s/bit | b: MacKenzie's stylus reanalysis, 95–122 ms/bit [S] (https://www.yorku.ca/mack/phd-ch2.html). a includes lifting and setting the brush down [E] |
| drawing the stroke | steering law: `b·L/W`, W = 4 mark widths (at least 3 mm), speed at most 400 mm/s | b = 0.11 s | Accot & Zhai, "Beyond Fitts' law", CHI 1997, stylus in a linear tunnel: a ≈ 0.18 s, b ≈ 0.11 s [S] (https://citeseerx.ist.psu.edu/document?doi=c55bfa1f5f3c8aa97a102ef88dba0026041c38b7&repid=rep1&type=pdf). A brushstroke isn't a tunnel task: tunnel width, floor and top speed [E] |
| a touch (stipple, dab) | Fitts to a spot 2 tips wide, 3 widths away, plus 0.08 s in contact; never faster than tapping | 0.34 s (about 3 a second) | fastest index-finger tapping is about 5–6 Hz [S] (e.g. https://pmc.ncbi.nlm.nih.gov/articles/PMC9028619/), hence the 1/6 s floor; the rest [E] |
| reload from a pile | two reaches of ~300 mm to a ~10 mm pile (Fitts, ~5 bits, ~0.7 s each) and a turn in the pile | 2.5 s | [E] from the Fitts numbers above |
| mixing a new pile | | 20 s (plus the reload) | [E] |
| wipe on the rag | | 2 s | [E] |
| look-and-fill dabs | a reload serves about 8 dabs | | [E] |
| pencil, chalk | 40 mm/s, 0.3 s per line | | [E] |
| glaze, varnish | a 25 mm brush at 150 mm/s, each part gone over twice | | [E] |
| same pile | colors closer than 0.035 in OKLab; 16 piles on the palette | | [E] |
| sittings | 3 h; a wait of 2 h or more is a rest; `rest()` is 16 h; slices of 15 min; brush strokes put on the clock once a minute piles up | | [E] ("a few hours, a couple of times a day") |

Some strokes as the model prices them (a 440 mm Friedrich canvas, 0.44 mm
a unit):

| stroke | hand time |
|---|---|
| a 10 cm sweep with a 12 mm flat | 0.64 s |
| a 44 mm body stroke with a 12-unit filbert | 0.62 s |
| a 3 mm hatch with a round 1.8 | 0.39 s |
| a 20 mm twig with a rigger 1.2 | 1.5 s |
| a 10 cm hairline | 4.6 s |
| a stipple touch | 0.34 s |

**What the benchmark paintings cost** (the ledger of each log, hand time
off, so the pictures are unchanged): `notes/loops/l5_near.lua` is about
**73 hours** of hand time (243,000 strokes, 28,000 touches, 66,000 palette
trips). **53 hours** of that is the fir wood in chunk 7, which has 211,000
hatched strokes. `notes/loops/l3_green.lua` is about **52 hours**
(149,000 strokes). A painter needs 17 to 25 sittings of 3 hours for these,
which is within "Friedrich over days or weeks". But half of `near` is one
wood behind the clearing, and the hand's time is where mark economy shows
first. The sky is 32 minutes, the fir wood two working weeks.

## Evidence

All at 1000 px. The logs are in `notes/time/`; `easel run notes/time/<log>.lua`
reproduces each picture.

**1. A hill's edge painted into a sky at five moments**
(`notes/time/edge_timing.jpg`, crops of the crest enlarged 3×; logs
`edge_*.lua`). The sky is brought 10 units past the hill's line, then the
hill is painted:
- **`dry()` first** (the sketchbook habit, hand time off): the clock jumps
  5 days. The hill sits on top, crisp, a cut-out.
- **no wait** (hand time off): the hill's top strokes drag the open sky
  down into a pale, broken band. The edge is soft and lost.
- **the same sitting, right after the sky** (hand time on, the sky took 36
  min): the same as no wait. The sky's thick lead-white coat is still open
  at 2 h, so an hour of hand time changes little here.
- **after lunch** (`rest(4)`): the sky is 45% setting and 9% tacky. The
  hill drags less of it.
- **the next day** (`rest(16)`): the sky is tacky. The hill sits on it
  crisp, like `dry()`.
- **the wet look after lunch**: the sky setting (green) with tacky islands
  (orange), the hill open (blue).

So the edge is now decided by when the hill is painted, not by a rule.
Whether the soft band looks good is the wet-on-wet stream's question: here
the drag is pale and streaky, like a halo.

**2. The example in sittings** (`notes/time/example_three_ways.jpg`; logs
`paintings/lua/example.lua`, `notes/time/example_sittings.lua`,
`notes/time/example_meet.lua`). The same program three ways:
- **as logged**, a day between stages: the knoll is a flat, crisp, dark
  mass under a crisp spruce.
- **in three sittings** with hand time on (sky; then the ridge, knoll and
  mist wet into each other; then the spruce and the grass), the stages
  otherwise unchanged. The ridge was painted over the whole lower canvas,
  so the knoll went on over open ridge paint and came out cobbled with
  churned-up light paint, its top lost in the ridge. The spruce, hatched
  over setting mist, has lighter patches where the tack stripped the
  brush.
- **in three sittings, the ridge laid only where it shows** (stopping 10
  units below the knoll's line): the knoll's body is solid again. Only its
  top band drags the open ridge, a soft shoulder against the distance
  instead of a cut line. The ridge took 42 minutes of hand time instead of
  90.

The timesheet printed after each chunk: sky 43 min, ridge 90 (or 42),
knoll 83, mist 58, spruce limbs 11, needles 88, grass 3. Sitting 2 ran
3.8 h against 3 planned and said so.

The lesson for a painter working in sittings: in one sitting the paint
under a later passage is still open and comes up into it, so lay each
passage only where it shows, just past where its neighbor will meet it.
That is also the economical way.

**3. The wet look mid-sitting** (`notes/time/wet_midsitting.jpg`): the
example in sittings during sitting 2, after the ridge, knoll and mist.
Yesterday's sky is tacky (orange), today's ridge and knoll are open (blue),
and a few thin spots are setting (green). Before the id fix, this look
showed green streaks along the slices' seams; that is how the bug was
found.

`notes/time/example_meet.jpg` is the whole of the third version.

## Tests

- `paint::tally::tests` (engine):
  - the ledger is the same at 1 and 4 threads and in a crop;
  - hand time puts all of the ledger on the clock, and a sliced pass
    paints the same at 1 and 4 threads and differs from the unaged one;
  - the slices after a wait are fresh work (the regression above);
  - batches cut the order by hand time;
  - piles reload near colors and mix new ones;
  - fine strokes are slower per mm, and touches are no faster than tapping.
- `easel` `time::tests`:
  - the clock advances by the exact hand time of a stroke, a touch, a new
    pile, a reload and a wipe, and by the timesheet's hand time after a
    `work`;
  - hand time off takes no time and paints bit for bit as without the
    option;
  - sittings, rests, overrun notes, undo and waits as rests;
  - a finishing verb's drying is not time at the easel.
- `look::tests::the_wet_look_shows_open_setting_tacky_and_dry`.
- `crates/easel/tests/hand_time.rs`:
  - `existing_logs_replay_unchanged`: `paintings/lua/example.lua` and
    (in release builds) `notes/loops/l5_near.lua` at 160 px give the PNG
    hashes recorded with the easel of commit 2c5a658, built in the same
    profile (debug and release floats differ); re-recorded for the
    wet-on-wet engine (r6-wet, `notes/wet.md`), which changes output on
    purpose (`PRINT_HASHES=1` prints them);
  - `hand_time_is_the_same_at_any_thread_count`: a hand-time program at
    `RAYON_NUM_THREADS` 1 and 4 gives identical PNGs and clocks.
- By hand: `l5_near.lua` and `l3_green.lua` at 1000 px are byte-identical
  (`cmp`) to the renders from before this branch.

## Review fixes (after the merge)

A code review (gpt-6-astra) found three defects. Each now has a regression
test that failed on the merged code:

1. **A checkpoint dropped hand time.** `Canvas::write_state` stored
   neither the slice setting nor the ledger. A resumed hand-timed painting
   had hand time off and an empty ledger: its passes no longer aged between
   slices, and the time still owed was lost. Checkpoint version 7
   (`PAINTCK7`) stores both, including `clocked`. The r6-wet branch drafted
   its two-layer film as version 7 too, so it becomes 8 when it merges
   (see the note in `checkpoint.rs`). Test:
   `tally::tests::a_checkpoint_keeps_hand_time_and_the_ledger`, which
   checks an uninterrupted run against a checkpointed one with time owed:
   ledger, clock bits and pixels. The easel's own checkpoints already
   carried the hand state; `time::tests::edits_and_undo_keep_the_hand_state_of_a_replay`
   now pins it (an `edit` and a deep undo match a fresh replay's clock,
   timesheet and picture).
2. **Clock queries after a finish reused its drying.** `varnish()` dries
   the paint without telling the studio clock. The flush left that
   interval unreported, so every later `timesheet()`, `clock()` or
   `drying()` in the chunk saw it again and started another sitting
   (2, 3, 7), and the sitting could go negative. Now the flush takes the
   interval into the clock exactly once and banks it for the chunk's
   "passed while the paint dried" note, which still appears once. Test:
   `time::tests::queries_after_a_finish_consume_its_drying_once`. One
   side effect: `clock()` called after a finish in the same chunk now
   returns the true clock, where before it returned the clock from before
   the finish until the chunk ended.
3. **Hand time overrode an order asked for.** With slicing on, explicit
   `order="scatter"` and `order="passages"` also became a sweep down.
   `Handling::order_set` (set by `order()` and `sweep()`) now separates a
   choice from the default, and only the default is swept. A preset's own
   order counts as a default. Test:
   `tally::tests::hand_time_keeps_an_order_asked_for`, which reads the
   paint's age: a sweep leaves the top rows about 2.3× as cured as the
   bottom, while explicit scatter and passages age the whole area alike.

Hand time off is unchanged by all three: the benchmark logs are still
byte-identical at 1000 px and the golden scene is unchanged.

## Known issues and ceilings

- **The numbers are estimates.** The movement laws are sourced; the tunnel
  width, the palette trips, mixing and the pencil are mine. The ranking
  should hold better than the absolute times: fine work costs more per mm
  than broad work, and trips to the palette cost the most.
- **The ledger prices what the simulation does, not what a painter would
  do.** A hatch pass reloads every few strokes because the simulated
  brush's reservoir says so (66,000 trips in `near`). A real painter on a
  fir wood might carry more paint. The trips are about 60% of `near`'s
  time (66,000 × 2.5 s ≈ 46 of 73 hours).
- **A slice is 15 minutes.** Within a slice the strokes go on at once. The
  paint's age follows a sweep down the pass, not a painter's actual path
  across it.
- **Crops.** A `look --scale` crop prices the look-and-fill dabs from the
  pixels it holds, so its clock can differ from the whole canvas's by those
  dabs, and its drying by a little.
- **A pass that runs past the gel point** (hours of hand time) can have its
  first slices baked before the look-and-fill runs. The fill then sees the
  set pixels as bare and dabs into them. This is untested at that length.
- **Hand time never rests on its own.** A painting that turns it on and
  never rests reports the overrun and goes on (by design, see above).
- **The wet-on-wet look is not mine.** How open paint behaves at a contour
  (the pale drag band) belongs to the r6-wet stream, which also decides
  whether the sketchbook's `dry()` rule goes.

## Next

- Reprice the palette trips once r6-wet settles how much paint a brush
  carries. Then a painting's timesheet is a fair measure of its economy
  (the lab's studies could report it next to the owner's verdict).
- Paint `near` and `green` in sittings once r6-wet lands, with passages
  laid to meet, and compare them with the dried-by-day logs.
- If a slice proves too coarse for skies, slice by stroke count within a
  tile. It is deterministic, but it costs a `wait` per slice.
