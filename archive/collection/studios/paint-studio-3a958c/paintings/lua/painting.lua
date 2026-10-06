-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=760, aspect=1.4, linen={16,16}, seed=1824,
  ground={
    {pile={{"red earth",3},{"yellow ochre",2},{"lead white",4}}, um=60, apply="knife", texture=0.25},
    {pile={{"lead white",10},{"yellow ochre",1},{"raw umber",0.3}}, um=35, apply="roller", texture=0.3}
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
h = pencil("2H")
-- far range
far = {{0,396},{70,391},{140,395},{210,388},{280,392},{350,397},{430,401},{520,404},{600,406},{680,402},{745,394}}
h:line(far, {pressure=0.22})
-- tabletop rock at right
rock = {{735,482},{742,430},{750,380},{752,352},{760,337},{800,332},{850,330},{868,338},{873,362},{879,402},{890,442},{905,482}}
h:line(rock, {pressure=0.25, smooth=false})
-- far range continuing behind and right of the rock
farR = {{905,396},{950,388},{1000,384}}
h:line(farR, {pressure=0.2})
-- mid hill with spire
midhill = {{480,505},{520,486},{560,470},{592,464},{640,468},{690,482},{740,505}}
h:line(midhill, {pressure=0.22})
-- foreground crest
crest = {{0,566},{80,556},{170,534},{240,524},{330,530},{430,545},{540,566},{650,596},{760,632},{860,668},{940,700},{1000,714}}
h:line(crest, {pressure=0.3})
print("ok")

--@ chunk 3
h2 = pencil("HB")
-- oak skeleton (center lines)
trunk = {{242,532},{238,500},{240,470},{235,445},{236,428}}
L1  = {{236,430},{212,402},{186,380},{160,350},{140,318},{118,282},{96,246},{80,210},{70,170}}
L1a = {{160,350},{130,345},{100,335},{72,338},{45,330}}
L1b = {{118,282},{140,250},{150,215},{146,180},{155,140}}
L1c = {{96,246},{70,240},{45,225}}
L2  = {{236,430},{244,398},{258,365},{256,332},{246,300},{254,262},{266,226},{260,190},{268,150},{262,112},{270,78}}
L2a = {{256,332},{222,308},{196,292},{170,276}}
L2b = {{254,262},{292,246},{320,226},{340,196},{350,160}}
L2c = {{266,226},{238,205},{222,172},{208,140}}
L2d = {{260,190},{292,170},{310,138},{320,100}}
L3  = {{238,440},{275,424},{318,412},{356,396},{392,376},{420,350},{440,318},{448,285},{445,250}}
L3b = {{392,376},{372,340},{360,304},{366,268},{356,236}}
L3c = {{420,350},{452,338},{488,322},{520,318}}
L3d = {{440,318},{470,290},{484,260},{480,225}}
for _, pts in ipairs({trunk,L1,L1a,L1b,L1c,L2,L2a,L2b,L2c,L2d,L3,L3b,L3c,L3d}) do
  h2:line(pts, {pressure=0.3})
end
print("ok")

--@ chunk 4
skyTop   = pile{{"lead white",4},{"smalt",3},{"cobalt blue",1}, medium=0.25}
skyMid   = pile{{"lead white",6},{"pale smalt",2},{"Prussian blue",0.15},{"yellow ochre",0.3}, medium=0.25}
skyGreen = pile{{"lead white",6},{"chrome yellow",0.6},{"Prussian blue",0.2}, medium=0.25}
skyGlow  = pile{{"lead white",6},{"chrome yellow",1},{"vermilion",0.15}, medium=0.2}
cloudWarm= pile{{"lead white",5},{"vermilion",0.5},{"yellow ochre",0.6}, medium=0.2}
cloudCool= pile{{"lead white",5},{"smalt",1.5},{"red earth",0.3},{"raw umber",0.2}, medium=0.2}
piles = {skyTop, skyMid, skyGreen, skyGlow, cloudWarm, cloudCool}
for i, p in ipairs(piles) do
  local x0 = 15 + (i-1) * 85
  local m = rect(x0, 640, 75, 50)
  work(m, {hand="body", pile=p, coverage=1.3, angle=0.1, clip=true})
end
print("done", wait(0))

--@ chunk 5
S1 = pile{{"lead white",3.5},{"smalt",3},{"cobalt blue",1.2}, medium=0.3}
S2 = pile{{"lead white",5},{"smalt",3},{"cobalt blue",0.6}, medium=0.3}
S3 = pile{{"lead white",6},{"pale smalt",2.5},{"cobalt blue",0.2},{"Prussian blue",0.05},{"yellow ochre",0.15}, medium=0.3}
S4 = pile{{"lead white",6},{"pale smalt",1},{"chrome yellow",0.5},{"Prussian blue",0.12}, medium=0.3}
S5 = pile{{"lead white",6},{"chrome yellow",0.8},{"vermilion",0.1},{"yellow ochre",0.3}, medium=0.3}
bands = {
  {rect(0,-20,1000,150), S1},
  {rect(0,100,1000,130), S2},
  {rect(0,200,1000,120), S3},
  {rect(0,290,1000,100), S4},
  {rect(0,360,1000,80),  S5},
}
for i, b in ipairs(bands) do
  work(b[1], {hand="broad", pile=b[2], angle=0.02, coverage=2.2, angle_jitter=0.05})
end
print(wait(0))

--@ chunk 6
blend(rect(0,0,500,440), {angle=0.0})
print(wait(0))

--@ chunk 7
work(rect(0,0,1000,140), {hand="glaze", pile=S1, angle=0.0, coverage=2.5})
work(rect(0,110,1000,120), {hand="glaze", pile=S2, angle=0.0, coverage=2.5})
print(wait(0))

--@ chunk 8
work(rect(0,200,1000,110), {hand="glaze", pile=S3, angle=0.0, coverage=2.5})
work(rect(0,280,1000,110), {hand="glaze", pile=S4, angle=0.0, coverage=2.5})
glowM = ellipse(620, 410, 430, 120):blur(60)
work(glowM, {hand="glaze", pile=S5, angle=0.0, coverage=3, clip=true})
print(wait(0))

--@ chunk 9
print(drying(500,50), drying(500,300), drying(600,420), drying(40,660))

--@ chunk 10
S6 = pile{{"lead white",5},{"vermilion",0.3},{"yellow ochre",0.4},{"chrome yellow",0.5}, medium=0.35}
hz = ellipse(640, 428, 400, 46):blur(28)
work(hz, {hand="glaze", pile=S6, angle=0.0, coverage=2.2, clip=true})
print(wait(0))

--@ chunk 11
print(wait(180))
print(drying(500,50), drying(500,300), drying(600,420), drying(40,660))

--@ chunk 12
print(wait(240))
print(drying(500,50), drying(500,300), drying(600,420), drying(40,660))

--@ chunk 13
print(wait(24*60))
print(drying(500,50), drying(500,300), drying(600,420), drying(40,660))

--@ chunk 14
R1c = {{0,392},{60,384},{110,377},{160,375},{210,382},{260,391},{310,395},{370,390},{420,396},{480,402},{560,406},{640,408},{720,402},{780,394},{830,388},{900,380},{950,372},{1000,370}}
local pts = {}
for _, p in ipairs(R1c) do pts[#pts+1] = p end
pts[#pts+1] = {1000, 480}
pts[#pts+1] = {0, 480}
R1m = poly(pts, true)
R1pile = pile{{"lead white",5},{"smalt",1.2},{"red earth",0.3},{"yellow ochre",0.2}, medium=0.3}
work(R1m, {hand="broad", pile=R1pile, angle=0.02, coverage=2.5, clip=true})
print(wait(0))

--@ chunk 15
R1b = pile{{"lead white",4},{"smalt",2},{"red earth",0.2},{"raw umber",0.1}, medium=0.3}
-- lighter toward the bottom: load fades below y=430
work(R1m, {hand="glaze", pile=R1b, angle=0.0, coverage=2.5, clip=true,
  load_at=function(x,y) return clamp(1.0 - (y-392)/110, 0.25, 1.0) end})
print(wait(0))

--@ chunk 16
rockPts = {{690,520},{700,500},{718,470},{730,440},{738,412},{744,386},{746,360},{749,346},{756,338},{768,336},{790,333},{815,334},{838,331},{858,332},{866,337},{869,350},{871,380},{876,400},{884,425},{900,452},{925,470},{960,488},{1000,500},{1000,540},{690,540}}
rockM = poly(rockPts):roughen(1.6, 18, 7)
RockD = pile{{"lead white",1.5},{"smalt",2.5},{"raw umber",0.6},{"red earth",0.4},{"cobalt blue",0.3}, medium=0.15}
work(rockM, {hand="body", pile=RockD, angle=1.4, coverage=3, clip=true, length={20,60}})
print(wait(0))

--@ chunk 17
RockD2 = pile{{"raw umber",2},{"Prussian blue",0.45},{"red earth",0.6},{"lead white",1.2},{"smalt",1}, medium=0.05}
work(rockM, {hand="body", pile=RockD2, angle=1.45, coverage=4, clip=true, length={20,60}, fill=true, load=1.0})
print(wait(0))

--@ chunk 18
RockG = pile{{"raw umber",1},{"Prussian blue",0.5},{"smalt",1},{"red earth",0.3}, medium=0.5}
work(rockM, {hand="glaze", pile=RockG, angle=1.45, coverage=2.5, clip=true,
  load_at=function(x,y) return clamp((480 - y)/140, 0.0, 1.0) end})
print(wait(0))

--@ chunk 19
nz = noise{seed=11, period=170, octaves=3, persistence=0.5}
MistC = pile{{"lead white",6},{"pale smalt",0.5},{"yellow ochre",0.3},{"red earth",0.1}, medium=0.4}
MistW = pile{{"lead white",6},{"yellow ochre",0.4},{"chrome yellow",0.25},{"vermilion",0.08}, medium=0.4}
m1 = mask(function(x,y)
  local top = 448 + 22*nz(x, 0)
  return smoothstep(top-22, top+28, y) * (1 - smoothstep(505, 545, y))
end)
work(m1, {hand="glaze", pile=MistC, angle=0.0, coverage=3, clip=true})
mW = mask(function(x,y)
  local top = 448 + 22*nz(x, 0)
  local g = math.exp(-((x-620)/330)^2)
  return g * smoothstep(top-22, top+28, y) * (1 - smoothstep(505, 545, y))
end)
work(mW, {hand="glaze", pile=MistW, angle=0.0, coverage=3, clip=true})
print(wait(0))

--@ chunk 20
MistW2 = pile{{"lead white",6},{"yellow ochre",0.35},{"chrome yellow",0.15},{"vermilion",0.06}, medium=0.2}
m1b = mask(function(x,y)
  local top = 462 + 18*nz(x+300, 40)
  return smoothstep(590, 700, x) * smoothstep(top-20, top+30, y) * (1 - smoothstep(535, 575, y))
end)
work(m1b, {hand="broad", pile=MistW2, angle=0.0, coverage=4, clip=true, angle_jitter=0.08})
print(wait(0))

--@ chunk 21
print(wait(20*60))
print(drying(800,450), drying(800,520), drying(300,50), drying(40,660))

--@ chunk 22
print(wait(10*60))
print(drying(800,450), drying(800,520), drying(300,50), drying(40,660))

--@ chunk 23
islA_pts = {{430,545},{452,520},{478,503},{505,490},{535,478},{566,470},{598,467},{630,470},{662,478},{690,490},{716,506},{745,525},{770,550},{770,600},{430,600}}
islA = poly(islA_pts, true):roughen(1.2, 14, 3)
IA = pile{{"lead white",3.5},{"smalt",1.8},{"red earth",0.25},{"raw umber",0.15}, medium=0.25}
work(islA, {hand="body", pile=IA, angle=0.1, coverage=3, clip=true, length={25,70}, fill=true})
print(wait(0))

--@ chunk 24
IB = pile{{"lead white",1.5},{"smalt",2.2},{"raw umber",0.5},{"red earth",0.3},{"Prussian blue",0.1}, medium=0.45}
work(islA, {hand="glaze", pile=IB, angle=0.05, coverage=2.5, clip=true,
  load_at=function(x,y) return clamp(1.0 - (y-470)/70, 0.0, 1.0) end})
-- mist over its base (soft top), warm cream, fairly opaque
MistO = pile{{"lead white",7},{"yellow ochre",0.35},{"chrome yellow",0.1},{"vermilion",0.05}, medium=0.08}
mB = mask(function(x,y)
  local top = 515 + 12*nz(x+90, 70)
  return smoothstep(top-26, top+22, y) * smoothstep(400, 470, x) * (1 - smoothstep(560, 600, y))
end)
work(mB, {hand="broad", pile=MistO, angle=0.0, coverage=3, clip=true, angle_jitter=0.06})
print(wait(0))

--@ chunk 25
crestPts = {{-20,567},{0,566},{40,562},{80,555},{125,545},{170,534},{210,528},{245,525},{290,527},{335,531},{385,538},{430,544},{480,553},{540,566},{600,580},{650,596},{705,613},{760,632},{815,650},{860,668},{905,686},{945,702},{980,712},{1020,720}}
local pts = {}
for _, p in ipairs(crestPts) do pts[#pts+1] = p end
pts[#pts+1] = {1020, 740}
pts[#pts+1] = {-20, 740}
fgM = poly(pts, true):roughen(1.5, 22, 5)
F1 = pile{{"raw umber",3},{"Prussian blue",0.7},{"red earth",0.6},{"lead white",0.3}, medium=0.1}
work(fgM, {hand="body", pile=F1, angle=function(x,y) return 0.12 end, coverage=3.2, clip=true, length={30,90}, fill=true})
print(wait(0))

--@ chunk 26
print(wait(6*60))
print(drying(560,560), drying(600,520), drying(200,600), drying(800,400))

--@ chunk 27
tp = pile{{"lead white",4},{"yellow ochre",1}, medium=0.2}
bt = brush{kind="round", width=8, point=1}
print("bt widths", bt:mark_width(0.1), bt:mark_width(0.3), bt:mark_width(0.6), bt:mark_width(1.0))
br = brush{kind="rigger", width=2, point=1}
print("br widths", br:mark_width(0.1), br:mark_width(0.3), br:mark_width(0.6), br:mark_width(1.0))
bm = brush{kind="round", width=3, point=1}
print("bm widths", bm:mark_width(0.1), bm:mark_width(0.3), bm:mark_width(0.6), bm:mark_width(1.0))
print("pressure_for 4:", bt:pressure_for(4), "pressure_for 1:", br:pressure_for(1), "for 0.5:", br:pressure_for(0.5))

--@ chunk 28
bt:load(tp, 1.0)
bt:stroke({{30,700},{80,690},{140,672},{200,660},{260,640}}, {pressure={1.0, 0.15}, ramps={0.02, 0.1}})
bt:stroke({{30,650},{60,648},{100,652},{150,646}}, {pressure={0.6, 0.6}})
br:load(tp, 1.0)
br:stroke({{40,630},{90,625},{130,632},{180,622},{230,628}}, {pressure={1.0, 0.2}, ramps={0.02,0.1}})
br:stroke({{200,700},{230,690},{250,670},{280,664}}, {pressure={0.7, 0.1}})
bm:load(tp, 1.0)
bm:stroke({{300,700},{330,690},{370,660},{420,650}}, {pressure={1.0, 0.1}})
print(bt:fullness(), br:fullness(), bm:fullness())

--@ chunk 29
o = body_of{spine={{300,712},{296,690},{300,668},{296,648}}, widths={16,13,11,9},
  limbs={ {{296,690},{330,672},{360,664},{392,650}, widths={9,7,5,3}},
          {{300,668},{270,650},{248,640},{226,620}, widths={8,6,4,2}} },
  blend=0.8, char="firm"}
print(o:length(), o:mask():area())
work(o:mask(), {hand="body", pile=tp, coverage=2, clip=true, angle=-1.4})

--@ chunk 30
skyM = rect(0,0,1000,470) - R1m - rockM
T1 = pile{{"smalt",3},{"cobalt blue",1},{"lead white",1.2}, medium=0.55}
topFade = mask(function(x,y) return 1 - smoothstep(30, 250, y) end)
work(skyM * topFade, {hand="glaze", pile=T1, angle=0.0, coverage=2.5, clip=true,
  load_at=function(x,y) return clamp(1.1 - y/260, 0.15, 1.0) end})
print(wait(0))

--@ chunk 31
blend(rect(0,0,1000,290) - R1m - rockM, {angle=0.0})
blend(rect(0,0,1000,290) - R1m - rockM, {angle=0.03})
print(wait(0))

--@ chunk 32
G1 = pile{{"lead white",5},{"Prussian blue",0.16},{"chrome yellow",0.55},{"pale smalt",1}, medium=0.45}
bandG = mask(function(x,y) return smoothstep(215, 262, y) * (1 - smoothstep(300, 345, y)) end)
work(skyM * bandG, {hand="glaze", pile=G1, angle=0.0, coverage=2.4, clip=true})
blend(rect(0,200,1000,200) - R1m - rockM, {angle=0.0})
print(wait(0))

--@ chunk 33
blend(rect(0,150,1000,260) - R1m - rockM, {angle=0.0})
blend(rect(0,150,1000,260) - R1m - rockM, {angle=0.02})
print(wait(0))

--@ chunk 34
PW = pile{{"lead white",6},{"red earth",0.2},{"vermilion",0.08},{"pale smalt",1.2}, medium=0.45}
bandP = mask(function(x,y) return math.exp(-((y-262)/52)^2) end)
work(skyM * bandP, {hand="glaze", pile=PW, angle=0.0, coverage=2.0, clip=true})
-- merge the whole sky wet-in-wet, softly (full sky so no seam)
blend(skyM, {angle=0.0})
print(wait(0))

--@ chunk 35
B2 = pile{{"lead white",3},{"smalt",2},{"cobalt blue",0.5}, medium=0.5}
fadeB = mask(function(x,y) return smoothstep(110, 160, y) * (1 - smoothstep(250, 330, y)) end)
work(skyM * fadeB, {hand="glaze", pile=B2, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(1.0 - (y-130)/190, 0.0, 1.0) end})
blend(skyM, {angle=0.0})
print(wait(0))

--@ chunk 36
print(drying(300,50), drying(300,250), drying(300,380), drying(800,400), drying(500,560), drying(600,520))

--@ chunk 37
function lens(x0, x1, yc, th, sag, soft)
  local hw = (x1-x0)/2
  local xm = (x0+x1)/2
  return mask(function(x,y)
    local t = (x-xm)/hw
    if math.abs(t) >= 1 then return 0 end
    local prof = (1 - t*t)^0.6
    local c = yc + (sag or 0)*t*t
    local d = math.abs(y - c)
    local e = th*prof
    return 1 - smoothstep(e*(1-(soft or 0.5)), e, d)
  end)
end
CoolC = pile{{"lead white",5},{"smalt",1.2},{"red earth",0.35},{"raw umber",0.15}, medium=0.5}
WarmC = pile{{"lead white",5},{"vermilion",0.45},{"yellow ochre",0.7},{"red earth",0.1}, medium=0.45}
-- cloud A: long low bar above the glow, left of the rock
cA = lens(260, 720, 322, 15, 4, 0.7) * skyM
work(cA, {hand="glaze", pile=CoolC, angle=0.0, coverage=1.6, clip=true, tool="flat 14"})
cAw = lens(300, 690, 331, 7, 3, 0.8) * skyM
work(cAw, {hand="glaze", pile=WarmC, angle=0.0, coverage=1.5, clip=true, tool="flat 10"})
-- cloud B: thinner bar, left
cB = lens(10, 300, 352, 9, 3, 0.7) * skyM
work(cB, {hand="glaze", pile=CoolC, angle=0.0, coverage=1.4, clip=true, tool="flat 10"})
cBw = lens(30, 270, 358, 5, 2, 0.8) * skyM
work(cBw, {hand="glaze", pile=WarmC, angle=0.0, coverage=1.4, clip=true, tool="flat 8"})
print(wait(0))

--@ chunk 38
IC = pile{{"smalt",2},{"raw umber",0.6},{"Prussian blue",0.12},{"lead white",1.2},{"red earth",0.2}, medium=0.3}
work(islA, {hand="body", pile=IC, angle=0.1, coverage=2.2, clip=true, length={25,60},
  load_at=function(x,y) return clamp(1.15 - (y-466)/70, 0.0, 1.0) end})
-- tree-line along the crest of the hill: small stippled dabs
rimTop = islA:rim(26, 14) * rect(0,0,1000,530)
stipple(rimTop, {pile=IC, width=3, coverage=1.6, feather=0.6, clip=islA, pressure={0.35,0.7}})
print(wait(0))

--@ chunk 39
blend(islA, {angle=0.05})
print(wait(0))

--@ chunk 40
function crestY(x)
  for i = 1, #crestPts-1 do
    local a, b = crestPts[i], crestPts[i+1]
    if x >= a[1] and x <= b[1] then
      local t = (x - a[1]) / (b[1]-a[1])
      return a[2] + t*(b[2]-a[2])
    end
  end
  return crestPts[#crestPts][2]
end
function crestAng(x, y)
  return math.atan((crestY(x+14) - crestY(x-14)) / 28)
end
F2 = pile{{"raw umber",3},{"red earth",1.1},{"Prussian blue",0.4},{"bone black",0.3}, medium=0.04}
work(fgM, {hand="body", pile=F2, angle=crestAng, coverage=3.6, clip=true, length={30,90}, fill=true, load=1.0})
print(wait(0))

--@ chunk 41
patch = ((rect(0,585,470,140) + rect(380,525,330,95)) * fgM)
work(patch, {hand="body", pile=F2, angle=crestAng, coverage=5, clip=true, length={20,70}, fill=true, load=1.0, tool="filbert 12"})
print(wait(0))

--@ chunk 42
print(wait(14*60))
print(drying(300,50), drying(300,250), drying(300,380), drying(800,450), drying(560,520), drying(200,600))

--@ chunk 43
nzz = noise{seed=23, period=140, octaves=3, persistence=0.55}
mNear = mask(function(x,y)
  local top = 540 + 16*nzz(x, 10) - 0.05*(x-500)
  return smoothstep(top-34, top+30, y) * smoothstep(380, 520, x)
end) * (-fgM)
MistN = pile{{"lead white",7},{"yellow ochre",0.3},{"chrome yellow",0.08},{"vermilion",0.06}, medium=0.12}
work(mNear, {hand="broad", pile=MistN, angle=0.0, coverage=3.2, clip=-fgM, angle_jitter=0.05,
  load_at=function(x,y) return 1.0 end})
print(wait(0))

--@ chunk 44
tpile = pile{{"lead white",4},{"yellow ochre",1}, medium=0.1}
-- test: a limb with side branches in the dark lower-left strip (will be overpainted)
tb = body_of{spine={{40,700},{80,688},{130,676},{190,668},{250,650}}, widths={16,13,10,7,4},
  limbs={ {{80,688},{100,660},{112,630},{126,600}, widths={7,5,4,2}},
          {{190,668},{214,640},{240,622},{268,614}, widths={5,4,3,1.5}} },
  blend=0.7, char="firm"}
work(tb:mask(), {hand="body", pile=tpile, coverage=2.5, clip=true, angle=-0.2, length={20,50}})
tb2 = body_of{spine={{300,700},{330,690},{372,680},{420,672},{470,650}}, widths={16,13,10,7,4},
  limbs={ {{330,690},{350,660},{360,630},{376,600}, widths={7,5,4,2}} },
  blend=0.7, char="broken"}
work(tb2:mask(), {hand="body", pile=tpile, coverage=2.5, clip=true, angle=-0.2, length={20,50}})
print(wait(0))

--@ chunk 45
-- Oak skeleton: each limb = {pts, widths}
OAK = {}
OAK.trunk = {{{244,538},{241,512},{239,486},{237,462},{236,440},{238,424}}, {66,53,47,45,44,40}}
OAK.L2 = {{{238,424},{244,398},{256,372},{262,346},{256,322},{246,300},{250,276},{262,252},{268,228},{262,204},{268,180},{274,156},{270,130},{276,104},{282,84}},
          {40,34,30,27,24,21,19,17,14,12,10,8,6,4,2.5}}
OAK.L1 = {{{236,446},{218,418},{198,396},{178,376},{166,350},{148,332},{128,312},{118,286},{100,266},{90,240},{80,214},{76,190},{70,166},{72,146}},
          {28,27,25,22,19,17,15,13,11,9,7,5,3.5,2}}
OAK.L3 = {{{240,448},{266,432},{298,424},{330,414},{360,398},{386,380},{410,364},{426,340},{432,314},{444,296},{446,270},{442,246},{446,222},{452,200}},
          {28,27,25,22,20,18,16,14,12,10,8,6,4,2.5}}
OAK.L4 = {{{242,466},{270,458},{304,455},{340,452},{378,447},{412,441},{446,436},{474,428},{498,426},{516,434},{526,448}},
          {24,21,19,17,15,13,11,9,7,4.5,2.5}}
OAK.L5 = {{{234,458},{206,450},{176,445},{146,439},{116,437},{88,439},{64,447},{42,459},{24,473},{12,488}},
          {24,21,18,16,13,11,9,6,4,2.5}}
function oakmask(char, amount)
  local m = nil
  for name, L in pairs(OAK) do
    local b = body_of{spine=L[1], widths=L[2], blend=0.8, char=char or "firm", amount=amount or 1.0}
    local bm = b:mask()
    if m == nil then m = bm else m = m + bm end
  end
  return m
end
oakM1 = oakmask("broken", 1.0)
print(oakM1:area())
h3 = pencil("2B")
h3:hatch(oakM1, {angle=-1.2, pressure=0.35, spacing=2.2})
print("hatched")

--@ chunk 46
erase(oakM1, {strength=1.0})
-- New oak skeleton (stouter, more irregular). Each entry: {pts, widths}
OAK = {}
OAK.trunk = {{{246,542},{243,516},{241,490},{240,466},{240,446},{242,428}}, {84,70,62,58,56,54}}
OAK.L2 = {{{242,432},{252,404},{268,380},{272,352},{258,328},{244,306},{248,282},{264,262},{270,236},{260,214},{262,190},{272,168},{276,144},{270,120},{272,98},{278,78}},
          {40,38,35,31,27,24,21,19,16,14,12,10,8,6,4,2.5}}
OAK.L1 = {{{238,446},{214,424},{190,408},{166,396},{146,374},{134,346},{120,322},{98,306},{80,282},{74,254},{82,226},{78,198},{70,172},{74,148},{80,126}},
          {36,34,31,28,24,21,18,16,13,11,9,7,5,3,2}}
OAK.L3 = {{{242,442},{270,428},{302,416},{332,398},{350,372},{372,352},{404,342},{432,336},{456,320},{470,296},{468,270},{480,246},{476,222},{486,200},{492,180}},
          {40,37,34,30,27,24,21,18,15,12,10,8,6,4,2.5}}
OAK.L4 = {{{242,472},{272,462},{306,458},{342,452},{380,446},{414,438},{448,436},{478,426},{504,424},{526,432},{540,446},{546,458}},
          {30,26,23,20,18,16,14,12,9,6,3.5,2}}
OAK.L5 = {{{236,464},{208,454},{178,448},{150,442},{122,440},{96,444},{70,452},{46,464},{28,478},{14,492}},
          {30,25,22,19,16,13,10,7,4.5,2.5}}
OAK.stub = {{{262,338},{290,320},{312,298},{324,280},{328,266}}, {17,14,11,9,8}}
OAK.S1 = {{{268,380},{296,368},{322,358},{346,336},{364,312},{386,298},{396,278},{400,262}}, {18,16,14,12,10,8,5,2.5}}
OAK.S2 = {{{258,328},{234,320},{210,306},{190,290},{172,268},{160,244},{156,226}}, {14,12,10,8,6,4,2}}
OAK.S3 = {{{264,262},{292,252},{318,240},{338,218},{352,194},{368,176},{374,160}}, {12,10,9,7,5,3,2}}
OAK.S4 = {{{262,190},{240,176},{222,152},{212,126},{200,106},{196,92}}, {9,7,6,4,2.5,1.5}}
OAK.S6 = {{{146,374},{170,356},{192,338},{210,316},{224,290},{226,264},{230,246}}, {14,12,10,8,6,4,2}}
OAK.S7 = {{{134,346},{112,356},{88,352},{66,340},{50,322},{40,300},{38,288}}, {12,10,8,6,4,2.5,1.5}}
OAK.S8 = {{{80,282},{62,268},{50,246},{46,222},{50,198},{54,184}}, {9,7,5,4,2.5,1.5}}
OAK.S9 = {{{82,226},{104,214},{124,196},{132,172},{146,152},{152,130},{156,116}}, {7,6,5,4,3,2,1.5}}
OAK.S10 = {{{350,372},{352,344},{366,318},{362,290},{374,264},{372,238},{376,224}}, {16,13,11,9,7,5,3}}
OAK.S11 = {{{432,336},{458,340},{486,336},{510,322},{530,304},{546,286},{552,274}}, {13,11,9,7,5,3,2}}
OAK.S12 = {{{470,296},{452,274},{440,248},{432,224},{424,200},{420,186}}, {9,7,5,3.5,2.5,1.5}}
OAK.S14 = {{{378,446},{384,420},{396,398},{394,372},{398,356}}, {12,10,8,6,3}}
OAK.S15 = {{{478,426},{492,404},{512,390},{532,384},{552,374},{564,368}}, {9,7,6,5,3,2}}
OAK.S16 = {{{122,440},{112,462},{100,480},{86,494},{78,502}}, {10,8,6,4,2}}
OAK.S17 = {{{70,452},{60,432},{46,416},{36,396},{32,386}}, {9,7,5,3.5,2}}
oakOrder = {"trunk","L2","L1","L3","L4","L5","stub","S1","S2","S3","S4","S6","S7","S8","S9","S10","S11","S12","S14","S15","S16","S17"}
function oakmask2(char, amount)
  local m = nil
  for _, name in ipairs(oakOrder) do
    local L = OAK[name]
    local b = body_of{spine=L[1], widths=L[2], blend=0.8, char=char or "firm", amount=amount or 1.0}
    local bm = b:mask()
    if m == nil then m = bm else m = m + bm end
  end
  return m
end
oakM2 = oakmask2("broken", 1.0)
h3 = pencil("2B")
h3:hatch(oakM2, {angle=-1.2, pressure=0.3, spacing=2.4})
print("hatched", oakM2:area())

--@ chunk 47
OakD  = pile{{"raw umber",3},{"Prussian blue",0.6},{"bone black",1.2},{"red earth",0.35}, medium=0.04}
function limbangle(L)
  local pts = L[1]
  return function(x, y)
    local best, ba = 1e12, 0
    for i = 1, #pts-1 do
      local ax, ay, bx, by = pts[i][1], pts[i][2], pts[i+1][1], pts[i+1][2]
      local dx, dy = bx-ax, by-ay
      local l2 = dx*dx + dy*dy
      local t = clamp(((x-ax)*dx + (y-ay)*dy) / l2, 0, 1)
      local px, py = ax + t*dx, ay + t*dy
      local d = (x-px)^2 + (y-py)^2
      if d < best then best = d; ba = math.atan(dy, dx) end
    end
    return ba
  end
end
function paintlimb(name, char, tool, cov)
  local L = OAK[name]
  local b = body_of{spine=L[1], widths=L[2], blend=0.8, char=char or "firm", amount=1.0}
  local m = b:mask()
  work(m, {hand="body", tool=tool or "filbert 9", pile=OakD, coverage=cov or 3.2, clip=true,
           angle=limbangle(L), length={25,60}, fill=true, load=1.0})
end
paintlimb("trunk", "broken", "filbert 12", 4)
paintlimb("L2", "broken", "filbert 9", 3.5)
paintlimb("L1", "broken", "filbert 9", 3.5)
paintlimb("L3", "broken", "filbert 9", 3.5)
paintlimb("L4", "broken", "filbert 9", 3.5)
paintlimb("L5", "broken", "filbert 9", 3.5)
print(wait(0))

--@ chunk 48
for _, name in ipairs({"stub","S1","S2","S3","S4","S6","S7","S8","S9","S10","S11","S12","S14","S15","S16","S17"}) do
  paintlimb(name, "firm", "filbert 7", 3.5)
end
print(wait(0))

--@ chunk 49
tw = brush{kind="round", width=6, point=1}
print("tw widths", tw:mark_width(0.05), tw:mark_width(0.2), tw:mark_width(0.4), tw:mark_width(0.6), tw:mark_width(0.8), tw:mark_width(1.0))
print("pf 0.6", tw:pressure_for(0.6), "pf 1", tw:pressure_for(1), "pf 2", tw:pressure_for(2), "pf 3", tw:pressure_for(3))
OakT = pile{{"raw umber",3},{"Prussian blue",0.55},{"bone black",1.0},{"red earth",0.4}, medium=0.15}

function shoot(x, y, ang, len, w0, depth, taper)
  local n = math.max(2, math.floor(len / 8))
  local seg = len / n
  local pts = {{x, y}}
  local angs = {}
  local a, cx, cy = ang, x, y
  for i = 1, n do
    a = a + rand(-0.32, 0.32) - 0.015 * math.sin(a) -- slight pull upward
    cx = cx + math.cos(a) * seg
    cy = cy + math.sin(a) * seg
    pts[#pts+1] = {cx, cy}
    angs[i] = a
  end
  local w1 = math.max(0.55, w0 * (taper or 0.25))
  tw:load(OakT, 0.8)
  tw:stroke(pts, {pressure={tw:pressure_for(w0), tw:pressure_for(w1)}, ramps={0.0, 0.0}})
  if depth > 0 then
    local k = (depth >= 2) and math.random(2, 3) or math.random(1, 3)
    for j = 1, k do
      local t = rand(0.3, 0.92)
      local i = clamp(math.floor(t * n) + 1, 1, n)
      local px, py = pts[i][1], pts[i][2]
      local side = ((j + math.random(0,1)) % 2 == 0) and 1 or -1
      local sa = angs[i] + side * rand(0.45, 1.0)
      local sl = len * rand(0.35, 0.65)
      shoot(px, py, sa, sl, math.max(0.7, w0 * rand(0.45, 0.7)), depth - 1, taper)
    end
  end
end

-- limb sprouting: walks along a limb spine and sprouts shoots from it
function sprout(name, from, to, spacing, wcap, depth, lenmin, lenmax)
  local L = OAK[name]
  local pts, wd = L[1], L[2]
  -- arclengths
  local s = {0}
  for i = 2, #pts do
    s[i] = s[i-1] + math.sqrt((pts[i][1]-pts[i-1][1])^2 + (pts[i][2]-pts[i-1][2])^2)
  end
  local total = s[#pts]
  local pos = total * from
  local flip = 1
  while pos < total * to do
    local i = 1
    while i < #pts-1 and s[i+1] < pos do i = i + 1 end
    local t = (pos - s[i]) / (s[i+1] - s[i])
    local px = pts[i][1] + t * (pts[i+1][1]-pts[i][1])
    local py = pts[i][2] + t * (pts[i+1][2]-pts[i][2])
    local la = math.atan(pts[i+1][2]-pts[i][2], pts[i+1][1]-pts[i][1])
    local wl = wd[i] + t * (wd[i+1]-wd[i])
    local sa = la + flip * rand(0.5, 1.05)
    flip = -flip
    shoot(px, py, sa, rand(lenmin, lenmax), clamp(wl * 0.4, 0.9, wcap), depth)
    pos = pos + spacing * rand(0.7, 1.3)
  end
  -- terminal fan at the tip
  local n = #pts
  local la = math.atan(pts[n][2]-pts[n-1][2], pts[n][1]-pts[n-1][1])
  for k = 1, 3 do
    shoot(pts[n][1], pts[n][2], la + (k-2) * rand(0.35, 0.6), rand(lenmin*0.7, lenmax*0.9), clamp(wd[n]*0.7, 0.9, wcap), depth)
  end
end

-- test on the left limb tip region
sprout("S8", 0.2, 1.0, 22, 2.6, 2, 24, 50)
sprout("S9", 0.2, 1.0, 22, 2.6, 2, 24, 50)
print(wait(0))

--@ chunk 50
function oakshoot(x, y, ang, len, w0, depth, opts)
  opts = opts or {}
  local pts = {{x, y}}
  local angs = {}
  local cx, cy, a = x, y, ang
  local used = 0
  local kink = opts.kink or 0.5
  while used < len do
    local l = math.min(rand(opts.lmin or 9, opts.lmax or 20), len - used)
    if l < 3.5 then break end
    a = a + rand(-kink, kink)
    cx = cx + math.cos(a) * l
    cy = cy + math.sin(a) * l
    used = used + l
    pts[#pts+1] = {cx, cy}
    angs[#angs+1] = a
  end
  if #pts < 2 then return end
  local w1 = math.max(0.55, w0 * 0.22)
  tw:load(OakT, 0.85)
  tw:stroke(pts, {pressure={tw:pressure_for(w0), tw:pressure_for(w1)}, ramps={0.0, 0.0}})
  if depth > 0 then
    local total = used
    local d = 0
    for i = 2, #pts do
      d = d + math.sqrt((pts[i][1]-pts[i-1][1])^2 + (pts[i][2]-pts[i-1][2])^2)
      if i < #pts or true then
        if math.random() < (opts.p or 0.6) then
          local side = (math.random() < 0.5) and 1 or -1
          local sa = angs[i-1] + side * rand(0.4, 0.95)
          local wl = w0 + (w1 - w0) * (d / total)
          local rem = total - d
          local sl = math.max(8, (rem + rand(6, 20)) * rand(0.45, 0.85))
          oakshoot(pts[i][1], pts[i][2], sa, sl, math.max(0.65, wl * 0.75), depth - 1, opts)
        end
      end
    end
  end
end

function sprout2(name, from, to, spacing, wcap, depth, lenmin, lenmax, opts)
  local L = OAK[name]
  local pts, wd = L[1], L[2]
  local s = {0}
  for i = 2, #pts do
    s[i] = s[i-1] + math.sqrt((pts[i][1]-pts[i-1][1])^2 + (pts[i][2]-pts[i-1][2])^2)
  end
  local total = s[#pts]
  local pos = total * from
  local flip = (math.random() < 0.5) and 1 or -1
  while pos < total * to do
    local i = 1
    while i < #pts-1 and s[i+1] < pos do i = i + 1 end
    local t = (pos - s[i]) / (s[i+1] - s[i])
    local px = pts[i][1] + t * (pts[i+1][1]-pts[i][1])
    local py = pts[i][2] + t * (pts[i+1][2]-pts[i][2])
    local la = math.atan(pts[i+1][2]-pts[i][2], pts[i+1][1]-pts[i][1])
    local wl = wd[i] + t * (wd[i+1]-wd[i])
    local sa = la + flip * rand(0.35, 0.85)
    flip = -flip
    oakshoot(px, py, sa, rand(lenmin, lenmax), clamp(wl * 0.42, 1.0, wcap), depth, opts)
    pos = pos + spacing * rand(0.7, 1.3)
  end
  local n = #pts
  local la = math.atan(pts[n][2]-pts[n-1][2], pts[n][1]-pts[n-1][1])
  for k = 1, 3 do
    oakshoot(pts[n][1], pts[n][2], la + (k-2) * rand(0.3, 0.55), rand(lenmin*0.8, lenmax), clamp(wd[n]*0.8, 1.0, wcap), depth, opts)
  end
end

sprout2("S1", 0.3, 1.0, 20, 3.2, 2, 34, 70, {p=0.6})
print(wait(0))

--@ chunk 51
for _, nm in ipairs({"S2","S3","S4","S6","S7","S10","S11","S12"}) do
  sprout2(nm, 0.25, 1.0, 18, 3.2, 2, 34, 70, {p=0.6})
end
print(wait(0))

--@ chunk 52
print(drying(700,350), drying(700,100), drying(300,50), drying(560,520), drying(800,600), drying(250,300), drying(600,300))
print(wait(0))

--@ chunk 53
glowHot = pile{{"lead white",5},{"chrome yellow",1.0},{"vermilion",0.4},{"yellow ochre",0.25}, medium=0.3}
core = ellipse(640, 396, 330, 38):blur(26)
work(core * skyM, {hand="glaze", pile=glowHot, angle=0.0, coverage=2.2, clip=true})
blend(core:grow(30) * skyM, {angle=0.0})
print(wait(0))

--@ chunk 54
print(wait(20*60))
print(drying(450,360), drying(300,200), drying(250,300), drying(560,520), drying(800,600), drying(600,330))

--@ chunk 55
print(wait(16*60))
print(drying(450,360), drying(300,200), drying(250,300), drying(560,520), drying(800,600), drying(600,330))

--@ chunk 56
Z = rect(285, 296, 715, 118) * skyM
Gbase = pile{{"lead white",6},{"chrome yellow",0.7},{"yellow ochre",0.3},{"Prussian blue",0.1}, medium=0.1}
Ggold = pile{{"lead white",4},{"chrome yellow",1.2},{"vermilion",0.3},{"yellow ochre",0.5}, medium=0.15}
Gpeach = pile{{"lead white",3},{"vermilion",0.7},{"chrome yellow",0.8},{"red earth",0.1}, medium=0.15}
work(Z, {hand="body", tool="filbert 14", pile=Gbase, angle=0.0, coverage=3.6, clip=true, length={40,110}, fill=true,
  angle_jitter=0.04,
  load_at=function(x,y) return smoothstep(296, 338, y) * smoothstep(285, 345, x) end})
work(Z, {hand="body", tool="filbert 14", pile=Ggold, angle=0.0, coverage=3.0, clip=true, length={40,110},
  angle_jitter=0.04,
  load_at=function(x,y) return math.exp(-((x-650)/300)^2) * smoothstep(325, 405, y)^1.1 end})
work(Z, {hand="body", tool="filbert 14", pile=Gpeach, angle=0.0, coverage=3.0, clip=true, length={40,110},
  angle_jitter=0.04,
  load_at=function(x,y) return 0.95 * math.exp(-((x-650)/190)^2) * smoothstep(368, 405, y)^1.5 end})
blend(Z, {angle=0.0})
blend(Z, {angle=0.02})
print(wait(0))

--@ chunk 57
print(wait(48*60))
print("trunk", drying(244,490), "L3", drying(300,418), "twig", drying(480,330), "Z", drying(500,330), drying(700,380), "rock", drying(800,400), "fg", drying(300,600))

--@ chunk 58
print(wait(48*60))
print("trunk", drying(244,490), "L3", drying(300,418), "twig", drying(480,330), "Z", drying(500,330), drying(700,380), "rock", drying(800,400), "fg", drying(300,600))

--@ chunk 59
Z2 = rect(268, 296, 732, 118) * skyM
Fb = rect(268, 232, 732, 74) * skyM
Gcool = pile{{"lead white",6},{"pale smalt",0.6},{"chrome yellow",0.45},{"Prussian blue",0.06}, medium=0.0}
Gbase = pile{{"lead white",6},{"chrome yellow",0.7},{"yellow ochre",0.3},{"Prussian blue",0.05}, medium=0.05}
Ggold = pile{{"lead white",4},{"chrome yellow",1.2},{"vermilion",0.3},{"yellow ochre",0.5}, medium=0.1}
Gpeach = pile{{"lead white",3},{"vermilion",0.7},{"chrome yellow",0.8},{"red earth",0.1}, medium=0.1}
-- opaque base to bury the grey smear
work(Z2, {hand="body", tool="filbert 14", pile=Gcool, angle=0.0, coverage=4.2, clip=true, length={40,110}, fill=true, angle_jitter=0.04})
-- feather above it
work(Fb, {hand="body", tool="filbert 14", pile=Gcool, angle=0.0, coverage=3.0, clip=skyM, length={40,110}, angle_jitter=0.04,
  load_at=function(x,y) return smoothstep(236, 302, y) end})
-- lemon, gold, peach built up wet-in-wet
work(Z2, {hand="body", tool="filbert 14", pile=Gbase, angle=0.0, coverage=3.0, clip=true, length={40,110}, angle_jitter=0.04,
  load_at=function(x,y) return smoothstep(310, 360, y) end})
work(Z2, {hand="body", tool="filbert 14", pile=Ggold, angle=0.0, coverage=3.0, clip=true, length={40,110}, angle_jitter=0.04,
  load_at=function(x,y) return (0.25 + 0.75*math.exp(-((x-650)/330)^2)) * smoothstep(335, 405, y)^1.1 end})
work(Z2, {hand="body", tool="filbert 14", pile=Gpeach, angle=0.0, coverage=3.0, clip=true, length={40,110}, angle_jitter=0.04,
  load_at=function(x,y) return 0.95 * math.exp(-((x-650)/200)^2) * smoothstep(368, 405, y)^1.5 end})
blend(Z2 + Fb, {angle=0.0})
blend(Z2 + Fb, {angle=0.02})
print(wait(0))

--@ chunk 60
-- richer horizon glow, wet into the open coat
work(Z2, {hand="body", tool="filbert 14", pile=Gpeach, angle=0.0, coverage=3.0, clip=true, length={40,110}, angle_jitter=0.04,
  load_at=function(x,y) return math.exp(-((x-650)/270)^2) * smoothstep(345, 408, y)^1.3 end})
work(Z2, {hand="body", tool="filbert 14", pile=Ggold, angle=0.0, coverage=2.5, clip=true, length={40,110}, angle_jitter=0.04,
  load_at=function(x,y) return 0.8 * math.exp(-((x-700)/380)^2) * (smoothstep(318, 350, y) * (1 - smoothstep(372, 400, y))) end})
-- soft grainy lift above the glow edge, stippled
Gfeath = pile{{"lead white",6},{"pale smalt",0.9},{"chrome yellow",0.3},{"yellow ochre",0.12}, medium=0.2}
stipple(rect(268,130,732,130) * skyM, {pile=Gfeath, width=3, coverage=function(x,y) return 2.4*(1-smoothstep(135,246,y)) end,
  feather=0.7, clip=skyM, pressure={0.35,0.8}})
blend(rect(268,150,732,170) * skyM, {angle=0.0})
print(wait(0))

--@ chunk 61
print(wait(21*24*60))
print("trunk", drying(244,490), "L3", drying(300,418), "twig", drying(480,330), "Z", drying(500,330), drying(700,380), "rock", drying(800,400), "fg", drying(300,600), "stip", drying(600,160), "mist", drying(800,600))

--@ chunk 62
L2pts = OAK.L2[1]
function xb(y)
  for i = 1, #L2pts-1 do
    local a, b = L2pts[i], L2pts[i+1]
    if (y <= a[2] and y >= b[2]) then
      local t = (a[2]-y)/(a[2]-b[2])
      return a[1] + t*(b[1]-a[1])
    end
  end
  return L2pts[#L2pts][1]
end
skyFix = mask(function(x,y)
  if x < xb(y) - 3 then return 0 end
  return smoothstep(112, 150, y)
end) * rect(0,100,1000,330) - R1m - rockM

N1 = pile{{"lead white",5},{"smalt",3},{"cobalt blue",0.6}, medium=0.1}
N2 = pile{{"lead white",6},{"pale smalt",2.5},{"cobalt blue",0.2},{"Prussian blue",0.05},{"yellow ochre",0.15}, medium=0.1}
N3 = pile{{"lead white",6},{"pale smalt",1},{"chrome yellow",0.5},{"Prussian blue",0.12}, medium=0.1}
N4 = pile{{"lead white",6},{"chrome yellow",0.7},{"yellow ochre",0.3},{"Prussian blue",0.05}, medium=0.05}

local function band(y0, y1) return skyFix * rect(0, y0, 1000, y1 - y0) end
work(band(110,215), {hand="broad", pile=N1, angle=0.0, coverage=3.4, clip=true, fill=true, angle_jitter=0.04})
work(band(190,285), {hand="broad", pile=N2, angle=0.0, coverage=3.4, clip=true, fill=true, angle_jitter=0.04})
work(band(255,340), {hand="broad", pile=N3, angle=0.0, coverage=3.4, clip=true, fill=true, angle_jitter=0.04})
work(band(310,372), {hand="broad", pile=N4, angle=0.0, coverage=3.4, clip=true, fill=true, angle_jitter=0.04})
print(wait(0))

--@ chunk 63
Gcore = pile{{"lead white",6},{"chrome yellow",0.55},{"yellow ochre",0.1}, medium=0.05}
local function core(x) return math.exp(-((x-620)/170)^2) end
local gb = skyFix * rect(0, 340, 1000, 90)
work(gb, {hand="broad", pile=Ggold, angle=0.0, coverage=3.4, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return 0.85 * smoothstep(345,400,y) end})
work(gb, {hand="broad", pile=Gpeach, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return 0.9 * (1 - 0.65*core(x)) * smoothstep(372,408,y)^1.2 end})
work(gb, {hand="broad", pile=Gcore, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return core(x) * smoothstep(362,408,y)^1.1 end})
blend(skyFix, {angle=0.0})
blend(skyFix, {angle=0.0})
print(wait(0))

--@ chunk 64
seamM = mask(function(x,y)
  if x < xb(y) - 2 then return 0 end
  return 1
end) * rect(0,70,1000,200) - R1m - rockM
work(seamM, {hand="glaze", pile=T1, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(1.0 - (y-100)/130, 0.0, 1.0)^1.3 end})
blend(rect(0,60,1000,240) * seamM, {angle=0.0})
print(wait(0))

--@ chunk 65
blend(rect(262,38,738,80), {angle=1.5708})
blend(rect(262,38,738,80), {angle=0.0})
print(wait(0))

--@ chunk 66
-- 1. bury the bulb below the soil line and the test twigs
bulb = poly({{180,547},{205,549},{245,553},{285,551},{312,548},{320,600},{170,600}}, true) * fgM
work(bulb, {hand="body", tool="filbert 12", pile=F2, angle=crestAng, coverage=5, clip=true, length={20,60}, fill=true, load=1.0})
twigs = rect(0,590,480,124) * fgM
work(twigs, {hand="body", tool="filbert 12", pile=F2, angle=crestAng, coverage=6, clip=true, length={20,70}, fill=true, load=1.0})
-- 2. root flares
function rootmask(spine, widths, seed)
  return body_of{spine=spine, widths=widths, blend=0.8, char="firm", amount=1.0}:mask()
end
rL = rootmask({{216,500},{208,524},{194,541},{176,551},{156,556}}, {20,20,15,8,3})
rR = rootmask({{276,500},{285,524},{300,541},{320,550},{342,555}}, {20,20,15,8,3})
rC = rootmask({{246,515},{249,540},{254,556}}, {26,16,6})
rootsM = (rL + rR + rC)
work(rootsM, {hand="body", tool="filbert 9", pile=OakD, coverage=4, clip=true, angle=1.4, length={15,40}, fill=true, load=1.0})
print(wait(0))

--@ chunk 67
print(wait(5*24*60))
print("sky", drying(600,200), drying(600,150), drying(500,300), drying(600,390), "soil", drying(250,560), "roots", drying(180,550))

--@ chunk 68
skyR = mask(function(x,y)
  if x < xb(y) - 3 then return 0 end
  return 1
end) * rect(0, 70, 1000, 328) - R1m - rockM
Wn = pile{{"lead white",7},{"pale smalt",0.5},{"chrome yellow",0.12},{"yellow ochre",0.08}, medium=0.0}
work(skyR * rect(0,96,1000,320), {hand="broad", pile=Wn, angle=0.0, coverage=4.0, clip=true, fill=true, angle_jitter=0.05,
  load_at=function(x,y) return smoothstep(96, 150, y) end})
print(wait(0))

--@ chunk 69
function coreF(x) return math.exp(-((x-620)/170)^2) end
C1 = pile{{"lead white",4.5},{"smalt",3},{"cobalt blue",0.8}, medium=0.15}
C2 = pile{{"lead white",5},{"pale smalt",2.5},{"cobalt blue",0.3},{"yellow ochre",0.1}, medium=0.15}
C3 = pile{{"lead white",5},{"pale smalt",1.2},{"chrome yellow",0.55},{"Prussian blue",0.12}, medium=0.12}
C4 = pile{{"lead white",5},{"chrome yellow",0.9},{"yellow ochre",0.3},{"Prussian blue",0.05}, medium=0.1}
region = skyR * rect(0, 90, 1000, 320)
function bump(y, c, w) return math.exp(-((y-c)/w)^2) end
work(region, {hand="broad", pile=C1, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return clamp(bump(y,100,70),0,1) end})
work(region, {hand="broad", pile=C2, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return clamp(bump(y,190,60),0,1) end})
work(region, {hand="broad", pile=C3, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return clamp(bump(y,275,50),0,1) end})
work(region, {hand="broad", pile=C4, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return clamp(bump(y,345,50),0,1) end})
work(region, {hand="broad", pile=Ggold, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return clamp(0.9*smoothstep(335,395,y),0,1) end})
work(region, {hand="broad", pile=Gpeach, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return clamp(0.95 * (1 - 0.6*coreF(x)) * smoothstep(368,404,y)^1.2,0,1) end})
work(region, {hand="broad", pile=Gcore, angle=0.0, coverage=3.0, clip=true, fill=true, angle_jitter=0.04,
  load_at=function(x,y) return coreF(x) * smoothstep(350,404,y)^1.1 end})
blend(region, {angle=0.0})
blend(region, {angle=0.0})
print(wait(0))

--@ chunk 70
print(wait(5*24*60))
print("sky", drying(600,200), drying(600,150), drying(500,300), drying(600,390), "soil", drying(250,560), "roots", drying(180,550))

--@ chunk 71
-- Opaque cover over ghost twigs: dense, medium-free, graded to match, wet-in-wet bands, then blended
ghostR = rect(296, 92, 318, 236) * skyR
D1 = pile{{"lead white",4.5},{"smalt",2.6},{"cobalt blue",0.7}}
D2 = pile{{"lead white",5.5},{"pale smalt",2.4},{"cobalt blue",0.25},{"yellow ochre",0.1}}
D3 = pile{{"lead white",5},{"pale smalt",1.0},{"chrome yellow",0.5},{"Prussian blue",0.1}}
D4 = pile{{"lead white",5},{"chrome yellow",0.8},{"yellow ochre",0.25},{"Prussian blue",0.04}}
work(ghostR, {hand="body", tool="filbert 14", pile=D2, angle=0.0, coverage=3.2, clip=true, fill=true, length={40,100}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(92,130,y) * (1 - smoothstep(190, 240, y)),0,1) end})
work(ghostR, {hand="body", tool="filbert 14", pile=D1, angle=0.0, coverage=2.6, clip=true, fill=true, length={40,100}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(bump(y,106,48),0,1) end})
work(ghostR, {hand="body", tool="filbert 14", pile=D3, angle=0.0, coverage=3.2, clip=true, fill=true, length={40,100}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(bump(y,262,52),0,1) end})
work(ghostR, {hand="body", tool="filbert 14", pile=D4, angle=0.0, coverage=2.6, clip=true, fill=true, length={40,100}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(210,250,y),0,1) end})
blend(ghostR:grow(24) * skyR, {angle=0.0})
print(wait(0))

--@ chunk 72
print(wait(6*24*60))
print("patch", drying(450,200), drying(450,300), "sky", drying(800,200))

--@ chunk 73
-- SKY PASS 3: one consistent opaque graded coat right of the oak's central limb
skyNew = mask(function(x,y)
  if x < xb(y) - 2 then return 0 end
  return 1
end) * rect(0, 56, 1000, 360) - R1m - rockM
P1 = pile{{"lead white",4},{"smalt",3},{"cobalt blue",0.9}}                       -- upper blue
P2 = pile{{"lead white",5},{"smalt",1.8},{"cobalt blue",0.5},{"yellow ochre",0.08}}  -- light blue
P3 = pile{{"lead white",6},{"pale smalt",1.6},{"chrome yellow",0.35},{"Prussian blue",0.08}} -- pale green-white
P4 = pile{{"lead white",6},{"chrome yellow",0.7},{"yellow ochre",0.25},{"Prussian blue",0.04}} -- lemon
P5 = pile{{"lead white",4},{"chrome yellow",1.1},{"yellow ochre",0.5},{"vermilion",0.2}} -- gold
P6 = pile{{"lead white",3},{"vermilion",0.75},{"chrome yellow",0.8},{"red earth",0.1}}  -- peach
P7 = pile{{"lead white",6},{"chrome yellow",0.6},{"vermilion",0.08}}              -- hot pale core
function sprd(m, p, f, cov)
  work(m, {hand="body", tool="filbert 16", pile=p, angle=0.0, coverage=cov or 3.4, clip=true, fill=true, length={50,130}, angle_jitter=0.045, load_at=f})
end
sprd(skyNew, P2, function(x,y) return clamp(smoothstep(66,110,y) * (1 - smoothstep(230,300,y)),0,1) end, 3.6)
sprd(skyNew, P1, function(x,y) return clamp(1 - smoothstep(66,190,y),0,1)^1.1 end, 3.0)
sprd(skyNew, P3, function(x,y) return clamp(bump(y,268,56),0,1) end, 3.4)
sprd(skyNew, P4, function(x,y) return clamp(smoothstep(250,330,y),0,1) end, 3.4)
sprd(skyNew, P5, function(x,y) return clamp((0.35 + 0.65*bump(x,650,380)) * smoothstep(330,400,y),0,1) end, 3.2)
sprd(skyNew, P6, function(x,y) return clamp(0.95*(1 - 0.55*coreF(x)) * bump(x,650,260) * smoothstep(368,410,y)^1.2,0,1) end, 3.0)
sprd(skyNew, P7, function(x,y) return clamp(coreF(x) * smoothstep(350,410,y)^1.1,0,1) end, 3.0)
blend(skyNew, {angle=0.0})
blend(skyNew, {angle=0.0})
print(wait(0))

--@ chunk 74
skyNew2 = mask(function(x,y)
  if x < xb(y) - 2 then return 0 end
  return 1
end) * rect(0, 0, 1000, 330) - R1m - rockM
Q1 = pile{{"lead white",3.4},{"smalt",3.2},{"cobalt blue",1.0}}
Q2 = pile{{"lead white",4.5},{"smalt",2.2},{"cobalt blue",0.6},{"yellow ochre",0.06}}
Q3 = pile{{"lead white",5.5},{"pale smalt",2.2},{"cobalt blue",0.3},{"yellow ochre",0.1}}
sprd(skyNew2, Q1, function(x,y) return clamp(1 - smoothstep(60,215,y),0,1)^0.9 end, 3.4)
sprd(skyNew2, Q2, function(x,y) return clamp(bump(y,170,48),0,1) end, 3.2)
sprd(skyNew2, Q3, function(x,y) return clamp(bump(y,235,40),0,1) end, 3.2)
blend(skyNew2, {angle=0.0})
blend(skyNew2, {angle=0.0})
print(wait(0))

--@ chunk 75
print(wait(3*24*60))
print("sky", drying(800,100), drying(600,300), drying(400,200), drying(700,380))

--@ chunk 76
GB = pile{{"cobalt blue",2},{"smalt",2.2},{"Prussian blue",0.06},{"lead white",0.6}, medium=0.7}
gm = skyNew2 * rect(0,0,1000,300)
work(gm, {hand="glaze", pile=GB, angle=0.0, coverage=2.6, clip=true,
  load_at=function(x,y) return clamp(1 - smoothstep(20,215,y),0,1)^1.0 end})
blend(gm, {angle=0.0})
print(wait(0))

--@ chunk 77
blend(gm, {angle=0.0})
blend(gm, {angle=1.5708})
blend(gm, {angle=0.0})
print(wait(0))

--@ chunk 78
print(drying(800,60), drying(600,150))
SV = pile{{"cobalt blue",1.2},{"smalt",2.4},{"lead white",2.2},{"Prussian blue",0.03}, medium=0.5}
tst = rect(700, 0, 300, 110) * skyNew2
stipple(tst, {pile=SV, width=4, coverage=1.8, clip=true, pressure={0.3,0.7}, feather=0.3, cluster=0.15})
print(wait(0))

--@ chunk 79
print(wait(3*24*60))
print(drying(800,60), drying(600,150), drying(700,50))

--@ chunk 80
-- pale veil: milky blue over the saturated glaze, strongest in the middle sky, light at the very top
VE = pile{{"lead white",5},{"pale smalt",2.0},{"cobalt blue",0.25}, medium=0.55}
veilM = skyNew2 * rect(0,0,1000,330)
work(veilM, {hand="glaze", pile=VE, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(0.18 + 0.82*smoothstep(10,150,y),0,1) * (1 - smoothstep(210,320,y)) * (0.55 + 0.45*(1-smoothstep(300,700,x))) end})
print(wait(0))

--@ chunk 81
blend(veilM, {angle=0.0})
blend(veilM, {angle=0.04})
print(wait(0))

--@ chunk 82
skyAll = rect(0,0,1000,336) - R1m - rockM
GB1 = pile{{"smalt",2.6},{"cobalt blue",1.4},{"lead white",0.6}, medium=0.75}
GW1 = pile{{"chrome yellow",1.0},{"vermilion",0.55},{"yellow ochre",0.3},{"lead white",1.0}, medium=0.75}
work(skyAll, {hand="glaze", pile=GB1, angle=0.0, coverage=2.0, clip=true,
  load_at=function(x,y) return clamp(1 - smoothstep(0,230,y),0,1)^1.15 end})
work(skyAll * rect(0,250,1000,90), {hand="glaze", pile=GW1, angle=0.0, coverage=2.0, clip=true,
  load_at=function(x,y) return clamp(smoothstep(290,400,y),0,1)^1.3 * (0.35+0.65*bump(x,640,420)) end})
blend(skyAll, {angle=0.0})
blend(skyAll, {angle=0.0})
print(wait(0))

--@ chunk 83
gz2 = rect(0,326,1000,90) - R1m - rockM
work(gz2, {hand="glaze", pile=GW1, angle=0.0, coverage=2.0, clip=true,
  load_at=function(x,y) return clamp(smoothstep(290,400,y),0,1)^1.3 * (0.35+0.65*bump(x,640,420)) end})
blend(rect(0,300,1000,112) - R1m - rockM, {angle=0.0})
blend(rect(0,300,1000,112) - R1m - rockM, {angle=0.0})
print(wait(0))

--@ chunk 84
-- Cloud bodies: opaque cool lilac-grey, lit-warm undersides. Sky only (never the oak: x right of its central limb)
CB = pile{{"lead white",5},{"smalt",1.3},{"red earth",0.3},{"yellow ochre",0.12}}
CB2 = pile{{"lead white",6},{"smalt",0.8},{"red earth",0.2},{"yellow ochre",0.2}}
CW = pile{{"lead white",5},{"vermilion",0.5},{"yellow ochre",0.7},{"red earth",0.1}}
local skyOnly = mask(function(x,y)
  if x < xb(y) + 4 then return 0 end
  return 1
end) * rect(0,0,1000,340) - R1m - rockM
cloudA1 = lens(300, 780, 205, 26, 6, 0.65) * skyOnly
cloudA2 = lens(560, 1010, 168, 20, 4, 0.65) * skyOnly
cloudA3 = lens(285, 650, 252, 15, 4, 0.65) * skyOnly
for _, c in ipairs({cloudA1, cloudA2, cloudA3}) do
  work(c, {hand="body", tool="filbert 14", pile=CB, angle=0.0, coverage=4.0, clip=true, fill=true, length={60,160}, angle_jitter=0.03,
    load_at=function(x,y) return 1.0 end})
end
print(wait(0))

--@ chunk 85
skyOnly = mask(function(x,y)
  if x < xb(y) + 4 then return 0 end
  return 1
end) * rect(0,0,1000,340) - R1m - rockM
cloudsAll = (cloudA1 + cloudA2 + cloudA3):grow(10) * skyOnly
blend(cloudsAll, {angle=0.0})
blend(cloudsAll, {angle=0.12})
blend(cloudsAll, {angle=-0.1})
print(wait(0))

--@ chunk 86
print(drying(700,168), drying(500,205), drying(400,252))
print(lose)
local c = (cloudA1 + cloudA2 + cloudA3)
local n = lose(c, {pile=pile{{"lead white",5},{"smalt",1.0},{"pale smalt",1.5},{"yellow ochre",0.1}}, where=1, reach={14,10}, load=0.3})
print("strokes", n)

--@ chunk 87
cl = (cloudA1 + cloudA2 + cloudA3)
reg = cl:grow(26)
blend(reg, {angle=0.0})
blend(reg, {angle=0.06})
print(wait(0))

--@ chunk 88
trunkIn = poly({{217,438},{264,438},{266,470},{271,486},{279,505},{286,520},{296,533},{314,543},{336,548},{300,553},{262,556},{226,554},{196,548},{206,530},{211,512},{213,490},{214,465}}, true)
BarkG = pile{{"raw umber",2},{"red earth",1},{"lead white",0.6},{"yellow ochre",0.3}, medium=0.65}
BarkH = pile{{"raw umber",2},{"yellow ochre",1},{"lead white",1.4},{"red earth",0.4}, medium=0.2}
-- (a) warm translucent glaze, stronger on the side facing the glow
work(trunkIn, {hand="glaze", pile=BarkG, angle=1.5708, coverage=2.0, clip=true,
  load_at=function(x,y) return 0.15 + 0.85*smoothstep(224, 270, x) end})
-- (b) vertical bark ridges, drier and lighter toward the right
work(trunkIn, {hand="detail", pile=BarkH, angle=function(x,y) return 1.5708 + 0.25*math.sin(y/17 + x/9) end,
  coverage=1.1, clip=true, length={10,34}, pressure={0.15,0.4}, load=0.5,
  load_at=function(x,y) return 0.1 + 0.9*smoothstep(232, 268, x)^1.4 end})
print(wait(0))

--@ chunk 89
OakW = pile{{"raw umber",3},{"Prussian blue",0.55},{"bone black",1.2},{"red earth",0.5}, medium=0.02}
work(trunkIn:grow(1), {hand="body", tool="filbert 9", pile=OakW, angle=function(x,y) return 1.5708 + 0.3*math.sin(y/23 + x/11) end,
  coverage=3.4, clip=true, length={20,55}, fill=true, load=1.0})
print(wait(0))

--@ chunk 90
-- Right half of the oak, new limbs painted over the (dry) sky. Each: {pts, widths}
OAKR = {}
OAKR.R1   = {{{258,432},{290,420},{318,406},{340,388},{354,364},{372,340}}, {38,35,32,29,26,23}}
OAKR.R1a  = {{{372,340},{362,312},{350,288},{352,262},{366,238},{364,212},{352,190},{356,164},{370,142}}, {19,17,15,13,11,9,7,5,3}}
OAKR.R1b  = {{{372,340},{396,326},{424,318},{450,320},{474,310},{496,292},{516,282}}, {19,17,15,13,10,7,3.5}}
OAKR.R1a2 = {{{364,212},{388,200},{410,176},{420,150},{436,130}}, {8,7,5.5,3.5,2}}
OAKR.R1b1 = {{{450,320},{446,292},{434,268},{436,242},{448,222},{452,200}}, {9,8,6.5,4.5,3,2}}
OAKR.R1b2 = {{{474,310},{500,316},{526,312},{552,300},{574,296}}, {8,7,5.5,3.5,2}}
OAKR.R1b3 = {{{496,292},{510,268},{530,252},{546,232}}, {7,6,4.5,2}}
OAKR.L2r1 = {{{270,236},{296,226},{318,206},{332,180},{336,156},{330,134}}, {12,10,8,6,4,2}}
OAKR.L2r2 = {{{276,144},{302,134},{326,112},{346,100},{366,92}}, {8,7,5.5,3.5,2}}
-- the central limb again, from the horizon up, slightly stouter than before so it covers the veil
OAKR.L2re = {{{250,404},{268,380},{272,352},{258,328},{244,306},{248,282},{264,262},{270,236},{260,214},{262,190},{272,168},{276,144},{270,120},{272,98},{278,78}},
             {42,41,38,34,30,27,24,22,19,17,15,13,11,9,6}}

function paintL(L, char, tool, cov, extra)
  local b = body_of{spine=L[1], widths=L[2], blend=0.8, char=char or "firm", amount=1.0}
  local m = b:mask()
  work(m, {hand="body", tool=tool or "filbert 9", pile=OakD, coverage=cov or 3.4, clip=true,
           angle=limbangle(L), length={25,60}, fill=true, load=1.0})
end
paintL(OAKR.L2re, "broken", "filbert 9", 3.4)
paintL(OAKR.R1, "broken", "filbert 12", 3.6)
paintL(OAKR.R1a, "broken", "filbert 9", 3.4)
paintL(OAKR.R1b, "broken", "filbert 9", 3.4)
print(wait(0))

--@ chunk 91
paintL(OAKR.R1a2, "firm", "filbert 7", 3.5)
paintL(OAKR.R1b1, "firm", "filbert 7", 3.5)
paintL(OAKR.R1b2, "firm", "filbert 7", 3.5)
paintL(OAKR.R1b3, "firm", "filbert 7", 3.5)
paintL(OAKR.L2r1, "firm", "filbert 7", 3.5)
paintL(OAKR.L2r2, "firm", "filbert 7", 3.5)
print(wait(0))

--@ chunk 92
for k, v in pairs(OAKR) do OAK["N_"..k] = v end
sprout2("N_R1", 0.55, 0.95, 26, 3.6, 2, 38, 74, {p=0.55})
sprout2("N_R1a", 0.15, 1.0, 17, 3.4, 2, 34, 70, {p=0.6})
sprout2("N_R1b", 0.15, 1.0, 17, 3.4, 2, 34, 70, {p=0.6})
sprout2("N_R1a2", 0.2, 1.0, 15, 3.0, 2, 30, 60, {p=0.6})
sprout2("N_R1b1", 0.2, 1.0, 15, 3.0, 2, 30, 60, {p=0.6})
sprout2("N_R1b2", 0.2, 1.0, 15, 3.0, 2, 30, 60, {p=0.6})
sprout2("N_R1b3", 0.2, 1.0, 15, 3.0, 2, 30, 60, {p=0.6})
sprout2("N_L2r1", 0.2, 1.0, 15, 3.0, 2, 30, 60, {p=0.6})
sprout2("N_L2r2", 0.2, 1.0, 15, 3.0, 2, 30, 60, {p=0.6})
sprout2("N_L2re", 0.55, 1.0, 17, 3.0, 2, 30, 62, {p=0.6})
print(wait(0))

--@ chunk 93
-- Masks of the solid limbs that must be spared by the sky repaint
function limbBody(L, seed)
  return body_of{spine=L[1], widths=L[2], blend=0.8, char="firm", amount=0.0}:mask()
end
local keepNames = {"L1","L2","S2","S4","S6","S7","S8","S9"}
limbsFat = nil
for _, nm in ipairs(keepNames) do
  local m = limbBody(OAK[nm])
  limbsFat = limbsFat and (limbsFat + m) or m
end
for nm, L in pairs(OAKR) do
  limbsFat = limbsFat + limbBody(L)
end
limbsFat = limbsFat:grow(2.5)
skyReg = rect(0, 100, 1000, 312) - R1m - rockM - limbsFat
print("area", skyReg:area())

-- The new gradient, wet-in-wet. Glow is localised at x~630.
function coreG(x) return math.exp(-((x-630)/175)^2) end
sprd(skyReg, P2, function(x,y) return clamp(smoothstep(104,150,y) * (1 - smoothstep(220,300,y)),0,1) end, 3.6)
sprd(skyReg, P1, function(x,y) return clamp(1 - smoothstep(100,200,y),0,1)^1.1 end, 3.0)
sprd(skyReg, P3, function(x,y) return clamp(bump(y,268,56),0,1) end, 3.4)
sprd(skyReg, P4, function(x,y) return clamp(smoothstep(250,335,y),0,1) end, 3.4)
sprd(skyReg, P5, function(x,y) return clamp((0.12 + 0.88*bump(x,640,330)) * smoothstep(335,402,y),0,1) end, 3.2)
sprd(skyReg, P6, function(x,y) return clamp(0.95*(1 - 0.5*coreG(x)) * bump(x,640,250) * smoothstep(370,410,y)^1.2,0,1) end, 3.0)
sprd(skyReg, P7, function(x,y) return clamp(coreG(x) * smoothstep(350,410,y)^1.1,0,1) end, 3.0)
blend(skyReg, {angle=0.0})
blend(skyReg, {angle=0.0})
print(wait(0))

--@ chunk 94
W0c = pile{{"lead white",8},{"pale smalt",0.5},{"yellow ochre",0.1}}
W0w = pile{{"lead white",8},{"chrome yellow",0.35},{"yellow ochre",0.15}}
work(skyReg, {hand="body", tool="filbert 18", pile=W0c, angle=0.0, coverage=4.4, clip=true, fill=true, length={60,150}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(1 - smoothstep(250,340,y),0,1) end})
work(skyReg, {hand="body", tool="filbert 18", pile=W0w, angle=0.0, coverage=4.4, clip=true, fill=true, length={60,150}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(240,330,y),0,1) end})
blend(skyReg, {angle=0.0})
print(wait(0))

--@ chunk 95
print(wait(3*24*60))
print(drying(600,200), drying(600,330), drying(400,150))

--@ chunk 96
B1 = pile{{"cobalt blue",2.2},{"smalt",2.0},{"lead white",2.0}}
B2 = pile{{"cobalt blue",1.2},{"smalt",1.2},{"lead white",3.6}}
B3 = pile{{"pale smalt",1.4},{"cobalt blue",0.35},{"lead white",5},{"Prussian blue",0.07},{"chrome yellow",0.12}}
B4 = pile{{"lead white",5},{"chrome yellow",0.65},{"Prussian blue",0.07},{"pale smalt",0.4}}
B5 = pile{{"lead white",3.6},{"chrome yellow",1.2},{"yellow ochre",0.4},{"vermilion",0.08}}
B6 = pile{{"lead white",2.6},{"vermilion",0.9},{"chrome yellow",0.9}}
B7 = pile{{"lead white",5},{"chrome yellow",0.8},{"vermilion",0.1}}
sprd(skyReg, B2, function(x,y) return clamp(smoothstep(104,160,y) * (1 - smoothstep(190,270,y)),0,1) end, 3.4)
sprd(skyReg, B1, function(x,y) return clamp(1 - smoothstep(100,215,y),0,1)^1.0 end, 3.2)
sprd(skyReg, B3, function(x,y) return clamp(bump(y,248,45),0,1) end, 3.4)
sprd(skyReg, B4, function(x,y) return clamp(smoothstep(250,335,y),0,1) end, 3.4)
sprd(skyReg, B5, function(x,y) return clamp((0.1 + 0.9*bump(x,630,340)) * smoothstep(338,402,y),0,1) end, 3.2)
sprd(skyReg, B6, function(x,y) return clamp(0.95*(1 - 0.45*coreG(x)) * bump(x,630,260) * smoothstep(372,410,y)^1.2,0,1) end, 3.0)
sprd(skyReg, B7, function(x,y) return clamp(coreG(x) * smoothstep(352,410,y)^1.1,0,1) end, 3.0)
blend(skyReg, {angle=0.0})
blend(skyReg, {angle=0.0})
print(wait(0))

--@ chunk 97
limbsTight = nil
for _, nm in ipairs({"L1","L2","S2","S4","S6","S7","S8","S9"}) do
  local m = limbBody(OAK[nm]); limbsTight = limbsTight and (limbsTight + m) or m
end
for nm, L in pairs(OAKR) do limbsTight = limbsTight + limbBody(L) end
limbsTight = limbsTight:grow(0.6)
skyAll2 = rect(0, -20, 1000, 434) - R1m - rockM - limbsTight

Y1 = pile{{"cobalt blue",2.2},{"smalt",2.6},{"lead white",1.6}}
Y2 = pile{{"cobalt blue",1.4},{"smalt",1.3},{"lead white",3.4}}
Y3 = pile{{"pale smalt",1.4},{"cobalt blue",0.45},{"lead white",5},{"Prussian blue",0.05}}
Y3b= pile{{"pale smalt",0.9},{"lead white",5.5},{"chrome yellow",0.3},{"Prussian blue",0.07}}
Y4 = pile{{"lead white",5.5},{"chrome yellow",0.75},{"Prussian blue",0.05},{"yellow ochre",0.1}}
Y5 = pile{{"lead white",3.6},{"chrome yellow",1.3},{"yellow ochre",0.4},{"vermilion",0.1}}
Y6 = pile{{"lead white",2.8},{"vermilion",0.9},{"chrome yellow",1.0}}
Y7 = pile{{"lead white",5},{"chrome yellow",0.8},{"vermilion",0.1}}
sprd(skyAll2, Y1, function(x,y) return clamp(1 - smoothstep(30,160,y),0,1) end, 3.4)
sprd(skyAll2, Y2, function(x,y) return clamp(bump(y,110,85),0,1) end, 3.2)
sprd(skyAll2, Y3, function(x,y) return clamp(bump(y,195,62),0,1) end, 3.2)
sprd(skyAll2, Y3b,function(x,y) return clamp(bump(y,265,48),0,1) end, 3.2)
sprd(skyAll2, Y4, function(x,y) return clamp(smoothstep(255,335,y),0,1) end, 3.2)
sprd(skyAll2, Y5, function(x,y) return clamp((0.12 + 0.88*bump(x,640,380)) * smoothstep(335,405,y),0,1) end, 3.2)
sprd(skyAll2, Y6, function(x,y) return clamp(0.95*(1 - 0.4*coreG(x)) * bump(x,640,290) * smoothstep(372,412,y)^1.2,0,1) end, 3.0)
sprd(skyAll2, Y7, function(x,y) return clamp(coreG(x) * smoothstep(350,412,y)^1.1,0,1) end, 3.0)
blend(skyAll2, {angle=0.0})
blend(skyAll2, {angle=0.0})
print(wait(0))

--@ chunk 98
print(wait(3*24*60))
print(drying(600,200), drying(600,330), drying(400,150), drying(630,400))

--@ chunk 99
print(wait(2*24*60))
print(drying(600,200), drying(600,330), drying(400,150), drying(630,400), drying(630,350))

--@ chunk 100
skyG = rect(0, -20, 1000, 434) - R1m - rockM
GoldG   = pile{{"chrome yellow",1.0},{"yellow ochre",0.45}, medium=0.8}
OrangeG = pile{{"vermilion",0.8},{"chrome yellow",1.0},{"yellow ochre",0.2}, medium=0.8}
BlueG   = pile{{"cobalt blue",2.0},{"smalt",2.4},{"Prussian blue",0.05}, medium=0.8}
work(skyG, {hand="glaze", pile=GoldG, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(smoothstep(270,400,y),0,1)^1.1 * (0.5 + 0.5*bump(x,640,420)) end})
work(skyG, {hand="glaze", pile=OrangeG, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(smoothstep(335,408,y),0,1)^1.3 * bump(x,630,300) end})
work(skyG, {hand="glaze", pile=BlueG, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(1 - smoothstep(0,230,y),0,1)^1.2 end})
blend(skyG, {angle=0.0})
print(wait(0))

--@ chunk 101
blend(rect(0,0,1000,300) - rockM, {angle=0.0})
blend(rect(0,0,1000,300) - rockM, {angle=0.03})
blend(rect(0,0,1000,300) - rockM, {angle=-0.03})
blend(rect(0,0,1000,300) - rockM, {angle=1.5708})
blend(rect(0,0,1000,300) - rockM, {angle=0.0})
print(wait(0))

--@ chunk 102
print(drying(600,350), drying(600,300), drying(600,250))
local zone = rect(0,230,1000,190) - R1m - rockM
blend(zone, {angle=1.5708})
blend(zone, {angle=0.0})
blend(zone, {angle=0.04})
blend(zone, {angle=-0.04})
print(wait(0))

--@ chunk 103
print(wait(4*24*60))
print(drying(600,200), drying(600,300), drying(600,380), drying(150,330), drying(400,400))

--@ chunk 104
VeilG = pile{{"lead white",6},{"yellow ochre",0.45},{"vermilion",0.14},{"chrome yellow",0.2}, medium=0.35}
zoneV = rect(0,225,1000,190) - R1m - rockM - limbsTight
work(zoneV, {hand="glaze", pile=VeilG, angle=0.0, coverage=2.0, clip=true,
  load_at=function(x,y) return clamp(smoothstep(235,300,y) * (1 - 0.72*smoothstep(345,404,y)*(0.35+0.65*bump(x,640,320))),0,1) end})
blend(zoneV, {angle=0.0})
blend(zoneV, {angle=0.03})
print(wait(0))

--@ chunk 105
cloudKeep = rect(0,0,1000,420) - R1m - rockM - limbsTight:grow(7)
cbA = lens(590, 990, 211, 12, 3, 0.6) * cloudKeep
cbB = lens(640, 930, 268, 7, 2, 0.6) * cloudKeep
cbC = lens(690, 1005, 322, 5, 2, 0.6) * cloudKeep
CBp = pile{{"lead white",6},{"smalt",0.9},{"red earth",0.25},{"raw umber",0.1}, medium=0.1}
CWp = pile{{"lead white",5},{"vermilion",0.55},{"yellow ochre",0.7},{"red earth",0.12}, medium=0.1}
for _, c in ipairs({cbA, cbB, cbC}) do
  work(c, {hand="body", tool="filbert 10", pile=CBp, angle=0.0, coverage=3.2, clip=true, fill=true, length={50,120}, angle_jitter=0.03})
end
-- warm undersides: thin bands along the bottom of each bar
uA = lens(610, 975, 219, 5, 3, 0.7) * cloudKeep
uB = lens(660, 915, 273, 3.2, 2, 0.7) * cloudKeep
uC = lens(710, 990, 325, 2.6, 2, 0.7) * cloudKeep
for _, c in ipairs({uA, uB, uC}) do
  work(c, {hand="body", tool="filbert 6", pile=CWp, angle=0.0, coverage=2.6, clip=true, fill=true, length={40,100}, angle_jitter=0.02})
end
print(wait(0))

--@ chunk 106
cl2 = (cbA + cbB + cbC):grow(14) * cloudKeep
blend(cl2, {angle=0.0})
blend(cl2, {angle=0.05})
blend(cl2, {angle=-0.04})
print(wait(0))

--@ chunk 107
big = rect(500,150,500,215) * cloudKeep
blend(big, {angle=0.0})
blend(big, {angle=0.02})
print(drying(700,214), drying(700,268))
print(wait(0))

--@ chunk 108
-- union of every limb (thin masks) so mist/sky work can spare the tree
treeAll = nil
for nm, L in pairs(OAK) do
  local m = body_of{spine=L[1], widths=L[2], blend=0.8, char="firm", amount=0.0}:mask()
  treeAll = treeAll and (treeAll + m) or m
end
treeAll = treeAll + trunkIn + rootsM
treeAllG = treeAll:grow(1.2)
mistReg = rect(-10, 372, 1020, 344) - fgM - rockM - treeAllG
MistCool = pile{{"lead white",3},{"smalt",1.6},{"red earth",0.25},{"raw umber",0.1}, medium=0.75}
work(mistReg, {hand="glaze", pile=MistCool, angle=0.0, coverage=2.4, clip=true,
  load_at=function(x,y) return clamp(smoothstep(430,690,y),0,1)^0.9 * 0.9 end})
print(wait(0))

--@ chunk 109
fogReg = rect(-10, 400, 1020, 330) - fgM - rockM - islA - treeAllG
PW1 = pile{{"lead white",7},{"yellow ochre",0.3},{"chrome yellow",0.1},{"vermilion",0.07}}
PW2 = pile{{"lead white",7},{"pale smalt",0.5},{"red earth",0.15},{"yellow ochre",0.15}}
PW3 = pile{{"lead white",6},{"smalt",1.0},{"red earth",0.25},{"raw umber",0.08}}
function fogf(f) return function(x,y) return clamp(f(x,y),0,1) end end
work(fogReg, {hand="body", tool="filbert 18", pile=PW2, angle=0.0, coverage=3.4, clip=true, fill=true, length={60,150}, angle_jitter=0.04,
  load_at=fogf(function(x,y) return smoothstep(412,470,y) end)})
work(fogReg, {hand="body", tool="filbert 18", pile=PW3, angle=0.0, coverage=3.0, clip=true, fill=true, length={60,150}, angle_jitter=0.04,
  load_at=fogf(function(x,y) return smoothstep(520,710,y)^1.1 * (0.55 + 0.45*(1-bump(x,640,420))) end)})
work(fogReg, {hand="body", tool="filbert 18", pile=PW1, angle=0.0, coverage=3.0, clip=true, fill=true, length={60,150}, angle_jitter=0.04,
  load_at=fogf(function(x,y) return bump(x,640,360) * smoothstep(412,470,y) * (1 - smoothstep(470,600,y)) end)})
blend(fogReg, {angle=0.0})
blend(fogReg, {angle=0.02})
print(wait(0))

--@ chunk 110
-- helper: left edge x of the rock at height y (from rockPts left side)
function rockLeft(y)
  local L = {{520,690},{500,700},{470,718},{440,730},{412,738},{386,744},{360,746},{346,749},{338,756},{336,768}}
  if y >= L[1][1] then return L[1][2] end
  for i = 1, #L-1 do
    local a, b = L[i], L[i+1]
    if y <= a[1] and y >= b[1] then
      local t = (a[1]-y)/(a[1]-b[1])
      return a[2] + t*(b[2]-a[2])
    end
  end
  return L[#L][2]
end
rockTop = rockM * rect(600, 300, 450, 190)
RkDark = pile{{"smalt",2.4},{"raw umber",1.1},{"Prussian blue",0.18},{"lead white",1.1},{"red earth",0.45}}
RkMid  = pile{{"smalt",1.6},{"raw umber",0.7},{"lead white",2.0},{"red earth",0.5}}
RkWarm = pile{{"yellow ochre",1.0},{"red earth",0.8},{"lead white",1.6},{"vermilion",0.2},{"smalt",0.25}}
-- broad dark-cool body, vertical strokes, fading toward the base into the mist
work(rockTop, {hand="body", tool="filbert 12", pile=RkDark, angle=function(x,y) return 1.5708 + 0.08*math.sin(x/9+y/40) end,
  coverage=3.4, clip=true, fill=true, length={25,70},
  load_at=function(x,y) return clamp(1.15 - smoothstep(420,500,y)*0.85, 0, 1) end})
-- slightly lighter mid-tone toward the lower, farther-looking part
work(rockTop, {hand="body", tool="filbert 10", pile=RkMid, angle=1.5708, coverage=2.4, clip=true, length={20,50},
  angle_jitter=0.12,
  load_at=function(x,y) return clamp(smoothstep(395,490,y)*0.8, 0, 1) end})
-- warm rim light on the glow-facing left edge
work(rockTop, {hand="body", tool="filbert 6", pile=RkWarm, angle=function(x,y) return 1.5708 + 0.12*math.sin(y/17) end,
  coverage=2.6, clip=true, length={14,40},
  load_at=function(x,y)
    local d = x - rockLeft(y)
    return clamp(math.exp(-(d/9)^2) * (1 - smoothstep(380,470,y)*0.7), 0, 1)
  end})
print(wait(0))

--@ chunk 111
function arete(y) return 794 - 0.13*(y-337) end
rockFull = rockM * rect(600, 300, 450, 215)
leftPlane  = rockFull * mask(function(x,y) return 1 - smoothstep(arete(y)-5, arete(y)+5, x) end)
rightPlane = rockFull * mask(function(x,y) return smoothstep(arete(y)-5, arete(y)+5, x) end)
RkDark2 = pile{{"Prussian blue",0.32},{"raw umber",1.3},{"smalt",1.2},{"lead white",0.45},{"red earth",0.2}}
RkLeft  = pile{{"raw umber",1.0},{"yellow ochre",0.7},{"red earth",0.7},{"lead white",1.5},{"smalt",0.55}}
RkLeftHi= pile{{"yellow ochre",0.8},{"red earth",0.6},{"lead white",2.4},{"vermilion",0.15},{"smalt",0.3}}
work(rightPlane, {hand="body", tool="filbert 10", pile=RkDark2, angle=function(x,y) return 1.5708 + 0.06*math.sin(x/11+y/50) end,
  coverage=4.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(1.1 - smoothstep(430,505,y)*0.6, 0, 1) end})
work(leftPlane, {hand="body", tool="filbert 9", pile=RkLeft, angle=function(x,y) return 1.5708 + 0.06*math.sin(x/9+y/45) end,
  coverage=4.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(1.1 - smoothstep(440,505,y)*0.6, 0, 1) end})
-- lighter warm toward the left edge of the left plane
work(leftPlane, {hand="body", tool="filbert 7", pile=RkLeftHi, angle=1.5708, coverage=2.2, clip=true, length={20,60}, angle_jitter=0.07,
  load_at=function(x,y)
    local d = x - rockLeft(y)
    return clamp(0.8*math.exp(-(d/14)^2) * (1 - smoothstep(400,480,y)*0.5), 0, 1)
  end})
blend(rockFull, {angle=1.5708})
print(wait(0))

--@ chunk 112
veilR = rect(640, 400, 380, 150) - fgM
work(veilR, {hand="body", tool="filbert 18", pile=PW2, angle=0.0, coverage=3.6, clip=true, fill=true, length={50,130}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(436,510,y)^1.2, 0, 1) end})
work(veilR, {hand="body", tool="filbert 18", pile=PW1, angle=0.0, coverage=2.6, clip=true, fill=true, length={50,130}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.6*smoothstep(470,520,y)*(1-smoothstep(900,1000,x)), 0, 1) end})
blend(veilR, {angle=0.0})
blend(veilR, {angle=0.03})
print(wait(0))

--@ chunk 113
print(wait(4*24*60))
print("rock", drying(800,380), "haze", drying(800,450), "mist", drying(500,480), "crown", drying(300,250))

--@ chunk 114
mistFix = rect(540, 394, 480, 176) - fgM
fx = function(x) return smoothstep(540, 660, x) end
-- warm cream base over the grey rectangle
work(mistFix, {hand="body", tool="filbert 18", pile=PW1, angle=0.0, coverage=4.6, clip=true, fill=true, length={60,150}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(fx(x) * (1 - 0.5*smoothstep(470,560,y)), 0, 1) end})
-- cooler lilac toward the lower part and the far right
work(mistFix, {hand="body", tool="filbert 18", pile=PW2, angle=0.0, coverage=3.6, clip=true, fill=true, length={60,150}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(fx(x) * (0.25 + 0.75*smoothstep(430,540,y)) * (0.5 + 0.5*smoothstep(700,1000,x)), 0, 1) end})
print(wait(0))

--@ chunk 115
skySky = rect(-10, -10, 1020, 440) - R1m - rockM
work(skySky, {hand="body", tool="filbert 20", pile=W0c, angle=0.0, coverage=4.6, clip=true, fill=true, length={60,160}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(1 - smoothstep(250,340,y),0,1) end})
work(skySky, {hand="body", tool="filbert 20", pile=W0w, angle=0.0, coverage=4.6, clip=true, fill=true, length={60,160}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(240,330,y),0,1) end})
print(wait(0))

--@ chunk 116
print(wait(4*24*60))
print(drying(300,100), drying(700,300), drying(600,420), drying(800,380))

--@ chunk 117
T0 = pile{{"lead white",3.2},{"smalt",3.4},{"cobalt blue",1.3}}
T1 = pile{{"lead white",4.4},{"smalt",2.5},{"cobalt blue",0.9}}
T2 = pile{{"lead white",5.6},{"pale smalt",2.0},{"cobalt blue",0.5}}
T3 = pile{{"lead white",6},{"pale smalt",1.0},{"cobalt blue",0.2},{"chrome yellow",0.25},{"Prussian blue",0.06}}
T4 = pile{{"lead white",6},{"pale smalt",0.7},{"Prussian blue",0.12},{"chrome yellow",0.5}}
T5 = pile{{"lead white",6},{"chrome yellow",0.8},{"yellow ochre",0.1},{"Prussian blue",0.03}}
T6 = pile{{"lead white",4},{"chrome yellow",1.2},{"yellow ochre",0.35},{"vermilion",0.1}}
T7 = pile{{"lead white",3},{"vermilion",0.8},{"chrome yellow",0.9},{"red earth",0.08}}
T8 = pile{{"lead white",6},{"chrome yellow",0.5},{"vermilion",0.05}}
function spr2(m, p, f, cov)
  work(m, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov or 3.2, clip=true, fill=true, length={80,200}, angle_jitter=0.04, load_at=f})
end
function vig(x) return 0.7 + 0.3*bump(x, 600, 650) end
spr2(skySky, T0, function(x,y) return clamp((1 - smoothstep(20,150,y)) * (1.1 - 0.35*bump(x,600,600)),0,1) end, 3.2)
spr2(skySky, T1, function(x,y) return clamp(bump(y,95,75),0,1) end, 3.0)
spr2(skySky, T2, function(x,y) return clamp(bump(y,175,60),0,1) end, 3.0)
spr2(skySky, T3, function(x,y) return clamp(bump(y,238,42),0,1) end, 3.0)
spr2(skySky, T4, function(x,y) return clamp(bump(y,280,36)*(0.55+0.45*bump(x,640,520)),0,1) end, 3.0)
blend(skySky * rect(0,0,1000,310), {angle=0.0})
blend(skySky * rect(0,0,1000,310), {angle=0.0})
print(wait(0))

--@ chunk 118
spr2(skySky, T5, function(x,y) return clamp(smoothstep(292,340,y),0,1) end, 3.0)
spr2(skySky, T6, function(x,y) return clamp((0.15 + 0.85*bump(x,640,360)) * smoothstep(335,400,y),0,1) end, 3.0)
spr2(skySky, T7, function(x,y) return clamp(0.95*bump(x,640,260)*smoothstep(372,410,y)^1.2*(1-0.4*coreF(x)),0,1) end, 2.8)
spr2(skySky, T8, function(x,y) return clamp(coreF(x)*smoothstep(350,412,y)^1.1,0,1) end, 2.8)
blend(skySky * rect(0,230,1000,200), {angle=0.0})
blend(skySky * rect(0,230,1000,200), {angle=0.0})
print(wait(0))

--@ chunk 119
print(wait(4*24*60))
print(drying(300,50), drying(700,250), drying(640,395), drying(200,300))

--@ chunk 120
skyG2 = rect(-10, -10, 1020, 440) - R1m - rockM
GBl = pile{{"cobalt blue",2.2},{"smalt",2.2},{"Prussian blue",0.08},{"lead white",0.3}, medium=0.8}
GTq = pile{{"Prussian blue",0.14},{"chrome yellow",0.8},{"lead white",1.2},{"pale smalt",0.6}, medium=0.8}
GPe = pile{{"vermilion",0.7},{"chrome yellow",0.55},{"red earth",0.1},{"lead white",1.0}, medium=0.8}
work(skyG2, {hand="glaze", pile=GBl, angle=0.0, coverage=2.4, clip=true,
  load_at=function(x,y) return clamp((1 - smoothstep(0,260,y))^1.0 * (0.85 + 0.15*(1-bump(x,620,500))),0,1) end})
work(skyG2, {hand="glaze", pile=GTq, angle=0.0, coverage=2.0, clip=true,
  load_at=function(x,y) return clamp(bump(y,268,34) * (0.5 + 0.5*bump(x,640,560)),0,1) end})
work(skyG2, {hand="glaze", pile=GPe, angle=0.0, coverage=2.0, clip=true,
  load_at=function(x,y) return clamp(smoothstep(345,408,y) * (0.3 + 0.7*bump(x,640,330)),0,1) end})
blend(skyG2, {angle=0.0})
print(wait(0))

--@ chunk 121
print(drying(300,60), drying(600,260), drying(600,380))
blend(skyG2, {angle=0.0})
blend(skyG2, {angle=1.5708})
blend(skyG2, {angle=0.0})
blend(skyG2, {angle=0.03})
print(wait(0))

--@ chunk 122
print(wait(5*24*60))
print(drying(300,60), drying(600,260), drying(600,380), drying(100,300))

--@ chunk 123
mistAll = rect(-10, 398, 1020, 330) - fgM - rockM - treeAllG
PW2b = pile{{"lead white",7},{"pale smalt",0.6},{"red earth",0.12},{"yellow ochre",0.12}}
PW3b = pile{{"lead white",6},{"smalt",0.9},{"red earth",0.28},{"raw umber",0.08}}
PW1b = pile{{"lead white",7},{"yellow ochre",0.35},{"chrome yellow",0.1},{"vermilion",0.09}}
function mf(f) return function(x,y) return clamp(f(x,y),0,1) end end
function mwork(p, f, cov)
  work(mistAll, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov or 3.4, clip=true, fill=true,
    length={80,200}, angle_jitter=0.035, load_at=mf(f)})
end
mwork(PW2b, function(x,y) return smoothstep(398,470,y) * (1 - 0.5*smoothstep(560,700,y)) end, 3.8)
mwork(PW3b, function(x,y) return smoothstep(500,720,y)^1.1 * (0.45 + 0.55*(1-bump(x,640,420))) end, 3.2)
mwork(PW1b, function(x,y) return bump(x,640,380) * (1 - smoothstep(420,520,y)) * smoothstep(398,412,y) end, 3.2)
blend(mistAll, {angle=0.0})
blend(mistAll, {angle=0.02})
print(wait(0))

--@ chunk 124
tst = rect(0, 30, 120, 230)
TT = pile{{"lead white",4.6},{"smalt",2.2},{"cobalt blue",0.7}, medium=0.4}
work(tst, {hand="body", tool="filbert 20", pile=TT, angle=0.0, coverage=6.0, clip=true, fill=true, length={60,140}, angle_jitter=0.04, load=1.0})
blend(tst, {angle=0.0})
print(wait(40))

--@ chunk 125
skyCoat = rect(-10, -10, 1020, 424) - rockM
T0 = pile{{"lead white",2.6},{"smalt",3.4},{"cobalt blue",1.7},{"Prussian blue",0.05}}
T1 = pile{{"lead white",3.6},{"smalt",2.8},{"cobalt blue",1.2}}
T2 = pile{{"lead white",5},{"pale smalt",2.2},{"cobalt blue",0.7}}
T3 = pile{{"lead white",6},{"pale smalt",1.2},{"cobalt blue",0.3},{"chrome yellow",0.2},{"Prussian blue",0.05}}
T4 = pile{{"lead white",6},{"pale smalt",0.6},{"Prussian blue",0.07},{"chrome yellow",0.55}}
spr2(skyCoat, T0, function(x,y) return clamp((1 - smoothstep(20,150,y)) * (1.1 - 0.3*bump(x,600,600)),0,1) end, 3.4)
spr2(skyCoat, T1, function(x,y) return clamp(bump(y,95,75),0,1) end, 3.2)
spr2(skyCoat, T2, function(x,y) return clamp(bump(y,175,60),0,1) end, 3.2)
spr2(skyCoat, T3, function(x,y) return clamp(bump(y,238,42),0,1) end, 3.2)
spr2(skyCoat, T4, function(x,y) return clamp(bump(y,282,36)*(0.55+0.45*bump(x,640,520)),0,1) end, 3.2)
blend(skyCoat * rect(0,0,1000,312), {angle=0.0})
blend(skyCoat * rect(0,0,1000,312), {angle=0.0})
print(wait(0))

--@ chunk 126
T5 = pile{{"lead white",6},{"chrome yellow",0.9},{"yellow ochre",0.15},{"Prussian blue",0.03}}
T6 = pile{{"lead white",4},{"chrome yellow",1.3},{"yellow ochre",0.4},{"vermilion",0.14}}
T7 = pile{{"lead white",3},{"vermilion",1.0},{"chrome yellow",0.9},{"red earth",0.1}}
T8 = pile{{"lead white",5.5},{"chrome yellow",0.6},{"vermilion",0.08}}
spr2(skyCoat, T5, function(x,y) return clamp(smoothstep(292,340,y),0,1) end, 3.2)
spr2(skyCoat, T6, function(x,y) return clamp((0.15 + 0.85*bump(x,640,360)) * smoothstep(335,402,y),0,1) end, 3.2)
spr2(skyCoat, T7, function(x,y) return clamp(0.95*bump(x,640,270)*smoothstep(372,412,y)^1.2*(1-0.4*coreF(x)),0,1) end, 3.0)
spr2(skyCoat, T8, function(x,y) return clamp(coreF(x)*smoothstep(350,414,y)^1.1,0,1) end, 3.0)
blend(skyCoat * rect(0,230,1000,200), {angle=0.0})
blend(skyCoat * rect(0,230,1000,200), {angle=0.0})
print(wait(0))

--@ chunk 127
print(wait(5*24*60))
print(drying(300,60), drying(600,260), drying(600,380), drying(100,300), drying(640,415))

--@ chunk 128
glowSoft = ellipse(650, 428, 470, 92):blur(46) * skyCoat
Hpeach = pile{{"lead white",3},{"vermilion",1.1},{"chrome yellow",0.8},{"red earth",0.12}}
Hgold  = pile{{"lead white",4},{"chrome yellow",1.4},{"yellow ochre",0.35},{"vermilion",0.2}}
Hcore  = pile{{"lead white",5.5},{"chrome yellow",0.7},{"vermilion",0.12}}
work(skyCoat * rect(0,300,1000,124), {hand="body", tool="filbert 22", pile=Hgold, angle=0.0, coverage=2.6, clip=true, fill=true, length={80,200}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(bump(x,650,360)*smoothstep(330,400,y)*0.9,0,1) end})
work(skyCoat * rect(0,300,1000,124), {hand="body", tool="filbert 22", pile=Hpeach, angle=0.0, coverage=2.6, clip=true, fill=true, length={80,200}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(bump(x,650,300)*smoothstep(366,418,y)^1.2*(1-0.45*bump(x,640,110)),0,1) end})
work(skyCoat * rect(0,300,1000,124), {hand="body", tool="filbert 22", pile=Hcore, angle=0.0, coverage=2.6, clip=true, fill=true, length={80,200}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(bump(x,640,130)*smoothstep(345,418,y)^1.2,0,1) end})
blend(glowSoft, {angle=0.0})
blend(glowSoft, {angle=0.0})
print(wait(0))

--@ chunk 129
function hillpoly(top, y1, x0, x1)
  local p = {}
  for _, q in ipairs(top) do p[#p+1] = q end
  p[#p+1] = {top[#top][1], y1}
  p[#p+1] = {top[1][1], y1}
  return poly(p, true):roughen(1.3, 16, 4)
end
function hill(m, pile_, y0, y1, tool, ang)
  work(m, {hand="body", tool=tool or "filbert 14", pile=pile_, angle=ang or 0.05, coverage=3.6, clip=true, fill=true, length={30,90},
    angle_jitter=0.08, load_at=function(x,y) return clamp(1.0 - smoothstep(y0, y1, y), 0.0, 1.0)^0.8 end})
end
-- A: farthest ridge
A_top = {{-10,396},{40,388},{90,380},{140,376},{190,380},{240,388},{290,394},{340,391},{390,397},{450,403},{520,408},{600,411},{680,408},{740,402},{800,395},{860,386},{920,378},{980,372},{1010,370}}
mA = hillpoly(A_top, 480)
PA = pile{{"lead white",4.2},{"pale smalt",1.2},{"smalt",0.6},{"red earth",0.3},{"yellow ochre",0.15}}
hill(mA - rockM, PA, 400, 450)
-- A2: hazy second ridges, left hump and right low ridge
A2L_top = {{-10,438},{50,431},{110,425},{170,421},{230,425},{290,431},{345,429},{395,434},{445,441},{495,449}}
A2R_top = {{770,448},{820,441},{870,437},{925,439},{975,444},{1010,448}}
mA2L = hillpoly(A2L_top, 500)
mA2R = hillpoly(A2R_top, 500) - rockM
PA2 = pile{{"lead white",3.8},{"smalt",1.5},{"red earth",0.3},{"raw umber",0.1}}
hill(mA2L, PA2, 432, 480)
hill(mA2R, PA2, 440, 485)
-- B: wooded dome (centre)
B_top = {{425,548},{452,522},{478,505},{505,492},{535,481},{566,473},{598,470},{630,473},{662,481},{690,493},{716,509},{745,528},{772,552}}
mB = hillpoly(B_top, 600)
PB = pile{{"lead white",2.6},{"smalt",2.0},{"raw umber",0.5},{"red earth",0.28},{"Prussian blue",0.06}}
hill(mB, PB, 500, 570)
-- C: left, behind oak; D: right wooded hill
C_top = {{-10,490},{40,482},{90,476},{140,474},{190,478},{232,486},{256,494}}
D_top = {{755,526},{800,509},{850,498},{900,494},{950,498},{1010,507}}
mC = hillpoly(C_top, 580) - treeAllG
mD = hillpoly(D_top, 600) - rockM
PD = pile{{"smalt",2.2},{"raw umber",0.65},{"Prussian blue",0.14},{"lead white",1.3},{"red earth",0.22}}
hill(mC, PD, 500, 575)
hill(mD, PD, 510, 590)
print(wait(0))

--@ chunk 130
print(wait(4*24*60))
print(drying(600,500), drying(100,500), drying(300,400), drying(850,560))

--@ chunk 131
rockLow = rockM * rect(600, 388, 450, 170)
RkD2 = pile{{"Prussian blue",0.3},{"raw umber",1.3},{"smalt",1.3},{"lead white",0.5},{"red earth",0.2}}
RkL  = pile{{"raw umber",1.0},{"yellow ochre",0.75},{"red earth",0.7},{"lead white",1.5},{"smalt",0.5}}
RkLH = pile{{"yellow ochre",0.8},{"red earth",0.6},{"lead white",2.4},{"vermilion",0.15},{"smalt",0.3}}
rP = rockLow * mask(function(x,y) return smoothstep(arete(y)-5, arete(y)+5, x) end)
lP = rockLow * mask(function(x,y) return 1 - smoothstep(arete(y)-5, arete(y)+5, x) end)
vert = function(x,y) return 1.5708 + 0.07*math.sin(x/10 + y/45) end
-- body: fade upward into the existing painted top (load low at the very top)
work(rP, {hand="body", tool="filbert 10", pile=RkD2, angle=vert, coverage=4.2, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(388,402,y) * (1 - smoothstep(470,520,y)), 0, 1) end})
work(lP, {hand="body", tool="filbert 9", pile=RkL, angle=vert, coverage=4.2, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(388,402,y) * (1 - smoothstep(465,520,y)), 0, 1) end})
work(lP, {hand="body", tool="filbert 7", pile=RkLH, angle=1.5708, coverage=2.2, clip=true, length={20,60}, angle_jitter=0.07,
  load_at=function(x,y)
    local d = x - rockLeft(y)
    return clamp(0.8*math.exp(-(d/14)^2) * smoothstep(392,408,y) * (1 - smoothstep(420,480,y)), 0, 1)
  end})
-- forested talus/slope below: dark blue-green, takes over from y 455 to 500
work(rockLow, {hand="body", tool="filbert 10", pile=PD, angle=0.05, coverage=4.0, clip=true, fill=true, length={25,70}, angle_jitter=0.2,
  load_at=function(x,y) return clamp(smoothstep(452,505,y), 0, 1) end})
-- strata: ledge lines
function ledge(y0, x0, x1, amp, per)
  return mask(function(x,y)
    if x < x0 or x > x1 then return 0 end
    local c = y0 + amp*math.sin(x/per + y0)
    return 1 - smoothstep(0.7, 1.7, math.abs(y - c))
  end)
end
ledges = (ledge(412, 744, 880, 1.6, 9) + ledge(428, 738, 885, 1.8, 11) + ledge(446, 728, 892, 2.0, 8)) * rockLow
LedgeD = pile{{"Prussian blue",0.3},{"raw umber",1.4},{"smalt",1.0},{"red earth",0.3}}
work(ledges, {hand="detail", pile=LedgeD, coverage=1.3, clip=true, angle=0.02, length={6,18}, pressure={0.2,0.45}, load=0.7,
  load_at=function(x,y) return clamp(1 - smoothstep(450,480,y),0,1) end})
print(wait(0))

--@ chunk 132
print(wait(3*24*60))
print(drying(800,430), drying(800,560), drying(760,520))

--@ chunk 133
-- wall shape of the butte (steep sides), drawn by hand
wallL = {{742,392},{741,410},{739,430},{737,452},{735,474},{733,500}}
wallR = {{874,392},{876,410},{879,430},{882,452},{886,474},{890,500}}
local p = {}
for _, q in ipairs(wallL) do p[#p+1] = q end
for i = #wallR, 1, -1 do p[#p+1] = wallR[i] end
wallsM = poly(p, true):roughen(1.4, 14, 9)
-- (1) restore mist around the walls where the flare was
flareOut = rockLow - wallsM:grow(2)
work(flareOut, {hand="body", tool="filbert 22", pile=PW2b, angle=0.0, coverage=4.4, clip=true, fill=true, length={60,160}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(smoothstep(392,440,y),0,1) end})
work(flareOut, {hand="body", tool="filbert 22", pile=PW1b, angle=0.0, coverage=3.0, clip=true, fill=true, length={60,160}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(0.8*(1 - smoothstep(392,470,y))*smoothstep(380,400,y),0,1) end})
-- (2) walls: full-strength dark body, warm left, dark right
wallsLow = wallsM * rect(700, 390, 230, 130)
wR = wallsLow * mask(function(x,y) return smoothstep(arete(y)-5, arete(y)+5, x) end)
wL = wallsLow * mask(function(x,y) return 1 - smoothstep(arete(y)-5, arete(y)+5, x) end)
work(wR, {hand="body", tool="filbert 10", pile=RkD2, angle=vert, coverage=5.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(388,400,y),0,1) end})
work(wL, {hand="body", tool="filbert 9", pile=RkL, angle=vert, coverage=5.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(388,400,y),0,1) end})
work(wL, {hand="body", tool="filbert 7", pile=RkLH, angle=1.5708, coverage=2.4, clip=true, length={20,60}, angle_jitter=0.07,
  load_at=function(x,y) local d = x - rockLeft(y); return clamp(0.8*math.exp(-(d/12)^2) * smoothstep(392,408,y) * (1 - smoothstep(430,490,y)), 0, 1) end})
print(wait(0))

--@ chunk 134
mistR = rect(560, 394, 470, 330) - fgM
ramp = function(x,y) return smoothstep(396, 440, y) * smoothstep(560, 700, x) end
wk = function(p, f, cov, tool)
  work(mistR, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={70,180}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
wk(PW2b, function(x,y) return ramp(x,y) end, 4.6)
wk(PW1b, function(x,y) return ramp(x,y) * bump(x,650,420) * (1 - smoothstep(430,520,y)) end, 3.2)
wk(PW3b, function(x,y) return ramp(x,y) * smoothstep(500,600,y) * 0.8 end, 3.2)
print(wait(0))

--@ chunk 135
function interp(pts, x)
  if x <= pts[1][1] then return pts[1][2] end
  for i = 1, #pts-1 do
    local a, b = pts[i], pts[i+1]
    if x >= a[1] and x <= b[1] then
      local t = (x - a[1]) / (b[1]-a[1])
      return a[2] + t*(b[2]-a[2])
    end
  end
  return pts[#pts][2]
end
function Atop(x) return interp(A_top, x) end
coatM = mask(function(x,y)
  if y < Atop(x) then return 0 end
  if y > crestY(x) - 1.5 then return 0 end
  local inW = smoothstep(733,740,x) * (1 - smoothstep(880,887,x))
  local keep = 1 - inW * (1 - smoothstep(425,452,y))
  return keep
end)
function cw(p, f, cov)
  work(coatM, {hand="body", tool="filbert 24", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
PWpeach = pile{{"lead white",6},{"yellow ochre",0.3},{"vermilion",0.14},{"chrome yellow",0.14}}
PWridge = pile{{"lead white",4.2},{"pale smalt",1.2},{"smalt",0.6},{"red earth",0.3},{"yellow ochre",0.15}}
cw(PW2b, function(x,y) return 1 end, 4.6)
cw(PWpeach, function(x,y) return bump(x,640,380) * (1 - smoothstep(Atop(x)+10, Atop(x)+110, y)) end, 3.2)
cw(PW3b, function(x,y) return smoothstep(500,640,y)*0.85 end, 3.2)
cw(PWridge, function(x,y) return (1 - smoothstep(Atop(x)+3, Atop(x)+34, y)) * 0.95 end, 3.4)
print(wait(0))

--@ chunk 136
print(wait(5*24*60))
print(drying(600,450), drying(200,500), drying(800,560), drying(800,430))

--@ chunk 137
-- ===== far ridges and wooded hills over the dry mist =====
PA3 = pile{{"lead white",3.3},{"pale smalt",1.4},{"smalt",0.9},{"red earth",0.35},{"yellow ochre",0.1}}
PA4 = pile{{"lead white",3.0},{"smalt",1.5},{"red earth",0.35},{"raw umber",0.12}}
local ridgeBand = mask(function(x,y)
  local t = Atop(x)
  if y < t - 1 then return 0 end
  if y > t + 70 then return 0 end
  local inW = smoothstep(733,740,x) * (1 - smoothstep(880,887,x))
  return 1 - inW*(1 - smoothstep(0,1,(y-t)/1000))  -- the butte is repainted below
end) * (-rockM)
work(ridgeBand, {hand="body", tool="filbert 14", pile=PA3, angle=0.02, coverage=3.4, clip=true, fill=true, length={40,110}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(1 - smoothstep(Atop(x)+2, Atop(x)+62, y), 0, 1)^0.9 end})
-- A2 humps
function topfn(pts) return function(x) return interp(pts, x) end end
A2Lt, A2Rt = topfn(A2L_top), topfn(A2R_top)
mA2Lb = mask(function(x,y) return (x<500 and y>=A2Lt(x)-1 and y<=A2Lt(x)+70) and 1 or 0 end) * (-fgM)
mA2Rb = mask(function(x,y) return (x>765 and y>=A2Rt(x)-1 and y<=A2Rt(x)+70) and 1 or 0 end) * (-rockM) * (-fgM)
work(mA2Lb, {hand="body", tool="filbert 12", pile=PA4, angle=0.03, coverage=3.4, clip=true, fill=true, length={40,100}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(1 - smoothstep(A2Lt(x)+2, A2Lt(x)+52, y), 0, 1)^0.9 end})
work(mA2Rb, {hand="body", tool="filbert 12", pile=PA4, angle=0.03, coverage=3.4, clip=true, fill=true, length={40,100}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(1 - smoothstep(A2Rt(x)+2, A2Rt(x)+52, y), 0, 1)^0.9 end})
print(wait(0))

--@ chunk 138
-- ===== the butte, whole =====
buttePts = {{746,362},{743,380},{741,400},{739,425},{736,452},{731,480},{722,506},{760,530},{830,534},{900,520},{896,505},{888,480},{882,452},{878,425},{875,400},{871,380},{869,350},{866,337},{858,332},{838,331},{815,334},{790,333},{768,336},{756,338},{749,346}}
butteM = poly(buttePts, true):roughen(1.3, 14, 9)
function arete2(y) return 792 - 0.12*(y-337) end
bR = butteM * mask(function(x,y) return smoothstep(arete2(y)-5, arete2(y)+5, x) end)
bL = butteM * mask(function(x,y) return 1 - smoothstep(arete2(y)-5, arete2(y)+5, x) end)
vert2 = function(x,y) return 1.5708 + 0.07*math.sin(x/10 + y/45) end
local fadeB = function(y) return 1 - smoothstep(440, 500, y) end
work(bR, {hand="body", tool="filbert 10", pile=RkD2, angle=vert2, coverage=4.8, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(fadeB(y)*1.05, 0, 1) end})
work(bL, {hand="body", tool="filbert 9", pile=RkL, angle=vert2, coverage=4.8, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(fadeB(y)*1.05, 0, 1) end})
-- lit rim on the glow side
function leftEdge(y) return interp({{332,747},{362,746},{400,741},{440,737},{480,731},{520,722}}, y) end
work(bL, {hand="body", tool="filbert 7", pile=RkLH, angle=1.5708, coverage=2.4, clip=true, length={20,60}, angle_jitter=0.07,
  load_at=function(x,y) local d = x - leftEdge(y); return clamp(0.85*math.exp(-(d/13)^2) * fadeB(y), 0, 1) end})
-- cool reflected light and weathering streaks on the shadow face
RkStreak = pile{{"smalt",1.2},{"raw umber",0.6},{"lead white",1.2},{"red earth",0.2}}
work(bR, {hand="detail", pile=RkStreak, angle=1.5708, coverage=0.7, clip=true, length={14,40}, pressure={0.15,0.4}, load=0.6, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(arete2(y)+14, arete2(y)+60, x) * fadeB(y) * 0.8, 0, 1) end})
-- ledges
ledges2 = (ledge(402, 744, 872, 1.6, 9) + ledge(421, 740, 878, 1.8, 11) + ledge(441, 735, 884, 2.0, 8)) * butteM
work(ledges2, {hand="detail", pile=LedgeD, coverage=1.2, clip=true, angle=0.02, length={6,18}, pressure={0.2,0.45}, load=0.7,
  load_at=function(x,y) return clamp(1 - smoothstep(430,470,y),0,1) end})
print(wait(0))

--@ chunk 139
-- ===== wooded hills with ragged tree-line tops =====
trn = noise{seed=5, period=13, octaves=2, persistence=0.55}
function hillMask(pts, amp, y1, xlo, xhi)
  return mask(function(x,y)
    if x < xlo or x > xhi then return 0 end
    local t = interp(pts, x) + amp*trn(x*1.0, 3.0)
    return smoothstep(t-0.6, t+0.6, y) * (y < y1 and 1 or 0)
  end)
end
B2_top = {{425,552},{448,528},{470,512},{496,498},{524,488},{552,480},{578,474},{604,472},{630,475},{656,481},{682,490},{708,503},{734,518},{758,538},{775,556}}
C2_top = {{-10,492},{40,484},{90,479},{140,478},{190,482},{232,490},{270,501},{300,512},{325,527},{345,545}}
D2_top = {{880,524},{905,508},{935,499},{970,499},{1010,505}}
mB2 = hillMask(B2_top, 2.6, 640, 420, 780) * (-fgM)
mC2 = hillMask(C2_top, 2.4, 640, -10, 350) * (-fgM)
mD2 = hillMask(D2_top, 2.2, 640, 870, 1010) * (-fgM)
PB2 = pile{{"lead white",2.4},{"smalt",2.0},{"raw umber",0.45},{"red earth",0.3},{"Prussian blue",0.06}}
PC2 = pile{{"lead white",2.0},{"smalt",1.9},{"raw umber",0.55},{"red earth",0.3},{"Prussian blue",0.08}}
PD2 = pile{{"smalt",2.0},{"raw umber",0.65},{"Prussian blue",0.14},{"lead white",1.4},{"red earth",0.22}}
function woodHill(m, p, y0, y1, ang)
  work(m, {hand="body", tool="filbert 12", pile=p, angle=ang or 0.04, coverage=4.0, clip=true, fill=true, length={25,70}, angle_jitter=0.2,
    load_at=function(x,y) return clamp(1.0 - smoothstep(y0, y1, y), 0, 1)^0.75 end})
end
woodHill(mB2, PB2, 505, 585)
woodHill(mC2, PC2, 505, 580)
woodHill(mD2, PD2, 515, 575)
-- horizon strip: restore the glow where the old mist rect lay over it
strip = rect(520, 386, 240, 30) - butteM
work(strip, {hand="body", tool="filbert 10", pile=Hgold, angle=0.0, coverage=3.0, clip=true, fill=true, length={30,90}, angle_jitter=0.03,
  load_at=function(x,y) return clamp(smoothstep(386,398,y) * (1 - smoothstep(Atop(x)-2, Atop(x)+1, y)),0,1) end})
work(strip, {hand="body", tool="filbert 10", pile=Hpeach, angle=0.0, coverage=2.4, clip=true, fill=true, length={30,90}, angle_jitter=0.03,
  load_at=function(x,y) return clamp(smoothstep(396,408,y) * (1 - smoothstep(Atop(x)-2, Atop(x)+1, y)),0,1) end})
print(wait(0))

--@ chunk 140
print(wait(6*24*60))
print(drying(600,520), drying(640,400), drying(800,480), drying(100,520), drying(940,540))

--@ chunk 141
-- ===== Step 1: repair the horizon strip, soft-edged =====
function sb(v, a, b, c, d) return smoothstep(a, b, v) * (1 - smoothstep(c, d, v)) end
GZ = mask(function(x,y)
  if y > Atop(x) + 1 then return 0 end
  return sb(x, 470, 560, 760, 830) * smoothstep(352, 392, y)
end) * (-butteM)
work(GZ, {hand="body", tool="filbert 14", pile=T5, angle=0.0, coverage=3.0, clip=true, fill=true, length={30,90}, angle_jitter=0.03,
  load_at=function(x,y) return clamp(smoothstep(340,380,y),0,1) end})
work(GZ, {hand="body", tool="filbert 14", pile=Hgold, angle=0.0, coverage=2.6, clip=true, fill=true, length={30,90}, angle_jitter=0.03,
  load_at=function(x,y) return clamp(0.8*bump(x,650,330)*smoothstep(360,402,y),0,1) end})
work(GZ, {hand="body", tool="filbert 14", pile=Hcore, angle=0.0, coverage=2.6, clip=true, fill=true, length={30,90}, angle_jitter=0.03,
  load_at=function(x,y) return clamp(bump(x,640,120)*smoothstep(372,410,y),0,1) end})
blend(GZ:grow(14), {angle=0.0})
print(wait(0))

--@ chunk 142
skyAboveRidge = mask(function(x,y) return (y <= Atop(x) + 0.5) and 1 or 0 end) - butteM
GLOW = ellipse(650, 398, 560, 76):blur(36) * skyAboveRidge
Hpe2 = pile{{"lead white",3.4},{"vermilion",0.9},{"chrome yellow",0.8},{"yellow ochre",0.2},{"red earth",0.08}}
work(GLOW, {hand="body", tool="filbert 22", pile=Hpe2, angle=0.0, coverage=3.0, clip=true, fill=true, length={80,200}, angle_jitter=0.03,
  load_at=function(x,y) return clamp(smoothstep(372,412,y)^1.1 * (0.55+0.45*bump(x,640,420)),0,1) end})
work(GLOW, {hand="body", tool="filbert 22", pile=Hcore, angle=0.0, coverage=2.6, clip=true, fill=true, length={80,200}, angle_jitter=0.03,
  load_at=function(x,y) return clamp(bump(x,640,150)*smoothstep(366,412,y)^1.2*0.9,0,1) end})
blend(GLOW:grow(10), {angle=0.0})
blend(GLOW:grow(10), {angle=0.02})
print(wait(0))

--@ chunk 143
fogn = noise{seed=31, period=260, octaves=3, persistence=0.5, stretch={0, 3}}
function fq(x,y) return clamp(0.55+0.75*fogn(x,y), 0, 1) end
function fogwork(m, p, f, cov)
  work(m, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov or 3.0, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
M1 = mask(function(x,y) return smoothstep(380,520,x) * smoothstep(436,466,y) * (1-smoothstep(500,545,y)) end) * (-mB2:grow(1)) * (-butteM) * (-fgM)
fogwork(M1, PW2b, function(x,y) return fq(x,y) end, 3.4)
fogwork(M1, PW1b, function(x,y) return fq(x+90,y)*bump(x,660,360)*(1-smoothstep(470,520,y)) end, 2.6)
M2 = mask(function(x,y) return smoothstep(380,470,x) * smoothstep(528,572,y) * (1-smoothstep(640,700,y)) end) * (-fgM) * (-butteM)
fogwork(M2, PW2b, function(x,y) return 0.35+0.65*fq(x,y) end, 3.6)
fogwork(M2, PW1b, function(x,y) return 0.8*fq(x+40,y)*bump(x,700,300) end, 2.4)
M2D = mask(function(x,y) return smoothstep(840,885,x) * smoothstep(486,520,y) * (1-smoothstep(600,640,y)) end) * (-fgM) * (-butteM)
fogwork(M2D, PW2b, function(x,y) return 0.5+0.5*fq(x,y) end, 3.6)
print(wait(0))

--@ chunk 144
print(wait(6*24*60))
print(drying(600,500), drying(640,420), drying(800,560), drying(450,500), drying(150,520))

--@ chunk 145
midReg = mask(function(x,y) return (y > Atop(x) + 2 and y < crestY(x) - 2) and 1 or 0 end) - butteM
GWm = pile{{"vermilion",0.55},{"chrome yellow",0.75},{"yellow ochre",0.2},{"lead white",1.1}, medium=0.85}
GCm = pile{{"smalt",1.6},{"red earth",0.35},{"cobalt blue",0.35},{"lead white",1.4}, medium=0.85}
work(midReg, {hand="glaze", pile=GWm, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(bump(x,640,430) * (1 - smoothstep(430,540,y)) * smoothstep(Atop(x)-2, Atop(x)+14, y) ,0,1) end})
work(midReg, {hand="glaze", pile=GCm, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(smoothstep(490,690,y)^1.0 * (0.55 + 0.45*(1-bump(x,640,420))) ,0,1) end})
blend(midReg, {angle=0.0})
blend(midReg, {angle=0.02})
print(wait(0))

--@ chunk 146
VeilM = pile{{"lead white",6},{"pale smalt",0.6},{"red earth",0.08},{"yellow ochre",0.12}, medium=0.45}
work(midReg, {hand="glaze", pile=VeilM, angle=0.0, coverage=2.4, clip=true,
  load_at=function(x,y) return clamp(0.9 * smoothstep(Atop(x)-2, Atop(x)+12, y) * (1 - 0.5*smoothstep(500,600,y)) * (0.75 + 0.25*(1-bump(x,640,300))), 0, 1) end})
blend(midReg, {angle=0.0})
blend(midReg, {angle=0.02})
print(wait(0))

--@ chunk 147
-- (1) butte base: mist wraps it
buBase = rect(690, 455, 260, 110) - fgM
work(buBase, {hand="body", tool="filbert 20", pile=PW2b, angle=0.0, coverage=4.0, clip=true, fill=true, length={60,160}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(smoothstep(462,515,y) * (0.9 + 0.1*fq(x,y)), 0, 1) end})
work(buBase, {hand="body", tool="filbert 20", pile=PW1b, angle=0.0, coverage=2.6, clip=true, fill=true, length={60,160}, angle_jitter=0.04,
  load_at=function(x,y) return clamp(0.8*smoothstep(478,520,y)*(1-smoothstep(530,560,y))*fq(x+50,y), 0, 1) end})
-- (2) far ridge A and hazy humps re-established, pale lilac, softly fading into the fog
PA5 = pile{{"lead white",3.2},{"pale smalt",1.2},{"smalt",1.0},{"red earth",0.4},{"raw umber",0.08}}
PA6 = pile{{"lead white",2.8},{"smalt",1.6},{"red earth",0.4},{"raw umber",0.14}}
ridgeB = mask(function(x,y)
  local t = Atop(x)
  return (y >= t - 0.5 and y <= t + 60) and 1 or 0
end) - butteM
work(ridgeB, {hand="body", tool="filbert 12", pile=PA5, angle=0.02, coverage=3.6, clip=true, fill=true, length={40,110}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.95*(1 - smoothstep(Atop(x)+1, Atop(x)+46, y)), 0, 1)^0.85 end})
work(mA2Lb, {hand="body", tool="filbert 12", pile=PA6, angle=0.03, coverage=3.6, clip=true, fill=true, length={40,100}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.95*(1 - smoothstep(A2Lt(x)+1, A2Lt(x)+46, y)), 0, 1)^0.85 end})
work(mA2Rb, {hand="body", tool="filbert 12", pile=PA6, angle=0.03, coverage=3.6, clip=true, fill=true, length={40,100}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.95*(1 - smoothstep(A2Rt(x)+1, A2Rt(x)+46, y)), 0, 1)^0.85 end})
print(wait(0))

--@ chunk 148
-- ===== rebuild of the mid-ground, step 1: one coherent opaque fog coat over the whole valley =====
coatM2 = mask(function(x,y)
  if y < Atop(x) then return 0 end
  if y > crestY(x) - 1.5 then return 0 end
  local inW = smoothstep(733,740,x) * (1 - smoothstep(880,887,x))
  return 1 - inW * (1 - smoothstep(428,456,y))
end)
function cw2(p, f, cov)
  work(coatM2, {hand="body", tool="filbert 24", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
cw2(PW2b, function(x,y) return 1 end, 4.6)
cw2(PWpeach, function(x,y) return bump(x,640,420) * (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) end, 3.2)
cw2(PW1b, function(x,y) return 0.85*bump(x,660,380) * smoothstep(Atop(x)+90, Atop(x)+150, y) * (1 - smoothstep(500,590,y)) end, 3.0)
cw2(PW3b, function(x,y) return smoothstep(520,660,y) * (0.45 + 0.55*(1 - bump(x,650,420))) end, 3.2)
blend(coatM2, {angle=0.0})
blend(coatM2, {angle=0.02})
print(wait(0))

--@ chunk 149
print(wait(3*24*60))
print(drying(300,450), drying(640,430), drying(800,560), drying(450,500), drying(150,520))

--@ chunk 150
-- ===== step 2: far ridges (crisp tops, lost bases) =====
PAw = pile{{"lead white",3.4},{"red earth",0.42},{"yellow ochre",0.18},{"pale smalt",1.0},{"smalt",0.25}}
PAc = pile{{"lead white",3.2},{"pale smalt",1.3},{"smalt",1.0},{"red earth",0.3},{"yellow ochre",0.08}}
ridgeA = mask(function(x,y)
  local t = Atop(x)
  return (y >= t - 0.4 and y <= t + 64) and 1 or 0
end) * (-fgM)
-- the butte stands in front of ridge A: keep paint off it
ridgeA = ridgeA - butteM:grow(1)
work(ridgeA, {hand="body", tool="filbert 12", pile=PAc, angle=0.02, coverage=3.8, clip=true, fill=true, length={40,110}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.97*(1 - smoothstep(Atop(x)+1, Atop(x)+50, y)), 0, 1)^0.85 end})
work(ridgeA, {hand="body", tool="filbert 12", pile=PAw, angle=0.02, coverage=3.2, clip=true, fill=true, length={40,110}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.95*bump(x,640,330)*(1 - smoothstep(Atop(x)+1, Atop(x)+46, y)), 0, 1)^0.85 end})
-- A2 left, tapering into the fog at its right end
A2L_top = {{-10,438},{50,431},{110,425},{170,421},{230,425},{290,431},{345,429},{395,434},{445,441},{495,449},{540,462},{580,480}}
A2R_top = {{740,452},{770,448},{820,441},{870,437},{925,439},{975,444},{1010,448}}
A2Lt, A2Rt = topfn(A2L_top), topfn(A2R_top)
mA2L = mask(function(x,y) local t = A2Lt(x); return (x < 600 and y >= t-0.4 and y <= t+72) and 1 or 0 end) * (-fgM)
mA2R = mask(function(x,y) local t = A2Rt(x); return (x > 730 and y >= t-0.4 and y <= t+72) and 1 or 0 end) * (-fgM) - butteM:grow(1)
work(mA2L, {hand="body", tool="filbert 12", pile=PA6, angle=0.03, coverage=3.8, clip=true, fill=true, length={40,100}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.95*(1 - smoothstep(A2Lt(x)+1, A2Lt(x)+50, y)) * (1 - smoothstep(455,585,x)), 0, 1)^0.85 end})
work(mA2R, {hand="body", tool="filbert 12", pile=PA6, angle=0.03, coverage=3.8, clip=true, fill=true, length={40,100}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.95*(1 - smoothstep(A2Rt(x)+1, A2Rt(x)+50, y)) * smoothstep(735,820,x), 0, 1)^0.85 end})
print(wait(0))

--@ chunk 151
-- ===== step 3: wooded hills B, C, D: ragged tree-line tops, bases lost in the fog =====
B3_top = {{400,566},{432,540},{462,518},{490,503},{514,494},{538,488},{562,484},{592,482},{622,486},{652,494},{682,503},{714,512},{744,524},{772,538},{800,556},{830,578}}
C3_top = {{-10,492},{40,484},{90,479},{140,478},{190,482},{232,490},{270,501},{300,512},{325,527},{345,545},{370,566}}
D3_top = {{840,552},{868,532},{892,516},{918,504},{948,498},{980,498},{1010,503}}
function topMask(pts, amp, xlo, xhi)
  return mask(function(x,y)
    if x < xlo or x > xhi then return 0 end
    local t = interp(pts, x) + amp*trn(x*1.0, 3.0)
    return smoothstep(t-0.6, t+0.6, y)
  end)
end
mB3 = topMask(B3_top, 2.6, 395, 835) * (-fgM)
mC3 = topMask(C3_top, 2.4, -10, 372) * (-fgM)
mD3 = topMask(D3_top, 2.2, 836, 1010) * (-fgM)
PB3 = pile{{"lead white",2.3},{"smalt",2.0},{"raw umber",0.45},{"red earth",0.3},{"Prussian blue",0.06}}
PC3 = pile{{"lead white",1.9},{"smalt",1.9},{"raw umber",0.6},{"red earth",0.3},{"Prussian blue",0.08}}
PD3 = pile{{"smalt",2.0},{"raw umber",0.7},{"Prussian blue",0.14},{"lead white",1.3},{"red earth",0.22}}
function woodHill2(m, p, topf, y0, y1, xfade)
  work(m, {hand="body", tool="filbert 12", pile=p, angle=0.04, coverage=4.2, clip=true, fill=true, length={25,70}, angle_jitter=0.2,
    load_at=function(x,y)
      local f = clamp(1.0 - smoothstep(y0, y1, y), 0, 1)^0.75
      if xfade then f = f * xfade(x) end
      return f
    end})
end
woodHill2(mB3, PB3, B3_top, 505, 590, function(x) return smoothstep(395,440,x) * (1 - smoothstep(780,835,x)) end)
woodHill2(mC3, PC3, C3_top, 505, 585, function(x) return 1 - smoothstep(330,372,x) end)
woodHill2(mD3, PD3, D3_top, 515, 580, function(x) return smoothstep(836,880,x) end)
print(wait(0))

--@ chunk 152
-- ===== butte lower body, emerging from the fog =====
buLow = butteM * rect(700, 418, 260, 100)
bRl = buLow * mask(function(x,y) return smoothstep(arete2(y)-5, arete2(y)+5, x) end)
bLl = buLow * mask(function(x,y) return 1 - smoothstep(arete2(y)-5, arete2(y)+5, x) end)
fadeLow = function(y) return smoothstep(418,432,y) * (1 - smoothstep(462, 506, y)) end
work(bRl, {hand="body", tool="filbert 10", pile=RkD2, angle=vert2, coverage=5.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(fadeLow(y)*1.05, 0, 1) end})
work(bLl, {hand="body", tool="filbert 9", pile=RkL, angle=vert2, coverage=5.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(fadeLow(y)*1.05, 0, 1) end})
work(bLl, {hand="body", tool="filbert 7", pile=RkLH, angle=1.5708, coverage=2.4, clip=true, length={20,60}, angle_jitter=0.07,
  load_at=function(x,y) local d = x - leftEdge(y); return clamp(0.8*math.exp(-(d/13)^2) * fadeLow(y), 0, 1) end})
-- a few more ledges and weathering streaks lower down
ledges3 = (ledge(458, 733, 888, 1.8, 10) + ledge(476, 730, 892, 2.0, 9)) * butteM
work(ledges3, {hand="detail", pile=LedgeD, coverage=1.1, clip=true, angle=0.02, length={6,18}, pressure={0.2,0.4}, load=0.6,
  load_at=function(x,y) return clamp(1 - smoothstep(455,490,y),0,1) end})
print(wait(0))

--@ chunk 153
-- ===== butte: talus shoulders, so it grows out of a massif rather than standing like a stump =====
shL = poly({{748,398},{736,406},{720,418},{704,434},{690,452},{678,472},{668,494},{664,520},{745,524}}, true):roughen(1.4, 15, 21)
shR = poly({{876,432},{892,442},{912,454},{934,468},{958,480},{984,490},{1010,496},{1010,525},{876,525}}, true):roughen(1.4, 15, 22)
PSh = pile{{"lead white",1.6},{"smalt",2.0},{"raw umber",0.55},{"red earth",0.28},{"Prussian blue",0.1}}
PShW = pile{{"yellow ochre",0.8},{"red earth",0.6},{"lead white",2.2},{"vermilion",0.12},{"smalt",0.35}}
function shfade(top0) return function(x,y) return clamp(1.0 - smoothstep(top0, top0+70, y), 0, 1)^0.6 end end
work(shL, {hand="body", tool="filbert 12", pile=PSh, angle=0.25, coverage=4.2, clip=true, fill=true, length={25,70}, angle_jitter=0.2,
  load_at=function(x,y) return clamp(1.0 - smoothstep(470, 515, y), 0, 1)^0.7 end})
work(shR, {hand="body", tool="filbert 12", pile=PSh, angle=-0.25, coverage=4.2, clip=true, fill=true, length={25,70}, angle_jitter=0.2,
  load_at=function(x,y) return clamp(1.0 - smoothstep(485, 520, y), 0, 1)^0.7 end})
-- warm light along the upper slope of the left shoulder
shLrim = shL:rim(9, 7) * mask(function(x,y) return 1 - smoothstep(440, 480, y) end)
work(shLrim, {hand="body", tool="filbert 6", pile=PShW, angle=0.5, coverage=1.8, clip=shL, length={14,40}, angle_jitter=0.15,
  load_at=function(x,y) return 0.65 end})
print(wait(0))

--@ chunk 154
print(wait(3*24*60))
print(drying(560,520), drying(900,520), drying(780,450), drying(100,520))

--@ chunk 155
fogProf = {{630,548},{690,530},{730,504},{770,478},{820,472},{870,486},{920,520},{970,544},{1010,552}}
fogwisp = noise{seed=77, period=95, octaves=2, persistence=0.5, stretch={0, 3.5}}
fogBehind = rect(630, 440, 390, 170) - fgM - mB3 - mD3
function fogTop(x) return interp(fogProf, x) + 11*fogwisp(x, 0) end
fb = function(p, f, cov)
  work(fogBehind, {hand="body", tool="filbert 20", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={60,170}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
fb(PW2b, function(x,y) return smoothstep(fogTop(x)-16, fogTop(x)+26, y) end, 4.4)
fb(PW1b, function(x,y) return 0.85*bump(x,700,170)*smoothstep(fogTop(x)-4, fogTop(x)+30, y) * (1 - smoothstep(fogTop(x)+60, fogTop(x)+110, y)) end, 3.0)
print(wait(0))

--@ chunk 156
-- ===== clouds: bars of stratus, flatter and thinner toward the horizon =====
skyHole = rect(0,0,1000,420) - R1m
CBc = pile{{"lead white",5.6},{"smalt",1.1},{"cobalt blue",0.25},{"red earth",0.28},{"yellow ochre",0.08}}
CBw = pile{{"lead white",5.5},{"smalt",0.7},{"red earth",0.35},{"yellow ochre",0.3}}
CWu = pile{{"lead white",5},{"vermilion",0.5},{"yellow ochre",0.7},{"red earth",0.1}}
function cloud(x0, x1, yc, th, sag, pile_, tool_)
  local c = lens(x0, x1, yc, th, sag, 0.65) * skyHole
  work(c, {hand="body", tool=tool_ or "filbert 12", pile=pile_, angle=0.0, coverage=3.4, clip=true, fill=true, length={50,130}, angle_jitter=0.03})
  return c
end
function under(x0, x1, yc, th, sag, pile_)
  local c = lens(x0+14, x1-14, yc, th, sag, 0.7) * skyHole
  work(c, {hand="body", tool="filbert 6", pile=pile_, angle=0.0, coverage=2.6, clip=true, fill=true, length={40,100}, angle_jitter=0.02})
  return c
end
cl = {}
-- upper right, cool and softer
cl[#cl+1] = cloud(560, 1005, 178, 20, 4, CBc, "filbert 14")
cl[#cl+1] = cloud(420, 770, 214, 13, 3, CBc, "filbert 12")
-- middle
cl[#cl+1] = cloud(470, 960, 262, 11, 3, CBw, "filbert 12")
cl[#cl+1] = cloud(10, 380, 276, 9, 3, CBw, "filbert 10")
cl[#cl+1] = cloud(40, 300, 232, 6, 2, CBc, "filbert 8")
-- near the glow
cl[#cl+1] = cloud(420, 905, 316, 6, 2, CBw, "filbert 8")
cl[#cl+1] = cloud(0, 330, 328, 5, 2, CBw, "filbert 8")
cl[#cl+1] = cloud(560, 1000, 350, 3.4, 1.5, CBw, "filbert 6")
cu = {}
cu[#cu+1] = under(560, 1005, 178, 6, 4, CWu)
cu[#cu+1] = under(470, 960, 262, 4, 3, CWu)
cu[#cu+1] = under(10, 380, 276, 3.5, 3, CWu)
cu[#cu+1] = under(420, 905, 316, 2.6, 2, CWu)
cu[#cu+1] = under(0, 330, 328, 2.2, 2, CWu)
print(wait(0))

--@ chunk 157
for i, c in ipairs(cl) do
  local reg = c:grow(9) * skyHole
  blend(reg, {angle=0.0})
  blend(reg, {angle=0.06})
end
print(wait(0))

--@ chunk 158
zoneC = rect(0,140,1000,235) - R1m - rockM
blend(zoneC, {angle=1.5708})
blend(zoneC, {angle=1.5708})
blend(zoneC, {angle=0.0})
print(wait(0))

--@ chunk 159
print(wait(6*24*60))
print(drying(500,250), drying(700,180), drying(300,330), drying(850,345), drying(300,60))

--@ chunk 160
skyFull = mask(function(x,y) return (y <= Atop(x) + 0.5) and 1 or 0 end) - butteM
print("area", skyFull:area())
spr2(skyFull, T0, function(x,y) return clamp((1 - smoothstep(20,150,y)) * (1.1 - 0.3*bump(x,600,600)),0,1) end, 3.4)
spr2(skyFull, T1, function(x,y) return clamp(bump(y,95,75),0,1) end, 3.2)
spr2(skyFull, T2, function(x,y) return clamp(bump(y,175,60),0,1) end, 3.4)
spr2(skyFull, T3, function(x,y) return clamp(bump(y,238,42),0,1) end, 3.4)
spr2(skyFull, T4, function(x,y) return clamp(bump(y,282,36)*(0.55+0.45*bump(x,640,520)),0,1) end, 3.4)
blend(skyFull * rect(0,0,1000,330), {angle=0.0})
blend(skyFull * rect(0,0,1000,330), {angle=0.0})
print(wait(0))

--@ chunk 161
print(wait(5*24*60))
print(drying(500,250), drying(700,180), drying(300,330), drying(850,345), drying(300,60), drying(640,395))

--@ chunk 162
skyG3 = skyFull
GBt = pile{{"cobalt blue",2.0},{"smalt",2.4},{"Prussian blue",0.06},{"lead white",0.2}, medium=0.85}
work(skyG3, {hand="glaze", pile=GBt, angle=0.0, coverage=2.2, clip=true, angle_jitter=0.03,
  load_at=function(x,y) return clamp((1 - smoothstep(0,250,y))^1.1 * (0.9 + 0.1*(1-bump(x,620,500))),0,1) end})
print(wait(0))

--@ chunk 163
zT = skyG3 * rect(0,0,1000,262)
blend(zT, {angle=0.0})
blend(zT, {angle=0.0})
blend(zT, {angle=1.5708})
blend(zT, {angle=0.0})
print(wait(0))

--@ chunk 164
print(drying(100,100), drying(600,50))
VS = pile{{"lead white",2.0},{"pale smalt",1.6},{"cobalt blue",1.0}, medium=0.6}
tz = rect(20, 20, 260, 150) * skyG3
stipple(tz, {tool="stippler 8", pile=VS, coverage=2.2, clip=true, pressure={0.25,0.55}, feather=0.4, cluster=0.0, dips={30,0.8,0.3}})
print(wait(0))

--@ chunk 165
zTL = skyG3 * rect(0,0,420,262)
blend(zTL, {angle=0.0})
blend(zTL, {angle=0.03})
blend(zTL, {angle=0.0})
print(wait(0))

--@ chunk 166
lowSky = skyFull * rect(0, 262, 1000, 160)
T7b = pile{{"lead white",2.4},{"vermilion",1.1},{"chrome yellow",1.0},{"red earth",0.12}}
T6b = pile{{"lead white",3.4},{"chrome yellow",1.5},{"yellow ochre",0.35},{"vermilion",0.2}}
T9  = pile{{"lead white",6},{"chrome yellow",0.55},{"vermilion",0.06}}
T5b = pile{{"lead white",5.5},{"chrome yellow",0.85},{"yellow ochre",0.12},{"Prussian blue",0.03}}
function ls(p, f, cov)
  work(lowSky, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
ls(T5b, function(x,y) return smoothstep(280,330,y) end, 3.6)
ls(T6b, function(x,y) return (0.25 + 0.75*bump(x,640,380)) * smoothstep(322,392,y) end, 3.4)
ls(T7b, function(x,y) return 0.95*bump(x,650,330) * smoothstep(356,408,y)^1.15 end, 3.2)
ls(T9,  function(x,y) return bump(x,640,150) * smoothstep(366,410,y)^1.3 end, 3.0)
blend(lowSky, {angle=0.0})
blend(lowSky, {angle=0.02})
blend(lowSky, {angle=-0.02})
print(wait(0))

--@ chunk 167
print(drying(500,300), drying(500,270), drying(640,390))
-- soften the hard upper edge by vertical blending across it, then re-lay the transition
edgeZ = skyFull * rect(0, 215, 1000, 120)
blend(edgeZ, {angle=1.5708})
blend(edgeZ, {angle=1.5708})
blend(edgeZ, {angle=1.2})
print(wait(0))

--@ chunk 168
skyPart = skyFull * mask(function(x,y) return smoothstep(120, 215, y) end)
function sp(p, f, cov)
  work(skyPart, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
-- vertical position of the glow-dependent thresholds
sp(T2,  function(x,y) return bump(y,175,70) end, 3.4)
sp(T3,  function(x,y) return bump(y,236,46) end, 3.4)
sp(T4,  function(x,y) return bump(y,280,40)*(0.6+0.4*bump(x,640,600)) end, 3.4)
sp(T5b, function(x,y) return smoothstep(285,335,y) * (0.8+0.2*bump(x,640,500)) end, 3.4)
sp(T6b, function(x,y) return (0.2 + 0.8*bump(x,640,360)) * smoothstep(326,392,y) end, 3.2)
sp(T7b, function(x,y) return 0.9*bump(x,650,300) * smoothstep(358,408,y)^1.15 * (1 - 0.7*bump(x,640,110)*smoothstep(380,410,y)) end, 3.0)
sp(T9,  function(x,y) return bump(x,640,170) * smoothstep(352,410,y)^1.2 * 0.95 end, 3.0)
blend(skyPart, {angle=0.0})
blend(skyPart, {angle=0.02})
blend(skyPart, {angle=-0.02})
print(wait(0))

--@ chunk 169
seamZ = skyFull * rect(0, 95, 1000, 130)
blend(seamZ, {angle=1.5708})
blend(seamZ, {angle=1.5708})
blend(seamZ, {angle=0.0})
blend(seamZ, {angle=0.0})
print(wait(0))

--@ chunk 170
print(drying(500,30), drying(500,90), drying(500,110), drying(500,200), drying(100,60))

--@ chunk 171
print(wait(7*24*60))
print(drying(500,30), drying(500,90), drying(500,110), drying(500,200), drying(100,60), drying(640,395))

--@ chunk 172
-- ===== FINAL SKY, chunk A: blue to pale-green (whole sky region, one wet-in-wet pass) =====
T0 = pile{{"lead white",2.6},{"smalt",3.4},{"cobalt blue",1.7},{"Prussian blue",0.05}}
T1 = pile{{"lead white",3.6},{"smalt",2.8},{"cobalt blue",1.2}}
T2 = pile{{"lead white",5},{"pale smalt",2.2},{"cobalt blue",0.7}}
T3 = pile{{"lead white",6},{"pale smalt",1.2},{"cobalt blue",0.3},{"chrome yellow",0.2},{"Prussian blue",0.05}}
T4 = pile{{"lead white",6},{"pale smalt",0.6},{"Prussian blue",0.07},{"chrome yellow",0.55}}
spr2(skyFull, T0, function(x,y) return clamp((1 - smoothstep(20,150,y)) * (1.1 - 0.3*bump(x,600,600)),0,1) end, 3.4)
spr2(skyFull, T1, function(x,y) return clamp(bump(y,95,75),0,1) end, 3.2)
spr2(skyFull, T2, function(x,y) return clamp(bump(y,175,60),0,1) end, 3.4)
spr2(skyFull, T3, function(x,y) return clamp(bump(y,238,42),0,1) end, 3.4)
spr2(skyFull, T4, function(x,y) return clamp(bump(y,282,36)*(0.55+0.45*bump(x,640,520)),0,1) end, 3.4)
blend(skyFull, {angle=0.0})
blend(skyFull, {angle=0.0})
print(wait(0))

--@ chunk 173
-- ===== FINAL SKY, chunk B: lemon -> gold -> peach glow, wet in wet =====
T5b = pile{{"lead white",5.5},{"chrome yellow",0.8},{"yellow ochre",0.12},{"Prussian blue",0.03}}
T6c = pile{{"lead white",3.6},{"chrome yellow",1.4},{"yellow ochre",0.35},{"vermilion",0.16}}
T7c = pile{{"lead white",2.6},{"vermilion",1.0},{"chrome yellow",0.9},{"red earth",0.1}}
T9c = pile{{"lead white",6},{"chrome yellow",0.55},{"vermilion",0.06}}
sp2 = function(p, f, cov)
  work(skyFull, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
sp2(T5b, function(x,y) return smoothstep(292,340,y) * (0.8+0.2*bump(x,640,500)) end, 3.2)
sp2(T6c, function(x,y) return (0.15 + 0.85*bump(x,640,330)) * smoothstep(334,396,y) end, 3.2)
sp2(T7c, function(x,y) return 0.9*bump(x,650,280) * smoothstep(362,408,y)^1.2 end, 3.0)
sp2(T9c, function(x,y) return bump(x,640,150) * smoothstep(354,410,y)^1.2 * 0.95 end, 3.0)
blend(skyFull, {angle=0.0})
blend(skyFull, {angle=0.02})
blend(skyFull, {angle=-0.02})
print(wait(0))

--@ chunk 174
print(wait(5*24*60))
print(drying(500,30), drying(500,200), drying(500,300), drying(640,395), drying(100,60))

--@ chunk 175
-- ===== deepen the top blue: one thin glaze over the whole sky, then blend the whole sky =====
GBu = pile{{"cobalt blue",2.0},{"smalt",2.4},{"Prussian blue",0.05},{"lead white",0.2}, medium=0.85}
work(skyFull, {hand="glaze", pile=GBu, angle=0.0, coverage=2.0, clip=true, angle_jitter=0.03,
  load_at=function(x,y) return clamp((1 - smoothstep(0,215,y))^1.25 * 0.9,0,1) end})
blend(skyFull, {angle=0.0})
blend(skyFull, {angle=0.0})
blend(skyFull, {angle=1.5708})
blend(skyFull, {angle=0.0})
print(wait(0))

--@ chunk 176
VT = pile{{"lead white",3.4},{"smalt",2.6},{"cobalt blue",1.1}, medium=0.45}
topReg = skyFull * rect(0,0,1000,250)
work(topReg, {hand="body", tool="filbert 26", pile=VT, angle=0.0, coverage=2.6, clip=true, fill=true, length={120,260}, angle_jitter=0.02,
  load_at=function(x,y) return clamp((1 - smoothstep(10,225,y)) * 0.9,0,1) end})
blend(topReg, {angle=0.0})
blend(topReg, {angle=0.0})
blend(topReg, {angle=0.01})
print(wait(0))

--@ chunk 177
print(drying(500,100), drying(500,240), drying(500,300))
T2n = pile{{"lead white",4.8},{"pale smalt",2.0},{"smalt",0.7},{"cobalt blue",0.6}}
T3n = pile{{"lead white",6},{"pale smalt",1.2},{"cobalt blue",0.3},{"chrome yellow",0.22},{"Prussian blue",0.05}}
spr2(skyFull, T0, function(x,y) return clamp((1 - smoothstep(10,130,y)) * 0.9,0,1) end, 3.6)
spr2(skyFull, T1, function(x,y) return clamp(bump(y,85,70),0,1) end, 3.6)
spr2(skyFull, T2n, function(x,y) return clamp(bump(y,170,62),0,1) end, 3.8)
spr2(skyFull, T3n, function(x,y) return clamp(bump(y,240,46),0,1) end, 3.8)
spr2(skyFull, T4, function(x,y) return clamp(bump(y,284,38)*(0.55+0.45*bump(x,640,520)),0,1) end, 3.4)
blend(skyFull, {angle=0.0})
blend(skyFull, {angle=0.0})
blend(skyFull, {angle=0.02})
print(wait(0))

--@ chunk 178
T5c = pile{{"lead white",5.2},{"chrome yellow",0.9},{"yellow ochre",0.14},{"Prussian blue",0.03}}
sp2(T5c, function(x,y) return smoothstep(296,344,y) * (0.8+0.2*bump(x,640,500)) end, 3.4)
sp2(T6c, function(x,y) return (0.2 + 0.8*bump(x,640,340)) * smoothstep(336,398,y) end, 3.4)
sp2(T7c, function(x,y) return 0.95*bump(x,650,300) * smoothstep(360,408,y)^1.1 end, 3.2)
sp2(T9c, function(x,y) return bump(x,640,150) * smoothstep(354,410,y)^1.2 * 0.95 end, 3.0)
glowZone = skyFull * rect(0, 255, 1000, 160)
blend(skyFull, {angle=0.0})
blend(skyFull, {angle=0.02})
blend(skyFull, {angle=-0.02})
print(wait(0))

--@ chunk 179
print(wait(5*24*60))
print(drying(500,30), drying(500,200), drying(500,300), drying(640,395), drying(100,60))

--@ chunk 180
-- ===== the butte, rebuilt as an alpenglow-lit sandstone massif =====
buPts = {{746,360},{747,346},{753,338},{764,335},{777,333},{791,334},{805,332},{820,334},{834,331},{848,333},{860,332},{866,337},{868,347},
         {870,362},{872,380},{871,398},{874,416},{876,436},{880,456},{884,474},{892,492},
         {730,492},{734,470},{737,450},{739,430},{740,410},{743,390},{744,374}}
buM = poly(buPts):roughen(1.1, 11, 4)
function aret3(y) return 797 - 0.10*(y-335) end
buR = buM * mask(function(x,y) return smoothstep(aret3(y)-4, aret3(y)+4, x) end)
buL = buM * mask(function(x,y) return 1 - smoothstep(aret3(y)-4, aret3(y)+4, x) end)
BCool   = pile{{"smalt",1.8},{"raw umber",0.7},{"lead white",2.0},{"red earth",0.45},{"Prussian blue",0.08}}
BShadow = pile{{"smalt",1.6},{"raw umber",1.0},{"Prussian blue",0.2},{"lead white",0.9},{"red earth",0.3}}
BWarm   = pile{{"yellow ochre",0.9},{"red earth",0.8},{"lead white",2.2},{"vermilion",0.3},{"smalt",0.15}}
BWarmHi = pile{{"lead white",2.4},{"vermilion",0.6},{"yellow ochre",0.8},{"red earth",0.2}}
vert3 = function(x,y) return 1.5708 + 0.06*math.sin(x/10 + y/45) end
-- 0. cover the old tent shoulders with lilac fading to fog
tentCover = (shL + shR):grow(4) - buM:grow(1)
work(tentCover, {hand="body", tool="filbert 14", pile=PAc, angle=0.02, coverage=4.4, clip=true, fill=true, length={30,90}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(1 - smoothstep(Atop(x)+26, Atop(x)+70, y), 0, 1) end})
work(tentCover, {hand="body", tool="filbert 14", pile=PW2b, angle=0.0, coverage=4.4, clip=true, fill=true, length={40,110}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(Atop(x)+22, Atop(x)+66, y), 0, 1) end})
-- 1. cool body
work(buM, {hand="body", tool="filbert 10", pile=BCool, angle=vert3, coverage=5.2, clip=true, fill=true, length={30,80}, angle_jitter=0.05})
-- 2. shadow face
work(buR, {hand="body", tool="filbert 10", pile=BShadow, angle=vert3, coverage=4.4, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.55 + 0.45*smoothstep(aret3(y), 870, x) * smoothstep(336, 400, y)^0.6, 0, 1) end})
-- 3. alpenglow on the glow-facing face, fading downward
work(buL, {hand="body", tool="filbert 9", pile=BWarm, angle=vert3, coverage=4.4, clip=true, fill=true, length={26,70}, angle_jitter=0.06,
  load_at=function(x,y) return clamp((1 - smoothstep(338, 446, y)) * (0.55 + 0.45*(1 - smoothstep(744, aret3(y), x))), 0, 1) end})
work(buL, {hand="body", tool="filbert 7", pile=BWarmHi, angle=vert3, coverage=2.6, clip=true, length={20,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(338, 392, y)) * (1 - smoothstep(744, aret3(y), x))^0.8 * 0.85, 0, 1) end})
work(buR, {hand="body", tool="filbert 7", pile=BWarmHi, angle=vert3, coverage=1.8, clip=true, length={20,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(336, 368, y)) * 0.32, 0, 1) end})
blend(buM, {angle=1.5708})
print(wait(0))

--@ chunk 181
-- (a) remove the dark sliver left of the old right wall: ridge colour over a thin strip beside the new silhouette
sliver = rect(866, 380, 30, 70) - buM:grow(0.5)
work(sliver, {hand="body", tool="filbert 7", pile=PAc, angle=0.02, coverage=4.5, clip=true, fill=true, length={10,30}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(1 - smoothstep(Atop(x)+30, Atop(x)+60, y), 0, 1) * smoothstep(Atop(x)-1, Atop(x)+3, y) end})
work(sliver, {hand="body", tool="filbert 7", pile=PW2b, angle=0.0, coverage=4.5, clip=true, fill=true, length={10,30}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(Atop(x)+26, Atop(x)+56, y), 0, 1) end})
-- (b) deepen the body: darker toward base and right
BDeep = pile{{"raw umber",1.2},{"Prussian blue",0.3},{"smalt",1.0},{"lead white",0.5},{"red earth",0.25}}
work(buR, {hand="body", tool="filbert 10", pile=BDeep, angle=vert3, coverage=3.2, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.35 + 0.65*smoothstep(336, 440, y), 0, 1) * (0.55 + 0.45*smoothstep(aret3(y), 868, x)) end})
work(buL, {hand="body", tool="filbert 10", pile=BDeep, angle=vert3, coverage=2.6, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(372, 466, y)*0.9, 0, 1) * (0.4 + 0.6*smoothstep(744, aret3(y), x)) end})
-- (c) fissures and strata
function fissure(x0, y0, y1, wob, w, seed)
  local pts = {}
  local n = math.max(3, math.floor((y1-y0)/12))
  for i = 0, n do
    local y = y0 + (y1-y0)*i/n
    pts[#pts+1] = {x0 + wob*math.sin(i*1.7 + seed) + rand(-0.8,0.8), y}
  end
  return ribbon(pts, w)
end
fis = fissure(775, 340, 452, 1.6, 1.3, 1) + fissure(758, 352, 420, 1.2, 1.0, 2) + fissure(818, 338, 436, 1.8, 1.4, 3)
    + fissure(846, 336, 410, 1.2, 1.1, 4) + fissure(832, 380, 460, 1.4, 0.9, 5) + fissure(802, 400, 466, 1.4, 1.0, 6)
FisD = pile{{"raw umber",1.4},{"Prussian blue",0.3},{"smalt",0.6},{"bone black",0.4}}
work(fis * buM, {hand="detail", pile=FisD, coverage=1.3, clip=true, angle=1.5708, length={14,40}, pressure={0.2,0.45}, load=0.7,
  load_at=function(x,y) return clamp(0.5 + 0.5*smoothstep(340, 400, y), 0, 1) end})
strata = (ledge(372, 746, 868, 1.4, 8) + ledge(399, 744, 870, 1.6, 10) + ledge(425, 740, 874, 1.8, 7) + ledge(449, 736, 878, 1.8, 9)) * buM
work(strata, {hand="detail", pile=FisD, coverage=1.1, clip=true, angle=0.02, length={6,20}, pressure={0.2,0.4}, load=0.6})
print(wait(0))

--@ chunk 182
blend(buM, {angle=1.5708})
blend(buM, {angle=1.5708})
blend(buM, {angle=1.45})
-- re-lay the alpenglow on the upper left face and a faint rose along the right face top, soft
work(buL, {hand="body", tool="filbert 9", pile=BWarm, angle=vert3, coverage=3.4, clip=true, fill=true, length={26,70}, angle_jitter=0.06,
  load_at=function(x,y) return clamp((1 - smoothstep(338, 420, y)) * (0.6 + 0.4*(1 - smoothstep(744, aret3(y), x))), 0, 1) end})
work(buL, {hand="body", tool="filbert 7", pile=BWarmHi, angle=vert3, coverage=2.4, clip=true, length={20,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(338, 380, y)) * (1 - smoothstep(744, aret3(y), x))^0.8 * 0.85, 0, 1) end})
work(buR, {hand="body", tool="filbert 7", pile=BWarm, angle=vert3, coverage=2.0, clip=true, length={20,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(336, 372, y)) * 0.4, 0, 1) end})
blend(buM, {angle=1.5708})
print(wait(0))

--@ chunk 183
zoneR = mask(function(x,y)
  if y < Atop(x) then return 0 end
  if y > crestY(x) - 1.5 then return 0 end
  return smoothstep(560, 690, x)
end) - buM:grow(1.2)
function zr(p, f, cov, tool)
  work(zoneR, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={70,200}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
-- fog body
zr(PW2b, function(x,y) return smoothstep(Atop(x)+24, Atop(x)+62, y) end, 4.6)
-- ridge A band: pale lilac (warmer toward the glow), crisp top
zr(PAc, function(x,y) return 0.97*(1 - smoothstep(Atop(x)+14, Atop(x)+62, y)) end, 4.0)
zr(PAw, function(x,y) return 0.9*bump(x,660,230)*(1 - smoothstep(Atop(x)+10, Atop(x)+52, y)) end, 3.0)
-- warm cream near the glow and cool lilac lower right
zr(PW1b, function(x,y) return 0.85*bump(x,680,200)*smoothstep(Atop(x)+50, Atop(x)+90, y)*(1 - smoothstep(470,520,y))*fq(x+30,y) end, 2.6)
zr(PW3b, function(x,y) return smoothstep(500,640,y)*0.8*smoothstep(700,900,x) end, 3.0)
print(wait(0))

--@ chunk 184
-- far lilac hump to the right of the butte
A2Rn_top = {{860,462},{890,452},{925,444},{960,438},{1010,436}}
mA2Rn = topMask(A2Rn_top, 1.2, 856, 1010) * (-fgM) - buM:grow(1)
work(mA2Rn, {hand="body", tool="filbert 12", pile=PA6, angle=0.03, coverage=4.0, clip=true, fill=true, length={40,100}, angle_jitter=0.06,
  load_at=function(x,y) local t = interp(A2Rn_top, x); return clamp(0.95*(1 - smoothstep(t+1, t+42, y)) * smoothstep(856, 905, x), 0, 1)^0.85 end})
-- hill B (right half, overlapping the seam at x 535-575), D, knolls E and F
mB3r = topMask(B3_top, 2.6, 535, 835) * (-fgM)
woodHill2(mB3r, PB3, B3_top, 505, 590, function(x) return smoothstep(535,585,x) * (1 - smoothstep(780,835,x)) end)
E_top = {{675,514},{698,498},{720,488},{742,481},{764,480},{786,486},{806,496},{828,512}}
F_top = {{826,522},{852,504},{876,491},{902,484},{928,486},{952,494},{976,505},{1000,516}}
PKn = pile{{"lead white",2.7},{"smalt",1.9},{"raw umber",0.4},{"red earth",0.3},{"Prussian blue",0.06}}
mE = topMask(E_top, 2.2, 668, 835) * (-fgM)
mF = topMask(F_top, 2.2, 820, 1010) * (-fgM)
woodHill2(mE, PKn, E_top, 492, 545, function(x) return smoothstep(668,705,x) * (1 - smoothstep(800,835,x)) end)
woodHill2(mF, PKn, F_top, 496, 552, function(x) return smoothstep(820,860,x) end)
woodHill2(mD3, PD3, D3_top, 515, 580, function(x) return smoothstep(836,880,x) end)
print(wait(0))

--@ chunk 185
-- fog veil wrapping the butte base (soft, partial)
fogBase = mask(function(x,y) return sb(x, 672, 740, 905, 960) * bump(y, 488, 20) end) - fgM
work(fogBase, {hand="body", tool="filbert 14", pile=PW2b, angle=0.0, coverage=3.4, clip=true, fill=true, length={40,120}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.95*bump(y,488,17)*(0.75+0.25*fq(x+20,y)), 0, 1) end})
work(fogBase, {hand="body", tool="filbert 14", pile=PW1b, angle=0.0, coverage=2.2, clip=true, fill=true, length={40,120}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.7*bump(y,496,10)*bump(x,760,120)*fq(x+70,y), 0, 1) end})
-- small conifers and a few round-headed trees along the plateau edge
pineD = pile{{"raw umber",1.2},{"Prussian blue",0.45},{"bone black",0.6},{"green earth",0.25}}
function tinyPine(x, ybase, h, w)
  local pts = {{x, ybase-h}}
  local n = math.max(3, math.floor(h/3.2))
  for i = 1, n do
    local t = i/n
    local yy = ybase - h + h*t
    local hw = w*0.5*t
    pts[#pts+1] = {x + hw*rand(0.55,1.0), yy - h/n*0.35}
    pts[#pts+1] = {x + hw*rand(0.2,0.45), yy}
  end
  pts[#pts+1] = {x + 0.8, ybase+1.5}
  pts[#pts+1] = {x - 0.8, ybase+1.5}
  for i = n, 1, -1 do
    local t = i/n
    local yy = ybase - h + h*t
    local hw = w*0.5*t
    pts[#pts+1] = {x - hw*rand(0.2,0.45), yy}
    pts[#pts+1] = {x - hw*rand(0.55,1.0), yy - h/n*0.35}
  end
  return poly(pts)
end
local xs = {754,762,771,779,790,799,809,818,828,838,846,855,862}
local topy = function(x) return interp({{746,346},{753,338},{764,335},{777,333},{791,334},{805,332},{820,334},{834,331},{848,333},{860,332},{866,337}}, x) end
for i, x in ipairs(xs) do
  local h = rand(6, 15) * ((i % 4 == 0) and 1.3 or 1)
  local m = tinyPine(x, topy(x)+2, h, h*0.45)
  work(m, {hand="detail", pile=pineD, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
end
-- rounded oak/beech crowns between them
for _, x in ipairs({758, 785, 812, 842}) do
  local m = ellipse(x, topy(x)-1.2, rand(3.5,5.5), rand(2.5,3.8))
  work(m, {hand="detail", pile=pineD, coverage=2.6, clip=true, length={3,8}, angle=0.0})
end
print(wait(0))

--@ chunk 186
topPts = {{746,346},{753,338},{764,335},{777,333},{791,334},{805,332},{820,334},{834,331},{848,333},{860,332},{866,337}}
function topy(x) return interp(topPts, x) end
function tl_top(x)
  local hmul = smoothstep(748, 764, x) * (1 - smoothstep(852, 868, x))
  return topy(x) + 3 - hmul * (7.5 + 4.0*trn(x*1.7, 11))
end
forestBand = mask(function(x,y)
  if x < 746 or x > 870 then return 0 end
  local t = tl_top(x)
  return smoothstep(t-0.5, t+0.5, y) * ((y < topy(x) + 5) and 1 or 0)
end)
work(forestBand, {hand="body", tool="filbert 5", pile=pineD, angle=1.5708, coverage=5.0, clip=true, fill=true, length={4,10}, angle_jitter=0.35})
for _, p in ipairs({{757,13},{783,11},{795,16},{803,9},{831,14},{841,10},{849,17},{860,9}}) do
  local m = tinyPine(p[1] + rand(-1.5,1.5), topy(p[1])+1, p[2], p[2]*0.4)
  work(m, {hand="detail", pile=pineD, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 187
-- ===== butte: needle, tower and notch, drawn by hand over the sky =====
needle = poly({{752,340},{754,322},{756,306},{758,294},{760,288},{762,296},{764,310},{765,326},{767,340}})
tower  = poly({{822,334},{823,322},{825,314},{830,310},{836,311},{840,309},{845,312},{846,322},{848,334}})
step   = poly({{860,334},{862,328},{868,326},{874,330},{878,342},{880,356},{868,356}})
extra  = (needle + tower + step):roughen(0.9, 9, 6)
work(extra, {hand="body", tool="filbert 6", pile=BCool, angle=1.5708, coverage=5.0, clip=true, fill=true, length={6,24}, angle_jitter=0.05})
work(extra * mask(function(x,y) return 1 - smoothstep(aret3(y)-3, aret3(y)+3, x) end),
  {hand="body", tool="filbert 5", pile=BWarm, angle=1.5708, coverage=3.2, clip=true, length={6,20}, angle_jitter=0.06})
work(extra * mask(function(x,y) return smoothstep(aret3(y)-3, aret3(y)+3, x) end),
  {hand="body", tool="filbert 5", pile=BShadow, angle=1.5708, coverage=3.2, clip=true, length={6,20}, angle_jitter=0.06})
-- trees crowning the tower and the step
for _, p in ipairs({{826,312,9},{833,310,12},{840,309,8},{864,327,7},{871,328,10}}) do
  local m = tinyPine(p[1], p[2]+1, p[3], p[3]*0.42)
  work(m, {hand="detail", pile=pineD, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
end
-- ragged lower edge of the forest band: rock colour stippled up into it, and a few trunk-streaks running down the cliff
ragged = mask(function(x,y) return (x>=748 and x<=868) and bump(y, topy(x)+3, 4.5) or 0 end) * buM
stipple(ragged, {pile=BCool, width=2.6, coverage=1.6, clip=true, pressure={0.3,0.6}, feather=0.5})
streaks = {}
for i = 1, 11 do
  local x = rand(752, 862)
  streaks[#streaks+1] = ribbon({{x, topy(x)+2}, {x+rand(-1,1), topy(x)+rand(7,16)}}, rand(0.8,1.6))
end
local sm = streaks[1]; for i = 2, #streaks do sm = sm + streaks[i] end
work(sm * buM, {hand="detail", pile=pineD, coverage=1.2, clip=true, length={4,10}, angle=1.5708, pressure={0.2,0.4}, load=0.6})
print(wait(0))

--@ chunk 188
towerSt = (tower + step):roughen(0.9, 9, 6)
work(towerSt, {hand="body", tool="filbert 5", pile=BShadow, angle=1.5708, coverage=5.5, clip=true, fill=true, length={6,22}, angle_jitter=0.06})
work(towerSt, {hand="body", tool="filbert 5", pile=BDeep, angle=1.5708, coverage=3.0, clip=true, length={6,22}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.2 + 0.8*smoothstep(822, 848, x), 0, 1) end})
-- warm light kissing the left edges of the tower and step
edgeL = (tower:rim(4, 3) + step:rim(4,3)) * mask(function(x,y) return (x < 833 or (x>858 and x<868)) and 1 or 0 end)
work(edgeL, {hand="body", tool="filbert 4", pile=BWarm, angle=1.5708, coverage=2.4, clip=towerSt, length={4,14}, load=0.7})
-- re-lay the forest crowns after the darkening
for _, p in ipairs({{826,312,9},{833,310,12},{840,309,8},{864,327,7},{871,328,10}}) do
  local m = tinyPine(p[1], p[2]+1, p[3], p[3]*0.42)
  work(m, {hand="detail", pile=pineD, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 189
-- ===== foreground crest: variegated, dark, with a lit rim =====
fgn1 = noise{seed=41, period=70, octaves=3, persistence=0.55}
fgn2 = noise{seed=42, period=210, octaves=2, persistence=0.5}
Fol  = pile{{"raw umber",3},{"green earth",1.1},{"Prussian blue",0.5},{"bone black",0.4},{"yellow ochre",0.25}}
Fwm  = pile{{"raw umber",3},{"red earth",1.6},{"yellow ochre",0.6},{"Prussian blue",0.2}}
Fcool= pile{{"raw umber",2.5},{"Prussian blue",0.8},{"bone black",0.8},{"smalt",0.4}}
fgReg = fgM
function dist_below_crest(x,y) return y - crestY(x) end
-- olive patches
work(fgReg, {hand="body", tool="filbert 18", pile=Fol, angle=crestAng, coverage=2.6, clip=true, length={30,90}, angle_jitter=0.15,
  load_at=function(x,y) local d = dist_below_crest(x,y); return clamp((0.5 + 0.9*fgn1(x,y)) * smoothstep(-2, 40, d) * (1 - smoothstep(110, 200, d)), 0, 1) end})
-- warm dry-grass patches nearer the crest and on the right slope facing the glow
work(fgReg, {hand="body", tool="filbert 14", pile=Fwm, angle=crestAng, coverage=2.4, clip=true, length={25,70}, angle_jitter=0.2,
  load_at=function(x,y) local d = dist_below_crest(x,y); return clamp((0.35 + 0.9*fgn2(x+90,y)) * (1 - smoothstep(6, 70, d)) * smoothstep(-3, 8, d) * (0.4 + 0.6*smoothstep(250,700,x)), 0, 1) end})
-- cool deep shade low in the picture
work(fgReg, {hand="body", tool="filbert 18", pile=Fcool, angle=crestAng, coverage=2.8, clip=true, length={30,90}, angle_jitter=0.15,
  load_at=function(x,y) local d = dist_below_crest(x,y); return clamp(smoothstep(60, 170, d) * (0.6 + 0.6*fgn1(x+30,y)), 0, 1) end})
print(wait(0))

--@ chunk 190
patches = (rect(60,500,170,90) + rect(380,525,110,50) + rect(270,525,90,40)) * fgM
Fmix = pile{{"raw umber",3},{"red earth",1.0},{"green earth",0.5},{"Prussian blue",0.4},{"bone black",0.3}}
work(patches, {hand="body", tool="filbert 12", pile=Fmix, angle=crestAng, coverage=5.0, clip=true, fill=true, length={20,60}, angle_jitter=0.18, load=1.0})
-- warm rim again, thin, soft
work(patches, {hand="body", tool="filbert 10", pile=Fwm, angle=crestAng, coverage=2.2, clip=true, length={20,50}, angle_jitter=0.2,
  load_at=function(x,y) local d = dist_below_crest(x,y); return clamp((1 - smoothstep(2, 26, d)) * smoothstep(-3, 5, d) * 0.8, 0, 1) end})
print(wait(0))

--@ chunk 191
work(fgM, {hand="body", tool="filbert 20", pile=Fmix, angle=crestAng, coverage=4.2, clip=true, fill=true, length={30,100}, angle_jitter=0.15, load=1.0})
work(fgM, {hand="body", tool="filbert 18", pile=Fol, angle=crestAng, coverage=2.6, clip=true, length={30,90}, angle_jitter=0.15,
  load_at=function(x,y) local d = dist_below_crest(x,y); return clamp((0.45 + 0.9*fgn1(x,y)) * smoothstep(-2, 40, d) * (1 - smoothstep(120, 210, d)), 0, 1) end})
work(fgM, {hand="body", tool="filbert 14", pile=Fwm, angle=crestAng, coverage=2.4, clip=true, length={25,70}, angle_jitter=0.2,
  load_at=function(x,y) local d = dist_below_crest(x,y); return clamp((0.3 + 0.9*fgn2(x+90,y)) * (1 - smoothstep(4, 60, d)) * smoothstep(-3, 6, d) * (0.45 + 0.55*smoothstep(200,700,x)), 0, 1) end})
work(fgM, {hand="body", tool="filbert 18", pile=Fcool, angle=crestAng, coverage=2.8, clip=true, length={30,90}, angle_jitter=0.15,
  load_at=function(x,y) local d = dist_below_crest(x,y); return clamp(smoothstep(60, 170, d) * (0.6 + 0.6*fgn1(x+30,y)), 0, 1) end})
print(wait(0))

--@ chunk 192
cn1 = noise{seed=51, period=90, octaves=3, persistence=0.55}
cn2 = noise{seed=52, period=40, octaves=2, persistence=0.5}
function barMask(x0, x1, yc, th, drift, seed, softf)
  local n = noise{seed=seed, period=110, octaves=3, persistence=0.55}
  local n2 = noise{seed=seed+7, period=34, octaves=2, persistence=0.5}
  local xm, hw = (x0+x1)/2, (x1-x0)/2
  return mask(function(x,y)
    local t = (x - xm)/hw
    if math.abs(t) >= 1 then return 0 end
    local env = (1 - t*t)^0.7
    local thick = th * env * (0.5 + 0.5*(0.6 + 0.8*n:at01(x, 7)))
    local c = yc + drift*t + 3.0*n(x, 31)
    local d = y - c
    local top = -thick*0.55*(0.8 + 0.4*n2(x, 3))
    local bot = thick*0.45
    if d < top - 3 or d > bot + 2 then return 0 end
    return smoothstep(top-3*(softf or 1), top+1.5, d) * (1 - smoothstep(bot-1.2, bot+2, d))
  end)
end
skyOnlyM = mask(function(x,y) return (y <= Atop(x) - 1) and 1 or 0 end) - buM:grow(2)
CBt = pile{{"lead white",5.6},{"smalt",1.2},{"red earth",0.35},{"raw umber",0.1}, medium=0.3}
CBw2 = pile{{"lead white",5.2},{"vermilion",0.45},{"yellow ochre",0.7},{"red earth",0.12}, medium=0.3}
b1 = barMask(330, 1010, 318, 12, -6, 61) * skyOnlyM
work(b1, {hand="body", tool="filbert 8", pile=CBt, angle=0.0, coverage=2.6, clip=true, length={40,120}, angle_jitter=0.03, load=0.8})
print(wait(0))

--@ chunk 193
print(drying(600,318), drying(760,318))
local reg = b1:grow(7)
blend(reg, {angle=0.0})
blend(reg, {angle=0.03})
blend(reg, {angle=-0.03})
print(wait(0))

--@ chunk 194
print(wait(6*24*60))
print(drying(600,318), drying(760,318), drying(800,330), drying(300,60), drying(300,560))

--@ chunk 195
-- repaint a band of sky across the old cloud bar: wet-in-wet, same recipe as the sky; butte excluded (needle/tower area excluded too)
butteTop = (buM + needle + tower + step):grow(1.2)
barBand = rect(300, 268, 710, 100) * skyOnlyM - butteTop
bandF = function(x) return smoothstep(300, 360, x) end
function bb(p, f, cov)
  work(barBand, {hand="body", tool="filbert 14", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={40,120}, angle_jitter=0.03,
    load_at=function(x,y) return clamp(f(x,y) * bandF(x),0,1) end})
end
bb(T4,  function(x,y) return bump(y,284,36)*(0.55+0.45*bump(x,640,520)) end, 3.6)
bb(T5c, function(x,y) return smoothstep(296,344,y) * (0.8+0.2*bump(x,640,500)) * (1 - smoothstep(330,372,y)*0.0) end, 3.6)
bb(T6c, function(x,y) return (0.2 + 0.8*bump(x,640,340)) * smoothstep(336,398,y) end, 3.4)
bb(T7c, function(x,y) return 0.95*bump(x,650,300) * smoothstep(360,408,y)^1.1 end, 3.2)
print(wait(0))

--@ chunk 196
blend(barBand, {angle=0.0})
blend(barBand, {angle=0.02})
blend(barBand, {angle=-0.02})
print(wait(0))

--@ chunk 197
crownM = (needle + tower + step):roughen(0.9, 9, 6)
work(crownM, {hand="body", tool="filbert 6", pile=BCool, angle=1.5708, coverage=5.5, clip=true, fill=true, length={6,24}, angle_jitter=0.05})
work(crownM * mask(function(x,y) return smoothstep(aret3(y)-3, aret3(y)+3, x) end),
  {hand="body", tool="filbert 5", pile=BShadow, angle=1.5708, coverage=4.0, clip=true, length={6,20}, angle_jitter=0.06})
work(crownM * mask(function(x,y) return 1 - smoothstep(aret3(y)-3, aret3(y)+3, x) end),
  {hand="body", tool="filbert 5", pile=BWarm, angle=1.5708, coverage=3.4, clip=true, length={6,20}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.5 + 0.5*(1 - smoothstep(290, 345, y)), 0, 1) end})
-- the tower and step darker on their right, as the main face
work(towerSt, {hand="body", tool="filbert 5", pile=BDeep, angle=1.5708, coverage=3.2, clip=true, length={6,22}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.15 + 0.85*smoothstep(826, 848, x), 0, 1) end})
-- forest band again, then trees along it
work(forestBand, {hand="body", tool="filbert 5", pile=pineD, angle=1.5708, coverage=5.0, clip=true, fill=true, length={4,10}, angle_jitter=0.35})
for _, p in ipairs({{757,13},{783,11},{795,16},{803,9},{831,14},{841,10},{849,17},{860,9},{826,312,9},{833,310,12},{840,309,8},{864,327,7},{871,328,10}}) do
  local xx, yb, hh = p[1], nil, nil
  if #p == 3 then yb = p[2]+1; hh = p[3] else yb = topy(p[1]) + 1; hh = p[2] end
  local m = tinyPine(xx + rand(-1,1), yb, hh, hh*0.42)
  work(m, {hand="detail", pile=pineD, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 198
-- ===== THE OAK, last: trunk and main limbs over the final dry sky =====
OAK.L2 = OAKR.L2re
OAK.R1 = OAKR.R1; OAK.R1a = OAKR.R1a; OAK.R1b = OAKR.R1b; OAK.R1a2 = OAKR.R1a2
OAK.R1b1 = OAKR.R1b1; OAK.R1b2 = OAKR.R1b2; OAK.R1b3 = OAKR.R1b3; OAK.L2r1 = OAKR.L2r1; OAK.L2r2 = OAKR.L2r2
paintlimb("trunk", "broken", "filbert 12", 4.5)
paintlimb("L2", "broken", "filbert 10", 3.8)
paintlimb("L1", "broken", "filbert 10", 3.8)
paintlimb("R1", "broken", "filbert 12", 4.0)
paintlimb("L4", "broken", "filbert 10", 3.6)
paintlimb("L5", "broken", "filbert 10", 3.6)
paintlimb("R1a", "broken", "filbert 9", 3.6)
paintlimb("R1b", "broken", "filbert 9", 3.6)
print(wait(0))

--@ chunk 199
print(drying(244,560), drying(244,500), drying(150,560))
-- cover the bulb with ground paint: the foreground mix, following the crest slope
bulbCover = fgM * rect(150,524,200,90)
work(bulbCover, {hand="body", tool="filbert 14", pile=Fmix, angle=crestAng, coverage=5.5, clip=true, fill=true, length={20,70}, angle_jitter=0.18, load=1.0})
work(bulbCover, {hand="body", tool="filbert 10", pile=Fwm, angle=crestAng, coverage=2.2, clip=true, length={16,50}, angle_jitter=0.2,
  load_at=function(x,y) local d = y - crestY(x); return clamp((1 - smoothstep(2, 30, d)) * 0.7, 0, 1) end})
print(wait(0))

--@ chunk 200
-- root flares and spreading roots, hand-drawn
flareL = poly({{224,462},{219,484},{212,506},{200,522},{182,534},{160,544},{150,552},{196,552},{240,548},{240,462}}, true):roughen(1.2, 12, 3)
flareR = poly({{266,462},{272,484},{281,506},{296,522},{316,534},{340,543},{356,550},{300,552},{256,548},{256,462}}, true):roughen(1.2, 12, 4)
rootMid = body_of{spine={{244,520},{246,540},{252,556}}, widths={30,22,10}, blend=0.8, char="firm", amount=0.8}:mask()
rootL2 = body_of{spine={{214,512},{198,534},{176,552},{150,566}}, widths={16,12,7,2.5}, blend=0.8, char="firm", amount=1.0}:mask()
rootR2 = body_of{spine={{278,512},{296,534},{322,550},{352,562}}, widths={16,12,7,2.5}, blend=0.8, char="firm", amount=1.0}:mask()
rootsAll = flareL + flareR + rootMid + rootL2 + rootR2
work(rootsAll, {hand="body", tool="filbert 10", pile=OakD, angle=1.4, coverage=4.5, clip=true, fill=true, length={15,45}, angle_jitter=0.3, load=1.0})
print(wait(0))

--@ chunk 201
for _, nm in ipairs({"stub","S2","S4","S6","S7","S8","S9","S14","S15","R1a2","R1b1","R1b2","R1b3","L2r1","L2r2"}) do
  paintlimb(nm, "firm", "filbert 7", 3.6)
end
print(wait(0))

--@ chunk 202
sprout2("S8", 0.2, 1.0, 16, 2.6, 2, 24, 50, {p=0.6})
sprout2("S9", 0.2, 1.0, 16, 2.6, 2, 24, 50, {p=0.6})
sprout2("S7", 0.2, 1.0, 16, 2.8, 2, 28, 56, {p=0.6})
print(wait(0))

--@ chunk 203
for _, nm in ipairs({"S2","S4","S6","stub","S14","S15","R1a2","R1b1","R1b2","R1b3","L2r1","L2r2"}) do
  sprout2(nm, 0.2, 1.0, 15, 2.8, 2, 26, 54, {p=0.6})
end
sprout2("R1a", 0.15, 1.0, 17, 3.4, 2, 34, 70, {p=0.6})
sprout2("R1b", 0.15, 1.0, 17, 3.4, 2, 34, 70, {p=0.6})
sprout2("R1", 0.55, 0.95, 24, 3.4, 2, 38, 72, {p=0.55})
sprout2("L2", 0.5, 1.0, 17, 3.2, 2, 34, 66, {p=0.6})
sprout2("L1", 0.5, 1.0, 17, 3.2, 2, 34, 66, {p=0.6})
sprout2("L4", 0.35, 1.0, 17, 3.2, 2, 30, 62, {p=0.6})
sprout2("L5", 0.3, 1.0, 17, 3.2, 2, 30, 62, {p=0.6})
print(wait(0))

--@ chunk 204
function oakshoot2(x, y, ang, len, w0, depth, opts)
  opts = opts or {}
  local pts = {{x, y}}
  local angs = {}
  local cx, cy, a = x, y, ang
  local used = 0
  local kink = opts.kink or 0.5
  while used < len do
    local l = math.min(rand(opts.lmin or 8, opts.lmax or 16), len - used)
    if l < 3.0 then break end
    a = a + rand(-kink, kink)
    cx = cx + math.cos(a) * l
    cy = cy + math.sin(a) * l
    used = used + l
    pts[#pts+1] = {cx, cy}
    angs[#angs+1] = a
  end
  if #pts < 2 then return end
  local w1 = math.max(0.87, w0 * 0.3)
  tw:load(OakT, 1.0)
  tw:stroke(pts, {pressure={tw:pressure_for(math.max(w0, 0.9)), tw:pressure_for(w1)}, ramps={0.0, 0.0}})
  if depth > 0 then
    local total = used
    local d = 0
    for i = 2, #pts do
      d = d + math.sqrt((pts[i][1]-pts[i-1][1])^2 + (pts[i][2]-pts[i-1][2])^2)
      if math.random() < (opts.p or 0.6) then
        local side = (math.random() < 0.5) and 1 or -1
        local sa = angs[i-1] + side * rand(0.4, 0.95)
        local rem = total - d
        local sl = math.max(7, (rem + rand(5, 16)) * rand(0.45, 0.85))
        oakshoot2(pts[i][1], pts[i][2], sa, sl, math.max(0.9, w0 * 0.75), depth - 1, opts)
      end
    end
  end
end
function sprout3(name, from, to, spacing, wcap, depth, lenmin, lenmax, opts)
  local L = OAK[name]
  local pts, wd = L[1], L[2]
  local s = {0}
  for i = 2, #pts do
    s[i] = s[i-1] + math.sqrt((pts[i][1]-pts[i-1][1])^2 + (pts[i][2]-pts[i-1][2])^2)
  end
  local total = s[#pts]
  local pos = total * from
  local flip = (math.random() < 0.5) and 1 or -1
  while pos < total * to do
    local i = 1
    while i < #pts-1 and s[i+1] < pos do i = i + 1 end
    local t = (pos - s[i]) / (s[i+1] - s[i])
    local px = pts[i][1] + t * (pts[i+1][1]-pts[i][1])
    local py = pts[i][2] + t * (pts[i+1][2]-pts[i][2])
    local la = math.atan(pts[i+1][2]-pts[i][2], pts[i+1][1]-pts[i][1])
    local wl = wd[i] + t * (wd[i+1]-wd[i])
    local sa = la + flip * rand(0.35, 0.85)
    flip = -flip
    oakshoot2(px, py, sa, rand(lenmin, lenmax), clamp(wl * 0.42, 0.9, wcap), depth, opts)
    pos = pos + spacing * rand(0.7, 1.3)
  end
end
for _, nm in ipairs({"S2","S4","S6","S7","S8","S9","S14","S15","R1a2","R1b1","R1b2","R1b3","L2r1","L2r2","R1a","R1b"}) do
  sprout3(nm, 0.45, 1.0, 11, 2.0, 2, 20, 44, {p=0.55})
end
print(wait(0))

--@ chunk 205
figD   = pile{{"raw umber",2.4},{"Prussian blue",0.9},{"bone black",1.2},{"green earth",0.4}}
figHair= pile{{"raw umber",2},{"red earth",0.8},{"bone black",0.6},{"yellow ochre",0.2}}
figLit = pile{{"lead white",1.5},{"yellow ochre",1.0},{"vermilion",0.35},{"red earth",0.2}}
fx0, fy0 = 421, 545.5
function FP(pts, sc)
  local out = {}
  for _, p in ipairs(pts) do out[#out+1] = {fx0 + p[1]*(sc or 1), fy0 - p[2]*(sc or 1)} end
  return out
end
bodyPts = {{-4.8,0},{-4.7,6},{-5.1,13},{-5.6,19},{-8.2,21},{-8.9,28},{-8.7,36},{-8.8,44},{-8.0,49.5},{-5.2,52},{-2.8,53.2},
           {-3.4,55.5},{-3.9,58},{-3.1,61},{-1.0,62.3},{1.4,62.1},{3.2,60.6},{3.8,58},{3.3,55.5},{2.7,53.2},
           {5.2,52},{8.0,49.8},{9.0,46},{9.8,40},{10.0,33.5},{9.3,27.5},{8.7,21.5},{5.8,19.5},{5.1,13},{4.9,6},{5.0,0},
           {0.7,0},{0.5,12},{-0.5,12},{-0.7,0}}
figM = poly(FP(bodyPts), false)
work(figM, {hand="detail", pile=figD, coverage=5.0, clip=true, angle=1.5708, length={3,9}, pressure={0.5,0.9}})
-- hair / head
headM = ellipse(fx0+0.2, fy0-58.5, 3.6, 4.6)
work(headM, {hand="detail", pile=figHair, coverage=3.0, clip=true, angle=1.5708, length={2,6}, pressure={0.4,0.8}})
-- staff
stf = brush{kind="rigger", width=2, point=1}
stf:load(figD, 1.0)
stf:stroke({{fx0+12.6, fy0+0.5},{fx0+11.8, fy0-14},{fx0+11.0, fy0-30},{fx0+10.4, fy0-44}}, {pressure={0.55,0.35}, ramps={0.0,0.0}})
print(wait(0))

--@ chunk 206
figD2 = pile{{"raw umber",2.6},{"bone black",1.6},{"green earth",0.7},{"Prussian blue",0.25},{"red earth",0.2}}
work(figM, {hand="detail", pile=figD2, coverage=4.0, clip=true, angle=1.5708, length={3,9}, pressure={0.5,0.9}})
-- slightly lighter sleeve/arm on the staff side to separate it from the torso
armM = poly(FP({{5.6,50.5},{8.0,49.8},{9.0,46},{9.8,40},{10.0,33.5},{9.3,27.5},{8.2,27},{7.6,34},{7.0,42},{5.8,47}}), false)
armP = pile{{"raw umber",2.4},{"bone black",1.0},{"green earth",0.8},{"red earth",0.3},{"lead white",0.25}}
work(armM, {hand="detail", pile=armP, coverage=2.0, clip=true, angle=1.5708, length={3,8}, pressure={0.3,0.5}, load=0.5})
-- warm rim of the afterglow on the glow-side contours (right side)
rimPts = {{3.7,59},{3.0,60.9},{3.4,56.4},{5.2,52.2},{8.0,50},{9.2,46},{10.0,40},{10.2,33.5},{9.5,27.5},{8.9,21.5}}
rimL = {}
for _, p in ipairs(rimPts) do rimL[#rimL+1] = p end
rb = brush{kind="round", width=1.6, point=1}
rb:load(figLit, 0.7)
rb:stroke(FP({{3.3,59.5},{3.8,57.5},{3.4,55.6}}), {pressure={0.4,0.2}})
rb:load(figLit, 0.7)
rb:stroke(FP({{5.5,51.8},{8.0,49.7},{9.1,46},{9.9,40},{10.1,33.5}}), {pressure={0.4,0.3}})
rb:load(figLit, 0.6)
rb:stroke(FP({{10.0,33.5},{9.4,27.5},{8.8,21.6}}), {pressure={0.3,0.2}})
-- hat-less hair highlight
rb:load(figLit, 0.4)
rb:stroke(FP({{1.2,62.0},{2.8,61.0},{3.4,59.3}}), {pressure={0.3,0.2}})
print(wait(0))

--@ chunk 207
-- ===== BUTTE REWORK, 1: forested talus slope below the cliff =====
talusPts = {{746,414},{736,428},{722,446},{708,466},{696,486},{688,504},{760,512},{840,512},{906,504},{898,486},{888,466},{879,446},{874,426},{871,412}}
talusM = poly(talusPts, true):roughen(1.6, 13, 17)
TalD  = pile{{"smalt",2.0},{"raw umber",0.8},{"Prussian blue",0.22},{"lead white",1.0},{"red earth",0.2}}
TalD2 = pile{{"smalt",1.6},{"raw umber",1.0},{"Prussian blue",0.3},{"lead white",0.6},{"green earth",0.25}}
TalLt = pile{{"smalt",1.4},{"raw umber",0.5},{"lead white",1.9},{"yellow ochre",0.45},{"red earth",0.25}}
fadeFog = function(y) return clamp(1 - smoothstep(468, 506, y), 0, 1) end
-- dark body of forest, dabbed (tree-like) strokes
work(talusM, {hand="body", tool="filbert 8", pile=TalD, angle=1.3, coverage=4.4, clip=true, fill=true, length={5,16}, angle_jitter=0.6,
  load_at=function(x,y) return clamp(fadeFog(y) * (0.85 + 0.15*cn2(x,y)), 0, 1) end})
work(talusM, {hand="body", tool="filbert 6", pile=TalD2, angle=1.4, coverage=2.6, clip=true, length={4,12}, angle_jitter=0.6,
  load_at=function(x,y) return clamp(fadeFog(y) * (0.4 + 0.6*cn1(x,y)) * smoothstep(420, 450, y), 0, 1) end})
-- sunlit crowns on the glow-facing (left) slope
work(talusM, {hand="body", tool="filbert 5", pile=TalLt, angle=1.4, coverage=1.8, clip=true, length={3,9}, angle_jitter=0.6,
  load_at=function(x,y) return clamp(fadeFog(y) * (0.5 + 0.5*cn1(x+20,y)) * (1 - smoothstep(712, 790, x)) * smoothstep(414, 440, y) * (1 - smoothstep(470,500,y)), 0, 1) end})
print(wait(0))

--@ chunk 208
TalDD = pile{{"raw umber",2.2},{"Prussian blue",0.55},{"bone black",0.5},{"smalt",1.0},{"green earth",0.35},{"red earth",0.25}}
TalDW = pile{{"raw umber",2.0},{"smalt",1.2},{"yellow ochre",0.5},{"red earth",0.5},{"lead white",0.8},{"Prussian blue",0.15}}
work(talusM, {hand="body", tool="filbert 9", pile=TalDD, angle=1.3, coverage=5.5, clip=true, fill=true, length={6,18}, angle_jitter=0.6,
  load_at=function(x,y) return clamp(0.95 + 0.05*cn2(x,y), 0, 1) end})
-- warm sunlit tree crowns on the glow-facing left slope, higher up
work(talusM, {hand="body", tool="filbert 5", pile=TalDW, angle=1.4, coverage=2.0, clip=true, length={3,9}, angle_jitter=0.6,
  load_at=function(x,y) return clamp((0.35 + 0.65*cn1(x+20,y)) * (1 - smoothstep(716, 800, x)) * smoothstep(414, 436, y) * (1 - smoothstep(452,488,y)) * 0.85, 0, 1) end})
print(wait(0))

--@ chunk 209
print(wait(12*24*60))
print(drying(800,470), drying(244,480), drying(421,520), drying(300,300), drying(300,600), drying(800,360))

--@ chunk 210
-- ===== BUTTE REWORK, 2: dissolve the hard line, veil with haze, bury the base in fog =====
buVeil = (buM + talusM):grow(1) - crownM
HazeP = pile{{"lead white",5.5},{"pale smalt",1.0},{"red earth",0.28},{"yellow ochre",0.2}, medium=0.5}
HazeW = pile{{"lead white",5.5},{"yellow ochre",0.55},{"red earth",0.3},{"vermilion",0.12}, medium=0.5}
-- a) the join between cliff and forest: dab irregular tree crowns of the talus colour up into the cliff foot (ragged top of forest)
joinM = mask(function(x,y) return smoothstep(738, 748, x) * (1 - smoothstep(872, 880, x)) * bump(y, 410, 11) end) * buM
work(joinM, {hand="body", tool="filbert 5", pile=TalD, angle=1.4, coverage=2.4, clip=true, length={3,9}, angle_jitter=0.7,
  load_at=function(x,y) return clamp(smoothstep(396, 420, y) * (0.6 + 0.4*cn2(x,y)), 0, 1) end})
-- b) cool haze glaze over the whole massif, thicker toward the base and the right (shadow) side
work(buVeil, {hand="glaze", pile=HazeP, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp((0.25 + 0.75*smoothstep(400, 500, y)) * (0.8 + 0.2*smoothstep(790, 870, x)), 0, 1) end})
work(buVeil * mask(function(x,y) return 1 - smoothstep(740, 800, x) end), {hand="glaze", pile=HazeW, angle=0.0, coverage=1.6, clip=true,
  load_at=function(x,y) return clamp(0.8*smoothstep(430, 500, y), 0, 1) end})
print(wait(0))

--@ chunk 211
-- ===== BUTTE REBUILD (wet-in-wet, one session) =====
buAll = (buM + talusM) - crownM:grow(0.5)
-- rebuild cliff first: the full mass in the cool mid blue-violet
BCool2 = pile{{"smalt",1.9},{"raw umber",0.55},{"lead white",1.9},{"red earth",0.45},{"Prussian blue",0.06}}
work(buAll, {hand="body", tool="filbert 12", pile=BCool2, angle=vert3, coverage=5.5, clip=true, fill=true, length={30,80}, angle_jitter=0.05})
-- shadow face: right of the arete, deeper downward
work(buAll * mask(function(x,y) return smoothstep(aret3(y)-5, aret3(y)+5, x) end),
  {hand="body", tool="filbert 10", pile=BShadow, angle=vert3, coverage=4.6, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp((0.5 + 0.5*smoothstep(aret3(y), 872, x)) * (0.75 + 0.25*smoothstep(336, 420, y)), 0, 1) end})
-- warm lit face: left of the arete, strongest at the top left, going cooler lower down
work(buAll * mask(function(x,y) return 1 - smoothstep(aret3(y)-5, aret3(y)+5, x) end),
  {hand="body", tool="filbert 9", pile=BWarm, angle=vert3, coverage=4.4, clip=true, fill=true, length={26,70}, angle_jitter=0.06,
  load_at=function(x,y) return clamp((1 - smoothstep(340, 450, y)) * (0.6 + 0.4*(1 - smoothstep(744, aret3(y), x))), 0, 1) end})
work(buAll * mask(function(x,y) return 1 - smoothstep(aret3(y)-5, aret3(y)+5, x) end),
  {hand="body", tool="filbert 7", pile=BWarmHi, angle=vert3, coverage=2.6, clip=true, length={20,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(338, 398, y)) * (1 - smoothstep(744, aret3(y), x))^0.8 * 0.9, 0, 1) end})
blend(buAll, {angle=1.5708})
blend(buAll, {angle=1.5708})
print(wait(0))

--@ chunk 212
print(wait(8*24*60))
print(drying(800,470), drying(800,360), drying(760,400))

--@ chunk 213
-- ===== BUTTE: modelling on a dry ground =====
colShadow = buM * mask(function(x,y) return smoothstep(aret3(y)-3, aret3(y)+3, x) end)
colLit    = buM * mask(function(x,y) return 1 - smoothstep(aret3(y)-3, aret3(y)+3, x) end)
BSh2 = pile{{"smalt",1.5},{"raw umber",1.1},{"Prussian blue",0.22},{"lead white",0.8},{"red earth",0.35}}
BDp2 = pile{{"raw umber",1.3},{"Prussian blue",0.32},{"smalt",1.0},{"lead white",0.45},{"red earth",0.25}}
BLit2 = pile{{"yellow ochre",0.8},{"red earth",0.95},{"lead white",1.7},{"vermilion",0.38},{"smalt",0.14}}
BLitHi2 = pile{{"lead white",2.2},{"vermilion",0.7},{"yellow ochre",0.9},{"red earth",0.2}}
BLitMid = pile{{"red earth",0.8},{"smalt",0.7},{"lead white",1.6},{"yellow ochre",0.5},{"raw umber",0.3}}
-- shadow face
work(colShadow, {hand="body", tool="filbert 10", pile=BSh2, angle=vert3, coverage=5.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(336,352,y), 0, 1) end})
work(colShadow, {hand="body", tool="filbert 9", pile=BDp2, angle=vert3, coverage=3.2, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp((0.25 + 0.75*smoothstep(aret3(y)+8, 874, x)) * (0.5 + 0.5*smoothstep(340, 420, y)) * (1 - smoothstep(440, 480, y)), 0, 1) end})
-- lit face: a mid warm-grey body, then the alpenglow on top
work(colLit, {hand="body", tool="filbert 9", pile=BLitMid, angle=vert3, coverage=4.6, clip=true, fill=true, length={26,70}, angle_jitter=0.06})
work(colLit, {hand="body", tool="filbert 9", pile=BLit2, angle=vert3, coverage=4.0, clip=true, fill=true, length={26,70}, angle_jitter=0.06,
  load_at=function(x,y) return clamp((1 - smoothstep(345, 440, y)) * (0.55 + 0.45*(1 - smoothstep(744, aret3(y), x))), 0, 1) end})
work(colLit, {hand="body", tool="filbert 7", pile=BLitHi2, angle=vert3, coverage=2.4, clip=true, length={20,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(338, 392, y)) * (1 - smoothstep(744, aret3(y), x))^0.8 * 0.9, 0, 1) end})
print(wait(0))

--@ chunk 214
blend(buM, {angle=1.5708})
-- gullies & buttresses: irregular vertical ribbons, hand-placed
function gully(x0, y0, y1, wob, w0, w1, seed)
  local pts, ws = {}, {}
  local n = math.max(3, math.floor((y1-y0)/10))
  for i = 0, n do
    local t = i/n
    pts[#pts+1] = {x0 + wob*math.sin(i*1.3 + seed) + rand(-0.7,0.7), y0 + (y1-y0)*t}
    ws[#ws+1] = w0 + (w1-w0)*t
  end
  return ribbon(pts, ws)
end
gulL = gully(756, 342, 440, 1.4, 1.0, 4.0, 1) + gully(771, 346, 420, 1.2, 0.8, 3.2, 2) + gully(781, 340, 400, 1.0, 0.8, 2.4, 3)
gulR = gully(806, 340, 450, 1.6, 1.2, 5.0, 4) + gully(826, 338, 440, 1.5, 1.0, 4.4, 5) + gully(846, 338, 420, 1.2, 0.9, 3.0, 6) + gully(862, 340, 440, 1.0, 1.0, 3.6, 7)
work(gulL * colLit, {hand="body", tool="filbert 4", pile=BShadow, angle=1.5708, coverage=3.0, clip=true, length={8,24}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.35 + 0.65*smoothstep(345, 420, y), 0, 1) end})
work(gulR * colShadow, {hand="body", tool="filbert 4", pile=BDp2, angle=1.5708, coverage=3.0, clip=true, length={8,24}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.4 + 0.6*smoothstep(345, 420, y), 0, 1) end})
-- lit buttresses on the shadow side: thin warm-grey streaks catching the afterglow on edges that face left
butt = gully(801, 344, 408, 1.0, 0.6, 1.8, 8) + gully(822, 346, 396, 1.0, 0.6, 1.6, 9) + gully(842, 344, 392, 0.8, 0.6, 1.4, 10)
work(butt * colShadow, {hand="body", tool="filbert 3", pile=BLitMid, angle=1.5708, coverage=2.2, clip=true, length={6,18}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.8*(1 - smoothstep(350, 410, y)), 0, 1) end})
print(wait(0))

--@ chunk 215
local a0 = trunkIn:area()
print("trunk area", a0, "grow2", trunkIn:grow(2):area(), "offset2", trunkIn:offset(2):area(), "offset-2", trunkIn:offset(-2):area())
-- test a shifted-mask crescent: right side of the trunk
local sh = mask(function(x,y) return trunkIn:at(x+3, y) end)
local cres = trunkIn - sh
print("crescent area", cres:area())

--@ chunk 216
trunkClean = body_of{spine=OAK.trunk[1], widths=OAK.trunk[2], blend=0.8, char="firm", amount=0.0}:mask() + rootMid
BarkRim = pile{{"raw umber",2},{"yellow ochre",1.1},{"lead white",1.2},{"red earth",0.5}, medium=0.15}
BarkFur = pile{{"raw umber",3},{"red earth",0.9},{"lead white",0.45},{"Prussian blue",0.25}, medium=0.1}
shl = mask(function(x,y) return trunkClean:at(x+3.2, y) end)
rimM = trunkClean:shrink(1.0) - shl
print("rim area", rimM:area())
-- furrows first (lower contrast), then the rim
inner = trunkClean:shrink(3.0)
work(inner, {hand="detail", pile=BarkFur, angle=function(x,y) return 1.5708 + 0.28*math.sin(y/19 + x/7) end,
  coverage=1.6, clip=true, length={10,38}, pressure={0.15,0.35}, load=0.55,
  load_at=function(x,y) return clamp(0.15 + 0.85*smoothstep(216, 262, x), 0, 1) end})
work(rimM, {hand="detail", pile=BarkRim, angle=1.5708, coverage=1.4, clip=true, length={12,40}, pressure={0.18,0.38}, load=0.6,
  load_at=function(x,y) return clamp(0.35 + 0.65*smoothstep(420, 520, y)*0 + 0.65*bump(y, 470, 90), 0, 1) end})
print(wait(0))

--@ chunk 217
OakW2 = pile{{"raw umber",3},{"bone black",1.5},{"Prussian blue",0.6},{"red earth",0.3}, medium=0.0}
work(trunkClean:grow(0.6), {hand="body", tool="filbert 9", pile=OakW2, angle=function(x,y) return 1.5708 + 0.3*math.sin(y/23 + x/11) end,
  coverage=5.0, clip=true, fill=true, length={20,55}, load=1.0})
print(wait(0))

--@ chunk 218
BarkRim2 = pile{{"raw umber",2.2},{"yellow ochre",0.8},{"lead white",0.7},{"red earth",0.5}, medium=0.2}
rimM2 = trunkClean:shrink(0.6) - mask(function(x,y) return trunkClean:at(x+2.2, y) end)
work(rimM2, {hand="detail", pile=BarkRim2, angle=1.5708, coverage=1.0, clip=true, length={10,30}, pressure={0.12,0.28}, load=0.4,
  load_at=function(x,y) return clamp(0.2 + 0.8*bump(y, 480, 70), 0, 1) end})
-- vertical furrows, barely lighter than the dark body
BarkFur2 = pile{{"raw umber",3},{"bone black",1.2},{"Prussian blue",0.4},{"red earth",0.6},{"lead white",0.25}, medium=0.2}
work(trunkClean:shrink(3), {hand="detail", pile=BarkFur2, angle=function(x,y) return 1.5708 + 0.25*math.sin(y/21 + x/8) end,
  coverage=0.9, clip=true, length={12,40}, pressure={0.12,0.25}, load=0.4,
  load_at=function(x,y) return clamp(0.1 + 0.9*smoothstep(226, 262, x), 0, 1) end})
print(wait(0))

--@ chunk 219
-- ===== BUTTE v3: simplified, hazy, one coherent mass =====
-- wall body: cool violet-blue, bluer/darker at the top (against bright glow), paler toward the foot
wallBody = buM - crownM:grow(0.3)
BTop  = pile{{"smalt",1.9},{"raw umber",0.75},{"Prussian blue",0.12},{"lead white",1.2},{"red earth",0.4}}
BMid  = pile{{"smalt",1.7},{"raw umber",0.45},{"lead white",2.2},{"red earth",0.45},{"Prussian blue",0.04}}
work(wallBody, {hand="body", tool="filbert 12", pile=BMid, angle=vert3, coverage=6.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05})
work(wallBody, {hand="body", tool="filbert 12", pile=BTop, angle=vert3, coverage=4.0, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(1.0 - smoothstep(336, 440, y), 0, 1)^0.8 end})
-- shadow side: a gradient, no hard arete
BShadow3 = pile{{"smalt",1.5},{"raw umber",1.0},{"Prussian blue",0.2},{"lead white",1.0},{"red earth",0.3}}
work(wallBody, {hand="body", tool="filbert 12", pile=BShadow3, angle=vert3, coverage=3.4, clip=true, fill=true, length={30,80}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(782, 862, x) * (0.9 - 0.45*smoothstep(380, 460, y)), 0, 1) end})
-- warm lit edge on the glow side: strongest high, fading down and inward
BLit3 = pile{{"yellow ochre",0.7},{"red earth",0.85},{"lead white",1.8},{"vermilion",0.35},{"smalt",0.1}}
work(wallBody, {hand="body", tool="filbert 9", pile=BLit3, angle=vert3, coverage=3.6, clip=true, fill=true, length={26,70}, angle_jitter=0.06,
  load_at=function(x,y) return clamp((1 - smoothstep(744, 792, x)) * (1 - smoothstep(340, 440, y)), 0, 1)^0.9 end})
work(wallBody, {hand="body", tool="filbert 7", pile=BLitHi2, angle=vert3, coverage=2.0, clip=true, length={20,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(744, 772, x)) * (1 - smoothstep(338, 390, y)) * 0.85, 0, 1) end})
blend(wallBody, {angle=1.5708})
blend(wallBody, {angle=1.5708})
print(wait(0))

--@ chunk 220
soilFix = (rect(176, 529, 170, 80) * fgM) - (flareL + flareR + rootL2 + rootR2)
FmixB = pile{{"raw umber",3},{"red earth",1.0},{"green earth",0.45},{"Prussian blue",0.4},{"bone black",0.3}}
work(soilFix, {hand="body", tool="filbert 12", pile=FmixB, angle=crestAng, coverage=6.5, clip=true, fill=true, length={18,60}, angle_jitter=0.2, load=1.0})
-- a touch of the warm rim just below the crest line, and darker, cooler soil in the shade below
work(soilFix, {hand="body", tool="filbert 8", pile=Fwm, angle=crestAng, coverage=1.8, clip=true, length={14,40}, angle_jitter=0.2,
  load_at=function(x,y) local d = y - crestY(x); return clamp((1 - smoothstep(1, 14, d)) * 0.55, 0, 1) end})
print(wait(0))

--@ chunk 221
soilFix3 = rect(200, 526, 96, 60) * fgM
work(soilFix3, {hand="body", tool="filbert 12", pile=FmixB, angle=crestAng, coverage=7.0, clip=true, fill=true, length={16,50}, angle_jitter=0.2, load=1.0})
work(soilFix3, {hand="body", tool="filbert 8", pile=Fwm, angle=crestAng, coverage=1.8, clip=true, length={14,40}, angle_jitter=0.2,
  load_at=function(x,y) local d = y - crestY(x); return clamp((1 - smoothstep(1, 12, d)) * 0.5, 0, 1) end})
print(wait(0))

--@ chunk 222
-- 1. re-blend the soil around the base: wide, feathered repaint so the rectangle disappears
soilWide = rect(110, 522, 300, 110) * fgM
work(soilWide, {hand="body", tool="filbert 14", pile=FmixB, angle=crestAng, coverage=5.0, clip=true, fill=true, length={20,70}, angle_jitter=0.2,
  load_at=function(x,y) return clamp(smoothstep(110, 175, x) * (1 - smoothstep(330, 410, x)), 0, 1) end})
work(soilWide, {hand="body", tool="filbert 10", pile=Fwm, angle=crestAng, coverage=1.8, clip=true, length={14,40}, angle_jitter=0.2,
  load_at=function(x,y) local d = y - crestY(x); return clamp((1 - smoothstep(1, 14, d)) * 0.5 * smoothstep(110,175,x) * (1 - smoothstep(330,410,x)), 0, 1) end})
-- 2. the trunk's foot: one broad flare with a rounded, slightly uneven bottom edge, roots reaching out along the soil
footPts = {{214,470},{217,496},{211,516},{200,532},{184,543},{168,549},{158,552},{172,556},{194,558},{210,562},{228,566},{248,567},{268,565},{286,561},{302,558},{322,556},{342,559},{356,556},{338,547},{318,539},{300,531},{289,516},{281,496},{276,470}}
footM = poly(footPts, true):roughen(1.2, 10, 8)
work(footM, {hand="body", tool="filbert 10", pile=OakW2, angle=1.45, coverage=5.5, clip=true, fill=true, length={14,44}, angle_jitter=0.35, load=1.0})
print(wait(0))

--@ chunk 223
-- root ridges catching a little light on their tops, and shadow gaps between
function rootRibbon(pts, w0, w1)
  local ws = {}
  for i = 1, #pts do ws[i] = w0 + (w1-w0)*(i-1)/(#pts-1) end
  return body_of{spine=pts, widths=ws, blend=0.8, char="firm", amount=1.0}:mask()
end
rr1 = rootRibbon({{222,528},{204,540},{186,550},{166,556}}, 12, 2.5)
rr2 = rootRibbon({{236,540},{222,554},{206,562},{188,567}}, 11, 2.2)
rr3 = rootRibbon({{262,540},{278,552},{298,558},{322,560}}, 11, 2.2)
rr4 = rootRibbon({{274,526},{292,538},{314,546},{338,551}}, 12, 2.5)
rr5 = rootRibbon({{248,548},{252,560},{256,570},{262,578}}, 9, 2.0)
roots2 = rr1 + rr2 + rr3 + rr4 + rr5
work(roots2, {hand="body", tool="filbert 8", pile=OakW2, angle=0.7, coverage=4.5, clip=true, fill=true, length={10,30}, angle_jitter=0.35, load=1.0})
-- warm light along upper edges of the right-hand roots / right face of the foot
footRim = (footM:shrink(0.8) - mask(function(x,y) return footM:at(x+2.4, y+1.4) end))
work(footRim, {hand="detail", pile=BarkRim2, angle=1.0, coverage=0.9, clip=true, length={6,18}, pressure={0.12,0.26}, load=0.4,
  load_at=function(x,y) return clamp(0.8*smoothstep(240, 290, x), 0, 1) end})
-- contact shadow: darker soil gathered around the foot
shadowM = (footM:grow(9) - footM):blur(3) * fgM
work(shadowM, {hand="body", tool="filbert 8", pile=Fcool, angle=crestAng, coverage=2.0, clip=true, length={10,30}, angle_jitter=0.3,
  load_at=function(x,y) return 0.55 end})
print(wait(0))

--@ chunk 224
ringM = (footM:grow(16) - footM:shrink(-1)) * fgM
work(ringM, {hand="body", tool="filbert 9", pile=FmixB, angle=crestAng, coverage=5.0, clip=true, fill=true, length={12,40}, angle_jitter=0.25, load=1.0})
ShadowW = pile{{"raw umber",3},{"bone black",1.6},{"red earth",0.5}}
work((footM:grow(7) - footM):blur(3) * fgM, {hand="body", tool="filbert 7", pile=ShadowW, angle=crestAng, coverage=2.2, clip=true, length={10,28}, angle_jitter=0.3,
  load_at=function(x,y) return 0.5 end})
print(wait(0))

--@ chunk 225
-- ===== fog banks rolling across the butte's foot: soft, horizontal, thin at the top =====
fogFoot = mask(function(x,y) return sb(x, 640, 700, 960, 1010) * smoothstep(436, 520, y) end) - fgM
function ffb(p, f, cov)
  work(fogFoot, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={70,200}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
fogEdge = function(x) return 468 + 12*fogwisp(x*1.2, 40) + 6*fogwisp(x*0.5, 90) end
ffb(PW2b, function(x,y) return smoothstep(fogEdge(x)-6, fogEdge(x)+26, y) end, 4.2)
ffb(PW1b, function(x,y) return 0.8*bump(x,760,160)*smoothstep(fogEdge(x)+4, fogEdge(x)+30, y)*(1-smoothstep(fogEdge(x)+60, fogEdge(x)+95, y)) end, 2.8)
-- a thin low band of mist higher on the butte, wispy, translucent
wispM = lens(690, 920, 438, 10, 2, 0.5) - fgM
work(wispM, {hand="glaze", pile=pile{{"lead white",6},{"pale smalt",0.5},{"yellow ochre",0.2}, medium=0.5}, angle=0.0, coverage=2.0, clip=true,
  load_at=function(x,y) return 0.9 end})
print(wait(0))

--@ chunk 226
print(wait(8*24*60))
print(drying(800,520), drying(800,445), drying(300,590), drying(420,520), drying(240,540))

--@ chunk 227
-- ===== STEP 1: butte lower body and wooded foot, re-laid over the dry fog coat, dissolving into the fog =====
lowB = (buM + talusM) * rect(680, 406, 240, 106)
topFade = function(y) return smoothstep(406, 432, y) end
-- 1. lilac body
work(lowB, {hand="body", tool="filbert 12", pile=BMid, angle=vert3, coverage=6.0, clip=true, fill=true, length={30,80}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(topFade(y), 0, 1) end})
-- 2. top-tone and the shadowed right side, as in the cliff above
work(lowB, {hand="body", tool="filbert 12", pile=BShadow3, angle=vert3, coverage=3.6, clip=true, fill=true, length={30,80}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(topFade(y) * smoothstep(782, 862, x) * 0.85, 0, 1) end})
-- 3. warm light on the left edge, fading down
work(lowB, {hand="body", tool="filbert 9", pile=BLit3, angle=vert3, coverage=3.2, clip=true, fill=true, length={26,70}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(topFade(y) * (1 - smoothstep(744, 792, x)) * (1 - smoothstep(412, 462, y)), 0, 1) end})
-- 4. forest on the foot: dark crowns dabbed on, thicker toward the base
work(talusM * rect(680, 420, 240, 90), {hand="body", tool="filbert 7", pile=TalD, angle=1.3, coverage=3.4, clip=true, fill=true, length={4,12}, angle_jitter=0.7,
  load_at=function(x,y) return clamp(smoothstep(432, 462, y) * (1 - smoothstep(470, 500, y)) * (0.75 + 0.25*cn2(x,y)), 0, 1) end})
work(talusM * rect(680, 420, 240, 90), {hand="body", tool="filbert 5", pile=TalDW, angle=1.4, coverage=1.6, clip=true, length={3,9}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((0.3 + 0.7*cn1(x+20,y)) * (1 - smoothstep(716, 800, x)) * smoothstep(425, 445, y) * (1 - smoothstep(462,484,y)) * 0.8, 0, 1) end})
-- 5. the fog takes the base: cool cream, strongest low; warm cream near the glow side
work(lowB, {hand="body", tool="filbert 18", pile=PW2b, angle=0.0, coverage=4.0, clip=true, fill=true, length={50,140}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(462, 498, y) * (0.85 + 0.15*fq(x,y)), 0, 1) end})
work(lowB, {hand="body", tool="filbert 14", pile=PW1b, angle=0.0, coverage=2.4, clip=true, fill=true, length={40,120}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.8*bump(x,740,150)*smoothstep(470, 496, y) * fq(x+60,y), 0, 1) end})
print(wait(0))

--@ chunk 228
-- ===== fog at the butte foot: raise the fog line to ~y440 so the rock stands IN the mist (no skirt) =====
fogB = rect(650, 414, 300, 120) - fgM
topF = function(x) return 441 + 5*fogwisp(x*1.3, 10) + 3*fogwisp(x*0.5, 60) end
function fbw(p, f, cov, tool)
  work(fogB, {hand="body", tool=tool or "filbert 20", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={60,170}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
sideF = function(x) return smoothstep(650, 700, x) * (1 - smoothstep(900, 950, x)) end
fbw(PW2b, function(x,y) return smoothstep(topF(x)-3, topF(x)+16, y) * sideF(x) end, 4.6)
fbw(PW1b, function(x,y) return 0.75*bump(x,720,120)*smoothstep(topF(x), topF(x)+20, y)*(1-smoothstep(topF(x)+34, topF(x)+60, y)) * sideF(x) end, 2.6)
-- thin streaming wisps across the cliff foot: noise-shaped, translucent
wispN = noise{seed=91, period=60, octaves=2, persistence=0.5, stretch={0, 4}}
wispMask = mask(function(x,y)
  if x < 730 or x > 890 then return 0 end
  local c = 424 + 8*wispN(x, 5)
  local d = (y - c)
  local env = 1 - smoothstep(0, 9, math.abs(d))
  return env * smoothstep(0.0, 0.55, wispN(x*0.8+40, y*0.3)) * sb(x, 730, 760, 860, 890)
end)
work(wispMask, {hand="glaze", pile=pile{{"lead white",6},{"pale smalt",0.6},{"yellow ochre",0.2}, medium=0.55}, angle=0.0, coverage=1.6, clip=true,
  load_at=function(x,y) return 0.8 end})
print(wait(0))

--@ chunk 229
-- experiment: does load_at=0 paint anything? (inside the cream region, to be repainted anyway)
EXP = pile{{"Prussian blue",1},{"bone black",0.3}}
tA = rect(740, 590, 100, 100) - fgM
tB = rect(860, 590, 100, 100) - fgM
work(tA, {hand="body", tool="filbert 14", pile=EXP, angle=0.0, coverage=3, clip=true, fill=true, length={40,90},
  load_at=function(x,y) return clamp(smoothstep(640, 660, y),0,1) end})
work(tB, {hand="body", tool="filbert 14", pile=EXP, angle=0.0, coverage=3, clip=true, fill=false, length={40,90},
  load_at=function(x,y) return clamp(smoothstep(640, 660, y),0,1) end})
print(wait(0))

--@ chunk 230
mC = mask(function(x,y) return smoothstep(505, 545, y) end) * rect(700,470,100,90)
mD = mask(function(x,y) return smoothstep(505, 545, y) end) * rect(830,470,100,90)
work(mC, {hand="body", tool="filbert 14", pile=EXP, angle=0.0, coverage=3, clip=true, fill=false, length={40,90}})
work(mD, {hand="body", tool="filbert 14", pile=EXP, angle=0.0, coverage=3, clip=true, fill=true, length={40,90}})
print(wait(0))

--@ chunk 231
-- ===== FOREGROUND UNIFICATION: one coherent texture over the whole crest/slope, sparing the oak's foot and the figure =====
protect = footM:grow(2.5) + figM:grow(2.0) + rect(404, 536, 36, 14)
fgWork = fgM - protect
-- base earth, long strokes along the slope
Fbase = pile{{"raw umber",3},{"red earth",1.0},{"green earth",0.55},{"Prussian blue",0.45},{"bone black",0.35}}
work(fgWork, {hand="body", tool="filbert 20", pile=Fbase, angle=crestAng, coverage=3.6, clip=true, fill=true, length={40,120}, angle_jitter=0.12, load=1.0})
-- olive patches
work(fgWork, {hand="body", tool="filbert 16", pile=Fol, angle=crestAng, coverage=2.4, clip=true, length={30,90}, angle_jitter=0.18,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.45 + 0.95*fgn1(x,y)) * smoothstep(-2, 36, d) * (1 - smoothstep(110, 200, d)), 0, 1) end})
-- warm dry grass in the light just below the crest, stronger to the right (nearer the glow)
work(fgWork, {hand="body", tool="filbert 12", pile=Fwm, angle=crestAng, coverage=2.2, clip=true, length={25,70}, angle_jitter=0.22,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.3 + 0.9*fgn2(x+90,y)) * (1 - smoothstep(4, 62, d)) * smoothstep(-3, 6, d) * (0.5 + 0.5*smoothstep(150,750,x)), 0, 1) end})
-- cool deep shade low down
work(fgWork, {hand="body", tool="filbert 18", pile=Fcool, angle=crestAng, coverage=2.6, clip=true, length={30,90}, angle_jitter=0.15,
  load_at=function(x,y) local d = y - crestY(x); return clamp(smoothstep(60, 170, d) * (0.6 + 0.6*fgn1(x+30,y)), 0, 1) end})
print(wait(0))

--@ chunk 232
print(wait(10*24*60))
print(drying(800,520), drying(800,445), drying(300,590), drying(420,520), drying(240,540), drying(700,500))

--@ chunk 233
-- ===== VALLEY REPAINT, step 1: one fresh fog coat over the damaged right half, soft edges everywhere =====
function bxf(x) return sb(x, 722, 748, 870, 892) end   -- 1 across the butte's width
function fogTopV(x) return lerp(Atop(x)+14, 440, bxf(x)) end
fogV = mask(function(x,y)
  if y > crestY(x) - 1.5 then return 0 end
  local t = fogTopV(x)
  return smoothstep(590, 665, x) * smoothstep(t, t+34, y)
end)
function fvw(p, f, cov, tool)
  work(fogV, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={60,170}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
fvw(PW2b, function(x,y) return 1 end, 5.0)
-- warm cream: strongest under the glow and high in the fog
fvw(PW1b, function(x,y) return 0.85*bump(x,660,260)*(1 - smoothstep(470,580,y)) end, 3.0)
-- peach tint hugging the horizon under the glow
fvw(PWpeach, function(x,y) return 0.6*bump(x,650,200)*(1 - smoothstep(430,480,y)) end, 2.2)
-- cool lilac, deeper in the nearer, lower and right parts
fvw(PW3b, function(x,y) return smoothstep(530,680,y)*(0.5 + 0.5*smoothstep(650,950,x)) * 0.85 end, 3.0)
print(wait(0))

--@ chunk 234
print(wait(9*24*60))
print(drying(800,520), drying(800,445), drying(700,500), drying(630,470), drying(300,600))

--@ chunk 235
-- ===== BUTTE FOOT, v4: soft masks, weighted by clip; fades into the fog by mask value, not by load =====
softDown = function(y0, y1) return mask(function(x,y) return 1 - smoothstep(y0, y1, y) end) end
lowSoft = (buM + talusM) * rect(690, 396, 220, 130) * softDown(436, 492) - crownM:grow(0.5)
-- 1. lilac body
work(lowSoft, {hand="body", tool="filbert 12", pile=BMid, angle=vert3, coverage=6.0, clip=true, fill=true, length={30,80}, angle_jitter=0.06})
-- 2. shadowed right half and the top-tone
work(lowSoft * mask(function(x,y) return smoothstep(782, 862, x) end), {hand="body", tool="filbert 12", pile=BShadow3, angle=vert3, coverage=3.6, clip=true, fill=true, length={30,80}, angle_jitter=0.06})
work(lowSoft * mask(function(x,y) return 1 - smoothstep(406, 446, y) end), {hand="body", tool="filbert 12", pile=BTop, angle=vert3, coverage=3.0, clip=true, fill=true, length={30,80}, angle_jitter=0.06})
-- 3. warm light on the left edge, fading down
work(lowSoft * mask(function(x,y) return (1 - smoothstep(744, 792, x)) * (1 - smoothstep(416, 466, y)) end), {hand="body", tool="filbert 9", pile=BLit3, angle=vert3, coverage=3.0, clip=true, fill=true, length={26,70}, angle_jitter=0.06})
-- 4. forest crowns on the foot (talus only), dark, ragged, thicker toward the base
forestFoot = talusM * rect(690, 428, 220, 80) * softDown(458, 500)
work(forestFoot, {hand="body", tool="filbert 6", pile=TalD, angle=1.3, coverage=3.4, clip=true, fill=true, length={4,12}, angle_jitter=0.7})
work(forestFoot * mask(function(x,y) return (1 - smoothstep(716, 800, x)) * (0.4 + 0.6*cn1(x+20,y)) * (1 - smoothstep(448, 484, y)) end),
  {hand="body", tool="filbert 5", pile=TalDW, angle=1.4, coverage=1.8, clip=true, length={3,9}, angle_jitter=0.7})
print(wait(0))

--@ chunk 236
-- crest edge repair on the lower right (blue swatch line) with the dark earth
edgeBand = mask(function(x,y) return (x > 700 and x < 1010 and y > crestY(x) - 3.5 and y < crestY(x) + 4) and 1 or 0 end)
work(edgeBand, {hand="body", tool="filbert 6", pile=F2, angle=crestAng, coverage=5.0, clip=true, fill=true, length={20,60}, angle_jitter=0.08, load=1.0})
-- a rim of warm, dim light along the edge
work(edgeBand, {hand="body", tool="filbert 4", pile=Fwm, angle=crestAng, coverage=1.2, clip=true, length={14,40}, angle_jitter=0.1, load=0.5})
print(wait(0))

--@ chunk 237
grassDark = pile{{"raw umber",3},{"bone black",1.4},{"Prussian blue",0.5},{"red earth",0.3}}
grassLit  = pile{{"raw umber",1.4},{"yellow ochre",1.3},{"lead white",1.3},{"red earth",0.35}}
grassMid  = pile{{"raw umber",2.0},{"yellow ochre",0.9},{"green earth",0.5},{"lead white",0.4}}
gb = brush{kind="rigger", width=2, point=1}
function blade(bx, by, h, lean, pile_, w)
  local ang = -1.5708 + lean
  local mx = bx + math.cos(ang - lean*0.35) * h * 0.55
  local my = by + math.sin(ang - lean*0.35) * h * 0.55
  local tx = bx + math.cos(ang) * h
  local ty = by + math.sin(ang) * h
  gb:load(pile_, 0.75)
  gb:stroke({{bx,by},{mx,my},{tx,ty}}, {pressure={w or 0.55, 0.04}, ramps={0.0, 0.0}})
end
function tuft(x, ybase, n, hmax, pile_, lean0, spread, w)
  for i = 1, n do
    blade(x + rand(-spread, spread), ybase + rand(-1.2, 1.2), hmax * rand(0.35, 1.0), (lean0 or 0) + rand(-0.55, 0.55), pile_, w)
  end
end
-- 1. dark silhouetted tufts standing on the crest line (against the bright fog / sky)
local x = 6
while x < 1000 do
  if not ((x > 150 and x < 372) or (x > 398 and x < 446)) then
    local yb = crestY(x) + 1.5
    tuft(x, yb, math.random(4, 8), rand(8, 20), grassDark, 0.0, 5, 0.5)
  end
  x = x + rand(10, 30)
end
print(wait(0))

--@ chunk 238
g2 = brush{kind="rigger", width=1.0, point=1}
print("g2 widths", g2:mark_width(0.05), g2:mark_width(0.2), g2:mark_width(0.4), g2:mark_width(0.7), g2:mark_width(1.0))
g3 = brush{kind="round", width=1.0, point=1}
print("g3 widths", g3:mark_width(0.05), g3:mark_width(0.2), g3:mark_width(0.4), g3:mark_width(0.7), g3:mark_width(1.0))

--@ chunk 239
function blade2(bx, by, h, lean, pile_, wmax)
  -- a curved, tapering blade: starts near vertical, bends to the lean side
  local n = 5
  local pts = {}
  for i = 0, n do
    local t = i/n
    local bend = lean * t * t * 1.8
    local x = bx + math.sin(bend) * h * t + lean * 0.0
    local y = by - math.cos(bend) * h * t
    pts[#pts+1] = {x, y}
  end
  g3:load(pile_, 0.8)
  g3:stroke(pts, {pressure={wmax or 0.55, 0.05}, ramps={0.0, 0.0}})
end
function tuft2(x, ybase, n, hmax, pile_, lean0, spread, wmax)
  for i = 1, n do
    local side = (math.random() < 0.5) and -1 or 1
    blade2(x + rand(-spread, spread), ybase + rand(-1.0, 1.0), hmax * rand(0.3, 1.0), (lean0 or 0) + side*rand(0.1, 0.6), pile_, wmax)
  end
end
-- fine dark blades fanning from the existing tufts, against the sky/fog
for i = 1, 40 do
  local x = rand(6, 990)
  if not ((x > 150 and x < 372) or (x > 398 and x < 446)) then
    tuft2(x, crestY(x) + 1.5, math.random(5, 9), rand(10, 24), grassDark, 0.0, 6, 0.5)
  end
end
print(wait(0))

--@ chunk 240
-- ===== BUTTE v5: fluted sandstone pillars, lit from the left; wet-in-wet in one session =====
bodyPts = {{745,343},{744,362},{743,380},{742,398},{741,414},{738,428},{733,441},{726,452},{716,462},{703,471},
           {703,492},{912,492},
           {904,472},{893,461},{884,450},{878,438},{875,424},{874,408},{873,392},{872,376},{870,360},{868,343}}
bodyM = poly(bodyPts, true):roughen(1.0, 12, 5) - crownM:grow(0.4)
function slab(x0t, x1t, x0b, x1b)
  return poly({{x0t,330},{x1t,330},{x1b,500},{x0b,500}})
end
slabs = {
  {slab(734,760,728,758), BLit3,   0.95},   -- warm lit left flank
  {slab(758,778,756,776), BMid,    0.9},    -- mid
  {slab(776,790,774,788), BShadow3,0.9},    -- dark crevice
  {slab(788,812,786,812), BLitMid, 0.9},    -- pale pillar catching light
  {slab(810,826,810,824), BDp2,    0.9},    -- dark cleft
  {slab(824,846,822,848), BSh2,    0.9},    -- mid-dark pillar
  {slab(844,880,846,914), BDp2,    0.9},    -- darkest right face
}
-- base: a cool lilac body for the whole mass first
work(bodyM, {hand="body", tool="filbert 12", pile=BMid, angle=vert3, coverage=6.0, clip=true, fill=true, length={30,90}, angle_jitter=0.05})
for i, s in ipairs(slabs) do
  work(s[1] * bodyM, {hand="body", tool="filbert 7", pile=s[2], angle=vert3, coverage=4.6, clip=true, fill=true, length={20,80}, angle_jitter=0.08})
end
-- light on the left edge of each lit slab, a rim of shadow on its right
work(slabs[4][1] * bodyM, {hand="body", tool="filbert 5", pile=BLit3, angle=vert3, coverage=2.4, clip=true, length={14,50},
  load_at=function(x,y) return clamp((1 - smoothstep(790, 806, x)) * (1 - smoothstep(345, 440, y)), 0, 1) end})
work(slabs[6][1] * bodyM, {hand="body", tool="filbert 5", pile=BLitMid, angle=vert3, coverage=2.0, clip=true, length={14,50},
  load_at=function(x,y) return clamp((1 - smoothstep(826, 838, x)) * (1 - smoothstep(345, 420, y)) * 0.9, 0, 1) end})
work(slabs[1][1] * bodyM, {hand="body", tool="filbert 5", pile=BLitHi2, angle=vert3, coverage=2.0, clip=true, length={14,50},
  load_at=function(x,y) return clamp((1 - smoothstep(742, 756, x)) * (1 - smoothstep(340, 400, y)) * 0.85, 0, 1) end})
-- haze deepens toward the foot: pale cool cream, load rises downward
work(bodyM, {hand="body", tool="filbert 12", pile=PW3b, angle=vert3, coverage=3.4, clip=true, fill=true, length={30,90}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(smoothstep(410, 470, y) * 0.8, 0, 1) end})
-- soften the joins between slabs: a short horizontal drag, then the vertical one
blend(bodyM, {angle=0.0})
blend(bodyM, {angle=1.5708})
print(wait(0))

--@ chunk 241
print(wait(9*24*60))
print(drying(800,400), drying(800,470), drying(300,590), drying(700,520))

--@ chunk 242
-- ===== BUTTE v6: wooded scree flanks make it a table-mountain on a broad ridge =====
bigPts = {{745,343},{744,362},{743,380},{742,398},{738,410},{731,421},{722,431},{711,441},{698,450},{683,459},{666,467},{646,474},{620,480},{590,486},
          {590,520},{1010,520},
          {1010,473},{1000,470},{975,465},{952,459},{932,452},{915,444},{901,435},{889,425},{880,414},{874,402},{873,392},{872,376},{870,360},{868,343}}
bigM = poly(bigPts, true):roughen(1.5, 11, 7)
tlT = function(x) return 407 + 7*trn(x*1.3, 5) end
cliffFoot = mask(function(x,y) return (x >= 741 and x <= 876 and y < tlT(x)) and 1 or 0 end)
screeM = bigM - cliffFoot
ScDark  = pile{{"raw umber",2.0},{"Prussian blue",0.55},{"smalt",1.0},{"green earth",0.4},{"lead white",0.45},{"bone black",0.25}}
ScMid   = pile{{"raw umber",1.4},{"Prussian blue",0.3},{"smalt",1.5},{"green earth",0.3},{"lead white",1.1},{"red earth",0.2}}
ScFar   = pile{{"lead white",2.3},{"smalt",2.0},{"raw umber",0.45},{"red earth",0.3},{"Prussian blue",0.06}}
ScLit   = pile{{"yellow ochre",0.9},{"raw umber",1.2},{"green earth",0.5},{"lead white",1.0},{"smalt",0.4}}
zoneFar  = screeM * mask(function(x,y) return 1 - smoothstep(620, 700, x) end)
zoneMid  = screeM * mask(function(x,y) return smoothstep(640, 700, x) * (1 - smoothstep(900, 980, x)) end)
zoneRFar = screeM * mask(function(x,y) return smoothstep(900, 985, x) end)
dab = {hand="body", tool="filbert 8", angle=1.35, coverage=5.0, clip=true, fill=true, length={5,14}, angle_jitter=0.6}
function dabwork(m, p, cov, tool, extra)
  local o = {hand="body", tool=tool or "filbert 8", pile=p, angle=1.35, coverage=cov, clip=true, fill=true, length={5,14}, angle_jitter=0.6}
  if extra then for k, v in pairs(extra) do o[k] = v end end
  work(m, o)
end
dabwork(zoneFar, ScFar, 5.0)
dabwork(zoneMid, ScMid, 5.0)
dabwork(zoneRFar, ScFar, 5.0)
-- the nearer, darker mass hugging the cliff foot, both sides
core = screeM * mask(function(x,y) return sb(x, 668, 728, 900, 960) * smoothstep(398, 430, y) end)
dabwork(core, ScDark, 4.6)
-- ragged tree-line where forest meets cliff: dark crowns climbing into the cliff base
rag = mask(function(x,y) return (x >= 742 and x <= 874) and bump(y, tlT(x), 7) or 0 end) * bigM
dabwork(rag, ScDark, 2.6, "filbert 5", {length={3,9}})
-- sunlit crowns on the glow-facing left slope
work(screeM * mask(function(x,y) return (1 - smoothstep(700, 770, x)) * smoothstep(404, 428, y) * (1 - smoothstep(448, 476, y)) * (0.35 + 0.65*cn1(x+20,y)) end),
  {hand="body", tool="filbert 5", pile=ScLit, angle=1.4, coverage=1.8, clip=true, length={3,9}, angle_jitter=0.7})
print(wait(0))

--@ chunk 243
-- ===== (a) the forested foot of the butte sinks into the fog (soft, by load) =====
fogLvl = function(x) return 486 + 7*fogwisp(x*1.1, 20) + 4*fogwisp(x*0.45, 70) end
veilZone = rect(570, 428, 450, 130) - fgM
function vz(p, f, cov, tool)
  work(veilZone, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={60,170}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
-- cool lilac first (thin at the top, opaque below the fog level)
vz(PW3b, function(x,y) return smoothstep(fogLvl(x)-46, fogLvl(x)+4, y) * smoothstep(575, 640, x) end, 4.0)
-- then cream, only in the lower part (fog top)
vz(PW2b, function(x,y) return smoothstep(fogLvl(x)-14, fogLvl(x)+22, y) * smoothstep(575, 640, x) end, 4.6)
vz(PW1b, function(x,y) return 0.8*bump(x,680,170)*smoothstep(fogLvl(x)-6, fogLvl(x)+14, y)*(1-smoothstep(fogLvl(x)+30, fogLvl(x)+70, y)) end, 2.6)
print(wait(0))

--@ chunk 244
tm = rect(600, 458, 380, 80) - fgM
work(tm, {hand="body", tool="filbert 22", pile=PW2b, coverage=3.2, angle=0.0, edge="lost", fill=true, length={60,150}, angle_jitter=0.04})
print(wait(0))

--@ chunk 245
print(wait(10*24*60))
print(drying(800,440), drying(800,500), drying(700,520), drying(650,470))

--@ chunk 246
-- ===== (b) the forested foot of the butte, restored over the dry veil =====
footZone = screeM * mask(function(x,y) return smoothstep(418, 432, y) * (1 - smoothstep(488, 506, y)) end)
zF = footZone * mask(function(x,y) return 1 - smoothstep(620, 700, x) end)
zM = footZone * mask(function(x,y) return smoothstep(640, 700, x) * (1 - smoothstep(900, 980, x)) end)
zR = footZone * mask(function(x,y) return smoothstep(900, 985, x) end)
dabwork(zF, ScFar, 5.0)
dabwork(zM, ScMid, 5.0)
dabwork(zR, ScFar, 5.0)
coreF = footZone * mask(function(x,y) return sb(x, 672, 730, 896, 950) end)
dabwork(coreF, ScDark, 4.6)
-- sunlit crowns on the left slope, a few darker conifer spikes against the lilac on both flanks
work(footZone * mask(function(x,y) return (1 - smoothstep(700, 770, x)) * (0.35 + 0.65*cn1(x+20,y)) * (1 - smoothstep(448, 486, y)) end),
  {hand="body", tool="filbert 5", pile=ScLit, angle=1.4, coverage=1.8, clip=true, length={3,9}, angle_jitter=0.7})
print(wait(0))

--@ chunk 247
-- ===== fog restored on both flanks (band cover) and over the forest's foot =====
bandCover = mask(function(x,y)
  if x < 562 or x > 1012 or y < 414 or y > 506 then return 0 end
  return smoothstep(562, 600, x) * smoothstep(414, 436, y)
end) - bigM:grow(1) - fgM
work(bandCover, {hand="body", tool="filbert 20", pile=PW2b, angle=0.0, coverage=4.4, clip=true, fill=true, length={60,160}, angle_jitter=0.04})
-- a little lilac in the far part of the left flank so it reads as ridge, not as fog
fogMaskLow = mask(function(x,y) return smoothstep(fogLvl(x)-8, fogLvl(x)+16, y) * smoothstep(570, 640, x) end) * rect(560, 440, 460, 130) - fgM
work(fogMaskLow, {hand="body", tool="filbert 22", pile=PW2b, coverage=4.4, angle=0.0, edge="lost", fill=true, length={60,160}, angle_jitter=0.04})
print(wait(0))

--@ chunk 248
-- ===== A: dome B restored across the fog seam, its base sinking into the fog =====
domeTop = {{520,497},{545,490},{570,485},{592,482},{622,486},{652,494},{682,503},{714,512},{744,524},{772,540},{800,558}}
function domeMaskFn(x, y)
  if x < 520 or x > 805 then return 0 end
  local t = interp(domeTop, x) + 2.6*trn(x*1.0, 3.0)
  return smoothstep(t-0.6, t+0.6, y) * smoothstep(520, 585, x) * (y < 600 and 1 or 0)
end
domeM = mask(domeMaskFn) - fgM
work(domeM, {hand="body", tool="filbert 12", pile=PB3, angle=0.04, coverage=4.4, clip=true, fill=true, length={25,70}, angle_jitter=0.2})
-- slightly darker, bluer wooded core, lit tree-tops on the left (glow side)
work(domeM * mask(function(x,y) return smoothstep(540, 620, x) * (1 - smoothstep(520, 556, y)) end),
  {hand="body", tool="filbert 8", pile=PC3, angle=0.05, coverage=2.0, clip=true, length={20,50}, angle_jitter=0.3})
-- fog rising over the dome's foot, wavy top, lost edge
domeFog = mask(function(x,y)
  local t = 524 + 8*fogwisp(x*1.2, 33) + 5*fogwisp(x*0.5, 17) + 0.08*(x-600)
  return smoothstep(t-6, t+14, y) * smoothstep(500, 570, x)
end) * rect(500, 480, 400, 140) - fgM
work(domeFog, {hand="body", tool="filbert 22", pile=PW2b, coverage=4.4, angle=0.0, edge="lost", fill=true, length={60,160}, angle_jitter=0.04})
print(wait(0))

--@ chunk 249
-- ===== restore the crest where the fog overran it =====
crestFix = fgM * rect(440, 536, 330, 120)
work(crestFix, {hand="body", tool="filbert 14", pile=Fbase, angle=crestAng, coverage=5.0, clip=true, fill=true, length={30,90}, angle_jitter=0.1, load=1.0})
work(crestFix, {hand="body", tool="filbert 16", pile=Fol, angle=crestAng, coverage=1.8, clip=true, length={30,80}, angle_jitter=0.18,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.4 + 0.9*fgn1(x,y)) * smoothstep(-2, 36, d), 0, 1) end})
work(crestFix, {hand="body", tool="filbert 10", pile=Fwm, angle=crestAng, coverage=1.6, clip=true, length={25,60}, angle_jitter=0.22,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.3 + 0.9*fgn2(x+90,y)) * (1 - smoothstep(4, 40, d)) * smoothstep(-3, 6, d), 0, 1) end})
print(wait(0))

--@ chunk 250
-- ===== soften the vertical seam at x~565: thin feathered cream over the lilac to its left =====
seamSoft = mask(function(x,y)
  local vy = smoothstep(402, 420, y) * (1 - smoothstep(484, 506, y))
  return sb(x, 470, 548, 590, 640) * vy
end) - fgM - mask(function(x,y) return 0 end)
work(seamSoft, {hand="body", tool="filbert 20", pile=PW2b, angle=0.0, coverage=3.0, clip=true, fill=true, length={50,140}, angle_jitter=0.05})
print(wait(0))

--@ chunk 251
print(wait(12*24*60))
print(drying(520,430), drying(600,450), drying(500,600), drying(700,520), drying(300,300))

--@ chunk 252
-- ===== repair: right end of the low limb L4 and the root of S15 =====
L4b = body_of{spine=OAK.L4[1], widths=OAK.L4[2], blend=0.8, char="broken", amount=1.0}:mask() * rect(436, 396, 140, 100)
work(L4b, {hand="body", tool="filbert 9", pile=OakD, coverage=4.2, clip=true, angle=limbangle(OAK.L4), length={25,60}, fill=true, load=1.0})
paintlimb("S15", "firm", "filbert 7", 4.0)
-- twigs along the repaired reach of L4 (from about x 440 to the drooping tip), thicker and denser than before
sprout3("L4", 0.74, 1.0, 11, 2.6, 3, 22, 52, {p=0.6})
sprout3("S15", 0.0, 0.5, 11, 2.0, 2, 20, 44, {p=0.55})
print(wait(0))

--@ chunk 253
-- ===== BUTTE rework: dark blue-violet sandstone, fluted, lit warm on the glow side =====
ext1 = poly({{746,368},{739,376},{735,388},{736,400},{741,410},{747,410}}, true):roughen(0.8, 8, 3)
ext2 = poly({{868,350},{876,357},{880,368},{879,380},{874,390},{868,392}}, true):roughen(0.8, 8, 4)
cliffFace = (buM * cliffFoot) + ext1 + ext2
cliffAll = cliffFace + crownM
BCliff = pile{{"smalt",1.9},{"raw umber",0.75},{"Prussian blue",0.14},{"lead white",1.0},{"red earth",0.4}}
BCliffD = pile{{"smalt",1.4},{"raw umber",1.15},{"Prussian blue",0.3},{"lead white",0.5},{"red earth",0.3}}
BCliffL = pile{{"red earth",0.85},{"yellow ochre",0.55},{"lead white",1.5},{"vermilion",0.28},{"smalt",0.3}}
-- 1. base: cool mid-dark over everything (covers the pale plaster look)
work(cliffAll, {hand="body", tool="filbert 10", pile=BCliff, angle=vert3, coverage=6.0, clip=true, fill=true, length={26,80}, angle_jitter=0.06})
-- 2. shadow: deepens to the right and toward the foot of the cliff
work(cliffAll, {hand="body", tool="filbert 9", pile=BCliffD, angle=vert3, coverage=4.0, clip=true, fill=true, length={26,80}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(smoothstep(776, 856, x) * 0.95 + 0.0, 0, 1) end})
-- 3. light: warm band down the left side, strongest at top
work(cliffAll, {hand="body", tool="filbert 8", pile=BCliffL, angle=vert3, coverage=3.2, clip=true, fill=true, length={20,60}, angle_jitter=0.07,
  load_at=function(x,y) return clamp((1 - smoothstep(742, 786, x)) * (1 - smoothstep(336, 420, y))^0.8, 0, 1) end})
blend(cliffAll, {angle=1.5708})
print(wait(0))

--@ chunk 254
-- broad corrections while the cliff is still open: narrower, more intense lit edge; cooler mid-tone; dark needle and knob
midCool = cliffAll * mask(function(x,y) return smoothstep(754, 776, x) end)
work(midCool, {hand="body", tool="filbert 9", pile=BCliff, angle=vert3, coverage=4.5, clip=true, fill=true, length={26,80}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(smoothstep(754, 780, x) * (1 - 0.5*smoothstep(790, 850, x)), 0, 1) end})
work(cliffAll, {hand="body", tool="filbert 9", pile=BCliffD, angle=vert3, coverage=3.4, clip=true, fill=true, length={26,80}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(smoothstep(790, 862, x) * 0.95, 0, 1) end})
-- needle: dark body, warm light on its left edge only
needleM = needle:roughen(0.9, 9, 6)
work(needleM, {hand="body", tool="filbert 5", pile=BCliff, angle=1.5708, coverage=5.0, clip=true, fill=true, length={8,30}})
work(needleM * mask(function(x,y) return smoothstep(758, 765, x) end), {hand="body", tool="filbert 4", pile=BCliffD, angle=1.5708, coverage=3.0, clip=true, length={8,24}})
work(needleM * mask(function(x,y) return 1 - smoothstep(754, 758, x) end), {hand="body", tool="filbert 3", pile=BCliffL, angle=1.5708, coverage=3.0, clip=true, length={6,18},
  load_at=function(x,y) return clamp(0.4 + 0.6*(1 - smoothstep(290, 340, y)), 0, 1) end})
-- knob
towerK = tower:roughen(0.9, 9, 6)
work(towerK, {hand="body", tool="filbert 5", pile=BCliffD, angle=1.5708, coverage=5.0, clip=true, fill=true, length={6,24}})
work(towerK * mask(function(x,y) return 1 - smoothstep(826, 834, x) end), {hand="body", tool="filbert 3", pile=BCliff, angle=1.5708, coverage=3.0, clip=true, length={6,18}})
work(step:roughen(0.9, 9, 6), {hand="body", tool="filbert 5", pile=BCliffD, angle=1.5708, coverage=5.0, clip=true, fill=true, length={6,24}})
blend(cliffAll, {angle=1.5708})
print(wait(0))

--@ chunk 255
-- ===== BUTTE: organ-pipe pillars, each lit on its left edge, darker to the right; lit amount falls toward the right =====
bnd = {744, 757, 770, 780, 796, 809, 821, 837, 851, 863, 880}
nP = #bnd - 1
for i = 1, nP do
  local a, b = bnd[i], bnd[i+1]
  local Li = clamp(1.0 - (i-1)/(nP-1) * 0.9, 0.08, 1.0)
  local pm = cliffAll * rect(a-1, 300, (b-a)+2, 200)
  -- 1. dark base for the pillar
  work(pm, {hand="body", tool="filbert 6", pile=BCliffD, angle=vert3, coverage=4.0, clip=true, fill=true, length={20,70}, angle_jitter=0.05})
  -- 2. mid tone, strong at the pillar's left, fading to the right
  local midM = pm * mask(function(x,y) local t = (x-a)/(b-a); return clamp(1.0 - smoothstep(0.15, 0.95, t), 0, 1) * (0.55 + 0.45*Li) end)
  work(midM, {hand="body", tool="filbert 5", pile=BCliff, angle=vert3, coverage=3.6, clip=true, length={20,70}, angle_jitter=0.05})
  -- 3. warm light on the left edge, strongest high up (the glow is at the horizon, so lower parts get less)
  local litM = pm * mask(function(x,y)
    local t = (x-a)/(b-a)
    return clamp((1.0 - smoothstep(0.0, 0.5, t)) * Li * (1 - smoothstep(340, 430, y)*0.8), 0, 1)
  end)
  work(litM, {hand="body", tool="filbert 4", pile=BCliffL, angle=vert3, coverage=2.8, clip=true, length={14,50}, angle_jitter=0.06})
end
print(wait(0))

--@ chunk 256
-- ===== the fog sea gets swells: soft, wide, translucent lilac shadows and warm tops =====
swA = noise{seed=201, period=150, octaves=3, persistence=0.5, stretch={0, 4.5}}
swB = noise{seed=202, period=70,  octaves=3, persistence=0.5, stretch={0, 4.0}}
swC = noise{seed=203, period=110, octaves=2, persistence=0.5, stretch={0, 3.5}}
fogReg2 = mask(function(x,y)
  if y > crestY(x) - 3 then return 0 end
  if y < 446 then return 0 end
  local xs = smoothstep(560, 640, x)
  if y < 520 then xs = xs * smoothstep(598, 640, x) end
  return xs * smoothstep(446, 476, y)
end) - buM:grow(2) - treeAllG:grow(4) - figM:grow(4)
PWc  = pile{{"lead white",4},{"smalt",1.2},{"red earth",0.35},{"raw umber",0.08}, medium=0.6}
PWc2 = pile{{"lead white",4.5},{"pale smalt",1.4},{"red earth",0.25},{"yellow ochre",0.1}, medium=0.6}
PWh  = pile{{"lead white",6},{"yellow ochre",0.4},{"chrome yellow",0.12},{"vermilion",0.1}, medium=0.5}
perspec = function(y) return clamp((y-446)/260, 0, 1) end
-- cool shadows in the troughs: larger and stronger toward the viewer
shM = mask(function(x,y)
  local v = swA(x*1.0, y*(1.0 + 1.2*(1-perspec(y))))
  return smoothstep(0.10, 0.62, v) * (0.35 + 0.65*perspec(y)^0.8)
end) * fogReg2
work(shM, {hand="glaze", pile=PWc, angle=0.0, coverage=2.2, clip=true})
-- finer ones near the horizon
shM2 = mask(function(x,y)
  local v = swB(x*1.0, y*1.6)
  return smoothstep(0.18, 0.7, v) * (1 - perspec(y)) * 0.8
end) * fogReg2
work(shM2, {hand="glaze", pile=PWc2, angle=0.0, coverage=2.0, clip=true})
-- warm lit crests of the swells, strongest toward the glow
hiM = mask(function(x,y)
  local v = swC(x*1.0, y*1.5)
  return smoothstep(0.15, 0.7, v) * (0.3 + 0.7*math.exp(-((x-660)/300)^2)) * (1 - 0.6*perspec(y))
end) * fogReg2
work(hiM, {hand="glaze", pile=PWh, angle=0.0, coverage=1.8, clip=true})
print(wait(0))

--@ chunk 257
spB = brush{kind="round", width=2.2, point=1}
print("spB widths", spB:mark_width(0.1), spB:mark_width(0.3), spB:mark_width(0.5), spB:mark_width(0.8), spB:mark_width(1.0))
function spruce(x, yb, h, w, lean, pile_)
  lean = lean or 0
  local ytop = yb - h
  -- trunk (slightly leaning), tapering to a thin leader
  local tp = {{x, yb+1.5}, {x + lean*0.35*h, yb - h*0.4}, {x + lean*h, ytop}}
  spB:load(pile_, 1.0)
  spB:stroke(tp, {pressure={0.6, 0.12}, ramps={0.0, 0.0}})
  local n = math.max(7, math.floor(h/5.2))
  for i = 1, n do
    local t = (i - 0.4 + rand(-0.25, 0.25)) / n
    local y = ytop + h * (0.04 + 0.95*t)
    local tx = x + lean*h*(1 - (0.04 + 0.95*t))
    local L = (w/2) * (0.12 + 0.88*t^0.85) * rand(0.82, 1.15)
    for _, s in ipairs({-1, 1}) do
      local Ls = L * rand(0.85, 1.12)
      local droop = Ls * rand(0.38, 0.6)
      local pts = {{tx, y}, {tx + s*Ls*0.45, y + droop*0.28}, {tx + s*Ls*0.82, y + droop*0.78}, {tx + s*Ls*1.0, y + droop*0.92}}
      spB:load(pile_, 0.9)
      spB:stroke(pts, {pressure={0.55, 0.12}, ramps={0.0, 0.0}})
      -- hanging fringe on the bough
      local nf = math.floor(Ls / 3.2)
      for k = 1, nf do
        local u = rand(0.3, 1.0)
        local fx = tx + s*Ls*u*0.95
        local fy = y + droop*(0.25 + 0.7*u)
        local fl = rand(0.18, 0.4) * Ls
        spB:load(pile_, 0.8)
        spB:stroke({{fx, fy}, {fx + s*fl*0.18, fy + fl*0.5}, {fx + s*fl*0.28, fy + fl}}, {pressure={0.38, 0.1}, ramps={0.0, 0.0}})
      end
    end
  end
end
spruce(892, crestY(892)+2, 88, 40, 0.01, pineD)
print(wait(0))

--@ chunk 258
function spruceMask(x, yb, h, w, lean)
  lean = lean or 0
  local ytop = yb - h
  local n = math.max(8, math.floor(h/4.2))
  local s = h / n
  local function tx(y) return x + lean*(yb - y) end
  local left, right = {}, {}
  left[#left+1] = {tx(ytop), ytop - 1}
  for i = 1, n do
    local t = i / n
    local yn = ytop + s*(i-1) + s*0.25
    local L = (w/2) * (0.10 + 0.90*t^0.85)
    local xn = tx(yn) - L*0.38*rand(0.8,1.2)
    local yt = yn + s*rand(1.3, 1.8)
    local xt = tx(yt) - L*rand(0.85, 1.12)
    left[#left+1] = {xn, yn}
    left[#left+1] = {xt, yt}
  end
  right[#right+1] = {tx(ytop), ytop - 1}
  for i = 1, n do
    local t = i / n
    local yn = ytop + s*(i-1) + s*0.25
    local L = (w/2) * (0.10 + 0.90*t^0.85)
    local xn = tx(yn) + L*0.38*rand(0.8,1.2)
    local yt = yn + s*rand(1.3, 1.8)
    local xt = tx(yt) + L*rand(0.85, 1.12)
    right[#right+1] = {xn, yn}
    right[#right+1] = {xt, yt}
  end
  local p = {}
  for _, q in ipairs(left) do p[#p+1] = q end
  -- base across the bottom
  p[#p+1] = {left[#left][1] + 3, yb + 1.5}
  p[#p+1] = {right[#right][1] - 3, yb + 1.5}
  for i = #right, 1, -1 do p[#p+1] = right[i] end
  return poly(p)
end
function spruceFull(x, yb, h, w, lean, pile_)
  local m = spruceMask(x, yb, h, w, lean)
  work(m, {hand="detail", pile=pile_, coverage=4.0, clip=true, angle=1.5708, length={3,8}, pressure={0.5,0.9}})
  -- the trunk and leader
  spB:load(pile_, 1.0)
  spB:stroke({{x, yb+1.5}, {x + lean*h*0.4, yb - h*0.45}, {x + lean*h, yb - h - 3}}, {pressure={0.55, 0.15}, ramps={0.0, 0.0}})
  -- hanging fringe along the outer tips (thin dark strokes drooping from the tier ends)
  local ytop = yb - h
  local n = math.max(8, math.floor(h/4.2))
  local s = h / n
  for i = 2, n do
    local t = i / n
    local y = ytop + s*(i-1) + s*1.4
    local L = (w/2) * (0.10 + 0.90*t^0.85)
    for _, sd in ipairs({-1, 1}) do
      local fx = x + lean*(yb - y) + sd*L*rand(0.7, 1.1)
      spB:load(pile_, 0.7)
      spB:stroke({{fx, y - s*0.4}, {fx + sd*s*0.2, y + s*0.4}, {fx + sd*s*0.4, y + s*1.1}}, {pressure={0.42, 0.1}, ramps={0.0, 0.0}})
    end
  end
end
-- cover the stiff first attempt, then draw the real thing
spruceFull(892, crestY(892)+2, 88, 40, 0.015, pineD)
print(wait(0))

--@ chunk 259
spruceFull(944, crestY(944)+2, 60, 28, -0.02, pineD)
spruceFull(842, crestY(842)+2, 34, 17, 0.03, pineD)
print(wait(0))

--@ chunk 260
brd = brush{kind="round", width=1.8, point=1}
print("brd widths", brd:mark_width(0.1), brd:mark_width(0.3), brd:mark_width(0.5), brd:mark_width(0.8))
BirdP = pile{{"raw umber",2},{"Prussian blue",0.8},{"bone black",1.5}}
function bird(cx, cy, w, tilt, flap)
  tilt = tilt or 0
  flap = flap or 1
  local s = w / 10
  local function P(dx, dy)
    local c, sn = math.cos(tilt), math.sin(tilt)
    return {cx + (dx*c - dy*sn)*s, cy + (dx*sn + dy*c)*s}
  end
  local pts = {P(-5.2, 0.3*flap + 0.8), P(-3.4, -1.7*flap), P(-1.4, -2.0*flap), P(0, 0.2), P(1.5, -1.9*flap), P(3.4, -1.5*flap), P(5.0, 0.6*flap + 0.9)}
  brd:load(BirdP, 0.8)
  brd:stroke(pts, {pressure={0.22, 0.22}, swell={0.6, 1.0, 1.5, 1.7, 1.5, 1.0, 0.6}, ramps={0.0, 0.0}})
end
bird(526, 206, 11, -0.08, 1.0)
bird(554, 191, 9, 0.05, 0.8)
bird(580, 214, 10, -0.12, 1.1)
bird(606, 197, 6.5, 0.1, 0.9)
bird(636, 228, 6, -0.05, 0.7)
bird(478, 150, 7, 0.08, 1.0)
bird(611, 152, 5, -0.1, 0.9)
print(wait(0))

--@ chunk 261
-- fix the white scribble at the foot of the forested slope (right side), then a wavy soft fog line beneath it
fixM = rect(852, 446, 84, 40) * bigM
dabwork(fixM, ScMid, 5.0)
dabwork(fixM * mask(function(x,y) return smoothstep(862, 890, x) end), ScDark, 3.6)
fogLine = mask(function(x,y)
  local t = 480 + 5*fogwisp(x*1.3, 55) + 3*fogwisp(x*0.6, 12)
  return smoothstep(t-5, t+10, y) * sb(x, 836, 868, 950, 990)
end) * rect(830, 460, 170, 70) - fgM
work(fogLine, {hand="body", tool="filbert 16", pile=PW2b, coverage=4.0, angle=0.0, edge="lost", fill=true, length={40,110}, angle_jitter=0.04})
print(wait(0))

--@ chunk 262
softFix = mask(function(x,y)
  return sb(x, 826, 850, 936, 975) * sb(y, 430, 448, 478, 492)
end) * bigM
dabwork(softFix, ScDark, 5.0)
dabwork(softFix * mask(function(x,y) return 0.5 + 0.5*cn1(x*1.3, y*1.3) end), ScMid, 2.4, "filbert 6")
-- re-fog the foot a little with a wavy lost edge, below the forest line
fogLine2 = mask(function(x,y)
  local t = 486 + 5*fogwisp(x*1.3, 75) + 3*fogwisp(x*0.6, 22)
  return smoothstep(t-4, t+10, y) * sb(x, 826, 860, 950, 990)
end) * rect(820, 470, 190, 70) - fgM
work(fogLine2, {hand="body", tool="filbert 16", pile=PW2b, coverage=4.0, angle=0.0, edge="lost", fill=true, length={40,110}, angle_jitter=0.04})
print(wait(0))

--@ chunk 263
-- ===== cliff: cool unifying glaze + irregular crevices and bedding =====
cliffG = cliffAll:grow(0.4)
HazeCl = pile{{"smalt",1.6},{"raw umber",0.9},{"Prussian blue",0.18},{"red earth",0.3},{"lead white",0.5}, medium=0.85}
work(cliffG, {hand="glaze", pile=HazeCl, angle=1.5708, coverage=2.0, clip=true,
  load_at=function(x,y) return clamp(0.55 + 0.45*smoothstep(760, 860, x) + 0.2*smoothstep(380, 430, y), 0, 1) end})
-- irregular vertical joints (hand-placed, differing widths/lengths; each stops at a bedding plane)
function joint(x0, y0, y1, w, wob, seed)
  local pts, ws = {}, {}
  local n = math.max(3, math.floor((y1-y0)/9))
  for i = 0, n do
    local t = i/n
    pts[#pts+1] = {x0 + wob*math.sin(i*1.9 + seed) + rand(-0.5,0.5), y0 + (y1-y0)*t}
    ws[#ws+1] = w*(0.6 + 0.7*math.sin(3.14*t)) 
  end
  return ribbon(pts, ws)
end
joints = joint(753, 344, 392, 1.8, 1.0, 1) + joint(764, 372, 412, 1.4, 0.8, 2) + joint(776, 340, 380, 2.2, 1.0, 3)
       + joint(786, 384, 414, 1.6, 0.8, 4) + joint(801, 336, 372, 1.5, 0.8, 5) + joint(815, 350, 404, 2.6, 1.2, 6)
       + joint(829, 338, 384, 1.8, 1.0, 7) + joint(842, 372, 412, 2.0, 0.9, 8) + joint(854, 340, 378, 2.4, 1.0, 9)
       + joint(862, 380, 412, 1.4, 0.8, 10) + joint(808, 392, 414, 1.4, 0.8, 11)
JointD = pile{{"raw umber",1.4},{"Prussian blue",0.4},{"smalt",0.7},{"bone black",0.5}}
work(joints * cliffG, {hand="body", tool="filbert 3", pile=JointD, angle=1.5708, coverage=3.0, clip=true, length={8,24}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.5 + 0.5*smoothstep(780, 850, x), 0, 1) end})
-- bedding planes: short, slightly tilted dashes of dark with a warm lip beneath on the lit left
function bed(y0, x0, x1, tilt)
  local pts = {}
  local n = math.max(3, math.floor((x1-x0)/14))
  for i = 0, n do
    local t = i/n
    pts[#pts+1] = {x0 + (x1-x0)*t, y0 + tilt*t + rand(-0.9,0.9)}
  end
  return ribbon(pts, 1.1)
end
beds = bed(358, 748, 800, 1.5) + bed(364, 818, 868, -1.0) + bed(381, 744, 792, 1.0) + bed(386, 806, 872, 1.5)
     + bed(399, 744, 780, 0.5) + bed(402, 796, 874, 1.0) + bed(414, 742, 870, 0.0)
work(beds * cliffG, {hand="detail", pile=JointD, coverage=1.1, clip=true, angle=0.03, length={6,18}, pressure={0.2,0.4}, load=0.6})
print(wait(0))

--@ chunk 264
-- re-model with soft light: warm on the left, deep cool shadow on the right, a pale lit pillar in the middle
BCliffW = pile{{"red earth",0.7},{"yellow ochre",0.5},{"lead white",1.7},{"vermilion",0.25},{"smalt",0.3}}
BCliffP = pile{{"smalt",1.2},{"lead white",2.0},{"red earth",0.45},{"yellow ochre",0.25},{"raw umber",0.2}}
BCliffDD = pile{{"smalt",1.3},{"raw umber",1.2},{"Prussian blue",0.35},{"lead white",0.35},{"red earth",0.25}}
irr = noise{seed=301, period=26, octaves=2, persistence=0.5, stretch={1.5708, 3.0}}
-- warm lit mass on the left, with an irregular right edge following the noise
litMass = cliffG * mask(function(x,y)
  local edge = 786 + 12*irr(x, y) + 6*math.sin(y/23)
  return (1 - smoothstep(edge-9, edge+9, x)) * (1 - smoothstep(344, 440, y)*0.85)
end)
work(litMass, {hand="body", tool="filbert 6", pile=BCliffW, angle=vert3, coverage=3.4, clip=true, fill=false, length={14,50}, angle_jitter=0.07})
-- pale lit pillar, centre-left
palM = cliffG * mask(function(x,y)
  local c = 806 + 6*irr(x*0.6, y)
  return (1 - smoothstep(9, 15, math.abs(x - c))) * (1 - smoothstep(350, 430, y)*0.9)
end)
work(palM, {hand="body", tool="filbert 5", pile=BCliffP, angle=vert3, coverage=2.8, clip=true, length={12,40}, angle_jitter=0.07})
-- deep shadow right of x~835, with irregular left edge
shMass = cliffG * mask(function(x,y)
  local edge = 836 + 9*irr(x+50, y)
  return smoothstep(edge-9, edge+9, x)
end)
work(shMass, {hand="body", tool="filbert 6", pile=BCliffDD, angle=vert3, coverage=3.4, clip=true, length={14,50}, angle_jitter=0.07,
  load_at=function(x,y) return clamp(0.85, 0, 1) end})
-- a little warm kiss on the top edges of the shadow masses (afterglow on the upper surfaces)
topLip = cliffG * mask(function(x,y) return (1 - smoothstep(330, 350, y)) * smoothstep(826, 840, x) end)
work(topLip, {hand="body", tool="filbert 4", pile=BCliffW, angle=0.0, coverage=1.8, clip=true, length={6,20}, load=0.6})
print(wait(0))

--@ chunk 265
-- ===== foreground: rim-lit grass on the dark slope, fine and varied =====
GrassTan  = pile{{"raw umber",1.2},{"yellow ochre",1.4},{"lead white",1.4},{"red earth",0.3}}
GrassGold = pile{{"yellow ochre",1.6},{"lead white",1.0},{"chrome yellow",0.35},{"red earth",0.25},{"raw umber",0.5}}
GrassOliv = pile{{"raw umber",1.6},{"green earth",1.0},{"yellow ochre",0.8},{"lead white",0.5}}
GrassDeep = pile{{"raw umber",3},{"green earth",0.8},{"bone black",0.7},{"yellow ochre",0.3}}
function lit_blade(bx, by, h, lean, pile_, wmax, load)
  local n = 5
  local pts = {}
  for i = 0, n do
    local t = i/n
    local bend = lean * (0.25 + t*t*1.6)
    pts[#pts+1] = {bx + math.sin(bend) * h * t, by - math.cos(bend) * h * t}
  end
  g3:load(pile_, load or 0.8)
  g3:stroke(pts, {pressure={wmax or 0.5, 0.06}, ramps={0.0, 0.0}})
end
local nb = 0
for i = 1, 760 do
  local x = rand(4, 996)
  local d = (rand(0,1)^1.7) * 62 + 1.5
  local y = crestY(x) + d
  local skip = false
  if footM:at(x, y) > 0.05 or figM:at(x, y) > 0.05 then skip = true end
  if x > 396 and x < 446 and y > 536 and y < 556 then skip = true end
  if not skip then
    local near = 1 - smoothstep(0, 60, d)       -- 1 near the crest, 0 further down
    local r = math.random()
    local p
    if r < 0.45 * (0.4 + 0.6*near) then p = GrassTan
    elseif r < 0.62 * (0.4 + 0.6*near) then p = GrassGold
    elseif r < 0.82 then p = GrassOliv
    else p = GrassDeep end
    local h = rand(5, 15) * (0.8 + 0.5*(1-near))
    local side = (math.random() < 0.5) and -1 or 1
    lit_blade(x, y, h, side*rand(0.08, 0.65), p, rand(0.4, 0.62), 0.55 + 0.35*near)
    nb = nb + 1
  end
end
print("blades", nb)
print(wait(0))

--@ chunk 266
GBdeep = pile{{"cobalt blue",2.0},{"smalt",2.4},{"Prussian blue",0.18},{"lead white",0.1}, medium=0.8}
vigS = function(x,y) return (1 - smoothstep(0, 235, y))^1.15 * (0.78 + 0.22*math.abs(x-560)/560) end
work(skyFull, {hand="glaze", pile=GBdeep, angle=0.0, coverage=2.0, clip=true, angle_jitter=0.03,
  load_at=function(x,y) return clamp(vigS(x,y), 0, 1) end})
blend(skyFull * rect(0,0,1000,260), {angle=0.0})
blend(skyFull * rect(0,0,1000,260), {angle=0.0})
print(wait(0))

--@ chunk 267
print(drying(700,60), drying(700,200), drying(800,320), drying(490,395), drying(300,380), drying(250,250))

--@ chunk 268
lf = brush("filbert", 26)
for i = 0, 3 do
  lf:wipe(1.0)
  lf:stroke({{790, 12 + i*22}, {985, 12 + i*22 + 2}}, {pressure={0.55,0.55}, ramps={0.05,0.05}})
end
print(wait(0))

--@ chunk 269
cliffProt = (cliffAll + needleM + towerK + step):grow(2.5)
liftM = skyFull - cliffProt
lifter = brush("filbert", 40)
local nStr = 0
local y = -6
local row = 0
while y < 418 do
  local x = -70
  local dir = (row % 2 == 0) and 1 or -1
  local segs = {}
  while x < 1070 do segs[#segs+1] = x; x = x + 150 end
  if dir < 0 then local r = {}; for i = #segs, 1, -1 do r[#r+1] = segs[i] end; segs = r end
  for _, sx in ipairs(segs) do
    lifter:wipe(1.0)
    local ex = sx + dir*185
    lifter:stroke({{sx, y}, {(sx+ex)/2, y + rand(-1.2,1.2)}, {ex, y + rand(-1.2,1.2)}}, {pressure={0.5,0.5}, ramps={0.12,0.12}, clip=liftM})
    nStr = nStr + 1
  end
  y = y + 22
  row = row + 1
end
print("lift strokes", nStr)
print(wait(0))

--@ chunk 270
print(wait(14*24*60))
print(drying(700,60), drying(700,200), drying(800,320), drying(490,395), drying(250,250), drying(800,380), drying(300,380))

--@ chunk 271
limbNames = {"trunk","L2","L1","L4","L5","R1","R1a","R1b","R1a2","R1b1","R1b2","R1b3","L2r1","L2r2","stub","S2","S4","S6","S7","S8","S9","S14","S15"}
limbsNow = nil
for _, nm in ipairs(limbNames) do
  local L = OAK[nm]
  local m = body_of{spine=L[1], widths=L[2], blend=0.8, char="firm", amount=0.0}:mask()
  limbsNow = limbsNow and (limbsNow + m) or m
end
limbsKeep = limbsNow:grow(0.8)
aboveRidge = mask(function(x,y) return (y <= Atop(x) + 0.5) and 1 or 0 end)
butteZone = (buM + ext1 + ext2 + crownM + needleM + towerK + step):grow(2) * rect(0, 0, 1000, 414)
skyCoatM2 = (aboveRidge + butteZone) - limbsKeep
print("areas", limbsKeep:area(), butteZone:area(), skyCoatM2:area())

--@ chunk 272
function spc(p, f, cov)
  work(skyCoatM2, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
spc(T0,  function(x,y) return clamp((1 - smoothstep(10,130,y)) * 0.95,0,1) end, 3.8)
spc(T1,  function(x,y) return clamp(bump(y,85,70),0,1) end, 3.6)
spc(T2n, function(x,y) return clamp(bump(y,170,62),0,1) end, 3.8)
spc(T3n, function(x,y) return clamp(bump(y,240,46),0,1) end, 3.8)
spc(T4,  function(x,y) return clamp(bump(y,284,38)*(0.55+0.45*bump(x,640,520)),0,1) end, 3.4)
blend(skyCoatM2, {angle=0.0})
blend(skyCoatM2, {angle=0.0})
print(wait(0))

--@ chunk 273
function spc2(p, f, cov)
  work(skyCoatM2, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.035,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
spc2(T5c, function(x,y) return smoothstep(296,344,y) * (0.8+0.2*bump(x,640,500)) end, 3.4)
spc2(T6c, function(x,y) return (0.2 + 0.8*bump(x,640,340)) * smoothstep(336,398,y) end, 3.4)
spc2(T7c, function(x,y) return 0.95*bump(x,650,300) * smoothstep(360,408,y)^1.1 end, 3.2)
spc2(T9c, function(x,y) return bump(x,640,150) * smoothstep(354,410,y)^1.2 * 0.95 end, 3.0)
blend(skyCoatM2, {angle=0.0})
blend(skyCoatM2, {angle=0.02})
blend(skyCoatM2, {angle=-0.02})
print(wait(0))

--@ chunk 274
print(wait(10*24*60))
print(drying(300,50), drying(600,150), drying(300,250), drying(500,380), drying(800,380))

--@ chunk 275
spc(T0,  function(x,y) return clamp((1 - smoothstep(10,130,y)) * 0.95,0,1) end, 3.6)
spc(T1,  function(x,y) return clamp(bump(y,85,70),0,1) end, 3.6)
spc(T2n, function(x,y) return clamp(bump(y,170,62),0,1) end, 4.2)
spc(T3n, function(x,y) return clamp(bump(y,240,46),0,1) end, 4.2)
spc(T4,  function(x,y) return clamp(bump(y,284,38)*(0.55+0.45*bump(x,640,520)),0,1) end, 3.8)
blend(skyCoatM2, {angle=0.0})
blend(skyCoatM2, {angle=0.0})
print(wait(0))

--@ chunk 276
print(wait(12*24*60))
print(drying(300,50), drying(600,150), drying(300,250), drying(500,380), drying(800,380))

--@ chunk 277
-- ===== restore ridge A on the right of the butte, soft join =====
ridgeAR = mask(function(x,y)
  local t = Atop(x)
  if x < 846 or x > 1010 then return 0 end
  return ((y >= t - 0.4 and y <= t + 64) and 1 or 0) * smoothstep(846, 880, x)
end) - fgM - limbsKeep
work(ridgeAR, {hand="body", tool="filbert 12", pile=PAc, angle=0.02, coverage=4.0, clip=true, fill=true, length={40,110}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.97*(1 - smoothstep(Atop(x)+1, Atop(x)+50, y)), 0, 1)^0.85 end})
work(ridgeAR, {hand="body", tool="filbert 12", pile=PAw, angle=0.02, coverage=3.0, clip=true, fill=true, length={40,110}, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.9*bump(x,660,330)*(1 - smoothstep(Atop(x)+1, Atop(x)+46, y)), 0, 1)^0.85 end})
-- the fog front of the right valley beneath it, restored where the strip is cut
fogRfix = mask(function(x,y)
  if x < 846 or x > 1010 then return 0 end
  return smoothstep(846, 880, x) * smoothstep(Atop(x)+24, Atop(x)+62, y) * (y < 520 and 1 or 0)
end) - fgM
work(fogRfix, {hand="body", tool="filbert 18", pile=PW2b, angle=0.0, coverage=4.4, clip=true, fill=true, length={60,160}, angle_jitter=0.04})
print(wait(0))

--@ chunk 278
-- ===== THE BUTTE, v7: drawn afresh, one wet-in-wet session =====
cliffPts = {{741,436},{742,418},{740,406},{743,394},{741,382},{744,370},{741,358},{745,348},{748,340},{750,326},{753,319},{760,317},{764,322},{765,333},{769,336},{772,331},
            {779,330},{787,327},{796,329},{804,331},{811,330},{818,333},{824,330},{827,323},{834,321},{840,324},{842,329},{846,327},{853,329},{859,333},{864,336},{868,339},
            {867,348},{871,357},{869,367},{872,377},{870,388},{874,398},{872,410},{875,421},{874,436}}
cliffM = poly(cliffPts):roughen(0.9, 10, 12)
print("cliff area", cliffM:area())
vertw = function(x,y) return 1.5708 + 0.05*math.sin(x/9 + y/37) end
-- 1. base: mid violet-blue, whole cliff
work(cliffM, {hand="body", tool="filbert 9", pile=BCliff, angle=vertw, coverage=6.0, clip=true, fill=true, length={24,70}, angle_jitter=0.06})
-- 2. shadow side with an irregular boundary (noise), deepest at the far right
shEdge = function(x,y) return 826 + 12*irr(x+30, y) + 5*math.sin(y/17) end
shadeM = cliffM * mask(function(x,y) return smoothstep(shEdge(x,y)-10, shEdge(x,y)+10, x) end)
work(shadeM, {hand="body", tool="filbert 8", pile=BCliffDD, angle=vertw, coverage=4.2, clip=true, fill=true, length={22,60}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.55 + 0.45*smoothstep(shEdge(x,y), 870, x), 0, 1) end})
-- 3. lit side: warm, irregular right boundary, stronger high up
litEdge = function(x,y) return 782 + 10*irr(x, y) + 5*math.sin(y/21 + 1) end
litM2 = cliffM * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-9, litEdge(x,y)+9, x)) end)
work(litM2, {hand="body", tool="filbert 7", pile=BCliffW, angle=vertw, coverage=3.8, clip=true, fill=true, length={18,55}, angle_jitter=0.07,
  load_at=function(x,y) return clamp(0.95 * (1 - 0.55*smoothstep(340, 430, y)), 0, 1) end})
-- 4. a pale lit pillar in the middle-left, a second lit rib on the shadow side
rib1 = cliffM * mask(function(x,y) local c = 800 + 5*irr(x*0.6, y); return (1 - smoothstep(6, 12, math.abs(x - c))) * (1 - 0.7*smoothstep(345, 428, y)) end)
work(rib1, {hand="body", tool="filbert 5", pile=BCliffP, angle=vertw, coverage=2.6, clip=true, length={12,40}, angle_jitter=0.07})
rib2 = cliffM * mask(function(x,y) local c = 846 + 4*irr(x*0.6+40, y); return (1 - smoothstep(3, 7, math.abs(x - c))) * (1 - 0.8*smoothstep(335, 400, y)) * 0.8 end)
work(rib2, {hand="body", tool="filbert 4", pile=BCliffW, angle=vertw, coverage=2.0, clip=true, length={10,30}, angle_jitter=0.07})
print(wait(0))

--@ chunk 279
function cliffTop(x)
  for y = 300, 440, 0.5 do
    if cliffM:at(x, y) > 0.5 then return y end
  end
  return 440
end
-- forest on the plateau: a dark ragged band with taller conifers and round oak crowns, sparse on the pinnacle
Crown = pile{{"raw umber",1.0},{"Prussian blue",0.5},{"bone black",0.5},{"green earth",0.3},{"smalt",0.4}}
CrownL = pile{{"raw umber",1.2},{"yellow ochre",0.5},{"green earth",0.4},{"red earth",0.3},{"lead white",0.4}}
local xs = {}
local x = 768
while x < 868 do xs[#xs+1] = x; x = x + rand(2.0, 4.2) end
for i, xx in ipairs(xs) do
  local yt = cliffTop(xx)
  local r = math.random()
  local m
  if r < 0.40 then
    local h = rand(6, 15) * ((i % 5 == 0) and 1.3 or 1)
    m = tinyPine(xx, yt + 2.5, h, h*rand(0.38, 0.5))
  elseif r < 0.8 then
    local rr = rand(2.6, 4.6)
    m = ellipse(xx, yt - rr*0.35, rr, rr*rand(0.7, 0.95))
  else
    local h = rand(4, 8)
    m = tinyPine(xx, yt + 2.5, h, h*0.5)
  end
  work(m, {hand="detail", pile=Crown, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
end
-- a few trees on the pinnacle too
for _, p in ipairs({{754,4.0},{758,6.0},{761,3.5}}) do
  local yt = cliffTop(p[1])
  work(ellipse(p[1], yt - 1, p[2]*0.6, p[2]*0.8), {hand="detail", pile=Crown, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
end
-- dark band just below the top edge so the forest sits on the cliff
band = cliffM * mask(function(x,y) local t = cliffTop(x); return (x > 766 and x < 866) and bump(y, t+2.5, 3.2) or 0 end)
work(band, {hand="body", tool="filbert 4", pile=Crown, angle=0.0, coverage=2.6, clip=true, length={6,16}, angle_jitter=0.3, load=0.8})
print(wait(0))

--@ chunk 280
tl2 = function(x) return 421 + 7*trn(x*1.3, 9) + 2.5*math.sin(x/6.5) end
footForest = mask(function(x,y)
  if x < 734 or x > 884 then return 0 end
  local t = tl2(x)
  return smoothstep(t-0.8, t+0.8, y) * (1 - smoothstep(448, 462, y)) * sb(x, 734, 742, 874, 884)
end)
dabwork(footForest, ScDark, 5.5, "filbert 6")
-- lit crowns along the sun side and some mid-tone crowns for variety
work(footForest * mask(function(x,y) return (1 - smoothstep(740, 800, x)) * (0.4 + 0.6*cn1(x+20,y)) end),
  {hand="body", tool="filbert 5", pile=ScLit, angle=1.4, coverage=1.8, clip=true, length={3,9}, angle_jitter=0.7})
work(footForest * mask(function(x,y) return (0.25 + 0.75*cn2(x*1.1,y)) * smoothstep(780, 860, x) end),
  {hand="body", tool="filbert 5", pile=ScMid, angle=1.4, coverage=1.4, clip=true, length={3,9}, angle_jitter=0.7})
-- single conifer spikes pricking up into the cliff foot
for _, xx in ipairs({748, 757, 769, 788, 801, 822, 836, 851, 863}) do
  local yb = tl2(xx) + 5
  local h = rand(8, 15)
  work(tinyPine(xx + rand(-1.5,1.5), yb, h, h*0.45), {hand="detail", pile=ScDark, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 281
TWIGS = {}
-- re-define the shoot generators so every polyline is recorded
function oakshoot3(x, y, ang, len, w0, depth, opts)
  opts = opts or {}
  local pts = {{x, y}}
  local angs = {}
  local cx, cy, a = x, y, ang
  local used = 0
  local kink = opts.kink or 0.5
  while used < len do
    local l = math.min(rand(opts.lmin or 8, opts.lmax or 17), len - used)
    if l < 3.0 then break end
    a = a + rand(-kink, kink) - (opts.up or 0.04) * math.sin(a)
    cx = cx + math.cos(a) * l
    cy = cy + math.sin(a) * l
    used = used + l
    pts[#pts+1] = {cx, cy}
    angs[#angs+1] = a
  end
  if #pts < 2 then return end
  local w1 = math.max(0.87, w0 * 0.3)
  tw:load(OakT, 1.0)
  tw:stroke(pts, {pressure={tw:pressure_for(math.max(w0, 0.9)), tw:pressure_for(w1)}, ramps={0.0, 0.0}})
  TWIGS[#TWIGS+1] = pts
  if depth > 0 then
    local total = used
    local d = 0
    for i = 2, #pts do
      d = d + math.sqrt((pts[i][1]-pts[i-1][1])^2 + (pts[i][2]-pts[i-1][2])^2)
      if math.random() < (opts.p or 0.6) then
        local side = (math.random() < 0.5) and 1 or -1
        local sa = angs[i-1] + side * rand(0.4, 0.95)
        local rem = total - d
        local sl = math.max(7, (rem + rand(5, 16)) * rand(0.45, 0.85))
        oakshoot3(pts[i][1], pts[i][2], sa, sl, math.max(0.9, w0 * 0.75), depth - 1, opts)
      end
    end
  end
end
function sprout4(name, from, to, spacing, wcap, depth, lenmin, lenmax, opts)
  local L = OAK[name]
  local pts, wd = L[1], L[2]
  local s = {0}
  for i = 2, #pts do
    s[i] = s[i-1] + math.sqrt((pts[i][1]-pts[i-1][1])^2 + (pts[i][2]-pts[i-1][2])^2)
  end
  local total = s[#pts]
  local pos = total * from
  local flip = (math.random() < 0.5) and 1 or -1
  while pos < total * to do
    local i = 1
    while i < #pts-1 and s[i+1] < pos do i = i + 1 end
    local t = (pos - s[i]) / (s[i+1] - s[i])
    local px = pts[i][1] + t * (pts[i+1][1]-pts[i][1])
    local py = pts[i][2] + t * (pts[i+1][2]-pts[i][2])
    local la = math.atan(pts[i+1][2]-pts[i][2], pts[i+1][1]-pts[i][1])
    local wl = wd[i] + t * (wd[i+1]-wd[i])
    local sa = la + flip * rand(0.35, 0.85)
    flip = -flip
    oakshoot3(px, py, sa, rand(lenmin, lenmax), clamp(wl * 0.42, 0.9, wcap), depth, opts)
    pos = pos + spacing * rand(0.7, 1.3)
  end
  -- terminal fan at the limb's tip
  local n = #pts
  local la = math.atan(pts[n][2]-pts[n-1][2], pts[n][1]-pts[n-1][1])
  for k = 1, 3 do
    oakshoot3(pts[n][1], pts[n][2], la + (k-2) * rand(0.3, 0.55), rand(lenmin*0.8, lenmax), clamp(wd[n]*0.9, 0.9, wcap), depth, opts)
  end
end
-- first generation: from the big limbs
for _, nm in ipairs({"L2","L1","L4","L5"}) do
  sprout4(nm, 0.45, 1.0, 17, 3.2, 2, 34, 66, {p=0.6})
end
for _, nm in ipairs({"R1","R1a","R1b"}) do
  sprout4(nm, 0.45, 1.0, 17, 3.4, 2, 34, 70, {p=0.6})
end
print("twigs so far", #TWIGS)
print(wait(0))

--@ chunk 282
for _, nm in ipairs({"S2","S4","S6","S7","S8","S9","S14","S15","R1a2","R1b1","R1b2","R1b3","L2r1","L2r2","stub"}) do
  sprout4(nm, 0.2, 1.0, 14, 2.8, 2, 26, 54, {p=0.6})
end
print("twigs", #TWIGS)
print(wait(0))

--@ chunk 283
-- ===== the butte's foot dissolves into the fog: soft weights, no hard edges =====
zoneF = rect(630, 420, 340, 140) - fgM - cliffM:grow(1.5)
Fw = function(x,y)
  local fy = smoothstep(452, 494, y)
  local fx = smoothstep(864, 904, x) * smoothstep(424, 450, y)
  local f = math.max(fy, fx)
  return f * (1 - smoothstep(522, 552, y))
end
foot2 = mask(function(x,y) return Fw(x,y) end) * zoneF
work(foot2, {hand="body", tool="filbert 20", pile=PW2b, angle=0.0, coverage=4.6, clip=true, fill=true, length={60,150}, angle_jitter=0.04})
work(foot2, {hand="body", tool="filbert 20", pile=PW2b, angle=0.0, coverage=3.0, clip=true, fill=true, length={60,150}, angle_jitter=0.04})
print(wait(0))

--@ chunk 284
print(wait(6*24*60))
print(drying(700,500), drying(800,470), drying(300,300), drying(300,560))

--@ chunk 285
-- ===== FOG SEA, right half: one wet-in-wet coat =====
cliffKeep = rect(730, 380, 160, 62)
spr1 = poly({{892,583},{917,686},{867,686}})
spr2m = poly({{944,642},{960,712},{928,712}})
spr3 = poly({{842,630},{854,672},{830,672}})
fogReg = mask(function(x,y)
  if x < 606 then return 0 end
  if y > crestY(x) - 1.5 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - cliffKeep - spr1 - spr2m - spr3
print("fog area", fogReg:area())
PW3c = pile{{"lead white",6.5},{"smalt",0.8},{"red earth",0.3},{"yellow ochre",0.05}}
PWpk = pile{{"lead white",6},{"yellow ochre",0.3},{"vermilion",0.14},{"chrome yellow",0.14}}
function fg_(p, f, cov)
  work(fogReg, {hand="body", tool="filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
fg_(PW2b, function(x,y) return 1 end, 4.4)
fg_(PW3c, function(x,y) return smoothstep(500, 690, y)^1.1 * 0.95 end, 3.4)
fg_(PW2b, function(x,y) return bump(y, 505, 85) end, 3.0)
fg_(PW1b, function(x,y) return (1 - smoothstep(412, 525, y)) * (0.45 + 0.55*bump(x, 700, 380)) end, 3.2)
fg_(PWpk, function(x,y) return bump(x,650,260) * (1 - smoothstep(Atop(x)+2, Atop(x)+70, y)) * 0.8 end, 2.6)
blend(fogReg, {angle=0.0})
blend(fogReg, {angle=0.0})
blend(fogReg, {angle=0.03})
blend(fogReg, {angle=1.5708})
blend(fogReg, {angle=0.0})
print(wait(0))

--@ chunk 286
-- ===== VALLEY, right half, session 2 (everything wet-in-wet, soft edges by edge=) =====
xbL = function(y) return 598 + 9*math.sin(y/37) + 5*math.sin(y/13 + 1) end
Rv = mask(function(x,y)
  if x < xbL(y) then return 0 end
  if y > crestY(x) - 1.5 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - cliffM:grow(1.5) - spr1 - spr2m - spr3
edgeL = function(x,y) return 0.12 + 0.85*(1 - smoothstep(606, 690, x)) end
PW4 = pile{{"lead white",4.2},{"smalt",1.5},{"red earth",0.45},{"raw umber",0.12}}
PW5 = pile{{"lead white",3.4},{"smalt",1.9},{"red earth",0.5},{"raw umber",0.2}}
-- 1. cream base (seam cover)
work(Rv, {hand="body", tool="filbert 22", pile=PW2b, angle=0.0, coverage=3.2, fill=true, length={80,220}, angle_jitter=0.04, edge=edgeL})
print(wait(0))

--@ chunk 287
-- 2. far ridge A: pale lilac strip, crisp top, dissolving underside, left end lost
stripA = Rv * mask(function(x,y) return (y <= Atop(x) + 40) and 1 or 0 end)
edgeA = function(x,y) return math.max(0.06 + 0.9*smoothstep(Atop(x)+8, Atop(x)+34, y), edgeL(x,y)) end
work(stripA, {hand="body", tool="filbert 14", pile=PAc, angle=0.02, coverage=3.4, fill=true, length={50,130}, angle_jitter=0.05, edge=edgeA,
  load_at=function(x,y) return clamp(0.95*(1 - 0.5*smoothstep(Atop(x)+4, Atop(x)+34, y)), 0, 1) end})
-- 3. low hills to the right of the butte, hazy lilac
humpTop = {{860,462},{890,452},{925,444},{960,438},{1010,436}}
humpM = Rv * mask(function(x,y)
  if x < 852 then return 0 end
  local t = interp(humpTop, x) + 1.5*trn(x*1.1, 5)
  return (y >= t and y <= t + 60) and 1 or 0
end)
edgeH = function(x,y) local t = interp(humpTop, x); return 0.06 + 0.9*smoothstep(t+8, t+44, y) end
work(humpM, {hand="body", tool="filbert 12", pile=PA6, angle=0.03, coverage=3.6, fill=true, length={40,110}, angle_jitter=0.06, edge=function(x,y) return math.max(edgeH(x,y), 0.85*(1 - smoothstep(852, 900, x))) end})
-- 4. lilac lower fog: deeper toward the crest; top boundary lost, bottom (crest) found
lowM = Rv * mask(function(x,y) return smoothstep(486, 530, y) end)
edgeLo = function(x,y) return 0.1 + 0.85*(1 - smoothstep(crestY(x)-120, crestY(x)-36, y)) end
work(lowM, {hand="body", tool="filbert 22", pile=PW4, angle=0.0, coverage=3.2, fill=true, length={80,220}, angle_jitter=0.04, edge=edgeLo,
  load_at=function(x,y) return clamp(0.35 + 0.65*smoothstep(520, crestY(x)-10, y), 0, 1) end})
work(lowM * mask(function(x,y) return smoothstep(crestY(x)-110, crestY(x)-30, y) end), {hand="body", tool="filbert 22", pile=PW5, angle=0.0, coverage=2.6, fill=true, length={80,220}, angle_jitter=0.04, edge=edgeLo,
  load_at=function(x,y) return clamp(0.8, 0, 1) end})
print(wait(0))

--@ chunk 288
print(wait(9*24*60))
print(drying(700,500), drying(800,470), drying(660,560), drying(950,420), drying(300,300))

--@ chunk 289
-- ===== RIGHT VALLEY, final coat: one wet-in-wet session, crisp where it meets solids, seam blended after =====
xbL2 = function(y) return 612 + 8*math.sin(y/37) + 4*math.sin(y/13 + 1) end
RvF = mask(function(x,y)
  if x < xbL2(y) then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - cliffM:grow(0.6) - crownM:grow(1.5)
print("RvF area", RvF:area())
function rvw(p, f, cov, tool)
  work(RvF, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
-- A. cool cream body everywhere
rvw(PW2b, function(x,y) return 1 end, 4.4)
-- B. pale lilac ridge A band at the top, crisp top edge, dissolving downward
rvw(PAc, function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+40, y)) end, 3.2, "filbert 14")
-- C. warm cream in the upper fog, centred under the glow
rvw(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+120, y)) * smoothstep(Atop(x)+10, Atop(x)+36, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 3.2)
-- D. peach breath at the horizon below the glow
rvw(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
-- E. lilac toward the viewer
rvw(PW3c, function(x,y) return smoothstep(500, 690, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * 0.95 end, 3.2)
rvw(PW4,  function(x,y) return smoothstep(560, 700, y) * smoothstep(640, 900, x) * 0.7 end, 2.4)
print(wait(0))

--@ chunk 290
seamZone = rect(556, 392, 110, 180) - fgM - limbsKeep:grow(2)
blendM = (RvF + seamZone) - cliffM:grow(0.6) - fgM
blend(blendM, {angle=0.0})
blend(blendM, {angle=0.0})
blend(blendM, {angle=0.03})
blend(blendM, {angle=1.5708})
blend(blendM, {angle=0.0})
print(wait(0))

--@ chunk 291
moundM = poly({{741,421},{733,428},{722,438},{709,450},{697,463},{690,478},{700,490},{760,494},{840,494},{900,490},{914,476},{906,460},{893,444},{882,432},{875,421}}, true):roughen(2.0, 16, 31)
print("mound", moundM:area())
dens = function(x,y)
  local base = 1 - smoothstep(438, 488, y)
  local sides = sb(x, 690, 735, 885, 915)
  return clamp(base * (0.35 + 0.65*sides) * (0.7 + 0.5*cn1(x*1.2, y*1.2)), 0, 1.6)
end
stipple(moundM, {pile=ScDark, width=3.4, coverage=function(x,y) return 2.6*dens(x,y) end, feather=0.5, clip=moundM,
  pressure={0.35,0.75}, dips={20,0.9,0.3}, cluster=0.25})
stipple(moundM, {pile=ScMid, width=3.0, coverage=function(x,y) return 1.5*dens(x,y)*smoothstep(450, 480, y)*0.0 + 1.6*dens(x,y)*smoothstep(444, 470, y) end, feather=0.5, clip=moundM,
  pressure={0.3,0.65}, dips={20,0.9,0.3}, cluster=0.25})
print(wait(0))

--@ chunk 292
moundM2 = poly({{741,421},{728,429},{712,439},{694,452},{672,467},{650,483},{640,500},{990,500},{976,482},{950,466},{921,450},{896,434},{876,421}}, true):roughen(2.4, 20, 41)
dens2 = function(x,y)
  local base = 1 - smoothstep(428, 492, y)
  -- distance from the cliff foot sides: the touches thin out with distance
  local dl = clamp((741 - x)/110, 0, 1)
  local dr = clamp((x - 876)/110, 0, 1)
  local side = 1 - math.max(dl, dr)
  return clamp(base * side^1.3 * (0.6 + 0.6*cn2(x*1.1, y*1.3)), 0, 1.4)
end
-- soft lilac crowns on the outer flanks (far, pale), then darker ones nearer the cliff
stipple(moundM2, {pile=ScFar, width=3.0, coverage=function(x,y) return 2.0*dens2(x,y) end, feather=0.6, clip=moundM2,
  pressure={0.3,0.65}, dips={20,0.9,0.3}, cluster=0.3})
stipple(moundM2, {pile=ScMid, width=3.2, coverage=function(x,y) return 1.6*dens2(x,y)*smoothstep(0.35, 0.8, dens2(x,y)) end, feather=0.5, clip=moundM2,
  pressure={0.3,0.7}, dips={20,0.9,0.3}, cluster=0.3})
-- ragged tops: forest climbing into the cliff foot at uneven heights
for i = 1, 26 do
  local xx = rand(745, 872)
  local yy = 421 - rand(0, 14) * (0.4 + 0.6*cn1(xx*1.7, 3)) 
  local r = rand(2.2, 4.4)
  if math.random() < 0.55 then
    work(ellipse(xx, yy, r, r*0.85), {hand="detail", pile=Crown, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
  else
    local h = rand(7, 15)
    work(tinyPine(xx, yy + 5, h, h*0.45), {hand="detail", pile=Crown, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
  end
end
print(wait(0))

--@ chunk 293
print(wait(10*24*60))
print(drying(700,500), drying(800,470), drying(660,560), drying(950,420), drying(800,440))

--@ chunk 294
-- ===== 1. forest mass of the butte's foot, over a clean cream ground =====
-- (a) cream coat over the stipple mess and the old crown blobs (mask excludes the cliff)
cleanM = (moundM2:grow(8) * rect(600, 380, 420, 150)) - cliffM:grow(0.4) - fgM
-- keep the original cliff-foot line: only fog below y=440 over the cliff zone
work(cleanM * mask(function(x,y) return 1 end), {hand="body", tool="filbert 22", pile=PW2b, angle=0.0, coverage=4.6, clip=true, fill=true, length={60,160}, angle_jitter=0.04})
print(wait(0))

--@ chunk 295
-- ===== RIGHT VALLEY, the fog coat (session 1 of 3): one wet-in-wet pass, two blends =====
RvG = mask(function(x,y)
  if x < xbL2(y) then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - cliffM:grow(0.6) - crownM:grow(1.5)
function rg(p, f, cov, tool)
  work(RvG, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
rg(PW2b, function(x,y) return 1 end, 5.0)
rg(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
rg(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+130, y)) * smoothstep(Atop(x)+10, Atop(x)+36, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 3.0)
rg(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
rg(PW3c, function(x,y) return smoothstep(500, 690, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * 0.9 end, 3.0)
blendZone = RvG + (rect(556, 392, 110, 180) - fgM - limbsKeep:grow(2))
blend(blendZone - cliffM:grow(0.6), {angle=0.0})
blend(blendZone - cliffM:grow(0.6), {angle=0.02})
print(wait(0))

--@ chunk 296
print(wait(10*24*60))
print(drying(700,500), drying(800,470), drying(660,560), drying(950,420), drying(800,440), drying(600,610))

--@ chunk 297
crestBlob = fgM * rect(520, 566, 200, 90)
work(crestBlob, {hand="body", tool="filbert 14", pile=Fbase, angle=crestAng, coverage=6.0, clip=true, fill=true, length={30,90}, angle_jitter=0.1, load=1.0})
work(crestBlob, {hand="body", tool="filbert 16", pile=Fol, angle=crestAng, coverage=1.8, clip=true, length={30,80}, angle_jitter=0.18,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.4 + 0.9*fgn1(x,y)) * smoothstep(-2, 36, d), 0, 1) end})
work(crestBlob, {hand="body", tool="filbert 10", pile=Fwm, angle=crestAng, coverage=1.6, clip=true, length={25,60}, angle_jitter=0.22,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.3 + 0.9*fgn2(x+90,y)) * (1 - smoothstep(4, 40, d)) * smoothstep(-3, 6, d), 0, 1) end})
-- re-sow the grass in the repaired patch
for i = 1, 70 do
  local x = rand(528, 716)
  local d = (rand(0,1)^1.7) * 50 + 1.5
  local y = crestY(x) + d
  local near = 1 - smoothstep(0, 60, d)
  local r = math.random()
  local p
  if r < 0.45 * (0.4 + 0.6*near) then p = GrassTan
  elseif r < 0.62 * (0.4 + 0.6*near) then p = GrassGold
  elseif r < 0.82 then p = GrassOliv
  else p = GrassDeep end
  local h = rand(5, 15) * (0.8 + 0.5*(1-near))
  local side = (math.random() < 0.5) and -1 or 1
  lit_blade(x, y, h, side*rand(0.08, 0.65), p, rand(0.4, 0.62), 0.55 + 0.35*near)
end
print(wait(0))

--@ chunk 298
rightFG = (fgM * rect(500, 540, 520, 180)) - spr1 - spr2m - spr3
edgeFn = function(x,y)
  local d = y - crestY(x)
  local crestCrisp = 1 - smoothstep(2, 8, d)      -- 1 near the crest (found)
  local leftSoft = 1 - smoothstep(500, 560, x)    -- 1 near the left boundary (lost)
  return clamp(leftSoft * (1 - crestCrisp), 0, 1)
end
work(rightFG, {hand="body", tool="filbert 20", pile=Fbase, angle=crestAng, coverage=3.4, fill=true, length={40,120}, angle_jitter=0.12, clip=fgM, edge=edgeFn})
work(rightFG, {hand="body", tool="filbert 16", pile=Fol, angle=crestAng, coverage=2.2, length={30,90}, angle_jitter=0.18, clip=fgM, edge=edgeFn,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.45 + 0.95*fgn1(x,y)) * smoothstep(-2, 36, d) * (1 - smoothstep(110, 200, d)) * smoothstep(500, 580, x), 0, 1) end})
work(rightFG, {hand="body", tool="filbert 12", pile=Fwm, angle=crestAng, coverage=2.2, length={25,70}, angle_jitter=0.22, clip=fgM, edge=edgeFn,
  load_at=function(x,y) local d = y - crestY(x); return clamp((0.3 + 0.9*fgn2(x+90,y)) * (1 - smoothstep(4, 62, d)) * smoothstep(-3, 6, d) * smoothstep(500, 600, x), 0, 1) end})
work(rightFG, {hand="body", tool="filbert 18", pile=Fcool, angle=crestAng, coverage=2.4, length={30,90}, angle_jitter=0.15, clip=fgM, edge=edgeFn,
  load_at=function(x,y) local d = y - crestY(x); return clamp(smoothstep(60, 170, d) * (0.6 + 0.6*fgn1(x+30,y)) * smoothstep(500, 580, x), 0, 1) end})
print(wait(0))

--@ chunk 299
-- dark crest tufts on the right part (standing against the fog)
for i = 1, 38 do
  local x = rand(560, 1000)
  tuft2(x, crestY(x) + 1.5, math.random(5, 9), rand(9, 22), grassDark, 0.0, 6, 0.5)
end
-- sunlit grass blades on the right slope
local nb = 0
for i = 1, 620 do
  local x = rand(500, 996)
  local d = (rand(0,1)^1.7) * 66 + 1.5
  local y = crestY(x) + d
  if y < 712 then
    local near = 1 - smoothstep(0, 60, d)
    local r = math.random()
    local p
    if r < 0.45 * (0.4 + 0.6*near) then p = GrassTan
    elseif r < 0.62 * (0.4 + 0.6*near) then p = GrassGold
    elseif r < 0.82 then p = GrassOliv
    else p = GrassDeep end
    local h = rand(5, 15) * (0.8 + 0.5*(1-near))
    local side = (math.random() < 0.5) and -1 or 1
    lit_blade(x, y, h, side*rand(0.08, 0.65), p, rand(0.4, 0.62), 0.55 + 0.35*near)
    nb = nb + 1
  end
end
print("blades", nb)
print(wait(0))

--@ chunk 300
spruceFull(892, crestY(892)+2, 88, 40, 0.015, pineD)
spruceFull(944, crestY(944)+2, 60, 28, -0.02, pineD)
spruceFull(842, crestY(842)+2, 34, 17, 0.03, pineD)
print(wait(0))

--@ chunk 301
-- ===== guards: never paint over the oak's twigs or the figure again =====
segs = {}
for _, pts in ipairs(TWIGS) do
  for i = 1, #pts-1 do
    local a, b = pts[i], pts[i+1]
    if math.max(a[1], b[1]) > 420 and math.min(a[1], b[1]) < 640 and math.max(a[2], b[2]) > 370 and math.min(a[2], b[2]) < 540 then
      segs[#segs+1] = {a[1], a[2], b[1], b[2]}
    end
  end
end
print("segments near the valley", #segs)
function segDist2(px, py, s)
  local dx, dy = s[3]-s[1], s[4]-s[2]
  local l2 = dx*dx + dy*dy
  local t = 0
  if l2 > 0 then t = clamp(((px-s[1])*dx + (py-s[2])*dy)/l2, 0, 1) end
  local qx, qy = s[1]+t*dx, s[2]+t*dy
  return (px-qx)^2 + (py-qy)^2
end
twigGuard = mask(function(x, y)
  if x < 420 or x > 640 or y < 370 or y > 540 then return 0 end
  for i = 1, #segs do
    if segDist2(x, y, segs[i]) <= 2.6 then return 1 end
  end
  return 0
end)
guard = limbsKeep:grow(2.5) + twigGuard + figM:grow(3) + footM:grow(2)
print("guard area", guard:area(), twigGuard:area())

--@ chunk 302
-- ===== the wooded ridge on which the butte stands =====
ridgePts = {{588,494},{612,486},{640,477},{668,468},{696,460},{720,451},{741,441},
            {875,441},{893,447},{914,452},{938,451},{962,447},{986,448},{1010,453}}
ridgeTop = function(x) return interp(ridgePts, x) + 2.6*trn(x*1.0, 8.0) end
ridgeM = mask(function(x,y)
  if x < 588 then return 0 end
  local t = ridgeTop(x)
  return smoothstep(t-0.6, t+0.6, y) * (y < t + 58 and 1 or 0)
end) - fgM - guard
-- far, pale, lilac body of the wooded ridge, then darker crowns
dabwork(ridgeM, ScFar, 5.0)
dabwork(ridgeM * mask(function(x,y) local t = ridgeTop(x); return smoothstep(t+4, t+26, y) * (0.55 + 0.45*cn2(x*1.1, y*1.3)) end), ScMid, 3.4, "filbert 7")
-- warm light on the crowns facing the afterglow, strongest left of the butte
work(ridgeM * mask(function(x,y) local t = ridgeTop(x); return (1 - smoothstep(t+1, t+14, y)) * (1 - smoothstep(650, 745, x)) * (0.4 + 0.6*cn1(x+20, y)) end),
  {hand="body", tool="filbert 5", pile=ScLit, angle=1.4, coverage=1.6, clip=true, length={3,9}, angle_jitter=0.7})
-- a scatter of conifer spikes along the skyline of the ridge
for i = 1, 30 do
  local xx = rand(600, 1000)
  if xx < 736 or xx > 880 then
    local t = ridgeTop(xx)
    local h = rand(6, 15)
    work(tinyPine(xx, t + 4, h, h*0.45), {hand="detail", pile=ScDark, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
  end
end
print(wait(0))

--@ chunk 303
print(drying(700,480), drying(800,500))
blend(ridgeM, {angle=0.0})
blend(ridgeM, {angle=0.02})
blend(ridgeM, {angle=-0.02})
print(wait(0))

--@ chunk 304
-- 1. close the gap between the butte and its ridge: the ridge climbs the cliff foot with ragged crowns
footFill = (rect(733, 416, 152, 40) - cliffM:grow(-1.0) * rect(700, 300, 250, 120)) - guard
-- (the cliff face above y=416 stays untouched; below, ridge colour over the old stipple band and the cream strip)
footFill2 = rect(733, 418, 152, 36) - guard
work(footFill2, {hand="body", tool="filbert 8", pile=ScMid, angle=0.03, coverage=5.0, clip=true, fill=true, length={20,50}, angle_jitter=0.2})
work(footFill2 * mask(function(x,y) return smoothstep(424, 440, y) end), {hand="body", tool="filbert 8", pile=ScFar, angle=0.03, coverage=3.0, clip=true, fill=true, length={20,50}, angle_jitter=0.2})
-- forest line at the cliff foot: dark crowns and spikes at uneven heights, none in a row
local xx = 738
while xx < 880 do
  local yb = 424 + rand(-3, 5) + 4*trn(xx*1.7, 12)
  local r = math.random()
  local m
  if r < 0.5 then
    local rr = rand(2.4, 4.6)
    m = ellipse(xx, yb - rr*0.3, rr, rr*rand(0.7, 0.95))
  else
    local h = rand(7, 16)
    m = tinyPine(xx, yb + 4, h, h*rand(0.4, 0.5))
  end
  work(m, {hand="detail", pile=ScDark, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
  xx = xx + rand(2.2, 5.0)
end
print(wait(0))

--@ chunk 305
print(wait(12*24*60))
print(drying(800,430), drying(700,480), drying(600,500), drying(800,500), drying(300,300))

--@ chunk 306
-- ===== mist: the butte stands in it; the ridge sinks into it; one wet-in-wet session =====
vt = function(x) return ridgeTop(x) + 1 + 15*smoothstep(590, 720, x) + 30*bump(x, 808, 130) + 7*fogwisp(x*1.2, 3) end
V2 = mask(function(x,y)
  if x < 575 or x > 1012 then return 0 end
  local t = vt(x)
  return smoothstep(t-2, t+2, y) * (y < 566 and 1 or 0)
end) - fgM - guard
vtf = function(x) return 414 + 10*fogwisp(x*1.3, 20) + 4*fogwisp(x*0.5, 77) end
V1 = mask(function(x,y)
  if x < 716 or x > 902 then return 0 end
  local t = vtf(x)
  return smoothstep(t-2, t+2, y) * (y < 476 and 1 or 0)
end) - guard
clipAll = (rect(556, 396, 470, 190) - guard - fgM) 
edgeV2 = function(x,y)
  local top = 1 - smoothstep(vt(x)+6, vt(x)+30, y)
  local left = 1 - smoothstep(590, 650, x)
  return clamp(math.max(top, left)*0.95, 0, 1)
end
edgeV1 = function(x,y) return 0.9 end
function vwork(m, p, f, cov, edgefn, tool)
  work(m, {hand="body", tool=tool or "filbert 20", pile=p, angle=0.0, coverage=cov, fill=true, length={60,160}, angle_jitter=0.04,
    clip=clipAll, edge=edgefn, load_at=function(x,y) return clamp(f(x,y),0,1) end})
end
vwork(V1, PW2b, function(x,y) return 1 end, 4.4, edgeV1)
vwork(V1, PW1b, function(x,y) return 0.8*(1 - smoothstep(vtf(x)+6, vtf(x)+40, y)) end, 2.4, edgeV1)
vwork(V2, PW2b, function(x,y) return 1 end, 4.6, edgeV2)
vwork(V2, PW1b, function(x,y) return (1 - smoothstep(vt(x)+10, vt(x)+90, y)) * (0.35 + 0.65*bump(x, 700, 360)) end, 2.6, edgeV2)
vwork(V2, PW3c, function(x,y) return smoothstep(500, 690, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * 0.9 end, 3.0, edgeV2)
print(wait(0))

--@ chunk 307
print(wait(12*24*60))
print(drying(800,430), drying(700,480), drying(600,500), drying(800,500), drying(300,300), drying(800,380))

--@ chunk 308
-- ===== masks for the repair =====
Fc = function(x) return 424 + 6*math.sin(x/13 + 0.5) + 3*math.sin(x/5.3) end
xb2 = function(y) return 556 + 10*math.sin(y/29) + 6*math.sin(y/11 + 1) end
-- twig guard (wider radius), all recorded twigs near the seam and the valley
segs2 = {}
for _, pts in ipairs(TWIGS) do
  for i = 1, #pts-1 do
    local a, b = pts[i], pts[i+1]
    if math.max(a[1], b[1]) > 440 and math.min(a[1], b[1]) < 680 and math.max(a[2], b[2]) > 350 and math.min(a[2], b[2]) < 570 then
      segs2[#segs2+1] = {a[1], a[2], b[1], b[2]}
    end
  end
end
print("segments", #segs2)
twigGuard2 = mask(function(x, y)
  if x < 440 or x > 680 or y < 350 or y > 570 then return 0 end
  for i = 1, #segs2 do
    if segDist2(x, y, segs2[i]) <= 4.8 then return 1 end
  end
  return 0
end)
guard2 = limbsKeep:grow(1.8) + twigGuard2 + figM:grow(3) + footM:grow(2)
cliffKeepM = cliffM * mask(function(x,y) return (y <= Fc(x) + 1) and 1 or 0 end)
Rr = mask(function(x,y)
  if x < xb2(y) then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guard2 - cliffKeepM:grow(0.5)
Rext = mask(function(x,y)
  if x < xb2(y) - 70 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guard2 - cliffKeepM:grow(0.5)
print("areas", twigGuard2:area(), guard2:area(), Rr:area(), Rext:area())

--@ chunk 309
-- ===== SESSION: cliff foot first (wet), then the fog coat =====
cliffLow = cliffM * mask(function(x,y) return (y >= 370 and y <= Fc(x) + 12) and 1 or 0 end)
ramp0 = function(y) return clamp(smoothstep(370, 392, y), 0.12, 1) end
work(cliffLow, {hand="body", tool="filbert 9", pile=BCliff, angle=vertw, coverage=6.0, clip=true, fill=true, length={24,70}, angle_jitter=0.06,
  load_at=function(x,y) return ramp0(y) end})
work(cliffLow * mask(function(x,y) return smoothstep(shEdge(x,y)-10, shEdge(x,y)+10, x) end),
  {hand="body", tool="filbert 8", pile=BCliffDD, angle=vertw, coverage=4.0, clip=true, fill=true, length={22,60}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(ramp0(y) * (0.55 + 0.45*smoothstep(shEdge(x,y), 870, x)), 0, 1) end})
work(cliffLow * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-9, litEdge(x,y)+9, x)) end),
  {hand="body", tool="filbert 7", pile=BCliffW, angle=vertw, coverage=3.6, clip=true, fill=true, length={18,55}, angle_jitter=0.07,
  load_at=function(x,y) return clamp(ramp0(y) * 0.9 * (1 - 0.5*smoothstep(380, 430, y)), 0, 1) end})
-- haze: the cliff pales and cools into the fog toward its foot
work(cliffLow, {hand="body", tool="filbert 9", pile=PW3c, angle=vertw, coverage=3.2, clip=true, fill=true, length={24,70}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(smoothstep(398, Fc(x)+4, y)^1.1 * 0.97, 0, 1) end})
print(wait(0))

--@ chunk 310
-- cache the cliff's top contour
cliffTopTab = {}
for x = 726, 884 do
  local t = 440
  for y = 300, 440 do
    if cliffM:at(x, y) > 0.5 then t = y; break end
  end
  cliffTopTab[x] = t
end
function cliffTopC(x)
  local xi = clamp(math.floor(x), 726, 883)
  local f = x - xi
  return cliffTopTab[xi]*(1-f) + cliffTopTab[xi+1]*f
end
cliffBody2 = cliffM * mask(function(x,y) return (y >= cliffTopC(x) + 11 and y <= Fc(x) + 12) and 1 or 0 end)
print("body2 area", cliffBody2:area())
hz = function(x,y) return clamp(smoothstep(cliffTopC(x)+34, Fc(x)+4, y)^1.0 * 0.96, 0, 1) end
work(cliffBody2, {hand="body", tool="filbert 9", pile=BCliff, angle=vertw, coverage=6.0, clip=true, fill=true, length={24,70}, angle_jitter=0.06})
work(cliffBody2 * mask(function(x,y) return smoothstep(shEdge(x,y)-10, shEdge(x,y)+10, x) end),
  {hand="body", tool="filbert 8", pile=BCliffDD, angle=vertw, coverage=4.0, clip=true, fill=true, length={22,60}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.55 + 0.45*smoothstep(shEdge(x,y), 870, x), 0, 1) end})
work(cliffBody2 * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-9, litEdge(x,y)+9, x)) end),
  {hand="body", tool="filbert 7", pile=BCliffW, angle=vertw, coverage=3.4, clip=true, fill=true, length={18,55}, angle_jitter=0.07,
  load_at=function(x,y) return clamp(0.8 * (1 - 0.55*smoothstep(345, 425, y)), 0, 1) end})
work(cliffBody2, {hand="body", tool="filbert 9", pile=PW3c, angle=vertw, coverage=3.4, clip=true, fill=true, length={24,70}, angle_jitter=0.06,
  load_at=function(x,y) return hz(x,y) end})
print(wait(0))

--@ chunk 311
-- more weight in the upper cliff: darker violet-blue under the forest, strongest on the shade side; the lit edge keeps its warmth
darkTop = function(x,y) return clamp((1 - smoothstep(cliffTopC(x)+14, cliffTopC(x)+62, y)) , 0, 1) end
work(cliffBody2 * mask(function(x,y) return smoothstep(litEdge(x,y)-4, litEdge(x,y)+14, x) end),
  {hand="body", tool="filbert 8", pile=BCliffDD, angle=vertw, coverage=3.6, clip=true, fill=true, length={20,56}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(darkTop(x,y) * (0.5 + 0.5*smoothstep(790, 850, x)), 0, 1) end})
work(cliffBody2 * mask(function(x,y) return smoothstep(litEdge(x,y)-4, litEdge(x,y)+14, x) end),
  {hand="body", tool="filbert 8", pile=BCliff, angle=vertw, coverage=2.6, clip=true, fill=true, length={20,56}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(darkTop(x,y) * 0.6 * (1 - smoothstep(790, 850, x)), 0, 1) end})
print(wait(0))

--@ chunk 312
blend(cliffM:grow(0.4) * rect(700, 360, 220, 90), {angle=1.5708})
blend(cliffM:grow(0.4) * rect(700, 360, 220, 90), {angle=1.5708})
print(wait(0))

--@ chunk 313
-- ===== cliff re-laid wet-in-wet, whole body at once =====
cliffBodyAll = cliffM * mask(function(x,y) return (y >= cliffTopC(x) + 7 and y <= Fc(x) + 12) and 1 or 0 end)
work(cliffBodyAll, {hand="body", tool="filbert 9", pile=BCliff, angle=vertw, coverage=6.5, clip=true, fill=true, length={24,70}, angle_jitter=0.06})
work(cliffBodyAll * mask(function(x,y) return smoothstep(shEdge(x,y)-10, shEdge(x,y)+10, x) end),
  {hand="body", tool="filbert 8", pile=BCliffDD, angle=vertw, coverage=4.4, clip=true, fill=true, length={22,60}, angle_jitter=0.06,
  load_at=function(x,y) return clamp(0.6 + 0.4*smoothstep(shEdge(x,y), 870, x), 0, 1) end})
work(cliffBodyAll * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-9, litEdge(x,y)+9, x)) end),
  {hand="body", tool="filbert 7", pile=BCliffW, angle=vertw, coverage=4.0, clip=true, fill=true, length={18,55}, angle_jitter=0.07,
  load_at=function(x,y) return clamp(0.95 * (1 - 0.55*smoothstep(345, 425, y)), 0, 1) end})
rib1b = cliffBodyAll * mask(function(x,y) local c = 800 + 5*irr(x*0.6, y); return (1 - smoothstep(6, 12, math.abs(x - c))) * (1 - 0.7*smoothstep(345, 428, y)) end)
work(rib1b, {hand="body", tool="filbert 5", pile=BCliffP, angle=vertw, coverage=2.6, clip=true, length={12,40}, angle_jitter=0.07})
rib2b = cliffBodyAll * mask(function(x,y) local c = 846 + 4*irr(x*0.6+40, y); return (1 - smoothstep(3, 7, math.abs(x - c))) * (1 - 0.8*smoothstep(335, 400, y)) * 0.8 end)
work(rib2b, {hand="body", tool="filbert 4", pile=BCliffW, angle=vertw, coverage=2.0, clip=true, length={10,30}, angle_jitter=0.07})
-- the gap between the old crown line and the new body top
topGap = cliffM * mask(function(x,y) return (y >= cliffTopC(x) - 1 and y < cliffTopC(x) + 9) and 1 or 0 end)
work(topGap, {hand="body", tool="filbert 5", pile=Crown, angle=0.0, coverage=2.4, clip=true, length={6,14}, angle_jitter=0.3, load=0.8})
print(wait(0))

--@ chunk 314
-- A. haze on the cliff foot: stipple dissolves it into the cream (wet-in-wet, no hard edge)
hazeCov = function(x,y) return 3.2 * smoothstep(cliffTopC(x)+36, Fc(x)+8, y)^1.2 end
stipple(cliffM, {pile=PW2b, width=3.4, coverage=hazeCov, feather=0.5, clip=cliffM, pressure={0.35,0.8}, dips={20,0.9,0.3}, cluster=0.2})
stipple(cliffM, {pile=PW3c, width=3.0, coverage=function(x,y) return 1.6*smoothstep(cliffTopC(x)+60, Fc(x)+8, y)^1.3 end, feather=0.5, clip=cliffM, pressure={0.3,0.7}, dips={20,0.9,0.3}, cluster=0.2})
print(wait(0))

--@ chunk 315
stipple(cliffM, {pile=PW2b, width=3.8, coverage=function(x,y) return 5.5 * smoothstep(cliffTopC(x)+50, Fc(x)+2, y)^1.1 end, feather=0.4, clip=cliffM, pressure={0.4,0.9}, dips={20,0.9,0.3}, cluster=0.15})
print(wait(0))

--@ chunk 316
-- ===== fog coat, right of the seam, one session =====
Rr2 = mask(function(x,y)
  if x < xb2(y) - 60 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guard2 - cliffKeepM:grow(0.5)
xfade = function(x,y) return clamp(0.12 + 0.88*smoothstep(xb2(y)-60, xb2(y)+36, x), 0, 1) end
function rr_(p, f, cov, tool)
  work(Rr2, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y) * xfade(x,y), 0, 1) end})
end
rr_(PW2b, function(x,y) return 1 end, 5.0)
rr_(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
rr_(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+130, y)) * smoothstep(Atop(x)+10, Atop(x)+36, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 3.0)
rr_(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
rr_(PW3c, function(x,y) return smoothstep(500, 690, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * 0.9 end, 3.0)
print(wait(0))

--@ chunk 317
print(drying(700,480), drying(900,450), drying(600,400), drying(600,520), drying(480,500), drying(400,450))

--@ chunk 318
-- grid-indexed twig guard for x in [290, 700]
GRID = 12
gidx = {}
local nseg = 0
for _, pts in ipairs(TWIGS) do
  for i = 1, #pts-1 do
    local a, b = pts[i], pts[i+1]
    if math.max(a[1], b[1]) > 280 and math.min(a[1], b[1]) < 710 and math.max(a[2], b[2]) > 340 and math.min(a[2], b[2]) < 580 then
      local s = {a[1], a[2], b[1], b[2]}
      nseg = nseg + 1
      local x0, x1 = math.floor((math.min(a[1], b[1]) - 6)/GRID), math.floor((math.max(a[1], b[1]) + 6)/GRID)
      local y0, y1 = math.floor((math.min(a[2], b[2]) - 6)/GRID), math.floor((math.max(a[2], b[2]) + 6)/GRID)
      for gx = x0, x1 do
        for gy = y0, y1 do
          local k = gx*1000 + gy
          local c = gidx[k]
          if not c then c = {}; gidx[k] = c end
          c[#c+1] = s
        end
      end
    end
  end
end
print("segments", nseg)
twigGuard3 = mask(function(x, y)
  if x < 280 or x > 710 or y < 340 or y > 580 then return 0 end
  local c = gidx[math.floor(x/GRID)*1000 + math.floor(y/GRID)]
  if not c then return 0 end
  for i = 1, #c do
    if segDist2(x, y, c[i]) <= 4.8 then return 1 end
  end
  return 0
end)
guard3 = limbsKeep:grow(1.8) + twigGuard3 + figM:grow(3) + footM:grow(2)
print("guard3", guard3:area(), twigGuard3:area())

--@ chunk 319
Rr3 = mask(function(x,y)
  if x < 292 or x > 650 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guard3 - cliffKeepM:grow(0.5)
xf3 = function(x) return clamp(0.10 + 0.90*smoothstep(300, 480, x), 0, 1) end
function r3(p, f, cov, tool)
  work(Rr3, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y) * xf3(x), 0, 1) end})
end
r3(PW2b, function(x,y) return 1 end, 5.0)
r3(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
r3(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+130, y)) * smoothstep(Atop(x)+10, Atop(x)+36, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 3.0)
r3(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
r3(PW3c, function(x,y) return smoothstep(500, 690, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * 0.9 end, 3.0)
print(wait(0))
print(drying(700,480), drying(500,480), drying(600,420))

--@ chunk 320
blendAll = (Rr3 + Rr2) 
blend(blendAll, {angle=0.0})
blend(blendAll, {angle=0.02})
blend(blendAll, {angle=-0.02})
blend(blendAll, {angle=0.0})
print(wait(0))

--@ chunk 321
twigTight = mask(function(x, y)
  if x < 280 or x > 710 or y < 340 or y > 580 then return 0 end
  local c = gidx[math.floor(x/GRID)*1000 + math.floor(y/GRID)]
  if not c then return 0 end
  for i = 1, #c do
    if segDist2(x, y, c[i]) <= 4.0 then return 1 end
  end
  return 0
end)
guardT = limbsKeep:grow(0.9) + twigTight + figM:grow(1.6) + footM:grow(1.2)
Rr4 = mask(function(x,y)
  if x < 292 or x > 640 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guardT - cliffKeepM:grow(0.5)
function r4(p, f, cov, tool)
  work(Rr4, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y) * xf3(x), 0, 1) end})
end
r4(PW2b, function(x,y) return 1 end, 4.4)
r4(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+130, y)) * smoothstep(Atop(x)+10, Atop(x)+36, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.6)
r4(PW3c, function(x,y) return smoothstep(500, 690, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * 0.8 end, 2.6)
blend(Rr4, {angle=0.0})
blend(Rr4, {angle=0.02})
blend(Rr4, {angle=-0.02})
print(wait(0))
print(drying(700,480), drying(500,480))

--@ chunk 322
seamR = mask(function(x,y)
  if x < 600 or x > 800 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guardT - cliffM:grow(0.6)
stipple(seamR, {pile=PW2b, width=5, coverage=function(x,y) return 4.0 * (1 - smoothstep(640, 790, x)) end, feather=0.5, clip=seamR,
  pressure={0.4,0.85}, dips={20,0.9,0.3}, cluster=0.0})
blend(seamR, {angle=0.0})
blend(seamR, {angle=0.02})
blend(seamR, {angle=-0.02})
print(wait(0))

--@ chunk 323
print(wait(12*24*60))
print(drying(700,480), drying(500,480), drying(600,420), drying(880,650), drying(300,300))

--@ chunk 324
-- ===== redraw twig polylines that the fog coat veiled (x 470..660), over the dry fog =====
local nre = 0
for _, pts in ipairs(TWIGS) do
  local mx = 0
  for _, p in ipairs(pts) do mx = math.max(mx, p[1]) end
  local touches = false
  for _, p in ipairs(pts) do
    if p[1] > 470 and p[1] < 700 and p[2] > 330 and p[2] < 560 then touches = true; break end
  end
  if touches and #pts >= 2 then
    local w0 = 1.3
    tw:load(OakT, 1.0)
    tw:stroke(pts, {pressure={tw:pressure_for(w0), tw:pressure_for(0.9)}, ramps={0.0, 0.0}})
    nre = nre + 1
  end
end
print("redrawn", nre)
print(wait(0))

--@ chunk 325
-- ===== hazy wooded hills rising out of the fog (right half) =====
clipH = (rect(540, 396, 480, 230) - guardT - fgM - cliffM:grow(1.0)) - spr1 - spr2m - spr3
function softHill(pts, amp, seed, x0, x1, depth, pile_, cov, tool_, fadeEnds)
  local topf = function(x) return interp(pts, x) + amp*trn(x*1.0 + seed, 3.0) end
  local m = mask(function(x,y)
    if x < x0 or x > x1 then return 0 end
    local t = topf(x)
    return smoothstep(t-0.6, t+0.6, y) * ((y < t + depth) and 1 or 0)
  end)
  local edgeFn = function(x,y)
    local t = topf(x)
    local bottom = smoothstep(t + depth*0.30, t + depth*0.9, y)
    local ends = 0
    if fadeEnds then
      ends = math.max(1 - smoothstep(x0, x0 + 80, x), smoothstep(x1 - 80, x1, x))
    end
    return clamp(math.max(0.05 + 0.9*bottom, 0.9*ends), 0, 1)
  end
  work(m, {hand="body", tool=tool_ or "filbert 12", pile=pile_, angle=0.03, coverage=cov or 4.0, fill=true, length={30,90}, angle_jitter=0.15,
    clip=clipH, edge=edgeFn})
  return m
end
-- far lilac ridge behind and either side of the butte's foot (pale, close to the fog)
hA = softHill({{600,484},{640,472},{690,460},{741,446}}, 2.0, 11, 600, 745, 56, PAc, 4.0, "filbert 12", true)
hB = softHill({{872,446},{905,447},{938,443},{972,440},{1010,442}}, 1.8, 21, 868, 1012, 54, PAc, 4.0, "filbert 12", true)
-- nearer, darker wooded hills in the mist
hC = softHill({{585,516},{625,500},{665,490},{705,488},{745,494},{790,508},{820,520}}, 2.4, 31, 585, 825, 70, ScFar, 4.2, "filbert 12", true)
hD = softHill({{800,540},{850,520},{900,508},{950,506},{1000,512},{1012,518}}, 2.2, 41, 800, 1012, 74, ScFar, 4.2, "filbert 12", true)
print(wait(0))

--@ chunk 326
print(wait(12*24*60))
print(drying(700,500), drying(900,480), drying(700,420), drying(300,300))

--@ chunk 327
-- ===== STEP A: cover the slabs: one cream coat over the right half of the fog sea, blended =====
clipFog = (rect(520, 396, 500, 330) - guardT - fgM - cliffM:grow(0.8))
FogM = mask(function(x,y)
  if x < 540 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) * clipFog
function fgA(p, f, cov, tool)
  work(FogM, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y), 0, 1) end})
end
fgA(PW2b, function(x,y) return 1 end, 5.2)
fgA(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+130, y)) * smoothstep(Atop(x)+10, Atop(x)+36, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fgA(PW3c, function(x,y) return smoothstep(520, 690, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * 0.8 end, 2.6)
blend(FogM, {angle=0.0})
blend(FogM, {angle=0.02})
blend(FogM, {angle=-0.02})
blend(FogM, {angle=0.0})
print(wait(0))

--@ chunk 328
print(drying(800,440), drying(800,415), drying(900,450))
blend(rect(704, 408, 210, 56), {angle=0.0})
blend(rect(704, 408, 210, 56), {angle=0.02})
print(wait(0))

--@ chunk 329
veilTop = cliffM * rect(704, 352, 210, 64)
stipple(veilTop, {pile=PW3c, width=3.6, coverage=function(x,y) return 3.2 * smoothstep(360, 410, y)^1.1 end, feather=0.4, clip=cliffM,
  pressure={0.3,0.7}, dips={20,0.9,0.3}, cluster=0.1})
stipple(veilTop, {pile=PW2b, width=3.2, coverage=function(x,y) return 2.4 * smoothstep(378, 412, y)^1.2 end, feather=0.4, clip=cliffM,
  pressure={0.3,0.7}, dips={20,0.9,0.3}, cluster=0.1})
print(wait(0))

--@ chunk 330
veilTop2 = cliffM * rect(704, 340, 210, 84)
stipple(veilTop2, {pile=PW3c, width=1.7, coverage=function(x,y) return 9.0 * smoothstep(352, 414, y)^1.0 end, feather=0.6, clip=cliffM,
  pressure={0.3,0.65}, dips={30,0.9,0.3}, cluster=0.0})
stipple(veilTop2, {pile=PW2b, width=1.7, coverage=function(x,y) return 8.0 * smoothstep(384, 420, y)^1.0 end, feather=0.6, clip=cliffM,
  pressure={0.3,0.65}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 331
print(drying(800,380), drying(800,400), drying(800,420))
local bz = cliffM:grow(0.4) * rect(704, 340, 210, 84)
blend(bz, {angle=1.5708})
blend(bz, {angle=1.5708})
blend(bz, {angle=1.45})
print(wait(0))

--@ chunk 332
print(wait(14*24*60))
print(drying(800,380), drying(800,400), drying(800,420), drying(800,450), drying(300,300))

--@ chunk 333
print(wait(0))
print(W, H, type(cliffBodyAll), type(OAK), type(TWIGS))

--@ chunk 334
-- ===== CLIFF, repainted in one wet-in-wet session; the haze is a colour gradient in bands =====
tRock = {}
tRock.lit  = {["red earth"]=0.7, ["yellow ochre"]=0.5, ["lead white"]=1.7, ["vermilion"]=0.25, ["smalt"]=0.3}
tRock.mid  = {["smalt"]=1.9, ["raw umber"]=0.75, ["Prussian blue"]=0.14, ["lead white"]=1.0, ["red earth"]=0.4}
tRock.dark = {["smalt"]=1.3, ["raw umber"]=1.2, ["Prussian blue"]=0.35, ["lead white"]=0.35, ["red earth"]=0.25}
tFogC = {["lead white"]=7, ["pale smalt"]=0.6, ["red earth"]=0.12, ["yellow ochre"]=0.12}
tFogW = {["lead white"]=7, ["yellow ochre"]=0.35, ["chrome yellow"]=0.1, ["vermilion"]=0.09}
function norm(t) local s = 0; for _, v in pairs(t) do s = s + v end; local o = {}; for k, v in pairs(t) do o[k] = v/s end; return o end
function mixp(rock, fog, f)
  local a, b = norm(rock), norm(fog)
  local acc = {}
  for k, v in pairs(a) do acc[k] = (acc[k] or 0) + v*(1-f) end
  for k, v in pairs(b) do acc[k] = (acc[k] or 0) + v*f end
  local lst = {}
  for k, v in pairs(acc) do if v > 0.004 then lst[#lst+1] = {k, v*10} end end
  table.sort(lst, function(p, q) return p[1] < q[1] end)
  return pile(lst)
end
fr = {0.0, 0.2, 0.45, 0.7, 0.92}
bandsY = {{340, 374}, {364, 394}, {384, 410}, {402, 422}, {414, 440}}
zLit = cliffBodyAll * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-9, litEdge(x,y)+9, x)) end)
zDark = cliffBodyAll * mask(function(x,y) return smoothstep(shEdge(x,y)-10, shEdge(x,y)+10, x) end)
zMid = cliffBodyAll
for i = 1, #fr do
  local y0, y1 = bandsY[i][1], bandsY[i][2]
  local prof = function(y)
    local up = (i == 1) and 1 or smoothstep(y0, y0 + 0.4*(y1-y0), y)
    local dn = (i == #fr) and 1 or (1 - smoothstep(y0 + 0.65*(y1-y0), y1, y))
    return clamp(up*dn, 0, 1)
  end
  local pMid  = mixp(tRock.mid,  tFogC, fr[i])
  local pDark = mixp(tRock.dark, tFogC, fr[i])
  local pLit  = mixp(tRock.lit,  tFogW, fr[i])
  work(zMid,  {hand="body", tool="filbert 8", pile=pMid,  angle=vertw, coverage=4.0, clip=true, fill=true, length={20,60}, angle_jitter=0.06,
    load_at=function(x,y) return prof(y) end})
  work(zDark, {hand="body", tool="filbert 8", pile=pDark, angle=vertw, coverage=3.6, clip=true, fill=true, length={20,60}, angle_jitter=0.06,
    load_at=function(x,y) return prof(y) end})
  work(zLit,  {hand="body", tool="filbert 8", pile=pLit,  angle=vertw, coverage=3.6, clip=true, fill=true, length={20,60}, angle_jitter=0.06,
    load_at=function(x,y) return prof(y) end})
end
blend(cliffBodyAll, {angle=1.5708})
blend(cliffBodyAll, {angle=1.5708})
print(wait(0))

--@ chunk 335
print(wait(14*24*60))
print(drying(800,380), drying(800,400), drying(800,420), drying(300,300))

--@ chunk 336
-- ===== CLIFF, session 2: horizontal short strokes keep each band within its own height =====
-- base coats: full rock colours over the entire face (covers the cream), by zone
work(zMid,  {hand="body", tool="filbert 8", pile=mixp(tRock.mid,  tFogC, 0.0), angle=0.0, coverage=5.0, clip=true, fill=true, length={8,24}, angle_jitter=0.05})
work(zDark, {hand="body", tool="filbert 8", pile=mixp(tRock.dark, tFogC, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,24}, angle_jitter=0.05})
work(zLit,  {hand="body", tool="filbert 8", pile=mixp(tRock.lit,  tFogW, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,24}, angle_jitter=0.05})
for i = 2, #fr do
  local y0, y1 = bandsY[i][1], bandsY[i][2]
  local prof = function(y)
    local up = smoothstep(y0, y0 + 0.45*(y1-y0), y)
    local dn = (i == #fr) and 1 or (1 - smoothstep(y0 + 0.7*(y1-y0), y1, y))
    return clamp(up*dn, 0, 1)
  end
  local pMid  = mixp(tRock.mid,  tFogC, fr[i])
  local pDark = mixp(tRock.dark, tFogC, fr[i])
  local pLit  = mixp(tRock.lit,  tFogW, fr[i])
  local bm = rect(690, y0 - 2, 240, (y1 - y0) + 4)
  work(zMid * bm,  {hand="body", tool="filbert 8", pile=pMid,  angle=0.0, coverage=4.0, clip=true, fill=true, length={8,24}, angle_jitter=0.05,
    load_at=function(x,y) return prof(y) end})
  work(zDark * bm, {hand="body", tool="filbert 8", pile=pDark, angle=0.0, coverage=3.6, clip=true, fill=true, length={8,24}, angle_jitter=0.05,
    load_at=function(x,y) return prof(y) end})
  work(zLit * bm,  {hand="body", tool="filbert 8", pile=pLit,  angle=0.0, coverage=3.6, clip=true, fill=true, length={8,24}, angle_jitter=0.05,
    load_at=function(x,y) return prof(y) end})
end
blend(cliffBodyAll, {angle=1.5708})
blend(cliffBodyAll, {angle=1.5708})
print(wait(0))

--@ chunk 337
-- ===== cliff foot: a transparent cream glaze ramping in toward the fog line, then a little body paint at the very foot =====
Pg  = pile{{"lead white",7},{"pale smalt",0.6},{"red earth",0.12},{"yellow ochre",0.12}, medium=0.55}
Pgw = pile{{"lead white",7},{"yellow ochre",0.35},{"chrome yellow",0.1},{"vermilion",0.09}, medium=0.55}
footZ = cliffBodyAll * rect(690, 380, 240, 70)
ramp1 = function(x,y) return clamp(smoothstep(392, 438, y)^1.3, 0, 1) end
work(footZ, {hand="glaze", pile=Pg, angle=1.5708, coverage=3.0, clip=true, load_at=ramp1})
work(footZ * mask(function(x,y) return 1 - smoothstep(744, 800, x) end), {hand="glaze", pile=Pgw, angle=1.5708, coverage=2.4, clip=true,
  load_at=function(x,y) return clamp(smoothstep(400, 438, y)^1.2, 0, 1) end})
print(wait(0))

--@ chunk 338
print(drying(800,420), drying(800,390))
blend(cliffBodyAll, {angle=1.5708})
blend(cliffBodyAll, {angle=1.5708})
blend(cliffBodyAll, {angle=1.45})
blend(cliffBodyAll, {angle=1.5708})
print(wait(0))

--@ chunk 339
print(wait(12*24*60))
print(drying(800,420), drying(800,390), drying(800,360))

--@ chunk 340
-- ===== cliff modelling with whole-face glazes (graded load, uniform faint film, edge = silhouette) =====
faceM = cliffBodyAll
GshadeP = pile{{"smalt",1.3},{"raw umber",1.2},{"Prussian blue",0.35},{"lead white",0.5},{"red earth",0.25}, medium=0.8}
GlitP   = pile{{"red earth",0.7},{"yellow ochre",0.55},{"lead white",1.4},{"vermilion",0.3}, medium=0.8}
-- shade: grows to the right, and is stronger high up (haze weakens it toward the foot)
work(faceM, {hand="glaze", pile=GshadeP, angle=1.5708, coverage=2.6, clip=true,
  load_at=function(x,y) return clamp((0.1 + 0.9*smoothstep(790, 862, x)) * (1 - 0.55*smoothstep(380, 436, y)), 0, 1) end})
-- light: strongest on the left, strongest high up
work(faceM, {hand="glaze", pile=GlitP, angle=1.5708, coverage=2.6, clip=true,
  load_at=function(x,y) return clamp((0.05 + 0.95*(1 - smoothstep(745, 800, x))) * (1 - 0.6*smoothstep(380, 436, y)), 0, 1) end})
print(wait(0))

--@ chunk 341
print(wait(14*24*60))
print(drying(800,420), drying(800,390), drying(800,360), drying(800,450), drying(600,500))

--@ chunk 342
-- ===== step 1: the pale rectangle under the butte: cover with matching cream, crisp but colour-matched, then blended =====
rectFix = (rect(696, 434, 230, 40)) - cliffM:grow(0.3) - fgM
work(rectFix, {hand="body", tool="filbert 20", pile=PW2b, angle=0.0, coverage=4.4, clip=true, fill=true, length={60,160}, angle_jitter=0.04})
blend(rect(680, 424, 270, 70) - cliffM:grow(0.3) - fgM, {angle=0.0})
blend(rect(680, 424, 270, 70) - cliffM:grow(0.3) - fgM, {angle=0.02})
print(wait(0))

--@ chunk 343
-- ===== re-coat the fog sea in one session: unified, softly billowed =====
FogM2 = mask(function(x,y)
  if x < 540 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) * (rect(520, 396, 500, 330) - guardT - fgM - cliffM:grow(0.8))
bil = noise{seed=411, period=130, octaves=3, persistence=0.5, stretch={0, 3.6}}
function fgB(p, f, cov, tool)
  work(FogM2, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y), 0, 1) end})
end
fgB(PW2b, function(x,y) return 1 end, 5.4)
fgB(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fgB(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fgB(PW4, function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
blend(FogM2, {angle=0.0})
blend(FogM2, {angle=0.02})
blend(FogM2, {angle=-0.02})
blend(FogM2, {angle=0.0})
print(wait(0))

--@ chunk 344
print(wait(12*24*60))
print(drying(800,450), drying(700,480), drying(900,550), drying(800,420))

--@ chunk 345
-- ===== the butte's foot dissolves into the mist: dry-on-dry stipple, density ramps downward =====
footStip = cliffM * rect(700, 380, 230, 62)
st1 = function(x,y) return 6.0 * smoothstep(396, 436, y)^1.25 end
stipple(footStip, {pile=PW2b, width=2.2, coverage=st1, feather=0.6, clip=cliffM, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.0})
stipple(footStip, {pile=PW2b, width=2.0, coverage=function(x,y) return 7.0 * smoothstep(414, 437, y)^1.1 end, feather=0.5, clip=cliffM, pressure={0.4,0.85}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 346
-- ===== soft hills emerging from the fog (right half). Crisp wooded tops, bases lost in the mist =====
clipH2 = (rect(596, 440, 420, 240) - guardT - fgM - cliffM:grow(1.0))
function softHill2(pts, amp, seed, x0, x1, depth, pile_, cov, tool_, bottomLost)
  local topf = function(x) return interp(pts, x) + amp*trn(x*1.0 + seed, 3.0) end
  local m = mask(function(x,y)
    if x < x0 or x > x1 then return 0 end
    local t = topf(x)
    return smoothstep(t-0.6, t+0.6, y) * ((y < t + depth) and 1 or 0)
  end) * clipH2
  local edgeFn = function(x,y)
    local t = topf(x)
    local bottom = smoothstep(t + depth*0.20, t + depth*0.85, y)
    local ends = math.max(1 - smoothstep(x0, x0 + 70, x), smoothstep(x1 - 70, x1, x))
    return clamp(math.max(0.04 + 0.92*bottom, 0.95*ends), 0, 1)
  end
  work(m, {hand="body", tool=tool_ or "filbert 12", pile=pile_, angle=0.03, coverage=cov or 4.0, fill=true, length={30,90}, angle_jitter=0.15,
    clip=clipH2, edge=edgeFn,
    load_at=function(x,y) local t = topf(x); return clamp(1.0 - 0.55*smoothstep(t+depth*0.3, t+depth, y), 0.2, 1) end})
  return m
end
h1 = softHill2({{604,500},{640,486},{672,478},{706,478},{738,486},{770,500}}, 2.2, 5, 604, 775, 56, ScFar, 4.2)
h2 = softHill2({{905,468},{935,458},{965,452},{1000,452},{1012,455}}, 2.0, 15, 900, 1012, 54, ScFar, 4.2)
h3 = softHill2({{745,540},{790,524},{835,514},{880,512},{925,520},{960,534}}, 2.4, 25, 745, 965, 66, ScMid, 4.2)
print(wait(0))

--@ chunk 347
print(wait(14*24*60))
print(drying(700,500), drying(850,540), drying(950,480), drying(800,420))

--@ chunk 348
-- ===== re-run the fog recipe (same masks, same noise) to bury the grey slabs =====
fgB(PW2b, function(x,y) return 1 end, 5.8)
fgB(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fgB(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fgB(PW4, function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
blend(FogM2, {angle=0.0})
blend(FogM2, {angle=0.02})
blend(FogM2, {angle=-0.02})
blend(FogM2, {angle=0.0})
print(wait(0))

--@ chunk 349
print(wait(12*24*60))
print(drying(700,500), drying(850,540), drying(950,480), drying(800,420))

--@ chunk 350
-- ===== hills in the fog, by paint-merging (the mask boundary vanishes because paint == fog there) =====
hillRock = {["lead white"]=2.4, ["smalt"]=2.0, ["raw umber"]=0.45, ["red earth"]=0.3, ["Prussian blue"]=0.06}
hillNear = {["smalt"]=1.6, ["raw umber"]=0.7, ["lead white"]=1.6, ["red earth"]=0.3, ["green earth"]=0.25, ["Prussian blue"]=0.15}
fogTab = {["lead white"]=7, ["pale smalt"]=0.6, ["red earth"]=0.12, ["yellow ochre"]=0.12}
fracs = {0.0, 0.25, 0.5, 0.75, 1.0}
function fogHill(pts, amp, seed, x0, x1, depth, rockT, fadeAt, tool_)
  local topf = function(x) return interp(pts, x) + amp*trn(x*1.0 + seed, 3.0) end
  local ext = 46   -- the mask runs well past the visible hill; there the paint is pure fog
  local M = mask(function(x,y)
    if x < x0 - ext or x > x1 + ext then return 0 end
    local t = topf(math.max(x0, math.min(x1, x)))
    -- ends: the top profile continues down into the fog so no step shows
    local endDrop = 0
    if x < x0 then endDrop = (x0 - x) * 1.1 elseif x > x1 then endDrop = (x - x1) * 1.1 end
    local tt = t + endDrop
    return smoothstep(tt-0.6, tt+0.6, y) * ((y < t + depth) and 1 or 0)
  end) * (rect(560, 430, 460, 260) - guardT - fgM - cliffM:grow(1.0))
  local fy = function(x,y)
    local t = topf(math.max(x0, math.min(x1, x)))
    return smoothstep(t + depth*0.10, t + depth*fadeAt, y)
  end
  local fxf = function(x)
    return math.max(1 - smoothstep(x0, x0 + 60, x), smoothstep(x1 - 60, x1, x))
  end
  local F = function(x,y) return clamp(math.max(fy(x,y), fxf(x)), 0, 1) end
  for k, fr_ in ipairs(fracs) do
    local p = mixp(rockT, fogTab, fr_)
    local w = function(x,y)
      local f = F(x,y)
      return clamp(1 - math.abs(f - fr_)/0.26, 0, 1)
    end
    work(M, {hand="body", tool=tool_ or "filbert 9", pile=p, angle=0.02, coverage=4.0, clip=true, fill=true, length={14,50}, angle_jitter=0.15,
      load_at=w})
  end
  blend(M, {angle=0.0})
  return M
end
hh1 = fogHill({{600,502},{632,489},{664,481},{698,480},{730,488},{762,502}}, 2.2, 5, 610, 760, 54, hillRock, 0.85)
print(wait(0))

--@ chunk 351
-- ===== clouds, right sky only (clear of the oak and the butte) =====
butteZone2 = (cliffM + crownM + needleM + towerK + step):grow(4)
cloudClip = (rect(628, 120, 390, 270) * aboveRidge) - butteZone2
CbodyP = pile{{"lead white",5.6},{"smalt",1.1},{"red earth",0.35},{"raw umber",0.08}, medium=0.3}
CunderP = pile{{"lead white",5.2},{"vermilion",0.5},{"yellow ochre",0.75},{"red earth",0.12}, medium=0.3}
function cloudBar(x0, x1, yc, th, drift, seed, tool_, ctool_)
  local b = barMask(x0, x1, yc, th, drift, seed, 1.0) * cloudClip
  work(b, {hand="body", tool=tool_ or "filbert 8", pile=CbodyP, angle=0.0, coverage=2.8, clip=true, fill=true, length={40,120}, angle_jitter=0.03, load=0.9})
  local under = (b - mask(function(x,y) return b:at(x, y+3.2) end))
  work(under, {hand="body", tool=ctool_ or "filbert 4", pile=CunderP, angle=0.0, coverage=2.2, clip=true, length={30,90}, angle_jitter=0.02, load=0.8})
  return b
end
cbA = cloudBar(636, 1010, 346, 11, -4, 61, "filbert 8", "filbert 4")
cbB = cloudBar(650, 980, 300, 8, 5, 71, "filbert 7", "filbert 3")
cbC = cloudBar(690, 1012, 214, 14, 9, 81, "filbert 9", "filbert 4")
cbD = cloudBar(660, 900, 262, 6, -3, 91, "filbert 6", "filbert 3")
for _, b in ipairs({cbA, cbB, cbC, cbD}) do
  local reg = b:grow(7) * cloudClip
  blend(reg, {angle=0.0})
  blend(reg, {angle=0.04})
end
print(wait(0))

--@ chunk 352
lift2 = brush("filbert", 34)
local nL = 0
for _, yc in ipairs({214, 262, 300, 346}) do
  for _, dy in ipairs({-9, 0, 9}) do
    for dir = 1, 2 do
      lift2:wipe(1.0)
      local xa, xb = 628, 1012
      if dir == 2 then xa, xb = 1012, 628 end
      lift2:stroke({{xa, yc+dy}, {(xa+xb)/2, yc+dy+rand(-1,1)}, {xb, yc+dy+rand(-1,1)}}, {pressure={0.55,0.55}, ramps={0.03,0.03}, clip=cloudClip})
      nL = nL + 1
    end
  end
end
print("lifts", nL)
print(wait(0))

--@ chunk 353
cb1 = brush("filbert", 9)
cb2 = brush("filbert", 3.5)
CbodyQ = pile{{"lead white",5.2},{"smalt",1.3},{"red earth",0.3},{"raw umber",0.12},{"yellow ochre",0.05}, medium=0.45}
CunderQ = pile{{"lead white",5},{"yellow ochre",0.8},{"vermilion",0.35},{"red earth",0.1}, medium=0.4}
function streak(xa, ya, xb, yb, wmul, p1, p2)
  local n = 6
  local pts = {}
  for i = 0, n do
    local t = i/n
    pts[#pts+1] = {xa + (xb-xa)*t, ya + (yb-ya)*t + rand(-0.9, 0.9)}
  end
  cb1:load(CbodyQ, p1 or 0.85)
  cb1:stroke(pts, {pressure={0.5*(wmul or 1), 0.12}, ramps={0.12, 0.0}, clip=cloudClip})
  -- the warm lower lip, thin and shorter, a hair below
  local pts2 = {}
  for i = 0, n do
    local t = i/n
    pts2[#pts2+1] = {xa + (xb-xa)*(0.1 + 0.8*t), ya + (yb-ya)*(0.1+0.8*t) + 3.2 + rand(-0.4, 0.4)}
  end
  cb2:load(CunderQ, p2 or 0.7)
  cb2:stroke(pts2, {pressure={0.4, 0.1}, ramps={0.15, 0.0}, clip=cloudClip})
end
streak(640, 350, 1000, 346, 1.0)
streak(990, 336, 700, 340, 0.8)
streak(700, 304, 960, 298, 0.9)
streak(980, 296, 760, 301, 0.7)
streak(690, 218, 1005, 212, 1.1)
streak(1005, 206, 790, 210, 0.9)
streak(670, 264, 880, 262, 0.8)
print(wait(0))

--@ chunk 354
print(wait(12*24*60))
print(drying(700,500), drying(850,540), drying(700,350), drying(800,215))

--@ chunk 355
-- ===== re-run the fog recipe a third time, to bury the pale trapezoid =====
fgB(PW2b, function(x,y) return 1 end, 5.8)
fgB(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fgB(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fgB(PW4, function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
blend(FogM2, {angle=0.0})
blend(FogM2, {angle=0.02})
blend(FogM2, {angle=-0.02})
blend(FogM2, {angle=0.0})
print(wait(0))

--@ chunk 356
-- ===== foreground: a warm glaze over the cool blotches (graded: none at the crest, full at the bottom) =====
GfgW = pile{{"raw umber",3},{"red earth",1.2},{"bone black",0.5},{"yellow ochre",0.2}, medium=0.8}
fgGl = fgM - footM:grow(1) - figM:grow(2)
work(fgGl, {hand="glaze", pile=GfgW, angle=crestAng, coverage=2.6, clip=true,
  load_at=function(x,y) local d = y - crestY(x); return clamp(smoothstep(30, 110, d) * 0.95, 0, 1) end})
print(wait(0))

--@ chunk 357
print(wait(10*24*60))
print(drying(300,650), drying(600,650), drying(800,420), drying(700,500))

--@ chunk 358
-- ===== dissolve the vertical seam at x~540 (stipple, density ramps down to the left) =====
seamZ2 = (rect(430, 400, 200, 175) - guardT - fgM)
PWs = pile{{"lead white",7},{"pale smalt",0.7},{"red earth",0.16},{"yellow ochre",0.08}}
stipple(seamZ2, {pile=PWs, width=5.0, coverage=function(x,y) return 4.2 * smoothstep(436, 580, x)^1.1 end, feather=0.6, clip=seamZ2,
  pressure={0.4,0.85}, dips={24,0.9,0.3}, cluster=0.0})
stipple(seamZ2, {pile=PWs, width=3.4, coverage=function(x,y) return 3.0 * smoothstep(470, 590, x) end, feather=0.6, clip=seamZ2,
  pressure={0.4,0.85}, dips={24,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 359
GfgD = pile{{"raw umber",3},{"Prussian blue",0.55},{"bone black",1.1},{"red earth",0.2}, medium=0.75}
work(fgGl, {hand="glaze", pile=GfgD, angle=crestAng, coverage=2.8, clip=true,
  load_at=function(x,y) local d = y - crestY(x); return clamp(0.15 + 0.85*smoothstep(10, 110, d), 0, 1) end})
blend(fgGl, {angle=0.12})
blend(fgGl, {angle=0.3})
print(wait(0))

--@ chunk 360
spruceFull(896, crestY(896)+3, 104, 46, 0.015, pineD)
spruceFull(951, crestY(951)+3, 70, 32, -0.02, pineD)
spruceFull(846, crestY(846)+3, 40, 20, 0.03, pineD)
print(wait(0))

--@ chunk 361
pineD2 = pile{{"raw umber",1.4},{"bone black",0.9},{"green earth",0.45},{"Prussian blue",0.18}}
function spruceRag(x, yb, h, w, lean, pile_, density)
  lean = lean or 0
  local ytop = yb - h
  local N = math.floor(h * (density or 0.9))
  for k = 1, N do
    local t = rand(0.04, 0.99)^0.8            -- fraction down from the top; denser lower
    local y = ytop + h * t
    local tx = x + lean*h*(1 - t)
    local Lmax = (w/2) * (0.12 + 0.88*t^0.85)
    local side = (math.random() < 0.5) and -1 or 1
    local L = Lmax * rand(0.75, 1.45)
    local droop = L * rand(0.35, 0.95)
    local pts = {{tx, y}, {tx + side*L*0.4, y + droop*0.1}, {tx + side*L*0.75, y + droop*0.45}, {tx + side*L, y + droop}}
    spB:load(pile_, 0.9)
    spB:stroke(pts, {pressure={0.5, 0.1}, ramps={0.0, 0.0}})
    -- hanging fringe from the outer half of the bough
    local nf = math.random(1, 3)
    for j = 1, nf do
      local u = rand(0.45, 1.0)
      local fx = tx + side*L*u
      local fy = y + droop*(u^1.3)
      local fl = rand(0.12, 0.34) * L + 2
      spB:load(pile_, 0.8)
      spB:stroke({{fx, fy}, {fx + side*fl*0.12, fy + fl*0.55}, {fx + side*fl*0.2, fy + fl}}, {pressure={0.36, 0.08}, ramps={0.0, 0.0}})
    end
  end
  -- a ragged leader
  spB:load(pile_, 1.0)
  spB:stroke({{x + lean*h*0.97, ytop + 8}, {x + lean*h, ytop - 2}, {x + lean*h + rand(-0.8,0.8), ytop - 7}}, {pressure={0.4, 0.05}, ramps={0.0, 0.0}})
end
spruceRag(896, crestY(896)+3, 104, 46, 0.015, pineD2, 1.0)
spruceRag(951, crestY(951)+3, 70, 32, -0.02, pineD2, 1.0)
spruceRag(846, crestY(846)+3, 40, 20, 0.03, pineD2, 1.0)
print(wait(0))

--@ chunk 362
-- ===== FINAL FOG COAT (whole right side, one wet-in-wet session) =====
sp1 = poly({{896,578},{924,690},{868,690}}):grow(3)
sp2 = poly({{951,630},{970,710},{932,710}}):grow(3)
sp3 = poly({{846,618},{859,668},{833,668}}):grow(3)
spAll = sp1 + sp2 + sp3
footLine = function(x) return 424 + 5*math.sin(x/8.5 + 1) + 3*trn(x*1.3, 17) end
cliffFootFog = cliffM * mask(function(x,y) return (y >= footLine(x)) and 1 or 0 end)
FogF = mask(function(x,y)
  if x < 380 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guardT - (cliffM - cliffFootFog):grow(0.8) - spAll
print("FogF area", FogF:area())
xfade2 = function(x) return clamp(0.12 + 0.88*smoothstep(380, 530, x), 0, 1) end
function ff(p, f, cov, tool)
  work(FogF, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y) * xfade2(x), 0, 1) end})
end
ff(PW2b, function(x,y) return 1 end, 5.6)
ff(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
ff(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
ff(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
ff(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
ff(PW4,  function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
blend(FogF, {angle=0.0})
blend(FogF, {angle=0.02})
blend(FogF, {angle=-0.02})
blend(FogF, {angle=0.0})
print(wait(0))

--@ chunk 363
print(wait(14*24*60))
print(drying(700,500), drying(850,540), drying(600,480), drying(800,420), drying(300,650))

--@ chunk 364
-- ===== butte: a cool haze glaze, graded (thin at the top, deeper toward the foot) =====
HzP  = pile{{"lead white",6},{"pale smalt",1.0},{"red earth",0.25},{"yellow ochre",0.1}, medium=0.7}
HzW  = pile{{"lead white",6},{"yellow ochre",0.3},{"red earth",0.25},{"vermilion",0.08}, medium=0.7}
cliffPlus = cliffM + crownM:grow(0.5) + needleM + towerK + step
hzLoad = function(x,y) return clamp(0.32 + 0.6*smoothstep(340, 420, y), 0, 1) end
work(cliffPlus, {hand="glaze", pile=HzP, angle=1.5708, coverage=2.4, clip=true, load_at=hzLoad})
-- a touch of warmth kept on the lit left shoulder
work(cliffPlus * mask(function(x,y) return 1 - smoothstep(742, 800, x) end), {hand="glaze", pile=HzW, angle=1.5708, coverage=1.6, clip=true,
  load_at=function(x,y) return clamp(0.5*(1 - smoothstep(340, 400, y)), 0, 1) end})
print(wait(0))

--@ chunk 365
-- ===== butte, final: one quiet hazy lilac mass, painted wet-in-wet, blended =====
ButBody  = {["smalt"]=1.2, ["raw umber"]=0.4, ["lead white"]=3.0, ["red earth"]=0.35, ["cobalt blue"]=0.15}
ButShade = {["smalt"]=1.4, ["raw umber"]=0.7, ["lead white"]=1.7, ["red earth"]=0.3, ["Prussian blue"]=0.08}
ButWarm  = {["lead white"]=3.0, ["red earth"]=0.6, ["yellow ochre"]=0.45, ["smalt"]=0.35, ["vermilion"]=0.12}
bPlus = cliffM + needleM + towerK + step
bShade = bPlus * mask(function(x,y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end)
bWarm  = bPlus * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x)) end)
-- 1. whole mass
work(bPlus, {hand="body", tool="filbert 8", pile=mixp(ButBody, tFogC, 0.0), angle=0.0, coverage=6.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08})
-- 2. shade on the right (irregular edge), 3. warm on the left
work(bShade, {hand="body", tool="filbert 8", pile=mixp(ButShade, tFogC, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.5 + 0.5*smoothstep(shEdge(x,y), 870, x), 0, 1) end})
work(bWarm, {hand="body", tool="filbert 8", pile=mixp(ButWarm, tFogW, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.85*(1 - 0.55*smoothstep(345, 420, y)), 0, 1) end})
-- 4. the haze deepens toward the foot: bands of the same three paints mixed toward the fog
for i = 2, #fr do
  local y0, y1 = bandsY[i][1], bandsY[i][2]
  local prof = function(y)
    local up = smoothstep(y0, y0 + 0.45*(y1-y0), y)
    local dn = (i == #fr) and 1 or (1 - smoothstep(y0 + 0.7*(y1-y0), y1, y))
    return clamp(up*dn, 0, 1)
  end
  local bm = rect(690, y0 - 2, 240, (y1 - y0) + 4)
  work(bPlus * bm, {hand="body", tool="filbert 8", pile=mixp(ButBody, tFogC, fr[i]), angle=0.0, coverage=3.6, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(bShade * bm, {hand="body", tool="filbert 8", pile=mixp(ButShade, tFogC, fr[i]), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(bWarm * bm, {hand="body", tool="filbert 8", pile=mixp(ButWarm, tFogW, fr[i]), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
end
blend(bPlus, {angle=0.0})
blend(bPlus, {angle=0.0})
blend(bPlus, {angle=1.5708})
print(wait(0))

--@ chunk 366
print(wait(14*24*60))
print(drying(800,380), drying(800,420), drying(800,330), drying(380,500), drying(600,480))

--@ chunk 367
-- ===== butte: a violet-blue glaze, strongest at the top, nil at the foot; shape and edges untouched =====
GvB = pile{{"smalt",1.4},{"cobalt blue",0.5},{"raw umber",0.35},{"red earth",0.25},{"lead white",0.4}, medium=0.85}
GvW = pile{{"red earth",0.6},{"yellow ochre",0.45},{"lead white",1.2},{"vermilion",0.2}, medium=0.85}
work(bPlus, {hand="glaze", pile=GvB, angle=1.5708, coverage=2.6, clip=true,
  load_at=function(x,y) return clamp((1 - smoothstep(340, 424, y))^1.1 * (0.35 + 0.65*smoothstep(755, 850, x)) , 0, 1) end})
work(bPlus * mask(function(x,y) return 1 - smoothstep(742, 790, x) end), {hand="glaze", pile=GvW, angle=1.5708, coverage=1.8, clip=true,
  load_at=function(x,y) return clamp(0.8*(1 - smoothstep(345, 410, y)), 0, 1) end})
print(wait(0))

--@ chunk 368
footBank = mask(function(x,y)
  local t = 414 + 7*fogwisp(x*1.1, 51) + 4*math.sin(x/17)
  return smoothstep(t-1, t+1, y) * ((y < 470) and 1 or 0) * sb(x, 672, 700, 940, 968)
end)
work(footBank, {hand="body", tool="filbert 20", pile=PW2b, angle=0.0, coverage=3.2, edge="lost", fill=false, length={50,130}, angle_jitter=0.04})
print(wait(0))

--@ chunk 369
-- ===== dissolve the hard edges of the fog bank (dry-on-dry stipple of the surrounding fog colour, density falling inward) =====
PWsur = pile{{"lead white",6.2},{"smalt",0.75},{"red earth",0.28},{"yellow ochre",0.08}}
bankReg = (rect(630, 420, 380, 70) - fgM - guardT - cliffM:grow(1.0))
bankIn = function(x,y) return math.min(x - 652, 988 - x, 481 - y) end
bankDens = function(x,y)
  local d = bankIn(x,y)
  local v = smoothstep(416, 452, y)
  return 4.6 * (1 - smoothstep(-3, 30, d)) * v
end
stipple(bankReg, {pile=PWsur, width=4.2, coverage=bankDens, feather=0.5, clip=bankReg, pressure={0.3,0.75}, dips={30,0.9,0.3}, cluster=0.0})
stipple(bankReg, {pile=PWsur, width=2.6, coverage=function(x,y) return 0.9*bankDens(x,y) end, feather=0.5, clip=bankReg, pressure={0.3,0.7}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 370
-- extension bands: continue the film beyond the rect boundaries and fade it out
extB = rect(590, 486, 430, 36) - fgM - guardT
extL = rect(580, 416, 54, 80) - fgM - guardT - cliffM:grow(1.0)
densB = function(x,y) return 4.6 * (1 - smoothstep(0, 24, y - 490)) * smoothstep(596, 640, x) end
densL = function(x,y) return 4.6 * (1 - smoothstep(0, 40, 630 - x)) * smoothstep(416, 452, y) * (1 - smoothstep(484, 494, y)) end
stipple(extB, {pile=PWsur, width=4.2, coverage=densB, feather=0.5, clip=extB, pressure={0.3,0.75}, dips={30,0.9,0.3}, cluster=0.0})
stipple(extL, {pile=PWsur, width=4.2, coverage=densL, feather=0.5, clip=extL, pressure={0.3,0.75}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 371
print(wait(10*24*60))
print(drying(800,380), drying(800,430), drying(700,470), drying(600,500), drying(850,560), drying(330,500))

--@ chunk 372
-- ===== exact guards: only the real twigs/limbs/figure are spared, no halos =====
GRID = 12
gidx = {}
local nseg = 0
for _, pts in ipairs(TWIGS) do
  for i = 1, #pts-1 do
    local a, b = pts[i], pts[i+1]
    if math.max(a[1], b[1]) > 270 and math.min(a[1], b[1]) < 720 and math.max(a[2], b[2]) > 330 and math.min(a[2], b[2]) < 590 then
      local s = {a[1], a[2], b[1], b[2]}
      nseg = nseg + 1
      local x0, x1 = math.floor((math.min(a[1], b[1]) - 4)/GRID), math.floor((math.max(a[1], b[1]) + 4)/GRID)
      local y0, y1 = math.floor((math.min(a[2], b[2]) - 4)/GRID), math.floor((math.max(a[2], b[2]) + 4)/GRID)
      for gx = x0, x1 do
        for gy = y0, y1 do
          local k = gx*1000 + gy
          local c = gidx[k]
          if not c then c = {}; gidx[k] = c end
          c[#c+1] = s
        end
      end
    end
  end
end
print("segments", nseg)
twigThin = mask(function(x, y)
  if x < 270 or x > 720 or y < 330 or y > 590 then return 0 end
  local c = gidx[math.floor(x/GRID)*1000 + math.floor(y/GRID)]
  if not c then return 0 end
  for i = 1, #c do
    if segDist2(x, y, c[i]) <= 2.4 then return 1 end
  end
  return 0
end)
guardX = limbsKeep:grow(0.4) + twigThin + figM:grow(1.2) + footM:grow(1.0)
print("guardX", guardX:area(), twigThin:area())

--@ chunk 373
-- ===== the whole fog coat again, left boundary at the trunk, exact guards, one session =====
FogX = mask(function(x,y)
  if x < 282 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guardX - (cliffM - cliffFootFog):grow(0.8) - spAll
print("FogX area", FogX:area())
xfade3 = function(x) return clamp(0.20 + 0.80*smoothstep(282, 380, x), 0, 1) end
function fx_(p, f, cov, tool)
  work(FogX, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y) * xfade3(x), 0, 1) end})
end
fx_(PW2b, function(x,y) return 1 end, 5.8)
fx_(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
fx_(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fx_(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
fx_(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fx_(PW4,  function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
print(wait(0))

--@ chunk 374
blend(FogX, {angle=0.0})
blend(FogX, {angle=0.02})
blend(FogX, {angle=-0.02})
blend(FogX, {angle=0.0})
print(wait(0))

--@ chunk 375
-- ===== the butte, once more: darker, bluer, soft; one opaque wet-in-wet session over the dry old coat =====
bNew = ((cliffM + needleM + towerK + step):grow(2.2)) * mask(function(x,y) return (y <= footLine(x) + 1.5) and 1 or 0 end)
print("bNew area", bNew:area())
NB = {}
NB.body  = {["smalt"]=1.7, ["raw umber"]=0.5, ["lead white"]=1.8, ["red earth"]=0.35, ["cobalt blue"]=0.2}
NB.shade = {["smalt"]=1.6, ["raw umber"]=0.85, ["lead white"]=1.0, ["red earth"]=0.3, ["Prussian blue"]=0.1}
NB.warm  = {["lead white"]=2.4, ["red earth"]=0.7, ["yellow ochre"]=0.5, ["smalt"]=0.45, ["vermilion"]=0.12}
nShade = bNew * mask(function(x,y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end)
nWarm  = bNew * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x)) end)
-- zero haze everywhere first: full-strength colours
work(bNew,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, 0.0), angle=0.0, coverage=6.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08})
work(nShade, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.55 + 0.45*smoothstep(shEdge(x,y), 872, x), 0, 1) end})
work(nWarm,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.85*(1 - 0.5*smoothstep(345, 420, y)), 0, 1) end})
-- haze bands toward the foot
hb = {{0.18, 358, 392}, {0.40, 380, 410}, {0.62, 398, 421}, {0.82, 410, 430}}
for i, b in ipairs(hb) do
  local f, y0, y1 = b[1], b[2], b[3]
  local prof = function(y)
    local up = smoothstep(y0, y0 + 0.45*(y1-y0), y)
    local dn = (i == #hb) and 1 or (1 - smoothstep(y0 + 0.75*(y1-y0), y1, y))
    return clamp(up*dn, 0, 1)
  end
  local bm = rect(690, y0 - 2, 240, (y1 - y0) + 4)
  work(bNew * bm,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, f), angle=0.0, coverage=3.6, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(nShade * bm, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, f), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(nWarm * bm,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, f), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
end
blend(bNew, {angle=1.5708})
blend(bNew, {angle=1.5708})
blend(bNew, {angle=0.0})
print(wait(0))

--@ chunk 376
print(drying(800,400), drying(800,420), drying(800,350))
footReg = (bNew:grow(3) * rect(690, 372, 240, 70)) - guardX
fogNear = pile{{"lead white",6.6},{"pale smalt",0.55},{"red earth",0.18},{"yellow ochre",0.1}}
stipple(footReg, {pile=fogNear, width=2.8, coverage=function(x,y) return 8.0 * smoothstep(384, footLine(x)+1, y)^1.15 end, feather=0.6, clip=footReg,
  pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
stipple(footReg, {pile=fogNear, width=4.6, coverage=function(x,y) return 6.0 * smoothstep(404, footLine(x)+2, y)^1.1 end, feather=0.5, clip=footReg,
  pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 377
stipple(footReg, {pile=fogNear, width=3.6, coverage=function(x,y) return 16.0 * smoothstep(392, footLine(x)+1, y)^1.0 end, feather=0.5, clip=footReg,
  pressure={0.4,0.9}, dips={30,0.9,0.3}, cluster=0.0})
lowCover = (bNew:grow(3.5) * mask(function(x,y) return (y >= footLine(x) - 15) and 1 or 0 end)) - guardX
work(lowCover, {hand="body", tool="filbert 14", pile=fogNear, angle=0.0, coverage=3.2, edge="lost", fill=true, length={30,90}, angle_jitter=0.05})
print(wait(0))

--@ chunk 378
bird(668, 168, 10.5, -0.10, 1.0)
bird(694, 152, 8.5, 0.06, 0.8)
bird(712, 176, 9.5, -0.14, 1.1)
bird(738, 160, 6.5, 0.10, 0.9)
bird(764, 189, 5.5, -0.06, 0.7)
bird(640, 146, 6.0, 0.08, 1.0)
bird(785, 138, 4.5, -0.1, 0.9)
print(wait(0))

--@ chunk 379
print(wait(14*24*60))
print(drying(800,380), drying(800,420), drying(700,470), drying(450,500), drying(660,380))

--@ chunk 380
GRID2 = 12
gidx2 = {}
local nseg = 0
for _, pts in ipairs(TWIGS) do
  for i = 1, #pts-1 do
    local a, b = pts[i], pts[i+1]
    if math.max(a[1], b[1]) > 260 and math.min(a[1], b[1]) < 790 and math.max(a[2], b[2]) > 240 and math.min(a[2], b[2]) < 430 then
      local s = {a[1], a[2], b[1], b[2]}
      nseg = nseg + 1
      local x0, x1 = math.floor((math.min(a[1], b[1]) - 4)/GRID2), math.floor((math.max(a[1], b[1]) + 4)/GRID2)
      local y0, y1 = math.floor((math.min(a[2], b[2]) - 4)/GRID2), math.floor((math.max(a[2], b[2]) + 4)/GRID2)
      for gx = x0, x1 do
        for gy = y0, y1 do
          local k = gx*1000 + gy
          local c = gidx2[k]
          if not c then c = {}; gidx2[k] = c end
          c[#c+1] = s
        end
      end
    end
  end
end
print("segments", nseg)
twigThin2 = mask(function(x, y)
  if x < 260 or x > 790 or y < 240 or y > 430 then return 0 end
  local c = gidx2[math.floor(x/GRID2)*1000 + math.floor(y/GRID2)]
  if not c then return 0 end
  for i = 1, #c do
    if segDist2(x, y, c[i]) <= 3.0 then return 1 end
  end
  return 0
end)
guardG = limbsKeep:grow(1.2) + twigThin2
glowReg = (rect(280, 286, 740, 140) * aboveRidge) - (cliffM + crownM + needleM + towerK + step):grow(3.5) - guardG
print("glowReg", glowReg:area(), "guardG", guardG:area())

--@ chunk 381
GlowP = pile{{"vermilion",0.7},{"chrome yellow",0.75},{"red earth",0.08},{"lead white",1.1}, medium=0.85}
glowCov = function(x,y)
  local t = Atop(x)
  return 3.4 * bump(x, 650, 230) * smoothstep(t-80, t-2, y)^1.4
end
stipple(glowReg, {pile=GlowP, width=6, coverage=glowCov, feather=0.7, clip=glowReg, pressure={0.3,0.7}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 382
blendG = (rect(280, 286, 740, 140) * aboveRidge) - (cliffM + crownM + needleM + towerK + step):grow(3.5) - guardG
blend(blendG, {angle=0.0})
blend(blendG, {angle=0.0})
blend(blendG, {angle=0.03})
blend(blendG, {angle=-0.03})
blend(blendG, {angle=0.0})
print(wait(0))

--@ chunk 383
-- ===== the butte: faint fissures and ribs (detail brush, low contrast, dry-on-dry) =====
FisL = pile{{"smalt",1.5},{"raw umber",0.7},{"lead white",1.3},{"red earth",0.3},{"Prussian blue",0.06}}
RibL = pile{{"lead white",2.2},{"red earth",0.5},{"yellow ochre",0.45},{"smalt",0.35},{"vermilion",0.1}}
local function ribw(x0, y0, y1, w0, w1, wob, seed)
  local pts, ws = {}, {}
  local n = math.max(3, math.floor((y1-y0)/8))
  for i = 0, n do
    local t = i/n
    pts[#pts+1] = {x0 + wob*math.sin(i*1.7 + seed) + rand(-0.5,0.5), y0 + (y1-y0)*t}
    ws[#ws+1] = w0 + (w1-w0)*math.sin(3.14*t)^0.7
  end
  return ribbon(pts, ws)
end
fis = ribw(752, 346, 396, 0.8, 1.8, 0.9, 1) + ribw(765, 358, 408, 0.7, 1.5, 0.8, 2) + ribw(778, 336, 390, 0.8, 2.0, 1.0, 3)
    + ribw(792, 340, 400, 0.8, 1.6, 0.9, 4) + ribw(806, 336, 404, 0.9, 2.2, 1.0, 5) + ribw(820, 340, 396, 0.8, 1.8, 0.9, 6)
    + ribw(834, 330, 392, 0.9, 2.2, 1.0, 7) + ribw(848, 336, 400, 0.8, 1.8, 0.9, 8) + ribw(860, 344, 392, 0.7, 1.5, 0.8, 9)
    + ribw(799, 384, 414, 0.7, 1.2, 0.7, 10) + ribw(842, 380, 414, 0.7, 1.3, 0.7, 11)
cliffFace = cliffM * mask(function(x,y) return (y < footLine(x) - 8) and 1 or 0 end)
work(fis * cliffFace, {hand="detail", pile=FisL, coverage=1.0, clip=true, angle=1.5708, length={10,30}, pressure={0.18,0.32}, load=0.6,
  load_at=function(x,y) return clamp(0.35 + 0.65*smoothstep(780, 850, x) * (1 - 0.6*smoothstep(380, 420, y)), 0, 1) end})
-- a few warm ribs on the lit left flank
rib = ribw(748, 344, 384, 0.7, 1.6, 0.7, 12) + ribw(758, 340, 372, 0.6, 1.4, 0.7, 13) + ribw(771, 346, 380, 0.6, 1.3, 0.7, 14)
work(rib * cliffFace, {hand="detail", pile=RibL, coverage=1.0, clip=true, angle=1.5708, length={8,24}, pressure={0.15,0.28}, load=0.5})
print(wait(0))

--@ chunk 384
-- ===== soft wooded hills emerging from the fog: dry-on-dry stipple, crisp ragged tops, density falling downward and sideways =====
HillP  = pile{{"lead white",3.3},{"smalt",1.6},{"red earth",0.36},{"raw umber",0.14},{"Prussian blue",0.03}}
HillP2 = pile{{"lead white",2.7},{"smalt",1.8},{"red earth",0.34},{"raw umber",0.2},{"Prussian blue",0.05}}
function stipHill(pts, amp, seed, x0, x1, depth, p1, p2, strength)
  local topf = function(x) return interp(pts, x) + amp*trn(x*1.0 + seed, 3.0) end
  local m = mask(function(x,y)
    if x < x0 or x > x1 then return 0 end
    local t = topf(x)
    return smoothstep(t-0.5, t+0.5, y) * ((y < t + depth) and 1 or 0)
  end) - guardX - fgM - spAll - cliffM:grow(2)
  local side = function(x) return smoothstep(x0, x0 + 0.28*(x1-x0), x) * (1 - smoothstep(x1 - 0.28*(x1-x0), x1, x)) end
  local dens = function(x,y)
    local t = topf(x)
    local d = (y - t) / depth
    return strength * (1 - smoothstep(0.0, 1.0, d))^1.3 * side(x)
  end
  stipple(m, {pile=p1, width=3.2, coverage=function(x,y) return dens(x,y) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  stipple(m, {pile=p2, width=2.2, coverage=function(x,y) return 0.8*dens(x,y)*(1 - smoothstep(0.0, 0.45, (y - topf(x))/depth)) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  return m
end
hillB = stipHill({{612,506},{640,494},{668,487},{700,484},{732,489},{764,498},{796,510},{820,522}}, 2.0, 7, 612, 822, 46, HillP, HillP2, 7.0)
hillA = stipHill({{870,526},{900,508},{932,498},{966,494},{1000,497},{1012,500}}, 1.8, 17, 868, 1012, 50, HillP, HillP2, 7.0)
print(wait(0))

--@ chunk 385
-- ===== left side: exact guard from the recorded twigs, x < 340 =====
GRIDL = 12
gl = {}
local nsegL = 0
for _, pts in ipairs(TWIGS) do
  for i = 1, #pts-1 do
    local a, b = pts[i], pts[i+1]
    if math.min(a[1], b[1]) < 345 and math.max(a[1], b[1]) > -10 and math.max(a[2], b[2]) > 330 and math.min(a[2], b[2]) < 590 then
      local s = {a[1], a[2], b[1], b[2]}
      nsegL = nsegL + 1
      local x0, x1 = math.floor((math.min(a[1], b[1]) - 4)/GRIDL), math.floor((math.max(a[1], b[1]) + 4)/GRIDL)
      local y0, y1 = math.floor((math.min(a[2], b[2]) - 4)/GRIDL), math.floor((math.max(a[2], b[2]) + 4)/GRIDL)
      for gx = x0, x1 do
        for gy = y0, y1 do
          local k = (gx + 5)*1000 + gy
          local c = gl[k]
          if not c then c = {}; gl[k] = c end
          c[#c+1] = s
        end
      end
    end
  end
end
print("segments", nsegL)
twigL = mask(function(x, y)
  if x < -10 or x > 345 or y < 330 or y > 590 then return 0 end
  local c = gl[(math.floor(x/GRIDL) + 5)*1000 + math.floor(y/GRIDL)]
  if not c then return 0 end
  for i = 1, #c do
    if segDist2(x, y, c[i]) <= 2.6 then return 1 end
  end
  return 0
end)
guardL = limbsKeep:grow(0.5) + twigL + footM:grow(1.0)
LR = mask(function(x,y)
  if x < -8 or x > 335 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guardL
print("LR", LR:area(), "guardL", guardL:area(), "twigL", twigL:area())
print(drying(100,450), drying(250,500), drying(300,480), drying(150,400))

--@ chunk 386
function lw(p, f, cov, tool, m)
  work(m or LR, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={60,200}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y), 0, 1) end})
end
-- 1. cream-lilac fog base everywhere on the left
lw(PW2b, function(x,y) return 1 end, 5.2)
-- 2. far ridge A: pale lilac, crisp top, dissolving downward (same recipe as on the right)
lw(PAc, function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+40, y)) * (1 - smoothstep(290, 345, x)) end, 3.4, "filbert 14")
-- 3. warm cream under it
lw(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+100, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * 0.7 end, 2.6)
-- 4. second hazy ridge (A2L), crisp top
A2bandL = LR * mask(function(x,y) local t = A2Lt(x); return (y >= t - 0.4 and y <= t + 72) and 1 or 0 end)
lw(PA6, function(x,y) return 0.9*(1 - smoothstep(A2Lt(x)+2, A2Lt(x)+52, y)) * (1 - smoothstep(210, 330, x)) end, 3.6, "filbert 12", A2bandL)
-- 5. hill C with a ragged wooded top, full strength to the crest, fading out on the right
CtopF = function(x) return interp(C3_top, x) + 2.4*trn(x*1.0, 3.0) end
Cband = LR * mask(function(x,y) return smoothstep(CtopF(x)-0.6, CtopF(x)+0.6, y) end)
lw(PC3, function(x,y) return (1 - smoothstep(255, 340, x)) end, 4.6, "filbert 12", Cband)
-- 6. soft mist at the foot of the hill, just above the crest
lw(PW2b, function(x,y) return 0.55 * smoothstep(crestY(x)-46, crestY(x)-3, y) * (1 - smoothstep(255, 335, x)) end, 2.4, "filbert 14", Cband)
print(wait(0))

--@ chunk 387
print(wait(14*24*60))
print(drying(100,450), drying(250,500), drying(300,480), drying(150,400), drying(320,450))

--@ chunk 388
staffGuard = ribbon({{433.6,547},{432.8,531},{432,515},{431.4,500}}, 3.2)
guardAll = guardX + guardL + staffGuard
FogAll = mask(function(x,y)
  if x < -8 or x > 1012 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guardAll - (cliffM - cliffFootFog):grow(0.8) - spAll
print("FogAll area", FogAll:area())
function fa_(p, f, cov, tool)
  work(FogAll, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y), 0, 1) end})
end
fa_(PW2b, function(x,y) return 1 end, 5.8)
fa_(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
fa_(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fa_(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
fa_(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fa_(PW4,  function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
print(wait(0))

--@ chunk 389
blend(FogAll, {angle=0.0})
blend(FogAll, {angle=0.02})
blend(FogAll, {angle=-0.02})
blend(FogAll, {angle=0.0})
print(wait(0))

--@ chunk 390
print(wait(14*24*60))
print(drying(100,450), drying(250,500), drying(600,480), drying(800,500), drying(330,450))

--@ chunk 391
-- ===== soft hills in the fog, dry-on-dry stipple that dissolves + a blend while it is open =====
clipHills = rect(-12, 395, 1030, 330) - fgM - guardAll - spAll - cliffM:grow(1.5)
function stipHill2(pts, amp, seed, depth, p1, p2, strength, gamma, fadeL, fadeR, xlo, xhi, tw1, tw2)
  local topf = function(x) return interp(pts, x) + amp*trn(x*1.0 + seed, 3.0) end
  local m = mask(function(x,y)
    if x < xlo or x > xhi then return 0 end
    local t = topf(x)
    return smoothstep(t-0.5, t+0.5, y) * ((y < t + depth) and 1 or 0)
  end) * clipHills
  local side = function(x)
    local l = fadeL and smoothstep(fadeL[1], fadeL[2], x) or 1
    local r = fadeR and (1 - smoothstep(fadeR[1], fadeR[2], x)) or 1
    return l * r
  end
  local dens = function(x,y)
    local t = topf(x)
    local d = (y - t) / depth
    return strength * (1 - smoothstep(0.0, 1.0, d))^(gamma or 1.3) * side(x)
  end
  stipple(m, {pile=p1, width=tw1 or 2.6, coverage=function(x,y) return dens(x,y) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  stipple(m, {pile=p2, width=tw2 or 1.8, coverage=function(x,y) return 0.8*dens(x,y)*(1 - smoothstep(0.0, 0.5, (y - topf(x))/depth)) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  blend(m:grow(3) * clipHills, {angle=0.0})
  blend(m:grow(3) * clipHills, {angle=0.03})
  return m
end
A2Lp = pile{{"lead white",3.2},{"smalt",1.5},{"red earth",0.36},{"raw umber",0.1}}
A2Lp2 = pile{{"lead white",2.8},{"smalt",1.7},{"red earth",0.36},{"raw umber",0.14}}
mA2Lx = stipHill2(A2L_top, 1.6, 3, 62, A2Lp, A2Lp2, 9.0, 1.2, nil, {430, 585}, -12, 590)
print(wait(0))

--@ chunk 392
PCp  = pile{{"lead white",2.0},{"smalt",1.9},{"raw umber",0.55},{"red earth",0.3},{"Prussian blue",0.07}}
PCp2 = pile{{"lead white",1.6},{"smalt",2.0},{"raw umber",0.7},{"red earth",0.3},{"Prussian blue",0.1}}
mCx = stipHill2(C3_top, 2.2, 9, 95, PCp, PCp2, 11.0, 0.9, nil, {255, 372}, -12, 380, 2.8, 2.0)
print(wait(0))

--@ chunk 393
PCd = pile{{"lead white",1.2},{"smalt",1.9},{"raw umber",0.8},{"red earth",0.3},{"Prussian blue",0.14}}
PCl = pile{{"lead white",2.6},{"smalt",1.5},{"raw umber",0.4},{"yellow ochre",0.25},{"red earth",0.3}}
CtopFx = function(x) return interp(C3_top, x) + 2.2*trn(x*1.0 + 9, 3.0) end
-- crown dabs along the upper face of hill C (dark), thinning downward
cM = mCx * mask(function(x,y) return (1 - smoothstep(CtopFx(x)+4, CtopFx(x)+34, y)) end)
work(cM, {hand="body", tool="filbert 5", pile=PCd, angle=1.4, coverage=3.0, clip=true, fill=false, length={3,9}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(CtopFx(x)+3, CtopFx(x)+30, y)) * (1 - smoothstep(230, 340, x)) * (0.6 + 0.4*cn1(x*1.3, y*1.3)), 0, 1) end})
-- sunlit crowns on the glow side
work(cM, {hand="body", tool="filbert 4", pile=PCl, angle=1.4, coverage=1.4, clip=true, length={3,7}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(CtopFx(x)+1, CtopFx(x)+14, y)) * smoothstep(110, 230, x) * (1 - smoothstep(250, 340, x)) * (0.4 + 0.6*cn2(x*1.3, y*1.3)), 0, 1) end})
-- conifer spikes along the skyline
for i = 1, 16 do
  local xx = rand(8, 250)
  local t = CtopFx(xx)
  local h = rand(7, 15)
  work(tinyPine(xx, t + 4, h, h*0.45), {hand="detail", pile=PCd, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 394
dotFix = ellipse(234, 491, 5, 7) * (trunkClean:grow(0.5))
work(dotFix, {hand="detail", pile=OakW2, coverage=4.0, clip=true, angle=1.5708, length={3,8}, pressure={0.5,0.9}})
print(wait(0))

--@ chunk 395
HRp  = pile{{"lead white",3.4},{"smalt",1.5},{"red earth",0.34},{"raw umber",0.12},{"Prussian blue",0.02}}
HRp2 = pile{{"lead white",2.9},{"smalt",1.7},{"red earth",0.34},{"raw umber",0.18},{"Prussian blue",0.04}}
HNp  = pile{{"lead white",2.4},{"smalt",1.8},{"red earth",0.3},{"raw umber",0.3},{"Prussian blue",0.06}}
HNp2 = pile{{"lead white",2.0},{"smalt",1.9},{"red earth",0.3},{"raw umber",0.42},{"Prussian blue",0.1}}
hR1 = stipHill2({{596,512},{640,498},{680,490},{720,490},{760,498},{800,510},{830,524}}, 2.0, 41, 66, HRp, HRp2, 9.0, 1.15, {596, 650}, {770, 840}, 590, 845, 2.6, 1.8)
hR2 = stipHill2({{872,520},{905,506},{940,498},{975,494},{1012,497}}, 1.8, 51, 66, HRp, HRp2, 9.0, 1.15, {872, 925}, nil, 868, 1012, 2.6, 1.8)
print(wait(0))

--@ chunk 396
PWend = pile{{"lead white",6.0},{"pale smalt",0.7},{"red earth",0.2},{"yellow ochre",0.1}}
function endFix(xc, dir, y0, y1, reach, strength)
  local m = (rect(math.min(xc, xc + dir*reach) - 14, y0, reach + 28, y1 - y0)) * clipHills
  local dens = function(x,y)
    local d = (x - xc) * dir          -- distance inward from the cut (positive inside the hill)
    local base = 1 - smoothstep(-6, reach, d)
    return strength * clamp(base, 0, 1) * smoothstep(y0, y0 + 16, y) * (1 - smoothstep(y1 - 20, y1, y))
  end
  stipple(m, {pile=PWend, width=4.4, coverage=dens, feather=0.5, clip=m, pressure={0.3,0.8}, dips={30,0.9,0.3}, cluster=0.0})
  stipple(m, {pile=PWend, width=2.6, coverage=function(x,y) return 0.8*dens(x,y) end, feather=0.5, clip=m, pressure={0.3,0.8}, dips={30,0.9,0.3}, cluster=0.0})
end
endFix(592, 1, 482, 580, 64, 9.0)    -- hR1 left end: inward is +x
endFix(845, -1, 482, 580, 72, 9.0)   -- hR1 right end: inward is -x
endFix(868, 1, 488, 580, 64, 9.0)    -- hR2 left end
print(wait(0))

--@ chunk 397
print(drying(700,520), drying(900,520), drying(300,500), drying(600,480))
fa_(PW2b, function(x,y) return 1 end, 5.8)
fa_(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
fa_(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fa_(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
fa_(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fa_(PW4,  function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
blend(FogAll, {angle=0.0})
blend(FogAll, {angle=0.02})
blend(FogAll, {angle=-0.02})
blend(FogAll, {angle=0.0})
print(wait(0))

--@ chunk 398
print(wait(16*24*60))
print(drying(700,520), drying(900,520), drying(300,500), drying(600,480), drying(500,450), drying(150,430))

--@ chunk 399
clipHills = rect(-12, 395, 1030, 330) - fgM - guardAll - spAll - cliffM:grow(1.5)
-- a hill that is a closed bump: its top profile reaches the fog level at both ends, so there is no vertical cut anywhere
function bumpHill(pts, amp, seed, depth, p1, p2, strength, gamma, tw1, tw2)
  local x0, x1 = pts[1][1], pts[#pts][1]
  local topf = function(x) return interp(pts, x) + amp*trn(x*1.0 + seed, 3.0) * smoothstep(x0, x0+30, x) * (1 - smoothstep(x1-30, x1, x)) end
  local m = mask(function(x,y)
    if x < x0 or x > x1 then return 0 end
    local t = topf(x)
    return smoothstep(t-0.5, t+0.5, y) * ((y < t + depth) and 1 or 0)
  end) * clipHills
  local dens = function(x,y)
    local t = topf(x)
    local d = (y - t) / depth
    local hh = clamp((pts[1][2] - t) / 40, 0, 1)    -- shallow near the ends: little paint where the bump meets the fog
    return strength * (1 - smoothstep(0.0, 1.0, d))^(gamma or 1.3) * hh^0.7
  end
  stipple(m, {pile=p1, width=tw1 or 2.6, coverage=function(x,y) return dens(x,y) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  stipple(m, {pile=p2, width=tw2 or 1.8, coverage=function(x,y) return 0.8*dens(x,y)*(1 - smoothstep(0.0, 0.5, (y - topf(x))/depth)) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  blend(m:grow(3) * clipHills, {angle=0.0})
  blend(m:grow(3) * clipHills, {angle=0.03})
  return m, topf
end
A2Lb = {{-12,452},{50,431},{110,425},{170,421},{230,425},{290,431},{345,429},{395,434},{445,441},{495,449},{540,462},{580,490}}
mA2Lb, A2Lfn = bumpHill(A2Lb, 1.6, 3, 62, A2Lp, A2Lp2, 9.0, 1.2)
print(wait(0))

--@ chunk 400
function bumpHill2(pts, amp, seed, depth, p1, p2, strength, gamma, tw1, tw2, ybase)
  local x0, x1 = pts[1][1], pts[#pts][1]
  local topf = function(x) return interp(pts, x) + amp*trn(x*1.0 + seed, 3.0) end
  local m = mask(function(x,y)
    if x < x0 or x > x1 then return 0 end
    local t = topf(x)
    return smoothstep(t-0.5, t+0.5, y) * ((y < t + depth) and 1 or 0)
  end) * clipHills
  local dens = function(x,y)
    local t = topf(x)
    local d = (y - t) / depth
    local hh = clamp((ybase - t) / 40, 0, 1)
    return strength * (1 - smoothstep(0.0, 1.0, d))^(gamma or 1.3) * hh^0.7
  end
  stipple(m, {pile=p1, width=tw1 or 2.6, coverage=function(x,y) return dens(x,y) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  stipple(m, {pile=p2, width=tw2 or 1.8, coverage=function(x,y) return 0.8*dens(x,y)*(1 - smoothstep(0.0, 0.5, (y - topf(x))/depth)) end, feather=0.6, clip=m, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
  blend(m:grow(3) * clipHills, {angle=0.0})
  blend(m:grow(3) * clipHills, {angle=0.03})
  return m, topf
end
C3b = {{-12,494},{40,484},{90,479},{140,478},{190,482},{232,490},{270,501},{300,512},{325,527},{345,545},{368,572}}
mCb, Cfn = bumpHill2(C3b, 2.2, 9, 95, PCp, PCp2, 11.0, 0.9, 2.8, 2.0, 572)
-- wooded crown on the upper face and conifer spikes on the skyline
cM2 = mCb * mask(function(x,y) return (1 - smoothstep(Cfn(x)+4, Cfn(x)+34, y)) end)
work(cM2, {hand="body", tool="filbert 5", pile=PCd, angle=1.4, coverage=3.0, clip=true, fill=false, length={3,9}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(Cfn(x)+3, Cfn(x)+30, y)) * (1 - smoothstep(230, 340, x)) * (0.6 + 0.4*cn1(x*1.3, y*1.3)), 0, 1) end})
work(cM2, {hand="body", tool="filbert 4", pile=PCl, angle=1.4, coverage=1.4, clip=true, length={3,7}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(Cfn(x)+1, Cfn(x)+14, y)) * smoothstep(90, 200, x) * (1 - smoothstep(250, 340, x)) * (0.4 + 0.6*cn2(x*1.3, y*1.3)), 0, 1) end})
for i = 1, 14 do
  local xx = rand(8, 240)
  local t = Cfn(xx)
  local h = rand(7, 15)
  work(tinyPine(xx, t + 4, h, h*0.45), {hand="detail", pile=PCd, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 401
spot = ellipse(237.5, 491, 7, 12) * trunkClean:grow(0.5)
work(spot, {hand="detail", pile=OakW2, coverage=5.0, clip=true, angle=1.5708, length={3,8}, pressure={0.5,0.9}})
work(spot, {hand="detail", pile=OakW2, coverage=3.0, clip=true, angle=1.4, length={3,8}, pressure={0.5,0.9}})
print(wait(0))

--@ chunk 402
-- ===== right-hand hills in the fog: closed bumps whose ends sit at the fog level (no vertical cuts) =====
R1pts = {{556,531},{592,512},{630,498},{672,490},{715,488},{758,492},{800,504},{836,520},{866,531}}
R2pts = {{872,533},{906,514},{942,502},{980,496},{1012,498},{1030,500}}
mR1, R1fn = bumpHill2(R1pts, 1.8, 61, 60, HRp, HRp2, 9.0, 1.15, 2.6, 1.8, 531)
mR2, R2fn = bumpHill2(R2pts, 1.6, 71, 58, HRp, HRp2, 9.0, 1.15, 2.6, 1.8, 533)
-- wooded crowns on the upper faces
for _, hh in ipairs({{mR1, R1fn, 556, 866}, {mR2, R2fn, 872, 1012}}) do
  local m, fn, xa, xb = hh[1], hh[2], hh[3], hh[4]
  local up = m * mask(function(x,y) return (1 - smoothstep(fn(x)+2, fn(x)+22, y)) end)
  work(up, {hand="body", tool="filbert 4", pile=HNp, angle=1.4, coverage=2.4, clip=true, fill=false, length={3,8}, angle_jitter=0.7,
    load_at=function(x,y) return clamp((1 - smoothstep(fn(x)+2, fn(x)+20, y)) * (0.5 + 0.5*cn1(x*1.3+7, y*1.3)) * (1 - 0.6*smoothstep(xb-40, xb, x)) * smoothstep(xa, xa+40, x), 0, 1) end})
end
-- a few conifer spikes along the skyline of R1
for i = 1, 12 do
  local xx = rand(620, 800)
  local t = R1fn(xx)
  local h = rand(5, 11)
  work(tinyPine(xx, t + 3, h, h*0.45), {hand="detail", pile=HNp, coverage=2.2, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 403
endFix(866, -1, 486, 566, 84, 9.0)    -- R1 right end: inward is -x
endFix(872, 1, 490, 566, 70, 9.0)     -- R2 left end: inward is +x
endFix(556, 1, 496, 566, 60, 9.0)     -- R1 left end
print(wait(0))

--@ chunk 404
print(drying(800,380), drying(800,350))
bP = (cliffM + needleM + towerK + step) * mask(function(x,y) return (y < footLine(x) + 2) and 1 or 0 end)
work(bP, {hand="glaze", pile=GshadeP, angle=1.5708, coverage=2.6, clip=true,
  load_at=function(x,y) return clamp(smoothstep(shEdge(x,y)-14, shEdge(x,y)+16, x) * (1 - 0.65*smoothstep(380, 428, y)) * 0.9, 0, 1) end})
work(bP, {hand="glaze", pile=GlitP, angle=1.5708, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp((1 - smoothstep(745, 792, x)) * (1 - smoothstep(340, 408, y)) * 0.95, 0, 1) end})
print(wait(0))

--@ chunk 405
print(wait(16*24*60))
print(drying(800,380), drying(800,350), drying(700,520), drying(900,520), drying(600,480), drying(300,500))

--@ chunk 406
-- ===== SESSION: the valley fog (cliff above its fog line excluded), then the cliff, wet-in-wet =====
FogAll2 = mask(function(x,y)
  if x < -8 or x > 1012 then return 0 end
  if y < Atop(x) + 0.5 then return 0 end
  return 1
end) - fgM - guardAll - (cliffM - cliffFootFog):grow(0.8) - spAll
function fa2(p, f, cov, tool)
  work(FogAll2, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y), 0, 1) end})
end
fa2(PW2b, function(x,y) return 1 end, 6.0)
fa2(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
fa2(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fa2(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
fa2(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fa2(PW4,  function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
blend(FogAll2, {angle=0.0})
blend(FogAll2, {angle=0.02})
blend(FogAll2, {angle=-0.02})
blend(FogAll2, {angle=0.0})
print(wait(0))

--@ chunk 407
-- ===== the cliff, wet-in-wet (the fog beneath is still open) =====
bNew2 = ((cliffM + needleM + towerK + step):grow(2.2)) * mask(function(x,y) return (y <= footLine(x) + 1.5) and 1 or 0 end)
nShade2 = bNew2 * mask(function(x,y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end)
nWarm2  = bNew2 * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x)) end)
work(bNew2,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, 0.0), angle=0.0, coverage=6.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08})
work(nShade2, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.55 + 0.45*smoothstep(shEdge(x,y), 872, x), 0, 1) end})
work(nWarm2,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.85*(1 - 0.5*smoothstep(345, 420, y)), 0, 1) end})
hb2 = {{0.18, 358, 392}, {0.40, 380, 410}, {0.62, 398, 421}, {0.82, 410, 430}}
for i, b in ipairs(hb2) do
  local f, y0, y1 = b[1], b[2], b[3]
  local prof = function(y)
    local up = smoothstep(y0, y0 + 0.45*(y1-y0), y)
    local dn = (i == #hb2) and 1 or (1 - smoothstep(y0 + 0.75*(y1-y0), y1, y))
    return clamp(up*dn, 0, 1)
  end
  local bm = rect(690, y0 - 2, 240, (y1 - y0) + 4)
  work(bNew2 * bm,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, f), angle=0.0, coverage=3.6, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(nShade2 * bm, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, f), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(nWarm2 * bm,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, f), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
end
blend(bNew2, {angle=1.5708})
blend(bNew2, {angle=1.5708})
blend(bNew2, {angle=0.0})
print(wait(0))

--@ chunk 408
footReg2 = (bNew2:grow(3) * rect(690, 372, 240, 70)) - guardX
stipple(footReg2, {pile=fogNear, width=2.8, coverage=function(x,y) return 8.0 * smoothstep(384, footLine(x)+1, y)^1.15 end, feather=0.6, clip=footReg2,
  pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
stipple(footReg2, {pile=fogNear, width=4.6, coverage=function(x,y) return 6.0 * smoothstep(404, footLine(x)+2, y)^1.1 end, feather=0.5, clip=footReg2,
  pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
stipple(footReg2, {pile=fogNear, width=3.6, coverage=function(x,y) return 16.0 * smoothstep(392, footLine(x)+1, y)^1.0 end, feather=0.5, clip=footReg2,
  pressure={0.4,0.9}, dips={30,0.9,0.3}, cluster=0.0})
lowCover2 = (bNew2:grow(3.5) * mask(function(x,y) return (y >= footLine(x) - 15) and 1 or 0 end)) - guardX
work(lowCover2, {hand="body", tool="filbert 14", pile=fogNear, angle=0.0, coverage=3.2, edge="lost", fill=true, length={30,90}, angle_jitter=0.05})
print(wait(0))

--@ chunk 409
-- ===== left wooded hill in the fog (closed bump, painted last, dry-on-dry) =====
clipHills = rect(-12, 395, 1030, 330) - fgM - guardAll - spAll - cliffM:grow(1.5)
C3c = {{-12,498},{40,487},{90,481},{140,480},{190,484},{232,492},{270,503},{300,514},{325,529},{345,547},{368,572}}
mCc, Cfn2 = bumpHill2(C3c, 2.2, 19, 84, PCp, PCp2, 9.0, 0.95, 2.8, 2.0, 572)
cM3 = mCc * mask(function(x,y) return (1 - smoothstep(Cfn2(x)+4, Cfn2(x)+34, y)) end)
work(cM3, {hand="body", tool="filbert 5", pile=PCd, angle=1.4, coverage=3.0, clip=true, fill=false, length={3,9}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(Cfn2(x)+3, Cfn2(x)+30, y)) * (1 - smoothstep(230, 340, x)) * (0.6 + 0.4*cn1(x*1.3+3, y*1.3)), 0, 1) end})
work(cM3, {hand="body", tool="filbert 4", pile=PCl, angle=1.4, coverage=1.4, clip=true, length={3,7}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(Cfn2(x)+1, Cfn2(x)+14, y)) * smoothstep(60, 170, x) * (1 - smoothstep(250, 340, x)) * (0.4 + 0.6*cn2(x*1.3+5, y*1.3)), 0, 1) end})
for i = 1, 16 do
  local xx = rand(8, 250)
  local t = Cfn2(xx)
  local h = rand(7, 15)
  work(tinyPine(xx, t + 4, h, h*0.45), {hand="detail", pile=PCd, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 410
trunkSpots = trunkClean:grow(0.3) * rect(205, 462, 50, 70)
work(trunkSpots, {hand="body", tool="filbert 6", pile=OakW2, angle=1.5708, coverage=5.0, clip=true, fill=true, length={6,20}, angle_jitter=0.2})
print(wait(0))

--@ chunk 411
-- ===== right: two wooded knolls in the fog, one with a village and a spire =====
K1pts = {{556,536},{580,520},{606,508},{630,500},{652,496},{680,497},{708,502},{734,512},{758,524},{782,540}}
K2pts = {{868,540},{892,525},{918,513},{946,505},{978,501},{1012,501},{1030,503}}
mK1, K1fn = bumpHill2(K1pts, 1.6, 83, 66, HRp, HRp2, 9.0, 1.1, 2.6, 1.8, 538)
mK2, K2fn = bumpHill2(K2pts, 1.5, 97, 62, HRp, HRp2, 9.0, 1.1, 2.6, 1.8, 540)
for _, hh in ipairs({{mK1, K1fn, 556, 782}, {mK2, K2fn, 868, 1012}}) do
  local m, fn, xa, xb = hh[1], hh[2], hh[3], hh[4]
  local up = m * mask(function(x,y) return (1 - smoothstep(fn(x)+2, fn(x)+22, y)) end)
  work(up, {hand="body", tool="filbert 4", pile=HNp, angle=1.4, coverage=2.6, clip=true, fill=false, length={3,8}, angle_jitter=0.7,
    load_at=function(x,y) return clamp((1 - smoothstep(fn(x)+2, fn(x)+20, y)) * (0.5 + 0.5*cn1(x*1.3+7, y*1.3)) * (1 - 0.7*smoothstep(xb-50, xb, x)) * smoothstep(xa, xa+50, x), 0, 1) end})
end
print(wait(0))

--@ chunk 412
-- lift the stiff arches while their stipple is still open: clean wiped filbert, horizontal passes, spares crest, spruces, twigs
liftReg = rect(536, 472, 480, 140) - fgM - guardAll - spAll - cliffM:grow(1.5)
lift3 = brush("filbert", 26)
local n3 = 0
local yy = 484
local row = 0
while yy < 600 do
  for pass = 1, 2 do
    lift3:wipe(1.0)
    local xa, xb = 540, 1012
    if (pass + row) % 2 == 0 then xa, xb = 1012, 540 end
    lift3:stroke({{xa, yy}, {(xa+xb)/2, yy + rand(-1.5,1.5)}, {xb, yy + rand(-1.5,1.5)}}, {pressure={0.5,0.5}, ramps={0.03,0.03}, clip=liftReg})
    n3 = n3 + 1
  end
  yy = yy + 11
  row = row + 1
end
print("lift strokes", n3)
print(wait(0))

--@ chunk 413
print(drying(560,520), drying(700,520), drying(560,500), drying(800,515))
edgeBlend = rect(512, 478, 140, 80) - fgM - guardAll
blend(edgeBlend, {angle=0.0})
blend(edgeBlend, {angle=0.0})
blend(edgeBlend, {angle=0.05})
blend(edgeBlend, {angle=-0.05})
blend(edgeBlend, {angle=0.0})
print(wait(0))

--@ chunk 414
softB = (rect(590, 480, 280, 80):blur(34)) - fgM - guardAll
blend(softB, {angle=0.0})
blend(softB, {angle=0.0})
blend(softB, {angle=0.04})
print(wait(0))

--@ chunk 415
print(wait(16*24*60))
print(drying(560,520), drying(700,520), drying(800,515), drying(900,540), drying(800,380), drying(300,500))

--@ chunk 416
-- ===== FINAL FOG, right valley: one wet-in-wet coat from x=470 (left end fades by load, same colours as what lies beneath) =====
FogR = FogAll2 * mask(function(x,y) return (x >= 470) and 1 or 0 end)
fadeR = function(x) return smoothstep(470, 640, x) end
function fr_(p, f, cov, tool)
  work(FogR, {hand="body", tool=tool or "filbert 22", pile=p, angle=0.0, coverage=cov, clip=true, fill=true, length={80,220}, angle_jitter=0.04,
    load_at=function(x,y) return clamp(f(x,y) * fadeR(x), 0, 1) end})
end
fr_(PW2b, function(x,y) return 1 end, 6.0)
fr_(PAc,  function(x,y) return 0.95*(1 - smoothstep(Atop(x)+3, Atop(x)+38, y)) end, 3.2, "filbert 14")
fr_(PW1b, function(x,y) return (1 - smoothstep(Atop(x)+30, Atop(x)+135, y)) * smoothstep(Atop(x)+8, Atop(x)+34, y) * (0.4 + 0.6*bump(x, 690, 340)) end, 2.8)
fr_(PWpk, function(x,y) return bump(x, 650, 190) * smoothstep(Atop(x)+14, Atop(x)+34, y) * (1 - smoothstep(Atop(x)+40, Atop(x)+90, y)) * 0.8 end, 2.4)
fr_(PW3c, function(x,y) return smoothstep(500, 700, y)^1.1 * (0.5 + 0.5*smoothstep(620, 900, x)) * (0.55 + 0.9*bil:at01(x,y)) * 0.75 end, 2.6)
fr_(PW4,  function(x,y) return smoothstep(430, 520, y) * (1 - smoothstep(520, 600, y)) * clamp(bil:at01(x+200,y) * 1.6 - 0.35, 0, 1) * 0.7 end, 2.0)
blend(FogR, {angle=0.0})
blend(FogR, {angle=0.02})
blend(FogR, {angle=-0.02})
blend(FogR, {angle=0.0})
print(wait(0))

--@ chunk 417
-- ===== one wooded knoll rising from the fog, right-centre; irregular profile, no blends =====
clipHills = rect(560, 440, 300, 140) - fgM - guardAll - spAll - cliffM:grow(1.5)
Kpts = {{598,552},{618,538},{636,528},{650,519},{664,521},{678,512},{694,508},{708,513},{722,509},{738,516},{754,526},{770,538},{792,552}}
Kx0, Kx1 = Kpts[1][1], Kpts[#Kpts][1]
Ktop = function(x) return interp(Kpts, x) + 2.4*trn(x*1.0 + 133, 3.0) * smoothstep(Kx0, Kx0+25, x) * (1 - smoothstep(Kx1-25, Kx1, x)) end
Kbody = mask(function(x,y)
  if x < Kx0 or x > Kx1 then return 0 end
  local t = Ktop(x)
  return smoothstep(t-0.5, t+0.5, y) * ((y < t + 50) and 1 or 0)
end) * clipHills
KbodyP  = pile{{"lead white",3.0},{"smalt",1.7},{"red earth",0.34},{"raw umber",0.2},{"Prussian blue",0.05}}
KbodyP2 = pile{{"lead white",2.5},{"smalt",1.8},{"red earth",0.32},{"raw umber",0.3},{"Prussian blue",0.08}}
Kd = function(x,y)
  local t = Ktop(x)
  local d = (y - t) / 50
  local hh = clamp((552 - t) / 36, 0, 1)
  return 8.0 * (1 - smoothstep(0.0, 1.0, d))^1.2 * hh^0.6
end
stipple(Kbody, {pile=KbodyP, width=2.6, coverage=Kd, feather=0.6, clip=Kbody, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
stipple(Kbody, {pile=KbodyP2, width=1.8, coverage=function(x,y) return 0.8*Kd(x,y)*(1 - smoothstep(0.0, 0.5, (y - Ktop(x))/50)) end, feather=0.6, clip=Kbody, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
-- crowns on the upper face
Kup = Kbody * mask(function(x,y) return (1 - smoothstep(Ktop(x)+2, Ktop(x)+24, y)) end)
work(Kup, {hand="body", tool="filbert 4", pile=HNp2, angle=1.4, coverage=2.8, clip=true, fill=false, length={3,8}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(Ktop(x)+2, Ktop(x)+22, y)) * (0.55 + 0.45*cn1(x*1.3+11, y*1.3)) * smoothstep(Kx0, Kx0+40, x) * (1 - smoothstep(Kx1-40, Kx1, x)), 0, 1) end})
-- warm light on the crowns facing the glow (left shoulder)
work(Kup, {hand="body", tool="filbert 3", pile=ScLit, angle=1.4, coverage=1.4, clip=true, length={3,7}, angle_jitter=0.7,
  load_at=function(x,y) return clamp((1 - smoothstep(Ktop(x)+1, Ktop(x)+12, y)) * (1 - smoothstep(640, 730, x)) * smoothstep(610, 650, x) * (0.4 + 0.6*cn2(x*1.3+3, y*1.3)), 0, 1) end})
-- conifer spikes on the skyline, uneven
for i = 1, 14 do
  local xx = rand(626, 764)
  local t = Ktop(xx)
  local h = rand(5, 12)
  work(tinyPine(xx, t + 3, h, h*0.45), {hand="detail", pile=HNp2, coverage=2.2, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 418
print(wait(16*24*60))
print(drying(700,540), drying(560,520), drying(300,500), drying(800,380))

--@ chunk 419
-- ===== the knoll: dissolve its stippled underside and its vertical ends into the fog (dry-on-dry veil, density ramps in) =====
PWk = pile{{"lead white",6.6},{"pale smalt",0.55},{"red earth",0.2},{"yellow ochre",0.1}}
knollReg = rect(Kx0-14, 498, (Kx1-Kx0)+28, 76) - fgM - guardAll - spAll
kEnd = function(x) return math.max(1 - smoothstep(Kx0-4, Kx0+46, x), smoothstep(Kx1-46, Kx1+4, x)) end
kBot = function(x,y) local t = Ktop(clamp(x, Kx0, Kx1)); return smoothstep(t+16, t+50, y) end
kDens = function(x,y)
  local ends = kEnd(x) * smoothstep(Ktop(clamp(x, Kx0, Kx1)) - 6, Ktop(clamp(x, Kx0, Kx1)) + 20, y)
  return 11.0 * clamp(math.max(kBot(x,y), 0.9*ends), 0, 1)
end
stipple(knollReg, {pile=PWk, width=3.2, coverage=kDens, feather=0.5, clip=knollReg, pressure={0.3,0.75}, dips={30,0.9,0.3}, cluster=0.0})
stipple(knollReg, {pile=PWk, width=2.0, coverage=function(x,y) return 0.8*kDens(x,y) end, feather=0.5, clip=knollReg, pressure={0.3,0.7}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 420
print(drying(700,560), drying(620,560), drying(700,530))
soft2 = (rect(Kx0-70, 478, (Kx1-Kx0)+140, 130):blur(26)) - fgM - guardAll - spAll
blend(soft2, {angle=0.0})
blend(soft2, {angle=0.0})
blend(soft2, {angle=0.04})
blend(soft2, {angle=-0.04})
blend(soft2, {angle=0.0})
print(wait(0))

--@ chunk 421
seamReg2 = rect(430, 428, 180, 150) - fgM - guardAll - spAll
seamD = function(x,y) return 8.0 * smoothstep(432, 486, x) * (1 - smoothstep(540, 606, x)) * smoothstep(430, 468, y) end
stipple(seamReg2, {pile=PW2b, width=4.6, coverage=seamD, feather=0.5, clip=seamReg2, pressure={0.3,0.75}, dips={30,0.9,0.3}, cluster=0.0})
stipple(seamReg2, {pile=PW2b, width=2.8, coverage=function(x,y) return 0.8*seamD(x,y) end, feather=0.5, clip=seamReg2, pressure={0.3,0.7}, dips={30,0.9,0.3}, cluster=0.0})
soft3 = (rect(404, 418, 230, 174):blur(28)) - fgM - guardAll - spAll
blend(soft3, {angle=0.0})
blend(soft3, {angle=0.0})
blend(soft3, {angle=0.04})
blend(soft3, {angle=-0.04})
blend(soft3, {angle=0.0})
print(wait(0))

--@ chunk 422
GfgK = pile{{"raw umber",3},{"bone black",1.2},{"Prussian blue",0.4},{"red earth",0.35}, medium=0.7}
fgK = fgM - footM:grow(0.5) - figM:grow(1.5)
work(fgK, {hand="glaze", pile=GfgK, angle=crestAng, coverage=2.4, clip=true, angle_jitter=0.1,
  load_at=function(x,y) local d = y - crestY(x); return clamp(smoothstep(14, 60, d) * 0.9, 0, 1) end})
print(wait(0))

--@ chunk 423
-- ===== the butte: modelling by dry-on-dry stipple (soft gradations), wooded crown, faint fissures =====
bFace = cliffM * mask(function(x,y) return (y < footLine(x) - 4) and 1 or 0 end)
PShd = mixp(NB.shade, tFogC, 0.0)
PBod = mixp(NB.body,  tFogC, 0.0)
PWrm = mixp(NB.warm,  tFogW, 0.0)
shDens = function(x,y) return 6.5 * smoothstep(shEdge(x,y)-14, shEdge(x,y)+16, x) * (1 - 0.55*smoothstep(380, 432, y)) end
wmDens = function(x,y) return 5.0 * (1 - smoothstep(742, 806, x)) * (1 - smoothstep(344, 412, y)) end
stipple(bFace, {pile=PShd, width=2.4, coverage=shDens, feather=0.6, clip=bFace, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
stipple(bFace, {pile=PShd, width=1.6, coverage=function(x,y) return 0.7*shDens(x,y) end, feather=0.6, clip=bFace, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
stipple(bFace, {pile=PWrm, width=2.4, coverage=wmDens, feather=0.6, clip=bFace, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
print(wait(0))

--@ chunk 424
-- soften the foot of the shaded face into the fog: fog-coloured stipple, density ramping toward the fog line
footStip2 = (cliffM:grow(2) * rect(700, 380, 240, 66)) - guardX
fdens = function(x,y) return 9.0 * smoothstep(392, footLine(x)+2, y)^1.1 end
stipple(footStip2, {pile=fogNear, width=2.6, coverage=fdens, feather=0.6, clip=footStip2, pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
stipple(footStip2, {pile=fogNear, width=1.8, coverage=function(x,y) return 0.8*fdens(x,y) end, feather=0.6, clip=footStip2, pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
-- the wooded crown: dark, ragged dabs along the top of the plateau and on both knobs, conifer spikes standing above the line
CrownD = pile{{"raw umber",1.0},{"Prussian blue",0.55},{"bone black",0.5},{"green earth",0.25},{"smalt",0.5}}
crownBand = cliffM * mask(function(x,y) return (y < cliffTopC(x) + 7) and 1 or 0 end) - needleM:grow(1)
work(crownBand, {hand="body", tool="filbert 3", pile=CrownD, angle=1.4, coverage=2.6, clip=true, fill=false, length={3,8}, angle_jitter=0.7,
  load_at=function(x,y) return clamp(0.5 + 0.5*cn1(x*1.3, y*1.3), 0, 1) end})
for _, xx in ipairs({826, 831, 836, 841, 845, 862, 866, 871, 876}) do
  local t = cliffTopC(xx)
  local h = rand(5, 10)
  work(tinyPine(xx + rand(-1,1), t + 3, h, h*0.45), {hand="detail", pile=CrownD, coverage=2.4, clip=true, length={3,8}, angle=1.5708})
end
print(wait(0))

--@ chunk 425
-- cover the heavy crown band and the stray edge strips with the rock's own colours (dry-on-dry stipple, dense)
topC = function(x) return cliffTopC(clamp(x, 750, 866)) end
bandLow = cliffM * mask(function(x,y)
  if x < 748 or x > 868 then return 0 end
  local t = topC(x)
  return smoothstep(t+2.2, t+3.4, y) * (1 - smoothstep(t+10, t+12, y))
end) - needleM:grow(1)
bandDens = function(x,y) return 16.0 end
-- cool body everywhere in the band first
stipple(bandLow, {pile=PBod, width=2.2, coverage=bandDens, feather=0.0, clip=bandLow, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
-- then warm on the left part and shade on the right part, following the same edges as the face
stipple(bandLow * mask(function(x,y) return 1 - smoothstep(742, 800, x) end), {pile=PWrm, width=2.2, coverage=function(x,y) return 12.0 end, feather=0.0, clip=bandLow, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(bandLow * mask(function(x,y) return smoothstep(shEdge(x,y)-10, shEdge(x,y)+10, x) end), {pile=PShd, width=2.2, coverage=function(x,y) return 12.0 end, feather=0.0, clip=bandLow, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
-- edge strips
stripL = cliffM * mask(function(x,y) return (x < 750 and y > 350) and 1 or 0 end)
stripR = cliffM * mask(function(x,y) return (x > 862 and y > 350) and 1 or 0 end)
stipple(stripL, {pile=PWrm, width=2.2, coverage=function(x,y) return 14.0 * (1 - smoothstep(392, 428, y)) end, feather=0.0, clip=stripL, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(stripL, {pile=PBod, width=2.2, coverage=function(x,y) return 14.0 * smoothstep(380, 428, y) end, feather=0.0, clip=stripL, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(stripR, {pile=PShd, width=2.2, coverage=function(x,y) return 14.0 * (1 - smoothstep(392, 430, y)*0.5) end, feather=0.0, clip=stripR, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(stripR, {pile=PBod, width=2.2, coverage=function(x,y) return 10.0 * smoothstep(396, 430, y) end, feather=0.0, clip=stripR, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 426
legL = (cliffM:grow(3) * rect(728, 396, 26, 50)) - guardX
legR = (cliffM:grow(3) * rect(856, 396, 30, 50)) - guardX
legDens = function(x,y) return 16.0 * smoothstep(400, 424, y) end
stipple(legL, {pile=fogNear, width=2.4, coverage=legDens, feather=0.3, clip=legL, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(legR, {pile=fogNear, width=2.4, coverage=legDens, feather=0.3, clip=legR, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
-- and the whole foot line once more, the fog thickening, so no cut-off shape remains
footAll = (cliffM:grow(3) * rect(724, 404, 170, 44)) - guardX
stipple(footAll, {pile=fogNear, width=3.0, coverage=function(x,y) return 14.0 * smoothstep(408, 428, y) end, feather=0.3, clip=footAll, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 427
-- ===== soften the butte's foot the way the knoll's was softened: fog stipple while open, then a blend through a blurred, wide mask =====
footX = (rect(700, 392, 214, 70) - guardX - fgM)
fDens2 = function(x,y)
  local t = smoothstep(400, 438, y)                       -- thickens downward
  local sideL = (1 - smoothstep(726, 770, x)) * smoothstep(380, 412, y)   -- along the left leg
  local sideR = smoothstep(846, 884, x) * smoothstep(380, 412, y)          -- along the right leg
  return 12.0 * clamp(math.max(t, 0.85*sideL, 0.85*sideR), 0, 1)
end
stipple(footX, {pile=fogNear, width=3.4, coverage=fDens2, feather=0.5, clip=footX, pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
stipple(footX, {pile=fogNear, width=2.2, coverage=function(x,y) return 0.8*fDens2(x,y) end, feather=0.5, clip=footX, pressure={0.35,0.85}, dips={30,0.9,0.3}, cluster=0.0})
softF = (rect(690, 384, 234, 100):blur(24)) - guardX - fgM
blend(softF, {angle=0.0})
blend(softF, {angle=0.0})
blend(softF, {angle=0.05})
blend(softF, {angle=-0.05})
blend(softF, {angle=1.5708})
blend(softF, {angle=0.0})
print(wait(0))

--@ chunk 428
-- more trees on the plateau: conifers of uneven heights and a few round crowns standing above the dark band
local xx = 764
local n = 0
while xx < 868 do
  local t = cliffTopC(clamp(xx, 750, 866))
  local r = math.random()
  if r < 0.55 then
    local h = rand(6, 15)
    work(tinyPine(xx, t + 4, h, h*rand(0.38, 0.5)), {hand="detail", pile=CrownD, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
  elseif r < 0.8 then
    local rr = rand(2.2, 3.6)
    work(ellipse(xx, t - rr*0.2, rr, rr*rand(0.75, 0.95)), {hand="detail", pile=CrownD, coverage=2.6, clip=true, length={3,8}, angle=1.5708})
  end
  xx = xx + rand(2.6, 6.0)
  n = n + 1
end
print("placed", n)
print(wait(0))

--@ chunk 429
stf2 = brush{kind="rigger", width=2, point=1}
stf2:load(figD2, 1.0)
stf2:stroke({{433.8, 547.5},{433.0, 531},{432.2, 515},{431.6, 500.5}}, {pressure={0.62,0.5}, ramps={0.0,0.0}})
print(wait(0))

--@ chunk 430
print(wait(20*24*60))
print(drying(800,330), drying(800,380), drying(300,640), drying(433,520), drying(700,520))

--@ chunk 431
-- ===== (a) a thin cool veil over the mesa's crown band: pushes the dark line back into the haze =====
GcrownV = pile{{"lead white",5},{"pale smalt",1.4},{"red earth",0.28},{"smalt",0.4},{"yellow ochre",0.08}, medium=0.85}
crownReg = (cliffM + needleM + towerK + step) * mask(function(x,y)
  if x < 744 or x > 872 then return 0 end
  return (y < cliffTopC(clamp(x, 750, 866)) + 14) and 1 or 0
end)
work(crownReg, {hand="glaze", pile=GcrownV, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y) return clamp(0.55 + 0.35*smoothstep(cliffTopC(clamp(x, 750, 866)) - 4, cliffTopC(clamp(x, 750, 866)) + 12, y), 0, 1) end})
print(wait(0))

--@ chunk 432
-- ===== undo the chalky veil: the cliff's own colours back over the top zone (dry-on-dry, dense), zones by light and shade =====
topZone = (cliffM + needleM + towerK + step) * mask(function(x,y)
  if x < 740 or x > 878 then return 0 end
  return (y < cliffTopC(clamp(x, 750, 866)) + 16) and 1 or 0
end)
tzLit  = topZone * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x)) end)
tzShd  = topZone * mask(function(x,y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end)
stipple(topZone, {pile=PBod, width=2.2, coverage=function(x,y) return 18.0 end, feather=0.0, clip=topZone, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(tzLit,   {pile=PWrm, width=2.2, coverage=function(x,y) return 14.0 * (1 - 0.4*smoothstep(344, 412, y)) end, feather=0.0, clip=tzLit, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(tzShd,   {pile=PShd, width=2.2, coverage=function(x,y) return 14.0 * (0.6 + 0.4*smoothstep(shEdge(x,y), 872, x)) end, feather=0.0, clip=tzShd, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 433
GunifyD = pile{{"smalt",1.3},{"raw umber",0.75},{"red earth",0.3},{"Prussian blue",0.08},{"lead white",0.25}, medium=0.85}
capBand = (cliffM) * mask(function(x,y)
  if x < 744 or x > 872 then return 0 end
  local t = cliffTopC(clamp(x, 750, 866))
  return (y >= t - 2 and y < t + 34) and 1 or 0
end)
work(capBand, {hand="glaze", pile=GunifyD, angle=0.0, coverage=2.2, clip=true,
  load_at=function(x,y)
    local t = cliffTopC(clamp(x, 750, 866))
    return clamp(0.15 + 0.55*smoothstep(t, t+8, y) * (1 - smoothstep(t+22, t+34, y)), 0, 1)
  end})
print(wait(0))

--@ chunk 434
-- ===== the mesa: complete recoat, as on day 582 (full colours, haze bands, three blends); one session =====
bNew3 = ((cliffM + needleM + towerK + step):grow(2.2)) * mask(function(x,y) return (y <= footLine(x) + 1.5) and 1 or 0 end)
nShade3 = bNew3 * mask(function(x,y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end)
nWarm3  = bNew3 * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x)) end)
work(bNew3,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, 0.0), angle=0.0, coverage=7.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08})
work(nShade3, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.55 + 0.45*smoothstep(shEdge(x,y), 872, x), 0, 1) end})
work(nWarm3,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, 0.0), angle=0.0, coverage=4.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.85*(1 - 0.5*smoothstep(345, 420, y)), 0, 1) end})
for i, b in ipairs(hb2) do
  local f, y0, y1 = b[1], b[2], b[3]
  local prof = function(y)
    local up = smoothstep(y0, y0 + 0.45*(y1-y0), y)
    local dn = (i == #hb2) and 1 or (1 - smoothstep(y0 + 0.75*(y1-y0), y1, y))
    return clamp(up*dn, 0, 1)
  end
  local bm = rect(690, y0 - 2, 240, (y1 - y0) + 4)
  work(bNew3 * bm,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, f), angle=0.0, coverage=3.6, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(nShade3 * bm, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, f), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(nWarm3 * bm,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, f), angle=0.0, coverage=3.2, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
end
blend(bNew3, {angle=1.5708})
blend(bNew3, {angle=1.5708})
blend(bNew3, {angle=0.0})
print(wait(0))

--@ chunk 435
print(wait(16*24*60))
print(drying(800,330), drying(800,380), drying(800,430), drying(760,350))

--@ chunk 436
-- ===== the mesa, modelled dry-on-dry with soft stipple (as on day 615), and nothing hard laid after =====
mFace = (cliffM + needleM + towerK + step) * mask(function(x,y) return (y < footLine(x) - 2) and 1 or 0 end)
shDens2 = function(x,y) return 7.5 * smoothstep(shEdge(x,y)-16, shEdge(x,y)+18, x) * (1 - 0.5*smoothstep(380, 436, y)) end
wmDens2 = function(x,y) return 6.0 * (1 - smoothstep(742, 812, x)) * (1 - smoothstep(344, 418, y)) end
stipple(mFace, {pile=PShd, width=2.4, coverage=shDens2, feather=0.6, clip=mFace, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
stipple(mFace, {pile=PShd, width=1.6, coverage=function(x,y) return 0.7*shDens2(x,y) end, feather=0.6, clip=mFace, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
stipple(mFace, {pile=PWrm, width=2.4, coverage=wmDens2, feather=0.6, clip=mFace, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
-- the mid-left rib (a lit pillar): a narrow band of warm stipple
ribD = function(x,y) return 5.0 * (1 - smoothstep(5, 11, math.abs(x - (800 + 5*irr(x*0.6, y))))) * (1 - 0.7*smoothstep(350, 428, y)) end
stipple(mFace, {pile=BCliffP, width=2.0, coverage=ribD, feather=0.6, clip=mFace, pressure={0.35,0.8}, dips={30,0.9,0.3}, cluster=0.1})
print(wait(0))

--@ chunk 437
-- 1. the rim ring that the face stipple didn't reach
ringM = bNew3 - mFace
ringL = ringM * mask(function(x,y) return 1 - smoothstep(780, 820, x) end)
ringR = ringM * mask(function(x,y) return smoothstep(shEdge(x,y)-16, shEdge(x,y)+18, x) end)
ringT = ringM * mask(function(x,y) return (y < 395) and 1 or 0 end)
stipple(ringM, {pile=PBod, width=2.0, coverage=function(x,y) return 14.0 end, feather=0.0, clip=ringM, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(ringL, {pile=PWrm, width=2.0, coverage=function(x,y) return 10.0 * (1 - smoothstep(344, 418, y)) end, feather=0.3, clip=ringL, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(ringR, {pile=PShd, width=2.0, coverage=function(x,y) return 14.0 end, feather=0.0, clip=ringR, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
-- 2. the foot dissolves: fog-coloured stipple, thicker downward, reaching beyond the cut
footW = (bNew3:grow(7) * rect(690, 384, 240, 100)) - guardX - fgM
fDens3 = function(x,y)
  local t = smoothstep(396, 446, y)^1.15
  local sideL = (1 - smoothstep(724, 772, x)) * smoothstep(372, 410, y)
  local sideR = smoothstep(846, 888, x) * smoothstep(372, 410, y)
  return 15.0 * clamp(math.max(t, 0.9*sideL, 0.9*sideR), 0, 1)
end
stipple(footW, {pile=fogNear, width=3.4, coverage=fDens3, feather=0.5, clip=footW, pressure={0.4,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(footW, {pile=fogNear, width=2.2, coverage=function(x,y) return 0.8*fDens3(x,y) end, feather=0.5, clip=footW, pressure={0.4,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 438
-- ===== the mesa, recoated in ONE session. Zones kept apart (vertical blends only); the foot is painted AS fog =====
footExt = poly({{722,418},{890,418},{890,452},{722,452}}) * mask(function(x,y) return (y >= footLine(x) - 6) and 1 or 0 end)
bNew4 = bNew3 + footExt
zShade4 = bNew4 * mask(function(x,y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end)
zWarm4  = bNew4 * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x)) end)
work(bNew4,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, 0.0), angle=0.0, coverage=7.0, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return 1.0 end})
work(zShade4, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, 0.0), angle=0.0, coverage=4.5, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.6 + 0.4*smoothstep(shEdge(x,y), 872, x), 0, 1) end})
work(zWarm4,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, 0.0), angle=0.0, coverage=4.5, clip=true, fill=true, length={8,22}, angle_jitter=0.08,
  load_at=function(x,y) return clamp(0.9*(1 - 0.5*smoothstep(345, 420, y)), 0, 1) end})
hb4 = {{0.16, 356, 390}, {0.34, 374, 404}, {0.52, 390, 416}, {0.72, 402, 428}, {0.90, 414, 438}, {1.0, 424, 452}}
for i, b in ipairs(hb4) do
  local f, y0, y1 = b[1], b[2], b[3]
  local prof = function(y)
    local up = smoothstep(y0, y0 + 0.45*(y1-y0), y)
    local dn = (i == #hb4) and 1 or (1 - smoothstep(y0 + 0.75*(y1-y0), y1, y))
    return clamp(up*dn, 0, 1)
  end
  local bm = rect(690, y0 - 2, 240, (y1 - y0) + 4)
  work(bNew4 * bm,   {hand="body", tool="filbert 8", pile=mixp(NB.body,  tFogC, f), angle=0.0, coverage=3.8, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(zShade4 * bm, {hand="body", tool="filbert 8", pile=mixp(NB.shade, tFogC, f), angle=0.0, coverage=3.4, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
  work(zWarm4 * bm,  {hand="body", tool="filbert 8", pile=mixp(NB.warm,  tFogW, f), angle=0.0, coverage=3.4, clip=true, fill=true, length={8,22}, angle_jitter=0.08, load_at=function(x,y) return prof(y) end})
end
blend(bNew4, {angle=1.5708})
blend(bNew4, {angle=1.5708})
blend(bNew4, {angle=1.5708})
print(wait(0))

--@ chunk 439
print(drying(800,430), drying(800,400), drying(740,440))
softFoot = (rect(706, 398, 200, 62):blur(12)) - guardX - fgM
blend(softFoot, {angle=0.0})
blend(softFoot, {angle=0.04})
blend(softFoot, {angle=-0.04})
blend(softFoot, {angle=0.0})
print(wait(0))

--@ chunk 440
print(wait(16*24*60))
print(drying(800,330), drying(800,380), drying(800,430), drying(760,450), drying(300,500))

--@ chunk 441
PfA = PW2b
PfB = pile{{"lead white",6},{"pale smalt",0.55},{"red earth",0.22},{"yellow ochre",0.2},{"smalt",0.2}}
PfC = pile{{"lead white",5.5},{"pale smalt",0.6},{"red earth",0.28},{"yellow ochre",0.25},{"raw umber",0.08}}
for i, p in ipairs({PfA, PfB, PfC}) do
  local m = rect(696, 408 + (i-1)*20, 26, 14)
  stipple(m, {pile=p, width=3.0, coverage=7.0, feather=0.0, clip=m, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
end
print(wait(0))

--@ chunk 442
-- ===== the grey skirt: covered with fog-coloured stipple, density falling off smoothly 30 units beyond it =====
function sdRect(x, y, x0, y0, x1, y1)
  local cx, cy = (x0+x1)/2, (y0+y1)/2
  local hx, hy = (x1-x0)/2, (y1-y0)/2
  local dx, dy = math.abs(x-cx) - hx, math.abs(y-cy) - hy
  local ox, oy = math.max(dx,0), math.max(dy,0)
  return math.sqrt(ox*ox+oy*oy) + math.min(math.max(dx,dy),0)
end
skirtM = rect(655, 376, 310, 130) - fgM - guardX
skDens = function(x,y)
  local d = sdRect(x, y, 702, 406, 918, 464)
  local ring = 1 - smoothstep(0, 30, d)
  local top = smoothstep(390, 412, y)
  return 8.0 * ring * top
end
stipple(skirtM, {pile=PfB, width=3.2, coverage=skDens, feather=0.25, clip=skirtM, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(skirtM, {pile=PfB, width=2.0, coverage=function(x,y) return 0.7*skDens(x,y) end, feather=0.25, clip=skirtM, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 443
print(wait(3*24*60))
print(drying(780,420), drying(700,440), drying(800,380), drying(800,350))

--@ chunk 444
-- ===== 1. restore the mesa's lower body inside its silhouette (full density, zones), then 2. dissolve with a long, gentle ramp =====
lowMesa = (cliffM + needleM) * mask(function(x,y) return (y >= 376 and y <= footLine(x) + 3) and 1 or 0 end)
lmLit = lowMesa * mask(function(x,y) return (1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x)) end)
lmShd = lowMesa * mask(function(x,y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end)
stipple(lowMesa, {pile=PBod, width=2.2, coverage=function(x,y) return 14.0 end, feather=0.0, clip=lowMesa, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(lmLit,   {pile=PWrm, width=2.2, coverage=function(x,y) return 9.0 * (1 - 0.5*smoothstep(345, 420, y)) end, feather=0.0, clip=lmLit, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(lmShd,   {pile=PShd, width=2.2, coverage=function(x,y) return 12.0 * (0.6 + 0.4*smoothstep(shEdge(x,y), 872, x)) end, feather=0.0, clip=lmShd, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 445
print(drying(750,400), drying(800,410), drying(850,420), drying(800,370))
blendZoneM = ((cliffM + needleM) * rect(700, 340, 220, 96)):blur(7) * cliffM
blend(blendZoneM, {angle=1.5708})
blend(blendZoneM, {angle=1.5708})
blend(blendZoneM, {angle=1.45})
blend(blendZoneM, {angle=1.5708})
print(wait(0))

--@ chunk 446
dsM = rect(704, 382, 212, 64) - fgM - guardX
dsD = function(x,y)
  local t = clamp((y - 384)/52, 0, 1.2)
  return 6.0 * t^3
end
stipple(dsM, {pile=PfB, width=3.0, coverage=dsD, feather=0.2, clip=dsM, pressure={0.45,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(dsM, {pile=PfB, width=2.0, coverage=function(x,y) return 0.8*dsD(x,y) end, feather=0.2, clip=dsM, pressure={0.45,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 447
-- ===== the mesa's modelling: soft dry-on-dry stipple, fine touches, graded; nothing hard laid after =====
mTop = (cliffM + needleM + towerK + step) * mask(function(x,y) return (y < 412) and 1 or 0 end)
mShdD = function(x,y)
  local up = 1 - smoothstep(352, 410, y)
  return 13.0 * smoothstep(shEdge(x,y)-18, shEdge(x,y)+22, x) * (0.35 + 0.65*up)
end
mWrmD = function(x,y) return 8.0 * (1 - smoothstep(742, 800, x)) * (1 - smoothstep(340, 408, y)) end
mCoolD = function(x,y) return 5.0 * (1 - smoothstep(332, 372, y)) * smoothstep(752, 780, x) end
stipple(mTop, {pile=PShd, width=1.8, coverage=mShdD, feather=0.7, clip=mTop, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.1})
stipple(mTop, {pile=PWrm, width=1.8, coverage=mWrmD, feather=0.7, clip=mTop, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.1})
stipple(mTop, {pile=PBod, width=1.8, coverage=mCoolD, feather=0.7, clip=mTop, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.1})
print(wait(0))

--@ chunk 448
-- (1) knock the terracotta column back toward the mesa's lilac, leaving a warm edge and warm flecks
colL = (cliffM + needleM) * mask(function(x,y) return (x < 800 and y < 410) and 1 or 0 end)
stipple(colL, {pile=PBod, width=1.8, coverage=function(x,y) return 8.0 * smoothstep(752, 786, x) * (1 - 0.5*smoothstep(380, 410, y)) + 3.0 * (1 - smoothstep(340, 400, y)) * (1 - smoothstep(742, 792, x)) end,
  feather=0.7, clip=colL, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.1})
-- (2) dissolve the hard bottom of the dark right side into the mist, gently rising from y=392
dissR = rect(696, 388, 226, 56) - fgM - guardX
dissD = function(x,y) local t = clamp((y - 392)/36, 0, 1.2); return 9.0 * t^1.6 end
stipple(dissR, {pile=PfB, width=2.4, coverage=dissD, feather=0.3, clip=dissR, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
stipple(dissR, {pile=PfB, width=1.6, coverage=function(x,y) return 0.8*dissD(x,y) end, feather=0.3, clip=dissR, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 449
print(drying(800,350), drying(800,500), drying(700,520), drying(900,450), drying(250,500), drying(450,600))
print(wait(0))

--@ chunk 450
local names = {}
for k, v in pairs(_G) do
  if type(k) == "string" then names[#names+1] = k end
end
table.sort(names)
local out = {}
for _, k in ipairs(names) do
  local v = _G[k]
  local t = type(v)
  local tag = (t == "function") and "f" or (t == "number") and "n" or (t == "table") and "t" or (t == "userdata") and "u" or t
  out[#out+1] = k .. ":" .. tag
end
print(table.concat(out, " "))

--@ chunk 451
for _, k in ipairs({"BarkFur","BarkFur2","BarkG","BarkH","BarkRim","BarkRim2","OakD","OakT","OakW","OakW2","PW2b","PW3c","fogNear","tFogC","tFogW","PfB","PfA","PfC","PBod","PShd","PWrm","BCliffP","CrownD","figD2","GrassGold","GrassTan","GrassOliv","GrassDeep"}) do
  print(k, tostring(_G[k]))
end
print("trunk", type(trunk), #trunk)
for k,v in pairs(trunk) do print(k, type(v), (type(v)=="table") and #v or tostring(v)) end
print("trunkClean", trunkClean:area(), "trunkIn", trunkIn:area(), "trunkSpots", trunkSpots:area(), "treeAllG", treeAllG:area(), "guardX", guardX:area(), "fgM", fgM:area(), "figM", figM:area())

--@ chunk 452
print(wait(20*24*60))
print(drying(800,350), drying(800,400), drying(780,430), drying(800,300), drying(250,480), drying(430,520))

--@ chunk 453
local s = {}
for x = 730, 890, 10 do s[#s+1] = string.format("%d:%.0f", x, footLine(x)) end
print(table.concat(s, " "))
print("cliffM area", cliffM:area(), "needleM", needleM:area(), "towerK", towerK:area(), "step", step:area())
-- ascii map of cliffM + guardX + fgM over region x 560..1000 step 10, y 380..700 step 10
local rows = {}
for y = 380, 700, 10 do
  local r = {}
  for x = 560, 1000, 8 do
    local c = "."
    if cliffM:at(x,y) > 0.5 or needleM:at(x,y) > 0.5 then c = "M"
    elseif fgM:at(x,y) > 0.5 then c = "#"
    elseif guardX:at(x,y) > 0.5 then c = "g" end
    r[#r+1] = c
  end
  rows[#rows+1] = string.format("%3d ", y) .. table.concat(r)
end
print(table.concat(rows, "\n"))

--@ chunk 454
for _, k in ipairs({"spAll","spr1","spr2m","spr3","sp1","sp2","sp3","spB","spruceMask","spc","spc2","spruce","spruceFull","spruceRag","stf","stf2","staffGuard","footM","footAll","twigGuard","twigGuard2","twigGuard3","twigTight","twigThin","twigL","guard","guard2","guard3","guardAll","guardG","guardL","guardT"}) do
  local v = _G[k]
  local d = type(v)
  if type(v) == "userdata" then
    local ok, a = pcall(function() return v:area() end)
    d = d .. (ok and (" area " .. string.format("%.0f", a)) or " (no area)")
  end
  print(k, d, tostring(v))
end

--@ chunk 455
local rows = {}
for y = 570, 714, 6 do
  local r = {}
  for x = 800, 1000, 4 do
    local c = "."
    if spAll:at(x,y) > 0.3 then c = "S"
    elseif fgM:at(x,y) > 0.5 then c = "#" end
    r[#r+1] = c
  end
  rows[#rows+1] = string.format("%3d ", y) .. table.concat(r)
end
print(table.concat(rows, "\n"))

--@ chunk 456
local function amap(m, x0,x1,y0,y1,sx,sy)
  local rows = {}
  for y = y0, y1, sy do
    local r = {}
    for x = x0, x1, sx do
      r[#r+1] = (m:at(x,y) > 0.3) and "K" or "."
    end
    rows[#rows+1] = string.format("%3d ", y) .. table.concat(r)
  end
  return table.concat(rows, "\n")
end
print("knollReg"); print(amap(knollReg, 600, 800, 480, 560, 4, 4))
print("Atop at x:")
local s = {}
for x = 560, 1000, 40 do s[#s+1] = string.format("%d:%.0f", x, Atop(x)) end
print(table.concat(s, " "))

--@ chunk 457
-- A1: dissolve the lozenge: S-coloured stipple, strongest at the old seam, fading smoothly inward (70 units) and outward (26..40)
mesaAll = (cliffM + needleM + towerK + step):grow(1.5)
A1reg = rect(610, 380, 400, 125) - fgM - guardX - spAll:grow(3) - mesaAll
lozD = function(x, y) return sdRect(x, y, 702, 406, 918, 464) end
A1dens = function(x, y)
  local d = lozD(x, y)
  local inner = smoothstep(-70, -4, d)
  local outer = 1 - smoothstep(26, 40, d)
  local top = smoothstep(Atop(x) + 3, Atop(x) + 16, y)
  return 3.2 * inner * outer * top
end
S1 = pile{{"lead white",6.6},{"pale smalt",0.7},{"smalt",0.35},{"red earth",0.22},{"yellow ochre",0.06}}
print(tostring(S1), A1reg:area())
local mx, sx, sy = 0, 0, 0
for y = 380, 505, 5 do for x = 610, 1010, 5 do local c = A1dens(x,y) * (A1reg:at(x,y)); if c > mx then mx = c; sx = x; sy = y end end end
print("max cov", mx, sx, sy)

--@ chunk 458
stipple(A1reg, {pile=S1, width=3.0, coverage=function(x,y) return 0.65*A1dens(x,y) end, feather=0.3, clip=A1reg, pressure={0.45,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 459
T_a = PW3c
T_b = pile{{"lead white",6.2},{"pale smalt",0.5},{"smalt",0.8},{"red earth",0.35},{"yellow ochre",0.08}}
T_c = pile{{"lead white",5.6},{"pale smalt",0.5},{"smalt",0.9},{"red earth",0.45},{"raw umber",0.15},{"yellow ochre",0.1}}
T_d = pile{{"lead white",6.0},{"pale smalt",0.5},{"smalt",0.6},{"red earth",0.5},{"yellow ochre",0.2},{"raw umber",0.1}}
local ps = {T_a, T_b, T_c, T_d}
for i, p in ipairs(ps) do
  local m = rect(968, 420 + (i-1)*22, 26, 14)
  stipple(m, {pile=p, width=3.0, coverage=7.0, feather=0.0, clip=m, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
end
print(wait(0))

--@ chunk 460
local function amap(m, x0,x1,y0,y1,sx,sy)
  local rows = {}
  for y = y0, y1, sy do
    local r = {}
    for x = x0, x1, sx do
      r[#r+1] = (m:at(x,y) > 0.3) and "K" or "."
    end
    rows[#rows+1] = string.format("%3d ", y) .. table.concat(r)
  end
  return table.concat(rows, "\n")
end
print("Kbody"); print(amap(Kbody, 590, 800, 480, 560, 4, 4))
print("mK1", amap(mK1, 590, 800, 480, 560, 4, 4))

--@ chunk 461
T_m = pile{{"lead white",5.9},{"pale smalt",0.5},{"smalt",0.85},{"red earth",0.4},{"raw umber",0.08},{"yellow ochre",0.09}}
A2reg = rect(610, 380, 400, 135) - fgM - guardX - spAll:grow(3) - mesaAll
knollFade = function(x, y)
  -- 1 inside an ellipse around the knoll (centre 705,522), 0 well outside
  local dx, dy = (x - 705)/95, (y - 522)/30
  local r = math.sqrt(dx*dx + dy*dy)
  return 1 - smoothstep(0.9, 1.5, r)
end
A2dens = function(x, y)
  local d = lozD(x, y)
  local inner = smoothstep(-70, -4, d)
  local outer = 1 - smoothstep(34, 56, d)
  local top = smoothstep(Atop(x) + 3, Atop(x) + 16, y)
  local left = smoothstep(690, 745, x)
  return 3.4 * inner * outer * top * left * (1 - knollFade(x, y))
end
local wTop = function(y) return 1 - smoothstep(425, 445, y) end
local wLow = function(y) return smoothstep(450, 470, y) end
local wMid = function(y) return clamp(1 - wTop(y) - wLow(y), 0, 1) end
local specs = {{T_a, wTop}, {T_m, wMid}, {T_c, wLow}}
for _, s in ipairs(specs) do
  local p, w = s[1], s[2]
  stipple(A2reg, {pile=p, width=3.0, coverage=function(x,y) return w(y)*A2dens(x,y) end, feather=0.3, clip=A2reg, pressure={0.45,0.9}, dips={30,0.9,0.3}, cluster=0.0})
end
print(wait(0))

--@ chunk 462
local function cover(y0, p)
  local x0, x1, y1 = 968, 994, y0 + 14
  local reg = rect(x0 - 16, y0 - 14, (x1 - x0) + 32, 14 + 14 + 14) - fgM - guardX
  local dens = function(x, y)
    local d = sdRect(x, y, x0 - 1, y0 - 1, x1 + 1, y1 + 1)
    return 9.0 * (1 - smoothstep(-1, 12, d))
  end
  stipple(reg, {pile=p, width=3.0, coverage=dens, feather=0.4, clip=reg, pressure={0.45,0.9}, dips={30,0.9,0.3}, cluster=0.0})
  stipple(reg, {pile=p, width=2.0, coverage=function(x,y) return 0.7*dens(x,y) end, feather=0.4, clip=reg, pressure={0.45,0.9}, dips={30,0.9,0.3}, cluster=0.0})
end
cover(420, T_m)
cover(442, T_m)
cover(464, T_c)
cover(486, T_c)
print(wait(0))

--@ chunk 463
print(drying(980,430), drying(960,440), drying(900,440), drying(990,470))
blurM = (rect(936, 398, 80, 128):blur(12)) - fgM - guardX
blend(blurM, {angle=0.0})
blend(blurM, {angle=0.03})
blend(blurM, {angle=-0.03})
print(wait(0))

--@ chunk 464
local function edgeAt(m, y, x0, x1)
  local prev = m:at(x0, y)
  for x = x0, x1, 0.5 do
    local v = m:at(x, y)
    if (prev > 0.5) ~= (v > 0.5) then return x end
    prev = v
  end
  return nil
end
for _, y in ipairs({310, 320, 330, 340, 350, 360, 380}) do
  print(y, "cliffM right edge", edgeAt(cliffM, y, 840, 900), "bNew3 right", edgeAt(bNew3, y, 840, 900), "bNew4", edgeAt(bNew4, y, 840, 900), "mFace", edgeAt(mFace, y, 840, 900))
end
print(shEdge(840, 330), litEdge(840, 330))

--@ chunk 465
mesaCore = (cliffM + needleM + towerK + step)
ringAll = (bNew3 * mask(function(x,y) return (y < 414) and 1 or 0 end)) - mesaCore
print("ringAll", ringAll:area())
local rows = {}
for y = 300, 360, 3 do
  local r = {}
  for x = 820, 890, 1.5 do
    local c = "."
    if mesaCore:at(x,y) > 0.5 then c = "#" elseif ringAll:at(x,y) > 0.5 then c = "o" end
    r[#r+1] = c
  end
  rows[#rows+1] = string.format("%3d ", y) .. table.concat(r)
end
print(table.concat(rows, "\n"))

--@ chunk 466
ringShd2 = ringAll * mask(function(x,y) return smoothstep(shEdge(x,y)-16, shEdge(x,y)+18, x) end)
ringWrm2 = ringAll * mask(function(x,y) return 1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x) end)
stipple(ringAll,  {pile=PBod, width=2.0, coverage=function(x,y) return 14.0 end, feather=0.0, clip=ringAll, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(ringWrm2, {pile=PWrm, width=2.0, coverage=function(x,y) return 10.0 * (1 - 0.7*smoothstep(344, 418, y)) end, feather=0.3, clip=ringWrm2, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(ringShd2, {pile=PShd, width=2.0, coverage=function(x,y) return 14.0 end, feather=0.0, clip=ringShd2, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 467
-- the thin 'legs' (silhouette edges that hang below the mesa's fringe) and the dark speck at the left foot: cover with the exact fog colour, dry-on-dry
legsReg = (rect(728, 396, 26, 36) + rect(862, 396, 26, 30) + rect(742, 414, 14, 18)) - fgM - guardX
legsD = function(x, y)
  local a = 1 - smoothstep(0, 8, math.min(sdRect(x, y, 733, 398, 746, 419), 99))
  local b = 1 - smoothstep(0, 8, math.min(sdRect(x, y, 867, 398, 881, 418), 99))
  local c = 1 - smoothstep(0, 6, math.min(sdRect(x, y, 744, 418, 753, 428), 99))
  return 10.0 * math.max(a, b, c)
end
stipple(legsReg, {pile=fogNear, width=2.6, coverage=legsD, feather=0.3, clip=legsReg, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
stipple(legsReg, {pile=fogNear, width=1.6, coverage=function(x,y) return 0.7*legsD(x,y) end, feather=0.3, clip=legsReg, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 468
-- (1) aerial gradient over the mesa body: lilac-cream stipple, none at the top, thickening toward the foot (dry-on-dry; same silhouette so no halo on the sky)
mesaSil = (cliffM + needleM + towerK + step):grow(1.2)
vReg = mesaSil * mask(function(x,y) return (y >= 346 and y <= 400) and 1 or 0 end) - guardX
vDens = function(x,y)
  local g = smoothstep(352, 408, y)
  return 7.0 * g^2.4
end
stipple(vReg, {pile=T_m, width=2.2, coverage=vDens, feather=0.5, clip=vReg, pressure={0.35,0.8}, dips={40,0.9,0.3}, cluster=0.0})
stipple(vReg, {pile=T_m, width=1.4, coverage=function(x,y) return 0.6*vDens(x,y) end, feather=0.5, clip=vReg, pressure={0.35,0.8}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 469
local function spans(m, y, x0, x1)
  local out = {}
  local inside = false
  local start
  for x = x0, x1, 0.5 do
    local v = m:at(x, y) > 0.5
    if v and not inside then inside = true; start = x
    elseif (not v) and inside then inside = false; out[#out+1] = string.format("[%.1f-%.1f]", start, x) end
  end
  if inside then out[#out+1] = string.format("[%.1f-%.1f..]", start, x1) end
  return table.concat(out, " ")
end
for y = 380, 560, 10 do
  print(y, "trunkClean", spans(trunkClean, y, 150, 380), " trunkIn", spans(trunkIn, y, 150, 380))
end

--@ chunk 470
trR = {{410,262.5},{420,268},{430,269},{440,268.5},{450,268.5},{460,269},{470,270},{480,271},{490,272.5},{500,275},{510,278},{520,282},{530,285},{540,288},{550,287.5}}
trL = {{410,222},{420,216.5},{430,214.5},{440,213},{450,212},{460,211.5},{470,211},{480,210.5},{490,210},{500,209.5},{510,208.5},{520,207.5},{530,206},{540,205.5},{550,205}}
xR_ = function(y) return interp(trR, clamp(y, 410, 550)) end
xL_ = function(y) return interp(trL, clamp(y, 410, 550)) end
print(xR_(450), xL_(450), xR_(520))
trunkBand = trunkClean * mask(function(x,y)
  if y < 420 or y > 548 then return 0 end
  local r, l = xR_(y), xL_(y)
  return ((x > l + 0.55*(r-l)) and 1 or 0)
end)
print("trunkBand", trunkBand:area())
-- trial: a short stretch of rim light, y 440-480 only
rimTest = trunkClean * mask(function(x,y)
  if y < 440 or y > 480 then return 0 end
  return (x > xR_(y) - 8) and 1 or 0
end)
print("rimTest", rimTest:area())
BarkRimT = pile{{"raw umber",2.0},{"yellow ochre",1.2},{"lead white",1.0},{"red earth",0.5}, medium=0.1}
work(rimTest, {hand="detail", pile=BarkRimT, coverage=1.4, clip=true, length={6,16}, angle=1.5708, angle_jitter=0.12,
  load_at=function(x,y) return clamp(0.25 + 0.5*smoothstep(xR_(y)-8, xR_(y)-1, x), 0, 1) end})
print(wait(0))

--@ chunk 471
print(drying(266, 460))
blendT = trunkClean * mask(function(x,y)
  if y < 434 or y > 488 then return 0 end
  return (x > xR_(y) - 18) and 1 or 0
end)
blend(blendT, {angle=0.0})
blend(blendT, {angle=0.1})
blend(blendT, {angle=1.5708})
print(wait(0))

--@ chunk 472
coverT = trunkClean * mask(function(x,y)
  if y < 430 or y > 492 then return 0 end
  return (x > xR_(y) - 24) and 1 or 0
end)
work(coverT, {hand="body", pile=OakD, angle=1.5708, coverage=3.2, clip=true, angle_jitter=0.08, fill=true})
print(wait(0))

--@ chunk 473
local ok, e = pcall(function() return noise{seed=4177, octaves=3, period=70, persistence=0.55, stretch={0.0, 4}} end)
print(ok, tostring(e))
local ok2, e2 = pcall(function() return noise{seed=4177, octaves=3, period=70, persistence=0.55, stretch={angle=0.0, k=4}} end)
print(ok2, tostring(e2))

--@ chunk 474
PW2c = pile{{"lead white",7},{"pale smalt",0.62},{"red earth",0.14},{"yellow ochre",0.1}}
nStr = noise{seed=4177, octaves=3, period=70, persistence=0.55, stretch={0.0, 4}}
nLow = noise{seed=913, octaves=2, period=120, persistence=0.5}
fogTopX = function(x) return 404 + 7*nLow(x, 0) + 4*nLow(x*2.3, 50) - 6*math.exp(-((x-790)/38)^2) end
local s = {}
for x = 720, 900, 20 do s[#s+1] = string.format("%d:%.0f", x, fogTopX(x)) end
print(table.concat(s, " "))
vA = mesaSil * mask(function(x,y) return (y >= 340 and y <= 432) and 1 or 0 end) - guardX
dA = function(x,y)
  local ft = fogTopX(x)
  local g = smoothstep(ft - 52, ft + 8, y)
  local st = 0.55 + 0.9 * nStr:at01(x, y)
  return 5.5 * g^1.6 * st
end
stipple(vA, {pile=T_m, width=2.0, coverage=dA, feather=0.5, clip=vA, drag={12, 0.0}, pressure={0.35,0.8}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 475
blockR = rect(690, 380, 240, 96) * mask(function(x,y) return (y > Atop(x) + 2) and 1 or 0 end) - fgM - guardX - spAll:grow(3)
blockD = function(x, y)
  local d = sdRect(x, y, 729, 380, 881, 433)
  local ramp = 1 - smoothstep(-3, 24, d)
  local top = smoothstep(Atop(x) + 2, Atop(x) + 9, y)
  return 5.2 * ramp * top
end
stipple(blockR, {pile=T_m, width=3.0, coverage=blockD, feather=0.4, clip=blockR, pressure={0.45,0.9}, dips={40,0.9,0.3}, cluster=0.0})
stipple(blockR, {pile=T_m, width=2.0, coverage=function(x,y) return 0.7*blockD(x,y) end, feather=0.4, clip=blockR, pressure={0.45,0.9}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 476
for _, p in ipairs({{735,410},{735,420},{875,410},{875,420},{800,420},{740,440},{760,415}}) do
  print(p[1], p[2], "guardX", guardX:at(p[1],p[2]), "blockR", blockR:at(p[1],p[2]), "blockD", blockD(p[1],p[2]), "fgM", fgM:at(p[1],p[2]), "cliffM", cliffM:at(p[1],p[2]))
end

--@ chunk 477
boxReg = (rect(700, 384, 220, 66)) * mask(function(x,y) return (y > Atop(x) + 2) and 1 or 0 end) - fgM - guardX - spAll:grow(3)
boxD = function(x, y)
  local a = sdRect(x, y, 729, 394, 750, 436)
  local b = sdRect(x, y, 860, 394, 889, 436)
  local d = math.min(a, b)
  return 9.0 * (1 - smoothstep(-2, 14, d)) * smoothstep(Atop(x) + 2, Atop(x) + 9, y)
end
stipple(boxReg, {pile=T_m, width=3.0, coverage=boxD, feather=0.4, clip=boxReg, pressure={0.45,0.9}, dips={40,0.9,0.3}, cluster=0.0})
stipple(boxReg, {pile=T_m, width=2.0, coverage=function(x,y) return 0.7*boxD(x,y) end, feather=0.4, clip=boxReg, pressure={0.45,0.9}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 478
local rows = {}
for y = 440, 560, 4 do
  local r = {}
  for x = 0, 210, 3 do
    local c = "."
    if treeAllG:at(x,y) > 0.3 then c = "T" end
    r[#r+1] = c
  end
  rows[#rows+1] = string.format("%3d ", y) .. table.concat(r)
end
print(table.concat(rows, "\n"))

--@ chunk 479
print(wait(20*24*60))
print(drying(800,350), drying(800,420), drying(730,420), drying(266,460), drying(430,520))

--@ chunk 480
-- (3) a wide, gentle veil of lilac-grey under the mesa: dry-on-dry stipple whose density falls off over ~90 units, so the grey patch and the pale columns lose their straight edges
vReg2 = rect(600, 396, 410, 170) * mask(function(x,y) return (y > Atop(x) + 2) and 1 or 0 end) - fgM - guardX - spAll:grow(3) - mesaAll
vD2 = function(x, y)
  local d = sdRect(x, y, 748, 396, 862, 452)
  local ramp = (1 - smoothstep(0, 95, d))
  local top = smoothstep(Atop(x) + 2, Atop(x) + 14, y)
  local bot = 1 - smoothstep(520, 570, y)
  return 2.2 * ramp * top * bot
end
local mx = 0
for y = 396, 566, 6 do for x = 600, 1010, 6 do local c = vD2(x,y) * vReg2:at(x,y); if c > mx then mx = c end end end
print("max", mx, tostring(T_b))
stipple(vReg2, {pile=T_b, width=4.0, coverage=vD2, feather=0.6, clip=vReg2, pressure={0.3,0.7}, dips={40,0.9,0.3}, cluster=0.0})
stipple(vReg2, {pile=T_b, width=2.6, coverage=function(x,y) return 0.6*vD2(x,y) end, feather=0.6, clip=vReg2, pressure={0.3,0.7}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 481
for y = 410, 556, 12 do
  print(y, string.format("L %.1f R %.1f", xL_(y), xR_(y)))
end
trunkIn2 = trunkClean:shrink(1.5)
print("trunkIn2", trunkIn2:area())
-- Test brushes
furB = brush{kind="round", width=3, point=1}
print(furB:mark_width(0.3), furB:mark_width(0.5), furB:mark_width(0.8), furB:pressure_for(1.2))

--@ chunk 482
function furrow(off, y0, len, p, load, pr, wob)
  local pts = {}
  local n = math.max(3, math.floor(len / 9))
  local ph = rand(0, 6.28)
  for i = 0, n do
    local y = y0 + len * i / n
    local x = xR_(y) - off - (wob or 1.2) * math.sin(ph + i * 1.3) + rand(-0.4, 0.4)
    pts[#pts+1] = {x, y}
  end
  furB:reload(p, load)
  furB:stroke(pts, {pressure={pr*0.5, pr, pr*0.6}, ramps={0.2, 0.35}, clip=trunkIn2})
end
local offs = {4, 7, 10, 14, 19, 25, 32, 40}
for i, off in ipairs(offs) do
  furrow(off, rand(418, 450), rand(40, 80), (i <= 3) and BarkRim2 or BarkFur, 0.6, 0.4)
end
print(wait(0))

--@ chunk 483
furBand = trunkIn2 * mask(function(x,y)
  if y < 412 or y > 540 then return 0 end
  return (x > xR_(y) - 46 and x < xR_(y) - 1) and 1 or 0
end)
print(drying(255, 440))
blend(furBand, {angle=1.5708})
print(wait(0))

--@ chunk 484
darkD = function(x, y)
  local u = xR_(y) - x
  local tp = 1 - smoothstep(412, 442, y)
  local bt = smoothstep(500, 540, y)
  local lft = smoothstep(4, 15, u)
  return 14.0 * clamp(math.max(lft, tp, bt), 0, 1)
end
local reg = trunkIn2 * mask(function(x,y) return (y >= 410 and y <= 542 and x < xR_(y) + 3) and 1 or 0 end)
stipple(reg, {pile=OakD, width=2.0, coverage=darkD, feather=0.4, clip=reg, pressure={0.5,0.9}, dips={30,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 485
footV = ((mesaSil:grow(2.5) + rect(722, 398, 170, 30)) * rect(722, 356, 170, 72)) - guardX - fgM
dV = function(x, y)
  local g = smoothstep(366, 402, y)^1.25
  local st = 0.5 + 1.0 * nStr:at01(x + 31, y)
  return 4.6 * g * st
end
local mx = 0
for y = 356, 428, 4 do for x = 722, 892, 4 do local c = dV(x,y) * footV:at(x,y); if c > mx then mx = c end end end
print("max", mx, "area", footV:area())
stipple(footV, {pile=T_m, width=2.6, coverage=dV, feather=0.5, clip=footV, drag={26, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 486
dV2 = function(x, y)
  local bell = math.exp(-((y - 382)/14)^2)
  local g = smoothstep(366, 396, y)
  local st = 0.5 + 1.0 * nStr:at01(x - 57, y + 13)
  return 7.0 * math.max(bell, g^1.5 * 0.8) * st
end
stipple(footV, {pile=T_a, width=3.0, coverage=dV2, feather=0.5, clip=footV, drag={34, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 487
extR = rect(676, 398, 262, 96) * mask(function(x,y) return (y > Atop(x) + 2) and 1 or 0 end) - rect(722, 398, 170, 30) - guardX - fgM - spAll:grow(3) - mesaAll
local fvx0, fvy0, fvx1, fvy1 = 722, 398, 892, 428
extRamp = function(x, y)
  local d = sdRect(x, y, fvx0, fvy0, fvx1, fvy1)
  return 1 - smoothstep(0, 42, d)
end
dExtM = function(x, y)
  local st = 0.5 + 1.0 * nStr:at01(x + 31, y)
  return 4.6 * st * extRamp(x, y)
end
dExtA = function(x, y)
  local st = 0.5 + 1.0 * nStr:at01(x - 57, y + 13)
  return 5.6 * st * extRamp(x, y)
end
print("area", extR:area())
stipple(extR, {pile=T_m, width=2.6, coverage=dExtM, feather=0.5, clip=extR, drag={26, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
stipple(extR, {pile=T_a, width=3.0, coverage=dExtA, feather=0.5, clip=extR, drag={34, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 488
print(crestY(300), crestY(500), crestY(700), crestY(900), crestAng(500), crestAng(800))
for _, k in ipairs({"blade","blade2","lit_blade","tuft","tuft2","spruce","cloud","bird"}) do print(k, type(_G[k])) end

--@ chunk 489
-- find where blade/tuft were defined in the log: print the source lines via the log tool isn't available here; test calling signature by pcall with bad args
local ok, e = pcall(function() return tuft() end)
print(ok, tostring(e))
local ok2, e2 = pcall(function() return blade() end)
print(ok2, tostring(e2))
local ok3, e3 = pcall(function() return tuft2() end)
print(ok3, tostring(e3))

--@ chunk 490
stalkB = brush{kind="rigger", width=2, point=1}
print(stalkB:mark_width(0.2), stalkB:mark_width(0.35), stalkB:mark_width(0.5), stalkB:mark_width(0.8))
function stalk(x0, h, lean, p, pr, headLen)
  local yb = crestY(x0) + 5
  local pts = {}
  local n = 6
  local ph = rand(0, 6.28)
  for i = 0, n do
    local t = i / n
    local x = x0 + lean * h * (t^1.7) + 0.6 * math.sin(ph + t*5.0) * t
    local y = yb - h * t
    pts[#pts+1] = {x, y}
  end
  stalkB:reload(p, 1.0)
  stalkB:stroke(pts, {pressure={pr, pr*0.75, pr*0.55}, ramps={0.0, 0.0}})
  if headLen and headLen > 0 then
    local tx, ty = pts[#pts][1], pts[#pts][2]
    local px, py = pts[#pts-1][1], pts[#pts-1][2]
    local dx, dy = tx - px, ty - py
    local dl = math.sqrt(dx*dx + dy*dy)
    dx, dy = dx/dl, dy/dl
    stalkB:reload(p, 1.0)
    stalkB:stroke({{tx - dx*0.5, ty - dy*0.5}, {tx + dx*headLen*0.5, ty + dy*headLen*0.5}, {tx + dx*headLen, ty + dy*headLen}}, {pressure={0.75, 0.9, 0.35}, ramps={0.0, 0.0}})
  end
end
stalk(655, 34, 0.12, OakD, 0.45, 5)
print(wait(0))

--@ chunk 491
function arch(x0, h, phi0, k, p, pr, seed)
  local yb = crestY(x0) + 6
  local L = h * 1.12
  local n = 10
  local pts = {}
  local x, y = x0, yb
  pts[1] = {x, y}
  for i = 1, n do
    local t = i / n
    local phi = phi0 + k * (t ^ 1.7)
    local ds = L / n
    x = x + math.sin(phi) * ds
    y = y - math.cos(phi) * ds
    pts[#pts+1] = {x, y}
  end
  stalkB:reload(p, 1.0)
  stalkB:stroke(pts, {pressure={pr, pr*0.8, pr*0.35}, ramps={0.0, 0.0}})
  return pts
end
-- a loose clump of tall dry grasses bowing over, right of the lone stalk
local specs = {
  {640, 26, -0.18, -1.5}, {646, 38, -0.10, -1.9}, {652, 30, 0.05, 1.3},
  {660, 46, 0.12, 1.8}, {667, 24, 0.20, 1.6}, {673, 34, -0.05, -1.4},
}
for _, s in ipairs(specs) do
  arch(s[1], s[2], s[3], s[4], OakD, 0.42)
end
print(wait(0))

--@ chunk 492
bandR2 = rect(716, 410, 190, 50) - guardX - fgM - mesaAll
bandRamp = function(x, y)
  local d = sdRect(x, y, 740, 427, 877, 438)
  return 1 - smoothstep(0, 10, d)
end
bdM = function(x, y)
  local st = 0.5 + 1.0 * nStr:at01(x + 31, y)
  return 5.2 * st * bandRamp(x, y)
end
bdA = function(x, y)
  local st = 0.5 + 1.0 * nStr:at01(x - 57, y + 13)
  return 6.4 * st * bandRamp(x, y)
end
stipple(bandR2, {pile=T_m, width=2.6, coverage=bdM, feather=0.5, clip=bandR2, drag={26, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
stipple(bandR2, {pile=T_a, width=3.0, coverage=bdA, feather=0.5, clip=bandR2, drag={34, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 493
print(drying(800,433), drying(800,425), drying(800,445), drying(760,433), drying(740,433), drying(878,433))
print("bandR2", bandR2:at(800,433), "guardX", guardX:at(800,433), "fgM", fgM:at(800,433), "mesaAll", mesaAll:at(800,433))
print("bdM", bdM(800,433), "bdA", bdA(800,433), "ramp", bandRamp(800,433))

--@ chunk 494
barM = (mesaAll * rect(716, 424, 190, 26)) - guardX - fgM
barRamp = function(x, y) return 1 - smoothstep(436, 448, y) end
bar1 = function(x, y)
  local st = 0.5 + 1.0 * nStr:at01(x + 31, y)
  return 9.0 * st * barRamp(x, y)
end
bar2 = function(x, y)
  local st = 0.5 + 1.0 * nStr:at01(x - 57, y + 13)
  return 8.0 * st * barRamp(x, y)
end
print("area", barM:area())
stipple(barM, {pile=T_m, width=2.6, coverage=bar1, feather=0.5, clip=barM, drag={26, 0.0}, pressure={0.4,0.8}, dips={40,0.9,0.3}, cluster=0.0})
stipple(barM, {pile=T_a, width=3.0, coverage=bar2, feather=0.5, clip=barM, drag={34, 0.0}, pressure={0.4,0.8}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 495
fineB = brush{kind="rigger", width=1.2, point=1}
print(fineB:mark_width(0.1), fineB:mark_width(0.2), fineB:mark_width(0.3), fineB:mark_width(0.5), fineB:mark_width(0.8))
fineB2 = brush{kind="round", width=1.6, point=1}
print(fineB2:mark_width(0.1), fineB2:mark_width(0.2), fineB2:mark_width(0.3), fineB2:mark_width(0.5), fineB2:mark_width(0.8))

--@ chunk 496
stemP = pile{{"raw umber",3},{"red earth",0.5},{"bone black",1.0},{"green earth",0.4}}
function stem2(x0, h, phi0, k, head, p, pr0)
  local yb = crestY(x0) + 5
  local n = 12
  local L = h * 1.05
  local x, y = x0, yb
  local pts = {{x, y}}
  local ph = rand(0, 6.28)
  local dirx, diry = 0, -1
  for i = 1, n do
    local t = i / n
    local phi = phi0 + k * (t ^ 1.8) + 0.05 * math.sin(ph + t * 6.0)
    local ds = L / n
    dirx, diry = math.sin(phi), -math.cos(phi)
    x = x + dirx * ds
    y = y + diry * ds
    pts[#pts+1] = {x, y}
  end
  fineB2:reload(p, 1.0)
  fineB2:stroke(pts, {pressure={pr0 or 0.5, (pr0 or 0.5) * 0.7, 0.14}, ramps={0.0, 0.0}})
  if head and head > 0 then
    local tx, ty = pts[#pts][1], pts[#pts][2]
    fineB2:reload(p, 1.0)
    fineB2:stroke({{tx - dirx*1.0, ty - diry*1.0}, {tx + dirx*head*0.5, ty + diry*head*0.5}, {tx + dirx*head, ty + diry*head}}, {pressure={0.55, 0.75, 0.2}, ramps={0.0, 0.0}})
  end
  return pts
end
stem2(646, 46, -0.07, 0.35, 6, stemP, 0.5)
stem2(661, 52, 0.05, 0.55, 6, stemP, 0.5)
stem2(668, 36, 0.13, 0.9, 5, stemP, 0.5)
stem2(641, 30, -0.17, -0.5, 5, stemP, 0.5)
stem2(652, 26, 0.02, 0.3, 4, stemP, 0.5)
stem2(674, 24, 0.22, 1.0, 0, stemP, 0.5)
print(wait(0))

--@ chunk 497
for _, k in ipairs({"PAc","PW1b","PWpk","PW4","PW3b","PW3","PW2","PW1","PWpeach","Hpeach","Hgold","Gpeach","GLOW","GlowP","MistW","MistW2","MistC","MistCool"}) do
  print(k, tostring(_G[k]))
end
print(Atop(690), Atop(720), Atop(740), Atop(880), Atop(900), Atop(940))

--@ chunk 498
dripReg = (mesaSil * rect(728, 340, 160, 50)) - guardX - fgM
dripD1 = function(x, y)
  local g = smoothstep(346, 372, y)
  local st = 0.55 + 0.9 * nStr:at01(x + 11, y + 5)
  return 9.0 * g^1.3 * st
end
dripD2 = function(x, y)
  local g = smoothstep(358, 378, y)
  local st = 0.55 + 0.9 * nStr:at01(x - 23, y + 17)
  return 8.0 * g * st
end
print("area", dripReg:area())
stipple(dripReg, {pile=T_m, width=2.4, coverage=dripD1, feather=0.5, clip=dripReg, drag={30, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
stipple(dripReg, {pile=T_a, width=2.8, coverage=dripD2, feather=0.5, clip=dripReg, drag={40, 0.0}, pressure={0.35,0.75}, dips={40,0.9,0.3}, cluster=0.0})
print(wait(0))

--@ chunk 499
for _, f in ipairs({0.0, 0.3, 0.6, 0.9}) do
  print(f, tostring(mixp(NB.body, tFogC, f)))
end
print("shade", tostring(mixp(NB.shade, tFogC, 0.5)))
print("warm", tostring(mixp(NB.warm, tFogW, 0.5)))
print(shEdge(800, 380), litEdge(800, 380), shEdge(800, 400), litEdge(800, 400), shEdge(800, 350), litEdge(800, 350))
local s = {}
for _, y in ipairs({340, 360, 380, 400, 420}) do s[#s+1] = string.format("y%d: lit %.0f sh %.0f", y, litEdge(800, y), shEdge(800, y)) end
print(table.concat(s, " | "))
for y = 340, 436, 8 do
  local r = {}
  for x = 720, 900, 6 do
    local c = "."
    if cliffM:at(x,y) > 0.5 then c = "M" elseif needleM:at(x,y) > 0.5 then c = "n" elseif towerK:at(x,y) > 0.5 then c = "t" elseif step:at(x,y) > 0.5 then c = "s" end
    r[#r+1] = c
  end
  print(string.format("%3d ", y) .. table.concat(r))
end

--@ chunk 500
print(wait(3*24*60))
print(drying(800,445), drying(878,433), drying(800,372), drying(790,400), drying(760,420))

--@ chunk 501
mesaB = (cliffM + needleM + towerK + step):blur(6)
mReg = rect(722, 348, 170, 96) - fgM - guardX
sideRamp = function(x, y)
  local v = mesaB:at(x, y)
  local ins = clamp(v * 1.8, 0, 1)
  if v < 0.5 then ins = ins * smoothstep(Atop(x) + 2, Atop(x) + 8, y) end
  return ins
end
litW = function(x, y) return 1 - smoothstep(litEdge(x,y)-10, litEdge(x,y)+10, x) end
shW  = function(x, y) return smoothstep(shEdge(x,y)-12, shEdge(x,y)+12, x) end
bandsM = {
  {f=0.30, c=366, wu=13, wd=13},
  {f=0.46, c=379, wu=13, wd=13},
  {f=0.62, c=392, wu=13, wd=13},
  {f=0.78, c=405, wu=13, wd=12},
  {f=0.90, c=417, wu=12, wd=12},
}
function tentY(y, c, wu, wd)
  if y < c then return smoothstep(c - wu, c, y) end
  return 1 - smoothstep(c, c + wd, y)
end
function mesaBand(b, peak)
  local pb = mixp(NB.body,  tFogC, b.f)
  local ps = mixp(NB.shade, tFogC, b.f)
  local pw = mixp(NB.warm,  tFogW, b.f)
  local base = function(x, y) return peak * tentY(y, b.c, b.wu, b.wd) * sideRamp(x, y) end
  stipple(mReg, {pile=pb, width=2.6, coverage=function(x,y) local w = clamp(1 - litW(x,y) - shW(x,y), 0, 1); return base(x,y) * w end,
    feather=0.5, clip=mReg, drag={8, 0.0}, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
  stipple(mReg, {pile=pw, width=2.6, coverage=function(x,y) return base(x,y) * litW(x,y) end,
    feather=0.5, clip=mReg, drag={8, 0.0}, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
  stipple(mReg, {pile=ps, width=2.6, coverage=function(x,y) return base(x,y) * shW(x,y) end,
    feather=0.5, clip=mReg, drag={8, 0.0}, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
end
-- check the maximum coverage and the sideRamp at a few points
for _, p in ipairs({{760,380},{740,380},{736,395},{736,410},{800,400},{878,400},{884,410},{760,430}}) do
  print(p[1], p[2], "sideRamp", string.format("%.2f", sideRamp(p[1], p[2])), "mesaB", string.format("%.2f", mesaB:at(p[1], p[2])), "reg", mReg:at(p[1], p[2]))
end
mesaBand(bandsM[1], 8.0)
print(wait(0))

--@ chunk 502
mesaBand(bandsM[2], 8.0)
mesaBand(bandsM[3], 8.0)
print(wait(0))

--@ chunk 503
mesaBand(bandsM[4], 8.0)
mesaBand(bandsM[5], 8.0)
print(wait(0))

--@ chunk 504
print(wait(2*24*60))
print(drying(800,372), drying(800,400), drying(760,420), drying(850,430))

--@ chunk 505
nV = noise{seed=6101, octaves=3, period=46, persistence=0.55, stretch={1.5708, 5}}
nV2 = noise{seed=6177, octaves=2, period=90, persistence=0.5, stretch={1.5708, 4}}
function mesaStreak(f, peak, yTop, yMid, yBot, thr, seedOff)
  local pb = mixp(NB.body,  tFogC, f)
  local ps = mixp(NB.shade, tFogC, f)
  local pw = mixp(NB.warm,  tFogW, f)
  local base = function(x, y)
    local s = clamp(nV:at01(x + seedOff, y) * 2.4 - thr, 0, 1)
    local prof = smoothstep(yTop, yTop + 5, y) * (1 - smoothstep(yMid, yBot, y))
    return peak * s * prof * sideRamp(x, y)
  end
  stipple(mReg, {pile=pb, width=2.2, coverage=function(x,y) local w = clamp(1 - litW(x,y) - shW(x,y), 0, 1); return base(x,y) * w end,
    feather=0.5, clip=mReg, drag={16, 1.5708}, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
  stipple(mReg, {pile=pw, width=2.2, coverage=function(x,y) return base(x,y) * litW(x,y) end,
    feather=0.5, clip=mReg, drag={16, 1.5708}, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
  stipple(mReg, {pile=ps, width=2.2, coverage=function(x,y) return base(x,y) * shW(x,y) end,
    feather=0.5, clip=mReg, drag={16, 1.5708}, pressure={0.4,0.85}, dips={40,0.9,0.3}, cluster=0.0})
end
mesaStreak(0.18, 6.0, 347, 354, 380, 0.9, 0)
print(wait(0))

--@ chunk 506
mesaStreak(0.08, 7.0, 332, 346, 372, 1.15, 37)
print(wait(0))

--@ chunk 507
rimB = brush{kind="round", width=3, point=1}
rimP1 = pile{{"raw umber",2.0},{"yellow ochre",1.2},{"lead white",0.9},{"red earth",0.5}, medium=0.1}
rimP2 = pile{{"raw umber",2.4},{"yellow ochre",0.9},{"lead white",0.5},{"red earth",0.6}, medium=0.1}
function rimSeg(y0, len, off, p, load, pr)
  local pts = {}
  local n = math.max(2, math.floor(len / 7))
  local ph = rand(0, 6.28)
  for i = 0, n do
    local y = y0 + len * i / n
    local x = xR_(y) - off - 0.5 * math.sin(ph + i * 1.7)
    pts[#pts+1] = {x, y}
  end
  rimB:reload(p, load)
  rimB:stroke(pts, {pressure={pr*0.6, pr, pr*0.5}, ramps={0.25, 0.4}, clip=trunkIn2:grow(1.0)})
end
local y = 414
while y < 536 do
  local len = rand(8, 20)
  local fadeDown = 1 - 0.7 * smoothstep(500, 540, y)
  local pr = rand(0.18, 0.3)
  rimSeg(y, len, rand(2.2, 4.4), (math.random() < 0.6) and rimP1 or rimP2, 0.55 * fadeDown + 0.1, pr)
  y = y + len + rand(3, 11)
end
print(wait(0))

--@ chunk 508
rimGlowReg = trunkIn2:grow(0.5) * mask(function(x,y) return (y >= 412 and y <= 540 and x > xR_(y) - 11 and x < xR_(y) + 1) and 1 or 0 end)
rimGlowD = function(x, y)
  local u = xR_(y) - x
  local fall = math.exp(-math.max(u, 0) / 4.0)
  local ends = smoothstep(412, 434, y) * (1 - smoothstep(500, 540, y))
  local br = 0.45 + 0.9 * nStr:at01(x * 0.2 + 7, y * 0.6)
  return 2.6 * fall * ends * br
end
stipple(rimGlowReg, {pile=BarkRim2, width=1.5, coverage=rimGlowD, feather=0.7, clip=rimGlowReg, pressure={0.3,0.6}, dips={40,0.9,0.3}, cluster=0.0, drag={5, 1.5708}})
print(wait(0))

--@ chunk 509
nBandA = noise{seed=7311, octaves=2, period=230, persistence=0.5}
nBandB = noise{seed=7312, octaves=2, period=160, persistence=0.5}
fogBandReg = rect(560, 424, 450, 230) - fgM:grow(2) - guardX - spAll:grow(5) - mesaAll:grow(8) - Kbody:grow(10)
print("fogBandReg area", fogBandReg:area())
bandsF = {
  {c=470, h=7,  x0=740, x1=1010},
  {c=500, h=9,  x0=600, x1=1010},
  {c=540, h=12, x0=640, x1=1010},
  {c=590, h=16, x0=700, x1=1010},
}
function fogBandCov(i, kind)
  local b = bandsF[i]
  return function(x, y)
    local hm = smoothstep(-0.05, 0.30, nBandA(x * 1.0 + i * 91, i * 37)) * smoothstep(b.x0, b.x0 + 70, x)
    local wob = 4.0 * nBandB(x, i * 53)
    local yy = y - wob
    if kind == "shade" then
      local u = (yy - (b.c + 0.2*b.h)) / b.h
      return 2.4 * math.exp(-u*u) * hm
    else
      local u = (yy - (b.c - 0.9*b.h)) / (0.55*b.h)
      return 1.6 * math.exp(-u*u) * hm
    end
  end
end
-- test: only band 2 (y=500)
local cs = fogBandCov(2, "shade")
local mx = 0
for y = 424, 560, 4 do for x = 560, 1000, 8 do local c = cs(x,y) * fogBandReg:at(x,y); if c > mx then mx = c end end end
print("max shade cov band2", mx)
stipple(fogBandReg, {pile=T_b, width=3.2, coverage=cs, feather=0.7, clip=fogBandReg, pressure={0.3,0.6}, dips={40,0.9,0.3}, cluster=0.0, drag={22, 0.0}})
print(wait(0))

--@ chunk 510
for _, i in ipairs({1, 3, 4}) do
  local cs = fogBandCov(i, "shade")
  stipple(fogBandReg, {pile=T_b, width=3.2, coverage=cs, feather=0.7, clip=fogBandReg, pressure={0.3,0.6}, dips={40,0.9,0.3}, cluster=0.0, drag={22, 0.0}})
end
-- warm lights on the tops of the swells
for i = 1, 4 do
  local cl = fogBandCov(i, "light")
  stipple(fogBandReg, {pile=PfC, width=3.0, coverage=cl, feather=0.7, clip=fogBandReg, pressure={0.3,0.6}, dips={40,0.9,0.3}, cluster=0.0, drag={22, 0.0}})
end
print(wait(0))
