-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 3

--@ chunk 1
canvas{size=1140, aspect=1.5, linen={16,14}, seed=41,
  ground={{pile={{"lead white",10},{"yellow ochre",0.4}}, um=120, apply="knife", texture=0.25},
          {pile={{"red earth",2},{"raw umber",2},{"lead white",3}}, um=6, apply="brush"}}}
print(W,H)

--@ chunk 2
HZ=412
c = chalk()
c:sketch({{0,HZ},{250,408},{520,414},{760,410},{1000,416}}, {pressure=0.25})
c:sketch({{0,372},{140,362},{330,380},{480,388},{640,384},{800,372},{1000,366}}, {pressure=0.2})
-- big left tree mass
c:sketch({{70,400},{50,300},{90,190},{170,110},{270,90},{360,140},{410,230},{400,320},{350,390}}, {pressure=0.25})
c:sketch({{238,450},{242,380},{250,300}}, {pressure=0.3})
-- right slender trees
c:sketch({{820,440},{826,330},{818,200},{830,120}}, {pressure=0.3})
c:sketch({{880,436},{872,330},{884,230}}, {pressure=0.25})
c:sketch({{760,200},{790,110},{850,70},{910,110},{930,200},{890,260},{800,260},{760,200}}, {pressure=0.2})
-- pool
c:sketch({{380,540},{470,515},{600,512},{700,530},{690,570},{560,592},{420,580},{380,540}}, {pressure=0.2})
-- lay-in piles
umb = pile{{"raw umber",3},{"bone black",0.5},{"red earth",0.3}, medium=0.3, thinner=0.45}
umbd = pile{{"raw umber",3},{"bone black",1.2},{"Antwerp blue",0.3}, medium=0.25, thinner=0.3}

-- masks
function treeL()
  local o = outline{{60,405},{40,310},{75,210},{120,150},{180,105},{265,85},{340,120},{385,180},{420,250},{405,330},{370,395},{300,420},{150,418}, char="soft", seed=5, lobe=38, closed=true}
  return o:mask()
end
function treeR()
  local a = outline{{770,215},{785,130},{835,72},{895,95},{928,170},{915,240},{860,275},{800,262}, char="soft", seed=9, lobe=26, closed=true}
  local b = outline{{850,300},{862,235},{905,225},{935,275},{920,330},{875,340}, char="soft", seed=11, lobe=20, closed=true}
  return a:mask(), b:mask()
end
local tl = treeL()
local ta, tb = treeR()
local far = (below(function(x) return 372 + 10*math.sin(x/90) + 6*math.sin(x/37) end) * above(function(x) return HZ+4 end))
local fg = below(function(x) return HZ end)
work(far, {hand="scumble", pile=umb, coverage=0.8})
work(fg, {hand="broad", pile=umb, coverage=1.0, angle=0.05})
work(tl, {hand="scumble", pile=umbd, coverage=1.3, tool="filbert 16", length={25,50}})
work(ta, {hand="scumble", pile=umbd, coverage=0.8, tool="filbert 12"})
work(tb, {hand="scumble", pile=umbd, coverage=0.7, tool="filbert 12"})
-- foreground darks: lower corners and band under trees
local dk = (below(function(x) return 560 + 40*math.sin(x/160) end) + rect(0,405,460,60) + rect(730,410,270,50)):roughen(14,60,3)
work(dk*fg, {hand="scumble", pile=umbd, coverage=0.9, tool="filbert 16", length={30,70}, angle=0.1})
-- wipe out the pool and a light path in the meadow
r = rag{width=30}
local pool = poly({{385,542},{470,518},{600,514},{700,532},{688,568},{560,590},{425,580}}, true)
r:wipe(pool, {pressure=0.7, angle=0.03, passes=2})
r:refold()
r:wipe({{480,470},{600,462},{720,470}}, {pressure=0.5})
r:wipe({{440,440},{560,432},{700,436}}, {pressure=0.4})

--@ chunk 3
blend(below(function(x) return 60 end), {angle=0.2})
wait(60*20)
skyGlow = pile{{"lead white",10},{"lemon chrome",2},{"orange chrome",0.5},{"yellow ochre",1}, medium=0.15}
skyPeach = pile{{"lead white",9},{"yellow ochre",2},{"orange chrome",0.8},{"red earth",0.4},{"cobalt blue",0.3}, medium=0.15}
skyMid = pile{{"lead white",8},{"yellow ochre",1.5},{"cobalt blue",1.2},{"raw umber",0.8},{"red earth",0.3}, medium=0.15}
skyTop = pile{{"lead white",6},{"cobalt blue",2},{"Antwerp blue",0.25},{"raw umber",1.4},{"yellow ochre",0.8}, medium=0.15}
farC = pile{{"lead white",4},{"cobalt blue",1.6},{"Indian red",0.7},{"raw umber",1},{"yellow ochre",0.5}, medium=0.15}
treeD = pile{{"raw umber",3},{"Antwerp blue",0.5},{"raw sienna",2},{"bone black",1}, medium=0.2}
treeM = pile{{"raw sienna",3},{"Antwerp blue",0.45},{"yellow ochre",1.5},{"raw umber",1.5},{"red earth",0.5}, medium=0.2}
treeW = pile{{"raw sienna",2},{"orange chrome",1},{"yellow ochre",2},{"red earth",0.8},{"raw umber",0.6}, medium=0.2}
grass = pile{{"yellow ochre",3},{"Antwerp blue",0.35},{"raw sienna",2},{"raw umber",1.2},{"lead white",1}, medium=0.2}
grassD = pile{{"raw umber",3},{"Antwerp blue",0.4},{"raw sienna",1.5},{"bone black",0.6},{"red earth",0.5}, medium=0.2}
grassL = pile{{"yellow ochre",3},{"lemon chrome",1},{"lead white",2},{"Antwerp blue",0.15},{"raw sienna",1}, medium=0.2}

--@ chunk 4
local function band(y0,y1) return mask(function(x,y) return (y>=y0 and y<y1) and 1 or 0 end) end
local function glowd(x,y) local dx,dy=(x-600)/330,(y-365)/150; return math.sqrt(dx*dx+dy*dy) end
local sky = above(function(x) return 392 end)
local gl = mask(function(x,y) return (y<392 and glowd(x,y)<0.75) and 1 or 0 end)
local pe = mask(function(x,y) local d=glowd(x,y); return (y<392 and d>=0.6 and d<1.35) and 1 or 0 end)
local mi = mask(function(x,y) local d=glowd(x,y); return (y<392 and d>=1.2 and d<2.0) and 1 or 0 end)
local tp = mask(function(x,y) local d=glowd(x,y); return (y<392 and d>=1.85) and 1 or 0 end)
work(tp, {hand="broad", pile=skyTop, coverage=1.5, angle=0.1, fill=true})
work(mi, {hand="broad", pile=skyMid, coverage=1.5, angle=0.05, fill=true})
work(pe, {hand="broad", pile=skyPeach, coverage=1.5, angle=0, fill=true, length={60,140}})
work(gl, {hand="body", pile=skyGlow, coverage=1.6, angle=0, fill=true, tool="filbert 16", length={40,90}})
blend(sky, {angle=0.1})
blend(sky, {angle=-0.15})

--@ chunk 5
skyDk = pile{{"lead white",4},{"cobalt blue",2},{"raw umber",2},{"Indian red",0.5},{"yellow ochre",0.6}, medium=0.15}
skyRose = pile{{"lead white",7},{"orange chrome",1},{"Indian red",0.5},{"yellow ochre",1.5},{"cobalt blue",0.4}, medium=0.15}
local tp = mask(function(x,y) return (y<150 - 60*math.exp(-((x-600)/400)^2)) and 1 or 0 end):roughen(20,150,2)
work(tp, {hand="broad", pile=skyDk, coverage=1.0, angle=0.08})
-- upper corners darker
work(poly({{0,0},{330,0},{200,130},{0,260}},true), {hand="broad", pile=skyDk, coverage=0.9, angle=-0.3})
work(poly({{1000,0},{760,0},{860,100},{1000,200}},true), {hand="broad", pile=skyDk, coverage=0.7, angle=0.3})
-- rose cloud bars
local bars = ribbon({{80,255},{250,240},{430,250}}, {10,26,8}) + ribbon({{520,205},{700,190},{900,205},{1000,200}}, {8,24,22,12}) + ribbon({{0,330},{160,322},{300,332}}, {16,18,6}) + ribbon({{760,300},{900,290},{1000,296}},{6,16,14})
work(bars, {hand="body", pile=skyRose, coverage=1.1, angle=-0.04, tool="filbert 14", length={40,90}})
-- hot glow near sun
local sun = ellipse(600,368,120,36)
work(sun, {hand="body", pile=pile{{"lead white",10},{"lemon chrome",3},{"cadmium yellow",0.6}, medium=0.15}, coverage=1.3, angle=0, tool="filbert 12"})
blend(above(function(x) return 392 end), {angle=0.02})
-- far tree line into wet sky
local far = (below(function(x) return 380 + 9*math.sin(x/70+1) + 5*math.sin(x/23) - 14*math.exp(-((x-720)/60)^2) end) * above(function(x) return HZ+6 end))
work(far, {hand="body", pile=farC, coverage=1.5, angle=0, tool="filbert 8", length={15,40}, fill=true})
blend(ribbon({{0,380},{1000,380}}, 26), {angle=0})

--@ chunk 6
blend(above(function(x) return 200 end), {angle=0.05})
local fg = below(function(x) return HZ+2 end)
work(fg, {hand="body", pile=grass, coverage=1.3, angle=0.03, tool="filbert 14", length={30,80}, fill=true})
local lit = (ribbon({{330,428},{520,424},{700,428},{860,426}}, {4,12,12,3}) + ribbon({{420,462},{560,456},{720,466}}, {3,9,3}))
work(lit, {hand="body", pile=grassL, coverage=1.2, angle=0, tool="filbert 7", length={20,50}})
print("a")

--@ chunk 7
local n1 = noise{seed=7, period=180}
local dk = mask(function(x,y)
  if y < HZ+10 then return 0 end
  local t = (y-HZ)/(H-HZ)
  local side = math.max(0, 1-math.abs(x-560)/520)
  local v = t*1.25 - side*0.55 + 0.35*n1(x*0.4,y)
  return v>0.42 and 1 or 0 end)
work(dk, {hand="body", pile=grassD, coverage=1.2, angle=0.06, tool="filbert 14", length={30,70}})
print("b")
poolM = outline{{392,545},{470,522},{590,516},{690,530},{705,552},{640,574},{540,588},{430,580}, char="soft", seed=4, closed=true}:mask()
work(poolM, {hand="body", pile=skyPeach, coverage=1.4, angle=0, tool="filbert 10", length={30,70}, clip=true, fill=true})
work(poly({{470,540},{620,532},{660,550},{540,566}},true), {hand="body", pile=skyGlow, coverage=1.0, angle=0, tool="filbert 8", clip=poolM})
blend(below(function(x) return HZ+2 end), {angle=0})

--@ chunk 8
tlO = outline{{0,300},{40,235},{70,170},{130,150},{150,95},{215,60},{290,75},{330,125},{395,150},{425,215},{400,265},{430,320},{395,380},{330,400},{300,425},{120,430},{0,435}, char="soft", seed=5, lobe=34, closed=true}
tlM = tlO:mask()
work(tlM, {hand="body", pile=treeD, coverage=1.6, tool="filbert 12", length={15,40}, angle=function(x,y) return math.atan(y-300, x-240) end, edge={found=0.2, soft=0.4, lost=0.4, period=50}, fill=true})
-- lower shrub band to the left and receding hedge to the right of the tree
local hedge = outline{{300,425},{340,396},{400,392},{450,402},{500,410},{520,422},{400,432}, char="soft", seed=8, closed=true}:mask()
work(hedge, {hand="body", pile=treeD, coverage=1.3, tool="filbert 8", length={10,25}, edge="soft"})
-- mid tones inside
local n2 = noise{seed=12, period=70}
local mid = tlM:shrink(12):times(function(x,y) return (n2(x,y) > 0.1 and x>150) and 1 or 0 end)
work(mid, {hand="body", pile=treeM, coverage=0.9, tool="filbert 9", length={10,25}, angle=function(x,y) return math.atan(y-300, x-240)+1.2 end})
-- warm lit edge on sun side
local warm = tlM:rim(26, 8):times(function(x,y) return (x>330 and y>120 and y<400) and 1 or 0 end)
stipple(warm, {pile=treeW, width=5, coverage=0.7, cluster=0.6, feather=0.5})
-- ragged leaf edge against the sky
local fringe = tlM:grow(14) - tlM:shrink(6)
stipple(fringe, {pile=treeD, width=4, coverage=0.45, cluster={0.7, 30}, feather=0.6})

--@ chunk 9
blend(tlM:grow(10), {angle=1.2})
blend(tlM:grow(6), {angle=-0.5})
-- shadow under the tree across the meadow
local sh = poly({{0,420},{300,418},{520,424},{470,448},{300,462},{120,470},{0,476}}, true)
work(sh, {hand="body", pile=grassD, coverage=1.2, angle=0.03, tool="filbert 12", length={30,70}, edge="soft"})
blend(sh:grow(12), {angle=0})
-- right trees: trunks then foliage
trk = pile{{"raw umber",3},{"bone black",1.5},{"Indian red",0.4}, medium=0.2}
tb = brush{kind="round", width=7, point=0.6}
tb:load(trk, 0.9)
tb:stroke({{822,446},{826,380},{820,300},{829,220},{838,150},{846,105}}, {pressure={0.9,0.3}})
tb:load(trk, 0.8)
tb:stroke({{884,442},{878,380},{874,320},{886,262},{897,235}}, {pressure={0.75,0.25}})
tb:load(trk, 0.6)
tb:stroke({{826,250},{800,205},{790,170}}, {pressure={0.4,0.1}})
tb:stroke({{832,190},{866,150},{880,120}}, {pressure={0.4,0.1}})
tb:stroke({{876,300},{905,268},{918,250}}, {pressure={0.35,0.1}})
tb:stroke({{822,300},{852,270},{862,250}}, {pressure={0.3,0.1}})
trA = outline{{772,210},{778,150},{812,92},{850,62},{892,84},{915,130},{930,185},{905,215},{865,240},{815,252},{785,240}, char="soft", seed=9, lobe=24, closed=true}:mask()
trB = outline{{848,300},{858,248},{890,222},{925,238},{945,280},{930,322},{890,338},{860,330}, char="soft", seed=13, lobe=18, closed=true}:mask()
local n3 = noise{seed=21, period=45}
local function holes(x,y) return n3(x,y) > -0.15 and 1 or 0 end
stipple((trA+trB):times(holes), {pile=treeD, width=7, coverage=1.1, cluster={0.6,25}, feather=0.5})
stipple((trA+trB):grow(10):times(holes), {pile=treeM, width=5, coverage=0.5, cluster={0.7,25}, feather=0.6})
stipple((trA+trB):rim(20,8):times(function(x,y) return x<850 and 1 or 0 end), {pile=treeW, width=4, coverage=0.5, cluster=0.6, feather=0.5})
blend((trA+trB):grow(8), {angle=1.0})

--@ chunk 10
wait(3*24*60)
print(drying(600,300), drying(250,250), drying(850,150), drying(500,480))

--@ chunk 11
tb = brush{kind="round", width=7, point=0.7}
local function trunk(pts, p0, p1) tb:load(trk, 0.9); tb:stroke(pts, {pressure={p0,p1}}) end
trunk({{846,100},{838,150},{829,220},{821,300},{825,380},{821,452}}, 0.15, 0.85)
trunk({{897,232},{886,262},{875,320},{878,380},{884,446}}, 0.12, 0.65)
trunk({{790,168},{802,208},{826,252}}, 0.08, 0.35)
trunk({{882,118},{866,152},{833,192}}, 0.08, 0.35)
trunk({{920,248},{905,270},{876,302}}, 0.08, 0.3)
trunk({{864,248},{850,272},{823,302}}, 0.08, 0.3)
local n3 = noise{seed=21, period=40}
local function holes(x,y) return n3(x,y) > -0.05 and 1 or 0 end
local tr = trA + trB
treeR1 = pile{{"raw umber",3},{"raw sienna",2},{"Antwerp blue",0.3},{"bone black",0.8},{"red earth",0.6}, medium=0.2}
stipple(tr:times(holes), {pile=treeR1, width=7, coverage=1.0, cluster={0.7,22}, feather=0.5, drag=2})
stipple(tr:grow(9):times(holes), {pile=treeR1, width=4, coverage=0.35, cluster={0.8,22}, feather=0.7})
stipple(tr:shrink(6):times(function(x,y) return (n3(x+50,y) > 0.2 and x < 870) and 1 or 0 end), {pile=treeW, width=5, coverage=0.6, cluster=0.6, feather=0.5})

--@ chunk 12
skyG = pile{{"lead white",6},{"cobalt blue",1.6},{"raw umber",1.5},{"yellow ochre",0.8},{"Indian red",0.25}, medium=0.15}
local reg = poly({{730,40},{960,40},{980,200},{970,360},{740,360},{720,200}}, true)
local function bandm(y0,y1) return reg:times(function(x,y) return (y>=y0 and y<y1) and 1 or 0 end) end
work(bandm(0,150), {hand="body", pile=skyG, coverage=1.5, angle=0.03, tool="filbert 16", length={40,90}, edge="lost", fill=true})
work(bandm(140,215), {hand="body", pile=skyMid, coverage=1.5, angle=0.03, tool="filbert 16", length={40,90}, edge="lost", fill=true})
work(bandm(205,400), {hand="body", pile=skyPeach, coverage=1.5, angle=0.0, tool="filbert 16", length={40,90}, edge="lost", fill=true})
blend(reg:grow(15), {angle=0.03})
blend(reg:grow(15), {angle=-0.1})

--@ chunk 13
skyRG = pile{{"lead white",7},{"yellow ochre",1.3},{"Indian red",0.45},{"orange chrome",0.5},{"cobalt blue",0.8},{"raw umber",0.5}, medium=0.15}
local function d(x,y) local dx,dy=(x-600)/380,(y-372)/170; return math.sqrt(dx*dx+dy*dy) end
local keep = tlM:shrink(5)
local function zone(a,b) return mask(function(x,y) local v=d(x,y); return (y<396 and v>=a and v<b) and 1 or 0 end) - keep end
local o = {hand="broad", coverage=1.5, fill=true, length={60,160}}
local function lay(m, p, ang) o.pile=p; o.angle=ang; work(m, o) end
lay(zone(1.85,9), skyDk, 0.06)
lay(zone(1.4,1.95), skyG, 0.04)
lay(zone(0.95,1.5), skyRG, 0.02)
lay(zone(0.45,1.05), skyPeach, 0)
lay(zone(0,0.5), skyGlow, 0)
local sky = above(function(x) return 396 end) - keep
blend(sky, {angle=0.04})
blend(sky, {angle=-0.08})

--@ chunk 14
farD = pile{{"lead white",3},{"cobalt blue",1.6},{"Indian red",0.8},{"raw umber",1.4},{"yellow ochre",0.6}, medium=0.15}
local far = (below(function(x) return 388 + 7*math.sin(x/70+1) + 4*math.sin(x/23) - 16*math.exp(-((x-700)/70)^2) - 10*math.exp(-((x-960)/80)^2) end) * above(function(x) return HZ+6 end))
work(far, {hand="body", pile=farD, coverage=1.4, angle=0, tool="filbert 8", length={15,40}, fill=true, edge="soft"})
blend(ribbon({{420,388},{1000,388}}, 22), {angle=0})
-- restate the big tree
work(tlM, {hand="body", pile=treeD, coverage=1.5, tool="filbert 12", length={15,40}, angle=function(x,y) return math.atan(y-300, x-240) end, edge={found=0.15, soft=0.45, lost=0.4, period=45}, fill=true})
local n2 = noise{seed=12, period=80}
local mid = tlM:shrink(14):times(function(x,y) return (n2(x,y) > 0.15 and x>170) and 1 or 0 end)
work(mid, {hand="scumble", pile=treeM, coverage=0.7, tool="filbert 9"})
blend(tlM:shrink(10), {angle=0.8})

--@ chunk 15
russet = pile{{"raw sienna",3},{"red earth",1.2},{"raw umber",1.5},{"yellow ochre",1}, medium=0.2}
gold = pile{{"yellow ochre",3},{"lead white",2.5},{"lemon chrome",0.8},{"raw sienna",1},{"orange chrome",0.3}, medium=0.2}
deep = pile{{"raw umber",3},{"bone black",1.2},{"Antwerp blue",0.4},{"red earth",0.6}, medium=0.2}
local n1 = noise{seed=31, period=160}
local n4 = noise{seed=33, period=90}
local fgm = below(function(x) return HZ+4 end) - poolM:grow(3)
-- russet midground patches
local rus = fgm:times(function(x,y) return (n1(x*0.35,y) > 0.0 and y>430) and 1 or 0 end)
work(rus, {hand="body", pile=russet, coverage=1.1, angle=0.02, tool="filbert 12", length={30,80}, edge="lost"})
-- deep foreground and corners
local dp = fgm:times(function(x,y)
  local t=(y-HZ)/(H-HZ); local side=math.max(0,1-math.abs(x-560)/480)
  return (t*1.3 - side*0.5 + 0.3*n4(x*0.4,y) > 0.55) and 1 or 0 end)
work(dp, {hand="body", pile=deep, coverage=1.3, angle=0.04, tool="filbert 14", length={30,80}, edge="lost"})
-- shadow below big tree and right-hand bank
local sh = poly({{0,424},{300,420},{540,426},{480,452},{300,468},{0,484}}, true) + poly({{740,432},{1000,424},{1000,470},{860,462}}, true)
work(sh, {hand="body", pile=deep, coverage=1.1, angle=0.02, tool="filbert 12", length={30,70}, edge="lost"})
-- gold light on far meadow, in the sun's path
local lit = ribbon({{470,424},{600,421},{760,426}}, {3,10,3}) + ribbon({{500,470},{610,464},{730,474}}, {3,12,3}) + ribbon({{560,500},{640,497},{720,504}},{2,7,2})
work(lit, {hand="body", pile=gold, coverage=1.1, angle=0, tool="filbert 7", length={20,50}})
blend(fgm, {angle=0.02})

--@ chunk 16
wait(3*24*60)
print(drying(850,150), drying(250,250), drying(500,480), drying(600,300))
-- deepen the big tree: shadow side glaze and dark body
treeGl = pile{{"raw umber",2},{"bone black",1},{"Antwerp blue",0.4},{"raw sienna",1}, medium=0.55}
local n2 = noise{seed=44, period=70}
local shade = tlM:shrink(6):times(function(x,y) return (n2(x,y) < 0.25 or y > 330 or x < 150) and 1 or 0 end)
work(shade, {hand="scumble", pile=treeGl, coverage=1.0, tool="filbert 14", length={20,40}, edge="lost"})
-- base of tree: dark undergrowth joining ground
local base = poly({{0,395},{120,385},{300,392},{430,404},{520,424},{300,440},{0,446}}, true)
work(base, {hand="scumble", pile=deep, coverage=1.1, tool="filbert 10", edge="lost"})
-- trunk glimpses
tb = brush{kind="round", width=9, point=0.5}
tb:load(trk, 0.8)
tb:stroke({{246,330},{240,380},{236,436}}, {pressure={0.5,0.9}})
tb:load(trk, 0.5)
tb:stroke({{250,330},{275,290},{290,262}}, {pressure={0.5,0.15}})
tb:stroke({{244,335},{215,300},{200,268}}, {pressure={0.45,0.15}})

--@ chunk 17
blend(tlM:grow(4), {angle=0.9})
blend(tlM:grow(4), {angle=-0.6})
local base = poly({{0,380},{300,380},{540,400},{560,436},{300,456},{0,460}}, true)
blend(base, {angle=0.02})
blend(base, {angle=0.3})
-- warm transparent leaves on the sun side, laid on top unblended
local warm = tlM:rim(34, 10):times(function(x,y) return (x>330 and y>130 and y<390) and 1 or 0 end)
work(warm, {hand="scumble", pile=treeW, coverage=0.45, tool="filbert 6", length={6,14}, edge="lost"})

--@ chunk 18
local warm = tlM:rim(40, 10):times(function(x,y) return (x>310 and y>110 and y<400) and 1 or 0 end)
blend(warm:grow(8), {angle=1.3})
blend(warm:grow(8), {angle=0.4})
-- break the slab tip: meadow strokes come in over it
local tip = poly({{440,404},{590,412},{600,440},{430,452},{330,462},{150,468},{150,452},{400,436}}, true)
work(tip, {hand="body", pile=russet, coverage=0.9, angle=0.02, tool="filbert 10", length={25,60}, edge="lost"})
work(poly({{470,398},{600,408},{600,428},{480,424}},true), {hand="body", pile=grass, coverage=1.0, angle=0, tool="filbert 8", length={20,50}, edge="lost"})
blend(tip:grow(10), {angle=0})

--@ chunk 19
local right = tlM:times(function(x,y) return x>230 and 1 or 0 end)
blend(right, {angle=0.1})
blend(right, {angle=1.0})
local n5 = noise{seed=52, period=55}
local cl = tlM:shrink(4):times(function(x,y) return (n5(x,y) > 0.05) and 1 or 0 end)
stipple(cl, {pile=treeD, width=9, coverage=0.7, cluster={0.6,30}, feather=0.5, drag=3})
local cl2 = tlM:shrink(10):times(function(x,y) return (n5(x+90,y+40) > 0.3 and x>200) and 1 or 0 end)
stipple(cl2, {pile=treeM, width=7, coverage=0.5, cluster={0.6,24}, feather=0.5, drag=2})
-- ragged leafy edge on the sky side
local fringe = (tlM:grow(9) - tlM:shrink(8)):times(function(x,y) return (n5(x*1.5,y*1.5) > -0.1 and y<395) and 1 or 0 end)
stipple(fringe, {pile=treeD, width=4, coverage=0.5, cluster={0.7,18}, feather=0.6})

--@ chunk 20
blend(rect(200,50,60,390)*tlM, {angle=0.0})
blend(rect(190,50,80,390)*tlM, {angle=0.7})
wait(4*24*60)
print(drying(850,150), drying(250,250), drying(500,480), drying(380,250))

--@ chunk 21
tb = brush{kind="round", width=7, point=0.7}
local function trunk(pts, p0, p1, l) tb:load(trk, l or 0.9); tb:stroke(pts, {pressure={p0,p1}}) end
trunk({{846,100},{838,150},{829,220},{821,300},{825,380},{821,440}}, 0.12, 0.8)
trunk({{897,232},{886,262},{875,320},{878,380},{884,436}}, 0.1, 0.6)
trunk({{790,168},{802,208},{826,252}}, 0.06, 0.3, 0.6)
trunk({{882,118},{866,152},{833,192}}, 0.06, 0.3, 0.6)
trunk({{920,248},{905,270},{876,302}}, 0.06, 0.25, 0.6)
trunk({{864,248},{850,272},{823,302}}, 0.06, 0.25, 0.6)
local n3 = noise{seed=21, period=38}
local tr = trA + trB
local function holes(x,y) return n3(x,y) > 0.0 and 1 or 0 end
leafV = pile{{"raw umber",2.5},{"raw sienna",2},{"Antwerp blue",0.3},{"bone black",0.6},{"red earth",0.6},{"lead white",0.6}, medium=0.45}
work(tr:times(holes), {hand="scumble", pile=leafV, coverage=0.9, tool="filbert 7", length={6,14}, edge="lost", load=0.45})
stipple(tr:grow(8):times(function(x,y) return n3(x,y) > -0.2 and 1 or 0 end), {pile=leafV, width=4, coverage=0.3, cluster={0.8,20}, feather=0.7})
stipple(tr:shrink(5):times(function(x,y) return (n3(x+50,y) > 0.25 and x < 880) and 1 or 0 end), {pile=treeW, width=4, coverage=0.4, cluster=0.6, feather=0.5})

--@ chunk 22
local tr = trA + trB
local r2 = rag{width=9}
for i,p in ipairs{{815,150},{858,120},{880,170},{840,205},{900,150},{800,215},{895,265},{915,300},{870,300},{830,110},{862,225}} do
  r2:blot(p[1], p[2], {pressure=0.7}); if i%3==0 then r2:refold() end
end
blend(tr:grow(14), {angle=1.1})
local n6 = noise{seed=61, period=32}
stipple(tr:shrink(4):times(function(x,y) return n6(x,y) > 0.1 and 1 or 0 end), {pile=treeD, width=6, coverage=0.7, cluster={0.7,18}, feather=0.5, drag=2})
blend(tr:grow(10), {angle=0.5})
stipple(tr:grow(5):times(function(x,y) return n6(x+30,y) > 0.2 and 1 or 0 end), {pile=treeD, width=3, coverage=0.35, cluster={0.8,14}, feather=0.7})

--@ chunk 23
local tr = trA + trB
local r2 = rag{width=14}
r2:dip(0.5)
local n7 = noise{seed=71, period=40}
local k=0
for a=0,6.2,0.22 do
  -- around tree A rim
  local rx,ry = 82+12*n7(a*100,0), 98+12*n7(0,a*100)
  r2:blot(850+rx*math.cos(a), 160+ry*math.sin(a), {pressure=0.75})
  r2:blot(897+52*math.cos(a)*(1+0.15*n7(a*80,50)), 282+58*math.sin(a), {pressure=0.75})
  k=k+1; if k%4==0 then r2:refold(); r2:dip(0.5) end
end
for i,p in ipairs{{815,150},{858,118},{884,172},{842,208},{902,140},{800,205},{893,262},{918,300},{872,304},{828,108},{866,228},{790,170},{850,160},{910,200},{880,90},{905,320},{880,280}} do
  r2:blot(p[1]+rand(-4,4), p[2]+rand(-4,4), {pressure=0.8}); if i%3==0 then r2:refold(); r2:dip(0.5) end
end

--@ chunk 24
local tr = (trA + trB):grow(22)
local r3 = rag{width=34}
r3:dip(0.8)
r3:wipe(tr, {pressure=0.85, angle=0.05, passes=3, refold=0.3})
r3 = rag{width=34}; r3:dip(0.8)
r3:wipe(tr, {pressure=0.9, angle=0.6, passes=2, refold=0.3})

--@ chunk 25
local reg = poly({{725,30},{965,30},{985,200},{975,365},{735,365},{715,200}}, true)
local function d(x,y) local dx,dy=(x-600)/380,(y-372)/170; return math.sqrt(dx*dx+dy*dy) end
local function zone(a,b) return reg:times(function(x,y) local v=d(x,y); return (v>=a and v<b) and 1 or 0 end) end
local o = {hand="body", coverage=1.6, fill=true, tool="filbert 16", length={40,100}, edge="lost"}
local function lay(m, p, ang) o.pile=p; o.angle=ang; work(m, o) end
lay(zone(1.85,9), skyDk, 0.06)
lay(zone(1.4,1.95), skyG, 0.04)
lay(zone(0.95,1.5), skyRG, 0.02)
lay(zone(0.45,1.05), skyPeach, 0)
local big = poly({{640,0},{1000,0},{1000,385},{640,385}})
blend(big, {angle=0.04})
blend(big, {angle=-0.08})

--@ chunk 26
local function d(x,y) local dx,dy=(x-600)/380,(y-372)/170; return math.sqrt(dx*dx+dy*dy) end
local keep = tlM:shrink(2)
local notree = -keep
local function zone(a,b) return mask(function(x,y) local v=d(x,y); return (y<392 and v>=a and v<b) and 1 or 0 end) - keep end
local o = {hand="broad", coverage=1.4, fill=true, length={60,160}, clip=notree}
local function lay(m, p, ang) o.pile=p; o.angle=ang; work(m, o) end
lay(zone(1.85,9), skyDk, 0.06)
lay(zone(1.4,1.95), skyG, 0.04)
lay(zone(0.95,1.5), skyRG, 0.02)
lay(zone(0.45,1.05), skyPeach, 0)
lay(zone(0,0.5), skyGlow, 0)
local sky = above(function(x) return 392 end) - keep
blend(sky, {angle=0.04})
blend(sky, {angle=-0.08})
local far = (below(function(x) return 388 + 7*math.sin(x/70+1) + 4*math.sin(x/23) - 16*math.exp(-((x-700)/70)^2) - 10*math.exp(-((x-960)/80)^2) end) * above(function(x) return HZ+6 end)) - keep
work(far, {hand="body", pile=farD, coverage=1.4, angle=0, tool="filbert 8", length={15,40}, fill=true, clip=notree})
blend(ribbon({{440,386},{1000,386}}, 16), {angle=0})

--@ chunk 27
local notree = -(tlM:grow(6))
local st = poly({{440,170},{620,200},{620,300},{440,390},{430,300}}, true) - tlM:grow(6)
work(st:times(function(x,y) return y<290 and 1 or 0 end), {hand="body", pile=skyPeach, coverage=1.5, angle=0, tool="filbert 14", length={30,80}, clip=notree, fill=true})
work(st:times(function(x,y) return y>=280 and 1 or 0 end), {hand="body", pile=skyGlow, coverage=1.5, angle=0, tool="filbert 14", length={30,80}, clip=notree, fill=true})
local st2 = poly({{0,10},{560,10},{560,120},{330,90},{120,60},{0,250}}, true) - tlM:grow(6)
work(st2, {hand="body", pile=skyDk, coverage=1.4, angle=0.05, tool="filbert 16", length={40,100}, clip=notree, fill=true})
blend(st:grow(20) - tlM:grow(8), {angle=0})
blend(st2:grow(10) - tlM:grow(8), {angle=0.05})

--@ chunk 28
local sky = above(function(x) return 388 end) - tlM:grow(3)
blend(sky, {angle=0.03})
blend(sky, {angle=-0.1})
blend(sky, {angle=0.2})
blend(sky, {angle=0})

--@ chunk 29
wait(4*24*60)
print(drying(850,150), drying(250,250), drying(500,480), drying(500,250))

--@ chunk 30
tb = brush{kind="round", width=6, point=0.8}
local function trunk(pts, p0, p1, l) tb:load(trk, l or 0.9); tb:stroke(pts, {pressure={p0,p1}}) end
trunk({{846,100},{838,150},{829,220},{821,300},{825,380},{821,440}}, 0.1, 0.8)
trunk({{897,232},{886,262},{875,320},{878,380},{884,436}}, 0.1, 0.6)
trunk({{790,168},{802,208},{826,252}}, 0.05, 0.3, 0.6)
trunk({{882,118},{866,152},{833,192}}, 0.05, 0.3, 0.6)
trunk({{920,248},{905,270},{876,302}}, 0.05, 0.25, 0.6)
trunk({{864,248},{850,272},{823,302}}, 0.05, 0.25, 0.6)
trunk({{800,120},{815,160},{834,180}}, 0.05, 0.25, 0.6)
leafD = pile{{"raw umber",3},{"raw sienna",1.5},{"Antwerp blue",0.4},{"bone black",1},{"red earth",0.5}, medium=0.35}
leafW = pile{{"raw sienna",3},{"red earth",1},{"yellow ochre",1.5},{"orange chrome",0.6},{"raw umber",0.8}, medium=0.35}
local n3 = noise{seed=83, period=42}
local n4 = noise{seed=84, period=16}
local tr = trA + trB
local dens = function(x,y) return clamp((n3(x,y)+0.15)*2.2,0,1) * (0.5+0.5*n4:at01(x,y)) end
stipple(tr:shrink(3):times(dens), {pile=leafD, width=4.5, coverage=1.3, cluster={0.5,14}, feather=0.7, drag={3,0.4}, dips={10,0.6,0.5}})
stipple(tr:grow(10):times(dens), {pile=leafD, width=2.5, coverage=0.5, cluster={0.7,12}, feather=0.8, drag={2,0.2}, dips={10,0.5,0.5}})
stipple(tr:shrink(4):times(function(x,y) return x<880 and clamp((n3(x+40,y+20))*2,0,1) or 0 end), {pile=leafW, width=3, coverage=0.45, cluster={0.6,10}, feather=0.7, drag={2,0.3}})

--@ chunk 31
local n3 = noise{seed=83, period=42}
local n4 = noise{seed=84, period=16}
local tr = trA + trB
leafDD = pile{{"raw umber",3},{"bone black",1.5},{"Antwerp blue",0.4},{"raw sienna",0.8}, medium=0.2}
local core = function(x,y) return clamp((n3(x,y)-0.05)*3,0,1) * (0.4+0.6*n4:at01(x,y)) end
stipple(tr:shrink(5):times(core), {pile=leafDD, width=5, coverage=1.6, cluster={0.5,12}, feather=0.6, drag={3,0.5}, dips={8,0.7,0.5}})
-- big tree: leafy fringe over halo, on dry sky
local n5 = noise{seed=91, period=30}
local n6 = noise{seed=92, period=11}
local fr = (tlM:grow(13) - tlM:shrink(10)):times(function(x,y) return y<400 and clamp((n5(x,y)+0.35)*1.8,0,1)*(0.4+0.6*n6:at01(x,y)) or 0 end)
stipple(fr, {pile=leafDD, width=5, coverage=1.5, cluster={0.5,14}, feather=0.6, drag={3,0.6}, dips={8,0.7,0.5}})
local fr2 = (tlM:grow(24) - tlM:grow(6)):times(function(x,y) return y<400 and clamp((n5(x,y)+0.0)*2,0,1)*n6:at01(x,y) or 0 end)
stipple(fr2, {pile=leafD, width=3, coverage=0.6, cluster={0.7,10}, feather=0.8, drag={2,0.3}})
-- sky holes / lights through foliage near the sunward edge and warm leaves
local warm = tlM:rim(46, 12):times(function(x,y) return (x>300 and y>150 and y<395) and clamp((n5(x+70,y)+0.1)*2,0,1)*n6:at01(x,y) or 0 end)
stipple(warm, {pile=leafW, width=4, coverage=0.7, cluster={0.6,12}, feather=0.7, drag={2,0.5}})
-- inner modeling: a few dark pockets and olive lights
local inn = tlM:shrink(18):times(function(x,y) return clamp((n5(x*0.5,y*0.5+40)-0.1)*2.5,0,1) end)
stipple(inn, {pile=leafDD, width=9, coverage=0.8, cluster={0.5,25}, feather=0.6, drag={4,0.7}})
local inl = tlM:shrink(18):times(function(x,y) return x>170 and clamp((n5(x*0.5+200,y*0.5)-0.2)*2.5,0,1) or 0 end)
stipple(inl, {pile=treeM, width=7, coverage=0.5, cluster={0.5,20}, feather=0.6, drag={3,0.7}})

--@ chunk 32
farW = pile{{"lead white",3},{"cobalt blue",1.2},{"Indian red",0.9},{"raw umber",1.6},{"yellow ochre",1}, medium=0.2}
farW2 = pile{{"lead white",1.5},{"cobalt blue",1},{"Indian red",0.7},{"raw umber",2.2},{"raw sienna",1}, medium=0.2}
-- far wooded ridge with an uneven crown line
local ridge = outline{{430,400},{470,386},{520,382},{560,388},{610,384},{660,374},{700,366},{740,372},{780,384},{830,380},{880,372},{930,366},{970,370},{1000,374},{1000,424},{430,424}, char="soft", seed=17, lobe=16, closed=true}:mask()
work(ridge, {hand="body", pile=farW, coverage=1.5, angle=0, tool="filbert 7", length={10,28}, fill=true, edge="soft"})
-- nearer hedge line, darker, along the meadow edge
local hedge = outline{{420,412},{480,404},{540,408},{600,412},{690,410},{760,402},{800,398},{850,404},{920,398},{1000,392},{1000,430},{420,430}, char="soft", seed=19, lobe=12, closed=true}:mask()
work(hedge, {hand="body", pile=farW2, coverage=1.3, angle=0, tool="filbert 6", length={8,22}, fill=true, edge="soft"})
blend(ribbon({{430,378},{700,362},{1000,366}}, 18), {angle=0})
blend(ribbon({{420,424},{1000,424}}, 14), {angle=0})
-- sun: pale disc low in haze with a warm halo
sunH = pile{{"lead white",8},{"lemon chrome",2},{"orange chrome",1.2}, medium=0.5}
sunC = pile{{"lead white",10},{"lemon chrome",1.5},{"orange chrome",0.3}}
work(ellipse(612,340,70,38), {hand="scumble", pile=sunH, coverage=0.6, tool="filbert 12", edge="lost", load=0.4})
blend(ellipse(612,340,95,55), {angle=0})
local sb = brush("round", 9)
sb:load(sunC, 0.9)
sb:touch(612,342,{pressure=0.8})
sb:touch(612,342,{pressure=0.6})

--@ chunk 33
blend(ellipse(612,335,150,80):soften(60), {angle=0})
blend(ellipse(612,335,170,90):soften(70), {angle=0.5})
-- ridge darker and warmer, overlapping hedge; keep the top soft
local ridge = outline{{425,402},{470,388},{520,384},{560,390},{610,386},{660,377},{700,370},{740,375},{780,386},{830,382},{880,375},{930,369},{970,372},{1000,376},{1000,430},{425,430}, char="soft", seed=17, lobe=16, closed=true}:mask()
work(ridge, {hand="body", pile=farW2, coverage=1.2, angle=0, tool="filbert 7", length={10,28}, fill=true, clip=ridge:soften(6)})
blend(ribbon({{420,400},{1000,400}}, 70):soften(20), {angle=0})

--@ chunk 34
local r4 = rag{width=30}; r4:dip(0.8)
local reg = poly({{440,255},{800,255},{800,372},{440,385}}) - tlM:grow(14)
r4:wipe(reg, {pressure=0.85, angle=0, passes=3, refold=0.3})

--@ chunk 35
local notree = -(tlM:grow(2))
-- sky back down to the ridge crest where slab rose too high
local crest = function(x) return 386 - 10*math.exp(-((x-700)/60)^2) - 6*math.exp(-((x-930)/50)^2) + 4*math.sin(x/31) - 5*math.exp(-((x-520)/40)^2) end
local skyfix = (above(crest) * below(function(x) return 345 end)):times(function(x,y) return x>430 and 1 or 0 end)
work(skyfix, {hand="body", pile=skyGlow, coverage=1.5, angle=0, tool="filbert 10", length={25,60}, fill=true, clip=above(crest)*notree})
blend(skyfix:grow(8)*above(crest), {angle=0})
-- meadow comes up to its far edge at ~414
local mead = below(function(x) return 414 + 3*math.sin(x/45) end) * above(function(x) return 450 end)
work(mead:times(function(x,y) return x>300 and 1 or 0 end), {hand="body", pile=grass, coverage=1.4, angle=0, tool="filbert 8", length={20,50}, fill=true})
work(ribbon({{480,424},{620,421},{770,426}}, {3,9,3}), {hand="body", pile=gold, coverage=1.1, angle=0, tool="filbert 6", length={20,45}})
-- dark hedge at the foot of the ridge, joining the big tree
local hedge = outline{{300,400},{380,394},{440,398},{500,404},{560,402},{640,407},{720,403},{790,397},{850,403},{930,399},{1000,395},{1000,418},{300,420}, char="soft", seed=23, lobe=12, closed=true}:mask()
hedgeC = pile{{"raw umber",3},{"cobalt blue",0.8},{"Indian red",0.5},{"raw sienna",1.2},{"lead white",0.8}, medium=0.2}
work(hedge, {hand="body", pile=hedgeC, coverage=1.2, angle=0, tool="filbert 6", length={8,20}, edge="soft"})
work(poly({{300,392},{440,396},{470,420},{300,428}},true), {hand="body", pile=treeD, coverage=1.3, tool="filbert 8", length={10,25}, edge="soft"})

--@ chunk 36
local crest = function(x) return 386 - 10*math.exp(-((x-700)/60)^2) - 6*math.exp(-((x-930)/50)^2) + 4*math.sin(x/31) - 5*math.exp(-((x-520)/40)^2) end
local notree = -(tlM:grow(2))
local up = above(crest) * notree
skyGP = pile{{"lead white",10},{"lemon chrome",1.2},{"yellow ochre",1.4},{"orange chrome",0.5},{"red earth",0.15}, medium=0.15}
local z1 = mask(function(x,y) return (x>425 and y>270 and y<345) and 1 or 0 end)
work(z1, {hand="broad", pile=skyGP, coverage=1.3, angle=0, length={60,150}, fill=true, clip=up, edge="lost"})
local z = mask(function(x,y) return (x>400 and y>230) and 1 or 0 end) * up
blend(z, {angle=0.03})
blend(z, {angle=-0.06})
-- ridge: soften into haze; hedge and meadow softened horizontally
local low = mask(function(x,y) return (x>300 and y>372 and y<460) and 1 or 0 end)
blend(low, {angle=0})
-- retouch trunks
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{829,220},{821,300},{825,380},{821,436}}, {pressure={0.5,0.8}})
tb:load(trk, 0.9); tb:stroke({{886,262},{875,320},{878,380},{884,432}}, {pressure={0.35,0.6}})

--@ chunk 37
wait(5*24*60)
print(drying(850,150), drying(600,400), drying(600,440), drying(600,320))

--@ chunk 38
ridgeC = pile{{"lead white",4},{"cobalt blue",1.3},{"Indian red",0.8},{"raw umber",1.3},{"yellow ochre",1.2}, medium=0.2}
ridgeL = pile{{"lead white",6},{"cobalt blue",0.8},{"Indian red",0.6},{"yellow ochre",2},{"orange chrome",0.4},{"raw umber",0.5}, medium=0.2}
local crest = {{400,392},{450,384},{500,380},{540,384},{590,381},{650,373},{700,368},{745,373},{790,382},{840,379},{890,372},{935,368},{975,371},{1000,373}}
local pts = {}
for i,p in ipairs(crest) do pts[#pts+1]=p end
pts[#pts+1]={1000,416}; pts[#pts+1]={400,416}
local ridge = outline{pts=pts, char="soft", seed=27, lobe=14, closed=true}:mask()
-- cover sky remnants of the rectangle top (372) first: ridge covers from crest down; the strip between crest and 372? crest is above/below 372, so add sky strip
local strip = mask(function(x,y) return (x>296 and y>362 and y<392) and 1 or 0 end) - tlM
work(strip, {hand="body", pile=skyGP, coverage=1.4, angle=0, tool="filbert 8", length={25,60}, fill=true, edge="lost", clip=-tlM})
work(ridge, {hand="body", pile=ridgeC, coverage=1.5, angle=0, tool="filbert 7", length={10,26}, fill=true, edge="soft"})
work(ridge * ellipse(620,392,150,40), {hand="scumble", pile=ridgeL, coverage=0.9, tool="filbert 7", edge="lost"})
print("ridge")

--@ chunk 39
local band = mask(function(x,y) return (x>385 and y>360 and y<418) and 1 or 0 end)
work(band * ellipse(620,396,170,30), {hand="body", pile=ridgeC, coverage=0.8, angle=0, tool="filbert 7", length={10,26}})
blend(band, {angle=0})
-- big tree lower right restated, with base
local low = tlM:times(function(x,y) return (x>250 and y>330) and 1 or 0 end) + poly({{280,380},{400,384},{450,402},{470,424},{280,436}}, true)
work(low, {hand="body", pile=treeD, coverage=1.5, tool="filbert 10", length={12,30}, fill=true, edge="soft"})
-- far meadow band reworked: broken strokes of several colors over the flat strip, no blending
local mb = mask(function(x,y) return (x>290 and y>414 and y<470) and 1 or 0 end)
local n1 = noise{seed=101, period=120}
work(mb:times(function(x,y) return n1(x*0.3,y*1.5)>0.0 and 1 or 0 end), {hand="body", pile=russet, coverage=1.0, angle=0, tool="filbert 8", length={25,70}, edge="lost"})
work(mb:times(function(x,y) return n1(x*0.3+300,y*1.5)>0.25 and 1 or 0 end), {hand="body", pile=grassD, coverage=0.9, angle=0.02, tool="filbert 8", length={25,70}, edge="lost"})
work(ribbon({{500,424},{620,421},{760,425}}, {3,8,3}) + ribbon({{520,446},{640,442},{760,450}},{2,7,2}), {hand="body", pile=gold, coverage=1.1, angle=0, tool="filbert 6", length={20,45}})
-- shadow thrown by the tree along the meadow
work(poly({{150,430},{440,422},{560,430},{430,450},{150,462}},true), {hand="body", pile=deep, coverage=1.1, angle=0.02, tool="filbert 10", length={30,70}, edge="lost"})

--@ chunk 40
local mb = mask(function(x,y) return (y>426 and y<500) and 1 or 0 end):soften(10)
blend(mb, {angle=0})
blend(tlM:times(function(x,y) return (x>250 and y>325 and y<395) and 1 or 0 end):soften(8), {angle=0.8})
print(drying(880,260), drying(840,300))
-- right trees: trunks & foliage again
tb = brush{kind="round", width=6, point=0.8}
local function trunk(pts, p0, p1, l) tb:load(trk, l or 0.9); tb:stroke(pts, {pressure={p0,p1}}) end
trunk({{846,100},{838,150},{829,220},{821,300},{825,380},{821,432}}, 0.1, 0.8)
trunk({{897,232},{886,262},{875,320},{878,380},{884,428}}, 0.1, 0.6)
trunk({{920,248},{905,270},{876,302}}, 0.05, 0.25, 0.6)
trunk({{864,248},{850,272},{823,302}}, 0.05, 0.25, 0.6)
local n3 = noise{seed=83, period=42}
local n4 = noise{seed=84, period=16}
local trb = trB
local dens = function(x,y) return clamp((n3(x,y)+0.15)*2.2,0,1) * (0.5+0.5*n4:at01(x,y)) end
stipple(trb:shrink(3):times(dens), {pile=leafD, width=4.5, coverage=1.3, cluster={0.5,14}, feather=0.7, drag={3,0.4}, dips={10,0.6,0.5}})
stipple(trb:shrink(5):times(function(x,y) return clamp((n3(x,y)-0.05)*3,0,1) * (0.4+0.6*n4:at01(x,y)) end), {pile=leafDD, width=5, coverage=1.5, cluster={0.5,12}, feather=0.6, drag={3,0.5}, dips={8,0.7,0.5}})
stipple(trb:grow(10):times(dens), {pile=leafD, width=2.5, coverage=0.5, cluster={0.7,12}, feather=0.8, drag={2,0.2}})
-- a few lower sprays on tree A
local lowA = ellipse(815,235,35,22) + ellipse(868,225,26,16)
stipple(lowA:times(dens), {pile=leafD, width=3.5, coverage=1.0, cluster={0.6,12}, feather=0.7, drag={3,0.3}})

--@ chunk 41
wait(5*24*60)
print(drying(600,400), drying(600,440), drying(350,360), drying(500,550))

--@ chunk 42
wait(4*24*60)
-- 1. deep glaze over big tree's shadow masses
shadeGl = pile{{"raw umber",2},{"bone black",1.2},{"Antwerp blue",0.5},{"bitumen",1}, medium=0.6}
local n2 = noise{seed=111, period=90}
local shade = tlM:shrink(8):times(function(x,y)
  local v = 0.5 - (x-240)/400 + (y-250)/350 + 0.6*n2(x,y)
  return clamp(v*1.5,0,1) end)
work(shade, {hand="glaze", pile=shadeGl, tool="filbert 20", length={30,70}, coverage=1.0, clip=tlM, load=0.5})
-- undergrowth base
local base = outline{{0,392},{90,384},{200,390},{300,386},{390,392},{450,404},{500,418},{420,432},{250,440},{0,446}, char="soft", seed=29, lobe=20, closed=true}:mask()
work(base, {hand="body", pile=deep, coverage=1.3, tool="filbert 10", length={15,40}, angle=0, edge="lost"})
-- far hedge across foot of ridge: broken, dark, soft
local n8 = noise{seed=120, period=60}
local hedge = ribbon({{470,414},{560,411},{650,414},{740,410},{830,412},{920,408},{1000,409}}, {6,9,7,10,12,10,12}):times(function(x,y) return clamp((n8(x,y*3)+0.45)*2,0,1) end)
work(hedge, {hand="scumble", pile=hedgeC, coverage=1.0, tool="filbert 5", length={6,14}, edge="soft"})
-- trunks through mist
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{823,330},{825,380},{821,432}}, {pressure={0.6,0.8}})
tb:load(trk, 0.9); tb:stroke({{875,330},{878,380},{884,428}}, {pressure={0.45,0.6}})

--@ chunk 43
local inner = tlM:shrink(3):times(function(x,y) return y<392 and 1 or 0 end)
blend(inner, {angle=0.6})
blend(inner, {angle=-0.7})
blend(inner, {angle=1.5})
local base = mask(function(x,y) return (x<520 and y>380 and y<450) and 1 or 0 end):soften(8)
blend(base, {angle=0})
blend(base, {angle=0.2})

--@ chunk 44
local n5 = noise{seed=131, period=40}
local seam = outline{{0,360},{150,355},{300,360},{420,366},{470,392},{530,420},{440,438},{300,446},{120,444},{0,450}, char="soft", seed=31, lobe=22, closed=true}:mask()
work(seam, {hand="body", pile=treeD, coverage=1.5, tool="filbert 10", length={12,30}, angle=function(x,y) return 0.3*n5(x,y) end, edge={found=0.1,soft=0.4,lost=0.5,period=40}, fill=true})
stipple(seam:times(function(x,y) return clamp(n5(x,y)*2+0.2,0,1) end), {pile=leafDD, width=8, coverage=0.8, cluster={0.5,20}, feather=0.6, drag={4,0.2}})
-- leafy clumps on the lower right flank of the tree, against sky and ridge
local n6 = noise{seed=92, period=11}
local flank = (tlM:grow(10) - tlM:shrink(12)):times(function(x,y) return (x>330 and y>230 and y<410) and clamp((n5(x,y)+0.3)*1.8,0,1)*(0.4+0.6*n6:at01(x,y)) or 0 end)
stipple(flank, {pile=leafDD, width=5, coverage=1.4, cluster={0.5,14}, feather=0.6, drag={3,0.6}})
stipple(ellipse(455,400,40,14):times(function(x,y) return n6:at01(x,y) end), {pile=leafDD, width=4, coverage=0.9, cluster={0.6,12}, feather=0.7, drag={2,0.3}})
-- warm lit leaves on the sunward flank
local warm = tlM:rim(40, 12):times(function(x,y) return (x>320 and y>150 and y<380) and clamp((n5(x+70,y)+0.1)*2,0,1)*n6:at01(x,y) or 0 end)
stipple(warm, {pile=leafW, width=4, coverage=0.8, cluster={0.6,12}, feather=0.7, drag={2,0.5}})

--@ chunk 45
local n5 = noise{seed=141, period=50}
local seam = outline{{0,345},{150,340},{300,345},{420,360},{475,395},{525,420},{440,436},{300,444},{120,442},{0,448}, char="soft", seed=31, lobe=22, closed=true}:mask()
deepG = pile{{"raw umber",3},{"bone black",1.5},{"Antwerp blue",0.5},{"raw sienna",1}, medium=0.25}
work(seam:times(function(x,y) return clamp(0.75+0.5*n5(x,y),0,1) end), {hand="body", pile=deepG, coverage=1.4, tool="filbert 12", length={15,35}, angle=function(x,y) return 1.2+0.5*n5(x,y) end, edge="lost"})
local j = mask(function(x,y) return (x<540 and y>300 and y<440) and 1 or 0 end) * (tlM + seam)
blend(j, {angle=1.2})
blend(j, {angle=0.3})

--@ chunk 46
local all = (tlM:shrink(4) + mask(function(x,y) return (x<500 and y>340 and y<430) and 1 or 0 end)):soften(6)
blend(all:times(function(x,y) return y<436 and 1 or 0 end), {angle=1.3})
blend(mask(function(x,y) return (x<470 and y>260 and y<350) and 1 or 0 end):soften(20) * tlM:shrink(6), {angle=0.2})
-- foreground grass strokes cover the drip marks at the base
local foot = mask(function(x,y) return (x<560 and y>428 and y<485) and 1 or 0 end)
work(foot, {hand="body", pile=deep, coverage=1.0, angle=0.02, tool="filbert 10", length={30,70}, edge="lost"})
work(foot:times(function(x,y) return y>450 and 1 or 0 end), {hand="body", pile=russet, coverage=0.6, angle=0.0, tool="filbert 8", length={25,60}, edge="lost"})

--@ chunk 47
local r5 = rag{width=16}; r5:dip(0.8)
r5:wipe({{508,343},{470,345},{448,349}}, {pressure=0.85}); r5:refold(); r5:dip(0.8)
r5:wipe({{508,356},{472,357},{452,360}}, {pressure=0.85}); r5:refold(); r5:dip(0.8)
r5:wipe({{505,349},{460,352}}, {pressure=0.9})
local foot = mask(function(x,y) return (x<600 and y>436 and y<495) and 1 or 0 end):soften(10)
blend(foot, {angle=0})
blend(foot, {angle=0.05})

--@ chunk 48
local n1 = noise{seed=151, period=140}
local n2 = noise{seed=152, period=70}
seamM = outline{{0,345},{150,340},{300,345},{420,360},{475,395},{525,420},{440,436},{300,444},{120,442},{0,448}, char="soft", seed=31, lobe=22, closed=true}:mask()
local fg = mask(function(x,y) return y>424 and 1 or 0 end) - seamM - poolM:grow(2)
local function patch(off, th, y0) return fg:times(function(x,y) return (y>y0 and n1(x*0.3+off, y*1.4) + 0.4*n2(x*0.5+off,y) > th) and 1 or 0 end) end
local o = {hand="body", angle=0.02, tool="filbert 12", length={35,100}, edge="lost", coverage=1.0}
o.pile=grass; work(patch(0,0.1,424), o)
o.pile=russet; work(patch(300,0.05,430), o)
o.pile=deep; work(fg:times(function(x,y)
  local t=(y-424)/(H-424); local side=math.max(0,1-math.abs(x-570)/470)
  return (t*1.2 - side*0.55 + 0.3*n1(x*0.3+700,y*1.4) > 0.5) and 1 or 0 end), o)
-- tree's shadow across the meadow at left
o.pile=deep; work(poly({{0,440},{440,434},{590,440},{470,462},{250,476},{0,486}},true) - seamM, o)
blend(fg, {angle=0.01})
-- gold lights last, unblended
o.pile=gold; o.tool="filbert 6"; o.length={20,50}
work(ribbon({{520,432},{640,429},{770,433}}, {2,7,2}) + ribbon({{540,468},{640,463},{750,472}},{2,8,2}) + ribbon({{590,500},{650,497},{720,503}},{2,5,2}), o)

--@ chunk 49
blend(mask(function(x,y) return (x>480 and x<800 and y>424 and y<512) and 1 or 0 end):soften(10), {angle=0})
-- pool restated
poolM = outline{{400,548},{470,524},{590,518},{680,530},{700,552},{640,574},{540,588},{440,580}, char="soft", seed=4, closed=true}:mask()
poolR = pile{{"lead white",8},{"yellow ochre",1.8},{"lemon chrome",0.8},{"orange chrome",0.4},{"cobalt blue",0.5},{"raw umber",0.4}, medium=0.15}
work(poolM, {hand="body", pile=poolR, coverage=1.5, angle=0, tool="filbert 8", length={25,60}, clip=poolM:soften(4), fill=true})
work(poly({{480,538},{610,530},{650,546},{540,560}},true), {hand="body", pile=skyGP, coverage=1.0, angle=0, tool="filbert 6", length={20,45}, clip=poolM})
-- dark bank reflections at near edge
work(poolM:rim(10,4):times(function(x,y) return y>560 and 1 or 0 end), {hand="body", pile=deep, coverage=0.9, angle=0, tool="filbert 5", length={10,25}, clip=poolM:grow(4)})
blend(poolM:shrink(3), {angle=0})
wait(6*24*60)
print(drying(300,300), drying(300,520), drying(560,550), drying(100,400))

--@ chunk 50
gl1 = pile{{"raw umber",2},{"bone black",1},{"bitumen",1.5},{"raw sienna",1}, medium=0.75}
local fg = mask(function(x,y) return y>428 and 1 or 0 end)
local function vig(th) return fg:times(function(x,y)
  local t=(y-428)/(H-428); local side=math.abs(x-590)/500
  return (t*0.9 + side*side*0.9 > th) and 1 or 0 end) - poolM:grow(4) end
work(vig(0.25), {hand="glaze", pile=gl1, clip=fg - poolM:grow(2), angle=0.02})
work(vig(0.6), {hand="glaze", pile=gl1, clip=fg - poolM:grow(2), angle=-0.03})
work(vig(0.95), {hand="glaze", pile=gl1, clip=fg, angle=0.02})
-- pool: warm glaze, then ground dragged over its edges
gl2 = pile{{"raw sienna",2},{"yellow ochre",1},{"raw umber",0.6}, medium=0.8}
work(poolM, {hand="glaze", pile=gl2, clip=poolM:grow(3), angle=0, length={40,90}})

--@ chunk 51
local rp = rag{width=24}; rp:dip(0.6)
rp:wipe(poolM:shrink(2), {pressure=0.8, angle=0, passes=2, refold=0.3})
local rg = rag{width=45}
local fg = mask(function(x,y) return y>430 and 1 or 0 end) - poolM:grow(6)
rg:wipe(fg, {pressure=0.5, angle=0.02, passes=1, refold=0.4})
local centre = fg:times(function(x,y) local t=(y-428)/(H-428); local side=math.abs(x-590)/500; return (t*0.9 + side*side*0.9 < 0.6) and 1 or 0 end)
rg = rag{width=45}
rg:wipe(centre, {pressure=0.7, angle=-0.02, passes=2, refold=0.4})

--@ chunk 52
local rp = rag{width=20}; rp:dip(0.7)
rp:wipe(poolM:shrink(4), {pressure=0.85, angle=0, passes=2, refold=0.25})
local rg = rag{width=26}
rg:wipe({{560,436},{700,432},{850,436},{1000,432}}, {pressure=0.6}); rg:refold()
rg:wipe({{0,440},{200,442},{420,438}}, {pressure=0.4}); rg:refold()
rg:wipe({{620,452},{800,446},{990,452}}, {pressure=0.45})
local fg = mask(function(x,y) return y>432 and 1 or 0 end) - poolM:grow(5)
blend(fg, {angle=0.02})
blend(poolM:shrink(3), {angle=0})

--@ chunk 53
bank = pile{{"raw umber",3},{"raw sienna",1.5},{"bone black",0.8},{"yellow ochre",0.8},{"Antwerp blue",0.2}, medium=0.2}
bankL = pile{{"raw sienna",2},{"yellow ochre",2},{"raw umber",1.5},{"lead white",0.5},{"Antwerp blue",0.15}, medium=0.2}
-- near bank overlaps the pool's lower edge; far bank its upper edge
local near = outline{{380,566},{440,572},{520,580},{600,572},{680,556},{720,548},{730,575},{640,600},{520,606},{400,596}, char="soft", seed=37, lobe=14, closed=true}:mask()
work(near, {hand="body", pile=bank, coverage=1.4, angle=0.0, tool="filbert 7", length={15,40}, edge="soft", fill=true})
local farb = outline{{380,552},{400,534},{470,518},{560,512},{650,518},{712,534},{720,548},{690,538},{600,524},{500,526},{430,540},{400,556}, char="soft", seed=39, lobe=12, closed=true}:mask()
work(farb, {hand="body", pile=bankL, coverage=1.3, angle=0.0, tool="filbert 6", length={15,35}, edge="soft"})
-- a few reeds / grass tufts at the near bank, dark verticals
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(16, 410, 520, 0.6, 0.6, 5)) do
  rb:load(bank, 0.5)
  local h = rand(10,22)
  rb:stroke({{x, 584+rand(-4,4)},{x+rand(-3,3), 584-h}}, {pressure={0.5,0.05}})
end
for i,x in ipairs(uneven(8, 660, 715, 0.6, 0.6, 6)) do
  rb:load(bank, 0.5)
  rb:stroke({{x, 560+rand(-3,3)},{x+rand(-3,3), 560-rand(8,16)}}, {pressure={0.45,0.05}})
end

--@ chunk 54
local water = poly({{420,552},{480,532},{590,526},{690,540},{640,562},{540,572},{450,566}}, true)
local ring = ellipse(552,556,215,75) - water:grow(3)
blend(ring, {angle=0})
blend(ring, {angle=0.05})
blend(ring, {angle=-0.05})

--@ chunk 55
local water = poly({{420,552},{480,532},{590,526},{690,540},{640,562},{540,572},{450,566}}, true)
local n1 = noise{seed=161, period=110}
local zone = ellipse(552,560,260,95):roughen(25,90,7) - water:grow(2)
local o = {hand="body", angle=0.0, tool="filbert 10", length={40,110}, edge="lost", coverage=0.9, clip=-(water:grow(1))}
o.pile=bank; work(zone:times(function(x,y) return y>552 and 1 or 0 end), o)
o.pile=russet; work(zone:times(function(x,y) return (y<=556 and n1(x*0.4,y*2)>-0.2) and 1 or 0 end), o)
o.pile=bankL; o.coverage=0.6; work(zone:times(function(x,y) return (y<540 and n1(x*0.4+200,y*2)>0.1) and 1 or 0 end), o)
o.pile=deep; o.coverage=0.8; work(zone:times(function(x,y) return (y>585) and 1 or 0 end), o)

--@ chunk 56
local water = poly({{420,552},{480,532},{590,526},{690,540},{640,562},{540,572},{450,566}}, true)
local fg = mask(function(x,y) return y>440 and 1 or 0 end) - water:grow(3)
blend(fg, {angle=0.0})
blend(fg, {angle=0.03})
-- deepen foreground bottom with strokes not glaze
local n1 = noise{seed=171, period=120}
work(fg:times(function(x,y) local t=(y-440)/(H-440); local side=math.abs(x-560)/500; return (t + side*side*0.7 + 0.25*n1(x*0.3,y*1.5) > 0.62) and 1 or 0 end),
  {hand="body", pile=deep, angle=0.02, tool="filbert 14", length={40,110}, edge="lost", coverage=1.0, clip=-(water:grow(1))})
blend(fg, {angle=-0.02})

--@ chunk 57
wait(7*24*60)
print(drying(300,300), drying(300,520), drying(560,600), drying(100,400), drying(600,400))

--@ chunk 58
local notree = -(tlM:grow(1) + seamM)
local strip = mask(function(x,y) return (x>400 and y>325 and y<400) and 1 or 0 end)
work(strip, {hand="body", pile=skyGP, coverage=1.6, angle=0, tool="filbert 12", length={40,110}, fill=true, edge="lost", clip=notree})
local crest = {{400,394},{450,388},{500,385},{540,388},{590,386},{650,380},{700,376},{745,380},{790,387},{840,384},{890,379},{935,376},{975,378},{1000,380}}
local pts = {}
for i,p in ipairs(crest) do pts[#pts+1]=p end
pts[#pts+1]={1000,416}; pts[#pts+1]={400,416}
local ridge = outline{pts=pts, char="soft", seed=27, lobe=14, closed=true}:mask()
work(ridge, {hand="body", pile=ridgeC, coverage=1.5, angle=0, tool="filbert 7", length={12,30}, fill=true, clip=notree})
-- haze of light where the sun sits over the ridge
work(ridge * ellipse(620,396,120,22), {hand="body", pile=ridgeL, coverage=0.8, angle=0, tool="filbert 6", length={12,30}})
-- hedge clumps
local n8 = noise{seed=120, period=50}
local hedge = ribbon({{440,410},{560,408},{650,411},{740,407},{830,409},{920,405},{1000,406}}, {8,10,8,11,13,11,13}):times(function(x,y) return clamp((n8(x,y*3)+0.35)*2,0,1) end)
work(hedge, {hand="body", pile=hedgeC, coverage=1.1, tool="filbert 5", length={6,16}, angle=0, clip=notree})
-- far meadow
local mead = mask(function(x,y) return (x>330 and y>414 and y<456) and 1 or 0 end)
work(mead, {hand="body", pile=grass, coverage=1.2, angle=0, tool="filbert 8", length={30,90}, edge="lost", clip=-seamM})
work(ribbon({{520,422},{640,420},{770,423}}, {2,6,2}) + ribbon({{540,440},{650,436},{760,442}},{2,6,2}), {hand="body", pile=gold, coverage=1.0, angle=0, tool="filbert 5", length={20,50}})
-- soft horizontal fusing with soft-edged masks only
blend(ribbon({{380,386},{1000,380}}, 30):soften(12) * notree, {angle=0})
blend(ribbon({{330,432},{1000,432}}, 50):soften(16) - seamM, {angle=0})

--@ chunk 59
local sk = mask(function(x,y) return (x>380 and y>270 and y<376) and 1 or 0 end):soften(25)
blend(sk, {angle=1.5})
blend(sk, {angle=0})
-- meadow: break the flat green with russet and deep strokes, and cover notch at tree base
local n1 = noise{seed=181, period=120}
local mead = mask(function(x,y) return (x>300 and y>412 and y<470) and 1 or 0 end)
local o = {hand="body", angle=0, tool="filbert 8", length={30,90}, edge="lost", coverage=0.9}
o.pile=russet; work(mead:times(function(x,y) return n1(x*0.3,y*2)>0.0 and 1 or 0 end), o)
o.pile=deep; work(poly({{280,418},{470,414},{560,424},{600,440},{450,456},{280,462}},true), o)
o.pile=bank; work(mead:times(function(x,y) return (y>440 and n1(x*0.3+500,y*2)>0.1) and 1 or 0 end), o)
blend(mask(function(x,y) return (y>418 and y<480) and 1 or 0 end):soften(14), {angle=0})

--@ chunk 60
local rr = rag{width=18}; rr:dip(0.8)
local flank = (tlM + seamM):times(function(x,y) return (x>340 and x<500 and y>250 and y<412) and 1 or 0 end):shrink(3)
rr:wipe(flank, {pressure=0.85, angle=1.2, passes=2, refold=0.25})
rr = rag{width=14}; rr:dip(0.7)
rr:wipe(trB:times(function(x,y) return y>255 and 1 or 0 end), {pressure=0.7, angle=0.3, passes=1, refold=0.25})

--@ chunk 61
wait(6*24*60)
print(drying(600,350), drying(420,330), drying(600,440))
-- tree flank
local flank = (tlM + seamM):times(function(x,y) return (x>330 and x<510 and y>240 and y<420) and 1 or 0 end)
work(flank, {hand="body", pile=treeD, coverage=1.5, tool="filbert 9", length={12,28}, angle=1.1, fill=true, edge="soft"})
local n5 = noise{seed=191, period=36}
local n6 = noise{seed=192, period=11}
local fr = ((tlM+seamM):grow(11) - (tlM+seamM):shrink(10)):times(function(x,y) return (x>340 and y>215 and y<408) and clamp((n5(x,y)+0.3)*1.8,0,1)*(0.4+0.6*n6:at01(x,y)) or 0 end)
stipple(fr, {pile=leafDD, width=5, coverage=1.4, cluster={0.5,14}, feather=0.6, drag={3,0.6}})
-- far ridge: semi-transparent violet-gray woods, lost edges, no blender
ridgeV = pile{{"lead white",3},{"cobalt blue",1.3},{"Indian red",0.8},{"raw umber",1.3},{"yellow ochre",1}, medium=0.4}
local keep = -(tlM + seamM)
local ridge = outline{{430,396},{480,387},{530,384},{580,387},{640,381},{700,376},{750,381},{800,388},{850,384},{900,378},{950,376},{1000,379},{1000,414},{430,414}, char="soft", seed=43, lobe=14, closed=true}:mask()
work(ridge, {hand="scumble", pile=ridgeV, coverage=1.1, tool="filbert 8", length={12,24}, edge="lost", clip=keep, load=0.5})
local n8 = noise{seed=120, period=50}
local hedge = ribbon({{440,410},{560,408},{650,411},{740,407},{830,409},{920,405},{1000,406}}, {7,9,7,10,12,10,12}):times(function(x,y) return clamp((n8(x,y*3)+0.35)*2,0,1) end)
work(hedge, {hand="scumble", pile=hedgeC, coverage=1.0, tool="filbert 5", length={6,14}, edge="soft", clip=keep})

--@ chunk 62
blend(ribbon({{540,372},{1000,366}}, 22):soften(8), {angle=0})
blend(ribbon({{520,408},{1000,405}}, 16):soften(6), {angle=0})
-- tree flank patch fused into the mass; add modeling across it
local fl = tlM:times(function(x,y) return (x>290 and x<470 and y>215 and y<400) and 1 or 0 end):shrink(6)
blend(fl, {angle=1.1})
local n5 = noise{seed=201, period=60}
stipple(tlM:shrink(8):times(function(x,y) return (x>250 and y>180) and clamp((n5(x,y))*2.5,0,1) or 0 end), {pile=treeM, width=7, coverage=0.6, cluster={0.5,20}, feather=0.6, drag={3,0.8}})
stipple(tlM:shrink(8):times(function(x,y) return clamp((n5(x+300,y+100)-0.05)*2.5,0,1) end), {pile=leafDD, width=9, coverage=0.7, cluster={0.5,24}, feather=0.6, drag={4,0.8}})
-- meadow's far edge
local n1 = noise{seed=211, period=100}
local medge = mask(function(x,y) return (x>440 and y>414+4*n1(x,0) and y<448) and 1 or 0 end)
work(medge, {hand="body", pile=grass, coverage=1.3, angle=0, tool="filbert 7", length={30,80}, fill=true, clip=mask(function(x,y) return y>410 and 1 or 0 end) - seamM})
work(medge:times(function(x,y) return n1(x*0.4,y*3+50)>0.1 and 1 or 0 end), {hand="body", pile=russet, coverage=0.8, angle=0, tool="filbert 6", length={30,70}, clip=mask(function(x,y) return y>412 and 1 or 0 end) - seamM})
work(ribbon({{540,424},{650,421},{770,425}}, {2,6,2}), {hand="body", pile=gold, coverage=1.0, angle=0, tool="filbert 5", length={20,50}})
blend(mask(function(x,y) return (x>440 and y>420 and y<470) and 1 or 0 end):soften(10), {angle=0})

--@ chunk 63
local body_ = (tlM + seamM):shrink(5):times(function(x,y) return y<432 and 1 or 0 end)
local n5 = noise{seed=221, period=70}
work(body_, {hand="body", pile=treeD, coverage=1.3, tool="filbert 12", length={15,40}, angle=function(x,y) return 1.0+0.6*n5(x,y) end, clip=(tlM+seamM):shrink(2)})
work(body_:times(function(x,y) return (x>160 and y<340 and n5(x+200,y)>0.1) and 1 or 0 end), {hand="body", pile=treeM, coverage=0.8, tool="filbert 9", length={10,25}, angle=function(x,y) return 0.6+0.6*n5(x,y) end, clip=(tlM+seamM):shrink(4)})
work(body_:times(function(x,y) return (n5(x+500,y+200)>0.15 or y>350) and 1 or 0 end), {hand="body", pile=deepG, coverage=0.8, tool="filbert 10", length={12,30}, angle=function(x,y) return 1.2+0.6*n5(x,y) end, clip=(tlM+seamM):shrink(3)})
blend(body_, {angle=1.0})
-- ridge crown clumps
local n8 = noise{seed=231, period=28}
local crown = ribbon({{480,372},{600,368},{700,362},{800,366},{900,362},{1000,364}}, {10,12,16,12,14,12}):times(function(x,y) return clamp((n8(x,y*1.5)+0.1)*2.2,0,1) end)
stipple(crown, {pile=ridgeV, width=6, coverage=1.2, cluster={0.5,14}, feather=0.6})
-- meadow strip carried left under the tree's foot
local ms = mask(function(x,y) return (x>250 and x<520 and y>418 and y<452) and 1 or 0 end)
work(ms, {hand="body", pile=deep, coverage=1.1, angle=0, tool="filbert 8", length={30,80}, edge="lost"})
blend(mask(function(x,y) return (x>250 and x<640 and y>420 and y<462) and 1 or 0 end):soften(10), {angle=0})

--@ chunk 64
wait(7*24*60)
print(drying(250,250), drying(600,440), drying(700,390), drying(400,440))

--@ chunk 65
local keep = -(tlM + seamM)
local function hills(pts, seed, lobe)
  local p = {}
  for i,q in ipairs(pts) do p[#p+1]=q end
  p[#p+1]={1000,418}; p[#p+1]={pts[1][1],418}
  return outline{pts=p, char="soft", seed=seed, lobe=lobe, closed=true}:mask() * keep
end
ridgeP = pile{{"lead white",5},{"cobalt blue",1.2},{"Indian red",0.7},{"raw umber",1},{"yellow ochre",1.4}, medium=0.2}
local far = hills({{440,392},{490,380},{530,372},{575,378},{620,366},{665,352},{705,348},{745,358},{780,372},{830,366},{880,352},{925,346},{965,352},{1000,356}}, 51, 18)
stipple(far, {pile=ridgeP, width=9, coverage=2.2, feather=0.3, drag={3,0}, dips={14,0.8,0.3}})
local nearw = hills({{440,404},{480,396},{520,390},{560,396},{600,392},{650,388},{700,384},{750,390},{800,396},{850,390},{900,384},{950,380},{1000,384}}, 53, 13)
stipple(nearw, {pile=ridgeV, width=7, coverage=2.2, feather=0.3, drag={3,0}, dips={14,0.8,0.3}})
local hedge = hills({{440,412},{500,408},{540,404},{580,409},{640,406},{690,409},{740,403},{790,400},{840,405},{900,402},{960,398},{1000,400}}, 55, 9)
stipple(hedge, {pile=hedgeC, width=5, coverage=2.0, feather=0.3, drag={3,0}, dips={14,0.8,0.3}})
-- meadow edge to cover bottom
work(mask(function(x,y) return (x>430 and y>414 and y<430) and 1 or 0 end), {hand="body", pile=grass, coverage=1.3, angle=0, tool="filbert 6", length={30,80}, fill=true, clip=mask(function(x,y) return y>412 and 1 or 0 end)*keep})
work(ribbon({{540,423},{650,420},{770,424}}, {2,5,2}), {hand="body", pile=gold, coverage=1.0, angle=0, tool="filbert 5", length={20,50}})

--@ chunk 66
local keep = -(tlM + seamM)
-- cover slab remnants with far-wood touches
local n8 = noise{seed=241, period=30}
local cover = (ribbon({{500,362},{600,356},{700,352}}, {12,14,10}) + ribbon({{780,358},{860,352},{930,348},{1000,350}}, {10,14,12,12})):times(function(x,y) return clamp((n8(x,y)+0.5)*2,0,1) end) * keep
stipple(cover, {pile=ridgeP, width=8, coverage=2.0, feather=0.4, drag={3,0}})
-- darker nearer wood accents
local acc = ribbon({{450,400},{560,398},{680,394},{800,396},{900,390},{1000,390}}, {8,9,10,9,11,10}):times(function(x,y) return clamp((n8(x+100,y*2)+0.1)*2.2,0,1) end) * keep
stipple(acc, {pile=hedgeC, width=5, coverage=1.6, feather=0.4, drag={3,0}})
-- meadow restated, top edge irregular
local mclip = mask(function(x,y) return y>413 and 1 or 0 end) - seamM:shrink(4)
local mead = mask(function(x,y) return (x>400 and y>414 and y<462) and 1 or 0 end)
work(mead, {hand="body", pile=grass, coverage=1.6, angle=0, tool="filbert 8", length={30,90}, clip=mclip, edge="lost"})
local n1 = noise{seed=251, period=110}
work(mead:times(function(x,y) return n1(x*0.35,y*2.5)>0.05 and 1 or 0 end), {hand="body", pile=russet, coverage=0.9, angle=0, tool="filbert 7", length={30,80}, clip=mclip, edge="lost"})
work(mead:times(function(x,y) return (x<600 and n1(x*0.35+300,y*2.5)>-0.1) and 1 or 0 end), {hand="body", pile=bank, coverage=0.9, angle=0, tool="filbert 7", length={30,80}, clip=mclip, edge="lost"})
blend(mask(function(x,y) return (x>380 and y>420 and y<468) and 1 or 0 end):soften(8), {angle=0})
work(ribbon({{560,424},{660,421},{770,425}}, {2,5,2}) + ribbon({{600,442},{680,439},{760,444}},{2,5,2}), {hand="body", pile=gold, coverage=1.0, angle=0, tool="filbert 5", length={20,50}})

--@ chunk 67
local water = poly({{420,552},{480,532},{590,526},{690,540},{640,562},{540,572},{450,566}}, true)
local gclip = mask(function(x,y) return y>413 and 1 or 0 end) - seamM:shrink(5) - water:grow(1)
local n1 = noise{seed=261, period=130}
local function zone(f) return mask(function(x,y) if y<=413 then return 0 end; return f(x,(y-413)/(H-413), n1(x*0.3,y*1.6)) and 1 or 0 end) end
local o = {hand="body", angle=0.01, tool="filbert 14", length={50,140}, coverage=1.5, clip=gclip, fill=true}
grassF = pile{{"yellow ochre",3},{"raw sienna",1.5},{"lead white",1.6},{"Antwerp blue",0.22},{"raw umber",0.8},{"red earth",0.2}, medium=0.2}
grassM = pile{{"raw sienna",2.5},{"yellow ochre",1.5},{"raw umber",2},{"Antwerp blue",0.3},{"red earth",0.4}, medium=0.2}
o.pile=grassF; work(zone(function(x,t,n) return t < 0.2 + 0.05*n end), o)
o.pile=grassM; work(zone(function(x,t,n) return t >= 0.16 + 0.05*n and t < 0.5 + 0.08*n end), o)
o.pile=bank;   work(zone(function(x,t,n) return t >= 0.44 + 0.08*n and t < 0.75 + 0.08*n end), o)
o.pile=deep;   work(zone(function(x,t,n) return t >= 0.68 + 0.08*n end), o)
-- side darks and the tree's shadow
o.fill=false; o.coverage=1.0; o.edge="lost"
o.pile=deep; work(zone(function(x,t,n) local s=math.abs(x-600)/520; return t>0.1 and (s*s*1.1 + t*0.5 + 0.2*n > 0.75) end), o)
o.pile=deepG; work(poly({{0,430},{430,424},{600,432},{480,452},{260,468},{0,480}},true), o)
-- russet warmth mid-field and light path to the pool
o.pile=russet; o.coverage=0.7; work(zone(function(x,t,n) return t>0.12 and t<0.6 and n>0.15 end), o)
blend(gclip, {angle=0.01})
blend(gclip, {angle=-0.02})

--@ chunk 68
local baseb = outline{{0,400},{120,404},{260,402},{380,400},{460,404},{520,416},{470,432},{360,442},{200,446},{0,452}, char="soft", seed=61, lobe=18, closed=true}:mask()
work(baseb, {hand="body", pile=deepG, coverage=1.5, angle=0.0, tool="filbert 10", length={20,50}, edge="lost", fill=true})
local shadow = poly({{0,446},{360,440},{560,430},{640,438},{520,458},{300,474},{0,488}}, true)
work(shadow, {hand="body", pile=deep, coverage=1.1, angle=0.0, tool="filbert 12", length={40,110}, edge="lost"})
blend(mask(function(x,y) return (x<700 and y>428 and y<500) and 1 or 0 end):soften(14), {angle=0})
-- lower tree body deeper, merging to base
local low = tlM:shrink(4):times(function(x,y) return clamp((y-300)/90,0,1) end)
work(low, {hand="body", pile=deepG, coverage=1.0, tool="filbert 12", length={15,35}, angle=0.9, clip=tlM:shrink(2)})
blend(tlM:shrink(5):times(function(x,y) return y>270 and 1 or 0 end), {angle=0.9})

--@ chunk 69
wait(8*24*60)
print(drying(250,250), drying(600,440), drying(700,390), drying(400,440), drying(500,620))

--@ chunk 70
-- right trees
tb = brush{kind="round", width=6, point=0.8}
local function trunk(pts, p0, p1, l) tb:load(trk, l or 0.9); tb:stroke(pts, {pressure={p0,p1}}) end
trunk({{829,220},{821,300},{825,380},{820,424}}, 0.4, 0.8)
trunk({{897,232},{886,262},{875,320},{878,380},{883,420}}, 0.1, 0.6)
trunk({{920,248},{905,270},{876,302}}, 0.05, 0.25, 0.6)
trunk({{864,248},{850,272},{823,302}}, 0.05, 0.25, 0.6)
local n3 = noise{seed=83, period=42}
local n4 = noise{seed=84, period=16}
local dens = function(x,y) return clamp((n3(x,y)+0.15)*2.2,0,1) * (0.5+0.5*n4:at01(x,y)) end
stipple(trB:shrink(3):times(dens), {pile=leafD, width=4.5, coverage=1.3, cluster={0.5,14}, feather=0.7, drag={3,0.4}, dips={10,0.6,0.5}})
stipple(trB:shrink(5):times(function(x,y) return clamp((n3(x,y)-0.05)*3,0,1) * (0.4+0.6*n4:at01(x,y)) end), {pile=leafDD, width=5, coverage=1.5, cluster={0.5,12}, feather=0.6, drag={3,0.5}})
stipple(trB:grow(9):times(dens), {pile=leafD, width=2.5, coverage=0.5, cluster={0.7,12}, feather=0.8, drag={2,0.2}})
-- grass tufts at the trunk feet
work(ellipse(850,426,60,7), {hand="scumble", pile=bank, coverage=0.8, tool="filbert 5", length={8,18}, edge="lost"})
-- ridge: thin violet glaze to set it back a tone darker, mostly lower part
ridgeGl = pile{{"cobalt blue",1.2},{"Indian red",0.7},{"raw umber",1.2}, medium=0.75}
local rg = mask(function(x,y) return (x>470 and y>372 and y<412) and 1 or 0 end)
work(rg, {hand="glaze", pile=ridgeGl, tool="filbert 10", length={40,100}, angle=0, clip=rg:grow(4) - tlM - seamM, load=0.4})
-- sun
sunH = pile{{"lead white",8},{"lemon chrome",2.5},{"orange chrome",1}, medium=0.6}
work(ellipse(640,318,60,26), {hand="scumble", pile=sunH, coverage=0.7, tool="filbert 10", edge="lost", load=0.3, angle=0})
blend(ellipse(640,318,90,42):soften(25), {angle=0})
local sb = brush("round", 8)
sb:load(pile{{"lead white",10},{"lemon chrome",1.2},{"orange chrome",0.25}}, 0.9)
sb:touch(640,320,{pressure=0.75})

--@ chunk 71
local rg = mask(function(x,y) return (x>462 and y>366 and y<416) and 1 or 0 end)
local r6 = rag{width=22}
r6:wipe(rg, {pressure=0.75, angle=0, passes=2, refold=0.25})
r6 = rag{width=22}; r6:dip(0.4)
r6:wipe(rg, {pressure=0.6, angle=0, passes=1, refold=0.25})
blend(ellipse(850,427,75,10):soften(5), {angle=0})

--@ chunk 72
local rg = mask(function(x,y) return (x>462 and y>366 and y<416) and 1 or 0 end) + ellipse(850,427,80,13)
local r6 = rag{width=22}; r6:dip(0.9)
r6:wipe(rg, {pressure=0.9, angle=0, passes=3, refold=0.2})
r6 = rag{width=22}; r6:dip(0.9)
r6:wipe(rg, {pressure=0.9, angle=0.3, passes=2, refold=0.2})

--@ chunk 73
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{823,340},{825,385},{820,426}}, {pressure={0.7,0.8}})
tb:load(trk, 0.9); tb:stroke({{876,340},{878,385},{883,422}}, {pressure={0.5,0.6}})
local ov = ellipse(850,428,90,12)
work(ov, {hand="body", pile=grassF, coverage=1.3, angle=0, tool="filbert 7", length={30,80}, edge="lost", clip=mask(function(x,y) return y>416 and 1 or 0 end)})
-- hedge dark touches along the meadow edge, irregular
local n8 = noise{seed=271, period=45}
local hedge = ribbon({{480,412},{600,411},{740,410},{860,411},{1000,409}}, {5,7,6,8,8}):times(function(x,y) return clamp((n8(x,y*3))*2.5,0,1) end)
stipple(hedge, {pile=hedgeC, width=5, coverage=1.3, feather=0.5, drag={4,0}})
-- grass tufts at the trunk feet
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(14, 805, 900, 0.6, 0.6, 5)) do
  rb:load(bank, 0.5)
  rb:stroke({{x, 430+rand(-3,3)},{x+rand(-3,3), 430-rand(6,12)}}, {pressure={0.5,0.05}})
end

--@ chunk 74
blend(ellipse(850,430,110,16):soften(8), {angle=0})
-- pool banks
local near = outline{{395,560},{430,566},{470,562},{520,570},{580,566},{630,560},{680,548},{715,540},{730,560},{650,588},{520,596},{410,586}, char="soft", seed=71, lobe=12, closed=true}:mask()
work(near, {hand="body", pile=deep, coverage=1.4, angle=0.0, tool="filbert 7", length={20,50}, edge="lost", fill=true})
local farb = outline{{400,548},{420,532},{470,526},{520,530},{570,522},{640,524},{700,534},{720,544},{690,538},{640,531},{570,529},{520,536},{470,533},{430,540}, char="soft", seed=73, lobe=10, closed=true}:mask()
work(farb, {hand="body", pile=grassM, coverage=1.3, angle=0.0, tool="filbert 6", length={15,40}, edge="lost"})
-- reflection darks: a soft tree-shadow reflection on the left part of the water
refl = pile{{"raw umber",2},{"raw sienna",1.5},{"bone black",0.5}, medium=0.6}
work(poly({{425,540},{490,534},{500,560},{440,562}},true), {hand="glaze", pile=refl, tool="filbert 8", length={15,40}, angle=0, clip=ellipse(555,550,135,26), load=0.4})
-- reeds
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(18, 405, 540, 0.6, 0.6, 5)) do
  rb:load(deep, 0.5)
  rb:stroke({{x, 574+rand(-4,4)},{x+rand(-4,4), 574-rand(10,24)}}, {pressure={0.5,0.05}})
end
for i,x in ipairs(uneven(9, 650, 720, 0.6, 0.6, 6)) do
  rb:load(deep, 0.5)
  rb:stroke({{x, 556+rand(-3,3)},{x+rand(-3,3), 556-rand(8,18)}}, {pressure={0.45,0.05}})
end

--@ chunk 75
local r7 = rag{width=30}; r7:dip(0.9)
local reg = ellipse(560,558,190,50)
r7:wipe(reg, {pressure=0.9, angle=0, passes=3, refold=0.2})
r7 = rag{width=30}; r7:dip(0.9)
r7:wipe(reg, {pressure=0.9, angle=0.2, passes=2, refold=0.2})

--@ chunk 76
waterM = outline{{430,551},{462,540},{505,536},{548,539},{590,532},{640,535},{684,543},{662,553},{615,558},{570,566},{520,563},{470,566},{440,560}, char="soft", seed=77, lobe=14, closed=true, amount=0.7}:mask()
local ring = ellipse(558,552,160,38) - waterM
local up = ring:times(function(x,y) return y<550 and 1 or 0 end)
local dn = ring:times(function(x,y) return y>=550 and 1 or 0 end)
work(up, {hand="body", pile=grassM, coverage=1.6, angle=0, tool="filbert 6", length={15,40}, fill=true, clip=-(waterM:shrink(1))})
work(dn, {hand="body", pile=bank, coverage=1.6, angle=0, tool="filbert 6", length={15,40}, fill=true, clip=-(waterM:shrink(1))})
work(ellipse(558,585,170,22), {hand="body", pile=deep, coverage=1.0, angle=0, tool="filbert 9", length={30,80}, edge="lost"})
-- water: sky reflection, cooler and a tone lower than the sky, brightest centre-right
work(waterM, {hand="body", pile=poolR, coverage=1.5, angle=0, tool="filbert 5", length={15,40}, fill=true, clip=waterM})
work(ribbon({{540,546},{600,543},{650,546}}, {2,6,2}), {hand="body", pile=skyGP, coverage=1.0, angle=0, tool="filbert 4", length={15,35}, clip=waterM})
-- soften outer ring into the meadow (ring only, not the water)
local outer = ellipse(558,556,200,56):soften(10) - waterM:grow(4)
blend(outer, {angle=0})

--@ chunk 77
wait(6*24*60)
print(drying(560,550), drying(600,390), drying(560,580))
local keep = -(tlM + seamM)
local function hills(pts, seed, lobe, bottom)
  local p = {}
  for i,q in ipairs(pts) do p[#p+1]=q end
  p[#p+1]={1000,bottom}; p[#p+1]={pts[1][1],bottom}
  return outline{pts=p, char="soft", seed=seed, lobe=lobe, closed=true}:mask() * keep
end
ridge2 = pile{{"lead white",3.2},{"cobalt blue",1.3},{"Indian red",0.75},{"raw umber",1.4},{"yellow ochre",1.1}, medium=0.2}
ridge3 = pile{{"lead white",1.6},{"cobalt blue",1.1},{"Indian red",0.6},{"raw umber",2.2},{"raw sienna",1.2}, medium=0.2}
local nearw = hills({{440,400},{480,390},{520,383},{560,390},{600,385},{650,378},{700,372},{750,380},{800,388},{850,380},{900,372},{950,368},{1000,372}}, 53, 13, 416)
stipple(nearw, {pile=ridge2, width=8, coverage=3.5, feather=0.2, drag={4,0}, dips={12,0.9,0.3}})
local hedge = hills({{440,410},{500,405},{540,400},{580,406},{640,402},{690,406},{740,399},{790,396},{840,402},{900,398},{960,394},{1000,396}}, 55, 9, 416)
stipple(hedge, {pile=ridge3, width=6, coverage=3.0, feather=0.2, drag={4,0}, dips={12,0.9,0.3}})

--@ chunk 78
local mclip = mask(function(x,y) return y>414 and 1 or 0 end) - seamM:shrink(5)
work(mask(function(x,y) return (x>430 and y>415 and y<432) and 1 or 0 end), {hand="body", pile=grassF, coverage=1.6, angle=0, tool="filbert 6", length={30,80}, fill=true, clip=mclip})
wait(4*24*60)
print(drying(700,395), drying(700,424))
local keep = -(tlM + seamM)
local n8 = noise{seed=281, period=40}
local hedge = ribbon({{470,409},{600,407},{740,405},{860,406},{1000,403}}, {7,9,8,10,10}):times(function(x,y) return clamp((n8(x,y*2.5)+0.15)*2.5,0,1) end) * keep
stipple(hedge, {pile=ridge3, width=6, coverage=2.2, feather=0.4, drag={4,0}})
local hd = ribbon({{470,412},{600,411},{740,410},{860,411},{1000,409}}, {3,5,4,6,6}):times(function(x,y) return clamp((n8(x+200,y*2.5)+0.1)*2.5,0,1) end) * keep
stipple(hd, {pile=hedgeC, width=4, coverage=1.8, feather=0.4, drag={4,0}})
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{823,350},{825,390},{820,426}}, {pressure={0.7,0.8}})
tb:load(trk, 0.9); tb:stroke({{876,350},{878,390},{883,422}}, {pressure={0.5,0.6}})
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(12, 805, 900, 0.6, 0.6, 5)) do
  rb:load(bank, 0.5)
  rb:stroke({{x, 428+rand(-3,3)},{x+rand(-3,3), 428-rand(5,10)}}, {pressure={0.5,0.05}})
end

--@ chunk 79
local mb = mask(function(x,y) return (x>400 and y>419 and y<452) and 1 or 0 end):soften(5)
blend(mb, {angle=0})
blend(mb, {angle=0.03})
wait(5*24*60)
print(drying(560,550), drying(700,424), drying(640,320))

--@ chunk 80
glowGl = pile{{"raw sienna",2},{"orange chrome",1},{"cadmium yellow",0.6}, medium=0.9}
local keep = -(tlM:grow(2) + seamM)
local low = mask(function(x,y) if y>376 or x<400 then return 0 end; local d=math.sqrt(((x-650)/330)^2+((y-365)/95)^2); return d<1 and 1 or 0 end) * keep
work(low, {hand="glaze", pile=glowGl, tool="filbert 20", length={60,160}, angle=0, clip=mask(function(x,y) return y<380 and 1 or 0 end)*keep, load=0.35, coverage=0.8})

--@ chunk 81
local keep = -(tlM:grow(2) + seamM)
local reg = (mask(function(x,y) return (x>395 and y>215 and y<384) and 1 or 0 end)):soften(18) * keep
for i,a in ipairs{0, 1.5, 0.05, 1.4, -0.05, 0} do blend(reg, {angle=a}) end

--@ chunk 82
local keep = -(tlM:grow(2) + seamM)
local reg = (mask(function(x,y) return (x>395 and y>200 and y<386) and 1 or 0 end)):soften(22) * keep
for i,a in ipairs{0.02, 1.57, 0, -0.03} do blend(reg, {angle=a}) end
-- sun: a pale disc wiped out of the glaze
local rs = rag{width=10}; rs:dip(0.5)
rs:blot(640,330,{pressure=0.8}); rs:refold(); rs:blot(640,330,{pressure=0.6})
-- meadow edge under the tree: quiet it with dark strokes
work(poly({{395,416},{600,417},{640,428},{560,446},{395,450}},true), {hand="body", pile=bank, coverage=1.3, angle=0, tool="filbert 8", length={40,100}, edge="lost", clip=mask(function(x,y) return y>415 and 1 or 0 end)})
blend(mask(function(x,y) return (x>380 and x<700 and y>419 and y<460) and 1 or 0 end):soften(6), {angle=0})

--@ chunk 83
local mclip = mask(function(x,y) return y>415 and 1 or 0 end)
local n1 = noise{seed=291, period=150}
local band = mask(function(x,y) return (y>415 and y<475) and 1 or 0 end)
local o = {hand="body", angle=0, tool="filbert 9", length={50,130}, coverage=1.4, clip=mclip, edge="lost"}
o.pile=grassF; work(band:times(function(x,y) return x>560+60*n1(x,y*3) and 1 or 0 end), o)
o.pile=grassM; work(band:times(function(x,y) return (x>330 and x<=620+60*n1(x,y*3)) and 1 or 0 end), o)
o.pile=deep; work(band:times(function(x,y) return (x<=420+50*n1(x,y*3)) and 1 or 0 end), o)
o.pile=russet; o.coverage=0.6; work(band:times(function(x,y) return (x>450 and y>440 and n1(x*0.4+400,y*3)>0.1) and 1 or 0 end), o)
local bl = mask(function(x,y) return (y>420 and y<490) and 1 or 0 end):soften(8)
blend(bl, {angle=0})
blend(bl, {angle=0.02})
o.pile=gold; o.tool="filbert 5"; o.length={20,50}; o.coverage=1.0
work(ribbon({{600,424},{690,421},{790,425}}, {2,5,2}) + ribbon({{620,441},{700,438},{770,443}},{2,4,2}), o)

--@ chunk 84
local bl = mask(function(x,y) return (y>450 and y<525) and 1 or 0 end):soften(14) - ellipse(558,552,150,34)
blend(bl, {angle=0})
blend(bl, {angle=-0.02})
blend(ribbon({{590,430},{800,430}}, 26):soften(6), {angle=0})
-- undergrowth at the foot of the big tree, irregular, over the straight edge
local under = outline{{0,398},{80,404},{170,400},{260,406},{340,402},{420,405},{475,409},{520,418},{470,428},{380,424},{300,432},{200,428},{100,434},{0,430}, char="soft", seed=81, lobe=16, closed=true}:mask()
work(under, {hand="body", pile=deepG, coverage=1.5, angle=0, tool="filbert 8", length={15,40}, edge={soft=0.5, lost=0.5, period=30}, fill=true})
local rb = brush{kind="round", width=3.5, point=0.9}
for i,x in ipairs(uneven(40, 10, 540, 0.7, 0.6, 9)) do
  rb:load(deepG, 0.5)
  local y0 = 428+rand(-5,6)
  rb:stroke({{x, y0},{x+rand(-4,4), y0-rand(7,16)}}, {pressure={0.55,0.05}})
end

--@ chunk 85
local bl = mask(function(x,y) return (x>420 and y>418 and y<470) and 1 or 0 end):soften(10)
blend(bl, {angle=0}); blend(bl, {angle=0.6}); blend(bl, {angle=0})
wait(8*24*60)
print(drying(560,550), drying(700,430), drying(250,415), drying(640,330))

--@ chunk 86
-- pool: warm glaze, wiped at the centre
poolGl = pile{{"raw sienna",2},{"yellow ochre",1.5},{"orange chrome",0.4},{"raw umber",0.5}, medium=0.85}
work(waterM, {hand="glaze", pile=poolGl, tool="filbert 8", length={20,60}, angle=0, clip=waterM:grow(2), load=0.35})
local rs = rag{width=12}
rs:wipe({{560,546},{610,543},{650,545}}, {pressure=0.6})
blend(waterM:grow(3), {angle=0})
-- dark reeds and bank accents at the near edge
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(16, 432, 540, 0.6, 0.6, 5)) do
  rb:load(deep, 0.5)
  local y0 = 568+rand(-3,3)
  rb:stroke({{x, y0},{x+rand(-4,4), y0-rand(8,20)}}, {pressure={0.5,0.05}})
end
for i,x in ipairs(uneven(7, 640, 690, 0.6, 0.6, 6)) do
  rb:load(deep, 0.5)
  local y0 = 556+rand(-3,3)
  rb:stroke({{x, y0},{x+rand(-3,3), y0-rand(7,15)}}, {pressure={0.45,0.05}})
end
-- sun: small pale disc in the haze
local sb = brush("round", 7)
sb:load(pile{{"lead white",10},{"lemon chrome",1.5},{"orange chrome",0.4}}, 0.8)
sb:touch(642,334,{pressure=0.7})
-- gold lights on the far meadow, small dragged touches
local gb = brush("filbert", 5)
for i,p in ipairs{{600,424,40},{660,422,50},{730,425,36},{640,436,44},{700,440,30},{590,452,36}} do
  gb:load(gold, 0.5)
  gb:stroke({{p[1],p[2]},{p[1]+p[3],p[2]+rand(-1,1)}}, {pressure={0.5,0.15}})
end
-- a small figure: a woman in a dull red shawl walking on the meadow path, right of centre
fig = brush{kind="round", width=3.5, point=0.6}
fig:load(pile{{"bone black",2},{"raw umber",2},{"cobalt blue",0.5}}, 0.7)
fig:stroke({{716,452},{717,466}}, {pressure={0.7,0.9}})
fig:load(pile{{"Indian red",2},{"red earth",1},{"raw umber",0.5}}, 0.6)
fig:stroke({{716,447},{716.5,455}}, {pressure={0.6,0.8}})
fig:load(pile{{"lead white",3},{"yellow ochre",1},{"red earth",0.5}}, 0.5)
fig:touch(716,444,{pressure=0.35})

--@ chunk 87
local rs = rag{width=16}
rs:wipe(waterM:shrink(3), {pressure=0.7, angle=0, passes=2, refold=0.3})
blend(ribbon({{580,438},{800,438}}, 40):soften(8), {angle=0})

--@ chunk 88
local lens = poly({{446,551},{500,544},{570,541},{640,542},{676,547},{640,554},{570,559},{490,558}}, true)
local ring = ellipse(558,551,165,30) - lens
local up = ring:times(function(x,y) return y<550 and 1 or 0 end)
local dn = ring:times(function(x,y) return y>=550 and 1 or 0 end)
local o = {hand="body", angle=0, tool="filbert 5", length={30,80}, coverage=1.6, fill=true, clip=-lens}
o.pile=grassM; work(up, o)
o.pile=bank; work(dn, o)
o.pile=deep; o.coverage=0.9; o.fill=false; o.edge="lost"; work(dn:times(function(x,y) return y>560 and 1 or 0 end), o)
blend(ellipse(558,551,185,40):soften(8) - lens:grow(2), {angle=0})
-- reeds again
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(14, 450, 545, 0.6, 0.6, 5)) do
  rb:load(deep, 0.5)
  local y0 = 562+rand(-3,3)
  rb:stroke({{x, y0},{x+rand(-4,4), y0-rand(8,18)}}, {pressure={0.5,0.05}})
end
-- figure
fig = brush{kind="round", width=3.5, point=0.6}
fig:load(pile{{"bone black",2},{"raw umber",2},{"cobalt blue",0.5}}, 0.7)
fig:stroke({{716,453},{717,467}}, {pressure={0.7,0.9}})
fig:load(pile{{"Indian red",2},{"red earth",1.5},{"raw umber",0.3}}, 0.6)
fig:stroke({{716,448},{716.5,456}}, {pressure={0.6,0.8}})
fig:load(pile{{"lead white",3},{"yellow ochre",1},{"red earth",0.5}}, 0.5)
fig:touch(716,445,{pressure=0.35})

--@ chunk 89
lensM = poly({{446,551},{500,544},{570,541},{640,542},{676,547},{640,554},{570,559},{490,558}}, true)
local under = outline{{0,398},{80,404},{170,400},{260,406},{340,402},{420,405},{475,409},{520,418},{470,426},{380,423},{300,429},{200,426},{100,431},{0,428}, char="soft", seed=81, lobe=16, closed=true}:mask()
local gclip = mask(function(x,y) return y>415 and 1 or 0 end) - under:shrink(3) - lensM
local n1 = noise{seed=301, period=130}
local function zone(f) return mask(function(x,y) if y<=415 then return 0 end; return f(x,(y-415)/(H-415), n1(x*0.3,y*1.6)) and 1 or 0 end) end
local o = {hand="body", angle=0.0, tool="filbert 14", length={60,160}, coverage=1.6, clip=gclip, fill=true}
o.pile=grassF; work(zone(function(x,t,n) return t < 0.14 + 0.04*n and x > 480 end), o)
o.pile=grassM; work(zone(function(x,t,n) return (t >= 0.10 + 0.04*n or x<=500) and t < 0.5 + 0.08*n end), o)
o.pile=bank;   work(zone(function(x,t,n) return t >= 0.44 + 0.08*n and t < 0.75 + 0.08*n end), o)
o.pile=deep;   work(zone(function(x,t,n) return t >= 0.68 + 0.08*n end), o)
o.fill=false; o.coverage=1.0
o.pile=deep; work(zone(function(x,t,n) local s=math.abs(x-620)/520; return (s*s*1.1 + t*0.5 + 0.2*n > 0.75) end), o)
o.pile=deepG; work(poly({{0,426},{470,424},{640,430},{500,450},{260,466},{0,478}},true), o)
o.pile=bank; work(ellipse(560,566,150,14), o)
o.pile=russet; o.coverage=0.6; work(zone(function(x,t,n) return t>0.1 and t<0.6 and n>0.15 end), o)
blend(gclip, {angle=0.0})
blend(gclip, {angle=-0.02})
blend(gclip, {angle=0.02})

--@ chunk 90
-- tree shadow & foot
local o = {hand="body", angle=0.0, tool="filbert 12", length={50,130}, coverage=1.1, edge="lost"}
o.pile=deepG; work(poly({{0,420},{480,420},{620,426},{520,444},{300,458},{0,468}},true), o)
local foot = mask(function(x,y) return (x<660 and y>410 and y<480) and 1 or 0 end):soften(8)
blend(foot, {angle=0}); blend(foot, {angle=0.15})
-- water, larger, wet into the bank
local w2 = poly({{425,553},{480,543},{560,538},{650,539},{705,546},{660,557},{580,564},{480,563}}, true)
work(w2, {hand="body", pile=poolR, coverage=1.5, angle=0, tool="filbert 5", length={25,70}, fill=true, clip=w2:soften(3)})
work(w2, {hand="body", pile=poolR, coverage=1.2, angle=0, tool="filbert 5", length={25,70}, clip=w2:shrink(2)})
work(ribbon({{540,548},{610,545},{670,548}}, {2,6,2}), {hand="body", pile=skyGP, coverage=1.2, angle=0, tool="filbert 4", length={20,45}, clip=w2:shrink(2)})
-- dark near bank under the water
o.pile=deep; o.tool="filbert 7"; o.length={30,80}; o.coverage=1.0
work(ribbon({{430,566},{560,572},{690,562}}, {4,8,4}), o)

--@ chunk 91
local r8 = rag{width=12}; r8:dip(0.9)
for k=1,3 do
  r8:wipe({{672,408},{600,408},{528,409}}, {pressure=0.9}); r8:refold(); r8:dip(0.9)
  r8:wipe({{672,402},{600,402},{540,403}}, {pressure=0.9}); r8:refold(); r8:dip(0.9)
end

--@ chunk 92
local m = mask(function(x,y) return (x>470 and x<900 and y>417 and y<500) and 1 or 0 end)
blend(m, {angle=0}); blend(m, {angle=0}); blend(m, {angle=0.1})
local o = {hand="body", angle=0.0, tool="filbert 9", length={50,130}, coverage=1.0, edge="lost", clip=mask(function(x,y) return y>417 and 1 or 0 end)}
o.pile=grassF; work(mask(function(x,y) return (x>600 and y>418 and y<440) and 1 or 0 end), o)
o.pile=grassM; work(mask(function(x,y) return (x>500 and x<700 and y>430 and y<456) and 1 or 0 end), o)
blend(mask(function(x,y) return (x>440 and y>417 and y<500) and 1 or 0 end), {angle=0})

--@ chunk 93
local w2 = poly({{425,553},{480,543},{560,538},{650,539},{705,546},{660,557},{580,564},{480,563}}, true)
local m = mask(function(x,y) return (y>417) and 1 or 0 end) - w2:grow(3)
blend(m, {angle=0}); blend(m, {angle=0.02})
-- tree foot: dark undergrowth strokes with soft irregular top over the straight seam
local under = outline{{0,396},{80,402},{170,398},{260,404},{340,400},{420,403},{475,408},{530,417},{470,425},{380,422},{300,428},{200,425},{100,430},{0,427}, char="soft", seed=85, lobe=16, closed=true}:mask()
work(under, {hand="body", pile=deepG, coverage=1.3, angle=0, tool="filbert 7", length={15,40}, edge="lost"})

--@ chunk 94
local u = mask(function(x,y) return (x<560 and y>392 and y<440) and 1 or 0 end):soften(6)
blend(u, {angle=0.4}); blend(u, {angle=0})
-- residue line right of the tree: cover with hedge touches
local n8 = noise{seed=311, period=40}
stipple(ribbon({{520,411},{600,410},{680,411}}, {5,7,5}), {pile=ridge3, width=6, coverage=2.0, feather=0.4, drag={4,0}})
-- shadow of the tree thrown toward us and right
local o = {hand="body", angle=0.0, tool="filbert 12", length={50,130}, coverage=0.9, edge="lost"}
o.pile=deepG; work(poly({{0,436},{420,430},{560,434},{470,452},{280,464},{0,474}},true), o)
blend(mask(function(x,y) return (x<640 and y>428 and y<485) and 1 or 0 end):soften(12), {angle=0})
-- trunk feet again
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{824,360},{825,395},{820,428}}, {pressure={0.7,0.8}})
tb:load(trk, 0.9); tb:stroke({{877,360},{878,395},{883,424}}, {pressure={0.5,0.6}})
wait(7*24*60)
print(drying(250,415), drying(600,450), drying(560,550), drying(250,300))

--@ chunk 95
-- 1. big tree modeling over dry paint
local n5 = noise{seed=321, period=55}
local n6 = noise{seed=322, period=14}
local T = tlM:shrink(6)
shadeGl = pile{{"raw umber",2},{"bone black",1.5},{"Antwerp blue",0.5}, medium=0.5}
local sh = T:times(function(x,y) local v = 0.35 - (x-230)/420 + (y-240)/300 + 0.5*n5(x,y); return clamp(v*1.6,0,1)*(0.5+0.5*n6:at01(x,y)) end)
stipple(sh, {pile=shadeGl, width=11, coverage=1.3, cluster={0.4,26}, feather=0.5, drag={5,0.9}, dips={10,0.6,0.4}})
local lt = T:times(function(x,y) local v = -0.25 + (x-230)/420 - (y-230)/330 + 0.7*n5(x+200,y+90); return clamp(v*2,0,1)*(0.4+0.6*n6:at01(x,y)) end)
treeL2 = pile{{"raw sienna",3},{"yellow ochre",2},{"Antwerp blue",0.3},{"raw umber",1},{"orange chrome",0.3}, medium=0.3}
stipple(lt, {pile=treeL2, width=7, coverage=0.7, cluster={0.5,20}, feather=0.6, drag={4,0.7}, dips={10,0.5,0.4}})
-- warm sun-struck leaves along right contour
local warm = tlM:rim(30, 10):times(function(x,y) return (x>330 and y>140 and y<380) and clamp((n5(x+70,y)+0.2)*2,0,1)*n6:at01(x,y) or 0 end)
stipple(warm, {pile=leafW, width=4, coverage=0.8, cluster={0.6,12}, feather=0.7, drag={2,0.5}})
-- trunk hint and a bough in the shadow
tb = brush{kind="round", width=8, point=0.5}
tb:load(trk, 0.7); tb:stroke({{252,330},{247,375},{243,418}}, {pressure={0.4,0.8}})
tb:load(trk, 0.4); tb:stroke({{254,330},{276,296},{292,274}}, {pressure={0.4,0.1}})

--@ chunk 96
local T = tlM:shrink(5)
blend(T, {angle=0.9}); blend(T, {angle=-0.5}); blend(T, {angle=1.4})

--@ chunk 97
-- sky: soft warm cloud bars, kept left of the slim trees and clear of the oak
cloudW = pile{{"lead white",7},{"orange chrome",0.9},{"Indian red",0.35},{"yellow ochre",1.5},{"cobalt blue",0.3}, medium=0.45}
local bars = ribbon({{470,236},{560,228},{660,232},{730,240}}, {4,12,10,3}) + ribbon({{500,196},{600,186},{720,190}}, {3,9,3}) + ribbon({{470,276},{540,272},{620,277}}, {3,7,2})
work(bars, {hand="scumble", pile=cloudW, coverage=0.8, tool="filbert 8", length={15,30}, angle=0, load=0.35, edge="lost"})
local sk = mask(function(x,y) return (x>455 and x<750 and y>165 and y<295) and 1 or 0 end):soften(14)
blend(sk, {angle=0}); blend(sk, {angle=0.05})
-- glow glaze low in the sky, thin, rubbed
glowGl2 = pile{{"raw sienna",1.5},{"orange chrome",1},{"lemon chrome",1}, medium=0.92}
local keep = -(tlM:grow(6) + seamM)
local lowsky = ellipse(640,345,140,26) * keep
work(lowsky, {hand="scumble", pile=glowGl2, coverage=0.6, tool="filbert 14", length={20,40}, angle=0, load=0.2, edge="lost"})
local bs = mask(function(x,y) return (x>470 and x<800 and y>300 and y<372) and 1 or 0 end):soften(14)
blend(bs, {angle=0}); blend(bs, {angle=1.5}); blend(bs, {angle=0})
local sb = brush("round", 7)
sb:load(pile{{"lead white",10},{"lemon chrome",1.5},{"orange chrome",0.4}}, 0.8)
sb:touch(642,336,{pressure=0.7})

--@ chunk 98
local keep = -(tlM:grow(3))
local reg = ellipse(630,290,240,150) * keep * mask(function(x,y) return y<392 and 1 or 0 end)
local r9 = rag{width=34}; r9:dip(0.9)
r9:wipe(reg, {pressure=0.9, angle=0, passes=3, refold=0.2})
r9 = rag{width=34}; r9:dip(0.9)
r9:wipe(reg, {pressure=0.9, angle=0.4, passes=2, refold=0.2})

--@ chunk 99
wait(7*24*60)
print(drying(250,300), drying(600,450), drying(560,550), drying(600,330))
local w2 = poly({{425,553},{480,543},{560,538},{650,539},{705,546},{660,557},{580,564},{480,563}}, true)
local gclip = mask(function(x,y) return y>416 and 1 or 0 end) - w2
local n1 = noise{seed=331, period=120}
local n2 = noise{seed=332, period=60}
local function zone(f) return mask(function(x,y) if y<=418 then return 0 end; return f(x,(y-415)/(H-415), n1(x*0.3,y*2), n2(x*0.4,y*2)) and 1 or 0 end) end
-- dry-brush broken color, light load, dragged
local o = {hand="body", angle=0.0, tool="filbert 10", length={40,120}, coverage=0.55, clip=gclip, load=0.22, pressure={0.25,0.5}, edge="lost"}
o.pile=russet; work(zone(function(x,t,n,m) return t>0.05 and t<0.7 and n>0.05 end), o)
o.pile=gold;   o.coverage=0.4; work(zone(function(x,t,n,m) return x>480 and t<0.38 and m>0.1 end), o)
o.pile=deep;   o.coverage=0.6; work(zone(function(x,t,n,m) return t>0.3 and m<-0.1 end), o)
o.pile=treeM;  o.coverage=0.4; work(zone(function(x,t,n,m) return t>0.15 and t<0.8 and n<-0.15 end), o)
-- soften the hard shadow tip by dragging meadow color over it
o.pile=grassM; o.coverage=0.9; o.load=0.4; work(poly({{540,440},{660,446},{660,480},{480,484}},true), o)
-- cover the residue line right of the oak
stipple(ribbon({{515,410},{600,409},{670,410}}, {6,8,5}), {pile=ridge3, width=6, coverage=2.5, feather=0.3, drag={4,0}})

--@ chunk 100
local w2 = poly({{425,553},{480,543},{560,538},{650,539},{705,546},{660,557},{580,564},{480,563}}, true)
local g = mask(function(x,y) return y>419 and 1 or 0 end) - w2:grow(3)
blend(g, {angle=0}); blend(g, {angle=0.03}); blend(g, {angle=-0.03})

--@ chunk 101
local n1 = noise{seed=341, period=90}
-- cast shadow and undergrowth, irregular
local shadow = outline{{0,410},{200,408},{420,408},{520,414},{600,424},{540,438},{430,444},{300,456},{150,460},{0,470}, char="soft", seed=91, lobe=24, closed=true}:mask()
work(shadow, {hand="body", pile=deepG, coverage=1.2, angle=0.0, tool="filbert 10", length={30,80}, edge="lost"})
blend(shadow:grow(12):soften(8) * mask(function(x,y) return y>421 and 1 or 0 end), {angle=0.02})
-- corners deepened
local corners = mask(function(x,y) if y<430 then return 0 end; local t=(y-430)/(H-430); local s=math.abs(x-600)/520; return (s*s + t*0.75 + 0.2*n1(x*0.4,y) > 0.95) and 1 or 0 end)
work(corners, {hand="body", pile=deep, coverage=1.0, angle=0.0, tool="filbert 14", length={50,130}, edge="lost"})
blend(corners:grow(20):soften(15) * mask(function(x,y) return y>432 and 1 or 0 end), {angle=0})
-- grass blades / tufts along the oak's foot
local rb = brush{kind="round", width=3.5, point=0.9}
for i,x in ipairs(uneven(46, 6, 560, 0.7, 0.6, 9)) do
  rb:load(deepG, 0.5)
  local y0 = 424+rand(-5,8)
  rb:stroke({{x, y0},{x+rand(-4,4), y0-rand(7,15)}}, {pressure={0.55,0.05}})
end

--@ chunk 102
local w2 = poly({{425,553},{480,543},{560,538},{650,539},{705,546},{660,557},{580,564},{480,563}}, true)
local g = mask(function(x,y) return y>430 and 1 or 0 end) - w2:grow(3)
blend(g, {angle=0}); blend(g, {angle=0.25}); blend(g, {angle=-0.25}); blend(g, {angle=0})

--@ chunk 103
local w2 = poly({{425,553},{480,543},{560,538},{650,539},{705,546},{660,557},{580,564},{480,563}}, true)
print(drying(560,550))
poolGl = pile{{"raw sienna",2},{"yellow ochre",1.5},{"orange chrome",0.5},{"raw umber",0.4}, medium=0.8}
work(w2, {hand="glaze", pile=poolGl, tool="filbert 6", length={20,60}, angle=0, clip=w2, load=0.25, coverage=0.8})
-- left third of the water darker: reflects the oak's side of the sky / bank
work(poly({{425,553},{480,543},{520,541},{510,563},{470,563}},true), {hand="glaze", pile=pile{{"raw umber",2},{"raw sienna",1},{"cobalt blue",0.4}, medium=0.75}, tool="filbert 5", length={15,40}, angle=0, clip=w2, load=0.25})
-- banks pulled over the water's edge in short strokes
local o = {hand="body", angle=0, tool="filbert 5", length={20,50}, coverage=1.1, edge="lost"}
o.pile=deep; work(ribbon({{420,560},{480,567},{580,568},{665,560},{708,550}}, {4,6,6,5,3}), o)
o.pile=grassM; work(ribbon({{424,549},{480,540},{560,535},{650,536},{706,543}}, {3,4,4,4,3}), o)
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(16, 430, 540, 0.6, 0.6, 5)) do
  rb:load(deep, 0.5); local y0 = 566+rand(-3,3)
  rb:stroke({{x, y0},{x+rand(-4,4), y0-rand(8,19)}}, {pressure={0.5,0.05}})
end
for i,x in ipairs(uneven(7, 660, 705, 0.6, 0.6, 6)) do
  rb:load(deep, 0.5); local y0 = 556+rand(-3,3)
  rb:stroke({{x, y0},{x+rand(-3,3), y0-rand(7,14)}}, {pressure={0.45,0.05}})
end
-- remove the hedge blob with meadow color
o.pile=grassF; o.tool="filbert 7"; o.length={30,80}
work(mask(function(x,y) return (x>560 and x<1000 and y>416 and y<434) and 1 or 0 end), o)
-- far gold lights
local gb = brush("filbert", 5)
for i,p in ipairs{{620,425,50},{700,423,60},{790,426,40},{660,437,50},{740,441,36}} do
  gb:load(gold, 0.5)
  gb:stroke({{p[1],p[2]},{p[1]+p[3],p[2]+rand(-1,1)}}, {pressure={0.5,0.15}})
end

--@ chunk 104
local st = mask(function(x,y) return (x>540 and y>419 and y<470) and 1 or 0 end):soften(6)
blend(st, {angle=0}); blend(st, {angle=0.05}); blend(st, {angle=0})
local pr = ellipse(565,553,175,32):soften(6) - ellipse(575,550,90,7)
blend(pr, {angle=0})
-- hedge touches to re-cover the meadow's top edge
stipple(ribbon({{540,411},{700,409},{860,410},{1000,408}}, {5,6,6,7}), {pile=ridge3, width=5, coverage=1.6, feather=0.4, drag={4,0}, cluster={0.5,20}})
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{824,370},{825,400},{821,428}}, {pressure={0.7,0.8}})
tb:load(trk, 0.9); tb:stroke({{877,370},{879,400},{883,424}}, {pressure={0.5,0.6}})

--@ chunk 105
local g = mask(function(x,y) return y>421 and 1 or 0 end) - ellipse(575,550,95,9)
blend(g, {angle=0}); blend(g, {angle=0.2}); blend(g, {angle=-0.2}); blend(g, {angle=0})

--@ chunk 106
wait(9*24*60)
print(drying(300,520), drying(560,570), drying(700,430), drying(100,600))

--@ chunk 107
-- foreground glaze: umber/bitumen, heavier at bottom and corners, then blended across whole foreground
fgGl = pile{{"raw umber",2},{"bitumen",2},{"bone black",0.8},{"raw sienna",1}, medium=0.7}
local wat = ellipse(575,550,95,9)
local g = mask(function(x,y) return y>423 and 1 or 0 end) - wat
local n1 = noise{seed=351, period=140}
local function z(th) return mask(function(x,y) if y<440 then return 0 end; local t=(y-430)/(H-430); local s=math.abs(x-610)/520; return (s*s*0.9 + t*0.8 + 0.15*n1(x*0.4,y) > th) and 1 or 0 end) - wat:grow(4) end
local o = {hand="body", pile=fgGl, angle=0, tool="filbert 16", length={60,160}, coverage=0.8, load=0.3, clip=g}
work(z(0.55), o)
work(z(0.8), o)
-- oak shadow on the grass
o.coverage=0.7; work(poly({{0,426},{430,424},{590,430},{480,448},{280,462},{0,472}},true), o)
blend(g, {angle=0}); blend(g, {angle=0.15}); blend(g, {angle=-0.15}); blend(g, {angle=0})

--@ chunk 108
-- pool repainted directly
local w3 = outline{{438,552},{480,544},{540,540},{610,538},{672,541},{712,547},{680,555},{620,560},{540,563},{470,561}, char="soft", seed=97, lobe=18, closed=true, amount=0.6}:mask()
local ring = ellipse(575,551,170,26)
work(ring - w3, {hand="body", pile=bank, coverage=1.5, angle=0, tool="filbert 6", length={25,60}, fill=true, clip=-w3})
poolW = pile{{"lead white",8},{"yellow ochre",2},{"lemon chrome",0.8},{"orange chrome",0.5},{"raw umber",0.5},{"cobalt blue",0.3}, medium=0.15}
work(w3, {hand="body", pile=poolW, coverage=1.8, angle=0, tool="filbert 5", length={20,50}, fill=true, clip=w3})
work(ribbon({{560,548},{620,546},{670,548}}, {2,5,2}), {hand="body", pile=skyGP, coverage=1.2, angle=0, tool="filbert 4", length={15,35}, clip=w3})
work(poly({{438,552},{480,544},{520,542},{510,562},{470,561}},true), {hand="body", pile=bank, coverage=0.6, angle=0, tool="filbert 4", length={15,35}, clip=w3, load=0.3})
blend((ellipse(575,551,200,40) - w3:grow(2)):soften(8), {angle=0})
blend(w3:grow(6):soften(4), {angle=0})
-- near-bank dark and reeds
work(ribbon({{440,563},{540,569},{640,565},{700,556}}, {3,5,5,3}), {hand="body", pile=deep, coverage=1.0, angle=0, tool="filbert 4", length={15,40}, edge="lost"})
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(14, 445, 540, 0.6, 0.6, 5)) do
  rb:load(deep, 0.5); local y0 = 566+rand(-3,3)
  rb:stroke({{x, y0},{x+rand(-4,4), y0-rand(8,18)}}, {pressure={0.5,0.05}})
end
for i,x in ipairs(uneven(6, 670, 708, 0.6, 0.6, 6)) do
  rb:load(deep, 0.5); local y0 = 557+rand(-3,3)
  rb:stroke({{x, y0},{x+rand(-3,3), y0-rand(7,13)}}, {pressure={0.45,0.05}})
end
-- oak foot seam
local under = outline{{0,400},{100,405},{220,401},{340,405},{440,404},{500,409},{560,416},{520,428},{420,430},{300,436},{180,432},{60,438},{0,436}, char="soft", seed=99, lobe=18, closed=true}:mask()
work(under, {hand="body", pile=deepG, coverage=1.3, angle=0, tool="filbert 8", length={20,50}, edge="lost"})
blend(under:grow(10):soften(10), {angle=0.1})
-- hedge line
local n8 = noise{seed=361, period=40}
stipple(ribbon({{545,410},{700,408},{860,409},{1000,407}}, {7,8,8,9}):times(function(x,y) return clamp(n8(x,y*2)*2+0.9,0,1) end), {pile=ridge3, width=6, coverage=2.4, feather=0.3, drag={4,0}})

--@ chunk 109
blend(ribbon({{440,564},{540,569},{640,565},{700,557}}, 12):soften(4), {angle=0})
-- dark weight into the oak's lower half and foot
local n5 = noise{seed=371, period=60}
local lowoak = (tlM:shrink(6) + rect(0,380,520,45)):times(function(x,y) if x>560 then return 0 end; return clamp((y-250)/110,0,1)*clamp(0.75+0.4*n5(x,y),0,1) end)
work(lowoak, {hand="body", pile=shadeGl, coverage=1.0, tool="filbert 14", length={20,50}, angle=function(x,y) return 0.4*n5(x+90,y) end, load=0.45, threshold=0.25, clip=(tlM:shrink(3) + rect(0,380,540,48)):soften(4)})
local bm = (tlM:shrink(8) + rect(0,385,520,36)):times(function(x,y) return (y>240 and x<545) and 1 or 0 end):soften(10)
blend(bm, {angle=0.8}); blend(bm, {angle=-0.3})

--@ chunk 110
local r = rag{width=12}; r:dip(0.9)
for k=1,3 do
  r:wipe({{540,381},{500,380},{470,378}}, {pressure=0.9}); r:refold(); r:dip(0.9)
  r:wipe({{545,392},{510,390},{480,386}}, {pressure=0.9}); r:refold(); r:dip(0.9)
end

--@ chunk 111
local oak = tlM + seamM
local keep = -(oak:shrink(3))
local function hills(pts, seed, lobe, bottom)
  local p = {}
  for i,q in ipairs(pts) do p[#p+1]=q end
  p[#p+1]={1000,bottom}; p[#p+1]={pts[1][1],bottom}
  return outline{pts=p, char="soft", seed=seed, lobe=lobe, closed=true, edge=4}:mask() * keep
end
local far = hills({{420,378},{470,370},{520,360},{570,366},{620,356},{665,346},{705,342},{745,352},{780,364},{830,358},{880,346},{925,340},{965,346},{1000,350}}, 51, 18, 416)
work(far, {hand="body", pile=ridgeP, coverage=2.0, angle=0, tool="filbert 6", length={12,30}, fill=true, clip=far})
local nearw = hills({{420,396},{480,388},{520,381},{560,388},{600,383},{650,376},{700,371},{750,379},{800,387},{850,379},{900,371},{950,367},{1000,371}}, 53, 13, 416)
work(nearw, {hand="body", pile=ridge2, coverage=2.0, angle=0, tool="filbert 5", length={10,26}, fill=true, clip=nearw})
local hedge = hills({{420,408},{500,403},{540,398},{580,404},{640,400},{690,404},{740,397},{790,394},{840,400},{900,396},{960,392},{1000,394}}, 55, 9, 417)
work(hedge, {hand="body", pile=ridge3, coverage=2.0, angle=0, tool="filbert 4", length={8,20}, fill=true, clip=hedge})
-- soften each crest with a narrow horizontal blend along it
blend(ribbon({{420,372},{520,360},{620,356},{705,342},{780,364},{880,346},{1000,350}}, 14):soften(5) * keep, {angle=0})
blend(ribbon({{420,394},{520,382},{650,377},{700,372},{800,387},{900,372},{1000,371}}, 10):soften(4) * keep, {angle=0})
blend(ribbon({{420,406},{540,399},{690,403},{790,395},{1000,394}}, 8):soften(3) * keep, {angle=0})

--@ chunk 112
local r = rag{width=22}; r:dip(0.9)
for k=1,3 do
  r:wipe({{1000,433},{800,433},{600,433},{480,432}}, {pressure=0.9}); r:refold(); r:dip(0.9)
  r:wipe({{1000,446},{800,446},{700,444}}, {pressure=0.8}); r:refold(); r:dip(0.9)
end

--@ chunk 113
wait(6*24*60)
print(drying(700,380), drying(700,410), drying(300,350))
-- meadow far edge
local mclip = mask(function(x,y) return y>419 and 1 or 0 end)
work(mask(function(x,y) return (x>470 and y>420 and y<436) and 1 or 0 end), {hand="body", pile=grassM, coverage=1.5, angle=0, tool="filbert 6", length={40,100}, fill=true, clip=mclip, edge="lost"})
work(mask(function(x,y) return (x>620 and y>421 and y<430) and 1 or 0 end), {hand="body", pile=grassF, coverage=0.8, angle=0, tool="filbert 5", length={40,100}, clip=mclip, load=0.4})
blend(mask(function(x,y) return (x>450 and y>424 and y<446) and 1 or 0 end):soften(5), {angle=0})
-- hedgerow trees along the meadow's far edge: rounded dark clumps of varied height
local n8 = noise{seed=381, period=34}
local hx = ribbon({{520,412},{600,411},{700,412},{800,410},{900,411},{1000,409}}, {10,12,10,14,12,14}):times(function(x,y) return clamp((n8(x,y*1.6)+0.2)*2.4,0,1) end)
stipple(hx, {pile=hedgeC, width=6, coverage=2.6, feather=0.3, drag={3,0}, cluster={0.4,16}})
stipple(ribbon({{500,418},{1000,417}}, 5), {pile=hedgeC, width=5, coverage=2.2, feather=0.3, drag={5,0}})
-- trunks
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{822,330},{825,385},{821,430}}, {pressure={0.65,0.8}})
tb:load(trk, 0.9); tb:stroke({{875,325},{878,385},{883,426}}, {pressure={0.45,0.6}})
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(12, 805, 900, 0.6, 0.6, 5)) do
  rb:load(bank, 0.5)
  rb:stroke({{x, 432+rand(-3,3)},{x+rand(-3,3), 432-rand(5,10)}}, {pressure={0.5,0.05}})
end

--@ chunk 114
wait(8*24*60)
print(drying(700,380), drying(700,412), drying(700,430), drying(560,552), drying(300,350))

--@ chunk 115
-- hedgerow
local n8 = noise{seed=391, period=34}
local hx = ribbon({{500,411},{600,410},{700,411},{800,409},{900,410},{1000,408}}, {10,12,10,14,12,14}):times(function(x,y) return clamp((n8(x,y*1.6)+0.2)*2.4,0,1) end)
stipple(hx, {pile=hedgeC, width=6, coverage=2.4, feather=0.3, drag={3,0}, cluster={0.4,16}})
stipple(ribbon({{480,418},{1000,417}}, 5), {pile=hedgeC, width=5, coverage=2.0, feather=0.3, drag={5,0}})
-- oak's right foot: leafy clumps over the hard diagonal
local n5 = noise{seed=392, period=30}
local n6 = noise{seed=393, period=11}
local foot = outline{{380,350},{430,345},{460,372},{500,392},{540,410},{500,424},{400,424}, char="soft", seed=101, lobe=16, closed=true}:mask()
stipple(foot:times(function(x,y) return 0.5+0.5*n6:at01(x,y) end), {pile=leafDD, width=6, coverage=2.2, cluster={0.4,14}, feather=0.5, drag={3,0.5}})
stipple((foot:grow(10) - foot:shrink(4)):times(function(x,y) return clamp((n5(x,y)+0.2)*2,0,1)*n6:at01(x,y) end), {pile=leafDD, width=4, coverage=1.2, cluster={0.6,12}, feather=0.6, drag={2,0.5}})
-- warm leaves at the sunward rim, sparse
local warm = (tlM+foot):rim(26, 8):times(function(x,y) return (x>330 and y>150 and y<410) and clamp((n5(x+70,y)+0.1)*2,0,1)*n6:at01(x,y) or 0 end)
stipple(warm, {pile=leafW, width=4, coverage=0.7, cluster={0.6,12}, feather=0.7, drag={2,0.5}})
-- pool: darken bank ring with a glaze, keep water
local w3 = outline{{438,552},{480,544},{540,540},{610,538},{672,541},{712,547},{680,555},{620,560},{540,563},{470,561}, char="soft", seed=97, lobe=18, closed=true, amount=0.6}:mask()
ringGl = pile{{"raw umber",2},{"bitumen",1.5},{"raw sienna",1},{"bone black",0.5}, medium=0.6}
local ring = ellipse(575,553,205,42) - w3:grow(1)
work(ring, {hand="body", pile=ringGl, coverage=0.9, angle=0, tool="filbert 8", length={30,80}, load=0.35, clip=-(w3:grow(1)), edge="lost"})
blend(ring:grow(14):soften(10) - w3:grow(2), {angle=0})
-- warm the water slightly
work(w3, {hand="glaze", pile=pile{{"raw sienna",1},{"yellow ochre",1},{"orange chrome",0.3}, medium=0.93}, tool="filbert 6", length={20,60}, angle=0, clip=w3, load=0.15, coverage=0.6})

--@ chunk 116
-- meadow patch over old shadow tip and residue streak
local o = {hand="body", angle=0, tool="filbert 10", length={50,130}, coverage=1.3, edge="lost", clip=mask(function(x,y) return y>421 and 1 or 0 end)}
o.pile=grassM; work(poly({{470,440},{660,438},{850,446},{850,470},{470,480}},true), o)
o.pile=deepG; o.coverage=0.9; work(poly({{0,428},{440,426},{540,432},{430,448},{250,460},{0,468}},true), o)
local mb = mask(function(x,y) return (y>426 and y<500) and 1 or 0 end):soften(10)
blend(mb, {angle=0}); blend(mb, {angle=0.04})
-- oak: break the band between olive crown and dark base
local n5 = noise{seed=401, period=50}
local n6 = noise{seed=402, period=13}
local T = tlM:shrink(8)
local trans = T:times(function(x,y) local c = 255 + 40*n5(x,0); return clamp(1-math.abs(y-c)/55,0,1)*(0.3+0.7*n6:at01(x,y)) end)
stipple(trans, {pile=treeD, width=10, coverage=1.3, cluster={0.4,22}, feather=0.5, drag={5,0.9}})
local up = T:times(function(x,y) return y<250 and clamp((n5(x+100,y+50)+0.05)*2.2,0,1)*(0.4+0.6*n6:at01(x,y)) or 0 end)
stipple(up, {pile=treeD, width=9, coverage=0.9, cluster={0.4,22}, feather=0.5, drag={5,0.9}})
local lo = T:times(function(x,y) return (y>270 and y<390 and x>120) and clamp((n5(x+300,y+150)-0.1)*2.2,0,1)*(0.4+0.6*n6:at01(x,y)) or 0 end)
stipple(lo, {pile=treeM, width=8, coverage=0.5, cluster={0.4,20}, feather=0.6, drag={4,0.9}})
blend(T:times(function(x,y) return (y>190 and y<330) and 1 or 0 end):soften(15), {angle=1.2})

--@ chunk 117
-- undergrowth breaking the straight seam under the oak
local n6 = noise{seed=412, period=18}
local seam = ribbon({{0,422},{150,421},{300,423},{440,421},{540,420}}, {10,12,9,12,6}):times(function(x,y) return clamp(n6(x,y)*2+0.7,0,1) end)
stipple(seam, {pile=deepG, width=7, coverage=1.5, cluster={0.4,16}, feather=0.5, drag={5,0}})
local rb = brush{kind="round", width=3.5, point=0.9}
for i,x in ipairs(uneven(44, 6, 545, 0.7, 0.6, 9)) do
  rb:load(deepG, 0.5)
  local y0 = 428+rand(-4,7)
  rb:stroke({{x, y0},{x+rand(-4,4), y0-rand(7,15)}}, {pressure={0.55,0.05}})
end
-- gold lights on far meadow
local gb = brush("filbert", 5)
for i,p in ipairs{{600,427,50},{690,425,60},{770,428,40},{650,438,50},{730,443,36},{900,430,50}} do
  gb:load(gold, 0.45)
  gb:stroke({{p[1],p[2]},{p[1]+p[3],p[2]+rand(-1,1)}}, {pressure={0.45,0.1}})
end
-- sun
local sb = brush("round", 7)
sb:load(pile{{"lead white",10},{"lemon chrome",1.5},{"orange chrome",0.5}}, 0.8)
sb:touch(642,330,{pressure=0.7})
-- figure on the meadow, between pool and slim trees
fig = brush{kind="round", width=3.5, point=0.6}
fig:load(pile{{"bone black",2},{"raw umber",2},{"cobalt blue",0.5}}, 0.7)
fig:stroke({{742,462},{743,477}}, {pressure={0.7,0.9}})
fig:load(pile{{"Indian red",2},{"red earth",1.5},{"raw umber",0.3}}, 0.6)
fig:stroke({{742,457},{742.5,465}}, {pressure={0.6,0.8}})
fig:load(pile{{"lead white",3},{"yellow ochre",1},{"red earth",0.5}}, 0.5)
fig:touch(742,453.5,{pressure=0.35})
-- signature
sg = brush{kind="rigger", width=1.6, point=1}
local sp = pile{{"red earth",2},{"raw umber",1},{"yellow ochre",1}, medium=0.3}
local function L(pts) sg:load(sp,0.6); sg:stroke(pts,{pressure=0.4}) end
local x0,y0=905,640
L({{x0,y0-9},{x0+5,y0-9},{x0+2.5,y0-9},{x0+2.5,y0}})          -- (mark) F-like monogram stem
L({{x0+9,y0-9},{x0+9,y0}}) L({{x0+9,y0-9},{x0+14,y0-9}}) L({{x0+9,y0-5},{x0+13,y0-5}})
L({{x0+18,y0},{x0+18.5,y0-0.5}})

--@ chunk 118
blend(mask(function(x,y) return (x>570 and y>421 and y<450) and 1 or 0 end):soften(5) - ellipse(742,465,6,14), {angle=0})
local sm = mask(function(x,y) return (x<560 and y>412 and y<440) and 1 or 0 end):soften(5)
blend(sm, {angle=1.4}); blend(sm, {angle=0.1})
-- shadow of oak restated softly, wider
work(poly({{0,424},{470,422},{560,426},{470,440},{280,452},{0,460}},true), {hand="body", pile=deepG, coverage=1.0, angle=0, tool="filbert 10", length={50,120}, edge="lost", clip=mask(function(x,y) return y>416 and 1 or 0 end)})
blend(mask(function(x,y) return (x<640 and y>414 and y<475) and 1 or 0 end):soften(10), {angle=0.03})

--@ chunk 119
tb = brush{kind="round", width=6, point=0.8}
tb:load(trk, 0.9); tb:stroke({{824,385},{824,410},{821,432}}, {pressure={0.75,0.85}})
tb:load(trk, 0.9); tb:stroke({{878,385},{880,410},{883,428}}, {pressure={0.55,0.65}})
local rb = brush{kind="round", width=3, point=0.9}
for i,x in ipairs(uneven(12, 806, 898, 0.6, 0.6, 5)) do
  rb:load(bank, 0.5)
  rb:stroke({{x, 434+rand(-3,3)},{x+rand(-3,3), 434-rand(5,10)}}, {pressure={0.5,0.05}})
end
