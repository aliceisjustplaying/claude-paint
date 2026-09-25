# r10_winter_b: Winter evening by a frozen pond

Program: `paintings/src/bin/r10_winter_b.rs`. Renders: `out/r10_winter_b.png`
(1000px), `out/r10_winter_b_full.png` (3200px). Scratch and intermediate
views: `~/tmp/r10-winter-b-28ade229/`.

## Composition and why

A flat snowfield under the afterglow of a winter sunset, the horizon at
about 0.65 of the height, so the sky is most of the picture (Friedrich's
low horizons and big empty skies, *Monk by the Sea*).

- **Sky**: cool smalt-gray at the zenith, through lilac and rose, to a pale
  gold band on the horizon. The glow is left of center, over the set sun's
  place. A few long thin streaks of cloud lie low over the glow. A thin
  crescent moon high right and the evening star low over the glow. The
  moonrise/crescent-and-star motif is his (*Two Men Contemplating the
  Moon*, the crescent pictures). Laid in thin, fused, then stippled twice:
  "skies, mist and distant hills are stippled" [NG p.56].
- **Far distance**: a low blue line of hills with a village and a church
  spire just showing in the haze. The Gothic church seen far off in a winter
  landscape is one of his signature ideas (the 1811 winter pictures). Dark
  on the right horizon is a fir wood, in short vertical hatching.
- **Left**: a stag-headed oak on a low rise, black against the glow,
  with snow lying along the tops of its limbs and two crows in its dead
  top, one more on the wing. Dead oaks in snow (*Oak Tree in the Snow*,
  *Monastery Graveyard in the Snow*) are among his core motifs.
- **Right**: three young spruces heavy with snow. The young firs are the
  hopeful counterpart of the dead oak, the pairing in his 1811 winter
  pair. Their needles are "short, hatched strokes" [NG pp.49–50].
- **Center**: a frozen pond catching the last of the sky, with wind-laid
  snow streaks across it.
- **The figure**: one man in a dark coat and hat, seen from behind (the
  *Rückenfigur*), small, walking the path along the pond toward the far
  church, his tracks curving in from the front.
- **Front**: a stone half buried in the snow. Dry grass and reeds stand up
  through the snow in clumps and along an old field edge under the snow,
  laid last with "fine upturning" strokes over the finished snow [NG p.56].
- **Finish**: a dark glaze growing toward the edges, after his advice to
  Carus to glaze a moonlit picture darker toward the edges, except the
  moon [MET p.35]. Then the aged varnish and craquelure.

Palette: `Palette::friedrich_early_greens()` (lead white, smalt, pale smalt,
ochres, red earth, vermilion, raw umber, bone black, Prussian blue, green
earth), for a c.1811 winter picture before cobalt and chrome yellow.

## Working method, stage by stage

1. `sky`: broad filbert lay-in in level elbow arcs (coverage 4, medium
   0.35), badger fused top to bottom, then two stipple passes (2.6 unit
   stippler everywhere, a finer 1.6 over the glow). Then the cloud streaks:
   a thin body pass through a stretched-fbm mask, fused with the badger.
2. `moon`: the crescent as five curved sable strokes following the limb,
   pressed in the middle and lifted at the horns, clipped to a crescent
   mask; a faint stippled halo; Venus as one touch.
3. `far`: the hills in short level body strokes plus stipple. The village is
   written with a pointed sable (gable, eave line, wall), the church as
   nave strokes, tower strokes and a spire of two strokes lifted to a
   point. The fir wood is a hatch pass through a spiky skyline mask, then
   rigger flicks for the tree tops.
4. `snow`: broad lay-in bowed by a slow direction field in front, fused;
   a stiffer body pass for the drifts' lit faces; the far field stippled
   with level drift; the rise under the oak worked separately; cool
   troughs laid thinly relative to the snow under them (`color_over`).
5. `pond`: ice in level body strokes, mirroring higher and cooler sky
   nearer the viewer, fused; wind-laid snow streaks; a cool band under the
   far bank; the snow's broken lip over the edge.
6. `oak`: grown with `Habit` (my own numbers: 32 years, lighter decline
   and decay than `dead_oak`, so more crown survives; seed chosen from a
   sheet of six). Painted with my own limb routine: each limb one movement,
   a new brush set down inside the wet end of the last run where the
   width falls below 45% or a load runs out. Then lit touches on the glow
   side, dry gray drags for bark, snow on the level upper faces of limbs,
   a drift over the root flare, and the crows.
7. `spruces`: `fir::Fir::grow` into `envelope_for("young")`. I paint the stem,
   then every hatch stroke with a pointed sable in three greens by how lit
   it is, then snow heaped along each bough's upper face.
8. `stone`: a rough lump in filbert strokes around its form, a thick snow
   cap hanging over it, snow banked against its foot.
9. `figure`: the tracks (touches alternating left and right, smaller
   with distance), then the walker as gestures.
10. `grass`: rigger and pointed-sable flicks, pressed at the root and
    lifted off.
11. `glaze`: umber, black and smalt glaze, thicker toward the edges and the
    top.

## FRICTION

(running list; the top five are marked ★)

1. ★ **`--resume X` means "start after X", and the error doesn't say so.**
   I ran `--resume oak --stop oak` to repaint the oak. It was refused as
   "stale" (true, but the real mistake was the stage name). To iterate on
   stage X you resume from the stage *before* X, so you need the stage
   order in your head. A `--redo X` (resume from X's predecessor) would
   match what a painter means.
2. ★ **Masks go silently global.** My stone mask's formula went negative far
   from the stone and made a 240×570-unit wedge "stone", so the handling
   painted snow over half the oak and sky. Nothing warned me that a "small
   motif" mask covered 14% of the canvas. A debug note from `work` when a
   mask's bounding box is far larger than the painter expects is hard to
   specify. Cheaper: `Mask::bounds()` / coverage printed per `work` call
   under a `--verbose` flag. Workaround: early-return 0 outside a box in
   every motif mask.
3. **NaN from `powf` of a slightly negative number** in my own grass
   placement panicked the whole run at the grass stage ("Gesture point 0 is
   not finite"). The message is good and names the point. The friction
   is that it came after 30 s of painting; with `--ckpt` I resumed right
   there, which made it cheap.
4. ★ **Oak habits come out sparse.** `Habit::dead_oak()` over six seeds gave
   thin, few-limbed "antler" trees with little of the dense, crooked,
   twiggy crown of his oaks. Lowering `decline`/`decay` and adding years
   helped, but it takes a sheet of seeds to find one that composes. The
   growth model gives no knob for "crown fullness" or a target silhouette
   (the fir has an envelope; the oak doesn't).
5. **Limb painting boilerplate.** Every painter must write the same thing:
   segmenting a limb by width, choosing a tool per run, solving pressure
   from width, overlapping runs so there is no joint gap. The first
   version (runs meeting end to end) left light bands across the trunk
   where each new brush set down: the soft set-down lifts wet paint, as
   motifs.md warns. `Tool::pressure_for` is exactly the right primitive.
   The run-splitting is the part that is easy to get wrong.
6. **Warm drift of pale cool colors through the aged varnish.** Pale ice and
   far snow asked as cool lilac-gray came out khaki after the varnish and
   the warm ground. I had to ask for much bluer colors than I wanted to
   see. It's physical (a yellowed varnish over a warm ground) but there is
   no quick way to preview the unvarnished look beside the varnished one
   (`--no-varnish` would help, like `--no-cracks`).
7. ★ **Helper functions below `main` stale every checkpoint.** Changing
   `paint_spruce` (only used by the `spruces` stage) staled `snow`, and
   changing `wood_top` (used from `far` on) staled `moon`. The staleness
   model hashes everything after the top-level item for every stage. So
   the natural way to organize a painting (one helper per motif) defeats
   resuming exactly when you are iterating on a motif. I used `--stale-ok`
   each time, having checked by hand that the earlier stages didn't
   change. That is the kind of judgment the tool exists to spare. A
   per-function dependency (which helpers each stage block calls) would fix
   it.
8. **Motif masks cost whole-canvas time even when tiny.** Masks are
   whole-canvas by design, so a 90×40-unit stone mask is a 3200×2240
   evaluation plus a `roughen` distance transform. Tolerable here, but it
   is why every small motif wants a bounding box early-out (which I then
   also needed for correctness, #2).
9. **Regular spacing is the default whenever the painter writes a
   silhouette function.** My first fir-wood skyline hashed a height per
   fixed cell, a perfect sawtooth that I only saw at 3200px. `noise`
   has `uneven` spacing, but a skyline "made of N trees at uneven
   spacing" is a painter-level loop over trees (what I ended up writing)
   and needs `per_column` to stay cheap. That's fine, but easy to miss.
10. **`FirHabit` presets are far apart.** `young()` gave long, drooping,
    palm-like needle strokes at 3200px; `spire()` at the same envelope gave
    skinny sparse trees. I shortened `young`'s hatch strokes toward their
    roots (×0.78) in my own painting code to get something between.
11. **Crescent moon has no primitive.** Fine; I clipped curved strokes to
    a difference-of-disks mask. The edge quality then depends on the clip
    mask's antialiasing, not the brush.
