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
