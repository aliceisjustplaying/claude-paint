-- easel session "lab1_foliage_A": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab1_foliage_A.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab, subject "foliage": one broadleaf crown in summer leaf against the sky,
-- trunk mostly hidden. Shared setup for foliage_A (old way) and foliage_B (new way):
-- canvas, palette, world and sun, sky colors, the crown and trunk drawn, the tree grown.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=4/3, size=320, seed=61}
HZ = 688
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=42}}
SUN = {-0.6, -0.6, 0.45}
-- a summer afternoon sky: clear blue-gray above, pale and warm at the horizon
sky = function(x, y)
  return gradient({{0, "#6d86a8"}, {0.45, "#95a8bc"}, {0.85, "#c6cbc6"}, {1, "#d6d3c3"}}, y / HZ)
end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- a far strip of land under the tree, hazy
land = below(function(x) return HZ + 3 * math.sin(x / 70) end)
landcol = function(x, y) return mix("#8d9278", "#5e6545", smoothstep(HZ, H, y)) end
-- the crown: broad, lopsided (heavier and lower on the right), lobed, a notch top left
CROWN = {{166,452},{132,392},{146,318},{196,268},{236,236},{246,196},{296,160},{356,150},{400,118},
         {462,96},{516,112},{540,146,"c"},{582,116},{646,106},{714,136},{744,184},{798,210},{846,262},
         {858,326},{830,372,"c"},{870,418},{884,488},{850,540},{790,556},{742,588},{672,590},{620,566,"c"},
         {560,590},{470,600},{420,578},{360,598},{290,590},{240,552},{212,508,"c"}}
TRUNK = {{508,752},{503,700},{497,640},{490,590}}
crown = outline{pts=CROWN, char="soft", seed=62}
tree = tree_in{crown=crown, trunk=TRUNK, species="oak", season="summer", sun=WORLD, seed=63}
print(tree)
-- leaf colors, deep shade to top light
LEAF = {deep="#212a1c", body="#34402a", sunny="#5c6a3a", shade="#263020", skyshade="#4a5446",
        mid="#46522e", lit="#6f7c45", top="#98a060"}
WOOD = {dark="#3b342c", light="#7d7566"}

--@ chunk 2 · clock 0

-- A chunk 2: the sky thin in long level strokes, blended; the far land as tone (sketchbook 2)
skym = above(function(x) return HZ + 6 + 3 * math.sin(x / 70) end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})
work(land, {hand="body", color=landcol, angle=0.02, length={20, 60}, coverage=3.4, medium=0.18})

--@ chunk 3 · clock 0

-- A chunk 3: a day later, the second sky layer stippled over the first (sketchbook 2), kept off the land
wait(24*60)
skyonly = above(function(x) return HZ - 2 + 3 * math.sin(x / 70) end)
stipple(skyonly, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55, clip=skyonly})

--@ chunk 4 · clock 1440

-- A chunk 4: dry() before the tree goes over the sky; the wood (sketchbook 5, tree_in step 1)
dry()
local thick = tree:wood(3.5)
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=WOOD.dark, medium=0.15})
local stout = tree:wood(7)
local lx, ly = -SUN[1] * 3, -SUN[2] * 1.5
local flank = stout * mask(function(x, y) return 1 - stout:at(x + lx, y + ly) end)
work(flank, {hand="body", tool="round 1.4", length={3, 8}, coverage=2.5, angle=1.5, clip=thick, color=WOOD.light, medium=0.15})
tree:paint_wood(brush("round", 2.4), {color=WOOD.dark, min=1.2, max=3.5})
tree:paint_wood(brush("rigger", 0.9), {color=WOOD.dark, max=1.2})
print(clock())

--@ chunk 5 · clock 17730.447265625

-- A chunk 5: dry() again, then the leaves in full (sketchbook 5, tree_in steps 2-4)
dry()
local lv, L = tree:leaves(), tree:light()
local turn = noise{seed=9, period=9}
local way = function(x, y) return 2.4 * turn(x, y) end
work(lv, {hand="hatch", tool="round 1.6", length={3, 6}, coverage=2.4, clip=lv, angle=way, angle_jitter=0.8, hug=false, medium=0.2,
  color=function(x, y) local v = L:at(x, y) return mix(mix(LEAF.deep, LEAF.body, smoothstep(0.1, 0.45, v)), LEAF.sunny, smoothstep(0.5, 0.85, v)) end})
local gp = tree:gaps()
work(gp, {hand="detail", tool="round 1.4", color=sky, clip=gp, coverage=2})
local b = brush("round", tree.touch_w)
local n = 0
n = n + tree:paint(b, {color=LEAF.shade, lit={0, 0.4}})
n = n + tree:paint(b, {color=LEAF.skyshade, lit={0.15, 0.4}, depth={0.25, 1}, share=0.5})
n = n + tree:paint(b, {color=LEAF.mid, lit={0.4, 0.6}})
n = n + tree:paint(brush("round", tree.touch_w * 0.9), {color=LEAF.lit, lit={0.6, 0.78}})
n = n + tree:paint(brush("round", tree.touch_w * 0.8), {color=LEAF.top, lit={0.78, 1}, every=6})
print("touches", n, "clock", clock())

--@ chunk 6 · clock 35255.853515625
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
