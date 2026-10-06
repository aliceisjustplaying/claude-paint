-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box giverny
--@ engine 5

--@ chunk 1
canvas{size=1000, aspect=1.0, linen={16,14}, seed=1907,
 ground={{pile={{"lead white",6},{"yellow ochre",0.08}}, um=120, apply="knife", texture=0.3},
         {pile={{"lead white",6},{"zinc white",1}}, um=60, apply="brush"}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2

water1 = pile({{"lead white",3},{"ultramarine blue",1},{"cobalt violet",1},{"viridian",0.3}}, {turps=0.6, name="water1"})

--@ chunk 3
print(water1)
water1 = pile{{"lead white",3},{"ultramarine blue",1},{"cobalt violet",1},{"viridian",0.3}, turps=0.6, name="water1"}
willow1 = pile{{"ultramarine blue",2},{"viridian",1.2},{"carmine lake",0.4},{"lead white",0.3}, turps=0.5, name="willow1"}
sky1 = pile{{"lead white",5},{"pale cadmium",0.5},{"rose madder",0.25}, turps=0.5, name="sky1"}
print(water1) print(willow1) print(sky1)

--@ chunk 4

local m = poly({{-10,-10},{330,-10},{260,180},{120,330},{-10,420}}, true)
local xs = uneven(22, -10, 400, 0.6, 0.4, 11)
for i,x in ipairs(xs) do
  local t = (x+10)/410
  local L = lerp(700, 260, t) + rand(-120,120)
  local w = rand(18,45)
  local dx = rand(-25,25)
  m = m + ribbon({{x,-20},{x+dx*0.5, L*0.5},{x+dx, L}}, {w, w*0.9, w*0.3})
end
willowL = m:roughen(6, 30, 3)
local xs2 = uneven(8, 860, 1010, 0.6, 0.4, 12)
local r = nil
for i,x in ipairs(xs2) do
  local L = lerp(180, 420, (x-860)/150) + rand(-60,60)
  local w = rand(16,36)
  local rb = ribbon({{x,-20},{x+rand(-15,15), L}}, {w, w*0.35})
  r = r and (r + rb) or rb
end
willowR = r:roughen(5, 30, 4)
skyM = poly({{430,-10},{870,-10},{850,200},{760,420},{600,600},{420,760},{300,900},{180,1010},{60,1010},{250,760},{420,520},{460,300}}, true):roughen(25,120,5)
print(willowL:area(), willowR:area(), skyM:area())

--@ chunk 5

local rest = everywhere() - willowL - willowR - skyM
work(rest, {hand="broad", pile=water1, angle=math.pi/2, angle_jitter=0.15, coverage=1.3, seed=21})
work(willowL + willowR, {hand="broad", pile=willow1, angle=math.pi/2, angle_jitter=0.08, coverage=1.4, tool="filbert 14", seed=22})
work(skyM - willowL - willowR, {hand="broad", pile=sky1, angle=function(x,y) return 0.1 end, angle_jitter=0.2, coverage=1.3, seed=23})

--@ chunk 6

water2 = pile{{"lead white",3},{"cobalt blue",0.8},{"cobalt violet",1.2},{"viridian",0.4}, turps=0.5, name="water2"}
willow2 = pile{{"viridian",1.5},{"ultramarine blue",1},{"yellow ochre",0.3},{"carmine lake",0.3},{"lead white",0.4}, turps=0.45, name="willow2"}
sky2 = pile{{"lead white",5},{"cadmium yellow",0.35},{"rose madder",0.4},{"cobalt violet",0.2}, turps=0.45, name="sky2"}
local rest = everywhere() - willowL - willowR - skyM
work(rest, {hand="broad", tool="filbert 28", pile=water2, angle=math.pi/2, angle_jitter=0.2, coverage=1.6, fill=true, seed=31})
work(willowL + willowR, {hand="broad", tool="filbert 20", piles={{willow2, 1},{willow1, function(x,y) return 0.6 end}}, angle=math.pi/2, angle_jitter=0.1, coverage=1.6, fill=true, seed=32})
work(skyM - willowL - willowR, {hand="broad", tool="filbert 28", piles={{sky1,1},{sky2,1}}, angle=0.05, angle_jitter=0.25, coverage=1.6, fill=true, seed=33})

--@ chunk 7
print(wait(24*60)); print(drying(200,200), drying(600,300), drying(800,800))

--@ chunk 8
print(wait(2*24*60)); print(drying(200,200), drying(600,300), drying(800,800), drying(950,100))

--@ chunk 9
print(wait(2*24*60)); print(drying(200,200), drying(600,300), drying(800,800), drying(950,100))

--@ chunk 10

deep = pile{{"ultramarine blue",2},{"cobalt violet",1.2},{"viridian",0.5},{"lead white",1}, medium=0.15, name="deep"}
midw = pile{{"cobalt blue",1.2},{"cobalt violet",1},{"lead white",2.5},{"viridian",0.2}, medium=0.1, name="midw"}
lilac = pile{{"lead white",3},{"cobalt violet",1.2},{"rose madder",0.3},{"cobalt blue",0.3}, medium=0.1, name="lilac"}

--@ chunk 11
deep:add{{"carmine lake",0.25},{"viridian",0.5},{"ultramarine blue",0.5}}
print(deep)

--@ chunk 12

local nz = noise{seed=5, period=220, octaves=3}
local function dband(x,y)
  local cx = lerp(650, 170, y/1000)
  return math.abs(x-cx)
end
local water = everywhere() - willowL - willowR - skyM:shrink(30)
work(water, {hand="body", tool="filbert 14", piles={
   {deep, function(x,y) return clamp((dband(x,y)-150)/250,0,1)*0.8 + 0.2*nz:at01(x,y) + (y>600 and 0.3 or 0) end},
   {midw, function(x,y) return 0.5 end},
   {lilac, function(x,y) return clamp(1-(dband(x,y)-100)/200,0,1)*0.8 end}},
   angle=math.pi/2, angle_jitter=0.18, length={30,90}, coverage=1.3, fill=true, edge="soft", seed=41})

--@ chunk 13

gold = pile{{"lead white",4},{"pale cadmium",0.6},{"deep cadmium",0.08}, medium=0.1, name="gold"}
rose = pile{{"lead white",4},{"rose madder",0.5},{"vermilion",0.08},{"cadmium yellow",0.1}, medium=0.1, name="rose"}
local n1 = noise{seed=8, period=160, octaves=3, stretch={0, 3.5}}
local n2 = noise{seed=9, period=200, octaves=3, stretch={0, 3}}
work(skyM:grow(10), {hand="body", tool="filbert 16", piles={
   {gold, function(x,y) return clamp(n1:at01(x,y)*1.4-0.2,0,1) + (y<300 and 0.4 or 0) end},
   {rose, function(x,y) return clamp(n2:at01(x,y)*1.3-0.3,0,1) end},
   {lilac, function(x,y) return clamp(0.8-n1:at01(x,y)*1.5,0,1)*0.9 end}},
   angle=0.0, angle_jitter=0.15, length={40,110}, coverage=1.4, fill=true, edge="loose", seed=42})

--@ chunk 14

wdark = pile{{"viridian",1.6},{"ultramarine blue",1.4},{"carmine lake",0.35},{"yellow ochre",0.2},{"lead white",0.25}, medium=0.2, name="wdark"}
wgreen = pile{{"viridian",1},{"cadmium yellow",0.5},{"ultramarine blue",0.3},{"lead white",0.8},{"yellow ochre",0.2}, medium=0.15, name="wgreen"}
bf = brush{kind="filbert", width=12, stiffness=0.5}
local xs = uneven(30, -5, 420, 0.5, 0.5, 51)
for i,x in ipairs(xs) do
  local t = (x+5)/425
  local L = lerp(720, 330, t) + rand(-130,110)
  local dx = rand(-30,30)
  bf:reload(wdark, rand(0.7,0.95))
  bf:gesture({{x, -10, 0.9},{x+dx*0.3, L*0.35, 0.85},{x+dx*0.7, L*0.75, 0.6},{x+dx, L, 0.05}}, {wobble=6, orient="along"})
end

--@ chunk 15

local reg = rect(-10,-10,460,780)
blend(reg, {angle=math.pi/2, clip=false})

--@ chunk 16
print(wait(3*24*60)); for _,p in ipairs({{200,200},{600,300},{800,800},{300,800},{100,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 17
print(wait(4*24*60)); for _,p in ipairs({{200,200},{600,300},{800,800},{300,800},{100,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 18

padG = pile{{"viridian",1},{"cadmium yellow",0.9},{"lead white",1.2},{"yellow ochre",0.2}, medium=0.05, name="padG"}
padY = pile{{"pale cadmium",1},{"viridian",0.35},{"lead white",1.5}, medium=0.05, name="padY"}
padB = pile{{"viridian",1},{"cobalt blue",0.8},{"lead white",1.2},{"cobalt violet",0.2}, medium=0.05, name="padB"}
padV = pile{{"cobalt violet",1},{"viridian",0.5},{"lead white",1.2},{"ultramarine blue",0.2}, medium=0.05, name="padV"}
padO = pile{{"cadmium yellow",0.6},{"vermilion",0.15},{"viridian",0.25},{"lead white",1.5}, medium=0.05, name="padO"}

--@ chunk 19

function padsize(y)
  local t = clamp(y/1000,0,1)
  local w = lerp(22, 115, t^1.25)
  return w, w*lerp(0.22, 0.40, t)
end
function pickpile(weights)
  local s=0 for _,w in ipairs(weights) do s=s+w[2] end
  local r=rand(0,s)
  for _,w in ipairs(weights) do r=r-w[2]; if r<=0 then return w[1] end end
  return weights[1][1]
end
function pad(cx, cy, weights, scale)
  local w,h = padsize(cy); w=w*(scale or 1)*rand(0.75,1.25); h=h*(scale or 1)*rand(0.8,1.15)
  local bw = clamp(h*0.55, 3, 26)
  local b = brush{kind="filbert", width=bw, stiffness=0.55}
  local p = pickpile(weights)
  b:reload(p, rand(0.6,0.9))
  local rows = math.max(2, math.floor(h/bw*1.6+0.5))
  for i=1,rows do
    local f = (i-0.5)/rows*2-1  -- -1..1
    local half = w/2*math.sqrt(math.max(0.05,1-f*f))*rand(0.85,1.1)
    local yy = cy + f*h*0.4
    local tilt = rand(-0.04,0.04)*w
    if math.random()<0.25 then b:reload(pickpile(weights), 0.6) end
    b:stroke({{cx-half, yy-tilt},{cx+half, yy+tilt}}, {pressure={rand(0.6,0.9), rand(0.4,0.8)}, ramps={0.1,0.3}})
  end
  return w,h
end
function island(cx, cy, rx, ry, n, weights, seed, scale)
  local out = {}
  for i=1,n do
    local a = rand(0, 2*math.pi); local r = math.sqrt(rand(0,1))
    local x = cx + math.cos(a)*rx*r; local y = cy + math.sin(a)*ry*r
    out[#out+1] = {x,y}
  end
  table.sort(out, function(a,b) return a[2]<b[2] end)
  for _,pt in ipairs(out) do pad(pt[1], pt[2], weights, scale) end
  return out
end
farW = {{padB,2},{padV,2},{padG,1.5},{padY,1},{padO,0.7}}
islA = island(560, 130, 120, 22, 16, farW, 1)

--@ chunk 20

islA2 = island(600, 120, 150, 18, 14, farW, 2, 0.9)
islB = island(720, 300, 170, 38, 22, {{padB,2},{padV,1.5},{padG,2},{padY,1.2},{padO,0.8}}, 3)
islC = island(440, 560, 210, 50, 24, {{padB,1.5},{padV,1},{padG,2.5},{padY,1.5},{padO,1}}, 4)

--@ chunk 21

function padsize(y)
  local t = clamp(y/1000,0,1)
  local w = lerp(32, 165, t^1.2)
  return w, w*lerp(0.2, 0.38, t)
end
island(580, 125, 150, 16, 12, farW, 5)
island(720, 305, 170, 30, 16, {{padB,2},{padV,1.5},{padG,2},{padY,1.2},{padO,0.8}}, 6)
island(430, 565, 220, 45, 18, {{padB,1.5},{padV,1},{padG,2.5},{padY,1.5},{padO,1}}, 7)

--@ chunk 22

nearW = {{padB,1.5},{padV,1},{padG,2.5},{padY,1.2},{padO,1}}
island(740, 860, 270, 85, 20, nearW, 8)
island(150, 790, 150, 55, 11, {{padB,1},{padV,1.5},{padG,2},{padY,1},{padO,0.8}}, 9)
island(920, 560, 100, 40, 7, {{padB,2},{padV,1.5},{padG,1.5},{padY,0.8},{padO,0.5}}, 10)

--@ chunk 23
print(wait(24*60)); print(drying(700,880), drying(800,600))

--@ chunk 24
print(wait(3*24*60)); print(drying(700,880), drying(150,800), drying(450,560))

--@ chunk 25
print(wait(3*24*60)); print(drying(700,880), drying(150,800), drying(450,560), drying(600,120))

--@ chunk 26

padzone = (ellipse(580,125,190,24) + ellipse(720,308,200,42) + ellipse(415,565,245,60) + ellipse(740,868,300,100) + ellipse(140,790,175,72) + ellipse(930,560,120,55)):roughen(8,40,7)
vio = pile{{"ultramarine blue",1.6},{"cobalt violet",1.5},{"lead white",1},{"carmine lake",0.15}, medium=0.15, name="vio"}
teal = pile{{"viridian",1.2},{"cobalt blue",1},{"lead white",1.2},{"ultramarine blue",0.4}, medium=0.15, name="teal"}
local free = everywhere() - skyM:grow(20) - padzone
local rightW = free * mask(function(x,y) return x > 380 and 1 or 0 end)
local leftLow = free * mask(function(x,y) return (x <= 380 and y > 500) and 1 or 0 end)
local nz = noise{seed=17, period=150, octaves=3, stretch={math.pi/2, 3}}
work(rightW + leftLow, {hand="body", tool="filbert 10", piles={
   {vio, function(x,y) return 0.4 + nz:at01(x,y) end},
   {teal, function(x,y) return clamp(1.2 - nz:at01(x,y)*1.6,0,1)*0.8 end},
   {midw, function(x,y) return 0.35 end},
   {lilac, function(x,y) return clamp(0.5 - nz:at01(x,y),0,1) end}},
   angle=math.pi/2, angle_jitter=0.12, length={25,70}, coverage=0.9, clip=-padzone, edge="soft", seed=61})

--@ chunk 27

local free = everywhere() - skyM:grow(20) - padzone
blend(free * mask(function(x,y) return (x>380 or y>500) and 1 or 0 end):soften(10), {angle=math.pi/2})

--@ chunk 28

local nz = noise{seed=19, period=120, octaves=3}
local seamH = rect(-10, 440, 420, 120):roughen(20,60,3) - padzone
local seamV = rect(340, -10, 90, 520):roughen(15,60,4) - skyM:shrink(10)
work(seamH + seamV, {hand="body", tool="filbert 9", piles={
   {vio, function(x,y) return 0.6 + 0.4*nz:at01(x,y) end},
   {deep, function(x,y) return 0.3 end},
   {lilac, function(x,y) return clamp(nz:at01(x,y)-0.3,0,1) end}},
   angle=math.pi/2, angle_jitter=0.2, length={40,110}, coverage=1.2, edge="lost", seed=71})
local ring = (padzone:grow(14) - padzone:shrink(18)) - skyM:shrink(5)
work(ring, {hand="body", tool="filbert 7", piles={{vio,1},{teal,0.6},{deep,0.4}}, angle=0, angle_jitter=0.25, length={15,40}, coverage=0.9, edge="soft", seed=72})

--@ chunk 29
blend(everywhere(), {angle=math.pi/2})

--@ chunk 30

rg = rag{width=40}
rg:dip(0.6)
rg:wipe(skyM:shrink(15), {pressure=0.6, angle=0, passes=2, refold=0.4, seed=3})

--@ chunk 31

rg = rag{width=35}
rg:dip(0.6)
local pz = (ellipse(740,868,270,85) + ellipse(140,790,155,62) + ellipse(930,560,100,45) + ellipse(415,565,225,50) + ellipse(720,308,180,32)):roughen(6,40,9)
rg:wipe(pz, {pressure=0.6, angle=0, passes=2, refold=0.35, seed=4})

--@ chunk 32
print(wait(5*24*60)); for _,p in ipairs({{200,200},{600,700},{800,700},{300,800},{100,600},{450,560}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 33

ultd = pile{{"ultramarine blue",2},{"cobalt violet",0.8},{"viridian",0.6},{"lead white",0.7}, blot=0.1, name="ultd"}
grn = pile{{"viridian",1.5},{"cobalt blue",0.5},{"lead white",1},{"yellow ochre",0.25}, blot=0.1, name="grn"}
vl2 = pile{{"cobalt violet",1.5},{"lead white",2},{"rose madder",0.3},{"ultramarine blue",0.4}, blot=0.1, name="vl2"}
cob = pile{{"cobalt blue",1.5},{"lead white",2},{"cobalt violet",0.3}, blot=0.1, name="cob"}

--@ chunk 34

local nz = noise{seed=23, period=180, octaves=3, stretch={math.pi/2, 2.5}}
local nz2 = noise{seed=24, period=130, octaves=3, stretch={math.pi/2, 2.5}}
local water = everywhere() - skyM:grow(10) - padzone:shrink(6) - willowL:shrink(10)
water = water * mask(function(x,y) return (x>330 or y>430) and 1 or 0 end):soften(30)
work(water, {hand="body", tool={kind="filbert", width=8, stiffness=0.6, lay=1.6}, piles={
   {ultd, function(x,y) return clamp(nz:at01(x,y)*1.6-0.4,0,1) end},
   {grn, function(x,y) return clamp(nz2:at01(x,y)*1.6-0.6,0,1)*0.8 end},
   {vl2, function(x,y) return clamp(0.9-nz:at01(x,y)*1.4,0,1) end},
   {cob, function(x,y) return 0.35 end}},
   angle=math.pi/2, angle_jitter=0.12, length={20,55}, coverage=0.75, edge="soft", broken=0.3, seed=81})

--@ chunk 35

local water = everywhere() - skyM:grow(25) - padzone:grow(8) - willowL:shrink(10)
water = water * mask(function(x,y) return (x>330 or y>430) and 1 or 0 end)
blend(water:shrink(6):soften(8), {angle=math.pi/2})

--@ chunk 36
print(wait(5*24*60)); for _,p in ipairs({{350,300},{600,700},{800,700},{300,800},{100,600},{700,880}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 37

wmid = pile{{"viridian",1.2},{"ultramarine blue",0.8},{"cobalt violet",0.4},{"lead white",1.0}, medium=0.1, name="wmid"}
wvio = pile{{"ultramarine blue",1.2},{"cobalt violet",1.4},{"carmine lake",0.15},{"lead white",0.5}, medium=0.15, name="wvio"}
local xs = uneven(46, -10, 440, 0.5, 0.5, 91)
for i,x in ipairs(xs) do
  local t = (x+10)/450
  local L = lerp(640, 360, t) + rand(-110,90)
  local y0 = rand(-20, 120)
  local dx = rand(-14,14)
  local r = math.random()
  local p = r<0.45 and wdark or (r<0.75 and wvio or wmid)
  local b = brush{kind="filbert", width=rand(9,18), stiffness=0.55}
  b:reload(p, rand(0.6,0.9))
  b:gesture({{x, y0, 0.85},{x+dx*0.4, lerp(y0,L,0.4), 0.8},{x+dx*0.8, lerp(y0,L,0.8), 0.55},{x+dx, L, 0.3}}, {wobble=2, orient="along"})
end

--@ chunk 38

local reg = poly({{-10,-10},{460,-10},{450,300},{400,520},{300,640},{150,690},{-10,700}}, true):soften(30) - skyM:grow(10)
blend(reg, {angle=math.pi/2})
blend(reg:shrink(40), {angle=math.pi/2+0.05})

--@ chunk 39
print(wait(6*24*60)); for _,p in ipairs({{150,300},{300,500},{50,100},{400,200}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 40

gdark = pile{{"ultramarine blue",1.5},{"viridian",1.2},{"carmine lake",0.35}, medium=0.55, name="gdark"}
local reg = poly({{-10,-10},{440,-10},{430,250},{380,470},{280,600},{120,640},{-10,650}}, true):soften(40) - skyM:grow(10)
local nz = noise{seed=33, period=90, octaves=3, stretch={math.pi/2, 4}}
work(reg, {hand="glaze", tool={kind="filbert", width=20, stiffness=0.3}, pile=gdark, angle=math.pi/2, angle_jitter=0.06, length={150,400}, coverage=1.0, clip=true, load_at=function(x,y) return 0.4+0.5*nz:at01(x,y) end, seed=101})

--@ chunk 41

local reg = poly({{-10,-10},{440,-10},{430,250},{380,470},{280,600},{120,640},{-10,650}}, true):soften(40) - skyM:grow(10)
blend(reg, {angle=math.pi/2})

--@ chunk 42

rwil = poly({{820,-10},{1010,-10},{1010,470},{950,430},{900,330},{860,200}}, true):roughen(15,60,8)
local xs = uneven(16, 840, 1005, 0.5, 0.5, 92)
for i,x in ipairs(xs) do
  local L = lerp(250, 470, (x-840)/165) + rand(-80,60)
  local y0 = rand(-20, 60)
  local dx = rand(-10,10)
  local r = math.random()
  local p = r<0.5 and wdark or (r<0.8 and wvio or wmid)
  local b = brush{kind="filbert", width=rand(9,16), stiffness=0.55}
  b:reload(p, rand(0.6,0.9))
  b:gesture({{x, y0, 0.85},{x+dx*0.5, lerp(y0,L,0.5), 0.75},{x+dx, L, 0.35}}, {wobble=2, orient="along"})
end
blend(rwil:soften(20), {angle=math.pi/2})

--@ chunk 43

blend(rwil:grow(10):soften(20), {angle=math.pi/2})
skyblue = pile{{"lead white",4},{"cobalt blue",0.6},{"viridian",0.1}, blot=0.15, name="skyblue"}
skypink = pile{{"lead white",4},{"rose madder",0.45},{"cobalt violet",0.2},{"vermilion",0.05}, blot=0.15, name="skypink"}
skygold = pile{{"lead white",4},{"cadmium yellow",0.45},{"vermilion",0.04}, blot=0.15, name="skygold"}
skycream= pile{{"lead white",5},{"pale cadmium",0.2},{"rose madder",0.05}, blot=0.15, name="skycream"}

--@ chunk 44

skypink:add{{"rose madder",0.2}}
local c1 = noise{seed=41, period=170, octaves=4, stretch={0, 4}}
local c2 = noise{seed=42, period=120, octaves=3, stretch={0, 3}}
local band = skyM:shrink(12) - padzone:grow(6)
work(band, {hand="body", tool={kind="filbert", width=9, stiffness=0.6, lay=1.8}, piles={
  {skygold, function(x,y) return clamp(c1:at01(x,y)*1.8-0.6,0,1)*(y<500 and 1.2 or 0.7) end},
  {skypink, function(x,y) return clamp(c2:at01(x,y)*1.8-0.7,0,1) end},
  {skyblue, function(x,y) return clamp(0.9-c1:at01(x,y)*1.6,0,1) end},
  {skycream, function(x,y) return 0.3 end}},
  angle=0, angle_jitter=0.12, length={25,70}, coverage=0.8, broken=0.4, edge="soft", seed=111})

--@ chunk 45
blend(skyM:grow(10), {angle=0})

--@ chunk 46

cloudlav = pile{{"lead white",3},{"cobalt violet",0.9},{"cobalt blue",0.35}, blot=0.1, name="cloudlav"}
local c1 = noise{seed=51, period=200, octaves=4, stretch={0, 5}}
local c2 = noise{seed=52, period=150, octaves=3, stretch={0, 4}}
local band = skyM:shrink(5)
work(band, {hand="body", tool={kind="filbert", width=14, stiffness=0.5, lay=1.4}, piles={
  {cloudlav, function(x,y) return clamp(c1:at01(x,y)*2-1.0,0,1) end},
  {skypink, function(x,y) return clamp(c2:at01(x,y)*2-0.9,0,1) end},
  {skygold, function(x,y) return clamp(0.9-c2:at01(x,y)*1.6,0,1)*(y<450 and 1 or 0.4) end},
  {skyblue, function(x,y) return clamp(0.8-c1:at01(x,y)*1.4,0,1) end}},
  angle=0.03, angle_jitter=0.1, length={50,160}, coverage=0.45, pressure={0.3,0.7}, broken=0.5, curve={0.2,0.2}, edge="loose", seed=121})

--@ chunk 47
blend(skyM:grow(25):soften(20), {angle=0.02})

--@ chunk 48
print(wait(7*24*60)); for _,p in ipairs({{600,200},{300,800},{150,300},{900,200},{700,700}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 49

gGold = pile{{"cadmium yellow",0.6},{"deep cadmium",0.15},{"lead white",0.6}, medium=0.7, name="gGold"}
gRose = pile{{"rose madder",0.8},{"cobalt violet",0.4},{"lead white",0.5}, medium=0.7, name="gRose"}
local band = skyM:grow(15):soften(25)
work(band * mask(function(x,y) return y < 480 and 1 or 0 end):soften(60), {hand="glaze", pile=gGold, angle=0, angle_jitter=0.1, coverage=0.8, clip=true, load_at=0.5, seed=131})
work(band * mask(function(x,y) return y > 380 and 1 or 0 end):soften(80), {hand="glaze", pile=gRose, angle=-0.6, angle_jitter=0.1, coverage=0.8, clip=true, load_at=0.5, seed=132})

--@ chunk 50

rg = rag{width=45}
rg:dip(0.4)
rg:wipe(skyM:grow(20), {pressure=0.55, angle=0, passes=2, refold=0.3, seed=7})

--@ chunk 51

rg = rag{width=45}
rg:dip(0.7)
rg:wipe(skyM:grow(20), {pressure=0.7, angle=0.1, passes=2, refold=0.2, seed=8})
rg:refold() rg:dip(0.5)
rg:wipe(skyM:grow(20) * mask(function(x,y) return y>380 and 1 or 0 end), {pressure=0.75, angle=-0.6, passes=2, refold=0.2, seed=9})

--@ chunk 52
print(wait(4*24*60))

--@ chunk 53

cl1 = pile{{"lead white",3},{"cobalt violet",0.7},{"rose madder",0.15},{"cobalt blue",0.15}, medium=0.1, name="cl1"}
cl2 = pile{{"lead white",3},{"rose madder",0.35},{"cadmium yellow",0.1}, medium=0.1, name="cl2"}
gl  = pile{{"lead white",3},{"cadmium yellow",0.35},{"pale cadmium",0.2}, medium=0.1, name="gl"}
local clouds = (ellipse(560,60,130,22) + ellipse(740,90,110,18) + ellipse(520,240,90,16) + ellipse(690,215,140,20) + ellipse(600,420,120,26) + ellipse(470,690,80,30) + ellipse(300,840,110,30)):roughen(10,50,12) * skyM:shrink(5)
local cn = noise{seed=61, period=100, octaves=3, stretch={0,3}}
work(clouds, {hand="scumble", tool={kind="filbert", width=12, stiffness=0.5}, piles={{cl1,function(x,y) return 0.6+0.6*cn:at01(x,y) end},{cl2, function(x,y) return y>400 and 0.9 or 0.3 end}}, angle=0, angle_jitter=0.15, coverage=0.9, edge="lost", pressure={0.3,0.6}, seed=141})
local goldz = (ellipse(640,150,170,40) + ellipse(700,330,130,30) + ellipse(560,520,120,30)):roughen(12,60,13) * skyM:shrink(8)
work(goldz, {hand="scumble", tool={kind="filbert", width=12, stiffness=0.5}, pile=gl, angle=0, angle_jitter=0.15, coverage=0.7, edge="lost", pressure={0.25,0.5}, seed=142})

--@ chunk 54
blend(skyM:shrink(4):soften(10), {angle=0.0})

--@ chunk 55
print(wait(5*24*60)); print(drying(600,300), drying(400,700))

--@ chunk 56
print(wait(4*24*60)); print(drying(600,300), drying(400,700), drying(300,850))

--@ chunk 57

local n1 = lose(skyM, {pile=vio, tool="filbert 7", reach={30,60}, load=0.35, pressure={0.5,0.1}, angle=math.pi/2, every=2.0, seed=151})
local n2 = lose(skyM, {pile=midw, tool="filbert 6", reach={20,40}, load=0.3, pressure={0.45,0.1}, angle=-math.pi/2, every=2.5, seed=152})
local n3 = lose(-skyM, {pile=cl1, tool="filbert 6", reach={20,50}, load=0.35, pressure={0.45,0.1}, angle=math.pi/2, every=2.5, seed=153})
print(n1,n2,n3)

--@ chunk 58

local ringm = skyM:grow(60) - skyM:shrink(75)
rg = rag{width=40}
rg:dip(0.8)
rg:wipe(ringm, {pressure=0.75, angle=math.pi/2, passes=3, refold=0.2, seed=17})

--@ chunk 59
print(wait(6*24*60)); for _,p in ipairs({{600,300},{400,700},{300,850},{500,800},{800,500}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 60

-- water piles, stiffer, slightly blotted for broken marks
wA = pile{{"ultramarine blue",1.2},{"cobalt violet",1},{"lead white",1.2},{"viridian",0.3}, name="wA"}
wB = pile{{"cobalt blue",1},{"viridian",0.6},{"lead white",1.5}, name="wB"}
wC = pile{{"cobalt violet",1.2},{"lead white",1.8},{"rose madder",0.15},{"cobalt blue",0.3}, name="wC"}
wD = pile{{"viridian",1},{"ultramarine blue",0.8},{"lead white",0.5},{"yellow ochre",0.15}, name="wD"}
local nz = noise{seed=71, period=160, octaves=3, stretch={math.pi/2, 3}}
local nz2 = noise{seed=72, period=110, octaves=3, stretch={math.pi/2, 3}}
local dist = function(x,y) return math.abs(x - lerp(640, 160, y/1000)) end
local water = (everywhere() - skyM:shrink(20) - willowL:shrink(40) - rwil:shrink(30)) * mask(function(x,y) return (x>360 or y>520) and 1 or 0 end):soften(40)
work(water, {hand="body", tool={kind="filbert", width=9, stiffness=0.6, lay=1.5}, piles={
   {wA, function(x,y) return 0.3 + nz:at01(x,y) end},
   {wB, function(x,y) return clamp(nz2:at01(x,y)*1.5-0.3,0,1) end},
   {wC, function(x,y) return clamp(1-dist(x,y)/350,0,1) + 0.3*(1-nz:at01(x,y)) end},
   {wD, function(x,y) return clamp((y-500)/500,0,1)*0.6*nz2:at01(x,y) end}},
   angle=math.pi/2, angle_jitter=0.1, length={18,50}, coverage=0.6, broken=0.4, pressure={0.4,0.8}, edge="lost", seed=161})

--@ chunk 61

local water = (everywhere() - skyM:shrink(20) - willowL:shrink(40) - rwil:shrink(30)) * mask(function(x,y) return (x>360 or y>520) and 1 or 0 end):soften(40)
rg = rag{width=50}
rg:dip(0.9)
rg:wipe(water:grow(20), {pressure=0.8, angle=math.pi/2, passes=3, refold=0.15, seed=21})

--@ chunk 62

kn = knife{width=50}
local water = (everywhere() - skyM:shrink(20) - willowL:shrink(40) - rwil:shrink(30)) * mask(function(x,y) return (x>360 or y>520) and 1 or 0 end):soften(40)
local cnt=0
for y0 = 0, 1000, 45 do
  local xs = {}
  for x = -20, 1020, 8 do
    if water:at(x, y0+20) > 0.3 then xs[#xs+1]=x end
  end
  -- scrape horizontal runs across the row
  local i=1
  while i <= #xs do
    local s=xs[i]; local e=s
    while i < #xs and xs[i+1]-xs[i] <= 8 do i=i+1; e=xs[i] end
    if e-s > 10 then
      kn:scrape({{s-10,y0+22},{e+10,y0+22}}, {pressure=1.0, angle=math.pi/2})
      kn:wipe(); cnt=cnt+1
    end
    i=i+1
  end
end
print(cnt)

--@ chunk 63
print(wait(8*24*60)); for _,p in ipairs({{600,700},{400,700},{300,850},{800,500},{100,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 64

dk1 = pile{{"ultramarine blue",1.5},{"viridian",0.6},{"cobalt violet",0.5},{"lead white",0.3}, medium=0.3, name="dk1"}
dk2 = pile{{"viridian",1},{"cobalt blue",0.6},{"ultramarine blue",0.4},{"lead white",0.3}, medium=0.3, name="dk2"}
dk3 = pile{{"cobalt violet",1.2},{"ultramarine blue",0.8},{"carmine lake",0.1},{"lead white",0.3}, medium=0.3, name="dk3"}

--@ chunk 65

local dist = function(x,y) return math.abs(x - lerp(640, 160, y/1000)) end
local nz = noise{seed=81, period=170, octaves=3, stretch={math.pi/2, 3}}
local nz2 = noise{seed=82, period=120, octaves=3, stretch={math.pi/2, 3}}
local dark = mask(function(x,y) return clamp((dist(x,y)-120)/200,0,1) * (0.4 + 0.8*nz:at01(x,y)) end)
local water = (everywhere() - skyM:shrink(10) - padzone:shrink(4) - willowL:shrink(50) - rwil:shrink(40))
work(water, {hand="body", tool={kind="filbert", width=10, stiffness=0.5}, piles={
   {dk1, function(x,y) return 0.3+nz:at01(x,y) end},
   {dk2, function(x,y) return clamp(nz2:at01(x,y)*1.6-0.4,0,1) end},
   {dk3, function(x,y) return clamp(1-nz:at01(x,y)*1.3,0,1)*0.8 end}},
   load_at=function(x,y) return 0.25 + 0.6*clamp((dist(x,y)-100)/250,0,1)*(0.4+0.6*nz:at01(x,y)) end,
   angle=math.pi/2, angle_jitter=0.08, length={40,130}, coverage=0.55, pressure={0.3,0.75}, broken=0.3, edge="lost", seed=171})

--@ chunk 66

local water = (everywhere() - skyM:grow(20) - padzone:grow(6))
blend(water:soften(10), {angle=math.pi/2})
blend(water:shrink(10):soften(10), {angle=math.pi/2+0.04})

--@ chunk 67
print(wait(8*24*60)); for _,p in ipairs({{600,700},{400,700},{300,850},{800,500},{100,600},{420,300}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 68

eM = pile{{"cobalt blue",1},{"cobalt violet",1},{"ultramarine blue",0.4},{"lead white",1.0}, medium=0.2, name="eM"}
local edgez = (skyM:grow(80) - skyM:shrink(10) - padzone:grow(4)) * mask(function(x,y) return y>40 and 1 or 0 end)
local nz = noise{seed=91, period=120, octaves=3, stretch={math.pi/2, 3}}
work(edgez, {hand="body", tool={kind="filbert", width=12, stiffness=0.5}, piles={{eM,1},{dk3, function(x,y) return 0.5*nz:at01(x,y) end},{wC, function(x,y) return 0.5*(1-nz:at01(x,y)) end}},
   angle=math.pi/2, angle_jitter=0.05, length={60,140}, coverage=1.1, curve={0,0}, pressure={0.5,0.85}, fill=true, edge="soft", seed=181})
blend(edgez:soften(15), {angle=math.pi/2})

--@ chunk 69

local edgez = (skyM:grow(80) - skyM:shrink(10) - padzone:grow(4)) * mask(function(x,y) return y>40 and 1 or 0 end)
for i=1,3 do
  rg = rag{width=50}
  rg:dip(0.9)
  rg:wipe(edgez:grow(15), {pressure=0.85, angle=math.pi/2 + (i-2)*0.3, passes=2, refold=0.15, seed=200+i})
end

--@ chunk 70

function bandL(y) for x=0,1000,4 do if skyM:at(x,y)>0.5 then return x end end return nil end
function bandR(y) for x=1000,0,-4 do if skyM:at(x,y)>0.5 then return x end end return nil end
for y=0,1000,100 do print(y, bandL(y), bandR(y)) end

--@ chunk 71

function vgest(x, y, len, p, w, load, press, lean)
  local b = brush{kind="filbert", width=w, stiffness=0.55}
  b:reload(p, load)
  lean = lean or 0
  b:gesture({{x, y, press*0.8},{x+lean*0.5, y+len*0.5, press},{x+lean, y+len, press*0.6}}, {wobble=1.5, orient="along", ramps={0.15,0.3}})
end
local pl = {dk1, dk3, eM, wC, dk2}
local n=0
for y = 20, 990, 18 do
  local bl = bandL(y)
  if bl then
    for k=1,3 do
      local d = rand(-30, 130)   -- distance outward from band edge
      local x = bl - d
      local t = clamp(d/130,0,1)
      local p
      local r = math.random()
      if t > 0.6 then p = (r<0.5) and dk1 or ((r<0.8) and dk3 or dk2)
      elseif t > 0.25 then p = (r<0.4) and dk3 or ((r<0.75) and eM or dk1)
      else p = (r<0.5) and wC or eM end
      if y < 520 or padzone:at(x, y) < 0.3 then
        vgest(x, y + rand(-8,8), rand(50,120), p, rand(8,13), rand(0.45,0.75), rand(0.5,0.8), rand(-4,4))
        n=n+1
      end
    end
  end
end
print(n)

--@ chunk 72

local n=0
for y = 120, 990, 18 do
  local br = bandR(y)
  if br then
    for k=1,3 do
      local d = rand(-30, 130)
      local x = br + d
      local t = clamp(d/130,0,1)
      local p
      local r = math.random()
      if t > 0.6 then p = (r<0.5) and dk1 or ((r<0.8) and dk2 or dk3)
      elseif t > 0.25 then p = (r<0.4) and dk3 or ((r<0.75) and eM or dk2)
      else p = (r<0.5) and wC or eM end
      if padzone:at(x, y) < 0.3 then
        vgest(x, y + rand(-8,8), rand(50,120), p, rand(8,13), rand(0.45,0.75), rand(0.5,0.8), rand(-4,4))
        n=n+1
      end
    end
  end
end
print(n)

--@ chunk 73

skyblue2 = pile{{"lead white",3},{"cobalt blue",0.5},{"cobalt violet",0.15}, name="skyblue2"}
peach = pile{{"lead white",3},{"cadmium yellow",0.25},{"vermilion",0.06},{"rose madder",0.08}, name="peach"}
pearl = pile{{"lead white",4},{"cobalt violet",0.15},{"pale cadmium",0.08}, name="pearl"}
local choices = {{skyblue2,3},{peach,2},{pearl,2},{cl1,2},{skycream,1}}
local n=0
for i=1,260 do
  local y = rand(0, 1000)
  local l, r = bandL(y), bandR(y)
  if l and r and r-l > 60 then
    local x = rand(l+15, r-15)
    local p = pickpile(choices)
    local b = brush{kind="filbert", width=rand(9,16), stiffness=0.6}
    b:reload(p, rand(0.35,0.65))
    local len = rand(30, 90); local a = randn(0, 0.08)
    if math.random()<0.25 then a = math.pi/2 + randn(0,0.06); len = len*0.8 end
    b:gesture({{x-math.cos(a)*len/2, y-math.sin(a)*len/2, 0.5},{x, y, rand(0.55,0.8)},{x+math.cos(a)*len/2, y+math.sin(a)*len/2, 0.35}}, {wobble=1, orient="along", ramps={0.2,0.3}})
    n=n+1
  end
end
print(n)

--@ chunk 74
blend(skyM:shrink(25):soften(15), {angle=0.0})
blend(skyM:shrink(25):soften(15), {angle=0.03})

--@ chunk 75
print(wait(12*24*60)); for _,p in ipairs({{600,300},{450,600},{700,600},{300,800},{900,300}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 76

pL = pile{{"pale cadmium",1},{"viridian",0.3},{"lead white",0.6}, name="pL"}
pM = pile{{"viridian",1},{"cadmium yellow",0.8},{"lead white",0.35}, name="pM"}
pS = pile{{"viridian",1},{"cobalt blue",0.6},{"cobalt violet",0.25},{"lead white",0.3}, name="pS"}
pW = pile{{"yellow ochre",0.6},{"cadmium yellow",0.5},{"viridian",0.3},{"vermilion",0.08},{"lead white",0.3}, name="pW"}
pV = pile{{"cobalt violet",0.8},{"viridian",0.5},{"cobalt blue",0.3},{"lead white",0.5}, name="pV"}

--@ chunk 77

function psize(y) local t=clamp(y/1000,0,1); local w=lerp(30,150,t^1.15); return w, w*lerp(0.22,0.36,t) end
function hstroke(p, x0, x1, y, bw, load, press, tilt)
  local b = brush{kind="filbert", width=bw, stiffness=0.6}
  b:reload(p, load)
  tilt = tilt or 0
  b:gesture({{x0, y-tilt, press*0.8},{(x0+x1)/2, y, press},{x1, y+tilt, press*0.7}}, {wobble=0.5, orient="along", ramps={0.12,0.25}})
end
function pad2(cx, cy, s, onLight)
  local w,h = psize(cy); w=w*s*rand(0.8,1.2); h=h*s*rand(0.85,1.15)
  local bw = clamp(h*0.5, 4, 30)
  local base = onLight and pickpile({{pS,2},{pM,2},{pV,1.5}}) or pickpile({{pM,3},{pW,1.2},{pV,0.8},{pS,0.8}})
  local rows = math.max(2, math.floor(h/bw*1.5+0.5))
  for i=1,rows do
    local f=(i-0.5)/rows*2-1
    local half=w/2*math.sqrt(math.max(0.08,1-f*f))*rand(0.85,1.08)
    local yy=cy+f*h*0.38
    hstroke(base, cx-half, cx+half, yy, bw, rand(0.6,0.85), rand(0.6,0.85), rand(-0.03,0.03)*w)
  end
  -- light on upper part
  local lp = onLight and pM or (math.random()<0.7 and pL or pW)
  local lw = w*rand(0.3,0.6); local lx = cx + rand(-0.2,0.2)*w
  hstroke(lp, lx-lw/2, lx+lw/2, cy - h*rand(0.05,0.25), bw*0.8, rand(0.5,0.75), rand(0.5,0.75), 0)
  -- shadow underneath
  if math.random()<0.7 then
    local sw = w*rand(0.4,0.8); local sx = cx + rand(-0.15,0.15)*w
    hstroke(pick or dk1, sx-sw/2, sx+sw/2, cy + h*0.48, math.max(3,bw*0.5), rand(0.4,0.6), rand(0.4,0.6), 0)
  end
end
function raft(pts, n, spread, s, seed)
  -- pts: center line polyline; n pads; spread: vertical scatter as fraction of pad height
  local items = {}
  for i=1,n do
    local t = rand(0,1)
    local seg = (#pts-1)*t; local k = math.min(#pts-1, math.floor(seg)+1); local u = seg-(k-1)
    local x = lerp(pts[k][1], pts[k+1][1], u); local y = lerp(pts[k][2], pts[k+1][2], u)
    local w,h = psize(y)
    y = y + randn(0, spread*h)
    x = x + randn(0, w*0.15)
    items[#items+1] = {x,y}
  end
  table.sort(items, function(a,b) return a[2]<b[2] end)
  for _,it in ipairs(items) do pad2(it[1], it[2], s, skyM:at(it[1], it[2])>0.5) end
end
raft({{380,165},{560,150},{700,175},{830,160}}, 26, 0.8, 1.0)

--@ chunk 78

raft({{560,400},{700,385},{830,410},{980,395}}, 22, 0.7, 1.0)
raft({{120,585},{300,570},{450,595},{560,580}}, 20, 0.7, 1.0)

--@ chunk 79

raft({{470,880},{620,860},{780,880},{960,870},{1020,890}}, 18, 0.6, 1.0)
raft({{-20,790},{100,770},{230,800}}, 8, 0.6, 1.0)
raft({{560,975},{760,985},{900,975}}, 6, 0.3, 1.0)

--@ chunk 80
print(wait(9*24*60)); for _,p in ipairs({{600,170},{650,390},{300,580},{700,880},{100,780}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 81
print(wait(6*24*60)); for _,p in ipairs({{600,170},{650,390},{300,580},{700,880},{100,780}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 82
print(wait(6*24*60)); for _,p in ipairs({{100,780},{150,790},{50,770}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 83

rafts = (ribbon({{380,165},{560,150},{700,175},{830,160}}, 30) + ribbon({{540,400},{700,385},{830,410},{1000,395}}, 55) + ribbon({{80,585},{300,570},{450,595},{570,580}}, 75) + ribbon({{440,880},{620,860},{780,880},{1010,880}}, 150) + ribbon({{-20,790},{100,770},{260,800}}, 80) + ribbon({{480,975},{760,985},{920,975}}, 60))
print(rafts:area())
mw1 = pile{{"cobalt blue",1},{"cobalt violet",0.8},{"ultramarine blue",0.5},{"lead white",0.6}, medium=0.15, name="mw1"}
mw2 = pile{{"viridian",0.8},{"cobalt blue",0.8},{"ultramarine blue",0.4},{"lead white",0.4}, medium=0.15, name="mw2"}
mw3 = pile{{"cobalt violet",1},{"ultramarine blue",0.6},{"lead white",0.7},{"rose madder",0.1}, medium=0.15, name="mw3"}

--@ chunk 84

local regs = (rect(560,430,450,400) + rect(230,620,380,390) + rect(-10,460,340,300) + rect(-10,820,220,190)) - skyM:grow(5) - rafts
local cnt=0
local choices = {{mw1,3},{mw2,2.5},{mw3,2},{dk3,0.8},{dk2,0.8}}
for i=1,520 do
  local x = rand(-10,1010); local y = rand(400,1000)
  if regs:at(x,y) > 0.5 then
    local len = rand(60,170)
    -- shorten if it would run into a raft
    local L = len
    for d=10,len,10 do if rafts:at(x, y+d) > 0.3 or skyM:at(x,y+d)>0.5 then L = d-5 break end end
    if L > 25 then
      vgest(x, y, L, pickpile(choices), rand(7,12), rand(0.35,0.6), rand(0.45,0.75), rand(-3,3))
      cnt=cnt+1
    end
  end
end
print(cnt)

--@ chunk 85

raft({{270,725},{360,715},{470,735}}, 7, 0.5, 0.9)
raft({{90,935},{200,925},{300,945}}, 5, 0.4, 0.9)

--@ chunk 86

fW = pile{{"lead white",4},{"pale cadmium",0.05}, blot=0.2, name="fW"}
fP = pile{{"lead white",3},{"rose madder",0.6},{"vermilion",0.08}, blot=0.15, name="fP"}
fR = pile{{"rose madder",1},{"vermilion",0.5},{"lead white",0.6}, blot=0.1, name="fR"}
fY = pile{{"pale cadmium",1},{"lead white",0.5},{"deep cadmium",0.1}, blot=0.15, name="fY"}
function petal(p, x, y, len, ang, bw, load)
  local b = brush{kind="filbert", width=bw, stiffness=0.7, lay=3}
  b:reload(p, load or 0.7)
  local dx, dy = math.cos(ang)*len/2, math.sin(ang)*len/2
  b:gesture({{x-dx,y-dy,0.6},{x,y,0.8},{x+dx,y+dy,0.4}}, {wobble=0.3, orient="along", ramps={0.2,0.3}})
end
function flower(x, y, kind)
  local w = psize(y); local s = clamp(w*0.22, 5, 30)
  local main, top = fP, fW
  if kind=="white" then main, top = fW, fW elseif kind=="red" then main, top = fR, fP end
  -- outer petals, low and wide
  for i=1,3 do petal(main, x + rand(-0.4,0.4)*s, y + rand(0,0.2)*s, s*rand(0.7,1.1), rand(-0.25,0.25), s*0.35) end
  -- inner upward petals
  for i=1,2 do petal(top, x + rand(-0.25,0.25)*s, y - s*rand(0.15,0.35), s*rand(0.4,0.6), -math.pi/2 + rand(-0.5,0.5), s*0.28) end
  if math.random()<0.6 then petal(fY, x, y - s*0.1, s*0.25, 0, s*0.2, 0.5) end
end
-- try one
flower(640, 170, "pink")
flower(600, 380, "white")
flower(700, 870, "pink")

--@ chunk 87

function flower2(x, y, kind, sc)
  local w = psize(y); local s = clamp(w*0.28, 6, 40) * (sc or 1)
  local main, light = fP, fW
  if kind=="white" then main, light = pile and fW, fW elseif kind=="red" then main, light = fR, fP end
  for i=1,7 do
    local px = x + rand(-0.55,0.55)*s; local py = y + rand(-0.15,0.2)*s
    petal(main, px, py, s*rand(0.35,0.6), rand(-0.6,0.6), s*rand(0.22,0.32), rand(0.6,0.85))
  end
  for i=1,4 do
    local px = x + rand(-0.35,0.35)*s; local py = y - s*rand(0.05,0.3)
    petal(light, px, py, s*rand(0.25,0.4), -math.pi/2 + rand(-0.7,0.7), s*rand(0.18,0.26), rand(0.6,0.85))
  end
  if kind ~= "red" then petal(fY, x+rand(-0.1,0.1)*s, y - s*0.05, s*0.2, 0, s*0.18, 0.6) end
end
flower2(700, 862, "pink")

--@ chunk 88

local F = {{470,152,"white",0.8},{730,168,"pink",0.8},{560,395,"pink",0.9},{790,400,"white",0.9},{965,382,"red",0.9},
 {180,575,"pink",1},{330,598,"white",1},{500,585,"red",0.9},{120,780,"white",1},{215,792,"pink",0.9},
 {380,730,"pink",0.9},{470,885,"white",1},{860,830,"red",0.9},{900,960,"pink",1},{250,915,"white",0.9},{640,975,"pink",1}}
for _,f in ipairs(F) do flower2(f[1], f[2], f[3], f[4]) end

--@ chunk 89
print(wait(14*24*60)); for _,p in ipairs({{700,862},{620,880},{300,590},{100,790},{500,700}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 90

pBlu = pile{{"cobalt blue",0.8},{"lead white",1.2},{"viridian",0.2},{"cobalt violet",0.2}, name="pBlu"}
pDG = pile{{"viridian",1},{"ultramarine blue",0.6},{"cadmium yellow",0.2},{"lead white",0.2}, name="pDG"}
pCool = pile{{"viridian",0.8},{"barium yellow",1},{"lead white",0.5}, name="pCool"}
pLav = pile{{"cobalt violet",1},{"lead white",1.2},{"cobalt blue",0.3}, name="pLav"}
flowersM = nil
local F = {{470,152},{730,168},{560,395},{790,400},{965,382},{180,575},{330,598},{500,585},{120,780},{215,792},{380,730},{470,885},{860,830},{900,960},{250,915},{640,975},{700,862},{640,170},{600,380}}
for _,f in ipairs(F) do local w=psize(f[2]); local e = ellipse(f[1], f[2]-w*0.05, w*0.28+8, w*0.16+8); flowersM = flowersM and (flowersM+e) or e end
padsNow = rafts:shrink(4)
local cnt=0
for i=1,700 do
  local x=rand(-10,1010); local y=rand(120,1000)
  if padsNow:at(x,y)>0.5 and flowersM:at(x,y)<0.1 then
    local w,h = psize(y)
    local len = w*rand(0.25,0.6)
    local r=math.random()
    local p = r<0.3 and pBlu or (r<0.55 and pDG or (r<0.8 and pCool or pLav))
    local b = brush{kind="filbert", width=clamp(h*rand(0.25,0.4),3,18), stiffness=0.7}
    b:reload(p, rand(0.2,0.4))
    b:gesture({{x-len/2,y,0.45},{x,y+rand(-1,1),0.6},{x+len/2,y,0.35}}, {wobble=0.5, orient="along", ramps={0.2,0.3}})
    cnt=cnt+1
  end
end
print(cnt)

--@ chunk 91
blend((padsNow - flowersM:grow(6)):soften(6), {angle=0})

--@ chunk 92

local cnt=0
local zone = padsNow * mask(function(x,y) return (x<560 and y>520) and 1 or 0 end) + (ribbon({{270,725},{360,715},{470,735}},40) + ribbon({{90,935},{200,925},{300,945}},40))
for i=1,700 do
  local x=rand(-10,600); local y=rand(520,1000)
  if zone:at(x,y)>0.5 and flowersM:at(x,y)<0.1 then
    local w,h = psize(y)
    local len = w*rand(0.25,0.6)
    local r=math.random()
    local p = r<0.3 and pBlu or (r<0.55 and pDG or (r<0.8 and pCool or pLav))
    local b = brush{kind="filbert", width=clamp(h*rand(0.25,0.4),3,18), stiffness=0.7}
    b:reload(p, rand(0.2,0.4))
    b:gesture({{x-len/2,y,0.45},{x,y+rand(-1,1),0.6},{x+len/2,y,0.35}}, {wobble=0.5, orient="along", ramps={0.2,0.3}})
    cnt=cnt+1
  end
end
print(cnt)
zoneL = zone
blend((zone:grow(4) - flowersM:grow(6)):soften(6), {angle=0})

--@ chunk 93
print(wait(14*24*60)); for _,p in ipairs({{700,862},{300,590},{100,790},{200,920},{600,900}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 94

raft({{600,690},{720,670},{860,700},{1010,680}}, 13, 0.7, 0.95)
local zone = ribbon({{600,690},{720,670},{860,700},{1010,680}}, 70)
local cnt=0
for i=1,250 do
  local x=rand(560,1010); local y=rand(630,740)
  if zone:at(x,y)>0.5 then
    local w,h = psize(y)
    local len = w*rand(0.2,0.5)
    local r=math.random()
    local p = r<0.3 and pBlu or (r<0.55 and pDG or (r<0.8 and pCool or pLav))
    local b = brush{kind="filbert", width=clamp(h*rand(0.25,0.4),3,18), stiffness=0.7}
    b:reload(p, rand(0.15,0.3))
    b:gesture({{x-len/2,y,0.45},{x,y,0.6},{x+len/2,y,0.35}}, {wobble=0.5, orient="along", ramps={0.2,0.3}})
    cnt=cnt+1
  end
end
print(cnt)
flower2(760, 668, "white", 0.9)
flower2(930, 690, "pink", 0.9)

--@ chunk 95

-- sparse broken-stroke raft, no solid base
local function brokenRaft(pts, n, spread)
  for i=1,n do
    local t = rand(0,1)
    local seg = (#pts-1)*t; local k = math.min(#pts-1, math.floor(seg)+1); local u = seg-(k-1)
    local x = lerp(pts[k][1], pts[k+1][1], u); local y = lerp(pts[k][2], pts[k+1][2], u)
    local w,h = psize(y)
    y = y + randn(0, spread*h)
    local len = w*rand(0.3,0.7)
    local r=math.random()
    local p = r<0.35 and pM or (r<0.55 and pCool or (r<0.75 and pS or (r<0.9 and pBlu or pL)))
    local b = brush{kind="filbert", width=clamp(h*rand(0.3,0.45),3,18), stiffness=0.7}
    b:reload(p, rand(0.3,0.55))
    b:gesture({{x-len/2,y,0.5},{x,y,0.7},{x+len/2,y,0.4}}, {wobble=0.5, orient="along", ramps={0.2,0.3}})
  end
end
brokenRaft({{760,535},{880,525},{1010,540}}, 40, 0.6)
brokenRaft({{20,470},{120,460},{240,478}}, 30, 0.6)

--@ chunk 96

fr1 = pile{{"viridian",1},{"cobalt blue",0.6},{"lead white",0.6},{"cadmium yellow",0.1}, medium=0.1, name="fr1"}
fr2 = pile{{"cobalt violet",1},{"ultramarine blue",0.5},{"lead white",0.7}, medium=0.1, name="fr2"}
fr3 = pile{{"ultramarine blue",1.4},{"viridian",1},{"carmine lake",0.2}, medium=0.2, name="fr3"}
local cnt=0
for i=1,110 do
  local x = rand(-10, 360); local y = rand(-30, 330)
  local L = rand(80, 220)
  local p = pickpile({{fr1,2},{fr2,2},{fr3,3}})
  local b = brush{kind="filbert", width=rand(5,9), stiffness=0.55}
  b:reload(p, rand(0.3,0.55))
  local lean = rand(-6,6)
  b:gesture({{x, y, 0.4},{x+lean*0.4, y+L*0.5, 0.6},{x+lean, y+L, 0.3}}, {wobble=1, orient="along", ramps={0.2,0.35}})
  cnt=cnt+1
end
print(cnt)

--@ chunk 97

local reg = poly({{-10,-10},{380,-10},{370,300},{330,450},{200,560},{-10,560}}, true):soften(30)
blend(reg, {angle=math.pi/2})

--@ chunk 98
print(wait(20*24*60)); for _,p in ipairs({{100,470},{800,530},{760,668},{200,300}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 99

sR = pile{{"lead white",3},{"rose madder",0.35},{"vermilion",0.05}, medium=0.1, name="sR"}
sG = pile{{"lead white",3},{"cadmium yellow",0.3},{"deep cadmium",0.04}, medium=0.1, name="sG"}
local band = skyM:shrink(30) - padzone:grow(10) - rafts:grow(10) - flowersM:grow(10)
local warm = (ellipse(620,80,170,50) + ellipse(650,280,120,60)):roughen(20,80,31) * band
local pink = (ellipse(600,470,140,50) + ellipse(420,650,90,40) + ellipse(260,880,90,40)):roughen(20,80,32) * band
work(warm, {hand="scumble", tool={kind="filbert", width=14, stiffness=0.5}, pile=sG, angle=0, angle_jitter=0.1, coverage=0.6, pressure={0.2,0.45}, load=0.35, edge="lost", seed=301})
work(pink, {hand="scumble", tool={kind="filbert", width=14, stiffness=0.5}, pile=sR, angle=0, angle_jitter=0.1, coverage=0.6, pressure={0.2,0.45}, load=0.35, edge="lost", seed=302})

--@ chunk 100
local band = skyM:shrink(20) - rafts:grow(4) - flowersM:grow(6)
blend(band:soften(8), {angle=0})
blend(band:soften(8), {angle=0.04})

--@ chunk 101
print(wait(12*24*60)); print(drying(600,100), drying(500,600))

--@ chunk 102

local choices = {{skyblue2,3},{pearl,2},{cl1,2.5},{peach,1.5},{skycream,1.5},{sR,1}}
local band = skyM:shrink(15) - rafts:grow(6) - flowersM:grow(8)
local n=0
for i=1,420 do
  local y = rand(0, 1000)
  local l, r = bandL(y), bandR(y)
  if l and r and r-l > 40 then
    local x = rand(l+10, r-10)
    if band:at(x,y) > 0.5 then
      local p = pickpile(choices)
      local b = brush{kind="filbert", width=rand(6,12), stiffness=0.7}
      b:reload(p, rand(0.12,0.3))
      local len = rand(25, 80); local a = randn(0, 0.04)
      b:gesture({{x-math.cos(a)*len/2, y-math.sin(a)*len/2, 0.35},{x, y, rand(0.35,0.55)},{x+math.cos(a)*len/2, y+math.sin(a)*len/2, 0.25}}, {wobble=0.8, orient="along", ramps={0.2,0.3}})
      n=n+1
    end
  end
end
print(n)

--@ chunk 103

local n=0
for i=1,26 do
  local x = rand(870, 1005); local y = rand(-30, 60)
  local p = pickpile({{dk1,2},{dk3,2},{wdark,1.5},{eM,1}})
  vgest(x, y, rand(90,200), p, rand(9,14), rand(0.5,0.75), rand(0.55,0.8), rand(-3,3))
  n=n+1
end
-- a cloud-shaped lavender reflection drifting in the upper band
local choices = {{cl1,3},{pearl,1},{skyblue2,1.5}}
for i=1,60 do
  local cx = rand(480, 820); local cy = rand(40, 300)
  local inside = (cx-640)^2/170^2 + (cy-200)^2/70^2 < 1 or (cx-560)^2/110^2 + (cy-60)^2/30^2 < 1
  if inside and skyM:shrink(25):at(cx,cy) > 0.5 and rafts:grow(8):at(cx,cy) < 0.2 then
    local b = brush{kind="filbert", width=rand(10,16), stiffness=0.5}
    b:reload(pickpile(choices), rand(0.2,0.35))
    local len = rand(40,110)
    b:gesture({{cx-len/2,cy,0.3},{cx,cy,0.45},{cx+len/2,cy,0.25}}, {wobble=0.6, orient="along", ramps={0.25,0.35}})
  end
end
print(n)

--@ chunk 104

-- a few hanging fronds crossing into the light at the top-left edge
for i=1,9 do
  local x = rand(420, 500); local y = rand(-20, 40)
  local b = brush{kind="filbert", width=rand(4,7), stiffness=0.5}
  b:reload(pickpile({{fr3,2},{fr2,1},{fr1,1}}), rand(0.35,0.55))
  local L = rand(120,300); local lean = rand(-5,5)
  b:gesture({{x,y,0.5},{x+lean*0.5,y+L*0.5,0.6},{x+lean,y+L,0.15}}, {wobble=0.8, orient="along", ramps={0.15,0.4}})
end
-- lavender cloud reflection, more load
local choices = {{cl1,3},{skyblue2,1.5}}
local cnt=0
for i=1,400 do
  local cx = rand(480, 820); local cy = rand(150, 320)
  local inside = (cx-660)^2/150^2 + (cy-235)^2/55^2 < 1
  if inside and skyM:shrink(25):at(cx,cy) > 0.5 and rafts:grow(8):at(cx,cy) < 0.2 and cnt < 40 then
    local b = brush{kind="filbert", width=rand(10,15), stiffness=0.5}
    b:reload(pickpile(choices), rand(0.35,0.5))
    local len = rand(40,100)
    b:gesture({{cx-len/2,cy,0.4},{cx,cy,0.55},{cx+len/2,cy,0.3}}, {wobble=0.6, orient="along", ramps={0.25,0.35}})
    cnt=cnt+1
  end
end
print(cnt)

--@ chunk 105
blend(rect(405,-10,110,340):soften(15), {angle=math.pi/2})

--@ chunk 106

sig = pile{{"ultramarine blue",1},{"carmine lake",0.4},{"viridian",0.3}, medium=0.2, name="sig"}
local r = brush{kind="rigger", width=2.2, point=1}
r:reload(sig, 0.7)
-- "Claude" in a quick hand, small, lower left
local ox, oy = 40, 978
r:gesture({{ox+10,oy-8,0.4},{ox+3,oy-10,0.6},{ox,oy-3,0.7},{ox+3,oy+2,0.6},{ox+10,oy,0.3}}, {wobble=0.3})
r:gesture({{ox+14,oy-14,0.3},{ox+14,oy+1,0.6}}, {wobble=0.2})
r:gesture({{ox+24,oy-4,0.4},{ox+19,oy-4,0.6},{ox+18,oy+1,0.6},{ox+23,oy+1,0.5},{ox+24,oy-5,0.4},{ox+25,oy+1,0.4}}, {wobble=0.2})
r:gesture({{ox+29,oy-5,0.4},{ox+29,oy,0.6},{ox+33,oy+1,0.5},{ox+35,oy-5,0.4},{ox+36,oy+1,0.4}}, {wobble=0.2})
r:gesture({{ox+46,oy-4,0.4},{ox+41,oy-3,0.6},{ox+41,oy+1,0.6},{ox+46,oy,0.5},{ox+47,oy-14,0.4},{ox+47,oy+1,0.5}}, {wobble=0.2})
r:gesture({{ox+51,oy-2,0.5},{ox+56,oy-4,0.5},{ox+52,oy-5,0.5},{ox+51,oy,0.6},{ox+57,oy+1,0.3}}, {wobble=0.2})

--@ chunk 107
for _,p in ipairs({{600,100},{700,200},{520,80},{870,60},{450,900},{800,900}}) do print(p[1],p[2],drying(p[1],p[2])) end
for y=0,400,40 do print(y, bandL(y), bandR(y)) end

--@ chunk 108
print(wait(10*24*60)); for _,p in ipairs({{600,100},{700,200},{520,80},{650,150},{760,250}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 109
cBlue = pile{{"lead white",2},{"cobalt blue",0.55},{"cobalt violet",0.25},{"viridian",0.05}, name="cBlue"}
cLav = pile{{"lead white",2},{"cobalt violet",0.8},{"rose madder",0.12},{"cobalt blue",0.15}, name="cLav"}
cRose = pile{{"lead white",2.5},{"rose madder",0.35},{"cobalt violet",0.2},{"cadmium yellow",0.05}, name="cRose"}
cWarm = pile{{"lead white",3},{"cadmium yellow",0.25},{"vermilion",0.05}, name="cWarm"}

--@ chunk 110
cBlue:add{{"cobalt blue",0.3},{"cobalt violet",0.1}}
cLav:add{{"cobalt violet",0.3},{"cobalt blue",0.05}}
print(cBlue, cLav)

--@ chunk 111
for _,p in ipairs({{680,140},{560,120},{740,150},{620,385},{600,330}}) do print(p[1],p[2],padsNow:at(p[1],p[2]), padzone:at(p[1],p[2]), rafts:at(p[1],p[2])) end

--@ chunk 112
farAvoid = ellipse(588,146,30,16) + ellipse(670,170,70,16) + ellipse(716,187,22,9) + ellipse(772,165,30,10) + ellipse(520,150,40,10) + ellipse(450,155,40,10) + ellipse(400,182,30,8)
farAvoid = farAvoid:grow(5)
function cloudpatch(cx, cy, rx, ry, p, n, wmin, wmax, ld)
  local clipM = everywhere() - farAvoid
  for i=1,n do
    local x,y
    repeat x=rand(-1,1); y=rand(-1,1) until x*x+y*y<1
    x = cx + x*rx; y = cy + y*ry
    local len = rx*rand(0.35,0.8)
    local b = brush{kind="filbert", width=rand(wmin,wmax), stiffness=0.55}
    b:reload(p, ld or rand(0.65,0.85))
    local t = rand(-0.04,0.04)*len
    b:gesture({{x-len/2, y-t, rand(0.45,0.6)},{x+rand(-5,5), y, rand(0.65,0.85)},{x+len/2, y+t, rand(0.3,0.5)}}, {wobble=1.5, orient="along", ramps={0.2,0.3}, clip=clipM})
  end
end
-- blue sky reflected, upper right, reaching to willow edge
cloudpatch(770, 40, 115, 45, cBlue, 26, 12, 20)
cloudpatch(830, 105, 50, 30, cBlue, 8, 10, 16)
-- left-middle blue gap
cloudpatch(560, 245, 85, 30, cBlue, 14, 10, 16)
cloudpatch(790, 300, 50, 22, cBlue, 7, 10, 14)
-- lavender cloud shadows
cloudpatch(660, 225, 100, 22, cLav, 14, 10, 16)
cloudpatch(500, 45, 55, 35, cLav, 10, 10, 16)
cloudpatch(600, 95, 70, 18, cLav, 8, 9, 14, 0.6)
-- warm rose glow lower, bridging to rose band
cloudpatch(680, 320, 110, 30, cRose, 14, 12, 18)

--@ chunk 113
cloudM = (ellipse(770,40,135,60) + ellipse(830,105,65,40) + ellipse(560,245,105,40) + ellipse(790,300,65,32) + ellipse(660,225,120,32) + ellipse(500,45,75,45) + ellipse(600,95,90,28) + ellipse(680,320,130,40)) - farAvoid
blend(cloudM:soften(12), {angle=0.0})
blend(cloudM:shrink(8):soften(12), {angle=0.04})

--@ chunk 114
local choices = {{pearl,2},{cWarm,2},{skycream,1.5},{cBlue,1.5},{cLav,1.5},{cRose,1}}
local clipM = everywhere() - farAvoid
local n=0
for i=1,170 do
  local y = rand(0, 370)
  local l, r = bandL(y), bandR(y)
  if l and r then
    local x = rand(l-10, r+15)
    local p = pickpile(choices)
    local b = brush{kind="filbert", width=rand(6,12), stiffness=0.6}
    b:reload(p, rand(0.35,0.6))
    local len = rand(25, 75)
    local t = rand(-1.5,1.5)
    b:gesture({{x-len/2, y-t, rand(0.4,0.55)},{x, y, rand(0.55,0.75)},{x+len/2, y+t, 0.3}}, {wobble=0.8, orient="along", ramps={0.2,0.35}, clip=clipM})
    n=n+1
  end
end
print(n)

--@ chunk 115
local top = (skyM:grow(30) * mask(function(x,y) return y<380 and 1 or 0 end)):soften(15) - farAvoid
blend(top, {angle=0.0})

--@ chunk 116
local zR = (rect(865,0,140,290) - skyM:shrink(25)):soften(10)
local zL = (rect(370,0,165,345) - skyM:shrink(20)):soften(10)
for i=1,2 do
  rg = rag{width=45}
  rg:dip(0.85)
  rg:wipe(zR, {pressure=0.8, angle=math.pi/2 + (i-1.5)*0.3, passes=2, refold=0.15, seed=300+i})
  rg = rag{width=45}
  rg:dip(0.85)
  rg:wipe(zL, {pressure=0.8, angle=math.pi/2 + (i-1.5)*0.3, passes=2, refold=0.15, seed=310+i})
end

--@ chunk 117
for _,p in ipairs({{588,146},{640,170},{700,175},{770,165},{520,150},{450,152}}) do print(p[1],p[2],padsNow:at(p[1],p[2]), rafts:at(p[1],p[2]), drying(p[1],p[2])) end

--@ chunk 118
local ring = farAvoid:grow(12) - padsNow:grow(2)
local choices = {{cBlue,1.5},{cLav,1.5},{pearl,1.5},{cRose,1},{cWarm,1}}
local n=0
for i=1,90 do
  local x, y = rand(370,810), rand(120,210)
  if ring:at(x,y) > 0.5 then
    local p = pickpile(choices)
    local b = brush{kind="filbert", width=rand(6,10), stiffness=0.6}
    b:reload(p, rand(0.4,0.6))
    local len = rand(20,50)
    b:gesture({{x-len/2,y,0.45},{x,y,0.65},{x+len/2,y,0.3}}, {wobble=0.6, orient="along", ramps={0.2,0.3}, clip=ring})
    n=n+1
  end
end
print(n)
blend(ring:soften(6), {angle=0})

--@ chunk 119
print(wait(12*24*60)); for _,p in ipairs({{700,40},{560,245},{680,320},{450,152},{600,150}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 120
local n=0
for i=1,70 do
  local x = rand(345, 470)
  local y = rand(60, 200)
  local t = clamp((470 - x)/110, 0, 1)  -- 1 = deep in willow
  local r = math.random()
  local p
  if t > 0.6 then p = (r<0.5) and dk1 or ((r<0.8) and dk3 or dk2)
  elseif t > 0.3 then p = (r<0.4) and dk3 or ((r<0.75) and eM or dk1)
  else p = (r<0.5) and wC or eM end
  vgest(x, y, rand(40,100), p, rand(7,12), rand(0.45,0.7), rand(0.5,0.8), rand(-3,3))
  n=n+1
end
print(n)

--@ chunk 121
blend(rect(345,55,125,250):soften(12), {angle=math.pi/2})

--@ chunk 122
-- strokes crossing the top boundary, from the top edge down
for i=1,22 do
  local x = rand(340, 480)
  local r = math.random()
  local p = (r<0.35) and dk1 or ((r<0.65) and dk3 or ((r<0.85) and eM or dk2))
  vgest(x, rand(-10,30), rand(60,130), p, rand(6,11), rand(0.4,0.6), rand(0.45,0.7), rand(-3,3))
end
-- right edge: lighter strokes fading into band
for i=1,16 do
  local x = rand(455, 505)
  local p = pickpile({{wC,2},{eM,1},{cLav,2},{cBlue,1.5}})
  vgest(x, rand(20,260), rand(50,110), p, rand(6,10), rand(0.35,0.55), rand(0.4,0.65), rand(-2,2))
end
local m = rect(330,0,190,310):roughen(25,60,77):soften(18)
blend(m, {angle=math.pi/2})

--@ chunk 123
for i=1,30 do
  local x = rand(330, 500)
  local y = rand(0, 200)
  local p = pickpile({{fr1,2},{fr2,2},{fr3,2},{wC,1}})
  local b = brush{kind="filbert", width=rand(3,6), stiffness=0.6}
  b:reload(p, rand(0.5,0.75))
  local len = rand(50,140); local lean = rand(-4,4)
  b:gesture({{x,y,0.55},{x+lean*0.5,y+len*0.5,0.7},{x+lean,y+len,0.35}}, {wobble=1.2, orient="along", ramps={0.15,0.35}})
end

--@ chunk 124
local n=0
for y = -10, 380, 14 do
  local br = bandR(math.max(0,y)) or 860
  for k=1,2 do
    local d = rand(-60, 30)   -- negative = into the band
    local x = br + d
    local p
    if d < -30 then p = pickpile({{cLav,2},{cBlue,2},{wC,1}})
    elseif d < 0 then p = pickpile({{wC,2},{eM,2},{cLav,1}})
    else p = pickpile({{eM,2},{dk3,1.5},{dk1,1}}) end
    local ld = d < -30 and rand(0.3,0.45) or rand(0.4,0.65)
    vgest(x, y + rand(-6,6), rand(40,100), p, rand(6,11), ld, rand(0.45,0.7), rand(-3,3))
    n=n+1
  end
end
print(n)
local m = (rect(760,0,150,400) * (skyM:grow(40) - skyM:shrink(70))):roughen(15,50,88):soften(12) - padsNow:grow(4)
blend(m, {angle=math.pi/2})

--@ chunk 125
local m = rect(715, 340, 130, 90):soften(6)
rg = rag{width=30}
rg:dip(0.85)
rg:wipe(m, {pressure=0.8, angle=0, passes=3, refold=0.12, seed=401})
for _,p in ipairs({{800,160},{820,165},{790,170}}) do print(padsNow:at(p[1],p[2]), drying(p[1],p[2])) end

--@ chunk 126
for i=1,14 do
  local x = rand(778, 845)
  local t = (x-778)/67
  local p = (t<0.4) and pickpile({{cLav,2},{pearl,1},{cBlue,1}}) or pickpile({{cLav,2},{wC,2}})
  vgest(x, rand(120,150), rand(40,75), p, rand(7,11), rand(0.4,0.6), rand(0.5,0.7), rand(-2,2))
end
blend(rect(768,120,90,75):roughen(8,30,9):soften(10), {angle=math.pi/2})

--@ chunk 127
local m = rect(762,112,100,90):roughen(6,30,10):soften(6)
rg = rag{width=30}
rg:dip(0.9)
rg:wipe(m, {pressure=0.85, angle=0, passes=3, refold=0.12, seed=402})
rg:refold(); rg:dip(0.9)
rg:wipe(m, {pressure=0.85, angle=math.pi/2, passes=2, refold=0.12, seed=403})

--@ chunk 128
hstroke(pS, 788, 826, 172, 5, 0.7, 0.7, 0.5)
hstroke(pM, 792, 818, 168, 4, 0.6, 0.6, -0.3)
hstroke(pDG, 790, 828, 176, 2.5, 0.6, 0.6, 0)
hstroke(pBlu, 832, 848, 171, 4, 0.5, 0.6, 0)

--@ chunk 129
for _,p in ipairs({{450,830},{560,830},{700,850},{850,900},{400,950},{620,960}}) do print(p[1],p[2],drying(p[1],p[2]), padsNow:at(p[1],p[2])) end

--@ chunk 130
-- a loose near pad: a few broken horizontal strokes, notch implied by gap
function loosepad(cx, cy, w, h, base, light, shade)
  local rows = math.random(3,4)
  for i=1,rows do
    local f = (i-0.5)/rows*2-1
    local half = w/2*math.sqrt(math.max(0.1,1-f*f))*rand(0.75,1.05)
    local yy = cy + f*h*0.4
    local gap = (math.random()<0.5) and rand(0.25,0.55) or nil
    local bw = clamp(h/rows*1.1, 4, 14)
    if gap then
      local xg = cx - half + 2*half*gap
      hstroke(base, cx-half, xg-rand(3,8), yy, bw, rand(0.55,0.75), rand(0.55,0.75), rand(-1.5,1.5))
      hstroke(base, xg+rand(3,8), cx+half, yy, bw, rand(0.55,0.75), rand(0.55,0.75), rand(-1.5,1.5))
    else
      hstroke(base, cx-half, cx+half, yy, bw, rand(0.55,0.75), rand(0.55,0.75), rand(-1.5,1.5))
    end
  end
  -- light stroke on top part, partial
  local lx = cx + rand(-0.25,0.1)*w
  hstroke(light, lx - w*rand(0.15,0.3), lx + w*rand(0.1,0.25), cy - h*0.2, clamp(h*0.18,3,8), rand(0.5,0.7), 0.55, 0)
  -- thin shadow under
  hstroke(shade, cx - w*rand(0.3,0.45), cx + w*rand(0.2,0.4), cy + h*0.5, clamp(h*0.1,2,5), 0.55, 0.5, 0)
end
loosepad(470, 805, 150, 34, pM, pL, pDG)
loosepad(360, 850, 120, 30, pS, pBlu, pDG)
loosepad(560, 905, 170, 40, pM, pCool, pDG)
loosepad(390, 960, 140, 34, pV, pBlu, pDG)
loosepad(820, 905, 150, 36, pS, pCool, pDG)
loosepad(940, 860, 110, 30, pM, pL, pDG)

--@ chunk 131
local m = ellipse(470,805,95,30)+ellipse(360,850,80,26)+ellipse(560,905,105,32)+ellipse(390,960,90,28)+ellipse(820,905,95,30)+ellipse(940,860,75,26)
for i=1,2 do
  rg = rag{width=40}
  rg:dip(0.9)
  rg:wipe(m, {pressure=0.85, angle=(i-1)*0.4, passes=2, refold=0.12, seed=500+i})
end
print(bf:mark_width(0.65), brush{kind="filbert",width=14}:mark_width(0.65))

--@ chunk 132
function pad3(cx, cy, w, h, base, light, shade, notch)
  local rows = 3
  local sp = h/3
  local bw = h/2.3
  for i=1,rows do
    local f = (i-2)  -- -1,0,1
    local half = w/2*(f==0 and 1 or rand(0.72,0.85))
    local yy = cy + f*sp + rand(-1,1)
    local x0, x1 = cx-half*rand(0.95,1.05), cx+half*rand(0.95,1.05)
    if notch and f==notch then
      local xg = cx + rand(-0.1,0.15)*w
      hstroke(base, x0, xg-4, yy, bw, rand(0.65,0.8), rand(0.6,0.75), rand(-1,1))
      hstroke(base, xg+8, x1, yy, bw, rand(0.65,0.8), rand(0.6,0.75), rand(-1,1))
    else
      hstroke(base, x0, x1, yy, bw, rand(0.65,0.8), rand(0.6,0.75), rand(-1,1))
    end
  end
  local lx = cx + rand(-0.25,0.05)*w
  hstroke(light, lx - w*rand(0.18,0.28), lx + w*rand(0.12,0.22), cy - sp*0.7, bw*0.55, rand(0.5,0.65), 0.55, 0)
  hstroke(shade, cx - w*rand(0.25,0.38), cx + w*rand(0.2,0.35), cy + sp*1.45, bw*0.3, 0.5, 0.5, 0)
end
pad3(520, 880, 165, 40, pM, pCool, pDG, -1)

--@ chunk 133
local m = ellipse(520,880,100,34)
for i=1,2 do
  rg = rag{width=35}
  rg:dip(0.9)
  rg:wipe(m, {pressure=0.85, angle=(i-1)*0.3, passes=2, refold=0.12, seed=520+i})
end

--@ chunk 134
print(debug == nil)

--@ chunk 135
function nearPad(cx, cy, w, h, n, choices, ld)
  for i=1,n do
    local u, v
    repeat u=rand(-1,1); v=rand(-1,1) until u*u+v*v<1
    local y = cy + v*h*0.42
    local half = w/2*math.sqrt(1-v*v)
    local x = cx + u*half*0.5
    local len = 2*half*rand(0.45,0.85)
    local p = pickpile(choices)
    local b = brush{kind="filbert", width=clamp(h*rand(0.32,0.45),4,22), stiffness=0.65}
    b:reload(p, rand(ld or 0.5, (ld or 0.5)+0.25))
    local t = rand(-1,1)
    b:gesture({{x-len/2,y-t,0.5},{x,y,rand(0.65,0.8)},{x+len/2,y+t,0.35}}, {wobble=0.6, orient="along", ramps={0.2,0.3}})
  end
end
nearPad(430, 952, 160, 46, 9, {{pM,3},{pS,2},{pCool,1.5},{pBlu,1}})
hstroke(pL, 380, 440, 938, 7, 0.55, 0.55, 0)
hstroke(pDG, 370, 470, 975, 4, 0.5, 0.5, 0)

--@ chunk 136
for i=1,4 do
  local x, y = rand(380,480), rand(935,968)
  hstroke(pickpile({{pBlu,2},{pLav,1}}), x-rand(10,25), x+rand(10,25), y, rand(4,7), 0.4, 0.5, 0)
end
blend(ellipse(430,952,95,34):soften(8), {angle=0})
-- restate the white flower at 470,885 crisply over the green tint
flower2(470, 883, "white", 0.9)

--@ chunk 137
local m = ellipse(430,952,105,40)
for i=1,3 do
  rg = rag{width=40}
  rg:dip(0.9)
  rg:wipe(m, {pressure=0.85, angle=(i-2)*0.3, passes=2, refold=0.12, seed=540+i})
end

--@ chunk 138
function brokenRaft2(pts, n, spread, choices, ld)
  for i=1,n do
    local t = rand(0,1)
    local seg = (#pts-1)*t; local k = math.min(#pts-1, math.floor(seg)+1); local u = seg-(k-1)
    local x = lerp(pts[k][1], pts[k+1][1], u); local y = lerp(pts[k][2], pts[k+1][2], u)
    local w,h = psize(y)
    y = y + randn(0, spread*h)
    local len = w*rand(0.25,0.6)
    local p = pickpile(choices)
    local b = brush{kind="filbert", width=clamp(h*rand(0.25,0.4),3,16), stiffness=0.7}
    b:reload(p, rand(ld, ld+0.25))
    local tl = rand(-1,1)
    b:gesture({{x-len/2,y-tl,0.5},{x,y,0.7},{x+len/2,y+tl,0.4}}, {wobble=0.5, orient="along", ramps={0.2,0.3}})
  end
end
local ch = {{pM,3},{pCool,2},{pS,2},{pBlu,1.5},{pL,0.7},{pLav,0.8}}
brokenRaft2({{330,940},{430,955},{520,945}}, 16, 0.5, ch, 0.45)
brokenRaft2({{380,800},{470,812},{560,800}}, 10, 0.5, ch, 0.4)
brokenRaft2({{560,960},{640,985}}, 6, 0.4, ch, 0.4)

--@ chunk 139
local ch = {{pS,2},{pCool,2},{pBlu,1.2},{pM,1.5},{pV,0.8}}
local spots = {{690,840,110,26},{900,805,90,16},{860,850,120,26},{800,975,120,20},{625,980,60,10}}
for _,s in ipairs(spots) do
  for i=1,6 do
    local x = s[1] + rand(-0.4,0.4)*s[3]; local y = s[2] + rand(-0.5,0.5)*s[4]
    local len = s[3]*rand(0.25,0.55)
    local p = pickpile(ch)
    local b = brush{kind="filbert", width=rand(5,10), stiffness=0.7}
    b:reload(p, rand(0.35,0.55))
    b:gesture({{x-len/2,y,0.45},{x,y,0.65},{x+len/2,y,0.35}}, {wobble=0.5, orient="along", ramps={0.2,0.3}, clip=everywhere()-flowersM})
  end
end
flower2(640, 972, "pink", 0.9)

--@ chunk 140
local ch = {{pS,2},{pCool,2},{pM,1.5},{pBlu,1}}
for _,s in ipairs({{700,836,60},{740,846,50},{675,850,45},{725,830,40}}) do
  local p = pickpile(ch)
  local b = brush{kind="filbert", width=rand(6,9), stiffness=0.7}
  b:reload(p, rand(0.4,0.55))
  b:gesture({{s[1]-s[3]/2,s[2],0.45},{s[1],s[2],0.65},{s[1]+s[3]/2,s[2],0.35}}, {wobble=0.5, orient="along", ramps={0.2,0.3}, clip=everywhere()-flowersM})
end

--@ chunk 141
print(wait(14*24*60)); for _,p in ipairs({{430,950},{640,972},{860,150},{420,120},{700,840}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 142
print(drying(340,962), drying(400,916))
gz = pile{{"ultramarine blue",0.6},{"cobalt violet",0.6},{"lead white",0.15}, medium=0.55}
bg = brush("filbert", 7)
bg:load(gz, 0.5)
bg:gesture({{316,963,0.3},{340,961,0.6},{372,962,0.2}}, {wobble=1})
bg:gesture({{372,970,0.3},{400,968,0.55},{432,969,0.15}}, {wobble=1})

--@ chunk 143
rg = rag{width=30}
rg:dip(0.9)
rg:wipe({{312,963},{376,961}}, {pressure=0.85})
rg:wipe({{368,969},{436,969}}, {pressure=0.85})
rg:refold(); rg:dip(0.9)
rg:wipe({{312,963},{376,961}}, {pressure=0.85})
rg:wipe({{368,969},{436,969}}, {pressure=0.85})

--@ chunk 144
gz2 = pile{{"cobalt blue",0.25},{"cobalt violet",0.35},{"lead white",0.1}, medium=0.75}
bg:reload(gz2, 0.3)
bg:gesture({{316,962,0.25},{345,960,0.4},{372,963,0.1}}, {wobble=1.5})

--@ chunk 145
local pills = {{318,416,915},{370,436,969},{521,578,779},{663,716,909},{694,766,957},{809,886,971},{102,178,790}}
for i,p in ipairs(pills) do
  bg:load(gz2, rand(0.25,0.35))
  local a, b, y = p[1], p[2], p[3]
  local s = rand(0, 0.35)
  local x0 = a + (b-a)*s
  local x1 = b - (b-a)*rand(0,0.3)
  bg:gesture({{x0,y+rand(-1.5,1.5),0.25},{(x0+x1)/2,y+rand(-1.5,1.5),rand(0.35,0.5)},{x1,y+rand(-1.5,1.5),0.1}}, {wobble=1.5})
end

--@ chunk 146
rg:refold(); rg:dip(0.6)
rg:wipe({{660,909},{720,909}}, {pressure=0.6})
rg:wipe({{805,971},{890,971}}, {pressure=0.6})
rg:wipe({{518,779},{582,779}}, {pressure=0.5})

--@ chunk 147
print(wait(5*24*60)); print(drying(345,962), drying(700,957))

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
