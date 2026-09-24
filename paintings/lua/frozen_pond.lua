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

--@ chunk 8 · clock 10556.654296875
dry()
-- snow blown over the ice in a few long streaks, more toward the far shore
local sn = noise{seed=81, octaves=5, period=150, stretch={0.0, 12}, warp={70, 10}}
local streak = mask(function(x, y)
  local t = clamp((y - shore(x)) / (bank(x) - shore(x)), 0, 1)
  return smoothstep(0.5 + 0.25*t, 0.7 + 0.25*t, sn:at01(x, y))
end) * POND
work(streak, {hand="body", tool="round 2.5", length={25, 80}, coverage=2.4, medium=0.15, hug=false, angle=0,
  angle_jitter=0.04, load=0.6, color=function(x, y) local t = clamp((y - shore(x)) / (bank(x) - shore(x)), 0, 1)
    return mix("#c9c0b6", "#a9acb4", t) end, edge="lost"})
-- open water: a long thin lead in the ice, dark, holding a little warm sky
HOLE = poly({{690,511},{730,509},{790,509.5},{850,512},{812,515.5},{752,516},{708,514.5}}, true):soften(0.8)
work(HOLE, {hand="detail", tool="round 1.5", length={15, 45}, coverage=3, medium=0.2, angle=0,
  color=function(x, y) return mix("#5b5c60", "#7e766d", smoothstep(509, 516, y)) end, edge={found=0.5, soft=0.5, period=25, seed=8}})

--@ chunk 9 · clock 26308.669921875
-- a low far wood on the left of the far bank, bare, a gray-violet mass in the haze
local wl = outline{{-10, HZ-8}, {60, HZ-19}, {120, HZ-14}, {190, HZ-24}, {250, HZ-17}, {320, HZ-21}, {380, HZ-10}, {430, HZ-4}, {460, HZ+1},
  open=true, char="soft", lobe=9, seed=91}
WOOD = wl:below(HZ + 6) * above(function(x) return farbank(x) + 1 end)
work(WOOD, {hand="hatch", tool="round 1.6", length={3, 9}, coverage=2.8, medium=0.2, angle=-1.45, angle_jitter=0.35,
  color=function(x, y) return mix("#8d8790", "#9c9599", 0) end, edge={soft=0.6, lost=0.4, period=30, seed=9}})

--@ chunk 10 · clock 26308.669921875
-- the oak, by hand: trunk and scaffold limbs placed, then limbs and twigs drawn stroke by stroke.
-- An oak: crooked, level limbs that change direction at elbows; twigs short and stiff.
OAKC = "#3d393a"
local ox, oy = OAK.x, OAK.foot
-- the trunk as a body of paint, flared at the foot
local trunkpts = {{ox-1, oy+2}, {ox, oy-20}, {ox-2, oy-45}, {ox-3, oy-62}, {ox-4, oy-72}}
TRUNK = ribbon(trunkpts, {11, 8.5, 7.5, 7, 6}):roughen(0.5, 6, 3, 0.3)
work(TRUNK, {hand="body", tool="round 2", length={5, 14}, coverage=3.4, medium=0.12, angle=-1.55, angle_jitter=0.2,
  color=function(x, y) return mix("#3a3637", "#57504b", smoothstep(ox-2, ox+5, x)*0.6) end, edge={found=0.6, soft=0.4, period=12, seed=3}})
-- scaffold limbs (from the trunk, placed by eye): {points, width at start, width at end}
OAKLIMBS = {
  {{{ox-4,oy-70},{ox-18,oy-92},{ox-36,oy-116},{ox-58,oy-138},{ox-84,oy-150},{ox-110,oy-156}}, 5.5, 1.6},
  {{{ox-3,oy-72},{ox+3,oy-104},{ox-1,oy-134},{ox+6,oy-162},{ox+2,oy-190}}, 5.5, 1.5},
  {{{ox-2,oy-52},{ox+22,oy-74},{ox+48,oy-92},{ox+72,oy-100},{ox+96,oy-112}}, 4.5, 1.4},
  {{{ox-1,oy-128},{ox-20,oy-150},{ox-30,oy-174},{ox-46,oy-190}}, 3.2, 1.1},
  {{{ox+2,oy-142},{ox+26,oy-166},{ox+50,oy-176},{ox+66,oy-192}}, 3.2, 1.1},
  {{{ox-36,oy-116},{ox-56,oy-110},{ox-80,oy-114},{ox-100,oy-108}}, 2.8, 1.0},
  {{{ox+48,oy-92},{ox+56,oy-118},{ox+52,oy-134}}, 2.4, 0.9},
  {{{ox+22,oy-74},{ox+30,oy-90},{ox+26,oy-100}}, 2.0, 1.2},   -- a broken stub
}
local function bend(pts, amt)   -- the hand's small wander between elbows
  local o = {}
  for i, p in ipairs(pts) do o[i] = {p[1] + (i > 1 and randn(0, amt) or 0), p[2] + (i > 1 and randn(0, amt) or 0)} end
  return o
end
local lb = brush{kind="round", width=3, point=0.6}
for i, L in ipairs(OAKLIMBS) do
  lb:reload(OAKC, 0.9)
  local pts = bend(L[1], 0.8)
  local w = {} for k = 1, #pts do w[k] = lerp(L[2], L[3], (k-1)/(#pts-1)) end
  local rib = ribbon(pts, w)
  work(rib, {hand="detail", tool="round 1.5", length={4, 10}, coverage=3, medium=0.12, color=OAKC, edge={found=0.7, soft=0.3, period=10, seed=i}})
end

--@ chunk 11 · clock 26308.669921875
-- the oak's finer wood: each branch a run of straight pieces changing direction at nodes (elbows),
-- twigs short and stiff, spreading outward and a little up; drawn thick to thin with pointed brushes
OAKB = {big=brush{kind="round", width=2.2, point=0.8}, mid=brush{kind="rigger", width=1.2, point=1}, fine=brush{kind="rigger", width=0.6, point=1}}
function oakbranch(x, y, ang, len, w, depth, col)
  local b = (w > 1.6) and OAKB.big or ((w > 0.7) and OAKB.mid or OAKB.fine)
  if b:fullness() < 0.35 then b:reload(col, 0.85) end
  local pts, n = {{x, y}}, math.max(2, math.floor(len / 9) + 1)
  local a, px, py = ang, x, y
  local nodes = {}
  for k = 1, n do
    a = a + randn(0, 0.35)                       -- an elbow at each node
    a = a + 0.15 * (math.atan(-1, 0.25*(math.cos(ang) > 0 and 1 or -1)) - a) * 0.3   -- a slight pull upward/outward
    local seg = len / n * rand(0.75, 1.25)
    px, py = px + seg*math.cos(a), py + seg*math.sin(a)
    pts[#pts+1] = {px, py}
    nodes[#nodes+1] = {px, py, a, k/n}
  end
  local p0 = b:pressure_for(w)
  b:stroke(pts, {pressure={p0, depth == 0 and 0 or p0*0.35}, ramps={0.02, 0.5}, shake=0.3})
  if depth > 0 then
    for _, nd in ipairs(nodes) do
      if rand() < 0.95 - 0.25*nd[4] then
        local side = (rand() < 0.5) and -1 or 1
        local ca = nd[3] + side * rand(0.45, 1.1)
        oakbranch(nd[1], nd[2], ca, len * rand(0.45, 0.7) * (1 - 0.35*nd[4]), math.max(0.3, w * rand(0.5, 0.75) * (1 - 0.4*nd[4])), depth - 1, col)
      end
    end
  end
end
OAKB.big:load(OAKC, 0.9); OAKB.mid:load(OAKC, 0.9); OAKB.fine:load(OAKC, 0.9)
-- from each scaffold limb: its tip runs on, and side branches leave along it
for i, L in ipairs(OAKLIMBS) do
  local pts = L[1]
  local n = #pts
  if i ~= 8 then
    local a = math.atan(pts[n][2] - pts[n-1][2], pts[n][1] - pts[n-1][1])
    oakbranch(pts[n][1], pts[n][2], a, 40 + rand(0, 18), L[3] * 1.0, 3, OAKC)
  end
  for k = 2, n - 1 do
    if rand() < 0.85 then
      local a = math.atan(pts[k+1][2] - pts[k][2], pts[k+1][1] - pts[k][1])
      local side = (k % 2 == 0) and -1 or 1
      oakbranch(pts[k][1], pts[k][2], a + side*rand(0.5, 1.0), rand(30, 52), lerp(L[2], L[3], k/n) * 0.5, 3, OAKC)
    end
  end
end
