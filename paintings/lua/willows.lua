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
