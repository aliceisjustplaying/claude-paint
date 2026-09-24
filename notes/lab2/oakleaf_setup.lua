-- Round 6 lab 2, subject "oakleaf": the shared setup chunk. oakleaf_A.lua and oakleaf_H.lua both
-- start with it byte for byte (chunk 1).
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

