-- easel session "easel4_near": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel4_near.lua [--width 3200]
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
local xs = uneven(17, 14, 657, 0.6, 0.4, 5)
for k, ex in ipairs(xs) do
  local tp = woodtop(ex) + (ex < 300 and rand(-70, 35) or randn(0, 12))
  edge[#edge + 1] = spruce{x=ex, top=tp, base=358, halfw=(358 - tp) * 0.26, seed=20 + k, droop=0.9}
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
