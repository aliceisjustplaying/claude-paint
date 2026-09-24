# Lab pair: rock (lab1)

One granite boulder sitting on a heath in afternoon daylight, its shadow cast to the
right. Canvas 1000 × 667 (3:2), a 300 mm panel, Friedrich ground, seed 71. Shared
setup: `rock_setup.lua` (sky and heath colors, the boulder drawn lopsided as a broken
`outline{}` with one crack, `rock{kind="granite"}`, sun from the upper left and a little
in front). Both logs start with it byte for byte.

| | A (old way) | B (new way) |
|---|---|---|
| log | `rock_A.lua` | `rock_B.lua` |
| 1000 px | `rock_A.jpg` | `rock_B.jpg` |
| 3200 crop (canvas 440–760 × 380–620: the foot, the shadow flank, the cast shadow) | `rock_A_crop.jpg` | `rock_B_crop.jpg` |
| chunks | 7 (setup, sky, stipple, rock, seat, grass, finish) | 5 (setup, sky + heath + dark family, lit face, foot, finish) |
| marks | rocks_in recipe (6 passes + seams + arris lights), 2 glazes, **1,205** rigger blades | 5 passes (sky ×2, heath, shadow side, cast), one lit pass, **306** heath strokes at the foot |
| clock | 33,514 min (`dry()` before the rock, the glazes and the grass) | 31,733 min (nearly all of it the finishing wait; the painting itself took 2.5 days of clock, 3 sessions) |

## What A did
Sketchbook §2 sky (broad + blend, next day a stipple), heath as tone, `dry()`. The
rock painted exactly as `paint_rock` in `paintings/lua/rocks_in.lua` (§6: mass down the
planes, lit planes stiffer, blend the shadow only, seams, arris lights), then
`seat_rock` (cast glaze 0.3 coats, contact glaze 0.35). Following §6 ("a straight base
line makes a rock architecture; sink the foot in grass"), `dry()` again and the §8
rigger meadow blades, 700 along the foot and the rest in clusters in front. My first
blades were a pale, even bristle (the §8 "too even, too bright"), so I redid them darker
and clustered.

## What B did
1. **Value families planned first** (chunk 2). The DARK family is the rock's shadow
   side + its cast shadow as one connected shape across the rock/ground boundary. The
   cast shadow is crisp at the foot and softens as it runs out (a mix of `cast():blur(1.2)`
   and `blur(7)` along x), with none under the lit front, where a dark strip reads as an
   outline. The LIGHT family is the lit face + the sunlit heath.
2. **One session**: the sky (two thin layers wet into wet, one blend), the sunlit heath
   with the rock and its cast shadow reserved, then the dark family **edge to edge**
   with it. On the rock the dark is cool where a plane turns to the sky (#5b5d62), warm
   where it faces the heath (#51483d) and darkest at the turn. Toward the foot it takes
   **the cast shadow's own value** (probe: rock foot L 0.35–0.38, cast at the foot L 0.32),
   so the edge between them is lost. The cast is the heath's own color shifted
   darker and cooler (`shift(groundcol, -0.15 + 0.05*far, -0.004, -0.02)`) and stroked
   the way the heath goes, not glazed.
3. **The lit face into the setting shadow** (chunk 3, 30 h later): filbert 5, stiffer
   paint, down each plane, values held inside the light family (top #ada28c, not chalk),
   `hug=false`. It goes over the crack groove, which the shadow mask had caught as a wide
   dark band, and a single thin crack survives.
4. **The foot seated by the heath itself** (chunk 4, next day, the lit face setting):
   306 short scrubbed strokes in the heath's OWN colors (sampled 8–12 units below the
   base; in the shadow a mix of cast and rock-foot colors) rising over the base in
   clumps. No line along the bottom.

## What I see
- **A** is the known failures, all of them: a chalky lit half, a flat mid-gray shadow
  flank split from it by a seam, a dark contact line along the bottom and then a
  **pale strip of heath between the foot and the cast shadow** (clearest in
  `rock_A_crop.jpg`). The cast shadow is a flat quadrilateral glazed on. The grass
  hides the base but reads as hair laid over a sticker. It looks pasted on.
- **B** sits. The shadow side, the foot and the cast shadow read as one dark shape, so
  the rock weighs on the ground. The upper and lower shadow planes differ (cool above,
  warm below), so the flank isn't flat, and the cast shadow carries the heath's own
  strokes and warmth. The lit face is lighter than the heath but in the same family.
- **Which looks better:** B, clearly. It's grounded, and it reads as paint instead
  of an asset.

## B's ceiling
- **The drawing is still a loaf.** The same outline gives a smooth dome with a
  two-plane shadow side; value work doesn't fix the shape (the setup's fault, shared
  by both).
- **The two shadow planes meet in a horizontal band**, a little too even, like a
  stripe across the flank.
- **The lit face is one flat light** with a soft vertical stroke texture. The planes of
  the lit side barely separate (the tool's lit planes differ only by 0.68–0.81 in value).
- **At 3200 the foot is a row of small vertical dashes** (the scrubbed heath strokes
  read as a barcode close up).
- **The far end of the cast shadow is a soft smudge**, not a penumbra with a drawn edge.
- A terminator blend (to turn the light into the shadow) reopened the crack and exposed
  the ground (see below), so the terminator stays crisp. On angular granite that's
  acceptable, but it's a found edge along its whole length, where a lost stretch would help.
- Relief embosses wormy grooves over both rocks and the heath at 3200 (not a painting
  difference; the relief A/B is a separate experiment).

## Wet-paint misbehavior
The same mechanisms as in the foliage study (`foliage.md`, crops in `wet/`), seen again here:
- **Blender on a wet seam exposes the ground**: `blend()` along the terminator (lit face
  open, shadow setting) smeared the crack into a broad gray band with orange ground
  specks. Undone.
- **Reserved areas painted edge to edge show the ground as warm flecks** in the
  thinner passes (the cast shadow, the rock's shadow side at 1000 px). In the cast
  shadow I kept them as warm ground glowing through a shadow.
- **Rigger blades over the setting lit face** (first try at the foot) laid dark hair
  that read as a beard. Short filbert strokes of the heath's own color worked.
- Timing: the heath (medium 0.18) was setting at 15 h and tacky by 30 h; the shadow family (medium 0.25)
  set at ~30 h and was tacky at ~50 h; the lit face (medium 0.15, laid at 30 h) was
  still open 15 h later and setting at 30 h.

## SKETCHBOOK CANDIDATE
*A rock is seated by its dark family, not by a contact line: its shadow side, its foot
and its cast shadow are one dark shape, painted first and together; the lit ground is
reserved around it.*
```lua
ROCKM, CASTM = rk:mask(), cast (crisp at the foot, blur 1.2 -> 7 along the fall,
               * smoothstep(x_terminator - 140, x_terminator, x): none under the lit front) - ROCKM
SHADEM = rk:shadow(0.3) * ROCKM
-- session 1: sky; heath over land - ROCKM:shrink(1.5) - CASTM:shrink(1.5) (clip=); then the dark family
SHADOWFN(x, y): mix(cool "#5b5d62", warm "#51483d", smoothstep(0.4, 0.75, (n.y + 1)/2))  -- sky vs ground facing
                -> mix toward core "#34322e" by 0.6 * lit:blur(10) -> mix toward the cast's value
                "#34332a" by smoothstep(foot - 70, foot - 6, y)
CASTFN(x, y):   shift(groundcol(x, y), -0.15 + 0.05*far, -0.004, -0.02 + 0.008*far)
work(SHADEM, {tool="filbert 5", angle=rk:field("plane"), coverage=3.8, load=0.85, medium=0.25, clip=ROCKM})
work(CASTM,  {tool="filbert 5", angle=0.04 (the heath's way), coverage=4.2, load=0.9, medium=0.25, hug=false})
-- session 2 (~30 h, the shadow SETTING): the lit face, filbert 5, medium 0.15, hug=false, top value ~#ada28c
-- session 3 (next day, lit face setting): ~300 filbert-3 strokes in the heath's own color
--   (sampled 8-12 below the base), rising 4-24 units over the base in noise clumps (period ~34)
-- never: a contact glaze ring, a dark line along the foot, rigger hair at the base
```
Ceiling: a loaf outline stays a loaf; the lit face needs its planes separated; the
foot strokes read as dashes at 3200.
