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

(continued below as the work goes on)

## HOW THE EASEL FELT

(written at the end)

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

## Critique

(written at the end)
