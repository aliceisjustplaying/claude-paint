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
