-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
print(table.concat(tubes(), ", "))

--@ chunk 2
canvas{size=710, aspect=1.42, linen={15, 13}, seed=1822,
 ground={{pile={{"red earth", 2}, {"yellow ochre", 3}, {"lead white", 1}}, um=120, apply="knife", texture=0.35},
         {pile={{"lead white", 8}, {"yellow ochre", 1}, {"raw umber", 0.3}}, um=60, apply="brush"}}}
print(W, H)

--@ chunk 3
h = pencil("2H")
-- horizon / far hills
h:sketch({{0,428},{80,422},{160,425},{230,417},{300,420},{380,424},{470,421},{560,426},{640,430},{720,428},{820,423},{900,418},{1000,421}}, {pressure=0.25})
-- meadow edge line (near land meets flat)
h:sketch({{0,445},{200,443},{400,446},{600,444},{800,447},{1000,445}}, {pressure=0.2})
-- pools
h:sketch({{380,478},{460,474},{560,476},{620,481}}, {pressure=0.2})
h:sketch({{650,500},{760,495},{870,499}}, {pressure=0.2})
h:sketch({{90,470},{170,467},{240,471}}, {pressure=0.2})
-- foreground bank edge
h:sketch({{0,575},{120,570},{260,578},{380,590},{520,600},{680,598},{820,588},{1000,578}}, {pressure=0.3})
-- path
h:sketch({{160,704},{230,660},{300,620},{380,598}}, {pressure=0.25})
h:sketch({{280,704},{320,660},{360,625},{410,600}}, {pressure=0.25})
-- oak trunk
hb = pencil("HB")
hb:line({{285,572},{290,520},{296,470},{300,420},{298,380}}, {pressure=0.4})
hb:line({{318,572},{314,520},{312,470},{314,420},{318,380}}, {pressure=0.4})
-- main limbs
hb:line({{300,390},{270,330},{240,280},{200,240}}, {pressure=0.35})
hb:line({{312,385},{335,320},{360,260},{390,220}}, {pressure=0.35})
hb:line({{305,380},{310,300},{300,220},{305,140}}, {pressure=0.35})  -- dead top
hb:line({{245,285},{200,300},{160,300}}, {pressure=0.3})
-- crown masses (loose)
h:sketch({{150,330},{170,270},{220,230},{280,210},{350,200},{420,220},{450,270},{440,330},{390,370},{320,390},{240,380},{170,360},{150,330}}, {pressure=0.2})
-- town
h:sketch({{620,432},{640,425},{700,425},{720,432}}, {pressure=0.2})
h:line({{668,428},{668,398},{670,386}}, {pressure=0.35})  -- spire
h:line({{700,430},{700,410}}, {pressure=0.3})
-- figures
hb:line({{575,600},{572,570},{574,545},{577,535}}, {pressure=0.4})
hb:line({{596,601},{594,572},{592,548},{593,538}}, {pressure=0.4})
-- moon
h:sketch({{770,160},{760,172},{762,186},{772,194}}, {pressure=0.25})

--@ chunk 4
skyTop = pile{{"lead white", 3}, {"smalt", 3}, {"cobalt blue", 0.6}, {"raw umber", 0.1}, medium=0.25}
skyMid = pile{{"lead white", 6}, {"pale smalt", 2}, {"smalt", 0.5}, medium=0.25}
skyLow = pile{{"lead white", 8}, {"pale smalt", 0.6}, {"chrome yellow", 0.25}, medium=0.25}
skyGlow = pile{{"lead white", 8}, {"chrome yellow", 0.9}, {"vermilion", 0.12}, {"yellow ochre", 0.2}, medium=0.25}
skyGlowHot = pile{{"lead white", 7}, {"chrome yellow", 1.4}, {"vermilion", 0.25}, medium=0.25}
local b1 = rect(0, 0, 1000, 130):soften(25)
local b2 = rect(0, 110, 1000, 150):soften(25)
local b3 = rect(0, 250, 1000, 110):soften(25)
local b4 = rect(0, 350, 1000, 100)
work(b1, {hand="broad", pile=skyTop, angle=0, coverage=2.5, fill=true, angle_jitter=0.05})
work(b2, {hand="broad", pile=skyMid, angle=0, coverage=2.5, fill=true, angle_jitter=0.05})
work(b3, {hand="broad", pile=skyLow, angle=0, coverage=2.5, fill=true, angle_jitter=0.05})
work(b4, {hand="broad", pile=skyGlow, angle=0, coverage=2.5, fill=true, angle_jitter=0.05})
work(ellipse(640, 415, 260, 45):soften(30), {hand="body", pile=skyGlowHot, angle=0, coverage=2, fill=true})

--@ chunk 5
local sky = rect(0, 0, 1000, 450)
blend(sky, {angle=0, coverage=2})
blend(rect(0, 80, 1000, 70):soften(20), {angle=0.0, coverage=2})
blend(rect(0, 220, 1000, 60):soften(20), {angle=0.0, coverage=2})
blend(rect(0, 320, 1000, 60):soften(20), {angle=0.0, coverage=2})
print(drying(500, 50), drying(500, 400))

--@ chunk 6
skyDeep = pile{{"lead white", 2}, {"smalt", 3}, {"cobalt blue", 1.2}, medium=0.2}
work(rect(0, 0, 1000, 110):soften(30), {hand="broad", pile=skyDeep, angle=0, coverage=2, fill=true, angle_jitter=0.04})
skyGreenish = pile{{"lead white", 8}, {"pale smalt", 1}, {"chrome yellow", 0.35}, medium=0.2}
work(rect(0, 230, 1000, 90):soften(25), {hand="broad", pile=skyGreenish, angle=0, coverage=1.5, fill=true})
blend(rect(0, 0, 1000, 200), {angle=0, coverage=2})
blend(rect(0, 190, 1000, 180):soften(10), {angle=0, coverage=2.5})

--@ chunk 7
skyMid2 = pile{{"lead white", 5}, {"smalt", 1.6}, {"pale smalt", 1}, {"cobalt blue", 0.3}, medium=0.2}
work(rect(0, 95, 1000, 90):soften(35), {hand="broad", pile=skyMid2, angle=0, coverage=1.8, fill=true, angle_jitter=0.04})
skyMid3 = pile{{"lead white", 7}, {"pale smalt", 1.4}, {"chrome yellow", 0.08}, medium=0.2}
work(rect(0, 175, 1000, 70):soften(30), {hand="broad", pile=skyMid3, angle=0, coverage=1.5, fill=true, angle_jitter=0.04})
blend(rect(0, 40, 1000, 250), {angle=0, coverage=3})

--@ chunk 8
print(wait(22*60)); print(drying(500,50), drying(500,400), drying(500,250))

--@ chunk 9
hillN = noise{seed=7, octaves=3, period=160}
function hillY(x) return 424 - 8*math.sin(x/150+1) - 5*math.sin(x/53) + 4*hillN(x, 0) - 7*math.exp(-((x-880)/90)^2) end
function flatY(x) return 441 + 2*math.sin(x/70) end
function bankY(x) return 578 + 16*math.sin((x-120)/280) - 6*math.cos(x/90) + (x>300 and x<700 and 8 or 0) end
hillsM = below(function(x) return hillY(x) end) - below(function(x) return flatY(x) end)
landM = below(function(x) return flatY(x) end)
-- river (in perspective: thin bands near horizon)
riverTop = {{1000,470},{900,468},{800,466},{700,463},{600,460},{520,457},{450,455},{400,454}}
riverBot = {{400,456},{450,459},{520,464},{600,471},{700,478},{800,486},{900,494},{1000,500}}
local pts = {}
for _, p in ipairs(riverTop) do pts[#pts+1] = p end
for _, p in ipairs(riverBot) do pts[#pts+1] = p end
riverM = poly(pts, true)
bankM = below(function(x) return bankY(x) end)
meadowM = landM - bankM
hillP = pile{{"lead white", 5}, {"smalt", 2}, {"vermilion", 0.25}, {"raw umber", 0.3}, medium=0.2}
work(hillsM:grow(2), {hand="body", pile=hillP, angle=0, coverage=2.5, fill=true, clip=true, length={20,50}})

--@ chunk 10
meadFar = pile{{"lead white", 2}, {"green earth", 3}, {"smalt", 1}, {"raw umber", 1}, medium=0.15}
meadMid = pile{{"lead white", 1}, {"green earth", 3}, {"yellow ochre", 1.5}, {"raw umber", 1.2}, {"Prussian blue", 0.08}, medium=0.15}
meadNear = pile{{"green earth", 3}, {"yellow ochre", 1}, {"raw umber", 2}, {"Prussian blue", 0.12}, medium=0.15}
bankP = pile{{"raw umber", 3}, {"green earth", 2}, {"bone black", 0.6}, {"Prussian blue", 0.15}, {"yellow ochre", 0.5}, medium=0.15}
local far = (meadowM * rect(0, 430, 1000, 42):soften(8)) - riverM
local mid = (meadowM * rect(0, 465, 1000, 60):soften(10)) - riverM
local near = meadowM * rect(0, 515, 1000, 120):soften(10)
work(far, {hand="body", pile=meadFar, angle=0, coverage=2.5, fill=true, length={20,50}, clip=true})
work(mid, {hand="body", pile=meadMid, angle=0.02, coverage=2.5, fill=true, length={20,50}, clip=true})
work(near, {hand="body", pile=meadNear, angle=0.03, coverage=2.5, fill=true, length={20,60}, clip=true})
work(bankM, {hand="broad", pile=bankP, angle=-0.1, coverage=2.5, fill=true})

--@ chunk 11
meadLight = pile{{"green earth", 3}, {"yellow ochre", 2}, {"lead white", 1.5}, {"raw umber", 0.7}, medium=0.15}
meadLight2 = pile{{"green earth", 3}, {"yellow ochre", 1.5}, {"lead white", 0.6}, {"raw umber", 1.2}, medium=0.15}
local mA = (meadowM * rect(0, 488, 1000, 45):soften(10)) - riverM
local mB = meadowM * rect(0, 525, 1000, 80):soften(10)
work(mA, {hand="body", pile=meadLight, angle=0.0, coverage=2.5, fill=true, length={25,60}, clip=true})
work(mB, {hand="body", pile=meadLight2, angle=0.0, coverage=2.5, fill=true, length={25,60}, clip=true})
blend(meadowM * rect(0, 460, 1000, 140), {angle=0, coverage=1.5})

--@ chunk 12
bankPts = {{0,556},{80,553},{160,558},{260,568},{360,578},{460,584},{560,587},{660,586},{760,580},{860,574},{940,569},{1000,566}}
bankM2 = below(bankPts):roughen(2.5, 12, 3)
local add = bankM2 - bankM
work(add:grow(1), {hand="body", pile=bankP, angle=0, coverage=3, fill=true, length={15,40}, clip=true})
work(bankM2 * rect(0, 540, 1000, 80), {hand="body", pile=bankP, angle=-0.05, coverage=1.5, fill=true, length={20,50}})

--@ chunk 13
print(wait(30*60)); for _,p in ipairs({{500,450},{500,500},{500,560},{500,650},{500,420}}) do print(p[2], drying(p[1],p[2])) end

--@ chunk 14
print(wait(20*60)); for _,p in ipairs({{500,450},{500,500},{500,560}}) do print(p[2], drying(p[1],p[2])) end

--@ chunk 15
print(wait(36*60)); for _,p in ipairs({{500,450},{500,500},{500,560},{200,540}}) do print(p[2], drying(p[1],p[2])) end

--@ chunk 16
riverA = poly({{1000,472},{900,471},{800,472},{720,474},{684,477},{690,482},{740,486},{840,492},{940,497},{1000,499}}, true)
riverB = poly({{360,459},{450,457},{560,457},{640,460},{690,465},{700,472},{690,476},{660,468},{600,464},{520,462},{430,461},{360,461}}, true)
riverC = poly({{110,453},{180,452},{260,453},{262,455},{180,455},{112,455}}, true)
riverAll = riverA + riverB + riverC
riverWarm = pile{{"lead white", 7}, {"chrome yellow", 0.9}, {"vermilion", 0.08}, {"yellow ochre", 0.3}, medium=0.1}
riverCool = pile{{"lead white", 7}, {"chrome yellow", 0.3}, {"pale smalt", 0.8}, {"raw umber", 0.15}, medium=0.1}
work(riverB + riverC, {hand="detail", pile=riverWarm, angle=0, coverage=3, fill=true, length={8,25}})
work(riverA * rect(0, 460, 1000, 28), {hand="detail", pile=riverWarm, angle=0, coverage=3, fill=true, length={8,30}})
work(riverA * rect(0, 486, 1000, 20), {hand="detail", pile=riverCool, angle=0, coverage=3, fill=true, length={8,30}})
blend(riverA, {angle=0, coverage=1.2, tool="flat 6"})

--@ chunk 17
barkDark = pile{{"raw umber", 3}, {"bone black", 1}, {"green earth", 1}, {"lead white", 0.3}, medium=0.1}
oakLimbs = {
 {pts={{296,592},{299,560},{301,520},{303,480},{300,440},{303,405}}, w={40,31,27,25,24,22}},
 {pts={{300,405},{282,388},{262,372},{240,352},{214,322},{182,302},{150,294}}, w={17,14,12,10,8,6,4}},
 {pts={{306,402},{322,385},{338,366},{352,334},{378,300},{410,281},{446,271}}, w={16,13,11,10,7,5,3.5}},
 {pts={{303,400},{308,370},{310,340},{303,292},{311,240},{305,192},{312,142},{308,110}}, w={14,12,10,9,7,5,3.2,1.8}},
 {pts={{298,452},{275,441},{252,434},{222,422},{188,418},{160,404}}, w={10,8,7,6,4.5,3}},
 {pts={{318,356},{340,330},{360,295},{358,252},{374,212}}, w={8,7,6,4.5,3}},
 {pts={{240,352},{234,300},{246,252},{232,202},{241,168}}, w={7,6,5,3.5,2}},
 {pts={{380,300},{396,252},{418,212},{431,180}}, w={6,5,3.5,2}},
 {pts={{214,322},{190,340},{168,352},{146,350}}, w={6,5,3.5,2.5}},
 {pts={{352,334},{385,340},{418,342},{452,330}}, w={6,5,3.5,2.5}},
}
oakM = nil
for _, L in ipairs(oakLimbs) do
  local r = ribbon(L.pts, L.w)
  oakM = oakM and (oakM + r) or r
end
-- root flare
oakM = oakM + poly({{268,596},{284,580},{296,560},{312,560},{322,582},{336,597}}, true)
work(oakM, {hand="detail", pile=barkDark, coverage=3, fill=true, angle=function(x,y) return -1.4 end, length={6,18}})

--@ chunk 18
barkD = pile{{"raw umber", 2}, {"bone black", 1.5}, {"green earth", 1}, {"Prussian blue", 0.1}, medium=0.1}
gn = noise{seed=31, octaves=3, period=40}
function spline(pts, step)
  local out = {}
  local n = #pts
  for i = 1, n-1 do
    local p0 = pts[math.max(1,i-1)]; local p1 = pts[i]; local p2 = pts[i+1]; local p3 = pts[math.min(n,i+2)]
    local seg = math.sqrt((p2[1]-p1[1])^2 + (p2[2]-p1[2])^2)
    local k = math.max(2, math.floor(seg/step))
    for j = 0, k-1 do
      local t = j/k; local t2, t3 = t*t, t*t*t
      local x = 0.5*((2*p1[1]) + (-p0[1]+p2[1])*t + (2*p0[1]-5*p1[1]+4*p2[1]-p3[1])*t2 + (-p0[1]+3*p1[1]-3*p2[1]+p3[1])*t3)
      local y = 0.5*((2*p1[2]) + (-p0[2]+p2[2])*t + (2*p0[2]-5*p1[2]+4*p2[2]-p3[2])*t2 + (-p0[2]+3*p1[2]-3*p2[2]+p3[2])*t3)
      out[#out+1] = {x, y, i, t}
    end
  end
  out[#out+1] = {pts[n][1], pts[n][2], n-1, 1}
  return out
end
function limbMask(L, grow)
  local sp = spline(L.pts, 3)
  local P, Wd = {}, {}
  for _, q in ipairs(sp) do
    local w = lerp(L.w[q[3]], L.w[q[3]+1] or L.w[q[3]], q[4]) * (grow or 1)
    w = w * (1 + 0.18*gn(q[1], q[2]))
    P[#P+1] = {q[1] + 1.2*gn(q[2], q[1]), q[2]}; Wd[#Wd+1] = w
  end
  return ribbon(P, Wd)
end
-- gnarlier main limbs: nudge points
oakLimbs[1].pts = {{296,594},{298,566},{302,530},{300,495},{305,462},{300,430},{304,402}}
oakLimbs[1].w = {46,33,28,27,26,25,23}
oakLimbs[2].pts = {{300,405},{286,392},{270,382},{256,364},{238,352},{226,330},{206,318},{182,305},{164,300},{148,292}}
oakLimbs[2].w = {19,16,14,12,11,9,8,6,5,4}
oakLimbs[3].pts = {{306,402},{324,388},{334,372},{344,356},{352,334},{366,322},{380,302},{402,290},{420,278},{446,272}}
oakLimbs[3].w = {18,15,13,12,11,9,8,6,5,3.5}
oakLimbs[4].pts = {{303,400},{309,372},{306,346},{312,318},{303,290},{309,262},{313,238},{304,212},{308,190},{314,160},{307,134},{309,110}}
oakLimbs[4].w = {15,13,12,11,10,9,8,6.5,5,4,3,2}
oakM2 = nil
for _, L in ipairs(oakLimbs) do
  local r = limbMask(L, 1.1)
  oakM2 = oakM2 and (oakM2 + r) or r
end
oakM2 = oakM2 + poly({{262,600},{280,584},{292,562},{314,562},{324,584},{342,600}}, true)
oakM2 = oakM2:roughen(1.2, 8, 5)
work(oakM2, {hand="detail", pile=barkD, coverage=3, fill=true, angle=-1.45, length={6,18}})

--@ chunk 19
branches = {}
function grow(x, y, ang, len, w, depth, crook)
  local pts = {{x, y}}
  local nseg = math.max(2, math.floor(len / 9))
  local sl = len / nseg
  local cx, cy, a = x, y, ang
  local ws = {w}
  for i = 1, nseg do
    a = a + randn(0, crook or 0.3)
    -- mild tendency: oak limbs spread and turn up at ends
    a = a + 0.06 * (-math.pi/2 - a) * (i/nseg)
    cx = cx + sl * math.cos(a); cy = cy + sl * math.sin(a)
    pts[#pts+1] = {cx, cy}
    ws[#ws+1] = w * (1 - 0.55 * i / nseg)
    if depth > 0 and i < nseg and rand(0,1) < (0.25 + 0.4 * i/nseg) then
      local side = (rand(0,1) < 0.5) and -1 or 1
      grow(cx, cy, a + side * rand(0.45, 1.1), len * rand(0.45, 0.7), ws[#ws] * rand(0.55, 0.75), depth - 1, crook)
    end
  end
  branches[#branches+1] = {pts=pts, w=ws}
  if depth > 0 then
    grow(cx, cy, a + rand(-0.5, 0.5), len * rand(0.4, 0.6), ws[#ws] * 0.8, depth - 1, crook)
  end
end
-- living branches off the boughs, toward crown edges
local seeds = {
 {150,294, math.pi+0.25, 50, 3.5, 3}, {182,304, math.pi+0.7, 45, 3.5, 2}, {206,318, math.pi+1.2, 40, 3, 2},
 {226,330, -2.2, 40, 3.5, 2}, {160,404, math.pi-0.1, 45, 2.8, 2}, {188,418, math.pi+0.9, 35, 2.5, 2},
 {222,422, -2.0, 40, 3, 2}, {252,434, -1.9, 35, 2.8, 2},
 {446,272, -0.25, 45, 3.2, 3}, {420,278, -0.9, 40, 3, 2}, {402,290, 0.3, 45, 3, 2}, {366,322, -0.4, 40, 3, 2},
 {452,330, 0.1, 40, 2.6, 2}, {418,342, 0.6, 35, 2.5, 2}, {385,340, 0.9, 30, 2.5, 2},
 {146,350, math.pi+0.2, 35, 2.4, 2}, {168,352, math.pi-0.5, 30, 2.3, 2},
 {330,380, -0.2, 45, 3.5, 2}, {275,385, math.pi+0.1, 40, 3.5, 2},
 {346,356, 0.5, 40, 3.5, 2}, {262,372, math.pi-0.4, 40, 3.2, 2},
}
for _, s in ipairs(seeds) do grow(s[1], s[2], s[3], s[4], s[5], s[6], 0.32) end
-- dead: short broken stubs on the stag-horns
local dead = {{309,262,-0.6,22,3,0},{304,212,-2.6,18,2.5,0},{312,160,-0.5,14,2,0},{246,252,-0.4,18,2.5,1},{234,300,-2.8,15,2.5,0},
  {396,252,-2.4,16,2.2,0},{418,212,0.1,15,2,0},{358,252,-2.6,14,2.4,0},{374,212,-0.3,12,2,0},{232,202,-2.5,12,2,0}}
for _, s in ipairs(dead) do grow(s[1], s[2], s[3], s[4], s[5], s[6], 0.25) end
rig = brush{kind="round", width=5, point=1}
rig:load(barkD, 0.9)
local n = 0
for _, B in ipairs(branches) do
  local w0 = B.w[1]; local w1 = B.w[#B.w]
  if w0 > 3 then
    work(ribbon(B.pts, B.w), {hand="detail", pile=barkD, coverage=2.5, fill=true, length={4,12}})
  else
    if n % 4 == 0 then rig:load(barkD, 0.8) end
    rig:stroke(B.pts, {pressure={rig:pressure_for(w0), rig:pressure_for(math.max(0.4, w1))}, ramps={0.02, 0.3}})
  end
  n = n + 1
end
print(#branches)

--@ chunk 20
local firstDead
for i, B in ipairs(branches) do if B.pts[1][1] == 309 and B.pts[1][2] == 262 then firstDead = i; break end end
print("firstDead", firstDead, #branches)
local cm = nil
local k = 0
for i = 1, firstDead - 1 do
  local B = branches[i]
  local e = B.pts[#B.pts]
  if e[2] > 225 and e[2] < 450 then
    k = k + 1
    local rx = rand(9, 20); local ry = rx * rand(0.5, 0.75)
    local el = ellipse(e[1] + rand(-3,3), e[2] - ry*0.3, rx, ry)
    cm = cm and (cm + el) or el
  end
end
-- larger hand-placed masses along the living boughs
local masses = {{175,300,42,24},{215,285,30,18},{140,330,26,16},{195,345,34,18},{250,330,26,16},
  {420,262,40,22},{455,300,30,16},{395,315,34,18},{350,300,22,14},{430,340,30,14},
  {185,405,36,16},{240,405,30,14},{150,395,22,12},{345,375,26,14},{300,360,18,12},{270,395,20,12}}
for _, m in ipairs(masses) do cm = cm + ellipse(m[1], m[2], m[3], m[4]) end
foliM = cm:roughen(6, 14, 11):roughen(2.5, 5, 12)
print(k)

--@ chunk 21
foliDark = pile{{"Prussian blue", 0.5}, {"raw umber", 2}, {"yellow ochre", 1}, {"bone black", 0.5}, {"green earth", 1}, medium=0.1}
stipple(foliM, {pile=foliDark, width=3.2, coverage=2.2, cluster={0.6, 6}, pressure={0.4, 0.8}, feather=0.3, dips={30, 0.7, 0}})

--@ chunk 22
treeFar = pile{{"lead white", 2}, {"smalt", 1.5}, {"green earth", 1}, {"raw umber", 1}, {"vermilion", 0.1}, medium=0.1}
townP = pile{{"lead white", 3}, {"smalt", 2}, {"vermilion", 0.3}, {"raw umber", 0.9}, medium=0.1}
-- tree lines on the far flat: bumpy crowns
local tl = nil
local function treeline(x0, x1, base, hmin, hmax, seed)
  local xs = uneven(math.floor((x1-x0)/5), x0, x1, 0.6, 0.4, seed)
  for _, x in ipairs(xs) do
    local h = rand(hmin, hmax); local r = h * rand(0.45, 0.7)
    local e = ellipse(x, base - h + r, r, h - r*0.3)
    tl = tl and (tl + e) or e
  end
  tl = tl + rect(x0, base - 3, x1 - x0, 5)
end
treeline(20, 150, 444, 5, 10, 1)
treeline(420, 600, 444, 6, 12, 2)
treeline(740, 830, 445, 5, 11, 3)
treeline(900, 990, 444, 4, 9, 4)
treeline(560, 740, 446, 4, 8, 5)
farTrees = tl:roughen(1, 3, 6)
work(farTrees, {hand="detail", pile=treeFar, coverage=3, fill=true, angle=-1.5, length={3,8}, clip=true})
-- town: church with a tall spire, a second tower, roofs
local town = poly({{612,446},{612,436},{622,430},{632,436},{640,436},{640,431},{650,425},{656,431},{661,431},{661,414},{664,412},{666,396},{668,381},{670,396},{672,412},{675,414},{675,431},{690,431},{690,428},{700,420},{700,408},{703,404},{706,408},{706,420},{716,428},{716,434},{726,434},{732,429},{738,434},{742,446}})
townM = town
work(townM, {hand="detail", pile=townP, coverage=3, fill=true, angle=-1.57, length={3,8}, clip=true})
-- poplars
local pop = nil
for _, p in ipairs({{600,446,30},{606,446,24},{748,446,26},{756,446,20},{540,446,22}}) do
  local e = poly({{p[1]-3, p[2]}, {p[1]-3.5, p[2]-p[3]*0.4}, {p[1], p[2]-p[3]}, {p[1]+3.5, p[2]-p[3]*0.4}, {p[1]+3, p[2]}}, true)
  pop = pop and (pop + e) or e
end
work(pop:roughen(0.8, 3, 9), {hand="detail", pile=treeFar, coverage=3, fill=true, angle=-1.57, length={3,8}, clip=true})

--@ chunk 23
bankWarm = pile{{"raw umber", 3}, {"yellow ochre", 1}, {"green earth", 2}, {"bone black", 0.4}, medium=0.1}
bankCool = pile{{"raw umber", 2}, {"green earth", 2}, {"Prussian blue", 0.2}, {"bone black", 0.6}, medium=0.1}
local bk = bankM2 - oakM2
work(bk * rect(0, 540, 1000, 90), {hand="body", pile=bankWarm, angle=-0.05, coverage=1.6, fill=true, length={15,40}})
work(bk * rect(0, 620, 1000, 90), {hand="body", pile=bankCool, angle=0.05, coverage=1.6, fill=true, length={15,40}})
pathPts = {{395,704},{420,680},{452,660},{490,642},{530,626},{565,612},{592,603}}
pathW   = {70, 58, 44, 32, 22, 13, 7}
pathM = ribbon(pathPts, pathW):roughen(3, 10, 21)
pathP = pile{{"yellow ochre", 2}, {"raw umber", 1.5}, {"lead white", 1.2}, {"green earth", 0.3}, medium=0.1}
work(pathM, {hand="body", pile=pathP, coverage=2, fill=true, angle=function(x,y) return -0.5 end, length={10,30}, edge="soft"})

--@ chunk 24
bankOl = pile{{"raw umber", 2.5}, {"green earth", 2.5}, {"yellow ochre", 0.8}, {"bone black", 0.5}, {"Prussian blue", 0.1}, medium=0.1}
local bk = bankM2 - oakM2 - pathM
work(bk * rect(0, 540, 1000, 110), {hand="body", pile=bankOl, angle=-0.03, coverage=2, fill=true, length={20,50}})
pathDark = pile{{"yellow ochre", 1.5}, {"raw umber", 2}, {"lead white", 0.6}, {"green earth", 0.6}, {"bone black", 0.15}, medium=0.1}
work(pathM, {hand="body", pile=pathDark, coverage=1.5, fill=true, angle=-0.5, length={10,30}})
blend(bankM2 * rect(0, 560, 1000, 144), {angle=0, coverage=1.5, clip=false})

--@ chunk 25
blend((bankM2 - oakM2:grow(6)) * rect(0, 615, 1000, 75), {angle=0.1, coverage=2.5})
local flare = poly({{258,600},{272,592},{284,578},{290,556},{312,556},{318,578},{330,592},{346,601},{330,603},{300,599},{275,603}}, true):roughen(1.5, 6, 41)
work(flare, {hand="detail", pile=barkD, coverage=3, fill=true, angle=-1.4, length={4,10}})

--@ chunk 26
print(wait(40*60)); for _,p in ipairs({{300,300},{500,600},{500,680},{200,300}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 27
print(wait(48*60)); for _,p in ipairs({{300,300},{500,600},{500,680},{200,300},{300,500}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 28
bankTop = pile{{"raw umber", 2.2}, {"green earth", 2.5}, {"yellow ochre", 1}, {"bone black", 0.35}, {"lead white", 0.2}, medium=0.08}
bankMidP = pile{{"raw umber", 2.5}, {"green earth", 2}, {"yellow ochre", 0.5}, {"bone black", 0.6}, {"Prussian blue", 0.1}, medium=0.08}
bankLow = pile{{"raw umber", 2.5}, {"green earth", 1.5}, {"bone black", 1}, {"Prussian blue", 0.15}, medium=0.08}
local bn = noise{seed=51, octaves=3, period=90}
local z1 = below(function(x) return 612 + 14*bn(x, 1) end)
local z2 = below(function(x) return 660 + 16*bn(x, 2) end)
local bk = bankM2 - oakM2:shrink(1)
local ang = function(x, y) return -1.35 + 0.25*bn(x, y) end
work(bk - z1, {hand="body", pile=bankTop, angle=ang, coverage=2.5, fill=true, length={10,24}, edge="firm"})
work(bk * z1 - z2, {hand="body", pile=bankMidP, angle=ang, coverage=2.5, fill=true, length={12,28}})
work(bk * z2, {hand="body", pile=bankLow, angle=ang, coverage=2.5, fill=true, length={14,32}})

--@ chunk 29
local bk = bankM2 - oakM2:shrink(1)
blend(bk, {angle=-1.4, coverage=2.5})
blend(bk, {angle=-0.1, coverage=1.5})

--@ chunk 30
flare2 = poly({{262,598},{276,590},{286,578},{290,560},{292,548},{314,548},{316,560},{320,578},{332,590},{348,598},{336,601},{318,597},{300,600},{284,598},{270,602}}, true):roughen(1.2, 5, 43)
local halo = ellipse(305, 585, 60, 22) - flare2
work(halo, {hand="detail", pile=bankTop, coverage=3, fill=true, angle=-1.4, length={4,10}})
work(flare2, {hand="detail", pile=barkD, coverage=3, fill=true, angle=-1.5, length={4,10}})
-- a couple of surface roots
rig:reload(barkD, 0.8)
rig:stroke({{280,592},{266,597},{252,600}}, {pressure={rig:pressure_for(4), rig:pressure_for(0.8)}})
rig:stroke({{330,592},{345,598},{360,601}}, {pressure={rig:pressure_for(4), rig:pressure_for(0.8)}})

--@ chunk 31
print(wait(50*60)); for _,p in ipairs({{300,300},{300,590},{500,680},{420,260},{300,500}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 32
foliDark2 = pile{{"raw umber", 2.5}, {"green earth", 1}, {"yellow ochre", 1}, {"Prussian blue", 0.25}, {"bone black", 0.6}, medium=0.1}
stipple(foliM, {pile=foliDark2, width=3, coverage=1.6, cluster={0.5, 5}, pressure={0.4, 0.8}, dips={30, 0.7, 0}})
-- leaves scattered beyond the clump edges
local fringe = foliM:grow(7) - foliM
stipple(fringe, {pile=foliDark2, width=2.2, coverage=0.5, cluster={0.7, 4}, pressure={0.3, 0.7}, feather=0.5, dips={30, 0.6, 0}})

--@ chunk 33
skyHoleHi = pile{{"lead white", 8}, {"pale smalt", 0.6}, {"chrome yellow", 0.15}, medium=0.05}
skyHoleLo = pile{{"lead white", 8}, {"chrome yellow", 0.7}, {"yellow ochre", 0.15}, medium=0.05}
local inner = foliM:shrink(5) - oakM2:grow(2)
stipple(inner * rect(0, 0, 1000, 310), {pile=skyHoleHi, width=2.6, coverage=0.18, cluster={0.9, 4}, pressure={0.3, 0.7}, dips={12, 0.6, 0}})
stipple(inner * rect(0, 310, 1000, 200), {pile=skyHoleLo, width=2.6, coverage=0.18, cluster={0.9, 4}, pressure={0.3, 0.7}, dips={12, 0.6, 0}})

--@ chunk 34
print(wait(24*60))
local inner = foliM:shrink(4) - oakM2:grow(2)
stipple(inner, {pile=foliDark2, width=3, coverage=1.3, cluster={0.3, 5}, pressure={0.45, 0.85}, dips={30, 0.7, 0}})

--@ chunk 35
local firstDead = 173
local sh = nil
for i = 1, firstDead - 1 do
  local B = branches[i]
  local e = B.pts[#B.pts]
  if e[2] > 225 and e[2] < 450 then
    local rx = 9 + (i * 7919 % 11)
    local el = ellipse(e[1], e[2] + 4, rx, rx * 0.55)
    sh = sh and (sh + el) or el
  end
end
local masses = {{175,300,42,24},{215,285,30,18},{140,330,26,16},{195,345,34,18},{250,330,26,16},
  {420,262,40,22},{455,300,30,16},{395,315,34,18},{350,300,22,14},{430,340,30,14},
  {185,405,36,16},{240,405,30,14},{150,395,22,12},{345,375,26,14},{300,360,18,12},{270,395,20,12}}
for _, m in ipairs(masses) do sh = sh + ellipse(m[1], m[2] + 7, m[3] * 0.95, m[4] * 0.9) end
foliTops = (foliM - sh:roughen(3, 8, 77)) - oakM2
foliLight = pile{{"green earth", 2}, {"yellow ochre", 1}, {"lead white", 0.9}, {"raw umber", 1}, {"pale smalt", 0.3}, medium=0.1}
stipple(foliTops, {pile=foliLight, width=2.6, coverage=0.9, cluster={0.5, 4}, pressure={0.35, 0.75}, feather=0.4, dips={20, 0.6, 0}})
print(foliTops:area())

--@ chunk 36
local tz = foliM:grow(10) + oakM2:grow(5)
for _, B in ipairs(branches) do
  local ws = {}
  for i, w in ipairs(B.w) do ws[i] = w + 6 end
  tz = tz + ribbon(B.pts, ws)
end
treeZone = tz
skyStip = above(function(x) return hillY(x) - 3 end) - treeZone
print(skyStip:area())

--@ chunk 37
skyTopS = pile{{"lead white", 2.5}, {"smalt", 3}, {"cobalt blue", 1}, medium=0.15}
skyMidS = pile{{"lead white", 4.5}, {"smalt", 1.8}, {"pale smalt", 1}, {"cobalt blue", 0.35}, medium=0.15}
stipple(skyStip * rect(0, 0, 1000, 150), {pile=skyTopS, width=4.5, coverage=function(x, y) return 1.3 * clamp((140 - y) / 110, 0, 1) end, cluster=0.1, pressure={0.35, 0.7}, feather=0.6, dips={40, 0.6, 0}})
stipple(skyStip * rect(0, 40, 1000, 190), {pile=skyMidS, width=4.5, coverage=function(x, y) return 1.1 * math.exp(-((y - 125) / 55)^2) end, cluster=0.1, pressure={0.35, 0.7}, feather=0.6, dips={40, 0.6, 0}})

--@ chunk 38
blend(skyStip * rect(0, 0, 1000, 245), {angle=0, coverage=2})

--@ chunk 39
local midB = pile{{"lead white", 3.5}, {"smalt", 2.4}, {"pale smalt", 0.8}, {"cobalt blue", 0.6}, medium=0.2}
stipple(skyStip * rect(0, 20, 1000, 140), {pile=midB, width=3, coverage=function(x, y) return 1.4 * math.exp(-((y - 80) / 28)^2) end, cluster=0, pressure={0.4, 0.8}, dips={40, 0.6, 0}})
blend(skyStip * rect(0, 0, 1000, 200), {angle=0, coverage=3, tool="badger 30"})

--@ chunk 40
print(wait(24*60))
cloudDusk = pile{{"lead white", 4}, {"smalt", 1.1}, {"vermilion", 0.35}, {"raw umber", 0.35}, medium=0.3}
cloudLit = pile{{"lead white", 5}, {"vermilion", 0.45}, {"chrome yellow", 0.6}, medium=0.2}
local strata = {
 {pts={{560,318},{640,314},{720,316},{800,312},{880,315},{960,311}}, w={2,4,5,4,5,2}},
 {pts={{600,334},{680,331},{760,333},{830,330}}, w={1.5,3.5,3,1.5}},
 {pts={{780,352},{850,349},{930,351},{1000,348}}, w={1.5,3,3.5,3}},
 {pts={{470,346},{540,343},{600,345}}, w={1,2.5,1.2}},
 {pts={{880,292},{940,289},{1000,290}}, w={1,2.5,2.5}},
 {pts={{0,322},{40,320},{80,322}}, w={2.5,3,1}},
}
local cm = nil
for _, s in ipairs(strata) do
  local r = ribbon(s.pts, s.w)
  cm = cm and (cm + r) or r
end
cloudM = (cm:roughen(1.2, 10, 61)) - treeZone
work(cloudM, {hand="detail", pile=cloudDusk, angle=0, coverage=2.5, fill=true, length={8,25}, clip=true})
-- lit lower edges
local lit = cloudM - cloudM:offset(-1.2)

--@ chunk 41
blend(cloudM:grow(5):soften(3), {angle=0, coverage=3, tool="badger 20"})

--@ chunk 42
moonP = pile{{"lead white", 8}, {"chrome yellow", 0.25}, {"yellow ochre", 0.05}}
local cx, cy, r = 835, 172, 7.5
local dx, dy = 195, -285
local l = math.sqrt(dx*dx + dy*dy); dx, dy = dx/l, dy/l
moonM = ellipse(cx, cy, r, r) - ellipse(cx + dx*r*0.5, cy + dy*r*0.5, r*1.02, r*1.02)
local b = brush{kind="round", width=3, point=0.6}
b:load(moonP, 0.9)
work(moonM, {hand="detail", pile=moonP, tool=b, coverage=4, fill=true, clip=true, length={2,5}})
print(moonM:area())

--@ chunk 43
reedP = pile{{"raw umber", 2}, {"green earth", 2}, {"smalt", 0.5}, {"lead white", 0.6}, {"bone black", 0.3}, medium=0.1}
reflP = pile{{"lead white", 3}, {"smalt", 1.2}, {"vermilion", 0.2}, {"raw umber", 0.6}, {"green earth", 0.3}, medium=0.3}
bankShadow = pile{{"raw umber", 2}, {"green earth", 1.5}, {"bone black", 0.4}, {"yellow ochre", 0.5}, medium=0.1}
-- far bank edges: thin dark lines on the top edges
farEdgeB = {{362,459.5},{450,457.5},{560,457.5},{640,460.5},{688,465.5}}
farEdgeA = {{700,474},{800,472.5},{900,471.5},{1000,472.5}}
farEdgeC = {{112,453.2},{180,452.3},{260,453.3}}
local fe = ribbon(farEdgeB, 1.6) + ribbon(farEdgeA, 2.2) + ribbon(farEdgeC, 1.1)
work(fe:roughen(0.5, 3, 71), {hand="detail", pile=reedP, angle=0, coverage=3, fill=true, clip=true, length={4,12}})
-- reflections of the far tree lines, just under the far bank
local refl = (riverA * rect(0, 470, 1000, 8) + riverB * rect(0, 456, 1000, 4)):roughen(0.8, 4, 72)
work(refl, {hand="detail", pile=reflP, angle=-1.57, coverage=1.2, clip=true, length={2,5}, pressure={0.2,0.4}})
-- near bank of reach A: dark cut edge
nearEdgeA = {{688,482},{740,486.5},{840,492.5},{940,497.5},{1000,499.5}}
nearEdgeB = {{430,461.5},{520,462.8},{600,465},{660,468.5}}
local ne = ribbon(nearEdgeA, 2.2) + ribbon(nearEdgeB, 1.4)
work(ne:roughen(0.6, 3, 73), {hand="detail", pile=bankShadow, angle=0, coverage=3, fill=true, clip=true, length={5,14}})

--@ chunk 44
meadL = pile{{"green earth", 2}, {"yellow ochre", 2}, {"lead white", 1.2}, {"raw umber", 0.5}, medium=0.15}
meadD = pile{{"green earth", 2.5}, {"raw umber", 1.5}, {"yellow ochre", 0.6}, {"Prussian blue", 0.08}, medium=0.15}
local mz = (meadowM - bankM2 - riverAll:grow(1.5) - oakM2:grow(2)) * rect(0, 462, 1000, 130)
-- mown strips: bands in perspective, thinner toward the horizon
local mn = noise{seed=91, octaves=2, period=120}
local strips = mask(function(x, y)
  local d = 1 / math.max(1, y - 438)            -- perspective
  local v = math.sin(1 / d * 0.0 + 1800 * d + 0.8*mn(x, y))
  return v > 0.25 and 1 or 0
end)
work(mz * strips, {hand="body", pile=meadL, angle=0, coverage=1.0, length={20,60}, pressure={0.25,0.5}, tool="filbert 5"})
work(mz - strips, {hand="body", pile=meadD, angle=0, coverage=0.7, length={20,60}, pressure={0.25,0.5}, tool="filbert 5"})

--@ chunk 45
local mzz = (meadowM - bankM2) * rect(0, 455, 1000, 140)
blend(mzz - riverAll - oakM2, {angle=0, coverage=3})
blend(mzz - riverAll - oakM2, {angle=0.05, coverage=2, tool="badger 25"})

--@ chunk 46
print(wait(48*60)); for _,p in ipairs({{300,480},{500,520},{600,462},{835,172}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 47
work(oakM2 * rect(0, 415, 1000, 150), {hand="detail", pile=barkD, coverage=3.5, fill=true, angle=-1.5, length={5,14}})
work(riverB + riverC, {hand="detail", pile=riverWarm, angle=0, coverage=3.5, fill=true, length={6,20}})
local fe = ribbon(farEdgeB, 1.2) + ribbon(farEdgeC, 0.9)
work(fe:roughen(0.4, 3, 81), {hand="detail", pile=reedP, angle=0, coverage=2, fill=true, clip=true, length={4,12}, pressure={0.2,0.5}})

--@ chunk 48
local mzone = (meadowM - bankM2:grow(1) - riverAll - oakM2:grow(1)) * rect(0, 452, 1000, 150)
meadFarL = pile{{"green earth", 2}, {"yellow ochre", 1.2}, {"lead white", 1.4}, {"raw umber", 0.7}, {"smalt", 0.3}, medium=0.15}
meadFarD = pile{{"green earth", 2}, {"raw umber", 1.2}, {"lead white", 0.6}, {"smalt", 0.4}, medium=0.15}
meadNearL = pile{{"green earth", 2}, {"yellow ochre", 1.6}, {"lead white", 0.8}, {"raw umber", 0.7}, medium=0.15}
meadNearD = pile{{"green earth", 2.2}, {"raw umber", 1.4}, {"yellow ochre", 0.5}, {"Prussian blue", 0.05}, medium=0.15}
local far = mzone * rect(0, 450, 1000, 45)
local near = mzone - rect(0, 450, 1000, 45)
local fn = noise{seed=101, octaves=3, period=60, stretch={0, 4}}
local farL = far * mask(function(x, y) return fn(x, y) > 0 and 1 or 0 end)
local nearL = near * mask(function(x, y) return fn(x, y) > -0.1 and 1 or 0 end)
work(farL, {hand="hatch", pile=meadFarL, angle=0, coverage=1.2, length={4,9}, pressure={0.2,0.4}, clip=true, angle_jitter=0.04})
work(far - farL, {hand="hatch", pile=meadFarD, angle=0, coverage=1.2, length={4,9}, pressure={0.2,0.4}, clip=true, angle_jitter=0.04})
work(nearL, {hand="hatch", pile=meadNearL, angle=0, coverage=1.1, length={6,14}, pressure={0.25,0.5}, clip=true, angle_jitter=0.06})
work(near - nearL, {hand="hatch", pile=meadNearD, angle=0, coverage=1.1, length={6,14}, pressure={0.25,0.5}, clip=true, angle_jitter=0.06})

--@ chunk 49
local mzone = (meadowM - bankM2:grow(1) - riverAll - oakM2:grow(1)) * rect(0, 452, 1000, 150)
blend(mzone, {angle=0, coverage=3, ruler=true, length={120, 300}})
blend(mzone, {angle=0, coverage=2, ruler=true, length={60, 200}, tool="badger 20"})

--@ chunk 50
print(wait(48*60)); for _,p in ipairs({{500,520},{500,560},{300,480}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 51
willowD = pile{{"green earth", 2}, {"raw umber", 1.4}, {"smalt", 0.4}, {"lead white", 0.5}, {"bone black", 0.15}, medium=0.1}
willowL = pile{{"lead white", 2}, {"green earth", 1}, {"pale smalt", 0.5}, {"yellow ochre", 0.4}, {"raw umber", 0.3}, medium=0.1}
willowTrunk = pile{{"raw umber", 2}, {"bone black", 0.8}, {"lead white", 0.4}, {"green earth", 0.4}, medium=0.1}
ditchP = pile{{"raw umber", 2}, {"green earth", 1.2}, {"bone black", 0.4}, medium=0.1}
willows = {}
local dn, df = 1/134, 1/42
for i = 0, 7 do
  local d = lerp(dn, df, (i/7)^0.9)
  local y = 440 + 1/d
  local x = 842 + (y - 574) * (140/92) + rand(-2, 2)
  local h = 0.36 * (y - 440) * rand(0.85, 1.1)
  willows[#willows+1] = {x=x, y=y, h=h}
end
-- ditch along them (a thin dark line with a glint)
local dpts = {}
for i = 0, 20 do local y = lerp(582, 480, i/20); dpts[#dpts+1] = {842 + (y - 574) * (140/92) - 6*(y-440)/134, y} end
local dw = {}
for i = 1, #dpts do dw[i] = lerp(3.2, 0.7, (i-1)/(#dpts-1)) end
ditchM = ribbon(dpts, dw):roughen(0.5, 3, 111)
work(ditchM, {hand="detail", pile=ditchP, coverage=3, fill=true, clip=true, length={4,10}})
local dg = {}
for i = 1, #dpts do dg[i] = dw[i]*0.35 end
work(ribbon(dpts, dg), {hand="detail", pile=riverWarm, coverage=1.5, clip=true, length={3,8}, pressure={0.15,0.3}})
-- trees, far first
for i = #willows, 1, -1 do
  local W = willows[i]; local x, y, h = W.x, W.y, W.h
  local trunk = poly({{x - 0.1*h, y}, {x - 0.08*h, y - 0.25*h}, {x - 0.11*h, y - 0.44*h}, {x + 0.12*h, y - 0.46*h}, {x + 0.09*h, y - 0.25*h}, {x + 0.11*h, y}}, true)
  local crown = ellipse(x + rand(-1,1)*0.05*h, y - 0.72*h, 0.34*h, 0.3*h):roughen(math.max(0.8, 0.05*h), math.max(3, 0.15*h), 200+i)
  W.trunk = trunk; W.crown = crown
  local sw = math.max(1.2, 0.07*h)
  stipple(crown, {pile=willowD, width=sw, coverage=2.6, cluster={0.4, sw*2}, pressure={0.4,0.8}, dips={30, 0.7, 0}})
  stipple(crown * rect(0, y - h, 1000, 0.25*h), {pile=willowL, width=sw*0.9, coverage=0.7, cluster={0.5, sw*2}, pressure={0.3,0.6}, feather=0.4})
  work(trunk - crown:shrink(0.1*h), {hand="detail", pile=willowTrunk, coverage=3, fill=true, clip=true, angle=-1.57, length={2, math.max(3, 0.2*h)}})
end

--@ chunk 52
local m = ditchM:grow(2) - rect(0,0,1000,0) ; for _,W in ipairs(willows) do m = m - W.trunk:grow(1) - W.crown:grow(1) end; blend(m, {angle=-0.9, coverage=4, tool="badger 12"}); blend(m, {angle=0, coverage=3, tool="badger 12"})

--@ chunk 53
print(wait(30*60))
willowD2 = pile{{"green earth", 2}, {"raw umber", 1.6}, {"smalt", 0.5}, {"lead white", 0.35}, {"bone black", 0.25}, medium=0.1}
willowSil = pile{{"lead white", 2.2}, {"green earth", 1}, {"pale smalt", 0.6}, {"yellow ochre", 0.3}, {"raw umber", 0.4}, medium=0.1}
willowTrunk2 = pile{{"raw umber", 1.6}, {"bone black", 1}, {"lead white", 0.5}, {"smalt", 0.3}, medium=0.1}
local rod = brush{kind="round", width=2.5, point=1}
for i = #willows, 1, -1 do
  local W = willows[i]; local x, y, h = W.x, W.y, W.h
  local crown = ellipse(x, y - 0.66*h, 0.42*h, 0.36*h):roughen(math.max(0.8, 0.06*h), math.max(3, 0.12*h), 300+i)
  W.crown2 = crown
  local sw = math.max(1.2, 0.07*h)
  stipple(crown, {pile=willowD2, width=sw, coverage=2.8, cluster={0.4, sw*2}, pressure={0.4,0.8}, dips={30, 0.7, 0}})
  -- upright rods rising from the head
  if h > 12 then
    rod:reload(willowD2, 0.7)
    local n = math.floor(h * 0.5)
    for k = 1, n do
      local a = -math.pi/2 + randn(0, 0.45)
      local L = h * rand(0.3, 0.55)
      local x0, y0 = x + rand(-0.12, 0.12)*h, y - 0.45*h
      rod:stroke({{x0, y0}, {x0 + 0.5*L*math.cos(a) , y0 + 0.5*L*math.sin(a)}, {x0 + L*math.cos(a+randn(0,0.1)), y0 + L*math.sin(a)}}, {pressure={rod:pressure_for(0.9), 0.0}, ramps={0.05, 0.5}})
      if k % 6 == 0 then rod:load(willowD2, 0.6) end
    end
  end
  stipple(crown - crown:offset(-math.max(1, 0.06*h)), {pile=willowSil, width=sw*0.8, coverage=0.5, cluster={0.5, sw*2}, pressure={0.3,0.6}, feather=0.4})
  local trunkVis = W.trunk - crown:shrink(0.05*h)
  work(trunkVis, {hand="detail", pile=willowTrunk2, coverage=3, fill=true, clip=true, angle=-1.57, length={2, math.max(3, 0.15*h)}})
end

--@ chunk 54
willowG = pile{{"green earth", 2}, {"yellow ochre", 0.6}, {"raw umber", 1.2}, {"Prussian blue", 0.08}, {"lead white", 0.4}, medium=0.1}
local rod = brush{kind="round", width=2, point=1}
local leafTips = nil
for i = #willows, 1, -1 do
  local W = willows[i]; local x, y, h = W.x, W.y, W.h
  if h > 9 then
    rod:reload(willowD2, 0.7)
    local n = math.floor(h * 0.7)
    for k = 1, n do
      local side = rand(-1, 1)
      local a = -math.pi/2 + side * 0.75 + randn(0, 0.12)
      local L = h * rand(0.5, 0.8)
      local x0, y0 = x + side * 0.1 * h, y - 0.46*h
      local bend = side * 0.15
      local x1, y1 = x0 + 0.5*L*math.cos(a), y0 + 0.5*L*math.sin(a)
      local x2, y2 = x0 + L*math.cos(a + bend), y0 + L*math.sin(a + bend)
      rod:stroke({{x0, y0}, {x1, y1}, {x2, y2}}, {pressure={rod:pressure_for(math.max(0.5, 0.05*h)), 0.0}, ramps={0.05, 0.6}})
      if k % 5 == 0 then rod:load(willowD2, 0.6) end
      if rand(0,1) < 0.7 then
        local e = ellipse((x1 + x2)/2, (y1 + y2)/2, 0.07*h, 0.12*h)
        leafTips = leafTips and (leafTips + e) or e
      end
    end
  end
end
if leafTips then
  local lt = leafTips:roughen(0.8, 3, 401)
  stipple(lt, {pile=willowG, width=1.6, coverage=1.3, cluster={0.5, 3}, pressure={0.3, 0.7}, feather=0.3})
  stipple(lt * rect(0, 0, 1000, 1000), {pile=willowSil, width=1.3, coverage=0.35, cluster={0.6, 3}, pressure={0.25, 0.5}, feather=0.5})
end

--@ chunk 55
local wm = nil
for _, W in ipairs(willows) do local m = W.crown2:grow(W.h*0.5) + W.trunk; wm = wm and (wm + m) or m end
willowZone = wm
local rA = riverA - willowZone
work(rA * rect(0, 462, 1000, 24), {hand="detail", pile=riverWarm, angle=0, coverage=3, fill=true, clip=true, length={6,20}})
work(rA * rect(0, 486, 1000, 20), {hand="detail", pile=riverCool, angle=0, coverage=3, fill=true, clip=true, length={6,20}})
work((ribbon(farEdgeA, 1.5)):roughen(0.4, 3, 121) - willowZone, {hand="detail", pile=reedP, angle=0, coverage=2, fill=true, clip=true, length={4,12}, pressure={0.2,0.5}})
work((ribbon(nearEdgeA, 1.6)):roughen(0.5, 3, 122) - willowZone, {hand="detail", pile=bankShadow, angle=0, coverage=2, fill=true, clip=true, length={4,12}, pressure={0.2,0.5}})
-- barge
hullP = pile{{"raw umber", 2}, {"bone black", 1.2}, {"red earth", 0.3}, medium=0.05}
sailP = pile{{"lead white", 2}, {"raw umber", 1.2}, {"red earth", 0.3}, {"yellow ochre", 0.6}, medium=0.05}
local bx, by = 905, 486
hullM = poly({{bx-24, by-3.5}, {bx+22, by-4}, {bx+25, by-6}, {bx+20, by+0.8}, {bx-20, by+0.8}, {bx-26, by-5.5}}, true)
sailM = poly({{bx-2, by-8}, {bx-4, by-46}, {bx+19, by-40}, {bx+17, by-9}}, false)
local sail2 = poly({{bx-4, by-8}, {bx-3, by-36}, {bx-18, by-10}}, false)
local mastM = ribbon({{bx-3, by-3}, {bx-4, by-58}}, {1.4, 0.6})
work(sailM + sail2, {hand="detail", pile=sailP, coverage=3, fill=true, clip=true, angle=-1.57, length={3,10}})
work(hullM, {hand="detail", pile=hullP, coverage=3, fill=true, clip=true, angle=0, length={3,10}})
work(mastM, {hand="detail", pile=hullP, coverage=3, fill=true, clip=true, angle=-1.57, length={3,10}})
-- reflection
local refl = poly({{bx-20, by+1}, {bx+20, by+1}, {bx+14, by+7}, {bx-14, by+7}}, true)
work(refl, {hand="detail", pile=reflP, coverage=1.5, clip=true, angle=-1.57, length={2,5}, pressure={0.2,0.4}})

--@ chunk 56
coatP = pile{{"bone black", 1}, {"Prussian blue", 0.3}, {"raw umber", 1}, {"green earth", 0.6}, {"lead white", 0.15}}
trouserP = pile{{"raw umber", 1.5}, {"lead white", 0.5}, {"bone black", 0.35}, {"yellow ochre", 0.2}}
hairP = pile{{"raw umber", 2}, {"red earth", 0.5}, {"bone black", 0.3}}
skinP = pile{{"lead white", 1}, {"red earth", 0.5}, {"yellow ochre", 0.5}, {"raw umber", 0.5}}
dressP = pile{{"red earth", 1.6}, {"raw umber", 0.9}, {"bone black", 0.3}, {"vermilion", 0.2}}
shawlP = pile{{"lead white", 2}, {"yellow ochre", 0.4}, {"raw umber", 0.5}, {"smalt", 0.15}}
local mx, fy = 596, 603
local legs = poly({{mx-6, 574}, {mx-5.5, fy-1}, {mx-7, fy+1}, {mx-2, fy+1}, {mx-1.5, 575}, {mx+1, 575}, {mx+2, fy+1}, {mx+6.5, fy+1}, {mx+5, fy-1}, {mx+5.5, 574}}, false)
local coat = poly({{mx-9, 537}, {mx-10.5, 555}, {mx-11.5, 578}, {mx+11.5, 579}, {mx+10.5, 555}, {mx+9.5, 537}, {mx+4, 533}, {mx-4, 533}}, true)
local head = ellipse(mx, 526, 4.6, 5.6)
local neck = rect(mx-2.6, 530, 5.2, 4.5)
local cap = ellipse(mx - 0.5, 521.5, 6.8, 2.6)
local wx = 571
local dress = poly({{wx-5, 548}, {wx-9, 570}, {wx-13, fy+1}, {wx+13, fy+1}, {wx+9, 570}, {wx+5, 548}}, true)
local bodice = poly({{wx-6.5, 534}, {wx-6, 550}, {wx+6, 550}, {wx+6.5, 534}, {wx+3, 531}, {wx-3, 531}}, true)
local shawl = poly({{wx-7.5, 533}, {wx-8, 545}, {wx, 553}, {wx+8, 545}, {wx+7.5, 533}, {wx+3, 530}, {wx-3, 530}}, true)
local whead = ellipse(wx, 524.5, 4.1, 5.1)
local bun = ellipse(wx, 527.5, 2.8, 2.4)
local arm = ribbon({{wx+6, 537}, {wx+11, 545}, {mx-8, 548}}, {3.4, 3, 2.8})
local d = function(m, p) work(m, {hand="detail", pile=p, coverage=3.5, fill=true, clip=true, angle=-1.57, length={2,6}}) end
d(legs, trouserP); d(coat, coatP); d(neck, skinP); d(head, hairP); d(cap, coatP)
d(dress, dressP); d(bodice, dressP); d(shawl, shawlP); d(arm, shawlP); d(whead, hairP); d(bun, hairP)
figM = legs + coat + head + neck + cap + dress + bodice + shawl + whead + arm
-- walking stick
local st = brush{kind="round", width=2, point=0.6}
st:load(hullP, 0.8)
st:stroke({{mx+10, 566}, {mx+13, 585}, {mx+15.5, fy+1}}, {pressure={st:pressure_for(1.0), st:pressure_for(0.9)}})

--@ chunk 57
print(wait(30*60)); for _,p in ipairs({{571,540},{596,555},{905,470},{300,480}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 58
function crestY(x)
  for i = 1, #bankPts - 1 do
    local a, b = bankPts[i], bankPts[i+1]
    if x >= a[1] and x <= b[1] then
      local t = (x - a[1]) / (b[1] - a[1]); t = t*t*(3-2*t)
      return lerp(a[2], b[2], t)
    end
  end
  return bankPts[#bankPts][2]
end
keepOut = figM:grow(2) + oakM2:grow(1) + flare2:grow(1)
local band = (bankM2 * above(function(x) return crestY(x) + 30 end)) - keepOut
crestP = pile{{"raw umber", 2.4}, {"green earth", 2.4}, {"yellow ochre", 0.7}, {"bone black", 0.45}, medium=0.08}
work(band, {hand="body", pile=crestP, angle=-1.3, coverage=2.5, fill=true, clip=true, length={6,16}, tool="filbert 5"})
-- the halo at the oak foot
local halo = ellipse(305, 590, 64, 22) * bankM2 - flare2 - oakM2
work(halo, {hand="body", pile=crestP, angle=-1.3, coverage=2.5, fill=true, clip=true, length={6,14}, tool="filbert 5"})

--@ chunk 59
grassDark = pile{{"raw umber", 2}, {"green earth", 2}, {"bone black", 0.6}, {"Prussian blue", 0.15}, medium=0.1}
grassMid = pile{{"green earth", 2.5}, {"raw umber", 1.3}, {"yellow ochre", 0.9}, {"Prussian blue", 0.05}, medium=0.1}
grassDry = pile{{"yellow ochre", 2}, {"raw umber", 1.2}, {"green earth", 1}, {"lead white", 0.4}, medium=0.1}
grassCool = pile{{"green earth", 2}, {"raw umber", 1}, {"smalt", 0.5}, {"lead white", 0.3}, medium=0.1}
gpiles = {grassDark, grassMid, grassDry, grassCool}
gb = {}
for i = 1, 4 do gb[i] = brush{kind="round", width=3, point=1}; gb[i]:load(gpiles[i], 0.8) end
gcount = {0,0,0,0}
function blade(k, x, y, L, a, w)
  local b = gb[k]
  gcount[k] = gcount[k] + 1
  if gcount[k] % 6 == 0 then b:load(gpiles[k], 0.75) end
  local bend = randn(0, 0.25)
  local p1 = {x + 0.5*L*math.cos(a), y + 0.5*L*math.sin(a)}
  local p2 = {x + L*math.cos(a + bend), y + L*math.sin(a + bend)}
  b:stroke({{x, y}, p1, p2}, {pressure={b:pressure_for(w), 0.0}, ramps={0.02, 0.7}})
end
function grassField(n, y0, y1, pickFn, seed)
  for c = 1, n do
    local x = rand(-10, 1010)
    local ytop = crestY(x)
    local y = lerp(math.max(y0, ytop + 2), y1, rand(0,1)^0.8)
    if keepOut:at(x, y) < 0.5 then
      local s = lerp(0.35, 1.6, clamp((y - ytop) / (704 - ytop), 0, 1))
      local k = pickFn(x, y)
      local nb = math.random(4, 10)
      for j = 1, nb do
        local L = s * rand(8, 20)
        local a = -math.pi/2 + randn(0, 0.3)
        blade(k, x + randn(0, 2*s), y + rand(0, 2*s), L, a, s * rand(0.6, 1.1))
      end
    end
  end
end
grassField(700, 560, 704, function(x, y) local r = rand(0,1); if y > 650 then return r < 0.6 and 1 or (r < 0.85 and 4 or 2) end; return r < 0.35 and 1 or (r < 0.7 and 2 or (r < 0.85 and 3 or 4)) end)

--@ chunk 60
local z = (bankM2 * above(function(x) return crestY(x) + 4 end, true)) - keepOut; blend(z, {angle=-1.5, coverage=3, length={8,24}})

--@ chunk 61
local a = above(function(x) return crestY(x)+4 end); print(a:area(), (bankM2*a):area(), bankM2:area(), (bankM2 - above(function(x) return crestY(x)+4 end)):area())

--@ chunk 62
local z = (bankM2 - above(function(x) return crestY(x)+4 end)) - keepOut; blend(z, {angle=-1.5, coverage=3, length={8,24}})

--@ chunk 63
local z = (bankM2 - above(function(x) return crestY(x)+4 end)) - keepOut
fgUp = pile{{"raw umber", 2.2}, {"green earth", 2.4}, {"yellow ochre", 0.5}, {"bone black", 0.5}, medium=0.08}
fgLow = pile{{"raw umber", 2.2}, {"green earth", 1.8}, {"bone black", 0.9}, {"Prussian blue", 0.1}, medium=0.08}
local up = z * above(function(x) return crestY(x) + 45 end)
local low = z - above(function(x) return crestY(x) + 35 end)
work(up, {hand="body", pile=fgUp, angle=-1.45, coverage=3.2, fill=true, clip=true, length={8,20}, tool="filbert 8"})
work(low, {hand="body", pile=fgLow, angle=-1.45, coverage=3.2, fill=true, clip=true, length={10,26}, tool="filbert 10"})
blend(z, {angle=-1.5, coverage=2, length={20,40}, tool="badger 20"})

--@ chunk 64
print(drying(835,172), drying(760,260), drying(500,300))

--@ chunk 65
function skyRepaint(zone)
  local bot = 238
  local base = zone * (rect(0, 0, 1000, bot):roughen(4, 30, 555))
  local b1 = base * rect(0, 0, 1000, 95)
  local b2 = base * rect(0, 70, 1000, 110)
  local b3 = base * rect(0, 160, 1000, 100)
  work(b1, {hand="body", pile=skyTop, angle=0, coverage=2.6, fill=true, clip=true, length={20,50}, tool="filbert 10"})
  work(b2, {hand="body", pile=skyMid, angle=0, coverage=2.6, fill=true, clip=true, length={20,50}, tool="filbert 10"})
  work(b3, {hand="body", pile=skyLow, angle=0, coverage=2.4, fill=true, clip=true, length={20,50}, tool="filbert 10"})
  blend(base, {angle=0, coverage=3, ruler=true, length={80, 250}, tool="badger 20"})
  blend(base, {angle=1.5708, coverage=2, length={20, 60}, tool="badger 20"})
  blend(base, {angle=0, coverage=2, ruler=true, length={80, 250}, tool="badger 20"})
end
skyKeep = treeZone:grow(3) + moonM:grow(4)
skyRepaint(rect(700, 0, 300, 260) - skyKeep)

--@ chunk 66
for _,n in ipairs({"skyTop","skyMid","skyLow","skyDeep","skyTopS","skyMidS","skyMid2","skyMid3"}) do print(n, tostring(_G[n])) end

--@ chunk 67
function skyRepaint2(zone)
  local base = zone * (rect(0, 0, 1000, 240):roughen(4, 30, 556))
  local bands = {{skyTopS, -5, 70}, {skyMidS, 45, 90}, {skyMid2, 105, 80}, {skyMid3, 165, 90}}
  for _, b in ipairs(bands) do
    work(base * rect(0, b[2], 1000, b[3]):roughen(5, 40, 600 + b[2]), {hand="body", pile=b[1], angle=0, coverage=2.8, fill=true, clip=true, length={20,50}, tool="filbert 10"})
  end
  blend(base, {angle=0, coverage=3, ruler=true, length={80, 250}, tool="badger 20"})
  blend(base, {angle=1.5708, coverage=2, length={25, 70}, tool="badger 20"})
  blend(base, {angle=0, coverage=2, ruler=true, length={100, 300}, tool="badger 20"})
end
skyRepaint2(rect(0, 0, 1000, 260) - treeZone:grow(3))
work(moonM, {hand="detail", pile=moonP, coverage=4, fill=true, clip=true, length={2,5}})

--@ chunk 68
local tw = nil
for i, b in ipairs(branches) do
  local pts = b.pts or b
  local ok, r = pcall(ribbon, pts, 2.2)
  if ok then tw = tw and (tw + r) or r end
end
twigM = tw
local gap = (treeZone:grow(5) - oakM2:grow(1) - twigM - foliM:grow(3)) * rect(0, 0, 1000, 245)
print(gap:area())
local bands = {{skyTopS, -5, 70}, {skyMidS, 45, 90}, {skyMid2, 105, 80}, {skyMid3, 165, 90}}
for _, b in ipairs(bands) do
  work(gap * rect(0, b[2], 1000, b[3]), {hand="detail", pile=b[1], angle=0, coverage=3, fill=true, clip=true, length={4,10}})
end
blend((gap:grow(6) - oakM2:grow(1) - twigM - foliM:grow(3)) * rect(0,0,1000,245), {angle=0, coverage=3, length={10, 30}, tool="badger 12"})

--@ chunk 69
print(drying(300,420), drying(300,500), drying(200,300), tostring(barkDark), tostring(barkD))

--@ chunk 70
print(wait(36*60))
print(drying(300,420), drying(300,570))
trunkZ = oakM2 * rect(282, 360, 50, 250)
barkMid = pile{{"raw umber", 3}, {"bone black", 1.2}, {"green earth", 0.8}, {"lead white", 0.35}, {"red earth", 0.2}, medium=0.1}
work(trunkZ, {hand="body", pile=barkMid, angle=-1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 5"})
-- shadow side left, a slightly lighter warm column right-of-centre
local L = trunkZ * mask(function(x, y) local c = 305 + (y - 480) * 0.02; return x < c - 8 and 1 or 0 end)
work(L, {hand="body", pile=barkD, angle=-1.5708, coverage=2, fill=true, clip=true, length={10,30}, tool="filbert 4"})
blend(trunkZ, {angle=-1.5708, coverage=3, length={30, 80}, tool="badger 12"})

--@ chunk 71
print(type(oakM2.offset), type(oakM2.translate), type(oakM2.shift), type(oakM2.move))

--@ chunk 72
trunkZ = (oakM2 * rect(262, 330, 90, 275)) - foliM
work(trunkZ, {hand="body", pile=barkD, angle=-1.5708, coverage=3.2, fill=true, clip=true, length={10,30}, tool="filbert 5"})
work(trunkZ * rect(262, 330, 36, 275), {hand="body", pile=barkDark, angle=-1.5708, coverage=1.5, fill=true, clip=true, length={10,30}, tool="filbert 4"})
blend(trunkZ, {angle=-1.5708, coverage=2.5, length={30, 80}, tool="badger 12"})
barkRim = pile{{"raw umber", 2}, {"lead white", 0.9}, {"red earth", 0.35}, {"yellow ochre", 0.3}, {"bone black", 0.4}, medium=0.1}
local rim = (trunkZ - trunkZ:shrink(2.5)) * rect(306, 330, 60, 250)
work(rim, {hand="detail", pile=barkRim, angle=-1.5708, coverage=1.6, clip=true, length={6,18}, pressure={0.3,0.6}})
-- bark fissures: a few long vertical dark lines
local fb = brush{kind="round", width=1.6, point=1}
for i = 1, 14 do
  fb:load(barkDark, 0.8)
  local x = rand(290, 322); local y0 = rand(380, 520); local L = rand(20, 60)
  local pts = {}
  for k = 0, 4 do pts[#pts+1] = {x + randn(0, 1.2), y0 + L*k/4} end
  local m = trunkZ:shrink(2)
  if m:at(x, y0) > 0.5 and m:at(x, y0 + L) > 0.5 then
    fb:stroke(pts, {pressure={fb:pressure_for(1.2), fb:pressure_for(0.5)}, ramps={0.2, 0.3}})
  end
end

--@ chunk 73
local z = (oakM2 * rect(280, 335, 70, 60)) - foliM:shrink(1); work(z, {hand="detail", pile=barkD, angle=-1.2, coverage=3.5, fill=true, clip=true, length={4,12}}); blend(z, {angle=-1.4, coverage=2, length={10,25}, tool="badger 12"})

--@ chunk 74
local fr = ((trunkZ - trunkZ:shrink(2.5)) * oakM2:shrink(2.5)) * rect(262, 326, 100, 250); fr = fr:grow(1.5) * oakM2 - foliM:shrink(1); work(fr, {hand="detail", pile=barkD, angle=-1.4, coverage=4, fill=true, clip=true, length={3,8}}); blend(fr:grow(2) * oakM2 - foliM, {angle=-1.4, coverage=2, length={8,20}, tool="badger 12"})

--@ chunk 75
print(tostring(foliDark)); print(tostring(foliDark2)); print(tostring(foliLight)); print(type(foliTops), foliM:area(), treeZone:area()); print(drying(200,250), drying(430,250))

--@ chunk 76
function leafEdge(rx, ry, rw, rh, seed)
  local R = rect(rx, ry, rw, rh)
  local band = ((foliM:grow(4):roughen(1.5, 3, seed) - foliM:shrink(2.5)) * R) - oakM2:grow(0.5)
  work(band, {hand="detail", pile=foliDark2, coverage=3.5, fill=true, clip=true, length={2,5}})
  local ring = (foliM:grow(8) - foliM:grow(3)) * R
  local sp = nil
  local n = 0
  for i = 1, math.floor(rw*rh/10) do
    local x, y = rand(rx, rx+rw), rand(ry, ry+rh)
    if ring:at(x, y) > 0.5 and rand(0,1) < 0.35 then
      local e = ellipse(x, y, rand(1.2, 2.6), rand(1.0, 2.0))
      sp = sp and (sp + e) or e; n = n + 1
    end
  end
  if sp then work(sp, {hand="detail", pile=foliDark, coverage=3.5, fill=true, clip=true, length={1.5,3}}) end
  return n
end
print(leafEdge(40, 215, 120, 100, 901))

--@ chunk 77
foliMidP = pile{{"green earth", 2}, {"raw umber", 1.6}, {"yellow ochre", 0.9}, {"Prussian blue", 0.25}, {"bone black", 0.25}, {"lead white", 0.3}, medium=0.1}
foliLitP = pile{{"green earth", 2}, {"yellow ochre", 1.2}, {"raw umber", 0.8}, {"lead white", 0.5}, medium=0.1}
function leafFringe(rx, ry, rw, rh, seed)
  local R = rect(rx, ry, rw, rh)
  local ring = (foliM:grow(8) - foliM:grow(1)) * R - oakM2
  stipple(ring, {pile=foliDark, width=1.3, coverage=0.55, cluster={0.6, 5}, pressure={0.3, 0.7}, feather=0.5, seed=seed})
  local inner = (foliM:grow(2) - foliM:shrink(8)) * R - oakM2
  stipple(inner, {pile=foliMidP, width=1.8, coverage=0.9, cluster={0.5, 4}, pressure={0.3, 0.7}, seed=seed+1})
  -- lit upper edges: upper part only (where the mask above is empty)
  local lit = (foliM - foliM:offset(0)) -- placeholder
end
leafFringe(40, 215, 120, 100, 902)

--@ chunk 78
local R = rect(40, 215, 120, 100)
local clump = (foliM:grow(6)) * R - oakM2
local top = clump * mask(function(x, y) local cy = 245 + (x - 40) * 0.25; return y < cy + 18 and 1 or 0 end)
stipple(clump, {pile=foliMidP, width=2.2, coverage=0.8, cluster={0.5, 5}, pressure={0.3, 0.7}, feather=0.3, seed=911})
stipple(top, {pile=foliLight, width=2.0, coverage=0.7, cluster={0.5, 5}, pressure={0.3, 0.65}, feather=0.4, seed=912})
local pz = (ellipse(307, 362, 7, 7) + ellipse(328, 371, 7, 7)) * oakM2 - foliM:shrink(1)
work(pz, {hand="detail", pile=barkD, coverage=4, fill=true, clip=true, length={2,6}})

--@ chunk 79
local R = rect(40, 215, 120, 100)
local ring = (foliM:grow(7) - foliM:shrink(4)) * R - oakM2
stipple(ring, {pile=foliMidP, width=2.0, coverage=0.9, cluster={0.5, 4}, pressure={0.3, 0.7}, seed=921})
local topring = ring * mask(function(x, y) local cy = 245 + (x - 40) * 0.25; return y < cy + 12 and 1 or 0 end)
stipple(topring, {pile=foliLight, width=1.8, coverage=0.8, cluster={0.5, 4}, pressure={0.3, 0.65}, seed=922})
local pz = ellipse(307, 362, 7, 7) + ellipse(329, 372, 7, 8)
print(pz:area(), (pz*oakM2):area(), (pz*foliM):area())
work(pz * (oakM2 + foliM) , {hand="detail", pile=barkD, coverage=4, fill=true, clip=true, length={2,6}})

--@ chunk 80
local R = rect(40, 215, 115, 90)
local body = (foliM:grow(4):roughen(2, 4, 931)) * R - oakM2
work(body, {hand="detail", pile=foliDark, coverage=3.5, fill=true, clip=true, length={2,6}})
local fringe = (foliM:grow(9) - foliM:grow(3)) * R - oakM2
stipple(fringe, {pile=foliDark, width=1.6, coverage=0.7, cluster={0.6, 5}, pressure={0.35, 0.7}, feather=0, seed=932})
local topf = (foliM:grow(6)) * R * mask(function(x, y) local cy = 243 + (x - 40) * 0.22; return y < cy + 14 and 1 or 0 end)
stipple(topf, {pile=foliLight, width=1.8, coverage=0.9, cluster={0.5, 5}, pressure={0.3, 0.65}, feather=0, seed=933})
stipple(body * mask(function(x, y) local cy = 243 + (x - 40) * 0.22; return (y > cy + 10 and y < cy + 35) and 1 or 0 end), {pile=foliMidP, width=2, coverage=0.6, cluster={0.5, 5}, pressure={0.3, 0.6}, feather=0, seed=934})
local pz = (ellipse(307, 362, 8, 8) + ellipse(329, 372, 8, 9)) * foliM - oakM2:grow(0.5)
stipple(pz, {pile=foliMidP, width=2, coverage=1.6, cluster={0.5, 4}, pressure={0.4, 0.8}, feather=0, seed=935})
stipple(pz, {pile=foliLight, width=1.8, coverage=0.4, cluster={0.5, 4}, pressure={0.3, 0.6}, feather=0, seed=936})

--@ chunk 81
print(wait(3*24*60)); for _,p in ipairs({{100,260},{307,362},{571,540},{500,650},{300,480}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 82
local wx, mx = 571, 596
local shawl = poly({{wx-7.5, 533}, {wx-8, 545}, {wx, 553}, {wx+8, 545}, {wx+7.5, 533}, {wx+3, 530}, {wx-3, 530}}, true)
local arm = ribbon({{wx+6, 537}, {wx+11, 545}, {mx-8, 548}}, {3.4, 3, 2.8})
shawlP2 = pile{{"lead white", 2}, {"yellow ochre", 0.5}, {"raw umber", 0.6}, {"smalt", 0.12}, medium=0.05}
shawlSh = pile{{"lead white", 1.4}, {"raw umber", 0.8}, {"smalt", 0.25}, {"yellow ochre", 0.3}, medium=0.05}
local sh = shawl - ellipse(wx, 524.5, 4.1, 5.1)
work(sh + arm, {hand="detail", pile=shawlP2, coverage=4, fill=true, clip=true, angle=1.2, length={2,5}})
work((sh * rect(wx-9, 530, 5, 25)) + (sh * rect(wx-3, 546, 8, 8)), {hand="detail", pile=shawlSh, coverage=2, clip=true, angle=1.2, length={2,4}, pressure={0.3,0.5}})
-- fringe/tassel at the point
local fb = brush{kind="round", width=0.8, point=1}
fb:load(shawlSh, 0.7)
for k = -3, 3 do fb:stroke({{wx + k*0.9, 552}, {wx + k*1.1, 555.5}}, {pressure={fb:pressure_for(0.5), fb:pressure_for(0.3)}}) end
-- man's collar and hat band hints
local hb = brush{kind="round", width=1, point=1}
hb:load(pile{{"bone black", 1}, {"raw umber", 0.5}}, 0.8)
hb:stroke({{mx-4, 534}, {mx, 536}, {mx+4, 534}}, {pressure={hb:pressure_for(0.9), hb:pressure_for(0.9)}})

--@ chunk 83
local mx = 596
work(poly({{mx-11.5, 566}, {mx-11.8, 579}, {mx+11.8, 580}, {mx+11.5, 566}}, true) - rect(mx-7, 576, 14, 10), {hand="detail", pile=coatP, coverage=3.5, fill=true, clip=true, angle=-1.57, length={2,5}})
crestG1 = pile{{"raw umber", 2.2}, {"green earth", 2.2}, {"bone black", 0.5}, {"yellow ochre", 0.3}, medium=0.1}
crestG2 = pile{{"raw umber", 2}, {"green earth", 2}, {"yellow ochre", 0.9}, {"lead white", 0.15}, medium=0.1}
local cb = {brush{kind="round", width=1.5, point=1}, brush{kind="round", width=1.5, point=1}}
local cp = {crestG1, crestG2}
cb[1]:load(crestG1, 0.8); cb[2]:load(crestG2, 0.8)
local cnt = 0
local x = -5
while x < 1005 do
  x = x + rand(0.8, 3.2)
  local y = crestY(x) + rand(0.5, 3)
  if keepOut:at(x, y - 3) < 0.5 and keepOut:at(x, y + 2) < 0.5 and not (x > 830 and x < 862 and y > 570) then
    local nb = math.random(1, 3)
    for j = 1, nb do
      local k = (rand(0,1) < 0.75) and 1 or 2
      local L = rand(2.5, 8) * (rand(0,1) < 0.1 and 1.8 or 1)
      local a = -math.pi/2 + randn(0, 0.28)
      local b = cb[k]
      cnt = cnt + 1
      if cnt % 8 == 0 then b:load(cp[k], 0.75) end
      local bend = randn(0, 0.3)
      b:stroke({{x, y}, {x + 0.5*L*math.cos(a), y + 0.5*L*math.sin(a)}, {x + L*math.cos(a+bend), y + L*math.sin(a+bend)}}, {pressure={b:pressure_for(0.9), 0.0}, ramps={0.02, 0.7}})
    end
  end
end
print(cnt)

--@ chunk 84
local mx, fy = 596, 603
local hem = poly({{mx-11.6, 570}, {mx-11.8, 580.5}, {mx, 581.5}, {mx+11.8, 580.5}, {mx+11.6, 570}}, false)
work(hem, {hand="detail", pile=coatP, coverage=4.5, fill=true, clip=true, angle=0, length={2,5}})
local legs = poly({{mx-5.8, 581}, {mx-5.3, fy-1}, {mx-7, fy+1}, {mx-2, fy+1}, {mx-1.6, 581}}, false) + poly({{mx+1.2, 581}, {mx+2, fy+1}, {mx+6.5, fy+1}, {mx+5, fy-1}, {mx+5.4, 581}}, false)
work(legs, {hand="detail", pile=trouserP, coverage=3, fill=true, clip=true, angle=-1.57, length={2,5}})
local boots = rect(mx-7, fy-4, 5.5, 5) + rect(mx+1.5, fy-4, 5.5, 5)
work(boots * (legs:grow(1)), {hand="detail", pile=coatP, coverage=3, fill=true, clip=true, angle=0, length={1,3}})

--@ chunk 85
local zone = ((ellipse(300, 592, 72, 34):roughen(3, 12, 1201)) * bankM2) - (oakM2 * above(function(x) return 582 end)) - figM:grow(2)
zone = zone - above(function(x) return crestY(x) + 1 end)
work(zone * above(function(x) return crestY(x) + 22 end), {hand="body", pile=crestP, angle=-1.4, coverage=3, fill=true, clip=true, length={4,10}, tool="filbert 4"})
work(zone - above(function(x) return crestY(x) + 18 end), {hand="body", pile=fgUp, angle=-1.4, coverage=3, fill=true, clip=true, length={4,10}, tool="filbert 4"})
blend(zone, {angle=-1.5, coverage=2, length={8,20}, tool="badger 12"})
-- roots
rootM = poly({{276, 572}, {274, 586}, {266, 594}, {252, 600}, {262, 601}, {276, 597}, {284, 600}, {292, 598}, {300, 603}, {306, 599}, {316, 598}, {330, 604}, {344, 603}, {332, 596}, {322, 588}, {320, 572}}, true)
work(rootM, {hand="detail", pile=barkD, angle=-1.2, coverage=4, fill=true, clip=true, length={3,8}})
local rl = brush{kind="round", width=1.4, point=1}
rl:load(barkRim, 0.6)
rl:stroke({{318, 578}, {321, 588}, {330, 596}, {340, 601}}, {pressure={rl:pressure_for(0.8), rl:pressure_for(0.3)}})
rl:load(barkRim, 0.5)
rl:stroke({{303, 590}, {305, 597}}, {pressure={rl:pressure_for(0.6), rl:pressure_for(0.2)}})

--@ chunk 86
baseGround = pile{{"raw umber", 2.3}, {"green earth", 2.1}, {"bone black", 0.7}, {"yellow ochre", 0.3}, medium=0.08}
local zone = ((ellipse(300, 594, 80, 36):roughen(3, 12, 1202)) * bankM2) - above(function(x) return crestY(x) + 1 end) - figM:grow(2)
zone = zone - (oakM2 * above(function(x) return 560 end))
work(zone, {hand="body", pile=baseGround, angle=-1.4, coverage=3.2, fill=true, clip=true, length={4,10}, tool="filbert 4"})
blend(zone, {angle=-1.5, coverage=2, length={8,20}, tool="badger 12"})
rootM2 = poly({{277, 556}, {275, 572}, {272, 586}, {264, 595}, {249, 601}, {258, 603}, {270, 601}, {280, 603}, {290, 601}, {299, 605}, {308, 602}, {318, 603}, {330, 605}, {351, 603}, {336, 597}, {326, 588}, {322, 574}, {320, 556}}, true)
work(rootM2, {hand="detail", pile=barkD, angle=-1.5, coverage=4, fill=true, clip=true, length={4,12}})
work(rootM2 * rect(260, 540, 30, 70), {hand="detail", pile=barkDark, angle=-1.5, coverage=1.5, clip=true, length={4,12}, pressure={0.3,0.6}})
blend(rootM2 + (oakM2 * rect(260, 540, 80, 30)), {angle=-1.5708, coverage=2, length={15, 40}, tool="badger 12"})
local rl = brush{kind="round", width=1.3, point=1}
rl:load(barkRim, 0.55)
rl:stroke({{319, 566}, {322, 582}, {329, 593}, {343, 601}}, {pressure={rl:pressure_for(0.7), rl:pressure_for(0.2)}})
rl:load(barkDark, 0.7)
rl:stroke({{296, 575}, {298, 590}, {300, 603}}, {pressure={rl:pressure_for(0.8), rl:pressure_for(0.3)}})
rl:stroke({{284, 580}, {282, 592}, {279, 601}}, {pressure={rl:pressure_for(0.7), rl:pressure_for(0.3)}})

--@ chunk 87
for _,p in ipairs({{870,140},{700,250},{450,515},{700,500},{850,480},{150,453}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 88
-- evening star
starP = pile{{"lead white", 3}, {"chrome yellow", 0.1}, medium=0.05}
local sb = brush{kind="round", width=2.2, point=1}
sb:load(starP, 0.9)
sb:touch(876, 131, {pressure=sb:pressure_for(1.7)})
-- distant birds (rooks going home), small shallow Vs
birdP = pile{{"bone black", 1}, {"raw umber", 1}, {"smalt", 0.3}, {"lead white", 0.6}, medium=0.05}
local bb = brush{kind="round", width=1.0, point=1}
local birds = {{612, 262, 3.2}, {628, 255, 2.8}, {641, 268, 2.5}, {655, 259, 2.2}, {667, 266, 2.0}, {600, 274, 2.6}}
for _, b in ipairs(birds) do
  bb:load(birdP, 0.8)
  local x, y, s = b[1], b[2], b[3]
  local up = rand(-0.4, 0.4)
  bb:stroke({{x - s, y - 0.5*s + up}, {x - 0.45*s, y - 0.2*s}, {x, y + 0.15*s}}, {pressure={bb:pressure_for(0.3), bb:pressure_for(0.55)}})
  bb:stroke({{x, y + 0.15*s}, {x + 0.45*s, y - 0.25*s}, {x + s, y - 0.55*s - up}}, {pressure={bb:pressure_for(0.55), bb:pressure_for(0.3)}})
end
-- hay cocks on the meadow, right side between willows and figures, and a few far left
hayP = pile{{"yellow ochre", 2}, {"raw umber", 1}, {"lead white", 0.7}, {"green earth", 0.3}, medium=0.05}
hayLit = pile{{"lead white", 1.5}, {"yellow ochre", 1.5}, {"chrome yellow", 0.15}, {"raw umber", 0.3}, medium=0.05}
hayShP = pile{{"raw umber", 2}, {"green earth", 0.8}, {"bone black", 0.3}, {"lead white", 0.3}, medium=0.05}
local cocks = {{672, 522}, {700, 515}, {728, 510}, {648, 508}, {690, 501}, {470, 498}, {498, 494}, {440, 502}, {120, 505}, {150, 500}}
for _, c in ipairs(cocks) do
  local x, y = c[1], c[2]
  local s = (y - 440) / 134
  local w, h = 11 * s, 9 * s
  local m = poly({{x - w/2, y}, {x - w*0.42, y - h*0.45}, {x - w*0.2, y - h*0.9}, {x, y - h}, {x + w*0.22, y - h*0.9}, {x + w*0.42, y - h*0.45}, {x + w/2, y}}, true)
  work(m, {hand="detail", pile=hayP, coverage=4, fill=true, clip=true, angle=-1.3, length={1,3}})
  work(m * rect(x - w, y - h - 2, w*0.9, h + 3), {hand="detail", pile=hayShP, coverage=2, clip=true, angle=-1.3, length={1,3}, pressure={0.3,0.5}})
  work(m * rect(x - w*0.05, y - h - 2, w*0.4, h*0.35), {hand="detail", pile=hayLit, coverage=1.5, clip=true, angle=-1.3, length={1,2}, pressure={0.3,0.5}})
  -- cast shadow toward the viewer (sun behind)
  work(ellipse(x + 0.5, y + 1.0*s, w*0.6, 1.2*s), {hand="detail", pile=bankShadow, coverage=1.2, clip=true, angle=0, length={1,3}, pressure={0.2,0.4}})
end

--@ chunk 89
skyGlazeP = pile{{"smalt", 1}, {"cobalt blue", 0.6}, {"lead white", 0.3}, medium=0.9}
local z = rect(680, 0, 320, 225) - treeZone:grow(2) - moonM:grow(1.2)
work(z, {hand="glaze", pile=skyGlazeP, angle=0, coverage=1.2, load_at=function(x, y) return clamp(lerp(0.75, 0.0, (y - 20) / 200), 0, 1) end, ruler=true})

--@ chunk 90
local z = rect(640, 0, 360, 240) - treeZone:grow(2) - moonM:grow(1.2)
for i = 1, 3 do
  blend(z, {angle=0, coverage=3, ruler=true, length={150, 400}, tool="badger 40"})
  blend(z, {angle=1.5708, coverage=2, length={40, 120}, tool="badger 40"})
end
blend(z, {angle=0, coverage=3, ruler=true, length={200, 400}, tool="badger 40"})

--@ chunk 91
-- granite boulder, lower right
stoneP = pile{{"lead white", 1}, {"raw umber", 1.2}, {"bone black", 0.4}, {"smalt", 0.2}, {"yellow ochre", 0.2}, medium=0.05}
stoneD = pile{{"raw umber", 1.6}, {"bone black", 0.9}, {"green earth", 0.4}, {"lead white", 0.25}, medium=0.05}
stoneLit = pile{{"lead white", 2}, {"raw umber", 0.7}, {"yellow ochre", 0.4}, {"smalt", 0.2}, medium=0.05}
lichenP = pile{{"yellow ochre", 1.5}, {"lead white", 1}, {"green earth", 0.6}, {"raw umber", 0.3}, medium=0.05}
local o = outline{{770, 668, "c"}, {776, 646}, {795, 632}, {822, 629}, {846, 638}, {858, 655}, {862, 670, "c"}, char="broken", seed=41, closed=true}
boulderM = o:mask()
work(boulderM, {hand="body", pile=stoneP, angle=-0.4, coverage=3, fill=true, clip=true, length={4,10}, tool="filbert 4"})
local bsh = boulderM * mask(function(x, y) return (y > 652 + (x - 770) * 0.05) and 1 or 0 end)
work(bsh, {hand="body", pile=stoneD, angle=-0.2, coverage=2.5, fill=true, clip=true, length={4,10}, tool="filbert 4"})
blend(boulderM, {angle=-0.3, coverage=1.5, length={8,20}, tool="badger 12"})

--@ chunk 92
stoneMid = pile{{"raw umber", 1.5}, {"bone black", 0.6}, {"lead white", 0.6}, {"smalt", 0.2}, {"green earth", 0.3}, medium=0.05}
work(boulderM, {hand="body", pile=stoneMid, angle=-0.3, coverage=3, fill=true, clip=true, length={4,10}, tool="filbert 4"})
local low = boulderM * mask(function(x, y) return (y > 645 + math.abs(x - 815) * 0.12) and 1 or 0 end)
work(low, {hand="body", pile=stoneD, angle=-0.2, coverage=3, fill=true, clip=true, length={4,10}, tool="filbert 4"})
blend(boulderM, {angle=-1.2, coverage=1.5, length={8,20}, tool="badger 12"})
local top = (boulderM - boulderM:shrink(3.5)) * mask(function(x, y) return (y < 642) and 1 or 0 end)
work(top, {hand="detail", pile=stoneP, angle=0, coverage=2, clip=true, length={3,8}, pressure={0.3, 0.6}})
local top2 = (boulderM - boulderM:shrink(1.5)) * mask(function(x, y) return (y < 636) and 1 or 0 end)
work(top2, {hand="detail", pile=stoneLit, angle=0, coverage=1.2, clip=true, length={3,8}, pressure={0.25, 0.5}})

--@ chunk 93
print(drying(60, 600), drying(80, 680))
thStem = pile{{"green earth", 1.5}, {"raw umber", 1.6}, {"bone black", 0.5}, {"yellow ochre", 0.3}, medium=0.05}
thLeaf = pile{{"green earth", 2}, {"raw umber", 1.2}, {"bone black", 0.4}, {"Prussian blue", 0.1}, {"lead white", 0.2}, medium=0.05}
thHead = pile{{"lead white", 0.8}, {"red earth", 0.5}, {"smalt", 0.7}, {"raw umber", 0.3}, {"vermilion", 0.1}, medium=0.05}
thBract = pile{{"green earth", 1.5}, {"raw umber", 1}, {"lead white", 0.3}, medium=0.05}
local st = brush{kind="round", width=2.2, point=1}
local lf = brush{kind="round", width=2.5, point=1}
function thistle(x0, y0, H, lean, seed)
  math.randomseed(seed)
  local pts = {}
  for i = 0, 8 do
    local t = i / 8
    pts[#pts+1] = {x0 + lean*H*t*t + randn(0, 0.6), y0 - H*t}
  end
  st:load(thStem, 0.9)
  st:stroke(pts, {pressure={st:pressure_for(2.2), st:pressure_for(1.0)}})
  -- leaves: jagged, alternate, getting smaller upward
  for i = 1, 6 do
    local t = 0.08 + i * 0.1
    local px, py = x0 + lean*H*t*t, y0 - H*t
    local side = (i % 2 == 0) and 1 or -1
    local L = H * lerp(0.28, 0.1, t)
    local a = side > 0 and (-0.35 + randn(0,0.15)) or (math.pi + 0.35 + randn(0,0.15))
    local lp = {{px, py}}
    for k = 1, 5 do
      local f = k / 5
      local jag = (k % 2 == 0) and -1 or 1
      lp[#lp+1] = {px + L*f*math.cos(a) , py + L*f*math.sin(a) + L*0.18*f*f + jag*L*0.07}
    end
    lf:load(thLeaf, 0.8)
    lf:stroke(lp, {pressure={lf:pressure_for(L*0.22), lf:pressure_for(L*0.14), lf:pressure_for(L*0.16), lf:pressure_for(L*0.08), lf:pressure_for(0.3), 0}})
    -- spines
    local sp = brush{kind="round", width=0.6, point=1}; sp:load(thStem, 0.6)
    for k = 2, 5, 2 do local q = lp[k]; sp:stroke({{q[1], q[2]}, {q[1] + randn(0, 1.5), q[2] - rand(1, 3)}}, {pressure={sp:pressure_for(0.3), 0}}) end
  end
  -- heads: branch into 2-3
  local top = pts[#pts]
  local heads = {{top[1], top[2]}}
  for k = 1, 2 do
    local bx, by = x0 + lean*H*0.7*0.7, y0 - H*0.7
    local hx, hy = bx + (k == 1 and -1 or 1) * H * rand(0.08, 0.14), by - H * rand(0.12, 0.22)
    st:load(thStem, 0.7)
    st:stroke({{bx, by}, {(bx+hx)/2 + randn(0,1), (by+hy)/2}, {hx, hy}}, {pressure={st:pressure_for(1.1), st:pressure_for(0.8)}})
    heads[#heads+1] = {hx, hy}
  end
  for _, h in ipairs(heads) do
    local r = H * 0.035 + 1.5
    work(ellipse(h[1], h[2] + r*0.3, r, r*1.1), {hand="detail", pile=thBract, coverage=4, fill=true, clip=true, length={1,2}})
    local hb = brush{kind="round", width=0.8, point=1}
    for k = 1, 9 do
      hb:load(thHead, 0.8)
      local ang = -math.pi/2 + randn(0, 0.45)
      local L = r * rand(0.8, 1.4)
      hb:stroke({{h[1] + randn(0, r*0.3), h[2] - r*0.4}, {h[1] + L*math.cos(ang), h[2] - r*0.4 + L*math.sin(ang)}}, {pressure={hb:pressure_for(0.7), hb:pressure_for(0.4)}})
    end
  end
end
thistle(62, 706, 175, 0.12, 7001)
thistle(104, 704, 120, -0.1, 7002)
thistle(34, 704, 90, 0.05, 7003)

--@ chunk 94
for _,n in ipairs({"meadNear","meadNearL","meadNearD","meadL","meadD","meadLight","meadLight2","meadMid"}) do print(n, tostring(_G[n])) end

--@ chunk 95
local abv = above(function(x) return crestY(x) + 1 end)
local m1 = (ellipse(83, 531, 12, 13) + rect(74, 525, 20, 40)) * abv
work(m1, {hand="detail", pile=meadNearL, angle=0, coverage=3.5, fill=true, clip=true, length={3,8}})
work(m1 * rect(0, 537, 200, 30), {hand="detail", pile=meadNear, angle=0, coverage=1.5, clip=true, length={3,8}, pressure={0.3,0.6}})
blend(m1:grow(2) * abv, {angle=0, coverage=2, length={6,14}, tool="badger 12"})
local zone = (rect(0, 540, 170, 170):roughen(4, 20, 7101)) * bankM2
local upper = zone * above(function(x) return crestY(x) + 40 end)
local lower = zone - above(function(x) return crestY(x) + 32 end)
work(upper, {hand="body", pile=crestP, angle=-1.45, coverage=3.2, fill=true, clip=true, length={6,16}, tool="filbert 6"})
work(lower, {hand="body", pile=fgLow, angle=-1.45, coverage=3.2, fill=true, clip=true, length={8,20}, tool="filbert 8"})
work(lower * mask(function(x, y) return y < crestY(x) + 70 and 1 or 0 end), {hand="body", pile=fgUp, angle=-1.45, coverage=1.5, clip=true, length={8,20}, tool="filbert 8"})
blend(zone, {angle=-1.5, coverage=2.5, length={20,40}, tool="badger 20"})

--@ chunk 96
print(wait(2*24*60)); for _,p in ipairs({{100,260},{307,362},{300,600},{815,650},{80,600},{83,531},{900,100}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 97
local R = rect(38, 212, 125, 100)
local cl = foliM:grow(6) * R - oakM2
local topf = cl * mask(function(x, y) local cy = 240 + (x - 40) * 0.22; return y < cy + 16 and 1 or 0 end)
stipple(cl, {pile=foliMidP, width=2.0, coverage=0.7, cluster={0.5, 5}, pressure={0.35, 0.7}, feather=0, seed=951})
stipple(topf, {pile=foliLight, width=1.9, coverage=0.8, cluster={0.5, 5}, pressure={0.3, 0.65}, feather=0, seed=952})
local edge = foliM:grow(4) * rect(140, 225, 40, 75) - oakM2
stipple(edge, {pile=foliMidP, width=2.0, coverage=1.0, cluster={0.5, 5}, pressure={0.35, 0.7}, feather=0, seed=953})
stipple(edge, {pile=foliLight, width=1.8, coverage=0.4, cluster={0.5, 5}, pressure={0.3, 0.6}, feather=0, seed=954})

--@ chunk 98
print(wait(30*60)); for _,p in ipairs({{307,362},{329,372},{300,600},{250,610},{815,650},{80,600},{83,531}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 99
local pz = (ellipse(307, 362, 10, 10) + ellipse(329, 372, 10, 11)) - oakM2:shrink(0.5)
stipple(pz, {pile=foliDark, width=2.2, coverage=1.2, cluster={0.5, 4}, pressure={0.4, 0.8}, feather=0, seed=961})
stipple(pz, {pile=foliMidP, width=2.0, coverage=1.3, cluster={0.5, 4}, pressure={0.4, 0.8}, feather=0, seed=962})
stipple(pz * rect(290, 345, 60, 20), {pile=foliLight, width=1.8, coverage=0.6, cluster={0.5, 4}, pressure={0.3, 0.6}, feather=0, seed=963})
-- bark inside the circles: small rim-lit edge on the right limb
local ib = (ellipse(307, 362, 10, 10) + ellipse(329, 372, 10, 11)) * oakM2:shrink(0.5)
work(ib, {hand="detail", pile=barkD, angle=-1.3, coverage=1.5, clip=true, length={3,8}, pressure={0.3,0.6}})

--@ chunk 100
print(drying(700, 485), drying(500, 462))
glintP = pile{{"lead white", 3}, {"chrome yellow", 0.25}, {"yellow ochre", 0.1}, medium=0.05}
rippleP = pile{{"lead white", 1.4}, {"smalt", 0.4}, {"raw umber", 0.5}, {"green earth", 0.2}, medium=0.05}
local gb = brush{kind="round", width=1.0, point=1}
local rb = brush{kind="round", width=0.9, point=1}
local rz = riverAll:shrink(1.2) - willowZone - hullM:grow(2)
local placed = 0
for i = 1, 900 do
  local x, y = rand(100, 1000), rand(450, 505)
  if rz:at(x, y) > 0.5 and rz:at(x + 10, y) > 0.5 and rz:at(x - 10, y) > 0.5 then
    local s = clamp((y - 440) / 60, 0.2, 1)
    local L = rand(4, 16) * s
    if rand(0,1) < 0.55 then
      gb:load(glintP, 0.7)
      gb:stroke({{x - L/2, y}, {x + L/2, y + randn(0, 0.1)}}, {pressure={0.0, gb:pressure_for(0.35*s + 0.15), 0.0}})
    else
      rb:load(rippleP, 0.6)
      rb:stroke({{x - L/2, y}, {x + L/2, y + randn(0, 0.1)}}, {pressure={0.0, rb:pressure_for(0.3*s + 0.12), 0.0}})
    end
    placed = placed + 1
    if placed > 140 then break end
  end
end
print(placed)
-- sail reflection, broken
sailRefP = pile{{"lead white", 2}, {"yellow ochre", 0.6}, {"raw umber", 0.7}, {"red earth", 0.2}, medium=0.05}
local bx, by = 905, 486
for k = 0, 10 do
  local yy = by + 3 + k * 1.6
  local w = lerp(18, 8, k/10) * rand(0.7, 1.1)
  if riverA:at(bx + 6, yy) > 0.5 then
    gb:load(sailRefP, 0.6)
    gb:stroke({{bx - 2 - w*0.3, yy}, {bx - 2 + w*0.7, yy}}, {pressure={0.0, gb:pressure_for(0.55), 0.0}})
  end
end

--@ chunk 101
local m = ellipse(767, 482.5, 11, 4):roughen(0.8,3,77) * riverA:shrink(0.6) - willowZone; print(m:area()); work(m, {hand="detail", pile=riverWarm, angle=0, coverage=4, fill=true, clip=true, length={3,8}})

--@ chunk 102
print(wait(24*60)); for _,p in ipairs({{300,600},{815,650},{80,600},{83,531},{50,650}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 103
stoneGl = pile{{"raw umber", 1.5}, {"bone black", 0.8}, {"green earth", 0.3}, medium=0.6}
local face = boulderM * mask(function(x, y) return (y > 638 + math.abs(x - 815) * 0.08) and 1 or 0 end)
work(face, {hand="detail", pile=stoneGl, angle=-0.2, coverage=2.5, fill=true, clip=true, length={4,12}, pressure={0.4, 0.8}})
local deep = boulderM * mask(function(x, y) return (y > 655) and 1 or 0 end)
work(deep, {hand="detail", pile=stoneD, angle=0, coverage=2.5, fill=true, clip=true, length={4,12}})
-- cracks
local cb = brush{kind="round", width=0.9, point=1}
cb:load(stoneD, 0.8); cb:stroke({{800, 634}, {803, 645}, {799, 656}, {802, 667}}, {pressure={cb:pressure_for(0.6), cb:pressure_for(0.3)}})
cb:load(stoneD, 0.8); cb:stroke({{832, 638}, {838, 648}, {836, 660}}, {pressure={cb:pressure_for(0.5), cb:pressure_for(0.25)}})
-- lichen spots on the upper surface
stipple(boulderM * rect(780, 628, 70, 14), {pile=lichenP, width=1.3, coverage=0.25, cluster={0.7, 3}, pressure={0.2, 0.5}, seed=981})
-- grass across the base
local k = 0
for x = 764, 868, rand(1.2, 2.4) do
  local y = 668 + randn(0, 1.5) + ((x > 850) and -3 or 0)
  for j = 1, math.random(2, 4) do
    local kk = (rand(0,1) < 0.7) and 1 or 2
    blade(kk, x + randn(0, 1), y + rand(0, 3), rand(6, 16), -math.pi/2 + randn(0, 0.3), rand(0.7, 1.1))
  end
end

--@ chunk 104
blend(boulderM:shrink(1), {angle=-1.4, coverage=2.5, length={8,18}, tool="badger 12"})
-- a facet: a lighter plane on the left shoulder and one on the right, catching sky
local f1 = poly({{778, 648}, {784, 638}, {798, 634}, {796, 648}, {786, 656}}, false) * boulderM
local f2 = poly({{836, 636}, {852, 642}, {858, 656}, {846, 652}}, false) * boulderM
work(f1 + f2, {hand="detail", pile=stoneMid, angle=-0.8, coverage=1.5, clip=true, length={3,8}, pressure={0.3,0.6}})

--@ chunk 105
blend(boulderM:shrink(1.5), {angle=-0.9, coverage=2, length={6,14}, tool="badger 12"})

--@ chunk 106
fgBladeD = pile{{"raw umber", 2}, {"green earth", 1.6}, {"bone black", 0.9}, {"Prussian blue", 0.12}, medium=0.1}
fgBladeM = pile{{"raw umber", 2}, {"green earth", 2.2}, {"yellow ochre", 0.5}, {"bone black", 0.35}, medium=0.1}
fgBladeL = pile{{"green earth", 2}, {"yellow ochre", 1.1}, {"raw umber", 1.3}, {"lead white", 0.15}, medium=0.1}
gpiles = {fgBladeD, fgBladeM, fgBladeL, fgBladeD}
for i = 1, 4 do gb[i]:reload(gpiles[i], 0.8) end
local avoid = keepOut + boulderM:grow(1) + ellipse(300, 594, 82, 38)
local n = 0
for c = 1, 1600 do
  local x = rand(-5, 1005)
  local ytop = crestY(x)
  local y = lerp(ytop + 6, 712, rand(0,1)^0.85)
  if avoid:at(x, y) < 0.5 and avoid:at(x, y - 12) < 0.5 then
    local t = clamp((y - ytop) / (704 - ytop), 0, 1)
    local s = lerp(0.5, 1.5, t)
    local r = rand(0,1)
    local k = (r < 0.5) and 1 or ((r < 0.88) and 2 or 3)
    if t > 0.6 and k == 3 and rand(0,1) < 0.6 then k = 2 end
    for j = 1, math.random(2, 5) do
      blade(k, x + randn(0, 1.5*s), y + rand(0, 2*s), s * rand(6, 15), -math.pi/2 + randn(0, 0.25), s * rand(0.5, 0.9))
    end
    n = n + 1
  end
end
print(n)

--@ chunk 107
mistP = pile{{"lead white", 2}, {"pale smalt", 0.3}, {"chrome yellow", 0.05}, medium=0.85}
local z = rect(0, 440, 272, 42) - treeZone:grow(1) - oakM2:grow(1)
work(z, {hand="glaze", pile=mistP, angle=0, coverage=1.0, ruler=true, load_at=function(x, y) return clamp(1 - math.abs(y - 452) / 22, 0, 1) * 0.6 end})
for i = 1, 2 do
  blend(z, {angle=0, coverage=3, ruler=true, length={100, 250}, tool="badger 40"})
  blend(z, {angle=1.5708, coverage=1.5, length={20, 40}, tool="badger 40"})
end

--@ chunk 108
local tb = oakM2 * rect(255, 420, 90, 70)
work(tb, {hand="detail", pile=barkD, angle=-1.5708, coverage=4, fill=true, clip=true, length={6,16}})
work(tb * rect(255, 420, 40, 70), {hand="detail", pile=barkDark, angle=-1.5708, coverage=1.5, clip=true, length={6,16}, pressure={0.3,0.6}})
blend(oakM2 * rect(255, 410, 90, 90), {angle=-1.5708, coverage=2, length={15,40}, tool="badger 12", clip=true})
local rim = (tb - oakM2:shrink(2.5)) * rect(306, 420, 40, 70)
work(rim, {hand="detail", pile=barkRim, angle=-1.5708, coverage=1.4, clip=true, length={6,16}, pressure={0.3,0.55}})
-- the hanging clump
local hc = foliM * rect(125, 415, 70, 55)
work(hc, {hand="detail", pile=foliDark, coverage=3.5, fill=true, clip=true, length={2,5}})

--@ chunk 109
local ex = treeZone:grow(1) + oakM2:grow(1) + willowZone + hullM:grow(2) + sailM:grow(2) + figM:grow(2)
local z = rect(0, 440, 1000, 42) - ex
local z2 = rect(262, 440, 738, 42) - ex
work(z2, {hand="glaze", pile=mistP, angle=0, coverage=1.0, ruler=true, clip=true, load_at=function(x, y) return clamp(1 - math.abs(y - 452) / 22, 0, 1) * 0.6 end})
for i = 1, 2 do
  blend(z, {angle=0, coverage=3, ruler=true, length={100, 250}, tool="badger 40", clip=true})
  blend(z, {angle=1.5708, coverage=1.5, length={20, 40}, tool="badger 40", clip=true})
end

--@ chunk 110
print(wait(2*24*60)); for _,p in ipairs({{300,600},{83,531},{155,450},{150,445},{500,680},{300,450},{700,455}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 111
for _,n in ipairs({"hillP","meadFar","meadFarL","meadFarD","treeFar"}) do print(n, tostring(_G[n])) end; print(hillY and hillY(155), flatY and flatY(155))

--@ chunk 112
local halo = ellipse(156, 448, 34, 22):roughen(2, 8, 1301) - foliM:grow(0.3) - riverC:grow(0.6) - oakM2:grow(1)
hillMist = pile{{"lead white", 6}, {"smalt", 1.8}, {"vermilion", 0.2}, {"raw umber", 0.35}, medium=0.2}
farMist = pile{{"lead white", 2.6}, {"green earth", 2.6}, {"smalt", 0.9}, {"raw umber", 0.9}, medium=0.15}
nearMist = pile{{"green earth", 2}, {"yellow ochre", 1.2}, {"lead white", 1.2}, {"raw umber", 0.8}, {"smalt", 0.2}, medium=0.15}
local A = halo * mask(function(x, y) return y < flatY(x) - 0.5 and 1 or 0 end)
local B = halo * mask(function(x, y) return (y >= flatY(x) - 0.5 and y < 453) and 1 or 0 end)
local C = halo * mask(function(x, y) return y >= 453 and 1 or 0 end)
work(A, {hand="detail", pile=hillMist, angle=0, coverage=3.5, fill=true, clip=true, length={4,10}})
work(B, {hand="detail", pile=farMist, angle=0, coverage=3.5, fill=true, clip=true, length={4,10}})
work(C, {hand="detail", pile=nearMist, angle=0, coverage=3.5, fill=true, clip=true, length={4,10}})
blend(A, {angle=0, coverage=2, length={8,20}, tool="badger 12", clip=true})
blend(B + C, {angle=0, coverage=2, length={8,20}, tool="badger 12", clip=true})

--@ chunk 113
local halo = ellipse(156, 448, 34, 22):roughen(2, 8, 1301) - foliM:grow(0.3) - riverC:grow(0.6) - oakM2:grow(1)
local C = halo * mask(function(x, y) return y >= 453 and 1 or 0 end)
nearMist2 = pile{{"green earth", 2}, {"yellow ochre", 0.9}, {"lead white", 2.2}, {"raw umber", 0.8}, {"smalt", 0.45}, medium=0.15}
work(C, {hand="detail", pile=nearMist2, angle=0, coverage=3.5, fill=true, clip=true, length={4,10}})
blend(C, {angle=0, coverage=2, length={8,20}, tool="badger 12", clip=true})

--@ chunk 114
local halo = ellipse(156, 448, 34, 22):roughen(2, 8, 1301) - foliM:grow(0.3) - riverC:grow(0.6) - oakM2:grow(1)
local C = halo * mask(function(x, y) return y >= 453 and 1 or 0 end)
nearMist3 = pile{{"green earth", 2}, {"yellow ochre", 1.1}, {"lead white", 1.6}, {"raw umber", 0.85}, {"smalt", 0.3}, medium=0.15}
work(C, {hand="detail", pile=nearMist3, angle=0, coverage=3.5, fill=true, clip=true, length={4,10}})
blend(C, {angle=0, coverage=2, length={8,20}, tool="badger 12", clip=true})

--@ chunk 115
print(wait(36*60)); for _,p in ipairs({{300,600},{83,531},{150,445},{160,418},{500,680},{300,450},{160,458}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 116
print(drying(60,650), drying(100,690), drying(62,600))
for i = 1, 4 do gb[i]:reload(gpiles[i], 0.8) end
local n = 0
-- blades rooted along the root bottoms
for x = 240, 362, 0.9 do
  local yb = 600 + 4 * math.exp(-((x - 300)/40)^2) + randn(0, 2)
  if rand(0,1) < 0.8 then
    local k = (rand(0,1) < 0.55) and 1 or ((rand(0,1) < 0.8) and 2 or 3)
    blade(k, x, yb + rand(2, 8), rand(7, 18), -math.pi/2 + randn(0, 0.28), rand(0.6, 1.0))
    n = n + 1
  end
end
-- scattered over the ellipse of ground
for c = 1, 260 do
  local x, y = rand(220, 385), rand(575, 635)
  local e = ((x - 300)/85)^2 + ((y - 596)/40)^2
  if e < 1 and oakM2:at(x, y - 3) < 0.5 and (rootM2:at(x, y - 2) < 0.5) and y > crestY(x) + 3 then
    local k = (rand(0,1) < 0.5) and 1 or ((rand(0,1) < 0.8) and 2 or 3)
    for j = 1, math.random(2, 4) do
      blade(k, x + randn(0, 1.2), y + rand(0, 2), rand(5, 13), -math.pi/2 + randn(0, 0.3), rand(0.5, 0.9))
    end
    n = n + 1
  end
end
print(n)
-- a couple of dock leaves at the root foot
dockP = pile{{"green earth", 2}, {"raw umber", 1.4}, {"bone black", 0.35}, {"yellow ochre", 0.4}, medium=0.05}
dockL = pile{{"green earth", 2}, {"yellow ochre", 1}, {"raw umber", 0.8}, {"lead white", 0.3}, medium=0.05}
local lb = brush{kind="round", width=3, point=1}
local docks = {{262, 610, -2.3, 18}, {268, 612, -1.9, 16}, {340, 611, -0.9, 17}, {347, 612, -1.3, 14}}
for _, d in ipairs(docks) do
  local x, y, a, L = d[1], d[2], d[3], d[4]
  lb:load(dockP, 0.9)
  lb:stroke({{x, y}, {x + 0.3*L*math.cos(a), y + 0.3*L*math.sin(a)}, {x + 0.7*L*math.cos(a + 0.1), y + 0.7*L*math.sin(a + 0.1)}, {x + L*math.cos(a + 0.2), y + L*math.sin(a + 0.2)}}, {pressure={lb:pressure_for(1), lb:pressure_for(4.5), lb:pressure_for(3.2), 0}})
  local vb = brush{kind="round", width=0.7, point=1}; vb:load(dockL, 0.6)
  vb:stroke({{x, y}, {x + 0.5*L*math.cos(a + 0.05), y + 0.5*L*math.sin(a + 0.05)}, {x + 0.9*L*math.cos(a + 0.18), y + 0.9*L*math.sin(a + 0.18)}}, {pressure={vb:pressure_for(0.5), vb:pressure_for(0.2)}})
end

--@ chunk 117
local fl = poly({{286, 528}, {283, 545}, {278, 558}, {272, 572}, {273, 590}, {326, 590}, {324, 572}, {321, 558}, {319, 545}, {318, 528}}, true)
work(fl, {hand="detail", pile=barkD, angle=-1.5708, coverage=4, fill=true, clip=true, length={6,16}})
work(fl * rect(260, 520, 22, 80), {hand="detail", pile=barkDark, angle=-1.5708, coverage=1.5, clip=true, length={6,16}, pressure={0.3,0.6}})
blend(fl, {angle=-1.5708, coverage=2, length={15,35}, tool="badger 12", clip=true})
local rim = (fl - fl:shrink(2)) * rect(316, 520, 20, 60)
work(rim, {hand="detail", pile=barkRim, angle=-1.5708, coverage=1.3, clip=true, length={6,16}, pressure={0.3,0.5}})

--@ chunk 118
barkLow = pile{{"raw umber", 2.6}, {"bone black", 1.25}, {"green earth", 0.9}, {"lead white", 0.3}, {"red earth", 0.12}, {"Prussian blue", 0.04}, medium=0.1}
local fl = poly({{286, 528}, {283, 545}, {278, 558}, {272, 572}, {273, 590}, {326, 590}, {324, 572}, {321, 558}, {319, 545}, {318, 528}}, true)
local col = fl + (oakM2 * rect(262, 478, 90, 60)) + (rootM2 * rect(240, 540, 120, 50))
work(col, {hand="detail", pile=barkLow, angle=-1.5708, coverage=4, fill=true, clip=true, length={8,20}})
work(col * mask(function(x, y) return x < 292 - (y - 480) * 0.1 and 1 or 0 end), {hand="detail", pile=barkD, angle=-1.5708, coverage=1.5, clip=true, length={8,20}, pressure={0.3,0.6}})
blend(col, {angle=-1.5708, coverage=3, length={30,70}, tool="badger 12", clip=true})
local rim = (col - col:shrink(2)) * mask(function(x, y) return x > 312 and 1 or 0 end) - rect(0, 586, 1000, 30)
work(rim, {hand="detail", pile=barkRim, angle=-1.5708, coverage=1.3, clip=true, length={6,16}, pressure={0.3,0.5}})
local fb = brush{kind="round", width=1.4, point=1}
for _, xs in ipairs({{292, 486, 540}, {303, 500, 575}, {312, 482, 530}, {298, 545, 588}}) do
  fb:load(barkDark, 0.7)
  local pts = {}
  for k = 0, 5 do pts[#pts+1] = {xs[1] + randn(0, 1) + (k/5) * ((xs[1] - 300) * 0.15), lerp(xs[2], xs[3], k/5)} end
  fb:stroke(pts, {pressure={fb:pressure_for(1.0), fb:pressure_for(0.4)}, ramps={0.2, 0.3}})
end

--@ chunk 119
thStemD = pile{{"raw umber", 1.6}, {"green earth", 1.2}, {"bone black", 0.6}, medium=0.05}
thLeafD = pile{{"green earth", 1.6}, {"raw umber", 1.6}, {"bone black", 0.55}, {"Prussian blue", 0.1}, medium=0.05}
thLeafEdge = pile{{"green earth", 1.6}, {"raw umber", 1}, {"yellow ochre", 0.5}, {"lead white", 0.35}, medium=0.05}
thBractD = pile{{"raw umber", 1.5}, {"green earth", 1}, {"bone black", 0.35}, {"lead white", 0.1}, medium=0.05}
thFlower = pile{{"lead white", 1}, {"red earth", 0.35}, {"smalt", 0.8}, {"vermilion", 0.15}, {"raw umber", 0.1}, medium=0.05}
local stb = brush{kind="round", width=2, point=1}
local tb = brush{kind="round", width=0.8, point=1}
function jaggedLeaf(x, y, a, L, w, droop)
  local up, lo = {}, {}
  local n = 8
  for i = 0, n do
    local t = i / n
    local cx = x + L*t*math.cos(a)
    local cy = y + L*t*math.sin(a) + droop * L * t * t
    local nx, ny = -math.sin(a), math.cos(a)
    local lobe = (i % 2 == 1) and 1.0 or 0.45
    local ww = w * math.sin(math.pi * math.min(t * 1.15, 1)) * lobe
    up[#up+1] = {cx + nx*ww, cy + ny*ww}
    lo[#lo+1] = {cx - nx*ww*0.8, cy - ny*ww*0.8}
  end
  local pts = {}
  for i = 1, #up do pts[#pts+1] = up[i] end
  for i = #lo, 1, -1 do pts[#pts+1] = lo[i] end
  return poly(pts, false)
end
function thistle2(base, head, sideHeads, leafs, seed)
  math.randomseed(seed)
  local pts = {}
  for i = 0, 8 do local t = i/8; pts[#pts+1] = {lerp(base[1], head[1], t) + math.sin(t*3) * 1.5, lerp(base[2], head[2], t)} end
  stb:load(thStemD, 0.9)
  stb:stroke(pts, {pressure={stb:pressure_for(2.0), stb:pressure_for(1.1)}})
  local lm = nil
  for _, l in ipairs(leafs) do
    local t = l[1]; local px, py = lerp(base[1], head[1], t) + math.sin(t*3) * 1.5, lerp(base[2], head[2], t)
    local m = jaggedLeaf(px, py, l[2], l[3], l[4], l[5] or 0.15)
    lm = lm and (lm + m) or m
  end
  if lm then
    work(lm, {hand="detail", pile=thLeafD, coverage=4, fill=true, clip=true, length={2,5}})
    -- spines at lobe tips: tiny pale strokes
  end
  local heads = {head}
  for _, s in ipairs(sideHeads) do
    local t = s[1]; local px, py = lerp(base[1], head[1], t), lerp(base[2], head[2], t)
    stb:load(thStemD, 0.7)
    stb:stroke({{px, py}, {(px + s[2])/2 + randn(0, 0.6), (py + s[3])/2}, {s[2], s[3]}}, {pressure={stb:pressure_for(1.0), stb:pressure_for(0.8)}})
    heads[#heads+1] = {s[2], s[3], s[4]}
  end
  for _, h in ipairs(heads) do
    local r = h[3] or 3.2
    work(ellipse(h[1], h[2] + r*0.35, r*0.9, r), {hand="detail", pile=thBractD, coverage=4, fill=true, clip=true, length={1,2}})
    for k = 1, 11 do
      tb:load(thFlower, 0.85)
      local ang = -math.pi/2 + (k - 6) * 0.13 + randn(0, 0.08)
      local L = r * rand(0.9, 1.3)
      local sx, sy = h[1] + (k - 6) * r * 0.1, h[2] - r * 0.35
      tb:stroke({{sx, sy}, {sx + L*math.cos(ang), sy + L*math.sin(ang)}}, {pressure={tb:pressure_for(0.8), tb:pressure_for(0.45)}})
    end
  end
end
-- A: tall one, head over the ghost
thistle2({72, 708}, {82, 527}, {{0.72, 70, 538, 2.8}, {0.78, 95, 537, 2.6}},
  {{0.12, -2.6, 22, 5, 0.25}, {0.2, -0.5, 24, 5, 0.25}, {0.33, -2.7, 20, 4.5, 0.2}, {0.45, -0.35, 19, 4, 0.2}, {0.56, -2.8, 16, 3.6, 0.2}, {0.66, -0.3, 15, 3.4, 0.15}, {0.76, -2.9, 12, 3, 0.1}, {0.85, -0.25, 10, 2.5, 0.1}}, 8101)
-- B: shorter, head at the pink tuft
thistle2({46, 708}, {52, 546}, {{0.8, 42, 556, 2.5}},
  {{0.15, -2.8, 18, 4.5, 0.25}, {0.3, -0.4, 17, 4, 0.2}, {0.47, -2.6, 14, 3.5, 0.2}, {0.62, -0.5, 12, 3, 0.15}, {0.78, -2.8, 9, 2.4, 0.1}}, 8102)
-- C: small one right
thistle2({128, 708}, {134, 612}, {},
  {{0.2, -0.4, 14, 3.5, 0.2}, {0.4, -2.7, 12, 3, 0.2}, {0.6, -0.3, 10, 2.6, 0.15}}, 8103)

--@ chunk 120
function leafSet(base, head, leafs)
  local lm = nil
  for _, l in ipairs(leafs) do
    local t = l[1]; local px, py = lerp(base[1], head[1], t) + math.sin(t*3) * 1.5, lerp(base[2], head[2], t)
    local m = jaggedLeaf(px, py, l[2], l[3], l[4], l[5] or 0.15)
    lm = lm and (lm + m) or m
  end
  return lm
end
local LA = leafSet({72, 708}, {82, 527}, {{0.12, -2.6, 22, 5, 0.25}, {0.2, -0.5, 24, 5, 0.25}, {0.33, -2.7, 20, 4.5, 0.2}, {0.45, -0.35, 19, 4, 0.2}, {0.56, -2.8, 16, 3.6, 0.2}, {0.66, -0.3, 15, 3.4, 0.15}, {0.76, -2.9, 12, 3, 0.1}, {0.85, -0.25, 10, 2.5, 0.1}})
local LB = leafSet({46, 708}, {52, 546}, {{0.15, -2.8, 18, 4.5, 0.25}, {0.3, -0.4, 17, 4, 0.2}, {0.47, -2.6, 14, 3.5, 0.2}, {0.62, -0.5, 12, 3, 0.15}, {0.78, -2.8, 9, 2.4, 0.1}})
local LC = leafSet({128, 708}, {134, 612}, {{0.2, -0.4, 14, 3.5, 0.2}, {0.4, -2.7, 12, 3, 0.2}, {0.6, -0.3, 10, 2.6, 0.15}})
local L = LA + LB + LC
local rim = (L - L:shrink(0.9)) * mask(function(x, y) return 1 end)
work(rim, {hand="detail", pile=thLeafEdge, coverage=1.6, clip=true, length={2,5}, pressure={0.3,0.6}})
-- mute the flowers slightly: a few darker purple strokes at their bases
thFlowerD = pile{{"lead white", 0.6}, {"red earth", 0.35}, {"smalt", 1}, {"raw umber", 0.35}, medium=0.05}
local tb = brush{kind="round", width=0.8, point=1}
for _, h in ipairs({{82, 527, 3.2}, {70, 538, 2.8}, {95, 537, 2.6}, {52, 546, 3.2}, {42, 556, 2.5}, {134, 612, 3.2}}) do
  for k = 1, 5 do
    tb:load(thFlowerD, 0.8)
    local ang = -math.pi/2 + (k - 3) * 0.25
    local sx, sy = h[1] + (k - 3) * h[3] * 0.15, h[2] - h[3] * 0.3
    tb:stroke({{sx, sy}, {sx + h[3]*0.6*math.cos(ang), sy + h[3]*0.6*math.sin(ang)}}, {pressure={tb:pressure_for(0.8), tb:pressure_for(0.5)}})
  end
end
-- tall grasses with drooping seed heads against the meadow
panP = pile{{"raw umber", 1.5}, {"yellow ochre", 1}, {"green earth", 0.6}, {"bone black", 0.3}, medium=0.05}
panL = pile{{"yellow ochre", 1.5}, {"lead white", 0.8}, {"raw umber", 0.6}, medium=0.05}
local gs = brush{kind="round", width=0.9, point=1}
local grs = {{62, 575, 66, 512, -1}, {90, 578, 97, 516, 1}, {100, 580, 106, 524, 1}, {76, 572, 71, 518, -1}, {110, 582, 113, 530, 1}, {58, 580, 57, 526, -1}, {24, 585, 28, 530, 1}}
for _, g in ipairs(grs) do
  local x0, y0, x1, y1, d = g[1], g[2], g[3], g[4], g[5]
  gs:load(thStemD, 0.8)
  gs:stroke({{x0, y0}, {lerp(x0, x1, 0.5) + d*1.5, lerp(y0, y1, 0.5)}, {x1, y1}, {x1 + d*3, y1 + 2}}, {pressure={gs:pressure_for(0.9), gs:pressure_for(0.6), gs:pressure_for(0.5), gs:pressure_for(0.3)}})
  for k = 1, 9 do
    local t = k / 9
    local px, py = x1 + d*3*t, y1 + 2*t
    local sp = (k % 2 == 0) and 1 or -1
    gs:load((k % 3 == 0) and panL or panP, 0.7)
    gs:stroke({{px, py}, {px + sp*1.6 + d*0.8, py + 3.2}}, {pressure={gs:pressure_for(0.7), gs:pressure_for(0.4)}})
  end
end

--@ chunk 121
print(wait(2*24*60)); for _,p in ipairs({{300,500},{300,570},{150,445},{160,418},{500,680},{160,458},{80,540}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 122
-- hanging clump and strip
local hc = foliM:grow(3) * rect(120, 410, 80, 60) - oakM2
stipple(hc, {pile=foliMidP, width=2.0, coverage=0.9, cluster={0.5, 4}, pressure={0.35, 0.7}, feather=0, seed=1401})
stipple(hc * rect(120, 410, 80, 26), {pile=foliLight, width=1.8, coverage=0.7, cluster={0.5, 4}, pressure={0.3, 0.6}, feather=0, seed=1402})
stipple(hc * rect(120, 436, 80, 40), {pile=foliLight, width=1.6, coverage=0.2, cluster={0.5, 4}, pressure={0.3, 0.55}, feather=0, seed=1403})
-- riverside shrubs along riverC's near bank
shrubP = pile{{"green earth", 2}, {"raw umber", 1.3}, {"lead white", 0.9}, {"smalt", 0.5}, medium=0.1}
shrubL = pile{{"green earth", 1.6}, {"lead white", 1.4}, {"yellow ochre", 0.6}, {"smalt", 0.3}, {"raw umber", 0.5}, medium=0.1}
local sm = nil
local x = 112
while x < 212 do
  local w = rand(4, 9); local h = rand(2.2, 4.2)
  local e = ellipse(x, 456.5 - h*0.4, w*0.6, h)
  sm = sm and (sm + e) or e
  x = x + w * rand(0.6, 1.1)
end
sm = sm:roughen(0.6, 2.5, 1404) - oakM2 - foliM
stipple(sm, {pile=shrubP, width=1.4, coverage=2.4, cluster={0.5, 3}, pressure={0.35, 0.7}, feather=0, seed=1405})
stipple(sm * rect(0, 440, 1000, 14), {pile=shrubL, width=1.2, coverage=0.5, cluster={0.5, 3}, pressure={0.3, 0.55}, feather=0, seed=1406})
-- dark unifying glaze over the lowest foreground
fgGlaze = pile{{"raw umber", 1.5}, {"bone black", 0.5}, {"green earth", 0.6}, {"Prussian blue", 0.08}, medium=0.75}
local fz = (bankM2 * rect(0, 628, 1000, 90)) - boulderM:grow(1) - figM
work(fz, {hand="glaze", pile=fgGlaze, angle=-0.05, coverage=1.0, clip=true, load_at=function(x, y) return clamp((y - 628) / 60, 0, 1) * 0.7 end})

--@ chunk 123
print(wait(24*60)); for _,p in ipairs({{300,500},{300,570},{290,540}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 124
print(wait(24*60)); for _,p in ipairs({{300,500},{290,540},{310,485}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 125
print(wait(36*60)); for _,p in ipairs({{300,500},{290,540},{300,520}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 126
trunkGl = pile{{"raw umber", 1.6}, {"bone black", 0.8}, {"green earth", 0.4}, {"Prussian blue", 0.05}, medium=0.55}
local fl = poly({{286, 528}, {283, 545}, {278, 558}, {272, 572}, {273, 590}, {326, 590}, {324, 572}, {321, 558}, {319, 545}, {318, 528}}, true)
local col = fl + (oakM2 * rect(255, 476, 100, 125)) + rootM2
work(col, {hand="glaze", pile=trunkGl, angle=-1.5708, coverage=1.3, clip=true, length={40, 110}, load_at=function(x, y) return clamp(0.55 + (305 - x) * 0.012, 0.3, 0.85) end})
work(col * mask(function(x, y) return x < 293 and 1 or 0 end), {hand="glaze", pile=trunkGl, angle=-1.5708, coverage=1.0, clip=true, length={40, 110}, load_at=0.6})

--@ chunk 127
local fl = poly({{286, 528}, {283, 545}, {278, 558}, {272, 572}, {273, 590}, {326, 590}, {324, 572}, {321, 558}, {319, 545}, {318, 528}}, true)
local col = fl + (oakM2 * rect(255, 476, 100, 125)) + rootM2
blend(col, {angle=-1.5708, coverage=3, length={30, 80}, tool="badger 20", clip=true})
blend(col, {angle=-1.4, coverage=2, length={20, 50}, tool="badger 20", clip=true})

--@ chunk 128
local fl = poly({{286, 528}, {283, 545}, {278, 558}, {272, 572}, {273, 590}, {326, 590}, {324, 572}, {321, 558}, {319, 545}, {318, 528}}, true)
local col = fl + (oakM2 * rect(255, 470, 100, 131)) + rootM2
work(col, {hand="detail", pile=barkD, angle=-1.5708, coverage=4, fill=true, clip=true, length={6,16}})
work(col * mask(function(x, y) return x < 294 - (y - 470) * 0.12 and 1 or 0 end), {hand="detail", pile=barkDark, angle=-1.5708, coverage=1.5, clip=true, length={6,16}, pressure={0.3,0.6}})
blend(col, {angle=-1.5708, coverage=2, length={15,40}, tool="badger 12", clip=true})
local rim = (col - col:shrink(2.5)) * mask(function(x, y) return x > 306 and 1 or 0 end) - rect(0, 588, 1000, 30)
work(rim, {hand="detail", pile=barkRim, angle=-1.5708, coverage=1.4, clip=true, length={6,16}, pressure={0.3,0.55}})

--@ chunk 129
local s = (oakM2:grow(5) - oakM2:grow(0.3)) * rect(312, 436, 20, 40) - foliM
local A = s * mask(function(x, y) return y < flatY(x) - 0.5 and 1 or 0 end)
local B = s * mask(function(x, y) return (y >= flatY(x) - 0.5 and y < 454) and 1 or 0 end)
local C = s * mask(function(x, y) return y >= 454 and 1 or 0 end)
work(A, {hand="detail", pile=hillMist, angle=0, coverage=3, fill=true, clip=true, length={2,5}})
work(B, {hand="detail", pile=farMist, angle=0, coverage=3, fill=true, clip=true, length={2,5}})
work(C, {hand="detail", pile=nearMist3, angle=0, coverage=3, fill=true, clip=true, length={2,5}})

--@ chunk 130
print(wait(36*60)); for _,p in ipairs({{300,595},{330,598},{270,598}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 131
print(wait(48*60)); for _,p in ipairs({{300,595},{330,598},{270,598}}) do print(p[1],p[2], drying(p[1],p[2])) end

--@ chunk 132
for i = 1, 4 do gb[i]:reload(gpiles[i], 0.8) end
for x = 250, 362, 0.8 do
  if rand(0,1) < 0.85 then
    local yb = 604 + randn(0, 2.5)
    local k = (rand(0,1) < 0.5) and 1 or ((rand(0,1) < 0.75) and 2 or 3)
    blade(k, x, yb, rand(8, 20), -math.pi/2 + randn(0, 0.28), rand(0.6, 1.0))
  end
end
for x = 262, 345, 2.2 do
  local k = (rand(0,1) < 0.6) and 2 or 3
  blade(k, x + randn(0,1), 598 + rand(0, 4), rand(5, 11), -math.pi/2 + randn(0, 0.3), rand(0.5, 0.8))
end

--@ chunk 133
print(drying(320,450), drying(815,650), drying(1000*0.6, 330))

--@ chunk 134
-- boulder: calm the glyph-like marks
local bz = boulderM:shrink(2) * rect(770, 633, 95, 22)
work(bz, {hand="detail", pile=stoneMid, angle=-0.2, coverage=2.5, fill=true, clip=true, length={3,8}, pressure={0.4,0.7}})
work(boulderM:shrink(2) * rect(770, 645, 95, 12), {hand="detail", pile=stoneD, angle=-0.1, coverage=1.5, clip=true, length={3,8}, pressure={0.3,0.6}})
-- trunk sliver
local sl = (oakM2 - oakM2:shrink(2.2)) * rect(308, 432, 30, 45)
work(sl, {hand="detail", pile=barkD, angle=-1.5708, coverage=3, fill=true, clip=true, length={3,8}})
work((oakM2 - oakM2:shrink(1.0)) * rect(308, 432, 30, 45), {hand="detail", pile=barkRim, angle=-1.5708, coverage=1.0, clip=true, length={3,8}, pressure={0.2,0.4}})
-- warm lit lower edges on the cloud strata
cloudUnder = pile{{"lead white", 3}, {"vermilion", 0.25}, {"chrome yellow", 0.3}, {"yellow ochre", 0.2}, medium=0.1}
local under = mask(function(x, y) return (cloudM:at(x, y) > 0.5 and cloudM:at(x, y + 2.2) < 0.5) and 1 or 0 end)
print(under:area())
work(under, {hand="detail", pile=cloudUnder, angle=0, coverage=1.6, clip=true, length={8,25}, pressure={0.25,0.5}})

--@ chunk 135
local under = mask(function(x, y) return (cloudM:at(x, y) > 0.5 and cloudM:at(x, y + 2.2) < 0.5) and 1 or 0 end)
local z = under:grow(4)
blend(z, {angle=1.5708, coverage=3, length={6, 12}, tool="badger 12"})
blend(z, {angle=0, coverage=2, length={20, 50}, tool="badger 12"})

--@ chunk 136
local under = mask(function(x, y) return (cloudM:at(x, y) > 0.5 and cloudM:at(x, y + 2.2) < 0.5) and 1 or 0 end); blend(under:grow(5), {angle=0, coverage=3, ruler=true, length={40, 120}, tool="badger 20"})

--@ chunk 137
local sl = (oakM2:grow(1.6) - oakM2:shrink(1)) * rect(314, 432, 20, 45) - foliM
work(sl, {hand="detail", pile=barkD, angle=-1.5708, coverage=3.5, fill=true, clip=true, length={3,8}})
-- boulder facets
local facet = poly({{826, 630}, {848, 636}, {860, 654}, {862, 670}, {835, 670}, {828, 652}}, false) * boulderM
work(facet, {hand="detail", pile=stoneD, angle=-1.2, coverage=3, fill=true, clip=true, length={3,8}})
local facet2 = poly({{772, 668}, {776, 650}, {790, 640}, {800, 648}, {796, 670}}, false) * boulderM
work(facet2, {hand="detail", pile=stoneMid, angle=-1.0, coverage=1.5, clip=true, length={3,8}, pressure={0.3,0.6}})
local cb = brush{kind="round", width=0.9, point=1}
cb:load(stoneD, 0.8); cb:stroke({{826, 631}, {828, 645}, {826, 657}, {829, 668}}, {pressure={cb:pressure_for(0.7), cb:pressure_for(0.3)}})
cb:load(stoneLit, 0.5); cb:stroke({{825, 632}, {812, 631}, {798, 634}}, {pressure={cb:pressure_for(0.5), cb:pressure_for(0.3)}})
for i = 1, 4 do gb[i]:reload(gpiles[i], 0.8) end
for x = 766, 868, 1.6 do
  if rand(0,1) < 0.7 then
    local k = (rand(0,1) < 0.6) and 1 or 2
    blade(k, x + randn(0, 1), 672 + rand(0, 4), rand(10, 22), -math.pi/2 + randn(0, 0.3), rand(0.7, 1.1))
  end
end
