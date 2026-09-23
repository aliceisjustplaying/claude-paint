# easel3_green: The Oak on the Common, Summer Morning

Session log: `paintings/lua/easel3_green.lua`. Renders: `out/easel3_green.png`
(1000px) and `out/easel3_green_full.png` (3200px).

## The picture and why

A single old oak in full leaf stands a little left of center on a broad
green common. Its crown rises well above the horizon into a clear summer
sky, and dead snags poke through the top of the leaves. A worn earth track
comes up from the lower right, runs to the oak's foot and passes it. Beyond
is a flat plain of fields: yellow-green meadow, ripening ochre strips and
dark blue-green groves and hedgerows that pale with distance. On the horizon
lies a long, pale range, swelling on the right into one rounded blue dome.
Low cumulus sit over the range, the sky deepens to cobalt at the top and the
morning sun comes from the left, a little behind the viewer.

What it draws on (from knowledge, no pictures consulted):
- **The lone tree as a figure.** Friedrich often set one tree, usually an
  old oak with a broken or dead crown, alone and nearly central against a
  big sky over a wide plain. The dead snags above living leaves are his
  sign of age and endurance.
- **The Bohemian and Riesengebirge distances:** long, low, blue ranges with
  one dome, stacked in thin hazy bands, with the ground at their feet
  dissolving into mist (stippled, per the research note: "skies, mist and
  distant hills are stippled" [NG p.56]).
- **High, deep horizon; a plain seen from a slight rise** (eye 6 m), so the
  fields become a patchwork read in perspective.
- **Color from the research note:** sky from lead white, pale smalt and
  cobalt with ochre and red earth toward the horizon (post-1820 palette);
  greens mixed from the `friedrich_1820_greens` palette, laid in several
  stacked green layers in the tree (MÄD: "up to four layers"); grass laid
  last as fine upturning strokes [NG p.56].
- **A darker foreground bank** under a passing cloud's shade, a common
  Friedrich device for pushing the middle distance into light.

## How I worked, stage by stage

1. `canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4}`.
   A world with the horizon at y=392, eye 6 m, a gentle knoll function.
2. **Sky.** `w:sky` + `w:clouds` (three cumulus, a far bank, a thin
   stratus), painted with `broad` then `blend`. The first attempt put the
   big cumulus off-canvas and left brown flecks: the red ground showed
   through thin places. I undid it, moved the clouds over the range and,
   after a day's drying, laid a second thin sky pass (`aim=1.6`). That
   second layer is what made it a clean, luminous summer sky, which matches
   Friedrich's layered skies.
3. **Distance.** `w:ranges` gave low, lumpy, boxy shapes, so I undid it and
   drew two crests by hand (noise plus Gaussian domes), in pale smalt-grays.
   A coarse stipple of haze read as frost; I undid it and used a graded
   glaze plus a fine, low-contrast stipple at the range feet.
4. **Plain.** One body pass whose color comes from ground coordinates:
   Worley cells in (X, Z) meters give fields that foreshorten correctly,
   and each field kind is hazed toward the sky's airlight.
5. **Groves.** Built as masks (a strip plus rounded crowns per grove) and
   hatched dark and lit. Three tries: physically sized by `w:height` they
   were enormous pillars; hazed too much they vanished into gray; the third
   try is a compromise.
6. **The oak.** `tree{habit="oak", seed=17, years=34}` (tested five seeds
   and three ages for a broad crown). Proxy crown and trunk for the cast
   shadow; the sun raised to 54 degrees and the proxy halved to get a
   believable shadow. Limbs are stroked, then the trunk and main limbs are
   filled as ribbons at the skeleton's widths with a gray lit flank on the
   sun side. Foliage: a mid-green layer, then darks, then lit greens, then a
   second pass over the limbs so they sit behind leaves, then small light
   touches, then dead snags.
7. **Track and foreground.** A ribbon path in canvas coordinates, then a
   graded darker foreground laid in body (a glaze band attempt made a hard
   scalloped stripe and was undone).
8. **Grass.** `sward` below a noisy line, blades darker toward the bottom,
   166 small flowers.

9. **Crest cut-in.** Sky sampled 26 units above the crest and laid back over
   the dome's edge (a slightly noisy crest line), then blended. This turned
   the crenellated mesa into a soft dome. The first version ran the whole
   width and dragged wet oak paint along the horizon (a gray smear through
   the trunk and the left range). I restricted it to x > 545 by editing the
   log and replaying (see friction 10).
10. **Staffage.** A shepherd in a dark coat and hat, back to us, leaning on a
    staff at the edge of the oak's shade, and five grazing sheep on the lit
    grass (first as white cotton blobs; redone warmer and grayer, with dark
    heads and legs).
11. **Foreground particulars.** Lichened granite boulders modeled with
    `form` (ellipsoids, roughened, lit from the left, pitted grain), edges in
    the hollows, lichen specks; after a day's drying so they don't pick up
    wet grass (the first try came out streaked green). Pebbles on the track;
    grass blades growing up over the boulders' feet; a thistle with silvery
    leaves and purple heads, dock leaves at its foot, and white yarrow
    umbels through the near grass.
12. `wait(24*60); varnish{coats=0.3}; relief()`.

## HOW THE EASEL FELT

**Like painting:** the loop of chunk, look, undo. Undo is instant and
cheap, so I tried things I'd never try in a batch program (three sets of
groves, two skies, two sets of sheep). Several of the best moments were
real painting ones: the second thin sky layer over a dry first layer turned
a streaky, flecked sky into a luminous one, exactly as layering does; the
boulders picked up wet grass paint and came out green-streaked, and waiting
a day fixed it. Paint has consequences here, and time is a real tool.
Cutting sky back over a ragged crest is the move a painter makes, and it
worked.

**Like programming:** everything about drawing things. A figure is a poly
of coordinates, a sheep is two ellipses, a thistle is a list of offsets.
There's no way to *draw* a shape by eye; I compute it. The masks, noise
fields and `color=function(x, y)` closures are powerful but they are
geometry and algebra, not looking. Most of my "painting" decisions were
numbers in a function (haze 0.1 + 0.45·far) which I then judged by eye and
revised: closer to tuning shaders than to handling a brush.

**Slow or confusing:** hand-edited logs cost a close, an edit and a
95–104 s replay per round trip, and `close` silently overwrote my first
edit. Physical scale (`w:height`, eye height, `s:size` radii) took three
tries each time. The clock readout (43,000+ painting minutes) became
meaningless. Tiny details (thistle, yarrow, figure) are the hardest: at
1000px a unit is a pixel, and small strokes vanish into same-valued grass
unless you exaggerate size and value.

**Compared with writing a program:** much better feedback, much worse
drawing. A plain program would have given me neither the paint's behavior
(pickup, drying, layering) nor the instant undo; but with a plain program
I'd also have designed shapes with the same coordinate lists. The easel
feels like painting for surfaces, air and light, and like programming for
things.

## FRICTION

(collected as I go)

1. **The clock ran away.** After the grove chunk the clock read 38,749
   minutes (about 27 days) although I had only asked for a day's `wait`.
   It came out the same on each redo. Painting time apparently counts every
   hatch stroke. It did no harm here (everything was dry), but a painter
   planning wet-into-wet would be surprised. Workaround: none needed; noted.
2. **`w:ranges` shapes read as boxy blobs** (little rectangular crenellations
   on every crest). Workaround: hand-written crest functions.
3. **Stroke-end blobs on silhouettes.** Even hand-drawn crests get squared
   bumps where body strokes end at the mask edge (the dome looks like a
   mesa with crenellations). Workaround: planned sky cut-in over the crest.
4. **`w:height` and physical scale.** Correct, but with a 6 m eye height
   the plain is much nearer than it looks, so physical trees are huge.
   I had to size distant groves by eye in canvas units.
5. **`s:size` takes half-sizes (radii)**, which I only found from an
   oversized shadow. The README example doesn't say.
6. **`w:to_ground` returns nil near the bottom edge** (runtime error
   indexing it), so the path had to be built in canvas coordinates.
7. **Coarse stipple reads as frost/snow** on a distant range; the default
   width with `aim=false` is very contrasty. Workaround: glaze + fine
   stipple with `fade=1`.
8. **The oak skeleton is savanna-like:** long bare limbs and a flat,
   clumpy "broccoli" crown, with a thin stroked trunk unless you fill it
   yourself from `l.w`.
9. **`sward` regions start on a visible line** unless the region is noisy
   and roughened; even then it reads as a band.

10. **`easel close` rewrote the log from the session's memory and dropped my
    hand edit** to an earlier chunk, with no warning. Workaround: close
    first, then edit, then `open` (a full replay, about 100 s at 1000px).
    There's no way to fix one early chunk in a live session: undo only
    reaches 8 back, and everything after it has to be replayed.
11. **Cut-in drags wet paint sideways.** A detail pass along a whole
    horizon picked up wet oak paint and smeared it for 200 units. It's
    physically right, but there's no "clean brush every stroke" option in
    `work` that I found. Workaround: restrict the mask to avoid wet
    motifs.
12. **Painting on wet grass lifts it.** Stones painted 4 h after the grass
    came out green-streaked; I had to `wait(24*60)` first. It's correct
    physics, but only easy to plan once you know the clock.
13. **Small particulars are hard to see at 1000px.** A 1 px rigger thistle
    is invisible over 1 px grass of the same value. I had to double every
    dimension and darken the stems.
14. **`pal:only` palettes and cut-ins.** Sampling the canvas (`sample`) for
    a cut-in color and aiming with the sky palette worked, but the laid
    color came out a touch lighter than the surrounding sky (a faint
    band).

## Critique

At 1000px it reads as a bright, clear summer landscape: a lone oak on a
common, a plain of fields and groves, a blue dome on the horizon, a
shepherd and sheep. The sky is the best passage: layered, cool at the top,
warm and pale near the horizon, with soft cumulus over the range. The
distant dome is convincing after the cut-in, and the haze at the range
feet works.

It is not yet a Friedrich. Honestly:
- **The oak is wrong for him.** It's a tall, lanky tree with a flat,
  clumpy "broccoli" crown on long bare limbs, closer to a savanna tree than
  his gnarled, dense, broken-crowned oaks. The dead snags are too thin to
  carry the meaning. The leaf masses have crisp cut-out edges.
- **The composition is too casual.** Friedrich's lone trees stand with a
  near-symmetrical gravity; mine is off-center with nothing answering it,
  and the track is a pale, even ribbon that reads as a path in a video
  game.
- **The oak's shadow** is a flat oval disk that reads almost as a pond.
- **The groves** are rows of identical round bushes, too dark and too
  regular for their distance: the most "digital" passage.
- **The grass** starts on a band and reads as a uniform field of stripes
  at 1000px, rather than the fine upturning strokes over a finished
  ground; the far half of the meadow is smooth lay-in with no incident.
- **Foreground particulars** exist (lichened boulders, thistle, dock,
  yarrow, pebbles) but are few and small for Friedrich's standard. His
  foregrounds reward close looking everywhere; mine reward it in three
  spots.
- **Palette:** the greens are a little too uniformly yellow-green and
  saturated; his summer greens are cooler and more varied, with
  blue-green darks.

With another hour I'd rebuild the oak (thicker, rounder, darker masses
with hooked leaf strokes at the edges instead of clipped masks), vary the
groves, break up the shadow, and fill the foreground with many more
particulars.
