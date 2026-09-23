# fresh2_winter: "Winter Evening with a Ruined Choir"

Painter: amnesia round 2 (winter). Program: `paintings/src/bin/fresh2_winter.rs`.
Renders: `out/fresh2_winter.png` (1000px), `out/fresh2_winter_full.png` (3200px).
Scratch: `~/tmp/fresh2-winter-1a51242f/`.

## Composition and why

A small landscape-format canvas (45 × 32.5 cm, the size of the London
*Winter Landscape* [RSC p.42]), aspect 1.385. Motif references below that
carry no source key (Eldena, Oybin, the winter pendants, beret-wearing
figures) are from my general knowledge of his work, not from
`notes/research/`; I used no images.

- **Low horizon, a lot of sky.** Friedrich's winter pictures give most of the
  field to a pale, still sky; the land is a thin band. The horizon here sits at
  about 63% of the height.
- **Dusk after snow.** A cool slate-violet zenith falling through mauve-gray to
  a pale straw band at the horizon (the *Winter Landscape* sky is "pale mauve",
  red iron oxide particles in lead white and smalt [NG p.56]). A thin crescent
  moon, high and off-center, its lit side turned down-left toward the
  afterglow that also lights the snow and the left edges of the oak, the
  fence posts and the figure. A few thin stratus bands low in the glow.
- **Distance.** A long low wooded ridge, blue-gray and dissolved in mist at its
  foot, and against it the ruined choir of a Gothic church, its tall lancet
  open to the sky: the church-in-mist of the winter pictures, a ruin as in the
  Eldena and Oybin motifs.
- **Middle ground.** A snowfield in gentle drifts. A frozen brook winds from the
  foreground back toward the ruin; it gives the eye a road into the picture
  without a literal road.
- **Left.** A stag-headed dead oak on a snow knoll, snow lying on the upper
  side of its limbs, a few crows.
- **Right.** A small group of young spruces on a rise, snow in their tiers
  (spruce as the evergreen hope set against the dead oak, as in the winter
  pendants).
- **Figure.** One small dark figure with a stick and a flat cap, seen from
  behind, walking along the brook toward the ruin; his footprints trail
  back to the lower right edge of the picture, where the viewer stands.
- **Fence.** An old paling fence running back from the right foreground,
  posts leaning, one broken, rails sagging or fallen: depth, and a
  man-made thing gone to ruin like the choir.
- **Foreground.** Snow, with dry grasses and reeds flicked up over the finished
  snow ("fine upturning strokes laid over the finished snow" [NG p.56]) and
  slight impasto in the lit foreground snow [NG p.50].

## Materials chosen

- Palette `friedrich_early` (smalt, not cobalt): the *Winter Landscape* is
  c.1811 and uses smalt [NG p.56].
- Ground: two layers of chalk and lead white with a little umber and ochre,
  the upper lighter and cooler [NG p.55]; the top brushed (striations show
  through thin paint [KÖR p.284]). No red ground: this is the light winter
  ground, not the Berlin red.

## Working method, stage by stage

All geometry (ridge line, snow surface, knolls, brook, ruin, oak skeleton)
is built outside the stages; every stage only paints.

1. **sky**: thin lay-in with `broad()` (level arcs, 0.35 medium, a shade
   duller than the target), badger fused top to bottom; stippled into the
   wet lay-in with a 2.6-unit stippler aimed at the sky's own tones; dried;
   a finer (1.5) lighter stipple that thickens toward the horizon for the
   glow. The gradient is slate-violet → mauve-gray → straw.
2. **moon**: a crescent in three strokes of a small round along the lit arc,
   swelling in the middle, lifted at the horns.
3. **ridge**: the far wooded ridge in short level hatching (`hatch()`),
   bluer at the crest, lighter at the foot; the tree line broken by tiny
   upward stipple drags along the crest.
4. **ruin**: the choir wall and gable as a polygon with a broken top, three
   pointed lancets cut out (the sky shows through), painted in vertical
   `detail()` strokes, a dusky violet-gray a shade darker than the ridge.
5. **mist**: stippled veil (not aimed, density builds it) in fbm banks at the
   ridge's foot, rising into the ruin's lower half.
6. **snow**: the snow surface is a height field (two knolls plus drifts
   stretched 4–5× across the picture); its gradient gives contre-jour light:
   faces that fall toward the viewer are turned from the afterglow and go
   blue-gray, faces turned left catch the warm light. Lean lay-in with
   strokes following the drifts, fused; dried; then a body pass of stiffer
   lead-white paint, loaded more toward the foreground (slight impasto of
   foreground snow [NG p.50]).
7. **brook**: a ribbon whose picture width is foreshortened by the
   direction of each reach (a reach running across is thin, one running into
   depth is not). Ice mirrors the low sky, grayed; a few reaches of open
   black water; a broken blue shadow line under the far bank's snow; snow
   stippled over the ice in places.
8. **oak**: grown with `Habit::dead_oak` (decline 0.6, decay 0.3), then
   **gnarled** by me: one short-period displacement field applied to every
   skeleton point (so twigs stay attached to their limbs) kinks the smooth
   limbs. Painted with my own limb hand (`wood`: brush handed down as the
   limb thins, set down in the last one's wet end). Then snow on the upper
   edge of every limb within ~50° of level (`limb_snow`), runs shorter than
   3 widths dropped (they read as beads).
9. **spruces**: my own gesture spruce: stem, tiers of drooping strokes top
   to bottom (straight-sided cone), hanging needle flicks, then snow laid on
   the upper side of the tiers (lit on the left, blue on the right).
10. **figure**: a man seen from behind walking up the brook, written as
    strokes in a local frame (`Hand`): shadow, legs, coat in four strokes,
    shoulders, arm, head and cap, stick.
11. **crows**: perched on upper limbs (hunched body, head, beak), two
    flying toward the ruin (a thin shallow M).
12. **grasses**: rigger flicks, fine upturning strokes laid last over the
    snow [NG p.56], placed in patches (fbm threshold), along the brook's
    banks and in the near foreground.
13. **fence**: an old paling fence running back from the right foreground,
    posts spaced and sized in perspective, leaning, one broken short, rails
    sagging or gone, one hanging down into the snow; a lit left edge; flat
    snow caps and snow on the rails; snow drifted against the feet.
14. **veil**: Friedrich's advice to Carus, a dark glaze over the whole
    picture except the moon, darker toward the edges [MET PDF p.35]. Very
    thin here (it is advice for moonlight pictures; this is dusk), cool
    gray-brown, heaviest in the lower corners, never zero anywhere.
15. **finish**: varnish, and cracks retuned for this ground (see friction).

Also, between the figure and the fence: **tracks** (the walker's
footprints, paired oval hollows along a path on the brook's right bank,
sized and spaced in perspective, from his feet to the lower right edge);
**stones** (four field stones in the lower left, each an irregular clipped
mass in dark stone colors from a stone family palette, lit on its left
shoulder, a snow cap clipped to its upper part down to a wavy line, snow
stippled against its foot and a stippled shadow toward the viewer); and in
the snow stage, **lee bands** (broken bands of blue shadow below low drift
crests, stippled, thin far off and wide near). The spruces got a stippled
shaded footing.

Last: the sky gradient got a mauve-rose band between the slate zenith and
the straw glow (dusk after sunset in winter), and a **far** stage before the
ridge: a second, paler range behind the wooded ridge, higher on the left,
stippled a little darker and cooler than the sky and fading into the glow at
its foot (a plane of depth the single ridge lacked).

Later additions to earlier stages: thin stratus bands low in the glow
(stippled, no strokes); the moon's glow stippled; the crescent filled with
~260 short arc-wise touches clipped to the crescent mask (sharp horns);
weathering, courses, lit reveals and snow on the ledges of the ruin; a
stippled seam where the far snow meets the ridge's foot; bark streaks and a
cool rim on the oak's bole and big limbs; the oak clipped at a wavy snow
line, with a soft blue cast shadow toward the viewer.

## FRICTION

1. **Craquelure overwhelms the preview.** `Finish::aged` at 1000px drew a
   dark rectangular grid over the whole picture: 3.5 mm islands on a 450 mm
   canvas are ~8 px, and a 70 µm crack is 0.16 px but is drawn at least a
   pixel dark. It dominated the first look at the painting. It is also
   weave-bound (rectangular) because `aged` assumes a 60 µm ground; my
   ground is 140 µm. *Workaround:* my own `Cracks` (ground 140 µm, 45 µm
   width, dirt 0.3). The engine should scale crack contrast with the pixel
   size (area coverage), and `Finish::aged` should take the ground
   thickness from the style.
2. **Geometry between stages invalidates the previous stage's
   checkpoint.** The oak skeleton is grown between the "brook" and "oak"
   stages (as the rules ask: masks and fields outside stages), so any edit
   to the oak's habit hashes into "brook" and `--resume brook` refuses it.
   *Workaround:* `--stale-ok`, which is safe only because I know the brook
   didn't change. A stage would want to declare its own inputs, or the hash
   should cover only the stage bodies.
3. **No quick way to try motif variants.** Growth seeds decide a tree's
   whole character; picking one means rendering several. *Workaround:*
   environment variables read by the painting (`OAK_SEED`, `OAK_DECLINE`
   ...) plus `--resume brook --stop oak --out ...`, and a Pillow contact
   sheet in scratch. A `--set key=value` in `Run` would make this a
   supported loop.
4. **Grown limbs are smooth noodles.** Dead oak limbs from `growth` are
   Catmull–Rom smooth and evenly thick. *Workaround:* `gnarl()`, one
   displacement field (Fbm, 9-unit period, 1.3-unit amplitude) applied to
   every point of every limb, faded to nothing at the foot. Connectivity
   survives because children spring from parent points and all points move
   by the same field.
5. **Root and low-sprout limbs.** Buttress roots and low sprouts come in
   the skeleton with no way to say "the tree stands in snow". A low shoot
   read as a boot. *Workaround:* filter `l.root` and order-1 limbs springing
   in the bottom 12% of the height.
6. **Snow on limbs has to be derived by hand.** There is no "upper side"
   of a limb; I compute the upward normal per point and offset by 0.42 of
   the width, keep runs where the limb is within ~50° of level. Short runs
   gave white beads at every fork until I dropped runs shorter than 3
   widths.
7. **Handling colors are per stroke, so fields with fine structure blur.**
   The snow's light comes from a height field, but a 25–80 unit body stroke
   takes one pile for its whole length, so the drifts' shading comes out as
   soft blotches ("clouds", "fog") unless the field is very smooth.
   *Workaround:* stretched the drifts 4–5× horizontally and lowered their
   amplitude.
8. **Brook width in perspective** is mine to compute: `Shape::ribbon` takes
   a width per point perpendicular to the path. A reach running across the
   picture must be foreshortened, one running into depth not; I compute
   `|dy| + fs·|dx|` per point. Fine, but a ribbon with separate x and y
   half-widths (or a ground-plane ribbon) would be the painter's natural
   tool.
9. **Crows came out as hearts.** A round sable of 0.32× the bird's size
   dragged in two arcs pressed into blobs at 1000px. *Workaround:* a finer
   brush (0.16×) for the wings, pulled out from the body and lifted at the
   tips.
10. **`Mask::roughen`'s `edge` is in mask-value units, and a big one
    leaks over the whole canvas.** I passed `edge = 1.0` thinking "units";
    `smoothstep(0.5 - 1, 0.5 + 1, 0)` is 0.16, so the "open water" mask was
    0.16 everywhere and `work` painted dark water over the entire picture.
    No warning; I found it by stopping after each stage and sampling pixels.
    *Workaround:* `edge 0.08`, then `.mul(&brook_m)` to be sure. The doc
    could say "(mask-value units, ~0.05–0.2)", or `roughen` could keep
    zero at zero.
11. **A glaze with a long soft falloff draws a hard line where its
    thickness underflows to 0.** I glazed a moon glow with
    `0.35·exp(-(d/38)²)`. `Canvas::glaze` settles the film and applies it
    wherever the requested thickness is > 0; the leveled film has a floor,
    so every pixel with a nonzero request (out to ~360 units, where `exp`
    underflows) gets a visible veil (~15 levels) and the ones beyond get
    none. Result: a pale right-angled line across the sky and down the
    snow, the boundary of `exp` underflow in f32 cut into tiles. Took a
    stage-by-stage bisection and a `NO_GLAZE` switch to find. *Workaround:*
    the moon's glow is now stippled (which is Friedrich's way anyway), and
    the final veil glaze has a thickness that is never 0 anywhere. The
    engine should treat thickness below a small epsilon as 0 *before*
    settling, or scale the floor with the requested thickness.
13. **A round brush's end overshoots the limb's base.** The trunk is
    dragged from `sk.base` with a big round, whose footprint reaches half a
    width past the first point, so the tree hung below the snow line in a
    rounded dark stub. Covering it with snow strokes after the fact failed
    twice (sampled colors didn't match the snow around: a blue cushion,
    then two white blobs). *Workaround:* `wood()` takes a clip mask, and
    the oak is painted through a wavy "above the snow line" mask, which is
    how a painter thinks of it: the snow cuts the bole.
14. **`per_column` closures aren't `Copy`,** so a mask closure and a
    `move` color closure can't both use the same profile (`snow_top`) without
    a `let stp = &snow_top;` dance. Minor Rust friction, but it bites in
    every stage that uses a profile twice.
15. **Weathering patches came out reddish.** A detail pass aimed at a
    cool violet-gray over the ruin picked red earth into some piles (the
    full palette trades recipes near the target), giving rusty leopard
    spots at 3200px. *Workaround:* `palette(&pal.only([lead white, bone
    black, pale smalt, raw umber]))`, a stone family, as notes/color.md
    suggests.
16. **A hard mask's edge sits right where the snow is thinnest.** The
    ruin polygon ended exactly on the snow line; the detail strokes, clipped,
    pile a darker rim along the clip edge, and the snow lay-in (soft-edged
    there) left it showing as a dotted line of dark dashes at 3200px. Found
    it with `--full --crop ... --stop ruin|mist|snow` and a contact sheet.
    *Workaround:* the ruin's foot runs below the snow line and its mask
    fades out there, so nothing painted has an edge where the snow is thin.
    Generally: a clipped `work` darkens its own boundary; a painter would
    carry the paint past the line and cut back with the next passage.
17. **`roughen` does nothing to a hard mask.** It moves the 0.5 contour
    by `amount · noise`; a shape mask is 0/1 with a one-pixel ramp, so the
    brook's banks stayed ruler-clean. *Workaround:* `.blur(1.5)` first.
18. **Cast shadows on snow as strokes look like boards.** A filbert
    shadow from the oak's foot was a hard-edged blue slab (and before that a
    round puddle). *Workaround:* a stippled shadow whose coverage falls off
    across and along a line, so it softens by density, like Friedrich's
    mist.
19. **Colors derived from my own color field drift from what is actually
    on the canvas, and stipple/handling color closures can't look.** The
    oak's and the stones' shadows were mixed as "the snow, bluer"
    (`mix(snow_col, blue, 0.4)`), but after the lay-in, body, lee bands and
    seam passes the real snow down there is darker than `snow_col`, so the
    "shadows" came out *lighter* than the snow (pale halos round the stones,
    a pale stripe from the oak). The closures run while the canvas is
    borrowed, so they can't sample it. *Workaround:* sample the canvas beside
    the motif *before* the pass (`c.sample` either side) and mix from that.
    A relative color in `Stipple`/`Handling` ("what is under, 0.05 darker and
    bluer") would be the painter's actual intention; `Canvas::aim` covers
    the paint side of this but not the target.
20. **Clipped edges of thick paint are outlined by the raking light.** The
    moon and the stones' snow caps, painted through a clip mask, get a thin
    dark dotted line along the edge at 3200px: the film steps up at the clip
    and `relief` shades the step. Thinning the paint (more medium, less
    load) cured the moon; it only reduced it on the stones. A real painter
    wouldn't leave a cliff-edge of paint at a stencil line; a soft-edged
    clip (paint thinning over the last half-unit of the mask) would avoid it.
21. **`--stop` after a resumed stage and the stage's own output are hard
    to tell apart.** `--resume sky --stop sky` repainted the sky (8.6 s)
    instead of saying there is nothing to do. Minor.

## Critique

What works:
- **The big design reads as Friedrich:** a low horizon under a wide, still,
  stippled dusk sky; a single dark stag-headed oak against the afterglow; the
  ruined choir small and central in the mist; the evergreens set against
  the dead tree; one small figure seen from behind, walking into the
  picture. At 1000px it is a calm, cold, legible picture with no digital
  banding in the sky.
- **The sky** is the best passage: lay-in, badger, two stipple passes and
  faint stipple stratus give a luminous grain with no stroke direction, and
  the moon's glow is built the same way.
- **The oak** is a real grown tree (bud-based growth) gnarled by my
  displacement field, with snow on the upper side of the level limbs, bark
  streaks and a cool rim; it stands in the snow, cut by it.
- **Particular details at 3200px:** snow on the ruin's broken tops and
  sills, lit window reveals, crows in flight as thin Ms, the figure's cap,
  stick and rim light, footprints trailing to the viewer, a broken rail
  hanging into the snow, snow caps on the posts.

What still reads as digital or weak:
- **The snowfield is too even and too pale in the middle distance.** The
  body pass's soft blotches read more as fog than as a snow surface; the
  lee bands help at 3200px but read as dotted lines at some depths. Real
  snow at dusk would carry more of the sky's color and more drift form.
- **The brook is still somewhat "road".** Its meanders are too regular a
  zig-zag and its width too constant along each reach; the snow crust on
  the ice reads as cotton at 3200px.
- **The spruces are stacks of horizontal dabs.** Better than the first
  polka dots, but they lack the dense, dark, closely hatched mass of
  Friedrich's firs ("short hatched strokes" [NG pp.49–50]); I painted them
  with drooping gesture strokes rather than hatching.
- **The ridge** is a uniform blue-gray band with a mottled stipple texture
  ("granite" at 3200px) and a hard top line. The paler far range behind it
  now gives a second plane, but both top edges are harder than distance
  and mist would allow.
- **The stones** look like little cakes (white top, dark flat underside)
  with a dotted dark rim at the cap edge (friction 20).
- **The figure** is a legible silhouette but crude at 3200px: legs are two
  sticks.
- **Color:** the late mauve-rose band fixed the sky's grayness, but the
  snow still lacks that color: its shadows should carry more violet from
  the sky. The final veil helped the edges; it didn't deepen the color.
- **No underdrawing.** Friedrich's graphite/ink underdrawing, sometimes
  visible through thin paint [CATS p.132; NG pp.49, 58], is absent; with
  more time I'd rule the ruin and draw the oak in a lean dark line first
  and let it shimmer through.
