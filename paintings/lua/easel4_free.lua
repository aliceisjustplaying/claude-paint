-- easel session "easel4_free": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel4_free.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
H0 = canvas{style="friedrich", aspect=1.45, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 452
mn = noise{seed=5, octaves=4, period=90}
mound = function(x) return HZ + 3 - 52*math.exp(-((x-430)/175)^2) - 14*math.exp(-((x-250)/90)^2) + 3*mn(x,0) end
oak = tree{habit="dead_oak", x=372, y=mound(372)+5, height=318, seed=11}
-- the dolmen: a heavy granite capstone on boulders, one fallen beside it
cap = outline{{498,394,"c"},{502,378},{522,364},{556,358},{590,362},{614,372},{624,388,"c"},{608,402},{566,404},{528,402}, char="broken", seed=4}
up1 = outline{{507,398},{522,395},{537,400},{539,414},{533,421,"c"},{510,419,"c"},{505,409}, char="broken", seed=5}
up2 = outline{{552,402},{566,402},{569,420},{556,424,"c"},{550,414}, char="broken", seed=6}
up3 = outline{{586,400},{604,398},{616,404},{619,422},{614,436,"c"},{590,437,"c"},{584,420}, char="broken", seed=7}
fall = outline{{628,440,"c"},{632,428},{650,423},{668,428},{674,439,"c"},{650,442}, char="broken", seed=9}
path = {{520,690},{566,640},{612,572},{636,515},{650,480},{644,460},{610,446}}
town = {{800,452},{806,443},{812,443},{812,436},{816,432},{820,436},{820,443},{826,443},{829,424},{832,410},{835,424},{838,443},{858,443},{860,434},{870,434},{872,443},{888,443},{892,439},{896,443},{904,452}}
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
local mp = {} for x = 0, 1000, 25 do mp[#mp+1] = {x, mound(x)} end
h:sketch(mp, {pressure=0.3})
h:sketch(path, {pressure=0.28})
h:sketch({{770,112},{758,122},{756,138},{766,150}}, {pressure=0.3})
b = pencil("HB")
for _, l in ipairs(oak.limbs) do
  if l.order <= 2 and #l.pts >= 2 then b:line(l.pts, {pressure=(l.order == 0) and 0.6 or 0.45}) end
end
for _, o in ipairs({cap, up1, up2, up3, fall}) do for _, p in ipairs(o:paths()) do b:line(p, {pressure=0.55, smooth=false}) end end
b:line(town, {pressure=0.4, smooth=false})
b:line({{655,448},{653,458},{651,470}}, {pressure=0.5})
b:line({{659,448},{660,458},{661,470}}, {pressure=0.5})

--@ chunk 3 · clock 0
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "chrome yellow", "vermilion", "red earth", "raw umber"}
skyn = noise{seed=31, octaves=4, period=300, stretch={0.05, 5}}
sky = function(x, y)
  local t = clamp(y / HZ, 0, 1)
  local c = gradient({{0,"#44537a"},{0.28,"#6c7f9f"},{0.52,"#a4b0b0"},{0.72,"#d6cf9f"},{0.88,"#eac284"},{1,"#df9f73"}}, t + 0.03*skyn(x, y))
  local g = math.exp(-((x - 610)/360)^2)
  return shift(c, -0.07*(1 - g)*t, 0, -0.01*(1-g)*t)
end
skym = above(function(x) return mound(x) + 10 end)
work(skym, {hand="broad", color=sky, angle=function(x, y) return 0.02*skyn(x, y) end, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})

--@ chunk 4 · clock 0

stipple(skym, {width=3.2, color=sky, coverage=3.5, pressure={0.45, 0.8}, dips={20, 0.4, 0.6}, medium=0.45, pal=skypal})
blend(skym, {angle=0, coverage=2})

--@ chunk 5 · clock 0
local bars = {
  {pts={{-30,322},{80,316},{210,313},{330,318},{450,315},{540,321}}, w={2,7,11,6,9,1}},
  {pts={{440,356},{520,349},{640,347},{730,351},{860,346},{1030,342}}, w={1,5,9,12,7,4}},
  {pts={{150,384},{240,380},{330,382},{430,385}}, w={1,4,5,1}},
  {pts={{690,398},{780,392},{900,394},{1030,397}}, w={1,6,8,5}},
  {pts={{-30,412},{60,408},{170,410},{250,413}}, w={3,5,3,1}},
  {pts={{560,262},{640,258},{720,262}}, w={1,3,1}},
}
clouds = nil
for i, b in ipairs(bars) do
  local m = ribbon(b.pts, b.w):roughen(3.5, 22, 40+i, 2):soften(1.5)
  clouds = clouds and (clouds + m) or m
end
work(clouds, {hand="broad", color=function(x, y) return mix("#948597", "#b28c88", clamp((y-300)/110,0,1)) end,
  angle=function(x, y) return 0.03*skyn(x*3, y) end, coverage=2.2, medium=0.4, length={30, 90}, pal=skypal, clip=clouds:grow(2)})
local under = mask(function(x, y) return clamp(clouds:at(x, y) - clouds:at(x, y + 3), 0, 1) end):soften(1)
work(under, {hand="detail", tool="round 1.4", angle=0, length={12, 40}, coverage=1.4, broken=0.4,
  color=function(x, y) return mix("#e3a888", "#f0cc98", math.exp(-((x-610)/300)^2)) end, pal=skypal})
blend(clouds:grow(3), {angle=0, coverage=1.2})

--@ chunk 6 · clock 0
wait(24*60)
landpal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "red earth", "raw umber", "bone black", "vermilion"}
-- a far wood line on the left and a lower one beyond the town
woodL = outline{{-20,HZ+2},{30,HZ-7},{90,HZ-9},{150,HZ-5},{200,HZ-2},{230,HZ+2}, open=true, char="soft", lobe=7, seed=12}
woodR = outline{{670,HZ+2},{700,HZ-5},{760,HZ-6},{790,HZ-3},{800,HZ+1}, open=true, char="soft", lobe=6, seed=13}
woodF = outline{{905,HZ+2},{940,HZ-4},{1020,HZ-5}, open=true, char="soft", lobe=5, seed=14}
local woods = (woodL:below(HZ+6) + woodR:below(HZ+6) + woodF:below(HZ+6))
work(woods, {hand="body", color=function(x, y) return mix("#6f6878", "#7d6f78", math.exp(-((x-610)/300)^2)) end,
  angle=0, length={4, 14}, coverage=3, medium=0.25, pal=landpal, clip=woods})
ground = below(function(x) return mound(x) end):roughen(1.2, 14, 3, 0.8)
gn = noise{seed=17, octaves=5, period=110, stretch={0.0, 3}}
groundcol = function(x, y)
  local bump = clamp((HZ + 3 - mound(x)) / 14, 0, 1)
  local t = math.max(clamp((y - HZ)/238, 0, 1), 0.3 * bump)
  local c = gradient({{0,"#5e5660"},{0.08,"#4e4749"},{0.3,"#3d3834"},{1,"#2a2620"}}, t + 0.05*gn(x, y))
  return c
end
work(ground, {hand="body", color=groundcol, angle=function(x, y) return 0.06*gn(x, y) end, length={20, 70}, coverage=3.4, medium=0.2, pal=landpal, clip=ground})

--@ chunk 7 · clock 1440
wait(12*60)
bark = "#2b2624"
oakm = nil
local thin = {}
for i, l in ipairs(oak.limbs) do
  if #l.pts >= 2 then
    if l.w[1] >= 2.6 then
      local w = {} for k = 1, #l.w do w[k] = l.w[k] * 1.05 end
      local r = ribbon(l.pts, w)
      oakm = oakm and (oakm + r) or r
    else thin[#thin+1] = l end
  end
end
-- flare the foot into the mound
local fx, fy = oak.limbs[1].pts[1][1], oak.limbs[1].pts[1][2]
oakm = oakm + poly({{fx-26, fy+4}, {fx-12, fy-10}, {fx+12, fy-12}, {fx+28, fy+5}}, true)
oakm = oakm:roughen(0.9, 7, 21, 0.4)
local dir = noise{seed=9, period=30}
work(oakm, {hand="body", tool="filbert 3", color=function(x, y) return mix("#2a2523", "#352e2a", clamp((fy - y)/300, 0, 1)) end,
  angle=function(x, y) return 1.5708 + 0.5*dir(x, y) end, length={5, 16}, coverage=3.4, medium=0.15, pal=landpal, clip=oakm})
local limb, twig = brush("round", 1.8), brush("rigger", 0.8)
for i, l in ipairs(thin) do
  local b = (l.w[1] > 1.1) and limb or twig
  if i % 5 == 1 or b:fullness() < 0.3 then b:reload(bark, 0.9, {pal=landpal}) end
  b:stroke(l.pts, {pressure={clamp(0.35 + l.w[1]/3, 0.3, 1), 0.15}, ramps={0.03, 0.5}, shake=0.4})
end
print(#thin, "thin limbs")

--@ chunk 8 · clock 2160
work(oakm:shrink(0.6), {hand="body", tool="round 2", color="#2e2825", angle=1.5708, angle_jitter=0.6, length={3, 9}, coverage=2, medium=0.12, pal=landpal, clip=oakm})
-- side branches: zigzag, elbowed, forking into hooked twigs
local cx = oak.limbs[1].pts[1][1]
local br = brush("round", 2.6); local tw = brush("round", 1.3)
local nb = 0
local function jag(b, x, y, ang, len, depth, pr)
  local pts = {{x, y}}
  local segs = math.random(3, 5)
  local a = ang
  for s = 1, segs do
    a = a + randn(0, 0.45)
    if math.random() < 0.25 then a = a + (math.random() < 0.5 and -0.8 or 0.8) end
    local sl = len / segs * rand(0.6, 1.4)
    x = x + sl * math.cos(a); y = y + sl * math.sin(a)
    pts[#pts+1] = {x, y}
  end
  if b:fullness() < 0.3 then b:reload(bark, 0.9, {pal=landpal}) end
  b:stroke(pts, {pressure={pr, pr*0.25}, ramps={0.02, 0.7}, shake=0.3})
  nb = nb + 1
  if depth > 0 then
    local k = math.random(1, 3)
    for j = 1, k do
      local p = pts[math.random(math.max(2, #pts - 2), #pts)]
      jag(tw, p[1], p[2], a + rand(-0.9, 0.9) - 0.25, len * rand(0.35, 0.6), depth - 1, pr * 0.7)
    end
  end
end
br:reload(bark, 0.9, {pal=landpal}); tw:reload(bark, 0.9, {pal=landpal})
for i, l in ipairs(oak.limbs) do
  if l.order >= 1 and l.order <= 2 and #l.pts >= 4 and l.w[1] > 1.5 then
    local n = math.max(2, math.floor(#l.pts / 3))
    for j = 1, n do
      local k = math.random(2, #l.pts - 1)
      local p = l.pts[k]
      if p[2] < 330 then
        local out = (p[1] < cx) and math.pi or 0
        local ang = out + (out == 0 and -1 or 1) * rand(0.2, 1.1)
        if math.random() < 0.3 then jag(br, p[1], p[2], ang, rand(5, 10), 0, 0.95) else jag(br, p[1], p[2], ang, rand(18, 44), 2, 0.8) end
      end
    end
  end
end
-- twig brushes at the tips
for _, t in ipairs(oak.tips) do
  for j = 1, math.random(2, 4) do jag(tw, t[1], t[2], -1.5708 + rand(-1.3, 1.3), rand(8, 18), 2, 0.6) end
end
print(nb, "branches and twigs")
