# Round 8, arm 1: a winter landscape at the easel (no procedural generators)

Session: `paintings/lua/frozen_pond.lua` (run with `EASEL_WITHOUT=procedural`).

## Composition and why

*Winter Evening by a Frozen Pond, with a Wayside Cross*.

- A tall evening sky over a low horizon (a little above the lower third), clear,
  cold blue-gray at the zenith fading through a pale green-lemon band to a warm
  rose-ocher at the horizon; a thin new moon low in the west over the glow, its
  lit limb turned toward the set sun, and a few crows going home over the oak. Friedrich's
  skies are the picture: smalt/cobalt over a warm ground, stippled (materials §6).
- The distance: a low, flat line of hills and a village church spire half lost in
  the evening haze, so that the eye ends at a building of faith (Friedrich's
  habit of the church seen far off: a far goal).
- The middle: a frozen pond, snow lying on it in drifted lines with darker
  polished ice between; the far bank with a low bare wood, a row of four pollard
  willows (unevenly spaced, one small, one leaning) and one bare oak, whose
  crooked limbs and twigs stand against the bright band.
- The front: a snowy bank with dried reeds and grass stalks, "fine upturning
  strokes laid over the finished snow" (NG p.56), a leaning wooden wayside cross
  and one figure in a dark cloak with her back to us (the Rückenfigur), small in
  the landscape.
- Symmetry and stillness, a picture of evening, winter and waiting.

## Method, stage by stage

Every chunk is in `paintings/lua/frozen_pond.lua`; the numbers below are its chunks.

1. **Canvas** (1): the `friedrich` style (after 1820), aspect 1.4, its warm
   red-ocher ground left as the mid-tone.
2. **Underdrawing** (2): the design as globals (horizon, hills, far bank,
   shore, near bank as noise-bent curves), then 2H and HB pencil: the horizon
   and the cross's upright and beam against the ruler, the rest sketched.
3. **Sky** (3, 4): a thin horizontal lay-in with the broad hand (6 stops from
   #667891 at the zenith to #e2c1a0 at the horizon), blended; dried, then a
   whole stippled second layer in the same scale (width 3.2, coverage 2.6) that
   fused the blotchy lay-in into an even gradation. This was the one passage
   that worked the first time.
4. **Distance** (5, 9): the far hills as one thin hazed band (body hand), the
   far wood as a hatched gray-violet mass under a soft outline with lobes.
5. **Ground** (6, 7): the pond in long horizontal strokes, the far bank's snow
   strip over its wet edge, the near bank's snow in contre-jour shadow, lighter
   on its crest, stroked along the slope.
6. **Ice** (8, 20): a few drifted snow streaks; later, over the dry lay-in,
   polished ice glazed darker along a strongly stretched noise, and 150 dry
   strokes of a small round laid flat for drifted snow lines.
7. **The oak** (10, 11): the trunk and 8 scaffold limbs placed by eye as
   ribbons of body paint; then my own branching function: each branch a run of
   straight pieces that turn at nodes (elbows), side shoots at nodes, three
   brushes (round 2.2, riggers 1.2 and 0.6, pointed) chosen by the width, the
   pressure held until near the tip.
8. **Willows** (12): my own pollard: a ribbon trunk, a head of 2 or 3 knuckle
   ellipses, rods as single pointed-rigger strokes from the knuckles, mostly
   upright (normal spread), lengths very uneven, a few thick old rods.
9. **Cross and figure** (13, 14, 23): the cross as two ribbons of dark timber
   (leaning 5 units); the woman as an outline cloak plus two ellipses for hood
   and neck; later modeled with folds, the hood's top and a hair of warm rim
   light on the side toward the glow, the hem buried in snow.
10. **Seating** (15, 16): drifts over the feet (colored from the snow they sit
    in), snow on the cross's top and arms, blue glazed hollows at the feet and
    18 footprints leading up the slope to her, each with a lit back wall.
11. **Particulars** (17, 18, 22): dry grass tufts (up to 60 blades in the near
    corners, sparse ones along the crest), a fringe of reeds on the far shore;
    the new moon as one pointed stroke swelling along the lit limb; the far
    village church and a few roofs as hazed as the hills; crows; a fringe of
    twigs on the far wood's skyline; mending a stray curl of hill paint.
12. **Evening** (19, 21): a cool glaze deepening down the near snow and a dark
    veil toward the edges (Friedrich's advice to Carus, materials §6); one long
    drift trough glazed across the foreground; two half-buried stones.
13. **Finish** (24): varnish, craquelure, relief.

## FRICTION

1. **Pencil is invisible at 1000px on this ground.** The underdrawing on the
   `friedrich` red-ocher ground can't be seen in a 1000px look at all; a crop at
   5x shows it faintly. So the drawing couldn't guide painting by eye; I painted
   from the coordinates in the globals instead. Workaround: none needed, but the
   drawing did nothing for the picture.
2. **A filbert ploughs its paint into rims.** `work{hand="body", tool="filbert 5"}`
   on the far hills laid every stroke as an outline: 399 µm of paint on the
   stroke's edges, 11 µm inside, so the sky showed through each stroke as a net
   of gray rings. Workaround: undo, default body tool.
3. **`edge="lost"` eats small shapes inside the region.** Pond passes with a
   body brush and `edge="lost"` (and `hug=false`) painted straight over the
   figure and the cross even though both were subtracted from the mask (the
   mask was right: 0 at the figure). The overrun scales with the brush width,
   so any hole smaller than about two brush widths gets filled. Workaround: an
   `edge=` function that is 0 (found) within 30 units of the figures, 1 elsewhere;
   that still nibbled the 5-unit upright of the cross, so I gave up on
   stroked passes there and glazed the ice instead.
4. **Scumbling light over darker paint gives opaque blobs.** `hand="scumble"`,
   load 0.35, medium 0.3, a pale color over a glazed shadow laid solid white
   patches, not a broken veil. `color_over={shift=...}` for a darker, bluer
   hollow came out *lighter* and bright blue (flat ovals). Workaround: glazes for
   every darkening; no lightening at all except with small single strokes.
5. **`easel show N` output can't be fed to `easel edit N`.** It prints the
   `--@ chunk` header, which `edit` refuses. Workaround: strip the first line.
6. **Small forms at 1000px are pixel mush.** A crescent of radius 6.5 painted
   as a mask came out a boomerang; crows of 3 units were invisible; footprints
   are a few pixels. Everything particular has to be judged at `--scale 3.2`,
   and the first such look costs a full replay (84 s here). Workaround: the
   crescent as one pointed stroke along the arc (reads at both sizes).
7. **Snow wants to be a clean cut.** Drifts over the foot of the cross in the
   snow's exact color leave a straight horizontal cut where the dark stops
   (correct but reads as pasted). The fix needs a lit lip and a shadow, each of
   which (4 above) was hard; glazed hollows read as plinths at 3200 until blurred
   and thinned.
8. **Body passages over big areas look like blotchy smudges** (the pond
   lay-in, the snow): the broad and body hands at 1000px give rounded
   blob-shaped strokes; there's no hand that lays a long flat, even
   horizontal the way a wide soft brush drags across a panel.
9. **Without `world`/`sky`, there's no scaffold for light**, but that turned
   out fine for this picture: a gradient function and my own palette choices
   did the sky, and contre-jour needs few decisions.
