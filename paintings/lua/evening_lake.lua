-- easel session "evening_lake": a painting replayed chunk by chunk.
--   easel run paintings/lua/evening_lake.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Evening at a Mountain Lake. Twilight after sunset, contre-jour: the afterglow low over the lake,
-- a thin waxing crescent above it, a far range in mist, a fir promontory from the left and its
-- reflection, a dark near shore with stones.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=47, hand=true}
skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth","chrome yellow","vermilion"}
G = function(x, c, s) return math.exp(-((x - c)/s)^2) end
HZ = 432                               -- the lake's far edge (eye level)
SUNX = 600                             -- the sun has set here
MOON = {676, 158}
rn = noise{seed=5, octaves=5, period=170}
rn2 = noise{seed=6, octaves=3, period=60}
crest = function(x)
  return HZ - 10 - 14*rn:at01(x, 0) - 3*rn2(x, 0) - 66*G(x, 790, 140) - 30*G(x, 960, 60) - 20*G(x, 470, 110) - 8*G(x, 640, 40)
end
-- the promontory: a fir wood's skyline falling from the upper left to a low point at the water
PROM = {{-10,206},{36,188},{70,214},{112,222},{150,212},{186,252},{226,270},{262,318},{296,352},{318,398},{334,440},{346,458},{380,462},{420,466},{448,470}}
PWL = function(x) return 470 + 0.02*(448 - x) end         -- its waterline (nearer than the horizon)
-- the near shore: a dark bank along the bottom, rising a little to the right
SHORE = function(x) return 646 - 44*G(x, 800, 170) - 12*G(x, 620, 60) - 8*rn2(x, 300) + 10*G(x, 200, 160) end
water = mask(function(x, y) return (y > HZ and y < SHORE(x) + 4) and 1 or 0 end)
print(H, water:area())

--@ chunk 2 · clock 0
-- the drawing: a light 2H search, then 3B for what I am sure of; the lake's far edge ruled
local function line_of(f, x0, x1, step) local p = {} for x = x0, x1, step do p[#p+1] = {x, f(x)} end return p end
RANGE_PTS = line_of(crest, 0, 1000, 25)
SHORE_PTS = line_of(SHORE, 0, 1000, 25)
local h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.3})
h:sketch(RANGE_PTS, {pressure=0.3})
h:sketch(PROM, {pressure=0.3})
h:sketch(SHORE_PTS, {pressure=0.3})
local b = pencil("3B")
b:line(PROM, {pressure={0.55, 0.7, 0.65, 0.5}})
b:rule({448, HZ}, {1000, HZ}, {pressure=0.45})
b:line(RANGE_PTS, {pressure={0.4, 0.55, 0.5}})
b:line(SHORE_PTS, {pressure={0.5, 0.65, 0.5}})
-- the moon's place, a small circle
local m = {} for i = 0, 16 do local a = i/16*2*math.pi m[#m+1] = {MOON[1] + 9*math.cos(a), MOON[2] + 9*math.sin(a)} end
h:sketch(m, {pressure=0.25})
fix()
print(drawing_mask():area())

--@ chunk 3 · clock 2.4503677748143673
-- masks for the whole picture
local pl = {} for i, p in ipairs(PROM) do pl[i] = {p[1], p[2]} end
pl[#pl+1] = {452, PWL(452) + 1}; pl[#pl+1] = {-10, PWL(-10) + 1}
promM = poly(pl)                                              -- the promontory's land, skyline to waterline
promSky = function(x)                                        -- its skyline as a function (linear between points)
  if x <= PROM[1][1] then return PROM[1][2] end
  for i = 2, #PROM do local a, b = PROM[i-1], PROM[i]
    if x <= b[1] then return lerp(a[2], b[2], (x - a[1])/(b[1] - a[1])) end end
  return PWL(x)
end
promRefl = mask(function(x, y)                               -- the mirror of the promontory, shortened (we look down a little)
  if x > 452 then return 0 end
  local wl = PWL(x); local d = wl - promSky(x)
  return (y > wl and y < wl + 0.78*d) and 1 or 0 end):soften(1.5)
rangeM = mask(function(x, y) return (y > crest(x) and y < HZ + 2) and 1 or 0 end) - promM
rangeRefl = mask(function(x, y) return (y >= HZ and y < HZ + 0.7*(HZ - crest(x))) and 1 or 0 end) - promM - promRefl
shoreM = below(SHORE)
skyM = above(crest) - promM
waterM = mask(function(x, y) return (y > HZ and y < SHORE(x) + 6) and 1 or 0 end) - promM
print(promM:area(), promRefl:area(), rangeM:area(), shoreM:area(), skyM:area(), waterM:area())
-- the dead color: thin, lean, one sitting; values only a little lighter than meant, cool in the sky
local dead = pal:only{"lead white","raw umber","bone black","yellow ochre","cobalt blue"}
work(skyM, {hand="body", tool="filbert 10", color=function(x, y)
    return mix(mix("#6f757f", "#a9a293", smoothstep(80, HZ, y)), "#bca98c", 0.6*G(x, SUNX, 260)*smoothstep(200, HZ, y)) end,
  angle=0, angle_jitter=0.15, length={40, 120}, coverage=3.2, medium=0.4, load=0.6, pal=dead, clip=skyM:grow(3)})
work(waterM - promRefl, {hand="body", tool="filbert 10", color=function(x, y)
    return mix(mix("#a49d8f", "#6f727a", smoothstep(HZ, 640, y)), "#b3a48b", 0.5*G(x, SUNX, 200)*(1 - smoothstep(HZ, 600, y))) end,
  angle=0, angle_jitter=0.08, length={40, 120}, coverage=3.2, medium=0.4, load=0.6, pal=dead, clip=(waterM - promRefl):grow(3)})
work(rangeM + rangeRefl, {hand="body", tool="filbert 8", color="#6e6c70", angle=0, length={30, 90}, coverage=3, medium=0.4, load=0.6, pal=dead, clip=(rangeM + rangeRefl):grow(1.5)})
work(promM + promRefl, {hand="body", tool="filbert 8", color="#3a3a35", angle=1.3, angle_jitter=0.6, length={20, 60}, coverage=3.2, medium=0.4, load=0.6, pal=dead, clip=(promM + promRefl):grow(1.5)})
work(shoreM, {hand="body", tool="filbert 10", color="#3e3934", angle=0.05, angle_jitter=0.3, length={30, 90}, coverage=3.2, medium=0.4, load=0.6, pal=dead, clip=shoreM:grow(2)})
local t = timesheet() print(string.format("%.0f min this sitting", t.sitting))

--@ chunk 4 · clock 185.17565667256713
print(dry())

--@ chunk 5 · clock 30339.550656672567
-- sitting 2. The darks of the distance first: the far range as one hazed mass, lighter and warmer toward
-- its misty foot, and its reflection pulled down under it, a little darker. The sky comes to it next, wet.
rangecol = function(x, y)
  local t = smoothstep(crest(x) + 4, HZ, y)
  local c = mix("#5d6072", "#7d7a80", t)                      -- blue-violet top, grayer and lighter at the foot
  return mix(c, "#8a7f7c", 0.25*G(x, SUNX, 200))              -- warmer where it stands in the glow
end
rangereflcol = function(x, y)
  local d = y - HZ
  return shift(rangecol(x, HZ - d/0.7), -0.03, 0, -0.01)
end
work(rangeM, {hand="body", tool="filbert 6", color=rangecol, angle=0.04, angle_jitter=0.12, length={24, 80},
  coverage=3.0, medium=0.3, load=0.8, clip=rangeM, pal=skypal})
work(rangeRefl, {hand="body", tool="filbert 6", color=rangereflcol, angle=math.pi/2, angle_jitter=0.06, length={8, 24},
  coverage=2.8, medium=0.3, load=0.8, pal=skypal, clip=rangeRefl:grow(1)})
local t = timesheet() print(string.format("%.0f min this sitting", t.sitting))

--@ chunk 6 · clock 30389.59835069999
-- the sky, same sitting: unequal horizontal bands of thin paint brought down over the wet range top
-- (it overlaps the crest by 3 units) and 14 units into the promontory, where the firs will stand over it.
gn = noise{seed=17, octaves=3, period=260}
glow = function(x, y)
  local gx = math.exp(-((x - SUNX)/(300 + 70*gn(x, 0)))^2)
  return gx * smoothstep(HZ - 330, HZ - 20, y)
end
skycol = function(x, y)
  local t = clamp(y / HZ, 0, 1)
  local c = gradient({{0,"#47536c"},{0.25,"#5f6a82"},{0.5,"#8b8d9c"},{0.66,"#a8999c"},{0.8,"#c6b39e"},{1,"#d2bd98"}}, t)
  return mix(c, "#ecd49e", 0.6*glow(x, y))
end
sn = noise{seed=31, octaves=3, period=190, stretch={0, 7}}
skyB = function(x, y) local v = sn(x, y) return shift(skycol(x, y), 0.02*v, 0.003*v, 0.008*v) end
skyS = mask(function(x, y) return y < crest(x) + 3 and 1 or 0 end) - promM:shrink(14)
work(skyS, {hand="broad", color=skyB, angle=0, angle_jitter=0.02, length={120, 320}, coverage=3.8, medium=0.3, load=0.75,
  pal=skypal, clip=skyS:grow(2):blur(1.5)})
blend(skyS, {angle=0, coverage=1.2, length={150, 400}, clip=skyS:grow(1):blur(1)})
local t = timesheet() print(string.format("%.0f min this sitting", t.sitting))

--@ chunk 7 · clock 30412.46690076217
-- the cloud bank into the open sky: one long low bank above the glow that thins and parts over it,
-- and one thin high wisp; unclipped, few long strokes; then warm lights on the bank's underside
-- toward the glow, and its top lost into the sky.
bn = noise{seed=9, octaves=5, period=240, stretch={0, 5}}
bn2 = noise{seed=23, octaves=4, period=120, stretch={0, 4}}
band = function(y, c, th) return 1 - smoothstep(0.5*th, th, math.abs(y - c)) end
c_low = function(x) return 300 + 12*bn(x, 90) - 0.05*(x - 600) end
c_high = function(x) return 196 + 16*bn(x*0.6, 0) + 0.03*(x - 600) end
bankLow = mask(function(x, y)
  local th = (26 + 14*bn2(x, 300)) * (1 - 0.8*G(x, SUNX + 30, 70)) * smoothstep(320, 440, x)
  return band(y, c_low(x) + 6*bn2(x, y*0.5), th) * (0.55 + 0.45*smoothstep(-0.5, 0.1, bn2(x*1.3, 500)))
end):blur(1.5) - promM
bankHigh = mask(function(x, y)
  local th = 9 * smoothstep(-0.1, 0.4, bn2(x, 40)) * smoothstep(440, 560, x)
  return band(y, c_high(x), th)
end):blur(1.5)
cloudcol = function(x, y)
  local high = y < 250
  local body = high and "#6a6c80" or "#6f6a7a"
  local under = high and "#a58f92" or "#cdb096"
  local c = high and c_high(x) or c_low(x)
  local lit = smoothstep(c - 4, c + 16, y) * (0.3 + 0.7*G(x, SUNX, 300))
  return mix(body, under, lit)
end
cm = (bankLow + bankHigh):blur(3)
work(cm, {hand="broad", color=cloudcol, hug=false, angle=-0.03, angle_jitter=0.02, curve={0, 0}, length={90, 260},
  coverage=2.2, medium=0.32, load=0.9, pressure={0.45, 0.7}, ramps={0.25, 0.35}, pal=skypal})
local pn = noise{seed=44, octaves=2, period=150, stretch={0, 3}}
local low = bankLow:blur(3)
lightm = ((low:grow(4) - low:shrink(10)) * mask(function(x, y)
  return smoothstep(c_low(x) - 2, c_low(x) + 8, y) * G(x, SUNX, 360) * smoothstep(-0.2, 0.2, pn(x, 0))
end)):blur(2)
work(lightm, {hand="broad", color=function(x, y) return mix("#bfa99c", "#e2c9a0", G(x, SUNX, 200)) end,
  hug=false, angle=-0.03, length={60, 170}, coverage=1.2, medium=0.3, load=0.9, pressure={0.35, 0.5}, ramps={0.35, 0.4}, pal=skypal})
topsm = (cm:grow(6) * mask(function(x, y)
  local c = y > 250 and c_low(x) or c_high(x)
  return 1 - smoothstep(c - 2, c + 8, y) end)):blur(5)
blend(topsm, {angle=0, coverage=2.4, length={60, 200}, hug=false})
local t = timesheet() print(string.format("%.0f min this sitting; bank %.0f lights %.0f", t.sitting, cm:area(), lightm:area()))

--@ chunk 8 · clock 30415.344495218247
-- the firs of the promontory (geometry only): spires standing along the wooded skyline, poking above it by
-- varying amounts, smaller toward the spit; a lopsided few, one tall one, one broken top
local xs = uneven(15, 4, 344, 0.6, 0.5, 11)
FIRS = {}
for i, x in ipairs(xs) do
  local sky = promSky(x)
  local slope = smoothstep(0, 344, x)
  local h = lerp(150, 58, slope) * rand(0.75, 1.2)
  if i == 5 then h = h * 1.35 end
  local poke = rand(14, 48) * lerp(1, 0.55, slope)
  local top = sky - poke
  local habit = (i == 9 or i == 3) and "old" or "spire"
  local foot = math.min(top + h, PWL(x) - 6); h = foot - top
  local f = fir{x=x + rand(-4, 4), y=foot, height=h, width=h * rand(0.2, 0.3), habit=habit, seed=100 + i}
  FIRS[#FIRS + 1] = f
end
firM = FIRS[1]:mask()
for i = 2, #FIRS do firM = firM + FIRS[i]:mask() end
woodTop = outline{pts=PROM, open=true, char="soft", lobe=12, amount=0.8, seed=31}
woodM = woodTop:below(H) * mask(function(x, y) return y < PWL(x) + 1 and 1 or 0 end)
landM = woodM + firM                                         -- everything of the promontory above the water
-- its mirror in the lake about the waterline, shortened (we look down a little)
reflM = mask(function(x, y)
  if x > 460 then return 0 end
  local wl = PWL(x); if y <= wl then return 0 end
  return landM:at(x, wl - (y - wl)/0.78) end):soften(1)
print(#FIRS, firM:area(), woodM:area(), reflM:area())

-- the lake, same sitting (the promontory's firs grown first as geometry, for their reflection). 1. the promontory's reflection as one dark mass pulled DOWN (the wood mirrored:
-- darkest at the waterline, a little cooler and lighter toward its far end)
promReflCol = function(x, y)
  local d = (y - PWL(x)) / 230
  return mix("#22251f", "#343a3c", smoothstep(0.1, 1, d))
end
work(reflM, {hand="body", tool="filbert 6", color=promReflCol, angle=math.pi/2, angle_jitter=0.05, length={12, 40},
  coverage=2.8, medium=0.3, load=0.85, clip=reflM:grow(2)})
-- 2. the open water around the reflections: the sky mirrored (the bank too), darker toward me, in long level
-- strokes, fenced softly by its own region grown a few units
wn = noise{seed=41, octaves=3, period=200, stretch={0, 8}}
waterCol = function(x, y)
  local d = y - HZ
  local ym = HZ - 1.05*d - 3
  local c = skyB(x, math.max(ym, 5))
  local k = cm:at(x, clamp(ym, 1, H - 1))
  if k > 0.02 then c = mix(c, cloudcol(x, ym), 0.7*k) end
  c = mix(c, "#434856", 0.5*smoothstep(HZ + 20, 660, y))
  local v = wn(x, y)
  return shift(c, 0.012*v, 0, 0.004*v)
end
openW = waterM - reflM - rangeRefl
work(openW, {hand="broad", color=waterCol, angle=0, angle_jitter=0.004, curve={0, 0}, length={120, 320},
  coverage=3.8, medium=0.3, load=0.75, pal=skypal, clip=openW:grow(3):blur(2)})
-- 3. fuse the reflections level, fenced to the water so the promontory stays
blend((reflM:grow(8) + rangeRefl:grow(4)) * waterM, {angle=0, angle_jitter=0.004, length={40, 160}, coverage=2.4, clip=waterM})
local t = timesheet() print(string.format("%.0f min this sitting", t.sitting))

--@ chunk 9 · clock 30467.17885844037
-- the range's mirror was as dark and as tall as the range: restate it lighter, horizontal loaded strokes of its
-- color mixed toward the water's (more toward its far end), laid into the wet reflection
rrD = function(x) return math.max(1, 0.7*(HZ - crest(x))) end
work(rangeRefl, {hand="body", tool="filbert 6", color=function(x, y)
    local t = clamp((y - HZ) / rrD(x), 0, 1)
    return mix(rangereflcol(x, y), waterCol(x, y), 0.3 + 0.45*t) end,
  angle=0, angle_jitter=0.01, curve={0, 0}, length={30, 110}, coverage=2.6, medium=0.3, load=0.9, pressure={0.35, 0.5},
  pal=skypal, clip=rangeRefl:grow(2):blur(1)})
blend(rangeRefl:grow(3), {angle=0, angle_jitter=0.004, length={60, 200}, coverage=1.8, clip=waterM - reflM})
-- one calm band of mist on the far edge, straddling the seam between the range and its mirror
mn = noise{seed=77, octaves=4, period=150, stretch={0, 5}}
mistD = function(x, y)
  local c = HZ - 2 + 3*mn(x, 0)
  local up, dn = 9 + 5*mn(x, 30) + 8*G(x, 470, 90), 5 + 3*mn(x, 50)
  return clamp(smoothstep(c - up, c - up*0.3, y) * (1 - smoothstep(c + dn*0.4, c + dn, y)) * (0.7 + 0.3*mn:at01(x*1.2, 0)), 0, 1)
end
mistM = mask(mistD) - landM:grow(1)
mistcol = function(x, y) return mix("#aeaaab", "#cdbfa6", G(x, SUNX, 230)) end
work(mistM, {hand="broad", color=mistcol, angle=0, angle_jitter=0.004, curve={0, 0}, length={120, 300}, coverage=2.6,
  medium=0.35, load=0.9, load_at=function(x, y) return mistD(x, y) end, pressure={0.3, 0.4}, ramps={0.3, 0.3},
  hug=false, pal=skypal, clip=(-landM):blur(1)})
blend(mistM:blur(3), {angle=0, angle_jitter=0.003, coverage=2.0, length={100, 300}, hug=false, clip=(-landM):blur(1)})
local t = timesheet() print(string.format("%.0f min this sitting, mist %.0f", t.sitting, mistM:area()))

--@ chunk 10 · clock 30477.39663292095
print(dry())

--@ chunk 11 · clock 69112.67397667095
-- sitting 3: the promontory over the dry sky. 1. the wood as one dark hatched mass, near-black green, a little
-- air toward its top edge; strokes turn slowly (a wood, not rows), the edge is the strokes' own fringe
local turn = noise{seed=12, period=40}
woodCol = function(x, y)
  local top = promSky(x)
  local air = 1 - smoothstep(top, top + 70, y)
  return mix(mix("#1c211d", "#262d27", smoothstep(PWL(x) - 120, PWL(x), y)), "#4a4f55", 0.28*air)
end
local spit = mask(function(x, y) return x > 330 and 1 or 0 end):blur(6)
wood = woodM - spit
work(wood, {hand="hatch", tool="round 1.8", length={3, 8}, coverage=2.8, clip=wood:grow(1.5), hug=false,
  angle=function(x, y) return 1.57 + 0.45*turn(x, y) end, angle_jitter=0.35, color=woodCol, medium=0.2})
-- 2. the spires: needles dark, a rigger for the stem and boughs, then the few shoots that catch the sky light
for i, f in ipairs(FIRS) do
  local nd = f:needles()
  local ax = f.foot[1]
  work(nd, {hand="hatch", tool="round 1.6", length={3, 7}, coverage=2.6, clip=nd,
    angle=function(x, y) return 1.57 + 0.5*clamp((ax - x)/12, -1, 1) end, color="#1b201c", medium=0.2})
end
local rb = brush("rigger", 1.0)
for i, f in ipairs(FIRS) do
  for j, b in ipairs(f.boughs) do
    if j % 6 == 1 or b.dead then rb:reload(b.dead and "#4a4a47" or "#1a1d1a", 0.8) end
    rb:stroke(b.pts, {pressure={0.7, 0.05}, ramps={0.05, 0.6}})
  end
end
for i, f in ipairs(FIRS) do
  local hb = brush("round", math.max(0.9, f.hatch))
  f:paint(hb, {color="#161b18", lit={0, 0.6}})
  f:paint(brush("round", math.max(0.8, f.hatch*0.8)), {color="#282e2d", lit={0.75, 1}, every=8})
end
local t = timesheet() print(string.format("%.0f min this sitting", t.sitting))

--@ chunk 12 · clock 69853.46864621714
-- the spit: a low grassy tongue running out from the wood's foot. First the water relaid over the smudged
-- drips under it (full load, laid lightly, level), then the spit, its reeds and its short mirror.
SPIT = outline{pts={{318,446},{336,455},{352,459},{372,462},{396,465},{420,467.5},{442,469.5},{456,471},
                    {440,472.5},{380,473},{318,474}}, char="soft", lobe=4, amount=0.6, seed=52}
spitM = SPIT:mask()
spitRefl = mask(function(x, y) local wl = 472
  if y <= wl then return 0 end return spitM:at(x, wl - (y - wl)/0.8) end):soften(0.8)
local fixW = mask(function(x, y)
  return smoothstep(338, 356, x) * (1 - smoothstep(470, 492, x)) * smoothstep(469, 472, y) * (1 - smoothstep(496, 508, y)) end) - reflM - spitM
work(fixW, {hand="body", tool="filbert 5", color=waterCol, angle=0, angle_jitter=0.004, curve={0, 0}, length={30, 90},
  coverage=3.0, medium=0.25, load=1.0, pressure={0.3, 0.45}, ramps={0.3, 0.3}, pal=skypal, clip=fixW:grow(1) * waterM})
work(spitM, {hand="body", tool="filbert 3", color=function(x, y)
    return mix("#3a3a32", "#22231e", smoothstep(460, 471, y)) end, angle=0.06, angle_jitter=0.1, length={8, 26},
  coverage=2.8, medium=0.2, load=0.9, clip=spitM:grow(0.6) - wood})
work(spitRefl, {hand="body", tool="filbert 3", color="#2c2e2a", angle=math.pi/2, length={3, 8}, coverage=2.4, medium=0.25, load=0.8,
  clip=spitRefl:grow(0.8)})
blend(spitRefl:grow(3), {angle=0, angle_jitter=0.004, length={20, 60}, coverage=1.6, clip=waterM - spitM})
-- reeds on the spit, a few upright flicks, darker near the wood, sparse toward the tip
local rr = brush("rigger", 0.6)
local n = 0
for x = 326, 440, 3.2 do
  if rand() < 0.75 - 0.5*smoothstep(360, 440, x) then
    local y0 = 458 + (x - 320)*0.095 + rand(0, 2)
    local h = rand(3, 9) * (1 - 0.5*smoothstep(360, 440, x))
    if n % 5 == 0 then rr:reload(rand() < 0.3 and "#5d5a4a" or "#23241f", 0.7) end
    rr:stroke({{x, y0}, {x + rand(-1, 1), y0 - h*0.6}, {x + rand(-2, 2.5), y0 - h}}, {pressure={0.5, 0}, ramps={0.05, 0.7}})
    n = n + 1
  end
end
-- small firs down the wood's steep right flank, so it isn't one cut edge
FLANK = {}
for i, p in ipairs({{314, 392, 46}, {326, 418, 36}, {333, 437, 26}, {344, 452, 17}}) do
  local f = fir{x=p[1], y=p[2] + p[3], height=p[3], width=p[3]*rand(0.22, 0.3), habit="spire", seed=300 + i}
  FLANK[#FLANK + 1] = f
  local nd = f:needles()
  work(nd, {hand="hatch", tool="round 1.2", length={2, 5}, coverage=2.6, clip=nd, angle=1.57, angle_jitter=0.4, color="#1b201c", medium=0.2})
  f:paint(brush("round", math.max(0.8, f.hatch)), {color="#171c19", lit={0, 1}})
end
print(n, spitM:area())

--@ chunk 13 · clock 69892.10989378765
-- a row of firs standing at the water in front of the wooded slope, so the slope doesn't end in a line of
-- spire bottoms: fir_wood along the waterline, two rows, dark, the back row a little aired
FRONTTOP = {{-10,352},{40,334},{80,360},{120,346},{165,372},{205,366},{245,392},{280,404},{310,430},{326,452}}
front = fir_wood{skyline=outline{pts=FRONTTOP, open=true, char="soft", lobe=12, seed=61},
  foot={{-10, PWL(-10) - 1}, {160, PWL(160) - 1}, {330, PWL(330) - 1}}, depth=2, count=11, horizon=HZ, seed=62}
print(front)
local air = "#5a6064"
local function row(r)
  local h, s, nd = front:haze(r), front:scale(r), front:needles(r)
  work(nd, {hand="hatch", tool=string.format("round %.1f", math.max(0.9, 1.7*s)), length={2, 6*s + 1}, coverage=2.5,
    clip=nd, angle=1.57, angle_jitter=0.5, color=mix("#1a1f1b", air, 0.5*h), medium=0.2})
  local rb = brush("rigger", math.max(0.6, 1.3*s))
  for n, f in ipairs(front:trees(r)) do
    if n % 5 == 1 then rb:reload(mix("#1c1a17", air, 0.5*h), 0.85) end
    rb:stroke(f.leader.pts, {pressure={0.9, 0.2}, ramps={0.02, 0.3}})
  end
  front:paint(brush("round", 1.9*s + 0.3), r, {color=mix("#141916", air, 0.5*h), lit={0, 0.7}})
  front:paint(brush("round", 1.5*s + 0.3), r, {color=mix("#262d2b", air, 0.4*h), lit={0.7, 1}, every=9})
end
row(2)
row(1)
local t = timesheet() print(string.format("%.0f min this sitting", t.sitting))

--@ chunk 14 · clock 70176.88759406283
rest(16)

--@ chunk 15 · clock 71136.88759406283
-- sitting 4: the near shore. The bank as one dark mass in level strokes, its lip catching a little sky light;
-- two stones on the water's edge laid into it wet so they sit in the bank, dark against the pale water,
-- lit only on the faces turned up to the sky.
shoreCol = function(x, y)
  local top = SHORE(x)
  return mix(mix("#3a3831", "#27251f", smoothstep(top, top + 18, y)), "#1d1c19", smoothstep(top + 30, H, y))
end
work(shoreM, {hand="body", tool="filbert 6", color=shoreCol, angle=0.04, angle_jitter=0.25, length={14, 50}, coverage=3.0,
  medium=0.2, load=0.85, clip=shoreM:grow(1.2)})
SKYLIGHT = {from={0.35, -1}, front=-0.2, ambient=0.35, bounce=0.2, bounce_from={0, 1, 0.3}}
S1 = outline{pts={{356,653,"c"},{360,641},{371,631,"c"},{398,623},{426,612,"c"},{445,610},{457,617,"c"},{467,634},{474,654,"c"},{420,657}},
  char="broken", seed=71}
S2 = outline{pts={{494,649,"c"},{497,640},{506,633,"c"},{521,631},{530,637,"c"},{535,650},{512,652}}, char="broken", seed=72}
stones = {}
for i, o in ipairs({S1, S2}) do
  local r = rock{outline=o, kind="granite", sun=SKYLIGHT, seed=80 + i,
    cracks=(i == 1) and {{{428,613},{431,630},{426,648}}} or nil}
  stones[i] = r
  local m, v = r:mask(), r:value()
  local lo, hi = r:levels(0.03, 0.97)
  local col = function(x, y)
    return gradient({{0, "#1f1e1b"}, {0.45, "#282724"}, {0.8, "#343432"}, {1, "#444548"}}, smoothstep(lo, hi, v:at(x, y)))
  end
  work(m, {hand="body", tool="filbert 4", color=col, angle=r:field("plane"), length={5, 14}, coverage=3.0, medium=0.2, load=0.9, clip=m})
  local rb = brush("round", 1.2)
  for j, s in ipairs(r.seams) do
    if j % 3 == 1 then rb:reload("#191816", 0.8) end
    rb:stroke(s.pts, {pressure={0.6, 0.15}, ramps={0.15, 0.45}, shake=0.5, clip=m})
  end
  blend(m, {angle=r:field("plane"), coverage=1.6, length={4, 12}, clip=m})
end
stoneM = stones[1]:mask() + stones[2]:mask()
print(shoreM:area(), stoneM:area(), stones[1])

--@ chunk 16 · clock 71209.39677127823
-- a man from behind on the rise, looking out over the lake toward the afterglow: a dark coat in short strokes,
-- (no rim light: the glow is too far off and low to catch him)
local fx, fy = 802, SHORE(802) + 1
FIG = body_of{spine={{fx - 0.3, fy - 55.4}, {fx, fy - 52.8}, {fx + 0.2, fy - 49.6}, {fx + 0.6, fy - 46.2}, {fx + 1.7, fy - 30}, {fx + 2.2, fy - 13}},
  widths={6.8, 5.0, 3.9, 10.8, 9.0, 13.2},
  limbs={{{fx - 1.6, fy - 14}, {fx - 2.6, fy}, widths={2.8, 2.0}}, {{fx + 4.6, fy - 14}, {fx + 5.4, fy + 0.5}, widths={2.8, 2.0}},
         {{fx - 4.2, fy - 45.2}, {fx - 5.8, fy - 37}, {fx - 5.0, fy - 28.5}, widths={3.0, 2.7, 2.2}},
         {{fx + 6.2, fy - 44.8}, {fx + 8.2, fy - 35}, {fx + 8.8, fy - 27.5}, widths={3.0, 2.7, 2.2}}},
  blend=0.4, char="firm", amount=1.1, seed=91}
figM = FIG:mask()
print(fx, fy, figM:area())
work(figM, {hand="detail", tool="round 1.6", color=function(x, y) return mix("#252321", "#1b1a19", smoothstep(560, 600, y)) end,
  angle=1.57, angle_jitter=0.3, length={2, 6}, coverage=5, medium=0.2, load=0.95, clip=figM:grow(0.3)})
-- his stick, from the right hand down to the ground beside him
local sb = brush("rigger", 0.9); sb:load("#1e1c1a", 0.9)
sb:stroke({{fx + 8.9, fy - 28}, {fx + 10.2, fy - 14}, {fx + 11.2, fy + 0.5}}, {pressure={0.8, 0.6}, ramps={0.05, 0.1}})

--@ chunk 17 · clock 71224.5979578495
-- grass on the near bank: rigger blades set down at the root and lifted off, in patches (a noise), taller
-- toward me, leaning with a slow noise; dark stalks, a few tips catching the sky. Denser along the bank's lip
-- so the line where it meets the water is broken, and around the man's feet and the stones' feet.
local patch = noise{seed=93, octaves=3, period=70}
local lean = noise{seed=94, octaves=2, period=180}
local g = brush("rigger", 0.8)
local n = 0
local cols = {"#1d1c18", "#26251f", "#2e2c24", "#1a1a17", "#34332b", "#43423a"}
for i = 1, 5200 do
  local x = rand(-5, 1005)
  local top = SHORE(x)
  local depth = rand()^1.6                                   -- most near the lip
  local y = top + 1 + depth*(H - top)
  local near = clamp((y - 600)/110, 0, 1)
  local lip = 1 - smoothstep(0, 14, y - top)
  local p = patch:at01(x, y) + 0.55*lip
  if p > 0.62 and stoneM:at(x, y - 2) < 0.5 then
    local h = (5 + 26*near) * rand(0.5, 1.5)
    local a = -1.57 + 0.35*lean(x, 0) + randn(0, 0.2)
    local curl = randn(0, 0.25)
    local tip = {x + h*math.cos(a + curl), y + h*math.sin(a + curl)}
    local mid = {x + 0.55*h*math.cos(a), y + 0.55*h*math.sin(a)}
    if n % 7 == 0 then
      local k = (rand() < 0.07) and (5 + (rand() < 0.5 and 1 or 0)) or math.random(1, 4)
      g:reload(cols[k], 0.7)
    end
    g:stroke({{x, y}, mid, tip}, {pressure={0.3 + 0.5*near, 0}, ramps={0.05, 0.75}})
    n = n + 1
  end
end
print(n)
local t = timesheet() print(string.format("%.0f min this sitting", t.sitting))

--@ chunk 18 · clock 71280.1509731412
-- the young moon, a thin crescent lit on the side toward the set sun (down and a little left), laid with short
-- touches clipped to its mask on the dry sky so the horns come to points
local mx, my, R = MOON[1], MOON[2], 8.5
local dx, dy = SUNX - mx, HZ - my
local l = math.sqrt(dx*dx + dy*dy); dx, dy = dx/l, dy/l
moonM = (ellipse(mx, my, R, R) - ellipse(mx - 3.4*dx, my - 3.4*dy, R*1.02, R*1.02)):soften(0.4)
work(moonM, {hand="detail", tool="round 1.2", color="#efe5c8", angle=math.atan(dy, dx) + math.pi/2, angle_jitter=0.3,
  length={2, 5}, coverage=4, medium=0.15, load=1.0, aim="masstone", clip=moonM})
print(moonM:area())

--@ chunk 19 · clock 71282.42082434893
-- the far crest still showed a double contour at 3200 (a dark line over a pale band). Now that it's dry, bring
-- the range up over both, about 5 units, loaded and laid lightly along the ridge, clipped softly at the new ridge.
print(drying(790, crest(790) - 3), drying(470, crest(470)))
crest2 = function(x) return crest(x) - 5.5 end
crestBand = mask(function(x, y) local c = crest2(x)
  return smoothstep(c - 1, c + 0.5, y) * (1 - smoothstep(c + 10, c + 16, y)) end) - landM:grow(3)
work(crestBand, {hand="body", tool="filbert 4", color=function(x, y) return rangecol(x, y + 5.5) end,
  angle=function(x, y) return math.atan(crest(x + 4) - crest(x - 4), 8) end,
  length={20, 60}, coverage=2.8, medium=0.2, load=1.0, pressure={0.35, 0.5}, ramps={0.3, 0.3}, pal=skypal,
  clip=mask(function(x, y) return smoothstep(crest2(x) - 0.7, crest2(x) + 0.7, y) end)})

--@ chunk 20 · clock 71293.12886172533
-- Friedrich's advice to Carus: a dark glaze over the whole picture except the moon, growing darker toward the
-- edges. A smooth elliptical falloff centered on the glow's reflection, transparent, light.
rest(16)
local vig = mask(function(x, y)
  local dx, dy = (x - 610)/640, (y - 440)/470
  return smoothstep(0.45, 1.25, math.sqrt(dx*dx + dy*dy)) end) * (-moonM)
glaze(vig, {color="#22232b", coats=0.32, pigment="transparent"})

--@ chunk 21 · clock 102172.06083780527
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
