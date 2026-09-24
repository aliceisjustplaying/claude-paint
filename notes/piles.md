# Piles: mix piles, don't paint a formula (Round 7)

Alice on the evening_lake sky: "the lighting is maybe a little too
perfect." The painter wrote the light as smooth math (`evening_lake.lua`
chunk 6: a six-stop `gradient` plus a Gaussian glow), and every stroke got
the formula's exact color at its center: `finish_plan` in
`crates/paint/src/handling.rs` calls `(hd.color)(c.0, c.1)` and then
`pal.aim_for(target, under, ...)` for each stroke, so a sky of 3,000
strokes is 3,000 bespoke recipes on a perfect curve. A painter knifes a
handful of piles, reloads from them and makes the transitions on the
canvas. This stream gives the easel that way of working.

## The API

```lua
SKYP = piles(skyB, {n=7, over=skyS, pal=skypal, medium=0.3, coverage=3.8, load=0.75, seed=601})
print(SKYP)       -- piles(7): "#4b5871: pale smalt 0.19 + cobalt blue 0.51 + raw umber 0.31" ...
work(skyS, {hand="broad", color=SKYP, angle=0, coverage=3.8, medium=0.3, load=0.75, pal=skypal})
blend(skyS, {angle=0, coverage=1.2, length={150, 400}})
```

`piles(field, opts)` returns a pile set; `color=` takes it anywhere a
color goes. In `work` each stroke loads from one pile's recipe; elsewhere
(`stipple`, brushes) it reads as the field stepped to the piles' looks.
Options: `n` (5), `over` (the mask whose colors and underlayer choose and
aim the piles), `colors={...}` (the painter's own piles), `pal`, `medium`,
`aim`/`coats`, `hand`/`tool`/`coverage`/`load` (the pass the piles are
mixed for: they are aimed at the thickness it lays), `mix=false` (only
step the field), `overlap` (0.5), `patch` (45), `vary` (0.08), `batch`
(130), `dirty` (0.12), `seed`. Methods: `P:colors()`, `P:recipes()`,
`P:at(x, y)`, `P:pick(x, y)`, `P:field(x, y)`, `#P`. Full table in the easel
guide (`crates/easel/README.md`, "Piles: mix piles, don't paint a
formula"). Old behavior is the default: nothing changes unless a painter
calls `piles`.

## How it works (`crates/paint/src/piles.rs`)

1. **Which piles.** At points of `over` (every 5 units) the paint a pass
   would mix there: `Palette::aim_for(field(x, y), judge_under(x, y), ...)`,
   the pile that looks right over what is on the canvas. Those paints'
   masstones are clustered (`choose`: colors binned at 0.006 OKLab, each
   distinct color weighted by the square root of its area, so a small glow
   gets its own pile; Lloyd's k-means from lightness quantiles). Each
   cluster is one pile, knifed once to its paint (`Palette::mix`) and
   missing a little (`vary`: a pile mixed by eye). Mixing happens when
   `piles` is called, looking at the canvas then, and each pile is a
   remix in the hand's ledger.
2. **Which pile per stroke** (`PileSet::pick_over`, from `finish_plan`): the
   pile nearest the paint the stroke needs over what is under it, never per
   pixel. The boundary between two piles' zones is displaced by a
   coherent noise (patches of `patch` units) and a little chance per
   stroke, oriented so it moves the boundary instead of swapping piles,
   so steps are ragged and interleaved, not contour lines of the formula.
3. **Batches.** Piles run out and are knifed again: each irregular Voronoi
   cell of about `batch` units gets its own batch, its proportions off by
   `vary` and with up to `dirty` of the neighboring pile off the knife. The
   handling's usual per-dip `mix_jitter` comes on top.
4. **Transitions** are left to the canvas: wet strokes of neighboring piles
   fuse where they meet, and the painter's `blend`.

Deterministic: everything follows from `seed` and position (tested:
replays are bit-exact).

### Why piles are chosen by paint, not by look (a bug caught in the study)

The first version clustered the field's *looks* and aimed each pile over
the median underlayer of its zone. In evening_lake the cloud bank crosses
both the dark upper sky and the warm glow; one look, two underlayers. The
bank's mid pile (lead white, cobalt, red earth 0.34) was right over the
dark and came out as flat violet lozenges over the glow, where the formula
version's per-stroke aim had given gray-brown. A pile is paint, so piles
are now chosen by the paint the strokes need, and each stroke takes the
pile nearest its need: the bank got a browner pile for the glow. Test:
`piles::tests::piles_by_paint_follow_the_underlayer` (one look over a
glazed-dark half and a light half: two piles, one per side; by look it's
one pile for both).

## Evidence

The study repaints the Round 7 painting (log from `r7-paint-wet`, the wet
engine) with five lines changed (`notes/piles/evening_lake_piles.diff`):
the sky (7 piles), the bank (4) and its lights (3), the open water (6,
reused for the water touch-up in chunk 12). Same seeds everywhere else
(each `piles` call takes an explicit seed, so the painting's auto seeds
don't shift). Rendered with `r7-paint-wet` plus this branch's three
commits cherry-picked (they apply cleanly).

- `notes/piles/piles_alice.png` (lossless, blind A/B, key in
  `notes/piles/key.md`): the whole at 1000 and three 1:1 crops at 3200.
  Plain images: `notes/piles/{A,B}_{1000,3200}.png`. The formula version
  is byte-identical to `notes/paint1/evening_lake_{1000,3200}.png`.
- What I see: the piles sky is stepped and patchy (blue-gray piles meet in
  wandering, overlapping passages; the glow is a few warm piles laid into
  each other, not a Gaussian); the formula sky is an even ramp. The bank
  loses the formula's mauve cast and reads as gray paint; its body is one
  flatter shape. The water mirrors the same steps.
- Measured, and not decisive: mean absolute mid-scale lightness (L* of a
  6 px blur minus a 60 px blur, 1000 px) is about the same (upper sky 2.69
  formula vs 2.72 piles, glow 4.36 vs 4.24, water 6.02 vs 7.01); the change
  is mostly in hue and chroma steps, which this measure doesn't see. The
  hand ledger counts about the same trips (sitting 2, chunks 6–9: 9 piles
  and 1,276 reloads vs 8 and 1,300), because its `Piles` already treats
  colors within 0.035 OKLab as one pile; the paint laid is what differs
  (20 recipes and their batches instead of one per stroke).
- Cost at 1000 on the shared machine: `piles` aims the field at a point
  every 5 units of `over` (about 14k for the sky) and each stroke judges
  its underlayer; the water chunk went from 5.7 s to 7.0 s, the whole
  painting from 74 s to 78 s. At 3200 the piles version took 14.7 min.

## Changes to shared files (small)

- `crates/paint/src/handling.rs`: a `piles: Option<&PileSet>` field (None
  by default), a `piles()` builder, and one early branch in `finish_plan`
  (with a palette: pick the pile over the stroke's underlayer, load its
  batch).
- `crates/easel/src/api.rs`: `color_field` reads a pile set as its stepped
  field; `work` keeps a pile set alive and passes it to the handling;
  `color_field`, `style`, `palette_of`, `no_canvas`, `tool_of` made
  `pub(crate)`; one `install` line.
- `crates/paint/src/lib.rs`, `crates/easel/src/main.rs`: one `mod` line each.
- New: `crates/paint/src/piles.rs`, `crates/easel/src/piles.rs`.

Benchmarks (`notes/loops/l5_near.lua`, `l3_green.lua` at 1000): byte-identical
to main before this branch. Golden unchanged.

## Known issues and next steps

- **Stroke ends show more.** A straight broad filbert ends in a square
  edge one brush wide (16 units); with piles, neighboring strokes differ
  more, so an end against a lighter pile reads as a short vertical seam
  (open water, A at 3200 near x 2158, y 1780). The formula version has the
  same ends, fainter. This is the edges stream's territory (a lifted brush
  should feather); a painter can blend more or lower `overlap`.
- **A pile over a whole cloud body is flat.** Four piles for the bank made
  its body one taupe shape. More piles (n 5–6) or a second pass of the
  bank's lights would break it; the painter decides.
- **Crop renders**: piles are mixed looking at the canvas, and a crop
  (`--crop`) holds only its window, so its piles can differ a little from
  the whole canvas's (as aimed strokes already do).
- The study only swaps the colors; a painter working with piles would also
  change the handling (shorter strokes laid into each other at the steps,
  a wet blend per step instead of one blend at the end). Worth a painter's
  round: paint the sky from piles from the start.
- `piles` could offer a "reach": push the lightest and darkest piles out
  to the field's extremes (k-means pulls them in a little).
