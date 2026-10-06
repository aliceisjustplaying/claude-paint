-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=490, aspect=1.4, linen={16, 13}, seed=1818,
 ground={{pile={{"red earth", 2}, {"yellow ochre", 3}, {"lead white", 2}}, um=90, apply="knife", texture=0.35},
         {pile={{"lead white", 10}, {"yellow ochre", 0.4}, {"raw umber", 0.1}}, um=45, apply="brush"}}}
print(W, H)

--@ chunk 2
-- geometry, kept as globals
HZ = 438
land_top = {{0,436},{60,433},{120,430},{180,432},{230,428},{300,431},{520,433},{600,430},{680,434},{760,431},{840,434},{920,432},{1000,435}}
rise_top = {{0,592},{120,586},{260,590},{400,598},{520,604},{640,606},{780,603},{900,598},{1000,596}}
river_c = {{-10,560},{80,548},{170,530},{230,512},{200,496},{240,482},{330,472},{420,466},{470,458},{520,452},{560,447}}
h = pencil("2H")
h:sketch(land_top, {pressure=0.25})
h:sketch(rise_top, {pressure=0.25})
h:sketch(river_c, {pressure=0.2})
-- town: churches
h:line({{330,432},{330,378},{322,378},{322,432}}, {pressure=0.3, smooth=false})
h:line({{321,378},{326,318},{331,378}}, {pressure=0.3, smooth=false})
h:line({{420,432},{420,372},{446,372},{446,432}}, {pressure=0.3, smooth=false})
h:line({{418,372},{433,352},{448,372}}, {pressure=0.3, smooth=false})
h:line({{262,432},{262,390},{272,390},{272,432}}, {pressure=0.3, smooth=false})
h:line({{261,390},{267,352},{273,390}}, {pressure=0.3, smooth=false})
-- nave roofs
h:line({{330,405},{345,396},{420,396}}, {pressure=0.25, smooth=false})
h:line({{446,405},{500,405},{510,418}}, {pressure=0.25, smooth=false})
-- windmill
h:line({{728,433},{731,405},{743,405},{746,433}}, {pressure=0.3, smooth=false})
h:line({{737,398},{712,370}}, {pressure=0.25}); h:line({{737,398},{765,374}}, {pressure=0.25})
h:line({{737,398},{710,425}}, {pressure=0.25}); h:line({{737,398},{762,424}}, {pressure=0.25})
-- figures
h:sketch({{604,655},{606,620},{612,600},{616,596},{620,600},{624,622},{626,655}}, {pressure=0.3})
h:sketch({{628,655},{627,628},{632,612},{636,606},{640,612},{644,632},{648,655}}, {pressure=0.3})
-- elder bush at right
h:sketch({{870,714},{860,640},{880,560},{920,500},{970,470},{1000,468}}, {pressure=0.25})
-- fence
for i, x in ipairs({70,118,168,215}) do h:line({{x,640+i*4},{x+1,600+i*4}}, {pressure=0.3}) end

--@ chunk 3
sky_m = above({{0,446},{250,442},{500,444},{750,443},{1000,446}})
sky1 = pile{{"lead white", 2}, {"cobalt blue", 1.1}, {"smalt", 1.0}, medium=0.08}
sky2 = pile{{"lead white", 3}, {"cobalt blue", 0.9}, {"smalt", 0.7}, medium=0.08}
sky3 = pile{{"lead white", 5}, {"cobalt blue", 0.7}, {"smalt", 0.5}, medium=0.08}
sky4 = pile{{"lead white", 8}, {"cobalt blue", 0.4}, {"pale smalt", 0.5}, {"yellow ochre", 0.1}, medium=0.08}
sky5 = pile{{"lead white", 10}, {"yellow ochre", 0.35}, {"vermilion", 0.06}, {"pale smalt", 0.2}, medium=0.08}
sky6 = pile{{"lead white", 10}, {"yellow ochre", 0.45}, {"chrome yellow", 0.15}, {"vermilion", 0.05}, medium=0.08}
local bands = {{sky1,-10,120},{sky2,95,210},{sky3,185,295},{sky4,270,355},{sky5,330,405},{sky6,385,460}}
for i, b in ipairs(bands) do
  local m = rect(-10, b[2], 1020, b[3]-b[2]):soften(12) * sky_m
  work(m, {hand="broad", pile=b[1], angle=function(x,y) return 0.04*math.sin(x/170+i) end, coverage=2.2, length={90,200}, pressure={0.5,0.8}, fill=true, edge="soft"})
end
print(wait(0))

--@ chunk 4
sky0 = pile{{"lead white", 1.5}, {"cobalt blue", 1.2}, {"smalt", 1.2}, medium=0.08}
work(rect(-10,-10,1020,75):soften(15), {hand="broad", pile=sky0, angle=0.02, coverage=1.6, length={120,220}, pressure={0.4,0.7}, edge="soft"})
blend(sky_m, {angle=0, coverage=1.6, clip=true})
blend(sky_m, {angle=0.03, coverage=1.0, clip=true})
print(wait(0), drying(500,100))

--@ chunk 5
cloudw = pile{{"lead white", 10}, {"yellow ochre", 0.15}, {"vermilion", 0.04}, medium=0.1}
cloudg = pile{{"lead white", 6}, {"pale smalt", 1.2}, {"red earth", 0.25}, {"raw umber", 0.15}, medium=0.1}
local streaks = {{250,212,210,11,1},{660,158,270,10,2},{830,262,170,9,3},{140,118,150,8,4},{480,285,160,7,5},{900,110,120,7,6}}
for _, s in ipairs(streaks) do
  local m = ellipse(s[1], s[2], s[3], s[4]):roughen(6, 60, s[5]):soften(4)
  work(m, {hand="scumble", pile=cloudw, angle=0.0, coverage=1.3, length={20,50}, pressure={0.3,0.55}, load=0.5, edge="lost"})
end
-- low grey band of cloud over the horizon at right and centre
low_cloud = poly({{430,392},{470,380},{520,376},{560,370},{610,372},{650,364},{700,368},{760,360},{820,366},{880,358},{940,364},{1010,362},{1010,400},{900,404},{760,400},{600,402},{470,398}}, true):roughen(5, 40, 7)
work(low_cloud, {hand="scumble", pile=cloudg, angle=0, coverage=1.2, length={25,60}, pressure={0.3,0.5}, load=0.45, edge="soft"})
print(wait(0))

--@ chunk 6
local cz = rect(0,80,1000,230):soften(20) * sky_m
blend(cz, {angle=0.0, coverage=2.2, clip=true})
blend(cz, {angle=0.25, coverage=1.2, clip=true})
blend(cz, {angle=-0.05, coverage=1.2, clip=true})
local lz = rect(400,340,620,80):soften(20) * sky_m
blend(lz, {angle=0.0, coverage=2.0, clip=true})
blend(lz, {angle=0.2, coverage=1.0, clip=true})
print(wait(0), drying(500,150))

--@ chunk 7
land_m = -sky_m
gFar  = pile{{"lead white", 5}, {"green earth", 3}, {"pale smalt", 1.5}, {"yellow ochre", 1}, medium=0.08}
gMidL = pile{{"lead white", 3}, {"yellow ochre", 3}, {"chrome yellow", 0.5}, {"Prussian blue", 0.2}, {"green earth", 2}, medium=0.08}
gShad = pile{{"yellow ochre", 2}, {"Prussian blue", 0.35}, {"raw umber", 1}, {"green earth", 2}, {"lead white", 1.2}, medium=0.08}
gLit2 = pile{{"lead white", 2}, {"yellow ochre", 3}, {"chrome yellow", 1}, {"Prussian blue", 0.3}, {"green earth", 1}, medium=0.08}
gFore = pile{{"yellow ochre", 2}, {"raw umber", 1.5}, {"Prussian blue", 0.4}, {"green earth", 2}, {"lead white", 0.6}, {"red earth", 0.2}, medium=0.08}
local function wav(y0, a, p, ph) local pts = {} for x=-20,1020,40 do pts[#pts+1] = {x, y0 + a*math.sin(x/p+ph) + 0.4*a*math.sin(x/(p*0.37)+2*ph)} end return pts end
b_far  = wav(474, 4, 90, 1)
b_mid  = wav(516, 7, 120, 2)
b_shad = wav(546, 8, 140, 3)
Zfar  = below(land_top) - below(b_far)
Zmid  = below(b_far) - below(b_mid)
Zsh   = below(b_mid) - below(b_shad)
Zlit  = below(b_shad) - below(rise_top)
Zfore = below(rise_top)
local zs = {{Zfar, gFar, {20,60}}, {Zmid, gMidL, {25,70}}, {Zsh, gShad, {25,70}}, {Zlit, gLit2, {30,80}}, {Zfore, gFore, {30,80}}}
for _, z in ipairs(zs) do
  work(z[1] * land_m, {hand="body", pile=z[2], angle=function(x,y) return 0.03*math.sin(x/200) end, coverage=2.0, length=z[3], pressure={0.5,0.85}, fill=true, clip=land_m, edge="soft"})
end
print(wait(0))

--@ chunk 8
local seams = (ribbon(b_far, 10) + ribbon(b_mid, 18) + ribbon(b_shad, 20) + ribbon(rise_top, 18)):soften(6) * land_m
blend(seams, {angle=0.9, coverage=1.5, clip=true, tool={kind="badger", width=16}})
blend(seams, {angle=-0.8, coverage=1.0, clip=true, tool={kind="badger", width=16}})
blend(land_m, {angle=0, coverage=0.8, clip=true, pressure={0.2,0.35}})
print(wait(0))

--@ chunk 9
print(wait(22*60)); print(drying(500,100), drying(500,380), drying(500,500), drying(500,650))

--@ chunk 10
print(wait(24*60)); print(drying(500,100), drying(500,380), drying(500,500), drying(500,650))

--@ chunk 11
farland = pile{{"lead white", 5}, {"pale smalt", 2}, {"green earth", 1.5}, {"raw umber", 0.35}, {"yellow ochre", 0.3}, medium=0.1}
-- distant land strip with low tree clumps along its top
local pts = {{-10,452}}
local x = -10
local s = 0
while x < 1010 do
  local base = 436 + 2*math.sin(x/130) 
  local bump = 0
  local n = noise{seed=11, period=40, octaves=3}
  bump = math.max(0, n(x, 0)) * 9
  if x > 540 and x < 700 then bump = bump + 4 end
  if x > 780 and x < 900 then bump = bump + 6*math.max(0, math.sin((x-780)/40)) end
  pts[#pts+1] = {x, base - bump}
  x = x + 5
end
pts[#pts+1] = {1010,452}
farstrip = poly(pts):roughen(1.2, 8, 3)
work(farstrip, {hand="detail", pile=farland, angle=0, coverage=2.2, length={6,16}, pressure={0.5,0.8}, fill=true, edge="firm"})
print(wait(0))

--@ chunk 12
townp = pile{{"lead white", 4}, {"pale smalt", 2}, {"smalt", 0.5}, {"red earth", 0.3}, {"raw umber", 0.45}, medium=0.1}
-- houses and roofs, a jagged row of gables
local hp = {{228,440},{228,426},{234,420},{240,426},{246,426},{246,422},{252,416},{258,422},{262,422}, {262,390},{272,390},{272,420},{280,420},{286,414},{292,420},{300,420},{300,418},{306,412},{312,418},{322,418},
  {322,378},{331,378},{331,410},{345,396},{420,396},{420,372},{446,372},{446,402},{452,398},{500,398},{508,410},{516,410},{522,404},{528,410},{540,412},{546,407},{552,412},{566,414},{575,418},{582,422},{590,440}}
town = poly(hp)
-- spires and caps
spJ = poly({{261.5,391},{267,351},{272.5,391}})
spN1 = poly({{323.5,379},{323.5,366},{329.5,366},{329.5,379}})
spN2 = poly({{323,367},{326.5,316},{330,367}})
capM = poly({{418,373},{433,356},{448,373}})
town_all = town + spJ + spN1 + spN2 + capM
work(town_all, {hand="detail", pile=townp, angle=math.pi/2, coverage=2.5, length={4,10}, pressure={0.5,0.8}, fill=true, edge="found", clip=true})
-- windmill
mill_body = poly({{729,436},{731.5,406},{742.5,406},{745,436}})
mill_cap = poly({{728,407},{730,401},{737,398},{744,401},{746,407}})
work(mill_body + mill_cap, {hand="detail", pile=townp, angle=math.pi/2, coverage=2.5, length={3,8}, fill=true, edge="found", clip=true})
print(wait(0))

--@ chunk 13
millp = pile{{"lead white", 3}, {"pale smalt", 2}, {"smalt", 0.6}, {"raw umber", 0.7}, {"red earth", 0.2}, medium=0.15}
local r = brush{kind="round", width=1.4, point=0.9}
r:load(millp, 0.6)
local hx, hy = 737, 400
for i, a in ipairs({-0.75, 0.82, 2.39, -2.33}) do
  local L = 27
  local ex, ey = hx + L*math.cos(a), hy + L*math.sin(a)
  r:stroke({{hx,hy},{ex,ey}}, {pressure={0.55,0.35}, ramps={0.05,0.1}})
  -- the sail frame, offset a little to one side of the stock
  local nx, ny = -math.sin(a)*2.2, math.cos(a)*2.2
  r:stroke({{hx+nx+ 6*math.cos(a), hy+ny+6*math.sin(a)},{ex+nx,ey+ny}}, {pressure={0.3,0.25}, ramps={0.1,0.1}})
  if i % 2 == 0 then r:load(millp, 0.5) end
end
print(wait(0))

--@ chunk 14
mL = pile{{"yellow ochre", 3}, {"lead white", 2.5}, {"green earth", 2}, {"Prussian blue", 0.12}, {"chrome yellow", 0.2}, {"raw umber", 0.2}, medium=0.1}
mS = pile{{"green earth", 2}, {"yellow ochre", 1.5}, {"raw umber", 1}, {"Prussian blue", 0.22}, {"lead white", 1}, {"smalt", 0.3}, medium=0.1}
meadow = below({{-10,448},{1010,448}}) - Zfore
local n = noise{seed=21, period=1, octaves=3, persistence=0.5}
shadow_fn = function(x, y)
  if y < 446 then return 0 end
  local d = y - 430
  local u = x / (60 * d / 60) 
  local v = 900 / d
  local s = n(u * 2.2 / 3.0, v * 0.9)
  return smoothstep(0.0, 0.18, s)
end
cshadow = mask(shadow_fn) * meadow
work(meadow, {hand="body", pile=mL, angle=function(x,y) return 0.02*math.sin(x/150) end, coverage=1.6, length={20,55}, pressure={0.45,0.75}, fill=true, clip=meadow, edge="soft"})
work(cshadow, {hand="body", pile=mS, angle=0, coverage=1.6, length={15,45}, pressure={0.45,0.75}, fill=true, clip=meadow, edge="soft"})
print(cshadow:area()/meadow:area(), wait(0))

--@ chunk 15
blend(meadow, {angle=0, coverage=2.0, clip=true})
blend(meadow * below({{0,445},{1000,445}}) - below({{0,520},{1000,520}}), {angle=0.02, coverage=1.5, clip=true, tool={kind="badger", width=24}})
print(wait(0))

--@ chunk 16
print(wait(240)); print(drying(430,400), drying(500,470), drying(500,560))

--@ chunk 17
print(wait(16*60)); print(drying(430,400), drying(500,470), drying(500,560), drying(300,650))

--@ chunk 18
townD = pile{{"lead white", 3}, {"smalt", 1.5}, {"pale smalt", 1}, {"raw umber", 0.6}, {"bone black", 0.08}, medium=0.12}
townT = pile{{"lead white", 3}, {"pale smalt", 1.5}, {"green earth", 1}, {"raw umber", 0.6}, {"smalt", 0.5}, medium=0.12}
-- shaded faces of towers and spires
local sh = poly({{267,391},{267,352},{272.5,391}}) + rect(267.5,390,4.5,42)
        + poly({{326.5,367},{326.5,317},{330,367}}) + rect(326.8,366,2.7,13) + rect(327,378,4,54)
        + poly({{433,356},{448,373},{433,373}}) + rect(434,372,12,60)
        + poly({{433,356},{448,373},{433,373}})
work(sh * town_all, {hand="detail", pile=townD, angle=math.pi/2, coverage=2.2, length={3,9}, fill=true, clip=true, edge="found"})
-- lower band of the town in the shadow of its own walls and trees
work(rect(226,424,370,14) * town_all, {hand="detail", pile=townD, angle=0, coverage=1.5, length={4,12}, fill=true, clip=true, edge="found"})
-- trees among the houses: small rounded clumps
local clumps = poly({{225,440},{226,431},{230,426},{236,425},{241,428},{244,433},{246,440}}, true)
  + poly({{560,440},{562,428},{568,422},{576,421},{583,425},{588,431},{592,440}}, true)
  + poly({{596,440},{598,433},{604,430},{611,432},{614,440}}, true)
  + poly({{300,440},{302,430},{308,427},{314,430},{316,440}}, true)
  + poly({{690,440},{692,431},{699,427},{707,429},{711,436},{712,440}}, true)
clumps = clumps:roughen(1.2, 5, 9)
stipple(clumps, {pile=townT, width=2.2, coverage=5, pressure={0.4,0.7}, clip=clumps})
print(wait(0))

--@ chunk 19
townL = pile{{"lead white", 6}, {"pale smalt", 1}, {"yellow ochre", 0.35}, {"red earth", 0.12}, medium=0.12}
townR = pile{{"lead white", 4}, {"pale smalt", 1.6}, {"red earth", 0.35}, {"raw umber", 0.35}, medium=0.12}
-- roofs of the naves: greyed a little
local roofs = poly({{331,410},{345,396},{420,396},{420,424},{331,424}}) + poly({{446,402},{452,398},{500,398},{508,410},{508,424},{446,424}})
work(roofs * town_all, {hand="detail", pile=townR, angle=0.0, coverage=1.2, length={5,14}, load=0.5, clip=true, edge="found"})
-- a row of small lit gable-fronted houses breaking the dark base line
houses = {}
local xs = {236, 247, 279, 291, 305, 343, 356, 372, 389, 404, 458, 471, 486, 512, 527, 542}
local m = nil
for i, x in ipairs(xs) do
  local w = 6 + (i % 3) * 1.5
  local top = 424 - (i % 4) * 1.5
  local hp = poly({{x, 438}, {x, top}, {x + w/2, top - w*0.55}, {x + w, top}, {x + w, 438}})
  m = m and (m + hp) or hp
end
houses = m
work(houses, {hand="detail", pile=townL, angle=math.pi/2, coverage=2.0, length={3,7}, fill=true, clip=true, edge="found"})
print(wait(0))

--@ chunk 20
townM = pile{{"lead white", 4}, {"pale smalt", 1.6}, {"raw umber", 0.4}, {"red earth", 0.15}, {"smalt", 0.35}, medium=0.1}
local base = (town_all * rect(222,412,380,30)) + houses:grow(0.6)
-- ragged roofline for the lower town: small gables drawn as one outline
local pts = {{226,441}}
local x = 226
local k = 0
while x < 590 do
  k = k + 1
  local w = 5 + (k*37 % 7)
  local top = 425 - (k*13 % 5)
  pts[#pts+1] = {x, top}
  pts[#pts+1] = {x + w*0.5, top - 3 - (k % 3)}
  x = x + w
  pts[#pts+1] = {x, top}
end
pts[#pts+1] = {592,441}
lowtown = poly(pts) + base
lowtown = lowtown - (spJ + rect(262,390,10,30) + spN1 + rect(322,378,9,40) + rect(420,372,26,50))
work(lowtown, {hand="detail", pile=townM, angle=0, coverage=2.5, length={3,9}, fill=true, clip=true, edge="found"})
print(wait(0))

--@ chunk 21
haze = pile{{"lead white", 6}, {"green earth", 2.2}, {"pale smalt", 1.5}, {"yellow ochre", 1.2}, {"raw umber", 0.15}, medium=0.12}
local below440 = below({{-10,440.5},{1010,440.5}})
local hz = mask(function(x,y) if y < 440 then return 0 end return 1 - smoothstep(452, 485, y) end)
work(hz, {hand="scumble", pile=haze, angle=0, coverage=1.6, length={15,40}, pressure={0.3,0.55}, load=0.45, clip=below440, threshold=0.25})
print(wait(0))

--@ chunk 22
gHz2 = pile{{"green earth", 2.2}, {"yellow ochre", 2}, {"lead white", 2}, {"pale smalt", 0.8}, {"raw umber", 0.3}, medium=0.1}
local below440 = below({{-10,440.5},{1010,440.5}})
local zone = mask(function(x,y) if y < 446 then return 0 end return smoothstep(446, 462, y) * (1 - smoothstep(484, 496, y)) end)
work(zone, {hand="body", pile=gHz2, angle=0, coverage=2.0, length={20,60}, pressure={0.4,0.7}, fill=true, clip=below440, threshold=0.3, edge="soft"})
local bz = below440 * mask(function(x,y) return 1 - smoothstep(492, 505, y) end)
blend(bz, {angle=0, coverage=2.0, clip=true, tool={kind="badger", width=20}})
blend(bz, {angle=0.35, coverage=1.0, clip=true, tool={kind="badger", width=14}, pressure={0.25,0.4}})
blend(bz, {angle=0, coverage=1.0, clip=true, tool={kind="badger", width=30}, pressure={0.2,0.35}})
print(wait(0))

--@ chunk 23
print(wait(2*24*60)); print(drying(500,460), drying(500,520), drying(500,560), drying(300,650))

--@ chunk 24
mid_m = below({{-10,468},{1010,468}}) - below(rise_top)
mA = pile{{"green earth", 2.2}, {"yellow ochre", 2}, {"lead white", 1.8}, {"pale smalt", 0.6}, {"raw umber", 0.3}, medium=0.1}
mB = pile{{"yellow ochre", 3}, {"green earth", 2}, {"lead white", 1.6}, {"chrome yellow", 0.3}, {"Prussian blue", 0.1}, {"raw umber", 0.2}, medium=0.1}
mC = pile{{"green earth", 2.5}, {"yellow ochre", 1.6}, {"raw umber", 1.0}, {"Prussian blue", 0.18}, {"lead white", 0.8}, medium=0.1}
local zA = mask(function(x,y) return 1 - smoothstep(488, 508, y) end) * mid_m
local zB = mask(function(x,y) return smoothstep(488, 508, y) end) * mid_m
work(zA, {hand="body", pile=mA, angle=0, coverage=1.8, length={25,60}, pressure={0.45,0.75}, fill=true, clip=mid_m, edge="soft", threshold=0.2})
work(zB, {hand="body", pile=mB, angle=0, coverage=1.8, length={25,70}, pressure={0.45,0.75}, fill=true, clip=mid_m, edge="soft", threshold=0.2})
-- deliberate cloud shadows: long flat shapes lying on the meadow
cs1 = poly({{-10,522},{80,516},{200,513},{330,515},{420,520},{470,527},{430,534},{300,537},{150,540},{-10,541}}, true):roughen(3, 30, 31)
cs2 = poly({{560,507},{650,502},{780,500},{900,503},{1010,505},{1010,520},{900,522},{760,519},{640,517}}, true):roughen(2.5, 30, 32)
cs3 = poly({{380,568},{520,560},{700,558},{860,563},{1010,560},{1010,600},{380,600}}, true):roughen(3, 40, 33)
cs4 = poly({{120,487},{220,484},{330,485},{360,490},{250,493},{140,494}}, true):roughen(1.5, 20, 34)
csh = (cs1 + cs2 + cs3 + cs4) * mid_m
work(csh, {hand="body", pile=mC, angle=0, coverage=1.8, length={20,60}, pressure={0.45,0.75}, fill=true, clip=mid_m, edge="soft"})
blend(mid_m, {angle=0, coverage=1.8, clip=true, tool={kind="badger", width=28}})
blend(mid_m, {angle=0.12, coverage=1.0, clip=true, tool={kind="badger", width=20}, pressure={0.25,0.4}})
print(wait(0))

--@ chunk 25
mAB = pile{{"green earth", 2.2}, {"yellow ochre", 2.1}, {"lead white", 2.0}, {"pale smalt", 0.7}, {"raw umber", 0.25}, medium=0.1}
local band = mask(function(x,y) return math.max(0, 1 - math.abs(y - 468)/11) end) * below({{-10,441},{1010,441}})
work(band, {hand="scumble", pile=mAB, angle=0, coverage=1.6, length={15,40}, pressure={0.3,0.5}, load=0.4, threshold=0.25, clip=below({{-10,441},{1010,441}})})
blend(band:grow(3), {angle=0, coverage=1.5, clip=true, tool={kind="badger", width=12}, pressure={0.25,0.4}})
print(wait(0))

--@ chunk 26
print(wait(26*60)); print(drying(500,460), drying(500,520), drying(500,580))

--@ chunk 27
print(wait(24*60)); print(drying(500,460), drying(500,520), drying(500,580))

--@ chunk 28
shG = pile{{"green earth", 3}, {"raw umber", 0.8}, {"Prussian blue", 0.25}, {"yellow ochre", 0.8}, {"lead white", 0.3}, medium=0.3}
local s1 = poly({{-10,524},{90,519},{210,516},{330,518},{430,523},{480,529},{420,535},{300,538},{150,541},{-10,543}}, true):roughen(2.5, 25, 41)
local s2 = poly({{590,506},{680,502},{800,501},{910,503},{1010,505},{1010,517},{910,519},{770,518},{650,514}}, true):roughen(2, 25, 42)
local s3 = poly({{430,570},{560,563},{720,561},{870,565},{1010,562},{1010,600},{430,600}}, true):roughen(3, 35, 43)
local s4 = poly({{600,538},{700,535},{780,537},{740,543},{640,544}}, true):roughen(1.5, 18, 44)
shad2 = (s1 + s2 + s3 + s4) * mid_m
work(shad2, {hand="body", pile=shG, angle=0, coverage=1.6, length={20,55}, pressure={0.4,0.65}, fill=true, clip=shad2:grow(1.5), edge="soft", load=0.6})
blend(shad2:grow(3), {angle=0, coverage=1.4, clip=true, tool={kind="badger", width=12}, pressure={0.2,0.35}})
print(wait(0))

--@ chunk 29
local z = shad2:grow(10):soften(6) * mid_m
blend(z, {angle=0, coverage=2.5, clip=true, tool={kind="badger", width=24}, pressure={0.45,0.65}})
blend(z, {angle=0.05, coverage=2.0, clip=true, tool={kind="badger", width=30}, pressure={0.4,0.6}})
blend(z, {angle=-0.05, coverage=1.5, clip=true, tool={kind="badger", width=18}, pressure={0.3,0.5}})
print(wait(0))

--@ chunk 30
print(wait(2*24*60)); print(drying(300,530), drying(700,580), drying(500,460))

--@ chunk 31
mL2 = pile{{"yellow ochre", 3}, {"green earth", 2}, {"lead white", 2.2}, {"chrome yellow", 0.25}, {"Prussian blue", 0.06}, {"raw umber", 0.25}, medium=0.1}
local A = poly({{-10,521},{120,519},{280,520},{380,523},{450,528},{380,531},{260,534},{100,536},{-10,537}}, true)
local B = poly({{640,506},{740,503},{860,503},{1010,504},{1010,514},{880,513},{760,512},{680,510}}, true)
local C = poly({{480,582},{600,574},{760,571},{900,572},{1010,570},{1010,600},{480,600}}, true)
keepSh = (A + B + C):roughen(2, 25, 51) * mid_m
local lights = mid_m * mask(function(x,y) return y > 470 and 1 or 0 end) - keepSh
work(lights, {hand="body", pile=mL2, angle=0, coverage=1.7, length={20,55}, pressure={0.4,0.7}, fill=true, clip=mid_m, edge="soft", threshold=0.4})
blend(lights, {angle=0, coverage=1.2, clip=true, tool={kind="badger", width=16}, pressure={0.25,0.4}})
print(wait(0))

--@ chunk 32
fore_m = below(rise_top)
fC = pile{{"yellow ochre", 2.5}, {"green earth", 2}, {"raw umber", 0.7}, {"Prussian blue", 0.12}, {"lead white", 1.0}, {"chrome yellow", 0.15}, medium=0.08}
fM = pile{{"green earth", 2.5}, {"yellow ochre", 1.8}, {"raw umber", 1.2}, {"Prussian blue", 0.2}, {"lead white", 0.5}, medium=0.08}
fD = pile{{"green earth", 2}, {"raw umber", 2}, {"yellow ochre", 1}, {"Prussian blue", 0.3}, {"bone black", 0.15}, {"lead white", 0.2}, medium=0.08}
local function crest_d(x, y) -- depth below the crest
  local t = 0
  for i = 1, #rise_top - 1 do local a, b = rise_top[i], rise_top[i+1]
    if x >= a[1] and x <= b[1] then t = a[2] + (b[2]-a[2]) * (x - a[1]) / (b[2]-a[1]) break end end
  return y - t
end
crest_depth = crest_d
local zC = mask(function(x,y) local d = crest_d(x,y) return d < 0 and 0 or (1 - smoothstep(18, 40, d)) end)
local zM = mask(function(x,y) local d = crest_d(x,y) return d < 0 and 0 or smoothstep(18, 40, d) * (1 - smoothstep(60, 90, d)) end)
local zD = mask(function(x,y) local d = crest_d(x,y) return d < 0 and 0 or smoothstep(60, 90, d) end)
work(zC, {hand="body", pile=fC, angle=0.03, coverage=2.0, length={20,50}, pressure={0.5,0.8}, fill=true, clip=fore_m, edge="found", threshold=0.3})
work(zM, {hand="body", pile=fM, angle=0.05, coverage=2.0, length={25,60}, pressure={0.5,0.8}, fill=true, clip=fore_m, threshold=0.3})
work(zD, {hand="body", pile=fD, angle=0.08, coverage=2.0, length={25,70}, pressure={0.5,0.85}, fill=true, clip=fore_m, threshold=0.3})
blend(fore_m, {angle=0.02, coverage=1.3, clip=true, tool={kind="badger", width=26}, pressure={0.3,0.5}})
print(wait(0))

--@ chunk 33
print(wait(2*24*60)); print(drying(300,480), drying(700,560), drying(500,620), drying(500,700))

--@ chunk 34
sC = pile{{"green earth", 3}, {"yellow ochre", 1.6}, {"raw umber", 1.0}, {"Prussian blue", 0.15}, {"lead white", 0.9}, {"smalt", 0.3}, medium=0.08}
sM = pile{{"green earth", 3}, {"yellow ochre", 1.2}, {"raw umber", 1.5}, {"Prussian blue", 0.2}, {"lead white", 0.5}, medium=0.08}
sD = pile{{"green earth", 2}, {"raw umber", 2.2}, {"yellow ochre", 0.8}, {"Prussian blue", 0.3}, {"bone black", 0.25}, {"lead white", 0.15}, medium=0.08}
local zC = mask(function(x,y) local d = crest_depth(x,y) return d < 0 and 0 or (1 - smoothstep(25, 45, d)) end)
local zM = mask(function(x,y) local d = crest_depth(x,y) return d < 0 and 0 or smoothstep(25, 45, d) * (1 - smoothstep(70, 95, d)) end)
local zD = mask(function(x,y) local d = crest_depth(x,y) return d < 0 and 0 or smoothstep(70, 95, d) end)
work(zC, {hand="body", pile=sC, angle=0.03, coverage=2.2, length={20,50}, pressure={0.55,0.85}, fill=true, clip=fore_m, edge="found", threshold=0.3})
work(zM, {hand="body", pile=sM, angle=0.05, coverage=2.2, length={25,60}, pressure={0.55,0.85}, fill=true, clip=fore_m, threshold=0.3})
work(zD, {hand="body", pile=sD, angle=0.08, coverage=2.2, length={25,70}, pressure={0.55,0.9}, fill=true, clip=fore_m, threshold=0.3})
blend(fore_m, {angle=0.02, coverage=1.5, clip=true, tool={kind="badger", width=26}, pressure={0.3,0.5}})
print(wait(0))

--@ chunk 35
riv_pts = {{-12,577},{50,573},{130,566},{205,556},{250,545},{240,533},{190,524},{160,515},{185,506},{260,498},{350,490},{420,482},{465,472},{505,462},{545,454},{575,449},{600,446}}
riv_w = {13,12.5,11.5,10,8.5,7.5,6.5,5.5,4.8,4.2,3.6,3.0,2.5,2.1,1.7,1.4,1.2}
river = ribbon(riv_pts, riv_w)
bankD = pile{{"green earth", 2}, {"raw umber", 1.6}, {"Prussian blue", 0.2}, {"yellow ochre", 0.6}, {"lead white", 0.4}, medium=0.08}
water = pile{{"lead white", 6}, {"pale smalt", 1.4}, {"cobalt blue", 0.35}, {"yellow ochre", 0.12}, medium=0.08}
-- the bank shadow: a slightly larger ribbon offset down, laid first
local bank = ribbon((function() local t = {} for i,p in ipairs(riv_pts) do t[i] = {p[1], p[2] + riv_w[i]*0.28} end return t end)(),
                    (function() local t = {} for i,w in ipairs(riv_w) do t[i] = w*1.25 end return t end)())
work(bank, {hand="detail", pile=bankD, angle=0, coverage=2, length={4,12}, fill=true, clip=true, edge="firm"})
work(river, {hand="detail", pile=water, angle=0, coverage=2.2, length={4,14}, fill=true, clip=true, edge="found"})
print(wait(0))

--@ chunk 36
print(wait(30*60)); print(drying(300,495), drying(100,570), drying(500,620))

--@ chunk 37
print(wait(20*60)); print(drying(300,495), drying(100,570), drying(500,620))

--@ chunk 38
print(wait(48*60)); print(drying(300,495), drying(100,570), drying(500,620), drying(500,700))

--@ chunk 39
local far_riv = river:grow(2.5) * mask(function(x,y) return (y < 513 and x > 170) and 1 or 0 end)
local a = far_riv * mask(function(x,y) return y >= 472 and 1 or 0 end)
local b = far_riv * mask(function(x,y) return y < 472 and 1 or 0 end)
work(a, {hand="detail", pile=mL2, angle=0, coverage=2.5, length={4,12}, fill=true, clip=true, edge="soft"})
work(b, {hand="detail", pile=mAB, angle=0, coverage=2.5, length={4,12}, fill=true, clip=true, edge="soft"})
print(wait(0))

--@ chunk 40
water2 = pile{{"lead white", 7}, {"pale smalt", 1.0}, {"cobalt blue", 0.25}, {"yellow ochre", 0.15}, medium=0.12}
bank2 = pile{{"green earth", 2}, {"raw umber", 1.4}, {"Prussian blue", 0.15}, {"yellow ochre", 0.8}, {"lead white", 0.6}, medium=0.12}
far_pts = {{158,511.5},{200,508},{250,503.5},{310,498},{370,492},{420,486},{455,480},{490,472},{520,465},{548,458},{572,452},{592,448}}
local r = brush{kind="round", width=3.2, point=0.2}
r:load(bank2, 0.5)
-- dark bank line under the water
local bp = {} for i,p in ipairs(far_pts) do bp[i] = {p[1], p[2] + 1.3 - i*0.08} end
r:stroke({table.unpack(bp, 1, 6)}, {pressure={0.5,0.35}, ramps={0.05,0.1}})
r:load(bank2, 0.4)
r:stroke({table.unpack(bp, 6, 12)}, {pressure={0.35,0.2}, ramps={0.05,0.3}})
local g = brush{kind="round", width=2.6, point=0.3}
g:load(water2, 0.55)
g:stroke({table.unpack(far_pts, 1, 5)}, {pressure={0.55,0.4}, ramps={0.02,0.1}})
g:load(water2, 0.45)
g:stroke({table.unpack(far_pts, 5, 9)}, {pressure={0.38,0.3}, ramps={0.05,0.1}})
g:load(water2, 0.4)
g:stroke({table.unpack(far_pts, 9, 12)}, {pressure={0.28,0.15}, ramps={0.05,0.4}})
print(wait(0))

--@ chunk 41
near_pts = {{-14,581},{50,576},{130,568},{200,558},{238,547},{233,535},{195,526},{168,518},{170,511},{198,507.5},{230,505}}
near_w   = {11,10,8.5,7,5.5,4.6,3.8,3.2,2.8,2.5,2.4}
river2 = ribbon(near_pts, near_w)
local old = river:grow(4) * mask(function(x,y) return x < 280 and 1 or 0 end) - fore_m
local cover = old - river2
work(cover, {hand="detail", pile=mL2, angle=0, coverage=2.6, length={4,12}, fill=true, clip=true, edge="soft"})
print(wait(0))

--@ chunk 42
print(wait(36*60)); print(drying(150,560), drying(220,540))

--@ chunk 43
-- near bank (the far, upper edge of the water is lit; the near lower edge a dark cut bank)
local bpts, bw = {}, {}
for i,p in ipairs(near_pts) do bpts[i] = {p[1], p[2] + near_w[i]*0.42}; bw[i] = near_w[i]*0.55 end
local bankm = ribbon(bpts, bw)
work(bankm, {hand="detail", pile=bank2, angle=0, coverage=2.2, length={4,10}, fill=true, clip=true, edge="firm"})
work(river2 - bankm:shrink(0.5), {hand="detail", pile=water2, angle=0, coverage=2.6, length={5,14}, fill=true, clip=true, edge="found"})
print(wait(0))

--@ chunk 44
print(wait(26*60)); print(drying(150,565), drying(220,540))

--@ chunk 45
print(wait(24*60)); print(drying(150,565), drying(220,540), drying(40,578))

--@ chunk 46
print(wait(48*60)); print(drying(150,565), drying(220,540), drying(40,578))

--@ chunk 47
wglaze = pile{{"pale smalt", 2}, {"smalt", 0.6}, {"lead white", 1.2}, {"raw umber", 0.2}, {"green earth", 0.3}, medium=0.55}
local rv = river2:shrink(0.3)
-- the nearer water, darker toward the near side of each reach
work(rv, {hand="detail", pile=wglaze, angle=0, coverage=1.6, length={6,16}, pressure={0.35,0.55}, load=0.45, fill=true, clip=true})
blend(rv, {angle=0, coverage=1.5, clip=true, tool={kind="badger", width=8}, pressure={0.2,0.35}})
print(wait(0))

--@ chunk 48
gG  = pile{{"green earth", 3}, {"yellow ochre", 1.6}, {"lead white", 1.6}, {"Prussian blue", 0.1}, {"chrome yellow", 0.2}, {"raw umber", 0.15}, medium=0.1}
gGf = pile{{"green earth", 3}, {"yellow ochre", 1.4}, {"lead white", 2.4}, {"pale smalt", 0.5}, {"raw umber", 0.15}, medium=0.1}
straw = pile{{"lead white", 4}, {"yellow ochre", 2}, {"green earth", 1}, {"raw umber", 0.2}, {"red earth", 0.05}, medium=0.1}
fieldB = poly({{318,522},{420,519},{560,515},{720,511},{880,508},{1010,507},{1010,574},{820,577},{640,579},{470,582},{430,581}})
local keep = river2:grow(1.5) + fore_m
fieldA = mask(function(x,y) return (y > 466 and y < 532) and 1 or 0 end) * mid_m - fieldB - keep
fieldC = mid_m * mask(function(x,y) return y >= 520 and 1 or 0 end) - fieldB - keep - fieldA
local fa_far = fieldA * mask(function(x,y) return y < 490 and 1 or 0 end)
work(fa_far, {hand="body", pile=gGf, angle=0, coverage=1.8, length={15,45}, pressure={0.4,0.7}, fill=true, clip=fa_far:grow(1), edge="soft"})
work(fieldA - fa_far, {hand="body", pile=gG, angle=0, coverage=1.8, length={15,50}, pressure={0.4,0.7}, fill=true, clip=(fieldA - fa_far):grow(1), edge="soft"})
work(fieldC, {hand="body", pile=gG, angle=0, coverage=1.8, length={15,50}, pressure={0.4,0.7}, fill=true, clip=fieldC:grow(1), edge="soft"})
work(fieldB - keep, {hand="body", pile=straw, angle=-0.02, coverage=1.8, length={20,60}, pressure={0.4,0.7}, fill=true, clip=(fieldB-keep):grow(1), edge="firm"})
local soft = (fieldA + fieldC):grow(2) * mid_m - river2:grow(1)
blend(soft, {angle=0, coverage=1.2, clip=true, tool={kind="badger", width=16}, pressure={0.25,0.4}})
blend(fieldB - keep, {angle=-0.02, coverage=1.0, clip=true, tool={kind="badger", width=16}, pressure={0.25,0.4}})
print(wait(0))

--@ chunk 49
print(wait(3*24*60)); print(drying(600,540), drying(300,500), drying(200,560))

--@ chunk 50
local r = brush{kind="round", width=2.4, point=0.3}
local fp = {{232,505},{270,501.5},{310,498},{370,492},{420,486},{455,480},{490,472},{520,465},{548,458},{572,452},{592,448}}
local bp = {} for i,p in ipairs(fp) do bp[i] = {p[1], p[2] + 1.1 - i*0.06} end
r:load(bank2, 0.45)
r:stroke({table.unpack(bp, 1, 6)}, {pressure={0.45,0.32}, ramps={0.05,0.1}})
r:load(bank2, 0.35)
r:stroke({table.unpack(bp, 6, 11)}, {pressure={0.32,0.18}, ramps={0.05,0.3}})
local g = brush{kind="round", width=2.2, point=0.3}
local wf = pile{{"lead white", 6}, {"pale smalt", 1.3}, {"cobalt blue", 0.3}, {"raw umber", 0.1}, medium=0.12}
g:load(wf, 0.5)
g:stroke({table.unpack(fp, 1, 5)}, {pressure={0.45,0.35}, ramps={0.02,0.1}})
g:load(wf, 0.4)
g:stroke({table.unpack(fp, 5, 8)}, {pressure={0.32,0.25}, ramps={0.05,0.1}})
g:load(wf, 0.35)
g:stroke({table.unpack(fp, 8, 11)}, {pressure={0.24,0.12}, ramps={0.05,0.4}})
-- ditch line along the far edge of the hay field and its left diagonal
local d = brush{kind="round", width=2.0, point=0.4}
d:load(bank2, 0.5)
d:stroke({{322,523},{420,520},{560,516},{720,512},{880,509},{1010,508}}, {pressure={0.4,0.35}, ramps={0.02,0.02}, shake=0.6})
print(wait(0))

--@ chunk 51
wD = pile{{"green earth", 2}, {"raw umber", 1.2}, {"Prussian blue", 0.15}, {"lead white", 0.9}, {"yellow ochre", 0.5}, medium=0.1}
wL = pile{{"lead white", 3}, {"green earth", 2}, {"yellow ochre", 0.8}, {"pale smalt", 0.6}, {"raw umber", 0.2}, medium=0.1}
wT = pile{{"raw umber", 2}, {"bone black", 0.3}, {"lead white", 0.8}, {"pale smalt", 0.5}, medium=0.1}
function willow(x, yb, s, seed)
  -- s = units per metre
  local th = 2.0 * s        -- trunk height
  local tw = 0.45 * s       -- trunk width
  local hr = 1.35 * s * (0.85 + 0.3 * ((seed * 7919) % 100) / 100)  -- head radius
  local lean = ((seed * 31) % 7 - 3) * 0.03 * s
  local trunk = poly({{x - tw*0.6, yb}, {x - tw*0.45 + lean, yb - th}, {x + tw*0.45 + lean, yb - th}, {x + tw*0.6, yb}})
  local hx, hy = x + lean, yb - th - hr * 0.7
  local head = ellipse(hx, hy, hr, hr * 0.9):roughen(math.max(0.8, hr*0.18), math.max(3, hr*0.5), seed)
  -- a few wands poking out at the top
  for k = 1, 5 do
    local a = -math.pi/2 + (k - 3) * 0.35 + rand(-0.1, 0.1)
    local L = hr * rand(1.0, 1.35)
    head = head + ribbon({{hx, hy}, {hx + L*math.cos(a), hy + L*math.sin(a)}}, {hr*0.35, 0.6})
  end
  head = head - trunk:shrink(0.1) * rect(0, hy + hr*0.6, 1000, 400)
  work(trunk, {hand="detail", pile=wT, angle=math.pi/2, coverage=2.5, length={2, 6}, fill=true, clip=true, edge="firm"})
  stipple(head, {pile=wD, width=math.max(1.6, hr*0.3), coverage=4, pressure={0.4,0.7}, clip=head, cluster=0.2})
  local lit = head * mask(function(px, py) return ((px - hx) * -0.6 + (py - hy) * -0.8) / hr > -0.05 and 1 or 0 end)
  stipple(lit:shrink(0.6), {pile=wL, width=math.max(1.3, hr*0.22), coverage=2.2, pressure={0.35,0.6}, clip=head, cluster=0.4, feather=0.5})
end
local S = function(y) return (y - 438) * 0.149 end
local xs = {578, 628, 671, 719, 771, 818, 872, 921, 972}
for i, x in ipairs(xs) do
  local yb = 516 - (x - 560) * 0.018 + ((i*37)%5 - 2) * 0.4
  willow(x + ((i*53)%9 - 4), yb, S(yb) * 0.85, i + 100)
end
print(wait(0))

--@ chunk 52
print(wait(24*60)); print(drying(628,490), drying(628,510))

--@ chunk 53
wDk = pile{{"green earth", 2}, {"raw umber", 1.4}, {"Prussian blue", 0.2}, {"lead white", 0.6}, {"yellow ochre", 0.4}, medium=0.1}
wSil = pile{{"lead white", 4}, {"green earth", 1.5}, {"pale smalt", 1}, {"yellow ochre", 0.5}, {"raw umber", 0.15}, medium=0.1}
wTg = pile{{"lead white", 2}, {"raw umber", 1.5}, {"pale smalt", 1.0}, {"bone black", 0.15}, medium=0.1}
wTd = pile{{"raw umber", 2}, {"bone black", 0.4}, {"pale smalt", 0.6}, {"lead white", 0.5}, medium=0.1}
willows = {}
local S = function(y) return (y - 438) * 0.149 end
local xs = {578, 628, 671, 719, 771, 818, 872, 921, 972}
for i, x0 in ipairs(xs) do
  local yb = 516 - (x0 - 560) * 0.018 + ((i*37)%5 - 2) * 0.4
  local s = S(yb) * 0.85
  local seed = i + 100
  local x = x0 + ((i*53)%9 - 4)
  local th = 2.0 * s
  local lean = ((seed * 31) % 7 - 3) * 0.03 * s
  willows[i] = {x=x, yb=yb, s=s, top=yb - th, lean=lean, seed=seed}
end
local wand = brush{kind="rigger", width=1.3, point=1}
local wand2 = brush{kind="round", width=1.6, point=0.8}
for i, w in ipairs(willows) do
  local kx, ky = w.x + w.lean, w.top + 0.5
  local size = w.s * (1.7 + 0.5 * ((i * 17) % 5) / 4)
  -- dark wands fanning up from the knuckle
  wand2:load(wDk, 0.7)
  local n = 22
  for k = 1, n do
    local a = -math.pi/2 + rand(-0.95, 0.95)
    local L = size * rand(0.55, 1.05) * (1 - 0.25 * math.abs(a + math.pi/2))
    local bend = rand(-0.12, 0.12)
    local p1 = {kx + rand(-1, 1), ky}
    local p2 = {kx + L*0.5*math.cos(a - bend), ky + L*0.5*math.sin(a - bend)}
    local p3 = {kx + L*math.cos(a + bend), ky + L*math.sin(a + bend)}
    wand2:stroke({p1, p2, p3}, {pressure={0.75, 0.05}, ramps={0.05, 0.6}})
    if k % 6 == 0 then wand2:load(wDk, 0.6) end
  end
  -- silvery wands, leaves catching light on the left and top
  wand:load(wSil, 0.6)
  for k = 1, 16 do
    local a = -math.pi/2 + rand(-0.9, 0.5)
    local L = size * rand(0.5, 1.0)
    local st = rand(0.25, 0.5)
    local p1 = {kx + L*st*math.cos(a), ky + L*st*math.sin(a)}
    local p3 = {kx + L*math.cos(a + rand(-0.1,0.1)), ky + L*math.sin(a + rand(-0.1,0.1))}
    wand:stroke({p1, p3}, {pressure={0.8, 0.1}, ramps={0.1, 0.5}})
    if k % 5 == 0 then wand:load(wSil, 0.5) end
  end
  -- trunk: grey lit side and a dark right side, a knuckle at the top
  local tw = 0.45 * w.s
  local tb = brush{kind="round", width=math.max(1.6, tw*0.6), point=0.3}
  tb:load(wTg, 0.6)
  tb:stroke({{w.x - tw*0.2, w.yb}, {w.x - tw*0.15 + w.lean, w.top + 1}}, {pressure={0.7, 0.6}})
  tb:load(wTd, 0.6)
  tb:stroke({{w.x + tw*0.3, w.yb}, {w.x + tw*0.3 + w.lean, w.top + 1}}, {pressure={0.55, 0.45}})
  tb:touch(kx, w.top + 0.5, {pressure=0.8})
end
print(wait(0))

--@ chunk 54
wMid = pile{{"lead white", 2.2}, {"green earth", 2.2}, {"yellow ochre", 0.6}, {"raw umber", 0.5}, {"pale smalt", 0.6}, {"Prussian blue", 0.05}, medium=0.1}
wSh  = pile{{"green earth", 2.2}, {"raw umber", 1.2}, {"lead white", 0.9}, {"pale smalt", 0.6}, {"Prussian blue", 0.1}, medium=0.1}
wHi  = pile{{"lead white", 3}, {"green earth", 1.4}, {"yellow ochre", 0.7}, {"pale smalt", 0.5}, medium=0.1}
for i, w in ipairs(willows) do
  local kx, ky = w.x + w.lean, w.top + 1
  local H = w.s * (3.0 + 0.6 * ((i * 13) % 5) / 4)
  local Wd = H * (1.05 + 0.2 * ((i * 7) % 3) / 2)
  local pts = {{kx - 1.5, ky + 1}, {kx - Wd*0.30, ky - H*0.35}, {kx - Wd*0.5, ky - H*0.62}, {kx - Wd*0.42, ky - H*0.85},
               {kx - Wd*0.2, ky - H*0.98}, {kx, ky - H}, {kx + Wd*0.22, ky - H*0.96}, {kx + Wd*0.44, ky - H*0.82},
               {kx + Wd*0.5, ky - H*0.6}, {kx + Wd*0.3, ky - H*0.33}, {kx + 1.5, ky + 1}}
  local hm = poly(pts, true):roughen(w.s*0.25, w.s*0.8, w.seed)
  w.head = hm; w.kx = kx; w.ky = ky; w.H = H; w.Wd = Wd
  stipple(hm, {pile=wMid, width=2.4, coverage=3.5, pressure={0.4,0.7}, clip=hm, cluster=0.25})
  local sh = hm * mask(function(px, py) local u = (px - kx)/Wd*0.9 + (py - (ky - H*0.55))/H*0.8 return u > 0.12 and 1 or 0 end)
  stipple(sh, {pile=wSh, width=2.2, coverage=2.5, pressure={0.4,0.7}, clip=hm, cluster=0.35, feather=0.3})
  local hi = hm * mask(function(px, py) local u = (px - kx)/Wd*0.9 + (py - (ky - H*0.55))/H*0.8 return u < -0.22 and 1 or 0 end)
  stipple(hi, {pile=wHi, width=1.8, coverage=1.6, pressure={0.35,0.6}, clip=hm, cluster=0.5, feather=0.6})
end
print(wait(0))

--@ chunk 55
tD = pile{{"green earth", 2.5}, {"raw umber", 1.2}, {"Prussian blue", 0.25}, {"lead white", 1.6}, {"pale smalt", 1.0}, medium=0.1}
tM = pile{{"green earth", 2.5}, {"yellow ochre", 0.8}, {"raw umber", 0.6}, {"lead white", 2.2}, {"pale smalt", 0.9}, {"Prussian blue", 0.08}, medium=0.1}
tL = pile{{"lead white", 3}, {"green earth", 1.6}, {"yellow ochre", 1.0}, {"pale smalt", 0.5}, {"raw umber", 0.15}, medium=0.1}
local function lobes(list, seed)
  local m
  for i, l in ipairs(list) do
    local e = ellipse(l[1], l[2], l[3], l[4])
    m = m and (m + e) or e
  end
  return m:roughen(3.2, 9, seed)
end
crownA = lobes({{52,430,40,34},{30,440,22,20},{70,410,26,24},{45,400,22,18},{85,440,20,20},{22,455,16,12},{60,455,34,14}}, 61)
crownB = lobes({{112,410,34,38},{100,385,22,20},{125,378,16,16},{135,420,24,26},{95,440,28,22},{140,450,20,14},{115,455,30,12}}, 62)
crownC = lobes({{165,440,26,24},{175,425,16,15},{152,430,14,14},{183,450,14,12},{160,462,22,9}}, 63)
local holes = ellipse(24,423,4,3) + ellipse(88,402,3.5,3) + ellipse(145,398,3,4) + ellipse(62,392,3,2.5) + ellipse(178,436,3,2.5) + ellipse(122,432,3,2.5)
crowns = (crownA + crownB + crownC) * above({{0,478},{1000,478}}) - holes
stipple(crowns, {pile=tD, width=3.2, coverage=4.5, pressure={0.45,0.75}, clip=crowns, cluster=0.2})
print(wait(0))

--@ chunk 56
local trees = {{crownA, 52, 425, 48, 40}, {crownB, 115, 415, 42, 48}, {crownC, 166, 442, 28, 24}}
local holes = ellipse(24,423,4,3) + ellipse(88,402,3.5,3) + ellipse(145,398,3,4) + ellipse(62,392,3,2.5) + ellipse(178,436,3,2.5) + ellipse(122,432,3,2.5)
local crownsF = (crownA + crownB + crownC) * above({{0,478},{1000,478}})
-- fill the punched holes back in with the mid tone
stipple(holes:grow(1.5) * crownsF, {pile=tM, width=2.5, coverage=4, pressure={0.4,0.7}, clip=crownsF})
for i, t in ipairs(trees) do
  local m, cx, cy, rx, ry = t[1] * crownsF, t[2], t[3], t[4], t[5]
  local midm = m * mask(function(x,y) local u = (x-cx)/rx*0.7 + (y-cy)/ry*0.75 return u < 0.35 and 1 or 0 end):blur(4)
  stipple(midm, {pile=tM, width=2.8, coverage=3.0, pressure={0.4,0.7}, clip=m, cluster=0.4, feather=0.4})
end
print(wait(0))

--@ chunk 57
print(wait(20*60)); print(drying(60,430), drying(110,400))

--@ chunk 58
skyH = pile{{"lead white", 10}, {"yellow ochre", 0.42}, {"chrome yellow", 0.1}, {"vermilion", 0.05}, {"pale smalt", 0.12}, medium=0.08}
-- sky gaps: a deep notch between crown A and B, a smaller one between B and C, ragged edge bites
local gaps = poly({{86,376},{92,386},{95,398},{93,410},{97,420},{90,428},{84,418},{80,404},{78,390}}, true)
           + poly({{150,398},{155,408},{152,418},{147,425},{143,414},{142,404}}, true)
           + ellipse(30,398,6,4) + ellipse(20,414,4,4) + ellipse(142,378,5,6) + ellipse(160,404,4,5) + ellipse(62,386,5,3)
           + ellipse(100,372,4,5) + ellipse(186,428,4,4)
gaps = gaps:roughen(2, 6, 71) * above({{0,440},{1000,440}})
work(gaps, {hand="detail", pile=skyH, angle=0.3, coverage=2.5, length={3,8}, fill=true, clip=true, edge="firm"})
-- small sky holes inside the crowns, irregular
local holes = poly({{40,418},{45,414},{48,419},{44,423}}) + poly({{118,398},{123,395},{125,401},{120,404}}) + poly({{64,440},{69,437},{72,442},{66,444}})
work(holes:roughen(0.8, 3, 72), {hand="detail", pile=skyH, coverage=2.5, length={2,5}, fill=true, clip=true, edge="firm"})
print(wait(0))

--@ chunk 59
brD = pile{{"raw umber", 2}, {"bone black", 0.3}, {"lead white", 1.2}, {"pale smalt", 1.0}, medium=0.1}
-- limbs seen through the gaps
local rb = brush{kind="round", width=2.2, point=0.8}
rb:load(brD, 0.6)
rb:stroke({{96,470},{95,440},{90,418},{86,398},{80,382}}, {pressure={0.7,0.25}, ramps={0.05,0.3}})
rb:stroke({{90,418},{96,402},{100,388}}, {pressure={0.45,0.15}, ramps={0.05,0.4}})
rb:stroke({{150,470},{148,440},{147,422},{150,408},{154,398}}, {pressure={0.6,0.2}, ramps={0.05,0.4}})
rb:load(brD, 0.5)
rb:stroke({{36,470},{38,440},{30,418},{26,404}}, {pressure={0.55,0.15}, ramps={0.05,0.4}})
rb:stroke({{40,436},{47,420},{44,408}}, {pressure={0.4,0.1}, ramps={0.05,0.5}})
rb:stroke({{120,425},{120,410},{117,398}}, {pressure={0.4,0.1}, ramps={0.05,0.5}})
rb:stroke({{65,450},{66,442},{67,438}}, {pressure={0.4,0.2}})
-- leaf sprays dragged in from the rims of every gap and the outer edge
local cr = (crownA + crownB + crownC) * above({{0,478},{1000,478}})
local rim = cr:rim(5, 2) + cr:grow(3) - cr
stipple(rim * above({{0,462},{1000,462}}), {pile=tD, width=1.6, coverage=1.3, pressure={0.35,0.65}, drag={2.5, -1.2}, twist=0.8, cluster={0.6, 6}, feather=0.6})
print(wait(0))

--@ chunk 60
print(wait(3*24*60)); print(drying(80,400), drying(110,440))

--@ chunk 61
print(wait(2*24*60)); print(drying(80,400), drying(110,440), drying(50,395))

--@ chunk 62
skyH2 = pile{{"lead white", 10}, {"yellow ochre", 0.5}, {"chrome yellow", 0.12}, {"vermilion", 0.05}, {"pale smalt", 0.14}, medium=0.08}
-- design the new group: three limes
lime_trees = {
  {bx=42, by=468, fx=44, fy=442, cx=46, cy=414, rx=40, ry=38, limbs={-2.3,-1.95,-1.6,-1.25,-0.9,-2.7,-0.5}},
  {bx=110, by=471, fx=111, fy=436, cx=112, cy=404, rx=42, ry=46, limbs={-2.4,-2.0,-1.7,-1.45,-1.15,-0.8,-2.8,-0.4}},
  {bx=170, by=472, fx=170, fy=452, cx=170, cy=433, rx=26, ry=26, limbs={-2.3,-1.8,-1.3,-0.8}},
}
clumps = {}
local sil
for ti, t in ipairs(lime_trees) do
  t.limbpts = {}
  for li, a in ipairs(t.limbs) do
    local L = math.sqrt((t.rx*math.cos(a))^2 + (t.ry*math.sin(a))^2) * rand(0.8, 1.0)
    local ex = t.cx + L * math.cos(a) * 1.05
    local ey = t.cy + L * math.sin(a) * 0.95 + 8
    local mx = lerp(t.fx, ex, 0.5) + rand(-4, 4)
    local my = lerp(t.fy, ey, 0.5) - rand(0, 6)
    t.limbpts[li] = {{t.fx, t.fy}, {mx, my}, {ex, ey}}
    for k = 1, 3 do
      local u = 0.45 + 0.28 * k
      local px = lerp(lerp(t.fx, mx, u), lerp(mx, ex, u), u)
      local py = lerp(lerp(t.fy, my, u), lerp(my, ey, u), u)
      local r = (t.rx * 0.23) * rand(0.75, 1.2) * (1.1 - 0.15 * k)
      clumps[#clumps+1] = {x=px + rand(-3,3), y=py + rand(-3,3), r=r, t=ti}
    end
  end
  -- inner filling clumps, leaving some air
  for k = 1, 6 do
    local a = rand(0, 2*math.pi)
    local d = rand(0.1, 0.55)
    clumps[#clumps+1] = {x=t.cx + t.rx*d*math.cos(a), y=t.cy + t.ry*d*math.sin(a) + 6, r=t.rx*rand(0.18, 0.26), t=ti}
  end
end
for i, c in ipairs(clumps) do
  local e = ellipse(c.x, c.y, c.r * 1.15, c.r * 0.9)
  sil = sil and (sil + e) or e
end
lime_sil = sil:roughen(2.2, 5, 81) * above({{0,470},{1000,470}})
local old = ((crownA + crownB + crownC) * above({{0,478},{1000,478}})):grow(5)
local skypatch = (old - lime_sil:shrink(1)) * above({{0,440.5},{1000,440.5}})
work(skypatch, {hand="detail", pile=skyH2, angle=0.2, coverage=2.5, length={3,9}, fill=true, clip=true, edge="firm"})
local lowpatch = (old - lime_sil:shrink(1)) - above({{0,440.5},{1000,440.5}})
work(lowpatch, {hand="detail", pile=mAB, angle=0, coverage=2.5, length={3,9}, fill=true, clip=true, edge="firm"})
print(#clumps, lime_sil:area(), wait(0))

--@ chunk 63
print(wait(30*60)); print(drying(80,400), drying(60,455))

--@ chunk 64
local sil
for i, c in ipairs(clumps) do
  local e = ellipse(c.x, c.y, c.r * 1.55, c.r * 1.2)
  sil = sil and (sil + e) or e
end
lime_sil2 = (sil:roughen(2.4, 5, 82) + lime_sil) * above({{0,472},{1000,472}})
-- trunks and limbs first, dark grey-brown, visible later in the gaps
local rb = brush{kind="round", width=3.2, point=0.6}
for ti, t in ipairs(lime_trees) do
  rb:load(brD, 0.7)
  rb:stroke({{t.bx, t.by}, {t.bx + 0.5, (t.by + t.fy)/2}, {t.fx, t.fy}}, {pressure={0.95, 0.8}})
  for li, l in ipairs(t.limbpts) do
    if li % 3 == 0 then rb:load(brD, 0.6) end
    rb:stroke(l, {pressure={0.55, 0.05}, ramps={0.02, 0.5}})
  end
end
-- dark foliage mass
stipple(lime_sil2, {pile=tD, width=2.6, coverage=3.2, pressure={0.4,0.7}, clip=lime_sil2, cluster=0.3})
print(lime_sil2:area(), wait(0))

--@ chunk 65
print(wait(8*60))
tM2 = pile{{"green earth", 2.5}, {"yellow ochre", 1.2}, {"raw umber", 0.4}, {"lead white", 2.0}, {"pale smalt", 0.5}, {"chrome yellow", 0.1}, medium=0.1}
tL2 = pile{{"lead white", 3}, {"green earth", 1.5}, {"yellow ochre", 1.3}, {"chrome yellow", 0.2}, {"pale smalt", 0.3}, medium=0.1}
for ti, t in ipairs(lime_trees) do
  local lit = lime_sil2 * mask(function(x,y) local u = (x-t.cx)/t.rx*0.75 + (y-t.cy)/t.ry*0.7 local d = ((x-t.cx)/t.rx)^2 + ((y-t.cy)/t.ry)^2 return (u < 0.15 and d < 1.8) and 1 or 0 end):blur(3)
  stipple(lit, {pile=tM2, width=2.4, coverage=2.4, pressure={0.4,0.7}, clip=lime_sil2, cluster={0.5, 7}, feather=0.5})
end
-- the brightest leaf clusters: upper-left side of clumps on the lit side of each crown
local hl
for i, c in ipairs(clumps) do
  local t = lime_trees[c.t]
  local u = (c.x-t.cx)/t.rx*0.75 + (c.y-t.cy)/t.ry*0.7
  if u < -0.05 then
    local e = ellipse(c.x - c.r*0.35, c.y - c.r*0.35, c.r*0.9, c.r*0.6)
    hl = hl and (hl + e) or e
  end
end
hl = hl:roughen(1.5, 4, 83) * lime_sil2
stipple(hl, {pile=tL2, width=1.9, coverage=1.6, pressure={0.35,0.6}, clip=lime_sil2, cluster={0.5, 4}, feather=0.6})
print(wait(0))

--@ chunk 66
tS = pile{{"green earth", 2.5}, {"raw umber", 0.9}, {"lead white", 1.4}, {"pale smalt", 0.9}, {"Prussian blue", 0.1}, {"yellow ochre", 0.4}, medium=0.1}
local sh
for i, c in ipairs(clumps) do
  local t = lime_trees[c.t]
  local u = (c.x-t.cx)/t.rx*0.75 + (c.y-t.cy)/t.ry*0.7
  if u < 0.3 then
    local e = ellipse(c.x + c.r*0.3, c.y + c.r*0.55, c.r*1.1, c.r*0.55)
    sh = sh and (sh + e) or e
  end
end
sh = sh:roughen(1.6, 4, 84) * lime_sil2
stipple(sh, {pile=tS, width=2.0, coverage=2.2, pressure={0.35,0.65}, clip=lime_sil2, cluster={0.5, 5}, feather=0.5})
-- break the long diagonal between light and shadow sides with a scatter of mid touches across it
for ti, t in ipairs(lime_trees) do
  local band = lime_sil2 * mask(function(x,y) local u = (x-t.cx)/t.rx*0.75 + (y-t.cy)/t.ry*0.7 local d = ((x-t.cx)/t.rx)^2 + ((y-t.cy)/t.ry)^2 return (math.abs(u - 0.15) < 0.18 and d < 1.8) and 1 or 0 end)
  stipple(band, {pile=tS, width=2.2, coverage=1.0, pressure={0.35,0.6}, clip=lime_sil2, cluster={0.7, 6}, feather=0.7})
end
print(wait(0))

--@ chunk 67
thatch = pile{{"raw umber", 1.5}, {"yellow ochre", 1.2}, {"lead white", 2.0}, {"pale smalt", 0.8}, {"red earth", 0.2}, medium=0.1}
thatchL = pile{{"yellow ochre", 1.5}, {"raw umber", 0.8}, {"lead white", 3.0}, {"pale smalt", 0.4}, {"red earth", 0.15}, medium=0.1}
wallW = pile{{"lead white", 6}, {"yellow ochre", 0.3}, {"pale smalt", 0.35}, medium=0.1}
wallS = pile{{"lead white", 3}, {"pale smalt", 1}, {"raw umber", 0.5}, medium=0.1}
hedge = pile{{"green earth", 2.5}, {"raw umber", 0.9}, {"lead white", 1.2}, {"pale smalt", 0.7}, {"yellow ochre", 0.5}, medium=0.1}
roof = poly({{54,462},{68,447},{150,445},{162,459},{162,463},{54,464}})
roofEnd = poly({{54,462},{68,447},{75,447},{66,463}})
walls = rect(57, 462, 103, 8)
gableLit = rect(57, 462, 12, 8)
work(roof, {hand="detail", pile=thatch, angle=0.0, coverage=2.5, length={3,9}, fill=true, clip=true, edge="found"})
work(roofEnd, {hand="detail", pile=thatchL, angle=-0.8, coverage=2.2, length={2,6}, fill=true, clip=true, edge="found"})
work(walls, {hand="detail", pile=wallS, angle=0, coverage=2.5, length={2,6}, fill=true, clip=true, edge="found"})
work(gableLit, {hand="detail", pile=wallW, angle=0, coverage=2.5, length={2,5}, fill=true, clip=true, edge="found"})
-- a hedge along the yard, a dark low mass
local hm = poly({{0,472},{0,462},{12,460},{25,462},{38,459},{52,463},{56,471}}, true) + poly({{160,471},{162,463},{175,461},{190,463},{205,460},{222,463},{232,468},{234,473}}, true)
hm = hm:roughen(1.2, 4, 91)
stipple(hm, {pile=hedge, width=2.0, coverage=4, pressure={0.4,0.7}, clip=hm, cluster=0.3})
print(wait(0))

--@ chunk 68
print(wait(20*60))
bush = pile{{"green earth", 2.5}, {"raw umber", 0.8}, {"lead white", 1.5}, {"pale smalt", 0.9}, {"yellow ochre", 0.6}, {"Prussian blue", 0.05}, medium=0.1}
local pts = {{-5,470}}
local x = -5
while x < 250 do
  local top = 446 + 4*math.sin(x/17) + 3*math.sin(x/7.3)
  if x > 190 then top = top + (x - 190) * 0.35 end
  pts[#pts+1] = {x, top}
  x = x + 4
end
pts[#pts+1] = {250, 471}
local back = poly(pts, true):roughen(1.5, 4, 92) - roof - walls - lime_sil2:shrink(0.5)
stipple(back, {pile=bush, width=2.2, coverage=4, pressure={0.4,0.7}, clip=back, cluster=0.3})
-- redraw trunks down to the ground, over the new shrubs
local rb = brush{kind="round", width=3.0, point=0.5}
for ti, t in ipairs(lime_trees) do
  if ti ~= 2 then
    rb:load(brD, 0.7)
    rb:stroke({{t.bx, t.by + 1}, {t.bx + 0.5, (t.by + t.fy)/2}, {t.fx, t.fy}}, {pressure={0.9, 0.8}})
  end
end
print(wait(0))

--@ chunk 69
local band = rect(-5, 440.8, 250, 12) - lime_sil2:grow(0.5) - roof:grow(0.5) - rect(0,452,260,30)
work(band, {hand="detail", pile=haze, angle=0, coverage=2.2, length={4,12}, fill=true, clip=true, edge="soft"})
-- lower foliage on the middle lime to hide the bare fan of limbs
local low = ellipse(98, 438, 9, 6) + ellipse(122, 440, 8, 5) + ellipse(110, 432, 7, 5)
low = low:roughen(1.5, 4, 93)
stipple(low, {pile=tS, width=2.0, coverage=3.5, pressure={0.4,0.7}, clip=low, cluster=0.3})
stipple(low * mask(function(x,y) return (y < 436) and 1 or 0 end), {pile=tM2, width=1.8, coverage=1.5, pressure={0.35,0.6}, clip=low, cluster={0.5,4}, feather=0.5})
print(wait(0))

--@ chunk 70
wBody = pile{{"lead white", 4}, {"pale smalt", 1.6}, {"cobalt blue", 0.3}, {"raw umber", 0.25}, {"green earth", 0.3}, medium=0.1}
wRefl = pile{{"green earth", 2}, {"raw umber", 1}, {"lead white", 1.2}, {"pale smalt", 0.6}, medium=0.1}
wGlint = pile{{"lead white", 8}, {"pale smalt", 0.6}, {"yellow ochre", 0.12}, medium=0.1}
local rv = river2:shrink(0.4)
work(rv, {hand="detail", pile=wBody, angle=0, coverage=2.4, length={5,14}, fill=true, clip=true, edge="found"})
-- reflection of the far bank at the upper edge of each reach
local top = {} local tw = {}
for i,p in ipairs(near_pts) do top[i] = {p[1], p[2] - near_w[i]*0.36}; tw[i] = near_w[i]*0.28 end
local refl = ribbon(top, tw) * rv
work(refl, {hand="detail", pile=wRefl, angle=0, coverage=1.8, length={3,9}, fill=true, clip=true, edge="soft", load=0.5})
-- a few pale glints across the broad near reach
local g = brush{kind="round", width=1.6, point=0.6}
g:load(wGlint, 0.5)
for k, s in ipairs({{12,578,40},{70,573,25},{120,568,30},{175,561,20},{212,552,14}}) do
  g:stroke({{s[1], s[2]}, {s[1] + s[3], s[2] - s[3]*0.08}}, {pressure={0.35,0.1}, ramps={0.2,0.5}, clip=rv})
end
print(wait(0))

--@ chunk 71
pathS = pile{{"yellow ochre", 2}, {"lead white", 2}, {"raw umber", 0.9}, {"red earth", 0.2}, {"pale smalt", 0.3}, medium=0.08}
pathD = pile{{"raw umber", 1.5}, {"yellow ochre", 1}, {"lead white", 0.8}, {"green earth", 0.6}, medium=0.08}
path_c = {{360,720},{392,690},{430,666},{480,646},{530,630},{570,618},{600,610},{622,605}}
path_w = {70,56,44,34,26,19,14,10}
path_m = ribbon(path_c, path_w):roughen(2.5, 12, 101) * below({{0,604},{1000,604}}) 
-- two ruts: the path is a cart track with grass in the middle
local mid = {} local mw = {}
for i,p in ipairs(path_c) do mid[i] = {p[1] + 1, p[2]}; mw[i] = path_w[i]*0.24 end
path_mid = ribbon(mid, mw):roughen(1.5, 6, 102)
work(path_m, {hand="body", pile=pathS, angle=function(x,y) return -0.45 + (x-360)/800 end, coverage=2.2, length={10,35}, pressure={0.45,0.75}, fill=true, clip=path_m, edge="soft"})
print(wait(0))

--@ chunk 72
print(wait(40*60)); print(drying(450,650), drying(100,570))

--@ chunk 73
print(wait(48*60)); print(drying(450,650), drying(100,570), drying(300,470))

--@ chunk 74
wg2 = pile{{"smalt", 1.5}, {"pale smalt", 1}, {"raw umber", 0.3}, {"green earth", 0.4}, {"lead white", 0.3}, medium=0.6}
local rv = river2:shrink(0.3)
-- darker on the near (lower) half of each reach
local low = {} local lw = {}
for i,p in ipairs(near_pts) do low[i] = {p[1], p[2] + near_w[i]*0.15}; lw[i] = near_w[i]*0.7 end
local lowm = ribbon(low, lw) * rv
work(rv, {hand="detail", pile=wg2, angle=0, coverage=1.2, length={6,16}, pressure={0.3,0.5}, load=0.35, fill=true, clip=true})
work(lowm, {hand="detail", pile=wg2, angle=0, coverage=1.2, length={6,16}, pressure={0.3,0.5}, load=0.4, fill=true, clip=true})
blend(rv, {angle=0, coverage=1.2, clip=true, tool={kind="badger", width=6}, pressure={0.2,0.3}})
-- the track: shade and ruts
pathG = pile{{"raw umber", 1.2}, {"green earth", 1}, {"yellow ochre", 0.5}, {"pale smalt", 0.3}, medium=0.55}
work(path_m, {hand="glaze", pile=pathG, angle=-0.3, coverage=1.2, load=0.4})
print(wait(0))

--@ chunk 75
print(wait(48*60)); print(drying(600,560), drying(500,600), drying(450,650))

--@ chunk 76
local trunks
for i, w in ipairs(willows) do
  local r = rect(w.x - 5, w.top - 2, 10, w.yb - w.top + 3)
  trunks = trunks and (trunks + r) or r
end
local hay = fieldB - river2:grow(1.5) - trunks - fore_m
work(hay, {hand="body", pile=straw, angle=-0.02, coverage=2.0, length={20,60}, pressure={0.45,0.75}, fill=true, clip=hay:grow(0.8), edge="firm"})
blend(hay, {angle=-0.02, coverage=1.0, clip=true, tool={kind="badger", width=16}, pressure={0.25,0.4}})
local strip = (mid_m * mask(function(x,y) return y > 560 and 1 or 0 end)) - fieldB - river2:grow(1.5)
work(strip, {hand="body", pile=gG, angle=0, coverage=2.0, length={15,50}, pressure={0.45,0.75}, fill=true, clip=strip:grow(0.8), edge="soft"})
blend(strip, {angle=0, coverage=1.0, clip=true, tool={kind="badger", width=14}, pressure={0.25,0.4}})
print(wait(0))

--@ chunk 77
local fg = fore_m - path_m:shrink(1)
local zC = mask(function(x,y) local d = crest_depth(x,y) return d < 0 and 0 or (1 - smoothstep(25, 45, d)) end) * fg
local zM = mask(function(x,y) local d = crest_depth(x,y) return d < 0 and 0 or smoothstep(25, 45, d) * (1 - smoothstep(70, 95, d)) end) * fg
local zD = mask(function(x,y) local d = crest_depth(x,y) return d < 0 and 0 or smoothstep(70, 95, d) end) * fg
work(zC, {hand="body", pile=sC, angle=0.03, coverage=2.0, length={20,50}, pressure={0.5,0.8}, fill=true, clip=fg, edge="found", threshold=0.3})
work(zM, {hand="body", pile=sM, angle=0.05, coverage=2.0, length={25,60}, pressure={0.5,0.8}, fill=true, clip=fg, threshold=0.3})
work(zD, {hand="body", pile=sD, angle=0.08, coverage=2.0, length={25,70}, pressure={0.5,0.85}, fill=true, clip=fg, threshold=0.3})
blend(fg, {angle=0.02, coverage=1.2, clip=true, tool={kind="badger", width=22}, pressure={0.3,0.45}})
pathS2 = pile{{"yellow ochre", 2}, {"lead white", 1.4}, {"raw umber", 1.2}, {"red earth", 0.15}, {"pale smalt", 0.5}, {"green earth", 0.3}, medium=0.08}
work(path_m, {hand="body", pile=pathS2, angle=function(x,y) return -0.45 + (x-360)/800 end, coverage=2.2, length={10,30}, pressure={0.45,0.75}, fill=true, clip=path_m, edge="soft"})
print(wait(0))

--@ chunk 78
print(wait(3*24*60)); print(drying(600,560), drying(500,640), drying(450,690))

--@ chunk 79
hcL = pile{{"lead white", 3}, {"yellow ochre", 2.5}, {"chrome yellow", 0.2}, {"raw umber", 0.25}, medium=0.08}
hcS = pile{{"yellow ochre", 1.5}, {"raw umber", 1.2}, {"lead white", 1.2}, {"pale smalt", 0.6}, {"green earth", 0.4}, medium=0.08}
hcC = pile{{"raw umber", 1}, {"green earth", 0.8}, {"yellow ochre", 0.8}, {"lead white", 0.6}, {"pale smalt", 0.4}, medium=0.1}
wind = pile{{"yellow ochre", 1.6}, {"green earth", 1.2}, {"lead white", 1.6}, {"raw umber", 0.35}, medium=0.08}
-- windrows first: slightly darker, greener lines of raked hay
local wr = brush{kind="round", width=1.8, point=0.4}
for k, y0 in ipairs({526, 534, 546, 559, 571}) do
  wr:load(wind, 0.5)
  local xs = 330 + (y0 - 520) * 1.55 + 20
  local pts = {}
  for x = xs, 1005, 45 do pts[#pts+1] = {x, y0 - (x - 330) * 0.006 + 0.8*math.sin(x/37 + k)} end
  wr:stroke(pts, {pressure={0.4,0.3}, ramps={0.05,0.05}, shake=0.5})
end
haycocks = {}
local rows = {{y=531, xs={430, 505, 590, 690, 800, 905}}, {y=552, xs={470, 560, 665, 770, 890, 985}}, {y=572, xs={520, 640, 745, 860, 960}}}
for ri, r in ipairs(rows) do
  for ci, x in ipairs(r.xs) do
    local y = r.y - (x - 330) * 0.006 + rand(-1.2, 1.2)
    local S = (y - 438) * 0.13
    local h = 1.05 * S * rand(0.9, 1.1)
    local w = 1.45 * S * rand(0.9, 1.1)
    x = x + rand(-8, 8)
    haycocks[#haycocks+1] = {x=x, y=y, h=h, w=w}
    local body = poly({{x - w/2, y}, {x - w*0.45, y - h*0.45}, {x - w*0.28, y - h*0.85}, {x, y - h}, {x + w*0.28, y - h*0.85}, {x + w*0.45, y - h*0.45}, {x + w/2, y}}, true)
    local cast = ellipse(x + w*0.45, y + 0.3, w*0.55, math.max(1.0, h*0.12))
    work(cast - body, {hand="detail", pile=hcC, angle=0, coverage=2, length={2,6}, fill=true, clip=true, edge="soft", load=0.5})
    work(body, {hand="detail", pile=hcS, angle=math.pi/2, coverage=2.2, length={2,6}, fill=true, clip=true, edge="firm"})
    local lit = body * mask(function(px, py) return ((px - x) / w + (py - (y - h*0.5)) / h * 0.4) < -0.02 and 1 or 0 end)
    work(lit, {hand="detail", pile=hcL, angle=-1.2, coverage=2.0, length={2,5}, fill=true, clip=body, edge="soft"})
  end
end
print(#haycocks, wait(0))

--@ chunk 80
coat = pile{{"bone black", 1}, {"Prussian blue", 0.3}, {"raw umber", 1}, {"lead white", 0.35}, {"green earth", 0.4}, medium=0.08}
dress = pile{{"raw umber", 1.2}, {"smalt", 1}, {"bone black", 0.35}, {"lead white", 0.6}, {"red earth", 0.2}, medium=0.08}
shawl = pile{{"vermilion", 2}, {"red earth", 1}, {"raw umber", 0.3}, {"bone black", 0.08}, medium=0.08}
blackp = pile{{"bone black", 2}, {"raw umber", 1}, {"lead white", 0.1}, medium=0.08}
skin = pile{{"lead white", 2}, {"red earth", 0.5}, {"yellow ochre", 0.6}, {"raw umber", 0.3}, medium=0.08}
hair = pile{{"raw umber", 2}, {"red earth", 0.4}, {"bone black", 0.3}, {"lead white", 0.2}, medium=0.08}
local mx, fy = 697, 612
-- man
man_legs = poly({{mx-4.2,598},{mx-3.8,fy},{mx-0.9,fy},{mx-0.6,598}}) + poly({{mx+0.6,598},{mx+1.0,fy},{mx+3.9,fy},{mx+4.2,598}})
man_coat = poly({{mx-2.2,579.5},{mx-5.2,581},{mx-6.2,584},{mx-6.4,592},{mx-6.8,601},{mx+6.8,601},{mx+6.5,592},{mx+6.3,584},{mx+5.4,581},{mx+2.2,579.5}}, true)
man_head = ellipse(mx, 576.2, 2.7, 3.3)
man_hat = ellipse(mx + 0.3, 572.9, 4.3, 1.7) + ellipse(mx + 0.2, 574.0, 3.2, 1.3)
man = man_legs + man_coat + man_head + man_hat
-- woman
local wx = 681
wo_dress = poly({{wx-3,587},{wx-4.2,595},{wx-6.2,605},{wx-7.0,fy},{wx+7.0,fy},{wx+6.3,605},{wx+4.4,595},{wx+3,587}}, true)
wo_body = poly({{wx-2,580.5},{wx-4.4,582},{wx-4.2,588},{wx+4.2,588},{wx+4.4,582},{wx+2,580.5}}, true)
wo_head = ellipse(wx, 577.6, 2.5, 3.0)
wo_bonnet = ellipse(wx, 576.6, 3.3, 3.2)
wo_shawl = poly({{wx-4.8,581.5},{wx+4.8,581.5},{wx+4.3,585},{wx, 593},{wx-4.3,585}}, true)
wo_arm = ribbon({{wx+3.8,584},{wx+6.5,587},{mx-5.5,588}}, {2.2, 2.0, 1.8})
woman = wo_dress + wo_body + wo_head + wo_bonnet + wo_shawl + wo_arm
figs = man + woman
work(man_legs, {hand="detail", pile=blackp, angle=math.pi/2, coverage=3, length={2,5}, fill=true, clip=true, edge="found"})
work(man_coat + wo_arm, {hand="detail", pile=coat, angle=math.pi/2, coverage=3, length={2,6}, fill=true, clip=true, edge="found"})
work(wo_dress + wo_body, {hand="detail", pile=dress, angle=math.pi/2, coverage=3, length={2,6}, fill=true, clip=true, edge="found"})
work(man_head, {hand="detail", pile=hair, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
work(man_hat, {hand="detail", pile=blackp, angle=0, coverage=3, length={2,4}, fill=true, clip=true, edge="found"})
work(wo_bonnet, {hand="detail", pile=dress, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
work(wo_shawl, {hand="detail", pile=shawl, angle=math.pi/2, coverage=3, length={2,5}, fill=true, clip=true, edge="found"})
print(wait(0))

--@ chunk 81
cowR = pile{{"red earth", 1.5}, {"raw umber", 1.2}, {"lead white", 0.6}, {"yellow ochre", 0.3}, medium=0.08}
cowW = pile{{"lead white", 5}, {"yellow ochre", 0.3}, {"pale smalt", 0.3}, medium=0.08}
cowB = pile{{"bone black", 1}, {"raw umber", 1}, {"lead white", 0.4}, medium=0.08}
cowSh = pile{{"green earth", 2}, {"raw umber", 1.2}, {"lead white", 0.6}, {"Prussian blue", 0.1}, medium=0.1}
function cow(x, yb, s, d, pile_, grazing, patch)
  local L = 1.7 * s
  local bt, bb = yb - 1.3*s, yb - 0.62*s
  local x0, x1 = x, x + L*d
  local function X(u) return x + u*L*d end
  local body = poly({{X(0.02), bb}, {X(0), bt + 0.12*s}, {X(0.08), bt}, {X(0.45), bt + 0.06*s}, {X(0.85), bt - 0.03*s}, {X(1.0), bt + 0.1*s}, {X(1.02), bb - 0.05*s}, {X(0.7), bb + 0.08*s}, {X(0.3), bb + 0.06*s}})
  local lw = math.max(0.9, 0.16*s)
  local legs = rect(math.min(X(0.08), X(0.08)+lw*d), bb - 1, lw, yb - bb + 1) + rect(math.min(X(0.2), X(0.2)+lw*d), bb - 1, lw, yb - bb + 0.6)
             + rect(math.min(X(0.8), X(0.8)+lw*d), bb - 1, lw, yb - bb + 1) + rect(math.min(X(0.92), X(0.92)+lw*d), bb - 1, lw, yb - bb + 0.6)
  local head
  if grazing then
    head = poly({{X(0.95), bt + 0.1*s}, {X(1.12), bt + 0.35*s}, {X(1.28), yb - 0.15*s}, {X(1.2), yb}, {X(1.08), yb - 0.25*s}, {X(0.97), bb - 0.2*s}})
  else
    head = poly({{X(0.95), bt + 0.05*s}, {X(1.12), bt - 0.25*s}, {X(1.3), bt - 0.05*s}, {X(1.33), bt + 0.35*s}, {X(1.2), bt + 0.4*s}, {X(1.0), bb - 0.15*s}})
  end
  local tail = ribbon({{X(0.0), bt + 0.1*s}, {X(-0.05), bb}, {X(-0.04), yb - 0.3*s}}, {0.5, 0.4, 0.5})
  local shadowm = ellipse(X(0.55), yb + 0.2, L*0.65, math.max(0.8, 0.14*s))
  work(shadowm, {hand="detail", pile=cowSh, angle=0, coverage=2, length={2,5}, fill=true, clip=true, edge="soft", load=0.5})
  work(body + head + legs + tail, {hand="detail", pile=pile_, angle=0, coverage=3, length={1.5,5}, fill=true, clip=true, edge="found"})
  if patch then
    local pm = (ellipse(X(0.35), bt + 0.35*s, 0.25*L, 0.28*s) + ellipse(X(0.8), bt + 0.25*s, 0.12*L, 0.22*s)):roughen(0.5, 2, 5) * body
    work(pm, {hand="detail", pile=patch, coverage=3, length={1,3}, fill=true, clip=true, edge="firm"})
  end
end
cow(292, 497, 6.2, 1, cowR, true)
cow(338, 493, 5.8, -1, cowB, true, cowW)
cow(372, 501, 6.6, 1, cowR, false)
cow(262, 505, 6.8, -1, cowW, true, cowR)
print(wait(0))

--@ chunk 82
print(wait(20*60)); print(drying(300,497), drying(681,600), drying(697,590))

--@ chunk 83
local out = (ellipse(265,501,14,8) + ellipse(297,492,14,8) + ellipse(334,489,13,7.5) + ellipse(379,496,15,8)):roughen(1.2, 5, 111)
out = out - river2:grow(1.0)
work(out, {hand="detail", pile=gG, angle=0, coverage=3, length={3,8}, fill=true, clip=true, edge="soft"})
print(wait(0))

--@ chunk 84
print(wait(2*24*60)); print(drying(300,497), drying(681,600))

--@ chunk 85
print(wait(24*60)); print(drying(300,497), drying(681,600))

--@ chunk 86
local reg = fieldA * rect(150, 470, 460, 52) - river2:grow(1.2) - fieldB:grow(0.5)
local up = reg * mask(function(x,y) return 1 - smoothstep(484, 494, y) end)
local dn = reg * mask(function(x,y) return smoothstep(484, 494, y) end)
work(up, {hand="body", pile=gGf, angle=-0.02, coverage=2.0, length={15,40}, pressure={0.45,0.7}, fill=true, clip=reg, threshold=0.3})
work(dn, {hand="body", pile=gG, angle=-0.02, coverage=2.0, length={15,40}, pressure={0.45,0.7}, fill=true, clip=reg, threshold=0.3})
blend(reg, {angle=0, coverage=1.5, clip=true, tool={kind="badger", width=16}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 87
local wm
for i, w in ipairs(willows) do local r = w.head:grow(1) + rect(w.x - 4, w.top - 2, 9, w.yb - w.top + 3) wm = wm and (wm + r) or r end
local z = (rect(570, 468, 60, 54) * fieldA) - fieldB:grow(0.5) - wm
blend(z, {angle=0, coverage=2.0, clip=true, tool={kind="badger", width=12}, pressure={0.35,0.5}})
blend(rect(150, 466, 460, 10) * fieldA, {angle=0, coverage=1.5, clip=true, tool={kind="badger", width=10}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 88
print(wait(3*24*60)); print(drying(400,490), drying(620,500))

--@ chunk 89
print(wait(2*24*60)); print(drying(400,490), drying(620,500), drying(300,478))

--@ chunk 90
wmask = nil
for i, w in ipairs(willows) do
  if i > 1 then
    local r = w.head:grow(0.5) + rect(w.x - 3.5 + w.lean*0.5, w.top - 2, 7.5, w.yb - w.top + 2.5)
    wmask = wmask and (wmask + r) or r
  end
end
local reg = fieldA * rect(236, 468, 780, 56) - river2:grow(1.2) - fieldB:grow(0.3) - wmask
local up = reg * mask(function(x,y) return 1 - smoothstep(482, 494, y) end)
local dn = reg * mask(function(x,y) return smoothstep(482, 494, y) end)
work(up, {hand="body", pile=gGf, angle=-0.01, coverage=2.0, length={15,45}, pressure={0.45,0.7}, fill=true, clip=reg, threshold=0.3})
work(dn, {hand="body", pile=gG, angle=-0.01, coverage=2.0, length={15,45}, pressure={0.45,0.7}, fill=true, clip=reg, threshold=0.3})
blend(reg, {angle=0, coverage=1.6, clip=true, tool={kind="badger", width=18}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 91
local z = fieldA * rect(200, 466, 90, 58) - river2:grow(1.2) - fieldB:grow(0.3)
blend(z, {angle=0, coverage=2.0, clip=true, tool={kind="badger", width=20}, pressure={0.45,0.6}})
-- continue a thin scumble of the same mid green leftward over the dry older pasture to match
local lft = fieldA * rect(0, 474, 250, 50) - river2:grow(1.2) - fore_m
work(lft, {hand="scumble", pile=gG, angle=0, coverage=1.2, length={15,40}, pressure={0.3,0.5}, load=0.35, clip=lft, threshold=0.3})
blend(lft + z, {angle=0, coverage=1.2, clip=true, tool={kind="badger", width=16}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 92
print(wait(3*24*60)); print(drying(400,490), drying(200,500), drying(300,478))

--@ chunk 93
local r = brush{kind="round", width=2.4, point=0.3}
local fp = {{226,505.5},{262,501.5},{305,497.5},{350,493},{392,488.5},{428,484},{458,479},{488,472.5},{515,466},{540,459.5},{562,454},{580,450},{596,447}}
local bp = {} for i,p in ipairs(fp) do bp[i] = {p[1], p[2] + 1.2 - i*0.06} end
r:load(bank2, 0.5)
r:stroke({table.unpack(bp, 1, 7)}, {pressure={0.5,0.35}, ramps={0.02,0.1}})
r:load(bank2, 0.4)
r:stroke({table.unpack(bp, 7, 13)}, {pressure={0.35,0.15}, ramps={0.05,0.3}})
local g = brush{kind="round", width=2.2, point=0.3}
local wf = pile{{"lead white", 5}, {"pale smalt", 1.6}, {"cobalt blue", 0.35}, {"raw umber", 0.12}, medium=0.12}
g:load(wf, 0.55)
g:stroke({table.unpack(fp, 1, 5)}, {pressure={0.55,0.42}, ramps={0.0,0.1}})
g:load(wf, 0.45)
g:stroke({table.unpack(fp, 5, 9)}, {pressure={0.4,0.3}, ramps={0.05,0.1}})
g:load(wf, 0.4)
g:stroke({table.unpack(fp, 9, 13)}, {pressure={0.28,0.12}, ramps={0.05,0.4}})
print(wait(0))

--@ chunk 94
local wm
for i, w in ipairs(willows) do if i > 1 then local r = w.head:grow(1.2) wm = wm and (wm + r) or r end end
willow_heads = wm
local band = rect(180, 444, 830, 30) * below({{-10,441.5},{1010,441.5}}) - wm - lime_sil2:grow(1) - roof:grow(1)
band = band * mask(function(x,y) return 1 - smoothstep(466, 474, y) end)
work(band, {hand="body", pile=mAB, angle=0, coverage=1.8, length={20,50}, pressure={0.45,0.7}, fill=true, clip=band:grow(0.5), threshold=0.3})
blend(band:grow(3) * below({{-10,441.5},{1010,441.5}}) - wm, {angle=0, coverage=1.4, clip=true, tool={kind="badger", width=14}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 95
local b = below({{-10,441.5},{1010,441.5}})
local lz = rect(150, 443, 70, 32) * b - lime_sil2:grow(1) - roof:grow(1)
blend(lz, {angle=0, coverage=2.0, clip=true, tool={kind="badger", width=14}, pressure={0.4,0.55}})
local top = rect(180, 441.5, 830, 10) * b - willow_heads
blend(top, {angle=0, coverage=1.5, clip=true, tool={kind="badger", width=8}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 96
print(wait(3*24*60)); print(drying(400,455))

--@ chunk 97
hedgeF = pile{{"green earth", 2.5}, {"raw umber", 0.7}, {"lead white", 2.0}, {"pale smalt", 1.1}, {"yellow ochre", 0.5}, medium=0.1}
hedgeFL = pile{{"green earth", 2}, {"lead white", 2.8}, {"yellow ochre", 1.0}, {"pale smalt", 0.6}, medium=0.1}
local pts = {}
local x = 175
local n = noise{seed=121, period=18, octaves=3}
while x <= 1010 do
  local base = 474.5 - (x - 175) * 0.0015
  local h = 3.2 + 1.8 * n(x, 1)
  -- occasional hedgerow trees
  for _, t in ipairs({{262,11,9},{395,8,7},{548,13,10},{612,7,6},{850,9,8},{975,12,9}}) do
    local d = math.abs(x - t[1])
    if d < t[3] then h = math.max(h, t[2] * math.sqrt(1 - (d/t[3])^2) + 3) end
  end
  pts[#pts+1] = {x, base - math.max(1.5, h)}
  x = x + 2.5
end
local top = {}
for i, p in ipairs(pts) do top[i] = p end
local poly_pts = {{175, 476.5}}
for i, p in ipairs(pts) do poly_pts[#poly_pts+1] = p end
poly_pts[#poly_pts+1] = {1010, 476.5}
hedgerow = poly(poly_pts):roughen(0.9, 3, 122) - willow_heads
stipple(hedgerow, {pile=hedgeF, width=1.8, coverage=5, pressure={0.4,0.7}, clip=hedgerow, cluster=0.25})
local lit = hedgerow * mask(function(px,py) return 1 end) - hedgerow:offset(-1.5)
print(wait(0))

--@ chunk 98
print(wait(30*60))
hedgeD = pile{{"green earth", 2.5}, {"raw umber", 1.0}, {"lead white", 0.9}, {"pale smalt", 0.7}, {"yellow ochre", 0.5}, {"Prussian blue", 0.06}, medium=0.1}
local h = hedgerow:shrink(0.4)
stipple(h, {pile=hedgeD, width=1.7, coverage=4, pressure={0.4,0.7}, clip=hedgerow, cluster=0.3})
-- sunlit tops, upper left edges of the clumps
local tops = hedgerow - hedgerow:offset(-1.6) 
local lit = tops * mask(function(x,y) return 1 end)
stipple(lit, {pile=hedgeFL, width=1.4, coverage=1.2, pressure={0.35,0.55}, clip=hedgerow, cluster={0.6, 5}, feather=0.6})
print(wait(0))

--@ chunk 99
ryeL = pile{{"lead white", 3}, {"yellow ochre", 2}, {"green earth", 1.4}, {"pale smalt", 0.5}, {"chrome yellow", 0.1}, medium=0.1}
local b = below({{-10,441.5},{1010,441.5}})
local rye = rect(170, 441.5, 840, 32) * b - lime_sil2:grow(0.5) - roof:grow(0.5) - hedgerow:grow(0.3) - willow_heads
local up = rye * mask(function(x,y) return 1 - smoothstep(446, 466, y) end)
work(up, {hand="scumble", pile=ryeL, angle=0, coverage=1.4, length={20,50}, pressure={0.3,0.5}, load=0.4, clip=rye, threshold=0.25})
blend(rye, {angle=0, coverage=1.2, clip=true, tool={kind="badger", width=12}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 100
grD = pile{{"green earth", 2}, {"raw umber", 1.6}, {"Prussian blue", 0.25}, {"yellow ochre", 0.6}, {"bone black", 0.1}, {"lead white", 0.2}, medium=0.1}
grM = pile{{"green earth", 2.5}, {"yellow ochre", 1.4}, {"raw umber", 0.8}, {"Prussian blue", 0.12}, {"lead white", 0.7}, medium=0.1}
grL = pile{{"yellow ochre", 2}, {"green earth", 1.5}, {"lead white", 1.4}, {"chrome yellow", 0.25}, {"raw umber", 0.2}, medium=0.1}
fg_grass = fore_m - path_m:shrink(2) - figs:grow(0.5)
local near = fg_grass * mask(function(x,y) return crest_depth(x,y) > 55 and 1 or 0 end)
local midz = fg_grass * mask(function(x,y) local d = crest_depth(x,y) return (d > 18 and d <= 55) and 1 or 0 end)
local farz = fg_grass * mask(function(x,y) local d = crest_depth(x,y) return (d > 1 and d <= 18) and 1 or 0 end)
local up = function(x,y) return -math.pi/2 + 0.25*math.sin(x/23 + y/31) end
local tool = {kind="round", width=2.2, point=0.9}
work(near, {hand="hatch", pile=grD, tool=tool, angle=up, length={8,20}, coverage=1.3, pressure={0.5,0.8}, ramps={0.05,0.7}, clip=fg_grass})
work(midz, {hand="hatch", pile=grD, tool={kind="round", width=1.7, point=0.9}, angle=up, length={5,11}, coverage=1.2, pressure={0.45,0.75}, ramps={0.05,0.7}, clip=fg_grass})
work(farz, {hand="hatch", pile=grM, tool={kind="round", width=1.3, point=0.9}, angle=up, length={3,6}, coverage=1.0, pressure={0.4,0.7}, ramps={0.05,0.7}, clip=fg_grass})
print(wait(0))

--@ chunk 101
local near = fg_grass * mask(function(x,y) return crest_depth(x,y) > 50 and 1 or 0 end)
local midz = fg_grass * mask(function(x,y) local d = crest_depth(x,y) return (d > 12 and d <= 50) and 1 or 0 end)
local up = function(x,y) return -math.pi/2 + 0.3*math.sin(x/19 + y/27) end
work(near, {hand="hatch", pile=grM, tool={kind="round", width=2.0, point=1}, angle=up, length={10,24}, coverage=0.9, pressure={0.45,0.75}, ramps={0.05,0.75}, clip=fg_grass})
work(near, {hand="hatch", pile=grL, tool={kind="rigger", width=1.6, point=1}, angle=up, length={10,22}, coverage=0.35, pressure={0.5,0.8}, ramps={0.05,0.75}, clip=fg_grass})
work(midz, {hand="hatch", pile=grL, tool={kind="rigger", width=1.2, point=1}, angle=up, length={5,11}, coverage=0.35, pressure={0.5,0.8}, ramps={0.05,0.75}, clip=fg_grass})
print(wait(0))

--@ chunk 102
rutD = pile{{"raw umber", 1.5}, {"yellow ochre", 1.0}, {"lead white", 0.6}, {"red earth", 0.15}, {"green earth", 0.4}, medium=0.1}
trackL = pile{{"yellow ochre", 2}, {"lead white", 2.2}, {"raw umber", 0.5}, {"red earth", 0.12}, medium=0.1}
-- ruts: two lines following the track, converging with distance
local function side(k)
  local pts = {}
  for i, p in ipairs(path_c) do pts[i] = {p[1] + k * path_w[i] * 0.28 * (-0.45), p[2] + k * path_w[i] * 0.28} end
  return pts
end
local r = brush{kind="round", width=3.5, point=0.6}
for _, k in ipairs({-1, 1}) do
  local pts = side(k)
  r:load(rutD, 0.7)
  r:stroke({table.unpack(pts, 1, 4)}, {pressure={0.75,0.5}, ramps={0.02,0.05}, shake=0.6, clip=path_m})
  r:load(rutD, 0.5)
  r:stroke({table.unpack(pts, 4, 8)}, {pressure={0.45,0.15}, ramps={0.02,0.3}, shake=0.5, clip=path_m})
end
-- grassy crown down the middle
local mid = {}
for i, p in ipairs(path_c) do mid[i] = {p[1], p[2]} end
local crown = ribbon(mid, {12,9,7,5,4,3,2,1.5}):roughen(1, 4, 131) * path_m
work(crown, {hand="hatch", pile=grM, tool={kind="round", width=1.5, point=1}, angle=-math.pi/2, length={3,8}, coverage=1.2, pressure={0.45,0.75}, ramps={0.05,0.7}, clip=crown:grow(1)})
-- light on the track's sunny far edge? it's in shade: just a few paler pebbles and a dry crust near the viewer
stipple(path_m * mask(function(x,y) return y > 660 and 1 or 0 end) - crown, {pile=trackL, width=1.4, coverage=0.25, pressure={0.3,0.6}, cluster=0.5})
-- grass encroaching over the track edges
local edges = path_m:rim(4, 1) 
work(edges * path_m, {hand="hatch", pile=grD, tool={kind="round", width=1.6, point=1}, angle=-math.pi/2, length={4,10}, coverage=0.8, pressure={0.45,0.75}, ramps={0.05,0.7}})
print(wait(0))

--@ chunk 103
eD = pile{{"green earth", 2}, {"raw umber", 1.5}, {"Prussian blue", 0.35}, {"yellow ochre", 0.5}, {"bone black", 0.12}, {"lead white", 0.2}, medium=0.1}
eStem = pile{{"raw umber", 1.5}, {"lead white", 0.8}, {"pale smalt", 0.4}, {"bone black", 0.2}, medium=0.1}
elder_br = {
  {{955,716},{945,660},{925,610},{900,575},{878,560}},
  {{960,716},{958,650},{950,590},{940,540},{930,515}},
  {{968,716},{972,640},{975,580},{985,530},{996,500}},
  {{975,716},{990,660},{1000,620},{1010,600}},
  {{950,716},{925,685},{895,660},{868,648}},
  {{958,690},{930,640},{905,625},{886,612}},
}
local rb = brush{kind="round", width=4, point=0.5}
for i, b in ipairs(elder_br) do
  rb:load(eStem, 0.7)
  rb:stroke(b, {pressure={0.9, 0.15}, ramps={0.02,0.5}})
end
eclumps = {}
for i, b in ipairs(elder_br) do
  for k = 2, #b do
    local p, q = b[k-1], b[k]
    for u = 0.2, 1.0, 0.4 do
      local x, y = lerp(p[1], q[1], u), lerp(p[2], q[2], u)
      eclumps[#eclumps+1] = {x = x + rand(-10, 10), y = y + rand(-8, 6), r = rand(10, 17) * (0.8 + 0.4*(y - 480)/230)}
    end
  end
end
local sil
for i, c in ipairs(eclumps) do
  local e = ellipse(c.x, c.y, c.r*1.2, c.r*0.85)
  sil = sil and (sil + e) or e
end
elder = sil:roughen(3, 7, 141) * rect(0, 0, 1000, 714)
stipple(elder, {pile=eD, width=3.2, coverage=4, pressure={0.45,0.75}, clip=elder, cluster=0.3})
print(#eclumps, wait(0))

--@ chunk 104
eM = pile{{"green earth", 2.5}, {"raw umber", 1.0}, {"Prussian blue", 0.25}, {"yellow ochre", 0.8}, {"lead white", 0.5}, medium=0.1}
local o = outline{{1004,716},{842,716,"c"},{846,690},{853,662},{849,636},{858,610},{866,584},{880,560},{897,548},{912,530},{922,508},{936,494},{952,496},{963,482},{980,474},{996,468},{1004,470,"c"}, char="soft", seed=151, lobe=9, amount=1.3, closed=true}
elder2 = (o:mask() + elder) * rect(0, 0, 1000, 714)
stipple(elder2, {pile=eD, width=3.0, coverage=3.2, pressure={0.45,0.75}, clip=elder2, cluster=0.25})
-- mid-green sprays over the upper left (the light comes from the left), dragged outward-downward like pinnate leaves
local lit = elder2 * mask(function(x,y) return ((x - 960) * -0.7 + (y - 600) * -0.5) > -20 and 1 or 0 end)
stipple(lit, {pile=eM, width=2.4, coverage=1.6, pressure={0.4,0.7}, clip=elder2, cluster={0.6, 9}, drag={3, 0.5}, twist=0.6, feather=0.4})
print(wait(0))

--@ chunk 105
print(drying(681,600), drying(697,590))
dress2 = pile{{"smalt", 1.2}, {"raw umber", 1.2}, {"bone black", 0.5}, {"lead white", 0.35}, {"red earth", 0.1}, medium=0.08}
work(wo_dress + wo_body + wo_bonnet, {hand="detail", pile=dress2, angle=math.pi/2, coverage=3, length={2,5}, fill=true, clip=true, edge="found"})
-- light down the left (sunward) edge of both figures: a hairline of grey
local lt = pile{{"lead white", 2}, {"pale smalt", 1}, {"raw umber", 0.4}, medium=0.08}
local r = brush{kind="round", width=1.0, point=1}
r:load(lt, 0.35)
r:stroke({{674.9,590},{674.3,600},{674.2,611}}, {pressure={0.35,0.25}})
r:stroke({{690.6,583},{690.3,592},{690.2,600}}, {pressure={0.3,0.2}})
-- the shawl's end hanging and a red ribbon at her bonnet
local sr = brush{kind="round", width=1.1, point=1}
sr:load(shawl, 0.5)
sr:stroke({{681,586},{681.5,592}}, {pressure={0.5,0.2}})
print(wait(0))

--@ chunk 106
print(wait(36*60)); print(drying(681,600), drying(900,560), drying(960,520))

--@ chunk 107
dress3 = pile{{"bone black", 1}, {"raw umber", 1.2}, {"red earth", 0.35}, {"smalt", 0.4}, medium=0.08}
work(wo_dress:shrink(0.4) + wo_body:shrink(0.3), {hand="detail", pile=dress3, angle=math.pi/2, coverage=3, length={2,5}, fill=true, clip=true, edge="found"})
local lt = pile{{"lead white", 1.5}, {"pale smalt", 1}, {"raw umber", 0.6}, {"red earth", 0.1}, medium=0.08}
local r = brush{kind="round", width=1.0, point=1}
r:load(lt, 0.35)
r:stroke({{675.3,592},{674.6,602},{674.4,611}}, {pressure={0.35,0.25}})
-- elder flowers: flat creamy umbels scattered over the upper, lit part of the bush
elderF = pile{{"lead white", 6}, {"yellow ochre", 0.35}, {"chrome yellow", 0.06}, medium=0.08}
elderFs = pile{{"lead white", 3}, {"yellow ochre", 0.4}, {"pale smalt", 0.5}, {"raw umber", 0.2}, medium=0.08}
local fb = brush{kind="round", width=1.4, point=0.5}
local count = 0
for i = 1, 70 do
  local x, y = rand(850, 1000), rand(478, 640)
  if elder2:at(x, y) > 0.9 and elder2:at(x, y - 6) > 0.5 then
    local s = 2.2 + (y - 470) / 60
    local lit = ((x - 960) * -0.7 + (y - 600) * -0.5) > -20
    fb:load(lit and elderF or elderFs, 0.5)
    for k = 1, 6 do
      local a = rand(0, 2*math.pi)
      local d = rand(0, 1) * s
      fb:touch(x + d*math.cos(a), y + d*math.sin(a)*0.45, {pressure=rand(0.35, 0.6)})
    end
    count = count + 1
  end
end
print(count, wait(0))

--@ chunk 108
eL = pile{{"green earth", 2.2}, {"yellow ochre", 1.4}, {"lead white", 1.1}, {"raw umber", 0.5}, {"Prussian blue", 0.12}, medium=0.1}
-- leaf sprays: pinnate, arching down and out, in layered tiers catching light on their tops
local sp = brush{kind="round", width=2.0, point=0.9}
local n = 0
for i = 1, 140 do
  local x, y = rand(850, 1000), rand(472, 610)
  if elder2:at(x, y) > 0.8 then
    local left = ((x - 960) * -0.7 + (y - 600) * -0.5)
    if left > -40 or rand(0,1) < 0.3 then
      local dir = (x < 930) and -1 or (rand(0,1) < 0.5 and -1 or 1)
      local L = rand(6, 11)
      local a = dir < 0 and (math.pi + rand(0.2, 0.7)) or rand(-0.7, -0.2) + 0.0
      a = dir < 0 and (math.pi - rand(-0.5, 0.3)) or rand(-0.3, 0.5)
      sp:load(eL, 0.45)
      local ex, ey = x + L*math.cos(a), y + L*math.sin(a) + 2
      sp:stroke({{x, y}, {(x+ex)/2, (y+ey)/2 - 1.5}, {ex, ey}}, {pressure={0.1, 0.02}, ramps={0.1,0.5}, clip=elder2})
      -- leaflets in pairs along it
      for k = 1, 3 do
        local u = k / 4
        local px, py = lerp(x, ex, u), lerp(y, ey, u) - 1
        sp:touch(px, py - 1.2, {pressure=0.55, drag={2.2, a - 0.7}})
        sp:touch(px, py + 1.2, {pressure=0.55, drag={2.2, a + 0.7}})
      end
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 109
local umb = nil
local cnt = 0
for i = 1, 200 do
  local x, y = rand(852, 1000), rand(476, 615)
  if elder2:at(x, y) > 0.95 and elder2:at(x, y - 5) > 0.6 and cnt < 34 then
    local s = 1.8 + (y - 470) / 55
    local e = ellipse(x, y, s, s * 0.5)
    local ok = true
    if umb and umb:at(x, y) > 0.1 then ok = false end
    if ok then umb = umb and (umb + e) or e cnt = cnt + 1 end
  end
end
umbels = umb
stipple(umb, {pile=elderFs, width=1.6, coverage=2.5, pressure={0.35,0.6}, clip=umb:grow(0.6), cluster=0.3})
local top = umb * mask(function(x,y) return 1 end) - umb:offset(0.6)
stipple(umb:shrink(0.3), {pile=elderF, width=1.3, coverage=1.6, pressure={0.35,0.55}, clip=umb:grow(0.4), cluster=0.4})
print(cnt, wait(0))

--@ chunk 110
wood = pile{{"raw umber", 2}, {"red earth", 0.4}, {"lead white", 0.6}, {"bone black", 0.2}, medium=0.08}
horseB = pile{{"red earth", 1.2}, {"raw umber", 1.5}, {"bone black", 0.25}, {"lead white", 0.2}, medium=0.08}
horseD = pile{{"raw umber", 1.5}, {"bone black", 0.8}, {"lead white", 0.2}, medium=0.08}
shirt = pile{{"lead white", 5}, {"yellow ochre", 0.3}, {"pale smalt", 0.4}, medium=0.08}
-- cast shadow of the whole group on the stubble
local cs = poly({{508,531},{600,530},{606,534},{512,535}}, true)
work(cs, {hand="detail", pile=hcC, angle=0, coverage=2, length={3,8}, fill=true, clip=true, edge="soft", load=0.5})
-- the load of hay
load_m = poly({{513,521},{511,512},{512,503},{516,497},{524,494},{536,493},{548,494},{556,497},{560,503},{561,512},{559,521}}, true):roughen(0.8, 3, 161)
work(load_m, {hand="detail", pile=hcS, angle=math.pi/2, coverage=2.5, length={2,6}, fill=true, clip=true, edge="firm"})
local lit = load_m * mask(function(x,y) return ((x - 536)/24 + (y - 507)/14*0.5) < -0.05 and 1 or 0 end)
work(lit, {hand="detail", pile=hcL, angle=-1.2, coverage=2.2, length={2,5}, fill=true, clip=load_m, edge="soft"})
-- wagon bed, ladder rail
bed = rect(514, 520.5, 45, 3.2)
work(bed, {hand="detail", pile=wood, angle=0, coverage=3, length={3,8}, fill=true, clip=true, edge="found"})
-- wheels
local wb = brush{kind="round", width=1.1, point=0.7}
local function wheel(cx, cy, r)
  wb:load(wood, 0.6)
  local pts = {}
  for k = 0, 24 do local a = k / 24 * 2 * math.pi pts[#pts+1] = {cx + r*math.cos(a), cy + r*math.sin(a)} end
  wb:stroke(pts, {pressure={0.6,0.6}, ramps={0.0,0.0}})
  wb:load(wood, 0.4)
  for k = 0, 5 do local a = k / 6 * math.pi + 0.2 wb:stroke({{cx - r*math.cos(a), cy - r*math.sin(a)}, {cx + r*math.cos(a), cy + r*math.sin(a)}}, {pressure={0.3,0.3}}) end
  wb:touch(cx, cy, {pressure=0.8})
end
wheel(523, 525.5, 6.2)
wheel(551, 527, 4.8)
print(wait(0))

--@ chunk 111
local function horse(dx, dy, p)
  local function T(pts) local o = {} for i, q in ipairs(pts) do o[i] = {q[1] + dx, q[2] + dy} end return o end
  local body = poly(T({{563,515},{565,513},{575,514},{583,511},{587,507},{590,503},{591,500.5},{592,502},{594,504},{597,510},{596,511.5},{593,511},{590,510},{588,514},{588,518},{583,521.5},{568,521.5},{563.5,519}}), true)
  local legs = ribbon(T({{565.5,519},{565.8,526},{565,532}}), 1.6) + ribbon(T({{569,520},{569.5,526},{570,532}}), 1.5)
             + ribbon(T({{584,520},{584.2,526},{584,532}}), 1.5) + ribbon(T({{587,519},{587.8,525},{588.5,532}}), 1.5)
  local tail = ribbon(T({{563.5,514.5},{561.8,519},{561.5,524}}), {1.4, 1.1, 0.8})
  local m = body + legs + tail
  work(m, {hand="detail", pile=p, angle=0, coverage=3, length={1.5,4}, fill=true, clip=true, edge="found"})
  return m
end
local hb = horse(3.5, -1.2, horseD)
local hf = horse(0, 0, horseB)
-- harness line, collar and the pole
local r = brush{kind="round", width=0.9, point=0.8}
r:load(horseD, 0.5)
r:stroke({{560,521},{575,518},{587,514}}, {pressure={0.4,0.4}})
r:stroke({{566,515},{585,511}}, {pressure={0.35,0.35}})
-- a light on the horse's back and rump from the upper left
local lt = pile{{"red earth", 1}, {"yellow ochre", 1}, {"lead white", 1.2}, medium=0.08}
r:load(lt, 0.4)
r:stroke({{564.5,514.5},{570,513.6},{577,514}}, {pressure={0.4,0.3}})
r:stroke({{584,510.5},{588,506},{590.5,502}}, {pressure={0.35,0.25}})
print(wait(0))

--@ chunk 112
trous = pile{{"raw umber", 1.5}, {"bone black", 0.4}, {"smalt", 0.4}, {"lead white", 0.4}, medium=0.08}
hatp = pile{{"yellow ochre", 1.5}, {"lead white", 1.5}, {"raw umber", 0.4}, medium=0.08}
local function worker(x, yb, s, lean)
  local h = 1.75 * s
  local function P(u, v) return {x + u * s + lean * v * s, yb - v * s} end
  local legs = poly({P(-0.2,0), P(-0.17,0.85), P(0.17,0.85), P(0.2,0), P(0.06,0), P(0.01,0.6), P(-0.05,0)})
  local torso = poly({P(-0.2,0.85), P(-0.21,1.3), P(-0.12,1.45), P(0.12,1.45), P(0.21,1.3), P(0.2,0.85)}, true)
  local hc = P(0, 1.6)
  local head = ellipse(hc[1], hc[2], 0.1*s, 0.12*s)
  local hat = ellipse(hc[1], hc[2] - 0.08*s, 0.2*s, 0.06*s) + ellipse(hc[1], hc[2] - 0.12*s, 0.1*s, 0.06*s)
  work(legs, {hand="detail", pile=trous, angle=math.pi/2, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
  work(torso, {hand="detail", pile=shirt, angle=math.pi/2, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
  work(head, {hand="detail", pile=skin, coverage=3, length={1,2}, fill=true, clip=true, edge="found"})
  work(hat, {hand="detail", pile=hatp, coverage=3, length={1,2}, fill=true, clip=true, edge="found"})
  return P
end
-- man pitching from the ground, left of the wagon
local P = worker(504, 533, 11.5, 0.12)
local r = brush{kind="round", width=0.9, point=0.8}
r:load(shirt, 0.5)
r:stroke({P(0.15,1.35), P(0.35,1.5), P(0.45,1.75)}, {pressure={0.55,0.5}})
r:load(wood, 0.5)
r:stroke({P(0.1,1.0), P(0.6,2.1), P(0.9,2.8)}, {pressure={0.35,0.3}})
local ft = P(0.9,2.8)
r:stroke({ft, {ft[1] + 1.8, ft[2] - 2.5}}, {pressure={0.25,0.1}})
r:stroke({ft, {ft[1] + 0.2, ft[2] - 3.0}}, {pressure={0.25,0.1}})
-- a forkful of hay on the tines
local hayf = ellipse(ft[1] + 1.2, ft[2] - 3.5, 3.2, 2)
work(hayf:roughen(0.5, 2, 171), {hand="detail", pile=hcL, coverage=3, length={1,3}, fill=true, clip=true, edge="soft"})
-- man on top of the load, receiving
local Q = worker(538, 494.5, 11.0, -0.05)
r:load(shirt, 0.5)
r:stroke({Q(-0.15,1.35), Q(-0.35,1.45), Q(-0.5,1.3)}, {pressure={0.55,0.5}})
print(wait(0))

--@ chunk 113
cowRed = pile{{"red earth", 1.6}, {"raw umber", 0.8}, {"yellow ochre", 0.4}, {"lead white", 0.5}, medium=0.08}
cowRedD = pile{{"red earth", 1}, {"raw umber", 1.4}, {"bone black", 0.2}, {"lead white", 0.2}, medium=0.08}
cowRedL = pile{{"red earth", 1}, {"yellow ochre", 1}, {"lead white", 1.4}, medium=0.08}
cowWh = pile{{"lead white", 5}, {"yellow ochre", 0.35}, {"pale smalt", 0.3}, medium=0.08}
cowWhD = pile{{"lead white", 2}, {"pale smalt", 0.8}, {"raw umber", 0.5}, medium=0.08}
grassSh = pile{{"green earth", 2.5}, {"raw umber", 0.9}, {"yellow ochre", 0.6}, {"lead white", 0.8}, {"Prussian blue", 0.08}, medium=0.1}
function cow2(x0, y0, s, d, grazing, cols)
  local function P(u, v) return {x0 + d * u * s, y0 - v * s} end
  local head
  local body = {P(-0.02,1.36), P(0.5,1.33), P(1.1,1.33), P(1.62,1.4), P(1.78,1.36)}
  if grazing then
    for _, q in ipairs({P(2.0,1.05), P(2.22,0.6), P(2.38,0.28), P(2.47,0.12), P(2.40,0.05), P(2.28,0.12), P(2.12,0.35), P(1.95,0.6), P(1.78,0.66)}) do body[#body+1] = q end
  else
    for _, q in ipairs({P(1.98,1.4), P(2.18,1.5), P(2.42,1.34), P(2.48,1.2), P(2.35,1.12), P(2.12,1.0), P(1.9,0.78), P(1.78,0.66)}) do body[#body+1] = q end
  end
  for _, q in ipairs({P(1.5,0.62), P(0.9,0.6), P(0.42,0.62), P(0.2,0.7), P(0.02,0.82), P(-0.1,1.05), P(-0.08,1.25)}) do body[#body+1] = q end
  local bm = poly(body, true)
  local lw = 0.14 * s
  local legs = ribbon({P(0.08,0.9), P(0.12,0.45), P(0.1,0.02)}, {lw*1.4, lw*0.8, lw*0.7})
             + ribbon({P(0.26,0.8), P(0.3,0.4), P(0.33,0.03)}, {lw*1.2, lw*0.8, lw*0.7})
             + ribbon({P(1.52,0.7), P(1.52,0.35), P(1.5,0.02)}, {lw*1.1, lw*0.8, lw*0.7})
             + ribbon({P(1.66,0.7), P(1.68,0.35), P(1.7,0.04)}, {lw*1.1, lw*0.8, lw*0.7})
  local tail = ribbon({P(-0.05,1.32), P(-0.12,0.95), P(-0.12,0.45)}, {0.07*s, 0.05*s, 0.05*s}) + ellipse(P(-0.12,0.38)[1], P(-0.12,0.38)[2], 0.06*s, 0.12*s)
  local ear
  if grazing then ear = ellipse(P(2.1,0.72)[1], P(2.1,0.72)[2], 0.1*s, 0.05*s) else ear = ellipse(P(2.12,1.42)[1], P(2.12,1.42)[2], 0.12*s, 0.05*s) end
  local all = bm + legs + tail + ear
  -- shadow on the grass, falling right (sun from the left)
  local sh = ellipse(P(1.1,0)[1] + 0.35*s, y0 + 0.05*s, 1.45*s, 0.13*s)
  work(sh - all, {hand="detail", pile=grassSh, angle=0, coverage=2.2, length={2,6}, fill=true, clip=true, edge="soft", load=0.55})
  work(all, {hand="detail", pile=cols[1], angle=0, coverage=3, length={2,5}, fill=true, clip=true, edge="found"})
  -- patches
  if cols.patch then
    local pm = (ellipse(P(0.7,1.0)[1], P(0.7,1.0)[2], 0.35*s, 0.28*s) + ellipse(P(1.35,1.1)[1], P(1.35,1.1)[2], 0.2*s, 0.2*s)):roughen(0.08*s, 0.2*s, math.floor(x0)) * bm
    work(pm, {hand="detail", pile=cols.patch, coverage=3, length={1,3}, fill=true, clip=true, edge="firm"})
  end
  -- belly and far legs shadow
  local low = all * mask(function(px, py) return (py > y0 - 0.78*s) and 1 or 0 end)
  work(low, {hand="detail", pile=cols[2], angle=0, coverage=2, length={1.5,4}, fill=true, clip=all, edge="soft", load=0.6})
  -- light along the back and rump
  local r = brush{kind="round", width=math.max(0.9, 0.08*s), point=0.8}
  r:load(cols[3], 0.5)
  r:stroke({P(-0.04,1.2), P(0.02,1.32), P(0.5,1.31), P(1.1,1.31), P(1.6,1.36)}, {pressure={0.5,0.3}, ramps={0.1,0.3}})
  -- horn
  local hb = brush{kind="round", width=0.8, point=1}
  hb:load(cowWh, 0.4)
  if grazing then hb:stroke({P(2.2,0.62), P(2.3,0.7)}, {pressure={0.5,0.1}}) else hb:stroke({P(2.22,1.5), P(2.28,1.62)}, {pressure={0.5,0.1}}) end
  return all
end
cows = {}
cows[1] = cow2(262, 562, 18.0, 1, true, {cowRed, cowRedD, cowRedL})
cows[2] = cow2(342, 549, 16.6, -1, false, {cowWh, cowWhD, cowWh, patch=cowRedD})
cows[3] = cow2(52, 533, 14.0, 1, true, {cowRedD, cowRedD, cowRed})
print(wait(0))

--@ chunk 114
print(drying(100,650), drying(300,555))
woodL = pile{{"lead white", 2.5}, {"raw umber", 1}, {"pale smalt", 0.6}, {"yellow ochre", 0.3}, medium=0.08}
woodD = pile{{"raw umber", 1.6}, {"bone black", 0.35}, {"lead white", 0.5}, {"pale smalt", 0.3}, medium=0.08}
posts = {}
local base = {{28,708},{96,676},{156,652},{206,636},{247,624},{280,616},{306,610}}
for i, b in ipairs(base) do
  local t = (i - 1) / (#base - 1)
  local h = lerp(62, 17, t^0.8)
  local w = lerp(7, 2.2, t^0.8)
  posts[i] = {x=b[1], y=b[2], h=h, w=w}
end
local fm
for i, p in ipairs(posts) do
  local lean = (i % 2 == 0) and 0.06 or -0.04
  local m = poly({{p.x - p.w/2, p.y}, {p.x - p.w/2 + lean*p.h, p.y - p.h}, {p.x + p.w/2 + lean*p.h, p.y - p.h - p.w*0.25}, {p.x + p.w/2, p.y}})
  p.lean = lean
  fm = fm and (fm + m) or m
end
-- rails between successive posts at 0.45 and 0.85 of height
local rails
for i = 1, #posts - 1 do
  local a, b = posts[i], posts[i+1]
  for _, f in ipairs({0.42, 0.82}) do
    local p1 = {a.x + a.lean*a.h*f, a.y - a.h*f}
    local p2 = {b.x + b.lean*b.h*f, b.y - b.h*f}
    local sag = {(p1[1]+p2[1])/2, (p1[2]+p2[2])/2 + 1.2}
    local r = ribbon({p1, sag, p2}, {a.w*0.5, (a.w+b.w)*0.22, b.w*0.5})
    rails = rails and (rails + r) or r
  end
end
fence = fm + rails
work(fence, {hand="detail", pile=woodD, angle=0, coverage=3, length={2,6}, fill=true, clip=true, edge="found"})
-- light on the upper/left faces
local lit = fence * mask(function(x,y) return 1 end) - fence:offset(1.2)
local litL = fence - fence:shrink(1.2)
print(wait(0))

--@ chunk 115
birdp = pile{{"raw umber", 1}, {"bone black", 0.6}, {"pale smalt", 0.8}, {"lead white", 1.0}, medium=0.1}
local r = brush{kind="rigger", width=1.1, point=1}
local birds = {{548,318,3.2,0.1},{566,309,2.6,-0.15},{579,322,2.4,0.2},{597,313,2.0,0.0},{611,327,1.8,-0.1},{529,331,2.2,0.05}}
for i, b in ipairs(birds) do
  r:load(birdp, 0.45)
  local x, y, s, t = b[1], b[2], b[3], b[4]
  r:stroke({{x - s, y - s*0.35 + t*s}, {x - s*0.45, y - s*0.45}, {x, y}}, {pressure={0.1, 0.55}, ramps={0.4,0.1}})
  r:stroke({{x, y}, {x + s*0.45, y - s*0.5}, {x + s, y - s*0.3 - t*s}}, {pressure={0.55, 0.1}, ramps={0.1,0.4}})
end
-- two swallows closer, higher, with forked tails
local sw = brush{kind="round", width=1.4, point=1}
for _, b in ipairs({{770,228,7,1},{812,246,5.5,-1}}) do
  local x, y, s, d = b[1], b[2], b[3], b[4]
  sw:load(birdp, 0.5)
  sw:stroke({{x - s, y + s*0.25}, {x - s*0.45, y - s*0.15}, {x, y}}, {pressure={0.05, 0.6}, ramps={0.5,0.1}})
  sw:stroke({{x, y}, {x + s*0.5, y - s*0.1}, {x + s*1.05, y + s*0.35}}, {pressure={0.6, 0.05}, ramps={0.1,0.5}})
  sw:touch(x, y + 0.3, {pressure=0.6})
  sw:stroke({{x, y + 0.5}, {x - s*0.1*d, y + s*0.45}}, {pressure={0.4,0.05}})
  sw:stroke({{x, y + 0.5}, {x + s*0.18*d, y + s*0.42}}, {pressure={0.4,0.05}})
end
print(wait(0))

--@ chunk 116
print(drying(200,680), drying(700,680))
butter = pile{{"chrome yellow", 2}, {"lead white", 1}, {"yellow ochre", 0.5}, medium=0.08}
daisy = pile{{"lead white", 6}, {"yellow ochre", 0.15}, medium=0.08}
clover = pile{{"vermilion", 0.6}, {"red earth", 0.4}, {"lead white", 2}, {"smalt", 0.3}, medium=0.08}
poppy = pile{{"vermilion", 2}, {"red earth", 0.3}, medium=0.08}
stemG = pile{{"green earth", 2}, {"yellow ochre", 1}, {"raw umber", 0.5}, {"lead white", 0.4}, medium=0.08}
local fl = brush{kind="round", width=1.6, point=0.4}
local st = brush{kind="rigger", width=1.0, point=1}
local ok = fg_grass - fence:grow(1) - elder2:grow(2)
local kinds = {{butter, 0.45}, {daisy, 0.25}, {clover, 0.2}, {poppy, 0.1}}
local n = 0
for i = 1, 420 do
  local x, y = rand(0, 1000), rand(600, 714)
  local d = crest_depth(x, y)
  if d > 6 and ok:at(x, y) > 0.9 then
    local near = clamp(d / 110, 0, 1)
    if rand(0,1) < 0.35 + 0.65*near then
      local u = rand(0,1)
      local k = u < 0.45 and 1 or (u < 0.7 and 2 or (u < 0.9 and 3 or 4))
      local s = lerp(0.8, 3.4, near) * rand(0.8, 1.2)
      if near > 0.3 then
        st:load(stemG, 0.4)
        st:stroke({{x + rand(-1,1), y + s*3}, {x, y}}, {pressure={0.5, 0.15}})
      end
      fl:load(kinds[k][1], 0.5)
      if k == 2 and s > 2 then
        for p = 0, 7 do local a = p/8*2*math.pi fl:touch(x + s*0.55*math.cos(a), y + s*0.3*math.sin(a), {pressure=0.25}) end
        fl:load(butter, 0.4)
        fl:touch(x, y, {pressure=0.25})
      else
        fl:touch(x, y, {pressure=clamp(0.15 + s*0.12, 0.15, 0.6)})
        if s > 2.2 then fl:touch(x + s*0.3, y - s*0.15, {pressure=0.3}) end
      end
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 117
print(wait(3*24*60)); print(drying(200,680), drying(700,680), drying(100,660), drying(300,555))

--@ chunk 118
local ok = fg_grass - fence:grow(1.5) - elder2:grow(2)
local n = 0
local tipsL = {}
for i = 1, 520 do
  local x, y = rand(0, 1000), rand(600, 714)
  local d = crest_depth(x, y)
  if d > 5 and ok:at(x, y) > 0.9 then
    local near = clamp(d / 110, 0, 1)
    -- clustered: denser in drifts
    local drift = 0.5 + 0.5 * math.sin(x / 47 + y / 23) * math.sin(x / 91 - y / 37)
    if rand(0,1) < (0.25 + 0.75*near) * (0.3 + 0.7*drift) then
      local u = rand(0,1)
      local k = u < 0.5 and 1 or (u < 0.72 and 2 or (u < 0.9 and 3 or 4))
      local s = lerp(1.0, 4.2, near^1.2) * rand(0.8, 1.2)
      local p = ({butter, daisy, clover, poppy})[k]
      if near > 0.35 then
        local st = brush{kind="rigger", width=1.0 + near*0.6, point=1}
        st:load(stemG, 0.45)
        st:stroke({{x + rand(-2,2), y + s*4}, {x + rand(-0.6,0.6), y + s*1.5}, {x, y}}, {pressure={0.55, 0.2}})
      end
      local fl = brush{kind="round", width=math.max(1.2, s), point=0.2}
      fl:load(p, 0.6)
      if k == 2 and s > 2.2 then
        local pb = brush{kind="round", width=math.max(1.0, s*0.4), point=0.3}
        pb:load(daisy, 0.6)
        for q = 0, 9 do local a = q/10*2*math.pi pb:touch(x + s*0.45*math.cos(a), y + s*0.25*math.sin(a), {pressure=0.5, drag={s*0.25, a}}) end
        pb:load(butter, 0.5)
        pb:touch(x, y, {pressure=0.6})
      elseif k == 4 and s > 2 then
        fl:touch(x, y, {pressure=0.6})
        fl:touch(x + s*0.25, y - s*0.1, {pressure=0.5})
        local c = brush{kind="round", width=math.max(0.9, s*0.3), point=0.5}
        c:load(blackp, 0.4)
        c:touch(x + s*0.1, y - s*0.05, {pressure=0.4})
      else
        fl:touch(x, y, {pressure=0.5})
        if k == 3 then fl:touch(x, y - s*0.25, {pressure=0.35}) end
      end
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 119
local ok = fg_grass - fence:grow(2) - elder2:grow(3)
local n = 0
for i = 1, 200 do
  local x, y = rand(0, 1000), rand(655, 712)
  local d = crest_depth(x, y)
  local drift = 0.5 + 0.5 * math.sin(x / 47 + y / 23) * math.sin(x / 91 - y / 37)
  if ok:at(x, y) > 0.9 and rand(0,1) < 0.25 + 0.6 * drift and n < 38 then
    local near = clamp((y - 650) / 60, 0, 1)
    local s = lerp(4, 7.5, near) * rand(0.85, 1.15)
    local u = rand(0,1)
    local st = brush{kind="rigger", width=1.5, point=1}
    st:load(stemG, 0.55)
    local sx = x + rand(-3, 3)
    st:stroke({{sx, 716}, {lerp(sx, x, 0.6) + rand(-1.5,1.5), (y + 716)/2}, {x, y}}, {pressure={0.6, 0.25}})
    if u < 0.4 then
      -- ox-eye daisy: white rays, yellow disc, rays on the shaded side greyer
      local pb = brush{kind="round", width=s*0.33, point=0.6}
      for q = 0, 11 do
        local a = q/12*2*math.pi + rand(-0.1,0.1)
        pb:load((math.cos(a) > 0.3) and elderFs or daisy, 0.6)
        pb:stroke({{x + s*0.12*math.cos(a), y + s*0.07*math.sin(a)}, {x + s*0.5*math.cos(a), y + s*0.3*math.sin(a)}}, {pressure={0.6, 0.2}})
      end
      local c = brush{kind="round", width=s*0.3, point=0.2}
      c:load(butter, 0.6)
      c:touch(x, y, {pressure=0.7})
    elseif u < 0.75 then
      -- buttercups: two or three cups on branching stalks
      local c = brush{kind="round", width=s*0.4, point=0.2}
      for q = 1, 3 do
        local bx, by = x + rand(-s*0.8, s*0.8), y + rand(-s*0.6, s*0.2)
        st:load(stemG, 0.4)
        st:stroke({{x, y + s*0.8}, {bx, by}}, {pressure={0.4,0.2}})
        c:load(butter, 0.6)
        c:touch(bx, by, {pressure=0.7})
        c:load(hcL, 0.3)
        c:touch(bx - s*0.08, by - s*0.08, {pressure=0.3})
      end
    else
      -- red clover heads / a poppy
      local c = brush{kind="round", width=s*0.45, point=0.2}
      if u < 0.9 then
        c:load(clover, 0.6); c:touch(x, y, {pressure=0.7}); c:touch(x, y - s*0.15, {pressure=0.5})
      else
        c:load(poppy, 0.7); c:touch(x, y, {pressure=0.8}); c:touch(x + s*0.2, y - s*0.1, {pressure=0.7})
        local k = brush{kind="round", width=s*0.14, point=0.4}; k:load(blackp, 0.4); k:touch(x + s*0.1, y - s*0.05, {pressure=0.6})
      end
    end
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 120
print(wait(2*24*60)); print(drying(62,660), drying(300,555), drying(681,600))

--@ chunk 121
-- fence: lit left faces of the posts, lit top edges of the rails, grain, and grass growing up in front of the post feet
local r = brush{kind="round", width=1.2, point=0.8}
for i, p in ipairs(posts) do
  local w = p.w
  r = brush{kind="round", width=math.max(0.9, w*0.35), point=0.6}
  r:load(woodL, 0.5)
  r:stroke({{p.x - w*0.3, p.y - 1}, {p.x - w*0.3 + p.lean*p.h, p.y - p.h + 1}}, {pressure={0.5, 0.4}, shake=0.5})
  -- the cut top catching light
  r:touch(p.x + p.lean*p.h, p.y - p.h - w*0.1, {pressure=0.4})
  if w > 3.5 then
    local g = brush{kind="rigger", width=0.8, point=1}
    g:load(woodD, 0.4)
    for k = 1, 2 do
      local off = w*(0.05 + 0.15*k)
      g:stroke({{p.x + off, p.y - p.h*0.1}, {p.x + off + p.lean*p.h*0.8, p.y - p.h*0.85}}, {pressure={0.3, 0.1}, shake=0.8})
    end
  end
end
for i = 1, #posts - 1 do
  local a, b = posts[i], posts[i+1]
  for _, f in ipairs({0.42, 0.82}) do
    local p1 = {a.x + a.lean*a.h*f, a.y - a.h*f - a.w*0.2}
    local p2 = {b.x + b.lean*b.h*f, b.y - b.h*f - b.w*0.2}
    local sag = {(p1[1]+p2[1])/2, (p1[2]+p2[2])/2 + 1.2}
    local rb = brush{kind="round", width=math.max(0.8, a.w*0.22), point=0.5}
    rb:load(woodL, 0.45)
    rb:stroke({p1, sag, p2}, {pressure={0.45, 0.3}, shake=0.4})
  end
end
-- grass in front of fence feet
for i, p in ipairs(posts) do
  local g = brush{kind="rigger", width=1.2 + p.w*0.12, point=1}
  for k = 1, 9 do
    g:load((k % 3 == 0) and grL or grM, 0.5)
    local x = p.x + rand(-p.w*1.6, p.w*1.6)
    local L = p.h * rand(0.18, 0.4)
    g:stroke({{x, p.y + 2}, {x + rand(-2,2), p.y - L*0.5}, {x + rand(-3,3), p.y - L}}, {pressure={0.7, 0.05}, ramps={0.05, 0.7}})
  end
end
print(wait(0))

--@ chunk 122
print(drying(628,490), drying(628,510))
wSh2 = pile{{"green earth", 2.4}, {"raw umber", 1.0}, {"lead white", 1.1}, {"pale smalt", 0.7}, {"Prussian blue", 0.08}, {"yellow ochre", 0.3}, medium=0.1}
wMid2 = pile{{"lead white", 1.8}, {"green earth", 2.2}, {"yellow ochre", 0.9}, {"raw umber", 0.4}, {"pale smalt", 0.4}, medium=0.1}
trunkG = pile{{"lead white", 1.6}, {"raw umber", 1.2}, {"pale smalt", 0.8}, {"green earth", 0.4}, medium=0.1}
for i, w in ipairs(willows) do
  if i > 1 then
    local hm = w.head
    local kx, ky, H, Wd = w.kx, w.ky, w.H, w.Wd
    -- the lower, inner part of the head in shade; right side too
    local sh = hm * mask(function(px, py) local u = (px - kx)/Wd*0.8 + (py - (ky - H*0.5))/H*1.1 return u > 0.0 and 1 or 0 end)
    stipple(sh, {pile=wSh2, width=2.0, coverage=2.2, pressure={0.4,0.65}, clip=hm, cluster={0.5, 5}, feather=0.4})
    -- mid tone clumps across the lit side so it isn't cotton
    local md = hm * mask(function(px, py) local u = (px - kx)/Wd*0.8 + (py - (ky - H*0.5))/H*1.1 return (u > -0.35 and u <= 0.1) and 1 or 0 end)
    stipple(md, {pile=wMid2, width=1.8, coverage=1.4, pressure={0.35,0.6}, clip=hm, cluster={0.6, 5}, feather=0.5})
    -- trunk re-greyed
    local tw = 0.45 * w.s
    local tb = brush{kind="round", width=math.max(1.4, tw*0.45), point=0.3}
    tb:load(trunkG, 0.6)
    tb:stroke({{w.x - tw*0.2, w.yb - 0.5}, {w.x - tw*0.15 + w.lean, w.top + 2}}, {pressure={0.6, 0.5}})
  end
end
print(wait(0))

--@ chunk 123
wSh3 = pile{{"green earth", 2.4}, {"raw umber", 1.3}, {"lead white", 0.6}, {"pale smalt", 0.6}, {"Prussian blue", 0.15}, {"yellow ochre", 0.3}, medium=0.08}
for i, w in ipairs(willows) do
  if i > 1 then
    local hm = w.head
    local kx, ky, H, Wd = w.kx, w.ky, w.H, w.Wd
    local sh = hm * mask(function(px, py) local u = (px - kx)/Wd*0.8 + (py - (ky - H*0.5))/H*1.1 return u > 0.05 and 1 or 0 end)
    stipple(sh, {pile=wSh3, width=2.6, coverage=3.0, pressure={0.55,0.8}, clip=hm, cluster={0.4, 4}, feather=0.3, dips={8, 0.8, 0.5}})
  end
end
print(wait(0))

--@ chunk 124
wMid3 = pile{{"lead white", 1.2}, {"green earth", 2.2}, {"yellow ochre", 0.7}, {"raw umber", 0.6}, {"pale smalt", 0.5}, medium=0.08}
for i, w in ipairs(willows) do
  if i > 1 then
    local hm = w.head
    local kx, ky, H, Wd = w.kx, w.ky, w.H, w.Wd
    local band = hm * mask(function(px, py) local u = (px - kx)/Wd*0.8 + (py - (ky - H*0.5))/H*1.1 return math.abs(u - 0.05) < 0.22 and 1 or 0 end)
    stipple(band, {pile=wMid3, width=2.2, coverage=1.6, pressure={0.45,0.7}, clip=hm, cluster={0.7, 4}, feather=0.4, dips={8, 0.7, 0.5}})
    -- a few dark sprays poking into the light side and light sprays into the shadow
    local sp = brush{kind="round", width=1.3, point=0.9}
    for k = 1, 5 do
      local a = -math.pi/2 + rand(-0.8, 0.2)
      local L = H * rand(0.35, 0.7)
      sp:load(wSh3, 0.5)
      sp:stroke({{kx + L*0.45*math.cos(a), ky + L*0.45*math.sin(a)}, {kx + L*math.cos(a), ky + L*math.sin(a)}}, {pressure={0.6, 0.1}, clip=hm})
    end
  end
end
print(wait(0))

--@ chunk 125
stub = pile{{"yellow ochre", 1.6}, {"raw umber", 0.7}, {"lead white", 1.4}, {"green earth", 0.6}, medium=0.08}
stubL = pile{{"lead white", 3}, {"yellow ochre", 1.6}, {"chrome yellow", 0.15}, medium=0.08}
local hay = fieldB - river2:grow(1.5) - fore_m - figs:grow(1) - elder2:grow(1)
local hc
for i, h in ipairs(haycocks) do local e = ellipse(h.x + h.w*0.1, h.y - h.h*0.4, h.w*0.8, h.h*0.75) hc = hc and (hc + e) or e end
hay = hay - hc - load_m:grow(3) - rect(500, 490, 100, 46)
work(hay, {hand="hatch", pile=stub, tool={kind="round", width=1.0, point=0.9}, angle=function(x,y) return -0.02 + rand(-0.05,0.05) end, length={3,9}, coverage=0.25, pressure={0.3,0.5}, ramps={0.3,0.3}, clip=hay})
work(hay, {hand="hatch", pile=stubL, tool={kind="round", width=1.0, point=0.9}, angle=-0.02, length={4,10}, coverage=0.15, pressure={0.3,0.5}, ramps={0.3,0.3}, clip=hay})
print(wait(0))

--@ chunk 126
print(drying(681,600), drying(697,590), drying(280,550))
bonnet = pile{{"raw umber", 1.5}, {"bone black", 0.6}, {"smalt", 0.3}, {"lead white", 0.35}, medium=0.08}
local wx = 681
local bm = ellipse(wx, 576.8, 3.1, 3.2) + ellipse(wx, 579.6, 2.4, 1.2)
work(bm, {hand="detail", pile=bonnet, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
-- red shawl across the shoulders and down the back as a triangle
local sh = poly({{wx-4.6,581.8},{wx+4.6,581.8},{wx+3.9,584.5},{wx+0.3,591.5},{wx-3.9,584.5}}, true)
work(sh, {hand="detail", pile=shawl, angle=math.pi/2, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
-- a light edge on the bonnet brim and the shawl's lit left side
local r = brush{kind="round", width=0.8, point=1}
r:load(pile{{"vermilion", 1}, {"lead white", 1}, {"yellow ochre", 0.3}}, 0.35)
r:stroke({{wx-4.4,582.3},{wx-3.6,585},{wx-1.5,588.5}}, {pressure={0.4,0.2}})
-- cast shadows of the pair falling right along the crest
local cs = ellipse(708, 612.6, 14, 1.6) + ellipse(690, 612.8, 12, 1.5)
work(cs - figs, {hand="detail", pile=sD, angle=0, coverage=2.2, length={2,6}, fill=true, clip=true, edge="soft", load=0.6})
-- a few grass blades in front of their feet
local g = brush{kind="rigger", width=1.0, point=1}
for k = 1, 16 do
  g:load((k % 3 == 0) and grM or grD, 0.5)
  local x = rand(670, 706)
  g:stroke({{x, 615}, {x + rand(-1, 1), 612 - rand(0, 2)}, {x + rand(-1.5, 1.5), 608 - rand(0, 3)}}, {pressure={0.6, 0.05}, ramps={0.05,0.7}})
end
print(wait(0))

--@ chunk 127
cowShade = pile{{"red earth", 0.8}, {"raw umber", 1.4}, {"green earth", 0.4}, {"bone black", 0.15}, {"lead white", 0.3}, medium=0.08}
local c1 = cows[1] * mask(function(x,y) return (y > 562 - 18*0.95 or x > 262 + 18*1.7) and 1 or 0 end)
local c3 = cows[3] * mask(function(x,y) return (y > 533 - 14*0.95 or x > 52 + 14*1.7) and 1 or 0 end)
stipple(c1, {pile=cowShade, width=1.6, coverage=2.5, pressure={0.4,0.65}, clip=cows[1], cluster=0.3})
stipple(c3, {pile=cowShade, width=1.4, coverage=2.5, pressure={0.4,0.65}, clip=cows[3], cluster=0.3})
local c2 = cows[2] * mask(function(x,y) return (y > 549 - 16.6*0.9 or x < 342 - 16.6*1.9) and 1 or 0 end)
stipple(c2, {pile=cowWhD, width=1.4, coverage=1.8, pressure={0.35,0.6}, clip=cows[2], cluster=0.3})
-- grass over the hooves
local g = brush{kind="rigger", width=0.9, point=1}
for _, c in ipairs({{262,562,18,1},{342,549,16.6,-1},{52,533,14,1}}) do
  for k = 1, 12 do
    g:load((k % 2 == 0) and gG or grM, 0.45)
    local x = c[1] + c[4] * rand(-0.1, 1.9) * c[3]
    g:stroke({{x, c[2] + 1.2}, {x + rand(-0.6, 0.6), c[2] - rand(1.2, 2.6)}}, {pressure={0.5, 0.05}})
  end
end
print(wait(0))

--@ chunk 128
pastD = pile{{"green earth", 2.5}, {"yellow ochre", 1.0}, {"raw umber", 0.7}, {"lead white", 0.9}, {"Prussian blue", 0.06}, medium=0.08}
pastL = pile{{"lead white", 2}, {"yellow ochre", 1.6}, {"green earth", 1.4}, {"chrome yellow", 0.1}, medium=0.08}
local cm = cows[1]:grow(3) + cows[2]:grow(3) + cows[3]:grow(3)
pasture = (fieldA + fieldC) * below({{-10,478},{1010,478}}) - river2:grow(1.5) - cm - fore_m - figs:grow(2) - fieldB:grow(1) - elder2:grow(1) - hedgerow:grow(1) - rect(495,470,110,70)
for i, w in ipairs(willows) do pasture = pasture - rect(w.x - 5, w.top - 3, 10, w.yb - w.top + 5) end
local function tuft_len(y) local s = (y - 438) * 0.12 return {math.max(1.5, s*0.25), math.max(3, s*0.55)} end
local far = pasture * mask(function(x,y) return y < 530 and 1 or 0 end)
local near = pasture * mask(function(x,y) return y >= 530 and 1 or 0 end)
work(far, {hand="hatch", pile=pastD, tool={kind="round", width=1.0, point=0.9}, angle=-math.pi/2 + 0.1, length={1.5,3.5}, coverage=0.25, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=pasture, clump=0.6})
work(near, {hand="hatch", pile=pastD, tool={kind="round", width=1.2, point=0.9}, angle=-math.pi/2 + 0.1, length={3,7}, coverage=0.3, pressure={0.35,0.6}, ramps={0.1,0.6}, clip=pasture, clump=0.6})
work(near, {hand="hatch", pile=pastL, tool={kind="round", width=1.0, point=0.9}, angle=-math.pi/2 - 0.1, length={2,5}, coverage=0.15, pressure={0.3,0.5}, ramps={0.1,0.6}, clip=pasture, clump=0.6})
print(wait(0))

--@ chunk 129
grDD = pile{{"green earth", 2}, {"raw umber", 2}, {"Prussian blue", 0.35}, {"bone black", 0.2}, {"yellow ochre", 0.4}, medium=0.08}
local ok = fore_m - path_m:shrink(3) - elder2:grow(1) - fence:grow(1)
local bottom = ok * mask(function(x,y) local d = crest_depth(x,y) return d > 75 and 1 or 0 end)
local cornerw = function(x,y) return 1 end
local up = function(x,y) return -math.pi/2 + 0.35*math.sin(x/29 + y/17) end
work(bottom, {hand="hatch", pile=grDD, tool={kind="rigger", width=2.4, point=1}, angle=up, length={16,34}, coverage=0.7, pressure={0.55,0.85}, ramps={0.05,0.75}, clip=ok, clump=0.5})
work(bottom, {hand="hatch", pile=grM, tool={kind="rigger", width=2.0, point=1}, angle=up, length={14,28}, coverage=0.3, pressure={0.5,0.8}, ramps={0.05,0.75}, clip=ok, clump=0.5})
-- a few long seeding grass stems with heads, catching the light
local st = brush{kind="rigger", width=1.2, point=1}
local hd = brush{kind="round", width=2.0, point=0.9}
for k = 1, 26 do
  local x = rand(0, 1000)
  local y0 = 716
  if ok:at(x, 700) > 0.9 then
    local L = rand(40, 75)
    local lean = rand(-0.25, 0.25)
    local tx, ty = x + L*lean, y0 - L
    st:load(stemG, 0.5)
    st:stroke({{x, y0}, {x + L*lean*0.4, y0 - L*0.5}, {tx, ty}}, {pressure={0.55, 0.3}})
    hd:load((k % 3 == 0) and pastL or stub, 0.5)
    hd:stroke({{tx, ty + 7}, {tx + lean*3, ty}}, {pressure={0.7, 0.1}})
  end
end
print(wait(0))

--@ chunk 130
print(wait(3*24*60)); print(drying(900,680), drying(500,700), drying(300,540))

--@ chunk 131
eDD = pile{{"green earth", 2}, {"raw umber", 1.8}, {"Prussian blue", 0.4}, {"bone black", 0.25}, {"yellow ochre", 0.3}, medium=0.08}
local o = outline{{1004,716},{836,716,"c"},{840,700},{846,684},{843,668},{850,652},{847,636},{856,618},{862,600},{870,586},{1004,580,"c"}, char="soft", seed=181, lobe=8, amount=1.2, closed=true}
lowE = (o:mask() * elder2:grow(6)) * rect(0, 585, 1000, 140)
lowE = lowE * mask(function(x,y) return 1 end)
stipple(lowE, {pile=eDD, width=3.2, coverage=4.5, pressure={0.5,0.8}, clip=lowE, cluster=0.25})
-- blend the join with the upper bush by stippling eD across the seam
local seam = elder2 * rect(840, 575, 170, 30)
stipple(seam, {pile=eD, width=2.8, coverage=2.0, pressure={0.45,0.75}, clip=elder2, cluster=0.4})
print(wait(0))

--@ chunk 132
print(wait(36*60)); print(drying(920,650), drying(920,690))

--@ chunk 133
eMs = pile{{"green earth", 2.4}, {"raw umber", 1.2}, {"Prussian blue", 0.25}, {"yellow ochre", 0.6}, {"lead white", 0.35}, medium=0.08}
local sp = brush{kind="round", width=2.4, point=0.9}
local n = 0
-- lower, shaded leaf sprays: bigger toward the viewer, arching out and down
for i = 1, 260 do
  local x, y = rand(840, 1000), rand(585, 712)
  if lowE:at(x, y) > 0.8 and n < 110 then
    local s = 1 + (y - 585) / 110
    local lit = (x < 900 and rand(0,1) < 0.6) or rand(0,1) < 0.25
    local p = lit and eL or eMs
    local dir = (x < 925) and -1 or (rand(0,1) < 0.6 and 1 or -1)
    local a = dir < 0 and (math.pi - rand(-0.6, 0.2)) or rand(-0.2, 0.6)
    local L = rand(8, 14) * s
    local ex, ey = x + L*math.cos(a), y + L*math.sin(a) + 3*s
    sp = brush{kind="round", width=1.6 + s*0.8, point=0.9}
    sp:load(p, 0.5)
    sp:stroke({{x, y}, {(x+ex)/2, (y+ey)/2 - 2*s}, {ex, ey}}, {pressure={0.1, 0.02}, ramps={0.1,0.5}})
    for k = 1, 4 do
      local u = k / 5
      local px, py = lerp(x, ex, u), lerp(y, ey, u) - 1.5*s*(1 - math.abs(u - 0.5))
      sp:touch(px, py - 1.4*s, {pressure=0.55, drag={2.6*s, a - 0.75}})
      sp:touch(px, py + 1.4*s, {pressure=0.55, drag={2.6*s, a + 0.75}})
    end
    sp:touch(ex, ey, {pressure=0.5, drag={2.4*s, a}})
    n = n + 1
  end
end
-- shaded umbels in the lower bush, cooler and greyer
local fb = brush{kind="round", width=1.8, point=0.4}
local m = 0
for i = 1, 200 do
  local x, y = rand(850, 1000), rand(590, 680)
  if lowE:at(x, y) > 0.95 and m < 14 then
    local s = 2.8 + (y - 585) / 30
    for k = 1, 10 do
      local a = rand(0, 2*math.pi)
      local d = rand(0, 1)^0.6 * s
      fb:load(elderFs, 0.5)
      fb:touch(x + d*math.cos(a), y + d*math.sin(a)*0.45, {pressure=rand(0.35, 0.6)})
    end
    m = m + 1
  end
end
print(n, m, wait(0))

--@ chunk 134
-- lit sprays across the seam on the sunward side, and a few bigger flower heads in half shade
local n = 0
for i = 1, 200 do
  local x, y = rand(842, 1000), rand(570, 640)
  if elder2:grow(4):at(x, y) + lowE:at(x, y) > 0.8 and n < 40 then
    local s = 1 + (y - 570) / 120
    local lit = x < 930 or rand(0,1) < 0.3
    local a = (x < 925) and (math.pi - rand(-0.5, 0.25)) or rand(-0.25, 0.5)
    local L = rand(8, 13) * s
    local ex, ey = x + L*math.cos(a), y + L*math.sin(a) + 3*s
    local sp = brush{kind="round", width=1.6 + s*0.6, point=0.9}
    sp:load(lit and eL or eM, 0.5)
    sp:stroke({{x, y}, {(x+ex)/2, (y+ey)/2 - 2*s}, {ex, ey}}, {pressure={0.1, 0.02}, ramps={0.1,0.5}})
    for k = 1, 4 do
      local u = k / 5
      local px, py = lerp(x, ex, u), lerp(y, ey, u) - 1.5*s*(1 - math.abs(u - 0.5))
      sp:touch(px, py - 1.3*s, {pressure=0.5, drag={2.4*s, a - 0.75}})
      sp:touch(px, py + 1.3*s, {pressure=0.5, drag={2.4*s, a + 0.75}})
    end
    n = n + 1
  end
end
local umb2
local m = 0
for i = 1, 300 do
  local x, y = rand(850, 990), rand(590, 665)
  if lowE:at(x, y) > 0.95 and m < 9 and (not umb2 or umb2:grow(10):at(x, y) < 0.1) then
    local s = 4.5 + (y - 585) / 22
    local e = ellipse(x, y, s, s*0.45):roughen(0.8, 2.5, 190 + m)
    umb2 = umb2 and (umb2 + e) or e
    m = m + 1
  end
end
stipple(umb2, {pile=elderFs, width=1.7, coverage=2.2, pressure={0.35,0.6}, clip=umb2:grow(0.8), cluster=0.4})
stipple(umb2 * mask(function(x,y) return 1 end):shrink(0.5), {pile=daisy, width=1.3, coverage=0.8, pressure={0.3,0.5}, clip=umb2, cluster=0.5})
print(n, m, wait(0))

--@ chunk 135
local seam = (elder2 + lowE) * mask(function(x,y) return math.max(0, 1 - math.abs(y - 586)/16) end)
stipple(seam, {pile=eD, width=2.6, coverage=1.6, pressure={0.4,0.7}, clip=(elder2 + lowE), cluster={0.7, 8}, feather=0.6})
stipple(seam * rect(830, 560, 90, 60), {pile=eM, width=2.2, coverage=0.8, pressure={0.4,0.6}, clip=(elder2 + lowE), cluster={0.7, 6}, feather=0.6, drag={3, 2.8}})
-- tall grass rising in front of the bush foot
local up = function(x,y) return -math.pi/2 + 0.3*math.sin(x/21 + y/13) end
local foot = rect(830, 655, 175, 62)
work(foot, {hand="hatch", pile=grM, tool={kind="rigger", width=2.2, point=1}, angle=up, length={20,40}, coverage=0.6, pressure={0.55,0.85}, ramps={0.05,0.75}, clump=0.5})
work(foot, {hand="hatch", pile=grL, tool={kind="rigger", width=1.8, point=1}, angle=up, length={18,36}, coverage=0.25, pressure={0.5,0.8}, ramps={0.05,0.75}, clump=0.5})
print(wait(0))

--@ chunk 136
print(wait(30*60))

--@ chunk 137
grShade = pile{{"green earth", 2.4}, {"raw umber", 1.2}, {"yellow ochre", 0.9}, {"Prussian blue", 0.15}, {"lead white", 0.25}, medium=0.1}
local up = function(x,y) return -math.pi/2 + 0.3*math.sin(x/17 + y/11) end
local foot = rect(830, 650, 175, 66)
work(foot, {hand="hatch", pile=grDD, tool={kind="rigger", width=1.8, point=1}, angle=up, length={16,34}, coverage=0.7, pressure={0.5,0.8}, ramps={0.05,0.75}, clump=0.5})
work(foot, {hand="glaze", pile=pile{{"green earth", 2}, {"raw umber", 1.2}, {"Prussian blue", 0.15}, medium=0.7}, angle=-math.pi/2, coverage=0.8, load=0.35})
print(wait(0))

--@ chunk 138
local pts = {{-5,462}}
for x = -5, 205, 4 do
  local top = 447 + 3*math.sin(x/13) + 2*math.sin(x/5.3)
  if x > 150 then top = top + (x - 150) * 0.12 end
  pts[#pts+1] = {x, top}
end
pts[#pts+1] = {205, 462}
local yard = poly(pts, true):roughen(1.2, 4, 201) - roof:grow(0.3) - walls:grow(0.3)
stipple(yard, {pile=bush, width=2.2, coverage=4, pressure={0.4,0.7}, clip=yard, cluster=0.3})
stipple(yard * mask(function(x,y) return 1 end) - yard:shrink(2), {pile=tM2, width=1.6, coverage=0.8, pressure={0.35,0.55}, clip=yard, cluster={0.6,4}, feather=0.6})
-- trunks over it
local rb = brush{kind="round", width=2.6, point=0.5}
for ti, t in ipairs(lime_trees) do
  rb:load(brD, 0.7)
  rb:stroke({{t.bx, t.by - 8}, {t.fx, t.fy}}, {pressure={0.85, 0.75}})
end
-- house details: windows, a door, chimney, the ridge line
local win = pile{{"raw umber", 1}, {"bone black", 0.5}, {"pale smalt", 0.6}, {"lead white", 0.5}, medium=0.08}
for _, x in ipairs({76, 88, 104, 118, 134, 148}) do
  work(rect(x, 464.2, 2.6, 2.8), {hand="detail", pile=win, coverage=3, length={1,2}, fill=true, clip=true, edge="found"})
end
work(rect(96, 463.5, 3, 6.3), {hand="detail", pile=win, coverage=3, length={1,2}, fill=true, clip=true, edge="found"})
local ch = pile{{"red earth", 1.2}, {"lead white", 1}, {"raw umber", 0.5}, {"pale smalt", 0.4}, medium=0.08}
work(rect(112, 441.5, 3.2, 5), {hand="detail", pile=ch, coverage=3, length={1,2}, fill=true, clip=true, edge="found"})
local r = brush{kind="round", width=0.9, point=0.8}
r:load(thatchL, 0.4)
r:stroke({{69,447.3},{149,445.3}}, {pressure={0.4,0.35}})
-- smoke from the chimney, a thin pale drift to the right
local sm = pile{{"lead white", 5}, {"pale smalt", 0.8}, {"raw umber", 0.15}, medium=0.3}
local s = brush{kind="round", width=2.2, point=0.5}
s:load(sm, 0.25)
s:stroke({{113.6,441},{115,436},{119,431},{126,428},{136,426}}, {pressure={0.3,0.05}, ramps={0.1,0.6}})
print(wait(0))

--@ chunk 139
print(wait(40*60)); print(drying(110,450), drying(113,443))

--@ chunk 140
local fix = rect(104, 445, 14, 17) * roof
work(fix, {hand="detail", pile=thatch, angle=0, coverage=3, length={2,6}, fill=true, clip=true, edge="found"})
local chm = pile{{"red earth", 0.8}, {"lead white", 1.4}, {"raw umber", 0.6}, {"pale smalt", 0.7}, medium=0.08}
work(rect(112, 441.5, 3.2, 5), {hand="detail", pile=chm, coverage=3, length={1,2}, fill=true, clip=true, edge="found"})
local pts = {{-5,462}}
for x = -5, 205, 4 do
  local top = 449 + 3*math.sin(x/13) + 2*math.sin(x/5.3)
  if x > 150 then top = top + (x - 150) * 0.12 end
  pts[#pts+1] = {x, top}
end
pts[#pts+1] = {205, 462}
local yard = poly(pts, true):roughen(1.2, 4, 202) - roof:grow(0.3) - walls:grow(0.3) - lime_sil2
stipple(yard, {pile=hedgeD, width=2.0, coverage=3.5, pressure={0.4,0.7}, clip=yard, cluster=0.3})
print(wait(0))

--@ chunk 141
acc = pile{{"lead white", 2}, {"pale smalt", 1.6}, {"smalt", 0.5}, {"raw umber", 0.6}, medium=0.1}
local r = brush{kind="round", width=0.9, point=0.8}
r:load(acc, 0.4)
-- belfry openings of the tall tower (x 322-331), the square tower (420-446) and the left spire's tower (262-272)
for _, s in ipairs({{324.5,381,385},{328,381,385},{264.5,393,397},{269,393,397},{424,376,382},{429,376,382},{437,376,382},{442,376,382}, {424,386,390},{429,386,390}}) do
  r:stroke({{s[1], s[2]}, {s[1], s[3]}}, {pressure={0.45,0.4}})
end
-- tall nave windows of the big church
for x = 340, 414, 9 do r:stroke({{x, 405}, {x, 412}}, {pressure={0.35,0.3}}) end
for x = 454, 500, 9 do r:stroke({{x, 408}, {x, 414}}, {pressure={0.35,0.3}}) end
-- a thin shadow line under the eaves
r:load(acc, 0.3)
r:stroke({{331,400},{420,400}}, {pressure={0.3,0.3}})
r:stroke({{446,404},{505,404}}, {pressure={0.3,0.3}})
-- mill: a door and a gallery
r:stroke({{737,430},{737,434}}, {pressure={0.4,0.4}})
r:stroke({{730,420},{744,420}}, {pressure={0.3,0.3}})
print(wait(0))

--@ chunk 142
roofT = pile{{"lead white", 3}, {"pale smalt", 1.2}, {"red earth", 0.45}, {"raw umber", 0.4}, medium=0.1}
wallT = pile{{"lead white", 5}, {"pale smalt", 0.9}, {"yellow ochre", 0.25}, medium=0.1}
local x = 228
local k = 0
local all
local towers = spJ + rect(262,389,10,52) + spN1 + rect(322,377,9,64) + rect(420,371,26,70) + rect(330,395,90,30) + rect(446,397,62,28)
while x < 588 do
  k = k + 1
  local w = rand(9, 17)
  local wallTop = 430 - rand(0, 4)
  local gable = (rand(0,1) < 0.45)
  local roofH = rand(4, 7)
  local rm
  if gable then
    rm = poly({{x, wallTop}, {x + w/2, wallTop - roofH}, {x + w, wallTop}})
  else
    rm = poly({{x, wallTop}, {x + 2, wallTop - roofH*0.7}, {x + w - 2, wallTop - roofH*0.7}, {x + w, wallTop}})
  end
  rm = rm - towers
  local wm = rect(x, wallTop, w, 440.5 - wallTop) - towers
  work(wm, {hand="detail", pile=wallT, angle=math.pi/2, coverage=2.5, length={1.5,4}, fill=true, clip=true, edge="found"})
  work(rm, {hand="detail", pile=roofT, angle=0, coverage=2.5, length={1.5,4}, fill=true, clip=true, edge="found"})
  x = x + w + rand(-1, 1.5)
end
print(wait(0))

--@ chunk 143
print(wait(30*60)); print(drying(400,435))

--@ chunk 144
print(wait(24*60)); print(drying(400,435), drying(300,435))

--@ chunk 145
print(wait(48*60)); print(drying(400,435), drying(300,435))

--@ chunk 146
local fronts = {
  pile{{"lead white", 3}, {"pale smalt", 1.3}, {"red earth", 0.35}, {"raw umber", 0.3}, medium=0.1},
  pile{{"lead white", 3}, {"pale smalt", 1.6}, {"raw umber", 0.45}, {"smalt", 0.2}, medium=0.1},
  pile{{"lead white", 4}, {"pale smalt", 1.0}, {"yellow ochre", 0.3}, {"red earth", 0.1}, medium=0.1},
  pile{{"lead white", 3}, {"pale smalt", 1.2}, {"red earth", 0.5}, {"yellow ochre", 0.15}, medium=0.1},
}
local towers = spJ + rect(262,389,10,52) + spN1 + rect(322,377,9,64) + rect(420,371,26,70)
local x = 226
local k = 0
while x < 592 do
  k = k + 1
  local w = rand(6, 14)
  local m = rect(x, 428.5, w, 12) - towers
  local p = fronts[1 + (k * 7 + math.floor(rand(0, 3))) % 4]
  work(m, {hand="detail", pile=p, angle=math.pi/2, coverage=2.4, length={1.5,4}, fill=true, clip=true, edge="found"})
  x = x + w
end
-- windows as tiny dark dots in a few fronts
local r = brush{kind="round", width=0.8, point=0.8}
r:load(acc, 0.35)
for i = 1, 40 do
  local wx = rand(230, 588)
  if towers:at(wx, 434) < 0.1 then r:touch(wx, rand(432, 436), {pressure=0.3}) end
end
print(wait(0))

--@ chunk 147
local towers = spJ + rect(262,389,10,52) + spN1 + rect(322,377,9,64) + rect(420,371,26,70)
local band = rect(224, 427, 370, 14) - towers:grow(0.5)
blend(band, {angle=0, coverage=1.5, clip=true, tool={kind="badger", width=6}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 148
print(drying(450,660), drying(400,700))
trackSh = pile{{"raw umber", 1.2}, {"yellow ochre", 1.2}, {"green earth", 0.8}, {"lead white", 0.5}, {"pale smalt", 0.3}, medium=0.2}
local near = path_m * mask(function(x,y) return smoothstep(625, 700, y) end)
stipple(near, {pile=trackSh, width=2.4, coverage=function(x,y) return 0.6 + 1.4 * smoothstep(625, 705, y) end, pressure={0.4,0.7}, clip=path_m, cluster=0.3, feather=0.5})
print(wait(0))

--@ chunk 149
print(drying(500,455))
ryeU = pile{{"lead white", 3.2}, {"yellow ochre", 2}, {"green earth", 1.1}, {"pale smalt", 0.7}, {"raw umber", 0.1}, medium=0.1}
ryeD = pile{{"lead white", 2.4}, {"yellow ochre", 2.2}, {"green earth", 1.3}, {"pale smalt", 0.4}, {"raw umber", 0.25}, {"chrome yellow", 0.08}, medium=0.1}
local b = below({{-10,441.8},{1010,441.8}})
local rye = rect(186, 441.8, 830, 34) * b - lime_sil2:grow(0.5) - hedgerow:shrink(0.8) - willow_heads - rect(830,440,200,60) * elder2:grow(1)
local u = rye * mask(function(x,y) return 1 - smoothstep(449, 460, y) end)
local d = rye * mask(function(x,y) return smoothstep(449, 460, y) end)
work(u, {hand="body", pile=ryeU, angle=0, coverage=1.8, length={15,45}, pressure={0.4,0.65}, fill=true, clip=rye, threshold=0.3})
work(d, {hand="body", pile=ryeD, angle=0, coverage=1.8, length={15,45}, pressure={0.4,0.65}, fill=true, clip=rye, threshold=0.3})
blend(rye, {angle=0, coverage=1.4, clip=true, tool={kind="badger", width=12}, pressure={0.3,0.45}})
print(wait(0))

--@ chunk 150
print(wait(3*24*60)); print(drying(500,455), drying(500,470))

--@ chunk 151
-- one continuous hedge line along the rye's lower edge, from the farmyard to the elder, a bit fuller
local pts = {}
local n = noise{seed=211, period=14, octaves=3}
for x = 120, 1010, 2.5 do
  local base = 474.8 - (x - 175) * 0.0015
  local h = 3.4 + 1.6 * n(x, 3)
  pts[#pts+1] = {x, base - h}
end
local poly_pts = {{120, 477}}
for i, p in ipairs(pts) do poly_pts[#poly_pts+1] = p end
poly_pts[#poly_pts+1] = {1010, 477}
hedge2 = poly(poly_pts):roughen(0.8, 3, 212) - willow_heads - elder2:grow(1)
stipple(hedge2, {pile=hedgeD, width=1.7, coverage=4.5, pressure={0.4,0.7}, clip=hedge2, cluster=0.25})
stipple(hedge2 - hedge2:offset(-1.4), {pile=hedgeFL, width=1.2, coverage=0.9, pressure={0.3,0.5}, clip=hedge2, cluster={0.6, 5}, feather=0.6})
-- the left end: carry the farmyard shrubs into the rye's edge, hiding the cut at x~186
local joint = poly({{150,474},{152,462},{160,456},{172,452},{186,451},{198,455},{206,462},{210,474}}, true):roughen(1.5, 4, 213) - lime_sil2:shrink(1)
stipple(joint, {pile=hedgeD, width=2.2, coverage=4, pressure={0.4,0.7}, clip=joint, cluster=0.3})
stipple(joint - joint:offset(-2), {pile=tM2, width=1.6, coverage=1.0, pressure={0.35,0.55}, clip=joint, cluster={0.6,4}, feather=0.6})
print(wait(0))

--@ chunk 152
print(drying(280,550), drying(330,540))
cowGl = pile{{"raw umber", 1}, {"green earth", 0.8}, {"pale smalt", 0.5}, {"lead white", 0.4}, medium=0.6}
local reds = (cows[1] + cows[3]):shrink(0.3)
work(reds, {hand="detail", pile=cowGl, angle=0, coverage=1.3, length={2,5}, pressure={0.3,0.5}, load=0.35, fill=true, clip=true})
local wh = cows[2]:shrink(0.3)
local cowGl2 = pile{{"pale smalt", 1}, {"raw umber", 0.4}, {"lead white", 1.5}, {"yellow ochre", 0.2}, medium=0.6}
work(wh, {hand="detail", pile=cowGl2, angle=0, coverage=0.9, length={2,5}, pressure={0.3,0.5}, load=0.3, fill=true, clip=true})
blend(reds + wh, {angle=0, coverage=1.0, clip=true, tool={kind="badger", width=4}, pressure={0.2,0.3}})
print(wait(0))

--@ chunk 153
print(wait(2*24*60)); print(drying(280,550), drying(330,540))

--@ chunk 154
cowBr = pile{{"red earth", 1.2}, {"raw umber", 1.5}, {"bone black", 0.12}, {"yellow ochre", 0.2}, medium=0.08}
cowBrD = pile{{"raw umber", 1.6}, {"red earth", 0.6}, {"bone black", 0.35}, medium=0.08}
cowBrL = pile{{"red earth", 1}, {"yellow ochre", 0.8}, {"raw umber", 0.5}, {"lead white", 0.5}, medium=0.08}
cowCr = pile{{"lead white", 4}, {"yellow ochre", 0.4}, {"pale smalt", 0.5}, {"raw umber", 0.15}, medium=0.08}
local defs = {{262,562,18,1,true,1}, {52,533,14,1,true,3}}
for _, c in ipairs(defs) do
  local m = cows[c[6]]
  work(m, {hand="detail", pile=cowBr, angle=0, coverage=3, length={1.5,4}, fill=true, clip=true, edge="found"})
  local x0, y0, s = c[1], c[2], c[3]
  local low = m * mask(function(px, py) return (py > y0 - 0.8*s or px > x0 + 1.8*s) and 1 or 0 end)
  work(low, {hand="detail", pile=cowBrD, angle=0, coverage=2.5, length={1.5,4}, fill=true, clip=true, edge="soft"})
  local r = brush{kind="round", width=math.max(0.9, 0.09*s), point=0.8}
  r:load(cowBrL, 0.5)
  r:stroke({{x0 - 0.04*s, y0 - 1.2*s}, {x0 + 0.02*s, y0 - 1.32*s}, {x0 + 0.5*s, y0 - 1.31*s}, {x0 + 1.1*s, y0 - 1.3*s}, {x0 + 1.6*s, y0 - 1.35*s}}, {pressure={0.55,0.3}, ramps={0.1,0.3}})
  r:stroke({{x0 + 0.02*s, y0 - 1.2*s}, {x0 + 0.05*s, y0 - 0.95*s}}, {pressure={0.4,0.2}})
end
-- white cow: restate as creamy white with red-brown patches
local m2 = cows[2]
work(m2, {hand="detail", pile=cowCr, angle=0, coverage=3, length={1.5,4}, fill=true, clip=true, edge="found"})
local x0, y0, s = 342, 549, 16.6
local pm = (ellipse(x0 - 0.7*s, y0 - 1.0*s, 0.33*s, 0.26*s) + ellipse(x0 - 1.35*s, y0 - 1.1*s, 0.2*s, 0.2*s) + ellipse(x0 - 2.25*s, y0 - 1.28*s, 0.18*s, 0.14*s)):roughen(0.08*s, 0.2*s, 342) * m2
work(pm, {hand="detail", pile=cowBr, coverage=3, length={1,3}, fill=true, clip=true, edge="firm"})
local low2 = m2 * mask(function(px, py) return (py > y0 - 0.78*s) and 1 or 0 end)
work(low2, {hand="detail", pile=cowWhD, angle=0, coverage=2, length={1.5,4}, fill=true, clip=true, edge="soft"})
print(wait(0))

--@ chunk 155
storkW = pile{{"lead white", 6}, {"yellow ochre", 0.15}, {"pale smalt", 0.2}, medium=0.08}
storkRed = pile{{"vermilion", 1.5}, {"red earth", 0.5}, medium=0.08}
local x, y = 122, 551
local body = poly({{x-5.5,y-8.2},{x-3,y-10},{x+1.5,y-10.4},{x+3.2,y-9.6},{x+2.4,y-7.6},{x-1,y-6.8},{x-4.5,y-7}}, true)
local neck = ribbon({{x+2.4,y-9.8},{x+3.4,y-12.5},{x+3.2,y-14.6}}, {1.8, 1.2, 1.3})
local head = ellipse(x+3.4, y-15, 1.2, 1.0)
local wing = poly({{x-5.8,y-8.1},{x-3,y-9.3},{x+0.8,y-8.8},{x+0.2,y-7.2},{x-4.5,y-7.1}}, true)
work(body + neck + head, {hand="detail", pile=storkW, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
work(wing, {hand="detail", pile=blackp, angle=0, coverage=3, length={1,3}, fill=true, clip=true, edge="found"})
local r = brush{kind="round", width=0.7, point=1}
r:load(storkRed, 0.5)
r:stroke({{x-0.8,y-7.2},{x-1.2,y-3.5},{x-1.4,y}}, {pressure={0.5,0.45}})
r:stroke({{x+0.2,y-7.2},{x+0.8,y-3.6},{x+0.4,y+0.2}}, {pressure={0.5,0.45}})
r:stroke({{x+4.3,y-15},{x+7.4,y-13.4}}, {pressure={0.55,0.1}})
-- its shadow
local sh = ellipse(x+3, y+0.5, 5, 0.8)
work(sh, {hand="detail", pile=grassSh, coverage=2, length={1,3}, fill=true, clip=true, edge="soft", load=0.5})
print(wait(0))

--@ chunk 156
print(wait(7*24*60))

--@ chunk 157
for i,w in ipairs(willows) do print(i, string.format("x=%.1f yb=%.1f s=%.2f top=%.1f kx=%.1f H=%.1f Wd=%.1f", w.x, w.yb, w.s, w.top, w.kx or 0, w.H or 0, w.Wd or 0)) end print(#haycocks) for i,h in ipairs(haycocks) do print(i, h.x, h.y, h.w, h.h) end print(drying(700,500), drying(300,520))

--@ chunk 158
-- pollards: long upright rods rising out of the crowns, each tree its own height, to break the lollipop row
wRod = pile{{"green earth", 2}, {"raw umber", 1.2}, {"lead white", 1.0}, {"pale smalt", 0.5}, {"yellow ochre", 0.3}, medium=0.1}
wLeafS = pile{{"lead white", 3.2}, {"green earth", 1.4}, {"pale smalt", 0.8}, {"yellow ochre", 0.4}, {"raw umber", 0.1}, medium=0.1}
local ext = {1.38, 1.12, 1.55, 1.22, 1.05, 1.48, 1.18, 1.32, 1.1}
local rod = brush{kind="rigger", width=0.9, point=1}
local lf = brush{kind="round", width=1.3, point=0.8}
for i = 1, 2 do
  local w = willows[i]
  local kx, ky, H, Wd = w.kx, w.ky, w.H, w.Wd
  local top = H * ext[i]
  rod:load(wRod, 0.6)
  for k = 1, 18 do
    local u = rand(-1, 1)
    local x0 = kx + u * Wd * 0.22
    local y0 = ky - H * rand(0.35, 0.6)
    local L = (top - (ky - y0)) * rand(0.7, 1.0) * (1 - 0.35 * math.abs(u))
    local a = -math.pi/2 + u * 0.42 + rand(-0.08, 0.08)
    local bend = rand(-0.06, 0.06)
    rod:stroke({{x0, y0}, {x0 + L*0.5*math.cos(a), y0 + L*0.5*math.sin(a)}, {x0 + L*math.cos(a+bend), y0 + L*math.sin(a+bend)}}, {pressure={0.55, 0.0}, ramps={0.05, 0.7}})
    if k % 5 == 0 then rod:load(wRod, 0.55) end
    -- narrow leaves along the upper rod, silvery on the lit left
    if k % 2 == 0 then
      lf:load(wLeafS, 0.35)
      for j = 1, 4 do
        local t = rand(0.45, 0.95)
        local px, py = x0 + L*t*math.cos(a), y0 + L*t*math.sin(a)
        local side = (j % 2 == 0) and 1 or -1
        lf:stroke({{px, py}, {px + side*rand(1.2, 2.2), py - rand(0.8, 1.8)}}, {pressure={0.45, 0.05}})
      end
    end
  end
end
print(wait(0))

--@ chunk 159
-- keep-out for the two horses and the load (same outlines as painted, used only to protect them)
local function hbody(dx, dy)
  local pts = {{563,515},{565,513},{575,514},{583,511},{587,507},{590,503},{591,500.5},{592,502},{594,504},{597,510},{596,511.5},{593,511},{590,510},{588,514},{588,518},{583,521.5},{568,521.5},{563.5,519}}
  local o = {} for i, q in ipairs(pts) do o[i] = {q[1] + dx, q[2] + dy} end
  return poly(o, true)
end
horses_keep = (hbody(0,0) + hbody(3.5,-1.2) + rect(560, 515, 32, 20)):grow(0.6) + load_m:grow(1) + rect(530, 480, 18, 20)
-- a fan-shaped pollard crown: spiky top of rods, rounder shoulders
function pollard_mask(kx, ky, H, Wd, seed)
  local pts = {{kx - 2, ky + 1}}
  local n = 15
  for k = 0, n do
    local t = k / n                      -- left to right over the top
    local a = math.pi * (1 - t)          -- 180 .. 0 degrees
    local rx = Wd * 0.5 * math.cos(a)
    local ry = H * (0.62 + 0.38 * math.sin(a))
    local spike = (k % 2 == 0) and 1 or 0.86
    local r = spike * (1 + 0.08 * math.sin(seed * 3.1 + k * 1.7))
    pts[#pts+1] = {kx + rx * r, ky - (ry * r) * (0.55 + 0.45 * math.sin(a)) - H * 0.25 * math.sin(a)}
  end
  pts[#pts+1] = {kx + 2, ky + 1}
  return poly(pts, false):roughen(1.0, 3, seed)
end
function paint_pollard(kx, ky, H, Wd, seed, keep)
  local m = pollard_mask(kx, ky, H, Wd, seed) - (keep or rect(0,0,1,1))
  local fan = function(x, y) return -math.pi/2 + clamp((x - kx) / (Wd * 0.5), -1, 1) * 0.55 end
  work(m, {hand="hatch", pile=wMid, tool={kind="round", width=1.5, point=0.8}, angle=fan, length={3,8}, coverage=2.2, pressure={0.45,0.7}, ramps={0.1,0.6}, clip=m, fill=true, seed=seed})
  local sh = m * mask(function(px, py) local u = (px - kx)/Wd*0.9 + (py - (ky - H*0.55))/H*0.8 return u > 0.08 and 1 or 0 end)
  work(sh, {hand="hatch", pile=wSh, tool={kind="round", width=1.4, point=0.8}, angle=fan, length={3,7}, coverage=1.3, pressure={0.4,0.65}, ramps={0.1,0.6}, clip=m, clump=0.5, seed=seed+1})
  local hi = m * mask(function(px, py) local u = (px - kx)/Wd*0.9 + (py - (ky - H*0.55))/H*0.8 return u < -0.2 and 1 or 0 end)
  work(hi, {hand="hatch", pile=wHi, tool={kind="round", width=1.2, point=0.9}, angle=fan, length={2,6}, coverage=0.8, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=m, clump=0.6, seed=seed+2})
  -- a few dark rods seen in the crown's lower half
  local rod = brush{kind="rigger", width=1.0, point=1}
  rod:load(wDk, 0.6)
  for k = 1, 9 do
    local u = rand(-0.8, 0.8)
    local a = -math.pi/2 + u * 0.55
    local L = H * rand(0.35, 0.6)
    local p0 = {kx + u * 2, ky}
    local p1 = {kx + u*2 + L*math.cos(a), ky + L*math.sin(a)}
    if not keep or keep:at(p1[1], p1[2]) < 0.5 then
      rod:stroke({p0, p1}, {pressure={0.6, 0.05}, ramps={0.05, 0.7}, clip=m})
    end
  end
  return m
end
-- willow no. 1, rebuilt behind the horses
local kx, ky = 577.5, 496.5
local m1 = paint_pollard(kx, ky, 35, 37, 301, horses_keep)
local tb = brush{kind="round", width=3.2, point=0.3}
tb:load(wTg, 0.6)
tb:stroke({{kx - 1.2, 514}, {kx - 1.4, 503}, {kx - 1.0, ky + 0.5}}, {pressure={0.8, 0.75}, clip=-horses_keep})
tb:load(wTd, 0.6)
tb:stroke({{kx + 1.6, 514}, {kx + 1.5, 503}, {kx + 1.3, ky + 0.8}}, {pressure={0.6, 0.55}, clip=-horses_keep})
tb:touch(kx, ky + 0.3, {pressure=0.9, clip=-horses_keep})
willow1_head = m1
print(wait(0))

--@ chunk 160
function shade_pollard(m, kx, ky, H, Wd, seed, keep)
  local fan = function(x, y) return -math.pi/2 + clamp((x - kx) / (Wd * 0.5), -1, 1) * 0.55 end
  local sh = m * mask(function(px, py) local u = (px - kx)/Wd*1.0 + (py - (ky - H*0.6))/H*0.9 return smoothstep(-0.05, 0.25, u) end)
  work(sh, {hand="hatch", pile=wDk, tool={kind="round", width=1.5, point=0.8}, angle=fan, length={3,8}, coverage=1.6, pressure={0.45,0.7}, ramps={0.1,0.6}, clip=m, clump=0.5, threshold=0.4, seed=seed})
  local low = m * mask(function(px, py) return smoothstep(ky - H*0.45, ky - H*0.1, py) end)
  work(low, {hand="hatch", pile=wSh, tool={kind="round", width=1.3, point=0.8}, angle=fan, length={3,7}, coverage=1.2, pressure={0.4,0.65}, ramps={0.1,0.6}, clip=m, clump=0.5, threshold=0.4, seed=seed+1})
  local rod = brush{kind="rigger", width=1.2, point=1}
  rod:load(wTd, 0.6)
  for k = 1, 12 do
    local u = rand(-0.9, 0.9)
    local a = -math.pi/2 + u * 0.6
    local L = H * rand(0.3, 0.55)
    local p0 = {kx + u * 1.5, ky + 0.5}
    local p1 = {kx + u*1.5 + L*math.cos(a), ky + 0.5 + L*math.sin(a)}
    rod:stroke({p0, p1}, {pressure={0.7, 0.05}, ramps={0.05, 0.6}, clip=keep and (m - keep) or m})
    if k % 6 == 0 then rod:load(wTd, 0.5) end
  end
  local hi = m * mask(function(px, py) local u = (px - kx)/Wd*1.0 + (py - (ky - H*0.6))/H*0.9 return 1 - smoothstep(-0.45, -0.2, u) end)
  work(hi, {hand="hatch", pile=wHi, tool={kind="round", width=1.1, point=0.9}, angle=fan, length={2,5}, coverage=0.7, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=m, clump=0.7, threshold=0.4, seed=seed+2})
end
shade_pollard(willow1_head, 577.5, 496.5, 35, 37, 311, horses_keep)
print(wait(0))

--@ chunk 161
print(wait(26*60)); print(drying(577,480), drying(590,470))

--@ chunk 162
shade_pollard(willow1_head, 577.5, 496.5, 35, 37, 321, horses_keep) print(wait(0))

--@ chunk 163
local ext = {nil, 1.18, 1.5, 1.22, 1.0, 1.42, 1.08, 1.3, 1.12}
local wid = {nil, 1.0, 0.95, 1.1, 0.9, 1.0, 1.12, 0.95, 1.0}
new_heads = {}
for i = 2, 9 do
  local w = willows[i]
  local H = w.H * ext[i]
  local Wd = w.Wd * wid[i]
  local keep = elder2:grow(1)
  local m = paint_pollard(w.kx, w.ky, H, Wd, 400 + i, keep)
  new_heads[i] = {m=m, H=H, Wd=Wd}
end
print(wait(0))

--@ chunk 164
-- broaden the tops: two lesser fans off-center, so each crown ends in a broad broom, not a single point
local keep = elder2:grow(1)
local f = {nil, {0.85, 0.72}, {0.8, 0.9}, {0.9, 0.75}, {0.92, 0.85}, {0.78, 0.88}, {0.9, 0.8}, {0.82, 0.9}, {0.9, 0.8}}
local all_heads = {willow1_head}
for i = 2, 9 do
  local w, nh = willows[i], new_heads[i]
  local a = f[i]
  local mL = paint_pollard(w.kx - nh.Wd * 0.17, w.ky - nh.H * 0.1, nh.H * a[1] * 0.9, nh.Wd * 0.62, 500 + i, keep)
  local mR = paint_pollard(w.kx + nh.Wd * 0.17, w.ky - nh.H * 0.1, nh.H * a[2] * 0.9, nh.Wd * 0.62, 520 + i, keep)
  nh.m = nh.m + mL + mR
end
-- willow 1 too
local mL = paint_pollard(577.5 - 6, 496.5 - 3, 27, 22, 541, horses_keep)
local mR = paint_pollard(577.5 + 6.5, 496.5 - 3, 30, 22, 542, horses_keep)
willow1_head = willow1_head + mL + mR
print(wait(0))

--@ chunk 165
print(wait(30*60)); print(drying(640,470), drying(700,470))
shade_pollard(willow1_head, 577.5, 496.5, 35, 37, 611, horses_keep)
for i = 2, 9 do
  local w, nh = willows[i], new_heads[i]
  shade_pollard(nh.m, w.kx, w.ky, nh.H, nh.Wd * 1.1, 620 + i, elder2:grow(1))
end
print(wait(0))

--@ chunk 166
local heads = willow1_head
for i = 2, 9 do heads = heads + new_heads[i].m end
pasture2 = pasture - heads:grow(1) - rect(112, 530, 18, 24) - horses_keep
pastC = pile{{"green earth", 2.5}, {"lead white", 2.0}, {"pale smalt", 0.8}, {"yellow ochre", 0.7}, {"raw umber", 0.45}, medium=0.08}
pastCf = pile{{"lead white", 2.8}, {"green earth", 2.0}, {"pale smalt", 1.0}, {"yellow ochre", 0.5}, {"raw umber", 0.25}, medium=0.08}
local left = pasture2 * rect(0, 470, 300, 140)
local near = left * mask(function(x,y) return y >= 525 and 1 or 0 end)
local far = left * mask(function(x,y) return y < 525 and 1 or 0 end)
work(far, {hand="hatch", pile=pastCf, tool={kind="round", width=1.2, point=0.9}, angle=-math.pi/2 + 0.08, length={1.5,3.5}, coverage=0.9, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=pasture2, clump=0.4})
work(near, {hand="hatch", pile=pastC, tool={kind="round", width=1.4, point=0.9}, angle=-math.pi/2 + 0.08, length={3,6}, coverage=0.9, pressure={0.35,0.6}, ramps={0.1,0.6}, clip=pasture2, clump=0.4})
print(wait(0))

--@ chunk 167
pastCm = pile{{"green earth", 2.6}, {"lead white", 2.1}, {"pale smalt", 0.8}, {"yellow ochre", 0.75}, {"raw umber", 0.4}, medium=0.08}
local nz = noise{seed=731, period=60, octaves=3}
local split = function(x, y) return 522 + 14 * nz(x, y) end
local far = pasture2 * mask(function(x,y) return y < split(x,y) and 1 or 0 end)
local near = pasture2 * mask(function(x,y) return y >= split(x,y) and 1 or 0 end)
local farL = far * rect(0, 470, 300, 100)
local farR = far - rect(0, 470, 300, 100)
local nearR = near - rect(0, 470, 300, 160)
-- darken the too-white far strokes on the left; lay the new green over the rest
work(farL, {hand="hatch", pile=pastC, tool={kind="round", width=1.2, point=0.9}, angle=-math.pi/2 + 0.08, length={1.5,3.5}, coverage=1.0, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=pasture2, clump=0.4})
work(farR, {hand="hatch", pile=pastCm, tool={kind="round", width=1.2, point=0.9}, angle=-math.pi/2 + 0.08, length={1.5,3.5}, coverage=0.9, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=pasture2, clump=0.4})
work(nearR, {hand="hatch", pile=pastC, tool={kind="round", width=1.4, point=0.9}, angle=-math.pi/2 + 0.08, length={3,6}, coverage=0.9, pressure={0.35,0.6}, ramps={0.1,0.6}, clip=pasture2, clump=0.4})
print(wait(0))

--@ chunk 168
local gap = (rect(488, 476, 124, 40) * mid_m) - fieldB - hedgerow:grow(0.5) - hedge2:grow(0.5) - horses_keep:grow(1) - load_m:grow(2) - rect(528, 470, 22, 30) - willow1_head:grow(1) - rect(572, 492, 11, 26) - rect(496, 505, 16, 30)
work(gap, {hand="hatch", pile=pastCm, tool={kind="round", width=1.2, point=0.9}, angle=-math.pi/2 + 0.08, length={1.5,3.5}, coverage=1.1, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=gap, clump=0.4})
print(wait(0))

--@ chunk 169
-- break the grid of haycocks: a few more, some only half-built heaps, off the rows
local extra = {{402, 547, 1.0, 1.0}, {622, 559, 0.95, 1.05}, {737, 537, 0.55, 1.35}, {846, 541, 0.7, 1.2}, {702, 571, 0.5, 1.5}, {940, 540, 0.9, 0.95}}
for i, e in ipairs(extra) do
  local x, y = e[1], e[2]
  local S = (y - 438) * 0.13
  local h = 1.05 * S * e[3]
  local w = 1.45 * S * e[4]
  local body
  if e[3] > 0.8 then
    body = poly({{x - w/2, y}, {x - w*0.46, y - h*0.4}, {x - w*0.3, y - h*0.82}, {x - w*0.05, y - h}, {x + w*0.22, y - h*0.9}, {x + w*0.44, y - h*0.5}, {x + w/2, y}}, true)
  else
    body = poly({{x - w/2, y}, {x - w*0.42, y - h*0.6}, {x - w*0.15, y - h}, {x + w*0.2, y - h*0.95}, {x + w*0.45, y - h*0.55}, {x + w/2, y}}, true)
  end
  body = body:roughen(0.5, 2, 800 + i)
  local cast = ellipse(x + w*0.45, y + 0.3, w*0.55, math.max(1.0, h*0.14))
  work(cast - body, {hand="detail", pile=hcC, angle=0, coverage=2, length={2,6}, fill=true, clip=true, edge="soft", load=0.5})
  work(body, {hand="detail", pile=hcS, angle=math.pi/2, coverage=2.2, length={2,6}, fill=true, clip=true, edge="firm"})
  local lit = body * mask(function(px, py) return ((px - x) / w + (py - (y - h*0.5)) / h * 0.4) < -0.02 and 1 or 0 end)
  work(lit, {hand="detail", pile=hcL, angle=-1.2, coverage=2.0, length={2,5}, fill=true, clip=body, edge="soft"})
  haycocks[#haycocks+1] = {x=x, y=y, h=h, w=w}
end
-- loose hay: short raked strands trailing from the heaps and along the windrows
local st = brush{kind="round", width=1.2, point=0.8}
for k = 1, 60 do
  local hcx = haycocks[math.random(1, #haycocks)]
  local x = hcx.x + rand(-2.5, 2.5) * hcx.w
  local y = hcx.y + rand(-0.4, 1.2) * hcx.h * 0.3
  if fieldB:at(x, y) > 0.9 and figs:grow(3):at(x, y) < 0.1 and elder2:grow(2):at(x, y) < 0.1 then
    st:load((k % 3 == 0) and hcL or wind, 0.4)
    local L = (y - 438) * 0.06 * rand(0.6, 1.4)
    st:stroke({{x, y}, {x + L, y + rand(-0.4, 0.4)}}, {pressure={0.4, 0.2}})
  end
end
print(wait(0))

--@ chunk 170
print(wait(24*60)); print(drying(940,533))
local fix = rect(918, 518, 44, 26) * elder2:shrink(0.3)
stipple(fix, {pile=eD, width=2.6, coverage=4.5, pressure={0.45,0.75}, clip=fix, cluster=0.25})
local sp = brush{kind="round", width=2.0, point=0.9}
for i = 1, 14 do
  local x, y = rand(922, 960), rand(518, 544)
  if fix:at(x, y) > 0.8 then
    local a = (x < 940) and (math.pi - rand(-0.5, 0.3)) or rand(-0.3, 0.5)
    local L = rand(6, 10)
    local ex, ey = x + L*math.cos(a), y + L*math.sin(a) + 2
    sp:load((i % 2 == 0) and eL or eM, 0.45)
    sp:stroke({{x, y}, {(x+ex)/2, (y+ey)/2 - 1.5}, {ex, ey}}, {pressure={0.1, 0.02}, ramps={0.1,0.5}, clip=elder2})
    for k = 1, 3 do
      local u = k / 4
      local px, py = lerp(x, ex, u), lerp(y, ey, u) - 1
      sp:touch(px, py - 1.2, {pressure=0.55, drag={2.2, a - 0.7}, clip=elder2})
      sp:touch(px, py + 1.2, {pressure=0.55, drag={2.2, a + 0.7}, clip=elder2})
    end
  end
end
-- one umbel in the shade
local fb = brush{kind="round", width=1.3, point=0.5}
fb:load(elderFs, 0.5)
for k = 1, 7 do local a = rand(0, 2*math.pi) local d = rand(0, 3.4) fb:touch(946 + d*math.cos(a), 527 + d*math.sin(a)*0.45, {pressure=rand(0.35,0.55)}) end
print(wait(0))

--@ chunk 171
print(wait(24*60))
-- one unifying pass over the whole pasture, full width, graded by depth, no zone seams
local all = pasture2 + (((rect(488, 476, 124, 40) * mid_m) - fieldB - hedgerow:grow(0.5) - hedge2:grow(0.5) - horses_keep:grow(1) - load_m:grow(2) - rect(528, 470, 22, 30) - willow1_head:grow(1) - rect(572, 492, 11, 26) - rect(496, 505, 16, 30)))
local tl = function(x, y) return -math.pi/2 + 0.08 + 0.1 * math.sin(x / 23 + y / 7) end
local b1 = all * mask(function(x,y) return 1 - smoothstep(505, 525, y) end)
local b2 = all * mask(function(x,y) return smoothstep(505, 525, y) end)
work(b1, {hand="hatch", pile=pastCm, tool={kind="round", width=1.1, point=0.9}, angle=tl, length={1.5,3.5}, coverage=1.3, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=all, clump=0.3, threshold=0.5})
work(b2, {hand="hatch", pile=pastC, tool={kind="round", width=1.3, point=0.9}, angle=tl, length={2.5,5.5}, coverage=1.3, pressure={0.35,0.6}, ramps={0.1,0.6}, clip=all, clump=0.3, threshold=0.5})
-- a scatter of lighter tops (sun on grass heads) and darker tussocks, sparse
work(all, {hand="hatch", pile=pastL, tool={kind="round", width=1.0, point=0.9}, angle=tl, length={2,4}, coverage=0.12, pressure={0.3,0.5}, ramps={0.1,0.6}, clip=all, clump=0.7})
work(all * mask(function(x,y) return y > 515 and 1 or 0 end), {hand="hatch", pile=pastD, tool={kind="round", width=1.2, point=0.9}, angle=tl, length={2,5}, coverage=0.15, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=all, clump=0.8})
print(wait(0))

--@ chunk 172
-- close the green haloes round the cattle and the stork with the new pasture colour
local cowsAll = cows[1] + cows[2] + cows[3]
local ring = (cows[1]:grow(4) + cows[2]:grow(4) + cows[3]:grow(4) + rect(110, 528, 22, 28)) * pasture:grow(4) - cowsAll:grow(0.3) - river2:grow(1) - fieldB
local stork = rect(115, 534, 16, 19) * mask(function(x,y) return 1 end)
local keepS = ellipse(122, 543, 7, 9) + ellipse(125, 551.5, 6, 1.3)
ring = ring - keepS
local tl = function(x, y) return -math.pi/2 + 0.08 + 0.1 * math.sin(x / 23 + y / 7) end
work(ring, {hand="hatch", pile=pastC, tool={kind="round", width=1.2, point=0.9}, angle=tl, length={2,4.5}, coverage=2.0, pressure={0.35,0.6}, ramps={0.1,0.6}, clip=ring, clump=0.2, threshold=0.3, fill=true})
-- the far river as a thin broken gleam of sky, running back to the hedge
local fp = {{226,505.3},{262,501.3},{305,497.3},{350,492.8},{392,488.3},{428,483.8},{458,478.8},{486,473.2}}
local wf = pile{{"lead white", 5}, {"pale smalt", 1.6}, {"cobalt blue", 0.3}, {"raw umber", 0.1}, medium=0.1}
local g = brush{kind="round", width=1.4, point=0.6}
for i = 1, #fp - 1 do
  g:load(wf, 0.45 - i * 0.03)
  local a, b = fp[i], fp[i+1]
  g:stroke({a, {lerp(a[1], b[1], 0.5), lerp(a[2], b[2], 0.5) + 0.3}, b}, {pressure={0.45 - i*0.03, 0.4 - i*0.035}, ramps={0.1, 0.2}})
end
print(wait(0))

--@ chunk 173
cowGrSh = pile{{"green earth", 2.4}, {"raw umber", 1.1}, {"lead white", 1.0}, {"pale smalt", 0.6}, {"yellow ochre", 0.4}, medium=0.08}
local cowsAll = cows[1] + cows[2] + cows[3]
local sh = ellipse(287, 562.6, 24, 1.9) + ellipse(328, 549.6, 22, 1.7) + ellipse(72, 533.5, 19, 1.5) + ellipse(125, 551.6, 5, 0.8)
sh = sh:roughen(0.6, 3, 901) - cowsAll
work(sh, {hand="detail", pile=cowGrSh, angle=0, coverage=2.2, length={2,6}, fill=true, clip=true, edge="soft", load=0.55})
-- grass over the hooves
local g = brush{kind="rigger", width=0.9, point=1}
for _, c in ipairs({{258, 306, 562}, {301, 345, 549}, {49, 87, 533}}) do
  for k = 1, 16 do
    g:load((k % 3 == 0) and pastL or pastC, 0.45)
    local x = rand(c[1], c[2])
    local y = c[3] + rand(0.2, 1.4)
    g:stroke({{x, y}, {x + rand(-0.6, 0.6), y - rand(1.6, 3.0)}}, {pressure={0.5, 0.05}})
  end
end
print(wait(0))

--@ chunk 174
local x, y = 122, 551
local storkK = poly({{x-5.5,y-8.2},{x-3,y-10},{x+1.5,y-10.4},{x+3.2,y-9.6},{x+2.4,y-7.6},{x-1,y-6.8},{x-4.5,y-7}}, true):grow(0.6)
  + ribbon({{x+2.4,y-9.8},{x+3.4,y-12.5},{x+3.2,y-14.6}}, 2.4) + ellipse(x+3.4, y-15, 1.8, 1.6)
  + ribbon({{x-0.8,y-7.2},{x-1.2,y-3.5},{x-1.4,y}}, 1.2) + ribbon({{x+0.2,y-7.2},{x+0.8,y-3.6},{x+0.4,y+0.2}}, 1.2)
  + ribbon({{x+4.3,y-15},{x+7.4,y-13.4}}, 1.2) + ellipse(x+3, y+0.5, 5, 0.9)
local ring = ellipse(122, 543, 10, 11) - storkK
local tl = function(px, py) return -math.pi/2 + 0.08 + 0.1 * math.sin(px / 23 + py / 7) end
work(ring, {hand="hatch", pile=pastC, tool={kind="round", width=1.1, point=0.9}, angle=tl, length={2,4}, coverage=2.0, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=ring, clump=0.2, fill=true})
print(wait(0))

--@ chunk 175
-- short grass on the crest behind and around the pair, scaled by distance below the crest line
local cz = fore_m * mask(function(x,y) local d = crest_depth(x,y) return (d > 0.5 and d < 30) and 1 or 0 end) - figs:grow(1.5) - path_m:shrink(1) - elder2:grow(1) - fence:grow(1)
local up = function(x, y) return -math.pi/2 + 0.2 * math.sin(x / 13 + y / 5) end
work(cz, {hand="hatch", pile=fM, tool={kind="round", width=1.0, point=0.9}, angle=up, length={2,5}, coverage=0.7, pressure={0.35,0.6}, ramps={0.1,0.6}, clip=cz, clump=0.6})
work(cz, {hand="hatch", pile=grL, tool={kind="round", width=0.9, point=0.9}, angle=up, length={2,4}, coverage=0.25, pressure={0.3,0.5}, ramps={0.1,0.6}, clip=cz, clump=0.7})
work(cz, {hand="hatch", pile=grD, tool={kind="round", width=1.0, point=0.9}, angle=up, length={2,5}, coverage=0.25, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=cz, clump=0.8})
print(wait(0))

--@ chunk 176
local band = mask(function(x,y) local d = crest_depth(x,y) return (d > -1.5 and d < 12) and 1 or 0 end) * rect(560, 590, 300, 30) - figs:grow(1.2) - path_m:shrink(1) - elder2:grow(1)
local up = function(x, y) return -math.pi/2 + 0.25 * math.sin(x / 11 + y / 4) end
work(band, {hand="hatch", pile=fM, tool={kind="round", width=1.1, point=0.9}, angle=up, length={2,4.5}, coverage=1.3, pressure={0.4,0.65}, ramps={0.1,0.6}, clip=band:grow(1.5) - figs:grow(1), clump=0.4})
work(band, {hand="hatch", pile=grD, tool={kind="round", width=1.0, point=0.9}, angle=up, length={2,4}, coverage=0.4, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=band:grow(1.5) - figs:grow(1), clump=0.7})
work(band, {hand="hatch", pile=grL, tool={kind="round", width=0.9, point=0.9}, angle=up, length={1.5,3.5}, coverage=0.3, pressure={0.3,0.5}, ramps={0.1,0.6}, clip=band:grow(1.5) - figs:grow(1), clump=0.7})
print(wait(0))

--@ chunk 177
local band = rect(700, 596, 265, 20) * mask(function(x,y) local d = crest_depth(x,y) return (d > -4 and d < 12) and 1 or 0 end) - figs:grow(1.2) - elder2:grow(1)
local up = function(x, y) return -math.pi/2 + 0.25 * math.sin(x / 11 + y / 4) end
work(band, {hand="hatch", pile=grD, tool={kind="round", width=1.1, point=0.9}, angle=up, length={2.5,5}, coverage=0.9, pressure={0.4,0.6}, ramps={0.1,0.6}, clip=band:grow(1), clump=0.5})
work(band, {hand="hatch", pile=grL, tool={kind="round", width=1.0, point=0.9}, angle=up, length={2,4}, coverage=0.6, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=band:grow(1), clump=0.6})
print(wait(0))

--@ chunk 178
-- (crest_depth has a slope bug; use the crest line directly)
local band = rect(690, 599, 275, 16) * below({{600,601},{700,602.5},{780,601},{900,598},{1000,596}}) - figs:grow(1.2) - elder2:grow(1) - fieldB
local up = function(x, y) return -math.pi/2 + 0.25 * math.sin(x / 11 + y / 4) end
work(band, {hand="hatch", pile=grD, tool={kind="round", width=1.1, point=0.9}, angle=up, length={2.5,5}, coverage=0.9, pressure={0.4,0.6}, ramps={0.1,0.6}, clip=band:grow(1), clump=0.5})
work(band, {hand="hatch", pile=grL, tool={kind="round", width=1.0, point=0.9}, angle=up, length={2,4}, coverage=0.6, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=band:grow(1), clump=0.6})
work(band, {hand="hatch", pile=fM, tool={kind="round", width=1.0, point=0.9}, angle=up, length={2,4}, coverage=0.5, pressure={0.35,0.55}, ramps={0.1,0.6}, clip=band:grow(1), clump=0.6})
print(wait(0))

--@ chunk 179
-- the far reach reads as a wire: cover it, then state it as a few meander gleams half hidden by banks and bushes
local fp = {{226,505.3},{262,501.3},{305,497.3},{350,492.8},{392,488.3},{428,483.8},{458,478.8},{486,473.2},{520,468}}
local cover = ribbon(fp, 4.5) * rect(205, 460, 330, 50) - river2:grow(0.5) - hedge2:grow(0.5) - hedgerow:grow(0.5)
local tl = function(x, y) return -math.pi/2 + 0.08 + 0.1 * math.sin(x / 23 + y / 7) end
work(cover, {hand="hatch", pile=pastCm, tool={kind="round", width=1.1, point=0.9}, angle=tl, length={1.5,3.5}, coverage=2.4, pressure={0.4,0.6}, ramps={0.1,0.6}, clip=cover, clump=0.2, fill=true})
print(wait(0))

--@ chunk 180
print(wait(36*60)); print(drying(350,493))
local fp = {{226,505.3},{262,501.3},{305,497.3},{350,492.8},{392,488.3},{428,483.8},{458,478.8},{486,473.2},{520,468}}
local cover = ribbon(fp, 4.0) * rect(232, 460, 300, 50) - river2:grow(0.5) - hedge2:grow(0.5) - hedgerow:grow(0.5)
local tl = function(x, y) return -math.pi/2 + 0.08 + 0.1 * math.sin(x / 23 + y / 7) end
work(cover, {hand="hatch", pile=pastCm, tool={kind="round", width=1.1, point=0.9}, angle=tl, length={1.5,3.5}, coverage=2.4, pressure={0.4,0.6}, ramps={0.1,0.6}, clip=cover, clump=0.2, fill=true})
print(wait(0))

--@ chunk 181
-- far river: flattened meanders, thinning, broken by banks and scrub
wfar = pile{{"lead white", 5}, {"pale smalt", 1.2}, {"cobalt blue", 0.2}, {"yellow ochre", 0.15}, {"raw umber", 0.15}, medium=0.1}
bankF = pile{{"green earth", 2}, {"raw umber", 1.3}, {"lead white", 0.9}, {"pale smalt", 0.5}, medium=0.1}
local segs = {
  {pts={{229,504.2},{246,503.2},{262,502.6},{276,501.2},{283,499.4}}, w=2.4},
  {pts={{283,499.4},{278,497.6},{266,496.6},{258,495.4},{266,494.0},{288,493.4},{312,493.0}}, w=1.9},
  {pts={{330,492.5},{352,491.6},{368,490.4},{372,489.0},{362,487.9},{350,487.2},{356,486.2},{378,485.6}}, w=1.5},
  {pts={{396,485.0},{420,484.2},{438,483.0},{444,481.8},{436,480.9},{442,480.0},{462,479.4}}, w=1.2},
  {pts={{478,478.6},{500,478.0},{516,477.2}}, w=0.9},
}
local bk = brush{kind="round", width=1.4, point=0.5}
local wb = brush{kind="round", width=2.2, point=0.6}
for i, s in ipairs(segs) do
  -- dark cut bank just below the water
  local bp = {} for k, p in ipairs(s.pts) do bp[k] = {p[1], p[2] + s.w * 0.55} end
  bk:load(bankF, 0.5)
  bk:stroke(bp, {pressure={0.45 - i*0.04, 0.4 - i*0.04}, ramps={0.05,0.1}})
  wb:load(wfar, 0.5)
  local pr = clamp(0.25 + s.w * 0.12, 0.2, 0.6)
  wb:stroke(s.pts, {pressure={pr, pr * 0.85}, ramps={0.1,0.15}})
end
print(wait(0))

--@ chunk 182
-- a few low alder and willow bushes on the far river's banks, breaking the gleam
bushF = pile{{"green earth", 2.2}, {"raw umber", 1.0}, {"lead white", 1.6}, {"pale smalt", 0.9}, {"Prussian blue", 0.05}, medium=0.1}
bushFL = pile{{"lead white", 2.6}, {"green earth", 1.6}, {"yellow ochre", 0.6}, {"pale smalt", 0.6}, medium=0.1}
local spots = {{286, 500.5, 4.2}, {316, 494.5, 3.2}, {329, 494.0, 2.4}, {384, 487.4, 2.8}, {392, 486.8, 2.0}, {464, 480.6, 2.2}, {474, 480.2, 1.6}}
for i, s in ipairs(spots) do
  local x, y, r = s[1], s[2], s[3]
  local m = (ellipse(x, y - r*0.55, r, r*0.7) + ellipse(x + r*0.6, y - r*0.35, r*0.7, r*0.5)):roughen(r*0.15, r*0.4, 950 + i) * above({{0, y + 0.6}, {1000, y + 0.6}})
  stipple(m, {pile=bushF, width=math.max(1.0, r*0.5), coverage=4, pressure={0.4,0.65}, clip=m, cluster=0.2})
  local lt = m * mask(function(px, py) return ((px - x) + (py - (y - r*0.6)) * 0.8) < -r*0.2 and 1 or 0 end)
  stipple(lt, {pile=bushFL, width=math.max(0.9, r*0.4), coverage=1.5, pressure={0.35,0.55}, clip=m, cluster=0.4, feather=0.5})
end
print(wait(0))

--@ chunk 183
print(wait(30*60))
bushD = pile{{"green earth", 2.2}, {"raw umber", 1.4}, {"lead white", 0.6}, {"pale smalt", 0.6}, {"Prussian blue", 0.1}, medium=0.08}
local spots = {{286, 500.5, 4.2}, {316, 494.5, 3.2}, {329, 494.0, 2.4}, {384, 487.4, 2.8}, {392, 486.8, 2.0}, {464, 480.6, 2.2}, {474, 480.2, 1.6}}
for i, s in ipairs(spots) do
  local x, y, r = s[1], s[2], s[3]
  local m = (ellipse(x, y - r*0.55, r, r*0.7) + ellipse(x + r*0.6, y - r*0.35, r*0.7, r*0.5)):roughen(r*0.15, r*0.4, 950 + i) * above({{0, y + 0.6}, {1000, y + 0.6}})
  local dk = m * mask(function(px, py) return ((px - x) + (py - (y - r*0.6)) * 0.8) > -r*0.3 and 1 or 0 end)
  stipple(dk, {pile=bushD, width=math.max(0.9, r*0.45), coverage=3.5, pressure={0.4,0.65}, clip=m, cluster=0.2})
end
print(wait(0))

--@ chunk 184
print(wait(7*24*60))
