-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=450, aspect=1.35, linen={14,12}, seed=7, ground={{pile={{"lead white", 8},{"yellow ochre", 1},{"raw umber", 0.4}}, um=120, apply="knife", texture=0.35}, {pile={{"lead white", 5},{"yellow ochre", 1}}, um=40, apply="brush"}}}

--@ chunk 2

print(table.concat(tubes(), ", "))
skyTop = pile{{"lead white", 4}, {"cobalt blue", 1}, {"smalt", 0.6}, {"bone black", 0.08}}
skyMid = pile{{"lead white", 5}, {"pale smalt", 1.2}, {"yellow ochre", 0.35}, {"vermilion", 0.12}}
horizonGlow = pile{{"lead white", 5}, {"chrome yellow", 1.1}, {"vermilion", 0.45}, {"red earth", 0.25}}
print(skyTop)
print(skyMid)
print(horizonGlow)
sky = rect(0, 0, 1000, 430)
work(sky, {hand="broad", pile=skyMid, angle=0.1, coverage=2.2, seed=11})

--@ chunk 3

-- strengthen sky mixtures, lay top cool and low warm
skyTop2 = pile{{"lead white", 3}, {"cobalt blue", 1}, {"smalt", 0.7}, {"Prussian blue", 0.12}}
skyLow = pile{{"lead white", 4}, {"pale smalt", 0.8}, {"chrome yellow", 0.35}, {"vermilion", 0.22}}
glowCore = pile{{"lead white", 4}, {"chrome yellow", 1.4}, {"vermilion", 0.6}, {"yellow ochre", 0.5}}
print(skyTop2); print(skyLow); print(glowCore)
topBand = rect(0,0,1000,240)
work(topBand, {hand="broad", pile=skyTop2, angle=-0.05, coverage=3.2, fill=true, seed=21})

--@ chunk 4

midBand = rect(0, 170, 1000, 190)
work(midBand, {hand="broad", pile=skyLow, angle=0.04, coverage=3.0, fill=true, seed=31})
lowBand = rect(0, 290, 1000, 150)
work(lowBand, {hand="broad", pile=glowCore, angle=0.02, coverage=3.2, fill=true, seed=32})

--@ chunk 5

skyAll = rect(0,0,1000,440)
blend(skyAll, {seed=41})
-- sun glow: warm bright pool left of centre, low
sunPool = ellipse(360, 360, 150, 55)
sunBright = pile{{"lead white", 6}, {"chrome yellow", 1}, {"yellow ochre", 0.3}}
work(sunPool, {hand="body", pile=sunBright, angle=0.1, coverage=2.5, seed=42})

--@ chunk 6

cloudShadow = pile{{"lead white", 3.5}, {"cobalt blue", 0.9}, {"raw umber", 0.3}, {"vermilion", 0.15}}
cloudLight = pile{{"lead white", 6}, {"vermilion", 0.3}, {"yellow ochre", 0.4}, {"pale smalt", 0.35}}
cloudPink = pile{{"lead white", 4.5}, {"vermilion", 0.35}, {"red earth", 0.2}, {"cobalt blue", 0.25}}
print(cloudShadow); print(cloudLight)
-- upper clouds, each its own hand-drawn ellipse shape
c1 = ellipse(720, 120, 150, 28)
c2 = ellipse(220, 170, 130, 24)
c3 = ellipse(500, 210, 170, 30)
c4 = ellipse(830, 230, 110, 20)
c5 = ellipse(130, 90, 90, 18)
work(c1, {hand="body", pile=cloudShadow, angle=0.05, coverage=2.2, edge="soft", seed=51})
work(c2, {hand="body", pile=cloudShadow, angle=-0.04, coverage=2.2, edge="soft", seed=52})
work(c3, {hand="body", pile=cloudShadow, angle=0.03, coverage=2.2, edge="soft", seed=53})
work(c4, {hand="body", pile=cloudPink, angle=0.0, coverage=2.2, edge="soft", seed=54})
work(c5, {hand="body", pile=cloudLight, angle=0.0, coverage=2.0, edge="soft", seed=55})

--@ chunk 7

-- light caps on the grey clouds, slightly above centre, smaller and hand-varied
c1l = ellipse(710, 108, 120, 18)
c2l = ellipse(215, 158, 100, 15)
c3l = ellipse(490, 196, 135, 18)
work(c1l, {hand="body", pile=cloudLight, angle=0.05, coverage=2.0, edge="soft", seed=61})
work(c2l, {hand="body", pile=cloudLight, angle=0.0, coverage=2.0, edge="soft", seed=62})
work(c3l, {hand="body", pile=cloudLight, angle=0.02, coverage=2.0, edge="soft", seed=63})
-- soften sun pool edge outward
sunSoft = ellipse(360, 360, 190, 80)
blend(sunSoft, {seed=64})
-- gently fuse clouds (each blended in place)
blend(ellipse(720, 115, 165, 40), {seed=65})
blend(ellipse(220, 165, 145, 36), {seed=66})
blend(ellipse(500, 205, 185, 42), {seed=67})

--@ chunk 8

-- knit sky: fresh paint across the cloud zone to even the texture
repairMid = rect(0, 40, 1000, 260)
work(repairMid, {hand="broad", pile=skyTop2, angle=0.06, coverage=1.6, seed=71})
work(rect(0, 180, 1000, 150), {hand="broad", pile=skyLow, angle=0.03, coverage=1.8, seed=72})

--@ chunk 9

print(drying(500,150))
print(drying(500,350))
farHill = pile{{"lead white", 2.5}, {"cobalt blue", 0.6}, {"raw umber", 0.5}, {"green earth", 0.5}, {"vermilion", 0.08}}
treeDark = pile{{"raw umber", 2}, {"Prussian blue", 0.6}, {"bone black", 0.35}, {"green earth", 0.7}, {"red earth", 0.3}}
print(farHill); print(treeDark)
-- wavy horizon around y 390
horizonY = function(x) return 388 + 10*math.sin(x/190) + 6*math.sin(x/73+1.2) end
farShore = below(horizonY) * above(function(x) return horizonY(x)-46 end)
-- far hill band, loose edge for aerial softness
work(farShore, {hand="body", pile=farHill, angle=0.0, coverage=2.6, edge="loose", seed=81})
-- darker tree clumps sitting on horizon: hand-picked blobs, each own shape
t1 = poly({{80,382},{140,362},{210,368},{250,384},{180,392},{100,392}}, true)
t2 = poly({{300,380},{360,360},{430,366},{470,382},{400,390},{320,390}}, true)
t3 = poly({{560,382},{620,358},{700,364},{750,380},{680,390},{590,390}}, true)
t4 = poly({{800,380},{860,362},{930,368},{970,382},{900,390},{830,390}}, true)
work(t1, {hand="body", pile=treeDark, angle=0.1, coverage=2.4, edge="soft", seed=82})
work(t2, {hand="body", pile=treeDark, angle=-0.05, coverage=2.4, edge="soft", seed=83})
work(t3, {hand="body", pile=treeDark, angle=0.08, coverage=2.4, edge="soft", seed=84})
work(t4, {hand="body", pile=treeDark, angle=0.0, coverage=2.4, edge="soft", seed=85})

--@ chunk 10

horizonY2 = function(x) return 388 + 10*math.sin(x/190) + 6*math.sin(x/73+1.2) end
topLine = function(x) return horizonY2(x)-48 end
farShore2 = below(topLine) * above(horizonY2)
print(farShore2:area())
work(farShore2, {hand="body", pile=farHill, angle=0.0, coverage=3.0, fill=true, edge="firm", seed=91})

--@ chunk 11

treeDark2 = pile{{"raw umber", 2.2}, {"Prussian blue", 0.7}, {"bone black", 0.4}, {"green earth", 0.6}}
treeLit = pile{{"raw umber", 1.5}, {"green earth", 1}, {"yellow ochre", 0.7}, {"chrome yellow", 0.25}}
print(treeDark2); print(treeLit)
u1 = poly({{60,386},{120,364},{190,366},{240,380},{220,392},{100,394}}, true)
u2 = poly({{290,384},{350,360},{420,364},{470,380},{440,392},{310,393}}, true)
u3 = poly({{550,384},{615,358},{690,362},{750,380},{720,392},{570,393}}, true)
u4 = poly({{795,383},{855,360},{925,366},{975,381},{940,392},{810,393}}, true)
work(u1, {hand="body", pile=treeDark2, angle=0.15, coverage=3.4, fill=true, edge="soft", seed=101})
work(u2, {hand="body", pile=treeDark2, angle=-0.08, coverage=3.4, fill=true, edge="soft", seed=102})
work(u3, {hand="body", pile=treeDark2, angle=0.1, coverage=3.4, fill=true, edge="soft", seed=103})
work(u4, {hand="body", pile=treeDark2, angle=0.0, coverage=3.4, fill=true, edge="soft", seed=104})
-- sunlit tops: thin warm rims along the upper edge of each mass
r1 = poly({{110,368},{160,362},{210,368}}, true)
r2 = poly({{340,364},{390,360},{440,368}}, true)
r3 = poly({{605,362},{655,356},{705,364}}, true)
work(r1:band(10, 6), {hand="detail", pile=treeLit, angle=0.1, coverage=2.0, seed=105})
work(r2:band(10, 6), {hand="detail", pile=treeLit, angle=0.0, coverage=2.0, seed=106})
work(r3:band(10, 6), {hand="detail", pile=treeLit, angle=0.05, coverage=2.0, seed=107})

--@ chunk 12

waterBase = pile{{"lead white", 2.8}, {"cobalt blue", 0.7}, {"green earth", 0.5}, {"raw umber", 0.35}, {"pale smalt", 0.4}}
waterWarm = pile{{"lead white", 3.5}, {"yellow ochre", 0.8}, {"vermilion", 0.3}, {"cobalt blue", 0.25}}
waterDeep = pile{{"Prussian blue", 1}, {"raw umber", 1}, {"green earth", 0.6}, {"bone black", 0.25}}
glitter = pile{{"lead white", 6}, {"chrome yellow", 1.2}, {"vermilion", 0.25}}
print(waterBase); print(waterWarm); print(waterDeep); print(glitter)
horizonY3 = function(x) return 388 + 10*math.sin(x/190) + 6*math.sin(x/73+1.2) end
waterZone = below(horizonY3) * above(function(x) return 590 end)
-- wait: above(590)? need y < 590? No: above(590) would be y<590? Actually above takes curve? above(590)? guide says above(curve) but maybe number? Use rect-style: simpler: waterZone = below(horizon line) - below(590 line)
waterZone = below(horizonY3) - below(function(x) return 590 end)
print(waterZone:area())
work(waterZone, {hand="broad", pile=waterBase, angle=0.03, coverage=3.0, fill=true, seed=111})

--@ chunk 13

-- warm sun-struck water under the glow, drawn by eye as a widening shaft
shaft = poly({{300,398},{430,398},{480,520},{470,565},{290,565},{260,500}}, true)
work(shaft, {hand="body", pile=waterWarm, angle=0.0, coverage=2.4, edge="soft", seed=121})
-- tree reflections: short dark smudges below each mass, each drawn separately
f1 = poly({{105,392},{175,392},{185,440},{160,455},{115,450},{95,430}}, true)
f2 = poly({{315,392},{415,392},{425,445},{380,460},{320,452}}, true)
f3 = poly({{580,392},{695,392},{710,445},{660,458},{590,450}}, true)
f4 = poly({{825,392},{935,392},{945,435},{900,448},{830,442}}, true)
work(f1, {hand="body", pile=waterDeep, angle=1.55, coverage=2.2, edge="soft", seed=122})
work(f2, {hand="body", pile=waterDeep, angle=1.55, coverage=2.2, edge="soft", seed=123})
work(f3, {hand="body", pile=waterDeep, angle=1.6, coverage=2.2, edge="soft", seed=124})
work(f4, {hand="body", pile=waterDeep, angle=1.5, coverage=2.2, edge="soft", seed=125})

--@ chunk 14

-- veil to push back the harsh reflections
veil = rect(0, 390, 1000, 180)
work(veil, {hand="broad", pile=waterBase, angle=0.02, coverage=1.4, seed=131})
-- softer warm shaft, horizontal handling
shaft2 = poly({{310,400},{425,400},{470,520},{450,560},{300,560},{275,490}}, true)
work(shaft2, {hand="body", pile=waterWarm, angle=0.0, coverage=1.8, edge="soft", fill=true, seed=132})
-- muted reflection mixture
reflMuted = pile{{"Prussian blue", 0.6}, {"raw umber", 0.8}, {"green earth", 0.5}, {"lead white", 1.5}, {"cobalt blue", 0.3}}
print(reflMuted)
g1 = poly({{110,394},{180,394},{190,430},{120,432}}, true)
g2 = poly({{320,394},{410,394},{420,432},{325,434}}, true)
g3 = poly({{590,394},{690,394},{700,432},{595,434}}, true)
g4 = poly({{830,394},{930,394},{940,428},{835,430}}, true)
work(g1, {hand="body", pile=reflMuted, angle=0.0, coverage=1.8, edge="soft", fill=true, seed=133})
work(g2, {hand="body", pile=reflMuted, angle=0.0, coverage=1.8, edge="soft", fill=true, seed=134})
work(g3, {hand="body", pile=reflMuted, angle=0.0, coverage=1.8, edge="soft", fill=true, seed=135})
work(g4, {hand="body", pile=reflMuted, angle=0.0, coverage=1.8, edge="soft", fill=true, seed=136})

--@ chunk 15

-- horizontal ripple lights across water, broken handling
rippleLight = pile{{"lead white", 4}, {"yellow ochre", 0.6}, {"pale smalt", 0.3}}
spark = pile{{"lead white", 7}, {"chrome yellow", 1}, {"vermilion", 0.15}}
print(rippleLight); print(spark)
ripZone = rect(40, 420, 920, 140)
work(ripZone, {hand="hatch", pile=rippleLight, angle=0.02, coverage=0.7, seed=141})
-- glitter path: small bright dashes concentrated in shaft
glitZone = poly({{300,410},{430,410},{470,540},{310,545}}, true)
work(glitZone, {hand="hatch", pile=spark, angle=0.0, coverage=0.9, seed=142})

--@ chunk 16

softVeil = rect(0, 400, 1000, 180)
work(softVeil, {hand="body", pile=waterBase, angle=0.0, coverage=2.0, fill=true, edge="soft", seed=151})
work(softVeil, {hand="broad", pile=waterWarm, angle=0.02, coverage=0.8, seed=152})

--@ chunk 17

mudBase = pile{{"raw umber", 2}, {"yellow ochre", 1}, {"green earth", 0.8}, {"lead white", 1.2}, {"bone black", 0.3}}
reedDark = pile{{"raw umber", 2.2}, {"bone black", 0.7}, {"Prussian blue", 0.4}, {"green earth", 0.6}}
grassLit = pile{{"yellow ochre", 1.5}, {"chrome yellow", 0.6}, {"green earth", 0.8}, {"lead white", 1.5}, {"raw umber", 0.6}}
print(mudBase); print(reedDark); print(grassLit)
bankTop = function(x) return 588 + 14*math.sin(x/160+0.5) + 8*math.sin(x/67+2.0) end
bank = below(bankTop)
print(bank:area())
work(bank, {hand="body", pile=mudBase, angle=0.1, coverage=3.2, fill=true, edge="firm", seed=161})
-- sunlit patches on the bank
litPatch1 = poly({{120,620},{260,610},{300,680},{200,710},{100,690}}, true)
litPatch2 = poly({{550,615},{700,610},{740,680},{620,705},{540,680}}, true)
work(litPatch1, {hand="body", pile=grassLit, angle=0.2, coverage=2.2, edge="soft", fill=true, seed=162})
work(litPatch2, {hand="body", pile=grassLit, angle=-0.1, coverage=2.2, edge="soft", fill=true, seed=163})

--@ chunk 18

-- knock back the harsh yellow ovals
toneDown = pile{{"raw umber", 1.8}, {"yellow ochre", 0.9}, {"green earth", 0.7}, {"lead white", 1.0}}
lp1soft = ellipse(195, 660, 110, 48)
lp2soft = ellipse(635, 658, 115, 50)
work(lp1soft, {hand="body", pile=toneDown, angle=0.1, coverage=1.8, edge="soft", seed=171})
work(lp2soft, {hand="body", pile=toneDown, angle=-0.08, coverage=1.8, edge="soft", seed=172})
-- drier sunlit grass: thin horizontal touches over the toned patches
grassSoft = pile{{"yellow ochre", 1.2}, {"green earth", 0.7}, {"lead white", 2.0}, {"raw umber", 0.5}}
print(grassSoft)
gA = poly({{110,630},{260,625},{290,670},{180,690},{105,675}}, true)
gB = poly({{550,625},{700,620},{730,670},{610,690},{545,670}}, true)
work(gA, {hand="body", pile=grassSoft, angle=0.05, coverage=1.4, edge="soft", seed=173})
work(gB, {hand="body", pile=grassSoft, angle=-0.04, coverage=1.4, edge="soft", seed=174})

--@ chunk 19

-- reed-bed base masses rising from the bank, irregular hand shapes
rb1 = poly({{40,620},{70,540},{130,520},{170,560},{180,630},{140,660},{60,660}}, true)
rb2 = poly({{420,625},{440,560},{490,545},{520,580},{515,640},{470,655}}, true)
rb3 = poly({{800,620},{830,540},{900,525},{950,560},{960,630},{910,660},{830,655}}, true)
work(rb1, {hand="body", pile=reedDark, angle=1.5, coverage=2.6, fill=true, edge="soft", seed=181})
work(rb2, {hand="body", pile=reedDark, angle=1.55, coverage=2.6, fill=true, edge="soft", seed=182})
work(rb3, {hand="body", pile=reedDark, angle=1.5, coverage=2.6, fill=true, edge="soft", seed=183})
-- ragged tops: lighter dry grass tufts catching sun on the reed masses
tuft = pile{{"yellow ochre", 1}, {"lead white", 2}, {"green earth", 0.5}, {"chrome yellow", 0.3}}
print(tuft)
t1m = poly({{75,545},{120,530},{155,550},{120,570},{80,568}}, true)
t3m = poly({{835,545},{885,530},{930,552},{890,572},{840,568}}, true)
work(t1m, {hand="body", pile=tuft, angle=0.1, coverage=1.6, edge="soft", seed=184})
work(t3m, {hand="body", pile=tuft, angle=-0.08, coverage=1.6, edge="soft", seed=185})

--@ chunk 20

stalkPile = pile{{"raw umber", 1.8}, {"bone black", 0.5}, {"green earth", 0.5}}
lightStalk = pile{{"yellow ochre", 1.2}, {"lead white", 1.2}, {"green earth", 0.5}}
print(stalkPile)
r = brush{kind="rigger", width=2.2, point=0.9, stiffness=0.5}
r:load(stalkPile, 0.55)
-- left bed stalks: each its own hand-drawn path, varying lean and height
r:stroke({{70, 620}, {66, 560}, {64, 505}}, {pressure={0.55, 0.0}, ramps={0.1, 0.25}})
r:stroke({{95, 625}, {96, 565}, {100, 500}}, {pressure={0.6, 0.0}, ramps={0.1, 0.25}})
r:stroke({{120, 630}, {118, 570}, {112, 498}}, {pressure={0.55, 0.0}, ramps={0.1, 0.3}})
r:stroke({{140, 628}, {144, 570}, {150, 510}}, {pressure={0.6, 0.0}, ramps={0.1, 0.25}})
r:stroke({{160, 625}, {162, 575}, {168, 520}}, {pressure={0.5, 0.0}, ramps={0.1, 0.2}})
r:load(lightStalk, 0.5)
r:stroke({{85, 622}, {82, 570}, {80, 520}}, {pressure={0.5, 0.0}, ramps={0.1, 0.25}})
r:stroke({{130, 628}, {132, 575}, {136, 515}}, {pressure={0.5, 0.0}, ramps={0.1, 0.25}})
-- right bed stalks
r:load(stalkPile, 0.55)
r:stroke({{835, 630}, {830, 570}, {828, 505}}, {pressure={0.6, 0.0}, ramps={0.1, 0.25}})
r:stroke({{865, 635}, {866, 570}, {862, 495}}, {pressure={0.6, 0.0}, ramps={0.1, 0.3}})
r:stroke({{895, 632}, {898, 570}, {905, 505}}, {pressure={0.55, 0.0}, ramps={0.1, 0.25}})
r:stroke({{920, 628}, {924, 575}, {930, 515}}, {pressure={0.5, 0.0}, ramps={0.1, 0.2}})
r:stroke({{850, 630}, {848, 580}, {845, 525}}, {pressure={0.5, 0.0}, ramps={0.1, 0.2}})
-- centre small bed
r:stroke({{450, 630}, {448, 575}, {446, 520}}, {pressure={0.55, 0.0}, ramps={0.1, 0.25}})
r:stroke({{475, 632}, {478, 578}, {482, 522}}, {pressure={0.55, 0.0}, ramps={0.1, 0.25}})
r:stroke({{495, 630}, {498, 580}, {502, 530}}, {pressure={0.5, 0.0}, ramps={0.1, 0.2}})
print("stalks done, fullness "..r:fullness())

--@ chunk 21

print(wait(480))
print(drying(110, 600))
print(drying(500, 480))
print(drying(500, 150))

--@ chunk 22

stalkDark = pile{{"raw umber", 2}, {"bone black", 0.8}, {"Prussian blue", 0.3}}
stalkSun = pile{{"yellow ochre", 1.4}, {"lead white", 1.6}, {"raw umber", 0.5}, {"chrome yellow", 0.35}}
headPile = pile{{"raw umber", 1.5}, {"red earth", 0.5}, {"bone black", 0.4}}
print(stalkDark); print(stalkSun)
s = brush{kind="rigger", width=3, point=0.85, stiffness=0.55}
s:load(stalkDark, 0.85)
s:stroke({{68, 640}, {64, 560}, {60, 478}}, {pressure={0.7, 0.05}, ramps={0.08, 0.2}})
s:stroke({{92, 645}, {90, 565}, {88, 485}}, {pressure={0.7, 0.05}, ramps={0.08, 0.2}})
s:stroke({{115, 648}, {114, 568}, {110, 472}}, {pressure={0.75, 0.05}, ramps={0.08, 0.2}})
s:stroke({{138, 645}, {142, 568}, {148, 482}}, {pressure={0.7, 0.05}, ramps={0.08, 0.2}})
s:stroke({{162, 640}, {166, 570}, {172, 495}}, {pressure={0.65, 0.05}, ramps={0.08, 0.2}})
s:stroke({{448, 645}, {446, 575}, {443, 498}}, {pressure={0.7, 0.05}, ramps={0.08, 0.2}})
s:stroke({{472, 648}, {476, 575}, {480, 495}}, {pressure={0.7, 0.05}, ramps={0.08, 0.2}})
s:stroke({{498, 642}, {502, 580}, {506, 512}}, {pressure={0.65, 0.05}, ramps={0.08, 0.2}})
s:stroke({{832, 645}, {828, 568}, {824, 480}}, {pressure={0.7, 0.05}, ramps={0.08, 0.2}})
s:stroke({{862, 650}, {862, 570}, {860, 470}}, {pressure={0.75, 0.05}, ramps={0.08, 0.2}})
s:stroke({{892, 648}, {896, 570}, {902, 482}}, {pressure={0.7, 0.05}, ramps={0.08, 0.2}})
s:stroke({{922, 642}, {927, 572}, {933, 498}}, {pressure={0.65, 0.05}, ramps={0.08, 0.2}})
s:load(stalkSun, 0.8)
s:stroke({{80, 642}, {78, 570}, {76, 505}}, {pressure={0.6, 0.05}, ramps={0.08, 0.2}})
s:stroke({{126, 646}, {128, 572}, {130, 500}}, {pressure={0.6, 0.05}, ramps={0.08, 0.2}})
s:stroke({{850, 648}, {849, 575}, {847, 500}}, {pressure={0.6, 0.05}, ramps={0.08, 0.2}})
s:stroke({{908, 645}, {911, 575}, {915, 505}}, {pressure={0.6, 0.05}, ramps={0.08, 0.2}})
print("fullness "..s:fullness())
-- seed heads: small pressed touches at chosen stalk tips, each placed by eye
h = brush{kind="round", width=4, point=0.5}
h:load(headPile, 0.7)
h:touch(60, 478, {pressure=0.55})
h:touch(88, 485, {pressure=0.55})
h:touch(110, 472, {pressure=0.6})
h:touch(148, 482, {pressure=0.55})
h:touch(443, 498, {pressure=0.55})
h:touch(480, 495, {pressure=0.55})
h:touch(824, 480, {pressure=0.6})
h:touch(860, 470, {pressure=0.62})
h:touch(902, 482, {pressure=0.55})

--@ chunk 23

-- mottle the bank to dissolve the oval edges: several thin layers
mottleDark = pile{{"raw umber", 1.6}, {"bone black", 0.3}, {"green earth", 0.6}, {"yellow ochre", 0.6}}
mottleLight = pile{{"yellow ochre", 1.1}, {"lead white", 1.8}, {"green earth", 0.6}, {"raw umber", 0.4}}
print(mottleDark); print(mottleLight)
bankAll = below(function(x) return 590 + 14*math.sin(x/160+0.5) + 8*math.sin(x/67+2.0) end)
work(bankAll, {hand="body", pile=mottleDark, angle=0.3, coverage=1.1, seed=231})
work(bankAll, {hand="body", pile=mottleLight, angle=-0.15, coverage=1.1, seed=232})
-- carve the reed blobs: bite bank-coloured notches into their round sides
carve = pile{{"raw umber", 2}, {"yellow ochre", 1}, {"green earth", 0.8}, {"lead white", 1.2}}
n1 = poly({{30,590},{55,585},{60,620},{35,630}}, true)
n2 = poly({{170,585},{195,590},{190,625},{168,620}}, true)
n3 = poly({{815,588},{840,583},{845,620},{818,625}}, true)
n4 = poly({{940,590},{965,595},{960,630},{938,625}}, true)
work(n1, {hand="detail", pile=carve, angle=0.2, coverage=2.0, seed=233})
work(n2, {hand="detail", pile=carve, angle=0.0, coverage=2.0, seed=234})
work(n3, {hand="detail", pile=carve, angle=0.1, coverage=2.0, seed=235})
work(n4, {hand="detail", pile=carve, angle=0.0, coverage=2.0, seed=236})

--@ chunk 24

mistPile = pile{{"lead white", 4}, {"pale smalt", 0.5}, {"yellow ochre", 0.4}}
print(mistPile)
-- pale veils at the feet of the far masses, each drawn by eye
m1 = poly({{70,384},{190,382},{210,402},{90,405}}, true)
m2 = poly({{290,383},{450,382},{465,403},{305,404}}, true)
m3 = poly({{565,383},{720,382},{735,404},{580,405}}, true)
m4 = poly({{805,383},{955,382},{970,403},{820,404}}, true)
work(m1, {hand="body", pile=mistPile, angle=0.0, coverage=1.2, edge="soft", seed=241})
work(m2, {hand="body", pile=mistPile, angle=0.0, coverage=1.2, edge="soft", seed=242})
work(m3, {hand="body", pile=mistPile, angle=0.0, coverage=1.2, edge="soft", seed=243})
work(m4, {hand="body", pile=mistPile, angle=0.0, coverage=1.2, edge="soft", seed=244})
-- foreground sunlit grass: short leaning strokes
fgLight = pile{{"yellow ochre", 1.1}, {"lead white", 1.6}, {"chrome yellow", 0.3}, {"green earth", 0.5}}
fore = below(function(x) return 610 + 12*math.sin(x/150+0.5) end)
work(fore, {hand="hatch", pile=fgLight, angle=0.25, coverage=0.5, seed=245})

--@ chunk 25

-- cover the chalky mist with local colour again
coverWater = pile{{"lead white", 2.8}, {"cobalt blue", 0.7}, {"green earth", 0.5}, {"raw umber", 0.35}}
r1 = poly({{65,386},{195,384},{215,405},{70,408}}, true)
r2 = poly({{285,385},{455,384},{470,406},{300,408}}, true)
r3 = poly({{560,385},{725,384},{740,406},{575,408}}, true)
r4 = poly({{800,385},{960,384},{975,406},{815,408}}, true)
work(r1, {hand="body", pile=coverWater, angle=0.0, coverage=2.0, fill=true, edge="firm", seed=251})
work(r2, {hand="body", pile=coverWater, angle=0.0, coverage=2.0, fill=true, edge="firm", seed=252})
work(r3, {hand="body", pile=coverWater, angle=0.0, coverage=2.0, fill=true, edge="firm", seed=253})
work(r4, {hand="body", pile=coverWater, angle=0.0, coverage=2.0, fill=true, edge="firm", seed=254})
-- re-seat islands: touch the dark masses down to the water with soft edge
touchDark = pile{{"raw umber", 2}, {"Prussian blue", 0.6}, {"bone black", 0.35}}
s1 = poly({{85,378},{175,376},{185,394},{90,395}}, true)
s2 = poly({{305,378},{425,376},{435,394},{310,395}}, true)
s3 = poly({{585,378},{700,376},{710,394},{590,395}}, true)
s4 = poly({{820,378},{940,376},{950,394},{825,395}}, true)
work(s1, {hand="detail", pile=touchDark, angle=0.0, coverage=2.2, seed=255})
work(s2, {hand="detail", pile=touchDark, angle=0.0, coverage=2.2, seed=256})
work(s3, {hand="detail", pile=touchDark, angle=0.0, coverage=2.2, seed=257})
work(s4, {hand="detail", pile=touchDark, angle=0.0, coverage=2.2, seed=258})

--@ chunk 26

freshFar = pile{{"lead white", 2.6}, {"cobalt blue", 0.55}, {"raw umber", 0.45}, {"green earth", 0.45}}
hTop = function(x) return 344 + 8*math.sin(x/210+0.3) end
hBot = function(x) return 392 + 10*math.sin(x/190) + 6*math.sin(x/73+1.2) end
farBand = below(hTop) - below(hBot)
work(farBand, {hand="broad", pile=freshFar, angle=0.02, coverage=2.8, fill=true, seed=261})
-- new treeline: one varied mass with gaps, hand-drawn
treeline = poly({{40,388},{90,368},{160,364},{230,372},{280,384},{340,366},{410,362},{470,374},{530,384},{600,368},{680,362},{750,374},{810,384},{870,366},{940,370},{975,382},{975,394},{40,394}}, true)
treeMass = pile{{"raw umber", 2}, {"Prussian blue", 0.55}, {"bone black", 0.35}, {"green earth", 0.55}}
work(treeline, {hand="body", pile=treeMass, angle=0.05, coverage=2.6, fill=true, edge="soft", seed=262})
-- break the top with sky-coloured notches (gaps where sky shows between trees)
gapSky = pile{{"lead white", 3.5}, {"chrome yellow", 0.5}, {"vermilion", 0.15}, {"pale smalt", 0.3}}
gA = poly({{240,368},{270,366},{285,382},{245,384}}, true)
gB = poly({{530,370},{565,368},{575,385},{540,386}}, true)
gC = poly({{770,372},{800,370},{810,386},{778,387}}, true)
work(gA, {hand="detail", pile=gapSky, angle=0.0, coverage=2.0, seed=263})
work(gB, {hand="detail", pile=gapSky, angle=0.0, coverage=2.0, seed=264})
work(gC, {hand="detail", pile=gapSky, angle=0.0, coverage=2.0, seed=265})

--@ chunk 27

-- soften the pale gaps: work dark back over their edges brokenly
gapTone = pile{{"raw umber", 1.8}, {"Prussian blue", 0.5}, {"bone black", 0.3}, {"green earth", 0.5}}
e1 = ellipse(258, 375, 28, 10)
e2 = ellipse(552, 377, 26, 10)
e3 = ellipse(788, 378, 24, 9)
work(e1, {hand="body", pile=gapTone, angle=0.05, coverage=1.4, edge="soft", seed=271})
work(e2, {hand="body", pile=gapTone, angle=0.0, coverage=1.4, edge="soft", seed=272})
work(e3, {hand="body", pile=gapTone, angle=0.0, coverage=1.4, edge="soft", seed=273})
-- faint reflections under the new treeline: soft horizontal veils, low contrast
reflSoft = pile{{"Prussian blue", 0.5}, {"raw umber", 0.6}, {"lead white", 1.8}, {"green earth", 0.4}}
print(reflSoft)
rAll = rect(60, 394, 880, 50)
work(rAll, {hand="body", pile=reflSoft, angle=0.0, coverage=0.9, edge="soft", seed=274})

--@ chunk 28

veilWater = pile{{"lead white", 2.8}, {"cobalt blue", 0.6}, {"green earth", 0.45}, {"yellow ochre", 0.4}}
softBand = poly({{40,398},{300,394},{600,396},{940,394},{950,430},{700,436},{350,434},{50,430}}, true)
work(softBand, {hand="body", pile=veilWater, angle=0.0, coverage=1.6, edge="soft", fill=true, seed=281})
warmTouch = pile{{"lead white", 3.2}, {"yellow ochre", 0.7}, {"vermilion", 0.22}, {"pale smalt", 0.25}}
glintZone = poly({{300,420},{450,418},{480,480},{420,510},{310,505},{280,460}}, true)
work(glintZone, {hand="body", pile=warmTouch, angle=0.0, coverage=1.0, edge="soft", seed=282})

--@ chunk 29

birdPile = pile{{"bone black", 1}, {"raw umber", 0.6}, {"Prussian blue", 0.3}}
print(birdPile)
b = brush{kind="rigger", width=2.5, point=0.9, stiffness=0.5}
b:load(birdPile, 0.7)
-- a few distant birds, each two short strokes, placed by eye high in the sky
b:stroke({{590, 150}, {598, 146}, {606, 150}}, {pressure={0.5, 0.1}, ramps={0.15, 0.15}})
b:stroke({{630, 165}, {637, 161}, {644, 165}}, {pressure={0.45, 0.1}, ramps={0.15, 0.15}})
b:stroke({{545, 172}, {551, 169}, {557, 172}}, {pressure={0.4, 0.1}, ramps={0.15, 0.15}})
b:stroke({{680, 140}, {686, 137}, {692, 140}}, {pressure={0.4, 0.1}, ramps={0.15, 0.15}})
-- leaning foreground grasses: short diagonals along the bank
gDark = pile{{"raw umber", 1.8}, {"bone black", 0.4}, {"green earth", 0.5}}
gLight = pile{{"yellow ochre", 1.1}, {"lead white", 1.5}, {"green earth", 0.4}}
b:load(gDark, 0.7)
b:stroke({{300, 700}, {310, 670}, {318, 648}}, {pressure={0.55, 0.05}, ramps={0.1, 0.2}})
b:stroke({{700, 705}, {692, 675}, {686, 652}}, {pressure={0.55, 0.05}, ramps={0.1, 0.2}})
b:stroke({{500, 710}, {505, 680}, {510, 658}}, {pressure={0.5, 0.05}, ramps={0.1, 0.2}})
b:load(gLight, 0.7)
b:stroke({{350, 705}, {358, 678}, {364, 655}}, {pressure={0.5, 0.05}, ramps={0.1, 0.2}})
b:stroke({{650, 708}, {644, 680}, {640, 658}}, {pressure={0.5, 0.05}, ramps={0.1, 0.2}})
-- sparse bright glints on the sun path: single touches by eye
glint = pile{{"lead white", 7}, {"chrome yellow", 0.8}}
t = brush{kind="round", width=3.5, point=0.4}
t:load(glint, 0.6)
t:touch(375, 445, {pressure=0.5})
t:touch(395, 468, {pressure=0.45})
t:touch(360, 488, {pressure=0.45})
t:touch(410, 498, {pressure=0.4})
t:touch(385, 520, {pressure=0.4})

--@ chunk 30

shadowPile = pile{{"raw umber", 1.8}, {"bone black", 0.5}, {"green earth", 0.5}}
print(shadowPile)
sh1 = poly({{0,700},{150,695},{180,741},{0,741}}, true)
sh2 = poly({{820,700},{1000,695},{1000,741},{850,741}}, true)
sh3 = poly({{380,680},{620,680},{650,730},{350,730}}, true)
work(sh1, {hand="body", pile=shadowPile, angle=0.2, coverage=1.0, edge="soft", seed=291})
work(sh2, {hand="body", pile=shadowPile, angle=-0.2, coverage=1.0, edge="soft", seed=292})
work(sh3, {hand="body", pile=shadowPile, angle=0.0, coverage=0.7, edge="soft", seed=293})
-- a few crisp blades to finish
bladeDark = pile{{"raw umber", 2}, {"bone black", 0.6}}
bladeLight = pile{{"yellow ochre", 1.2}, {"lead white", 1.4}, {"chrome yellow", 0.3}}
bb = brush{kind="rigger", width=2.8, point=0.9, stiffness=0.55}
bb:load(bladeDark, 0.75)
bb:stroke({{180, 730}, {186, 700}, {192, 672}}, {pressure={0.6, 0.05}, ramps={0.1, 0.2}})
bb:stroke({{830, 732}, {826, 702}, {822, 674}}, {pressure={0.6, 0.05}, ramps={0.1, 0.2}})
bb:stroke({{600, 735}, {604, 708}, {608, 684}}, {pressure={0.55, 0.05}, ramps={0.1, 0.2}})
bb:load(bladeLight, 0.7)
bb:stroke({{220, 732}, {228, 704}, {234, 680}}, {pressure={0.55, 0.05}, ramps={0.1, 0.2}})
bb:stroke({{780, 734}, {776, 706}, {772, 682}}, {pressure={0.55, 0.05}, ramps={0.1, 0.2}})

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
