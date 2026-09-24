-- baretree setup (shared by A and B): a small study panel, 4:3, one bare winter oak
-- against a pale sky over a low ground. Everything here is geometry and color, no paint.
canvas{style="friedrich", aspect=4/3, size=300, seed=61}
HZ = 600                                   -- the far edge of the low ground
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=14}}   -- a low winter sun, from the left
SUN = {-0.8, -0.35, 0.45}
-- the sky: a pale, slightly warm winter sky, cool gray-blue overhead
sky = function(x, y)
  return gradient({{0,"#9aa6b3"},{0.45,"#bfc3c0"},{0.8,"#d9d3bf"},{1,"#e0d6ba"}}, y/HZ)
end
skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth"}
-- the ground: a low, gently uneven line; darker and warmer toward us
local gn = noise{seed=4, octaves=4, period=300}
GROUND = function(x) return HZ + 6 * gn(x, 0) - 10 * math.exp(-((x - 470) / 160)^2) end
skym = above(function(x) return GROUND(x) + 8 end)
land = below(GROUND)
ground = function(x, y)
  local t = smoothstep(HZ - 10, H, y)
  return mix(mix("#8b8a78", "#5f5c48", t), "#4a4636", smoothstep(0.6, 1, t) * 0.6 + 0.1 * gn(x * 4, y * 2))
end
-- the oak: a broad, lopsided winter crown with a long low limb to the right; a trunk leaning a little
OAKCROWN = {{120,420},{128,350},{165,285},{210,230},{250,168},{318,118},{372,108},{420,70},{488,58},{548,84},
            {610,78},{690,112},{760,150},{830,210},{880,262},{912,330,"c"},{880,372},{842,392},{812,430,"c"},
            {740,452},{660,468},{580,474},{500,488},{420,480},{330,486},{250,478},{190,470,"c"},{140,452}}
OAKTRUNK = {{452,660},{448,590},{440,500}}
oak = tree_in{crown=outline{pts=OAKCROWN, char="soft", seed=5}, trunk=OAKTRUNK, species="oak",
              season="winter", sun=WORLD, seed=11, girth=0.06}
print(oak)
