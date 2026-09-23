# l5_near: loop 5 rework of the near winter (erratic, young spruce, birch stump)

The critique I answered (three critics, totals 24/20/19). The worst defects: the stamped young
spruce with drooping hooked tiers and no trunk (critic 1); the black wood block with a hard
ruled snow ledge at its foot, "as if standing in water" (critic 3, and "next" for critic 2);
the stump's cartoon sawtooth top and barbed-wire brambles (critic 1). The integrator asked
for the wood and the spruce to be repainted with the new `fir{}` / `fir_wood{}`.

## What I changed and why

1. **The wood is grown by `fir_wood{}` (chunk 7 rewritten).** The old version was a hand-made
   `spruce()` comb over a near-black interior poly. The new skyline `WT` follows the old
   pencil `WOODTOP`, pulled into the canvas at the left. It has a wavy foot line
   `WFOOT` (y 351–359) and `horizon=300` (HZ), `depth=4, count=13, seed=32, sun={-0.9,
   -0.2, 0.37}` (azimuth −112°, elevation 11°). Rows are painted far to near per the README
   recipe, with `air="#a3a59d"` (a shade under this warm low sky) and `mix(dark, air,
   0.62*haze)`. The floor goes between rows 3 and 2 (`#2a302d`→`#5d6266` over y 330–360).
   `woodm = wood:mask()` keeps the later chunks working. The `spruce()` function is still
   defined, but nothing calls it now.
   The result: separate firs in receding rows, with air and sky between them, where there
   used to be one black block.
2. **The foot of the wood (chunk 8 rewritten).** The old bank was a white shelf clipped by
   `rect(-10, 320, 720, 140):soften(12)`, which is the ruled edge. Now the field runs in
   under the trunks. At the foot it's the wood's cool shade (`#8e93a1`; the sun is
   behind-left, so the wood's shadow falls toward us). The shade warms into the sampled
   field at an uneven depth `392 + 26*noise(period 70)`, with a stretched ripple.
   There's no rect. The old white stipple of snow on every bough (`tops`) is gone. In its
   place, the front row's sun-side top strokes carry a little snow:
   `wood:paint(round 1.3, 1, {kind="top", lit={0.62, 1}, every=5, color="#c9c5b8"})`.
3. **Chunks 20–22 dropped** (the hand-drawn back spruces and trunks, the "opened" floor and
   veil, and the speck hatching). They were patches on the black wood and would have painted
   over the new one. The "pale blotch" hatch lines in chunk 16 are removed for the same
   reason.
4. **The young spruce is a `fir{}` (chunk 12 rewritten).** I drew the envelope `YSP` myself:
   lopsided, fuller on the sun side, with a torn notch in the right flank at y 244–262. It's
   `habit="spire", seed=75`, so 459 units tall with 177 boughs (5 dead, 18 broken). I painted
   it with the README order: continuous-angle hatch body, rigger boughs, twigs, the stem only
   under the crown, then the shaded, half-lit and lit shoots. `ysm = yf:mask() * notstone`
   feeds chunks 15 and 18 as before. It now has uneven whorls, a visible trunk at the foot,
   and no hooked chevrons.
5. **The stump (chunk 13).** The break now slants down from one long splinter on the far
   side. The old top was five teeth. The weathered-wood band follows the new top.
6. **The barbed-wire fronds are removed (chunk 16):** the three bracken fronds around the
   stump and the one at (820, 600). The small ones at the boulder's foot stay.

The boulder's lit/shadow seam and white skirt (critics' "next") are **not** addressed; I
ran out of time.

Renders: `out/l5_near.png` (1000) and `out/l5_near_full.png` (3200).

## SKETCHBOOK CANDIDATES (only if the critic's scores improve)

- **Replacing a black hand-made wood with `fir_wood{}` in an existing log** (§5): replace
  the chunk that built it. Reuse the old pencil skyline, clamped into the canvas, and give
  `foot=` a wavy point line (±4 units), not a constant. Pass `horizon=HZ`. Keep the old
  global name (`woodm = wood:mask()`) so later chunks still run. Then drop every chunk that
  patched the old wood's interior (back spruces, veils, speck hatching, "blot" darkening).
  They paint over the new rows. The quickest way to do all of that is `close`, edit the
  file, then `open` (one replay).
- **The snow at a wood's foot without a ledge** (§7, fixes "hard ruled snow band"): don't
  build a bank with a rect. Take the foot outline's `below(H)` times a fade whose depth
  varies with noise (`392 + 26*n(x)`, period 70, ±18 soft). Use the wood's cool cast shade
  (`#8e93a1`) at the foot, mixed into `sample()` of the field below. Under a low sun behind
  the wood, the snow at its foot is in the wood's shadow.
- **Snow on a fir wood with `wood:paint(b, 1, {kind="top", lit={0.62, 1}, every=5})`**
  instead of a mask stipple on every upper edge. That removes the white "screen noise"
  specks.
- *Pitfall:* `easel edit N -f file` rejects a file that still holds its `--@ chunk` header
  (for example, one saved from `easel show N`). Strip that line first.

## Still weak

- The foot line of the wood still reads as a nearly level row of soft lobes at 1000 px
  (y ≈ 345). It should dip and rise more, with trunks standing in it at different depths.
- The boulder: a hard vertical terminator between the tan lit half and the purple shadow
  half, and a white drift skirt that makes it float (chunks 10, 11, 19).
- The young fir came out narrower than its envelope (the spire doesn't fill the flanks). A
  higher `fill=` or `habit="old"` might give the curtain boughs Friedrich draws.
- The stump's lenticels are still an even ladder of dashes, and the square drift patch at
  its foot (chunk 16/17 `sdm`, smoothstep x bounds 156–182) is the old x ≈ 187 seam.
- Sky and snow ripple striations (tool).
