# easel3_near: a sandstone block at the edge of a spruce wood

Session log (the painting): `paintings/lua/easel3_near.lua`.
Renders: `out/easel3_near.png` (1000px), `out/easel3_near_full.png` (3200px).

## The picture and why

A close view of a bedded sandstone block of the Elbsandstein kind, standing
at the edge of a spruce wood on a still late-autumn morning. Low light from
the left catches the block's lit faces; its right side and the undercut
beneath the middle bed are in shadow. A young birch, still holding a few
yellow leaves, has rooted in a crack on the block's top and grips the edge
with its roots. Moss, dry grass and a little heather grow along the ledges.
At left, the dark spruces of the wood's edge, with young spruces coming up
in front of them. At the foot of the rock: dead rust-colored bracken, a
mossy boulder, sparse grass, small stones, a fallen gray branch and a few
fallen birch leaves. Behind it all, a pale strokeless sky, cool gray-blue
above and warm cream near the horizon, a misty band of far spruce wood and
four crows very small and far off.

What it draws on in Friedrich (from knowledge, no pictures consulted):
- his close nature studies of rocks, trees and plants, and the Elbsandstein
  rock motifs, where a single rock or tree is the whole subject;
- the pairing of old, enduring stone with a young tree growing out of it:
  a sign of life renewing itself on something ancient;
- a dark, particular foreground against a pale, luminous and nearly empty
  sky that carries the mood;
- spruces as dark silhouettes in short hatched strokes (NG pp.49-50 in
  `notes/research/friedrich_materials.md`);
- a sky laid thin and then stippled to hide the brushing (NG p.56);
- grass as fine upturning strokes laid last (NG p.56), and small particulars
  added at the end (the gulls added to *Monk*, CATS p.130), here the crows;
- his advice to Carus to glaze darker toward the edges (MET p.35), used very
  lightly; a thin warm varnish over the whole.
Palette: `friedrich_1820_greens` (lead white, pale smalt, yellow ochre, red
earth, vermilion, raw umber, bone black, cobalt blue, chrome yellow,
Prussian blue, green earth, Rinmann's green). The sky used only lead white,
smalt, cobalt, ochre, umber and red earth.

## How I worked, stage by stage

1. **Canvas.** `friedrich` style, 1820 greens palette, aspect 1.3, seed 23.
2. **Sky.** A noise-warped gradient in broad strokes and a blend. The
   red ground came through in blotches, so after a day I stippled a thin
   veil of the same colors over it. That's what made it read as Friedrich's
   smooth sky rather than scrubbed paint.
3. **Distance and floor.** A band of far spruce spires hatched in hazy
   blue-gray, and the forest floor as warm brown tone. First try: an even
   crest that read as a clipped hedge. Undone; redone with a crest built
   from unevenly spaced triangular spires.
4. **The rock, four tries.** (a) One block from `body.block`: a hay bale.
   (b) Three stacked beds with big rounded edges: pillows. Worse, the paint
   went on semi-transparent, because the floor under it was still open after
   `wait(24*60)` (see friction). (c) `dry()` first, harder beds: a stack of
   bricks. (d) Four blocks (bottom bed, overhanging middle bed, two top blocks
   split by a cleft), each tilted a little, roughened at three scales and
   chamfered by cuts. Painted as a dark-to-light underpainting from
   `f:value`, then local color: sandstone from cool gray to pale ochre, rain
   streaks, algae low down. Strokes run down the planes in shadow and along
   the beds on the lit faces.
5. **Wood edge.** A dark interior mass of spruce spires, then five spruces
   (`tree{habit="spruce"}`): limbs with round brushes and a rigger, needles
   as hatching in drooping ribbons along the branches, clipped behind the
   rock.
6. **Rock surface.** A fine stipple of the modeled stone color to give it a
   grain. Then, after `dry()`, the bed joints, the cleft and the vertical
   cracks with a blunt round brush along noise-wobbled paths, plus light
   lines on the lit lips of the beds and faint laminations.
7. **Birch.** `tree{habit="birch", seed=82}` (it leans toward the light):
   purple-brown twigs first, then the white trunk, a gray shadow side, black
   lenticels, white starts on the main limbs. Leaves: a first try with
   `stipple` over the foliage mask read as pollen. I redid them as a few dabs
   on about a quarter of the foliage clumps.
8. **Floor again.** Humus, moss, straw and leaf-litter patches from two
   stretched noises, darker toward the viewer.
9. **Shadows.** A cast shadow to the right and a contact shadow at the foot
   as transparent glazes, plus a soft glaze under the overhang. It took three
   tries (see friction). The boulder was repainted with a moss crown.
10. **Grass, then bracken.** Grass (`sward`) went through three versions:
    an even lawn over everything, an even olive meadow, then patches from a
    noise mask with each tuft's color taken from `sample()` of the floor under
    it. Bracken: a function that draws an arching stem, pinnae that shorten
    toward the tip and pinnule dabs, over dark rust masses. The first version
    looked like little orange fern icons; the second is twice the size, in
    muted rusts.
11. **Ledges.** Scanned the rock mask for its top line. Moss cushions in some
    places along it, dark water stains below, dry grass and heather standing
    against the sky, birch roots over the edge. The first version made a
    bright green outline around the rock; toned down and made patchy.
12. **Foreground particulars.** Angular stones with their own cast shadows,
    a crooked fallen branch, dry grass in front of the rock's foot to break
    its straight base line, tall seed stalks.
13. **Values.** A value+squint look showed the rock, sky and middle floor all
    at the same value. I glazed the foreground darker toward the bottom and
    sides, and the boulder and the rock's shadow face darker. A try at warm
    light scumbled onto the lit bed tops put opaque cream blobs everywhere;
    undone.
14. **Wood edge, young spruces.** First try: a row of little Christmas trees
    at even spacing. Redone clustered, overlapping, at different depths,
    darker, with grass at their feet.
15. **Weathering.** Dark streaks glazed down from each ledge (stretched
    noise), lichen rosettes clustered by a Worley field, pits in the
    undercut, moss on the boulder's crown. This turned the camouflage mottle
    of the rock into something that reads as weather.
16. **Last things.** Four crows far off. (A try at faint morning cloud wisps
    was invisible; undone.) Fallen birch leaves on the ledges and floor. A
    faint dark glaze toward the edges, `dry()`, a thin warm varnish,
    `relief()`.

## HOW THE EASEL FELT

**What felt like painting.** Looking, then deciding what was wrong, then
changing it: that loop was real, and it was the best part. `look --crop` and
`--mode value,squint` did what stepping back from an easel does. The value
look is what told me the rock didn't read, and the answer (darken
everything around it) was a painter's answer, not a programmer's. Glazing
felt like glazing: a transparent dark over the foreground changed the whole
picture's weight in half a second, and I could see why Friedrich told Carus
to do it. Wet-into-wet surprises (cracks turning into dashes when drawn into
open paint) felt like paint misbehaving in a physical way I could reason
about. The brush running dry across a load made grass and cracks vary on
their own.

**What felt like programming.** Almost everything below the level of
"which color, where". Most of the time I was writing geometry: a
wobbled-polyline helper for cracks, a bracken frond generator, a top-line
scan of the rock mask, masks multiplied and subtracted. The rock was a CSG
problem (block + block - slab, cut, rough) that I solved by parameter
guessing and looking, four times over. That isn't how a painter finds a
rock's shape. A painter draws it; I compiled it. The bracken and the crows
were drawn in code a few points at a time, the way you'd write SVG by hand.

**Slow, confusing, missing.**
- Undo is great for the canvas, but the undone code is gone from the log. After
  the first time, I wrote every chunk to a scratch file and ran
  `do -f`, which made editing and rerunning easy (and I could patch a chunk
  file with a small script and redo it). That's a text-editor workflow more
  than an easel one.
- The drying state is invisible until it bites. I learned the floor was
  still open after a "day" only by calling `drying(x, y)` after the rock
  came out translucent.
- Time is partly hidden: `glaze()` seems to spend painting time (a chunk with
  16 small glazes and one `wait(24*60)` advanced the clock by about 19,000
  minutes), and `dry()` after glazes once jumped 87 days. Physically fair,
  but not what I expected from reading my own chunk.
- Most "first tries" were wrong in the same way: too even, too dense, too
  bright (the hedge, the lawn, the orange ferns, the green moss outline, the
  row of young spruces, the pollen leaves). The defaults lean toward
  covering everything evenly, and every irregularity had to be written by
  hand with noise.

**Compared with writing a program.** Faster feedback than a program
(a chunk and a look in 1-15 seconds), and the canvas has memory and
physics that a program doesn't: the order and timing of what I did shows
in the result. But I never touched the picture. My control was indirect:
every mark went through coordinates I chose by estimating from a JPEG. Where
the easel gave me a solid (form) or a grower (tree, sward), I got
something plausible cheaply. Where it didn't (bracken, crows, stones,
cracks), I was writing a procedural generator on the spot.

## FRICTION

1. **Paint still open after `wait(24*60)`.** The README's recipe waits a day
   between passages, but my thick floor (coverage 4.5, body) was still
   "open" then, and the rock painted over it mixed in wet and went
   semi-transparent (the far wood showed straight through it). Workaround:
   call `dry()` before any passage over earlier work. The floor needed
   about 26 painting days.
2. **Masks for contact and cast shadows came out as hard outlines.**
   `(m:grow(n) - m):soften(k)` gives a ring around the whole shape, which
   painted or glazed reads as a cartoon outline. A `rect()` band softened by
   8 units, glazed, is a hard-edged bar. Workaround: build shadow masks from
   `mask(function)` with `smoothstep` falloffs limited to the base (y below
   the foot line), or `rockm * mask(...)` for the band under the overhang.
3. **`sward` covers everything.** Defaults gave a uniform pale lawn (69k
   tufts) that erased the floor, the shadows and the bracken. There's no
   way to tint tufts from what's under them. Workaround: a patch mask from
   noise as `region`, spacing 2.6, thin 0.5, and each tuft's color set by
   `mix(sample(t.x, t.y, 2), straw, ~0.2)`.
4. **Thin marks and wet-into-wet lines.** Round and rigger brushes are
   pointed, so cracks at pressure 0.7 were about 2 units wide (hairlines at
   1000px). Cracks drawn into open paint came out dashed, like stitching.
   Workaround: `brush{kind="round", width=5, point=0.35}` checked with
   `mark_width`, and only after `dry()`.
5. **Options differ between `work` and `stroke`.** `broken=` is a `work`
   option but an error on `b:stroke` (the error message listing valid options
   helped).
6. **`hand="scumble"` was opaque.** Scumbling warm light onto the lit bed tops
   laid solid cream blobs across every face. I expected broken, semi-covering
   paint. Workaround: none found in time; I left the lit faces alone and
   darkened their surroundings instead.
7. **Spruce tiers can't be lit.** Needle ribbons overlap into one dark
   mass; short light strokes along the upper side of each order-1 limb read
   as scratches all over the tree, not as lit tier tops. Nothing in `limbs`
   says which part of the crown faces up and out. Workaround: I kept the
   spruces as near-silhouettes (true to Friedrich anyway).
8. **CSG joints don't show.** Subtracting thin slabs from the rock body (to
   make vertical joints) made no visible difference in the form's shading.
   Big `round` values on blocks give pillows. Workaround: joints drawn by
   hand as strokes.
9. **Undo loses the chunk's code** (see above). Workaround: chunks in files
   and `do -f`.
10. **Hidden clock time in `glaze()`** and big jumps from `dry()` (see above).
    I couldn't predict the clock from reading my own chunk.
11. **The foliage stipple for sparse leaves read as pollen.** `stipple` with
    low coverage spreads even single dots. Workaround: explicit dabs on a
    random subset of `foliage().clumps`.
12. **Invisible subtle passes.** A stipple of cloud wisps close to the sky
    value didn't show at all. It's hard to judge small value steps from a
    1000px JPEG; I undid it rather than guess.

## Honest critique

What works: the value structure after the glazes (dark framing foreground,
lit rock, luminous sky), the stippled sky, the birch (delicate and
particular, leaning to the light), the ledges with grass against the sky,
the weathering streaks that make the rock read as old stone. At 3200px the
bracken and the grass tufts hold up as particular things.

What doesn't: the rock is still too architectural. Its beds are
rectangular and its base is a straight horizontal line, so from across the
room it can read as a stack of giant masonry blocks rather than a natural
crag. Friedrich's rocks have much more irregular, fissured, particular
silhouettes. The surface under the streaks is still a noise mottle, not
drawn stone. The spruces are flat dark cutouts with pixel-sharp sky holes;
Friedrich's are dark too, but you can read every tier. The middle floor
between the wood and the rock is vague. The foreground is murky rather than
dark-and-particular: the details are there at 3200px but little of it
catches light. The boulder is still an egg. The composition is honest but
static: a big block in the middle, trees at left, empty sky at right.
Friedrich would have drawn the rock first, stone by stone, and it shows
that I modeled it.
