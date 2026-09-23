-- easel session "l3_near": a painting replayed chunk by chunk.
--   easel run paintings/lua/l3_near.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.3, seed=23}; print(H); print(pal)

--@ chunk 2 · clock 0
-- the place: a snowy clearing at the foot of a spruce wood, late afternoon in
-- early winter; a low sun from the left, a little behind me
HZ = 300
w = world{horizon=HZ, eye=1.6, fov=52, sun={azimuth=-112, elevation=11}, visibility=6000,
  ground=function(X, Z) return 0.18*math.sin(X/2.3 + Z/3.1) + 0.12*math.sin(X/5.7 - Z/1.7) + 0.02*Z end}
-- the erratic: a granite boulder left by the ice, broad and low, turned a little
bs = w:spot_at(-0.5, 6.4)
boulder = body.ellipsoid(bs:p(0, 0.3, 0), bs:size(1.45, 0.82, 1.05))
  :turn(bs:p(0, 0, 0), 0.35, 0.08, -0.07)
  :rough(bs:m(0.12), bs:m(1.3), 3)
  :cut(bs:p(-0.2, 0.95, 0.1), {-0.25, -1, 0.35}, 10, bs:m(0.08))
  :cut(bs:p(0.75, 0.5, 0.3), {0.9, -0.35, 0.3}, 11, bs:m(0.06))
  :rough(bs:m(0.025), bs:m(0.25), 5, true)
w, stone = w:place(bs, boulder)
-- a small second stone half sunk at its right foot
ss = w:spot_at(0.95, 5.5)
w, stone2 = w:place(ss, body.ellipsoid(ss:p(0, 0.05, 0), ss:size(0.34, 0.2, 0.28)):rough(ss:m(0.04), ss:m(0.4), 8))
-- the young spruce behind-right of the stone
ts = w:spot_at(1.55, 7.6)
print("boulder", bs, "spruce", ts, "tall 3.2m =", w:height(ts.x, ts.y, 3.2))
v = w:view()
show(v:bodies_mask{stone, stone2}, {label="stone"})
show(ts.x, ts.y, "spruce foot")

--@ chunk 3 · clock 0
-- the drawing, first pass: a hard pencil, light and searching
h = pencil("2H")
-- the boulder's contour, by eye off the grid
BOULDER = {{186,492},{188,432},{214,388},{262,352},{330,330},{410,318},{498,316},{538,334},{572,378},{596,430},{604,494}}
h:sketch(BOULDER, {pressure=0.3})
h:sketch({{612,550},{626,516},{668,504},{712,506},{738,526},{742,556}}, {pressure=0.28})
-- the young spruce: its axis leaning a hair, the spread of its tiers
h:sketch({{709,484},{711,360},{713,200},{716,56}}, {pressure=0.3})
h:sketch({{716,56},{672,200},{640,330},{612,470}}, {pressure=0.22})
h:sketch({{716,56},{756,200},{790,330},{812,470}}, {pressure=0.22})
-- the wood's edge: tall spruces on the left falling away toward the clearing
WOODTOP = {{0,-10},{120,-6},{250,8},{330,30},{410,64},{480,104},{540,150},{585,200},{612,250},{640,300}}
h:sketch(WOODTOP, {pressure=0.25})
h:sketch({{0,352},{200,350},{400,348},{640,340}}, {pressure=0.2})
-- the far wood line across the clearing, and the horizon behind it
h:sketch({{640,298},{760,292},{880,296},{1000,290}}, {pressure=0.22})
-- the birch stump, near, lower left
STUMP = {{96,700},{100,610},{92,568},{112,556},{128,574},{140,548},{150,600},{154,700}}
h:sketch(STUMP, {pressure=0.3})

--@ chunk 4 · clock 0
-- second pass: a soft lead, firm where I'm sure
b = pencil("3B")
b:line(BOULDER, {pressure={0.6, 0.75, 0.7, 0.55}})
-- the big fracture on its top and the joint running down the right shoulder
b:line({{300,338},{360,352},{430,350},{500,336}}, {pressure={0.4, 0.55, 0.4}})
b:line({{540,338},{548,390},{566,452},{572,494}}, {pressure={0.45, 0.6, 0.35}})
b:line({{612,550},{626,516},{668,504},{712,506},{738,526},{742,556}}, {pressure={0.5, 0.6, 0.5}})
b:line({{709,484},{711,360},{713,200},{716,56}}, {pressure={0.5, 0.4, 0.3}})
b:line(WOODTOP, {pressure={0.35, 0.5, 0.55}})
b:line(STUMP, {pressure={0.6, 0.7, 0.6}, smooth=false})
b:line({{640,298},{760,292},{880,296},{1000,290}}, {pressure=0.35})
b:hatch(poly({{540,338},{572,378},{596,430},{604,494},{566,494},{548,400}}), {angle=-1.1, pressure=0.3})
fix()

--@ chunk 5 · clock 0
-- the sky: a pale winter afternoon, a thin high veil drawing across it
sk = w:sky{haze=3.2, overcast=0.35, uneven={0.4, 30000, 5}}
cl = w:clouds{sky=sk, cell=3,
  {kind="stratus", base=4200, thick=400, cover=0.5, seed=6, wind={-0.25, 2.5}, breaks={12000, 0.25}},
  {kind="bank", x0=-30000, x1=30000, z=30000, depth=8000, base=500, top=1300, seed=3}}
skym = above(function(x) return HZ + 14 end)
work(skym, {hand="broad", color=cl, angle=0, coverage=4.5, medium=0.3,
  pal=pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "red earth", "vermilion", "raw umber"}})
blend(skym, {angle=0})

--@ chunk 6 · clock 0
wait(24*60)
-- the far wood across the clearing: a low soft band, blue with distance
farwood = outline{{560,300},{640,288},{700,291},{760,283},{830,289},{900,281},{960,287},{1010,284}, open=true, char="soft", lobe=9, seed=12}
fw = farwood:below(HZ + 4)
work(fw, {hand="body", tool="round 3", length={4, 10}, angle=-1.45, angle_jitter=0.5, coverage=3.2, clip=fw,
  color=function(x, y) return mix("#5b6570", "#76808a", smoothstep(280, 302, y)) end, medium=0.25})
-- snow on the ground, as tone: the low sun rakes the drifts; farther, grayer and bluer
local grain = noise{seed=5, period=30, stretch={0.05, 4}}
snow = below(function(x) return HZ + 1 end)
work(snow, {hand="body", angle=function(x, y) return 0.04 + 0.08 * grain(x, y) end, length={14, 45}, coverage=3.5, medium=0.18,
  color=function(x, y)
    local p = v:at(x, y)
    local lit = (p and p.what == "ground" and p.shade) and p.shade.value or 0.45
    local c = mix("#aeb2c2", "#f0e8d6", smoothstep(0.3, 0.5, lit))
    local d = p and p.dist or 200
    c = mix(c, "#d0d0d0", clamp((d - 10) / 60, 0, 0.5))
    return shift(c, 0.02 * grain(x, y), 0, 0)
  end})

--@ chunk 7 · clock 1440
wait(24*60)
-- my spruce hand: an axis, then whorls of branches that droop and lift at the tip,
-- longer toward the foot; a few shorter ones reaching toward me between them
function spruce(o)
  local x, top, base, hw = o.x, o.top, o.base, o.halfw
  local tier = o.tier or (base - top) / 22
  local lean = o.lean or 0
  local r = noise{seed=o.seed or 1, period=40}
  local sp = {axis={}, branches={}}
  for k = 0, 8 do local t = k / 8; sp.axis[#sp.axis + 1] = {x + lean * t * (base - top) + 2 * r(k * 13, 0), top + t * (base - top)} end
  local y = top + tier * 0.6
  local i = 0
  while y < base - tier * 0.2 do
    local t = (y - top) / (base - top)
    local ax = x + lean * t * (base - top)
    for _, s in ipairs{-1, 1} do
      i = i + 1
      local L = hw * (0.12 + 0.88 * t ^ 0.85) * (0.85 + 0.3 * rand())
      local droop = (0.18 + 0.2 * t) * L * (o.droop or 1)
      local lift = 0.08 * L
      local yy = y + randn(0, tier * 0.12)
      local pts = {{ax, yy}, {ax + s * 0.4 * L, yy + 0.55 * droop}, {ax + s * 0.8 * L, yy + droop}, {ax + s * L, yy + droop - lift}}
      local wd = tier * (0.42 + 0.3 * t) * (o.thick or 1)
      sp.branches[#sp.branches + 1] = {pts=pts, w={wd * 0.8, wd, wd * 0.7, wd * 0.25}, s=s, t=t}
      if rand() < 0.7 then  -- a branch toward me, foreshortened
        local L2 = L * (0.35 + 0.3 * rand())
        local y2 = yy + tier * 0.45
        local p2 = {{ax, y2}, {ax + s * 0.5 * L2, y2 + 0.4 * droop * 0.6}, {ax + s * L2, y2 + 0.5 * droop * 0.6}}
        sp.branches[#sp.branches + 1] = {pts=p2, w={wd * 0.9, wd * 0.8, wd * 0.3}, s=s, t=t, front=true}
      end
    end
    y = y + tier * (0.85 + 0.3 * rand())
  end
  local m = ribbon(sp.axis, math.max(1.5, hw * 0.06))
  for _, b in ipairs(sp.branches) do m = m + ribbon(b.pts, b.w) end
  sp.mask = m:roughen(math.max(0.8, tier * 0.12), math.max(3, tier * 0.4), o.seed or 1, 0.5)
  return sp
end
-- the wood's edge: spruces stepping down toward the clearing, tips on my drawn line
local function woodtop(x)  -- the pencil line WOODTOP, read back
  for k = 2, #WOODTOP do
    local a, b = WOODTOP[k - 1], WOODTOP[k]
    if x <= b[1] then return a[2] + (b[2] - a[2]) * (x - a[1]) / (b[1] - a[1]) end
  end
  return WOODTOP[#WOODTOP][2]
end
edge = {}
-- not a comb stepping down the slope: clumps and gaps, one tall old tree standing out of the
-- line, a pair grown together, a young one low between them, spires thin and broad
local xs = uneven(18, 14, 650, 0.85, 0.55, 5)
for k, ex in ipairs(xs) do
  local tp = woodtop(ex) + (ex < 300 and rand(-70, 35) or randn(0, 30))
  if k == 12 then tp = tp - 55 end            -- the old one, above the others
  if k % 5 == 3 then tp = tp + rand(25, 60) end -- young ones low in the line
  local hwf = rand(0.18, 0.32)
  edge[#edge + 1] = spruce{x=ex, top=tp, base=358, halfw=(358 - tp) * hwf, seed=20 + k, droop=rand(0.7, 1.2), lean=randn(0, 0.012), thick=rand(0.85, 1.15)}
  if k % 4 == 1 and ex > 300 then            -- a second spire grown in against it
    local tp2 = tp + rand(15, 45)
    edge[#edge + 1] = spruce{x=ex + rand(9, 16), top=tp2, base=358, halfw=(358 - tp2) * rand(0.18, 0.26), seed=60 + k, droop=rand(0.8, 1.2)}
  end
end
-- the dense interior behind the edge trees
local inner = poly({{-10,95},{60,80},{130,100},{200,86},{270,110},{330,140},{390,175},{450,215},{510,258},{570,300},{630,340},{660,362},{-10,362}})
  :roughen(4, 18, 7)
woodm = inner
for _, e in ipairs(edge) do woodm = woodm + e.mask end
woodm = woodm * below(function(x) return -20 end) - below(function(x) return 364 end)
print(woodm:area())
show(woodm, {label="wood"})
show()
-- paint it: near-black green in short hatched strokes, lying along the branches
local turn = noise{seed=31, period=14}
local wcol = function(x, y)
  local c = mix("#1f2824", "#2c3630", smoothstep(40, 360, y))
  return shift(c, 0.015 * turn(x, y), 0, 0)
end
work(woodm, {hand="hatch", tool="round 2", length={3, 9}, coverage=3.2, clip=woodm, medium=0.2,
  angle=function(x, y) return 0.25 * turn(x, y) + (x < 330 and 0.1 or 0.35) end, angle_jitter=0.5, color=wcol})
-- the branches of the edge spruces drawn out with a pointed brush, so their tiers read against the sky
local rb = brush("rigger", 0.9)
local n = 0
for _, e in ipairs(edge) do
  rb:reload("#1c2420", 0.8)
  rb:stroke(e.axis, {pressure={0.2, 0.7}, ramps={0.3, 0.1}})
  for _, b in ipairs(e.branches) do
    n = n + 1
    if n % 6 == 0 then rb:reload("#1f2723", 0.75) end
    rb:stroke(b.pts, {pressure={0.8, 0.05}, ramps={0.05, 0.6}})
  end
end
print(n, "branches")

--@ chunk 8 · clock 2880
dry()
-- snow banked at the wood's foot: uneven drifts, bough tips hanging over them, fading into the field
local foot = outline{{-10,362},{60,354},{120,360},{190,350},{250,358},{320,347},{390,357},{450,350},{520,358},{580,349},{640,356},{700,360}, open=true, char="soft", lobe=10, amount=1.4, seed=41}
bank = foot:below(H) * above(function(x) return 425 end):soften(18) * rect(-10, 320, 720, 140):soften(12)
work(bank, {hand="body", length={6, 20}, coverage=3.2, medium=0.15, angle=0.03, hug=false,
  color=function(x, y) return mix("#c4c4c8", sample(x, 440, 3), smoothstep(352, 400, y)) end})
-- snow lying on the boughs: only where a bough's upper face meets the air above it
local lie = noise{seed=44, period=7}
tops = mask(function(x, y) return woodm:at(x, y) * (1 - woodm:at(x, y - 2.2)) end) * mask(function(x, y) return smoothstep(0.35, 0.7, lie:at01(x, y)) end)
stipple(tops, {width=1.1, color=function(x, y) return mix("#a9aaa8", "#d6d0c0", smoothstep(0, 360, y)) end,
  coverage=1.2, pressure={0.3, 0.6}, drag={1, 0}, aim=false, medium=0.2, fade=0})
print(tops:area())

--@ chunk 9 · clock 30910.8671875
-- the erratic: gray granite, warm where the low sun takes it, cold violet-gray in its own shadow
local f = v.form
stonem = v:visible("bodies")
local mott = noise{seed=51, period=35, warp={50, 20}}
local fine = noise{seed=52, period=8}
function granite(x, y)
  local val = f:value(x, y) or 0.4
  local c = gradient({{0, "#3e3f48"}, {0.3, "#5d5c62"}, {0.55, "#8a847a"}, {0.8, "#b9ad98"}, {1, "#d2c3a6"}}, clamp(val * 1.05, 0, 1))
  c = shift(c, 0.035 * mott(x, y) + 0.015 * fine(x, y), 0.004 * mott(x + 90, y), 0.006 * mott(x, y + 70))
  return c
end
work(stonem, {hand="body", tool="filbert 4", length={6, 18}, coverage=3.6, clip=stonem, medium=0.15,
  angle=f:field("fall"), angle_jitter=0.4, color=granite})

--@ chunk 10 · clock 30910.8671875
dry()
local f = v.form
-- the shadow flank pulled together: a cool violet-gray scumbled over it, down the plane
local sh = f:shadow{parts={1, 2}, soft=0.15} * stonem
work(sh, {hand="scumble", tool="filbert 5", length={10, 26}, coverage=2.2, clip=stonem, angle=f:field("fall"),
  color_over=function(x, y, under) return mix(under, "#4b4a55", 0.55) end})
-- fissures: where the planes break into hollows, and the fracture I drew on the top
local cr = f:edges{turn=0.7, step=3, span=2.5, concave=true} * stonem:shrink(3)
work(cr, {hand="detail", tool="round 1.4", length={4, 12}, coverage=1.6, clip=stonem, angle=f:field("edge"), color="#2e2c2f"})
local fb = brush("round", 1.6)
fb:load("#34302f", 0.8)
fb:stroke({{302,340},{330,346},{362,353},{396,352},{430,350},{468,344},{500,337}}, {pressure={0.2, 0.7, 0.5, 0.15}, ramps={0.2, 0.3}, shake=1.2})
fb:reload("#2f2c30", 0.8)
fb:stroke({{541,340},{546,372},{552,402},{561,436},{566,466},{570,492}}, {pressure={0.5, 0.8, 0.3}, ramps={0.1, 0.4}, shake=1.5})
fb:stroke({{553,410},{540,432},{532,461}}, {pressure={0.4, 0.05}, ramps={0.1, 0.6}, shake=1})
fb:stroke({{250,430},{276,436},{300,452},{318,478}}, {pressure={0.1, 0.45, 0.05}, ramps={0.3, 0.4}, shake=1.2})
-- lichen: crusts of pale gray-green and ochre on the lit face
local lc = worley{seed=55, period=9}
local lz = noise{seed=56, period=60}
local lich = stonem:shrink(2) * mask(function(x, y) local _, _, e, r = lc:at(x, y); return smoothstep(0.1, 0.35, lz:at01(x, y)) * (r < 0.35 and 1 or 0) end)
stipple(lich, {width=1.4, color=function(x, y) local _, _, _, r = lc:at(x, y); return r < 0.15 and "#a9a77f" or "#8f8f78" end,
  coverage=1.1, pressure={0.3, 0.6}, aim=false, medium=0.15, fade=0.5})
print(lich:area())

--@ chunk 11 · clock 54556.580078125
dry()
local f = v.form
-- the first snow lying on the top: a thin continuous sheet where the surface faces up,
-- torn open only where the grain of the stone breaks through
local lie = noise{seed=61, period=22, warp={30, 8}}
cap = stonem * f:mask(function(s) return smoothstep(-0.5, -0.8, s.n[2]) end) * mask(function(x, y) return smoothstep(0.18, 0.42, lie:at01(x, y)) end)
cap = cap:roughen(2, 10, 62, 0.8) * stonem
work(cap, {hand="body", tool="filbert 4", length={6, 16}, coverage=2.6, clip=cap, medium=0.32, load=0.6, angle=0.05, angle_jitter=0.25,
  color=function(x, y) local val = f:value(x, y) or 0.5; return mix("#b0b3c2", "#efe7d5", smoothstep(0.35, 0.75, val)) end})
blend(cap, {angle=0.05, clip=cap})
-- snow drifted against the foot of both stones: the field itself banking up the stone,
-- thin paint that fades into the snow in front and creeps up the rock in uneven tongues
local m = v:visible("bodies")
FOOT = {}
for x = 180, 760, 2 do
  local b
  for y = 600, 300, -1 do if m:at(x, y) > 0.5 then b = y; break end end
  FOOT[x] = b
end
function footat(x)
  local x0 = 2 * (x // 2)
  local a, b = FOOT[x0], FOOT[x0 + 2]
  if not a or not b then return nil end
  return a + (b - a) * (x - x0) / 2
end
-- snow banked against the foot: one continuous bank, not tongues. The wind came from the
-- left: it piled deepest at the windward end, runs low and ragged along the front, thins to
-- almost nothing under the shaded flank and rises once more in the corner by the small stone.
local hn = noise{seed=63, period=95}
local hn2 = noise{seed=64, period=19}
local hn3 = noise{seed=68, period=6}
function driftenv(x)
  return 4 + 13 * math.exp(-((x - 222) / 48) ^ 2) + 6 * math.exp(-((x - 430) / 60) ^ 2)
    + 5 * math.exp(-((x - 590) / 16) ^ 2) - 3 * smoothstep(500, 560, x) * (1 - math.exp(-((x - 590) / 16) ^ 2))
    - (x > 606 and 2 or 0)
end
function drifth(x) return math.max(0, driftenv(x) + 4 * hn(x, 0) + 2.2 * hn2(x, 0) + 0.8 * hn3(x, 0)) end
footd = mask(function(x, y)
  local b = footat(x); if not b then return 0 end
  local top = b - drifth(x)
  return smoothstep(top - 1, top + 1.2, y) * (1 - smoothstep(b + 2, b + 9, y)) * smoothstep(0.3, 1.5, drifth(x))
end):roughen(1.2, 5, 66, 0.8)
local cliptop = mask(function(x, y) local b = footat(x); if not b then return 1 end; return smoothstep(b - drifth(x) - 1, b - drifth(x) + 1.2, y) end):roughen(1.2, 5, 66, 0.8)
local tw = noise{seed=67, period=25}
-- its color is the field's own, a touch lighter on the slope that turns to the low sun,
-- cooler under the shaded flank and where the bank tucks into the rock
work(footd, {hand="body", tool="flat 6", length={12, 30}, coverage=3.4, medium=0.3, load=0.75, angle=0.02, hug=false, clip=cliptop,
  color=function(x, y)
    local b = footat(x) or y
    local top = b - drifth(x)
    local field = sample(x, b + 12, 3)
    local shade = x < 606 and smoothstep(530, 590, x) or (0.45 + 0.3 * smoothstep(630, 700, x))
    local c = shift(field, 0.012 * (1 - shade), 0, 0.004 * (1 - shade))
    c = mix(c, shift(field, -0.07, 0, -0.02), 0.85 * shade)
    -- the crease: a little of the stone's shade on the snow right where it tucks in
    c = mix(c, shift(c, -0.05, 0, -0.015), 0.6 * (1 - smoothstep(top, top + 4, y)))
    return shift(c, 0.01 * tw(x, y), 0, 0)
  end})
blend(footd, {angle=0.02, clip=cliptop})
blend(footd, {angle=0.08, clip=cliptop, coverage=2})
print(cap:area(), footd:area())

--@ chunk 12 · clock 111432.091796875
wait(24*60)
-- the young spruce, dense to the ground, its boughs weighed down a little by snow
yspr = spruce{x=709, top=56, base=492, halfw=104, seed=71, droop=1.35, thick=1.15, lean=0.012}
-- no two whorls alike: a spruce in the open grows lopsided, some boughs long and sweeping,
-- some short or snapped, left and right of one whorl rarely the same, the whorls unevenly spaced
local asym = noise{seed=73, period=70}
for i, b in ipairs(yspr.branches) do
  local ax, ay = b.pts[1][1], b.pts[1][2]
  local f = clamp(1 + 0.3 * asym(ay, b.s * 60) + randn(0, 0.2), 0.5, 1.4)
  if rand() < 0.1 and b.t > 0.2 then f = f * rand(0.35, 0.55) end   -- snapped or stunted
  local sag = randn(0, 0.12)
  for k = 2, #b.pts do
    local p = b.pts[k]
    local dx, dy = (p[1] - ax) * f, (p[2] - ay) * f
    p[1] = ax + dx; p[2] = ay + dy + sag * math.abs(dx)
  end
  b.f = f
end
local tier = (492 - 56) / 22
local m = ribbon(yspr.axis, math.max(1.5, 104 * 0.06))
for _, b in ipairs(yspr.branches) do m = m + ribbon(b.pts, b.w) end
yspr.mask = m:roughen(3.2, 6, 71, 0.4)
local notstone = -v:visible("bodies"):grow(1)
ysm = yspr.mask * notstone
local turn = noise{seed=72, period=10}
local dark = function(x, y) return shift(mix("#1b2620", "#26342a", smoothstep(60, 480, y)), 0.02 * turn(x, y), 0, 0) end
work(ysm, {hand="hatch", tool="round 1.8", length={3, 8}, coverage=3.4, clip=ysm, medium=0.18,
  angle=function(x, y) return (x < 709 and 2.75 or 0.4) + 0.35 * turn(x, y) end, angle_jitter=0.45, color=dark})
-- the boughs drawn out, then the needles at their tips catching the low light on the left
local rb = brush("rigger", 1.1)
for i, b in ipairs(yspr.branches) do
  if i % 5 == 1 then rb:reload("#18211c", 0.8) end
  rb:stroke(b.pts, {pressure={0.9, 0.05}, ramps={0.05, 0.6}, clip=notstone})
end
rb:reload("#2a2420", 0.9)
rb:stroke(yspr.axis, {pressure={0.15, 0.8}, ramps={0.4, 0.1}, clip=notstone})
-- the curtain: short branchlets hanging from every bough in short hatched strokes, so the
-- underside of a tier is a fringe, not a clean chevron edge
local hb = brush("round", 1.2)
local nh = 0
for i, b in ipairs(yspr.branches) do
  if not b.front then
    local p1, p4 = b.pts[1], b.pts[#b.pts]
    local L = math.abs(p4[1] - p1[1])
    local cnt = math.floor(L / 3.2)
    for k = 1, cnt do
      local t = rand(0.2, 0.97)
      -- a point along the bough (piecewise between its points)
      local seg = t * (#b.pts - 1)
      local j = math.min(#b.pts - 1, math.floor(seg) + 1)
      local u = seg - (j - 1)
      local a, c = b.pts[j], b.pts[j + 1]
      local x, y = a[1] + (c[1] - a[1]) * u, a[2] + (c[2] - a[2]) * u
      local hl = tier * rand(0.2, 0.55) * (0.6 + 0.6 * t)
      if nh % 7 == 0 then hb:reload(nh % 14 == 0 and "#18211c" or "#223029", 0.75) end
      nh = nh + 1
      local sw = b.s * rand(-0.5, 1.5)
      hb:stroke({{x, y + 1}, {x + sw * 0.5, y + hl * 0.6}, {x + sw, y + hl}}, {pressure={0.75, 0.05}, ramps={0.05, 0.6}, clip=notstone})
    end
  end
end
-- ragged needle tips spilling past the edge: short strokes that let the silhouette go
work(ysm:rim(4, 1), {hand="hatch", tool="round 1.2", length={2, 5}, coverage=1.3, hug=false, clip=notstone, medium=0.2,
  angle=function(x, y) return (x < 709 and 2.3 or 0.85) + 0.6 * turn(x, y) end, angle_jitter=0.8, color=dark})
local lit = ysm * mask(function(x, y) return smoothstep(700, 640, x) end)
work(lit, {hand="hatch", tool="round 1.2", length={2, 5}, coverage=1.1, clip=ysm, angle=2.8, angle_jitter=0.5, color="#4d5a3a"})
print(nh, "branchlets")

--@ chunk 13 · clock 112872.091796875
-- the stump of a birch, snapped off by a storm: white bark, black scars, a jagged top
stump = outline{{94,712,"c"},{97,660},{100,612},{93,574,"c"},{104,560,"c"},{113,571},{122,556,"c"},{129,566},{139,545,"c"},{146,572},{150,630},{156,712,"c"}, char="broken", seed=81}
stm = stump:mask()
local sx = function(x) return clamp((x - 94) / 62, 0, 1) end
work(stm, {hand="body", tool="filbert 3", length={4, 12}, coverage=3.5, clip=stm, medium=0.15, angle=math.pi/2, angle_jitter=0.2,
  color=function(x, y) return gradient({{0, "#e9e2d2"}, {0.45, "#d6d0c4"}, {0.8, "#9a9aa6"}, {1, "#7c7d8a"}}, sx(x)) end})
-- the splintered wood of the break, warm and raw
local brk = stm * above(function(x) return 578 + 0.2 * (x - 94) end)
-- the break weathered silver-gray, as a snag goes after a winter or two: only a little raw
-- brown left deep in the splits, darker on the side turned from the sun
work(brk, {hand="detail", tool="round 1.4", length={3, 9}, coverage=2.5, clip=stm, angle=-math.pi/2,
  color=function(x, y) return shift(gradient({{0, "#a39a8a"}, {0.5, "#857d70"}, {1, "#5f5a54"}}, sx(x)), 0, 0.004, 0.006 * smoothstep(575, 560, y)) end})
-- the black lenticels and scars, across the trunk; darker toward the root
local bb = brush("round", 1.6)
for i = 1, 26 do
  local y = rand(585, 705)
  local x0 = rand(95, 130)
  local len = rand(4, 16) * (1 + smoothstep(640, 705, y))
  if i % 4 == 1 then bb:reload(i % 8 == 1 and "#1e1c1c" or "#35302e", 0.8) end
  bb:stroke({{x0, y}, {x0 + len * 0.5, y + randn(0, 0.6)}, {x0 + len, y + randn(0, 1)}}, {pressure={0.2, 0.8, 0.1}, ramps={0.3, 0.4}, clip=stm, shake=0.5})
end
local contour = brush("round", 1.3)
contour:load("#4a4644", 0.8)
stump:paint(contour, {pressure=0.7, clip=stm:grow(0.8), dip={"#4a4644", 0.7}, every=3})

--@ chunk 14 · clock 112872.091796875
-- shadows of the spruce and the stump, from the same low sun
local sp = w:spot(709, 492)
local w2 = w:proxy(sp, body.ellipsoid(sp:p(0, 1.6, 0), sp:size(0.75, 1.6, 0.75)))
vs = w2:view()
-- a shadow on snow is not a flat stripe: it deepens on the rises that face the sun, pales in the
-- troughs already turned from it, and its edge is scalloped by the drifts it crosses
local ripple = noise{seed=141, period=22, stretch={0.02, 4}}
local slow = noise{seed=142, period=110}
local mod = mask(function(x, y)
  local p = v:at(x, y)
  local lit = (p and p.what == "ground" and p.shade) and p.shade.value or 0.4
  return clamp(0.35 + 0.5 * smoothstep(0.3, 0.5, lit) + 0.3 * ripple(x, y) + 0.2 * slow(x, y), 0.15, 1.1)
end)
-- the erratic is sunk in the ground, not an egg lying on it: no shadow under a belly in
-- front of its lit face, only the contact shade below
local belly = mask(function(x, y)
  local b = footat(x); if not b then return 0 end
  return smoothstep(b - 6, b + 1, y) * smoothstep(606, 575, x)
end)
local cs = vs:cast_shadow{soft=2.2}
glaze(cs * mod * -stm * -v:visible("bodies") * -belly, {color="#6a7090", coats=0.36, view=vs})
-- the stump's shadow: the stump is 0.6 m, so at an 11 degree sun it reaches ~3 m to the right
-- and away; under the veiled sun its penumbra outgrows a 23 cm stump, so it is dark and
-- crisp at the foot, widens and fades out long before it would end
STSH = {{114, 713.4}, {254, 693.6}, {382, 675.5}, {498, 659.1}, {604, 644.1}, {702, 630.2}, {810, 615}}
local function shy(x)
  for k = 2, #STSH do
    local a, b = STSH[k - 1], STSH[k]
    if x <= b[1] then return a[2] + (b[2] - a[2]) * (x - a[1]) / (b[1] - a[1]), (x - 114) / 696 end
  end
  return STSH[#STSH][2], 1
end
local wob = noise{seed=143, period=28}
local wob2 = noise{seed=144, period=9}
stsh = mask(function(x, y)
  if x < 110 or x > 800 then return 0 end
  local yc, t = shy(x)
  yc = yc - 1.5 + 2.2 * wob(x, 0) + 0.6 * wob2(x, y)
  local hw = 6.5 + 7 * t
  local soft = 1 + 9 * t
  local d = math.abs(y - yc)
  local across = 1 - smoothstep(hw - soft * 0.5, hw + soft * 0.5, d)
  local along = (1 - smoothstep(0.08, 0.62, t)) * smoothstep(122, 140, x)
  return across * along
end) - stm
glaze(stsh * mod, {color="#62688a", coats=0.42})
-- the contact shade runs up into the crease over the drift (the snow tucks under the stone
-- there), so the stone sits in it instead of floating over a stripe
glaze(vs:contact_shadow{reach=0.14}:blur(1.5) * -stm, {color="#4a4a58", coats=0.22, view=vs})
print(stsh:area())

--@ chunk 15 · clock 132464.623046875
-- dry grass standing through the first snow: straw, rust and gray, in upturning strokes laid last
local patch = noise{seed=91, period=90}
local keep = below(function(x) return 372 end) * mask(function(x, y) return smoothstep(0.5, 0.7, patch:at01(x, y)) end)
  - v:visible("bodies"):grow(2) - stm:grow(1) - ysm:grow(2)
local tufts = sward{region=keep, horizon=HZ, near=H, height=34, thin=0.55, seed=92, flowers=0,
  wind={lean=0.22, gust=0.25, period=140, seed=3}}
local g = brush("rigger", 0.7)
local straws = {"#8a7a5c", "#6f6150", "#a39473", "#5a4f44", "#7f7462"}
local n = 0
local pick = noise{seed=93, period=6}
for i, t in ipairs(tufts) do
  if pick:at01(t.x, t.y) > 0.62 or t.scale > 0.75 and pick:at01(t.x, t.y) > 0.45 then
  if i % 3 == 1 then g:reload(straws[1 + (i // 3) % #straws], 0.7) end
  local p = clamp(0.2 + 0.5 * t.scale, 0.15, 0.8)
  for _, bl in ipairs(t.blades) do g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}}); n = n + 1 end
  end
end
print(#tufts, "tufts", n, "blades")

--@ chunk 16 · clock 132464.623046875
wait(3*60)
-- the pale blotches low in the wood: darkened back into the trees
local blot = woodm * rect(0, 280, 680, 40) * mask(function(x, y) return smoothstep(0.35, 0.55, sample(x, y, 1).L) end):grow(2)
work(blot, {hand="hatch", tool="round 1.8", length={3, 7}, coverage=3, clip=woodm, angle=0.3, angle_jitter=0.5, color="#222b26"})
-- snow drifted round the stump's foot
local sd = outline{{70,712},{88,700},{104,706},{122,698},{140,705},{160,699},{178,712}, open=true, char="soft", lobe=6, amount=1.2, seed=101}
local sdm = sd:below(H) * mask(function(x, y) return smoothstep(66, 92, x) * smoothstep(182, 156, x) * smoothstep(742, 722, y) end)
work(sdm, {hand="body", tool="filbert 3", length={4, 12}, coverage=3, hug=false, medium=0.15, angle=0.02,
  color=function(x, y) return mix("#ece4d4", "#b5b7c6", smoothstep(118, 150, x)) end})
-- dead bracken: rust-brown fronds bent over near the stump and at the stone's foot
local fr = brush("rigger", 1.1)
local pin = brush("round", 1.5)
local function frond(x, y, len, ang, bend, seed)
  local pts = {}
  for k = 0, 6 do
    local t = k / 6
    pts[#pts + 1] = {x + len * t * math.cos(ang) + bend * len * t * t, y + len * t * math.sin(ang) + 0.5 * bend * len * t * t}
  end
  fr:reload(({"#7a4e2c", "#8c5d33", "#6a4a32"})[1 + seed % 3], 0.8)
  fr:stroke(pts, {pressure={0.6, 0.1}, ramps={0.05, 0.6}, shake=0.4})
  pin:reload(({"#8a5a30", "#9c6a3a", "#6f4a2e"})[1 + seed % 3], 0.8)
  for k = 2, 6 do
    local p, q = pts[k], pts[k - 1]
    local dx, dy = p[1] - q[1], p[2] - q[2]
    local l = math.sqrt(dx * dx + dy * dy)
    local nx, ny = -dy / l, dx / l
    local pl = len * 0.26 * (1 - (k - 2) / 6)
    for _, s in ipairs{-1, 1} do
      pin:stroke({p, {p[1] + s * nx * pl + dx * 0.3, p[2] + s * ny * pl + dy * 0.3 + 0.25 * pl}}, {pressure={0.55, 0.05}, ramps={0.1, 0.5}, shake=0.3})
    end
  end
end
frond(160, 708, 70, -1.2, 0.5, 1)
frond(174, 714, 58, -0.7, 0.4, 2)
frond(62, 718, 62, -1.9, -0.4, 3)
frond(186, 506, 36, -1.0, 0.45, 4)
frond(202, 510, 30, -0.5, 0.35, 5)
frond(578, 508, 32, -2.2, -0.4, 6)
frond(820, 600, 46, -1.3, 0.4, 7)
-- fallen spruce twigs and a birch twig lying on the snow
local tw = brush("rigger", 0.6)
for i = 1, 9 do
  local x, y = rand(230, 900), rand(560, 750)
  if not (x > 90 and x < 160 and y > 540) then
    tw:reload(i % 3 == 0 and "#3a302a" or "#2a2622", 0.8)
    local a = rand(-0.5, 0.5); local L = rand(10, 24) * (0.6 + (y - 560) / 380)
    local p1 = {x, y}; local p2 = {x + L * math.cos(a), y + L * math.sin(a) * 0.4}
    tw:stroke({p1, {(p1[1] + p2[1]) / 2, (p1[2] + p2[2]) / 2 + rand(-1.5, 1.5)}, p2}, {pressure={0.6, 0.2}, shake=0.6})
    tw:stroke({{(p1[1] + p2[1]) / 2, (p1[2] + p2[2]) / 2}, {(p1[1] + p2[1]) / 2 + L * 0.25, (p1[2] + p2[2]) / 2 - L * 0.18}}, {pressure={0.4, 0.05}})
  end
end

--@ chunk 17 · clock 132644.623046875
local sd = mask(function(x, y) return smoothstep(60, 90, x) * smoothstep(190, 160, x) * smoothstep(750, 725, y) * smoothstep(680, 700, y) end) - stm:shrink(1)
blend(sd, {angle=0.03, clip=-stm})

--@ chunk 18 · clock 132644.623046875
dry()
-- snow lies on some boughs, not on every tier: in lumps where a bough is broad and flat enough to
-- hold it, mostly on the sun side, shaken off elsewhere; never a rim along the whole tier
local lie = noise{seed=111, period=6}
local patch = noise{seed=112, period=26, stretch={0.05, 3}}
local ytops = mask(function(x, y) return ysm:at(x, y) * (1 - ysm:at(x, y - 2.5)) end) * mask(function(x, y)
  return smoothstep(0.3, 0.6, lie:at01(x, y)) * smoothstep(0.4, 0.56, patch:at01(x, y)) * (0.25 + 0.75 * smoothstep(760, 650, x)) end)
stipple(ytops, {width=1.6, color=function(x, y) return mix("#e8e0cf", "#a4a7b6", smoothstep(680, 760, x)) end,
  coverage=2.0, pressure={0.4, 0.8}, drag={1, 0}, aim=false, medium=0.15, fade=0})
print(ytops:area())

--@ chunk 19 · clock 151838.54296875
-- retouch: the hollow in front of the big stone lit again, a thin veil only
local tw = noise{seed=121, period=18}
local lipbot = function(x) return (footat(x) or 505) + 10 end
local front = mask(function(x, y) return smoothstep(lipbot(x) + 3, lipbot(x) + 14, y) * smoothstep(572, 530, y) * smoothstep(180, 225, x) * smoothstep(600, 520, x) end)
work(front, {hand="body", tool="filbert 4", length={8, 22}, coverage=2.2, medium=0.4, load=0.5, angle=0.03, hug=false,
  color_over=function(x, y, under) return shift(mix(under, "#e6ddca", 0.5), 0.012 * tw(x, y), 0, 0) end})
blend(front, {angle=0.02})

--@ chunk 20 · clock 151838.54296875
-- the wood's interior drawn: a back row of spruces a step grayer and cooler than the edge trees,
-- their tiers catching a little sky light, and the trunks standing in the snow at the foot
local nostone = -v:visible("bodies"):grow(3)
local inside = woodm:shrink(5) * below(function(x) return 60 end) * above(function(x) return 338 end) * nostone
local xs = uneven(22, 10, 560, 0.7, 0.45, 9)
local back = {}
for k, ex in ipairs(xs) do
  local tp = 70 + 0.42 * ex + rand(-30, 40)
  back[#back + 1] = spruce{x=ex, top=tp, base=352, halfw=(352 - tp) * rand(0.2, 0.3), seed=150 + k, droop=rand(0.8, 1.2), lean=randn(0, 0.01)}
end
local rb = brush("rigger", 0.9)
local n = 0
for k, e in ipairs(back) do
  rb:reload(k % 2 == 0 and "#2d3834" or "#313c38", 0.6)
  for _, b in ipairs(e.branches) do
    if rand() < 0.6 then
    n = n + 1
    if n % 5 == 0 then rb:reload(k % 2 == 0 and "#2d3834" or "#333e3a", 0.55) end
    rb:stroke(b.pts, {pressure={0.45, 0.05}, ramps={0.05, 0.6}, clip=inside})
    end
  end
end
-- trunks at the foot of the wood, between the lowest boughs
local tb = brush("round", 1.6)
for i, tx in ipairs(uneven(26, 8, 600, 0.7, 0.4, 13)) do
  if i % 4 == 1 then tb:reload(i % 8 == 1 and "#2a2622" or "#3b3630", 0.8) end
  local y0 = 318 + rand(-12, 10)
  tb:stroke({{tx, y0}, {tx + randn(0, 0.6), 349}}, {pressure={0.3, 0.8}, ramps={0.3, 0.1}, clip=woodm * below(function(x) return 300 end) * above(function(x) return 352 end) * nostone})
end
print(n, "back branches")

--@ chunk 21 · clock 151838.54296875
wait(24*60)
-- the wood opened: under the crowns you see in between the trunks to the dim snow of the
-- wood's floor, and the air of the wood deepens behind the edge trees
local nostone = -v:visible("bodies"):grow(3) - ysm:grow(2)
local body_in = woodm:shrink(3) * nostone
local col = noise{seed=171, period=38, stretch={0.03, 4}}
local slow = noise{seed=172, period=150}
airm = body_in * mask(function(x, y)
  return smoothstep(0.45, 0.62, col:at01(x, y)) * smoothstep(313, 321, y) * (1 - smoothstep(348, 356, y))
    * smoothstep(0.25, 0.5, slow:at01(x, y))
end)
work(airm, {hand="body", tool="round 2", length={5, 14}, coverage=3, medium=0.25, load=0.7, hug=false, angle=0.03, angle_jitter=0.15, clip=body_in,
  color=function(x, y) return shift(gradient({{0, "#323b38"}, {0.6, "#4a524f"}, {1, "#6b706f"}}, smoothstep(316, 352, y)), 0.012 * slow(x, y), 0, 0) end})
-- deeper air behind: a veil in vertical columns, a step lighter and cooler, where the back of the wood recedes
local col2 = noise{seed=173, period=34, stretch={math.pi/2, 4}}
local deep = body_in * mask(function(x, y)
  return smoothstep(0.48, 0.7, col2:at01(x, y)) * smoothstep(120, 220, y) * (1 - smoothstep(280, 320, y)) * smoothstep(0.3, 0.55, slow:at01(x + 400, y))
end)
work(deep, {hand="glaze", tool="round 3", length={6, 16}, medium=0.8, coverage=2.4, angle=math.pi/2, angle_jitter=0.15, hug=false, clip=body_in,
  color_over=function(x, y, under) return mix(under, "#46524f", 0.35) end})
-- trunks standing in it, and the lowest boughs sweeping down across the gaps
local fl = woodm * below(function(x) return 200 end) * above(function(x) return 356 end) * nostone
local tb = brush("round", 3.2)
local rb = brush("round", 1.4)
local n = 0
for i, tx in ipairs(uneven(44, 6, 620, 0.75, 0.5, 177)) do
  if i % 3 == 1 then tb:reload(i % 2 == 1 and "#171a18" or "#22241f", 0.85) end
  local y0 = rand(250, 315)
  local lean = randn(0, 1.2)
  tb:stroke({{tx, y0}, {tx + lean * 0.5, (y0 + 352) / 2}, {tx + lean, 353}}, {pressure={0.5, 1}, ramps={0.3, 0.1}, clip=fl})
  if i % 4 == 1 then rb:reload("#1b211e", 0.8) end
  for k = 1, math.random(2, 4) do
    local by = rand(math.max(y0 + 5, 290), 330)
    local s = rand() < 0.5 and -1 or 1
    local L = rand(8, 20)
    rb:stroke({{tx, by}, {tx + s * 0.5 * L, by + 0.25 * L}, {tx + s * L, by + 0.45 * L}, {tx + s * 1.1 * L, by + 0.4 * L}},
      {pressure={0.9, 0.05}, ramps={0.05, 0.6}, clip=fl})
    n = n + 1
  end
end
print(airm:area(), deep:area(), n, "boughs")

--@ chunk 22 · clock 153278.54296875
-- the white specks inside the wood read as screen noise, not snow on boughs under a low sun:
-- hatched back into the dark wherever they aren't on an edge tree's silhouette against the sky
local inner = woodm:shrink(7) * below(function(x) return 20 end) * above(function(x) return 306 end) - ysm:grow(3)
local pale = inner * mask(function(x, y) return smoothstep(0.34, 0.46, sample(x, y, 0.8).L) end):grow(1.8)
local turn = noise{seed=221, period=14}
work(pale, {hand="hatch", tool="round 1.6", length={2, 6}, coverage=3, clip=inner, medium=0.2, angle=function(x, y) return 0.3 + 0.25 * turn(x, y) end,
  angle_jitter=0.5, color=function(x, y) return shift(mix("#1f2824", "#2c3630", smoothstep(40, 300, y)), 0.012 * turn(x, y), 0, 0) end})
print(pale:area())

--@ chunk 23 · clock 153278.54296875
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
