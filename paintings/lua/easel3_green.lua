-- easel session "easel3_green": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel3_green.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 392
knollg = function(X, Z)
  return 3.2*math.exp(-(((X + 6)/42)^2 + ((Z - 78)/34)^2)) + 0.6*math.sin(X/23 + Z/31) * math.min(1, Z/40)
end
w = world{horizon=HZ, eye=6, fov=50, sun={azimuth=-118, elevation=40}, ground=knollg}
for _, p in ipairs({{430, 470}, {430, 520}, {200, 600}, {800, 560}, {500, 420}}) do
  print(w:spot(p[1], p[2]))
end
print(w:project(-6, 3.2, 78))

--@ chunk 3 · clock 0
sk = w:sky{haze=2.0, uneven={0.35, 30000, 7}}
cl = w:clouds{sky=sk, cell=3,
  {kind="cumulus", x=4600, z=14000, base=1250, width=3000, height=1400, seed=31},
  {kind="cumulus", x=7400, z=19000, base=1300, width=2600, height=900, seed=8},
  {kind="cumulus", x=-4300, z=16000, base=1350, width=2000, height=700, seed=12},
  {kind="bank", x0=-30000, x1=30000, z=42000, depth=9000, base=800, top=1800, seed=5},
  {kind="stratus", base=4200, thick=300, cover=0.12, seed=3, wind={-0.1, 2.5}, breaks={12000, 0.25}}}
skym = above(function(x) return HZ + 14 end)
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "red earth"}
work(skym, {hand="broad", color=cl, angle=0, coverage=4.5, medium=0.3, pal=skypal, angle_jitter=0.05})
blend(skym, {angle=0})

--@ chunk 4 · clock 0
wait(24*60)
work(above(function(x) return HZ + 10 end), {hand="broad", color=cl, angle=0, coverage=3.5, medium=0.35, pal=skypal, angle_jitter=0.04, aim=1.6})
blend(above(function(x) return HZ + 10 end), {angle=0})

--@ chunk 5 · clock 1440
wait(24*60)
local n1 = noise{seed=41, octaves=5, period=220, persistence=0.45}
local n2 = noise{seed=42, octaves=5, period=160, persistence=0.5}
crest1 = function(x) return HZ - 16 - 10*n1:at01(x, 0) - 44*math.exp(-((x - 770)/105)^2) - 14*math.exp(-((x - 640)/60)^2) end
crest2 = function(x) return HZ - 4 - 7*n2:at01(x, 0) - 16*math.exp(-((x - 170)/130)^2) - 9*math.exp(-((x - 900)/80)^2) end
far1 = below(crest1):roughen(0.8, 9) * above(function(x) return HZ + 6 end)
far2 = below(crest2):roughen(1.0, 7) * above(function(x) return HZ + 8 end)
work(far1, {hand="body", length={25, 70}, coverage=3.5, angle=function(x, y) return 0.03 * n1(x, 3) end, medium=0.3,
  color=function(x, y) return mix("#8795ad", "#a8b0bc", smoothstep(crest1(x), HZ, y)) end})
work(far2, {hand="body", length={20, 60}, coverage=3.5, angle=0.02, medium=0.25,
  color=function(x, y) return mix("#74848f", "#95a0a6", smoothstep(crest2(x), HZ + 6, y)) end})

--@ chunk 6 · clock 2880
wait(24*60)
fieldcells = worley{seed=9, period=70}
fieldkinds = {"#6f8a3c", "#5d7a33", "#a89452", "#7d8a45", "#4f6530", "#8e8a4e", "#667f36"}
fieldwobble = noise{seed=19, octaves=4, period=40}
groundcol = function(x, y)
  local p = w:to_ground(x, y)
  local X, Z = 0, 4000
  if p then X, Z = p[1], p[3] end
  local _, _, edge, r = fieldcells:at(X * 0.55, Z * 1.25)
  local k = 1 + math.floor(r * #fieldkinds) % #fieldkinds
  local c = color(fieldkinds[k])
  c = shift(c, 0.03 * fieldwobble(X, Z), 0, 0)
  -- near ground: warm meadow, foreground darker
  local nearT = smoothstep(90, 25, Z)
  c = mix(c, mix("#5f7430", "#4a5a2a", smoothstep(560, 700, y)), nearT)
  return mix(c, sk:airlight(x), math.min(0.85, w:aerial(Z) * 0.95))
end
land = below(function(x) return HZ + 3 end):roughen(0.8, 10)
work(land, {hand="body", length={18, 60}, coverage=3.5, medium=0.2,
  angle=function(x, y) return 0.03 * fieldwobble(x, y) + 0.1 * smoothstep(480, 700, y) * fieldwobble(y, x) end, color=groundcol})

--@ chunk 7 · clock 4320
wait(24*60)
v = w:view()
local s = v:at(430, 480); print(s.what, s.shade, s.lit)
for k, val in pairs(s.shade or {}) do print(k, val) end

--@ chunk 8 · clock 5760
local mistn = noise{seed=51, octaves=4, period=120, stretch={0, 5}}
hazem = mask(function(x, y)
  if y > HZ + 6 or y < crest1(x) - 3 then return 0 end
  return clamp(0.25 + 0.75 * smoothstep(crest1(x) + 6, HZ + 3, y) + 0.2 * mistn(x, y), 0, 1)
end):blur(1.5)
glaze(hazem, {color="#c6cbd2", coats=0.55, pigment="semi"})
stipple(hazem, {width=1.5, color="#b9c0c9", coverage=function(x, y) return 1.2 + 1.2 * hazem:at(x, y) end,
  pressure={0.35, 0.7}, dips={20, 0.3, 0.7}, aim=false, medium=0.6, fade=1})

--@ chunk 9 · clock 5760
local s = w:spot(900, 410); print(s, w:height(900, 410, 15), w:height(430, 480, 15), w:scale_at(357))

--@ chunk 10 · clock 5760
wait(24*60)
-- groves: {x0, x1, foot y, crown height in units, density}
groves = {
  {30, 200, 402, 9, 1.0}, {292, 340, 400, 11, 1.0}, {530, 690, 403, 8, 0.8},
  {770, 980, 406, 11, 1.0}, {0, 110, 418, 16, 0.9}, {620, 740, 424, 17, 0.8},
  {880, 1000, 440, 26, 1.0}, {95, 150, 424, 20, 0.5}, {170, 260, 409, 7, 0.6}}
local gm, tops = nil, nil
local add = function(a, b) return a and (a + b) or b end
for gi, g in ipairs(groves) do
  local th = g[4]
  local n = math.max(3, math.floor((g[2] - g[1]) / (th * 0.5) * g[5]))
  local xs = uneven(n, g[1], g[2], 0.6, 0.4, gi)
  gm = add(gm, rect(g[1], g[3] - th * 0.45, g[2] - g[1], th * 0.45 + 1))
  for k, x in ipairs(xs) do
    local h = th * rand(0.55, 1.3)
    local r = th * rand(0.3, 0.45)
    gm = add(gm, ellipse(x, g[3] - h + r, r, r * 1.1))
    tops = add(tops, ellipse(x - r * 0.3, g[3] - h + r * 0.75, r * 0.65, r * 0.6))
  end
end
grovem = gm:roughen(0.9, 4) * above(function(x) return HZ + 70 end)
grovelit = tops:roughen(0.8, 3) * grovem
local grovecol = function(x, y, base)
  local p = w:to_ground(x, math.min(H, y + 8)); local Z = p and p[3] or 800
  local far = 1 - smoothstep(HZ + 4, HZ + 48, y)
  return mix(base, mix("#8697a8", sk:airlight(x), 0.3), 0.1 + 0.45 * far)
end
local turn = noise{seed=77, period=6}
work(grovem, {hand="hatch", tool="round 1.2", length={1.5, 3.5}, coverage=2.6, angle=function(x, y) return 1.6 + 1.5 * turn(x, y) end,
  color=function(x, y) return grovecol(x, y, "#28382a") end})
work(grovelit, {hand="hatch", tool="round 1.0", length={1.2, 3}, coverage=1.6, angle=function(x, y) return 1.6 + 1.5 * turn(x, y) end,
  color=function(x, y) return grovecol(x, y, "#5c6f3b") end})

--@ chunk 11 · clock 38749.6328125

for _, hab in ipairs({"oak", "dead_oak"}) do
  local t = tree{habit=hab, x=430, y=484, height=300, seed=17}
  local dead = 0
  for _, l in ipairs(t.limbs) do if l.dead then dead = dead + 1 end end
  print(hab, #t.limbs, "dead", dead, table.concat(t.bounds, ","))
end

--@ chunk 12 · clock 38749.6328125

for _, sd in ipairs({3, 17, 29, 44, 58}) do
  for _, yr in ipairs({80, 160}) do
    local t = tree{habit="oak", x=430, y=484, height=300, seed=sd, years=yr}
    local dead = 0
    for _, l in ipairs(t.limbs) do if l.dead then dead = dead + 1 end end
    local b = t.bounds
    print(sd, yr, #t.limbs, dead, math.floor(b[3]-b[1]), math.floor(b[4]-b[2]))
  end
end

--@ chunk 13 · clock 38749.6328125

for _, sd in ipairs({3, 17, 29, 44}) do
  for _, yr in ipairs({250, 400}) do
    local t = tree{habit="oak", x=430, y=484, height=300, seed=sd, years=yr}
    local dead = 0
    for _, l in ipairs(t.limbs) do if l.dead then dead = dead + 1 end end
    local b = t.bounds
    print(sd, yr, #t.limbs, dead, math.floor(b[3]-b[1]), math.floor(b[4]-b[2]), math.floor(b[1]))
  end
end

--@ chunk 14 · clock 38749.6328125

for _, sd in ipairs({17, 29, 44}) do
  for _, yr in ipairs({24, 34, 44}) do
    local t = tree{habit="oak", x=430, y=484, height=300, seed=sd, years=yr}
    local dead = 0
    for _, l in ipairs(t.limbs) do if l.dead then dead = dead + 1 end end
    local b = t.bounds
    print(sd, yr, #t.limbs, dead, math.floor(b[3]-b[1]), math.floor(b[4]-b[2]), math.floor(b[1]))
  end
end

--@ chunk 15 · clock 38749.6328125
wait(24*60)
OX, OY = 430, 486
w = world{horizon=HZ, eye=6, fov=50, sun={azimuth=-112, elevation=54}, ground=knollg}
oak = tree{habit="oak", x=OX, y=OY, height=300, seed=17, years=34}
oakleaves = oak:foliage{sun={-0.75, -0.6, 0.3}, seed=17, clump=0.04, spray=2}
local s = w:spot(OX, OY)
w = w:proxy(s, body.ellipsoid(s:p(-1.3, 10.5, 0), s:size(5, 4, 4.6)))
w = w:proxy(s, body.block(s:p(0, 3, 0), s:size(0.5, 3, 0.5), s:m(0.2)))
v = w:view()
local sa = w:shadow_angle(OX, OY + 2) or 0
local sh = (v:shadows() * v:land()):roughen(7, 26, 3, 5):soften(5)
work(sh, {hand="body", tool="filbert 3", length={5, 14}, coverage=3, angle=sa, angle_jitter=0.3,
  color_over={shift={-0.055, -0.004, -0.016}}})

--@ chunk 16 · clock 40189.6328125
wait(3*60)
-- limbs, then the trunk and main limbs as ribbons at their widths, lit flank on the sun side
local trunk, limb, twig = brush("round", 4.5), brush("round", 1.8), brush("rigger", 0.7)
for i, l in ipairs(oak.limbs) do
  if #l.pts >= 2 then
    local b = (l.order == 0) and trunk or ((l.w[1] > 1.0) and limb or twig)
    local c = l.dead and "#6e675c" or "#3a332b"
    if i % 5 == 1 or b:fullness() < 0.3 then b:reload(c, 0.9) end
    b:stroke(l.pts, {pressure=(l.order == 0) and {1.0, 0.55} or {0.85, 0.2}, ramps={0.03, 0.5}, shake=0.6})
  end
end
local wood, woodlit = nil, nil
for _, l in ipairs(oak.limbs) do
  if #l.pts >= 2 and l.w[1] > 1.6 then
    local ws, lp, lw = {}, {}, {}
    for k = 1, #l.pts do
      ws[k] = math.max(0.6, l.w[k] * 1.05)
      lp[k] = {l.pts[k][1] - ws[k] * 0.22, l.pts[k][2]}
      lw[k] = ws[k] * 0.42
    end
    wood = wood and (wood + ribbon(l.pts, ws)) or ribbon(l.pts, ws)
    woodlit = woodlit and (woodlit + ribbon(lp, lw)) or ribbon(lp, lw)
  end
end
oakwood = wood:roughen(0.7, 4)
local bn = noise{seed=5, period=6, stretch={1.5, 4}}
work(oakwood, {hand="body", tool="round 1.6", length={3, 9}, coverage=3, angle=function(x, y) return 1.5 + 0.3 * bn(x, y) end,
  color=function(x, y) return mix("#2a2520", "#3b342b", bn:at01(x, y)) end})
work(woodlit * oakwood, {hand="detail", tool="round 1.2", length={2, 6}, coverage=2.2, angle=function(x, y) return 1.5 + 0.4 * bn(x, y) end,
  color=function(x, y) return mix("#686255", "#8b8371", bn:at01(x * 1.3, y)) end})

--@ chunk 17 · clock 40369.6328125
local l = oak.limbs[1]; print(#l.pts, l.w[1], l.w[#l.w], l.pts[1][1], l.pts[1][2], l.pts[#l.pts][1], l.pts[#l.pts][2]); local c=0; for _, m in ipairs(oak.limbs) do if m.order==1 then c=c+1; if c<6 then print("o1", m.w[1], #m.pts) end end end

--@ chunk 18 · clock 40369.6328125
OSKY = cl; OB = oak.bounds; OTOP = oak.bounds[2]
DEAD1 = {{OX - 32, OY - 240}, {OX - 30, OY - 270}, {OX - 36, OY - 296}, {OX - 31, OY - 322}, {OX - 35, OY - 342}, {OX - 32, OY - 356}}
DEADB = {
  {{OX - 35, OY - 300}, {OX - 48, OY - 312}, {OX - 55, OY - 328}, {OX - 66, OY - 334}},
  {{OX - 33, OY - 336}, {OX - 22, OY - 346}, {OX - 17, OY - 360}},
  {{OX - 50, OY - 315}, {OX - 48, OY - 327}},
  {{OX + 48, OY - 232}, {OX + 62, OY - 252}, {OX + 67, OY - 270}, {OX + 78, OY - 280}},
  {{OX + 64, OY - 256}, {OX + 76, OY - 257}}}

--@ chunk 19 · clock 40369.6328125
wait(2*60)
-- leaves: a warm mid layer, the shadow masses, the lit masses; soft, uneven edges
local turn = noise{seed=71, period=9}
oakway = function(x, y) return 2.6 * turn(x, y) end
local crown = oakleaves:mask()
local lit = oakleaves:lit() * crown
local yy = function(y) return clamp((y - OTOP) / 300, 0, 1) end
local cn = noise{seed=72, octaves=3, period=40}
work(crown, {hand="hatch", tool="round 1.8", length={3, 7}, coverage=2.4, angle=oakway, angle_jitter=0.8, hug=false,
  color=function(x, y) return mix("#3e5230", "#34462c", yy(y) + 0.3 * cn(x, y)) end})
work(crown - lit, {hand="hatch", tool="round 1.6", length={2.5, 6}, coverage=2.2, angle=oakway, angle_jitter=0.8, hug=false,
  color=function(x, y) return mix(mix("#26331f", "#1e2a1d", yy(y)), "#2a3a33", 0.35 * cn:at01(y, x)) end})
work(lit, {hand="hatch", tool="round 1.4", length={2, 5}, coverage=2.0, angle=oakway, angle_jitter=0.8, hug=false,
  color=function(x, y) return mix(mix("#6a833a", "#587131", yy(y)), "#7d8a3e", 0.4 * cn:at01(x, y)) end})
-- leaf marks round the silhouette: small hooked strokes that break the edge
local rim = crown:rim(5, 1)
local lf = brush("round", 1.4)
local n = 0
for i = 1, 9000 do
  local x, y = rand(OB[1] - 6, OB[3] + 6), rand(OB[2] - 6, OB[4])
  if rim:at(x, y) > 0.5 then
    n = n + 1
    if n % 12 == 1 then
      local litv = lit:at(x, y)
      lf:reload(litv > 0.5 and mix("#6a833a", "#88964a", rand()) or mix("#243020", "#3a4c2c", rand()), 0.8)
    end
    local a = oakway(x, y) + randn(0, 0.6)
    local len = rand(2.5, 5)
    local hook = randn(0, 0.8)
    lf:stroke({{x, y}, {x + len * 0.6 * math.cos(a), y + len * 0.6 * math.sin(a)},
      {x + len * math.cos(a + hook), y + len * math.sin(a + hook)}}, {pressure={0.7, 0.05}, ramps={0.1, 0.6}})
  end
end
-- sky back into the holes of the crown
work(oakleaves:gaps(6) * crown:grow(2) - oakwood:grow(2), {hand="detail", tool="round 1.4", length={2, 4}, coverage=2, color=OSKY, angle=0.3, pal=skypal})
-- the sun's touches on the lit tops
stipple((lit * mask(function(x, y) return 1 - yy(y) * 0.8 end)):shrink(1.5), {width=1.5, color="#a0b060", coverage=function(x, y) return 0.8 end,
  pressure={0.35, 0.8}, dips={14, 0.4, 0.7}, aim=false, medium=0.3, fade=1, drag={1, -0.4}, twist=0.6})
print(n, "leaf marks")
-- the old oak's dead crown: a broken limb and snags rising above the leaves, silver-gray in the sun
local taper = function(pts, w0, w1) local ws = {}; for k = 1, #pts do ws[k] = lerp(w0, w1, (k - 1) / math.max(1, #pts - 1)) end; return ws end
local dead = ribbon(DEAD1, taper(DEAD1, 4.2, 1.4))
local deadlit = ribbon((function() local o = {}; for k, p in ipairs(DEAD1) do o[k] = {p[1] - 1.0, p[2]} end; return o end)(), taper(DEAD1, 1.8, 0.6))
for bi, br in ipairs(DEADB) do
  local w0 = (bi == 1 or bi == 4) and 2.2 or 1.3
  dead = dead + ribbon(br, taper(br, w0, 0.5))
  local o = {}; for k, p in ipairs(br) do o[k] = {p[1] - 0.5, p[2] - 0.4} end
  deadlit = deadlit + ribbon(o, taper(br, w0 * 0.4, 0.25))
end
oakdead = dead:roughen(0.3, 3)
work(oakdead, {hand="detail", tool="round 1", length={2, 6}, coverage=3.2, angle=1.5, color="#4f4a42", medium=0.1})
work(deadlit * oakdead, {hand="detail", tool="round 0.8", length={2, 5}, coverage=2.6, angle=1.5, color="#a29b8d", medium=0.1})

--@ chunk 20 · clock 40489.6328125
wait(24*60)
-- a sandy track from the lower right, rising to pass left of the oak's foot
pathpts = {{800, 720}, {720, 660}, {640, 612}, {575, 570}, {520, 535}, {478, 508}, {440, 492}, {395, 480}, {340, 470}, {290, 462}}
pathw = {48, 40, 32, 25, 19, 14, 11, 8, 6, 5}
pathm = ribbon(pathpts, pathw):roughen(2.5, 10, 5, 1):soften(1.5)
local pn = noise{seed=61, octaves=4, period=20}
work(pathm, {hand="body", tool="filbert 4", length={6, 18}, coverage=3, medium=0.15,
  angle=function(x, y) return -0.65 + 0.2 * pn(x, y) end,
  color=function(x, y) local c = mix(mix("#7f7650", "#968a60", pn:at01(x, y)), "#5f6e33", 0.3 + 0.3 * pn:at01(y, x)); return mix(c, "#9c9f7c", smoothstep(560, 470, y) * 0.4) end})
-- ruts: two darker lines along it
local rut = brush("round", 2.2)
for side = -1, 1, 2 do
  local pts = {}
  for k, p in ipairs(pathpts) do pts[k] = {p[1] + side * pathw[k] * 0.22, p[2] + side * pathw[k] * 0.12} end
  rut:reload("#7d6c4d", 0.7)
  rut:stroke(pts, {pressure={0.7, 0.15}, ramps={0.02, 0.5}, shake=1.5})
end
-- the foreground bank under a passing cloud's shade: deeper, warmer greens laid in body
local fgt = function(x, y) return smoothstep(585, 700, y + 30 * pn(x * 0.25, 7)) end
fgm = mask(function(x, y) return fgt(x, y) > 0.02 and 1 or 0 end)
local gn = noise{seed=63, octaves=4, period=35}
work(fgm - pathm:shrink(2), {hand="body", tool="filbert 5", length={10, 30}, coverage=3, medium=0.18,
  angle=function(x, y) return 0.06 * gn(x, y) end,
  color=function(x, y, u) local t = fgt(x, y); return mix(mix("#5d7230", "#6a7a36", gn:at01(x, y)), mix("#3c4a24", "#4a4a28", gn:at01(y, x)), t) end})
work(fgm * pathm, {hand="body", tool="filbert 4", length={6, 16}, coverage=2.5, medium=0.18, angle=-0.6,
  color=function(x, y) return mix("#7f7650", "#5e5638", fgt(x, y)) end})

--@ chunk 21 · clock 41929.6328125
wait(4*60)
local en = noise{seed=88, octaves=4, period=90}
local tufts = sward{region=below(function(x) return 528 + 30 * en(x, 0) end):roughen(14, 40, 3, 10) - pathm:shrink(3), horizon=HZ, near=H, height=34, flowers=0.04, seed=21,
  wind={lean=0.12, gust=0.25, period=180, seed=4}}
local g = brush("rigger", 0.7)
local greens = {"#56702c", "#7a8e3a", "#3e5226", "#8e9a4c", "#6a7a34", "#a39a5a"}
local n = 0
for i, t in ipairs(tufts) do
  if i % 4 == 1 then
    local c = greens[1 + (i // 4) % #greens]
    c = shift(color(c), -0.13 * smoothstep(570, 690, t.y), 0.002, 0.008 * smoothstep(570, 690, t.y))
    g:reload(c, 0.7)
  end
  local p = clamp(0.25 + 0.5 * t.scale, 0.2, 0.9)
  for _, bl in ipairs(t.blades) do
    g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}})
    n = n + 1
  end
end
local f = brush("round", 1.2)
local k = 0
for _, t in ipairs(tufts) do
  if t.flower then
    if k % 6 == 0 then f:reload(({"#e6e0c8", "#d6b43a", "#b9c2dc", "#c9d27a"})[1 + t.flower.kind % 4], 0.8) end
    f:touch(t.flower.x, t.flower.y, {pressure=clamp(t.flower.r / 2, 0.2, 0.8)})
    k = k + 1
  end
end
print(#tufts, "tufts", n, "blades", k, "flowers")

--@ chunk 22 · clock 42169.6328125
local cn = noise{seed=93, octaves=3, period=26}
crest1b = function(x) return crest1(x) + 1.4 * cn(x, 0) end
local cut = mask(function(x, y) local c = crest1b(x); return (y < c + 0.6 and y > c - 22) and smoothstep(540, 610, x) or 0 end):soften(0.7)
work(cut, {hand="detail", tool="round 2", length={10, 26}, coverage=2.2, angle=0, medium=0.35, pal=skypal,
  color=function(x, y) return sample(x, crest1(x) - 26, 4) end})
blend(cut:grow(2) * above(function(x) return crest1b(x) - 1 end), {angle=0})

--@ chunk 23 · clock 42169.6328125
wait(3*60)
-- a shepherd leaning on his staff in the oak's shade, back to us, looking out
local fx, fy = 462, 492
local coat = poly({{fx - 4.5, fy - 2}, {fx - 5.5, fy - 16}, {fx - 4, fy - 25}, {fx - 1.5, fy - 28}, {fx + 2.5, fy - 28},
  {fx + 4.5, fy - 24}, {fx + 5.2, fy - 15}, {fx + 4.2, fy - 2}}, true)
local legs = rect(fx - 3, fy - 3, 2, 3.5) + rect(fx + 0.8, fy - 3, 2, 3.5)
local head = ellipse(fx + 0.4, fy - 31, 2.6, 3.0)
local hat = ellipse(fx + 0.4, fy - 33.2, 4.4, 1.3) + ellipse(fx + 0.4, fy - 35, 2.4, 2.0)
work(coat + legs, {hand="detail", tool="round 1.2", length={2, 6}, coverage=3, angle=1.57, color="#2d2a2c"})
work(head, {hand="detail", tool="round 1", length={1, 3}, coverage=3, angle=0.5, color="#4a3a2e"})
work(hat, {hand="detail", tool="round 1", length={2, 5}, coverage=3, angle=0, color="#1f1c1c"})
-- the sun catches his left shoulder and sleeve
work(coat * rect(fx - 6, fy - 28, 3.5, 20), {hand="detail", tool="round 0.9", length={2, 5}, coverage=1.5, angle=1.5, color="#55505a"})
local staff = brush("round", 0.9)
staff:load("#3a3026", 0.9)
staff:stroke({{fx + 6, fy + 1}, {fx + 7.5, fy - 18}, {fx + 8.5, fy - 38}}, {pressure={0.8, 0.7}, ramps={0.02, 0.1}})
-- sheep grazing on the lit common, off to the left of the track
local sheep = {{352, 505, 1.0, 1}, {378, 511, 1.05, -1}, {331, 516, 1.1, 1}, {300, 509, 0.95, 1}, {395, 500, 0.9, -1}}
for _, sp in ipairs(sheep) do
  local x, y, s, d = sp[1], sp[2], sp[3] * 1.15, sp[4]
  local body = ellipse(x, y - 6 * s, 7 * s, 3.8 * s):roughen(0.5, 2)
  local headm = ellipse(x + d * 8.2 * s, y - 3.6 * s, 2.3 * s, 1.7 * s)
  local shade = body * mask(function(px, py) return (py > y - 5.8 * s or px * d > (x + 3 * s) * d) and 1 or 0 end)
  work(body, {hand="detail", tool="round 1.1", length={1.5, 4}, coverage=3, angle=0, color="#c2bca6"})
  work(shade, {hand="detail", tool="round 1", length={1.5, 3.5}, coverage=2.2, angle=0, color="#8a8674"})
  work(headm, {hand="detail", tool="round 0.9", length={1, 3}, coverage=3, angle=0.4 * d, color="#4e463a"})
  local lg = brush("round", 0.7)
  lg:load("#3b362e", 0.9)
  for _, lx in ipairs({-5, -2, 3, 6}) do lg:stroke({{x + lx * s, y - 3 * s}, {x + lx * s, y + 1.5 * s}}, {pressure={0.55, 0.45}}) end
end

--@ chunk 24 · clock 42349.6328125
wait(24*60)
local r1 = body.ellipsoid({150, 684, 40}, {58, 30, 40}):turn({150, 684, 40}, 0.4, 0.15, -0.08):rough(5, 45, 3):rough(0.8, 9, 4, true)
local r2 = body.ellipsoid({214, 694, 60}, {26, 16, 20}):turn({214, 694, 60}, -0.3, 0.1, 0.1):rough(3, 30, 5):rough(0.6, 7, 6, true)
local r3 = body.ellipsoid({92, 700, 30}, {22, 13, 16}):rough(2.5, 25, 7):rough(0.5, 6, 8, true)
stones = form{ {r1, dist=0.05}, {r2, dist=0.05}, {r3, dist=0.05},
  light={from={-1, -0.75}, front=0.45, ambient=0.25, penumbra=0.08} }
local sn = noise{seed=101, octaves=4, period=8}
local stonecol = function(x, y)
  local v = stones:value(x, y)
  local c = mix("#45423c", "#a69d88", smoothstep(0.08, 0.85, v))
  return shift(c, 0.035 * sn(x, y), 0.004 * sn(y, x), 0.01 * sn(x * 1.3, y))
end
local all = stones:silhouette{parts={1, 2, 3}, soft=0.5} * above(function(x) return 704 end)
work(all * stones:shadow{parts={1, 2, 3}}, {hand="body", tool="round 1.6", length={3, 9}, coverage=3.2, color=stonecol, angle=stones:field("fall")})
work(all * stones:lit{parts={1, 2, 3}, soft=0.1}, {hand="body", tool="round 1.4", length={3, 8}, coverage=3.2, color=stonecol, angle=stones:field("across")})
work(stones:edges{concave=true} * all, {hand="detail", tool="round 0.8", color="#2e2b27", angle=stones:field("edge"), coverage=1.2})
-- lichen specks on the lit tops
stipple(all * stones:lit{parts={1, 2, 3}, soft=0.1}:shrink(3), {width=1.1, color="#c8c49a", coverage=function(x, y) return 0.35 end,
  pressure={0.3, 0.6}, aim=false, fade=0, medium=0.2})
-- pebbles on the track
local pb = brush("round", 1.4)
for i = 1, 26 do
  local t = rand(0.05, 0.55)
  local k = 1 + math.floor(t * (#pathpts - 1))
  local a, b = pathpts[k], pathpts[k + 1]
  local f = t * (#pathpts - 1) - (k - 1)
  local x = lerp(a[1], b[1], f) + randn(0, pathw[k] * 0.25)
  local y = lerp(a[2], b[2], f) + randn(0, 3)
  local sz = clamp((y - 480) / 180, 0.25, 1.4)
  pb:reload(mix("#6d665a", "#bdb49c", rand()), 0.7)
  pb:touch(x, y, {pressure=0.3 + 0.4 * sz, drag={1, 0}})
end

--@ chunk 25 · clock 43789.6328125
wait(3*60)
local g = brush("rigger", 0.8)
-- blades growing up over the boulder's foot
for i = 1, 120 do
  local x = rand(70, 250)
  local base = 700 + rand(-2, 12)
  if i % 6 == 1 then g:reload(({"#4a5e2a", "#6e7e36", "#3a4a22", "#8c9048"})[1 + (i // 6) % 4], 0.7) end
  local h = rand(8, 22)
  local lean = randn(0.05, 0.25)
  g:stroke({{x, base}, {x + lean * h * 0.4, base - h * 0.55}, {x + lean * h, base - h}}, {pressure={0.75, 0}, ramps={0.05, 0.7}})
end
-- a thistle in the right foreground, big enough to count, with a dock at its foot
local S = 2.1
local tx, ty = 900, 716
local st = brush("round", 2.4)
st:load("#2c3520", 0.95)
st:stroke({{tx, ty}, {tx - 2*S, ty - 30*S}, {tx + 1*S, ty - 58*S}}, {pressure={0.9, 0.55}, shake=0.8})
st:reload("#2c3520", 0.9)
st:stroke({{tx - 1*S, ty - 34*S}, {tx + 9*S, ty - 44*S}, {tx + 12*S, ty - 52*S}}, {pressure={0.7, 0.35}})
st:stroke({{tx - 1*S, ty - 22*S}, {tx - 10*S, ty - 32*S}, {tx - 13*S, ty - 41*S}}, {pressure={0.7, 0.35}})
local lf = brush("round", 2.6)
for _, L in ipairs({{-1, -12, -17, -9, 1}, {1, -18, 16, -12, -1}, {0, -40, -10, -37, 1}, {1, -8, 15, 1, -1}, {0, -28, 12, -30, 1}}) do
  lf:reload("#62705a", 0.9)
  local x0, y0 = tx + L[1]*S, ty + L[2]*S
  local pts = {{x0, y0}}
  for k = 1, 6 do
    local t = k / 6
    pts[#pts + 1] = {x0 + L[3]*S*t, y0 + L[4]*S*t + L[5] * 2.2 * ((k % 2 == 0) and 1 or -1)}
  end
  lf:stroke(pts, {pressure={0.95, 0.05}, ramps={0.1, 0.6}, shake=0.5})
  local hi = brush("rigger", 0.6); hi:load("#a9b39a", 0.7)
  hi:stroke({pts[1], pts[3], pts[5]}, {pressure={0.5, 0.1}})
end
local fl = brush("round", 2.2)
for _, h in ipairs({{tx + 1*S, ty - 60*S}, {tx + 12*S, ty - 54*S}, {tx - 13*S, ty - 43*S}}) do
  fl:reload("#56643a", 0.95)
  fl:touch(h[1], h[2] + 3, {pressure=0.95}); fl:touch(h[1] + 1, h[2] + 4.5, {pressure=0.8})
  local tuft = brush("rigger", 0.8)
  tuft:load("#8e4f8a", 0.95)
  for k = -3, 3 do
    tuft:stroke({{h[1] + k * 0.7, h[2] + 1}, {h[1] + k * 1.6, h[2] - 5}}, {pressure={0.8, 0.2}})
  end
end
local dk = brush("filbert", 5)
for _, D in ipairs({{858, 712, -0.8}, {872, 708, -1.5}, {846, 714, -0.25}, {884, 713, -2.3}}) do
  dk:reload("#3d5028", 0.95)
  local a = D[3]
  dk:stroke({{D[1], D[2]}, {D[1] + 20 * math.cos(a), D[2] + 20 * math.sin(a)}, {D[1] + 40 * math.cos(a + 0.15), D[2] + 40 * math.sin(a + 0.15)}},
    {pressure={0.95, 0.05}, ramps={0.15, 0.6}, swell={0.7, 1.3, 0.4}})
  local rib = brush("rigger", 0.6); rib:load("#7f8a58", 0.7)
  rib:stroke({{D[1], D[2]}, {D[1] + 34 * math.cos(a + 0.1), D[2] + 34 * math.sin(a + 0.1)}}, {pressure={0.5, 0.05}})
end
-- yarrow umbels scattered through the near grass
local ya = brush("round", 1.6)
for i = 1, 18 do
  local x, y = rand(20, 980), rand(600, 700)
  if pathm:at(x, y) < 0.2 then
    local stem = brush("rigger", 0.9)
    stem:load("#556438", 0.8)
    local h = rand(22, 38) * (y - 480) / 220
    stem:stroke({{x, y}, {x + randn(0, 1), y - h}}, {pressure={0.6, 0.3}})
    ya:reload("#e3dfcf", 0.8)
    for k = 1, 6 do ya:touch(x + randn(0, 2.2), y - h + randn(0, 0.9), {pressure=0.5}) end
  end
end

