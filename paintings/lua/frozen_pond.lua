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

--@ chunk 12 · clock 26308.669921875
-- pollard willows on the far bank: a short thick trunk, a knuckled head, rods rising from its knuckles.
-- (moved from the drawing: spacing made uneven, one smaller and one leaning)
WILLOWS = {{112, 0.8, -0.1}, {198, 1.1, 0.04}, {241, 0.72, 0.16}, {352, 0.95, -0.05}}
WILC = "#393536"
local rodb = brush{kind="rigger", width=0.9, point=1}
local rodf = brush{kind="rigger", width=0.5, point=1}
function willow(fx, fy, s, lean, seed)
  local hx, hy = fx + lean*30*s, fy - 34*s
  local tr = ribbon({{fx, fy+2}, {fx + lean*10*s + randn(0, 1), fy - 12*s}, {hx, hy}}, {8*s, 6.2*s, 8*s}):roughen(0.6*s, 5, seed, 0.3)
  -- the head: two or three knuckles
  local knuck = {}
  local nk = 2 + (seed % 2)
  local head = nil
  for k = 1, nk do
    local kx, ky = hx + (k - (nk+1)/2)*6*s + randn(0, 1.2*s), hy - 2*s + randn(0, 1.5*s)
    knuck[k] = {kx, ky}
    local e = ellipse(kx, ky, rand(4, 6)*s, rand(3, 4.5)*s)
    head = head and (head + e) or e
  end
  local m = tr + head:roughen(1*s, 4, seed + 1, 0.3)
  work(m, {hand="body", tool="round 1.5", length={3, 8}, coverage=4.5, medium=0.1, load=0.9, angle=-1.5, angle_jitter=0.5,
    color=function(x, y) return mix(WILC, "#57504d", 0.35*smoothstep(fx-2, fx+6*s, x)) end, edge={found=0.5, soft=0.5, period=8, seed=seed}})
  -- rods: mostly upright, from every knuckle; lengths very uneven, a few thick old ones, a few crossing
  local n = math.floor(26 + 18*s)
  for i = 1, n do
    local kn = knuck[1 + (i % nk)]
    local u = randn(0, 0.42)
    local a = -math.pi/2 + u*0.95 + lean*0.4 + (kn[1] - hx)/(12*s)*0.25
    local L = s*(66 - 30*math.min(1, math.abs(u))) * rand(0.35, 1.1)
    local bx, by = kn[1] + randn(0, 2.2*s), kn[2] - rand(0, 3)*s
    local bow = randn(0, 0.12)
    local pts = {{bx, by}}
    for k = 1, 3 do local aa = a + bow*k/3
      pts[#pts+1] = {pts[#pts][1] + L/3*math.cos(aa), pts[#pts][2] + L/3*math.sin(aa)} end
    local thick = rand() < 0.2
    local b = thick and rodb or rodf
    if b:fullness() < 0.3 or rand() < 0.1 then b:reload(mix(WILC, rand() < 0.5 and "#6e584a" or "#7d6a55", rand(0.2, 0.45)), rand(0.7, 0.95)) end
    b:stroke(pts, {pressure={thick and rand(0.6, 0.85) or rand(0.4, 0.8), 0}, ramps={0.02, rand(0.5, 0.8)}, shake=0.25})
  end
end
rodb:load(WILC, 0.9); rodf:load(WILC, 0.9)
for i, w in ipairs(WILLOWS) do willow(w[1], farbank(w[1]) + 3, w[2], w[3], 120 + i) end

--@ chunk 13 · clock 26308.669921875
-- the wayside cross: old weathered timber, leaning a little, snow on its top and arms
local cx, cf = CROSS.x, bank(CROSS.x) + 20
local ct = cf - 142
CROSS.foot, CROSS.top, CROSS.lean = cf, ct, 5
local lean = 5
local function P(t) return {cx + lean*t, cf + (ct - cf)*t} end
UPRIGHT = ribbon({P(0), P(0.5), P(1)}, {5.2, 4.8, 4.4}):roughen(0.25, 4, 31, 0.15)
local by = ct + 31
CROSS.beam = by
BEAM = ribbon({{cx - 26 + lean*0.75, by + 1.2}, {cx + lean*0.75, by}, {cx + 27 + lean*0.75, by - 1}}, {4.2, 4.4, 4}):roughen(0.25, 4, 32, 0.15)
CROSSM = UPRIGHT + BEAM
work(CROSSM, {hand="body", tool="round 1.5", length={4, 12}, coverage=4, medium=0.1, load=0.9, angle=-1.53, angle_jitter=0.15,
  color=function(x, y) return mix("#2f2a29", "#4a423d", 0.5*smoothstep(-2, 2.5, x - (cx + lean*(cf - y)/(cf - ct)))) end,
  edge={found=0.8, soft=0.2, period=14, seed=33}})
work(BEAM, {hand="detail", tool="round 1.2", length={5, 14}, coverage=2.5, medium=0.1, angle=0.02, color="#332d2b", edge="found"})

--@ chunk 14 · clock 26308.669921875
-- the woman seen from behind: a long dark cloak from the shoulders to the snow, a close hood
FIG.x = 530; FIG.foot = bank(530) + 22
local fx, fy = FIG.x, FIG.foot
local cloak = outline{pts={{fx-12.5,fy+0.5,"c"},{fx-10.5,fy-16},{fx-8.6,fy-33},{fx-7.6,fy-44},{fx-6.6,fy-49.5},{fx-3.5,fy-51.8},
  {fx+0.5,fy-52.2},{fx+4.6,fy-51.5},{fx+7.6,fy-48.8},{fx+8.8,fy-43},{fx+10,fy-31},{fx+11.8,fy-15},{fx+13.4,fy+0.5,"c"},{fx+3,fy+1.5},{fx-6,fy+1.2}},
  char="firm", seed=141, amount=0.5}
local head = ellipse(fx + 0.6, fy - 56.8, 3.7, 5.2) + ellipse(fx + 0.3, fy - 52.5, 2.6, 2.5)
FIGM = cloak:mask() + head:soften(0.3)
work(FIGM, {hand="body", tool="round 1.2", length={3, 10}, coverage=4.4, medium=0.1, load=0.9, angle=-1.57, angle_jitter=0.12,
  color=function(x, y) return mix("#262224", "#312c2c", smoothstep(fy - 30, fy, y)) end, edge={found=0.75, soft=0.25, period=16, seed=14}})

--@ chunk 15 · clock 26308.669921875
dry()
-- seat things in the snow: small drifts over the feet (opaque lead-white body, shaded like the bank)
local function drift(x, y, w, h, seed)
  return poly({{x - w, y + 1}, {x - w*0.55, y - h*0.55}, {x - w*0.1, y - h}, {x + w*0.35, y - h*0.8}, {x + w, y + 1}, {x + w*0.2, y + h*0.5}, {x - w*0.4, y + h*0.4}}, true):roughen(0.5, 5, seed, 0.4)
end
local cf = CROSS.foot
local drifts = drift(CROSS.x + 0.5, cf - 1, 9, 5, 151) + drift(FIG.x, FIG.foot + 0.5, 15, 2.6, 152)
work(drifts, {hand="detail", tool="round 1.5", length={3, 9}, coverage=4, medium=0.08, load=0.9, angle=0.05, color=snowcol, edge={soft=0.6, lost=0.4, period=10, seed=15}})
-- the far bank's snow drawn up round the oak's foot and the willows' feet
local far = drift(OAK.x - 1, OAK.foot + 2, 8, 2.2, 153)
for i, w in ipairs(WILLOWS) do far = far + drift(w[1], farbank(w[1]) + 5, 6*w[2], 1.8, 154 + i) end
work(far, {hand="detail", tool="round 1.2", length={2, 7}, coverage=4, medium=0.08, load=0.9, angle=0, color=function(x, y) return shift(sample(x, farbank(x) + 3, 1.5), -0.01, 0, -0.004) end, edge={soft=0.6, lost=0.4, period=10, seed=16}})
-- snow on the cross: the top of the upright and the upper edges of the arms
local lean, ct, by = CROSS.lean, CROSS.top, CROSS.beam
local sb = brush{kind="round", width=1.6, point=0.4}
sb:load("#dcd6d0", 0.9)
sb:stroke({{CROSS.x + lean - 2.5, ct + 0.6}, {CROSS.x + lean, ct - 0.8}, {CROSS.x + lean + 2.6, ct + 0.5}}, {pressure={0.8, 0.5}})
sb:stroke({{CROSS.x - 25.5 + lean*0.75, by - 1.3}, {CROSS.x - 14 + lean*0.75, by - 1.9}, {CROSS.x - 3.5 + lean*0.75, by - 2.1}}, {pressure={0.55, 0.7, 0.3}, ramps={0.1, 0.3}})
sb:stroke({{CROSS.x + 4.5 + lean*0.75, by - 2.4}, {CROSS.x + 16 + lean*0.75, by - 2.6}, {CROSS.x + 26.5 + lean*0.75, by - 2.8}}, {pressure={0.4, 0.7, 0.5}, ramps={0.2, 0.2}})

--@ chunk 16 · clock 46744.912109375
-- wind scoops and her footprints, glazed as blue-gray hollows over the dry snow, with a lit lip on each
local function scoop(x, y, w) return ellipse(x, y + 1.5, w, w*0.3):roughen(0.6, 4, 161, 0.8) end
local hollows = (scoop(CROSS.x + 1, CROSS.foot - 7.5, 7) + scoop(FIG.x + 0.5, FIG.foot - 0.3, 13)) - (CROSSM + FIGM)
PRINTS = {}
local steps = 17
for i = 0, steps do
  local t = i / steps
  local s = lerp(1.0, 0.36, t)
  local x = lerp(372, FIG.x - 4, t^0.85) + 16*math.sin(t*2.6) + ((i % 2 == 0) and -3.4 or 3.4)*s + randn(0, 0.7*s)
  local y = lerp(714, FIG.foot + 3, t^0.8) + randn(0, 0.5*s)
  PRINTS[#PRINTS+1] = {x, y, s}
  hollows = hollows + ellipse(x, y, 3.2*s, 1.5*s):roughen(0.35*s, 3, 170 + i, 0.4*s)
end
glaze(hollows:blur(1.2), {color="#5f6a86", coats=0.26})
local lip = brush{kind="round", width=1.2, point=0.5}
lip:load("#c6c5ca", 0.6)
lip:stroke({{CROSS.x - 6, CROSS.foot - 7.2}, {CROSS.x - 1, CROSS.foot - 8.4}, {CROSS.x + 3, CROSS.foot - 8.2}, {CROSS.x + 8, CROSS.foot - 6.8}}, {pressure={0.1, 0.35, 0.1}, ramps={0.4, 0.4}})
for i, p in ipairs(PRINTS) do
  local x, y, s = p[1], p[2], p[3]
  if i % 4 == 1 then lip:reload("#cfcdd0", 0.7) end
  lip:stroke({{x - 2.6*s, y + 0.9*s}, {x, y + 1.6*s}, {x + 2.6*s, y + 0.8*s}}, {pressure={0.1, 0.25 + 0.35*s, 0.05}, ramps={0.3, 0.4}})
end

--@ chunk 17 · clock 58644.58984375
-- dry grass and sedge through the snow: fine upturning strokes laid last over the snow
local gb = brush{kind="rigger", width=0.9, point=1}
local gf = brush{kind="rigger", width=0.55, point=1}
local GRASS = {"#4d4337", "#5b4f3f", "#6a5b45", "#433a32", "#7a6a4e", "#504740"}
function tuft(x, y, h, n, lean, seed)
  for i = 1, n do
    local b = (rand() < 0.3) and gb or gf
    if b:fullness() < 0.35 or rand() < 0.15 then b:reload(mix(GRASS[math.random(#GRASS)], "#8e9098", rand(0, 0.2)), rand(0.6, 0.9)) end
    local bx = x + randn(0, h*0.12)
    local a = -math.pi/2 + lean + randn(0, 0.28)
    local L = h * rand(0.45, 1.05)
    local bend = randn(0.12, 0.18) * ((a > -math.pi/2) and 1 or -1)
    local p = {{bx, y + rand(0, 1.5)}}
    for k = 1, 3 do
      local aa = a + bend*(k/3)^1.6
      p[#p+1] = {p[#p][1] + L/3*math.cos(aa), p[#p][2] + L/3*math.sin(aa)}
    end
    b:stroke(p, {pressure={clamp(0.35 + h/60, 0.3, 0.95), 0}, ramps={0.03, rand(0.55, 0.85)}, shake=0.3})
  end
end
-- near left corner: a big clump and a few smaller ones
local spots = {{40, 692, 78, 60}, {84, 704, 60, 44}, {14, 664, 52, 34}, {126, 710, 44, 26}, {168, 680, 30, 16}, {58, 640, 30, 14},
  {226, 700, 22, 10}, {930, 704, 70, 52}, {972, 676, 58, 40}, {884, 692, 36, 20}, {992, 636, 40, 22}, {846, 710, 28, 14}, {790, 690, 18, 8}}
for i, s in ipairs(spots) do tuft(s[1], s[2], s[3], s[4], randn(0.05, 0.12), 170 + i) end
-- along the crest of the bank, small and sparse, leaving the middle clear
for i = 1, 26 do
  local x = (i <= 13) and rand(10, 380) or rand(730, 995)
  local y = bank(x) + rand(2, 12)
  tuft(x, y, rand(9, 20), math.random(4, 9), randn(0.08, 0.1), 200 + i)
end
-- reeds at the far shore's edge, a hazed thin fringe
local rf = brush{kind="rigger", width=0.45, point=1}
for i = 1, 70 do
  local x = rand(420, 1000)
  if i % 6 == 1 then rf:reload(mix("#8a7c68", "#b8b0a8", 0.35), 0.7) end
  local y = shore(x) + rand(0.5, 2.5)
  rf:stroke({{x, y}, {x + randn(0.3, 0.4), y - rand(3, 7)}}, {pressure={0.35, 0}, ramps={0.05, 0.7}})
end

--@ chunk 18 · clock 58644.58984375
dry()
-- two long thin streaks of cloud low in the glow, lit rose from below, gray above
local cn = noise{seed=181, octaves=5, period=260, stretch={0.0, 14}, warp={90, 8}}
local band = function(y0, th, x0, x1) return mask(function(x, y)
  local e = smoothstep(x0, x0 + 80, x) * (1 - smoothstep(x1 - 120, x1, x))
  local d = math.abs(y - y0 - 6*cn(x*0.3, 3)) / th
  return e * clamp(1.2 - d, 0, 1) * smoothstep(0.45, 0.7, cn:at01(x, y))
end) end
STREAKS = band(372, 5, 520, 1000) + band(398, 3.5, 60, 520) + band(344, 3, 700, 980)
work(STREAKS, {hand="scumble", tool="round 2", length={30, 90}, coverage=2.2, medium=0.35, angle=0, angle_jitter=0.02, hug=false,
  color=function(x, y) return mix("#a69ea6", "#d8b7a4", smoothstep(-3, 4, y - 372 + (x < 520 and -26 or 0))) end, edge="lost"})
-- the village church far off on the hills: nave, a slim tower and spire, as hazed as the hills
local chx, chy = 648, hills(648) + 5
CHURCH = rect(chx - 14, chy - 7, 20, 9) + poly({{chx - 15, chy - 7}, {chx - 4, chy - 11.5}, {chx + 7, chy - 7}}) +
  rect(chx + 4, chy - 22, 4.2, 22) + poly({{chx + 3.6, chy - 22}, {chx + 6.1, chy - 38}, {chx + 8.6, chy - 22}})
work(CHURCH, {hand="detail", tool="round 1", length={2, 6}, coverage=3.4, medium=0.15, angle=-1.57, color="#8a8590", edge={found=0.4, soft=0.6, period=8, seed=18}})
local roofs = rect(chx - 34, chy - 4, 14, 6) + poly({{chx - 35, chy - 4}, {chx - 27, chy - 8}, {chx - 19, chy - 4}}) + rect(chx + 16, chy - 3, 11, 5)
work(roofs, {hand="detail", tool="round 1", length={2, 6}, coverage=3, medium=0.15, angle=0, color="#948e97", edge="soft"})
-- the new moon, a thin crescent low in the west over the glow, its lit side toward the set sun
MOON = {x=452, y=236, r=6.5}
-- one stroke of a pointed brush from horn to horn along the lit limb, swelling in the middle
local mb = brush{kind="round", width=2.6, point=1}
mb:load("#f2ecd6", 1)
local arc = {}
for k = 0, 8 do local a = math.rad(-35 + 150*k/8)   -- lit limb toward the lower right
  arc[#arc+1] = {MOON.x + MOON.r*math.cos(a), MOON.y + MOON.r*math.sin(a)} end
mb:stroke(arc, {pressure={0.05, 0.85, 0.05}, ramps={0.45, 0.45}})
mb:reload("#f5f0de", 0.8)
mb:stroke(arc, {pressure={0.02, 0.55, 0.02}, ramps={0.4, 0.4}})
-- crows going home over the oak
local cb = brush{kind="round", width=1.4, point=1}
cb:load("#2f2b2d", 0.9)
for _, c in ipairs({{700, 214, 5.2, 0.1}, {731, 203, 4.4, -0.15}, {664, 238, 3.8, 0.25}, {848, 188, 3.2, 0}}) do
  local x, y, s, t = c[1], c[2], c[3], c[4]
  cb:stroke({{x - s, y - 0.9*s + t*s}, {x - 0.4*s, y - 0.1*s}, {x, y + 0.15*s}}, {pressure={0.1, 0.7}, ramps={0.5, 0.1}})
  cb:stroke({{x, y + 0.15*s}, {x + 0.45*s, y - 0.15*s}, {x + 1.1*s, y - 0.8*s - t*s}}, {pressure={0.7, 0.1}, ramps={0.1, 0.5}})
end

--@ chunk 19 · clock 63072.85498046875
-- the evening comes down: a cool glaze over the near snow, deeper toward the bottom, and a dark veil
-- growing toward the picture's edges (Friedrich to Carus: darker toward the edges)
local vn = noise{seed=191, octaves=3, period=300}
NEARDARK = mask(function(x, y) return smoothstep(bank(x) + 10, 714, y) * (0.75 + 0.25*vn:at01(x, y)) end)
glaze(NEARDARK, {color="#5d6680", coats=0.32})
VIG = mask(function(x, y) local dx, dy = (x - 520)/560, (y - 430)/430
  return smoothstep(0.6, 1.3, math.sqrt(dx*dx + dy*dy)) * (0.8 + 0.2*vn:at01(y, x)) end)
glaze(VIG, {color="#3f4458", coats=0.3})

--@ chunk 20 · clock 71762.13427734375
-- the ice worked over the dry lay-in: thin long streaks of polished ice glazed darker, and a few pale
-- drifted lines of snow laid with a small brush, all running flat to the horizon
local KEEP = -(CROSSM + FIGM):grow(0.8)
local pn = noise{seed=201, octaves=4, period=260, stretch={0.0, 45}, warp={300, 1.5}}
local function tpond(x, y) return clamp((y - shore(x)) / (bank(x) - shore(x)), 0, 1) end
POLISH = mask(function(x, y) local t = tpond(x, y)
  return smoothstep(0.6, 0.66, pn:at01(x, y) + 0.08*t) * (0.6 + 0.4*t) end) * POND * KEEP
glaze(POLISH:blur(0.6), {color="#4b5663", coats=0.4})
-- snow lines: short and long dry strokes of a small round, flat, lighter toward the far shore
local sb = brush{kind="round", width=1.6}
for i = 1, 150 do
  local x = rand(-20, 1000)
  local t = rand()^1.4
  local y = lerp(shore(x) + 3, bank(x) + 2, t)
  if i % 6 == 1 then sb:reload(mix(mix("#d2c8bd", "#aeb2ba", t), "#8e949c", rand(0.15, 0.4)), rand(0.3, 0.55)) end
  local L = rand(20, 110) * (1 - 0.4*t)
  local pts = {{x, y}, {x + L/2, y + randn(0, 0.35)}, {x + L, y + randn(0, 0.5)}}
  sb:stroke(pts, {pressure={0.05, rand(0.2, 0.45), 0.03}, ramps={rand(0.3, 0.5), rand(0.35, 0.6)}, clip=POND * KEEP})
end

--@ chunk 21 · clock 71762.13427734375
-- the near snow: long wind drifts, each a cool shadow on its lee under a lit crest; a few stones
local KEEPF = -(CROSSM + FIGM):grow(1)
local dn = noise{seed=211, octaves=4, period=200, stretch={-0.08, 6}, warp={140, 10}}
DRIFTSH = mask(function(x, y)
  local d = y - bank(x); if d < 14 then return 0 end
  local v = dn:at01(x, y*1.6)
  return smoothstep(0.55, 0.62, v) * (1 - smoothstep(0.66, 0.74, v)) * smoothstep(14, 40, d) end) * KEEPF
glaze(DRIFTSH:blur(1.5), {color="#56607c", coats=0.3})
-- stones through the snow: dark tops, snow lying on them
local stb = brush{kind="round", width=3, point=0.3}
for i, s in ipairs({{206, 676, 17, 7}, {232, 682, 8, 4}}) do
  local x, y, w, h = s[1], s[2], s[3], s[4]
  local st = poly({{x - w, y}, {x - w*0.7, y - h*0.8}, {x - w*0.1, y - h}, {x + w*0.6, y - h*0.7}, {x + w, y}}, true):roughen(0.5, 3, 212 + i, 0.3)
  work(st, {hand="detail", tool="round 1.2", length={2, 6}, coverage=4, medium=0.1, angle=0.2, color="#4c4a4d", edge="found"})
  stb:reload("#c9c7cc", 0.8)
  stb:stroke({{x - w*0.75, y - h*0.7}, {x - w*0.1, y - h*1.05}, {x + w*0.55, y - h*0.75}}, {pressure={0.3, 0.6, 0.25}, ramps={0.2, 0.3}})
  stb:reload("#b4b5bd", 0.7)
  stb:stroke({{x - w*1.3, y + 0.8}, {x, y + 1.2}, {x + w*1.3, y + 0.6}}, {pressure={0.4, 0.7, 0.3}, ramps={0.2, 0.3}})
end

--@ chunk 22 · clock 72780.63623046875
-- mending: a stray curl of the hill paint at the far right (fill the hollow, cut the hook back to sky)
local fill = poly({{898, 447}, {912, 442}, {930, 441}, {940, 446}, {936, 450}, {904, 451}}, true)
work(fill, {hand="detail", tool="round 1.5", length={4, 10}, coverage=3.4, medium=0.25, angle=0, color=sample(918, 453, 2), edge="soft"})
local hook = poly({{897, 437}, {905, 434}, {920, 434}, {938, 439}, {944, 444}, {930, 441}, {910, 440}, {899, 441}}, true)
work(hook, {hand="detail", tool="round 1.5", length={4, 10}, coverage=3.4, medium=0.25, angle=0.05, color=sample(920, 428, 3), edge="soft"})
-- the far wood's skyline: bare crowns against the glow, a fringe of fine upright twig strokes
local tw = brush{kind="rigger", width=0.45, point=1}
for i = 1, 260 do
  local x = rand(0, 440)
  local top = nil
  for y = HZ - 32, HZ + 4 do if WOOD:at(x, y) > 0.5 then top = y break end end
  if top then
    if i % 8 == 1 then tw:reload(mix("#8d8790", "#a9a1a0", rand(0, 0.4)), 0.7) end
    local h = rand(2, 7)
    tw:stroke({{x, top + 2}, {x + randn(0, 0.8), top - h*0.5}, {x + randn(0, 1.4), top - h}}, {pressure={0.4, 0}, ramps={0.05, 0.7}})
  end
end
-- the willow trunks: a dark glaze to pull their speckle together
local wt = nil
for _, w in ipairs(WILLOWS) do local fy = farbank(w[1]) + 3; local m = ellipse(w[1] + w[3]*15*w[2], fy - 20*w[2], 7*w[2], 22*w[2])
  wt = wt and (wt + m) or m end
glaze(wt * mask(function(x, y) local c = sample(x, y); return (c.L < 0.5) and 1 or 0 end), {color="#2c2729", coats=0.35})

--@ chunk 23 · clock 85342.18896484375
dry()
-- the woman, modeled over the dry silhouette: long folds of the cloak, the hood, a thin rim of the glow
local fx, fy = FIG.x, FIG.foot
local pb = brush{kind="round", width=1.1, point=1}
-- folds: lighter where the cloth turns to the sky, darker in the hollows between
local folds = {{-5.5, -46, -8.5, -1, "#3f3938"}, {-1, -49, -2, 0, "#3a3434"}, {4.5, -47, 8, -1, "#3f3938"}, {8, -40, 11.2, -2, "#453e3c"},
               {-3.2, -45, -5.4, 0, "#1d1a1c"}, {2, -48, 3.2, 0, "#1d1a1c"}, {6.2, -44, 9.8, 0, "#201c1e"}}
for _, f in ipairs(folds) do
  pb:reload(f[5], 0.8)
  local mx = (f[1] + f[3])/2 + randn(0, 0.25)
  pb:stroke({{fx + f[1], fy + f[2]}, {fx + mx, fy + (f[2] + f[4])/2}, {fx + f[3], fy + f[4]}}, {pressure={0.05, 0.45, 0.6}, ramps={0.35, 0.1}, clip=FIGM})
end
-- the hood's top and its fall onto the shoulders, a shade lighter
pb:reload("#433c3a", 0.7)
pb:stroke({{fx - 3, fy - 60}, {fx + 0.5, fy - 61.8}, {fx + 3.6, fy - 59.6}}, {pressure={0.2, 0.5, 0.2}, ramps={0.3, 0.3}, clip=FIGM})
pb:stroke({{fx - 6.5, fy - 49.8}, {fx - 3.5, fy - 51.4}, {fx + 0.5, fy - 51.6}}, {pressure={0.15, 0.4, 0.1}, ramps={0.3, 0.3}, clip=FIGM})
-- the rim of the glow: a hair of warm light on the right side of hood and shoulder, and on the left sleeve
local rb = brush{kind="round", width=0.6, point=1}
rb:load("#9a8474", 0.6)
rb:stroke({{fx + 3.9, fy - 60.5}, {fx + 4.4, fy - 57}, {fx + 5, fy - 53}, {fx + 8.2, fy - 49.4}, {fx + 9.1, fy - 44}}, {pressure={0.05, 0.3, 0.25, 0.05}, ramps={0.3, 0.4}})
rb:reload("#857368", 0.5)
rb:stroke({{fx - 7.8, fy - 48}, {fx - 8.6, fy - 42}, {fx - 9.2, fy - 34}}, {pressure={0.05, 0.25, 0.05}, ramps={0.3, 0.5}})
-- the hem sunk in the snow: short strokes of the snow over the bottom edge, uneven
local sn = brush{kind="round", width=1.6, point=0.4}
for i = 1, 9 do
  local x = fx - 13 + i*2.9 + randn(0, 0.5)
  sn:reload(snowcol(x, fy + 3):hex(), 0.7)
  sn:stroke({{x - 2.2, fy + 0.4 + randn(0, 0.5)}, {x + 2.2, fy - rand(0, 1.4)}}, {pressure={0.4, 0.6}, ramps={0.2, 0.3}})
end

--@ chunk 24 · clock 85342.18896484375
wait(24*60); varnish(); cracks{}; relief()
