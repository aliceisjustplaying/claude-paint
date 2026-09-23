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
