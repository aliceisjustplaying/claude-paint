-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
print(table.concat(tubes(), ", "))

--@ chunk 2
canvas{size=600, aspect=1.5, linen={18,15}, seed=7, ground={{pile={{"lead white",1}}, um=120, apply="knife", texture=0.3},{pile={{"lead white",3},{"yellow ochre",1},{"red earth",0.4}}, um=25, apply="brush"}}}

--@ chunk 3

h = pencil("HB")
h:rule({0, 440}, {1000, 440}, {pressure=0.25})
-- far shore and trees right
h:sketch({{560,436},{700,432},{760,420},{800,408},{850,404},{900,412},{950,418},{1000,420}}, {pressure=0.3})
-- barge hull
h:sketch({{215,448},{240,462},{360,464},{395,450}}, {pressure=0.35})
-- mast and sail
h:line({{300,452},{303,150}}, {pressure=0.35})
h:sketch({{303,165},{365,215},{380,300},{385,440}}, {pressure=0.3})
h:sketch({{300,170},{250,300},{238,445}}, {pressure=0.3})
-- foresail
h:sketch({{300,180},{215,440}}, {pressure=0.3})
-- mud spit lower left
h:sketch({{0,560},{120,545},{260,570},{380,610},{450,667}}, {pressure=0.3})
-- posts
h:line({{120,500},{122,575}}, {pressure=0.3})
h:line({{175,520},{176,585}}, {pressure=0.3})

--@ chunk 4
skyA = pile{{"lead white",3},{"smalt",1.5},{"raw umber",0.2}}
skyB = pile{{"lead white",3},{"smalt",0.7},{"red earth",0.35},{"raw umber",0.2}}
skyC = pile{{"lead white",4},{"yellow ochre",0.5},{"red earth",0.12},{"smalt",0.1}}
skyD = pile{{"lead white",5},{"chrome yellow",0.9},{"yellow ochre",0.3}}
local function band(y0,y1,seed)
  return below(function(x) return y0 + 14*math.sin(x/130+seed) end) * above(function(x) return y1 + 12*math.sin(x/110+seed*2) end)
end
local skyAll = rect(0,0,1000,446)
local ang = function(x,y) return 0.05*math.sin(x/200) end
work(band(-20,150,1)*skyAll, {hand="broad", pile=skyA, angle=ang, coverage=2.5, fill=true})
work(band(130,270,2)*skyAll, {hand="broad", pile=skyB, angle=ang, coverage=2.5, fill=true})
work(band(255,360,3)*skyAll, {hand="broad", pile=skyC, angle=ang, coverage=2.5, fill=true})
work(band(345,460,4)*skyAll, {hand="broad", pile=skyD, angle=ang, coverage=2.5, fill=true})

--@ chunk 5
blend(rect(0,0,1000,446), {angle=0})

--@ chunk 6
cloudD = pile{{"lead white",2},{"smalt",1},{"red earth",0.45},{"raw umber",0.45},{"bone black",0.08}}
cloudM = pile{{"lead white",3},{"smalt",0.8},{"red earth",0.5},{"raw umber",0.25}}
local n1 = noise{seed=11, octaves=5, period=160, stretch={0, 3.5}}
local n2 = noise{seed=12, octaves=4, period=90, stretch={0, 4}}
-- main bank: denser to the right, thinning left
bank = mask(function(x,y)
  local c = 205 + 25*math.sin(x/260+0.5) - 0.02*(x-500)
  local prof = 1 - ((y-c)/70)^2
  if prof <= 0 then return 0 end
  local v = n1(x,y)*0.9 + prof*0.9 + (x/1000)*0.35 - 0.55
  return clamp(v*4, 0, 1)
end)
-- high wisps
wisps = mask(function(x,y)
  local prof = 1 - ((y-60)/55)^2
  if prof <= 0 then return 0 end
  local v = n2(x,y) + prof*0.6 - 0.62
  return clamp(v*4,0,1)
end)
work(bank, {hand="body", pile=cloudD, angle=function(x,y) return -0.04 end, coverage=2, length={30,90}, pressure={0.4,0.8}})
work(wisps, {hand="body", pile=cloudM, angle=-0.05, coverage=1.5, length={30,80}, pressure={0.3,0.6}})

--@ chunk 7
blend(bank:grow(20):soften(10), {angle=-0.04}); blend(wisps:grow(15):soften(8), {angle=-0.05})

--@ chunk 8
skyTop = pile{{"lead white",2},{"smalt",1.6},{"cobalt blue",0.3},{"raw umber",0.2}}
horizGold = pile{{"lead white",3},{"yellow ochre",0.8},{"chrome yellow",0.6},{"vermilion",0.15}}
core = pile{{"lead white",6},{"chrome yellow",1}}
local top = mask(function(x,y) return clamp((110 - y)/50 + 0.5,0,1) end) - wisps:grow(6)
work(top, {hand="broad", pile=skyTop, angle=0.02, coverage=2, fill=true})
local hb = rect(0,412,1000,34):roughen(5,60,3)
work(hb, {hand="body", pile=horizGold, angle=0, coverage=2.5, length={40,120}, fill=true})
local c = ellipse(610,392,190,38):soften(20)
work(c, {hand="body", pile=core, angle=0, coverage=2, length={40,110}, fill=true})
blend(rect(0,0,1000,140):soften(20), {angle=0.02})
blend(rect(0,300,1000,146), {angle=0})

--@ chunk 9
strat = pile{{"lead white",2.5},{"smalt",0.6},{"red earth",0.5},{"raw umber",0.3}}
rimP = pile{{"lead white",4},{"vermilion",0.25},{"yellow ochre",0.6},{"chrome yellow",0.3}}
local b = brush("filbert", 7)
local streaks = {
  {40, 372, 330, 4}, {120, 382, 260, 3}, {760, 362, 230, 4}, {820, 374, 180, 3},
  {-10, 345, 190, 3}, {470, 340, 160, 2.5}, {880, 318, 140, 3}, {280, 322, 120, 2.5},
  {690, 402, 150, 2.5}, {0, 402, 120, 3},
}
for i, s in ipairs(streaks) do
  b:reload(strat, 0.5)
  local x0, y0, L, wv = s[1], s[2], s[3], s[4]
  local pts = {}
  for k = 0, 6 do
    local t = k/6
    pts[#pts+1] = {x0 + L*t, y0 + wv*math.sin(t*3.1 + i) - 3*t}
  end
  b:stroke(pts, {pressure={0.35, 0.05}, ramps={0.2, 0.5}, swell={0.8,1.1,0.7}})
end
-- lit underside of the bank
local under = bank:grow(4) - bank:shrink(3)
local lowerHalf = mask(function(x,y) local c = 205 + 25*math.sin(x/260+0.5) - 0.02*(x-500); return clamp((y - c)/10, 0, 1) end)
work(under*lowerHalf, {hand="detail", pile=rimP, angle=-0.04, coverage=1.2, pressure={0.3,0.6}})

--@ chunk 10
blend((bank:grow(14) - bank:shrink(12)):soften(6), {angle=-0.04}); blend(rect(0,300,1000,112):soften(8), {angle=0})

--@ chunk 11
spit = outline{{-20,556},{60,548},{140,547},{230,560},{320,590},{400,630},{450,690},{-20,690}, closed=true, char="soft", seed=5}:mask()
W1 = pile{{"lead white",3},{"yellow ochre",0.8},{"chrome yellow",0.4},{"red earth",0.1},{"smalt",0.12}}
W2 = pile{{"lead white",3},{"yellow ochre",0.3},{"red earth",0.25},{"smalt",0.55},{"raw umber",0.2}}
W3 = pile{{"lead white",2},{"smalt",1},{"raw umber",0.35},{"red earth",0.15}}
local water = rect(0,436,1000,240) - spit:shrink(4)
local function band(y0,y1) return rect(0,y0,1000,y1-y0):roughen(8,80,y0) end
work(water*band(434,500), {hand="body", pile=W1, angle=0, angle_jitter=0.02, coverage=2.5, length={40,120}, fill=true})
work(water*band(488,572), {hand="body", pile=W2, angle=0, angle_jitter=0.02, coverage=2.5, length={40,120}, fill=true})
work(water*band(560,680), {hand="body", pile=W3, angle=0, angle_jitter=0.02, coverage=2.5, length={40,120}, fill=true})
blend(water, {angle=0})

--@ chunk 12
print(wait(20*60)); print(drying(500,100), drying(600,220), drying(600,420), drying(600,600))

--@ chunk 13
print(wait(24*60)); print(drying(500,100), drying(600,220), drying(600,420), drying(600,600))

--@ chunk 14
shoreP = pile{{"lead white",1.6},{"smalt",1},{"red earth",0.35},{"raw umber",0.5}}
treeP = pile{{"lead white",1},{"smalt",1},{"red earth",0.3},{"raw umber",0.7},{"green earth",0.3}}
shoreline = function(x)
  local base = 437 + 2*math.sin(x/47) + 1.5*math.sin(x/13)
  -- tree clump on the right
  local t = 0
  if x > 720 and x < 980 then
    local u = (x-720)/260
    t = 30*math.sin(u*math.pi)^0.7 + 7*math.sin(x/9)*math.sin(u*math.pi) + 5*math.sin(x/23)
  end
  -- small clump left of centre, low
  if x > 430 and x < 520 then t = t + 9*math.sin((x-430)/90*math.pi) + 3*math.sin(x/7) end
  -- a few trees far left
  if x > 20 and x < 130 then t = t + 12*math.sin((x-20)/110*math.pi)^1.5 + 3*math.sin(x/6) end
  return base - math.max(t,0)
end
local pts = {}
for x = -10, 1010, 3 do pts[#pts+1] = {x, shoreline(x)} end
shore = below(pts) * above(function(x) return 448 + 1.5*math.sin(x/31) end)
shore = shore:roughen(2, 12, 4)
work(shore, {hand="detail", pile=shoreP, angle=0, coverage=2.5, fill=true})
local trees = shore * above(function(x) return 436 end)
work(trees, {hand="detail", pile=treeP, angle=-1.4, angle_jitter=0.5, coverage=1.5})

--@ chunk 15
treeD = pile{{"lead white",0.8},{"smalt",1},{"red earth",0.35},{"raw umber",0.9},{"green earth",0.4}}
local crowns = {
 {735,428,18,10},{758,421,17,15},{781,414,16,19},{803,404,7,30},{814,409,12,24},{838,413,22,19},
 {862,402,17,26},{884,409,15,22},{905,414,20,18},{930,421,20,14},{955,429,18,9},
 {447,431,12,6},{468,428,14,8},{492,430,14,7},{40,429,12,8},{62,425,10,12},{78,428,9,9},{100,431,12,6},
}
local m = nil
for i,c in ipairs(crowns) do
  local e = ellipse(c[1],c[2],c[3],c[4])
  m = m and (m + e) or e
end
m = m + rect(725,425,245,14)
m = m:roughen(4, 9, 21)
treesM = m * above(function(x) return 441 end)
stipple(treesM, {pile=treeD, width=3, coverage=5, pressure={0.4,0.8}, cluster=0.3})

--@ chunk 16
print(wait(240)); print(drying(850,425), drying(850,400)); stipple(treesM, {pile=treeD, width=3, coverage=4, pressure={0.4,0.8}, cluster=0.3})

--@ chunk 17
print(wait(20*60)); print(drying(850,425), drying(850,400), drying(300,442)); work(treesM:shrink(3), {hand="detail", pile=treeD, angle=-1.3, angle_jitter=0.6, coverage=2.5, fill=true})

--@ chunk 18
sailP = pile{{"red earth",1},{"raw umber",0.7},{"vermilion",0.1},{"lead white",0.15}}
sail2P = pile{{"red earth",1},{"raw umber",0.9},{"bone black",0.1},{"lead white",0.2},{"smalt",0.2}}
hullP = pile{{"bone black",1},{"raw umber",0.8},{"smalt",0.3}}
mainsail = outline{{303,208,"c"},{335,196},{371,186,"c"},{380,300},{391,437,"c"},{345,441},{304,441,"c"}, closed=true, char="firm", amount=0.4, seed=31}
topsail = outline{{303,150,"c"},{334,178},{363,193,"c"},{303,205,"c"}, closed=true, char="firm", amount=0.4, seed=32}
foresail = outline{{297,186,"c"},{296,440,"c"},{262,441},{226,442,"c"},{268,320}, closed=true, char="firm", amount=0.4, seed=33}
mizzen = outline{{393,388,"c"},{407,381,"c"},{411,445,"c"},{395,446,"c"}, closed=true, char="firm", amount=0.3, seed=34}
work(mainsail:mask(), {hand="body", tool="filbert 5", pile=sailP, angle=-1.5, angle_jitter=0.1, coverage=3, fill=true, edge="firm", length={20,50}})
work(topsail:mask(), {hand="body", tool="filbert 4", pile=sailP, angle=-1.5, coverage=3, fill=true, edge="firm", length={15,40}})
work(foresail:mask(), {hand="body", tool="filbert 5", pile=sail2P, angle=-1.25, coverage=3, fill=true, edge="firm", length={20,50}})
work(mizzen:mask(), {hand="detail", pile=sailP, angle=-1.5, coverage=3, fill=true})

--@ chunk 19
hull = outline{{214,441,"c"},{260,447},{340,448},{414,443,"c"},{407,462,"c"},{320,466},{232,462,"c"}, closed=true, char="firm", amount=0.3, seed=41}
work(hull:mask(), {hand="detail", pile=hullP, angle=0, coverage=3, fill=true})
-- the gap between sails and hull: deck clutter, dark
local deck = rect(226,438,188,8):roughen(2,8,3)
work(deck, {hand="detail", pile=hullP, angle=0, coverage=2, fill=true})

--@ chunk 20
print(wait(22*60)); print(drying(340,300), drying(260,400), drying(320,455))

--@ chunk 21
print(wait(20*60))
print(drying(340,300))
glazeU = pile{{"raw umber",1},{"red earth",0.5},{"bone black",0.1}, medium=0.6}
local sails = mainsail:mask() + topsail:mask() + foresail:mask() + mizzen:mask()
-- darken the lower sails and the forward parts
local lower = sails * mask(function(x,y) return clamp((y-230)/120,0,1) end)
work(lower, {hand="glaze", tool="filbert 10", pile=glazeU, angle=-1.5, coverage=1.5, clip=true})
work(foresail:mask(), {hand="glaze", tool="filbert 10", pile=glazeU, angle=-1.3, coverage=1.5, clip=true})

--@ chunk 22
sailsM = mainsail:mask() + topsail:mask() + foresail:mask() + mizzen:mask(); blend(sailsM:shrink(2), {angle=-1.5})

--@ chunk 23
print(wait(36*60)); print(drying(340,300), drying(340,200))
glazeU2 = pile{{"raw umber",1},{"red earth",0.7},{"smalt",0.15}, medium=0.7}
work(sailsM, {hand="glaze", tool="filbert 10", pile=glazeU2, angle=-1.5, coverage=1.2, clip=true})
blend(sailsM:shrink(2), {angle=-1.5})

--@ chunk 24
print(wait(24*60)); print(drying(340,300))
sailDark = pile{{"red earth",1},{"raw umber",1.1},{"bone black",0.12},{"lead white",0.1}}
sailMid = pile{{"red earth",1.2},{"raw umber",0.7},{"vermilion",0.1},{"lead white",0.15}}
local luffside = mask(function(x,y) return x < 340 and 1 or 0 end)
work(mainsail:mask()*luffside, {hand="body", tool="filbert 5", pile=sailDark, angle=-1.52, angle_jitter=0.08, coverage=2.5, fill=true, clip=true, length={25,60}})
work(mainsail:mask()-luffside, {hand="body", tool="filbert 5", pile=sailMid, angle=-1.55, angle_jitter=0.08, coverage=2.5, fill=true, clip=true, length={25,60}})
work(foresail:mask(), {hand="body", tool="filbert 5", pile=sailDark, angle=-1.25, angle_jitter=0.08, coverage=2.5, fill=true, clip=true, length={25,60}})
work(topsail:mask(), {hand="body", tool="filbert 4", pile=sailMid, angle=-1.5, coverage=2.5, fill=true, clip=true})
work(mizzen:mask(), {hand="detail", pile=sailDark, angle=-1.5, coverage=2.5, fill=true})
blend(mainsail:mask():shrink(2), {angle=-1.5})

--@ chunk 25
print(drying(500,550))
Wa = pile{{"lead white",3},{"yellow ochre",0.7},{"chrome yellow",0.3},{"red earth",0.15},{"smalt",0.2},{"raw umber",0.1}}
Wb = pile{{"lead white",2},{"smalt",0.7},{"red earth",0.4},{"raw umber",0.35},{"yellow ochre",0.2}}
Wc = pile{{"lead white",1.3},{"smalt",1},{"raw umber",0.55},{"red earth",0.2},{"Prussian blue",0.04}}
water2 = rect(0,449,1000,230) - spit:shrink(6) - hull:mask():grow(1)
local function band(y0,y1) return rect(0,y0,1000,y1-y0):roughen(8,90,y0+1) end
work(water2*band(446,520), {hand="body", pile=Wa, angle=0, angle_jitter=0.015, coverage=2.5, length={50,140}, fill=true})
work(water2*band(508,592), {hand="body", pile=Wb, angle=0, angle_jitter=0.015, coverage=2.5, length={50,140}, fill=true})
work(water2*band(580,680), {hand="body", pile=Wc, angle=0, angle_jitter=0.015, coverage=2.5, length={50,140}, fill=true})
blend(water2, {angle=0})

--@ chunk 26
reflP = pile{{"red earth",1},{"raw umber",1},{"lead white",0.5},{"smalt",0.2}}
reflDark = pile{{"raw umber",1},{"bone black",0.4},{"smalt",0.3},{"lead white",0.3}}
local rip = noise{seed=51, octaves=3, period=40, stretch={0, 6}}
local main_r = poly({{306,470},{390,468},{384,560},{374,640},{362,700},{314,700}}, true)
local fore_r = poly({{297,468},{230,468},{262,560},{290,660},{297,660}}, true)
local miz_r = poly({{395,466},{412,466},{410,520},{397,522}})
local breakm = mask(function(x,y) local fade = clamp(1 - (y-470)/260, 0, 1); return clamp((rip(x,y)*0.8 + fade*0.9 - 0.15)*3, 0, 1) end)
reflM = (main_r + fore_r + miz_r) * breakm * rect(0,462,1000,220)
local hullR = outline{{232,461,"c"},{320,462},{408,460,"c"},{400,474},{320,480},{240,474}, closed=true, char="soft", amount=0.6, seed=52}:mask()
work(reflM, {hand="hatch", pile=reflP, angle=0, angle_jitter=0.03, length={6,22}, coverage=1.8, pressure={0.3,0.6}})
work(hullR, {hand="detail", pile=reflDark, angle=0, coverage=2, fill=true})
blend(reflM:grow(6):soften(4) + hullR:grow(4), {angle=0, clip=false})

--@ chunk 27
print(wait(30*60)); print(drying(500,550), drying(330,500), drying(240,430))

--@ chunk 28
print(wait(24*60)); print(drying(500,550), drying(330,500), drying(240,430))

--@ chunk 29
skyFix = pile{{"lead white",5},{"chrome yellow",0.7},{"yellow ochre",0.7},{"vermilion",0.04}}
local blob = (ellipse(220,418,26,24) * rect(0,390,1000,48)) - foresail:mask():grow(1)
work(blob, {hand="detail", pile=skyFix, angle=0, coverage=3, fill=true})
local sh = rect(190,436,40,10) - foresail:mask()
work(sh, {hand="detail", pile=shoreP, angle=0, coverage=2.5, fill=true})
-- restate foresail edge crisp
work(foresail:mask(), {hand="detail", pile=sailDark, angle=-1.25, coverage=1.2, clip=true, threshold=0.5})

--@ chunk 30
blend((ellipse(220,418,34,32)*rect(0,380,1000,56)) - foresail:mask():grow(2), {angle=0})

--@ chunk 31
Wa2 = pile{{"lead white",3},{"yellow ochre",0.8},{"chrome yellow",0.3},{"red earth",0.15},{"smalt",0.25},{"raw umber",0.15}}
Wb2 = pile{{"lead white",2},{"smalt",0.8},{"red earth",0.45},{"raw umber",0.45},{"yellow ochre",0.15}}
Wc2 = pile{{"lead white",1},{"smalt",1},{"raw umber",0.7},{"red earth",0.2},{"Prussian blue",0.06}}
water3 = rect(0,449,1000,230) - spit:shrink(6) - hull:mask():grow(1)
local function band(y0,y1) return rect(0,y0,1000,y1-y0):roughen(8,90,y0+7) end
local o = {hand="body", angle=0, angle_jitter=0.015, coverage=2.5, length={50,140}, fill=true, clip=water3}
o.pile=Wa2; work(water3*band(446,505), o)
o.pile=Wb2; work(water3*band(492,578), o)
o.pile=Wc2; work(water3*band(565,680), o)
blend(water3, {angle=0})

--@ chunk 32
reflP2 = pile{{"red earth",1},{"raw umber",1.2},{"lead white",0.3},{"smalt",0.25}}
local rip = noise{seed=61, octaves=3, period=35, stretch={0, 7}}
local main_r = poly({{305,468},{392,466},{387,540},{378,620},{368,680},{318,680},{306,600}}, true)
local fore_r = poly({{296,468},{236,468},{258,530},{280,600},{296,640}}, true)
local miz_r = poly({{396,464},{411,464},{409,515},{398,518}})
local breakm = mask(function(x,y) local fade = clamp(1 - (y-468)/230, 0, 1); return clamp((rip(x,y)*0.9 + fade*1.0 - 0.2)*3, 0, 1) end)
reflM2 = (main_r + fore_r + miz_r) * breakm * rect(0,462,1000,220) - spit:shrink(6)
work(reflM2, {hand="hatch", pile=reflP2, angle=0, angle_jitter=0.03, length={8,26}, coverage=2, pressure={0.35,0.7}, clip=true})
blend(reflM2:grow(2), {angle=0})

--@ chunk 33
hull2 = outline{{214,440,"c"},{260,446},{340,447},{414,442,"c"},{407,461,"c"},{320,465},{232,461,"c"}, closed=true, char="firm", amount=0.25, seed=71}
local hm = hull2:mask() + rect(228,436,184,8)
work(hm, {hand="detail", pile=hullP, angle=0, coverage=3.5, fill=true, clip=true})
hullR2 = outline{{234,462,"c"},{320,466},{406,462,"c"},{396,472},{320,478},{246,472}, closed=true, char="soft", amount=0.5, seed=72}:mask()
work(hullR2, {hand="detail", pile=reflDark, angle=0, coverage=2.5, fill=true})

--@ chunk 34
print(wait(26*60)); print(drying(330,520), drying(320,452), drying(700,600), drying(220,418))

--@ chunk 35
print(wait(24*60)); print(drying(330,520), drying(320,452), drying(700,600), drying(220,418))

--@ chunk 36
waterGlaze = pile{{"raw umber",1},{"smalt",1},{"Prussian blue",0.1},{"red earth",0.15}, medium=0.45}
local low = (rect(0,530,1000,150):roughen(10,120,3)) - spit:shrink(6)
work(low, {hand="glaze", pile=waterGlaze, angle=0, coverage=1.2})
local lower2 = (rect(0,595,1000,90):roughen(10,120,4)) - spit:shrink(6)
work(lower2, {hand="glaze", pile=waterGlaze, angle=0, coverage=1.0})
blend(rect(0,520,1000,160) - spit:shrink(6), {angle=0})

--@ chunk 37
local wm = rect(0,500,1000,180) - spit:shrink(6) - reflM2:grow(3)
local trans = wm * mask(function(x,y) return clamp(1-(y-505)/60,0,1) end)
work(trans, {hand="body", pile=Wb2, angle=0, angle_jitter=0.01, coverage=2, length={60,160}, pressure={0.3,0.6}, threshold=0.2})
-- ripple lights
local b = brush("filbert", 4)
local ys = uneven(26, 520, 665, 0.6, 0.4, 81)
for i, y in ipairs(ys) do
  local nseg = math.random(2,4)
  for k = 1, nseg do
    local x0 = rand(-40, 1000)
    local L = rand(60, 220) * (1 - (y-520)/300)
    local pile_ = (math.random() < 0.6) and Wb2 or skyA
    b:reload(pile_, rand(0.3,0.6))
    local w = rand(0.5, 2)
    b:stroke({{x0, y}, {x0 + L*0.5, y + w*0.5}, {x0 + L, y + rand(-1,1)}}, {pressure={rand(0.2,0.45), 0.05}, ramps={0.1,0.5}, clip=wm})
  end
end

--@ chunk 38
blend(rect(0,488,1000,120) - spit:shrink(6) - reflM2:grow(3), {angle=0})

--@ chunk 39
mudD = pile{{"raw umber",1},{"red earth",0.3},{"bone black",0.25},{"lead white",0.25},{"green earth",0.3}}
mudM = pile{{"raw umber",1},{"yellow ochre",0.4},{"red earth",0.2},{"lead white",0.6},{"smalt",0.2}}
spit2 = outline{{-20,566},{50,560},{130,561},{210,572},{290,596},{360,630},{410,672},{-20,690}, closed=true, char="soft", seed=91}:mask()
work(spit2, {hand="body", pile=mudD, angle=function(x,y) return 0.12 + (x/1000)*0.3 end, angle_jitter=0.1, coverage=3, length={20,70}, fill=true})
-- a lighter muddy top edge where the bank catches the sky
local lip = spit2 - spit2:offset(-7)
local lipTop = lip * mask(function(x,y) return 1 end)
work(lip, {hand="detail", pile=mudM, angle=0.2, coverage=1.2, pressure={0.3,0.6}})

--@ chunk 40
mudDD = pile{{"raw umber",1},{"bone black",0.35},{"smalt",0.25},{"red earth",0.15}}
local inner = spit2:shrink(12)
work(inner * mask(function(x,y) return clamp((y-580)/40,0,1) end), {hand="body", pile=mudDD, angle=0.15, angle_jitter=0.1, coverage=2, length={30,90}})
blend(spit2:shrink(2), {angle=0.12})

--@ chunk 41
print(wait(36*60)); print(drying(100,620), drying(600,560), drying(330,520))

--@ chunk 42
glowP = pile{{"red earth",0.6},{"vermilion",0.4},{"chrome yellow",0.5},{"lead white",0.6}}
foldP = pile{{"raw umber",1},{"red earth",0.5},{"bone black",0.15}}
lineP = pile{{"bone black",1},{"raw umber",0.6}}
-- leech glow: a band inside the right edge of the mainsail
local leech = mainsail:mask() - mainsail:mask():offset(-9)
local rightside = mask(function(x,y) return clamp((x - 360)/12, 0, 1) end)
work(leech*rightside, {hand="scumble", tool="filbert 3", pile=glowP, angle=-1.5, coverage=1.2, pressure={0.2,0.45}, clip=mainsail:mask()})
-- topsail catches more light
work(topsail:mask():shrink(3), {hand="scumble", tool="filbert 3", pile=glowP, angle=-1.2, coverage=0.7, pressure={0.15,0.35}, clip=topsail:mask()})
-- crease along the sprit, peak to tack
local fb = brush("filbert", 5)
fb:reload(foldP, 0.5)
fb:stroke({{368,195},{350,260},{330,340},{312,420}}, {pressure={0.5,0.2}, ramps={0.1,0.4}, clip=mainsail:mask()})
fb:reload(foldP, 0.35)
fb:stroke({{372,200},{365,290},{352,380},{345,436}}, {pressure={0.4,0.15}, ramps={0.2,0.5}, clip=mainsail:mask()})
-- mast, topmast
local r = brush{kind="rigger", width=2.2, point=1}
r:reload(lineP, 0.8)
r:stroke({{301,446},{301.5,300},{302,190},{302.5,132}}, {pressure={0.75,0.35}, ramps={0.02,0.2}})
-- sprit spar
r:reload(lineP, 0.7)
r:stroke({{305,425},{330,340},{352,260},{373,184}}, {pressure={0.6,0.35}})
-- forestay to the stem, shrouds, vang
local t = brush{kind="rigger", width=1.2, point=1}
t:reload(lineP, 0.5)
t:stroke({{302,150},{258,300},{217,440}}, {pressure={0.35,0.4}})
t:reload(lineP, 0.4)
t:stroke({{302,205},{318,445}}, {pressure={0.3,0.35}})
t:reload(lineP, 0.4)
t:stroke({{373,186},{392,300},{410,442}}, {pressure={0.25,0.3}})
-- pennant
local pen = poly({{303,134},{322,137},{303,142}})
work(pen, {hand="detail", pile=pile{{"vermilion",1},{"red earth",0.3}}, coverage=2, fill=true})

--@ chunk 43
blend(mainsail:mask():shrink(1.5), {angle=-1.5}); blend(topsail:mask():shrink(1.5), {angle=-1.3})

--@ chunk 44
print(wait(48*60)); print(drying(340,300), drying(100,620))

--@ chunk 45
sailDeep = pile{{"red earth",1},{"raw umber",1.3},{"bone black",0.15},{"lead white",0.08}}
sailMid2 = pile{{"red earth",1.2},{"raw umber",0.8},{"vermilion",0.08},{"lead white",0.12}}
local mm = mainsail:mask()
local left = mask(function(x,y) return x < 348 + 10*math.sin(y/40) and 1 or 0 end)
work(mm*left, {hand="body", tool="filbert 5", pile=sailDeep, angle=-1.5, angle_jitter=0.06, coverage=3, fill=true, clip=mm, length={30,70}})
work(mm-left, {hand="body", tool="filbert 5", pile=sailMid2, angle=-1.53, angle_jitter=0.06, coverage=3, fill=true, clip=mm, length={30,70}})
work(topsail:mask(), {hand="body", tool="filbert 4", pile=sailMid2, angle=-1.3, coverage=3, fill=true, clip=topsail:mask()})
local seam = mm * mask(function(x,y) local c = 348 + 10*math.sin(y/40); return clamp(1 - math.abs(x-c)/14, 0, 1) end)
blend(seam, {angle=-1.5, tool="filbert 8"})

--@ chunk 46
print(wait(30*60)); print(drying(340,300))
glowThin = pile{{"red earth",0.5},{"vermilion",0.35},{"chrome yellow",0.4},{"lead white",0.35}, medium=0.3}
local r = brush{kind="rigger", width=2.4, point=1}
r:reload(lineP, 0.8)
r:stroke({{306,430},{328,345},{350,262},{372,186}}, {pressure={0.65,0.4}, ramps={0.05,0.15}})
-- a thin light along the leech, backlit
local g = brush{kind="round", width=3, point=0.8}
g:reload(glowThin, 0.35)
g:stroke({{372,192},{378,260},{384,340},{389,430}}, {pressure={0.35,0.15}, ramps={0.05,0.5}, clip=mainsail:mask()})
g:reload(glowThin, 0.3)
g:stroke({{305,150},{330,172},{360,190}}, {pressure={0.3,0.15}, ramps={0.05,0.4}, clip=topsail:mask()})

--@ chunk 47
mudDD2 = pile{{"raw umber",1},{"bone black",0.3},{"smalt",0.2},{"red earth",0.2},{"lead white",0.12}}
sheen = pile{{"lead white",2},{"smalt",0.5},{"red earth",0.3},{"raw umber",0.3},{"yellow ochre",0.3}}
local sp = spit2:shrink(3)
work(sp, {hand="body", pile=mudDD2, angle=function(x,y) return 0.1 + x/2000 end, angle_jitter=0.08, coverage=2.5, length={30,90}, fill=true, clip=spit2})
-- wet sheen streaks on the mud, following the bank
local b = brush("filbert", 4)
local streaks = {{10,585,120},{60,600,150},{-10,622,90},{140,618,110},{30,648,160},{200,640,90},{240,625,40},{90,575,60}}
for i, s in ipairs(streaks) do
  b:reload(sheen, 0.3)
  local x0, y0, L = s[1], s[2], s[3]
  b:stroke({{x0,y0},{x0+L*0.5,y0+L*0.08+2},{x0+L,y0+L*0.18}}, {pressure={0.3,0.05}, ramps={0.15,0.5}, clip=spit2:shrink(8)})
end

--@ chunk 48
blend(spit2:shrink(6), {angle=0.2})

--@ chunk 49
print(wait(48*60)); print(drying(100,620), drying(330,520), drying(340,300))

--@ chunk 50
WbC = pile{{"lead white",1.5},{"smalt",0.9},{"red earth",0.35},{"raw umber",0.6},{"yellow ochre",0.1}}
local zone = rect(150,466,370,118):roughen(10,50,5) - spit2:grow(2) - hull2:mask():grow(1)
local function band(y0,y1) return rect(0,y0,1000,y1-y0):roughen(5,60,y0+3) end
local o = {hand="body", tool="filbert 6", angle=0, angle_jitter=0.01, coverage=2.5, length={30,90}, fill=true, clip=zone}
o.pile=Wa2; work(zone*band(462,494), o)
o.pile=Wb2; work(zone*band(488,545), o)
o.pile=WbC; work(zone*band(538,590), o)
blend(zone:shrink(4), {angle=0})

--@ chunk 51
Wd = pile{{"lead white",0.8},{"smalt",1},{"raw umber",0.8},{"red earth",0.2},{"Prussian blue",0.08}}
waterAll = rect(0,457,1000,220) - spit2:shrink(3) - hull2:mask():grow(0.5)
local function band(y0,y1,s) return rect(0,y0,1000,y1-y0):roughen(10,140,s) end
local o = {hand="body", tool="filbert 8", angle=0, angle_jitter=0.012, coverage=2.5, length={60,160}, fill=true, clip=waterAll}
o.pile=Wa2; work(waterAll*band(455,496,1), o)
o.pile=WbC; work(waterAll*band(490,545,2), o)
o.pile=Wd;  work(waterAll*band(538,680,3), o)
blend(waterAll*rect(0,480,1000,80), {angle=0})

--@ chunk 52
reflRuss = pile{{"raw umber",1},{"red earth",0.9},{"bone black",0.15},{"lead white",0.12}}
reflHull = pile{{"bone black",1},{"raw umber",1},{"smalt",0.4},{"lead white",0.1}}
local rip = noise{seed=77, octaves=3, period=30, stretch={0, 8}}
-- the sails seen upside down in the water, drawn freshly as shapes
local main_r3 = poly({{304,474},{391,473},{388,520},{383,580},{376,640},{360,690},{312,690},{305,600}}, true)
local fore_r3 = poly({{296,474},{234,474},{250,515},{268,570},{286,630},{296,660}}, true)
local miz_r3 = poly({{395,470},{411,470},{410,515},{397,517}})
local brk = mask(function(x,y) local fade = clamp(1.1 - (y-474)/200, 0, 1); return clamp((rip(x,y)*0.8 + fade - 0.25)*3, 0, 1) end)
reflM3 = (main_r3 + fore_r3 + miz_r3) * brk * rect(0,470,1000,210) - spit2:grow(2)
work(reflM3, {hand="hatch", pile=reflRuss, angle=0, angle_jitter=0.02, length={10,30}, coverage=2.2, pressure={0.4,0.75}})
-- hull reflection: a dark band right under the hull
local hr = outline{{224,462,"c"},{320,466},{410,461,"c"},{402,471},{320,477},{238,471}, closed=true, char="soft", amount=0.5, seed=78}:mask()
work(hr, {hand="hatch", pile=reflHull, angle=0, length={8,24}, coverage=2.5, pressure={0.5,0.8}})

--@ chunk 53
spit3 = outline{{-30,574},{40,568},{110,566},{190,575},{260,594},{320,620},{370,652},{400,700,"c"},{-30,700,"c"}, closed=true, char="firm", amount=0.5, seed=95}:mask()
mudA = pile{{"raw umber",1},{"bone black",0.3},{"red earth",0.25},{"smalt",0.15},{"lead white",0.1}}
work(spit3, {hand="body", pile=mudA, angle=function(x,y) return 0.12 + x/2500 end, angle_jitter=0.08, coverage=3, length={30,90}, fill=true, edge={found=0.5, soft=0.5, period=50}})

--@ chunk 54
local bot = rect(-10,630,420,50) * spit3
work(bot, {hand="body", pile=mudA, angle=0.1, coverage=3, fill=true, length={30,80}, clip=rect(-10,600,420,80)*spit3:grow(1)})
grassD = pile{{"raw umber",1},{"bone black",0.3},{"green earth",0.4},{"yellow ochre",0.15}}
grassL = pile{{"yellow ochre",1},{"raw umber",0.4},{"red earth",0.2},{"lead white",0.3}}
-- edge line of the bank, sampled along my drawn curve
local edgepts = {{-5,572},{40,566},{110,564},{190,573},{260,592},{320,618},{370,650},{395,672}}
local function edgeAt(t)
  local n = #edgepts - 1
  local f = t*n; local i = math.floor(f); if i >= n then i = n-1 end
  local u = f - i
  local a, b = edgepts[i+1], edgepts[i+2]
  return a[1] + (b[1]-a[1])*u, a[2] + (b[2]-a[2])*u
end
local r = brush{kind="rigger", width=1.4, point=1}
local ts = uneven(170, 0, 1, 0.8, 0.7, 101)
for i, t in ipairs(ts) do
  local x, y = edgeAt(t)
  y = y + rand(0, 6)
  local h = rand(5, 22) * (1 - 0.4*t)
  local lean = randn(0.15, 0.25)
  local p = (math.random() < 0.8) and grassD or grassL
  r:reload(p, rand(0.4,0.7))
  r:stroke({{x, y}, {x + lean*h*0.4, y - h*0.55}, {x + lean*h, y - h}}, {pressure={rand(0.4,0.7), 0}, ramps={0.05, 0.8}})
end

--@ chunk 55
print(wait(180))
local edgepts = {{-5,572},{40,566},{110,564},{190,573},{260,592},{320,618},{370,650},{395,672}}
local function edgeAt(t)
  local n = #edgepts - 1
  local f = t*n; local i = math.floor(f); if i >= n then i = n-1 end
  local u = f - i
  local a, b = edgepts[i+1], edgepts[i+2]
  return a[1] + (b[1]-a[1])*u, a[2] + (b[2]-a[2])*u
end
local r = brush{kind="rigger", width=1.8, point=1}
local clumps = uneven(22, 0.0, 0.93, 0.7, 0.3, 111)
for _, c in ipairs(clumps) do
  local nb = math.random(4, 10)
  local H = rand(14, 40) * (1 - 0.35*c)
  for k = 1, nb do
    local t = c + randn(0, 0.008)
    local x, y = edgeAt(clamp(t,0,1))
    y = y + rand(1, 8)
    local h = H * rand(0.5, 1.1)
    local lean = randn(0.2, 0.3)
    local p = (math.random() < 0.7) and grassD or grassL
    r:reload(p, rand(0.45,0.75))
    r:stroke({{x, y}, {x + lean*h*0.3, y - h*0.5}, {x + lean*h, y - h}}, {pressure={rand(0.5,0.8), 0}, ramps={0.05, 0.85}})
  end
end

--@ chunk 56
print(wait(40*60)); print(drying(100,620), drying(330,520), drying(700,600))

--@ chunk 57
mudGlaze = pile{{"bone black",0.6},{"raw umber",1},{"smalt",0.2}, medium=0.5}
local corner = spit3 * mask(function(x,y) return clamp(((y-600) - 0.35*x)/30 + 0.5, 0, 1) end)
work(corner, {hand="glaze", pile=mudGlaze, angle=0.15, coverage=1.2, clip=spit3})
sheen2 = pile{{"lead white",2},{"smalt",0.4},{"red earth",0.25},{"raw umber",0.3},{"yellow ochre",0.4}}
local sheenZone = spit3:shrink(10) * mask(function(x,y) local v = 1 - ((y-600) - 0.35*x)/40; return clamp(v,0,1) end)
work(sheenZone, {hand="scumble", tool="filbert 6", pile=sheen2, angle=function(x,y) return 0.18 + x/2500 end, coverage=0.5, pressure={0.12,0.3}, load=0.25, length={20,60}, clip=spit3:shrink(4)})

--@ chunk 58
blend(spit3:shrink(3), {angle=0.2}); blend(spit3:shrink(3), {angle=0.25})

--@ chunk 59
print(wait(36*60)); print(drying(100,620), drying(200,640))

--@ chunk 60
mudCool = pile{{"raw umber",1},{"bone black",0.3},{"smalt",0.45},{"lead white",0.3},{"red earth",0.1}}
mudWarm = pile{{"raw umber",1},{"bone black",0.35},{"red earth",0.35},{"lead white",0.08}}
local upper = spit3 * mask(function(x,y) local v = 1 - ((y-590) - 0.33*x)/25; return clamp(v,0,1) end)
local lower = spit3 - upper
local o = {hand="body", tool="filbert 8", angle=function(x,y) return 0.15 + x/2500 end, angle_jitter=0.08, coverage=3, length={30,90}, fill=true, clip=spit3}
o.pile = mudCool; work(upper, o)
o.pile = mudWarm; work(lower, o)

--@ chunk 61
print(wait(36*60)); print(drying(100,620), drying(200,640))
mudDark = pile{{"raw umber",1},{"bone black",0.5},{"smalt",0.2},{"red earth",0.2}}
local o = {hand="body", tool="filbert 8", pile=mudDark, angle=function(x,y) return 0.12 + x/2500 end, angle_jitter=0.1, coverage=3.5, length={25,80}, fill=true, clip=spit3, load=0.9}
work(spit3, o)

--@ chunk 62
print(wait(30*60)); print(drying(100,620))
stakeP = pile{{"raw umber",1},{"bone black",0.6},{"red earth",0.15}}
puddleP = pile{{"lead white",2.2},{"yellow ochre",0.5},{"chrome yellow",0.15},{"red earth",0.15},{"smalt",0.2},{"raw umber",0.15}}
puddleP2 = pile{{"lead white",1.6},{"smalt",0.7},{"red earth",0.3},{"raw umber",0.4}}
-- puddles, drawn as thin flat lenses
local p1 = outline{{40,606},{75,603},{112,605},{80,609}, closed=true, char="soft", amount=0.4, seed=121}:mask()
local p2 = outline{{130,626},{175,621},{215,625},{170,630}, closed=true, char="soft", amount=0.4, seed=122}:mask()
local p3 = outline{{10,641},{60,637},{95,640},{55,645}, closed=true, char="soft", amount=0.4, seed=123}:mask()
work(p1 + p2, {hand="detail", pile=puddleP, angle=0.05, coverage=2, fill=true, clip=true})
work(p3, {hand="detail", pile=puddleP2, angle=0.05, coverage=2, fill=true, clip=true})
-- stakes: old mooring posts leaning a little
local s = brush("flat", 4.5)
s:reload(stakeP, 0.9)
s:stroke({{118,578},{119.5,545},{121,512}}, {pressure={0.9,0.8}, ramps={0.02,0.05}, orient="across"})
s:reload(stakeP, 0.9)
s:stroke({{168,584},{167,560},{166,536}}, {pressure={0.85,0.75}, ramps={0.02,0.05}, orient="across"})
s:reload(stakeP, 0.8)
s:stroke({{205,590},{206,575},{207,560}}, {pressure={0.75,0.7}, ramps={0.02,0.05}, orient="across"})

--@ chunk 63
local clipW = rect(0,452,1000,220) - spit3:grow(2) - hull2:mask():grow(1)
local b = brush("filbert", 4)
function ripple_lines(n, y0, y1, piles, Lmin, Lmax, pmin, pmax, xs0, xs1)
  for i = 1, n do
    local y = rand(y0, y1)
    local x = rand(xs0 or -50, xs1 or 1000)
    local L = rand(Lmin, Lmax)
    local p = piles[math.random(#piles)]
    b:reload(p, rand(0.3, 0.55))
    local dy = rand(-0.8, 0.8)
    b:stroke({{x, y}, {x + L*0.5, y + dy}, {x + L, y + rand(-0.5,0.5)}}, {pressure={rand(pmin,pmax), rand(0.05,0.2)}, ramps={0.15,0.5}, clip=clipW})
  end
end
ripple_lines(45, 470, 492, {Wa2, WbC, Wb2}, 40, 160, 0.2, 0.4)
ripple_lines(45, 548, 575, {WbC, Wd, Wb2}, 40, 180, 0.2, 0.45)
ripple_lines(30, 500, 545, {Wb2, WbC}, 60, 200, 0.15, 0.35)
ripple_lines(40, 580, 667, {Wd, WbC, Wb2}, 60, 220, 0.2, 0.45)

--@ chunk 64
millP = pile{{"lead white",0.6},{"smalt",1},{"red earth",0.3},{"raw umber",0.9}}
farSailP = pile{{"lead white",1.8},{"smalt",0.7},{"red earth",0.5},{"raw umber",0.4}}
-- windmill tower on the far shore
local tower = poly({{575,438},{584,438},{581.5,421},{577.5,421}})
local cap = ellipse(579.5,420,3.2,2.2)
work(tower + cap, {hand="detail", pile=millP, coverage=3, fill=true, clip=true})
local r = brush{kind="rigger", width=1.3, point=1}
local arms = {{-11,-8},{11,8},{-9,10},{9,-10}}
for _, a in ipairs(arms) do
  r:reload(millP, 0.6)
  r:stroke({{579.5,419.5},{579.5 + a[1], 419.5 + a[2]}}, {pressure={0.55,0.35}})
end
-- a distant sail far off to the right
local fs = outline{{661,417,"c"},{666,437,"c"},{655,437,"c"}, closed=true, char="firm", amount=0.2, seed=141}:mask()
work(fs, {hand="detail", pile=farSailP, coverage=3, fill=true, clip=true})
local fh = poly({{651,437},{670,437},{668,440},{654,440}})
work(fh, {hand="detail", pile=millP, coverage=3, fill=true, clip=true})

--@ chunk 65
discFix = pile{{"lead white",4},{"yellow ochre",0.9},{"chrome yellow",0.25},{"red earth",0.08},{"smalt",0.1}, medium=0.3}
local dm = (ellipse(214,416,32,28) * rect(0,380,1000,56)) - foresail:mask():grow(3)
work(dm, {hand="detail", pile=discFix, angle=0, coverage=1.5, pressure={0.2,0.4}, fill=true, clip=true})
blend(dm:grow(4):soften(6) - foresail:mask():grow(3), {angle=0})
-- mast reflection: a thin wavering dark line in the water
local r = brush{kind="rigger", width=1.6, point=1}
r:reload(lineP, 0.6)
local pts = {}
for i = 0, 10 do local y = 474 + i*9; pts[#pts+1] = {301 + 2.2*math.sin(i*1.9), y} end
r:stroke(pts, {pressure={0.5,0.05}, ramps={0.05,0.6}})

--@ chunk 66
print(wait(30*60)); print(drying(301,520), drying(80,606), drying(320,470))

--@ chunk 67
-- darker hull reflection right under the hull
local hr2 = outline{{226,462,"c"},{320,466},{410,461,"c"},{400,470},{320,474},{240,470}, closed=true, char="soft", amount=0.4, seed=151}:mask()
work(hr2, {hand="hatch", pile=reflHull, angle=0, length={8,24}, coverage=2.5, pressure={0.5,0.8}})
-- dull the puddles: thin glaze of mud over them
puddleGlaze = pile{{"raw umber",1},{"smalt",0.3},{"red earth",0.2}, medium=0.55}
local pz = ellipse(76,606,45,8) + ellipse(172,625,52,8) + ellipse(52,641,50,8)
work(pz*spit3, {hand="glaze", tool="filbert 8", pile=puddleGlaze, angle=0.05, coverage=1.3, clip=spit3})
-- light ripple breaks across the reflection
local b = brush("filbert", 3)
local rl = {{250,488,120},{280,503,90},{230,519,100},{300,534,80},{260,551,110},{315,566,70},{275,583,90},{330,598,60}}
for _, s in ipairs(rl) do
  b:reload(Wb2, 0.35)
  b:stroke({{s[1],s[2]},{s[1]+s[3]*0.5,s[2]+0.5},{s[1]+s[3],s[2]}}, {pressure={0.3,0.1}, ramps={0.2,0.4}, clip=rect(0,470,1000,200)-spit3:grow(2)})
end

--@ chunk 68
local b = brush("filbert", 3)
for i, y in ipairs({487, 499, 514, 528, 541, 556}) do
  local p = (i % 2 == 0) and Wb2 or reflRuss
  b:reload(p, 0.45)
  b:stroke({{292, y}, {302, y+0.4}, {313, y}}, {pressure={0.55,0.3}, ramps={0.1,0.3}})
end
-- restate the stakes
local s = brush("flat", 4.5)
s:reload(stakeP, 0.9)
s:stroke({{118,578},{119.5,545},{121,512}}, {pressure={0.9,0.8}, ramps={0.02,0.05}, orient="across"})
s:reload(stakeP, 0.9)
s:stroke({{168,584},{167,560},{166,536}}, {pressure={0.85,0.75}, ramps={0.02,0.05}, orient="across"})
s:reload(stakeP, 0.8)
s:stroke({{205,590},{206,575},{207,560}}, {pressure={0.75,0.7}, ramps={0.02,0.05}, orient="across"})
-- a touch of the glow on the stake tops
local g = brush{kind="round", width=2, point=0.6}
g:reload(glowThin, 0.4)
g:stroke({{122,513},{122.5,530}}, {pressure={0.3,0.1}})
g:reload(glowThin, 0.4)
g:stroke({{167.5,537},{167.8,550}}, {pressure={0.3,0.1}})

--@ chunk 69
birdP = pile{{"raw umber",1},{"bone black",0.4},{"smalt",0.3},{"lead white",0.3}}
local r = brush{kind="rigger", width=1.5, point=1}
local birds = {{468,300,7},{497,286,6},{522,311,5},{548,296,4}}
for i, bd in ipairs(birds) do
  local x, y, s = bd[1], bd[2], bd[3]
  local tilt = rand(-0.15, 0.15)
  r:reload(birdP, 0.5)
  r:stroke({{x - s, y - s*0.25 + tilt*s}, {x - s*0.45, y - s*0.45}, {x, y}}, {pressure={0.1, 0.5}, ramps={0.3,0.1}})
  r:reload(birdP, 0.5)
  r:stroke({{x, y}, {x + s*0.45, y - s*0.5}, {x + s, y - s*0.2 - tilt*s}}, {pressure={0.5, 0.1}, ramps={0.1,0.3}})
end
-- two poplars breaking the top of the far clump
local pop1 = outline{{741,431},{743,412},{745,398},{747,412},{750,431}, closed=true, char="soft", amount=0.5, seed=161}:mask()
local pop2 = outline{{896,420},{899,396},{902,384},{905,398},{908,420}, closed=true, char="soft", amount=0.5, seed=162}:mask()
work(pop1 + pop2, {hand="detail", pile=treeD, angle=-1.5, coverage=3, fill=true, clip=true})

--@ chunk 70
work(ellipse(795,425,6,6) + ellipse(872,421,6,6), {hand="detail", pile=treeD, coverage=3, fill=true, clip=true})
cornerGlaze = pile{{"raw umber",1},{"smalt",1},{"Prussian blue",0.08},{"red earth",0.15}, medium=0.72}
local corner = mask(function(x,y) return clamp(((y-600)/60) + ((x-600)/400) - 0.3, 0, 1) end) - spit3:grow(3)
work(corner, {hand="glaze", pile=cornerGlaze, angle=0, coverage=1.0})

--@ chunk 71
blend(rect(430,560,580,120) - spit3:grow(3), {angle=0}); blend(rect(430,560,580,120) - spit3:grow(3), {angle=0.03})

--@ chunk 72
topGlaze = pile{{"smalt",1},{"raw umber",0.5},{"red earth",0.15},{"lead white",0.2}, medium=0.75}
local topm = mask(function(x,y) return clamp((90 - y)/50, 0, 1) end)
work(topm, {hand="glaze", pile=topGlaze, angle=0.02, coverage=1.0})
blend(rect(0,0,1000,120), {angle=0.02})
blend(rect(0,0,1000,130), {angle=-0.02})

--@ chunk 73
blend(rect(0,0,1000,125), {angle=0.0, pressure={0.2,0.35}})

--@ chunk 74
print(wait(60)); print(drying(340,300), drying(385,350))
rimP = pile{{"lead white",1},{"chrome yellow",0.35},{"yellow ochre",0.3},{"red earth",0.25}}
local r = brush{kind="rigger", width=2, point=1}
r:reload(rimP, 0.5)
r:stroke({{372.5,190},{376,240},{380,300},{385,370},{388.5,432}}, {pressure={0.45,0.25}, ramps={0.05,0.3}, clip=mainsail:mask():shrink(0.3)})
r:reload(rimP, 0.4)
r:stroke({{306,153},{322,165},{340,178},{362,191}}, {pressure={0.4,0.2}, ramps={0.05,0.4}, clip=topsail:mask():shrink(0.3)})

--@ chunk 75
rimP2 = pile{{"red earth",0.6},{"vermilion",0.3},{"chrome yellow",0.6},{"lead white",0.5}}
local mm = mainsail:mask()
local rs = mask(function(x,y) return (x > 360 and y > 188) and 1 or 0 end)
local band = (mm - mm:shrink(2.2)) * rs
work(band, {hand="detail", tool={kind="round", width=1.6, point=0.6}, pile=rimP2, angle=-1.5, coverage=1.5, pressure={0.3,0.5}, clip=true})
local tm = topsail:mask()
local ts = mask(function(x,y) return (y < 0.42*(x-303)+151 + 2) and 1 or 0 end)
work((tm - tm:shrink(2)) * ts, {hand="detail", tool={kind="round", width=1.4, point=0.6}, pile=rimP2, angle=0.6, coverage=1.5, pressure={0.3,0.5}, clip=true})

--@ chunk 76
local mm = mainsail:mask()
local foot = (mm - mm:offset(0) ) 
local fb = mm * rect(355,430,40,14) - mm:shrink(0):offset(-3)
work(mm * rect(355,432,36,12), {hand="detail", tool={kind="round", width=1.6, point=0.5}, pile=sailDeep, angle=0, coverage=1.8, pressure={0.35,0.55}, clip=true})

--@ chunk 77
local b = brush{kind="flat", width=5}
b:reload(sailDeep, 0.8)
b:stroke({{350,434.5},{370,435},{392,435.5}}, {pressure={0.7,0.6}, ramps={0.05,0.1}, orient="across", clip=mainsail:mask()})
b:reload(sailDeep, 0.6)
b:stroke({{392,437},{370,436.5},{348,436}}, {pressure={0.6,0.5}, ramps={0.05,0.1}, orient="across", clip=mainsail:mask()})

--@ chunk 78
print(drying(60,600), drying(150,625), drying(340,300), drying(700,550))

--@ chunk 79
print(wait(3*24*60)); print(drying(60,600), drying(150,625), drying(52,641))
pudCool = pile{{"smalt",1},{"red earth",0.3},{"raw umber",0.35},{"lead white",0.35}, medium=0.6}
pz1 = outline{{40,606},{75,603},{112,605},{80,609}, closed=true, char="soft", amount=0.4, seed=121}:mask()
work(pz1:grow(0.5), {hand="detail", tool={kind="round", width=2.5}, pile=pudCool, angle=0.03, coverage=1.5, pressure={0.3,0.5}, fill=true, clip=true})

--@ chunk 80
pudCool2 = pile{{"smalt",1},{"red earth",0.3},{"raw umber",0.3},{"lead white",0.5},{"yellow ochre",0.15}, medium=0.6}
pz2 = outline{{130,626},{175,621},{215,625},{170,630}, closed=true, char="soft", amount=0.4, seed=122}:mask()
work(pz2:grow(0.5), {hand="detail", tool={kind="round", width=2.5}, pile=pudCool2, angle=0.03, coverage=1.5, pressure={0.3,0.5}, fill=true, clip=true})

--@ chunk 81
local g = pile{{"lead white",1},{"chrome yellow",0.3},{"yellow ochre",0.3},{"red earth",0.08}}
local r = brush{kind="round", width=1.6, point=0.7}
r:reload(g, 0.35)
r:stroke({{140,624.5},{165,622.8},{195,623.5}}, {pressure={0.35,0.15}, ramps={0.2,0.4}, clip=pz2})
r:reload(g, 0.3)
r:stroke({{55,605},{78,603.8},{100,604.5}}, {pressure={0.3,0.12}, ramps={0.2,0.4}, clip=pz1})
