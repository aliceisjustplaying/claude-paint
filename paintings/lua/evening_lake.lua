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
rest(16) print(drying(200, 205), drying(300, 330), drying(600, 300), drying(200, 520))
