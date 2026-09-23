-- easel session "easel3_near": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel3_near.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.3, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0

HZ = 470
skyn = noise{seed=5, octaves=4, period=300, warp={120, 40}}
sky = function(x, y)
  local t = clamp(y / 520 + 0.06 * skyn(x, y), 0, 1)
  return gradient({{0, "#7f8d9c"}, {0.35, "#a9b1b5"}, {0.72, "#d9d3bf"}, {1, "#e8dcbc"}}, t)
end
skym = above(function(x) return 600 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

wait(24*60)
stipple(skym, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55})

--@ chunk 4 · clock 1440

wait(24*60)
farn = noise{seed=8, octaves=6, period=70}
spires = {}
for i, px in ipairs(uneven(70, 0, 1000, 0.7, 0.5, 9)) do
  spires[i] = {px, rand(8, 42) * (0.4 + 0.6*math.abs(math.sin(px/170 + 1))), rand(5, 10)}
end
farcrest = function(x)
  local top = 478 - 14*farn:at01(x, 0) - 10*math.exp(-((x-820)/120)^2)
  local best = top
  for _, s in ipairs(spires) do
    local d = math.abs(x - s[1]) / s[3]
    if d < 1 then best = math.min(best, top - 6 - s[2]*(1 - d)) end
  end
  return best
end
farwood = below(farcrest):roughen(1.5, 4, 3, 0.8)
work(farwood, {hand="hatch", tool="round 2.2", length={4, 10}, coverage=3.4, angle=function(x,y) return (math.floor(x/7)%2==0) and 2.5 or 0.64 end, angle_jitter=0.5, medium=0.3,
  color=function(x, y) return mix(mix("#858c93", "#9fa39f", smoothstep(430, 520, y)), "#7c8588", 0.4*farn:at01(x*2, y)) end})
floor = function(x) return 512 + 6*math.sin(x/140) end
floorm = below(floor):roughen(1.5, 30, 4, 1)
fn = noise{seed=12, octaves=5, period=110}
work(floorm, {hand="body", length={10, 30}, coverage=4.5, angle=function(x, y) return 0.08*fn(x, y) end, medium=0.15, aim="masstone",
  color=function(x, y)
    local t = smoothstep(505, 769, y)
    local c = mix("#8e8a76", "#5d4c34", smoothstep(0, 0.4, t))
    c = mix(c, "#3a2f23", smoothstep(0.4, 1, t))
    return mix(c, "#6e5d3c", 0.4*fn:at01(x, y))
  end})

--@ chunk 5 · clock 2880
print(dry(), drying(500,480), drying(500,600))

--@ chunk 6 · clock 38574.35546875

local A = body.block({620, 590, 0}, {530, 140, 220}, 14):turn({620, 590, 0}, 0, 0, -0.03)
local B = body.block({598, 458, 25}, {600, 125, 270}, 18):turn({598, 458, 25}, 0.03, 0, 0.02)
local C1 = body.block({500, 335, 10}, {330, 135, 230}, 24):turn({500, 335, 10}, 0, 0.05, 0.05)
local C2 = body.block({760, 352, 15}, {150, 105, 200}, 28):turn({760, 352, 15}, 0, 0, -0.08)
rock = (A + B + C1 + C2)
  :turn({600, 460, 0}, 0.32, 0.08, 0.02)
  :rough(14, 130, 3)
rock = rock:cut({330, 300, 40}, {-0.7, -0.7, 0.3}, 11, 3)
  :cut({845, 300, 40}, {0.6, -0.7, 0.3}, 12, 3)
  :cut({300, 610, 60}, {-0.9, 0.3, 0.4}, 14, 3)
  :rough(4, 36, 8, true)
  :rough(1.3, 13, 4, true)
boulder = body.ellipsoid({215, 672, 90}, {105, 52, 70}):turn({215, 672, 90}, -0.3, 0.2, 0.05):rough(9, 70, 6)
  :cut({215, 635, 90}, {-0.2, -1, 0.3}, 13, 4):cut({300, 672, 90}, {1, -0.2, 0.3}, 15, 4):rough(0.8, 16, 7, true)
f = form{ {rock, dist=0.25}, {boulder, dist=0.1}, light={from={-1, -0.55}, front=0.35, ambient=0.22, penumbra=0.06, bounce=0.15} }
rockm = f:silhouette{parts={1}, soft=0.5}
boulm = f:silhouette{parts={2}, soft=0.5}
work(rockm + boulm, {hand="body", color=function(x, y) return mix("#443e36", "#b9a98a", smoothstep(0.05, 0.85, f:value(x, y))) end,
  angle=f:field("fall"), length={10, 30}, coverage=4, medium=0.12})

--@ chunk 7 · clock 38574.35546875
behind = -((rockm + boulm):grow(0.5))
-- the wood's dark interior behind the edge trees
local inn = noise{seed=61, octaves=5, period=40}
local sp2 = {}
for i, px in ipairs(uneven(16, -10, 330, 0.6, 0.5, 5)) do sp2[i] = {px, rand(60, 190), rand(12, 22)} end
local crest = function(x)
  local top = 470 - 60 * smoothstep(360, 60, x)
  local best = top
  for _, s in ipairs(sp2) do
    local d = math.abs(x - s[1]) / s[3]
    if d < 1 then best = math.min(best, top - s[2]*(1 - d)^1.3) end
  end
  return best
end
woodm = (below(crest) * above(function(x) return 525 end) * mask(function(x, y) return x < 330 + 40*inn(x, y) and 1 or 0 end)):roughen(3, 5, 7, 1) * behind
work(woodm, {hand="hatch", tool="round 2", length={4, 10}, coverage=3, medium=0.2, angle=function(x, y) return (math.floor(x/9)%2==0) and 2.5 or 0.64 end, angle_jitter=0.6,
  color=function(x, y) return mix("#3b4540", "#56605a", 0.5*inn:at01(x, y) + 0.3*smoothstep(520, 380, y)) end})
spruces = {}
local specs = {{40, 530, 830, 41}, {178, 524, 700, 42}, {296, 518, 520, 43}, {112, 522, 430, 44}, {245, 520, 300, 45}}
local trunk, limb, twig = brush("round", 5), brush("round", 2.2), brush("rigger", 1.0)
for k, sp in ipairs(specs) do
  local t = tree{habit="spruce", x=sp[1], y=sp[2], height=sp[3], seed=sp[4]}
  spruces[k] = t
  local dark = mix("#1c231e", "#3a423b", (k >= 4) and 0.55 or 0.1 * k)
  for i, l in ipairs(t.limbs) do
    if #l.pts >= 2 then
      local b = (l.order == 0) and trunk or ((l.w[1] > 1.2) and limb or twig)
      if i % 4 == 1 or b:fullness() < 0.25 then b:reload(l.order == 0 and "#2e2822" or dark, 0.9) end
      b:stroke(l.pts, {pressure=(l.order == 0) and {0.95, 0.35} or {0.8, 0.2}, ramps={0.03, 0.5}, clip=behind})
    end
  end
  local f2 = nil
  for _, l in ipairs(t.limbs) do
    if l.order == 1 and #l.pts >= 2 then
      local w, pts = {}, {}
      local droop = 6 + 10 * rand()
      for j = 1, #l.pts do
        local u = (j-1)/#l.pts
        w[j] = 3 + 9 * (1 - u) * (0.7 + 0.6*rand())
        pts[j] = {l.pts[j][1], l.pts[j][2] + droop * u * u}
      end
      local r = ribbon(pts, w)
      f2 = f2 and (f2 + r) or r
    end
  end
  local nd = f2:roughen(3, 5, k, 0.8) * behind
  local cx = sp[1]
  local shade = noise{seed=70 + k, octaves=3, period=30}
  work(nd, {hand="hatch", tool="round 1.6", length={3, 8}, coverage=2.8,
    color=function(x, y)
      local lit = smoothstep(0.1, -0.6, (x - cx) / 60) * 0.5 + 0.35 * shade:at01(x, y)
      return mix(dark, (k >= 4) and "#4c5647" or "#35433a", lit)
    end,
    angle=function(x, y) return (x < cx and 2.55 or 0.6) + 0.35*math.sin(y/7) end, angle_jitter=0.55})
end

--@ chunk 8 · clock 38574.35546875

dry()
stainn = noise{seed=31, octaves=5, period=60, stretch={1.5708, 4}}
grain = noise{seed=32, octaves=4, period=18}
lich = worley{seed=33, period=14}
stone = function(x, y)
  local v = smoothstep(0.08, 0.9, f:value(x, y))
  local c = gradient({{0, "#4d4842"}, {0.35, "#7a7163"}, {0.7, "#aa9b80"}, {1, "#d6c6a3"}}, v)
  local s = stainn:at01(x, y)
  c = mix(c, "#3f3d38", 0.5 * smoothstep(0.6, 0.9, s))
  c = mix(c, "#6a7253", 0.4 * smoothstep(560, 690, y) * grain:at01(x, y))
  return shift(c, 0.035 * grain(x, y), 0, 0)
end
local faces = rockm * f:lit{parts={1}, soft=0.1}
local shade = rockm * f:shadow{parts={1}, soft=0.1}
work(shade, {hand="body", tool="filbert 4", color=stone, angle=f:field("fall"), length={5, 16}, coverage=3.6, medium=0.12, angle_jitter=0.3})
work(faces, {hand="body", tool="filbert 4", color=stone, angle=function(x, y) return 0.04 + 0.1*grain(x, y) end, length={5, 18}, coverage=3.6, medium=0.12})
work(boulm, {hand="body", tool="filbert 3", color=function(x, y) return mix(stone(x, y), "#687056", 0.35*grain:at01(x*2, y)) end, angle=f:field("across"), length={4, 12}, coverage=3.6, medium=0.12})

--@ chunk 9 · clock 69048.79296875

dry()
local smooth = function(x, y)
  local v = smoothstep(0.08, 0.9, f:value(x, y))
  return gradient({{0, "#4f4a44"}, {0.35, "#7b7264"}, {0.7, "#a99a80"}, {1, "#d2c2a0"}}, v)
end
stipple(rockm:shrink(2), {width=2.2, color=smooth, coverage=1.8, pressure={0.4, 0.8}, dips={20, 0.35, 0.7}, medium=0.3, fade=0.6})
stipple(boulm:shrink(2), {width=2, color=function(x, y) return mix(smooth(x, y), "#6b7258", 0.3) end, coverage=1.8, pressure={0.4, 0.8}, dips={20, 0.35, 0.7}, medium=0.3})

--@ chunk 10 · clock 95568.83984375
dry()
wob = noise{seed=41, octaves=3, period=25}
function wobble(pts, amp)
  local out = {}
  for i = 1, #pts - 1 do
    local a, b = pts[i], pts[i+1]
    local steps = math.max(2, math.floor(math.sqrt((b[1]-a[1])^2 + (b[2]-a[2])^2) / 5))
    for s = 0, steps - 1 do
      local t = s / steps
      local x, y = lerp(a[1], b[1], t), lerp(a[2], b[2], t)
      out[#out+1] = {x + amp*wob(x, y+50), y + amp*wob(x+99, y)}
    end
  end
  out[#out+1] = pts[#pts]
  return out
end
function crack(pts, w, col, press, amp)
  local b = brush{kind="round", width=w, point=0.35}
  b:load(col, 0.95)
  b:stroke(wobble(pts, amp or 2.5), {pressure=press or {0.7, 0.2}, ramps={0.08, 0.4}, shake=0.6, swell={1, 1.5, 0.7, 1.3, 0.9}})
end
local D, D2, L = "#2b2723", "#3d3832", "#d9c9a6"
crack({{305, 398}, {420, 402}, {540, 404}, {640, 404}}, 6, D, {0.7, 0.45})
crack({{648, 410}, {760, 412}, {850, 408}}, 5.5, D, {0.7, 0.4})
crack({{300, 390}, {420, 393}, {560, 395}}, 3, L, {0.5, 0.2}, 1.5)
crack({{641, 272}, {637, 320}, {643, 360}, {639, 402}}, 6.5, D, {0.8, 0.5})
crack({{649, 300}, {647, 350}, {651, 400}}, 2.6, L, {0.4, 0.1}, 1.5)
crack({{325, 262}, {400, 250}, {500, 246}, {600, 252}, {680, 268}}, 3.2, L, {0.55, 0.2}, 2)
crack({{655, 303}, {720, 299}, {800, 304}, {840, 312}}, 3, L, {0.5, 0.2}, 2)
crack({{262, 500}, {300, 512}, {420, 520}, {600, 526}, {820, 530}}, 3, "#bfae8c", {0.5, 0.2}, 2)
crack({{470, 560}, {466, 600}, {472, 640}, {468, 675}}, 4.2, D, {0.7, 0.3})
crack({{612, 565}, {618, 610}, {612, 650}}, 3, D2, {0.6, 0.1})
crack({{770, 560}, {774, 610}, {768, 668}}, 4, D, {0.7, 0.3})
crack({{540, 410}, {536, 450}, {545, 490}, {541, 518}}, 3, D2, {0.6, 0.1})
crack({{742, 414}, {746, 470}, {739, 526}}, 4, D, {0.7, 0.25})
crack({{420, 262}, {428, 310}, {421, 360}, {426, 398}}, 3.4, D2, {0.6, 0.15})
crack({{560, 250}, {556, 300}, {563, 340}}, 2.6, D2, {0.55, 0.05})
crack({{870, 420}, {878, 480}, {872, 540}, {880, 610}}, 3.4, "#25221f", {0.6, 0.2})
local lam = brush{kind="round", width=2, point=0.5}
for i = 1, 16 do
  local y0 = ({440, 462, 478, 590, 612, 636, 300, 330, 358})[1 + (i-1) % 9] + rand(-4, 4)
  local x0 = rand(320, 620); local len = rand(60, 180)
  lam:reload(mix("#5e574d", "#8a7e6a", rand()), 0.6)
  lam:stroke(wobble({{x0, y0}, {x0 + len/2, y0 + rand(-2, 2)}, {x0 + len, y0 + rand(-3, 3)}}, 1.2), {pressure={0.15, 0.45}, ramps={0.3, 0.4}})
end

--@ chunk 11 · clock 96638.71716308594
wait(24*60)
birch = tree{habit="birch", x=478, y=252, height=235, seed=82}
local trunkw = brush{kind="round", width=8, point=0.4}
local limb = brush("round", 2.2)
local twig = brush("rigger", 0.7)
local tw = "#5a4a45"
-- limbs and twigs first, dark purple-brown, fine
for i, l in ipairs(birch.limbs) do
  if l.order >= 1 and #l.pts >= 2 then
    local b = (l.w[1] > 1.6) and limb or twig
    if i % 6 == 1 or b:fullness() < 0.3 then b:reload(mix(tw, "#3d3431", rand()), 0.8) end
    local wb = (l.w[1] > 1.6) and {0.8, 0.25} or {0.55, 0.0}
    b:stroke(l.pts, {pressure=wb, ramps={0.05, 0.6}})
  end
end
-- the white trunk: a light body, a gray shadow side, black marks
local tr = birch.limbs[1]
trunkw:load("#e6e0d0", 1)
trunkw:stroke(tr.pts, {pressure={0.95, 0.35}, ramps={0.02, 0.4}})
local sh = {}
for k, p in ipairs(tr.pts) do sh[k] = {p[1] + 0.3 * tr.w[k], p[2]} end
local shb = brush{kind="round", width=4, point=0.5}
shb:load("#8f8c86", 0.9)
shb:stroke(sh, {pressure={0.8, 0.2}, ramps={0.02, 0.4}})
-- the first order limbs out of the trunk get a white start too
local wl = brush("round", 2.4)
for _, l in ipairs(birch.limbs) do
  if l.order == 1 and l.w[1] > 2 and #l.pts >= 3 then
    wl:reload("#d8d2c4", 0.8)
    local n = math.max(2, #l.pts // 2)
    local pts = {}; for k = 1, n do pts[k] = l.pts[k] end
    wl:stroke(pts, {pressure={0.8, 0.3}, ramps={0.05, 0.5}})
  end
end
-- lenticels and black patches
local mk = brush{kind="flat", width=3}
mk:load("#1f1c1a", 0.9)
local n = #tr.pts
for k = 2, n - 1 do
  local p = tr.pts[k]
  local wdt = tr.w[k]
  if rand() < 0.55 and wdt > 1.5 then
    local off = rand(-0.4, 0.4) * wdt
    local len = wdt * rand(0.3, 0.9)
    mk:stroke({{p[1] + off - len/2, p[2] + rand(-1, 1)}, {p[1] + off + len/2, p[2] + rand(-1, 1)}}, {pressure={0.5, 0.2}, orient="along"})
    if k % 7 == 0 then mk:reload("#1f1c1a", 0.9) end
  end
end
print(#birch.limbs)

--@ chunk 12 · clock 98078.71716308594

wait(3*60)
bl = birch:foliage{sun={-0.7, -0.6, 0.35}, seed=9, fill=0.5}
print(#bl.clumps)
local lf = brush{kind="filbert", width=2.6}
local k = 0
for i, c in ipairs(bl.clumps) do
  if rand() < 0.28 then
    local n = math.random(1, 4)
    for j = 1, n do
      if k % 5 == 0 then lf:reload(mix("#a77f2a", "#dcb650", clamp(c.lit or 0.5, 0, 1) * 0.8 + 0.2 * rand()), 0.8) end
      k = k + 1
      local x, y = c.x + randn(0, c.r * 0.5), c.y + randn(0, c.r * 0.4) + 1.5
      lf:touch(x, y, {pressure=rand(0.35, 0.7), angle=rand(0, 3.14), drag={randn(0, 0.6), rand(0.3, 1.2)}})
    end
  end
end
print(k)

--@ chunk 13 · clock 98258.71716308594

dry()
local notrock = -((rockm + boulm):shrink(1))
local gm = (below(function(x) return 522 + 4*math.sin(x/60) end) * notrock)
pat = noise{seed=91, octaves=5, period=70, stretch={0.05, 3}}
pat2 = noise{seed=92, octaves=4, period=35, stretch={0.0, 2.5}}
floorcol = function(x, y)
  local t = smoothstep(520, 769, y)
  local humus = mix("#6e6450", "#2e261c", smoothstep(0, 0.8, t))
  local moss = mix("#7b8058", "#3c4526", smoothstep(0, 0.8, t))
  local straw = mix("#a09474", "#7d6a42", smoothstep(0, 1, t))
  local litter = mix("#8a7458", "#6b3f22", smoothstep(0, 1, t))
  local a, b = pat:at01(x, y), pat2:at01(x, y)
  local c = humus
  c = mix(c, moss, smoothstep(0.55, 0.75, a) * (x < 420 and 1 or 0.5))
  c = mix(c, straw, smoothstep(0.55, 0.8, b) * 0.8)
  c = mix(c, litter, smoothstep(0.35, 0.15, a) * 0.7)
  return c
end
work(gm, {hand="body", tool="filbert 4", length={6, 18}, coverage=4.2, medium=0.12, aim="masstone",
  angle=function(x, y) return 0.06 * pat2(x, y) end, angle_jitter=0.25, color=floorcol})

--@ chunk 14 · clock 107321.41540527344

dry()
local all = rockm + boulm
local castm = (poly({{840, 660}, {905, 628}, {1000, 632}, {1000, 700}, {940, 712}, {850, 705}}, true):roughen(10, 25, 5, 8) - all):soften(14)
glaze(castm, {color="#2a2119", coats=0.5, pigment="transparent"})
local bw = noise{seed=97, octaves=3, period=50}
local foot = (all:grow(16) - all) * mask(function(x, y)
  local rb = (x > 320 and x < 900) and smoothstep(640, 668, y) or 0
  local bb = (x > 110 and x < 320) and smoothstep(695, 718, y) or 0
  return math.max(rb, bb) * (0.7 + 0.3 * bw:at01(x, y))
end)
glaze(foot:soften(5), {color="#241c15", coats=0.5, pigment="transparent"})
local under = rockm * mask(function(x, y)
  local e = smoothstep(516, 530, y) * smoothstep(590, 548, y)
  return e * (0.75 + 0.25 * bw:at01(x * 2, y))
end)
glaze(under, {color="#2c2620", coats=0.3, pigment="transparent"})
local bg = noise{seed=95, octaves=4, period=20}
work(boulm, {hand="body", tool="filbert 3", length={4, 12}, coverage=3.8, medium=0.12, aim="masstone", angle=f:field("across"),
  color=function(x, y)
    local v = smoothstep(0.05, 0.95, f:value(x, y))
    local c = gradient({{0, "#35312c"}, {0.4, "#5f584d"}, {0.75, "#8a8070"}, {1, "#b5a78b"}}, v)
    local top = smoothstep(660, 630, y) * smoothstep(0.35, 0.65, bg:at01(x, y))
    return mix(c, mix("#46511f", "#76803c", v), 0.8 * top)
  end})

--@ chunk 15 · clock 130865.54431152344
wait(24*60)
local gp = noise{seed=101, octaves=4, period=90, stretch={0.0, 2.5}}
local patches = mask(function(x, y) return smoothstep(0.5, 0.62, gp:at01(x, y) + 0.12 * smoothstep(600, 769, y)) end)
local region = (below(function(x) return 530 end) - (rockm + boulm):shrink(3)) * patches
local tufts = sward{region=region, horizon=470, near=H, height=30, flowers=0, seed=14, spacing=2.6, thin=0.5,
  wind={lean=0.1, gust=0.2, period=180, seed=3}}
local g = brush("rigger", 0.7)
local straws = {"#9c8a5c", "#b3a06a", "#7f7446", "#6c6a3c", "#8e8a52", "#5d5a34", "#c2ae78", "#4a4a2c"}
local n = 0
local keep = -((rockm + boulm):grow(1))
for i, t in ipairs(tufts) do
  if i % 2 == 1 then
    local base = straws[1 + (i // 2) % #straws]
    local under = sample(t.x, t.y, 2)
    g:reload(mix(under, base, 0.12 + 0.35 * rand() * rand()), 0.7)
  end
  local p = clamp(0.2 + 0.45 * t.scale, 0.2, 0.8)
  for _, bl in ipairs(t.blades) do
    g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}, clip=keep})
    n = n + 1
  end
end
print(#tufts, "tufts", n, "blades", region:area())

--@ chunk 16 · clock 132305.54431152344
wait(3*60)
rusts = {"#6e3d20", "#83492a", "#94592f", "#5c3520", "#a2683a", "#4f3322"}
function frond(x, y, len, ang, bend, col, scale)
  local pts = {}
  local n = 12
  for i = 0, n do
    local t = i / n
    pts[#pts+1] = {x + len * t * math.cos(ang + bend * t * 0.6), y + len * t * math.sin(ang + bend * t * 0.6)}
  end
  local r = brush("rigger", 1.1 * scale)
  r:load(shift(color(col), -0.1, 0, 0), 0.8)
  r:stroke(pts, {pressure={0.85, 0.1}, ramps={0.05, 0.6}})
  local p = brush{kind="round", width=2.8 * scale, point=0.5}
  local q = brush{kind="filbert", width=2.2 * scale}
  p:load(col, 0.9); q:load(shift(color(col), 0.05, 0, 0.01), 0.9)
  for i = 3, n do
    local a, b = pts[i-1], pts[i]
    local dx, dy = b[1] - a[1], b[2] - a[2]
    local d = math.sqrt(dx*dx + dy*dy) + 1e-6
    local ux, uy = dx / d, dy / d
    local nx, ny = -uy, ux
    local t = i / n
    local pl = len * 0.42 * (1 - t) ^ 0.7 * (0.8 + 0.4 * rand())
    for side = -1, 1, 2 do
      if rand() < 0.9 then
        local droop = 0.3 * pl
        local sx, sy = b[1], b[2]
        local ex, ey = sx + side * nx * pl + ux * pl * 0.3, sy + side * ny * pl + uy * pl * 0.3 + droop
        p:stroke({{sx, sy}, {(sx + ex) / 2, (sy + ey) / 2 + droop * 0.2}, {ex, ey}}, {pressure={0.8, 0.1}, ramps={0.1, 0.6}})
        -- pinnules: small dabs along the pinna
        local m = math.floor(pl / (2.5 * scale))
        for j = 1, m do
          local u = j / (m + 1)
          local cx, cy = lerp(sx, ex, u), lerp(sy, ey, u) + droop * 0.2 * math.sin(u * math.pi)
          local s2 = (1 - u) * 0.6 + 0.2
          q:touch(cx + side * ux * 1.2, cy + 1.5 * scale, {pressure=clamp(0.3 + 0.5 * s2, 0.2, 0.8), angle=math.atan(ny, nx) + side * 0.6, drag={ux * 0.5, 1.2}})
        end
        if p:fullness() < 0.3 then p:reload(col, 0.9) end
        if q:fullness() < 0.3 then q:reload(shift(color(col), 0.05 * rand(), 0, 0.01), 0.9) end
      end
    end
  end
end
local groups = {
  {335, 672, 9, 90, -1}, {880, 670, 12, 100, 1}, {965, 700, 8, 95, 1},
  {40, 760, 9, 140, 1}, {150, 772, 5, 120, -1}, {590, 775, 6, 110, -1}, {740, 772, 6, 100, 1}, {20, 610, 5, 60, 1}}
-- dark masses under the groups
local dm = nil
for _, g in ipairs(groups) do
  local e = ellipse(g[1], g[2] - g[4] * 0.2, g[4] * 0.7, g[4] * 0.3)
  dm = dm and (dm + e) or e
end
dm = dm:roughen(10, 18, 4, 4) * below(function(x) return 560 end)
work(dm - (rockm + boulm), {hand="hatch", tool="round 2.5", length={5, 14}, coverage=2.5, medium=0.15, angle=function(x, y) return -1.2 + 0.8 * math.sin(x / 13) end, angle_jitter=0.6,
  color=function(x, y) return mix("#3a2518", "#5a3620", rand()) end})
local nf = 0
for gi, g in ipairs(groups) do
  for k = 1, g[3] do
    local x = g[1] + randn(0, g[4] * 0.45)
    local y = g[2] + randn(0, 10)
    local up = -math.pi/2 + randn(0, 0.45) + 0.35 * g[5]
    local scale = clamp((y - 540) / 200, 0.5, 1.3)
    local col = rusts[1 + (gi * 7 + k) % #rusts]
    frond(x, y, g[4] * rand(0.6, 1.1) * scale, up, (rand() < 0.5 and 1 or -1) * rand(0.9, 1.9), col, scale)
    nf = nf + 1
  end
end
print(nf, "fronds")

--@ chunk 17 · clock 132485.54431152344
wait(12*60)
tops = {}
for x = 262, 918, 3 do
  for y = 150, 700 do if rockm:at(x, y) > 0.5 then tops[#tops+1] = {x, y}; break end end
end
local mz = noise{seed=111, octaves=4, period=30}
-- moss cushions along the top, thicker where the stone is flat, hanging a little over the edge
local cush = nil
for i, p in ipairs(tops) do
  local th = 3 + 7 * mz:at01(p[1], 0)
  if mz:at01(p[1], 50) > 0.55 then
    local e = ellipse(p[1], p[2] + th * 0.3, 4 + 3 * mz:at01(p[1], 9), th * 0.7)
    cush = cush and (cush + e) or e
  end
end
cush = cush:roughen(2, 6, 11, 0.8)
work(cush, {hand="hatch", tool="round 1.8", length={2, 5}, coverage=3, medium=0.12, angle=function(x, y) return -1.57 + 0.9 * mz(x * 3, y * 3) end, angle_jitter=0.8,
  color=function(x, y) return mix("#3b3f24", "#6a6d3a", smoothstep(-0.3, 0.7, mz(x * 2, y * 2)) * (x < 700 and 0.9 or 0.5)) end})
-- mossy streaks where water runs from the ledges
local streak = nil
for _, xs in ipairs({{350, 262, 60}, {455, 252, 80}, {610, 255, 50}, {720, 308, 70}, {800, 310, 55}, {300, 380, 60}}) do
  local r = ribbon({{xs[1], xs[2]}, {xs[1] + rand(-4, 4), xs[2] + xs[3] * 0.5}, {xs[1] + rand(-6, 6), xs[2] + xs[3]}}, {6, 3.5, 0.5})
  streak = streak and (streak + r) or r
end
glaze((streak:roughen(3, 8, 3, 3):soften(2) * rockm), {color="#3d3b2e", coats=0.3, pigment="transparent"})
-- dry grass and heather standing on the top, against the sky
local g = brush("rigger", 0.9)
local straws = {"#8d7d52", "#a8955f", "#6f6440", "#b9a46e", "#5d5536"}
for i, p in ipairs(tops) do
  if mz:at01(p[1], 50) > 0.45 and rand() < 0.6 then
    if i % 3 == 1 then g:reload(straws[1 + i % #straws], 0.7) end
    for k = 1, math.random(2, 5) do
      local h = rand(8, 24) * (0.6 + 0.8 * mz:at01(p[1], 90))
      local lean = randn(0.15, 0.35)
      local x0, y0 = p[1] + rand(-2, 2), p[2] + rand(1, 4)
      g:stroke({{x0, y0}, {x0 + lean * h * 0.4, y0 - h * 0.6}, {x0 + lean * h, y0 - h}}, {pressure={0.6, 0}, ramps={0.05, 0.7}})
    end
  end
end
local hb = brush("rigger", 1.1)
for _, hx in ipairs({372, 395, 548, 575, 690, 745, 782, 810}) do
  local yt = 0
  for _, p in ipairs(tops) do if math.abs(p[1] - hx) < 2 then yt = p[2] end end
  hb:reload(mix("#3a2e2c", "#5e4546", rand()), 0.8)
  for k = 1, 9 do
    local a = -math.pi/2 + randn(0, 0.55)
    local l = rand(8, 20)
    local x0 = hx + rand(-4, 4)
    hb:stroke({{x0, yt + 3}, {x0 + math.cos(a) * l * 0.5, yt + 3 + math.sin(a) * l * 0.5}, {x0 + math.cos(a) * l, yt + 3 + math.sin(a) * l}}, {pressure={0.7, 0.05}, ramps={0.05, 0.6}})
  end
  local hd = brush("round", 2.2)
  hd:reload(mix("#5a3f4a", "#7d5a5e", rand()), 0.7)
  for k = 1, 14 do hd:touch(hx + randn(0, 7), yt - rand(2, 16), {pressure=rand(0.3, 0.6)}) end
end
-- the birch roots over the edge
local rb = brush{kind="round", width=3.2, point=0.5}
for _, rt in ipairs({{{474, 252}, {462, 258}, {448, 270}, {440, 290}, {436, 315}}, {{482, 252}, {494, 259}, {505, 272}, {510, 292}}, {{478, 253}, {476, 266}, {472, 282}}, {{470, 252}, {450, 254}, {428, 262}, {425, 275}}}) do
  rb:reload("#6d6154", 0.9)
  rb:stroke(rt, {pressure={0.8, 0.15}, ramps={0.05, 0.6}, shake=0.5})
  local hl = {}
  for k, p in ipairs(rt) do hl[k] = {p[1] - 0.8, p[2] - 0.8} end
  local lb = brush("rigger", 0.8); lb:load("#b3a58e", 0.7)
  lb:stroke(hl, {pressure={0.5, 0.05}, ramps={0.05, 0.6}})
end

--@ chunk 18 · clock 133205.54431152344
wait(24*60)
-- stones on the floor
local st = brush("filbert", 3)
local stones = {}
for i = 1, 16 do
  local x, y
  if i <= 10 then x, y = rand(320, 900), rand(668, 700) else x, y = rand(20, 980), rand(690, 765) end
  local r = (2.5 + 8 * rand() ^ 2) * clamp((y - 540) / 170, 0.5, 1.4)
  if (rockm + boulm):at(x, y) < 0.1 then stones[#stones+1] = {x, y, r} end
end
for _, s in ipairs(stones) do
  local x, y, r = s[1], s[2], s[3]
  local pts = {}
  local nv = math.random(5, 7)
  local a0 = rand(0, 6.28)
  for k = 1, nv do
    local a = a0 + (k - 1) * 6.283 / nv + rand(-0.3, 0.3)
    local rr = r * rand(0.7, 1.15)
    pts[k] = {x + math.cos(a) * rr * 1.35, y + math.sin(a) * rr * 0.7}
  end
  local e = poly(pts):soften(0.6)
  glaze(ellipse(x + r * 0.8, y + r * 0.5, r * 1.5, r * 0.45):soften(r * 0.3) - e, {color="#231b14", coats=0.5, pigment="transparent"})
  local lc = mix("#7f7663", "#a99c80", rand())
  work(e, {hand="detail", tool="filbert 2", length={2, 5}, coverage=3.5, medium=0.12, aim="masstone", angle=0.1,
    color=function(px, py) local v = smoothstep(y + r * 0.3, y - r * 0.6, py) * smoothstep(x + r * 1.2, x - r * 0.8, px) return mix("#35302a", lc, v) end})
end
-- a fallen dead spruce branch across the front
local br = {{30, 760}, {78, 748}, {112, 746}, {150, 734}, {196, 731}, {232, 720}, {262, 719}, {300, 711}, {338, 712}, {372, 704}, {405, 706}}
local w1 = brush{kind="round", width=7, point=0.4}
w1:load("#4f473e", 1)
w1:stroke(br, {pressure={0.95, 0.35}, ramps={0.02, 0.5}, shake=1.2, swell={1, 0.8, 1.15, 0.85, 1, 0.9}})
local hl = {}
for k, p in ipairs(br) do hl[k] = {p[1] + 0.5, p[2] - 2.0 * (1 - k / #br) - 1.0} end
local w2 = brush{kind="round", width=3, point=0.5}
w2:load("#a79d89", 0.7)
w2:stroke(hl, {pressure={0.6, 0.15}, ramps={0.05, 0.5}, swell={1, 0.4, 1, 0.3, 0.9}})
local tw = brush("rigger", 1)
for k = 1, 24 do
  local u = rand(0.05, 0.95)
  local i = 1 + math.floor(u * (#br - 1))
  local a, b = br[i], br[i + 1]
  local t = rand()
  local x, y = lerp(a[1], b[1], t), lerp(a[2], b[2], t)
  local ang = (rand() < 0.5 and -1 or 1) * rand(0.5, 1.3) - 0.2
  local l = rand(10, 34) * (1 - 0.5 * u)
  local c = mix("#4a423a", "#978c79", rand())
  tw:reload(c, 0.8)
  local mx, my = x + math.cos(ang) * l * 0.5, y - math.abs(math.sin(ang)) * l * 0.5 * (ang < 0 and 1 or -0.4)
  local ex, ey = x + math.cos(ang) * l, y - math.abs(math.sin(ang)) * l * (ang < 0 and 1 or -0.4)
  tw:stroke({{x, y}, {mx, my}, {ex, ey}}, {pressure={0.8, 0.1}, ramps={0.05, 0.5}})
  if rand() < 0.5 then
    local sx = lerp(x, ex, 0.6); local sy = lerp(y, ey, 0.6)
    tw:stroke({{sx, sy}, {sx + rand(-6, 6), sy - rand(3, 9)}}, {pressure={0.6, 0}, ramps={0.05, 0.6}})
  end
end
-- dry grass in front of the rock foot, breaking its line
local g = brush("rigger", 0.9)
local straws = {"#8d7d52", "#a8955f", "#6f6440", "#b9a46e", "#5d5536", "#4d4630"}
for i = 1, 150 do
  local x = rand(300, 910)
  local y = 668 + rand(0, 26)
  if i % 4 == 1 then g:reload(straws[1 + i % #straws], 0.7) end
  local h = rand(10, 30)
  local lean = randn(0.1, 0.3)
  g:stroke({{x, y}, {x + lean * h * 0.35, y - h * 0.6}, {x + lean * h, y - h}}, {pressure={0.65, 0}, ramps={0.05, 0.7}})
end
-- seed stalks in the foreground
local sk = brush("rigger", 0.8)
local hd = brush{kind="filbert", width=2.2}
for i = 1, 16 do
  local x, y = rand(20, 980), rand(730, 769)
  if (rockm + boulm):at(x, y) < 0.1 then
    local h = rand(45, 95)
    local lean = randn(0.12, 0.2)
    local tx, ty = x + lean * h, y - h
    sk:reload(mix("#8b7a4e", "#b4a06b", rand()), 0.8)
    sk:stroke({{x, y}, {x + lean * h * 0.3, y - h * 0.55}, {tx, ty}}, {pressure={0.7, 0.2}, ramps={0.05, 0.5}})
    hd:reload(mix("#7d6a44", "#a58f5e", rand()), 0.8)
    for k = 1, 6 do hd:touch(tx - lean * k * 2.4 + rand(-1.5, 1.5), ty + k * 2.6, {pressure=rand(0.3, 0.6), angle=-1.3, drag={0, 1}}) end
  end
end

--@ chunk 19 · clock 152396.62048339844

dry()
local all = rockm + boulm
local fgd = mask(function(x, y)
  local d = smoothstep(560, 769, y)
  local side = smoothstep(350, 0, x) * 0.5 + smoothstep(700, 1000, x) * 0.3
  return clamp(d * 0.8 + side * smoothstep(520, 700, y), 0, 1)
end) - all:shrink(1)
glaze(fgd, {color="#2a2016", coats=0.45, pigment="transparent"})
glaze(boulm * mask(function(x, y) return 0.5 + 0.5 * smoothstep(180, 300, x) end), {color="#2f2c27", coats=0.45, pigment="transparent"})
glaze(rockm * f:shadow{parts={1}, soft=0.25} * mask(function(x, y) return smoothstep(780, 860, x) * 0.7 + 0.3 end), {color="#2e2d2c", coats=0.25, pigment="transparent"})

--@ chunk 20 · clock 278358.19763183594

wait(3*60)
local behind = -((rockm + boulm):grow(0.5))
local specs = {{30, 532, 105, 131}, {58, 546, 62, 132}, {96, 538, 84, 137}, {128, 556, 38, 138}, {205, 540, 56, 134}, {272, 552, 92, 135}, {300, 561, 44, 136}, {176, 566, 26, 139}}
for k, sp in ipairs(specs) do
  local t = tree{habit="spruce", x=sp[1], y=sp[2], height=sp[3], seed=sp[4], years=8}
  local dark = mix("#1c251f", "#2b352c", rand())
  local tb = brush("round", 1.6)
  local lb = brush("rigger", 0.8)
  for i, l in ipairs(t.limbs) do
    if #l.pts >= 2 then
      local b = (l.order == 0) and tb or lb
      if i % 5 == 1 then b:reload(dark, 0.9) end
      b:stroke(l.pts, {pressure={0.8, 0.2}, ramps={0.03, 0.5}, clip=behind})
    end
  end
  local f2 = nil
  for _, l in ipairs(t.limbs) do
    if l.order == 1 and #l.pts >= 2 then
      local w, pts = {}, {}
      for j = 1, #l.pts do local u = (j-1)/#l.pts; w[j] = 1.5 + 4.5 * (1 - u) * (0.7 + 0.6 * rand()); pts[j] = {l.pts[j][1], l.pts[j][2] + 3 * u * u} end
      local r = ribbon(pts, w)
      f2 = f2 and (f2 + r) or r
    end
  end
  if f2 then
    local cx = sp[1]
    work(f2:roughen(2, 4, k, 0.6) * behind, {hand="hatch", tool="round 1.2", length={2, 5}, coverage=2.6,
      color=function(x, y) return mix(dark, "#3f4c3a", (x < cx and 0.4 or 0.05) + 0.2 * rand()) end,
      angle=function(x, y) return (x < cx and 2.55 or 0.6) + 0.3 * math.sin(y / 5) end, angle_jitter=0.5})
  end
  local g = brush("rigger", 0.8)
  for j = 1, 14 do
    if j % 5 == 1 then g:reload(mix("#5d5536", "#8d7d52", rand()), 0.7) end
    local x = sp[1] + randn(0, sp[3] * 0.15); local y = sp[2] + rand(-1, 4); local h = rand(4, 11)
    g:stroke({{x, y}, {x + randn(0, 0.3) * h, y - h}}, {pressure={0.6, 0}, ramps={0.05, 0.7}})
  end
end
