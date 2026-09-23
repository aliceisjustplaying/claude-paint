# l3_green: loop 3 rework (Friedrich, the wanderer over the Saxon plain)

Starting point: l2_green, critic S5 score 17 (friedrich 5, paint 2, drawing 3, light 4,
detail 3). All three critics named the same worst defect: yellow-green glowing halos around
the trunk and roots, the boulder, the figure and the hedge line. Their second note was the
"loop scribbles" in the foreground grass. This session went to those two surface defects;
the craft notes (loaf boulder, blob figure, stamped trees) are untouched.

## What changed and why

1. **Chunk 24, the halos (critique "worst").** The olive hill glaze cut out
   `FIG:grow(2)`, `STONE:grow(2)` and `OAKALL` (`oak:mask():grow(2) + CROWN:grow(3)`). That
   left a ring 2 to 3 units wide in the old sugary yellow-green around every motif on the
   brow. Now it cuts `HILL - FIG - STONE - OAKWOOD`: exact masks, no ring. (`oak:mask()`
   covers about 3,900 units² more than the painted wood, so I cut the painted `OAKWOOD`
   instead.) The cloud veil
   in the same chunk also cut the grown oak *before* a `blur(8)`, which left a soft
   unveiled rim around the crown. Now it blurs first and subtracts the exact
   `oak:mask()` and `CROWN` afterward.
2. **Chunk 23, the same fault on the plain.** The muted plain veil skipped
   `HILL:grow(1)`, `WOOD:grow(1)`, `FIELDTREES:grow(1.5)`, `VILLAGE:grow(1.5)`, `OAKALL`,
   `FIG:grow(3)` and `STONE:grow(3)`. That kept an unveiled ring along the brow and the hedge
   line and around every field tree. Now every motif is cut with its exact mask
   (`OAKWOOD` for the oak).
   (`OAKALL` stays defined because chunk 26 still uses it for the sky, followed by its own
   ring-closing pass.)
3. **Chunk 28.** The stone's contact crease cut `FIG:grow(1)`. Now it cuts `FIG`.
4. **New chunk 30, the boulder's pale fringe.** At 3200 a pale cloudy fringe hugged the
   stone's top: chunk 14's pale stone strokes and chunk 15's stipple have no `clip=` and
   overshot onto the plain, and later darkening glazes were confined to `STONE`. The fix
   was one detail pass (round 1.6, lengths 2 to 6, coverage 3, medium 0.25) over
   `(STONE:grow(7):soften(2) - STONE)` above the foot, with `clip=-STONE`. Each touch's
   color is `sample()`d 11 units farther out along the ray from the stone's center, so the
   plain and turf close up to the exact edge in their own color. At 3200
   (`look --crop 330,430,500,520 --scale 3.2`) the fringe is gone and no outline appears.

5. **New chunk 31, the trunk's glow at 3200.** Even with exact masks, the full render still
   showed a yellow-green glow about 5 units wide around the trunk and roots. At 1000 it
   was gone. My best guess is that the late `glaze()` films stop short beside the thick
   trunk paint (two body passes). I used the same fix as the stone: a detail pass over
   `(OAKWOOD:grow(7):soften(2) - OAKWOOD)` below y 330, `clip=-OAKWOOD`. Each touch is
   `sample()`d 11 units out along the gradient of `OAKWOOD:distance()`, away from the wood.
   An `easel run --width 3200 --crop 170,400,400,520` render confirms the glow is gone.

Before and after at 1000 (`look --crop 180,400,560,530`): the yellow-green rim around the
trunk and roots and the bright ring around the stone are gone.

## Tried and reverted

- **The loop scribbles come from `glaze()` at 3200, not from brushwork.** I bisected with
  `easel run <truncated log> --width 3200 --crop 380,560,620,640`. Through chunk 23 the turf
  is clean. After chunk 24 (a semi glaze), faint dotted lines appear. After chunk 29 (the
  transparent umber `glaze`, 0.42 coats), they are strong dark dotted closed loops and
  dashes. The glaze film seems to collect along old stroke and film edges. None of this
  shows at 1000 px.
- I rebuilt chunk 29 as a brush veil (`work(land, {hand="glaze", tool="filbert 8",
  medium=0.8, color_over=<linear-light multiply>})`). **It ruined the picture:**
  `color_over` is sampled every 2 units, so the veil repainted the whole land as a blurred
  copy of itself. It wiped out the grass blades, the figure, the field trees and the lower
  crown. I reverted to the original `glaze()`, so the dotted loops remain. This is the
  next thing to fix, possibly with lower-coat glazes, a `semi` pigment or a glaze placed
  before the fine grass.

## Not done

- The boulder is still a loaf with a dark flat end (craft).
- The figure is still a dark hooded blob at 3200.
- The field trees are still dots, and the hedge line on the right still has dash texture.
- At 3200, an embossed patch of dark dashes sits in the oak's cast shadow on the brow
  (chunk 15's filbert shadow strokes, darkened by the later glazes).
- The dotted loops from `glaze()` described above.

## SKETCHBOOK CANDIDATES (add only if the critic's scores improve)

- **Removing halos already in a log:** find every `- X:grow(n)` in veil and glaze masks
  (`rg ':grow\(' paintings/lua/<name>.lua`) and replace it with the exact `- X`. When a mask
  is blurred, subtract the motifs *after* the blur: `(m):blur(8) - motif`, never
  `(m - motif):blur(8)`. Fixed the critics' "worst" in l3_green (chunks 23, 24 and 28).
- **Check halos at 3200, not at 1000.** Exact-mask cuts cleared the halos at 1000, but a
  glow still hugged the thick trunk at 3200. Only `easel run --width 3200 --crop ...`
  showed it.
- **Closing a pale overshoot fringe or glow around a hand-painted motif (stone, trunk):** a detail pass over
  `(M:grow(7):soften(2) - M)`, `clip=-M`, `hug=false`, with the color taken from
  `sample(x + 11*dx/d, y + 11*dy/d, 2.5)` along the ray out from the motif's center (round
  1.6, lengths 2 to 6, coverage 3, medium 0.25). For a long motif like a trunk, take the
  direction from the distance field instead: `D = M:distance()`, then sample at
  `-11 * grad(D) / |grad(D)|`. It paints the surroundings back in their
  own color up to the exact edge, with no outline, unlike a darker ring glaze.
- **Pitfall: `glaze()` pools into dotted loops and dashes at 3200.** Over a turf of many
  strokes, the film collects along old stroke edges and reads as "looping scribbles" and
  "embossed dashes". It's invisible at 1000 px, so check every late glaze with a 3200
  crop, and bisect with `easel run <truncated log> --width 3200 --crop ...`.
- **Pitfall: a brush veil with a `color_over` function over a detailed area** repaints it
  as a 2-unit-sampled blur and erases fine detail (blades, figure, small trees). Use
  `color_over` veils only on smooth passages or with small, fenced masks.
