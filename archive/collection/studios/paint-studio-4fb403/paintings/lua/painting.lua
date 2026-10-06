-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=600, aspect=1.5, linen={16,13}, seed=7, ground={{pile={{"lead white",6},{"yellow ochre",1}}, um=120, apply="knife", texture=0.3},{pile={{"lead white",3},{"red earth",1},{"yellow ochre",1},{"raw umber",0.5}}, um=25, apply="brush"}}}
print(W,H)

--@ chunk 2

h = pencil("2B")
h:rule({0,415},{1000,415},{pressure=0.3})
-- distant shore
h:sketch({{0,392},{60,386},{130,390},{200,384},{260,392},{340,395},{420,400},{520,405},{620,410},{700,414}},{pressure=0.3})
-- spire
h:line({{228,392},{232,352},{236,392}},{pressure=0.4, smooth=false})
-- channel edges
h:sketch({{560,667},{600,600},{660,540},{700,490},{690,455},{650,430},{640,418}},{pressure=0.3})
h:sketch({{1000,560},{930,520},{850,480},{780,450},{720,430},{690,418}},{pressure=0.3})
-- mud near bank lower left
h:sketch({{0,520},{120,505},{260,500},{400,500},{520,480},{600,450},{640,430}},{pressure=0.25})
-- boat
h:sketch({{150,560},{200,585},{300,590},{380,570},{400,545}},{pressure=0.4})
h:sketch({{150,560},{250,548},{400,545}},{pressure=0.35})
h:line({{268,550},{262,330}},{pressure=0.4})
-- posts
h:line({{585,470},{586,432}},{pressure=0.4})
h:line({{615,478},{616,445}},{pressure=0.4})
h:line({{760,500},{761,462}},{pressure=0.4})
-- cloud masses
h:sketch({{0,120},{150,90},{320,140},{480,110},{640,170},{800,150},{1000,200}},{pressure=0.2})
h:sketch({{380,300},{500,260},{640,250},{760,270},{900,300},{1000,290}},{pressure=0.2})

--@ chunk 3

sky_top = pile{{"lead white",5},{"smalt",2},{"raw umber",0.3},{"red earth",0.1}}
sky_mid = pile{{"lead white",7},{"smalt",1},{"red earth",0.35},{"yellow ochre",0.3}}
sky_low = pile{{"lead white",9},{"chrome yellow",0.8},{"yellow ochre",0.6},{"red earth",0.1}}
skyM = rect(0,0,1000,420)
local n = noise{seed=3, period=300, octaves=3}
local b1 = below(function(x) return 170 + 25*n(x,0) end)
local b2 = below(function(x) return 300 + 20*n(x,100) end)
work(skyM - b1, {hand="broad", pile=sky_top, angle=function(x,y) return 0.1*n(x,y) end, coverage=2.5, clip=skyM})
work(skyM * b1 - b2:grow(0), {hand="broad", pile=sky_mid, angle=0.05, coverage=2.5, clip=skyM})
work(skyM * b2, {hand="broad", pile=sky_low, angle=0, coverage=2.5, clip=skyM})

--@ chunk 4

sky_top2 = pile{{"lead white",4},{"smalt",2.5},{"raw umber",0.4},{"red earth",0.15}}
local n = noise{seed=3, period=300, octaves=3}
local b1 = below(function(x) return 170 + 25*n(x,0) end)
local b2 = below(function(x) return 300 + 20*n(x,100) end)
work(skyM - b1:grow(10), {hand="body", tool="filbert 14", pile=sky_top2, angle=0.05, angle_jitter=0.3, coverage=3, length={40,120}, fill=true, clip=skyM})
work(skyM * b1 - b2, {hand="body", tool="filbert 14", pile=sky_mid, angle=0.03, coverage=3, length={40,120}, fill=true, clip=skyM})
work(skyM * b2, {hand="body", tool="filbert 14", pile=sky_low, angle=0, coverage=3, length={40,120}, fill=true, clip=skyM})
blend(skyM, {angle=0})

--@ chunk 5

blend(rect(0,110,1000,110), {angle=1.45, clip=false, length={30,70}})
blend(rect(0,250,1000,100), {angle=1.5, clip=false, length={30,70}})

--@ chunk 6

cloud_d = pile{{"lead white",2.5},{"smalt",1.1},{"red earth",0.6},{"raw umber",0.9}}
cloud_m = pile{{"lead white",4},{"smalt",1},{"red earth",0.55},{"raw umber",0.4}}
local n = noise{seed=11, period=160, octaves=4}
local n2 = noise{seed=12, period=90, octaves=3, stretch={0,3}}
bank = mask(function(x,y)
  local edge = 150 + 45*n(x,0) + 30*math.sin(x/140) - 40*smoothstep(500,900,x)
  local v = smoothstep(edge+10, edge-10, y)
  local holes = n2(x,y) > 0.35 and y < edge-30 and 0 or 1
  return v*holes
end)
work(bank, {hand="body", tool="filbert 12", pile=cloud_m, angle=0.08, angle_jitter=0.25, coverage=2.2, length={30,90}})
local core = mask(function(x,y)
  local edge = 110 + 40*n(x,50) + 30*math.sin(x/140) - 50*smoothstep(450,900,x)
  return smoothstep(edge+10, edge-10, y) * (n2(x,y+40) < 0.2 and 1 or 0)
end)
work(core, {hand="body", tool="filbert 12", pile=cloud_d, angle=0.08, angle_jitter=0.25, coverage=2, length={30,90}})

--@ chunk 7

blend(rect(0,0,1000,240), {angle=0.05})

--@ chunk 8

strat = pile{{"lead white",3.5},{"smalt",1},{"red earth",0.6},{"raw umber",0.6}}
glow = pile{{"lead white",10},{"chrome yellow",1},{"vermilion",0.08}}
local bars = {{250,232,230,13},{640,212,270,15},{140,282,170,9},{830,268,200,11},{470,300,170,7},{930,322,110,6},{90,340,130,6},{360,352,120,4}}
stratM = nil
for i,b in ipairs(bars) do
  local e = ellipse(b[1],b[2],b[3],b[4]):roughen(6, 50, i, 0)
  stratM = stratM and (stratM + e) or e
end
work(stratM, {hand="body", tool="filbert 8", pile=strat, angle=0, angle_jitter=0.05, coverage=2.5, length={40,140}, pressure={0.3,0.6}, clip=true})
sunG = ellipse(720,350,170,55):blur(30)
work(sunG, {hand="body", tool="filbert 12", pile=glow, angle=0, coverage=2, length={40,120}, pressure={0.3,0.6}})

--@ chunk 9

blend(rect(0,190,1000,225), {angle=0, pressure={0.3,0.6}})

--@ chunk 10

land = rect(0,415,1000,260)
local chanL = {{560,680},{600,600},{660,540},{700,490},{692,455},{655,430},{640,416}}
local chanR = {{690,416},{725,430},{780,450},{850,480},{930,520},{1010,562}}
local cp = {}
for _,p in ipairs(chanL) do cp[#cp+1]=p end
for _,p in ipairs(chanR) do cp[#cp+1]=p end
cp[#cp+1] = {1010,680}
chan = poly(cp, true) * land
local rb = {{690,416},{1010,416}}
for i=#chanR,1,-1 do rb[#rb+1]=chanR[i] end
rbank = (poly(rb, true) * land) - chan
local nm = {{-10,522},{120,506},{260,501},{400,500},{520,482},{600,452},{645,432}}
for i=#chanL,1,-1 do if chanL[i][2] > 440 then nm[#nm+1]=chanL[i] end end
nm[#nm+1] = {560,680}; nm[#nm+1]={-10,680}
nearmud = poly(nm, true):roughen(4,40,5,0) * land - chan
flats = land - chan - rbank - nearmud
print(chan:area(), rbank:area(), nearmud:area(), flats:area())

--@ chunk 11

trees_p = pile{{"lead white",2.2},{"smalt",1},{"raw umber",1.2},{"green earth",1},{"red earth",0.25}}
local n = noise{seed=21, period=40, octaves=3, kind="billow"}
local n2 = noise{seed=22, period=200, octaves=2}
treesM = mask(function(x,y)
  if y > 418 then return 0 end
  local base = 402 + 6*n2(x,0) + 13*smoothstep(380,700,x) 
  local top = base - 9*math.max(0,n(x,0)+0.3) - 5*smoothstep(0,150,150-x)
  if x > 705 then return 0 end
  return smoothstep(top-1.5, top+1.5, y)
end)
work(treesM, {hand="detail", tool={kind="round", width=3, point=0.3}, pile=trees_p, angle=0, coverage=3, length={6,20}, fill=true})
local sp = pile{{"lead white",2.2},{"smalt",1},{"raw umber",1.4},{"red earth",0.25}}
spire = poly({{229,396},{232.5,352},{236,396}})
b_sp = brush{kind="round", width=2.2, point=0.8}
b_sp:load(sp, 0.7)
b_sp:stroke({{232.5,354},{232.8,398}}, {pressure={0.1,0.8}})
b_sp:stroke({{230,380},{230,400}}, {pressure={0.8,0.8}})
b_sp:stroke({{235,380},{235,400}}, {pressure={0.8,0.8}})

--@ chunk 12

flat_p = pile{{"lead white",3},{"raw umber",1.4},{"red earth",0.5},{"smalt",0.5},{"yellow ochre",0.4}}
mud_p = pile{{"raw umber",3},{"red earth",0.7},{"bone black",0.35},{"lead white",0.6},{"smalt",0.4}}
rb_p = pile{{"raw umber",2},{"lead white",1.6},{"red earth",0.4},{"smalt",0.6}}
work(flats, {hand="body", tool="filbert 8", pile=flat_p, angle=0, angle_jitter=0.06, coverage=3, length={30,90}, fill=true, clip=true})
work(rbank, {hand="body", tool="filbert 8", pile=rb_p, angle=function(x,y) return 0.35 end, angle_jitter=0.08, coverage=3, length={30,90}, fill=true, clip=true})
work(nearmud, {hand="body", tool="filbert 12", pile=mud_p, angle=-0.08, angle_jitter=0.12, coverage=3, length={40,110}, fill=true, clip=true})

--@ chunk 13

local nn = noise{seed=31, period=120, octaves=3}
nearmud2 = mask(function(x,y)
  local top = 525 - 25*smoothstep(0,700,x) + 6*nn(x,0)
  return smoothstep(top-1.5, top+1.5, y)
end) * land - chan
mid_p = pile{{"lead white",2},{"raw umber",1.8},{"red earth",0.55},{"smalt",0.5},{"yellow ochre",0.3}}
local R = (nearmud - nearmud2) + (flats * rect(0,470,1000,100))
work(R - nearmud2, {hand="body", tool="filbert 8", pile=mid_p, angle=0, angle_jitter=0.04, coverage=3, length={40,120}, fill=true, clip=true})

--@ chunk 14

ch_hi = pile{{"lead white",9},{"chrome yellow",0.7},{"yellow ochre",0.5},{"smalt",0.1}}
ch_mid = pile{{"lead white",6},{"smalt",0.8},{"red earth",0.45},{"yellow ochre",0.3},{"raw umber",0.3}}
ch_lo = pile{{"lead white",4},{"smalt",1.2},{"red earth",0.55},{"raw umber",0.7}}
local n = noise{seed=41, period=150, octaves=2}
local y1 = below(function(x) return 470 + 15*n(x,0) end)
local y2 = below(function(x) return 560 + 20*n(x,50) end)
work(chan - y1, {hand="body", tool="filbert 8", pile=ch_hi, angle=0, angle_jitter=0.03, coverage=3, length={30,100}, fill=true, clip=true})
work(chan * y1 - y2, {hand="body", tool="filbert 10", pile=ch_mid, angle=0, angle_jitter=0.03, coverage=3, length={40,120}, fill=true, clip=true})
work(chan * y2, {hand="body", tool="filbert 12", pile=ch_lo, angle=0, angle_jitter=0.03, coverage=3, length={50,140}, fill=true, clip=true})
blend(chan, {angle=0})

--@ chunk 15

print(wait(0), drying(500,100), drying(500,380), drying(300,450), drying(300,600), drying(800,600))

--@ chunk 16

local n = noise{seed=51, period=100, octaves=3}
farwater = mask(function(x,y)
  if y < 416 then return 0 end
  local lim
  if x < 690 then lim = 421 + 2*n(x,0) else lim = 428 + 8*smoothstep(700,1000,x) + 4*n(x,9) end
  return smoothstep(lim+1, lim-1, y)
end) - chan
work(farwater, {hand="body", tool={kind="flat", width=4}, pile=ch_hi, angle=0, angle_jitter=0.01, coverage=3, length={40,160}, pressure={0.4,0.7}, fill=true, clip=true})

--@ chunk 17

print(wait(36*60), drying(500,100), drying(300,600), drying(800,600), drying(300,450), drying(900,425))

--@ chunk 18

print(wait(30*60), drying(500,100), drying(300,600), drying(800,600), drying(300,450), drying(900,425), drying(700,440))

--@ chunk 19

function interp(pts, y)
  if y <= pts[1][2] then return pts[1][1] end
  for i=2,#pts do
    if y <= pts[i][2] then
      local a, b = pts[i-1], pts[i]
      local t = (y - a[2]) / (b[2] - a[2])
      return a[1] + (b[1]-a[1]) * t
    end
  end
  return pts[#pts][1]
end
LE = {{660,416},{640,424},{602,434},{628,448},{592,466},{636,488},{652,505},{628,540},{592,590},{548,667}}
local nL = noise{seed=61, period=30, octaves=2}
leftland = mask(function(x,y)
  if y < 416 then return 0 end
  local e = interp(LE, y) + 4*nL(0,y)
  return smoothstep(e+1.2, e-1.2, x)
end)
sandR = poly({{742,452},{780,443},{850,440},{930,442},{1010,445},{1010,492},{960,482},{900,470},{840,462},{780,458}}, true):roughen(3,30,62,0)
spit = poly({{1010,598},{960,612},{900,640},{860,680},{1010,680}}, true):roughen(4,40,63,0)
water = land - leftland - sandR - spit
print(water:area(), leftland:area())

--@ chunk 20

w_mid = pile{{"lead white",6},{"smalt",0.7},{"red earth",0.35},{"chrome yellow",0.25},{"raw umber",0.25}}
w_lo = pile{{"lead white",3.6},{"smalt",1.2},{"red earth",0.55},{"raw umber",0.8}}
local n = noise{seed=71, period=120, octaves=2}
local a = below(function(x) return 485 + 12*n(x,0) end)
local b = below(function(x) return 575 + 15*n(x,40) end)
work(water - a, {hand="body", tool={kind="flat", width=6}, pile=ch_hi, angle=0, angle_jitter=0.02, coverage=3, length={40,140}, pressure={0.4,0.7}, fill=true, clip=true})
work(water * a - b, {hand="body", tool={kind="flat", width=9}, pile=w_mid, angle=0, angle_jitter=0.02, coverage=3, length={50,160}, fill=true, clip=true})
work(water * b, {hand="body", tool={kind="flat", width=12}, pile=w_lo, angle=0, angle_jitter=0.02, coverage=3, length={60,180}, fill=true, clip=true})
blend(water, {angle=0})

--@ chunk 21

local nn = noise{seed=81, period=140, octaves=3}
nearTop = function(x) return 522 - 24*smoothstep(0,650,x) + 5*nn(x,0) end
nearM = mask(function(x,y) local t = nearTop(x); return smoothstep(t-1.2, t+1.2, y) end) * leftland
flatsL = leftland - nearM
-- water lenses in the flats
local lenses = nil
math.randomseed(812)
local specs = {
 {80,426,160,2.2},{330,428,120,1.8},{520,431,90,2},{210,436,140,2.5},{430,440,110,2.4},
 {60,447,120,3},{300,452,180,3.2},{540,455,70,2.8},{160,466,150,4},{420,470,160,4.2},
 {40,486,110,4.5},{270,490,120,5},{500,482,90,4},{600,470,35,3}}
for i,s in ipairs(specs) do
  local e = ellipse(s[1],s[2],s[3],s[4]):roughen(1.5, 20, 100+i, 0)
  lenses = lenses and (lenses + e) or e
end
lensM = lenses * flatsL
sand_p = pile{{"lead white",2.4},{"raw umber",1.6},{"red earth",0.55},{"smalt",0.55},{"yellow ochre",0.35}}
work(flatsL - lensM, {hand="body", tool={kind="flat", width=5}, pile=sand_p, angle=0, angle_jitter=0.03, coverage=3, length={30,120}, pressure={0.4,0.7}, fill=true, clip=true})
work(lensM, {hand="detail", tool={kind="flat", width=3}, pile=ch_hi, angle=0, coverage=3, length={20,80}, fill=true})

--@ chunk 22

blend(flatsL, {angle=0, pressure={0.4,0.7}})

--@ chunk 23

mud_d = pile{{"raw umber",3},{"bone black",0.45},{"red earth",0.6},{"smalt",0.5},{"lead white",0.35}}
mud_l = pile{{"raw umber",2.4},{"red earth",0.6},{"lead white",0.9},{"smalt",0.45},{"yellow ochre",0.2}}
local n = noise{seed=91, period=90, octaves=2}
local lo = below(function(x) return 575 + 20*n(x,0) end)
work(nearM - lo, {hand="body", tool="filbert 10", pile=mud_l, angle=function(x,y) return -0.03 + 0.08*n(x,y) end, coverage=3, length={40,120}, fill=true, clip=true})
work(nearM * lo, {hand="body", tool="filbert 14", pile=mud_d, angle=function(x,y) return 0.05*n(x,y) end, coverage=3, length={50,150}, fill=true, clip=true})
blend(nearM * rect(0,540,1000,80), {angle=1.3, pressure={0.3,0.5}})

--@ chunk 24

blend(nearM, {angle=0.02, pressure={0.4,0.7}})

--@ chunk 25
print(wait(26*60), drying(300,600), drying(800,600), drying(300,460))

--@ chunk 26
print(wait(24*60), drying(300,600), drying(800,600), drying(300,460), drying(900,470))

--@ chunk 27

mud_dd = pile{{"raw umber",3},{"bone black",0.6},{"red earth",0.5},{"smalt",0.4},{"lead white",0.1}}
mud_m2 = pile{{"raw umber",2.6},{"red earth",0.6},{"bone black",0.2},{"lead white",0.5},{"smalt",0.35}}
local n = noise{seed=101, period=110, octaves=3}
local lo = below(function(x) return 565 + 18*n(x,0) end)
local mid = below(function(x) return nearTop(x) + 10 + 6*n(x,30) end)
work(nearM * mid - lo, {hand="body", tool="filbert 10", pile=mud_m2, angle=0, angle_jitter=0.05, coverage=3, length={40,120}, fill=true, clip=true})
work(nearM * lo, {hand="body", tool="filbert 14", pile=mud_dd, angle=0.02, angle_jitter=0.06, coverage=3.5, length={50,150}, fill=true, clip=true, load=0.9})
blend(nearM * mid, {angle=0, pressure={0.35,0.6}})

--@ chunk 28

trees2 = pile{{"lead white",1.6},{"smalt",1},{"raw umber",1.3},{"green earth",0.8},{"red earth",0.3},{"bone black",0.1}}
local n = noise{seed=111, period=25, octaves=3, kind="billow"}
local n2 = noise{seed=112, period=180, octaves=2}
treesM2 = mask(function(x,y)
  if y > 416.5 or x > 700 then return 0 end
  local lift = math.max(0, n2(x,0)+0.2) * 10 + math.max(0,n(x,0)) * 7
  local top = 408 - lift + 10*smoothstep(420,700,x)
  if x > 300 and x < 380 then top = top - 5 end
  return smoothstep(top-1, top+1, y)
end)
work(treesM2, {hand="detail", tool={kind="round", width=2.5, point=0.2}, pile=trees2, angle=0, coverage=3, length={5,16}, fill=true})

--@ chunk 29

local n = noise{seed=121, period=60, octaves=3}
local n3 = noise{seed=123, period=14, octaves=2, kind="billow"}
-- taller clumps of trees in places, a few gaps
clumps = mask(function(x,y)
  if y > 412 or x > 690 then return 0 end
  local h = 0
  h = h + 12*math.max(0, n(x,0) - 0.05)*2
  h = h + 5*math.max(0, n3(x,0))
  local top = 404 - h + 10*smoothstep(430,690,x)
  return smoothstep(top-1, top+1, y)
end)
local band = rect(0,386,705,30) - clumps - treesM2:grow(0.5)
work(band, {hand="detail", tool={kind="flat", width=3}, pile=sky_low, angle=0, coverage=2.5, length={8,25}, fill=true})
work(clumps - treesM2, {hand="detail", tool={kind="round", width=2.2, point=0.2}, pile=trees2, angle=-1.4, angle_jitter=0.5, coverage=3, length={3,9}, fill=true})

--@ chunk 30

church_p = pile{{"lead white",1.4},{"smalt",1},{"raw umber",1.4},{"red earth",0.35},{"bone black",0.15}}
tower = rect(228,377,8.5,30) + poly({{227.5,378},{232.3,349},{236.8,378}}) + rect(236,392,20,14) + poly({{235,392},{246,386},{257,392}})
work(tower, {hand="detail", tool={kind="flat", width=2}, pile=church_p, angle=-1.5708, coverage=4, length={4,12}, fill=true, clip=true})

--@ chunk 31

cl_dark = pile{{"lead white",2},{"smalt",1.2},{"red earth",0.55},{"raw umber",0.9},{"bone black",0.05}}
cl_mid = pile{{"lead white",3.2},{"smalt",1.1},{"red earth",0.6},{"raw umber",0.5}}
local n = noise{seed=131, period=180, octaves=4, warp={120,40}}
local n2 = noise{seed=132, period=70, octaves=3}
local function edge(x) return 95 + 35*n(x,0) - 25*smoothstep(550,1000,x) + 12*math.sin(x/70) end
upper = mask(function(x,y) local e = edge(x); return smoothstep(e+8, e-8, y) end)
upperCore = mask(function(x,y) local e = edge(x) - 30 + 15*n2(x,y); return smoothstep(e+8, e-8, y) end)
work(upper - upperCore, {hand="body", tool="filbert 12", pile=cl_mid, angle=0.06, angle_jitter=0.25, coverage=2.5, length={30,90}, fill=true, curve={0.3,0.2}})
work(upperCore, {hand="body", tool="filbert 14", pile=cl_dark, angle=0.06, angle_jitter=0.25, coverage=2.8, length={30,100}, fill=true, curve={0.3,0.2}})
blend(upper:grow(15), {angle=0.08, clip=true})

--@ chunk 32

local rim = upper:rim(28, 1)
blend(rim, {angle=0.1, clip=false, pressure={0.3,0.55}})
blend(rim:shrink(4), {angle=1.2, clip=false, pressure={0.2,0.4}, length={15,40}})

--@ chunk 33

cl_lit = pile{{"lead white",8},{"vermilion",0.25},{"yellow ochre",0.5},{"smalt",0.15}}
local n = noise{seed=141, period=50, octaves=3}
local rimL = upper:rim(22, 1) * mask(function(x,y) return n:at01(x,y) > 0.5 and 1 or 0 end):soften(4)
work(rimL * rect(300,0,700,300), {hand="scumble", tool="filbert 6", pile=cl_lit, angle=0.1, coverage=1.2, load=0.4, pressure={0.2,0.4}})
blend(rimL:grow(6) * rect(300,0,700,300), {angle=0.1, clip=false, pressure={0.2,0.4}})

--@ chunk 34

wr_dark = pile{{"lead white",2.6},{"smalt",1.2},{"red earth",0.55},{"raw umber",0.8}}
wr_mid = pile{{"lead white",4.5},{"smalt",1},{"red earth",0.45},{"raw umber",0.4}}
local n = noise{seed=151, period=160, octaves=3, stretch={0,4}}
local a = below(function(x) return 540 + 15*n(x,0) end)
local b = below(function(x) return 600 + 15*n(x,70) end)
work(water * a - b, {hand="body", tool={kind="flat", width=8}, pile=wr_mid, angle=0, angle_jitter=0.015, coverage=2.5, length={60,200}, pressure={0.4,0.7}, fill=true, clip=true})
work(water * b, {hand="body", tool={kind="flat", width=10}, pile=wr_dark, angle=0, angle_jitter=0.015, coverage=3, length={60,200}, pressure={0.5,0.8}, fill=true, clip=true})
local c = below(function(x) return 505 + 10*n(x,140) end)
blend(water * c, {angle=0, pressure={0.35,0.6}})

--@ chunk 35

wr_deep = pile{{"lead white",1.2},{"smalt",1.3},{"red earth",0.5},{"raw umber",1.0},{"bone black",0.08}}
local n = noise{seed=161, period=200, octaves=3, stretch={0,4}}
local b = below(function(x) return 612 + 12*n(x,0) end)
work(water * b, {hand="body", tool={kind="flat", width=10}, pile=wr_deep, angle=0, angle_jitter=0.015, coverage=3, length={60,200}, pressure={0.5,0.8}, fill=true, clip=true})
work(spit, {hand="body", tool="filbert 10", pile=mud_dd, angle=-0.3, angle_jitter=0.1, coverage=3.2, length={30,90}, fill=true, clip=true, load=0.9})
blend(water * below(function(x) return 570 + 10*n(x,30) end), {angle=0, pressure={0.35,0.6}})

--@ chunk 36
print(wait(30*60), drying(800,640), drying(300,600), drying(500,60), drying(600,90))

--@ chunk 37
print(wait(30*60), drying(800,640), drying(300,600), drying(500,60), drying(600,90), drying(950,640))

--@ chunk 38
print(wait(36*60), drying(800,640), drying(300,600), drying(500,60), drying(600,90), drying(950,640))

--@ chunk 39

refl = pile{{"lead white",2.2},{"smalt",1.1},{"red earth",0.55},{"raw umber",0.75}}
local n = noise{seed=171, period=220, octaves=3, stretch={0,5}}
nearW = water * mask(function(x,y) local t = 585 + 18*n(x,0); return smoothstep(t-20, t+25, y) end)
work(nearW, {hand="body", tool={kind="flat", width=9}, pile=refl, angle=0, angle_jitter=0.01, coverage=2.6, length={80,240}, pressure={0.5,0.8}, load=0.8, fill=true, clip=true, threshold=0.4})
blend(nearW:grow(10) * water, {angle=0, pressure={0.25,0.45}})

--@ chunk 40

-- promontories of mud into water along the diagonal edge
local lumps = poly({{640,500},{662,508},{680,522},{668,528},{640,522}}, true)
  + poly({{600,585},{628,588},{652,598},{640,604},{596,600}}, true)
  + poly({{575,628},{610,632},{626,640},{600,644},{570,640}}, true)
lumps = lumps:roughen(2,15,172,0)
work(lumps - leftland, {hand="detail", tool={kind="flat", width=3}, pile=mud_m2, angle=0, coverage=3, length={8,25}, fill=true})
-- inlets of water into the mud
local inl = poly({{648,540},{600,552},{560,558},{600,560},{636,556}}, true) + poly({{600,610},{540,618},{500,622},{545,624},{590,620}}, true)
inl = inl:roughen(1.5,12,173,0)
work(inl, {hand="detail", tool={kind="flat", width=2.5}, pile=refl, angle=0, coverage=3, length={8,25}, fill=true})

--@ chunk 41

local E2 = {{652,497},{676,514},{670,527},{645,535},{628,547},{620,565},{634,582},{652,594},{640,605},{610,613},{598,628},{616,640},{598,652},{570,668}}
local o = outline{pts=E2, open=true, char="soft", amount=0.6, seed=174}
local pts = {{-10,497}}
for _,p in ipairs(E2) do pts[#pts+1] = p end
pts[#pts+1] = {-10,668}
bankExt = poly(pts, true):roughen(2, 18, 175, 0) * rect(0,490,1000,180)
local ext = bankExt - leftland:shrink(3)
local n = noise{seed=176, period=90, octaves=2}
local lo = below(function(x) return 572 + 10*n(x,0) end)
work(ext - lo, {hand="detail", tool={kind="flat", width=4}, pile=mud_m2, angle=0, coverage=3.2, length={10,30}, fill=true})
work(ext * lo, {hand="detail", tool={kind="flat", width=4}, pile=mud_dd, angle=0, coverage=3.2, length={10,30}, fill=true})

--@ chunk 42

mudAll = (nearM + (bankExt - flatsL)) * rect(0,480,1000,190)
local n = noise{seed=181, period=130, octaves=3}
local mid = below(function(x) return nearTop(x) + 14 + 6*n(x,30) end)
local lo = below(function(x) return 572 + 14*n(x,0) end)
mud_top = pile{{"raw umber",2.2},{"red earth",0.6},{"lead white",1.0},{"smalt",0.4},{"yellow ochre",0.25}}
work(mudAll - mid, {hand="body", tool={kind="flat", width=6}, pile=mud_top, angle=0, angle_jitter=0.03, coverage=3, length={30,100}, fill=true, clip=true})
work(mudAll * mid - lo, {hand="body", tool="filbert 10", pile=mud_m2, angle=0, angle_jitter=0.04, coverage=3.2, length={40,120}, fill=true, clip=true, load=0.9})
work(mudAll * lo, {hand="body", tool="filbert 14", pile=mud_dd, angle=0, angle_jitter=0.05, coverage=3.5, length={50,150}, fill=true, clip=true, load=0.9})
blend(mudAll, {angle=0, pressure={0.3,0.5}})

--@ chunk 43

shine = pile{{"lead white",6},{"yellow ochre",0.6},{"smalt",0.3},{"red earth",0.15}}
bs = brush{kind="flat", width=3.5}
local rows = {{380,528,560,533,0.5},{120,540,300,543,0.45},{450,556,615,552,0.6},{200,562,330,566,0.35},{520,590,628,588,0.55},{60,600,170,603,0.3},{380,612,470,614,0.3},{540,640,605,638,0.5}}
for i,r in ipairs(rows) do
  bs:reload(shine, r[5])
  local mx = (r[1]+r[3])/2
  bs:stroke({{r[1],r[2]},{mx, (r[2]+r[4])/2 + rand(-1.5,1.5)},{r[3],r[4]}}, {pressure={0.25,0.45}, ramps={0.3,0.4}, swell={0.6,1,0.5}})
end

--@ chunk 44

blend(mudAll * rect(0,520,700,130), {angle=0, pressure={0.2,0.35}, tool={kind="badger", width=20}})

--@ chunk 45
print(wait(60*60), drying(300,600), drying(620,560), drying(300,540))

--@ chunk 46

hullO = outline{{190,580,"c"},{196,570,"c"},{240,575},{285,577},{330,571},{372,555,"c"},{377,566},{370,583},{352,595},{300,601},{240,600},{206,596},{192,590}, closed=true, char="firm", amount=0.4, seed=201}
hull = hullO:mask()
interiorM = poly({{197,570},{240,566},{290,566},{335,561},{370,554},{335,572},{285,578},{240,575},{197,571}}, true)
strake = hull * mask(function(x,y)
  local top = 570 + (x<285 and lerp(1,7,(x-196)/89) or lerp(7,-15,(x-285)/87))
  return (y > top and y < top + 7) and 1 or 0
end)
tar = pile{{"bone black",1.2},{"raw umber",2},{"smalt",0.4},{"lead white",0.25}}
strake_p = pile{{"red earth",1.5},{"lead white",1},{"raw umber",0.6}}
inside_p = pile{{"raw umber",1.4},{"yellow ochre",1},{"lead white",0.8},{"red earth",0.3}}
work(hull, {hand="detail", tool={kind="flat", width=4}, pile=tar, angle=0.02, coverage=4, length={10,40}, fill=true, clip=true, load=0.9})
work(interiorM - hull, {hand="detail", tool={kind="flat", width=2.5}, pile=inside_p, angle=-0.05, coverage=3.5, length={8,25}, fill=true, clip=true})

--@ chunk 47

tar2 = pile{{"bone black",1.5},{"raw umber",1.5},{"smalt",0.3}}
bowM = poly({{340,566},{366,556},{381,547},{383,552},{379,568},{372,584},{356,596},{330,598}})
sternM = poly({{189,578},{194,567},{205,569},{210,596},{194,592}})
hull2 = hull + bowM + sternM
work(hull2, {hand="detail", tool={kind="flat", width=4}, pile=tar2, angle=0.02, coverage=4, length={10,40}, fill=true, clip=true, load=1})

--@ chunk 48

post_p = pile{{"bone black",1},{"raw umber",2},{"smalt",0.3},{"lead white",0.3}}
local posts = {{742,470,4.2,34,0.03},{768,478,3.6,30,-0.04},{790,484,4.6,36,0.06},{835,493,3.2,22,0.0},{704,452,2.4,16,0.02},{716,455,2.2,13,-0.05}}
bp = brush{kind="flat", width=3}
for i,p in ipairs(posts) do
  local x,y,w,h,lean = p[1],p[2],p[3],p[4],p[5]
  bp = brush{kind="flat", width=w}
  bp:reload(post_p, 0.9)
  bp:stroke({{x + lean*h, y - h},{x, y}}, {pressure={0.9,0.85}, orient="across"})
  bp:stroke({{x + lean*h, y - h + 2},{x, y}}, {pressure={0.9,0.85}, orient="across"})
end

--@ chunk 49

post_r = pile{{"lead white",1.5},{"raw umber",1.5},{"smalt",0.6},{"red earth",0.3}}
local posts = {{742,470,3.4,20},{768,478,3,16},{790,484,3.8,20},{835,493,2.6,12},{704,452,2,9},{716,455,1.8,8}}
for i,p in ipairs(posts) do
  local x,y,w,h = p[1],p[2],p[3],p[4]
  local br = brush{kind="flat", width=w}
  br:reload(post_r, 0.6)
  br:stroke({{x, y+1},{x+0.8, y+h*0.35},{x-0.8, y+h*0.65},{x+0.5, y+h}}, {pressure={0.7,0.1}, orient="across", ramps={0.05,0.6}})
end
-- square off the tops with a touch of sky
local tb = brush{kind="flat", width=5}
for i,p in ipairs(posts) do end

--@ chunk 50

mast_p = pile{{"raw umber",2},{"bone black",0.6},{"lead white",0.5},{"red earth",0.3}}
local bm = brush{kind="round", width=3, point=0.5}
bm:reload(mast_p, 0.85)
bm:stroke({{286,572},{281,480},{276,395}}, {pressure={0.85,0.55}, ramps={0.02,0.2}})
bm:reload(mast_p, 0.7)
bm:stroke({{286,572},{281,480},{276.5,400}}, {pressure={0.75,0.5}, ramps={0.02,0.2}})
-- a stay line to the bow and stern, hairlines
local rg = brush{kind="rigger", width=1.4, point=1}
rg:reload(mast_p, 0.5)
rg:stroke({{277,402},{330,480},{378,550}}, {pressure={0.3,0.35}})
rg:reload(mast_p, 0.5)
rg:stroke({{277,402},{240,480},{198,568}}, {pressure={0.3,0.35}})

--@ chunk 51

print(wait(20*60))
inside2 = pile{{"raw umber",2},{"yellow ochre",0.6},{"red earth",0.4},{"lead white",0.35},{"bone black",0.1}}
local inM = interiorM - hull2
work(inM, {hand="detail", tool={kind="flat", width=2.5}, pile=inside2, angle=-0.05, coverage=4, length={8,25}, fill=true, clip=true, load=0.9})
-- lit stern side of interior (light from the low sun on the right shows on the far inner planks)

--@ chunk 52

print(drying(280,590), drying(280,572), drying(283,500))
local st = hull2 * mask(function(x,y)
  local top
  if x < 290 then top = lerp(572, 577.5, (x-192)/98) else top = lerp(577.5, 553, (x-290)/88) end
  return (y > top + 1.5 and y < top + 6.5) and 1 or 0
end)
strake2 = pile{{"red earth",1.6},{"lead white",0.5},{"raw umber",0.6}}
work(st, {hand="detail", tool={kind="flat", width=2}, pile=strake2, angle=0, coverage=4, length={10,30}, fill=true, clip=true})
local gw = brush{kind="round", width=1.6, point=0.7}
gun = pile{{"lead white",3},{"yellow ochre",0.8},{"raw umber",0.5}}
gw:reload(gun, 0.6)
gw:stroke({{214,572},{250,575},{290,576.5},{330,570},{360,560},{381,548}}, {pressure={0.2,0.55}})

--@ chunk 53

shadow_p = pile{{"raw umber",2},{"bone black",0.5},{"smalt",0.5},{"lead white",0.15}, medium=0.3}
local sh = poly({{150,592},{190,588},{250,598},{330,598},{372,586},{385,590},{360,603},{300,609},{220,608},{160,601}}, true):roughen(2,20,221,0) - hull2
work(sh, {hand="detail", tool={kind="flat", width=3}, pile=shadow_p, angle=0, coverage=3, length={10,40}, fill=true, clip=true})
blend(sh:grow(3) - hull2, {angle=0, tool={kind="badger", width=8}, pressure={0.2,0.35}})

--@ chunk 54

print(wait(26*60), drying(280,572), drying(280,590), drying(300,604))

--@ chunk 55

local rg = brush{kind="round", width=2.2, point=0.6}
thw = pile{{"raw umber",2},{"bone black",0.4},{"red earth",0.3},{"lead white",0.2}}
-- thwarts across the interior
for _,t in ipairs({{{232,569.5},{244,575.5}},{{300,567.5},{312,573.5}},{{338,561},{349,566}}}) do
  rg:reload(thw, 0.7)
  rg:stroke(t, {pressure={0.7,0.7}})
end
-- inside shadow under near gunwale
local ish = brush{kind="flat", width=2.2}
ish:reload(thw, 0.5)
ish:stroke({{205,573},{250,575.5},{290,576},{330,569.5},{365,558}}, {pressure={0.35,0.35}})
-- contact dark under hull
local cb = brush{kind="flat", width=3}
cb:reload(tar2, 0.8)
cb:stroke({{205,597.5},{250,600},{300,600},{340,596},{362,590}}, {pressure={0.6,0.4}})

--@ chunk 56

sandR2 = poly({{745,452},{770,446},{800,443},{850,441},{900,440},{950,441},{1010,442},{1010,489},{980,486},{950,480},{915,476},{880,470},{850,468},{820,463},{790,459},{765,456}}, true):roughen(2.5,25,231,0)
local n = noise{seed=232, period=60, octaves=2}
sr_body = pile{{"raw umber",2},{"red earth",0.5},{"lead white",1.3},{"smalt",0.55},{"yellow ochre",0.2}}
sr_top = pile{{"lead white",3},{"raw umber",1},{"red earth",0.35},{"yellow ochre",0.5},{"smalt",0.3}}
sr_foot = pile{{"raw umber",2.2},{"red earth",0.4},{"lead white",0.6},{"smalt",0.6},{"bone black",0.1}}
work(sandR2, {hand="detail", tool={kind="flat", width=4}, pile=sr_body, angle=-0.03, coverage=3.5, length={15,50}, fill=true, clip=true})
local top = sandR2 - sandR2:offset(0):shrink(0) * mask(function(x,y) return y > 441 + 0.04*(1010-x)*0 + 6 + 3*n(x,0) and 1 or 0 end)
work(top, {hand="detail", tool={kind="flat", width=2.5}, pile=sr_top, angle=-0.02, coverage=3, length={15,50}, fill=true, clip=true})
local foot = sandR2 * mask(function(x,y) local b = 489 - (1010-x)*0.13 ; return y > b - 5 + 2*n(x,50) and 1 or 0 end)
work(foot, {hand="detail", tool={kind="flat", width=2.5}, pile=sr_foot, angle=0.12, coverage=3, length={15,50}, fill=true, clip=true})

--@ chunk 57

blend(sandR2:shrink(1.5), {angle=-0.03, tool={kind="badger", width=10}, pressure={0.25,0.4}})
-- a soft reflection of the bank in the water below it
bankRefl = poly({{760,462},{800,466},{850,472},{900,478},{950,484},{1010,491},{1010,500},{960,496},{900,489},{840,481},{790,472}}, true):roughen(2,20,241,0) - sandR2
refl_b = pile{{"lead white",3},{"raw umber",1.2},{"red earth",0.35},{"smalt",0.6}, medium=0.2}
work(bankRefl, {hand="detail", tool={kind="flat", width=3}, pile=refl_b, angle=0, coverage=2, length={20,60}, fill=true, clip=true, load=0.6})
blend(bankRefl:grow(3) - sandR2, {angle=0, tool={kind="badger", width=10}, pressure={0.2,0.35}})

--@ chunk 58
print(wait(28*60), drying(850,475), drying(850,455))

--@ chunk 59

local posts = {{768,478,3.6,30,-0.04},{790,484,4.6,36,0.06},{835,493,3.2,22,0.0}}
for i,p in ipairs(posts) do
  local x,y,w,h,lean = p[1],p[2],p[3],p[4],p[5]
  local b = brush{kind="flat", width=w*0.95}
  b:reload(post_p, 0.9)
  b:stroke({{x + lean*h, y - h + 1},{x, y}}, {pressure={0.9,0.85}, orient="across"})
end
local rp = {{768,478,3,16},{790,484,3.8,20},{835,493,2.6,12}}
for i,p in ipairs(rp) do
  local x,y,w,h = p[1],p[2],p[3],p[4]
  local br = brush{kind="flat", width=w}
  br:reload(post_r, 0.6)
  br:stroke({{x, y+1},{x+0.8, y+h*0.35},{x-0.8, y+h*0.65},{x+0.5, y+h}}, {pressure={0.7,0.1}, orient="across", ramps={0.05,0.6}})
end

--@ chunk 60

local NE = {{664,416},{636,424},{614,435},{606,450},{612,468},{626,485},{644,498},{656,506}}
local nn = noise{seed=251, period=18, octaves=2}
newLeft = mask(function(x,y)
  if y < 416 or y > 506 then return 0 end
  local e = interp(NE, y) + 2.5*nn(0,y)
  return smoothstep(e+1, e-1, x)
end)
local band = rect(570,416,110,90)
local toSand = newLeft * band - leftland:grow(1)
local toWater = (band - newLeft) * leftland:shrink(0) - mudAll:grow(2)
edge_sand = pile{{"lead white",4},{"raw umber",1.3},{"red earth",0.45},{"smalt",0.4},{"yellow ochre",0.5}}
edge_water = pile{{"lead white",9},{"chrome yellow",0.6},{"yellow ochre",0.5},{"smalt",0.12},{"red earth",0.05}}
work(toWater:grow(1), {hand="detail", tool={kind="flat", width=3}, pile=edge_water, angle=0, coverage=3.5, length={8,30}, fill=true, clip=true})
work(toSand:grow(1), {hand="detail", tool={kind="flat", width=3}, pile=edge_sand, angle=0, coverage=3.5, length={8,30}, fill=true, clip=true})

--@ chunk 61

blend(newLeft:shrink(1.5) * rect(560,418,100,78), {angle=0, tool={kind="badger", width=12}, pressure={0.3,0.5}})
local blob = ellipse(650,503,6,5)
work(blob, {hand="detail", tool={kind="flat", width=2.5}, pile=mud_top, angle=0, coverage=3.5, length={4,10}, fill=true, clip=true})

--@ chunk 62

sunlight = pile{{"lead white",10},{"chrome yellow",0.9},{"vermilion",0.06}}
local g = ellipse(820,335,130,40):blur(35)
work(g, {hand="scumble", tool="filbert 10", pile=sunlight, angle=0, coverage=1.6, load=0.5, pressure={0.25,0.5}, threshold=0.2})
local core = ellipse(830,338,55,14):blur(15)
work(core, {hand="body", tool="filbert 8", pile=pile{{"lead white",10},{"chrome yellow",0.5}}, angle=0, coverage=2, length={20,60}, pressure={0.3,0.6}, threshold=0.25})
blend(ellipse(820,335,170,60):blur(20), {angle=0, pressure={0.25,0.45}, clip=false})

--@ chunk 63

glint = pile{{"lead white",10},{"chrome yellow",0.7}}
local b = brush{kind="flat", width=2.2}
local n = 0
for i=1,46 do
  local t = rand(0,1)^1.3
  local y = 422 + t*125
  local halfw = 12 + t*70
  local cx = 830 + randn(0, halfw*0.45)
  if water:at(cx, y) > 0.5 and sandR2:at(cx,y) < 0.1 and bankRefl:at(cx,y) < 0.3 then
    local len = rand(4, 10 + t*28)
    b:reload(glint, rand(0.35,0.6))
    b:stroke({{cx-len/2, y},{cx+len/2, y+rand(-0.4,0.4)}}, {pressure={rand(0.25,0.45), 0.1}, ramps={0.2,0.5}})
    n = n + 1
  end
end
print(n)

--@ chunk 64

print(drying(320,55), drying(505,75))
local warmcl = pile{{"lead white",5},{"smalt",0.8},{"red earth",0.5},{"raw umber",0.3},{"yellow ochre",0.3}}
local m = ellipse(322,56,26,14):blur(5) + ellipse(508,74,16,8):blur(4)
work(m, {hand="scumble", tool="filbert 6", pile=warmcl, angle=0.1, coverage=1.8, load=0.4, pressure={0.2,0.4}, threshold=0.2})
blend(m:grow(8), {angle=0.1, tool={kind="badger", width=14}, pressure={0.25,0.4}, clip=false})

--@ chunk 65

local fix_p = pile{{"lead white",3},{"smalt",1.1},{"red earth",0.6},{"raw umber",0.55}}
local m = ellipse(318,55,34,18):roughen(5,20,301,0) + ellipse(520,74,28,12):roughen(4,20,302,0) + ribbon({{530,78},{560,84},{575,90}}, 7)
work(m, {hand="body", tool="filbert 8", pile=fix_p, angle=0.12, coverage=3, length={15,40}, load=0.8, fill=true})
blend(m:grow(10), {angle=0.1, tool={kind="badger", width=14}, pressure={0.25,0.45}, clip=false})

--@ chunk 66

print(drying(300,50))
blend(rect(170,15,450,95):soften(15), {angle=0.08, tool={kind="badger", width=24}, pressure={0.3,0.5}, clip=false})

--@ chunk 67

local seam_p = pile{{"lead white",7},{"smalt",0.7},{"red earth",0.35},{"yellow ochre",0.35},{"raw umber",0.15}}
local band = rect(0,178,780,26):soften(6)
work(band, {hand="body", tool="filbert 9", pile=seam_p, angle=0, angle_jitter=0.02, coverage=1.8, length={60,220}, pressure={0.15,0.45}, load=0.45})
blend(rect(0,170,800,42):soften(10), {angle=0, tool={kind="badger", width=20}, pressure={0.25,0.45}, clip=false})

--@ chunk 68
print(wait(40*60), drying(300,50), drying(500,190), drying(800,340))

--@ chunk 69
print(wait(30*60), drying(300,50), drying(500,190), drying(800,340), drying(100,30))

--@ chunk 70

canopy_d = pile{{"lead white",2.4},{"smalt",1.2},{"red earth",0.55},{"raw umber",0.75}}
canopy_m = pile{{"lead white",3.6},{"smalt",1.1},{"red earth",0.55},{"raw umber",0.5}}
local n = noise{seed=401, period=220, octaves=4, warp={150,30}}
local function e1(x) return 55 + 22*n(x,0) + 15*math.sin(x/110 + 1) - 10*smoothstep(600,1000,x) end
canopy = mask(function(x,y) local e = e1(x) + 40; return smoothstep(e+10, e-10, y) end)
local core = mask(function(x,y) local e = e1(x); return smoothstep(e+8, e-8, y) end)
work(canopy - core, {hand="body", tool="filbert 12", pile=canopy_m, angle=function(x,y) return 0.04 + 0.08*n(x,y+300) end, angle_jitter=0.08, coverage=2.6, length={60,180}, pressure={0.35,0.6}, fill=true})
work(core, {hand="body", tool="filbert 14", pile=canopy_d, angle=function(x,y) return 0.04 + 0.08*n(x,y+300) end, angle_jitter=0.08, coverage=3, length={60,200}, pressure={0.4,0.7}, fill=true})
blend(canopy:grow(12), {angle=0.04, pressure={0.35,0.55}})

--@ chunk 71

bird_p = pile{{"raw umber",2},{"bone black",0.5},{"smalt",0.4},{"lead white",0.4}}
local birds = {{452,262,7},{470,254,6},{489,266,6.5},{438,272,5},{505,258,5},{520,270,4.5},{548,262,4}}
for i,b in ipairs(birds) do
  local x,y,s = b[1],b[2],b[3]
  local rg = brush{kind="rigger", width=1.5, point=1}
  rg:reload(bird_p, 0.7)
  local lift = rand(0.2,0.6)
  rg:stroke({{x - s, y - s*lift},{x - s*0.45, y - s*0.35},{x, y}}, {pressure={0.1,0.6}, ramps={0.3,0.1}})
  rg:reload(bird_p, 0.7)
  rg:stroke({{x, y},{x + s*0.45, y - s*0.4},{x + s*1.05, y - s*(lift+0.1)}}, {pressure={0.6,0.1}, ramps={0.1,0.3}})
end

--@ chunk 72

print(drying(800,570), drying(650,503))
local wt = pile{{"lead white",4.2},{"smalt",1.05},{"red earth",0.5},{"raw umber",0.5}}
local lb = brush{kind="flat", width=5}
for i=1,26 do
  local y = 548 + rand(0,44)
  local x0 = rand(600,960)
  local len = rand(60,200)
  lb:reload(wt, rand(0.3,0.5))
  local pts = {{x0,y},{x0+len/2,y+rand(-0.8,0.8)},{x0+len,y+rand(-0.8,0.8)}}
  lb:stroke(pts, {pressure={rand(0.15,0.4), rand(0.1,0.3)}, ramps={0.15,0.4}, clip=water})
end
local wl = pile{{"lead white",6},{"smalt",0.8},{"red earth",0.35},{"yellow ochre",0.35},{"raw umber",0.2}}
for i=1,14 do
  local y = 500 + rand(0,50)
  local x0 = rand(700,960)
  local len = rand(50,140)
  lb:reload(wl, rand(0.25,0.4))
  lb:stroke({{x0,y},{x0+len,y+rand(-0.6,0.6)}}, {pressure={rand(0.12,0.3), 0.08}, ramps={0.2,0.5}, clip=water})
end

--@ chunk 73

local wz = (water - mudAll:grow(1) - bankExt:grow(1)) * rect(560,490,440,120)
blend(wz, {angle=0, tool={kind="badger", width=16}, pressure={0.25,0.4}})
work(bankExt * rect(560,540,110,70) - leftland:shrink(2), {hand="detail", tool={kind="flat", width=3}, pile=mud_m2, angle=0, coverage=2.5, length={8,25}, fill=true, clip=true, load=0.7})

--@ chunk 74

local wz = (water - mudAll:grow(1) - bankExt:grow(1)) * rect(560,596,440,30):soften(8)
blend(wz, {angle=0, tool={kind="badger", width=16}, pressure={0.2,0.35}, clip=false})
local wedge = bankExt * rect(555,535,120,80) - leftland:shrink(2)
local mix = pile{{"raw umber",2.8},{"red earth",0.6},{"bone black",0.35},{"lead white",0.35},{"smalt",0.4}}
work(wedge, {hand="detail", tool={kind="flat", width=4}, pile=mix, angle=0, coverage=3.2, length={10,30}, fill=true, clip=true, load=0.9})
blend(wedge:grow(6) - water:shrink(0), {angle=0, tool={kind="badger", width=10}, pressure={0.2,0.35}})

--@ chunk 75
print(wait(48*60), drying(620,580), drying(750,610), drying(560,650))

--@ chunk 76
print(wait(36*60), drying(620,580), drying(750,610), drying(560,650), drying(600,560))

--@ chunk 77

SH = {{712,500},{692,511},{672,521},{650,531},{632,544},{623,560},{630,575},{646,589},{638,600},{616,610},{600,624},{611,638},{599,651},{590,668}}
local nn = noise{seed=501, period=12, octaves=2}
shoreR = mask(function(x,y)
  if y < 500 then return 0 end
  local e = interp(SH, y) + 1.5*nn(0,y)
  return smoothstep(e-1, e+1, x)
end)
local nl = noise{seed=502, period=40, octaves=2}
mudFix = mask(function(x,y)
  if y < 498 then return 0 end
  local l = 420 + 25*nl(0,y)
  return smoothstep(l-3, l+3, x)
end) * (-shoreR) * rect(0,490,1000,180) * (mudAll + bankExt + leftland*rect(0,500,1000,170)) - hull2:grow(4)
local mid = below(function(x) return nearTop(x) + 14 end)
local lo = below(function(x) return 572 end)
work(mudFix - mid, {hand="body", tool={kind="flat", width=6}, pile=mud_top, angle=0, angle_jitter=0.03, coverage=3, length={30,100}, fill=true, clip=true})
work(mudFix * mid - lo, {hand="body", tool="filbert 10", pile=mud_m2, angle=0, angle_jitter=0.04, coverage=3.2, length={40,120}, fill=true, clip=true, load=0.9})
work(mudFix * lo, {hand="body", tool="filbert 12", pile=mud_dd, angle=0, angle_jitter=0.05, coverage=3.5, length={40,120}, fill=true, clip=true, load=0.9})
blend(mudFix, {angle=0, pressure={0.3,0.5}})

--@ chunk 78

blend(rect(360,505,120,160), {angle=0, tool={kind="badger", width=16}, pressure={0.3,0.5}, clip=false, length={40,90}})
blend((-shoreR) * rect(430,555,200,40), {angle=0.9, tool={kind="badger", width=14}, pressure={0.25,0.45}, length={20,40}})

--@ chunk 79

print(drying(500,560))
blend((-shoreR) * rect(300,540,340,70), {angle=0, tool={kind="badger", width=24}, pressure={0.3,0.5}, length={60,160}})

--@ chunk 80
print(wait(60*60), drying(500,560), drying(370,570), drying(300,590))

--@ chunk 81

-- the patch of mud where the bow smear lies, from the bow out to x 460
local area = rect(376,540,90,70):soften(3) - hull2:grow(1)
work(area * below(function(x) return 572 end), {hand="detail", tool={kind="flat", width=4}, pile=mud_dd, angle=0, coverage=3.2, length={15,40}, fill=true, clip=true, load=0.9})
work(area - below(function(x) return 572 end), {hand="detail", tool={kind="flat", width=4}, pile=mud_m2, angle=0, coverage=3.2, length={15,40}, fill=true, clip=true, load=0.9})
-- hull restated
work(hull2, {hand="detail", tool={kind="flat", width=4}, pile=tar2, angle=0.02, coverage=4, length={10,40}, fill=true, clip=true, load=1})

--@ chunk 82

blend(rect(378,545,110,70) - hull2:grow(3), {angle=0, tool={kind="badger", width=18}, pressure={0.3,0.5}, length={40,100}})
blend(rect(378,560,110,25) - hull2:grow(3), {angle=1.5708, tool={kind="badger", width=10}, pressure={0.25,0.4}, length={15,30}})
blend(rect(378,562,110,20) - hull2:grow(3), {angle=0, tool={kind="badger", width=18}, pressure={0.25,0.4}, length={40,100}})

--@ chunk 83

local nn = noise{seed=601, period=30, octaves=2}
local patch = mask(function(x,y)
  if x < 280 or x > 440 then return 0 end
  local t = lerp(nearTop(280), 497.5, smoothstep(280,430,x)) + 0.8*nn(x,0)
  return smoothstep(t-0.8, t+0.8, y) * (y < 509 and 1 or 0)
end)
work(patch, {hand="detail", tool={kind="flat", width=3}, pile=mud_top, angle=0, coverage=3.5, length={15,40}, fill=true, clip=true, load=0.8})

--@ chunk 84

local m = mask(function(x,y)
  if x < 270 or x > 470 then return 0 end
  local t = lerp(nearTop(280), 497.5, smoothstep(280,430,x)) + 1.5
  return (y > t and y < 520) and 1 or 0
end)
blend(m, {angle=0, tool={kind="badger", width=12}, pressure={0.3,0.5}, length={30,80}})

--@ chunk 85
print(wait(50*60), drying(280,590), drying(400,505), drying(300,580))

--@ chunk 86
print(wait(40*60), drying(280,590), drying(300,580), drying(360,575))

--@ chunk 87
print(wait(72*60), drying(280,590), drying(300,580), drying(360,575))

--@ chunk 88

local st = hull2 * mask(function(x,y)
  local top
  if x < 290 then top = lerp(572, 577.5, (x-192)/98) else top = lerp(577.5, 553, (x-290)/88) end
  return (y > top + 2 and y < top + 6) and 1 or 0
end)
work(st, {hand="detail", tool={kind="flat", width=2}, pile=strake2, angle=0, coverage=4, length={10,30}, fill=true, clip=true, load=0.8})
local gw = brush{kind="round", width=1.8, point=0.6}
gw:reload(gun, 0.65)
gw:stroke({{200,571},{250,574.5},{290,576},{330,569.5},{360,559.5},{380,548.5}}, {pressure={0.3,0.6}})
-- warm lit highlight on the bow top from the low sun
local hl = brush{kind="round", width=1.6, point=0.7}
hl:reload(pile{{"lead white",5},{"chrome yellow",0.5},{"vermilion",0.1}}, 0.55)
hl:stroke({{352,563},{368,555},{381,548}}, {pressure={0.2,0.5}})

--@ chunk 89

waterFix = shoreR * rect(560,560,450,110) - spit:grow(1)
local n = noise{seed=701, period=200, octaves=2, stretch={0,4}}
local wa = pile{{"lead white",3.3},{"smalt",1.15},{"red earth",0.52},{"raw umber",0.65}}
local wb = pile{{"lead white",2.6},{"smalt",1.2},{"red earth",0.55},{"raw umber",0.8}}
local d = below(function(x) return 612 + 10*n(x,0) end)
work(waterFix - d, {hand="body", tool={kind="flat", width=9}, pile=wa, angle=0, angle_jitter=0.01, coverage=3, length={80,240}, pressure={0.5,0.8}, fill=true, clip=true})
work(waterFix * d, {hand="body", tool={kind="flat", width=10}, pile=wb, angle=0, angle_jitter=0.01, coverage=3, length={80,240}, pressure={0.5,0.8}, fill=true, clip=true})
blend(waterFix, {angle=0, pressure={0.3,0.5}})

--@ chunk 90

local band = shoreR * rect(560,530,450,45) - sandR2 - spit
local tr1 = pile{{"lead white",5},{"smalt",0.9},{"red earth",0.45},{"raw umber",0.35},{"yellow ochre",0.15}}
local tr2 = pile{{"lead white",4},{"smalt",1.0},{"red earth",0.5},{"raw umber",0.5}}
work(band * rect(560,530,450,22), {hand="body", tool={kind="flat", width=7}, pile=tr1, angle=0, angle_jitter=0.01, coverage=2.8, length={80,240}, pressure={0.4,0.7}, fill=true, clip=true})
work(band * rect(560,552,450,23), {hand="body", tool={kind="flat", width=7}, pile=tr2, angle=0, angle_jitter=0.01, coverage=2.8, length={80,240}, pressure={0.4,0.7}, fill=true, clip=true})
blend(shoreR * rect(560,528,450,70) - spit, {angle=0, pressure={0.3,0.5}})
blend(shoreR * rect(560,528,450,70) - spit, {angle=0, tool={kind="badger", width=24}, pressure={0.25,0.4}})

--@ chunk 91

local cl = shoreR - sandR2:grow(2) - bankRefl
local tr0 = pile{{"lead white",6},{"smalt",0.85},{"red earth",0.42},{"yellow ochre",0.25},{"raw umber",0.3},{"chrome yellow",0.1}}
work(rect(620,508,380,30), {hand="body", tool={kind="flat", width=6}, pile=tr0, angle=0, angle_jitter=0.01, coverage=2.2, length={80,220}, pressure={0.25,0.55}, load_at=function(x,y) return lerp(0.25,0.8,smoothstep(505,535,y)) end, clip=cl})
blend(cl * rect(600,500,400,55), {angle=0, tool={kind="badger", width=24}, pressure={0.25,0.45}})

--@ chunk 92

local cl = shoreR - sandR2:grow(2) - bankRefl - spit
blend(cl * rect(600,535,400,45), {angle=0, tool={kind="badger", width=24}, pressure={0.25,0.45}})
blend(cl * rect(600,545,400,20), {angle=0, tool={kind="badger", width=24}, pressure={0.2,0.35}})

--@ chunk 93

print(drying(390,560), drying(300,620))
local rope = brush{kind="rigger", width=1.3, point=1}
rope:reload(pile{{"raw umber",2},{"bone black",0.4},{"lead white",0.6}}, 0.6)
rope:stroke({{381,552},{400,566},{420,578},{445,586},{462,590}}, {pressure={0.4,0.3}, shake=0.5})
-- a small anchor stake where the rope ends
local sb = brush{kind="flat", width=2.2}
sb:reload(post_p, 0.8)
sb:stroke({{463,582},{462.5,592}}, {pressure={0.85,0.8}, orient="across"})
-- a little wet sheen under the stake and along a runnel
local sh = brush{kind="round", width=2.5, point=0.5}
sh:reload(pile{{"lead white",4},{"raw umber",0.9},{"smalt",0.35},{"red earth",0.3}, medium=0.2}, 0.35)
sh:stroke({{440,596},{470,597},{500,599.5}}, {pressure={0.1,0.35}, ramps={0.4,0.5}})
sh:reload(pile{{"lead white",4},{"raw umber",0.9},{"smalt",0.35},{"red earth",0.3}, medium=0.2}, 0.3)
sh:stroke({{60,628},{110,631},{150,633}}, {pressure={0.08,0.3}, ramps={0.4,0.5}})

--@ chunk 94

print(wait(48*60), drying(420,578), drying(470,597))

--@ chunk 95

local dk = pile{{"bone black",1},{"raw umber",1.5}}
local rope = brush{kind="round", width=2, point=0.8}
rope:reload(dk, 0.8)
rope:stroke({{381,552},{400,566},{420,578},{445,586},{462,590}}, {pressure={0.55,0.45}})
local sb = brush{kind="flat", width=3}
sb:reload(dk, 0.9)
sb:stroke({{463,580},{462.5,593}}, {pressure={0.95,0.9}, orient="across"})
-- knock back the hard sheen line with mud, then a softer sheen
local mb = brush{kind="flat", width=4}
mb:reload(mud_dd, 0.7)
mb:stroke({{436,597},{470,597.5},{505,599.5}}, {pressure={0.7,0.7}})
mb:reload(mud_dd, 0.7)
mb:stroke({{56,628},{110,631},{154,633}}, {pressure={0.7,0.7}})

--@ chunk 96
print(wait(60*24*3)); for _,p in ipairs{{100,600},{300,580},{560,470},{800,600},{700,450},{400,100}} do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 97
local mud = poly({{0,548},{300,545},{600,548},{606,580},{588,620},{578,667},{0,667}}, true)
local boat = poly({{180,540},{395,535},{395,615},{180,615}}, true):soften(6)
local grad = mask(function(x,y) return smoothstep(548,660,y) end)
local m = (mud - boat) * grad
local gl = pile{{"raw umber",2},{"bone black",0.5},{"smalt",0.3}, medium=0.7}
work(m, {hand="glaze", pile=gl, angle=function(x,y) return 0.03*math.sin(x/140) end, coverage=1.6, clip=true, pressure={0.2,0.35}})

--@ chunk 98
local function lens(cx, cy, rx, ry, tilt, seed)
  local pts = {}
  local n = 14
  for i = 0, n - 1 do
    local a = i / n * 2 * math.pi
    local r = 1 + 0.18 * math.sin(3 * a + seed) + 0.1 * math.cos(5 * a + seed * 2)
    local x = cx + rx * math.cos(a) * r
    local y = cy + ry * math.sin(a) * (1 + 0.3 * math.sin(2*a + seed)) + tilt * (x - cx)
    pts[#pts + 1] = {x, y}
  end
  return poly(pts, true)
end
puddles = {
  lens(80, 626, 58, 4.5, -0.02, 1),
  lens(470, 618, 44, 3.5, 0.015, 2),
  lens(318, 645, 30, 3, 0.0, 3),
  lens(525, 652, 26, 2.6, -0.03, 4),
}
local all = puddles[1] + puddles[2] + puddles[3] + puddles[4]
local near = pile{{"lead white",4},{"smalt",1.0},{"red earth",0.5},{"raw umber",0.55}}
work(all, {hand="body", tool="filbert 4", pile=near, angle=0, coverage=2.2, length={10,40}, clip=true, pressure={0.4,0.7}, fill=true})

--@ chunk 99
print(wait(60*30)); print(drying(80,626), drying(470,618))

--@ chunk 100
print(wait(60*24)); print(drying(80,626), drying(470,618))

--@ chunk 101
print(wait(60*24*3)); print(drying(80,626), drying(470,618))

--@ chunk 102
local cys = {626, 618, 645, 652}
local all = puddles[1] + puddles[2] + puddles[3] + puddles[4]
local g1 = pile{{"raw umber",1.5},{"smalt",0.8},{"red earth",0.2}, medium=0.8}
work(all:grow(1), {hand="glaze", tool="filbert 6", pile=g1, angle=0, coverage=1.3, length={20,60}, clip=true, pressure={0.2,0.3}})
local lower
for i, p in ipairs(puddles) do
  local c = cys[i]
  local m = p * mask(function(x, y) return smoothstep(c - 1, c + 3, y) end)
  lower = lower and (lower + m) or m
end
work(lower, {hand="glaze", tool="filbert 6", pile=g1, angle=0, coverage=1.2, length={20,50}, clip=true, pressure={0.2,0.3}})

--@ chunk 103
local cys = {626, 618, 645, 652}
local dark = pile{{"raw umber",2.5},{"bone black",0.6},{"red earth",0.3},{"lead white",0.2}}
local mudc = pile{{"raw umber",2.8},{"red earth",0.6},{"bone black",0.45},{"lead white",0.35},{"smalt",0.4}}
local all = puddles[1] + puddles[2] + puddles[3] + puddles[4]
local n = lose(all, {pile=mudc, where=0.6, tool="filbert 3", reach={4,5}, load=0.3, angle=0.05})
print("lose", n)
local lip
for i, p in ipairs(puddles) do
  local c = cys[i]
  local m = p:grow(0.8) - p:shrink(0.6)
  m = m * mask(function(x, y) return 1 - smoothstep(c - 2, c, y) end)
  lip = lip and (lip + m) or m
end
work(lip, {hand="detail", tool={kind="round", width=1.2, point=0.6}, pile=dark, angle=0, coverage=1.5, clip=true, pressure={0.3,0.5}})

--@ chunk 104
print(wait(60*24*3))
local cys = {626, 618, 645, 652}
local cx = {80, 470, 318, 525}
local rx = {58, 44, 30, 26}
local gl = pile{{"lead white",6},{"smalt",0.7},{"red earth",0.3},{"yellow ochre",0.3},{"raw umber",0.25}}
local b = brush{kind="round", width=1.6, point=0.7}
for i = 1, 4 do
  b:reload(gl, 0.45)
  local x0 = cx[i] - rx[i] * rand(0.5, 0.7)
  local x1 = cx[i] + rx[i] * rand(0.2, 0.55)
  local y = cys[i] - 0.8
  b:stroke({{x0, y + rand(-0.3,0.3)}, {(x0+x1)/2, y + rand(-0.4,0.4)}, {x1, y}}, {pressure={0.5, 0.15}, ramps={0.15, 0.5}, clip=puddles[i]:shrink(0.5), shake=0.6})
end

--@ chunk 105
print(drying(280,590), drying(100,620), wait(0))

--@ chunk 106
print(hull2 ~= nil, hull2:area())

--@ chunk 107
-- sink the keel into the mud: a ragged lip of mud over the hull's lower edge
local n = noise{seed=611, period=14, octaves=2}
local function yb(x)
  -- hull's lower edge, roughly
  if x < 206 then return lerp(590, 596, (x-192)/14) end
  if x < 240 then return lerp(596, 600, (x-206)/34) end
  if x < 300 then return lerp(600, 601, (x-240)/60) end
  if x < 352 then return lerp(601, 595, (x-300)/52) end
  return lerp(595, 583, (x-352)/18)
end
lipM = hull2:grow(1.5) * mask(function(x,y)
  if x < 188 or x > 368 then return 0 end
  local t = yb(x) - 4.5 - 2.2*n(x, 0)
  return y > t and 1 or 0 end)
local lipP = pile{{"raw umber",2.6},{"red earth",0.55},{"bone black",0.3},{"lead white",0.5},{"smalt",0.35}}
work(lipM, {hand="detail", tool={kind="flat", width=2.5}, pile=lipP, angle=0.02, coverage=3, length={6,18}, fill=true, clip=true, load=0.8})
-- a dark crease where hull meets mud
local cr = brush{kind="round", width=1.4, point=0.5}
local pts = {}
for x = 196, 364, 6 do pts[#pts+1] = {x, yb(x) - 4.5 - 2.2*n(x,0) - 0.3} end
cr:reload(pile{{"bone black",1},{"raw umber",1.5}}, 0.6)
cr:stroke(pts, {pressure={0.5,0.35}, shake=0.4})

--@ chunk 108
local lp2 = pile{{"raw umber",2.2},{"bone black",0.35},{"smalt",0.5},{"lead white",0.55},{"red earth",0.15}}
work(lipM:grow(1), {hand="detail", tool={kind="flat", width=2.5}, pile=lp2, angle=0.02, coverage=3.5, length={6,18}, fill=true, clip=true, load=0.9})

--@ chunk 109
print(wait(24*60), drying(280,597), drying(220,594))

--@ chunk 110
print(wait(24*60), drying(280,597), drying(220,594))

--@ chunk 111
local lp3 = pile{{"raw umber",2.2},{"bone black",0.6},{"smalt",0.45},{"lead white",0.3},{"red earth",0.1}}
work(lipM:grow(1.5), {hand="detail", tool={kind="flat", width=2.5}, pile=lp3, angle=0.02, coverage=3.5, length={6,18}, fill=true, edge="soft", load=0.9})

--@ chunk 112
local shA = poly({{140,590},{190,586},{250,596},{330,596},{372,584},{392,588},{368,606},{300,613},{220,612},{150,604}}, true):roughen(3,25,640,0):soften(4) - hull2:shrink(3)
local g = pile{{"raw umber",2},{"red earth",0.35},{"yellow ochre",0.1}, medium=0.75}
work(shA, {hand="glaze", tool={kind="filbert", width=10, stiffness=0.3}, pile=g, angle=0.02, coverage=1.4, pressure={0.2,0.35}, edge="lost"})

--@ chunk 113
print(wait(7*24*60), drying(280,597))

--@ chunk 114
print(wait(0), drying(790,500), drying(80,626), #puddles, puddles[1]:area())

--@ chunk 115

local pz = pile{{"lead white",3},{"smalt",1.0},{"red earth",0.45},{"raw umber",0.75}}
for i = 1, 4 do
  work(puddles[i]:shrink(1.2), {hand="detail", tool={kind="flat", width=3}, pile=pz, angle=0.01, coverage=3, length={8,24}, fill=true, clip=true, load=0.8, pressure={0.6,0.8}})
end

--@ chunk 116

local cys = {626, 618, 645, 652}
local pd = pile{{"lead white",1.3},{"smalt",0.9},{"raw umber",1.3},{"red earth",0.35}}
for i = 1, 4 do
  local c = cys[i]
  local lo = puddles[i]:shrink(1.2) * below(function(x) return c - 0.3 + 0.8*math.sin(x/11 + i) end)
  work(lo, {hand="detail", tool={kind="flat", width=2.5}, pile=pd, angle=0.01, coverage=2.5, length={6,20}, fill=true, clip=true, load=0.8, pressure={0.5,0.7}})
  blend(puddles[i]:shrink(0.8), {angle=0, tool={kind="badger", width=6}, pressure={0.2,0.35}, coverage=1})
end

--@ chunk 117

local refl = pile{{"bone black",0.8},{"raw umber",2},{"smalt",0.5},{"lead white",0.7}}
local rp = {{742,470,3.4,22},{768,478,3.0,17},{790,484,3.8,21},{835,493,2.6,13},{704,452,2.0,10},{716,455,1.8,9}}
for i,p in ipairs(rp) do
  local x,y,w,h = p[1],p[2],p[3],p[4]
  local br = brush{kind="flat", width=w*0.9}
  br:reload(refl, 0.75)
  br:stroke({{x+0.2, y+0.5},{x-0.5, y+h*0.3},{x+0.4, y+h*0.55}}, {pressure={0.85,0.6}, orient="across", ramps={0.02,0.3}})
  br:reload(refl, 0.45)
  br:stroke({{x+0.6, y+h*0.66},{x-0.3, y+h*0.82},{x+0.5, y+h}}, {pressure={0.55,0.05}, orient="across", ramps={0.05,0.7}})
end

--@ chunk 118
print(wait(5*60), drying(200,626), drying(470,618))

--@ chunk 119

local cys = {626, 618, 645, 652}
local pd = pile{{"lead white",0.9},{"smalt",0.9},{"raw umber",1.5},{"red earth",0.35}, medium=0.3}
for i = 1, 4 do
  local c = cys[i]
  local lo = puddles[i]:shrink(0.8) * below(function(x) return c - 0.8 + 0.9*math.sin(x/9 + 2*i) end)
  work(lo, {hand="detail", tool={kind="flat", width=2.2}, pile=pd, angle=0.01, coverage=2, length={6,18}, fill=true, clip=true, load=0.7, pressure={0.4,0.6}})
end

--@ chunk 120

local mud = pile{{"raw umber",2.6},{"red earth",0.55},{"bone black",0.35},{"lead white",0.45},{"smalt",0.35}}
local cys = {626, 618, 645, 652}
local cx = {86, 470, 322, 525}
local rx = {58, 42, 30, 26}
local b = brush{kind="filbert", width=3}
for i = 1, 4 do
  local c, x0, r = cys[i], cx[i], rx[i]
  -- left end: mud crosses in from outside
  b:reload(mud, 0.6)
  b:stroke({{x0 - r - 6, c + rand(-0.5,0.5)}, {x0 - r + r*rand(0.15,0.3), c + rand(-1,0.5)}}, {pressure={0.6,0.05}, ramps={0.05,0.6}, shake=0.8})
  -- right end
  b:reload(mud, 0.6)
  b:stroke({{x0 + r + 6, c + rand(-0.5,0.5)}, {x0 + r - r*rand(0.2,0.35), c + rand(-0.5,1)}}, {pressure={0.6,0.05}, ramps={0.05,0.6}, shake=0.8})
  -- a bite into the top edge, a spit of mud
  local bx = x0 + r*rand(-0.4,0.3)
  b:reload(mud, 0.5)
  b:stroke({{bx - 8, c - 2.5}, {bx, c - 0.8 + rand(-0.3,0.3)}, {bx + 7, c - 2.2}}, {pressure={0.1,0.1}, swell={0.3,1,0.3}, shake=0.6})
end
