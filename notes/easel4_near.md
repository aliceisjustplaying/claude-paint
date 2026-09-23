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

(continued below as the refinement goes on)

## HOW THE EASEL FELT

(see end of file)

## FRICTION

(see end of file)
