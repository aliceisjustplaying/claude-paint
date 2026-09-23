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
    a = a + randn(0, 0.18)
    if math.random() < 0.3 then a = a + (math.random() < 0.5 and -0.55 or 0.55) end
    a = a + 0.12 * (-1.5708 - a)
    local sl = len / segs * rand(0.6, 1.4)
    x = x + sl * math.cos(a); y = y + sl * math.sin(a)
    pts[#pts+1] = {x, y}
  end
  if b:fullness() < 0.3 then b:reload(bark, 0.9, {pal=landpal}) end
  local ns = #pts - 1
  for s = 1, ns do
    local p0 = pr * (1 - 0.7 * (s - 1) / ns)
    local p1 = pr * (1 - 0.7 * s / ns)
    b:stroke({pts[s], pts[s + 1]}, {pressure={p0, (s == ns) and 0.05 or p1}, ramps={0.0, (s == ns) and 0.6 or 0.0}, shake=0.2})
  end
  nb = nb + 1
  if depth > 0 then
    local k = math.random(1, 3)
    for j = 1, k do
      local p = pts[math.random(math.max(2, #pts - 2), #pts)]
      jag(tw, p[1], p[2], a + rand(-0.9, 0.9) - 0.25, len * rand(0.3, 0.5), depth - 1, pr * 0.7)
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
        if math.random() < 0.3 then jag(br, p[1], p[2], ang, rand(5, 10), 0, 0.95) else jag(br, p[1], p[2], ang, rand(14, 32), 2, 0.8) end
      end
    end
  end
end
-- twig brushes at the tips
for _, t in ipairs(oak.tips) do
  for j = 1, math.random(2, 4) do jag(tw, t[1], t[2], -1.5708 + rand(-1.3, 1.3), rand(5, 12), 2, 0.6) end
end
print(nb, "branches and twigs")

-- bark: fissures running up the trunk and the big limbs, and a hollow where a limb broke
local fis = brush("round", 0.9)
local trunkpts = oak.limbs[1].pts
for k = 1, 16 do
  local off = rand(-0.4, 0.4)
  local pts = {}
  local j0 = math.random(1, 8)
  for j = j0, math.min(#trunkpts, j0 + math.random(5, 12)) do
    local p = trunkpts[j]
    local w = oak.limbs[1].w[j]
    pts[#pts+1] = {p[1] + off * w + randn(0, 0.4), p[2]}
  end
  if #pts >= 2 then
    if k % 4 == 1 then fis:reload("#1c1817", 0.8, {pal=landpal}) end
    fis:stroke(pts, {pressure={0.5, 0.2}, ramps={0.2, 0.4}, shake=0.6})
  end
end
-- the glow catches the right edge of the trunk faintly
local rimm = mask(function(x, y) return clamp(oakm:at(x, y) - oakm:at(x + 2.2, y), 0, 1) end) * below(function() return 150 end)
work(rimm, {hand="detail", tool="round 0.9", color="#5a4a44", angle=1.5708, length={3, 10}, coverage=1.2, broken=0.5, pal=landpal, clip=oakm})

--@ chunk 9 · clock 2160
cap = outline{{497,396,"c"},{503,381},{519,370,"c"},{553,363},{588,366},{611,374,"c"},{626,389,"c"},{612,399},{575,403,"c"},{540,401},{515,402,"c"}, char="broken", seed=4}
up1 = outline{{510,400,"c"},{528,399,"c"},{534,411},{532,423,"c"},{509,422,"c"},{506,410}, char="broken", seed=5}
up2 = outline{{556,402,"c"},{566,402},{569,426,"c"},{555,428,"c"},{553,414}, char="broken", seed=6}
up3 = outline{{588,400,"c"},{607,399,"c"},{615,414},{612,437,"c"},{592,438,"c"},{586,418}, char="broken", seed=7}
fall = outline{{627,442,"c"},{636,430},{656,425,"c"},{671,431},{677,442,"c"}, char="broken", seed=9}
stones = {cap, up1, up2, up3, fall}
local sn = noise{seed=44, octaves=5, period=14}
for i, o in ipairs(stones) do
  local m = o:mask()
  local base = (o == up2) and "#262221" or "#312d2b"
  work(m, {hand="body", tool="filbert 3", color=function(x, y) return mix(base, "#3d3734", 0.7*sn:at01(x, y)) end,
    angle=function(x, y) return 0.4 + 1.2*sn(x, y) end, length={3, 9}, coverage=3.4, medium=0.15, pal=landpal, clip=m})
end
local m = cap:mask()
local top = mask(function(x, y) return clamp(m:at(x, y) - m:at(x, y - 4), 0, 1) end):soften(0.6) * m * below(function(x) return 360 end) * above(function(x) return 392 end)
work(top, {hand="detail", tool="round 1.2", color=function(x, y) return mix("#4b4850", "#5d5862", sn:at01(x, y)) end,
  angle=0.08, angle_jitter=0.5, length={4, 10}, coverage=1.6, medium=0.2, pal=landpal, clip=m, broken=0.5})

--@ chunk 10 · clock 2160
local tp = {}
for _, p in ipairs(town) do tp[#tp+1] = {p[1], p[2]} end
tp[#tp+1] = {904, HZ + 3}; tp[#tp+1] = {800, HZ + 3}
townm = poly(tp):roughen(0.4, 4, 3, 0.3)
work(townm, {hand="detail", tool="round 1.2", color="#5d5462", angle=1.5708, length={2, 6}, coverage=3, medium=0.2, pal=landpal, clip=townm})
-- the crescent: lit toward the sun, down and to the left
MX, MY, MR = 762, 128, 8.5
local disc = ellipse(MX, MY, MR, MR)
moon = (disc - ellipse(MX + 3.6, MY - 2.4, MR * 0.93, MR * 0.93)):soften(0.35)
work(moon, {hand="detail", tool="round 1", color="#f4ecd2", angle=2.3, length={2, 5}, coverage=3.5, medium=0.2, pal=skypal, clip=moon:grow(0.4)})

--@ chunk 11 · clock 2160
wait(10*60)
trail = {{410,720},{470,660},{560,610},{628,568},{640,530},{616,500},{604,480},{622,464},{612,450},{590,442}}
local tw = {150, 118, 84, 56, 36, 24, 15, 9, 5, 3}
pathm = ribbon(trail, tw):roughen(3, 18, 71, 2.5)
local pn = noise{seed=72, octaves=4, period=24, stretch={0.9, 3}}
work(pathm, {hand="body", tool="filbert 4", color=function(x, y)
    local t = clamp((y - 440)/260, 0, 1)
    return mix(mix("#5f5753", "#3b352f", t), "#2c2823", 0.45*pn:at01(x, y)) end,
  angle=function(x, y) return -0.7 + 0.35*pn(x, y) end, length={6, 22}, coverage=3.2, medium=0.18, pal=landpal, clip=pathm})
for k, off in ipairs({-0.22, 0.2}) do
  local rp = {}
  for i = 1, 6 do local p = trail[i]; rp[i] = {p[1] + off*tw[i] + randn(0, 2), p[2] + randn(0, 2)} end
  local rm = ribbon(rp, {9, 7, 5, 3.4, 2.2, 1.2}):roughen(1.6, 10, 80 + k, 1.2) * pathm
  work(rm, {hand="detail", tool="round 2.5", color_over={shift={-0.035, 0, -0.006}}, angle=-0.7, length={6, 18}, coverage=1.6, broken=0.4, pal=landpal})
end

--@ chunk 12 · clock 2760
wait(6*60)
hn = noise{seed=91, octaves=4, period=40}
hn2 = noise{seed=92, octaves=3, period=120}
heathcol = function(x, y)
  local t = clamp((y - HZ)/238, 0, 1)
  local a = mix("#2d2624", "#3e3629", hn:at01(x, y))
  a = mix(a, "#46362f", 0.5*hn2:at01(x, y))
  return mix(mix(a, "#554c52", 0.55*(1 - t)^3), "#221e1b", 0.3*t)
end
local notpath = -(pathm:shrink(6))
local turn = noise{seed=93, period=10}
local far = (ground * above(function(x) return 530 end)) * notpath
work(far, {hand="hatch", tool="round 1.4", length={2, 5}, coverage=2.4, color=heathcol,
  angle=function(x, y) return -1.5708 + 0.6*turn(x, y) end, angle_jitter=0.5, medium=0.15, pal=landpal})
local near = (ground * below(function(x) return 520 end)) * notpath
work(near, {hand="hatch", tool="round 2.2", length={4, 11}, coverage=2.6, color=heathcol,
  angle=function(x, y) return -1.5708 + 0.7*turn(x, y) end, angle_jitter=0.5, medium=0.15, pal=landpal})

--@ chunk 13 · clock 3120
wait(6*60)
rockA = outline{{58,668,"c"},{66,634},{92,612,"c"},{130,604},{168,614,"c"},{190,640},{196,668,"c"},{130,676}, char="broken", seed=101}
rockB = outline{{182,672,"c"},{190,652},{214,644,"c"},{240,650},{252,672,"c"}, char="broken", seed=102}
rockC = outline{{818,622,"c"},{826,606},{848,600,"c"},{866,608},{872,624,"c"}, char="broken", seed=103}
local rn = noise{seed=104, octaves=5, period=12}
for _, o in ipairs({{rockA, 604, 72}, {rockB, 644, 28}, {rockC, 600, 24}}) do
  local m = o[1]:mask()
  local top, h = o[2], o[3]
  local col = function(x, y)
    local u = smoothstep(0, 1, (y - top) / (h * 0.8) + 0.25*rn(x, y))
    return mix(mix("#57535a", "#4a4546", rn:at01(x*2, y)), mix("#262220", "#302a27", rn:at01(x, y)), u)
  end
  work(m, {hand="body", tool="filbert 4", color=col, angle=function(x, y) return 0.2 + 0.9*rn(x, y) end,
    length={4, 12}, coverage=3.6, medium=0.15, pal=landpal, clip=m})
  work(m:shrink(1.5), {hand="detail", tool="round 1.4", color_over={shift={-0.04, 0, -0.005}}, angle=function(x, y) return 1.2 + rn(x, y) end,
    length={3, 8}, coverage=0.8, broken=0.5, pal=landpal, clip=m})
end
-- lichen flecks on the big stone
stipple(rockA:mask():shrink(3) * above(function(x) return 640 end), {width=1.4, color="#57584a", coverage=0.35, pressure={0.3, 0.6}, fade=0, aim=false, pal=landpal})
-- junipers on the right flat, dark columns
junipers = {}
for i, j in ipairs({{722,470,26,9},{738,474,16,7},{930,468,20,8}}) do
  local x, y, h, r = j[1], j[2], j[3], j[4]
  local o = outline{{x - r, y}, {x - r*0.8, y - h*0.5}, {x - r*0.3, y - h*0.95}, {x + r*0.2, y - h}, {x + r*0.7, y - h*0.55}, {x + r, y}, char="soft", lobe=3, seed=110 + i}
  junipers[i] = o
  local m = o:mask()
  work(m, {hand="hatch", tool="round 1.2", color=function(x, y) return mix("#23241f", "#2f2e28", rn:at01(x, y)) end,
    angle=-1.3, angle_jitter=0.7, length={2, 5}, coverage=3, medium=0.15, pal=landpal, clip=m:grow(0.8)})
end

--@ chunk 14 · clock 3480
wait(4*60)
local region = below(function(x) return HZ + 60 end) - pathm:shrink(10)
local tufts = sward{region=region, horizon=HZ, near=H, height=34, flowers=0, seed=14, thin=0.55,
  wind={lean=0.25, gust=0.25, period=180, seed=3}}
local g = brush("rigger", 0.8)
local cols = {"#2a2420", "#3a3128", "#332a26", "#4a3f33"}
local n = 0
for i, t in ipairs(tufts) do
  if i % 4 == 1 then g:reload(cols[1 + (i // 4) % #cols], 0.75, {pal=landpal}) end
  local p = clamp(0.25 + 0.55 * t.scale, 0.2, 0.95)
  for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}}); n = n + 1 end
end
-- pale tips catching the sky on the nearest tufts
local tip = brush("rigger", 0.6)
local k = 0
for i, t in ipairs(tufts) do
  if t.scale > 0.45 and i % 3 == 0 then
    if k % 8 == 0 then tip:reload(mix("#7c7068", "#8f8278", rand()), 0.6, {pal=landpal}) end
    local bl = t.blades[1 + (i % #t.blades)]
    local a, b, c = bl[1], bl[2], bl[3]
    tip:stroke({{lerp(a[1], c[1], 0.55), lerp(a[2], c[2], 0.55)}, b, c}, {pressure={0.35, 0.0}, ramps={0.1, 0.8}})
    k = k + 1
  end
end
-- grass and heather along the mound's crest against the sky
local cg = brush("rigger", 0.5)
cg:reload("#2c2624", 0.8, {pal=landpal})
local m = 0
for x = 150, 700, 1.3 do
  local y = mound(x) + rand(0.5, 2.5)
  if y < HZ - 1 and rand() < 0.55 then
    local h = rand(1.5, 5.5)
    local lean = randn(0.15, 0.35)
    if m % 25 == 0 then cg:reload("#2c2624", 0.8, {pal=landpal}) end
    cg:stroke({{x, y}, {x + lean*h*0.4, y - h*0.6}, {x + lean*h, y - h}}, {pressure={0.55, 0.0}, ramps={0.05, 0.7}})
    m = m + 1
  end
end
print(#tufts, "tufts", n, "blades", k, "tips", m, "crest blades")

--@ chunk 15 · clock 3720
-- the wanderer on the crest, from behind, in a long coat and a low cap, with a staff
FX = 292; FY = mound(FX) + 2
local function P(dx, dy) return {FX + dx, FY + dy} end
man = body_of{spine={P(0,-2.5), P(0.2,-8), P(0.4,-14), P(0.5,-17.4), P(0.6,-19.6)}, widths={7.0,5.8,6.2,2.2,2.9},
  limbs={{P(-1.4,-3), P(-1.6,0.4), widths={1.4,1.2}}, {P(1.3,-3), P(1.8,0.3), widths={1.4,1.2}},
         {P(2.6,-14.5), P(3.8,-10.5), P(4.3,-7.5), widths={1.7,1.4,1.1}}}, char="firm", seed=121}
man_m = man:mask() + ellipse(FX + 0.6, FY - 21.1, 2.3, 0.85)
work(man_m, {hand="detail", tool="round 1", color="#1f1b1b", angle=1.5708, length={1, 4}, coverage=4, medium=0.12, pal=landpal, clip=man_m:grow(0.3)})
local st = brush("round", 0.7); st:load("#1f1b1b", 0.9, {pal=landpal})
st:stroke({P(5.2, 0.5), P(5.0, -9), P(4.5, -19)}, {pressure={0.6, 0.45}})
-- two ravens going home
local rv = brush("round", 0.9)
for _, r in ipairs({{548,196,1.0,0.1},{571,183,0.8,-0.15}}) do
  local x, y, s, t = r[1], r[2], r[3], r[4]
  rv:reload("#1c1a1b", 0.9, {pal=landpal})
  rv:stroke({{x - 5*s, y - 2.2*s + t}, {x - 2.2*s, y - 0.6*s}, {x, y}}, {pressure={0.1, 0.7}, ramps={0.4, 0.1}})
  rv:stroke({{x, y}, {x + 2.4*s, y - 1.4*s}, {x + 5.2*s, y - 1.8*s - t}}, {pressure={0.7, 0.1}, ramps={0.1, 0.4}})
  rv:touch(x, y + 0.2, {pressure=0.6})
end

--@ chunk 16 · clock 3720
local vig = mask(function(x, y)
  local fg = smoothstep(HZ + 20, H + 40, y)
  local ex = math.max(0, math.abs(x - 560) / 560 - 0.5) / 0.5
  local ey = math.max(0, (120 - y) / 120)
  return clamp(0.6 * fg + 0.25 * ex^2 * smoothstep(HZ - 80, HZ + 60, y) + 0.12 * ey^2 + 0.08 * ex^2, 0, 1)
end)
glaze(vig, {color="#2e2622", coats=0.4})

--@ chunk 17 · clock 48492.05859375
local fgn = noise{seed=131, octaves=3, period=60}
local busy = pathm:grow(2) + rockA:mask():grow(3) + rockB:mask():grow(3) + rockC:mask():grow(3)
-- tufts of dry grass: fine upturning strokes, a few blades lit by the sky
local g = brush("rigger", 0.7)
local tuftn = 0
local centers = {{90,590},{330,600},{420,660},{700,640},{860,680},{950,600},{40,690},{250,690}}
for i = 1, 70 do
  local c = centers[1 + i % #centers]
  local x, y = c[1] + randn(0, 38), c[2] + randn(0, 16)
  if busy:at(x, y) < 0.1 then
    local s = 0.55 + (y - 540) / 170
    local nb = math.random(7, 16)
    for k = 1, nb do
      local lit = math.random() < 0.15
      if k == 1 or k % 5 == 0 then g:reload(lit and mix("#4a413b", "#5c5148", rand()) or "#2a2320", 0.6, {pal=landpal}) end
      local h = rand(9, 26) * s
      local a = -1.5708 + randn(0, 0.32) + 0.12
      local bx = x + randn(0, 2.2 * s)
      local bend = randn(0, 0.25)
      g:stroke({{bx, y}, {bx + math.cos(a) * h * 0.5, y + math.sin(a) * h * 0.5},
        {bx + math.cos(a + bend) * h, y + math.sin(a + bend) * h}}, {pressure={clamp(0.45 * s, 0.3, 0.8), 0}, ramps={0.05, 0.75}})
    end
    tuftn = tuftn + 1
  end
end
-- pebbles along the track
local pb = brush("round", 1.5)
for i = 1, 60 do
  local t = rand(0.05, 0.75)
  local j = 1 + math.floor(t * (#trail - 1))
  local p, q = trail[j], trail[j + 1]
  local u = t * (#trail - 1) - (j - 1)
  local x, y = lerp(p[1], q[1], u), lerp(p[2], q[2], u)
  local side = (math.random() < 0.5) and -1 or 1
  local w = lerp(({150,118,84,56,36,24,15,9,5,3})[j], ({150,118,84,56,36,24,15,9,5,3})[j+1], u)
  x = x + side * w * rand(0.3, 0.55)
  if i % 6 == 1 then pb:reload(mix("#5d5650", "#77706a", rand()), 0.7, {pal=landpal}) end
  pb:touch(x, y, {pressure=clamp((y - 440) / 400, 0.15, 0.6), drag={1, 0}})
end
-- a fallen oak branch in the grass
local fb = brush("round", 3.6); fb:load("#453d38", 0.9, {pal=landpal})
fb:stroke({{246,646},{272,640},{300,637},{334,628}}, {pressure={0.9, 0.35}, shake=0.4})
local ft = brush("round", 1.2); ft:load("#3f3833", 0.9, {pal=landpal})
ft:stroke({{272,640},{280,630},{286,626}}, {pressure={0.6, 0.05}})
ft:stroke({{300,637},{312,642},{318,641}}, {pressure={0.5, 0.05}})
ft:stroke({{318,632},{322,622},{330,618}}, {pressure={0.5, 0.05}})
local fl = brush("round", 0.9); fl:load("#5d554e", 0.6, {pal=landpal})
fl:stroke({{258,641.4},{280,637.0},{306,633.4},{330,625.5}}, {pressure={0.4, 0.2}})
print(tuftn, "tufts")
local fd = brush("round", 1.2); fd:load("#1c1716", 0.8, {pal=landpal})
fd:stroke({{250,647.6},{272,642},{300,639.2},{332,630.4}}, {pressure={0.6, 0.25}})

--@ chunk 18 · clock 48492.05859375
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
