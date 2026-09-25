# r11 study 1: one trunk against sky and snow

Program: `paintings/src/bin/r11_study1.rs`. Renders: `out/r11_study1.png`
(1000px), `out/r11_study1_full.png` (3200px). Scratch crops: `out/foot.png`,
`out/fork.png`.

## The picture
A small portrait canvas (26 cm wide, aspect 0.8). An old oak trunk rises
out of a drift of snow and crosses a pale evening sky. It forks at about a
quarter of the way down: the leader leaves the top edge, a crooked limb
crosses the sky to the right and there is a broken stub on the left. The sun
is just under the horizon behind the trunk, a little left, so the trunk is
contre-jour. Its shadow comes toward the viewer across the snow. A faint
low line of far bushes breaks the horizon.

## Method (Friedrich's order, from notes/research/friedrich_materials.md)
1. **Ground**: `Style::friedrich()` (a red-brown knifed ground under a
   brushed lighter top), with `width_mm` set to 260 for a small study.
2. **Drawing**: the horizon ruled faint in H, then each limb's two edges in
   HB with hand tremor and the drift's crest faint [CATS pp.128, 131].
3. **Sky**: laid thin with the broad filbert in long, nearly level strokes a
   shade duller than the target, then fused with the badger. Two stipple
   passes follow: one into the wet lay-in aimed at the sky's tones, then a
   finer, lighter one on the dry paint that thickens toward the glow
   [NG p.56; CATS p.127]. Sky family: lead white, pale smalt, cobalt,
   ochre, chrome yellow and vermilion (the 1820 palette).
4. **Far**: a low band of far bushes stippled into the dry sky.
5. **Snow plain**: body paint rich in lead white in low strokes that follow
   the ground. The trunk's cast shadow is painted into it wet (a color
   field, not a later glaze), plus a cleaner pass along the horizon with a
   smaller brush.
6. **Trunk**: over the finished sky, as he did [ALF p.346]. A warm dark
   laid thin with strokes running along each limb (the angle comes from the
   nearest limb's axis) and cut in with a small round.
7. **Bark**: hand gestures along the axis. Dark fissures are drawn with a
   pointed round. Plates are dragged with a near-dry small filbert so they
   break on the weave. Gray lichen and sky light sit on the left flank, with
   a broken warm rim where the glow wraps the edge.
8. **Twigs**: crooked, angular wood, recursive and turning at each "bud",
   with a pointed rigger pressed where it leaves the limb.
9. **Drift**: the snow heaped against the foot, painted *over* the bottom
   of the trunk along the drift's slope. It is shaded blue-gray right
   against the bark (the trunk's own shade).
10. **Light**: a few long thin touches, low in contrast, along the drift's
   crest, and a very thin cobalt/pale-smalt veil in the cast shadow.
11. **Grass**: dry stalks flicked up last over the snow [NG p.56].

No `Finish::aged`: no varnish and no cracks, as the brief asks.

## What I tried and why (in order)
- v1 (1000px): the sky worked straight away. The snow was too white against
  a glow that should outshine it. The drift was a hard pale box: I had
  multiplied its mask by x-smoothsteps, which made square shoulders. A root
  "limb" showed as a black lozenge below the drift, and the lodged snow on
  the bark read as white commas. The cast shadow was a saturated blue slab.
- v2: the tree mask is cut off below the drift crest, the drift mask is
  feathered by a Gaussian around the trunk, the snow is darker and grayer
  (checked by sampling: snow `srgb(172,175,182)`, glow `srgb(231,219,170)`)
  and the shadow is narrower with a wobbling edge.
- 3200px foot crop: the drift's top edge was scalloped like cotton (the
  cut-in tool dabbing along a curved crest). The crest lights were pasted-on
  cream lozenges and the bark was hairlines that read as hair. I removed the
  cut-in on the drift, made the crest lights long thin strokes close to the
  snow's tone and made the bark plates short near-dry filbert drags plus
  fewer dark fissures.
- I tried a hand-drawn "contact" line (a small round along the snow's edge
  in the shade color). It landed as a gray hem *below* the crest and read as
  a seam, so I removed it. The shade is now part of the drift's color field,
  so the drift strokes carry it.
- Fork crop: twigs floated free of their limbs. Their parents had lifted off
  to nothing (ramps release 0.75) before the children branched, so a child
  started from invisible wood. Now a twig ends at 45% pressure when it has
  children, and each child starts at the pressure its parent had at that
  point.
- Final pass, 3200px crop of the mid-trunk: the silhouette stepped in
  rectangles, and the trunk was one flat dark. I printed the mask's right
  edge down the trunk. The steps were in *my mask*: the isotropic bark noise
  moved the edge 5 units within 8 units of height (x 504.75 at y 594 to 509
  at y 602), and vertical strokes hugging it squared the notches off. Now
  the noise is stretched along each limb's axis (sampled in the limb's
  along/across frame), so the edge breaks into long plates. The underpaint
  is modeled across the trunk (lighter warm gray on the left flank lit by
  the sky dome, darkest right of center, a little snow-light low on the
  right) and clipped to the silhouette. The contour is then drawn by hand
  down each edge with a pointed round, following the pencil line. The bark
  plates are longer and fewer. The grass comes in tufts of 2–5 fanning
  blades.

## FRICTION
1. **Masks can't be clipped to "behind this later passage", and a Handling
   has no idea of occlusion.** Friedrich paints the trunk over the sky and
   the snow over the trunk's foot. To stop the trunk mask running below the
   snow, I had to reach into my tree mask and multiply by the drift curve
   (`m * (1 − smoothstep(drift_top+14, drift_top+22, y))`). The first time I
   forgot, and a root showed as a black lozenge below the drift. A painter
   just paints over it; here, what lies under a thin passage keeps showing
   (KM), so every overlap has to be settled in geometry beforehand.
2. **`cut_in` on a curved, bright crest makes scallops.** Cutting in the
   snow's edge against the dark bark gave a row of half-disc dabs: a cotton
   edge at 3200px, invisible at 1000px. Workaround: no cut-in, and a sharp
   mask edge (±1.2 units). There is no tool for "draw a crisp edge along this
   curve with the side of the brush". A hand-made edge stroke along the curve
   landed about 2 units off, as a seam, because a stroke's centerline isn't
   its edge and I can't place a stroke by its edge.
3. **A fixed `Paint` in a gesture (`bark_pal.paint(col, …)`) is judged by
   masstone, but I think in "how it will look here".** Light plates over the
   dark trunk came out as gray blobs (camouflage) on the first try because a
   near-dry filbert over dark paint is hard to predict. `c.aim(&pal, want,
   (x,y), r, medium, coats)` exists, but for hundreds of hand-made strokes I
   had to guess the coats. I tuned by eye in crops.
4. **Hand-made branches need their own pressure bookkeeping.**
   `Gesture::ramps(attack, release)` fades pressure over a share of the
   stroke, but nothing tells me the pressure (so the wood's width) at a point
   along it. Branching children from a parent's lifted-off tip left floating
   twigs. Workaround: I compute the linear pressure at the branch point
   myself and keep a floor. A `Gesture::width_at(s)` (or growth skeleton
   widths for hand-drawn limbs) would have prevented it. The tips of fine
   twigs still break into dashes at 3200px (dry rigger at low pressure). Up
   close it reads as dry brush, but it's more beaded than a real rigger
   flick.
5. **Brush sizes are in canvas units, not mm.** Setting `width_mm: 260` for
   a small study silently makes every style tool physically smaller (the
   22-unit filbert becomes 5.7 mm). The brushed ground's hog is also sized
   in units, so the ground's striations shrink with the canvas. I left it
   because it suits the scale, but a painter picking up "the same brushes"
   on a smaller canvas would expect them to stay the same mm.
6. **The palette changed under me.** `Palette::friedrich_1820()` has no
   "smalt" (it's retained out), so `pal.only(&["smalt", …])` panicked at
   runtime. That's fine, but the panic only comes at run time after 12 s of
   ground. A list of tube names in the error helped.
7. **Stippling a far band with `aim(false)` over a warm sky went violet and
   teal.** The unaimed pile's masstone in thin dots over yellow reads as a
   different hue. Workaround: aim it and use the snow family (with raw
   umber), not the sky family.
8. **Crops of detail stages aren't cheap here.** Each crop re-runs the
   brushed ground and sky for the window (25–40 s). `--resume` needs a
   checkpoint at the same resolution. The ones I made at 1000px don't serve
   `--full --crop`, and any edit to the tree geometry (defined before the
   first stage) invalidates every checkpoint. For a trunk study, where the
   silhouette is what I keep editing, that's all of them.
9. **Noise has no orientation of its own.** Anything grown along a form
   (bark, grain, plates) needs noise sampled in the form's own frame. I
   rotated coordinates into each limb's axis by hand inside the mask
   closure (`Aniso` exists but takes one global angle). Isotropic fbm on a
   silhouette made rectangular notches that I first blamed on the brushes.
10. **Snow color judged in a JPEG vs. numbers.** The peek JPEGs made the snow
   look paler than it is. I had to sample pixels with `magick` to trust the
   tones. A `scripts/peek` option that prints a few sampled values would help.

## Critique (honest)
- **Works**: the sky. A thin lay-in, the badger, then two stipple passes
  give a gradation from cool gray-blue to a warm glow that looks like paint:
  the red ground pricks through the pale blue as a fine warm speckle, the way
  thin paint pools in a textured ground. The composition reads at once: dark
  vertical, pale sky, a low horizon, the shadow coming at you.
- **Half works**: the trunk against the sky. The silhouette is irregular
  enough at 1000px, but up close the edge is blocky in places (noise-driven
  knobs, not bark plates). The bark is too uniform a dark, with texture but
  no real modeling of a round trunk in reflected light. The twigs are sparse
  and a little regular for Friedrich's dense, gnarled oak crowns, and their
  tips bead at 3200px.
- **Weakest**: the trunk entering the snow. The drift is plausible, but the
  cast shadow is made of blotchy, visible dabs that look like brush tests,
  not a shadow lying on a snow surface. The grass is scattered straight pins
  rather than tufts. The contact line itself is clean but plain: no root
  humps breaking the snow and no melt ring.
- **Digital tells**: the trunk is too straight and columnar through the
  middle, and the far band is a thin even strip. The snow plain's strokes are
  uniform in size across the whole depth (no perspective in the handling).
