-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=560, aspect=1.4, linen={18,15}, seed=7, ground={{pile={{"lead white",4},{"yellow ochre",0.3}}, um=110, apply="knife", texture=0.3},{pile={{"lead white",1},{"red earth",0.5},{"raw umber",0.5},{"yellow ochre",0.5}}, um=20, apply="brush"}}}
print(W,H)

--@ chunk 2
draw = pile{{"raw umber",2},{"bone black",0.3}, medium=0.55}
rb = brush{kind="round", width=3, point=0.7}
rb:load(draw, 0.6)
-- horizon line of far land
rb:stroke({{0,442},{200,440},{420,443},{620,441},{800,444},{1000,440}}, {pressure={0.4,0.3}})
rb:load(draw, 0.6)
-- left tree group silhouette
rb:stroke({{40,442},{55,380},{90,330},{130,300},{170,285},{220,270},{260,290},{300,300},{340,330},{380,370},{410,420},{420,442}}, {pressure={0.5,0.3}})
rb:load(draw, 0.6)
-- right, lower copse
rb:stroke({{760,442},{780,410},{820,395},{860,392},{900,405},{930,425},{950,442}}, {pressure={0.5,0.3}})
rb:load(draw, 0.6)
-- creek banks: from horizon ~ (600..660) to bottom (330..700)
rb:stroke({{605,445},{570,470},{520,500},{500,530},{540,570},{560,610},{480,660},{380,714}}, {pressure={0.4,0.5}})
rb:load(draw, 0.6)
rb:stroke({{660,445},{650,470},{610,500},{600,530},{650,570},{700,620},{680,670},{700,714}}, {pressure={0.4,0.5}})

--@ chunk 3
hz = function(x) return 441 + 2*math.sin(x/130) end
sky = above(hz)
s1 = pile{{"lead white",3},{"smalt",1},{"cobalt blue",0.6},{"raw umber",0.25}, medium=0.1}
s2 = pile{{"lead white",4},{"cobalt blue",0.5},{"smalt",0.4},{"red earth",0.08}, medium=0.1}
s3 = pile{{"lead white",5},{"vermilion",0.12},{"yellow ochre",0.25},{"cobalt blue",0.12}, medium=0.1}
s4 = pile{{"lead white",6},{"chrome yellow",0.6},{"vermilion",0.12}, medium=0.1}
local bands = {{s1,0,150},{s2,120,270},{s3,240,370},{s4,340,450}}
for i,b in ipairs(bands) do
  local m = rect(0,b[2],1000,b[3]-b[2]) * sky
  work(m, {hand="broad", pile=b[1], angle=function(x,y) return 0.05*math.sin(x/200+y/90) end, coverage=2.5, fill=true, tool="filbert 18"})
end

--@ chunk 4
blend(sky, {angle=0}); blend(sky, {angle=0.18})

--@ chunk 5
function cloudm(list, seed)
  local m = nil
  for i,e in ipairs(list) do
    local el = ellipse(e[1],e[2],e[3],e[4])
    m = m and (m + el) or el
  end
  return m:roughen(6, 40, seed):soften(3) * sky
end
cl_bank = cloudm({{520,170,120,14},{680,160,150,20},{850,150,170,26},{960,175,110,18},{760,185,120,10},{420,182,70,8}}, 11)
cl_mid  = cloudm({{120,280,140,10},{260,292,110,8},{60,300,80,7},{380,300,60,5}}, 12)
cl_low  = cloudm({{240,378,160,5},{470,388,120,4},{830,370,100,5},{120,395,70,3}}, 13)
cl_top  = cloudm({{120,60,180,22},{320,40,160,16},{40,90,120,14},{560,55,90,10}}, 14)
c1 = pile{{"lead white",3},{"smalt",0.6},{"red earth",0.3},{"raw umber",0.2},{"vermilion",0.05}, medium=0.15}
c2 = pile{{"lead white",3.5},{"red earth",0.3},{"cobalt blue",0.3},{"yellow ochre",0.15}, medium=0.15}
c0 = pile{{"lead white",2},{"smalt",0.7},{"raw umber",0.3},{"red earth",0.2}, medium=0.15}
local hor = function(x,y) return 0.03*math.sin(x/150) end
work(cl_top, {hand="body", pile=c0, angle=hor, coverage=2, length={40,120}, tool="filbert 10", load=0.6})
work(cl_bank, {hand="body", pile=c1, angle=hor, coverage=2.2, length={40,140}, tool="filbert 10", load=0.6})
work(cl_mid, {hand="body", pile=c1, angle=hor, coverage=2, length={30,110}, tool="filbert 7", load=0.55})
work(cl_low, {hand="body", pile=c2, angle=hor, coverage=1.8, length={30,100}, tool="filbert 5", load=0.5})

--@ chunk 6
blend(sky, {angle=0.02})

--@ chunk 7
local L = {{627,441},{612,452},{585,465},{548,482},{522,500},{515,520},{535,548},{560,580},{555,612},{510,645},{440,685},{370,720}}
local R = {{635,720},{700,714},{692,665},{702,625},{682,590},{632,550},{600,522},{592,500},{602,482},{622,466},{633,452},{634,441}}
local pts = {}
for _,p in ipairs(L) do pts[#pts+1]=p end
for _,p in ipairs(R) do pts[#pts+1]=p end
creek = poly(pts, true):roughen(1.5, 25, 21)
land = -sky
marsh = land - creek
m_far  = pile{{"raw umber",1},{"green earth",1},{"smalt",0.5},{"lead white",0.6},{"bone black",0.1}, medium=0.1}
m_mid  = pile{{"yellow ochre",1},{"raw umber",1},{"green earth",1},{"lead white",0.3}, medium=0.1}
m_warm = pile{{"yellow ochre",1.5},{"red earth",0.3},{"raw umber",0.6},{"lead white",0.4}, medium=0.1}
m_near = pile{{"raw umber",1.5},{"green earth",1},{"bone black",0.3},{"yellow ochre",0.5}, medium=0.1}
w_far  = pile{{"lead white",5},{"chrome yellow",0.4},{"vermilion",0.1},{"raw umber",0.12}, medium=0.1}
w_mid  = pile{{"lead white",4},{"yellow ochre",0.2},{"cobalt blue",0.25},{"raw umber",0.2}, medium=0.1}
w_near = pile{{"lead white",3},{"cobalt blue",0.4},{"smalt",0.5},{"raw umber",0.35}, medium=0.1}
local hor = function(x,y) return 0.04*math.sin(x/90+y/40) end
-- marsh
work(marsh * rect(0,436,1000,50), {hand="body", pile=m_far, angle=hor, coverage=2.2, length={30,90}, tool="filbert 8", fill=true, clip=true})
work(marsh * rect(0,480,1000,90), {hand="body", pile=m_mid, angle=hor, coverage=2.2, length={30,90}, tool="filbert 10", fill=true, clip=marsh})
work(marsh * rect(0,560,1000,160), {hand="body", pile=m_near, angle=hor, coverage=2.2, length={40,110}, tool="filbert 12", fill=true, clip=marsh})
-- water
work(creek * rect(0,436,1000,90), {hand="body", pile=w_far, angle=0, coverage=2.5, length={15,50}, tool="filbert 5", fill=true, clip=true})
work(creek * rect(0,520,1000,90), {hand="body", pile=w_mid, angle=0, coverage=2.5, length={30,80}, tool="filbert 8", fill=true, clip=true})
work(creek * rect(0,600,1000,120), {hand="body", pile=w_near, angle=0, coverage=2.5, length={30,100}, tool="filbert 10", fill=true, clip=true})

--@ chunk 8
local C = {{668,442},{610,445},{565,450},{572,457},{612,465},{602,478},{566,493},{548,511},{556,536},{586,566},{612,602},{606,642},{572,682},{540,725}}
-- catmull-rom
local pts, ws = {}, {}
for i=1,#C-1 do
  local p0 = C[math.max(1,i-1)]; local p1=C[i]; local p2=C[i+1]; local p3=C[math.min(#C,i+2)]
  for s=0,9 do
    local t=s/10; local t2=t*t; local t3=t2*t
    local function cr(a,b,c,d) return 0.5*((2*b)+(-a+c)*t+(2*a-5*b+4*c-d)*t2+(-a+3*b-3*c+d)*t3) end
    local x=cr(p0[1],p1[1],p2[1],p3[1]); local y=cr(p0[2],p1[2],p2[2],p3[2])
    pts[#pts+1]={x,y}; ws[#ws+1]= 1.5 + 0.85*math.max(0,y-441)
  end
end
creek_line = pts
creek2 = ribbon(pts, ws):roughen(1.2, 20, 22) * land
local extra_marsh = (creek - creek2) * land
local extra_water = creek2 - creek
local hor = function(x,y) return 0.04*math.sin(x/90+y/40) end
work(extra_marsh * rect(0,436,1000,50), {hand="body", pile=m_far, angle=hor, coverage=2.5, length={15,50}, tool="filbert 6", fill=true, clip=extra_marsh})
work(extra_marsh * rect(0,480,1000,85), {hand="body", pile=m_mid, angle=hor, coverage=2.5, length={20,60}, tool="filbert 8", fill=true, clip=extra_marsh})
work(extra_marsh * rect(0,560,1000,165), {hand="body", pile=m_near, angle=hor, coverage=2.5, length={30,80}, tool="filbert 10", fill=true, clip=extra_marsh})
work(extra_water * rect(0,436,1000,90), {hand="body", pile=w_far, angle=0, coverage=2.5, length={10,30}, tool="filbert 4", fill=true, clip=extra_water})
work(extra_water * rect(0,520,1000,200), {hand="body", pile=w_mid, angle=0, coverage=2.5, length={10,40}, tool="filbert 5", fill=true, clip=extra_water})
creek = creek2
marsh = land - creek

--@ chunk 9
local L = {{627,441},{612,452},{585,465},{548,482},{522,500},{515,520},{535,548},{560,580},{555,612},{510,645},{440,685},{370,720}}
local R = {{635,720},{700,714},{692,665},{702,625},{682,590},{632,550},{600,522},{592,500},{602,482},{622,466},{633,452},{634,441}}
local pts = {}
for _,p in ipairs(L) do pts[#pts+1]=p end
for _,p in ipairs(R) do pts[#pts+1]=p end
local old = poly(pts, true):roughen(1.5, 25, 21)
local ex = ((old:grow(3) - creek) * land)
local hor = function(x,y) return 0.04*math.sin(x/90+y/40) end
for pass=1,2 do
work(ex * rect(0,436,1000,50), {hand="body", pile=m_far, angle=hor, coverage=2.5, length={15,40}, tool="filbert 6", fill=true, clip=ex, dips={1,0.9,0.5}})
work(ex * rect(0,480,1000,85), {hand="body", pile=m_mid, angle=hor, coverage=2.5, length={15,50}, tool="filbert 8", fill=true, clip=ex, dips={1,0.9,0.5}})
work(ex * rect(0,560,1000,165), {hand="body", pile=m_near, angle=hor, coverage=2.5, length={20,60}, tool="filbert 10", fill=true, clip=ex, dips={1,0.9,0.5}})
end

--@ chunk 10
blend(creek:shrink(1), {angle=0}); blend(creek:shrink(1), {angle=0.1}); print(wait(48*60)); print(drying(300,200), drying(600,600), drying(200,600), drying(540,560))

--@ chunk 11
local r = ribbon({{500,300},{600,300}},20); print(r:at(550,305), r:at(550,309), r:at(550,311), r:at(550,315)); for _,y in ipairs({470,500,550,600,650,700}) do local l,rr=nil,nil; for x=380,760 do if creek:at(x,y)>0.5 then l=l or x; rr=x end end; print(y,l,rr) end

--@ chunk 12
print(wait(3*24*60)); for _,p in ipairs({{300,200},{600,600},{200,600},{540,560},{100,460},{300,520},{620,450}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 13
local C = {{668,442},{610,445},{565,450},{572,457},{612,465},{602,478},{566,493},{548,511},{556,536},{586,566},{612,602},{606,642},{572,682},{540,725}}
local pts, ws = {}, {}
for i=1,#C-1 do
  local p0 = C[math.max(1,i-1)]; local p1=C[i]; local p2=C[i+1]; local p3=C[math.min(#C,i+2)]
  for s=0,9 do
    local t=s/10; local t2=t*t; local t3=t2*t
    local function cr(a,b,c,d) return 0.5*((2*b)+(-a+c)*t+(2*a-5*b+4*c-d)*t2+(-a+3*b-3*c+d)*t3) end
    local x=cr(p0[1],p1[1],p2[1],p3[1]); local y=cr(p0[2],p1[2],p2[2],p3[2])
    pts[#pts+1]={x,y}; ws[#ws+1]= 1.0 + 0.5*math.max(0,y-441)
  end
end
creek_line = pts
creek = ribbon(pts, ws):roughen(1.0, 18, 23) * land
marsh = land - creek
for _,y in ipairs({470,500,550,600,650,700}) do local l,rr=nil,nil; for x=380,760 do if creek:at(x,y)>0.5 then l=l or x; rr=x end end; print(y,l,rr) end
g_far  = pile{{"green earth",1},{"raw umber",0.8},{"smalt",0.4},{"lead white",0.8},{"yellow ochre",0.3}, medium=0.1}
g_mid  = pile{{"yellow ochre",1.2},{"raw umber",0.8},{"green earth",0.8},{"lead white",0.35},{"red earth",0.12}, medium=0.1}
g_lmid = pile{{"raw umber",1},{"green earth",1},{"yellow ochre",0.7},{"bone black",0.1}, medium=0.1}
g_near = pile{{"raw umber",1.5},{"green earth",0.8},{"bone black",0.3},{"red earth",0.2}, medium=0.1}
local function band(a, b, s1, s2)
  return below(function(x) return a + 4*math.sin(x/70+s1) + 3*math.sin(x/23+s2) end) * above(function(x) return b + 5*math.sin(x/60+s2) + 3*math.sin(x/19+s1) end)
end
local hor = function(x,y) return 0.03*math.sin(x/90+y/40) end
local bands = {{g_far,430,472,"filbert 5",{15,45}},{g_mid,465,535,"filbert 8",{25,70}},{g_lmid,525,600,"filbert 10",{30,90}},{g_near,590,730,"filbert 14",{40,120}}}
for i,b in ipairs(bands) do
  local m = band(b[2], b[3], i, i*2) * marsh
  work(m, {hand="body", pile=b[1], angle=hor, coverage=2.6, length=b[5], tool=b[4], fill=true, clip=marsh, dips={2,0.85,0.3}})
end
blend(marsh, {angle=0})

--@ chunk 14
local function ells(list)
  local m=nil
  for _,e in ipairs(list) do local el=ellipse(e[1],e[2],e[3],e[4]); m = m and (m+el) or el end
  return m
end
-- far tree line: bumpy band along horizon
local n1 = noise{seed=31, period=40, octaves=3}
local n2 = noise{seed=32, period=12, octaves=2}
farline = below(function(x) return 438 - 5*math.max(0, n1(x,0)) * 2 - 2*n2(x,0) - (x>680 and x<760 and 4 or 0) end) * above(function(x) return 449 end)
t_far = pile{{"lead white",1.2},{"smalt",0.6},{"raw umber",0.5},{"red earth",0.12},{"green earth",0.3}, medium=0.1}
work(farline, {hand="detail", pile=t_far, angle=0, coverage=2.5, length={6,16}, tool={kind="round", width=3, point=0.4}, fill=true, clip=true})
-- left group
treesL = ells({{180,302,48,38},{150,340,60,45},{212,345,52,48},{168,390,72,45},{205,418,80,30},
               {292,338,44,40},{322,370,55,45},{268,388,58,40},{312,415,72,32},
               {72,368,38,40},{50,408,55,36},{98,418,52,28},{130,380,40,40},
               {382,392,30,30},{402,420,36,24},{240,395,40,40}}) + rect(20,418,410,32)
treesL = treesL:roughen(7, 22, 41):roughen(3, 8, 42) * above(function(x) return 450 end)
t_dark = pile{{"raw umber",1},{"bone black",0.35},{"green earth",1},{"smalt",0.3},{"lead white",0.2}, medium=0.08}
work(treesL, {hand="body", pile=t_dark, angle=function(x,y) return -1.2 + 0.4*math.sin(x/30) end, coverage=2.6, length={10,30}, tool="filbert 6", fill=true, clip=true, dips={3,0.8,0.3}})
-- right copse, bluish, more distant
treesR = ells({{830,405,45,22},{880,400,40,20},{790,420,40,18},{920,420,40,18},{860,425,90,16},{770,432,30,12},{950,435,30,10}})
treesR = treesR:roughen(5, 16, 43):roughen(2, 6, 44) * above(function(x) return 448 end)
t_mid = pile{{"raw umber",1},{"smalt",0.6},{"green earth",0.6},{"lead white",0.5},{"bone black",0.15}, medium=0.08}
work(treesR, {hand="body", pile=t_mid, angle=-1.3, coverage=2.6, length={8,20}, tool="filbert 4", fill=true, clip=true})

--@ chunk 15
cb = pile{{"lead white",2.2},{"smalt",0.6},{"red earth",0.22},{"raw umber",0.3},{"vermilion",0.04}, medium=0.2}
cbh = pile{{"lead white",2.6},{"cobalt blue",0.35},{"smalt",0.35},{"raw umber",0.25},{"red earth",0.1}, medium=0.2}
cg = pile{{"lead white",4},{"vermilion",0.22},{"chrome yellow",0.35},{"yellow ochre",0.1}, medium=0.15}
function streak(b, pl, x, y, len, load, pr, bow)
  b:reload(pl, load)
  local n = 6; local pts = {}
  for i=0,n do local t=i/n; pts[#pts+1] = {x + len*t, y + (bow or 0)*math.sin(t*math.pi) + randn(0,0.6)} end
  b:stroke(pts, {pressure={pr, pr*0.6}, ramps={0.25,0.4}, swell={0.8,1.1,1,0.7}})
end
local fb = brush{kind="filbert", width=7, stiffness=0.4}
local fs = brush{kind="filbert", width=4, stiffness=0.4}
-- bank A bodies (upper right)
for i=1,26 do
  local x = rand(330, 900); local y = rand(142, 182) - (x-330)*0.02
  streak(fb, cb, x, y, rand(60, 180), rand(0.35,0.55), rand(0.35,0.6), rand(-3,3))
end
-- bank A gold undersides
for i=1,18 do
  local x = rand(360, 940); local y = rand(180, 196) - (x-330)*0.02
  streak(fs, cg, x, y, rand(40, 140), rand(0.35,0.5), rand(0.3,0.5), rand(-2,2))
end
-- upper-left high clouds
for i=1,24 do
  local x = rand(-40, 560); local y = rand(35, 95) + (x)*0.015
  streak(fb, cbh, x, y, rand(70, 200), rand(0.3,0.5), rand(0.3,0.55), rand(-4,4))
end
for i=1,12 do
  local x = rand(-20, 520); local y = rand(90, 106) + (x)*0.015
  streak(fs, cg, x, y, rand(40, 130), rand(0.3,0.45), rand(0.25,0.45), rand(-2,2))
end
-- mid-left streak
for i=1,14 do
  local x = rand(-30, 380); local y = rand(270, 292)
  streak(fs, cb, x, y, rand(60, 160), rand(0.35,0.5), rand(0.35,0.55), rand(-2,2))
end
for i=1,8 do
  local x = rand(0, 400); local y = rand(292, 300)
  streak(fs, cg, x, y, rand(40, 110), rand(0.3,0.45), rand(0.3,0.45), rand(-1,1))
end

--@ chunk 16
skyb = (sky - treesL:grow(10) - treesR:grow(10)) * above(function(x) return 425 end); blend(skyb, {angle=0}); blend(skyb, {angle=0.08})

--@ chunk 17
print(wait(3*24*60)); for _,p in ipairs({{180,300},{200,420},{850,410},{300,560},{600,650},{600,160}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 18
print(wait(3*24*60)); for _,p in ipairs({{180,300},{200,420},{850,410},{300,560},{100,650},{600,160}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 19
local function ells(list)
  local m=nil
  for _,e in ipairs(list) do local el=ellipse(e[1],e[2],e[3],e[4]); m = m and (m+el) or el end
  return m
end
crownA = ells({{185,285,30,24},{160,305,34,27},{212,308,36,30},{175,335,50,35},{218,345,44,35},{148,362,44,35},{195,375,60,38},{168,400,58,24},{228,398,38,24}})
crownB = ells({{300,322,34,28},{332,345,40,32},{278,352,36,30},{312,375,55,38},{348,388,40,30},{288,402,52,24},{330,410,40,18}})
crownC = ells({{75,352,30,28},{52,385,44,35},{102,378,34,32},{75,405,50,22},{30,410,26,20}})
crownD = ells({{396,388,24,22},{412,410,27,20},{384,414,24,15}})
bushes = ells({{128,428,34,14},{252,430,34,13},{362,432,28,11},{20,432,22,12}})
crowns = (crownA + crownB + crownC + crownD):roughen(6, 20, 51):roughen(2.5, 7, 52) + bushes:roughen(3, 10, 53)
trunks = ribbon({{182,398},{181,420},{184,447}}, 6) + ribbon({{306,400},{303,425},{302,447}}, 5)
       + ribbon({{70,402},{71,425},{73,447}}, 4.5) + ribbon({{400,415},{401,447}}, 3.2)
       + ribbon({{205,405},{209,447}}, 3) + ribbon({{330,412},{328,447}}, 3) + ribbon({{150,410},{146,447}}, 2.5)
treesL2 = (crowns + trunks) * above(function(x) return 450 end)
local carve = treesL - treesL2
sk_mid = pile{{"lead white",5},{"vermilion",0.07},{"yellow ochre",0.25},{"chrome yellow",0.12},{"cobalt blue",0.06}, medium=0.1}
sk_low = pile{{"lead white",6},{"chrome yellow",0.45},{"vermilion",0.08},{"yellow ochre",0.1}, medium=0.1}
local c1 = carve * above(function(x) return 372 end)
local c2 = carve * below(function(x) return 372 end) * above(function(x) return 438 end)
local c3 = carve * below(function(x) return 438 end)
for pass=1,2 do
  work(c1, {hand="detail", pile=sk_mid, angle=0, coverage=2.5, length={5,14}, tool={kind="filbert", width=4}, fill=true, clip=true})
  work(c2, {hand="detail", pile=sk_low, angle=0, coverage=2.5, length={5,14}, tool={kind="filbert", width=4}, fill=true, clip=true})
  work(c3, {hand="detail", pile=t_far, angle=0, coverage=2.5, length={4,10}, tool={kind="filbert", width=3}, fill=true, clip=true})
end

--@ chunk 20
clumps = {{185,285,30,24},{160,305,34,27},{212,308,36,30},{175,335,50,35},{218,345,44,35},{148,362,44,35},{195,375,60,38},{168,400,58,24},{228,398,38,24},
 {300,322,34,28},{332,345,40,32},{278,352,36,30},{312,375,55,38},{348,388,40,30},{288,402,52,24},{330,410,40,18},
 {75,352,30,28},{52,385,44,35},{102,378,34,32},{75,405,50,22},{30,410,26,20},
 {396,388,24,22},{412,410,27,20},{384,414,24,15}}
t_base = pile{{"raw umber",1},{"bone black",0.3},{"green earth",1.1},{"smalt",0.25},{"lead white",0.15}, medium=0.08}
t_under = pile{{"raw umber",1},{"bone black",0.6},{"green earth",0.5}, medium=0.08}
t_top = pile{{"raw umber",1},{"green earth",1},{"lead white",0.55},{"smalt",0.35},{"yellow ochre",0.2}, medium=0.08}
local cm = crowns * above(function(x) return 446 end)
work(cm, {hand="body", pile=t_base, angle=function(x,y) return -1.0+0.5*math.sin(x/17+y/13) end, coverage=2.4, length={6,18}, tool="filbert 5", fill=true, clip=true, dips={4,0.8,0.3}})
-- undersides and tops per clump
local und, top = nil, nil
for _,e in ipairs(clumps) do
  local cx,cy,rx,ry = e[1],e[2],e[3],e[4]
  local u = ellipse(cx-0.05*rx, cy+0.45*ry, 0.8*rx, 0.5*ry) * ellipse(cx,cy,rx,ry)
  local t = ellipse(cx+0.1*rx, cy-0.4*ry, 0.7*rx, 0.5*ry) * ellipse(cx,cy,rx,ry)
  und = und and (und + u) or u
  top = top and (top + t) or t
end
clump_und = (und:roughen(4, 10, 61)) * cm
clump_top = (top:roughen(4, 10, 62) - clump_und) * cm
stipple(clump_und, {pile=t_under, width=4, coverage=1.3, pressure={0.4,0.7}, cluster=0.4, feather=0.3})
stipple(clump_top, {pile=t_top, width=3.5, coverage=0.9, pressure={0.3,0.6}, cluster=0.5, feather=0.4})

--@ chunk 21
g_lit  = pile{{"yellow ochre",1.2},{"lead white",0.55},{"red earth",0.15},{"green earth",0.3}, medium=0.12}
g_lit2 = pile{{"yellow ochre",1},{"lead white",0.3},{"green earth",0.6},{"raw umber",0.2}, medium=0.12}
g_dk   = pile{{"raw umber",1},{"bone black",0.3},{"green earth",0.6},{"smalt",0.15}, medium=0.12}
local mm = marsh - treesL2:grow(2)
local N = noise{seed=71, period=120, octaves=3, stretch={0, 4}}
-- distance bands: {y0,y1,tool width,len}
local bands = {{450,470,1.4,{3,8}},{468,495,2,{5,12}},{493,525,2.6,{7,16}},{523,560,3.4,{9,22}}}
for i,b in ipairs(bands) do
  local m = mm * rect(0,b[1],1000,b[2]-b[1])
  local lit = m * mask(function(x,y) return N:at01(x,y) > 0.52 and 1 or 0 end)
  local dk  = m * mask(function(x,y) return N:at01(x,y) < 0.36 and 1 or 0 end)
  work(lit, {hand="hatch", pile=(i%2==0) and g_lit or g_lit2, angle=0, coverage=0.7, length=b[4], tool={kind="flat", width=b[3]}, clip=m, load=0.45, pressure={0.3,0.6}})
  work(dk, {hand="hatch", pile=g_dk, angle=0, coverage=0.6, length=b[4], tool={kind="flat", width=b[3]}, clip=m, load=0.45, pressure={0.3,0.6}})
end

--@ chunk 22
local mm = (marsh - treesL2:grow(3)) * rect(0,446,1000,130); blend(mm, {angle=0}); blend(mm, {angle=0.04})

--@ chunk 23
g_brown = pile{{"raw umber",1},{"red earth",0.3},{"yellow ochre",0.6},{"green earth",0.4}, medium=0.1}
local fg = marsh * below(function(x) return 548 + 10*math.sin(x/47) + 6*math.sin(x/13) end)
local N = noise{seed=81, period=90, octaves=3}
local A = fg * mask(function(x,y) return N:at01(x,y) > 0.5 and 1 or 0 end)
local B = fg - A
local ang = function(x,y) return 0.05*math.sin(x/60) - 0.25*smoothstep(600,714,y)*math.sin(x/37) end
work(A, {hand="body", pile=g_brown, angle=ang, coverage=1.8, length={20,60}, tool="filbert 9", clip=marsh, fill=true, dips={3,0.8,0.3}})
work(B, {hand="body", pile=g_lmid, angle=ang, coverage=1.8, length={20,60}, tool="filbert 9", clip=marsh, fill=true, dips={3,0.8,0.3}})
local deep = marsh * below(function(x) return 640 + 12*math.sin(x/53) end)
work(deep, {hand="body", pile=g_near, angle=ang, coverage=1.5, length={25,70}, tool="filbert 11", clip=marsh, dips={3,0.8,0.3}})

--@ chunk 24
local fgb = marsh * below(function(x) return 530 + 8*math.sin(x/61) end); blend(fgb, {angle=0}); blend(fgb, {angle=-0.1}); blend(fgb, {angle=0.06})

--@ chunk 25
print(wait(3*24*60)); for _,p in ipairs({{180,330},{300,380},{850,410},{300,500},{100,650},{600,650},{880,500}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 26
print(wait(2*24*60)); for _,p in ipairs({{180,330},{300,380},{100,650},{300,620},{800,690}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 27
print(wait(2*24*60)); for _,p in ipairs({{180,330},{300,380},{100,650},{800,690},{60,380},{400,400}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 28
t_rim = pile{{"yellow ochre",1},{"red earth",0.25},{"lead white",0.5},{"green earth",0.4},{"raw umber",0.2}, medium=0.1}
local cres = nil
for _,e in ipairs(clumps) do
  local cx,cy,rx,ry = e[1],e[2],e[3],e[4]
  local c = ellipse(cx,cy,rx,ry) - ellipse(cx-0.22*rx, cy+0.12*ry, rx, ry)
  cres = cres and (cres + c) or c
end
local cm = crowns * above(function(x) return 446 end)
rimR = cres:roughen(2, 6, 91) * (cm - cm:shrink(9))
stipple(clump_und, {pile=t_under, width=3, coverage=0.8, pressure={0.35,0.6}, cluster=0.6, feather=0.4, drag=1.5})
stipple(clump_top, {pile=t_top, width=2.6, coverage=0.7, pressure={0.3,0.55}, cluster=0.6, feather=0.5, drag=1.5})
stipple(rimR, {pile=t_rim, width=2.2, coverage=0.8, pressure={0.3,0.55}, cluster=0.5, feather=0.5, drag=1.2})

--@ chunk 29
wa_far = pile{{"lead white",5},{"chrome yellow",0.4},{"vermilion",0.1},{"raw umber",0.2}, medium=0.1}
wa_mid = pile{{"lead white",3},{"yellow ochre",0.25},{"cobalt blue",0.3},{"smalt",0.2},{"raw umber",0.35}, medium=0.1}
wa_near = pile{{"lead white",2},{"smalt",0.7},{"cobalt blue",0.4},{"raw umber",0.45},{"red earth",0.1}, medium=0.1}
local wm = creek:shrink(0.5)
local function band(a,b) return wm * rect(0,a,1000,b-a) end
work(band(436,505), {hand="detail", pile=wa_far, angle=0, coverage=2.5, length={6,20}, tool={kind="filbert", width=3}, fill=true, clip=true})
work(band(495,600), {hand="body", pile=wa_mid, angle=0, coverage=2.5, length={15,40}, tool="filbert 6", fill=true, clip=wm})
work(band(585,720), {hand="body", pile=wa_near, angle=0, coverage=2.5, length={20,60}, tool="filbert 9", fill=true, clip=wm})
blend(wm, {angle=0}); blend(wm, {angle=0.08})

--@ chunk 30
gz = pile{{"raw umber",1},{"green earth",0.7},{"bone black",0.25}, medium=0.8}
local fgm = (marsh - creek:grow(3)) * below(function(x) return 545 + 10*math.sin(x/71) end)
local deep = (marsh - creek:grow(3)) * below(function(x) return 610 + 12*math.sin(x/43) end)
work(fgm, {hand="glaze", pile=gz, angle=0, coverage=1.3, clip=fgm})
work(deep, {hand="glaze", pile=gz, angle=0.05, coverage=1.3, clip=deep})
local bm = (marsh - creek:grow(4)) * below(function(x) return 520 + 10*math.sin(x/71) end)
blend(bm, {angle=0}); blend(bm, {angle=0.12}); blend(bm, {angle=-0.08})

--@ chunk 31
f_d1 = pile{{"raw umber",1.4},{"green earth",0.9},{"bone black",0.35},{"yellow ochre",0.3}, medium=0.12}
f_d2 = pile{{"raw umber",1},{"green earth",0.6},{"red earth",0.25},{"yellow ochre",0.4},{"bone black",0.15}, medium=0.12}
local fgm = (marsh - creek:grow(1)) * below(function(x) return 565 + 12*math.sin(x/71) + 6*math.sin(x/23) end)
local N = noise{seed=101, period=70, octaves=3}
local A = fgm * mask(function(x,y) return N:at01(x,y) > 0.45 and 1 or 0 end)
local B = fgm - A
local ang = function(x,y) return -0.08 + 0.1*math.sin(x/40+y/25) end
work(A, {hand="body", pile=f_d1, angle=ang, coverage=1.6, length={15,45}, tool="filbert 7", clip=marsh-creek, edge="loose", dips={3,0.75,0.3}, load=0.7})
work(B, {hand="body", pile=f_d2, angle=ang, coverage=1.4, length={15,45}, tool="filbert 7", clip=marsh-creek, edge="loose", dips={3,0.75,0.3}, load=0.7})

--@ chunk 32
local bm = (marsh - creek:grow(4)) * below(function(x) return 525 + 10*math.sin(x/71) end); blend(bm, {angle=0}); blend(bm, {angle=0.15})

--@ chunk 33
print(wait(3*24*60)); for _,p in ipairs({{600,650},{300,620},{800,690},{580,520},{620,460}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 34
w_refl = pile{{"raw umber",1},{"green earth",0.6},{"smalt",0.3},{"lead white",0.35},{"bone black",0.1}, medium=0.2}
local function edgeband(y0,y1,w) return (creek - creek:shrink(w)):roughen(w*0.4, 12, math.floor(y0)) * rect(0,y0,1000,y1-y0) end
local e1 = edgeband(441,505,1.6)
local e2 = edgeband(500,585,3.2)
local e3 = edgeband(580,720,5.5)
work(e1, {hand="detail", pile=w_refl, angle=0, coverage=1.6, length={3,8}, tool={kind="round", width=1.4}, clip=true})
work(e2, {hand="detail", pile=w_refl, angle=0, coverage=1.6, length={4,12}, tool={kind="round", width=2.2}, clip=true})
work(e3, {hand="detail", pile=w_refl, angle=0, coverage=1.6, length={6,16}, tool={kind="round", width=3}, clip=true})

--@ chunk 35
blend(creek, {angle=0}); blend(creek, {angle=math.pi/2})

--@ chunk 36
print(wait(2*24*60)); for _,p in ipairs({{600,650},{575,560},{560,510},{540,690},{300,620}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 37
wv = {
 {pile{{"lead white",5},{"chrome yellow",0.45},{"vermilion",0.1},{"raw umber",0.15}, medium=0.1}, 436, 482, 2.2, {5,14}},
 {pile{{"lead white",5},{"chrome yellow",0.25},{"yellow ochre",0.15},{"vermilion",0.06},{"raw umber",0.2},{"cobalt blue",0.05}, medium=0.1}, 474, 532, 3, {8,20}},
 {pile{{"lead white",4},{"yellow ochre",0.15},{"vermilion",0.05},{"cobalt blue",0.2},{"raw umber",0.25}, medium=0.1}, 524, 592, 4, {12,28}},
 {pile{{"lead white",3.5},{"cobalt blue",0.35},{"smalt",0.25},{"raw umber",0.3}, medium=0.1}, 584, 652, 5, {15,35}},
 {pile{{"lead white",3},{"cobalt blue",0.45},{"smalt",0.4},{"raw umber",0.4},{"red earth",0.05}, medium=0.1}, 644, 725, 6, {18,45}},
}
for i,b in ipairs(wv) do
  local m = creek * below(function(x) return b[2] + 4*math.sin(x/19+i) end) * above(function(x) return b[3] + 4*math.sin(x/23+2*i) end)
  work(m, {hand="body", pile=b[1], angle=function(x,y) return 0.03*math.sin(x/30+y/7) end, coverage=2.6, length=b[5], tool={kind="flat", width=b[4], stiffness=0.5}, fill=true, clip=creek, dips={3,0.8,0.3}, pressure={0.5,0.8}})
end

--@ chunk 38
gr_dk  = pile{{"raw umber",1},{"bone black",0.4},{"green earth",0.7}, medium=0.15}
gr_mid = pile{{"raw umber",1},{"green earth",1},{"yellow ochre",0.6}, medium=0.15}
gr_lit = pile{{"yellow ochre",1},{"lead white",0.4},{"red earth",0.15},{"green earth",0.2}, medium=0.12}
fgg = marsh - creek:grow(3)
function blade(b, x, y, len, ang, bend, pr)
  local c, s = math.cos(ang), math.sin(ang)
  local px, py = -s, c
  b:stroke({{x,y},{x+c*len*0.5+px*bend*0.3, y+s*len*0.5+py*bend*0.3},{x+c*len+px*bend, y+s*len+py*bend}},
    {pressure={pr, 0.0}, ramps={0.05,0.7}})
end
function tufts(n, y0, y1, pl, prmax, nbl, seed)
  local brushes = {}
  for i=1,n do
    local x = rand(0,1000); local y = rand(y0,y1)
    if fgg:at(x,y) > 0.5 then
      local k = (y-525)/190
      local w = 1.0 + 2.4*k
      local b = brush{kind="rigger", width=w, point=1}
      b:load(pl, 0.7)
      local L = 4 + 30*k
      local nb = math.random(nbl[1], nbl[2])
      for j=1,nb do
        local ang = -math.pi/2 + randn(0, 0.35)
        blade(b, x + rand(-L*0.25, L*0.25), y + rand(-1,1), L*rand(0.6,1.2), ang, randn(0, L*0.15), rand(0.5,prmax))
        if b:fullness() < 0.2 then b:load(pl, 0.6) end
      end
    end
  end
end
tufts(420, 530, 720, gr_dk, 0.9, {4,9})
tufts(260, 540, 720, gr_mid, 0.8, {3,7})

--@ chunk 39
function tuft_at(x, y, pl, prmax, nb, scale)
  if fgg:at(x,y) < 0.5 then return end
  local k = (y-525)/190
  local b = brush{kind="rigger", width=1.0 + 2.4*k, point=1}
  b:load(pl, 0.75)
  local L = (4 + 30*k) * (scale or 1)
  for j=1,nb do
    local ang = -math.pi/2 + randn(0, 0.3)
    blade(b, x + rand(-L*0.25, L*0.25), y + rand(-1,1), L*rand(0.6,1.25), ang, randn(0, L*0.15), rand(0.5,prmax))
    if b:fullness() < 0.2 then b:load(pl, 0.6) end
  end
end
-- clusters, denser near the bottom
for c=1,90 do
  local y = 530 + 190*math.sqrt(rand(0,1))
  local x = rand(0,1000)
  local r = 6 + 30*(y-525)/190
  for t=1,math.random(4,9) do
    tuft_at(x + randn(0, r), y + randn(0, r*0.35), (t%3==0) and gr_mid or gr_dk, 0.9, math.random(4,8), 1.0)
  end
end
-- tall reeds in corners and along the near left bank
local reed = brush{kind="rigger", width=2.6, point=1}
local function reeds(n, x0, x1, y0, y1, L0, L1)
  for i=1,n do
    local x, y = rand(x0,x1), rand(y0,y1)
    if fgg:at(x,y) > 0.5 then
      reed:load((i%4==0) and gr_mid or gr_dk, 0.7)
      local L = rand(L0, L1)
      blade(reed, x, y, L, -math.pi/2 + randn(0.05, 0.12), randn(0, L*0.08), rand(0.55,0.8))
    end
  end
end
reeds(70, 0, 220, 640, 720, 40, 95)
reeds(45, 380, 490, 640, 720, 35, 80)
reeds(50, 860, 1000, 650, 720, 40, 90)

--@ chunk 40
print(wait(4*24*60)); for _,p in ipairs({{600,650},{575,560},{100,690},{300,620},{900,700}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 41
wn1 = pile{{"lead white",2},{"cobalt blue",0.3},{"smalt",0.3},{"raw umber",0.35},{"yellow ochre",0.1}, medium=0.1}
wn2 = pile{{"lead white",1.6},{"cobalt blue",0.4},{"smalt",0.5},{"raw umber",0.5}, medium=0.1}
local ang = function(x,y) return 0.03*math.sin(x/30+y/7) end
local A = creek * rect(0,548,1000,40)
local B = creek * below(function(x) return 585 + 4*math.sin(x/17) end)
local C = creek * below(function(x) return 645 + 5*math.sin(x/21) end)
work(A, {hand="body", pile=wn1, angle=ang, coverage=0.9, length={10,30}, tool={kind="flat", width=3.5, stiffness=0.5}, clip=creek, load=0.4, pressure={0.3,0.6}})
work(B, {hand="body", pile=wn1, angle=ang, coverage=2.4, length={15,40}, tool={kind="flat", width=5, stiffness=0.5}, fill=true, clip=creek, dips={3,0.8,0.3}, pressure={0.5,0.8}})
work(C, {hand="body", pile=wn2, angle=ang, coverage=2.4, length={18,45}, tool={kind="flat", width=6, stiffness=0.5}, fill=true, clip=creek, dips={3,0.8,0.3}, pressure={0.5,0.8}})

--@ chunk 42
gz2 = pile{{"raw umber",1},{"bone black",0.35},{"green earth",0.6}, medium=0.6}
local base = marsh - creek:grow(1.5)
local g1 = base * below(function(x) return 560 + 10*math.sin(x/71) end)
local g2 = base * mask(function(x,y) local d = (y-620)/90 + math.abs(x-560)/900; return d > 0.35 and 1 or 0 end)
work(g1, {hand="glaze", pile=gz2, angle=0.03, coverage=1.3, clip=g1})
work(g2, {hand="glaze", pile=gz2, angle=-0.05, coverage=1.4, clip=g2})
local bm = base * below(function(x) return 545 + 10*math.sin(x/71) end)
blend(bm, {angle=0}); blend(bm, {angle=0.2})

--@ chunk 43
fig = pile{{"raw umber",1},{"bone black",0.5},{"red earth",0.15}, medium=0.1}
local r1 = brush{kind="round", width=1.6, point=0.8}
local r2 = brush{kind="round", width=1.0, point=1}
-- hull: slightly curved, low
r1:load(fig, 0.8)
r1:stroke({{570,456.2},{578,457.6},{590,457.8},{600,456.6},{603,455.2}}, {pressure={0.9,0.8}, ramps={0.05,0.1}})
r1:load(fig, 0.8)
r1:stroke({{572,457.5},{588,458.8},{600,457.6}}, {pressure={0.7,0.5}, ramps={0.05,0.2}})
-- figure standing near the stern (right), coat
r1:load(fig, 0.8)
r1:stroke({{594.2,442.5},{594.4,447},{594.6,451},{594.8,455.5}}, {pressure={0.95,0.8}, ramps={0.02,0.05}, swell={1,1.3,1.4,1.2}})
r2:load(fig, 0.8)
r2:stroke({{593.4,450},{593.2,455.6}}, {pressure={0.8,0.7}})
r2:stroke({{595.8,450},{596.2,455.6}}, {pressure={0.8,0.7}})
-- head and hat
r1:load(fig, 0.8)
r1:touch(594.3, 440.6, {pressure=0.7})
r2:load(fig, 0.6)
r2:stroke({{592.8,439.6},{595.8,439.4}}, {pressure={0.6,0.5}})
-- arms and pole (pole leaning forward-left into water)
r2:load(fig, 0.7)
r2:stroke({{594.3,444},{592.2,446.5},{591.2,447.2}}, {pressure={0.7,0.5}})
r2:stroke({{594.5,444.2},{596.5,442.8},{597.2,441.8}}, {pressure={0.7,0.5}})
r2:load(fig, 0.7)
r2:stroke({{599.5,434},{595,443},{589,455},{587,459.5}}, {pressure={0.5,0.35}, ramps={0.1,0.2}})
-- a second seated figure at the bow (small hump)
r1:load(fig, 0.7)
r1:stroke({{577.5,452.5},{577.8,455.5}}, {pressure={0.9,0.9}, swell={1,1.4}})
r1:touch(577.6, 451.2, {pressure=0.55})
-- reflection in the water beneath the boat, broken
local r3 = brush{kind="flat", width=1.2}
r3:load(pile{{"raw umber",1},{"bone black",0.3},{"smalt",0.3},{"lead white",0.4}, medium=0.2}, 0.5)
for i=1,5 do local y = 459.5 + i*0.9; r3:stroke({{574 + rand(-2,2), y},{598 + rand(-2,2), y}}, {pressure={0.4,0.2}}) end

--@ chunk 44
glow = pile{{"lead white",6},{"chrome yellow",0.8},{"vermilion",0.12}, medium=0.15}
glow2 = pile{{"lead white",6},{"chrome yellow",0.5},{"vermilion",0.25},{"yellow ochre",0.1}, medium=0.15}
local sunx, suny = 645, 436
local E = ellipse(sunx, suny, 170, 60) * above(function(x) return 436 end) - treesR:grow(3) - farline:grow(1)
local d = function(x,y) local dx=(x-sunx)/170; local dy=(y-suny)/60; return math.sqrt(dx*dx+dy*dy) end
work(E, {hand="body", pile=glow2, angle=0, coverage=1.6, length={20,60}, tool="filbert 8", load_at=function(x,y) return clamp(0.7*(1-d(x,y)), 0.05, 0.7) end, pressure={0.3,0.6}})
local E2 = ellipse(sunx, suny, 90, 30) * above(function(x) return 437 end) - farline:grow(1)
work(E2, {hand="body", pile=glow, angle=0, coverage=2, length={15,40}, tool="filbert 6", load_at=function(x,y) return clamp(0.8*(1-d(x,y)*1.6), 0.05, 0.8) end, pressure={0.4,0.7}})
local BE = ellipse(sunx, suny, 200, 75):soften(20) * above(function(x) return 437 end) - treesR:grow(4) - farline:grow(1)
blend(BE, {angle=0}); blend(BE, {angle=0.1})

--@ chunk 45
print(wait(2*24*60)); for _,p in ipairs({{594,440},{560,436},{700,436},{760,420},{300,650},{600,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 46
t_far2 = pile{{"lead white",1.3},{"smalt",0.5},{"raw umber",0.45},{"red earth",0.2},{"vermilion",0.03},{"green earth",0.2}, medium=0.1}
local fl = farline * rect(470, 415, 300, 32) - creek:grow(0.5)
work(fl, {hand="detail", pile=t_far2, angle=0, coverage=2.5, length={4,12}, tool={kind="round", width=2.2, point=0.4}, fill=true, clip=true})
work(treesR * rect(735, 380, 70, 70), {hand="detail", pile=t_mid, angle=-1.3, coverage=2.2, length={4,10}, tool={kind="round", width=2.5, point=0.3}, fill=true, clip=true})
local r1 = brush{kind="round", width=1.6, point=0.8}
local r2 = brush{kind="round", width=1.0, point=1}
r1:load(fig, 0.8)
r1:stroke({{594.2,442.5},{594.4,447}}, {pressure={0.95,0.9}, ramps={0.02,0.05}})
r1:load(fig, 0.8)
r1:touch(594.3, 440.6, {pressure=0.7})
r2:load(fig, 0.6)
r2:stroke({{592.8,439.6},{595.8,439.4}}, {pressure={0.6,0.5}})
r2:load(fig, 0.7)
r2:stroke({{599.5,434},{595,443}}, {pressure={0.5,0.45}, ramps={0.1,0.05}})
r2:stroke({{594.5,444.2},{596.5,442.8},{597.2,441.8}}, {pressure={0.7,0.5}})

--@ chunk 47
t_mid2 = pile{{"raw umber",1},{"smalt",0.6},{"green earth",0.6},{"lead white",0.55},{"bone black",0.12},{"red earth",0.05}, medium=0.08}
work(treesR, {hand="detail", pile=t_mid2, angle=function(x,y) return -1.2+0.5*math.sin(x/9+y/7) end, coverage=2.6, length={4,10}, tool={kind="round", width=2.6, point=0.3}, fill=true, clip=true, dips={6,0.8,0.3}})
local cr = {{830,405,45,22},{880,400,40,20},{790,420,40,18},{920,420,40,18}}
local top, lrim = nil, nil
for _,e in ipairs(cr) do
  local cx,cy,rx,ry = e[1],e[2],e[3],e[4]
  local t = ellipse(cx+0.05*rx, cy-0.45*ry, 0.75*rx, 0.5*ry)
  local l = ellipse(cx,cy,rx,ry) - ellipse(cx+0.2*rx, cy+0.1*ry, rx, ry)
  top = top and (top+t) or t; lrim = lrim and (lrim+l) or l
end
local tr_top = pile{{"raw umber",1},{"smalt",0.6},{"green earth",0.5},{"lead white",1.0}, medium=0.08}
local tr_rim = pile{{"yellow ochre",1},{"red earth",0.3},{"lead white",0.7},{"smalt",0.2}, medium=0.08}
stipple(top:roughen(3,8,121) * treesR, {pile=tr_top, width=2.2, coverage=0.8, pressure={0.3,0.5}, cluster=0.5, feather=0.4})
stipple(lrim:roughen(2,6,122) * (treesR - treesR:shrink(6)), {pile=tr_rim, width=1.8, coverage=0.7, pressure={0.3,0.5}, cluster=0.4, feather=0.4})

--@ chunk 48
gr_lit2 = pile{{"yellow ochre",1},{"lead white",0.25},{"red earth",0.2},{"raw umber",0.2},{"green earth",0.2}, medium=0.12}
-- seam: small dark tufts along the fore/mid boundary
for i=1,160 do
  local x = rand(0,1000)
  local y = 528 + 10*math.sin(x/71) + rand(-6,8)
  tuft_at(x, y, (i%3==0) and gr_mid or gr_dk, 0.85, math.random(3,6), 0.9)
end
-- lit tips, sparser toward the bottom
for i=1,170 do
  local y = 535 + 170*(rand(0,1)^1.6)
  local x = rand(0,1000)
  tuft_at(x, y, (i%2==0) and gr_lit or gr_lit2, 0.6, math.random(2,4), 0.9)
end

--@ chunk 49
print(wait(3*24*60)); for _,p in ipairs({{300,650},{200,560},{850,410},{600,650},{900,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 50
gz3 = pile{{"raw umber",1},{"bone black",0.25},{"green earth",0.5},{"red earth",0.1}, medium=0.62}
local base = marsh - creek:grow(1.5)
local g1 = base * below(function(x) return 538 + 10*math.sin(x/71) end)
local g2 = base * mask(function(x,y) local d = (y-640)/80 + math.abs(x-560)/700; return d > 0.45 and 1 or 0 end)
work(g1, {hand="glaze", pile=gz3, angle=0.02, coverage=1.2, clip=g1})
work(g2, {hand="glaze", pile=gz3, angle=-0.04, coverage=1.0, clip=g2})
local bm = base * below(function(x) return 530 + 10*math.sin(x/71) end)
blend(bm, {angle=0}); blend(bm, {angle=-0.15})

--@ chunk 51
gr_olive = pile{{"green earth",1},{"raw umber",0.8},{"yellow ochre",0.4},{"bone black",0.1}, medium=0.12}
gr_rust = pile{{"raw umber",1},{"red earth",0.35},{"yellow ochre",0.3}, medium=0.12}
local piles = {gr_dk, gr_dk, gr_olive, gr_rust, gr_dk, gr_olive}
for i=1,900 do
  local y = 530 + 190*math.sqrt(rand(0,1))
  local x = rand(0,1000)
  tuft_at(x, y, piles[(i % #piles) + 1], 0.85, math.random(4,8), rand(0.8,1.4))
end

--@ chunk 52
local refl = pile{{"raw umber",1},{"green earth",0.5},{"smalt",0.35},{"lead white",0.5},{"bone black",0.1}, medium=0.15}
local rf = brush{kind="flat", width=1.5, stiffness=0.5}
local N = #creek_line
for i=2,N-1 do
  local p, a, b = creek_line[i], creek_line[i-1], creek_line[i+1]
  local tx, ty = b[1]-a[1], b[2]-a[2]; local l = math.sqrt(tx*tx+ty*ty); tx, ty = tx/l, ty/l
  local nx, ny = -ty, tx
  local y = p[2]
  if y > 470 then
    local w = 1 + 0.5*(y-441)
    local k = (y-441)/280
    for side=-1,1,2 do
      local ex, ey = p[1] + side*nx*w/2, p[2] + side*ny*w/2
      -- dark reflection just inside the water
      if math.random() < 0.7 then
        rf:reload(refl, 0.45)
        local ix, iy = ex - side*nx*(1+3*k), ey - side*ny*(1+3*k)
        local len = 3 + 14*k
        rf:stroke({{ix - len*0.5, iy}, {ix + len*0.5, iy + rand(-0.5,0.5)}}, {pressure={0.4,0.2}})
      end
      -- tufts on the bank, leaning a bit toward the water
      if math.random() < 0.8 then
        local lean = -side*nx*0.35
        local bb = brush{kind="rigger", width=1.0+2.2*k, point=1}
        local pl = (math.random()<0.5) and gr_dk or gr_olive
        bb:load(pl, 0.75)
        local L = 4 + 32*k
        for j=1,math.random(3,6) do
          local ang = -math.pi/2 + lean + randn(0,0.3)
          blade(bb, ex + side*nx*rand(0.5,3) + rand(-2,2), ey + side*ny*rand(0.5,3) + rand(0,2), L*rand(0.6,1.2), ang, randn(0, L*0.15), rand(0.5,0.85))
          if bb:fullness() < 0.2 then bb:load(pl, 0.6) end
        end
      end
    end
  end
end

--@ chunk 53
print(drying(600,650), drying(560,690), drying(590,560))
local pinkw = pile{{"lead white",2.2},{"red earth",0.2},{"vermilion",0.06},{"smalt",0.3},{"raw umber",0.2}, medium=0.2}
local glint = pile{{"lead white",5},{"chrome yellow",0.35},{"vermilion",0.06}, medium=0.15}
local f = brush{kind="flat", width=3, stiffness=0.5}
local inner = creek:shrink(3)
for i=1,22 do
  local y = rand(672, 714); local x = rand(470, 660)
  if inner:at(x,y) > 0.5 then
    f:reload(pinkw, rand(0.3,0.45))
    local len = rand(15, 45)
    f:stroke({{x-len/2, y},{x, y+rand(-0.4,0.4)},{x+len/2, y}}, {pressure={0.35,0.15}, ramps={0.3,0.4}, clip=inner})
  end
end
local g = brush{kind="flat", width=1.6, stiffness=0.5}
for i=1,26 do
  local y = 500 + 150*rand(0,1)^1.3; local x = rand(500, 680)
  if inner:at(x,y) > 0.5 then
    g:reload(glint, rand(0.25,0.4))
    local len = 3 + (y-470)*0.12*rand(0.5,1.2)
    g:stroke({{x-len/2, y},{x+len/2, y+rand(-0.3,0.3)}}, {pressure={0.3,0.1}, ramps={0.3,0.5}, clip=inner})
  end
end

--@ chunk 54
print(drying(850,410), drying(800,425), drying(900,400))
local birdp = pile{{"raw umber",1},{"bone black",0.4},{"smalt",0.3},{"lead white",0.5}, medium=0.15}
local rg = brush{kind="rigger", width=1.1, point=1}
local function bird(x, y, s, tilt)
  rg:reload(birdp, 0.5)
  local lw = {x - s, y - s*0.35 + tilt}
  local rw = {x + s, y - s*0.4 - tilt}
  rg:stroke({lw, {x - s*0.45, y - s*0.45 + tilt*0.5}, {x, y}}, {pressure={0.1,0.55}, ramps={0.4,0.05}})
  rg:stroke({{x, y}, {x + s*0.45, y - s*0.5 - tilt*0.5}, rw}, {pressure={0.55,0.05}, ramps={0.05,0.5}})
end
bird(548, 262, 5.5, 0.5)
bird(566, 251, 4.5, -0.6)
bird(589, 270, 3.8, 0.3)
bird(530, 281, 3.2, -0.3)
bird(612, 258, 2.8, 0.4)

--@ chunk 55
s_top = pile{{"lead white",2.5},{"smalt",1},{"cobalt blue",0.5},{"raw umber",0.35},{"red earth",0.05}, medium=0.15}
local T = rect(0,0,1000,85)
work(T, {hand="broad", pile=s_top, angle=function(x,y) return 0.03*math.sin(x/150) end, coverage=1.6, tool="filbert 14", load_at=function(x,y) return clamp(0.55*(1 - y/85), 0.03, 0.55) end, pressure={0.35,0.6}})
local BT = rect(0,0,1000,125)
blend(BT, {angle=0}); blend(BT, {angle=0.1})

--@ chunk 56
print(wait(3*24*60)); for _,p in ipairs({{850,410},{800,425},{500,30},{300,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 57
print(wait(2*24*60)); print(drying(800,425), drying(760,430), drying(950,430))

--@ chunk 58
local cr = {{830,405,45,22},{880,400,40,20},{790,420,40,18},{920,420,40,18},{860,425,90,16}}
local top, lrim, und = nil, nil, nil
for _,e in ipairs(cr) do
  local cx,cy,rx,ry = e[1],e[2],e[3],e[4]
  local t = ellipse(cx+0.05*rx, cy-0.45*ry, 0.75*rx, 0.5*ry) * ellipse(cx,cy,rx,ry)
  local l = ellipse(cx,cy,rx,ry) - ellipse(cx+0.18*rx, cy+0.1*ry, rx, ry)
  local u = ellipse(cx, cy+0.5*ry, 0.85*rx, 0.45*ry) * ellipse(cx,cy,rx,ry)
  top = top and (top+t) or t; lrim = lrim and (lrim+l) or l; und = und and (und+u) or u
end
local tr_top = pile{{"raw umber",1},{"smalt",0.6},{"green earth",0.5},{"lead white",1.1}, medium=0.08}
local tr_rim = pile{{"yellow ochre",1},{"red earth",0.3},{"lead white",0.8},{"smalt",0.15}, medium=0.08}
local tr_und = pile{{"raw umber",1},{"smalt",0.5},{"bone black",0.25},{"green earth",0.4},{"lead white",0.2}, medium=0.08}
stipple(und:roughen(3,8,131) * treesR, {pile=tr_und, width=2.2, coverage=0.9, pressure={0.35,0.55}, cluster=0.5, feather=0.4, drag=1})
stipple(top:roughen(3,8,132) * treesR, {pile=tr_top, width=2, coverage=0.7, pressure={0.3,0.5}, cluster=0.6, feather=0.5, drag=1})
stipple(lrim:roughen(2,6,133) * (treesR - treesR:shrink(6)), {pile=tr_rim, width=1.7, coverage=0.7, pressure={0.3,0.5}, cluster=0.4, feather=0.5})
-- a few small sky holes and trunk gaps
local hole = pile{{"lead white",6},{"chrome yellow",0.4},{"vermilion",0.08},{"yellow ochre",0.1}, medium=0.1}
local r = brush{kind="round", width=1.6, point=0.3}
for _,p in ipairs({{812,432},{846,436},{893,434},{935,436},{870,408},{905,413}}) do r:reload(hole, 0.6); r:touch(p[1], p[2], {pressure=rand(0.4,0.6)}) end

--@ chunk 59
for i=1,230 do
  local x = rand(0,1000)
  local y = 530 + 10*math.sin(x/71) + rand(-3,10)
  local pl = ({gr_dk, gr_olive, gr_dk, gr_rust})[(i%4)+1]
  tuft_at(x, y, pl, 0.85, math.random(3,7), rand(1.6,2.8))
end

--@ chunk 60
s_top2 = pile{{"lead white",2},{"smalt",1},{"cobalt blue",0.6},{"raw umber",0.4},{"red earth",0.06}, medium=0.15}
local T = rect(0,0,1000,70)
work(T, {hand="broad", pile=s_top2, angle=function(x,y) return 0.03*math.sin(x/170+1) end, coverage=1.6, tool="filbert 14", load_at=function(x,y) return clamp(0.6*(1 - y/70)^1.3, 0.02, 0.6) end, pressure={0.35,0.6}})
local BT = rect(0,0,1000,110)
blend(BT, {angle=0}); blend(BT, {angle=-0.08})

--@ chunk 61
print(wait(120*24*60)); print(drying(500,30), drying(850,410), drying(300,600)); varnish{coats=0.3, vary=0.08}
