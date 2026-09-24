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
