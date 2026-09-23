# easel3_free: Dolmen on the Baltic Shore at Evening

Session: 2026-09-23, 13:13 to about 13:50 BST (wall clock). Canvas `style="friedrich"`,
palette `friedrich_1820`, aspect 1.4, seed 23. 36 chunks in
`paintings/lua/easel3_free.lua`; `easel check` reports "replay matches the live
canvas exactly (36 chunks, 43.2s)". Renders: `out/easel3_free.png` (1000px) and
`out/easel3_free_full.png` (3200px).

## The picture and why

A Hünengrab (a megalithic dolmen: glacial granite boulders with a capstone)
stands on a low barrow at the edge of a dune above the Baltic, a little after
sunset. The sky is a clear afterglow: dark violet blue at the top, a pale
green-gray band, then lemon and a warm orange band on the sea horizon. It is
strongest just left of center, where the sun went down. A thin crescent moon
hangs upper right with its lit limb turned down toward the set sun. The
evening star shows above the glow. Two old oaks stand bare. The big one on the
left branches low and reaches over the capstone; the smaller dead oak on the
right is twisted and broken. A single man in a dark greatcoat and cap, seen
from behind with a walking stick, stands between the right-hand upright and
the dead oak, looking out to sea. Two small sails sit on the horizon. The
foreground is dark dune grass, heather and a faint sandy path, with two
half-buried stones and dry seed heads.

What it draws on (from knowledge only, no pictures looked at):
- Friedrich's dolmen subjects: the Hünengrab paintings of the 1800s and
  around 1820, with oaks around the stones.
- His Rügen and Pomeranian coast evenings and moonrise-over-the-sea pictures:
  a low horizon, a large quiet sky, sails far out.
- The Rückenfigur: a figure seen from behind, dwarfed by the scene, in
  altdeutsch dress.
- Dead or broken oaks as a memento mori beside an ancient grave.
- His method as the research notes give it: a thin sky over a warm ground,
  stippled rather than blended thick (friedrich_materials.md §6, "Stippling");
  trees painted over the finished sky and grass last in fine upturned strokes
  (§9, "Order"); a dark glaze growing darker toward the edges, from his advice
  to Carus (§6, "Glazing"); and cobalt blue and chrome yellow in the palette,
  which fits a post-1820 picture (§4).

## How I worked, stage by stage

1. **World (chunks 2).** A `world{}` with the sun at 4° below the horizon
   and a ground function: a knoll, a bank falling away toward the sea sooner
   on the right, and water at -3.9 m. It took three tries. The first put the
   camera under the knoll because `eye` is an absolute height, not a height
   above the ground. The second left only a 40-unit band of sea. The third
   gave sea from the horizon at 428 down to about 505–565 and land below.
2. **Sky (3–5).** `w:sky` plus a stratus layer read as noon: gray-white
   blotches high up and a muddy orange band. I replaced it with my own
   gradient function, with a Gaussian glow at x=640 and a little stretched
   noise. Broad pass, blend, a second longer blend, then `wait(180)` and a
   stippled veil of the same gradient. The stipple is what made it
   Friedrich-like: it evened out the swirls and the red ground patches.
3. **Sea (6, 13–14).** The mirrored sky (y reflected about the horizon and
   compressed), darkened toward the viewer, clipped to the water. A glaze
   darkened the nearer water, then there are sparse horizontal glints, pale
   where they mirror the glow and dark in the troughs, spaced closer near the
   horizon.
4. **Land (7–8).** A shoreline polyline traced from `v:at`, roughened; a dark
   olive tone; an uneven strip of beach; and a low barrow mound.
5. **Dolmen (9–10).** Rough ellipsoids with `cut` fracture planes in one
   `form{}`, lit contre-jour (`front=-0.35`). It took six tries (see
   Friction). What worked: a solid first pass with strokes **clipped** to the
   silhouette, then a blend, then light stippled onto the sky-facing planes.
6. **Oaks (11–12).** I searched seeds 1–30 for an oak that branches low and
   spreads wide (seed 22). Trunks and big limbs are filled ribbons; the rest
   are tapered strokes by width, then about 2,600 jagged twiglets from limb
   tips. All of it is clipped out of the stones, because the trees stand
   behind them.
7. **Path, heather, grass (15–16, 21–22).** A path ribbon, heather stippled
   in, a sward of marram tufts, then a glaze over the land (darker toward the
   bottom) and dark tufts across the path.
8. **Figure, moon, star, sails (17–19).** The figure is a smoothed coat
   polygon plus head, cap and leg ribbons, with a warm rim on the side toward
   the glow. He is sized to the stones, not to the world (see Friction).
9. **Foam, edge glazes (20).** A broken foam line and a wet-sand line along
   the shore; a transparent dark glaze at the bottom and corners; a faint blue
   glaze at the top.
10. **Foreground particulars (23–24, 28, 31).** Two boulders from a small
    `form`. A near layer of tall dark blades. 26 dry stalks: umbels, knobs
    and grass panicles. Grass over the oak roots and the stone feet. Heather
    sprigs over the stippled heather patches.
11. **Dolmen particulars (25).** A shadow on the downward-facing planes under
    the capstone, fissures and very sparse lichen.
12. **Cloud wisps (26–27).** Three thin streaks stippled low in the glow,
    clipped behind trees and stones, then blended.
13. **Finish (30, 32, 33).** A walking stick. After a 3200 render: I sloped
    the figure's square shoulders by stippling sampled sea color into the
    corners, gave him a collar and one continuous warm edge, sank the dead
    oak's roots in grass and glazed the purple heather back toward the dune
    (chunk 32). Then `dry(); varnish{color="#e6d3a4", coats=0.3, vary=0.1};
    relief(0.14)`.
14. **Sea, second pass (33–36).** The sea was the weakest passage, so I undid
    the varnish and stippled a veil of a smoother sea gradient over it (the
    same fix that worked in the sky), blended it while wet with long
    horizontal strokes, let it set and laid fresh, sparser glints gathered
    toward the glow. Then the final varnish again.

## HOW THE EASEL FELT

**What felt like painting.** The look–judge–undo loop. Each chunk took
seconds, and a look took well under a second, so I could put something down,
see it was wrong and take it back without any cost to my attention. That
rhythm is close to painting and nothing like writing a program. Judging
mattered more than coding: most of my chunks were fine as code and wrong as
pictures (a muddy sky, a mushroom capstone, camouflage-spot sand). The
engine's physics made some decisions for me the way paint does. Thin dark
strokes over a bright sky let the sky show through in stripes. A flat brush
dragged wet tree paint across the sea. Stippling the sky over a dry first
layer is what made it look painted. Those felt like material lessons, not
API lessons.

**What felt like programming.** Almost every mark I wanted went through a
mask algebra expression, and a surprising share of my errors were mask
errors, not painting errors: a mound mask that lost its x-limit, glints not
clipped out of the stones, glints not clipped out of the barrow. Fixing an
early mistake meant closing the session, editing the log by hand with sed
and replaying it (27–42 s each time). That is plainly programming, and it is
the part I would least expect a painter to put up with. I also wrote probe
chunks (seed searches, `v:at` tables, `w:spot` scales) and then had to
remember to undo them so they didn't clutter the log.

**Compared with writing a program.** I didn't design this picture ahead of
time; I discovered it chunk by chunk, and about a third of the chunks I ran
were undone. A program written in one go would have had the mushroom
capstone and the lawn-green foreground, and I wouldn't have known. The easel
made looking cheap, and that did most of the work.

**What was slow or missing.** Crops at 1000px top out at the canvas's own
pixels, so real detail work (the figure is about 45 px tall) needs a 3200
render, which takes about 2¼ minutes for the whole log. I ran those in the
background and kept painting, which worked but meant judging details from
renders that were a few chunks behind. A live 3200 crop (render one window
at 3200 without replaying everything) would have helped the most. There is
also no way to say "paint this behind that": depth order is always masks you
subtract by hand.

## FRICTION (and workarounds)

1. **`wait()` after a `glaze()` jumps the clock by about 29–30 days.**
   Chunk 13 (a glaze) ended at clock 3150. In chunk 15, `print(clock())`
   printed 3150.0, then `wait(120)` took it to 45631.6. In a test,
   `glaze(...); wait(24*60)` went from 45631 to 76541, and `dry()` after the
   final glaze went to 96942 and then 99438. Later plain `wait(1)` and
   `wait(10)` added exactly 1 and 10. Everything was dry by then, so it did
   no harm here, but it would wreck any wet-into-wet plan made after a glaze.
   *Workaround:* none needed this time; I only noticed it from the
   `ok · clock` line. A later case points at a cause: after I undid a
   `dry(); varnish{}` chunk that had ended at clock 99438, the clock went back
   to 45631, but the next `wait(240)` landed on 99678, which is exactly
   99438 + 240. So undo appears to restore the easel's clock but not the
   time the canvas has already seen, and `glaze` may advance canvas time
   the same way. I didn't read the source to confirm.
2. **`work()` strokes overshoot the mask unless you pass `clip=`.** With the
   default `hug=true`, strokes reach past a hard silhouette, so the stones'
   outline looked like stacked plates and the sea's first pass bumped up over
   the horizon. *Workaround:* `clip=` equal to the mask on every pass where
   the edge matters. It would help if the README said plainly that the mask
   picks where strokes start and `clip` is what actually bounds them.
3. **World scale and canvas-unit motifs don't agree, and nothing warns you.**
   I built the dolmen and trees in canvas units. When I asked the world how
   tall a 1.75 m man is on the path, it said 157–181 units, taller than my
   stones. On flat ground a standing figure's head is always at the horizon,
   so world-correct figures near the viewer are huge. *Workaround:* I sized
   the figure to the stones (uprights ≈ human height) and imagined the viewer
   standing higher up.
4. **`eye=` is absolute, not above the ground.** My first ground put the
   camera inside the knoll, and every `v:at` returned the same point at
   Z=0.2 m. *Workaround:* rebuilt the ground so it's about 0 at the camera.
   One line in the README would prevent this.
5. **The form's silhouette is too smooth, and the rough/pitting options go
   from invisible to cork.** `rough(0.1·r, 1.4·r)` gave eggs and lozenges;
   `rough(1.2, 14, s, true)` gave a crumbly, pitted crust. *Workaround:*
   `rough(0.16·r, 0.7·r)` plus a second finer `rough`, several `cut` planes,
   then `silhouette{}:roughen(0.9, 16)`. The first `roughen(1.6, 9)` made the
   outline look like foliage.
6. **Glazes change what "the right color" is for everything after them.** Once
   the land was glazed darker, grass loaded with colors that had been right
   before stood out pale. I first blamed wet glaze, and waiting didn't help.
   *Workaround:* `sample()` the glazed ground and mix blade colors from it
   (chunk 31). This is physics, not a bug, but the easel could offer
   `color_over` for brush strokes or a "sample under me" option on
   `b:reload`.
7. **Fixing an early chunk means close, `sed` the log, reopen (27–42 s
   replay).** Undo only reaches back 8 chunks and only as a stack. I did this
   three times (mound x-limit, glints over stones, glints over barrow).
8. **Probe chunks get logged.** A chunk that only prints (seed search,
   `v:at` table, `w:spot`) is still a chunk in the painting. *Workaround:*
   `easel undo` after each probe. A `easel eval` that runs without logging
   (and rolls back) would be nicer.
9. **`tree{years=180}` gave a 10-limb oak.** *Workaround:* leave `years`
   out.
10. **`tree` root limbs flare as straight spikes** that looked like a
    tripod at 3200. A painted bark flare looked like a bell skirt, so I
    covered the roots with dense tufts.
11. **`w:sun_canvas()` returned (883, 304)** for a sun at azimuth 18° and
    elevation −4°, which puts it above the horizon line on the canvas. I
    didn't investigate and set the glow center by hand (x=640).
12. **A `flat` brush in `hand="broad"` left rectangular brick marks** on the
    sea and dragged wet tree paint sideways (chunk undone). *Workaround:*
    glaze plus glints instead of a second body layer.
13. **`relief(strength)` has no stated default,** and 0.6 turned the
    sky into swirling impasto. The style default is `(0.2, 0.02)`
    (`crates/paint/src/style.rs:128`), and at 3200 even that put a pale
    edge on the stones' silhouettes. *Workaround:* `relief(0.14)`. The README
    could give the default and a sensible range.
14. **`look --crop` can't magnify more than the whole-pixel limit** of 1000px
    for wide crops (a 550-unit crop came back at 1:1). *Workaround:* crops
    500 units wide or less, and 3200 renders via `scripts/peek`.
15. My own mistakes, for the record: I redid the mound without its x-limit,
    so a dark strip ran across the sea. I forgot to clip glints out of stones
    and then out of the barrow. And I ran one scratch edit with bare
    `python3` instead of `uv`.

## Honest critique

What works: the value structure is strong and simple. A light horizon band
sits behind the dark silhouettes of the dolmen and oaks, over a dark
foreground (the value+squint look confirms it). The sky is the best passage:
smooth, stippled, with the warm ground just breathing through it. The oaks
at 3200 have a convincing lace of hooked twigs. The figure is small enough to
be dwarfed but still readable. The palette stays within Friedrich's
post-1820 tubes.

What doesn't:
- **The sea is better but still plain.** The second pass removed the cloudy
  blotches, and it now reads as calm water. But it's a flat band with no
  swell or current lines, and a faint grain from the stipple veil shows
  near the horizon at 3200.
- **The foreground is dark but generic.** The particulars (stalks, umbels,
  sunk stones) are there at 3200 but hard to find at normal size. There's a
  faint band where the near grass layer begins (y≈628). Friedrich would have
  given one or two foreground plants real individuality.
- **The dolmen reads as a silhouette more than as granite.** The fissures
  and lichen are subtle, and the underside shadow is a little blotchy.
- **The path is still a slightly too even ribbon.**
- **The figure is schematic.** After the fix the shoulders slope, but the
  coat is still a simple bell and at 3200 he looks like a cut-out.
- **The heather was purple stains;** it's toned back now but still reads as
  patches more than as plants.
- **The composition is somewhat stagey:** two trees bracketing a central
  object. That's defensible for Friedrich, who liked symmetry, but here it's
  a bit obvious.
- **The mid-ground grass reads a touch too much like a lawn** in the
  1000px view.
