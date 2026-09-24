-- easel session "willows": a painting replayed chunk by chunk.
--   easel run paintings/lua/willows.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=23, hand=true}; print(H); print(pal); local t=timesheet(); print(t.hours, t.sitting)

--@ chunk 2 · clock 0
HZ = 452
BANK = {{-10,583},{80,580},{170,577},{260,574},{350,571},{420,568},{470,566},{520,566},{580,568},{660,571},{740,575},{830,578},{920,581},{1010,584}}
-- the trunk, drawn as its outline: a squat, swollen, leaning bole, a cleft on the right
TRUNK = {{436,620},{447,604},{458,586},{461,566},{455,546},{447,528},{445,508},{436,494},{428,478},{434,462},{452,454},{474,446},{497,451},{517,441},{540,447},{557,457},{569,472},{562,488},{550,499},{544,512},{549,530},{547,552},{554,574},{566,598},{584,618}}
KNOBS = {{438,470,7,-2.45,0.3},{452,456,9,-2.1,0.28},{474,446,9,-1.85,0.25},{498,448,10,-1.6,0.3},{520,442,10,-1.38,0.26},{542,450,9,-1.1,0.28},{560,464,7,-0.8,0.32}}
RODS = {}
local function rod(x0, y0, a, len, w, bend, depth)
  local pts = {{x0, y0}}
  local n = 5
  local ax = a
  for s = 1, n do
    ax = ax + bend / n + randn(0, 0.025)
    -- the outer rods arch over: gravity pulls their tips
    if math.abs(a + 1.57) > 0.55 then ax = ax + 0.05 * (a < -1.57 and -1 or 1) * s / n end
    local px, py = pts[#pts][1], pts[#pts][2]
    pts[#pts + 1] = {px + math.cos(ax) * len / n, py + math.sin(ax) * len / n}
  end
  RODS[#RODS + 1] = {pts = pts, len = len, w = w, depth = depth}
  return pts
end
for k, kn in ipairs(KNOBS) do
  for i = 1, kn[3] do
    local a = kn[4] + randn(0, kn[5])
    local len = rand(110, 320) * (1 - 0.3 * math.abs(a + 1.57))
    if rand() < 0.12 then len = len * 0.4 end
    local x0, y0 = kn[1] + rand(-5, 5), kn[2] + rand(-4, 4)
    local w = clamp(len / 90, 1.0, 3.2) * rand(0.8, 1.2)
    local pts = rod(x0, y0, a, len, w, randn(0, 0.14), 0)
    -- an older rod forks: one or two side rods from its lower half
    if len > 200 and rand() < 0.35 then
      for f = 1, (rand() < 0.5 and 1 or 2) do
        local j = math.random(2, 3)
        local p = pts[j]
        rod(p[1], p[2], a + (rand() < 0.5 and -1 or 1) * rand(0.2, 0.45), len * rand(0.35, 0.6), w * 0.6, randn(0, 0.1), 1)
      end
    end
  end
end

-- the far shore: a low wood on the left, a row of small pollards on the right, lost in mist at the end
FAR = {{-10,452},{40,450},{62,442},{90,436},{120,433},{150,436},{180,431},{214,434},{246,438},{280,444},{310,449},{360,451},{460,452},{600,451},{700,450},{800,451},{900,452},{1010,452}}
FARWILL = {}
local xs = uneven(11, 612, 905, 0.6, 0.4, 5)
for i, x in ipairs(xs) do FARWILL[i] = {x, 451, rand(7, 14) * (1 - 0.3 * (x - 612) / 300)} end
print(#RODS, #FARWILL)
-- the drawing: a searching 2H pass, then 3B for what I'm sure of
local h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.3})
h:sketch(BANK, {pressure=0.3})
h:sketch(TRUNK, {pressure=0.3})
h:sketch(FAR, {pressure=0.25})
local b = pencil("3B")
b:line(TRUNK, {pressure={0.55, 0.7, 0.65, 0.5}, smooth=true})
b:line(BANK, {pressure={0.45, 0.55, 0.5}})
for i, r in ipairs(RODS) do
  if r.depth == 0 and i % 2 == 1 then b:line(r.pts, {pressure={0.5, 0.2}}) else h:line(r.pts, {pressure={0.4, 0.15}}) end
end
for _, f in ipairs(FARWILL) do h:line({{f[1], f[2]}, {f[1] + randn(0, 1), f[2] - f[3]}}, {pressure=0.35}) end
-- the cleft in the bole
b:line({{516,452},{520,480},{514,510},{518,540},{512,566}}, {pressure={0.4, 0.65, 0.4}})
fix()

--@ chunk 3 · clock 4.672002384904772
umber = pal:only{"raw umber", "bone black", "lead white", "yellow ochre"}
bankM = below(BANK):roughen(1.2, 30, 3, 0.8)
trunkM = poly(TRUNK, true)
-- the sepia lay-in: thin raw umber, in values only, two washes crossing a little
work(bankM, {hand="glaze", color="#5b4633", medium=0.6, coverage=1.4, angle=0.02, length={80, 220}, pal=umber, edge="found"})
work(bankM * below(function(x) return 610 + 10 * math.sin(x / 90) end), {hand="glaze", color="#4d3b2b", medium=0.6, coverage=1.4, angle=-0.06, length={60, 200}, pal=umber, edge="found"})
work(trunkM, {hand="glaze", tool="filbert 7", color="#3d2f23", medium=0.6, coverage=1.4, angle=1.52, length={15, 50}, pal=umber, edge={found=0.5, soft=0.5, period=30}})
work(trunkM, {hand="glaze", tool="filbert 5", color="#3d2f23", medium=0.6, coverage=1.4, angle=1.3, length={15, 40}, pal=umber, edge={found=0.5, soft=0.5, period=30}})

--@ chunk 4 · clock 25.69328863406554
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "chrome yellow"}
-- five piles, mixed on the palette with the knife
SKY = {top="#6d7c94", upper="#949eac", mid="#b0a9a6", low="#d6bd9c", glow="#ead7a4"}
local wob = noise{seed=41, period=260}
local function band(y0, y1)
  return mask(function(x, y)
    local a = y0 + 14 * wob(x, 0) ; local b = y1 + 14 * wob(x + 900, 0)
    return (y >= a and y < b) and 1 or 0 end)
end
skyM = above(function(x) return HZ + 4 end) - trunkM:shrink(3)
local o = {hand="broad", angle=0, coverage=3.2, medium=0.3, pal=skypal, length={90, 240}}
local function pass(m, c) local t = {} for k, v in pairs(o) do t[k] = v end; t.color = c; work(m * skyM, t) end
pass(band(-20, 150), SKY.top)
pass(band(120, 270), SKY.upper)
pass(band(245, 355), SKY.mid)
pass(band(335, 420), SKY.low)
pass(band(400, 520), SKY.glow)
blend(skyM - trunkM:grow(2), {angle=0})

--@ chunk 5 · clock 65.60102064395323
waterpal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "bone black"}
-- the water mirrors the sky, a shade darker: glow under the far shore, then the higher sky toward me
WATER = {glow="#d9c69b", low="#c2ae92", mid="#a09a93", upper="#7f8694"}
local wob = noise{seed=43, period=300}
local function band(y0, y1)
  return mask(function(x, y)
    local a = y0 + 4 * wob(x, 0); local b = y1 + 4 * wob(x + 700, 0)
    return (y >= a and y < b) and 1 or 0 end)
end
waterM = below(function(x) return HZ + 2 end) * above(function(x) return 590 end) - trunkM:grow(2)
local o = {hand="broad", angle=0, coverage=3.2, medium=0.3, pal=waterpal, length={100, 260}, broken=0, tail=0, angle_jitter=0.01, edge="found"}
local function pass(m, c) local t = {} for k, v in pairs(o) do t[k] = v end; t.color = c; work(m * waterM, t) end
pass(band(440, 484), WATER.glow)
pass(band(476, 522), WATER.low)
pass(band(514, 556), WATER.mid)
pass(band(548, 600), WATER.upper)
blend(waterM - trunkM:grow(4), {angle=0, angle_jitter=0.003})

--@ chunk 6 · clock 78.07954365992919
rest(16); sitting{hours=4}; print(drying(500,200), drying(500,520))

--@ chunk 7 · clock 1038.0795436599292
-- the second sky: a thin stipple, each touch from the pile for its band, dirtied with what's there
local wob = noise{seed=47, period=240}
local P = {SKY.top, mix(SKY.top, SKY.upper, 0.5), SKY.upper, mix(SKY.upper, SKY.mid, 0.5), SKY.mid, mix(SKY.mid, SKY.low, 0.5), SKY.low, mix(SKY.low, SKY.glow, 0.5), SKY.glow}
local Y = {-40, 95, 150, 220, 272, 322, 360, 398, 424, 480}
local function edge(i, x, y)   -- 0 above the join i, 1 below it
  if i <= 1 then return 1 end
  if i >= 10 then return 0 end
  local j = Y[i] + 15 * wob(x + 700 * i, 0) + 5 * wob(x * 3.1, 40 * i)
  return smoothstep(j - 5, j + 5, y)
end
SKYB = {}
for i = 1, 9 do
  local c = color(P[i])
  SKYB[i] = {m = mask(function(x, y) return edge(i, x, y) * (1 - edge(i + 1, x, y)) end) * skyM,
    over = function(x, y, under) return shift(mix(under, c, 0.15), 0.012, 0, 0) end}
end
function stband(i)
  local b = SKYB[i]
  stipple(b.m, {width=3.2, coverage=1.7, pressure={0.4, 0.8}, dips={40, 0.45, 0.7}, medium=0.62, pal=skypal, color_over=b.over})
  local t = timesheet(); print(string.format("band %d: sitting %.0f min, %d touches", i, t.sitting, t.touches))
end
rest(2); sitting{hours=8}
stband(1); stband(2)

--@ chunk 8 · clock 1502.581439266447
rest(16); sitting{hours=8}; stband(3); stband(4)

--@ chunk 9 · clock 2746.4506447413005
rest(16); sitting{hours=8}; stband(5); stband(6)

--@ chunk 10 · clock 3910.6074113943614
rest(16); sitting{hours=8}; stband(7); stband(8)

--@ chunk 11 · clock 5018.223361978773
stband(9)

--@ chunk 12 · clock 5092.4990615942515
-- evening streaks: tapered bands, brushed thin and fused into the wet sky at their edges
local function streak(x0, x1, y, th, tilt, seed)
  local pts, ws = {}, {}
  local n = 9
  local sn = noise{seed=seed, period=90}
  for i = 0, n do
    local t = i / n
    local x = x0 + (x1 - x0) * t
    pts[#pts + 1] = {x, y + (x - x0) * tilt + 3 * sn(x, 0)}
    ws[#ws + 1] = th * (0.25 + 0.75 * math.sin(math.pi * t) ^ 0.6) * (0.7 + 0.5 * sn:at01(x, 50))
  end
  return ribbon(pts, ws):blur(3)
end
CL = {
  {m=streak(-60, 470, 410, 12, -0.012, 61), dark="#9d8e90", lit="#e8cb9a"},
  {m=streak(700, 1030, 399, 5, 0.004, 62), dark="#a09298", lit="#e5c79b"},
}
for i, c in ipairs(CL) do
  local d = color(c.dark)
  work(c.m, {hand="broad", tool="flat 8", length={40, 140}, coverage=1.4, load=0.45, medium=0.4, angle=0, broken=0.2, pal=skypal, 
    color_over=function(x, y, under) return mix(under, d, 0.42) end, hug=false})
  -- the glow catches the underside
  local low = c.m * mask(function(x, y) return 1 - c.m:at(x, y + 3) end)
  local l = color(c.lit)
  work(low, {hand="broad", tool="flat 4", length={30, 90}, coverage=1.2, load=0.4, medium=0.4, angle=0, pal=skypal, edge="soft",
    color_over=function(x, y, under) return mix(under, l, 0.5) end})
  blend(c.m:grow(4):blur(2), {angle=0, coverage=1.5})
end
local t = timesheet(); print(t.sitting)

--@ chunk 13 · clock 5095.9054563590325
rest(16); sitting{hours=6}
farpal = pal:only{"lead white", "pale smalt", "cobalt blue", "raw umber", "bone black", "yellow ochre"}
farO = outline{pts=FAR, open=true, char="soft", lobe=7, amount=0.8, seed=71}
farM = farO:below(HZ + 30) * above(function(x) return HZ + 2.5 + 0.8 * math.sin(x / 37) end)
FARC = {wood="#58535c", land="#645d62", mist="#8f8a8f"}
-- the far land: darker under the wood, paler into the mist on the right
local c = function(x, y)
  local m = smoothstep(820, 1000, x)
  return mix(y < HZ - 2 and FARC.wood or FARC.land, FARC.mist, 0.75 * m)
end
work(farM, {hand="detail", tool="round 2.2", length={6, 22}, coverage=3, angle=0, angle_jitter=0.15, pal=farpal, clip=false,
  edge={found=0.55, soft=0.45, period=35, seed=5}, color=c, medium=0.2})
-- the wood's crowns, touched upward along its top
local rb = brush("round", 1.6)
for i = 1, 170 do
  local x = rand(45, 305)
  local top = nil
  for yy = 425, HZ, 0.5 do if farM:at(x, yy) > 0.5 then top = yy; break end end
  if top then
    if i % 12 == 1 then rb:reload(mix(FARC.wood, "#3f3c44", rand(0, 0.5)), 0.6) end
    rb:stroke({{x, top + rand(3, 7)}, {x + randn(0, 0.6), top + rand(-1.5, 0.8)}}, {pressure={0.55, 0.15}, ramps={0.1, 0.5}})
  end
end

--@ chunk 14 · clock 6093.465015465859
-- small far pollards along the far shore, each by hand: bole, knob, a head of many short rods; hazier to the right
local tb, kb, rg = brush("round", 1.5), brush("round", 1.2), brush{kind="rigger", width=0.5, point=1}
for i, f in ipairs(FARWILL) do
  local x, y, h = f[1], f[2], f[3] * 1.2
  local haze = smoothstep(640, 920, x) * 0.6
  local col = mix("#4e4a52", FARC.mist, haze)
  local bh = h * rand(0.3, 0.42)
  local lean = randn(0, 0.06)
  tb:reload(col, 0.7)
  tb:stroke({{x, y + 1}, {x + lean * bh, y - bh}}, {pressure={0.85, 0.7}, ramps={0.05, 0.2}})
  local hx, hy = x + lean * bh, y - bh
  local cr = (h - bh) * rand(0.45, 0.6)          -- the head's radius
  local sky = sample(hx, hy - h, 3)
  -- the head's mass: soft touches of the rods' color thinned with the sky
  kb:reload(mix(col, sky, 0.22), 0.6)
  for k = 1, math.random(14, 22) do
    local a, r = rand(-3.3, 0.2), cr * math.sqrt(rand()) * 0.9
    kb:touch(hx + math.cos(a) * r, hy - cr * 0.8 + math.sin(a) * r * 0.9, {pressure=rand(0.25, 0.5), drag={0, -rand(0.3, 1)}})
  end
  rg:reload(mix(col, sky, 0.2), 0.7)
  for r = 1, math.random(14, 22) do
    local a = -1.57 + randn(0, 0.55)
    local len = cr * rand(1.0, 2.0)
    rg:stroke({{hx + randn(0, 0.5), hy}, {hx + math.cos(a) * len * 0.5, hy + math.sin(a) * len * 0.5}, {hx + math.cos(a) * len, hy + math.sin(a) * len}},
      {pressure={0.45, 0.02}, ramps={0.05, 0.7}})
  end
end

--@ chunk 15 · clock 6098.579453478102
-- the far shore mirrored in the still water: the wood's height turned down, darker than the water, lost below
local sn = noise{seed=81, period=40, stretch={0, 6}}
reflM = mask(function(x, y)
  if y < HZ + 2 then return 0 end
  -- the height of the far land at x, found on its mask
  local top = HZ + 3
  for yy = 420, HZ + 3, 1 do if farM:at(x, yy) > 0.5 then top = yy; break end end
  local depth = (HZ + 3 - top) * 0.9 + 2.5
  return 1 - smoothstep(HZ + 2 + depth * 0.6, HZ + 2 + depth * (1.1 + 0.3 * sn(x, y)), y)
end)
glaze(reflM:blur(1.2), {color="#4a4552", coats=0.55, pigment="transparent"})

--@ chunk 16 · clock 21715.97642190056
bankpal = pal:only{"lead white", "yellow ochre", "raw umber", "bone black", "pale smalt", "red earth"}
BANKP = {top="#4b4737", mid="#3c3729", low="#2d2a22"}
local bw = noise{seed=91, period=180}
local function under(dy, amp) return function(x) local b = 0; for i = 1, #BANK - 1 do if x >= BANK[i][1] and x < BANK[i+1][1] then local t = (x - BANK[i][1]) / (BANK[i+1][1] - BANK[i][1]); b = BANK[i][2] + t * (BANK[i+1][2] - BANK[i][2]) end end; return b + dy + amp * bw(x, dy) end end
bankTop = function(x) return under(0, 0)(x) end
local topBand = bankM * above(under(26, 8))
local midBand = bankM * below(under(18, 8)) * above(under(80, 14))
local lowBand = bankM * below(under(70, 14))
local o = {hand="body", tool="filbert 6", length={25, 70}, coverage=2.6, medium=0.18, pal=bankpal, angle=function(x, y) return -0.02 + 0.04 * bw(x, y) end}
local function pass(m, c, e) local t = {} for k, v in pairs(o) do t[k] = v end; t.color = c; t.edge = e; work(m, t) end
pass(lowBand, BANKP.low, "soft")
pass(midBand, BANKP.mid, "soft")
pass(topBand, BANKP.top, {found=0.4, soft=0.6, period=40, seed=9})
local t = timesheet(); print(t.sitting)

--@ chunk 17 · clock 21788.34510953026
blend(bankM:shrink(4), {angle=0, coverage=1.6, length={60, 180}})

--@ chunk 18 · clock 21791.200350828934
rest(16); sitting{hours=6}
barkpal = pal:only{"lead white", "yellow ochre", "raw umber", "bone black", "pale smalt", "red earth"}
BARK = {dark="#29241f", body="#342c25", ridge="#4a4036", rim="#6f6c70", cleft="#1a1715"}
-- the bole: body strokes up the form, found against the light
local lean = function(x, y) return -1.57 + 0.004 * (x - 500) end
work(trunkM, {hand="body", tool="filbert 4", length={12, 36}, coverage=2.8, medium=0.15, pal=barkpal, angle=lean, angle_jitter=0.2,
  color=function(x, y) return mix(BARK.body, BARK.dark, smoothstep(450, 610, y) * 0.7) end, edge={found=0.65, soft=0.35, period=25, seed=13}})
local t = timesheet(); print(t.sitting)

--@ chunk 19 · clock 22775.981428213883
-- close the gap round the bole: the bark carried a little past the drawn line
trunk2 = trunkM:grow(2.5)
work(trunk2 - trunkM:shrink(2), {hand="detail", tool="round 2.5", length={6, 18}, coverage=2.6, medium=0.15, pal=barkpal, clip=false,
  angle=function(x, y) return -1.57 + 0.004 * (x - 500) end, color=BARK.body, edge={found=0.5, soft=0.5, period=20, seed=17}})
-- furrows: long wavering vertical lines of the darkest bark, and the cleft
local fb = brush{kind="round", width=1.6, point=0.6}
local sn = noise{seed=101, period=30}
for i = 1, 38 do
  local x0 = rand(445, 560)
  local y0, y1 = rand(470, 520), rand(560, 620)
  local pts = {}
  for k = 0, 6 do
    local y = y0 + (y1 - y0) * k / 6
    pts[#pts + 1] = {x0 + 5 * sn(x0, y) + 0.06 * (y - 540) * (x0 - 500) / 60, y}
  end
  if trunkM:at(pts[1][1], pts[1][2]) > 0.5 and trunkM:at(pts[#pts][1], pts[#pts][2]) > 0.5 then
    if i % 5 == 1 then fb:reload(BARK.dark, 0.7) end
    fb:stroke(pts, {pressure={rand(0.3, 0.6), rand(0.1, 0.4)}, ramps={0.15, 0.3}, shake=0.6, clip=trunkM})
  end
end
fb:reload(BARK.cleft, 0.9)
fb:stroke({{516,452},{520,480},{514,510},{518,540},{512,566},{516,590}}, {pressure={0.9, 0.7, 0.4}, ramps={0.1, 0.4}, shake=0.5})
fb:stroke({{519,456},{522,478},{517,506},{520,530}}, {pressure={0.9, 0.3}, ramps={0.1, 0.4}, shake=0.5})
-- ridges between the furrows, lit faintly by the sky on the left and on the top of the head
local rb = brush{kind="round", width=1.2, point=0.5}
for i = 1, 40 do
  local x0 = rand(440, 540)
  local y0 = rand(450, 590)
  local len = rand(8, 30)
  local lit = smoothstep(530, 440, x0) * 0.6 + smoothstep(500, 455, y0) * 0.5
  if trunkM:shrink(1.5):at(x0, y0) > 0.5 and rand() < 0.3 + lit then
    rb:reload(mix(BARK.ridge, BARK.rim, clamp(lit, 0, 1) * 0.5), 0.5)
    rb:stroke({{x0, y0}, {x0 + randn(0, 1.2), y0 + len * 0.5}, {x0 + randn(0, 1.5), y0 + len}}, {pressure={0.35, 0.1}, ramps={0.2, 0.5}, clip=trunkM})
  end
end
-- the cool rim of sky light: a few broken strokes just inside the upper left flank and the head
local rimb = brush{kind="round", width=1.1, point=0.6}
local P = TRUNK
for k = 4, 13 do
  local a, b = P[k], P[k + 1]
  if rand() < 0.75 then
    local dx, dy = b[1] - a[1], b[2] - a[2]
    local L = math.sqrt(dx * dx + dy * dy)
    local nx, ny = dy / L, -dx / L         -- inward for this (counterclockwise on screen) run
    if trunkM:at(a[1] + nx * 3, a[2] + ny * 3) < 0.5 then nx, ny = -nx, -ny end
    local t0, t1 = rand(0, 0.3), rand(0.6, 1)
    local off = rand(1.0, 2.2)
    rimb:reload(mix(BARK.rim, BARK.ridge, rand(0.2, 0.6)), 0.5)
    rimb:stroke({{a[1] + dx * t0 + nx * off, a[2] + dy * t0 + ny * off}, {a[1] + dx * t1 + nx * (off + randn(0, 0.4)), a[2] + dy * t1 + ny * (off + randn(0, 0.4))}},
      {pressure={rand(0.25, 0.45), rand(0.05, 0.2)}, ramps={0.25, 0.5}, shake=0.4})
  end
end
local t = timesheet(); print(t.sitting)

--@ chunk 20 · clock 22799.88660844462
-- the lower bole again, over the bank paint that came up into it; the foot sinks into the bank
local ln = noise{seed=23, period=35}
local low = trunkM * mask(function(x, y) return smoothstep(545 + 14 * ln(x, 0), 575 + 14 * ln(x, 9), y) end)
work(low, {hug=false, hand="body", tool="filbert 4", length={10, 28}, coverage=2.6, medium=0.15, pal=barkpal, angle=-1.57, angle_jitter=0.25,
  color=BARK.dark, edge={found=0.5, soft=0.5, period=20, seed=19}})
-- knobs on the head where the rods sprout: burrs, lumpy against the light
local kb = brush{kind="round", width=5, stiffness=0.6}
for k, kn in ipairs(KNOBS) do
  for j = 1, 3 do
    kb:reload(mix(BARK.body, BARK.dark, rand()), 0.7)
    local x, y = kn[1] + randn(0, 4), kn[2] + randn(0, 2.5) + 3
    kb:touch(x, y, {pressure=rand(0.5, 0.9), drag={randn(0, 0.6), -rand(0.3, 1.2)}, twist=rand(-0.3, 0.3)})
  end
end
-- the furrows carried down over it
local fb = brush{kind="round", width=1.6, point=0.6}
local sn = noise{seed=103, period=30}
for i = 1, 26 do
  local x0 = rand(450, 570)
  local y0, y1 = rand(540, 570), rand(600, 622)
  local pts = {}
  for k = 0, 5 do
    local y = y0 + (y1 - y0) * k / 5
    pts[#pts + 1] = {x0 + 4 * sn(x0, y) + 0.12 * (y - 560) * (x0 - 505) / 60, y}
  end
  if trunkM:at(pts[1][1], pts[1][2]) > 0.5 then
    if i % 5 == 1 then fb:reload(i % 2 == 0 and BARK.cleft or BARK.body, 0.7) end
    fb:stroke(pts, {pressure={rand(0.3, 0.6), rand(0.1, 0.4)}, ramps={0.15, 0.3}, shake=0.6, clip=trunkM})
  end
end
local t = timesheet(); print(t.sitting)

--@ chunk 21 · clock 22813.09234359162
-- the rods: each drawn from its knob to the tip with a pointed brush, tapering; three piles
RODP = {old="#2c2521", mid="#3a2c26", young="#4e3629"}
-- a crop of thin young shoots among the rods, short and upright
for k, kn in ipairs(KNOBS) do
  for i = 1, math.random(9, 13) do
    local a = -1.57 + 0.55 * (kn[4] + 1.57) + randn(0, 0.3)
    local len = rand(35, 110)
    local x0, y0 = kn[1] + rand(-6, 6), kn[2] + rand(-2, 4)
    local pts = {{x0, y0}}
    local ax = a
    for s = 1, 4 do ax = ax + randn(0, 0.04); pts[#pts + 1] = {pts[#pts][1] + math.cos(ax) * len / 4, pts[#pts][2] + math.sin(ax) * len / 4} end
    RODS[#RODS + 1] = {pts = pts, len = len, w = rand(0.7, 1.1), depth = 2}
  end
end
print(#RODS)
local brushes = {}
local function pick(w)
  local key = string.format("%.1f", math.max(0.6, math.floor(w * 5 + 0.5) / 5))
  if not brushes[key] then brushes[key] = brush{kind="round", width=tonumber(key), point=1} end
  return brushes[key]
end
for i, r in ipairs(RODS) do
  local b = pick(r.w * 0.75)
  local c = r.depth == 2 and RODP.young or (r.len > 200 and RODP.old or (rand() < 0.5 and RODP.mid or RODP.young))
  b:reload(c, rand(0.6, 0.85))
  b:stroke(r.pts, {pressure={rand(0.7, 0.9), rand(0.2, 0.35), 0}, ramps={0.03, rand(0.25, 0.4)}, shake=0.25})
end
local t = timesheet(); print(t.sitting)

--@ chunk 22 · clock 22826.234600984026
rest(20); sitting{hours=6}; print(drying(200,600), drying(200,690), drying(500,600), drying(480,300))

--@ chunk 23 · clock 24026.234600984026
-- grass, blade by blade: tussocks on the bank's crest against the water, rushes, and the bank's face
GR = {dark="#27251e", olive="#3a3627", straw="#7a6a4a", pale="#948259"}
local wind = noise{seed=111, period=160}
local g1 = brush{kind="rigger", width=0.9, point=1}
local g2 = brush{kind="rigger", width=0.6, point=1}
local function blade(b, x, y, h, lean, curl, p)
  local tx, ty = x + lean * h, y - h
  b:stroke({{x, y}, {x + lean * h * 0.35, y - h * 0.5}, {tx + curl * h * 0.3, ty + math.abs(curl) * h * 0.2}}, {pressure={p, 0}, ramps={0.04, 0.7}, shake=0.2})
end
-- 1. tussocks along the crest, spaced by hand
local xs = uneven(80, 4, 996, 0.7, 0.5, 7)
local n = 0
for i, x0 in ipairs(xs) do
  local y0 = bankTop(x0) + rand(1, 5)
  local size = rand(0.5, 1.4) * (1 + 0.4 * wind:at01(x0, 3))
  local count = math.floor(rand(14, 30) * size)
  for k = 1, count do
    local lean = 0.25 * wind(x0, 0) + randn(0, 0.28)
    local h = rand(7, 26) * size
    local b = (k % 3 == 0) and g2 or g1
    if n % 6 == 0 then b:reload(k % 4 == 0 and GR.olive or GR.dark, 0.7) end
    if k > count - 2 and rand() < 0.5 then b:reload(mix(GR.straw, GR.dark, rand(0.1, 0.5)), 0.6) end
    blade(b, x0 + randn(0, 2.5 * size), y0 + rand(0, 3), h, lean, randn(0, 0.25), rand(0.45, 0.75))
    n = n + 1
  end
end
-- the crest between the tussocks: low grass all along, so the bank has no ruled edge
for x = -4, 1004, 1.6 do
  local y0 = bankTop(x) + rand(0, 4)
  if n % 9 == 0 then g2:reload(rand() < 0.7 and GR.dark or GR.olive, 0.7) end
  blade(g2, x + rand(-0.8, 0.8), y0, rand(3, 10), 0.2 * wind(x, 0) + randn(0, 0.3), randn(0, 0.2), rand(0.4, 0.7))
  n = n + 1
end
-- 2. rushes: straight stems in a few clumps, one or two snapped
local rb = brush{kind="rigger", width=0.8, point=1}
for _, cx in ipairs({92, 318, 671, 866}) do
  local y0 = bankTop(cx) + 2
  rb:reload(GR.dark, 0.8)
  for k = 1, math.random(10, 18) do
    local x = cx + randn(0, 4)
    local h = rand(16, 42)
    local a = -1.57 + randn(0, 0.12) + 0.08 * wind(cx, 0)
    local pts = {{x, y0 + rand(0, 3)}, {x + math.cos(a) * h * 0.5, y0 - h * 0.5}}
    if rand() < 0.12 then a = a + (rand() < 0.5 and -1 or 1) * rand(0.8, 1.4) end
    pts[3] = {pts[2][1] + math.cos(a) * h * 0.5, pts[2][2] + math.sin(a) * h * 0.5}
    if k % 5 == 0 then rb:reload(GR.dark, 0.8) end
    rb:stroke(pts, {pressure={0.6, 0.05}, ramps={0.03, 0.5}, shake=0.1})
  end
end
-- 3. the bank's face: fewer, taller blades toward me, a few pale stalks catching the sky
local fb = brush{kind="rigger", width=1.1, point=1}
local patch = noise{seed=113, period=90}
local i = 0
while i < 480 do
  local x = rand(-5, 1005)
  local y = rand(bankTop(x) + 12, 718)
  local pz = patch:at01(x, y)
  if rand() < pz * pz * 1.6 then
  i = i + 1
  local depth = (y - 570) / 150
  local h = (5 + 26 * depth) * rand(0.5, 1.5)
  local pale = pz > 0.72 and rand() < 0.25
  local b = depth > 0.5 and fb or g1
  if i % 7 == 1 or pale then b:reload(pale and mix(GR.straw, GR.olive, rand(0.35, 0.7)) or (rand() < 0.5 and GR.dark or "#4a4432"), 0.7) end
  blade(b, x, y, h, 0.2 * wind(x, y) + randn(0, 0.25), randn(0, 0.25), 0.35 + 0.45 * depth)
  end
end
-- 4. grass round the willow's foot
for i = 1, 90 do
  local x = rand(430, 590)
  local y = 612 + rand(-6, 10)
  if i % 6 == 1 then g1:reload(i % 12 == 1 and GR.olive or GR.dark, 0.7) end
  blade(g1, x, y, rand(6, 20), randn(0, 0.3), randn(0, 0.25), rand(0.5, 0.75))
end
local t = timesheet(); print(t.sitting)

--@ chunk 24 · clock 24099.824012279976
-- the bank's face: many blades close to the ground's own value, clumped, longer toward me
local pn = noise{seed=121, period=60, octaves=3}
local wind = noise{seed=111, period=160}
local cols = {"#2a271f", "#312d23", "#433d2d", "#4d4633", "#23211b"}
local b1 = brush{kind="rigger", width=0.8, point=1}
local b2 = brush{kind="rigger", width=1.2, point=1}
local i, tries = 0, 0
while i < 2400 and tries < 40000 do
  tries = tries + 1
  local x = rand(-5, 1005)
  local y = rand(bankTop(x) + 8, 720)
  local pz = pn:at01(x, y)
  if rand() < smoothstep(0.35, 0.75, pz) then
    i = i + 1
    local depth = clamp((y - 575) / 140, 0, 1)
    local b = depth > 0.45 and b2 or b1
    if i % 5 == 1 then b:reload(cols[math.random(1, #cols)], 0.7) end
    local h = (4 + 22 * depth) * rand(0.5, 1.4)
    local lean = 0.22 * wind(x, y) + randn(0, 0.22)
    local curl = randn(0, 0.2)
    b:stroke({{x, y}, {x + lean * h * 0.35, y - h * 0.5}, {x + lean * h + curl * h * 0.3, y - h + math.abs(curl) * h * 0.2}},
      {pressure={0.3 + 0.4 * depth, 0}, ramps={0.04, 0.7}, shake=0.2})
  end
end
print(i)
local t = timesheet(); print(t.sitting)

--@ chunk 25 · clock 24151.705882847775
-- the bole straighter: its waist filled out on both flanks (a pollard's bole is a post, not a vase)
TRUNKB = {{436,620},{447,604},{452,584},{449,562},{446,540},{441,518},{437,496},{428,478},{434,462},{452,454},{474,446},{497,451},{517,441},{540,447},{557,457},{569,472},{562,488},{555,502},{556,524},{558,548},{561,572},{568,598},{584,618}}
trunkB = outline{pts=TRUNKB, char="broken", amount=0.5, seed=131}:mask()
local add = trunkB - trunkM:shrink(1)
work(add, {hand="detail", tool="round 2.5", length={8, 24}, coverage=2.8, medium=0.15, pal=barkpal, clip=false,
  angle=-1.57, angle_jitter=0.2, color=function(x, y) return mix(BARK.body, BARK.dark, smoothstep(470, 610, y) * 0.7) end,
  edge={found=0.6, soft=0.4, period=20, seed=23}})
trunkM = trunkM + trunkB
local fb = brush{kind="round", width=1.4, point=0.6}
local sn = noise{seed=133, period=30}
for i = 1, 16 do
  local left = i % 2 == 0
  local x0 = left and rand(441, 458) or rand(546, 560)
  local y0, y1 = rand(480, 520), rand(560, 610)
  local pts = {}
  for k = 0, 5 do local y = y0 + (y1 - y0) * k / 5; pts[#pts + 1] = {x0 + 3 * sn(x0, y), y} end
  if i % 4 == 1 then fb:reload(i % 8 == 1 and BARK.cleft or BARK.dark, 0.7) end
  fb:stroke(pts, {pressure={rand(0.3, 0.55), rand(0.1, 0.3)}, ramps={0.15, 0.3}, shake=0.6, clip=trunkM})
end
local t = timesheet(); print(t.sitting)

--@ chunk 26 · clock 24164.75727960514
-- furrows long enough to cross the old seam low on the bole
local fb = brush{kind="round", width=1.8, point=0.6}
local sn = noise{seed=141, period=26}
for i = 1, 30 do
  local x0 = rand(448, 562)
  local y0, y1 = rand(515, 545), rand(590, 616)
  local pts = {}
  for k = 0, 6 do local y = y0 + (y1 - y0) * k / 6; pts[#pts + 1] = {x0 + 4 * sn(x0, y) + 0.1 * (y - 560) * (x0 - 505) / 60, y} end
  if i % 4 == 1 then fb:reload(i % 3 == 0 and "#3b322a" or BARK.dark, 0.7) end
  fb:stroke(pts, {pressure={rand(0.35, 0.7), rand(0.2, 0.5)}, ramps={0.2, 0.3}, shake=0.6, clip=trunkM})
end
-- the foot seated in grass: dense blades across its line
local g1 = brush{kind="rigger", width=1.0, point=1}
local g2 = brush{kind="rigger", width=0.7, point=1}
local wind = noise{seed=111, period=160}
local cols = {"#2a271f", "#312d23", "#433d2d", "#23211b", "#5a5038"}
for i = 1, 420 do
  local x = rand(420, 600)
  local edge = 606 + 12 * math.exp(-((x - 510) / 70) ^ 2)
  local y = edge + rand(-10, 12)
  local b = i % 3 == 0 and g2 or g1
  if i % 6 == 1 then b:reload(cols[math.random(1, #cols)], 0.7) end
  local h = rand(6, 26)
  local lean = 0.22 * wind(x, y) + randn(0, 0.3)
  local curl = randn(0, 0.25)
  b:stroke({{x, y}, {x + lean * h * 0.35, y - h * 0.5}, {x + lean * h + curl * h * 0.3, y - h + math.abs(curl) * h * 0.2}},
    {pressure={rand(0.4, 0.7), 0}, ramps={0.04, 0.7}, shake=0.2})
end
local t = timesheet(); print(t.sitting)

--@ chunk 27 · clock 24175.04935827898
-- bark strokes across the seam, the color of the bark just above it
local sn = noise{seed=151, period=20}
local seam = trunkM:shrink(3) * mask(function(x, y) local c = 556 + 6 * sn(x, 0); return smoothstep(c - 14, c - 6, y) * (1 - smoothstep(c + 6, c + 14, y)) end)
work(seam, {hand="detail", tool="filbert 3", length={10, 22}, coverage=2, medium=0.15, pal=barkpal, angle=-1.57, angle_jitter=0.2, clip=false, hug=false,
  color=function(x, y) return sample(505 + (x - 505) * 0.75, 535, 2) end})
-- a low mound of earth and grass over the foot line
local mn = noise{seed=153, period=30}
local mound = mask(function(x, y)
  local top = 608 + 5 * mn(x, 0) - 6 * math.exp(-((x - 505) / 60) ^ 2)
  return smoothstep(top - 2, top + 2, y) * smoothstep(410, 440, x) * (1 - smoothstep(580, 610, x)) * (1 - smoothstep(632, 640, y))
end)
work(mound, {hand="body", tool="filbert 4", length={10, 30}, coverage=2.4, medium=0.15, pal=bankpal, angle=-0.1, hug=false,
  color=function(x, y) return sample(x < 505 and x - 70 or x + 70, y + 4, 4) end})
local g1 = brush{kind="rigger", width=1.0, point=1}
local g2 = brush{kind="rigger", width=0.7, point=1}
local wind = noise{seed=111, period=160}
local cols = {"#4a4432", "#433d2d", "#5a5038", "#6b5d40", "#2f2b22"}
for i = 1, 300 do
  local x = rand(425, 595)
  local top = 608 + 5 * mn(x, 0) - 6 * math.exp(-((x - 505) / 60) ^ 2)
  local y = top + rand(0, 14)
  local b = i % 3 == 0 and g2 or g1
  if i % 5 == 1 then b:reload(cols[math.random(1, #cols)], 0.7) end
  local h = rand(6, 24)
  local lean = 0.22 * wind(x, y) + randn(0, 0.3)
  local curl = randn(0, 0.25)
  b:stroke({{x, y}, {x + lean * h * 0.35, y - h * 0.5}, {x + lean * h + curl * h * 0.3, y - h + math.abs(curl) * h * 0.2}},
    {pressure={rand(0.4, 0.7), 0}, ramps={0.04, 0.7}, shake=0.2})
end
local t = timesheet(); print(t.sitting)

--@ chunk 28 · clock 24190.847946733702
-- more rods: the crown fuller, a broom, not a few spikes; some older ones bowed outward
local brushes = {}
local function pick(w)
  local key = string.format("%.1f", math.max(0.6, math.floor(w * 5 + 0.5) / 5))
  if not brushes[key] then brushes[key] = brush{kind="round", width=tonumber(key), point=1} end
  return brushes[key]
end
local added = 0
for k, kn in ipairs(KNOBS) do
  for i = 1, math.random(9, 14) do
    local a = kn[4] + randn(0, kn[5] * 1.3)
    local len = rand(120, 300) * (1 - 0.28 * math.abs(a + 1.57))
    local x0, y0 = kn[1] + rand(-7, 7), kn[2] + rand(-3, 4)
    local bow = randn(0, 0.12) + 0.08 * (a + 1.57)
    local pts = {{x0, y0}}
    local ax = a
    for s = 1, 6 do
      ax = ax + bow / 6 + randn(0, 0.02)
      pts[#pts + 1] = {pts[#pts][1] + math.cos(ax) * len / 6, pts[#pts][2] + math.sin(ax) * len / 6}
    end
    local ok = pts[#pts][2] < y0 - 40
    for _, p in ipairs(pts) do if p[2] > y0 + 2 then ok = false end end
    if ok then
    local w = clamp(len / 110, 0.8, 2.4) * rand(0.8, 1.15)
    local b = pick(w * 0.75)
    b:reload(rand() < 0.4 and RODP.old or (rand() < 0.6 and RODP.mid or RODP.young), rand(0.6, 0.85))
    b:stroke(pts, {pressure={rand(0.7, 0.9), rand(0.2, 0.35), 0}, ramps={0.03, rand(0.25, 0.4)}, shake=0.25})
    RODS[#RODS + 1] = {pts = pts, len = len, w = w, depth = 3}
    added = added + 1
    end
  end
end
print(added, #RODS)
local t = timesheet(); print(t.sitting)

--@ chunk 29 · clock 24199.496261179913
-- the water's surface: a few faint level lines of light, closer and shorter toward the far shore
local lb = brush{kind="rigger", width=0.9, point=1}
local db = brush{kind="round", width=1.4}
local y = HZ + 7
local n = 0
while y < 560 do
  local d = (y - HZ) / 110
  local cnt = math.random(1, 3)
  for k = 1, cnt do
    local len = rand(40, 180) * (0.5 + d)
    local x0 = rand(-40, 1000)
    if not (x0 + len > 425 and x0 < 575) or rand() < 0.2 then
      local under = sample(x0 + len / 2, y, 3)
      if rand() < 0.75 then
        lb:reload(shift(mix(under, color(SKY.glow), 0.6), 0.03, 0, 0), 0.5)
        lb:stroke({{x0, y}, {x0 + len * 0.5, y + randn(0, 0.2)}, {x0 + len, y + randn(0, 0.3)}}, {pressure={rand(0.25, 0.45), 0.05}, ramps={0.3, 0.4}})
      else
        db:reload(shift(under, -0.04, 0, -0.01), 0.4)
        db:stroke({{x0, y}, {x0 + len * 0.6, y + randn(0, 0.2)}, {x0 + len * 0.8, y}}, {pressure={0.25, 0.1}, ramps={0.3, 0.4}})
      end
      n = n + 1
    end
  end
  y = y + 2.5 + d * 9 + rand(0, 4)
end
print(n)
