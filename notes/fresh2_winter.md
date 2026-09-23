# fresh2_winter: "Winter Evening with a Ruined Choir"

Painter: amnesia round 2 (winter). Program: `paintings/src/bin/fresh2_winter.rs`.
Renders: `out/fresh2_winter.png` (1000px), `out/fresh2_winter_full.png` (3200px).
Scratch: `~/tmp/fresh2-winter-1a51242f/`.

## Composition and why

A small upright-landscape canvas (45 × 32.5 cm, the size of the London
*Winter Landscape* [RSC p.42]), aspect 1.385.

- **Low horizon, a lot of sky.** Friedrich's winter pictures give most of the
  field to a pale, still sky; the land is a thin band. The horizon here sits at
  about 63% of the height.
- **Dusk after snow.** A cool slate-violet zenith falling through mauve-gray to
  a pale straw band at the horizon (the *Winter Landscape* sky is "pale mauve",
  red iron oxide particles in lead white and smalt [NG p.56]). A thin crescent
  moon, high and off-center.
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
- **Figure.** One small dark figure with a stick, seen from behind, walking
  along the brook toward the ruin.
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
13. **finish**: varnish, and cracks retuned for this ground (see friction).

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

## Critique

(at the end)
