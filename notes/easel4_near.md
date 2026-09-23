# easel4_near: an erratic boulder at the edge of a spruce wood, first snow

Session log: `paintings/lua/easel4_near.lua` · renders: `out/easel4_near.png` (1000px),
`out/easel4_near_full.png` (3200px). Painted live at the easel, 2026-09-23, about 2 hours.

## The picture and why

A near view, about six meters from a granite erratic, the kind of boulder the ice left
across Pomerania and Rügen. Friedrich drew these stones again and again and put them in
pictures (the *Hünengrab* paintings, the stones on the Baltic shore). It is late afternoon
in early winter. The first thin snow lies in patches on the stone's top and over the
ground. A low sun from the left, a little behind the painter (azimuth −112°, elevation
11°), warms the stone's left face and throws long blue shadows to the right. To the left
a spruce wood's edge steps down toward a clearing: dark spires cut against a pale,
veiled sky. That is the wood-edge silhouette of *Early Snow*, *The Chasseur in the
Forest* and *Spruce Forest in the Snow*. A young spruce, dense to the ground, stands
alone just right of the stone, dark against the light. A snapped birch stump in the near
left answers it, and dry grass pokes through the snow in sparse tufts. The motifs are
his (the stone, the young fir, the broken stump, the first snow, the wood's edge, the
low light), and so is the moral undertone: the old stone and the dead birch beside the
living evergreen. He painted winter pictures of that kind (*Winter Landscape*, 1811).

Materials, from notes/research/friedrich_materials.md: the `friedrich` style (warm,
multi-layer ground), the post-1820 palette with greens (Prussian blue, green earth,
Rinmann's green). The sky was restricted to lead white, pale smalt, cobalt, ochre, red
earth, vermilion and umber. A pencil underdrawing went down in two passes, a faint 2H
and then a firmer 3B, as he did (CATS pp.128, 131). The sky is thin and fused. The
spruces are built from short hatched strokes (NG pp.49–50), and the grass went on last
in fine upturning strokes over the finished snow (NG p.56).

## How I worked, stage by stage

1. `canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.3, seed=23}`.
2. World: horizon 300, eye 1.6 m, fov 52, the sun low on the left, gently undulating
   ground. The boulder is an ellipsoid turned, roughened and cut by two fracture planes,
   with a small stone at its right foot. I previewed it with `try` + `show()`. The first
   try was twice the size I meant because `size` takes radii, not full sizes.
3. Drawing: 2H `sketch` of everything (invisible at 1000px on the mid-tone ground:
   `drawing_mask():area()` said the graphite was there), then a 3B `line` pass with
   `hatch` on the shadow flank, then `fix()`. The points were read off `look --grid`.
4. Sky: `w:sky{overcast=0.35}` + a thin stratus veil and a far bank, `broad`, then `blend`.
5. Far wood line (soft open `outline`) and the snow as tone from `v:at` light. The first
   pass came out lilac-gray everywhere because lit flat snow at an 11° sun is only
   0.37–0.49 in `shade.value`. I probed those values and remapped.
6. The wood edge: I wrote a `spruce{}` drawing function (axis plus whorls of drooping,
   tip-lifted branches as ribbons, some foreshortened toward me), placed 17 spires with
   `uneven()` so their tips sit on my pencil line WOODTOP, and filled them and a lower
   interior mass with dark hatching. Then I drew the branches with a rigger. Three
   iterations: blobs → thinner tiers → spires.
7. Snow banked at the wood's foot (after `dry()`). The first try smeared into wet
   black; the second try of snow on the boughs made a chevron "rain" pattern. The
   bough snow that stayed is a stipple on the upper faces only, and it barely reads.
8. The boulder: body color from `v.form` light (a five-stop granite gradient, OKLab
   mottling), strokes along `fall`. Then a scumble pulling the shadow flank together,
   concave `edges` as fissures, my drawn fracture lines restated with a round brush and
   lichen crusts from a `worley` stipple. Then a snow cap wherever the normal faces up
   (`f:mask` on `s.n[2]`), broken by noise.
9. Snow drifted against the foot: three tries (a rect edge showed; a cream band that
   didn't match; finally a narrow ribbon along a soft outline, sampled from the snow
   below).
10. The young spruce with the same `spruce{}` hand, kept behind the stone by clipping
    with `-v:visible("bodies")`, plus lit needle tips on the left.
11. The birch stump: a `broken` outline, white bark shaded left to right, raw wood at
    the break, black lenticels across it and a contour drawn with the outline's own
    strokes.
12. Shadows: proxies for the spruce and stump in a second world/view, `cast_shadow`
    and `contact_shadow` glazes. Too heavy at first (coats 0.5), then 0.32.
13. Dry grass: `sward`, then thinned hard by a noise pick so it stands in single tufts.

14. Refinement: darkened some pale blotches low in the wood, banked snow round the
    stump's foot (the rect again gave ruled sides the first time), drew bracken fronds
    (rigger rachis plus round-brush pinnae from a small function) and fallen twigs. A
    blend over the stone's foot smeared the rust bracken into orange streaks, so I
    undid it and blended only the stump's snow. Then snow on the young spruce's boughs,
    stippled on each bough's upper face and heaviest on the sunward side.
15. `wait(24*60); varnish{coats=0.3}; relief()`.

19 chunks. About 16 more chunks were tried and undone, or rolled back by `try`.

## HOW THE EASEL FELT

**What felt like painting.** Looking, disliking and taking it back. The loop of `do`,
look, "that's a gray fog strip, not a snowbank", `undo` is exactly the rhythm of
scraping out a passage. Time felt real: the snow bank over the wood smeared because the
black was still open after a day (`drying()` said "open"). That is what would happen
with real bone-black-heavy paint, and it forced a painter's decision (let it dry). The
sky and the stone came from light: I chose a sun, and the stone's warm left face and
violet flank followed. The glaze and scumble verbs (`color_over` with a mix toward
violet-gray) felt like handling paint, not setting pixels.

**What felt like programming.** Almost every *shape*. The young spruce and the wood's
spires are a Lua function I wrote (`spruce{}`: an axis and whorls of ribbons with
droop and lift), tuned by numbers (tier = height/22, width = tier·0.42) over three
tries. The bracken is a function too, and so is the snow on the boughs (a per-pixel
mask comparing a mask with itself shifted 2.5 units up). The upside is that a function
is a hand you can use again: the same spruce hand drew 17 wood spires and the near tree.
The downside is that I was specifying the tree, not drawing it.

**Could I draw?** Partly. The pencil pass was drawing: I read points off `look --grid`
and placed the boulder contour, the wood-edge line and the stump by eye, and `outline{}`
turned rough points into a believable broken stump and soft drift edges. That's the
closest the easel came to a hand. I could not draw the conifers by eye. There's no
`body_of`-like tool for a tree's silhouette, and the tree generator's spruce is a
mature forest tree (bare lower trunk; with `years=` a lollipop), so I computed
them. The boulder I modeled as a solid rather than drew, then traced its silhouette in
pencil from the overlay. Drawing followed the model, which is backward for Friedrich.
The stump was drawn (a dozen points, "c" corners) and it's also the weakest object in
the picture, so drawing by eye is possible but my hand is crude.

**Versus writing a program.** It's much better than a program you run blind. Undo,
`try` and the look after every chunk made it iterative in a way that changed my
decisions (the wood's mass came down twice, the grass went from meadow to single
tufts). But the unit of work is still "write a function with parameters, then run it",
and every non-trivial mark (a snowy bough, a frond) became a small geometry problem.

## FRICTION

1. **No young or dense conifer.** `tree{habit="spruce"}` grows a tall forest spruce with
   a bare, thick lower trunk and a gappy crown; `years=14` made a lollipop with a
   19-unit trunk. *Workaround:* my own `spruce{}` from ribbons (chunk 7), used for the
   wood and the near tree.
2. **Masks with a box in them leave ruled edges.** `rect()` inside a region (the snow
   bank, the foot drift, the stump's snow) printed straight edges three times.
   `rect():soften()` wasn't enough when the strokes hug the mask. *Workaround:*
   hand-written `mask(function) smoothstep...` fades and `hug=false`. A
   `fade{top=, bottom=...}` or a soft-rect helper would save this.
3. **"The upper face of a shape" (where snow lies) had no mask op.** *Workaround:*
   `mask(function(x,y) return m:at(x,y) * (1 - m:at(x, y-2.5)) end)`, per pixel. A
   `m:shift(dx, dy)` would make it `m - m:shift(0, 2.5)`.
4. **Hidden open paint.** After `wait(24*60)` the wood was still open and the snow over
   it turned to a gray fog band. I only learned why by printing `drying()`.
   *Workaround:* `dry()`, which jumped the clock 20 days. A look mode showing open or
   tacky areas would have warned me.
5. **The 2H first pass is invisible** on the warm mid-tone `friedrich` ground at
   1000px, even in a 2× crop. `drawing_mask():area()` proved it was there.
   *Workaround:* the 3B second pass (which is also what Friedrich did).
6. **Low sun, narrow light range.** Lit flat snow at an 11° sun is only 0.37–0.49 in
   `v:at(...).shade.value`, and `v:at` over a body returns the body's shade, so a
   ground-color field needed `what == "ground"` and a probed remap. My first snow
   was lilac everywhere.
7. **`sward` defaults** gave 6,286 tufts and 24,871 blades in hard-edged patches (a
   meadow, not winter stubble). *Workaround:* cull with my own noise.
8. **Cast shadow of a small object at a low sun** is a long, even, ruler-straight band.
   That's correct, but pictorially it's too clean. There's no "break it with the snow's
   relief" option. *Workaround:* lighter coats (0.32) and a wider `soft`, then grass
   over it.
9. **`look --scale 3.2` timed out** (60 s) late in the session under load (my own
   3200 render plus two other painters), so the last details went unchecked at 3200.
10. **`size` for bodies is radii**; my first boulder was twice the intended size (the
    README does say "center, radii" for `body.ellipsoid`, but `s:size` reads like full
    size).
11. **`blend` smears everything inside the region, including small details laid
    earlier** (bracken went orange across the foot). That's physically fair. It was my
    mistake, but it's easy to make when the region is a grown mask.

## Critique

What works: the big design is Friedrich-like. The dark wood's spires step down against
a pale veiled sky, the lone young spruce stands dark against the light and the
snow-capped erratic sits between them. The value structure is strong in the squint:
dark left mass, pale field, a dark vertical right of center. The spruce with snow on
its boughs is the best passage. The varnish pulls the palette into a winter-afternoon
warmth.

What doesn't:
- **The stump** is a white post with a flat top of raw wood and a dab of snow. It has
  no roundness, no peeling bark, no roots. It reads as a prop.
- **The wood's interior** is a flat near-black dome with no inner structure, trunks or
  depth. The pale blotches low in it are only partly fixed.
- **The boulder** reads as stone but is loaf-shaped and a little too even. The black
  "fissure" specks are pure black and scattered like pepper, and the snow lip at its
  foot is streaky and hard.
- **Regularity:** the spruce tiers are nearly symmetric chevrons. The wood spires share
  one shape. The stump's cast shadow is a ruler band.
- **The foreground** is still thin for Friedrich: a few tufts, fronds and twigs. He
  would have given me every blade, seed head and pebble at the bottom edge.
- The sky has faintly blotchy mauve flecks rather than his smooth stippled gradations.
  I never did the smoothing stipple I planned.
