-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=850, aspect=1.5, linen={15,13}, seed=7,
  ground={
    {pile={{"red earth",4},{"yellow ochre",2},{"raw umber",1}}, um=90, apply="knife", texture=0.35},
    {pile={{"lead white",8},{"yellow ochre",1},{"red earth",0.6}}, um=45, apply="brush"}
  }
}
print("canvas ready", W, H)

--@ chunk 2
h = pencil("2H")
-- horizon / distant ridge faint ruled guide
h:rule({0, 372}, {1000, 366}, {pressure=0.18})
-- cliff edge - main foreground line, hand-drawn
h:line({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}}, {pressure=0.32})
-- central rocky nose where figure stands
h:line({{420,456},{445,500},{480,520},{520,520},{555,498},{585,454}}, {pressure=0.3})
-- left oak trunk spine
h:line({{130,667},{142,590},{155,520},{162,440},{175,360},{188,285}}, {pressure=0.35})
-- left oak main limbs (gathered from notes, each its own line)
h:line({{175,360},{110,320},{55,295},{20,280}}, {pressure=0.3})
h:line({{182,310},{250,270},{320,250},{380,245}}, {pressure=0.3})
h:line({{188,285},{200,230},{210,180}}, {pressure=0.28})
h:line({{162,440},{210,420},{260,410}}, {pressure=0.25})
-- right oak - leaner, dead top
h:line({{885,667},{870,590},{855,515},{842,430},{835,340},{828,250}}, {pressure=0.35})
h:line({{842,430},{790,400},{740,385}}, {pressure=0.28})
h:line({{835,340},{890,305},{930,285}}, {pressure=0.28})
h:line({{828,250},{810,210},{805,175}}, {pressure=0.25})
-- distant pine ridge right + ruin spire
h:line({{620,378},{680,368},{740,366},{810,364},{880,362},{950,360}}, {pressure=0.22})
h:line({{810,364},{812,330},{818,330},{820,364}}, {pressure=0.3})
h:line({{812,330},{815,312}}, {pressure=0.3})
-- far mountain left silhouette
h:line({{0,372},{60,348},{120,336},{190,344},{260,360},{320,372}}, {pressure=0.2})
-- tiny figure - head, shoulders, coat
h:line({{500,428},{500,434}}, {pressure=0.4})
h:line({{492,438},{508,438},{510,462},{506,476},{494,476},{490,462}}, {pressure=0.38})
print("underdrawing done")

--@ chunk 3
print(drawing_guide():area())

--@ chunk 4
-- piles for thin underpainting, lean and pale
pSkyLight = pile{{"lead white",7},{"pale smalt",1.2},{"yellow ochre",0.8},{"vermilion",0.25}, medium=0.35}
pSkyCool = pile{{"lead white",6},{"smalt",1.5},{"bone black",0.4},{"raw umber",0.25}, medium=0.35}
pSkyTop = pile{{"lead white",4},{"smalt",2.2},{"bone black",0.7},{"Prussian blue",0.25}, medium=0.3}
pFogLight = pile{{"lead white",8},{"pale smalt",0.9},{"yellow ochre",0.7},{"red earth",0.2}, medium=0.35}
pForeDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",1}, medium=0.25}
pForeWarm = pile{{"raw umber",3},{"yellow ochre",1.5},{"red earth",1.2}, medium=0.25}
print(pSkyLight)
print(pSkyCool)
print(pForeDark)
-- masks from drawing lines (own shapes)
skyM = above({{0,372},{1000,366}})
fogM = below({{0,372},{1000,366}}) * above({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}})
foreM = below({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}})
print("masks", skyM:area(), fogM:area(), foreM:area())
-- thin sky lay-in: cool top, light near horizon
work(skyM, {hand="broad", pile=pSkyCool, coverage=1.2, angle=0.1, seed=11})
work(skyM:times(function(x,y) return clamp((y-40)/330,0,1) end), {hand="glaze", pile=pSkyLight, coverage=1.0, angle=0.05, seed=12})
-- fog wash
work(fogM, {hand="broad", pile=pFogLight, coverage=1.2, angle=0.02, seed=13})
-- foreground thin dark
work(foreM, {hand="broad", pile=pForeWarm, coverage=1.4, angle=0.15, seed=14})
work(foreM, {hand="body", pile=pForeDark, coverage=0.8, angle=-0.2, seed=15})
print("underpainting laid")

--@ chunk 5
blend(skyM, {seed=21})
blend(fogM, {seed=22})
print("blended underpainting")

--@ chunk 6
pSkyWarm = pile{{"lead white",7},{"yellow ochre",1.4},{"vermilion",0.5},{"chrome yellow",0.5}, medium=0.4}
pSun = pile{{"lead white",5},{"chrome yellow",1.5},{"yellow ochre",1},{"vermilion",0.7}, medium=0.4}
print(pSkyWarm)
topM = above({{0,200},{1000,190}})
midM = above({{0,372},{1000,366}}) - above({{0,200},{1000,190}})
glowM = ellipse(500,335,170,55):soften(28)
sunM = ellipse(500,335,38,22):soften(10)
-- cool the zenith thinly
work(topM, {hand="glaze", pile=pSkyTop, coverage=1.0, angle=0.05, seed=31})
blend(topM, {seed=32})
-- warm the middle sky toward horizon
work(midM, {hand="glaze", pile=pSkyWarm, coverage=1.2, angle=0.02, seed=33})
blend(midM, {seed=34})
-- sun glow veils
work(glowM, {hand="glaze", pile=pSkyWarm, coverage=1.0, seed=35})
work(sunM, {hand="body", pile=pSun, coverage=1.0, seed=36})
print("sky light built")

--@ chunk 7
pCloudDark = pile{{"lead white",4},{"smalt",1.6},{"bone black",0.9},{"raw umber",0.3}, medium=0.3}
pCloudLight = pile{{"lead white",8},{"pale smalt",0.6},{"yellow ochre",0.9}, medium=0.35}
print(pCloudDark)
-- soften the hard step between zenith and warm band
seamM = rect(0,140,1000,110):soften(32)
work(seamM, {hand="glaze", pile=pSkyCool, coverage=0.9, angle=0.03, seed=41})
blend(seamM, {seed=42})
-- a few hand-drawn cloud banks, each its own ellipse, softened
c1 = ellipse(280,115,210,26):soften(14)
c2 = ellipse(720,95,240,30):soften(16)
c3 = ellipse(520,180,280,24):soften(14)
c4 = ellipse(180,220,150,18):soften(12)
c5 = ellipse(820,200,160,20):soften(12)
work(c1, {hand="body", pile=pCloudDark, coverage=0.9, angle=0.02, seed=43})
work(c2, {hand="body", pile=pCloudDark, coverage=0.9, angle=-0.02, seed=44})
work(c3, {hand="body", pile=pCloudDark, coverage=0.8, angle=0.01, seed=45})
work(c4, {hand="body", pile=pCloudLight, coverage=0.9, angle=0.02, seed=46})
work(c5, {hand="body", pile=pCloudLight, coverage=0.9, angle=-0.01, seed=47})
blend(c1+c2+c3+c4+c5, {seed=48})
-- strengthen sun veil broadly
veilM = ellipse(500,330,230,80):soften(35)
work(veilM, {hand="glaze", pile=pSkyWarm, coverage=0.9, seed=49})
print("clouds laid")

--@ chunk 8
pDistFar = pile{{"lead white",5},{"pale smalt",1.8},{"bone black",0.5},{"raw umber",0.3}, medium=0.35}
pDistMid = pile{{"lead white",4},{"smalt",1.2},{"bone black",0.8},{"raw umber",0.4}, medium=0.3}
pFogShadow = pile{{"lead white",6},{"smalt",1.0},{"raw umber",0.6},{"bone black",0.4}, medium=0.35}
pRidge = pile{{"raw umber",3},{"bone black",2.2},{"Prussian blue",0.4},{"green earth",0.6}, medium=0.3}
print(pDistFar)
farM = poly({{0,372},{60,348},{120,336},{190,344},{260,360},{320,372},{320,380},{0,380}}):soften(6)
ridgeM = poly({{610,380},{670,368},{735,366},{810,363},{950,359},{960,364},{950,380},{810,382},{670,384},{610,386}}):soften(5)
fogTopM = rect(0,360,1000,90):soften(18)
-- far blue mountain, thin, then softened by blending
work(farM, {hand="body", pile=pDistFar, coverage=1.1, angle=0.05, seed=51})
blend(farM, {seed=52})
-- nearer pine ridge right, darker
work(ridgeM, {hand="body", pile=pRidge, coverage=1.2, angle=0.03, seed=53})
-- fog modelling: shadow hollows and light billows
fShadow1 = ellipse(320,400,210,22):soften(16)
fShadow2 = ellipse(750,412,190,20):soften(16)
fLight1 = ellipse(520,390,260,24):soften(18)
work(fShadow1, {hand="glaze", pile=pFogShadow, coverage=1.0, seed=54})
work(fShadow2, {hand="glaze", pile=pFogShadow, coverage=1.0, seed=55})
work(fLight1, {hand="glaze", pile=pFogLight, coverage=1.1, seed=56})
blend(fogTopM, {seed=57})
print("distance laid")

--@ chunk 9
pMoss = pile{{"raw umber",3},{"green earth",1.5},{"yellow ochre",0.8},{"bone black",1}, medium=0.2}
pRockLight = pile{{"lead white",2.5},{"raw umber",3},{"yellow ochre",1},{"red earth",0.8}, medium=0.2}
print(pMoss)
-- close the foreground, two passes, second fills gaps
work(foreM, {hand="body", pile=pForeDark, coverage=2.2, angle=-0.15, fill=true, seed=61})
work(foreM:times(function(x,y) return clamp((y-460)/200,0,1)*0.8+0.2 end), {hand="body", pile=pForeWarm, coverage=1.2, angle=0.2, seed=62})
-- mossy lights on upper cliff
mossM = below({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}})*above({{0,580},{120,560},{270,540},{410,530},{500,520},{585,530},{720,540},{880,560},{1000,580}})
mossM = mossM:soften(8)
work(mossM, {hand="body", pile=pMoss, coverage=1.0, angle=0.1, seed=63})
-- sunlit lip along cliff edge
lipM = ribbon({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}}, 5):soften(3)
work(lipM, {hand="detail", pile=pRockLight, coverage=1.2, seed=64})
print("foreground closed")

--@ chunk 10
print("sky top", drying(500,100))
print("fog centre", drying(500,400))
print("foreground", drying(500,600))
print(wait(2*24*60))

--@ chunk 11
print(wait(5*24*60))
print("sky top", drying(500,100))
print("fog", drying(500,400))
print("fore", drying(500,600))

--@ chunk 12
pTrunkDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",0.8}, medium=0.15}
pTrunkLight = pile{{"raw umber",3},{"yellow ochre",1.2},{"lead white",1.5},{"red earth",0.5}, medium=0.15}
pTwig = pile{{"raw umber",3},{"bone black",3}, medium=0.1}
print(pTrunkDark)
-- LEFT OAK body
leftTrunk = body_of{spine={{130,667},{142,590},{155,520},{162,440},{175,360},{188,285}},
  widths={28,24,19,14,10,6.5},
  limbs={
    {pts={{175,360},{110,320},{55,295},{20,280}}, widths={9,6.5,4,2}},
    {pts={{182,310},{250,270},{320,250},{380,245}}, widths={8,5.5,3.5,1.8}},
    {pts={{188,285},{200,230},{210,180}}, widths={6,3.8,1.6}},
    {pts={{162,440},{210,420},{260,410}}, widths={7,4.5,2.2}}
  }, blend=0.85, char="firm", seed=71}
work(leftTrunk:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, angle=1.5, fill=true, seed=72})
print("left trunk mass", leftTrunk:mask():area())

--@ chunk 13
-- repair: lay sky back over ragged limbs, keeping central trunk core
repairM = rect(0,150,420,300):soften(20)
work(repairM, {hand="glaze", pile=pSkyWarm, coverage=1.2, angle=0.02, seed=81})
blend(repairM, {seed=82})
-- re-establish a slimmer, twisting trunk core with clipped detail so edges stay drawn
leftCore = body_of{spine={{134,667},{144,585},{154,515},{161,435},{172,355},{186,280},{198,220}},
  widths={26,21,17,12.5,9,6,3.5},
  limbs={}, blend=0.85, char="firm", seed=72}
work(leftCore:mask(), {hand="detail", pile=pTrunkDark, coverage=2.5, seed=83})
print("repaired left")

--@ chunk 14
print(wait(4*24*60))
print(drying(160,300), drying(500,200))

--@ chunk 15
pSkyClean = pile{{"lead white",7},{"yellow ochre",1.2},{"pale smalt",0.8},{"vermilion",0.3}, medium=0.12}
pSkyBlueClean = pile{{"lead white",5},{"smalt",1.4},{"bone black",0.35}, medium=0.12}
print(pSkyClean)
cleanM = rect(0,140,460,270):soften(24)
work(cleanM, {hand="body", pile=pSkyClean, coverage=2.0, angle=0.03, fill=true, seed=91})
blend(cleanM, {seed=92})
-- cool cloud caps back on top of cleaned area
cc1 = ellipse(180,190,170,26):soften(14)
cc2 = ellipse(120,130,150,24):soften(14)
work(cc1, {hand="body", pile=pSkyBlueClean, coverage=1.2, angle=0.02, seed=93})
work(cc2, {hand="body", pile=pSkyBlueClean, coverage=1.0, angle=0.02, seed=94})
blend(cc1+cc2, {seed=95})
print("sky cleaned")

--@ chunk 16
-- cover remaining stump with fog light (opaque-ish)
stumpM = rect(80,350,180,150):soften(18)
work(stumpM, {hand="body", pile=pFogLight, coverage=2.0, fill=true, angle=0.02, seed=101})
blend(stumpM, {seed=102})
-- soften vertical seam
seam2 = rect(380,140,140,280):soften(30)
work(seam2, {hand="glaze", pile=pSkyClean, coverage=0.8, angle=0.02, seed=103})
blend(seam2, {seed=104})
print("seam softened")

--@ chunk 17
toneM = rect(40,300,280,200):soften(35)
work(toneM, {hand="glaze", pile=pSkyWarm, coverage=1.0, seed=111})
blend(toneM, {seed=112})
bigBlend = rect(0,140,600,270):soften(40)
blend(bigBlend, {seed=113})
print("toned")

--@ chunk 18
-- cover white drip with foreground, up to edge, clipped to foreground
fixFore = rect(0,430,320,130)
work(fixFore*foreM, {hand="body", pile=pForeDark, coverage=2.0, fill=true, angle=-0.1, seed=121})
-- re-cut crisp cliff lip left
lipLeft = ribbon({{0,512},{120,492},{270,470},{410,456}}, 4):soften(2)
work(lipLeft, {hand="detail", pile=pRockLight, coverage=1.0, seed=122})
print("edge fixed")
print(wait(3*24*60))

--@ chunk 19
aboveCliff = -foreM
overM = rect(0,430,330,110):soften(10)
-- paint light back only above the cliff, clipped so it stops at the edge
work(overM*aboveCliff, {hand="body", pile=pFogLight, coverage=2.0, fill=true, clip=true, angle=0.02, seed=131})
blend(overM*aboveCliff, {seed=132})
print("overhang cleared")

--@ chunk 20
whiteStrip = rect(0,460,320,60):soften(18)
work(whiteStrip, {hand="glaze", pile=pSkyWarm, coverage=1.2, seed=141})
blend(whiteStrip, {seed=142})
darkBlock = rect(0,500,320,90):soften(20)
work(darkBlock, {hand="glaze", pile=pForeWarm, coverage=1.0, seed=143})
-- tiny white blob on lip
blobM = ellipse(285,490,22,12):soften(8)
work(blobM, {hand="body", pile=pForeDark, coverage=1.2, clip=true, seed=144})
print("harmonized")
print(wait(2*24*60))

--@ chunk 21
leftFore = rect(0,470,360,200)*foreM
leftFog = rect(0,360,360,140)*aboveCliff
-- fog first (light), clipped to stay above edge
work(leftFog, {hand="body", pile=pFogLight, coverage=2.0, fill=true, clip=true, angle=0.02, seed=151})
blend(leftFog, {seed=152})
-- foreground, clipped to stay below edge
work(leftFore, {hand="body", pile=pForeDark, coverage=2.2, fill=true, clip=true, angle=-0.1, seed=153})
work(leftFore, {hand="body", pile=pMoss, coverage=0.9, clip=true, angle=0.1, seed=154})
print("left reset")
print(drying(100,550))

--@ chunk 22
seamV = rect(280,360,160,300):soften(28)
blend(seamV, {seed=161})
print("blended seam while open")

--@ chunk 23
veilSky = rect(140,350,380,150):soften(50)
work(veilSky, {hand="glaze", pile=pSkyWarm, coverage=1.0, edge="lost", seed=171})
blend(veilSky, {seed=172})
veilFore = rect(140,480,380,180):soften(50)
work(veilFore, {hand="glaze", pile=pForeWarm, coverage=0.9, edge="lost", seed=173})
print("veiled")

--@ chunk 24
work(fogM:soften(4), {hand="broad", pile=pFogLight, coverage=1.6, angle=0.02, seed=181})
blend(fogM:soften(4), {seed=182})
work(foreM, {hand="body", pile=pForeDark, coverage=2.2, fill=true, clip=true, angle=-0.08, seed=183})
work(foreM, {hand="body", pile=pMoss, coverage=0.8, clip=true, angle=0.12, seed=184})
print("unified")
print(wait(1*24*60))

--@ chunk 25
print(wait(4*24*60))
print(drying(500,400), drying(500,600))

--@ chunk 26
pBarkLight = pile{{"raw umber",2.5},{"yellow ochre",1},{"lead white",2},{"green earth",0.5}, medium=0.15}
-- LEFT OAK, slimmer, own S-curved spine
leftM = body_of{spine={{138,667},{148,590},{156,520},{163,445},{172,370},{182,300},{192,235}},
  widths={20,17,14,11,8.5,6,4},
  limbs={}, blend=0.85, char="firm", seed=201}
work(leftM:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.52, seed=202})
print("left trunk", leftM:mask():area())

--@ chunk 27
limbA = body_of{spine={{163,445},{210,430},{260,420}}, widths={7,4.5,2.2}, limbs={}, blend=0.8, char="firm", seed=211}
limbB = body_of{spine={{172,370},{120,335},{70,315}}, widths={7.5,5,2.2}, limbs={}, blend=0.8, char="firm", seed=212}
limbC = body_of{spine={{178,330},{240,295},{310,275}}, widths={6.5,4.2,2}, limbs={}, blend=0.8, char="firm", seed=213}
limbD = body_of{spine={{192,235},{200,190},{205,155}}, widths={4.5,2.8,1.2}, limbs={}, blend=0.8, char="firm", seed=214}
work(limbA:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.1, seed=215})
work(limbB:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.25, seed=216})
work(limbC:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.15, seed=217})
work(limbD:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.4, seed=218})
print("limbs on")

--@ chunk 28
barkM = ribbon({{150,590},{158,520},{165,445},{174,370},{184,300}}, 5):soften(2)
work(barkM, {hand="detail", pile=pBarkLight, coverage=1.2, seed=221})
tw = brush{kind="rigger", width=2.6, point=1, stiffness=0.55}
tw:load(pTwig, 0.55)
tw:stroke({{70,315},{48,305},{28,298}}, {pressure={0.6,0.05}, ramps={0.1,0.4}})
tw:stroke({{260,420},{285,415},{310,412}}, {pressure={0.55,0.05}, ramps={0.1,0.4}})
tw:stroke({{310,275},{340,268},{368,264}}, {pressure={0.55,0.05}, ramps={0.1,0.4}})
tw:stroke({{205,155},{208,130},{210,108}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw:stroke({{172,370},{150,355},{130,348}}, {pressure={0.5,0.05}})
tw:stroke({{178,330},{200,318},{220,310}}, {pressure={0.5,0.05}})
print("twigs done", tw:fullness())

--@ chunk 29
rightM = body_of{spine={{872,667},{866,590},{858,515},{848,435},{840,355},{832,275}},
  widths={18,15,12.5,10,7.5,5},
  limbs={}, blend=0.85, char="firm", seed=231}
work(rightM:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.55, seed=232})
rA = body_of{spine={{848,435},{800,412},{752,398}}, widths={6.5,4.2,2}, limbs={}, blend=0.8, char="firm", seed=233}
rB = body_of{spine={{840,355},{888,328},{930,310}}, widths={6,3.8,1.8}, limbs={}, blend=0.8, char="firm", seed=234}
rC = body_of{spine={{832,275},{818,230},{812,185}}, widths={4.5,2.8,1.2}, limbs={}, blend=0.8, char="firm", seed=235}
rD = body_of{spine={{832,275},{848,235},{858,195}}, widths={4,2.5,1.1}, limbs={}, blend=0.8, char="firm", seed=236}
work(rA:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.15, seed=237})
work(rB:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.12, seed=238})
work(rC:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.45, seed=239})
work(rD:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.35, seed=240})
print("right oak mass")

--@ chunk 30
-- soften left trunk's harsh light into broken bark
breakM = ribbon({{150,590},{158,520},{165,445},{174,370}}, 7):soften(3)
stipple(breakM, {pile=pTrunkDark, width=2.2, coverage=1.5, seed=241})
-- right trunk sunlit side (faces centre sun, so west side)
barkR = ribbon({{864,590},{856,515},{846,435},{838,355}}, 4.5):soften(2)
work(barkR, {hand="detail", pile=pBarkLight, coverage=1.0, seed=242})
stipple(barkR, {pile=pTrunkDark, width=2, coverage=0.8, seed=243})
-- right twigs
tw2 = brush{kind="rigger", width=2.4, point=1, stiffness=0.55}
tw2:load(pTwig, 0.55)
tw2:stroke({{752,398},{730,392},{708,388}}, {pressure={0.55,0.05}, ramps={0.1,0.4}})
tw2:stroke({{930,310},{952,302},{972,296}}, {pressure={0.55,0.05}, ramps={0.1,0.4}})
tw2:stroke({{812,185},{808,160},{806,138}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw2:stroke({{858,195},{864,172},{868,150}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw2:stroke({{848,435},{828,425},{810,418}}, {pressure={0.5,0.05}})
tw2:stroke({{840,355},{860,345},{878,338}}, {pressure={0.5,0.05}})
print("right twigs", tw2:fullness())

--@ chunk 31
-- root flares, each own shape
flareL = body_of{spine={{138,667},{144,620}}, widths={30,20}, limbs={}, blend=0.8, char="firm", seed=251}
flareR = body_of{spine={{872,667},{868,620}}, widths={28,18}, limbs={}, blend=0.8, char="firm", seed=252}
work(flareL:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.5, seed=253})
work(flareR:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.5, seed=254})
-- extra upper limbs to fill crowns
upL1 = body_of{spine={{182,300},{150,278},{118,266}}, widths={5.5,3.5,1.6}, limbs={}, blend=0.8, char="firm", seed=255}
upL2 = body_of{spine={{186,270},{218,252},{250,242}}, widths={5,3.2,1.5}, limbs={}, blend=0.8, char="firm", seed=256}
upR1 = body_of{spine={{840,355},{812,338},{786,328}}, widths={5,3.2,1.5}, limbs={}, blend=0.8, char="firm", seed=257}
upR2 = body_of{spine={{836,315},{862,295},{886,284}}, widths={4.8,3,1.4}, limbs={}, blend=0.8, char="firm", seed=258}
work(upL1:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.2, seed=259})
work(upL2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.15, seed=260})
work(upR1:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.15, seed=261})
work(upR2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.12, seed=262})
print("aged")

--@ chunk 32
pFigure = pile{{"raw umber",3},{"bone black",4}, medium=0.1}
pRuin = pile{{"raw umber",2.5},{"bone black",2.5},{"smalt",0.6}, medium=0.2}
-- figure: coat + head as two small bodies, own shapes
figCoat = body_of{spine={{500,438},{500,458},{500,476}}, widths={10,8.5,6}, limbs={}, blend=0.7, char="firm", seed=271}
figHead = ellipse(500,431,4.5,5.5):soften(1.5)
work(figCoat:mask(), {hand="detail", pile=pFigure, coverage=2.5, seed=272})
work(figHead, {hand="detail", pile=pFigure, coverage=2.0, seed=273})
-- ruined spire on ridge, tiny
spireM = poly({{810,364},{812,332},{815,312},{818,332},{820,364}}):soften(1.5)
work(spireM, {hand="detail", pile=pRuin, coverage=2.0, seed=274})
-- a broken wall stub beside it
wallM = poly({{822,364},{822,348},{838,348},{838,364}}):soften(1.5)
work(wallM, {hand="detail", pile=pRuin, coverage=1.8, seed=275})
-- birds: three small strokes with rigger
brd = brush{kind="rigger", width=1.8, point=1, stiffness=0.6}
brd:load(pFigure, 0.5)
brd:stroke({{380,180},{385,183},{390,180}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
brd:stroke({{410,165},{415,168},{420,165}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
brd:stroke({{620,140},{625,143},{630,140}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
print("figure and ruin")

--@ chunk 33
pMute = pile{{"raw umber",3},{"bone black",1.2}, medium=0.45}
muteL = ribbon({{138,667},{148,590},{156,520},{163,445},{172,370},{182,300}}, 12):soften(4)
muteR = ribbon({{872,667},{866,590},{858,515},{848,435},{840,355}}, 11):soften(4)
work(muteL, {hand="glaze", pile=pMute, coverage=0.9, seed=281})
work(muteR, {hand="glaze", pile=pMute, coverage=0.9, seed=282})
-- fix figure head: small solid touch
figFix = brush{kind="round", width=3, point=1, stiffness=0.5}
figFix:load(pFigure, 0.6)
figFix:touch(500,431, {pressure=0.55})
figFix:touch(500,438, {pressure=0.5})
print("muted, figure fixed")

--@ chunk 34
smL1 = ellipse(170,340,70,30):soften(14)
smL2 = ellipse(160,450,60,26):soften(14)
smR1 = ellipse(845,375,120,36):soften(18)
-- exclude trunks: subtract their masks (keep globals if alive, else rebuild rough)
-- rebuild exclusion as ribbons along trunks+limbs to be safe
excl = ribbon({{138,667},{148,590},{156,520},{163,445},{172,370},{182,300},{192,235}}, 14)
  + ribbon({{872,667},{866,590},{858,515},{848,435},{840,355},{832,275}}, 13)
  + ribbon({{172,370},{120,335},{70,315}}, 10) + ribbon({{178,330},{240,295},{310,275}}, 9)
  + ribbon({{848,435},{800,412},{752,398}}, 9) + ribbon({{840,355},{888,328},{930,310}}, 9)
repM = (smL1+smL2+smR1)*aboveCliff - excl
repM = repM:soften(6)
work(repM, {hand="body", pile=pSkyClean, coverage=1.8, fill=true, clip=true, angle=0.02, seed=291})
blend(repM, {seed=292})
print("smears covered", repM:area())

--@ chunk 35
veilD = (ellipse(170,340,90,42)+ellipse(160,450,80,38)+ellipse(845,375,140,48)):soften(28)
work(veilD, {hand="glaze", pile=pSkyWarm, coverage=1.0, edge="lost", seed=301})
blend(veilD, {seed=302})
-- add faint horizontal cloud texture over discs to break them
texM = rect(80,300,300,180):soften(35)
work(texM, {hand="glaze", pile=pSkyClean, coverage=0.6, edge="lost", angle=0.02, seed=303})
blend(texM, {seed=304})
print("veiled discs")

--@ chunk 36
print(wait(4*24*60))
print(drying(160,400), drying(840,380))

--@ chunk 37
work(leftM:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.52, seed=311})
work(rightM:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.55, seed=312})
work(limbA:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=0.1, seed=313})
work(limbB:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=0.25, seed=314})
work(limbC:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=-0.15, seed=315})
work(limbD:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.4, seed=316})
work(rA:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=0.15, seed=317})
work(rB:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=-0.12, seed=318})
work(rC:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.45, seed=319})
work(rD:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.35, seed=320})
work(upL1:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=321})
work(upL2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=322})
work(upR1:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=323})
work(upR2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=324})
print("trunks recut")

--@ chunk 38
tw3 = brush{kind="rigger", width=2.2, point=1, stiffness=0.55}
tw3:load(pTwig, 0.5)
-- left crown fans
tw3:stroke({{70,315},{46,304},{24,296}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
tw3:stroke({{118,266},{96,258},{74,252}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw3:stroke({{250,242},{274,236},{298,232}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw3:stroke({{310,275},{340,268},{368,262}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
tw3:stroke({{260,420},{286,415},{312,411}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw3:stroke({{205,155},{208,130},{210,106}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
-- right crown fans
tw3:stroke({{752,398},{730,392},{708,386}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
tw3:stroke({{930,310},{952,302},{974,294}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
tw3:stroke({{786,328},{766,322},{746,318}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw3:stroke({{886,284},{908,277},{928,271}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw3:stroke({{812,185},{808,158},{806,136}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
tw3:stroke({{858,195},{864,170},{868,148}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
print("twig fans", tw3:fullness())
-- foreground stones and path: small own shapes
pStone = pile{{"raw umber",2},{"bone black",1.5},{"lead white",1.2}, medium=0.15}
s1 = ellipse(350,560,26,12):soften(4)
s2 = ellipse(620,540,20,10):soften(4)
s3 = ellipse(250,600,32,14):soften(5)
work(s1, {hand="detail", pile=pStone, coverage=1.5, seed=331})
work(s2, {hand="detail", pile=pStone, coverage=1.5, seed=332})
work(s3, {hand="detail", pile=pStone, coverage=1.2, seed=333})
print("stones")

--@ chunk 39
-- break pale discs into earthy boulders
work(s1, {hand="detail", pile=pMoss, coverage=1.2, seed=341})
work(s2, {hand="detail", pile=pMoss, coverage=1.2, seed=342})
work(s3, {hand="detail", pile=pMoss, coverage=1.2, seed=343})
stipple(s1+s2+s3, {pile=pForeDark, width=2.5, coverage=1.0, seed=344})
-- small sunlit tops (thin, clipped)
top1 = ellipse(350,555,18,6):soften(2)
top3 = ellipse(250,595,22,6):soften(2)
work(top1, {hand="detail", pile=pRockLight, coverage=1.0, seed=345})
work(top3, {hand="detail", pile=pRockLight, coverage=1.0, seed=346})
print("boulders toned")

--@ chunk 40
print(wait(3*24*60))
print(drying(160,400), drying(500,400))

--@ chunk 41
pGlow = pile{{"lead white",6},{"chrome yellow",1},{"yellow ochre",0.9},{"vermilion",0.4}, medium=0.35}
-- glow behind figure, excluding figure itself
glowC = ellipse(500,360,190,75):soften(32)
figEx = ellipse(500,454,14,28):soften(4)
work(glowC-figEx, {hand="glaze", pile=pGlow, coverage=1.0, edge="lost", seed=401})
-- fog billows: shadows and lights
shA = ellipse(340,415,200,20):soften(18)
shB = ellipse(700,425,180,18):soften(18)
liA = ellipse(520,395,220,22):soften(20)
work(shA, {hand="glaze", pile=pFogShadow, coverage=0.9, edge="lost", seed=402})
work(shB, {hand="glaze", pile=pFogShadow, coverage=0.9, edge="lost", seed=403})
work(liA, {hand="glaze", pile=pFogLight, coverage=0.9, edge="lost", seed=404})
blend(glowC+shA+shB+liA, {seed=405})
print("glow and fog modeled") 
print(wait(60))

--@ chunk 42
bandM = rect(100,380,800,80):soften(30)
blend(bandM, {seed=411})
work(bandM, {hand="glaze", pile=pSkyWarm, coverage=0.8, edge="lost", seed=412})
blend(bandM, {seed=413})
print("softened bands")

--@ chunk 43
figCoat2 = body_of{spine={{500,436},{500,456},{500,476}}, widths={9,8,5.5}, limbs={}, blend=0.7, char="firm", seed=421}
figHead2 = ellipse(500,429,4.2,5):soften(1.2)
work(figCoat2:mask()+figHead2, {hand="detail", pile=pFigure, coverage=2.5, seed=422})
-- grasses: dry tufts catching light along cliff and near stones
pGrass = pile{{"yellow ochre",2},{"raw umber",1.5},{"lead white",0.8}, medium=0.2}
gr = brush{kind="rigger", width=1.6, point=1, stiffness=0.6}
gr:load(pGrass, 0.5)
gr:stroke({{320,545},{322,530},{324,518}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr:stroke({{335,548},{338,533},{341,522}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr:stroke({{580,530},{582,516},{584,505}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr:stroke({{240,585},{242,570},{244,558}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr:stroke({{660,535},{662,521},{664,510}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr:stroke({{420,540},{422,526},{424,515}}, {pressure={0.45,0.0}, ramps={0.15,0.5}})
print("figure recut, grasses")

--@ chunk 44
print(wait(2*24*60))
print(drying(500,400))

--@ chunk 45
-- distant ridge re-affirmed, own shape, thin and soft
ridge2 = poly({{640,382},{700,372},{760,368},{820,365},{900,362},{950,360},{950,372},{900,374},{820,377},{700,384},{640,388}}):soften(4)
work(ridge2, {hand="detail", pile=pRidge, coverage=1.4, seed=441})
-- ruin spire and gable, tiny, crisp over dry mist
spire2 = poly({{812,365},{813,334},{816,314},{819,334},{820,365}}):soften(1.2)
gable2 = poly({{821,365},{821,350},{835,350},{835,365}}):soften(1.2)
work(spire2+gable2, {hand="detail", pile=pRuin, coverage=2.0, seed=442})
-- stippled light in sky for gradation (Friedrich's stippling)
stipple(skyM:times(function(x,y) return clamp((y-80)/280,0,1)*0.7+0.2 end), {pile=pSkyClean, width=2.8, coverage=0.7, seed=443})
print("distance re-affirmed")

--@ chunk 46
-- cover snow dots in sky (above fog) and black bar
snowM = above({{0,372},{1000,366}})
barM = rect(700,350,300,40):soften(10)
work(snowM, {hand="body", pile=pSkyClean, coverage=1.6, fill=true, clip=true, angle=0.03, seed=451})
-- fog over bar to erase it
work(barM*aboveCliff, {hand="body", pile=pFogLight, coverage=2.0, fill=true, clip=true, angle=0.02, seed=452})
blend(barM*aboveCliff, {seed=453})
print("snow and bar covered")

--@ chunk 47
skyFull = above({{0,372},{1000,366}}):soften(6)
work(skyFull, {hand="broad", pile=pSkyCool, coverage=1.6, angle=0.05, seed=461})
blend(skyFull, {seed=462})
midSky = (above({{0,372},{1000,366}})-above({{0,210},{1000,200}})):soften(10)
work(midSky, {hand="glaze", pile=pSkyWarm, coverage=1.2, angle=0.02, seed=463})
blend(midSky, {seed=464})
-- clouds anew, soft, few
n1 = ellipse(280,120,210,28):soften(16)
n2 = ellipse(720,100,240,32):soften(16)
n3 = ellipse(520,185,280,26):soften(16)
work(n1, {hand="body", pile=pCloudDark, coverage=1.0, angle=0.02, seed=465})
work(n2, {hand="body", pile=pCloudDark, coverage=1.0, angle=-0.02, seed=466})
work(n3, {hand="body", pile=pCloudDark, coverage=0.9, angle=0.01, seed=467})
blend(n1+n2+n3, {seed=468})
print("sky repainted")

--@ chunk 48
pZenith = pile{{"lead white",4},{"smalt",2.5},{"bone black",0.8}, medium=0.15}
zenM = above({{0,160},{1000,150}}):soften(28)
work(zenM, {hand="body", pile=pZenith, coverage=1.6, fill=true, angle=0.04, seed=471})
blend(zenM, {seed=472})
-- soften pancake clouds by veiling and blending larger
cloudVeil = rect(50,60,900,160):soften(35)
blend(cloudVeil, {seed=473})
print("zenith added")
print(wait(2*24*60))

--@ chunk 49
barCover = rect(690,345,310,45):soften(12)
work(barCover, {hand="body", pile=pFogLight, coverage=2.0, fill=true, clip=true, angle=0.02, seed=481})
blend(barCover, {seed=482})
-- pale distant ridge, thin
thinRidge = ribbon({{640,376},{720,368},{800,365},{900,361},{960,359}}, 6):soften(4)
work(thinRidge, {hand="detail", pile=pDistFar, coverage=1.4, seed=483})
-- recut tree tops (upper trunks + limbs that were erased)
topL = body_of{spine={{172,370},{182,300},{192,235},{198,170}}, widths={8.5,6,4,2.2}, limbs={}, blend=0.8, char="firm", seed=484}
topR = body_of{spine={{840,355},{832,275},{828,210}}, widths={7.5,5,2.8}, limbs={}, blend=0.8, char="firm", seed=485}
work(topL:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.5, seed=486})
work(topR:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.5, seed=487})
-- upper limbs anew (the ones erased)
work(upL1:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=488})
work(upL2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=489})
work(upR1:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=490})
work(upR2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, seed=491})
print("tops recut")

--@ chunk 50
whiteBlock = rect(790,365,210,55):soften(18)
blend(whiteBlock, {seed=501})
work(whiteBlock, {hand="glaze", pile=pSkyWarm, coverage=0.9, edge="lost", seed=502})
blend(whiteBlock, {seed=503})
-- reconnect stems through mist (full masks ensure continuity)
work(leftM:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.52, seed=504})
work(rightM:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.55, seed=505})
print("reconnected")

--@ chunk 51
mist1 = ribbon({{0,395},{250,390},{500,385},{750,380},{1000,375}}, 26):soften(18)
mist2 = ribbon({{0,425},{250,420},{500,415},{750,412},{1000,410}}, 22):soften(18)
work(mist1, {hand="glaze", pile=pFogLight, coverage=0.8, edge="lost", seed=511})
work(mist2, {hand="glaze", pile=pSkyWarm, coverage=0.8, edge="lost", seed=512})
blend(mist1+mist2, {seed=513})
print("mist banks unified")
print(wait(2*24*60))

--@ chunk 52
brd2 = brush{kind="rigger", width=1.7, point=1, stiffness=0.6}
brd2:load(pFigure, 0.5)
brd2:stroke({{385,205},{390,208},{395,205}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
brd2:stroke({{412,190},{417,193},{422,190}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
brd2:stroke({{615,175},{620,178},{625,175}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
-- boulder contact shadows, tiny, to ground them
shB1 = ellipse(350,572,26,7):soften(3)
shB3 = ellipse(250,612,32,8):soften(3)
work(shB1, {hand="detail", pile=pForeDark, coverage=1.0, seed=521})
work(shB3, {hand="detail", pile=pForeDark, coverage=1.0, seed=522})
print("birds and shadows")
print(wait(3*24*60))

--@ chunk 53
print("drying check")
print("sky", drying(500,120))
print("mid fog", drying(500,350))
print("fog bank", drying(170,430), drying(850,430))
print("fore", drying(500,600), drying(250,600))
print("trunk L", drying(170,500), drying(170,600))
print("trunk R", drying(850,500))
print("figure", drying(500,455))

--@ chunk 54
-- fresh piles, lean and thin for dry overpainting
pTrunkDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",0.8}, medium=0.15}
pBarkLight = pile{{"raw umber",2.5},{"yellow ochre",1},{"lead white",2},{"green earth",0.5}, medium=0.15}
pTwig = pile{{"raw umber",3},{"bone black",3}, medium=0.1}
pFogLight = pile{{"lead white",8},{"pale smalt",0.9},{"yellow ochre",0.7},{"red earth",0.2}, medium=0.35}
pFigure = pile{{"raw umber",3},{"bone black",4}, medium=0.1}
print(pTrunkDark)
print(pFogLight)
-- left stem continuous, own gently S-curved spine, tapered by area rule
leftStem = body_of{spine={{132,667},{142,595},{152,525},{160,450},{170,375},{180,305},{190,235},{195,180}},
  widths={22,18,15,12,9,6.5,4.5,2.5}, limbs={}, blend=0.85, char="firm", seed=601}
print("left stem", leftStem:mask():area())
work(leftStem:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.52, seed=602})
print("left reconnected")

--@ chunk 55
-- right stem, own lean, not a copy of left
rightStem = body_of{spine={{878,667},{872,595},{864,520},{855,445},{846,365},{838,285},{833,210},{831,165}},
  widths={20,17,14,11,8.5,6,4,2.5}, limbs={}, blend=0.85, char="firm", seed=603}
print("right stem", rightStem:mask():area())
work(rightStem:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.55, seed=604})
print("right reconnected")

--@ chunk 56
pSkyCover = pile{{"lead white",7},{"yellow ochre",1.2},{"pale smalt",0.8},{"vermilion",0.25}, medium=0.2}
print(pSkyCover)
-- old ghost stem on right tree, left of new stem: cover with sky
-- narrow ribbon along the ghost: from ~ (818,165) down to (825,360)
ghostR = ribbon({{818,160},{820,220},{823,280},{826,340}}, 9):soften(3)
work(ghostR, {hand="body", pile=pSkyCover, coverage=2.0, fill=true, clip=true, angle=0.05, seed=605})
print("ghost covered")

--@ chunk 57
-- cover the remaining left parallel on right tree, keep new stem
leftGhost = ribbon({{810,180},{813,250},{817,340},{820,380}}, 8):soften(2)
work(leftGhost, {hand="detail", pile=pSkyCover, coverage=2.0, fill=true, clip=true, seed=606})
print("left ghost dabbed")
-- soften the pale scar broadly so it melts into sky
softSky = rect(770,160,120,220):soften(28)
work(softSky, {hand="glaze", pile=pSkyCover, coverage=0.9, edge="lost", seed=607})
blend(softSky, {seed=608})
print("softened")

--@ chunk 58
print(wait(4*24*60))
print("right top", drying(830,280))
print("sky nearby", drying(800,250))

--@ chunk 59
-- recut right stem crisp over dry smear, no blending after
work(rightStem:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.55, seed=609})
print("right stem recut")
-- two side limbs, each own shape joining the stem
rLowLeft = body_of{spine={{855,430},{815,412},{775,398}}, widths={6.5,4.2,2}, limbs={}, blend=0.8, char="firm", seed=610}
rMidRight = body_of{spine={{846,360},{885,335},{920,318}}, widths={6,3.8,1.8}, limbs={}, blend=0.8, char="firm", seed=611}
work(rLowLeft:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.15, seed=612})
work(rMidRight:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.12, seed=613})
print("limbs rejoined")

--@ chunk 60
-- clean smear left of right trunk, excluding the trunk itself
exclR = ribbon({{878,667},{872,595},{864,520},{855,445},{846,365},{838,285},{833,210},{831,165}}, 16)
  + ribbon({{855,430},{815,412},{775,398}}, 11) + ribbon({{846,360},{885,335},{920,318}}, 10)
cleanR = rect(740,160,140,280) * -exclR
cleanR = cleanR:soften(4)
work(cleanR, {hand="body", pile=pSkyCover, coverage=1.8, fill=true, clip=true, angle=0.03, seed=614})
print("right smear cleaned, trunk kept")
-- NO blend across trunk

--@ chunk 61
-- soften the rect edges, still keeping off the trunk
edgeL = rect(700,160,90,280):soften(22)
edgeR = rect(860,160,90,280):soften(22)
-- exclude trunk from veils
veilL = edgeL * -exclR
veilR = edgeR * -exclR
work(veilL, {hand="glaze", pile=pSkyCover, coverage=0.9, edge="lost", clip=true, seed=615})
work(veilR, {hand="glaze", pile=pSkyCover, coverage=0.9, edge="lost", clip=true, seed=616})
print("edges veiled")
-- blend only the far edges, away from trunk
blend(rect(700,160,50,280):soften(10), {seed=617})
blend(rect(900,160,50,280):soften(10), {seed=618})
print("far edges blended")

--@ chunk 62
print(wait(4*24*60))
print("smear", drying(830,300), drying(800,350))

--@ chunk 63
skyBlock = rect(750,160,200,280):soften(18)
work(skyBlock, {hand="body", pile=pSkyCover, coverage=2.0, fill=true, edge="lost", angle=0.03, seed=619})
print("sky block reset, trunk sacrificed for clean ground")

--@ chunk 64
print(wait(3*24*60))
print("new sky", drying(830,300))

--@ chunk 65
rightUpper = body_of{spine={{856,465},{851,410},{846,360},{841,300},{836,240},{832,185}},
  widths={11.5,9.5,8,6,4,2.5}, limbs={}, blend=0.85, char="firm", seed=620}
work(rightUpper:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.55, seed=621})
print("upper recut")
rLowLeft2 = body_of{spine={{851,418},{815,405},{778,393}}, widths={6.5,4.2,2}, limbs={}, blend=0.8, char="firm", seed=622}
rMidRight2 = body_of{spine={{844,352},{882,330},{916,315}}, widths={6,3.8,1.8}, limbs={}, blend=0.8, char="firm", seed=623}
work(rLowLeft2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.18, seed=624})
work(rMidRight2:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.12, seed=625})
print("limbs on, no blend")

--@ chunk 66
-- fresh foreground piles
pForeDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",1}, medium=0.2}
pMoss = pile{{"raw umber",3},{"green earth",1.5},{"yellow ochre",0.8},{"bone black",1}, medium=0.2}
pStone = pile{{"raw umber",2},{"bone black",1.5},{"lead white",1.2}, medium=0.15}
pRockLight = pile{{"lead white",2.5},{"raw umber",3},{"yellow ochre",1},{"red earth",0.8}, medium=0.2}
pGrass = pile{{"yellow ochre",2},{"raw umber",1.5},{"lead white",0.8}, medium=0.2}
print(pStone)
-- sink the three coin-discs with a thin dark veil so they sit down
sink1 = ellipse(350,560,30,15):soften(5)
sink2 = ellipse(250,600,36,17):soften(5)
sink3 = ellipse(620,540,24,13):soften(5)
work(sink1, {hand="glaze", pile=pForeDark, coverage=1.0, edge="lost", seed=631})
work(sink2, {hand="glaze", pile=pForeDark, coverage=1.0, edge="lost", seed=632})
work(sink3, {hand="glaze", pile=pForeDark, coverage=1.0, edge="lost", seed=633})
print("stones sunk")
-- cover the single yellow sticks (tiny touches of fore dark)
cov = brush{kind="round", width=3, point=1, stiffness=0.5}
cov:load(pForeDark, 0.6)
-- positions from earlier: approx (320,530),(335,533),(580,518),(240,572),(660,523),(420,528) vertical sticks
cov:touch(322,530, {pressure=0.5})
cov:touch(338,533, {pressure=0.5})
cov:touch(582,518, {pressure=0.5})
cov:touch(242,572, {pressure=0.5})
cov:touch(662,523, {pressure=0.5})
cov:touch(422,528, {pressure=0.5})
cov:touch(232,602, {pressure=0.5})
print("sticks covered", cov:fullness())

--@ chunk 67
print(wait(2*24*60))
print("fore", drying(350,560), drying(250,600))

--@ chunk 68
-- cover sticks with narrow vertical ribbons of fore dark
stickM = ribbon({{210,512},{210,548}}, 5) + ribbon({{330,508},{330,544}}, 5)
  + ribbon({{345,510},{345,546}}, 5) + ribbon({{530,505},{530,540}}, 5)
  + ribbon({{660,498},{660,534}}, 5) + ribbon({{690,498},{690,534}}, 5)
work(stickM, {hand="detail", pile=pForeDark, coverage=2.0, seed=641})
print("sticks gone")
-- three boulders as own irregular polys, not ellipses
b1 = poly({{330,555},{340,550},{360,549},{375,553},{378,562},{365,568},{340,567},{328,562}}):soften(2.5)
b2 = poly({{190,600},{205,594},{230,593},{245,598},{247,608},{230,613},{200,612},{188,607}}):soften(2.5)
b3 = poly({{605,535},{615,532},{630,532},{638,536},{637,543},{622,545},{608,543}}):soften(2)
work(b1, {hand="detail", pile=pStone, coverage=1.8, seed=642})
work(b2, {hand="detail", pile=pStone, coverage=1.8, seed=643})
work(b3, {hand="detail", pile=pStone, coverage=1.6, seed=644})
print("boulders based")
-- moss on lower halves
moss1 = poly({{330,560},{345,558},{365,559},{375,562},{368,568},{340,567},{330,563}}):soften(2)
moss2 = poly({{190,605},{210,603},{235,604},{245,608},{230,613},{200,612},{190,608}}):soften(2)
work(moss1+moss2, {hand="detail", pile=pMoss, coverage=1.2, seed=645})
-- sunlit tops, thin
top1 = ribbon({{340,551},{360,550},{373,553}}, 3):soften(1.5)
top2 = ribbon({{205,595},{228,594},{242,598}}, 3):soften(1.5)
work(top1+top2, {hand="detail", pile=pRockLight, coverage=1.0, seed=646})
-- narrow contact shadows below
sh1 = ellipse(354,570,24,6):soften(3)
sh2 = ellipse(218,615,28,7):soften(3)
work(sh1+sh2, {hand="detail", pile=pForeDark, coverage=1.2, seed=647})
print("grounded")

--@ chunk 69
-- break up the dark posts and yellow sticks with foreground stipple, not more bars
-- areas around posts
postAreas = rect(180,500,30,70) + rect(315,500,55,60) + rect(520,495,25,55)
  + rect(650,490,55,60) + rect(200,560,25,60)
postAreas = postAreas:soften(4)
stipple(postAreas, {pile=pForeDark, width=3, coverage=1.2, seed=651})
stipple(postAreas, {pile=pMoss, width=3, coverage=0.9, seed=652})
print("posts broken")
-- break the big dark blob left of middle boulder
blobM = ellipse(310,572,45,18):soften(8)
stipple(blobM, {pile=pMoss, width=3.2, coverage=1.0, seed=653})
print("blob textured")

--@ chunk 70
-- opaque small patches over posts, then re-texture lightly
patch1 = rect(182,502,22,62):soften(3)
patch2 = rect(322,502,42,60):soften(3)
patch3 = rect(522,498,20,52):soften(3)
patch4 = rect(652,492,28,62):soften(3)
patch5 = rect(205,562,20,56):soften(3)
work(patch1+patch2+patch3+patch4+patch5, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, angle=-0.1, seed=661})
work(patch1+patch2+patch3+patch4+patch5, {hand="body", pile=pMoss, coverage=0.9, clip=true, angle=0.12, seed=662})
print("patches covered opaque")

--@ chunk 71
-- repaint middle boulder whole again
b1full = poly({{330,555},{340,550},{360,549},{375,553},{378,562},{365,568},{340,567},{328,562}}):soften(2.5)
work(b1full, {hand="detail", pile=pStone, coverage=1.8, seed=671})
work(b1full:soften(1), {hand="detail", pile=pMoss, coverage=0.7, seed=672})
top1re = ribbon({{340,551},{360,550},{373,553}}, 3):soften(1.5)
work(top1re, {hand="detail", pile=pRockLight, coverage=1.0, seed=673})
-- tiny opaque dabs over remaining yellow sticks
stickDots = ellipse(232,590,6,12):soften(2) + ellipse(532,525,5,12):soften(2) + ellipse(688,518,5,11):soften(2)
work(stickDots, {hand="detail", pile=pForeDark, coverage=2.0, seed=674})
print("boulder restored, sticks dabbed")

--@ chunk 72
-- larger opaque covers for the three remaining sticks
sA = rect(225,565,14,35):soften(2)
sB = rect(525,505,14,40):soften(2)
sC = rect(681,500,14,36):soften(2)
work(sA+sB+sC, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=675})
work(sA+sB+sC, {hand="body", pile=pMoss, coverage=0.8, clip=true, seed=676})
print("sticks patched opaque")

--@ chunk 73
pTest = pile{{"vermilion",3},{"lead white",1}, medium=0.2}
print(pTest)
testM = ellipse(232,580,8,8):soften(1)
print("area", testM:area())
work(testM, {hand="detail", pile=pTest, coverage=2.0, seed=677})
print("test dot placed")

--@ chunk 74
-- cover remaining sticks at corrected positions, plus remove red test dot
fixA = rect(232,562,22,42):soften(2)
fixB = rect(412,505,22,45):soften(2)
fixC = rect(570,500,22,45):soften(2)
fixD = rect(652,498,22,45):soften(2)
work(fixA+fixB+fixC+fixD, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=678})
work(fixA+fixB+fixC+fixD, {hand="body", pile=pMoss, coverage=0.8, clip=true, seed=679})
print("sticks corrected")
-- cover red dot with fore dark + moss
redCover = ellipse(232,580,10,10):soften(2)
work(redCover, {hand="detail", pile=pForeDark, coverage=2.0, seed=680})
work(redCover, {hand="detail", pile=pMoss, coverage=0.8, seed=681})
print("red gone")

--@ chunk 75
-- redefine foreground mask from cliff line
foreM2 = below({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}})
-- exclude lower trunks so they stay
exclLow = ribbon({{132,667},{142,595},{152,525}}, 30) + ribbon({{878,667},{872,595},{864,520}}, 28)
foreClean = foreM2 * -exclLow
foreClean = foreClean:soften(3)
work(foreClean, {hand="body", pile=pForeDark, coverage=2.2, fill=true, clip=true, angle=-0.08, seed=691})
work(foreClean, {hand="body", pile=pMoss, coverage=0.9, clip=true, angle=0.12, seed=692})
print("foreground reset unified")

--@ chunk 76
print(wait(3*24*60))
print("fore dry?", drying(350,570), drying(400,600))

--@ chunk 77
-- two quiet boulders, own irregular shapes, thin
bA = poly({{335,568},{345,563},{362,562},{373,566},{374,573},{360,578},{342,577},{333,572}}):soften(2.5)
bB = poly({{198,608},{212,603},{232,602},{244,606},{244,614},{228,618},{206,617},{196,612}}):soften(2.5)
work(bA, {hand="detail", pile=pStone, coverage=1.3, seed=701})
work(bB, {hand="detail", pile=pStone, coverage=1.3, seed=702})
work(bA+bB, {hand="detail", pile=pMoss, coverage=0.8, seed=703})
-- sunlit tops whisper
tA = ribbon({{345,564},{360,563}}, 2.5):soften(1.2)
tB = ribbon({{212,604},{230,603}}, 2.5):soften(1.2)
work(tA+tB, {hand="detail", pile=pRockLight, coverage=0.8, seed=704})
-- contact shadows narrow
cA = ellipse(354,580,20,5):soften(2.5)
cB = ellipse(221,620,24,6):soften(2.5)
work(cA+cB, {hand="detail", pile=pForeDark, coverage=1.0, seed=705})
print("boulders quiet")
-- grass tufts: three fans, each 3 strokes from one base, tapering to point
gr2 = brush{kind="rigger", width=1.7, point=1, stiffness=0.6}
gr2:load(pGrass, 0.5)
-- tuft near left boulder
gr2:stroke({{250,612},{248,600},{246,590}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr2:stroke({{252,612},{253,600},{254,590}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr2:stroke({{254,612},{258,601},{262,592}}, {pressure={0.45,0.0}, ramps={0.15,0.5}})
-- tuft near middle boulder
gr2:stroke({{380,570},{379,559},{378,549}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr2:stroke({{382,570},{384,559},{386,549}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
-- tuft mid-foreground right
gr2:stroke({{560,548},{559,537},{558,527}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
gr2:stroke({{562,548},{565,537},{568,528}}, {pressure={0.45,0.0}, ramps={0.15,0.5}})
print("tufts", gr2:fullness())

--@ chunk 78
-- halo around figure: cover with local fog tone, edge lost, no blend
haloM = ellipse(500,443,20,32):soften(6)
work(haloM, {hand="body", pile=pSkyCover, coverage=1.8, fill=true, edge="lost", angle=0.02, seed=711})
print("halo covered, figure sacrificed - will recut when dry")

--@ chunk 79
-- recut cliff edge through white blob: foreground only, clipped to stay below edge
blobFore = rect(465,430,70,60) * foreM2
blobFore = blobFore:soften(2)
work(blobFore, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, angle=-0.08, seed=712})
work(blobFore, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=713})
print("cliff recut, blob now only above edge")

--@ chunk 80
print(wait(3*24*60))
print("blob", drying(500,425), drying(500,460))

--@ chunk 81
pFogTone = pile{{"lead white",6},{"yellow ochre",1.4},{"pale smalt",0.7},{"vermilion",0.3}, medium=0.3}
print(pFogTone)
-- tone the white blob back to surrounding cream, above cliff only, edge lost
toneBlob = ellipse(500,425,24,22):soften(10)
aboveCliff2 = -foreM2
work(toneBlob*aboveCliff2, {hand="glaze", pile=pFogTone, coverage=1.0, edge="lost", clip=true, seed=721})
print("toned")
-- opaque foreground patch to cover streaks, clipped below edge
forePatch = rect(468,442,64,40)*foreM2
forePatch = forePatch:soften(2)
work(forePatch, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, angle=-0.08, seed=722})
work(forePatch, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=723})
print("fore streaks covered")

--@ chunk 82
print(wait(2*24*60))
print("fig ground", drying(500,430))

--@ chunk 83
-- tiny watcher recut, own shapes, no exclusion halo
figCoat = body_of{spine={{500,438},{500,456},{500,474}}, widths={9,7.5,5}, limbs={}, blend=0.7, char="firm", seed=731}
figHead = ellipse(500,431,4.2,5):soften(1.2)
work(figCoat:mask()+figHead, {hand="detail", pile=pFigure, coverage=2.5, seed=732})
print("watcher stands")

--@ chunk 84
-- erase the below-edge tail of the figure, keep only above
tailM = rect(493,445,14,32)*foreM2
tailM = tailM:soften(1.5)
work(tailM, {hand="detail", pile=pForeDark, coverage=2.0, seed=741})
work(tailM, {hand="detail", pile=pMoss, coverage=0.7, seed=742})
print("tail erased, feet at edge")

--@ chunk 85
-- bark lights on inner sides (facing pale sun), broken, thin
barkL = ribbon({{150,590},{158,520},{166,450},{174,380}}, 4):soften(1.5)
barkR = ribbon({{868,590},{860,520},{852,445},{846,380}}, 4):soften(1.5)
work(barkL, {hand="detail", pile=pBarkLight, coverage=0.9, seed=751})
work(barkR, {hand="detail", pile=pBarkLight, coverage=0.9, seed=752})
stipple(barkL, {pile=pTrunkDark, width=2, coverage=0.7, seed=753})
stipple(barkR, {pile=pTrunkDark, width=2, coverage=0.7, seed=754})
print("bark broken")
-- root flares to seat both stems
flareL = body_of{spine={{132,667},{140,630}}, widths={30,20}, limbs={}, blend=0.8, char="firm", seed=755}
flareR = body_of{spine={{878,667},{874,630}}, widths={28,19}, limbs={}, blend=0.8, char="firm", seed=756}
work(flareL:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.5, seed=757})
work(flareR:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.5, seed=758})
print("roots flared")

--@ chunk 86
-- terminal fans: a few fine twigs at each branch tip and leader, drawn as own strokes
twF = brush{kind="rigger", width=2.0, point=1, stiffness=0.55}
twF:load(pTwig, 0.5)
-- left crown: leader top + 2 branch tips + 1 inner twig
twF:stroke({{195,180},{198,155},{200,132}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
twF:stroke({{70,368},{48,358},{28,350}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
twF:stroke({{310,318},{335,310},{360,304}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
twF:stroke({{172,370},{152,358},{134,350}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
-- right crown: leader + 2 branch tips
twF:stroke({{832,185},{830,160},{829,138}}, {pressure={0.5,0.0}, ramps={0.1,0.5}})
twF:stroke({{778,393},{758,387},{738,382}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
twF:stroke({{916,315},{936,308},{956,301}}, {pressure={0.55,0.0}, ramps={0.1,0.5}})
twF:stroke({{810,405},{792,398},{774,392}}, {pressure={0.5,0.0}, ramps={0.15,0.5}})
print("fans", twF:fullness())
-- three birds anew, small, own strokes
brdF = brush{kind="rigger", width=1.7, point=1, stiffness=0.6}
brdF:load(pFigure, 0.5)
brdF:stroke({{385,225},{390,228},{395,225}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
brdF:stroke({{412,210},{417,213},{422,210}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
brdF:stroke({{615,195},{620,198},{625,195}}, {pressure={0.5,0.5}, ramps={0.2,0.2}})
print("birds", brdF:fullness())

--@ chunk 87
-- mute the birch-white bark back to a whisper of light
pMuteDark = pile{{"raw umber",3},{"bone black",1.2}, medium=0.45}
print(pMuteDark)
muteL = ribbon({{140,620},{148,540},{158,460},{168,390}}, 9):soften(3)
muteR = ribbon({{874,620},{868,545},{860,465},{852,390}}, 9):soften(3)
work(muteL, {hand="glaze", pile=pMuteDark, coverage=1.0, seed=761})
work(muteR, {hand="glaze", pile=pMuteDark, coverage=1.0, seed=762})
print("bark muted")

--@ chunk 88
print(wait(4*24*60))
print("smear dry?", drying(880,435), drying(850,500))

--@ chunk 89
-- erase the brown smear with sky, sacrificing the trunk segment inside it
smearM = rect(790,405,210,60):soften(14)
work(smearM, {hand="body", pile=pSkyCover, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=771})
print("smear erased")

--@ chunk 90
remnantM = rect(780,430,160,50):soften(14)
work(remnantM, {hand="body", pile=pSkyCover, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=772})
print("remnant covered")

--@ chunk 91
pFogTone = pile{{"lead white",6},{"yellow ochre",1.4},{"pale smalt",0.7},{"vermilion",0.3}, medium=0.3}
print(pFogTone)
bankReset = rect(680,380,320,100):soften(22)
work(bankReset, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=773})
print("bank unified to cream")

--@ chunk 92
print(wait(4*24*60))
print("bank", drying(850,420), drying(850,460))

--@ chunk 93
-- recut cliff firmly through cream bank, foreground only
cliffBand = rect(680,440,320,80)*foreM2
cliffBand = cliffBand:soften(2)
work(cliffBand, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, angle=-0.08, seed=781})
work(cliffBand, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=782})
print("cliff firm")
-- recut right trunk mid-segment + lower branch through clean bank
midTrunk = body_of{spine={{856,465},{851,410},{846,360}}, widths={11.5,9.5,8}, limbs={}, blend=0.85, char="firm", seed=783}
work(midTrunk:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.55, seed=784})
lowBranch = body_of{spine={{851,418},{815,405},{778,393}}, widths={6.5,4.2,2}, limbs={}, blend=0.8, char="firm", seed=785}
work(lowBranch:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.18, seed=786})
print("trunk and branch reconnected, no blend")

--@ chunk 94
lowerTrunkR = body_of{spine={{856,465},{860,520},{866,590},{872,660}}, widths={11.5,13,16,19}, limbs={}, blend=0.85, char="firm", seed=787}
work(lowerTrunkR:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.55, seed=788})
print("trunk to ground")

--@ chunk 95
print(drying(50,410))
print(drying(200,390))
print(drying(385,360))
-- erase three floating twigs with cream, detail only, no blend
twigE1 = ribbon({{28,412},{52,402}}, 6):soften(2)
twigE2 = ribbon({{182,398},{218,384}}, 6):soften(2)
twigE3 = ribbon({{348,372},{418,352}}, 7):soften(2)
work(twigE1, {hand="detail", pile=pFogTone, coverage=2.0, seed=791})
work(twigE2, {hand="detail", pile=pFogTone, coverage=2.0, seed=792})
work(twigE3, {hand="detail", pile=pFogTone, coverage=2.0, seed=793})
print("floaters covered")
-- soften left white bark with small dark touches, detail, no large strokes
whiteL = ribbon({{142,620},{150,540},{158,470}}, 6):soften(2)
stipple(whiteL, {pile=pTrunkDark, width=2.5, coverage=1.2, seed=794})
print("left bark quieted")

--@ chunk 96
f1 = rect(15,390,60,30):soften(8)
f2 = rect(170,375,60,30):soften(8)
f3 = rect(340,340,90,35):soften(10)
work(f1, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=795})
work(f2, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=796})
work(f3, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=797})
print("floater patches")

--@ chunk 97
g1 = rect(20,345,60,30):soften(8)
g2 = rect(125,345,55,30):soften(8)
g3 = rect(300,295,70,30):soften(8)
work(g1, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=798})
work(g2, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=799})
work(g3, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=800})
print("corrected covers")

--@ chunk 98
print(wait(3*24*60))
print("left mid", drying(175,360))

--@ chunk 99
leftMid = body_of{spine={{160,450},{170,375},{180,305}}, widths={12,9,6.5}, limbs={}, blend=0.85, char="firm", seed=801}
work(leftMid:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.52, seed=802})
print("left reconnected")
-- rejoin its two branches to the recut segment
limbLB = body_of{spine={{168,380},{120,350},{75,332}}, widths={7,4.5,2}, limbs={}, blend=0.8, char="firm", seed=803}
limbLC = body_of{spine={{176,335},{235,305},{295,288}}, widths={6.5,4,1.8}, limbs={}, blend=0.8, char="firm", seed=804}
work(limbLB:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.2, seed=805})
work(limbLC:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.15, seed=806})
print("branches rejoined, no blend")

--@ chunk 100
stipple(ribbon({{140,620},{148,540},{156,470}}, 7), {pile=pTrunkDark, width=2.5, coverage=1.0, seed=807})
print("final bark quiet, touches only")

--@ chunk 101
print(wait(4*24*60))
print("final drying", drying(175,360), drying(850,420), drying(500,445), drying(350,570))

--@ chunk 102
print("time check")
print("sky", drying(500,120))
print("mid fog", drying(500,350))
print("horizon centre", drying(500,440), drying(520,445), drying(540,445))
print("fore", drying(350,570), drying(500,600))
print("trunk L low", drying(150,600), drying(150,500))
print("trunk R low", drying(860,550))
print("orange area", drying(232,580))

--@ chunk 103
-- fresh lean piles for final dry touches (thin, as Friedrich)
pForeDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",1}, medium=0.2}
pMoss = pile{{"raw umber",3},{"green earth",1.5},{"yellow ochre",0.8},{"bone black",1}, medium=0.2}
pTrunkDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",0.8}, medium=0.15}
pFogTone = pile{{"lead white",6},{"yellow ochre",1.4},{"pale smalt",0.7},{"vermilion",0.3}, medium=0.3}
pSkyCover = pile{{"lead white",7},{"yellow ochre",1.2},{"pale smalt",0.8},{"vermilion",0.25}, medium=0.2}
print(pForeDark)
print(pFogTone)
-- redefine cliff line (same as chunk 75) to clip safely
foreM2 = below({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}})
print("masks ready")

--@ chunk 104
-- cover the small orange test remnant above left boulder, own tiny shape
orangeM = ellipse(228,590,10,9):soften(2)
work(orangeM, {hand="detail", pile=pForeDark, coverage=1.5, seed=811})
work(orangeM, {hand="detail", pile=pMoss, coverage=0.7, seed=812})
print("orange sunk")

--@ chunk 105
-- cover orange halo just north of previous dab
haloOrange = ellipse(228,578,9,8):soften(2)
work(haloOrange, {hand="detail", pile=pForeDark, coverage=1.4, seed=813})
work(haloOrange, {hand="detail", pile=pMoss, coverage=0.6, seed=814})
-- break the round dab into foreground texture
breakM = ellipse(228,584,13,12):soften(3)
stipple(breakM, {pile=pMoss, width=2.5, coverage=0.9, seed=815})
print("halo covered, dab broken")

--@ chunk 106
-- dull straw for final tufts (earthier than before)
pGrassDull = pile{{"yellow ochre",1.5},{"raw umber",2},{"green earth",0.7},{"lead white",0.4}, medium=0.2}
print(pGrassDull)
-- cover the two bright stick clusters with thin fore, own small shapes
stickL = rect(288,585,26,34):soften(2)
stickR = rect(384,560,18,36):soften(2)
work(stickL, {hand="detail", pile=pForeDark, coverage=1.5, seed=816})
work(stickR, {hand="detail", pile=pForeDark, coverage=1.5, seed=817})
work(stickL+stickR, {hand="detail", pile=pMoss, coverage=0.6, seed=818})
print("sticks sunk")

--@ chunk 107
-- texture the two dark patches back into foreground (break hard rect edges)
patchC = rect(288,585,26,34):soften(2)
patchE = rect(384,560,18,36):soften(2)
stipple(patchC, {pile=pMoss, width=3, coverage=1.0, seed=819})
stipple(patchC, {pile=pForeDark, width=3, coverage=0.8, seed=820})
stipple(patchE, {pile=pMoss, width=3, coverage=1.0, seed=821})
stipple(patchE, {pile=pForeDark, width=3, coverage=0.8, seed=822})
print("patches textured")

--@ chunk 108
-- opaque covers over bright sticks, own small shapes
coverL = rect(245,585,22,28):soften(2)
coverR = rect(380,555,16,38):soften(2)
work(coverL, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=823})
work(coverR, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=824})
work(coverL+coverR, {hand="body", pile=pMoss, coverage=0.8, clip=true, seed=825})
print("sticks covered opaque")

--@ chunk 109
-- break the blocky covers into surrounding moss pattern
stipple(rect(245,585,22,28):soften(2), {pile=pMoss, width=3, coverage=1.0, seed=826})
stipple(rect(245,585,22,28):soften(2), {pile=pForeDark, width=3, coverage=0.7, seed=827})
-- remaining single stick far right
stickFar = rect(394,552,12,38):soften(2)
work(stickFar, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=828})
work(stickFar, {hand="body", pile=pMoss, coverage=0.8, clip=true, seed=829})
print("covers broken, far stick gone")

--@ chunk 110
-- lay horizontal moss dashes over the vertical covers to match surroundings
fixM = rect(245,585,22,28) + rect(288,585,26,34) + rect(380,555,28,38)
fixM = fixM:soften(2)
work(fixM, {hand="body", pile=pMoss, coverage=0.9, clip=true, angle=0.12, seed=830})
print("dashes over")
-- taller cover for the last top stick
topStick = rect(393,545,12,30):soften(2)
work(topStick, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=831})
work(topStick, {hand="body", pile=pMoss, coverage=0.8, clip=true, seed=832})
print("top gone")

--@ chunk 111
farEast = rect(398,542,14,48):soften(2)
work(farEast, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=833})
work(farEast, {hand="body", pile=pMoss, coverage=0.9, clip=true, angle=0.12, seed=834})
print("far east covered")

--@ chunk 112
stickEdge = rect(402,538,14,52):soften(2)
work(stickEdge, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, seed=835})
work(stickEdge, {hand="body", pile=pMoss, coverage=0.8, clip=true, seed=836})
print("edge stick")

--@ chunk 113
-- fill the white notch on cliff edge, foreground only, clipped below edge
notchM = rect(683,448,18,38):soften(2)
notchFore = notchM * foreM2
work(notchFore, {hand="body", pile=pForeDark, coverage=2.0, fill=true, clip=true, angle=-0.08, seed=840})
work(notchFore, {hand="body", pile=pMoss, coverage=0.8, clip=true, angle=0.12, seed=841})
print("notch filled")

--@ chunk 114
print("notch area", (rect(683,448,18,38)*foreM2):area())
print("foreM2 at notch?", foreM2:at(692,470), foreM2:at(692,455), foreM2:at(692,445))
-- try a plain opaque dab without foreM2 to test
testNotch = rect(683,455,18,30):soften(1)
work(testNotch, {hand="detail", pile=pForeDark, coverage=2.0, seed=842})
print("test dab done")

--@ chunk 115
-- clean the dark block above the edge back to fog, keeping below-edge fill
aboveNotch = rect(683,448,18,20):soften(2) * -foreM2
work(aboveNotch, {hand="detail", pile=pFogTone, coverage=1.6, seed=843})
print("top cleaned, edge kept")
print("above area", aboveNotch:area())

--@ chunk 116
-- melt the blocky patch: split one soft rect by the cliff edge so halves meet cleanly
fixRect = rect(670,440,45,50):soften(8)
lowPart = fixRect * foreM2
highPart = fixRect * -foreM2
work(lowPart, {hand="body", pile=pForeDark, coverage=1.6, fill=true, clip=true, angle=-0.08, seed=844})
work(lowPart, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=845})
work(highPart, {hand="body", pile=pFogTone, coverage=1.6, fill=true, clip=true, angle=0.02, seed=846})
print("halves meet", lowPart:area(), highPart:area())

--@ chunk 117
-- unify mid-foreground where dark blocks stand, clipped below edge, soft so no new seams
midFore = rect(200,540,230,90):soften(18) * foreM2
work(midFore, {hand="body", pile=pForeDark, coverage=1.6, fill=true, clip=true, angle=-0.08, seed=847})
work(midFore, {hand="body", pile=pMoss, coverage=0.8, clip=true, angle=0.12, seed=848})
print("mid unified", midFore:area())

--@ chunk 118
pStone = pile{{"raw umber",2},{"bone black",1.5},{"lead white",1.2}, medium=0.15}
pRockLight = pile{{"lead white",2.5},{"raw umber",3},{"yellow ochre",1},{"red earth",0.8}, medium=0.2}
print(pStone)
-- two quiet boulders, own irregular shapes, thin (same spots as before)
bA = poly({{335,568},{345,563},{362,562},{373,566},{374,573},{360,578},{342,577},{333,572}}):soften(2.5)
bB = poly({{198,608},{212,603},{232,602},{244,606},{244,614},{228,618},{206,617},{196,612}}):soften(2.5)
work(bA, {hand="detail", pile=pStone, coverage=1.3, seed=849})
work(bB, {hand="detail", pile=pStone, coverage=1.3, seed=850})
work(bA+bB, {hand="detail", pile=pMoss, coverage=0.6, seed=851})
-- whisper tops
tA = ribbon({{345,564},{360,563}}, 2.5):soften(1.2)
tB = ribbon({{212,604},{230,603}}, 2.5):soften(1.2)
work(tA+tB, {hand="detail", pile=pRockLight, coverage=0.7, seed=852})
-- narrow shadows below
cA = ellipse(354,580,20,5):soften(2.5)
cB = ellipse(221,620,24,6):soften(2.5)
work(cA+cB, {hand="detail", pile=pForeDark, coverage=1.0, seed=853})
print("boulders back, quiet")

--@ chunk 119
-- two tiny dull tufts, each 2-3 short strokes fanning from one base, tapering to point
grDull = brush{kind="rigger", width=1.7, point=1, stiffness=0.6}
grDull:load(pGrassDull, 0.45)
-- tuft by left boulder, base (252,612), short, leaning
grDull:stroke({{252,612},{250,603},{248,595}}, {pressure={0.45,0.0}, ramps={0.15,0.5}})
grDull:stroke({{254,612},{256,603},{258,596}}, {pressure={0.4,0.0}, ramps={0.15,0.5}})
-- tuft by middle boulder, base (378,572), short
grDull:stroke({{378,572},{377,564},{376,557}}, {pressure={0.45,0.0}, ramps={0.15,0.5}})
grDull:stroke({{380,572},{383,564},{386,557}}, {pressure={0.4,0.0}, ramps={0.15,0.5}})
print("dull tufts", grDull:fullness())

--@ chunk 120
-- white blob straddling edge right of figure: split by edge so halves meet
blobRect = rect(530,438,28,32):soften(6)
blobLow = blobRect * foreM2
blobHigh = blobRect * -foreM2
work(blobLow, {hand="body", pile=pForeDark, coverage=1.8, fill=true, clip=true, angle=-0.08, seed=854})
work(blobLow, {hand="body", pile=pMoss, coverage=0.6, clip=true, angle=0.12, seed=855})
work(blobHigh, {hand="body", pile=pFogTone, coverage=1.6, fill=true, clip=true, angle=0.02, seed=856})
print("blob split", blobLow:area(), blobHigh:area())

--@ chunk 121
-- erase below-edge tail of figure, keep feet at edge, match surrounding dashes
tailM = rect(493,445,14,30):soften(1.5) * foreM2
work(tailM, {hand="detail", pile=pForeDark, coverage=1.6, seed=857})
work(tailM, {hand="detail", pile=pMoss, coverage=0.7, seed=858})
print("tail sunk", tailM:area())

--@ chunk 122
-- quiet the birch-white bark to a whisper: small dark touches only, thin
whiteRibbon = ribbon({{142,620},{150,540},{158,470}}, 7)
stipple(whiteRibbon, {pile=pTrunkDark, width=2.5, coverage=1.1, seed=859})
print("bark quieted")
-- veil the pale diagonal streak east of trunk, keeping off the stem itself
streakM = rect(165,415,60,30):soften(8) * -leftStem:mask()
work(streakM, {hand="glaze", pile=pFogTone, coverage=0.9, edge="lost", seed=860})
print("streak veiled", streakM:area())

--@ chunk 123
-- recut trunk through the accidental veil, crisp, clipped, no blend after
leftMidFix = body_of{spine={{160,450},{170,375},{180,305}}, widths={12,9,6.5}, limbs={}, blend=0.85, char="firm", seed=861}
work(leftMidFix:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.52, seed=862})
print("mid recut through veil")

--@ chunk 124
pMuteDark = pile{{"raw umber",3},{"bone black",1.2}, medium=0.45}
print(pMuteDark)
-- second quieting: a whisper glaze then dark touches, touches only
whiteLow = ribbon({{144,610},{150,540},{157,480}}, 6):soften(1.5)
work(whiteLow, {hand="glaze", pile=pMuteDark, coverage=0.8, seed=863})
stipple(whiteLow, {pile=pTrunkDark, width=2.2, coverage=1.0, seed=864})
print("whispered")

--@ chunk 125
-- soften dark smudge just west of left stem above cliff, keep off stem, clipped so no overrun
smudgeW = rect(115,450,35,35):soften(8) * -leftStem:mask() * -leftMidFix:mask()
work(smudgeW, {hand="body", pile=pFogTone, coverage=1.2, fill=true, clip=true, angle=0.02, seed=865})
print("smudge softened", smudgeW:area())

--@ chunk 126
-- final whisper on the brightest white, value-led: break white into grey bark
brightWhite = ribbon({{146,595},{151,545},{156,495}}, 5):soften(1.5)
stipple(brightWhite, {pile=pTrunkDark, width=2.0, coverage=1.2, seed=866})
print("final bark")

--@ chunk 127
print(wait(4*24*60))
print("final drying", drying(150,550), drying(692,460), drying(250,610), drying(500,445))

--@ chunk 128
print("drying check resume")
print("left patches", drying(150,380), drying(120,400))
print("left trunk", drying(175,500), drying(150,550))
print("right trunk scar", drying(850,550))
print("notch", drying(692,460), drying(690,445))
print("fig", drying(500,445), drying(500,470))
print("fore boulders", drying(280,610), drying(360,570), drying(560,550))
print("sky streak", drying(200,370), drying(750,410))

--@ chunk 129
pForeDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",1}, medium=0.2}
pMoss = pile{{"raw umber",3},{"green earth",1.5},{"yellow ochre",0.8},{"bone black",1}, medium=0.2}
pTrunkDark = pile{{"raw umber",4},{"bone black",2.5},{"red earth",0.8}, medium=0.15}
pFogTone = pile{{"lead white",6},{"yellow ochre",1.4},{"pale smalt",0.7},{"vermilion",0.3}, medium=0.3}
pSkyCover = pile{{"lead white",7},{"yellow ochre",1.2},{"pale smalt",0.8},{"vermilion",0.25}, medium=0.2}
pGrassDull = pile{{"yellow ochre",1.5},{"raw umber",2},{"green earth",0.7},{"lead white",0.4}, medium=0.2}
pStone = pile{{"raw umber",2},{"bone black",1.5},{"lead white",1.2}, medium=0.15}
print(pFogTone)
foreM2 = below({{0,512},{120,492},{270,470},{410,456},{500,444},{585,454},{720,466},{880,488},{1000,508}})
print("piles mixed, edge ready")
-- broad soft fog reset over left patchwork, above cliff only, sacrificing trunk for clean ground
leftFog = (ellipse(180,385,150,75)+ellipse(120,410,90,50)):soften(30)
leftFogAbove = leftFog * -foreM2
work(leftFogAbove, {hand="body", pile=pFogTone, coverage=2.0, fill=true, edge="lost", angle=0.02, seed=871})
print("left fog unified", leftFogAbove:area())

--@ chunk 130
-- sink pale mushroom boulders and bright V grasses while left fog rests (separate area, below edge)
sinkA = ellipse(290,600,45,20):soften(5)
sinkB = ellipse(355,565,35,16):soften(5)
sinkC = ellipse(560,555,30,18):soften(5)
work(sinkA, {hand="detail", pile=pForeDark, coverage=1.6, seed=872})
work(sinkB, {hand="detail", pile=pForeDark, coverage=1.6, seed=873})
work(sinkC, {hand="detail", pile=pForeDark, coverage=1.6, seed=874})
work(sinkA+sinkB+sinkC, {hand="detail", pile=pMoss, coverage=0.7, seed=875})
print("boulders and Vs sunk")

--@ chunk 131
-- cover far-left pale boulder remnant and the last bright Vs, tiny own shapes
farLeft = ellipse(205,612,22,10):soften(3)
work(farLeft, {hand="detail", pile=pForeDark, coverage=1.8, seed=876})
work(farLeft, {hand="detail", pile=pMoss, coverage=0.7, seed=877})
-- two Vs top-right of that patch and centre V: small opaque dabs
v1 = rect(372,572,24,20):soften(2)
v2 = rect(542,532,28,26):soften(2)
work(v1+v2, {hand="detail", pile=pForeDark, coverage=1.8, seed=878})
work(v1+v2, {hand="detail", pile=pMoss, coverage=0.6, seed=879})
print("far left and Vs covered")

--@ chunk 132
-- tiny precise touches over last pale flecks and yellow tips, then horizontal dashes to break circles
cov = brush{kind="round", width=3, point=1, stiffness=0.5}
cov:load(pForeDark, 0.6)
cov:touch(228,602, {pressure=0.55})
cov:touch(234,602, {pressure=0.5})
cov:touch(384,578, {pressure=0.5})
cov:touch(552,534, {pressure=0.5})
cov:touch(562,534, {pressure=0.5})
print("flecks touched", cov:fullness())
-- horizontal moss dashes across the round patches to melt them into surroundings
meltM = (ellipse(215,612,38,18)+ellipse(300,600,55,24)+ellipse(360,572,30,18)+ellipse(557,555,40,22)):soften(6)
work(meltM, {hand="body", pile=pMoss, coverage=0.9, clip=true, angle=0.12, seed=880})
print("melted")

--@ chunk 133
cov2 = brush{kind="round", width=3, point=1, stiffness=0.5}
cov2:load(pForeDark, 0.6)
cov2:touch(260,600, {pressure=0.5})
cov2:touch(552,534, {pressure=0.5})
cov2:touch(562,534, {pressure=0.5})
print("tips", cov2:fullness())
-- darken the pale donut ring back toward surroundings, horizontal strokes
ringM = ellipse(557,555,42,22):soften(5)
work(ringM, {hand="body", pile=pForeDark, coverage=1.0, clip=true, angle=-0.08, seed=881})
-- whisper moss back so it is not flat
work(ringM, {hand="body", pile=pMoss, coverage=0.6, clip=true, angle=0.12, seed=882})
print("ring sunk")

--@ chunk 134
-- erase pedestal column under watcher, foreground only, soft so no seams
pedM = rect(475,445,50,70):soften(10) * foreM2
work(pedM, {hand="body", pile=pForeDark, coverage=1.6, fill=true, clip=true, angle=-0.08, seed=883})
work(pedM, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=884})
print("pedestal sunk", pedM:area())
-- notch teeth: one broad soft rect split by the edge so halves meet cleanly
notchFix = rect(640,425,110,60):soften(14)
notchLow = notchFix * foreM2
notchHigh = notchFix * -foreM2
work(notchLow, {hand="body", pile=pForeDark, coverage=1.6, fill=true, clip=true, angle=-0.08, seed=885})
work(notchLow, {hand="body", pile=pMoss, coverage=0.6, clip=true, angle=0.12, seed=886})
work(notchHigh, {hand="body", pile=pFogTone, coverage=1.6, fill=true, clip=true, angle=0.02, seed=887})
print("notch split", notchLow:area(), notchHigh:area())

--@ chunk 135
print(wait(4*24*60))
print("left fog", drying(180,385), drying(150,400))
print("pedestal", drying(500,470))
print("notch", drying(685,455))
print("fore", drying(300,600), drying(557,555))

--@ chunk 136
print(wait(2*24*60))
print("fore tacky?", drying(557,555), drying(560,550))
print("left", drying(180,385))

--@ chunk 137
-- recut left mid-segment crisp through unified fog, no blend after
leftMidNew = body_of{spine={{160,450},{168,410},{173,370},{178,335},{182,305}}, widths={12,10.5,9,7.8,6.5}, limbs={}, blend=0.85, char="firm", seed=888}
work(leftMidNew:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.52, seed=889})
print("left mid recut")
-- rejoin its two limbs to the recut segment, each own shape
limbLeft = body_of{spine={{168,382},{125,355},{80,335}}, widths={7,4.5,2}, limbs={}, blend=0.8, char="firm", seed=890}
limbRight = body_of{spine={{176,338},{230,310},{290,292}}, widths={6.5,4,1.8}, limbs={}, blend=0.8, char="firm", seed=891}
work(limbLeft:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=0.2, seed=892})
work(limbRight:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=-0.15, seed=893})
print("limbs rejoined, no blend")

--@ chunk 138
-- reconnect left stem across the cliff edge gap, own gently curved spine, tapered
leftGap = body_of{spine={{160,450},{155,480},{151,510},{148,535}}, widths={12,13,14,15}, limbs={}, blend=0.85, char="firm", seed=894}
work(leftGap:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.52, seed=895})
print("gap reconnected")

--@ chunk 139
-- tiny split-fix for the pale smudge right of watcher, halves meet at edge
smudgeFix = rect(512,432,28,26):soften(7)
smLow = smudgeFix * foreM2
smHigh = smudgeFix * -foreM2
work(smLow, {hand="detail", pile=pForeDark, coverage=1.5, seed=896})
work(smLow, {hand="detail", pile=pMoss, coverage=0.6, seed=897})
work(smHigh, {hand="detail", pile=pFogTone, coverage=1.5, seed=898})
print("smudge split", smLow:area(), smHigh:area())

--@ chunk 140
print(wait(3*24*60))
print("gap", drying(155,490), drying(150,510))
print("lower bark", drying(150,580))
print("right scar", drying(850,550))

--@ chunk 141
pMuteDark = pile{{"raw umber",3},{"bone black",1.2}, medium=0.45}
print(pMuteDark)
-- whisper the birch-white patches to grey, touches only, thin
whiteLow = ribbon({{142,620},{148,560},{152,510}}, 7):soften(2)
stipple(whiteLow, {pile=pTrunkDark, width=2.2, coverage=1.1, seed=899})
work(whiteLow, {hand="glaze", pile=pMuteDark, coverage=0.8, seed=900})
stipple(whiteLow, {pile=pTrunkDark, width=2.0, coverage=0.9, seed=901})
print("left whispered")
-- right scar, own small shape
scarR = ellipse(852,558,10,22):soften(3)
stipple(scarR, {pile=pTrunkDark, width=2.2, coverage=1.1, seed=902})
work(scarR, {hand="glaze", pile=pMuteDark, coverage=0.8, seed=903})
print("right whispered")

--@ chunk 142
-- melt the round stipple halos beside right stem back into horizontal foreground
haloMelt = (ellipse(825,555,32,28)+ellipse(870,585,28,26)):soften(8)
work(haloMelt, {hand="body", pile=pForeDark, coverage=1.2, clip=true, angle=-0.08, seed=904})
work(haloMelt, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=905})
print("halos melted")
-- tiny touches over last two bright flecks on left stem
t = brush{kind="round", width=2.5, point=1, stiffness=0.5}
t:load(pTrunkDark, 0.6)
t:touch(156,545, {pressure=0.5})
t:touch(157,582, {pressure=0.5})
print("flecks", t:fullness())

--@ chunk 143
-- recut right mid-segment sacrificed by melt, own short shape, no blend
rightMidSmall = body_of{spine={{845,500},{848,540},{851,575}}, widths={10,11,12}, limbs={}, blend=0.85, char="firm", seed=906}
work(rightMidSmall:mask(), {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.55, seed=907})
print("right mid back")
-- tiny touches a touch east for left flecks
te = brush{kind="round", width=2.5, point=1, stiffness=0.5}
te:load(pTrunkDark, 0.65)
te:touch(162,543, {pressure=0.55})
te:touch(163,581, {pressure=0.55})
print("flecks east", te:fullness())

--@ chunk 144
-- erase the misaligned recut (too far west), foreground only
wrongSeg = ribbon({{845,500},{848,540},{851,575}}, 14):soften(2)
work(wrongSeg, {hand="detail", pile=pForeDark, coverage=1.8, seed=908})
work(wrongSeg, {hand="detail", pile=pMoss, coverage=0.6, seed=909})
print("wrong erased")
-- tiny opaque dabs over the two bright flecks on left stem
fleckA = ellipse(162,543,4,6):soften(1.5)
fleckB = ellipse(163,581,4,6):soften(1.5)
work(fleckA+fleckB, {hand="detail", pile=pTrunkDark, coverage=2.0, seed=910})
print("flecks dabbed")

--@ chunk 145
-- correct positions from grid: left flecks on trunk edge, not east in field
fleckA2 = ellipse(154,540,4,6):soften(1.2)
fleckB2 = ellipse(155,585,4,6):soften(1.2)
work(fleckA2+fleckB2, {hand="detail", pile=pTrunkDark, coverage=2.2, seed=911})
print("flecks at trunk")
-- melt the vertical streak west of right stem back into horizontals
streakW = rect(815,515,30,90):soften(6)
work(streakW, {hand="body", pile=pForeDark, coverage=1.2, clip=true, angle=-0.08, seed=912})
work(streakW, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=913})
print("streak melted")

--@ chunk 146
-- broader opaque covers across full trunk width at the two fleck heights
bandA = ribbon({{147,530},{149,540},{151,550}}, 10):soften(1.5)
bandB = ribbon({{149,575},{151,585},{153,595}}, 10):soften(1.5)
work(bandA+bandB, {hand="detail", pile=pTrunkDark, coverage=2.2, seed=914})
print("bands covered")

--@ chunk 147
pDarkOpaque = pile{{"raw umber",3},{"bone black",4}, medium=0.1}
print(pDarkOpaque)
-- generous bands across trunk at fleck zone to be sure to hit them
testBand = rect(138,528,28,72):soften(2)
print("test area", testBand:area())
work(testBand, {hand="detail", pile=pDarkOpaque, coverage=2.5, seed=915})
print("test covered")

--@ chunk 148
-- turn the black test block back to trunk brown, vertical strokes, then cover upper flecks
blackM = rect(138,528,28,72):soften(2)
work(blackM, {hand="body", pile=pTrunkDark, coverage=2.0, fill=true, clip=true, angle=1.52, seed=916})
print("black to brown")
upperFleck = rect(140,505,24,28):soften(2)
work(upperFleck, {hand="detail", pile=pTrunkDark, coverage=2.2, seed=917})
print("upper covered")

--@ chunk 149
bigUp = rect(130,500,50,40):soften(2)
print("area", bigUp:area())
work(bigUp, {hand="detail", pile=pTrunkDark, coverage=2.5, seed=918})
print("big up covered")

--@ chunk 150
-- recut trunk narrow through the blocky covers so trunk edges replace rect edges
trunkThrough = body_of{spine={{152,498},{150,540},{148,580},{144,615}}, widths={13,14,15,17}, limbs={}, blend=0.85, char="firm", seed=919}
work(trunkThrough:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.52, seed=920})
print("trunk through blocks")

--@ chunk 151
-- erase the old thin edge west of the recut, keeping the recut itself
oldWest = ribbon({{135,505},{130,560},{126,620}}, 10):soften(2) * -trunkThrough:mask()
work(oldWest, {hand="detail", pile=pForeDark, coverage=1.8, seed=921})
work(oldWest, {hand="detail", pile=pMoss, coverage=0.6, seed=922})
print("old west erased", oldWest:area())
-- melt the east spill similarly, keeping trunk
eastSpill = rect(160,500,30,115):soften(5) * -trunkThrough:mask()
work(eastSpill, {hand="body", pile=pForeDark, coverage=1.2, clip=true, angle=-0.08, seed=923})
work(eastSpill, {hand="body", pile=pMoss, coverage=0.7, clip=true, angle=0.12, seed=924})
print("east melted")

--@ chunk 152
print(wait(3*24*60))
print("left low", drying(150,560), drying(145,600))
print("right gap", drying(855,550))

--@ chunk 153
print(wait(3*24*60))
print("left", drying(150,560))
print("right", drying(855,550))

--@ chunk 154
leftLowerFinal = body_of{spine={{152,498},{148,550},{140,600},{133,660}}, widths={13,15,18,24}, limbs={}, blend=0.85, char="firm", seed=925}
work(leftLowerFinal:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.52, seed=926})
print("left lower final")
rightLowerFinal = body_of{spine={{850,495},{855,545},{862,595},{870,660}}, widths={11,13,16,20}, limbs={}, blend=0.85, char="firm", seed=927}
work(rightLowerFinal:mask(), {hand="body", pile=pTrunkDark, coverage=2.2, fill=true, clip=true, angle=1.55, seed=928})
print("right lower final, no blend")

--@ chunk 155
print(wait(5*24*60))
print(" lowers", drying(150,560), drying(855,550))
print(" mids", drying(170,380), drying(500,445))

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
