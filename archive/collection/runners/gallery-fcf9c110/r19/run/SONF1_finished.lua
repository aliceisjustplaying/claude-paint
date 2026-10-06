-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=560, aspect=1.4, linen={18, 15}, seed=11,
  ground={
    {pile={{"red earth", 3}, {"yellow ochre", 2}, {"lead white", 3}}, um=120, apply="knife", texture=0.3},
    {pile={{"lead white", 10}, {"yellow ochre", 0.6}, {"raw umber", 0.2}}, um=60, apply="brush"}
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
-- swatch test strip (bottom of canvas, will be covered by dark foreground later)
P = {}
P.smalt_w1   = pile{{"smalt",1},{"lead white",1}}
P.smalt_w3   = pile{{"smalt",1},{"lead white",3}}
P.psmalt_w2  = pile{{"pale smalt",1},{"lead white",2}}
P.cobalt_w2  = pile{{"cobalt blue",1},{"lead white",2}}
P.prus_w     = pile{{"Prussian blue",0.3},{"lead white",4}}
P.sm_co_w    = pile{{"smalt",2},{"cobalt blue",1},{"lead white",2}}
P.w_cy       = pile{{"lead white",6},{"chrome yellow",1}}
P.w_cy_v     = pile{{"lead white",6},{"chrome yellow",1},{"vermilion",0.5}}
P.w_yo_v     = pile{{"lead white",6},{"yellow ochre",1},{"vermilion",0.3}}
P.rose       = pile{{"lead white",4},{"vermilion",1},{"smalt",0.3}}
local names = {"smalt_w1","smalt_w3","psmalt_w2","cobalt_w2","prus_w","sm_co_w","w_cy","w_cy_v","w_yo_v","rose"}
local x = 8
for i, n in ipairs(names) do
  local m = rect(x, 652, 46, 26)
  work(m, {hand="body", pile=P[n], coverage=3, angle=0.2, clip=true})
  x = x + 50
end
print("done")

--@ chunk 3
P.smalt_only = pile{{"smalt",1}}
P.sm3co1w1   = pile{{"smalt",3},{"cobalt blue",1},{"lead white",1}}
P.co_sm      = pile{{"cobalt blue",1},{"smalt",1}}
P.deep       = pile{{"Prussian blue",0.15},{"smalt",2},{"lead white",2}}
P.pgreen     = pile{{"lead white",5},{"Prussian blue",0.2},{"chrome yellow",1}}
P.gearth_w   = pile{{"green earth",1},{"lead white",1}}
P.blk_umb    = pile{{"bone black",1},{"raw umber",1}}
P.prus_umb   = pile{{"Prussian blue",1},{"raw umber",1}}
P.sm_umb_w   = pile{{"smalt",1},{"raw umber",1},{"lead white",2}}
P.violet     = pile{{"smalt",3},{"vermilion",0.5},{"lead white",3}}
local names = {"smalt_only","sm3co1w1","co_sm","deep","pgreen","gearth_w","blk_umb","prus_umb","sm_umb_w","violet"}
local x = 8
for i, n in ipairs(names) do
  local m = rect(x, 684, 46, 24)
  work(m, {hand="body", pile=P[n], coverage=3, angle=0.2, clip=true})
  x = x + 50
end
print("done")

--@ chunk 4
-- glaze / stipple / blend tests in the future-mound area (left, y 560-650)
local q1 = pile{{"cobalt blue",1},{"lead white",2}, medium=0.4}
local q2 = pile{{"lead white",6},{"chrome yellow",1},{"vermilion",0.5}, medium=0.3}
-- A: glaze of blue on the left 
work(rect(0,560,110,40), {hand="glaze", pile=q1, angle=0, coverage=1.5})
-- B: warm glaze on the right of it
work(rect(110,560,110,40), {hand="glaze", pile=q2, angle=0, coverage=1.5})
-- C: stipple blue into lower band
stipple(rect(0,606,110,40), {pile=q1, width=6, coverage=1.0})
-- D: stipple warm
stipple(rect(110,606,110,40), {pile=q2, width=6, coverage=1.0})
print("ok")

--@ chunk 5
blend(rect(0,556,230,50), {angle=0})
print("blended")

--@ chunk 6
print(drying(100, 580), drying(150, 625), drying(50, 668))

--@ chunk 7
local cl = pile{{"smalt",3},{"vermilion",0.5},{"lead white",3}}
local b = brush("flat", 8)
b:load(cl, 0.9)
b:stroke({{20, 578},{80, 576},{140, 579},{200, 577}}, {pressure={0.6,0.6}, ramps={0.1,0.3}})
b:stroke({{40, 592},{100, 590},{150, 592}}, {pressure={0.5,0.4}, ramps={0.1,0.4}})
print(b:fullness())

--@ chunk 8
-- ===== composition geometry (canvas units) =====
FAR  = {{0,481},{60,479},{130,478},{200,479},{260,477},{330,478},{400,476},{470,477},{540,475},{590,475},{625,473},{650,468},{680,470},{700,473},{760,475},{820,472},{880,470},{940,471},{1000,473}}
NEAR = {{0,548},{40,556},{90,570},{140,586},{190,602},{240,616},{290,628},{340,640},{400,654},{460,668},{530,682},{610,694},{700,703},{790,708},{860,700},{920,686},{970,672},{1000,662}}
FAR2 = {{770,472},{800,465},{850,459},{900,456},{950,459},{1000,463}}

-- oak: trunk + primary limbs, each {pts, widths}
OAK = {}
OAK.trunk = {pts={{206,666},{207,638},{204,602},{207,568},{210,543}}, w={86,62,48,43,42}}
OAK.A = {pts={{198,552},{166,540},{128,532},{94,516},{64,492},{36,472},{10,468},{-14,480}}, w={30,26,22,19,16,13,10,7}}
OAK.B = {pts={{209,544},{214,504},{206,468},{218,432},{212,398},{226,362},{219,326},{231,292},{224,258},{236,226},{231,196},{241,166},{239,138},{244,112}},
         w={36,33,30,27,24,21,18,15,12,10,8,6,4,2}}
OAK.C = {pts={{216,552},{256,534},{296,524},{338,505},{376,476},{406,444},{434,418},{462,400},{490,394},{514,398}}, w={30,27,24,21,18,15,12,9,7,5}}
OAK.D = {pts={{222,430},{258,400},{284,360},{304,320},{318,276},{338,240},{350,204},{364,174},{370,148}}, w={18,16,14,12,10,8,6,4,2}}
OAK.E = {pts={{214,470},{180,440},{156,398},{138,354},{124,312},{104,276},{90,236},{84,200},{78,170}}, w={20,18,16,13,11,9,7,5,3}}
OAK.F = {pts={{94,512},{74,474},{54,434},{36,396},{18,360},{6,324}}, w={14,12,10,8,6,4}}
OAK.G = {pts={{338,502},{348,460},{362,424},{380,386},{402,352},{420,320}}, w={16,13,10,8,6,4}}

-- edges of a ribbon, for drawing
function edges(pts, ws)
  local L, R = {}, {}
  for i = 1, #pts do
    local a = pts[math.max(1, i-1)]; local b = pts[math.min(#pts, i+1)]
    local tx, ty = b[1]-a[1], b[2]-a[2]
    local n = math.sqrt(tx*tx+ty*ty); tx, ty = tx/n, ty/n
    local w = ws[i]/2
    L[#L+1] = {pts[i][1] - ty*w, pts[i][2] + tx*w}
    R[#R+1] = {pts[i][1] + ty*w, pts[i][2] - tx*w}
  end
  return L, R
end

h = pencil("H")
h:line(FAR,  {pressure=0.28})
h:line(FAR2, {pressure=0.2})
h:line(NEAR, {pressure=0.3})
-- ruin and firs
h:line({{639,470},{639,457},{642,453},{645,456},{646,470}}, {pressure=0.3, smooth=false})
h:line({{651,470},{651,447},{658,431},{662,437},{664,434},{667,447},{667,470}}, {pressure=0.3, smooth=false})
h:line({{655,466},{655,452},{658,445},{661,452},{661,466}}, {pressure=0.3, smooth=false})
h:line({{671,471},{671,460},{674,457},{676,461},{679,459},{679,471}}, {pressure=0.3, smooth=false})
h:line({{694,474},{700,430},{706,474}}, {pressure=0.3, smooth=false})
h:line({{708,474},{714,440},{720,474}}, {pressure=0.3, smooth=false})
h:line({{721,474},{727,434},{733,474}}, {pressure=0.3, smooth=false})
h:line({{737,474},{742,450},{747,474}}, {pressure=0.3, smooth=false})
-- oak outlines
for k, t in pairs(OAK) do
  local L, R = edges(t.pts, t.w)
  h:sketch(L, {pressure=0.22, passes=2, wander=0.6})
  h:sketch(R, {pressure=0.22, passes=2, wander=0.6})
end
-- figure placeholder
h:line({{338,650},{334,650},{337,634},{340,622},{337,614},{340,610},{342,604},{345,602},{348,604},{350,610},{353,614},{351,622},{354,634},{357,650},{338,650}}, {pressure=0.3})
print("drawn")

--@ chunk 9
-- sky piles
SK = {}
SK.zen   = pile{{"smalt",3},{"cobalt blue",1},{"lead white",1.2}, medium=0.1}
SK.blue2 = pile{{"cobalt blue",1},{"lead white",2.2}, medium=0.1}
SK.teal  = pile{{"Prussian blue",0.3},{"lead white",4}, medium=0.1}
SK.lime  = pile{{"lead white",5},{"Prussian blue",0.2},{"chrome yellow",1}, medium=0.1}
SK.pale  = pile{{"lead white",6},{"chrome yellow",1}, medium=0.1}
SK.apri  = pile{{"lead white",6},{"chrome yellow",1},{"vermilion",0.5}, medium=0.1}
SK.glow  = pile{{"lead white",8},{"chrome yellow",0.6}, medium=0.1}

-- test: top band on the real canvas
work(rect(0, 0, 1000, 100), {hand="broad", pile=SK.zen, angle=0, coverage=1.6, seed=3})
print("top band laid")

--@ chunk 10
work(rect(0, 0, 1000, 100), {hand="broad", pile=SK.zen, angle=0, coverage=3, fill=true, seed=5})
blend(rect(0, 0, 1000, 100), {angle=0})
print("ok")

--@ chunk 11
print(drying(500,50), drying(100,580), drying(50,668))
print(wait(0))

--@ chunk 12
local bands = {
  {80, 120, SK.blue2},
  {190, 100, SK.teal},
  {275, 95, SK.lime},
  {345, 75, SK.pale},
  {395, 65, SK.apri},
  {440, 62, SK.glow},
}
for i, b in ipairs(bands) do
  work(rect(0, b[1], 1000, b[2]), {hand="broad", pile=b[3], angle=0, coverage=3, fill=true, seed=20+i})
end
print(wait(0))

--@ chunk 13
blend(rect(0, 0, 1000, 500), {angle=0})
print(wait(0))

--@ chunk 14
CL = {}
CL.body = pile{{"smalt",2},{"red earth",0.6},{"raw umber",0.3},{"lead white",3}}
CL.dark = pile{{"smalt",2},{"vermilion",0.4},{"raw umber",0.5},{"lead white",1.5}}
CL.rim  = pile{{"lead white",4},{"vermilion",1},{"chrome yellow",0.6}}
CL.pink = pile{{"lead white",4},{"vermilion",0.8},{"smalt",0.15}}

-- a cloud bar: pts = centerline, thick = brush width; laid with a flat brush, then a warm rim beneath
function cloudbar(pts, thick, body, rimpile, seed)
  math.randomseed(seed or 1)
  local b = brush("flat", thick)
  b:load(body, 0.85)
  b:stroke(pts, {pressure={0.55, 0.5}, ramps={0.12, 0.35}, swell={0.8,1.1,1.0,0.9,1.05,0.7}})
  if rimpile then
    local r = brush("flat", math.max(1.5, thick*0.45))
    r:load(rimpile, 0.8)
    local low = {}
    for i, p in ipairs(pts) do low[i] = {p[1] + 4, p[2] + thick*0.55} end
    r:stroke(low, {pressure={0.5, 0.45}, ramps={0.2, 0.4}})
  end
end

cloudbar({{330,456},{450,454},{560,457},{680,455},{800,457},{900,455}}, 5, CL.dark, CL.rim, 1)
cloudbar({{470,434},{580,432},{700,435},{820,433},{980,436}}, 6, CL.body, CL.rim, 2)
cloudbar({{140,418},{260,416},{380,419},{500,417},{640,420}}, 8, CL.body, CL.pink, 3)
cloudbar({{700,397},{800,395},{900,398},{1000,396}}, 6, CL.body, CL.pink, 4)
cloudbar({{0,464},{110,462},{230,465},{350,463},{440,465},{520,464}}, 9, CL.dark, CL.rim, 5)
print(wait(0))

--@ chunk 15
-- soft cloud banks, hand-drawn outlines
CL.veil = pile{{"smalt",2},{"red earth",0.6},{"raw umber",0.3},{"lead white",3}, medium=0.25}
local function bank(top, bot, seed)
  local pts = {}
  for _, p in ipairs(top) do pts[#pts+1] = {p[1], p[2]} end
  for i = #bot, 1, -1 do pts[#pts+1] = {bot[i][1], bot[i][2]} end
  return outline{pts=pts, closed=true, char="soft", seed=seed, amount=0.8, lobe=10}
end
A1 = bank({{520,459},{600,455},{700,452},{800,453},{900,456},{990,459}}, {{990,463},{900,466},{800,465},{700,466},{600,465},{520,462}}, 11)
work(A1:mask(), {hand="body", pile=CL.veil, angle=0, coverage=2.2, fill=true, seed=31})
print(wait(0))

--@ chunk 16
blend(A1:mask():grow(5), {angle=0})
print(wait(0))

--@ chunk 17
local function bank2(top, bot, seed, lobe)
  local pts = {}
  for _, p in ipairs(top) do pts[#pts+1] = {p[1], p[2]} end
  for i = #bot, 1, -1 do pts[#pts+1] = {bot[i][1], bot[i][2]} end
  return outline{pts=pts, closed=true, char="soft", seed=seed, amount=0.6, lobe=lobe or 7}
end
C1 = bank2({{0,461},{70,459},{150,457},{240,459},{330,462},{420,464},{500,465}}, {{500,469},{420,470},{330,470},{240,471},{150,471},{70,471},{0,471}}, 12, 7)
work(C1:mask(), {hand="body", pile=CL.veil, angle=0, coverage=2.2, fill=true, seed=32})
blend(C1:mask():grow(4), {angle=0})
print(wait(0))

--@ chunk 18
blend(ribbon({{130,418},{300,417},{500,418},{650,420}}, 16), {angle=0})
blend(ribbon({{690,397},{800,395},{900,397},{1000,396}}, 16), {angle=0})
blend(ribbon({{460,434},{600,432},{800,434},{990,436}}, 16), {angle=0})
print(wait(0))

--@ chunk 19
WA = {}
WA.mist = pile{{"lead white",8},{"chrome yellow",0.45},{"raw umber",0.05}, medium=0.1}
WA.apri = pile{{"lead white",6},{"chrome yellow",1},{"vermilion",0.35},{"smalt",0.1}, medium=0.1}
WA.pale = pile{{"lead white",6},{"chrome yellow",0.7},{"Prussian blue",0.05},{"smalt",0.2}, medium=0.1}
WA.teal = pile{{"lead white",4},{"Prussian blue",0.25},{"raw umber",0.1}, medium=0.1}
WA.blue = pile{{"cobalt blue",1},{"lead white",2},{"smalt",0.5},{"raw umber",0.1}, medium=0.1}
local wb = {
  {474, 32, WA.mist},
  {496, 50, WA.apri},
  {536, 56, WA.pale},
  {580, 62, WA.teal},
  {628, 90, WA.blue},
}
for i, b in ipairs(wb) do
  work(rect(0, b[1], 1000, b[2]), {hand="broad", pile=b[3], angle=0, coverage=3, fill=true, seed=40+i})
end
blend(rect(0, 470, 1000, 244), {angle=0})
print(wait(0))

--@ chunk 20
print(wait(24*60))
print(drying(500,50), drying(500,300), drying(500,600))

--@ chunk 21
SK.deepglaze = pile{{"smalt",3},{"Prussian blue",0.12},{"lead white",0.6}, medium=0.55}
local topfade = function(x, y)
  local t = clamp(1 - y/230, 0, 1)
  return 0.12 + 0.85 * t * t
end
work(rect(0, 0, 1000, 260), {hand="glaze", pile=SK.deepglaze, angle=0, coverage=2, load_at=topfade, seed=61})
print(wait(0))

--@ chunk 22
blend(rect(0, 0, 1000, 270), {angle=0})
print(wait(0))

--@ chunk 23
SK.seam = pile{{"lead white",5},{"Prussian blue",0.12},{"chrome yellow",0.7},{"smalt",0.15}, medium=0.15}
local seamcov = function(x, y)
  -- strongest just below the seam, fading upward into the blue
  return 1.3 * smoothstep(222, 296, y) * (1 - smoothstep(296, 330, y))
end
stipple(rect(0, 215, 1000, 125), {pile=SK.seam, width=6, coverage=seamcov, pressure={0.25,0.5}, seed=71})
print(wait(0))

--@ chunk 24
blend(rect(0, 205, 1000, 140), {angle=0})
print(wait(0))

--@ chunk 25
SK.h2   = pile{{"lead white",5},{"chrome yellow",1},{"vermilion",0.8}, medium=0.15}
SK.h1   = pile{{"lead white",8},{"chrome yellow",1.0}, medium=0.15}
SK.core = pile{{"lead white",10},{"chrome yellow",0.35},{"vermilion",0.04}, medium=0.1}

local function radial(cx, cy, rx, ry, peak, power)
  return function(x, y)
    local dx, dy = (x-cx)/rx, (y-cy)/ry
    local r = math.sqrt(dx*dx + dy*dy)
    if r >= 1 then return 0 end
    return peak * (1 - r)^power
  end
end

local m2 = ellipse(650, 466, 430, 50)
stipple(m2, {pile=SK.h2, width=7, coverage=radial(650,466,430,50,1.6,1.2), pressure={0.3,0.55}, seed=81})
blend(m2:grow(6), {angle=0})
local m1 = ellipse(650, 452, 270, 100)
stipple(m1, {pile=SK.h1, width=7, coverage=radial(650,452,270,100,1.5,1.3), pressure={0.3,0.55}, seed=82})
blend(m1:grow(6), {angle=0})
local m0 = ellipse(650, 460, 120, 42)
stipple(m0, {pile=SK.core, width=6, coverage=radial(650,460,120,42,1.7,1.0), pressure={0.3,0.55}, seed=83})
blend(m0:grow(4), {angle=0})
print(wait(0))

--@ chunk 26
function radial(cx, cy, rx, ry, peak, power)
  return function(x, y)
    local dx, dy = (x-cx)/rx, (y-cy)/ry
    local r = math.sqrt(dx*dx + dy*dy)
    if r >= 1 then return 0 end
    return peak * (1 - r)^power
  end
end
SK.glazeA = pile{{"lead white",3},{"chrome yellow",1},{"vermilion",0.8}, medium=0.6}
local gm = ellipse(650, 448, 520, 75)
work(gm, {hand="glaze", pile=SK.glazeA, angle=0, coverage=1.6, load_at=radial(650,455,520,75,1.0,0.9), seed=91})
blend(gm:grow(10), {angle=0})
print(wait(0))

--@ chunk 27
print(drying(650,450), drying(300,480), drying(500,100), drying(100,300))

--@ chunk 28
print(wait(20*60))
print(drying(650,450), drying(300,480), drying(500,100), drying(100,300))

--@ chunk 29
FS = {}
FS.mid  = pile{{"smalt",2},{"red earth",0.5},{"raw umber",0.5},{"lead white",1.6}, medium=0.15}
FS.dark = pile{{"smalt",2},{"raw umber",0.9},{"red earth",0.4},{"lead white",0.7}, medium=0.1}
FS.pale = pile{{"smalt",2},{"red earth",0.6},{"raw umber",0.3},{"lead white",3.2}, medium=0.15}

-- far shore, left stretch: hand-drawn edge (top) and a straight-ish base hidden later by mist
local function shore(top, base, seed, amount, lobe)
  local pts = {}
  for _, p in ipairs(top) do pts[#pts+1] = {p[1], p[2]} end
  for i = #base, 1, -1 do pts[#pts+1] = {base[i][1], base[i][2]} end
  return outline{pts=pts, closed=true, char="soft", seed=seed, amount=amount or 0.7, lobe=lobe or 9, edge=1.0}
end

SL = shore({{-10,477},{40,474},{90,476},{150,473},{210,475},{270,472},{330,474},{390,471},{450,473},{510,471},{560,470}},
           {{560,486},{450,486},{330,485},{210,486},{90,486},{-10,486}}, 101, 0.7, 8)
work(SL:mask(), {hand="body", pile=FS.mid, angle=0, coverage=2.6, fill=true, clip=true, seed=111})
print(wait(0))

--@ chunk 30
local m = SL:mask()
print(m:at(300, 473), m:at(300, 478), m:at(300, 481), m:at(300, 485), m:area())
print(SL:length())

--@ chunk 31
local m = SL:mask()
for _, x in ipairs({100, 300}) do
  local s = ""
  for y = 466, 490 do s = s .. string.format("%d:%.2f ", y, m:at(x, y)) end
  print(x, s)
end

--@ chunk 32
SLo = outline{{-10,477},{40,474},{90,476},{150,473},{210,475},{270,472},{330,474},{390,471},{450,473},{510,471},{560,470},{620,470}, char="soft", seed=101, amount=0.7, lobe=8}
local m = SLo:below(487)
for _, x in ipairs({100, 300}) do
  local s = ""
  for y = 466, 490 do s = s .. string.format("%d:%.2f ", y, m:at(x, y)) end
  print(x, s)
end
print(m:area())

--@ chunk 33
local p = SLo:path(1)
print(type(p), #p)
for i = 1, 6 do print(i, p[i][1], p[i][2]) end
local ps = SLo:paths()
print(#ps)
local st = SLo:strokes()
print(#st)
print(SLo:length())

--@ chunk 34
function under(o, base)
  local p = o:path(1)
  local pts = {}
  for i = 1, #p do pts[i] = {p[i][1], p[i][2]} end
  table.sort(pts, function(a, b) return a[1] < b[1] end)
  local baseFn = type(base) == "function" and base or function(x) return base end
  return below(pts) * above(baseFn)
end
local m = under(SLo, 487)
for _, x in ipairs({100, 300}) do
  local s = ""
  for y = 466, 490 do s = s .. string.format("%d:%.2f ", y, m:at(x, y)) end
  print(x, s)
end
print(m:area())

--@ chunk 35
SH = outline{{-10,477},{40,474},{90,476},{150,473},{210,475},{270,472},{330,474},{390,471},{450,473},{510,471},{560,469},{610,467},{650,466},{700,466},{750,467},{800,469},{850,471},{900,470},{950,472},{1010,473},
                char="soft", seed=102, amount=0.6, lobe=9}
SHm = under(SH, 488)
work(SHm, {hand="body", pile=FS.mid, angle=0, coverage=2.8, fill=true, clip=true, seed=112})
work(under(SH, 482), {hand="body", pile=FS.dark, angle=0, coverage=1.6, fill=true, clip=true, seed=113})
print(wait(0))

--@ chunk 36
local p = SH:path(1)
local pts = {}
for i = 1, #p, 6 do pts[#pts+1] = {p[i][1], p[i][2]} end
table.sort(pts, function(a, b) return a[1] < b[1] end)
local edgeband = ribbon(pts, 9)
blend(edgeband, {angle=0})
print(wait(0))

--@ chunk 37
WA.haze = pile{{"lead white",8},{"chrome yellow",0.5},{"vermilion",0.12}, medium=0.15}
local hazecov = function(x, y)
  local t = smoothstep(474, 488, y) * (1 - smoothstep(488, 494, y))
  return 2.2 * t
end
stipple(rect(0, 470, 1000, 28), {pile=WA.haze, width=6, coverage=hazecov, pressure={0.35,0.6}, seed=121})
blend(rect(0, 476, 1000, 20), {angle=0})
print(wait(0))

--@ chunk 38
WA.mist2 = pile{{"lead white",8},{"chrome yellow",0.55},{"vermilion",0.15},{"smalt",0.05}, medium=0.05}
local function depth_load(y0, y1, lo, hi)
  return function(x, y) return lerp(lo, hi, smoothstep(y0, y1, y)) end
end
-- opaque-ish mist over the lower part of the far-shore band, thinning upward
work(rect(0, 484, 1000, 12), {hand="body", pile=WA.mist2, angle=0, coverage=3.5, fill=true, load_at=depth_load(484, 490, 0.75, 0.95), seed=131, tool="filbert 6"})
work(rect(0, 479, 1000, 9),  {hand="body", pile=WA.mist2, angle=0, coverage=1.6, fill=true, load_at=depth_load(479, 486, 0.25, 0.6), seed=132, tool="filbert 5"})
blend(rect(0, 476, 1000, 22), {angle=0})
print(wait(0))

--@ chunk 39
blend(rect(0, 484, 1000, 34), {angle=0})
print(wait(0))

--@ chunk 40
FS.fir  = pile{{"smalt",2},{"raw umber",1.1},{"Prussian blue",0.25},{"lead white",0.5}, medium=0.05}
FS.ruin = pile{{"smalt",2},{"raw umber",1.0},{"red earth",0.5},{"lead white",0.8}, medium=0.05}

-- irregular tree crowns along the far shore: stippled clumps rising from the dark band
local tn = noise{seed=5, octaves=3, period=55, persistence=0.55}
local function crownTop(x) return 476 - (1.5 + 8.5 * smoothstep(0.38, 0.85, tn:at01(x, 0))) end
local cc = function(x, y)
  local top = crownTop(x)
  return 2.2 * smoothstep(top - 1, top + 3.5, y) * (1 - smoothstep(478, 481, y))
end
stipple(rect(0, 462, 1000, 22), {pile=FS.mid, width=3, coverage=cc, pressure={0.4,0.7}, seed=141, dips={12, 0.8, 0.5}})
print(wait(0))

--@ chunk 41
SK.veil = pile{{"lead white",6},{"vermilion",0.3},{"yellow ochre",0.35},{"smalt",0.1}, medium=0.7}
local vcov = function(x, y)
  return smoothstep(262, 318, y) * (1 - smoothstep(372, 420, y))
end
work(rect(0, 255, 1000, 175), {hand="glaze", pile=SK.veil, angle=0, coverage=1.8, load_at=vcov, seed=201})
blend(rect(0, 255, 1000, 175), {angle=0})
print(wait(0))

--@ chunk 42
SK.core2 = pile{{"lead white",10},{"chrome yellow",0.3},{"vermilion",0.03}, medium=0.45}
local cm = ellipse(650, 456, 230, 60)
work(cm, {hand="glaze", pile=SK.core2, angle=0, coverage=2.2, load_at=radial(650,458,230,60,1.0,1.1), seed=211})
blend(cm:grow(10), {angle=0})
print(wait(0))

--@ chunk 43
print(wait(22*60))
print(drying(600,480), drying(650,450), drying(300,300))

--@ chunk 44
print(wait(12*60))
print(drying(600,480), drying(650,450), drying(300,300), drying(800,470))

--@ chunk 45
-- distant pale hills on the right, drawn as a soft-edged headland
FS.hill = pile{{"smalt",2},{"red earth",0.45},{"raw umber",0.2},{"lead white",4.2}, medium=0.25}
local hpts = {{690,478},{720,470},{752,466},{790,462},{830,457},{870,453},{905,451},{940,454},{975,458},{1010,461},{1010,482},{690,482}}
local hm = poly(hpts, true)
work(hm, {hand="body", pile=FS.hill, angle=0, coverage=2.2, fill=true, clip=true, seed=301})
-- a second, fainter ridge behind
local hpts2 = {{560,472},{600,468},{640,465},{690,463},{740,460},{780,458},{780,480},{560,480}}
work(poly(hpts2, true), {hand="body", pile=FS.hill, angle=0, coverage=1.3, fill=true, clip=true, seed=302})
print(wait(0))

--@ chunk 46
FS.shore = pile{{"smalt",2},{"raw umber",1.0},{"red earth",0.35},{"lead white",0.9}, medium=0.05}

local n1 = noise{seed=17, octaves=3, period=90, persistence=0.5}
local n2 = noise{seed=23, octaves=3, period=17, persistence=0.5}
-- tree crowns and copses, each its own profile: {x, height, width, kind}
local feats = {
  {58, 9, 15, "round"}, {74, 13, 12, "round"}, {88, 7, 10, "round"},
  {140, 11, 13, "round"}, {156, 16, 9, "pop"}, {172, 8, 12, "round"},
  {262, 6, 22, "round"}, {296, 21, 6, "pop"}, {306, 9, 12, "round"},
  {372, 8, 14, "round"}, {388, 12, 12, "round"}, {405, 15, 8, "pop"}, {420, 7, 14, "round"},
  {470, 7, 16, "round"}, {520, 9, 12, "round"}, {534, 5, 12, "round"},
  {585, 8, 16, "round"}, {603, 5, 10, "round"},
  {770, 5, 20, "round"}, {828, 6, 22, "round"}, {880, 4, 18, "round"}, {940, 5, 20, "round"},
}
local function feat_top(x, base)
  local top = base
  for i, f in ipairs(feats) do
    local dx = (x - f[1]) / (f[3] / 2)
    if math.abs(dx) < 1 then
      local prof
      if f[4] == "pop" then prof = (1 - math.abs(dx)) ^ 0.75
      else prof = math.sqrt(1 - dx*dx) ^ 0.9 end
      local jag = 0.9 * n2(x * 1.5, i * 13.0)
      local y = base - f[2] * prof + jag
      if y < top then top = y end
    end
  end
  return top
end
local function shore_top(x)
  local base = 476.5 - 1.4 * n1(x, 3) - 0.5 * n2(x, 9)
  if x > 700 then base = base + 0.6 end
  return feat_top(x, base)
end
local pts = {}
for x = -6, 1006, 1.5 do pts[#pts+1] = {x, shore_top(x)} end
local poly_pts = {}
for _, p in ipairs(pts) do poly_pts[#poly_pts+1] = p end
poly_pts[#poly_pts+1] = {1006, 490}
poly_pts[#poly_pts+1] = {-6, 490}
SHORE2 = poly(poly_pts)
work(SHORE2, {hand="body", pile=FS.shore, angle=0, coverage=3, fill=true, clip=true, seed=311})
print(wait(0))

--@ chunk 47
-- rough up the far shore: vertical tufts along the top edge, heavier near copses
local nA = noise{seed=41, octaves=3, period=40, persistence=0.6}
local nB = noise{seed=42, octaves=2, period=9, persistence=0.5}
local function tuft_cov(x, y)
  local top = 476.5 - 2.0 * (nA(x, 0) + 0.6) - 1.2 * nB(x, 5)
  local d = y - top
  if d < -3 then return 0 end
  local up = smoothstep(-3, 1.5, d)          -- fades in going down through the edge
  local dn = 1 - smoothstep(4, 9, d)         -- and out below
  return 2.6 * up * dn
end
stipple(rect(0, 468, 1000, 26), {pile=FS.shore, width=2.2, coverage=tuft_cov, pressure={0.35,0.7}, drag={2.6, -1.5708}, cluster=0.5, seed=321, dips={14, 0.8, 0.5}})
print(wait(0))

--@ chunk 48
print(wait(16*60))
print(drying(300,482), drying(700,482))

--@ chunk 49
print(wait(10*60))
print(drying(300,482), drying(700,482))

--@ chunk 50
print(wait(24*60))
print(drying(300,482), drying(700,482))

--@ chunk 51
print(wait(3*24*60))
print(drying(300,482), drying(700,482))

--@ chunk 52
FS.ruin = pile{{"smalt",2},{"raw umber",1.3},{"red earth",0.4},{"lead white",0.5}, medium=0.05}

-- ruined gable with a lancet window; the window is cut out so the glow shows through
RUIN = poly({
  {634,478},{634,471},{637,469},{640,470},{642,467},{646,467},{647,464},{650,463},{650,460},{652,459},
  {653,457},{655.5,458},{657,455.5},{658.5,457},{660,456.5},{661.5,459},{663,460},{664,463},{664,467},
  {667,466},{669,469},{672,468},{674,471},{678,472},{680,478}})
LANCET = poly({{654.3,472.5},{654.3,467},{654.9,463.6},{656.6,460.6},{658.3,463.4},{659,467},{659,472.5}})
work(RUIN - LANCET, {hand="detail", pile=FS.ruin, angle=1.4, coverage=4, fill=true, clip=true, seed=401})

-- a small stand of firs: each drawn on its own
FIR1 = poly({{696,477},{696.5,470},{695,469},{696.3,466},{695.2,465},{696.6,462},{696,460.5},{697.4,459},{697.6,456},{698.4,459.5},{699.2,461},{698.8,463},{700.6,464.5},{699.6,466.5},{701.6,468},{700.4,470},{702.6,472},{701.4,474},{703,477}})
FIR2 = poly({{704.5,477},{705.5,471},{704,470.5},{705.6,468},{704.6,466.5},{706,464},{705.2,462.5},{706.4,460.5},{706,457.5},{707.2,454.5},{707.9,451.5},{708.6,455},{709.2,457.5},{710.6,460},{709.8,462},{711.8,464},{710.8,466},{713,468.5},{711.8,470.5},{714.2,473.5},{712.6,475},{714,477}})
FIR3 = poly({{717,477},{717.6,472},{716.2,471},{717.8,468.5},{717,467},{718.4,464.5},{718,462},{719.4,459.5},{719.6,456.5},{720.6,455},{721.4,458},{722.8,460},{722.4,462.5},{724,464},{723,466.5},{725,468.5},{724,471},{726,473.5},{725,477}})
FIR4 = poly({{731,477},{731.6,472.5},{730.4,471.5},{731.8,469},{731.6,466.5},{733,464.5},{733.6,464},{734.6,466},{735.4,468},{734.8,470},{736.6,472},{736.2,474},{737.6,477}})
for i, f in ipairs({FIR1, FIR2, FIR3, FIR4}) do
  work(f, {hand="detail", pile=FS.ruin, angle=1.5, coverage=4, fill=true, clip=true, seed=410+i})
end
print(wait(0))

--@ chunk 53
-- Mist at the foot of the far shore, and its dark reflection in the still water
WA.mist3 = pile{{"lead white",8},{"chrome yellow",0.5},{"vermilion",0.14},{"smalt",0.06}, medium=0.12}
work(rect(0, 481, 1000, 14), {hand="body", tool="filbert 5", pile=WA.mist3, angle=0, coverage=2.4, fill=true,
     load_at=function(x, y) return 0.9 * smoothstep(481, 491, y) end, seed=501})

-- reflection of the shore: broken horizontal dashes just under the mist, paler and greyer than the shore itself
WA.refl = pile{{"smalt",2},{"raw umber",0.8},{"red earth",0.4},{"lead white",3.4}, medium=0.3}
math.randomseed(77)
local rb = brush("flat", 2.2)
for i = 1, 210 do
  local y = 492.5 + rand(0, 9) ^ 1.0
  local x = rand(-10, 1000)
  local len = rand(8, 46)
  rb:load(WA.refl, rand(0.35, 0.7))
  rb:stroke({{x, y}, {x + len * 0.5, y + rand(-0.25, 0.25)}, {x + len, y + rand(-0.25, 0.25)}}, {pressure={0.35, 0.3}, ramps={0.3, 0.5}})
end
print(wait(0))

--@ chunk 54
blend(rect(0, 482, 1000, 28), {angle=0})
print(wait(0))

--@ chunk 55
FG = {}
FG.base  = pile{{"raw umber",3},{"bone black",2},{"Prussian blue",0.5},{"green earth",1}}
FG.warm  = pile{{"raw umber",3},{"bone black",1.2},{"red earth",0.6},{"Prussian blue",0.25}}
FG.olive = pile{{"raw umber",2},{"green earth",1.5},{"yellow ochre",0.7},{"bone black",1}}

-- shoreline of the bank, drawn as points with a little hand-wobble (its own noise, not a copy of anything)
local wob = noise{seed=61, octaves=3, period=60, persistence=0.5}
local edge_pts = {{-10,538},{20,537},{50,541},{85,549},{120,558},{155,566},{190,572},{225,578},{258,588},{285,600},{310,608},{335,611},{360,609},{380,614},{400,626},{425,640},{455,652},{490,662},{530,672},{575,682},{625,691},{680,699},{740,704},{800,707},{850,704},{895,694},{935,680},{970,663},{1010,648}}
local ep = {}
for i, p in ipairs(edge_pts) do ep[i] = {p[1], p[2] + 2.2 * wob(p[1], 7)} end
BANK_EDGE = ep
BANK = below(ep)
BANKm = BANK   -- 'below' = under the curve (larger y)
print(BANK:at(100, 700), BANK:at(100, 500), BANK:at(700, 706), BANK:at(700, 690))

--@ chunk 56
work(BANK, {hand="body", pile=FG.base, angle=0.15, coverage=3.6, fill=true, clip=true, seed=601, length={30,70}})
print(wait(0))

--@ chunk 57
print(drying(100,650), drying(300,690))

--@ chunk 58
local d = BANK:distance()
print(d:at(100, 700), d:at(100, 560), d:at(100, 540), d:at(100, 500), d:at(400, 700))
local r = BANK:rim(24, 16)
print(r:at(100, 552), r:at(100, 570), r:at(100, 600), r:at(100, 700))
local bd = BANK:band(0, 30, 12)
print(bd:at(100, 552), bd:at(100, 580), bd:at(100, 640), bd:at(100, 700))

--@ chunk 59
FG.lit  = pile{{"raw umber",2},{"yellow ochre",1.3},{"green earth",1.2},{"lead white",0.6},{"smalt",0.3}}
FG.deep = pile{{"raw umber",3},{"bone black",1.5},{"Prussian blue",0.4},{"red earth",0.3}}
local d = BANK:distance()
LITZ  = d:map(function(v) return smoothstep(-1.5, 2, v) * (1 - smoothstep(6, 30, v)) end)
DEEPZ = d:map(function(v) return smoothstep(45, 110, v) end)
local ang = function(x, y) return 0.3 - 0.7 * smoothstep(430, 980, x) end
work(LITZ, {hand="body", pile=FG.lit, angle=ang, coverage=1.6, fill=true, clip=BANK, seed=611, length={25,60}, load_at=function(x,y) return 0.5 + 0.3*math.sin(x/37) end})
work(DEEPZ, {hand="body", pile=FG.deep, angle=ang, coverage=1.6, fill=true, clip=BANK, seed=612, length={30,80}})
print(wait(0))

--@ chunk 60
blend(BANK, {angle=1.2})
blend(BANK, {angle=0.25})
print(wait(0))

--@ chunk 61
-- ===== oak skeleton and silhouette machinery =====
function crspline(pts, step)
  local n = #pts
  local out, idx = {}, {}
  local function P(i) return pts[math.max(1, math.min(n, i))] end
  for i = 1, n - 1 do
    local p0, p1, p2, p3 = P(i-1), P(i), P(i+1), P(i+2)
    local dx, dy = p2[1]-p1[1], p2[2]-p1[2]
    local k = math.max(2, math.ceil(math.sqrt(dx*dx+dy*dy) / step))
    for j = 0, k - 1 do
      local t = j / k
      local t2, t3 = t*t, t*t*t
      local x = 0.5*((2*p1[1]) + (-p0[1]+p2[1])*t + (2*p0[1]-5*p1[1]+4*p2[1]-p3[1])*t2 + (-p0[1]+3*p1[1]-3*p2[1]+p3[1])*t3)
      local y = 0.5*((2*p1[2]) + (-p0[2]+p2[2])*t + (2*p0[2]-5*p1[2]+4*p2[2]-p3[2])*t2 + (-p0[2]+3*p1[2]-3*p2[2]+p3[2])*t3)
      out[#out+1] = {x, y}; idx[#idx+1] = i + t
    end
  end
  out[#out+1] = {pts[n][1], pts[n][2]}; idx[#idx+1] = n
  return out, idx
end

local function widthAt(ws, s)
  local i = math.max(1, math.min(#ws - 1, math.floor(s)))
  local f = s - i
  f = f*f*(3 - 2*f)
  return ws[i] * (1 - f) + ws[i+1] * f
end

BARK = noise{seed=71, octaves=3, period=26, persistence=0.55}
BURL = noise{seed=72, octaves=2, period=90, persistence=0.5}

-- returns polygon points of a limb outline; rough = bark irregularity in units at width 40
function limb_outline(pts, ws, rough, seedoff)
  local d, idx = crspline(pts, 3)
  local L, R = {}, {}
  for i = 1, #d do
    local a, b = d[math.max(1, i-1)], d[math.min(#d, i+1)]
    local tx, ty = b[1]-a[1], b[2]-a[2]
    local n = math.sqrt(tx*tx + ty*ty); if n < 1e-6 then n = 1 end
    tx, ty = tx/n, ty/n
    local w = widthAt(ws, idx[i]) / 2
    local amp = rough * math.min(1, w / 12)
    local wl = w + amp * BARK(d[i][1] + seedoff, d[i][2]) + 0.5 * amp * BURL(d[i][1], d[i][2] + seedoff)
    local wr = w + amp * BARK(d[i][1], d[i][2] + seedoff + 50) + 0.5 * amp * BURL(d[i][1] + 30, d[i][2] + seedoff)
    L[#L+1] = {d[i][1] - ty*wl, d[i][2] + tx*wl}
    R[#R+1] = {d[i][1] + ty*wr, d[i][2] - tx*wr}
  end
  local poly_pts = {}
  for i = 1, #L do poly_pts[#poly_pts+1] = L[i] end
  -- rounded cap at the tip
  local e = d[#d]
  local a = d[#d-1]
  local tx, ty = e[1]-a[1], e[2]-a[2]
  local n = math.sqrt(tx*tx+ty*ty); tx, ty = tx/n, ty/n
  local wt = widthAt(ws, #ws) / 2
  poly_pts[#poly_pts+1] = {e[1] + tx*wt*0.9, e[2] + ty*wt*0.9}
  for i = #R, 1, -1 do poly_pts[#poly_pts+1] = R[i] end
  return poly_pts
end

OAK = {
  trunk = {pts={{204,668},{198,640},{200,606},{206,572},{212,540},{216,510}}, ws={100,76,58,50,46,44}},
  B = {pts={{216,512},{220,472},{226,434},{223,396},{234,358},{245,322},{240,286},{251,252},{257,220},{253,192},{259,168}}, ws={44,40,37,33,30,26,22,18,14,10,4}},
  A = {pts={{202,560},{168,551},{132,544},{98,529},{66,507},{36,485},{6,471},{-22,468}}, ws={40,32,27,23,18,14,11,8}},
  E = {pts={{219,482},{190,457},{164,425},{153,387},{133,353},{125,317},{104,289},{96,259},{88,232}}, ws={28,23,20,16,13,10,8,6,3}},
  C = {pts={{224,516},{262,503},{300,487},{338,477},{374,461},{406,437},{432,409},{455,381},{481,361},{507,347}}, ws={38,31,27,24,21,18,15,12,9,5}},
  D = {pts={{232,430},{262,404},{290,377},{306,343},{336,323},{352,293},{376,273},{392,245}}, ws={24,19,16,14,11,9,7,3}},
  H = {pts={{304,489},{330,495},{354,507},{372,527},{382,549},{385,569}}, ws={18,14,12,9,6,2.5}},
  F = {pts={{100,532},{86,501},{66,471},{48,437},{30,401},{14,367},{4,341}}, ws={16,12,10,8,6,4,2}},
  G = {pts={{376,463},{385,429},{377,395},{393,361},{389,327},{404,297}}, ws={14,10,8,6,4,2}},
}
OAKPOLY = {}
local order = {"trunk","B","A","E","C","D","H","F","G"}
OAKMASK = nil
for i, k in ipairs(order) do
  local t = OAK[k]
  local pp = limb_outline(t.pts, t.ws, 7, i * 17)
  OAKPOLY[k] = pp
  local m = poly(pp)
  OAKMASK = OAKMASK and (OAKMASK + m) or m
end
-- ascii preview
local rows = {}
for y = 90, 690, 10 do
  local s = ""
  for x = 0, 560, 6 do
    local v = OAKMASK:at(x, y)
    s = s .. (v > 0.5 and "#" or ".")
  end
  rows[#rows+1] = string.format("%3d ", y) .. s
end
print(table.concat(rows, "\n"))

--@ chunk 62
WA.col = pile{{"lead white",8},{"chrome yellow",0.7},{"vermilion",0.12}, medium=0.35}
local function colw(y) return 34 + (y - 496) * 0.62 end
local colload = function(x, y)
  if y < 497 then return 0 end
  local w = colw(y)
  local u = (x - 652) / w
  local a = math.exp(-u*u*1.3)
  local fadeTop = smoothstep(497, 530, y)
  return a * (0.35 + 0.65 * fadeTop)
end
local wm = rect(300, 496, 700, 210) - BANK
work(wm, {hand="glaze", pile=WA.col, angle=0, coverage=2.2, load_at=colload, seed=701})
blend(wm, {angle=0})
print(wait(0))

--@ chunk 63
print(wait(3*24*60))
print(drying(100,650), drying(300,690), drying(650,600), drying(400,650))

--@ chunk 64
local wob2 = noise{seed=63, octaves=3, period=55, persistence=0.5}
local e2 = {{-10,538},{20,537},{50,541},{85,549},{120,558},{155,566},{190,572},{225,578},{250,586},{275,597},{300,604},{325,606},{350,604},{372,608},{395,620},{420,635},{452,648},{490,658},{530,668},{575,677},{625,686},{680,694},{740,700},{800,703},{850,702},{895,696},{930,684},{955,664},{975,640},{995,626},{1010,622}}
BANK2_EDGE = {}
for i, p in ipairs(e2) do BANK2_EDGE[i] = {p[1], p[2] + 1.8 * wob2(p[1], 3)} end
BANK2 = below(BANK2_EDGE)
work(BANK2, {hand="body", pile=FG.base, angle=0.15, coverage=4, fill=true, clip=BANK2, seed=801, length={30,70}})
print(wait(0))
print(BANK2:at(500, 690), BANK2:at(500, 640))

--@ chunk 65
WA.mist4 = pile{{"lead white",8},{"chrome yellow",0.45},{"vermilion",0.14},{"smalt",0.07}, medium=0.4}
local mistload = function(x, y)
  -- soft mist around the shore's foot, strongest just below the trees
  return smoothstep(478.5, 484, y) * (1 - 0.55 * smoothstep(488, 497, y))
end
local mm = rect(0, 477, 1000, 22)
work(mm, {hand="glaze", pile=WA.mist4, angle=0, coverage=2.6, load_at=mistload, seed=901})
blend(mm, {angle=0})
print(wait(0))

--@ chunk 66
function lens(x0, x1, y, th, seed, sag)
  -- a long thin cloud stratum; returns an outline whose mask is a lens with hand-wobbled edges
  local top, bot = {}, {}
  local n = 9
  for i = 0, n do
    local t = i / n
    local x = lerp(x0, x1, t)
    local prof = math.sin(math.pi * t) ^ 0.7
    local yy = y + (sag or 0) * (t - 0.5)
    top[#top+1] = {x, yy - th * 0.5 * prof * (0.75 + 0.5 * math.abs(math.sin(i * 2.3 + seed)))}
    bot[#bot+1] = {x, yy + th * 0.5 * prof * (0.75 + 0.5 * math.abs(math.cos(i * 1.7 + seed)))}
  end
  local pts = {}
  for _, p in ipairs(top) do pts[#pts+1] = p end
  for i = #bot, 1, -1 do pts[#pts+1] = bot[i] end
  return outline{pts=pts, closed=true, char="soft", seed=seed, amount=0.5, lobe=11}
end

CL.veil2 = pile{{"smalt",2},{"red earth",0.7},{"raw umber",0.25},{"lead white",3.2}, medium=0.3}
CL.rim2  = pile{{"lead white",4},{"vermilion",0.9},{"chrome yellow",0.7}, medium=0.3}

local S1 = lens(330, 1010, 352, 7, 21, 4)
local S2 = lens(520, 900, 318, 5, 22, -3)
local S3 = lens(0, 330, 372, 6, 23, 3)
for i, S in ipairs({S1, S2, S3}) do
  local m = S:mask()
  work(m, {hand="body", tool="filbert 4", pile=CL.veil2, angle=0, coverage=2.2, fill=true, clip=m, seed=930+i})
end
print(wait(0))

--@ chunk 67
blend(ribbon({{330,354},{500,352},{700,352},{900,354},{1010,356}}, 24), {angle=0})
blend(ribbon({{520,319},{700,318},{900,318}}, 22), {angle=0})
blend(ribbon({{0,372},{150,372},{330,373}}, 22), {angle=0})
blend(ribbon({{330,354},{500,352},{700,352},{900,354},{1010,356}}, 30), {angle=0})
print(wait(0))

--@ chunk 68
print(wait(2*24*60))
print(drying(500,490), drying(700,352), drying(300,372), drying(650,600))

--@ chunk 69
SK.veil3 = pile{{"lead white",7},{"chrome yellow",0.55},{"vermilion",0.12},{"yellow ochre",0.2}, medium=0.55}
SK.veil3g = pile{{"lead white",7},{"chrome yellow",0.6},{"Prussian blue",0.04},{"smalt",0.06}, medium=0.55}
-- veil the upper (yellow-green) strip and the middle/left ones with the local tone
local function bandload(y0, y1, pad)
  return function(x, y)
    return smoothstep(y0 - pad, y0, y) * (1 - smoothstep(y1, y1 + pad, y))
  end
end
local r1 = rect(480, 296, 560, 46)
work(r1, {hand="glaze", pile=SK.veil3g, angle=0, coverage=1.8, load_at=bandload(305, 331, 10), seed=951})
local r2 = rect(300, 330, 710, 46)
work(r2, {hand="glaze", pile=SK.veil3, angle=0, coverage=1.8, load_at=bandload(338, 368, 10), seed=952})
local r3 = rect(0, 352, 360, 40)
work(r3, {hand="glaze", pile=SK.veil3, angle=0, coverage=1.8, load_at=bandload(361, 384, 10), seed=953})
print(wait(0))

--@ chunk 70
blend(rect(480, 298, 560, 42), {angle=0})
blend(rect(300, 332, 710, 42), {angle=0})
blend(rect(0, 354, 360, 36), {angle=0})
print(wait(0))

--@ chunk 71
FS.shore2 = pile{{"smalt",2},{"raw umber",0.9},{"red earth",0.4},{"lead white",1.4}}
FS.shore3 = pile{{"smalt",1},{"red earth",0.45},{"raw umber",0.1},{"lead white",4.5}, medium=0.1}

local nT1 = noise{seed=201, octaves=3, period=70, persistence=0.55}
local nT2 = noise{seed=202, octaves=3, period=13, persistence=0.6}
local nT3 = noise{seed=203, octaves=2, period=4.0, persistence=0.5}
local function h_tree(x)
  local c = smoothstep(-0.2, 0.6, nT1(x, 0))
  local h = 0.5 + 6.0 * c + 2.2 * nT2(x, 0) * (0.35 + c) + 0.9 * nT3(x, 0)
  return math.max(0.3, h)
end
local pts = {}
for x = -6, 1006, 1.25 do pts[#pts+1] = {x, 479.5 - h_tree(x)} end
local ring = {}
for _, p in ipairs(pts) do ring[#ring+1] = p end
ring[#ring+1] = {1006, 485}
ring[#ring+1] = {-6, 485}
SH3 = poly(ring)

work(SH3, {hand="body", tool="filbert 4", pile=FS.shore2, angle=0, coverage=3.2, fill=true, clip=true, seed=1101})
work(SH3, {hand="body", tool="filbert 4", pile=FS.shore3, angle=0, coverage=2.2, fill=true, clip=true, seed=1102,
           load_at=function(x, y) return smoothstep(474.5, 483, y) end})
blend(SH3, {angle=0})
print(wait(0))

--@ chunk 72
print(wait(2*24*60))
print(drying(500,480), drying(300,482), drying(700,352))

--@ chunk 73
print(wait(36*60))
print(drying(500,480), drying(300,482), drying(700,352), drying(650,600))

--@ chunk 74
-- ===== far shore, repainted as one opaque band =====
FS.band  = pile{{"raw umber",2},{"smalt",1.6},{"red earth",0.5},{"bone black",0.5},{"lead white",1.1}}
FS.bandL = pile{{"raw umber",1.2},{"smalt",1.6},{"red earth",0.5},{"lead white",2.2}}

local nT1 = noise{seed=301, octaves=3, period=80, persistence=0.55}
local nT2 = noise{seed=302, octaves=3, period=11, persistence=0.6}
local nT3 = noise{seed=303, octaves=2, period=3.5, persistence=0.5}
-- crowns: {x, height, half-width}; poplars: narrow, tall
local crowns = {
  {36,7,14},{62,10,13},{86,6,11},{118,4,12},
  {146,9,12},{171,7,11},{206,5,16},
  {250,6,16},{281,10,12},{306,7,11},{338,4,14},
  {366,8,12},{389,11,13},{420,6,14},{450,5,14},
  {482,7,15},{514,9,12},{538,6,12},{568,5,16},{600,7,12},
  {772,4,16},{812,5,18},{858,3.5,16},{905,4.5,18},{950,3.5,16},{985,3,14},
}
local poplars = {{158,21,2.6},{290,24,2.4},{402,26,2.7},{526,17,2.3},{93,15,2.2}}
local function top_at(x)
  local base = 480.2 - 0.8 * nT1(x, 0)
  local top = base
  for _, c in ipairs(crowns) do
    local u = (x - c[1]) / c[3]
    if math.abs(u) < 1 then
      local prof = (1 - u*u) ^ 0.55
      local y = base - c[2] * prof - 1.1 * nT2(x, c[1])
      if y < top then top = y end
    end
  end
  for _, p in ipairs(poplars) do
    local u = (x - p[1]) / p[3]
    if math.abs(u) < 1 then
      local prof = (1 - math.abs(u)) ^ 0.8
      local y = base - p[2] * prof
      if y < top then top = y end
    end
  end
  return top - 0.5 * nT3(x, 5) - 0.4
end
local ring = {}
for x = -6, 1006, 1.0 do ring[#ring+1] = {x, top_at(x)} end
ring[#ring+1] = {1006, 487.5}
ring[#ring+1] = {-6, 487.5}
BAND = poly(ring)
work(BAND, {hand="body", tool="filbert 4", pile=FS.band, angle=0, coverage=4, fill=true, clip=true, seed=1201})
print(wait(0))

--@ chunk 75
nT1 = noise{seed=301, octaves=3, period=80, persistence=0.55}
nT2 = noise{seed=302, octaves=3, period=11, persistence=0.6}
nT3 = noise{seed=303, octaves=2, period=3.5, persistence=0.5}
CROWNS = {
  {36,7,14},{62,10,13},{86,6,11},{118,4,12},
  {146,9,12},{171,7,11},{206,5,16},
  {250,6,16},{281,10,12},{306,7,11},{338,4,14},
  {366,8,12},{389,11,13},{420,6,14},{450,5,14},
  {482,7,15},{514,9,12},{538,6,12},{568,5,16},{600,7,12},
  {772,4,16},{812,5,18},{858,3.5,16},{905,4.5,18},{950,3.5,16},{985,3,14},
}
POPLARS = {{158,21,2.6},{290,24,2.4},{402,26,2.7},{526,17,2.3},{93,15,2.2}}
function TOPAT(x)
  local base = 480.2 - 0.8 * nT1(x, 0)
  local top = base
  for _, c in ipairs(CROWNS) do
    local u = (x - c[1]) / c[3]
    if math.abs(u) < 1 then
      local prof = (1 - u*u) ^ 0.55
      local y = base - c[2] * prof - 1.1 * nT2(x, c[1])
      if y < top then top = y end
    end
  end
  for _, p in ipairs(POPLARS) do
    local u = (x - p[1]) / p[3]
    if math.abs(u) < 1 then
      local prof = (1 - math.abs(u)) ^ 0.8
      local y = base - p[2] * prof
      if y < top then top = y end
    end
  end
  return top - 0.5 * nT3(x, 5) - 0.4
end

FS.band2 = pile{{"raw umber",2},{"smalt",1.8},{"Prussian blue",0.25},{"bone black",0.6},{"red earth",0.2}}
work(BAND, {hand="body", tool="filbert 4", pile=FS.band2, angle=0, coverage=3.5, fill=true, clip=true, seed=1211})

-- foliage fuzz: small touches over and around the top edge, clustered
local fz = function(x, y)
  local d = y - TOPAT(x)
  if d < -3.2 or d > 4 then return 0 end
  local a = smoothstep(-3.2, 0.2, d) * (1 - smoothstep(0.5, 4, d))
  -- tall poplars carry finer fuzz: nothing to do, same rule
  return 2.2 * a
end
stipple(rect(0, 440, 1000, 50), {pile=FS.band2, width=2.0, coverage=fz, pressure={0.35,0.65}, cluster=0.45, seed=1221, dips={12, 0.85, 0.5}})
print(wait(0))

--@ chunk 76
print(wait(4*24*60))
print(drying(300,484), drying(150,478), drying(650,485))

--@ chunk 77
print(wait(3*24*60))
print(drying(300,484), drying(150,478), drying(650,485), drying(900,482))

--@ chunk 78
HZ = {}
HZ.lilac = pile{{"smalt",1.5},{"red earth",0.55},{"raw umber",0.15},{"lead white",4.5}, medium=0.5}
HAZE = rect(0, 448, 1000, 42) - BAND
local hzload = function(x, y)
  local v = smoothstep(452, 480, y)
  local core = 1 - 0.55 * math.exp(-((x - 655)/110)^2)
  return v * core
end
work(HAZE, {hand="glaze", pile=HZ.lilac, angle=0, coverage=2.2, load_at=hzload, clip=HAZE, seed=1301})
blend(HAZE, {angle=0})
print(wait(0))

--@ chunk 79
WT = {}
WT.glow = pile{{"lead white",8},{"chrome yellow",0.8},{"vermilion",0.15}, medium=0.08}
WT.apri = pile{{"lead white",6},{"chrome yellow",1},{"vermilion",0.45},{"smalt",0.1}, medium=0.08}
WT.pale = pile{{"lead white",6},{"chrome yellow",0.6},{"Prussian blue",0.05},{"smalt",0.25}, medium=0.08}
WT.teal = pile{{"lead white",5},{"Prussian blue",0.18},{"chrome yellow",0.35},{"smalt",0.35}, medium=0.08}
WT.blue = pile{{"cobalt blue",1},{"lead white",1.8},{"smalt",0.8},{"raw umber",0.12}, medium=0.08}

WAT = rect(-5, 487, 1010, 232) - BANK2
local wb = {
  {487, 26, WT.glow},
  {508, 54, WT.apri},
  {558, 58, WT.pale},
  {610, 54, WT.teal},
  {656, 62, WT.blue},
}
for i, b in ipairs(wb) do
  work(rect(-5, b[1], 1010, b[2]) * WAT, {hand="broad", pile=b[3], angle=0, coverage=3, fill=true, clip=WAT, seed=1400+i})
end
print(wait(0))

--@ chunk 80
blend(WAT, {angle=0})
print(wait(0))

--@ chunk 81
WT.rdark = pile{{"smalt",1.2},{"raw umber",0.9},{"red earth",0.3},{"lead white",1.6}, medium=0.2}
WT.rmid  = pile{{"smalt",1.2},{"raw umber",0.5},{"red earth",0.35},{"lead white",3.2}, medium=0.25}

-- 1. poplars and firs: thin wavering verticals below the waterline, each laid on its own
local rg = brush{kind="rigger", width=1.2, point=1}
local function vrefl(x, y0, len, w, seed, pile_, load)
  math.randomseed(seed)
  local pts = {}
  local n = 5
  for i = 0, n do
    local t = i / n
    pts[#pts+1] = {x + rand(-0.9, 0.9) * (0.4 + t), y0 + len * t}
  end
  rg:load(pile_, load or 0.8)
  local p1 = rg:pressure_for(w)
  rg:stroke(pts, {pressure={p1, 0.05}, ramps={0.02, 0.5}, clip=WAT})
end
-- poplars (x, height above waterline): 93/15, 158/21, 290/24, 402/26, 526/17
vrefl(93.5, 488, 13, 1.6, 1, WT.rmid, 0.7)
vrefl(158.5, 488, 20, 2.0, 2, WT.rdark, 0.8)
vrefl(289.5, 488, 24, 2.0, 3, WT.rdark, 0.8)
vrefl(402, 488, 25, 2.2, 4, WT.rdark, 0.8)
vrefl(526.5, 488, 15, 1.8, 5, WT.rmid, 0.7)
-- ruin gable and firs, fainter
vrefl(650, 488, 12, 3.2, 6, WT.rmid, 0.6)
vrefl(657, 488, 9, 2.4, 7, WT.rmid, 0.5)
vrefl(698, 488, 14, 2.2, 8, WT.rmid, 0.6)
vrefl(708, 488, 20, 2.6, 9, WT.rmid, 0.7)
vrefl(721, 488, 18, 2.6, 10, WT.rmid, 0.7)
vrefl(733, 488, 10, 2.0, 11, WT.rmid, 0.6)

-- 2. crowns: broken horizontal dark dashes just under the waterline
local fb = brush("flat", 1.6)
math.randomseed(202)
for i = 1, 260 do
  local x = rand(-5, 1005)
  local y = 488.5 + rand(0, 1) ^ 1.6 * 9
  local len = rand(6, 30)
  fb:load(WT.rmid, rand(0.4, 0.8))
  fb:stroke({{x, y}, {x + len*0.5, y + rand(-0.2, 0.2)}, {x + len, y + rand(-0.2, 0.2)}}, {pressure={0.4, 0.3}, ramps={0.25, 0.5}, clip=WAT})
end
print(wait(0))

--@ chunk 82
WT.rband = pile{{"smalt",1.2},{"raw umber",1.0},{"red earth",0.35},{"bone black",0.2},{"lead white",1.0}, medium=0.3}
local rl = function(x, y)
  -- strong at the waterline, dying away downward; a little longer under the copses
  local reach = 13 + 6 * math.sin(x / 47) * math.sin(x / 131 + 1)
  local t = clamp((y - 487.5) / reach, 0, 1)
  return (1 - t) ^ 1.3
end
RB = rect(-5, 487, 1010, 24) * WAT
work(RB, {hand="glaze", pile=WT.rband, angle=0, coverage=2.4, load_at=rl, clip=WAT, seed=1501})
blend(rect(-5, 488, 1010, 22) * WAT, {angle=0})
print(wait(0))

--@ chunk 83
print(wait(4*24*60))
print(drying(300,495), drying(650,500), drying(650,530))

--@ chunk 84
WT.g0 = pile{{"lead white",8},{"chrome yellow",0.7},{"vermilion",0.12}, medium=0.05}
WT.g1 = pile{{"lead white",6},{"chrome yellow",1.0},{"vermilion",0.35},{"smalt",0.06}, medium=0.05}
WT.g2 = pile{{"lead white",6},{"chrome yellow",0.8},{"vermilion",0.2},{"smalt",0.1}, medium=0.05}
local zones = {
  {486.5, 12, WT.g0},
  {497,   26, WT.g1},
  {519,   26, WT.g2},
}
for i, z in ipairs(zones) do
  work(rect(-5, z[1], 1010, z[2]) * WAT, {hand="body", tool="filbert 12", pile=z[3], angle=0, coverage=3.4, fill=true, clip=WAT, seed=1600+i})
end
blend(rect(-5, 486, 1010, 62) * WAT, {angle=0})
print(wait(0))

--@ chunk 85
SKYM = rect(-5, 0, 1010, 490) - BAND
GZ = {}
GZ.teal   = pile{{"lead white",4},{"Prussian blue",0.10},{"chrome yellow",0.25},{"smalt",0.15}, medium=0.6}
GZ.lemon  = pile{{"lead white",5},{"chrome yellow",0.9},{"Rinmann's green",0.1}, medium=0.6}
GZ.gold   = pile{{"lead white",3},{"chrome yellow",1.2},{"vermilion",0.25}, medium=0.6}
GZ.orange = pile{{"lead white",2},{"chrome yellow",1},{"vermilion",0.7}, medium=0.6}
GZ.rose   = pile{{"lead white",3},{"vermilion",0.6},{"smalt",0.25}, medium=0.6}

local sunfall = function(x, s) return math.exp(-((x - 655) / s) ^ 2) end
-- warm glazes first (horizon glow)
work(SKYM, {hand="glaze", pile=GZ.gold, angle=0, coverage=1.6, clip=SKYM, seed=2001,
  load_at=function(x, y) return smoothstep(350, 405, y) * (1 - smoothstep(448, 476, y)) * (0.45 + 0.55 * sunfall(x, 420)) end})
work(SKYM, {hand="glaze", pile=GZ.orange, angle=0, coverage=1.6, clip=SKYM, seed=2002,
  load_at=function(x, y) return smoothstep(418, 462, y) * sunfall(x, 300) * 0.85 end})
print(wait(0))

--@ chunk 86
SK2 = {}
SK2.a = pile{{"smalt",3},{"cobalt blue",1},{"Prussian blue",0.1},{"lead white",1.2}, medium=0.1}
SK2.b = pile{{"cobalt blue",1},{"lead white",2},{"smalt",1.2}, medium=0.1}
SK2.c = pile{{"cobalt blue",1},{"lead white",3},{"smalt",0.5}, medium=0.1}
SK2.d = pile{{"lead white",4},{"Prussian blue",0.10},{"smalt",0.3},{"chrome yellow",0.06}, medium=0.1}
SK2.e = pile{{"lead white",5},{"Prussian blue",0.07},{"chrome yellow",0.5}, medium=0.1}
local bands = {
  {-5,  92, SK2.a},
  {80,  80, SK2.b},
  {148, 70, SK2.c},
  {208, 52, SK2.d},
  {252, 52, SK2.e},
}
for i, b in ipairs(bands) do
  local fade = nil
  if i == #bands then
    fade = function(x, y) return 1 - smoothstep(288, 306, y) end
  end
  work(rect(-5, b[1], 1010, b[2]), {hand="broad", pile=b[3], angle=0, coverage=3, fill=true, seed=2100 + i, load_at=fade})
end
blend(rect(-5, -5, 1010, 312), {angle=0})
print(wait(0))

--@ chunk 87
print(wait(3*24*60))
print(drying(500,100), drying(650,450), drying(650,500))

--@ chunk 88
print(wait(2*24*60))
print(drying(500,100), drying(650,450), drying(650,500))

--@ chunk 89
GZ.core  = pile{{"lead white",9},{"chrome yellow",0.35},{"vermilion",0.03}, medium=0.35}
GZ.core2 = pile{{"lead white",10},{"chrome yellow",0.2}, medium=0.3}
local M1 = ellipse(655, 462, 340, 58) * SKYM
work(M1, {hand="glaze", pile=GZ.core, angle=0, coverage=2.2, clip=SKYM, seed=2201,
  load_at=radial(655, 464, 340, 58, 1.0, 1.0)})
blend(M1:grow(6), {angle=0})
local M2 = ellipse(655, 466, 160, 34) * SKYM
work(M2, {hand="glaze", pile=GZ.core2, angle=0, coverage=2.2, clip=SKYM, seed=2202,
  load_at=radial(655, 467, 160, 34, 1.0, 0.9)})
blend(M2:grow(4), {angle=0})
print(wait(0))

--@ chunk 90
-- A. repaint the horizon strip so the far shore can be redone cleanly on a fresh, luminous ground
HS = {}
HS.a = pile{{"lead white",4},{"chrome yellow",1.1},{"vermilion",0.35}, medium=0.08}
HS.b = pile{{"lead white",6},{"chrome yellow",0.7},{"vermilion",0.3}, medium=0.08}
HS.c = pile{{"lead white",8},{"chrome yellow",0.3},{"vermilion",0.2},{"smalt",0.1}, medium=0.08}
STRIP = rect(-5, 444, 1010, 44.5)      -- down to the waterline at 488.5
local top_fade = function(x, y) return smoothstep(444, 456, y) end
work(rect(-5, 444, 1010, 26), {hand="broad", pile=HS.a, angle=0, coverage=2.6, fill=true, clip=STRIP, seed=2301, load_at=top_fade})
work(rect(-5, 464, 1010, 16), {hand="broad", pile=HS.b, angle=0, coverage=3.2, fill=true, clip=STRIP, seed=2302})
work(rect(-5, 475, 1010, 14.5), {hand="broad", pile=HS.c, angle=0, coverage=3.2, fill=true, clip=STRIP, seed=2303})
blend(rect(-5, 446, 1010, 42), {angle=0})
print(wait(0))

--@ chunk 91
GZ.seam = pile{{"lead white",4},{"chrome yellow",1.0},{"vermilion",0.3}, medium=0.45}
local sm = rect(-5, 426, 1010, 50)
work(sm, {hand="glaze", pile=GZ.seam, angle=0, coverage=1.8, clip=sm, seed=2401,
  load_at=function(x, y) return smoothstep(428, 446, y) * (1 - smoothstep(452, 474, y)) end})
blend(rect(-5, 432, 1010, 34), {angle=0})
print(wait(0))

--@ chunk 92
print(wait(3*24*60))
print(drying(300,470), drying(650,480), drying(650,450), drying(650,488))

--@ chunk 93
FS.hills = pile{{"smalt",2},{"red earth",0.5},{"raw umber",0.2},{"lead white",4.5}, medium=0.2}
FS.land  = pile{{"smalt",2},{"raw umber",1.0},{"red earth",0.45},{"lead white",1.2}}
FS.copse = pile{{"smalt",2},{"raw umber",1.5},{"red earth",0.3},{"Prussian blue",0.15},{"lead white",0.6}}
FS.dark  = pile{{"smalt",2},{"raw umber",1.7},{"Prussian blue",0.3},{"bone black",0.4},{"lead white",0.3}}

-- pale distant ridge on the right, behind everything
local hp = {{700,486},{735,481},{770,477.5},{805,474},{845,470.5},{885,468},{925,467},{965,469},{1010,472},{1010,488},{700,488}}
work(poly(hp, true), {hand="body", tool="filbert 4", pile=FS.hills, angle=0, coverage=2.6, fill=true, clip=true, seed=2501})
-- and a very low, fainter one on the left
local hp2 = {{-5,485},{40,482},{90,480.5},{140,482},{190,481},{240,483},{290,484},{290,488},{-5,488}}
work(poly(hp2, true), {hand="body", tool="filbert 4", pile=FS.hills, angle=0, coverage=2.0, fill=true, clip=true, seed=2502})

-- the low land strip along the whole waterline: its top wanders by a couple of units
LANDTOP = outline{{-8,484.6},{60,484.0},{130,484.9},{200,484.2},{280,485.0},{350,484.1},{430,484.8},{520,484.3},{600,485.0},{660,484.6},{740,485.2},{830,484.4},{920,485.0},{1010,484.6},
                  char="soft", seed=2510, amount=0.9, lobe=6}
local function under2(o, base)
  local p = o:path(1)
  local pts = {}
  for i = 1, #p do pts[i] = {p[i][1], p[i][2]} end
  table.sort(pts, function(a, b) return a[1] < b[1] end)
  return below(pts) * above(function(x) return base end)
end
LANDM = under2(LANDTOP, 487.7)
work(LANDM, {hand="body", tool="filbert 3", pile=FS.land, angle=0, coverage=4, fill=true, clip=true, seed=2511})
print(wait(0))

--@ chunk 94
-- cover the comb-edged strip and the stray dark flecks
HS.c2 = pile{{"lead white",8},{"chrome yellow",0.32},{"vermilion",0.2},{"smalt",0.08}, medium=0.05}
local cover_left = rect(-5, 466, 705, 22.5)
work(cover_left, {hand="body", tool="filbert 8", pile=HS.c2, angle=0, coverage=4, fill=true, clip=cover_left, seed=2601})
local cover_right = rect(700, 479, 310, 9.5)
work(cover_right, {hand="body", tool="filbert 6", pile=FS.hills, angle=0, coverage=4, fill=true, clip=cover_right, seed=2602})
print(wait(0))

--@ chunk 95
blend(rect(-5, 455, 710, 24), {angle=0})
blend(rect(-5, 455, 710, 24), {angle=0})
print(wait(0))

--@ chunk 96
HG = {}
HG.gold  = pile{{"lead white",4},{"chrome yellow",1.1},{"vermilion",0.35}, medium=0.06}
HG.mid   = pile{{"lead white",6},{"chrome yellow",0.85},{"vermilion",0.3}, medium=0.06}
HG.cream = pile{{"lead white",8},{"chrome yellow",0.5},{"vermilion",0.22}, medium=0.06}
HZONE = rect(-5, 426, 1010, 63)          -- to the waterline (489)
local ramp_top = function(x, y) return smoothstep(426, 452, y) end
work(rect(-5, 426, 1010, 34), {hand="broad", pile=HG.gold, angle=0, coverage=3, fill=true, clip=HZONE, seed=2701, load_at=ramp_top})
work(rect(-5, 452, 1010, 26), {hand="broad", pile=HG.mid, angle=0, coverage=3, fill=true, clip=HZONE, seed=2702, load_at=function(x, y) return smoothstep(452, 464, y) end})
work(rect(-5, 470, 1010, 19), {hand="broad", pile=HG.cream, angle=0, coverage=3.4, fill=true, clip=HZONE, seed=2703, load_at=function(x, y) return smoothstep(470, 478, y) end})
blend(rect(-5, 428, 1010, 61), {angle=0})
print(wait(0))

--@ chunk 97
blend(rect(-5, 418, 1010, 22), {angle=0})
print(wait(0))

--@ chunk 98
-- ===== far shore, third and careful version =====
FS.d1 = pile{{"smalt",2},{"raw umber",1.3},{"red earth",0.4},{"Prussian blue",0.15},{"lead white",0.5}}
FS.d2 = pile{{"smalt",2},{"raw umber",1.0},{"red earth",0.45},{"lead white",1.3}}
FS.d3 = pile{{"smalt",2},{"raw umber",0.85},{"red earth",0.5},{"lead white",2.0}}
local function pick(x)
  if x < 430 then return FS.d1 elseif x < 610 then return FS.d2 elseif x < 800 then return FS.d3 else return FS.d2 end
end

-- 1. the low land strip at the waterline
LANDTOP = outline{{-8,485.7},{60,485.2},{130,486.0},{200,485.4},{280,486.1},{350,485.3},{430,485.9},{520,485.4},{600,486.0},{660,485.6},{740,486.2},{830,485.5},{920,486.1},{1010,485.7},
                  char="soft", seed=2610, amount=0.9, lobe=6}
LANDM = under(LANDTOP, 489)
work(LANDM, {hand="detail", pile=FS.d2, angle=0, coverage=4, fill=true, clip=true, seed=2611})

-- 2. tree crowns: each a union of a few lobes drawn from its own numbers
local function crown(cx, w, h, seed)
  math.randomseed(seed)
  local m
  local n = math.random(3, 5)
  for i = 1, n do
    local t = (i - 0.5) / n
    local x = cx + (t - 0.5) * w * 0.85 + rand(-w * 0.06, w * 0.06)
    local hh = h * (0.5 + 0.5 * math.sin(math.pi * (0.12 + 0.76 * t))) * rand(0.8, 1.05)
    local rx = (w / n) * rand(0.8, 1.05)
    local e = ellipse(x, 487 - hh * 0.5, rx, hh * 0.5 + 1.2)
    m = m and (m + e) or e
  end
  return m:roughen(0.9, 5, seed)
end
local CR = {
  {14,26,6},{44,30,9},{74,22,7},{104,28,10},{134,20,6},
  {176,22,7},{200,26,5},{228,12,3},
  {258,26,7},{286,22,9},{316,24,7},{342,18,5},
  {372,30,8},{398,26,11},{430,24,7},{456,20,5},
  {486,28,7},{516,24,9},{548,20,6},{575,26,5},{602,18,5},
  {690,10,3},
  {760,24,4},{790,28,5},{826,22,4},{860,30,5},{898,26,4},{934,24,3.5},{968,26,4},{996,20,3},
}
for i, c in ipairs(CR) do
  local m = crown(c[1], c[2], c[3], 3000 + i)
  work(m, {hand="detail", pile=pick(c[1]), angle=0.3, coverage=4, fill=true, clip=true, seed=3100 + i})
end

-- 3. poplars, spire and church, ruin and firs
local function poplar(x, h, w, seed)
  math.randomseed(seed)
  local L, R = {}, {}
  local n = 9
  for i = 0, n do
    local t = i / n
    local prof
    if t < 0.35 then prof = 0.55 + 0.45 * (t / 0.35) else prof = (1 - (t - 0.35) / 0.65) ^ 0.85 end
    local hw = w * 0.5 * prof + ((i > 0 and i < n) and rand(-0.25, 0.3) or 0)
    local y = 487.5 - h * t
    L[#L+1] = {x - hw, y}
    R[#R+1] = {x + hw + rand(-0.15, 0.2), y + rand(-0.4, 0.4)}
  end
  local pts = {}
  for i = 1, #L do pts[#pts+1] = L[i] end
  for i = #R, 1, -1 do pts[#pts+1] = R[i] end
  return poly(pts)
end
local PP = {{93,15,4.4},{158,22,5.2},{404,27,5.4},{526,18,4.6},{32,13,4.0}}
for i, p in ipairs(PP) do
  work(poplar(p[1], p[2], p[3], 3300 + i), {hand="detail", pile=pick(p[1]), angle=1.5, coverage=4, fill=true, clip=true, seed=3400 + i})
end
-- church with spire at x=291
CHURCH = poly({{276,487.5},{276,481.5},{281,478.8},{287.2,478.8},{287.2,470},{288,469.2},{291,452},{294,469.2},{294.8,470},{294.8,487.5}})
work(CHURCH, {hand="detail", pile=FS.d1, angle=1.5, coverage=4, fill=true, clip=true, seed=3500})
work(RUIN - LANCET, {hand="detail", pile=FS.d3, angle=1.4, coverage=4, fill=true, clip=true, seed=3501})
for i, f in ipairs({FIR1, FIR2, FIR3, FIR4}) do
  work(f, {hand="detail", pile=FS.d3, angle=1.5, coverage=4, fill=true, clip=true, seed=3510 + i})
end
print(wait(0))

--@ chunk 99
print(wait(2*24*60))
print(drying(300,484), drying(650,470), drying(100,480))

--@ chunk 100
print(wait(3*24*60))
print(drying(300,484), drying(650,470), drying(100,480))

--@ chunk 101
-- ===== fixes to the far shore =====
FS.knoll = pile{{"smalt",2},{"raw umber",0.7},{"red earth",0.5},{"lead white",2.7}}
FS.dk    = pile{{"raw umber",2},{"smalt",1.6},{"Prussian blue",0.3},{"bone black",0.5},{"lead white",0.25}}

-- knoll under the ruin and firs
KN = poly({{575,487.5},{592,485},{610,482.5},{625,479.5},{640,478.2},{665,477.7},{700,477.4},{730,477.7},{750,479},{770,482},{795,485},{815,487.5}}, true)
KNm = KN - RUIN - FIR1 - FIR2 - FIR3 - FIR4
work(KNm, {hand="detail", pile=FS.knoll, angle=0.1, coverage=4, fill=true, clip=true, seed=3601})

-- shoreline scrub: a low bushy band whose top is a noise profile; covers the comb marks
local nS1 = noise{seed=811, octaves=3, period=40, persistence=0.6}
local nS2 = noise{seed=812, octaves=2, period=7, persistence=0.5}
local sp = {}
for x = -6, 1006, 2 do
  local h = 2.4 + 1.5 * nS1(x, 0) + 0.7 * nS2(x, 0)
  sp[#sp+1] = {x, 487.6 - math.max(0.8, h)}
end
sp[#sp+1] = {1006, 489.5}; sp[#sp+1] = {-6, 489.5}
SCRUB = poly(sp)
work(SCRUB, {hand="detail", pile=FS.d2, angle=0.2, coverage=5, fill=true, clip=true, seed=3602})

-- darken the crowns of the left and middle: same lobes, second pass
local function crown(cx, w, h, seed)
  math.randomseed(seed)
  local m
  local n = math.random(3, 5)
  for i = 1, n do
    local t = (i - 0.5) / n
    local x = cx + (t - 0.5) * w * 0.85 + rand(-w * 0.06, w * 0.06)
    local hh = h * (0.5 + 0.5 * math.sin(math.pi * (0.12 + 0.76 * t))) * rand(0.8, 1.05)
    local rx = (w / n) * rand(0.8, 1.05)
    local e = ellipse(x, 487 - hh * 0.5, rx, hh * 0.5 + 1.2)
    m = m and (m + e) or e
  end
  return m:roughen(0.9, 5, seed)
end
local CR = {
  {14,26,6},{44,30,9},{74,22,7},{104,28,10},{134,20,6},
  {176,22,7},{200,26,5},{228,12,3},
  {258,26,7},{286,22,9},{316,24,7},{342,18,5},
  {372,30,8},{398,26,11},{430,24,7},{456,20,5},
  {486,28,7},{516,24,9},{548,20,6},{575,26,5},{602,18,5},
  {690,10,3},
  {760,24,4},{790,28,5},{826,22,4},{860,30,5},{898,26,4},{934,24,3.5},{968,26,4},{996,20,3},
}
CROWNALL = nil
for i, c in ipairs(CR) do
  local m = crown(c[1], c[2], c[3], 3000 + i)
  CROWNALL = CROWNALL and (CROWNALL + m) or m
  if c[1] < 600 then
    work(m, {hand="detail", pile=FS.dk, angle=0.3, coverage=3, fill=true, clip=true, seed=3700 + i, load_at=function(x, y) return 0.5 + 0.4 * smoothstep(478, 487, y) end})
  end
end
print(wait(0))

--@ chunk 102
-- ===== reflection of the far shore in the still water =====
WT.r1 = pile{{"smalt",1.3},{"raw umber",1.0},{"red earth",0.35},{"bone black",0.15},{"lead white",1.6}, medium=0.35}
WT.r2 = pile{{"smalt",1.3},{"raw umber",0.6},{"red earth",0.4},{"lead white",3.0}, medium=0.35}
WATERTOP = 489.2
local rl2 = function(x, y)
  local k = 1.0 - 0.62 * smoothstep(560, 700, x) * (1 - 0.5 * smoothstep(850, 1000, x))
  local reach = 8 + 4 * math.sin(x / 41) * math.sin(x / 97 + 1)
  local t = clamp((y - WATERTOP) / reach, 0, 1)
  return k * (1 - t) ^ 1.5
end
RB2 = rect(-5, 488.5, 1010, 20) * WAT
work(RB2, {hand="glaze", pile=WT.r1, angle=0, coverage=2.4, load_at=rl2, clip=WAT, seed=4001})
blend(rect(-5, 489, 1010, 18) * WAT, {angle=0})
print(wait(0))

--@ chunk 103
print(wait(4*24*60))
print(drying(300,495), drying(700,495), drying(650,520))

--@ chunk 104
-- ===== repair the water under the far shore: clean base band =====
WT.g0 = pile{{"lead white",8},{"chrome yellow",0.7},{"vermilion",0.14}, medium=0.05}
WT.g1 = pile{{"lead white",6},{"chrome yellow",1.0},{"vermilion",0.33},{"smalt",0.06}, medium=0.05}
WT.g2 = pile{{"lead white",6},{"chrome yellow",0.85},{"vermilion",0.2},{"smalt",0.1}, medium=0.05}
WB = rect(-5, 488.8, 1010, 46) * WAT
local fade_bot = function(x, y) return 1 - smoothstep(520, 536, y) end
work(rect(-5, 488.8, 1010, 12) * WAT, {hand="body", tool="filbert 10", pile=WT.g0, angle=0, coverage=3.4, fill=true, clip=WAT, seed=4101})
work(rect(-5, 497, 1010, 22) * WAT, {hand="body", tool="filbert 10", pile=WT.g1, angle=0, coverage=3.4, fill=true, clip=WAT, seed=4102})
work(rect(-5, 514, 1010, 24) * WAT, {hand="body", tool="filbert 10", pile=WT.g2, angle=0, coverage=3.0, fill=true, clip=WAT, seed=4103, load_at=fade_bot})
blend(rect(-5, 489, 1010, 48) * WAT, {angle=0})
print(wait(0))

--@ chunk 105
WT.rf1 = pile{{"smalt",1.6},{"raw umber",1.0},{"red earth",0.4},{"Prussian blue",0.1},{"lead white",1.3}, medium=0.1}
WT.rf2 = pile{{"smalt",1.4},{"raw umber",0.6},{"red earth",0.4},{"lead white",3.0}, medium=0.1}

-- hand-drawn reflection blob for a crown or a spire: wavering sides, hanging from the waterline
local function refl_shape(cx, w, depth, seed, taper)
  math.randomseed(seed)
  local n = 7
  local Lp, Rp = {}, {}
  for i = 0, n do
    local t = i / n
    local y = 488.8 + depth * t
    local half = (w / 2) * (1 - (taper or 0.45) * t ^ 1.3) + rand(-0.6, 0.6) * (0.4 + t)
    Lp[#Lp+1] = {cx - half + rand(-0.8, 0.8) * t, y}
    Rp[#Rp+1] = {cx + half + rand(-0.8, 0.8) * t, y}
  end
  local pts = {}
  for i = 1, #Lp do pts[#pts+1] = Lp[i] end
  for i = #Rp, 1, -1 do pts[#pts+1] = Rp[i] end
  return poly(pts, true)
end

-- crowns: {x, width, height}; the reflections reach about their own height and a little more
local CRR = {
  {14,26,6},{44,30,9},{74,22,7},{104,28,10},{134,20,6},
  {176,22,7},{200,26,5},{258,26,7},{286,22,9},{316,24,7},{342,18,5},
  {372,30,8},{398,26,11},{430,24,7},{456,20,5},
  {486,28,7},{516,24,9},{548,20,6},{575,26,5},{602,18,5},
  {760,24,4},{790,28,5},{826,22,4},{860,30,5},{898,26,4},{934,24,3.5},{968,26,4},{996,20,3},
}
REFLALL = nil
for i, c in ipairs(CRR) do
  local m = refl_shape(c[1], c[2] * 0.95, c[3] * 1.15 + 3, 5000 + i)
  REFLALL = REFLALL and (REFLALL + m) or m
  local pl = (c[1] < 640) and WT.rf1 or WT.rf2
  work(m * WAT, {hand="body", tool="filbert 3", pile=pl, angle=0, coverage=3, fill=true, clip=WAT, seed=5100 + i})
end
-- spires and poplars: thin hanging lines
local rg = brush{kind="rigger", width=1.2, point=1}
local function hang(x, y0, len, w, seed, pl, load)
  math.randomseed(seed)
  local pts = {}
  for i = 0, 6 do
    local t = i / 6
    pts[#pts+1] = {x + rand(-0.5, 0.5) * (0.3 + t), y0 + len * t}
  end
  rg:load(pl, load or 0.8)
  rg:stroke(pts, {pressure={rg:pressure_for(w), 0.06}, ramps={0.02, 0.6}, clip=WAT})
end
hang(93.5, 488.8, 17, 1.8, 6001, WT.rf1, 0.8)
hang(158.5, 488.8, 23, 2.2, 6002, WT.rf1, 0.85)
hang(291, 488.8, 27, 2.6, 6003, WT.rf1, 0.85)
hang(404, 488.8, 28, 2.4, 6004, WT.rf1, 0.85)
hang(526.5, 488.8, 19, 2.0, 6005, WT.rf1, 0.8)
hang(32, 488.8, 14, 1.8, 6006, WT.rf1, 0.75)
hang(968, 488.8, 22, 2.4, 6007, WT.rf2, 0.7)
hang(650, 488.8, 14, 3.6, 6008, WT.rf2, 0.6)
hang(699, 488.8, 16, 2.6, 6009, WT.rf2, 0.6)
hang(708, 488.8, 22, 2.8, 6010, WT.rf2, 0.65)
hang(721, 488.8, 20, 2.8, 6011, WT.rf2, 0.65)
hang(733, 488.8, 12, 2.2, 6012, WT.rf2, 0.55)
print(wait(0))

--@ chunk 106
blend(rect(-5, 489, 1010, 26) * WAT, {angle=0})
print(wait(0), drying(300, 495), drying(700, 500))

--@ chunk 107
blend(rect(-5, 500, 1010, 42) * WAT, {angle=0})
print(wait(0))

--@ chunk 108
-- ===== the oak, redesigned: skeleton =====
OAK2 = {
  T  = {pts={{198,660},{203,610},{208,565},{212,520},{216,478}}, ws={94,74,62,56,52}},
  L1 = {pts={{214,488},{184,480},{150,464},{118,440},{86,422},{52,416},{22,402},{-8,374}}, ws={42,37,32,27,22,17,13,9}},
  C1 = {pts={{216,482},{224,442},{216,400},{234,360},{228,318},{246,280},{240,242},{256,210},{250,180},{262,152}}, ws={46,42,38,33,29,25,21,16,10,3}},
  R1 = {pts={{222,490},{262,478},{300,454},{336,438},{372,422},{408,394},{438,362},{470,338},{504,324}}, ws={42,38,34,30,26,22,18,14,9}},
  L2 = {pts={{118,440},{108,406},{88,378},{86,342},{66,310},{60,274},{44,242}}, ws={22,19,16,13,10,7,3}},
  L3 = {pts={{218,406},{192,380},{170,348},{164,312},{142,284},{130,248},{108,224}}, ws={24,20,17,14,11,7,3}},
  R2 = {pts={{234,360},{262,340},{292,314},{296,278},{318,250},{322,216},{342,192}}, ws={22,18,16,13,10,6,3}},
  R3 = {pts={{336,438},{362,450},{388,470},{404,496},{410,524}}, ws={20,16,12,8,3}},
  R4 = {pts={{408,394},{404,358},{420,324},{414,290},{430,258}}, ws={15,12,9,6,3}},
  L4 = {pts={{52,416},{40,382},{18,354},{6,324}}, ws={13,10,7,3}},
  R5 = {pts={{470,338},{482,308},{500,284}}, ws={10,6,3}},
}
OAK2_ORDER = {"T","L1","C1","R1","L2","L3","R2","R3","R4","L4","R5"}
OAK2_POLY = {}
OAK2_MASK = nil
for i, k in ipairs(OAK2_ORDER) do
  local t = OAK2[k]
  local pp = limb_outline(t.pts, t.ws, 5, i * 19)
  OAK2_POLY[k] = pp
  local m = poly(pp)
  OAK2_MASK = OAK2_MASK and (OAK2_MASK + m) or m
end
-- pencil preview on the (dry) canvas: edges of every limb
local pv = pencil("HB")
for k, pp in pairs(OAK2_POLY) do
  pv:line(pp, {pressure=0.3, smooth=false})
end
print("previewed")

--@ chunk 109
local rows = {}
for y = 140, 600, 8 do
  local s = ""
  for x = -10, 530, 5 do
    local v = OAK2_MASK:at(x, y)
    s = s .. (v > 0.5 and "#" or ".")
  end
  rows[#rows+1] = string.format("%3d ", y) .. s
end
print(table.concat(rows, "\n"))

--@ chunk 110
-- final widths (secondary limbs a little heavier), rebuild polygons
OAK2_W = {}
for k, t in pairs(OAK2) do
  local f = (k == "T") and 1.0 or 1.12
  local ws = {}
  for i, w in ipairs(t.ws) do ws[i] = w * f end
  OAK2_W[k] = ws
end
OAK2_POLY = {}
OAK2_MASKS = {}
OAK2_DENSE = {}
for i, k in ipairs(OAK2_ORDER) do
  local t = OAK2[k]
  local pp = limb_outline(t.pts, OAK2_W[k], 5, i * 19)
  OAK2_POLY[k] = pp
  OAK2_MASKS[k] = poly(pp)
  OAK2_DENSE[k] = crspline(t.pts, 4)
end

-- stubs: {x, y, angle, length, w0, w1}
STUBS = {
  {190, 532, -2.55, 30, 15, 8},
  {228, 322, -2.30, 26, 9, 5},
  {304, 456,  1.35, 30, 11, 6},
  {156, 466,  2.05, 24, 9, 5},
  {208, 262, -0.95, 22, 8, 4},
  {392, 398, -0.35, 26, 8, 4},
}
STUBP = {}
for i, s in ipairs(STUBS) do
  local x, y, a, L, w0, w1 = table.unpack(s)
  local pts = {{x, y}, {x + math.cos(a) * L * 0.5 + rand(-1.5, 1.5), y + math.sin(a) * L * 0.5 + rand(-1.5, 1.5)}, {x + math.cos(a) * L, y + math.sin(a) * L}}
  STUBP[i] = limb_outline(pts, {w0, (w0 + w1) * 0.5, w1}, 1.2, 300 + i)
end

OK = {}
OK.base = pile{{"raw umber",3},{"bone black",1.6},{"Prussian blue",0.5},{"red earth",0.35}}
OK.thin = pile{{"raw umber",3},{"bone black",1.4},{"Prussian blue",0.4},{"red earth",0.4}, medium=0.1}

local function angfn(k)
  local d = OAK2_DENSE[k]
  return function(x, y)
    local best, bi = 1e18, 1
    for i = 1, #d do
      local dx, dy = x - d[i][1], y - d[i][2]
      local dd = dx*dx + dy*dy
      if dd < best then best = dd; bi = i end
    end
    local a, b = d[math.max(1, bi - 1)], d[math.min(#d, bi + 1)]
    return math.atan(b[2] - a[2], b[1] - a[1])
  end
end

for i, k in ipairs(OAK2_ORDER) do
  local big = (OAK2_W[k][1] > 30)
  work(OAK2_MASKS[k], {hand="detail", tool=big and "filbert 6" or "round 3", pile=OK.base, angle=angfn(k),
    coverage=big and 3.2 or 4, fill=true, clip=true, seed=7000 + i, length=big and {18, 40} or {6, 14}})
end
for i, pp in ipairs(STUBP) do
  work(poly(pp), {hand="detail", tool="round 3", pile=OK.base, angle=STUBS[i][3], coverage=4, fill=true, clip=true, seed=7100 + i})
end
print(wait(0))

--@ chunk 111
function angfn(k)
  local d = OAK2_DENSE[k]
  return function(x, y)
    local best, bi = 1e18, 1
    for i = 1, #d do
      local dx, dy = x - d[i][1], y - d[i][2]
      local dd = dx*dx + dy*dy
      if dd < best then best = dd; bi = i end
    end
    local a, b = d[math.max(1, bi - 1)], d[math.min(#d, bi + 1)]
    return math.atan(b[2] - a[2], b[1] - a[1])
  end
end
OK.deep = pile{{"raw umber",2.4},{"bone black",2.2},{"Prussian blue",0.55},{"red earth",0.25}}
for i, k in ipairs(OAK2_ORDER) do
  local big = (OAK2_W[k][1] > 30)
  work(OAK2_MASKS[k], {hand="detail", tool=big and "filbert 7" or "round 3", pile=OK.deep, angle=angfn(k),
    coverage=big and 5 or 4, fill=true, clip=true, seed=7200 + i, length=big and {14, 34} or {6, 14},
    load=0.95})
end
for i, pp in ipairs(STUBP) do
  work(poly(pp), {hand="detail", tool="round 3", pile=OK.deep, angle=STUBS[i][3], coverage=4, fill=true, clip=true, seed=7300 + i})
end
print(wait(0))

--@ chunk 112
-- test: rigger branch segments with tapering pressure, in pale paint on the dark bank (bottom-left), to be covered later
local tp = pile{{"lead white",4},{"yellow ochre",1}}
local rg = brush{kind="rigger", width=8, point=1}
print("pressure_for 8:", rg:pressure_for(8), "4:", rg:pressure_for(4), "2:", rg:pressure_for(2), "1:", rg:pressure_for(1), "0.5:", rg:pressure_for(0.5))
local function seg_branch(b, pts, ws)
  for i = 1, #pts - 1 do
    b:stroke({pts[i], pts[i+1]}, {pressure={b:pressure_for(ws[i]), b:pressure_for(ws[i+1])}, ramps={0, 0}})
  end
end
rg:load(tp, 1.0)
seg_branch(rg, {{20,690},{36,676},{52,670},{70,656},{92,650},{110,636}}, {8,6.5,5,3.6,2.4,1.2})
rg:load(tp, 1.0)
seg_branch(rg, {{20,705},{44,700},{66,706},{86,696},{108,698},{130,688}}, {4,3.2,2.6,2,1.4,0.8})
-- whole-stroke version
rg:load(tp, 1.0)
rg:stroke({{150,700},{170,690},{190,684},{210,670},{236,664}}, {pressure={rg:pressure_for(5), 0.05}, ramps={0, 0.3}})
print("ok")

--@ chunk 113
local tp = pile{{"lead white",4},{"yellow ochre",1}}
local tests = {
  {"rigger", 3, 1.0, {1.0, 0.0}, {0.0, 0.85}},
  {"rigger", 6, 1.0, {1.0, 0.0}, {0.0, 0.85}},
  {"round",  6, 1.0, {1.0, 0.0}, {0.0, 0.85}},
  {"round",  3, 1.0, {0.8, 0.0}, {0.0, 0.85}},
  {"rigger", 3, 1.0, {0.6, 0.6}, {0.0, 0.0}},
  {"rigger", 1.5, 1.0, {1.0, 0.3}, {0.0, 0.0}},
}
for i, t in ipairs(tests) do
  local b = brush{kind=t[1], width=t[2], point=t[3]}
  b:load(tp, 1.0)
  local y = 636 + (i - 1) * 12
  b:stroke({{6, y}, {40, y - 4}, {75, y + 3}, {110, y - 2}, {150, y + 2}}, {pressure=t[4], ramps=t[5]})
  print(i, t[1], t[2], "mark_width(0.3)=", b:mark_width(0.3), "mark_width(1)=", b:mark_width(1.0), "mark_width(0.05)=", b:mark_width(0.05))
end

--@ chunk 114
-- ===== procedural twig generator (each branch is its own random walk) =====
OK.tw = pile{{"raw umber",2.4},{"bone black",2.2},{"Prussian blue",0.55},{"red earth",0.25}}
BR = {}   -- every generated branch: {pts=, w0=}

local function angdiff(a, b) local d = (b - a + math.pi) % (2*math.pi) - math.pi; return d end

-- grow one branch; a random walk with kinks, an upward tropism, and side branches by the area rule
function grow(x, y, ang, len, w0, depth, cfg)
  local step = clamp(len / 5, 4.5, 13)
  local n = math.max(2, math.floor(len / step))
  local pts = {{x, y}}
  local cx, cy, a = x, y, ang
  local K = cfg.kink[math.min(depth + 1, #cfg.kink)]
  for i = 1, n do
    a = a + rand(-1, 1) * K + cfg.trop * angdiff(a, cfg.up)
    cx = cx + math.cos(a) * step
    cy = cy + math.sin(a) * step
    pts[#pts+1] = {cx, cy}
    local wl = w0 * (1 - i / n)
    if depth < cfg.maxd and i >= 2 and i < n and wl * 0.62 > 0.4 and rand() < cfg.psp[math.min(depth + 1, #cfg.psp)] then
      local side = (rand() < 0.5) and -1 or 1
      local ca = a + side * rand(0.45, 1.05)
      local cl = len * rand(0.35, 0.7) * (1 - i / n) + 8
      grow(cx, cy, ca, cl, wl * rand(0.5, 0.72), depth + 1, cfg)
    end
  end
  BR[#BR+1] = {pts=pts, w0=w0, depth=depth}
  return pts
end

-- paint a branch as one tapering stroke from its base to a hairline tip
function paint_branch(b, pile_)
  local w0 = b.w0
  local kind, W
  if w0 >= 1.6 then kind, W = "round", w0 / 1.225 else kind, W = "rigger", math.max(0.7, w0 / 1.3) end
  local br = brush{kind=kind, width=W, point=1}
  br:load(pile_, 1.0)
  local p0 = math.min(1, br:pressure_for(w0) + 0.02)
  br:stroke(b.pts, {pressure={p0, 0.0}, ramps={0, 0}})
end
print("generator ready")

--@ chunk 115
function widthAt(ws, s)
  local i = math.max(1, math.min(#ws - 1, math.floor(s)))
  local f = s - i
  f = f*f*(3 - 2*f)
  return ws[i] * (1 - f) + ws[i+1] * f
end

CFG = {kink={0.30, 0.42, 0.55, 0.6}, trop=0.10, up=-math.pi/2, maxd=3, psp={0.34, 0.42, 0.34, 0.2}}

function limb_branches(k, t0, t1, gap, lenmul, side_pref)
  local t = OAK2[k]
  local dpts, idx = crspline(t.pts, 4)
  local s = 0
  local made = 0
  local flip = (rand() < 0.5) and 1 or -1
  for i = 2, #dpts - 1 do
    local u = idx[i]
    if u >= t0 and u <= t1 then
      local px, py = dpts[i-1][1], dpts[i-1][2]
      local d = math.sqrt((dpts[i][1]-px)^2 + (dpts[i][2]-py)^2)
      s = s + d
      if s >= gap then
        s = rand(-0.3, 0.5) * gap
        local a = dpts[i+1]; local b = dpts[i-1]
        local ta = math.atan(a[2]-b[2], a[1]-b[1])
        local w = widthAt(OAK2_W[k], u)
        flip = -flip
        local sd = side_pref and (rand() < 0.7 and side_pref or -side_pref) or flip
        local ca = ta + sd * rand(0.5, 1.0)
        local x0 = dpts[i][1] + math.cos(ta + sd * math.pi/2) * w * 0.35
        local y0 = dpts[i][2] + math.sin(ta + sd * math.pi/2) * w * 0.35
        local L = clamp(w * rand(3.0, 5.5) * (lenmul or 1), 22, 110)
        if y0 < 585 then
          grow(x0, y0, ca, L, clamp(w * rand(0.3, 0.46), 1.2, 11), 1, CFG)
          made = made + 1
        end
      end
    end
  end
  return made
end

local before = #BR
local m1 = limb_branches("L1", 1.6, 7.6, 30, 1.0)
print("L1 side branches:", m1, "total branches:", #BR - before)
for i = before + 1, #BR do paint_branch(BR[i], OK.tw) end
print(wait(0))

--@ chunk 116
-- ===== oak twig generator, version 2: zigzag axes (sympodial), side branches on the outside of each elbow =====
function angdiff(a, b) return (b - a + math.pi) % (2*math.pi) - math.pi end

TWCFG = {
  zz = {{0.12,0.40},{0.20,0.52},{0.28,0.66},{0.34,0.80}},
  trop = 0.07, maxd = 4, minw = 0.5, psp = 0.62,
}
BR2 = {}
function oakgrow(x, y, ang, len, w0, depth)
  local cfg = TWCFG
  local zz = cfg.zz[math.min(depth + 1, #cfg.zz)]
  local pts = {{x, y}}
  local cx, cy, a = x, y, ang
  local s = 0
  local sgn = (rand() < 0.5) and -1 or 1
  local kids = {}
  local guard = 0
  while s < len and guard < 40 do
    guard = guard + 1
    local step = rand(0.8, 1.2) * clamp(len / 4.5, 4.5, 15)
    local osgn = sgn
    a = a + sgn * rand(zz[1], zz[2]) + cfg.trop * angdiff(a, -math.pi/2)
    sgn = -sgn
    cx = cx + math.cos(a) * step
    cy = cy + math.sin(a) * step
    s = s + step
    pts[#pts+1] = {cx, cy}
    local frac = math.min(1, s / len)
    local wl = w0 * (1 - frac)
    if depth < cfg.maxd and frac < 0.9 and wl * 0.7 > cfg.minw and rand() < cfg.psp then
      local ca = a - osgn * rand(0.45, 0.95)
      local cl = rand(0.45, 0.85) * (len - s) + 5
      kids[#kids+1] = {cx, cy, ca, cl, math.max(cfg.minw * 1.2, wl * rand(0.5, 0.78)), depth + 1}
    end
  end
  BR2[#BR2+1] = {pts=pts, w0=w0, depth=depth}
  for _, k in ipairs(kids) do oakgrow(k[1], k[2], k[3], k[4], k[5], k[6]) end
end

-- side branches along a limb
function limb_twigs(k, u0, u1, opts)
  opts = opts or {}
  local dpts, idx = crspline(OAK2[k].pts, 3)
  local s, made = 0, 0
  local flip = (rand() < 0.5) and 1 or -1
  for i = 2, #dpts - 1 do
    local u = idx[i]
    local px, py = dpts[i-1][1], dpts[i-1][2]
    s = s + math.sqrt((dpts[i][1]-px)^2 + (dpts[i][2]-py)^2)
    local w = widthAt(OAK2_W[k], u)
    local gap = clamp(w * (opts.gapmul or 1.15), opts.gapmin or 13, 34)
    if u >= u0 and u <= u1 and s >= gap then
      s = rand(-0.25, 0.4) * gap
      flip = -flip
      local a, b = dpts[i+1], dpts[i-1]
      local ta = math.atan(a[2]-b[2], a[1]-b[1])
      local ca = ta + flip * rand(0.42, 0.98)
      local y0 = dpts[i][2]
      local keep = 1.0
      if opts.thin then keep = opts.thin end
      -- sparser in the dead top
      keep = keep * (0.30 + 0.70 * smoothstep(150, 300, y0))
      local down_ok = opts.down
      if (math.sin(ca) < 0.5 or down_ok) and rand() < keep then
        local x0 = dpts[i][1] + math.cos(ta + flip * math.pi/2) * w * 0.30
        local yy = dpts[i][2] + math.sin(ta + flip * math.pi/2) * w * 0.30
        if yy < 588 then
          local L = clamp(w * rand(2.4, 5.0) * (opts.lenmul or 1), 16, 96)
          oakgrow(x0, yy, ca, L, clamp(w * rand(0.34, 0.5), 1.0, 9), 1)
          made = made + 1
        end
      end
    end
  end
  return made
end

-- extend a limb's tip into a fan of twigs
function tip_twigs(k, n, len)
  local pts = OAK2[k].pts
  local e, p = pts[#pts], pts[#pts-1]
  local ta = math.atan(e[2]-p[2], e[1]-p[1])
  for i = 1, n do
    local ca = ta + rand(-0.75, 0.75)
    oakgrow(e[1], e[2], ca, len * rand(0.7, 1.15), rand(2.0, 3.2), 1)
  end
end

BR2 = {}
local counts = {}
local plan = {
  {"L1", 2.2, 7.9, {}},
  {"C1", 5.0, 9.9, {thin=0.7}},
  {"R1", 2.0, 8.9, {}},
  {"L2", 1.5, 6.9, {}},
  {"L3", 1.6, 6.9, {}},
  {"R2", 1.6, 6.9, {}},
  {"R3", 1.3, 4.9, {down=true}},
  {"R4", 1.3, 4.9, {}},
  {"L4", 1.3, 3.9, {}},
  {"R5", 1.3, 2.9, {}},
}
for _, p in ipairs(plan) do counts[p[1]] = limb_twigs(p[1], p[2], p[3], p[4]) end
for _, k in ipairs({"L1","L2","L3","R1","R2","R4","R5","L4","C1"}) do tip_twigs(k, 3, 46) end
tip_twigs("R3", 2, 30)
local nb = {0, 0, 0, 0, 0}
for _, b in ipairs(BR2) do nb[math.min(5, b.depth + 1)] = nb[math.min(5, b.depth + 1)] + 1 end
print("branches:", #BR2, "by depth:", table.concat(nb, ","))

-- coarse preview
local gx0, gy0, cell = -10, 100, 5
local cols, rows = 116, 100
local grid = {}
for r = 1, rows do grid[r] = {} for c = 1, cols do grid[r][c] = 0 end end
local function mark(x, y, w)
  local c = math.floor((x - gx0) / cell) + 1
  local r = math.floor((y - gy0) / cell) + 1
  if c >= 1 and c <= cols and r >= 1 and r <= rows then
    local v = (w >= 3.5) and 3 or ((w >= 1.2) and 2 or 1)
    if v > grid[r][c] then grid[r][c] = v end
  end
end
for _, b in ipairs(BR2) do
  for i = 1, #b.pts - 1 do
    local p, q = b.pts[i], b.pts[i+1]
    local d = math.sqrt((q[1]-p[1])^2 + (q[2]-p[2])^2)
    local n = math.max(1, math.floor(d / 1.5))
    for j = 0, n do
      local t = j / n
      local frac = ((i - 1) + t) / (#b.pts - 1)
      mark(p[1] + (q[1]-p[1]) * t, p[2] + (q[2]-p[2]) * t, b.w0 * (1 - frac))
    end
  end
end
for r = 1, rows do
  local x = gx0 + (r-1)*0
  for _, k in ipairs(OAK2_ORDER) do end
end
-- main limbs in the preview as '#'
for y = 1, rows do
  for c = 1, cols do
    local xx, yy = gx0 + (c - 0.5) * cell, gy0 + (y - 0.5) * cell
    if OAK2_MASK and OAK2_MASK:at(xx, yy) > 0.5 then grid[y][c] = 4 end
  end
end
local chars = {[0]=".", "'", "+", "*", "#"}
local out = {}
for r = 1, rows, 2 do
  local s = ""
  for c = 1, cols do s = s .. chars[grid[r][c]] end
  out[#out+1] = string.format("%3d ", gy0 + (r-1)*cell) .. s
end
print(table.concat(out, "\n"))

--@ chunk 117
-- paint the generated twigs: polygons for the thicker ones, tapering rigger strokes for the fine ones
function twig_poly(b)
  local n = #b.pts
  local ws = {}
  for i = 1, n do
    local t = (i - 1) / (n - 1)
    ws[i] = math.max(0.45, b.w0 * (1 - t) ^ 0.85)
  end
  return limb_outline(b.pts, ws, 0.9, math.floor(b.pts[1][1] * 7 + b.pts[1][2]) % 97)
end

function paint_twig(b, pile_, seed)
  if b.w0 >= 1.8 then
    local pp = twig_poly(b)
    local m = poly(pp)
    work(m, {hand="detail", tool=(b.w0 >= 5) and "round 3" or "round 1.8", pile=pile_, angle=math.atan(b.pts[#b.pts][2]-b.pts[1][2], b.pts[#b.pts][1]-b.pts[1][1]),
             coverage=4, fill=true, clip=true, seed=seed, load=0.95})
  else
    local W = math.max(0.7, b.w0 / 1.3)
    local br = brush{kind="rigger", width=W, point=1}
    br:load(pile_, 1.0)
    local p0 = math.min(1, br:pressure_for(b.w0) + 0.02)
    br:stroke(b.pts, {pressure={p0, 0.0}, ramps={0, 0}})
  end
end

-- order: thick first
table.sort(BR2, function(a, b) return a.w0 > b.w0 end)
local t0 = 0
for i, b in ipairs(BR2) do
  paint_twig(b, OK.tw, 9000 + i)
end
print("painted", #BR2)
print(wait(0))

--@ chunk 118
-- solidify the first (bristly) L1 twigs: repaint the thicker ones as polygons
local n = 0
table.sort(BR, function(a, b) return a.w0 > b.w0 end)
for i, b in ipairs(BR) do
  if b.w0 >= 1.8 then
    paint_twig(b, OK.tw, 9800 + i)
    n = n + 1
  end
end
print("solidified", n)

-- connect the floating stub to the leader as a short broken side limb
local sp = {{238,276},{224,270},{212,262},{202,250}}
local pp = limb_outline(sp, {13, 10, 8, 5}, 1.2, 411)
work(poly(pp), {hand="detail", tool="round 3", pile=OK.tw, angle=-2.6, coverage=4, fill=true, clip=true, seed=9900, load=0.95})
print(wait(0))

--@ chunk 119
-- ===== the bank: cover the test strokes and the trunk's flat foot, give the ground some life =====
WAT2 = rect(-5, 487, 1010, 232) - BANK2
TRUNKKEEP = OAK2_MASKS.T * rect(-20, -20, 1050, 660)     -- trunk above y=640 stays
BANKW = BANK2 - TRUNKKEEP

FG.b1 = pile{{"raw umber",3},{"bone black",1.8},{"Prussian blue",0.45},{"green earth",1.2},{"red earth",0.15}}
FG.b2 = pile{{"raw umber",3},{"bone black",1.4},{"green earth",1.6},{"yellow ochre",0.35},{"Prussian blue",0.3}}
FG.b3 = pile{{"raw umber",2.6},{"bone black",2.4},{"Prussian blue",0.6},{"red earth",0.2}}

work(BANKW, {hand="body", tool="filbert 9", pile=FG.b1, angle=0.12, coverage=4.5, fill=true, clip=BANKW, seed=12001, length={30,80}})
-- variation in the field: patches of a greener and a bluer dark
local nb = noise{seed=91, octaves=3, period=120, persistence=0.55}
work(BANKW, {hand="body", tool="filbert 8", pile=FG.b2, angle=0.2, coverage=1.6, fill=false, clip=BANKW, seed=12002, length={20,50},
  load_at=function(x, y) return clamp(0.5 + 0.9 * nb(x, y), 0, 1) * 0.7 end})
work(BANKW, {hand="body", tool="filbert 8", pile=FG.b3, angle=0.1, coverage=1.6, fill=false, clip=BANKW, seed=12003, length={20,50},
  load_at=function(x, y) return clamp(0.5 - 0.9 * nb(x + 300, y), 0, 1) * 0.7 end})
print(wait(0))

--@ chunk 120
-- roots: buttresses diving into the ground at the trunk's foot, each its own drawn shape
ROOTS = {
  {{{234,606},{246,622},{264,640},{290,652},{318,656}}, {17,15,11,6,2}},
  {{{238,626},{252,644},{270,662},{296,672},{326,678}}, {15,13,9,5,2}},
  {{{166,610},{152,626},{136,642},{114,652},{88,656}}, {17,15,11,6,2}},
  {{{162,628},{148,646},{130,664},{106,676},{80,684}}, {15,12,9,5,2}},
  {{{190,640},{186,656},{180,672},{170,690},{160,704}}, {14,12,9,5,2}},
  {{{214,640},{218,656},{226,672},{238,688},{248,704}}, {14,12,9,5,2}},
}
RTP = {}
for i, r in ipairs(ROOTS) do RTP[i] = limb_outline(r[1], r[2], 1.6, 500 + i * 13) end

-- lower trunk repainted in ground colours so its foot dissolves into the bank
LOWT = OAK2_MASKS.T * rect(-20, 570, 1050, 90)
work(LOWT, {hand="body", tool="filbert 8", pile=FG.b3, angle=math.pi/2, coverage=3.5, fill=true, clip=LOWT, seed=12101, length={14,34},
  load_at=function(x, y) return smoothstep(570, 625, y) end})
for i, pp in ipairs(RTP) do
  work(poly(pp), {hand="detail", tool="round 3", pile=(i % 2 == 0) and FG.b3 or FG.b1, angle=math.atan(ROOTS[i][1][5][2]-ROOTS[i][1][1][2], ROOTS[i][1][5][1]-ROOTS[i][1][1][1]),
    coverage=4, fill=true, clip=true, seed=12200 + i, load=0.95})
end
print(wait(0))

--@ chunk 121
-- rim light: warm, broken dashes along the edges that face the low sun (655, 470)
RIM = pile{{"yellow ochre",1.0},{"lead white",1.0},{"red earth",0.35},{"raw umber",0.5}}
SUN = {655, 470}

local function poly_area(pp)
  local a = 0
  for i = 1, #pp do
    local p, q = pp[i], pp[i % #pp + 1]
    a = a + (p[1] * q[2] - q[1] * p[2])
  end
  return a / 2
end

function rim_runs(pp, thr, ymin, ymax)
  local n = #pp
  local sgn = (poly_area(pp) > 0) and 1 or -1   -- y down: positive area = clockwise on screen
  local runs, cur = {}, nil
  for i = 1, n do
    local a, b = pp[(i - 2) % n + 1], pp[i % n + 1]
    local tx, ty = b[1] - a[1], b[2] - a[2]
    local L = math.sqrt(tx*tx + ty*ty); if L < 1e-6 then L = 1 end
    tx, ty = tx / L, ty / L
    -- outward normal
    local nx, ny = sgn * ty, -sgn * tx
    local p = pp[i]
    local sx, sy = SUN[1] - p[1], SUN[2] - p[2]
    local sl = math.sqrt(sx*sx + sy*sy)
    local f = (nx * sx + ny * sy) / sl
    if f > thr and p[2] >= ymin and p[2] <= ymax then
      cur = cur or {}
      cur[#cur+1] = {p[1] - nx * 1.3, p[2] - ny * 1.3, f}
    else
      if cur and #cur > 3 then runs[#runs+1] = cur end
      cur = nil
    end
  end
  if cur and #cur > 3 then runs[#runs+1] = cur end
  return runs
end

function paint_rim(pp, thr, ymin, ymax, w, strength, seed)
  math.randomseed(seed)
  local runs = rim_runs(pp, thr, ymin, ymax)
  local b = brush{kind="round", width=w, point=0.5}
  local count = 0
  for _, run in ipairs(runs) do
    local i = 1
    while i < #run do
      local len = math.floor(rand(5, 14))
      local seg = {}
      for j = i, math.min(#run, i + len) do seg[#seg+1] = {run[j][1], run[j][2]} end
      if #seg >= 3 and rand() < 0.85 then
        local f = run[math.min(#run, i + 2)][3]
        b:load(RIM, rand(0.4, 0.8) * strength)
        local pr = rand(0.35, 0.6)
        b:stroke(seg, {pressure={pr, pr * 0.8}, ramps={0.3, 0.45}})
        count = count + 1
      end
      i = i + len + math.floor(rand(0, 4))
    end
  end
  return count
end

local n = 0
n = n + paint_rim(OAK2_POLY.T, 0.25, 480, 632, 1.8, 1.0, 13001)
n = n + paint_rim(OAK2_POLY.C1, 0.3, 150, 500, 1.6, 0.9, 13002)
n = n + paint_rim(OAK2_POLY.R1, 0.3, 300, 540, 1.6, 0.9, 13003)
n = n + paint_rim(OAK2_POLY.L1, 0.3, 360, 560, 1.6, 0.85, 13004)
n = n + paint_rim(OAK2_POLY.R3, 0.3, 430, 540, 1.4, 0.8, 13005)
print("rim strokes", n)
print(wait(0))

--@ chunk 122
print(wait(7*24*60))
print(drying(200,620), drying(150,400), drying(350,500), drying(500,600))

--@ chunk 123
print(wait(5*24*60))
print(drying(200,620), drying(150,400), drying(350,500), drying(500,600), drying(100,650))

--@ chunk 124
WG = {}
WG.cool  = pile{{"smalt",3},{"cobalt blue",1},{"raw umber",0.35},{"lead white",0.9}, medium=0.65}
WG.edge  = pile{{"raw umber",2},{"bone black",1},{"smalt",1.2},{"Prussian blue",0.2}, medium=0.7}

WAT2 = rect(-5, 489, 1010, 230) - BANK2
local bankd = BANK2:distance()   -- positive inside the bank, negative outside (water)
function colcut(x, y)
  -- 1 in the glow column, falling to 0 outside it; the column widens toward the viewer
  local hw = 60 + 0.5 * math.max(0, y - 489)
  local u = (x - 652) / hw
  return math.exp(-u * u * 1.2)
end
local coolload = function(x, y)
  local t = smoothstep(520, 690, y)
  local base = 0.12 + 0.88 * t ^ 1.2
  return base * (1 - 0.7 * colcut(x, y))
end
work(WAT2, {hand="glaze", pile=WG.cool, angle=0, coverage=2.4, load_at=coolload, clip=WAT2, seed=14001})
print(wait(0))

--@ chunk 125
TREEM = nil
for _, k in ipairs(OAK2_ORDER) do TREEM = TREEM and (TREEM + OAK2_MASKS[k]) or OAK2_MASKS[k] end
TREEM = TREEM:grow(2)
WBL = WAT2 - TREEM
blend(WBL, {angle=0})
blend(WBL, {angle=0.03})
print(wait(0))

--@ chunk 126
-- the trunk between the horizon and the bank crest got veiled by the water glaze: repaint it dark, then model it
TR_LOW = OAK2_MASKS.T * rect(-20, 470, 1050, 190)
OK.trunk = pile{{"raw umber",3},{"bone black",1.8},{"Prussian blue",0.45},{"red earth",0.3}}
work(TR_LOW, {hand="body", tool="filbert 7", pile=OK.trunk, angle=math.pi/2, coverage=5, fill=true, clip=TR_LOW, seed=15001, length={12,34}, load=0.95})
work(TR_LOW, {hand="body", tool="filbert 7", pile=OK.deep, angle=math.pi/2 + 0.1, coverage=3.5, fill=true, clip=TR_LOW, seed=15002, length={12,30}, load=0.9})
print(wait(0))

--@ chunk 127
for i, k in ipairs(OAK2_ORDER) do
  local m = OAK2_MASKS[k] * rect(-20, 484, 1050, 90)
  work(m, {hand="detail", tool="round 3", pile=OK.deep, angle=angfn(k), coverage=4, fill=true, clip=m, seed=15100 + i, length={6,14}, load=0.95})
end
for i, pp in ipairs(STUBP) do
  if STUBS[i][2] > 480 then
    work(poly(pp), {hand="detail", tool="round 3", pile=OK.deep, angle=STUBS[i][3], coverage=4, fill=true, clip=true, seed=15200 + i, load=0.95})
  end
end
print(wait(0))

--@ chunk 128
-- mask of everything of the oak that lies in the band 452..495, so the haze does not veil it
TWM = nil
local cnt = 0
for _, list in ipairs({BR2, BR}) do
  for _, b in ipairs(list) do
    local hit = false
    for _, p in ipairs(b.pts) do if p[2] > 452 and p[2] < 498 and p[1] < 470 then hit = true break end end
    if hit then
      local w = math.max(2.6, b.w0 + 1.6)
      local m = ribbon(b.pts, w)
      TWM = TWM and (TWM + m) or m
      cnt = cnt + 1
    end
  end
end
print("twig ribbons:", cnt)
SHBAND = rect(-5, 452, 1010, 38) - TREEM - TWM
HZ.shore = pile{{"smalt",1.4},{"red earth",0.55},{"raw umber",0.1},{"lead white",4.5},{"chrome yellow",0.25}, medium=0.55}
local hload = function(x, y)
  local t = smoothstep(458, 484, y)
  return 0.35 + 0.55 * t
end
work(SHBAND, {hand="glaze", pile=HZ.shore, angle=0, coverage=1.8, load_at=hload, clip=SHBAND, seed=16001})
print(wait(0))

--@ chunk 129
print(wait(9*24*60))
print(drying(300,470), drying(650,470), drying(200,600), drying(600,600))

--@ chunk 130
-- ribbons for every twig that touches the horizon band, so glazes can be kept off the tree
TWM2 = nil
local cnt = 0
for _, list in ipairs({BR2, BR}) do
  for _, b in ipairs(list) do
    local hit = false
    for _, p in ipairs(b.pts) do if p[2] > 412 and p[2] < 500 and p[1] < 560 then hit = true break end end
    if hit then
      local w = math.max(2.8, b.w0 + 1.8)
      local m = ribbon(b.pts, w)
      TWM2 = TWM2 and (TWM2 + m) or m
      cnt = cnt + 1
    end
  end
end
print("ribbons", cnt)

HZ.fade = pile{{"smalt",1.2},{"red earth",0.5},{"raw umber",0.08},{"lead white",4.5},{"chrome yellow",0.5}, medium=0.6}
STEPFIX = rect(-5, 418, 1010, 38) - TREEM - TWM2
local fixload = function(x, y)
  return 0.38 * smoothstep(420, 452, y)
end
work(STEPFIX, {hand="glaze", pile=HZ.fade, angle=0, coverage=1.8, load_at=fixload, clip=STEPFIX, seed=17001})
print(wait(0))

--@ chunk 131
-- ===== horizon zone, repainted opaque (everything from y=430 to the waterline) =====
HG.gold  = pile{{"lead white",4},{"chrome yellow",1.1},{"vermilion",0.35}, medium=0.06}
HG.mid   = pile{{"lead white",6},{"chrome yellow",0.85},{"vermilion",0.3}, medium=0.06}
HG.cream = pile{{"lead white",8},{"chrome yellow",0.5},{"vermilion",0.22}, medium=0.06}
ZONEM = rect(-5, 400, 1010, 89.4) - TREEM
local z1 = function(x, y) return smoothstep(430, 452, y) end
local z2 = function(x, y) return smoothstep(452, 466, y) end
local z3 = function(x, y) return smoothstep(468, 478, y) end
work(rect(-5, 405, 1010, 60) * ZONEM,  {hand="body", tool="filbert 14", pile=HG.gold,  angle=0, coverage=3.0, fill=true, clip=ZONEM, seed=18001, load_at=z1})
work(rect(-5, 440, 1010, 42) * ZONEM,  {hand="body", tool="filbert 14", pile=HG.mid,   angle=0, coverage=3.0, fill=true, clip=ZONEM, seed=18002, load_at=z2})
work(rect(-5, 458, 1010, 31.4) * ZONEM, {hand="body", tool="filbert 14", pile=HG.cream, angle=0, coverage=3.4, fill=true, clip=ZONEM, seed=18003, load_at=z3})
blend(rect(-5, 420, 1010, 69.4) * ZONEM, {angle=0})
print(wait(0))

--@ chunk 132
print(wait(4*24*60))
print(drying(300,470), drying(650,450), drying(650,430), drying(900,480))

--@ chunk 133
-- ===== far shore, fourth version: on the dry cream, no veil =====
FS.d1 = pile{{"smalt",2},{"raw umber",1.3},{"red earth",0.4},{"Prussian blue",0.15},{"lead white",0.5}}
FS.d2 = pile{{"smalt",2},{"raw umber",1.0},{"red earth",0.45},{"lead white",1.3}}
FS.d3 = pile{{"smalt",2},{"raw umber",0.8},{"red earth",0.5},{"lead white",2.2}}
FS.d4 = pile{{"smalt",2},{"raw umber",0.5},{"red earth",0.55},{"lead white",3.6}}
FS.hills = pile{{"smalt",2},{"red earth",0.55},{"raw umber",0.15},{"lead white",4.2}, medium=0.15}
FS.knoll = pile{{"smalt",2},{"raw umber",0.7},{"red earth",0.5},{"lead white",2.8}}
function pick(x)
  if x < 250 then return FS.d1
  elseif x < 470 then return FS.d1
  elseif x < 620 then return FS.d2
  elseif x < 800 then return FS.d3
  else return FS.d4 end
end
WL = 489.5

-- pale headlands on the right, and a very low one on the left
local hp = {{700,489.4},{735,483},{770,478},{805,474},{845,470.5},{885,468},{925,467},{965,469},{1010,472},{1010,489.4},{700,489.4}}
work(poly(hp, true), {hand="body", tool="filbert 4", pile=FS.hills, angle=0, coverage=3, fill=true, clip=true, seed=19001})
local hp2 = {{-5,486},{40,483.5},{90,482},{140,483.5},{190,482.5},{240,484},{290,485},{290,489.4},{-5,489.4}}
work(poly(hp2, true), {hand="body", tool="filbert 4", pile=FS.hills, angle=0, coverage=2.4, fill=true, clip=true, seed=19002})

-- low land strip at the water line
LANDTOP = outline{{-8,485.7},{60,485.2},{130,486.0},{200,485.4},{280,486.1},{350,485.3},{430,485.9},{520,485.4},{600,486.0},{660,485.6},{740,486.2},{830,485.5},{920,486.1},{1010,485.7},
                  char="soft", seed=2610, amount=0.9, lobe=6}
LANDM = under(LANDTOP, WL)
work(LANDM, {hand="detail", pile=FS.d2, angle=0, coverage=4, fill=true, clip=true, seed=19003})

function crown(cx, w, h, seed)
  math.randomseed(seed)
  local m
  local n = math.random(3, 5)
  for i = 1, n do
    local t = (i - 0.5) / n
    local x = cx + (t - 0.5) * w * 0.85 + rand(-w * 0.06, w * 0.06)
    local hh = h * (0.5 + 0.5 * math.sin(math.pi * (0.12 + 0.76 * t))) * rand(0.8, 1.05)
    local rx = (w / n) * rand(0.8, 1.05)
    local e = ellipse(x, 487.6 - hh * 0.5, rx, hh * 0.5 + 1.4)
    m = m and (m + e) or e
  end
  return m:roughen(0.9, 5, seed)
end
CR = {
  {14,26,6},{44,30,9},{74,22,7},{104,28,10},{134,20,6},
  {176,22,7},{200,26,5},{228,12,3},
  {258,26,7},{286,22,9},{316,24,7},{342,18,5},
  {372,30,8},{398,26,11},{430,24,7},{456,20,5},
  {486,28,7},{516,24,9},{548,20,6},{575,26,5},{602,18,5},
  {690,10,3},
  {760,24,4},{790,28,5},{826,22,4},{860,30,5},{898,26,4},{934,24,3.5},{968,26,4},{996,20,3},
}
for i, c in ipairs(CR) do
  local m = crown(c[1], c[2], c[3], 3000 + i)
  work(m, {hand="detail", pile=pick(c[1]), angle=0.3, coverage=4, fill=true, clip=true, seed=19100 + i})
end
print(wait(0))

--@ chunk 134
FS.e1 = pile{{"raw umber",2},{"smalt",1.6},{"Prussian blue",0.35},{"bone black",0.5},{"green earth",0.4}}
FS.e2 = pile{{"raw umber",1.6},{"smalt",1.8},{"Prussian blue",0.25},{"bone black",0.3},{"lead white",0.5}}
FS.e3 = pile{{"raw umber",1.2},{"smalt",1.8},{"Prussian blue",0.15},{"bone black",0.15},{"lead white",1.3}}
FS.e4 = pile{{"raw umber",0.8},{"smalt",1.8},{"Prussian blue",0.1},{"lead white",2.4}}
function pick2(x)
  if x < 470 then return FS.e1
  elseif x < 620 then return FS.e2
  elseif x < 800 then return FS.e3
  else return FS.e4 end
end
work(LANDM, {hand="body", tool="filbert 4", pile=FS.e2, angle=0, coverage=4, fill=true, clip=true, seed=19201})
for i, c in ipairs(CR) do
  local m = crown(c[1], c[2], c[3], 3000 + i)
  work(m, {hand="body", tool="filbert 4", pile=pick2(c[1]), angle=0.25, coverage=3.5, fill=true, clip=true, seed=19300 + i, load=1.0})
end
print(wait(0))

--@ chunk 135
function poplar(x, h, w, seed)
  math.randomseed(seed)
  local L, R = {}, {}
  local n = 9
  for i = 0, n do
    local t = i / n
    local prof
    if t < 0.35 then prof = 0.55 + 0.45 * (t / 0.35) else prof = (1 - (t - 0.35) / 0.65) ^ 0.85 end
    local hw = w * 0.5 * prof + ((i > 0 and i < n) and rand(-0.25, 0.3) or 0)
    local y = 488.2 - h * t
    L[#L+1] = {x - hw, y}
    R[#R+1] = {x + hw + rand(-0.15, 0.2), y + rand(-0.4, 0.4)}
  end
  local pts = {}
  for i = 1, #L do pts[#pts+1] = L[i] end
  for i = #R, 1, -1 do pts[#pts+1] = R[i] end
  return poly(pts)
end
PP = {{93,15,4.4},{158,22,5.2},{404,27,5.4},{526,18,4.6},{32,13,4.0}}
for i, p in ipairs(PP) do
  work(poplar(p[1], p[2], p[3], 3300 + i), {hand="detail", pile=pick2(p[1]), angle=1.5, coverage=4, fill=true, clip=true, seed=19400 + i})
end
-- church with spire at x=291
work(CHURCH, {hand="detail", pile=FS.e1, angle=1.5, coverage=4, fill=true, clip=true, seed=19410})

-- knoll under the ruin; ruin and firs darker than before, but still pale with distance
KNm = KN - RUIN - FIR1 - FIR2 - FIR3 - FIR4
work(KN, {hand="body", tool="filbert 4", pile=FS.e3, angle=0.1, coverage=3.5, fill=true, clip=true, seed=19420})
FS.ruin2 = pile{{"raw umber",1.0},{"smalt",1.8},{"Prussian blue",0.15},{"bone black",0.15},{"lead white",1.1}}
work(RUIN - LANCET, {hand="detail", pile=FS.ruin2, angle=1.4, coverage=4, fill=true, clip=true, seed=19430})
for i, f in ipairs({FIR1, FIR2, FIR3, FIR4}) do
  work(f, {hand="detail", pile=FS.ruin2, angle=1.5, coverage=4, fill=true, clip=true, seed=19440 + i})
end
print(wait(0))

--@ chunk 136
local list = {}
for _, src in ipairs({BR, BR2}) do
  for _, b in ipairs(src) do
    local hit = false
    for _, p in ipairs(b.pts) do if p[2] >= 392 and p[1] < 570 then hit = true break end end
    if hit then list[#list+1] = b end
  end
end
table.sort(list, function(a, b) return a.w0 > b.w0 end)
for i, b in ipairs(list) do paint_twig(b, OK.tw, 20000 + i) end
-- limbs and stubs that dip into the horizon band: repaint the parts below y=392 again as well
for i, k in ipairs(OAK2_ORDER) do
  local m = OAK2_MASKS[k] * rect(-20, 392, 1050, 110)
  work(m, {hand="detail", tool="round 3", pile=OK.deep, angle=angfn(k), coverage=3.5, fill=true, clip=m, seed=20500 + i, length={6,14}, load=0.95})
end
print("repainted twigs:", #list)
print(wait(0))

--@ chunk 137
print(wait(5*24*60))
print(drying(300,484), drying(650,470), drying(200,485), drying(700,478))

--@ chunk 138
print(wait(4*24*60))
print(drying(200,485), drying(230,560), drying(150,300), drying(300,600))

--@ chunk 139
-- exclusion masks for the sky re-lay: the big forms of the tree (exact) and the far-shore silhouette
TREEX = nil
for _, k in ipairs(OAK2_ORDER) do TREEX = TREEX and (TREEX + OAK2_MASKS[k]) or OAK2_MASKS[k] end
for i, pp in ipairs(STUBP) do TREEX = TREEX + poly(pp) end
TREEX = TREEX + poly(limb_outline({{238,276},{224,270},{212,262},{202,250}}, {13, 10, 8, 5}, 1.2, 411))
local nthick = 0
for _, list in ipairs({BR, BR2}) do
  for _, b in ipairs(list) do
    if b.w0 >= 3.0 then
      TREEX = TREEX + poly(twig_poly(b))
      nthick = nthick + 1
    end
  end
end
print("thick twigs in mask:", nthick)

SHOREX = poly({{700,489.4},{735,483},{770,478},{805,474},{845,470.5},{885,468},{925,467},{965,469},{1010,472},{1010,489.4},{700,489.4}}, true)
SHOREX = SHOREX + poly({{-5,486},{40,483.5},{90,482},{140,483.5},{190,482.5},{240,484},{290,485},{290,489.4},{-5,489.4}}, true)
SHOREX = SHOREX + LANDM + KN + CHURCH + RUIN + FIR1 + FIR2 + FIR3 + FIR4
for i, c in ipairs(CR) do SHOREX = SHOREX + crown(c[1], c[2], c[3], 3000 + i) end
for i, p in ipairs(PP) do SHOREX = SHOREX + poplar(p[1], p[2], p[3], 3300 + i) end
SKYREG = rect(-5, 246, 1010, 243.4) - TREEX - SHOREX:grow(0.7)
print("sky region area:", SKYREG:area())
local rows = {}
for y = 246, 490, 6 do
  local s = ""
  for x = 0, 1000, 8 do s = s .. (SKYREG:at(x, y) > 0.5 and "." or "#") end
  rows[#rows+1] = string.format("%3d ", y) .. s
end
print(table.concat(rows, "\n"))

--@ chunk 140
SR = {}
SR.t0 = pile{{"lead white",5},{"Prussian blue",0.07},{"chrome yellow",0.5}, medium=0.1}
SR.t1 = pile{{"lead white",6},{"chrome yellow",0.75},{"Prussian blue",0.03},{"smalt",0.05}, medium=0.1}
SR.t2 = pile{{"lead white",5},{"chrome yellow",1.0},{"vermilion",0.12}, medium=0.1}
SR.t3 = pile{{"lead white",4.5},{"chrome yellow",1.1},{"vermilion",0.28}, medium=0.1}
SR.t4 = pile{{"lead white",4},{"chrome yellow",1.1},{"vermilion",0.35}, medium=0.06}
local bands = {
  {246, 40, SR.t0, function(x, y) return smoothstep(246, 262, y) end},
  {274, 52, SR.t1, nil},
  {316, 52, SR.t2, nil},
  {358, 50, SR.t3, nil},
  {398, 52, SR.t4, function(x, y) return 1 - smoothstep(430, 450, y) end},
}
for i, b in ipairs(bands) do
  work(rect(-5, b[1], 1010, b[2]) * SKYREG, {hand="broad", pile=b[3], angle=0, coverage=3, fill=true, clip=SKYREG, seed=21000 + i, load_at=b[4]})
end
blend(rect(-5, 246, 1010, 206) * SKYREG, {angle=0})
print(wait(0))

--@ chunk 141
SKYREG2 = rect(-5, 190, 1010, 84) - TREEX
local ramp = function(x, y) return smoothstep(198, 262, y) ^ 1.4 end
work(SKYREG2, {hand="broad", pile=SR.t0, angle=0, coverage=3, fill=true, clip=SKYREG2, seed=21100, load_at=ramp})
blend(rect(-5, 200, 1010, 100) - TREEX, {angle=0})
print(wait(0))

--@ chunk 142
function crest_y(x)
  local e = BANK2_EDGE
  if x <= e[1][1] then return e[1][2] end
  for i = 1, #e - 1 do
    if x <= e[i+1][1] then
      local t = (x - e[i][1]) / (e[i+1][1] - e[i][1])
      return e[i][2] * (1 - t) + e[i+1][2] * t
    end
  end
  return e[#e][2]
end
print(crest_y(0), crest_y(100), crest_y(322), crest_y(500), crest_y(800), crest_y(950))

-- 1. crest light: warm olive dry-brush along the top of the bank, thinning downward
FG.lit2 = pile{{"raw umber",2},{"yellow ochre",1.4},{"green earth",1.2},{"lead white",0.5},{"red earth",0.2}}
local bd = BANK2:distance()
CRESTZ = bd:map(function(v) return smoothstep(-0.5, 1.5, v) * (1 - smoothstep(4, 22, v)) end)
local crestload = function(x, y)
  -- brightest where the crest faces the glow (right of the oak), weaker left, with lumps
  local k = 0.35 + 0.65 * smoothstep(150, 600, x)
  return k * (0.6 + 0.4 * math.sin(x / 23 + 1.3) * math.sin(x / 61))
end
stipple(CRESTZ * BANK2, {pile=FG.lit2, width=2.4, coverage=function(x, y) local d = bd:at(x, y); return 2.0 * (1 - smoothstep(2, 20, d)) * (0.5 + 0.5 * crestload(x, y)) end,
  pressure={0.25, 0.5}, drag={3, 0}, cluster=0.5, seed=23001, dips={12, 0.6, 0.6}})
print(wait(0))

--@ chunk 143
-- tame the crest speckle: work the dark back into it while both are still wet, leaving a warm olive glimmer
work(CRESTZ * BANK2, {hand="body", tool="filbert 5", pile=FG.b1, angle=0.2, coverage=2.6, fill=true, clip=BANK2, seed=23101, length={8,20}, load=0.8})
print(wait(0))

--@ chunk 144
-- ===== grass along the crest: clumps of dark blades, each blade its own curve =====
GB = pile{{"raw umber",2.6},{"bone black",2.2},{"Prussian blue",0.5},{"green earth",0.5}}
GRG = brush{kind="rigger", width=1.3, point=1}
function blade(x, y, len, lean, w, seed_pile, load)
  local mid = {x + lean * len * 0.35 + rand(-0.7, 0.7), y - len * 0.5}
  local tip = {x + lean * len + rand(-0.6, 0.6), y - len}
  GRG:load(seed_pile, load or 0.9)
  local pr = math.min(1, GRG:pressure_for(w) + 0.02)
  GRG:stroke({{x, y + 1.5}, mid, tip}, {pressure={pr, 0.0}, ramps={0, 0}})
end

math.randomseed(31337)
local nclumps = 0
local x = -4
while x < 1004 do
  local cy = crest_y(x)
  local n = math.floor(rand(3, 9))
  local spread = rand(3, 9)
  local maxlen = rand(9, 24)
  -- fewer and shorter on the far left where the bank is high and hides the sky
  if x < 120 then maxlen = maxlen * 0.6 end
  for i = 1, n do
    local bx = x + rand(-spread, spread)
    local by = crest_y(bx) + 0.5
    local len = maxlen * rand(0.35, 1.0)
    local lean = rand(-0.35, 0.35) + 0.12 * math.sin(bx / 90)
    blade(bx, by, len, lean, rand(0.8, 1.5), GB, 0.9)
  end
  nclumps = nclumps + 1
  x = x + rand(12, 46)
end
print("clumps", nclumps)
print(wait(0))

--@ chunk 145
-- ===== reeds at the water's edge on the right; longer, curving, some with seed heads =====
RG = brush{kind="rigger", width=1.6, point=1}
RH = brush{kind="round", width=2.6, point=0.8}
function reed(x, y, len, lean, bend, w, head)
  local pts = {}
  local n = 6
  for i = 0, n do
    local t = i / n
    local px = x + lean * len * t + bend * len * t * t + rand(-0.25, 0.25) * t
    local py = y - len * t
    pts[#pts+1] = {px, py + (i == 0 and 1.5 or 0)}
  end
  RG:load(GB, 0.95)
  local pr = math.min(1, RG:pressure_for(w) + 0.02)
  RG:stroke(pts, {pressure={pr, 0.0}, ramps={0, 0}})
  if head then
    local e, p = pts[#pts], pts[#pts - 2]
    local dx, dy = e[1] - p[1], e[2] - p[2]
    local L = math.sqrt(dx*dx + dy*dy)
    dx, dy = dx / L, dy / L
    local h0 = {e[1] - dx * head, e[2] - dy * head}
    RH:load(GB, 0.9)
    RH:stroke({h0, {e[1] + dx * 1.5, e[2] + dy * 1.5}}, {pressure={0.55, 0.15}, ramps={0.2, 0.6}})
  end
end

math.randomseed(4242)
local x = 738
local cnt = 0
while x < 1004 do
  local dens = 0.35 + 0.65 * smoothstep(738, 900, x)
  local n = math.floor(rand(2, 6) * dens + 0.5)
  local spread = rand(3, 8)
  for i = 1, n do
    local bx = x + rand(-spread, spread)
    local by = crest_y(bx) + 0.5
    local h = rand(22, 58) * (0.55 + 0.45 * dens)
    local lean = rand(-0.22, 0.22) - 0.05
    local bend = rand(-0.28, 0.28)
    reed(bx, by, h, lean, bend, rand(1.2, 1.9), (rand() < 0.42) and rand(5, 8) or nil)
    cnt = cnt + 1
  end
  x = x + rand(8, 26)
end
print("reeds", cnt)
print(wait(0))

--@ chunk 146
-- ===== water: glow column (glaze) and ripple streaks =====
WT.colg = pile{{"lead white",7},{"chrome yellow",0.9},{"vermilion",0.2}, medium=0.5}
local colw = function(y) return 46 + 0.55 * math.max(0, y - 490) end
local function colprofile(x, y)
  local u = (x - 652) / colw(y)
  return math.exp(-u * u * 1.4)
end
WATG = rect(-5, 489.6, 1010, 230) - BANK2
local colload2 = function(x, y)
  local fadeD = 1 - 0.55 * smoothstep(500, 640, y)
  return colprofile(x, y) * fadeD
end
work(WATG, {hand="glaze", pile=WT.colg, angle=0, coverage=1.8, load_at=colload2, clip=WATG, seed=24001})
print(wait(0))

--@ chunk 147
print(wait(10*24*60))
print(drying(300,600), drying(400,640), drying(950,650), drying(300,560), drying(700,600), drying(240,560))

--@ chunk 148
-- ===== water, repainted as opaque bands over the spoiled surface (everything beneath is dry) =====
WATR = rect(-5, 489.4, 1010, 230) - BANK2 - TREEX
WT2 = {}
WT2.w1 = pile{{"lead white",8},{"chrome yellow",0.7},{"vermilion",0.14}, medium=0.05}
WT2.w2 = pile{{"lead white",6},{"chrome yellow",1.0},{"vermilion",0.30},{"smalt",0.06}, medium=0.05}
WT2.w3 = pile{{"lead white",6},{"chrome yellow",0.75},{"Prussian blue",0.03},{"smalt",0.05}, medium=0.05}
WT2.w4 = pile{{"lead white",5},{"Prussian blue",0.07},{"chrome yellow",0.5}, medium=0.05}
WT2.w5 = pile{{"lead white",4},{"Prussian blue",0.10},{"smalt",0.35},{"chrome yellow",0.05}, medium=0.05}
WT2.w6 = pile{{"cobalt blue",1},{"lead white",2.0},{"smalt",1.0},{"raw umber",0.18}, medium=0.05}

local bands = {
  {489.4, 16, WT2.w1, nil},
  {500,   26, WT2.w2, function(x, y) return smoothstep(500, 512, y) end},
  {522,   40, WT2.w3, function(x, y) return smoothstep(522, 536, y) end},
  {556,   44, WT2.w4, function(x, y) return smoothstep(556, 570, y) end},
  {596,   56, WT2.w5, function(x, y) return smoothstep(596, 612, y) end},
  {646,   80, WT2.w6, function(x, y) return smoothstep(646, 666, y) end},
}
for i, b in ipairs(bands) do
  work(rect(-5, b[1], 1010, b[2]) * WATR, {hand="broad", pile=b[3], angle=0, coverage=3, fill=true, clip=WATR, seed=25000 + i, load_at=b[4]})
end
blend(WATR, {angle=0})
print(wait(0))

--@ chunk 149
print(wait(4*24*60))
print(drying(300,520), drying(700,600), drying(600,680), drying(150,485))

--@ chunk 150
WT2.near = pile{{"cobalt blue",1},{"smalt",1.3},{"lead white",1.3},{"raw umber",0.28}, medium=0.08}
WT2.deep = pile{{"smalt",1.4},{"cobalt blue",0.5},{"raw umber",0.4},{"lead white",1.0}, medium=0.08}
function colw2(y) return 40 + 0.62 * math.max(0, y - 490) end
function colshape(x, y)
  local u = (x - 652) / colw2(y)
  return math.exp(-u * u * 1.3)
end
local nearload = function(x, y)
  local t = smoothstep(580, 705, y) ^ 1.15
  return t * (1 - 0.72 * colshape(x, y))
end
local deepload = function(x, y)
  local t = smoothstep(650, 714, y) ^ 1.3
  return t * (1 - 0.6 * colshape(x, y))
end
work(WATR, {hand="broad", pile=WT2.near, angle=0, coverage=3, fill=true, clip=WATR, seed=26001, load_at=nearload})
work(WATR, {hand="broad", pile=WT2.deep, angle=0, coverage=2.4, fill=true, clip=WATR, seed=26002, load_at=deepload})
blend(WATR, {angle=0})
print(wait(0))

--@ chunk 151
-- re-ink the trunk and limbs where the horizon haze had veiled them
BANDX = TREEX * rect(-20, 446, 620, 60)
work(BANDX, {hand="detail", tool="round 3", pile=OK.deep, angle=math.pi/2, coverage=4.5, fill=true, clip=true, seed=27001, length={6,14}, load=0.95})
work(BANDX, {hand="detail", tool="round 2.2", pile=OK.deep, angle=0.2, coverage=3, fill=true, clip=true, seed=27002, length={5,10}, load=0.95})
print(wait(0))

--@ chunk 152
function polyline_at(pts, t)
  -- position and direction at fraction t of a polyline's length
  local L, segs = 0, {}
  for i = 1, #pts - 1 do
    local d = math.sqrt((pts[i+1][1]-pts[i][1])^2 + (pts[i+1][2]-pts[i][2])^2)
    segs[i] = d; L = L + d
  end
  local target = t * L
  local acc = 0
  for i = 1, #pts - 1 do
    if acc + segs[i] >= target or i == #pts - 1 then
      local u = (segs[i] > 0) and (target - acc) / segs[i] or 0
      u = clamp(u, 0, 1)
      local x = pts[i][1] + (pts[i+1][1] - pts[i][1]) * u
      local y = pts[i][2] + (pts[i+1][2] - pts[i][2]) * u
      return x, y, math.atan(pts[i+1][2] - pts[i][2], pts[i+1][1] - pts[i][1])
    end
    acc = acc + segs[i]
  end
end

HAIR = {}
function hair(x, y, ang, len, w0)
  local n = math.max(2, math.floor(len / 5.5))
  local pts = {{x, y}}
  local a = ang
  local cx, cy = x, y
  local sgn = (rand() < 0.5) and -1 or 1
  for i = 1, n do
    a = a + sgn * rand(0.12, 0.42) + 0.05 * angdiff(a, -math.pi/2)
    sgn = -sgn
    local st = len / n * rand(0.8, 1.2)
    cx = cx + math.cos(a) * st
    cy = cy + math.sin(a) * st
    pts[#pts+1] = {cx, cy}
  end
  HAIR[#HAIR+1] = {pts=pts, w0=w0}
  return pts
end
function paint_hair(h, pile_)
  local W = math.max(0.7, h.w0 / 1.3)
  local br = brush{kind="rigger", width=W, point=1}
  br:load(pile_, 1.0)
  local p0 = math.min(1, br:pressure_for(h.w0) + 0.02)
  br:stroke(h.pts, {pressure={p0, 0.0}, ramps={0, 0}})
end

-- generate hairs on twigs of the upper crown first (y < 330), as a test
math.randomseed(555)
HAIR = {}
local cnt = 0
for _, b in ipairs(BR2) do
  local y0 = b.pts[1][2]
  if b.w0 >= 1.0 and b.w0 <= 7 and y0 < 330 then
    local k = (b.w0 > 2.5) and 3 or 2
    for j = 1, k do
      local t = rand(0.35, 0.95)
      local x, y, ta = polyline_at(b.pts, t)
      local sd = (rand() < 0.5) and -1 or 1
      hair(x, y, ta + sd * rand(0.35, 0.95), rand(8, 24) * (0.7 + 0.6 * (1 - t)), rand(0.8, 1.25))
      cnt = cnt + 1
    end
    if b.w0 >= 1.6 then
      local e = b.pts[#b.pts]; local p = b.pts[#b.pts - 1]
      local ta = math.atan(e[2] - p[2], e[1] - p[1])
      for j = 1, 2 do
        hair(e[1], e[2], ta + rand(-0.6, 0.6), rand(6, 15), rand(0.8, 1.1))
        cnt = cnt + 1
      end
    end
  end
end
print("hairs:", cnt)
for i, h in ipairs(HAIR) do paint_hair(h, OK.tw) end
print(wait(0))

--@ chunk 153
math.randomseed(556)
HAIR = {}
local cnt = 0
local function want(b)
  return b.w0 >= 1.0 and b.w0 <= 7 and b.pts[1][2] >= 330 and b.pts[1][2] < 590
end
for _, src in ipairs({BR2, BR}) do
  for _, b in ipairs(src) do
    if want(b) then
      local y0 = b.pts[1][2]
      local k = (b.w0 > 2.5) and 3 or 2
      if rand() < 0.25 then k = k - 1 end
      for j = 1, k do
        local t = rand(0.35, 0.95)
        local x, y, ta = polyline_at(b.pts, t)
        local sd = (rand() < 0.5) and -1 or 1
        hair(x, y, ta + sd * rand(0.35, 0.95), rand(8, 24) * (0.7 + 0.6 * (1 - t)), rand(0.8, 1.25))
        cnt = cnt + 1
      end
      if b.w0 >= 1.6 then
        local e = b.pts[#b.pts]; local p = b.pts[#b.pts - 1]
        local ta = math.atan(e[2] - p[2], e[1] - p[1])
        for j = 1, 2 do
          hair(e[1], e[2], ta + rand(-0.6, 0.6), rand(6, 15), rand(0.8, 1.1))
          cnt = cnt + 1
        end
      end
    end
  end
end
print("hairs:", cnt)
for i, h in ipairs(HAIR) do paint_hair(h, OK.tw) end
print(wait(0))

--@ chunk 154
print(wait(8*24*60))
print(drying(300,520), drying(700,600), drying(600,680), drying(150,485), drying(200,250), drying(230,560))

--@ chunk 155
-- ===== cloud strata on the right (dry sky) =====
CL.body2 = pile{{"smalt",1.6},{"vermilion",0.30},{"yellow ochre",0.25},{"raw umber",0.12},{"lead white",4.2}, medium=0.25}
CL.dusk  = pile{{"smalt",2.0},{"vermilion",0.45},{"raw umber",0.25},{"lead white",3.2}, medium=0.2}
CL.lit   = pile{{"lead white",3},{"vermilion",0.8},{"chrome yellow",0.9}, medium=0.3}

function strat_poly(x0, x1, yc, th, seed, tilt, skew)
  math.randomseed(seed)
  local nz = noise{seed=seed, octaves=3, period=70, persistence=0.55}
  local top, bot = {}, {}
  local n = 24
  for i = 0, n do
    local t = i / n
    local x = lerp(x0, x1, t)
    local prof = math.sin(math.pi * t ^ (skew or 1)) ^ 0.7
    local yy = yc + (tilt or 0) * (t - 0.5)
    local a = th * prof * (0.65 + 0.55 * (nz(x, 3) * 0.5 + 0.5))
    local b = th * prof * (0.55 + 0.55 * (nz(x, 91) * 0.5 + 0.5))
    top[#top+1] = {x, yy - a * 0.62}
    bot[#bot+1] = {x, yy + b * 0.38}
  end
  local pts = {}
  for _, p in ipairs(top) do pts[#pts+1] = p end
  for i = #bot, 1, -1 do pts[#pts+1] = bot[i] end
  return pts, top, bot
end

local pts, top, bot = strat_poly(585, 1010, 322, 15, 71, -5, 0.9)
local o = outline{pts=pts, closed=true, char="soft", seed=71, amount=0.5, lobe=12}
CA = o:mask()
work(CA, {hand="body", tool="filbert 4", pile=CL.body2, angle=0, coverage=2.2, fill=true, edge="soft", seed=28001})
print(wait(0))

--@ chunk 156
blend(CA:grow(5), {angle=0})
print(wait(0))

--@ chunk 157
function strat_cov(x0, x1, yc, th, amp, seed, tilt, asym)
  local nz = noise{seed=seed, octaves=3, period=60, persistence=0.55}
  local nz2 = noise{seed=seed + 5, octaves=2, period=14, persistence=0.5}
  return function(x, y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local tap = math.sin(math.pi * t) ^ 0.6
    local yy = yc + (tilt or 0) * (t - 0.5) + 3 * nz(x, 7)
    local d = (y - yy) / (th * (0.5 + 0.7 * tap) * (0.8 + 0.4 * nz(x, 44)))
    if asym and d > 0 then d = d * asym end      -- sharper underside
    local prof = math.exp(-d * d * 1.6)
    return amp * prof * tap * (0.7 + 0.5 * (nz2(x, y) * 0.5 + 0.5))
  end
end
local cv = strat_cov(575, 1010, 322, 9, 2.6, 81, -5, 1.5)
stipple(rect(560, 290, 460, 70), {pile=CL.body2, width=3.2, coverage=cv, pressure={0.2, 0.45}, drag={16, 0}, cluster=0.35, seed=28101, dips={16, 0.7, 0.5}})
print(wait(0))

--@ chunk 158
-- ===== grass on the crest, second time, on dry paint =====
math.randomseed(90210)
local function hump_len(maxlen, k) return maxlen * (0.45 + 0.55 * math.sin(math.pi * k)) end
local nclumps = 0
local x = 6
while x < 1002 do
  local n = math.floor(rand(5, 12))
  local spread = rand(4, 11)
  local maxlen = rand(8, 26)
  if x < 110 then maxlen = maxlen * 0.55 end
  if x > 300 and x < 420 then maxlen = maxlen * 0.8 end
  for i = 1, n do
    local k = (i - 0.5) / n
    local bx = x + (k - 0.5) * 2 * spread + rand(-1.2, 1.2)
    local by = crest_y(bx) + 0.8
    local len = hump_len(maxlen, k) * rand(0.6, 1.05)
    local lean = 0.10 + rand(-0.32, 0.32) + 0.22 * (k - 0.5)
    blade(bx, by, len, lean, rand(0.9, 1.6), GB, 0.9)
  end
  nclumps = nclumps + 1
  x = x + rand(9, 34)
end
print("clumps", nclumps)

-- tall dry stems with seed heads near the lower right, standing against the pale water
local function stem(x, y, len, lean, bend, w, headlen, seed)
  math.randomseed(seed)
  local pts = {}
  for i = 0, 7 do
    local t = i / 7
    pts[#pts+1] = {x + lean * len * t + bend * len * t * t + rand(-0.2, 0.2) * t, y - len * t + (i == 0 and 2 or 0)}
  end
  RG:load(GB, 0.95)
  local pr = math.min(1, RG:pressure_for(w) + 0.02)
  RG:stroke(pts, {pressure={pr, 0.25}, ramps={0, 0}})
  -- seed head: a slim dark spindle at the top
  local e, p = pts[#pts], pts[#pts - 2]
  local dx, dy = e[1] - p[1], e[2] - p[2]
  local L = math.sqrt(dx*dx + dy*dy); dx, dy = dx / L, dy / L
  RH:load(GB, 0.95)
  RH:stroke({{e[1] - dx * headlen * 0.3, e[2] - dy * headlen * 0.3}, {e[1] + dx * headlen, e[2] + dy * headlen}}, {pressure={0.6, 0.35}, ramps={0.15, 0.5}})
end
stem(506, crest_y(506) + 1, 44, 0.05, -0.10, 1.5, 9, 1)
stem(514, crest_y(514) + 1, 33, -0.12, 0.05, 1.3, 7, 2)
stem(561, crest_y(561) + 1, 52, 0.02, 0.12, 1.6, 11, 3)
stem(570, crest_y(570) + 1, 37, 0.14, -0.05, 1.3, 8, 4)
stem(618, crest_y(618) + 1, 30, -0.06, 0.08, 1.3, 7, 5)
stem(668, crest_y(668) + 1, 40, 0.06, 0.05, 1.4, 9, 6)
print(wait(0))

--@ chunk 159
-- ===== the figure: seen from behind, standing on the crest to the right of the oak =====
FX = 351
FY = crest_y(FX) + 1.6
print("figure feet at", FX, FY)
local function P(dx, dy) return {FX + dx, FY + dy} end
-- coat and body: hand-set outline, feet at y=0, head top at about -46
FIGBODY = poly({
  P(-2.0,-38.6), P(-3.4,-36.9), P(-6.3,-35.6), P(-7.3,-32.0), P(-7.9,-27.0), P(-8.3,-21.0), P(-8.9,-14.5),
  P(-9.7,-8.0),  P(-10.4,-2.6), P(-9.8,0.6),   P(-5.0,1.5),   P(0.6,1.0),    P(5.6,1.5),   P(9.9,0.5),
  P(10.5,-2.6),  P(9.6,-8.5),   P(8.6,-14.5),  P(8.0,-21.0),  P(7.6,-27.0),  P(7.0,-32.2),  P(6.4,-35.4),
  P(3.6,-36.9),  P(2.1,-38.6)
}, true)
FIGHEAD = ellipse(FX + 0.2, FY - 42.3, 2.75, 3.25)
FIGNECK = poly({P(-1.5,-40), P(1.8,-40), P(2.2,-37.5), P(-2.0,-37.5)})
-- a staff in the right hand, planted at the crest
FIGARM = poly({P(6.6,-34.5), P(8.6,-31), P(10.6,-24.5), P(10.9,-21.5), P(9.4,-21.2), P(8.0,-27), P(6.2,-31)}, true)
OK.fig = pile{{"raw umber",2.4},{"bone black",2.4},{"Prussian blue",0.55},{"red earth",0.35}}
for i, m in ipairs({FIGBODY, FIGNECK, FIGHEAD, FIGARM}) do
  work(m, {hand="detail", tool="round 2.2", pile=OK.fig, angle=math.pi/2, coverage=5, fill=true, clip=true, seed=29000 + i, length={4, 9}, load=0.95})
end
-- staff
local sb = brush{kind="rigger", width=1.2, point=1}
sb:load(OK.fig, 1.0)
sb:stroke({{FX + 11.6, FY + 2.2}, {FX + 11.3, FY - 14}, {FX + 10.9, FY - 30}, {FX + 10.7, FY - 41}}, {pressure={0.75, 0.55}, ramps={0, 0.15}})
print(wait(0))

--@ chunk 160
GZ.sky1 = pile{{"cobalt blue",1},{"lead white",3.2},{"smalt",0.6},{"Prussian blue",0.02}, medium=0.6}
local m = rect(400, 168, 610, 92)
local fixl = function(x, y)
  return smoothstep(168, 186, y) * (1 - smoothstep(214, 252, y)) * (0.55 + 0.45 * smoothstep(400, 560, x))
end
work(m, {hand="glaze", pile=GZ.sky1, angle=0, coverage=2.0, load_at=fixl, clip=m, seed=30001})
blend(m, {angle=0})
print(wait(0))

--@ chunk 161
SR.fix = pile{{"lead white",6},{"chrome yellow",0.7},{"Prussian blue",0.03},{"smalt",0.04}, medium=0.6}
local fm = rect(396, 240, 620, 56)
local floadf = function(x, y)
  return smoothstep(238, 258, y) * (1 - smoothstep(268, 292, y)) * (0.5 + 0.5 * smoothstep(400, 560, x))
end
work(fm, {hand="glaze", pile=SR.fix, angle=0, coverage=1.6, load_at=floadf, clip=fm, seed=30002})
local softm = rect(400, 226, 610, 90):blur(20)
blend(softm, {angle=0})
print(softm:at(700, 226), softm:at(700, 250), softm:at(700, 271), softm:at(700, 310))
print(wait(0))

--@ chunk 162
-- remove the pill-shaped cloud: cover it with the local sky mixtures, feathered at every side
local R = rect(560, 296, 452, 54)
local lx = function(x, y)
  local ex = smoothstep(560, 610, x)
  return ex
end
local ly1 = function(x, y) return lx(x, y) * smoothstep(296, 306, y) * (1 - smoothstep(322, 336, y)) end
local ly2 = function(x, y) return lx(x, y) * smoothstep(318, 330, y) * (1 - smoothstep(340, 352, y)) end
work(R, {hand="body", tool="filbert 10", pile=SR.t2, angle=0, coverage=3.6, fill=true, clip=R, seed=30101, load_at=ly1})
work(R, {hand="body", tool="filbert 10", pile=SR.t3, angle=0, coverage=3.6, fill=true, clip=R, seed=30102, load_at=ly2})
local sm = rect(540, 284, 480, 84):blur(18)
blend(sm, {angle=0})
print(wait(0))

--@ chunk 163
-- soften the left edge of the warm patch with a feathered glaze of the neighbouring lemon, then blend in a soft mask
SR.edge = pile{{"lead white",6},{"chrome yellow",0.6},{"Prussian blue",0.03},{"smalt",0.05}, medium=0.55}
local em = rect(520, 286, 120, 90)
local eload = function(x, y)
  local a = smoothstep(548, 566, x) * (1 - smoothstep(590, 632, x))
  local b = smoothstep(290, 306, y) * (1 - smoothstep(350, 372, y))
  return a * b
end
work(em, {hand="glaze", pile=SR.edge, angle=0, coverage=1.6, load_at=eload, clip=em, seed=30201})
blend(rect(500, 276, 170, 110):blur(20), {angle=0})
print(wait(0))

--@ chunk 164
-- regenerate the first hair set (seed 555, upper crown) without painting it, so it can be re-inked where needed
HAIR_KEEP = HAIR
math.randomseed(555)
HAIR = {}
for _, b in ipairs(BR2) do
  local y0 = b.pts[1][2]
  if b.w0 >= 1.0 and b.w0 <= 7 and y0 < 330 then
    local k = (b.w0 > 2.5) and 3 or 2
    for j = 1, k do
      local t = rand(0.35, 0.95)
      local x, y, ta = polyline_at(b.pts, t)
      local sd = (rand() < 0.5) and -1 or 1
      hair(x, y, ta + sd * rand(0.35, 0.95), rand(8, 24) * (0.7 + 0.6 * (1 - t)), rand(0.8, 1.25))
    end
    if b.w0 >= 1.6 then
      local e = b.pts[#b.pts]; local p = b.pts[#b.pts - 1]
      local ta = math.atan(e[2] - p[2], e[1] - p[1])
      for j = 1, 2 do
        hair(e[1], e[2], ta + rand(-0.6, 0.6), rand(6, 15), rand(0.8, 1.1))
      end
    end
  end
end
HAIR_OLD = HAIR
HAIR = HAIR_KEEP
print("old hairs regenerated:", #HAIR_OLD, " current set:", #HAIR)

-- region to re-ink
RX0, RX1, RY0, RY1 = 380, 570, 150, 450
local function inreg(pts)
  for _, p in ipairs(pts) do
    if p[1] >= RX0 and p[1] <= RX1 and p[2] >= RY0 and p[2] <= RY1 then return true end
  end
  return false
end
local REG = rect(RX0, RY0, RX1 - RX0, RY1 - RY0)
-- limbs
for i, k in ipairs({"R1","R2","R4","R5"}) do
  local m = OAK2_MASKS[k] * REG
  work(m, {hand="detail", tool="round 3", pile=OK.deep, angle=angfn(k), coverage=4, fill=true, clip=m, seed=31000 + i, length={6,14}, load=0.95})
end
-- twigs
local list = {}
for _, src in ipairs({BR, BR2}) do
  for _, b in ipairs(src) do if inreg(b.pts) then list[#list+1] = b end end
end
table.sort(list, function(a, b) return a.w0 > b.w0 end)
for i, b in ipairs(list) do paint_twig(b, OK.tw, 31100 + i) end
-- hairs
local nh = 0
for _, src in ipairs({HAIR_OLD, HAIR}) do
  for _, h in ipairs(src) do
    if inreg(h.pts) then paint_hair(h, OK.tw); nh = nh + 1 end
  end
end
print("twigs:", #list, "hairs:", nh)
print(wait(0))

--@ chunk 165
-- occupancy grid of every branch, twig and hair (1-unit cells), for masks that must keep off the tree
GX, GY = 1010, 720
TWG = {}
for i = 1, GX do TWG[i] = {} end
local function stamp(x, y, r)
  local x0, x1 = math.floor(x - r), math.ceil(x + r)
  local y0, y1 = math.floor(y - r), math.ceil(y + r)
  for cx = x0, x1 do
    if cx >= 0 and cx < GX then
      local col = TWG[cx + 1]
      for cy = y0, y1 do
        if cy >= 0 and cy < GY then
          local dx, dy = cx + 0.5 - x, cy + 0.5 - y
          if dx*dx + dy*dy <= (r + 0.6)^2 then col[cy + 1] = 1 end
        end
      end
    end
  end
end
local function stamp_path(pts, r0, r1)
  for i = 1, #pts - 1 do
    local p, q = pts[i], pts[i+1]
    local d = math.sqrt((q[1]-p[1])^2 + (q[2]-p[2])^2)
    local n = math.max(1, math.ceil(d / 0.7))
    for j = 0, n do
      local t = j / n
      local f = ((i - 1) + t) / (#pts - 1)
      stamp(p[1] + (q[1]-p[1]) * t, p[2] + (q[2]-p[2]) * t, r0 + (r1 - r0) * f)
    end
  end
end
local nn = 0
for _, src in ipairs({BR, BR2}) do
  for _, b in ipairs(src) do stamp_path(b.pts, math.max(0.9, b.w0 * 0.5 + 0.5), 0.9); nn = nn + 1 end
end
for _, src in ipairs({HAIR_OLD, HAIR}) do
  for _, h in ipairs(src) do stamp_path(h.pts, 1.0, 0.8); nn = nn + 1 end
end
TWALLM = mask(function(x, y)
  local i, j = math.floor(x) + 1, math.floor(y) + 1
  if i < 1 or i > GX or j < 1 or j > GY then return 0 end
  return TWG[i][j] or 0
end)
TREEALL = TREEX + TWALLM
print("stamped", nn, "twig area", TWALLM:area())

--@ chunk 166
GZ.sky1 = pile{{"cobalt blue",1},{"lead white",3.2},{"smalt",0.6},{"Prussian blue",0.02}, medium=0.6}
local SKYFIX = rect(230, 160, 200, 110) - TREEALL
local fixl2 = function(x, y)
  local ex = smoothstep(236, 404, x)
  return ex * smoothstep(168, 186, y) * (1 - smoothstep(214, 252, y)) * 0.75
end
work(SKYFIX, {hand="glaze", pile=GZ.sky1, angle=0, coverage=2.0, load_at=fixl2, clip=SKYFIX, seed=32001})
print(wait(0))

--@ chunk 167
-- re-lay the blue -> lemon transition across x=200..1010, y=105..305 (tree gets re-inked afterwards)
SKB = rect(200, 105, 812, 200)
SK2.c2 = pile{{"cobalt blue",1},{"lead white",3.2},{"smalt",0.55}, medium=0.1}
SK2.d2 = pile{{"lead white",4},{"Prussian blue",0.10},{"smalt",0.3},{"chrome yellow",0.06}, medium=0.1}
local xin = function(x) return smoothstep(200, 262, x) end
local L1 = function(x, y) return xin(x) * smoothstep(105, 150, y) * (1 - smoothstep(196, 226, y)) end
local L2 = function(x, y) return xin(x) * smoothstep(176, 208, y) * (1 - smoothstep(236, 262, y)) end
local L3 = function(x, y) return xin(x) * smoothstep(222, 246, y) * (1 - smoothstep(270, 296, y)) end
local L4 = function(x, y) return xin(x) * smoothstep(262, 282, y) * (1 - smoothstep(292, 304, y)) end
work(SKB, {hand="broad", pile=SK2.c2, angle=0, coverage=3, fill=true, clip=SKB, seed=33001, load_at=L1})
work(SKB, {hand="broad", pile=SK2.d2, angle=0, coverage=3, fill=true, clip=SKB, seed=33002, load_at=L2})
work(SKB, {hand="broad", pile=SR.t0,  angle=0, coverage=3, fill=true, clip=SKB, seed=33003, load_at=L3})
work(SKB, {hand="broad", pile=SR.t1,  angle=0, coverage=3, fill=true, clip=SKB, seed=33004, load_at=L4})
blend(SKB:shrink(6):blur(14), {angle=0})
print(wait(0))

--@ chunk 168
print(wait(8*24*60))
print(drying(700,200), drying(400,290), drying(300,250), drying(600,150), drying(250,700))

--@ chunk 169
-- ===== the sky again, full width, y 40..350; the tree is re-inked afterwards =====
local AN = function(x, y) return -0.02 + 0.018 * math.sin(x / 170 + y / 60) end
local bands = {
  {36,  72, SK2.a, function(x, y) return smoothstep(38, 90, y) end},
  {84,  86, SK2.b, nil},
  {150, 80, SK2.c, nil},
  {206, 60, SK2.d, nil},
  {246, 54, SR.t0, nil},
  {280, 54, SR.t1, nil},
  {316, 40, SR.t2, function(x, y) return 1 - smoothstep(332, 350, y) end},
}
for i, b in ipairs(bands) do
  work(rect(-5, b[1], 1010, b[2]), {hand="broad", pile=b[3], angle=AN, coverage=3, fill=true, seed=34000 + i, load_at=b[4]})
end
blend(rect(-5, 40, 1010, 312), {angle=0})
print(wait(0))

--@ chunk 170
-- ===== re-ink the tree above y=372 =====
YCUT = 372
UPPER = rect(-20, 20, 1060, YCUT - 20)
for i, k in ipairs(OAK2_ORDER) do
  local m = OAK2_MASKS[k] * UPPER
  local big = (OAK2_W[k][1] > 30)
  work(m, {hand="detail", tool=big and "filbert 7" or "round 3", pile=OK.deep, angle=angfn(k),
    coverage=big and 4.5 or 4, fill=true, clip=m, seed=36000 + i, length=big and {14, 34} or {6, 14}, load=0.95})
end
for i, pp in ipairs(STUBP) do
  work(poly(pp) * UPPER, {hand="detail", tool="round 3", pile=OK.deep, angle=STUBS[i][3], coverage=4, fill=true, clip=true, seed=36100 + i, load=0.95})
end
local sp = {{238,276},{224,270},{212,262},{202,250}}
work(poly(limb_outline(sp, {13, 10, 8, 5}, 1.2, 411)), {hand="detail", tool="round 3", pile=OK.deep, angle=-2.6, coverage=4, fill=true, clip=true, seed=36200, load=0.95})

local function above_cut(pts)
  for _, p in ipairs(pts) do if p[2] < YCUT then return true end end
  return false
end
local list = {}
for _, src in ipairs({BR, BR2}) do
  for _, b in ipairs(src) do if above_cut(b.pts) then list[#list+1] = b end end
end
table.sort(list, function(a, b) return a.w0 > b.w0 end)
for i, b in ipairs(list) do paint_twig(b, OK.tw, 37000 + i) end
local nh = 0
for _, src in ipairs({HAIR_OLD, HAIR}) do
  for _, h in ipairs(src) do
    if above_cut(h.pts) then paint_hair(h, OK.tw); nh = nh + 1 end
  end
end
print("twigs:", #list, "hairs:", nh)
print(wait(0))

--@ chunk 171
function above_cut(pts)
  for _, p in ipairs(pts) do if p[2] < YCUT then return true end end
  return false
end
OK.jet = pile{{"raw umber",2.2},{"bone black",2.6},{"Prussian blue",0.6},{"red earth",0.2}}
for i, k in ipairs(OAK2_ORDER) do
  local m = OAK2_MASKS[k] * UPPER
  local big = (OAK2_W[k][1] > 30)
  work(m, {hand="detail", tool=big and "filbert 6" or "round 2.2", pile=OK.jet, angle=angfn(k),
    coverage=big and 4 or 3.5, fill=true, clip=m, seed=38000 + i, length=big and {10, 26} or {5, 10}, load=1.0, dips={2, 1.0, 0.6}})
end
for i, pp in ipairs(STUBP) do
  work(poly(pp) * UPPER, {hand="detail", tool="round 2.2", pile=OK.jet, angle=STUBS[i][3], coverage=3.5, fill=true, clip=true, seed=38100 + i, load=1.0, dips={2, 1.0, 0.6}})
end
local list2 = {}
for _, src in ipairs({BR, BR2}) do
  for _, b in ipairs(src) do if b.w0 >= 2.2 and above_cut(b.pts) then list2[#list2+1] = b end end
end
table.sort(list2, function(a, b) return a.w0 > b.w0 end)
for i, b in ipairs(list2) do
  local m = poly(twig_poly(b))
  work(m, {hand="detail", tool="round 1.8", pile=OK.jet, angle=math.atan(b.pts[#b.pts][2]-b.pts[1][2], b.pts[#b.pts][1]-b.pts[1][1]),
    coverage=3, fill=true, clip=true, seed=38500 + i, load=1.0, dips={3, 1.0, 0.6}})
end
print("second-pass twigs:", #list2)
print(wait(0))

--@ chunk 172
print(drying(520, 656), drying(351, 590), drying(300, 620), drying(700, 700), drying(200, 200), drying(500, 330))
print(wait(0))

--@ chunk 173
print(wait(5*24*60))
print(drying(500,330), drying(200,200), drying(700,250), drying(300,600), drying(650,600))

--@ chunk 174
print(wait(4*24*60))
print(drying(500,330), drying(500,300), drying(500,350), drying(700,250), drying(300,600))

--@ chunk 175
for _, p in ipairs({{650,440},{650,470},{800,400},{300,340},{500,340},{500,320},{900,300},{700,520},{700,620}}) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 176
print(wait(3*24*60))
for _, p in ipairs({{500,340},{500,320},{650,440}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 177
GL = {}
GL.warm = pile{{"lead white",9},{"chrome yellow",0.6},{"vermilion",0.07}, medium=0.35}
GL.core = pile{{"lead white",10},{"chrome yellow",0.22},{"vermilion",0.02}, medium=0.3}

SKYFREE = rect(-5, 372, 1010, 118) - SHOREX:grow(0.8) - TREEALL
GLM1 = ellipse(655, 470, 350, 92) * SKYFREE
work(GLM1, {hand="glaze", pile=GL.warm, angle=0, coverage=2.2, clip=SKYFREE, seed=40001,
  load_at=radial(655, 472, 350, 92, 1.0, 1.0)})
blend(GLM1:blur(10), {angle=0})
print(wait(0))

--@ chunk 178
FS.ruin3 = pile{{"smalt",2},{"raw umber",0.9},{"Prussian blue",0.2},{"bone black",0.25},{"lead white",0.55}}
FS.knoll3 = pile{{"smalt",2},{"raw umber",0.7},{"red earth",0.35},{"Prussian blue",0.1},{"lead white",1.2}}
local fade_base = function(x, y) return 1.0 - 0.45 * smoothstep(476, 488, y) end
work(KN - RUIN - FIR1 - FIR2 - FIR3 - FIR4, {hand="detail", pile=FS.knoll3, angle=0.1, coverage=3.5, fill=true, clip=true, seed=41001, load_at=fade_base})
work(RUIN - LANCET, {hand="detail", pile=FS.ruin3, angle=1.4, coverage=4, fill=true, clip=true, seed=41002, load_at=fade_base})
for i, f in ipairs({FIR1, FIR2, FIR3, FIR4}) do
  work(f, {hand="detail", pile=FS.ruin3, angle=1.5, coverage=4, fill=true, clip=true, seed=41010 + i, load_at=fade_base})
end
print(wait(0))

--@ chunk 179
CL.body3 = pile{{"smalt",1.7},{"vermilion",0.32},{"yellow ochre",0.2},{"raw umber",0.14},{"lead white",3.6}, medium=0.25}
CL.lit3  = pile{{"lead white",3},{"vermilion",0.75},{"chrome yellow",1.0}, medium=0.3}
SKYCLOUD = rect(430, 372, 585, 118) - SHOREX:grow(1) - TREEALL

function stratum(x0, x1, yc, th, tilt, seed, amp, bodyp, litp)
  local nz = noise{seed=seed, octaves=3, period=55, persistence=0.55}
  local nz2 = noise{seed=seed + 9, octaves=2, period=12, persistence=0.5}
  local function yline(x, t) return yc + tilt * (t - 0.5) + 2.5 * nz(x, 7) end
  local function tapf(t) return math.sin(math.pi * clamp(t, 0, 1)) ^ 0.55 end
  local body = function(x, y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local ty = th * (0.5 + 0.7 * tapf(t)) * (0.8 + 0.4 * nz(x, 44))
    local d = (y - (yline(x, t) - 0.18 * ty)) / (0.5 * ty)
    return amp * math.exp(-d * d * 1.7) * tapf(t) * (0.7 + 0.5 * (nz2(x, y) * 0.5 + 0.5))
  end
  local lit = function(x, y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local ty = th * (0.5 + 0.7 * tapf(t)) * (0.8 + 0.4 * nz(x, 44))
    local d = (y - (yline(x, t) + 0.42 * ty)) / (0.3 * ty)
    return 0.85 * amp * math.exp(-d * d * 1.7) * tapf(t) * (0.6 + 0.6 * (nz2(x + 40, y) * 0.5 + 0.5))
  end
  local band = rect(x0 - 10, yc - th * 1.6 - 6, x1 - x0 + 20, th * 3.2 + 12) * SKYCLOUD
  stipple(band, {pile=bodyp, width=3.0, coverage=body, pressure={0.22, 0.45}, drag={14, 0}, cluster=0.3, seed=seed, dips={16, 0.7, 0.5}, clip=SKYCLOUD})
  stipple(band, {pile=litp,  width=2.6, coverage=lit,  pressure={0.2, 0.42},  drag={12, 0}, cluster=0.3, seed=seed + 1, dips={16, 0.7, 0.5}, clip=SKYCLOUD})
  return band
end
SA = stratum(560, 985, 412, 8, -4, 42001, 2.2, CL.body3, CL.lit3)
SB = stratum(470, 900, 437, 9, 3, 42003, 2.4, CL.body3, CL.lit3)
SC = stratum(690, 1010, 458, 6, -2, 42005, 2.4, CL.body3, CL.lit3)
print(wait(0))

--@ chunk 180
-- 1. zenith blemish: a feathered veil of the zenith blue over it
local zm = rect(600, 28, 90, 44)
local zl = function(x, y)
  return smoothstep(602, 620, x) * (1 - smoothstep(654, 684, x)) * smoothstep(30, 40, y) * (1 - smoothstep(52, 68, y))
end
SK2.aa = pile{{"smalt",3},{"cobalt blue",1},{"Prussian blue",0.1},{"lead white",1.3}, medium=0.5}
work(zm, {hand="glaze", pile=SK2.aa, angle=0, coverage=1.8, load_at=zl, clip=zm, seed=43001})

-- 2. water: exclusions and the glow-column shape
WATR2 = rect(-5, 489.4, 1010, 230) - BANK2 - TREEALL
function colshape2(x, y)
  local hw = 34 + 0.55 * math.max(0, y - 489)
  local u = (x - 652) / hw
  return math.exp(-u * u * 1.25)
end

-- 3. patch the teal smear (x 845-1005, y 508-552) with body colour of the local water, feathered
WT2.p1 = pile{{"lead white",6},{"chrome yellow",0.75},{"Prussian blue",0.03},{"smalt",0.05}, medium=0.05}
WT2.p2 = pile{{"lead white",5},{"Prussian blue",0.07},{"chrome yellow",0.5}, medium=0.05}
local pm = rect(838, 504, 172, 52) * WATR2
local pl1 = function(x, y) return smoothstep(840, 862, x) * smoothstep(506, 516, y) * (1 - smoothstep(526, 536, y)) end
local pl2 = function(x, y) return smoothstep(840, 862, x) * smoothstep(524, 534, y) * (1 - smoothstep(546, 556, y)) end
work(pm, {hand="body", tool="filbert 10", pile=WT2.p1, angle=0, coverage=3, fill=true, clip=WATR2, seed=43002, load_at=pl1})
work(pm, {hand="body", tool="filbert 10", pile=WT2.p2, angle=0, coverage=3, fill=true, clip=WATR2, seed=43003, load_at=pl2})
print(wait(0))

--@ chunk 181
print(wait(5*24*60))
for _, p in ipairs({{900,520},{600,420},{700,440},{830,415},{984,579}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 182
-- water: cool deepening toward the viewer, kept out of the glow column; then the column itself
WG.cool2 = pile{{"smalt",3},{"cobalt blue",1},{"raw umber",0.5},{"lead white",0.5}, medium=0.7}
local cload = function(x, y)
  local t = smoothstep(540, 705, y) ^ 1.05
  return t * (1 - 0.8 * colshape2(x, y))
end
work(WATR2, {hand="glaze", pile=WG.cool2, angle=0, coverage=2.6, load_at=cload, clip=WATR2, seed=44001})
WT.colg2 = pile{{"lead white",8},{"chrome yellow",0.7},{"vermilion",0.12}, medium=0.45}
local colload3 = function(x, y)
  local d = smoothstep(489, 498, y) * (1 - 0.35 * smoothstep(520, 700, y))
  return colshape2(x, y) * d
end
work(WATR2, {hand="glaze", pile=WT.colg2, angle=0, coverage=2.0, load_at=colload3, clip=WATR2, seed=44002})
print(wait(0))

--@ chunk 183
print(wait(6*24*60))
for _, p in ipairs({{700,600},{800,680},{652,520},{500,620},{484,590},{984,579}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 184
WATR3 = rect(-5, 489.4, 1010, 230) - BANK2 - TREEX
WN = {}
WN.w1 = pile{{"lead white",8},{"chrome yellow",0.6},{"vermilion",0.12}, medium=0.05}
WN.w2 = pile{{"lead white",6},{"chrome yellow",0.8},{"vermilion",0.22},{"smalt",0.06}, medium=0.05}
WN.w3 = pile{{"lead white",6},{"chrome yellow",0.5},{"Prussian blue",0.04},{"smalt",0.12}, medium=0.05}
WN.w4 = pile{{"lead white",5},{"Prussian blue",0.09},{"smalt",0.35},{"chrome yellow",0.12}, medium=0.05}
WN.w5 = pile{{"lead white",3},{"smalt",1.2},{"cobalt blue",0.6},{"raw umber",0.2}, medium=0.05}
WN.w6 = pile{{"lead white",1.6},{"smalt",1.6},{"cobalt blue",0.5},{"raw umber",0.5}, medium=0.05}
local bands = {
  {489.4, 18, WN.w1, nil},
  {500,   36, WN.w2, function(x, y) return smoothstep(500, 512, y) end},
  {528,   50, WN.w3, function(x, y) return smoothstep(528, 542, y) end},
  {566,   56, WN.w4, function(x, y) return smoothstep(566, 582, y) end},
  {606,   64, WN.w5, function(x, y) return smoothstep(606, 626, y) end},
  {650,   70, WN.w6, function(x, y) return smoothstep(650, 672, y) end},
}
for i, b in ipairs(bands) do
  work(rect(-5, b[1], 1010, b[2]) * WATR3, {hand="broad", pile=b[3], angle=0, coverage=3, fill=true, clip=WATR3, seed=45000 + i, load_at=b[4]})
end
function colshape3(x, y)
  local hw = 26 + 0.36 * math.max(0, y - 489)
  local u = (x - 652) / hw
  return math.exp(-u * u * 1.4)
end
WN.col = pile{{"lead white",8},{"chrome yellow",0.75},{"vermilion",0.14}, medium=0.05}
local cl = function(x, y)
  local s = colshape3(x, y)
  return smoothstep(0.2, 0.8, s) * (1 - 0.3 * smoothstep(560, 714, y))
end
work(WATR3, {hand="body", tool="filbert 12", pile=WN.col, angle=0, coverage=3.2, fill=true, clip=WATR3, seed=45010, load_at=cl})
blend(WATR3, {angle=0})
print(wait(0))

--@ chunk 185
print(wait(7*24*60))
for _, p in ipairs({{700,600},{800,680},{652,520},{500,620},{484,590},{350,575},{330,500}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 186
-- ===== water, definitive re-lay: full-load overlapping bands on dry ground; seams blended one at a time =====
WATR4 = rect(-5, 489.4, 1010, 232) - BANK2 - TREEX
WN.b1 = pile{{"lead white",8},{"chrome yellow",0.6},{"vermilion",0.12}, medium=0.05}
WN.b2 = pile{{"lead white",6},{"chrome yellow",0.85},{"vermilion",0.24},{"smalt",0.05}, medium=0.05}
WN.b3 = pile{{"lead white",6},{"chrome yellow",0.6},{"Prussian blue",0.03},{"smalt",0.10}, medium=0.05}
WN.b4 = pile{{"lead white",5},{"smalt",0.9},{"cobalt blue",0.3},{"chrome yellow",0.15}, medium=0.05}
WN.b5 = pile{{"lead white",3},{"smalt",1.3},{"cobalt blue",0.6},{"raw umber",0.2}, medium=0.05}
WN.b6 = pile{{"lead white",1.6},{"smalt",1.6},{"cobalt blue",0.6},{"raw umber",0.5}, medium=0.05}
local bands = {
  {489.4, 22, WN.b1},
  {503,   34, WN.b2},
  {528,   44, WN.b3},
  {562,   50, WN.b4},
  {602,   60, WN.b5},
  {652,   72, WN.b6},
}
for i, b in ipairs(bands) do
  work(rect(-5, b[1], 1010, b[2]) * WATR4, {hand="broad", pile=b[3], angle=0, coverage=3.4, fill=true, clip=WATR4, seed=46000 + i})
end
-- seams, one at a time, with soft-edged strips so the blender stops gently
local seams = {503, 528, 562, 602, 652}
for _, s in ipairs(seams) do
  blend(rect(-5, s - 12, 1010, 26):blur(6) * WATR4, {angle=0})
end
print(wait(0))

--@ chunk 187
-- ===== birds and the evening star (sky is dry) =====
BIRD = pile{{"raw umber",2},{"bone black",2},{"Prussian blue",0.5},{"lead white",0.5}}
function bird(cx, cy, s, k, tilt, w, seed)
  math.randomseed(seed)
  local function wing(sd)
    local ca, sa = math.cos(tilt), math.sin(tilt)
    local function rot(dx, dy) return {cx + dx * ca - dy * sa, cy + dx * sa + dy * ca} end
    local mid_y = -s * (0.05 + 0.055 * (k + 1)) + rand(-0.4, 0.4)
    local tip_y = s * (0.11 - 0.185 * (k + 1) * 0.5 * 1.35) + rand(-0.4, 0.4)
    local tip_x = sd * s * (0.5 - 0.06 * math.max(0, k)) * rand(0.94, 1.06)
    local pts = {rot(sd * 0.7, 0.2), rot(sd * s * 0.22, mid_y * 0.85), rot(sd * s * 0.38, mid_y * 0.72 + (tip_y - mid_y) * 0.25), rot(tip_x, tip_y)}
    return pts
  end
  local br = brush{kind="rigger", width=w, point=1}
  for _, sd in ipairs({-1, 1}) do
    br:load(BIRD, 0.9)
    br:stroke(wing(sd), {pressure={math.min(1, br:pressure_for(w * 1.05) + 0.05), 0.0}, ramps={0, 0}})
  end
  -- body: a tiny dash
  local bb = brush{kind="round", width=w * 1.15, point=0.8}
  bb:load(BIRD, 0.9)
  local ca, sa = math.cos(tilt), math.sin(tilt)
  bb:stroke({{cx - 1.1 * ca, cy - 1.1 * sa + 0.3}, {cx + 1.3 * ca, cy + 1.3 * sa + 0.4}}, {pressure={0.55, 0.45}, ramps={0.2, 0.4}})
end
bird(744, 356, 14.5, 0.75, -0.06, 1.3, 1)
bird(768, 344, 13.0, -0.2, 0.04, 1.25, 2)
bird(783, 366, 11.5, 0.9, -0.03, 1.2, 3)
bird(808, 351, 12.0, 0.1, 0.05, 1.2, 4)
bird(826, 336, 9.5, -0.75, -0.02, 1.0, 5)
bird(853, 358, 9.0, 0.6, 0.03, 1.0, 6)
bird(874, 344, 7.5, 0.0, -0.05, 0.9, 7)

-- the evening star: a point and a faint halo
STAR = brush{kind="round", width=2.2, point=0.7}
STARP = pile{{"lead white",8},{"chrome yellow",0.15}}
STAR:load(STARP, 1.0)
STAR:touch(690, 148, {pressure=0.5})
local halo = ellipse(690, 148, 10, 10)
stipple(halo, {pile=pile{{"lead white",6},{"cobalt blue",0.25}, medium=0.4}, width=2.6, coverage=function(x, y)
  local r = math.sqrt((x - 690)^2 + (y - 148)^2)
  return 1.2 * (1 - smoothstep(1.5, 9.5, r))
end, pressure={0.2, 0.4}, seed=47001})
print(wait(0))

--@ chunk 188
-- soften the star's sparkle into a small halo: blend only its own fresh paint, then a small bright core
local sm = ellipse(690, 148, 13, 13):blur(4)
blend(sm, {angle=0.6})
local core = brush{kind="round", width=1.6, point=0.7}
core:load(STARP, 1.0)
core:touch(690, 148, {pressure=0.45})
print(wait(0))

--@ chunk 189
print(wait(8*24*60))
for _, p in ipairs({{700,600},{800,680},{652,520},{500,620},{484,590},{350,575},{330,500},{900,700},{100,500}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 190
WATR5 = rect(-5, 489.4, 1010, 232) - BANK2 - TREEX
WG.deep2 = pile{{"smalt",3},{"cobalt blue",1.2},{"raw umber",0.55},{"lead white",0.35}, medium=0.7}
function colshape4(x, y)
  local hw = 30 + 0.42 * math.max(0, y - 489)
  local u = (x - 652) / hw
  return math.exp(-u * u * 1.3)
end
local dload = function(x, y)
  local t = smoothstep(560, 712, y) ^ 1.1
  local side = 0.55 + 0.45 * smoothstep(0, 420, math.abs(x - 652))
  return t * side * (1 - 0.85 * colshape4(x, y))
end
work(WATR5, {hand="glaze", pile=WG.deep2, angle=0, coverage=2.6, load_at=dload, clip=WATR5, seed=48001})
blend(WATR5:shrink(1), {angle=0})
print(wait(0))

--@ chunk 191
print(wait(6*24*60))
for _, p in ipairs({{700,680},{800,690},{652,600},{500,660},{900,690},{300,560}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 192
-- ===== mist at the far shore's foot, then the reflection of the shore (all dry beneath) =====
WN.mist = pile{{"lead white",8},{"chrome yellow",0.55},{"vermilion",0.14},{"smalt",0.05}, medium=0.3}
local mistcov = function(x, y)
  local a = smoothstep(481, 489, y) * (1 - smoothstep(489.5, 497, y))
  local sunk = 0.65 + 0.35 * math.exp(-((x - 655) / 260) ^ 2)
  return 1.7 * a * sunk
end
stipple(rect(-5, 476, 1010, 26) - TREEX, {pile=WN.mist, width=4.5, coverage=mistcov, pressure={0.18, 0.4}, cluster=0.25, seed=49001, dips={20, 0.7, 0.5}})
blend(rect(-5, 484, 1010, 12):blur(3) - TREEX, {angle=0})

-- reflection of the shore: broken horizontal strokes hanging from the water line
WN.rf = pile{{"smalt",1.5},{"raw umber",0.8},{"red earth",0.35},{"bone black",0.1},{"lead white",1.9}, medium=0.3}
WN.rf2 = pile{{"smalt",1.4},{"raw umber",0.5},{"red earth",0.4},{"lead white",3.4}, medium=0.3}
math.randomseed(50077)
local fb = brush("flat", 1.5)
for i = 1, 300 do
  local x = rand(-5, 1005)
  local d = rand(0, 1) ^ 1.7
  local y = 490.2 + d * 8.5
  local len = rand(7, 36) * (1 - 0.4 * d)
  local pl = (x < 620) and WN.rf or WN.rf2
  fb:load(pl, rand(0.35, 0.7) * (1 - 0.5 * d))
  fb:stroke({{x, y}, {x + len * 0.5, y + rand(-0.2, 0.2)}, {x + len, y + rand(-0.2, 0.2)}}, {pressure={0.4, 0.3}, ramps={0.25, 0.5}, clip=WATR5})
end
print(wait(0))

--@ chunk 193
WATR6 = rect(-5, 489.4, 1010, 26) - BANK2 - TREEALL
WN.rfg = pile{{"smalt",1.4},{"raw umber",0.9},{"red earth",0.3},{"lead white",1.3}, medium=0.55}
-- 1. cover the blobs with the local water colours
work(rect(-5, 489.4, 1010, 13) * WATR6, {hand="body", tool="filbert 8", pile=WN.b1, angle=0, coverage=4, fill=true, clip=WATR6, seed=51001})
work(rect(-5, 499, 1010, 15) * WATR6, {hand="body", tool="filbert 8", pile=WN.b2, angle=0, coverage=4, fill=true, clip=WATR6, seed=51002,
     load_at=function(x, y) return 1 - smoothstep(506, 514, y) end})
-- 2. a soft dark reflection hanging from the shore, wet in wet
local rl3 = function(x, y)
  local k = 1.0 - 0.6 * smoothstep(560, 700, x) * (1 - 0.5 * smoothstep(850, 1000, x))
  local reach = 9 + 4 * math.sin(x / 43) * math.sin(x / 101 + 1)
  local t = clamp((y - 489.6) / reach, 0, 1)
  return k * (1 - t) ^ 1.5
end
work(rect(-5, 489.4, 1010, 18) * WATR6, {hand="glaze", pile=WN.rfg, angle=0, coverage=2.6, load_at=rl3, clip=WATR6, seed=51003})
blend(rect(-5, 489.6, 1010, 20):blur(3) * WATR6, {angle=0})
print(wait(0))

--@ chunk 194
print(wait(7*24*60))
for _, p in ipairs({{100,495},{500,494},{800,494},{420,486},{700,600},{300,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 195
WATR7 = rect(-5, 489.4, 1010, 232) - BANK2 - TREEALL
RP = {}
RP.dark  = pile{{"smalt",1.4},{"cobalt blue",0.6},{"raw umber",0.5},{"lead white",1.3}, medium=0.25}
RP.mid   = pile{{"smalt",1.2},{"raw umber",0.35},{"red earth",0.3},{"lead white",3.0}, medium=0.25}
RP.pale  = pile{{"lead white",8},{"chrome yellow",0.5},{"vermilion",0.12}, medium=0.2}
RP.paleb = pile{{"lead white",8},{"cobalt blue",0.5},{"chrome yellow",0.12}, medium=0.2}

local function ripple_field(y0, y1, n, pile_, w, lmin, lmax, seed, weight, xrange)
  math.randomseed(seed)
  local b = brush("flat", w)
  for i = 1, n do
    local u = rand(0, 1) ^ weight
    local y = lerp(y0, y1, u)
    local x = rand(xrange[1], xrange[2])
    local len = rand(lmin, lmax) * (0.6 + 0.8 * u)
    b:load(pile_, rand(0.3, 0.7))
    local dy = rand(-0.35, 0.35)
    b:stroke({{x, y}, {x + len * 0.5, y + dy}, {x + len, y + rand(-0.3, 0.3)}}, {pressure={0.28, 0.22}, ramps={0.3, 0.5}, clip=WATR7})
  end
end
-- dark ripples on the pale upper/middle water and the blue lower water
ripple_field(506, 700, 260, RP.dark, 0.9, 10, 46, 52001, 1.5, {-5, 1005})
-- pale glints, mostly in and near the glow column
ripple_field(520, 700, 240, RP.pale, 0.9, 8, 40, 52002, 1.2, {480, 830})
ripple_field(560, 706, 120, RP.paleb, 0.9, 10, 42, 52003, 1.0, {-5, 1005})
print(wait(0))

--@ chunk 196
blend(WATR7:shrink(1.5), {angle=0})
print(wait(0))

--@ chunk 197
-- ===== trunk modelling: bark ridges catching the glow on the sun side, then rim light =====
BK = {}
BK.ridge = pile{{"raw umber",2},{"yellow ochre",1.0},{"red earth",0.8},{"lead white",0.5},{"bone black",0.4}}
BK.ridge2 = pile{{"raw umber",2.2},{"red earth",0.7},{"yellow ochre",0.6},{"bone black",0.9}}
TRK = OAK2_MASKS.T * rect(-20, 478, 1050, 190)
-- centre line of the trunk by y, and its width, from the control points
function trunk_cx(y)
  local pts, ws = OAK2.T.pts, OAK2_W.T
  local best = 1
  for i = 1, #pts - 1 do if y <= pts[i][2] and y >= pts[i+1][2] then best = i break end end
  local a, b = pts[best], pts[best+1] or pts[best]
  local t = (a[2] - y) / math.max(1e-6, a[2] - b[2])
  t = clamp(t, 0, 1)
  return a[1] + (b[1] - a[1]) * t, ws[best] + ((ws[best+1] or ws[best]) - ws[best]) * t
end
local sidelit = function(x, y)
  local cx, w = trunk_cx(y)
  local u = (x - cx) / (w * 0.5)         -- -1 (left edge) .. +1 (right edge)
  return smoothstep(-0.15, 0.85, u)
end
-- long broken vertical strokes, mostly on the right half
work(TRK, {hand="body", tool="round 2.6", pile=BK.ridge, angle=function(x, y) return math.pi/2 + 0.12 * math.sin(y / 23 + x / 9) end,
  coverage=2.4, fill=false, clip=TRK, seed=53001, length={10, 34}, load=0.55, load_at=function(x, y) return 0.15 + 0.85 * sidelit(x, y) end,
  pressure={0.25, 0.5}})
-- fissures: dark, thin, on the whole trunk
work(TRK, {hand="body", tool="round 1.8", pile=BK.ridge2, angle=function(x, y) return math.pi/2 + 0.16 * math.sin(y / 17 + x / 7) end,
  coverage=1.6, fill=false, clip=TRK, seed=53002, length={8, 26}, load=0.7, pressure={0.3, 0.55}})
print(wait(0))

--@ chunk 198
print(wait(10*24*60))
for _, p in ipairs({{210,560},{215,640},{700,600},{500,540},{300,650}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 199
-- ===== repaint the trunk dark (dry brown beneath), from the crown junction down through the root flare =====
OK.trk = pile{{"raw umber",2.3},{"bone black",2.6},{"Prussian blue",0.6},{"red earth",0.2}}
TRK2 = OAK2_MASKS.T * rect(-20, 440, 1050, 240)
for pass = 1, 3 do
  work(TRK2, {hand="body", tool="filbert 7", pile=OK.trk, angle=function(x, y) return math.pi/2 + 0.08 * math.sin(y / 31 + pass) end,
    coverage=4, fill=true, clip=TRK2, seed=54000 + pass, length={14, 40}, load=1.0})
end
-- root flare: the six buttress roots again, dark, so the foot sinks into the bank
for i, pp in ipairs(RTP) do
  work(poly(pp), {hand="detail", tool="round 3", pile=OK.trk, angle=math.atan(ROOTS[i][1][5][2]-ROOTS[i][1][1][2], ROOTS[i][1][5][1]-ROOTS[i][1][1][1]),
    coverage=4, fill=true, clip=true, seed=54100 + i, load=1.0})
end
print(wait(0))

--@ chunk 200
-- foot of the trunk: cover the flat base and the long tentacle roots with ground, then draw shorter buttresses
ROOTM = nil
for i, pp in ipairs(RTP) do ROOTM = ROOTM and (ROOTM + poly(pp)) or poly(pp) end
local nfoot = noise{seed=77, octaves=3, period=38, persistence=0.55}
FOOTCUT = below(function(x) return 640 + 7 * nfoot(x, 3) end)
FOOTCOVER = (OAK2_MASKS.T + ROOTM):grow(1.5) * FOOTCUT * rect(-20, 600, 400, 110)
FG.b1 = pile{{"raw umber",3},{"bone black",1.8},{"Prussian blue",0.45},{"green earth",1.2},{"red earth",0.15}}
FG.b2 = pile{{"raw umber",3},{"bone black",1.4},{"green earth",1.6},{"yellow ochre",0.35},{"Prussian blue",0.3}}
FG.b3 = pile{{"raw umber",2.6},{"bone black",2.4},{"Prussian blue",0.6},{"red earth",0.2}}
work(FOOTCOVER, {hand="body", tool="filbert 9", pile=FG.b1, angle=0.12, coverage=4.5, fill=true, clip=FOOTCOVER, seed=55001, length={20,50}})
work(FOOTCOVER, {hand="body", tool="filbert 8", pile=FG.b2, angle=0.2, coverage=0.5, fill=false, clip=FOOTCOVER, seed=55002, length={14,36}})
print(wait(0))

--@ chunk 201
-- cover the two stray pale flecks with local ground
local f1 = ellipse(155.5, 696, 9, 4)
local f2 = ellipse(349.5, 621, 5, 4)
for i, f in ipairs({f1, f2}) do
  work(f, {hand="body", tool="filbert 4", pile=FG.b1, angle=0.1, coverage=4, fill=true, clip=f, seed=56000 + i, length={6, 14}})
end
print(wait(0))

--@ chunk 202
-- ===== water, calm re-lay: deeper blues toward the viewer, everything beneath is dry =====
WATR8 = rect(-5, 489.4, 1010, 232) - BANK2 - TREEX
WN.c1 = pile{{"lead white",8},{"chrome yellow",0.6},{"vermilion",0.12}, medium=0.05}
WN.c2 = pile{{"lead white",6},{"chrome yellow",0.85},{"vermilion",0.24},{"smalt",0.05}, medium=0.05}
WN.c3 = pile{{"lead white",6},{"chrome yellow",0.5},{"Prussian blue",0.03},{"smalt",0.12}, medium=0.05}
WN.c4 = pile{{"lead white",4},{"smalt",1.0},{"cobalt blue",0.3},{"Prussian blue",0.05},{"chrome yellow",0.08}, medium=0.05}
WN.c5 = pile{{"lead white",2.4},{"smalt",1.4},{"cobalt blue",0.7},{"Prussian blue",0.08},{"raw umber",0.15}, medium=0.05}
WN.c6 = pile{{"lead white",1.3},{"smalt",1.6},{"cobalt blue",0.8},{"Prussian blue",0.14},{"raw umber",0.3}, medium=0.05}
local ramp = function(y0, y1) return function(x, y) return smoothstep(y0, y1, y) end end
local bands = {
  {489.4, 30, WN.c1, nil},
  {505,   36, WN.c2, ramp(505, 517)},
  {530,   46, WN.c3, ramp(530, 544)},
  {566,   54, WN.c4, ramp(566, 582)},
  {608,   60, WN.c5, ramp(608, 626)},
  {654,   72, WN.c6, ramp(654, 672)},
}
for i, b in ipairs(bands) do
  work(rect(-5, b[1], 1010, b[2]) * WATR8, {hand="broad", pile=b[3], angle=0, coverage=3.4, fill=true, clip=WATR8, seed=57000 + i, load_at=b[4]})
end
-- warm reflection of the glow under the sun, widening toward the viewer
function colshape5(x, y)
  local hw = 30 + 0.40 * math.max(0, y - 489)
  local u = (x - 652) / hw
  return math.exp(-u * u * 1.35)
end
WN.col2 = pile{{"lead white",8},{"chrome yellow",0.7},{"vermilion",0.13}, medium=0.05}
local cl = function(x, y)
  local s = colshape5(x, y)
  return smoothstep(0.25, 0.85, s) * smoothstep(492, 506, y) * (1 - 0.35 * smoothstep(560, 714, y))
end
work(rect(400, 492, 520, 222) * WATR8, {hand="body", tool="filbert 12", pile=WN.col2, angle=0, coverage=3.0, fill=true, clip=WATR8, seed=57010, load_at=cl})
-- soft joins, one seam at a time, and the column edges
for _, s in ipairs({505, 530, 566, 608, 654}) do
  blend(rect(-5, s - 12, 1010, 28):blur(6) * WATR8, {angle=0})
end
blend(rect(520, 494, 260, 220):blur(10) * WATR8, {angle=0})
print(wait(0))

--@ chunk 203
-- ===== wet-in-wet: deepen the lower water on the safe side (x > 430), narrower and fainter column =====
WATSAFE = WATR8 * rect(430, 0, 600, 730)
WN.d4 = pile{{"lead white",3.6},{"smalt",1.1},{"cobalt blue",0.4},{"Prussian blue",0.06},{"chrome yellow",0.05}, medium=0.05}
WN.d5 = pile{{"lead white",2.0},{"smalt",1.5},{"cobalt blue",0.8},{"Prussian blue",0.10},{"raw umber",0.15}, medium=0.05}
WN.d6 = pile{{"lead white",1.0},{"smalt",1.8},{"cobalt blue",0.9},{"Prussian blue",0.20},{"raw umber",0.35}, medium=0.05}
local ramp = function(y0, y1) return function(x, y) return smoothstep(y0, y1, y) end end
local xfade = function(y0, y1)
  return function(x, y) return smoothstep(y0, y1, y) * smoothstep(430, 470, x) end
end
local bands = {
  {562, 62, WN.d4, xfade(562, 582)},
  {600, 62, WN.d5, xfade(600, 622)},
  {644, 76, WN.d6, xfade(644, 664)},
}
for i, b in ipairs(bands) do
  work(rect(430, b[1], 600, b[2]) * WATSAFE, {hand="broad", pile=b[3], angle=0, coverage=3.4, fill=true, clip=WATSAFE, seed=58000 + i, load_at=b[4]})
end
-- a narrow warm shimmer under the sun that dies out well above the viewer
function colshape6(x, y)
  local hw = 18 + 0.17 * math.max(0, y - 489)
  local u = (x - 652) / hw
  return math.exp(-u * u * 1.3)
end
local cl2 = function(x, y)
  local s = colshape6(x, y)
  return smoothstep(0.3, 0.9, s) * smoothstep(492, 506, y) * (1 - smoothstep(540, 650, y)) * 0.85
end
work(rect(560, 492, 200, 170) * WATSAFE, {hand="body", tool="filbert 10", pile=WN.col2, angle=0, coverage=2.6, fill=true, clip=WATSAFE, seed=58010, load_at=cl2})
blend(rect(430, 552, 600, 170):blur(8) * WATSAFE, {angle=0})
blend(rect(560, 500, 200, 150):blur(10) * WATSAFE, {angle=0})
print(wait(0))

--@ chunk 204
-- ===== melt the vertical seam at x=430 into the left water (stipple, then a soft blend) =====
TRANS = WATR8 * rect(290, 540, 240, 130)
local cov4 = function(x, y) return 1.7 * smoothstep(320, 470, x) * smoothstep(552, 584, y) * (1 - smoothstep(604, 632, y)) end
local cov5 = function(x, y) return 2.0 * smoothstep(320, 470, x) * smoothstep(586, 616, y) end
stipple(TRANS, {pile=WN.d4, width=7, coverage=cov4, pressure={0.3, 0.55}, cluster=0.2, seed=59001, dips={20, 0.7, 0.5}, clip=WATR8})
stipple(TRANS, {pile=WN.d5, width=7, coverage=cov5, pressure={0.3, 0.55}, cluster=0.2, seed=59002, dips={20, 0.7, 0.5}, clip=WATR8})
blend(rect(345, 548, 200, 118):blur(10) * WATR8, {angle=0})
print(wait(0))

--@ chunk 205
print(wait(8*24*60))
for _, p in ipairs({{700,600},{500,620},{400,560},{300,600},{800,690},{652,520},{200,640}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 206
print(wait(4*24*60))
for _, p in ipairs({{200,640},{215,600},{230,560},{150,690},{700,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 207
-- ===== grass on the crest (dry paint beneath) =====
math.randomseed(90210)
local function hump_len(maxlen, k) return maxlen * (0.45 + 0.55 * math.sin(math.pi * k)) end
local nclumps = 0
local x = 6
while x < 1002 do
  local n = math.floor(rand(5, 12))
  local spread = rand(4, 11)
  local maxlen = rand(8, 26)
  if x < 110 then maxlen = maxlen * 0.55 end
  if x > 300 and x < 420 then maxlen = maxlen * 0.8 end
  for i = 1, n do
    local k = (i - 0.5) / n
    local bx = x + (k - 0.5) * 2 * spread + rand(-1.2, 1.2)
    local by = crest_y(bx) + 0.8
    local len = hump_len(maxlen, k) * rand(0.6, 1.05)
    local lean = 0.10 + rand(-0.32, 0.32) + 0.22 * (k - 0.5)
    blade(bx, by, len, lean, rand(0.9, 1.6), GB, 0.9)
  end
  nclumps = nclumps + 1
  x = x + rand(9, 34)
end
print("grass clumps", nclumps)

-- reeds at the right
math.randomseed(4242)
local x = 738
local cnt = 0
while x < 1004 do
  local dens = 0.35 + 0.65 * smoothstep(738, 900, x)
  local n = math.floor(rand(2, 6) * dens + 0.5)
  local spread = rand(3, 8)
  for i = 1, n do
    local bx = x + rand(-spread, spread)
    local by = crest_y(bx) + 0.5
    local h = rand(22, 58) * (0.55 + 0.45 * dens)
    local lean = rand(-0.22, 0.22) - 0.05
    local bend = rand(-0.28, 0.28)
    reed(bx, by, h, lean, bend, rand(1.2, 1.9), (rand() < 0.42) and rand(5, 8) or nil)
    cnt = cnt + 1
  end
  x = x + rand(8, 26)
end
print("reeds", cnt)

-- tall stems with seed heads
local function stem(x, y, len, lean, bend, w, headlen, seed)
  math.randomseed(seed)
  local pts = {}
  for i = 0, 7 do
    local t = i / 7
    pts[#pts+1] = {x + lean * len * t + bend * len * t * t + rand(-0.2, 0.2) * t, y - len * t + (i == 0 and 2 or 0)}
  end
  RG:load(GB, 0.95)
  local pr = math.min(1, RG:pressure_for(w) + 0.02)
  RG:stroke(pts, {pressure={pr, 0.25}, ramps={0, 0}})
  local e, p = pts[#pts], pts[#pts - 2]
  local dx, dy = e[1] - p[1], e[2] - p[2]
  local L = math.sqrt(dx*dx + dy*dy); dx, dy = dx / L, dy / L
  RH:load(GB, 0.95)
  RH:stroke({{e[1] - dx * headlen * 0.3, e[2] - dy * headlen * 0.3}, {e[1] + dx * headlen, e[2] + dy * headlen}}, {pressure={0.6, 0.35}, ramps={0.15, 0.5}})
end
stem(506, crest_y(506) + 1, 44, 0.05, -0.10, 1.5, 9, 1)
stem(514, crest_y(514) + 1, 33, -0.12, 0.05, 1.3, 7, 2)
stem(561, crest_y(561) + 1, 52, 0.02, 0.12, 1.6, 11, 3)
stem(570, crest_y(570) + 1, 37, 0.14, -0.05, 1.3, 8, 4)
stem(618, crest_y(618) + 1, 30, -0.06, 0.08, 1.3, 7, 5)
stem(668, crest_y(668) + 1, 40, 0.06, 0.05, 1.4, 9, 6)
print(wait(0))

--@ chunk 208
-- ===== the figure, seen from behind (same shapes as before, now on dry paint) =====
for i, m in ipairs({FIGBODY, FIGNECK, FIGHEAD, FIGARM}) do
  work(m, {hand="detail", tool="round 2.2", pile=OK.fig, angle=math.pi/2, coverage=5, fill=true, clip=true, seed=59000 + i, length={4, 9}, load=0.95})
end
local sb = brush{kind="rigger", width=1.2, point=1}
sb:load(OK.fig, 1.0)
sb:stroke({{FX + 11.6, FY + 2.2}, {FX + 11.3, FY - 14}, {FX + 10.9, FY - 30}, {FX + 10.7, FY - 41}}, {pressure={0.75, 0.55}, ramps={0, 0.15}})
print(wait(0))

--@ chunk 209
-- (1) unify the fork: heavy dark over the trunk's top cap and the limb roots that meet it
FORKM = (OAK2_MASKS.T + OAK2_MASKS.L1 + OAK2_MASKS.C1 + OAK2_MASKS.R1) * rect(150, 436, 150, 84)
for pass = 1, 2 do
  work(FORKM, {hand="detail", tool="round 3", pile=OK.jet, angle=function(x, y) return -math.pi/2 + 0.3 * math.sin(x / 13) end,
    coverage=4, fill=true, clip=FORKM, seed=60000 + pass, length={6, 14}, load=1.0, dips={2, 1.0, 0.6}})
end
print(wait(0))

--@ chunk 210
nfoot = noise{seed=77, octaves=3, period=38, persistence=0.55}
local nfoot2 = noise{seed=811, octaves=3, period=14, persistence=0.6}
local footx = function(x) return smoothstep(128, 160, x) * (1 - smoothstep(250, 296, x)) end
local dcov = function(x, y)
  local edge = 640 + 7 * nfoot(x, 3)
  local d = y - edge
  if d < -4 then return 0 end
  local a = smoothstep(-4, 1, d) * (1 - smoothstep(4, 26 + 14 * (nfoot2(x, 9) * 0.5 + 0.5), d))
  return 2.4 * a * footx(x)
end
stipple(rect(120, 624, 190, 56), {pile=OK.trk, width=3.2, coverage=dcov, pressure={0.35, 0.7}, drag={7, math.pi/2}, cluster=0.55, seed=61001, dips={12, 0.9, 0.5}})
local bcov = function(x, y)
  local edge = 640 + 7 * nfoot(x, 3)
  local d = edge - y
  if d < -3 then return 0 end
  local a = smoothstep(-3, 1, d) * (1 - smoothstep(3, 16, d))
  return 1.6 * a * footx(x)
end
stipple(rect(120, 620, 190, 34), {pile=FG.b1, width=2.6, coverage=bcov, pressure={0.3, 0.55}, drag={5, -math.pi/2}, cluster=0.5, seed=61002, dips={12, 0.8, 0.5}})
print(wait(0))

--@ chunk 211
for _, p in ipairs({{200,620},{200,660},{240,600},{330,500},{150,300},{700,450}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 212
HILLM = poly({{700,489.4},{735,483},{770,478},{805,474},{845,470.5},{885,468},{925,467},{965,469},{1010,472},{1010,489.4},{700,489.4}}, true)
HG.pale2 = pile{{"lead white",8},{"chrome yellow",0.55},{"vermilion",0.2},{"smalt",0.04}, medium=0.4}
-- mist thickening toward the foot of the hill (clipped to the hill so nothing else is touched)
local hl = function(x, y)
  local t = smoothstep(468, 489, y)
  return 0.15 + 0.7 * t
end
work(HILLM, {hand="glaze", pile=HG.pale2, angle=0, coverage=1.8, load_at=hl, clip=HILLM, seed=62001})
-- lose the top edge into the sky
local n = lose(HILLM, {pile=HG.pale2, where=0.8, reach={5, 4}, load=0.25, tool="filbert 4", seed=62002})
print("lose strokes:", n)
print(wait(0))

--@ chunk 213
print(wait(12*24*60))
for _, p in ipairs({{200,620},{200,660},{900,480},{850,470},{940,457},{250,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 214
-- ===== the right-hand hill and the low far shore, again, as body paint on dry ground =====
FS.hillA = pile{{"smalt",2},{"red earth",0.55},{"raw umber",0.15},{"lead white",3.6}}
FS.hillB = pile{{"smalt",2},{"red earth",0.5},{"raw umber",0.1},{"lead white",5.2}}
HILLA = poly({{700,489.4},{735,483},{770,478},{805,474},{845,470.5},{885,468},{925,467},{965,469},{1010,472},{1010,489.4},{700,489.4}}, true)
HILLLOW = HILLA * rect(690, 476, 330, 14)
local hb = function(x, y) return smoothstep(469, 489, y) end
work(HILLA, {hand="body", tool="filbert 5", pile=FS.hillA, angle=0, coverage=3.4, fill=true, clip=HILLA, seed=63001})
work(HILLA, {hand="body", tool="filbert 5", pile=FS.hillB, angle=0, coverage=3.0, fill=true, clip=HILLA, seed=63002, load_at=hb})
-- crowns on the right, at the foot of the hill
for i, c in ipairs(CR) do
  if c[1] >= 700 then
    local m = crown(c[1], c[2], c[3], 3000 + i)
    work(m, {hand="body", tool="filbert 4", pile=FS.e4, angle=0.25, coverage=3.5, fill=true, clip=m, seed=63100 + i, load=1.0})
  end
end
work(LANDM * rect(690, 470, 330, 30), {hand="body", tool="filbert 4", pile=FS.e3, angle=0, coverage=4, fill=true, clip=LANDM * rect(690, 470, 330, 30), seed=63200})
print(wait(0))

--@ chunk 215
-- the stray pale crescent in the sky near (942,457)
local cm = ellipse(942, 456.5, 7, 5)
work(cm, {hand="detail", tool="round 2.2", pile=HG.gold, angle=0, coverage=3, fill=true, clip=cm, seed=64001, load=0.6})
blend(ellipse(942, 456.5, 11, 8):blur(4), {angle=0})
print(wait(0))

--@ chunk 216
for _, p in ipairs({{200,620},{200,660},{160,650},{280,650},{200,690},{230,600}}) do print(p[1], p[2], drying(p[1], p[2])) end
print(wait(0))

--@ chunk 217
-- ===== the trunk's foot, redone =====
-- 1. cover the dotted patch with bank colour, soft-edged
PATCH = rect(100, 622, 250, 86):blur(10)
FG.b1 = pile{{"raw umber",3},{"bone black",1.8},{"Prussian blue",0.45},{"green earth",1.2},{"red earth",0.15}}
work(PATCH, {hand="body", tool="filbert 9", pile=FG.b1, angle=0.12, coverage=4.5, fill=true, clip=PATCH, seed=65001, length={20, 50}})
work(PATCH, {hand="body", tool="filbert 8", pile=FG.b2, angle=0.2, coverage=0.5, fill=false, clip=PATCH, seed=65002, length={14, 36}})
print(wait(0))

--@ chunk 218
-- 2. the flare of the trunk and its buttress roots, each drawn on its own
FLARE = poly({
  {166,592},{162,606},{158,618},{152,628},{147,637},{141,645},{136,651},{144,655},{153,653},{160,658},{163,666},
  {170,662},{178,660},{190,663},{202,662},{214,664},{226,660},{238,658},{248,656},{258,652},{268,647},{262,639},
  {256,630},{251,620},{247,608},{243,592}
}, true)
RT2 = {
  {{{146,644},{132,650},{114,654},{94,656},{74,655}}, {14,11,8,5,2}},
  {{{160,652},{152,664},{142,678},{130,690},{118,698}}, {14,11,8,5,2}},
  {{{200,660},{202,674},{206,690},{212,704},{218,714}}, {12,10,8,5,2}},
  {{{246,652},{258,662},{274,670},{296,674},{318,674}}, {14,11,8,5,2}},
  {{{236,656},{240,670},{248,684},{260,696},{272,704}}, {13,10,7,4,2}},
}
RT2P = {}
for i, r in ipairs(RT2) do RT2P[i] = limb_outline(r[1], r[2], 1.6, 700 + i * 11) end
OK.trk = pile{{"raw umber",2.3},{"bone black",2.6},{"Prussian blue",0.6},{"red earth",0.2}}
for pass = 1, 2 do
  work(FLARE, {hand="body", tool="filbert 7", pile=OK.trk, angle=math.pi/2, coverage=3.5, fill=true, clip=FLARE, seed=66000 + pass, length={10, 30}, load=1.0})
end
for i, pp in ipairs(RT2P) do
  local r = RT2[i][1]
  work(poly(pp), {hand="detail", tool="round 3", pile=OK.trk, angle=math.atan(r[5][2] - r[1][2], r[5][1] - r[1][1]),
    coverage=4, fill=true, clip=true, seed=66100 + i, load=1.0})
end
print(wait(0))

--@ chunk 219
-- cover the ledge and the tentacles with bank colour, then draw one smooth flare that continues the trunk's own edges
ROOTALL = nil
for i, pp in ipairs(RT2P) do ROOTALL = ROOTALL and (ROOTALL + poly(pp)) or poly(pp) end
OLDFOOT = (FLARE + ROOTALL):grow(3)
NEWFLARE = poly({
  {169,596},{166,608},{162,620},{156,631},{148,642},{138,652},{124,660},{106,665},
  {106,690},{330,690},
  {314,666},{294,661},{277,653},{263,643},{252,631},{245,619},{241,607},{239,596}
}, true)
COV = (OLDFOOT - NEWFLARE:shrink(0.5)) * rect(60, 585, 300, 130)
work(COV, {hand="body", tool="filbert 8", pile=FG.b1, angle=0.12, coverage=4.5, fill=true, clip=COV, seed=67001, length={16, 40}})
work(COV, {hand="body", tool="filbert 8", pile=FG.b2, angle=0.2, coverage=0.5, fill=false, clip=COV, seed=67002, length={14, 34}})
print(wait(0))

--@ chunk 220
print(wait(14*24*60))
for _, p in ipairs({{200,640},{200,690},{160,650},{280,650},{200,600},{230,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 221
-- ===== unifying dark coat over the trunk and every limb, twig and hair between y=366 and the water =====
function in_band(pts, y0, y1)
  for _, p in ipairs(pts) do if p[2] >= y0 and p[2] <= y1 then return true end end
  return false
end
BY0, BY1 = 366, 525
SOFTTOP = rect(-30, BY0, 1080, 400):blur(8)
OK.jet = pile{{"raw umber",2.2},{"bone black",2.6},{"Prussian blue",0.6},{"red earth",0.2}}

-- trunk, with its outer edge grown 1 unit to bury the pale fringe
TRUNKBAND = OAK2_MASKS.T:grow(1.0) * rect(-30, BY0, 1080, 236):blur(8)
for pass = 1, 3 do
  work(TRUNKBAND, {hand="body", tool="filbert 7", pile=OK.jet, angle=function(x, y) return math.pi/2 + 0.08 * math.sin(y / 31 + pass) end,
    coverage=4, fill=true, clip=TRUNKBAND, seed=68000 + pass, length={14, 40}, load=1.0})
end
-- limbs
for i, k in ipairs(OAK2_ORDER) do
  if k ~= "T" then
    local m = OAK2_MASKS[k] * SOFTTOP
    local big = (OAK2_W[k][1] > 30)
    for pass = 1, 2 do
      work(m, {hand="detail", tool=big and "filbert 7" or "round 3", pile=OK.jet, angle=angfn(k),
        coverage=big and 4 or 3.5, fill=true, clip=m, seed=68100 + i * 3 + pass, length=big and {12, 30} or {6, 14}, load=1.0})
    end
  end
end
for i, pp in ipairs(STUBP) do
  local m = poly(pp) * SOFTTOP
  work(m, {hand="detail", tool="round 3", pile=OK.jet, angle=STUBS[i][3], coverage=4, fill=true, clip=m, seed=68200 + i, load=1.0})
end
work(poly(limb_outline({{238,276},{224,270},{212,262},{202,250}}, {13, 10, 8, 5}, 1.2, 411)) * SOFTTOP, {hand="detail", tool="round 3", pile=OK.jet, angle=-2.6, coverage=4, fill=true, clip=true, seed=68250, load=1.0})
print(wait(0))

--@ chunk 222
-- twigs and hairs in the band, thick to thin
local list = {}
for _, src in ipairs({BR, BR2}) do
  for _, b in ipairs(src) do if in_band(b.pts, BY0, BY1 + 40) then list[#list+1] = b end end
end
table.sort(list, function(a, b) return a.w0 > b.w0 end)
for i, b in ipairs(list) do paint_twig(b, OK.jet, 69000 + i) end
local nh = 0
for _, src in ipairs({HAIR_OLD, HAIR}) do
  for _, h in ipairs(src) do
    if in_band(h.pts, BY0, BY1 + 40) then paint_hair(h, OK.jet); nh = nh + 1 end
  end
end
print("twigs:", #list, "hairs:", nh)
print(wait(0))

--@ chunk 223
-- ===== the foot again, on dry paint: cover the tentacles, keep a broad irregular flare =====
nroot = noise{seed=913, octaves=3, period=26, persistence=0.6}
local cutline = {}
for x = 90, 340, 5 do cutline[#cutline+1] = {x, 662 + 6 * nroot(x, 2)} end
local lowcut = below(cutline)
COV2 = (ROOTALL + FLARE + OAK2_MASKS.T):grow(3) * lowcut * rect(60, 640, 320, 80)
FG.b1 = pile{{"raw umber",3},{"bone black",1.8},{"Prussian blue",0.45},{"green earth",1.2},{"red earth",0.15}}
FG.b2 = pile{{"raw umber",3},{"bone black",1.4},{"green earth",1.6},{"yellow ochre",0.35},{"Prussian blue",0.3}}
work(COV2, {hand="body", tool="filbert 9", pile=FG.b1, angle=0.12, coverage=5, fill=true, clip=COV2, seed=70001, length={20, 50}, load=1.0})
work(COV2, {hand="body", tool="filbert 8", pile=FG.b1, angle=-0.1, coverage=3, fill=true, clip=COV2, seed=70002, length={16, 40}, load=1.0})
print(wait(0))

--@ chunk 224
-- broad, short buttresses that leave the trunk and dive into the ground, each its own drawn line
RT3 = {
  {{{150,640},{140,652},{126,662},{108,668},{90,670}}, {22,17,11,6,2.5}},
  {{{176,652},{170,664},{160,676},{148,684},{136,690}}, {20,15,10,6,2.5}},
  {{{214,654},{216,668},{220,680},{226,690},{232,698}}, {18,14,10,6,2.5}},
  {{{252,646},{266,656},{282,662},{300,666},{318,667}}, {21,16,11,6,2.5}},
}
RT3P = {}
for i, r in ipairs(RT3) do RT3P[i] = limb_outline(r[1], r[2], 2.2, 900 + i * 7) end
for i, pp in ipairs(RT3P) do
  local r = RT3[i][1]
  work(poly(pp), {hand="detail", tool="round 3", pile=OK.trk, angle=math.atan(r[5][2] - r[1][2], r[5][1] - r[1][1]),
    coverage=4, fill=true, clip=true, seed=71000 + i, load=1.0})
end
-- feather the flare's lower edge into the bank
local fcov = function(x, y)
  local edge = 660 + 6 * nroot(x, 2)
  local d = y - edge
  if d < -3 then return 0 end
  local a = smoothstep(-3, 2, d) * (1 - smoothstep(3, 15, d))
  return 1.4 * a * smoothstep(120, 165, x) * (1 - smoothstep(250, 300, x))
end
stipple(rect(120, 640, 190, 50), {pile=OK.trk, width=3.0, coverage=fcov, pressure={0.3, 0.6}, drag={6, math.pi/2}, cluster=0.4, seed=71100, dips={12, 0.9, 0.5}})
print(wait(0))

--@ chunk 225
print(wait(10*24*60))
for _, p in ipairs({{200,660},{250,660},{300,650},{200,690},{230,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 226
-- (1) cover the two low thorn roots and the ragged lower rim of the flare
THORN = (poly(RT3P[2]) + poly(RT3P[3])):grow(2.5) * rect(120, 655, 140, 60)
work(THORN, {hand="body", tool="filbert 8", pile=FG.b1, angle=0.12, coverage=5, fill=true, clip=THORN, seed=72001, length={16, 40}, load=1.0})
work(THORN, {hand="body", tool="filbert 8", pile=FG.b2, angle=0.2, coverage=0.4, fill=false, clip=THORN, seed=72002, length={14, 34}})
print(wait(0))

--@ chunk 227
RIM = pile{{"yellow ochre",1.0},{"lead white",0.9},{"red earth",0.45},{"raw umber",0.6}}
local n = 0
n = n + paint_rim(OAK2_POLY.T, 0.45, 500, 640, 1.7, 0.75, 73001)
n = n + paint_rim(OAK2_POLY.R1, 0.5, 330, 520, 1.5, 0.7, 73002)
n = n + paint_rim(OAK2_POLY.C1, 0.55, 190, 480, 1.4, 0.65, 73003)
n = n + paint_rim(OAK2_POLY.R3, 0.5, 440, 540, 1.3, 0.6, 73004)
n = n + paint_rim(OAK2_POLY.L1, 0.55, 400, 500, 1.3, 0.55, 73005)
print("rim strokes", n)
print(wait(0))

--@ chunk 228
-- ===== three slender cloud bars over the glow (sky is dry beneath); each is its own drawn shape =====
CL.body4 = pile{{"smalt",1.8},{"vermilion",0.30},{"yellow ochre",0.22},{"raw umber",0.16},{"lead white",3.2}, medium=0.3}
CL.lit4  = pile{{"lead white",3},{"vermilion",0.8},{"chrome yellow",1.1}, medium=0.3}

local bars = {
  {x0=610, x1=1012, yc=391, th=9,   seed=81, sag=-4},
  {x0=700, x1=1012, yc=421, th=8,   seed=82, sag=3},
  {x0=790, x1=1012, yc=448, th=6,   seed=83, sag=-2},
}
local rims = {
  {x0=640, x1=1005, yc=397, th=4.2, seed=91, sag=-4},
  {x0=728, x1=1005, yc=426, th=3.8, seed=92, sag=3},
  {x0=812, x1=1005, yc=452, th=3.0, seed=93, sag=-2},
}
BARM = {}
for i, b in ipairs(bars) do
  local o = lens(b.x0, b.x1, b.yc, b.th, b.seed, b.sag)
  local m = o:mask() * SKYCLOUD
  BARM[i] = m
  local away = function(x, y) return 0.55 + 0.45 * smoothstep(650, 950, x) end
  work(m, {hand="body", tool="filbert 4", pile=CL.body4, angle=0, coverage=2.0, fill=true, clip=m, seed=82000 + i, load_at=away})
end
for i, r in ipairs(rims) do
  local o = lens(r.x0, r.x1, r.yc, r.th, r.seed, r.sag)
  local m = o:mask() * SKYCLOUD
  work(m, {hand="body", tool="filbert 3", pile=CL.lit4, angle=0, coverage=1.6, fill=true, clip=m, seed=82100 + i})
end
print(wait(0))

--@ chunk 229
for i = 1, 3 do
  local mm = BARM[i]:grow(8) * SKYCLOUD
  blend(mm, {angle=0})
  blend(mm, {angle=0.04})
end
print(wait(0))

--@ chunk 230
print(wait(12*24*60))
for _, p in ipairs({{800,392},{800,421},{900,449},{238,560},{238,620},{250,640}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 231
-- ===== restore the right horizon sky under the failed bars =====
RG1 = SKYCLOUD * rect(548, 366, 470, 126)
local xin = function(x) return smoothstep(548, 606, x) end
local fA = function(x, y) return xin(x) * smoothstep(368, 386, y) * (1 - smoothstep(404, 420, y)) end
local fB = function(x, y) return xin(x) * smoothstep(396, 414, y) * (1 - smoothstep(436, 452, y)) end
local fC = function(x, y) return xin(x) * smoothstep(428, 446, y) * (1 - smoothstep(462, 476, y)) end
local fD = function(x, y) return xin(x) * smoothstep(458, 472, y) end
work(RG1, {hand="broad", pile=SR.t2, angle=0, coverage=3, fill=true, clip=RG1, seed=90001, load_at=fA})
work(RG1, {hand="broad", pile=SR.t3, angle=0, coverage=3, fill=true, clip=RG1, seed=90002, load_at=fB})
work(RG1, {hand="broad", pile=HG.mid, angle=0, coverage=3, fill=true, clip=RG1, seed=90003, load_at=fC})
work(RG1, {hand="broad", pile=HG.cream, angle=0, coverage=3, fill=true, clip=RG1, seed=90004, load_at=fD})
blend(rect(556, 372, 470, 118):blur(12) * SKYCLOUD, {angle=0})
print(wait(0))

--@ chunk 232
print(wait(10*24*60))
for _, p in ipairs({{800,392},{700,421},{900,449},{800,440}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 233
-- ===== strata by stipple, double the earlier amplitude, no blending =====
CL.body5 = pile{{"smalt",1.8},{"vermilion",0.32},{"yellow ochre",0.18},{"raw umber",0.16},{"lead white",2.6}, medium=0.2}
CL.lit5  = pile{{"lead white",2.4},{"vermilion",0.75},{"chrome yellow",1.1}, medium=0.2}
SKYC2 = rect(548, 366, 470, 126) - SHOREX:grow(1) - TREEALL

function stratum2(x0, x1, yc, th, tilt, seed, amp)
  local nz = noise{seed=seed, octaves=3, period=55, persistence=0.55}
  local nz2 = noise{seed=seed + 9, octaves=2, period=12, persistence=0.5}
  local function yline(x, t) return yc + tilt * (t - 0.5) + 2.5 * nz(x, 7) end
  local function tapf(t) return math.sin(math.pi * clamp(t, 0, 1)) ^ 0.5 end
  local body = function(x, y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local ty = th * (0.5 + 0.7 * tapf(t)) * (0.8 + 0.4 * nz(x, 44))
    local d = (y - (yline(x, t) - 0.15 * ty)) / (0.5 * ty)
    return amp * math.exp(-d * d * 1.6) * tapf(t) * (0.7 + 0.5 * (nz2(x, y) * 0.5 + 0.5))
  end
  local lit = function(x, y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local ty = th * (0.5 + 0.7 * tapf(t)) * (0.8 + 0.4 * nz(x, 44))
    local d = (y - (yline(x, t) + 0.5 * ty)) / (0.3 * ty)
    return 0.9 * amp * math.exp(-d * d * 1.6) * tapf(t) * (0.6 + 0.6 * (nz2(x + 40, y) * 0.5 + 0.5))
  end
  local band = rect(x0 - 10, yc - th * 1.8 - 6, x1 - x0 + 20, th * 3.6 + 12) * SKYC2
  stipple(band, {pile=CL.body5, width=3.2, coverage=body, pressure={0.28, 0.5}, drag={16, 0}, cluster=0.3, seed=seed, dips={16, 0.8, 0.5}, clip=SKYC2})
  stipple(band, {pile=CL.lit5,  width=2.6, coverage=lit,  pressure={0.25, 0.48}, drag={12, 0}, cluster=0.3, seed=seed + 1, dips={16, 0.8, 0.5}, clip=SKYC2})
end
stratum2(590, 1000, 398, 8, -5, 92001, 4.4)
stratum2(680, 1010, 428, 7, 3, 92003, 4.6)
stratum2(770, 1010, 452, 5, -2, 92005, 4.4)
print(wait(0))

--@ chunk 234
-- ===== bury the gold rim lines: a single even dark coat over the trunk and the four limbs that carried them =====
RIMBAND = rect(-30, 300, 1080, 360)
TRUNKB2 = OAK2_MASKS.T:grow(1.0) * rect(-30, 470, 1080, 200):blur(6)
for pass = 1, 2 do
  work(TRUNKB2, {hand="body", tool="filbert 7", pile=OK.jet, angle=function(x, y) return math.pi/2 + 0.08 * math.sin(y / 29 + pass) end,
    coverage=4, fill=true, clip=TRUNKB2, seed=95000 + pass, length={14, 40}, load=1.0})
end
for i, k in ipairs({"R1", "C1", "R3", "L1"}) do
  local m = OAK2_MASKS[k] * rect(-30, 300, 1080, 250):blur(5)
  for pass = 1, 2 do
    work(m, {hand="detail", tool="filbert 6", pile=OK.jet, angle=angfn(k), coverage=4, fill=true, clip=m, seed=95100 + i * 3 + pass, length={10, 26}, load=1.0})
  end
end
-- pale flecks on the far hill
for i, p in ipairs({{899, 462.5, 6, 3}, {911, 462, 5, 2.5}, {925, 462.8, 6, 2.5}}) do
  local e = ellipse(p[1], p[2], p[3], p[4])
  work(e, {hand="detail", tool="round 2.2", pile=FS.hillA, angle=0, coverage=4, fill=true, clip=e, seed=95200 + i, load=0.9})
end
print(wait(0))

--@ chunk 235
-- ===== two higher, broader strata (thicker and farther apart than the ones near the horizon) =====
CL.body6 = pile{{"smalt",1.6},{"vermilion",0.30},{"yellow ochre",0.15},{"raw umber",0.12},{"lead white",3.4}, medium=0.2}
CL.lit6  = pile{{"lead white",3},{"vermilion",0.55},{"chrome yellow",0.8}, medium=0.2}
SKYC3 = rect(490, 250, 530, 110) - TREEALL

function stratum3(x0, x1, yc, th, tilt, seed, amp, wbody)
  local nz = noise{seed=seed, octaves=3, period=75, persistence=0.55}
  local nz2 = noise{seed=seed + 9, octaves=2, period=16, persistence=0.5}
  local function yline(x, t) return yc + tilt * (t - 0.5) + 3.5 * nz(x, 7) end
  local function tapf(t) return math.sin(math.pi * clamp(t, 0, 1)) ^ 0.5 end
  local body = function(x, y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local ty = th * (0.5 + 0.7 * tapf(t)) * (0.8 + 0.4 * nz(x, 44))
    local d = (y - (yline(x, t) - 0.12 * ty)) / (0.5 * ty)
    return amp * math.exp(-d * d * 1.5) * tapf(t) * (0.7 + 0.5 * (nz2(x, y) * 0.5 + 0.5))
  end
  local lit = function(x, y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local ty = th * (0.5 + 0.7 * tapf(t)) * (0.8 + 0.4 * nz(x, 44))
    local d = (y - (yline(x, t) + 0.5 * ty)) / (0.3 * ty)
    return 0.85 * amp * math.exp(-d * d * 1.5) * tapf(t) * (0.6 + 0.6 * (nz2(x + 40, y) * 0.5 + 0.5))
  end
  local band = rect(x0 - 10, yc - th * 1.8 - 6, x1 - x0 + 20, th * 3.6 + 12) * SKYC3
  stipple(band, {pile=CL.body6, width=wbody, coverage=body, pressure={0.25, 0.48}, drag={20, 0}, cluster=0.3, seed=seed, dips={16, 0.8, 0.5}, clip=SKYC3})
  stipple(band, {pile=CL.lit6,  width=wbody * 0.85, coverage=lit,  pressure={0.22, 0.45}, drag={16, 0}, cluster=0.3, seed=seed + 1, dips={16, 0.8, 0.5}, clip=SKYC3})
end
stratum3(560, 1010, 344, 11, -6, 96001, 3.6, 3.8)
stratum3(600, 1010, 298, 14, 4, 96003, 3.2, 4.2)
print(wait(0))

--@ chunk 236
print(wait(12*24*60))
for _, p in ipairs({{899,462},{925,462},{900,471},{800,452},{238,560},{700,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 237
-- ===== cover the three lilac ellipses in the sky, and the pale spots inside the hill =====
ELL = ellipse(899, 462.5, 6, 3):grow(2.5) + ellipse(911, 462, 5, 2.5):grow(2.5) + ellipse(925, 462.8, 6, 2.5):grow(2.5)
ELLM = ELL * rect(880, 456, 70, 14)
local up = function(x, y) return 1 - smoothstep(462, 468, y) end
work(ELLM, {hand="body", tool="filbert 4", pile=HG.mid, angle=0, coverage=4, fill=true, clip=ELLM, seed=97001, load=1.0, load_at=function(x, y) return 0.4 + 0.6 * (1 - smoothstep(460, 468, y)) end})
work(ELLM, {hand="body", tool="filbert 4", pile=HG.cream, angle=0, coverage=3, fill=true, clip=ELLM, seed=97002, load=1.0, load_at=function(x, y) return smoothstep(461, 467, y) end})
-- hill: cover the pale spots (x 885-935, y 468-476)
SPOTS = rect(880, 468.5, 70, 8) * HILLA
work(SPOTS, {hand="body", tool="filbert 3", pile=FS.hillB, angle=0, coverage=4, fill=true, clip=SPOTS, seed=97003, load=1.0})
print(wait(0))

--@ chunk 238
SPOTS2 = rect(872, 466.5, 84, 12):blur(5) * HILLA
work(SPOTS2, {hand="body", tool="filbert 3", pile=FS.hillA, angle=0, coverage=4, fill=true, clip=SPOTS2, seed=97010, load=1.0})
work(SPOTS2, {hand="body", tool="filbert 3", pile=FS.hillB, angle=0, coverage=3, fill=true, clip=SPOTS2, seed=97011, load=1.0, load_at=function(x, y) return smoothstep(469, 489, y) end})
print(wait(0))

--@ chunk 239
-- ===== deepen the zenith (nothing but sky above y=125) =====
SK2.zen2 = pile{{"smalt",3},{"cobalt blue",0.8},{"Prussian blue",0.16},{"lead white",0.7}, medium=0.7}
ZEN = rect(-5, -5, 1010, 140)
local zl = function(x, y)
  local t = 1 - smoothstep(20, 125, y)
  return 0.85 * t ^ 1.2 * (0.8 + 0.2 * math.sin(x / 140))
end
work(ZEN, {hand="glaze", pile=SK2.zen2, angle=0, coverage=2.0, load_at=zl, clip=ZEN, seed=98001})
print(wait(0))

--@ chunk 240
local bm = rect(-5, -5, 1010, 150):blur(6)
blend(bm, {angle=0})
blend(bm, {angle=0.03})
blend(bm, {angle=-0.03})
print(wait(0))

--@ chunk 241
print(wait(10*24*60))
for _, p in ipairs({{300,60},{700,100},{500,20},{700,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 242
-- ===== warm ring of light round the pale core: apricot on the horizon band, kept off the shore, the ruin and the tree =====
GL.apri = pile{{"lead white",3},{"chrome yellow",0.9},{"vermilion",0.55}, medium=0.7}
SKYFREE2 = rect(380, 396, 640, 94) - SHOREX:grow(0.8) - TREEALL
local ringload = function(x, y)
  local vy = smoothstep(408, 446, y) * (1 - smoothstep(472, 484, y))
  local core = 1 - 0.85 * math.exp(-((x - 655) / 105) ^ 2)
  local reach = math.exp(-((x - 655) / 330) ^ 2)
  return 0.9 * vy * core * reach
end
work(SKYFREE2, {hand="glaze", pile=GL.apri, angle=0, coverage=1.8, load_at=ringload, clip=SKYFREE2, seed=99001})
blend(rect(400, 400, 600, 86):blur(10) * SKYFREE2, {angle=0})
print(wait(0))

--@ chunk 243
-- ===== dissolve the horizontal seam at y~396 by stippling both colours across it =====
SEAMM = rect(410, 362, 610, 64) - TREEALL - SHOREX
GL.aprithin = pile{{"lead white",3.2},{"chrome yellow",0.95},{"vermilion",0.5}, medium=0.5}
SR.lemthin  = pile{{"lead white",5},{"chrome yellow",0.9},{"vermilion",0.10}, medium=0.5}
local reach = function(x) return 0.25 + 0.75 * smoothstep(410, 520, x) * (1 - 0.5 * smoothstep(900, 1010, x)) end
local covUp = function(x, y)
  return 1.5 * (1 - smoothstep(368, 397, y)) * smoothstep(360, 397, y) * reach(x) * (y < 400 and 1 or 0)
end
local covDn = function(x, y)
  return 1.4 * smoothstep(392, 397, y) * (1 - smoothstep(397, 424, y)) * reach(x)
end
stipple(SEAMM, {pile=GL.aprithin, width=5, coverage=covUp, pressure={0.2, 0.42}, cluster=0.2, seed=99101, dips={20, 0.7, 0.5}, clip=SEAMM})
stipple(SEAMM, {pile=SR.lemthin,  width=5, coverage=covDn, pressure={0.2, 0.42}, cluster=0.2, seed=99102, dips={20, 0.7, 0.5}, clip=SEAMM})
print(wait(0))

--@ chunk 244
for _, p in ipairs({{700,380},{700,396},{700,420},{500,390},{900,410}}) do print(p[1], p[2], drying(p[1], p[2])) end
blend(SEAMM:blur(6), {angle=0})
print(wait(0))

--@ chunk 245
-- tiny pale flecks on the bank: cover each with local ground colour
local pts = {{158,584},{452,665},{587,696}}
for i, p in ipairs(pts) do
  local e = ellipse(p[1], p[2], 2.6, 1.8)
  work(e, {hand="detail", tool="round 2.2", pile=FG.b1, angle=0.1, coverage=4, fill=true, clip=e, seed=99300 + i, load=1.0})
end
print(wait(0))

--@ chunk 246
-- bury the gold strokes on the leader and darken the pale stub
C1UP = OAK2_MASKS.C1:grow(0.6) * rect(150, 140, 200, 200):blur(4)
for pass = 1, 2 do
  work(C1UP, {hand="detail", tool="filbert 6", pile=OK.jet, angle=angfn("C1"), coverage=4.5, fill=true, clip=C1UP, seed=100000 + pass, length={10, 24}, load=1.0})
end
SIDEL = poly(limb_outline({{238,276},{224,270},{212,262},{202,250}}, {13, 10, 8, 5}, 1.2, 411))
STUB5 = poly(STUBP[5])
for pass = 1, 2 do
  work(SIDEL, {hand="body", tool="round 3", pile=OK.jet, angle=-2.6, coverage=5, fill=true, clip=SIDEL, seed=100010 + pass, load=1.0})
  work(STUB5, {hand="body", tool="round 3", pile=OK.jet, angle=STUBS[5][3], coverage=5, fill=true, clip=STUB5, seed=100020 + pass, load=1.0})
end
print(wait(0))

--@ chunk 247
-- ===== thin, broken warm rim along the crest where it catches the last light =====
CRIM = pile{{"yellow ochre",1.0},{"raw umber",0.7},{"green earth",0.9},{"lead white",0.7},{"red earth",0.15}}
math.randomseed(110001)
local rb = brush{kind="round", width=1.3, point=0.6}
local x = 262
local n = 0
while x < 985 do
  local len = rand(9, 26)
  local x1 = math.min(985, x + len)
  local pts = {}
  local steps = math.max(3, math.floor((x1 - x) / 4))
  for i = 0, steps do
    local px = lerp(x, x1, i / steps)
    pts[#pts+1] = {px, crest_y(px) + 0.15 + 0.15 * math.sin(px * 0.9)}
  end
  -- stronger in the middle of the stretch, dying out toward the far right where reeds crowd
  local k = smoothstep(262, 420, x) * (1 - 0.6 * smoothstep(760, 985, x))
  if rand() < 0.75 * (0.4 + 0.6 * k) then
    rb:load(CRIM, 0.35 + 0.35 * k * rand(0.6, 1.0))
    local pr = rand(0.32, 0.5)
    rb:stroke(pts, {pressure={pr, pr * 0.8}, ramps={0.3, 0.45}, clip=BANK2})
    n = n + 1
  end
  x = x1 + rand(2, 16)
end
print("rim strokes:", n)
print(wait(0))

--@ chunk 248
-- ===== clean knoll under the ruin and firs; the pale streaks at its foot are covered =====
FS.knoll4 = pile{{"smalt",2},{"raw umber",0.75},{"red earth",0.4},{"Prussian blue",0.08},{"lead white",1.5}}
FS.knoll5 = pile{{"smalt",2},{"raw umber",0.55},{"red earth",0.45},{"lead white",2.6}}
KNOLL2 = poly({{590,489.4},{604,486},{620,483},{634,479.4},{650,477.4},{680,476.6},{710,476.8},{738,478},{760,480.6},{782,483.6},{806,486.6},{826,489.4}}, true)
KNM2 = KNOLL2 - RUIN:grow(0.4) - FIR1:grow(0.4) - FIR2:grow(0.4) - FIR3:grow(0.4) - FIR4:grow(0.4)
work(KNM2, {hand="detail", pile=FS.knoll4, angle=0.05, coverage=4, fill=true, clip=KNM2, seed=120001, load=1.0})
-- lighter toward the foot (atmosphere), same mask
work(KNM2, {hand="detail", pile=FS.knoll5, angle=0.05, coverage=3, fill=true, clip=KNM2, seed=120002, load=1.0, load_at=function(x, y) return smoothstep(482, 489.4, y) * 0.9 end})
-- ruin and firs, one more pass so their feet sit cleanly on the knoll
work(RUIN - LANCET, {hand="detail", pile=FS.ruin3, angle=1.4, coverage=3, fill=true, clip=true, seed=120003, load=1.0})
for i, f in ipairs({FIR1, FIR2, FIR3, FIR4}) do
  work(f, {hand="detail", pile=FS.ruin3, angle=1.5, coverage=3, fill=true, clip=true, seed=120010 + i, load=1.0})
end
print(wait(0))

--@ chunk 249
-- ===== the far-right hill: mist thickening at its foot, done as stipple on the dry hill; land strip below it =====
HZ2 = pile{{"lead white",8},{"chrome yellow",0.55},{"vermilion",0.2},{"smalt",0.05}, medium=0.35}
HILLB = HILLA - KNOLL2:grow(1)
local hz = function(x, y) return 1.6 * smoothstep(472, 489, y) * smoothstep(808, 850, x) end
stipple(HILLB, {pile=HZ2, width=4.5, coverage=hz, pressure={0.2, 0.42}, cluster=0.2, seed=121001, dips={20, 0.7, 0.5}, clip=HILLB})
-- a low crowded tree line along the foot, right side, drawn as separate small crowns
for i, c in ipairs({{826,22,4},{860,30,5},{898,26,4},{934,24,3.5},{968,26,4},{996,20,3}}) do
  local m = crown(c[1], c[2], c[3], 3000 + 22 + i)
  work(m, {hand="detail", pile=FS.e4, angle=0.25, coverage=3, fill=true, clip=m, seed=121100 + i, load=1.0})
end
print(wait(0))

--@ chunk 250
-- ===== reflections under the far shore (all beneath is dry) =====
WATR9 = rect(-5, 489.4, 1010, 232) - BANK2 - TREEALL
WN.rfg2 = pile{{"smalt",1.4},{"raw umber",0.9},{"red earth",0.3},{"lead white",1.4}, medium=0.55}
local rl4 = function(x, y)
  local k = 0.85 * (1.0 - 0.5 * smoothstep(560, 700, x) * (1 - 0.5 * smoothstep(850, 1000, x)))
  local reach = 8 + 4 * math.sin(x / 43) * math.sin(x / 101 + 1)
  local t = clamp((y - 489.6) / reach, 0, 1)
  return k * (1 - t) ^ 1.5
end
work(rect(-5, 489.4, 1010, 16) * WATR9, {hand="glaze", pile=WN.rfg2, angle=0, coverage=2.4, load_at=rl4, clip=WATR9, seed=130001})
blend(rect(-5, 489.6, 1010, 18):blur(3) * WATR9, {angle=0})

-- reflections of the ruin and the firs: each its own wavering, tapering line, shorter and paler than the thing itself
WN.rff = pile{{"smalt",1.5},{"raw umber",0.7},{"red earth",0.3},{"lead white",1.5}, medium=0.35}
RGR = brush{kind="rigger", width=1.2, point=1}
function hangr(x, y0, len, w, seed, pl, load)
  math.randomseed(seed)
  local pts = {}
  for i = 0, 6 do
    local t = i / 6
    pts[#pts+1] = {x + rand(-0.7, 0.7) * (0.3 + t), y0 + len * t}
  end
  RGR:load(pl, load)
  RGR:stroke(pts, {pressure={math.min(1, RGR:pressure_for(w) + 0.02), 0.04}, ramps={0, 0.6}, clip=WATR9})
end
hangr(699.4, 490, 15, 2.6, 131001, WN.rff, 0.7)
hangr(709.2, 490, 24, 3.2, 131002, WN.rff, 0.75)
hangr(721.6, 490, 20, 3.0, 131003, WN.rff, 0.72)
hangr(733.6, 490, 11, 2.4, 131004, WN.rff, 0.65)
-- the ruin: a broken, fading dark shape with a paler heart where the window is
function refl_blob(cx, w, depth, seed, taper)
  math.randomseed(seed)
  local Lp, Rp = {}, {}
  for i = 0, 7 do
    local t = i / 7
    local y = 489.6 + depth * t
    local half = (w / 2) * (1 - (taper or 0.5) * t ^ 1.2) + rand(-0.7, 0.7) * (0.4 + t)
    Lp[#Lp+1] = {cx - half + rand(-0.8, 0.8) * t, y}
    Rp[#Rp+1] = {cx + half + rand(-0.8, 0.8) * t, y}
  end
  local pts = {}
  for i = 1, #Lp do pts[#pts+1] = Lp[i] end
  for i = #Rp, 1, -1 do pts[#pts+1] = Rp[i] end
  return poly(pts, true)
end
RUINR = refl_blob(657, 40, 20, 131010, 0.55)
work(RUINR * WATR9, {hand="detail", pile=WN.rff, angle=math.pi/2, coverage=2.4, fill=true, clip=WATR9, seed=131011, load=0.55,
  load_at=function(x, y) return 1 - 0.8 * smoothstep(490, 510, y) end})
print(wait(0))

--@ chunk 251
-- ===== a broken row of fishing stakes leading out into the glow toward the ruin =====
POSTS = {
  {566, 637, 31, 1.8, -0.030},
  {594, 598, 22, 1.5,  0.035},
  {612, 569, 15, 1.2, -0.050},
  {627, 546, 12, 1.0,  0.020},
  {638, 528,  8, 0.9,  0.000},
  {645, 514,  6, 0.8,  0.040},
  {650, 504,  4, 0.7, -0.020},
}
math.randomseed(140001)
for i, p in ipairs(POSTS) do
  local x, y, h, w, lean = table.unpack(p)
  local t = clamp((y - 500) / 140, 0, 1)
  local pl = pile{{"raw umber", 1.0 + 1.0 * t}, {"bone black", 0.4 + 1.4 * t}, {"smalt", 1.6 - 1.0 * t}, {"lead white", 1.6 - 1.2 * t}}
  local br = brush{kind="rigger", width=w, point=1}
  br:load(pl, 1.0)
  local pr = math.min(1, br:pressure_for(w))
  br:stroke({{x, y + 0.5}, {x + lean * h * 0.5 + rand(-0.15, 0.15), y - h * 0.5}, {x + lean * h, y - h}}, {pressure={pr, pr}, ramps={0, 0}})
  -- its reflection: its own hanging line, paler, breaking as it goes
  local rp = pile{{"smalt", 1.4}, {"raw umber", 0.8 + 0.4 * t}, {"lead white", 1.6 - 0.6 * t}, medium=0.3}
  local rr = brush{kind="rigger", width=w, point=1}
  rr:load(rp, 0.85)
  local len = h * rand(0.75, 1.0)
  local pts = {}
  for k = 0, 5 do
    local u = k / 5
    pts[#pts+1] = {x + rand(-0.5, 0.5) * (0.3 + u) * (0.6 + w * 0.3), y + 1 + len * u}
  end
  rr:stroke(pts, {pressure={math.min(1, rr:pressure_for(w * 0.9)), 0.05}, ramps={0, 0.55}})
end
print(wait(0))

--@ chunk 252
-- ===== the foot of the trunk: an irregular lip of turf overlaps the flat bottom, with short olive blades in front =====
FOOTLIP = poly({
  {166,674},{170,664},{175,659},{180,662},{185,656},{191,660},{196,654},{203,659},{209,653},{215,658},{221,652},{227,657},
  {233,653},{239,659},{245,655},{251,660},{257,656},{263,662},{270,668},{276,674}
}, true)
FG.b1 = pile{{"raw umber",3},{"bone black",1.8},{"Prussian blue",0.45},{"green earth",1.2},{"red earth",0.15}}
FG.b2 = pile{{"raw umber",3},{"bone black",1.4},{"green earth",1.6},{"yellow ochre",0.35},{"Prussian blue",0.3}}
work(FOOTLIP, {hand="detail", pile=FG.b1, angle=0.05, coverage=4, fill=true, clip=FOOTLIP, seed=150001, load=1.0})
work(FOOTLIP, {hand="detail", pile=FG.b2, angle=0.1, coverage=0.8, fill=false, clip=FOOTLIP, seed=150002, load=0.8})

-- short blades, olive and dry, rising over the lip
GBL = pile{{"raw umber",2},{"yellow ochre",0.9},{"green earth",1.3},{"lead white",0.3}}
math.randomseed(150010)
local gb = brush{kind="rigger", width=1.2, point=1}
local xs = {}
local x = 172
while x < 272 do
  local top = 657 + 3 * math.sin(x * 0.31) 
  local n = math.floor(rand(2, 4))
  for i = 1, n do
    local bx = x + rand(-3, 3)
    local len = rand(6, 15) * (0.6 + 0.4 * math.sin(math.pi * (bx - 166) / 110))
    local lean = rand(-0.35, 0.35)
    local by = 660 + rand(-2, 4)
    gb:load(GBL, rand(0.55, 0.9))
    gb:stroke({{bx, by}, {bx + lean * len * 0.4, by - len * 0.5}, {bx + lean * len, by - len}}, {pressure={math.min(1, gb:pressure_for(rand(0.9, 1.4)) + 0.02), 0.0}, ramps={0, 0}})
  end
  x = x + rand(5, 12)
end
print(wait(0))

--@ chunk 253
print(wait(10*24*60))
for _, p in ipairs({{200,650},{200,660},{240,660},{700,600},{690,148}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 254
-- cover the straw blades with bank colour, then draw a few dark olive tufts
BLADEZONE = rect(166, 640, 116, 30):blur(3) - OAK2_MASKS.T:shrink(2)
work(BLADEZONE, {hand="body", tool="filbert 6", pile=FG.b1, angle=0.1, coverage=5, fill=true, clip=BLADEZONE, seed=151001, length={10, 26}, load=1.0})
work(BLADEZONE, {hand="body", tool="filbert 6", pile=FG.b2, angle=0.15, coverage=0.4, fill=false, clip=BLADEZONE, seed=151002, length={10, 22}})
print(wait(0))

--@ chunk 255
-- regenerate the exact blade geometry (same seed, same order of random calls) and cover each blade with bank dark
math.randomseed(150010)
local cover = brush{kind="rigger", width=2.2, point=1}
local x = 172
local nb = 0
while x < 272 do
  local top = 657 + 3 * math.sin(x * 0.31)
  local n = math.floor(rand(2, 4))
  for i = 1, n do
    local bx = x + rand(-3, 3)
    local len = rand(6, 15) * (0.6 + 0.4 * math.sin(math.pi * (bx - 166) / 110))
    local lean = rand(-0.35, 0.35)
    local by = 660 + rand(-2, 4)
    local _ = rand(0.55, 0.9)          -- was the load
    local w = rand(0.9, 1.4)
    cover:load(FG.b1, 1.0)
    local pr = math.min(1, cover:pressure_for(w + 1.1) + 0.02)
    cover:stroke({{bx, by + 0.8}, {bx + lean * len * 0.4, by - len * 0.5}, {bx + lean * len, by - len - 0.8}}, {pressure={pr, pr * 0.5}, ramps={0, 0.2}})
    nb = nb + 1
  end
  x = x + rand(5, 12)
end
print("blades covered:", nb)
print(wait(0))

--@ chunk 256
RINGM = (ellipse(912, 461, 34, 8) - HILLA:grow(0.3))
HG.ring = pile{{"lead white",6},{"chrome yellow",0.95},{"vermilion",0.33}, medium=0.05}
work(RINGM, {hand="detail", pile=HG.ring, angle=0, coverage=4, fill=true, clip=RINGM, edge="soft", seed=160001, load=1.0})
-- re-establish the hill's crest right there
HCREST = HILLA * rect(880, 464, 64, 8)
work(HCREST, {hand="detail", pile=FS.hillA, angle=0, coverage=3, fill=true, clip=HCREST, seed=160002, load=1.0})
print(wait(0))

--@ chunk 257
GL.warm2 = pile{{"lead white",3.2},{"chrome yellow",1.2},{"vermilion",0.62}, medium=0.55}
RINGG = ellipse(912, 460, 46, 12) - HILLA:grow(0.3)
local rg = function(x, y)
  local dx, dy = (x - 912) / 44, (y - 460) / 11
  local r = math.sqrt(dx * dx + dy * dy)
  return clamp(1 - r, 0, 1) ^ 0.6 * 0.95
end
work(RINGG, {hand="glaze", pile=GL.warm2, angle=0, coverage=2.0, load_at=rg, clip=RINGG, seed=160010})
print(wait(0))

--@ chunk 258
print(wait(8*24*60))
for _, p in ipairs({{900,470},{912,458},{280,655},{120,675},{690,148},{200,650}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 259
-- (1) the hill: smooth body coat over its lower slope, then its crest; no stipple
HILLLOWER = HILLA * rect(690, 468, 330, 22)
FS.hillM = pile{{"smalt",2},{"red earth",0.52},{"raw umber",0.12},{"lead white",4.4}}
work(HILLLOWER, {hand="body", tool="filbert 4", pile=FS.hillM, angle=0, coverage=4, fill=true, clip=HILLLOWER, seed=170001, load=1.0})
work(HILLLOWER, {hand="body", tool="filbert 4", pile=FS.hillB, angle=0, coverage=3, fill=true, clip=HILLLOWER, seed=170002, load=1.0,
  load_at=function(x, y) return smoothstep(474, 489, y) * 0.85 end})
-- the tree line at its foot, re-laid on the fresh coat
for i, c in ipairs({{826,22,4},{860,30,5},{898,26,4},{934,24,3.5},{968,26,4},{996,20,3}}) do
  local m = crown(c[1], c[2], c[3], 3000 + 22 + i)
  work(m, {hand="detail", pile=FS.e4, angle=0.25, coverage=3, fill=true, clip=m, seed=170100 + i, load=1.0})
end
print(wait(0))

--@ chunk 260
-- shorten the two horns: cover their outer parts with bank colour, soft-edged
HORNL = poly({{80,652},{132,646},{140,676},{80,684}}, true):blur(2)
HORNR = poly({{282,650},{330,652},{334,680},{282,676}}, true):blur(2)
for i, m in ipairs({HORNL, HORNR}) do
  local mm = m * BANK2
  work(mm, {hand="body", tool="filbert 7", pile=FG.b1, angle=0.12, coverage=5, fill=true, clip=mm, seed=180001 + i, length={12, 30}, load=1.0})
  work(mm, {hand="body", tool="filbert 7", pile=FG.b2, angle=0.2, coverage=0.4, fill=false, clip=mm, seed=180011 + i, length={10, 24}})
end
print(wait(0))

--@ chunk 261
-- the tree's own shadow pooling at its foot: a dark glaze that dies away into the lit bank
SHADEP = pile{{"raw umber",2.2},{"bone black",2.6},{"Prussian blue",0.6},{"red earth",0.2}, medium=0.6}
SHADEM = ellipse(208, 664, 104, 30) * BANK2 - OAK2_MASKS.T:shrink(1)
local sl = function(x, y)
  local dx, dy = (x - 208) / 100, (y - 664) / 27
  local r = math.sqrt(dx * dx + dy * dy)
  return clamp(1 - r, 0, 1) ^ 0.8 * 0.95
end
work(SHADEM, {hand="glaze", pile=SHADEP, angle=0, coverage=2.2, load_at=sl, clip=SHADEM, seed=181001})
print(wait(0))

--@ chunk 262
BL2 = rect(164, 645, 82, 18):blur(3) - OAK2_MASKS.T:shrink(3)
work(BL2, {hand="body", tool="filbert 5", pile=OK.trk, angle=0.05, coverage=5, fill=true, clip=BL2, seed=182001, length={8, 20}, load=1.0})
print(wait(0))

--@ chunk 263
BL3 = rect(160, 640, 92, 30):blur(2)
work(BL3, {hand="body", tool="filbert 5", pile=OK.trk, angle=math.pi/2, coverage=5, fill=true, clip=BL3, seed=183001, length={8, 20}, load=1.0})
print(wait(0))

--@ chunk 264
-- ===== the right hill, again on dry paint: body coat, a darker lilac along the ridge, a continuous low woodland at the foot =====
local ridge_pts = {{700,489.4},{735,483},{770,478},{805,474},{845,470.5},{885,468},{925,467},{965,469},{1010,472}}
function ridge_y(x)
  if x <= ridge_pts[1][1] then return ridge_pts[1][2] end
  for i = 1, #ridge_pts - 1 do
    if x <= ridge_pts[i+1][1] then
      local t = (x - ridge_pts[i][1]) / (ridge_pts[i+1][1] - ridge_pts[i][1])
      return ridge_pts[i][2] * (1 - t) + ridge_pts[i+1][2] * t
    end
  end
  return ridge_pts[#ridge_pts][2]
end
HILLZ = HILLA * rect(786, 456, 230, 34)
work(HILLZ, {hand="body", tool="filbert 4", pile=FS.hillM, angle=0, coverage=4, fill=true, clip=HILLZ, seed=190001, load=1.0})
FS.hillTop = pile{{"smalt",2},{"red earth",0.42},{"raw umber",0.18},{"lead white",2.6}, medium=0.55}
local topl = function(x, y)
  local d = y - ridge_y(x)
  return (1 - smoothstep(0, 15, d)) * 0.9
end
work(HILLA * rect(700, 456, 320, 34), {hand="glaze", pile=FS.hillTop, angle=0, coverage=2.0, load_at=topl, clip=HILLA, seed=190002})
print(wait(0))

--@ chunk 265
print(wait(9*24*60))
for _, p in ipairs({{900,478},{850,475},{980,480},{200,650},{200,660}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 266
-- ridge darkening: a thin body coat of a slightly bluer lilac, feathered downward
FS.hillTop2 = pile{{"smalt",2},{"red earth",0.42},{"raw umber",0.2},{"lead white",2.3}}
RIDGEZ = HILLA * rect(700, 456, 320, 34)
local topl2 = function(x, y)
  local d = y - ridge_y(x)
  return (1 - smoothstep(0.5, 13, d)) * 0.9
end
work(RIDGEZ, {hand="body", tool="filbert 4", pile=FS.hillTop2, angle=0, coverage=3.5, fill=true, clip=HILLA, seed=191001, load_at=topl2})
-- continuous low woodland at the foot
local nw1 = noise{seed=931, octaves=3, period=46, persistence=0.55}
local nw2 = noise{seed=932, octaves=2, period=7, persistence=0.5}
local ringp = {}
for x = 792, 1006, 1.2 do
  local top = 482.4 - 3.4 * (nw1(x, 0) * 0.5 + 0.5) - 1.1 * nw2(x, 0)
  ringp[#ringp+1] = {x, top}
end
ringp[#ringp+1] = {1006, 489.4}
ringp[#ringp+1] = {792, 489.4}
WOODR = poly(ringp)
FS.wood1 = pile{{"smalt",2},{"raw umber",0.65},{"red earth",0.4},{"lead white",2.2}}
FS.wood2 = pile{{"smalt",2},{"raw umber",0.5},{"red earth",0.45},{"lead white",3.6}}
work(WOODR, {hand="detail", pile=FS.wood1, angle=0, coverage=4, fill=true, clip=WOODR, seed=191002, load=1.0})
work(WOODR, {hand="detail", pile=FS.wood2, angle=0, coverage=3, fill=true, clip=WOODR, seed=191003, load=1.0, load_at=function(x, y) return smoothstep(484, 489.4, y) * 0.85 end})
print(wait(0))

--@ chunk 267
-- ===== soften the woodland strip on the right hill: hide its left end, then set small crowns of varying height =====
FEATH = rect(776, 479, 70, 11):blur(7) * HILLA
work(FEATH, {hand="body", tool="filbert 3", pile=FS.hillB, angle=0, coverage=4, fill=true, clip=FEATH, seed=200001, load=1.0})
work(FEATH, {hand="body", tool="filbert 3", pile=FS.hillM, angle=0, coverage=2, fill=true, clip=FEATH, seed=200002, load=1.0, load_at=function(x, y) return 1 - smoothstep(484, 489, y) end})
print(wait(0))

--@ chunk 268
-- small crowns breaking the flat top of the woodland band
local crowns_r = {
  {842, 14, 4.0}, {858, 18, 6.0}, {877, 14, 4.5}, {893, 20, 7.5}, {914, 16, 5.0}, {930, 22, 8.5},
  {952, 16, 5.5}, {969, 20, 7.0}, {988, 16, 5.0}, {1002, 14, 4.0},
}
for i, c in ipairs(crowns_r) do
  local m = crown(c[1], c[2], c[3] + 3, 5000 + i)
  local shade = (i % 3 == 0) and FS.wood1 or FS.wood2
  work(m, {hand="detail", pile=shade, angle=0.25, coverage=3, fill=true, clip=m, seed=201000 + i, load=1.0})
end
print(wait(0))

--@ chunk 269
for _, p in ipairs({{870,480},{930,480},{800,480}}) do print(p[1], p[2], drying(p[1], p[2])) end
-- the wet crowns are too fresh to overpaint cleanly; wait first
print(wait(10*24*60))
for _, p in ipairs({{870,480},{930,480},{800,480}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 270
-- ===== the right hill as one clean form: body coat, hazy foot, cool ridge =====
work(HILLA, {hand="body", tool="filbert 5", pile=FS.hillM, angle=0, coverage=4.5, fill=true, clip=HILLA, seed=210001, load=1.0})
work(HILLA, {hand="body", tool="filbert 5", pile=FS.hillB, angle=0, coverage=3.5, fill=true, clip=HILLA, seed=210002, load=1.0,
  load_at=function(x, y) return smoothstep(473, 489, y) * 0.9 end})
local topl3 = function(x, y)
  local d = y - ridge_y(x)
  return (1 - smoothstep(0.5, 12, d)) * 0.8
end
work(HILLA, {hand="body", tool="filbert 4", pile=FS.hillTop2, angle=0, coverage=3.0, fill=true, clip=HILLA, seed=210003, load_at=topl3})
print(wait(0))

--@ chunk 271
print(wait(10*24*60))
for _, p in ipairs({{870,480},{930,480},{800,480},{960,470}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 272
-- ===== low far woodland at the foot of the right hill: a fine-toothed dark-lilac band =====
FS.lw1 = pile{{"smalt",2},{"raw umber",0.6},{"red earth",0.42},{"lead white",2.4}}
FS.lw2 = pile{{"smalt",2},{"raw umber",0.5},{"red earth",0.45},{"lead white",3.4}}
local nl1 = noise{seed=951, octaves=3, period=60, persistence=0.55}
local nl2 = noise{seed=952, octaves=3, period=9, persistence=0.6}
local nl3 = noise{seed=953, octaves=2, period=3.2, persistence=0.5}
local function lw_top(x)
  local fade = smoothstep(770, 850, x)             -- grows out of the knoll's foot
  local h = 0.6 + 3.2 * (nl1(x, 0) * 0.5 + 0.5) + 1.3 * nl2(x, 0) + 0.6 * nl3(x, 0)
  return 489.2 - math.max(0.4, h) * fade
end
local lp = {}
for x = 760, 1008, 0.9 do lp[#lp+1] = {x, lw_top(x)} end
lp[#lp+1] = {1008, 489.6}
lp[#lp+1] = {760, 489.6}
LWOOD = poly(lp)
work(LWOOD, {hand="detail", pile=FS.lw1, angle=0.1, coverage=4, fill=true, clip=LWOOD, seed=220001, load=1.0})
work(LWOOD, {hand="detail", pile=FS.lw2, angle=0.1, coverage=3, fill=true, clip=LWOOD, seed=220002, load=1.0,
  load_at=function(x, y) return smoothstep(485, 489.4, y) * 0.9 end})
print(wait(0))

--@ chunk 273
BKD = pile{{"raw umber",2},{"yellow ochre",1.2},{"red earth",0.6},{"lead white",0.9},{"bone black",0.3}}
local bt = brush{kind="filbert", width=4, stiffness=0.9}
-- test on a small stretch of the trunk's right edge (x 222-246, y 520-560)
math.randomseed(777)
for i = 1, 6 do
  local x0 = 232 + rand(-3, 6)
  local y0 = 522 + rand(0, 12)
  bt:load(BKD, 0.07)
  bt:stroke({{x0, y0}, {x0 + rand(-1.5, 1.5), y0 + 14}, {x0 + rand(-2, 2), y0 + rand(24, 40)}}, {pressure={0.35, 0.3}, ramps={0.15, 0.35}, clip=OAK2_MASKS.T})
end
print(wait(0))

--@ chunk 274
print(wait(12*24*60))
for _, p in ipairs({{232,540},{232,530},{700,470}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 275
PATCHM = OAK2_MASKS.T:grow(0.8) * rect(218, 514, 34, 56):blur(1.5)
for pass = 1, 3 do
  work(PATCHM, {hand="body", tool="filbert 6", pile=OK.jet, angle=math.pi/2 + 0.05 * pass, coverage=4, fill=true, clip=PATCHM, seed=230000 + pass, length={8, 22}, load=1.0})
end
print(wait(0))

--@ chunk 276
TRG = pile{{"raw umber",2},{"yellow ochre",0.8},{"red earth",0.5},{"lead white",0.9}, medium=0.75}
TRKM = OAK2_MASKS.T * rect(-20, 486, 1050, 158)
local function tcx(y)
  local pts, ws = OAK2.T.pts, OAK2_W.T
  local best = 1
  for i = 1, #pts - 1 do if y <= pts[i][2] and y >= pts[i+1][2] then best = i break end end
  local a, b = pts[best], pts[best+1] or pts[best]
  local t = clamp((a[2] - y) / math.max(1e-6, a[2] - b[2]), 0, 1)
  return a[1] + (b[1] - a[1]) * t, ws[best] + ((ws[best+1] or ws[best]) - ws[best]) * t
end
local side = function(x, y)
  local cx, w = tcx(y)
  local u = (x - cx) / (w * 0.5)
  local s = smoothstep(0.05, 1.0, u)
  local fade = smoothstep(486, 520, y) * (1 - smoothstep(590, 644, y))
  return 0.5 * s * s * fade
end
work(TRKM, {hand="glaze", pile=TRG, angle=math.pi/2, coverage=1.6, load_at=side, clip=TRKM, seed=240001})
print(wait(0))

--@ chunk 277
TRB = pile{{"raw umber",3},{"red earth",0.9},{"yellow ochre",0.5},{"bone black",0.8},{"lead white",0.35}}
local side2 = function(x, y)
  local cx, w = trunk_cx(y)
  local u = (x - cx) / (w * 0.5)
  local s = smoothstep(0.1, 1.0, u)
  local fade = smoothstep(490, 525, y) * (1 - smoothstep(585, 640, y))
  return s * s * fade
end
work(TRKM, {hand="body", tool="filbert 4", pile=TRB, angle=function(x, y) return math.pi/2 + 0.06 * math.sin(y / 17 + x / 5) end,
  coverage=1.1, fill=false, clip=TRKM, seed=241001, length={10, 30}, load=0.8, load_at=side2, pressure={0.3, 0.55}})
print(wait(0))

--@ chunk 278
print(wait(12*24*60))
for _, p in ipairs({{232,540},{236,560},{240,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 279
print(wait(6*24*60))
for _, p in ipairs({{232,540},{236,560},{240,600},{232,520}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 280
TRKD = pile{{"raw umber",2.2},{"bone black",2.6},{"Prussian blue",0.5},{"red earth",0.35}, medium=0.55}
local dark_load = function(x, y)
  local cx, w = trunk_cx(y)
  local u = (x - cx) / (w * 0.5)
  -- heavy over the middle of the patches, lighter at the very edge so a thin warm line stays there
  return 0.95 - 0.55 * smoothstep(0.78, 1.0, u)
end
local RIGHTHALF = OAK2_MASKS.T * rect(214, 486, 60, 158)
for pass = 1, 2 do
  work(RIGHTHALF, {hand="glaze", pile=TRKD, angle=math.pi/2, coverage=1.6, load_at=dark_load, clip=RIGHTHALF, seed=250000 + pass})
end
print(wait(0))

--@ chunk 281
print(wait(10*24*60))
for _, p in ipairs({{232,540},{236,560},{240,640},{232,520}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 282
TRUNKFIX = OAK2_MASKS.T:grow(1.0) * rect(-30, 470, 1080, 200):blur(5)
for pass = 1, 3 do
  work(TRUNKFIX, {hand="body", tool="filbert 8", pile=OK.jet, angle=function(x, y) return math.pi/2 + 0.07 * math.sin(y / 29 + pass) end,
    coverage=4, fill=true, clip=TRUNKFIX, seed=260000 + pass, length={14, 42}, load=1.0})
end
print(wait(0))

--@ chunk 283
-- ===== remove the ghost 'second sun' above the right hill: repaint that patch of sky in bands, feathered =====
PATCHSKY = (rect(806, 428, 210, 46) - HILLA:grow(0.2)):blur(5)
local xin = function(x) return smoothstep(806, 856, x) end
local fA = function(x, y) return xin(x) * smoothstep(430, 446, y) * (1 - smoothstep(450, 462, y)) end
local fB = function(x, y) return xin(x) * smoothstep(446, 458, y) * (1 - smoothstep(462, 470, y)) end
local fC = function(x, y) return xin(x) * smoothstep(458, 466, y) end
work(PATCHSKY, {hand="body", tool="filbert 8", pile=HG.gold, angle=0, coverage=3, fill=true, clip=PATCHSKY, seed=270001, load_at=fA})
work(PATCHSKY, {hand="body", tool="filbert 8", pile=HG.mid,  angle=0, coverage=3, fill=true, clip=PATCHSKY, seed=270002, load_at=fB})
work(PATCHSKY, {hand="body", tool="filbert 8", pile=HG.cream, angle=0, coverage=3, fill=true, clip=PATCHSKY, seed=270003, load_at=fC})
blend(rect(800, 434, 216, 36):blur(6) - HILLA:grow(0.5), {angle=0})
print(wait(0))

--@ chunk 284
blend(rect(770, 436, 110, 36):blur(14) - HILLA:grow(0.5), {angle=0})
print(wait(0))

--@ chunk 285
-- the evening star, a little firmer: a pale core with a thin halo, on dry sky
STAR2 = brush{kind="round", width=3.0, point=0.6}
STARP2 = pile{{"lead white",9},{"chrome yellow",0.08}}
STAR2:load(STARP2, 1.0)
STAR2:touch(690, 148, {pressure=0.7})
-- faint halo: a few pale, low-pressure touches round it
local hp = pile{{"lead white",6},{"cobalt blue",0.15}, medium=0.5}
local hb = brush{kind="round", width=2.2, point=0.5}
math.randomseed(280001)
for i = 1, 9 do
  local a = rand(0, 2 * math.pi)
  local r = rand(3.2, 6.5)
  hb:load(hp, 0.35)
  hb:touch(690 + math.cos(a) * r, 148 + math.sin(a) * r, {pressure=0.22})
end
print(wait(0))

--@ chunk 286
print(wait(0))
local pts = {{200,600},{200,660},{150,640},{300,650},{100,600},{400,690},{500,700},{200,520},{240,500},{360,400},{700,300},{700,600},{800,470},{660,470},{600,700}}
for _,p in ipairs(pts) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 287
print(wait(21*24*60))
local pts = {{200,600},{200,660},{200,520},{240,500},{800,470},{900,475},{700,478}}
for _,p in ipairs(pts) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 288
local names = {}
for k,v in pairs(_G) do
  local t = type(v)
  if t ~= "function" or true then names[#names+1] = k .. ":" .. t end
end
table.sort(names)
print(#names)
print(table.concat(names, "  "))

--@ chunk 289
for k,v in pairs(OK) do print("OK."..k, v) end
for k,v in pairs(FG) do print("FG."..k, v) end
for k,v in pairs(HG) do print("HG."..k, v) end
for k,v in pairs(FS) do if type(v)=="userdata" then print("FS."..k, v) end end

--@ chunk 290
-- irregular shadow pool at the foot of the oak, hand-placed points (the tree's shadow falls toward the viewer, left)
FOOTPTS = {{160,626},{146,640},{130,652},{116,664},{113,677},{128,689},{152,698},{180,704},{208,706},{238,703},{264,696},{286,685},{300,671},{298,656},{285,643},{267,634},{252,626}}
POOLM = poly(FOOTPTS, true)
POOLB = POOLM:blur(18)
print(POOLM:area(), POOLB:at(205,672), POOLB:at(113,677), POOLB:at(205,700), POOLB:at(205,720))
print(drying(205,690), drying(120,670), drying(285,660))

--@ chunk 291
local loadf = function(x, y) return clamp((POOLB:at(x, y) - 0.5) * 2.2, 0, 1) ^ 0.9 end
work(POOLM, {hand="body", tool="filbert 8", pile=OK.base, angle=function(x, y) return 0.10 + 0.05 * math.sin(x / 23 + y / 9) end,
  coverage=3, fill=true, clip=POOLM, seed=300001, length={14, 40}, load_at=loadf})
print(wait(0))

--@ chunk 292
local T = OAK2_MASKS.T
for _, y in ipairs({520, 560, 590, 610, 625, 640, 655, 665, 672, 680, 690}) do
  local xl, xr = nil, nil
  for x = 60, 340, 0.5 do
    local v = T:at(x, y)
    if v >= 0.5 then if not xl then xl = x end; xr = x end
  end
  print(y, xl, xr, xl and xr and (xr - xl))
end

--@ chunk 293
print(wait(10*24*60))
for _, p in ipairs({{205,640},{205,670},{205,700},{130,670},{280,670}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 294
-- the oak's foot: T's own sides continue outward as a gentle flare; the black fades into the shadowed ground
local T = OAK2_MASKS.T
FLPTS = {{168,600},{163,615},{157,630},{150,645},{143,658},{135,669},{129,679},
         {146,689},{172,695},{204,698},{236,694},{258,687},{271,678},
         {262,665},{254,652},{248,640},{243,625},{241,610},{238,600}}
FLARE2 = poly(FLPTS, true) + (T * rect(-50, 596, 1200, 100))
FL2B = poly(FLPTS, true):blur(9)
local arc = {{129,679},{146,689},{172,695},{204,698},{236,694},{258,687},{271,678}}
local function by(x)
  if x <= arc[1][1] then return arc[1][2] - 2 end
  for i = 1, #arc - 1 do
    if x <= arc[i+1][1] then
      local t = (x - arc[i][1]) / (arc[i+1][1] - arc[i][1])
      return arc[i][2] * (1 - t) + arc[i+1][2] * t
    end
  end
  return arc[#arc][2] - 2
end
local loadf2 = function(x, y)
  local sf = math.max(T:at(x, y), clamp((FL2B:at(x, y) - 0.5) * 3, 0, 1))
  local vf = 1 - smoothstep(by(x) - 20, by(x) + 2, y)
  return sf * vf
end
for pass = 1, 2 do
  work(FLARE2, {hand="body", tool="filbert 7", pile=OK.jet, angle=function(x, y) return math.pi/2 + 0.08 * math.sin(y / 21 + pass) end,
    coverage=3.5, fill=true, clip=FLARE2, seed=310000 + pass, length={10, 34}, load_at=loadf2})
end
print(wait(0))

--@ chunk 295
local nz = noise{seed=4412, octaves=2, period=38, persistence=0.5}
local nz2 = noise{seed=4413, octaves=2, period=14, persistence=0.5}
local function xl(y) if y <= 640 then return 154.5 elseif y <= 655 then return 154.5 - 3.5 * (y - 640) / 15 else return 151 - 0.15 * (y - 655) end end
local function xr(y) if y <= 640 then return 244 elseif y <= 655 then return 244 + (y - 640) / 15 else return 245 + 0.15 * (y - 655) end end
BELLZ = FLARE2 * rect(0, 636, 1000, 80)
DITHER_C = function(x, y)
  local ye = y + 7 * nz(x, 0) + 2 * nz2(x, y)
  local rv = smoothstep(650, 692, ye)
  local d = math.max(xl(y) - x, x - xr(y)) + 3 * nz2(x, y)
  local rs = smoothstep(0, 12, d)
  return 3.0 * math.max(rv, rs)
end
stipple(BELLZ, {pile=FG.b1, width=3, coverage=DITHER_C, drag={2.5, 0.12}, clip=BELLZ, seed=320001, cluster=0.15, dips={30, 1.0, 0.3}})
print(wait(0))

--@ chunk 296
TA = pile{{"raw umber",3},{"bone black",1.2},{"green earth",1.6},{"yellow ochre",0.6},{"lead white",0.5}}
TB = pile{{"raw umber",3},{"bone black",1.0},{"green earth",1.8},{"yellow ochre",0.8},{"lead white",0.9}}
TC = FG.olive
TD = FG.b2
local cand = {TA, TB, TC, TD}
for i, p in ipairs(cand) do
  local x0 = 146 + (i - 1) * 27
  local m = rect(x0, 682, 24, 10)
  work(m, {hand="body", tool="filbert 4", pile=p, angle=0.1, coverage=4, fill=true, clip=m, seed=330000 + i, load=1.0})
end
print(wait(0))

--@ chunk 297
M1 = pile{{"raw umber",3},{"bone black",1.6},{"Prussian blue",0.38},{"green earth",1.4},{"yellow ochre",0.18},{"red earth",0.08}}
M2 = pile{{"raw umber",3},{"bone black",1.5},{"Prussian blue",0.34},{"green earth",1.5},{"yellow ochre",0.25},{"red earth",0.05}}
M3 = pile{{"raw umber",3},{"bone black",1.5},{"Prussian blue",0.34},{"green earth",1.5},{"yellow ochre",0.25},{"red earth",0.05}, medium=0.35}
local cand = {M1, M2, M3, FG.b1}
for i, p in ipairs(cand) do
  local x0 = 146 + (i - 1) * 27
  local m = rect(x0, 682, 24, 10)
  work(m, {hand="body", tool="filbert 4", pile=p, angle=0.1, coverage=4, fill=true, clip=m, seed=331000 + i, load=1.0})
end
print(wait(0))

--@ chunk 298
print(wait(12*24*60))
for _, p in ipairs({{158,687},{185,687},{212,687},{240,687},{205,660},{205,700}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 299
SWM = rect(143, 679, 112, 16):blur(1.2)
work(SWM, {hand="body", tool="filbert 5", pile=OK.jet, angle=0.08, coverage=5, fill=true, clip=SWM, seed=340001, length={10, 26}, load=1.0})
-- graded cloud of the trunk black around the bell: density falls with distance outside the flare
FLB = FLARE2:blur(26)
CLOUDM = FLARE2:grow(38) * rect(0, 600, 1000, 114)
local cloud_c = function(x, y)
  local v = FLB:at(x, y)
  return 3.2 * smoothstep(0.03, 0.5, v) * smoothstep(598, 632, y)
end
stipple(CLOUDM, {pile=OK.jet, width=4, coverage=cloud_c, drag={2.2, 0.1}, clip=CLOUDM, seed=340002, cluster=0.2, feather=1, dips={30, 1.0, 0.3}})
print(wait(0))

--@ chunk 300
print(wait(12*24*60))
for _, p in ipairs({{205,640},{205,670},{205,700},{140,670},{270,670},{160,690},{250,690}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 301
local T = OAK2_MASKS.T
for y = 586, 672, 6 do
  local xl, xr
  for x = 100, 320, 0.5 do
    local v = T:at(x, y)
    if v >= 0.5 then if not xl then xl = x end; xr = x end
  end
  print(y, xl, xr)
end
print(FLARE2:area(), CLOUDM:area())

--@ chunk 302
TFL = {{173,588},{170,598},{166,606},{163,614},{160,622},{158,630},{156,638},{154,646},{152.5,654},{151,662},{150,670},{151,677}}
TFR = {{236,588},{238,598},{240,606},{241,614},{240,622},{240.5,630},{242,638},{244,646},{245,654},{246,662},{247,670},{248,677}}
local pts = {}
for i = 1, #TFL do pts[#pts+1] = TFL[i] end
-- bottom edge, irregular, hand-placed
local bot = {{157,679},{166,676},{176,680},{188,677},{199,681},{211,677},{222,680},{233,676},{242,679}}
for i = 1, #bot do pts[#pts+1] = bot[i] end
for i = #TFR, 1, -1 do pts[#pts+1] = TFR[i] end
TF = poly(pts, true):roughen(1.0, 12, 5)
print(TF:area())
COVERW = (FLARE2:grow(50) * rect(0, 590, 1000, 130)):blur(8)
OUTERSOFT = COVERW - TF
work(OUTERSOFT, {hand="body", tool="filbert 8", pile=OK.base, angle=function(x, y) return 0.10 + 0.05 * math.sin(x / 23 + y / 9) end,
  coverage=3, fill=true, clip=OUTERSOFT, seed=350001, length={14, 40}})
print(wait(0))

--@ chunk 303
for x = 236, 330, 6 do
  -- topmost dark pixel of the current spill: scan BANK2 vs OUTERSOFT
  local top
  for y = 560, 640, 0.5 do
    if OUTERSOFT:at(x, y) > 0.4 then top = y break end
  end
  print(x, "crest_y", crest_y(x), "spill top", top)
end

--@ chunk 304
for x = 100, 300, 10 do
  local top
  for y = 560, 700, 0.5 do
    if OUTERSOFT:at(x, y) > 0.08 then top = y break end
  end
  print(x, "crest_y", string.format("%.1f", crest_y(x)), "OUTER>0.08 from", top)
end
print(WN.c3, WN.c4)
print(BANK2:at(270, 592), BANK2:at(270, 598))

--@ chunk 305
print(wait(12*24*60))
for _, p in ipairs({{205,640},{205,675},{270,590},{200,590},{140,650},{280,690},{250,700}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 306
TRB2 = OAK2_MASKS.T * rect(160, 580, 90, 12):blur(2)
work(TRB2, {hand="body", tool="filbert 5", pile=OK.jet, angle=math.pi/2 + 0.04, coverage=4, fill=true, clip=TRB2, seed=360001, length={8, 22}, load=1.0})
print(wait(0))

--@ chunk 307
local nh1 = noise{seed=8801, octaves=2, period=44, persistence=0.5}
local nh2 = noise{seed=8802, octaves=2, period=9, persistence=0.5}
local function hem(x) return 675.0 + 2.6 * nh1(x, 0) + 0.7 * nh2(x, 0) end
HEMLINE = below(function(x) return hem(x) end)
HEMM = (TF:grow(2.5) * HEMLINE) * rect(140, 660, 120, 40)
print(HEMM:area())
work(HEMM, {hand="body", tool="filbert 5", pile=OK.base, angle=0.06, coverage=5, fill=true, clip=HEMM, seed=370001, length={8, 22}, load=1.0})
print(wait(0))

--@ chunk 308
WEDGE = (OUTERSOFT:grow(2.5)) * rect(236, 572, 84, 40) - BANK2:grow(0.4) - OAK2_MASKS.T:grow(0.5)
print(WEDGE:area())
-- rows of the wedge coverage
for y = 578, 606, 3 do
  local s = ""
  for x = 236, 316, 2 do s = s .. (WEDGE:at(x, y) > 0.5 and "#" or ".") end
  print(y, s)
end

--@ chunk 309
WEDGE = OUTERSOFT:map(function(v) return v > 0.02 and 1 or 0 end) * rect(236, 572, 84, 44) - BANK2:grow(0.4) - OAK2_MASKS.T:grow(0.5)
print(WEDGE:area())
for y = 578, 606, 2 do
  local s = ""
  for x = 236, 316, 2 do s = s .. (WEDGE:at(x, y) > 0.5 and "#" or ".") end
  print(y, s)
end

--@ chunk 310
WEDGES = WEDGE:grow(1.2):blur(1.5) - BANK2:grow(0.3) - OAK2_MASKS.T:grow(0.5)
work(WEDGES, {hand="body", tool="filbert 6", pile=WN.c4, angle=0, coverage=4, fill=true, clip=WEDGES, seed=380001, length={10, 26}, load=1.0})
print(wait(0))

--@ chunk 311
WN.m31 = pile{{"lead white",22},{"chrome yellow",1.58},{"Prussian blue",0.14},{"smalt",1.36},{"cobalt blue",0.3}, medium=0.05}
print(drying(270, 592), drying(290, 596))
work(WEDGES, {hand="body", tool="filbert 6", pile=WN.m31, angle=0, coverage=3, fill=true, clip=WEDGES, seed=380002, length={10, 26}, load=1.0})
print(wait(0))

--@ chunk 312
local inner = WEDGES:shrink(2.5) - BANK2:grow(2) - OAK2_MASKS.T:grow(2)
print(inner:area())
blend(inner, {angle=0})
blend(inner, {angle=0.05})
print(wait(0))

--@ chunk 313
print(wait(3*24*60))
for _, p in ipairs({{270,590},{290,597},{255,585},{300,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 314
print(wait(3*24*60))
for _, p in ipairs({{270,590},{290,597},{255,585},{300,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 315
for _, p in ipairs({{205,672},{205,678},{205,690},{160,676},{250,676},{120,660}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 316
print(wait(8*24*60))
for _, p in ipairs({{205,672},{205,678},{205,690},{160,676},{250,676},{120,660}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 317
SHM2 = rect(105, 652, 200, 56):blur(4)
local sl2 = function(x, y)
  local lat = 1 - smoothstep(55, 100, math.abs(x - 205))
  local dn = 1 - smoothstep(671, 702, y)
  local up = smoothstep(650, 668, y)
  return 0.92 * lat * dn * up
end
work(SHM2, {hand="glaze", pile=SHADEP, angle=0, coverage=2.2, load_at=sl2, clip=SHM2, seed=390001})
print(wait(0))

--@ chunk 318
print(wait(12*24*60))
for _, p in ipairs({{205,672},{205,690},{160,680},{270,685},{130,670},{290,690}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 319
POOLCOV = SHM2:grow(3) - TF:shrink(1.5)
work(POOLCOV, {hand="body", tool="filbert 8", pile=OK.base, angle=function(x, y) return 0.10 + 0.05 * math.sin(x / 23 + y / 9) end,
  coverage=3, fill=true, clip=POOLCOV, seed=400001, length={14, 40}})
print(wait(0))

--@ chunk 320
print(wait(12*24*60))
for _, p in ipairs({{205,672},{205,690},{160,680},{270,685},{130,670},{290,690}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 321
math.randomseed(90210)
local function hump_len(maxlen, k) return maxlen * (0.45 + 0.55 * math.sin(math.pi * k)) end
local x = 6
local list = {}
while x < 1002 do
  local n = math.floor(rand(5, 12))
  local spread = rand(4, 11)
  local maxlen = rand(8, 26)
  if x < 110 then maxlen = maxlen * 0.55 end
  if x > 300 and x < 420 then maxlen = maxlen * 0.8 end
  for i = 1, n do
    local k = (i - 0.5) / n
    local bx = x + (k - 0.5) * 2 * spread + rand(-1.2, 1.2)
    local by = crest_y(bx) + 0.8
    local len = hump_len(maxlen, k) * rand(0.6, 1.05)
    local lean = 0.10 + rand(-0.32, 0.32) + 0.22 * (k - 0.5)
    local w = rand(0.9, 1.6)
    local r1 = rand(-0.7, 0.7)
    local r2 = rand(-0.6, 0.6)
    if bx >= 236 and bx <= 312 then
      list[#list+1] = string.format("bx=%.1f base=%.1f len=%.1f tipx=%.1f tipy=%.1f", bx, by, len, bx + lean * len + r2, by - len)
    end
  end
  x = x + rand(9, 34)
end
print(#list)
print(table.concat(list, "\n"))

--@ chunk 322
math.randomseed(90210)
local function hump_len(maxlen, k) return maxlen * (0.45 + 0.55 * math.sin(math.pi * k)) end
local x = 6
local painted = 0
while x < 1002 do
  local n = math.floor(rand(5, 12))
  local spread = rand(4, 11)
  local maxlen = rand(8, 26)
  if x < 110 then maxlen = maxlen * 0.55 end
  if x > 300 and x < 420 then maxlen = maxlen * 0.8 end
  for i = 1, n do
    local k = (i - 0.5) / n
    local bx = x + (k - 0.5) * 2 * spread + rand(-1.2, 1.2)
    local by = crest_y(bx) + 0.8
    local len = hump_len(maxlen, k) * rand(0.6, 1.05)
    local lean = 0.10 + rand(-0.32, 0.32) + 0.22 * (k - 0.5)
    local w = rand(0.9, 1.6)
    local r1 = rand(-0.7, 0.7)
    local r2 = rand(-0.6, 0.6)
    if bx >= 236 and bx <= 312 then
      local mid = {bx + lean * len * 0.35 + r1, by - len * 0.5}
      local tip = {bx + lean * len + r2, by - len}
      GRG:load(GB, 0.9)
      local pr = math.min(1, GRG:pressure_for(w) + 0.02)
      GRG:stroke({{bx, by + 1.5}, mid, tip}, {pressure={pr, 0.0}, ramps={0, 0}})
      painted = painted + 1
    end
  end
  x = x + rand(9, 34)
end
print("blades redrawn:", painted)
print(wait(0))

--@ chunk 323
print(wait(14*24*60))
for _, p in ipairs({{205,672},{205,690},{270,590},{280,596},{300,600},{130,670}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk (finishing)
-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end
when_dry(function() varnish{coats=0.4} end)
when_dry(function() cracks{} end)
