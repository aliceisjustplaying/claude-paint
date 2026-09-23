# easel4_green: "Summer Morning above the River Valley"

Session log: `paintings/lua/easel4_green.lua`. Renders: `out/easel4_green.png` (1000 px),
`out/easel4_green_full.png` (3200 px).

## The picture and why

A summer morning, full daylight, seen from a grassy height. On the brow at the left
stands a single broad oak in full leaf; to its right a lone erratic boulder lies in the
grass; between them a small dark figure, seen from behind, stands looking out. Below
the brow the ground drops away over the tops of a wood into a wide, flat, green river
valley: a patchwork of meadows and fields with hedgerows and copses, a river winding
out to a village with a church spire, and low blue ranges under the haze at the
horizon. Above, a tall summer sky with a few cumulus.

What it draws on in Friedrich (from what is known of his motifs and method, not from
any one picture):
- **The height and the jump.** A near foreground zone and a far distance with the
  middle ground hidden (here behind the brow and the dark wood): his habit of cutting
  out the transition so the distance opens like a view.
- **The single oak** as a portrait of a tree (Carus's point about specific tree forms,
  not "foliage"), gnarled, standing against the sky.
- **The erratic boulder** (a glacial "Hünenstein" of the kind in his Rügen and
  Pomeranian studies) and the **figure from behind** (Rückenfigur), small, dark, still.
- **The church spire in the distance**: a small particular that carries meaning.
- **Materials**: the 1820 palette with greens (Prussian blue, green earth, chrome
  yellow, ochres), greens mixed from the sky's blues and the foreground's yellows
  (Field's rule quoted in friedrich_materials.md §9), a pencil underdrawing in two
  passes (2H searching, 2B firm), the sky laid first, trees painted on the painted sky,
  grass last as fine upturning strokes (NG p.56).

## How I worked (stage by stage)

1. `canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}`.
2. A world: horizon y=296, eye 28 m above the valley floor (standing on a height),
   sun from the left behind (azimuth -118, elevation 36).
3. Underdrawing: 2H sketch of horizon, brow, oak, boulder, figure, village, river; a 2B
   restatement of the main contours.
4. Sky from `w:sky` + `w:clouds` (three cumulus, a far bank, thin stratus), broad and
   blended. A day later the lit cumulus tops were scumbled with warm white and the
   bellies glazed a little darker, then blended. (First try: a stipple and a hard-masked
   glaze made speckle and gray slabs; undone.)
5. Three hazed ranges; the valley floor colored per point from world coordinates: a
   rotated Worley grid gives field parcels, each a different green/straw, hazed by
   `w:aerial(Z)`. First try had 140 m parcels (far too big); redone at 55 x 95 m.
6. River as `w:ribbon` in world meters; hedgerows along field edges (a per-pixel mask
   using `w:to_ground`), only beyond 280 m, broken by noise.
7. Village: small hand-built rects and polys scaled with `w:height` (units per meter),
   church with tower and spire, a few trees; hazed.
8. The hill: an open `outline{char="soft"}` along a hand-placed brow, `:below()` as
   mask, painted in a gradient from sunlit yellow-green at the crest to cooler green.
9. Woodland just beyond the brow (tree tops seen from above): a lobed outline with
   irregular heights and gaps, dark hatch, lit clumps. Took three tries (caterpillar;
   too sparse; right).
10. Oak (`tree{habit="oak", seed=3}` chosen from four seeds previewed with `show`):
    thick limbs as `ribbon(l.pts, l.w)` masks in body color with a lit left edge, thin
    limbs as strokes, foliage dark then lit, a day later the trunk restated.
11. Boulder as a `body.ellipsoid` with cuts and roughness, lit by `form{}`; the figure
    as `body_of{}`.
12. Shadows (oak, stone), a darker foreground band, stone cracks and lichen, then a
    `sward` over the hill: 5,282 tufts, 18,783 blades, 258 flowers.

13. Refinement: the village glazed back into the haze (the glaze waited 29 days of
    clock for the paint under it); single field trees sized by distance, each with a
    cast shadow to the right; a lane from the figure's feet down to the river, drawn as
    canvas points turned into world meters (`w:to_ground`) so it narrows properly;
    fine grass flicked by hand over the flat brow around the figure; a stand of pale,
    seeding grass at the lower left; a thistle at the lower right (painted twice: the
    first was a transparent green ghost on green); small stones.
14. Values: a transparent graded glaze darkens the near hill toward the bottom edge
    (the dark foreground band); light scumbled back onto the stone's lit top; one
    very light cloud shadow over the empty right half of the near valley (the first
    try was two dark ponds); varnish and relief.

## How the easel felt

The best of it felt like painting. The order is a painter's order: draw, lay the sky
thin, wait a day, go on. Time mattered. Twice the wet paint underneath decided what
happened: the boulder dragged the still-open wood into its face, and the river
ploughed through the valley to the red ground. The fix both times was a painter's fix:
wait until the passage has set. `glaze` reporting "waited 28.9 days" is a good kind of
honesty. Undo was cheap and I used it a lot (about ten times): paint, look, judge,
undo, change one number, paint again. That loop is quick enough (1–8 s per chunk) to
feel like working rather than compiling.

The worst of it felt like programming with a picture as the debugger. I placed every
mark by coordinates. Some things I could draw by eye: the brow, the wood's crown line
and the lane were rough points I read off a grid, and `outline{}` turned them into a
hand's line, which is the easel's best tool. The oak I did not draw. I picked
it from four seeds previewed with `show`, which is choosing more than drawing, though
the choice was an artistic one. The figure was a `body_of` skeleton I wrote in offsets
and could only judge at 4x zoom. The thistle was pure computation: stems, leaves in a
loop, spines as three rigger flicks each. It came out as a plant, but a diagram of one.
The stone came from a solid with cuts, lit by `form{}`. That is modeling, not drawing,
and it shows: a smooth loaf. So I could draw lines and silhouettes. For plants and
figures I had to compute them and then judge the result.

Compared with writing a program: the engine does the part a program would get wrong
(bristles, pickup, drying, optics), so a naive stroke already looks like paint.
What is left to me is the part a program also gets wrong: regularity. Every
place the picture looks digital (the field trees as equal balls, the grass as a
barcode, the hedgerows as ruled lines, the flat shadow band) is a place where I wrote
a loop or a mask faster than I looked.

The pencil was a disappointment on this ground. 2H on the mid-toned red-brown
Friedrich ground is nearly invisible even at 3x, and a 2B pass is only faint. I drew
mostly with `show()` overlays instead, which are not the drawing.

## Friction

1. **Pencil on the default ground.** 2H is invisible and 2B faint on the mid-toned
   warm ground at 1000 px. The underdrawing couldn't guide me. Workaround: `show()`
   overlays of the same point lists.
2. **Wet-into-wet ploughing.** A flat brush laying the river over wet valley paint
   piled 800–950 µm of paint and showed red ground through it in streaks. The boulder
   pulled the wet wood into its lit face. Workaround: `wait(24*60)` before any passage
   over a fresh one. This is physically right, but nothing warns you. A "this lies on
   open paint" note in the chunk reply would help.
3. **Body passes on narrow masks build impasto.** The oak's trunk ribbon (about 20
   units wide) reached 1.5 mm of open paint after two body passes at coverage 3–3.5.
   Friedrich's films are thin. There is no obvious knob for "thin, one coat" in `work`
   short of guessing `load`/`coverage`.
4. **Hard masks read as digital.** `glaze(mask)` over cloud shade gave flat gray slabs,
   and `stipple` on lit cloud edges gave white speckle. A hand-made cloud-shadow mask
   glazed as two dark ponds. Every mask needed `blur`/`roughen` and lower coats. The
   `work` hand "glaze" with `color_over` behaves better than `glaze()` for this.
5. **Glaze waits for everything under it.** Each `glaze` advanced the clock by 8–29
   days, so the painting's clock is 70 days. That's harmless, but it means a glaze can't be used
   as a quick wet veil. The `work{hand="glaze"}` pass doesn't wait, which I didn't
   know at first.
6. **World ↔ canvas mix-ups.** `w:ribbon` takes `{X, Z}` in meters, and my first lane
   went off to the right as a ruled line near the horizon. Workaround: author in
   canvas points and convert with `w:to_ground`. A `w:ribbon_canvas(pts, width_m)`
   would be the natural tool.
7. **Worley parcels at the wrong scale.** The fields were 140 m cells the first time,
   giant blotches. The scale in meters versus what you see is hard to predict without a
   try. `easel try` with a probe helped.
8. **Transparent mixes vanish on green.** The first thistle (masstone mixes with
   Prussian blue and green earth) was a ghost on the grass, and I had to push values
   to near black. Hiding power isn't visible until you look.
9. **Figures at 1000 px.** A 50-unit figure is about 50 px, and `body_of` offsets are hard to
   judge. I only saw it properly at a 4x crop.
10. **Shell TMPDIR** isn't inherited between tool calls (not an easel problem). I
    wrote chunk files with `-f` from a scratch dir and had to re-export each time.

## Critique

The structure works: the brow, the dark wood and then the opened valley, with the oak
as a strong silhouette against the sky and the spire as a small point the lane and
river lead to. The foreground's dark band and the hazed distance give real depth,
and the sky is a believable summer morning. The oak is the best passage: gnarled,
lit from the left, with sky through the crown.

Weaknesses, harshly: the grass is a uniform barcode of short upright dashes, not
meadow. The field trees are identical balls with identical shadows, scattered like
stamps. The stone is a pale smooth loaf without the granite's weight, cracks or lichen
(they're there but too small to read). The thistle and seeding grass are thin
diagrams. The small round cloudlets in the upper sky are leftovers of the cloud model
I couldn't remove cleanly (painting over them left rings). The village is too neat
a row. The valley floor's field pattern is soft, blotchy greens where Friedrich would
have had crisp, particular strips. The figure is readable but generic.
Where it is most like Friedrich: the stillness, the lone tree, the view.
Where it is least: the foreground particulars, which he would have drawn leaf by
leaf and which I computed.
