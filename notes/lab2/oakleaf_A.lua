-- easel session "oakleaf_A": a painting replayed chunk by chunk.
--   easel run notes/lab2/oakleaf_A.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab 2, subject "oakleaf": a broad summer oak in the middle distance (the crown about a
-- third of the canvas wide) standing in a meadow, a low line of distant trees behind, soft
-- daylight. Shared setup for oakleaf_A (round 1's winning detailed recipe) and oakleaf_H
-- (hierarchical detail): canvas, world and sun, sky, the far tree line, the meadow, the oak
-- drawn and grown, its cast shadow.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=3/2, size=440, seed=71}
HZ = 432
WORLD = world{horizon=HZ, sun={azimuth=-125, elevation=40}}
SUN = {-0.6, -0.6, 0.45}
-- soft daylight: a pale blue-gray summer sky, milky and warm toward the horizon
sky = function(x, y)
  return gradient({{0, "#7a90ae"}, {0.4, "#9aabbd"}, {0.8, "#c3c8c3"}, {1, "#d3d1c0"}}, y / HZ)
end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the far tree line: a low wood along the horizon with a few breaks (fields between groves)
local fn = noise{seed=5, period=60, octaves=3}
FARPTS = {}
for i = 0, 50 do
  local x = -10 + i * 20.4
  local h = 17 + 11 * fn(x, 0) + 9 * math.exp(-((x - 600) / 60)^2) + 6 * math.exp(-((x - 60) / 50)^2)
  if x > 165 and x < 225 then h = -6 end                  -- breaks: open field seen through
  if x > 706 and x < 736 then h = 3 end
  if x > 860 and x < 930 then h = h * 0.35 end
  FARPTS[#FARPTS + 1] = {x, HZ + 2 - h}
end
farline = outline{pts=FARPTS, open=true, char="soft", lobe=10, seed=9}
FAR = farline:below(HZ + 6) * above(function(x) return HZ + 5 + 1.5 * math.sin(x / 50) end)
-- the meadow: hazy and light at the horizon, deeper and warmer toward us
land = below(function(x) return HZ + 2 * math.sin(x / 80) end)
landcol = function(x, y)
  local t = smoothstep(HZ, H, y)
  return gradient({{0, "#8f9570"}, {0.25, "#7f8a55"}, {0.6, "#66743c"}, {1, "#4c5a2c"}}, t)
end
-- the oak: broad, lopsided (heavier and lower on the left), lobed, a notch in the top right
CROWN = {{292,352},{272,318},{280,282},{300,256},{306,226},{334,204},{366,196},{388,172},{426,160},
         {462,168},{486,186,"c"},{512,168},{552,172},{580,192},{604,220},{606,252},{626,280},
         {620,314},{600,338},{606,358,"c"},{580,376},{546,372},{520,386},{480,378},{452,390},
         {420,380},{384,386},{350,374},{318,378}}
TRUNK = {{452,474},{450,440},{447,405},{446,380}}
crown = outline{pts=CROWN, char="soft", seed=72}
tree = tree_in{crown=crown, trunk=TRUNK, species="oak", season="summer", sun=WORLD, seed=73}
print(tree, "touch_w", tree.touch_w, "grain", tree.grain)
-- its shadow on the meadow, thrown right and a little toward us
SHADOW = ellipse(520, 478, 118, 11):roughen(4, 30, 3, 3)
LEAF = {deep="#212a1c", body="#34402a", sunny="#5c6a3a", shade="#263020", skyshade="#4a5446",
        mid="#46522e", lit="#6f7c45", top="#98a060"}
WOOD = {dark="#3b342c", light="#7d7566"}

--@ chunk 2 · clock 0

-- A chunk 2: the sky thin in long level strokes, blended; the meadow as tone (sketchbook 2, 4)
skym = above(function(x) return HZ + 6 + 2 * math.sin(x / 80) end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})
work(land, {hand="body", color=landcol, angle=0.02, length={20, 60}, coverage=3.4, medium=0.18})

--@ chunk 3 · clock 0

-- A chunk 3: a day later, the second sky layer stippled over the first, kept off the land
wait(24*60)
skyonly = above(function(x) return HZ - 2 + 2 * math.sin(x / 80) end)
stipple(skyonly, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55, clip=skyonly})

--@ chunk 4 · clock 1440
-- A chunk 4: dry() before anything goes over the sky and meadow. The far tree line hatched
-- (sketchbook 4 "hedgerows and groves": round 1.2-1.6, lengths 1.5-5, turning angles), a lit
-- layer on its sun side; the oak shadow on the meadow; the oak wood (tree_in step 1)
dry()
local turn = noise{seed=21, period=8}
local way = function(x, y) return 2.2 * turn(x, y) end
work(FAR, {hand="hatch", tool="round 1.4", length={1.5, 5}, coverage=3, clip=FAR, angle=way, hug=false, medium=0.2,
  color=function(x, y) return mix("#6c776d", "#7d8676", smoothstep(HZ - 20, HZ + 4, y)) end})
local farlit = FAR * mask(function(x, y) return smoothstep(0.1, 0.6, FAR:at(x + 3, y + 4) - FAR:at(x - 3, y - 4) + 0.3) end)
work(farlit, {hand="hatch", tool="round 1.2", length={1.5, 4}, coverage=1.6, clip=FAR, angle=way, hug=false, medium=0.2, color="#8e9580"})
work(SHADOW, {hand="hatch", tool="round 1.6", length={3, 8}, angle=0.02, coverage=2.4, clip=SHADOW, hug=false, color="#5a6538", medium=0.2})
-- and dry() again before the wood goes over them (the trunk laid over open far paint churned it)
dry()
local thick = tree:wood(3.5)
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=WOOD.dark, medium=0.15})
local stout = tree:wood(7)
local lx, ly = -SUN[1] * 3, -SUN[2] * 1.5
local flank = stout * mask(function(x, y) return 1 - stout:at(x + lx, y + ly) end)
work(flank, {hand="body", tool="round 1.4", length={3, 8}, coverage=2.5, angle=1.5, clip=thick, color="#6a6154", medium=0.15})  -- a step lighter than round 1: at this size the full light read as a bandage
tree:paint_wood(brush("round", 2.4), {color=WOOD.dark, min=1.2, max=3.5})
tree:paint_wood(brush("rigger", 0.9), {color=WOOD.dark, min=0.5, max=1.2})
tree:paint_wood(brush("rigger", 0.55), {color=WOOD.dark, max=0.5})
print(clock())

--@ chunk 5 · clock 28144.2763671875

-- A chunk 5: dry() again, then the leaves in full (sketchbook 5, tree_in steps 2-4), as round 1 A
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

--@ chunk 6 · clock 43678.212890625
-- A chunk 6: the meadow particulars (sketchbook 8): a sward over a noise patch field, each tuft
-- tinted from what is under it; the foot of the oak buried in grass; then the 1,400 long curving
-- rigger blades through the near band ("the single most effective foreground chunk")
wait(24*60)
local pn = noise{seed=33, period=90}
local reg = land * mask(function(x, y) return smoothstep(-0.2, 0.3, pn(x, y)) * smoothstep(HZ + 30, HZ + 90, y) end)
local tufts = sward{region=reg, horizon=HZ, near=H, height=26, spacing=2.6, thin=0.5, flowers=0.02, seed=4,
  wind={lean=0.12, gust=0.2, period=160, seed=2}}
local g = brush("rigger", 0.7)
local straw = "#a39a5c"
for i, t in ipairs(tufts) do
  if i % 4 == 1 then g:reload(mix(sample(t.x, t.y, 2), straw, 0.12 + 0.35 * rand(0, 1) * rand(0, 1)), 0.7) end
  for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={clamp(0.25 + 0.5 * t.scale, 0.2, 0.9), 0}, ramps={0.05, 0.7}}) end
end
print("tufts", #tufts)
-- grass over the foot of the trunk
local fb = brush("rigger", 0.6)
for i = 1, 180 do
  local x, y = rand(432, 474), rand(476, 489)
  if i % 7 == 1 then fb:reload(i % 14 == 1 and "#6f7a3e" or "#4d5a2c", 0.7) end
  local h = rand(4, 11)
  fb:stroke({{x, y}, {x + randn(0, 1), y - h * 0.6}, {x + randn(0, 1.6), y - h}}, {pressure={0.4, 0}, ramps={0.05, 0.75}})
end
-- the near band: long curving blades
local lean = noise{seed=12, period=140}
local cols = {"#3e4a24", "#56652f", "#6f7d3c", "#8a9150", "#344020", "#9c9a5c"}
local rb = brush("rigger", 0.8)
local nb = 0
for i = 1, 1400 do
  local x, y = rand(-10, 1010), rand(560, 675)
  local depth = smoothstep(560, 667, y)
  local h = (8 + 34 * depth) * rand(0.5, 1.6)
  local a = -math.pi / 2 + 0.35 * lean(x, y) + randn(0, 0.22)
  local c = randn(0, 0.25)
  if nb % 7 == 0 then rb:reload(cols[1 + (nb // 7) % #cols], 0.75) end
  local mx, my = x + math.cos(a) * h * 0.5, y + math.sin(a) * h * 0.5
  local tx, ty = x + math.cos(a + c) * h, y + math.sin(a + c) * h
  rb:stroke({{x, y}, {mx, my}, {tx, ty}}, {pressure={0.35 + 0.5 * depth, 0}, ramps={0.05, 0.75}})
  nb = nb + 1
end
print("blades", nb)

--@ chunk 7 · clock 45118.212890625
-- the finish, the same for both versions: varnish thinner than round 1 (coats 0.3 pooled into brown
-- worm lines around every dab edge at 3200 on the smooth crown; coats 0.12 does not)
wait(24*60); varnish{color="#e6d3a4", coats=0.12, vary=0.1}; relief()
