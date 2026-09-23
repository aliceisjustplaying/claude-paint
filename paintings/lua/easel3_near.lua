-- easel session "easel3_near": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel3_near.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.3, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0

HZ = 470
skyn = noise{seed=5, octaves=4, period=300, warp={120, 40}}
sky = function(x, y)
  local t = clamp(y / 520 + 0.06 * skyn(x, y), 0, 1)
  return gradient({{0, "#7f8d9c"}, {0.35, "#a9b1b5"}, {0.72, "#d9d3bf"}, {1, "#e8dcbc"}}, t)
end
skym = above(function(x) return 600 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

wait(24*60)
stipple(skym, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55})

--@ chunk 4 · clock 1440

wait(24*60)
farn = noise{seed=8, octaves=6, period=70}
spires = {}
for i, px in ipairs(uneven(70, 0, 1000, 0.7, 0.5, 9)) do
  spires[i] = {px, rand(8, 42) * (0.4 + 0.6*math.abs(math.sin(px/170 + 1))), rand(5, 10)}
end
farcrest = function(x)
  local top = 478 - 14*farn:at01(x, 0) - 10*math.exp(-((x-820)/120)^2)
  local best = top
  for _, s in ipairs(spires) do
    local d = math.abs(x - s[1]) / s[3]
    if d < 1 then best = math.min(best, top - 6 - s[2]*(1 - d)) end
  end
  return best
end
farwood = below(farcrest):roughen(1.5, 4, 3, 0.8)
work(farwood, {hand="hatch", tool="round 2.2", length={4, 10}, coverage=3.4, angle=function(x,y) return (math.floor(x/7)%2==0) and 2.5 or 0.64 end, angle_jitter=0.5, medium=0.3,
  color=function(x, y) return mix(mix("#858c93", "#9fa39f", smoothstep(430, 520, y)), "#7c8588", 0.4*farn:at01(x*2, y)) end})
floor = function(x) return 512 + 6*math.sin(x/140) end
floorm = below(floor):roughen(1.5, 30, 4, 1)
fn = noise{seed=12, octaves=5, period=110}
work(floorm, {hand="body", length={10, 30}, coverage=4.5, angle=function(x, y) return 0.08*fn(x, y) end, medium=0.15, aim="masstone",
  color=function(x, y)
    local t = smoothstep(505, 769, y)
    local c = mix("#8e8a76", "#5d4c34", smoothstep(0, 0.4, t))
    c = mix(c, "#3a2f23", smoothstep(0.4, 1, t))
    return mix(c, "#6e5d3c", 0.4*fn:at01(x, y))
  end})

--@ chunk 5 · clock 2880
print(dry(), drying(500,480), drying(500,600))

--@ chunk 6 · clock 38574.35546875

local A = body.block({620, 590, 0}, {530, 140, 220}, 14):turn({620, 590, 0}, 0, 0, -0.03)
local B = body.block({598, 458, 25}, {600, 125, 270}, 18):turn({598, 458, 25}, 0.03, 0, 0.02)
local C1 = body.block({500, 335, 10}, {330, 135, 230}, 24):turn({500, 335, 10}, 0, 0.05, 0.05)
local C2 = body.block({760, 352, 15}, {150, 105, 200}, 28):turn({760, 352, 15}, 0, 0, -0.08)
rock = (A + B + C1 + C2)
  :turn({600, 460, 0}, 0.32, 0.08, 0.02)
  :rough(14, 130, 3)
rock = rock:cut({330, 300, 40}, {-0.7, -0.7, 0.3}, 11, 3)
  :cut({845, 300, 40}, {0.6, -0.7, 0.3}, 12, 3)
  :cut({300, 610, 60}, {-0.9, 0.3, 0.4}, 14, 3)
  :rough(4, 36, 8, true)
  :rough(1.3, 13, 4, true)
boulder = body.ellipsoid({215, 672, 90}, {105, 52, 70}):turn({215, 672, 90}, -0.3, 0.2, 0.05):rough(9, 70, 6)
  :cut({215, 635, 90}, {-0.2, -1, 0.3}, 13, 4):cut({300, 672, 90}, {1, -0.2, 0.3}, 15, 4):rough(0.8, 16, 7, true)
f = form{ {rock, dist=0.25}, {boulder, dist=0.1}, light={from={-1, -0.55}, front=0.35, ambient=0.22, penumbra=0.06, bounce=0.15} }
rockm = f:silhouette{parts={1}, soft=0.5}
boulm = f:silhouette{parts={2}, soft=0.5}
work(rockm + boulm, {hand="body", color=function(x, y) return mix("#443e36", "#b9a98a", smoothstep(0.05, 0.85, f:value(x, y))) end,
  angle=f:field("fall"), length={10, 30}, coverage=4, medium=0.12})

--@ chunk 7 · clock 38574.35546875
behind = -((rockm + boulm):grow(0.5))
-- the wood's dark interior behind the edge trees
local inn = noise{seed=61, octaves=5, period=40}
local sp2 = {}
for i, px in ipairs(uneven(16, -10, 330, 0.6, 0.5, 5)) do sp2[i] = {px, rand(60, 190), rand(12, 22)} end
local crest = function(x)
  local top = 470 - 60 * smoothstep(360, 60, x)
  local best = top
  for _, s in ipairs(sp2) do
    local d = math.abs(x - s[1]) / s[3]
    if d < 1 then best = math.min(best, top - s[2]*(1 - d)^1.3) end
  end
  return best
end
woodm = (below(crest) * above(function(x) return 525 end) * mask(function(x, y) return x < 330 + 40*inn(x, y) and 1 or 0 end)):roughen(3, 5, 7, 1) * behind
work(woodm, {hand="hatch", tool="round 2", length={4, 10}, coverage=3, medium=0.2, angle=function(x, y) return (math.floor(x/9)%2==0) and 2.5 or 0.64 end, angle_jitter=0.6,
  color=function(x, y) return mix("#3b4540", "#56605a", 0.5*inn:at01(x, y) + 0.3*smoothstep(520, 380, y)) end})
spruces = {}
local specs = {{40, 530, 830, 41}, {178, 524, 700, 42}, {296, 518, 520, 43}, {112, 522, 430, 44}, {245, 520, 300, 45}}
local trunk, limb, twig = brush("round", 5), brush("round", 2.2), brush("rigger", 1.0)
for k, sp in ipairs(specs) do
  local t = tree{habit="spruce", x=sp[1], y=sp[2], height=sp[3], seed=sp[4]}
  spruces[k] = t
  local dark = mix("#1c231e", "#3a423b", (k >= 4) and 0.55 or 0.1 * k)
  for i, l in ipairs(t.limbs) do
    if #l.pts >= 2 then
      local b = (l.order == 0) and trunk or ((l.w[1] > 1.2) and limb or twig)
      if i % 4 == 1 or b:fullness() < 0.25 then b:reload(l.order == 0 and "#2e2822" or dark, 0.9) end
      b:stroke(l.pts, {pressure=(l.order == 0) and {0.95, 0.35} or {0.8, 0.2}, ramps={0.03, 0.5}, clip=behind})
    end
  end
  local f2 = nil
  for _, l in ipairs(t.limbs) do
    if l.order == 1 and #l.pts >= 2 then
      local w, pts = {}, {}
      local droop = 6 + 10 * rand()
      for j = 1, #l.pts do
        local u = (j-1)/#l.pts
        w[j] = 3 + 9 * (1 - u) * (0.7 + 0.6*rand())
        pts[j] = {l.pts[j][1], l.pts[j][2] + droop * u * u}
      end
      local r = ribbon(pts, w)
      f2 = f2 and (f2 + r) or r
    end
  end
  local nd = f2:roughen(3, 5, k, 0.8) * behind
  local cx = sp[1]
  local shade = noise{seed=70 + k, octaves=3, period=30}
  work(nd, {hand="hatch", tool="round 1.6", length={3, 8}, coverage=2.8,
    color=function(x, y)
      local lit = smoothstep(0.1, -0.6, (x - cx) / 60) * 0.5 + 0.35 * shade:at01(x, y)
      return mix(dark, (k >= 4) and "#4c5647" or "#35433a", lit)
    end,
    angle=function(x, y) return (x < cx and 2.55 or 0.6) + 0.35*math.sin(y/7) end, angle_jitter=0.55})
end
