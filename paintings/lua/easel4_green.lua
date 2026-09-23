-- easel session "easel4_green": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel4_green.lua [--width 3200]
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
