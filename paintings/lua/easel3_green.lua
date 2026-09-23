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
oakleaves = oak:foliage{sun={-0.75, -0.6, 0.3}, seed=17}
local s = w:spot(OX, OY)
w = w:proxy(s, body.ellipsoid(s:p(-1.3, 10.5, 0), s:size(5, 4, 4.6)))
w = w:proxy(s, body.block(s:p(0, 3, 0), s:size(0.5, 3, 0.5), s:m(0.2)))
v = w:view()
local sa = w:shadow_angle(OX, OY + 2) or 0
local sh = (v:shadows() * v:land()):roughen(5, 18, 3, 3):soften(3)
work(sh, {hand="body", tool="filbert 3", length={5, 14}, coverage=3, angle=sa, angle_jitter=0.3,
  color_over={shift={-0.065, -0.004, -0.018}}})

--@ chunk 16 · clock 40189.6328125
wait(3*60)
local trunk, limb, twig = brush("round", 4.5), brush("round", 1.8), brush("rigger", 0.7)
for i, l in ipairs(oak.limbs) do
  if #l.pts >= 2 then
    local b = (l.order == 0) and trunk or ((l.w[1] > 1.0) and limb or twig)
    local c = l.dead and "#6e675c" or "#3a332b"
    if i % 5 == 1 or b:fullness() < 0.3 then b:reload(c, 0.9) end
    local p = (l.order == 0) and {1.0, 0.55} or {0.85, 0.2}
    b:stroke(l.pts, {pressure=p, ramps={0.03, 0.5}, shake=0.6})
  end
end
local turn = noise{seed=71, period=10}
oakway = function(x, y) return 2.4 * turn(x, y) end
local crown = oakleaves:mask()
local lit = oakleaves:lit()
local yy = function(y) return clamp((y - 180) / 300, 0, 1) end
work(crown, {hand="hatch", tool="round 1.8", length={3, 7}, coverage=2.4, angle=oakway, angle_jitter=0.7,
  color=function(x, y) return mix("#3c4e2c", "#32422a", yy(y)) end})
work(crown - lit:grow(1), {hand="hatch", tool="round 1.5", length={2.5, 6}, coverage=2.0, angle=oakway, angle_jitter=0.7,
  color=function(x, y) return mix("#27331f", "#1f2a1c", yy(y)) end})
work(lit * crown, {hand="hatch", tool="round 1.3", length={2, 5}, coverage=2.0, angle=oakway, angle_jitter=0.7,
  color=function(x, y) return mix("#667f38", "#556d31", yy(y)) end})

--@ chunk 17 · clock 40369.6328125
local l = oak.limbs[1]; print(#l.pts, l.w[1], l.w[#l.w], l.pts[1][1], l.pts[1][2], l.pts[#l.pts][1], l.pts[#l.pts][2]); local c=0; for _, m in ipairs(oak.limbs) do if m.order==1 then c=c+1; if c<6 then print("o1", m.w[1], #m.pts) end end end

--@ chunk 18 · clock 40369.6328125
local wood, woodlit = nil, nil
for _, l in ipairs(oak.limbs) do
  if #l.pts >= 2 and l.w[1] > 1.6 then
    local ws, lp, lw = {}, {}, {}
    for k = 1, #l.pts do
      ws[k] = math.max(0.6, l.w[k] * 0.95)
      lp[k] = {l.pts[k][1] - ws[k] * 0.22, l.pts[k][2]}
      lw[k] = ws[k] * 0.45
    end
    local r = ribbon(l.pts, ws)
    wood = wood and (wood + r) or r
    local rl = ribbon(lp, lw)
    woodlit = woodlit and (woodlit + rl) or rl
  end
end
oakwood = wood:roughen(0.6, 4)
oakwoodlit = woodlit * oakwood
local bn = noise{seed=5, period=6, stretch={1.5, 4}}
work(oakwood, {hand="body", tool="round 1.6", length={3, 9}, coverage=3, angle=function(x, y) return 1.5 + 0.3 * bn(x, y) end,
  color=function(x, y) return mix("#2c2721", "#3d352c", bn:at01(x, y)) end})
work(oakwoodlit, {hand="detail", tool="round 1.2", length={2, 6}, coverage=2.2, angle=function(x, y) return 1.5 + 0.4 * bn(x, y) end,
  color=function(x, y) return mix("#6b6558", "#8c8472", bn:at01(x * 1.3, y)) end})

--@ chunk 19 · clock 40369.6328125
wait(2*60)
local crown = oakleaves:mask()
local lit = oakleaves:lit()
local yy = function(y) return clamp((y - 180) / 300, 0, 1) end
local keep = crown * (oakwood:shrink(0.5) * mask(function(x, y) return y > 420 and 1 or 0 end)):map(function(v) return 1 - v end)
work(keep - lit:grow(1), {hand="hatch", tool="round 1.4", length={2.5, 5}, coverage=1.3, angle=oakway, angle_jitter=0.8,
  color=function(x, y) return mix("#2a3621", "#212b1c", yy(y)) end})
work(lit * keep, {hand="hatch", tool="round 1.2", length={2, 4.5}, coverage=1.4, angle=oakway, angle_jitter=0.8,
  color=function(x, y) return mix("#6e883c", "#5b7433", yy(y)) end})
-- the sun's touches: small, uneven, on the lit tops
local hl = lit * crown * mask(function(x, y) return 1 - yy(y) * 0.7 end)
stipple(hl:shrink(1.5), {width=1.6, color="#9aae5c", coverage=function(x, y) return 0.9 end, pressure={0.35, 0.8},
  dips={14, 0.4, 0.7}, aim=false, medium=0.3, fade=1, drag={1, -0.4}, twist=0.6})
-- dead snags through the top of the crown
local snag = brush("round", 1.6)
snag:load("#5f594f", 0.9)
snag:stroke({{392, 214}, {388, 196}, {394, 176}, {389, 160}}, {pressure={0.9, 0.1}, ramps={0.05, 0.7}, shake=0.8})
snag:stroke({{391, 190}, {377, 178}, {371, 167}}, {pressure={0.6, 0.05}, ramps={0.05, 0.7}})
snag:stroke({{394, 176}, {404, 168}, {406, 158}}, {pressure={0.5, 0.0}, ramps={0.05, 0.7}})
snag:reload("#5f594f", 0.8)
snag:stroke({{482, 262}, {498, 246}, {503, 230}, {514, 221}}, {pressure={0.75, 0.05}, ramps={0.05, 0.7}, shake=0.8})
snag:stroke({{500, 240}, {510, 243}}, {pressure={0.4, 0.0}})
local tip = brush("rigger", 0.6)
tip:load("#8c8578", 0.7)
tip:stroke({{390, 212}, {387, 195}, {392, 177}}, {pressure={0.5, 0.2}, ramps={0.05, 0.6}})

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
