-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=500, aspect=1.4, linen={18,15}, seed=7, ground={{pile={{"lead white",4},{"yellow ochre",1}}, um=100, apply="knife", texture=0.3},{pile={{"red earth",1},{"yellow ochre",2},{"lead white",3}}, um=20, apply="brush"}}}
print(W,H)

--@ chunk 2
h = pencil("2B")
h:rule({0, 440}, {1000, 440}, {pressure=0.25})
-- tree mass outline
h:sketch({{0,200},{40,170},{90,150},{140,175},{180,160},{230,190},{270,230},{300,290},{330,350},{345,410},{350,440}}, {pressure=0.3})
-- distant hills
h:sketch({{560,440},{620,418},{700,405},{790,412},{870,400},{950,410},{1000,415}}, {pressure=0.2})
-- far bank
h:sketch({{350,450},{500,448},{700,452},{1000,450}}, {pressure=0.2})
-- boat
h:sketch({{590,548},{605,556},{660,557},{680,547}}, {pressure=0.35})
h:sketch({{630,550},{632,522},{636,515}}, {pressure=0.35})
-- foreground bank
h:sketch({{0,600},{80,610},{160,640},{230,680},{260,714}}, {pressure=0.3})

--@ chunk 3
sky = above(function(x) return 452 end)
local function cut(y0, amp, s) return function(x) return y0 + amp*math.sin(x/130 + s) + 0.5*amp*math.sin(x/47 + 2*s) end end
local b1 = above(cut(150, 14, 1))
local b2 = above(cut(270, 12, 2))
local b3 = above(cut(370, 8, 3))
skyA = sky * b1
skyB = sky * b2 - b1
skyC = sky * b3 - b2
skyD = sky - b3
pA = pile{{"lead white",5},{"smalt",2},{"Prussian blue",0.08}}
pB = pile{{"lead white",7},{"pale smalt",1.2},{"yellow ochre",0.15}}
pC = pile{{"lead white",7},{"chrome yellow",0.35},{"vermilion",0.06},{"pale smalt",0.2}}
pD = pile{{"lead white",6},{"chrome yellow",0.8},{"vermilion",0.2}}
local opt = function(p) return {hand="body", tool="filbert 14", pile=p, angle=0.03, angle_jitter=0.12, coverage=3, fill=true, length={60,160}} end
work(skyA, opt(pA))
work(skyB, opt(pB))
work(skyC, opt(pC))
work(skyD, opt(pD))

--@ chunk 4
pA2 = pile{{"lead white",3},{"smalt",2.5},{"Prussian blue",0.2},{"red earth",0.08}}
pB2 = pile{{"lead white",5},{"smalt",1.3},{"vermilion",0.12},{"yellow ochre",0.1}}
pD2 = pile{{"lead white",4},{"chrome yellow",1.1},{"vermilion",0.4},{"red earth",0.08}}
local opt = function(p, cov) return {hand="body", tool="filbert 14", pile=p, angle=0.02, angle_jitter=0.1, coverage=cov, fill=true, length={70,180}} end
work(skyA:shrink(4), opt(pA2, 2.5))
work(skyB:shrink(4), opt(pB2, 2))
work(skyD:shrink(3), opt(pD2, 2.5))

--@ chunk 5
blend(sky, {angle=0, angle_jitter=0.08})
local seams = rect(0,120,1000,60) + rect(0,245,1000,50) + rect(0,350,1000,45)
blend(seams * sky, {angle=1.5, length={30,70}, clip=false})
blend(sky, {angle=0.02, pressure={0.25,0.4}})

--@ chunk 6
local c = {{520,212,270,20},{250,252,210,15},{770,182,220,24},{630,302,190,11},{140,172,150,13},{860,322,150,9},{420,150,160,10}}
clouds = nil
for i, e in ipairs(c) do
  local m = ellipse(e[1], e[2], e[3], e[4]):roughen(10, 60, i, 0)
  clouds = clouds and (clouds + m) or m
end
pCl = pile{{"lead white",3},{"smalt",1},{"red earth",0.35},{"raw umber",0.2},{"vermilion",0.1}}
work(clouds, {hand="body", tool="filbert 8", pile=pCl, angle=0.0, angle_jitter=0.06, coverage=2, length={40,120}, pressure={0.4,0.7}, edge="soft"})

--@ chunk 7
blend(clouds:grow(14), {angle=0, angle_jitter=0.05, tool="badger 24"})

--@ chunk 8
water = below(function(x) return 446 end)
local function cut(y0, amp, s) return function(x) return y0 + amp*math.sin(x/110 + s) + 0.4*amp*math.sin(x/37 + 2*s) end end
local c1 = above(cut(520, 5, 4))
local c2 = above(cut(610, 6, 5))
wA = water * c1
wB = water * c2 - c1
wC = water - c2
pwA = pile{{"lead white",4},{"chrome yellow",0.8},{"vermilion",0.3},{"raw umber",0.15}}
pwB = pile{{"lead white",4},{"smalt",0.9},{"vermilion",0.15},{"raw umber",0.25}}
pwC = pile{{"lead white",3},{"smalt",1.5},{"Prussian blue",0.12},{"raw umber",0.4}}
local opt = function(p) return {hand="body", tool="filbert 14", pile=p, angle=0, angle_jitter=0.03, coverage=3, fill=true, length={80,200}} end
work(wA, opt(pwA))
work(wB, opt(pwB))
work(wC, opt(pwC))
blend(water, {angle=0, angle_jitter=0.02})

--@ chunk 9
local seams = rect(0,495,1000,50) + rect(0,585,1000,50)
blend(seams * water, {angle=1.5, length={30,60}, clip=false})
blend(water, {angle=0, pressure={0.25,0.4}})
print(wait(20))

--@ chunk 10
print(wait(26*60)); print(drying(500,100), drying(500,400), drying(500,650), drying(500,215))

--@ chunk 11
print(wait(20*60)); print(drying(500,100), drying(500,400), drying(500,650), drying(500,215))

--@ chunk 12
hills = below({{380,452},{430,440},{500,432},{560,424},{620,414},{680,404},{730,402},{790,410},{840,404},{880,396},{930,400},{970,408},{1000,410}}) * above(function(x) return 452 end)
hills = hills:roughen(3, 40, 11, 0)
pHill = pile{{"lead white",3},{"smalt",1.2},{"red earth",0.35},{"raw umber",0.3},{"vermilion",0.08}}
work(hills, {hand="body", tool="filbert 8", pile=pHill, angle=0.05, angle_jitter=0.15, coverage=3, fill=true, length={30,80}, edge="firm"})

--@ chunk 13
local top = {{280,440},{330,441},{380,443},{430,442},{460,436},{470,428},{482,424},{495,427},{505,433},{520,430},{532,434},{545,441},{600,444},{680,445},{740,443},{770,446},{830,445},{862,438},{870,430},{884,426},{896,429},{905,436},{915,440},{960,443},{1000,442}}
local bot = function(x) return 458 + 2*math.sin(x/60) end
bank = below(top) * above(bot)
bank = bank:roughen(2.5, 25, 13, 0)
pBank = pile{{"raw umber",2},{"green earth",1},{"yellow ochre",0.5},{"lead white",0.8},{"smalt",0.4}}
work(bank, {hand="body", tool="filbert 5", pile=pBank, angle=0, angle_jitter=0.2, coverage=3, fill=true, length={15,50}, edge="firm"})

--@ chunk 14
treeO = outline{{-10,480,"c"},{-10,190,"c"},{20,172},{45,150},{75,128},{105,122},{130,138},{150,150},{175,142},{200,158},{222,178},{240,200},{262,215},{280,240},{290,268},{305,285},{318,310},{326,335},{340,360},{345,385},{356,405},{368,425},{385,440},{395,452},{400,466,"c"},{200,478},{60,484}, closed=true, char="soft", seed=21, lobe=18, amount=1.2}
trees = treeO:mask()
pDark = pile{{"Prussian blue",0.4},{"yellow ochre",1.2},{"raw umber",1.5},{"bone black",0.4}}
work(trees, {hand="body", tool="filbert 10", pile=pDark, angle=function(x,y) return 0.6*math.sin(x/40+y/55) end, angle_jitter=0.5, coverage=3.5, load=0.9, fill=true, length={20,50}, edge="soft"})

--@ chunk 15
local tall = outline{{-10,200,"c"},{-10,110},{15,85},{40,62},{62,48},{85,52},{100,70},{112,95},{120,125},{110,150},{60,170}, closed=true, char="soft", seed=31, lobe=14, amount=1.3}
local small = outline{{380,462,"c"},{384,440},{392,420},{404,405},{418,398},{432,404},{442,420},{446,440},{450,458,"c"}, closed=true, char="soft", seed=32, lobe=8, amount=1.2}
crowns = tall:mask() + small:mask()
work(crowns, {hand="body", tool="filbert 8", pile=pDark, angle=function(x,y) return 0.6*math.sin(x/40+y/55) end, angle_jitter=0.5, coverage=3.5, load=0.9, fill=true, length={15,40}, edge="soft"})
trees = trees + crowns

--@ chunk 16
local cr = {{70,100,50},{165,200,60},{250,265,55},{55,260,70},{305,335,42},{200,345,60},{345,400,32},{420,425,24},{110,390,60},{230,420,50}}
lit = nil
for i, c in ipairs(cr) do
  local m = ellipse(c[1]+0.35*c[3], c[2]-0.35*c[3], 0.55*c[3], 0.45*c[3]):roughen(0.18*c[3], 0.4*c[3], 40+i, 0)
  lit = lit and (lit + m) or m
end
lit = lit * trees:shrink(4)
pLeaf = pile{{"yellow ochre",2},{"green earth",1},{"raw umber",0.8},{"Prussian blue",0.1},{"lead white",0.25}}
stipple(lit, {pile=pLeaf, width=5, coverage=1.3, cluster={0.6, 8}, pressure={0.4,0.7}, dips={12,0.7,0.3}, feather=0.3})

--@ chunk 17
reflO = outline{{-10,470,"c"},{458,466,"c"},{452,480},{438,500},{420,512},{398,520},{388,540},{370,560},{352,580},{345,605},{330,630},{318,650},{300,668},{285,690},{275,720,"c"},{-10,720,"c"}, closed=true, char="soft", seed=51, lobe=14, amount=1.0}
refl = reflO:mask()
pRefl = pile{{"Prussian blue",0.35},{"yellow ochre",1},{"raw umber",1.5},{"bone black",0.3},{"lead white",0.5},{"smalt",0.4}}
work(refl, {hand="body", tool="filbert 10", pile=pRefl, angle=math.pi/2, angle_jitter=0.15, coverage=3.2, load=0.9, fill=true, length={25,70}, edge="soft"})
blend(refl, {angle=0, angle_jitter=0.03, tool="badger 30", clip=true})

--@ chunk 18
print(wait(30*60)); print(drying(100,300), drying(150,600), drying(420,420), drying(700,445))

--@ chunk 19
print(wait(24*60)); print(drying(100,300), drying(150,600), drying(420,420), drying(700,445))

--@ chunk 20
print(wait(40*60)); print(drying(100,300), drying(150,600), drying(420,420), drying(700,445))

--@ chunk 21
pRefl2 = pile{{"Prussian blue",0.35},{"yellow ochre",0.9},{"raw umber",1.5},{"bone black",0.35},{"smalt",0.3},{"lead white",0.08}}
local r = refl * below(function(x) return 482 end)
work(r, {hand="body", tool="filbert 12", pile=pRefl2, angle=math.pi/2, angle_jitter=0.12, coverage=3.5, load=0.9, fill=true, length={30,80}, edge="soft"})
blend(r, {angle=0, angle_jitter=0.03, tool="badger 30"})

--@ chunk 22
local skyside = above(function(x) return 455 end)
local outer = (trees:grow(14) - trees) * skyside
pDark2 = pile{{"Prussian blue",0.4},{"yellow ochre",1.2},{"raw umber",1.5},{"bone black",0.4}}
stipple(outer, {pile=pDark2, width=4, coverage=function(x,y) return 0.5 end, cluster={0.8, 10}, pressure={0.3,0.6}, dips={10,0.6,0.3}, feather=0.5, drag={3, 1.2}})

--@ chunk 23
edgepts = {}
for y = 45, 455, 9 do
  local ex = nil
  for x = 470, 0, -2 do if trees:at(x, y) > 0.5 then ex = x break end end
  if ex then edgepts[#edgepts+1] = {ex, y} end
end
for x = 0, 140, 9 do
  for y = 20, 200, 2 do if trees:at(x, y) > 0.5 then edgepts[#edgepts+1] = {x, y} break end end
end
print(#edgepts)
local b = brush("filbert", 6)
local n = 0
for i, p in ipairs(edgepts) do
  if math.random() < 0.75 then
    local k = math.random(2, 5)
    local out = rand(3, 16)
    local base = (p[2] < 200 and p[1] < 140) and -math.pi/2 or 0
    for j = 1, k do
      if n % 4 == 0 then b:load(pDark2, 0.9) end
      n = n + 1
      local a = base + randn(0, 0.8)
      local d = rand(-4, out)
      local cx = p[1] + d*math.cos(base) + rand(-6,6)*math.sin(base)
      local cy = p[2] + d*math.sin(base) + rand(-6,6)*math.cos(base)
      b:touch(cx, cy, {pressure=rand(0.5,0.9), drag={math.cos(a)*rand(2,6), math.sin(a)*rand(2,6)}, twist=rand(-0.5,0.5), angle=a})
    end
  end
end
print(n)

--@ chunk 24
local b = brush("filbert", 7)
local n = 0
for i, p in ipairs(edgepts) do
  if math.random() < 0.45 and p[2] < 440 then
    local top = (p[2] < 200 and p[1] < 140)
    local base = top and -math.pi/2 or (-0.5 + rand(-0.4, 0.4))
    local d = rand(10, 24)
    local cx = p[1] + d*math.cos(base)
    local cy = p[2] + d*math.sin(base)
    local k = math.random(4, 8)
    local r = rand(5, 10)
    for j = 1, k do
      if n % 3 == 0 then b:load(pDark2, 0.95) end
      n = n + 1
      local t = j / k
      -- touches from the clump back toward the edge so it stays attached
      local px = lerp(cx, p[1] - 4*math.cos(base), t*0.8) + rand(-r, r)
      local py = lerp(cy, p[2] - 4*math.sin(base), t*0.8) + rand(-r, r)
      local a = rand(0, math.pi)
      b:touch(px, py, {pressure=rand(0.7,1.0), drag={math.cos(a)*rand(2,5), math.sin(a)*rand(2,5)}, twist=rand(-0.6,0.6), angle=a})
    end
  end
end
print(n)

--@ chunk 25
local b = brush("filbert", 7); print(b:mark_width(0.9), b:mark_width(0.5)); local r = brush("round", 6); print(r:mark_width(0.9))

--@ chunk 26
local b = brush("filbert", 7); b:load(pDark2, 0.95); b:touch(298, 262, {pressure=0.9, drag={3,1}}); print(b:fullness()); b:stroke({{300,280},{306,276},{310,270}}, {pressure={0.9,0.6}}); print(b:fullness()); print(trees:at(285,262), trees:at(295,262))

--@ chunk 27
clumps = nil
local cnt = 0
for i, p in ipairs(edgepts) do
  if math.random() < 0.5 and p[2] < 445 then
    local top = (p[2] < 200 and p[1] < 140)
    local base = top and -math.pi/2 or (-0.6 + rand(-0.5, 0.5))
    local rr = rand(7, 16)
    local d = rr * rand(0.4, 1.0)
    local cx, cy = p[1] + d*math.cos(base), p[2] + d*math.sin(base)
    local m = ellipse(cx, cy, rr*rand(0.9,1.4), rr*rand(0.6,0.9)):roughen(rr*0.35, rr*0.6, 100+i, 0)
    clumps = clumps and (clumps + m) or m
    cnt = cnt + 1
  end
end
print(cnt)
clumps = clumps - trees
stipple(clumps, {pile=pDark2, width=5, coverage=2.8, pressure={0.6,0.9}, dips={16,0.8,0.2}, drag={2.5, 0.8}, twist=0.5, cluster={0.3,5}, clip=clumps:grow(2)})
trees = trees + clumps

--@ chunk 28
local dist = trees:distance()
holesHi, holesMid = nil, nil
local cnt = 0
for k = 1, 400 do
  local x, y = rand(0, 440), rand(40, 420)
  if trees:at(x, y) > 0.9 and cnt < 34 then
    -- depth inside: find edge distance by probing
    local near = false
    for _, d in ipairs({{25,0},{18,-18},{0,-25},{-18,-18},{25,-10}}) do
      if trees:at(x + d[1], y + d[2]) < 0.5 then near = true end
    end
    if near then
      cnt = cnt + 1
      local r = rand(2.5, 6)
      local m = ellipse(x, y, r*rand(1,1.6), r*rand(0.6,1)):roughen(r*0.4, r*0.8, 200+k, 0)
      if y < 170 then holesHi = holesHi and (holesHi + m) or m else holesMid = holesMid and (holesMid + m) or m end
    end
  end
end
print(cnt)
pHoleHi = pile{{"lead white",4},{"smalt",1.5},{"Prussian blue",0.05},{"raw umber",0.1}}
pHoleMid = pile{{"lead white",6},{"pale smalt",0.6},{"vermilion",0.08},{"chrome yellow",0.2},{"raw umber",0.1}}
local t = {kind="round", width=3.5, point=0.3}
if holesHi then work(holesHi, {hand="detail", tool=t, pile=pHoleHi, coverage=3, load=0.6, fill=true}) end
if holesMid then work(holesMid, {hand="detail", tool=t, pile=pHoleMid, coverage=3, load=0.6, fill=true}) end

--@ chunk 29
print(wait(8*60))
local h = (holesHi + holesMid)
print(drying(345/1.0*1, 0), drying(345,128))
stipple(h:grow(3), {pile=pDark2, width=3.5, coverage=1.6, pressure={0.5,0.9}, dips={14,0.8,0.2}, drag={2, 0.5}, twist=0.6, cluster={0.5,3}})

--@ chunk 30
local h = holesHi + holesMid; local found = 0; for y=40,420,1 do for x=0,440,1 do if found < 4 and h:at(x,y) > 0.95 then print(x,y,drying(x,y)); found = found + 1 end end end

--@ chunk 31
print(wait(20*60))
print(drying(121,64))
pDark3 = pile{{"Prussian blue",0.4},{"yellow ochre",1.1},{"raw umber",1.5},{"bone black",0.5}}
local h = (holesHi + holesMid):grow(2.5)
work(h, {hand="detail", tool={kind="round", width=4, point=0.2}, pile=pDark3, coverage=4, load=1, fill=true})

--@ chunk 32
local cr = {{60,85,45},{140,160,50},{230,215,55},{300,285,45},{335,345,35},{410,420,26},{60,230,60},{170,300,65},{250,370,55},{100,380,60},{360,405,30}}
litM, shM = nil, nil
for i, c in ipairs(cr) do
  local e = ellipse(c[1], c[2], c[3], c[3]*0.8):roughen(c[3]*0.18, c[3]*0.35, 300+i, 0)
  local l = e - ellipse(c[1]-0.3*c[3], c[2]+0.35*c[3], c[3]*1.05, c[3]*0.85)
  local s = e * ellipse(c[1]-0.1*c[3], c[2]+0.75*c[3], c[3]*1.0, c[3]*0.55)
  litM = litM and (litM + l) or l
  shM = shM and (shM + s) or s
end
litM = litM * trees:shrink(2)
shM = shM * trees:shrink(2)
pShade = pile{{"Prussian blue",0.5},{"raw umber",1.5},{"bone black",0.6},{"yellow ochre",0.5}}
pLit = pile{{"yellow ochre",2},{"green earth",1.2},{"raw umber",0.7},{"Prussian blue",0.12},{"lead white",0.35},{"red earth",0.15}}
stipple(shM, {pile=pShade, width=5, coverage=1.8, pressure={0.5,0.9}, dips={14,0.8,0.2}, drag={3, 0.6}, twist=0.6, cluster={0.5,6}, feather=0.3})
stipple(litM, {pile=pLit, width=4, coverage=1.1, pressure={0.4,0.8}, dips={12,0.7,0.2}, drag={2.5, -0.5}, twist=0.6, cluster={0.6,6}, feather=0.4})

--@ chunk 33
blend((litM + shM):grow(8) * trees:shrink(3), {tool="badger 16", angle=function(x,y) return 0.8*math.sin(x/30 + y/40) end, angle_jitter=0.6, length={10,25}})

--@ chunk 34
print(wait(30*60)); print(drying(80,230), drying(200,330), drying(400,300))

--@ chunk 35
local inner = trees:shrink(10)
nz = noise{seed=77, octaves=4, period=90, persistence=0.55}
warmPart = inner * mask(function(x,y) return nz(x,y) > 0.05 and 1 or 0 end)
coolPart = inner - warmPart
pTW = pile{{"Prussian blue",0.3},{"yellow ochre",1.3},{"raw umber",1.6},{"bone black",0.35},{"green earth",0.4}}
pTC = pile{{"Prussian blue",0.5},{"yellow ochre",0.8},{"raw umber",1.4},{"bone black",0.55}}
local ang = function(x,y) return 0.9*math.sin(x/35 + y/50) end
work(coolPart, {hand="body", tool="filbert 9", pile=pTC, angle=ang, angle_jitter=0.6, coverage=3.2, load=0.9, fill=true, length={12,35}, edge="soft"})
work(warmPart, {hand="body", tool="filbert 9", pile=pTW, angle=ang, angle_jitter=0.6, coverage=3.2, load=0.9, fill=true, length={12,35}, edge="soft"})

--@ chunk 36
local seam = (warmPart:grow(6) - warmPart:shrink(6)) * trees:shrink(10)
blend(seam, {tool="badger 12", angle=function(x,y) return 0.9*math.sin(x/35 + y/50) end, angle_jitter=0.7, length={8,20}, pressure={0.25,0.45}})

--@ chunk 37
local pts = {{455,452}}
local x = 455
local s = 0
while x < 1005 do
  s = s + 1
  local y = 446 + 3*math.sin(x/70) 
  if math.random() < 0.3 then y = y - rand(3, 9) end
  pts[#pts+1] = {x, y}
  x = x + rand(6, 16)
end
pts[#pts+1] = {1005, 452}
local topc = pts
farTrees = (below(topc) * above(function(x) return 459 + 1.5*math.sin(x/50) end)):roughen(1.5, 12, 61, 0)
pFar = pile{{"lead white",1},{"smalt",1},{"raw umber",1.5},{"green earth",0.6},{"red earth",0.2}}
work(farTrees, {hand="body", tool="filbert 4", pile=pFar, angle=0, angle_jitter=0.4, coverage=3, fill=true, length={8,25}, edge="firm", load=0.8})

--@ chunk 38
nearBank = (below(function(x) return 450 + 3*math.sin(x/40) end) * above(function(x) return 470 + 3*math.sin(x/55) - (x>400 and (x-400)*0.12 or 0) end) * rect(-10,0,478,714)):roughen(2, 15, 71, 0)
pNB = pile{{"raw umber",2},{"bone black",0.5},{"Prussian blue",0.2},{"yellow ochre",0.6},{"red earth",0.2}}
work(nearBank, {hand="body", tool="filbert 6", pile=pNB, angle=0, angle_jitter=0.3, coverage=3.5, load=0.9, fill=true, length={15,40}, edge="firm"})

--@ chunk 39
refl2O = outline{{-10,466,"c"},{466,466,"c"},{470,478},{462,490},{466,500},{450,511},{434,518},{424,532},{426,546},{408,558},{396,574},{392,592},{376,606},{372,624},{358,640},{352,660},{336,676},{330,698},{322,720,"c"},{-10,720,"c"}, closed=true, char="soft", seed=81, lobe=10, amount=1.0}
refl2 = refl2O:mask()
local nz2 = noise{seed=88, octaves=3, period=120, stretch={0, 4}}
local upper = refl2 * above(function(x) return 560 end)
local lower = refl2 - upper
pR1 = pile{{"Prussian blue",0.45},{"yellow ochre",1.0},{"raw umber",1.6},{"bone black",0.5},{"smalt",0.2}}
pR2 = pile{{"Prussian blue",0.4},{"yellow ochre",1.0},{"raw umber",1.4},{"bone black",0.35},{"smalt",0.4},{"lead white",0.12}}
work(upper, {hand="body", tool="filbert 12", pile=pR1, angle=0, angle_jitter=0.05, coverage=3.5, load=0.95, fill=true, length={40,110}, edge="soft"})
work(lower, {hand="body", tool="filbert 12", pile=pR2, angle=0, angle_jitter=0.05, coverage=3.2, load=0.9, fill=true, length={40,110}, edge="soft"})
blend(refl2, {angle=0, angle_jitter=0.02, tool="badger 30", pressure={0.25,0.45}})

--@ chunk 40
local function bar(pts, seed)
  return poly(pts, true):roughen(5, 30, seed, 0)
end
cA = bar({{300,172},{380,158},{470,150},{560,146},{650,152},{760,146},{860,156},{960,150},{1005,160},{1005,184},{900,186},{800,190},{700,188},{600,184},{500,190},{400,186},{320,182}}, 91)
cA2 = bar({{520,214},{600,204},{700,200},{800,206},{900,198},{1005,204},{1005,224},{900,226},{780,230},{660,228},{560,226}}, 92)
cB = bar({{450,300},{520,288},{600,282},{700,286},{790,280},{870,288},{960,294},{1005,300},{1005,318},{900,322},{800,326},{680,324},{560,322},{470,316}}, 93)
cC = bar({{640,362},{720,356},{820,354},{900,360},{960,366},{960,374},{860,376},{740,376},{650,372}}, 94)
cloudM = cA + cA2 + cB + cC
pCb = pile{{"lead white",2.5},{"smalt",1},{"red earth",0.45},{"raw umber",0.25},{"vermilion",0.05}}
work(cloudM, {hand="body", tool="filbert 8", pile=pCb, angle=0, angle_jitter=0.05, coverage=3, load=0.8, fill=true, length={40,120}, edge="soft"})

--@ chunk 41
blend(cloudM:grow(8), {angle=0, angle_jitter=0.04, tool="badger 20", clip=false, length={60,160}, pressure={0.3,0.5}})

--@ chunk 42
print(wait(24*60)); print(drying(600,170), drying(600,300), drying(200,460), drying(200,520))

--@ chunk 43
pCore = pile{{"lead white",1.6},{"smalt",1},{"red earth",0.5},{"raw umber",0.35}}
local cores = (cloudM:shrink(5)) * mask(function(x,y) return 1 end)
work(cores, {hand="body", tool="filbert 5", pile=pCore, angle=0, angle_jitter=0.03, coverage=1.3, load=0.6, length={60,200}, pressure={0.15,0.45}, edge="soft"})

--@ chunk 44
K1 = poly({{235,168},{280,150},{340,138},{420,130},{500,124},{580,126},{640,120},{720,126},{800,122},{880,130},{960,124},{1010,128},{1010,250},{960,246},{900,238},{820,244},{740,240},{660,236},{600,230},{540,224},{480,214},{430,202},{380,198},{320,192},{270,182}}, true):roughen(7, 35, 101, 0)
K2 = poly({{400,300},{450,284},{520,270},{600,262},{680,266},{760,258},{840,262},{920,256},{1010,262},{1010,340},{930,338},{850,342},{760,336},{680,338},{600,334},{520,326},{450,316}}, true):roughen(6, 30, 102, 0)
K3 = poly({{600,366},{660,352},{740,346},{820,348},{900,344},{970,352},{990,368},{940,386},{860,388},{760,390},{680,386},{620,380}}, true):roughen(5, 25, 103, 0)
cl = K1 + K2 + K3
pSkyHi = pile{{"lead white",4},{"smalt",1.6},{"Prussian blue",0.05},{"red earth",0.05}}
-- clean the dark smear from the tree blend
local smear = rect(240,145,110,40) - trees:grow(3)
work(smear, {hand="body", tool="filbert 6", pile=pile{{"lead white",6},{"pale smalt",1},{"vermilion",0.08},{"yellow ochre",0.15}}, angle=0, coverage=3, fill=true, length={20,50}, load=0.8, clip=smear})

--@ chunk 45
pV1 = pile{{"lead white",2.2},{"smalt",1},{"red earth",0.4},{"raw umber",0.3},{"vermilion",0.05}}
pV2 = pile{{"lead white",2.8},{"smalt",0.9},{"red earth",0.35},{"raw umber",0.2},{"vermilion",0.1}}
pV3 = pile{{"lead white",3.5},{"smalt",0.6},{"vermilion",0.25},{"red earth",0.2},{"chrome yellow",0.1}}
local cut = K1 - trees
local o = function(p) return {hand="body", tool="filbert 10", pile=p, angle=0, angle_jitter=0.04, coverage=3, load=0.85, fill=true, length={50,140}, edge="soft"} end
work(cut, o(pV1))
work(K2, o(pV2))
work(K3, o(pV3))

--@ chunk 46
local function under(m, d) return (m - m:offset(0) * m:shrink(0)) end
-- underside bands: part of each cloud within d units of its lower edge
local function lower(m, d)
  return m * mask(function(x,y) return (m:at(x, y + d) < 0.5) and 1 or 0 end)
end
uK1 = lower(K1, 16) - trees
uK2 = lower(K2, 14)
uK3 = lower(K3, 12)
pDarkC = pile{{"lead white",1},{"smalt",1},{"red earth",0.45},{"raw umber",0.4}}
pPeach = pile{{"lead white",4},{"chrome yellow",0.8},{"vermilion",0.45},{"red earth",0.05}}
local midK1 = (K1 - trees):shrink(12) * above(function(x) return 205 end)
local midK2 = K2:shrink(12)
work(midK1 + midK2, {hand="body", tool="filbert 8", pile=pDarkC, angle=0, angle_jitter=0.03, coverage=1.6, load=0.8, length={60,180}, pressure={0.4,0.7}})
work(uK1 + uK2 + uK3, {hand="body", tool="filbert 6", pile=pPeach, angle=0, angle_jitter=0.05, coverage=2.2, load=0.8, fill=true, length={30,90}, pressure={0.5,0.8}})

--@ chunk 47
local c = (K1 - trees:grow(4)) + K2 + K3
blend(c, {angle=0, angle_jitter=0.03, tool="badger 24", length={40,120}, clip=c:soften(4)})
blend(c, {angle=0, angle_jitter=0.02, tool="badger 30", length={60,160}, pressure={0.25,0.4}, clip=c:soften(4)})

--@ chunk 48
local pts = {}
for x = 460, 1010, 12 do pts[#pts+1] = {x, 462 + rand(0,2)} end
local bot = {}
for x = 460, 1010, 10 do bot[#bot+1] = {x, 472 + rand(-1,4) + 2*math.sin(x/30)} end
shoreRefl = (below(pts) * above(bot)):roughen(1.5, 10, 121, 0)
pSR = pile{{"lead white",0.8},{"smalt",1},{"raw umber",1.5},{"green earth",0.5},{"red earth",0.2}}
work(shoreRefl, {hand="body", tool="filbert 4", pile=pSR, angle=0, angle_jitter=0.05, coverage=2.5, load=0.7, fill=true, length={20,60}, edge="soft"})
-- reflected cloud colour: drawn as its own soft horizontal patches
rc = (poly({{480,560},{600,552},{760,556},{900,550},{1010,556},{1010,590},{880,596},{740,592},{600,596},{500,588}}, true):roughen(8, 40, 122, 0)
   + poly({{560,640},{700,632},{860,636},{1010,630},{1010,668},{860,672},{700,668},{580,662}}, true):roughen(8, 40, 123, 0))
pRC = pile{{"lead white",2.5},{"smalt",1},{"red earth",0.4},{"raw umber",0.3},{"vermilion",0.05}}
work(rc, {hand="body", tool="filbert 10", pile=pRC, angle=0, angle_jitter=0.02, coverage=1.8, load=0.6, pressure={0.3,0.6}, length={60,200}, edge="soft"})
blend(rc:grow(10) - refl2, {angle=0, angle_jitter=0.01, tool="badger 30", length={80,200}})

--@ chunk 49
local rim = (rc:grow(10) - rc:shrink(10)) - refl2:grow(4)
blend(rim, {angle=1.5, angle_jitter=0.2, tool="badger 14", length={15,35}, clip=false})
-- thin sky-colour ripple lines across the reflected cloud to break it
pRip = pile{{"lead white",5},{"pale smalt",1},{"yellow ochre",0.15},{"vermilion",0.05}}
local b = brush{kind="flat", width=3, stiffness=0.4}
for i = 1, 16 do
  local y = rand(545, 680)
  local x0 = rand(470, 900)
  local L = rand(60, 220)
  b:reload(pRip, 0.35)
  b:stroke({{x0, y}, {x0 + L*0.5, y + rand(-1,1)}, {x0 + L, y + rand(-1,1)}}, {pressure={0.3, 0.1}, ramps={0.2,0.5}, orient="across"})
end

--@ chunk 50
local area = rect(460, 530, 560, 160) - refl2:grow(4)
blend(area, {angle=0, angle_jitter=0.01, tool="badger 24", length={60,180}, pressure={0.3,0.45}})

--@ chunk 51
print(wait(20*60)); print(drying(600,170), drying(600,300), drying(700,570), drying(200,460), drying(700,468))

--@ chunk 52
local top = {{-10,440}}
local x = -10
while x < 470 do
  x = x + rand(10, 28)
  top[#top+1] = {x, 438 + rand(-8, 6) + (x > 380 and (x-380)*0.15 or 0)}
end
top[#top+1] = {478, 462}
local bot = {}
for xx = -10, 480, 15 do bot[#bot+1] = {xx, 469 + rand(-1.5, 1.5)} end
under = (below(top) * above(bot)):roughen(3, 14, 131, 0)
pUnder = pile{{"Prussian blue",0.45},{"raw umber",1.6},{"bone black",0.5},{"yellow ochre",0.7},{"green earth",0.3}}
work(under, {hand="body", tool="filbert 6", pile=pUnder, angle=function(x,y) return 0.5*math.sin(x/20) end, angle_jitter=0.5, coverage=3.5, load=0.95, fill=true, length={10,30}, edge="soft"})
-- the stub where the far-shore reflection runs under the near bank
local stub = rect(452, 460, 30, 20) * refl2:grow(12)
work(stub, {hand="detail", tool={kind="flat", width=4}, pile=pR1, coverage=3, load=0.9, fill=true})

--@ chunk 53
print(under:at(475,520), under:at(475,470), rect(452,460,30,20):at(475,520))

--@ chunk 54
for y=480,714,20 do print(y, under:at(475,y), under:at(465,y), under:at(485,y), under:at(700,y)) end

--@ chunk 55
print(wait(48*60)); print(drying(472,520), drying(472,650), drying(200,455))

--@ chunk 56
print(wait(72*60)); print(drying(472,520), drying(472,650), drying(200,455))

--@ chunk 57
print(wait(72*60)); print(drying(472,520), drying(472,650), drying(200,455))

--@ chunk 58
col = (under:grow(4) * below(function(x) return 476 end)) - refl2
print(col:area())
local z1 = col * above(function(x) return 525 end)
local z2 = col * below(function(x) return 525 end) * above(function(x) return 610 end)
local z3 = col * below(function(x) return 610 end)
fA = pile{{"lead white",5},{"chrome yellow",0.7},{"vermilion",0.15},{"raw umber",0.05}}
fB = pile{{"lead white",5},{"pale smalt",0.8},{"vermilion",0.15},{"yellow ochre",0.2}}
fC = pile{{"lead white",4},{"smalt",1.0},{"raw umber",0.15},{"yellow ochre",0.1}}
local o = function(p) return {hand="body", tool="filbert 6", pile=p, angle=0, angle_jitter=0.05, coverage=4, load=0.9, fill=true, length={15,35}, clip=true} end
work(z1, o(fA)); work(z2, o(fB)); work(z3, o(fC))

--@ chunk 59
local band = (col:grow(12) * below(function(x) return 477 end)) - refl2:grow(1)
local z1 = band * above(function(x) return 522 end)
local z2 = band * below(function(x) return 522 end) * above(function(x) return 612 end)
local z3 = band * below(function(x) return 612 end)
fA2 = pile{{"lead white",4},{"chrome yellow",1.0},{"vermilion",0.25},{"raw umber",0.12}}
fB2 = pile{{"lead white",4},{"smalt",0.9},{"vermilion",0.15},{"red earth",0.15},{"raw umber",0.15}}
fC2 = pile{{"lead white",3},{"smalt",1.3},{"Prussian blue",0.03},{"raw umber",0.3}}
local o = function(p) return {hand="body", tool="filbert 8", pile=p, angle=0, angle_jitter=0.03, coverage=2.5, load=0.8, fill=true, length={25,50}, clip=true, pressure={0.4,0.7}} end
work(z1, o(fA2)); work(z2, o(fB2)); work(z3, o(fC2))
blend(band:grow(6) - refl2:grow(2), {angle=0, angle_jitter=0.02, tool="badger 16", length={30,60}})
blend((band * (rect(0,510,1000,24) + rect(0,600,1000,24))):grow(4) - refl2:grow(2), {angle=1.5, tool="badger 10", length={12,26}})

--@ chunk 60
print(wait(40*60)); print(drying(472,520), drying(472,650), drying(700,650))

--@ chunk 61
print(wait(72*60)); print(drying(472,520), drying(472,650), drying(700,650), drying(200,455), drying(200,600))

--@ chunk 62
openW = (below(function(x) return 474 + 2*math.sin(x/33) end) - refl2) * rect(380, 0, 700, 800)
local c1 = function(x) return 530 + 4*math.sin(x/90) end
local c2 = function(x) return 560 + 5*math.sin(x/120+1) end
local c3 = function(x) return 604 + 4*math.sin(x/100+2) end
local c4 = function(x) return 632 + 5*math.sin(x/80+3) end
local c5 = function(x) return 672 + 4*math.sin(x/110+4) end
local w1 = openW * above(c1)
local w2 = openW * below(c1) * above(c2)
local w3 = openW * below(c2) * above(c3)
local w4 = openW * below(c3) * above(c4)
local w5 = openW * below(c4) * above(c5)
local w6 = openW * below(c5)
gw1 = pile{{"lead white",4},{"chrome yellow",1.0},{"vermilion",0.3},{"raw umber",0.1}}
gw2 = pile{{"lead white",5},{"chrome yellow",0.4},{"vermilion",0.25},{"pale smalt",0.3}}
gw3 = pile{{"lead white",3},{"smalt",0.8},{"red earth",0.35},{"vermilion",0.12},{"raw umber",0.15}}
gw4 = pile{{"lead white",4},{"smalt",1.0},{"yellow ochre",0.1},{"raw umber",0.2}}
gw5 = pile{{"lead white",3},{"smalt",0.9},{"red earth",0.3},{"raw umber",0.25}}
gw6 = pile{{"lead white",3},{"smalt",1.4},{"Prussian blue",0.04},{"raw umber",0.4}}
local o = function(p) return {hand="body", tool="filbert 14", pile=p, angle=0, angle_jitter=0.02, coverage=3, load=0.85, fill=true, length={80,200}, clip=openW} end
work(w1, o(gw1)); work(w2, o(gw2)); work(w3, o(gw3)); work(w4, o(gw4)); work(w5, o(gw5)); work(w6, o(gw6))

--@ chunk 63
local extra = (below(function(x) return 474 end) - refl2) * rect(300, 0, 82, 800)
local c4 = function(x) return 632 + 5*math.sin(x/80+3) end
local c5 = function(x) return 672 + 4*math.sin(x/110+4) end
local e1 = extra * above(c4)
local e2 = extra * below(c4) * above(c5)
local e3 = extra * below(c5)
local o = function(p) return {hand="body", tool="filbert 8", pile=p, angle=0, angle_jitter=0.02, coverage=3.5, load=0.85, fill=true, length={30,60}, clip=extra} end
work(e1, o(gw4)); work(e2, o(gw5)); work(e3, o(gw6))
openW = openW + extra
blend(openW, {angle=0, angle_jitter=0.015, tool="badger 30", length={80,220}})
local seams = nil
for _, y in ipairs({530, 560, 604, 632, 672}) do
  local r = rect(300, y-14, 800, 28)
  seams = seams and (seams + r) or r
end
blend(seams * openW, {angle=1.5, angle_jitter=0.15, tool="badger 16", length={20,40}})
blend(openW, {angle=0, angle_jitter=0.01, tool="badger 30", length={100,250}, pressure={0.25,0.4}})

--@ chunk 64
gw7 = pile{{"lead white",2},{"smalt",1.5},{"Prussian blue",0.06},{"raw umber",0.5},{"red earth",0.1}}
local bot = openW * below(function(x) return 655 + 6*math.sin(x/70) end)
work(bot, {hand="body", tool="filbert 12", pile=gw7, angle=0, angle_jitter=0.02, coverage=2.5, load=0.8, fill=true, length={80,200}, clip=openW})
local mid = openW * below(function(x) return 625 end)
blend(mid, {angle=0, angle_jitter=0.015, tool="badger 30", length={80,220}})
blend(openW * rect(0,635,1000,40), {angle=1.5, angle_jitter=0.15, tool="badger 16", length={25,50}})
blend(mid, {angle=0, angle_jitter=0.01, tool="badger 30", length={100,250}, pressure={0.25,0.4}})

--@ chunk 65
local s = openW * rect(0, 600, 1000, 50)
blend(s, {angle=1.5, angle_jitter=0.2, tool="badger 16", length={25,50}, clip=false})
blend(openW * rect(0, 590, 1000, 70), {angle=0, angle_jitter=0.02, tool="badger 24", length={80,200}, pressure={0.3,0.45}})
blend(openW * rect(0, 610, 1000, 30), {angle=1.5, angle_jitter=0.2, tool="badger 12", length={20,40}, pressure={0.25,0.4}, clip=false})

--@ chunk 66
print(drying(700,660), drying(700,620), drying(700,500))

--@ chunk 67
blend(openW * rect(0, 640, 1000, 40), {angle=1.5, angle_jitter=0.25, tool="badger 20", length={20,45}, clip=false, pressure={0.3,0.45}})
blend(openW * below(function(x) return 570 end), {angle=0, angle_jitter=0.02, tool="badger 30", length={120,300}, pressure={0.25,0.4}})

--@ chunk 68
local m = (ellipse(222, 158, 30, 13):roughen(5, 10, 141, 0) + ellipse(246, 180, 13, 10):roughen(4, 8, 142, 0) + ellipse(262, 170, 8, 6):roughen(3, 6, 143, 0))
stipple(m, {pile=pTC, width=5, coverage=3.2, pressure={0.6,0.95}, dips={14,0.9,0.2}, drag={2.5, 0.7}, twist=0.6, cluster={0.3,4}, clip=m:grow(1.5)})
trees = trees + m
-- sliver under the bank
local sl = rect(290, 468, 190, 12) * mask(function(x,y) return 1 end) - openW:grow(2)
local sl2 = sl * (refl2:grow(6))
work(sl2, {hand="detail", tool={kind="flat", width=4}, pile=pR1, coverage=4, load=0.95, fill=true})

--@ chunk 69
local b = brush{kind="flat", width=7}
for i, y in ipairs({471, 475, 478}) do
  b:reload(pR1, 0.95)
  b:stroke({{285, y}, {340, y+rand(-0.5,0.5)}, {390, y}}, {pressure={0.8,0.7}, orient="across"})
  b:reload(pR1, 0.95)
  b:stroke({{380, y}, {420, y+rand(-0.5,0.5)}, {462, y}}, {pressure={0.8,0.7}, orient="across"})
end

--@ chunk 70
local b = brush{kind="flat", width=6}
for _, x0 in ipairs({295, 335, 342}) do
  for _, y in ipairs({473, 476}) do
    b:reload(pR1, 1.0)
    b:stroke({{x0, y}, {x0+30, y}}, {pressure={0.9,0.9}, orient="across"})
  end
end

--@ chunk 71
print(wait(3*24*60)); print(drying(330,475), drying(700,660), drying(700,500), drying(222,158))

--@ chunk 72
local m = rect(300, 468, 80, 12)
work(m, {hand="detail", tool={kind="flat", width=5}, pile=pR1, coverage=4, load=1.0, fill=true, angle=0})

--@ chunk 73
hull = outline{{612,544,"c"},{628,549},{660,551},{694,549},{708,543,"c"},{700,547},{690,552},{662,555},{630,554},{618,550}, closed=true, char="firm", seed=151, amount=0.4}
hullM = hull:mask()
fig = body_of{spine={{672,545},{671,532},{670,522},{669,514}}, widths={5.5,5,5.5,4.2}, limbs={{{669,512},{669,506},widths={3.4,3.4}}, {{670,524},{664,530},{659,536},widths={2.2,2,1.8}}, {{670,522},{676,528},{681,531},widths={2,1.8,1.6}}}, char="soft", seed=152}
figM = fig:mask()
pBoat = pile{{"raw umber",1.5},{"bone black",0.7},{"smalt",0.3},{"red earth",0.2}}
work(hullM, {hand="detail", tool={kind="round", width=2.5, point=0.4}, pile=pBoat, coverage=4, load=0.95, fill=true})
work(figM, {hand="detail", tool={kind="round", width=2, point=0.4}, pile=pBoat, coverage=4, load=0.95, fill=true})
local r = brush{kind="rigger", width=1.6, point=1}
r:load(pBoat, 0.8)
r:stroke({{690,488},{676,520},{662,552},{656,566}}, {pressure={0.55,0.45}, ramps={0.05,0.1}})

--@ chunk 74
hull2 = poly({{606,541},{620,546},{645,548},{672,548},{698,546},{712,539},{706,547},{696,554},{670,557},{640,557},{622,553}}, true)
figure2 = poly({{657,551},{660,540},{662,528},{663,520},{666,517},{674,517},{678,521},{680,526},{678,527},{676,524},{676,532},{679,540},{682,551}}, true)
head2 = ellipse(670, 511, 3.8, 4.4)
boatAll = hull2 + figure2 + head2
work(boatAll, {hand="detail", tool={kind="round", width=2.5, point=0.4}, pile=pBoat, coverage=4, load=0.95, fill=true})

--@ chunk 75
pBoatR = pile{{"raw umber",1.3},{"bone black",0.5},{"smalt",0.6},{"lead white",0.5},{"red earth",0.15}}
local b = brush{kind="flat", width=4, stiffness=0.4}
-- hull reflection: a few horizontal strokes, wobbling, getting shorter and broken downward
local rows = {{560, 616, 706, 0.8},{564, 624, 700, 0.7},{568, 632, 694, 0.6},{573, 645, 690, 0.5},{578, 650, 684, 0.45},{584, 656, 682, 0.4},{590, 660, 680, 0.35},{597, 662, 678, 0.3},{604, 664, 676, 0.25}}
for i, r in ipairs(rows) do
  b:reload(pBoatR, 0.7)
  local y = r[1]
  local x0, x1 = r[2] + rand(-3,3), r[3] + rand(-3,3)
  if i > 4 and math.random() < 0.5 then
    local mid = lerp(x0, x1, rand(0.35, 0.65))
    b:stroke({{x0, y}, {mid - 2, y + rand(-0.6,0.6)}}, {pressure={r[4], r[4]*0.8}, orient="across"})
    b:reload(pBoatR, 0.6)
    b:stroke({{mid + 3, y + rand(-0.5,0.5)}, {x1, y}}, {pressure={r[4]*0.9, r[4]*0.7}, orient="across"})
  else
    b:stroke({{x0, y}, {(x0+x1)/2, y + rand(-0.6,0.6)}, {x1, y}}, {pressure={r[4], r[4]*0.8}, orient="across"})
  end
end
-- pole reflection: short broken wavy dashes
local r = brush{kind="rigger", width=1.6, point=1}
local segs = {{660,566,664,576},{666,580,669,589},{670,594,674,602},{676,608,678,614},{680,620,682,626}}
for i, s in ipairs(segs) do
  r:reload(pBoatR, 0.7)
  r:stroke({{s[1] + rand(-1,1), s[2]}, {(s[1]+s[3])/2 + rand(-1.5,1.5), (s[2]+s[4])/2}, {s[3], s[4]}}, {pressure={0.5, 0.2}, ramps={0.1,0.4}})
end

--@ chunk 76
local m = rect(596, 559, 130, 70)
blend(m, {tool="badger 8", angle=0, angle_jitter=0.06, length={10,26}, pressure={0.3,0.5}, coverage=1.5})

--@ chunk 77
local m = ellipse(662, 592, 50, 36):soften(8)
blend(m, {tool="badger 12", angle=0, angle_jitter=0.1, length={20,45}, pressure={0.3,0.5}, coverage=1.5})
local e = rect(585, 556, 160, 80) - rect(605, 566, 120, 60)
blend(e, {tool="badger 12", angle=0, length={20,40}, pressure={0.25,0.4}})

--@ chunk 78
print(drying(560,410), drying(560,500), drying(660,540))

--@ chunk 79
glowM = (ellipse(560, 408, 150, 30):soften(18) * above(function(x) return 424 end)) - trees:grow(2) - K3 
pGlow = pile{{"lead white",6},{"chrome yellow",1.1},{"vermilion",0.12}}
pGlow2 = pile{{"lead white",8},{"chrome yellow",0.8},{"vermilion",0.05}}
work(glowM, {hand="body", tool="filbert 10", pile=pGlow, angle=0, angle_jitter=0.05, coverage=2.5, load=0.7, pressure={0.35,0.6}, length={40,100}, fill=true})
local core = (ellipse(555, 412, 55, 12):soften(8)) - trees:grow(2)
work(core, {hand="body", tool="filbert 8", pile=pGlow2, angle=0, angle_jitter=0.05, coverage=2.5, load=0.7, pressure={0.4,0.6}, length={20,60}})
blend(glowM:grow(10) - trees:grow(3) - hills, {angle=0, angle_jitter=0.04, tool="badger 24", length={40,120}, clip=true})

--@ chunk 80
local m = rect(300, 360, 700, 72)
blend(m, {angle=0, angle_jitter=0.03, tool="badger 30", length={80,200}})
blend(m, {angle=0, angle_jitter=0.02, tool="badger 30", length={100,250}, pressure={0.25,0.4}})

--@ chunk 81
local m = rect(330, 350, 670, 22)
blend(m, {angle=1.5, angle_jitter=0.3, tool="badger 12", length={15,35}, clip=false, pressure={0.3,0.5}})

--@ chunk 82
print(wait(60*60)); print(drying(560,400), drying(420,420), drying(660,540), drying(900,380))

--@ chunk 83
local zone = trees * rect(285, 330, 200, 128)
local w = zone * warmPart:grow(2)
local c = zone - w
local ang = function(x,y) return 0.9*math.sin(x/35 + y/50) end
work(c, {hand="body", tool="filbert 7", pile=pTC, angle=ang, angle_jitter=0.6, coverage=3.5, load=0.95, fill=true, length={10,28}, clip=zone})
work(w, {hand="body", tool="filbert 7", pile=pTW, angle=ang, angle_jitter=0.6, coverage=3.5, load=0.95, fill=true, length={10,28}, clip=zone})

--@ chunk 84
gap = (rect(330, 352, 110, 82) - trees:grow(1))
pGap = pile{{"lead white",7},{"chrome yellow",0.9},{"vermilion",0.08},{"yellow ochre",0.1}}
work(gap, {hand="body", tool="filbert 5", pile=pGap, angle=0, angle_jitter=0.1, coverage=3.5, load=0.8, fill=true, length={10,30}, clip=gap})

--@ chunk 85
local r = (rect(420, 345, 40, 90) + rect(330, 340, 130, 22)) - trees:grow(3)
blend(r, {angle=0.3, angle_jitter=0.6, tool="badger 12", length={15,40}, pressure={0.25,0.4}, clip=false})

--@ chunk 86
print(wait(4*24*60)); print(drying(450,390), drying(420,380), drying(360,390), drying(660,540))

--@ chunk 87
print(wait(3*24*60)); print(drying(360,390), drying(300,300), drying(420,420))

--@ chunk 88
crown2 = outline{{384,440,"c"},{388,420},{394,404},{404,392},{414,384},{426,381},{438,386},{447,397},{452,410},{456,424},{462,440,"c"}, closed=true, char="soft", seed=171, lobe=7, amount=1.1}
crown2M = crown2:mask()
wz = (rect(380, 334, 170, 84):roughen(6, 20, 172, 0) - crown2M - trees:shrink(2)) * above(function(x) return 418 end)
work(wz, {hand="body", tool="filbert 6", pile=pGap, angle=0, angle_jitter=0.08, coverage=3.5, load=0.85, fill=true, length={15,40}, clip=wz})

--@ chunk 89
work(crown2M, {hand="body", tool="filbert 6", pile=pTC, angle=function(x,y) return 0.9*math.sin(x/20+y/25) end, angle_jitter=0.7, coverage=3.5, load=0.95, fill=true, length={8,20}, clip=crown2M})
local fringe = (crown2M:grow(6) - crown2M:shrink(2)) * above(function(x) return 436 end)
stipple(fringe, {pile=pTC, width=3.5, coverage=0.9, pressure={0.5,0.9}, dips={12,0.9,0.2}, drag={2, 0.8}, twist=0.6, cluster={0.7,4}, feather=0.4})
trees = trees + crown2M

--@ chunk 90
pRip2 = pile{{"lead white",3},{"smalt",1},{"red earth",0.3},{"raw umber",0.5},{"green earth",0.3}, medium=0.25}
pRip3 = pile{{"lead white",4},{"pale smalt",1},{"vermilion",0.1},{"yellow ochre",0.2}, medium=0.2}
local b = brush{kind="flat", width=2.5, stiffness=0.4}
local n = 0
for i = 1, 40 do
  local y = rand(490, 710)
  local t = (y - 480) / 230
  -- lines start from the reflection's right edge and run in toward the left
  local xe = nil
  for x = 480, 0, -3 do if refl2:at(x, y) > 0.5 then xe = x break end end
  if xe then
    local L = rand(30, 90) * (0.5 + t)
    local x1 = xe + rand(-5, 25)
    local x0 = x1 - L
    local p = (math.random() < 0.3) and pRip3 or pRip2
    b:reload(p, rand(0.3, 0.5))
    b:stroke({{x0, y}, {(x0+x1)/2, y + rand(-0.5,0.5)}, {x1, y}}, {pressure={0.15, 0.35}, ramps={0.4,0.2}, orient="across"})
    n = n + 1
  end
end
-- a few long faint lines across the middle of the reflection
for i = 1, 10 do
  local y = rand(520, 700)
  local x0 = rand(0, 150)
  local L = rand(80, 200)
  b:reload(pRip2, 0.3)
  b:stroke({{x0, y}, {x0 + L/2, y + rand(-0.5,0.5)}, {x0 + L, y}}, {pressure={0.1, 0.25}, ramps={0.3,0.3}, orient="across"})
end
print(n)

--@ chunk 91
blend(refl2 * below(function(x) return 485 end), {angle=0, angle_jitter=0.02, tool="badger 16", length={30,80}, pressure={0.35,0.55}, coverage=2})

--@ chunk 92
print(wait(2*24*60))
local top = {{-10,628}}
local x = -10
while x < 420 do
  x = x + rand(8, 22)
  local base = 628 + (x+10) * 0.08 + ((x > 250) and (x-250)*0.35 or 0)
  top[#top+1] = {x, base + rand(-7, 4)}
end
top[#top+1] = {440, 730}
fg = below(top):roughen(3, 14, 181, 0)
pFG = pile{{"raw umber",1.8},{"bone black",0.6},{"Prussian blue",0.25},{"yellow ochre",0.5},{"red earth",0.2}}
work(fg, {hand="body", tool="filbert 10", pile=pFG, angle=function(x,y) return -0.3 + 0.4*math.sin(x/30) end, angle_jitter=0.5, coverage=3.5, load=0.95, fill=true, length={15,40}, edge="soft"})

--@ chunk 93
local r = brush{kind="rigger", width=2.2, point=1}
local n = 0
-- find the bank's top edge along x
local function edgeY(x)
  for y = 560, 714, 2 do if fg:at(x, y) > 0.5 then return y end end
  return 714
end
local xs = uneven(70, 0, 430, 0.7, 0.6, 191)
for i, x in ipairs(xs) do
  local y0 = edgeY(x) + rand(2, 10)
  local tall = (x > 250) and rand(40, 110) or rand(25, 70)
  local lean = randn(0.15, 0.25)
  local bend = randn(0, 0.12)
  local pts = {}
  for k = 0, 4 do
    local t = k / 4
    local a = -math.pi/2 + lean * t + bend * t * t
    pts[#pts+1] = {x + tall * t * math.cos(-math.pi/2 + lean*t) * 0 + tall * t * math.sin(lean * t + bend*t), y0 - tall * t}
  end
  if n % 3 == 0 then r:reload(pFG, 0.85) end
  n = n + 1
  r:stroke(pts, {pressure={rand(0.6,0.8), 0.0}, ramps={0.02, 0.7}})
end
print(n)

--@ chunk 94
-- darker, cooler weight in the bank and a few warm grass notes catching the last light along its top
local nzb = noise{seed=201, octaves=3, period=60}
local dark = fg:shrink(12) * mask(function(x,y) return nzb(x,y) > 0.1 and 1 or 0 end)
pFG2 = pile{{"raw umber",1.4},{"bone black",0.9},{"Prussian blue",0.35},{"yellow ochre",0.2}}
work(dark, {hand="body", tool="filbert 8", pile=pFG2, angle=-0.4, angle_jitter=0.6, coverage=2.5, load=0.9, length={10,30}, clip=fg:shrink(4)})
local r = brush{kind="rigger", width=1.8, point=1}
pGrass = pile{{"yellow ochre",2},{"raw umber",0.8},{"red earth",0.3},{"lead white",0.4}}
local function edgeY(x)
  for y = 560, 714, 2 do if fg:at(x, y) > 0.5 then return y end end
  return 714
end
local xs = uneven(55, 5, 400, 0.7, 0.7, 202)
for i, x in ipairs(xs) do
  local y0 = edgeY(x) + rand(3, 12)
  local h = rand(10, 28)
  local lean = randn(0.1, 0.3)
  if i % 3 == 1 then r:reload(pGrass, 0.6) end
  r:stroke({{x, y0}, {x + h*0.3*lean, y0 - h*0.5}, {x + h*lean, y0 - h}}, {pressure={0.55, 0}, ramps={0.02, 0.7}})
end

--@ chunk 95
print(drying(100,690), drying(300,700))

--@ chunk 96
blend(fg:shrink(14), {angle=-0.3, angle_jitter=0.5, tool="badger 20", length={25,60}, pressure={0.35,0.55}, coverage=2})

--@ chunk 97
print(drying(560,495), drying(600,480))

--@ chunk 98
local skyLow = (ellipse(590, 425, 170, 26):soften(10)) * above(function(x) return 436 end) - trees:grow(4) - hills:grow(2)
pGlow3 = pile{{"lead white",9},{"chrome yellow",0.7},{"vermilion",0.04}}
work(skyLow, {hand="body", tool="filbert 6", pile=pGlow3, angle=0, angle_jitter=0.04, coverage=2.2, load=0.7, pressure={0.35,0.6}, length={30,80}, clip=skyLow})
blend(skyLow:grow(6) - trees:grow(5) - hills:grow(2), {angle=0, angle_jitter=0.03, tool="badger 16", length={40,100}, clip=true})

--@ chunk 99
print(wait(3*24*60)); print(drying(590,425), drying(450,380), drying(100,690))

--@ chunk 100
lowBand = (rect(300, 346, 710, 98) - trees:grow(1) - hills) 
local b1 = lowBand * above(function(x) return 372 + 4*math.sin(x/80) end)
local b2 = lowBand * below(function(x) return 372 + 4*math.sin(x/80) end) * above(function(x) return 404 + 3*math.sin(x/60) end)
local b3 = lowBand * below(function(x) return 404 + 3*math.sin(x/60) end)
lb1 = pile{{"lead white",6},{"chrome yellow",0.5},{"vermilion",0.18},{"pale smalt",0.25}}
lb2 = pile{{"lead white",7},{"chrome yellow",0.8},{"vermilion",0.12}}
lb3 = pile{{"lead white",9},{"chrome yellow",0.9},{"vermilion",0.06}}
local o = function(p) return {hand="body", tool="filbert 10", pile=p, angle=0, angle_jitter=0.03, coverage=3.2, load=0.85, fill=true, length={50,140}, clip=lowBand} end
work(b1, o(lb1)); work(b2, o(lb2)); work(b3, o(lb3))
blend(lowBand, {angle=0, angle_jitter=0.02, tool="badger 24", length={60,180}})

--@ chunk 101
seamBand = (rect(300, 334, 710, 24):roughen(4, 30, 211, 0)) - trees:grow(2)
pSeam = pile{{"lead white",6},{"chrome yellow",0.5},{"vermilion",0.3},{"pale smalt",0.35},{"red earth",0.05}}
work(seamBand, {hand="body", tool="filbert 8", pile=pSeam, angle=0, angle_jitter=0.1, coverage=2.5, load=0.7, pressure={0.35,0.6}, length={20,60}, clip=seamBand, fill=true})
blend(seamBand:grow(6) - trees:grow(3), {angle=1.5, angle_jitter=0.25, tool="badger 12", length={12,26}, clip=true})
blend(seamBand:grow(8) - trees:grow(3), {angle=0, angle_jitter=0.03, tool="badger 20", length={50,140}, clip=true, pressure={0.25,0.4}})

--@ chunk 102
print(wait(3*24*60)); print(drying(700,400), drying(700,430))

--@ chunk 103
local hm = hills * above(function(x) return 447 end)
pHill2 = pile{{"lead white",4},{"smalt",1.2},{"red earth",0.3},{"raw umber",0.2},{"vermilion",0.12}}
work(hm, {hand="body", tool="filbert 6", pile=pHill2, angle=0, angle_jitter=0.1, coverage=3.5, load=0.85, fill=true, length={20,60}, clip=hm})

--@ chunk 104
local ft = farTrees * above(function(x) return 452 end)
work(ft, {hand="body", tool="filbert 4", pile=pFar, angle=0, angle_jitter=0.4, coverage=3.5, load=0.9, fill=true, length={8,25}, clip=ft:grow(1)})

--@ chunk 105
local cl = nil
local xs = uneven(22, 480, 1000, 0.8, 0.8, 221)
for i, x in ipairs(xs) do
  local h = rand(3, 11)
  local w = rand(5, 16)
  local m = ellipse(x, 448 - h*0.5, w, h):roughen(2, 5, 230+i, 0)
  cl = cl and (cl + m) or m
end
distClumps = cl * above(function(x) return 452 end)
pFar2 = pile{{"lead white",1.2},{"smalt",1},{"raw umber",1.4},{"green earth",0.5},{"red earth",0.25}}
stipple(distClumps, {pile=pFar2, width=2.5, coverage=3, pressure={0.5,0.8}, dips={20,0.8,0.2}, drag={1.2, 0.5}, twist=0.6, clip=distClumps:grow(1)})

--@ chunk 106
print(wait(30*60)); print(drying(600,445), drying(700,440))

--@ chunk 107
work(distClumps, {hand="detail", tool={kind="round", width=2.5, point=0.3}, pile=pFar2, coverage=4, load=0.95, fill=true})

--@ chunk 108
shrub = outline{{380,472,"c"},{384,450},{398,440},{414,437},{430,436},{446,434},{458,437},{470,440},{482,446},{492,455},{496,464},{498,471,"c"},{440,473}, closed=true, char="soft", seed=241, lobe=6, amount=1.1}
shrubM = shrub:mask()
work(shrubM, {hand="body", tool="filbert 5", pile=pUnder, angle=function(x,y) return 0.6*math.sin(x/12) end, angle_jitter=0.6, coverage=3.8, load=0.95, fill=true, length={8,20}, clip=shrubM:grow(1)})
stipple((shrubM:grow(4) - shrubM) * above(function(x) return 462 end), {pile=pUnder, width=3, coverage=0.8, pressure={0.5,0.9}, dips={12,0.9,0.2}, drag={1.5,0.6}, twist=0.6, cluster={0.6,3}, feather=0.4})

--@ chunk 109
local z = trees:shrink(1.5) * rect(200, 140, 80, 50)
work(z, {hand="detail", tool={kind="round", width=3, point=0.3}, pile=pTC, coverage=4, load=1, fill=true})

--@ chunk 110
print(wait(24*60))
local T = trees + shrubM
rimM = (T - T:shrink(5)) * mask(function(x,y) return (T:at(x+9, y-3) < 0.5) and 1 or 0 end) * rect(0, 30, 520, 410) - rect(200,140,80,50)
pRim = pile{{"yellow ochre",2},{"red earth",0.4},{"lead white",0.8},{"chrome yellow",0.3},{"raw umber",0.3}}
stipple(rimM, {pile=pRim, width=2.5, coverage=0.45, pressure={0.3,0.6}, dips={10,0.6,0.3}, drag={1.5, 0.3}, twist=0.6, cluster={0.7,6}, feather=0.5})

--@ chunk 111
local b = brush{kind="flat", width=2.2, stiffness=0.4}
pGlint = pile{{"lead white",8},{"chrome yellow",0.9},{"vermilion",0.05}, medium=0.15}
-- short bright glints in the glow band, under the brightest sky
for i = 1, 22 do
  local y = rand(478, 520)
  local x = rand(500, 900)
  if math.abs(x - 660) > 40 or y < 530 then
    local L = rand(6, 22) * (1 - (y-478)/80)
    b:reload(pGlint, 0.35)
    b:stroke({{x, y}, {x + L, y + rand(-0.3,0.3)}}, {pressure={0.3, 0.15}, ramps={0.3,0.4}, orient="across"})
  end
end
-- long faint wind lines, lower right, a little lighter and cooler than the water
pWind = pile{{"lead white",5},{"pale smalt",1.2},{"vermilion",0.08}, medium=0.3}
for i = 1, 9 do
  local y = rand(600, 700)
  local x0 = rand(420, 800)
  local L = rand(90, 260)
  b:reload(pWind, 0.3)
  b:stroke({{x0, y}, {x0 + L*0.5, y + rand(-0.6,0.6)}, {math.min(x0 + L, 1000), y + rand(-0.6, 0.6)}}, {pressure={0.1, 0.25}, ramps={0.35,0.35}, orient="across"})
end

--@ chunk 112
pBird = pile{{"raw umber",1.5},{"bone black",0.6},{"smalt",0.3},{"lead white",0.3}}
local r = brush{kind="rigger", width=1.5, point=1}
local function bird(x, y, s, tilt)
  r:reload(pBird, 0.6)
  r:stroke({{x - 6*s, y - 2.5*s + tilt}, {x - 3*s, y - 3*s + tilt*0.5}, {x, y}}, {pressure={0.1, 0.6}, ramps={0.3, 0.1}})
  r:reload(pBird, 0.6)
  r:stroke({{x, y}, {x + 3*s, y - 3.2*s - tilt*0.5}, {x + 6.5*s, y - 2*s - tilt}}, {pressure={0.6, 0.1}, ramps={0.1, 0.3}})
end
bird(770, 368, 1.0, 0.5)
bird(796, 378, 0.8, -0.6)

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
