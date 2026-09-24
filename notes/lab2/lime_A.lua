-- easel session "lab2_lime_A": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab2_lime_A.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab 2, subject "lime": one tall summer lime (linden), a dense oval crown against a
-- pale afternoon sky, the whole tree in the frame with some ground. Shared setup for lime_A
-- and lime_H: canvas, palette, world and sun, sky colors, the crown and trunk drawn, the tree grown.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=3/4, size=320, seed=71}
HZ = 1150
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=42}}
SUN = {-0.6, -0.6, 0.45}
-- a pale summer afternoon: a light gray-blue above, paler and warm toward the horizon
sky = function(x, y)
  return gradient({{0, "#8ea1b8"}, {0.4, "#aab7c3"}, {0.8, "#cdd1cb"}, {1, "#dbd7c6"}}, y / HZ)
end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the ground: a far strip of land at the horizon, a meadow coming forward to the foot of the canvas
land = below(function(x) return HZ + 3 * math.sin(x / 70) end)
landcol = function(x, y) return mix(mix("#98997f", "#6d7249", smoothstep(HZ, HZ + 60, y)), "#4f5635", smoothstep(HZ + 60, H, y)) end
-- the crown: a tall egg, widest a little below the middle, a blunt dome at the top, lobed,
-- leaning a touch left; the lower edge hangs a little lower on the right
CROWN = {}
local N = 34
for i = 0, N - 1 do
  local t = 2 * math.pi * i / N
  local r = 1 + 0.045 * math.sin(5 * t + 1.3) + 0.03 * math.sin(9 * t + 0.4) + 0.02 * math.sin(13 * t + 2.2)
  local x = 500 - 12 * math.cos(t) + 285 * math.sin(t) * (1 - 0.2 * math.cos(t)) * r
  local y = 548 - 440 * math.cos(t) * r + 22 * math.max(0, math.sin(t)) * math.max(0, -math.cos(t))
  CROWN[#CROWN + 1] = (i % 6 == 3) and {math.floor(x), math.floor(y), "c"} or {math.floor(x), math.floor(y)}
end
TRUNK = {{502, 1215}, {499, 1140}, {495, 1060}, {492, 980}}
crown = outline{pts=CROWN, char="soft", seed=72}
-- denser than the species default (fewer voids, leaves deeper than the shell, larger clumps)
tree = tree_in{crown=crown, trunk=TRUNK, species="lime", season="summer", sun=WORLD, seed=73, girth=0.04, voids=0.06, shell=0.45, clump=0.03, leafiness=1.6}
print(tree)
-- leaf colors, deep shade to top light (a lime's leaf is a little yellower than an oak's)
LEAF = {deep="#1f291b", body="#34422a", sunny="#5f6f3a", shade="#253020", skyshade="#4a5548",
        mid="#48562f", lit="#728245", top="#a0a862"}
WOOD = {dark="#3a332b", light="#7d7566"}

--@ chunk 2 · clock 0

-- A chunk 2: the sky thin in long level strokes, blended; the ground as tone (sketchbook 2), the
-- tree's shadow on the meadow at its foot (the sun upper left: it falls back and to the right)
skym = above(function(x) return HZ + 6 + 3 * math.sin(x / 70) end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})
work(land, {hand="body", color=landcol, angle=0.02, length={20, 60}, coverage=3.4, medium=0.18})
SHADOW = ellipse(560, 1222, 120, 14):roughen(4, 10, 3, 1)
work(SHADOW, {hand="body", color="#3d4428", angle=0.02, length={10, 30}, coverage=2.6, medium=0.18, clip=SHADOW:grow(2)})

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

--@ chunk 5 · clock 14728.4453125

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

--@ chunk 6 · clock 30605.2216796875
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
