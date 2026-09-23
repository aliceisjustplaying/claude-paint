-- easel session "l2_green": a painting replayed chunk by chunk.
--   easel run paintings/lua/l2_green.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}; print(W,H); print(pal)

--@ chunk 2 · clock 0
HZ = 296
w = world{horizon=HZ, eye=28, fov=52, sun={azimuth=-118, elevation=36},
  ground=function(X, Z) return 1.5*math.sin(X/70+0.4)*math.min(1, Z/300) + 6*math.sin(Z/900 + X/1300) end}
BROW = {{-10,474},{90,466},{190,470},{300,482},{400,494},{520,512},{640,534},{760,552},{880,566},{1010,574}}
RIVER = {{1010,420},{900,398},{780,372},{700,352},{640,338},{600,326},{560,316},{530,308},{505,302}}
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
h:sketch(BROW, {pressure=0.3})
h:sketch(RIVER, {pressure=0.25})
-- oak: trunk and crown envelope
h:sketch({{205,482},{210,420},{206,360},{214,300}}, {pressure=0.3})
h:sketch({{228,482},{226,420},{230,360},{226,300}}, {pressure=0.3})
h:sketch({{120,330},{90,260},{110,180},{170,120},{240,98},{320,130},{352,210},{330,300},{280,340}}, {pressure=0.25})
-- boulder
h:sketch({{378,508},{384,478},{410,458},{446,452},{470,466},{482,496},{476,516}}, {pressure=0.3})
-- seated figure, back to us
h:sketch({{494,512},{492,494},{496,478},{500,470},{506,478},{510,496},{512,512}}, {pressure=0.3})
-- village and spire
h:sketch({{640,322},{660,318},{700,317},{740,319},{770,322}}, {pressure=0.25})
h:rule({708, 318}, {708, 282}, {pressure=0.3})

--@ chunk 3 · clock 0
b2 = pencil("2B")
b2:line(BROW, {pressure={0.5, 0.6, 0.5}})
b2:line({{120,330},{90,260},{110,180},{170,120},{240,98},{320,130},{352,210},{330,300},{280,340}}, {pressure=0.45})
b2:line({{205,482},{210,420},{206,360},{214,300}}, {pressure=0.5})
b2:line({{228,482},{226,420},{230,360},{226,300}}, {pressure=0.5})
b2:line({{378,508},{384,478},{410,458},{446,452},{470,466},{482,496},{476,516}}, {pressure=0.55, smooth=false})
b2:line({{494,512},{492,494},{496,478},{500,470},{506,478},{510,496},{512,512}}, {pressure=0.5})
b2:line(RIVER, {pressure=0.35})
b2:rule({0, HZ}, {1000, HZ}, {pressure=0.35})
show(BROW, {label="brow"})

--@ chunk 4 · clock 0
sk = w:sky{haze=2.0, uneven={0.5, 30000, 5}}
cl = w:clouds{sky=sk, cell=3,
  {kind="cumulus", x=-2600, z=7000, base=1200, width=2600, height=1700, seed=14},
  {kind="cumulus", x=1800, z=11000, base=1300, width=2000, height=1100, seed=6},
  {kind="cumulus", x=5200, z=16000, base=1300, width=2600, height=900, seed=31},
  {kind="bank", x0=-30000, x1=30000, z=40000, depth=9000, base=800, top=1900, seed=4},
  {kind="stratus", base=4200, thick=400, cover=0.25, seed=2, wind={-0.3, 2}}}
SKYPAL = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
local skym = above(function(x) return HZ + 14 end)
work(skym, {hand="broad", color=cl, angle=0, coverage=4.5, medium=0.3, pal=SKYPAL})
blend(skym, {angle=0})

--@ chunk 5 · clock 0
wait(24*60)
local top = above(function(x) return HZ - 30 end)
local litm = (cl:mask{alpha={0.45, 0.85}, lit={0.5, 0.9}} * top):blur(3)
work(litm, {hand="scumble", color="#f1eadb", coverage=2.2, medium=0.35, angle=function(x,y) return -0.3 + 0.4*math.sin(x/40) end, length={6, 18}, hug=false, pal=SKYPAL})
local shm = (cl:mask{alpha={0.5, 0.95}, shade={0.4, 0.85}} * top):blur(4)
work(shm, {hand="glaze", color_over={shift={-0.05, 0, -0.012}}, coverage=2, medium=0.8, angle=0, length={10, 30}, hug=false})
air = haze{visibility=22000, height=900, mist={60, 4, 900, 7}}
rs = w:ranges{near=9000, far=26000, count=3, seed=19, heights={160, 620}, kinds={"dome", "saddle", "plateau"}}
for i = #rs, 1, -1 do
  local l = rs[i]
  work(l:mask() * above(function(x) return HZ + 3 end), {hand="body", length={20, 60}, coverage=3.5, angle=0.04,
    color=function(x, y) return mix(mix("#44566a", "#5c6d86", (i-1)/2), sk:airlight(x), 0.8*l:haze(air, x, y)) end, medium=0.3})
end
for i, l in ipairs(rs) do print(i, l:crest(200), l:crest(600), l:crest(900)) end

--@ chunk 6 · clock 1440

local top = above(function(x) return HZ - 24 end)
local shm = (cl:mask{alpha={0.3, 0.95}} * top):grow(4):blur(4)
blend(shm, {angle=function(x,y) return 0.25*math.sin(x/70) end, coverage=3})

--@ chunk 7 · clock 1440
fields = worley{seed=5, period=1}
fnz = noise{seed=9, octaves=4, period=90}
FIELDC = {"#5d7a33", "#6b8538", "#4f6b2c", "#7d8f3f", "#a39a52", "#6f7a36", "#58743a", "#8c8a4a"}
function valley(x, y)
  local p = w:to_ground(x, y); if not p then return sk:airlight(x) end
  local X, Z = p[1], p[3]
  local ca, sa = math.cos(0.45), math.sin(0.45)
  local U, V = X*ca - Z*sa, X*sa + Z*ca
  local _, _, edge, r = fields:at(U/55, V/95)
  local c = FIELDC[1 + math.floor(r * #FIELDC) % #FIELDC]
  c = shift(c, 0.03*fnz(x, y) - (edge < 0.06 and 0.05 or 0), 0, 0)
  return mix(c, sk:airlight(x), clamp(w:aerial(Z) * 0.95, 0, 0.9))
end
local land = below(function(x) return HZ end)
work(land, {hand="body", length={20, 60}, coverage=3.5, angle=function(x, y) return 0.03*math.sin(x/90) end, color=valley, medium=0.25})

--@ chunk 8 · clock 1440
wait(24*60)
RIVW = {{160, 170}, {120, 260}, {40, 360}, {-10, 480}, {30, 640}, {110, 820}, {60, 1050}, {-80, 1300}, {-160, 1700}, {-120, 2300}, {-260, 3300}, {-420, 4600}}
river = w:ribbon(RIVW, 16)
work(river, {hand="body", tool="round 2.4", length={5, 14}, coverage=2.6, angle=0, medium=0.35, load=0.45,
  color=function(x, y) local p = w:to_ground(x, y); local Z = p and p[3] or 3000
    return mix(mix("#8e9fb2", "#c3cbd0", smoothstep(300, 2500, Z)), sk:airlight(x), 0.5*w:aerial(Z)) end})
-- hedgerows and copses along some field edges
hedge = mask(function(x, y)
  if y < HZ + 2 then return 0 end
  local p = w:to_ground(x, y); if not p then return 0 end
  local X, Z = p[1], p[3]
  local ca, sa = math.cos(0.45), math.sin(0.45)
  local U, V = X*ca - Z*sa, X*sa + Z*ca
  local _, _, edge, r = fields:at(U/55, V/95)
  if Z < 280 then return 0 end
  local keep = (fnz(X*0.9, Z*0.35) > 0.05 and fnz(X*4.1 + 77, Z*1.3) > -0.25) and 1 or 0
  local _, _, _, r2 = fields:at(U/55 + 0.37, V/95 + 0.61)
  local copse = (r2 > 0.94) and 1 or 0
  return math.max(keep * (1 - smoothstep(0.02, 0.05, edge)), copse * (1 - smoothstep(0.1, 0.25, edge)))
end) - river
work(hedge, {hand="hatch", tool="round 1.6", length={2, 5}, coverage=2.5, angle=function(x,y) return 2.2*fnz(x*3,y*3) end, medium=0.25,
  color=function(x, y) local p = w:to_ground(x, y); local Z = p and p[3] or 3000
    return mix("#34472a", sk:airlight(x), clamp(w:aerial(Z)*0.9, 0, 0.85)) end})

--@ chunk 9 · clock 2880
local s = w:spot(706, 318); local u = s.x and w:height(s.x, s.y, 1) or 0.8
print("units/m", u, s)
local air = sk:airlight(706)
local function hz(c) return mix(c, air, 0.42) end
local walls, roofs, shade = nil, nil, nil
local function add(a, m) if a then return a + m else return m end end
local hs = {{648, 321, 12, 7, 6}, {664, 320, 10, 6, 6}, {678, 319, 14, 7, 7}, {730, 319, 12, 7, 6}, {746, 320, 10, 6, 5}, {760, 321, 13, 7, 6}, {688, 320, 9, 6, 5}}
for _, h in ipairs(hs) do
  local x, b, wd, ht, rf = h[1], h[2], h[3]*u, h[4]*u, h[5]*u
  walls = add(walls, rect(x, b - ht, wd, ht))
  shade = add(shade, rect(x + wd*0.62, b - ht, wd*0.38, ht))
  roofs = add(roofs, poly({{x - 1, b - ht}, {x + wd*0.2, b - ht - rf}, {x + wd*0.8, b - ht - rf}, {x + wd + 1, b - ht}}))
end
-- church: nave and tower with spire
local cx, cb = 706, 318
local nave = rect(cx - 2, cb - 11*u, 26*u, 11*u)
local tower = rect(cx - 5*u, cb - 30*u, 7*u, 30*u)
local spire = poly({{cx - 5.6*u, cb - 30*u}, {cx - 1.5*u, cb - 50*u}, {cx - 1.2*u, cb - 50*u}, {cx + 2.6*u, cb - 30*u}})
local naveroof = poly({{cx + 2*u, cb - 11*u}, {cx + 5*u, cb - 18*u}, {cx + 24*u, cb - 18*u}, {cx + 26*u, cb - 11*u}})
CHURCH = nave + tower + spire + naveroof
VILLAGE = walls + roofs + CHURCH
work(walls + nave + tower, {hand="detail", tool="round 1.2", length={1.5, 4}, coverage=3, angle=1.57, color=hz("#d9ccaa"), medium=0.2})
work(shade + rect(cx + 0.5*u, cb - 30*u, 1.5*u, 30*u), {hand="detail", tool="round 1", length={1.5, 3}, coverage=2.5, angle=1.57, color=hz("#8d8a82"), medium=0.2})
work(roofs + naveroof, {hand="detail", tool="round 1.2", length={1.5, 4}, coverage=3, angle=0, color=hz("#8a4f38"), medium=0.2})
work(spire, {hand="detail", tool="round 1", length={1.5, 3}, coverage=3, angle=1.57, color=hz("#4d5a5e"), medium=0.2})
-- a few trees among the houses
local tm = nil
for _, t in ipairs({{642, 318, 6}, {700, 316, 5}, {722, 317, 6}, {770, 318, 7}, {658, 316, 4}}) do
  tm = add(tm, ellipse(t[1], t[2] - t[3]*0.6, t[3]*0.8, t[3]*0.7):roughen(1.2, 4, t[1]))
end
tm = tm - CHURCH
work(tm, {hand="hatch", tool="round 1.2", length={1.5, 3}, coverage=2.6, angle=function(x,y) return 2.2*fnz(x*4,y*4) end, color=hz("#3a4b2c"), medium=0.2})

--@ chunk 10 · clock 2880
BROW2 = {{-12,466},{80,457},{170,461},{260,471},{350,485},{420,489},{500,497},{580,515},{660,540},{740,566},{830,589},{920,604},{1012,612}}
hillo = outline{pts=BROW2, open=true, char="soft", lobe=5, amount=0.6, seed=12}
HILL = hillo:below(H + 20)
hn = noise{seed=21, octaves=5, period=70}
function hillc(x, y)
  local d = y - (466 + (x/1000)*140)
  local c = gradient({{0, "#8f9a4c"}, {0.25, "#76883e"}, {0.6, "#566b30"}, {1, "#44572a"}}, clamp(d/220, 0, 1))
  return shift(c, 0.035*hn(x, y), 0.004*hn(x+300, y), 0.01*hn(x, y+500))
end
work(HILL, {hand="body", length={12, 34}, coverage=3.5, color=hillc, medium=0.2,
  angle=function(x, y) return 0.12 + 0.35*hn(x*0.5, y*0.5) end})

--@ chunk 11 · clock 2880
wait(120)
local wn = noise{seed=33, octaves=3, period=110}
local top = {}
local x = -12
while x < 1030 do
  local by = 466 + (x/1000)*146
  local g = wn:at01(x, 0)
  local rise = (g < 0.24) and (5 + 12*g) or (24 + 110*(g - 0.24)^1.3 + rand(0, 18))
  rise = rise * (0.75 + 0.5*smoothstep(200, 900, x))
  top[#top+1] = {x, by - rise}
  x = x + rand(14, 30)
end
woodo = outline{pts=top, open=true, char="soft", lobe=11, amount=1.2, seed=41}
WOOD = woodo:below(H + 20) - HILL:grow(1)
local tn = noise{seed=44, period=14}
work(WOOD, {hand="hatch", tool="round 2", length={3, 7}, coverage=3, angle=function(x,y) return 2.4*tn(x,y) end, medium=0.2,
  color=function(x, y) return mix("#2a3825", "#3b4a31", wn:at01(x*3, y*3)) end})
local cells = worley{seed=8, period=16}
local lit = WOOD * mask(function(x, y)
  local f1, f2, edge, r = cells:at(x + 5, y + 6)
  local g = cells:at(x, y)
  return smoothstep(0.3, 0.05, (f1 or 1)) * (r > 0.25 and 1 or 0.3)
end)
work(lit, {hand="hatch", tool="round 1.5", length={2, 4}, coverage=2.2, angle=function(x,y) return 2.4*tn(x+9,y) end, medium=0.2,
  color=function(x, y) return mix("#51663a", "#6c7f45", wn:at01(x, y)) end})

--@ chunk 12 · clock 3000
wait(180)
oak = tree{habit="oak", x=215, y=494, height=380, seed=3}
lv = oak:foliage{sun={-0.7, -0.6, 0.4}, seed=3}
local thick
for _, l in ipairs(oak.limbs) do
  if #l.pts >= 2 and l.w[1] >= 3 then
    local r = ribbon(l.pts, l.w)
    thick = thick and (thick + r) or r
  end
end
OAKWOOD = thick
local bn = noise{seed=5, period=6, stretch={1.5, 3}}
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, medium=0.2, angle=1.5,
  color=function(x, y) return shift(mix("#3b342c", "#2c2823", smoothstep(300, 490, y)), 0.03*bn(x, y), 0, 0) end, clip=thick})
local litside = thick * mask(function(x, y) return 1 - OAKWOOD:at(x - 3.5, y + 1.5) end)
work(litside, {hand="detail", tool="round 1.2", length={3, 9}, coverage=2.4, medium=0.2, angle=1.5,
  color=function(x, y) return shift("#8a8270", 0.04*bn(x, y), 0, 0) end, clip=litside})
local limb, twig = brush("round", 1.8), brush("rigger", 0.6)
for i, l in ipairs(oak.limbs) do
  if #l.pts >= 2 and l.w[1] < 3 then
    local b = (l.w[1] > 1.0) and limb or twig
    if i % 5 == 1 or b:fullness() < 0.3 then b:reload("#3a342d", 0.9) end
    b:stroke(l.pts, {pressure={clamp(0.35 + 0.25*l.w[1], 0.3, 0.95), 0.15}, ramps={0.03, 0.5}})
  end
end

--@ chunk 13 · clock 3180
wait(24*60)
-- the trunk again, over the set hill, so the green no longer muddies it
local lower = OAKWOOD * rect(0, 330, 1000, 200)
work(lower, {hand="body", tool="round 2", length={4, 10}, coverage=3, medium=0.15, angle=1.5, load=0.9,
  color=function(x, y) return mix("#3d362d", "#2a2621", smoothstep(380, 494, y)) end, clip=lower})
local ll = lower * mask(function(x, y) return 1 - OAKWOOD:at(x - 3.5, y + 1.5) end)
work(ll, {hand="detail", tool="round 1.2", length={3, 8}, coverage=2.4, medium=0.15, angle=1.5, color="#857c68", clip=ll})
local turn = noise{seed=7, period=9}
local way = function(x, y) return 2.4 * turn(x, y) end
CROWN = lv:mask()
work(CROWN, {hand="hatch", tool="round 1.8", length={3, 7}, coverage=2.6, angle=way, angle_jitter=0.6, medium=0.2,
  color=function(x, y) return mix("#26321f", "#2f3c25", (y - 120)/300) end})
work(CROWN * lv:lit():soften(2), {hand="hatch", tool="round 1.5", length={2, 6}, coverage=2.2, angle=way, angle_jitter=0.6, medium=0.2,
  color=function(x, y) return mix("#4a5e2e", "#3f5229", (y - 120)/300) end})

--@ chunk 14 · clock 4620
wait(2*24*60)
stone = body.ellipsoid({432, 486, 40}, {56, 36, 42}):turn({432, 486, 40}, 0.4, 0.15, -0.08):rough(5, 60, 3)
  :cut({432, 458, 40}, {-0.3, -1, 0.4}, 10, 3):cut({470, 480, 40}, {1, -0.3, 0.3}, 11, 3):rough(0.8, 12, 4, true)
fs = form{ {stone, dist=0.1}, light={from={-1, -0.75}, front=0.4, ambient=0.25, penumbra=0.06} }
local seen = fs:silhouette{parts={1}, soft=0.4} * above(function(x) return 506 + 3*math.sin(x/7) end)
STONE = seen
local sn = noise{seed=12, period=8}
local function stonec(x, y, lt, dk)
  local v = fs:value(x, y) or 0.4
  return shift(mix(dk, lt, smoothstep(0.08, 0.85, v)), 0.035*sn(x, y), 0.004*sn(x+40, y), 0.008*sn(x, y+40))
end
work(seen * fs:shadow{parts={1}}, {hand="body", tool="round 2.5", length={4, 12}, coverage=3.2, medium=0.2, angle=fs:field("fall"),
  color=function(x, y) return stonec(x, y, "#77736a", "#3f3d3a") end})
work(seen * fs:lit{parts={1}}, {hand="body", tool="round 2.5", length={4, 12}, coverage=3.2, medium=0.2, angle=fs:field("across"),
  color=function(x, y) return stonec(x, y, "#b4ab95", "#716b60") end})
work(fs:edges{concave=true} * seen, {hand="detail", tool="round 1", color="#2f2c29", angle=fs:field("edge"), coverage=1.4, medium=0.2})
-- the wanderer on the brow, back to us, looking out over the valley
local x, y, k = 348, 488, 1.3
local function F(dx, dy) return {x + k*dx, y + k*dy} end
fig = body_of{spine={F(0, -37), F(0, -33), F(0.5, -24), F(1, -13)}, widths={4.2*k, 7.5*k, 7.2*k, 9.5*k},
  limbs={{F(-2, -13), F(-2.2, -6), F(-2.4, 0), widths={2.6*k, 2.2*k, 2*k}}, {F(2.2, -13), F(2.8, -6), F(3.2, 0), widths={2.6*k, 2.2*k, 2*k}},
         {F(-3.5, -33), F(-4.6, -26), F(-4, -20), widths={2.4*k, 2.1*k, 1.7*k}}, {F(3.6, -33), F(4.4, -26), F(5.6, -21), widths={2.4*k, 2.1*k, 1.7*k}},
         {F(-0.2, -39.5), F(0.2, -40), widths={5.8*k, 5.8*k}}},
  blend=0.5, char="firm", seed=31}
FIG = fig:mask()
work(FIG, {hand="body", tool="round 1", length={1.5, 4}, coverage=3, medium=0.15, angle=1.5, clip=FIG,
  color=function(px, py) return mix("#2e3136", "#23211f", (py - y + 52)/52) end})
local lit = fig:band(1.2, 0.5) * FIG * rect(0, 0, x - 1, 714)
work(lit, {hand="detail", tool="round 0.8", length={1, 3}, coverage=2, medium=0.15, angle=1.5, color="#5a5a58", clip=lit})

--@ chunk 15 · clock 7500
wait(24*60)
-- the oak's shadow on the brow, falling right and away
local shn = noise{seed=51, period=18}
local osh = (poly({{205,496},{240,486},{300,480},{360,478},{410,482},{400,494},{330,500},{260,504},{215,504}}, true)
  * HILL):roughen(5, 16, 7, 3) - STONE - FIG
work(osh, {hand="body", tool="filbert 4", length={8, 20}, coverage=2.6, angle=0.05, medium=0.35,
  color_over={shift={-0.075, -0.004, -0.02}}})
-- the stone's cast shadow on the grass, to its right
local ssh = (ellipse(500, 504, 42, 6):roughen(2, 8, 3, 2) * HILL) - STONE
work(ssh, {hand="body", tool="filbert 3", length={6, 14}, coverage=2.5, angle=0.1, medium=0.35, color_over={shift={-0.08, -0.004, -0.02}}})
-- the near hill falls into a cooler, darker band toward the bottom edge
local low = HILL * mask(function(x, y) return smoothstep(560, 714, y + 30*shn(x, y)) end)
work(low, {hand="glaze", tool="filbert 8", length={20, 50}, coverage=2, angle=0.1, medium=0.8, color_over={shift={-0.06, -0.006, -0.012}}})
-- the stone: cracks, a darker belly, lichen
local belly = STONE * mask(function(x, y) return smoothstep(484, 504, y) end)
work(belly, {hand="glaze", tool="round 2", length={3, 8}, coverage=2, angle=0, medium=0.8, color_over={shift={-0.08, 0, -0.01}}})
local crack = brush("rigger", 0.5)
crack:load("#2c2a27", 0.8)
for _, c in ipairs({{{404,470},{414,476},{420,486},{419,496}}, {{452,460},{448,470},{455,478}}, {{386,484},{396,486}}, {{430,496},{442,493},{452,499}}}) do
  crack:stroke(c, {pressure={0.7, 0.2}, ramps={0.1, 0.4}, shake=0.6})
end
stipple(STONE * fs:lit{parts={1}}, {width=1.4, color="#b9b27a", coverage=function(x, y) return 0.08 + 0.12*(sn and 0 or 0) end, cluster={0.3, 4}, fade=0, medium=0.4})
stipple(STONE, {width=1.1, color="#5e6a4a", coverage=function(x, y) return 0.06 end, cluster={0.5, 5}, fade=0, medium=0.4})

--@ chunk 16 · clock 8940
wait(6*60)
local tufts = sward{region=HILL:shrink(2), horizon=430, near=H + 60, height=34, flowers=0.06, seed=7, smallest=1.2,
  wind={lean=0.12, gust=0.2, period=180, seed=3}}
local g = brush("rigger", 0.7)
local sunlit = {"#8c9a48", "#7a8c3e", "#9aa252", "#6b7f36"}
local shaded = {"#4a5c2c", "#3f5128", "#56683a", "#34452a"}
local n = 0
for i, t in ipairs(tufts) do
  if i % 3 == 1 then
    local x, y = t.x, t.y
    local dark = y > 600 and (i % 2 == 0) or (i % 5 == 0)
    local pal4 = dark and shaded or sunlit
    g:reload(pal4[1 + (i // 3) % 4], 0.7)
  end
  local p = clamp(0.25 + 0.55 * t.scale, 0.2, 0.95)
  for _, bl in ipairs(t.blades) do
    g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}})
    n = n + 1
  end
end
local f = brush("round", 1.2)
local k = 0
for _, t in ipairs(tufts) do
  if t.flower then
    if k % 5 == 0 then f:reload(({"#ece6cf", "#d8b43a", "#9aa6c8", "#ece6cf", "#c9a13a"})[1 + t.flower.kind % 5], 0.8) end
    f:touch(t.flower.x, t.flower.y, {pressure=clamp(t.flower.r / 2, 0.2, 0.8)})
    k = k + 1
  end
end
print(#tufts, "tufts", n, "blades", k, "flowers")

--@ chunk 17 · clock 9300
-- the village farther into the morning haze
glaze(VILLAGE:grow(1.5):soften(1), {color=sk:airlight(706), coats=0.35, pigment="semi"})
-- field trees in loose groups along the hedges: each crown drawn, sizes and habits varied, one soft shadow per group
local groups = {{588,421,4},{846,452,3},{908,441,1},{760,402,2},{690,385,3},{524,398,1},{872,405,5},{728,362,2},{415,404,2},{70,398,4},{960,470,2}}
local tb = brush("rigger", 0.7)
FIELDTREES = nil
for gi, gp in ipairs(groups) do
  local gx, gy, cnt = gp[1], gp[2], gp[3]
  local p = w:to_ground(gx, gy); local Z = p and p[3] or 800
  local hazet = clamp(w:aerial(Z) * 0.9, 0, 0.8)
  local air = sk:airlight(gx)
  local base = 1.3 + (gy - 340) * 0.06
  local crowns, lits, sh = nil, nil, nil
  for k = 1, cnt do
    local x = gx + (k - (cnt + 1) / 2) * base * rand(1.0, 1.9) + randn(0, base * 0.3)
    local y = gy + randn(0, base * 0.15)
    local r = base * rand(0.55, 1.35)
    local tall = rand(0, 1) < 0.3 and rand(1.5, 2.1) or rand(0.85, 1.15)   -- some narrow and tall, most broad
    local rx, ry = r * (tall > 1.4 and 0.6 or rand(0.85, 1.2)), r * tall
    local cy = y - r * 0.5 - ry
    local pts = {}
    local np = 7
    for j = 0, np - 1 do
      local a = j / np * 2 * math.pi + randn(0, 0.2)
      local rr = rand(0.75, 1.15)
      pts[#pts + 1] = {x + math.cos(a) * rx * rr, cy + math.sin(a) * ry * rr * (math.sin(a) > 0 and 0.8 or 1)}
    end
    local cm = outline{pts=pts, closed=true, char="soft", lobe=math.max(1.2, r * 0.45), amount=0.9, seed=gi * 10 + k}:mask()
    crowns = crowns and (crowns + cm) or cm
    local lm = cm * ellipse(x - rx * 0.45, cy - ry * 0.4, rx * 0.8, ry * 0.75):soften(r * 0.3)
    lits = lits and (lits + lm) or lm
    -- a bit of trunk under the broad ones
    if tall < 1.4 and r > 3 then
      tb:reload(mix("#3a332b", air, hazet), 0.7)
      tb:stroke({{x, y + 0.5}, {x + randn(0, 0.3), cy + ry * 0.6}}, {pressure={0.6, 0.3}})
    end
    local s = ellipse(x + r * 0.6 + rx * 0.4, y + 0.6, rx * rand(0.9, 1.4), math.max(0.8, r * 0.18))
    sh = sh and (sh + s) or s
  end
  sh = (sh:roughen(base * 0.2, base, gi, base * 0.3)) - crowns
  work(sh, {hand="detail", tool="round 1", length={2, 5}, coverage=1.6, angle=0, hug=false, medium=0.3,
    color_over={shift={-0.05 * (1 - hazet), -0.003, -0.012}}})
  local dk = mix(mix("#2d3627", "#3a3d2b", rand(0, 1)), air, hazet)
  local lt = mix(mix("#58603e", "#646a45", rand(0, 1)), air, hazet)
  work(crowns, {hand="hatch", tool="round 1.1", length={1.2, 3}, coverage=2.6, angle=function(px, py) return 2.2*fnz(px*5, py*5) end, color=dk, medium=0.25, clip=crowns:grow(0.6)})
  work(lits, {hand="detail", tool="round 0.9", length={1, 2.2}, coverage=1.8, angle=function(px, py) return 0.6 + 1.2*fnz(px*6, py*6) end, color=lt, medium=0.25, hug=false})
  FIELDTREES = FIELDTREES and (FIELDTREES + crowns) or crowns
end
print("field trees", FIELDTREES:area())
-- a pale lane winding down to the river
local lp = {}
for _, c in ipairs({{296, 460}, {322, 440}, {312, 418}, {340, 398}, {400, 381}, {446, 366}, {478, 354}}) do
  local p = w:to_ground(c[1], c[2]); lp[#lp+1] = {p[1], p[3]}
end
local lane = w:ribbon(lp, 3.5)
work(lane - WOOD, {hand="detail", tool="round 1.2", length={3, 8}, coverage=2.4, angle=0, color=function(x, y)
  local p = w:to_ground(x, y); local Z = p and p[3] or 800; return mix("#b7a87e", sk:airlight(x), w:aerial(Z)*0.8) end, medium=0.25})

--@ chunk 18 · clock 50973.75
wait(12*60)
-- the lane: darker, greener, less even
local lp = {}
for _, c in ipairs({{296, 460}, {322, 440}, {312, 418}, {340, 398}, {400, 381}, {446, 366}, {478, 354}}) do
  local p = w:to_ground(c[1], c[2]); lp[#lp+1] = {p[1], p[3]}
end
LANE = w:ribbon(lp, 4.5)
work(LANE - WOOD, {hand="glaze", tool="round 2", length={3, 8}, coverage=2, angle=0, medium=0.7, hug=false,
  color_over={shift={-0.08, -0.012, -0.004}}})
-- the stone: a warmer, darker granite
work(STONE, {hand="glaze", tool="round 2", length={3, 8}, coverage=2.2, angle=0.3, medium=0.75, color_over={shift={-0.07, 0.002, 0.008}}})
-- fine grass on the brow around the figure and the stone
local g = brush("rigger", 0.5)
local brow = (HILL * rect(170, 466, 400, 50)) - STONE - FIG
local gn = noise{seed=61, period=20}
for i = 1, 900 do
  local x, y = rand(170, 570), rand(470, 516)
  if brow:at(x, y) > 0.5 then
    if i % 12 == 1 then g:reload(({"#7f8f40", "#5c7032", "#95a050", "#4a5c2c"})[1 + (i // 12) % 4], 0.6) end
    local h = 2 + (y - 466) * 0.12 + rand(0, 2)
    local lean = randn(0.1, 0.25)
    g:stroke({{x, y}, {x + lean*h*0.5, y - h*0.55}, {x + lean*h, y - h}}, {pressure={0.4, 0}, ramps={0.05, 0.7}})
  end
end
-- a stand of tall seeding grass at the lower left
local st = brush("rigger", 0.8)
for i = 1, 70 do
  local x = 20 + rand(0, 150) + 30*math.sin(i)
  local y = 714 + rand(-8, 10)
  local h = rand(70, 150)
  local lean = randn(0.15, 0.15)
  local tip = {x + lean*h, y - h}
  if i % 8 == 1 then st:reload(({"#8e9448", "#6f7c3a", "#a6a061", "#58683a"})[1 + (i // 8) % 4], 0.8) end
  st:stroke({{x, y}, {x + lean*h*0.4, y - h*0.5}, tip}, {pressure={0.55, 0.05}, ramps={0.05, 0.7}})
  if i % 3 == 0 then
    local hb = brush("round", 1.1); hb:load(({"#b9a86a", "#9a8f55", "#c7b884"})[1 + i % 3], 0.8)
    for j = 0, 5 do
      local t = j / 6
      hb:touch(tip[1] - lean*h*0.12*t + randn(0, 1.2), tip[2] + h*0.12*t, {pressure=0.45, drag={0.2, 1}, angle=1.4})
    end
  end
end
-- a thistle at the lower right
local tx, ty = 846, 714
local stem = brush("round", 1.8); stem:load("#56643a", 0.9)
local heads = {}
for _, s in ipairs({{{tx, ty}, {tx + 4, ty - 60}, {tx - 2, ty - 120}}, {{tx + 3, ty - 50}, {tx + 18, ty - 84}, {tx + 26, ty - 106}}, {{tx + 1, ty - 80}, {tx - 16, ty - 98}, {tx - 24, ty - 110}}}) do
  stem:stroke(s, {pressure={0.8, 0.4}, ramps={0.05, 0.3}})
  heads[#heads + 1] = s[#s]
end
local lf = brush("round", 2.2)
for j = 1, 9 do
  lf:reload(j % 2 == 0 and "#4e6036" or "#6c7d48", 0.8)
  local sy = ty - 10 - j * 9
  local dir = (j % 2 == 0) and 1 or -1
  lf:stroke({{tx + 2, sy}, {tx + dir*14, sy - 6 + randn(0, 2)}, {tx + dir*24, sy + 2 + randn(0, 3)}}, {pressure={0.9, 0.05}, ramps={0.1, 0.6}, shake=1.5})
end
for _, hd in ipairs(heads) do
  local hb = brush("round", 3); hb:load("#5a6a3c", 0.9)
  hb:touch(hd[1], hd[2] + 3, {pressure=0.9})
  local pb = brush("rigger", 0.6); pb:load("#9a5f86", 0.9)
  for k = 1, 9 do pb:stroke({{hd[1] + randn(0, 1.5), hd[2]}, {hd[1] + randn(0, 4), hd[2] - rand(4, 8)}}, {pressure={0.6, 0.1}}) end
end
-- small stones in the grass
local sb = brush("round", 2.4)
for _, s in ipairs({{520, 510, 3}, {531, 512, 2}, {372, 505, 2.5}, {612, 572, 4}, {300, 598, 5}, {690, 640, 3}}) do
  sb:reload("#8d8677", 0.8); sb:touch(s[1], s[2], {pressure=0.8, drag={s[3], 0}})
  sb:reload("#4a4640", 0.6); sb:touch(s[1] + s[3]*0.4, s[2] + 1, {pressure=0.5, drag={s[3]*0.8, 0}})
end

--@ chunk 19 · clock 51693.75
wait(24*60)
-- the thistle, restated dark against the light grass, larger
local tx, ty = 852, 718
local stem = brush("round", 2.4)
local heads = {}
local stems = {{{tx, ty}, {tx + 5, ty - 90}, {tx - 3, ty - 178}}, {{tx + 4, ty - 80}, {tx + 24, ty - 128}, {tx + 36, ty - 158}},
               {{tx + 1, ty - 118}, {tx - 22, ty - 142}, {tx - 34, ty - 160}}, {{tx + 3, ty - 60}, {tx + 30, ty - 82}, {tx + 44, ty - 104}}}
for _, s in ipairs(stems) do
  stem:reload("#2f3a24", 0.9)
  stem:stroke(s, {pressure={0.9, 0.45}, ramps={0.05, 0.3}})
  heads[#heads + 1] = s[#s]
end
local lf = brush("round", 3)
for j = 1, 11 do
  local sy = ty - 8 - j * 11
  local dir = (j % 2 == 0) and 1 or -1
  local L = 30 - j * 1.4
  local pts = {{tx + 3, sy}, {tx + dir*L*0.35, sy - 8 + randn(0, 2)}, {tx + dir*L*0.7, sy - 2 + randn(0, 2)}, {tx + dir*L, sy + 6 + randn(0, 3)}}
  lf:reload("#34422a", 0.9)
  lf:stroke(pts, {pressure={1, 0.05}, ramps={0.1, 0.6}, shake=2})
  local e = brush("rigger", 0.6); e:load("#8c9a5a", 0.8)
  e:stroke({{pts[1][1], pts[1][2] - 2}, {pts[2][1], pts[2][2] - 2}, {pts[3][1], pts[3][2] - 1}}, {pressure={0.5, 0.1}, shake=1.2})
  -- spines
  local sp = brush("rigger", 0.4); sp:load("#c8c3a0", 0.6)
  for q = 2, 4 do sp:stroke({{pts[q][1], pts[q][2]}, {pts[q][1] + dir*3, pts[q][2] - 3}}, {pressure={0.4, 0}}) end
end
for _, hd in ipairs(heads) do
  local hb = brush("round", 4); hb:load("#3e4a2c", 0.9)
  hb:touch(hd[1], hd[2] + 4, {pressure=0.9, drag={0, 2}})
  local lit = brush("round", 1.2); lit:load("#7d8a50", 0.8); lit:touch(hd[1] - 2, hd[2] + 3, {pressure=0.6})
  local pb = brush("rigger", 0.7); pb:load("#8e4d7a", 0.95)
  for k = 1, 14 do pb:stroke({{hd[1] + randn(0, 1.8), hd[2] + 1}, {hd[1] + randn(0, 5), hd[2] - rand(5, 10)}}, {pressure={0.7, 0.1}}) end
  local pl = brush("rigger", 0.5); pl:load("#c690b4", 0.9)
  for k = 1, 5 do pl:stroke({{hd[1] - 1 + randn(0, 1), hd[2]}, {hd[1] - 2 + randn(0, 3), hd[2] - rand(5, 9)}}, {pressure={0.5, 0.1}}) end
end
-- seeding grass at the lower left, pale straw against the dark band
local st = brush("rigger", 0.9)
for i = 1, 55 do
  local x = 15 + rand(0, 170)
  local y = 716 + rand(-6, 8)
  local h = rand(80, 170)
  local lean = randn(0.12, 0.14)
  local tip = {x + lean*h, y - h}
  if i % 5 == 1 then st:reload(({"#b3a76a", "#8f8c4e", "#c4b886", "#6f7a42"})[1 + (i // 5) % 4], 0.85) end
  st:stroke({{x, y}, {x + lean*h*0.35, y - h*0.5}, tip}, {pressure={0.6, 0.05}, ramps={0.05, 0.7}})
  if i % 2 == 0 then
    local hb = brush("round", 1.3); hb:load(({"#cdbf8a", "#a89a60", "#d9cfa4"})[1 + i % 3], 0.9)
    for j = 0, 6 do
      local t = j / 7
      hb:touch(tip[1] - lean*h*0.14*t + randn(0, 1.4), tip[2] + h*0.14*t, {pressure=0.5, drag={0.2, 1.4}, angle=1.4})
    end
  end
end

--@ chunk 20 · clock 53133.75
local gn = noise{seed=71, octaves=3, period=140}
local band = HILL * mask(function(x, y) return 0.85 * smoothstep(520, 714, y + 40*gn(x, y)) end)
glaze(band, {color="#26331f", coats=0.55, pigment="transparent"})
local lane_near = LANE * rect(280, 395, 80, 70)
glaze(lane_near, {color="#5f6a3c", coats=0.5, pigment="semi"})
-- light back on the stone's sunlit top
local top = STONE * fs:lit{parts={1}, soft=0.1} * mask(function(x, y) return smoothstep(505, 470, y) end)
work(top, {hand="scumble", tool="round 2", length={3, 8}, coverage=2, angle=fs:field("across"), medium=0.35, clip=STONE,
  color=function(x, y) return mix("#a39a86", "#c4b89c", smoothstep(0.3, 0.9, fs:value(x, y) or 0.5)) end})

--@ chunk 21 · clock 64992.4501953125
local cs = (ellipse(760, 462, 230, 42):roughen(22, 90, 5, 18) + ellipse(560, 404, 120, 16):roughen(10, 60, 6, 10)) * below(function(x) return HZ + 40 end) - HILL - WOOD
glaze(cs:blur(18), {color="#55655a", coats=0.16, pigment="transparent"})

--@ chunk 22 · clock 100816.7353515625
-- long, curving blades through the dark foreground band: sunlit tips and dark stalks, uneven lengths
local g = brush("rigger", 0.8)
local gn = noise{seed=81, octaves=3, period=90}
local cols = {"#7d8d44", "#2f3d24", "#9aa25a", "#3c4b2a", "#6a7a3a", "#253220"}
local n = 0
for i = 1, 1400 do
  local x = rand(-10, 1010)
  local y = rand(560, 722)
  if HILL:at(x, math.min(y, 713)) > 0.5 and not (x > 830 and x < 900 and y > 540) then
    local depth = (y - 540) / 180
    local h = (8 + 34 * depth) * rand(0.5, 1.6) * (1 + 0.4*gn(x, y))
    local lean = 0.25*gn(x*0.5, 0) + randn(0, 0.22)
    local curl = randn(0, 0.25)
    if i % 7 == 1 then g:reload(cols[1 + (i // 7) % #cols], 0.75) end
    g:stroke({{x, y}, {x + lean*h*0.4, y - h*0.5}, {x + (lean + curl)*h, y - h}}, {pressure={clamp(0.35 + 0.5*depth, 0.3, 0.9), 0}, ramps={0.05, 0.75}})
    n = n + 1
  end
end
-- plantain rosettes and clover in the grass below the brow
local lb = brush("round", 2)
for _, p in ipairs({{560, 600}, {690, 626}, {420, 650}, {150, 610}, {960, 660}, {330, 688}}) do
  for k = 0, 5 do
    local a = -math.pi + k * math.pi / 5 + randn(0, 0.15)
    local L = rand(10, 18)
    lb:reload(k % 2 == 0 and "#4d6130" or "#6b7f3e", 0.8)
    lb:stroke({{p[1], p[2]}, {p[1] + math.cos(a)*L*0.5, p[2] + math.sin(a)*L*0.25 - 2}, {p[1] + math.cos(a)*L, p[2] + math.sin(a)*L*0.35}}, {pressure={0.4, 1.0, 0.1}, ramps={0.3, 0.5}})
  end
end
local fb = brush("round", 1.3)
for i = 1, 40 do
  local x, y = rand(20, 980), rand(540, 700)
  if HILL:at(x, y) > 0.5 then
    fb:reload(({"#efe9d4", "#e0c24a", "#efe9d4", "#b7a0c8"})[1 + i % 4], 0.8)
    fb:touch(x, y, {pressure=0.5 + 0.3*(y - 540)/160})
  end
end
print(n, "blades")

--@ chunk 23 · clock 100816.7353515625
-- the plain: fuse the dash-rows a little and cut the sugary greens with a thin muted veil
OAKALL = oak:mask():grow(2) + CROWN:grow(3)
local keep = HILL:grow(1) + WOOD:grow(1) + FIELDTREES:grow(1.5) + VILLAGE:grow(1.5) + OAKALL + FIG:grow(3) + STONE:grow(3)
PLAIN = below(function(x) return HZ + 1 end) - keep
local vn = noise{seed=91, octaves=3, period=60, stretch={0, 3}}
work(PLAIN, {hand="glaze", tool="filbert 4", length={6, 16}, coverage=1.6, angle=function(x, y) return 0.02*math.sin(x/120) end, medium=0.75, load=0.35, clip=PLAIN,
  color_over=function(x, y, under)
    local s = 255 * (under.value ^ (1/2.2))
    local muted = mix(under, rgb(s, s, s), 0.4)
    return mix(muted, "#7b7a5a", 0.12 + 0.06*vn(x, y))
  end})
print("plain", PLAIN:area())

--@ chunk 24 · clock 100816.7353515625
-- mute the sugary yellow-green of the near hill toward an olive earth green, heaviest where it is lit
local hn2 = noise{seed=93, octaves=3, period=120}
local hillg = (HILL - FIG:grow(2) - STONE:grow(2) - OAKALL) * mask(function(x, y) return 0.75 + 0.25*hn2:at01(x, y) end)
glaze(hillg, {color="#6d6a4c", coats=0.32, pigment="semi"})
-- quiet the cumulus: a veil of the sky's own blue-gray over the clouds, so they sit back into a calm sky
local cm = (cl:mask{alpha={0.25, 0.9}} * above(function(x) return HZ - 30 end) - OAKALL):blur(8)
glaze(cm, {color="#aeb6bd", coats=0.4, pigment="semi"})
print("ok")

--@ chunk 25 · clock 111723.3642578125
-- the wanderer: a hat brim, a head under it, a lit left flank of the coat, a staff; so he is a man, not a hooded blob
local x, y, k = 348, 488, 1.3
local function F(dx, dy) return {x + k*dx, y + k*dy} end
local sb = brush("rigger", 0.7); sb:load("#3a3128", 0.9)
sb:stroke({F(-5.5, -22), F(-7.5, -10), F(-9.5, 1)}, {pressure={0.7, 0.5}})             -- staff
local brim = brush("round", 1.2); brim:load("#1e1d1c", 0.9)
brim:stroke({F(-3.6, -38.6), F(0, -38.9), F(3.8, -38.4)}, {pressure={0.8, 0.7}})         -- hat brim
local hair = brush("round", 1.2); hair:load("#4a3a2c", 0.8)
hair:touch(F(-0.6, -36.4)[1], F(0, -36.4)[2], {pressure=0.6, drag={0, 1.2}})           -- nape and hair under the hat
local collar = brush("round", 1); collar:load("#6b6454", 0.7)
collar:stroke({F(-3.2, -33.2), F(-0.8, -34.2), F(1.6, -33.6)}, {pressure={0.5, 0.3}})   -- collar catching the light
local lit = fig:band(1.4, 0.5) * FIG * rect(0, 0, x - 1.5, 714)
work(lit, {hand="detail", tool="round 0.8", length={1.5, 4}, coverage=1.8, medium=0.15, angle=1.5, color="#5d6152", clip=FIG})
local fold = brush("rigger", 0.5); fold:load("#454a44", 0.7)
fold:stroke({F(-1.5, -30), F(-2.2, -20), F(-2.6, -13)}, {pressure={0.4, 0.2}})          -- a fold down the coat's back
fold:stroke({F(1.8, -28), F(2.4, -18)}, {pressure={0.35, 0.15}})
-- the boulder: break its flat pale face with a shadowed lower plane, a second fracture and weather streaks
local lower = STONE * mask(function(px, py) return smoothstep(478, 498, py + 6*math.sin(px/9)) end)
glaze(lower, {color="#5d5446", coats=0.35, pigment="transparent"})
local face = STONE * mask(function(px, py) return smoothstep(452, 470, py) * (1 - smoothstep(470, 490, py)) end)
local wn = noise{seed=95, period=5, stretch={1.5, 4}}
work(face * mask(function(px, py) return wn:at01(px, py) > 0.6 and 1 or 0 end), {hand="glaze", tool="round 1.2", length={3, 7}, coverage=1.6, angle=1.45, medium=0.8, clip=STONE,
  color_over={shift={-0.07, 0, 0.004}}})
local cr = brush("rigger", 0.6); cr:load("#34302b", 0.8)
for _, c in ipairs({{{392,466},{405,463},{420,466},{433,463}}, {{440,462},{446,472},{443,482},{448,492}}, {{384,478},{398,480}}}) do
  cr:stroke(c, {pressure={0.65, 0.2}, ramps={0.1, 0.4}, shake=0.8})
end
local ml = brush("round", 1.4)
for i = 1, 26 do
  local px = rand(380, 475); local py = 500 + rand(-4, 3)
  if STONE:at(px, py) > 0.5 then ml:reload(i % 2 == 0 and "#4f5a38" or "#6b6a46", 0.6); ml:touch(px, py, {pressure=0.5, drag={rand(1, 3), -0.5}}) end
end

--@ chunk 26 · clock 118524.91064453125
-- a stilled sky: the puffy cumulus overpainted into long, low banks; slate above, a warm glow low over the village
local function crest(x) local m = HZ for _, l in ipairs(rs) do local c = l:crest(x); if c and c < m then m = c end end return m end
local sn = noise{seed=101, octaves=4, period=140, stretch={0.0, 7}}
local sn2 = noise{seed=102, octaves=3, period=60, stretch={0.02, 5}}
function calmsky(x, y)
  local t = clamp(y / (HZ - 20), 0, 1)
  local c = gradient({{0, "#6f7f90"}, {0.35, "#8e9ba5"}, {0.7, "#bdbdb0"}, {1, "#dccfae"}}, t)
  local glow = math.exp(-((x - 700)/380)^2) * smoothstep(0.45, 1, t)
  c = mix(c, "#e8d6ac", 0.45*glow)
  -- long banks: darker, violet-gray, heaviest in the middle sky
  local b = smoothstep(0.15, 0.55, sn:at01(x, y)) * (1 - smoothstep(0.75, 0.95, t)) * smoothstep(0.05, 0.3, t)
  c = mix(c, shift(c, -0.09, 0.004, -0.012), 0.6*b)
  -- lit undersides of the banks toward the glow
  local e = smoothstep(0.55, 0.75, sn2:at01(x, y)) * smoothstep(0.4, 0.8, t) * (1 - smoothstep(0.9, 1, t))
  return mix(c, "#e9dcbc", 0.35*e)
end
SKYM = mask(function(x, y) return 1 - smoothstep(crest(x) - 8, crest(x) - 1, y) end) - OAKALL:grow(1)
local skyclip = SKYM - OAKALL:grow(1)
work(SKYM, {hand="body", color_over=function(x, y, under) return mix(under, calmsky(x, y), 0.82) end, angle=function(x, y) return 0.02*math.sin(x/200) end,
  length={30, 90}, coverage=3.4, medium=0.3, pal=SKYPAL, clip=skyclip})
blend(SKYM, {angle=0, clip=skyclip})
-- close the old pale sky left in the gap around the crown, right up to the leaves
local ring = (OAKALL:grow(3) - CROWN - oak:mask()) * SKYM:grow(2)
local ringclip = ring - CROWN:shrink(0.5) - oak:mask()
work(ring, {hand="detail", tool="round 2", length={3, 8}, coverage=3.2, medium=0.3, pal=SKYPAL, angle=0, clip=ringclip,
  color_over=function(x, y, under) return mix(under, calmsky(x, y), 0.85) end})
print("ring", ring:area())

--@ chunk 27 · clock 118524.91064453125
-- the oak's crown restated clump by clump: each clump a mass with a shadowed underside and a lit cap
-- of hooked leaf strokes toward the low sun (upper left), so the crown has structure, not one flat blob
local dkb, mdb, ltb = brush("round", 1.8), brush("round", 1.6), brush("round", 1.3)
local n, used = 0, 0
local cs = lv.clumps
for i, c in ipairs(cs) do
  if c.r > 3.5 and CROWN:at(c.x, c.y) > 0.3 then
    used = used + 1
    local r = c.r * 0.9
    -- shadowed underside: a few short strokes along the lower right arc
    if i % 4 == 1 or dkb:fullness() < 0.3 then dkb:reload(mix("#1c2419", "#253020", rand(0, 1)), 0.8) end
    for k = 1, 3 do
      local a = rand(0.2, 2.6)
      local px, py = c.x + math.cos(a)*r*0.7, c.y + math.sin(a)*r*0.55
      dkb:stroke({{px - r*0.25, py}, {px, py + r*0.08}, {px + r*0.25, py - r*0.02}}, {pressure={0.6, 0.1}, ramps={0.1, 0.6}, clip=CROWN})
      n = n + 1
    end
    -- lit cap: hooked leaf strokes on the upper-left of the clump, fewer where the clump is in shade
    local lit = clamp(c.lit or 0.3, 0, 1)
    local cnt = math.floor(2 + 7*lit)
    local b = lit > 0.45 and ltb or mdb
    if i % 3 == 1 or b:fullness() < 0.3 then
      local col = lit > 0.45 and mix("#5d6a37", "#77804a", rand(0, 1)) or mix("#34422a", "#46532f", rand(0, 1))
      b:reload(col, 0.75)
    end
    for k = 1, cnt do
      local a = -math.pi*0.5 - 0.9 + rand(-0.9, 0.9)
      local d = r * rand(0.25, 0.85)
      local px, py = c.x + math.cos(a)*d, c.y + math.sin(a)*d*0.7
      local L = rand(2.5, 4.5) * (0.7 + 0.3*c.r/10)
      local ang = a + math.pi*0.5 + randn(0, 0.35)
      local hook = randn(0, 0.9)
      b:stroke({{px, py}, {px + math.cos(ang)*L*0.5, py + math.sin(ang)*L*0.5}, {px + math.cos(ang + hook*0.6)*L, py + math.sin(ang + hook*0.6)*L}},
        {pressure={0.7, 0.05}, ramps={0.1, 0.7}})
      n = n + 1
    end
  end
end
-- break the silhouette and close the pale rim: hooked strokes outward along the crown's edge
local rim = CROWN:rim(4, 1)
local hb = brush("round", 1.4)
local k = 0
for i = 1, 5000 do
  local x, y = rand(0, 380), rand(90, 360)
  if rim:at(x, y) > 0.5 and k < 900 then
    local up = y < 250
    if k % 8 == 0 then hb:reload(up and (rand(0,1) < 0.5 and "#4d5a31" or "#2a3522") or "#222b1c", 0.7) end
    local dx, dy = x - 215, y - 250
    local dn = math.sqrt(dx*dx + dy*dy) + 1e-6
    local ang = math.atan(dy, dx) + randn(0, 0.6)
    local L = rand(2.5, 5)
    local hook = randn(0, 0.8)
    hb:stroke({{x, y}, {x + math.cos(ang)*L*0.5, y + math.sin(ang)*L*0.5}, {x + math.cos(ang + hook)*L, y + math.sin(ang + hook)*L}},
      {pressure={0.7, 0.05}, ramps={0.1, 0.7}})
    k = k + 1
  end
end
print("clumps used", used, "strokes", n, "rim", k)

--@ chunk 28 · clock 118524.91064453125
-- the crown as one mass in the low light: lit upper left, a transparent shade deepening to the lower right
local cn = noise{seed=111, octaves=3, period=50}
local shade = CROWN * mask(function(x, y) return smoothstep(-0.1, 0.9, ((x - 120)/240 + (y - 140)/200)*0.7 + 0.2*cn(x, y)) end)
glaze(shade:blur(4), {color="#1b2419", coats=0.5, pigment="transparent"})
-- the wanderer stands ON the brow: a dark crease at his feet and a short cast shadow falling right, like the oak's and the stone's
local fx, fy = 348, 488
local cast = (ellipse(fx + 12, fy + 0.8, 13, 1.8):roughen(0.8, 5, 3, 1) + ellipse(fx + 1, fy, 5, 1.4)) - FIG
glaze(cast:blur(1), {color="#26301f", coats=0.6, pigment="transparent"})
-- the stone seated in the turf: a contact crease along its foot, darkest in the middle
local foot = mask(function(x, y) local b = 503 + 3*math.sin(x/11) return smoothstep(6, 0, math.abs(y - b)) * smoothstep(372, 395, x) * (1 - smoothstep(470, 486, x)) end) - FIG:grow(1)
glaze(foot:blur(1.5), {color="#232a1c", coats=0.55, pigment="transparent"})
-- grass blades over his boots and the stone's foot, so neither sits on the ground like a sticker
local g = brush("rigger", 0.5)
for i = 1, 160 do
  local x = (i <= 40) and rand(fx - 7, fx + 8) or rand(374, 486)
  local y = (i <= 40) and (fy + rand(-0.5, 2.5)) or (503 + 3*math.sin(x/11) + rand(-1, 3))
  if i % 8 == 1 then g:reload(({"#5f6a38", "#3b4727", "#747b44", "#2f3a22"})[1 + (i // 8) % 4], 0.6) end
  local h = rand(2.5, 6)
  local lean = randn(0.1, 0.3)
  g:stroke({{x, y}, {x + lean*h*0.5, y - h*0.55}, {x + lean*h, y - h}}, {pressure={0.45, 0}, ramps={0.05, 0.7}})
end
print("cast", cast:area(), "foot", foot:area())

--@ chunk 29 · clock 133667.92138671875
-- the land under the evening sky: a transparent warm umber glaze, light on the far plain, heavier toward us,
-- so the noon greens sink into one stilled light and the glow over the village is the brightest thing
local ln = noise{seed=121, octaves=3, period=160}
local land = mask(function(x, y) return smoothstep(HZ - 2, HZ + 6, y) * (0.3 + 0.7*smoothstep(300, 640, y + 25*ln(x, y))) end)
glaze(land, {color="#4f4a33", coats=0.42, pigment="transparent"})

--@ chunk 30 · clock 134695.46948242188
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
