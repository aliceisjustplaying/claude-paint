-- easel session "example": a painting replayed chunk by chunk.
--   easel run paintings/lua/example.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=7}; print(H); print(pal)

--@ chunk 2 · clock 0

HZ = 470
sky = function(x, y) return gradient({{0,"#5d7396"},{0.5,"#9fabb8"},{0.85,"#d9cfb4"},{1,"#e9d6a6"}}, y/HZ) end
skym = above(function(x) return HZ + 15 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.5, medium=0.25})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

wait(24*60)
far = noise{seed=3, octaves=5, period=260}
crest = function(x) return 420 - 45*far:at01(x, 0) - 25*math.exp(-((x-640)/120)^2) end
ridge = below(crest):roughen(1.5, 12)
work(ridge, {hand="body", color=function(x,y) return mix("#7d8aa0", "#9aa2ad", (y-380)/100) end, angle=0.05, length={25,70}, coverage=3, medium=0.3})

--@ chunk 4 · clock 1440

wait(24*60)
near = noise{seed=11, octaves=5, period=180}
knoll = function(x) return 560 - 70*math.exp(-((x-300)/260)^2) + 12*near(x, 0) end
ground = below(knoll):roughen(2, 20)
work(ground, {hand="body", color=function(x,y) return mix("#3a3b2c", "#23251f", (y-480)/200) end, angle=function(x,y) return 0.15*near(x,y) end, length={15,45}, coverage=3.2, medium=0.15})

--@ chunk 5 · clock 2880

wait(24*60)
banks = noise{seed=5, octaves=4, period=220}
mistcov = function(x, y) local up = (590 - y)/110 + 0.4*banks(x*0.6, y*2.5); return 3.2*(1 - smoothstep(0, 1, up)) end
stipple(ridge - ground:grow(4), {width=3, color="#cfccc2", coverage=mistcov, pressure={0.5,0.9}, dips={16,0.35,0.7}, aim=false, medium=0.6})

--@ chunk 6 · clock 4320

wait(24*60)
sp = tree{habit="spruce", x=310, y=knoll(310)+6, height=210, seed=4}
print(#sp.limbs, "limbs; bounds", table.concat(sp.bounds, ", "))
local orders = {}
for _, l in ipairs(sp.limbs) do orders[l.order] = (orders[l.order] or 0) + 1 end
for o, n in pairs(orders) do print("order", o, n) end
print("trunk base width", sp.limbs[1].w[1], "points", #sp.limbs[1].pts)

--@ chunk 7 · clock 5760

local dark = "#1d2520"
local trunk = brush("round", 5)
trunk:load("#2b2620", 1)
trunk:stroke(sp.limbs[1].pts, {pressure={0.95, 0.35}, ramps={0.02, 0.3}})
local limb, twig = brush("round", 2.2), brush("rigger", 1.0)
for i, l in ipairs(sp.limbs) do
  if l.order >= 1 and #l.pts >= 2 then
    local b = (l.w[1] > 1.2) and limb or twig
    if i % 4 == 0 or b:fullness() < 0.25 then b:reload(dark, 0.9) end
    b:stroke(l.pts, {pressure={0.8, 0.25}, ramps={0.05, 0.5}})
  end
end
print(limb, twig)

--@ chunk 8 · clock 5760

local f = nil
for _, l in ipairs(sp.limbs) do
  if l.order == 1 and #l.pts >= 2 then
    local w = {}
    for k = 1, #l.pts do w[k] = 2 + 4 * (1 - (k-1)/#l.pts) end
    local pts = {}
    for k, p in ipairs(l.pts) do pts[k] = {p[1], p[2] + 2.5 * (k-1)/#l.pts} end
    local r = ribbon(pts, w)
    f = f and (f + r) or r
  end
end
needles = f:roughen(1.5, 5)
work(needles, {hand="hatch", tool="round 1.4", color=function(x,y) return mix("#1a2419", "#2e3828", (y-290)/200) end,
  angle=function(x,y) return (x < 310 and 2.4 or 0.7) end, angle_jitter=0.5, length={3,7}, coverage=2.2})

--@ chunk 9 · clock 5760

local g = brush("rigger", 0.8)
for i = 1, 90 do
  local x = rand(40, 960)
  local y = knoll(x) + rand(2, 30)
  if i % 6 == 1 then g:reload(mix("#6d6a4a", "#a39a6a", rand()), 0.7) end
  local h = rand(6, 16)
  local lean = randn(0, 0.3)
  g:stroke({{x, y}, {x + lean*h*0.5, y - h*0.6}, {x + lean*h, y - h}}, {pressure={0.7, 0.1}})
end

--@ chunk 10 · clock 5760
wait(24*60); varnish(); relief()
