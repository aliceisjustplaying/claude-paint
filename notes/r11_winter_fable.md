# r11_winter_fable: Winter dusk on the marsh

Program: `paintings/src/bin/r11_winter_fable.rs`. Renders:
`out/r11_winter_fable.png` (1000px), `out/r11_winter_fable_full.png` (3200px).
Scratch: `~/tmp/r11-winter-fable-92d3ae5a/`.

## The composition and why

A frozen marsh on the flat land north of Dresden, half an hour after
sunset in winter. The sky takes 62% of the picture (horizon at y 440 of
714, aspect 1.4, near the 32.5 × 45 cm of the London *Winter Landscape*
[RSC p.42 in notes/research/friedrich_materials.md]). The afterglow lies
low and left of center; the sky cools upward through a pale gray to a
smalt-gray zenith and goes colder on the right, away from the sun. A low
wooded ridge lies along the horizon in mist; a village church spire stands
just right of center, half lost in the mist. In the middle ground a frozen
pool mirrors the sky dully. A dead, stag-headed oak leans in from the
left with snow lying on its near-level limbs; two spruces and a smaller
third stand on the right. One wanderer, from behind, has stopped on the
snow before the pool and looks toward the spire; his tracks come up from
the bottom edge. Three crows over the oak, one hunched on a dead limb; a
thin waxing crescent low in the afterglow, right of the glow.

What it draws on (from knowledge, not pictures):
- Friedrich's winter subjects: bare and dead oaks in snow, firs, a
  distant church in mist, low horizons over a plain, a single small figure
  seen from behind, crows, the crescent moon with a low sun. None of these
  is copied from one picture; the arrangement is mine.
- His materials and method (`notes/research/friedrich_materials.md`):
  ready-primed Dresden linen with a warm ground (`Style::friedrich()`),
  a graphite underdrawing with ruled straights (§3: horizon and spire
  ruled with a 2H, the pool and trunk freehand HB), one thin
  underpainting and one or two thin layers, skies and distant hills
  stippled (§6, NG p.56), firs in short hatched strokes (NG pp.49–50),
  grass as fine upturning strokes laid last over the finished snow (NG
  p.56), slight impasto only in foreground snow (NG p.50), lead white
  snow, a smalt/cobalt sky, few pigments (the 1820 palette).
- The one sun: the engine's `World`/`Sky` with the sun 3.6° below the
  horizon at azimuth −24° gives the glow its place and the cooling to the
  right. I use the physics for the sky's *structure* and pull the tints
  toward what I want (straw-pink glow, gray-violet middle, cold zenith).

## Working method, stage by stage

1. **drawing**: graphite on the ground. Horizon ruled (2H, light), spire
   ruled (four short lines), pool edge and trunk freehand (HB). The
   horizon line and spire lines sit under thin sky paint at the horizon.
2. **sky**: lay-in with the broad filbert in long elbow arcs, slightly
   fanning (the angle tilts ±0.04 rad across the width), coverage 3.6,
   medium 0.38; badger fused top to bottom; dried; stipple 2.6 at
   coverage 1.7 aimed at the same field; dried; a finer, lighter stipple
   1.7 whose coverage thickens toward the glow and thins to 0.3 overhead
   (`fade` keeps the thin part from reading as salt); the moon as one
   arc of the round sable's point with a pressure swell in the middle.
3. **far**: the wooded ridge, a `per_column` crest of fbm with two long
   swells (higher hills left of center and right), mask roughened 3
   units; laid thin in short level body strokes then stippled 1.7; color
   only 0.22–0.58 of the way from the airlight to a blue-gray, darker in
   tree clumps (a second fbm). The spire in thin gray with a round sable:
   nave, tower in three vertical strokes, two lifted strokes to the point.
   Then a mist veil stippled `aim(false)` over the ridge foot and the far
   snow, its coverage wandering with fbm.
4. **snow**: a height field of hummocks (fbm, foreshortened toward the
   horizon) gives a slope; slopes facing the glow (left and front) are
   warm lead white, the rest cold; a shadow family on the far side. First
   a thin opaque underpainting by masstone (kills the warm ground), then
   the modeled pass with strokes following the hummock contours and a
   little more load toward the bottom (impasto), a shorter body pass
   where the hummocks turn, the far snow fused and then stippled into the
   mist. The snow uses a palette family without red earth or vermilion.
5. **pool**: an oval mask with an fbm-ragged edge; the ice color is the
   sky mirrored (sampled above the horizon, squashed) grayed and
   darkened, paler toward the rim where snow dusts it; long level strokes
   fused; a rim of short level detail touches half snow, half ice.
6. **oak**: `Habit::dead_oak` grown with lean 0.09, decline 0.75, decay
   0.5, breakage 0.3, 330 units tall from (207, 648). My own limb
   painter: each limb sectioned where it thins past 42% of its width,
   round sable above 1.7 units and rigger below, pressure solved from
   the width with `Tool::pressure_for`, live tips lifted to nothing,
   broken ends stopped short and split with three rigger flicks. Dry.
   Lit streaks down the glow side of limbs wider than 2.2. Dry. Snow:
   on every segment that lies nearer level than vertical, a run of
   touches of lead white on its upper side, more likely and heavier the
   more level and thicker the wood.
7. **spruces**: my own: a stem lifted to a point, whorls from the top
   down with one or two limbs per side of uneven reach and droop, and
   short hatched needle flicks hanging down and outward from each limb;
   snow touches on the upper side of the lower whorls; a paler third
   spruce farther back mixed toward the airlight.
8. **grass**: dead grass and reeds, rigger and a small sable, flicked up
   from the root and lifted off, clumps around the pool, at the oak's
   foot and scattered; colors from straw to dark umber, pulled toward the
   airlight with distance; a bent reed head on some.
9. **figure**: the wanderer from behind, 54 units: five downward coat
   strokes side by side, the hem, two legs, neck, hat crown and brim,
   a rigger stick, a rim of the glow's light down the left edge. Tracks:
   pairs of touches of the snow's own color shifted darker and bluer,
   spacing shrinking toward him.
10. **birds**: three crows as two wing strokes lifted off from a body
    touch; one hunched on a dead limb tip.
11. **finish**: `Finish::aged` (yellowed varnish, craquelure, relief).

## FRICTION (running list)

1. **Two `Mark` types.** `paint::Mark` (re-exported at the crate root) is
   `hand::Mark { pts: &[..], pressure: (f32, f32), ramps }`, but
   `Canvas::draw` takes `graphite::Mark { pts: Vec, pressure: Vec<f32> }`,
   which is not re-exported. The compiler error ("expected `&Mark`, found
   `&Mark<'_>`") is baffling until you read graphite.rs. Workaround: a
   `pencil(pts, smooth, p0, p1)` helper that calls `graphite::resample`
   and builds the per-point pressure vector.
2. **`Palette::only` panics on a missing tube name at run time**, after
   the sky stage (25 s in), not at build time: `no tube "smalt" in
   palette Friedrich, after 1820`. The 1820 palette drops smalt but keeps
   "pale smalt". A `try_only` or a compile-time list would save a
   render. Workaround: read palette.rs.
3. **Aimed snow over the warm ground came out with pink piles.** With the
   full palette, `Aim::Laid` strokes of near-white over the reddish
   ground gave patches of pinkish lead white + red earth (the aim's
   family term isn't enough when the target is nearly neutral and the
   under is orange). Workaround: a snow family (`only`) without red earth
   and vermilion, plus a thin opaque underpainting by masstone first.
4. **`World::height` is physically right and compositionally useless
   here.** With the eye at 1.7 m, a 1.72 m figure anywhere on the plain
   has his head on the horizon (the docs say so). Friedrich's figures are
   small in their plains because he raised the viewpoint or put them
   far; the camera has no way to say "the eye is 1.7 m but the figure is
   on a plain 200 m off while the pool is 30 m off" without breaking the
   ground plane. Workaround: hand-set height (54 units) and ignore the
   world's scale for the figure.
5. **Snow on limbs needs the limb's upper side, which the skeleton
   doesn't give.** `Limb` has `pts` and `w` but no per-segment normal or
   "up" flag; I compute the normal from consecutive points and pick the
   side with negative canvas y. Works, but a `Limb::normal(i)` or an
   "up-facing" mask per limb would be the natural primitive for snow,
   moss and lit sides.
6. **Stipple mist with `aim(false)` at coverage 1.4 was invisible** over
   a ridge only a little darker than the mist color. It needed coverage
   1.6 and a denser mask band before it read at all, and it still reads
   more as a paler stripe than a veil. The mist wants a thickness
   (film µm), not just a density.
7. **Stage timings hide where the work is.** The "drawing" stage reports
   9–10 s, but that is the ground preparation (linen + three priming
   layers) charged to the first stage. Misleading when choosing what to
   optimize; a "prepare" line would help.
8. **No `Stipple::mark_mm`**: stippler widths are in canvas units, so the
   same mark size means different mm on a 44 cm and a 171 cm canvas
   (already noted in notes/stipple.md; still true).
9. **The crescent moon**: no engine notion of a moon; a thin crescent is
   one arc stroke with a pressure swell, which works but its hiding
   (0.98) had to be forced because the aimed lead white over the glow
   would have vanished.
10. **`Gesture` has no width field**: a stroke's width is the tool's width
    × pressure. To draw a limb of a known width you must solve the
    pressure (`Tool::pressure_for`), and the round sable's mark width at
    pressure 1 is not the tool width (spread ≈ 1.0–1.3). Every motif
    painter re-derives this.

## Critique (honest, updated as I look)

At 1000px, after the third preview:
- The sky reads as thin stippled paint with a real glow and a cold
  zenith; the transition is a little too even and the zenith is a
  little heavy. There is no cloud; Friedrich's winter skies are often
  clear, so I let it stand.
- The horizon ridge still reads as a strip rather than a wooded hill line;
  its variation is fbm, not hills. The mist barely shows.
- The snow plain: the hummock modeling is faint at 1000px, and the field
  reads as an even warm gray. It needs more value range and a few
  clearer drift shadows.
- The oak reads as a dead oak with snow on its limbs, and the lit streaks
  give it some roundness; the limbs are still smooth ("noodles").
- The spruces read as spruces with snow. The far one is too similar in
  drawing to the near ones.
- The figure is small and dark and reads as a man in a coat and hat, from
  behind. His tracks barely show.
- The grass is too regular in its scatter.
- The craquelure at 1000px is strong; at 3200px it should be finer.
