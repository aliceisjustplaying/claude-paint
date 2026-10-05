-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=760, aspect=1.5, linen={16,14}, seed=1888,
 ground={{pile={{"lead white",10},{"yellow ochre",0.3}}, um=120, apply="knife", texture=0.35}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
stain = pile{{"raw umber",3},{"raw sienna",1}, thinner=0.6, medium=0.1}
work(everywhere(), {hand="broad", pile=stain, coverage=1.6, angle=function(x,y) return 0.3*math.sin(y/90) end})
r = rag{width=70}
r:wipe(everywhere(), {pressure=0.45, angle=0.2, passes=1, refold=0.5})

--@ chunk 3
stain2 = pile{{"raw umber",4},{"raw sienna",1}, thinner=0.45, medium=0.1}
work(everywhere(), {hand="broad", pile=stain2, coverage=1.5, fill=true, angle=function(x,y) return -0.2+0.3*math.sin(x/150) end})
r:refold()
r:wipe(everywhere(), {pressure=0.3, angle=-0.1, passes=1, refold=0.5})

--@ chunk 4
print(wait(20*60))
print(drying(500,300))

--@ chunk 5
c = chalk()
-- far tree line
c:line({{0,378},{60,370},{120,374},{190,362},{240,368},{320,376},{420,380},{520,384},{580,386}}, {pressure=0.35})
-- big tree group
c:sketch({{585,410},{592,320},{610,240},{650,170},{700,110},{760,85},{820,95},{870,140},{895,210},{930,270},{955,340},{990,395}}, {pressure=0.4})
c:line({{700,410},{705,340},{712,290}}, {pressure=0.35})
c:line({{790,415},{795,330},{800,270}}, {pressure=0.35})
-- pond
c:sketch({{210,430},{300,418},{450,414},{600,418},{690,428},{620,452},{470,466},{330,462},{230,448},{210,430}}, {pressure=0.3})
-- path
c:line({{290,667},{340,600},{400,540},{450,495},{475,472}}, {pressure=0.3})
c:line({{380,667},{420,600},{460,530},{485,480}}, {pressure=0.25})
-- figure
c:line({{468,472},{466,450},{469,438}}, {pressure=0.5})
-- house
c:line({{150,372},{150,358},{168,348},{186,358},{186,372}}, {pressure=0.35, smooth=false})

--@ chunk 6
horizon = {{0,380},{60,372},{120,375},{190,364},{240,370},{320,378},{420,382},{520,386},{600,390},{700,395},{800,398},{1000,400}}
treesM = poly({{575,420},{585,330},{596,260},{625,200},{660,150},{705,108},{760,84},{822,92},{868,132},{892,195},{925,255},{955,330},{985,390},{1000,400},{1000,425}}, true):roughen(14, 40, 3)
farM = (below({{0,382},{60,372},{120,377},{190,366},{240,372},{320,380},{420,384},{520,388},{600,392}}) * above(function(x) return 402 end)):roughen(5,25,4)
pondM = poly({{205,432},{280,420},{420,414},{560,416},{660,424},{690,432},{640,448},{520,462},{400,466},{300,460},{230,448}}, true):roughen(4,30,5)
groundM = below(function(x) return 395 + 0.01*x end)
skyM = -groundM
lay_dark = pile{{"raw umber",3},{"bone black",1}, thinner=0.5, medium=0.1}
lay_warm = pile{{"raw umber",2},{"raw sienna",2}, thinner=0.5, medium=0.1}
work(treesM, {hand="body", tool="filbert 14", pile=lay_dark, coverage=2.2, fill=true, angle=function(x,y) return -1.2 + 0.5*math.sin(x/40+y/50) end, scrub=0.5})
work(groundM - pondM, {hand="broad", pile=lay_warm, coverage=1.8, fill=true, angle=0.05})
work(farM, {hand="body", pile=lay_dark, coverage=1.5, angle=0})

--@ chunk 7
lay_dark2 = pile{{"raw umber",3},{"bone black",1.5}, thinner=0.2, medium=0.15}
lay_warm2 = pile{{"raw umber",3},{"raw sienna",2},{"bone black",0.5}, thinner=0.25, medium=0.15}
work(treesM, {hand="body", tool="filbert 14", pile=lay_dark2, coverage=2.5, fill=true, angle=function(x,y) return -1.2 + 0.5*math.sin(x/40+y/50) end, scrub=0.5, edge="soft"})
local fg = (groundM - pondM)
work(fg, {hand="broad", pile=lay_warm2, coverage=2, fill=true, angle=0.05, load_at=function(x,y) return 0.5 + 0.5*smoothstep(400,667,y) end})
work(farM, {hand="body", pile=lay_dark2, coverage=2, angle=0, edge="soft"})

--@ chunk 8
local fgLow = below(function(x) return 480 + 40*math.sin(x/170) end):soften(40)
work(fgLow - pondM, {hand="broad", pile=lay_dark2, coverage=1.5, angle=-0.05, load_at=function(x,y) return 0.3+0.6*smoothstep(470,667,y) end})
-- a darker band of near bank under the trees and in front of the pond
local bank = poly({{540,410},{1000,415},{1000,470},{700,455},{600,445}}, true):roughen(10,40,7)
work(bank, {hand="body", pile=lay_dark2, coverage=1.8, angle=0, edge="soft"})
-- wipe the path and pond lights
r:refold(); r:refold()
r:wipe({{320,667},{370,600},{420,545},{460,500},{478,476}}, {pressure={0.6,0.3}})
r:wipe({{240,437},{400,428},{560,428},{660,433}}, {pressure=0.6})
r:wipe({{260,447},{420,446},{600,440}}, {pressure=0.5})

--@ chunk 9
print(wait(24*60)); print(drying(750,250), drying(400,600))

--@ chunk 10
print(wait(36*60)); print(drying(750,250), drying(400,600), drying(700,430))

--@ chunk 11
skyTop = pile{{"lead white",5},{"cobalt blue",1},{"bone black",0.35},{"Indian red",0.25}, medium=0.1}
skyMid = pile{{"lead white",6},{"cobalt blue",0.35},{"yellow ochre",0.5},{"Indian red",0.2},{"bone black",0.1}, medium=0.1}
skyLow = pile{{"lead white",8},{"yellow ochre",0.6},{"lemon chrome",0.4},{"Indian red",0.05}, medium=0.1}
glow = pile{{"lead white",9},{"lemon chrome",1.1},{"cadmium yellow",0.15}, medium=0.1}
skyArea = above(function(x) return 392 + 0.01*x end)
local top = skyArea * above(function(x) return 150 + 20*math.sin(x/200) end)
local mid = skyArea * below(function(x) return 135 + 20*math.sin(x/200) end) * above(function(x) return 285 + 15*math.sin(x/130) end)
local low = skyArea * below(function(x) return 270 + 15*math.sin(x/130) end)
local hz = function(x,y) return 0.04*math.sin(x/120+y/60) end
work(top, {hand="broad", pile=skyTop, coverage=2, fill=true, angle=hz})
work(mid, {hand="broad", pile=skyMid, coverage=2, fill=true, angle=hz})
work(low, {hand="broad", pile=skyLow, coverage=2, fill=true, angle=hz})
local glowM = ellipse(330, 360, 260, 55):soften(30) * skyArea
work(glowM, {hand="body", tool="filbert 16", pile=glow, coverage=1.8, fill=true, angle=hz, edge="lost"})

--@ chunk 12
blend(skyArea, {angle=0.03})
blend(skyArea * below(function(x) return 100 end), {angle=-0.05})

--@ chunk 13
cloud = pile{{"lead white",4},{"cobalt blue",0.5},{"Indian red",0.35},{"bone black",0.25},{"yellow ochre",0.3}, medium=0.15}
local b = brush("filbert", 18)
local streaks = {
 {{-10,60},{200,50},{420,70},{600,55}},
 {{300,120},{520,110},{760,125},{1010,105}},
 {{-10,200},{180,190},{360,205},{520,198}},
 {{120,248},{300,240},{480,252},{620,244}},
 {{560,175},{760,168},{1010,180}},
}
for i,s in ipairs(streaks) do
  b:load(cloud, 0.6)
  b:stroke(s, {pressure={0.5,0.25}, ramps={0.2,0.4}, shake=1.5, swell={0.7,1.2,0.6}})
end
b:reload(glow, 0.8)
for i=1,6 do
  local y = 335 + i*8
  b:stroke({{60+i*10,y},{250,y-3+i},{450,y+2},{560-i*5,y}}, {pressure=0.6, ramps={0.2,0.3}, shake=1})
  b:load(glow,0.5)
end

--@ chunk 14
blend(skyArea * above(function(x) return 300 end), {angle=0.02})
local hz = function(x,y) return 0.05*math.sin(x/90+y/40) end
work(skyArea * above(function(x) return 300 end), {hand="scumble", pile=skyMid, coverage=0.4, angle=hz, pressure={0.2,0.4}, load=0.3})

--@ chunk 15
blend(skyArea * above(function(x) return 310 end), {angle=0.0})
blend(skyArea * above(function(x) return 310 end), {angle=0.08})

--@ chunk 16
local band = (below(function(x) return 270 end) * above(function(x) return 350 end)):soften(25)
blend(band, {angle=0.0})
blend(band, {angle=1.4})

--@ chunk 17
print(wait(9*60)); print(drying(300,100), drying(300,350))

--@ chunk 18
treeDark = pile{{"raw umber",3},{"bone black",1},{"Antwerp blue",0.3},{"yellow ochre",0.6}, medium=0.12}
treeMid = pile{{"raw umber",2},{"bone black",0.5},{"yellow ochre",1},{"lead white",0.6},{"Antwerp blue",0.2}, medium=0.12}
-- tree crowns as their own drawn shapes
elm = outline{{715,400},{712,330},{690,300},{660,250},{668,190},{700,140},{735,95},{780,72},{830,84},{862,120},{880,170},{872,230},{850,280},{820,320},{800,345},{795,400}, closed=true, char="soft", seed=11, lobe=22}
oak = outline{{585,405},{580,350},{592,300},{615,262},{650,240},{690,250},{715,290},{720,340},{712,405}, closed=true, char="soft", seed=12, lobe=18}
rgt = outline{{840,405},{850,330},{880,280},{905,230},{935,210},{970,220},{1005,250},{1005,405}, closed=true, char="soft", seed=13, lobe=20}
treeAll = elm:mask() + oak:mask() + rgt:mask()
work(treeAll, {hand="body", tool="filbert 10", pile=treeDark, coverage=2.2, fill=true, edge="soft", angle=function(x,y) return -1.3+0.7*math.sin(x/30+y/45) end})

--@ chunk 19
local litSide = treeAll:shrink(10) * mask(function(x,y) if x < 700 and y > 230 then return 1 elseif x < 760 and y < 230 then return 0.8 else return 0 end end):blur(30)
stipple(litSide, {pile=treeMid, width=7, coverage=0.7, cluster={0.7, 30}, feather=0.5, pressure={0.3,0.6}})
local core = (elm:mask():shrink(30) + rgt:mask():shrink(30)):blur(15)
stipple(core, {pile=pile{{"raw umber",2},{"bone black",1.5},{"Antwerp blue",0.3}, medium=0.15}, width=8, coverage=0.6, cluster={0.6,40}, feather=0.5})

--@ chunk 20
skyHole = pile{{"lead white",6},{"cobalt blue",0.3},{"yellow ochre",0.45},{"Indian red",0.15},{"bone black",0.12}, medium=0.1}
local rimM = treeAll:rim(18, 8) * above(function(x) return 330 end)
stipple(rimM, {pile=skyHole, width=6, coverage=0.35, cluster={0.8, 20}, pressure={0.3,0.6}, drag=2})
-- a few holes inside the crowns
for i, p in ipairs({{760,130},{800,170},{735,200},{830,210},{700,260},{905,260},{950,240},{860,290},{680,320},{780,250}}) do
  local hb = brush("round", rand(3,6))
  hb:load(skyHole, 0.5)
  for k=1,math.random(2,4) do
    hb:touch(p[1]+rand(-8,8), p[2]+rand(-8,8), {pressure=rand(0.3,0.6), drag={rand(-2,2), rand(-2,2)}})
  end
end

--@ chunk 21
blend(treeAll:rim(14, 10) * above(function(x) return 380 end), {angle=-1.2})

--@ chunk 22
farTree = pile{{"lead white",3},{"cobalt blue",0.5},{"Indian red",0.35},{"raw umber",0.8},{"yellow ochre",0.3}, medium=0.12}
farTree2 = pile{{"lead white",2},{"cobalt blue",0.4},{"Indian red",0.3},{"raw umber",1.2},{"bone black",0.2}, medium=0.12}
farLine = outline{{-10,404},{-10,372},{30,366},{70,360},{100,350},{125,340},{150,345},{175,338},{205,350},{235,360},{270,368},{320,372},{380,378},{440,381},{500,384},{560,386},{600,388},{600,404}, closed=true, char="soft", seed=21, lobe=10}
work(farLine:mask(), {hand="body", tool="filbert 7", pile=farTree, coverage=2, fill=true, edge="soft", angle=function(x,y) return -1.4+0.4*math.sin(x/20) end})
-- darker clump around the farmhouse
local clump = outline{{90,402},{95,360},{110,345},{130,335},{150,340},{165,346},{175,360},{178,402}, closed=true, char="soft", seed=22, lobe=8}
work(clump:mask(), {hand="body", tool="filbert 5", pile=farTree2, coverage=1.6, fill=true, edge="soft", angle=-1.4})
-- strip of ground under the far trees to the pond
farField = pile{{"lead white",2},{"yellow ochre",0.8},{"raw umber",0.8},{"cobalt blue",0.15},{"Indian red",0.1}, medium=0.1}
local ff = poly({{-10,396},{600,398},{700,410},{700,425},{200,430},{-10,430}}) - pondM
work(ff, {hand="body", pile=farField, coverage=1.8, fill=true, angle=0.02, edge="soft"})

--@ chunk 23
blend(farLine:mask():grow(6):soften(6), {angle=0.1})
blend(farLine:mask():grow(6):soften(6), {angle=-1.2})

--@ chunk 24
pondLight = pile{{"lead white",8},{"lemon chrome",0.8},{"yellow ochre",0.3}, medium=0.12}
pondGray = pile{{"lead white",4},{"cobalt blue",0.3},{"yellow ochre",0.4},{"raw umber",0.4},{"Indian red",0.1}, medium=0.12}
pond = outline{{205,432},{260,424},{340,419},{440,416},{540,417},{620,421},{680,428},{660,440},{600,450},{510,460},{410,464},{320,460},{250,450}, closed=true, char="firm", seed=31, amount=0.6}
pondMask = pond:mask()
work(pondMask, {hand="body", tool="flat 8", pile=pondGray, coverage=1.6, fill=true, angle=0, clip=true})
local core = ellipse(380, 438, 150, 14):soften(10) * pondMask
work(core, {hand="body", tool="flat 8", pile=pondLight, coverage=1.8, fill=true, angle=0, clip=true})
blend(pondMask, {angle=0})

--@ chunk 25
mead1 = pile{{"yellow ochre",2},{"raw umber",2},{"bone black",0.6},{"lead white",0.4}, medium=0.12}
mead2 = pile{{"raw umber",3},{"bone black",1},{"yellow ochre",1},{"raw sienna",0.5}, medium=0.15}
meadLight = pile{{"yellow ochre",2},{"lead white",1.5},{"raw umber",0.8},{"raw sienna",0.4}, medium=0.12}
local fgAll = below(function(x) return 425 + 0.02*x end) - pondMask
local hz = function(x,y) return 0.04*math.sin(x/80) - 0.03 end
-- middle meadow, mid olive
work(fgAll * above(function(x) return 540 end), {hand="body", tool="filbert 12", pile=mead1, coverage=1.8, fill=true, angle=hz, edge="soft"})
-- lower foreground darker
work(fgAll * below(function(x) return 520 + 30*math.sin(x/140) end), {hand="broad", pile=mead2, coverage=2, fill=true, angle=hz})

--@ chunk 26
bankDark = pile{{"raw umber",3},{"bone black",1.2},{"yellow ochre",0.6},{"Antwerp blue",0.15}, medium=0.15}
local bankM = outline{{560,445},{580,412},{640,405},{720,402},{800,405},{880,402},{960,404},{1010,404},{1010,470},{900,462},{800,455},{700,448}, closed=true, char="soft", seed=41, lobe=14}:mask()
work(bankM, {hand="body", tool="filbert 9", pile=bankDark, coverage=2, fill=true, angle=0.02, edge="soft"})
-- trunks
local tb = brush{kind="round", width=7, point=0.5}
tb:load(bankDark, 0.8)
tb:stroke({{748,412},{746,370},{742,335},{735,305}}, {pressure={0.9,0.4}})
tb:load(bankDark, 0.8)
tb:stroke({{652,414},{650,385},{646,360}}, {pressure={0.8,0.4}})
tb:load(bankDark, 0.8)
tb:stroke({{885,412},{884,380},{888,350}}, {pressure={0.8,0.4}})
tb:load(bankDark, 0.6)
tb:stroke({{770,410},{774,375},{785,345}}, {pressure={0.6,0.2}})

--@ chunk 27
print(wait(5*24*60)); print(drying(300,600), drying(750,200), drying(400,440), drying(300,100))

--@ chunk 28
print(wait(3*24*60)); print(drying(300,600), drying(750,200), drying(400,440), drying(300,100))

--@ chunk 29
s_top = pile{{"lead white",4},{"cobalt blue",1},{"bone black",0.45},{"Indian red",0.35},{"raw umber",0.2}, medium=0.12}
s_rose = pile{{"lead white",6},{"Indian red",0.35},{"cobalt blue",0.3},{"yellow ochre",0.4}, medium=0.12}
s_gold = pile{{"lead white",6},{"yellow ochre",0.8},{"lemon chrome",0.6},{"orange chrome",0.12}, medium=0.12}
s_glow = pile{{"lead white",7},{"lemon chrome",1.4},{"cadmium yellow",0.2}, medium=0.1}
sky2 = above(function(x) return 400 end) - treeAll:shrink(12)
local hz = function(x,y) return 0.05*math.sin(x/110+y/50) end
work(sky2 * above(function(x) return 130+25*math.sin(x/160) end), {hand="broad", pile=s_top, coverage=1.8, fill=true, angle=hz})
work(sky2 * below(function(x) return 110+25*math.sin(x/160) end) * above(function(x) return 240+20*math.sin(x/120) end), {hand="broad", pile=s_rose, coverage=1.8, fill=true, angle=hz})
work(sky2 * below(function(x) return 225+20*math.sin(x/120) end) * above(function(x) return 320 end), {hand="broad", pile=s_gold, coverage=1.8, fill=true, angle=hz})
work(sky2 * below(function(x) return 305 end) * mask(function(x,y) return x < 640 and 1 or 0 end), {hand="body", tool="filbert 16", pile=s_glow, coverage=2, fill=true, angle=hz})

--@ chunk 30
local skyB = above(function(x) return 405 end)
blend(skyB, {angle=0.02})
blend(skyB * below(function(x) return 90 end), {angle=0.06})

--@ chunk 31
farLine2 = outline{{-10,410},{-10,378},{30,372},{70,366},{100,356},{125,346},{150,350},{175,344},{205,356},{235,366},{270,374},{320,378},{380,383},{440,386},{500,389},{560,391},{620,393},{620,410}, closed=true, char="soft", seed=51, lobe=9}
work(farLine2:mask(), {hand="body", tool="filbert 7", pile=farTree, coverage=2, fill=true, edge="soft", angle=function(x,y) return -1.45+0.3*math.sin(x/17) end})
local clump = outline{{95,408},{98,368},{112,352},{132,342},{152,346},{166,354},{176,368},{180,408}, closed=true, char="soft", seed=52, lobe=7}
work(clump:mask(), {hand="body", tool="filbert 5", pile=farTree2, coverage=1.6, fill=true, edge="soft", angle=-1.4})

--@ chunk 32
elm2 = outline{{722,412},{716,350},{695,318},{662,280},{655,225},{672,170},{705,125},{742,92},{790,76},{835,90},{866,128},{884,178},{880,236},{858,285},{830,322},{808,350},{800,412}, closed=true, char="soft", seed=61, lobe=20}
oak2 = outline{{580,412},{574,365},{585,320},{606,285},{640,262},{678,268},{705,300},{716,350},{712,412}, closed=true, char="soft", seed=62, lobe=16}
rgt2 = outline{{842,412},{852,345},{876,295},{902,248},{935,226},{970,232},{1010,260},{1010,412}, closed=true, char="soft", seed=63, lobe=18}
trees2 = elm2:mask() + oak2:mask() + rgt2:mask()
work(trees2, {hand="body", tool="filbert 10", pile=treeDark, coverage=2.4, fill=true, edge="loose", angle=function(x,y) return -1.3+0.7*math.sin(x/30+y/45) end})

--@ chunk 33
blend(farLine2:mask():grow(6):soften(6), {angle=0.05})
local litSide = trees2:shrink(8) * mask(function(x,y) if x < 720 and y > 250 then return 1 elseif x < 780 and y < 250 then return 0.7 else return 0 end end):blur(30)
stipple(litSide, {pile=treeMid, width=7, coverage=0.6, cluster={0.7, 30}, feather=0.5, pressure={0.3,0.6}})
local core = (elm2:mask():shrink(30) + rgt2:mask():shrink(30)):blur(15)
stipple(core, {pile=pile{{"raw umber",2},{"bone black",1.5},{"Antwerp blue",0.3}, medium=0.15}, width=8, coverage=0.5, cluster={0.6,40}, feather=0.5})

--@ chunk 34
holeHi = pile{{"lead white",6},{"Indian red",0.25},{"cobalt blue",0.3},{"yellow ochre",0.4}, medium=0.1}
holeLo = pile{{"lead white",6},{"yellow ochre",0.7},{"lemon chrome",0.4}, medium=0.1}
local pts = {{770,120,holeHi},{812,160,holeHi},{740,190,holeHi},{842,205,holeHi},{700,232,holeLo},{915,265,holeLo},{955,250,holeLo},{868,290,holeLo},{688,310,holeLo},{790,255,holeHi},{640,300,holeLo},{980,300,holeLo},{860,240,holeHi}}
for i, p in ipairs(pts) do
  local hb = brush("round", rand(3,5.5))
  hb:load(p[3], 0.5)
  for k=1,math.random(2,4) do
    hb:touch(p[1]+rand(-9,9), p[2]+rand(-9,9), {pressure=rand(0.3,0.6), drag={rand(-2,2), rand(-2,2)}})
  end
end
blend(trees2:rim(16,10) * above(function(x) return 395 end), {angle=-1.2})

--@ chunk 35
print(wait(30*60)); print(drying(750,200), drying(300,250), drying(200,390))

--@ chunk 36
local pts = {{770,118,holeHi},{812,158,holeHi},{742,188,holeHi},{846,205,holeHi},{702,236,holeLo},{915,262,holeLo},{955,248,holeLo},{868,292,holeLo},{690,312,holeLo},{792,255,holeHi},{645,298,holeLo},{982,302,holeLo},{862,242,holeHi},{660,350,holeLo},{885,330,holeLo}}
for i, p in ipairs(pts) do
  local hb = brush("round", rand(2.5,5))
  hb:load(p[3], 0.45)
  for k=1,math.random(2,5) do
    hb:touch(p[1]+rand(-10,10), p[2]+rand(-10,10), {pressure=rand(0.25,0.55), drag={rand(-2,2), rand(-2,2)}})
  end
end
-- edge flicker: small sky touches breaking the contour
stipple(trees2:rim(10,6) * above(function(x) return 360 end), {pile=holeHi, width=4, coverage=0.25, cluster={0.85,15}, pressure={0.25,0.5}, drag=2})

--@ chunk 37
print(wait(3*24*60)); print(drying(750,200), drying(300,250), drying(200,390), drying(400,440))

--@ chunk 38
local pts = {{770,118,holeHi},{812,158,holeHi},{742,188,holeHi},{846,205,holeHi},{702,236,holeLo},{915,262,holeLo},{955,248,holeLo},{868,292,holeLo},{690,312,holeLo},{792,255,holeHi},{645,298,holeLo},{982,302,holeLo},{862,242,holeHi},{660,350,holeLo},{885,330,holeLo}}
for i, p in ipairs(pts) do
  local hb = brush{kind="filbert", width=rand(3,6)}
  hb:load(p[3], 0.8)
  for k=1,math.random(2,5) do
    hb:touch(p[1]+rand(-10,10), p[2]+rand(-10,10), {pressure=rand(0.4,0.8), drag={rand(-3,3), rand(-3,3)}})
  end
end

--@ chunk 39
local function gap(cx, cy, n, pl, sz)
  local hb = brush("filbert", sz)
  hb:load(pl, 0.7)
  for k=1,n do
    local x, y = cx+rand(-12,12), cy+rand(-10,10)
    local a = rand(-0.6,0.6) - 1.3
    local l = rand(4,9)
    hb:stroke({{x,y},{x+l*math.cos(a), y+l*math.sin(a)}}, {pressure={0.6,0.3}, ramps={0.2,0.5}})
    if k % 4 == 0 then hb:load(pl, 0.6) end
  end
end
gap(770,118,6,holeHi,6)
gap(815,160,5,holeHi,5)
gap(700,238,9,holeLo,6)
gap(688,300,7,holeLo,6)
gap(645,300,5,holeLo,5)
gap(860,240,7,holeHi,6)
gap(918,262,6,holeLo,5)
gap(870,300,5,holeLo,5)
gap(985,300,5,holeLo,5)
gap(745,185,4,holeHi,5)
gap(660,352,5,holeLo,5)

--@ chunk 40
local m = ellipse(770,118,22,18)+ellipse(815,160,18,15)+ellipse(700,238,22,18)+ellipse(688,300,20,18)+ellipse(645,300,16,14)+ellipse(860,240,20,16)+ellipse(918,262,18,15)+ellipse(870,300,16,14)+ellipse(985,300,16,14)+ellipse(745,185,15,13)+ellipse(660,352,16,14)
blend(m:soften(6), {angle=-1.3})
local tb = brush("filbert", 5)
for i, p in ipairs({{770,118},{815,160},{700,238},{688,300},{860,240},{918,262},{745,185}}) do
  tb:load(treeDark, 0.5)
  for k=1,3 do
    local x,y = p[1]+rand(-10,10), p[2]+rand(-10,10)
    tb:stroke({{x,y},{x+rand(-3,3), y-rand(4,8)}}, {pressure={0.5,0.2}})
  end
end

--@ chunk 41
r2 = rag{width=30}
r2:dip(0.6)
for i, p in ipairs({{770,118},{815,160},{700,238},{688,300},{645,300},{860,240},{918,262},{870,300},{985,300},{745,185},{660,352}}) do
  r2:blot(p[1], p[2], {pressure=0.7})
  r2:wipe({{p[1]-12,p[2]},{p[1]+12,p[2]+2}}, {pressure=0.7})
  r2:refold()
  if i % 3 == 0 then r2:dip(0.5) end
end

--@ chunk 42
print(wait(3*24*60)); print(drying(698,240), drying(770,118))

--@ chunk 43
print(wait(2*24*60)); print(drying(698,240), drying(770,118))

--@ chunk 44
treeDark2 = pile{{"raw umber",3},{"bone black",1.2},{"Antwerp blue",0.25},{"yellow ochre",0.5}, medium=0.15}
treeOlive = pile{{"raw umber",2},{"yellow ochre",1.2},{"bone black",0.5},{"raw sienna",0.5},{"lead white",0.3}, medium=0.15}
local inner = trees2:shrink(14):blur(8)
work(inner, {hand="body", tool="filbert 10", pile=treeDark2, coverage=1.8, fill=true, edge="soft", angle=function(x,y) return -1.3+0.8*math.sin(x/25+y/35) end, pressure={0.5,0.9}})
-- olive masses on the glow side as clumps of foliage
for i=1,9 do
  local cx, cy = rand(620,760), rand(150,390)
  if trees2:at(cx,cy) > 0.5 then
    local cl = outline{{cx-25,cy+10},{cx-18,cy-12},{cx,cy-20},{cx+20,cy-14},{cx+26,cy+6},{cx+5,cy+16}, closed=true, char="soft", seed=100+i, lobe=8}
    work(cl:mask() * inner, {hand="body", tool="filbert 7", pile=treeOlive, coverage=1, edge="lost", angle=-1.0, pressure={0.3,0.6}, load=0.5})
  end
end

--@ chunk 45
blend(trees2:shrink(4):soften(10), {angle=-1.2})
blend(trees2:shrink(4):soften(10), {angle=-0.3})

--@ chunk 46
local d = ellipse(871,300,20,18):soften(5)
work(d, {hand="body", tool="filbert 6", pile=treeDark2, coverage=2.5, fill=true, angle=-1.2, edge="soft"})
blend(ellipse(871,300,28,26):soften(8), {angle=-1.2})

--@ chunk 47
fgGlaze = pile{{"raw umber",2},{"bone black",1},{"raw sienna",1}, medium=0.6, thinner=0.3}
local fgM = below(function(x) return 470 + 25*math.sin(x/160+1) end):soften(40)
work(fgM, {hand="glaze", pile=fgGlaze, coverage=1.5, angle=0.03, load_at=function(x,y) return 0.4+0.6*smoothstep(470,660,y) end})
-- wipe the path out of the wet glaze
local rp = rag{width=40}
rp:wipe({{300,672},{350,610},{405,555},{445,515},{468,490}}, {pressure={0.75,0.4}})
rp:refold()
rp:wipe({{330,672},{380,605},{425,550},{458,505}}, {pressure={0.6,0.35}})

--@ chunk 48
fgDeep = pile{{"raw umber",3},{"bone black",0.8},{"raw sienna",1},{"yellow ochre",0.5}, medium=0.15}
fgMid = pile{{"raw umber",2},{"yellow ochre",1.2},{"bone black",0.4},{"raw sienna",0.6},{"lead white",0.3}, medium=0.15}
local hz = function(x,y) return 0.06*math.sin(x/90+y/70) - 0.02 end
local lower = below(function(x) return 545 + 25*math.sin(x/150+0.5) end)
local middle = below(function(x) return 468 + 8*math.sin(x/100) end) * above(function(x) return 560 + 25*math.sin(x/150+0.5) end) - pondMask
work(middle, {hand="body", tool="filbert 12", pile=fgMid, coverage=1.6, fill=true, angle=hz, edge="soft"})
work(lower, {hand="broad", pile=fgDeep, coverage=2.2, fill=true, angle=hz, edge="soft"})

--@ chunk 49
print(wait(5*24*60)); print(drying(300,500), drying(300,620), drying(200,390), drying(400,440), drying(750,250))

--@ chunk 50
print(wait(2*24*60)); print(drying(300,500), drying(600,500))

--@ chunk 51
farTree3 = pile{{"lead white",2},{"cobalt blue",0.6},{"Indian red",0.4},{"raw umber",1.4},{"bone black",0.35}, medium=0.12}
farTree4 = pile{{"lead white",1.2},{"cobalt blue",0.5},{"Indian red",0.35},{"raw umber",1.6},{"bone black",0.5}, medium=0.12}
farLine3 = outline{{-10,412},{-10,384},{20,378},{50,372},{80,364},{100,356},{118,348},{140,344},{160,348},{180,346},{200,354},{230,364},{262,372},{300,378},{360,383},{420,387},{480,390},{540,392},{590,394},{590,412}, closed=true, char="soft", seed=71, lobe=8}
work(farLine3:mask(), {hand="body", tool="filbert 6", pile=farTree3, coverage=2.2, fill=true, edge="soft", angle=function(x,y) return -1.45+0.3*math.sin(x/17) end})
local clump = outline{{92,410},{95,370},{108,354},{128,344},{150,348},{166,356},{176,372},{180,410}, closed=true, char="soft", seed=72, lobe=6}
work(clump:mask(), {hand="body", tool="filbert 5", pile=farTree4, coverage=1.8, fill=true, edge="soft", angle=-1.4})
blend(farLine3:mask():grow(5):soften(5) * above(function(x) return 395 end), {angle=-1.3})

--@ chunk 52
local fl = farLine3:mask():grow(4):soften(6)
blend(fl, {angle=0.05})
blend(fl, {angle=-0.6})
-- touches of tree shapes: a few taller crowns breaking the line
farTreeG = pile{{"lead white",1.6},{"cobalt blue",0.4},{"yellow ochre",0.4},{"raw umber",1.6},{"bone black",0.35},{"Indian red",0.15}, medium=0.12}
local fb = brush("filbert", 6)
for i, p in ipairs({{40,370},{230,362},{300,376},{420,384},{510,388},{560,390}}) do
  fb:load(farTreeG, 0.6)
  for k=1,5 do
    local x = p[1] + rand(-14,14)
    local y = p[2] + rand(-4,10)
    fb:touch(x, y, {pressure=rand(0.3,0.6), drag={rand(-2,2),rand(-3,1)}})
  end
end

--@ chunk 53
fgGlaze2 = pile{{"raw umber",2},{"bone black",1.2},{"raw sienna",0.6},{"Antwerp blue",0.15}, medium=0.55, thinner=0.25}
local fgM = (below(function(x) return 455 end) - pondMask:grow(3)):soften(10)
work(fgM, {hand="glaze", pile=fgGlaze2, coverage=1.6, angle=function(x,y) return 0.03*math.sin(x/100) end,
  load_at=function(x,y) return 0.25 + 0.55*smoothstep(470,660,y) + 0.3*smoothstep(550,900,x)*smoothstep(500,440,y) end})
local rp = rag{width=45}
rp:wipe({{300,672},{350,612},{405,556},{445,516},{470,490}}, {pressure={0.7,0.35}})
rp:refold()
rp:wipe({{250,470},{350,474},{450,476}}, {pressure=0.35})

--@ chunk 54
fgGlaze3 = pile{{"raw umber",2},{"bone black",1.5},{"Antwerp blue",0.2}, medium=0.5, thinner=0.1}
local fgM = (below(function(x) return 470 end) - pondMask:grow(3)):soften(20)
work(fgM, {hand="glaze", pile=fgGlaze3, coverage=2.5, fill=true, angle=function(x,y) return 0.04*math.sin(x/100) end, pressure={0.4,0.7},
  load_at=function(x,y) return 0.3 + 0.6*smoothstep(480,660,y) + 0.3*smoothstep(550,900,x) end})
blend(fgM, {angle=0.02})

--@ chunk 55
local rp = rag{width=36}
rp:dip(0.3)
rp:wipe({{300,672},{345,615},{398,560},{440,520},{466,494},{478,478}}, {pressure={0.8,0.4}})
rp:refold()
rp:wipe({{335,672},{378,612},{420,556},{455,512},{474,486}}, {pressure={0.6,0.3}})
rp:refold()
-- light on the meadow strip near the water, wiped
rp:wipe({{0,468},{120,466},{240,470}}, {pressure=0.4})
rp:wipe({{540,462},{620,458},{700,460}}, {pressure=0.3})

--@ chunk 56
print(wait(4*24*60)); print(drying(300,600), drying(200,390), drying(700,600))

--@ chunk 57
skyMatch = pile{{"lead white",7},{"lemon chrome",1.1},{"yellow ochre",0.25},{"cadmium yellow",0.1}, medium=0.12}
local prof = {{-10,388},{15,380},{30,368},{45,372},{62,366},{80,360},{96,352},{112,343},{128,340},{142,346},{158,341},{172,348},{190,356},{205,352},{220,362},{248,370},{275,368},{290,376},{320,380},{350,377},{370,383},{410,386},{440,383},{470,389},{520,391},{560,390},{600,394},{620,400}}
local cut = above(prof) * below(function(x) return 325 end) * mask(function(x,y) return x < 600 and 1 or 0 end)
work(cut, {hand="body", tool="filbert 8", pile=skyMatch, coverage=2, fill=true, angle=0, edge="soft"})
blend(cut:grow(4):soften(4), {angle=0})

--@ chunk 58
local prof = {{-10,388},{15,380},{30,368},{45,372},{62,366},{80,360},{96,352},{112,343},{128,340},{142,346},{158,341},{172,348},{190,356},{205,352},{220,362},{248,370},{275,368},{290,376},{320,380},{350,377},{370,383},{410,386},{440,383},{470,389},{520,391},{560,390},{600,394},{620,400}}
local function profY(x)
  for i=1,#prof-1 do
    if x >= prof[i][1] and x <= prof[i+1][1] then
      local t = (x-prof[i][1])/(prof[i+1][1]-prof[i][1]); return lerp(prof[i][2], prof[i+1][2], t)
    end
  end
  return 400
end
local fb = brush{kind="filbert", width=5, stiffness=0.4}
local x = 0
local n = 0
while x < 590 do
  local y = profY(x)
  fb:load(farTree3, 0.5)
  for k=1,3 do
    local xx = x + rand(-4,4)
    local yy = y + rand(-6,3)
    fb:stroke({{xx, yy+8},{xx+rand(-1.5,1.5), yy - rand(0,5)}}, {pressure={0.5,0.15}, ramps={0.1,0.5}})
  end
  x = x + rand(5,11)
  n = n + 1
end
print(n)

--@ chunk 59
local band = (below(function(x) return 325 end) * above(function(x) return 400 end) * mask(function(x,y) return x < 610 and 1 or 0 end)):soften(6)
blend(band, {angle=0.1})
blend(band, {angle=-1.4})
blend(band, {angle=0.6})

--@ chunk 60
farProf = outline{{-10,420},{-10,382},{40,374},{85,362},{115,350},{140,347},{165,352},{200,360},{240,370},{290,377},{350,381},{420,385},{500,389},{580,392},{615,396},{615,420}, closed=true, char="soft", seed=81, lobe=7, amount=1.2}
local fm = farProf:mask()
local skyPart = below(function(x) return 300 end) * above(function(x) return 402 end) * mask(function(x,y) return x < 620 and 1 or 0 end) - fm
work(skyPart, {hand="body", tool="filbert 10", pile=skyMatch, coverage=2.2, fill=true, angle=function(x,y) return 0.03*math.sin(x/60) end, edge="soft"})
work(fm * above(function(x) return 403 end), {hand="body", tool="filbert 7", pile=farTree3, coverage=2.2, fill=true, angle=function(x,y) return -1.4+0.4*math.sin(x/15) end, edge="soft"})

--@ chunk 61
blend(farProf:mask():grow(3):soften(5) * above(function(x) return 403 end), {angle=0.15})
-- the farmhouse grove, a little darker and warmer, wet into wet
local clump = outline{{95,402},{97,368},{110,352},{130,343},{150,347},{165,355},{175,370},{178,402}, closed=true, char="soft", seed=82, lobe=6}
work(clump:mask(), {hand="body", tool="filbert 5", pile=farTree4, coverage=1.4, edge="soft", angle=-1.4, pressure={0.3,0.6}})

--@ chunk 62
local cm = ellipse(136,375,52,40):soften(10)
blend(cm, {angle=-0.5})
blend(cm, {angle=0.4})

--@ chunk 63
local seam = (below(function(x) return 280 end) * above(function(x) return 320 end) * mask(function(x,y) return x < 600 and 1 or 0 end)):soften(10)
blend(seam, {angle=0.05})
blend(seam, {angle=1.5})
print(wait(5*24*60))
print(drying(300,350), drying(200,390), drying(610,330))

--@ chunk 64
print(wait(2*24*60))
print(drying(300,350), drying(200,390), drying(610,330))

--@ chunk 65
print(wait(2*24*60))
print(drying(610,330))

--@ chunk 66
oakL = outline{{640,414},{612,410},{592,398},{578,378},{582,356},{590,336},{588,312},{600,292},{618,276},{640,262},{662,258},{680,270},{690,300},{690,360},{685,414}, closed=true, char="soft", seed=91, lobe=14}
work(oakL:mask(), {hand="body", tool="filbert 9", pile=treeDark2, coverage=2.2, fill=true, edge="loose", angle=function(x,y) return -1.2+0.7*math.sin(x/25+y/30) end})
stipple(oakL:mask():rim(12,8), {pile=treeDark2, width=6, coverage=0.4, cluster={0.7,18}, pressure={0.3,0.6}, drag=3})

--@ chunk 67
blend(oakL:mask():grow(6):soften(10), {angle=-1.1})
-- olive light on the glow-facing side, scumbled thin
treeLitSc = pile{{"raw umber",1.5},{"yellow ochre",1.2},{"lead white",0.6},{"raw sienna",0.3},{"bone black",0.2}, medium=0.2}
local lit = oakL:mask():shrink(6) * mask(function(x,y) return x < 625 and 1 or 0 end):blur(15)
work(lit, {hand="scumble", pile=treeLitSc, coverage=0.6, pressure={0.2,0.4}, load=0.35, angle=-1.2})

--@ chunk 68
local m = oakL:mask():shrink(4):soften(8)
blend(m, {angle=-1.2})
blend(m, {angle=0.3})
blend(m, {angle=-0.7})

--@ chunk 69
local cb = brush{kind="filbert", width=8, stiffness=0.5}
local clumps = {{612,300},{640,285},{600,340},{630,330},{660,310},{596,375},{625,370},{655,355},{615,400},{650,395},{675,380},{670,285}}
for i,p in ipairs(clumps) do
  cb:load(treeDark2, 0.6)
  -- an arc under each clump: shadow
  local x,y = p[1], p[2]
  cb:stroke({{x-14,y+4},{x-4,y+10},{x+10,y+9},{x+16,y+2}}, {pressure={0.6,0.3}, ramps={0.2,0.4}})
end
local lb = brush{kind="filbert", width=6}
for i,p in ipairs(clumps) do
  if p[1] < 640 then
    lb:load(treeLitSc, 0.4)
    local x,y = p[1]-3, p[2]-4
    lb:stroke({{x-10,y+2},{x-4,y-4},{x+6,y-5}}, {pressure={0.35,0.15}, ramps={0.2,0.5}})
  end
end

--@ chunk 70
local m = oakL:mask():shrink(2):soften(8)
blend(m, {angle=-1.0})
blend(m, {angle=0.5})
blend(m, {angle=-0.4})
blend(m, {angle=1.2})

--@ chunk 71
print(wait(36*60)); print(drying(630,330), drying(600,300), drying(585,380))

--@ chunk 72
print(wait(24*60)); print(drying(630,330), drying(600,300), drying(585,380))

--@ chunk 73
skyEdgeHi = pile{{"lead white",6},{"Indian red",0.25},{"cobalt blue",0.35},{"yellow ochre",0.4},{"bone black",0.08}, medium=0.08}
skyEdgeLo = pile{{"lead white",7},{"lemon chrome",0.9},{"yellow ochre",0.3}, medium=0.08}
local allT = trees2 + oakL:mask()
local rimT = allT:rim(10, 4)
local hiR = rimT * above(function(x) return 200 end)
local loR = rimT * below(function(x) return 190 end) * above(function(x) return 395 end)
lose(allT * above(function(x) return 200 end), {pile=skyEdgeHi, tool="filbert 4", reach={6,10}, load=0.25, every=2.5, pressure={0.4,0.05}})
lose(allT * below(function(x) return 190 end) * above(function(x) return 395 end), {pile=skyEdgeLo, tool="filbert 4", reach={6,10}, load=0.25, every=2.5, pressure={0.4,0.05}})

--@ chunk 74
local allT = trees2 + oakL:mask()
local m = allT:rim(16, 4):grow(6) + (allT:grow(12) * (ribbon({{550,200},{1010,200}}, 26) + ribbon({{550,396},{1010,396}}, 26)))
local rr = rag{width=28}
rr:dip(0.8)
rr:wipe(m, {pressure=0.75, passes=2, refold=0.25, angle=0})
rr:refold(); rr:dip(0.8)
rr:wipe(m, {pressure=0.8, passes=1, refold=0.25, angle=1.2})

--@ chunk 75
local allT = trees2 + oakL:mask()
local fix = allT:rim(18, 4):grow(4) + (allT:grow(10) * (ribbon({{550,200},{1010,200}}, 30) + ribbon({{550,396},{1010,396}}, 30)))
work(fix, {hand="body", tool="filbert 8", pile=treeDark2, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -1.3+0.8*math.sin(x/25+y/35) end, pressure={0.6,0.9}, load=0.8})

--@ chunk 76
print(wait(4*24*60)); print(drying(630,330), drying(750,200), drying(700,180))

--@ chunk 77
print(wait(4*24*60)); print(drying(630,330), drying(750,200), drying(700,180), drying(800,300))

--@ chunk 78
print(wait(3*24*60)); print(drying(700,180), drying(660,170))

--@ chunk 79
oldT = trees2 + oakL:mask()
elmN = outline{{712,306},{680,272},{668,222},{682,166},{712,124},{748,94},{790,80},{830,90},{860,120},{878,166},{876,220},{862,262},{836,292},{800,306},{760,312}, closed=true, char="soft", seed=201, lobe=16, amount=1.3}
rgtN = outline{{852,338},{860,292},{882,256},{912,232},{946,224},{976,236},{1012,256},{1012,352},{978,348},{944,342},{904,348},{872,346}, closed=true, char="soft", seed=202, lobe=14, amount=1.3}
oakN = outline{{640,414},{612,410},{592,398},{578,378},{582,356},{590,336},{588,312},{600,292},{618,276},{640,262},{662,258},{680,268},{694,292},{700,330},{696,370},{690,414}, closed=true, char="soft", seed=203, lobe=13, amount=1.2}
underN = outline{{570,420},{575,398},{600,392},{640,394},{690,390},{720,384},{745,388},{770,392},{800,386},{830,390},{860,384},{900,388},{940,384},{980,388},{1012,386},{1012,420}, closed=true, char="soft", seed=204, lobe=8, amount=1.5}
newT = elmN:mask() + rgtN:mask() + oakN:mask() + underN:mask()
R = oldT:grow(6) * -(newT:shrink(5)) * above(function(x) return 405 end)
sA = pile{{"lead white",6},{"Indian red",0.25},{"cobalt blue",0.35},{"yellow ochre",0.4},{"bone black",0.08}, medium=0.1}
sB = pile{{"lead white",7},{"yellow ochre",0.55},{"cobalt blue",0.15},{"Indian red",0.12}, medium=0.1}
sC = pile{{"lead white",7},{"yellow ochre",0.6},{"lemon chrome",0.35},{"Indian red",0.05}, medium=0.1}
local hz = function(x,y) return 0.05*math.sin(x/60+y/40) end
work(R * above(function(x) return 165 end), {hand="body", tool="filbert 9", pile=sA, coverage=2.4, fill=true, angle=hz, clip=true})
work(R * below(function(x) return 155 end) * above(function(x) return 290 end), {hand="body", tool="filbert 9", pile=sB, coverage=2.4, fill=true, angle=hz, clip=true})
work(R * below(function(x) return 280 end), {hand="body", tool="filbert 9", pile=sC, coverage=2.4, fill=true, angle=hz, clip=true})
blend(R:soften(3), {angle=1.5})

--@ chunk 80
sB2 = pile{{"lead white",7},{"yellow ochre",0.45},{"cobalt blue",0.25},{"Indian red",0.16},{"bone black",0.04}, medium=0.1}
sC2 = pile{{"lead white",7},{"yellow ochre",0.55},{"lemon chrome",0.2},{"cobalt blue",0.08},{"Indian red",0.07}, medium=0.1}
R2 = oldT:grow(34) * -(newT:shrink(5)) * above(function(x) return 405 end)
local hz = function(x,y) return 0.05*math.sin(x/60+y/40) end
work(R2 * above(function(x) return 150 end), {hand="body", tool="filbert 9", pile=sA, coverage=2.2, fill=true, angle=hz, clip=true})
work(R2 * below(function(x) return 140 end) * above(function(x) return 300 end), {hand="body", tool="filbert 9", pile=sB2, coverage=2.2, fill=true, angle=hz, clip=true})
work(R2 * below(function(x) return 290 end), {hand="body", tool="filbert 9", pile=sC2, coverage=2.2, fill=true, angle=hz, clip=true})

--@ chunk 81
print(drying(700,350), drying(300,200))
skyT = pile{{"lead white",4.5},{"cobalt blue",1},{"bone black",0.4},{"Indian red",0.35},{"raw umber",0.15}, medium=0.12}
skyU = pile{{"lead white",5.5},{"cobalt blue",0.6},{"Indian red",0.35},{"yellow ochre",0.3},{"bone black",0.15}, medium=0.12}
skyR = pile{{"lead white",6},{"Indian red",0.3},{"cobalt blue",0.3},{"yellow ochre",0.5},{"bone black",0.05}, medium=0.12}
skyG = pile{{"lead white",6.5},{"yellow ochre",0.8},{"lemon chrome",0.4},{"Indian red",0.08}, medium=0.12}
skyL = pile{{"lead white",7},{"lemon chrome",1.2},{"yellow ochre",0.2},{"cadmium yellow",0.1}, medium=0.1}

--@ chunk 82
local sky = above(function(x) return 404 end) - newT:shrink(6)
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return sky * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(-20, 95), {hand="broad", pile=skyT, coverage=2.2, fill=true, angle=hz})
work(band(80, 175), {hand="broad", pile=skyU, coverage=2.2, fill=true, angle=hz})
work(band(160, 255), {hand="broad", pile=skyR, coverage=2.2, fill=true, angle=hz})
work(band(240, 330), {hand="broad", pile=skyG, coverage=2.2, fill=true, angle=hz})
work(band(315, 420) * mask(function(x,y) return x < 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyL, coverage=2.2, fill=true, angle=hz})
work(band(315, 420) * mask(function(x,y) return x >= 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyG, coverage=2.2, fill=true, angle=hz})

--@ chunk 83
local sky = above(function(x) return 404 end)
blend(sky, {angle=0.02})
blend(sky, {angle=-0.04})
blend(sky * below(function(x) return 200 end), {angle=1.5})

--@ chunk 84
local sky = above(function(x) return 404 end)
blend(sky, {angle=0.0})
blend((below(function(x) return 150 end) * above(function(x) return 260 end)):soften(30), {angle=0.03})
blend(sky, {angle=0.05})

--@ chunk 85
print(wait(5*24*60)); print(drying(300,200), drying(800,250), drying(300,380))

--@ chunk 86
farV = pile{{"lead white",3},{"cobalt blue",0.6},{"Indian red",0.45},{"raw umber",0.9},{"yellow ochre",0.2}, medium=0.12}
farV2 = pile{{"lead white",2},{"cobalt blue",0.55},{"Indian red",0.4},{"raw umber",1.3},{"bone black",0.25}, medium=0.12}
fp = outline{{-10,420},{-10,386},{25,380},{55,376},{78,368},{98,358},{116,350},{136,346},{156,350},{172,356},{192,366},{225,372},{262,376},{300,380},{330,377},{352,381},{400,385},{455,388},{505,390},{545,389},{580,393},{620,398},{620,420}, closed=true, char="soft", seed=301, lobe=7, amount=1.0}
work(fp:mask() * above(function(x) return 408 end), {hand="body", tool="filbert 6", pile=farV, coverage=2.2, fill=true, edge="soft", angle=function(x,y) return -1.5+0.35*math.sin(x/13) end, pressure={0.4,0.7}})
local grove = outline{{90,408},{94,372},{106,358},{122,348},{140,345},{156,350},{168,360},{176,376},{180,408}, closed=true, char="soft", seed=302, lobe=6}
work(grove:mask(), {hand="body", tool="filbert 5", pile=farV2, coverage=1.6, fill=true, edge="soft", angle=-1.45, pressure={0.4,0.7}})
blend(fp:mask():grow(4):soften(5) * above(function(x) return 404 end), {angle=0.1})

--@ chunk 87
trunkP = pile{{"raw umber",3},{"bone black",1.2},{"Indian red",0.2}, medium=0.15}
local rb = brush{kind="round", width=9, point=0.6}
local function limb(pts, w, pr)
  rb = brush{kind="round", width=w, point=0.7}
  rb:load(trunkP, 0.8)
  rb:stroke(pts, {pressure=pr or {0.9,0.15}, ramps={0.05,0.6}, shake=0.8})
end
-- main elm: trunk splitting into a vase of limbs
limb({{762,418},{760,380},{757,345},{752,318}}, 11, {1,0.8})
limb({{752,320},{738,280},{722,240},{708,200},{700,170}}, 7)
limb({{754,318},{762,270},{772,225},{780,180},{790,140}}, 7)
limb({{756,322},{790,290},{820,255},{845,225}}, 6)
limb({{750,325},{720,300},{690,282},{668,272}}, 5)
limb({{770,240},{800,210},{828,190}}, 3.5)
limb({{725,250},{700,232},{680,224}}, 3.5)
limb({{780,180},{772,140},{765,110}}, 3)
-- tree on the left
limb({{648,418},{646,385},{642,355}}, 7, {1,0.6})
limb({{643,360},{628,330},{615,305}}, 4)
limb({{644,360},{660,330},{672,305}}, 4)
-- tree at the right edge
limb({{935,418},{932,380},{928,345},{924,320}}, 9, {1,0.7})
limb({{926,330},{905,290},{890,260}}, 5)
limb({{928,325},{950,285},{975,255},{1000,240}}, 5)

--@ chunk 88
folD = pile{{"raw umber",3},{"bone black",1},{"yellow ochre",0.7},{"Antwerp blue",0.2},{"lead white",0.3}, medium=0.15}
folM = pile{{"raw umber",2},{"bone black",0.6},{"yellow ochre",1},{"lead white",0.9},{"cobalt blue",0.2}, medium=0.15}
-- clumps: x, y, rx, ry
clumpsE = {{700,170,40,30},{745,130,45,32},{795,110,42,30},{840,150,40,32},{860,210,32,30},{720,225,38,30},{775,190,45,35},{820,240,38,30},{680,270,30,24},{760,265,40,26},{805,290,32,20}}
clumpsL = {{620,310,30,26},{660,300,28,24},{640,345,34,26},{612,370,26,22},{668,375,28,24}}
clumpsR = {{895,265,32,26},{945,250,40,30},{990,250,30,30},{915,310,38,28},{975,305,35,30},{870,330,28,22}}
foliage = nil
local function addc(list)
  for i,c in ipairs(list) do
    local o = outline{{c[1]-c[3],c[2]+c[4]*0.3},{c[1]-c[3]*0.7,c[2]-c[4]*0.6},{c[1],c[2]-c[4]},{c[1]+c[3]*0.7,c[2]-c[4]*0.6},{c[1]+c[3],c[2]+c[4]*0.3},{c[1]+c[3]*0.4,c[2]+c[4]},{c[1]-c[3]*0.4,c[2]+c[4]}, closed=true, char="soft", seed=400+i+#list*7, lobe=7, amount=1.4}
    local m = o:mask()
    foliage = foliage and (foliage + m) or m
  end
end
addc(clumpsE); addc(clumpsL); addc(clumpsR)
stipple(foliage, {pile=folD, width=9, coverage=1.6, cluster={0.5,22}, pressure={0.4,0.8}, drag={4, -1.2}, twist=0.5, feather=0.6})

--@ chunk 89
folDD = pile{{"raw umber",3},{"bone black",1.4},{"Antwerp blue",0.25},{"yellow ochre",0.3}, medium=0.15}
local under = nil
local function adds(list)
  for i,c in ipairs(list) do
    local m = ellipse(c[1]+c[3]*0.15, c[2]+c[4]*0.35, c[3]*0.8, c[4]*0.6):soften(8)
    under = under and (under + m) or m
  end
end
adds(clumpsE); adds(clumpsL); adds(clumpsR)
local m = under * foliage
stipple(m, {pile=folDD, width=8, coverage=1.4, cluster={0.5,18}, pressure={0.4,0.8}, drag={4,-1.2}, twist=0.5, feather=0.6})

--@ chunk 90
underM = nil
for _, list in ipairs({clumpsE, clumpsL, clumpsR}) do
  for i,c in ipairs(list) do
    local m = ellipse(c[1]+c[3]*0.15, c[2]+c[4]*0.35, c[3]*0.8, c[4]*0.6):soften(8)
    underM = underM and (underM + m) or m
  end
end
blend(foliage:grow(4):soften(6), {angle=-1.0})
stipple(foliage:shrink(6) * underM, {pile=folDD, width=11, coverage=1.0, cluster={0.6,25}, pressure={0.5,0.9}, drag={3,-1.0}, feather=0.5})
blend(foliage:shrink(2):soften(6), {angle=-0.4})

--@ chunk 91
print(wait(4*24*60)); print(drying(750,200), drying(640,340), drying(940,300))

--@ chunk 92
treeGlaze = pile{{"raw umber",2},{"bone black",1.5},{"Antwerp blue",0.3}, medium=0.55, thinner=0.2}
local tg = foliage:grow(3):soften(4)
work(tg, {hand="glaze", tool={kind="filbert", width=14, stiffness=0.3}, pile=treeGlaze, coverage=2, fill=true, clip=true, angle=-1.0, length={30,70}, pressure={0.5,0.8}})
-- wipe back the glow side and the tops a little
local rr = rag{width=30}
for _, p in ipairs({{625,300},{612,340},{605,372},{690,165},{735,128},{790,104},{700,220},{880,262},{870,325}}) do
  rr:wipe({{p[1]-10,p[2]+8},{p[1]+8,p[2]-6}}, {pressure=0.35})
end

--@ chunk 93
bushD = pile{{"raw umber",3},{"bone black",1.2},{"yellow ochre",0.5},{"Antwerp blue",0.2},{"lead white",0.2}, medium=0.15}
bushM = pile{{"raw umber",2},{"bone black",0.6},{"yellow ochre",0.9},{"lead white",0.7},{"cobalt blue",0.25},{"Indian red",0.1}, medium=0.15}
bank2 = outline{{555,452},{560,420},{575,400},{598,392},{625,388},{650,380},{680,384},{705,376},{740,382},{770,372},{800,380},{830,376},{860,384},{890,378},{925,384},{960,376},{1012,380},{1012,470},{900,466},{800,460},{700,456},{620,454}, closed=true, char="soft", seed=501, lobe=10, amount=1.5}
work(bank2:mask(), {hand="body", tool="filbert 8", pile=bushD, coverage=2.2, fill=true, edge="soft", angle=function(x,y) return -0.2+0.6*math.sin(x/20) end})
-- grayer distant bushes near the pond end, cooler
local farB = bank2:mask() * mask(function(x,y) return x < 640 and 1 or 0 end):blur(20)
work(farB, {hand="scumble", pile=bushM, coverage=0.8, pressure={0.3,0.5}, load=0.4, angle=-0.3})

--@ chunk 94
local m = bank2:mask():soften(4)
blend(m, {angle=0.0})
blend(m, {angle=0.5})
blend(m * mask(function(x,y) return x < 680 and 1 or 0 end):blur(20), {angle=-0.4})

--@ chunk 95
print(wait(5*24*60)); print(drying(750,400), drying(750,200), drying(600,420))

--@ chunk 96
meadNear = pile{{"yellow ochre",1.5},{"raw umber",1.5},{"lead white",1.2},{"bone black",0.3},{"cobalt blue",0.15}, medium=0.12}
meadMid = pile{{"yellow ochre",1.2},{"raw umber",2},{"lead white",0.6},{"bone black",0.5},{"raw sienna",0.3}, medium=0.12}
meadDark = pile{{"raw umber",3},{"bone black",1},{"raw sienna",0.6},{"yellow ochre",0.6}, medium=0.15}
-- new meadow shape: from pond shore/bank bottom to the bottom of canvas
local top = {{-10,428},{100,426},{205,436},{260,452},{330,466},{420,470},{520,466},{600,456},{660,446},{700,436},{760,430},{850,428},{940,430},{1012,428}}
local mead = below(top)
local hz = function(x,y) return 0.05*math.sin(x/120+y/60) - 0.02 end
-- strip near water / under the bank: lighter
local strip = mead * above(function(x) return 500 + 15*math.sin(x/90) end)
work(strip, {hand="body", tool="filbert 12", pile=meadNear, coverage=2.2, fill=true, angle=hz, edge="soft"})
local midb = mead * below(function(x) return 490 + 15*math.sin(x/90) end) * above(function(x) return 580 + 20*math.sin(x/140+1) end)
work(midb, {hand="body", tool="filbert 14", pile=meadMid, coverage=2.2, fill=true, angle=hz, edge="soft"})
local low = mead * below(function(x) return 570 + 20*math.sin(x/140+1) end)
work(low, {hand="broad", pile=meadDark, coverage=2.2, fill=true, angle=hz})

--@ chunk 97
local top = {{-10,428},{100,426},{205,436},{260,452},{330,466},{420,470},{520,466},{600,456},{660,446},{700,436},{760,430},{850,428},{940,430},{1012,428}}
local mead = below(top)
blend(mead * above(function(x) return 620 end), {angle=0.0})
blend(mead * below(function(x) return 470 end), {angle=-0.05})
blend(mead, {angle=1.4})

--@ chunk 98
local top = {{-10,428},{100,426},{205,436},{260,452},{330,466},{420,470},{520,466},{600,456},{660,446},{700,436},{760,430},{850,428},{940,430},{1012,428}}
local mead = below(top)
local hz = function(x,y) return 0.05*math.sin(x/120+y/60) - 0.02 end
local low = mead * below(function(x) return 540 + 25*math.sin(x/150+1) end):soften(30)
work(low, {hand="broad", pile=meadDark, coverage=2, fill=true, angle=hz, load_at=function(x,y) return 0.4+0.6*smoothstep(540,667,y) end})
blend(mead, {angle=0.02})
blend(mead, {angle=-0.03})

--@ chunk 99
print(wait(6*24*60)); print(drying(300,600), drying(700,500), drying(400,480))

--@ chunk 100
gl1 = pile{{"raw umber",2},{"bone black",1},{"raw sienna",0.8},{"Antwerp blue",0.15}, medium=0.6, thinner=0.2}
local top = {{-10,428},{100,426},{205,436},{260,452},{330,466},{420,470},{520,466},{600,456},{660,446},{700,436},{760,430},{850,428},{940,430},{1012,428}}
local mead = below(top):grow(2)
work(mead, {hand="glaze", pile=gl1, coverage=2.4, fill=true, angle=function(x,y) return 0.04*math.sin(x/90) end, pressure={0.45,0.75},
  load_at=function(x,y) return 0.35 + 0.65*smoothstep(470,660,y) + 0.2*smoothstep(600,950,x) end})
-- wipe back the light: strip along the water and the path
local rr = rag{width=50}
rr:wipe({{0,440},{120,442},{230,458},{330,478},{450,482},{560,470},{650,458}}, {pressure=0.5})
rr:refold()
rr:wipe({{60,456},{200,466},{330,490}}, {pressure=0.35})
rr:refold()
rr:wipe({{290,672},{340,615},{395,560},{438,520},{465,494},{478,480}}, {pressure={0.7,0.45}})
rr:refold()
rr:wipe({{335,672},{378,612},{420,556},{456,512}}, {pressure={0.55,0.3}})

--@ chunk 101
print(wait(2*24*60)); print(drying(300,600), drying(700,500))

--@ chunk 102
gl2 = pile{{"raw umber",2},{"bone black",1.3},{"raw sienna",0.5},{"Antwerp blue",0.2}, medium=0.45}
local top = {{-10,428},{100,426},{205,436},{260,452},{330,466},{420,470},{520,466},{600,456},{660,446},{700,436},{760,430},{850,428},{940,430},{1012,428}}
local mead = below(top):grow(2)
work(mead, {hand="glaze", pile=gl2, coverage=2.6, fill=true, angle=function(x,y) return 0.04*math.sin(x/90) end, pressure={0.5,0.85},
  load_at=function(x,y) return 0.25 + 0.75*smoothstep(470,640,y) + 0.25*smoothstep(600,950,x)*smoothstep(430,520,y) end})
blend(mead, {angle=0.02})

--@ chunk 103
local rr = rag{width=34}
rr:wipe({{300,672},{345,615},{398,560},{440,522},{466,496},{480,480}}, {pressure={0.75,0.45}})
rr:refold()
rr:wipe({{338,672},{380,612},{420,558},{455,515},{474,490}}, {pressure={0.6,0.35}})
rr:refold()
rr:wipe({{10,445},{120,447},{230,462},{330,480},{450,484},{560,470},{640,458}}, {pressure=0.35})

--@ chunk 104
crown = foliage
local inner = crown:shrink(9)
work(inner, {hand="body", tool="filbert 9", pile=folDD, coverage=2.2, fill=true, edge="loose", angle=function(x,y) return -1.2+0.8*math.sin(x/22+y/31) end, pressure={0.5,0.85}})
local ring = crown:grow(14) - crown:shrink(6)
local hz = function(x,y) return 0.05*math.sin(x/60+y/40) end
work(ring * above(function(x) return 170 end), {hand="scumble", pile=sA, coverage=1.0, pressure={0.3,0.6}, load=0.5, angle=hz, clip=crown:grow(18)})
work(ring * below(function(x) return 160 end) * above(function(x) return 290 end), {hand="scumble", pile=sB2, coverage=1.0, pressure={0.3,0.6}, load=0.5, angle=hz, clip=crown:grow(18)})
work(ring * below(function(x) return 280 end) * above(function(x) return 380 end), {hand="scumble", pile=sC2, coverage=1.0, pressure={0.3,0.6}, load=0.5, angle=hz, clip=crown:grow(18)})

--@ chunk 105
local zone = (crown:grow(30)) * above(function(x) return 382 end)
local rr = rag{width=40}
rr:dip(0.7)
rr:wipe(zone, {pressure=0.7, passes=2, refold=0.3, angle=0.3})
rr:refold(); rr:dip(0.7)
rr:wipe(zone, {pressure=0.7, passes=1, refold=0.3, angle=-0.5})

--@ chunk 106
print(wait(5*24*60)); print(drying(750,200), drying(700,250), drying(640,340))

--@ chunk 107
-- Re-draw the crown masses as a few big shapes
E1 = outline{{708,318},{682,300},{660,272},{654,236},{668,196},{690,160},{716,128},{748,100},{786,84},{826,92},{856,118},{874,156},{884,198},{878,240},{860,276},{832,300},{796,316},{760,322}, closed=true, char="soft", seed=611, lobe=18, amount=1.4}
L1 = outline{{600,404},{582,384},{578,356},{588,328},{604,302},{628,284},{656,276},{682,284},{700,308},{706,342},{698,376},{680,400}, closed=true, char="soft", seed=612, lobe=14, amount=1.3}
R1 = outline{{850,352},{858,310},{878,272},{908,244},{944,230},{982,236},{1012,252},{1012,360},{970,356},{930,358},{890,362}, closed=true, char="soft", seed=613, lobe=16, amount=1.3}
crowns = E1:mask() + L1:mask() + R1:mask()
work(crowns, {hand="body", tool="filbert 10", pile=folD, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -1.2+0.8*math.sin(x/22+y/31) end, pressure={0.5,0.85}, load=0.8})

--@ chunk 108
print(wait(6*24*60)); print(drying(750,200), drying(700,300), drying(620,360), drying(950,300))

--@ chunk 109
print(wait(6*24*60)); print(drying(750,200), drying(700,300), drying(620,360), drying(950,300))

--@ chunk 110
print(wait(5*24*60)); print(drying(750,200), drying(760,150), drying(800,250))

--@ chunk 111
E2 = outline{{735,330},{712,322},{700,300},{684,292},{672,270},{676,248},{668,228},{676,204},{692,190},{694,166},{712,148},{722,124},{744,112},{760,96},{784,92},{806,98},{826,96},{844,112},{852,134},{868,150},{870,176},{882,198},{876,222},{884,246},{872,268},{856,280},{846,300},{822,308},{800,322},{776,318},{756,330}, closed=true, char="soft", seed=711, lobe=10, amount=1.0}
L2 = outline{{612,398},{596,388},{590,366},{596,346},{606,328},{618,308},{636,296},{656,292},{672,300},{684,316},{690,338},{688,362},{682,386},{664,398}, closed=true, char="soft", seed=712, lobe=9, amount=1.0}
R2 = outline{{882,362},{872,340},{880,316},{896,296},{904,272},{924,256},{948,246},{970,250},{990,244},{1012,250},{1012,368},{980,366},{950,370},{918,364}, closed=true, char="soft", seed=713, lobe=10, amount=1.0}
keep = E2:mask() + L2:mask() + R2:mask()
-- show the planned silhouette with a faint chalk? no: just count area
print(keep:area())

--@ chunk 112
bankLine = {{540,420},{560,398},{590,392},{700,390},{740,388},{780,384},{820,388},{870,382},{920,386},{960,380},{1012,384}}
skyNew = above(function(x) return 404 end) - fp:mask() - keep:shrink(2) - (below(bankLine) * mask(function(x,y) return x > 545 and 1 or 0 end))
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return skyNew * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(-20, 95), {hand="broad", pile=skyT, coverage=2.4, fill=true, angle=hz, clip=skyNew})
work(band(80, 175), {hand="broad", pile=skyU, coverage=2.4, fill=true, angle=hz, clip=skyNew})
work(band(160, 255), {hand="broad", pile=skyR, coverage=2.4, fill=true, angle=hz, clip=skyNew})
work(band(240, 330), {hand="broad", pile=skyG, coverage=2.4, fill=true, angle=hz, clip=skyNew})
work(band(315, 420) * mask(function(x,y) return x < 680 and 1 or 0 end), {hand="body", tool="filbert 14", pile=skyL, coverage=2.4, fill=true, angle=hz, clip=skyNew})
work(band(315, 420) * mask(function(x,y) return x >= 680 and 1 or 0 end), {hand="body", tool="filbert 14", pile=skyG, coverage=2.4, fill=true, angle=hz, clip=skyNew})

--@ chunk 113
blend(skyNew, {angle=0.02})
blend(skyNew, {angle=-0.03})
blend((skyNew * below(function(x) return 60 end) * above(function(x) return 340 end)), {angle=0.0})

--@ chunk 114
local near = (keep:grow(25) - keep) * above(function(x) return 382 end)
local n = lose(-keep * above(function(x) return 382 end) * keep:grow(30), {pile=folD, tool="filbert 4", reach={8,7}, load=0.2, every=1.6, pressure={0.4,0.03}})
print(n)

--@ chunk 115
local ring = (keep:grow(38) - keep:shrink(1)) * above(function(x) return 384 end)
blend(ring, {angle=0.02})
blend(ring, {angle=0.6})
blend(ring, {angle=-0.6})

--@ chunk 116
ringS = (keep:grow(48) - keep:shrink(1)) * above(function(x) return 386 end) - (below(bankLine) * mask(function(x,y) return x > 545 and 1 or 0 end))
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return ringS * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(80, 175), {hand="body", tool="filbert 10", pile=skyU, coverage=2.6, fill=true, angle=hz, clip=ringS, load=0.9})
work(band(160, 255), {hand="body", tool="filbert 10", pile=skyR, coverage=2.6, fill=true, angle=hz, clip=ringS, load=0.9})
work(band(240, 330), {hand="body", tool="filbert 10", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=ringS, load=0.9})
work(band(315, 420), {hand="body", tool="filbert 10", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=ringS, load=0.9})

--@ chunk 117
blend(ringS:grow(20):soften(25) - keep:grow(2), {angle=0.0})

--@ chunk 118
local fb = brush{kind="filbert", width=4, stiffness=0.45}
local count = 0
for _, o in ipairs({E2, L2, R2}) do
  local L = o:length()
  local t = 0
  while t < 1 do
    local px, py = o:at(t)
    local qx, qy = o:at(math.min(1, t + 0.003))
    local tx, ty = qx-px, qy-py
    local len = math.sqrt(tx*tx+ty*ty) + 1e-6
    local nx, ny = -ty/len, tx/len
    if keep:at(px+nx*6, py+ny*6) > 0.5 then nx, ny = -nx, -ny end
    if py < 384 and px < 1005 then
      fb:load(folD, 0.35)
      local a = math.atan(ny, nx) + rand(-0.6, 0.6)
      local l = rand(6, 14)
      local sx, sy = px - nx*5, py - ny*5
      fb:stroke({{sx, sy}, {sx + math.cos(a)*l, sy + math.sin(a)*l}}, {pressure={0.5, 0.05}, ramps={0.1, 0.7}})
      count = count + 1
    end
    t = t + rand(5, 12) / L
  end
end
print(count)

--@ chunk 119
local rim = (keep:grow(18) - keep:shrink(3)) * above(function(x) return 384 end)
blend(rim:soften(4), {angle=-0.8})
blend(rim:soften(4), {angle=0.8})

--@ chunk 120
local d1 = keep:grow(8) - keep:shrink(2)
local d2 = keep:grow(16) - keep:grow(6)
local sky_only = above(function(x) return 386 end)
stipple(d1 * sky_only, {pile=folD, width=6, coverage=1.3, cluster={0.5,14}, pressure={0.4,0.7}, drag={3,-1.2}, twist=0.6})
stipple(d2 * sky_only, {pile=folD, width=5, coverage=0.45, cluster={0.75,12}, pressure={0.3,0.6}, drag={3,-1.2}, twist=0.6, feather=0.6})

--@ chunk 121
print(wait(8*24*60)); print(drying(700,140), drying(860,250), drying(620,300), drying(300,200))

--@ chunk 122
E3 = outline{{735,336},{706,330},{690,312},{670,300},{660,276},{662,250},{652,226},{662,198},{678,180},{682,154},{700,134},{712,110},{738,96},{758,80},{786,74},{810,80},{832,78},{852,96},{864,120},{880,138},{884,166},{898,190},{892,218},{900,246},{886,272},{868,288},{856,308},{828,318},{800,332},{776,328},{756,340}, closed=true, char="soft", seed=811, lobe=9, amount=1.1}
L3 = outline{{612,404},{588,396},{578,372},{582,350},{592,330},{602,308},{620,290},{642,282},{666,282},{684,294},{698,316},{704,342},{702,368},{694,392},{670,404}, closed=true, char="soft", seed=812, lobe=8, amount=1.0}
R3 = outline{{878,372},{860,350},{866,322},{882,300},{890,274},{912,252},{940,238},{966,240},{988,232},{1012,236},{1012,376},{980,374},{950,378},{914,374}, closed=true, char="soft", seed=813, lobe=9, amount=1.0}
K3 = E3:mask() + L3:mask() + R3:mask()
folBase = pile{{"raw umber",3},{"bone black",1},{"yellow ochre",0.8},{"Antwerp blue",0.2},{"lead white",0.35}, medium=0.15}
work(K3, {hand="body", tool="filbert 9", pile=folBase, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -1.2+0.8*math.sin(x/22+y/31) end, pressure={0.5,0.85}, load=0.85})

--@ chunk 123
folLit = pile{{"raw umber",1.6},{"yellow ochre",1.4},{"lead white",1.0},{"raw sienna",0.4},{"bone black",0.25},{"cobalt blue",0.1}, medium=0.15}
folCore = pile{{"raw umber",2},{"bone black",1.6},{"Antwerp blue",0.35}, medium=0.15}
local cb = brush{kind="filbert", width=9, stiffness=0.5}
-- lit tops of clumps on the glow side (left), small arcs
local lit = {{672,205},{690,170},{716,130},{748,104},{668,262},{690,240},{604,320},{630,298},{596,356},{660,300},{712,190},{740,150},{792,92},{884,268},{906,262},{940,250}}
for i,p in ipairs(lit) do
  cb:load(folLit, 0.45)
  local x,y = p[1]+rand(-4,4), p[2]+rand(-4,4)
  cb:stroke({{x-12,y+6},{x-4,y-2},{x+8,y-3},{x+14,y+3}}, {pressure={0.45,0.15}, ramps={0.2,0.5}, shake=1})
end
-- dark undersides of clumps
local db = brush{kind="filbert", width=10, stiffness=0.5}
local und = {{720,230},{770,180},{820,140},{800,260},{850,220},{740,300},{640,350},{660,390},{620,385},{930,320},{980,300},{900,350},{780,320},{830,295},{700,280}}
for i,p in ipairs(und) do
  db:load(folCore, 0.6)
  local x,y = p[1]+rand(-4,4), p[2]+rand(-4,4)
  db:stroke({{x-16,y-2},{x-6,y+6},{x+8,y+7},{x+18,y}}, {pressure={0.6,0.3}, ramps={0.2,0.4}, shake=1})
end

--@ chunk 124
local m = K3:shrink(4)
blend(m, {angle=-1.2})
blend(m, {angle=0.4})
blend(m, {angle=-0.5})

--@ chunk 125
print(wait(30*60)); print(drying(760,200), drying(640,350), drying(940,320))

--@ chunk 126
print(wait(48*60)); print(drying(760,200), drying(640,350), drying(940,320))

--@ chunk 127
print(wait(36*60)); print(drying(760,200), drying(640,350), drying(940,320))

--@ chunk 128
local function skypile(y) if y < 170 then return skyU elseif y < 250 then return skyR else return skyG end end
local hb = brush{kind="filbert", width=5, stiffness=0.5}
local function hole(cx, cy, n, spread)
  local pl = skypile(cy)
  hb:reload(pl, 0.6)
  for k=1,n do
    local x, y = cx + rand(-spread, spread), cy + rand(-spread*0.7, spread*0.7)
    local a = rand(-0.5,0.5)
    local l = rand(2,6)
    hb:stroke({{x,y},{x+l*math.cos(a), y+l*math.sin(a)}}, {pressure=rand(0.3,0.6), ramps={0.2,0.5}})
    if k % 3 == 0 then hb:load(pl, 0.5) end
  end
end
-- the gap between the elm and the left tree
local gap = outline{{648,282},{660,268},{676,262},{690,268},{694,284},{684,296},{668,292}, closed=true, char="soft", seed=901, lobe=4}
work(gap:mask(), {hand="body", tool="filbert 5", pile=skyG, coverage=2.4, fill=true, edge="soft", angle=0.1})
hole(700, 300, 6, 7)
-- holes near crown edges
hole(690,150,5,6); hole(722,112,4,5); hole(672,215,4,5); hole(870,140,5,6); hole(890,225,4,5); hole(845,300,5,6)
hole(600,330,4,5); hole(588,372,3,4); hole(918,258,4,5); hole(990,250,4,5); hole(880,320,4,5)
-- a couple inside, smaller
hole(800,180,3,4); hole(760,240,3,4); hole(950,300,3,3)

--@ chunk 129
local rr = rag{width=16}
local pts = {{700,300},{690,150},{722,112},{672,215},{870,140},{890,225},{845,300},{600,330},{588,372},{918,258},{990,250},{880,320},{800,180},{760,240},{950,300}}
for i,p in ipairs(pts) do
  rr:blot(p[1], p[2], {pressure=0.8})
  rr:wipe({{p[1]-8,p[2]},{p[1]+8,p[2]}}, {pressure=0.8})
  rr:refold()
end
local rg = rag{width=30}
rg:wipe(ellipse(670,280,30,22), {pressure=0.8, passes=2})
print(drying(670,280))

--@ chunk 130
print(wait(10*24*60)); print(drying(760,200), drying(640,350), drying(940,320), drying(670,280), drying(700,300))

--@ chunk 131
plaster = K3:grow(22) * above(function(x) return 392 end) - (below(bankLine) * mask(function(x,y) return x > 545 and 1 or 0 end))
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return plaster * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
-- thick, opaque: two coats of the band colours
for pass=1,2 do
  work(band(-20, 95), {hand="body", tool="filbert 12", pile=skyT, coverage=2.2, fill=true, angle=hz, clip=plaster, load=0.9})
  work(band(80, 175), {hand="body", tool="filbert 12", pile=skyU, coverage=2.2, fill=true, angle=hz, clip=plaster, load=0.9})
  work(band(160, 255), {hand="body", tool="filbert 12", pile=skyR, coverage=2.2, fill=true, angle=hz, clip=plaster, load=0.9})
  work(band(240, 330), {hand="body", tool="filbert 12", pile=skyG, coverage=2.2, fill=true, angle=hz, clip=plaster, load=0.9})
  work(band(315, 420), {hand="body", tool="filbert 12", pile=skyG, coverage=2.2, fill=true, angle=hz, clip=plaster, load=0.9})
end
blend(plaster:grow(10):soften(15), {angle=0.02})

--@ chunk 132
print(wait(10*24*60)); print(drying(760,200), drying(640,350), drying(940,320), drying(800,100))

--@ chunk 133
print(wait(6*24*60)); print(drying(760,200), drying(640,350), drying(940,320), drying(800,100))

--@ chunk 134
trunkP2 = pile{{"raw umber",3},{"bone black",1.3},{"Indian red",0.15},{"cobalt blue",0.2}, medium=0.12}
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.7}
  rb:load(trunkP2, 0.8)
  rb:stroke(pts, {pressure=pr or {0.9,0.1}, ramps={0.05,0.6}, shake=0.8})
end
limb({{762,420},{761,385},{758,350},{755,322}}, 12, {1,0.85})
limb({{755,325},{742,290},{728,250},{715,210},{706,175},{700,140}}, 7.5)
limb({{757,322},{764,280},{772,235},{782,190},{792,150},{796,110}}, 7.5)
limb({{758,326},{788,296},{818,262},{846,228},{866,200}}, 6.5)
limb({{753,330},{724,308},{694,290},{670,280}}, 5.5)
limb({{772,240},{802,212},{830,190},{850,170}}, 4)
limb({{726,252},{702,232},{684,218}}, 4)
limb({{784,190},{776,150},{766,118}}, 3.5)
limb({{714,205},{736,170},{750,140}}, 3)
limb({{818,262},{838,252},{870,248}}, 3)
limb({{646,420},{645,388},{641,358}}, 8, {1,0.7})
limb({{642,362},{626,334},{612,310}}, 4.5)
limb({{643,362},{660,334},{674,310}}, 4.5)
limb({{938,420},{935,384},{931,350},{927,322}}, 10, {1,0.75})
limb({{929,330},{908,292},{892,262}}, 5.5)
limb({{931,328},{954,288},{978,258},{1004,242}}, 5.5)
limb({{931,340},{960,320},{990,312}}, 4)

--@ chunk 135
clE = {{705,150,30},{740,115,34},{785,95,32},{828,112,30},{858,150,28},{690,200,30},{735,175,36},{790,150,38},{840,190,34},{872,232,26},{700,250,30},{750,230,36},{805,225,36},{850,262,28},{680,285,24},{730,290,30},{790,285,32},{835,300,24},{760,320,22}}
clL = {{620,312,24},{660,305,24},{605,350,24},{645,345,28},{680,345,22},{600,385,20},{640,385,24},{680,385,20}}
clR = {{905,268,24},{945,252,28},{988,248,26},{890,310,26},{935,300,30},{980,295,28},{880,350,22},{925,345,26},{975,345,26}}
nz = noise{seed=77, period=30, octaves=3}
function dens(x, y)
  local d = 0
  for _, list in ipairs({clE, clL, clR}) do
    for _, c in ipairs(list) do
      local dx, dy = (x-c[1])/c[3], (y-c[2])/(c[3]*0.8)
      local r2 = dx*dx+dy*dy
      if r2 < 4 then d = math.max(d, math.exp(-r2*1.1)) end
    end
  end
  d = d + 0.25*nz(x,y)
  return clamp(d, 0, 1)
end
treeZone = mask(function(x,y) return dens(x,y) > 0.08 and 1 or 0 end)
print(treeZone:area())

--@ chunk 136
function dens(x, y)
  local d = 0
  for _, list in ipairs({clE, clL, clR}) do
    for _, c in ipairs(list) do
      local dx, dy = (x-c[1])/c[3], (y-c[2])/(c[3]*0.8)
      local r2 = dx*dx+dy*dy
      if r2 < 4 then d = math.max(d, math.exp(-r2*1.1)) end
    end
  end
  if d > 0.02 then d = d + 0.3*nz(x,y)*math.min(1, d*3) end
  return clamp(d, 0, 1)
end
treeZone = mask(function(x,y) return dens(x,y) > 0.06 and 1 or 0 end)
print(treeZone:area())

--@ chunk 137
folA = pile{{"raw umber",3},{"bone black",1},{"yellow ochre",0.7},{"Antwerp blue",0.2},{"lead white",0.3}, medium=0.15}
-- first, a test on the left tree only
local zone = treeZone * mask(function(x,y) return x < 700 and y > 280 and 1 or 0 end)
stipple(zone, {pile=folA, tool={kind="filbert", width=7, stiffness=0.5, splay=0.4, ragged=0.5}, coverage=function(x,y) local d = dens(x,y); return 2.5*d*d end, pressure={0.35,0.75}, drag={5, -1.3}, twist=0.6, cluster={0.4, 10}, dips={14, 0.6, 0.3}})

--@ chunk 138
folB = pile{{"raw umber",3},{"bone black",1.3},{"yellow ochre",0.5},{"Antwerp blue",0.25},{"lead white",0.2}, medium=0.08}
local zone = treeZone * mask(function(x,y) return x < 700 and y > 280 and 1 or 0 end)
stipple(zone, {pile=folB, tool={kind="filbert", width=8, stiffness=0.5, splay=0.4, ragged=0.5}, coverage=function(x,y) local d = dens(x,y); return 3*d*d end, pressure={0.5,0.9}, drag={5, -1.3}, twist=0.6, cluster={0.4, 10}, dips={10, 0.9, 0.2}})

--@ chunk 139
local zone = treeZone * mask(function(x,y) return (x < 700 and y > 280) and 0 or 1 end)
local tool = {kind="filbert", width=8, stiffness=0.5, splay=0.4, ragged=0.5}
stipple(zone, {pile=folA, tool=tool, coverage=function(x,y) local d = dens(x,y); return 2.5*d*d end, pressure={0.35,0.75}, drag={5, -1.3}, twist=0.6, cluster={0.4, 10}, dips={14, 0.6, 0.3}})
stipple(zone, {pile=folB, tool=tool, coverage=function(x,y) local d = dens(x,y); return 3*d*d end, pressure={0.5,0.9}, drag={5, -1.3}, twist=0.6, cluster={0.4, 10}, dips={10, 0.9, 0.2}})

--@ chunk 140
folC = pile{{"raw umber",2.5},{"bone black",1.6},{"Antwerp blue",0.35},{"yellow ochre",0.2}, medium=0.08}
function coreD(x, y)
  local d = 0
  for _, list in ipairs({clE, clL, clR}) do
    for _, c in ipairs(list) do
      local dx, dy = (x-c[1]-c[3]*0.2)/(c[3]*0.75), (y-c[2]-c[3]*0.3)/(c[3]*0.55)
      local r2 = dx*dx+dy*dy
      if r2 < 4 then d = math.max(d, math.exp(-r2*1.2)) end
    end
  end
  return d
end
local tool = {kind="filbert", width=7, stiffness=0.5, splay=0.4, ragged=0.5}
stipple(treeZone, {pile=folC, tool=tool, coverage=function(x,y) local d = coreD(x,y); return 2.2*d*d*(0.6+0.6*smoothstep(600,900,x)) end, pressure={0.5,0.85}, drag={4, -1.2}, twist=0.6, cluster={0.4, 10}, dips={10, 0.8, 0.2}})

--@ chunk 141
blend(treeZone:grow(6):soften(6), {angle=-1.1})

--@ chunk 142
print(wait(6*24*60)); print(drying(760,200), drying(640,350), drying(940,320))

--@ chunk 143
undergrowth = outline{{548,440},{552,418},{566,404},{586,398},{606,392},{630,394},{660,386},{690,392},{715,384},{740,390},{770,380},{800,388},{830,382},{860,390},{890,384},{915,390},{945,382},{975,388},{1012,384},{1012,450},{950,448},{850,444},{750,442},{650,444}, closed=true, char="soft", seed=1001, lobe=8, amount=1.6}
ugD = pile{{"raw umber",3},{"bone black",1.3},{"yellow ochre",0.4},{"Antwerp blue",0.25},{"lead white",0.15}, medium=0.12}
ugM = pile{{"raw umber",2},{"bone black",0.7},{"yellow ochre",0.8},{"lead white",0.8},{"cobalt blue",0.3},{"Indian red",0.15}, medium=0.12}
work(undergrowth:mask(), {hand="body", tool="filbert 8", pile=ugD, coverage=2.4, fill=true, edge="soft", angle=function(x,y) return -0.3+0.8*math.sin(x/18) end})
-- the top edge broken with upward touches
stipple(undergrowth:mask():rim(10,4) * above(function(x) return 405 end), {pile=ugD, tool={kind="filbert", width=5, splay=0.4, ragged=0.5}, coverage=0.8, pressure={0.4,0.7}, drag={4,-1.5}, cluster={0.5,10}})
-- the far (left) end cooler and grayer: atmosphere
work(undergrowth:mask() * mask(function(x,y) return x < 620 and 1 or 0 end):blur(25), {hand="scumble", pile=ugM, coverage=0.9, pressure={0.3,0.5}, load=0.4, angle=-0.2})

--@ chunk 144
local rr = rag{width=22}
rr:dip(0.4)
local gaps = {{690,398},{720,396},{800,394},{835,396},{870,398},{975,396},{1000,394},{630,404}}
for i,p in ipairs(gaps) do
  rr:wipe({{p[1]-14,p[2]-6},{p[1]+14,p[2]-4}}, {pressure=0.75})
  rr:wipe({{p[1]-10,p[2]},{p[1]+10,p[2]+2}}, {pressure=0.5})
  rr:refold()
  if i%3==0 then rr:dip(0.4) end
end
blend(undergrowth:mask() * mask(function(x,y) return x < 640 and 1 or 0 end):blur(20):soften(6), {angle=-0.2})
blend(undergrowth:mask() * mask(function(x,y) return x < 640 and 1 or 0 end):blur(20):soften(6), {angle=0.5})

--@ chunk 145
local seam = rect(615, 380, 50, 75):soften(12) * undergrowth:mask()
blend(seam, {angle=0})
blend(seam, {angle=0.7})
blend(seam, {angle=-0.7})

--@ chunk 146
print(wait(8*24*60)); print(drying(800,420), drying(600,420))

--@ chunk 147
-- new bush line: lower, broken, with gaps of sky under the crowns
newTop = {{545,436},{560,424},{575,418},{596,414},{612,418},{628,412},{648,416},{668,410},{684,418},{700,420},{716,414},{736,418},{752,410},{770,414},{786,420},{806,424},{826,418},{846,412},{866,418},{886,424},{904,416},{924,410},{944,416},{962,420},{984,414},{1012,412}}
local skyStrip = above(newTop) * below(function(x) return 380 end) * mask(function(x,y) return x > 540 and 1 or 0 end) - treeZone:shrink(3)
work(skyStrip, {hand="body", tool="filbert 6", pile=skyG, coverage=2.6, fill=true, angle=0.05, clip=skyStrip:grow(2), load=0.9})
print(skyStrip:area())

--@ chunk 148
print(wait(5*24*60)); print(drying(800,400), drying(543,450))

--@ chunk 149
print(wait(5*24*60)); print(drying(800,400), drying(543,450))

--@ chunk 150
print(wait(4*24*60)); print(drying(800,400), drying(543,450), drying(543,500))

--@ chunk 151
local strip = rect(528, 440, 36, 240):soften(4)
local hz = function(x,y) return 0.04*math.sin(x/90) end
work(strip * above(function(x) return 540 end), {hand="body", tool="filbert 8", pile=meadMid, coverage=2.4, fill=true, angle=hz, clip=strip:grow(4)})
work(strip * below(function(x) return 530 end), {hand="body", tool="filbert 8", pile=meadDark, coverage=2.4, fill=true, angle=hz, clip=strip:grow(4)})
blend(rect(510, 430, 70, 250):soften(15), {angle=0.05})

--@ chunk 152
bushClumps = {{560,428,14},{585,420,18},{615,416,20},{650,420,18},{690,418,22},{730,422,20},{775,416,24},{820,420,22},{860,416,20},{900,420,24},{945,414,22},{985,416,24},{1010,420,18}}
bn = noise{seed=91, period=14, octaves=3}
function bushD(x,y)
  local d = 0
  for _,c in ipairs(bushClumps) do
    local dx, dy = (x-c[1])/c[3], (y-c[2])/(c[3]*0.7)
    if dy > 0 then dy = dy*0.35 end
    local r2 = dx*dx+dy*dy
    if r2 < 4 then d = math.max(d, math.exp(-r2*1.3)) end
  end
  if y > 432 then d = math.max(d, smoothstep(432, 445, y)) end
  if x < 548 then d = d * smoothstep(538, 552, x) end
  if y > 452 then d = 0 end
  d = d + 0.3*bn(x,y)*math.min(1,d*3)
  return clamp(d,0,1)
end
bushZone = mask(function(x,y) if x < 535 or y < 380 or y > 455 then return 0 end; return bushD(x,y) > 0.08 and 1 or 0 end)
local base = mask(function(x,y) if x < 535 or y < 380 or y > 455 then return 0 end; return bushD(x,y) > 0.55 and 1 or 0 end)
work(base, {hand="body", tool="filbert 7", pile=ugD, coverage=2.4, fill=true, angle=function(x,y) return -0.2+0.8*math.sin(x/15) end, clip=base:grow(3)})
local tool = {kind="filbert", width=6, stiffness=0.5, splay=0.4, ragged=0.5}
stipple(bushZone, {pile=ugD, tool=tool, coverage=function(x,y) local d=bushD(x,y); return 2.4*d*d end, pressure={0.4,0.8}, drag={4,-1.4}, twist=0.6, cluster={0.4,8}, dips={10,0.8,0.2}})

--@ chunk 153
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.5}
  rb:load(trunkP2, 0.8)
  rb:stroke(pts, {pressure=pr or {0.9,0.6}, ramps={0.05,0.2}, shake=0.6})
end
limb({{763,428},{762,400},{761,372}}, 12, {1,0.95})
limb({{646,428},{645,405},{645,385}}, 8, {1,0.9})
limb({{939,428},{937,400},{936,372}}, 10, {1,0.95})
-- crown bottoms hanging over the straight line; and the left tree's skirt
function hangD(x,y)
  local d = 0
  local cl = {{600,392,22},{632,398,20},{668,394,22},{700,388,18},{725,380,16},{790,378,18},{830,380,16},{880,384,18},{915,380,16},{960,378,18},{995,382,16}}
  for _,c in ipairs(cl) do
    local dx, dy = (x-c[1])/c[3], (y-c[2])/(c[3]*0.7)
    local r2 = dx*dx+dy*dy
    if r2 < 4 then d = math.max(d, math.exp(-r2*1.3)) end
  end
  d = d + 0.3*nz(x,y)*math.min(1,d*3)
  return clamp(d,0,1)
end
local hz = mask(function(x,y) if y < 360 or y > 420 or x < 560 then return 0 end; return hangD(x,y) > 0.08 and 1 or 0 end)
local tool = {kind="filbert", width=7, stiffness=0.5, splay=0.4, ragged=0.5}
stipple(hz, {pile=folA, tool=tool, coverage=function(x,y) local d=hangD(x,y); return 2.4*d*d end, pressure={0.35,0.75}, drag={5,-1.3}, twist=0.6, cluster={0.4,10}, dips={14,0.6,0.3}})
stipple(hz, {pile=folB, tool=tool, coverage=function(x,y) local d=hangD(x,y); return 2.6*d*d end, pressure={0.5,0.9}, drag={5,-1.3}, twist=0.6, cluster={0.4,10}, dips={10,0.9,0.2}})

--@ chunk 154
print(wait(8*24*60)); print(drying(800,430), drying(760,400), drying(700,370))

--@ chunk 155
g1 = outline{{706,410},{712,400},{728,396},{744,398},{752,404},{748,412},{730,414}, closed=true, char="soft", seed=1101, lobe=4}
g2 = outline{{790,408},{800,398},{822,393},{850,394},{874,398},{884,406},{872,412},{830,413}, closed=true, char="soft", seed=1102, lobe=5}
g3 = outline{{962,406},{970,398},{990,396},{1004,402},{998,410},{975,411}, closed=true, char="soft", seed=1103, lobe=4}
gaps = g1:mask() + g2:mask() + g3:mask()
darkBand = outline{{548,452},{545,430},{556,410},{572,392},{596,382},{630,372},{680,366},{740,362},{800,362},{860,364},{920,362},{980,360},{1012,362},{1012,454},{900,452},{750,452},{640,454}, closed=true, char="soft", seed=1104, lobe=12, amount=1.2}
local m = darkBand:mask() - gaps
work(m, {hand="body", tool="filbert 10", pile=ugD, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -0.3+0.8*math.sin(x/17+y/23) end, load=0.85, pressure={0.5,0.85}})

--@ chunk 156
local rr = rag{width=18}
rr:dip(0.6)
for _, s in ipairs({{{710,405},{748,404}}, {{795,404},{880,403}}, {{965,404},{1000,404}}, {{714,409},{744,409}}, {{805,399},{868,398}}}) do
  rr:wipe(s, {pressure=0.85})
  rr:refold(); rr:dip(0.5)
end
local tr = darkBand:mask() * above(function(x) return 392 end)
blend(tr:soften(10), {angle=-1.3})
blend(tr:soften(10), {angle=-0.5})

--@ chunk 157
print(wait(10*24*60)); print(drying(800,430), drying(560,430))

--@ chunk 158
print(wait(6*24*60)); print(drying(800,430), drying(700,400))

--@ chunk 159
grove = treeZone:grow(4) + darkBand:mask()
groveGl = pile{{"raw umber",2},{"bone black",1.4},{"Antwerp blue",0.3},{"raw sienna",0.3}, medium=0.55, thinner=0.15}
work(grove:shrink(3):soften(6), {hand="glaze", tool={kind="filbert", width=16, stiffness=0.3}, pile=groveGl, coverage=2.2, fill=true, clip=grove:grow(2), angle=-1.0, length={40,90}, pressure={0.5,0.8},
  load_at=function(x,y) return 0.5 + 0.5*smoothstep(620,820,x) end})

--@ chunk 160
blend(grove:shrink(2), {angle=-1.1})
blend(grove:shrink(2), {angle=0.2})

--@ chunk 161
print(wait(5*24*60)); print(drying(760,200), drying(800,420))

--@ chunk 162
-- sky down over the top of the far tree band
local cut = below(function(x) return 330 end) * above(function(x) return 392 end) * mask(function(x,y) return x < 548 and 1 or 0 end)
work(cut, {hand="body", tool="filbert 12", pile=skyL, coverage=2.6, fill=true, angle=function(x,y) return 0.03*math.sin(x/70) end, load=0.9})
blend(cut:soften(8), {angle=0.02})

--@ chunk 163
farG = pile{{"lead white",3},{"raw umber",1.1},{"cobalt blue",0.45},{"yellow ochre",0.35},{"Indian red",0.2}, medium=0.12}
farG2 = pile{{"lead white",2},{"raw umber",1.4},{"cobalt blue",0.45},{"yellow ochre",0.3},{"Indian red",0.2},{"bone black",0.15}, medium=0.12}
farTL = outline{{-10,412},{-10,392},{20,388},{42,383},{60,386},{78,378},{96,372},{108,364},{122,358},{138,356},{152,360},{164,366},{176,374},{196,380},{214,384},{236,382},{256,386},{280,388},{310,386},{335,389},{362,390},{392,388},{420,391},{455,392},{490,393},{520,394},{550,396},{550,412}, closed=true, char="soft", seed=1201, lobe=6, amount=1.3}
work(farTL:mask(), {hand="body", tool="filbert 6", pile=farG, coverage=2.4, fill=true, edge="soft", angle=function(x,y) return -1.5+0.3*math.sin(x/11) end, pressure={0.4,0.7}})
local grv = outline{{92,408},{96,378},{108,366},{124,360},{140,358},{154,363},{166,372},{174,384},{178,408}, closed=true, char="soft", seed=1202, lobe=5}
work(grv:mask(), {hand="body", tool="filbert 5", pile=farG2, coverage=1.8, fill=true, edge="soft", angle=-1.45, pressure={0.4,0.7}})
blend(farTL:mask():grow(4):soften(4), {angle=-1.4})
blend(farTL:mask():grow(4):soften(4), {angle=0.1})

--@ chunk 164
local lowerBand = farTL:mask() * below(function(x) return 384 + 4*math.sin(x/37) end)
work(lowerBand, {hand="body", tool="filbert 6", pile=farG2, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -1.5+0.4*math.sin(x/9) end, pressure={0.5,0.85}, load=0.9})
local fb = brush{kind="filbert", width=5, stiffness=0.45}
for i, x in ipairs(uneven(40, 0, 545, 0.6, 0.5, 12)) do
  local top = 0
  local m = farTL:mask()
  local y = 400
  while y > 350 and m:at(x, y) > 0.5 do y = y - 2 end
  fb:load(farG2, 0.5)
  fb:stroke({{x+rand(-2,2), 404},{x+rand(-2,2), y+rand(3,8)}}, {pressure={0.55,0.15}, ramps={0.1,0.5}})
end

--@ chunk 165
mistField = pile{{"lead white",4},{"yellow ochre",0.6},{"raw umber",0.5},{"cobalt blue",0.2},{"Indian red",0.1}, medium=0.12}
local base = rect(-10, 392, 560, 12)
work(base, {hand="body", tool="filbert 6", pile=farG2, coverage=2.6, fill=true, angle=0, clip=base:grow(2), load=0.9})
local field = poly({{-10,403},{550,403},{550,420},{300,420},{230,428},{-10,428}})
work(field, {hand="body", tool="filbert 8", pile=mistField, coverage=2.4, fill=true, angle=0.02, edge="soft", load=0.8})
blend(rect(-10, 380, 560, 45):soften(4), {angle=0})

--@ chunk 166
local fb = brush{kind="filbert", width=6, stiffness=0.4}
local xs = uneven(70, 0, 548, 0.7, 0.6, 33)
for i, x in ipairs(xs) do
  local h = 8 + 10*math.abs(nz(x*3, 50)) + (math.abs(x-140) < 45 and 14 or 0)
  fb:load(farG2, 0.6)
  fb:stroke({{x+rand(-2,2), 398},{x+rand(-2,2), 386 - h}}, {pressure={0.65,0.2}, ramps={0.1,0.5}})
end
-- the tree band base a bit darker, wet into wet
local bb = brush{kind="filbert", width=8}
for x = 0, 540, 22 do
  bb:load(farG2, 0.6)
  bb:stroke({{x, 396+rand(-1,1)},{x+26, 396+rand(-1,1)}}, {pressure=0.6})
end

--@ chunk 167
local band = rect(-10, 352, 560, 50):soften(6)
blend(band, {angle=0})
blend(band, {angle=0.9})
blend(band, {angle=-0.9})
blend(band, {angle=0})

--@ chunk 168
profPts = {{-10,384},{20,380},{45,376},{62,379},{80,372},{100,366},{118,360},{136,357},{152,360},{168,366},{186,374},{210,378},{232,375},{252,380},{276,383},{300,380},{322,384},{350,386},{380,383},{400,387},{432,389},{462,387},{490,390},{520,391},{556,393}}
local skyCut = above(profPts) * below(function(x) return 340 end) * mask(function(x,y) return x < 552 and 1 or 0 end)
work(skyCut, {hand="body", tool="filbert 7", pile=skyL, coverage=2.8, fill=true, angle=0, clip=skyCut, load=0.9})
blend(skyCut:grow(3):soften(3) * above(function(x) return 395 end), {angle=0})

--@ chunk 169
print(wait(8*24*60)); print(drying(300,390), drying(300,415), drying(540,600))

--@ chunk 170
cornerCurve = {{520,452},{535,446},{550,436},{565,424},{585,412},{605,404},{625,396},{645,390},{660,386}}
local corner = above(cornerCurve) * rect(510, 330, 140, 130) - treeZone:shrink(2)
local skyPart = corner * above(function(x) return 394 end)
local mistPart = corner * below(function(x) return 392 end) * above(function(x) return 420 end)
local waterPart = corner * below(function(x) return 418 end)
work(skyPart, {hand="body", tool="filbert 6", pile=skyL, coverage=2.6, fill=true, angle=0, clip=skyPart, load=0.9})
work(mistPart, {hand="body", tool="filbert 6", pile=farG2, coverage=2.6, fill=true, angle=0, clip=mistPart, load=0.9})
work(waterPart, {hand="body", tool="filbert 6", pile=pondGray, coverage=2.6, fill=true, angle=0, clip=waterPart, load=0.9})
blend(corner:soften(3), {angle=0})

--@ chunk 171
shore = {{460,470},{520,466},{560,462},{600,456},{660,446},{700,436}}
local R = rect(470, 338, 220, 135) * above(shore) - treeZone:shrink(1) - (below(cornerCurve) * mask(function(x,y) return x > 520 and 1 or 0 end))
local hill = function(x) return 391 + (x-490)*0.03 end
local sk = R * above(hill)
local hb = R * below(hill) * above(function(x) return 404 end)
local mf = R * below(function(x) return 403 end) * above(function(x) return 418 end)
local wt = R * below(function(x) return 417 end)
work(sk, {hand="body", tool="filbert 6", pile=skyL, coverage=2.6, fill=true, angle=0, clip=sk:grow(1), load=0.9})
work(hb, {hand="body", tool="filbert 5", pile=farG2, coverage=2.6, fill=true, angle=0, clip=hb:grow(1), load=0.9})
work(mf, {hand="body", tool="filbert 5", pile=mistField, coverage=2.6, fill=true, angle=0, clip=mf:grow(1), load=0.9})
work(wt, {hand="body", tool="filbert 6", pile=pondLight, coverage=2.6, fill=true, angle=0, clip=wt:grow(1), load=0.9})
blend(R:soften(2), {angle=0})

--@ chunk 172
shoreAll = {{-10,428},{100,426},{205,436},{260,452},{330,466},{420,470},{520,466},{600,456},{660,446},{700,436},{720,432}}
hillPts = {{-10,384},{20,380},{45,376},{62,379},{80,372},{100,366},{118,360},{136,357},{152,360},{168,366},{186,374},{210,378},{232,375},{252,380},{276,383},{300,380},{322,384},{350,386},{380,383},{400,387},{432,389},{462,387},{490,390},{520,391},{556,393},{600,394},{650,395},{720,396}}
strip = rect(-10, 330, 730, 150) * above(shoreAll) - treeZone:shrink(1)
local sk = strip * above(hillPts)
local hb = strip * below(hillPts) * above(function(x) return 405 end)
local mf = strip * below(function(x) return 404 end) - pondMask:shrink(2)
local wt = strip * pondMask:shrink(2) * below(function(x) return 404 end)
work(sk, {hand="body", tool="filbert 8", pile=skyL, coverage=2.6, fill=true, angle=0, clip=sk:grow(2), load=0.9})
work(hb, {hand="body", tool="filbert 6", pile=farG2, coverage=2.6, fill=true, angle=0, clip=hb:grow(2), load=0.9})
work(mf, {hand="body", tool="filbert 6", pile=mistField, coverage=2.6, fill=true, angle=0, clip=mf:grow(2), load=0.9})
work(wt, {hand="body", tool="flat 8", pile=pondGray, coverage=2.4, fill=true, angle=0, clip=wt:grow(1), load=0.9})
local core = ellipse(400, 437, 190, 14):soften(10) * wt
work(core, {hand="body", tool="flat 8", pile=pondLight, coverage=2.4, fill=true, angle=0, clip=wt, load=0.9})

--@ chunk 173
farBank = outline{{548,418},{552,404},{566,398},{590,396},{612,392},{640,394},{668,392},{696,396},{726,398},{726,420},{690,418},{640,420},{590,420}, closed=true, char="soft", seed=1301, lobe=6, amount=1.4}
work(farBank:mask(), {hand="body", tool="filbert 6", pile=ugD, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -0.2+0.7*math.sin(x/12) end, load=0.85})
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.5}
  rb:load(trunkP2, 0.8)
  rb:stroke(pts, {pressure=pr or {0.9,0.6}, ramps={0.05,0.2}, shake=0.6})
end
limb({{646,404},{645,385},{645,370}}, 7, {1,0.9})
limb({{612,402},{611,388},{610,378}}, 4, {0.9,0.6})

--@ chunk 174
-- reflections: vertical drags down from the bank into the wet water
local rb = brush{kind="filbert", width=7, stiffness=0.4}
for i, x in ipairs(uneven(26, 552, 724, 0.4, 0.3, 44)) do
  rb:load(ugD, 0.35)
  local y0 = 416
  local y1 = 430 + rand(4, 20)
  rb:stroke({{x, y0},{x+rand(-1,1), y1}}, {pressure={0.6,0.15}, ramps={0.1,0.6}})
end
-- reflection of the left tree's crown, broken and soft
for i, x in ipairs(uneven(12, 585, 700, 0.5, 0.3, 45)) do
  rb:load(folA, 0.25)
  rb:stroke({{x, 425},{x+rand(-1,1), 440+rand(0,12)}}, {pressure={0.45,0.1}, ramps={0.1,0.6}})
end
blend((rect(548, 412, 180, 50) * pondMask:grow(40) - farBank:mask()):soften(3) * above(shoreAll), {angle=1.5708})

--@ chunk 175
print(wait(8*24*60)); print(drying(640,410), drying(400,440), drying(600,440), drying(300,360))

--@ chunk 176
print(wait(8*24*60)); print(drying(640,410), drying(400,440), drying(600,440), drying(300,360), drying(620,380))

--@ chunk 177
function massD(x, y)
  local d = 0
  -- left tree skirt, continuous with crown above
  local cl = {{595,365,24},{625,375,26},{660,372,26},{690,368,22},{612,395,20},{650,398,22},{690,396,20},{720,388,18}}
  for _,c in ipairs(cl) do
    local dx, dy = (x-c[1])/c[3], (y-c[2])/(c[3]*0.75)
    local r2 = dx*dx+dy*dy
    if r2 < 4 then d = math.max(d, math.exp(-r2*1.2)) end
  end
  -- far bank line under it
  if x > 552 and x < 735 and y > 396 and y < 416 then
    local e = smoothstep(552, 575, x) * (1 - smoothstep(722, 738, x)) * smoothstep(394, 402, y) * (1 - smoothstep(410, 418, y))
    d = math.max(d, e)
  end
  -- near bush clump at the right block's edge, falling to the shore
  local nb = {{735,400,26},{722,425,22},{745,440,24},{705,440,16},{690,446,12}}
  for _,c in ipairs(nb) do
    local dx, dy = (x-c[1])/c[3], (y-c[2])/(c[3]*0.8)
    local r2 = dx*dx+dy*dy
    if r2 < 4 then d = math.max(d, math.exp(-r2*1.2)) end
  end
  if x > 740 and y > 380 and y < 452 then d = math.max(d, smoothstep(740, 760, x)) end
  d = d + 0.3*nz(x,y)*math.min(1,d*3)
  return clamp(d,0,1)
end
massZone = mask(function(x,y) if x < 540 or x > 780 or y < 340 or y > 462 then return 0 end; return massD(x,y) > 0.07 and 1 or 0 end)
local core = mask(function(x,y) if x < 540 or x > 780 or y < 340 or y > 462 then return 0 end; return massD(x,y) > 0.6 and 1 or 0 end)
work(core, {hand="body", tool="filbert 7", pile=ugD, coverage=2.4, fill=true, angle=function(x,y) return -0.3+0.8*math.sin(x/15+y/20) end, clip=core:grow(2), load=0.85})
local tool = {kind="filbert", width=6, stiffness=0.5, splay=0.4, ragged=0.5}
stipple(massZone, {pile=folB, tool=tool, coverage=function(x,y) local d=massD(x,y); return 2.6*d*d end, pressure={0.4,0.85}, drag={4,-1.3}, twist=0.6, cluster={0.4,8}, dips={10,0.85,0.2}})

--@ chunk 178
bigMass = outline{{548,422},{552,404},{566,392},{584,380},{596,366},{620,356},{650,352},{680,356},{700,350},{722,346},{745,344},{770,338},{800,342},{830,338},{860,344},{890,340},{920,346},{950,340},{980,344},{1012,342},{1012,456},{940,454},{860,452},{780,452},{730,448},{700,442},{680,432},{640,424},{600,422}, closed=true, char="soft", seed=1401, lobe=10, amount=1.3}
local m = bigMass:mask()
work(m, {hand="body", tool="filbert 10", pile=ugD, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -0.4+0.9*math.sin(x/19+y/27) end, load=0.85, pressure={0.5,0.85}})
-- wet-into-wet variation: a few warmer olive and cooler gray-green passages
ugO = pile{{"raw umber",2},{"yellow ochre",1},{"bone black",0.6},{"lead white",0.4},{"raw sienna",0.3}, medium=0.12}
ugG = pile{{"raw umber",2},{"bone black",0.8},{"cobalt blue",0.4},{"lead white",0.6},{"yellow ochre",0.3}, medium=0.12}
local v1 = m * (ellipse(620,395,45,22) + ellipse(700,385,40,20) + ellipse(830,410,50,20)):soften(15)
local v2 = m * (ellipse(570,405,25,15) + ellipse(660,412,40,10)):soften(10)
work(v1, {hand="scumble", pile=ugO, coverage=0.8, pressure={0.3,0.5}, load=0.4, angle=-0.4})
work(v2, {hand="scumble", pile=ugG, coverage=0.9, pressure={0.3,0.5}, load=0.4, angle=0})

--@ chunk 179
local m = bigMass:mask():shrink(2)
blend(m, {angle=0.0})
blend(m, {angle=-0.8})
blend(m, {angle=0.6})
blend(m, {angle=0.1})

--@ chunk 180
print(wait(12*24*60)); print(drying(700,400), drying(600,420), drying(900,420))

--@ chunk 181
groveAll = treeZone:grow(6) + bigMass:mask():grow(3)
groveGl2 = pile{{"raw umber",2},{"bone black",1.6},{"Antwerp blue",0.3},{"raw sienna",0.3}, medium=0.6, thinner=0.1}
work(groveAll:shrink(4):soften(6), {hand="glaze", tool={kind="filbert", width=18, stiffness=0.3}, pile=groveGl2, coverage=2.8, fill=true, clip=groveAll:grow(2), angle=-1.0, length={40,90}, pressure={0.6,0.9},
  load_at=function(x,y) return 0.6 + 0.4*smoothstep(620,820,x) end})
blend(groveAll:shrink(3), {angle=-1.1})

--@ chunk 182
local rr = rag{width=22}
local arcs = {
 {{660,215},{675,195},{695,182}}, {{690,160},{708,140},{730,124}}, {{735,108},{758,96},{785,90}},
 {{655,262},{668,246},{690,238}}, {{700,205},{718,190},{742,182}}, {{590,330},{604,312},{624,302}},
 {{582,368},{594,352},{612,346}}, {{640,300},{656,290},{676,292}}, {{760,150},{780,140},{800,142}},
 {{880,262},{898,250},{918,246}}, {{935,240},{955,232},{975,236}}, {{565,400},{580,388},{600,384}},
 {{625,365},{645,356},{668,358}}, {{705,250},{722,240},{744,240}},
}
for i, a in ipairs(arcs) do
  rr:wipe(a, {pressure=0.45})
  if i % 3 == 0 then rr:refold() end
end

--@ chunk 183
print(wait(4*24*60)); print(drying(760,200), drying(800,420))

--@ chunk 184
veilHi = pile{{"lead white",6},{"Indian red",0.3},{"cobalt blue",0.35},{"yellow ochre",0.4},{"bone black",0.06}, medium=0.2, thinner=0.55}
veilLo = pile{{"lead white",6.5},{"yellow ochre",0.6},{"lemon chrome",0.25},{"cobalt blue",0.1},{"Indian red",0.08}, medium=0.2, thinner=0.55}
-- test on the left lobe edge: veil across the edge
local edge = (groveAll:grow(10) - groveAll:shrink(16)) * mask(function(x,y) return (x < 700 and y > 250 and y < 400) and 1 or 0 end)
work(edge, {hand="scumble", tool={kind="filbert", width=10, stiffness=0.4}, pile=veilLo, coverage=1.2, pressure={0.3,0.55}, load=0.6, angle=-0.6})

--@ chunk 185
local edge = (groveAll:grow(10) - groveAll:shrink(18)) * above(function(x) return 395 end) - mask(function(x,y) return (x < 700 and y > 250 and y < 400) and 1 or 0 end)
work(edge * above(function(x) return 200 end), {hand="scumble", tool={kind="filbert", width=10, stiffness=0.4}, pile=veilHi, coverage=1.2, pressure={0.3,0.55}, load=0.6, angle=-0.6})
work(edge * below(function(x) return 190 end), {hand="scumble", tool={kind="filbert", width=10, stiffness=0.4}, pile=veilLo, coverage=1.2, pressure={0.3,0.55}, load=0.6, angle=-0.6})
-- mist rising from the water, across the base of the grove
mistV = pile{{"lead white",6},{"yellow ochre",0.4},{"cobalt blue",0.2},{"Indian red",0.12},{"raw umber",0.1}, medium=0.25, thinner=0.6}
local mist = (below(function(x) return 385 + 8*math.sin(x/70) end) * above(function(x) return 440 end) * mask(function(x,y) return x > 540 and 1 or 0 end)):soften(10)
work(mist, {hand="glaze", tool={kind="filbert", width=20, stiffness=0.3}, pile=mistV, coverage=1.2, angle=0.02, pressure={0.3,0.5}, length={60,160},
  load_at=function(x,y) return 0.7 - 0.4*smoothstep(600,950,x) end})

--@ chunk 186
local mistA = (below(function(x) return 380 end) * above(function(x) return 450 end) * mask(function(x,y) return x > 535 and 1 or 0 end)):soften(8)
blend(mistA, {angle=0})
blend(mistA, {angle=0.05})
blend(mistA, {angle=-0.05})
local rimA = (groveAll:grow(14) - groveAll:shrink(22)) * above(function(x) return 395 end)
blend(rimA, {angle=-0.8})
blend(rimA, {angle=0.6})

--@ chunk 187
local rimA = (groveAll:grow(16) - groveAll:shrink(24)) * above(function(x) return 395 end)
local rr = rag{width=40}
rr:dip(0.7)
rr:wipe(rimA, {pressure=0.7, passes=2, refold=0.25, angle=-0.6})
rr = rag{width=40}
rr:dip(0.7)
local spots = ellipse(722,332,32,24) + ellipse(808,330,34,24)
rr:wipe(spots, {pressure=0.8, passes=2, refold=0.2})
rr:refold(); rr:dip(0.6)
local mistA = (below(function(x) return 360 end) * above(function(x) return 455 end) * mask(function(x,y) return x > 535 and 1 or 0 end))
rr:wipe(mistA, {pressure=0.6, passes=1, refold=0.25, angle=0})

--@ chunk 188
print(wait(5*24*60)); print(drying(700,120), drying(720,330), drying(800,420))

--@ chunk 189
-- rim of the crowns: dark stippled foliage again, densest inside, thinning outward
local outer = groveAll:grow(6)
local dz = groveAll:distance()
rimFol = pile{{"raw umber",3},{"bone black",1.4},{"yellow ochre",0.5},{"Antwerp blue",0.25},{"lead white",0.15}, medium=0.08}
local zone = (groveAll:grow(8) - groveAll:shrink(28)) * above(function(x) return 455 end)
local tool = {kind="filbert", width=7, stiffness=0.5, splay=0.4, ragged=0.5}
stipple(zone, {pile=rimFol, tool=tool, coverage=function(x,y) local inside = groveAll:at(x,y); return 0.5 + 2.2*inside end, pressure={0.45,0.85}, drag={4,-1.3}, twist=0.6, cluster={0.35,8}, dips={10,0.85,0.2}})
local spots = (ellipse(722,332,36,28) + ellipse(808,330,38,28)):soften(6)
stipple(spots, {pile=rimFol, tool=tool, coverage=3, pressure={0.5,0.9}, drag={4,-1.3}, twist=0.6, dips={10,0.85,0.2}})

--@ chunk 190
cl2 = {
 -- elm, tall vase
 {760,100,30},{730,120,28},{795,98,30},{828,118,28},{700,150,26},{850,150,26},{690,195,28},{865,195,26},{735,165,34},{800,160,36},
 {705,240,30},{860,240,26},{760,215,38},{820,225,34},{690,280,24},{740,280,32},{800,285,34},{850,290,26},{770,320,30},
 -- left tree, lower
 {625,318,24},{660,305,26},{600,350,24},{640,345,28},{680,340,24},{598,385,22},{640,385,26},{690,378,22},
 -- right tree
 {915,262,24},{955,248,28},{995,246,28},{895,305,26},{940,300,30},{985,295,30},{885,350,24},{935,345,28},{985,345,28},
 -- bush band
 {575,410,18},{620,418,22},{670,420,24},{720,415,26},{770,412,26},{820,415,26},{870,412,26},{920,414,26},{970,412,26},{1010,414,22},
 {720,438,22},{790,440,24},{860,440,24},{930,438,24},{1000,440,24},
}
nz2 = noise{seed=303, period=22, octaves=3}
function gdens(x,y)
  local d = 0
  for _,c in ipairs(cl2) do
    local dx, dy = (x-c[1])/c[3], (y-c[2])/(c[3]*0.85)
    local r2 = dx*dx+dy*dy
    if r2 < 5 then d = d + math.exp(-r2*1.1) end
  end
  if y > 395 and y < 452 and x > 700 then d = d + 1 end
  return d + 0.22*nz2(x,y)
end
newGrove = mask(function(x,y) if x < 540 or y < 40 or y > 456 then return 0 end; return gdens(x,y) > 0.42 and 1 or 0 end)
-- don't let it run below the shoreline on the left
newGrove = newGrove * above({{540,430},{600,428},{660,440},{700,446},{760,452},{1012,456}})
print(newGrove:area(), groveAll:area())

--@ chunk 191
for y = 40, 460, 14 do
  local s = ""
  for x = 540, 1000, 8 do
    local a = newGrove:at(x,y) > 0.5
    local b = groveAll:at(x,y) > 0.5
    s = s .. (a and "#" or (b and "." or " "))
  end
  print(string.format("%3d %s", y, s))
end

--@ chunk 192
table.insert(cl2, {770,350,22}); table.insert(cl2, {745,372,18}); table.insert(cl2, {960,380,22}); table.insert(cl2, {900,385,20})
newGrove = mask(function(x,y) if x < 540 or y < 40 or y > 456 then return 0 end; return gdens(x,y) > 0.42 and 1 or 0 end)
newGrove = newGrove * above({{540,430},{600,428},{660,440},{700,446},{760,452},{1012,456}})
for y = 290, 420, 10 do
  local s = ""
  for x = 540, 1000, 8 do
    local a = newGrove:at(x,y) > 0.5
    local b = groveAll:at(x,y) > 0.5
    s = s .. (a and "#" or (b and "." or " "))
  end
  print(string.format("%3d %s", y, s))
end

--@ chunk 193
local region = groveAll:grow(30) * above(function(x) return 397 end) - newGrove
region = region * mask(function(x,y) return x > 520 and 1 or 0 end)
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return region * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
for pass = 1, 2 do
  work(band(-20, 95), {hand="body", tool="filbert 10", pile=skyT, coverage=2.2, fill=true, angle=hz, clip=region, load=0.9})
  work(band(80, 175), {hand="body", tool="filbert 10", pile=skyU, coverage=2.2, fill=true, angle=hz, clip=region, load=0.9})
  work(band(160, 255), {hand="body", tool="filbert 10", pile=skyR, coverage=2.2, fill=true, angle=hz, clip=region, load=0.9})
  work(band(240, 330), {hand="body", tool="filbert 10", pile=skyG, coverage=2.2, fill=true, angle=hz, clip=region, load=0.9})
  work(band(315, 420), {hand="body", tool="filbert 10", pile=skyL, coverage=2.2, fill=true, angle=hz, clip=region, load=0.9})
end
-- the bit by the water at the far left end of the grove
local wat = groveAll:grow(10) * below(function(x) return 397 end) * mask(function(x,y) return x < 600 and 1 or 0 end) - newGrove
work(wat * above(function(x) return 404 end), {hand="body", tool="filbert 6", pile=farG2, coverage=2.4, fill=true, angle=0, clip=wat, load=0.9})
work(wat * below(function(x) return 403 end), {hand="body", tool="filbert 6", pile=pondGray, coverage=2.4, fill=true, angle=0, clip=wat, load=0.9})

--@ chunk 194
gOl = pile{{"raw umber",2},{"yellow ochre",1.2},{"bone black",0.5},{"lead white",0.7},{"raw sienna",0.3},{"cobalt blue",0.1}, medium=0.12}
gDk = pile{{"raw umber",3},{"bone black",1.3},{"yellow ochre",0.4},{"Antwerp blue",0.25},{"lead white",0.2}, medium=0.12}
gCool = pile{{"raw umber",2},{"bone black",0.9},{"cobalt blue",0.5},{"lead white",0.9},{"Indian red",0.15}, medium=0.12}
local ang = function(x,y) return -1.1 + 0.9*math.sin(x/28 + y/37) end
work(newGrove, {hand="body", tool="filbert 10", pile=gDk, coverage=2.4, fill=true, edge="soft", angle=ang, load=0.85, pressure={0.5,0.85}})
-- light side (toward the glow, left/upper-left of each mass)
local lit = newGrove * mask(function(x,y) local g = gdens(x-14, y-10); local h = gdens(x+10, y+8); return clamp((h - g)*0.9, 0, 1) end):blur(6)
work(lit, {hand="body", tool="filbert 8", pile=gOl, coverage=1.2, edge="lost", angle=ang, load=0.5, pressure={0.35,0.6}})
-- cool upper edges where the sky above is cool
local cool = newGrove * above(function(x) return 200 end) * mask(function(x,y) return clamp(1.2 - gdens(x,y)*0.6, 0, 1) end)
work(cool, {hand="scumble", pile=gCool, coverage=0.8, pressure={0.3,0.5}, load=0.4, angle=ang})

--@ chunk 195
local g = newGrove:grow(4)
blend(g, {angle=-1.1})
blend(g, {angle=0.3})
blend(g, {angle=-0.4})
blend(g, {angle=1.0})

--@ chunk 196
gDeep = pile{{"raw umber",2.5},{"bone black",1.6},{"Antwerp blue",0.35}, medium=0.1}
-- darker shadow masses within: under each big clump, right sides
local sh = newGrove:shrink(10) * mask(function(x,y) local g = gdens(x+12, y+10); local h = gdens(x-10, y-8); return clamp((h - g)*0.9 + 0.15, 0, 1) end):blur(8)
work(sh, {hand="body", tool="filbert 12", pile=gDeep, coverage=1.8, fill=true, edge="lost", angle=function(x,y) return -1.0 + 0.8*math.sin(x/30+y/40) end, load=0.8, pressure={0.5,0.85}})
-- olive light on the upper-left of masses, laid thick so it survives
local lit = newGrove:shrink(4) * mask(function(x,y) local g = gdens(x-16, y-12); local h = gdens(x+8, y+6); return clamp((h - g)*1.2 - 0.1, 0, 1) end):blur(5) * mask(function(x,y) return 1 - 0.6*smoothstep(800, 950, x) end)
work(lit, {hand="body", tool="filbert 8", pile=gOl, coverage=1.4, fill=false, edge="lost", angle=function(x,y) return -0.8 + 0.6*math.sin(x/20+y/30) end, load=0.7, pressure={0.4,0.7}})

--@ chunk 197
local g = newGrove:grow(2)
blend(g, {angle=-0.9})
blend(g, {angle=0.4})

--@ chunk 198
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.6}
  rb:load(gDeep, 0.85)
  rb:stroke(pts, {pressure=pr or {0.9,0.3}, ramps={0.05,0.4}, shake=0.7})
end
-- elm trunk, rising through the gap into the mass, with a fork
limb({{758,420},{757,395},{755,372},{752,350},{748,330}}, 11, {1,0.8})
limb({{752,352},{738,338},{724,326},{712,318}}, 5, {0.8,0.3})
limb({{754,348},{770,332},{786,322}}, 5, {0.8,0.3})
-- left tree trunk
limb({{647,420},{646,400},{644,380},{640,360}}, 7, {1,0.6})
limb({{644,378},{632,365},{622,356}}, 3.5, {0.7,0.2})
-- right tree: a trunk seen in its small gap
limb({{948,420},{947,398},{945,380},{942,362}}, 8, {1,0.6})
-- a thin sapling in the sky gap
limb({{690,420},{692,395},{696,370},{700,350}}, 3, {0.7,0.2})

--@ chunk 199
print(wait(10*24*60)); print(drying(760,200), drying(650,380), drying(900,420))

--@ chunk 200
groveF = newGrove
skyAll = above(function(x) return 392 end) - groveF:shrink(3)
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return skyAll * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(-20, 95), {hand="broad", pile=skyT, coverage=2.4, fill=true, angle=hz, clip=skyAll})
work(band(80, 175), {hand="broad", pile=skyU, coverage=2.4, fill=true, angle=hz, clip=skyAll})
work(band(160, 255), {hand="broad", pile=skyR, coverage=2.4, fill=true, angle=hz, clip=skyAll})
work(band(240, 330), {hand="broad", pile=skyG, coverage=2.4, fill=true, angle=hz, clip=skyAll})
work(band(315, 420) * mask(function(x,y) return x < 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyL, coverage=2.4, fill=true, angle=hz, clip=skyAll})
work(band(315, 420) * mask(function(x,y) return x >= 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyG, coverage=2.4, fill=true, angle=hz, clip=skyAll})
blend(skyAll, {angle=0.02})
blend(skyAll, {angle=-0.03})

--@ chunk 201
for _, p in ipairs({{833,183,9},{768,250,9},{876,268,14}}) do
  local m = ellipse(p[1], p[2], p[3]+5, p[3]+4)
  work(m, {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=-1.0, load=0.9, clip=m:grow(3)})
end
-- the edge: lose into the wet sky
local edge = groveF:grow(6) - groveF:shrink(8)
blend(edge * above(function(x) return 392 end), {angle=-0.9})
blend(edge * above(function(x) return 392 end), {angle=0.6})
blend((ellipse(833,183,20,18)+ellipse(768,250,20,18)+ellipse(876,268,26,22)):soften(6), {angle=-1})

--@ chunk 202
local ring = (groveF:grow(10) - groveF:shrink(10)) * above(function(x) return 393 end)
local outer = ring - groveF
local inner = ring * groveF
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(m, a, b) return m * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(outer, 80, 175), {hand="body", tool="filbert 7", pile=skyU, coverage=2.6, fill=true, angle=hz, clip=outer, load=0.9})
work(band(outer, 160, 255), {hand="body", tool="filbert 7", pile=skyR, coverage=2.6, fill=true, angle=hz, clip=outer, load=0.9})
work(band(outer, 240, 330), {hand="body", tool="filbert 7", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=outer, load=0.9})
work(band(outer, 315, 420) * mask(function(x,y) return x < 680 and 1 or 0 end), {hand="body", tool="filbert 7", pile=skyL, coverage=2.6, fill=true, angle=hz, clip=outer, load=0.9})
work(band(outer, 315, 420) * mask(function(x,y) return x >= 680 and 1 or 0 end), {hand="body", tool="filbert 7", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=outer, load=0.9})
work(inner, {hand="body", tool="filbert 7", pile=gDk, coverage=2.6, fill=true, angle=function(x,y) return -1.0+0.8*math.sin(x/20+y/30) end, clip=inner, load=0.9})
-- holes again
for _, p in ipairs({{833,183,16},{768,250,16},{876,268,22}}) do
  local m = ellipse(p[1], p[2], p[3], p[3]*0.9)
  work(m, {hand="body", tool="filbert 7", pile=gDk, coverage=2.6, fill=true, angle=-1.0, load=0.9, clip=m})
end

--@ chunk 203
local ang = function(x,y) return -1.1 + 0.9*math.sin(x/28 + y/37) end
work(groveF:shrink(1), {hand="body", tool="filbert 10", pile=gDk, coverage=2.6, fill=true, angle=ang, clip=groveF:grow(1), load=0.9, pressure={0.5,0.85}})
-- lose the edge outward with small strokes of the grove colour into wet sky
lose(groveF * above(function(x) return 392 end), {pile=gDk, tool="filbert 4", reach={6,5}, load=0.15, every=1.4, pressure={0.35,0.02}})

--@ chunk 204
local test = (groveF:grow(8) - groveF:shrink(6)) * rect(560, 250, 120, 120)
blend(test, {angle=-0.8, pressure={0.15,0.3}, coverage=1})

--@ chunk 205
local test = (groveF:grow(8) - groveF:shrink(6)) * rect(560, 250, 120, 120)
local insideT = test * groveF:grow(1)
work(insideT, {hand="body", tool="filbert 6", pile=gDk, coverage=2.6, fill=true, angle=-0.8, clip=insideT, load=0.9})
local outT = test - groveF:grow(1)
work(outT * above(function(x) return 330 end), {hand="body", tool="filbert 6", pile=skyG, coverage=2.6, fill=true, angle=0, clip=outT, load=0.9})
work(outT * below(function(x) return 330 end), {hand="body", tool="filbert 6", pile=skyL, coverage=2.6, fill=true, angle=0, clip=outT, load=0.9})
gMidHi = pile{{"raw umber",2},{"bone black",0.6},{"yellow ochre",0.3},{"lead white",2.2},{"cobalt blue",0.3},{"Indian red",0.15}, medium=0.12}
gMidLo = pile{{"raw umber",2},{"bone black",0.5},{"yellow ochre",0.7},{"lead white",2.2},{"lemon chrome",0.2}, medium=0.12}
local rim = (groveF:grow(3) - groveF:shrink(4)) * above(function(x) return 392 end)
local tool = {kind="filbert", width=5, stiffness=0.5, splay=0.3, ragged=0.4}
stipple(rim * above(function(x) return 240 end), {pile=gMidHi, tool=tool, coverage=1.3, pressure={0.3,0.6}, drag={3,-1.2}, twist=0.5})
stipple(rim * below(function(x) return 230 end), {pile=gMidLo, tool=tool, coverage=1.3, pressure={0.3,0.6}, drag={3,-1.2}, twist=0.5})

--@ chunk 206
local tool = {kind="filbert", width=6, stiffness=0.5, splay=0.3, ragged=0.4}
bushTop = outline{{690,396},{694,386},{702,380},{712,384},{720,378},{732,382},{742,376},{752,382},{766,378},{778,384},{790,380},{804,386},{818,382},{832,386},{846,380},{858,386},{872,382},{886,388},{900,384},{916,388},{932,382},{950,386},{970,380},{990,384},{1012,382},{1012,400},{690,400}, closed=true, char="soft", seed=1501, lobe=5, amount=1.6}
local bt = bushTop:mask() * mask(function(x,y) return x > 690 and 1 or 0 end)
work(bt, {hand="body", tool="filbert 5", pile=gDk, coverage=2.6, fill=true, angle=function(x,y) return -1.3+0.6*math.sin(x/9) end, clip=bt, load=0.9})
stipple(bt:rim(6,3), {pile=gMidLo, tool=tool, coverage=0.9, pressure={0.3,0.55}, drag={3,-1.4}, twist=0.5})
-- distant, paler bushes seen through the big gap (atmosphere)
farBushes = pile{{"lead white",3},{"raw umber",1.2},{"cobalt blue",0.4},{"yellow ochre",0.5},{"Indian red",0.15}, medium=0.12}
local fbz = outline{{730,386},{736,372},{750,366},{770,370},{790,362},{810,368},{828,372},{836,386}, closed=true, char="soft", seed=1502, lobe=4}:mask() - groveF
work(fbz, {hand="body", tool="filbert 4", pile=farBushes, coverage=2, fill=true, angle=-1.4, clip=fbz, load=0.6})

--@ chunk 207
blend(rect(725, 360, 120, 40):soften(4), {angle=0})
blend(rect(725, 360, 120, 40):soften(4), {angle=-1.4})

--@ chunk 208
print(wait(10*24*60)); print(drying(760,200), drying(780,380), drying(300,200))

--@ chunk 209
print(wait(6*24*60)); print(drying(760,200), drying(650,380))

--@ chunk 210
-- far tree line (left), as trees, not a hill
farTL2 = outline{{-10,404},{-10,386},{14,384},{30,378},{44,382},{58,376},{72,380},{88,372},{104,366},{118,360},{134,358},{150,362},{164,368},{180,376},{198,380},{216,377},{232,382},{250,384},{270,381},{290,385},{312,387},{336,384},{356,388},{384,389},{410,387},{440,390},{470,391},{500,392},{530,393},{556,394},{560,404}, closed=true, char="soft", seed=1601, lobe=5, amount=1.4}
local ft = farTL2:mask()
work(ft, {hand="body", tool="filbert 5", pile=farG, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -1.5+0.3*math.sin(x/9) end, load=0.85})
local fb = brush{kind="filbert", width=4, stiffness=0.4}
for i, x in ipairs(uneven(55, 0, 556, 0.7, 0.6, 61)) do
  local y = 400
  while y > 340 and ft:at(x, y) > 0.5 do y = y - 2 end
  fb:load(farG2, 0.45)
  fb:stroke({{x+rand(-1.5,1.5), 401},{x+rand(-1.5,1.5), y + rand(1,5)}}, {pressure={0.55,0.1}, ramps={0.1,0.6}})
end
blend(ft:grow(3):soften(3), {angle=-1.45})
blend(ft:grow(3):soften(3), {angle=0})

--@ chunk 211
local b = rect(-10, 380, 566, 22)
work(b, {hand="body", tool="filbert 6", pile=farG, coverage=2.6, fill=true, angle=0, clip=b, load=0.9})
local b2 = rect(-10, 393, 566, 9)
work(b2, {hand="body", tool="filbert 4", pile=farG2, coverage=2.4, fill=true, angle=0, clip=b2, load=0.8})
blend(rect(-10, 345, 570, 60):soften(4), {angle=-1.45})
blend(rect(-10, 345, 570, 60):soften(4), {angle=0})

--@ chunk 212
profT = {{-10,384},{14,382},{30,376},{44,380},{58,374},{72,378},{88,370},{104,364},{118,358},{134,356},{150,360},{164,366},{180,374},{198,378},{216,375},{232,380},{250,382},{270,379},{290,383},{312,385},{336,382},{356,386},{384,387},{410,385},{440,388},{470,389},{500,390},{530,391},{560,392}}
local skyFix = rect(-10, 330, 572, 70) * above(profT)
work(skyFix, {hand="body", tool="filbert 8", pile=skyL, coverage=3, fill=true, angle=0, clip=skyFix, load=1})
local treeFix = rect(-10, 340, 572, 62) * below(profT)
work(treeFix, {hand="body", tool="filbert 5", pile=farG, coverage=2.6, fill=true, angle=function(x,y) return -1.5+0.3*math.sin(x/9) end, clip=treeFix, load=0.9})

--@ chunk 213
local function profY(x)
  for i=1,#profT-1 do
    if x >= profT[i][1] and x <= profT[i+1][1] then
      local t = (x-profT[i][1])/(profT[i+1][1]-profT[i][1]); return lerp(profT[i][2], profT[i+1][2], t)
    end
  end
  return 392
end
local fb = brush{kind="filbert", width=5, stiffness=0.4}
for i, x in ipairs(uneven(80, 0, 556, 0.7, 0.6, 71)) do
  local y = profY(x)
  local h = 3 + 9*math.abs(nz(x*2.3, 11))
  fb:load(farG, 0.5)
  fb:stroke({{x+rand(-1,1), y+6},{x+rand(-1.5,1.5), y - h}}, {pressure={0.6,0.12}, ramps={0.1,0.6}})
end
-- a few darker clumps (nearer trees), and the farmhouse grove
for i, x in ipairs({40, 128, 145, 160, 300, 420, 515}) do
  fb:load(farG2, 0.55)
  local y = profY(x)
  for k=1,4 do
    local xx = x + rand(-8,8)
    fb:stroke({{xx, y+12},{xx+rand(-1,1), y - rand(0,8)}}, {pressure={0.6,0.15}, ramps={0.1,0.6}})
  end
end
blend(ribbon(profT, 10):soften(3), {angle=-1.45})

--@ chunk 214
field2 = pile{{"lead white",2.2},{"yellow ochre",0.6},{"raw umber",1.1},{"cobalt blue",0.25},{"Indian red",0.12}, medium=0.12}
pondNew = outline{{200,433},{240,423},{300,416},{380,412},{460,410},{540,410},{600,412},{650,418},{690,424},{700,432},{660,442},{600,452},{520,462},{430,467},{340,464},{270,455},{225,444}, closed=true, char="firm", seed=1701, amount=0.5}
local pm = pondNew:mask()
local fieldM = rect(-10, 400, 720, 70) * above(shoreAll) - pm - groveF
work(fieldM, {hand="body", tool="filbert 7", pile=field2, coverage=2.6, fill=true, angle=0.01, clip=fieldM, load=0.9})
work(pm, {hand="body", tool="flat 8", pile=pondGray, coverage=2.4, fill=true, angle=0, clip=pm, load=0.9})
glowW = pile{{"lead white",7},{"lemon chrome",1.0},{"yellow ochre",0.2}, medium=0.1}
local core = ellipse(360, 430, 170, 13):soften(10) * pm
work(core, {hand="body", tool="flat 8", pile=glowW, coverage=2.4, fill=true, angle=0, clip=pm, load=0.9})
blend(pm, {angle=0})

--@ chunk 215
local pm = pondNew:mask()
local fb = brush{kind="flat", width=10}
for i=1,14 do
  local y = 424 + i*2.2 + rand(-1,1)
  local x0 = 220 + rand(0,60) + math.abs(y-436)*3
  local x1 = 540 - rand(0,60) - math.abs(y-436)*3
  fb:load(glowW, 0.7)
  fb:stroke({{x0,y},{(x0+x1)/2, y+rand(-0.5,0.5)},{x1,y}}, {pressure={0.6,0.4}, ramps={0.2,0.3}, clip=pm})
end
-- reflected far trees along the far shore, soft and gray
local rf = pm * above(function(x) return 419 end)
work(rf, {hand="body", tool="flat 6", pile=farG, coverage=1.2, angle=0, clip=pm, load=0.4, pressure={0.3,0.5}})
-- reflection of the grove at the right end of the pond
local gr = pm * mask(function(x,y) return smoothstep(590, 680, x) end)
work(gr, {hand="body", tool="flat 6", pile=gDk, coverage=1.6, angle=1.5708, clip=pm, load=0.5, pressure={0.4,0.6}, length={10,25}})
blend(pm * mask(function(x,y) return smoothstep(560, 640, x) end), {angle=1.5708})
blend(pm, {angle=0, pressure={0.1,0.2}})

--@ chunk 216
shore2 = {{-10,430},{60,430},{120,430},{180,434},{215,442},{250,452},{300,462},{360,468},{430,471},{500,468},{560,462},{610,455},{660,447},{700,440},{740,452},{800,456},{900,457},{1012,457}}
meadow = below(shore2) - pondNew:mask():grow(1)
local hz = function(x,y) return 0.05*math.sin(x/120+y/60) - 0.015 end
local b1 = meadow * above(function(x) return 505 + 10*math.sin(x/110) end)
local b2 = meadow * below(function(x) return 495 + 10*math.sin(x/110) end) * above(function(x) return 575 + 18*math.sin(x/150+1) end)
local b3 = meadow * below(function(x) return 565 + 18*math.sin(x/150+1) end)
work(b1, {hand="body", tool="filbert 12", pile=meadNear, coverage=2.4, fill=true, angle=hz, clip=meadow, load=0.85})
work(b2, {hand="broad", pile=meadMid, coverage=2.4, fill=true, angle=hz, clip=meadow})
work(b3, {hand="broad", pile=meadDark, coverage=2.4, fill=true, angle=hz, clip=meadow})
blend(meadow:shrink(2), {angle=0.0})

--@ chunk 217
-- path, wiped out of the wet meadow
local rp = rag{width=34}
rp:wipe({{300,672},{340,620},{388,568},{428,528},{455,498},{470,478}}, {pressure={0.75,0.4}})
rp:refold()
rp:wipe({{340,672},{378,618},{418,565},{450,522},{468,492}}, {pressure={0.6,0.3}})
-- the gray box in the grove gap
local gapR = rect(722, 352, 128, 52) - groveF:shrink(2)
local bushLine = {{720,392},{735,386},{748,389},{760,383},{774,387},{790,382},{806,388},{820,384},{836,389},{852,386}}
local skyP = gapR * above(bushLine)
local bushP = gapR * below(bushLine)
work(skyP, {hand="body", tool="filbert 6", pile=skyG, coverage=2.8, fill=true, angle=0, clip=skyP, load=1})
work(bushP, {hand="body", tool="filbert 5", pile=gDk, coverage=2.8, fill=true, angle=-1.3, clip=bushP, load=1})
local fb = brush{kind="filbert", width=4, stiffness=0.4}
for i, x in ipairs(uneven(18, 724, 850, 0.6, 0.5, 81)) do
  fb:load(gDk, 0.5)
  fb:stroke({{x, 394},{x+rand(-1.5,1.5), 383 - rand(0,7)}}, {pressure={0.6,0.1}, ramps={0.1,0.6}})
end

--@ chunk 218
local lobe = outline{{712,352},{722,364},{738,370},{758,372},{776,368},{790,358},{794,346},{712,340}, closed=true, char="soft", seed=1801, lobe=5}:mask()
local gapR = (rect(712, 340, 145, 64) - groveF:shrink(2) - lobe) * mask(function(x,y) return 1 end)
local skyP = gapR * above(function(x) return 384 end)
work(skyP, {hand="body", tool="filbert 6", pile=skyG, coverage=3, fill=true, angle=0, clip=skyP, load=1})
local farP = gapR * below(function(x) return 383 + 2*math.sin(x/7) end) * above(function(x) return 394 end)
work(farP, {hand="body", tool="filbert 4", pile=farG, coverage=2.6, fill=true, angle=-1.45, clip=farP, load=0.9})
local nearP = gapR * below(function(x) return 392 end)
work(nearP, {hand="body", tool="filbert 5", pile=gDk, coverage=2.8, fill=true, angle=-0.3, clip=nearP, load=1})
work(lobe, {hand="body", tool="filbert 5", pile=gDk, coverage=2.8, fill=true, angle=-0.9, clip=lobe:grow(1), load=1})
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.6}
  rb:load(gDeep, 0.9)
  rb:stroke(pts, {pressure=pr or {1,0.8}, ramps={0.02,0.2}, shake=0.5})
end
limb({{760,400},{759,385},{757,372},{754,362}}, 10)
limb({{796,400},{797,388},{799,378}}, 4, {0.9,0.4})
blend((gapR + lobe):grow(3):soften(3) * above(function(x) return 395 end), {angle=0, pressure={0.1,0.2}})

--@ chunk 219
local box = rect(706, 334, 156, 72)
local rr = rag{width=30}
rr:dip(0.9)
rr:wipe(box, {pressure=0.85, passes=3, refold=0.15, angle=0})
rr:refold(); rr:dip(0.9)
rr:wipe(box, {pressure=0.85, passes=2, refold=0.15, angle=1.57})

--@ chunk 220
print(wait(12*24*60)); print(drying(780,370), drying(400,550), drying(400,440), drying(300,370))

--@ chunk 221
-- the repair: a dark mass over the whole box, leaving a drawn gap of glow
local box = rect(700, 330, 166, 82)
gap2 = outline{{728,384},{732,372},{742,364},{756,366},{768,360},{784,358},{800,362},{814,358},{828,364},{840,372},{846,384},{830,386},{800,385},{770,387},{745,386}, closed=true, char="soft", seed=1901, lobe=6, amount=1.0}
local gm = gap2:mask()
local darkPart = box - gm
work(darkPart, {hand="body", tool="filbert 7", pile=gDk, coverage=3, fill=true, angle=function(x,y) return -1.0+0.8*math.sin(x/15+y/20) end, clip=darkPart, load=1, pressure={0.6,0.9}})
work(gm * above(function(x) return 380 end), {hand="body", tool="filbert 5", pile=skyG, coverage=3, fill=true, angle=0, clip=gm, load=1})
work(gm * below(function(x) return 379 end), {hand="body", tool="filbert 4", pile=farG, coverage=3, fill=true, angle=-1.45, clip=gm, load=1})

--@ chunk 222
G1 = outline{{690,312},{700,302},{716,300},{732,306},{742,318},{744,334},{738,350},{726,362},{712,366},{700,360},{692,346},{688,330}, closed=true, char="soft", seed=2001, lobe=6, amount=0.8}
G2 = outline{{800,316},{812,304},{830,300},{850,302},{868,310},{878,324},{880,344},{876,366},{870,386},{840,388},{800,388},{770,388},{764,374},{772,360},{788,350},{796,334}, closed=true, char="soft", seed=2002, lobe=7, amount=0.8}
local gaps = (G1:mask() + G2:mask()) * above(function(x) return 387 end)
local region = rect(686, 296, 198, 104)
local dark = region - gaps
work(dark, {hand="body", tool="filbert 7", pile=gDk, coverage=3, fill=true, angle=function(x,y) return -1.0+0.8*math.sin(x/15+y/20) end, clip=dark, load=1, pressure={0.6,0.9}})
work(gaps * above(function(x) return 345 end), {hand="body", tool="filbert 6", pile=skyG, coverage=3, fill=true, angle=0, clip=gaps, load=1})
work(gaps * below(function(x) return 344 end), {hand="body", tool="filbert 6", pile=skyL, coverage=3, fill=true, angle=0, clip=gaps, load=1})
-- far trees seen low in the big gap
local ft = G2:mask() * below(function(x) return 378 + 2*math.sin(x/6) end)
work(ft, {hand="body", tool="filbert 4", pile=farG, coverage=2.6, fill=true, angle=-1.45, clip=ft, load=0.9})

--@ chunk 223
print(wait(12*24*60)); print(drying(830,340), drying(720,330), drying(760,380))

--@ chunk 224
print(wait(6*24*60)); print(drying(830,340), drying(720,330), drying(760,380))

--@ chunk 225
print(wait(10*24*60)); print(drying(760,380), drying(760,390))

--@ chunk 226
local gaps = (G1:mask():shrink(1) + G2:mask():shrink(1)) * above(function(x) return 376 end)
skyGthick = pile{{"lead white",6.5},{"yellow ochre",0.8},{"lemon chrome",0.4},{"Indian red",0.08}}
skyLthick = pile{{"lead white",7},{"lemon chrome",1.2},{"yellow ochre",0.2},{"cadmium yellow",0.1}}
for pass=1,2 do
  work(gaps * above(function(x) return 345 end), {hand="body", tool="filbert 6", pile=skyGthick, coverage=2.6, fill=true, angle=0, clip=gaps, load=1, pressure={0.6,0.9}})
  work(gaps * below(function(x) return 344 end), {hand="body", tool="filbert 6", pile=skyLthick, coverage=2.6, fill=true, angle=0, clip=gaps, load=1, pressure={0.6,0.9}})
end
blend(gaps:shrink(3), {angle=0})

--@ chunk 227
local mud = rect(735, 340, 40, 52) - G1:mask():grow(2) - G2:mask():grow(2)
mud = mud + (rect(780, 376, 100, 18) - G2:mask():shrink(4) * above(function(x) return 374 end))
work(mud, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.2, clip=mud, load=1, pressure={0.6,0.9}})
-- the bush line along the bottom of the big gap
local fb = brush{kind="filbert", width=4, stiffness=0.4}
for i, x in ipairs(uneven(22, 772, 880, 0.6, 0.5, 91)) do
  fb:load(gDk, 0.6)
  fb:stroke({{x, 392},{x+rand(-1.5,1.5), 378 - rand(0,8)}}, {pressure={0.65,0.1}, ramps={0.1,0.6}})
end

--@ chunk 228
print(wait(14*24*60)); print(drying(830,340), drying(720,330), drying(750,360))

--@ chunk 229
-- fill the left hole, leaving a couple of small flickers; reshape the big gap into a long low opening
local fillL = G1:mask():grow(3)
local fillTop = G2:mask():grow(3) * above(function(x) return 345 + 6*math.sin(x/11) end)
local fill = fillL + fillTop
work(fill, {hand="body", tool="filbert 7", pile=gDk, coverage=3, fill=true, angle=function(x,y) return -1.0+0.8*math.sin(x/15+y/20) end, clip=fill, load=1, pressure={0.6,0.9}})
-- long low opening between y ~348 and the bush line, from x 700 to 885
lowGap = outline{{700,384},{704,370},{716,362},{734,358},{756,352},{780,350},{806,346},{830,348},{852,346},{872,352},{884,364},{886,384},{860,386},{820,385},{780,386},{740,386}, closed=true, char="soft", seed=2101, lobe=7, amount=1.2}
local lg = lowGap:mask() * above(function(x) return 380 end)
work(lg, {hand="body", tool="filbert 6", pile=skyLthick, coverage=3, fill=true, angle=0, clip=lg, load=1, pressure={0.6,0.9}})

--@ chunk 230
-- far tree line seen through the opening, faint
local ftl = lowGap:mask() * below(function(x) return 372 + 2*math.sin(x/5) + 2*math.sin(x/13) end)
work(ftl, {hand="body", tool="filbert 4", pile=farG, coverage=2.6, fill=true, angle=-1.45, clip=ftl, load=0.8})
-- near bushes along the bottom with an uneven top
local fb = brush{kind="filbert", width=5, stiffness=0.4}
for i, x in ipairs(uneven(30, 700, 888, 0.6, 0.6, 101)) do
  fb:load(gDk, 0.6)
  local h = 4 + 10*math.abs(nz2(x*3, 7))
  fb:stroke({{x, 392},{x+rand(-1.5,1.5), 380 - h}}, {pressure={0.75,0.15}, ramps={0.1,0.6}})
end
-- trunks crossing the opening
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.6}
  rb:load(gDeep, 0.9)
  rb:stroke(pts, {pressure=pr or {1,0.85}, ramps={0.02,0.2}, shake=0.5})
end
limb({{760,394},{759,375},{757,360},{754,344}}, 10)
limb({{722,394},{723,378},{726,362},{730,350}}, 5)
limb({{818,394},{816,376},{814,360},{810,342}}, 6)
limb({{866,394},{867,378},{870,362},{874,346}}, 7)
limb({{842,392},{843,378},{845,366}}, 3, {0.8,0.3})
-- soften the top edge of the opening: dark touches dragged down from the canopy
local tool = {kind="filbert", width=5, stiffness=0.5, splay=0.3, ragged=0.4}
local topEdge = (lowGap:mask():rim(8,3)) * above(function(x) return 372 end)
stipple(topEdge, {pile=gDk, tool=tool, coverage=1.1, pressure={0.4,0.7}, drag={4,1.4}, twist=0.5})

--@ chunk 231
print(wait(14*24*60)); print(drying(790,365), drying(720,330))

--@ chunk 232
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.5}
  rb:load(gDeep, 1)
  rb:stroke(pts, {pressure=pr or {1,0.9}, ramps={0.02,0.1}, shake=0.4})
  rb:load(gDeep, 1)
  rb:stroke(pts, {pressure=pr or {1,0.9}, ramps={0.02,0.1}, shake=0.4})
end
limb({{760,396},{759,375},{757,358},{754,340}}, 10)
limb({{722,396},{723,378},{726,362},{730,348}}, 5)
limb({{818,396},{816,376},{814,358},{810,340}}, 6)
limb({{866,396},{867,378},{870,360},{874,344}}, 7)
-- bush band at the foot of the opening, solid
local band = rect(700, 380, 190, 16) * lowGap:mask():grow(4)
work(band, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.3, clip=band, load=1})
local fb = brush{kind="filbert", width=5, stiffness=0.4}
for i, x in ipairs(uneven(26, 702, 886, 0.6, 0.6, 111)) do
  fb:load(gDk, 0.9)
  local h = 2 + 7*math.abs(nz2(x*3, 9))
  fb:stroke({{x, 388},{x+rand(-1.5,1.5), 380 - h}}, {pressure={0.8,0.2}, ramps={0.1,0.6}})
end

--@ chunk 233
local region = outline{{690,300},{720,292},{780,290},{840,290},{890,296},{896,330},{894,370},{892,398},{800,400},{700,400},{688,360}, closed=true, char="soft", seed=2201, lobe=10, amount=1.0}:mask()
for pass=1,2 do
  work(region, {hand="body", tool="filbert 8", pile=gDk, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -1.0+0.8*math.sin(x/15+y/20) end, load=1, pressure={0.6,0.9}})
end
-- a slightly cooler, lighter low band to suggest depth under the canopy (not a hole)
local under = outline{{705,388},{712,376},{740,370},{780,368},{830,366},{870,370},{885,388}, closed=true, char="soft", seed=2202, lobe=6}:mask()
work(under, {hand="scumble", pile=gCool, coverage=0.9, pressure={0.3,0.5}, load=0.45, angle=0})
blend(region:shrink(4), {angle=-1.0})

--@ chunk 234
print(drying(400,550), drying(300,500))
mgl = pile{{"raw umber",2},{"bone black",1.2},{"raw sienna",0.5},{"Antwerp blue",0.2}, medium=0.55, thinner=0.15}
local mm = meadow:shrink(1)
work(mm, {hand="glaze", tool={kind="filbert", width=24, stiffness=0.3}, pile=mgl, coverage=2.6, fill=true, clip=mm, angle=function(x,y) return 0.03*math.sin(x/100) end, pressure={0.5,0.85}, length={120,260},
  load_at=function(x,y) return 0.35 + 0.65*smoothstep(470,640,y) + 0.3*smoothstep(600,950,x)*(1-smoothstep(470,560,y)) end})
-- lights wiped back: near-shore strip, path
local rr = rag{width=46}
rr:wipe({{0,440},{100,442},{200,452},{290,470},{380,478},{470,480},{560,472},{640,460}}, {pressure=0.5})
rr:refold()
rr:wipe({{30,452},{160,460},{280,478}}, {pressure=0.35})
rr = rag{width=30}
rr:wipe({{298,672},{340,620},{386,568},{426,528},{452,500},{468,480}}, {pressure={0.8,0.45}})
rr:refold()
rr:wipe({{336,672},{376,620},{416,566},{448,524},{466,494}}, {pressure={0.65,0.35}})

--@ chunk 235
print(wait(5*24*60)); print(drying(400,550), drying(700,350))

--@ chunk 236
mNear2 = pile{{"yellow ochre",1.2},{"raw umber",1.6},{"lead white",0.9},{"bone black",0.35},{"cobalt blue",0.15},{"raw sienna",0.2}, medium=0.12}
mMid2 = pile{{"raw umber",2.4},{"yellow ochre",0.9},{"bone black",0.7},{"raw sienna",0.5},{"lead white",0.2}, medium=0.12}
mDark2 = pile{{"raw umber",3},{"bone black",1.3},{"raw sienna",0.6},{"Antwerp blue",0.15}, medium=0.12}
local hz = function(x,y) return 0.05*math.sin(x/120+y/60) - 0.015 end
local mm = meadow
local b1 = mm * above(function(x) return 492 + 8*math.sin(x/110) end)
local b2 = mm * below(function(x) return 488 + 8*math.sin(x/110) end) * above(function(x) return 556 + 14*math.sin(x/150+1) end)
local b3 = mm * below(function(x) return 550 + 14*math.sin(x/150+1) end)
work(b1, {hand="body", tool="filbert 12", pile=mNear2, coverage=2.6, fill=true, angle=hz, clip=mm, load=0.9})
work(b2, {hand="broad", pile=mMid2, coverage=2.6, fill=true, angle=hz, clip=mm, load=0.9})
work(b3, {hand="broad", pile=mDark2, coverage=2.6, fill=true, angle=hz, clip=mm, load=0.9})
-- blend only across the band junctions, softly
blend((mm * below(function(x) return 470 end) * above(function(x) return 510 end)), {angle=0, pressure={0.15,0.3}})
blend((mm * below(function(x) return 530 end) * above(function(x) return 580 end)), {angle=0, pressure={0.15,0.3}})

--@ chunk 237
local rr = rag{width=30}
rr:wipe({{300,672},{338,622},{384,570},{424,530},{450,502},{466,482}}, {pressure={0.8,0.45}})
rr:refold()
rr:wipe({{338,672},{374,622},{414,568},{446,526},{464,496}}, {pressure={0.65,0.35}})
rr:refold()
-- path painted in: worn earth, warm light, thinning into the distance
pathP = pile{{"yellow ochre",1.2},{"raw umber",1.2},{"lead white",1.3},{"raw sienna",0.4},{"Indian red",0.1}, medium=0.12}
local pb = brush{kind="filbert", width=9, stiffness=0.5}
local paths = {
  {{305,668},{340,622},{384,572},{424,532},{452,503},{468,484}},
  {{332,668},{368,622},{410,570},{442,528},{462,498}},
}
for k=1,2 do
  for i, pth in ipairs(paths) do
    pb:load(pathP, 0.6)
    pb:stroke(pth, {pressure={0.55,0.2}, ramps={0.1,0.5}, shake=1.5, swell={1,0.8,0.6}})
  end
end

--@ chunk 238
pathShape = poly({{270,672},{330,612},{384,560},{424,522},{452,496},{466,482},{474,482},{462,500},{438,530},{404,572},{366,620},{336,672}}, true)
pathP2 = pile{{"raw umber",1.6},{"yellow ochre",1},{"lead white",0.6},{"raw sienna",0.3},{"bone black",0.2}, medium=0.12}
work(pathShape, {hand="body", tool="filbert 8", pile=pathP2, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -0.85 end, load=0.85})
blend(pathShape:grow(10):soften(10), {angle=-0.85})
blend(pathShape:grow(10):soften(10), {angle=0})

--@ chunk 239
pathNarrow = poly({{296,672},{344,616},{394,562},{432,524},{456,498},{468,484},{471,484},{460,502},{436,534},{402,576},{364,624},{334,672}}, true)
local edges = pathShape:grow(14) - pathNarrow
local hz = function(x,y) return 0.05*math.sin(x/120+y/60) - 0.015 end
work(edges * above(function(x) return 520 end), {hand="body", tool="filbert 7", pile=mMid2, coverage=2.6, fill=true, angle=hz, clip=edges, load=0.9})
work(edges * below(function(x) return 520 end), {hand="body", tool="filbert 7", pile=mDark2, coverage=2.6, fill=true, angle=hz, clip=edges, load=0.9})
blend((pathShape:grow(16)):soften(6), {angle=0, pressure={0.15,0.3}})

--@ chunk 240
print(wait(8*24*60)); print(drying(400,560), drying(700,350), drying(400,500))

--@ chunk 241
print(wait(8*24*60)); print(drying(400,560), drying(700,350), drying(380,600))

--@ chunk 242
pond3 = outline{{190,436},{214,428},{250,422},{300,418},{360,415},{420,413},{480,412},{540,412},{590,414},{630,418},{668,424},{700,431},{690,437},{664,442},{640,446},{606,452},{570,456},{520,461},{470,465},{430,466},{390,464},{350,462},{318,457},{290,452},{262,448},{236,444},{208,441}, closed=true, char="soft", seed=2301, lobe=12, amount=0.9}
local pm = pond3:mask()
wFar = pile{{"lead white",3.5},{"raw umber",0.9},{"cobalt blue",0.4},{"yellow ochre",0.4},{"Indian red",0.2}, medium=0.12}
wGlow = pile{{"lead white",7},{"lemon chrome",1.1},{"yellow ochre",0.3}, medium=0.1}
wNear = pile{{"lead white",4.5},{"cobalt blue",0.5},{"Indian red",0.3},{"yellow ochre",0.4},{"raw umber",0.3}, medium=0.12}
work(pm * above(function(x) return 419 end), {hand="body", tool="flat 6", pile=wFar, coverage=2.6, fill=true, angle=0, clip=pm, load=0.9})
work(pm * below(function(x) return 418 end) * above(function(x) return 446 end), {hand="body", tool="flat 8", pile=wGlow, coverage=2.6, fill=true, angle=0, clip=pm, load=0.9})
work(pm * below(function(x) return 445 end), {hand="body", tool="flat 8", pile=wNear, coverage=2.6, fill=true, angle=0, clip=pm, load=0.9})
-- grove reflection at the right end
local gref = pm * mask(function(x,y) return smoothstep(585, 640, x) end)
work(gref, {hand="body", tool="flat 6", pile=gDk, coverage=2.4, fill=true, angle=1.5708, clip=pm, load=0.8, length={8,20}})
blend(pm, {angle=0, pressure={0.15,0.3}})

--@ chunk 243
farShore = {{180,436},{230,426},{300,420},{380,417},{460,416},{540,416},{600,418},{650,422},{700,430}}
nearShore = {{180,436},{230,444},{290,452},{350,459},{420,463},{480,462},{540,458},{600,452},{650,445},{700,434}}
local zone = pond3:mask():grow(6) + pondNew:mask():grow(4)
local fieldPart = zone * above(farShore)
local meadPart = zone * below(nearShore)
work(fieldPart, {hand="body", tool="filbert 5", pile=field2, coverage=3, fill=true, angle=0, clip=fieldPart, load=1})
work(meadPart, {hand="body", tool="filbert 6", pile=mNear2, coverage=3, fill=true, angle=0, clip=meadPart, load=1})
-- reflection of grove: cleaner vertical drags, then horizontal soft
local water = zone * below(farShore) * above(nearShore)
local gref = water * mask(function(x,y) return x > 600 and 1 or 0 end)
work(gref, {hand="body", tool="flat 6", pile=gDk, coverage=2.6, fill=true, angle=1.5708, clip=water, load=0.9, length={8,16}})
blend(water * mask(function(x,y) return smoothstep(560,620,x) end), {angle=0, pressure={0.1,0.2}})

--@ chunk 244
local bankR = rect(560, 398, 160, 30) * above(farShore) * mask(function(x,y) return smoothstep(555, 590, x) end)
work(bankR, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-0.3, clip=bankR:grow(1), load=1})
-- and the reflection directly below it, darker at the far shore
local refl = rect(590, 414, 120, 20) * below(farShore) * above(nearShore)
work(refl, {hand="body", tool="flat 5", pile=gDk, coverage=2, fill=true, angle=1.5708, clip=refl, load=0.7, length={6,12}})

--@ chunk 245
print(wait(10*24*60)); print(drying(400,440), drying(650,410), drying(300,600))

--@ chunk 246
-- olive light dry-brushed on the glow-facing flanks of the grove masses (over dry paint)
oliveDry = pile{{"raw umber",1.4},{"yellow ochre",1.3},{"lead white",0.9},{"raw sienna",0.35},{"bone black",0.2},{"cobalt blue",0.1}}
local lit = groveF:shrink(5) * mask(function(x,y) local g = gdens(x-18, y-12); local h = gdens(x+8, y+6); return clamp((h - g)*1.2 - 0.05, 0, 1) end):blur(6) * mask(function(x,y) return 1 - 0.7*smoothstep(780, 950, x) end) * above(function(x) return 395 end)
work(lit, {hand="scumble", tool={kind="filbert", width=8, stiffness=0.7}, pile=oliveDry, coverage=0.9, pressure={0.2,0.4}, load=0.25, angle=function(x,y) return -0.8 + 0.6*math.sin(x/20+y/30) end})

--@ chunk 247
local rr = rag{width=40}
local zone = groveF:grow(4) * above(function(x) return 400 end)
rr:wipe(zone, {pressure=0.55, passes=2, refold=0.3, angle=-0.8})

--@ chunk 248
for k=1,3 do
  local rr = rag{width=36}
  rr:dip(0.9)
  local zone = groveF:grow(10) * above(function(x) return 400 end)
  rr:wipe(zone, {pressure=0.8, passes=2, refold=0.2, angle=(k%2==0) and -0.8 or 0.7})
end

--@ chunk 249
print(wait(6*24*60)); print(drying(700,200), drying(620,420))

--@ chunk 250
print(wait(6*24*60)); print(drying(700,200), drying(620,420), drying(640,410))

--@ chunk 251
farShore2 = {{180,436},{230,426},{300,420},{380,417},{460,416},{540,416},{580,417},{620,419},{660,423},{700,429},{740,436},{780,440}}
nearShore2 = {{180,436},{230,444},{290,452},{350,459},{420,463},{480,462},{540,458},{600,452},{650,446},{700,440},{740,438},{780,440}}
local leftEdge = outline{{566,440},{562,420},{566,404},{572,392},{580,382},{590,374},{600,370},{770,370},{770,440}, closed=true, char="soft", seed=2401, lobe=6, amount=1.2}:mask()
local darkA = leftEdge * above(farShore2)
work(darkA, {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=function(x,y) return -0.4+0.8*math.sin(x/12+y/17) end, clip=darkA, load=1, pressure={0.6,0.9}})
local water = below(farShore2) * above(nearShore2) * mask(function(x,y) return (x > 530 and x < 780) and 1 or 0 end)
work(water * mask(function(x,y) return x < 590 and 1 or 0 end), {hand="body", tool="flat 5", pile=wGlow, coverage=3, fill=true, angle=0, clip=water, load=1})
work(water * mask(function(x,y) return x >= 590 and 1 or 0 end), {hand="body", tool="flat 5", pile=gDk, coverage=3, fill=true, angle=1.5708, clip=water, load=1, length={5,12}})
local mead = below(nearShore2) * rect(530, 430, 250, 40)
work(mead, {hand="body", tool="filbert 6", pile=mNear2, coverage=3, fill=true, angle=0, clip=mead, load=1})
-- also the strip left of the grove edge above the water: far field + far trees
local leftStrip = rect(530, 370, 50, 70) * above(farShore2) - leftEdge
work(leftStrip * above(function(x) return 394 end), {hand="body", tool="filbert 4", pile=farG, coverage=3, fill=true, angle=-1.45, clip=leftStrip, load=1})
work(leftStrip * below(function(x) return 393 end) * above(function(x) return 404 end), {hand="body", tool="filbert 4", pile=farG2, coverage=3, fill=true, angle=0, clip=leftStrip, load=1})
work(leftStrip * below(function(x) return 403 end), {hand="body", tool="filbert 4", pile=field2, coverage=3, fill=true, angle=0, clip=leftStrip, load=1})

--@ chunk 252
print(wait(14*24*60)); print(drying(650,400), drying(560,430), drying(700,450))

--@ chunk 253
GL = outline{{560,419},{566,410},{572,398},{578,380},{584,360},{590,340},{600,320},{613,300},{700,296},{800,296},{900,296},{1012,296},{1012,458},{900,457},{800,453},{745,447},{712,441},{690,433},{660,426},{630,422},{600,420}, closed=true, char="soft", seed=2501, lobe=7, amount=0.9}
local gm = GL:mask()
local ang = function(x,y) return -0.6 + 0.9*math.sin(x/19 + y/27) end
work(gm, {hand="body", tool="filbert 9", pile=gDk, coverage=2.8, fill=true, edge="soft", angle=ang, load=0.95, pressure={0.55,0.9}})
-- under the canopy: a cooler, slightly lighter band where mist sits, and darker trunks
local mistBand = gm * (below(function(x) return 365 + 6*math.sin(x/40) end) * above(function(x) return 412 + 4*math.sin(x/30) end)):soften(10)
work(mistBand, {hand="scumble", pile=gCool, coverage=0.9, pressure={0.3,0.5}, load=0.45, angle=0})
-- olive flank toward the glow
local flank = gm * mask(function(x,y) return 1 - smoothstep(580, 640, x) end) * above(function(x) return 400 end)
work(flank, {hand="scumble", pile=gOl, coverage=0.9, pressure={0.3,0.5}, load=0.45, angle=-1.0})
-- trunks
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.5}
  rb:load(gDeep, 1)
  rb:stroke(pts, {pressure=pr or {1,0.8}, ramps={0.02,0.3}, shake=0.4})
end
limb({{760,446},{759,420},{757,390},{754,362},{750,340}}, 11)
limb({{646,424},{645,405},{643,385},{640,365}}, 7)
limb({{948,456},{947,430},{945,405},{942,380}}, 9)
limb({{860,452},{861,430},{863,410},{866,392}}, 5, {0.9,0.4})
limb({{700,436},{701,415},{703,395}}, 3.5, {0.8,0.3})

--@ chunk 254
local gm = GL:mask():shrink(3)
blend(gm, {angle=0})
blend(gm, {angle=0.08})
blend(gm, {angle=-0.08})

--@ chunk 255
fieldShape = poly({{-10,401},{200,401},{400,402},{560,403},{568,408},{562,418},{540,416},{460,416},{380,417},{300,420},{230,426},{180,436},{120,432},{60,431},{-10,431}})
work(fieldShape, {hand="body", tool="filbert 7", pile=field2, coverage=2.8, fill=true, angle=0.01, edge="soft", load=0.95})
-- a thin darker hedge line along the base of the far trees
local hb = brush{kind="filbert", width=5}
for x = -10, 560, 26 do
  hb:load(farG2, 0.6)
  hb:stroke({{x, 402+rand(-1,1)},{x+30, 402+rand(-1,1)}}, {pressure=0.5})
end
pondF = outline{{180,437},{230,427},{300,421},{380,418},{460,417},{540,417},{580,418},{620,421},{660,425},{695,432},{708,439},{680,442},{650,446},{600,452},{540,458},{480,463},{420,464},{350,460},{290,453},{230,445}, closed=true, char="firm", seed=2601, amount=0.5}
local pm = pondF:mask()
work(pm * above(function(x) return 421 end), {hand="body", tool="flat 6", pile=wFar, coverage=2.8, fill=true, angle=0, clip=pm, load=0.95})
work(pm * below(function(x) return 420 end) * above(function(x) return 447 end), {hand="body", tool="flat 8", pile=wGlow, coverage=2.8, fill=true, angle=0, clip=pm, load=0.95})
work(pm * below(function(x) return 446 end), {hand="body", tool="flat 8", pile=wNear, coverage=2.8, fill=true, angle=0, clip=pm, load=0.95})
local refl = pm * mask(function(x,y) return smoothstep(560, 610, x) end)
work(refl, {hand="body", tool="flat 5", pile=gDk, coverage=2.6, fill=true, angle=1.5708, clip=pm, load=0.95, length={5,12}})
blend(pm:shrink(1), {angle=0, pressure={0.15,0.3}})

--@ chunk 256
local box = poly({{524,360},{582,360},{578,380},{572,398},{568,404},{524,404}}) - GL:mask()
local skyP = box * above(function(x) return 391 end)
local treeP = box * below(function(x) return 390 end)
work(skyP, {hand="body", tool="filbert 5", pile=skyL, coverage=3, fill=true, angle=0, edge="soft", load=1})
work(treeP, {hand="body", tool="filbert 4", pile=farG, coverage=3, fill=true, angle=-1.45, edge="soft", load=1})
local fb = brush{kind="filbert", width=4, stiffness=0.4}
for i, x in ipairs(uneven(8, 525, 570, 0.6, 0.5, 131)) do
  fb:load(farG, 0.5)
  fb:stroke({{x, 398},{x+rand(-1,1), 388 - rand(0,6)}}, {pressure={0.55,0.1}, ramps={0.1,0.6}})
end
blend(ribbon({{-10,402},{570,403}}, 8), {angle=0, pressure={0.15,0.3}})
blend(box:grow(4), {angle=0, pressure={0.1,0.2}})

--@ chunk 257
meadowF = below({{-10,431},{60,431},{120,432},{180,437},{230,445},{290,453},{350,460},{420,464},{480,463},{540,458},{600,452},{650,446},{680,442},{708,440},{745,448},{800,454},{900,458},{1012,459}})
local hz = function(x,y) return 0.05*math.sin(x/120+y/60) - 0.015 end
local b1 = meadowF * above(function(x) return 492 + 8*math.sin(x/110) end)
local b2 = meadowF * below(function(x) return 486 + 8*math.sin(x/110) end) * above(function(x) return 556 + 14*math.sin(x/150+1) end)
local b3 = meadowF * below(function(x) return 548 + 14*math.sin(x/150+1) end)
work(b1, {hand="body", tool="filbert 12", pile=mNear2, coverage=2.8, fill=true, angle=hz, clip=meadowF, load=0.95})
work(b2, {hand="broad", pile=mMid2, coverage=2.8, fill=true, angle=hz, clip=meadowF, load=0.95})
work(b3, {hand="broad", pile=mDark2, coverage=2.8, fill=true, angle=hz, clip=meadowF, load=0.95})
-- the path in body colour, narrow, wet into wet
local pb = brush{kind="filbert", width=7, stiffness=0.5}
local paths = {
  {{298,668},{338,622},{384,572},{424,532},{450,503},{464,486}},
  {{318,668},{356,622},{400,570},{436,530},{458,500},{468,486}},
  {{280,668},{322,624},{370,576},{414,534},{444,506}},
}
for k=1,2 do
  for i, pth in ipairs(paths) do
    pb:load(pathP2, 0.7)
    pb:stroke(pth, {pressure={0.6,0.25}, ramps={0.1,0.5}, shake=1.2, swell={1.2,1,0.7}})
  end
end

--@ chunk 258
wholeG = (groveF + GL:mask()) 
local ang = function(x,y) return -0.9 + 0.9*math.sin(x/24 + y/33) end
work(wholeG:shrink(2), {hand="body", tool="filbert 10", pile=gDk, coverage=2.8, fill=true, angle=ang, clip=wholeG, load=0.95, pressure={0.55,0.9}})
blend(wholeG:shrink(3), {angle=-1.0})
blend(wholeG:shrink(3), {angle=0.3})

--@ chunk 259
local litF = function(x,y) local g = gdens(x-16, y-12); local h = gdens(x+8, y+6); return clamp((h - g)*1.1, 0, 1) end
local lit = wholeG:shrink(5) * mask(litF):blur(6) * mask(function(x,y) return 1 - 0.6*smoothstep(800, 960, x) end)
work(lit, {hand="body", tool="filbert 8", pile=gOl, coverage=1.3, edge="lost", angle=function(x,y) return -0.8 + 0.6*math.sin(x/20+y/30) end, load=0.6, pressure={0.35,0.65}})
local sh = wholeG:shrink(10) * mask(function(x,y) local g = gdens(x+12, y+10); local h = gdens(x-10, y-8); return clamp((h - g)*0.9 + 0.15, 0, 1) end):blur(8)
work(sh, {hand="body", tool="filbert 12", pile=gDeep, coverage=1.5, edge="lost", angle=function(x,y) return -1.0 + 0.8*math.sin(x/30+y/40) end, load=0.7, pressure={0.5,0.85}})
-- cool mist under the canopy near the water
local mist = wholeG * (below(function(x) return 380 + 6*math.sin(x/40) end) * above(function(x) return 425 end)):soften(12) * mask(function(x,y) return smoothstep(580, 640, x) end)
work(mist, {hand="scumble", pile=gCool, coverage=1.0, pressure={0.3,0.5}, load=0.45, angle=0})
-- olive on the left flank, lower tree
local flank = wholeG * mask(function(x,y) return 1 - smoothstep(590, 650, x) end) * below(function(x) return 290 end) * above(function(x) return 420 end)
work(flank, {hand="body", tool="filbert 7", pile=gOl, coverage=1.2, edge="lost", angle=-1.0, load=0.55, pressure={0.35,0.6}})

--@ chunk 260
blend(wholeG:shrink(3), {angle=-0.9})
blend(wholeG:shrink(3), {angle=0.4})

--@ chunk 261
blend(meadowF:shrink(2), {angle=0.0, pressure={0.2,0.4}})
blend(meadowF:shrink(2), {angle=-0.05, pressure={0.2,0.4}})
-- path: a worn earth band, wider near us, wet into wet
local pth = poly({{262,672},{318,616},{372,564},{414,526},{446,498},{462,484},{470,484},{458,502},{430,534},{394,578},{356,624},{326,672}}, true)
work(pth, {hand="body", tool="filbert 7", pile=pathP2, coverage=2.0, edge="soft", angle=-0.85, load=0.6, pressure={0.4,0.7}})
local pthLight = poly({{290,672},{336,620},{384,568},{424,530},{452,500},{465,486},{467,487},{452,506},{424,540},{388,582},{350,628},{318,672}}, true)
work(pthLight, {hand="body", tool="filbert 6", pile=pathP, coverage=1.4, edge="soft", angle=-0.85, load=0.5, pressure={0.3,0.6}})

--@ chunk 262
local pthZone = poly({{250,672},{312,612},{368,560},{412,522},{446,494},{464,480},{476,482},{462,504},{434,538},{398,582},{360,628},{338,672}}, true):grow(6)
blend(pthZone, {angle=-0.85, pressure={0.2,0.35}})
blend(pthZone * below(function(x) return 560 end), {angle=0, pressure={0.2,0.35}})

--@ chunk 263
print(wait(14*24*60)); print(drying(700,300), drying(400,600), drying(560,380), drying(400,440))

--@ chunk 264
print(wait(10*24*60)); print(drying(700,300), drying(400,600), drying(560,380), drying(400,440), drying(900,420))

--@ chunk 265
veilR = pile{{"lead white",6},{"Indian red",0.28},{"cobalt blue",0.32},{"yellow ochre",0.45},{"bone black",0.05}, medium=0.15, thinner=0.6}
veilG = pile{{"lead white",6.5},{"yellow ochre",0.7},{"lemon chrome",0.3},{"Indian red",0.07}, medium=0.15, thinner=0.6}
local rimOut = (wholeG:grow(12) - wholeG:shrink(1)) * above(function(x) return 395 end)
work(rimOut * above(function(x) return 200 end), {hand="body", tool={kind="filbert", width=8, stiffness=0.4}, pile=veilR, coverage=2, fill=true, clip=rimOut, angle=-0.7, load=0.6, pressure={0.4,0.6}})
work(rimOut * below(function(x) return 199 end), {hand="body", tool={kind="filbert", width=8, stiffness=0.4}, pile=veilG, coverage=2, fill=true, clip=rimOut, angle=-0.7, load=0.6, pressure={0.4,0.6}})

--@ chunk 266
ringO = (wholeG:grow(16) - wholeG:grow(1)) * above(function(x) return 396 end)
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(m, a, b) return m * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(ringO, -20, 95), {hand="body", tool="filbert 7", pile=skyT, coverage=3, fill=true, angle=hz, clip=ringO, load=1})
work(band(ringO, 80, 175), {hand="body", tool="filbert 7", pile=skyU, coverage=3, fill=true, angle=hz, clip=ringO, load=1})
work(band(ringO, 160, 255), {hand="body", tool="filbert 7", pile=skyR, coverage=3, fill=true, angle=hz, clip=ringO, load=1})
work(band(ringO, 240, 330), {hand="body", tool="filbert 7", pile=skyG, coverage=3, fill=true, angle=hz, clip=ringO, load=1})
work(band(ringO, 315, 420), {hand="body", tool="filbert 7", pile=skyL, coverage=3, fill=true, angle=hz, clip=ringO, load=1})
blend(ringO:shrink(1), {angle=0.02, pressure={0.15,0.3}})

--@ chunk 267
-- fill the holes
local holes = (wholeG:grow(6):shrink(6) - wholeG) * below(function(x) return 60 end)
work(holes:grow(3), {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.0, clip=holes:grow(4), load=1})
-- dark foliage touches across the edge, feathering out into the wet sky
local edgeZ = (wholeG:grow(12) - wholeG:shrink(6)) * above(function(x) return 396 end)
local tool = {kind="filbert", width=6, stiffness=0.5, splay=0.4, ragged=0.5}
stipple(edgeZ, {pile=gDk, tool=tool, coverage=function(x,y) return 0.25 + 1.6*wholeG:at(x,y) end, pressure={0.35,0.75}, drag={4,-1.3}, twist=0.6, cluster={0.5,10}, feather=0.6, dips={12,0.8,0.25}})

--@ chunk 268
local band = (wholeG:grow(7) - wholeG:shrink(7)) * above(function(x) return 396 end) * mask(function(x,y) return (x < 700) and 1 or 0 end)
blend(band, {angle=-0.6, pressure={0.2,0.35}})

--@ chunk 269
local L = mask(function(x,y) return (x < 705) and 1 or 0 end) * above(function(x) return 396 end)
local outB = (wholeG:grow(10) - wholeG:grow(1)) * L
local inB = (wholeG:grow(1) - wholeG:shrink(9)) * L
work(outB * above(function(x) return 330 end), {hand="body", tool="filbert 5", pile=skyG, coverage=3, fill=true, angle=-0.6, clip=outB, load=1})
work(outB * below(function(x) return 329 end), {hand="body", tool="filbert 5", pile=skyL, coverage=3, fill=true, angle=-0.6, clip=outB, load=1})
work(inB, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-0.6, clip=inB, load=1})
local n1 = lose(wholeG * L, {pile=skyG, tool="filbert 3", reach={4,6}, load=0.15, every=2.0, pressure={0.3,0.02}})
local n2 = lose(-wholeG * L * wholeG:grow(20), {pile=gDk, tool="filbert 3", reach={4,6}, load=0.15, every=2.0, pressure={0.3,0.02}})
print(n1, n2)

--@ chunk 270
-- repaint the flecked lines inside the dark
local lineX = rect(695, 190, 20, 210) * wholeG:shrink(3)
local lineY = rect(560, 386, 150, 18) * wholeG:shrink(3)
local ln = lineX + lineY
work(ln, {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=-1.2, clip=ln, load=1})
-- repaint the sky band where the dark flecks fell
local L = mask(function(x,y) return (x < 705) and 1 or 0 end) * above(function(x) return 396 end)
local sk = (wholeG:grow(24) - wholeG:grow(5)) * L
work(sk * above(function(x) return 330 end), {hand="body", tool="filbert 6", pile=skyG, coverage=3, fill=true, angle=0, clip=sk, load=1})
work(sk * below(function(x) return 329 end), {hand="body", tool="filbert 6", pile=skyL, coverage=3, fill=true, angle=0, clip=sk, load=1})
-- tangent field from the blurred grove
soft = wholeG:blur(10)
tangent = function(x,y)
  local gx = soft:at(x+3,y) - soft:at(x-3,y)
  local gy = soft:at(x,y+3) - soft:at(x,y-3)
  return math.atan(gy, gx) + math.pi/2
end
local edge = (wholeG:grow(7) - wholeG:shrink(7)) * L
blend(edge, {angle=tangent, pressure={0.15,0.3}})

--@ chunk 271
print(wait(16*24*60)); print(drying(700,300), drying(620,330), drying(590,380))

--@ chunk 272
print(drying(700,300), drying(620,330), drying(590,380))

--@ chunk 273
groveGl3 = pile{{"raw umber",2},{"bone black",1.4},{"Antwerp blue",0.25},{"raw sienna",0.4}, medium=0.6, thinner=0.15}
local g = wholeG:grow(7):soften(4)
work(g, {hand="glaze", tool={kind="filbert", width=16, stiffness=0.3}, pile=groveGl3, coverage=2.6, fill=true, clip=g, angle=-1.0, length={40,90}, pressure={0.55,0.85}})
blend(wholeG:shrink(2), {angle=-1.0, pressure={0.2,0.35}})

--@ chunk 274
wait(10*24*60); print(drying(700,300), drying(650,200), drying(300,200))

--@ chunk 275
-- the grove silhouette, final: drawn as an outline, a little smaller than the old one at the left so the sky can cover the halo
GS = outline{
 {568,420},{572,404},{580,388},{586,372},{592,354},{600,336},{612,318},{626,304},{646,294},{662,288},{672,272},{668,252},{668,232},
 {676,210},{676,188},{688,166},{700,146},{716,122},{736,104},{760,90},{786,84},{808,88},{828,96},{846,112},{858,132},{868,152},
 {878,172},{884,196},{882,220},{896,236},{920,228},{948,222},{976,220},{1012,216},{1012,470},{900,466},{780,460},{700,446},{640,430},{600,424},
 closed=true, char="soft", seed=2701, lobe=9, amount=1.1}
local gs = GS:mask()
print(gs:area(), wholeG:area())
for y = 60, 420, 20 do
  local s = ""
  for x = 540, 1000, 8 do
    local a = gs:at(x,y) > 0.5
    local b = wholeG:at(x,y) > 0.5
    s = s .. ((a and b) and "#" or (a and "+" or (b and "." or " ")))
  end
  print(string.format("%3d %s", y, s))
end

--@ chunk 276
local gs = GS:mask()
skyAll2 = above(function(x) return 394 end) - gs:shrink(2)
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return skyAll2 * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(-20, 95), {hand="broad", pile=skyT, coverage=2.6, fill=true, angle=hz, clip=skyAll2})
work(band(80, 175), {hand="broad", pile=skyU, coverage=2.6, fill=true, angle=hz, clip=skyAll2})
work(band(160, 255), {hand="broad", pile=skyR, coverage=2.6, fill=true, angle=hz, clip=skyAll2})
work(band(240, 330), {hand="broad", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=skyAll2})
work(band(315, 420) * mask(function(x,y) return x < 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyL, coverage=2.6, fill=true, angle=hz, clip=skyAll2})
work(band(315, 420) * mask(function(x,y) return x >= 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=skyAll2})
blend(skyAll2:shrink(3), {angle=0.02})
blend(skyAll2:shrink(3), {angle=-0.03})

--@ chunk 277
local gs = GS:mask()
local top = above(function(x) return 396 end)
local inB = (gs:grow(1) - gs:shrink(12)) * top
work(inB, {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=tangent, clip=inB, load=1})
soft2 = gs:blur(10)
tangent2 = function(x,y)
  local gx = soft2:at(x+3,y) - soft2:at(x-3,y)
  local gy = soft2:at(x,y+3) - soft2:at(x,y-3)
  return math.atan(gy, gx) + math.pi/2
end
blend((gs:grow(4) - gs:shrink(4)) * top, {angle=tangent2, pressure={0.12,0.25}})
local n = lose(gs, {pile=gDk, tool="filbert 3", reach={7,4}, load=0.15, every=1.6, pressure={0.35,0.02}, where=function(x,y) return y < 392 and x < 1000 and 1 or 0 end})
print(n)

--@ chunk 278
local gs = GS:mask()
local ang = function(x,y) return -0.9 + 0.9*math.sin(x/24 + y/33) end
work(gs:shrink(8), {hand="body", tool="filbert 10", pile=gDk, coverage=3, fill=true, angle=ang, clip=gs:shrink(6), load=1, pressure={0.55,0.9}})
-- modelling, wet into wet: olive light on glow side, deep shadow on far side
local litF = function(x,y) local g = gdens(x-16, y-12); local h = gdens(x+8, y+6); return clamp((h - g)*1.1, 0, 1) end
local lit = gs:shrink(8) * mask(litF):blur(6) * mask(function(x,y) return 1 - 0.6*smoothstep(800, 960, x) end)
work(lit, {hand="body", tool="filbert 8", pile=gOl, coverage=1.2, edge="lost", angle=function(x,y) return -0.8 + 0.6*math.sin(x/20+y/30) end, load=0.55, pressure={0.35,0.6}})
local sh = gs:shrink(12) * mask(function(x,y) local g = gdens(x+12, y+10); local h = gdens(x-10, y-8); return clamp((h - g)*0.9 + 0.15, 0, 1) end):blur(8)
work(sh, {hand="body", tool="filbert 12", pile=gDeep, coverage=1.4, edge="lost", angle=function(x,y) return -1.0 + 0.8*math.sin(x/30+y/40) end, load=0.7, pressure={0.5,0.85}})
local mist = gs * (below(function(x) return 380 + 6*math.sin(x/40) end) * above(function(x) return 428 end)):soften(12) * mask(function(x,y) return smoothstep(590, 650, x) end)
work(mist, {hand="scumble", pile=gCool, coverage=1.0, pressure={0.3,0.5}, load=0.45, angle=0})
blend(gs:shrink(4), {angle=-0.9, pressure={0.2,0.35}})

--@ chunk 279
farLineF = outline{{-10,404},{-10,388},{20,385},{40,380},{60,383},{80,376},{100,370},{118,364},{134,362},{150,365},{166,371},{186,378},{210,381},{236,379},{260,384},{290,386},{320,385},{350,388},{390,389},{430,390},{470,391},{510,392},{560,393},{575,404}, closed=true, char="soft", seed=2801, lobe=6, amount=1.3}
local fm = farLineF:mask() - GS:mask()
work(fm, {hand="body", tool="filbert 5", pile=farG, coverage=2.6, fill=true, edge="soft", angle=function(x,y) return -1.5+0.3*math.sin(x/9) end, load=0.85})
local fb = brush{kind="filbert", width=4, stiffness=0.4}
for i, x in ipairs(uneven(70, 0, 560, 0.7, 0.6, 141)) do
  local y = 400
  while y > 340 and farLineF:mask():at(x, y) > 0.5 do y = y - 2 end
  fb:load(farG, 0.45)
  fb:stroke({{x+rand(-1,1), y+8},{x+rand(-1.5,1.5), y - 2 - rand(0,7)}}, {pressure={0.55,0.1}, ramps={0.1,0.6}})
end
-- the farmhouse grove a touch darker
local grv = outline{{100,400},{104,378},{116,366},{132,360},{148,363},{162,372},{170,386},{172,400}, closed=true, char="soft", seed=2802, lobe=5}
work(grv:mask(), {hand="body", tool="filbert 4", pile=farG2, coverage=1.6, edge="soft", angle=-1.45, load=0.6})
blend(farLineF:mask():grow(4) - GS:mask(), {angle=-1.45, pressure={0.15,0.3}})
blend(farLineF:mask():grow(4) - GS:mask(), {angle=0, pressure={0.15,0.3}})

--@ chunk 280
local fieldS = poly({{-10,401},{200,401},{400,402},{575,403},{575,416},{540,416},{460,416},{380,417},{300,420},{230,426},{180,436},{120,432},{60,431},{-10,431}}) - GS:mask()
work(fieldS, {hand="body", tool="filbert 6", pile=field2, coverage=3, fill=true, angle=0.0, clip=fieldS, load=1})
local pm = pondF:mask() - GS:mask()
local topW = pm * above(function(x) return 424 end)
work(topW, {hand="body", tool="flat 6", pile=wFar, coverage=3, fill=true, angle=0, clip=topW, load=1})
local hb = brush{kind="filbert", width=4}
for x = -10, 570, 24 do
  hb:load(farG2, 0.5)
  hb:stroke({{x, 402+rand(-1,1)},{x+28, 402+rand(-1,1)}}, {pressure=0.45})
end
blend(pm, {angle=0, pressure={0.12,0.25}})

--@ chunk 281
local pm = pondF:mask() - GS:mask()
local core = pm * ellipse(380, 436, 210, 15):soften(8)
local fb = brush{kind="flat", width=8}
for i=1,16 do
  local y = 425 + i*1.9 + rand(-0.8,0.8)
  local half = 200 - math.abs(y-437)*9 + rand(-20,20)
  fb:load(wGlow, 0.8)
  fb:stroke({{380-half,y},{380, y+rand(-0.4,0.4)},{380+half,y}}, {pressure={0.65,0.45}, ramps={0.2,0.3}, clip=pm})
end
blend(pm * ellipse(380,437,230,20), {angle=0, pressure={0.1,0.2}})

--@ chunk 282
local pm = pondF:mask()
local refl = pm * mask(function(x,y) return smoothstep(570, 610, x) end)
work(refl, {hand="body", tool="flat 5", pile=gDk, coverage=3, fill=true, angle=1.5708, clip=pm, load=1, length={5,12}})
local fb = brush{kind="flat", width=5}
for i, x in ipairs(uneven(14, 560, 600, 0.5, 0.4, 151)) do
  fb:load(gDk, 0.4)
  fb:stroke({{x, 420},{x, 432+rand(0,10)}}, {pressure={0.5,0.1}, clip=pm})
end
-- the tree-reflections of the far line, faint vertical, along far shore
for i, x in ipairs(uneven(30, 200, 560, 0.6, 0.5, 152)) do
  fb:load(farG, 0.25)
  fb:stroke({{x, 418},{x+rand(-0.5,0.5), 422+rand(0,5)}}, {pressure={0.35,0.05}, clip=pm})
end

--@ chunk 283
wait(14*24*60); print(drying(700,300), drying(400,440), drying(300,390))

--@ chunk 284
local pm = pondF:mask() - GS:mask()
local strip = pm * above(function(x) return 433 end) * mask(function(x,y) return x < 565 and 1 or 0 end)
work(strip * above(function(x) return 424 end), {hand="body", tool="flat 6", pile=wFar, coverage=3, fill=true, angle=0, clip=strip, load=1})
work(strip * below(function(x) return 423 end), {hand="body", tool="flat 6", pile=wGlow, coverage=3, fill=true, angle=0, clip=strip, load=1})
blend(strip, {angle=0, pressure={0.12,0.25}})
-- the odd dark block at the grove's foot in the water, soften to reflection
local blk = pm * mask(function(x,y) return x >= 560 and 1 or 0 end)
blend(blk, {angle=1.5708, pressure={0.12,0.25}})

--@ chunk 285
local pm = pondF:mask()
local blk = pm * mask(function(x,y) return x >= 555 and 1 or 0 end) - GS:mask():shrink(1)
-- water first, gray-violet (shadowed by the grove)
wShade = pile{{"lead white",3},{"cobalt blue",0.45},{"Indian red",0.3},{"raw umber",0.7},{"yellow ochre",0.2}, medium=0.12}
work(blk, {hand="body", tool="flat 6", pile=wShade, coverage=3, fill=true, angle=0, clip=blk, load=1})
-- reflection strokes, down from the shore under the grove, length shrinking leftward
local fb = brush{kind="flat", width=5}
for i, x in ipairs(uneven(30, 572, 712, 0.5, 0.4, 161)) do
  local len = 8 + 30*smoothstep(570, 680, x)
  local y0 = 418
  fb:load(gDk, 0.5)
  fb:stroke({{x, y0},{x+rand(-0.5,0.5), y0+len}}, {pressure={0.6,0.2}, ramps={0.05,0.5}, clip=pm})
end
blend(blk, {angle=1.5708, pressure={0.12,0.25}})
blend(pm * mask(function(x,y) return smoothstep(520, 560, x) * (1 - smoothstep(580, 600, x)) end), {angle=0, pressure={0.12,0.25}})

--@ chunk 286
local bank = outline{{566,422},{570,414},{590,412},{620,414},{650,412},{680,415},{710,414},{730,418},{730,426},{700,424},{660,423},{620,424},{590,424}, closed=true, char="soft", seed=2901, lobe=4, amount=1.4}:mask()
work(bank, {hand="body", tool="filbert 4", pile=gDeep, coverage=3, fill=true, angle=0, edge="soft", load=1})
-- pale streak at 402
local streak = ellipse(585, 402, 22, 4)
work(streak, {hand="body", tool="filbert 4", pile=gDk, coverage=3, fill=true, angle=0, clip=streak:grow(2), load=1})
local fb = brush{kind="filbert", width=4, stiffness=0.4}
for i, x in ipairs(uneven(14, 568, 730, 0.6, 0.5, 171)) do
  fb:load(gDk, 0.6)
  fb:stroke({{x, 422},{x+rand(-1,1), 412 - rand(0,6)}}, {pressure={0.6,0.1}, ramps={0.1,0.6}})
end

--@ chunk 287
local z = rect(560, 395, 180, 35)
blend(z, {angle=0, pressure={0.2,0.35}})
blend(z, {angle=0, pressure={0.2,0.35}})

--@ chunk 288
wait(14*24*60); print(drying(650,410), drying(600,420))

--@ chunk 289
bankLine2 = {{556,416},{570,414},{590,417},{610,415},{630,418},{650,416},{670,419},{690,417},{710,420},{730,419},{750,422},{770,424}}
local groveFoot = outline{{556,416},{560,404},{568,396},{580,388},{600,384},{760,384},{770,424},{750,422},{730,419},{710,420},{690,417},{670,419},{650,416},{630,418},{610,415},{590,417},{570,414}, closed=true, char="soft", seed=3001, lobe=5, amount=1.2}:mask()
work(groveFoot, {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=function(x,y) return -0.4+0.8*math.sin(x/12+y/17) end, edge="soft", load=1, pressure={0.6,0.9}})
local mist = groveFoot * below(function(x) return 392 end) * above(function(x) return 412 end) * mask(function(x,y) return smoothstep(570,620,x) end)
work(mist, {hand="scumble", pile=gCool, coverage=0.8, pressure={0.3,0.5}, load=0.4, angle=0})
local water = (pondF:mask() * below(bankLine2) * mask(function(x,y) return x > 545 and 1 or 0 end))
work(water, {hand="body", tool="flat 6", pile=wShade, coverage=3, fill=true, angle=0, clip=water, load=1})
local fb = brush{kind="flat", width=5}
for i, x in ipairs(uneven(28, 562, 712, 0.5, 0.4, 181)) do
  local len = 6 + 26*smoothstep(565, 680, x)
  fb:load(gDk, 0.45)
  fb:stroke({{x, 417},{x+rand(-0.5,0.5), 417+len}}, {pressure={0.55,0.15}, ramps={0.05,0.5}, clip=water})
end

--@ chunk 290
local pm = pondF:mask()
local seamZ = pm * outline{{500,418},{600,418},{610,470},{500,470}, closed=true, char="soft", seed=3101, amount=0.3}:mask():soften(20)
blend(seamZ, {angle=0, pressure={0.2,0.35}})
blend(seamZ, {angle=0.1, pressure={0.2,0.35}})
-- soften blotchy mist inside the grove foot
local gf = outline{{566,412},{572,396},{590,388},{760,388},{760,414},{700,414},{620,414}, closed=true, char="soft", seed=3102, lobe=5}:mask()
blend(gf, {angle=0, pressure={0.2,0.35}})

--@ chunk 291
local pm = pondF:mask() * below(bankLine2)
local fb = brush{kind="flat", width=7}
local rows = 0
for y = 419, 466, 2.2 do
  local xs = {}
  -- find extents of the pond at this y
  local x0, x1 = nil, nil
  for x = 170, 720, 2 do
    if pm:at(x, y) > 0.5 then x0 = x0 or x; x1 = x end
  end
  if x0 then
    local x = x0 - 6
    while x < x1 do
      local seg = rand(40, 90)
      local xm = x + seg/2
      local pl
      local t = smoothstep(470, 590, xm)
      if y < 424 then pl = (t < 0.5) and wFar or wShade
      elseif y > 452 then pl = (t < 0.5) and wNear or wShade
      else pl = (t < 0.5) and wGlow or wShade end
      fb:load(pl, 0.7)
      fb:stroke({{x, y+rand(-0.4,0.4)},{x+seg, y+rand(-0.4,0.4)}}, {pressure={0.55,0.45}, ramps={0.15,0.25}, clip=pm})
      x = x + seg*0.8
      rows = rows + 1
    end
  end
end
print(rows)

--@ chunk 292
local pm = pondF:mask():shrink(1)
blend(pm, {angle=0, pressure={0.25,0.4}})
blend(pm, {angle=0.03, pressure={0.25,0.4}})
blend(pm, {angle=-0.03, pressure={0.25,0.4}})
-- grove reflections again, soft, after blending
local fb = brush{kind="flat", width=5}
for i, x in ipairs(uneven(28, 566, 712, 0.5, 0.4, 191)) do
  local len = 6 + 24*smoothstep(565, 680, x)
  fb:load(gDk, 0.35)
  fb:stroke({{x, 418},{x+rand(-0.5,0.5), 418+len}}, {pressure={0.5,0.1}, ramps={0.05,0.6}, clip=pm})
end

--@ chunk 293
wait(12*24*60); print(drying(400,440), drying(650,440))

--@ chunk 294
wGlowT = pile{{"lead white",7},{"lemon chrome",1.0},{"yellow ochre",0.3}}
wMidT = pile{{"lead white",6},{"lemon chrome",0.5},{"yellow ochre",0.4},{"cobalt blue",0.1},{"Indian red",0.1}}
local pm = pondF:mask():shrink(2)
local fb = brush{kind="flat", width=6, stiffness=0.6}
-- core glow: horizontal strokes, longer in the middle rows, centred left of middle
for i=1,22 do
  local y = 422 + i*1.6 + rand(-0.4,0.4)
  local d = math.abs(y - 440)/18
  local half = (230 - 150*d*d) * rand(0.8, 1.05)
  local cx = 380 + rand(-20,20)
  fb:load(wGlowT, 0.8)
  fb:stroke({{cx-half,y},{cx,y+rand(-0.3,0.3)},{cx+half,y}}, {pressure={0.55,0.4}, ramps={0.25,0.35}, clip=pm})
end
-- transition strokes on the right of the glow
for i=1,14 do
  local y = 424 + i*2.2
  local x0 = 520 + rand(0,40)
  fb:load(wMidT, 0.5)
  fb:stroke({{x0,y},{x0+rand(40,70),y}}, {pressure={0.45,0.15}, ramps={0.2,0.6}, clip=pm})
end

--@ chunk 295
local pm = pondF:mask():shrink(2)
local right = pm * mask(function(x,y) return smoothstep(540, 600, x) end)
work(right, {hand="body", tool="flat 6", pile=wShade, coverage=2.6, fill=true, angle=0, clip=pm, load=0.9})
local topEdge = pm * above(function(x) return 423 end)
work(topEdge, {hand="body", tool="flat 5", pile=wFar, coverage=2.4, fill=true, angle=0, clip=pm, load=0.8})
local botEdge = pm * below(function(x) return 455 end)
work(botEdge, {hand="body", tool="flat 5", pile=wNear, coverage=2.4, fill=true, angle=0, clip=pm, load=0.8})
blend(pm, {angle=0, pressure={0.25,0.4}})
blend(pm, {angle=0.02, pressure={0.25,0.4}})

--@ chunk 296
local pm = pondF:mask():shrink(2)
local fb = brush{kind="flat", width=5, stiffness=0.6}
for i=1,8 do
  local y = 430 + i*1.8
  local half = 110 - math.abs(i-4.5)*14
  local cx = 360 + rand(-15,15)
  fb:load(wGlowT, 0.6)
  fb:stroke({{cx-half,y},{cx+half,y}}, {pressure={0.5,0.35}, ramps={0.3,0.4}, clip=pm})
end
for i, x in ipairs(uneven(24, 572, 712, 0.5, 0.4, 201)) do
  local len = 6 + 22*smoothstep(565, 680, x)
  fb:load(gDk, 0.3)
  fb:stroke({{x, 418},{x+rand(-0.5,0.5), 418+len}}, {pressure={0.5,0.1}, ramps={0.05,0.6}, clip=pm})
end
blend(pm * mask(function(x,y) return smoothstep(560,600,x) end), {angle=1.5708, pressure={0.15,0.25}})

--@ chunk 297
print(drying(800,400), drying(650,400), drying(900,440), drying(500,550), drying(300,200))

--@ chunk 298
-- the base of the grove: darker, warmer; trunks; a few cool glimpses of mist between them
local base = outline{{556,418},{562,404},{580,388},{620,380},{700,374},{800,372},{900,374},{1012,372},{1012,472},{950,468},{860,462},{780,456},{740,448},{712,440},{690,430},{650,422},{600,420}, closed=true, char="soft", seed=3201, lobe=8, amount=1.2}:mask()
local ang = function(x,y) return -1.3 + 0.5*math.sin(x/14) end
work(base, {hand="body", tool="filbert 8", pile=gDk, coverage=2.8, fill=true, angle=ang, edge="soft", load=1, pressure={0.55,0.9}})
-- blend the top junction into the canopy (one tone family)
local junction = base * above(function(x) return 392 end) + (GS:mask() * below(function(x) return 360 end) * above(function(x) return 395 end))
blend(junction, {angle=-1.3, pressure={0.15,0.3}})
-- glimpses of mist/glow between trunks, small vertical slots
local slots = {{690,410,8,10},{735,404,6,12},{800,402,9,12},{845,406,6,10},{905,404,8,12},{975,404,7,10}}
for _, s in ipairs(slots) do
  local m = ellipse(s[1], s[2], s[3]/2, s[4]/2)
  work(m, {hand="body", tool="filbert 4", pile=gCool, coverage=2, fill=true, angle=-1.5, clip=m, load=0.6, pressure={0.4,0.6}})
end
local function limb(pts, w, pr)
  local rb = brush{kind="round", width=w, point=0.5}
  rb:load(gDeep, 1)
  rb:stroke(pts, {pressure=pr or {1,0.8}, ramps={0.02,0.3}, shake=0.4})
end
limb({{760,452},{759,425},{757,400},{754,375}}, 12)
limb({{646,426},{645,405},{643,385}}, 7)
limb({{948,466},{947,435},{945,405},{942,380}}, 10)
limb({{862,458},{863,430},{865,405}}, 5, {0.9,0.4})
limb({{712,440},{713,415},{715,395}}, 4, {0.8,0.3})

--@ chunk 299
for _, s in ipairs({{690,410},{735,404},{800,402},{845,406},{905,404},{975,404}}) do
  local m = ellipse(s[1], s[2], 9, 10)
  work(m, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.3, clip=m:grow(2), load=1})
end
local seam = (GS:mask() + above(function(x) return 500 end)) * below(function(x) return 345 + 8*math.sin(x/37) end) * above(function(x) return 380 + 8*math.sin(x/29) end) * mask(function(x,y) return x > 590 and 1 or 0 end)
blend(seam, {angle=-1.2, pressure={0.25,0.4}})
blend(seam, {angle=-0.5, pressure={0.25,0.4}})

--@ chunk 300
wait(14*24*60); print(drying(800,420), drying(700,300), drying(400,440))

--@ chunk 301
scOl = pile{{"raw umber",1.2},{"yellow ochre",1.3},{"lead white",1.2},{"raw sienna",0.3},{"bone black",0.15},{"cobalt blue",0.1}}
local test = ellipse(640, 330, 40, 35)
local tb = brush{kind="filbert", width=8, stiffness=0.8, splay=0.3}
work(test * GS:mask():shrink(4), {hand="scumble", tool=tb, pile=scOl, coverage=0.7, pressure={0.15,0.3}, load=0.2, angle=-0.9})

--@ chunk 302
local rr = rag{width=30}
rr:wipe(ellipse(640,330,45,40), {pressure=0.6, passes=2, refold=0.3})

--@ chunk 303
local m = outline{{600,330},{608,306},{630,294},{660,296},{676,312},{676,340},{660,362},{630,366},{606,352}, closed=true, char="soft", seed=3301, lobe=8}:mask()
work(m, {hand="body", tool="filbert 7", pile=gDk, coverage=2.4, fill=true, angle=-0.9, edge="soft", load=0.85})
blend(m:grow(6), {angle=-0.9, pressure={0.2,0.35}})
blend(m:grow(6), {angle=0.3, pressure={0.2,0.35}})

--@ chunk 304
wait(12*24*60); print(drying(640,330), drying(400,600))

--@ chunk 305
mgl2 = pile{{"raw umber",2},{"bone black",1.1},{"raw sienna",0.6},{"Antwerp blue",0.15}, medium=0.6, thinner=0.15}
local mm = meadowF:shrink(1)
work(mm, {hand="glaze", tool={kind="filbert", width=26, stiffness=0.3}, pile=mgl2, coverage=2.6, fill=true, clip=mm, angle=function(x,y) return 0.03*math.sin(x/100) end, pressure={0.5,0.85}, length={140,280},
  load_at=function(x,y) return 0.25 + 0.75*smoothstep(475,650,y) + 0.35*smoothstep(650,950,x)*(1-smoothstep(470,540,y)) end})
-- wipe lights: the strip along the near shore, a soft swell in the mid meadow, the path
local rr = rag{width=50}
rr:wipe({{0,445},{100,446},{200,458},{290,472},{380,480},{470,482},{560,474},{640,464},{700,458}}, {pressure=0.5})
rr:refold()
rr:wipe({{40,470},{150,476},{260,490}}, {pressure=0.35})
rr:refold()
rr:wipe({{560,488},{680,486},{800,494}}, {pressure=0.3})
rr = rag{width=26}
rr:wipe({{300,672},{340,622},{386,572},{426,532},{452,502},{466,484}}, {pressure={0.6,0.35}})
rr:refold()
rr:wipe({{320,672},{360,622},{404,570},{440,530},{460,498}}, {pressure={0.5,0.3}})

--@ chunk 306
wait(7*24*60); print(drying(400,600), drying(300,500))

--@ chunk 307
-- cover the stiff straight path with meadow colour
local oldPath = poly({{240,672},{312,606},{370,552},{414,514},{446,490},{462,478},{480,480},{466,502},{438,536},{400,582},{362,628},{350,672}}, true):grow(6)
local hz = function(x,y) return 0.05*math.sin(x/120+y/60) - 0.015 end
mDark3 = pile{{"raw umber",3},{"bone black",1.4},{"raw sienna",0.6},{"Antwerp blue",0.15},{"yellow ochre",0.3}, medium=0.12}
mMid3 = pile{{"raw umber",2.6},{"yellow ochre",0.8},{"bone black",0.9},{"raw sienna",0.5},{"lead white",0.15}, medium=0.12}
work(oldPath * below(function(x) return 545 end), {hand="body", tool="filbert 10", pile=mDark3, coverage=2.8, fill=true, angle=hz, edge="soft", load=0.9})
work(oldPath * above(function(x) return 550 end), {hand="body", tool="filbert 8", pile=mMid3, coverage=2.8, fill=true, angle=hz, edge="soft", load=0.9})
blend(oldPath, {angle=0, pressure={0.15,0.3}})

--@ chunk 308
local L = {{430,672},{418,640},{400,606},{392,576},{402,548},{424,522},{446,502},{460,489},{467,483}}
local R = {{500,672},{478,640},{452,606},{436,578},{438,554},{452,530},{466,510},{472,494},{470,483}}
local pts = {}
for i=1,#L do table.insert(pts, L[i]) end
for i=#R,1,-1 do table.insert(pts, R[i]) end
newPath = poly(pts, true)
work(newPath, {hand="body", tool="filbert 7", pile=pathP2, coverage=2.6, fill=true, edge="soft", angle=function(x,y) if y > 580 then return -1.9 else return -0.9 end end, load=0.8})
-- lighter worn centre, catching the sky
local cL = {{450,672},{434,640},{414,606},{406,578},{414,552},{434,526},{452,506},{463,490}}
local pb = brush{kind="filbert", width=8, stiffness=0.5}
for k=1,2 do
  pb:load(pathP, 0.6)
  pb:stroke({{462,672},{446,640},{426,606},{416,578},{422,552},{440,526},{456,506},{466,488}}, {pressure={0.55,0.15}, ramps={0.1,0.5}, shake=1.2})
end

--@ chunk 309
local z = newPath:grow(10):soften(6)
blend(z, {angle=0, pressure={0.2,0.35}})
blend(z, {angle=-1.2, pressure={0.2,0.35}})

--@ chunk 310
local rr = rag{width=40}
rr:dip(0.8)
rr:wipe(newPath:grow(8), {pressure=0.7, passes=2, refold=0.25, angle=-1.1})

--@ chunk 311
pathSub = pile{{"raw umber",2.4},{"yellow ochre",1.0},{"bone black",0.6},{"raw sienna",0.5},{"lead white",0.45}, medium=0.12}
oldPath = poly({{240,672},{312,606},{370,552},{414,514},{446,490},{462,478},{480,480},{466,502},{438,536},{400,582},{362,628},{350,672}}, true):grow(6)
local z = newPath:grow(4) + oldPath
work(z * below(function(x) return 560 end), {hand="body", tool="filbert 9", pile=mDark3, coverage=2.8, fill=true, angle=function(x,y) return 0.05*math.sin(x/120) end, edge="soft", load=1, pressure={0.6,0.9}})
work(z * above(function(x) return 565 end), {hand="body", tool="filbert 9", pile=mMid3, coverage=2.8, fill=true, angle=function(x,y) return 0.05*math.sin(x/120) end, edge="soft", load=1, pressure={0.6,0.9}})
work(newPath:shrink(3), {hand="body", tool="filbert 6", pile=pathSub, coverage=1.4, edge="lost", angle=function(x,y) if y > 580 then return -1.9 else return -0.9 end end, load=0.6, pressure={0.4,0.6}})
blend(newPath:grow(12), {angle=0, pressure={0.15,0.3}})

--@ chunk 312
wait(14*24*60); print(drying(420,600), drying(300,500), drying(800,520))

--@ chunk 313
local hz = function(x,y) return 0.06*math.sin(x/110+y/50) - 0.015 end
local mm = meadowF
local b1 = mm * above(function(x) return 494 + 8*math.sin(x/110) end)
local b2 = mm * below(function(x) return 488 + 8*math.sin(x/110) end) * above(function(x) return 560 + 14*math.sin(x/150+1) end)
local b3 = mm * below(function(x) return 552 + 14*math.sin(x/150+1) end)
work(b1, {hand="body", tool="filbert 12", pile=mNear2, coverage=2.8, fill=true, angle=hz, clip=mm, load=1})
work(b2, {hand="broad", pile=mMid3, coverage=2.8, fill=true, angle=hz, clip=mm, load=1})
work(b3, {hand="broad", pile=mDark3, coverage=2.8, fill=true, angle=hz, clip=mm, load=1})
-- path wet into wet: a lighter, warmer ribbon, broken, laid with a dry-ish filbert
local pb = brush{kind="filbert", width=10, stiffness=0.6}
local P = {{456,672},{440,640},{420,606},{410,578},{416,552},{434,526},{452,506},{463,490},{468,483}}
for k=1,3 do
  pb:load(pathSub, 0.5)
  local off = (k-2)*7
  local pts = {}
  for i,p in ipairs(P) do table.insert(pts, {p[1] + off*(1 - (i-1)/#P), p[2]}) end
  pb:stroke(pts, {pressure={0.6,0.2}, ramps={0.05,0.5}, shake=1.5})
end
pb:load(pathP, 0.4)
pb:stroke({{452,672},{436,640},{418,606},{410,580},{418,554},{436,528},{454,506},{464,490}}, {pressure={0.45,0.1}, ramps={0.05,0.6}, shake=1.5})

--@ chunk 314
local pb = brush{kind="filbert", width=14, stiffness=0.6}
for k=1,2 do
  pb:load(pathSub, 0.7)
  pb:stroke({{470,672},{452,640},{432,606},{420,578},{424,556}}, {pressure={0.75,0.3}, ramps={0.05,0.5}, shake=1.5})
end
local pb2 = brush{kind="filbert", width=7, stiffness=0.6}
for k=1,2 do
  pb2:load(pathSub, 0.6)
  pb2:stroke({{424,560},{434,532},{450,510},{462,494},{468,484}}, {pressure={0.6,0.25}, ramps={0.05,0.5}, shake=1.2})
end
pb2:load(pathP, 0.4)
pb2:stroke({{462,668},{446,638},{428,604},{420,578},{426,552},{442,526},{458,504}}, {pressure={0.45,0.1}, ramps={0.05,0.6}, shake=1.5})

--@ chunk 315
wait(12*24*60); print(drying(420,600), drying(300,500), drying(800,520), drying(440,620))

--@ chunk 316
mgl3 = pile{{"raw umber",2},{"bone black",1.2},{"raw sienna",0.5},{"Antwerp blue",0.15}, medium=0.6, thinner=0.15}
local mm = meadowF:shrink(1) * below(function(x) return 500 end)
work(mm:soften(30), {hand="glaze", tool={kind="filbert", width=26, stiffness=0.3}, pile=mgl3, coverage=2.4, fill=true, clip=meadowF, angle=function(x,y) return 0.03*math.sin(x/100) end, pressure={0.5,0.85}, length={140,280},
  load_at=function(x,y) return 0.2 + 0.8*smoothstep(510,660,y) end})
-- lift the track a little out of the wet glaze
local rr = rag{width=18}
rr:wipe({{466,672},{448,640},{428,606},{418,578},{424,554},{440,528},{456,506},{466,488}}, {pressure={0.6,0.3}})

--@ chunk 317
local band = meadowF * below(function(x) return 462 end) * above(function(x) return 530 end)
blend(band, {angle=0, pressure={0.25,0.4}})
blend(band, {angle=1.4, pressure={0.2,0.35}})
blend(band, {angle=0, pressure={0.25,0.4}})

--@ chunk 318
wait(12*24*60); print(drying(420,600), drying(300,500), drying(800,520))

--@ chunk 319
-- foreground grass: dry-brush strokes, nearly horizontal, some upward flicks; darker and warmer near us
grassW = pile{{"raw umber",2},{"raw sienna",1},{"yellow ochre",0.8},{"bone black",0.4}}
grassL = pile{{"yellow ochre",1.4},{"raw umber",1.2},{"lead white",0.6},{"raw sienna",0.3},{"bone black",0.2}}
grassD = pile{{"raw umber",3},{"bone black",1.2},{"Antwerp blue",0.15}}
local fg = meadowF * below(function(x) return 540 end) - newPath:grow(2)
work(fg, {hand="hatch", tool={kind="flat", width=5, stiffness=0.8}, pile=grassW, coverage=0.5, length={8,22}, angle=function(x,y) return -1.4 + 0.3*math.sin(x/30) end, pressure={0.25,0.5}, load=0.3, clump=0.6})
work(fg * below(function(x) return 590 end), {hand="hatch", tool={kind="flat", width=5, stiffness=0.8}, pile=grassD, coverage=0.6, length={10,28}, angle=function(x,y) return -1.4 + 0.35*math.sin(x/25) end, pressure={0.3,0.6}, load=0.35, clump=0.6})
local mid = meadowF * below(function(x) return 480 end) * above(function(x) return 545 end) - newPath:grow(2)
work(mid, {hand="scumble", tool={kind="filbert", width=8, stiffness=0.8}, pile=grassL, coverage=0.35, pressure={0.15,0.3}, load=0.2, angle=0})

--@ chunk 320
local z = meadowF * below(function(x) return 470 end)
for k=1,4 do
  local rr = rag{width=60}
  rr:dip(0.9)
  rr:wipe(z, {pressure=0.8, passes=2, refold=0.15, angle=(k%2==0) and 0.1 or -0.1})
end

--@ chunk 321
-- the figure: a woman in a dark skirt and red shawl, walking away toward the water
figDark = pile{{"raw umber",2},{"bone black",1.4},{"cobalt blue",0.3}, medium=0.05}
shawl = pile{{"Indian red",2},{"red earth",1},{"orange chrome",0.3},{"lead white",0.25}}
shawlLit = pile{{"red earth",1.5},{"orange chrome",0.6},{"lead white",0.8},{"Indian red",0.5}}
headP = pile{{"raw umber",2},{"bone black",0.6},{"raw sienna",0.6},{"lead white",0.3}}
local fx, fy = 452, 506  -- feet
-- skirt: a narrow bell
local skirt = poly({{fx-5.5,fy},{fx-4,fy-8},{fx-3,fy-15},{fx+3,fy-15},{fx+4,fy-8},{fx+5,fy}}, true)
work(skirt, {hand="detail", tool={kind="round", width=2.5, point=0.5}, pile=figDark, coverage=3, fill=true, angle=-1.57, clip=skirt, load=0.9})
-- shawl over shoulders and back
local sh = poly({{fx-4.5,fy-14},{fx-4.5,fy-21},{fx-2.5,fy-25},{fx+2.5,fy-25},{fx+4.5,fy-21},{fx+4,fy-14}}, true)
work(sh, {hand="detail", tool={kind="round", width=2.2, point=0.5}, pile=shawl, coverage=3, fill=true, angle=-1.57, clip=sh, load=0.9})
-- head
local hd = ellipse(fx-0.3, fy-28, 2.2, 2.7)
work(hd, {hand="detail", tool={kind="round", width=1.8, point=0.6}, pile=headP, coverage=3, fill=true, angle=-1.57, clip=hd, load=0.9})

--@ chunk 322
local fx, fy = 452, 506
local b = brush{kind="round", width=1.4, point=0.7}
b:load(shawlLit, 0.6)
b:stroke({{fx-4,fy-15},{fx-4.2,fy-20},{fx-2.5,fy-24}}, {pressure={0.5,0.3}})
b:load(headP, 0.5)
-- a little shadow to the right on the ground
local sb = brush{kind="filbert", width=3}
sb:load(figDark, 0.3)
sb:stroke({{fx+2,fy+0.5},{fx+12,fy+1.5}}, {pressure={0.35,0.1}})
-- a slight stride: one foot
b:load(figDark, 0.6)
b:stroke({{fx-3,fy},{fx-4.5,fy+1.5}}, {pressure=0.4})

--@ chunk 323
-- the farmhouse in the far grove, and its smoke
houseW = pile{{"lead white",2.5},{"yellow ochre",0.4},{"raw umber",0.6},{"cobalt blue",0.2},{"Indian red",0.15}}
roofP = pile{{"raw umber",1.5},{"Indian red",0.4},{"cobalt blue",0.3},{"lead white",1}}
local hx, hy = 150, 396
local wall = poly({{hx-8,hy},{hx-8,hy-6},{hx+7,hy-6},{hx+7,hy}})
work(wall, {hand="detail", tool={kind="round", width=1.6, point=0.5}, pile=houseW, coverage=3, fill=true, angle=0, clip=wall, load=0.8})
local roof = poly({{hx-9.5,hy-6},{hx-4,hy-11},{hx+5,hy-11},{hx+8.5,hy-6}})
work(roof, {hand="detail", tool={kind="round", width=1.6, point=0.5}, pile=roofP, coverage=3, fill=true, angle=0, clip=roof, load=0.8})
local b = brush{kind="round", width=1.2, point=0.6}
b:load(roofP, 0.6)
b:stroke({{hx+3,hy-11},{hx+3,hy-14}}, {pressure=0.6})
-- a warm lit window
local wb = brush{kind="round", width=1.3, point=0.6}
wb:load(pile{{"lead white",2},{"cadmium yellow",1},{"orange chrome",0.4}}, 0.5)
wb:touch(hx-3, hy-3, {pressure=0.4})

--@ chunk 324
houseVeil = pile{{"lead white",3},{"raw umber",1.0},{"cobalt blue",0.4},{"Indian red",0.2},{"yellow ochre",0.3}, medium=0.2, thinner=0.5}
local hm = rect(140, 383, 20, 15)
work(hm, {hand="detail", tool={kind="round", width=2, point=0.5}, pile=houseVeil, coverage=1.5, angle=0, clip=hm, load=0.5})
-- darker grove behind the house to set it off
local behind = outline{{126,384},{130,370},{142,364},{158,364},{170,372},{174,384}, closed=true, char="soft", seed=3401, lobe=4}:mask() - rect(141, 384, 18, 14)
work(behind, {hand="body", tool="filbert 4", pile=farG2, coverage=1.8, edge="soft", angle=-1.45, load=0.6})
-- smoke, a thin rising and drifting line
smokeP = pile{{"lead white",4},{"cobalt blue",0.25},{"raw umber",0.3},{"Indian red",0.1}, medium=0.3, thinner=0.5}
local sb = brush{kind="round", width=3, point=0.3}
sb:load(smokeP, 0.5)
sb:stroke({{153,382},{152,372},{155,360},{162,350},{174,343},{192,338},{214,336}}, {pressure={0.35,0.15}, ramps={0.1,0.6}, swell={0.8,1.2,1.6,1.4}})
blend(ribbon({{153,382},{152,372},{155,360},{162,350},{174,343},{192,338},{214,336}}, 8), {angle=-0.4, pressure={0.1,0.2}})

--@ chunk 325
local behind = outline{{120,388},{124,366},{140,358},{162,358},{178,368},{182,388}, closed=true, char="soft", seed=3402, lobe=5}:mask() - rect(141, 383, 18, 15)
blend(behind, {angle=-1.45, pressure={0.25,0.4}})
blend(behind, {angle=0, pressure={0.25,0.4}})
local rr = rag{width=12}
rr:wipe({{158,352},{168,346},{180,341},{196,337},{214,335}}, {pressure=0.6})
rr:refold()
rr:wipe({{153,378},{153,368},{156,358}}, {pressure=0.3})

--@ chunk 326
local rr = rag{width=16}
rr:dip(0.8)
rr:wipe({{150,372},{156,358},{166,348},{180,341},{198,337},{218,335}}, {pressure=0.8})
rr:refold(); rr:dip(0.8)
rr:wipe({{150,372},{156,358},{166,348},{180,341},{198,337},{218,335}}, {pressure=0.8})

--@ chunk 327
wait(10*24*60); print(drying(150,370), drying(452,495), drying(190,340))

--@ chunk 328
-- sky over the smoke streak and down to the tree tops around the house grove
local skyBit = outline{{140,358},{150,346},{170,334},{200,326},{235,328},{240,345},{220,352},{190,356},{176,366},{160,368}, closed=true, char="soft", seed=3501, lobe=6}:mask() * above(profT)
work(skyBit, {hand="body", tool="filbert 6", pile=skyL, coverage=3, fill=true, angle=0, edge="soft", load=1})
-- the house grove: a gentler, rounded mass, value between far trees and before
houseGrove = pile{{"lead white",2.2},{"raw umber",1.3},{"cobalt blue",0.45},{"yellow ochre",0.3},{"Indian red",0.2},{"bone black",0.08}, medium=0.12}
local hg = outline{{118,392},{120,378},{126,368},{136,362},{148,360},{160,362},{172,368},{180,378},{184,392}, closed=true, char="soft", seed=3502, lobe=5, amount=1.2}:mask() - rect(141, 384, 18, 14)
work(hg, {hand="body", tool="filbert 5", pile=houseGrove, coverage=3, fill=true, angle=-1.45, edge="soft", load=1})
blend(hg:grow(4) - rect(141, 384, 18, 14), {angle=-1.45, pressure={0.15,0.3}})

--@ chunk 329
wait(8*24*60); print(drying(150,375), drying(150,392))

--@ chunk 330
local hx, hy = 150, 398
houseW2 = pile{{"lead white",3},{"yellow ochre",0.5},{"raw umber",0.5},{"cobalt blue",0.15},{"Indian red",0.12}}
roofP2 = pile{{"raw umber",1.4},{"Indian red",0.3},{"cobalt blue",0.35},{"lead white",1.1}}
local wall = poly({{hx-7,hy},{hx-7,hy-6},{hx+6,hy-6},{hx+6,hy}})
work(wall, {hand="detail", tool={kind="round", width=1.5, point=0.5}, pile=houseW2, coverage=3, fill=true, angle=0, clip=wall, load=0.8})
local roof = poly({{hx-8.5,hy-6},{hx-3.5,hy-10.5},{hx+4.5,hy-10.5},{hx+7.5,hy-6}})
work(roof, {hand="detail", tool={kind="round", width=1.5, point=0.5}, pile=roofP2, coverage=3, fill=true, angle=0, clip=roof, load=0.8})
local b = brush{kind="round", width=1.1, point=0.6}
b:load(roofP2, 0.6)
b:stroke({{hx+2.5,hy-10.5},{hx+2.5,hy-13.5}}, {pressure=0.6})
local wb = brush{kind="round", width=1.2, point=0.6}
wb:load(pile{{"lead white",2},{"cadmium yellow",1},{"orange chrome",0.5}}, 0.5)
wb:touch(hx-3, hy-3, {pressure=0.35})
-- smoke: a thin veil, dabbed then softened
smokeV = pile{{"lead white",4},{"cobalt blue",0.3},{"raw umber",0.3},{"Indian red",0.12}, medium=0.3, thinner=0.7}
local sm = ribbon({{hx+2.5,hy-15},{hx+2,hy-24},{hx+5,hy-34},{hx+12,hy-42},{hx+24,hy-48},{hx+42,hy-52}}, {3,4,6,8,10,12})
stipple(sm, {pile=smokeV, width=4, coverage=1.2, pressure={0.2,0.4}, feather=0.5})
blend(sm:grow(3), {angle=-0.6, pressure={0.12,0.2}})

--@ chunk 331
print(drying(640,330), drying(720,200))
warmGl = pile{{"raw sienna",2},{"yellow ochre",1},{"lemon chrome",0.2}, medium=0.6, thinner=0.3}
local t = GS:mask():shrink(6) * ellipse(630, 330, 40, 40):soften(15)
work(t, {hand="glaze", tool={kind="filbert", width=12, stiffness=0.3}, pile=warmGl, coverage=1.5, clip=GS:mask():shrink(4), angle=-0.9, length={20,50}, pressure={0.4,0.6}})

--@ chunk 332
local t = GS:mask():shrink(4) * ellipse(630, 330, 50, 50)
blend(t, {angle=-0.9, pressure={0.3,0.5}})
blend(t, {angle=0.4, pressure={0.3,0.5}})
blend(t, {angle=-0.3, pressure={0.3,0.5}})

--@ chunk 333
for k=1,4 do
  local rr = rag{width=40}
  rr:dip(1)
  rr:wipe(ellipse(630,330,55,55), {pressure=0.9, passes=2, refold=0.1, angle=k*0.7})
end

--@ chunk 334
oliveVeil = pile{{"yellow ochre",1.4},{"raw umber",1.0},{"lead white",1.0},{"raw sienna",0.3},{"bone black",0.1}, medium=0.3, thinner=0.6}
local t = GS:mask():shrink(8) * outline{{690,200},{700,160},{730,125},{770,105},{790,120},{760,150},{730,185},{710,215}, closed=true, char="soft", seed=3601, lobe=10}:mask():blur(8)
work(t, {hand="glaze", tool={kind="filbert", width=14, stiffness=0.3}, pile=oliveVeil, coverage=1.5, clip=GS:mask():shrink(6), angle=-0.9, length={30,70}, pressure={0.35,0.55}})

--@ chunk 335
local t = GS:mask():shrink(8) * outline{{680,210},{690,160},{730,118},{775,98},{800,120},{765,155},{735,190},{712,225}, closed=true, char="soft", seed=3602, lobe=10}:mask():blur(6)
blend(t, {angle=-0.9, pressure={0.2,0.35}})

--@ chunk 336
local a = outline{{596,340},{606,312},{630,294},{660,290},{668,305},{640,318},{620,340},{606,362}, closed=true, char="soft", seed=3603, lobe=8}:mask()
local b = outline{{900,240},{930,226},{970,222},{1000,226},{990,240},{950,246},{915,256}, closed=true, char="soft", seed=3604, lobe=8}:mask()
local t = GS:mask():shrink(8) * (a + b):blur(6)
work(t, {hand="glaze", tool={kind="filbert", width=12, stiffness=0.3}, pile=oliveVeil, coverage=1.3, clip=GS:mask():shrink(6), angle=-0.9, length={25,60}, pressure={0.35,0.5}})
blend(t, {angle=-0.9, pressure={0.2,0.35}})

--@ chunk 337
wait(10*24*60); print(drying(300,600), drying(800,520), drying(452,495), drying(720,150))

--@ chunk 338
vig = pile{{"raw umber",2},{"bone black",1.2},{"raw sienna",0.4},{"Antwerp blue",0.15}, medium=0.65, thinner=0.25}
local mm = meadowF:shrink(1)
work(mm, {hand="glaze", tool={kind="filbert", width=26, stiffness=0.3}, pile=vig, coverage=2.2, fill=true, clip=mm, angle=function(x,y) return 0.04*math.sin(x/90) end, pressure={0.5,0.8}, length={140,280},
  load_at=function(x,y)
    local corner = smoothstep(200, 0, x) + smoothstep(780, 1000, x)
    local bottom = smoothstep(540, 667, y)
    local shade = smoothstep(640, 900, x) * smoothstep(455, 520, y)
    return clamp(0.15 + 0.5*bottom*(0.5+corner) + 0.5*shade, 0, 1)
  end})
local rr = rag{width=16}
rr:wipe({{466,672},{448,640},{428,606},{418,578},{424,554},{440,528},{454,508}}, {pressure={0.5,0.3}})

--@ chunk 339
twigP = pile{{"raw umber",2},{"bone black",1.2}, medium=0.2}
local gs = GS:mask()
local cnt = 0
local rb = brush{kind="rigger", width=1.6, point=1}
for _, a in ipairs(uneven(70, 0, 1, 0.6, 0.5, 211)) do
  local px, py = GS:at(a)
  if py < 300 and px < 1000 and px > 600 then
    local qx, qy = GS:at(math.min(1, a + 0.002))
    local tx, ty = qx-px, qy-py
    local len = math.sqrt(tx*tx+ty*ty) + 1e-6
    local nx, ny = -ty/len, tx/len
    if gs:at(px+nx*6, py+ny*6) > 0.5 then nx, ny = -nx, -ny end
    local ang = math.atan(ny, nx) + rand(-0.5, 0.5)
    -- bias upward
    if math.sin(ang) > 0.3 then ang = ang - 0.6 end
    local L = rand(8, 20)
    local sx, sy = px - nx*3, py - ny*3
    local mx, my = sx + math.cos(ang)*L*0.5 + rand(-2,2), sy + math.sin(ang)*L*0.5 + rand(-2,2)
    local ex, ey = sx + math.cos(ang)*L, sy + math.sin(ang)*L
    rb:load(twigP, 0.6)
    rb:stroke({{sx,sy},{mx,my},{ex,ey}}, {pressure={0.55,0.0}, ramps={0.05,0.8}})
    if math.random() < 0.6 then
      local ba = ang + rand(-0.8, 0.8)
      rb:stroke({{mx,my},{mx+math.cos(ba)*L*0.45, my+math.sin(ba)*L*0.45}}, {pressure={0.35,0.0}, ramps={0.05,0.8}})
    end
    cnt = cnt + 1
  end
end
print(cnt)

--@ chunk 340
local rim = (GS:mask():grow(22) - GS:mask():shrink(4)) * above(function(x) return 310 end)
for k=1,2 do
  local rr = rag{width=20}
  rr:dip(1)
  rr:wipe(rim, {pressure=0.8, passes=2, refold=0.15, angle=(k==1) and 0.5 or -0.6})
end

--@ chunk 341
wait(10*24*60); print(drying(700,150), drying(880,200))

--@ chunk 342
-- carve sky notches into the crown silhouette (opaque sky colour over dry paint), drawn shapes
skyUt = pile{{"lead white",5.5},{"cobalt blue",0.6},{"Indian red",0.35},{"yellow ochre",0.3},{"bone black",0.15}}
skyRt = pile{{"lead white",6},{"Indian red",0.3},{"cobalt blue",0.3},{"yellow ochre",0.5},{"bone black",0.05}}
skyGt = pile{{"lead white",6.5},{"yellow ochre",0.8},{"lemon chrome",0.4},{"Indian red",0.08}}
local notches = {
  {{742,88},{752,98},{758,112},{764,100},{770,84}},     -- top dip between lobes
  {{862,150},{850,160},{846,176},{858,182},{874,176}},  -- right side notch
  {{690,196},{702,204},{708,220},{696,226},{678,222}},  -- left side notch
  {{882,214},{890,226},{900,232},{904,220}},            -- where elm meets right tree
  {{660,262},{672,268},{676,282},{664,286},{652,280}},
}
for i, n in ipairs(notches) do
  local o = outline{n[1], n[2], n[3], n[4], n[5] or n[4], closed=true, char="soft", seed=3700+i, lobe=4, amount=1.0}
  local m = o:mask()
  local cy = n[3][2]
  local pl = (cy < 165) and skyUt or ((cy < 250) and skyRt or skyGt)
  for k=1,2 do
    work(m, {hand="detail", tool={kind="filbert", width=4, stiffness=0.6}, pile=pl, coverage=3, fill=true, angle=0, clip=m, load=0.9})
  end
end

--@ chunk 343
local spots = {{756,100},{858,168},{696,212},{893,224},{664,276}}
for k=1,4 do
  for i, s in ipairs(spots) do
    local rr = rag{width=26}
    rr:dip(1)
    rr:wipe(ellipse(s[1], s[2], 18, 16), {pressure=0.95, passes=2, refold=0.1, angle=k*0.8})
  end
end

--@ chunk 344
wait(12*24*60); print(drying(756,100), drying(858,168), drying(696,212))

--@ chunk 345
local spots = {{756,98,24,22},{858,168,24,22},{697,212,24,22},{884,222,22,18},{664,278,22,20}}
local all = nil
for _, s in ipairs(spots) do
  local m = outline{{s[1]-s[3],s[2]},{s[1]-s[3]*0.6,s[2]-s[4]*0.8},{s[1],s[2]-s[4]},{s[1]+s[3]*0.7,s[2]-s[4]*0.7},{s[1]+s[3],s[2]},{s[1]+s[3]*0.6,s[2]+s[4]*0.8},{s[1],s[2]+s[4]},{s[1]-s[3]*0.7,s[2]+s[4]*0.7}, closed=true, char="soft", seed=s[1], lobe=6}:mask()
  all = all and (all + m) or m
end
all = all * GS:mask():grow(2)
for k=1,2 do
  work(all, {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=-0.9, clip=all, load=1})
end
blend(all:shrink(3), {angle=-0.9, pressure={0.15,0.3}})

--@ chunk 346
local gs = GS:mask()
for _, y in ipairs({80, 90, 100, 120, 140, 160, 180, 200, 220, 240, 260, 280}) do
  local l, r = nil, nil
  for x = 560, 1000, 2 do
    if gs:at(x,y) > 0.5 then l = l or x; r = x end
  end
  print(y, l, r)
end

--@ chunk 347
local lobes = {
  outline{{846,118},{858,104},{876,100},{892,110},{900,128},{896,146},{884,160},{870,164},{860,150}, closed=true, char="soft", seed=3801, lobe=5, amount=1.3},
  outline{{790,90},{800,76},{816,70},{832,74},{842,86},{836,98},{816,96},{800,96}, closed=true, char="soft", seed=3802, lobe=4, amount=1.3},
  outline{{680,190},{668,182},{656,186},{650,198},{654,212},{666,220},{680,218}, closed=true, char="soft", seed=3803, lobe=4, amount=1.3},
  outline{{880,190},{892,182},{906,186},{912,198},{906,210},{892,212},{882,206}, closed=true, char="soft", seed=3804, lobe=4, amount=1.3},
  outline{{720,112},{714,100},{722,90},{736,88},{748,94},{746,106},{734,114}, closed=true, char="soft", seed=3805, lobe=4, amount=1.3},
}
local m = nil
for _, o in ipairs(lobes) do m = m and (m + o:mask()) or o:mask() end
lobeM = m
for k=1,2 do
  work(m, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-0.9, clip=m, load=1})
end
-- feather their outer edges with small dark touches into the dry sky
local tool = {kind="filbert", width=4, stiffness=0.5, splay=0.3, ragged=0.4}
stipple(m:grow(5) - m:shrink(2), {pile=gDk, tool=tool, coverage=0.7, pressure={0.3,0.6}, drag={3,-1.3}, twist=0.5, cluster={0.5,6}, feather=0.6})

--@ chunk 348
local lob = outline{{676,226},{664,232},{656,246},{654,262},{660,276},{656,290},{664,302},{680,304},{690,290},{690,250}, closed=true, char="soft", seed=3901, lobe=5, amount=1.3}:mask()
for k=1,2 do
  work(lob, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.2, clip=lob, load=1})
end
-- clean the ghost flecks in the sky with sky colour, clipped away from the dark
local sky = outline{{600,200},{650,160},{672,170},{648,190},{646,226},{644,262},{646,300},{630,320},{600,320}, closed=true, char="soft", seed=3902, lobe=6}:mask() - GS:mask():grow(2) - lobeM:grow(2) - lob:grow(2)
work(sky * above(function(x) return 250 end), {hand="body", tool="filbert 6", pile=skyRt, coverage=3, fill=true, angle=0, clip=sky, load=1})
work(sky * below(function(x) return 249 end), {hand="body", tool="filbert 6", pile=skyGt, coverage=3, fill=true, angle=0, clip=sky, load=1})

--@ chunk 349
local sky = outline{{600,200},{650,160},{672,170},{648,190},{646,226},{644,262},{646,300},{630,320},{600,320}, closed=true, char="soft", seed=3902, lobe=6}:mask() - GS:mask():grow(2) - lobeM:grow(2)
local lob = outline{{676,226},{664,232},{656,246},{654,262},{660,276},{656,290},{664,302},{680,304},{690,290},{690,250}, closed=true, char="soft", seed=3901, lobe=5, amount=1.3}:mask()
sky = sky - lob:grow(2)
skyMixT = pile{{"lead white",6.3},{"yellow ochre",0.65},{"lemon chrome",0.2},{"cobalt blue",0.12},{"Indian red",0.15}}
work(sky * above(function(x) return 250 end), {hand="body", tool="filbert 6", pile=skyMixT, coverage=3, fill=true, angle=0, clip=sky, load=1})
blend(sky:shrink(2), {angle=0, pressure={0.2,0.35}})

--@ chunk 350
local z = outline{{580,150},{620,130},{660,130},{672,170},{652,200},{646,240},{646,300},{620,330},{580,320},{560,240}, closed=true, char="soft", seed=4001, lobe=10}:mask() - GS:mask():grow(3) - lobeM:grow(3)
blend(z:soften(15), {angle=0.3, pressure={0.25,0.4}})
blend(z:soften(15), {angle=-0.3, pressure={0.25,0.4}})

--@ chunk 351
wait(10*24*60)
sig = pile{{"raw sienna",1},{"Indian red",1},{"raw umber",0.5}, medium=0.1}
local b = brush{kind="rigger", width=1.4, point=1}
b:load(sig, 0.6)
-- a small cursive mark, lower right
local x0, y0 = 900, 648
b:stroke({{x0,y0},{x0+2,y0-7},{x0+4,y0},{x0+6,y0-6},{x0+8,y0}}, {pressure={0.5,0.3}})
b:stroke({{x0+11,y0-5},{x0+13,y0},{x0+15,y0-4},{x0+18,y0-1},{x0+22,y0-3},{x0+26,y0}}, {pressure={0.5,0.25}})
b:stroke({{x0+30,y0-6},{x0+31,y0},{x0+35,y0-2}}, {pressure={0.45,0.2}})

--@ chunk 352
for _, n in ipairs({"skyT","skyU","skyR","skyG","skyL","gDk","gOl","gDeep","gCool"}) do print(n, tostring(_G[n])) end
for _, p in ipairs({{700,300},{800,200},{950,300},{800,420},{300,200}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 353
E2 = outline{{672,292},{668,270},{674,250},{668,228},{676,206},{684,186},{690,166},{702,146},{716,126},{734,108},{752,92},{772,80},{794,74},{816,78},{836,86},{852,100},{866,116},{880,134},{890,156},{896,178},{894,200},{886,222},{878,244},{872,262},{862,270},{850,264},{838,274},{822,284},{806,292},{790,300},{774,296},{758,302},{742,294},{726,302},{710,308},{694,304},
 closed=true, char="soft", seed=5101, lobe=5, amount=1.0}
R2 = outline{{878,272},{884,250},{894,232},{908,218},{926,210},{948,204},{972,203},{996,207},{1012,210},{1012,470},{870,462},{866,420},{868,380},{874,340},{880,300},
 closed=true, char="soft", seed=5102, lobe=5, amount=1.0}
L2 = outline{{556,420},{562,404},{572,390},{580,372},{588,352},{598,334},{610,316},{624,300},{640,288},{658,282},{676,284},{694,296},{706,312},{712,334},{716,356},{718,378},{720,400},{700,420},{640,424},
 closed=true, char="soft", seed=5103, lobe=5, amount=1.0}
U2 = outline{{560,420},{600,400},{660,390},{700,384},{720,378},{740,383},{760,375},{780,381},{800,376},{820,383},{840,374},{860,380},{880,372},{1012,370},{1012,472},{950,468},{860,462},{780,456},{740,448},{712,440},{690,430},{650,422},
 closed=true, char="soft", seed=5104, lobe=4, amount=1.0}
newG = E2:mask() + R2:mask() + L2:mask() + U2:mask()
oldG = (GS:mask() + lobeM):grow(6)
for y = 60, 460, 12 do
  local s = ""
  for x = 540, 1000, 7 do
    local a = newG:at(x,y) > 0.5
    local b = oldG:at(x,y) > 0.5
    s = s .. ((a and b) and "#" or (a and "+" or (b and "." or " ")))
  end
  print(string.format("%3d %s", y, s))
end

--@ chunk 354
local pl = (oldG - newG:shrink(5)) * above(function(x) return 380 end)
plasterB = pl
local function band(a, b) return pl * below(function(x) return a end) * above(function(x) return b end) end
local hz = function(x,y) return 0.05*math.sin(x/90+y/60) end
for k=1,2 do
  work(band(-20, 120), {hand="body", tool="filbert 10", pile=skyU, coverage=3, fill=true, angle=hz, clip=pl, load=1})
  work(band(120, 210), {hand="body", tool="filbert 10", pile=skyR, coverage=3, fill=true, angle=hz, clip=pl, load=1})
  work(band(210, 420), {hand="body", tool="filbert 10", pile=skyG, coverage=3, fill=true, angle=hz, clip=pl, load=1})
end
print(pl:area())

--@ chunk 355
print(wait(8*24*60)); print(drying(800,340), drying(760,90), drying(650,250))

--@ chunk 356
print(wait(20*24*60)); print(drying(800,340), drying(760,90), drying(900,150), drying(600,340))

--@ chunk 357
print(wait(10*24*60)); print(drying(760,90), drying(770,85), drying(740,100))

--@ chunk 358
local s="" for i,p in ipairs(profT) do s = s .. string.format("(%.0f,%.0f) ", p[1], p[2]) end print(s)

--@ chunk 359
skyM = above(function(x) return 394 end) - (farLineF:mask():grow(1) + below(profT)) - newG:shrink(2)
local hz = function(x,y) return 0.04*math.sin(x/130+y/70) end
local function band(a, b) return skyM * below(function(x) return a + 15*math.sin(x/170) end) * above(function(x) return b + 15*math.sin(x/170+1) end) end
work(band(-20, 95), {hand="broad", pile=skyT, coverage=2.6, fill=true, angle=hz, clip=skyM})
work(band(80, 175), {hand="broad", pile=skyU, coverage=2.6, fill=true, angle=hz, clip=skyM})
work(band(160, 255), {hand="broad", pile=skyR, coverage=2.6, fill=true, angle=hz, clip=skyM})
work(band(240, 330), {hand="broad", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=skyM})
work(band(315, 420) * mask(function(x,y) return x < 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyL, coverage=2.6, fill=true, angle=hz, clip=skyM})
work(band(315, 420) * mask(function(x,y) return x >= 680 and 1 or 0 end), {hand="body", tool="filbert 16", pile=skyG, coverage=2.6, fill=true, angle=hz, clip=skyM})
blend(skyM:shrink(3), {angle=0.02})
blend(skyM:shrink(3), {angle=-0.03})

--@ chunk 360
local g = newG
local nzA = noise{seed=611, period=40, octaves=3}
local ang = function(x,y) return -1.0 + 0.8*math.sin(x/26 + y/35) end
-- body of all masses
work(g, {hand="body", tool="filbert 10", pile=gDk, coverage=3, fill=true, angle=ang, clip=g:grow(1), load=1, pressure={0.55,0.9}})
-- olive light on the glow side (left / lower left edges), broken by noise
local litM = mask(function(x,y)
  local a = g:at(x,y); if a < 0.5 then return 0 end
  local b = g:at(x-16, y+4)
  local r = (1 - b) * (0.55 + 0.6*nzA(x,y))
  return clamp(r, 0, 1)
end):blur(5) * mask(function(x,y) return 1 - 0.7*smoothstep(860, 980, x) end)
work(litM, {hand="body", tool="filbert 7", pile=gOl, coverage=1.3, edge="lost", angle=ang, load=0.6, pressure={0.35,0.6}})
-- cool sky light on the tops of the crowns
local topM = mask(function(x,y)
  local a = g:at(x,y); if a < 0.5 then return 0 end
  local b = g:at(x+2, y-14)
  return clamp((1 - b) * (0.5 + 0.7*nzA(x+300,y)), 0, 1)
end):blur(5)
work(topM, {hand="body", tool="filbert 7", pile=gCool, coverage=1.1, edge="lost", angle=ang, load=0.5, pressure={0.3,0.55}})
-- deep core: under the elm crown and in the right tree's heart
local deepM = (E2:mask():shrink(14) * mask(function(x,y) return smoothstep(170, 270, y) * (0.6 + 0.5*nzA(x+600,y)) end)
   + R2:mask():shrink(16) * mask(function(x,y) return smoothstep(900, 980, x) * (0.6 + 0.5*nzA(x,y+500)) end)
   + L2:mask():shrink(12) * mask(function(x,y) return smoothstep(650, 700, x) end)):blur(8)
work(deepM, {hand="body", tool="filbert 10", pile=gDeep, coverage=1.3, edge="lost", angle=ang, load=0.7, pressure={0.5,0.85}})
-- haze along the top of the understory, where it is seen against the glow
local hazeM = U2:mask() * (above(function(x) return 404 end) * below(function(x) return 376 end)):soften(10) * mask(function(x,y) return smoothstep(705, 740, x) * (1 - smoothstep(850, 880, x)) end)
work(hazeM, {hand="scumble", pile=gCool, coverage=1.0, pressure={0.3,0.5}, load=0.45, angle=0})
-- blend inside the dark only, gently, to marry the modelling
blend(g:shrink(5), {angle=-0.9, pressure={0.15,0.3}})

--@ chunk 361
local g = newG
local count = 0
clumpAll = nil
local function addClumps(o, n, seed, rmin, rmax, out)
  for i, t in ipairs(uneven(n, 0, 1, 0.7, 0.5, seed)) do
    local px, py = o:at(t)
    local qx, qy = o:at((t + 0.003) % 1)
    local tx, ty = qx-px, qy-py
    local L = math.sqrt(tx*tx+ty*ty) + 1e-6
    local nx, ny = ty/L, -tx/L
    if g:at(px+nx*6, py+ny*6) > 0.5 then nx, ny = -nx, -ny end
    if g:at(px+nx*12, py+ny*12) < 0.5 and py < 392 and px < 1000 then
      local r = rand(rmin, rmax)
      local cx, cy = px + nx*rand(0, out)*r/10, py + ny*rand(0, out)*r/10
      -- drooping: shift down a bit on the sides
      cy = cy + rand(0, 3)
      local pts = {}
      local k = 7
      for j = 1, k do
        local a = (j-1)/k*2*math.pi + rand(-0.3,0.3)
        local rr = r * rand(0.6, 1.1)
        table.insert(pts, {cx + math.cos(a)*rr*1.15, cy + math.sin(a)*rr*0.85})
      end
      pts.closed = true; pts.char = "soft"; pts.seed = seed*100+i; pts.lobe = 3; pts.amount = 1.0
      local m = outline(pts):mask()
      clumpAll = clumpAll and (clumpAll + m) or m
      count = count + 1
    end
  end
end
addClumps(E2, 70, 711, 5, 13, 9)
addClumps(R2, 26, 712, 5, 12, 8)
addClumps(L2, 30, 713, 4, 11, 8)
print(count)
work(clumpAll, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.0, clip=clumpAll, load=1, pressure={0.5,0.8}})

--@ chunk 362
grove3 = newG + clumpAll
local sft = grove3:blur(8)
tangent3 = function(x,y)
  local gx = sft:at(x+3,y) - sft:at(x-3,y)
  local gy = sft:at(x,y+3) - sft:at(x,y-3)
  return math.atan(gy, gx) + math.pi/2
end
local top = above(function(x) return 392 end)
blend((grove3:grow(4) - grove3:shrink(4)) * top, {angle=tangent3, pressure={0.12,0.25}})

--@ chunk 363
local top = above(function(x) return 392 end)
local nzB = noise{seed=733, period=60, octaves=3}
local ring = (grove3:grow(1) - grove3:shrink(12)) * top
local tool = {kind="filbert", width=5, stiffness=0.5, splay=0.3, ragged=0.4}
stipple(ring, {pile=gDk, tool=tool, coverage=function(x,y) return clamp(0.2 + 1.4*nzB:at01(x,y)*nzB:at01(x,y)*2, 0, 1.8) end, pressure={0.4,0.75}, drag={3,-1.2}, twist=0.5, cluster={0.6,12}, feather=0.5, dips={14,0.8,0.25}})
blend((grove3:shrink(3) - grove3:shrink(20)) * top, {angle=tangent3, pressure={0.15,0.28}})

--@ chunk 364
local rr = rag{width=9}
rr:wipe({{540,432},{548,422},{556,410},{564,398},{572,386}}, {pressure=0.6})
rr:refold()
rr:wipe({{548,434},{570,436},{600,436},{630,434}}, {pressure=0.6})
rr:refold()
rr:wipe({{546,428},{566,430},{590,431}}, {pressure=0.5})

--@ chunk 365
local bankL = outline{{544,424},{548,412},{556,402},{568,394},{584,390},{604,392},{606,424},{580,427},{560,427}, closed=true, char="soft", seed=5201, lobe=3, amount=0.8}:mask()
work(bankL, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-0.2, clip=bankL, load=1, pressure={0.6,0.9}})
work(bankL * above(function(x) return 412 end), {hand="body", tool="filbert 4", pile=gCool, coverage=0.8, angle=0, clip=bankL, load=0.4, pressure={0.3,0.5}})
blend(bankL:grow(2), {angle=0, pressure={0.12,0.22}})

--@ chunk 366
local cl = {
 {740,140,34,"cool"},{822,118,34,"cool"},{866,196,28,"cool"},{722,228,34,"olive"},{800,250,30,"olive"},
 {948,250,34,"cool"},{930,330,28,"olive"},{622,330,30,"olive"},{596,380,24,"olive"},{690,180,22,"olive"}}
local inside = grove3:shrink(4)
for i, c in ipairs(cl) do
  local cx, cy, r, kind = c[1], c[2], c[3], c[4]
  local disk = ellipse(cx, cy, r*1.1, r*0.9):roughen(5, 18, 900+i)
  local dx, dy = (kind == "olive") and 9 or 3, (kind == "olive") and 4 or 10
  local lit = (disk - ellipse(cx+dx, cy+dy, r*1.1, r*0.9):roughen(5, 18, 950+i)) * inside
  local shade = (disk - ellipse(cx-dx*0.8, cy-dy*0.8, r*1.1, r*0.9)) * inside
  work(lit:soften(3), {hand="body", tool="filbert 6", pile=(kind=="olive") and gOl or gCool, coverage=1.0, angle=-0.9, clip=inside, load=0.45, pressure={0.3,0.55}})
  work(shade:soften(3), {hand="body", tool="filbert 6", pile=gDeep, coverage=1.0, angle=-0.9, clip=inside, load=0.55, pressure={0.4,0.7}})
end
blend(grove3:shrink(6) * above(function(x) return 392 end), {angle=-0.9, pressure={0.15,0.28}})

--@ chunk 367
blend(grove3:shrink(8) * above(function(x) return 392 end), {angle=0.7, pressure={0.15,0.25}})
blend(grove3:shrink(8) * above(function(x) return 392 end), {angle=-1.3, pressure={0.12,0.2}})

--@ chunk 368
print(wait(21*24*60)); print(drying(790,340), drying(780,200), drying(600,380), drying(300,200), drying(900,100))

--@ chunk 369
local function limb(pts, w, pr, pl)
  local rb = brush{kind="round", width=w, point=0.5, stiffness=0.5}
  rb:load(pl or trunkP2, 1)
  rb:stroke(pts, {pressure=pr or {1,0.6}, ramps={0.02,0.3}, shake=0.6})
end
-- the elm: trunk and three limbs into the crown
limb({{791,386},{789,366},{786,348},{784,332}}, 9, {1,0.9})
limb({{785,336},{775,322},{764,309},{752,297},{742,286}}, 6, {0.9,0.6})
limb({{786,334},{789,318},{793,302},{796,288}}, 5, {0.9,0.6})
limb({{787,338},{800,326},{814,312},{828,300},{838,290}}, 5, {0.9,0.5})
-- smaller branches
limb({{760,305},{752,304},{744,300}}, 2.5, {0.8,0.2})
limb({{812,314},{820,316},{830,312}}, 2.2, {0.8,0.2})
limb({{775,322},{770,312},{768,300}}, 2.2, {0.8,0.2})
-- a slender sapling in the gap and the trunks at its sides
limb({{742,380},{740,358},{738,334},{735,310}}, 3, {0.8,0.4})
limb({{856,380},{857,354},{855,330},{858,306}}, 6, {1,0.7})
limb({{726,380},{728,356},{727,334}}, 4, {0.9,0.5})

--@ chunk 370
trunkF = pile{{"raw umber",3},{"bone black",1.3},{"Indian red",0.15},{"cobalt blue",0.2}, medium=0.35}
local function limb(pts, w, pr, kind)
  local rb = brush{kind=kind or "round", width=w, point=0.4, stiffness=0.35}
  for k=1,2 do
    rb:load(trunkF, 1)
    rb:stroke(pts, {pressure=pr or {1,0.6}, ramps={0.02,0.3}, shake=0.4})
  end
end
limb({{791,388},{789,366},{786,348},{784,332}}, 9, {1,0.9}, "filbert")
limb({{785,336},{775,322},{764,309},{752,297},{742,286}}, 6, {0.95,0.6})
limb({{786,334},{789,318},{793,302},{796,288}}, 5, {0.95,0.6})
limb({{787,338},{800,326},{814,312},{828,300},{838,290}}, 5, {0.95,0.5})
limb({{742,382},{740,358},{738,334},{735,310}}, 3, {0.9,0.5})
limb({{856,382},{857,354},{855,330},{858,306}}, 6, {1,0.7}, "filbert")
limb({{726,382},{728,356},{727,334}}, 4, {0.95,0.6})

--@ chunk 371
local shapes = {
  outline{{720,384},{722,366},{730,352},{742,348},{752,356},{758,370},{764,384}, closed=true, char="soft", seed=5301, lobe=4, amount=1.2},
  outline{{770,386},{774,372},{786,366},{800,368},{812,374},{818,386}, closed=true, char="soft", seed=5302, lobe=4, amount=1.2},
  outline{{826,386},{830,362},{840,346},{852,338},{864,344},{866,386}, closed=true, char="soft", seed=5303, lobe=4, amount=1.2},
  outline{{838,284},{850,284},{862,290},{864,304},{858,318},{848,316},{840,304}, closed=true, char="soft", seed=5304, lobe=4, amount=1.2},
  outline{{796,288},{808,290},{816,298},{812,306},{802,304}, closed=true, char="soft", seed=5305, lobe=3, amount=1.2},
  outline{{722,296},{736,292},{744,300},{740,312},{730,318},{722,312}, closed=true, char="soft", seed=5306, lobe=3, amount=1.2},
}
local m = nil
for _, o in ipairs(shapes) do m = m and (m + o:mask()) or o:mask() end
work(m, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.0, clip=m, load=1, pressure={0.5,0.8}})
-- haze over the far bushes low in the gap
local hz = (m * below(function(x) return 360 end)) * mask(function(x,y) return x > 760 and x < 830 and 1 or 0 end)
work(hz, {hand="scumble", pile=gCool, coverage=0.7, load=0.35, pressure={0.25,0.45}, angle=0})
local sft = m:blur(6)
local tg = function(x,y)
  local gx = sft:at(x+3,y) - sft:at(x-3,y)
  local gy = sft:at(x,y+3) - sft:at(x,y-3)
  return math.atan(gy, gx) + math.pi/2
end
blend(m:grow(3) - m:shrink(3), {angle=tg, pressure={0.12,0.22}})
-- soften the trunks a touch, along their length
blend(ribbon({{791,388},{789,366},{786,348},{784,332}}, 14), {angle=-1.6, pressure={0.1,0.18}})
blend(ribbon({{856,382},{857,354},{855,330},{858,306}}, 10), {angle=-1.6, pressure={0.1,0.18}})

--@ chunk 372
local low = (grove3 + outline{{544,424},{548,412},{556,402},{568,394},{584,390},{604,392},{606,424},{580,427},{560,427}, closed=true, char="soft", seed=5201, lobe=3, amount=0.8}:mask()) * below(function(x) return 360 + 6*math.sin(x/23) end) * above(function(x) return 426 end) * mask(function(x,y) return x < 720 and 1 or 0 end)
work(low:shrink(1), {hand="body", tool="filbert 7", pile=gDk, coverage=3, fill=true, angle=function(x,y) return -0.6 + 0.6*math.sin(x/15) end, clip=low, load=1, pressure={0.5,0.85}})
-- a cool band of mist in the foot of the mass, wet into wet
local mist = low * (below(function(x) return 398 + 4*math.sin(x/30) end) * above(function(x) return 414 end)):soften(6) * mask(function(x,y) return smoothstep(556, 600, x) end)
work(mist, {hand="scumble", pile=gCool, coverage=0.9, pressure={0.3,0.5}, load=0.4, angle=0})
blend(low:shrink(3), {angle=0, pressure={0.15,0.28}})
blend(low:shrink(3), {angle=-1.2, pressure={0.12,0.22}})

--@ chunk 373
local lob = {
  outline{{900,226},{904,208},{916,196},{932,192},{944,200},{946,216},{930,224}, closed=true, char="soft", seed=5401, lobe=4, amount=1.2},
  outline{{946,212},{950,190},{962,178},{978,174},{992,180},{1004,192},{1006,214}, closed=true, char="soft", seed=5402, lobe=4, amount=1.2},
  outline{{882,250},{874,238},{878,224},{890,220},{900,230},{896,246}, closed=true, char="soft", seed=5403, lobe=3, amount=1.2},
}
local m = nil
for _, o in ipairs(lob) do m = m and (m + o:mask()) or o:mask() end
local all = m + R2:mask()
work(m:grow(4) * R2:mask():grow(14), {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=-1.0, clip=m:grow(5), load=1, pressure={0.5,0.8}})
work(m, {hand="body", tool="filbert 5", pile=gDk, coverage=3, fill=true, angle=-1.0, clip=m, load=1, pressure={0.5,0.8}})
-- cool top-light on the lobes and the right crown's top, broken
local nzC = noise{seed=741, period=30, octaves=3}
local topM = mask(function(x,y)
  if all:at(x,y) < 0.5 or x < 870 then return 0 end
  return clamp((1 - all:at(x+1, y-12)) * (0.5 + 0.8*nzC(x,y)), 0, 1)
end):blur(4)
work(topM, {hand="body", tool="filbert 5", pile=gCool, coverage=1.0, edge="lost", angle=-0.9, load=0.4, pressure={0.3,0.5}})
local sft = all:blur(6)
local tg = function(x,y)
  local gx = sft:at(x+3,y) - sft:at(x-3,y)
  local gy = sft:at(x,y+3) - sft:at(x,y-3)
  return math.atan(gy, gx) + math.pi/2
end
blend((all:grow(3) - all:shrink(3)) * rect(860, 160, 160, 120), {angle=tg, pressure={0.12,0.22}})
blend(m:shrink(3) + (R2:mask():shrink(4) * rect(870, 200, 140, 50)), {angle=-0.9, pressure={0.12,0.22}})

--@ chunk 374
print(wait(21*24*60)); print(drying(790,340), drying(600,400), drying(950,200))

--@ chunk 375
local mm = meadowF:shrink(1)
local lightZone = outline{{280,500},{360,486},{440,500},{520,512},{548,540},{540,600},{520,667},{300,672},{200,620},{220,540}, closed=true, char="soft", seed=5501, lobe=20}:mask():blur(30)
work(mm, {hand="glaze", tool={kind="filbert", width=26, stiffness=0.3}, pile=vig, coverage=2.2, fill=true, clip=mm, angle=function(x,y) return 0.04*math.sin(x/90) end, pressure={0.5,0.8}, length={140,280},
  load_at=function(x,y)
    local base = 0.12 + 0.25*smoothstep(470, 640, y)
    local edges = 0.25*(smoothstep(250, 0, x) + smoothstep(760, 1000, x))*smoothstep(480,600,y)
    local fix = 0.45*lightZone:at(x,y)
    local near = 1 - 0.7*(1 - smoothstep(440, 470, y))
    return clamp((base + edges + fix)*near, 0, 1)
  end})
local rr = rag{width=14}
rr:wipe({{466,672},{448,640},{428,606},{418,578},{424,554},{440,528},{452,510}}, {pressure={0.45,0.25}})

--@ chunk 376
print(wait(14*24*60))

--@ chunk 377
local seam = outline{{704,350},{716,340},{730,346},{736,380},{734,420},{722,440},{704,432},{700,390}, closed=true, char="soft", seed=5601, lobe=5}:mask()
work(seam, {hand="body", tool="filbert 6", pile=gDk, coverage=3, fill=true, angle=-1.2, edge="soft", load=0.9, pressure={0.5,0.8}})
local hz = seam * (below(function(x) return 382 end) * above(function(x) return 404 end)):soften(6)
work(hz, {hand="scumble", pile=gCool, coverage=0.7, load=0.35, pressure={0.25,0.45}, angle=0})
blend(seam:grow(4), {angle=0, pressure={0.15,0.25}})

--@ chunk 378
print(wait(30*24*60))

--@ chunk 379
print(drying(700,200), drying(500,500), drying(900,300), drying(450,440))

--@ chunk 380
for _,p in ipairs{{700,200},{760,80},{640,250},{600,300},{560,400},{990,200},{960,400},{720,300},{780,340},{600,420}} do print(p[1],p[2], grove3:at(p[1],p[2]), meadowF:at(p[1],p[2])) end
print(meadowF:at(300,450), meadowF:at(300,435), meadowF:at(500,460), meadowF:at(800,470))

--@ chunk 381
for _,p in ipairs{{100,440},{100,500},{900,600},{300,640},{900,470},{990,470},{600,470},{20,660}} do print(p[1],p[2], meadowF:at(p[1],p[2]), meadow:at(p[1],p[2])) end
for _,n in ipairs{"E2","L2","R2","U2"} do local o=_G[n]; local m=o:mask(); print(n, m:at(700,200), m:at(990,200), m:at(600,300), m:at(900,300), m:at(740,380)) end

--@ chunk 382
gAll = E2:mask() + R2:mask() + L2:mask() + U2:mask()
print(gAll:area())
for _,p in ipairs{{760,60},{655,250},{915,170},{600,290},{560,380},{995,180}} do print(p[1],p[2],gAll:at(p[1],p[2])) end

--@ chunk 383
for _,p in ipairs{{794,69},{890,117},{695,95},{720,80},{663,170},{900,200},{880,330}} do print(p[1],p[2],gAll:at(p[1],p[2]), gAll:grow(15):at(p[1],p[2])) end

--@ chunk 384
crownO = outline{{548,425,"c"},{548,395},{560,378},{585,362},{590,330},{600,305},{618,290},{642,280},{658,262},{655,230},{658,200},{672,180},{678,155},{700,140},{705,112},{735,95},{742,75},{775,58},{800,68},{840,85},{862,105},{878,140},{900,165},{915,185},{945,178},{975,162},{1000,158,"c"},{1000,470,"c"},{548,470,"c"}, amount=0.4, seed=11}
crownM = crownO:mask()
edgeDk = pile{{"raw umber",2.5},{"bone black",1.1},{"yellow ochre",0.4},{"Antwerp blue",0.2},{"lead white",0.5}, medium=0.3, thinner=0.35}
local n = lose(-crownM, {pile=edgeDk, where=function(x,y) return (x>670 and x<760 and y<175) and 1 or 0 end, tool="filbert 4", reach={10,16}, load=0.25, seed=5})
print(n)

--@ chunk 385
local nz = noise{seed=21, period=60}
local n = lose(-crownM, {pile=edgeDk, where=function(x,y)
  if y > 415 or x > 995 then return 0 end
  if x>670 and x<760 and y<175 then return 0 end
  return clamp(0.55 + 0.5*nz(x,y), 0, 1) end, tool="filbert 4", reach={10,16}, load=0.25, seed=6})
print(n)

--@ chunk 386
r = rag{width=10}
r:dip(0.5)
r:wipe(poly({{925,175},{945,140},{995,135},{995,158},{970,163},{945,175}}), {pressure=0.5, angle=0.3, passes=2, refold=0.3})

--@ chunk 387
r:refold(); r:dip(0.6)
for i=0,4 do
  local y = 150 + i*5
  r:wipe({{930,y+12},{960,y},{995,y-6}}, {pressure={0.6,0.6}})
end
print(drying(960,155))

--@ chunk 388
r:refold(); r:dip(0.7)
for i=0,5 do
  local y = 140 + i*5
  r:wipe({{935,y+14},{965,y+2},{998,y-4}}, {pressure={0.7,0.7}})
end

--@ chunk 389
local low = meadowF * below(function(x) return 470 + 0.02*x end):soften(25)
work(low, {hand="glaze", pile=mgl3, angle=function(x,y) return 0.03 end, coverage=1.0, load=0.5, seed=31})

--@ chunk 390
local band = meadowF * above(function(x) return 505 + 0.02*x end)
local keep = ellipse(450, 470, 75, 30):soften(30)
local m = (band - keep):soften(15)
work(m, {hand="glaze", pile=mgl3, angle=0.02, coverage=0.9, load=0.4, seed=32})

--@ chunk 391
r = rag{width=8}
r:dip(0.6)
for i=0,3 do
  local y = 414 + i*4.5
  r:wipe({{0,y},{60,y-1},{120,y},{190,y+2}}, {pressure={0.6,0.6}})
  r:refold()
end
r:dip(0.6)
for i=0,2 do
  local d = 4 + i*4
  r:wipe({{540,456-d},{600,450-d},{650,446-d},{695,441-d}}, {pressure={0.55,0.55}})
  r:refold()
end

--@ chunk 392
print(drying(50,425), drying(160,428), drying(600,445), drying(300,520))

--@ chunk 393
r = rag{width=12}
r:dip(0.9)
local m1 = poly({{0,430},{0,410},{200,410},{200,432},{120,431},{60,432}}, true)
r:wipe(m1, {pressure=0.85, angle=0, passes=3, refold=0.2})
r:refold(); r:dip(0.9)
local m2 = poly({{520,452},{580,436},{700,428},{700,440},{620,446},{540,456}}, true)
r:wipe(m2, {pressure=0.85, angle=-0.1, passes=3, refold=0.2})

--@ chunk 394
print(wait(7*24*60))

--@ chunk 395
local m = (crownM:shrink(3) * above(function(x) return 415 end)):soften(4)
work(m, {hand="glaze", pile=groveGl3, coverage=0.8, load=0.3, clip=true, seed=41})

--@ chunk 396
r = rag{width=20}
r:dip(0.9)
local m = crownM:grow(4) * above(function(x) return 420 end)
r:wipe(m, {pressure=0.8, angle=0, passes=3, refold=0.15})
r = rag{width=20}; r:dip(0.9)
r:wipe(m, {pressure=0.8, angle=1.2, passes=2, refold=0.15})

--@ chunk 397
print(wait(14*24*60))

--@ chunk 398
local m = R2:mask()
local x0,x1,y0,y1=1e9,-1e9,1e9,-1e9
for y=0,667,5 do for x=0,1000,5 do if m:at(x,y)>0.5 then x0=math.min(x0,x);x1=math.max(x1,x);y0=math.min(y0,y);y1=math.max(y1,y) end end end
print(x0,x1,y0,y1, m:area())
for _,p in ipairs({{900,300},{950,250},{980,400},{890,450}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 399
veilT = pile{{"lead white",5},{"cobalt blue",0.5},{"Indian red",0.3},{"raw umber",0.6},{"yellow ochre",0.2}, medium=0.2, thinner=0.65}
local base = R2:mask():shrink(6):soften(10)
rtVeil = base:times(function(x,y) return clamp(1-(y-200)/200,0,1)*clamp((x-875)/40,0,1) end)
work(rtVeil, {hand="glaze", pile=veilT, clip=true, coverage=1, angle=function(x,y) return -1.2 end, load=0.35, pressure={0.15,0.3}, seed=7})

--@ chunk 400
local r2 = rag{width=18}
r2:dip(0.7)
r2:wipe(R2:mask():shrink(4) * rect(870,180,130,260), {pressure=0.55, angle=-1.2, passes=2, refold=0.3})
r2:refold(); r2:dip(0.6)
r2:wipe(R2:mask():shrink(4) * rect(870,180,130,260), {pressure=0.6, angle=0.3, passes=1, refold=0.3})

--@ chunk 401
local r3 = rag{width=16}
r3:dip(0.8)
r3:wipe(R2:mask():shrink(3) * rect(870,180,130,280), {pressure=0.65, angle=-1.2, passes=2, refold=0.25})
r3:refold(); r3:dip(0.8)
r3:wipe(R2:mask():shrink(3) * rect(870,180,130,280), {pressure=0.65, angle=-1.1, passes=1, refold=0.25})

--@ chunk 402
print(wait(3*24*60)); print(drying(940,280), drying(920,330))

--@ chunk 403
local m = R2:mask():shrink(8) * rect(880,200,120,230)
stipple(m:soften(8), {pile=gDk, width=5, coverage=0.9, pressure={0.3,0.6}, cluster={0.6, 18}, feather=0.5, drag=2, dips={30,0.5,0.3}, clip=R2:mask():shrink(4), seed=11})

--@ chunk 404
dryDk = pile{{"raw umber",2.4},{"bone black",0.8},{"yellow ochre",0.6},{"raw sienna",0.3}, medium=0.05}
local t = poly({{40,470},{160,468},{165,520},{45,525}}, true):soften(8)
work(t, {hand="broad", tool="filbert 14", pile=dryDk, load=0.25, pressure={0.08,0.18}, angle=0.03, coverage=1.5, length={40,90}, seed=3})

--@ chunk 405
for x=0,1000,50 do local yt
 for y=380,667 do if meadowF:at(math.min(x,999),y)>0.5 then yt=y break end end
 print(x, yt) end
print(pathShape:area(), newPath:area())
for _,p in ipairs({{100,500},{450,600},{800,500},{452,490}}) do print(drying(p[1],p[2])) end

--@ chunk 406
for _,nm in ipairs({"newPath","pathShape","pathNarrow","oldPath"}) do local m=_G[nm]
 local s=nm..": "
 for y=480,667,30 do local xs={} for x=300,600,2 do if m:at(x,y)>0.5 then xs[#xs+1]=x end end
 if #xs>0 then s=s..string.format("y%d[%d-%d] ",y,xs[1],xs[#xs]) end end print(s) end

--@ chunk 407
mA = pile{{"yellow ochre",1.2},{"raw umber",1.6},{"lead white",0.45},{"bone black",0.25},{"raw sienna",0.25},{"cobalt blue",0.08}, medium=0.1}
mB = pile{{"raw umber",2.4},{"yellow ochre",0.9},{"bone black",0.6},{"raw sienna",0.4},{"lead white",0.15}, medium=0.1}
local t = poly({{180,445},{300,455},{300,485},{180,480}}, true):soften(6)
work(t, {hand="broad", tool="filbert 14", pile=mA, load=0.5, pressure={0.35,0.6}, angle=0.05, coverage=2.5, length={40,100}, seed=5})

--@ chunk 408
local r4 = rag{width=14}
r4:dip(0.9)
local m = rect(150,425,200,70)
r4:wipe(m, {pressure=0.6, angle=0, passes=3, refold=0.2})
r4:refold(); r4:dip(0.9)
r4:wipe(m, {pressure=0.7, angle=0.1, passes=2, refold=0.2})

--@ chunk 409
print(drying(200,465), drying(230,438))
local r5 = rag{width=12}
for i=1,4 do r5:dip(1); r5:wipe(rect(150,440,200,50), {pressure=0.8, angle=0, passes=1, refold=0.15}); r5:refold() end
print(r5)

--@ chunk 410
local r6 = rag{width=8}
for i=1,3 do r6:dip(1); r6:wipe({{180,432},{215,436},{250,441},{280,444}}, {pressure={0.6,0.5}}); r6:refold() end
for i=1,2 do r6:dip(1); r6:wipe({{175,429},{200,431},{225,437}}, {pressure=0.6}); r6:refold() end

--@ chunk 411
mRub = pile{{"raw umber",2},{"yellow ochre",1},{"bone black",0.35},{"raw sienna",0.3},{"lead white",0.2}, medium=0.3}
fig = ellipse(452,486,14,22)
bandA = (meadowF:shrink(2) * above(function(x) return 515 + 12*math.sin(x/90) + x*0.015 end):soften(10)) - fig:grow(4)
local L = bandA * rect(0,400,430,200)
work(L, {hand="broad", tool="filbert 14", pile=mRub, load=0.45, pressure={0.35,0.6}, angle=0.04, coverage=2, length={40,100}, clip=true, seed=21})
local rr = rag{width=16}
for i=1,2 do rr:dip(0.9); rr:wipe(L, {pressure=0.6, angle=0.03, passes=1, refold=0.2}); rr:refold() end

--@ chunk 412
local L = bandA * rect(0,400,440,200)
for k=1,3 do
 local rr = rag{width=14}
 for i=1,3 do rr:dip(1); rr:wipe(L, {pressure=0.8, angle=0.03+0.05*i, passes=1, refold=0.12}); rr:refold() end
 print(rr)
end

--@ chunk 413
print(wait(5*24*60))

--@ chunk 414
local g = meadowF:shrink(2):times(function(x,y) return smoothstep(470,530,y)*clamp((470-x)/80,0,1) end) - ellipse(452,486,18,26)
work(g, {hand="glaze", pile=mgl3, clip=true, coverage=1.2, angle=0.03, load=0.4, pressure={0.2,0.35}, seed=31})

--@ chunk 415
print(wait(4*24*60))

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
