-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=700, aspect=1.4, linen={15,13}, seed=1822,
 ground={{pile={{"red earth",2},{"yellow ochre",3},{"lead white",1}}, um=80, apply="knife", texture=0.35},
         {pile={{"lead white",10},{"yellow ochre",0.4},{"raw umber",0.15}}, um=45, apply="roller"}}}
print(W,H)

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
erase(everywhere(), {strength=1})

--@ chunk 4
-- underdrawing in pencil
HZ = 438
h = pencil("HB")
h2 = pencil("2H")
-- ruled horizon, lightly
h2:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
-- far land line with slight undulation
h2:sketch({{0,432},{120,430},{260,433},{420,431},{520,434},{640,436},{800,435},{920,433},{1000,434}}, {pressure=0.3})
-- town: main church
h:line({{596,436},{596,365},{614,365},{614,436}}, {pressure=0.45, smooth=false})
h:line({{596,365},{605,320},{614,365}}, {pressure=0.45, smooth=false})
h:line({{614,415},{614,400},{690,400},{700,415},{700,436}}, {pressure=0.4, smooth=false})
-- second church
h:line({{720,436},{720,380},{738,380},{738,436}}, {pressure=0.4, smooth=false})
h:line({{720,380},{729,368},{738,380}}, {pressure=0.4, smooth=false})
h:line({{738,418},{746,407},{788,407},{794,418}}, {pressure=0.35, smooth=false})
-- third church
h:line({{540,436},{540,395},{551,395},{551,436}}, {pressure=0.4, smooth=false})
h:line({{540,395},{545.5,372},{551,395}}, {pressure=0.4, smooth=false})
-- roofs
h2:sketch({{510,436},{515,426},{530,426},{536,430},{560,428},{570,422},{590,422},{596,426}}, {pressure=0.3})
h2:sketch({{700,426},{712,424},{720,428},{738,426},{800,424},{808,430},{825,432},{840,436}}, {pressure=0.3})
-- windmills
h:line({{178,432},{178,420},{186,420},{186,432}}, {pressure=0.35, smooth=false})
h:line({{170,410},{194,430}}, {pressure=0.3})
h:line({{194,410},{170,430}}, {pressure=0.3})
h:line({{877,433},{877,423},{884,423},{884,433}}, {pressure=0.35, smooth=false})
h:line({{870,414},{891,432}}, {pressure=0.3})
h:line({{891,414},{870,432}}, {pressure=0.3})
-- river
h2:sketch({{1000,528},{900,515},{780,500},{680,482},{600,468},{520,458},{440,450},{380,446},{330,443}}, {pressure=0.3})
h2:sketch({{1000,548},{890,532},{770,514},{670,494},{590,477},{510,464},{440,454},{380,448},{330,444}}, {pressure=0.3})
-- foreground rise
h:sketch({{0,578},{80,566},{180,558},{280,564},{380,582},{460,600},{540,622},{620,640}}, {pressure=0.35})
-- oak trunk and crown outline
h:line({{232,604},{238,540},{240,470},{236,420}}, {pressure=0.5})
h:line({{268,604},{262,540},{258,480},{262,420}}, {pressure=0.5})
h:line({{240,470},{200,400},{160,350}}, {pressure=0.4})
h:line({{258,450},{300,380},{340,330}}, {pressure=0.4})
h:line({{250,420},{248,300},{255,200}}, {pressure=0.4})
h2:sketch({{120,460},{95,400},{105,330},{140,270},{160,200},{210,150},{270,120},{330,140},{370,190},{400,250},{415,320},{410,390},{380,450},{330,470},{270,470},{200,470},{120,460}}, {pressure=0.3})
-- figures
h:line({{360,590},{358,570},{360,556},{364,556},{366,570},{366,590}}, {pressure=0.4, smooth=false})
h:line({{376,590},{372,572},{376,560},{380,560},{384,572},{382,590}}, {pressure=0.4, smooth=false})

--@ chunk 5
s1 = pile{{"lead white",5},{"cobalt blue",2},{"smalt",1.6}, medium=0.12}
s2 = pile{{"lead white",7},{"cobalt blue",1.2},{"smalt",1.0}, medium=0.12}
s3 = pile{{"lead white",9},{"pale smalt",1.6},{"cobalt blue",0.4}, medium=0.12}
s4 = pile{{"lead white",10},{"pale smalt",0.7},{"yellow ochre",0.15}, medium=0.12}
s5 = pile{{"lead white",10},{"yellow ochre",0.45},{"vermilion",0.06}, medium=0.12}
local bands = {{s1,0,110},{s2,95,210},{s3,195,300},{s4,285,370},{s5,355,HZ+4}}
for i,b in ipairs(bands) do
  local m = rect(-10, b[2], 1020, b[3]-b[2])
  work(m, {hand="broad", pile=b[1], angle=function(x,y) return 0.03*math.sin(x/170+i) end, coverage=2.5, fill=true, pressure={0.5,0.75}, seed=10+i})
end

--@ chunk 6
blend(rect(0,0,1000,HZ+2), {angle=0, coverage=2})
blend(rect(0,0,1000,HZ+2), {angle=0.02, coverage=1.5, seed=77})

--@ chunk 7
s0 = pile{{"lead white",3},{"cobalt blue",2},{"smalt",2}, medium=0.15}
work(rect(-10,-5,1020,80), {hand="broad", pile=s0, angle=0, coverage=2, fill=true, pressure={0.45,0.65}, seed=31})
blend(rect(0,0,1000,170), {angle=0, coverage=2.5})

--@ chunk 8
blend(rect(0,40,1000,110), {angle=0.08, coverage=2, seed=5})
blend(rect(0,50,1000,90), {angle=-0.06, coverage=2, seed=6})

--@ chunk 9
local B = {{615,378,55,30},{688,352,62,44},{758,322,66,56},{828,342,58,46},{896,358,66,40},{958,378,60,30},{722,380,80,30},{850,388,90,24},{572,398,40,16},{800,300,40,35}}
local parts = {}
for i,b in ipairs(B) do
  parts[#parts+1] = {body.ellipsoid({b[1],b[2],0},{b[3],b[4],(b[3]+b[4])/2}):rough(4, 60, i)}
end
parts.light = {from={-1,-0.9}, front=0.4, ambient=0.25}
cloudF = form(parts)
local sil = cloudF:silhouette{} * below(function(x) return 0 end):map(function(v) return 1 end)
cloudM = (cloudF:silhouette{} * rect(0,0,1000,404)):soften(3)
cloudLit = (cloudF:lit{soft=0.2} * rect(0,0,1000,404))
cs = pile{{"lead white",8},{"pale smalt",2},{"red earth",0.25},{"yellow ochre",0.2}, medium=0.1}
cl = pile{{"lead white",12},{"yellow ochre",0.35},{"vermilion",0.04}}
work(cloudM, {hand="body", pile=cs, angle=0.05, coverage=2, fill=true, pressure={0.4,0.7}, edge="soft", seed=41})
work(cloudLit, {hand="body", pile=cl, tool="filbert 6", angle=function(x,y) return cloudF:across(x,y) end, coverage=2, fill=true, pressure={0.5,0.8}, edge={found=0.5, soft=0.5}, seed=42})
print(cloudM:area(), cloudLit:area())

--@ chunk 10
riseCurve = {{-10,578},{80,566},{180,558},{280,564},{380,582},{460,600},{540,622},{620,640},{720,662},{820,690},{900,714},{1010,730}}
landM = below(function(x) return HZ + 1 end)
riseM = below(riseCurve)
u1 = pile{{"raw umber",1},{"yellow ochre",1.2},{"green earth",2}, medium=0.45}
u2 = pile{{"raw umber",2},{"green earth",1.5},{"bone black",0.25},{"yellow ochre",0.5}, medium=0.45}
work(landM - riseM, {hand="broad", pile=u1, angle=0.02, coverage=1.6, fill=true, pressure={0.4,0.6}, seed=51})
work(riseM, {hand="broad", pile=u2, angle=-0.05, coverage=1.6, fill=true, pressure={0.4,0.6}, seed=52})

--@ chunk 11
print(wait(20*60)); print(drying(500,100), drying(700,330), drying(500,600))

--@ chunk 12
print(wait(4*60)); print(drying(500,100), drying(700,330), drying(500,600))

--@ chunk 13
local cloudHole = cloudM:grow(1)
b2 = pile{{"lead white",6},{"cobalt blue",1.1},{"smalt",0.8},{"pale smalt",0.6}, medium=0.12}
b3 = pile{{"lead white",8},{"pale smalt",1.8},{"cobalt blue",0.45}, medium=0.12}
b4 = pile{{"lead white",10},{"pale smalt",1.2},{"cobalt blue",0.1},{"yellow ochre",0.08}, medium=0.12}
local bands = {{b2,55,165},{b3,150,265},{b4,250,345}}
for i,b in ipairs(bands) do
  local m = rect(-10,b[2],1020,b[3]-b[2]) - cloudHole
  work(m, {hand="broad", pile=b[1], angle=function(x,y) return 0.03*math.sin(x/200+i*2) end, coverage=2.2, fill=true, pressure={0.45,0.7}, edge=0.12, seed=60+i})
end
blend(rect(0,40,1000,320) - cloudHole, {angle=0, coverage=2.5, seed=64})

--@ chunk 14
local cloudHole = cloudM:grow(1)
d1 = pile{{"lead white",3},{"cobalt blue",2},{"smalt",1.6}, medium=0.12}
d2 = pile{{"lead white",5},{"cobalt blue",1.6},{"smalt",1.0}, medium=0.12}
d3 = pile{{"lead white",7},{"cobalt blue",0.9},{"pale smalt",1.5}, medium=0.12}
local bands = {{d1,-5,105},{d2,90,195},{d3,180,275}}
for i,b in ipairs(bands) do
  local m = rect(-10,b[2],1020,b[3]-b[2]) - cloudHole
  work(m, {hand="broad", pile=b[1], angle=function(x,y) return 0.03*math.sin(x/230+i*3) end, coverage=2.2, fill=true, pressure={0.45,0.7}, edge=0.12, seed=70+i})
end
blend(rect(0,0,1000,350) - cloudHole, {angle=0, coverage=3, seed=74})
blend(rect(0,60,1000,260) - cloudHole, {angle=0.03, coverage=2, seed=75})

--@ chunk 15
print(wait(22*60)); print(drying(500,100), drying(700,330), drying(500,600), drying(500,250))

--@ chunk 16
cg = pile{{"lead white",4},{"smalt",2},{"red earth",0.35},{"raw umber",0.25}, medium=0.25}
local sh = cloudF:shadow{soft=0.25} * rect(0,0,1000,404)
work(sh, {hand="body", tool="filbert 7", pile=cg, angle=function(x,y) return cloudF:across(x,y) end, coverage=1.8, fill=true, pressure={0.35,0.6}, edge="soft", seed=81})
-- the flat underside: a band of deeper grey along the base
local base = (cloudM * rect(0,372,1000,34)):soften(4)
work(base, {hand="body", tool="filbert 6", pile=cg, angle=0.02, coverage=1.5, pressure={0.35,0.55}, edge="soft", seed=82})
blend(cloudM, {angle=0.1, coverage=1.2, seed=83})

--@ chunk 17
local n = noise{seed=5, octaves=4, period=60}
local n2 = noise{seed=9, octaves=3, period=18}
farTop = function(x)
  local base = 433 + 2*math.sin(x/140)
  -- groves: little rounded bumps in places
  local g = math.max(0, n(x,0)) * 9 + math.max(0, n2(x,3)) * 3
  return base - g
end
farM = below(farTop) * above(function(x) return 452 end)
fl = pile{{"lead white",5},{"pale smalt",2},{"green earth",1},{"raw umber",0.25}, medium=0.1}
work(farM, {hand="body", tool="filbert 5", pile=fl, angle=0, coverage=2.2, fill=true, pressure={0.4,0.6}, edge=0.1, seed=91})

--@ chunk 18
fl2 = pile{{"lead white",3},{"smalt",1.5},{"green earth",1.5},{"raw umber",0.45},{"Prussian blue",0.06}, medium=0.1}
work(farM, {hand="body", tool="filbert 5", pile=fl2, angle=0, coverage=2, fill=true, pressure={0.45,0.65}, edge=0.08, seed=92})

--@ chunk 19
local P = {}
-- main church tower with bell cap, lantern, spire
P[#P+1] = poly({{596,440},{596,368},{594,367},{597,360},{600,352},{603,349},{603,338},{604.2,336},{605,322},{605.8,336},{607,338},{607,349},{610,352},{613,360},{616,367},{614,368},{614,440}})
-- main nave
P[#P+1] = poly({{613,440},{613,416},{616,401},{688,401},{697,414},{699,416},{699,440}})
-- stair turret / small ridge turret on nave
P[#P+1] = poly({{655,402},{655,396},{656.5,390},{658,396},{658,402}})
-- second church, massive tower with low cap
P[#P+1] = poly({{720,440},{720,382},{722,380},{729,369},{736,380},{738,382},{738,440}})
P[#P+1] = poly({{737,440},{737,420},{745,408},{787,408},{794,420},{794,440}})
-- third church slender spire
P[#P+1] = poly({{540,440},{540,396},{539,395},{545.5,370},{552,395},{551,396},{551,440}})
P[#P+1] = poly({{551,440},{551,424},{556,416},{582,416},{586,424},{586,440}})
-- houses and roofs
local roofs = {{498,510,429,425},{509,522,430,424},{521,540,428,422},{586,596,430,425},{699,712,430,423},{711,720,427,423},{794,806,429,424},{805,818,431,426},{818,832,433,429},{832,846,435,431},{655,672,430,426},{565,579,432,428}}
for _,r in ipairs(roofs) do
  local x0,x1,e,rg = r[1],r[2],r[3],r[4]
  P[#P+1] = poly({{x0,440},{x0,e},{(x0+x1)/2,rg-3},{x1,e},{x1,440}})
end
townM = P[1]
for i=2,#P do townM = townM + P[i] end
tw = pile{{"lead white",3},{"smalt",2},{"red earth",0.35},{"raw umber",0.45},{"bone black",0.05}, medium=0.1}
work(townM, {hand="detail", pile=tw, tool={kind="round", width=1.6, point=0.6}, angle=math.pi/2, coverage=3, fill=true, seed=101})

--@ chunk 20
print(wait(20*60)); print(drying(605,400), drying(700,330), drying(640,320))

--@ chunk 21
local top = {{556,394},{572,383},{590,373},{618,362},{640,350},{648,334},{660,318},{680,311},{700,317},{712,303},{730,287},{750,273},{770,266},{790,268},{804,282},{822,290},{842,297},{860,305},{874,318},{898,329},{924,332},{946,340},{966,346},{986,352},{1004,360}}
local n = noise{seed=17, octaves=3, period=25}
local pts = {}
for i,p in ipairs(top) do pts[#pts+1] = {p[1], p[2]-1} end
for i=#top,1,-1 do
  local p = top[i]
  pts[#pts+1] = {p[1]+3, p[2] + 13 + 8*n(p[1],p[2])}
end
litTopM = poly(pts, true):roughen(2.5, 14, 3)
cl2 = pile{{"lead white",14},{"yellow ochre",0.3},{"vermilion",0.05}}
work(litTopM, {hand="body", tool="filbert 4", pile=cl2, angle=function(x,y) return cloudF:across(x,y) end, coverage=2.2, fill=true, pressure={0.45,0.75}, edge={found=0.7, soft=0.3}, seed=111})
-- inner billow rims
local rims = {
 {{690,338},{705,328},{722,322},{742,322},{756,330}},
 {{770,312},{785,302},{800,300},{812,308}},
 {{800,338},{815,328},{835,326},{852,334},{862,345}},
 {{880,350},{900,342},{922,343},{940,352}},
 {{612,378},{628,370},{648,368},{664,374}},
 {{940,366},{958,358},{978,360},{996,368}},
}
for j,r in ipairs(rims) do
  local rp = {}
  for i,p in ipairs(r) do rp[#rp+1] = {p[1], p[2]+3} end
  local m = ribbon(rp, 6):roughen(2, 10, 20+j)
  work(m, {hand="body", tool="filbert 3", pile=cl2, angle=0.1, coverage=1.8, pressure={0.4,0.6}, edge={found=0.6, soft=0.4}, seed=120+j})
end

--@ chunk 22
local m = (cloudM + litTopM) * rect(540,250,470,152)
blend(m, {angle=function(x,y) return 1.35 + 0.3*math.sin(x/40) end, coverage=2.5, seed=131})
blend(m, {angle=0.9, coverage=1.2, seed=132})

--@ chunk 23
print(wait(24*60)); print(drying(605,400), drying(700,330), drying(640,320))

--@ chunk 24
print(wait(12*60)); print(drying(605,400), drying(700,330), drying(760,300), drying(900,350))

--@ chunk 25
cgl = pile{{"lead white",2},{"smalt",2},{"red earth",0.3},{"raw umber",0.3}, medium=0.5}
local grad = mask(function(x,y) return smoothstep(290, 395, y) end)
local sh = (cloudF:shadow{soft=0.35} + grad:map(function(v) return v*0.8 end)) * cloudM * rect(540,250,470,152)
local shm = sh:map(function(v) return v > 0.45 and 1 or 0 end):soften(4)
work(shm, {hand="glaze", tool={kind="filbert", width=9, stiffness=0.3}, length={25,60}, pile=cgl, angle=function(x,y) return cloudF:across(x,y) end, coverage=1.3, pressure={0.25,0.45}, edge="soft", seed=141})
blend(shm:grow(4) * cloudM, {angle=0.2, coverage=1.2, seed=142})

--@ chunk 26
local m = cloudM * rect(540,250,470,151) - townM:grow(3)
blend(m, {angle=0.3, coverage=2.5, seed=151})
blend(m, {angle=-0.4, coverage=2.5, seed=152})
blend(m, {angle=1.2, coverage=1.5, seed=153})

--@ chunk 27
riverM = poly({{1010,527},{900,514},{780,499},{680,481},{600,467},{520,457},{440,449.5},{380,446.5},{330,445},{330,445.6},{380,448.5},{440,455},{510,465},{590,478},{670,495},{770,515},{890,533},{1010,550}}, true)
local land = below(function(x) return 446 end) - farM:shrink(1)
g1 = pile{{"lead white",4},{"green earth",2},{"pale smalt",1},{"yellow ochre",1.2}, medium=0.08}
g2 = pile{{"yellow ochre",3},{"Prussian blue",0.25},{"lead white",2},{"green earth",1}, medium=0.08}
g3 = pile{{"yellow ochre",3},{"Prussian blue",0.45},{"lead white",1},{"raw umber",0.3}, medium=0.08}
local z1 = (land * rect(-10,440,1020,42)) - riverM
local z2 = (land * rect(-10,476,1020,70)) - riverM - riseM
local z3 = (land * rect(-10,538,1020,190)) - riverM - riseM
work(z1, {hand="body", tool="filbert 6", pile=g1, angle=0, angle_jitter=0.03, coverage=2.2, fill=true, pressure={0.45,0.65}, seed=161})
work(z2, {hand="body", tool="filbert 8", pile=g2, angle=function(x,y) return 0.12*math.sin(x/260)+0.02 end, coverage=2.2, fill=true, pressure={0.45,0.7}, seed=162})
work(z3, {hand="body", tool="filbert 10", pile=g3, angle=function(x,y) return 0.18 end, coverage=2.2, fill=true, pressure={0.45,0.7}, seed=163})
blend((land - riverM - riseM) * rect(-10,440,1020,280), {angle=0.03, coverage=1.2, seed=164})

--@ chunk 28
rv1 = pile{{"lead white",8},{"pale smalt",1.2},{"yellow ochre",0.15}, medium=0.08}
rv2 = pile{{"lead white",6},{"pale smalt",1.5},{"cobalt blue",0.35}, medium=0.08}
work(riverM * rect(300,430,380,80), {hand="body", tool="filbert 3", pile=rv1, angle=0.08, coverage=2.5, fill=true, clip=true, pressure={0.4,0.6}, seed=171})
work(riverM * rect(680,430,340,140), {hand="body", tool="filbert 5", pile=rv2, angle=0.15, coverage=2.5, fill=true, clip=true, pressure={0.4,0.6}, seed=172})
blend(riverM, {angle=0.12, coverage=1.5, seed=173})

--@ chunk 29
print(wait(24*60)); print(drying(605,400), drying(700,330), drying(700,490), drying(500,520),drying(700,500))

--@ chunk 30
print(wait(36*60)); print(drying(605,400), drying(700,330), drying(700,490), drying(500,520),drying(700,500), drying(800,300))

--@ chunk 31
local up = {{600,352,18},{630,340,22},{660,325,24},{690,306,26},{722,288,26},{752,275,26},{785,270,24},{812,280,20},{838,295,20},{866,309,22},{898,321,20},{930,330,18},{960,340,18},{990,350,18}}
local lo = {{650,362,18},{705,332,20},{760,316,22},{820,326,18},{880,346,18},{940,358,16},{985,370,15}}
local parts = {}
for i,b in ipairs(up) do parts[#parts+1] = {body.ellipsoid({b[1],b[2],0},{b[3]*1.1,b[3]*0.9,b[3]}):rough(2.5, 25, i)} end
for i,b in ipairs(lo) do parts[#parts+1] = {body.ellipsoid({b[1],b[2],25},{b[3]*1.15,b[3]*0.85,b[3]}):rough(2.5, 25, 50+i)} end
parts.light = {from={-1,-1.1}, front=0.25, ambient=0.15}
billF = form(parts)
local notTown = -(townM:grow(2))
ch = pile{{"lead white",9},{"pale smalt",1},{"red earth",0.1},{"yellow ochre",0.25}, medium=0.05}
cl3 = pile{{"lead white",14},{"yellow ochre",0.28},{"vermilion",0.04}}
local half = billF:lit{soft=0.45} * notTown
local lit = billF:lit{soft=0.12} * notTown
work(half, {hand="body", tool="filbert 4", pile=ch, angle=function(x,y) return billF:across(x,y) end, coverage=1.6, fill=true, pressure={0.35,0.55}, edge={found=0.6, soft=0.4}, seed=181})
work(lit, {hand="body", tool="filbert 3", pile=cl3, angle=function(x,y) return billF:across(x,y) end, coverage=2, fill=true, pressure={0.4,0.65}, edge={found=0.7, soft=0.3}, seed=182})
print(half:area(), lit:area())

--@ chunk 32
local m = (cloudM:shrink(8)) * rect(540,295,470,100) - townM:grow(3)
blend(m, {tool={kind="badger", width=14}, angle=0.75, coverage=1.6, seed=191})
blend(m, {tool={kind="badger", width=10}, angle=0.2, coverage=1.0, seed=192})

--@ chunk 33
function riseY(x)
  local c = riseCurve
  for i=1,#c-1 do
    if x >= c[i][1] and x <= c[i+1][1] then
      local t = (x - c[i][1])/(c[i+1][1]-c[i][1])
      t = t*t*(3-2*t)
      return lerp(c[i][2], c[i+1][2], t)
    end
  end
  return c[#c][2]
end
local riseTopM = below(function(x) return riseY(x) - 1 end)
local bandA = riseTopM - below(function(x) return riseY(x) + 22 end)
local bandB = below(function(x) return riseY(x) + 18 end) - below(function(x) return 655 + (x-300)*0.05 end)
local bandC = below(function(x) return 650 + (x-300)*0.05 end) * riseTopM
r1 = pile{{"yellow ochre",3},{"Prussian blue",0.3},{"lead white",1.5},{"chrome yellow",0.35}, medium=0.06}
r2 = pile{{"yellow ochre",2.5},{"Prussian blue",0.55},{"raw umber",0.6},{"lead white",0.4}, medium=0.06}
r3 = pile{{"raw umber",1.5},{"Prussian blue",0.6},{"yellow ochre",1},{"bone black",0.12}, medium=0.06}
work(bandC, {hand="body", tool="filbert 10", pile=r3, angle=-0.08, coverage=2.2, fill=true, pressure={0.45,0.7}, seed=201})
work(bandB, {hand="body", tool="filbert 8", pile=r2, angle=function(x,y) return math.atan(riseY(x+5)-riseY(x-5), 10) end, coverage=2.2, fill=true, pressure={0.45,0.7}, seed=202})
work(bandA, {hand="body", tool="filbert 5", pile=r1, angle=function(x,y) return math.atan(riseY(x+5)-riseY(x-5), 10) end, coverage=2.4, fill=true, pressure={0.45,0.7}, edge={found=0.3, soft=0.7}, seed=203})
blend(riseTopM:shrink(3), {angle=function(x,y) return math.atan(riseY(x+5)-riseY(x-5), 10) end, coverage=1.3, seed=204})

--@ chunk 34
print(wait(26*60)); print(drying(250,570), drying(250,650), drying(700,330), drying(800,520))

--@ chunk 35
oakLimbs = {
  {{250,478},{228,452},{205,420},{182,395},{155,365},{128,338}, widths={11,9,7.5,6,4.5,3}},
  {{249,470},{240,430},{232,385},{224,340},{216,290},{208,240},{203,195}, widths={13,11,9.5,8,6,4.5,3}},
  {{253,468},{266,430},{280,395},{297,350},{312,300},{322,250},{330,195}, widths={12,10.5,9,7.5,6,4.5,3}},
  {{255,488},{280,470},{310,450},{345,425},{378,398},{400,380}, widths={10,8.5,7,5.5,4,2.5}},
  {{226,338},{200,312},{175,290},{150,262}, widths={5.5,4.5,3.5,2.5}},
  {{300,340},{330,318},{362,300},{392,280}, widths={5.5,4.5,3.5,2.5}},
  {{212,250},{232,215},{248,180},{262,145}, widths={4.5,3.8,3,2.2}},
  {{318,265},{300,230},{285,195},{280,160}, widths={4.5,3.8,3,2.2}},
  {{185,398},{160,405},{135,415},{115,425}, widths={4,3.3,2.6,2}},
  {{330,432},{350,445},{372,452}, widths={3.5,2.8,2}},
}
oakBody = body_of{spine={{248,584},{245,560},{247,530},{250,500},{251,478}}, widths={34,29,26,23,21}, limbs=oakLimbs, blend=0.8, char="soft", seed=7}
oakM = oakBody:mask()
bk = pile{{"raw umber",2},{"bone black",0.4},{"yellow ochre",0.5},{"lead white",0.4}, medium=0.05}
work(oakM, {hand="detail", pile=bk, tool={kind="round", width=3, point=0.5}, angle=math.pi/2, coverage=2.5, fill=true, seed=211})

--@ chunk 36
clumps = {
 {240,142,34},{287,128,28},{203,172,30},{330,175,32},
 {160,218,34},{232,212,36},{300,214,34},{372,232,30},
 {128,278,32},{198,282,28},{262,272,30},{333,288,34},{397,302,26},
 {104,342,30},{170,340,32},{242,342,26},{312,352,30},{382,350,28},
 {120,400,26},{186,405,24},{290,410,24},{360,410,28},{410,392,20},{150,440,18},{343,446,16},
}
local parts = {}
for i,c in ipairs(clumps) do
  parts[#parts+1] = {body.ellipsoid({c[1],c[2],rand(-10,10)},{c[3]*1.12,c[3]*0.88,c[3]}):rough(c[3]*0.18, c[3]*0.5, 300+i)}
end
parts.light = {from={-1,-0.6}, front=0.35, ambient=0.15}
oakF = form(parts)
oakSil = oakF:silhouette{}
fd = pile{{"Prussian blue",1},{"raw umber",1.1},{"yellow ochre",1},{"bone black",0.15}, medium=0.05}
work(oakSil:shrink(6), {hand="body", tool="filbert 6", pile=fd, angle=function(x,y) return oakF:across(x,y) end, coverage=2, fill=true, pressure={0.45,0.7}, seed=221})
stipple(oakSil:grow(2), {pile=fd, width=5, coverage=function(x,y) return 1.4 end, pressure={0.4,0.8}, cluster={0.5, 6}, feather=0.5, dips={20,0.7,0.2}, seed=222})
print(oakSil:area())

--@ chunk 37
local nz = noise{seed=33, octaves=3, period=22}
local fringe = oakSil:grow(10) - oakSil:shrink(3)
stipple(fringe, {pile=fd, width=3.5, coverage=function(x,y) local d = oakSil:at(x,y); return clamp(0.25 + 0.9*d + 0.6*nz(x,y), 0, 1.6) end, pressure={0.35,0.75}, cluster={0.6, 5}, feather=0.8, dips={18,0.6,0.2}, seed=231})

--@ chunk 38
print(wait(8*60)); print(drying(240,200), drying(300,300), drying(250,540))

--@ chunk 39
fm = pile{{"yellow ochre",2},{"Prussian blue",0.45},{"raw umber",0.3},{"lead white",0.5}, medium=0.04}
local nz = noise{seed=44, octaves=3, period=16}
local half = oakF:lit{soft=0.5}
stipple(half:grow(3), {pile=fm, width=4, coverage=function(x,y) return clamp(1.1*half:at(x,y) + 0.5*nz(x,y), 0, 1.5) end, pressure={0.35,0.7}, cluster={0.55, 5}, feather=0.7, dips={16,0.6,0.2}, seed=241})

--@ chunk 40
print(wait(6*60))
fl3 = pile{{"yellow ochre",2},{"chrome yellow",0.6},{"Prussian blue",0.2},{"lead white",1.3}, medium=0.03}
local nz = noise{seed=55, octaves=3, period=12}
local lit = oakF:lit{soft=0.15}
stipple(lit, {pile=fl3, width=3, coverage=function(x,y) return clamp(0.8*lit:at(x,y) + 0.5*nz(x,y), 0, 1.1) end, pressure={0.3,0.6}, cluster={0.6, 4}, feather=0.8, dips={14,0.5,0.2}, seed=251})

--@ chunk 41
local P = {}
-- churches as before
P[#P+1] = poly({{596,440},{596,368},{594,367},{597,360},{600,352},{603,349},{603,338},{604.2,336},{605,322},{605.8,336},{607,338},{607,349},{610,352},{613,360},{616,367},{614,368},{614,440}})
P[#P+1] = poly({{613,440},{613,416},{616,401},{688,401},{697,414},{699,416},{699,440}})
P[#P+1] = poly({{655,402},{655,396},{656.5,390},{658,396},{658,402}})
P[#P+1] = poly({{720,440},{720,382},{722,380},{729,369},{736,380},{738,382},{738,440}})
P[#P+1] = poly({{737,440},{737,420},{745,408},{787,408},{794,420},{794,440}})
P[#P+1] = poly({{540,440},{540,396},{539,395},{545.5,370},{552,395},{551,396},{551,440}})
P[#P+1] = poly({{551,440},{551,424},{556,416},{582,416},{586,424},{586,440}})
-- houses: a continuous but varied roofline
local hs = {
 {492,500,432,"g"},{500,514,428,"e"},{514,524,424,"g"},{524,540,427,"e"},
 {586,597,426,"g"},{614,630,424,"e"},{699,708,425,"g"},{708,720,428,"e"},
 {794,805,425,"g"},{805,822,429,"e"},{822,832,427,"g"},{832,848,432,"e"},{848,856,435,"g"},
}
for _,h in ipairs(hs) do
  local x0,x1,top,k = h[1],h[2],h[3],h[4]
  if k == "g" then
    P[#P+1] = poly({{x0,440},{x0,top+5},{(x0+x1)/2,top-1},{x1,top+5},{x1,440}})
  else
    P[#P+1] = poly({{x0,440},{x0,top+5},{x0+3,top},{x1-3,top},{x1,top+5},{x1,440}})
  end
end
townM2 = P[1]
for i=2,#P do townM2 = townM2 + P[i] end
tw2 = pile{{"lead white",2.5},{"smalt",2},{"raw umber",0.5},{"red earth",0.3},{"Prussian blue",0.04}, medium=0.08}
work(townM2, {hand="detail", pile=tw2, tool={kind="round", width=1.4, point=0.6}, angle=math.pi/2, coverage=3, fill=true, seed=261})

--@ chunk 42
print(wait(10*60)); print(drying(600,420), drying(240,200), drying(300,300))

--@ chunk 43
crownF = form{ {body.ellipsoid({255,290,0},{190,185,150})}, light={from={-1,-0.5}, front=0.3, ambient=0.1} }
local nz = noise{seed=66, octaves=3, period=20}
local sh = crownF:shadow{soft=0.5} * oakSil
fd2 = pile{{"Prussian blue",1},{"raw umber",1.2},{"yellow ochre",0.8},{"bone black",0.2}, medium=0.1}
stipple(sh, {pile=fd2, width=4.5, coverage=function(x,y) return clamp(1.3*sh:at(x,y) + 0.4*nz(x,y), 0, 1.6) end, pressure={0.4,0.75}, cluster={0.5, 5}, feather=0.6, dips={18,0.6,0.2}, seed=271})
-- the underside of each lower mass in deep shade
local under = oakSil * mask(function(x,y) return smoothstep(380, 470, y) end)
stipple(under, {pile=fd2, width=4.5, coverage=function(x,y) return clamp(1.2*under:at(x,y) + 0.3*nz(x,y), 0, 1.4) end, pressure={0.4,0.75}, cluster={0.5, 5}, feather=0.6, dips={18,0.6,0.2}, seed=272})

--@ chunk 44
local sp = {{248,584},{245,560},{247,530},{250,500},{251,478}}
function spineX(y)
  for i=1,#sp-1 do
    local a,b = sp[i],sp[i+1]
    if y <= a[2] and y >= b[2] then return lerp(a[1], b[1], (a[2]-y)/(a[2]-b[2])) end
  end
  return 251
end
local trunk = oakM * rect(200,470,100,120) - oakSil
tb1 = pile{{"raw umber",2},{"bone black",0.5},{"lead white",0.8},{"Prussian blue",0.1}, medium=0.05}
tb2 = pile{{"lead white",2},{"raw umber",1},{"yellow ochre",0.8},{"bone black",0.15}, medium=0.05}
-- dark bark over all visible limbs and trunk
work(oakM - oakSil:shrink(2), {hand="detail", pile=tb1, tool={kind="round", width=2.2, point=0.5}, angle=math.pi/2, coverage=2.2, fill=true, seed=281})
-- lit left side of trunk
local lit = trunk * mask(function(x,y) return smoothstep(-2, -9, x - spineX(y)) end)
work(lit, {hand="detail", pile=tb2, tool={kind="round", width=1.8, point=0.5}, angle=math.pi/2, coverage=2, fill=true, seed=282})

--@ chunk 45
trunkM = poly({{226,590},{230,578},{233,560},{232,540},{234,515},{237,495},{238,478},{244,470},{258,470},{264,478},{264,495},{265,515},{266,540},{267,560},{271,578},{276,590}}, true)
tb3 = pile{{"raw umber",2},{"bone black",0.6},{"Prussian blue",0.15},{"lead white",0.25}, medium=0.05}
tb4 = pile{{"lead white",1.6},{"raw umber",1},{"yellow ochre",0.9},{"bone black",0.1}, medium=0.05}
work(trunkM, {hand="detail", pile=tb3, tool={kind="round", width=2.5, point=0.5}, angle=math.pi/2, coverage=2.5, fill=true, seed=291})
local lit = trunkM * mask(function(x,y) return smoothstep(-3, -11, x - spineX(y)) end)
work(lit, {hand="detail", pile=tb4, tool={kind="round", width=2, point=0.5}, angle=math.pi/2, coverage=2, fill=true, seed=292})
-- limbs underside dark too
local limbs = (oakM - trunkM:grow(1) - oakSil:shrink(2)) * rect(90,300,340,200)
work(limbs * mask(function(x,y) return 1 end), {hand="detail", pile=tb3, tool={kind="round", width=1.8, point=0.5}, angle=math.pi/2, coverage=1.5, fill=true, seed=293})

--@ chunk 46
print(wait(14*60)); print(drying(250,540), drying(600,420), drying(600,520), drying(800,600), drying(300,640))

--@ chunk 47
-- distant groves on the horizon
local groves = {{8,50,424},{92,130,425},{165,250,422},{395,445,427},{455,470,429},{870,905,426},{930,990,425}}
local G = nil
for i,g in ipairs(groves) do
  local x0,x1,top = g[1],g[2],g[3]
  local pts = {{x0,436}}
  local n = math.max(3, math.floor((x1-x0)/7))
  for k=0,n do
    local x = lerp(x0,x1,k/n)
    local bump = math.sin(k/n*math.pi)^0.6
    pts[#pts+1] = {x, 436 - (436-top)*bump - rand(0,2.5)}
  end
  pts[#pts+1] = {x1,436}
  local m = poly(pts, true)
  G = G and (G + m) or m
end
grovesM = G
fg = pile{{"lead white",2},{"smalt",1.5},{"green earth",1.5},{"raw umber",0.6}, medium=0.08}
work(grovesM, {hand="detail", pile=fg, tool={kind="round", width=1.8, point=0.5}, angle=math.pi/2, coverage=3, fill=true, seed=301})

--@ chunk 48
local fgd = pile{{"lead white",1.2},{"smalt",1.5},{"green earth",1.5},{"raw umber",0.8},{"Prussian blue",0.05}, medium=0.08}
local avoid = oakM:grow(1) + oakSil + trunkM:grow(1)
local gm = (grovesM:grow(1.5) - avoid)
stipple(gm, {pile=fgd, width=2.4, coverage=1.3, pressure={0.35,0.7}, cluster={0.5,3}, feather=0.6, clip=grovesM:grow(2.5) - avoid, dips={20,0.6,0.2}, seed=311})
-- repair the limbs the grove went over
local rep = (oakM + trunkM) * grovesM:grow(3) - oakSil:shrink(2)
work(rep, {hand="detail", pile=tb3, tool={kind="round", width=1.8, point=0.5}, angle=math.pi/2, coverage=2.5, fill=true, seed=312})

--@ chunk 49
function interp(pts, x)
  if x <= pts[1][1] then return pts[1][2] end
  for i=1,#pts-1 do
    if x >= pts[i][1] and x <= pts[i+1][1] then return lerp(pts[i][2], pts[i+1][2], (x-pts[i][1])/(pts[i+1][1]-pts[i][1])) end
  end
  return pts[#pts][2]
end
farBank = {{330,444.5},{380,446.5},{440,449.5},{520,457},{600,467},{680,481},{780,499},{900,514},{1010,527}}
nearBank = {{330,445.6},{380,448.5},{440,455},{510,465},{590,478},{670,495},{770,515},{890,533},{1010,550}}
wCrD = pile{{"green earth",2},{"raw umber",0.7},{"Prussian blue",0.2},{"lead white",0.4}, medium=0.05}
wCrL = pile{{"lead white",2},{"green earth",1.5},{"pale smalt",0.5},{"yellow ochre",0.4}, medium=0.05}
wTr = pile{{"raw umber",1.5},{"bone black",0.3},{"lead white",0.6}, medium=0.05}
willowSil = nil
function willow(x, s, seed)
  local y = interp(farBank, x) + 0.5
  local lean = rand(-0.08, 0.08)
  local th = 0.42*s
  local tr = poly({{x-0.09*s,y},{x-0.08*s+lean*th, y-th*0.7},{x-0.15*s+lean*th, y-th},{x+0.15*s+lean*th,y-th},{x+0.08*s+lean*th,y-th*0.7},{x+0.09*s,y}}, true)
  local cx, cy = x + lean*th, y - th - 0.28*s
  local crown = ellipse(cx, cy, 0.45*s, 0.33*s):roughen(0.06*s, 0.15*s, seed)
  local tipw = math.max(1.2, 0.05*s)
  work(tr, {hand="detail", pile=wTr, tool={kind="round", width=math.max(1.2, 0.06*s), point=0.5}, angle=math.pi/2, coverage=2.5, fill=true, seed=seed})
  stipple(crown, {pile=wCrD, width=math.max(1.6,0.09*s), coverage=2.2, drag={0.12*s, -math.pi/2}, pressure={0.4,0.8}, cluster={0.3,3}, feather=0.5, seed=seed+1})
  local litc = crown * mask(function(px,py) return smoothstep(0.1*s, -0.35*s, (px-cx) + 0.6*(py-cy)) end)
  stipple(litc, {pile=wCrL, width=math.max(1.3,0.07*s), coverage=1.4, drag={0.1*s, -math.pi/2+0.1}, pressure={0.35,0.7}, cluster={0.4,3}, feather=0.7, seed=seed+2})
  local sil = tr + crown
  willowSil = willowSil and (willowSil + sil) or sil
end
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  willow(w[1], w[2], 400 + i*7)
end

--@ chunk 50
local rg = brush{kind="rigger", width=1.0, point=1}
local wTr2 = pile{{"raw umber",1.2},{"bone black",0.4},{"lead white",1.0},{"green earth",0.3}, medium=0.05}
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local y = interp(farBank, x) + 0.5
  local ky = y - 0.42*s
  rg:reload(wCrD, 0.7)
  local n = math.floor(8 + s*0.3)
  for k=1,n do
    local a = -math.pi/2 + (k/(n+1) - 0.5) * 2.1 + rand(-0.1,0.1)
    local L = s * rand(0.55, 0.78)
    local x0, y0 = x + rand(-0.08,0.08)*s, ky + rand(-0.02,0.04)*s
    local bend = rand(-0.1,0.1)
    rg:stroke({{x0,y0},{x0+math.cos(a)*L*0.5, y0+math.sin(a)*L*0.5},{x0+math.cos(a+bend)*L, y0+math.sin(a+bend)*L}}, {pressure={0.35,0.0}, ramps={0.05,0.6}})
    if k % 4 == 0 then rg:reload(wCrD, 0.7) end
  end
  -- the gnarled grey trunk restated
  local th = 0.42*s
  local tb = brush{kind="round", width=math.max(1.3, 0.12*s), point=0.3}
  tb:reload(wTr2, 0.8)
  tb:stroke({{x, y}, {x+rand(-0.5,0.5), y-th*0.5}, {x+rand(-0.8,0.8), y-th}}, {pressure={0.9,0.8}})
end

--@ chunk 51
local bankP = pile{{"raw umber",1},{"green earth",1.5},{"Prussian blue",0.2},{"lead white",0.5}, medium=0.1}
local wRef = pile{{"green earth",1.5},{"raw umber",0.8},{"Prussian blue",0.15},{"lead white",1.0}, medium=0.25}
-- a thin dark line under the far bank: the bank's own reflection
local pts, wd = {}, {}
for x=335,1005,10 do pts[#pts+1] = {x, interp(farBank,x) + 0.4 + 0.004*(x-335)} ; wd[#wd+1] = 0.8 + 0.0035*(x-335) + rand(-0.3,0.3) end
local bankLine = ribbon(pts, wd) * riverM:grow(0.5)
work(bankLine, {hand="detail", pile=bankP, tool={kind="round", width=1.2, point=0.5}, angle=0.1, coverage=2.5, fill=true, seed=321})
-- willow reflections, each drawn as its own shape in the water
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x) + 0.5
  local yn = interp(nearBank, x)
  local th = 0.42*s
  local shape = poly({{x-0.1*s, yb},{x+0.1*s, yb},{x+0.12*s, yb+th*0.9},{x+0.42*s, yb+th*1.05},{x+0.46*s, yb+th*1.7},{x-0.46*s, yb+th*1.7},{x-0.42*s, yb+th*1.05},{x-0.12*s, yb+th*0.9}}, true)
  local m = shape * riverM:shrink(0.6)
  work(m, {hand="detail", pile=wRef, tool={kind="round", width=math.max(1,0.07*s), point=0.4}, angle=math.pi/2, coverage=1.8, fill=true, seed=330+i})
end

--@ chunk 52
blend(riverM:shrink(0.3), {tool={kind="badger", width=6}, angle=math.pi/2, coverage=2.5, seed=341})
blend(riverM:shrink(0.3), {tool={kind="badger", width=8}, angle=0.12, coverage=1.0, seed=342})

--@ chunk 53
print(wait(22*60)); print(drying(700,490), drying(700,470), drying(220,430), drying(600,420))

--@ chunk 54
print(wait(24*60)); print(drying(700,490), drying(700,470), drying(220,430), drying(600,420))

--@ chunk 55
rvA = pile{{"lead white",9},{"pale smalt",1},{"yellow ochre",0.25}, medium=0.06}
rvB = pile{{"lead white",7},{"pale smalt",1.5},{"cobalt blue",0.2}, medium=0.06}
rvC = pile{{"lead white",6},{"pale smalt",1.5},{"cobalt blue",0.45}, medium=0.06}
local R = riverM
work(R * rect(320,430,300,60), {hand="body", tool="filbert 3", pile=rvA, angle=0.1, coverage=2.8, fill=true, clip=true, pressure={0.45,0.7}, seed=351})
work(R * rect(600,430,210,100), {hand="body", tool="filbert 3", pile=rvB, angle=0.16, coverage=2.8, fill=true, clip=true, pressure={0.45,0.7}, seed=352})
work(R * rect(790,430,230,140), {hand="body", tool="filbert 4", pile=rvC, angle=0.14, coverage=2.8, fill=true, clip=true, pressure={0.45,0.7}, seed=353})
-- soft willow reflections laid wet into wet with small vertical strokes
local rb = brush{kind="round", width=1.4, point=0.4}
local wRef2 = pile{{"green earth",1.5},{"raw umber",0.7},{"Prussian blue",0.2},{"lead white",0.5}, medium=0.1}
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x) + 1
  local yn = interp(nearBank, x) - 1
  local L = math.min(yn - yb, 0.9*s)
  rb:reload(wRef2, 0.5)
  -- trunk reflection
  rb:stroke({{x, yb},{x, yb + math.min(L, 0.42*s)}}, {pressure={0.7,0.5}, clip=R})
  -- the crown, broader, below it
  local n = math.max(3, math.floor(s/4))
  for k=1,n do
    local xx = x + (k/(n+1)-0.5) * 0.8*s + rand(-0.5,0.5)
    local y0 = yb + 0.42*s*rand(0.9,1.05)
    if y0 < yn then rb:stroke({{xx, y0},{xx, math.min(yn, y0 + rand(0.2,0.45)*s)}}, {pressure={0.6,0.2}, clip=R}) end
  end
end

--@ chunk 56
local R = riverM
local wRef2 = pile{{"green earth",1.5},{"raw umber",0.7},{"Prussian blue",0.2},{"lead white",0.5}, medium=0.1}
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x) + 0.8
  local yn = interp(nearBank, x) - 0.8
  local b = brush{kind="flat", width=math.max(1.2, 0.1*s), stiffness=0.4}
  b:reload(wRef2, 0.35)
  b:stroke({{x, yb},{x, math.min(yn, yb + 0.42*s)}}, {pressure={0.6,0.35}, orient="across", clip=R})
  local cb = brush{kind="filbert", width=math.max(1.5, 0.16*s), stiffness=0.4}
  cb:reload(wRef2, 0.3)
  local n = 3
  for k=1,n do
    local xx = x + (k-2) * 0.22*s
    local y0 = yb + 0.40*s
    if y0 < yn - 1 then cb:stroke({{xx, y0},{xx, math.min(yn, y0 + 0.4*s)}}, {pressure={0.55,0.15}, orient="across", clip=R}) end
  end
end
-- a thin dark line of shadow beneath the far bank
local bb = brush{kind="round", width=1.6, point=0.3}
local bankP2 = pile{{"raw umber",1},{"green earth",1.5},{"Prussian blue",0.2},{"lead white",0.8}, medium=0.15}
local xs = {}
for x=335,1005,45 do
  local seg = {}
  for xx=x, math.min(x+45,1005), 5 do seg[#seg+1] = {xx, interp(farBank, xx) + 0.9} end
  bb:reload(bankP2, 0.35)
  bb:stroke(seg, {pressure={0.35 + 0.0004*(x-335), 0.45 + 0.0004*(x-335)}, clip=R})
end

--@ chunk 57
print(wait(6*60)); print(drying(500,500), drying(800,560), drying(400,470), drying(700,470),drying(700,510))

--@ chunk 58
avoidMid = riverM:grow(0.5) + willowSil:grow(1) + oakM:grow(1) + oakSil:grow(2) + trunkM:grow(1) + riseM
hayP = pile{{"yellow ochre",3},{"lead white",2},{"green earth",0.6},{"chrome yellow",0.15}, medium=0.06}
freshP = pile{{"yellow ochre",2.5},{"Prussian blue",0.3},{"lead white",1.4},{"green earth",0.8}, medium=0.06}
farStripP = pile{{"lead white",3},{"yellow ochre",1.5},{"green earth",1.5},{"pale smalt",0.4}, medium=0.06}
M1 = poly({{350,474},{460,480},{560,490},{660,508},{760,528},{880,552},{1010,578},{1010,612},{880,586},{760,560},{660,538},{560,516},{460,500},{350,490}}, true) - avoidMid
F2 = poly({{430,453},{520,459},{600,465},{700,476},{800,488},{900,498},{1010,508},{1010,520},{900,509},{800,499},{700,486},{600,472},{520,464},{430,457}}, true) - avoidMid
work(F2, {hand="body", tool="filbert 3", pile=farStripP, angle=function(x,y) return 0.1 + 0.0001*(x-430) end, coverage=2.5, fill=true, pressure={0.4,0.6}, edge="soft", seed=361})
work(M1, {hand="body", tool="filbert 5", pile=hayP, angle=function(x,y) return 0.1 + 0.00015*(x-350) end, coverage=2.5, fill=true, pressure={0.4,0.65}, edge="soft", seed=362})

--@ chunk 59
hay2 = pile{{"yellow ochre",1.6},{"lead white",1.6},{"green earth",1.8},{"raw umber",0.25},{"Prussian blue",0.05}, medium=0.06}
work(M1, {hand="body", tool="filbert 5", pile=hay2, angle=function(x,y) return 0.1 + 0.00015*(x-350) end, coverage=1.6, pressure={0.35,0.55}, edge="soft", seed=371})

--@ chunk 60
local n = noise{seed=71, octaves=3, period=14}
local hedge = poly((function()
  local pts = {}
  for x=-5,372,4 do pts[#pts+1] = {x, 458 + 0.012*x - 4 - 3.5*math.max(0,n(x,0)) - 1.5*n(x,50)} end
  for x=372,-5,-4 do pts[#pts+1] = {x, 461 + 0.014*x} end
  return pts
end)(), true)
hedgeM = hedge - (oakM:grow(1) + oakSil:grow(1) + trunkM:grow(1))
hd = pile{{"green earth",1.5},{"Prussian blue",0.35},{"raw umber",0.6},{"lead white",1.2}, medium=0.06}
stipple(hedgeM, {pile=hd, width=2.2, coverage=2.2, pressure={0.4,0.8}, cluster={0.4,3}, feather=0.4, clip=hedgeM:grow(1.2), dips={20,0.6,0.2}, seed=381})
-- a few taller hedgerow trees
for i,t in ipairs({{28,5},{75,6},{340,5}}) do
  local x, r = t[1], t[2]
  local y = 456 + 0.012*x
  local m = ellipse(x, y - r*1.3, r, r*1.3):roughen(1, 3, i) - (oakM:grow(1) + oakSil:grow(1))
  stipple(m, {pile=hd, width=2, coverage=2.5, pressure={0.4,0.8}, clip=m:grow(0.8), seed=390+i})
end

--@ chunk 61
print(drying(220,430), drying(250,470), drying(240,520))

--@ chunk 62
local region = rect(90,380,340,120)
local limbs = (oakM * region) - oakSil:shrink(1)
tb5 = pile{{"raw umber",1.6},{"bone black",0.5},{"lead white",0.9},{"Prussian blue",0.1}, medium=0.04}
work(limbs, {hand="detail", pile=tb5, tool={kind="round", width=2.0, point=0.4}, angle=function(x,y) return oakBody and math.pi/2 or math.pi/2 end, coverage=3.2, fill=true, seed=401})
-- a lighter top edge on the limbs where the sun from the left catches them
local tops = limbs * mask(function(x,y) return 1 end) - limbs:shrink(1.5)
local lb = pile{{"lead white",1.5},{"raw umber",0.8},{"yellow ochre",0.8}, medium=0.04}

--@ chunk 63
print(wait(20*60)); print(drying(240,440), drying(250,470), drying(100,458), drying(600,500))

--@ chunk 64
print(wait(30*60)); print(drying(240,440), drying(250,470), drying(100,458), drying(600,500))

--@ chunk 65
print(wait(36*60)); print(drying(240,440), drying(250,470), drying(100,458), drying(600,500))

--@ chunk 66
for i,l in ipairs(oakLimbs) do local s="" for j,p in ipairs(l) do s=s..string.format("(%d,%d) ",p[1],p[2]) end print(i, s, l.widths and table.concat(l.widths,",")) end

--@ chunk 67
local region = rect(90,380,340,125)
local limbs = (oakM * region) - oakSil:shrink(1)
local barkD = pile{{"raw umber",2},{"bone black",0.45},{"lead white",0.55},{"yellow ochre",0.3},{"Prussian blue",0.08}, medium=0.05}
work(limbs, {hand="detail", pile=barkD, tool={kind="round", width=2.4, point=0.4}, angle=math.pi/2, coverage=3.5, fill=true, pressure={0.6,0.9}, seed=411})

--@ chunk 68
local region = rect(90,380,340,125)
local limbs = (oakM * region) - oakSil:shrink(1)
local barkDD = pile{{"raw umber",2},{"bone black",0.9},{"Prussian blue",0.15},{"lead white",0.15}, medium=0.05}
work(limbs, {hand="hatch", pile=barkDD, tool={kind="round", width=2.2, point=0.4}, angle=math.pi/2, coverage=2.2, pressure={0.6,0.9}, clip=true, seed=421})

--@ chunk 69
local region = rect(90,380,340,150)
local limbs = (oakM * region + trunkM*rect(200,460,100,60)) - oakSil:shrink(1)
local litB = pile{{"lead white",1.8},{"raw umber",0.9},{"yellow ochre",0.9},{"bone black",0.08}, medium=0.05}
local b = brush{kind="round", width=2.0, point=0.6}
local paths = {
  {{226,382},{226,400},{228,420},{230,442}},
  {{224,458},{216,446},{209,432},{203,421}},
  {{252,451},{255,438},{259,424},{262,413}},
  {{271,463},{283,452},{300,442},{317,434},{333,428}},
  {{235,472},{234,490},{234,505},{233,520}},
}
for i,p in ipairs(paths) do
  b:reload(litB, 0.6)
  b:stroke(p, {pressure={0.55,0.35}, ramps={0.1,0.3}, shake=0.4, clip=limbs})
  b:reload(litB, 0.4)
  -- a second broken touch alongside, for bark texture
  local q = {}
  for j,pt in ipairs(p) do q[#q+1] = {pt[1] + rand(1,2), pt[2] + rand(-1,1)} end
  b:stroke(q, {pressure={0.3,0.15}, ramps={0.2,0.4}, shake=0.8, clip=limbs})
end

--@ chunk 70
local n = noise{seed=77, octaves=3, period=22}
local hedge2 = poly((function()
  local pts = {}
  for x=-5,374,3 do pts[#pts+1] = {x, 456 + 0.012*x - 2.5*math.max(0,n(x,0)+0.3) - 1.2*n(x,30)} end
  for x=374,-5,-4 do pts[#pts+1] = {x, 462 + 0.014*x} end
  return pts
end)(), true)
local hm = hedge2 - (oakM:grow(1) + oakSil:grow(1) + trunkM:grow(1))
local hdk = pile{{"green earth",1.5},{"yellow ochre",0.8},{"raw umber",0.7},{"Prussian blue",0.15},{"lead white",1.3}, medium=0.06}
local hlt = pile{{"lead white",2},{"green earth",1},{"yellow ochre",1},{"pale smalt",0.3}, medium=0.06}
stipple(hm, {pile=hdk, width=2.2, coverage=2.5, pressure={0.5,0.9}, cluster={0.4,4}, clip=hm:grow(1), seed=431})
local top = hm * mask(function(x,y) return smoothstep(459+0.013*x, 455+0.013*x, y) end)
stipple(top, {pile=hlt, width=1.6, coverage=0.8, pressure={0.35,0.7}, cluster={0.6,4}, clip=hm, seed=432})

--@ chunk 71
local T = poly({{210,597},{222,591},{228,580},{231,563},{232,540},{233,515},{234,495},{236,476},{262,476},{263,495},{264,515},{265,540},{267,563},{272,580},{280,590},{294,598},{250,601}}, true):roughen(1.2, 8, 51)
trunk2M = T
local barkDD = pile{{"raw umber",2},{"bone black",0.8},{"Prussian blue",0.12},{"lead white",0.35}, medium=0.05}
local barkM = pile{{"raw umber",1.6},{"bone black",0.35},{"lead white",1.0},{"yellow ochre",0.4}, medium=0.05}
local barkL = pile{{"lead white",2},{"raw umber",0.8},{"yellow ochre",0.9},{"bone black",0.08}, medium=0.05}
work(T, {hand="hatch", pile=barkDD, tool={kind="round", width=2.6, point=0.3}, angle=math.pi/2, coverage=3.2, fill=true, pressure={0.6,0.95}, clip=true, seed=441})
-- the lit left side: sun from the upper left, strongest at the left third, broken by the furrows
local litSide = T * mask(function(x,y)
  local l = lerp(236, 222, clamp((y-476)/120,0,1)); local r = lerp(262, 290, clamp((y-476)/120,0,1))
  local t = (x - l)/(r - l)
  return smoothstep(0.55, 0.15, t) end)
work(litSide, {hand="hatch", pile=barkM, tool={kind="round", width=1.8, point=0.4}, angle=math.pi/2, coverage=1.6, pressure={0.5,0.85}, clip=true, broken=0.4, seed=442})
local litEdge = T * mask(function(x,y)
  local l = lerp(236, 222, clamp((y-476)/120,0,1)); local r = lerp(262, 290, clamp((y-476)/120,0,1))
  local t = (x - l)/(r - l)
  return smoothstep(0.3, 0.05, t) end)
work(litEdge, {hand="hatch", pile=barkL, tool={kind="round", width=1.4, point=0.5}, angle=math.pi/2, coverage=1.0, pressure={0.4,0.8}, clip=true, broken=0.5, seed=443})

--@ chunk 72
local T = trunk2M
local ridge = pile{{"raw umber",1.6},{"bone black",0.5},{"lead white",0.7},{"green earth",0.3}, medium=0.05}
local right = T * mask(function(x,y)
  local l = lerp(236, 222, clamp((y-476)/120,0,1)); local r = lerp(262, 290, clamp((y-476)/120,0,1))
  local t = (x - l)/(r - l)
  return smoothstep(0.4, 0.6, t) * smoothstep(1.0, 0.85, t) end)
work(right, {hand="hatch", pile=ridge, tool={kind="round", width=1.2, point=0.5}, angle=math.pi/2+0.05, coverage=0.9, length={6,16}, pressure={0.35,0.7}, clip=true, broken=0.6, seed=451})
-- grass at the foot, over the root flare
local gr = pile{{"yellow ochre",2.5},{"Prussian blue",0.5},{"raw umber",0.4},{"lead white",0.6}, medium=0.06}
local grL = pile{{"yellow ochre",3},{"Prussian blue",0.3},{"lead white",1.5},{"chrome yellow",0.3}, medium=0.06}
local b = brush{kind="rigger", width=1.2, point=1}
for i=1,70 do
  local x = rand(200, 305)
  local y = rand(590, 606)
  local L = rand(4, 11)
  if i % 10 == 1 then b:reload((x<245) and grL or gr, 0.6) end
  local a = -math.pi/2 + rand(-0.35, 0.35)
  b:stroke({{x,y},{x+math.cos(a)*L*0.5+rand(-0.5,0.5), y+math.sin(a)*L*0.5},{x+math.cos(a)*L, y+math.sin(a)*L}}, {pressure={0.6,0.0}})
end

--@ chunk 73
print(drying(600,500), drying(800,560), drying(450,485))

--@ chunk 74
local R = riverM
local rA = pile{{"lead white",9},{"pale smalt",0.9},{"yellow ochre",0.3}, medium=0.07}
local rB = pile{{"lead white",7},{"pale smalt",1.5},{"cobalt blue",0.35},{"yellow ochre",0.05}, medium=0.07}
local rC = pile{{"lead white",5.5},{"pale smalt",1.5},{"cobalt blue",0.7},{"smalt",0.2}, medium=0.07}
-- far part reflects the pale horizon, the near part the bluer sky higher up
work(R * mask(function(x,y) return smoothstep(640,560,x) end), {hand="body", tool="filbert 3", pile=rA, angle=0.12, coverage=2.6, fill=true, clip=true, pressure={0.45,0.7}, seed=461})
work(R * mask(function(x,y) return smoothstep(560,640,x)*smoothstep(860,780,x) end), {hand="body", tool="filbert 3", pile=rB, angle=0.15, coverage=2.6, fill=true, clip=true, pressure={0.45,0.7}, seed=462})
work(R * mask(function(x,y) return smoothstep(780,860,x) end), {hand="body", tool="filbert 4", pile=rC, angle=0.15, coverage=2.6, fill=true, clip=true, pressure={0.45,0.7}, seed=463})
blend(R:shrink(0.5), {tool={kind="badger", width=10}, angle=0.14, coverage=1.2, seed=464})

--@ chunk 75
local R = riverM
local wRef3 = pile{{"green earth",1.5},{"raw umber",0.5},{"Prussian blue",0.25},{"lead white",1.6},{"pale smalt",0.5}, medium=0.08}
local refl = nil
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x)
  local yn = interp(nearBank, x)
  local d = yn - yb
  -- a narrow trunk shadow and a soft, ragged crown reflection, drawn freehand
  local shape = poly({{x-0.07*s, yb},{x+0.07*s, yb},{x+0.1*s, yb+0.33*s},{x+0.36*s, yb+0.42*s},{x+0.3*s, yb+0.75*s},{x+0.05*s, yb+0.95*s},{x-0.08*s, yb+0.9*s},{x-0.34*s, yb+0.72*s},{x-0.38*s, yb+0.44*s},{x-0.1*s, yb+0.33*s}}, true):roughen(0.05*s, 0.2*s, 470+i)
  local m = shape * R:shrink(0.4)
  refl = refl and (refl + m) or m
  local tb = brush{kind="filbert", width=math.max(1.2, 0.12*s), stiffness=0.3}
  local n = math.max(3, math.floor(s/3.5))
  for k=1,n do
    local xx = x + (k/(n+1) - 0.5) * 0.7*s
    tb:reload(wRef3, 0.35)
    tb:stroke({{xx, yb + 0.3*s},{xx + rand(-0.3,0.3), yb + 0.9*s}}, {pressure={0.45,0.1}, ramps={0.1,0.6}, clip=m})
  end
  tb:reload(wRef3, 0.4)
  tb:stroke({{x, yb},{x, yb+0.4*s}}, {pressure={0.5,0.4}, clip=m})
end
reflM = refl
blend(refl:grow(1), {tool={kind="badger", width=5}, angle=math.pi/2, coverage=1.2, seed=479})

--@ chunk 76
local R = riverM
local wRef4 = pile{{"green earth",1.5},{"raw umber",0.7},{"Prussian blue",0.3},{"lead white",0.5}, medium=0.06}
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x)
  local tb = brush{kind="filbert", width=math.max(1.2, 0.1*s), stiffness=0.4}
  local n = math.max(3, math.floor(s/3))
  for k=1,n do
    local xx = x + (k/(n+1) - 0.5) * 0.75*s + rand(-0.3,0.3)
    tb:reload(wRef4, 0.7)
    local top = yb + 0.36*s + rand(-0.03,0.03)*s
    tb:stroke({{xx, top},{xx + rand(-0.2,0.2), top + (0.4 - 0.25*math.abs(k/(n+1)-0.5))*s}}, {pressure={0.75,0.3}, ramps={0.05,0.5}, clip=R:shrink(0.5)})
  end
end

--@ chunk 77
local R = riverM
local zone = nil
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x)
  local e = ellipse(x, yb + 0.55*s, 0.5*s, 0.35*s)
  zone = zone and (zone + e) or e
end
zone = zone * R:shrink(0.3)
blend(zone, {tool={kind="badger", width=4}, angle=math.pi/2, coverage=1.5, seed=481})
blend(zone, {tool={kind="badger", width=4}, angle=0.1, coverage=0.8, seed=482})

--@ chunk 78
local wg = pile{{"lead white",1.5},{"raw umber",1},{"bone black",0.25},{"green earth",0.4}, medium=0.05}
local wgl = pile{{"lead white",2.5},{"raw umber",0.6},{"yellow ochre",0.3},{"green earth",0.3}, medium=0.05}
print(drying(792,490), drying(978,500))
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x) + 0.5
  local th = 0.42*s
  local b = brush{kind="round", width=math.max(1.0, 0.1*s), point=0.3}
  b:reload(wg, 0.7)
  b:stroke({{x+0.02*s, yb}, {x+0.02*s, yb - th*0.55}, {x+0.03*s, yb - th*0.95}}, {pressure={0.8,0.75}})
  local l = brush{kind="round", width=math.max(0.8, 0.045*s), point=0.5}
  l:reload(wgl, 0.6)
  l:stroke({{x-0.04*s, yb-1}, {x-0.045*s, yb - th*0.9}}, {pressure={0.6,0.4}})
end

--@ chunk 79
local wd = pile{{"raw umber",1.2},{"bone black",0.35},{"green earth",0.8},{"lead white",0.5}, medium=0.05}
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x) + 0.5
  local th = 0.42*s
  local b = brush{kind="round", width=math.max(0.9, 0.06*s), point=0.4}
  b:reload(wd, 0.7)
  b:stroke({{x+0.06*s, yb}, {x+0.065*s, yb - th*0.6}, {x+0.07*s, yb - th*0.95}}, {pressure={0.8,0.7}})
  -- the knob where the shoots start, in shadow
  b:reload(wd, 0.6)
  b:stroke({{x-0.08*s, yb - th*0.95}, {x+0.1*s, yb - th*1.0}}, {pressure={0.8,0.6}})
end

--@ chunk 80
local stub = pile{{"lead white",2},{"yellow ochre",1.6},{"green earth",1.4},{"raw umber",0.25},{"pale smalt",0.2}, medium=0.06}
work(M1, {hand="body", tool="filbert 5", pile=stub, angle=function(x,y) return 0.12 + 0.00015*(x-350) end, coverage=2.8, fill=true, pressure={0.45,0.7}, edge="soft", seed=491})

--@ chunk 81
local topP = {{350,474},{460,480},{560,490},{660,508},{760,528},{880,552},{1010,578}}
local botP = {{350,490},{460,500},{560,516},{660,538},{760,560},{880,586},{1010,612}}
function hayY(x, f) return lerp(interp(topP,x), interp(botP,x), f) end
local rowL = pile{{"yellow ochre",2},{"lead white",2.2},{"raw umber",0.3},{"green earth",0.4}, medium=0.05}
local rowD = pile{{"raw umber",1},{"green earth",1.2},{"yellow ochre",0.6},{"lead white",0.6}, medium=0.05}
for r,f in ipairs({0.2, 0.42, 0.64, 0.86}) do
  local x0 = 380 + r*12 + rand(-5,5)
  local x = x0
  while x < 1005 do
    local segL = rand(40, 110)
    local x1 = math.min(1005, x + segL)
    local pts, pts2 = {}, {}
    for xx = x, x1, 6 do
      local y = hayY(xx, f) + rand(-0.4, 0.4)
      pts[#pts+1] = {xx, y}
      pts2[#pts2+1] = {xx + 0.5, y + 0.9 + 0.0015*(xx-350)}
    end
    if #pts >= 2 then
      local w = 1.0 + 0.0035*(x-350)
      local bd = brush{kind="round", width=w*1.1, point=0.3}
      bd:reload(rowD, 0.5)
      bd:stroke(pts2, {pressure={0.5,0.6}, ramps={0.15,0.2}, shake=0.4, clip=M1})
      local bl = brush{kind="round", width=w, point=0.3}
      bl:reload(rowL, 0.6)
      bl:stroke(pts, {pressure={0.55,0.65}, ramps={0.15,0.2}, shake=0.4, clip=M1})
    end
    x = x1 + rand(6, 18)
  end
end

--@ chunk 82
local cL = pile{{"yellow ochre",2},{"lead white",2.5},{"raw umber",0.2},{"green earth",0.3}, medium=0.05}
local cM = pile{{"yellow ochre",2},{"lead white",1},{"raw umber",0.6},{"green earth",0.6}, medium=0.05}
local cD = pile{{"raw umber",1.4},{"green earth",1},{"yellow ochre",0.5},{"bone black",0.15},{"lead white",0.3}, medium=0.05}
local shP = pile{{"green earth",1.5},{"raw umber",0.8},{"Prussian blue",0.2},{"yellow ochre",0.5},{"lead white",0.4}, medium=0.08}
cocks = {}
for i,c in ipairs({{468,0.55},{532,0.3},{606,0.62},{688,0.33},{752,0.72},{842,0.42},{934,0.64}}) do
  local x, f = c[1], c[2]
  local y = hayY(x, f)
  local s = 3.2 + 0.009*(x-350)
  cocks[#cocks+1] = {x, y, s}
  -- cast shadow to the right on the stubble
  local sh = ellipse(x + 0.9*s, y + 0.1*s, 1.2*s, 0.28*s)
  work(sh, {hand="detail", pile=shP, tool={kind="round", width=math.max(1,0.3*s), point=0.3}, angle=0.1, coverage=2.5, fill=true, seed=500+i})
  -- the cock: a rounded cone, lit from the left
  local body = poly({{x-1.0*s, y+0.1*s},{x-0.85*s, y-0.5*s},{x-0.45*s, y-1.05*s},{x, y-1.3*s},{x+0.45*s, y-1.05*s},{x+0.85*s, y-0.5*s},{x+1.0*s, y+0.1*s}}, true):roughen(0.08*s, 0.4*s, 510+i)
  work(body, {hand="detail", pile=cM, tool={kind="round", width=math.max(1,0.28*s), point=0.3}, angle=math.pi/2, coverage=3, fill=true, clip=true, seed=520+i})
  local dark = body * mask(function(px,py) return smoothstep(x-0.05*s, x+0.45*s, px + 0.3*(py-y)) end)
  work(dark, {hand="detail", pile=cD, tool={kind="round", width=math.max(1,0.22*s), point=0.3}, angle=math.pi/2-0.3, coverage=2.5, clip=true, seed=530+i})
  local lit = body * mask(function(px,py) return smoothstep(x-0.1*s, x-0.6*s, px + 0.4*(py-y)) end)
  work(lit, {hand="detail", pile=cL, tool={kind="round", width=math.max(0.9,0.18*s), point=0.4}, angle=math.pi/2+0.4, coverage=2.0, clip=true, seed=540+i})
end

--@ chunk 83
local shirt = pile{{"lead white",4},{"yellow ochre",0.15},{"pale smalt",0.1}, medium=0.04}
local trous = pile{{"raw umber",1},{"bone black",0.4},{"Prussian blue",0.3},{"lead white",0.5}, medium=0.04}
local skirtR = pile{{"red earth",1.5},{"vermilion",0.3},{"lead white",0.4},{"raw umber",0.2}, medium=0.04}
local skirtB = pile{{"Prussian blue",0.6},{"lead white",1},{"raw umber",0.3}, medium=0.04}
local flesh = pile{{"lead white",2},{"red earth",0.5},{"yellow ochre",0.4}, medium=0.04}
local hatP = pile{{"yellow ochre",1.5},{"lead white",1.5},{"raw umber",0.3}, medium=0.04}
local wood = pile{{"raw umber",1},{"yellow ochre",0.8},{"lead white",0.6}, medium=0.04}
function figure(x, y, H, o)
  local lean = o.lean or 0   -- forward bend of the torso, radians; + leans right
  local b = brush{kind="round", width=math.max(0.7, 0.13*H), point=0.4}
  local hipY = y - 0.47*H
  local hx = x
  if o.skirt then
    b:reload(o.skirt, 0.7)
    for k=-1,1 do b:stroke({{hx + k*0.04*H, hipY},{hx + k*0.1*H, y-0.02*H}}, {pressure={0.7,0.9}}) end
  else
    b:reload(trous, 0.7)
    b:stroke({{hx-0.03*H, hipY},{hx-0.07*H - (o.stride or 0), y}}, {pressure={0.8,0.7}})
    b:stroke({{hx+0.03*H, hipY},{hx+0.06*H + (o.stride or 0), y}}, {pressure={0.8,0.7}})
  end
  local sx, sy = hx + math.sin(lean)*0.36*H, hipY - math.cos(lean)*0.36*H
  local tb = brush{kind="round", width=math.max(0.8, 0.17*H), point=0.3}
  tb:reload(o.top or shirt, 0.7)
  tb:stroke({{hx, hipY},{sx, sy}}, {pressure={0.9,0.9}})
  -- head
  local hdx, hdy = sx + math.sin(lean)*0.08*H, sy - math.cos(lean)*0.08*H
  local hb = brush{kind="round", width=math.max(0.7, 0.11*H), point=0.2}
  hb:reload(flesh, 0.6); hb:touch(hdx, hdy, {pressure=0.8})
  hb:reload(o.hat or hatP, 0.6); hb:touch(hdx, hdy - 0.04*H, {pressure=0.7, drag={0.06*H, 0}})
  -- arms and tool
  local ab = brush{kind="round", width=math.max(0.5, 0.07*H), point=0.4}
  ab:reload(o.top or shirt, 0.6)
  local hxA, hyA = o.hand[1], o.hand[2]
  ab:stroke({{sx, sy + 0.03*H},{hxA, hyA}}, {pressure={0.7,0.6}})
  if o.tool then
    local wb = brush{kind="rigger", width=0.6, point=1}
    wb:reload(wood, 0.6)
    wb:stroke({{o.tool[1][1], o.tool[1][2]},{o.tool[2][1], o.tool[2][2]}}, {pressure={0.5,0.4}})
  end
end
-- a man raking near the third cock, leaning to the right
local x, y = 596, hayY(596, 0.7)
local H = 8.5
figure(x, y, H, {lean=0.45, hand={x+4, y-3.2}, tool={{x+1.2, y-5.5},{x+7.5, y+0.3}}, stride=0.4})
-- a woman in a red skirt with a rake, upright, beside him
x, y = 624, hayY(624, 0.5)
figure(x, y, 8.2, {skirt=skirtR, lean=0.05, hand={x-2.2, y-4}, tool={{x-2.8, y-7.5},{x-4, y+0.4}}, hat=shirt})
-- a man pitching hay onto the fifth cock
x, y = 768, hayY(768, 0.74)
figure(x, y, 10.5, {lean=-0.25, hand={x-3.5, y-7}, tool={{x-1, y-2.5},{x-6.5, y-11}}, stride=0.6})
-- a woman in blue further right, walking
x, y = 870, hayY(870, 0.5)
figure(x, y, 11.5, {skirt=skirtB, lean=0.08, hand={x+2.5, y-5}, tool={{x+2.5, y-11},{x+3.2, y+0.2}}, hat=shirt})

--@ chunk 84
function horse(x, y, L, facing, coat, lit, dark, mane, seed)
  local f = facing  -- -1 faces left, 1 faces right
  local function P(u, v) return {x + f*u*L, y + v*L} end
  local body = poly({P(0.52,-0.52), P(0.3,-0.6), P(0.0,-0.58), P(-0.28,-0.62), P(-0.42,-0.58), P(-0.58,-0.36), P(-0.72,-0.1), P(-0.78,0.0), P(-0.7,0.02), P(-0.62,-0.12), P(-0.46,-0.3), P(-0.34,-0.32), P(0.0,-0.33), P(0.36,-0.33), P(0.5,-0.38)}, true)
  local bw = brush{kind="round", width=math.max(1.0, 0.07*L), point=0.3}
  work(body, {hand="detail", pile=coat, tool={kind="round", width=math.max(1,0.08*L), point=0.3}, angle=0.05, coverage=3.5, fill=true, clip=true, seed=seed})
  -- legs
  local lb = brush{kind="round", width=math.max(0.8, 0.055*L), point=0.2}
  for i,u in ipairs({-0.34, -0.26, 0.36, 0.45}) do
    lb:reload((i%2==0) and dark or coat, 0.6)
    local du = (i==1 or i==3) and -0.02 or 0.03
    lb:stroke({P(u, -0.36), P(u+du*0.5, -0.18), P(u+du, 0.0)}, {pressure={0.9,0.7}})
  end
  -- belly and shadow side, then the lit back
  local shade = body * mask(function(px,py) return smoothstep(y-0.44*L, y-0.34*L, py) end)
  work(shade, {hand="detail", pile=dark, tool={kind="round", width=math.max(0.9,0.06*L), point=0.3}, angle=0.05, coverage=2.2, clip=true, seed=seed+1})
  local back = body * mask(function(px,py) return smoothstep(y-0.5*L, y-0.58*L, py) end)
  work(back, {hand="detail", pile=lit, tool={kind="round", width=math.max(0.8,0.05*L), point=0.3}, angle=0.05, coverage=1.8, clip=true, seed=seed+2})
  -- mane and tail
  lb:reload(mane, 0.6)
  lb:stroke({P(-0.3,-0.63), P(-0.45,-0.56), P(-0.58,-0.38)}, {pressure={0.8,0.5}})
  lb:stroke({P(0.52,-0.52), P(0.58,-0.35), P(0.57,-0.15)}, {pressure={0.8,0.3}})
  -- a soft shadow on the grass
  return body
end
local bay = pile{{"red earth",1.5},{"raw umber",1.2},{"yellow ochre",0.3},{"lead white",0.3}, medium=0.05}
local bayL = pile{{"red earth",1},{"yellow ochre",1},{"lead white",1.2},{"raw umber",0.3}, medium=0.05}
local bayD = pile{{"raw umber",1.5},{"bone black",0.5},{"red earth",0.4}, medium=0.05}
local blk = pile{{"bone black",1},{"raw umber",1}, medium=0.05}
local wht = pile{{"lead white",3},{"yellow ochre",0.2},{"raw umber",0.15},{"pale smalt",0.15}, medium=0.05}
local whtL = pile{{"lead white",4},{"yellow ochre",0.25}, medium=0.05}
local whtD = pile{{"lead white",1.5},{"raw umber",0.5},{"pale smalt",0.5},{"bone black",0.1}, medium=0.05}
local sh = pile{{"green earth",1.5},{"raw umber",0.8},{"Prussian blue",0.3},{"yellow ochre",0.6},{"lead white",0.3}, medium=0.08}
-- shadows on the grass first, to the right of each horse
work(ellipse(452, 552, 11, 1.8), {hand="detail", pile=sh, tool={kind="round", width=1.5, point=0.3}, angle=0.05, coverage=2.5, fill=true, seed=601})
work(ellipse(528, 566, 12, 2), {hand="detail", pile=sh, tool={kind="round", width=1.5, point=0.3}, angle=0.05, coverage=2.5, fill=true, seed=602})
horse(440, 552, 17, -1, bay, bayL, bayD, blk, 610)
horse(515, 566, 18, 1, wht, whtL, whtD, whtD, 620)

--@ chunk 85
local mP = pile{{"lead white",2},{"smalt",1.5},{"raw umber",0.7},{"red earth",0.3},{"bone black",0.08}, medium=0.05}
local mL = pile{{"lead white",3.5},{"smalt",0.8},{"raw umber",0.3},{"yellow ochre",0.3}, medium=0.05}
function postmill(x, base, H, rot, seed)
  local bw = 0.3*H
  local bTop, bBot = base - H*0.95, base - H*0.45
  -- trestle and post
  local r = brush{kind="rigger", width=0.7, point=1}
  r:reload(mP, 0.6)
  r:stroke({{x, bBot},{x-0.18*H, base}}, {pressure={0.6,0.6}})
  r:stroke({{x, bBot},{x+0.18*H, base}}, {pressure={0.6,0.6}})
  r:stroke({{x, bBot},{x, base}}, {pressure={0.7,0.7}})
  -- the body with its gabled roof
  local body = poly({{x-bw/2, bBot},{x-bw/2, bTop+0.08*H},{x, bTop-0.02*H},{x+bw/2, bTop+0.08*H},{x+bw/2, bBot}})
  work(body, {hand="detail", pile=mP, tool={kind="round", width=1, point=0.2}, angle=math.pi/2, coverage=3, fill=true, clip=true, seed=seed})
  work(body * rect(x-bw/2, bTop-2, bw*0.4, H), {hand="detail", pile=mL, tool={kind="round", width=0.8, point=0.2}, angle=math.pi/2, coverage=1.5, clip=true, seed=seed+1})
  -- four sails from the hub on the front of the body
  local hx, hy = x - 0.05*H, bTop + 0.15*H
  local R = 0.62*H
  for k=0,3 do
    local a = rot + k*math.pi/2
    local ca, sa = math.cos(a), math.sin(a)
    r:reload(mP, 0.6)
    r:stroke({{hx, hy},{hx + ca*R, hy + sa*R}}, {pressure={0.55,0.45}})
    -- the lattice frame as a narrow blade along the outer part
    local px, py = -sa, ca
    local w = 0.09*H
    local blade = poly({{hx+ca*R*0.25, hy+sa*R*0.25},{hx+ca*R, hy+sa*R},{hx+ca*R+px*w, hy+sa*R+py*w},{hx+ca*R*0.25+px*w, hy+sa*R*0.25+py*w}})
    work(blade, {hand="detail", pile=mP, tool={kind="round", width=0.7, point=0.3}, angle=a, coverage=1.3, clip=true, seed=seed+2+k})
  end
end
postmill(488, 427, 17, 0.35, 700)
postmill(846, 431, 14, 1.05, 710)

--@ chunk 86
print(riseY(320), riseY(335), riseY(350), riseY(365))

--@ chunk 87
-- two figures seen from behind on the crest, looking out toward the town
local coat = pile{{"Prussian blue",0.5},{"raw umber",1.2},{"bone black",0.6},{"green earth",0.4},{"lead white",0.15}, medium=0.05}
local coatL = pile{{"Prussian blue",0.4},{"raw umber",1},{"green earth",0.6},{"lead white",0.8},{"bone black",0.2}, medium=0.05}
local trs = pile{{"lead white",1.5},{"raw umber",0.8},{"yellow ochre",0.6},{"bone black",0.1}, medium=0.05}
local boot = pile{{"bone black",1},{"raw umber",0.8}, medium=0.05}
local dress = pile{{"red earth",1.3},{"raw umber",0.6},{"vermilion",0.2},{"bone black",0.15},{"lead white",0.2}, medium=0.05}
local dressL = pile{{"red earth",1.2},{"vermilion",0.3},{"lead white",0.6},{"yellow ochre",0.2}, medium=0.05}
local shawl = pile{{"lead white",1.2},{"yellow ochre",0.5},{"raw umber",0.4},{"pale smalt",0.15}, medium=0.05}
local hair = pile{{"raw umber",1.5},{"red earth",0.3},{"bone black",0.3},{"lead white",0.2}, medium=0.05}
local skin = pile{{"lead white",2},{"red earth",0.45},{"yellow ochre",0.4}, medium=0.05}
local gsh = pile{{"green earth",1.5},{"raw umber",0.7},{"Prussian blue",0.35},{"yellow ochre",0.8},{"lead white",0.2}, medium=0.08}

local mx, my = 328, 573   -- man's feet
local function M(u, v) return {mx + u, my - v} end
local wx, wy = 344, 577   -- woman's feet
local function W(u, v) return {wx + u, wy - v} end

-- cast shadows on the grass, falling to the right
work(poly({M(-3,0), M(4,-0.5), M(22,-3.5), M(24,-1.5), M(8,1)}, true), {hand="detail", pile=gsh, tool={kind="round", width=1.6, point=0.3}, angle=0.1, coverage=2.5, fill=true, seed=801})
work(poly({W(-6,0), W(8,-0.5), W(28,-3.5), W(30,-1), W(10,1.5)}, true), {hand="detail", pile=gsh, tool={kind="round", width=1.6, point=0.3}, angle=0.1, coverage=2.5, fill=true, seed=802})

-- the man: legs, then the long coat, then the head and cap
local legs = poly({M(-3.6,15.5), M(-3.3,1), M(-0.9,1), M(-0.4,15.5)}) + poly({M(0.6,15.5), M(1.0,1), M(3.4,1), M(3.8,15.5)})
work(legs, {hand="detail", pile=trs, tool={kind="round", width=1.2, point=0.3}, angle=math.pi/2, coverage=3, fill=true, clip=true, seed=803})
local boots = rect(mx-3.8, my-4, 3.2, 4.2) + rect(mx+0.8, my-4, 3.0, 4.2)
work(boots, {hand="detail", pile=boot, tool={kind="round", width=1.0, point=0.3}, angle=0, coverage=3, fill=true, clip=true, seed=804})
local coatM = poly({M(-2.2,43), M(-5.5,41.8), M(-6.8,39.5), M(-7.4,30), M(-7.8,22), M(-7.2,14.5), M(0,14), M(7.2,14.5), M(7.6,22), M(7.2,30), M(6.8,39.5), M(5.5,41.8), M(2.2,43)}, true)
work(coatM, {hand="detail", pile=coat, tool={kind="round", width=1.8, point=0.3}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, seed=805})
-- lit left side of the coat and the arm
local coatLit = coatM * mask(function(x,y) return smoothstep(mx-2, mx-5.5, x) end)
work(coatLit, {hand="detail", pile=coatL, tool={kind="round", width=1.2, point=0.3}, angle=math.pi/2, coverage=1.8, clip=true, seed=806})
-- the arms' seams and the coat's centre vent, in a darker line
local r = brush{kind="rigger", width=0.8, point=1}
r:reload(boot, 0.5)
r:stroke({M(-5.2,38), M(-5.6,30), M(-5.2,23)}, {pressure={0.4,0.3}})
r:stroke({M(5.2,38), M(5.6,30), M(5.2,23)}, {pressure={0.4,0.3}})
r:stroke({M(0.2,30), M(0.1,14.5)}, {pressure={0.35,0.4}})
-- neck, head and the dark cap
local neck = ellipse(mx, my-44, 1.8, 1.4)
work(neck, {hand="detail", pile=skin, tool={kind="round", width=1, point=0.2}, angle=0, coverage=3, fill=true, clip=true, seed=807})
local head = ellipse(mx+0.2, my-47, 3.0, 3.5)
work(head, {hand="detail", pile=hair, tool={kind="round", width=1.2, point=0.3}, angle=math.pi/2, coverage=3, fill=true, clip=true, seed=808})
local cap = ellipse(mx+0.4, my-50, 4.1, 1.9)
work(cap, {hand="detail", pile=boot, tool={kind="round", width=1.2, point=0.3}, angle=0, coverage=3, fill=true, clip=true, seed=809})

-- the woman: a long dress, a pale shawl over the shoulders, hair gathered up
local dressM = poly({W(-3.4,31), W(-4.2,24), W(-6.2,12), W(-8.2,1.5), W(-7.5,0), W(7.8,0), W(8.6,1.5), W(6.6,12), W(4.6,24), W(3.8,31)}, true)
work(dressM, {hand="detail", pile=dress, tool={kind="round", width=1.8, point=0.3}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, seed=811})
local dressLit = dressM * mask(function(x,y) return smoothstep(wx-1, wx-5.5, x) end)
work(dressLit, {hand="detail", pile=dressL, tool={kind="round", width=1.2, point=0.3}, angle=math.pi/2, coverage=1.6, clip=true, seed=812})
r:reload(boot, 0.4)
for k,u in ipairs({-3.5, -0.8, 2.2, 5.0}) do r:stroke({W(u*0.55,24), W(u,1)}, {pressure={0.2,0.35}}) end
local shawlM = poly({W(-2,39.5), W(-5.6,37.5), W(-6.4,33), W(-5.2,27), W(0,24.5), W(5.4,27), W(6.4,33), W(5.6,37.5), W(2,39.5)}, true)
work(shawlM, {hand="detail", pile=shawl, tool={kind="round", width=1.3, point=0.3}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, seed=813})
local shawlSh = shawlM * mask(function(x,y) return smoothstep(wx+1, wx+5, x) end)
work(shawlSh, {hand="detail", pile=pile{{"lead white",1},{"raw umber",0.6},{"pale smalt",0.4},{"yellow ochre",0.2}, medium=0.05}, tool={kind="round", width=1, point=0.3}, angle=math.pi/2, coverage=2, clip=true, seed=814})
local neckW = ellipse(wx, wy-40.2, 1.5, 1.2)
work(neckW, {hand="detail", pile=skin, tool={kind="round", width=0.9, point=0.2}, angle=0, coverage=3, fill=true, clip=true, seed=815})
local headW = ellipse(wx-0.2, wy-43.2, 2.7, 3.1)
work(headW, {hand="detail", pile=hair, tool={kind="round", width=1.1, point=0.3}, angle=math.pi/2, coverage=3, fill=true, clip=true, seed=816})
local bun = ellipse(wx-0.2, wy-46.2, 1.7, 1.4)
work(bun, {hand="detail", pile=hair, tool={kind="round", width=1, point=0.3}, angle=0, coverage=3, fill=true, clip=true, seed=817})

--@ chunk 88
print(drying(760,300), drying(850,350), drying(900,390), drying(500,660), drying(300,600))

--@ chunk 89
local parts = {}
local function B(x,y,r,z,sd) parts[#parts+1] = {body.ellipsoid({x,y,z or 0},{r*1.12, r*0.88, r}):rough(r*0.1, r*0.8, sd)} end
B(598,384,30,0,1) B(648,356,40,0,2) B(700,320,46,5,3) B(752,290,50,0,4) B(810,294,48,-5,5) B(862,318,44,0,6)
B(912,338,40,0,7) B(958,350,36,0,8) B(1002,360,34,0,9)
B(742,258,28,10,10) B(792,260,30,5,11) B(690,302,26,15,12) B(838,278,26,5,13)
B(655,392,36,25,14) B(730,374,50,30,15) B(810,370,56,30,16) B(890,380,48,25,17) B(965,390,40,20,18)
parts.light = {from={-1,-0.8}, front=0.35, ambient=0.25}
cloud2F = form(parts)
local town = (townM + townM2):grow(0.6)
local cut = rect(0,0,1000,413)
cloud2M = ((cloud2F:silhouette{} + cloudM) * cut) - town
cloud2Lit = cloud2F:lit{soft=0.25} * cut - town
local csh = pile{{"lead white",7},{"pale smalt",1.4},{"red earth",0.22},{"raw umber",0.12},{"yellow ochre",0.2}, medium=0.08}
local cmid = pile{{"lead white",10},{"pale smalt",0.7},{"yellow ochre",0.35},{"red earth",0.08}, medium=0.08}
local chi = pile{{"lead white",14},{"yellow ochre",0.3},{"vermilion",0.03}, medium=0.06}
work(cloud2M, {hand="body", tool="filbert 8", pile=csh, angle=function(x,y) return 0.3 end, coverage=2.6, fill=true, edge="soft", pressure={0.45,0.7}, curve={0.3,0.1}, seed=901})
work(cloud2Lit, {hand="body", tool="filbert 7", pile=cmid, angle=cloud2F:field("across"), coverage=2.2, edge="soft", pressure={0.4,0.65}, seed=902})
local hi = cloud2Lit:map(function(v) return smoothstep(0.55, 0.9, v) end)
work(hi, {hand="body", tool="filbert 5", pile=chi, angle=cloud2F:field("across"), coverage=1.8, edge="soft", pressure={0.4,0.6}, seed=903})

--@ chunk 90
local town = (townM + townM2):grow(3)
local cut = rect(0,0,1000,413)
local shd = cloud2F:shadow{} * cut - town
local csd = pile{{"lead white",5},{"pale smalt",1.5},{"smalt",0.25},{"red earth",0.3},{"raw umber",0.25},{"yellow ochre",0.1}, medium=0.08}
work(shd, {hand="body", tool="filbert 6", pile=csd, angle=cloud2F:field("across"), coverage=2.0, clip=true, pressure={0.45,0.7}, seed=911})
-- the base in shadow: a band along the lower part of the cloud
local base = cloud2M * mask(function(x,y) return smoothstep(365, 400, y) end) - town
work(base, {hand="body", tool="filbert 6", pile=csd, angle=0.05, coverage=1.8, clip=true, pressure={0.4,0.65}, seed=912})

--@ chunk 91
local town = (townM + townM2):grow(0.8)
local m = (cloud2M:shrink(1)) - town
blend(m, {tool={kind="badger", width=14}, angle=cloud2F:field("across"), coverage=1.6, seed=921})
blend(m, {tool={kind="badger", width=10}, angle=-0.5, coverage=1.0, seed=922})

--@ chunk 92
local figs = rect(318,515,40,64)   -- keep the figures clear
local keep = trunk2M:grow(1.5) + figs
local R = riseM - keep
-- texture of grass over the rise: short upright hatching in three greens, lighter on the crest
local gL = pile{{"yellow ochre",3},{"Prussian blue",0.28},{"lead white",1.6},{"chrome yellow",0.35}, medium=0.05}
local gM = pile{{"yellow ochre",2.5},{"Prussian blue",0.5},{"raw umber",0.4},{"lead white",0.7}, medium=0.05}
local gD = pile{{"raw umber",1.2},{"Prussian blue",0.6},{"yellow ochre",1.2},{"green earth",0.6},{"bone black",0.08}, medium=0.05}
local crest = R * mask(function(x,y) local c = riseY(x); return smoothstep(c+45, c+15, y) end)
local slope = R * mask(function(x,y) local c = riseY(x); return smoothstep(c+10, c+40, y) * smoothstep(c+95, c+60, y) end)
local foot = R * mask(function(x,y) local c = riseY(x); return smoothstep(c+55, c+90, y) end)
work(crest, {hand="hatch", pile=gL, tool={kind="round", width=1.6, point=0.6}, angle=-math.pi/2+0.15, length={4,9}, coverage=1.4, pressure={0.3,0.7}, seed=931})
work(slope, {hand="hatch", pile=gM, tool={kind="round", width=2.0, point=0.6}, angle=-math.pi/2+0.2, length={5,12}, coverage=1.3, pressure={0.3,0.7}, seed=932})
work(foot, {hand="hatch", pile=gD, tool={kind="round", width=2.6, point=0.6}, angle=-math.pi/2+0.2, length={8,18}, coverage=1.2, pressure={0.3,0.75}, seed=933})

--@ chunk 93
local gL = pile{{"yellow ochre",3},{"Prussian blue",0.28},{"lead white",1.6},{"chrome yellow",0.35}, medium=0.05}
local gM = pile{{"yellow ochre",2.5},{"Prussian blue",0.45},{"raw umber",0.3},{"lead white",1.0}, medium=0.05}
local b = brush{kind="rigger", width=1.1, point=1}
local keep = trunk2M:grow(1) + rect(318,500,40,80)
local n = 0
for x = -3, 1000, 1.3 do
  local c = riseY(x)
  if keep:at(x, c) < 0.5 then
    n = n + 1
    if n % 12 == 1 then b:reload((math.random() < 0.7) and gL or gM, 0.6) end
    local y0 = c + rand(1, 4)
    local L = rand(2.5, 7) * (1 + 0.4*math.max(0, (c-560)/100))
    local a = -math.pi/2 + rand(-0.4, 0.45)
    b:stroke({{x, y0},{x + math.cos(a)*L*0.5, y0 + math.sin(a)*L*0.5},{x + math.cos(a)*L + rand(-0.6,0.6), y0 + math.sin(a)*L}}, {pressure={0.55, 0.0}, clip=-keep})
  end
end

--@ chunk 94
local shG = pile{{"yellow ochre",2},{"Prussian blue",0.55},{"raw umber",0.55},{"green earth",0.5},{"lead white",0.35}, medium=0.06}
local keep = trunk2M:grow(0.5) + rect(318,500,40,80)
-- the crown's shadow lying on the slope to the right of the trunk, broken by light through the leaves
local sh = poly({{262,598},{300,594},{350,596},{400,604},{440,614},{452,626},{420,632},{360,630},{300,622},{250,612},{226,604}}, true):roughen(4, 16, 941) - keep
work(sh, {hand="hatch", pile=shG, tool={kind="round", width=1.8, point=0.6}, angle=-math.pi/2+0.2, length={4,10}, coverage=2.2, pressure={0.35,0.75}, edge="soft", broken=0.3, seed=942})
-- grass over the foot of the trunk
local gM = pile{{"yellow ochre",2.5},{"Prussian blue",0.45},{"raw umber",0.3},{"lead white",1.0}, medium=0.05}
local gL = pile{{"yellow ochre",3},{"Prussian blue",0.28},{"lead white",1.6},{"chrome yellow",0.35}, medium=0.05}
local b = brush{kind="rigger", width=1.2, point=1}
for i=1,110 do
  local x = rand(206, 300)
  local y = rand(593, 606)
  if i % 10 == 1 then b:reload((x < 240) and gL or gM, 0.6) end
  local L = rand(4, 10)
  local a = -math.pi/2 + rand(-0.4, 0.4)
  b:stroke({{x,y},{x+math.cos(a)*L*0.5, y+math.sin(a)*L*0.5},{x+math.cos(a)*L+rand(-0.7,0.7), y+math.sin(a)*L}}, {pressure={0.6,0.0}})
end

--@ chunk 95
local gM = pile{{"yellow ochre",2.5},{"Prussian blue",0.5},{"raw umber",0.4},{"lead white",0.8}, medium=0.05}
local gL = pile{{"yellow ochre",3},{"Prussian blue",0.28},{"lead white",1.6},{"chrome yellow",0.35}, medium=0.05}
local b = brush{kind="round", width=1.8, point=0.8}
for i=1,90 do
  local x = rand(208, 298)
  local y = rand(596, 604)
  if i % 8 == 1 then b:reload((i % 16 == 1) and gL or gM, 0.7) end
  local L = rand(5, 11)
  local a = -math.pi/2 + rand(-0.35, 0.35)
  b:stroke({{x,y},{x+math.cos(a)*L*0.5, y+math.sin(a)*L*0.5},{x+math.cos(a)*L+rand(-0.7,0.7), y+math.sin(a)*L}}, {pressure={0.8,0.0}, ramps={0.05,0.7}})
end

--@ chunk 96
local st = body.ellipsoid({108,684,0},{40,24,30}):rough(3, 20, 961):cut({108,700,0},{0,1,0}, 1, 2)
local st2 = body.ellipsoid({166,694,0},{17,11,14}):rough(1.5, 10, 962)
local stF = form{ {st}, {st2}, light={from={-1,-0.9}, front=0.3, ambient=0.2} }
stoneM = stF:silhouette{} * rect(0,0,1000,703)
local sD = pile{{"lead white",1},{"raw umber",1},{"bone black",0.4},{"Prussian blue",0.1}, medium=0.05}
local sM = pile{{"lead white",2.2},{"raw umber",0.8},{"bone black",0.2},{"yellow ochre",0.3},{"red earth",0.1}, medium=0.05}
local sL = pile{{"lead white",3.5},{"yellow ochre",0.4},{"raw umber",0.3},{"red earth",0.05}, medium=0.05}
work(stoneM, {hand="body", tool="filbert 3", pile=sD, angle=stF:field("across"), coverage=3, fill=true, clip=true, seed=963})
work(stF:lit{soft=0.3} * stoneM, {hand="body", tool="filbert 3", pile=sM, angle=stF:field("across"), coverage=2.2, clip=true, seed=964})
local hi = (stF:lit{soft=0.2} * stoneM):map(function(v) return smoothstep(0.5, 0.85, v) end)
work(hi, {hand="detail", pile=sL, tool={kind="round", width=2.0, point=0.3}, angle=stF:field("across"), coverage=1.6, clip=true, seed=965})

--@ chunk 97
local sD2 = pile{{"lead white",0.8},{"raw umber",1},{"bone black",0.45},{"Prussian blue",0.15},{"green earth",0.3}, medium=0.05}
local lower = stoneM * mask(function(x,y) return smoothstep(-0.1, 0.5, (x-95)/60 + (y-680)/25) end)
work(lower, {hand="body", tool="filbert 3", pile=sD2, angle=0.3, coverage=2.2, clip=true, pressure={0.4,0.7}, seed=971})
blend(stoneM:shrink(1), {tool={kind="badger", width=6}, angle=-0.6, coverage=1.0, seed=972})
-- lichen speckles and a few cracks
local lic = pile{{"lead white",2},{"yellow ochre",1},{"green earth",0.5}, medium=0.05}
stipple(stoneM:shrink(2) * rect(70,660,80,25), {pile=lic, width=1.0, coverage=0.25, pressure={0.3,0.6}, cluster={0.6,3}, seed=973})
local r = brush{kind="rigger", width=0.8, point=1}
r:reload(sD2, 0.5)
r:stroke({{112,666},{116,674},{113,684}}, {pressure={0.4,0.2}})
r:stroke({{150,672},{140,680},{142,690}}, {pressure={0.3,0.1}})
-- grass growing up in front of the stones
local gD = pile{{"raw umber",1.2},{"Prussian blue",0.6},{"yellow ochre",1.2},{"green earth",0.6},{"bone black",0.08}, medium=0.05}
local gM = pile{{"yellow ochre",2.5},{"Prussian blue",0.5},{"raw umber",0.4},{"lead white",0.8}, medium=0.05}
local b = brush{kind="round", width=2.0, point=0.8}
for i=1,80 do
  local x = rand(62, 188)
  local y = rand(698, 716)
  if i % 8 == 1 then b:reload((i%24==1) and gM or gD, 0.7) end
  local L = rand(7, 18)
  local a = -math.pi/2 + rand(-0.35, 0.35)
  b:stroke({{x,y},{x+math.cos(a)*L*0.5, y+math.sin(a)*L*0.5},{x+math.cos(a)*L+rand(-1,1), y+math.sin(a)*L}}, {pressure={0.8,0.0}, ramps={0.05,0.7}})
end

--@ chunk 98
local botP = {{350,490},{460,500},{560,516},{660,538},{760,560},{880,586},{1010,612}}
local keep = trunk2M:grow(1) + rect(316,510,44,72) + rect(420,530,40,28) + rect(496,548,46,24) + oakSil:grow(1) + oakM:grow(1)
local meadow = (below(botP) - riseM - keep) * rect(0,470,1000,250)
local mA = pile{{"yellow ochre",2.4},{"Prussian blue",0.32},{"lead white",1.4},{"green earth",0.6}, medium=0.05}
local mB = pile{{"yellow ochre",2.4},{"Prussian blue",0.45},{"raw umber",0.3},{"lead white",0.9},{"green earth",0.4}, medium=0.05}
-- small far strokes near the hay, larger ones nearer
work(meadow * mask(function(x,y) return smoothstep(interp(botP,x)+45, interp(botP,x)+15, y) end), {hand="hatch", pile=mA, tool={kind="round", width=1.2, point=0.6}, angle=-math.pi/2+0.1, length={2,5}, coverage=1.0, pressure={0.3,0.6}, seed=981})
work(meadow * mask(function(x,y) return smoothstep(interp(botP,x)+15, interp(botP,x)+45, y) end), {hand="hatch", pile=mB, tool={kind="round", width=1.5, point=0.6}, angle=-math.pi/2+0.15, length={3,8}, coverage=1.1, pressure={0.3,0.65}, seed=982})

--@ chunk 99
local mB = pile{{"yellow ochre",2.4},{"Prussian blue",0.45},{"raw umber",0.3},{"lead white",0.9},{"green earth",0.4}, medium=0.05}
work(poly({{765,672},{830,676},{900,688},{925,700},{925,714},{765,714}}) - riseM:shrink(2), {hand="hatch", pile=mB, tool={kind="round", width=1.8, point=0.6}, angle=-math.pi/2+0.15, length={5,10}, coverage=2.2, pressure={0.4,0.75}, seed=991})
-- a drainage ditch across the meadow, catching the sky
local dpts = {{470,577},{540,584},{600,591},{680,601},{750,611},{830,622},{900,633},{1005,648}}
local wd = {}
for i,p in ipairs(dpts) do wd[i] = 1.2 + 0.0045*(p[1]-470) end
ditchM = ribbon(dpts, wd) - riseM
local keep2 = rect(496,548,46,24)
ditchM = ditchM - keep2
local dk = pile{{"raw umber",1},{"green earth",1.2},{"Prussian blue",0.3},{"lead white",0.3}, medium=0.05}
local dw = pile{{"lead white",6},{"pale smalt",1.2},{"cobalt blue",0.25},{"yellow ochre",0.1}, medium=0.05}
work(ditchM, {hand="detail", pile=dw, tool={kind="round", width=1.3, point=0.3}, angle=0.13, coverage=3, fill=true, clip=true, seed=992})
-- the shadowed far bank above the water
local b = brush{kind="round", width=1.2, point=0.4}
for i=1,#dpts-1 do
  b:reload(dk, 0.5)
  local a, c = dpts[i], dpts[i+1]
  local wa, wc = wd[i], wd[i+1]
  b:stroke({{a[1], a[2]-wa/2-0.2},{c[1], c[2]-wc/2-0.2}}, {pressure={0.35+0.001*(a[1]-470), 0.35+0.001*(c[1]-470)}, clip=-riseM})
end

--@ chunk 100
local mB = pile{{"yellow ochre",2.4},{"Prussian blue",0.45},{"raw umber",0.3},{"lead white",0.9},{"green earth",0.4}, medium=0.05}
local mD = pile{{"yellow ochre",2},{"Prussian blue",0.55},{"raw umber",0.5},{"lead white",0.5},{"green earth",0.5}, medium=0.05}
-- grass of the near bank growing up over the water, breaking the line
local band = ditchM:grow(2.5) * mask(function(x,y) return 1 end)
local n = noise{seed=1001, octaves=2, period=25}
work(band, {hand="hatch", pile=mB, tool={kind="round", width=1.4, point=0.7}, angle=-math.pi/2+0.1, length={3,7}, coverage=1.1, broken=0.4, pressure={0.35,0.7}, seed=1002})
-- a line of reeds and a few willow stakes along the ditch
local b = brush{kind="rigger", width=1, point=1}
for i=1,40 do
  local x = rand(560, 1000)
  local y = interp({{470,577},{540,584},{600,591},{680,601},{750,611},{830,622},{900,633},{1005,648}}, x) - 1
  if i % 8 == 1 then b:reload(mD, 0.6) end
  local L = 3 + 0.012*(x-470)
  b:stroke({{x,y},{x+rand(-0.8,0.8), y-L}}, {pressure={0.55,0.0}})
end

--@ chunk 101
print(wait(24*60)); print(drying(780,300), drying(900,380), drying(700,600), drying(110,680), drying(600,395))

--@ chunk 102
local mB = pile{{"yellow ochre",2.4},{"Prussian blue",0.45},{"raw umber",0.3},{"lead white",0.9},{"green earth",0.4}, medium=0.05}
local mBd = pile{{"yellow ochre",2.2},{"Prussian blue",0.5},{"raw umber",0.4},{"lead white",0.7},{"green earth",0.5}, medium=0.05}
local n = noise{seed=1011, octaves=2, period=30}
-- cover most of the ditch with the meadow; leave glints where the water shows between the grass
local cover = ditchM:grow(0.8) * mask(function(x,y) return smoothstep(-0.25, 0.1, n(x,y)) end)
work(cover, {hand="hatch", pile=mBd, tool={kind="round", width=1.3, point=0.6}, angle=-math.pi/2+0.1, length={2,6}, coverage=2.4, pressure={0.4,0.75}, seed=1012})
-- a quiet glaze of grey-green over the remaining water so it sits back in the meadow
local gz = pile{{"green earth",1},{"pale smalt",0.5},{"lead white",0.5},{"raw umber",0.2}, medium=0.5}
work(ditchM, {hand="detail", pile=gz, tool={kind="round", width=1.5, point=0.2}, angle=0.13, coverage=1.2, clip=true, pressure={0.3,0.5}, seed=1013})

--@ chunk 103
local town = (townM + townM2):grow(0.3)
local cut = rect(0,0,1000,413)
local shd = (cloud2F:shadow{} * cut - town):soften(4)
local gl = pile{{"lead white",2},{"pale smalt",1.5},{"smalt",0.3},{"red earth",0.3},{"raw umber",0.3}, medium=0.6}
work(shd * cloud2M, {hand="glaze", pile=gl, tool={kind="filbert", width=12, stiffness=0.2}, angle=cloud2F:field("across"), coverage=1.4, clip=true, seed=1031})
local belly = cloud2M * mask(function(x,y) return smoothstep(340, 395, y) * smoothstep(600, 700, x) end) - town
work(belly, {hand="glaze", pile=gl, tool={kind="filbert", width=14, stiffness=0.2}, angle=0.05, coverage=1.0, clip=true, seed=1032})

--@ chunk 104
local town = (townM + townM2):grow(0.5)
local m = cloud2M:shrink(2) - town
blend(m, {tool={kind="badger", width=16}, angle=cloud2F:field("across"), coverage=1.8, seed=1041})
blend(m, {tool={kind="badger", width=12}, angle=0.4, coverage=1.2, seed=1042})

--@ chunk 105
local town = (townM + townM2):grow(0.5)
local cut = rect(0,0,1000,413)
local lit = (cloud2F:lit{soft=0.18} * cut - town)
local heads = lit:map(function(v) return smoothstep(0.62, 0.92, v) end) * cloud2M:shrink(1)
local warm = pile{{"lead white",14},{"yellow ochre",0.35},{"vermilion",0.04}, medium=0.04}
local half = pile{{"lead white",10},{"pale smalt",0.5},{"yellow ochre",0.35},{"red earth",0.08}, medium=0.05}
local mid = lit:map(function(v) return smoothstep(0.35, 0.6, v) * smoothstep(0.85, 0.6, v) end) * cloud2M:shrink(1)
work(mid, {hand="body", tool="filbert 6", pile=half, angle=cloud2F:field("across"), coverage=1.2, clip=true, pressure={0.35,0.6}, seed=1051})
work(heads, {hand="body", tool="filbert 5", pile=warm, angle=cloud2F:field("across"), coverage=2.0, clip=true, pressure={0.45,0.7}, load=0.9, seed=1052})

--@ chunk 106
local town = (townM + townM2):grow(0.5)
local m = cloud2M:shrink(1.5) - town
blend(m, {tool={kind="badger", width=9}, angle=cloud2F:field("across"), coverage=0.9, pressure={0.2,0.4}, seed=1061})

--@ chunk 107
function horseMask(x, y, L, f)
  local function P(u, v) return {x + f*u*L, y + v*L} end
  local m = poly({P(0.52,-0.52), P(0.3,-0.6), P(0.0,-0.58), P(-0.28,-0.62), P(-0.42,-0.58), P(-0.58,-0.36), P(-0.72,-0.1), P(-0.78,0.0), P(-0.7,0.02), P(-0.62,-0.12), P(-0.46,-0.3), P(-0.34,-0.32), P(0.0,-0.33), P(0.36,-0.33), P(0.5,-0.38)}, true)
  for i,u in ipairs({-0.34, -0.26, 0.36, 0.45}) do
    local du = (i==1 or i==3) and -0.02 or 0.03
    m = m + ribbon({P(u, -0.36), P(u+du*0.5, -0.18), P(u+du, 0.0)}, 0.06*L)
  end
  m = m + ribbon({P(0.52,-0.52), P(0.58,-0.35), P(0.57,-0.15)}, 0.06*L)
  return m:grow(0.8)
end
local mx, my, wx, wy = 328, 573, 344, 577
local function M(u, v) return {mx + u, my - v} end
local function W(u, v) return {wx + u, wy - v} end
figM = (poly({M(-3.6,15.5), M(-3.3,-0.5), M(-0.9,-0.5), M(-0.4,15.5)}) + poly({M(0.6,15.5), M(1.0,-0.5), M(3.4,-0.5), M(3.8,15.5)})
  + poly({M(-2.2,43), M(-5.5,41.8), M(-6.8,39.5), M(-7.4,30), M(-7.8,22), M(-7.2,14.5), M(0,14), M(7.2,14.5), M(7.6,22), M(7.2,30), M(6.8,39.5), M(5.5,41.8), M(2.2,43)}, true)
  + ellipse(mx, my-44, 1.8, 1.4) + ellipse(mx+0.2, my-47, 3.0, 3.5) + ellipse(mx+0.4, my-50, 4.1, 1.9)
  + poly({W(-3.4,31), W(-4.2,24), W(-6.2,12), W(-8.2,1.5), W(-7.5,-0.5), W(7.8,-0.5), W(8.6,1.5), W(6.6,12), W(4.6,24), W(3.8,31)}, true)
  + poly({W(-2,39.5), W(-5.6,37.5), W(-6.4,33), W(-5.2,27), W(0,24.5), W(5.4,27), W(6.4,33), W(5.6,37.5), W(2,39.5)}, true)
  + ellipse(wx, wy-40.2, 1.5, 1.2) + ellipse(wx-0.2, wy-43.2, 2.7, 3.1) + ellipse(wx-0.2, wy-46.2, 1.7, 1.4)):grow(0.7)
horsesM = horseMask(440, 552, 17, -1) + horseMask(515, 566, 18, 1)
print(figM:area(), horsesM:area())

--@ chunk 108
local botP = {{350,490},{460,500},{560,516},{660,538},{760,560},{880,586},{1010,612}}
local top = below(function(x) return 462 + 0.014*x end)
local notFar = mask(function(x,y) return (x > 360 and y < interp(botP,x) + 1) and 0 or 1 end)
local excl = M1:grow(1) + riverM:grow(1) + riseM + oakM:grow(1) + oakSil:grow(1) + trunk2M:grow(1) + trunkM:grow(1) + figM + horsesM + ditchM:grow(1.2)
meadowM = (top * notFar) - excl
local mFar = pile{{"yellow ochre",2.4},{"Prussian blue",0.3},{"lead white",1.6},{"green earth",0.7}, medium=0.05}
local mMid = pile{{"yellow ochre",2.4},{"Prussian blue",0.38},{"lead white",1.2},{"green earth",0.6}, medium=0.05}
local mNear = pile{{"yellow ochre",2.3},{"Prussian blue",0.48},{"raw umber",0.3},{"lead white",0.8},{"green earth",0.5}, medium=0.05}
-- depth within the meadow: 0 at its far edge, 1 at the crest of the rise
local function depth(x,y)
  local t = (x > 360) and interp(botP,x) or (462 + 0.014*x)
  local b = riseY(math.max(-10, math.min(1010, x)))
  return clamp((y - t) / math.max(20, b - t), 0, 1)
end
work(meadowM * mask(function(x,y) return smoothstep(0.45, 0.2, depth(x,y)) end), {hand="body", tool="filbert 5", pile=mFar, angle=0.1, coverage=2.6, fill=true, pressure={0.4,0.65}, edge="found", seed=1081})
work(meadowM * mask(function(x,y) local d = depth(x,y); return smoothstep(0.2, 0.45, d) * smoothstep(0.8, 0.6, d) end), {hand="body", tool="filbert 6", pile=mMid, angle=0.12, coverage=2.6, fill=true, pressure={0.4,0.65}, edge="found", seed=1082})
work(meadowM * mask(function(x,y) return smoothstep(0.6, 0.8, depth(x,y)) end), {hand="body", tool="filbert 6", pile=mNear, angle=0.12, coverage=2.6, fill=true, pressure={0.4,0.65}, edge="found", seed=1083})
blend(meadowM:shrink(1.5), {tool={kind="badger", width=10}, angle=0.1, coverage=1.0, seed=1084})

--@ chunk 109
local gsh = pile{{"green earth",1.2},{"raw umber",0.6},{"Prussian blue",0.4},{"yellow ochre",0.9},{"lead white",0.2}, medium=0.06}
local mx, my, wx, wy = 328, 573, 344, 577
local function M(u, v) return {mx + u, my - v} end
local function W(u, v) return {wx + u, wy - v} end
local s1 = poly({M(1,0.5), M(4,-0.5), M(22,-3.2), M(24,-1.8), M(8,1.2)}, true) - figM
local s2 = poly({W(4,0.5), W(8,-0.5), W(28,-3.2), W(30,-1.2), W(10,1.5)}, true) - figM
work(s1 + s2, {hand="detail", pile=gsh, tool={kind="round", width=1.4, point=0.3}, angle=0.12, coverage=2.5, fill=true, clip=true, seed=1091})
local h1 = ellipse(449, 552.5, 9, 1.4) - horsesM
local h2 = ellipse(526, 566.5, 10, 1.5) - horsesM
work(h1 + h2, {hand="detail", pile=gsh, tool={kind="round", width=1.2, point=0.3}, angle=0.05, coverage=2.2, fill=true, clip=true, seed=1092})

--@ chunk 110
-- a lace of small leaf touches along the crown's outer edge, to break the brushy flicks into foliage
local rim = oakSil:rim(10, 4) * rect(0,80,480,380)
local fdk = pile{{"Prussian blue",1},{"raw umber",1.1},{"yellow ochre",0.9},{"bone black",0.12}, medium=0.05}
local fmd = pile{{"yellow ochre",2},{"Prussian blue",0.5},{"raw umber",0.35},{"lead white",0.45}, medium=0.04}
stipple(rim, {pile=fdk, width=1.8, coverage=0.9, pressure={0.35,0.8}, drag={1.5, -1.2}, twist=0.6, cluster={0.7,5}, feather=0.4, seed=1101})
-- mid-green leaves bridging the yellow lights into the darks, on the lit (left and upper) side
local litSide = oakSil * mask(function(x,y) return smoothstep(0.2, -0.4, ((x-255)/190) + 0.6*((y-290)/185)) end)
stipple(litSide, {pile=fmd, width=1.6, coverage=0.5, pressure={0.35,0.7}, drag={1.2,-1.0}, cluster={0.6,4}, feather=0.5, seed=1102})

--@ chunk 111
local lights = (oakF:lit{soft=0.15}:map(function(v) return smoothstep(0.6, 0.9, v) end)) * crownF:lit{soft=0.3} * oakSil:shrink(2)
local lt = pile{{"yellow ochre",2},{"chrome yellow",0.5},{"Prussian blue",0.18},{"lead white",1.6}, medium=0.03}
stipple(lights, {pile=lt, width=1.5, coverage=0.55, pressure={0.35,0.75}, drag={1.0,-1.0}, twist=0.5, cluster={0.7,4}, feather=0.8, seed=1111})

--@ chunk 112
local flicks = {{165,152,16,4.5},{120,203,10,4},{112,231,9,4},{82,282,11,4},{74,402,17,5},{245,108,13,4},{320,120,8,4},{378,185,12,6},{420,248,13,8},{443,300,16,5},{433,316,9,5},{200,125,7,3}}
local keep = oakSil:shrink(2.5)
local function skyPile(y)
  if y < 130 then return pile{{"lead white",8},{"pale smalt",1.8},{"cobalt blue",0.45}, medium=0.1}
  elseif y < 210 then return pile{{"lead white",9},{"pale smalt",1.5},{"cobalt blue",0.25},{"yellow ochre",0.04}, medium=0.1}
  elseif y < 290 then return pile{{"lead white",10},{"pale smalt",1.0},{"cobalt blue",0.1},{"yellow ochre",0.12}, medium=0.1}
  elseif y < 360 then return pile{{"lead white",10},{"pale smalt",0.6},{"yellow ochre",0.3},{"vermilion",0.03}, medium=0.1}
  else return pile{{"lead white",10},{"yellow ochre",0.45},{"vermilion",0.06},{"pale smalt",0.15}, medium=0.1} end
end
for i,f in ipairs(flicks) do
  local m = ellipse(f[1], f[2], f[3]+2, f[4]+2) - keep
  work(m, {hand="detail", pile=skyPile(f[2]), tool={kind="round", width=2.2, point=0.2}, angle=0.05, coverage=3, fill=true, edge="soft", pressure={0.5,0.8}, seed=1120+i})
end

--@ chunk 113
local flicks = {{165,152,16,4.5},{120,203,10,4},{112,231,9,4},{82,282,11,4},{74,402,17,5},{245,108,13,4},{320,120,8,4},{378,185,12,6},{420,248,13,8},{443,300,16,5},{433,316,9,5},{200,125,7,3}}
local keep = oakSil:shrink(2.5)
local function skyPile(y)
  if y < 130 then return d2
  elseif y < 210 then return d3
  elseif y < 260 then return pile{{"lead white",7},{"pale smalt",1.6},{"cobalt blue",0.4},{"yellow ochre",0.05}, medium=0.1}
  elseif y < 330 then return s4
  else return s5 end
end
for i,f in ipairs(flicks) do
  local m = ellipse(f[1], f[2], f[3]+2.5, f[4]+2.5) - keep
  work(m, {hand="detail", pile=skyPile(f[2]), tool={kind="round", width=2.2, point=0.2}, angle=0.05, coverage=3, fill=true, edge="soft", pressure={0.5,0.8}, seed=1130+i})
  blend(ellipse(f[1], f[2], f[3]+6, f[4]+5) - keep, {tool={kind="badger", width=6}, angle=0.05, coverage=1.2, pressure={0.2,0.35}, seed=1150+i})
end

--@ chunk 114
wait(60*24*3)
local keep = oakSil:shrink(2.5)
local m = ellipse(165,152,19,7) - keep
work(m, {hand="body", pile=d1, tool={kind="filbert", width=4}, angle=0.05, coverage=2.5, fill=true, edge="soft", pressure={0.45,0.7}, seed=1161})

--@ chunk 115
local keep = oakSil:shrink(2.5)
skyA = pile{{"lead white",7},{"pale smalt",1.5},{"cobalt blue",0.55},{"smalt",0.15},{"yellow ochre",0.06}, medium=0.1}
work(ellipse(245,108,16,7) - keep, {hand="body", pile=skyA, tool={kind="filbert", width=4}, angle=0.05, coverage=2.5, fill=true, edge="soft", pressure={0.45,0.7}, seed=1171})
work(ellipse(200,125,10,6) - keep, {hand="body", pile=skyA, tool={kind="filbert", width=4}, angle=0.05, coverage=2.5, fill=true, edge="soft", pressure={0.45,0.7}, seed=1172})

--@ chunk 116
local keep = oakSil:shrink(2.5)
skyB = pile{{"lead white",4},{"cobalt blue",1.2},{"smalt",1.0},{"pale smalt",0.6},{"raw umber",0.05}, medium=0.1}
work(ellipse(245,108,16,7) - keep, {hand="body", pile=skyB, tool={kind="filbert", width=4}, angle=0.05, coverage=2.5, fill=true, edge="soft", pressure={0.45,0.7}, seed=1181})

--@ chunk 117
wait(60*24*4)
local keep = oakSil:shrink(2.5)
skyT = pile{{"lead white",4},{"cobalt blue",1.4},{"smalt",1.1},{"pale smalt",0.5},{"raw umber",0.1},{"yellow ochre",0.05}, medium=0.1}
for i,e in ipairs({{165,152,19,7},{245,108,17,7},{200,125,11,6},{320,120,11,6}}) do
  work(ellipse(e[1],e[2],e[3],e[4]) - keep, {hand="body", pile=skyT, tool={kind="filbert", width=4}, angle=0.05, coverage=2.5, fill=true, edge="soft", pressure={0.45,0.7}, seed=1190+i})
end

--@ chunk 118
local keep = oakSil:shrink(2.5)
for i,e in ipairs({{165,152,19,7},{245,108,17,7},{200,125,11,6},{320,120,11,6}}) do
  blend(ellipse(e[1],e[2],e[3]+4,e[4]+4) - keep, {tool={kind="badger", width=6}, angle=0.05, coverage=1.5, pressure={0.25,0.4}, seed=1200+i})
end
local P2 = pile{{"lead white",5},{"cobalt blue",1.2},{"smalt",0.9},{"pale smalt",0.6},{"raw umber",0.1},{"yellow ochre",0.08}, medium=0.1}
local P3 = pile{{"lead white",6},{"pale smalt",1.0},{"cobalt blue",0.7},{"smalt",0.4},{"yellow ochre",0.15},{"raw umber",0.08}, medium=0.1}
local P4 = pile{{"lead white",8},{"pale smalt",0.6},{"cobalt blue",0.2},{"yellow ochre",0.35},{"raw umber",0.05}, medium=0.1}
local P5 = pile{{"lead white",8},{"yellow ochre",0.5},{"vermilion",0.05},{"pale smalt",0.25},{"raw umber",0.04}, medium=0.1}
local rest = {{120,203,13,7,P2},{378,185,15,9,P2},{112,231,12,7,P3},{420,248,16,11,P3},{82,282,14,7,P4},{443,300,19,8,P4},{433,316,12,8,P4},{74,402,20,8,P5}}
for i,e in ipairs(rest) do
  local m = ellipse(e[1],e[2],e[3],e[4]) - keep
  work(m, {hand="body", pile=e[5], tool={kind="filbert", width=4}, angle=0.05, coverage=2.5, fill=true, edge="soft", pressure={0.45,0.7}, seed=1210+i})
  blend(ellipse(e[1],e[2],e[3]+4,e[4]+4) - keep, {tool={kind="badger", width=6}, angle=0.05, coverage=1.5, pressure={0.25,0.4}, seed=1230+i})
end

--@ chunk 119
wait(60*24*3)
local town = (townM + townM2)
local cut = rect(0,0,1000,413)
local CM = cloud2M - town
local lit = cloud2F:lit{soft=0.3} * cut
local cMid = pile{{"lead white",7},{"pale smalt",1.2},{"red earth",0.15},{"yellow ochre",0.25},{"raw umber",0.08}, medium=0.12}
local cSh  = pile{{"lead white",4.5},{"pale smalt",1.5},{"smalt",0.35},{"red earth",0.25},{"raw umber",0.18}, medium=0.12}
local cLt  = pile{{"lead white",14},{"yellow ochre",0.3},{"vermilion",0.04}, medium=0.08}
local ang = cloud2F:field("across")
work(CM, {hand="body", pile=cMid, tool="filbert 8", angle=ang, coverage=2.4, fill=true, edge="soft", pressure={0.45,0.7}, seed=1191})
local shW = lit:map(function(v) return smoothstep(0.5, 0.25, v) end)
local belly = mask(function(x,y) return smoothstep(345, 405, y) end)
work(CM * (shW + belly), {hand="body", pile=cSh, tool="filbert 8", angle=ang, coverage=2.0, fill=true, edge="soft", pressure={0.45,0.7}, seed=1192})
local ltW = lit:map(function(v) return smoothstep(0.62, 0.85, v) end) * mask(function(x,y) return smoothstep(380, 330, y) end)
work(CM:shrink(1) * ltW, {hand="body", pile=cLt, tool="filbert 6", angle=ang, coverage=2.0, fill=true, edge="soft", pressure={0.45,0.7}, seed=1193})
blend(CM:shrink(1), {tool={kind="badger", width=18}, angle=ang, coverage=1.6, pressure={0.25,0.45}, seed=1194})
blend(CM:shrink(1), {tool={kind="badger", width=14}, angle=0.35, coverage=1.0, pressure={0.2,0.35}, seed=1195})

--@ chunk 120
wait(60*24*3)
local base = 441
local P = {}
-- great church: west tower, octagon, needle spire
P[#P+1] = poly({{599.3,base},{599.3,362},{610.2,362},{610.2,base}}, true)
P[#P+1] = poly({{600.8,362.5},{601.2,353},{608.4,353},{608.8,362.5}}, true)
P[#P+1] = poly({{601.0,353.5},{604.6,322},{605.0,322},{608.6,353.5}}, true)
P[#P+1] = ellipse(604.8, 321.2, 0.6, 1.0)
-- nave with steep roof and lower chancel
P[#P+1] = poly({{610,base},{610,417},{614,403.5},{688,403.5},{693,417},{693,base}}, true)
P[#P+1] = poly({{692,base},{692,421},{695,412},{708,412},{712,421},{712,base}}, true)
P[#P+1] = poly({{645,404},{646,398},{647,404}}, true)  -- ridge turret
-- left church: slim tower with spire
P[#P+1] = poly({{540.2,base},{540.2,406},{551,406},{551,base}}, true)
P[#P+1] = poly({{540,406.5},{545.5,370},{546,370},{551.3,406.5}}, true)
P[#P+1] = poly({{551,base},{551,420},{555,410},{580,410},{584,420},{584,base}}, true)
-- right tower with pyramid cap and little lantern
P[#P+1] = poly({{722,base},{722,386},{735,386},{735,base}}, true)
P[#P+1] = poly({{721.5,386.5},{728.5,374},{735.5,386.5}}, true)
P[#P+1] = poly({{727.6,375},{727.8,367},{729.2,367},{729.4,375}}, true)
-- houses: gables and roofs
local houses = {{520,528,424},{528,536,428},{584,592,426},{592,599,422},{712,719,425},{735,746,418},{746,753,423},{753,765,414},{765,772,424},{772,784,419},{784,791,426},{791,800,429},{800,808,432}}
for _,hs in ipairs(houses) do
  local x0,x1,t = hs[1],hs[2],hs[3]
  local mx = (x0+x1)/2
  P[#P+1] = poly({{x0,base},{x0,t+ (x1-x0)*0.45},{mx,t},{x1,t+(x1-x0)*0.45},{x1,base}}, true)
end
local T = P[1]
for i=2,#P do T = T + P[i] end
townM3 = T
local tw3 = pile{{"lead white",2.5},{"smalt",2},{"raw umber",0.5},{"red earth",0.35},{"Prussian blue",0.04}, medium=0.08}
work(townM3, {hand="detail", pile=tw3, tool={kind="round", width=1.6, point=0.3}, angle=math.pi/2, coverage=3, fill=true, clip=true, pressure={0.5,0.8}, seed=1201})

--@ chunk 121
wait(60*24*2)
local tw4 = pile{{"lead white",2.2},{"smalt",2},{"raw umber",0.6},{"red earth",0.35},{"Prussian blue",0.06}, medium=0.06}
work(townM3, {hand="detail", pile=tw4, tool={kind="round", width=1.0, point=0.5}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, pressure={0.5,0.8}, seed=1211})
-- lit west faces, warm and faint
local lit = pile{{"lead white",4},{"smalt",1.2},{"red earth",0.35},{"yellow ochre",0.35},{"raw umber",0.3}, medium=0.06}
local faces = poly({{599.5,440},{599.5,362},{602.5,362},{602.5,440}}, true)
  + poly({{601.2,362},{601.6,354},{603.4,354},{603.4,362}}, true)
  + poly({{601.4,353},{604.7,324},{604.8,353}}, true)
  + poly({{540.4,440},{540.4,406},{543.2,406},{543.2,440}}, true)
  + poly({{540.6,405.5},{545.6,372},{545.7,405.5}}, true)
  + poly({{722.2,440},{722.2,387},{725.5,387},{725.5,440}}, true)
  + poly({{722.3,386},{728.4,375},{728.4,386}}, true)
work(faces, {hand="detail", pile=lit, tool={kind="round", width=0.9, point=0.5}, angle=math.pi/2, coverage=2.5, clip=true, pressure={0.4,0.7}, seed=1212})
-- belfry openings
local dk = pile{{"smalt",1.5},{"raw umber",1},{"lead white",0.6},{"bone black",0.1}, medium=0.05}
local op = poly({{602.3,370},{602.3,365.5},{603.3,365.5},{603.3,370}}, true) + poly({{606.2,370},{606.2,365.5},{607.2,365.5},{607.2,370}}, true)
  + poly({{727.5,395},{727.5,391},{729.5,391},{729.5,395}}, true)
  + poly({{544.8,414},{544.8,410.5},{546.4,410.5},{546.4,414}}, true)
work(op, {hand="detail", pile=dk, tool={kind="round", width=0.7, point=0.5}, angle=math.pi/2, coverage=3, clip=true, pressure={0.4,0.6}, seed=1213})

--@ chunk 122
wait(60*24*2)
local base = 441
local P = {}
P[#P+1] = poly({{599.3,base},{599.3,362},{610.2,362},{610.2,base}})
P[#P+1] = poly({{600.8,362.5},{601.2,353},{608.4,353},{608.8,362.5}})
P[#P+1] = poly({{601.0,353.5},{604.7,322},{604.9,322},{608.6,353.5}})
P[#P+1] = poly({{610,base},{610,418},{615,404},{687,404},{692,418},{692,base}})
P[#P+1] = poly({{692,base},{692,421},{696,413},{707,413},{711,421},{711,base}})
P[#P+1] = poly({{645.2,404.5},{646,396},{646.8,404.5}})
P[#P+1] = poly({{540.2,base},{540.2,406},{551,406},{551,base}})
P[#P+1] = poly({{540,406.5},{545.5,370},{545.7,370},{551.2,406.5}})
P[#P+1] = poly({{551,base},{551,420},{555,410.5},{580,410.5},{584,420},{584,base}})
P[#P+1] = poly({{722,base},{722,386},{735,386},{735,base}})
P[#P+1] = poly({{721.6,386.5},{728.5,374},{735.4,386.5}})
P[#P+1] = poly({{727.8,375},{728.1,366.5},{728.9,366.5},{729.2,375}})
local houses = {{518,527,425},{527,535,429},{584,592,425},{592,599,421},{711,718,426},{735,745,419},{745,752,424},{752,763,415},{763,771,425},{771,782,420},{782,790,426},{790,799,430},{799,808,433}}
for _,hs in ipairs(houses) do
  local x0,x1,t = hs[1],hs[2],hs[3]
  local mx = (x0+x1)/2
  P[#P+1] = poly({{x0,base},{x0,t+(x1-x0)*0.55},{mx,t},{x1,t+(x1-x0)*0.55},{x1,base}})
end
local T = P[1]
for i=2,#P do T = T + P[i] end
townM4 = T
local tw4 = pile{{"lead white",2.2},{"smalt",2},{"raw umber",0.6},{"red earth",0.35},{"Prussian blue",0.06}, medium=0.06}
work(townM4, {hand="detail", pile=tw4, tool={kind="round", width=1.2, point=0.5}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=1221})

--@ chunk 123
local old = (townM + townM2 + townM3):grow(1.2) * rect(500,300,320,141)
local cover = old - townM4:grow(0.2)
local cb = pile{{"lead white",6},{"pale smalt",1.3},{"red earth",0.2},{"yellow ochre",0.22},{"raw umber",0.12},{"smalt",0.1}, medium=0.08}
local cr = pile{{"lead white",10},{"yellow ochre",0.45},{"vermilion",0.05},{"pale smalt",0.3},{"raw umber",0.03}, medium=0.08}
local up = cover * rect(500,300,320,114)
local lo = cover * rect(500,414,320,26)
work(up, {hand="detail", pile=cb, tool={kind="round", width=1.4, point=0.3}, angle=0.2, coverage=3.5, fill=true, clip=true, pressure={0.5,0.8}, seed=1231})
work(lo, {hand="detail", pile=cr, tool={kind="round", width=1.4, point=0.3}, angle=0.0, coverage=3.5, fill=true, clip=true, pressure={0.5,0.8}, seed=1232})

--@ chunk 124
wait(60*24*3)
local keep = oakSil:shrink(2.5)
local spots = {{165,152},{120,203},{112,231},{82,282},{74,402},{245,108},{320,120},{378,185},{420,248},{443,300},{433,316},{200,125}}
local function skyAt(y)
  if y < 140 then return pile{{"lead white",4},{"cobalt blue",1.5},{"smalt",1.2},{"pale smalt",0.4},{"raw umber",0.08}, medium=0.3}
  elseif y < 215 then return pile{{"lead white",5},{"cobalt blue",1.3},{"smalt",1.0},{"pale smalt",0.5},{"raw umber",0.08},{"yellow ochre",0.05}, medium=0.3}
  elseif y < 265 then return pile{{"lead white",6},{"pale smalt",1.1},{"cobalt blue",0.8},{"smalt",0.4},{"yellow ochre",0.12}, medium=0.3}
  elseif y < 340 then return pile{{"lead white",8},{"pale smalt",0.5},{"cobalt blue",0.15},{"yellow ochre",0.4},{"vermilion",0.02}, medium=0.3}
  else return pile{{"lead white",9},{"yellow ochre",0.5},{"vermilion",0.05},{"pale smalt",0.2}, medium=0.3} end
end
for i,s in ipairs(spots) do
  local m = ellipse(s[1], s[2], 24, 13):soften(4) - keep
  work(m, {hand="scumble", pile=skyAt(s[2]), tool={kind="filbert", width=5}, angle=0.05, coverage=1.2, load=0.35, edge="lost", pressure={0.3,0.5}, seed=1240+i})
  blend(ellipse(s[1], s[2], 26, 14) - keep, {tool={kind="badger", width=8}, angle=0.05, coverage=1.3, pressure={0.2,0.35}, seed=1260+i})
end

--@ chunk 125
wait(60*24*4)
local keep = oakSil:shrink(1)
local function skyAt(y)
  if y < 140 then return pile{{"lead white",3.6},{"cobalt blue",1.5},{"smalt",1.2},{"pale smalt",0.5},{"raw umber",0.1},{"yellow ochre",0.05}, medium=0.1}
  elseif y < 215 then return pile{{"lead white",4.3},{"cobalt blue",1.35},{"smalt",1.05},{"pale smalt",0.5},{"raw umber",0.1},{"yellow ochre",0.07}, medium=0.1}
  elseif y < 265 then return pile{{"lead white",5.2},{"cobalt blue",1.0},{"smalt",0.6},{"pale smalt",0.9},{"raw umber",0.08},{"yellow ochre",0.14}, medium=0.1}
  elseif y < 340 then return pile{{"lead white",7},{"pale smalt",0.8},{"cobalt blue",0.3},{"yellow ochre",0.4},{"vermilion",0.03},{"raw umber",0.04}, medium=0.1}
  else return pile{{"lead white",8},{"yellow ochre",0.5},{"vermilion",0.05},{"pale smalt",0.3},{"raw umber",0.04}, medium=0.1} end
end
local spots = {{165,152,26,12},{120,203,22,11},{112,231,20,10},{82,282,20,10},{74,402,22,10},{245,108,28,12},{320,120,22,11},{378,185,22,12},{420,248,22,13},{443,300,22,12},{433,316,16,10},{200,125,18,10},{150,112,20,10}}
for i,s in ipairs(spots) do
  local m = ellipse(s[1], s[2], s[3], s[4]):soften(3) - keep
  work(m, {hand="body", pile=skyAt(s[2]), tool={kind="filbert", width=5}, angle=0.05, coverage=2.2, fill=true, edge="soft", pressure={0.45,0.7}, seed=1280+i})
end
for i,s in ipairs(spots) do
  blend(ellipse(s[1], s[2], s[3]+5, s[4]+4) - keep, {tool={kind="badger", width=8}, angle=0.05, coverage=1.4, pressure={0.2,0.35}, seed=1300+i})
end

--@ chunk 126
wait(60*24*4)
local keep = oakSil:shrink(1) + (cloud2M + townM4):grow(2)
-- long horizontal wisps: {cx, cy, rx, ry, tilt}
local wisps = {
  {215,112,165,15,-0.03},{150,153,120,12,0.02},{105,204,95,12,0.0},{395,188,95,13,-0.02},
  {95,234,80,10,0.02},{440,250,85,14,0.01},{75,284,70,10,0.0},{455,304,95,16,0.0},
  {640,118,150,9,-0.02},{820,158,170,10,0.01},{600,205,110,8,0.0},{900,92,110,7,-0.02},{520,72,120,7,0.01}}
local function wm(w)
  local cx,cy,rx,ry,t = w[1],w[2],w[3],w[4],w[5]
  return mask(function(x,y)
    local u = (x-cx)/rx; local v = (y-cy - t*(x-cx))/ry
    local r = u*u + v*v*(1 + 0.6*u*u)
    return smoothstep(1.0, 0.45, r)
  end)
end
local function wp(y)
  if y < 140 then return pile{{"lead white",9},{"pale smalt",1.4},{"cobalt blue",0.35},{"yellow ochre",0.08}, medium=0.1}
  elseif y < 220 then return pile{{"lead white",10},{"pale smalt",1.1},{"cobalt blue",0.2},{"yellow ochre",0.12}, medium=0.1}
  elseif y < 270 then return pile{{"lead white",10},{"pale smalt",0.8},{"yellow ochre",0.2},{"vermilion",0.02}, medium=0.1}
  else return pile{{"lead white",11},{"pale smalt",0.4},{"yellow ochre",0.35},{"vermilion",0.04}, medium=0.1} end
end
for i,w in ipairs(wisps) do
  local m = wm(w) - keep
  work(m, {hand="body", pile=wp(w[2]), tool={kind="filbert", width=6}, angle=w[5], coverage=2.0, fill=true, edge="lost", pressure={0.4,0.65}, length={30,90}, seed=1320+i})
end
for i,w in ipairs(wisps) do
  local m = (wm({w[1],w[2],w[3]*1.15,w[4]*1.5,w[5]})) - keep
  blend(m, {tool={kind="badger", width=12}, angle=w[5], coverage=1.4, pressure={0.2,0.4}, seed=1340+i})
end

--@ chunk 127
local keep = oakSil:shrink(1) + (cloud2M + townM4):grow(2)
local wisps = {
  {215,112,165,15,-0.03},{150,153,120,12,0.02},{105,204,95,12,0.0},{395,188,95,13,-0.02},
  {95,234,80,10,0.02},{440,250,85,14,0.01},{75,284,70,10,0.0},{455,304,95,16,0.0},
  {640,118,150,9,-0.02},{820,158,170,10,0.01},{600,205,110,8,0.0},{900,92,110,7,-0.02},{520,72,120,7,0.01}}
local function wm(w, k)
  local cx,cy,rx,ry,t = w[1],w[2],w[3]*k,w[4]*k,w[5]
  return mask(function(x,y)
    local u = (x-cx)/rx; local v = (y-cy - t*(x-cx))/ry
    local r = u*u + v*v*(1 + 0.6*u*u)
    return smoothstep(1.0, 0.3, r)
  end)
end
local function bp(y)
  if y < 140 then return pile{{"lead white",3},{"cobalt blue",1.6},{"smalt",1.3},{"pale smalt",0.4},{"raw umber",0.06}, medium=0.2}
  elseif y < 220 then return pile{{"lead white",4},{"cobalt blue",1.3},{"smalt",1.0},{"pale smalt",0.6},{"raw umber",0.06}, medium=0.2}
  elseif y < 270 then return pile{{"lead white",5},{"pale smalt",1.2},{"cobalt blue",0.7},{"smalt",0.4},{"yellow ochre",0.1}, medium=0.2}
  else return pile{{"lead white",7},{"pale smalt",0.8},{"cobalt blue",0.2},{"yellow ochre",0.35},{"vermilion",0.03}, medium=0.2} end
end
for i,w in ipairs(wisps) do
  work(wm(w,1.05) - keep, {hand="scumble", pile=bp(w[2]), tool={kind="filbert", width=8}, angle=w[5], coverage=1.6, load=0.6, edge="lost", pressure={0.35,0.6}, seed=1360+i})
end
for i,w in ipairs(wisps) do
  blend(wm(w,1.3) - keep, {tool={kind="badger", width=16}, angle=w[5], coverage=2.0, pressure={0.25,0.45}, seed=1380+i})
end

--@ chunk 128
local keep = oakSil:shrink(1) + (cloud2M + townM4):grow(2)
local wisps = {
  {215,112,165,15,-0.03},{150,153,120,12,0.02},{105,204,95,12,0.0},{395,188,95,13,-0.02},
  {95,234,80,10,0.02},{440,250,85,14,0.01},{75,284,70,10,0.0},{455,304,95,16,0.0},
  {640,118,150,9,-0.02},{820,158,170,10,0.01},{600,205,110,8,0.0},{900,92,110,7,-0.02},{520,72,120,7,0.01}}
local Z = nil
for i,w in ipairs(wisps) do
  local cx,cy,rx,ry,t = w[1],w[2],w[3]*1.2,w[4]*2.0,w[5]
  local m = ellipse(cx, cy, rx, ry):soften(6)
  Z = Z and (Z + m) or m
end
Z = Z - keep
blend(Z, {tool={kind="badger", width=20}, angle=0.0, coverage=2.2, pressure={0.3,0.5}, seed=1401})
blend(Z, {tool={kind="badger", width=14}, angle=0.25, coverage=1.2, pressure={0.2,0.4}, seed=1402})

--@ chunk 129
wait(60*24*5)
local keep = oakSil + oakM + (cloud2M + townM4):grow(0.5)
local zone = rect(-10,-10,1020,355) - keep
local bands = {
  {-10, 80,  pile{{"lead white",5},{"cobalt blue",1.6},{"smalt",1.1},{"pale smalt",0.3}, medium=0.12}},
  {60, 150,  pile{{"lead white",6.5},{"cobalt blue",1.2},{"smalt",0.8},{"pale smalt",0.7}, medium=0.12}},
  {130, 220, pile{{"lead white",8},{"pale smalt",1.7},{"cobalt blue",0.5},{"smalt",0.1}, medium=0.12}},
  {200, 290, pile{{"lead white",10},{"pale smalt",1.2},{"cobalt blue",0.12},{"yellow ochre",0.1}, medium=0.12}},
  {270, 350, pile{{"lead white",10},{"pale smalt",0.6},{"yellow ochre",0.3},{"vermilion",0.03}, medium=0.12}},
}
for i,b in ipairs(bands) do
  local m = zone * rect(-10, b[1], 1020, b[2]-b[1])
  work(m, {hand="broad", pile=b[3], tool="filbert 16", angle=0.0, coverage=2.4, fill=true, edge="firm", pressure={0.45,0.7}, seed=1410+i})
end
local bz = (rect(-10,-10,1020,362) - keep) 
blend(bz, {tool={kind="badger", width=40}, angle=0.0, coverage=2.0, pressure={0.3,0.5}, seed=1421})
blend(bz, {tool={kind="badger", width=30}, angle=0.02, coverage=1.2, pressure={0.2,0.4}, seed=1422})

--@ chunk 130
local keep = oakSil:grow(1) + oakM:grow(1) + (cloud2M + townM4):grow(1)
local zone = rect(-10,-10,1020,250) - keep
local bands = {
  {-10, 60,  pile{{"lead white",3},{"cobalt blue",2},{"smalt",1.6}, medium=0.12}},
  {40, 120,  pile{{"lead white",4.5},{"cobalt blue",1.6},{"smalt",1.1},{"pale smalt",0.4}, medium=0.12}},
  {100, 180, pile{{"lead white",6},{"cobalt blue",1.1},{"smalt",0.7},{"pale smalt",0.9}, medium=0.12}},
  {160, 240, pile{{"lead white",8},{"pale smalt",1.6},{"cobalt blue",0.45}, medium=0.12}},
}
for i,b in ipairs(bands) do
  local m = zone * rect(-10, b[1], 1020, b[2]-b[1])
  work(m, {hand="body", pile=b[3], tool="filbert 12", length={60,160}, angle=0.0, coverage=2.2, fill=true, clip=true, pressure={0.45,0.7}, seed=1430+i})
end
blend(rect(-10,-10,1020,300) - keep, {tool={kind="badger", width=24}, angle=0.0, coverage=2.0, clip=true, pressure={0.3,0.5}, seed=1441})

--@ chunk 131
wait(60*24*5)
local town = townM4:grow(0.3) + townM + townM2
local cut = rect(0,0,1000,413)
local CM = cloud2M - town
local lit = cloud2F:lit{soft=0.3} * cut
local ang = cloud2F:field("across")
local cMid = pile{{"lead white",7},{"pale smalt",1.3},{"red earth",0.18},{"yellow ochre",0.25},{"raw umber",0.1}, medium=0.1}
local cSh  = pile{{"lead white",4},{"pale smalt",1.6},{"smalt",0.4},{"red earth",0.28},{"raw umber",0.2}, medium=0.1}
local cLt  = pile{{"lead white",14},{"yellow ochre",0.32},{"vermilion",0.045}, medium=0.06}
work(CM, {hand="body", pile=cMid, tool="filbert 8", angle=ang, coverage=2.6, fill=true, clip=true, edge="soft", pressure={0.45,0.7}, seed=1451})
local shW = lit:map(function(v) return smoothstep(0.5, 0.2, v) end)
local belly = mask(function(x,y) return smoothstep(350, 408, y) end)
work(CM:shrink(2) * (shW + belly), {hand="body", pile=cSh, tool="filbert 7", angle=ang, coverage=2.0, fill=true, clip=true, pressure={0.45,0.7}, seed=1452})
local ltW = lit:map(function(v) return smoothstep(0.6, 0.85, v) end) * mask(function(x,y) return smoothstep(385, 330, y) end)
work(CM:shrink(1.5) * ltW, {hand="body", pile=cLt, tool="filbert 6", angle=ang, coverage=2.2, fill=true, clip=true, pressure={0.45,0.7}, seed=1453})
blend(CM:shrink(2), {tool={kind="badger", width=14}, angle=ang, coverage=1.6, clip=true, pressure={0.25,0.45}, seed=1454})
blend(CM:shrink(2), {tool={kind="badger", width=10}, angle=0.35, coverage=0.9, clip=true, pressure={0.2,0.35}, seed=1455})

--@ chunk 132
local town = townM4:grow(1.5) + townM:grow(1) + townM2:grow(1)
local cut = rect(0,0,1000,414)
local ring = ((cloud2M:grow(11) - cloud2M:shrink(1)) * cut) - town
local lit = cloud2F:lit{soft=0.3}
local cMid = pile{{"lead white",7},{"pale smalt",1.3},{"red earth",0.18},{"yellow ochre",0.25},{"raw umber",0.1}, medium=0.1}
local cLt  = pile{{"lead white",14},{"yellow ochre",0.32},{"vermilion",0.045}, medium=0.06}
local litSide = lit:map(function(v) return smoothstep(0.35, 0.6, v) end)
work(ring * litSide, {hand="detail", pile=cLt, tool={kind="filbert", width=4}, angle=cloud2F:field("edge"), coverage=2.5, fill=true, clip=true, edge="found", pressure={0.45,0.7}, seed=1461})
work(ring - litSide, {hand="detail", pile=cMid, tool={kind="filbert", width=4}, angle=cloud2F:field("edge"), coverage=2.5, fill=true, clip=true, edge="found", pressure={0.45,0.7}, seed=1462})
blend((cloud2M:grow(10) * cut) - town, {tool={kind="badger", width=8}, angle=cloud2F:field("across"), coverage=1.4, clip=true, pressure={0.2,0.4}, seed=1463})

--@ chunk 133
wait(60*24*4)
local halo = ((townM4:grow(5) + townM:grow(3) + townM2:grow(3)) - townM4) * rect(500,300,320,114)
local cMid = pile{{"lead white",7},{"pale smalt",1.3},{"red earth",0.18},{"yellow ochre",0.25},{"raw umber",0.1}, medium=0.1}
local cr = pile{{"lead white",10},{"yellow ochre",0.45},{"vermilion",0.05},{"pale smalt",0.3},{"raw umber",0.03}, medium=0.08}
work(halo * cloud2M:grow(8), {hand="detail", pile=cMid, tool={kind="round", width=1.4, point=0.3}, angle=math.pi/2, coverage=3, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=1471})
work(halo - cloud2M:grow(8), {hand="detail", pile=cr, tool={kind="round", width=1.4, point=0.3}, angle=math.pi/2, coverage=3, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=1472})
local lo = ((townM + townM2):grow(1.5) - townM4) * rect(500,414,320,26)
work(lo, {hand="detail", pile=cr, tool={kind="round", width=1.2, point=0.3}, angle=0, coverage=3, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=1473})

--@ chunk 134
wait(60*24*3)
local tw4 = pile{{"lead white",2.2},{"smalt",2},{"raw umber",0.6},{"red earth",0.35},{"Prussian blue",0.06}, medium=0.06}
work(townM4, {hand="detail", pile=tw4, tool={kind="round", width=1.0, point=0.5}, angle=math.pi/2, coverage=4, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=1481})
local cSh = pile{{"lead white",5},{"pale smalt",1.5},{"smalt",0.3},{"red earth",0.25},{"raw umber",0.18}, medium=0.3}
local around = ((townM4:grow(14) - townM4:grow(0.5)) * cloud2M) * rect(500,300,320,114)
work(around, {hand="glaze", pile=cSh, tool={kind="filbert", width=6, stiffness=0.2}, angle=0.1, coverage=1.0, clip=true, edge="found", pressure={0.25,0.45}, seed=1482})

--@ chunk 135
wait(60*24*2)
local R = mask(function(x,y) return smoothstep(368, 335, y) end)
local nz = noise{seed=133, octaves=3, period=22}
work(oakSil:shrink(6) * R, {hand="body", tool="filbert 6", pile=fd, angle=function(x,y) return oakF:across(x,y) end, coverage=2, fill=true, clip=true, pressure={0.45,0.7}, seed=1491})
stipple(oakSil:grow(1) * R, {pile=fd, width=5, coverage=1.4, pressure={0.4,0.8}, cluster={0.5, 6}, feather=0.5, dips={20,0.7,0.2}, seed=1492})
local fringe = (oakSil:grow(10) - oakSil:shrink(3)) * R
stipple(fringe, {pile=fd, width=3.5, coverage=function(x,y) local d = oakSil:at(x,y); return clamp(0.25 + 0.9*d + 0.6*nz(x,y), 0, 1.6) end, pressure={0.35,0.75}, cluster={0.6, 5}, feather=0.8, dips={18,0.6,0.2}, seed=1493})

--@ chunk 136
wait(8*60)
local R = mask(function(x,y) return smoothstep(380, 345, y) end)
local nz = noise{seed=144, octaves=3, period=16}
local half = oakF:lit{soft=0.5}
stipple(half:grow(3) * R, {pile=fm, width=4, coverage=function(x,y) return clamp(1.1*half:at(x,y) + 0.5*nz(x,y), 0, 1.5) end, pressure={0.35,0.7}, cluster={0.55, 5}, feather=0.7, dips={16,0.6,0.2}, seed=1501})
wait(6*60)
local nz2 = noise{seed=155, octaves=3, period=12}
local lit = oakF:lit{soft=0.15}
stipple(lit * R, {pile=fl3, width=3, coverage=function(x,y) return clamp(0.8*lit:at(x,y) + 0.5*nz2(x,y), 0, 1.1) end, pressure={0.3,0.6}, cluster={0.6, 4}, feather=0.8, dips={14,0.5,0.2}, seed=1502})

--@ chunk 137
wait(6*60)
local R = mask(function(x,y) return smoothstep(380, 345, y) end)
local nz = noise{seed=166, octaves=3, period=20}
local sh = crownF:shadow{soft=0.5} * oakSil * R
stipple(sh, {pile=fd2, width=4.5, coverage=function(x,y) return clamp(1.3*sh:at(x,y) + 0.4*nz(x,y), 0, 1.6) end, pressure={0.4,0.75}, cluster={0.5, 5}, feather=0.6, dips={18,0.6,0.2}, seed=1511})
-- undersides of each clump, a little shade
local und = oakF:shadow{soft=0.4} * oakSil * R
stipple(und, {pile=fd2, width=3.5, coverage=function(x,y) return clamp(0.8*und:at(x,y) + 0.3*nz(x,y), 0, 1.0) end, pressure={0.35,0.7}, cluster={0.5, 4}, feather=0.7, dips={18,0.6,0.2}, seed=1512})
-- branches seen through the gaps
local gaps = -oakSil:shrink(0.5)
local br = {
  {{{250,470},{238,400},{215,330},{195,260},{185,200}}, {7,6,4.5,3.2,2}},
  {{{255,470},{275,395},{300,320},{320,250},{330,190}}, {7,6,4.5,3.2,2}},
  {{{238,400},{185,352},{140,300}}, {4.5,3.2,2}},
  {{{275,395},{340,360},{392,318}}, {4.5,3.2,2}},
  {{{215,330},{238,262},{250,180}}, {4,3,1.8}},
  {{{300,320},{282,242},{290,160}}, {4,3,1.8}},
  {{{195,260},{160,225},{130,205}}, {3,2.2,1.4}},
  {{{320,250},{360,225},{385,205}}, {3,2.2,1.4}},
}
local B = nil
for _,b in ipairs(br) do local m = ribbon(b[1], b[2]):roughen(0.6, 8, 3); B = B and (B + m) or m end
branchM = B
work(B * gaps * rect(0,100,480,340), {hand="detail", pile=tb3, tool={kind="round", width=1.6, point=0.4}, angle=function(x,y) return -1.3 end, coverage=3, fill=true, clip=true, edge="found", pressure={0.45,0.75}, seed=1513})

--@ chunk 138
local R = mask(function(x,y) return smoothstep(385, 350, y) end)
local rim = oakSil:rim(10, 4) * rect(0,80,480,380) * R
local fdk = pile{{"Prussian blue",1},{"raw umber",1.1},{"yellow ochre",0.9},{"bone black",0.12}, medium=0.05}
stipple(rim, {pile=fdk, width=1.8, coverage=0.9, pressure={0.35,0.8}, drag={1.5, -1.2}, twist=0.6, cluster={0.7,5}, feather=0.4, seed=1521})
-- sprays of leaves reaching out from the masses into the gaps and the sky
local nz = noise{seed=177, octaves=3, period=9}
local spray = (oakSil:grow(7) - oakSil:shrink(1)) * R * mask(function(x,y) return clamp(nz(x,y)*1.6, 0, 1) end)
stipple(spray, {pile=fdk, width=2.2, coverage=0.8, pressure={0.35,0.75}, drag={1.2,-1.0}, twist=0.6, cluster={0.7,4}, feather=0.6, seed=1522})
local litSide = oakSil * R * mask(function(x,y) return smoothstep(0.2, -0.4, ((x-255)/190) + 0.6*((y-290)/185)) end)
stipple(litSide, {pile=fm, width=1.6, coverage=0.45, pressure={0.35,0.7}, drag={1.2,-1.0}, cluster={0.6,4}, feather=0.5, seed=1523})

--@ chunk 139
wait(60*24*3)
local town = townM4:grow(1)
local cut = rect(0,0,1000,414)
local CM = (cloud2M:grow(8) * cut) - town
local shd = cloud2F:shadow{soft=0.45} * cut
local gl = pile{{"lead white",2},{"pale smalt",1.6},{"smalt",0.35},{"red earth",0.3},{"raw umber",0.3}, medium=0.55}
local w = CM:shrink(3) * (shd + mask(function(x,y) return 0.8*smoothstep(345, 400, y) end))
work(w, {hand="glaze", pile=gl, tool={kind="filbert", width=12, stiffness=0.2}, angle=cloud2F:field("across"), coverage=1.3, clip=true, edge="found", pressure={0.3,0.5}, seed=1531})
blend(CM:shrink(4), {tool={kind="badger", width=12}, angle=cloud2F:field("across"), coverage=1.5, clip=true, pressure={0.2,0.4}, seed=1532})

--@ chunk 140
local town = townM4:grow(1.2)
local cut = rect(0,0,1000,414)
local CM = ((cloud2M:grow(8) * cut) - town):shrink(2)
blend(CM, {tool={kind="badger", width=18}, angle=cloud2F:field("across"), coverage=2.2, clip=true, pressure={0.3,0.5}, seed=1541})
blend(CM, {tool={kind="badger", width=14}, angle=0.35, coverage=1.5, clip=true, pressure={0.25,0.45}, seed=1542})
blend(CM, {tool={kind="badger", width=12}, angle=-0.3, coverage=1.0, clip=true, pressure={0.2,0.4}, seed=1543})

--@ chunk 141
wait(60*24*2)
local rng = rng_seed and rng_seed(9) 
local bY = pile{{"chrome yellow",2},{"lead white",1},{"yellow ochre",0.3}, medium=0.05}
local dW = pile{{"lead white",6},{"yellow ochre",0.1}, medium=0.05}
local cR = pile{{"vermilion",1},{"lead white",1.5},{"red earth",0.5}, medium=0.05}
local keep = trunkM:grow(2) + oakM:grow(1) + stoneM:grow(1) + figM:grow(2)
local function scatter(p, n, seed, sz)
  local b = brush{kind="round", width=3, point=0.8}
  b:load(p, 0.9)
  local cnt = 0
  math.randomseed(seed)
  for i=1,n*3 do
    local x = math.random()*1000
    local ytop = riseY(x) + 6
    local y = ytop + math.random()^0.8 * (714 - ytop)
    if y < 712 and not (keep:at(x,y) > 0.5) then
      local s = sz * (0.5 + 1.2*(y - 560)/150)
      b:touch(x, y, {pressure=clamp(0.25*s,0.1,0.8), twist=math.random(), drag={math.random()-0.5, 0}})
      cnt = cnt + 1
      if cnt % 25 == 0 then b:load(p, 0.9) end
      if cnt >= n then break end
    end
  end
end
scatter(bY, 220, 11, 1.0)
scatter(dW, 120, 12, 0.9)
scatter(cR, 45, 13, 0.9)

--@ chunk 142
local bY = pile{{"chrome yellow",2},{"lead white",0.8},{"yellow ochre",0.3}, medium=0.05}
local dW = pile{{"lead white",6},{"yellow ochre",0.1}, medium=0.05}
local st = pile{{"green earth",1.5},{"yellow ochre",1},{"Prussian blue",0.15},{"lead white",0.3}, medium=0.05}
local so = pile{{"red earth",1.5},{"vermilion",0.4},{"raw umber",0.4},{"lead white",0.3}, medium=0.05}
local keep = trunkM:grow(2) + oakM:grow(1) + stoneM:grow(1) + figM:grow(2)
math.randomseed(77)
local stem = brush{kind="rigger", width=1.2, point=1}
local head = brush{kind="round", width=4, point=0.7}
local plants = {}
-- clumps along the near foreground
for c=1,16 do
  local cx = 30 + math.random()*960
  local cy = 640 + math.random()*68
  if cx < 60 or cx > 400 or cy < 660 then
  for k=1,5+math.floor(math.random()*6) do
    local x = cx + (math.random()-0.5)*40
    local y = cy + (math.random()-0.5)*14
    if y > riseY(x)+20 and y < 712 and keep:at(x,y) < 0.5 then plants[#plants+1] = {x,y} end
  end end
end
stem:load(st, 0.9)
local tops = {}
for i,p in ipairs(plants) do
  local s = 0.6 + (p[2]-620)/90
  local hgt = (14 + math.random()*16) * s
  local lean = (math.random()-0.5)*6*s
  local tx, ty = p[1]+lean, p[2]-hgt
  stem:stroke({{p[1],p[2]},{p[1]+lean*0.4, p[2]-hgt*0.5},{tx,ty}}, {pressure={0.45,0.15}, ramps={0.05,0.3}})
  if i % 12 == 0 then stem:load(st, 0.9) end
  tops[#tops+1] = {tx,ty,s,math.random()}
end
wait(1)
head:load(bY, 0.9)
local n=0
for _,t in ipairs(tops) do if t[4] < 0.6 then
  head:touch(t[1], t[2], {pressure=clamp(0.35*t[3],0.2,0.7), twist=math.random()}); n=n+1
  if n % 15 == 0 then head:load(bY,0.9) end
end end
head:reload(dW, 0.9)
for _,t in ipairs(tops) do if t[4] >= 0.6 and t[4] < 0.85 then
  head:touch(t[1], t[2], {pressure=clamp(0.35*t[3],0.2,0.7), twist=math.random()})
end end
-- sorrel spikes: small elongated rusty dabs
local sb = brush{kind="round", width=2, point=0.9}
sb:load(so, 0.9)
for _,t in ipairs(tops) do if t[4] >= 0.85 then
  for j=0,3 do sb:touch(t[1]+(math.random()-0.5)*1.5, t[2]+j*2.2*t[3], {pressure=0.35}) end
end end
print(#plants)

--@ chunk 143
local bY = pile{{"chrome yellow",2},{"lead white",0.8},{"yellow ochre",0.3}, medium=0.05}
local dW = pile{{"lead white",6},{"yellow ochre",0.1}, medium=0.05}
local st = pile{{"green earth",1.5},{"yellow ochre",1},{"Prussian blue",0.15},{"lead white",0.3}, medium=0.05}
local so = pile{{"red earth",1.5},{"vermilion",0.4},{"raw umber",0.4},{"lead white",0.3}, medium=0.05}
local keep = trunkM:grow(2) + oakM:grow(1) + stoneM:grow(1) + figM:grow(2)
math.randomseed(91)
local stem = brush{kind="rigger", width=1.2, point=1}
local head = brush{kind="round", width=4, point=0.7}
local plants = {}
-- clumps along the near foreground
for c=1,14 do
  local cx = 10 + math.random()*510
  local cy = 645 + math.random()*62
  if true then
  for k=1,5+math.floor(math.random()*6) do
    local x = cx + (math.random()-0.5)*40
    local y = cy + (math.random()-0.5)*14
    if y > riseY(x)+20 and y < 712 and keep:at(x,y) < 0.5 then plants[#plants+1] = {x,y} end
  end end
end
stem:load(st, 0.9)
local tops = {}
for i,p in ipairs(plants) do
  local s = 0.6 + (p[2]-620)/90
  local hgt = (14 + math.random()*16) * s
  local lean = (math.random()-0.5)*6*s
  local tx, ty = p[1]+lean, p[2]-hgt
  stem:stroke({{p[1],p[2]},{p[1]+lean*0.4, p[2]-hgt*0.5},{tx,ty}}, {pressure={0.45,0.15}, ramps={0.05,0.3}})
  if i % 12 == 0 then stem:load(st, 0.9) end
  tops[#tops+1] = {tx,ty,s,math.random()}
end
wait(1)
head:load(bY, 0.9)
local n=0
for _,t in ipairs(tops) do if t[4] < 0.6 then
  head:touch(t[1], t[2], {pressure=clamp(0.35*t[3],0.2,0.7), twist=math.random()}); n=n+1
  if n % 15 == 0 then head:load(bY,0.9) end
end end
head:reload(dW, 0.9)
for _,t in ipairs(tops) do if t[4] >= 0.6 and t[4] < 0.85 then
  head:touch(t[1], t[2], {pressure=clamp(0.35*t[3],0.2,0.7), twist=math.random()})
end end
-- sorrel spikes: small elongated rusty dabs
local sb = brush{kind="round", width=2, point=0.9}
sb:load(so, 0.9)
for _,t in ipairs(tops) do if t[4] >= 0.85 then
  for j=0,3 do sb:touch(t[1]+(math.random()-0.5)*1.5, t[2]+j*2.2*t[3], {pressure=0.35}) end
end end
print(#plants)

--@ chunk 144
wait(60*6)
local gD = pile{{"Prussian blue",0.6},{"raw umber",1},{"green earth",1.5},{"yellow ochre",0.6}, medium=0.05}
local gM = pile{{"green earth",1.5},{"yellow ochre",1.2},{"Prussian blue",0.3},{"lead white",0.3}, medium=0.05}
local keep = trunkM:grow(2) + stoneM:grow(1)
math.randomseed(311)
local b = brush{kind="rigger", width=1.4, point=1}
for pass=1,2 do
  local p = pass==1 and gD or gM
  b:reload(p, 0.9)
  local N = pass==1 and 420 or 200
  for i=1,N do
    local x = math.random()*1000
    local y0 = riseY(x) + 40
    if y0 < 712 then
      local y = y0 + math.random()*(716-y0)
      if keep:at(x,y) < 0.5 then
        local s = 0.6 + (y-620)/90
        local hgt = (10 + math.random()*14)*s
        local lean = (math.random()-0.5)*8*s
        b:stroke({{x,y},{x+lean*0.3,y-hgt*0.5},{x+lean,y-hgt}}, {pressure={0.5,0.05}, ramps={0.05,0.5}})
        if i % 15 == 0 then b:load(p, 0.9) end
      end
    end
  end
end

--@ chunk 145
wait(60*24)
local cr = pile{{"lead white",10},{"yellow ochre",0.42},{"vermilion",0.05},{"pale smalt",0.25}, medium=0.08}
local smear = (ellipse(92,414,14,6) - oakSil:grow(1)) - oakM:grow(1)
work(smear, {hand="detail", pile=cr, tool={kind="filbert", width=3}, angle=0, coverage=3, fill=true, clip=true, edge="found", pressure={0.5,0.7}, seed=1601})
blend(smear:shrink(1), {tool={kind="badger", width=6}, angle=0, coverage=1.2, clip=true, pressure={0.2,0.35}, seed=1602})
-- hanging bough under the lowest clump
local bough = ellipse(150,446,40,9):roughen(3, 10, 3)
local nz = noise{seed=188, octaves=3, period=8}
stipple(bough, {pile=fd, width=3.5, coverage=function(x,y) return clamp(0.9 + 0.5*nz(x,y), 0, 1.4) end, pressure={0.35,0.75}, cluster={0.6,5}, feather=0.7, seed=1603})
stipple(bough:grow(4) - bough:shrink(2), {pile=fd, width=2.2, coverage=function(x,y) return clamp(0.3+0.8*nz(x,y),0,1) end, pressure={0.35,0.75}, drag={1,-0.8}, cluster={0.6,4}, feather=0.8, seed=1604})
local top = bough * mask(function(x,y) return smoothstep(452, 440, y) * smoothstep(195, 120, x) end)
stipple(top, {pile=fm, width=2.5, coverage=function(x,y) return clamp(0.8*top:at(x,y)+0.4*nz(x,y),0,1.1) end, pressure={0.35,0.7}, cluster={0.6,4}, feather=0.7, seed=1605})

--@ chunk 146
local bd = pile{{"raw umber",1},{"smalt",0.8},{"lead white",1.2},{"bone black",0.2}, medium=0.05}
local b = brush{kind="rigger", width=1.0, point=1}
b:load(bd, 0.8)
local birds = {{560,150,5.0,0.1},{588,168,4.0,-0.15},{612,142,3.4,0.2},{700,205,2.6,0},{470,120,3.8,-0.1}}
for _,q in ipairs(birds) do
  local x,y,s,t = q[1],q[2],q[3],q[4]
  b:stroke({{x-s, y-0.6*s+t*s}, {x-0.45*s, y-0.15*s}, {x, y+0.15*s}}, {pressure={0.1,0.45}, ramps={0.1,0.1}})
  b:stroke({{x, y+0.15*s}, {x+0.5*s, y-0.2*s}, {x+s*1.05, y-0.5*s-t*s}}, {pressure={0.45,0.1}, ramps={0.1,0.1}})
end

--@ chunk 147
wait(60*24*14)

--@ chunk 148
print(drying(300,300), drying(800,300), drying(500,650))

--@ chunk 149
local cl = oakSil:grow(14):shrink(14)
print(oakSil:area(), cl:area(), (cl-oakSil):area())
for _,p in ipairs({{275,172},{229,313},{283,309},{154,305},{342,313},{337,247},{200,245},{250,250}}) do
  print(p[1],p[2], oakSil:at(p[1],p[2]), cl:at(p[1],p[2]))
end

--@ chunk 150
-- Oak rework: close the small gaps between clumps into larger masses, keep a few deliberate openings
oakHoles = poly({{262,178},{270,166},{283,160},{292,166},{290,178},{280,186},{268,187}}, true)
  + poly({{188,248},{200,238},{215,240},{222,250},{210,258},{195,258}}, true)
  + poly({{210,318},{222,302},{240,300},{248,312},{240,326},{222,330}}, true)
  + poly({{288,305},{300,292},{312,300},{308,322},{296,332},{288,320}}, true)
  + poly({{316,254},{326,240},{340,238},{344,250},{334,262},{320,264}}, true)
oakHoles = oakHoles:roughen(2.5, 9, 41)
oakFork = poly({{205,475},{212,420},{228,388},{250,376},{285,388},{302,420},{312,475}}, true)
oakNew = (oakSil:grow(14):shrink(14)):roughen(4, 16, 42) + oakSil
oakNew = oakNew - oakHoles - (oakFork - oakSil)
local gapFill = oakNew - oakSil:shrink(3)
print(gapFill:area(), oakNew:area())
stipple(gapFill, {pile=fd, width=4.5, coverage=1.7, pressure={0.45,0.85}, cluster={0.4,5}, feather=0.3, dips={20,0.7,0.2}, clip=oakNew:grow(1.5), seed=2001})
local nz = noise{seed=2002, octaves=3, period=10}
local edgeLace = (oakNew:grow(6) - oakNew:shrink(2)) - oakHoles:shrink(2) - oakFork
stipple(edgeLace, {pile=fd, width=3, coverage=function(x,y) return clamp(0.3 + 0.7*oakNew:at(x,y) + 0.7*nz(x,y), 0, 1.3) end, pressure={0.35,0.75}, drag={1.2,-1.0}, twist=0.5, cluster={0.65,5}, feather=0.8, dips={16,0.6,0.2}, seed=2003})

--@ chunk 151
print(wait(60*24))
print(drying(300,250), drying(200,300))
local mp = {}
local function M(x,y,rx,ry,z,sd) mp[#mp+1] = {body.ellipsoid({x,y,z},{rx,ry,(rx+ry)/2}):rough(ry*0.12, ry*0.9, sd)} end
M(255,150,105,62,10,1)   -- top crown
M(160,240,72,50,0,2)     -- left upper
M(365,245,62,52,-5,3)    -- right upper
M(265,262,70,40,15,4)    -- middle band
M(125,345,68,50,5,5)     -- left mid
M(360,335,72,52,-5,6)    -- right mid
M(265,350,48,32,20,7)    -- centre, in front of the fork
M(150,415,62,38,10,8)    -- left low
M(355,410,70,42,0,9)     -- right low
mp.light = {from={-1,-0.7}, front=0.3, ambient=0.12}
massF = form(mp)
local nz = noise{seed=2011, octaves=3, period=14}
-- shade the lower part of each mass, which darkens the old per-ball lights sitting there
local sh = massF:shadow{soft=0.35} * oakNew
stipple(sh, {pile=fd2, width=3.5, coverage=function(x,y) return clamp(1.2*sh:at(x,y) + 0.4*nz(x,y), 0, 1.5) end, pressure={0.4,0.75}, cluster={0.5,5}, feather=0.6, dips={18,0.6,0.2}, clip=oakNew:grow(3), seed=2012})

--@ chunk 152
print(wait(60*12))
local nz = noise{seed=2021, octaves=3, period=16}
local nz2 = noise{seed=2022, octaves=3, period=9}
local lit = massF:lit{soft=0.5}
local cl = crownF:lit{soft=0.6}
fmd = pile{{"yellow ochre",1.6},{"Prussian blue",0.6},{"raw umber",0.5},{"lead white",0.3}, medium=0.04}
-- the right, shaded side of the crown: only a dull green on the tops of its masses
local litR = lit * oakNew
stipple(litR, {pile=fmd, width=3.5, coverage=function(x,y) return clamp(1.0*litR:at(x,y)*(1 - 0.6*cl:at(x,y)) + 0.45*nz(x,y), 0, 1.2) end, pressure={0.35,0.7}, cluster={0.6,5}, feather=0.7, dips={16,0.6,0.2}, clip=oakNew:grow(2), seed=2023})
-- the sunlit side: half-tone on the tops of the masses
stipple(litR, {pile=fm, width=3.5, coverage=function(x,y) return clamp(1.2*litR:at(x,y)*cl:at(x,y) + 0.5*nz2(x,y), 0, 1.4) end, pressure={0.35,0.7}, cluster={0.6,5}, feather=0.7, dips={16,0.6,0.2}, clip=oakNew:grow(2), seed=2024})

--@ chunk 153
print(wait(60*8))
local nz = noise{seed=2031, octaves=3, period=10}
local lit = massF:lit{soft=0.15}
local cl = crownF:lit{soft=0.5}
local L = lit * oakNew
stipple(L, {pile=fl3, width=2.6, coverage=function(x,y) return clamp(0.9*L:at(x,y)*(0.25 + 0.75*cl:at(x,y)) + 0.5*nz(x,y), 0, 1.0) end, pressure={0.3,0.6}, drag={1.0,-0.8}, cluster={0.7,4}, feather=0.8, dips={12,0.5,0.2}, clip=oakNew:grow(1), seed=2032})
-- a cooler, silvery sheen on the upper edges of the shaded right-hand masses, where the sky catches the leaves
local sheen = pile{{"lead white",1},{"Prussian blue",0.25},{"yellow ochre",0.5},{"raw umber",0.2}, medium=0.05}
local rimR = (oakNew - oakNew:shrink(7)) * mask(function(x,y) return smoothstep(270, 340, x) * smoothstep(420, 300, y) end)
stipple(rimR, {pile=sheen, width=2, coverage=function(x,y) return clamp(0.35*rimR:at(x,y) + 0.3*nz(x,y), 0, 0.6) end, pressure={0.3,0.55}, drag={1.0,-0.8}, cluster={0.7,3}, feather=0.9, seed=2033})

--@ chunk 154
print(wait(60*6))
local nz = noise{seed=2041, octaves=3, period=6}
-- leaf sprays reaching into the openings from their rims, so they read as gaps in foliage, not holes
local inner = (oakHoles:grow(1) - oakHoles:shrink(5)) * mask(function(x,y) return clamp(0.5 + 1.2*nz(x,y), 0, 1) end)
stipple(inner, {pile=fd, width=2.2, coverage=0.9, pressure={0.3,0.7}, drag={1.4,-1.1}, twist=0.7, cluster={0.7,3}, feather=0.6, seed=2042})
-- small scattered leaves on the outer edge, reaching out into the sky
local nz2 = noise{seed=2043, octaves=3, period=8}
local out = (oakNew:grow(9) - oakNew:grow(1)) * mask(function(x,y) return clamp(1.4*nz2(x,y), 0, 1) end) - oakFork
stipple(out, {pile=fd, width=2.2, coverage=0.5, pressure={0.3,0.7}, drag={1.4,-1.1}, twist=0.7, cluster={0.75,4}, feather=0.7, seed=2044})
-- a dead limb breaking out of the crown at the upper right, grey and bare
local dead = pile{{"raw umber",1.5},{"lead white",1.2},{"bone black",0.3},{"yellow ochre",0.3}, medium=0.05}
local b = brush{kind="round", width=2.6, point=0.6}
b:load(dead, 0.8)
b:stroke({{322,168},{334,150},{343,132},{356,116},{366,104}}, {pressure={0.8,0.25}, ramps={0.05,0.4}, shake=0.6})
b:load(dead, 0.6)
b:stroke({{343,132},{334,118},{331,104}}, {pressure={0.55,0.1}, ramps={0.05,0.5}, shake=0.6})
b:stroke({{356,116},{368,114},{378,106}}, {pressure={0.5,0.1}, ramps={0.05,0.5}, shake=0.6})
b:stroke({{366,104},{366,94},{370,88}}, {pressure={0.4,0.05}, ramps={0.05,0.5}, shake=0.6})
b:stroke({{334,150},{346,146},{352,140}}, {pressure={0.4,0.05}, ramps={0.05,0.5}, shake=0.6})
-- a thin dark underside to the dead limb
local dk = pile{{"raw umber",1.5},{"bone black",0.5}, medium=0.05}
local r = brush{kind="rigger", width=1.0, point=1}
r:load(dk, 0.5)
r:stroke({{325,170},{337,152},{346,134},{358,118},{367,106}}, {pressure={0.45,0.1}, shake=0.4})

--@ chunk 155
print(wait(60*20))
print(drying(343,132))
local dead2 = pile{{"raw umber",1.6},{"bone black",0.35},{"lead white",0.45},{"yellow ochre",0.2}, medium=0.05}
local b = brush{kind="round", width=2.8, point=0.6}
b:load(dead2, 0.8)
b:stroke({{321,170},{334,150},{343,132},{356,116},{366,104}}, {pressure={0.85,0.3}, ramps={0.05,0.4}, shake=0.5})
b:load(dead2, 0.6)
b:stroke({{343,132},{334,118},{331,104}}, {pressure={0.6,0.1}, ramps={0.05,0.5}, shake=0.5})
b:stroke({{356,116},{368,114},{378,106}}, {pressure={0.55,0.1}, ramps={0.05,0.5}, shake=0.5})
b:stroke({{366,104},{366,94},{370,88}}, {pressure={0.45,0.05}, ramps={0.05,0.5}, shake=0.5})
b:stroke({{334,150},{346,146},{352,140}}, {pressure={0.45,0.05}, ramps={0.05,0.5}, shake=0.5})
-- a thin silvery light along its upper-left edge
local lt = pile{{"lead white",2},{"raw umber",0.5},{"yellow ochre",0.2}, medium=0.05}
local r = brush{kind="rigger", width=0.9, point=1}
r:load(lt, 0.4)
r:stroke({{322,166},{333,148},{341,131},{353,115}}, {pressure={0.35,0.15}, shake=0.4})

--@ chunk 156
print(wait(60*24))
local dead3 = pile{{"raw umber",2},{"bone black",0.8},{"lead white",0.3}, medium=0.02}
local b = brush{kind="round", width=2.4, point=0.5}
for i=1,2 do
b:load(dead3, 1.0)
b:stroke({{321,170},{334,150},{343,132},{356,116},{366,104}}, {pressure={0.9,0.35}, ramps={0.05,0.3}, shake=0.4})
b:load(dead3, 0.8)
b:stroke({{343,132},{334,118},{331,104}}, {pressure={0.7,0.15}, ramps={0.05,0.5}, shake=0.4})
b:stroke({{356,116},{368,114},{378,106}}, {pressure={0.65,0.15}, ramps={0.05,0.5}, shake=0.4})
b:stroke({{366,104},{366,94},{370,88}}, {pressure={0.5,0.1}, ramps={0.05,0.5}, shake=0.4})
b:stroke({{334,150},{346,146},{352,140}}, {pressure={0.5,0.1}, ramps={0.05,0.5}, shake=0.4})
end

--@ chunk 157
local R = riverM
work(R * mask(function(x,y) return smoothstep(640,560,x) end), {hand="body", tool="filbert 3", pile=rvA, angle=0.12, coverage=3, fill=true, clip=true, pressure={0.5,0.75}, seed=3001})
work(R * mask(function(x,y) return smoothstep(560,640,x)*smoothstep(860,780,x) end), {hand="body", tool="filbert 3", pile=rvB, angle=0.15, coverage=3, fill=true, clip=true, pressure={0.5,0.75}, seed=3002})
work(R * mask(function(x,y) return smoothstep(780,860,x) end), {hand="body", tool="filbert 4", pile=rvC, angle=0.15, coverage=3, fill=true, clip=true, pressure={0.5,0.75}, seed=3003})
blend(R:shrink(0.5), {tool={kind="badger", width=10}, angle=0.14, coverage=1.2, clip=true, seed=3004})

--@ chunk 158
local R = riverM
rvD = pile{{"lead white",4.5},{"pale smalt",1.6},{"cobalt blue",0.9},{"smalt",0.3},{"raw umber",0.05}, medium=0.06}
rvE = pile{{"lead white",6},{"pale smalt",1.5},{"cobalt blue",0.5},{"yellow ochre",0.05}, medium=0.06}
work(R * mask(function(x,y) return smoothstep(640,760,x)*smoothstep(900,820,x) end), {hand="body", tool="filbert 3", pile=rvE, angle=0.15, coverage=2.4, fill=true, clip=true, pressure={0.5,0.75}, seed=3011})
work(R * mask(function(x,y) return smoothstep(800,900,x) end), {hand="body", tool="filbert 4", pile=rvD, angle=0.15, coverage=2.6, fill=true, clip=true, pressure={0.5,0.75}, seed=3012})
blend(R:shrink(0.5) * rect(600,440,420,130), {tool={kind="badger", width=10}, angle=0.14, coverage=1.2, clip=true, seed=3013})

--@ chunk 159
local R = riverM:shrink(0.3)
local rc = pile{{"green earth",1.5},{"raw umber",0.8},{"Prussian blue",0.3},{"lead white",0.7}, medium=0.08}
local rt = pile{{"raw umber",1.2},{"lead white",0.8},{"pale smalt",0.3}, medium=0.08}
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x) + 0.6
  local yn = interp(nearBank, x) - 0.3
  local th = 0.42*s
  -- trunk reflection
  local tb = brush{kind="round", width=math.max(0.9, 0.11*s), point=0.3}
  tb:load(rt, 0.5)
  tb:stroke({{x, yb},{x + rand(-0.2,0.2), math.min(yn, yb + th*0.95)}}, {pressure={0.6,0.45}, clip=R})
  -- crown reflection: short vertical strokes, a soft mass widening a little, broken by ripples
  local y0 = yb + th*0.85
  if y0 < yn - 0.5 then
    local cb = brush{kind="flat", width=math.max(1.0, 0.09*s), stiffness=0.4}
    local n = math.max(4, math.floor(s/3))
    for k=1,n do
      local t = (k-0.5)/n - 0.5
      local xx = x + t * 0.85*s + rand(-0.04,0.04)*s
      local yy0 = y0 + math.abs(t)*0.12*s
      local yy1 = math.min(yn, y0 + (0.55 - 0.5*math.abs(t))*s*rand(0.85,1.1))
      if yy1 > yy0 + 0.5 then
        cb:load(rc, 0.35)
        cb:stroke({{xx, yy0},{xx + rand(-0.2,0.2), yy1}}, {pressure={0.55,0.3}, ramps={0.15,0.4}, orient="across", clip=R})
      end
    end
  end
end

--@ chunk 160
local R = riverM:shrink(0.3)
local zone = nil
for i,w in ipairs({{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x)
  local e = ellipse(x, yb + 0.6*s, 0.6*s, 0.45*s)
  zone = zone and (zone + e) or e
end
blend(zone * R, {tool={kind="badger", width=5}, angle=0.14, coverage=1.3, clip=true, pressure={0.2,0.35}, seed=3021})
blend(zone * R, {tool={kind="badger", width=4}, angle=math.pi/2, coverage=0.8, clip=true, pressure={0.2,0.3}, seed=3022})

--@ chunk 161
blend(riverM:shrink(0.5) * rect(480,440,540,130), {tool={kind="badger", width=8}, angle=0.14, coverage=1.0, clip=true, pressure={0.2,0.35}, seed=3031})

--@ chunk 162
print(wait(60*30))
print(drying(700,490), drying(900,525))
local R = riverM:shrink(0.3)
local rt = pile{{"raw umber",1.2},{"lead white",0.6},{"pale smalt",0.4},{"green earth",0.3}, medium=0.08}
for i,w in ipairs({{452,11},{505,14},{560,18},{626,22},{702,27},{792,33},{884,38},{978,44}}) do
  local x, s = w[1], w[2]
  local yb = interp(farBank, x) + 0.4
  local yn = interp(nearBank, x) - 0.3
  local tb = brush{kind="round", width=math.max(0.9, 0.12*s), point=0.3}
  tb:load(rt, 0.45)
  local y1 = math.min(yn, yb + 0.42*s)
  tb:stroke({{x, yb},{x + 0.1, (yb+y1)/2},{x - 0.1, y1}}, {pressure={0.6,0.15}, ramps={0.05,0.6}, clip=R})
end
-- the far bank's slim shadow line on the water
local bankP = pile{{"raw umber",1},{"green earth",1.5},{"Prussian blue",0.2},{"lead white",0.9}, medium=0.12}
local bb = brush{kind="round", width=1.2, point=0.4}
for x=335,1005,40 do
  local seg = {}
  for xx=x, math.min(x+42,1008), 6 do seg[#seg+1] = {xx, interp(farBank, xx) + 0.7} end
  if #seg > 1 then bb:load(bankP, 0.35); bb:stroke(seg, {pressure={0.3 + 0.0004*(x-335), 0.4 + 0.0004*(x-335)}, clip=R}) end
end

--@ chunk 163
function ditchY(x) return 577 + 0.12*(x-480) end
local post = pile{{"raw umber",1.4},{"lead white",0.9},{"bone black",0.25},{"yellow ochre",0.2}, medium=0.05}
local postL = pile{{"lead white",2},{"raw umber",0.5},{"yellow ochre",0.35}, medium=0.05}
local pb = brush{kind="round", width=1.3, point=0.3}
local lb = brush{kind="rigger", width=0.8, point=1}
local xs = uneven(18, 600, 995, 0.3, 0.2, 77)
fenceTops = {}
for i,x in ipairs(xs) do
  local y = ditchY(x) + 7 + 0.004*(x-600)
  local h = 7.5 + 0.004*(x-600) + rand(-0.8,0.8)
  local lean = rand(-0.5,0.5)
  pb:reload(post, 0.6)
  pb:stroke({{x+lean, y-h},{x, y}}, {pressure={0.75,0.85}})
  lb:reload(postL, 0.4)
  lb:stroke({{x-0.35+lean, y-h+0.3},{x-0.35, y-1.5}}, {pressure={0.3,0.2}})
  fenceTops[#fenceTops+1] = {x+lean, y-h, y}
end
-- two rails, sagging a little between posts
local rail = pile{{"raw umber",1.2},{"lead white",1.0},{"bone black",0.2}, medium=0.06}
local rb = brush{kind="round", width=0.9, point=0.6}
for k,f in ipairs({0.12, 0.5}) do
  for i=1,#fenceTops-1 do
    local a, b = fenceTops[i], fenceTops[i+1]
    if not (i == 7 and k == 1) then
      local ya = a[2] + f*(a[3]-a[2]); local yb = b[2] + f*(b[3]-b[2])
      rb:reload(rail, 0.45)
      rb:stroke({{a[1], ya},{(a[1]+b[1])/2, (ya+yb)/2 + 0.5},{b[1], yb}}, {pressure={0.45,0.45}, shake=0.3})
    end
  end
end

--@ chunk 164
print(wait(60*24))
local post2 = pile{{"raw umber",1.6},{"bone black",0.45},{"lead white",0.35},{"green earth",0.3}, medium=0.03}
local postL = pile{{"lead white",2},{"raw umber",0.4},{"yellow ochre",0.4}, medium=0.03}
local pb = brush{kind="round", width=1.5, point=0.2}
local lb = brush{kind="rigger", width=0.8, point=1}
for i,f in ipairs(fenceTops) do
  pb:reload(post2, 0.8)
  pb:stroke({{f[1]+0.2, f[2]},{f[1]+0.25, f[3]+0.3}}, {pressure={0.85,0.9}})
end
for i,f in ipairs(fenceTops) do
  lb:reload(postL, 0.5)
  lb:stroke({{f[1]-0.45, f[2]+0.4},{f[1]-0.4, f[3]-1.8}}, {pressure={0.4,0.25}})
end
local rail = pile{{"raw umber",1.4},{"bone black",0.3},{"lead white",0.5}, medium=0.04}
local rb = brush{kind="round", width=1.0, point=0.5}
for k,fr in ipairs({0.12, 0.5}) do
  for i=1,#fenceTops-1 do
    local a, b = fenceTops[i], fenceTops[i+1]
    if not (i == 7 and k == 1) and not (i == 12) then
      local ya = a[2] + fr*(a[3]-a[2]); local yb = b[2] + fr*(b[3]-b[2])
      rb:reload(rail, 0.55)
      rb:stroke({{a[1], ya},{(a[1]+b[1])/2, (ya+yb)/2 + 0.5},{b[1], yb}}, {pressure={0.55,0.55}, shake=0.3})
    end
  end
end

--@ chunk 165
function cow(x, y, L, f, coat, lit, dark, seed, grazing)
  local function P(u, v) return {x + f*u*L, y + v*L} end
  -- boxy body, straight back, hip bone and a deep chest; head at the u>0 end
  local body = poly({P(-0.55,-0.52), P(-0.5,-0.58), P(-0.2,-0.57), P(0.2,-0.58), P(0.4,-0.56), P(0.46,-0.45), P(0.46,-0.3), P(0.3,-0.24), P(-0.1,-0.25), P(-0.45,-0.26), P(-0.56,-0.33)})
  local head
  if grazing then
    head = poly({P(0.4,-0.56), P(0.56,-0.5), P(0.66,-0.22), P(0.7,-0.08), P(0.62,-0.06), P(0.52,-0.2), P(0.44,-0.34)})
  else
    head = poly({P(0.4,-0.56), P(0.55,-0.64), P(0.68,-0.6), P(0.72,-0.47), P(0.64,-0.44), P(0.52,-0.44), P(0.44,-0.36)})
  end
  local sh = pile{{"green earth",1.5},{"raw umber",0.9},{"Prussian blue",0.3},{"yellow ochre",0.6},{"lead white",0.3}, medium=0.08}
  work(ellipse(x + f*0.18*L, y + 0.01*L, 0.6*L, 0.08*L), {hand="detail", pile=sh, tool={kind="round", width=1.4, point=0.3}, angle=0.05, coverage=2.2, fill=true, clip=true, seed=seed})
  work(body + head, {hand="detail", pile=coat, tool={kind="round", width=math.max(1,0.07*L), point=0.3}, angle=0.05, coverage=3.5, fill=true, clip=true, seed=seed+1})
  local lb = brush{kind="round", width=math.max(0.9, 0.06*L), point=0.2}
  for i,u in ipairs({-0.47, -0.38, 0.3, 0.38}) do
    lb:reload((i%2==0) and dark or coat, 0.6)
    lb:stroke({P(u, -0.3), P(u, 0.0)}, {pressure={0.9,0.75}})
  end
  -- tail
  local tb = brush{kind="rigger", width=0.6, point=1}
  tb:reload(dark, 0.5)
  tb:stroke({P(-0.55,-0.52), P(-0.6,-0.35), P(-0.59,-0.12)}, {pressure={0.4,0.3}})
  return body, head
end
local red = pile{{"red earth",1.6},{"raw umber",0.7},{"yellow ochre",0.5},{"lead white",0.35}, medium=0.05}
local redL = pile{{"red earth",0.8},{"yellow ochre",1},{"lead white",1.5}, medium=0.05}
local redD = pile{{"raw umber",1.5},{"red earth",0.7},{"bone black",0.3}, medium=0.05}
local blk = pile{{"bone black",1},{"raw umber",0.9},{"lead white",0.25}, medium=0.05}
local blkD = pile{{"bone black",1},{"raw umber",0.6}, medium=0.05}
local wh = pile{{"lead white",3},{"yellow ochre",0.2},{"raw umber",0.12}, medium=0.05}
cowA = {cow(650, 612, 20, -1, red, redL, redD, 3101, true)}
cowB = {cow(700, 612, 21, 1, blk, wh, blkD, 3111, false)}
cowC = {cow(860, 640, 23, -1, red, redL, redD, 3121, false)}

--@ chunk 166
print(wait(60*6))
local redL = pile{{"red earth",0.8},{"yellow ochre",1.1},{"lead white",1.4}, medium=0.05}
local redD = pile{{"raw umber",1.5},{"red earth",0.7},{"bone black",0.35}, medium=0.05}
local wh = pile{{"lead white",3},{"yellow ochre",0.2},{"raw umber",0.12}, medium=0.05}
local blkL = pile{{"bone black",0.6},{"raw umber",0.6},{"lead white",1.2},{"pale smalt",0.3}, medium=0.05}
local function band(c, lo, hi, x, y, L) return (c[1]+c[2]) * mask(function(px,py) return smoothstep(y+lo*L-1, y+lo*L, py) * smoothstep(y+hi*L+1, y+hi*L, py) end) end
local specs = {{cowA, 650, 612, 20, redL, redD}, {cowB, 700, 612, 21, blkL, nil}, {cowC, 860, 640, 23, redL, redD}}
for i,s in ipairs(specs) do
  local c, x, y, L = s[1], s[2], s[3], s[4]
  local all = c[1] + c[2]
  local top = all * mask(function(px,py) return smoothstep(y-0.5*L, y-0.57*L, py) end)
  work(top, {hand="detail", pile=s[5], tool={kind="round", width=1.0, point=0.3}, angle=0.05, coverage=2, clip=true, seed=3200+i})
  if s[6] then
    local bel = all * mask(function(px,py) return smoothstep(y-0.34*L, y-0.27*L, py) end)
    work(bel, {hand="detail", pile=s[6], tool={kind="round", width=1.0, point=0.3}, angle=0.05, coverage=2, clip=true, seed=3210+i})
  end
end
-- the black cow: white patches on flank and a white face
local x, y, L = 700, 612, 21
local p1 = poly({{700-0.15*L, y-0.52*L},{700+0.1*L, y-0.55*L},{700+0.15*L, y-0.36*L},{700-0.05*L, y-0.27*L},{700-0.22*L, y-0.33*L}}, true) * cowB[1]
local p2 = poly({{700-0.5*L, y-0.45*L},{700-0.38*L, y-0.5*L},{700-0.35*L, y-0.3*L},{700-0.5*L, y-0.3*L}}, true) * cowB[1]
local face = poly({{700+0.6*L, y-0.62*L},{700+0.7*L, y-0.6*L},{700+0.72*L, y-0.47*L},{700+0.65*L, y-0.45*L}}, true) * cowB[2]
work(p1 + p2 + face, {hand="detail", pile=wh, tool={kind="round", width=1.0, point=0.3}, angle=0.3, coverage=3, fill=true, clip=true, seed=3221})
-- small horns and ears
local hb = brush{kind="rigger", width=0.6, point=1}
local horn = pile{{"lead white",2},{"yellow ochre",0.5},{"raw umber",0.3}, medium=0.05}
hb:reload(horn, 0.4)
hb:stroke({{700+0.6*L, y-0.63*L},{700+0.58*L, y-0.7*L}}, {pressure={0.5,0.1}})
hb:stroke({{860-0.6*23, 640-0.63*23},{860-0.58*23, 640-0.7*23}}, {pressure={0.5,0.1}})

--@ chunk 167
local cut = rect(0,0,1000,414)
local C = (cloud2M:grow(9) * cut)
print(C:area())
for _,p in ipairs({{560,400},{600,360},{650,320},{700,280},{760,250},{820,240},{900,290},{990,330},{1000,400},{620,405}}) do print(p[1],p[2],C:at(p[1],p[2])) end

--@ chunk 168
local cut = rect(0,0,1000,414)
local town = townM4:grow(1.5) + townM:grow(1) + townM2:grow(1)
cloudC = (cloud2M:grow(9) * cut) - town
bigF = form{ {body.ellipsoid({790,330,0},{250,130,160})}, light={from={-1,-0.9}, front=0.3, ambient=0.15} }
local big = bigF:lit{soft=0.45}
local bil = cloud2F:lit{soft=0.3}
cloudV = mask(function(x,y) return clamp(0.5*big:at(x,y) + 0.5*bil:at(x,y) - 0.35*smoothstep(372, 408, y), 0, 1) end)
cLz = cloudV:map(function(v) return smoothstep(0.56, 0.64, v) end) * cloudC
cSz = cloudV:map(function(v) return smoothstep(0.40, 0.32, v) end) * cloudC
cMz = cloudC - cLz - cSz
print(cLz:area(), cMz:area(), cSz:area())

--@ chunk 169
kS = pile{{"lead white",5},{"pale smalt",1.5},{"smalt",0.25},{"red earth",0.18},{"raw umber",0.18},{"yellow ochre",0.1}, medium=0.06}
kM = pile{{"lead white",8},{"pale smalt",1.0},{"red earth",0.1},{"yellow ochre",0.25},{"raw umber",0.06}, medium=0.06}
kL = pile{{"lead white",14},{"yellow ochre",0.3},{"vermilion",0.03}, medium=0.05}
local ang = cloud2F:field("across")
work(cSz:grow(2) * cloudC, {hand="body", tool="filbert 5", pile=kS, angle=ang, length={10,28}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3301})
work(cMz:grow(2) * cloudC, {hand="body", tool="filbert 5", pile=kM, angle=ang, length={10,28}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3302})
work(cLz:grow(1) * cloudC, {hand="body", tool="filbert 5", pile=kL, angle=ang, length={10,28}, coverage=2.8, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3303})

--@ chunk 170
blend(cloudC:shrink(1), {tool={kind="badger", width=10}, angle=cloud2F:field("across"), coverage=1.0, clip=true, pressure={0.2,0.35}, seed=3311})
print(wait(60*24*2))
print(drying(700,300), drying(800,390))

--@ chunk 171
print(wait(60*24*2))
print(drying(700,300), drying(800,390))
local S2 = cloudV:map(function(v) return smoothstep(0.44, 0.34, v) end) * cloudC
local D = cloudV:map(function(v) return smoothstep(0.24, 0.15, v) end) * cloudC
local S2o = S2 - D
print(S2o:area(), D:area())
kS2 = pile{{"lead white",3.5},{"pale smalt",1.6},{"smalt",0.35},{"red earth",0.22},{"raw umber",0.3},{"yellow ochre",0.08}, medium=0.06}
kD = pile{{"lead white",2.6},{"pale smalt",1.5},{"smalt",0.45},{"red earth",0.28},{"raw umber",0.45}, medium=0.06}
local ang = cloud2F:field("across")
work(S2o:grow(1.5) * cloudC, {hand="body", tool="filbert 5", pile=kS2, angle=ang, length={10,26}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3321})
work(D:grow(1.5) * cloudC, {hand="body", tool="filbert 5", pile=kD, angle=ang, length={10,26}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3322})

--@ chunk 172
local S2 = cloudV:map(function(v) return smoothstep(0.44, 0.34, v) end) * cloudC
local edgeZ = (S2:grow(9) - S2:shrink(5)) * cloudC:shrink(1)
blend(edgeZ, {tool={kind="badger", width=10}, angle=cloud2F:field("across"), coverage=1.6, clip=cloudC:shrink(1), pressure={0.25,0.4}, seed=3331})
blend(S2:shrink(4) * cloudC:shrink(1), {tool={kind="badger", width=14}, angle=0.1, coverage=0.8, clip=true, pressure={0.2,0.35}, seed=3332})

--@ chunk 173
print(wait(60*24*3))
print(drying(700,300), drying(800,390), drying(650,330))
local function Z(lo, hi, s) return cloudV:map(function(v) return smoothstep(lo - s, lo + s, v) * smoothstep(hi + s, hi - s, v) end) * cloudC end
local ang = cloud2F:field("across")
kH = pile{{"lead white",8},{"pale smalt",1.1},{"red earth",0.1},{"yellow ochre",0.2},{"raw umber",0.08}, medium=0.06}
kQ = pile{{"lead white",5.5},{"pale smalt",1.4},{"smalt",0.2},{"red earth",0.16},{"raw umber",0.16},{"yellow ochre",0.1}, medium=0.06}
local zH = Z(0.47, 0.58, 0.015)
local zQ = Z(0.36, 0.47, 0.015)
local zS = Z(0.24, 0.36, 0.015)
local zD = Z(-1, 0.24, 0.015)
work(zD:grow(1) * cloudC, {hand="body", tool="filbert 4", pile=kD, angle=ang, length={8,22}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3341})
work(zS:grow(1) * cloudC, {hand="body", tool="filbert 4", pile=kS2, angle=ang, length={8,22}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3342})
work(zQ:grow(1) * cloudC, {hand="body", tool="filbert 4", pile=kQ, angle=ang, length={8,22}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3343})
work(zH:grow(1) * cloudC, {hand="body", tool="filbert 4", pile=kH, angle=ang, length={8,22}, coverage=2.6, fill=true, clip=cloudC, pressure={0.5,0.75}, seed=3344})

--@ chunk 174
blend(cloudC:shrink(1.5), {tool={kind="badger", width=12}, angle=cloud2F:field("across"), coverage=1.2, clip=true, pressure={0.2,0.35}, seed=3351})

--@ chunk 175
-- a leafy bough hiding the blunt end of the right-hand limb
local bough = (poly({{312,428},{330,422},{350,428},{366,438},{372,450},{360,458},{340,456},{322,452},{310,444}}, true)):roughen(3, 8, 51)
bough = bough + ellipse(305,432,12,9):roughen(2.5,7,52)
boughM = bough
local nz = noise{seed=3401, octaves=3, period=7}
stipple(bough, {pile=fd, width=3.2, coverage=1.6, pressure={0.4,0.8}, cluster={0.5,4}, feather=0.4, dips={18,0.6,0.2}, seed=3402})
local fr = (bough:grow(6) - bough:shrink(2)) * mask(function(x,y) return clamp(0.4 + nz(x,y)*1.2, 0, 1) end)
stipple(fr, {pile=fd, width=2.2, coverage=0.8, pressure={0.3,0.7}, drag={1.3,1.2}, twist=0.6, cluster={0.7,3}, feather=0.7, seed=3403})

--@ chunk 176
print(wait(60*8))
local nz = noise{seed=3411, octaves=3, period=8}
local top = boughM * mask(function(x,y) return smoothstep(446, 430, y) end)
stipple(top, {pile=fmd, width=2.6, coverage=function(x,y) return clamp(0.6*top:at(x,y) + 0.5*nz(x,y), 0, 1) end, pressure={0.3,0.65}, drag={1.0,-0.8}, cluster={0.6,3}, feather=0.7, seed=3412})
-- lift some of the tree's lower edge into leaves so the limb emerges into the crown, not under it

--@ chunk 177
local st = body.ellipsoid({108,684,0},{40,24,30}):rough(3, 20, 961):cut({108,700,0},{0,1,0}, 1, 2)
local st2 = body.ellipsoid({166,694,0},{17,11,14}):rough(1.5, 10, 962)
stF = form{ {st}, {st2}, light={from={-1,-0.9}, front=0.3, ambient=0.2} }
local stSh = pile{{"raw umber",1.2},{"lead white",1.2},{"pale smalt",0.5},{"bone black",0.2}, medium=0.35}
local sh = stF:shadow{soft=0.5} * stoneM
work(sh, {hand="glaze", pile=stSh, tool={kind="filbert", width=5, stiffness=0.2}, angle=stF:field("across"), coverage=1.3, clip=stoneM, pressure={0.3,0.5}, seed=3501})
local all = pile{{"raw umber",1},{"lead white",1.6},{"yellow ochre",0.3},{"green earth",0.3}, medium=0.45}
work(stoneM, {hand="glaze", pile=all, tool={kind="filbert", width=6, stiffness=0.2}, angle=stF:field("across"), coverage=0.8, clip=stoneM, pressure={0.25,0.4}, seed=3503})
local ground = pile{{"Prussian blue",0.8},{"raw umber",1.2},{"green earth",0.8},{"bone black",0.2}, medium=0.05}
stipple((stoneM:grow(3) - stoneM) * rect(60,688,140,30), {pile=ground, width=2.5, coverage=1.4, pressure={0.4,0.8}, cluster={0.5,3}, seed=3502})

--@ chunk 178
print(wait(60*24))
local sDk = pile{{"raw umber",1},{"bone black",0.35},{"lead white",0.7},{"pale smalt",0.3},{"green earth",0.2}, medium=0.05}
local sMd = pile{{"lead white",1.4},{"raw umber",0.9},{"bone black",0.2},{"yellow ochre",0.25},{"pale smalt",0.2}, medium=0.05}
local sLt = pile{{"lead white",2.4},{"raw umber",0.45},{"yellow ochre",0.35},{"bone black",0.05}, medium=0.05}
local ang = stF:field("across")
work(stoneM, {hand="body", tool="filbert 3", pile=sMd, angle=ang, coverage=3, fill=true, clip=true, pressure={0.5,0.75}, seed=3511})
local sh = stF:shadow{soft=0.35} * stoneM
work(sh, {hand="body", tool="filbert 3", pile=sDk, angle=ang, coverage=2.4, clip=true, pressure={0.45,0.7}, seed=3512})
local lt = (stF:lit{soft=0.25} * stoneM):map(function(v) return smoothstep(0.55, 0.9, v) end)
work(lt, {hand="detail", pile=sLt, tool={kind="round", width=2.2, point=0.3}, angle=ang, coverage=1.8, clip=true, pressure={0.45,0.7}, seed=3513})
blend(stoneM:shrink(1), {tool={kind="badger", width=6}, angle=-0.6, coverage=0.8, clip=true, pressure={0.2,0.35}, seed=3514})

--@ chunk 179
print(wait(60*24))
local g2 = pile{{"raw umber",1},{"bone black",0.4},{"lead white",1.0},{"pale smalt",0.35}, medium=0.05}
local g3 = pile{{"raw umber",1},{"bone black",0.5},{"lead white",0.5},{"pale smalt",0.3},{"green earth",0.3}, medium=0.05}
local ang = stF:field("across")
local lower = stoneM * mask(function(x,y) return smoothstep(-0.2, 0.45, (x-92)/60 + (y-676)/22) end)
local lower2 = stoneM * mask(function(x,y) return smoothstep(0.3, 0.9, (x-92)/60 + (y-676)/22) end)
work(lower, {hand="body", tool="filbert 3", pile=g2, angle=ang, coverage=2.6, fill=true, clip=true, pressure={0.5,0.75}, seed=3521})
work(lower2, {hand="body", tool="filbert 3", pile=g3, angle=ang, coverage=2.4, fill=true, clip=true, pressure={0.5,0.75}, seed=3522})
blend(stoneM:shrink(1), {tool={kind="badger", width=6}, angle=-0.6, coverage=0.8, clip=true, pressure={0.2,0.35}, seed=3523})

--@ chunk 180
print(wait(60*12))
local lic = pile{{"lead white",2},{"yellow ochre",1.2},{"green earth",0.6}, medium=0.05}
stipple(stoneM:shrink(3) * mask(function(x,y) return smoothstep(0.4, -0.2, (x-92)/60 + (y-676)/22) end), {pile=lic, width=1.2, coverage=0.3, pressure={0.3,0.6}, cluster={0.7,3}, seed=3531})
local crk = pile{{"raw umber",1},{"bone black",0.5},{"lead white",0.3}, medium=0.05}
local r = brush{kind="rigger", width=0.7, point=1}
r:reload(crk, 0.5)
r:stroke({{110,668},{114,677},{111,688}}, {pressure={0.35,0.1}})
r:stroke({{84,683},{95,686},{104,684}}, {pressure={0.3,0.1}})
-- grass growing up in front of and around the stones' foot
local gD = pile{{"raw umber",1.2},{"Prussian blue",0.6},{"yellow ochre",1.2},{"green earth",0.6},{"bone black",0.08}, medium=0.05}
local gM = pile{{"yellow ochre",2.5},{"Prussian blue",0.5},{"raw umber",0.4},{"lead white",0.8}, medium=0.05}
local b = brush{kind="round", width=1.6, point=0.9}
for i=1,110 do
  local x = rand(60, 192)
  local y = rand(692, 716)
  if i % 8 == 1 then b:reload((i%3==1) and gM or gD, 0.7) end
  local L = rand(6, 20)
  local a = -math.pi/2 + rand(-0.4, 0.4)
  b:stroke({{x,y},{x+math.cos(a)*L*0.5, y+math.sin(a)*L*0.5},{x+math.cos(a)*L+rand(-1.5,1.5), y+math.sin(a)*L}}, {pressure={0.75,0.0}, ramps={0.05,0.7}})
end

--@ chunk 181
local nz = noise{seed=3601, octaves=3, period=6}
-- close the left and lower-left openings to small irregular chinks, break the rest
local c1 = poly({{188,248},{200,238},{215,240},{222,250},{210,258},{195,258}}, true):grow(2) - ellipse(214,250,5,3):roughen(1.5,4,61)
local c2 = poly({{210,318},{222,302},{240,300},{248,312},{240,326},{222,330}}, true):grow(2) - poly({{226,312},{236,306},{242,312},{234,318}}, true):roughen(1.5,4,62)
local c3 = (oakHoles:grow(1) - oakHoles:shrink(4)) * mask(function(x,y) return clamp(0.3 + 1.4*nz(x,y), 0, 1) end)
stipple(c1 + c2, {pile=fd, width=2.6, coverage=1.8, pressure={0.4,0.8}, drag={1.2,-1.0}, twist=0.5, cluster={0.4,3}, feather=0.4, seed=3602})
stipple(c3, {pile=fd, width=2.4, coverage=1.0, pressure={0.3,0.7}, drag={1.4,-1.1}, twist=0.7, cluster={0.7,3}, feather=0.6, seed=3603})

--@ chunk 182
local nz = noise{seed=3611, octaves=3, period=5}
local h4 = poly({{288,305},{300,292},{312,300},{308,322},{296,332},{288,320}}, true):grow(2)
local keep4 = poly({{298,300},{307,302},{306,318},{299,324},{295,312}}, true):roughen(1.5,4,63)
local h1 = poly({{262,178},{270,166},{283,160},{292,166},{290,178},{280,186},{268,187}}, true):grow(2)
local keep1 = poly({{272,170},{284,165},{289,172},{281,181},{271,180}}, true):roughen(1.5,4,64)
stipple((h4 - keep4) + (h1 - keep1), {pile=fd, width=2.5, coverage=1.6, pressure={0.4,0.8}, drag={1.2,-1.0}, twist=0.5, cluster={0.5,3}, feather=0.5, seed=3612})
-- a few mid-green leaves on the new closures where the light reaches
stipple(((h4 - keep4) + (h1 - keep1)) * mask(function(x,y) return clamp(0.2 + nz(x,y), 0, 1) end), {pile=fmd, width=2.2, coverage=0.4, pressure={0.3,0.6}, drag={1.0,-0.8}, cluster={0.6,3}, feather=0.6, seed=3613})

--@ chunk 183
print(wait(60*6))
local nz = noise{seed=3621, octaves=3, period=9}
local lit = massF:lit{soft=0.3} * oakNew * mask(function(x,y) return smoothstep(300, 340, x) end)
stipple(lit, {pile=fmd, width=2.8, coverage=function(x,y) return clamp(0.7*lit:at(x,y) + 0.45*nz(x,y), 0, 0.9) end, pressure={0.3,0.65}, drag={1.0,-0.8}, cluster={0.65,4}, feather=0.8, dips={14,0.5,0.2}, clip=oakNew:grow(1), seed=3622})

--@ chunk 184
print(wait(60*24*3)); print(drying(600,380), drying(250,300), drying(700,470), drying(120,690))

--@ chunk 185
-- a small river barge with tanned gaff sail, heading upstream toward the town
boatHull = poly({{642.5,480.6},{647,481.4},{656,481.6},{662.5,480.4},{660.5,484.2},{646,484.4}})
boatSail = poly({{651.2,479.6},{651.2,452.5},{663.5,447.5},{665.5,479.2}})
boatJib  = poly({{650.2,454.5},{642.5,479.4},{650.2,479.6}})
local sailP = pile{{"red earth",1},{"yellow ochre",0.7},{"lead white",1.3},{"pale smalt",0.15}, medium=0.05}
local sailS = pile{{"red earth",1},{"raw umber",0.5},{"yellow ochre",0.3},{"lead white",0.8},{"pale smalt",0.2}, medium=0.05}
local hullP = pile{{"raw umber",1},{"bone black",0.4},{"lead white",0.35},{"pale smalt",0.15}, medium=0.05}
work(boatSail, {hand="detail", pile=sailP, tool={kind="round", width=1.2, point=0.4}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=3701})
work(boatJib, {hand="detail", pile=sailS, tool={kind="round", width=1.0, point=0.4}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=3702})
work(boatHull, {hand="detail", pile=hullP, tool={kind="round", width=1.0, point=0.4}, angle=0, coverage=3.5, fill=true, clip=true, edge="found", pressure={0.5,0.8}, seed=3703})

--@ chunk 186
print(wait(60*5))
local sailD = pile{{"red earth",1},{"raw umber",0.7},{"lead white",0.9},{"pale smalt",0.35}, medium=0.08}
local shade = boatSail * mask(function(x,y) return smoothstep(656, 662, x + (y-465)*0.08) end)
work(shade, {hand="detail", pile=sailD, tool={kind="round", width=1.0, point=0.4}, angle=math.pi/2, coverage=3, fill=true, clip=true, pressure={0.45,0.7}, seed=3711})
local dk = pile{{"raw umber",1},{"bone black",0.6},{"lead white",0.25}, medium=0.1}
local r = brush{kind="rigger", width=0.6, point=1}
r:reload(dk, 0.6)
r:stroke({{651,481},{651,465},{650.9,446}}, {pressure={0.55,0.3}})      -- mast
r:stroke({{651.2,452.6},{657,450.2},{663.8,447.4}}, {pressure={0.35,0.3}}) -- gaff
r:stroke({{651.4,479.3},{658,479.2},{665.4,479.1}}, {pressure={0.3,0.25}}) -- boom
r:reload(dk, 0.3)
r:stroke({{650.9,446.5},{646,462},{642.6,479.8}}, {pressure={0.12,0.12}}) -- forestay

--@ chunk 187
-- the boat's reflection: its own broken ripples in the river, hull dark, sail a dull warm brown
local R = riverM:shrink(0.4)
local hr = pile{{"raw umber",1},{"bone black",0.3},{"lead white",0.6},{"pale smalt",0.4}, medium=0.2}
local sr = pile{{"red earth",0.8},{"raw umber",0.6},{"lead white",1.4},{"pale smalt",0.6}, medium=0.2}
local r = brush{kind="round", width=1.0, point=0.6}
r:reload(hr, 0.4)
r:stroke({{645,485.2},{653,485.4},{660,485.0}}, {pressure={0.35,0.2}, clip=R})
r:stroke({{647.5,486.4},{652,486.6},{657.5,486.3}}, {pressure={0.3,0.1}, clip=R})
r:reload(sr, 0.4)
local ys = {487.5, 488.6, 489.6, 490.5}
for i, y in ipairs(ys) do
  local x0 = 651 + rand(-0.8, 0.8)
  local x1 = 664 - i * 0.9 + rand(-1, 1)
  r:stroke({{x0,y},{(x0+x1)/2, y+rand(-0.15,0.15)},{x1,y+rand(-0.2,0.2)}}, {pressure={0.3,0.12}, clip=R})
end

--@ chunk 188
-- test: kill the cream halo along the tall spire by veiling its rim with the cloud's own grey
haloR = (townM4:grow(5) - townM4:grow(0.4)) * cloud2M:grow(6) * rect(500,300,320,104)
local hv = pile{{"lead white",5},{"pale smalt",1.5},{"smalt",0.25},{"red earth",0.2},{"raw umber",0.2},{"yellow ochre",0.1}, medium=0.35}
work(haloR * rect(588,318,32,86), {hand="detail", pile=hv, tool={kind="round", width=1.2, point=0.3}, angle=math.pi/2, coverage=2.5, fill=true, clip=true, edge="found", pressure={0.4,0.6}, seed=3721})

--@ chunk 189
-- repaint the cloud close around the tall spire in its own value zones, cut crisply to the tower
local town = townM4:grow(0.25) + townM:grow(0.2) + townM2:grow(0.2)
local near = ((townM4:grow(9) - town) * cloudC * rect(580,316,50,88)):soften(0)
nearS = near
local function Z(lo, hi, s) return cloudV:map(function(v) return smoothstep(lo - s, lo + s, v) * smoothstep(hi + s, hi - s, v) end) * near end
local ang = cloud2F:field("across")
local zH = Z(0.47, 0.58, 0.015)
local zQ = Z(0.36, 0.47, 0.015)
local zS = Z(0.24, 0.36, 0.015)
local zD = Z(-1, 0.24, 0.015)
local zL = cLz * near
for _, t in ipairs({{zD,kD},{zS,kS2},{zQ,kQ},{zH,kH},{zL,kL}}) do
  if t[1]:area() > 1 then
    work(t[1]:grow(0.8) * near, {hand="detail", tool={kind="round", width=1.6, point=0.3}, pile=t[2], angle=ang, coverage=3, fill=true, clip=near, pressure={0.5,0.75}, seed=3730 + _})
  end
end
blend(near:shrink(0.5), {tool={kind="badger", width=5}, angle=math.pi/2, coverage=1.2, clip=true, pressure={0.2,0.35}, seed=3736})

--@ chunk 190
print(wait(60*24*2)); print(drying(600,360))

--@ chunk 191
-- a small flock grazing in the left meadow, with the shepherd and his dog
sheep = {}
local pos = {{66,499},{74,502.5},{83,498.5},{90,503},{97,500},{104,505},{112,501},{121,506.5},{128,502},{140,504.5},{86,508},{151,507},{160,503.5}}
local shM, bodyM, headM = nil, nil, nil
for i, p in ipairs(pos) do
  local x, y = p[1] + rand(-1,1), p[2] + rand(-0.6,0.6)
  local dir = (rand(0,1) < 0.5) and -1 or 1
  local sc = 0.85 + (y - 495) * 0.02
  local b = ellipse(x, y, 1.9*sc, 1.05*sc)
  local h = ellipse(x + dir*2.1*sc, y + 0.5*sc, 0.65*sc, 0.55*sc)
  local s = ellipse(x + 1.3*sc, y + 1.0*sc, 2.4*sc, 0.55*sc)
  sheep[#sheep+1] = {x, y, dir, sc}
  bodyM = bodyM and (bodyM + b) or b
  headM = headM and (headM + h) or h
  shM = shM and (shM + s) or s
end
sheepBody, sheepHead, sheepShadow = bodyM, headM, shM
local gsh = pile{{"Prussian blue",0.5},{"raw umber",1},{"green earth",0.8},{"yellow ochre",0.6},{"lead white",0.4}, medium=0.15}
work(shM - bodyM - headM, {hand="detail", pile=gsh, tool={kind="round", width=0.8, point=0.4}, angle=0, coverage=2, fill=true, clip=true, pressure={0.35,0.6}, seed=3801})
local wool = pile{{"lead white",4},{"yellow ochre",0.5},{"raw umber",0.15},{"pale smalt",0.1}, medium=0.05}
work(bodyM, {hand="detail", pile=wool, tool={kind="round", width=0.9, point=0.4}, angle=0, coverage=3.5, fill=true, clip=true, pressure={0.45,0.7}, seed=3802})
local hd = pile{{"raw umber",1},{"bone black",0.4},{"lead white",0.4}, medium=0.05}
work(headM, {hand="detail", pile=hd, tool={kind="round", width=0.7, point=0.4}, angle=0, coverage=3, fill=true, clip=true, pressure={0.4,0.6}, seed=3803})

--@ chunk 192
print(wait(90))
-- shepherd standing, leaning on his crook, facing the flock (left); dog lying by him
local coat = poly({{177.2,500.2},{179.4,500.2},{180.2,504.8},{176.6,504.8}})
local legs = poly({{177.4,504.6},{179.6,504.6},{179.4,507},{177.6,507}})
local head = ellipse(178.3,499.2,0.8,0.85)
local hat = poly({{176.9,498.6},{179.7,498.6},{179.2,498.0},{177.4,498.0}})
local sh = ellipse(181.5,507.2,3.2,0.6)
local dog = ellipse(171.5,506.2,1.6,0.75) + ellipse(170.0,505.6,0.6,0.55)
local dsh = ellipse(172.8,506.9,1.8,0.4)
local gsh = pile{{"Prussian blue",0.5},{"raw umber",1},{"green earth",0.8},{"yellow ochre",0.6},{"lead white",0.4}, medium=0.15}
work(sh + dsh - legs - dog, {hand="detail", pile=gsh, tool={kind="round", width=0.8, point=0.4}, angle=0, coverage=2, fill=true, clip=true, pressure={0.35,0.6}, seed=3811})
local cp = pile{{"raw umber",1},{"smalt",0.3},{"bone black",0.3},{"lead white",0.5}, medium=0.05}
work(coat + hat, {hand="detail", pile=cp, tool={kind="round", width=0.7, point=0.4}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, pressure={0.45,0.7}, seed=3812})
local lg = pile{{"raw umber",1},{"yellow ochre",0.4},{"lead white",0.8}, medium=0.05}
work(legs, {hand="detail", pile=lg, tool={kind="round", width=0.6, point=0.4}, angle=math.pi/2, coverage=3, fill=true, clip=true, pressure={0.4,0.6}, seed=3813})
local sk = pile{{"lead white",1.5},{"red earth",0.35},{"yellow ochre",0.4},{"raw umber",0.1}, medium=0.05}
work(head, {hand="detail", pile=sk, tool={kind="round", width=0.6, point=0.4}, angle=0, coverage=3, fill=true, clip=true, pressure={0.4,0.6}, seed=3814})
local dg = pile{{"raw umber",1},{"bone black",0.5},{"lead white",0.3}, medium=0.05}
work(dog, {hand="detail", pile=dg, tool={kind="round", width=0.6, point=0.4}, angle=0, coverage=3, fill=true, clip=true, pressure={0.4,0.6}, seed=3815})
local r = brush{kind="rigger", width=0.5, point=1}
r:reload(dg, 0.5)
r:stroke({{175.6,497.4},{175.9,502},{176.2,507.2}}, {pressure={0.35,0.3}})
r:stroke({{175.6,497.4},{174.8,497.0},{174.6,497.9}}, {pressure={0.3,0.2}})

--@ chunk 193
-- a hay wagon being loaded on the mown meadow, a bay horse in the shafts
local loadM = poly({{791.5,537.2},{791,533},{792.6,529.6},{796,527.6},{801,526.8},{806,527.2},{810.4,528.8},{813,531.8},{813.4,537.8}}, true)
local bedM = poly({{791.2,536.8},{813.6,537.6},{813.4,538.9},{791.2,538.1}})
local shadowM = poly({{793,540.6},{816,541.8},{826,542.6},{822,543.8},{806,543.2},{794,542}})
wagonLoad, wagonBed = loadM, bedM
local hayShadow = pile{{"yellow ochre",1},{"raw umber",0.6},{"green earth",0.4},{"lead white",0.5},{"pale smalt",0.2}, medium=0.15}
work(shadowM, {hand="detail", pile=hayShadow, tool={kind="round", width=1.0, point=0.4}, angle=0.05, coverage=2.2, fill=true, clip=true, pressure={0.4,0.6}, seed=3821})
local hayL = pile{{"yellow ochre",1.2},{"lead white",1.3},{"raw umber",0.1},{"chrome yellow",0.08}, medium=0.05}
local hayS = pile{{"yellow ochre",1},{"raw umber",0.45},{"lead white",0.7},{"pale smalt",0.15}, medium=0.05}
work(loadM, {hand="detail", pile=hayL, tool={kind="round", width=1.1, point=0.4}, angle=0.3, coverage=3.5, fill=true, clip=true, pressure={0.45,0.7}, seed=3822})
local shade = loadM * mask(function(x,y) return smoothstep(0, 1, (x - 804)/5 + (y - 530)/4) end)
work(shade, {hand="detail", pile=hayS, tool={kind="round", width=1.0, point=0.4}, angle=0.3, coverage=3, fill=true, clip=true, pressure={0.4,0.65}, seed=3823})
local wd = pile{{"raw umber",1},{"bone black",0.35},{"red earth",0.2},{"lead white",0.3}, medium=0.05}
work(bedM, {hand="detail", pile=wd, tool={kind="round", width=0.7, point=0.4}, angle=0.04, coverage=3.5, fill=true, clip=true, pressure={0.45,0.65}, seed=3824})

--@ chunk 194
print(wait(60*4))
local wd = pile{{"raw umber",1},{"bone black",0.5},{"red earth",0.15},{"lead white",0.2}, medium=0.05}
-- wheels
local w1 = ellipse(795.2,539.6,2.1,2.3) - ellipse(795.2,539.6,1.2,1.35)
local w2 = ellipse(808.6,540.3,2.1,2.3) - ellipse(808.6,540.3,1.2,1.35)
work(w1 + w2, {hand="detail", pile=wd, tool={kind="round", width=0.6, point=0.5}, angle=0, coverage=3.5, fill=true, clip=true, pressure={0.4,0.6}, seed=3831})
-- deeper shade on the load's right and underside
local hayD = pile{{"yellow ochre",1},{"raw umber",0.8},{"lead white",0.5},{"pale smalt",0.25}, medium=0.05}
local sh = wagonLoad * mask(function(x,y) return smoothstep(0.2, 1.2, (x - 806)/5 + (y - 531)/3) end)
work(sh, {hand="detail", pile=hayD, tool={kind="round", width=0.9, point=0.4}, angle=0.4, coverage=3, fill=true, clip=true, pressure={0.4,0.6}, seed=3832})
-- lit top wisps
local hayT = pile{{"lead white",2},{"yellow ochre",1},{"chrome yellow",0.1}, medium=0.05}
local r = brush{kind="rigger", width=0.5, point=1}
r:reload(hayT, 0.5)
for i = 1, 9 do
  local x = 792.5 + i * 1.5 + rand(-0.4, 0.4)
  local y = 530.5 - math.sin((i/10) * math.pi) * 2.6 + rand(-0.3, 0.3)
  r:stroke({{x,y},{x+1.4,y-0.5+rand(-0.2,0.2)},{x+2.8,y-0.2}}, {pressure={0.35,0.05}})
end
-- the horse in the shafts, facing left
local hb = poly({{781,533.4},{788.6,533.2},{789.4,534.4},{789,536.4},{781.4,536.6},{780.6,535}}, true)
local hn = poly({{781.4,534.4},{779.6,531.4},{778.4,530.8},{777.0,532.4},{777.5,533.0},{778.8,532.6},{780.4,535.4}})
local horse = hb + hn
horseW = horse
local bay = pile{{"red earth",1},{"raw umber",0.9},{"lead white",0.35}, medium=0.05}
work(horse, {hand="detail", pile=bay, tool={kind="round", width=0.7, point=0.5}, angle=0, coverage=3.5, fill=true, clip=true, pressure={0.4,0.6}, seed=3833})
r:reload(bay, 0.6)
for _, lx in ipairs({{781.6,-0.3},{783,0.4},{787.4,-0.2},{788.6,0.5}}) do
  r:stroke({{lx[1],536},{lx[1]+lx[2]*0.5,538},{lx[1]+lx[2],540.3}}, {pressure={0.55,0.45}})
end
r:reload(wd, 0.6)
r:stroke({{789.2,535.2},{792,536.4}}, {pressure={0.3,0.3}})             -- shaft
r:stroke({{789.6,534.0},{789.8,536.8}}, {pressure={0.25,0.2}})           -- tail
r:stroke({{779.6,531.2},{781.4,532.6},{783.6,533.4}}, {pressure={0.3,0.2}}) -- mane

--@ chunk 195
-- a loader standing on the hay, and a man below pitching up a forkful
local shirt = pile{{"lead white",3},{"yellow ochre",0.2},{"pale smalt",0.2}, medium=0.05}
local dk = pile{{"raw umber",1},{"bone black",0.4},{"lead white",0.35}, medium=0.05}
local sk = pile{{"lead white",1.5},{"red earth",0.4},{"yellow ochre",0.4},{"raw umber",0.1}, medium=0.05}
local t1 = poly({{801.6,523.2},{803.4,523.2},{803.7,526.0},{801.3,526.0}})
local t1l = poly({{801.5,525.8},{803.5,525.8},{803.3,527.4},{801.7,527.4}})
local t2 = poly({{816.3,535.4},{818.3,535.4},{818.6,538.6},{816.0,538.6}})
local t2l = poly({{816.2,538.4},{818.4,538.4},{818.2,541.6},{816.4,541.6}})
work(t1 + t2, {hand="detail", pile=shirt, tool={kind="round", width=0.6, point=0.5}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, pressure={0.4,0.6}, seed=3841})
work(t1l + t2l, {hand="detail", pile=dk, tool={kind="round", width=0.6, point=0.5}, angle=math.pi/2, coverage=3.5, fill=true, clip=true, pressure={0.4,0.6}, seed=3842})
work(ellipse(802.5,522.4,0.65,0.7) + ellipse(817.3,534.6,0.65,0.7), {hand="detail", pile=sk, tool={kind="round", width=0.5, point=0.5}, angle=0, coverage=3, fill=true, clip=true, pressure={0.4,0.6}, seed=3843})
local r = brush{kind="rigger", width=0.45, point=1}
r:reload(dk, 0.5)
r:stroke({{801.8,521.8},{803.2,521.6}}, {pressure={0.4,0.4}})           -- hat
r:stroke({{816.6,534.0},{818.0,533.8}}, {pressure={0.4,0.4}})
r:reload(pile{{"raw umber",1},{"yellow ochre",0.5},{"lead white",0.6}}, 0.5)
r:stroke({{816.8,537},{814.2,532},{812.4,528.2}}, {pressure={0.3,0.25}}) -- fork shaft raised to the load
r:stroke({{804,524.6},{806.2,522.4},{807.6,520.8}}, {pressure={0.25,0.2}})
local hay = pile{{"yellow ochre",1.2},{"lead white",1.3},{"chrome yellow",0.08}, medium=0.05}
r:reload(hay, 0.5)
r:stroke({{811.4,527.8},{812.6,527.2},{813.8,527.6}}, {pressure={0.55,0.3}}) -- the forkful
local gsh = pile{{"yellow ochre",1},{"raw umber",0.6},{"green earth",0.4},{"lead white",0.5},{"pale smalt",0.2}, medium=0.15}
work(ellipse(820.5,541.9,2.6,0.5) - t2l, {hand="detail", pile=gsh, tool={kind="round", width=0.6, point=0.4}, angle=0, coverage=2, fill=true, clip=true, pressure={0.35,0.5}, seed=3844})

--@ chunk 196
-- the cream halos are the old, wider town masks that every cloud repaint excluded; paint them as cloud now
local cut = rect(0,0,1000,414)
local hz = (((townM + townM2 + townM3):grow(2.5)) - townM4:grow(0.2)) * cloud2M:grow(9) * cut
haloZ = hz
print(hz:area())
local function Z(lo, hi, s) return cloudV:map(function(v) return smoothstep(lo - s, lo + s, v) * smoothstep(hi + s, hi - s, v) end) * hz end
local zL = cloudV:map(function(v) return smoothstep(0.56, 0.64, v) end) * hz
local list = {{Z(-1,0.24,0.015),kD},{Z(0.24,0.36,0.015),kS2},{Z(0.36,0.47,0.015),kQ},{Z(0.47,0.58,0.015),kH},{zL,kL}}
for i, t in ipairs(list) do
  if t[1]:area() > 1 then
    work(t[1]:grow(0.8) * hz, {hand="detail", tool={kind="round", width=1.4, point=0.3}, pile=t[2], angle=math.pi/2, coverage=3.2, fill=true, clip=hz, edge="found", pressure={0.5,0.75}, seed=3850 + i})
  end
end
blend(hz:shrink(0.4), {tool={kind="badger", width=4}, angle=math.pi/2, coverage=1.0, clip=true, pressure={0.2,0.3}, seed=3856})

--@ chunk 197
print(wait(60*24*3))

--@ chunk 198
local gz = pile{{"lead white",2},{"pale smalt",1.4},{"smalt",0.3},{"red earth",0.2},{"raw umber",0.3}, medium=0.55}
work(haloZ, {hand="detail", tool={kind="round", width=1.4, point=0.3}, pile=gz, angle=math.pi/2, coverage=2.0, fill=true, clip=true, edge="found", pressure={0.35,0.55}, seed=3861})
blend(haloZ:shrink(0.4), {tool={kind="badger", width=4}, angle=math.pi/2, coverage=1.0, clip=true, pressure={0.2,0.3}, seed=3862})

--@ chunk 199
print(wait(60*24*2))
local hz = haloZ
local function Z(lo, hi, s) return cloudV:map(function(v) return smoothstep(lo - s, lo + s, v) * smoothstep(hi + s, hi - s, v) end) * hz end
local zL = cloudV:map(function(v) return smoothstep(0.56, 0.64, v) end) * hz
local list = {{Z(-1,0.24,0.015),kD},{Z(0.24,0.36,0.015),kS2},{Z(0.36,0.47,0.015),kQ},{Z(0.47,0.58,0.015),kH},{zL,kL}}
for i, t in ipairs(list) do
  if t[1]:area() > 1 then
    work(t[1]:grow(0.8) * hz, {hand="detail", tool={kind="round", width=1.4, point=0.3}, pile=t[2], angle=math.pi/2, coverage=1.6, fill=false, clip=hz, edge="found", pressure={0.35,0.55}, load=0.5, seed=3870 + i})
  end
end
blend(hz:shrink(0.4), {tool={kind="badger", width=4}, angle=math.pi/2, coverage=1.0, clip=true, pressure={0.2,0.3}, seed=3876})

--@ chunk 200
print(wait(60*24*2))
local cut = rect(0,0,1000,414)
local hz = (((townM + townM2 + townM3):grow(8)) - townM4:grow(0.2)) * cloud2M:grow(9) * cut * rect(560,300,200,114)
haloW = hz
local function Z(lo, hi, s) return cloudV:map(function(v) return smoothstep(lo - s, lo + s, v) * smoothstep(hi + s, hi - s, v) end) * hz end
local zL = cloudV:map(function(v) return smoothstep(0.56, 0.64, v) end) * hz
local list = {{Z(-1,0.24,0.015),kD},{Z(0.24,0.36,0.015),kS2},{Z(0.36,0.47,0.015),kQ},{Z(0.47,0.58,0.015),kH},{zL,kL}}
local ang = cloud2F:field("across")
for i, t in ipairs(list) do
  if t[1]:area() > 1 then
    work(t[1]:grow(0.8) * hz, {hand="detail", tool={kind="filbert", width=2.5}, pile=t[2], angle=ang, coverage=3, fill=true, clip=hz, pressure={0.45,0.7}, seed=3880 + i})
  end
end
blend(hz:shrink(0.4), {tool={kind="badger", width=6}, angle=ang, coverage=1.3, clip=true, pressure={0.2,0.35}, seed=3886})

--@ chunk 201
print(wait(60*24*5))

--@ chunk 202
local gz = pile{{"lead white",2},{"pale smalt",1.4},{"smalt",0.3},{"red earth",0.2},{"raw umber",0.3}, medium=0.55}
local m = (haloW:shrink(1.5):blur(3)) - townM4:grow(0.2)
local ang = cloud2F:field("across")
work(m, {hand="detail", tool={kind="filbert", width=3, stiffness=0.2}, pile=gz, angle=ang, coverage=2.0, fill=true, clip=m, pressure={0.3,0.5}, seed=3891})
blend(m, {tool={kind="badger", width=6}, angle=ang, coverage=1.2, clip=m, pressure={0.2,0.3}, seed=3892})

--@ chunk 203
print(wait(60*24*3))
print(drying(600,360), drying(700,300))
-- lesson again: no local sky patches. Repaint the whole cumulus body in its zones, now cut only by the present town
local cut = rect(0,0,1000,414)
cloudC3 = (cloud2M:grow(9) * cut) - townM4:grow(0.2)
local function Z(lo, hi, s) return cloudV:map(function(v) return smoothstep(lo - s, lo + s, v) * smoothstep(hi + s, hi - s, v) end) * cloudC3 end
local ang = cloud2F:field("across")
local zL = cloudV:map(function(v) return smoothstep(0.56, 0.64, v) end) * cloudC3
local list = {{Z(-1,0.24,0.015),kD},{Z(0.24,0.36,0.015),kS2},{Z(0.36,0.47,0.015),kQ},{Z(0.47,0.58,0.015),kH},{zL,kL}}
for i, t in ipairs(list) do
  work(t[1]:grow(1) * cloudC3, {hand="body", tool="filbert 4", pile=t[2], angle=ang, length={8,22}, coverage=2.6, fill=true, clip=cloudC3, pressure={0.5,0.75}, seed=3900 + i})
end

--@ chunk 204
blend(cloudC3:shrink(1.5), {tool={kind="badger", width=12}, angle=cloud2F:field("across"), coverage=1.2, clip=true, pressure={0.2,0.35}, seed=3911})

--@ chunk 205
print(wait(60*24*3))

--@ chunk 206
-- a thin cool glaze over the town so it stands a half-step darker than the cloud behind
local tg = pile{{"smalt",1},{"raw umber",0.5},{"red earth",0.15},{"lead white",0.6}, medium=0.7}
work(townM4:shrink(0.3), {hand="detail", pile=tg, tool={kind="round", width=1.0, point=0.5}, angle=math.pi/2, coverage=2.5, fill=true, clip=true, edge="found", pressure={0.35,0.55}, seed=3921})

--@ chunk 207
blend(townM4:shrink(0.4), {tool={kind="badger", width=4}, angle=0, coverage=1.2, clip=true, pressure={0.2,0.3}, seed=3922})

--@ chunk 208
print(wait(60*24*4))

--@ chunk 209
print(drying(380,340), drying(300,300), drying(700,300), drying(800,520)); print(fd, fmd, fm, fl3)

--@ chunk 210
-- the shaded right side of the crown: break the flat dark into leaf clusters, each with a dull top
local cl = {}
local function C(x,y,rx,ry,sd) cl[#cl+1] = {body.ellipsoid({x,y,0},{rx,ry,(rx+ry)/2}):rough(ry*0.18, ry*0.8, sd)} end
C(318,205,22,15,1) C(352,222,20,14,2) C(336,250,18,12,3) C(372,262,22,15,4)
C(300,272,16,11,5) C(392,300,20,14,6) C(350,292,24,14,7) C(318,318,18,12,8)
C(404,338,18,13,9) C(366,332,22,14,10) C(330,356,20,13,11) C(390,372,22,14,12)
C(300,388,18,12,13) C(352,392,24,14,14) C(410,410,18,12,15) C(330,425,22,13,16)
C(376,430,20,12,17) C(298,440,16,10,18) C(386,240,14,10,19) C(342,450,18,9,20)
cl.light = {from={-1,-0.8}, front=0.25, ambient=0.05}
clF = form(cl)
local R = oakNew:shrink(2) * mask(function(x,y) return smoothstep(285, 310, x) end)
local nz = noise{seed=4101, octaves=3, period=7}
local lit = (clF:lit{soft=0.25} * R):map(function(v) return smoothstep(0.35, 0.85, v) end)
dullG = pile{{"yellow ochre",1.4},{"Prussian blue",0.45},{"raw umber",0.6},{"lead white",0.35},{"green earth",0.4}, medium=0.04}
stipple(lit, {pile=dullG, width=2.4, coverage=function(x,y) return clamp(0.9*lit:at(x,y) + 0.45*nz(x,y), 0, 1.1) end, pressure={0.3,0.6}, drag={1.0,-0.8}, twist=0.5, cluster={0.7,3}, feather=0.8, dips={12,0.5,0.2}, clip=R, seed=4102})

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
