-- easel session "rocks": a painting replayed chunk by chunk.
--   easel run paintings/lua/rocks.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=3}

--@ chunk 2 · clock 0

sky = function(x, y) return gradient({{0,"#6e7f98"},{0.7,"#b8bcbc"},{1,"#ddd2b4"}}, y/450) end
work(everywhere(), {hand="broad", color=sky, angle=0, coverage=4})
wait(24*60)
hills = noise{seed=4, octaves=4, period=300}
range = ridge{crest=function(x) return 330 - 90*hills:at01(x, 0) - 60*math.exp(-((x-620)/140)^2) end, depth=320, seed=7, lean={0.9, 0.7}, gullies={45, 0.5}, base=560, z0=-600}
rock = body.ellipsoid({330, 560, 60}, {150, 105, 110}):turn({330, 560, 60}, 0.3, 0.1, -0.12):rough(15, 150, 1):cut({330, 480, 60}, {-0.35, -1, 0.45}, 10, 4):rough(0.9, 25, 2, true)
f = form{ {range, dist={2.0, 0}}, {rock, dist=0.3}, light={from={-1, -0.7}, front=0.5, ambient=0.2, penumbra=0.05} }
print(f:part(330, 470), f:sample(300, 520).shade.value)

--@ chunk 3 · clock 1440

local air = "#aeb4bc"
local function stone(x, y, light, shadow)
  local s = f:shade(x, y)
  local c = mix(shadow, light, smoothstep(0.05, 0.8, s.value))
  return mix(c, air, aerial(f:dist(x, y, 3), 6) * 0.6)
end
local rng = f:silhouette{parts=1, soft=0.6, haze={3, 4}}
work(rng * f:shadow{parts=1}, {hand="body", color=function(x,y) return stone(x, y, "#8d93a0", "#4f5866") end, angle=f:field("fall"), length={15,40}, coverage=3.5})
work(rng * f:lit{parts=1}, {hand="body", color=function(x,y) return stone(x, y, "#b9b3a6", "#6d7382") end, angle=f:field("fall"), length={15,40}, coverage=3.5})
local r = f:silhouette{parts=2, soft=0.3}
work(r * f:shadow{parts=2}, {hand="body", color=function(x,y) return stone(x, y, "#8a8070", "#3e3a36") end, angle=f:field("fall"), length={10,30}, coverage=3.5})
work(r * f:lit{parts=2}, {hand="body", color=function(x,y) return stone(x, y, "#d2c09c", "#6d6558") end, angle=f:field("across"), length={10,30}, coverage=3.5})
work(f:edges{concave=true} * f:parts_mask(2), {hand="detail", color="#2e2a28", angle=f:field("edge"), coverage=1.5})
