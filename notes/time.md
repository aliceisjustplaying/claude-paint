# A hand in time (round 6, branch `r6-time`)

Paint dried on a clock (`notes/drying.md`: open, setting, tacky, dry;
`wait(minutes)` at the easel), but painting took no time. A `work{}` of
200,000 hatched strokes happened in zero minutes, so what was wet or dry
depended only on the `wait()` calls a painter wrote. The sketchbook's habit
of `dry()` before every passage then finishes each object alone over dry
paint.

Alice's principle: *a stroke costs the time a hand takes to make it; a
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
  `timesheet()` bring it up to date first, and so do the verbs that make
  the clock jump (`wait`, `dry`, `rest`, `glaze`, `varnish`, `cracks`,
  `relief`), so the time owed is spent before the paint dries. Brushing a
  glaze or a varnish on is hand time too (by its area), after the wait.
- A long pass is painted in **slices of 15 minutes of hand time**, and the
  paint ages between slices. A sky that takes an hour has begun to set at
  its first passages by the time the hand reaches the last. With the paint
  ageing, the default passage order becomes a sweep down the region (the
  engine's checkerboard phases are a device for threads, not a hand's
  path). An `order=` you set is kept.
- `wait(minutes)` stays as it was. A wait of 2 hours or more is a rest (the
  next mark starts a new sitting). So is `dry()`, and so is the time a
  finishing verb spends drying the paint (`glaze`, `varnish`, `cracks`,
  `relief`), which it reports (`varnish: waited 9.4 days for the paint
  under it to dry (clock 178325 min)`).
- Palette trips: a dip is a reload if a pile of that color is already on
  the palette this sitting, or a new pile to mix if not. One palette
  serves the sitting: held brushes (`b:load`, `b:reload`) and covering
  passes (`work`, `blend`, `stipple`, with their fill and cut-in) dip into
  the same piles. It holds 16 piles, and a new sitting starts with a clean
  palette. A pass judges its piles by the color asked for, not the aimed
  result (but with `color_over` the color asked for comes from the
  canvas; see the known issues on crops).
- The reply after every chunk and `easel status` read:
  `clock 1175.6 min · sitting 2: 2.9 h of 3.0 h · hand time on · open 46% setting 0% tacky 54% dry <1%`.

**A sitting that runs long ends (Round 7).** "Constraints are enforced,
not reported" (notes/principles.md, against reward hacking, item 4):
Evening at a Mountain Lake reported a 3 h sitting running 17.8 h and went
on painting. Now, with hand time on, once a sitting's time (the clock
since it began, with the hand time not yet clocked) reaches its hours, the
easel refuses every verb that marks the canvas (strokes, touches and the
motif verbs built on them, `work`, `blend`, `stipple`, `glaze`, pencil
lines, `varnish`) with `the sitting is over after 3.0 h: rest(hours)
first`. Queries (`timesheet`, `clock`, `drying`, `look`, `status`) still
answer; `wait`, `dry` and `rest` still run, and a rest (`rest`, or a wait
of 2 h or more) starts the next sitting. As any failing chunk, the
refused chunk is rolled back whole, so plan chunks by the time left
(`t.hours*60 - t.sitting` from `timesheet()`).

- **The passage in hand finishes, uncapped.** The rule is checked when a
  verb starts. A pass that begins inside the sitting runs to its end (the
  passage is finished while it is open, as a painter would), and its
  overrun is not cut. Strokes made one at a time stop at the next stroke,
  so only a pass can overrun far. Cutting a pass short would leave half a
  sky on the canvas and need the engine to stop mid-pass. The cost: a long
  pass started late overruns by its length (Evening Lake's sitting 3,
  replayed strict, stops in chunk 11 at 6.8 h of 3 h, after one pass).
- **No way around it from Lua.** In a strict session `sitting{hours=}`
  sets a sitting's length only before anything is painted in it (after
  `canvas{}` or a rest), at most 8 h; mid-sitting it is refused (it used to
  start a new sitting on the spot, which would dodge the rest). And
  `hand_time(false)` is refused once hand time is on. With hand time off
  there is no clock and no sitting to enforce.
- **Only new logs.** Every existing log must replay unchanged, overruns
  and all, so strictness is a property of the session, not of a Lua
  option a painter could leave out: `easel open` on a new name starts a
  strict session, and its log carries a header line the easel writes,
  `-- sittings enforced: a sitting ends at its length; ...`
  (`session::STRICT`). `easel run`, a resumed `easel open`, `check` and the
  `look --scale` crop sessions read it from the header. A log without it
  (every log from before) replays with overruns only reported, as it was
  painted: Evening Lake at 160 px replays byte-identically, its printout
  too, and so does `crates/easel/tests/logs/overran.lua` (a golden hash).
  A log edited to add the line is held to it on replay (a test).

The overrun note stays (`sitting 2: 3.8 h at the easel, 3.0 h planned;
finish the passage while it is open, then rest(hours)`). In a strict
session it now means that the pass that just ended was the last one.

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
  mixed, wipes, pencil lines, seconds), the pace constants and `Piles`
  (the palette). `Canvas::tally()`, `tally_mut()`,
  `set_hand_time(slice_min)`, `hand_owed_secs()`, `clock_hand_min()`,
  `mm_per_unit()`; `stages()` and `stage_shares()` live in `drying.rs`
  with `drying_at`, all three on one stage rule. `Canvas::work_with` and
  `stipple_with` dip into a caller's `Piles` (the easel passes the
  sitting's); `work` and `stipple` start a clean palette.
- `sched.rs`: `Canvas::paint_pass` paints a covering pass's tiles for
  both `work` (`run_plans`) and `stipple`. It owns the slices of hand time
  (`batches`), stroke ids per slice, the crop filter, the dirty bounds, the
  `hand_pass` between slices and the order rule: an order asked for
  (`Handling::order` is `Some`, which `order()`, `sweep()` and `ruler()`
  set) is kept, else hand time paints the tiles in `sweep_down`.
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
- The easel (`time.rs`): every verb that marks the canvas, reads the clock
  or moves it runs through `time::verb(st, Verb, f)`, which holds the
  rules: `Marks` put their hand time on the clock once a minute piled up,
  a `Pass` when it ends, a `Query` first; a `Jump` puts the time owed on
  first, then takes the jump into the studio clock, reports it, starts a
  sitting by its `Rest` rule and clocks the verb's own hand time.

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
  - `example_log_replays_as_recorded`: `paintings/lua/example.lua` at
    160 px gives its recorded PNG hash, a tripwire re-recorded on intended
    changes; `l5_near_replays_as_recorded` (`#[ignore]`d, run it with
    `cargo test --release -p easel --test hand_time -- --ignored`) does the
    same for `notes/loops/l5_near.lua`;
  - `a_strict_log_is_deterministic`: a hand-time program at
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
   the finish until the chunk ended. (Since replaced: every verb that
   moves the clock now takes the jump in itself; see the thermos
   maintenance below.)
3. **Hand time overrode an order asked for.** With slicing on, explicit
   `order="scatter"` and `order="passages"` also became a sweep down.
   `Handling::order_set` (set by `order()` and `sweep()`) now separates a
   choice from the default, and only the default is swept. A preset's own
   order counted as a default. (Since replaced by `Option<Order>`, and
   the ruler preset's scatter now counts as asked for: thermos B2.) Test:
   `tally::tests::hand_time_keeps_an_order_asked_for`, which reads the
   paint's age: a sweep leaves the top rows about 2.3× as cured as the
   bottom, while explicit scatter and passages age the whole area alike.

Hand time off is unchanged by all three: the benchmark logs are still
byte-identical at 1000 px and the golden scene is unchanged.

## Maintenance after the thermos review

The thermos review (notes/round6/thermos.md) found four bugs in hand time
and asked for its code to be restructured. Each bug has a test that fails
on the code before the fix:

- **B1: a long pass filled over its own set paint.** A pass of hours has
  its first slices past the gel point before the look-and-fill; baked
  paint has no wet volume, so the look read it as bare. `work` now keeps
  the dry film from before a hand-timed pass that fills, and the look
  counts the paint laid as the wet volume plus the film gained since.
  `handling::tests::a_long_timed_pass_fills_only_its_gaps` (a 7 h pass at
  160 px): 219 fill dabs with hand time off, 540 on before, 155 on after.
- **B2: `ruler()` lost its scatter under hand time.** `Handling::order` is
  now `Option<Order>` (no `order_set` flag), and `ruler()` sets it through
  `order()`. `tally::tests::hand_time_keeps_an_order_asked_for` has a
  ruler case.
- **B4: `varnish()` skipped the hand clock.** The time owed before it went
  on the clock after it had dried the canvas, and its brushing cost
  nothing. It now runs as a `Jump` like `glaze`, and its brushing is
  priced like a glaze over the whole canvas, in the sitting after the
  wait. `time::tests::varnish_is_hand_time_after_the_rest`.
- **B5: every pass started with an empty palette.** Each `work` and
  `stipple` (and a `work`'s cut-in) billed its first dip as a new pile
  (22.5 s), even for a color mixed a moment before. The sitting's palette
  is now passed through (`Canvas::work_with`). I chose that over keeping
  `Piles` on the `Canvas`: a canvas field would have to be checkpointed for
  a resume to stay exact, which means a new checkpoint format, and the
  sitting is the easel's idea. `time::tests::a_sitting_mixes_on_one_palette`:
  one color loaded, passed, stippled and passed again was 4 piles, now 1.
- **B6** is documented, not fixed (see Crops below).

The restructuring: one timed-pass runner (`paint_pass`, with one
`sweep_down` order where there were two), the `time::verb` wrapper, and
smaller items. `touch_secs()` lost its unused width, `stage_at_index` in
`drying.rs` holds the one stage rule, the engine's unit-ambiguous names
now carry their units (`hand_owed_secs`, `clock_hand_min`), and the
checkpoint writes the ledger through `Tally::to_words` in PAINTCK7's byte
order. The `Verb::Jump` rule keeps the studio clock in step with the
canvas, so the flush no longer infers time spent away by subtraction, and
the chunk's "passed while the paint dried" note is gone. `varnish`,
`cracks` and `relief` now report their own wait, as `glaze` did.

Hand time off paints the same bytes: l5_near and l3_green at 1000 px are
identical to a render from before this round when both are built alike
(see below), and the golden scene and replay hashes are unchanged. With
hand time on, output changes where it's meant to: the ruler (B2), the
clock (B4, B5), fill dabs in passes that ran past the gel point (B1) and
the sweep down of `work` (stipple's row-by-row order now serves both;
`work` used `Order::Sweep(π/2)`, whose float bands can mix neighboring
rows when tiles are square). That moved 0.3% of the pixels of a
hand-timed l5_near at 400 px.

**A build caveat for byte checks.** The release profile builds
incrementally (`incremental = true`). The same commit (eff0ef6) built in
two target directories renders l3_green at 1000 px one pixel apart, by
1/255. Compare renders from binaries built the same way, e.g. both with
`CARGO_INCREMENTAL=0`.

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
- **Crops.** A `look --scale` crop prices what it judges from the pixels
  it holds: the look-and-fill dabs, and for aimed marks (`color_over`, a
  stipple's look) the color a dip asks for, so whether it is a reload or a
  new pile, which the sitting's palette carries on. Its clock can differ
  from the whole canvas's by those (a probe in the thermos review: 76.003
  vs 76.013 min for a sky), and its drying by a little.
- **Hand time never rests on its own.** A strict session refuses marks
  until the painter rests; an old log that never rests reports the
  overrun and goes on (see above). A pass started near the end of a
  sitting still runs over by its whole length (not capped).
- **The wet-on-wet look is not mine.** How open paint behaves at a contour
  (the pale drag band) belongs to the r6-wet stream, which also decides
  whether the sketchbook's `dry()` rule goes.

## Next

- Reprice the palette trips once r6-wet settles how much paint a brush
  carries. Then a painting's timesheet is a fair measure of its economy
  (the lab's studies could report it next to Alice's verdict).
- Paint `near` and `green` in sittings once r6-wet lands, with passages
  laid to meet, and compare them with the dried-by-day logs.
- If a slice proves too coarse for skies, slice by stroke count within a
  tile. It is deterministic, but it costs a `wait` per slice.
