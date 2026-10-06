-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=450, aspect=1.42, linen={15,13}, seed=1806, ground={{pile={{"red earth",2},{"yellow ochre",3},{"lead white",1}}, um=90, apply="knife", texture=0.35},{pile={{"lead white",10},{"yellow ochre",0.4},{"raw umber",0.12}}, um=45, apply="brush"}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
h = pencil("HB")
HZ = 468
h:rule({0,HZ},{1000,HZ},{pressure=0.25})
-- ground undulation left
h:sketch({{0,462},{80,460},{180,457},{260,462},{340,467}},{pressure=0.25})
-- town
h:sketch({{560,466},{565,456},{590,452},{600,452}},{pressure=0.25})
h:line({{593,468},{593,432},{600,424},{607,432},{607,468}},{pressure=0.3,smooth=false})
h:line({{674,468},{674,426},{686,426},{686,468}},{pressure=0.3,smooth=false})
h:line({{675,426},{676,412},{684,412},{685,426}},{pressure=0.3,smooth=false})
h:sketch({{676,412},{674,406},{680,400},{686,406},{684,412}},{pressure=0.3})
h:line({{680,400},{680,386}},{pressure=0.3})
h:line({{755,468},{755,438},{765,438},{765,468}},{pressure=0.3,smooth=false})
h:line({{755,438},{760,402},{765,438}},{pressure=0.3,smooth=false})
h:sketch({{610,452},{620,444},{660,444},{670,452}},{pressure=0.25})
h:sketch({{690,454},{700,446},{745,446},{752,454}},{pressure=0.25})
h:sketch({{770,458},{800,455},{820,466}},{pressure=0.25})
-- windmills
h:line({{175,460},{178,438},{190,438},{193,460}},{pressure=0.3,smooth=false})
h:line({{168,418},{200,458}},{pressure=0.25}); h:line({{204,420},{165,456}},{pressure=0.25})
h:line({{886,466},{888,452},{895,452},{897,466}},{pressure=0.25,smooth=false})
-- river channel
h:sketch({{360,490},{500,482},{700,478},{1000,476}},{pressure=0.25})
h:sketch({{380,496},{520,489},{720,484},{1000,482}},{pressure=0.25})
-- willow ditch
h:sketch({{0,540},{120,524},{260,508},{380,498}},{pressure=0.25})
for i,p in ipairs({{40,528},{95,522},{150,516},{205,511},{258,506},{305,502},{350,499}}) do
  local s = 1 - i*0.08
  h:sketch({{p[1]-4*s,p[2]},{p[1]-3*s,p[2]-30*s}},{pressure=0.25})
  h:sketch({{p[1]+4*s,p[2]},{p[1]+3*s,p[2]-30*s}},{pressure=0.25})
  h:sketch({{p[1]-22*s,p[2]-30*s},{p[1],p[2]-58*s},{p[1]+22*s,p[2]-30*s}},{pressure=0.2})
end
-- path
h:sketch({{470,704},{490,660},{470,610},{430,570},{420,540}},{pressure=0.25})
h:sketch({{620,704},{590,660},{540,610},{470,570},{440,540}},{pressure=0.25})
-- figures
h:sketch({{505,632},{503,600},{498,585},{503,570},{510,570},{514,585},{512,600},{511,632}},{pressure=0.3})
h:sketch({{503,570},{502,560},{512,560},{510,570}},{pressure=0.3})
h:sketch({{520,634},{518,600},{524,578},{530,578},{536,600},{540,634}},{pressure=0.3})
h:sketch({{524,578},{525,568},{532,568},{531,578}},{pressure=0.3})
-- fence
h:sketch({{690,612},{800,600},{1000,590}},{pressure=0.25})
h:sketch({{690,628},{800,618},{1000,612}},{pressure=0.25})

--@ chunk 3
skyTop = pile{{"lead white",3},{"smalt",2.2},{"cobalt blue",0.8}, medium=0.1}
skyMid = pile{{"lead white",6},{"smalt",1.6},{"cobalt blue",0.3}, medium=0.1}
skyLow = pile{{"lead white",10},{"pale smalt",1.2},{"smalt",0.3}, medium=0.1}
skyHz = pile{{"lead white",12},{"yellow ochre",0.35},{"vermilion",0.05},{"pale smalt",0.3}, medium=0.1}
local function band(a,b,s) return mask(function(x,y) return smoothstep(a-s,a+s,y)*(1-smoothstep(b-s,b+s,y)) end) end
SKY = below(function(x) return -1 end) * above(function(x) return 474 end)
local bands = {{skyTop,-40,150},{skyMid,140,290},{skyLow,280,400},{skyHz,390,480}}
for i,bd in ipairs(bands) do
  work(band(bd[2],bd[3],25)*SKY, {hand="body", pile=bd[1], tool="filbert 12", angle=function(x,y) return 0.03*math.sin(x/170+y/90) end, angle_jitter=0.05, length={50,120}, coverage=1.6, fill=true})
end
print(wait(0))

--@ chunk 4
skyT2 = pile{{"lead white",4},{"smalt",2},{"cobalt blue",0.5}, medium=0.1}
skyT3 = pile{{"lead white",8},{"smalt",0.9},{"pale smalt",0.8}, medium=0.1}
stipple(SKY, {pile=skyT2, width=5, coverage=function(x,y) return 2.2*smoothstep(90,150,y)*(1-smoothstep(200,320,y)) end, pressure={0.5,0.8}, cluster=0.2})
stipple(SKY, {pile=skyT3, width=5, coverage=function(x,y) return 2.0*smoothstep(230,290,y)*(1-smoothstep(330,420,y)) end, pressure={0.5,0.8}, cluster=0.2})
blend(SKY, {angle=0, coverage=2.2})
blend(SKY, {angle=0.02, coverage=1.5})
print(wait(0))

--@ chunk 5
print(wait(60*44)); print(drying(500,50), drying(500,200), drying(500,440))

--@ chunk 6
SK = {
 pile{{"lead white",3},{"smalt",2.6},{"cobalt blue",1.1}, medium=0.14},
 pile{{"lead white",3.5},{"smalt",2.2},{"cobalt blue",0.8}, medium=0.14},
 pile{{"lead white",4.5},{"smalt",2.0},{"cobalt blue",0.55}, medium=0.14},
 pile{{"lead white",5.5},{"smalt",1.8},{"cobalt blue",0.35}, medium=0.14},
 pile{{"lead white",7},{"smalt",1.5},{"pale smalt",0.5}, medium=0.14},
 pile{{"lead white",8.5},{"smalt",1.0},{"pale smalt",0.9}, medium=0.14},
 pile{{"lead white",10},{"pale smalt",1.3},{"smalt",0.4},{"yellow ochre",0.06}, medium=0.14},
 pile{{"lead white",12},{"pale smalt",1.0},{"yellow ochre",0.22},{"vermilion",0.03}, medium=0.14}}
SKYB = {{-40,70},{55,130},{115,190},{175,250},{235,310},{295,370},{355,420},{405,480}}
function yband(a,b,s) return mask(function(x,y) return smoothstep(a-s,a+s,y)*(1-smoothstep(b-s,b+s,y)) end) end
function skyband(i)
  local bd = SKYB[i]
  work(yband(bd[1],bd[2],12)*SKY, {hand="body", pile=SK[i], tool="filbert 12", angle=function(x,y) return 0.03*math.sin(x/170+y/90) end, angle_jitter=0.04, length={60,140}, coverage=1.5, fill=true, load=0.5})
end
for i=1,3 do skyband(i) end
blend(yband(-40,200,20)*SKY, {angle=0, coverage=2.2})
print(wait(0))

--@ chunk 7
for i=4,6 do skyband(i) end
blend(yband(150,375,20)*SKY, {angle=0, coverage=2.2})
for i=7,8 do skyband(i) end
blend(yband(340,490,20)*SKY, {angle=0, coverage=2.2})
blend(SKY, {angle=0.01, coverage=1.0})
print(wait(0))

--@ chunk 8
cloudLit = pile{{"lead white",14},{"yellow ochre",0.12},{"vermilion",0.025}, medium=0.08}
cloudSh = pile{{"lead white",8},{"pale smalt",1.1},{"raw umber",0.18},{"red earth",0.06}, medium=0.1}
local c = ellipse(90,370,70,28) + ellipse(160,352,60,40) + ellipse(230,362,55,32) + ellipse(290,378,50,22) + ellipse(125,385,90,18) + ellipse(200,330,35,30)
c = c * above(function(x) return 398 + 4*math.sin(x/40) end)
CUM = c:roughen(8, 30, 11)
local base = CUM * below(function(x) return 362 - 0.1*(x-150) end)
work(base, {hand="body", pile=cloudSh, tool="filbert 6", angle=0, angle_jitter=0.2, length={12,30}, coverage=1.6, fill=true, load=0.6})
local top = CUM - below(function(x) return 375 - 0.1*(x-150) end)
work(top, {hand="scumble", pile=cloudLit, tool="filbert 6", coverage=1.8, load=0.7, fill=true})
STRAT = (ribbon({{520,236},{640,230},{780,233},{900,226},{990,228}}, {6,10,9,7,4}) + ribbon({{380,322},{500,318},{620,321},{700,317}}, {3,7,8,4}) + ribbon({{700,392},{820,388},{960,393},{1010,390}},{4,9,10,8})):roughen(3,20,5)
work(STRAT, {hand="body", pile=cloudLit, tool="filbert 5", angle=-0.01, length={30,70}, coverage=1.4, load=0.5, fill=true})
blend(CUM:grow(4), {angle=0, coverage=1.2, tool={kind="badger", width=14}, pressure={0.2,0.35}})
blend(STRAT:grow(4), {angle=0, coverage=1.2, tool={kind="badger", width=14}, pressure={0.2,0.35}})
print(wait(0))

--@ chunk 9
cloudSh2 = pile{{"lead white",6},{"pale smalt",1.6},{"smalt",0.35},{"raw umber",0.22},{"red earth",0.05}, medium=0.1}
cloudW = pile{{"lead white",1}, medium=0.05}
local sh = CUM * below(function(x) return 368 + 0.06*(x-150) + 8*math.sin(x/33) end)
stipple(sh:shrink(2), {pile=cloudSh2, width=5, coverage=2.2, pressure={0.5,0.8}, cluster=0.4, dips={10,0.7,0.4}})
local lit = CUM - CUM:offset(-9)
lit = lit * mask(function(x,y) return y < 372 and 1 or 0 end)
stipple(lit, {pile=cloudW, width=4, coverage=2.5, pressure={0.5,0.8}, cluster=0.3, dips={10,0.7,0.4}})
blend(CUM:grow(8), {angle=0, coverage=1.6, tool={kind="badger", width=18}, pressure={0.3,0.5}})
blend(STRAT:grow(12), {angle=0, coverage=2.0, tool={kind="badger", width=20}, pressure={0.35,0.55}})
blend(STRAT:grow(12), {angle=0.3, coverage=1.0, tool={kind="badger", width=20}, pressure={0.3,0.5}})
print(wait(0))

--@ chunk 10
print(wait(60*24)); print(drying(150,360), drying(500,460), drying(600,230), drying(500,50))

--@ chunk 11
function hzc(x)
  if x < 340 then return 462 - 5*math.exp(-((x-180)/70)^2) + 6*smoothstep(240,340,x) - 6 + 6*smoothstep(-100,0,x)*0 end
  return 468 end
LAND = below(function(x) return hzc(x) end)
print(hzc(0), hzc(180), hzc(300), hzc(340))
under = pile{{"raw umber",1},{"yellow ochre",1.6},{"green earth",1.6}, medium=0.45}
work(LAND, {hand="glaze", pile=under, clip=true, angle=0.05, coverage=1.3, load=0.5})
print(wait(0))

--@ chunk 12
function hzc(x) return 468 - 11*math.exp(-((x-170)/130)^2) end
LAND = below(function(x) return hzc(x) end)
blend(LAND, {angle=0, coverage=2.5})
blend(LAND, {angle=0.6, coverage=1.5})
blend(LAND, {angle=-0.1, coverage=1.5})
print(wait(0))

--@ chunk 13
print(wait(60*30)); print(drying(500,600), drying(150,360), drying(600,230), drying(500,690))

--@ chunk 14
function hzc(x) return 468 - (13 + 5*math.exp(-((x-170)/80)^2)) * (1 - smoothstep(300, 370, x)) end
LAND = below(hzc)
function rtop(x) return 485.5 - 5*(x-330)/670 + 1.2*math.sin(x/37) end
function rbot(x) return 487 + 6*(x-330)/670 + 1.5*math.sin(x/53+1) end
RIVER = (below(rtop) * above(rbot) * mask(function(x,y) return smoothstep(328,350,x) end)):soften(0.6)
local Lp = {
 {pile{{"lead white",5},{"pale smalt",1.2},{"green earth",1.2},{"yellow ochre",0.6},{"raw umber",0.2}, medium=0.12}, 430, 489, "filbert 6", {10,30}},
 {pile{{"lead white",4},{"yellow ochre",1.4},{"green earth",1},{"Prussian blue",0.08},{"pale smalt",0.4}, medium=0.12}, 484, 522, "filbert 7", {15,40}},
 {pile{{"lead white",3},{"yellow ochre",2},{"chrome yellow",0.6},{"Prussian blue",0.18},{"green earth",0.5}, medium=0.12}, 516, 572, "filbert 8", {20,50}},
 {pile{{"lead white",1.5},{"yellow ochre",2},{"chrome yellow",0.4},{"Prussian blue",0.25},{"green earth",1}, medium=0.12}, 566, 618, "filbert 9", {25,60}},
 {pile{{"lead white",0.4},{"yellow ochre",1.5},{"Prussian blue",0.35},{"raw umber",0.8},{"green earth",1.2}, medium=0.12}, 612, 662, "filbert 10", {25,60}},
 {pile{{"yellow ochre",1.2},{"Prussian blue",0.4},{"raw umber",1.2},{"green earth",1},{"bone black",0.1}, medium=0.12}, 656, 740, "filbert 11", {30,70}},
}
LANDP = Lp
for i,l in ipairs(Lp) do
  local m = yband(l[2], l[3], 6) * LAND - RIVER
  work(m, {hand="body", pile=l[1], tool=l[4], clip=LAND, angle=function(x,y) return -0.02 + 0.04*math.sin(x/140) end, angle_jitter=0.06, length=l[5], coverage=1.6, fill=true, load=0.55})
end
print(wait(0))

--@ chunk 15
local m = LAND - RIVER:grow(1)
blend(m * mask(function(x,y) return y > 490 and 1 or 0 end), {angle=1.35, coverage=1.6, tool={kind="badger", width=24}})
blend(m, {angle=0, coverage=1.5, tool={kind="badger", width=24}})
print(wait(0))

--@ chunk 16
print(wait(60*40)); print(drying(500,480), drying(500,550), drying(500,690), drying(150,360))

--@ chunk 17
print(wait(60*24)); print(drying(500,480), drying(500,550), drying(500,690), drying(150,360))

--@ chunk 18
cloudMid = pile{{"lead white",9},{"pale smalt",0.9},{"raw umber",0.08},{"vermilion",0.01}, medium=0.1}
local low = CUM * mask(function(x,y) return smoothstep(350, 385, y + 0.08*(x-150)) end)
local mid = CUM * mask(function(x,y) return smoothstep(335, 360, y)*(1-smoothstep(372,392,y)) end)
stipple(mid, {pile=cloudMid, width=4, coverage=1.5, pressure={0.4,0.7}, cluster=0.5, feather=0.5})
stipple(low, {pile=cloudSh2, width=4, coverage=2.0, pressure={0.45,0.75}, cluster=0.4, feather=0.6})
local rim = (CUM - CUM:shrink(7)) * mask(function(x,y) return 1 - smoothstep(345, 365, y) end)
stipple(rim, {pile=cloudW, width=3.5, coverage=2.2, pressure={0.5,0.8}, cluster=0.3})
blend(CUM:grow(3), {angle=0, coverage=1.2, tool={kind="badger", width=12}, pressure={0.2,0.35}})
print(wait(0))

--@ chunk 19
blend(CUM:grow(2), {angle=0.1, coverage=1.8, tool={kind="badger", width=16}, pressure={0.35,0.5}})
print(wait(0))

--@ chunk 20
riverP = pile{{"lead white",8},{"pale smalt",1.1},{"smalt",0.25},{"yellow ochre",0.04}, medium=0.1}
work(RIVER, {hand="detail", pile=riverP, angle=0, length={8,24}, coverage=2, fill=true, clip=true})
-- far tree line along horizon
local n = noise{seed=41, period=14, octaves=3}
FARTREES = mask(function(x,y)
  if x < 330 then return 0 end
  local top = 463.5 - 2.5*n(x,0) - 2*math.max(0, math.sin(x/23)) 
  return (y > top and y < 470) and smoothstep(330,380,x) or 0 end)
farTreeP = pile{{"lead white",5},{"pale smalt",1.3},{"green earth",0.8},{"raw umber",0.3}, medium=0.1}
work(FARTREES, {hand="detail", pile=farTreeP, angle=0, coverage=2, fill=true, clip=true})
-- town
local T = {}
local function P(pts) T[#T+1] = poly(pts) end
P{{560,470},{560,458},{566,452},{572,458},{580,458},{580,454},{590,454},{590,470}}
-- Marien: squat tower + nave
P{{593,470},{593,432},{596,432},{600,424},{604,432},{607,432},{607,470}}
P{{607,470},{607,452},{612,444},{664,444},{669,452},{669,470}}
P{{614,452},{618,446},{622,452}}
-- houses between
P{{669,470},{669,456},{672,452},{674,456},{674,470}}
-- Nikolai tower
P{{674,470},{674,426},{686,426},{686,470}}
P{{676,426},{676.5,413},{683.5,413},{684,426}}
T[#T+1] = ellipse(680,406.5,4.8,6.2)
P{{678.6,401},{678.6,394},{681.4,394},{681.4,401}}
T[#T+1] = ellipse(680,392,2.2,2.6)
P{{679.4,391},{680,382},{680.6,391}}
-- Nikolai nave
P{{686,470},{686,453},{691,446},{748,446},{752,453},{752,470}}
-- houses
P{{752,470},{752,458},{757,452},{762,458},{770,458},{770,455},{776,450},{782,455},{782,470}}
-- Jacobi tower + spire
P{{755,470},{755,438},{765,438},{765,470}}
P{{754.5,438.5},{760,401},{765.5,438.5}}
P{{782,470},{782,460},{790,454},{798,460},{806,460},{806,470}}
P{{806,470},{806,462},{812,458},{820,463},{822,470}}
-- trees in town
T[#T+1] = ellipse(585,457,8,6) 
T[#T+1] = ellipse(728,449,9,6)
T[#T+1] = ellipse(800,455,10,7)
T[#T+1] = ellipse(566,461,8,5)
TOWN = T[1]
for i=2,#T do TOWN = TOWN + T[i] end
TOWN = TOWN * above(function(x) return 471 end)
townP = pile{{"lead white",4},{"smalt",1.1},{"raw umber",0.5},{"red earth",0.12}, medium=0.08}
work(TOWN, {hand="detail", pile=townP, tool={kind="round", width=1.6, point=0.5}, angle=math.pi/2, length={3,10}, coverage=2.5, fill=true, clip=true, edge="found"})
print(wait(0))

--@ chunk 21
townP2 = pile{{"lead white",3},{"smalt",1.6},{"bone black",0.22},{"raw umber",0.15},{"pale smalt",0.5}, medium=0.08}
work(TOWN, {hand="detail", pile=townP2, tool={kind="round", width=1.6, point=0.5}, angle=math.pi/2, length={3,10}, coverage=2.5, fill=true, clip=true, edge="found", load=0.7})
print(wait(0))

--@ chunk 22
millP = pile{{"raw umber",1},{"bone black",0.3},{"lead white",1.6},{"smalt",0.4}, medium=0.08}
millL = pile{{"lead white",3},{"yellow ochre",0.5},{"raw umber",0.5},{"red earth",0.1}, medium=0.08}
-- post mill body, turned slightly: lit left face, dark side face
local body = poly{{176,444},{176,429},{183,423},{190,429},{190,444}}
local side = poly{{190,444},{190,429},{194,431},{194,443}}
local post = poly{{182,444},{182,451},{185,451},{185,444}}
local legs = ribbon({{178,452},{183.5,446},{189,452}}, 0.9)
MILL = body + side + post + legs
work(body, {hand="detail", pile=millL, tool={kind="round", width=1.4, point=0.5}, angle=math.pi/2, length={3,8}, coverage=2.5, fill=true, clip=true, edge="found"})
work(side + post + legs, {hand="detail", pile=millP, tool={kind="round", width=1.2, point=0.5}, angle=math.pi/2, length={3,8}, coverage=2.5, fill=true, clip=true, edge="found"})
-- roof shadow line
local r = brush{kind="rigger", width=1.0, point=1}
r:load(millP, 0.5)
r:stroke({{175.5,429.5},{183,423},{190.5,429.5}}, {pressure={0.5,0.4}})
-- hub on the front face, sails as an X, slightly off-diagonal
local hx, hy = 181, 432
local arms = {{-0.72, 34}, {0.85, 34}, {2.42, 33}, {3.99, 34}}
local sb = brush{kind="rigger", width=1.1, point=1}
for i,a in ipairs(arms) do
  local ex, ey = hx + math.cos(a[1])*a[2], hy + math.sin(a[1])*a[2]
  sb:reload(millP, 0.5)
  sb:stroke({{hx,hy},{ex,ey}}, {pressure={0.55,0.35}})
  -- lattice sail: a parallel thin line offset, with cross bars
  local ox, oy = -math.sin(a[1])*3.2, math.cos(a[1])*3.2
  local fb = brush{kind="rigger", width=0.7, point=1}
  fb:load(millP, 0.35)
  local s0 = 0.3
  fb:stroke({{hx+math.cos(a[1])*a[2]*s0+ox, hy+math.sin(a[1])*a[2]*s0+oy},{ex+ox,ey+oy}}, {pressure={0.35,0.3}})
  for k=0,6 do
    local t = s0 + (1-s0)*k/6
    local px, py = hx + math.cos(a[1])*a[2]*t, hy + math.sin(a[1])*a[2]*t
    fb:stroke({{px,py},{px+ox,py+oy}}, {pressure={0.3,0.3}})
  end
end
print(wait(0))

--@ chunk 23
print(wait(60*24)); print(drying(680,440), drying(500,600), drying(500,690), drying(600,487))

--@ chunk 24
function grass(m, p, len, cov, w, jit, seed)
  work(m, {hand="hatch", pile=p, tool={kind="round", width=w, point=0.8}, angle=-math.pi/2 + 0.0, angle_jitter=jit or 0.25, length=len, coverage=cov, pressure={0.35,0.7}, ramps={0.1,0.6}, seed=seed})
end
function hgrass(m, p, len, cov, w, seed)
  work(m, {hand="hatch", pile=p, tool={kind="round", width=w, point=0.6}, angle=0, angle_jitter=0.08, length=len, coverage=cov, pressure={0.4,0.7}, seed=seed})
end
M_rise = LAND * mask(function(x,y) return (y < 492 - 0.02*x) and (1-smoothstep(330,380,x)) or 0 end)
riseP = pile{{"lead white",4},{"yellow ochre",1.6},{"green earth",0.8},{"chrome yellow",0.3},{"Prussian blue",0.05}, medium=0.1}
riseD = pile{{"lead white",2.5},{"green earth",1.4},{"yellow ochre",0.8},{"pale smalt",0.5},{"raw umber",0.2}, medium=0.1}
hgrass(M_rise, riseP, {6,16}, 1.4, 1.6, 1)
hgrass(M_rise * mask(function(x,y) return smoothstep(0.2,0.8,noise{seed=7,period=60}:at01(x,y)) end), riseD, {5,12}, 0.8, 1.4, 2)
M_far = LAND * mask(function(x,y) return (x > 330 and y < rtop(x) + 1) and 1 or 0 end) - TOWN - FARTREES
farP = pile{{"lead white",4},{"green earth",1.2},{"pale smalt",0.7},{"yellow ochre",0.7}, medium=0.1}
hgrass(M_far, farP, {5,14}, 1.5, 1.2, 3)
print(wait(0))

--@ chunk 25
local m = LAND * mask(function(x,y) return (y > 452) and (1-smoothstep(330,380,x)) or 0 end) * above(function(x) return 505 end)
blend(m - MILL:grow(2), {angle=0, coverage=2, tool={kind="badger", width=18}, pressure={0.3,0.5}})
local tone = pile{{"lead white",4},{"green earth",1.4},{"pale smalt",0.9},{"yellow ochre",0.5},{"raw umber",0.12}, medium=0.1}
hgrass(M_rise - MILL:grow(1), tone, {5,14}, 1.2, 1.3, 5)
print(wait(0))

--@ chunk 26
print(wait(60*36)); print(drying(100,470), drying(680,440), drying(600,487), drying(500,600))

--@ chunk 27
print(wait(60*30)); print(drying(100,470), drying(680,440), drying(600,487), drying(500,600))

--@ chunk 28
local nz = noise{seed=77, period=120, octaves=3}
KEEP = TOWN:grow(0.5) + FARTREES + MILL:grow(1) + RIVER
LANDC = LAND - KEEP
local function zone(a, b, amp)
  return mask(function(x,y)
    local yy = y + amp*nz(x, y*3)
    return smoothstep(a-4, a+4, yy) * (1 - smoothstep(b-4, b+4, yy)) end) * LANDC
end
G = {
 far = pile{{"lead white",3},{"green earth",1.4},{"pale smalt",0.7},{"yellow ochre",0.8},{"raw umber",0.1}, medium=0.1},
 lit = pile{{"lead white",1.5},{"yellow ochre",2},{"chrome yellow",0.6},{"Prussian blue",0.15},{"green earth",0.8}, medium=0.1},
 mid = pile{{"lead white",0.8},{"yellow ochre",1.6},{"Prussian blue",0.3},{"green earth",1.2},{"raw umber",0.3}, medium=0.1},
 sh  = pile{{"lead white",0.5},{"yellow ochre",1},{"Prussian blue",0.4},{"raw umber",0.9},{"green earth",1.2}, medium=0.1},
 dark= pile{{"raw umber",1.2},{"Prussian blue",0.45},{"yellow ochre",0.8},{"green earth",1},{"bone black",0.15}, medium=0.1},
}
local Z = {
 {G.far, -100, 499, 3, "filbert 5", {10,30}},
 {G.lit, 497, 521, 5, "filbert 6", {15,40}},
 {G.sh, 519, 546, 6, "filbert 6", {15,45}},
 {G.lit, 544, 598, 8, "filbert 8", {20,55}},
 {G.mid, 596, 640, 10, "filbert 9", {25,60}},
 {G.dark, 636, 800, 10, "filbert 10", {25,70}},
}
for i,z in ipairs(Z) do
  work(zone(z[2], z[3], z[4]), {hand="body", pile=z[1], tool=z[5], clip=LANDC, angle=function(x,y) return -0.02 + 0.05*math.sin(x/150+y/40) end, angle_jitter=0.06, length=z[6], coverage=1.5, fill=true, load=0.6, threshold=0.5})
end
print(wait(0))

--@ chunk 29
local m = LANDC * mask(function(x,y) return y > 490 and 1 or 0 end)
blend(m, {angle=1.2, coverage=1.6, tool={kind="badger", width=30}})
blend(m, {angle=-1.2, coverage=1.6, tool={kind="badger", width=30}})
blend(m, {angle=0.02, coverage=1.2, tool={kind="badger", width=30}})
print(wait(0))

--@ chunk 30
local m = LANDC * mask(function(x,y) return y > 484 and 1 or 0 end)
blend(m, {angle=0, coverage=2.2, tool={kind="badger", width=40}})
blend(m, {angle=0.03, coverage=1.5, tool={kind="badger", width=40}})
print(wait(0))

--@ chunk 31
print(wait(60*48)); print(drying(500,500), drying(500,600), drying(500,690), drying(680,440))

--@ chunk 32
print(wait(60*24)); print(drying(500,500), drying(500,600), drying(300,540), drying(800,560))

--@ chunk 33
local tr = pile{{"lead white",2.5},{"green earth",1.5},{"pale smalt",0.5},{"yellow ochre",1.0},{"Prussian blue",0.04}, medium=0.1}
local tr2 = pile{{"lead white",1.6},{"green earth",1.5},{"yellow ochre",1.2},{"Prussian blue",0.1},{"raw umber",0.1}, medium=0.1}
local band1 = LANDC * mask(function(x,y) return smoothstep(478,484,y)*(1-smoothstep(496,504,y)) end) - M_rise:shrink(1)
local band2 = LANDC * mask(function(x,y) return smoothstep(492,497,y)*(1-smoothstep(508,516,y)) end)
hgrass(band1, tr, {8,22}, 1.8, 1.8, 11)
hgrass(band2, tr2, {8,24}, 1.5, 2.0, 12)
-- river: tone down, lighter streak in middle, far bank edge
local rv = pile{{"lead white",6},{"pale smalt",1.4},{"smalt",0.35},{"green earth",0.15}, medium=0.1}
work(RIVER, {hand="detail", pile=rv, angle=0, length={10,30}, coverage=1.2, clip=true, load=0.5})
-- reeds along near bank
local reed = pile{{"green earth",1.3},{"raw umber",0.6},{"yellow ochre",0.8},{"Prussian blue",0.12},{"lead white",0.8}, medium=0.1}
local rb = brush{kind="rigger", width=0.8, point=1}
rb:load(reed, 0.5)
local x = 345
while x < 1000 do
  local yb = rbot(x) + 1.0
  if math.random() < 0.7 then
    local h = rand(2, 5) * (1 + (x-345)/1400)
    rb:stroke({{x, yb}, {x + rand(-0.8,0.8), yb - h}}, {pressure={0.5,0.05}})
  end
  if rb:fullness() < 0.2 then rb:reload(reed, 0.5) end
  x = x + rand(1.5, 4.5)
end
-- dark line of far bank grass under town
local fb = brush{kind="round", width=1.0, point=0.6}
fb:load(reed, 0.4)
for x0 = 345, 990, 30 do
  local pts = {}
  for xx = x0, x0+32, 4 do pts[#pts+1] = {xx, rtop(xx) - 0.6} end
  fb:stroke(pts, {pressure={0.35,0.3}})
  if fb:fullness() < 0.25 then fb:reload(reed, 0.4) end
end
print(wait(0))

--@ chunk 34
local tr3 = pile{{"lead white",1.2},{"green earth",1.5},{"yellow ochre",1.4},{"Prussian blue",0.14},{"raw umber",0.15}, medium=0.1}
local band3 = LANDC * mask(function(x,y) return smoothstep(503,509,y)*(1-smoothstep(522,532,y)) end)
hgrass(band3, tr3, {10,28}, 1.3, 2.2, 13)
local tr2b = pile{{"lead white",2},{"green earth",1.5},{"yellow ochre",1.1},{"Prussian blue",0.08},{"pale smalt",0.2}, medium=0.1}
local band4 = LANDC * mask(function(x,y) return smoothstep(494,498,y)*(1-smoothstep(506,512,y)) end)
hgrass(band4, tr2b, {8,22}, 0.8, 1.8, 14)
print(wait(0))

--@ chunk 35
WL = {{35,580,1.0},{112,563,0.8},{178,550,0.64},{231,539,0.52},{274,531,0.43},{309,524,0.35},{338,519,0.29},{361,515,0.24},{380,512,0.2},{396,510,0.17}}
-- ditch: a dark bank line and water glint in front of the willows
local dpts, wpts = {}, {}
for x = -10, 420, 10 do
  local y = 583 - 0.176*(x+10)  -- rough line through bases
  y = 583 - (583-509)*((x+10)/430)^0.8
  dpts[#dpts+1] = {x, y+4}
  wpts[#wpts+1] = {x, y+6}
end
local ditchW = {}
for i,p in ipairs(dpts) do ditchW[i] = lerp(7, 1.2, (i-1)/(#dpts-1)) end
DITCH = ribbon(dpts, ditchW)
local bankD = pile{{"raw umber",1},{"green earth",1.2},{"Prussian blue",0.25},{"yellow ochre",0.6},{"lead white",0.3}, medium=0.1}
work(DITCH, {hand="detail", pile=bankD, angle=-0.17, length={8,20}, coverage=2, clip=true, fill=true})
local ww = {}
for i,p in ipairs(wpts) do ww[i] = lerp(2.4, 0.5, (i-1)/(#wpts-1)) end
DWATER = ribbon(wpts, ww) * mask(function(x,y) return (math.sin(x/9)+math.sin(x/23)) > -0.6 and 1 or 0 end)
local glint = pile{{"lead white",6},{"pale smalt",1.2},{"smalt",0.2}, medium=0.1}
work(DWATER, {hand="detail", pile=glint, angle=-0.17, length={6,16}, coverage=1.5, clip=true})
-- trunks
trunkD = pile{{"raw umber",1.2},{"bone black",0.3},{"lead white",0.6},{"yellow ochre",0.3}, medium=0.08}
trunkL = pile{{"lead white",2},{"raw umber",0.8},{"yellow ochre",0.5},{"pale smalt",0.3}, medium=0.08}
TRUNKS = nil
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local pts = {{x-4.5*s, y+1}, {x-6*s, y+2*s}, {x-4.8*s, y-8*s}, {x-4.2*s, y-22*s}, {x-7*s, y-30*s}, {x-3*s, y-34*s}, {x+2*s, y-33*s}, {x+7.5*s, y-31*s}, {x+4.6*s, y-22*s}, {x+4.6*s, y-8*s}, {x+6.5*s, y+2*s}, {x+4.5*s, y+1}}
  local t = poly(pts, true)
  TRUNKS = TRUNKS and (TRUNKS + t) or t
  work(t, {hand="detail", pile=trunkD, tool={kind="round", width=math.max(0.8, 2*s), point=0.4}, angle=math.pi/2, length={3,10}, coverage=2.5, clip=true, fill=true, edge="found"})
  local lit = t * mask(function(xx,yy) return xx < x - 1.2*s and 1 or 0 end)
  work(lit, {hand="detail", pile=trunkL, tool={kind="round", width=math.max(0.7, 1.4*s), point=0.4}, angle=math.pi/2, length={2,7}, coverage=1.4, clip=true})
end
print(wait(0))

--@ chunk 36
wDark = pile{{"green earth",1.5},{"raw umber",0.8},{"Prussian blue",0.3},{"yellow ochre",0.5},{"lead white",0.5}, medium=0.1}
wMid  = pile{{"lead white",1.5},{"green earth",1.6},{"pale smalt",0.6},{"yellow ochre",0.6},{"raw umber",0.3}, medium=0.1}
wLit  = pile{{"lead white",3.5},{"green earth",1.2},{"pale smalt",0.5},{"yellow ochre",0.8},{"chrome yellow",0.1}, medium=0.1}
CROWNS = {}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local hy = y - 32*s
  local cm = ellipse(x, hy-26*s, 34*s, 27*s) + ellipse(x-18*s, hy-12*s, 20*s, 14*s) + ellipse(x+19*s, hy-13*s, 19*s, 14*s) + ellipse(x+3*s, hy-40*s, 22*s, 16*s)
  cm = cm * above(function(xx) return hy + 2*s end)
  cm = cm:roughen(math.max(1.5, 5*s), math.max(6, 14*s), 100+i)
  CROWNS[i] = cm
  local wid = math.max(1.2, 3.2*s)
  stipple(cm, {pile=wDark, width=wid, coverage=2.2, pressure={0.5,0.8}, cluster=0.3, dips={16,0.7,0.4}})
  work(cm, {hand="hatch", pile=wMid, tool={kind="rigger", width=math.max(0.6,1.1*s), point=1}, clip=true,
    angle=function(xx,yy) return math.atan(yy - (hy+4*s), xx - x) end, angle_jitter=0.12, length={8*s+2, 26*s+3}, coverage=0.9, pressure={0.3,0.6}, ramps={0.1,0.7}})
  local lit = cm * mask(function(xx,yy) local dx, dy = (xx - x + 14*s)/(30*s), (yy - hy + 44*s)/(30*s); return 1 - smoothstep(0.5, 1.1, math.sqrt(dx*dx+dy*dy)) end)
  stipple(lit, {pile=wLit, width=wid*0.9, coverage=1.6, pressure={0.45,0.75}, cluster=0.5, feather=0.6, dips={10,0.6,0.4}})
end
print(wait(0))

--@ chunk 37
print(wait(60*30)); print(drying(35,500), drying(112,500), drying(100,580))

--@ chunk 38
print(wait(60*20)); print(drying(35,500), drying(112,500), drying(300,500))

--@ chunk 39
print(wait(60*48)); print(drying(35,500), drying(112,500), drying(300,500))

--@ chunk 40
print(wait(60*36)); print(drying(35,500), drying(112,500), drying(300,500), drying(240,500))

--@ chunk 41
wD2 = pile{{"green earth",1.6},{"raw umber",0.9},{"Prussian blue",0.35},{"yellow ochre",0.5},{"lead white",0.25}, medium=0.08}
wM2 = pile{{"lead white",1.0},{"green earth",1.6},{"pale smalt",0.4},{"yellow ochre",0.7},{"raw umber",0.35},{"Prussian blue",0.05}, medium=0.08}
wL2 = pile{{"lead white",2.6},{"green earth",1.2},{"pale smalt",0.3},{"yellow ochre",1.0},{"chrome yellow",0.15}, medium=0.08}
CROWN2 = {}
local leans = {0.08,-0.05,0.12,0.0,-0.1,0.06,0.02,-0.04,0.05,0}
local widths = {1.1,0.95,1.0,0.8,1.05,0.9,1.0,0.85,1.0,0.9}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local hy = y - 32*s
  local lean, wf = leans[i], widths[i]
  local m = CROWNS[i]:grow(1.2*s)
  for k=1,7 do
    local a = -math.pi/2 + lean + rand(-1.3, 1.3)
    local r = rand(18, 34)*s
    local cx, cy = x + math.cos(a)*r*wf*1.1, hy - 8*s + math.sin(a)*r
    m = m + ellipse(cx, cy, rand(9,15)*s*wf, rand(7,12)*s)
  end
  m = m * above(function(xx) return hy + 3*s end)
  m = m:roughen(math.max(1.2, 4*s), math.max(5, 10*s), 300+i)
  CROWN2[i] = m
  local wid = math.max(1.1, 2.8*s)
  stipple(m, {pile=wD2, width=wid, coverage=3.2, pressure={0.6,0.9}, cluster=0.25, dips={12,0.8,0.4}})
  local cl = m * mask(function(xx,yy) local d = (yy - (hy - 30*s))/(30*s); return 0.2 + 0.8*(1-smoothstep(-0.2, 0.9, d)) end)
  stipple(cl, {pile=wM2, width=wid, coverage=function(xx,yy) return 1.6*cl:at(xx,yy) end, pressure={0.5,0.8}, cluster={0.7, 7*s+2}, feather=0.5, dips={10,0.7,0.4}})
  local lit = m * mask(function(xx,yy) local dx, dy = (xx - x + 12*s)/(28*s), (yy - hy + 42*s)/(26*s); return 1 - smoothstep(0.3, 1.0, math.sqrt(dx*dx+dy*dy)) end)
  stipple(lit, {pile=wL2, width=wid*0.85, coverage=function(xx,yy) return 1.5*lit:at(xx,yy) end, pressure={0.5,0.8}, cluster={0.7, 5*s+2}, feather=0.7, dips={8,0.6,0.4}})
end
print(wait(0))

--@ chunk 42
-- path: centre line and half widths, perspective
PC = {{560,712},{548,690},{528,665},{506,640},{486,616},{470,596},{460,578},{455,562},{456,548},{462,537},{472,528},{484,521},{495,516}}
local hw = {}
for i,p in ipairs(PC) do hw[i] = lerp(52, 1.5, ((i-1)/(#PC-1))^0.7) end
local L, R = {}, {}
for i=1,#PC do
  local a = PC[math.max(1,i-1)]; local b = PC[math.min(#PC,i+1)]
  local dx, dy = b[1]-a[1], b[2]-a[2]; local d = math.sqrt(dx*dx+dy*dy)
  local nx, ny = -dy/d, dx/d
  L[#L+1] = {PC[i][1] + nx*hw[i], PC[i][2] + ny*hw[i]}
  R[#R+1] = {PC[i][1] - nx*hw[i], PC[i][2] - ny*hw[i]}
end
local pts = {}
for i=1,#L do pts[#pts+1] = L[i] end
for i=#R,1,-1 do pts[#pts+1] = R[i] end
PATH = poly(pts, true):roughen(1.5, 8, 71)
pathL = pile{{"lead white",3},{"yellow ochre",1.5},{"raw umber",0.4},{"red earth",0.12}, medium=0.1}
pathD = pile{{"lead white",1},{"yellow ochre",1.2},{"raw umber",0.8},{"green earth",0.3}, medium=0.1}
work(PATH, {hand="body", pile=pathD, tool="filbert 6", angle=function(x,y) return -1.0 + (y-520)/190*0.0 end, length={8,20}, coverage=1.6, fill=true, clip=true, edge="firm"})
local lit = PATH * mask(function(x,y) return smoothstep(0.35,0.6, noise{seed=9,period=30}:at01(x,y)) end)
work(lit, {hand="scumble", pile=pathL, tool="filbert 5", coverage=1.4, clip=PATH, load=0.5})
print(wait(0))

--@ chunk 43
print(wait(60*40)); print(drying(35,500), drying(300,500), drying(500,640), drying(465,560))

--@ chunk 44
print(wait(60*48)); print(drying(35,500), drying(300,500), drying(500,640), drying(465,560))

--@ chunk 45
wSil = pile{{"lead white",3},{"green earth",1.3},{"pale smalt",0.6},{"yellow ochre",0.5},{"raw umber",0.1}, medium=0.08}
wSil2 = pile{{"lead white",5},{"green earth",0.9},{"pale smalt",0.5},{"yellow ochre",0.7},{"chrome yellow",0.08}, medium=0.08}
rodP = pile{{"lead white",2},{"green earth",1.4},{"yellow ochre",0.6},{"raw umber",0.4},{"pale smalt",0.3}, medium=0.08}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local hy = y - 32*s
  local m = CROWN2[i]
  local wid = math.max(1.0, 2.4*s)
  -- silver-grey clusters on the upper and left, in clumps
  local up = m * mask(function(xx,yy) local d = (yy - (hy - 45*s))/(40*s) - (xx - x)/(90*s); return 1 - smoothstep(0.1, 0.9, d) end)
  stipple(up, {pile=wSil, width=wid, coverage=function(xx,yy) return 2.0*up:at(xx,yy) end, pressure={0.5,0.85}, cluster={0.75, 6*s+2}, feather=0.6, dips={8,0.7,0.4}})
  local top = m * mask(function(xx,yy) local dx, dy = (xx - x + 14*s)/(26*s), (yy - hy + 48*s)/(22*s); return 1 - smoothstep(0.2, 1.0, math.sqrt(dx*dx+dy*dy)) end)
  stipple(top, {pile=wSil2, width=wid*0.85, coverage=function(xx,yy) return 1.4*top:at(xx,yy) end, pressure={0.45,0.8}, cluster={0.8, 4*s+2}, feather=0.7, dips={6,0.6,0.4}})
  -- rods standing out of the crown edge
  local rb = brush{kind="rigger", width=math.max(0.5, 0.9*s), point=1}
  rb:load(rodP, 0.5)
  local n = math.floor(10 + 14*s)
  for k=1,n do
    local a = -math.pi/2 + rand(-1.5, 1.5)
    local r0 = rand(12, 22)*s
    local sx, sy = x + math.cos(a)*r0, hy - 22*s + math.sin(a)*r0*0.9
    local len = rand(10, 22)*s
    local a2 = a*0.8 - math.pi/2*0.2
    rb:stroke({{sx,sy},{sx+math.cos(a2)*len*0.5 + rand(-1,1)*s, sy+math.sin(a2)*len*0.5},{sx+math.cos(a2)*len, sy+math.sin(a2)*len - 2*s}}, {pressure={0.45,0.0}})
    if rb:fullness() < 0.25 then rb:reload(rodP, 0.5) end
  end
end
print(wait(0))

--@ chunk 46
wGreen = pile{{"green earth",2},{"yellow ochre",0.9},{"Prussian blue",0.12},{"raw umber",0.3},{"lead white",0.5}, medium=0.08}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local hy = y - 32*s
  local m = CROWN2[i]:shrink(0.6*s)
  local wid = math.max(1.0, 2.6*s)
  stipple(m, {pile=wGreen, width=wid, coverage=function(xx,yy) local d = (yy - (hy - 50*s))/(50*s); return 1.6*(1 - 0.5*smoothstep(0.4,1,d)) end, pressure={0.55,0.85}, cluster={0.5, 5*s+2}, dips={10,0.8,0.4}})
end
print(wait(0))

--@ chunk 47
print(wait(60*50)); print(drying(35,500), drying(300,500), drying(500,640), drying(465,560))

--@ chunk 48
-- glaze the path down, warmer and darker
local pg = pile{{"raw umber",1},{"yellow ochre",1},{"green earth",0.6},{"lead white",0.3}, medium=0.55}
work(PATH:grow(1), {hand="detail", pile=pg, tool={kind="filbert", width=4}, angle=-1.0, length={10,25}, coverage=1.2, clip=true, load=0.35})
blend(PATH:grow(1), {angle=-1.0, coverage=1.5, tool={kind="badger", width=10}})
-- two ruts: darker lines along the path
local rutP = pile{{"raw umber",1.2},{"yellow ochre",0.6},{"bone black",0.1},{"lead white",0.4}, medium=0.15}
local function offs(k)
  local out = {}
  for i=1,#PC-2 do
    local a = PC[math.max(1,i-1)]; local b = PC[math.min(#PC,i+1)]
    local dx, dy = b[1]-a[1], b[2]-a[2]; local d = math.sqrt(dx*dx+dy*dy)
    local hw = lerp(52, 1.5, ((i-1)/(#PC-1))^0.7)
    out[#out+1] = {PC[i][1] - dy/d*hw*k, PC[i][2] + dx/d*hw*k}
  end
  return out
end
RUT1, RUT2 = offs(0.55), offs(-0.5)
local rb = brush{kind="round", width=4, point=0.5}
for _,r in ipairs({RUT1, RUT2}) do
  rb:reload(rutP, 0.6)
  rb:stroke(r, {pressure={0.8,0.1}, ramps={0.02,0.5}, shake=0.6})
end
print(wait(0))

--@ chunk 49
print(wait(60*40)); print(drying(500,640), drying(465,560), drying(300,500))

--@ chunk 50
print(wait(60*30)); print(drying(500,640), drying(530,690), drying(470,600))

--@ chunk 51
pathS = pile{{"lead white",3},{"raw umber",0.7},{"yellow ochre",0.8},{"pale smalt",0.35},{"green earth",0.2}, medium=0.1}
pathS2 = pile{{"lead white",4},{"raw umber",0.5},{"yellow ochre",0.9},{"pale smalt",0.2}, medium=0.1}
pathSh = pile{{"lead white",1.5},{"raw umber",0.9},{"yellow ochre",0.5},{"pale smalt",0.4},{"green earth",0.4}, medium=0.1}
work(PATH:grow(1.5), {hand="body", pile=pathS, tool="filbert 5", angle=-1.0, length={8,20}, coverage=2.0, fill=true, clip=true, edge="found"})
blend(PATH:grow(1.5), {angle=-1.0, coverage=1.2, tool={kind="badger", width=12}})
-- lit patches: dry-ish sand in the middle distance
local lit = PATH * mask(function(x,y) return smoothstep(0.45,0.65, noise{seed=19,period=24}:at01(x,y*1.6)) end)
work(lit, {hand="scumble", pile=pathS2, tool="filbert 4", coverage=1.0, clip=PATH, load=0.35})
print(wait(0))

--@ chunk 52
print(wait(60*30)); print(drying(500,640), drying(530,690), drying(470,600))

--@ chunk 53
function horseMask(x0, y0, s, dir, grazing)
  local function P(u, v) return {x0 + dir*u*s, y0 - v*s} end
  local body = poly({P(-0.85,1.05),P(-0.8,1.35),P(-0.5,1.48),P(0.2,1.42),P(0.6,1.48),P(0.85,1.3),P(0.8,0.95),P(0.4,0.88),P(-0.3,0.9),P(-0.75,0.92)}, true)
  local neck, head
  if grazing then
    neck = poly({P(0.55,1.45),P(0.85,1.35),P(1.1,0.9),P(1.2,0.45),P(1.05,0.4),P(0.85,0.95),P(0.6,1.05)}, true)
    head = poly({P(1.02,0.55),P(1.22,0.52),P(1.3,0.12),P(1.2,0.04),P(1.1,0.1),P(1.0,0.4)}, true)
  else
    neck = poly({P(0.5,1.4),P(0.72,1.5),P(0.95,1.85),P(1.05,2.0),P(1.15,1.9),P(0.95,1.4),P(0.85,1.1),P(0.6,1.1)}, true)
    head = poly({P(0.98,2.02),P(1.1,2.08),P(1.45,1.7),P(1.42,1.6),P(1.3,1.62),P(1.0,1.8)}, true)
  end
  local legs = ribbon({P(-0.62,1.0),P(-0.7,0.5),P(-0.66,0.05)}, 0.11*s) + ribbon({P(-0.5,1.0),P(-0.48,0.5),P(-0.52,0.05)}, 0.1*s)
             + ribbon({P(0.6,1.0),P(0.62,0.5),P(0.64,0.05)}, 0.1*s) + ribbon({P(0.48,1.0),P(0.44,0.5),P(0.4,0.05)}, 0.1*s)
  local tail = ribbon({P(-0.82,1.3),P(-0.92,0.95),P(-0.9,0.6)}, {0.14*s, 0.12*s, 0.06*s})
  return body + neck + head + legs + tail
end
HORSES = {}
local bay = pile{{"red earth",1.2},{"raw umber",1.0},{"bone black",0.25},{"lead white",0.3}, medium=0.08}
local bayL = pile{{"red earth",1},{"yellow ochre",0.6},{"raw umber",0.4},{"lead white",0.6}, medium=0.08}
local grey = pile{{"lead white",4},{"raw umber",0.3},{"pale smalt",0.3},{"yellow ochre",0.1}, medium=0.08}
local greyS = pile{{"lead white",2},{"raw umber",0.5},{"pale smalt",0.6},{"bone black",0.05}, medium=0.08}
local s = 13
local h1 = horseMask(688, 534, s, 1, true)
local h2 = horseMask(742, 531, 12.5, -1, false)
HORSES = h1 + h2
local t = {kind="round", width=1.2, point=0.5}
work(h1, {hand="detail", pile=bay, tool=t, angle=0, length={2,6}, coverage=3, clip=true, fill=true, edge="found"})
work(h1 * mask(function(x,y) return y < 534 - 1.3*s and 1 or 0 end), {hand="detail", pile=bayL, tool={kind="round", width=1.0, point=0.5}, angle=0, length={2,5}, coverage=1.2, clip=true})
work(h2, {hand="detail", pile=greyS, tool=t, angle=0, length={2,6}, coverage=3, clip=true, fill=true, edge="found"})
work(h2 * mask(function(x,y) return y < 531 - 1.12*12.5 and 1 or 0 end), {hand="detail", pile=grey, tool={kind="round", width=1.0, point=0.5}, angle=0, length={2,6}, coverage=2, clip=true, fill=true})
print(wait(0))

--@ chunk 54
local function Pf(x0,y0,s,dir) return function(u,v) return {x0 + dir*u*s, y0 - v*s} end end
local P = Pf(742,531,12.5,-1)
local grey = pile{{"lead white",4},{"raw umber",0.3},{"pale smalt",0.3},{"yellow ochre",0.1}, medium=0.08}
local greyS = pile{{"lead white",2},{"raw umber",0.5},{"pale smalt",0.6},{"bone black",0.05}, medium=0.08}
local add = poly({P(0.4,1.45),P(0.75,1.62),P(0.95,1.95),P(1.12,1.98),P(1.05,1.6),P(0.95,1.2),P(0.8,0.98),P(0.5,0.95)}, true)
         + poly({P(-0.8,1.0),P(-0.3,0.84),P(0.4,0.84),P(0.8,0.98),P(0.8,1.2),P(-0.8,1.2)}, true)
         + poly({P(1.0,1.95),P(1.15,2.05),P(1.48,1.68),P(1.44,1.56),P(1.28,1.58),P(1.0,1.75)}, true)
local t = {kind="round", width=1.2, point=0.5}
work(add, {hand="detail", pile=grey, tool=t, angle=0, length={2,6}, coverage=3, clip=true, fill=true, edge="found"})
local under = (add + HORSES) * ellipse(742,531,30,30) * mask(function(x,y) return y > 531 - 1.05*12.5 and 1 or 0 end)
work(under, {hand="detail", pile=greyS, tool={kind="round", width=1.0, point=0.5}, angle=0, length={2,5}, coverage=1.5, clip=true})
HORSES = HORSES + add
-- manes and tails
local dark = pile{{"raw umber",1},{"bone black",0.6},{"lead white",0.2}, medium=0.08}
local r = brush{kind="rigger", width=0.9, point=1}
r:load(dark, 0.5)
r:stroke({P(0.72,1.62),P(0.9,1.9),P(1.05,2.03)}, {pressure={0.6,0.3}})
r:stroke({P(-0.84,1.3),P(-0.92,1.0),P(-0.88,0.62)}, {pressure={0.4,0.1}})
local Q = Pf(688,534,13,1)
r:reload(dark, 0.5)
r:stroke({Q(0.6,1.46),Q(0.85,1.36),Q(1.08,0.95),Q(1.16,0.55)}, {pressure={0.6,0.3}})
r:stroke({Q(-0.84,1.3),Q(-0.94,0.98),Q(-0.9,0.55)}, {pressure={0.7,0.2}})
-- hooves and cast shadows on the grass
local sh = pile{{"green earth",1.2},{"raw umber",0.8},{"Prussian blue",0.2},{"lead white",0.3}, medium=0.1}
local shm = ellipse(700,534.5,20,1.6) + ellipse(733,531.5,19,1.5)
work(shm, {hand="detail", pile=sh, angle=0, length={4,10}, coverage=2, clip=true, fill=true})
print(wait(0))

--@ chunk 55
print(wait(60*30)); print(drying(500,640), drying(530,690), drying(470,600), drying(700,520))

--@ chunk 56
print(wait(60*48)); print(drying(500,640), drying(530,690), drying(470,600), drying(700,520))

--@ chunk 57
local cr = CROWN2[1]
for i=2,#CROWN2 do cr = cr + CROWN2[i] end
CROWNS_ALL = cr
KEEPM = HORSES:grow(1) + TRUNKS:grow(0.5) + CROWNS_ALL:grow(0.5) + PATH:shrink(2)
MEAD = LAND * mask(function(x,y) return y > 496 and 1 or 0 end) - KEEPM
gP = {
 lit = pile{{"lead white",1.5},{"yellow ochre",1.5},{"chrome yellow",0.5},{"green earth",0.8},{"Prussian blue",0.1}, medium=0.1},
 mid = pile{{"yellow ochre",1.5},{"green earth",1.4},{"Prussian blue",0.25},{"lead white",0.7},{"raw umber",0.2}, medium=0.1},
 sh  = pile{{"green earth",1.4},{"Prussian blue",0.3},{"raw umber",0.6},{"yellow ochre",0.8},{"lead white",0.6},{"pale smalt",0.3}, medium=0.1},
 dark= pile{{"raw umber",1.2},{"Prussian blue",0.45},{"green earth",1},{"yellow ochre",0.6},{"bone black",0.1}, medium=0.1},
}
local nz = noise{seed=5, period=160, octaves=3}
function zoneN(a, b, amp, soft)
  return mask(function(x,y) local yy = y + amp*nz(x, y*2); return smoothstep(a-soft, a+soft, yy)*(1 - smoothstep(b-soft, b+soft, yy)) end)
end
function mpass(m, p, y0, cov, seed)
  local d = y0 - 468
  local len = {d*0.05, d*0.12}
  local w = clamp(0.6 + d*0.008, 0.8, 2.4)
  work(m, {hand="hatch", pile=p, tool={kind="round", width=w, point=0.8}, angle=-math.pi/2, angle_jitter=0.3, length=len, coverage=cov, pressure={0.35,0.75}, ramps={0.1,0.6}, threshold=0.5, seed=seed})
end
-- cloud shadow band 532-578, cool
mpass(MEAD * zoneN(532, 560, 8, 5), gP.sh, 545, 1.4, 1)
mpass(MEAD * zoneN(555, 580, 8, 5), gP.sh, 568, 1.4, 2)
-- lit bands
mpass(MEAD * zoneN(496, 520, 4, 4), gP.lit, 510, 1.2, 3)
mpass(MEAD * zoneN(516, 536, 5, 4), gP.lit, 526, 1.2, 4)
mpass(MEAD * zoneN(578, 604, 8, 5), gP.lit, 590, 1.3, 5)
mpass(MEAD * zoneN(600, 628, 8, 5), gP.mid, 614, 1.3, 6)
print(wait(0))

--@ chunk 58
local FG = LAND * mask(function(x,y) return y > 600 and 1 or 0 end) - PATH:shrink(3)
local function fpass(m, p, len, w, cov, seed, jit)
  work(m, {hand="hatch", pile=p, tool={kind="round", width=w, point=0.8}, angle=-math.pi/2, angle_jitter=jit or 0.3, length=len, coverage=cov, pressure={0.4,0.8}, ramps={0.1,0.6}, threshold=0.5, seed=seed})
end
fpass(FG * zoneN(620, 660, 8, 6), gP.sh, {8,20}, 2.0, 1.4, 11)
fpass(FG * zoneN(650, 800, 8, 6), gP.dark, {12,30}, 2.4, 1.5, 12)
fpass(FG * zoneN(640, 800, 10, 6), gP.mid, {10,28}, 2.0, 0.8, 13)
print(wait(0))

--@ chunk 59
local nb = noise{seed=31, period=90, octaves=3, stretch={0, 3}}
local patchy = mask(function(x,y) return smoothstep(0.45, 0.62, nb:at01(x,y)) end)
local band = function(a,b) return MEAD * zoneN(a, b, 10, 10) end
local mixP = pile{{"yellow ochre",1.3},{"green earth",1.5},{"Prussian blue",0.22},{"lead white",0.9},{"raw umber",0.25},{"pale smalt",0.15}, medium=0.1}
mpass(band(500, 540) * patchy, mixP, 520, 0.7, 21)
mpass(band(540, 580), mixP, 560, 0.5, 22)
mpass(band(575, 612), mixP, 592, 0.9, 23)
mpass(band(575, 612) * patchy, gP.sh, 592, 0.6, 24)
mpass(band(605, 640), gP.sh, 620, 0.6, 25)
print(wait(0))

--@ chunk 60
local RISE = LAND * mask(function(x,y) return (y < 486 and x < 380) and 1 or 0 end) - MILL:grow(0.5)
local r1 = pile{{"lead white",3},{"green earth",1.5},{"yellow ochre",0.9},{"pale smalt",0.5},{"raw umber",0.12}, medium=0.1}
local r2 = pile{{"lead white",2},{"green earth",1.5},{"yellow ochre",0.8},{"pale smalt",0.6},{"raw umber",0.25}, medium=0.1}
local r3 = pile{{"lead white",4},{"yellow ochre",1.2},{"green earth",0.8},{"chrome yellow",0.15}, medium=0.1}
work(RISE, {hand="hatch", pile=r1, tool={kind="round", width=1.2, point=0.7}, angle=0, angle_jitter=0.1, length={4,12}, coverage=1.6, clip=true, seed=41})
local shade = RISE * mask(function(x,y) return smoothstep(470, 484, y) end)
work(shade, {hand="hatch", pile=r2, tool={kind="round", width=1.1, point=0.7}, angle=0, angle_jitter=0.1, length={4,10}, coverage=1.2, clip=true, seed=42, threshold=0.4})
local crest = RISE * mask(function(x,y) return 1 - smoothstep(0, 8, y - hzc(x)) end)
work(crest, {hand="hatch", pile=r3, tool={kind="round", width=1.0, point=0.7}, angle=0, angle_jitter=0.1, length={3,9}, coverage=1.2, clip=true, seed=43, threshold=0.4})
-- a hedge running down the slope
local hedge = pile{{"green earth",1.5},{"raw umber",0.5},{"pale smalt",0.8},{"lead white",1.2}, medium=0.1}
local hb = ribbon({{250,461},{275,467},{300,474},{322,480},{345,485}}, {2.2,2.5,2.8,3,3.2}):roughen(1,4,51)
stipple(hb * above(function(x) return hzc(x) - 3 end), {pile=hedge, width=1.4, coverage=3, pressure={0.5,0.8}, cluster=0.4})
-- a farmstead near the mill
local wall = pile{{"lead white",3},{"yellow ochre",0.6},{"raw umber",0.2},{"pale smalt",0.3}, medium=0.08}
local roof = pile{{"red earth",1},{"raw umber",0.7},{"lead white",1.2},{"pale smalt",0.4}, medium=0.08}
local treeF = pile{{"green earth",1.5},{"raw umber",0.4},{"pale smalt",0.9},{"lead white",1.3}, medium=0.08}
local trees = (ellipse(92,452,11,8) + ellipse(106,449,9,9) + ellipse(80,455,8,6)):roughen(1.5,5,52)
stipple(trees, {pile=treeF, width=1.5, coverage=3, pressure={0.5,0.8}, cluster=0.4})
local w1 = poly{{90,462},{90,455},{112,455},{112,462}}
local r1m = poly{{88,455.5},{94,449.5},{108,449.5},{114,455.5}}
work(r1m, {hand="detail", pile=roof, tool={kind="round", width=1, point=0.5}, angle=0, length={2,5}, coverage=3, clip=true, fill=true, edge="found"})
work(w1, {hand="detail", pile=wall, tool={kind="round", width=1, point=0.5}, angle=math.pi/2, length={2,5}, coverage=3, clip=true, fill=true, edge="found"})
local dk = pile{{"raw umber",1},{"bone black",0.3},{"lead white",0.8}, medium=0.08}
local r = brush{kind="round", width=0.8, point=0.6}
r:load(dk, 0.4)
r:touch(96, 459, {pressure=0.5}); r:touch(104, 459, {pressure=0.5}); r:touch(100.5, 460, {pressure=0.5})
print(wait(0))

--@ chunk 61
RISE = LAND * mask(function(x,y) return (y < 487 and x < 385) and 1 or 0 end) - MILL:grow(0.5) - poly{{86,463},{86,445},{118,445},{118,463}}
local g1 = pile{{"lead white",1.4},{"green earth",2},{"yellow ochre",0.8},{"pale smalt",0.6},{"raw umber",0.15}, medium=0.1}
local g2 = pile{{"lead white",1.0},{"green earth",2},{"yellow ochre",0.5},{"pale smalt",0.8},{"raw umber",0.3}, medium=0.1}
work(RISE, {hand="hatch", pile=g1, tool={kind="round", width=1.3, point=0.7}, angle=0.03, angle_jitter=0.1, length={4,12}, coverage=1.5, clip=true, seed=61, load=0.7})
local low = RISE * mask(function(x,y) return smoothstep(468, 482, y) * (0.6 + 0.4*smoothstep(150, 300, x)) end)
work(low, {hand="hatch", pile=g2, tool={kind="round", width=1.3, point=0.7}, angle=0.03, angle_jitter=0.1, length={4,12}, coverage=1.3, clip=true, seed=62, threshold=0.35, load=0.7})
print(wait(0))

--@ chunk 62
local nn = noise{seed=83, period=25, octaves=2}
local FARB = LAND * mask(function(x,y) return y < 500 and 1 or 0 end) - MILL:grow(0.5) - TOWN:grow(0.5) - FARTREES - RIVER:grow(0.5) - poly{{88,462},{88,449},{114,449},{114,462}} - CROWNS_ALL:grow(0.5) - TRUNKS
local g2 = pile{{"lead white",1.3},{"green earth",2},{"yellow ochre",0.6},{"pale smalt",0.8},{"raw umber",0.25}, medium=0.1}
local g3 = pile{{"lead white",2.2},{"green earth",1.6},{"yellow ochre",0.6},{"pale smalt",0.9},{"raw umber",0.12}, medium=0.1}
local left = FARB * mask(function(x,y) return 1 - smoothstep(340, 470, x + 40*nn(x,y)) end)
local right = FARB * mask(function(x,y) return smoothstep(340, 470, x + 40*nn(x,y)) end)
work(left, {hand="hatch", pile=g2, tool={kind="round", width=1.2, point=0.7}, angle=0.03, angle_jitter=0.1, length={4,12}, coverage=0.9, clip=true, seed=71, threshold=0.5, load=0.6})
work(right, {hand="hatch", pile=g3, tool={kind="round", width=1.2, point=0.7}, angle=0, angle_jitter=0.1, length={4,14}, coverage=1.3, clip=true, seed=72, threshold=0.5, load=0.6})
print(wait(0))

--@ chunk 63
local nn = noise{seed=91, period=30, octaves=2}
FARB = LAND * mask(function(x,y) return y < 502 and 1 or 0 end) - MILL:grow(0.5) - TOWN:grow(0.5) - FARTREES - RIVER:grow(0.3) - poly{{88,462},{88,449},{114,449},{114,462}} - CROWNS_ALL:grow(0.5) - TRUNKS
local farG = pile{{"lead white",1.6},{"green earth",2},{"yellow ochre",0.7},{"pale smalt",0.7},{"Prussian blue",0.04},{"raw umber",0.15}, medium=0.1}
local farG2 = pile{{"lead white",2.6},{"green earth",1.6},{"yellow ochre",0.5},{"pale smalt",1.0},{"Prussian blue",0.03}, medium=0.1}
local a = FARB * mask(function(x,y) return (1 - smoothstep(420, 640, x + 60*nn(x,y))) end)
local b = FARB * mask(function(x,y) return smoothstep(420, 640, x + 60*nn(x,y)) end)
work(a, {hand="hatch", pile=farG, tool={kind="round", width=1.3, point=0.7}, angle=0.02, angle_jitter=0.12, length={5,14}, coverage=1.7, clip=true, seed=81, threshold=0.5, load=0.7})
work(b, {hand="hatch", pile=farG2, tool={kind="round", width=1.2, point=0.7}, angle=0, angle_jitter=0.12, length={5,14}, coverage=1.7, clip=true, seed=82, threshold=0.5, load=0.7})
print(wait(0))

--@ chunk 64
print(drying(380,478), drying(200,470))
local seam = FARB * rect(300, 455, 200, 50)
blend(seam, {angle=0, coverage=2.5, tool={kind="badger", width=24}})
local hs = FARB * rect(0, 478, 560, 22)
blend(hs, {angle=1.4, coverage=1.5, tool={kind="badger", width=10}, pressure={0.3,0.5}})
blend(hs, {angle=0, coverage=1.5, tool={kind="badger", width=16}, pressure={0.3,0.5}})
print(wait(0))

--@ chunk 65
print(wait(60*50)); print(drying(380,478), drying(200,470), drying(600,600), drying(300,680), drying(100,510))

--@ chunk 66
-- tone the white-topped willows: green-silver cluster stipple over the tops, smaller & cooler
local wS3 = pile{{"lead white",1.6},{"green earth",1.8},{"pale smalt",0.5},{"yellow ochre",0.7},{"raw umber",0.12}, medium=0.08}
local wS4 = pile{{"lead white",1.0},{"green earth",1.8},{"yellow ochre",0.8},{"Prussian blue",0.08},{"raw umber",0.3}, medium=0.08}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local hy = y - 32*s
  local m = CROWN2[i]:shrink(0.5*s)
  local wid = math.max(1.0, 2.3*s)
  local up = m * mask(function(xx,yy) local d = (yy - (hy - 45*s))/(40*s) - (xx - x)/(90*s); return 1 - smoothstep(0.0, 1.0, d) end)
  stipple(up, {pile=wS3, width=wid, coverage=function(xx,yy) return 1.8*up:at(xx,yy) end, pressure={0.5,0.85}, cluster={0.6, 4*s+2}, dips={8,0.7,0.4}})
  -- dark hollows in the crown, lower right
  local low = m * mask(function(xx,yy) local d = (yy - (hy - 30*s))/(30*s) + (xx - x)/(60*s); return smoothstep(0.1, 0.9, d) end)
  stipple(low, {pile=wD2, width=wid, coverage=function(xx,yy) return 1.2*low:at(xx,yy) end, pressure={0.5,0.85}, cluster={0.7, 5*s+2}, feather=0.5, dips={8,0.7,0.4}})
end
print(wait(0))

--@ chunk 67
local tD = pile{{"raw umber",1.2},{"bone black",0.35},{"lead white",0.9},{"pale smalt",0.3},{"green earth",0.3}, medium=0.08}
local tL = pile{{"lead white",2.2},{"raw umber",0.6},{"pale smalt",0.4},{"yellow ochre",0.3}, medium=0.08}
TR2 = nil
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local pts = {{x-6*s, y+1.5}, {x-7.5*s, y+1*s}, {x-6.2*s, y-6*s}, {x-5.6*s, y-18*s}, {x-8*s, y-27*s}, {x-3*s, y-31*s}, {x+3*s, y-30*s}, {x+8.5*s, y-27*s}, {x+6*s, y-18*s}, {x+6*s, y-6*s}, {x+8*s, y+1*s}, {x+6*s, y+1.5}}
  local t = poly(pts, true):roughen(math.max(0.4, 0.8*s), math.max(3, 6*s), 400+i) - CROWN2[i]:shrink(1.5*s)
  TR2 = TR2 and (TR2 + t) or t
  work(t, {hand="detail", pile=tD, tool={kind="round", width=math.max(0.8, 1.8*s), point=0.4}, angle=math.pi/2, length={2,8}, coverage=3, clip=true, fill=true, edge="found"})
  local lit = t * mask(function(xx,yy) return smoothstep(x - 3*s, x - 5.5*s, xx) end)
  work(lit, {hand="detail", pile=tL, tool={kind="round", width=math.max(0.7, 1.2*s), point=0.4}, angle=math.pi/2, length={2,6}, coverage=1.5, clip=true})
  -- bark fissures
  if s > 0.4 then
    local r = brush{kind="rigger", width=0.6, point=1}
    r:load(tD, 0.4)
    for k=1,3 do
      local xx = x + rand(-3,3)*s
      r:stroke({{xx, y - rand(2,6)*s}, {xx + rand(-1,1)*s, y - rand(14,24)*s}}, {pressure={0.4,0.1}})
    end
  end
end
print(wait(0))

--@ chunk 68
local det = function(m, p, w, cov, ang)
  work(m, {hand="detail", pile=p, tool={kind="round", width=w or 1.0, point=0.5}, angle=ang or math.pi/2, length={2,6}, coverage=cov or 3, clip=true, fill=true, edge="found"})
end
coatP = pile{{"bone black",1},{"Prussian blue",0.25},{"raw umber",0.6},{"green earth",0.4},{"lead white",0.35}, medium=0.08}
coatL = pile{{"bone black",0.6},{"Prussian blue",0.2},{"raw umber",0.5},{"green earth",0.6},{"lead white",1.1}, medium=0.08}
hairP = pile{{"raw umber",1},{"bone black",0.4},{"red earth",0.2},{"lead white",0.15}, medium=0.08}
dressP = pile{{"red earth",1},{"raw umber",0.7},{"bone black",0.2},{"vermilion",0.15},{"lead white",0.2}, medium=0.08}
dressL = pile{{"red earth",1},{"raw umber",0.4},{"vermilion",0.25},{"lead white",0.6},{"yellow ochre",0.2}, medium=0.08}
shawlP = pile{{"lead white",4},{"yellow ochre",0.5},{"raw umber",0.12}, medium=0.08}
shawlS = pile{{"lead white",2},{"yellow ochre",0.4},{"raw umber",0.3},{"pale smalt",0.3}, medium=0.08}
bootP = pile{{"bone black",1},{"raw umber",0.5},{"lead white",0.1}, medium=0.08}
-- MAN, x~446, feet 562
local mx = 446
MAN_COAT = poly({{mx-6,475},{mx-9.5,478},{mx-10.5,486},{mx-11,500},{mx-12.5,520},{mx-13.5,533},{mx-4,535},{mx+3,535},{mx+13,532},{mx+12,518},{mx+11,500},{mx+10.5,486},{mx+9.5,478},{mx+6,475}}, true)
MAN_LEGS = poly({{mx-7,533},{mx-7.2,556},{mx-3.4,557},{mx-2,534}}) + poly({{mx+1.5,534},{mx+2.2,557},{mx+6,557},{mx+6.5,533}})
MAN_BOOTS = poly({{mx-7.6,553},{mx-8,561},{mx-2.8,561.5},{mx-3,553}}) + poly({{mx+1.8,553},{mx+1.8,561.5},{mx+7.4,561},{mx+6.4,553}})
MAN_HEAD = ellipse(mx+0.5, 468.5, 5.0, 6.2)
MAN_CAP = poly({{mx-6.5,465},{mx-4,460.5},{mx+2,459.3},{mx+6.5,461.5},{mx+7,465},{mx+1,466}}, true)
MAN_COLLAR = poly({{mx-6,476},{mx-5,471},{mx+6,471},{mx+6.8,476},{mx,478}}, true)
MAN = MAN_COAT + MAN_LEGS + MAN_BOOTS + MAN_HEAD + MAN_CAP + MAN_COLLAR
det(MAN_LEGS, coatP, 1.0)
det(MAN_BOOTS, bootP, 1.0)
det(MAN_COAT + MAN_COLLAR, coatP, 1.2)
det(MAN_HEAD, hairP, 1.0)
det(MAN_CAP, bootP, 1.0, 3, 0)
local mlit = (MAN_COAT + MAN_COLLAR) * mask(function(x,y) return smoothstep(mx-6, mx-10, x) end)
work(mlit, {hand="detail", pile=coatL, tool={kind="round", width=0.9, point=0.5}, angle=math.pi/2, length={3,10}, coverage=1.5, clip=true})
-- WOMAN x~470, feet 561
local wx = 470
local W_DRESS = poly({{wx-6.5,484},{wx-7.5,500},{wx-9.5,520},{wx-12,545},{wx-13.5,561},{wx-2,562.5},{wx+8,562},{wx+13.5,560},{wx+11.5,544},{wx+9,520},{wx+7.5,500},{wx+6.5,484}}, true)
local W_HEAD = ellipse(wx, 475.5, 4.6, 5.6)
local W_BUN = ellipse(wx+0.3, 471.2, 2.8, 2.4)
local W_SHAWL = poly({{wx-7,482.5},{wx-8.5,486},{wx-7.5,492},{wx-2,503},{wx,506},{wx+2,503},{wx+7.5,492},{wx+8.5,486},{wx+7,482.5},{wx,480.5}}, true)
WOMAN = W_DRESS + W_HEAD + W_BUN + W_SHAWL
det(W_DRESS, dressP, 1.2)
local wlit = W_DRESS * mask(function(x,y) return smoothstep(wx-4, wx-9, x) end)
work(wlit, {hand="detail", pile=dressL, tool={kind="round", width=0.9, point=0.5}, angle=math.pi/2, length={4,12}, coverage=1.5, clip=true})
det(W_HEAD, hairP, 1.0)
det(W_BUN, hairP, 0.8)
det(W_SHAWL, shawlP, 1.0, 3, 0.4)
local ssh = W_SHAWL * mask(function(x,y) return smoothstep(wx+1, wx+6, x) end)
work(ssh, {hand="detail", pile=shawlS, tool={kind="round", width=0.8, point=0.5}, angle=math.pi/2, length={2,6}, coverage=1.4, clip=true})
FIGS = MAN + WOMAN
print(wait(0))

--@ chunk 69
print(wait(60*30)); print(drying(446,500), drying(470,520), drying(35,570), drying(112,550))

--@ chunk 70
print(wait(60*40)); print(drying(446,500), drying(470,520), drying(35,570), drying(112,550), drying(600,495), drying(700,650))

--@ chunk 71
local tD = pile{{"bone black",0.6},{"raw umber",1},{"green earth",0.6},{"lead white",0.5},{"pale smalt",0.4}, medium=0.08}
local tL = pile{{"lead white",1.6},{"raw umber",0.5},{"green earth",0.5},{"pale smalt",0.5},{"bone black",0.1}, medium=0.08}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local t = TR2 * ellipse(x, y - 15*s, 12*s + 1, 20*s + 3)
  work(t, {hand="detail", pile=tD, tool={kind="round", width=math.max(0.8, 1.8*s), point=0.4}, angle=math.pi/2, length={2,8}, coverage=3, clip=true, fill=true, edge="found", load=0.9})
  local lit = t * mask(function(xx,yy) return smoothstep(x - 2*s, x - 5*s, xx) end)
  work(lit, {hand="detail", pile=tL, tool={kind="round", width=math.max(0.7, 1.1*s), point=0.4}, angle=math.pi/2, length={2,6}, coverage=1.3, clip=true, load=0.8})
end
print(wait(0))

--@ chunk 72
local tD = pile{{"bone black",0.5},{"yellow ochre",0.5},{"green earth",0.8},{"lead white",0.8},{"pale smalt",0.3}, medium=0.08}
local tL = pile{{"lead white",2},{"yellow ochre",0.4},{"green earth",0.5},{"pale smalt",0.4},{"bone black",0.12}, medium=0.08}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local t = TR2 * ellipse(x, y - 15*s, 12*s + 1, 20*s + 3)
  work(t, {hand="detail", pile=tD, tool={kind="round", width=math.max(0.8, 1.8*s), point=0.4}, angle=math.pi/2, length={2,8}, coverage=3.5, clip=true, fill=true, edge="found", load=1.0})
  local lit = t * mask(function(xx,yy) return smoothstep(x - 2*s, x - 5*s, xx) end)
  work(lit, {hand="detail", pile=tL, tool={kind="round", width=math.max(0.7, 1.1*s), point=0.4}, angle=math.pi/2, length={2,6}, coverage=1.3, clip=true, load=0.8})
end
print(wait(0))

--@ chunk 73
local tS = pile{{"bone black",0.8},{"green earth",0.8},{"yellow ochre",0.3},{"lead white",0.35},{"raw umber",0.2}, medium=0.08}
local tuft = pile{{"green earth",1.3},{"yellow ochre",1},{"Prussian blue",0.2},{"raw umber",0.4},{"lead white",0.6}, medium=0.08}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local t = TR2 * ellipse(x, y - 15*s, 12*s + 1, 20*s + 3)
  local sh = t * mask(function(xx,yy) return smoothstep(x + 0.5*s, x + 3*s, xx) end)
  work(sh, {hand="detail", pile=tS, tool={kind="round", width=math.max(0.7, 1.4*s), point=0.4}, angle=math.pi/2, length={2,7}, coverage=2.5, clip=true, fill=true, load=0.9})
  -- underside shadow of crown on the head of the trunk
  local hd = t * mask(function(xx,yy) return smoothstep(y - 24*s, y - 30*s, yy) end)
  work(hd, {hand="detail", pile=tS, tool={kind="round", width=math.max(0.7, 1.4*s), point=0.4}, angle=0, length={2,6}, coverage=2, clip=true, load=0.8})
  -- bark fissures
  if s > 0.3 then
    local r = brush{kind="rigger", width=math.max(0.45, 0.7*s), point=1}
    r:load(tS, 0.5)
    for k=1,math.floor(3 + 4*s) do
      local xx = x + rand(-4.5,3.5)*s
      local y0 = y - rand(1,8)*s
      r:stroke({{xx, y0}, {xx + rand(-0.8,0.8)*s, y0 - rand(6,14)*s}, {xx + rand(-1.2,1.2)*s, y0 - rand(14,22)*s}}, {pressure={0.45,0.1}})
    end
  end
  -- grass tufts over the foot
  local g = brush{kind="rigger", width=math.max(0.5, 1.0*s), point=1}
  g:load(tuft, 0.6)
  for k=1,math.floor(6 + 12*s) do
    local xx = x + rand(-9,9)*s
    local yb = y + rand(0.5, 3)*s
    g:stroke({{xx, yb}, {xx + rand(-2,2)*s, yb - rand(4,10)*s}}, {pressure={0.55,0.0}})
    if g:fullness() < 0.25 then g:reload(tuft, 0.6) end
  end
end
print(wait(0))

--@ chunk 74
print(wait(60*72)); print(drying(446,500), drying(470,520), drying(35,570), drying(112,550), drying(40,500))

--@ chunk 75
local rodG = pile{{"lead white",1.2},{"green earth",1.8},{"yellow ochre",0.7},{"pale smalt",0.4},{"raw umber",0.3}, medium=0.08}
local rodS = pile{{"lead white",2.6},{"green earth",1.3},{"pale smalt",0.6},{"yellow ochre",0.5}, medium=0.08}
local rodD = pile{{"green earth",1.6},{"raw umber",0.8},{"Prussian blue",0.3},{"yellow ochre",0.4},{"lead white",0.3}, medium=0.08}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local hy = y - 32*s
  local cx, cy = x, hy + 2*s
  local fan = CROWN2[i]:grow(7*s) * mask(function(xx,yy) return yy < hy - 6*s and 1 or 0 end)
  fan = fan * mask(function(xx,yy) local a = math.atan(yy - cy, xx - cx) + math.pi/2; return math.abs(a) < 1.25 and 1 or 0 end)
  local ang = function(xx,yy) return math.atan(yy - cy, xx - cx) end
  local rw = math.max(0.45, 0.9*s)
  -- dark rods inside first, then green, then silver ones at top edge
  work(CROWN2[i]:shrink(3*s), {hand="hatch", pile=rodD, tool={kind="rigger", width=rw, point=1}, clip=CROWN2[i], angle=ang, angle_jitter=0.12, length={10*s+2, 24*s+3}, coverage=0.5, pressure={0.35,0.7}, ramps={0.05,0.8}, seed=500+i})
  work(fan, {hand="hatch", pile=rodG, tool={kind="rigger", width=rw, point=1}, angle=ang, angle_jitter=0.1, length={12*s+2, 26*s+3}, coverage=0.6, pressure={0.35,0.7}, ramps={0.05,0.8}, seed=600+i})
  local top = fan * mask(function(xx,yy) return smoothstep(hy - 30*s, hy - 55*s, yy) end)
  work(top, {hand="hatch", pile=rodS, tool={kind="rigger", width=rw, point=1}, angle=ang, angle_jitter=0.1, length={8*s+2, 18*s+3}, coverage=0.6, pressure={0.35,0.65}, ramps={0.05,0.8}, seed=700+i, threshold=0.4})
end
print(wait(0))

--@ chunk 76
print(wait(60*30)); print(drying(35,470), drying(112,480), drying(300,500))

--@ chunk 77
local lfD = pile{{"green earth",1.7},{"raw umber",0.6},{"Prussian blue",0.22},{"yellow ochre",0.6},{"lead white",0.5}, medium=0.08}
local lfM = pile{{"lead white",1.3},{"green earth",1.8},{"yellow ochre",0.8},{"pale smalt",0.5},{"raw umber",0.2}, medium=0.08}
local lfS = pile{{"lead white",2.8},{"green earth",1.3},{"pale smalt",0.6},{"yellow ochre",0.6}, medium=0.08}
ENV = {}
for i,w in ipairs(WL) do
  local x, y, s = w[1], w[2], w[3]
  local hy = y - 32*s
  local cy = hy + 2*s
  local env = CROWN2[i]:grow(3*s) * mask(function(xx,yy) local a = math.atan(yy - cy, xx - x) + math.pi/2; return (math.abs(a) < 1.35 and yy < hy + 1*s) and 1 or 0 end)
  env = env:roughen(math.max(1.5, 4*s), math.max(5, 9*s), 800+i)
  ENV[i] = env
  local wid = math.max(1.1, 2.6*s)
  local core = env:shrink(3*s)
  stipple(core, {pile=lfD, width=wid, coverage=2.0, pressure={0.55,0.85}, cluster={0.5, 5*s+2}, dips={10,0.8,0.4}})
  local midm = env * mask(function(xx,yy) local d = (yy - (hy - 40*s))/(40*s) - (xx - x)/(80*s); return 1 - smoothstep(-0.2, 0.9, d) end)
  stipple(midm, {pile=lfM, width=wid, coverage=function(xx,yy) return 2.2*midm:at(xx,yy) end, pressure={0.5,0.85}, cluster={0.7, 5*s+2}, feather=0.6, dips={8,0.7,0.4}})
  local litm = env * mask(function(xx,yy) local dx, dy = (xx - x + 12*s)/(28*s), (yy - hy + 52*s)/(24*s); return 1 - smoothstep(0.2, 1.1, math.sqrt(dx*dx+dy*dy)) end)
  stipple(litm, {pile=lfS, width=wid*0.85, coverage=function(xx,yy) return 1.8*litm:at(xx,yy) end, pressure={0.5,0.8}, cluster={0.75, 4*s+2}, feather=0.7, dips={6,0.6,0.4}})
end
print(wait(0))

--@ chunk 78
print(drying(520,650), drying(500,600), drying(470,540), drying(700,600), drying(300,680))

--@ chunk 79
-- path darkened toward the foreground, cool shadow
local pS = pile{{"lead white",1.2},{"raw umber",0.9},{"yellow ochre",0.7},{"pale smalt",0.5},{"green earth",0.5}, medium=0.1}
local pM = pile{{"lead white",2.2},{"raw umber",0.7},{"yellow ochre",0.9},{"pale smalt",0.3},{"green earth",0.2}, medium=0.1}
local low = PATH * mask(function(x,y) return smoothstep(600, 660, y) end)
local mid = PATH * mask(function(x,y) return smoothstep(565, 600, y) * (1 - smoothstep(620, 660, y)) end)
work(mid, {hand="hatch", pile=pM, tool={kind="round", width=1.8, point=0.5}, angle=0.1, angle_jitter=0.3, length={3,9}, coverage=1.4, clip=true, threshold=0.4, load=0.6})
work(low, {hand="hatch", pile=pS, tool={kind="round", width=2.2, point=0.5}, angle=0.1, angle_jitter=0.3, length={4,12}, coverage=1.8, clip=true, threshold=0.4, load=0.6})
-- faint continuation of the path beyond the figures toward the river
local cont = ribbon({{484,540},{488,528},{496,518},{508,508},{522,500},{540,493}}, {3.2,2.6,2.1,1.6,1.2,0.8}) - FIGS:grow(0.5)
local pF = pile{{"lead white",3},{"yellow ochre",0.9},{"raw umber",0.35},{"green earth",0.4},{"pale smalt",0.2}, medium=0.1}
work(cont, {hand="detail", pile=pF, tool={kind="round", width=1.0, point=0.5}, angle=-0.5, length={3,8}, coverage=1.6, clip=true, fill=true})
print(wait(0))

--@ chunk 80
print(wait(60*26)); print(drying(500,520), drying(520,650), drying(446,500), drying(470,520))

--@ chunk 81
-- grass over the far path continuation and the path edges
local nearCont = rect(470, 490, 90, 55) - FIGS:grow(1)
local gm = pile{{"yellow ochre",1.4},{"green earth",1.4},{"Prussian blue",0.18},{"lead white",1.0},{"chrome yellow",0.2}, medium=0.1}
work(nearCont * LAND, {hand="hatch", pile=gm, tool={kind="round", width=1.2, point=0.8}, angle=-math.pi/2, angle_jitter=0.3, length={3,7}, coverage=1.0, pressure={0.35,0.7}, ramps={0.1,0.6}, seed=901})
-- blades breaking the path edges
local edgeBand = (PATH:grow(5) - PATH:shrink(3)) * mask(function(x,y) return y > 545 and 1 or 0 end)
local gd = pile{{"green earth",1.4},{"yellow ochre",1.1},{"Prussian blue",0.3},{"raw umber",0.5},{"lead white",0.5}, medium=0.1}
local rb = brush{kind="rigger", width=1.2, point=1}
rb:load(gd, 0.6)
local cnt = 0
for yy = 548, 704, 1.3 do
  for side = -1, 1, 2 do
    -- find edge x at this y by scanning
    local ex
    if side < 0 then
      for xx = 350, 700, 1 do if PATH:at(xx, yy) > 0.5 then ex = xx; break end end
    else
      for xx = 700, 350, -1 do if PATH:at(xx, yy) > 0.5 then ex = xx; break end end
    end
    if ex and math.random() < 0.75 then
      local sc = 0.4 + (yy - 540)/160
      local x0 = ex - side*rand(-2, 6)*sc
      local h = rand(4, 12)*sc
      local lean = side * rand(-0.1, 0.5)
      rb:stroke({{x0, yy}, {x0 + lean*h*0.5, yy - h*0.5}, {x0 + lean*h, yy - h}}, {pressure={0.5,0.0}})
      cnt = cnt + 1
      if rb:fullness() < 0.3 then rb:reload(gd, 0.6) end
    end
  end
end
print(cnt, wait(0))

--@ chunk 82
local FB2 = LAND * mask(function(x,y) return (y < rtop(x) - 0.3 or x < 345) and y < 492 and 1 or 0 end) - MILL:grow(0.5) - TOWN:grow(0.5) - FARTREES - RIVER:grow(0.3) - CROWNS_ALL:grow(0.5) - ENV[10]:grow(0.5) - ENV[9]:grow(0.5) - FIGS:grow(0.8) - poly{{86,463},{86,445},{118,445},{118,463}}
local nn = noise{seed=313, period=40, octaves=2}
local joinm = FB2 * mask(function(x,y) local d = math.abs(x - 380 + 25*nn(x,y)); return 1 - smoothstep(20, 70, d) end)
local jp = pile{{"lead white",2.0},{"green earth",1.8},{"yellow ochre",0.65},{"pale smalt",0.8},{"raw umber",0.18}, medium=0.1}
work(joinm, {hand="hatch", pile=jp, tool={kind="round", width=1.1, point=0.7}, angle=0.01, angle_jitter=0.1, length={4,12}, coverage=1.6, clip=true, seed=1001, threshold=0.2, load=0.6})
-- far hedgerows and tree clumps across the flat
local hedgeP = pile{{"lead white",1.8},{"green earth",1.5},{"pale smalt",1.0},{"raw umber",0.35}, medium=0.1}
local hedges = nil
local function add(m) hedges = hedges and (hedges + m) or m end
add(ribbon({{395,476},{450,475.5},{505,475}}, 1.1))
add(ribbon({{830,474},{900,473.5},{990,473}}, 1.0))
for _,c in ipairs({{412,473.5,5,3},{432,474,3,2.2},{520,472.5,6,3.5},{535,473,4,2.6},{848,471.8,5,3},{925,471.5,7,3.6},{940,472,4,2.5},{975,471.5,5,3}}) do
  add(ellipse(c[1], c[2], c[3], c[4]) * above(function(x) return c[2] + 2 end))
end
hedges = hedges:roughen(0.8, 3, 1002) - FIGS:grow(0.8) - TOWN
stipple(hedges, {pile=hedgeP, width=1.2, coverage=3, pressure={0.5,0.8}, cluster=0.3})
-- far cattle, tiny
local cow = pile{{"red earth",0.8},{"raw umber",0.8},{"lead white",1.2},{"pale smalt",0.4}, medium=0.08}
local cowW = pile{{"lead white",3},{"raw umber",0.2},{"pale smalt",0.3}, medium=0.08}
local r = brush{kind="round", width=1.0, point=0.3}
for _,c in ipairs({{870,478.5,cow},{878,479,cowW},{884,478.2,cow},{620,479.5,cowW}}) do
  r:reload(c[3], 0.5)
  r:stroke({{c[1]-1.6,c[2]},{c[1]+1.6,c[2]}}, {pressure={0.7,0.7}})
end
print(wait(0))

--@ chunk 83
local m = (ellipse(382, 470.5, 30, 4.5) - RIVER:grow(0.5)) * LAND
local p = pile{{"lead white",2.4},{"green earth",1.7},{"yellow ochre",0.7},{"pale smalt",0.8},{"raw umber",0.15}, medium=0.1}
work(m, {hand="hatch", pile=p, tool={kind="round", width=1.0, point=0.7}, angle=0.05, angle_jitter=0.1, length={4,12}, coverage=2.2, clip=true, seed=1101, threshold=0.2, load=0.6})
local m2 = (ellipse(386, 484, 12, 3) - RIVER:grow(0.3)) * LAND
work(m2, {hand="hatch", pile=p, tool={kind="round", width=1.0, point=0.7}, angle=0.0, angle_jitter=0.1, length={4,10}, coverage=1.5, clip=true, seed=1102, threshold=0.2, load=0.6})
print(wait(0))

--@ chunk 84
print(wait(60*48)); print(drying(520,650), drying(446,500), drying(470,520), drying(688,520), drying(380,470))

--@ chunk 85
local REG = LAND * mask(function(x,y) return (x < 440 and y > 452 and y < 600) and 1 or 0 end) - MILL:grow(1) - FIGS:grow(1) - poly{{84,464},{84,443},{120,443},{120,464}} - PATH:shrink(1)
COVREG = REG
local nz = noise{seed=77, period=120, octaves=3}
local function z(a,b) return REG * mask(function(x,y) local yy = y + 5*nz(x,y*3); return smoothstep(a-5,a+5,yy)*(1-smoothstep(b-5,b+5,yy)) end) end
local riseC = pile{{"lead white",1.8},{"green earth",1.9},{"yellow ochre",0.7},{"pale smalt",0.75},{"raw umber",0.2}, medium=0.1}
local farC  = pile{{"lead white",1.6},{"green earth",1.7},{"yellow ochre",1.0},{"pale smalt",0.4},{"Prussian blue",0.05},{"raw umber",0.15}, medium=0.1}
local litC  = pile{{"lead white",1.2},{"yellow ochre",1.7},{"chrome yellow",0.5},{"green earth",0.9},{"Prussian blue",0.12}, medium=0.1}
local shC   = pile{{"lead white",0.9},{"green earth",1.5},{"yellow ochre",1.0},{"Prussian blue",0.3},{"raw umber",0.4},{"pale smalt",0.3}, medium=0.1}
local Z = {{riseC,440,488,"filbert 5",{8,20}},{farC,484,506,"filbert 5",{10,25}},{litC,503,532,"filbert 6",{12,30}},{shC,529,578,"filbert 7",{15,35}},{litC,575,610,"filbert 8",{15,40}}}
for i,zz in ipairs(Z) do
  work(z(zz[2],zz[3]), {hand="body", pile=zz[1], tool=zz[4], clip=REG, angle=0, angle_jitter=0.05, length=zz[5], coverage=2.2, fill=true, load=0.8, threshold=0.4})
end
print(wait(0))

--@ chunk 86
local m = COVREG * mask(function(x,y) return y > 486 and 1 or 0 end)
blend(m, {angle=1.3, coverage=1.2, tool={kind="badger", width=24}})
blend(m, {angle=0, coverage=2.0, tool={kind="badger", width=30}})
blend(COVREG * mask(function(x,y) return y < 500 and 1 or 0 end), {angle=0, coverage=1.5, tool={kind="badger", width=20}})
print(wait(0))

--@ chunk 87
local REG = (HORSES:grow(3) + ellipse(700,534.5,24,4) + ellipse(733,531.5,23,4)) * LAND
HREG = REG
local litC  = pile{{"lead white",1.2},{"yellow ochre",1.7},{"chrome yellow",0.5},{"green earth",0.9},{"Prussian blue",0.12}, medium=0.1}
local mixC = pile{{"lead white",1.0},{"yellow ochre",1.5},{"chrome yellow",0.3},{"green earth",1.1},{"Prussian blue",0.18},{"raw umber",0.1}, medium=0.1}
work(REG, {hand="body", pile=litC, tool="filbert 5", clip=REG, angle=0, length={8,20}, coverage=2.4, fill=true, load=0.8})
work(REG * mask(function(x,y) return y > 527 and 1 or 0 end), {hand="body", pile=mixC, tool="filbert 4", clip=REG, angle=0, length={8,20}, coverage=1.5, fill=true, load=0.7})
-- remove the tiny far cattle
local cows = ellipse(877,478.7,11,2.5) + ellipse(620,479.5,3,2)
local farC = pile{{"lead white",2.2},{"green earth",1.6},{"yellow ochre",0.6},{"pale smalt",0.9},{"Prussian blue",0.03}, medium=0.1}
work(cows * LAND - RIVER:grow(0.3), {hand="detail", pile=farC, tool={kind="round", width=1.2, point=0.5}, angle=0, length={3,8}, coverage=3, clip=true, fill=true})
print(wait(0))

--@ chunk 88
print(wait(60*60)); print(drying(200,540), drying(200,480), drying(700,520), drying(446,500))

--@ chunk 89
local R = (COVREG + HREG) * mask(function(x,y) return y > 488 and 1 or 0 end)
local nb = noise{seed=131, period=90, octaves=3, stretch={0, 3}}
local patchy = mask(function(x,y) return smoothstep(0.42, 0.6, nb:at01(x,y)) end)
local mixP = pile{{"yellow ochre",1.3},{"green earth",1.5},{"Prussian blue",0.22},{"lead white",0.9},{"raw umber",0.25},{"pale smalt",0.15}, medium=0.1}
local function zz(a,b) return R * zoneN(a,b,8,6) end
mpass(zz(488, 512), G.far, 500, 1.3, 2001)
mpass(zz(505, 535), gP.lit, 520, 1.4, 2002)
mpass(zz(530, 578), gP.sh, 552, 1.5, 2003)
mpass(zz(575, 610), gP.lit, 592, 1.4, 2004)
mpass(zz(495, 610) * patchy, mixP, 550, 0.8, 2005)
print(wait(0))

--@ chunk 90
PW = {
 {x=15,  y=545,   H=219, lean=-0.10, seed=1},
 {x=100, y=535,   H=191, lean=0.06,  seed=2},
 {x=262, y=515.7, H=136, lean=-0.04, seed=3},
 {x=330, y=507.7, H=113, lean=0.08,  seed=4},
 {x=382, y=501.5, H=95,  lean=-0.02, seed=5},
}
local barkD = pile{{"raw umber",1},{"bone black",0.35},{"green earth",0.5},{"lead white",0.45},{"pale smalt",0.2}, medium=0.08}
local barkM = pile{{"lead white",1.3},{"raw umber",0.7},{"green earth",0.5},{"pale smalt",0.4},{"bone black",0.1},{"yellow ochre",0.2}, medium=0.08}
local barkL = pile{{"lead white",2.6},{"raw umber",0.4},{"yellow ochre",0.4},{"pale smalt",0.4},{"green earth",0.2}, medium=0.08}
PWTR = {}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local th = 0.42*H
  local function P(u, v) -- u across (in H), v up (in H)
    local yy = y0 - v*H
    local xx = x0 + u*H + t.lean*(v*H)
    return {xx, yy}
  end
  local pts = {P(-0.085,0.0), P(-0.07,0.03), P(-0.058,0.12), P(-0.055,0.25), P(-0.07,0.33), P(-0.1,0.38), P(-0.095,0.43), P(-0.05,0.445), P(-0.01,0.43), P(0.03,0.45), P(0.08,0.435), P(0.1,0.39), P(0.07,0.34), P(0.055,0.25), P(0.058,0.12), P(0.068,0.03), P(0.09,0.0)}
  local m = poly(pts, true):roughen(math.max(0.8, H*0.006), math.max(4, H*0.03), 1300+i)
  m = m * above(function(x) return y0 + 1.5 end)
  PWTR[i] = m
  local wbr = math.max(1.2, H*0.012)
  work(m, {hand="detail", pile=barkM, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, length={3,12}, coverage=3, clip=true, fill=true, edge="found"})
  local sh = m * mask(function(xx,yy) local v = (y0 - yy)/H; local cx = x0 + t.lean*(y0-yy); return smoothstep(cx - 0.005*H, cx + 0.04*H, xx) end)
  work(sh, {hand="detail", pile=barkD, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, length={3,12}, coverage=2.5, clip=true, fill=true})
  local lit = m * mask(function(xx,yy) local cx = x0 + t.lean*(y0-yy); return smoothstep(cx - 0.03*H, cx - 0.06*H, xx) end)
  work(lit, {hand="detail", pile=barkL, tool={kind="round", width=wbr*0.8, point=0.4}, angle=math.pi/2, length={3,10}, coverage=1.5, clip=true})
end
print(wait(0))

--@ chunk 91
local rodP = pile{{"raw umber",1},{"green earth",0.8},{"bone black",0.15},{"lead white",0.6},{"yellow ochre",0.3}, medium=0.08}
PWCR = {}
PWRODS = {}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  local rods = {}
  local n = math.floor(22 + H*0.08)
  local crown = nil
  local rb = brush{kind="rigger", width=math.max(0.6, H*0.007), point=1}
  rb:load(rodP, 0.6)
  for k=1,n do
    local a = -math.pi/2 + randn(0, 0.5) + t.lean*0.5
    a = clamp(a, -math.pi/2 - 1.25, -math.pi/2 + 1.25)
    local L = H*rand(0.3, 0.58) * (1 - 0.25*math.abs(a + math.pi/2))
    local sx, sy = hx + rand(-0.07,0.07)*H, hy + rand(-0.01, 0.02)*H
    local bend = rand(-0.12, 0.12)
    local pts = {}
    for j=0,4 do
      local f = j/4
      local aa = a + bend*f + (a + math.pi/2)*0.25*f
      pts[#pts+1] = {sx + math.cos(aa)*L*f, sy + math.sin(aa)*L*f}
    end
    rods[#rods+1] = pts
    rb:stroke(pts, {pressure={0.6,0.05}})
    if rb:fullness() < 0.3 then rb:reload(rodP, 0.6) end
    for j=2,5 do
      local p = pts[j]
      local r = H*rand(0.035, 0.07)*(0.6 + 0.12*j)
      local e = ellipse(p[1] + rand(-0.02,0.02)*H, p[2], r, r*0.8)
      crown = crown and (crown + e) or e
    end
  end
  crown = crown:roughen(math.max(1.2, H*0.012), math.max(4, H*0.035), 1400+i)
  PWCR[i] = crown
  PWRODS[i] = rods
end
print(wait(0))

--@ chunk 92
local lD = pile{{"green earth",1.7},{"raw umber",0.55},{"Prussian blue",0.2},{"yellow ochre",0.5},{"lead white",0.6}, medium=0.08}
local lM = pile{{"lead white",1.5},{"green earth",1.8},{"yellow ochre",0.6},{"pale smalt",0.6},{"raw umber",0.2}, medium=0.08}
local lS = pile{{"lead white",3.2},{"green earth",1.2},{"pale smalt",0.7},{"yellow ochre",0.4}, medium=0.08}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  local cr = PWCR[i]
  local wid = math.max(1.4, H*0.022)
  -- darker underside & core
  stipple(cr, {pile=lD, width=wid, coverage=function(xx,yy) local v = (hy - yy)/(0.55*H); return 1.6 - 0.9*smoothstep(0.2, 0.9, v) end, pressure={0.5,0.85}, cluster={0.7, H*0.04}, dips={10,0.7,0.4}})
  -- middle grey-green
  stipple(cr, {pile=lM, width=wid, coverage=function(xx,yy) local v = (hy - yy)/(0.55*H); return 0.4 + 1.2*smoothstep(0.1, 0.7, v) end, pressure={0.5,0.85}, cluster={0.75, H*0.035}, feather=0.4, dips={8,0.7,0.4}})
  -- silver light from upper left
  local lit = cr * mask(function(xx,yy) local u = (xx - hx)/(0.5*H); local v = (hy - yy)/(0.55*H); return smoothstep(0.35, 0.95, v - 0.45*u) end)
  stipple(lit, {pile=lS, width=wid*0.85, coverage=function(xx,yy) return 1.6*lit:at(xx,yy) end, pressure={0.45,0.8}, cluster={0.8, H*0.03}, feather=0.7, dips={6,0.6,0.4}})
end
print(wait(0))

--@ chunk 93
local rvA = pile{{"lead white",4},{"pale smalt",1.4},{"smalt",0.35},{"green earth",0.25},{"raw umber",0.08}, medium=0.1}
local rvB = pile{{"lead white",7},{"pale smalt",1.0},{"yellow ochre",0.08}, medium=0.1}
local lower = RIVER * mask(function(x,y) return smoothstep(rtop(x)+0.8, rtop(x)+2.2, y) end)
work(lower, {hand="detail", pile=rvA, tool={kind="round", width=1.2, point=0.5}, angle=0, length={10,30}, coverage=2.2, clip=true, fill=true, load=0.6})
-- reflections of far bank trees and town: soft vertical dabs
local refl = pile{{"lead white",3},{"pale smalt",1.3},{"smalt",0.3},{"raw umber",0.25},{"green earth",0.2}, medium=0.1}
local rb = brush{kind="round", width=1.4, point=0.3}
rb:load(refl, 0.5)
for _,x in ipairs({596,600,604,640,660,678,680,682,700,725,757,760,763,790,810}) do
  local yt = rtop(x) + 0.5
  rb:stroke({{x, yt}, {x + rand(-0.3,0.3), yt + rand(2.5, 5)}}, {pressure={0.5,0.3}})
  if rb:fullness() < 0.3 then rb:reload(refl, 0.5) end
end
-- a few horizontal glints
local g = brush{kind="round", width=0.9, point=0.5}
g:load(rvB, 0.5)
for k=1,14 do
  local x = rand(360, 990)
  local y = rtop(x) + rand(2.5, 4.5)
  g:stroke({{x,y},{x+rand(6,20),y}}, {pressure={0.4,0.3}})
end
print(wait(0))

--@ chunk 94
print(drying(680,440), drying(600,460), drying(760,420))
townP3 = pile{{"lead white",2.2},{"smalt",1.8},{"bone black",0.3},{"raw umber",0.2},{"pale smalt",0.6}, medium=0.08}
work(TOWN, {hand="detail", pile=townP3, tool={kind="round", width=1.4, point=0.5}, angle=math.pi/2, length={3,10}, coverage=3, fill=true, clip=true, edge="found", load=0.8})
print(wait(0))

--@ chunk 95
local bay = pile{{"red earth",1.2},{"raw umber",1.0},{"bone black",0.3},{"lead white",0.3}, medium=0.08}
local bayL = pile{{"red earth",1},{"yellow ochre",0.7},{"raw umber",0.4},{"lead white",0.7}, medium=0.08}
local blk = pile{{"bone black",1},{"raw umber",0.6},{"lead white",0.25},{"pale smalt",0.2}, medium=0.08}
local blkL = pile{{"bone black",0.5},{"raw umber",0.5},{"lead white",0.9},{"pale smalt",0.4}, medium=0.08}
local function P2(x0,y0,s,dir) return function(u,v) return {x0 + dir*u*s, y0 - v*s} end end
function horse2(x0, y0, s, dir, grazing)
  local P = P2(x0,y0,s,dir)
  local body = poly({P(-0.85,1.02),P(-0.84,1.3),P(-0.55,1.45),P(0.1,1.38),P(0.55,1.47),P(0.85,1.3),P(0.83,0.98),P(0.45,0.84),P(-0.3,0.86),P(-0.75,0.9)}, true)
  local neck, head
  if grazing then
    neck = poly({P(0.5,1.45),P(0.82,1.4),P(1.05,1.0),P(1.18,0.5),P(1.02,0.45),P(0.85,0.92),P(0.62,1.0)}, true)
    head = poly({P(1.0,0.58),P(1.2,0.55),P(1.3,0.14),P(1.2,0.06),P(1.08,0.12),P(0.98,0.42)}, true)
  else
    neck = poly({P(0.45,1.42),P(0.72,1.62),P(0.92,1.92),P(1.1,2.0),P(1.1,1.62),P(0.95,1.2),P(0.8,0.98),P(0.5,0.95)}, true)
    head = poly({P(0.98,1.98),P(1.12,2.06),P(1.47,1.68),P(1.43,1.56),P(1.28,1.58),P(1.0,1.74)}, true)
  end
  local legs = ribbon({P(-0.64,1.0),P(-0.72,0.5),P(-0.68,0.04)}, 0.1*s) + ribbon({P(-0.48,0.95),P(-0.45,0.5),P(-0.5,0.04)}, 0.09*s)
             + ribbon({P(0.6,0.95),P(0.62,0.5),P(0.64,0.04)}, 0.09*s) + ribbon({P(0.46,0.95),P(0.42,0.5),P(0.38,0.04)}, 0.09*s)
  local tail = ribbon({P(-0.83,1.28),P(-0.93,0.95),P(-0.9,0.55)}, {0.12*s, 0.11*s, 0.05*s})
  return body + neck + head + legs + tail, P
end
local h1, P1 = horse2(772, 503, 19, 1, true)
local h2, P2b = horse2(838, 501.5, 17.5, -1, false)
HORSES2 = h1 + h2
local t = {kind="round", width=1.4, point=0.5}
work(h1, {hand="detail", pile=bay, tool=t, angle=0, length={3,8}, coverage=3, clip=true, fill=true, edge="found"})
work(h1 * mask(function(x,y) return y < 503 - 1.2*19 and 1 or 0 end), {hand="detail", pile=bayL, tool={kind="round", width=1.1, point=0.5}, angle=0, length={3,7}, coverage=1.3, clip=true})
work(h2, {hand="detail", pile=blk, tool=t, angle=0, length={3,8}, coverage=3, clip=true, fill=true, edge="found"})
work(h2 * mask(function(x,y) return y < 501.5 - 1.25*17.5 and 1 or 0 end), {hand="detail", pile=blkL, tool={kind="round", width=1.0, point=0.5}, angle=0, length={3,7}, coverage=1.0, clip=true})
-- manes, tails, hooves
local dark = pile{{"bone black",1},{"raw umber",0.5},{"lead white",0.15}, medium=0.08}
local r = brush{kind="rigger", width=1.0, point=1}
r:load(dark, 0.6)
r:stroke({P1(0.55,1.47),P1(0.83,1.4),P1(1.05,1.0),P1(1.16,0.55)}, {pressure={0.7,0.3}})
r:stroke({P1(-0.84,1.3),P1(-0.95,0.95),P1(-0.9,0.5)}, {pressure={0.8,0.2}})
r:reload(dark, 0.6)
r:stroke({P2b(0.72,1.62),P2b(0.92,1.92),P2b(1.08,2.02)}, {pressure={0.7,0.4}})
-- blaze on the bay's face
local w = brush{kind="round", width=0.8, point=0.6}
w:load(pile{{"lead white",3},{"yellow ochre",0.2}}, 0.4)
w:stroke({P1(1.15,0.5),P1(1.22,0.2)}, {pressure={0.5,0.3}})
print(wait(0))

--@ chunk 96
local f = 1000
local function proj(X, Y, Z) return 500 + f*X/Z, 468 + f*(1.75 - Y)/Z end
local wood = pile{{"raw umber",1},{"lead white",0.8},{"bone black",0.25},{"pale smalt",0.3},{"yellow ochre",0.2}, medium=0.08}
local woodL = pile{{"lead white",2.4},{"raw umber",0.5},{"yellow ochre",0.4},{"pale smalt",0.3}, medium=0.08}
local woodD = pile{{"bone black",0.8},{"raw umber",1},{"lead white",0.25}, medium=0.08}
FENCE = nil
local Zs = {}
local z = 13.2
while z < 75 do Zs[#Zs+1] = z; z = z * 1.0 + 2.9 + rand(-0.2,0.2) end
local posts = {}
for i,Z in ipairs(Zs) do
  local X = 7 + 0.02*i + rand(-0.06,0.06)
  local hgt = 1.15 + rand(-0.08, 0.08)
  local lean = rand(-0.04, 0.04)
  local w = 0.13
  local xb, yb = proj(X, 0, Z)
  local xt, yt = proj(X + lean, hgt, Z)
  local hw = f*w/Z/2
  local pts = {{xb-hw, yb+0.4}, {xt-hw*0.9, yt+hw*0.3}, {xt-hw*0.3, yt-hw*0.2}, {xt+hw*0.9, yt}, {xb+hw, yb+0.4}}
  local m = poly(pts)
  posts[i] = {xb=xb, yb=yb, xt=xt, yt=yt, hw=hw, Z=Z, X=X, lean=lean}
  FENCE = FENCE and (FENCE + m) or m
  local tw = math.max(0.7, hw*0.5)
  work(m, {hand="detail", pile=wood, tool={kind="round", width=tw, point=0.4}, angle=math.pi/2, length={2,10}, coverage=3, clip=true, fill=true, edge="found"})
  if hw > 1.2 then
    local lit = m * mask(function(x,y) return x < lerp(xb, xt, (yb-y)/(yb-yt)) - hw*0.2 and 1 or 0 end)
    work(lit, {hand="detail", pile=woodL, tool={kind="round", width=math.max(0.6,tw*0.7), point=0.4}, angle=math.pi/2, length={2,8}, coverage=1.5, clip=true})
  end
end
FPOSTS = posts
-- rails: two, from post to post, slight sag
local rails = nil
for i=1,#posts-1 do
  local a, b = posts[i], posts[i+1]
  for _,Y in ipairs({0.95, 0.5}) do
    local x1, y1 = proj(a.X + a.lean*Y/1.15, Y, a.Z)
    local x2, y2 = proj(b.X + b.lean*Y/1.15, Y, b.Z)
    local w1, w2 = f*0.09/a.Z, f*0.09/b.Z
    local mx, my = (x1+x2)/2, (y1+y2)/2 + f*0.03/((a.Z+b.Z)/2)
    local rm = ribbon({{x1,y1},{mx,my},{x2,y2}}, {math.max(0.8,w1), math.max(0.7,(w1+w2)/2), math.max(0.6,w2)})
    rails = rails and (rails + rm) or rm
  end
end
RAILS = rails - FENCE
work(RAILS, {hand="detail", pile=wood, tool={kind="round", width=1.0, point=0.4}, angle=-0.6, length={4,14}, coverage=3, clip=true, fill=true, edge="found"})
local rtop = RAILS * mask(function(x,y) return 1 end) - RAILS:offset(-1.0)
print(#posts, wait(0))

--@ chunk 97
local wx = 470
W_SHAWL = poly({{wx-7,482.5},{wx-8.5,486},{wx-7.5,492},{wx-2,503},{wx,506},{wx+2,503},{wx+7.5,492},{wx+8.5,486},{wx+7,482.5},{wx,480.5}}, true)
local band = rect(400, 482.5, 120, 8)
local mc = (MAN_COAT + MAN_COLLAR) * band
work(mc, {hand="detail", pile=coatP, tool={kind="round", width=1.0, point=0.5}, angle=math.pi/2, length={2,6}, coverage=3.5, clip=true, fill=true, load=0.9})
local mcl = mc * mask(function(x,y) return smoothstep(446-6, 446-10, x) end)
work(mcl, {hand="detail", pile=coatL, tool={kind="round", width=0.9, point=0.5}, angle=math.pi/2, length={2,6}, coverage=1.2, clip=true})
local cream = pile{{"lead white",4},{"yellow ochre",0.45},{"raw umber",0.1},{"pale smalt",0.05}, medium=0.06}
local creamS = pile{{"lead white",2.2},{"yellow ochre",0.4},{"raw umber",0.3},{"pale smalt",0.35}, medium=0.06}
work(W_SHAWL, {hand="detail", pile=cream, tool={kind="round", width=1.0, point=0.5}, angle=0.4, length={2,6}, coverage=3.5, clip=true, fill=true, edge="found", load=0.9})
local ssh = W_SHAWL * mask(function(x,y) return smoothstep(wx+0.5, wx+6, x) end)
work(ssh, {hand="detail", pile=creamS, tool={kind="round", width=0.8, point=0.5}, angle=math.pi/2, length={2,6}, coverage=1.6, clip=true})
-- neck band of hair/head repaired
local hd = ellipse(wx, 475.5, 4.6, 5.6) * rect(460, 479, 20, 4)
work(hd, {hand="detail", pile=hairP, tool={kind="round", width=0.8, point=0.5}, angle=0, length={2,5}, coverage=3, clip=true, fill=true})
print(wait(0))

--@ chunk 98
print(wait(60*54)); print(drying(446,486), drying(772,490), drying(838,490), drying(100,400), drying(930,540))

--@ chunk 99
local bay = pile{{"red earth",1.1},{"raw umber",1.1},{"bone black",0.35},{"lead white",0.2}, medium=0.06}
local bayL = pile{{"red earth",1},{"yellow ochre",0.8},{"raw umber",0.4},{"lead white",0.6}, medium=0.06}
local blk = pile{{"bone black",1},{"raw umber",0.5},{"lead white",0.12},{"pale smalt",0.1}, medium=0.06}
local blkL = pile{{"bone black",0.5},{"raw umber",0.4},{"lead white",0.9},{"pale smalt",0.5}, medium=0.06}
local h1 = HORSES2 * rect(740, 470, 70, 40)
local h2 = HORSES2 * rect(810, 460, 60, 50)
local t = {kind="round", width=1.2, point=0.5}
work(h1, {hand="detail", pile=bay, tool=t, angle=0.2, length={2,6}, coverage=3.5, clip=true, fill=true, edge="found", load=0.9})
work(h1 * mask(function(x,y) return y < 503 - 1.25*19 and 1 or 0 end), {hand="detail", pile=bayL, tool={kind="round", width=1.0, point=0.5}, angle=0, length={2,6}, coverage=1.2, clip=true})
work(h2, {hand="detail", pile=blk, tool=t, angle=0.2, length={2,6}, coverage=3.5, clip=true, fill=true, edge="found", load=0.9})
work(h2 * mask(function(x,y) return y < 501.5 - 1.3*17.5 and 1 or 0 end), {hand="detail", pile=blkL, tool={kind="round", width=0.9, point=0.5}, angle=0, length={2,6}, coverage=0.8, clip=true})
-- shadows under them
local sh = pile{{"green earth",1.4},{"raw umber",0.7},{"Prussian blue",0.25},{"yellow ochre",0.4},{"lead white",0.3}, medium=0.08}
local shm = (ellipse(776, 503.5, 20, 1.4) + ellipse(834, 502, 17, 1.3)) - HORSES2
work(shm, {hand="detail", pile=sh, tool={kind="round", width=0.9, point=0.5}, angle=0, length={3,8}, coverage=1.5, clip=true})
print(wait(0))

--@ chunk 100
local shG = pile{{"green earth",1.6},{"Prussian blue",0.3},{"raw umber",0.6},{"yellow ochre",0.7},{"lead white",0.35}, medium=0.1}
local tuft = pile{{"green earth",1.3},{"yellow ochre",1.1},{"Prussian blue",0.22},{"raw umber",0.4},{"lead white",0.5}, medium=0.08}
local tuftL = pile{{"yellow ochre",1.4},{"chrome yellow",0.4},{"green earth",0.8},{"lead white",1.2},{"Prussian blue",0.08}, medium=0.08}
local ALLPW = PWTR[1]
for i=2,#PWTR do ALLPW = ALLPW + PWTR[i] end
PWALL = ALLPW
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local s = H/200
  -- cast shadow of the crown, to the right and a little forward, on the grass
  local sm = (ellipse(x0 + 0.32*H, y0 + 0.03*H, 0.42*H, 0.035*H) + ellipse(x0 + 0.1*H, y0 + 0.01*H, 0.12*H, 0.02*H)):roughen(2, 8, 1500+i) - ALLPW - FIGS:grow(1) - PATH
  work(sm, {hand="hatch", pile=shG, tool={kind="round", width=math.max(1.0, 2*s), point=0.8}, angle=-math.pi/2, angle_jitter=0.3, length={3*s+2, 9*s+3}, coverage=1.8, pressure={0.4,0.75}, ramps={0.1,0.6}, clip=sm:grow(2)})
  -- grass tufts over the trunk foot
  local g = brush{kind="rigger", width=math.max(0.6, 1.3*s), point=1}
  g:load(tuft, 0.6)
  for k=1,math.floor(12 + 20*s) do
    local xx = x0 + rand(-0.12, 0.12)*H
    local yb = y0 + rand(0, 0.015)*H
    local p = (k % 3 == 0) and tuftL or tuft
    if k % 3 == 0 then g:reload(p, 0.5) end
    g:stroke({{xx, yb}, {xx + rand(-0.02,0.02)*H, yb - rand(0.02,0.06)*H}}, {pressure={0.6,0.0}})
    if g:fullness() < 0.25 then g:reload(tuft, 0.6) end
  end
end
print(wait(0))

--@ chunk 101
local m = LAND * rect(0, 495, 440, 90) - PWALL:grow(1) - FIGS:grow(2) - PATH
blend(m, {angle=0, coverage=2.0, tool={kind="badger", width=18}, pressure={0.4,0.6}})
blend(m, {angle=-1.4, coverage=1.0, tool={kind="badger", width=12}, pressure={0.3,0.5}})
print(wait(0))

--@ chunk 102
print(wait(60*72)); print(drying(100,540), drying(250,530), drying(446,486), drying(15,420))

--@ chunk 103
local R = LAND * rect(0, 492, 445, 100) - PWALL:grow(0.5) - FIGS:grow(1.5) - PATH:shrink(1)
MREP = R
local nz = noise{seed=701, period=120, octaves=3}
local function z(a,b) return R * mask(function(x,y) local yy = y + 5*nz(x,y*3); return smoothstep(a-6,a+6,yy)*(1-smoothstep(b-6,b+6,yy)) end) end
local farC  = pile{{"lead white",1.5},{"green earth",1.7},{"yellow ochre",1.0},{"pale smalt",0.4},{"Prussian blue",0.05},{"raw umber",0.15}, medium=0.1}
local litC  = pile{{"lead white",1.2},{"yellow ochre",1.7},{"chrome yellow",0.45},{"green earth",0.9},{"Prussian blue",0.12}, medium=0.1}
local midC  = pile{{"lead white",0.9},{"yellow ochre",1.5},{"chrome yellow",0.2},{"green earth",1.2},{"Prussian blue",0.2},{"raw umber",0.15}, medium=0.1}
work(z(488,512), {hand="body", pile=farC, tool="filbert 5", clip=R, angle=0, length={8,20}, coverage=2.4, fill=true, load=0.85, threshold=0.4})
work(z(508,548), {hand="body", pile=litC, tool="filbert 6", clip=R, angle=0, length={10,26}, coverage=2.4, fill=true, load=0.85, threshold=0.4})
work(z(544,600), {hand="body", pile=midC, tool="filbert 7", clip=R, angle=0, length={10,30}, coverage=2.4, fill=true, load=0.85, threshold=0.4})
print(wait(0))

--@ chunk 104
blend(MREP, {angle=0, coverage=2.0, tool={kind="badger", width=24}, clip=true})
print(wait(60*20)); print(drying(200,540), drying(200,580))

--@ chunk 105
local barkD = pile{{"raw umber",1},{"bone black",0.4},{"green earth",0.6},{"lead white",0.5},{"pale smalt",0.2}, medium=0.06}
local barkM = pile{{"lead white",1.4},{"raw umber",0.6},{"green earth",0.6},{"pale smalt",0.45},{"bone black",0.12},{"yellow ochre",0.25}, medium=0.06}
local barkL = pile{{"lead white",2.8},{"raw umber",0.35},{"yellow ochre",0.45},{"pale smalt",0.35},{"green earth",0.2}, medium=0.06}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local m = PWTR[i] - PWCR[i]:shrink(H*0.02)
  local wbr = math.max(1.2, H*0.011)
  work(m, {hand="detail", pile=barkM, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, length={3,12}, coverage=3.5, clip=true, fill=true, edge="found", load=1.0})
  local sh = m * mask(function(xx,yy) local cx = x0 + t.lean*(y0-yy); return smoothstep(cx + 0.0*H, cx + 0.045*H, xx) end)
  work(sh, {hand="detail", pile=barkD, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, length={3,12}, coverage=2.5, clip=true, fill=true, load=0.9})
  local lit = m * mask(function(xx,yy) local cx = x0 + t.lean*(y0-yy); return smoothstep(cx - 0.025*H, cx - 0.06*H, xx) end)
  work(lit, {hand="detail", pile=barkL, tool={kind="round", width=wbr*0.8, point=0.4}, angle=math.pi/2, length={3,10}, coverage=1.6, clip=true, load=0.9})
end
print(wait(0))

--@ chunk 106
print(wait(60*60)); print(drying(100,500), drying(250,480), drying(200,560), drying(446,486), drying(100,400))

--@ chunk 107
print(wait(60*30)); print(drying(100,500), drying(15,480), drying(200,560), drying(262,480))

--@ chunk 108
print(drying(700,650), drying(200,680), drying(900,600), drying(520,680), drying(446,500))

--@ chunk 109
local m = MAN_COAT + MAN_COLLAR
local coatP2 = pile{{"bone black",1},{"Prussian blue",0.15},{"raw umber",0.8},{"green earth",0.5},{"lead white",0.3}, medium=0.06}
local coatL2 = pile{{"bone black",0.5},{"raw umber",0.6},{"green earth",0.6},{"lead white",1.2},{"pale smalt",0.3}, medium=0.06}
work(m, {hand="detail", pile=coatP2, tool={kind="round", width=1.1, point=0.5}, angle=math.pi/2, length={3,10}, coverage=3.5, clip=true, fill=true, edge="found", load=0.9})
local lit = m * mask(function(x,y) return smoothstep(446-5, 446-10, x) end)
work(lit, {hand="detail", pile=coatL2, tool={kind="round", width=0.9, point=0.5}, angle=math.pi/2, length={4,12}, coverage=1.4, clip=true, load=0.7})
-- fold lines
local r = brush{kind="rigger", width=0.7, point=1}
r:load(pile{{"bone black",1},{"raw umber",0.4}}, 0.4)
r:stroke({{444,500},{443.5,520},{443,533}}, {pressure={0.3,0.1}})
r:stroke({{450,505},{451,522},{452,532}}, {pressure={0.3,0.1}})
-- the woman's arm linked: a sleeve line
print(wait(0))

--@ chunk 110
local FGM = LAND * mask(function(x,y) return y > 600 and 1 or 0 end) - PATH:shrink(2)
local tall = pile{{"yellow ochre",1.5},{"green earth",1.2},{"lead white",1.0},{"Prussian blue",0.1},{"chrome yellow",0.2}, medium=0.08}
local tallD = pile{{"green earth",1.4},{"Prussian blue",0.3},{"raw umber",0.6},{"yellow ochre",0.6},{"lead white",0.3}, medium=0.08}
local seedP = pile{{"yellow ochre",1.3},{"raw umber",0.5},{"red earth",0.2},{"lead white",1.2}, medium=0.08}
local buttercup = pile{{"chrome yellow",2},{"yellow ochre",0.3},{"lead white",0.4}, medium=0.06}
local daisy = pile{{"lead white",6},{"yellow ochre",0.08}, medium=0.05}
local sorrel = pile{{"red earth",1},{"vermilion",0.5},{"raw umber",0.3},{"lead white",0.3}, medium=0.06}
local clover = pile{{"lead white",3},{"vermilion",0.3},{"red earth",0.2},{"pale smalt",0.3}, medium=0.06}
-- tall grass stems with seed heads
local rb = brush{kind="rigger", width=1.0, point=1}
local sb = brush{kind="round", width=1.8, point=0.7}
rb:load(tallD, 0.6); sb:load(seedP, 0.6)
local n = 0
for k=1,260 do
  local x, y = rand(0, 1000), rand(625, 712)
  if FGM:at(x, math.min(y,703)) > 0.5 then
    local sc = 0.6 + (y - 600)/110
    local h = rand(28, 70)*sc
    local bend = rand(-0.25, 0.25)
    local p = (k % 2 == 0) and tall or tallD
    rb:reload(p, 0.5)
    local tx, ty = x + bend*h, y - h
    rb:stroke({{x, y}, {x + bend*h*0.35, y - h*0.5}, {tx, ty}}, {pressure={0.55,0.15}})
    if k % 3 == 0 then
      sb:reload(seedP, 0.5)
      local a = math.atan(ty - (y - h*0.5), tx - (x + bend*h*0.35))
      sb:stroke({{tx, ty}, {tx + math.cos(a)*6*sc, ty + math.sin(a)*6*sc}}, {pressure={0.6,0.1}})
    end
    n = n + 1
  end
end
-- flowers: buttercups in the middle band and foreground, daisies, sorrel, clover
local ft = brush{kind="round", width=2.2, point=0.2}
local function dots(p, cnt, y0, y1, size)
  ft:reload(p, 0.6)
  for k=1,cnt do
    local x, y = rand(0,1000), rand(y0,y1)
    if FGM:at(x,y) > 0.5 or (y < 600 and LAND:at(x,y) > 0.5 and PATH:at(x,y) < 0.3 and FIGS:at(x,y) < 0.1 and PWALL:at(x,y) < 0.1 and FENCE:at(x,y) < 0.1 and RAILS:at(x,y) < 0.1) then
      local s = size * (0.35 + (y - 500)/200)
      ft:touch(x, y, {pressure=clamp(s, 0.15, 0.9)})
      if ft:fullness() < 0.3 then ft:reload(p, 0.6) end
    end
  end
end
dots(buttercup, 120, 560, 700, 0.8)
dots(buttercup, 90, 520, 600, 0.5)
dots(daisy, 70, 600, 700, 0.7)
dots(sorrel, 50, 580, 700, 0.6)
dots(clover, 30, 620, 700, 0.8)
print(n, wait(0))

--@ chunk 111
print(wait(60*40)); print(drying(100,500), drying(15,480), drying(200,560), drying(262,480), drying(300,540))

--@ chunk 112
local R = LAND * rect(0, 486, 445, 125) - PWALL:grow(0.5) - FIGS:grow(1) - PATH:shrink(2)
local nz = noise{seed=909, period=140, octaves=3}
local function z(a,b,s) return R * mask(function(x,y) local yy = y + 6*nz(x,y*2); return smoothstep(a-s,a+s,yy)*(1-smoothstep(b-s,b+s,yy)) end) end
local mixP = pile{{"yellow ochre",1.3},{"green earth",1.5},{"Prussian blue",0.22},{"lead white",0.9},{"raw umber",0.25},{"pale smalt",0.15}, medium=0.1}
mpass(z(486, 510, 3), G.far, 498, 1.5, 3001)
mpass(z(506, 540, 4), gP.lit, 522, 1.4, 3002)
mpass(z(536, 572, 5), mixP, 555, 1.4, 3003)
mpass(z(568, 600, 6), gP.sh, 585, 1.5, 3004)
mpass(z(595, 625, 6), gP.mid, 610, 1.2, 3005)
local nb = noise{seed=931, period=90, octaves=3, stretch={0, 3}}
local patchy = mask(function(x,y) return smoothstep(0.45, 0.62, nb:at01(x,y)) end)
mpass(z(500, 600, 6) * patchy, gP.lit, 550, 0.7, 3006)
print(wait(0))

--@ chunk 113
print(wait(60*48)); print(drying(100,500), drying(15,480), drying(200,500), drying(262,480), drying(15,400), drying(300,540))

--@ chunk 114
-- darker, cooler trunks: a semi-opaque dark glaze on the shadow side and a mid tone on the body
local tShade = pile{{"raw umber",1},{"bone black",0.5},{"green earth",0.8},{"lead white",0.3},{"Prussian blue",0.08}, medium=0.08}
local tMid = pile{{"lead white",0.9},{"raw umber",0.8},{"green earth",0.7},{"pale smalt",0.4},{"bone black",0.2},{"yellow ochre",0.2}, medium=0.08}
local moss = pile{{"green earth",1.5},{"yellow ochre",0.8},{"lead white",0.6},{"raw umber",0.3}, medium=0.08}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local m = PWTR[i] - PWCR[i]:shrink(H*0.02)
  local wbr = math.max(1.2, H*0.011)
  local body = m * mask(function(xx,yy) local cx = x0 + t.lean*(y0-yy); return smoothstep(cx - 0.035*H, cx - 0.015*H, xx) end)
  work(body, {hand="detail", pile=tMid, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, length={3,12}, coverage=2.5, clip=true, fill=true, load=0.9})
  local sh = m * mask(function(xx,yy) local cx = x0 + t.lean*(y0-yy); return smoothstep(cx + 0.01*H, cx + 0.04*H, xx) end)
  work(sh, {hand="detail", pile=tShade, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, length={3,12}, coverage=3, clip=true, fill=true, load=0.9})
  -- shadow under the crown on the head
  local hd = m * mask(function(xx,yy) return smoothstep(y0 - 0.37*H, y0 - 0.43*H, yy) end)
  work(hd, {hand="detail", pile=tShade, tool={kind="round", width=wbr, point=0.4}, angle=0, length={3,10}, coverage=2.5, clip=true, fill=true, load=0.8})
  -- moss on the north (shadow) side lower down
  local ms = m * mask(function(xx,yy) local cx = x0 + t.lean*(y0-yy); return smoothstep(cx + 0.02*H, cx + 0.05*H, xx) * smoothstep(y0 - 0.25*H, y0 - 0.1*H, yy) end)
  stipple(ms, {pile=moss, width=math.max(1, H*0.012), coverage=1.2, pressure={0.4,0.7}, cluster=0.6, feather=0.5})
  -- vertical bark fissures
  local r = brush{kind="rigger", width=math.max(0.5, H*0.004), point=1}
  r:load(tShade, 0.6)
  for k=1, math.floor(4 + H*0.04) do
    local u = rand(-0.05, 0.04)
    local v0 = rand(0.02, 0.2); local v1 = v0 + rand(0.06, 0.18)
    local p0 = {x0 + u*H + t.lean*v0*H, y0 - v0*H}
    local p1 = {x0 + (u + rand(-0.008,0.008))*H + t.lean*v1*H, y0 - v1*H}
    r:stroke({p0, p1}, {pressure={0.5,0.1}})
    if r:fullness() < 0.3 then r:reload(tShade, 0.6) end
  end
end
print(wait(0))

--@ chunk 115
local R = LAND * rect(0, 484, 445, 30) - PWALL:grow(0.5) - FIGS:grow(1) - PATH:shrink(2)
local p = pile{{"lead white",1.2},{"green earth",1.8},{"yellow ochre",1.1},{"pale smalt",0.4},{"Prussian blue",0.06},{"raw umber",0.2}, medium=0.1}
local m = R * mask(function(x,y) return smoothstep(484, 488, y) * (1 - smoothstep(506, 514, y)) end)
work(m, {hand="hatch", pile=p, tool={kind="round", width=1.3, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={2,5}, coverage=1.8, pressure={0.35,0.75}, ramps={0.1,0.6}, threshold=0.3, seed=3101})
print(wait(60*20))
print(drying(15,480), drying(262,480), drying(100,440))

--@ chunk 116
print(wait(60*40)); print(drying(15,480), drying(262,470), drying(330,460), drying(100,420))

--@ chunk 117
local lD = pile{{"green earth",1.6},{"raw umber",0.7},{"Prussian blue",0.3},{"yellow ochre",0.45},{"lead white",0.35}, medium=0.07}
local lM = pile{{"lead white",1.0},{"green earth",1.8},{"yellow ochre",0.7},{"pale smalt",0.5},{"raw umber",0.3},{"Prussian blue",0.05}, medium=0.07}
local lS = pile{{"lead white",2.6},{"green earth",1.3},{"pale smalt",0.6},{"yellow ochre",0.55}, medium=0.07}
PWCR2 = {}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  -- a skirt of foliage hanging over the head of the trunk
  local skirt = (ellipse(hx, hy - 0.01*H, 0.14*H, 0.05*H) + ellipse(hx - 0.13*H, hy - 0.04*H, 0.08*H, 0.05*H) + ellipse(hx + 0.13*H, hy - 0.04*H, 0.08*H, 0.05*H)):roughen(math.max(1, H*0.01), math.max(4, H*0.03), 1600+i)
  local cr = PWCR[i] + skirt
  PWCR2[i] = cr
  local wid = math.max(1.3, H*0.02)
  -- dark underside and inner shadow, clustered
  local low = cr * mask(function(xx,yy) local v = (hy - yy)/(0.55*H); local u = (xx - hx)/(0.5*H); return 1 - smoothstep(0.15, 0.7, v - 0.3*u) end)
  stipple(low, {pile=lD, width=wid, coverage=function(xx,yy) return 2.4*low:at(xx,yy) end, pressure={0.55,0.9}, cluster={0.6, H*0.04}, dips={10,0.8,0.4}})
  -- mid clusters over the whole, sparser at top
  stipple(cr, {pile=lM, width=wid, coverage=function(xx,yy) local v = (hy - yy)/(0.55*H); return 0.9 * (0.4 + 0.6*smoothstep(0.1, 0.6, v)) end, pressure={0.5,0.85}, cluster={0.8, H*0.035}, feather=0.5, dips={8,0.7,0.4}})
  -- sparing silver on top-left clumps
  local lit = cr * mask(function(xx,yy) local u = (xx - hx)/(0.5*H); local v = (hy - yy)/(0.55*H); return smoothstep(0.5, 1.05, v - 0.5*u) end)
  stipple(lit, {pile=lS, width=wid*0.8, coverage=function(xx,yy) return 1.2*lit:at(xx,yy) end, pressure={0.45,0.8}, cluster={0.85, H*0.025}, feather=0.7, dips={6,0.6,0.4}})
end
print(wait(0))

--@ chunk 118
local lit = pile{{"lead white",4},{"pale smalt",0.9},{"yellow ochre",0.15},{"raw umber",0.1}, medium=0.08}
local roofD = pile{{"lead white",1.8},{"smalt",1.4},{"bone black",0.35},{"red earth",0.1}, medium=0.08}
local r = brush{kind="round", width=1.0, point=0.6}
local det = function(m, p, w) work(m, {hand="detail", pile=p, tool={kind="round", width=w or 0.9, point=0.5}, angle=math.pi/2, length={2,6}, coverage=2.5, clip=true, fill=true, edge="found", load=0.7}) end
-- lit west faces of the towers
det(poly{{593,468},{593,433},{596.5,433},{596.5,468}}, lit)
det(poly{{674,468},{674,427},{677.5,427},{677.5,468}}, lit)
det(poly{{676.2,425},{676.6,414},{678.5,414},{678.5,425}}, lit, 0.7)
det(poly{{755,468},{755,439},{758,439},{758,468}}, lit)
det(poly{{755,438},{759.5,404},{758.5,438}}, lit, 0.7)
-- lit gable end of the naves
det(poly{{607,468},{607,452},{611,445},{611,468}}, lit)
det(poly{{686,468},{686,454},{690.5,447},{690.5,468}}, lit)
-- dark roof bands on the naves
det(poly{{612,445},{664,445},{668,451},{611,451}}, roofD)
det(poly{{691,447},{748,447},{751,452},{690,452}}, roofD)
-- windows: tiny dark slits in the towers (belfry openings)
local dk = pile{{"smalt",1},{"bone black",0.6},{"lead white",0.6}, medium=0.08}
r:load(dk, 0.4)
for _,p in ipairs({{599,437},{601.5,437},{679,430},{682,430},{679,441},{682,441},{759,443},{761.5,443},{680,418},{682,418}}) do
  r:stroke({{p[1],p[2]},{p[1],p[2]+3}}, {pressure={0.35,0.35}})
end
-- nave windows as tall slits
for x = 620, 660, 8 do r:stroke({{x,455},{x,462}}, {pressure={0.3,0.3}}) end
for x = 698, 744, 8 do r:stroke({{x,457},{x,464}}, {pressure={0.3,0.3}}) end
-- a sail barge on the river, far right
local sail = pile{{"red earth",1},{"raw umber",0.6},{"lead white",1.6},{"pale smalt",0.5}, medium=0.08}
local hull = pile{{"bone black",0.6},{"raw umber",0.8},{"lead white",1.2},{"pale smalt",0.4}, medium=0.08}
local sm = poly{{934,484},{934,466},{944,482}}
det(sm, sail, 0.8)
det(poly{{929,485.5},{947,485.5},{945,488},{931,488}}, hull, 0.8)
local mb = brush{kind="rigger", width=0.6, point=1}
mb:load(hull, 0.5)
mb:stroke({{933.4,486},{933.4,463}}, {pressure={0.5,0.3}})
print(wait(0))

--@ chunk 119
print(wait(60*30)); print(drying(50,495), drying(150,500), drying(300,495), drying(400,505))

--@ chunk 120
local R = LAND * rect(0, 481, 440, 34) - PWALL:grow(0.3) - FIGS:grow(1) - PATH:shrink(2)
local m = R * mask(function(x,y) return smoothstep(481, 485, y) * (1 - smoothstep(508, 514, y)) end)
local p = pile{{"lead white",1.1},{"green earth",1.9},{"yellow ochre",1.2},{"pale smalt",0.35},{"Prussian blue",0.06},{"raw umber",0.2}, medium=0.1}
work(m, {hand="body", pile=p, tool="filbert 4", clip=R, angle=0, length={8,20}, coverage=2.6, fill=true, load=0.85, threshold=0.3})
blend(m, {angle=0, coverage=1.5, tool={kind="badger", width=12}, clip=true, pressure={0.3,0.5}})
local p2 = pile{{"lead white",0.8},{"green earth",1.7},{"yellow ochre",1.4},{"chrome yellow",0.15},{"Prussian blue",0.1},{"raw umber",0.2}, medium=0.1}
work(m, {hand="hatch", pile=p2, tool={kind="round", width=1.1, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={2,5}, coverage=0.9, pressure={0.35,0.7}, ramps={0.1,0.6}, threshold=0.3, seed=3201})
print(wait(0))

--@ chunk 121
local lD = pile{{"green earth",1.6},{"raw umber",0.7},{"Prussian blue",0.3},{"yellow ochre",0.45},{"lead white",0.4}, medium=0.07}
local all = PWCR2[1]
for i=2,#PWCR2 do all = all + PWCR2[i] end
PWCRALL = all
local band = all:shrink(1.5) * rect(0, 440, 450, 30)
stipple(band, {pile=lD, width=2.0, coverage=2.5, pressure={0.55,0.85}, cluster=0.4, dips={10,0.8,0.4}})
print(wait(0))

--@ chunk 122
local lM = pile{{"lead white",0.9},{"green earth",1.9},{"yellow ochre",0.75},{"pale smalt",0.45},{"raw umber",0.3},{"Prussian blue",0.06}, medium=0.07}
local lS = pile{{"lead white",2.2},{"green earth",1.4},{"pale smalt",0.5},{"yellow ochre",0.7},{"chrome yellow",0.05}, medium=0.07}
local lD = pile{{"green earth",1.6},{"raw umber",0.7},{"Prussian blue",0.3},{"yellow ochre",0.45},{"lead white",0.4}, medium=0.07}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  local cr = PWCR2[i]:shrink(math.max(0.8, H*0.006))
  local wid = math.max(1.3, H*0.018)
  local up = cr * mask(function(xx,yy) local v = (hy - yy)/(0.55*H); return smoothstep(0.35, 0.7, v) end)
  stipple(up, {pile=lM, width=wid, coverage=function(xx,yy) return 1.8*up:at(xx,yy) end, pressure={0.55,0.9}, cluster={0.6, H*0.03}, dips={10,0.8,0.4}})
  -- shadowed pockets between clumps
  local nz = worley{seed=1700+i, period=H*0.12}
  local pockets = up * mask(function(xx,yy) local c = nz:at(xx,yy); return 1 end)
  stipple(up, {pile=lD, width=wid*0.9, coverage=function(xx,yy) return 0.5*up:at(xx,yy) end, pressure={0.5,0.8}, cluster={0.85, H*0.02}, dips={6,0.7,0.4}})
  local lit = cr * mask(function(xx,yy) local u = (xx - hx)/(0.5*H); local v = (hy - yy)/(0.55*H); return smoothstep(0.55, 1.1, v - 0.5*u) end)
  stipple(lit, {pile=lS, width=wid*0.8, coverage=function(xx,yy) return 1.0*lit:at(xx,yy) end, pressure={0.45,0.8}, cluster={0.85, H*0.022}, feather=0.7, dips={6,0.6,0.4}})
end
print(wait(0))

--@ chunk 123
print(wait(60*48)); print(drying(100,420), drying(262,420), drying(200,500), drying(15,400))

--@ chunk 124
local shootD = pile{{"green earth",1.4},{"raw umber",0.6},{"Prussian blue",0.15},{"yellow ochre",0.5},{"lead white",0.6}, medium=0.07}
local shootL = pile{{"lead white",2.2},{"green earth",1.3},{"pale smalt",0.4},{"yellow ochre",0.7}, medium=0.07}
local leafT = pile{{"lead white",1.4},{"green earth",1.6},{"yellow ochre",0.7},{"pale smalt",0.4},{"raw umber",0.15}, medium=0.07}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  local cr = PWCR2[i]
  local rb = brush{kind="rigger", width=math.max(0.5, H*0.005), point=1}
  local sb = brush{kind="round", width=math.max(1.0, H*0.01), point=0.6}
  rb:load(shootD, 0.6); sb:load(leafT, 0.5)
  local n = math.floor(8 + H*0.05)
  for k=1,n do
    local ang = -math.pi/2 + randn(0, 0.35)
    local rr = 0.2*H
    -- start inside the crown near its upper edge along this direction
    local sx, sy = hx, hy
    local dist = 0
    for d = 0, 0.9*H, 1.5 do
      local px, py = hx + math.cos(ang)*d, hy + math.sin(ang)*d
      if cr:at(px, py) > 0.5 then dist = d end
    end
    local d0 = dist - rand(0.02, 0.06)*H
    sx, sy = hx + math.cos(ang)*d0, hy + math.sin(ang)*d0
    local L = rand(0.06, 0.16)*H
    local ex, ey = sx + math.cos(ang)*L, sy + math.sin(ang)*L
    local mx, my = (sx+ex)/2 + rand(-0.01,0.01)*H, (sy+ey)/2
    rb:stroke({{sx,sy},{mx,my},{ex,ey}}, {pressure={0.55,0.0}})
    if rb:fullness() < 0.3 then rb:reload(shootD, 0.6) end
    -- leaves along the shoot: small dragged touches
    local p = (math.cos(ang) < -0.05) and shootL or leafT
    sb:reload(p, 0.4)
    for j=1,4 do
      local f = 0.3 + 0.18*j
      local px, py = sx + (ex-sx)*f, sy + (ey-sy)*f
      local side = (j % 2 == 0) and 1 or -1
      sb:touch(px + side*H*0.008, py, {pressure=0.35, drag={H*0.01, ang + side*0.8}})
    end
  end
end
print(wait(0))

--@ chunk 125
local lD = pile{{"green earth",1.6},{"raw umber",0.75},{"Prussian blue",0.3},{"yellow ochre",0.45},{"lead white",0.3}, medium=0.07}
local band = PWCRALL:shrink(2) * rect(0, 462, 450, 12)
stipple(band, {pile=lD, width=1.8, coverage=3.5, pressure={0.6,0.9}, cluster=0.3, dips={8,0.9,0.3}})
-- seam at the lit meadow edge on the left: grass blades crossing it
local seam = LAND * rect(0, 505, 440, 18) - PWALL:grow(0.5) - FIGS:grow(1) - PATH:shrink(2)
local gA = pile{{"yellow ochre",1.4},{"green earth",1.3},{"chrome yellow",0.25},{"lead white",1.0},{"Prussian blue",0.1}, medium=0.1}
local gB = pile{{"lead white",1.0},{"green earth",1.9},{"yellow ochre",1.1},{"pale smalt",0.35},{"Prussian blue",0.06},{"raw umber",0.2}, medium=0.1}
work(seam * mask(function(x,y) return smoothstep(508,516,y) end), {hand="hatch", pile=gA, tool={kind="round", width=1.3, point=0.8}, angle=-math.pi/2, angle_jitter=0.3, length={4,9}, coverage=1.4, pressure={0.35,0.75}, ramps={0.1,0.6}, threshold=0.3, seed=3301})
work(seam * mask(function(x,y) return 1 - smoothstep(508,516,y) end), {hand="hatch", pile=gB, tool={kind="round", width=1.1, point=0.8}, angle=-math.pi/2, angle_jitter=0.3, length={3,6}, coverage=1.0, pressure={0.35,0.7}, ramps={0.1,0.6}, threshold=0.3, seed=3302})
print(wait(0))

--@ chunk 126
local lD = pile{{"green earth",1.6},{"raw umber",0.75},{"Prussian blue",0.3},{"yellow ochre",0.45},{"lead white",0.3}, medium=0.07}
local band = PWCRALL:shrink(1.5) * mask(function(x,y) return math.abs(y - hzc(x)) < 3.5 and 1 or 0 end)
work(band, {hand="detail", pile=lD, tool={kind="round", width=1.6, point=0.4}, angle=0, length={4,10}, coverage=3, clip=true, fill=true, load=0.8})
stipple(band:grow(1.5) * PWCRALL:shrink(1), {pile=lD, width=1.8, coverage=1.5, pressure={0.5,0.8}, cluster=0.4})
print(wait(0))

--@ chunk 127
local bp = pile{{"raw umber",1},{"bone black",0.5},{"lead white",0.7},{"pale smalt",0.4}, medium=0.08}
local r = brush{kind="rigger", width=0.9, point=1}
r:load(bp, 0.5)
local function bird(x, y, s, tilt)
  -- two slightly curved wings meeting at the body
  local a = tilt or 0
  local function R(u,v) return {x + (u*math.cos(a) - v*math.sin(a))*s, y + (u*math.sin(a) + v*math.cos(a))*s} end
  r:stroke({R(-4,-1.2), R(-2,-1.4), R(0,0)}, {pressure={0.1,0.6}})
  r:stroke({R(0,0), R(2,-1.6), R(4.5,-1.0)}, {pressure={0.6,0.1}})
  if r:fullness() < 0.3 then r:reload(bp, 0.5) end
end
bird(612, 300, 1.3, 0.1)
bird(640, 288, 1.1, -0.15)
bird(598, 318, 0.9, 0.2)
bird(830, 180, 1.0, 0.05)
bird(360, 250, 0.8, -0.1)
-- a lark high up, tiny
bird(470, 160, 0.6, 0)
print(wait(0))

--@ chunk 128
local edgeD = pile{{"raw umber",0.9},{"green earth",0.6},{"yellow ochre",0.6},{"lead white",0.9},{"pale smalt",0.3}, medium=0.1}
local pebL = pile{{"lead white",3},{"yellow ochre",0.4},{"raw umber",0.2}, medium=0.08}
local pebD = pile{{"raw umber",1},{"lead white",1},{"pale smalt",0.3},{"bone black",0.1}, medium=0.08}
-- shaded edges of the path: a thin darker band inside each edge
local inner = (PATH - PATH:shrink(3)) * mask(function(x,y) return y > 565 and 1 or 0 end)
work(inner, {hand="hatch", pile=edgeD, tool={kind="round", width=1.3, point=0.6}, angle=0.2, angle_jitter=0.4, length={2,6}, coverage=0.9, clip=true, threshold=0.3, seed=3401})
-- the worn centre strip of grass between two tracks, fading into distance
local strip = {}
for i=1,#PC-3 do strip[#strip+1] = PC[i] end
local ws = {}
for i=1,#strip do ws[i] = lerp(9, 0.8, ((i-1)/(#strip-1))^0.7) end
local cs = ribbon(strip, ws) * PATH:shrink(2) * mask(function(x,y) return y > 575 and 1 or 0 end)
local gC = pile{{"yellow ochre",1.3},{"green earth",1.3},{"lead white",1.2},{"Prussian blue",0.08},{"raw umber",0.2}, medium=0.1}
work(cs, {hand="hatch", pile=gC, tool={kind="round", width=1.2, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={2,6}, coverage=0.9, pressure={0.35,0.7}, ramps={0.1,0.6}, threshold=0.4, seed=3402})
-- pebbles
local b = brush{kind="round", width=1.6, point=0.3}
for k=1,70 do
  local y = rand(575, 704)
  local x = rand(420, 620)
  if PATH:shrink(2):at(x,y) > 0.5 then
    local s = 0.25 + (y-570)/250
    b:reload(pebD, 0.4)
    b:touch(x, y + 0.6*s, {pressure=clamp(0.3*s+0.15, 0.15, 0.6)})
    b:reload(pebL, 0.4)
    b:touch(x - 0.4*s, y, {pressure=clamp(0.25*s+0.12, 0.12, 0.5)})
  end
end
-- the man's walking stick
local st = brush{kind="rigger", width=0.9, point=0.8}
st:load(pile{{"raw umber",1},{"bone black",0.3},{"lead white",0.3}, medium=0.08}, 0.6)
st:stroke({{456.5,517},{458.5,540},{460.2,561}}, {pressure={0.6,0.45}})
print(wait(0))

--@ chunk 129
local sh = pile{{"raw umber",0.8},{"green earth",0.8},{"pale smalt",0.6},{"lead white",0.6},{"Prussian blue",0.1}, medium=0.1}
local m = (poly({{438,561},{452,561},{500,566},{510,569},{470,568},{440,564}}, true) + poly({{458,562},{483,562},{528,567},{536,570},{495,570},{462,566}}, true)) - FIGS
work(m, {hand="detail", pile=sh, tool={kind="round", width=1.2, point=0.5}, angle=0.08, length={3,10}, coverage=1.8, clip=true, fill=true, load=0.5})
-- the walking stick again, more pressure
local st = brush{kind="round", width=1.2, point=0.8}
st:load(pile{{"raw umber",1},{"bone black",0.5},{"lead white",0.2}, medium=0.08}, 0.7)
st:stroke({{457.5,520},{459,540},{460.5,561.5}}, {pressure={0.7,0.6}})
print(wait(0))

--@ chunk 130
print(wait(60*50)); print(drying(520,650), drying(500,600), drying(480,566))

--@ chunk 131
local g = pile{{"raw umber",0.7},{"green earth",0.6},{"pale smalt",0.4},{"lead white",0.25}, medium=0.6}
local m = PATH:grow(1) * mask(function(x,y) return smoothstep(590, 690, y) end)
work(m, {hand="detail", pile=g, tool={kind="filbert", width=5}, angle=-1.0, length={10,25}, coverage=1.2, clip=true, load=0.3, threshold=0.2})
blend(PATH:grow(1) * mask(function(x,y) return y > 580 and 1 or 0 end), {angle=-1.0, coverage=1.3, tool={kind="badger", width=10}, pressure={0.25,0.4}})
print(wait(0))

--@ chunk 132
print(drying(200,680), drying(800,690), drying(520,690), drying(300,650))

--@ chunk 133
local g = pile{{"green earth",1},{"Prussian blue",0.25},{"raw umber",0.6},{"yellow ochre",0.3}, medium=0.6}
local nz = noise{seed=1901, period=150, octaves=2}
local m = LAND * mask(function(x,y) local yy = y + 10*nz(x, y); return smoothstep(650, 690, yy) end) - PATH:grow(1)
work(m, {hand="detail", pile=g, tool={kind="filbert", width=6}, angle=-math.pi/2, angle_jitter=0.3, length={10,25}, coverage=1.1, clip=true, load=0.3, threshold=0.2})
print(wait(0))

--@ chunk 134
print(wait(60*40)); print(drying(200,690), drying(800,690), drying(520,690))

--@ chunk 135
print(wait(60*24)); print(drying(200,690), drying(100,680), drying(900,690))

--@ chunk 136
local stemP = pile{{"green earth",1.3},{"yellow ochre",0.8},{"raw umber",0.4},{"lead white",0.6},{"Prussian blue",0.1}, medium=0.08}
local flowerP = pile{{"lead white",6},{"yellow ochre",0.1},{"green earth",0.1}, medium=0.05}
local flowerS = pile{{"lead white",3},{"green earth",0.4},{"pale smalt",0.4},{"raw umber",0.1}, medium=0.05}
local litBlade = pile{{"yellow ochre",1.4},{"chrome yellow",0.3},{"green earth",0.9},{"lead white",1.2},{"Prussian blue",0.06}, medium=0.08}
local rs = brush{kind="rigger", width=1.3, point=1}
local ft = brush{kind="round", width=1.8, point=0.4}
local function umbel(x, y, h, lean)
  local tx, ty = x + lean*h, y - h
  rs:reload(stemP, 0.7)
  rs:stroke({{x, y}, {x + lean*h*0.4, y - h*0.5}, {tx, ty}}, {pressure={0.7,0.35}})
  -- rays and florets
  local R = h*0.14
  local rb = brush{kind="rigger", width=0.7, point=1}
  rb:load(stemP, 0.5)
  local n = math.random(7, 10)
  local heads = {}
  for k=1,n do
    local a = -math.pi/2 + (k - (n+1)/2) * (2.4/n) + rand(-0.08,0.08)
    local ex, ey = tx + math.cos(a)*R, ty + math.sin(a)*R*0.55 - R*0.15
    rb:stroke({{tx,ty},{ex,ey}}, {pressure={0.4,0.2}})
    heads[#heads+1] = {ex, ey}
  end
  for _,hd in ipairs(heads) do
    ft:reload(flowerS, 0.4)
    ft:touch(hd[1] + 0.5, hd[2] + 0.6, {pressure=0.35})
    ft:reload(flowerP, 0.5)
    for j=1,4 do ft:touch(hd[1] + rand(-1.6,1.6), hd[2] + rand(-1.2,0.4), {pressure=rand(0.2,0.35)}) end
  end
end
umbel(62, 704, 120, 0.06)
umbel(92, 706, 96, -0.05)
umbel(35, 708, 80, 0.12)
umbel(905, 706, 110, -0.07)
umbel(948, 708, 132, 0.03)
umbel(870, 710, 70, 0.1)
umbel(640, 706, 58, -0.04)
-- lit blades catching sun over the dark bottom
local lb = brush{kind="rigger", width=1.2, point=1}
lb:load(litBlade, 0.6)
for k=1,90 do
  local x, y = rand(0,1000), rand(660, 712)
  if PATH:at(x, math.min(y,703)) < 0.3 then
    local h = rand(14, 40)
    local b = rand(-0.35, 0.35)
    lb:stroke({{x, y}, {x + b*h*0.4, y - h*0.55}, {x + b*h, y - h}}, {pressure={0.55,0.0}})
    if lb:fullness() < 0.3 then lb:reload(litBlade, 0.6) end
  end
end
print(wait(0))

--@ chunk 137
local flowerP = pile{{"lead white",6},{"yellow ochre",0.12},{"green earth",0.1}, medium=0.05}
local flowerS = pile{{"lead white",2.5},{"green earth",0.6},{"pale smalt",0.4},{"raw umber",0.15}, medium=0.05}
local U = {{62,704,120,0.06},{92,706,96,-0.05},{35,708,80,0.12},{905,706,110,-0.07},{948,708,132,0.03},{870,710,70,0.1},{640,706,58,-0.04}}
for i,u in ipairs(U) do
  local x, y, h, lean = u[1], u[2], u[3], u[4]
  local tx, ty = x + lean*h, y - h
  local R = h*0.14
  -- the umbel is a flattish dome of small clusters
  local m = nil
  for k=1,9 do
    local a = -math.pi/2 + (k-5)*0.27
    local cx, cy = tx + math.cos(a)*R, ty + math.sin(a)*R*0.55 - R*0.15
    local e = ellipse(cx, cy, R*0.28, R*0.2)
    m = m and (m + e) or e
  end
  stipple(m, {pile=flowerS, width=1.5, coverage=1.5, pressure={0.4,0.7}, cluster=0.5})
  local top = m * mask(function(xx,yy) return yy < ty - R*0.35 and 1 or 0.4 end)
  stipple(top, {pile=flowerP, width=1.4, coverage=function(xx,yy) return 2.2*top:at(xx,yy) end, pressure={0.4,0.7}, cluster=0.5})
end
print(wait(0))

--@ chunk 138
local flowerP = pile{{"lead white",6},{"yellow ochre",0.12},{"green earth",0.1}, medium=0.05}
local flowerS = pile{{"lead white",2.2},{"green earth",0.7},{"pale smalt",0.4},{"raw umber",0.2}, medium=0.05}
local stemD = pile{{"green earth",1.2},{"raw umber",0.9},{"Prussian blue",0.2},{"yellow ochre",0.4},{"lead white",0.3}, medium=0.08}
local stemL = pile{{"yellow ochre",1},{"green earth",1},{"lead white",1.3}, medium=0.08}
local U = {{62,704,120,0.06},{92,706,96,-0.05},{35,708,80,0.12},{905,706,110,-0.07},{948,708,132,0.03},{870,710,70,0.1},{640,706,58,-0.04}}
local sb = brush{kind="round", width=1.8, point=0.7}
local lb = brush{kind="rigger", width=0.8, point=1}
local rb = brush{kind="rigger", width=0.7, point=1}
for i,u in ipairs(U) do
  local x, y, h, lean = u[1], u[2], u[3], u[4]
  local tx, ty = x + lean*h, y - h
  local R = h*0.14
  sb:reload(stemD, 0.7)
  sb:stroke({{x, y}, {x + lean*h*0.4, y - h*0.5}, {tx, ty + R*0.1}}, {pressure={0.8,0.45}})
  lb:reload(stemL, 0.5)
  lb:stroke({{x - 0.5, y - h*0.2}, {x + lean*h*0.4 - 0.5, y - h*0.5}, {tx - 0.4, ty + R*0.2}}, {pressure={0.4,0.2}})
  -- rays
  rb:reload(stemD, 0.5)
  for k=1,7 do
    local a = -math.pi/2 + (k-4)*0.36
    rb:stroke({{tx, ty + R*0.1}, {tx + math.cos(a)*R*0.9, ty + math.sin(a)*R*0.45 - R*0.2}}, {pressure={0.45,0.2}})
  end
  -- side leaf, divided
  if h > 70 then
    local lx, ly = x + lean*h*0.25, y - h*0.3
    rb:reload(stemD, 0.6)
    local side = (i % 2 == 0) and 1 or -1
    for j=0,3 do
      rb:stroke({{lx + side*j*3, ly - j*2}, {lx + side*(j*3 + 5), ly - j*2 - 4}}, {pressure={0.8,0.1}})
    end
  end
  local dome = (ellipse(tx, ty - R*0.28, R*1.05, R*0.42) * above(function(xx) return ty - R*0.05 end)):roughen(1.2, 4, 1800+i)
  stipple(dome, {pile=flowerS, width=1.5, coverage=1.8, pressure={0.4,0.7}, cluster=0.5})
  local top = dome * mask(function(xx,yy) return smoothstep(ty - R*0.1, ty - R*0.5, yy) end)
  stipple(top, {pile=flowerP, width=1.4, coverage=function(xx,yy) return 2.5*top:at(xx,yy) end, pressure={0.4,0.7}, cluster=0.5})
end
print(wait(0))

--@ chunk 139
local stemD = pile{{"green earth",1},{"raw umber",1},{"Prussian blue",0.25},{"bone black",0.1},{"lead white",0.15}, medium=0.06}
local stemL = pile{{"yellow ochre",1.2},{"green earth",0.8},{"lead white",1.6}, medium=0.06}
local U = {{62,704,120,0.06},{92,706,96,-0.05},{35,708,80,0.12},{905,706,110,-0.07},{948,708,132,0.03},{870,710,70,0.1},{640,706,58,-0.04}}
local sb = brush{kind="round", width=2.4, point=0.6}
local lb = brush{kind="round", width=1.2, point=0.8}
for i,u in ipairs(U) do
  local x, y, h, lean = u[1], u[2], u[3], u[4]
  local tx, ty = x + lean*h, y - h
  local R = h*0.14
  sb:reload(stemD, 0.8)
  sb:stroke({{x, y}, {x + lean*h*0.4, y - h*0.5}, {tx, ty - R*0.05}}, {pressure={0.9,0.6}, ramps={0.02,0.1}})
  lb:reload(stemL, 0.6)
  lb:stroke({{x - 0.8, y - h*0.35}, {x + lean*h*0.4 - 0.8, y - h*0.55}, {tx - 0.7, ty + R*0.25}}, {pressure={0.55,0.3}})
end
print(wait(0))

--@ chunk 140
local wood = pile{{"raw umber",1},{"lead white",1.1},{"bone black",0.2},{"pale smalt",0.3},{"yellow ochre",0.3}, medium=0.08}
local m = (RAILS + FENCE) * (HORSES2:grow(2) + ellipse(776, 503.5, 24, 4) + ellipse(834, 502, 21, 4))
work(m, {hand="detail", pile=wood, tool={kind="round", width=0.9, point=0.4}, angle=-0.3, length={3,8}, coverage=3.5, clip=true, fill=true, edge="found", load=0.9})
-- break the shadow platforms with grass blades
local shm = (ellipse(776, 503.5, 22, 2.5) + ellipse(834, 502, 19, 2.5)) - HORSES2 - RAILS - FENCE
local gA = pile{{"lead white",1.2},{"green earth",1.7},{"yellow ochre",1.2},{"pale smalt",0.3},{"Prussian blue",0.05}, medium=0.1}
local gB = pile{{"green earth",1.5},{"yellow ochre",0.9},{"raw umber",0.4},{"lead white",0.6},{"Prussian blue",0.1}, medium=0.1}
work(shm, {hand="hatch", pile=gB, tool={kind="round", width=1.0, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={2,4}, coverage=2.0, pressure={0.35,0.7}, ramps={0.1,0.6}, threshold=0.3, seed=3501})
local around = (ellipse(776, 503, 30, 4) + ellipse(834, 501.5, 26, 4)) - HORSES2 - RAILS - FENCE
work(around, {hand="hatch", pile=gA, tool={kind="round", width=1.0, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={2,4}, coverage=1.0, pressure={0.35,0.7}, ramps={0.1,0.6}, threshold=0.3, seed=3502})
print(wait(0))

--@ chunk 141
print(wait(0)); print(PATH ~= nil, TRUNKS ~= nil, FIGS ~= nil, PWALL ~= nil, LAND ~= nil); print(drying(520,650), drying(300,500), drying(40,500))

--@ chunk 142
for _,yy in ipairs{600,620,640,660,670,680,685,690,695,700,703} do local l,r; for xx=380,700 do if PATH:at(xx,yy)>0.5 then l=l or xx; r=xx end end; print(yy,l,r) end

--@ chunk 143

local ext = mask(function(x,y) if y < 668 then return 0 end; local rx = 588 + (y-670)*1.15; return (x > 500 and x < rx) and 1 or 0 end):roughen(1.5, 8, 7001) - PATH:shrink(3)
PATHX = (PATH + ext):roughen(0.8, 6, 7002)
work(ext:grow(1), {hand="body", pile=pathD, tool="filbert 5", angle=-1.0, length={8,18}, coverage=2.0, fill=true, clip=true, edge="found"})
local lit = ext * mask(function(x,y) return smoothstep(0.45,0.65, noise{seed=19,period=24}:at01(x,y*1.6)) end)
work(lit, {hand="scumble", pile=pathS, tool="filbert 4", coverage=1.0, clip=ext:grow(1), load=0.35})
print(wait(0))

--@ chunk 144
print(wait(60*30)); print(drying(600,695), drying(503,695))

--@ chunk 145

local ext = (PATHX - PATH:shrink(4)) * mask(function(x,y) return (y > 668 and x > 520) and 1 or 0 end)
local pS = pile{{"lead white",1.2},{"raw umber",0.9},{"yellow ochre",0.7},{"pale smalt",0.5},{"green earth",0.5}, medium=0.1}
local pM = pile{{"lead white",2.2},{"raw umber",0.7},{"yellow ochre",0.9},{"pale smalt",0.3},{"green earth",0.2}, medium=0.1}
work(ext, {hand="hatch", pile=pS, tool={kind="round", width=2.2, point=0.5}, angle=0.1, angle_jitter=0.3, length={4,12}, coverage=1.8, clip=true, threshold=0.4, load=0.6})
work(ext, {hand="hatch", pile=pM, tool={kind="round", width=1.8, point=0.5}, angle=0.1, angle_jitter=0.3, length={3,9}, coverage=0.6, clip=true, threshold=0.4, load=0.5})
print(wait(0))

--@ chunk 146
print(wait(60*24)); print(drying(600,695), drying(560,690))

--@ chunk 147
print(wait(60*12)); print(drying(600,695), drying(590,690))

--@ chunk 148

local g = pile{{"raw umber",0.7},{"green earth",0.7},{"pale smalt",0.4},{"lead white",0.2}, medium=0.65}
local m = PATHX:grow(1) * mask(function(x,y) return smoothstep(575, 700, y) end)
work(m, {hand="detail", pile=g, tool={kind="filbert", width=5}, angle=-1.0, length={10,25}, coverage=1.3, clip=true, load=0.3, threshold=0.15})
local m2 = (PATHX - PATH:shrink(2)) * mask(function(x,y) return y > 672 and 1 or 0 end)
work(m2, {hand="detail", pile=g, tool={kind="filbert", width=4}, angle=-1.0, length={8,18}, coverage=1.0, clip=true, load=0.35})
print(wait(0))

--@ chunk 149
local gD = pile{{"green earth",1.4},{"Prussian blue",0.3},{"raw umber",0.6},{"yellow ochre",0.4},{"lead white",0.3}, medium=0.08}
local gM = pile{{"green earth",1.4},{"yellow ochre",1.1},{"Prussian blue",0.15},{"raw umber",0.3},{"lead white",0.7}, medium=0.08}
local gL = pile{{"yellow ochre",1.2},{"chrome yellow",0.35},{"green earth",0.8},{"lead white",1.2}, medium=0.08}
local rb = brush{kind="rigger", width=1.3, point=1}
local function edgeAt(yy, side)
  if side < 0 then for xx = 400, 700 do if PATHX:at(xx, yy) > 0.5 then return xx end end
  else for xx = 700, 400, -1 do if PATHX:at(xx, yy) > 0.5 then return xx end end end
end
local n = 0
local yy = 572
while yy < 706 do
  local sc = 0.35 + (yy - 560)/110
  for side = -1, 1, 2 do
    if math.random() < 0.8 then
      local ey = math.min(yy, 703)
      local ex = edgeAt(ey, side)
      if ex then
        -- a tuft: base a little outside the edge, blades leaning in over the path
        local bx = ex - side*rand(-3, 2)*sc
        local nb = math.random(3, 7)
        local p = (yy > 640) and gD or ((math.random() < 0.5) and gM or gD)
        rb:reload(p, 0.55)
        for k = 1, nb do
          local h = rand(6, 16) * sc
          local lean = -side * rand(0.05, 0.6) + randn(0, 0.15)
          local x0 = bx + rand(-2, 2)*sc
          local y0 = yy + rand(0, 3)
          local curl = rand(0.1, 0.4) * lean
          rb:stroke({{x0, y0}, {x0 + lean*h*0.4, y0 - h*0.5}, {x0 + lean*h*0.8 + curl*h, y0 - h}}, {pressure={0.55, 0.0}})
          n = n + 1
        end
        if math.random() < 0.45 then
          rb:reload(gL, 0.5)
          for k = 1, math.random(1, 3) do
            local h = rand(5, 13) * sc
            local lean = -side * rand(0.1, 0.5)
            local x0 = bx + rand(-2, 2)*sc
            rb:stroke({{x0, yy+1}, {x0 + lean*h*0.4, yy - h*0.5}, {x0 + lean*h*0.9, yy - h}}, {pressure={0.45, 0.0}})
            n = n + 1
          end
        end
      end
    end
  end
  yy = yy + rand(3, 7) * (0.5 + (yy-560)/140)
end
-- a few low clumps inside the path, on the worn middle
for k = 1, 9 do
  local y0 = rand(585, 700)
  local l, r = edgeAt(y0, -1), edgeAt(y0, 1)
  if l and r then
    local x0 = lerp(l, r, rand(0.35, 0.65))
    local sc = 0.35 + (y0 - 560)/110
    rb:reload((math.random() < 0.5) and gM or gD, 0.45)
    for j = 1, math.random(3, 6) do
      local h = rand(3, 8) * sc
      local lean = randn(0, 0.4)
      local xx = x0 + rand(-3, 3)*sc
      rb:stroke({{xx, y0}, {xx + lean*h*0.5, y0 - h*0.5}, {xx + lean*h, y0 - h}}, {pressure={0.45, 0.0}})
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 150
local gD = pile{{"green earth",1.4},{"Prussian blue",0.3},{"raw umber",0.6},{"yellow ochre",0.4},{"lead white",0.3}, medium=0.08}
local gM = pile{{"green earth",1.4},{"yellow ochre",1.1},{"Prussian blue",0.15},{"raw umber",0.3},{"lead white",0.7}, medium=0.08}
local nz = noise{seed=7301, period=18, octaves=3}
local band = mask(function(x,y)
  if y < 566 then return 0 end
  local sc = 0.35 + (y-560)/110
  local w = sc * (3 + 9 * math.max(0, nz(x,y) + 0.25))
  return w end)
-- inside distance from the path edge
local d = PATHX:distance()
local enc = PATHX * mask(function(x,y) local sc = 0.35 + (y-560)/110; local w = sc*(2 + 10*math.max(0, nz(x,y)+0.2)); return (d:at(x,y) < w and y > 568) and 1 or 0 end)
enc = enc:roughen(1.2, 5, 7302)
local up = enc * mask(function(x,y) return y < 628 and 1 or 0 end)
local lo = enc * mask(function(x,y) return y >= 628 and 1 or 0 end)
work(up, {hand="hatch", pile=gM, tool={kind="round", width=1.3, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={4,9}, coverage=1.6, pressure={0.35,0.7}, ramps={0.1,0.7}, load=0.6})
work(lo, {hand="hatch", pile=gD, tool={kind="round", width=1.8, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={6,14}, coverage=1.6, pressure={0.35,0.7}, ramps={0.1,0.7}, load=0.6})
print(enc:area(), wait(0))

--@ chunk 151

local gS = pile{{"green earth",1.4},{"Prussian blue",0.2},{"raw umber",0.4},{"yellow ochre",0.5},{"lead white",0.8}, medium=0.08}
local gM = pile{{"green earth",1.4},{"yellow ochre",1.1},{"Prussian blue",0.15},{"raw umber",0.3},{"lead white",0.7}, medium=0.08}
local m = rect(474, 558, 70, 16) - PATHX:shrink(1) - FIGS:grow(0.5)
work(m, {hand="hatch", pile=gS, tool={kind="round", width=1.0, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={3,7}, coverage=1.3, pressure={0.35,0.7}, ramps={0.1,0.7}, load=0.6, clip=false})
work(m, {hand="hatch", pile=gM, tool={kind="round", width=0.9, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={3,6}, coverage=0.5, pressure={0.35,0.7}, ramps={0.1,0.7}, load=0.5, clip=false})
-- the blob at the bottom left of the path
local gD = pile{{"green earth",1.4},{"Prussian blue",0.3},{"raw umber",0.6},{"yellow ochre",0.4},{"lead white",0.3}, medium=0.08}
local b = rect(492, 676, 20, 28) - PATH:shrink(1)
work(b, {hand="hatch", pile=gD, tool={kind="round", width=1.8, point=0.8}, angle=-math.pi/2, angle_jitter=0.3, length={8,16}, coverage=2.0, pressure={0.4,0.8}, ramps={0.1,0.7}, load=0.7})
print(wait(0))

--@ chunk 152
print(#PWTR, #PWCR, drying(20,500), drying(100,480), drying(60,420)); for i,t in ipairs(PW) do print(i, t.x + t.lean*0.43*t.H, t.y-0.43*t.H) end

--@ chunk 153
local glz = pile{{"raw umber",1},{"green earth",0.7},{"bone black",0.15},{"pale smalt",0.2}, medium=0.6}
local glzL = pile{{"raw umber",0.6},{"green earth",0.4},{"yellow ochre",0.2}, medium=0.75}
local fis = pile{{"raw umber",1},{"bone black",0.5},{"green earth",0.4}, medium=0.1}
local ridge = pile{{"lead white",2.2},{"raw umber",0.5},{"yellow ochre",0.5},{"pale smalt",0.3},{"green earth",0.2}, medium=0.08}
local moss = pile{{"green earth",1.5},{"yellow ochre",0.8},{"raw umber",0.3},{"lead white",0.3}, medium=0.1}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local m = PWTR[i]
  local function cx(yy) return x0 + t.lean*(y0-yy) end
  -- an overall light glaze to lower the value and warm it toward grey-olive
  work(m, {hand="detail", pile=glzL, tool={kind="filbert", width=math.max(2, H*0.03)}, angle=math.pi/2, length={6,20}, coverage=1.2, clip=true, load=0.35})
  -- deeper glaze on the shadow side (right)
  local sh = m * mask(function(xx,yy) return smoothstep(cx(yy) - 0.03*H, cx(yy) + 0.05*H, xx) end)
  work(sh, {hand="detail", pile=glz, tool={kind="filbert", width=math.max(2, H*0.025)}, angle=math.pi/2, length={6,18}, coverage=1.4, clip=true, load=0.4, threshold=0.2})
  -- fissures: wavering vertical dark lines
  local rb = brush{kind="rigger", width=math.max(0.5, H*0.005), point=1}
  rb:load(fis, 0.6)
  local nf = math.floor(6 + H*0.06)
  for k=1,nf do
    local u = rand(-0.07, 0.07)
    local v0 = rand(0.0, 0.3); local v1 = v0 + rand(0.05, 0.16)
    local pts = {}
    for j=0,4 do
      local v = lerp(v0, v1, j/4)
      local yy = y0 - v*H
      pts[#pts+1] = {cx(yy) + u*H*(1 + 0.3*math.sin(v*20)) + randn(0, 0.004*H), yy}
    end
    local inside = true
    for _,p in ipairs(pts) do if m:at(p[1], p[2]) < 0.5 then inside = false end end
    if inside then rb:stroke(pts, {pressure={0.2, 0.6}, ramps={0.3,0.3}, swell={0.6,1.2,0.7}}) end
    if rb:fullness() < 0.3 then rb:reload(fis, 0.6) end
  end
  -- lit ridges on the left
  rb:reload(ridge, 0.5)
  for k=1,math.floor(4 + H*0.03) do
    local u = rand(-0.075, -0.02)
    local v0 = rand(0.02, 0.3); local v1 = v0 + rand(0.04, 0.1)
    local pts = {}
    for j=0,3 do local v = lerp(v0, v1, j/3); local yy = y0 - v*H; pts[#pts+1] = {cx(yy) + u*H + randn(0, 0.003*H), yy} end
    local inside = true
    for _,p in ipairs(pts) do if m:at(p[1], p[2]) < 0.5 then inside = false end end
    if inside then rb:stroke(pts, {pressure={0.15, 0.4}, ramps={0.3,0.4}}) end
  end
  -- moss at the foot and up the shaded side
  local mm = m * mask(function(xx,yy) local v = (y0-yy)/H; return smoothstep(0.12, 0.02, v) * smoothstep(cx(yy) - 0.02*H, cx(yy) + 0.06*H, xx) end)
  stipple(mm, {pile=moss, width=math.max(1, H*0.012), coverage=0.8, pressure={0.4,0.7}, cluster=0.6, clip=m})
end
print(wait(0))

--@ chunk 154
print(wait(60*30)); print(drying(20,500), drying(105,500), drying(262,490))

--@ chunk 155
print(wait(60*24)); print(drying(20,500), drying(105,500), drying(262,490))

--@ chunk 156
local mid = pile{{"lead white",1},{"raw umber",0.9},{"green earth",0.45},{"pale smalt",0.3},{"bone black",0.12}, medium=0.08}
local dk  = pile{{"raw umber",1},{"bone black",0.3},{"green earth",0.5},{"lead white",0.35},{"pale smalt",0.15}, medium=0.08}
local lit = pile{{"lead white",2.4},{"raw umber",0.5},{"pale smalt",0.45},{"yellow ochre",0.25},{"green earth",0.2}, medium=0.08}
local fis = pile{{"raw umber",1},{"bone black",0.6},{"green earth",0.3}, medium=0.1}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local m = PWTR[i]
  local function cx(yy) return x0 + t.lean*(y0-yy) end
  local wbr = math.max(1.2, H*0.012)
  work(m, {hand="detail", pile=mid, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, angle_jitter=0.15, length={3,12}, coverage=2.2, clip=true, fill=true})
  local sh = m * mask(function(xx,yy) return smoothstep(cx(yy) + 0.0*H, cx(yy) + 0.05*H, xx) end)
  work(sh, {hand="detail", pile=dk, tool={kind="round", width=wbr, point=0.4}, angle=math.pi/2, angle_jitter=0.15, length={3,12}, coverage=1.8, clip=true, threshold=0.3})
  local lm = m * mask(function(xx,yy) return smoothstep(cx(yy) - 0.035*H, cx(yy) - 0.065*H, xx) end)
  work(lm, {hand="detail", pile=lit, tool={kind="round", width=wbr*0.8, point=0.4}, angle=math.pi/2, angle_jitter=0.15, length={3,10}, coverage=1.3, clip=true, threshold=0.3})
  local rb = brush{kind="rigger", width=math.max(0.5, H*0.005), point=1}
  rb:load(fis, 0.6)
  for k=1,math.floor(8 + H*0.08) do
    local u = rand(-0.06, 0.07)
    local v0 = rand(0.0, 0.36); local v1 = v0 + rand(0.05, 0.14)
    local pts = {}
    for j=0,4 do
      local v = lerp(v0, v1, j/4); local yy = y0 - v*H
      pts[#pts+1] = {cx(yy) + u*H + 0.006*H*math.sin(v*35 + k) + randn(0, 0.002*H), yy}
    end
    local inside = true
    for _,p in ipairs(pts) do if m:at(p[1], p[2]) < 0.6 then inside = false end end
    if inside then rb:stroke(pts, {pressure={0.2, 0.55}, ramps={0.3,0.3}, swell={0.6,1.2,0.7}}) end
    if rb:fullness() < 0.3 then rb:reload(fis, 0.6) end
  end
end
print(wait(0))

--@ chunk 157
print(wait(60*48)); print(drying(20,500), drying(105,500), drying(262,490), drying(100,450))

--@ chunk 158
print(wait(60*30)); print(drying(20,500), drying(105,500), drying(262,490), drying(380,480))

--@ chunk 159
local skyC  = pile{{"lead white",5},{"yellow ochre",0.22},{"pale smalt",0.15},{"raw umber",0.05}, medium=0.08}
local riseC = pile{{"lead white",1.8},{"green earth",1.9},{"yellow ochre",0.7},{"pale smalt",0.75},{"raw umber",0.2}, medium=0.1}
PWGAP = {}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  local nz = noise{seed=7400+i, period=math.max(6, H*0.06), octaves=2}
  local halfw = 0.34*H
  local curve = function(x)
    local u = (x - hx)/halfw
    if math.abs(u) >= 1 then return hy + 0.08*H end
    return hy + 0.03*H - 0.14*H*(1 - u*u) + 0.025*H*nz(x, hy)
  end
  local G = PWCRALL * below(curve) * rect(hx - halfw, hy - 0.2*H, 2*halfw, 0.3*H) - PWTR[i]:shrink(0.5) - MILL:grow(1.5)
  G = G:roughen(math.max(0.8, H*0.006), math.max(3, H*0.02), 7410+i)
  PWGAP[i] = G
  local sk = G - LAND
  local ld = G * LAND
  work(sk, {hand="detail", pile=skyC, tool={kind="round", width=math.max(1.2,H*0.012), point=0.3}, angle=0, length={3,10}, coverage=3, clip=true, fill=true, load=0.8})
  work(ld, {hand="detail", pile=riseC, tool={kind="round", width=math.max(1.2,H*0.012), point=0.3}, angle=0, length={3,10}, coverage=3, clip=true, fill=true, load=0.8})
end
print(wait(0))

--@ chunk 160
local skyC  = pile{{"lead white",5},{"yellow ochre",0.22},{"pale smalt",0.15},{"raw umber",0.05}, medium=0.08}
local riseC = pile{{"lead white",1.8},{"green earth",1.9},{"yellow ochre",0.7},{"pale smalt",0.75},{"raw umber",0.2}, medium=0.1}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  local halfw = 0.34*H
  local R = (PWGAP[i]:grow(2.5) * PWCRALL:grow(2.5)) - PWGAP[i]:shrink(1) - PWTR[i]:shrink(0.5) - MILL:grow(1.5)
  R = R * mask(function(x,y) return y > hy - 0.1*H and 1 or 0 end)
  -- keep only the rims below the crown: pixels whose upper neighbour 4 units up is gap or sky
  local sk = R - LAND
  local ld = R * LAND
  work(sk, {hand="detail", pile=skyC, tool={kind="round", width=1.0, point=0.3}, angle=0, length={3,8}, coverage=2.5, clip=true, fill=true, load=0.7})
  work(ld, {hand="detail", pile=riseC, tool={kind="round", width=1.0, point=0.3}, angle=0, length={3,8}, coverage=2.5, clip=true, fill=true, load=0.7})
end
print(wait(0))

--@ chunk 161
print(wait(60*36)); print(drying(60,420), drying(130,425), drying(262,445))

--@ chunk 162
local rodD = pile{{"raw umber",1},{"green earth",0.6},{"bone black",0.3},{"lead white",0.4}, medium=0.08}
local rodL = pile{{"lead white",1.6},{"raw umber",0.6},{"yellow ochre",0.4},{"green earth",0.3},{"pale smalt",0.2}, medium=0.08}
local knob = pile{{"raw umber",1},{"green earth",0.5},{"bone black",0.2},{"lead white",0.6},{"pale smalt",0.15}, medium=0.08}
local lD = pile{{"green earth",1.6},{"raw umber",0.7},{"Prussian blue",0.3},{"yellow ochre",0.45},{"lead white",0.4}, medium=0.07}
local lM = pile{{"lead white",0.9},{"green earth",1.9},{"yellow ochre",0.75},{"pale smalt",0.45},{"raw umber",0.3},{"Prussian blue",0.06}, medium=0.07}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local hx, hy = x0 + t.lean*0.43*H, y0 - 0.43*H
  -- knobbly head: a few swollen bosses on the trunk top
  local kb = nil
  for k=1,5 do
    local e = ellipse(hx + rand(-0.075,0.075)*H, hy + rand(-0.005,0.02)*H, rand(0.02,0.035)*H, rand(0.015,0.025)*H)
    kb = kb and (kb + e) or e
  end
  kb = kb:roughen(math.max(0.6,H*0.004), math.max(3,H*0.015), 7500+i) - PWCRALL:shrink(1)
  work(kb, {hand="detail", pile=knob, tool={kind="round", width=math.max(0.9,H*0.008), point=0.4}, angle=0, length={2,6}, coverage=2.5, clip=true, fill=true, load=0.8})
  -- rods from the head up into the crown
  local rb = brush{kind="rigger", width=math.max(0.6, H*0.0065), point=1}
  rb:load(rodD, 0.6)
  local n = math.floor(14 + H*0.08)
  for k=1,n do
    local sx = hx + rand(-0.08, 0.08)*H
    local sy = hy + rand(-0.005, 0.015)*H
    local a = -math.pi/2 + (sx - hx)/(0.08*H)*0.55 + randn(0, 0.22)
    local L = H*rand(0.12, 0.22)
    local bend = rand(-0.15, 0.15)
    local pts = {}
    for j=0,4 do
      local f = j/4
      local aa = a + bend*f
      pts[#pts+1] = {sx + math.cos(aa)*L*f, sy + math.sin(aa)*L*f}
    end
    rb:stroke(pts, {pressure={0.65, 0.15}, ramps={0.05,0.5}})
    if rb:fullness() < 0.3 then rb:reload(rodD, 0.6) end
  end
  -- a few lit rods on the left
  rb:reload(rodL, 0.5)
  for k=1,math.floor(4 + H*0.02) do
    local sx = hx + rand(-0.08, -0.01)*H
    local sy = hy + rand(-0.005, 0.01)*H
    local a = -math.pi/2 + (sx - hx)/(0.08*H)*0.55 + randn(0, 0.2)
    local L = H*rand(0.1, 0.18)
    local pts = {}
    for j=0,3 do local f=j/3; pts[#pts+1] = {sx + math.cos(a)*L*f, sy + math.sin(a)*L*f} end
    rb:stroke(pts, {pressure={0.45, 0.1}, ramps={0.05,0.5}})
  end
  -- ragged leafy fringe hanging below the crown edge, over the rods' upper parts
  local fr = (PWCRALL:grow(0.05*H) - PWCRALL:shrink(0.01*H)) * mask(function(x,y) return (y > hy - 0.15*H and y < hy + 0.06*H and math.abs(x - hx) < 0.36*H) and 1 or 0 end)
  fr = fr * PWGAP[i]:grow(1) + fr * PWCRALL
  stipple(fr, {pile=lD, width=math.max(1.2,H*0.016), coverage=function(x,y) return 0.9 end, pressure={0.5,0.85}, cluster={0.85, H*0.03}, feather=0.6, dips={10,0.7,0.4}})
  stipple(fr, {pile=lM, width=math.max(1.1,H*0.013), coverage=0.35, pressure={0.45,0.8}, cluster={0.85, H*0.025}, feather=0.6, dips={8,0.6,0.4}})
end
print(wait(0))

--@ chunk 163
print(wait(60*30)); print(drying(40,455), drying(150,457), drying(262,457), drying(330,460))

--@ chunk 164
local riseC = pile{{"lead white",1.8},{"green earth",1.9},{"yellow ochre",0.7},{"pale smalt",0.75},{"raw umber",0.2}, medium=0.1}
local skyC  = pile{{"lead white",5},{"yellow ochre",0.22},{"pale smalt",0.15},{"raw umber",0.05}, medium=0.08}
local gaps = PWGAP[1]; for i=2,#PWGAP do gaps = gaps + PWGAP[i] end
CROWNNOW = PWCRALL - gaps
local cols = nil
for i,t in ipairs(PW) do
  local H = t.H; local hx, hy = t.x + t.lean*0.43*H, t.y - 0.43*H
  local c = poly{{hx-0.1*H, hy+0.03*H},{hx-0.2*H, hy-0.25*H},{hx+0.2*H, hy-0.25*H},{hx+0.1*H, hy+0.03*H}}
  cols = cols and (cols + c) or c
end
local band = mask(function(x,y) return (y > 430 and y < 505 and x < 445) and 1 or 0 end)
local zone = (PWCRALL:grow(12) - CROWNNOW:grow(0.5)) * band - cols - PWALL:grow(0.5) - MILL:grow(1.5) - FIGS:grow(1)
local zl = zone * LAND
local zs = zone - LAND
work(zl, {hand="detail", pile=riseC, tool={kind="round", width=1.3, point=0.3}, angle=0, length={3,10}, coverage=2.5, clip=true, fill=true, load=0.8})
work(zs, {hand="detail", pile=skyC, tool={kind="round", width=1.3, point=0.3}, angle=0, length={3,10}, coverage=2.5, clip=true, fill=true, load=0.8})
print(zl:area(), zs:area(), wait(0))

--@ chunk 165
print(drying(20,500), drying(105,500), drying(262,490), drying(330,480))

--@ chunk 166
local g = pile{{"raw umber",1},{"green earth",0.8},{"pale smalt",0.15},{"bone black",0.05}, medium=0.7}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local function cx(yy) return x0 + t.lean*(y0-yy) end
  local m = PWTR[i]
  -- a thin glaze all over, heavier to the right and toward the foot
  local mm = m * mask(function(xx,yy) local v = (y0-yy)/H; return 0.45 + 0.55*smoothstep(cx(yy) - 0.04*H, cx(yy) + 0.05*H, xx) end)
  work(mm, {hand="detail", pile=g, tool={kind="filbert", width=math.max(2, H*0.025)}, angle=math.pi/2, length={6,20}, coverage=1.3, clip=true, load=0.35, threshold=0.3})
end
print(wait(0))

--@ chunk 167
print(drying(20,500)); for i,t in ipairs(PW) do blend(PWTR[i]:shrink(0.5), {angle=math.pi/2, coverage=2.0, tool={kind="badger", width=math.max(4, t.H*0.05)}, clip=true, pressure={0.3,0.5}}) end; print(wait(0))

--@ chunk 168
print(wait(60*48)); print(drying(20,500), drying(105,500), drying(262,490), drying(330,480))

--@ chunk 169
local ridge = pile{{"lead white",2.4},{"raw umber",0.45},{"pale smalt",0.35},{"yellow ochre",0.3},{"green earth",0.15}, medium=0.08}
local fis = pile{{"raw umber",1},{"bone black",0.55},{"green earth",0.3}, medium=0.1}
local tuft = pile{{"green earth",1.3},{"yellow ochre",1.1},{"Prussian blue",0.22},{"raw umber",0.4},{"lead white",0.5}, medium=0.08}
local tuftL = pile{{"yellow ochre",1.4},{"chrome yellow",0.4},{"green earth",0.8},{"lead white",1.2},{"Prussian blue",0.08}, medium=0.08}
for i,t in ipairs(PW) do
  local H, x0, y0 = t.H, t.x, t.y
  local function cx(yy) return x0 + t.lean*(y0-yy) end
  local m = PWTR[i]
  local rb = brush{kind="rigger", width=math.max(0.5, H*0.006), point=1}
  rb:load(ridge, 0.5)
  for k=1,math.floor(5 + H*0.04) do
    local u = rand(-0.075, -0.03)
    local v0 = rand(0.02, 0.33); local v1 = v0 + rand(0.04, 0.1)
    local pts = {}
    for j=0,3 do local v = lerp(v0, v1, j/3); local yy = y0 - v*H; pts[#pts+1] = {cx(yy) + u*H + 0.004*H*math.sin(v*30+k), yy} end
    local ok = true
    for _,p in ipairs(pts) do if m:at(p[1], p[2]) < 0.6 then ok = false end end
    if ok then rb:stroke(pts, {pressure={0.15, 0.45}, ramps={0.3,0.4}}) end
    if rb:fullness() < 0.3 then rb:reload(ridge, 0.5) end
  end
  rb:reload(fis, 0.6)
  for k=1,math.floor(4 + H*0.04) do
    local u = rand(-0.02, 0.07)
    local v0 = rand(0.0, 0.33); local v1 = v0 + rand(0.05, 0.12)
    local pts = {}
    for j=0,4 do local v = lerp(v0, v1, j/4); local yy = y0 - v*H; pts[#pts+1] = {cx(yy) + u*H + 0.005*H*math.sin(v*33+k), yy} end
    local ok = true
    for _,p in ipairs(pts) do if m:at(p[1], p[2]) < 0.6 then ok = false end end
    if ok then rb:stroke(pts, {pressure={0.2, 0.55}, ramps={0.3,0.3}, swell={0.6,1.2,0.7}}) end
    if rb:fullness() < 0.3 then rb:reload(fis, 0.6) end
  end
  -- grass tufts over the trunk's foot
  local s = H/200
  local g = brush{kind="rigger", width=math.max(0.6, 1.3*s), point=1}
  for pass=1,2 do
    g:reload(pass == 1 and tuft or tuftL, 0.6)
    for k=1, (pass == 1 and 30 or 12) do
      local x = x0 + rand(-0.12, 0.12)*H
      local yb = y0 + rand(-1, 3)*s
      local h = rand(4, 12)*s
      local lean = randn(0, 0.35)
      g:stroke({{x, yb}, {x + lean*h*0.5, yb - h*0.5}, {x + lean*h, yb - h}}, {pressure={0.55, 0.0}})
      if g:fullness() < 0.3 then g:reload(pass == 1 and tuft or tuftL, 0.6) end
    end
  end
end
print(wait(0))

--@ chunk 170
local lD = pile{{"green earth",1.6},{"raw umber",0.7},{"Prussian blue",0.3},{"yellow ochre",0.45},{"lead white",0.4}, medium=0.07}
local lM = pile{{"lead white",0.9},{"green earth",1.9},{"yellow ochre",0.75},{"pale smalt",0.45},{"raw umber",0.3},{"Prussian blue",0.06}, medium=0.07}
local nz = worley{seed=7700, period=9, jitter=1}
for i,t in ipairs(PW) do
  local H = t.H
  local hx, hy = t.x + t.lean*0.43*H, t.y - 0.43*H
  local under = (CROWNNOW:grow(0.035*H) - CROWNNOW) * PWGAP[i]
  -- keep only hanging lobes: cells chosen by the worley field
  local lobes = under * mask(function(x,y) local d1, d2, wall, id = nz:at(x, y); return (id < 0.45) and 1 or 0 end)
  lobes = lobes:roughen(1, 4, 7710+i) - PWTR[i]:grow(0.5)
  stipple(lobes, {pile=lD, width=math.max(1.1, H*0.014), coverage=1.4, pressure={0.5,0.85}, cluster={0.6, H*0.02}, dips={10,0.7,0.4}})
  stipple(lobes * mask(function(x,y) return x < hx and 1 or 0 end), {pile=lM, width=math.max(1.0, H*0.011), coverage=0.4, pressure={0.45,0.8}, cluster={0.7, H*0.02}, feather=0.6})
end
print(wait(0))

--@ chunk 171
print(drying(650,460), drying(880,466)); print(TOWN:at(650,460), TOWN:at(560,466), TOWN:at(820,466))

--@ chunk 172
local roofR = pile{{"lead white",2.6},{"red earth",0.35},{"smalt",0.6},{"raw umber",0.1}, medium=0.08}
local wallL = pile{{"lead white",4},{"pale smalt",0.7},{"yellow ochre",0.12}, medium=0.08}
local wallS = pile{{"lead white",2.2},{"smalt",0.9},{"bone black",0.12}, medium=0.08}
local treeH = pile{{"lead white",2.2},{"green earth",0.9},{"smalt",0.8},{"raw umber",0.1}, medium=0.08}
local det = function(m, p, w) work(m, {hand="detail", pile=p, tool={kind="round", width=w or 0.8, point=0.5}, angle=math.pi/2, length={2,5}, coverage=2.5, clip=true, fill=true, edge="found", load=0.7}) end
-- trees at the town's edges and between houses, hazy
local tr = nil
for _,c in ipairs({{540,462,9,6},{551,459,7,8},{529,464,6,4},{832,461,8,7},{845,463,9,5},{856,465,6,3},{664,463,5,4},{705,464,4,3}}) do
  local e = ellipse(c[1], c[2], c[3], c[4])
  tr = tr and (tr + e) or e
end
tr = (tr * above(function(x) return 468.5 end)):roughen(1.2, 3, 7801)
stipple(tr, {pile=treeH, width=1.4, coverage=3, pressure={0.5,0.8}, cluster=0.3, clip=tr})
-- low houses along the foot: each a wall and a pitched roof, gable ends to the west lit
local xs = {566, 576, 585, 602, 662, 672, 699, 712, 726, 776, 790, 803}
for k,x in ipairs(xs) do
  local w = rand(6, 10); local wh = rand(3, 5); local rh = rand(3, 5.5)
  local base = 468.5
  local wall = rect(x, base - wh, w, wh)
  local roof = poly{{x - 0.5, base - wh}, {x + 1.5, base - wh - rh}, {x + w - 1.5, base - wh - rh}, {x + w + 0.5, base - wh}}
  local gable = poly{{x, base - wh}, {x + 1.5, base - wh - rh}, {x + 3, base - wh}}
  det(roof - gable, roofR, 0.7)
  det(wall, (k % 3 == 0) and wallS or wallL, 0.7)
  det(gable, wallL, 0.6)
end
-- a far post mill on the horizon, right
local mc = pile{{"lead white",2.4},{"smalt",0.8},{"raw umber",0.2}, medium=0.08}
det(rect(884, 459, 4, 6), mc, 0.7)
det(poly{{885,465},{887,465},{888,468.5},{884,468.5}}, mc, 0.5)
local rb = brush{kind="rigger", width=0.45, point=1}
rb:load(mc, 0.5)
rb:stroke({{879,455},{893,469}}, {pressure={0.45,0.45}})
rb:stroke({{893,455},{879,469}}, {pressure={0.45,0.45}})
print(wait(0))

--@ chunk 173
print(wait(60*30)); print(drying(570,466), drying(540,462))

--@ chunk 174
print(wait(60*24)); print(drying(570,466), drying(670,466))

--@ chunk 175
print(wait(60*48)); print(drying(570,466), drying(670,466))

--@ chunk 176

local hz = pile{{"lead white",2.2},{"smalt",0.9},{"bone black",0.1},{"raw umber",0.05}, medium=0.55}
local m = rect(555, 452, 265, 17) * above(function(x) return 468.8 end)
work(m, {hand="detail", pile=hz, tool={kind="filbert", width=3}, angle=0, length={6,14}, coverage=1.4, clip=true, load=0.35})
local tz = pile{{"lead white",2.0},{"green earth",0.7},{"smalt",1.0},{"bone black",0.08}, medium=0.5}
local t = (rect(520, 450, 40, 19) + rect(820, 450, 45, 19)) * above(function(x) return 468.8 end)
work(t, {hand="detail", pile=tz, tool={kind="filbert", width=2.5}, angle=0, length={4,10}, coverage=1.0, clip=true, load=0.3, threshold=0.3})
print(wait(0))

--@ chunk 177
print(wait(60*48)); print(drying(570,462), drying(840,462))

--@ chunk 178
local band = rect(516, 446, 356, 24) * above(function(x) return 469.5 end)
local sk = band - TOWN:shrink(0.3) - LAND
local skyH = pile{{"lead white",12},{"yellow ochre",0.35},{"vermilion",0.05},{"pale smalt",0.3},{"raw umber",0.03}, medium=0.08}
work(sk, {hand="detail", pile=skyH, tool={kind="round", width=2.0, point=0.3}, angle=0, length={4,12}, coverage=3, clip=true, fill=true, load=0.85})
local tw = TOWN * rect(553, 446, 275, 24)
work(tw, {hand="detail", pile=townP3, tool={kind="round", width=1.3, point=0.5}, angle=math.pi/2, length={3,8}, coverage=3, fill=true, clip=true, edge="found", load=0.85})
print(wait(0))

--@ chunk 179
local far = pile{{"lead white",6},{"pale smalt",1.1},{"raw umber",0.25},{"red earth",0.04}, medium=0.08}
local nz = noise{seed=7901, period=25, octaves=3}
local top = function(x) return 465.8 - 1.2*smoothstep(560, 510, x) - 1.0*smoothstep(830, 880, x) + 0.9*nz(x, 0) end
local m = (below(top) * rect(505, 455, 380, 15) * above(function(x) return 469.6 end)) - TOWN:shrink(0.2) - LAND
work(m, {hand="detail", pile=far, tool={kind="round", width=1.2, point=0.4}, angle=0, length={4,12}, coverage=2.5, clip=true, fill=true, load=0.7})
print(wait(0))

--@ chunk 180
print(wait(60*72)); print(drying(600,460), drying(700,466), drying(540,466))

--@ chunk 181
print(wait(60*72)); print(drying(600,460), drying(700,466), drying(540,466))

--@ chunk 182
local lit = pile{{"lead white",4},{"pale smalt",0.9},{"smalt",0.25},{"yellow ochre",0.1},{"raw umber",0.08}, medium=0.08}
local litH = pile{{"lead white",3},{"pale smalt",0.9},{"smalt",0.5},{"yellow ochre",0.06},{"raw umber",0.1}, medium=0.08}
local roofD = pile{{"lead white",1.8},{"smalt",1.4},{"bone black",0.35},{"red earth",0.18}, medium=0.08}
local roofW = pile{{"lead white",2.0},{"smalt",1.0},{"red earth",0.3},{"bone black",0.15}, medium=0.08}
local det = function(m, p, w) work(m, {hand="detail", pile=p, tool={kind="round", width=w or 0.8, point=0.5}, angle=math.pi/2, length={2,5}, coverage=2.5, clip=true, fill=true, edge="found", load=0.7}) end
det(poly{{593,469},{593,446},{596.5,446},{596.5,469}}, lit)
det(poly{{674,469},{674,446},{677.5,446},{677.5,469}}, lit)
det(poly{{755,469},{755,446},{758,446},{758,469}}, lit)
det(poly{{607,469},{607,452},{611,445},{611,469}}, lit)
det(poly{{686,469},{686,454},{690.5,447},{690.5,469}}, lit)
det(poly{{612,445},{664,445},{668,451},{611,451}}, roofD)
det(poly{{691,447},{748,447},{751,452},{690,452}}, roofD)
local r = brush{kind="round", width=1.0, point=0.6}
local dk = pile{{"smalt",1},{"bone black",0.6},{"lead white",0.6}, medium=0.08}
r:load(dk, 0.4)
for x = 620, 660, 8 do r:stroke({{x,455},{x,462}}, {pressure={0.3,0.3}}) end
for x = 698, 744, 8 do r:stroke({{x,457},{x,464}}, {pressure={0.3,0.3}}) end
-- low houses in front, hazy: {x, width, wall h, roof h}
local H = {{558,8,3.5,3},{567,7,3,3.5},{575,9,4,3},{600,6,3,3},{614,10,3.5,3},{626,8,3,2.5},{648,9,3.5,3},{693,8,3,3},{704,10,4,3},{719,7,3,3},{733,9,3.5,3},{768,9,3.5,3.5},{782,8,3,3},{795,10,4,3},{808,8,3,3}}
for k,h in ipairs(H) do
  local x, w, wh, rh = h[1], h[2], h[3], h[4]
  local b = 469.6
  local wall = rect(x, b - wh, w, wh)
  local roof = poly{{x - 0.4, b - wh}, {x + 1.6, b - wh - rh}, {x + w - 1.6, b - wh - rh}, {x + w + 0.4, b - wh}}
  local gable = poly{{x - 0.2, b - wh + 0.2}, {x + 1.6, b - wh - rh}, {x + 3.2, b - wh + 0.2}}
  det(roof - gable, (k % 2 == 0) and roofW or roofD, 0.7)
  det(wall * rect(x, b - wh, 3.2, wh) + gable, (k % 3 == 0) and litH or lit, 0.6)
  det(wall - rect(x, b - wh, 3.2, wh), litH, 0.6)
  -- a dark door/window dot
  r:stroke({{x + rand(4, w-1.5), b - wh + 1}, {x + rand(4, w-1.5), b - wh + 1.8}}, {pressure={0.25,0.25}})
end
print(wait(0))

--@ chunk 183
print(wait(60*24*3))

--@ chunk 184
print(drying(820,485), drying(500,250), drying(450,520))

--@ chunk 185
-- sheen of sky light on the black horse's back, crest and rump; a warm lit line on the bay's back
local h2 = HORSES2 * rect(810, 460, 60, 50)
local h1 = HORSES2 * rect(740, 470, 70, 40)
local sheen = pile{{"bone black",0.35},{"raw umber",0.5},{"lead white",1.3},{"pale smalt",0.6},{"yellow ochre",0.1}, medium=0.06}
local warm = pile{{"red earth",0.8},{"yellow ochre",1.0},{"lead white",1.0},{"raw umber",0.2}, medium=0.06}
local rim2 = (h2 - h2:shrink(1.1)) * mask(function(x,y) return y < 480 and 1 or 0 end)
work(rim2, {hand="detail", pile=sheen, tool={kind="round", width=0.8, point=0.6}, angle=0, length={2,5}, coverage=1.4, clip=true, load=0.5})
local rim1 = (h1 - h1:shrink(1.1)) * mask(function(x,y) return y < 481 and 1 or 0 end)
work(rim1, {hand="detail", pile=warm, tool={kind="round", width=0.8, point=0.6}, angle=0, length={2,5}, coverage=1.2, clip=true, load=0.5})
print(wait(0))

--@ chunk 186
print(wait(60*30)); print(drying(835,475), drying(790,478))
local h2 = HORSES2 * rect(810, 460, 60, 50)
local blk = pile{{"bone black",1},{"raw umber",0.5},{"lead white",0.12},{"pale smalt",0.1}, medium=0.06}
-- take the outline back off the head, the throat and the front of the neck: keep sheen only on crest-back-rump
local front = (h2 - h2:shrink(1.6)) * mask(function(x,y) return (x < 827 and 1 or 0) end)
work(front, {hand="detail", pile=blk, tool={kind="round", width=0.8, point=0.6}, angle=0, length={2,5}, coverage=3, clip=true, fill=true, load=0.8})
-- the upper line of the crest and forehead gets a single thin cool glint
local r = brush{kind="round", width=0.7, point=0.8}
r:load(pile{{"bone black",0.3},{"raw umber",0.4},{"lead white",1.2},{"pale smalt",0.6}, medium=0.06}, 0.35)
print(wait(0))

--@ chunk 187
-- a white stork walking in the meadow between the first and second pollards, foraging to the left
local body = poly({{176,500},{178,497.2},{183,496.4},{190,497.5},{195,500.2},{197.5,503},{193,503.6},{186,505},{180,504.4},{176.5,502.5}}, true)
local neck = poly({{176.5,500.5},{175.2,496},{173.2,491.5},{172.6,489.2},{174.4,488.4},{175.8,490.6},{178,495},{180,498}}, true)
local head = ellipse(173.6, 489.1, 1.7, 1.35)
STORK_W = body + neck + head
local white = pile{{"lead white",4},{"yellow ochre",0.25},{"pale smalt",0.05}, medium=0.05}
local whiteS = pile{{"lead white",2.2},{"raw umber",0.35},{"pale smalt",0.5},{"yellow ochre",0.15}, medium=0.05}
work(STORK_W, {hand="detail", pile=white, tool={kind="round", width=0.9, point=0.6}, angle=0.1, length={2,5}, coverage=4, clip=true, fill=true, edge="found", load=0.9})
-- shade on the belly and under the neck
local shade = STORK_W * mask(function(x,y) return smoothstep(500.5, 504, y) end)
work(shade, {hand="detail", pile=whiteS, tool={kind="round", width=0.8, point=0.6}, angle=0.1, length={2,4}, coverage=1.4, clip=true, threshold=0.3})
print(wait(0))

--@ chunk 188
print(wait(60*28)); print(drying(186,500))
local blackP = pile{{"bone black",1},{"raw umber",0.4},{"lead white",0.1}, medium=0.05}
local red = pile{{"vermilion",1},{"red earth",0.5},{"lead white",0.15}, medium=0.05}
-- black flight feathers: the rear lower half of the folded wing, running out over the tail
local wing = poly({{183,501.2},{188,499.6},{193,500.2},{198.6,503.4},{196,504.2},{190,504.6},{184.5,504.2}}, true)
work(wing, {hand="detail", pile=blackP, tool={kind="round", width=0.7, point=0.6}, angle=0.15, length={2,4}, coverage=3.5, clip=true, fill=true, edge="found", load=0.8})
-- legs: long, red, one stepping back
local r = brush{kind="round", width=0.75, point=0.5}
r:load(red, 0.7)
r:stroke({{184.6,504.5},{184.2,510},{184.6,513.5},{183.6,521.5}}, {pressure={0.75,0.6}})
r:reload(red, 0.7)
r:stroke({{188.6,504.6},{189.6,510},{190.4,513},{192.8,520.6}}, {pressure={0.75,0.6}})
-- bill, pointing down to the grass
local b = brush{kind="round", width=0.8, point=0.9}
b:load(red, 0.6)
b:stroke({{172.4,489.4},{170,492.2},{167.6,495.6}}, {pressure={0.8,0.1}})
-- eye
local e = brush{kind="round", width=0.5, point=0.9}
e:load(blackP, 0.3)
e:touch(173.4, 488.8, {pressure=0.25})
print(wait(0))

--@ chunk 189
-- the stork's feet sunk in the grass; a faint shadow to the right
local gA = pile{{"lead white",1.2},{"green earth",1.5},{"yellow ochre",1.5},{"chrome yellow",0.2},{"pale smalt",0.2}, medium=0.1}
local gB = pile{{"green earth",1.5},{"yellow ochre",0.9},{"raw umber",0.4},{"lead white",0.6},{"Prussian blue",0.1}, medium=0.1}
local sh = ellipse(194, 521.5, 8, 1.3) - ellipse(186,521.5,3,2)
work(sh, {hand="hatch", pile=gB, tool={kind="round", width=0.9, point=0.8}, angle=-math.pi/2, angle_jitter=0.3, length={2,3.5}, coverage=1.6, pressure={0.35,0.6}, ramps={0.1,0.6}, seed=881})
local feet = ellipse(188, 520, 9, 2.8)
work(feet, {hand="hatch", pile=gA, tool={kind="round", width=0.9, point=0.8}, angle=-math.pi/2, angle_jitter=0.35, length={2,4.5}, coverage=1.2, pressure={0.35,0.7}, ramps={0.1,0.6}, seed=882})
print(wait(0))

--@ chunk 190
print(drying(820,470), drying(835,475), drying(450,500))

--@ chunk 191
print(mx, wx); print(W_DRESS ~= nil, MAN_COLLAR ~= nil)

--@ chunk 192
-- folds and a half-belt on the man's greatcoat; folds in the woman's dress and shawl
local mx, wx = 446, 470
local r = brush{kind="round", width=0.8, point=0.7}
local dark = pile{{"bone black",1},{"raw umber",0.6},{"Prussian blue",0.1},{"lead white",0.05}, medium=0.1}
local ridge = pile{{"bone black",0.5},{"raw umber",0.6},{"green earth",0.6},{"lead white",1.0},{"pale smalt",0.3}, medium=0.1}
-- back vent, from the half-belt to the hem
r:load(dark, 0.4)
r:stroke({{mx+0.3,510},{mx+0.1,522},{mx-0.4,534}}, {pressure={0.35,0.5}, clip=MAN_COAT})
-- half-belt across the small of the back
local belt = poly({{mx-7,505},{mx+7.5,505},{mx+7.5,507.2},{mx-7,507.2}}) * MAN_COAT
work(belt, {hand="detail", pile=dark, tool={kind="round", width=0.7, point=0.6}, angle=0, length={2,5}, coverage=2, clip=true, load=0.5})
-- lit ridges of the skirt folds, on the left (lit) side and a fainter one right
r:reload(ridge, 0.35)
r:stroke({{mx-4.5,508},{mx-6.5,520},{mx-8.5,533}}, {pressure={0.3,0.45}, clip=MAN_COAT})
r:reload(ridge, 0.25)
r:stroke({{mx+4,509},{mx+5.5,521},{mx+7,532}}, {pressure={0.2,0.35}, clip=MAN_COAT})
r:reload(ridge, 0.3)
r:stroke({{mx-6,480},{mx-7,490},{mx-6.5,500}}, {pressure={0.25,0.3}, clip=MAN_COAT})
-- two buttons on the belt
local e = brush{kind="round", width=0.6, point=0.9}
e:load(pile{{"lead white",1},{"raw umber",0.6},{"yellow ochre",0.4}, medium=0.05}, 0.3)
e:touch(mx-2.2, 506.1, {pressure=0.3}); e:touch(mx+2.6, 506.1, {pressure=0.3})
-- woman: shadow folds falling from the waist and spreading to the hem
local dshade = pile{{"red earth",0.8},{"raw umber",1},{"bone black",0.35},{"lead white",0.1}, medium=0.1}
local dlit = pile{{"red earth",1},{"raw umber",0.3},{"vermilion",0.2},{"lead white",0.8},{"yellow ochre",0.3}, medium=0.1}
local WD = poly({{wx-6.5,484},{wx-7.5,500},{wx-9.5,520},{wx-12,545},{wx-13.5,561},{wx-2,562.5},{wx+8,562},{wx+13.5,560},{wx+11.5,544},{wx+9,520},{wx+7.5,500},{wx+6.5,484}}, true)
r:reload(dshade, 0.4)
r:stroke({{wx-2.5,508},{wx-4,530},{wx-5.5,560}}, {pressure={0.25,0.6}, clip=WD})
r:reload(dshade, 0.4)
r:stroke({{wx+3,506},{wx+4.5,530},{wx+6.5,560}}, {pressure={0.25,0.6}, clip=WD})
r:reload(dshade, 0.3)
r:stroke({{wx+7,520},{wx+9.5,545},{wx+11,559}}, {pressure={0.2,0.5}, clip=WD})
r:reload(dlit, 0.35)
r:stroke({{wx-0.5,509},{wx-1,532},{wx-1.5,560}}, {pressure={0.2,0.45}, clip=WD})
r:reload(dlit, 0.35)
r:stroke({{wx-6,512},{wx-8.5,535},{wx-10,559}}, {pressure={0.2,0.45}, clip=WD})
-- shawl: two soft folds converging to its point
r:reload(shawlS, 0.3)
r:stroke({{wx+4.5,485},{wx+2.5,495},{wx+0.6,504}}, {pressure={0.3,0.2}, clip=W_SHAWL})
r:reload(shawlS, 0.2)
r:stroke({{wx-3,486},{wx-1.8,496},{wx-0.4,503}}, {pressure={0.2,0.15}, clip=W_SHAWL})
print(wait(0))

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
