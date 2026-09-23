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

local A = body.block({615, 578, 0}, {540, 150, 220}, 12)
local B = body.block({598, 445, 25}, {610, 130, 270}, 16)
local C = body.block({560, 322, 5}, {460, 125, 230}, 22)
rock = (A + B + C)
  :turn({600, 450, 0}, 0.32, 0.08, 0.025)
  :rough(11, 150, 3)
  - body.block({738, 440, 0}, {10, 420, 500}, 3)
  - body.block({470, 560, 0}, {7, 200, 500}, 3)
rock = rock:cut({360, 320, 40}, {-0.7, -0.6, 0.3}, 11, 3)
  :cut({820, 300, 40}, {0.6, -0.7, 0.2}, 12, 3)
  :cut({330, 600, 60}, {-0.9, 0.2, 0.4}, 14, 3)
  :rough(3, 40, 8, true)
  :rough(1.3, 14, 4, true)
boulder = body.ellipsoid({215, 670, 90}, {105, 52, 70}):turn({215, 670, 90}, -0.3, 0.2, 0.05):rough(7, 80, 6)
  :cut({215, 635, 90}, {-0.2, -1, 0.3}, 13, 4):cut({300, 670, 90}, {1, -0.2, 0.3}, 15, 4):rough(0.8, 16, 7, true)
f = form{ {rock, dist=0.25}, {boulder, dist=0.1}, light={from={-1, -0.55}, front=0.35, ambient=0.22, penumbra=0.06, bounce=0.15} }
rockm = f:silhouette{parts={1}, soft=0.5}
boulm = f:silhouette{parts={2}, soft=0.5}
work(rockm + boulm, {hand="body", color=function(x, y) return mix("#443e36", "#b9a98a", smoothstep(0.05, 0.85, f:value(x, y))) end,
  angle=f:field("fall"), length={10, 30}, coverage=4, medium=0.12})

--@ chunk 7 · clock 38574.35546875

behind = -((rockm + boulm):grow(0.5))
spruces = {}
local specs = {{40, 528, 820, 41}, {175, 522, 700, 42}, {300, 516, 560, 43}, {118, 520, 420, 44}}
local trunk, limb, twig = brush("round", 5), brush("round", 2.2), brush("rigger", 1.0)
for k, sp in ipairs(specs) do
  local t = tree{habit="spruce", x=sp[1], y=sp[2], height=sp[3], seed=sp[4]}
  spruces[k] = t
  local dark = mix("#1e2520", "#39403a", (k == 4) and 0.6 or 0.15)
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
      for j = 1, #l.pts do w[j] = 3 + 7 * (1 - (j-1)/#l.pts); pts[j] = {l.pts[j][1], l.pts[j][2] + 4 * (j-1)/#l.pts} end
      local r = ribbon(pts, w)
      f2 = f2 and (f2 + r) or r
    end
  end
  local nd = f2:roughen(2, 6, k, 0.8) * behind
  local cx = sp[1]
  work(nd, {hand="hatch", tool="round 1.6", length={3, 8}, coverage=2.6,
    color=function(x, y) return mix(dark, mix("#2c3a2c", "#4a5244", (k == 4) and 0.5 or 0), 0.5 + 0.5*math.sin(x*0.3 + y*0.2)) end,
    angle=function(x, y) return (x < cx and 2.5 or 0.64) + 0.3*math.sin(y/9) end, angle_jitter=0.5})
end
print(#spruces)

--@ chunk 8 · clock 38574.35546875

dry()
stainn = noise{seed=31, octaves=5, period=60, stretch={1.5708, 4}}
grain = noise{seed=32, octaves=4, period=18}
stone = function(x, y)
  local v = smoothstep(0.08, 0.9, f:value(x, y))
  local c = gradient({{0, "#4f4943"}, {0.35, "#7c7263"}, {0.7, "#ab9c80"}, {1, "#d4c3a0"}}, v)
  local s = stainn:at01(x, y)
  c = mix(c, "#3f3d38", 0.45 * smoothstep(0.62, 0.9, s))            -- rain streaks down the face
  c = mix(c, "#6d7456", 0.35 * smoothstep(560, 680, y) * grain:at01(x, y))  -- algae low on the rock
  return shift(c, 0.03 * grain(x, y), 0, 0)
end
local faces = rockm * f:lit{parts={1}, soft=0.1}
local shade = rockm * f:shadow{parts={1}, soft=0.1}
work(shade, {hand="body", tool="filbert 4", color=stone, angle=f:field("fall"), length={6, 18}, coverage=3.5, medium=0.12, angle_jitter=0.25})
work(faces, {hand="body", tool="filbert 4", color=stone, angle=function(x, y) return 0.04 + 0.08*grain(x, y) end, length={6, 20}, coverage=3.5, medium=0.12})
work(boulm, {hand="body", tool="filbert 3", color=function(x, y) return mix(stone(x, y), "#6a6c58", 0.3*grain:at01(x*2, y)) end, angle=f:field("across"), length={5, 14}, coverage=3.5, medium=0.12})
