-- easel session "frozen_pond": a painting replayed chunk by chunk.
--   easel run paintings/lua/frozen_pond.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0
-- the design, as globals: horizon, banks, pond; then the drawing
HZ = 468
-- far hills: a low, uneven line
hn = noise{seed=41, octaves=4, period=180}
function hills(x) return HZ - 6 - 10*hn:at01(x, 0) - 14*math.exp(-((x-820)/150)^2) - 6*math.exp(-((x-140)/110)^2) end
-- the far bank's top edge (snow) and the pond's far shore
function farbank(x) return HZ + 4 + 3*hn(x*2.1, 7) end
function shore(x) return HZ + 13 + 4*hn(x*1.7, 19) end
-- the near bank's top (where the snow starts in front)
bn = noise{seed=43, octaves=4, period=220}
function bank(x) return 566 - 28*math.exp(-((x-560)/210)^2) + 26*((x-500)/500)^2 + 6*bn(x, 0) end
CROSS = {x=572, foot=bank(572)+4, top=bank(572)-118}
FIG = {x=522, foot=bank(522)+6}
WILLOWS = {{128, 0.9}, {196, 1.05}, {262, 0.8}, {330, 0.95}}
OAK = {x=792, foot=farbank(792)+3}
MOON = {x=318, y=160, r=10}

local h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.22})
local pts = {} for x = -5, 1005, 20 do pts[#pts+1] = {x, hills(x)} end
h:sketch(pts, {pressure=0.25})
pts = {} for x = -5, 1005, 25 do pts[#pts+1] = {x, shore(x)} end
h:sketch(pts, {pressure=0.25})
pts = {} for x = -5, 1005, 25 do pts[#pts+1] = {x, bank(x)} end
h:sketch(pts, {pressure=0.3})
-- the cross: an upright and a short beam, against the ruler, leaning a little
local b = pencil("HB")
b:rule({CROSS.x, CROSS.foot}, {CROSS.x + 4, CROSS.top}, {pressure=0.45})
b:rule({CROSS.x - 22, CROSS.top + 26}, {CROSS.x + 25, CROSS.top + 24}, {pressure=0.45})
-- the figure, a cloaked woman seen from behind
h:sketch({{FIG.x-8, FIG.foot}, {FIG.x-6, FIG.foot-30}, {FIG.x-4, FIG.foot-44}, {FIG.x, FIG.foot-50}, {FIG.x+4, FIG.foot-44}, {FIG.x+6, FIG.foot-30}, {FIG.x+9, FIG.foot}}, {pressure=0.35})
-- willows: short trunks and heads
for _, w in ipairs(WILLOWS) do
  local fx, fy, s = w[1], farbank(w[1]) + 2, w[2]
  h:sketch({{fx, fy}, {fx+1, fy - 40*s}}, {pressure=0.3})
  h:sketch({{fx-24*s, fy-70*s}, {fx-10*s, fy-95*s}, {fx+12*s, fy-98*s}, {fx+26*s, fy-72*s}}, {pressure=0.2})
end
-- the oak: trunk and the main limbs
h:sketch({{OAK.x, OAK.foot}, {OAK.x-2, OAK.foot-60}, {OAK.x-10, OAK.foot-110}, {OAK.x-40, OAK.foot-160}}, {pressure=0.3})
h:sketch({{OAK.x-2, OAK.foot-60}, {OAK.x+20, OAK.foot-105}, {OAK.x+55, OAK.foot-140}}, {pressure=0.3})
h:sketch({{OAK.x-60, OAK.foot-150}, {OAK.x-20, OAK.foot-185}, {OAK.x+30, OAK.foot-190}, {OAK.x+75, OAK.foot-150}}, {pressure=0.15})
-- the spire far off
h:sketch({{640, hills(640)+2}, {640, 440}, {643, 418}, {646, 440}, {646, hills(646)+2}}, {pressure=0.2})

--@ chunk 3 · clock 0
-- the sky, first lay-in: thin, long horizontal passes, cold above, warm at the horizon
skyn = noise{seed=51, octaves=3, period=340, stretch={0.05, 5}}
function skycol(x, y)
  local t = clamp(y / (HZ - 10), 0, 1) + 0.03*skyn(x, y)
  return gradient({{0, "#667891"}, {0.28, "#8795a8"}, {0.55, "#a9b2b3"}, {0.74, "#c7c6b2"}, {0.88, "#dccbab"}, {1, "#e2c1a0"}}, t)
end
SKYM = above(function(x) return hills(x) + 8 end)
work(SKYM, {hand="broad", color=skycol, angle=function(x, y) return 0.02*skyn(y, x) end, coverage=4.2, medium=0.3})
blend(SKYM, {angle=0})

--@ chunk 4 · clock 0
dry()
-- the sky's second layer: stippled, the same scale of color, to fuse the lay-in into a gradation
stipple(SKYM, {width=3.2, color=skycol, coverage=2.6, pressure={0.45, 0.85}, dips={18, 0.35, 0.7}, medium=0.45})

--@ chunk 5 · clock 8529.7841796875
dry()
-- the far hills: a thin hazed band, a little darker than the sky above them, then stippled into the air
HILLM = (below(hills) * above(function(x) return shore(x) + 2 end)):roughen(0.8, 9, 3, 0.4)
function hillcol(x, y) local t = smoothstep(hills(x), HZ + 4, y)
  return mix(mix("#a49ea7", "#9b9aa6", hn:at01(x*2, 3)), "#c2b6ac", 0.6*t) end
work(HILLM, {hand="body", length={18, 50}, coverage=3.2, medium=0.25,
  angle=function(x, y) return 0.04*hn(x*3, 1) end, edge={found=0.3, soft=0.5, lost=0.2, period=60, seed=5}, color=hillcol})

--@ chunk 6 · clock 10556.654296875
-- the frozen pond: dull ice that holds a little of the evening, in horizontal strokes
icen = noise{seed=61, octaves=4, period=160, stretch={0.0, 7}}
POND = below(function(x) return shore(x) - 1 end) * above(function(x) return bank(x) + 8 end)
function icecol(x, y)
  local t = clamp((y - shore(x)) / (bank(x) - shore(x)), 0, 1)
  local c = gradient({{0, "#b9ad9f"}, {0.15, "#a9a39c"}, {0.5, "#8f959a"}, {1, "#7d858d"}}, t)
  return shift(c, 0.035*icen(x, y), 0, 0.006*icen(y, x))
end
work(POND, {hand="body", length={30, 90}, coverage=3.4, medium=0.22, angle=function(x, y) return 0.015*icen(x, y*3) end,
  color=icecol, edge="soft"})
-- the far bank's snow, over the pond's edge while it is wet
FARB = below(farbank) * above(function(x) return shore(x) + 1.5 end)
work(FARB:roughen(0.6, 7, 9, 0.3), {hand="body", tool="round 2.5", length={8, 26}, coverage=3.5, medium=0.12, angle=0,
  color=function(x, y) return mix("#d9cabb", "#c3bdbd", smoothstep(farbank(x), shore(x), y)) end, edge={found=0.5, soft=0.4, lost=0.1, period=30, seed=6}})

--@ chunk 7 · clock 10556.654296875
-- the near bank: snow in the evening's shadow (the light is behind it), catching the sky along its crest
drn = noise{seed=71, octaves=4, period=120, stretch={0.1, 3}}
NEAR = below(bank):roughen(1.2, 14, 7, 0.5)
function snowcol(x, y)
  local d = y - bank(x)
  local c = gradient({{0, "#c6c2c2"}, {10, "#b3b3ba"}, {40, "#a2a6b2"}, {110, "#959ba8"}, {170, "#878c99"}}, d)
  local drift = drn(x, y)
  c = shift(c, 0.03*drift, 0, -0.004*drift)
  local edge = math.max(math.abs(x - 500)/500, 0)
  return shift(c, -0.05*smoothstep(0.55, 1, edge)*smoothstep(560, 714, y), 0, -0.004)
end
work(NEAR, {hand="body", length={20, 60}, coverage=3.6, medium=0.14, color=snowcol,
  angle=function(x, y) local dx = (bank(x+6) - bank(x-6))/12; return math.atan(dx) + 0.12*drn(x*2, y) end})
