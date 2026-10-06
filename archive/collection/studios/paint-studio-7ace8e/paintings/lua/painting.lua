-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=520, aspect=1.4, linen={15,13}, seed=7, ground={{pile={{"red earth",2},{"yellow ochre",2},{"lead white",1}}, um=80, apply="knife", texture=0.3},{pile={{"lead white",6},{"yellow ochre",0.3},{"raw umber",0.1}}, um=45, apply="brush"}}}; print(W,H)

--@ chunk 2
HZ = 445
h = pencil("2H")
-- horizon
h:line({{0,446},{200,444},{400,445},{700,446},{1000,445}}, {pressure=0.3})
-- town: St Jacobi (slender spire)
h:line({{388,445},{388,392},{402,392},{402,445}}, {pressure=0.35, smooth=false})
h:line({{386,392},{395,352},{404,392}}, {pressure=0.35, smooth=false})
-- St Nikolai (tall, stepped lantern)
h:line({{460,445},{460,378},{482,378},{482,445}}, {pressure=0.35, smooth=false})
h:line({{463,378},{465,362},{471,352},{477,362},{479,378}}, {pressure=0.35, smooth=false})
h:line({{471,352},{471,330}}, {pressure=0.3})
-- St Marien (massive, low cap)
h:line({{548,445},{548,395},{578,395},{578,445}}, {pressure=0.35, smooth=false})
h:line({{546,395},{563,378},{580,395}}, {pressure=0.35, smooth=false})
-- roofs
h:line({{350,445},{352,432},{375,428},{388,430}}, {pressure=0.3})
h:line({{402,428},{430,422},{460,425}}, {pressure=0.3})
h:line({{482,425},{510,420},{530,418},{548,424}}, {pressure=0.3})
h:line({{578,424},{600,428},{625,433},{650,440},{660,445}}, {pressure=0.3})
-- windmills
h:line({{712,445},{712,425}}, {pressure=0.3}); h:line({{700,410},{724,440}}, {pressure=0.3}); h:line({{724,410},{700,440}}, {pressure=0.3})
h:line({{795,445},{795,428}}, {pressure=0.3}); h:line({{784,414},{806,442}}, {pressure=0.3}); h:line({{806,414},{784,442}}, {pressure=0.3})
-- water strip
h:line({{0,458},{150,456},{330,460},{420,463}}, {pressure=0.25})
h:line({{0,470},{160,468},{300,470},{420,463}}, {pressure=0.25})
-- willows along ditch (left)
for _,t in ipairs({{110,575,95},{178,552,80},{236,535,64},{286,522,50}}) do
  local x,y,s = t[1],t[2],t[3]
  h:line({{x-6*s/80,y},{x-5*s/80,y-s*0.55}}, {pressure=0.35})
  h:line({{x+6*s/80,y},{x+5*s/80,y-s*0.55}}, {pressure=0.35})
  h:sketch({{x-s*0.45,y-s*0.6},{x-s*0.5,y-s*1.05},{x,y-s*1.3},{x+s*0.5,y-s*1.05},{x+s*0.45,y-s*0.6}}, {pressure=0.25})
end
-- ditch line
h:line({{60,600},{150,568},{250,540},{340,520},{420,505}}, {pressure=0.25})
-- haycocks
for _,t in ipairs({{330,503,9},{372,508,8},{612,497,8},{655,505,10},{560,515,11}}) do
  h:line({{t[1]-t[3],t[2]},{t[1]-t[3]*0.6,t[2]-t[3]*1.1},{t[1],t[2]-t[3]*1.4},{t[1]+t[3]*0.6,t[2]-t[3]*1.1},{t[1]+t[3],t[2]}}, {pressure=0.3})
end
-- path
h:line({{360,714},{420,660},{470,625},{520,600},{560,585}}, {pressure=0.3})
h:line({{520,714},{540,660},{560,625},{580,595},{590,585}}, {pressure=0.3})
-- figures
h:line({{503,610},{505,572},{511,562}}, {pressure=0.35})
h:line({{518,610},{517,572},{511,562}}, {pressure=0.35})
h:line({{531,612},{527,585},{534,570},{541,585},{544,612}}, {pressure=0.35})

--@ chunk 3
sky1 = pile{{"smalt",4},{"lead white",3},{"bone black",0.12}, medium=0.2}
sky2 = pile{{"smalt",2.5},{"lead white",5}, medium=0.2}
sky3 = pile{{"pale smalt",1},{"lead white",6},{"chrome yellow",0.15}, medium=0.2}
sky4 = pile{{"lead white",7},{"chrome yellow",0.5},{"yellow ochre",0.2}, medium=0.2}
sky5 = pile{{"lead white",7},{"yellow ochre",0.6},{"vermilion",0.22}, medium=0.2}
local bands = {{sky1,0,120},{sky2,110,235},{sky3,225,335},{sky4,325,405},{sky5,395,455}}
for i,b in ipairs(bands) do
  work(rect(0,b[2],1000,b[3]-b[2]), {hand="broad", pile=b[1], angle=0, angle_jitter=0.03, coverage=2.5, fill=true, length={120,300}, pressure={0.5,0.8}, curve={0.02,0.01}, seed=30+i})
end

--@ chunk 4
work(rect(0,-10,1000,95), {hand="broad", pile=sky1, angle=0, angle_jitter=0.02, coverage=2.5, fill=true, length={150,300}, pressure={0.6,0.9}, seed=41})
work(rect(0,215,1000,60), {hand="broad", pile=sky3, angle=0, angle_jitter=0.02, coverage=2, fill=true, length={150,300}, pressure={0.6,0.9}, seed=42})
SKYM = rect(0,0,1000,452)
for i=1,3 do
  blend(SKYM, {angle=0, ruler=true, length={150,400}, seed=50+i})
end

--@ chunk 5
print(drying(500,100), drying(500,400))
sky25 = pile{{"smalt",1.2},{"pale smalt",1},{"lead white",6},{"chrome yellow",0.05}, medium=0.2}
sky0 = pile{{"smalt",5},{"lead white",2},{"bone black",0.15},{"red earth",0.05}, medium=0.2}
work(rect(0,180,1000,70), {hand="broad", pile=sky25, angle=0, angle_jitter=0.02, coverage=2.5, fill=true, length={150,300}, pressure={0.6,0.9}, seed=61})
work(rect(0,-10,1000,60), {hand="broad", pile=sky0, angle=0, angle_jitter=0.02, coverage=2.2, fill=true, length={150,300}, pressure={0.6,0.9}, seed=62})
for i=1,2 do
  blend(rect(0,0,1000,300), {angle=0, ruler=true, length={150,400}, seed=70+i})
end

--@ chunk 6
LAND = below(function(x) return 446 + 1.2*math.sin(x/70) + 0.8*math.sin(x/23) end)
under = pile{{"raw umber",3},{"red earth",1},{"yellow ochre",1.5}, medium=0.55}
underfar = pile{{"raw umber",1},{"pale smalt",2},{"lead white",1}, medium=0.5}
work(LAND * rect(0,440,1000,40), {hand="body", pile=underfar, angle=0, coverage=1.8, fill=true, clip=true, length={40,90}, pressure={0.4,0.7}, seed=81})
work(LAND * rect(0,478,1000,240), {hand="broad", pile=under, angle=function(x,y) return 0.1*math.sin(x/200) end, coverage=1.6, fill=true, length={80,200}, pressure={0.4,0.7}, seed=82})

--@ chunk 7
print(wait(26*60)); print(drying(500,50), drying(500,300), drying(500,430), drying(500,600), drying(500,455))

--@ chunk 8
gz = pile{{"smalt",3},{"lead white",0.6},{"bone black",0.05}, medium=0.72}
local m = rect(0,-10,1000,230)
work(m, {hand="glaze", pile=gz, angle=0, clip=true, coverage=2, fill=true, load_at=function(x,y) return clamp(0.9 - y/230, 0.05, 0.9) end, length={200,400}, seed=91})
for i=1,3 do blend(rect(0,0,1000,240), {angle=0, ruler=true, length={200,450}, seed=92+i}) end

--@ chunk 9
glow1 = pile{{"lead white",6},{"yellow ochre",0.5},{"chrome yellow",0.3},{"vermilion",0.12}, medium=0.35}
glow2 = pile{{"lead white",6},{"yellow ochre",0.8},{"vermilion",0.45}, medium=0.3}
local sunx = 600
work(rect(0,372,1000,48), {hand="body", pile=glow1, angle=0, angle_jitter=0.02, coverage=2, fill=true, clip=true, length={60,140}, pressure={0.4,0.7}, load_at=function(x,y) return clamp(0.25+0.4*(y-372)/48 + 0.2*math.exp(-((x-sunx)/250)^2),0.1,0.9) end, seed=101})
work(rect(0,412,1000,40), {hand="body", pile=glow2, angle=0, angle_jitter=0.02, coverage=2.4, fill=true, clip=true, length={60,140}, pressure={0.5,0.8}, load_at=function(x,y) return clamp(0.4+0.4*(y-412)/40 + 0.25*math.exp(-((x-sunx)/250)^2),0.1,0.95) end, seed=102})
for i=1,2 do blend(rect(0,368,1000,85), {angle=0, ruler=true, length={150,350}, seed=103+i}) end

--@ chunk 10
print(wait(24*60)); print(drying(500,50), drying(500,380), drying(500,430), drying(500,600))

--@ chunk 11
print(wait(14*60)); print(drying(500,430), drying(200,445), drying(800,420))

--@ chunk 12
function spline(pts, n)
  local out = {}
  local function cr(p0,p1,p2,p3,t)
    local t2,t3=t*t,t*t*t
    return 0.5*((2*p1)+(-p0+p2)*t+(2*p0-5*p1+4*p2-p3)*t2+(-p0+3*p1-3*p2+p3)*t3)
  end
  for i=1,#pts-1 do
    local p0=pts[math.max(1,i-1)]; local p1=pts[i]; local p2=pts[i+1]; local p3=pts[math.min(#pts,i+2)]
    for k=0,n-1 do local t=k/n
      out[#out+1]={cr(p0[1],p1[1],p2[1],p3[1],t), cr(p0[2],p1[2],p2[2],p3[2],t)} end
  end
  out[#out+1]=pts[#pts]
  return out
end
function strip(pts, wmax, seed, taper)
  local s = spline(pts, 12)
  local nz = noise{seed=seed, period=40, octaves=3}
  local ws = {}
  for i,p in ipairs(s) do
    local t = (i-1)/(#s-1)
    local tp = math.sin(math.pi*t)^(taper or 0.6)
    ws[i] = math.max(0.3, wmax*tp*(0.75+0.35*nz(p[1],p[2])))
  end
  return ribbon(s, ws)
end
cloudA = pile{{"pale smalt",1.2},{"lead white",4},{"vermilion",0.25},{"raw umber",0.25}, medium=0.35}
cloudB = pile{{"smalt",1},{"lead white",3},{"vermilion",0.15},{"raw umber",0.3}, medium=0.35}
cloudLit = pile{{"lead white",5},{"vermilion",0.35},{"yellow ochre",0.5}, medium=0.3}
-- low long bar
CL1 = strip({{140,372},{300,369},{470,371},{640,368},{780,370},{920,367}}, 5.5, 11)
CL1b = strip({{470,393},{560,391},{660,394},{760,392},{830,393}}, 3.2, 12)
CL1c = strip({{40,354},{130,356},{230,353},{300,355}}, 3, 13)
-- upper wisps around y~150-190
CU1 = strip({{180,156},{260,153},{330,158},{420,156}}, 7, 14)
CU2 = strip({{520,168},{600,166},{680,170},{760,167},{880,163}}, 6, 15)
CU3 = strip({{700,128},{800,125},{930,128},{1010,124}}, 5, 16)
CU4 = strip({{20,190},{110,192},{190,189}}, 4, 17)
CLOW = CL1 + CL1b + CL1c
CUP = CU1 + CU2 + CU3 + CU4
work(CLOW, {hand="detail", pile=cloudA, angle=0, coverage=3, fill=true, length={10,30}, seed=121})
work(CUP, {hand="detail", pile=cloudB, angle=0, coverage=3, fill=true, length={10,30}, seed=122})

--@ chunk 13
blend(CLOW:grow(5), {angle=0, length={40,120}, tool={kind="badger", width=14}, seed=131})
blend(CUP:grow(7), {angle=0, length={40,120}, tool={kind="badger", width=18}, seed=132})

--@ chunk 14
print(wait(20*60)); print(drying(500,370), drying(300,156))
-- far land strip
FAR = rect(0,444,1000,30) * below(function(x) return 445.5 + 0.6*math.sin(x/37) end)
-- roofs
math.randomseed(77)
local pts = {{345,450}}
local x = 345
local hs = function(x) -- rooftop envelope: higher toward the town centre
  return 9 + 12*math.exp(-((x-500)/120)^2) end
while x < 668 do
  local w = rand(7,16); local hh = hs(x)*rand(0.75,1.15)
  local kind = math.random()
  if kind < 0.55 then -- gable end facing us
    pts[#pts+1] = {x, 445-hh*0.6}; pts[#pts+1] = {x+w/2, 445-hh}; pts[#pts+1] = {x+w, 445-hh*0.6}
  else -- long roof ridge
    pts[#pts+1] = {x+1.5, 445-hh*0.8}; pts[#pts+1] = {x+w-1.5, 445-hh*0.8}
  end
  x = x + w
end
pts[#pts+1] = {668,450}
ROOFS = poly(pts)
-- naves
NAVES = poly({{402,446},{402,420},{408,414},{452,414},{458,420},{458,446}})
  + poly({{482,446},{482,414},{489,405},{540,405},{546,414},{546,446}})
  + poly({{578,446},{578,418},{584,409},{636,409},{642,418},{642,446}})
-- towers
JAC = rect(388,392,14,56) + poly({{387,393},{389,386},{395,350},{401,386},{403,393}}) + rect(386,390,3,5)+rect(401,390,3,5)
NIK = rect(460,378,22,70) + poly({{463,379},{464,370},{478,370},{479,379}}) + ellipse(471,363,7,8) + rect(469,348,4,10) + ellipse(471,346,3.2,3.6) + rect(470.4,329,1.2,16)
MAR = rect(548,395,30,53) + poly({{546,396},{563,379},{580,396}}) + rect(562.4,372,1.2,8)
TOWN = ROOFS + NAVES + JAC + NIK + MAR
-- windmills: bodies and trestles
MILLB = rect(707,419,10,13) + poly({{707,419},{712,415},{717,419}}) + poly({{709,432},{711,432},{708,446},{705,446}}) + poly({{713,432},{715,432},{719,446},{716,446}}) + rect(711,430,2,16)
     + rect(790,422,9,12) + poly({{790,422},{794.5,418},{799,422}}) + poly({{792,434},{794,434},{791,446},{788,446}}) + poly({{795,434},{797,434},{801,446},{798,446}}) + rect(793.5,432,2,14)
townp = pile{{"pale smalt",2},{"lead white",3},{"raw umber",0.7},{"vermilion",0.12}, medium=0.3}
farp = pile{{"pale smalt",2},{"lead white",3.5},{"raw umber",0.5},{"green earth",0.6}, medium=0.3}
work(FAR, {hand="detail", pile=farp, angle=0, coverage=3, fill=true, length={10,30}, seed=141})
work(TOWN + MILLB, {hand="detail", pile=townp, angle=math.pi/2, coverage=3.5, fill=true, length={4,12}, seed=142})

--@ chunk 15
print(wait(22*60)); print(drying(470,400), drying(500,455), drying(500,370))

--@ chunk 16
g1 = pile{{"green earth",2},{"yellow ochre",1.2},{"raw umber",0.6},{"pale smalt",1},{"lead white",1.6}, medium=0.25}
g2 = pile{{"green earth",2},{"yellow ochre",1.5},{"raw umber",0.9},{"Prussian blue",0.06},{"lead white",0.8}, medium=0.25}
g3 = pile{{"green earth",2},{"yellow ochre",1.2},{"raw umber",1.4},{"Prussian blue",0.1},{"lead white",0.3}, medium=0.2}
g4 = pile{{"raw umber",2},{"green earth",2},{"yellow ochre",0.8},{"Prussian blue",0.15},{"bone black",0.12}, medium=0.2}
local wav = function(y0,a,s) return function(x) return y0 + a*math.sin(x/s) + 0.5*a*math.sin(x/(s*0.37)+1) end end
B1 = rect(0,466,1000,40) * above(wav(503,3,90))
B2 = below(wav(503,3,90)) * above(wav(548,5,120))
B3 = below(wav(548,5,120)) * above(wav(605,8,150))
B4 = below(wav(605,8,150))
work(B1, {hand="body", pile=g1, angle=0, angle_jitter=0.04, coverage=2.2, fill=true, clip=true, length={25,70}, pressure={0.4,0.7}, seed=161})
work(B2, {hand="body", pile=g2, angle=0, angle_jitter=0.06, coverage=2.2, fill=true, length={25,70}, pressure={0.4,0.7}, seed=162})
work(B3, {hand="body", pile=g3, angle=0.02, angle_jitter=0.08, coverage=2.2, fill=true, length={30,80}, pressure={0.4,0.8}, seed=163})
work(B4, {hand="body", pile=g4, angle=0.03, angle_jitter=0.12, coverage=2.5, fill=true, length={30,90}, pressure={0.5,0.8}, seed=164})
blend(rect(0,470,1000,250), {angle=0, length={60,200}, seed=165})

--@ chunk 17
print(wait(24*60)); print(drying(470,400), drying(500,455), drying(500,480), drying(500,650))

--@ chunk 18
print(wait(36*60)); print(drying(470,400), drying(395,420), drying(500,455), drying(500,480), drying(500,650))

--@ chunk 19
town2 = pile{{"smalt",2},{"lead white",2.2},{"raw umber",1},{"vermilion",0.1}, medium=0.3}
far2 = pile{{"pale smalt",2},{"green earth",1},{"raw umber",0.9},{"lead white",2}, medium=0.3}
-- town trees
TTREES = ellipse(352,440,9,7) + ellipse(364,437,8,9) + ellipse(338,442,10,5) + ellipse(655,438,10,8) + ellipse(672,441,12,6) + ellipse(690,443,10,4)
  + ellipse(250,442,22,4) + ellipse(870,442,30,4) + ellipse(930,443,20,3)
TTREES = TTREES:roughen(1.5, 6, 191)
work(FAR, {hand="detail", pile=far2, angle=0, coverage=3, fill=true, length={10,30}, seed=191})
work(TOWN + MILLB, {hand="detail", pile=town2, angle=math.pi/2, coverage=3.5, fill=true, length={4,12}, seed=192})
work(TTREES, {hand="detail", pile=far2, angle=0, coverage=3, fill=true, length={4,10}, seed=193})
-- sails
sb = brush{kind="round", width=1.2, point=0.6}
for _,m in ipairs({{712,420,0.35,15},{794.5,423,0.85,13}}) do
  for k=0,3 do
    local a = m[3] + k*math.pi/2
    sb:load(town2, 0.5)
    sb:stroke({{m[1]+2*math.cos(a), m[2]+2*math.sin(a)},{m[1]+m[4]*math.cos(a), m[2]+m[4]*math.sin(a)}}, {pressure={0.45,0.35}})
    -- lattice of the sail: a second parallel line offset
    local ox, oy = -math.sin(a)*2.2, math.cos(a)*2.2
    sb:stroke({{m[1]+5*math.cos(a)+ox, m[2]+5*math.sin(a)+oy},{m[1]+m[4]*math.cos(a)+ox, m[2]+m[4]*math.sin(a)+oy}}, {pressure={0.3,0.25}})
  end
end

--@ chunk 20
hay = pile{{"yellow ochre",2},{"lead white",1.4},{"green earth",0.6},{"raw umber",0.6},{"vermilion",0.03}, medium=0.25}
MOWN = poly({{400,512},{560,503},{760,500},{880,512},{960,540},{700,552},{520,556},{380,548}}, true):roughen(2, 20, 201)
work(MOWN, {hand="body", pile=hay, angle=0, angle_jitter=0.05, coverage=2.4, fill=true, clip=true, length={20,60}, pressure={0.4,0.7}, seed=201})
blend(MOWN:grow(3), {angle=0, length={30,90}, seed=202})

--@ chunk 21
hay2 = pile{{"yellow ochre",1.5},{"green earth",1.5},{"raw umber",0.8},{"lead white",1.1}, medium=0.25}
FIELD = poly({{300,508},{620,502},{1000,496},{1000,533},{640,537},{300,540}})
local blob = MOWN:grow(6)
work(blob - FIELD, {hand="body", pile=g2, angle=0, angle_jitter=0.05, coverage=4, fill=true, clip=true, length={20,60}, pressure={0.5,0.8}, seed=211})
work(FIELD, {hand="body", pile=hay2, angle=0, angle_jitter=0.03, coverage=3, fill=true, clip=true, length={30,80}, pressure={0.5,0.8}, seed=212})
blend(rect(280,490,720,80), {angle=0, ruler=true, length={60,200}, seed=213})

--@ chunk 22
print(wait(48*60)); print(drying(470,400), drying(395,420), drying(500,455), drying(500,520), drying(300,560))

--@ chunk 23
print(wait(60*60)); print(drying(470,400), drying(395,420), drying(500,455), drying(500,520), drying(300,560))

--@ chunk 24
town3 = pile{{"smalt",2},{"raw umber",1.6},{"bone black",0.25},{"lead white",1.2},{"vermilion",0.1}, medium=0.25}
local function stepgable(x, w, hh, eave)
  -- stepped gable facing us: returns polygon points (base at 446)
  local p = {{x,446},{x,446-eave}}
  local steps = math.max(2, math.floor(w/3))
  local sw = (w/2)/steps
  for i=1,steps do
    local yy = 446-eave-(hh-eave)*i/steps
    p[#p+1] = {x+(i-1)*sw, yy}; p[#p+1] = {x+i*sw, yy}
  end
  for i=steps,1,-1 do
    local yy = 446-eave-(hh-eave)*i/steps
    p[#p+1] = {x+w-i*sw, yy}; p[#p+1] = {x+w-(i-1)*sw, yy}
  end
  p[#p+1] = {x+w,446-eave}; p[#p+1] = {x+w,446}
  return poly(p)
end
local H2 = rect(330,437,352,10)
local x = 332
local k = 0
while x < 675 do
  k = k + 1
  local w = rand(8,14)
  local hh = (10 + 7*math.exp(-((x-500)/140)^2)) * rand(0.85,1.15)
  if k % 3 ~= 0 then H2 = H2 + stepgable(x, w, hh, hh*0.55)
  else H2 = H2 + poly({{x,446},{x,446-hh*0.6},{x+w*0.5,446-hh*0.95},{x+w,446-hh*0.6},{x+w,446}}) end
  x = x + w + rand(-1,1.5)
end
local N2 = NAVES + stepgable(452,10,32,20) + stepgable(538,10,44,30) + stepgable(634,10,40,26)
for bx=410,450,8 do N2 = N2 + rect(bx,420,1.6,26) end
for bx=490,540,8 do N2 = N2 + rect(bx,410,1.6,36) end
for bx=586,634,8 do N2 = N2 + rect(bx,414,1.6,32) end
local J2 = rect(387,393,16,54) + poly({{386,394},{389,387},{395,351},{401,387},{404,394}}) + rect(385,389,3,6) + rect(402,389,3,6)
local bell = poly({{462,370},{463,366},{466,363},{468,360},{468.6,357},{471,355.5},{473.4,357},{474,360},{476,363},{479,366},{480,370}}, true)
local K2 = rect(459,379,24,68) + rect(462,369,18,11) + bell + rect(468.8,349,4.4,8) + poly({{468,349.5},{469,346},{471,344},{473,346},{474,349.5}}, true) + rect(470.5,330,1,15)
local M2 = rect(547,396,32,51) + poly({{545,397},{563,381},{581,397}}) + rect(562.5,373,1,9)
TOWN2 = H2 + N2 + J2 + K2 + M2
work(TOWN2, {hand="detail", pile=town3, angle=math.pi/2, coverage=4, fill=true, length={4,12}, seed=241})

--@ chunk 25
skyfix = pile{{"lead white",7},{"chrome yellow",0.35},{"yellow ochre",0.3},{"vermilion",0.06}, medium=0.25}
local bellish = poly({{462,370},{463,366},{466,363},{468,360},{468.6,357},{471,355.5},{473.4,357},{474,360},{476,363},{479,366},{480,370}}, true) + rect(462,369,18,11) + rect(468.8,349,4.4,8)
GHOST = ellipse(471,363,8.5,9.5) - bellish:grow(0.4)
work(GHOST, {hand="detail", pile=skyfix, tool={kind="round", width=1.5}, angle=0, coverage=4, fill=true, length={3,8}, seed=251})
treep = pile{{"smalt",1.5},{"raw umber",1.6},{"green earth",1.2},{"bone black",0.2},{"lead white",1.3}, medium=0.25}
work(TTREES, {hand="detail", pile=treep, angle=0, coverage=4, fill=true, length={3,8}, seed=252})
local SAILS = nil
for _,m in ipairs({{712,420,0.35},{794.5,423,0.85}}) do
  local L = (m[1]==712) and 15 or 13
  for k=0,3 do
    local a = m[3] + k*math.pi/2
    local dx,dy = math.cos(a), math.sin(a); local px,py = -dy, dx
    local q = poly({{m[1]+2.5*dx, m[2]+2.5*dy},{m[1]+L*dx, m[2]+L*dy},{m[1]+L*dx+2.6*px, m[2]+L*dy+2.6*py},{m[1]+4.5*dx+2.6*px, m[2]+4.5*dy+2.6*py}})
    q = q + ribbon({{m[1],m[2]},{m[1]+L*dx, m[2]+L*dy}}, 0.8)
    SAILS = SAILS and (SAILS + q) or q
  end
end
millp = pile{{"smalt",2},{"raw umber",1.4},{"bone black",0.2},{"lead white",1.6},{"vermilion",0.1}, medium=0.25}
work(MILLB + SAILS, {hand="detail", pile=millp, tool={kind="round", width=1.2, point=0.5}, angle=math.pi/2, coverage=4, fill=true, length={2,6}, seed=253})

--@ chunk 26
far3 = pile{{"pale smalt",2},{"smalt",0.5},{"green earth",1.2},{"raw umber",1.1},{"lead white",1.6}, medium=0.25}
waterp = pile{{"lead white",6},{"yellow ochre",0.6},{"vermilion",0.22},{"pale smalt",0.4}, medium=0.25}
WATER = strip({{-10,459},{80,458.5},{180,459.5},{260,460},{330,461}}, 5, 261, 0.25) + strip({{690,461},{800,460},{900,461},{1010,460}}, 4, 262, 0.3)
FAR2 = rect(0,445,1000,32) * below(function(x) return 446.3 + 0.5*math.sin(x/37) end)
work(FAR2 - WATER - TOWN2 - TTREES - MILLB, {hand="detail", pile=far3, angle=0, coverage=3.5, fill=true, length={8,24}, seed=261})
work(WATER, {hand="detail", pile=waterp, angle=0, coverage=3.5, fill=true, length={10,30}, seed=262})

--@ chunk 27
hedge = pile{{"green earth",1.5},{"raw umber",1.3},{"smalt",0.7},{"lead white",0.9}, medium=0.25}
far4 = pile{{"green earth",1.5},{"pale smalt",1.5},{"raw umber",1},{"yellow ochre",0.4},{"lead white",1.2}, medium=0.25}
-- tone variation stipple in the far plain
stipple(FAR2 * below(function(x) return 463 end) - WATER, {pile=far4, width=2.2, coverage=0.9, cluster=0.5, seed=271})
-- hedgerows
local HEDGES = strip({{0,466},{120,465.5},{240,466.5},{320,466}}, 2.2, 272, 0.3) + strip({{420,469},{560,468},{700,469},{820,468.5}}, 2.4, 273, 0.3)
  + strip({{0,474},{200,473},{400,474.5},{600,473.5},{800,474.5},{1000,473.5}}, 3.2, 274, 0.2) + strip({{600,452},{680,451.5},{760,452}}, 1.5, 275, 0.3)
HEDGES = HEDGES:roughen(0.6, 4, 276)
work(HEDGES, {hand="detail", pile=hedge, angle=0, coverage=3, fill=true, length={4,12}, seed=277})
-- distant clumps of trees, many small crowns
local CL = nil
for _,c in ipairs({{250,446,40,6},{870,446,50,7},{935,446,30,5},{100,446,24,4},{980,446,18,4}}) do
  local n = math.floor(c[3]/3)
  for i=1,n do
    local x = c[1] + rand(-c[3]/2, c[3]/2)
    local hh = c[4] * (0.6 + 0.5*math.random()) * (1 - 0.6*math.abs(x-c[1])/(c[3]/2))
    local e = ellipse(x, 446 - hh*0.6, rand(1.6,3.2), hh*0.6)
    CL = CL and (CL + e) or e
  end
end
DCLUMPS = CL:roughen(0.5, 3, 278)
work(DCLUMPS, {hand="detail", pile=treep, tool={kind="round", width=1.3, point=0.5}, angle=math.pi/2, coverage=4, fill=true, length={2,6}, seed=279})

--@ chunk 28
local CL = nil
for _,c in ipairs({{250,24,9},{870,32,10},{930,22,8},{352,14,11},{662,20,10}}) do
  local n = math.floor(c[2]/1.8)
  for i=1,n do
    local x = c[1] + rand(-c[2], c[2])
    local f = 1 - 0.55*(math.abs(x-c[1])/c[2])^1.5
    local hh = c[3] * (0.65 + 0.45*math.random()) * f
    local rx = rand(2,3.8)
    local e = ellipse(x, 446 - hh + rx*0.9, rx, rx*1.05) + rect(x-rx*0.8, 446-hh+rx, rx*1.6, hh-rx+1)
    CL = CL and (CL + e) or e
  end
end
DCL2 = CL:roughen(0.6, 3, 281) - TOWN2
work(DCL2, {hand="detail", pile=treep, tool={kind="round", width=1.3, point=0.5}, angle=math.pi/2, coverage=4, fill=true, length={2,6}, seed=282})

--@ chunk 29
print(wait(60*60)); print(drying(470,400), drying(250,440), drying(500,455), drying(500,470), drying(300,520))

--@ chunk 30
L1 = pile{{"green earth",1.5},{"pale smalt",1.2},{"raw umber",1},{"yellow ochre",0.6},{"lead white",1.2}, medium=0.25}
Lhay = pile{{"yellow ochre",1.2},{"lead white",1.2},{"raw umber",0.8},{"green earth",0.8},{"pale smalt",0.4}, medium=0.25}
L2 = pile{{"green earth",2},{"yellow ochre",1},{"raw umber",1.2},{"Prussian blue",0.08},{"lead white",0.6}, medium=0.25}
L3 = pile{{"green earth",2},{"raw umber",1.6},{"yellow ochre",0.8},{"Prussian blue",0.12},{"lead white",0.2}, medium=0.22}
L4 = pile{{"raw umber",2.2},{"green earth",2},{"Prussian blue",0.2},{"bone black",0.15},{"yellow ochre",0.5}, medium=0.2}
local wv = function(y0,a,s,ph) return function(x) return y0 + a*math.sin(x/s+ph) + 0.4*a*math.sin(x/(s*0.41)+2*ph) end end
LANDB = below(function(x) return 477 + 0.8*math.sin(x/50) end)
Z1 = LANDB * above(wv(503,2,110,0))
Z2 = below(wv(503,2,110,0)) * above(wv(556,4,140,1))
Z3 = below(wv(556,4,140,1)) * above(wv(616,6,170,2))
Z4 = below(wv(616,6,170,2))
FIELD2 = poly({{300,506},{620,501},{1000,496},{1000,534},{640,538},{300,541}}, false):roughen(1.2, 30, 301)
work(Z1, {hand="body", pile=L1, angle=0, angle_jitter=0.03, coverage=2.5, fill=true, clip=true, length={25,70}, pressure={0.4,0.7}, seed=301})
work(Z2 - FIELD2, {hand="body", pile=L2, angle=0, angle_jitter=0.05, coverage=2.5, fill=true, length={25,70}, pressure={0.4,0.7}, seed=302})
work(FIELD2, {hand="body", pile=Lhay, angle=0, angle_jitter=0.03, coverage=2.5, fill=true, clip=true, length={25,70}, pressure={0.4,0.7}, seed=303})
work(Z3, {hand="body", pile=L3, angle=0.02, angle_jitter=0.08, coverage=2.5, fill=true, length={30,80}, pressure={0.4,0.8}, seed=304})
work(Z4, {hand="body", pile=L4, angle=0.03, angle_jitter=0.12, coverage=2.8, fill=true, length={30,90}, pressure={0.5,0.8}, seed=305})
blend(below(function(x) return 482 end), {angle=0, length={60,180}, seed=306})

--@ chunk 31
print(wait(72*60)); print(drying(500,455), drying(500,470), drying(300,520), drying(300,650))

--@ chunk 32
mist = pile{{"lead white",5},{"pale smalt",1},{"vermilion",0.06},{"yellow ochre",0.2}, medium=0.6}
local nz = noise{seed=321, period=180, octaves=3}
MISTB = rect(0,456,1000,56) - TOWN2 - DCL2
work(MISTB, {hand="glaze", pile=mist, angle=0, angle_jitter=0.02, clip=true, coverage=2, fill=true, length={120,300},
  load_at=function(x,y) return clamp(0.75*math.exp(-((y-478)/11)^2) * (0.75+0.35*nz(x,y)), 0.02, 0.9) end, seed=322})
for i=1,2 do blend(MISTB, {angle=0, ruler=true, length={100,300}, seed=323+i}) end

--@ chunk 33
local band = below(function(x) return 497 + 2*math.sin(x/60) end) * above(function(x) return 522 + 2*math.sin(x/45) end)
blend(band, {angle=math.pi/2, angle_jitter=0.2, length={12,28}, clip=false, tool={kind="badger", width=16}, seed=331})
blend(band, {angle=math.pi/2 + 0.3, angle_jitter=0.2, length={10,22}, clip=false, tool={kind="badger", width=12}, seed=332})

--@ chunk 34
print(wait(24*60)); print(drying(500,478), drying(200,500))

--@ chunk 35
function taperstrip(pts, w0, w1, seed, jit)
  local s = spline(pts, 12)
  local nz = noise{seed=seed, period=30, octaves=3}
  local ws = {}
  for i,p in ipairs(s) do local t=(i-1)/(#s-1); ws[i] = math.max(0.3, lerp(w0,w1,t)*(1+(jit or 0.25)*nz(p[1],p[2]))) end
  return ribbon(s, ws), s
end
DITCHPTS = {{-10,650},{80,626},{170,591},{250,561},{320,539},{390,523},{450,513},{500,507}}
DITCH, DITCHS = taperstrip(DITCHPTS, 9, 1.2, 351, 0.3)
bankp = pile{{"raw umber",2},{"bone black",0.3},{"green earth",1},{"Prussian blue",0.1}, medium=0.2}
ditchw = pile{{"lead white",5},{"pale smalt",1.4},{"yellow ochre",0.3},{"raw umber",0.3}, medium=0.25}
BANK = DITCH:grow(2.2):offset(0) 
work(DITCH:grow(2.5), {hand="detail", pile=bankp, angle=-0.3, coverage=3, fill=true, length={6,16}, seed=352})
work(DITCH:shrink(0.6), {hand="detail", pile=ditchw, angle=-0.33, coverage=3.5, fill=true, length={6,20}, seed=353})
-- path
PATH = poly({{372,716},{540,716},{552,680},{566,640},{580,610},{592,590},{606,570},{618,552},{628,541},{620,541},{600,552},{572,568},{540,588},{505,612},{460,642},{415,675}}, true):roughen(2, 18, 354)
pathp = pile{{"lead white",1.4},{"yellow ochre",1},{"raw umber",1.3},{"pale smalt",0.6},{"green earth",0.3}, medium=0.22}
work(PATH, {hand="body", pile=pathp, tool="filbert 6", angle=function(x,y) return -1.0 end, angle_jitter=0.2, coverage=3, fill=true, clip=true, length={15,40}, pressure={0.4,0.7}, seed=355})

--@ chunk 36
NPATH, NPS = taperstrip({{482,720},{468,690},{474,660},{500,632},{536,607},{572,586},{600,566},{620,548},{632,538}}, 80, 5, 361, 0.12)
NPATH = NPATH:roughen(1.5, 14, 362)
pathp2 = pile{{"raw umber",1.6},{"lead white",0.7},{"pale smalt",0.8},{"green earth",0.6},{"yellow ochre",0.35}, medium=0.2}
local rest = PATH:grow(3) - NPATH
local upper = rest * above(function(x) return 616 end)
local lower = rest * below(function(x) return 616 end)
work(upper, {hand="body", pile=L3, tool="filbert 6", angle=0, angle_jitter=0.1, coverage=4, fill=true, clip=true, length={10,30}, seed=363})
work(lower, {hand="body", pile=L4, tool="filbert 6", angle=0, angle_jitter=0.1, coverage=4, fill=true, clip=true, length={10,30}, seed=364})
work(NPATH, {hand="body", pile=pathp2, tool="filbert 6", angle=-0.7, angle_jitter=0.2, coverage=3.5, fill=true, clip=true, length={12,35}, seed=365})

--@ chunk 37
print(wait(72*60)); print(drying(500,650), drying(420,690), drying(200,590))

--@ chunk 38
print(wait(72*60)); print(drying(500,650), drying(420,690), drying(560,690), drying(600,560))

--@ chunk 39
local rest = PATH:grow(4) - NPATH
L4b = pile{{"raw umber",2.2},{"green earth",2},{"Prussian blue",0.22},{"bone black",0.2},{"yellow ochre",0.4}, medium=0.15}
L3b = pile{{"green earth",2},{"raw umber",1.7},{"yellow ochre",0.7},{"Prussian blue",0.14},{"lead white",0.15}, medium=0.15}
work(rest * below(function(x) return 612 end), {hand="body", pile=L4b, tool="filbert 5", angle=-0.1, angle_jitter=0.15, coverage=5, fill=true, clip=true, length={8,24}, seed=391})
work(rest * above(function(x) return 612 end), {hand="body", pile=L3b, tool="filbert 5", angle=-0.1, angle_jitter=0.15, coverage=5, fill=true, clip=true, length={8,24}, seed=392})
pglaze = pile{{"raw umber",2},{"smalt",1},{"bone black",0.15}, medium=0.6}
work(NPATH, {hand="glaze", tool={kind="filbert", width=10}, pile=pglaze, angle=-0.8, clip=true, coverage=1.5, fill=true, length={20,60},
  load_at=function(x,y) return clamp(0.25 + 0.5*(y-540)/170, 0.1, 0.8) end, seed=393})

--@ chunk 40
print(wait(96*60)); print(drying(500,650), drying(420,690), drying(560,690), drying(600,560))

--@ chunk 41
print(wait(120*60)); print(drying(420,690), drying(400,700), drying(450,705), drying(380,650))

--@ chunk 42
FGTOP = function(x) return 588 + 5*math.sin(x/130+0.5) + 2*math.sin(x/37) end
FG = below(FGTOP) - DITCH:grow(3)
FGA = FG * above(function(x) return 640 + 6*math.sin(x/90) end)
FGB = FG - FGA
work(FGA, {hand="body", pile=L3b, tool="filbert 6", angle=function(x,y) return -0.05 + 0.1*math.sin(x/80) end, angle_jitter=0.15, coverage=3.5, fill=true, length={15,40}, pressure={0.5,0.8}, seed=421})
work(FGB, {hand="body", pile=L4b, tool="filbert 7", angle=function(x,y) return -0.08 + 0.15*math.sin(x/70) end, angle_jitter=0.2, coverage=3.8, fill=true, length={15,45}, pressure={0.5,0.85}, seed=422})
pathp4 = pile{{"raw umber",1.5},{"green earth",0.8},{"lead white",0.65},{"pale smalt",0.6},{"yellow ochre",0.3}, medium=0.2}
FPATH, FPS = taperstrip({{455,720},{468,684},{497,650},{538,619},{580,592},{615,566},{642,545},{660,533}}, 44, 3, 423, 0.15)
FPATH = FPATH:roughen(1.2, 10, 424)
work(FPATH, {hand="body", pile=pathp4, tool="filbert 5", angle=-0.6, angle_jitter=0.25, coverage=3, fill=true, clip=true, length={10,28}, pressure={0.4,0.7}, seed=425})

--@ chunk 43
local old = (NPATH:grow(5) - FPATH) * above(FGTOP)
local a = old * above(function(x) return 556 end)
local b = old - a
work(a, {hand="body", pile=L2, tool="filbert 5", angle=0, angle_jitter=0.1, coverage=4.5, fill=true, clip=true, length={8,24}, seed=431})
work(b, {hand="body", pile=L3b, tool="filbert 5", angle=0, angle_jitter=0.1, coverage=4.5, fill=true, clip=true, length={8,24}, seed=432})

--@ chunk 44
moonp = pile{{"lead white",8},{"chrome yellow",0.35},{"yellow ochre",0.1}, medium=0.1}
local cx, cy, r = 742, 250, 7.5
-- sun is below the horizon at about (600, 480): the lit limb faces down-left
local ang = math.atan(480-cy, 600-cx)
local off = 3.4
MOON = ellipse(cx, cy, r, r) - ellipse(cx - off*math.cos(ang), cy - off*math.sin(ang), r*1.02, r*1.02):soften(0.4)
work(MOON, {hand="detail", pile=moonp, tool={kind="round", width=1.2, point=0.6}, angle=ang+math.pi/2, coverage=5, fill=true, length={2,5}, seed=441})
local sb = brush{kind="round", width=1.6, point=0.3}
sb:load(moonp, 0.6)
sb:touch(826, 296, {pressure=0.45})
print(ang)

--@ chunk 45
creamfix = pile{{"lead white",7},{"pale smalt",0.45},{"chrome yellow",0.15},{"yellow ochre",0.12}, medium=0.2}
work(ellipse(742,250,10,10), {hand="detail", pile=creamfix, tool={kind="round", width=2}, angle=0, coverage=4, fill=true, length={3,8}, seed=451})
blend(ellipse(742,250,13,13), {angle=0, tool={kind="badger", width=10}, length={6,14}, seed=452})
work(ellipse(826,296,4,4), {hand="detail", pile=creamfix, tool={kind="round", width=2}, angle=0, coverage=3, fill=true, length={3,6}, seed=453})

--@ chunk 46
print(wait(72*60)); print(drying(742,250), drying(600,560), drying(500,650), drying(300,520))

--@ chunk 47
veil = pile{{"lead white",6},{"pale smalt",0.9},{"chrome yellow",0.12},{"raw umber",0.08}, medium=0.4}
stipple(ellipse(742,250,17,17), {pile=veil, width=1.6, coverage=function(x,y) local d=math.sqrt((x-742)^2+(y-250)^2); return 1.6*math.exp(-(d/10)^2) end, feather=0.8, pressure={0.3,0.5}, seed=471})
stipple(ellipse(826,296,6,6), {pile=veil, width=1.4, coverage=function(x,y) local d=math.sqrt((x-826)^2+(y-296)^2); return 1.4*math.exp(-(d/4)^2) end, feather=0.8, pressure={0.3,0.5}, seed=472})

--@ chunk 48
moonw = pile{{"lead white",10},{"chrome yellow",0.2}, medium=0.02}
local cx, cy, r = 742, 250, 8.6
local ang = 2.12
MOON2 = ellipse(cx, cy, r, r) - ellipse(cx - 3.6*math.cos(ang), cy - 3.6*math.sin(ang), r*1.03, r*1.03)
work(MOON2, {hand="detail", pile=moonw, tool={kind="round", width=1.1, point=0.6}, angle=ang+math.pi/2, coverage=6, fill=true, length={2,5}, pressure={0.6,0.9}, seed=481})
local sb = brush{kind="round", width=1.4, point=0.4}
sb:load(moonw, 0.8)
sb:touch(826, 296, {pressure=0.7})
sb:touch(826.2, 296.1, {pressure=0.5})

--@ chunk 49
print(wait(96*60)); print(drying(600,560), drying(500,650), drying(470,700), drying(300,520))

--@ chunk 50
print(wait(96*60)); print(drying(600,560), drying(560,575), drying(620,545))

--@ chunk 51
M1 = pile{{"green earth",1.4},{"pale smalt",1.2},{"raw umber",0.8},{"yellow ochre",0.5},{"lead white",1.6}, medium=0.22}
M2 = pile{{"green earth",1.6},{"yellow ochre",1},{"raw umber",1},{"lead white",1},{"pale smalt",0.4}, medium=0.22}
M3 = pile{{"yellow ochre",1.3},{"lead white",1.3},{"raw umber",0.9},{"green earth",0.6},{"pale smalt",0.3},{"vermilion",0.03}, medium=0.22}
M3b = pile{{"yellow ochre",1},{"lead white",1},{"raw umber",1},{"green earth",0.8},{"pale smalt",0.4}, medium=0.22}
M4 = pile{{"green earth",2},{"yellow ochre",0.8},{"raw umber",1.3},{"Prussian blue",0.08},{"lead white",0.5}, medium=0.22}
M5 = pile{{"green earth",2},{"raw umber",1.6},{"yellow ochre",0.6},{"Prussian blue",0.12},{"lead white",0.25}, medium=0.2}
local ys = {495,503,512,524,538,555,574}
local piles = {M1,M2,M3,M3b,M4,M5,L3b}
local function edge(y0, k)
  local nz = noise{seed=510+k, period=160, octaves=2}
  return function(x) return y0 - (x-500)*0.004*(y0-470)/60 + 1.2*nz(x,y0) end
end
local excl = FPATH:grow(1) + DITCH:grow(3)
STRIPS = {}
for i=1,#ys do
  local top = edge(ys[i], i)
  local bot = (i < #ys) and edge(ys[i+1], i+1) or FGTOP
  local m = below(top) * above(bot) - excl
  STRIPS[i] = m
  work(m, {hand="body", pile=piles[i], tool="filbert 4", angle=0, angle_jitter=0.04, coverage=2.6, fill=true, clip=true, length={15,45}, pressure={0.4,0.7}, seed=520+i})
end

--@ chunk 52
trunkp = pile{{"raw umber",2},{"bone black",0.3},{"smalt",0.5},{"lead white",0.35}, medium=0.2}
fdark = pile{{"green earth",2},{"raw umber",1.3},{"smalt",0.6},{"bone black",0.12},{"lead white",0.35}, medium=0.2}
fmid = pile{{"green earth",1.5},{"pale smalt",1},{"lead white",1.2},{"raw umber",0.7}, medium=0.2}
flight = pile{{"lead white",2},{"yellow ochre",0.35},{"green earth",0.5},{"pale smalt",0.3},{"vermilion",0.03}, medium=0.2}
WILLOWS = {{58,626,232,1},{178,586,182,2},{262,558,146,3},{318,540,122,4}}
WINFO = {}
for _,w in ipairs(WILLOWS) do
  local x0,y0,s,id = w[1],w[2],w[3],w[4]
  math.randomseed(600+id)
  local lean = rand(-0.04,0.04)*s
  local th = s*0.36
  local hx, hy = x0+lean, y0-th
  local tw = s*0.085
  -- trunk: flared base, slight waist, swollen head
  local T = poly({{x0-tw*1.25,y0+2},{x0-tw*0.95,y0-th*0.2},{x0+lean*0.5-tw*0.8,y0-th*0.6},{hx-tw*1.1,y0-th*0.95},{hx-tw*1.4,hy-tw*0.3},
                  {hx-tw*0.6,hy-tw*0.8},{hx+tw*0.3,hy-tw*0.6},{hx+tw*1.3,hy-tw*0.5},{hx+tw*1.2,y0-th*0.92},{x0+lean*0.5+tw*0.85,y0-th*0.55},
                  {x0+tw*1.0,y0-th*0.2},{x0+tw*1.35,y0+2}}, true):roughen(s*0.006, s*0.05, 610+id)
  -- shoots and crown
  local C = nil
  local shoots = {}
  local n = math.floor(30 + s*0.12)
  for i=1,n do
    local sx = hx + rand(-1,1)*tw*1.1
    local sy = hy - tw*0.5 + rand(-0.2,0.2)*tw
    local dev = rand(-1,1)
    local a = -math.pi/2 + dev*1.05 + rand(-0.1,0.1)
    local L = s*0.64*rand(0.7,1.0)*(1 - 0.28*math.abs(dev))
    local bow = rand(-0.12,0.12) + dev*0.12
    local pts = {}
    for k=0,6 do
      local t = k/6
      local aa = a + bow*t
      local px = sx + math.cos(aa)*L*t
      local py = sy + math.sin(aa)*L*t
      pts[#pts+1] = {px, py}
    end
    shoots[#shoots+1] = {pts=pts, L=L}
    for k=3,6 do
      local p = pts[k+1]
      local r = s*rand(0.045,0.075)*(0.7+0.3*k/6)
      local e = ellipse(p[1], p[2], r*0.8, r)
      C = C and (C + e) or e
    end
  end
  C = C:roughen(s*0.012, s*0.04, 620+id)
  WINFO[id] = {T=T, C=C, hx=hx, hy=hy, s=s, shoots=shoots, x0=x0, y0=y0, tw=tw}
end

--@ chunk 53
for id=1,4 do
  local w = WINFO[id]
  local hx, hy = w.hx, w.hy
  local ang = function(x,y) return math.atan(y-(hy+w.s*0.05), x-hx) end
  work(w.C, {hand="hatch", pile=fdark, tool={kind="round", width=math.max(1.2, w.s*0.011), point=0.8}, angle=ang, angle_jitter=0.25, coverage=2.2, fill=true, length={w.s*0.03, w.s*0.07}, pressure={0.4,0.8}, edge={found=0.3, soft=0.5, lost=0.2, period=30}, seed=630+id})
end

--@ chunk 54
print(wait(60*60)); print(drying(100,450), drying(200,500), drying(300,520))

--@ chunk 55
print(wait(72*60)); print(drying(100,450), drying(200,500), drying(300,520), drying(60,420))

--@ chunk 56
GROVEALL = WINFO[1].C + WINFO[2].C + WINFO[3].C + WINFO[4].C
local gb = function(x) return 498 + 1.5*math.sin(x/23) end
local zone = rect(0,480,430,70) * below(gb)
local piles = {M1,M2,M3,M3b}
for i=1,4 do
  work(STRIPS[i] * zone, {hand="body", pile=piles[i], tool="filbert 4", angle=0, angle_jitter=0.04, coverage=3.2, fill=true, clip=true, length={12,35}, pressure={0.45,0.75}, seed=560+i})
end
-- also the far-meadow band between strips' top (495) and the grove bottom
local m0 = rect(0,480,430,20) * below(gb) * above(function(x) return 496 end)
work(m0, {hand="body", pile=M1, tool="filbert 3", angle=0, coverage=3, fill=true, clip=true, length={10,30}, seed=565})

--@ chunk 57
local D = DITCH:grow(3.5)
local ys = {495,503,512,524,538,555,574}
local piles = {M1,M2,M3,M3b,M4,M5,L3b}
for i=1,#ys do
  local y1 = (i<#ys) and ys[i+1] or 590
  work(D * rect(0,ys[i],1000,y1-ys[i]) * above(FGTOP), {hand="body", pile=piles[i], tool="filbert 4", angle=-0.3, angle_jitter=0.1, coverage=4, fill=true, clip=true, length={8,20}, seed=570+i})
end
work(D * below(FGTOP) * above(function(x) return 640 + 6*math.sin(x/90) end), {hand="body", pile=L3b, tool="filbert 4", angle=-0.3, coverage=4, fill=true, clip=true, length={8,20}, seed=578})
work(D * below(function(x) return 640 + 6*math.sin(x/90) end), {hand="body", pile=L4b, tool="filbert 4", angle=-0.3, coverage=4, fill=true, clip=true, length={8,20}, seed=579})

--@ chunk 58
print(wait(96*60)); print(drying(100,450), drying(200,500), drying(160,600), drying(60,420))

--@ chunk 59
gdeep = pile{{"green earth",2},{"raw umber",1.5},{"Prussian blue",0.15},{"bone black",0.1},{"smalt",0.3}, medium=0.3}
local inner = GROVEALL:shrink(4)
for id=1,4 do
  local w = WINFO[id]
  local hx, hy = w.hx, w.hy
  local ang = function(x,y) return math.atan(y-(hy+w.s*0.05), x-hx) end
  work(inner * w.C, {hand="hatch", pile=gdeep, tool={kind="round", width=math.max(1.2, w.s*0.01), point=0.8}, angle=ang, angle_jitter=0.3, coverage=1.6, fill=false, length={w.s*0.025, w.s*0.06}, pressure={0.4,0.8},
    load_at=function(x,y) return clamp(0.3 + (y-400)/150, 0.15, 0.9) end, seed=590+id})
end
-- trunks at the foot of the grove
local tb = brush{kind="round", width=2.4, point=0.5}
for i,t in ipairs({{28,3.2},{70,2.6},{112,3.4},{150,2.2},{196,3},{233,2.4},{275,2.8},{318,2.2},{352,1.8}}) do
  tb:load(gdeep, 0.6)
  local x = t[1]
  tb:stroke({{x, 499},{x+rand(-1,1), 490},{x+rand(-2,2), 481}}, {pressure={t[2]/3.4, 0.2}})
end

--@ chunk 60
print(wait(96*60)); print(drying(100,450), drying(60,515), drying(200,470))

--@ chunk 61
GB = function(x) return 498 + 1.5*math.sin(x/23) end
GROVE = GROVEALL * above(GB)
-- repaint strips below the grove foot where dark hatching spilled
local zone = rect(0,497,440,50) * below(GB)
local piles = {M1,M2,M3,M3b,M4}
for i=1,5 do
  work(STRIPS[i] * zone, {hand="body", pile=piles[i], tool="filbert 4", angle=0, angle_jitter=0.04, coverage=4, fill=true, clip=true, length={12,35}, pressure={0.5,0.8}, seed=610+i})
end
local m0 = rect(0,480,440,20) * below(GB) * above(function(x) return 496 end)
work(m0, {hand="body", pile=M1, tool="filbert 3", angle=0, coverage=4, fill=true, clip=true, length={10,30}, seed=616})
-- break the halo: feathery dark touches in the rim, except on the upper sky-facing edge
local rim = GROVE - GROVE:shrink(5)
local top = GROVE - GROVE:offset(0)  -- placeholder
local upper = GROVE:offset(0)
local sky_edge = mask(function(x,y) return 1 end)
stipple(rim * below(function(x) return 430 end), {pile=gdeep, tool={kind="round", width=1.6, point=0.7}, coverage=1.1, cluster=0.5, drag={2.5, -math.pi/2}, seed=617})
stipple(rim * above(function(x) return 430 end), {pile=gdeep, tool={kind="round", width=1.5, point=0.7}, coverage=0.55, cluster=0.6, drag={2.5, -math.pi/2}, seed=618})

--@ chunk 62
cockp = pile{{"raw umber",1.4},{"yellow ochre",1.2},{"green earth",0.5},{"lead white",0.5},{"pale smalt",0.3}, medium=0.2}
cocklit = pile{{"yellow ochre",1.2},{"lead white",1.8},{"raw umber",0.4},{"vermilion",0.05}, medium=0.2}
cockdk = pile{{"raw umber",2},{"green earth",0.8},{"bone black",0.15},{"smalt",0.3}, medium=0.2}
local function cockshape(x, y, h)
  local w = h*0.46
  return poly({{x-w,y},{x-w*0.98,y-h*0.3},{x-w*0.8,y-h*0.62},{x-w*0.45,y-h*0.88},{x,y-h},{x+w*0.45,y-h*0.88},{x+w*0.8,y-h*0.62},{x+w*0.98,y-h*0.3},{x+w,y}}, true)
end
COCKS = {}
local list = {{452,520,12},{508,520,12.5},{577,519.5,12},{693,519,12},{742,519,12},{803,519,11.5},{877,518.5,11.5},{943,518.5,11},
              {474,534.5,15},{563,534,15},{722,533.5,15},{832,533,14.5},{915,533,14.5}}
local all = nil
for i,c in ipairs(list) do
  local m = cockshape(c[1]+rand(-2,2), c[2], c[3]*rand(0.92,1.08)):roughen(0.5, 3, 620+i)
  COCKS[i] = {m=m, x=c[1], y=c[2], h=c[3]}
  all = all and (all + m) or m
end
ALLCOCKS = all
-- contact shadow under each
for i,c in ipairs(COCKS) do
  local sh = ellipse(c.x, c.y+0.5, c.h*0.62, c.h*0.09)
  work(sh, {hand="detail", pile=cockdk, tool={kind="round", width=1.2}, angle=0, coverage=3, fill=true, length={3,8}, seed=640+i})
end
work(ALLCOCKS, {hand="detail", pile=cockp, tool={kind="round", width=1.3, point=0.6}, angle=function(x,y) return -math.pi/2 end, angle_jitter=0.4, coverage=4, fill=true, length={2,6}, seed=660})

--@ chunk 63
for i,c in ipairs(COCKS) do
  local m = c.m
  local rimm = m - m:offset(0)  -- dummy to keep type
  -- light on the upper, sky-facing surface: the part of the cock above its own shape shifted down
  local sh = c.h*0.22
  local up = m * above(function(x) return c.y - c.h*0.55 end)
  local toprim = m - (m * below(function(x) return c.y - c.h*0.98 + 0.0*x end))
  -- a crescent along the top edge
  local lower_shape = ellipse(c.x - c.h*0.05, c.y - c.h*0.35, c.h*0.42, c.h*0.6)
  local crest = up - lower_shape
  work(crest, {hand="detail", pile=cocklit, tool={kind="round", width=1.0, point=0.7}, angle=0.3, angle_jitter=0.5, coverage=2.2, fill=false, length={2,4}, seed=670+i})
  -- shade on the lower part facing us
  local low = m * below(function(x) return c.y - c.h*0.35 end)
  work(low, {hand="detail", pile=cockdk, tool={kind="round", width=0.9, point=0.7}, angle=-math.pi/2, angle_jitter=0.3, coverage=0.9, fill=false, length={2,5}, seed=690+i})
end

--@ chunk 64
function horse(bx, base, H, dir, grazing, seed)
  local s = dir -- 1 faces right, -1 faces left
  local by = base - 0.66*H
  local m = ellipse(bx, by, 0.56*H, 0.2*H)
  m = m + ellipse(bx + s*0.34*H, by - 0.02*H, 0.2*H, 0.19*H) + ellipse(bx - s*0.36*H, by - 0.01*H, 0.22*H, 0.2*H)
  -- legs
  local lw = 0.075*H
  for _,lx in ipairs({-0.42, -0.3, 0.3, 0.4}) do
    local x = bx + s*lx*H
    m = m + ribbon({{x, by + 0.1*H},{x + rand(-0.03,0.03)*H, base - 0.18*H},{x + rand(-0.02,0.02)*H, base}}, {lw*1.3, lw*0.8, lw*0.7})
  end
  -- neck and head
  local nx, ny = bx + s*0.46*H, by - 0.08*H
  if grazing then
    local hx, hy = bx + s*0.9*H, base - 0.08*H
    m = m + ribbon({{nx, ny},{nx + s*0.2*H, by + 0.12*H},{hx, hy - 0.1*H}}, {0.26*H, 0.17*H, 0.12*H})
    m = m + ribbon({{hx, hy - 0.14*H},{hx + s*0.04*H, hy}}, {0.12*H, 0.08*H})
  else
    local hx, hy = bx + s*0.78*H, by - 0.52*H
    m = m + ribbon({{nx, ny},{nx + s*0.14*H, by - 0.3*H},{hx - s*0.02*H, hy}}, {0.28*H, 0.18*H, 0.13*H})
    m = m + ribbon({{hx - s*0.02*H, hy},{hx + s*0.2*H, hy + 0.14*H}}, {0.13*H, 0.08*H})
    m = m + ribbon({{hx - s*0.06*H, hy - 0.04*H},{hx - s*0.04*H, hy - 0.13*H}}, 0.04*H)  -- ear
  end
  -- tail
  local tx, ty = bx - s*0.56*H, by - 0.05*H
  m = m + ribbon({{tx, ty},{tx - s*0.07*H, ty + 0.2*H},{tx - s*0.05*H, ty + 0.46*H}}, {0.07*H, 0.06*H, 0.03*H})
  return m
end
bayp = pile{{"raw umber",1.8},{"red earth",0.7},{"bone black",0.3},{"smalt",0.2}, medium=0.2}
greyp = pile{{"lead white",2.2},{"pale smalt",0.8},{"raw umber",0.6},{"yellow ochre",0.2}, medium=0.2}
HORSEA = horse(800, 574, 24, -1, true, 1)
HORSEB = horse(852, 567, 22, 1, false, 2)
work(HORSEA, {hand="detail", pile=bayp, tool={kind="round", width=1.0, point=0.6}, angle=0, angle_jitter=0.4, coverage=4.5, fill=true, length={2,5}, seed=701})
work(HORSEB, {hand="detail", pile=greyp, tool={kind="round", width=1.0, point=0.6}, angle=0, angle_jitter=0.4, coverage=4.5, fill=true, length={2,5}, seed=702})

--@ chunk 65
woodp = pile{{"raw umber",1.5},{"lead white",0.6},{"pale smalt",0.5},{"bone black",0.12}, medium=0.2}
woodlit = pile{{"lead white",2},{"pale smalt",0.6},{"raw umber",0.5},{"yellow ochre",0.2}, medium=0.2}
local posts = {}
local x = 612
while x < 1010 do posts[#posts+1] = x; x = x + rand(34,42) end
local gx = {488, 526, 566}
FENCE = nil
local function add(m) FENCE = FENCE and (FENCE + m) or m end
local function fbase(x) return FGTOP(x) + 3 end
local function postm(x, h, lean)
  local b = fbase(x)
  return poly({{x-1.2,b},{x+1.2,b},{x+1.1+lean,b-h},{x-1.1+lean,b-h-0.6}})
end
for i,px in ipairs(posts) do add(postm(px, rand(19,22), rand(-1.2,1.2))) end
for i,px in ipairs(gx) do add(postm(px, rand(18,21), rand(-1.5,1.5))) end
-- rails right section
local function rails(xs)
  for r,hh in ipairs({17, 9}) do
    for i=1,#xs-1 do
      local x1, x2 = xs[i], xs[i+1]
      local y1, y2 = fbase(x1)-hh, fbase(x2)-hh
      local mid = {(x1+x2)/2, (y1+y2)/2 + rand(0.3,1.2)}
      add(ribbon({{x1,y1},mid,{x2,y2}}, 1.3))
    end
  end
end
rails(posts)
rails(gx)
-- the gate: open, swung back against the fence on the right
local gp = posts[1]
add(ribbon({{gp, fbase(gp)-17},{gp+30, fbase(gp+30)-16.5}}, 1.2))
add(ribbon({{gp, fbase(gp)-5},{gp+30, fbase(gp+30)-5}}, 1.2))
add(ribbon({{gp, fbase(gp)-17},{gp+30, fbase(gp+30)-5}}, 1.0))
add(ribbon({{gp+30, fbase(gp+30)-17},{gp+30, fbase(gp+30)-4}}, 1.0))
work(FENCE, {hand="detail", pile=woodp, tool={kind="round", width=1.0, point=0.6}, angle=math.pi/2, angle_jitter=0.2, coverage=4, fill=true, length={2,5}, seed=711})
FPOSTS = posts

--@ chunk 66
print(wait(96*60)); print(drying(852,555), drying(700,585), drying(577,515), drying(474,534))

--@ chunk 67
gdk = pile{{"raw umber",2},{"green earth",2},{"Prussian blue",0.25},{"bone black",0.2}, medium=0.2}
gmd = pile{{"green earth",2},{"yellow ochre",0.9},{"raw umber",1.2},{"Prussian blue",0.08},{"lead white",0.3}, medium=0.2}
glt = pile{{"green earth",1.5},{"pale smalt",0.8},{"lead white",1.3},{"yellow ochre",0.4},{"raw umber",0.3}, medium=0.2}
gstraw = pile{{"yellow ochre",1.2},{"raw umber",0.9},{"lead white",0.8},{"green earth",0.4}, medium=0.2}
function blades(m, n, p, opts)
  local b = brush{kind="round", width=opts.w or 1.4, point=1}
  local cnt = 0
  local tries = 0
  while cnt < n and tries < n*20 do
    tries = tries + 1
    local x = rand(opts.x0 or 0, opts.x1 or 1000)
    local y = rand(opts.y0, opts.y1)
    if m:at(x,y) > 0.5 then
      cnt = cnt + 1
      if cnt % (opts.per or 6) == 1 then b:load(p, opts.load or 0.5) end
      local t = (y - opts.y0)/(opts.y1 - opts.y0)
      local L = lerp(opts.L0, opts.L1, t) * rand(0.6, 1.3)
      local a = -math.pi/2 + (opts.lean or 0.12) + randn(0, opts.spread or 0.25)
      local bend = randn(0, 0.15)
      local wid = lerp(opts.W0, opts.W1, t) * rand(0.7,1.2)
      local pts = {}
      for k=0,4 do local tt=k/4; local aa = a + bend*tt; pts[#pts+1] = {x + math.cos(aa)*L*tt, y + math.sin(aa)*L*tt} end
      b:stroke(pts, {pressure={b:pressure_for(wid), 0}, ramps={0.05, 0.6}})
    end
  end
  return cnt
end
local FGM = below(function(x) return FGTOP(x) + 6 end)
print(blades(FGM, 1400, gdk, {y0=596, y1=716, L0=4, L1=20, W0=0.6, W1=1.8, lean=0.1}))
print(blades(FGM, 900, gmd, {y0=596, y1=716, L0=4, L1=18, W0=0.5, W1=1.5, lean=0.15}))

--@ chunk 68
local FGM = below(function(x) return FGTOP(x) + 6 end) - FPATH:shrink(3)
print(blades(FGM, 2600, gdk, {y0=596, y1=716, L0=3, L1=16, W0=0.45, W1=1.3, lean=0.08, spread=0.3, per=8}))
print(blades(FGM, 500, glt, {y0=610, y1=716, L0=4, L1=18, W0=0.4, W1=1.1, lean=0.18, spread=0.2, load=0.35}))

--@ chunk 69
print(wait(96*60)); print(drying(510,630), drying(852,555), drying(577,515), drying(600,700))

--@ chunk 70
coatp = pile{{"smalt",1.2},{"bone black",0.8},{"raw umber",1},{"lead white",0.15}, medium=0.15}
hatp = pile{{"bone black",1.2},{"raw umber",0.6},{"smalt",0.3}, medium=0.15}
dressp = pile{{"red earth",1.2},{"raw umber",1},{"vermilion",0.15},{"bone black",0.2}, medium=0.15}
shawlp = pile{{"lead white",1.2},{"pale smalt",0.8},{"raw umber",0.7},{"yellow ochre",0.2}, medium=0.15}
skinp = pile{{"lead white",1},{"red earth",0.35},{"yellow ochre",0.3},{"raw umber",0.4}, medium=0.15}
bonnetp = pile{{"lead white",1.5},{"yellow ochre",0.4},{"raw umber",0.5},{"pale smalt",0.2}, medium=0.15}
-- man
local mx, mb = 503, 641
MAN_COAT = poly({{499.6,606.2},{501.5,605.3},{504.5,605.3},{506.4,606.2},{507.6,608.5},{508.3,616},{509.2,627},{509.6,633.5},{496.6,633.5},{497.1,627},{497.8,616},{498.4,608.5}}, true)
MAN_LEGS = ribbon({{501.2,633},{501.0,640.5}}, 2.0) + ribbon({{505.0,633},{505.3,640.5}}, 2.0) + ellipse(500.8,641,1.6,0.8) + ellipse(505.6,641,1.6,0.8)
MAN_HEAD = ellipse(503, 603.2, 2.5, 2.8)
MAN_HAT = rect(500.5, 596.3, 5.0, 5.2) + ellipse(503, 601.3, 3.9, 0.8)
MAN_COLLAR = ellipse(503, 606.2, 3.4, 1.2)
-- woman
WOM_DRESS = poly({{515.3,611},{522.7,611},{523.9,620},{525.2,631},{526.4,641.6},{511.6,641.6},{512.8,631},{514.1,620}}, true)
WOM_SHAWL = poly({{514.3,609.5},{516.5,608.4},{521.5,608.4},{523.7,609.5},{524.6,613},{523.8,619.5},{519,623.5},{514.2,619.5},{513.4,613}}, true)
WOM_HEAD = ellipse(519, 605.6, 2.3, 2.6)
WOM_BONNET = ellipse(519, 604.6, 3.1, 3.3)
work(MAN_LEGS, {hand="detail", pile=hatp, tool={kind="round", width=0.8, point=0.6}, angle=math.pi/2, coverage=5, fill=true, length={2,4}, seed=721})
work(MAN_COAT, {hand="detail", pile=coatp, tool={kind="round", width=0.9, point=0.6}, angle=math.pi/2, angle_jitter=0.1, coverage=5, fill=true, length={2,6}, seed=722})
work(MAN_HEAD, {hand="detail", pile=skinp, tool={kind="round", width=0.8, point=0.6}, angle=math.pi/2, coverage=4, fill=true, length={1,3}, seed=723})
work(MAN_HAT + MAN_COLLAR, {hand="detail", pile=hatp, tool={kind="round", width=0.8, point=0.6}, angle=0, coverage=5, fill=true, length={1,4}, seed=724})
work(WOM_DRESS, {hand="detail", pile=dressp, tool={kind="round", width=0.9, point=0.6}, angle=math.pi/2, angle_jitter=0.1, coverage=5, fill=true, length={2,6}, seed=725})
work(WOM_SHAWL, {hand="detail", pile=shawlp, tool={kind="round", width=0.8, point=0.6}, angle=math.pi/2, coverage=5, fill=true, length={2,5}, seed=726})
work(WOM_BONNET, {hand="detail", pile=bonnetp, tool={kind="round", width=0.8, point=0.6}, angle=0.4, coverage=5, fill=true, length={1,3}, seed=727})

--@ chunk 71
birdp = pile{{"raw umber",1.2},{"bone black",0.5},{"smalt",0.5},{"lead white",0.6}, medium=0.2}
local bb = brush{kind="round", width=0.9, point=1}
local flock = {{262,262,5.2},{276,256,4.6},{291,266,4.8},{247,270,4.2},{305,259,4.0},{318,268,3.6},{233,279,3.4},{284,276,3.8}}
for i,f in ipairs(flock) do
  bb:load(birdp, 0.5)
  local x,y,s = f[1], f[2], f[3]
  local up = rand(-0.2,0.35)
  -- left wing, body, right wing, as two shallow arcs meeting at the body
  bb:stroke({{x - s, y - s*0.25*up - s*0.1},{x - s*0.5, y - s*0.32},{x, y}}, {pressure={bb:pressure_for(0.35), bb:pressure_for(0.9)}})
  bb:stroke({{x, y},{x + s*0.5, y - s*0.3},{x + s*0.95, y - s*0.25*up - s*0.12}}, {pressure={bb:pressure_for(0.9), bb:pressure_for(0.3)}})
  bb:touch(x, y+0.2, {pressure=bb:pressure_for(1.1)})
end

--@ chunk 72
print(wait(96*60)); print(drying(519,630), drying(503,620), drying(519,605), drying(270,262))

--@ chunk 73
hglz = pile{{"pale smalt",1},{"raw umber",1.1},{"lead white",0.3}, medium=0.55}
work(HORSEB * below(function(x) return 567 - 22*0.62 end), {hand="detail", pile=hglz, tool={kind="round", width=0.9, point=0.6}, angle=0, coverage=3, fill=true, length={2,4}, seed=731})
work(HORSEB, {hand="detail", pile=hglz, tool={kind="round", width=0.9, point=0.6}, angle=0.3, coverage=1.5, fill=false, length={2,4}, seed=732})
-- windrows and stubble over the mown strips
stub1 = pile{{"yellow ochre",1},{"green earth",1.2},{"raw umber",1.1},{"lead white",0.6},{"pale smalt",0.4}, medium=0.22}
stub2 = pile{{"raw umber",1.3},{"green earth",1},{"yellow ochre",0.5},{"pale smalt",0.4},{"lead white",0.3}, medium=0.22}
local M = (STRIPS[3] + STRIPS[4]) - ALLCOCKS:grow(1.5)
work(M, {hand="hatch", pile=stub1, tool={kind="round", width=1.3, point=0.5}, angle=0, angle_jitter=0.05, coverage=1.0, fill=false, length={5,16}, pressure={0.3,0.6}, clump=0.5, seed=733})
work(M, {hand="hatch", pile=stub2, tool={kind="round", width=0.9, point=0.5}, angle=0, angle_jitter=0.05, coverage=0.5, fill=false, length={4,12}, pressure={0.3,0.6}, clump=0.6, seed=734})
-- haycock shade glaze
cglz = pile{{"raw umber",1.2},{"green earth",0.6},{"pale smalt",0.4}, medium=0.55}
for i,c in ipairs(COCKS) do
  work(c.m * below(function(x) return c.y - c.h*0.5 end), {hand="detail", pile=cglz, tool={kind="round", width=0.9, point=0.6}, angle=-math.pi/2, coverage=2, fill=false, length={2,4}, seed=740+i})
end

--@ chunk 74
for i,c in ipairs(COCKS) do
  blend(c.m, {angle=-math.pi/2, tool={kind="badger", width=math.max(3, c.h*0.35)}, length={c.h*0.4, c.h*0.8}, seed=760+i})
end

--@ chunk 75
print(wait(96*60)); print(drying(519,630), drying(503,620), drying(500,520), drying(560,530))

--@ chunk 76
mist2 = pile{{"lead white",5},{"pale smalt",1.1},{"vermilion",0.05},{"yellow ochre",0.15}, medium=0.45}
local nz = noise{seed=761, period=90, octaves=3}
local band = rect(-10,462,470,44)
work(band, {hand="scumble", pile=mist2, tool={kind="filbert", width=7}, angle=0, angle_jitter=0.08, coverage=1.6, fill=false, length={20,50}, pressure={0.25,0.45},
  load_at=function(x,y) return clamp(0.35*smoothstep(466, 494, y)*(1-smoothstep(499, 506, y))*(0.8+0.4*nz(x,y)), 0.0, 0.5) end, seed=762})
blend(band, {angle=0, length={40,120}, tool={kind="badger", width=20}, seed=763})

--@ chunk 77
local band = rect(-10,455,480,56)
for i=1,4 do
  blend(band, {angle=0, length={30,90}, tool={kind="badger", width=16}, seed=770+i})
end

--@ chunk 78
local seam = rect(0,448,470,22) * below(function(x) return 449 + 4*math.sin(x/17) end)
blend(seam, {angle=-math.pi/2, angle_jitter=0.35, length={6,16}, clip=false, tool={kind="badger", width=10}, seed=781})
blend(seam, {angle=-math.pi/2+0.4, angle_jitter=0.35, length={6,14}, clip=false, tool={kind="badger", width=8}, seed=782})

--@ chunk 79
local e = rect(440,455,70,56)
for i=1,3 do blend(e, {angle=0, angle_jitter=0.2, length={15,45}, clip=false, tool={kind="badger", width=12}, seed=790+i}) end

--@ chunk 80
print(wait(96*60)); print(drying(200,480), drying(600,650), drying(300,690))

--@ chunk 81
local FGM = below(function(x) return FGTOP(x) + 10 end) - FPATH:grow(2) - (MAN_COAT+WOM_DRESS+WOM_SHAWL):grow(4)
stemp = pile{{"green earth",1.5},{"raw umber",1.2},{"yellow ochre",0.3}, medium=0.2}
umbp = pile{{"lead white",2},{"yellow ochre",0.3},{"pale smalt",0.3},{"raw umber",0.3}, medium=0.2}
champ = pile{{"lead white",2.5},{"pale smalt",0.2},{"raw umber",0.15}, medium=0.2}
sorrp = pile{{"red earth",1.5},{"vermilion",0.3},{"raw umber",0.8}, medium=0.2}
yelp = pile{{"yellow ochre",1.5},{"lead white",0.8},{"raw umber",0.2}, medium=0.2}
local st = brush{kind="round", width=1, point=1}
local dt = brush{kind="round", width=1.2, point=0.5}
local function pick(y0,y1)
  for k=1,60 do local x,y = rand(5,995), rand(y0,y1); if FGM:at(x,y)>0.5 then return x,y end end
end
local function stem(x,y,L,a)
  local tx, ty = x + math.cos(a)*L, y + math.sin(a)*L
  st:stroke({{x,y},{(x+tx)/2 + randn(0,0.6),(y+ty)/2},{tx,ty}}, {pressure={st:pressure_for(math.max(0.5,L/25)), st:pressure_for(0.35)}})
  return tx, ty
end
-- cow parsley umbels, larger nearer
for i=1,26 do
  local x,y = pick(612,712)
  if x then
    local t = (y-600)/110
    local L = lerp(10,34,t)*rand(0.8,1.2)
    st:load(stemp,0.5)
    local tx,ty = stem(x,y,L,-math.pi/2+randn(0,0.12))
    local r = lerp(2,6,t)
    dt:load(umbp,0.6)
    for k=1,math.floor(lerp(5,12,t)) do
      local a = rand(math.pi, 2*math.pi)
      dt:touch(tx + math.cos(a)*r*rand(0.2,1), ty + math.sin(a)*r*0.45*rand(0.2,1) - r*0.1, {pressure=dt:pressure_for(lerp(0.7,1.5,t))})
    end
  end
end
-- chamomile / daisies: small white dots with a yellow eye
for i=1,70 do
  local x,y = pick(605,712)
  if x then
    local t = (y-600)/110
    st:load(stemp,0.4)
    local tx,ty = stem(x,y,lerp(4,14,t)*rand(0.7,1.3),-math.pi/2+randn(0,0.2))
    dt:load(champ,0.5)
    dt:touch(tx,ty,{pressure=dt:pressure_for(lerp(0.8,2.0,t))})
    if t > 0.4 then dt:load(yelp,0.4); dt:touch(tx,ty,{pressure=dt:pressure_for(lerp(0.3,0.8,t))}) end
  end
end
-- sorrel: rusty red spikes
for i=1,40 do
  local x,y = pick(600,712)
  if x then
    local t = (y-600)/110
    local L = lerp(8,26,t)*rand(0.8,1.2)
    st:load(stemp,0.4)
    local tx,ty = stem(x,y,L,-math.pi/2+randn(0,0.1))
    dt:load(sorrp,0.5)
    for k=0,math.floor(lerp(3,7,t)) do
      local f = k/7
      dt:touch(lerp(x,tx,0.45+f*0.55)+randn(0,0.5), lerp(y,ty,0.45+f*0.55), {pressure=dt:pressure_for(lerp(0.6,1.3,t))})
    end
  end
end
-- buttercups
for i=1,40 do
  local x,y = pick(605,712)
  if x then
    local t = (y-600)/110
    st:load(stemp,0.4)
    local tx,ty = stem(x,y,lerp(4,16,t)*rand(0.7,1.3),-math.pi/2+randn(0,0.2))
    dt:load(yelp,0.5); dt:touch(tx,ty,{pressure=dt:pressure_for(lerp(0.6,1.6,t))})
  end
end

--@ chunk 82
leafp = pile{{"green earth",1.5},{"raw umber",1.6},{"Prussian blue",0.2},{"bone black",0.15}, medium=0.2}
leaflt = pile{{"green earth",1.5},{"yellow ochre",0.6},{"lead white",0.6},{"raw umber",0.6}, medium=0.2}
thisp = pile{{"red earth",0.7},{"smalt",0.7},{"lead white",0.8},{"raw umber",0.4},{"vermilion",0.1}, medium=0.2}
local function leaf(x,y,L,a,w)
  local tx,ty = x+math.cos(a)*L, y+math.sin(a)*L
  local nx,ny = -math.sin(a), math.cos(a)
  local mx,my = x+math.cos(a)*L*0.45, y+math.sin(a)*L*0.45
  return poly({{x,y},{mx+nx*w,my+ny*w},{tx,ty},{mx-nx*w,my-ny*w}}, true)
end
-- dock clump bottom right
local D = nil
local base = {{930,716},{955,716},{905,716}}
local specs = {{930,716,46,-1.9,9},{934,716,40,-1.2,8},{926,716,34,-2.5,7},{955,716,30,-1.5,6},{905,716,28,-2.2,6},{945,716,24,-0.8,5}}
for _,s in ipairs(specs) do local l = leaf(s[1],s[2],s[3],s[4],s[5]); D = D and D + l or l end
work(D, {hand="detail", pile=leafp, tool={kind="round", width=1.4, point=0.6}, angle=-1.5, angle_jitter=0.3, coverage=4, fill=true, length={3,8}, seed=821})
local st = brush{kind="round", width=1, point=1}
st:load(leaflt, 0.4)
for _,s in ipairs(specs) do
  local tx,ty = s[1]+math.cos(s[4])*s[3]*0.9, s[2]+math.sin(s[4])*s[3]*0.9
  st:stroke({{s[1],s[2]},{tx,ty}}, {pressure={st:pressure_for(1.1), st:pressure_for(0.2)}})
end
-- a tall dock seed spike, rusty
local dk = brush{kind="round", width=1.2, point=0.6}
st:load(leafp,0.5); st:stroke({{940,700},{942,660},{946,628}}, {pressure={st:pressure_for(1.6), st:pressure_for(0.5)}})
dk:load(sorrp,0.6)
for k=0,14 do local f=k/14; dk:touch(lerp(942,946,f)+randn(0,1.2), lerp(662,628,f), {pressure=dk:pressure_for(lerp(2.2,1.0,f))}) end
-- thistle bottom left
local tx0, ty0 = 70, 716
st:load(leafp,0.6); st:stroke({{tx0,ty0},{72,680},{76,646}}, {pressure={st:pressure_for(2.2), st:pressure_for(1.2)}})
st:load(leafp,0.5); st:stroke({{72,690},{64,676},{56,668}}, {pressure={st:pressure_for(1.5), st:pressure_for(0.3)}})
st:stroke({{73,672},{80,662},{89,656}}, {pressure={st:pressure_for(1.5), st:pressure_for(0.3)}})
st:stroke({{74,660},{70,652},{64,646}}, {pressure={st:pressure_for(1.2), st:pressure_for(0.3)}})
-- spiny leaf blades
local TL = leaf(71,700,34,-2.7,5) + leaf(73,700,32,-0.45,5) + leaf(72,682,22,-2.4,4) + leaf(74,680,22,-0.6,4)
work(TL, {hand="detail", pile=leafp, tool={kind="round", width=1.2, point=0.6}, angle=0, angle_jitter=0.5, coverage=4, fill=true, length={2,6}, seed=822})
-- heads: dark bulb + mauve tuft
for _,h in ipairs({{76,644,4},{56,666,3},{89,654,3.2},{64,644,2.6}}) do
  work(ellipse(h[1],h[2]+h[3]*0.3,h[3]*0.8,h[3]*0.8), {hand="detail", pile=leafp, tool={kind="round", width=0.9, point=0.6}, coverage=4, fill=true, length={1,3}, seed=823})
  dk:load(thisp,0.6)
  for k=1,9 do local a = rand(-math.pi*0.9,-math.pi*0.1); dk:stroke({{h[1],h[2]-h[3]*0.2},{h[1]+math.cos(a)*h[3]*1.1, h[2]-h[3]*0.2+math.sin(a)*h[3]*1.2}}, {pressure={dk:pressure_for(0.9), dk:pressure_for(0.3)}}) end
end

--@ chunk 83
local R = rect(20,640,110,76) + rect(880,660,110,56)
blades(R, 120, gmd, {y0=650, y1=716, L0=10, L1=22, W0=0.8, W1=1.5, lean=0.1, spread=0.3})
blades(R, 50, glt, {y0=650, y1=716, L0=10, L1=22, W0=0.5, W1=1.1, lean=0.15, spread=0.25, load=0.35})
local st = brush{kind="round", width=0.8, point=1}
st:load(leaflt, 0.35)
for _,e in ipairs({{{930,716},{918,690},{913,675}}, {{934,716},{944,690},{948,680}}, {{73,700},{60,695},{42,688}}, {{73,700},{90,691},{104,686}}, {{926,716},{910,700},{902,694}}}) do
  st:stroke(e, {pressure={st:pressure_for(0.8), st:pressure_for(0.2)}})
end

--@ chunk 84
print(drying(500,540), drying(700,556), drying(800,590))

--@ chunk 85
bglz = pile{{"green earth",1.3},{"yellow ochre",0.6},{"raw umber",0.8},{"lead white",0.3}, medium=0.6}
bglz2 = pile{{"green earth",1.2},{"raw umber",1.2},{"pale smalt",0.3}, medium=0.6}
local nz = noise{seed=851, period=60, octaves=2}
local function band(yc, h, p, seed, keep)
  local m = rect(-5, yc-h, 1010, 2*h) - keep
  work(m, {hand="scumble", pile=p, tool={kind="filbert", width=5}, angle=0, angle_jitter=0.1, coverage=1.3, fill=false, length={15,40}, pressure={0.25,0.45},
    load_at=function(x,y) local d = math.abs(y-yc-3*nz(x,y))/h; return clamp(0.4*(1-d*d), 0, 0.4) end, seed=seed})
end
local keep = (FPATH + ALLCOCKS + FENCE + HORSEA + HORSEB):grow(1.5)
band(538, 6, bglz, 852, keep)
band(555, 5, bglz2, 853, keep)
band(512, 4, bglz, 854, keep)

--@ chunk 86
local keep = (FPATH + ALLCOCKS + FENCE + HORSEA + HORSEB):grow(2)
for i,b in ipairs({{538,8},{555,8},{512,6}}) do
  local m = rect(-5, b[1]-b[2], 1010, 2*b[2]) - keep
  for k=1,3 do blend(m, {angle=0, angle_jitter=0.05, length={20,60}, tool={kind="badger", width=8}, seed=860+i*10+k}) end
end

--@ chunk 87
print(wait(120*60)); print(drying(852,555), drying(625,552), drying(700,555))

--@ chunk 88
greyp2 = pile{{"lead white",1.6},{"pale smalt",0.9},{"raw umber",0.9},{"yellow ochre",0.15}, medium=0.2}
greydk = pile{{"lead white",0.8},{"pale smalt",0.8},{"raw umber",1.2},{"bone black",0.1}, medium=0.2}
work(HORSEB, {hand="detail", pile=greyp2, tool={kind="round", width=0.9, point=0.6}, angle=0, angle_jitter=0.3, coverage=5, fill=true, length={2,4}, seed=881})
work(HORSEB * below(function(x) return 567 - 22*0.55 end), {hand="detail", pile=greydk, tool={kind="round", width=0.8, point=0.6}, angle=math.pi/2, coverage=3, fill=true, length={2,4}, seed=882})
-- path smudge
local sm = FPATH * rect(605,540,40,20)
work(sm, {hand="detail", pile=pathp4, tool={kind="round", width=1.2, point=0.6}, angle=-0.7, angle_jitter=0.2, coverage=4, fill=true, length={2,5}, seed=883})

--@ chunk 89
print(wait(96*60)); print(drying(852,555))

--@ chunk 90
hglz2 = pile{{"pale smalt",1},{"raw umber",1.3},{"red earth",0.1}, medium=0.7}
work(HORSEB:shrink(0.3), {hand="detail", pile=hglz2, tool={kind="round", width=1.2, point=0.6}, angle=0, angle_jitter=0.05, coverage=3, fill=true, length={3,6}, pressure={0.4,0.6}, seed=901})

--@ chunk 91
tglz = pile{{"smalt",1},{"raw umber",1.2},{"bone black",0.3}, medium=0.65}
local nz = noise{seed=911, period=40, octaves=2}
work(TOWN2, {hand="scumble", pile=tglz, tool={kind="filbert", width=4}, angle=0, angle_jitter=0.05, coverage=1.6, fill=false, length={8,20}, pressure={0.3,0.5},
  load_at=function(x,y) return clamp(0.45*smoothstep(400, 440, y), 0, 0.45) end, seed=912})

--@ chunk 92
for k=1,4 do blend(TOWN2, {angle=(k%2==0) and 0 or math.pi/2, angle_jitter=0.1, length={6,18}, tool={kind="badger", width=6}, seed=920+k}) end

--@ chunk 93
print(wait(144*60)); print(drying(500,430), drying(430,407), drying(650,420))

--@ chunk 94
skyapr = pile{{"lead white",7},{"chrome yellow",0.3},{"yellow ochre",0.35},{"vermilion",0.12}, medium=0.25}
RING = (TOWN2:grow(5) - TOWN2) * rect(330,330,340,116)
work(RING * rect(420,395,40,20), {hand="detail", pile=skyapr, tool={kind="round", width=1.2, point=0.6}, angle=0, coverage=4, fill=true, length={2,5}, seed=941})

--@ chunk 95
RING = (TOWN2:grow(8) - TOWN2:grow(0.3) - TTREES - MILLB) * rect(320,320,380,122)
work(RING, {hand="detail", pile=skyapr, tool={kind="round", width=1.2, point=0.6}, angle=0, coverage=4, fill=true, length={2,5}, seed=951})

--@ chunk 96
local R = (TOWN2:grow(12) - TOWN2:grow(1.5) - TTREES - MILLB) * rect(320,320,380,122)
for k=1,3 do blend(R, {angle=k*1.1, angle_jitter=0.6, length={4,10}, clip=false, tool={kind="badger", width=5}, seed=960+k}) end

--@ chunk 97
print(wait(144*60)); print(drying(395,380), drying(470,350), drying(420,405), drying(600,430))

--@ chunk 98
work(TOWN2, {hand="detail", pile=town3, tool={kind="round", width=1.0, point=0.6}, angle=math.pi/2, angle_jitter=0.08, coverage=5, fill=true, length={2,6}, seed=981})

--@ chunk 99
print(wait(144*60)); print(drying(395,380), drying(600,430))

--@ chunk 100
local keep = TOWN2:grow(1.2) + TTREES:grow(1.5) + MILLB:grow(1.5)
local P = (rect(400,400,60,14) + rect(636,408,32,30) + rect(330,405,40,40)) * above(function(x) return 444 end) - keep
work(P, {hand="detail", pile=skyapr, tool={kind="round", width=1.0, point=0.6}, angle=0, coverage=4, fill=true, length={2,4}, seed=1001})

--@ chunk 101
local keep = TOWN2:grow(3) + TTREES:grow(2) + MILLB:grow(2)
local P = (rect(400,400,60,14) + rect(636,408,32,30) + rect(330,405,40,40))
local E = (P:grow(6) - P:shrink(4)) * above(function(x) return 443 end) - keep
for k=1,3 do blend(E, {angle=k*0.9, angle_jitter=0.8, length={4,12}, clip=true, tool={kind="badger", width=6}, seed=1010+k}) end

--@ chunk 102
local nz = noise{seed=1021, period=6, octaves=2}
local C = ellipse(338,430,11,17) + ellipse(352,432,9,13) + ellipse(365,437,8,9) + ellipse(328,428,8,14)
C = C - TOWN2:grow(0.5)
work(C, {hand="detail", pile=gdeep, tool={kind="round", width=1.2, point=0.6}, angle=-1.2, angle_jitter=0.9, coverage=4, fill=true, length={2,5}, seed=1022})

--@ chunk 103
print(wait(144*60)); print(drying(345,430), drying(330,425))

--@ chunk 104
print(wait(240*60)); print(drying(345,430), drying(330,425), drying(365,437))

--@ chunk 105
local crowns = {{334,426,12,17},{350,431,10,13},{364,436,8,9},{322,424,7,12}}
local C = nil
for _,c in ipairs(crowns) do
  for k=1,55 do
    local a = rand(0, 2*math.pi); local r = math.sqrt(rand(0,1))
    local x = c[1] + math.cos(a)*r*c[3]; local y = c[2] + math.sin(a)*r*c[4]
    local e = ellipse(x, y, rand(1.5,3.2), rand(1.2,2.6))
    C = C and C + e or e
  end
end
C = C * above(function(x) return 446 end) - TOWN2:grow(0.4)
work(C, {hand="detail", pile=gdeep, tool={kind="round", width=1.1, point=0.6}, angle=-1.2, angle_jitter=1.0, coverage=4, fill=true, length={1.5,4}, seed=1051})

--@ chunk 106
print(wait(240*60)); print(drying(345,430), drying(330,425), drying(365,437))

--@ chunk 107
local R = (ellipse(338,430,20,18) + ellipse(362,436,10,10)) * above(function(x) return 446 end) - TOWN2:grow(1)
local b = brush{kind="round", width=1, point=0.8}
local n = 0
for k=1,400 do
  local x,y = rand(315,375), rand(408,446)
  if R:at(x,y) > 0.5 and n < 90 then
    n = n + 1
    if n % 8 == 1 then b:load((n%3==0) and fmid or treep, 0.4) end
    local a = -1.1 + randn(0,0.5); local L = rand(1.5,3.5)
    b:stroke({{x,y},{x+math.cos(a)*L, y+math.sin(a)*L}}, {pressure={b:pressure_for(0.9), b:pressure_for(0.3)}})
  end
end

--@ chunk 108
print(wait(240*60)); for _,p in ipairs({{345,430},{500,430},{852,555},{519,630},{930,700},{70,690}}) do print(drying(p[1],p[2])) end
