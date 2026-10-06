-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=560, aspect=1.4, linen={16,13}, seed=7, ground={{pile={{"lead white",1}}, um=110, apply="knife", texture=0.3}, {pile={{"lead white",3},{"yellow ochre",1},{"red earth",0.35},{"raw umber",0.3}}, um=20, apply="brush"}}}
print(W,H); print(table.concat(tubes(), ", "))

--@ chunk 2

h = pencil("HB")
h:rule({0,400},{1000,400},{pressure=0.25})
-- spit of land at right
h:sketch({{600,402},{660,396},{720,392},{760,370},{790,330},{830,305},{870,310},{900,330},{930,360},{960,378},{1000,380}}, {pressure=0.3})
-- far shore left
h:sketch({{0,392},{80,388},{160,390},{220,386},{300,392},{420,396},{540,399}}, {pressure=0.2})
-- mud bank foreground left
h:sketch({{0,560},{120,540},{260,548},{380,575},{470,610},{540,660},{600,714}}, {pressure=0.3})
-- middle sandbar
h:sketch({{120,468},{240,458},{380,462},{520,470},{640,478}}, {pressure=0.25})
h:sketch({{700,500},{820,492},{1000,488}}, {pressure=0.25})
-- boat hull
h:sketch({{190,528},{230,540},{280,542},{320,532},{330,520}}, {pressure=0.35})
h:sketch({{190,528},{215,520},{280,518},{330,520}}, {pressure=0.3})
h:line({{262,520},{250,430}}, {pressure=0.35})
-- stakes
for i,p in ipairs({{600,452,18},{640,462,22},{690,476,28},{750,494,36},{830,520,46}}) do h:line({{p[1],p[2]},{p[1]+2,p[2]+p[3]}},{pressure=0.3}) end

--@ chunk 3
sky1 = pile{{"lead white",3},{"smalt",1.2},{"raw umber",0.25},{"red earth",0.1}}
sky2 = pile{{"lead white",4},{"cobalt blue",0.5},{"red earth",0.25},{"raw umber",0.1}}
sky3 = pile{{"lead white",5},{"yellow ochre",0.8},{"vermilion",0.2},{"cobalt blue",0.1}}
sky4 = pile{{"lead white",6},{"chrome yellow",0.7},{"yellow ochre",0.5},{"vermilion",0.12}}
local n = noise{seed=3, period=300, octaves=3}
local function band(y0, y1) return mask(function(x,y) local w = n(x,y)*18; return (y > y0 + w and y < y1 + w) and 1 or 0 end) end
work(band(-20, 150), {hand="body", tool="filbert 14", pile=sky1, angle=function(x,y) return 0.05*math.sin(x/200) end, coverage=2.5, length={40,110}, fill=true})
work(band(140, 255), {hand="body", tool="filbert 14", pile=sky2, angle=0.02, coverage=2.5, length={40,110}, fill=true})
work(band(245, 345), {hand="body", tool="filbert 14", pile=sky3, angle=-0.02, coverage=2.5, length={40,110}, fill=true})
work(rect(0, 335, 1000, 68), {hand="body", tool="filbert 12", pile=sky4, angle=0, coverage=2.5, length={40,120}, fill=true, clip=rect(0,0,1000,401)})

--@ chunk 4
skyTop = pile{{"lead white",2},{"cobalt blue",1},{"smalt",0.4},{"red earth",0.15},{"raw umber",0.15}}
skyMid = pile{{"lead white",3},{"cobalt blue",0.5},{"red earth",0.3},{"raw umber",0.05}}
local n = noise{seed=5, period=260, octaves=3}
local function band(y0, y1) return mask(function(x,y) local w = n(x,y)*20; return (y > y0 + w and y < y1 + w) and 1 or 0 end) end
work(band(-20, 110), {hand="body", tool="filbert 14", pile=skyTop, angle=0.03, coverage=2.5, length={50,120}, fill=true})
work(band(95, 215), {hand="body", tool="filbert 14", pile=skyMid, angle=-0.03, coverage=2.2, length={50,120}, fill=true})
skyM = rect(0,0,1000,401)
blend(skyM, {angle=0})
blend(skyM, {angle=0.25})

--@ chunk 5
mudL = poly({{-10,556},{60,546},{130,538},{200,540},{260,547},{330,560},{390,575},{440,595},{480,617},{520,645},{555,680},{575,720},{-10,720}}, true):roughen(4, 40, 11)
mudR = poly({{1010,556},{960,562},{905,580},{860,610},{835,650},{822,690},{818,720},{1010,720}}, true):roughen(4, 40, 12)
bar = poly({{110,470},{170,462},{260,457},{360,459},{470,464},{560,469},{640,476},{560,479},{420,476},{280,474},{180,475}}, true):roughen(2, 30, 13)
barR = poly({{680,500},{760,492},{860,488},{1010,484},{1010,512},{900,508},{800,506},{720,505}}, true):roughen(2, 30, 14)
mudAll = mudL + mudR + bar + barR
waterM = rect(0,400,1000,320) - mudAll
wat1 = pile{{"lead white",5},{"chrome yellow",0.6},{"yellow ochre",0.6},{"vermilion",0.12},{"raw umber",0.1}}
wat2 = pile{{"lead white",4},{"yellow ochre",0.6},{"vermilion",0.15},{"cobalt blue",0.25},{"raw umber",0.1}}
wat3 = pile{{"lead white",3},{"cobalt blue",0.6},{"red earth",0.3},{"raw umber",0.15}}
wat4 = pile{{"lead white",2},{"cobalt blue",0.9},{"smalt",0.3},{"red earth",0.2},{"raw umber",0.25}}
local c = rect(0,399,1000,321)
work(waterM * rect(0,398,1000,62), {hand="body", tool="filbert 10", pile=wat1, angle=0, coverage=2.5, length={40,120}, fill=true, clip=c})
work(waterM * rect(0,455,1000,70), {hand="body", tool="filbert 12", pile=wat2, angle=0, coverage=2.5, length={40,120}, fill=true, clip=c})
work(waterM * rect(0,520,1000,90), {hand="body", tool="filbert 14", pile=wat3, angle=0, coverage=2.5, length={40,120}, fill=true, clip=c})
work(waterM * rect(0,605,1000,115), {hand="body", tool="filbert 14", pile=wat4, angle=0, coverage=2.5, length={40,120}, fill=true, clip=c})

--@ chunk 6
blend(rect(0,401,1000,320), {angle=0}); blend(rect(0,401,1000,320), {angle=0.08}); print(wait(100)); print(drying(500,600), drying(500,300))

--@ chunk 7
print(wait(120)); for _,p in ipairs({{500,600},{200,600},{500,470},{500,420},{900,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 8
mudDark = pile{{"raw umber",2},{"red earth",0.4},{"bone black",0.3},{"lead white",0.4}, medium=0.1}
mudMid = pile{{"raw umber",1.5},{"red earth",0.4},{"lead white",1},{"cobalt blue",0.2}, medium=0.1}
barP = pile{{"raw umber",1},{"lead white",1.3},{"cobalt blue",0.35},{"red earth",0.25}}
work(mudL, {hand="body", tool="filbert 10", pile=mudDark, angle=function(x,y) return 0.12 + 0.3*(x/600) end, coverage=2.5, length={30,80}, fill=true, edge={found=0.5, soft=0.4, lost=0.1, period=50}})
work(mudR, {hand="body", tool="filbert 10", pile=mudDark, angle=-0.25, coverage=2.5, length={30,80}, fill=true, edge={found=0.5, soft=0.5, period=50}})
work(bar, {hand="detail", tool="round 4", pile=barP, angle=0, coverage=2, length={10,30}, fill=true})
work(barR, {hand="detail", tool="round 5", pile=barP, angle=-0.03, coverage=2, length={10,30}, fill=true})

--@ chunk 9
for _,p in ipairs({{820,340},{900,370},{300,392},{650,395}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 10
local nz = noise{seed=21, period=40, octaves=3}
local nz2 = noise{seed=22, period=12, octaves=2}
local bumps = {{775,70,30},{835,98,34},{880,82,26},{925,55,28},{975,38,30},{720,26,18},{690,14,14}}
function treeTop(x)
  local h = 0
  if x > 600 then h = 4 + math.min(1,(x-600)/80)*6 end
  for _,b in ipairs(bumps) do local d=(x-b[1])/b[3]; h = math.max(h, b[2]*math.exp(-d*d)) end
  if h > 10 then h = h + nz(x, 0)*6 + nz2(x, 3)*4 end
  return 402 - h
end
spit = mask(function(x,y) if x < 596 then return 0 end; return (y > treeTop(x) and y < 406) and 1 or 0 end):roughen(2.5, 9, 23)
farShore = mask(function(x,y) if x > 640 then return 0 end; local t = 399 - (3 + 3*math.sin(x/37) + 2*nz(x,9)) ; if x > 540 then t = t + (x-540)/25 end; return (y > t and y < 401.5) and 1 or 0 end)
farP = pile{{"lead white",2.5},{"cobalt blue",0.5},{"red earth",0.3},{"raw umber",0.3}}
spitP = pile{{"raw umber",1},{"bone black",0.3},{"cobalt blue",0.4},{"lead white",0.7},{"green earth",0.3}}
work(farShore, {hand="detail", tool="round 3", pile=farP, angle=0, coverage=2, length={8,25}, fill=true})
work(spit, {hand="detail", tool="round 4", pile=spitP, angle=function(x,y) return -1.4 + 0.3*math.sin(x/9) end, coverage=2.5, length={6,16}, fill=true, edge="soft"})

--@ chunk 11
print(wait(18*60)); for _,p in ipairs({{820,340},{820,250},{300,100},{500,600},{200,600},{840,380},{300,466}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 12
print(wait(30*60)); for _,p in ipairs({{820,340},{820,250},{300,100},{500,600},{200,600},{840,380},{300,466}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 13
local crowns = {{775,360,30,32},{835,335,36,40},{880,345,30,32},{922,358,28,26},{968,372,26,22},{995,380,18,16},{722,384,20,16},{748,375,20,20},{805,352,24,30},{857,340,22,30}}
local m = rect(596,394,410,12)
for _,c in ipairs(crowns) do m = m + ellipse(c[1],c[2],c[3],c[4]) end
m = m + mask(function(x,y) return (x>596 and y > 402 - math.min(10,(x-596)/8) and y < 406) and 1 or 0 end)
trees = m:roughen(4, 10, 31):roughen(1.5, 4, 32)
treeP = pile{{"raw umber",1},{"bone black",0.5},{"Prussian blue",0.08},{"green earth",0.3},{"lead white",0.35}}
work(trees, {hand="body", tool="filbert 6", pile=treeP, angle=function(x,y) return -1.2 + 0.6*math.sin(x/13+y/17) end, coverage=3, length={8,18}, fill=true, edge={found=0.3, soft=0.6, lost=0.1, period=20}})

--@ chunk 14
work(trees * rect(700,360,310,48), {hand="detail", tool="round 4", pile=treeP, angle=-1.5, coverage=2, length={6,14}, fill=true})
reflM = mask(function(x,y) if y < 404 or y > 490 then return 0 end; local v = trees:at(x, 808 - y); return v * (1 - smoothstep(440, 488, y)) end)
reflP = pile{{"raw umber",1},{"bone black",0.3},{"cobalt blue",0.3},{"lead white",1.3},{"yellow ochre",0.3}}
work(reflM, {hand="body", tool="filbert 5", pile=reflP, angle=math.pi/2, coverage=2, length={10,30}, fill=true, threshold=0.4})

--@ chunk 15
local r = rect(585,405,425,85); blend(r, {angle=0}); blend(r, {angle=0.05})

--@ chunk 16
local nzc = noise{seed=41, period=60, octaves=3}
function lens(x0, x1, yc, th, tilt, seed)
  local nn = noise{seed=seed, period=50, octaves=3}
  return mask(function(x,y)
    if x < x0 or x > x1 then return 0 end
    local t = (x - x0)/(x1 - x0)
    local w = th * math.sin(math.pi*t)^0.6 * (0.8 + 0.4*nn(x, 0))
    local c = yc + tilt*(x-x0) + 4*nn(x, 50)
    return (math.abs(y - c) < w) and 1 or 0 end):roughen(3, 14, seed+1)
end
cloudA = lens(-40, 720, 222, 16, -0.012, 101) + lens(480, 1040, 205, 11, 0.01, 103)
cloudB = lens(40, 380, 300, 5, 0.004, 105) + lens(430, 820, 312, 4, -0.004, 107) + lens(600, 1000, 285, 6, 0, 109)
cloudC = lens(-40, 420, 70, 22, 0.02, 111) + lens(560, 1040, 110, 18, -0.01, 113) + lens(250, 700, 150, 8, 0, 115)
cloudDark = pile{{"lead white",2},{"cobalt blue",0.5},{"red earth",0.4},{"raw umber",0.3}, medium=0.15}
cloudHi = pile{{"lead white",2.2},{"cobalt blue",0.6},{"red earth",0.3},{"raw umber",0.35}, medium=0.15}
cloudWarm = pile{{"lead white",3},{"vermilion",0.25},{"yellow ochre",0.5},{"cobalt blue",0.1}, medium=0.1}
work(cloudC, {hand="body", tool="filbert 8", pile=cloudHi, angle=0.02, coverage=2, length={25,70}, fill=true, edge={soft=0.5, lost=0.4, found=0.1, period=40}})
work(cloudA, {hand="body", tool="filbert 7", pile=cloudDark, angle=-0.01, coverage=2.2, length={25,70}, fill=true, edge={soft=0.5, lost=0.3, found=0.2, period=40}})
work(cloudB, {hand="detail", tool="round 4", pile=cloudWarm, angle=0, coverage=2, length={15,40}, fill=true, edge="soft"})

--@ chunk 17
blend(cloudC:grow(10):soften(6), {angle=0.03}); blend(cloudC:grow(10):soften(6), {angle=-0.05})

--@ chunk 18
local m = cloudA:grow(8):soften(6); blend(m, {angle=0.02}); blend(m, {angle=-0.04}); local m2 = cloudB:grow(6):soften(4); blend(m2, {angle=0})

--@ chunk 19
print(wait(3*24*60)); for _,p in ipairs({{820,340},{820,220},{200,75},{700,450},{200,600},{900,650},{300,466},{300,300}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 20
skyClip = rect(0,0,1000,401) - trees:grow(0.5)
s1 = pile{{"lead white",1.8},{"cobalt blue",1},{"smalt",0.3},{"red earth",0.2},{"raw umber",0.2}}
s2 = pile{{"lead white",3},{"cobalt blue",0.5},{"vermilion",0.1},{"red earth",0.2}}
s3 = pile{{"lead white",4},{"yellow ochre",0.55},{"vermilion",0.22},{"cobalt blue",0.05}}
s4 = pile{{"lead white",6},{"chrome yellow",0.8},{"yellow ochre",0.3},{"vermilion",0.08}}
local n = noise{seed=51, period=280, octaves=3}
local function band(y0, y1) return mask(function(x,y) local w = n(x,y)*15; return (y > y0 + w and y < y1 + w) and 1 or 0 end) end
local o = {hand="body", tool="filbert 14", angle=0, coverage=2.5, length={50,130}, fill=true, clip=skyClip, angle_jitter=0.05}
o.pile=s1; work(band(-30,125) * skyClip, o)
o.pile=s2; work(band(115,245) * skyClip, o)
o.pile=s3; work(band(235,330) * skyClip, o)
o.pile=s4; o.tool="filbert 12"; work(rect(0,320,1000,82) * skyClip, o)

--@ chunk 21
local nu = noise{seed=61, period=140, octaves=4, stretch={0.03, 5}}
local nm = noise{seed=62, period=90, octaves=4, stretch={-0.01, 7}}
local nl = noise{seed=63, period=80, octaves=3, stretch={0, 8}}
cU = mask(function(x,y) if y < 10 or y > 185 then return 0 end; local fade = smoothstep(10,50,y) * (1 - smoothstep(140,185,y)); return (nu(x,y)*fade > 0.18) and 1 or 0 end):roughen(3, 12, 64)
cM = mask(function(x,y) if y < 195 or y > 250 then return 0 end; local fade = smoothstep(195,212,y) * (1 - smoothstep(228,250,y)) * (1 - smoothstep(700,900,x)); return (nm(x,y)*0.6 + fade*0.7 > 0.55) and 1 or 0 end):roughen(2.5, 10, 65)
cL = mask(function(x,y) if y < 270 or y > 335 then return 0 end; local fade = smoothstep(270,285,y) * (1 - smoothstep(315,335,y)); return (nl(x,y)*0.8 + fade*0.5 > 0.55) and 1 or 0 end):roughen(2, 10, 66)
cuP = pile{{"lead white",1.6},{"cobalt blue",0.6},{"red earth",0.4},{"raw umber",0.35}}
cmP = pile{{"lead white",1.8},{"cobalt blue",0.45},{"red earth",0.45},{"vermilion",0.1},{"raw umber",0.3}}
clP = pile{{"lead white",3},{"vermilion",0.35},{"yellow ochre",0.45}}
work(cU * skyClip, {hand="body", tool="filbert 8", pile=cuP, angle=0.03, coverage=2, length={30,90}, fill=true, clip=skyClip, edge={soft=0.6, lost=0.3, found=0.1, period=40}})
work(cM * skyClip, {hand="body", tool="filbert 6", pile=cmP, angle=-0.01, coverage=2, length={30,90}, fill=true, clip=skyClip, edge={soft=0.6, lost=0.2, found=0.2, period=40}})
work(cL * skyClip, {hand="detail", tool="round 4", pile=clP, angle=0, coverage=2, length={20,50}, fill=true, clip=skyClip})

--@ chunk 22
local m = rect(0,0,1000,399) - trees:grow(3); blend(m, {angle=0}); blend(m, {angle=0.12})

--@ chunk 23
wClip = rect(0,401,1000,320) - mudL - mudR - bar - barR
w1 = pile{{"lead white",6},{"chrome yellow",0.8},{"yellow ochre",0.35},{"vermilion",0.08},{"raw umber",0.06}}
w2 = pile{{"lead white",4},{"yellow ochre",0.55},{"vermilion",0.22},{"cobalt blue",0.1},{"raw umber",0.05}}
w3 = pile{{"lead white",3},{"cobalt blue",0.5},{"vermilion",0.1},{"red earth",0.25},{"raw umber",0.08}}
local o = {hand="body", tool="filbert 10", angle=0, coverage=3, length={40,120}, fill=true, clip=wClip, angle_jitter=0.02}
o.pile=w1; work(rect(0,401,1000,42) * wClip, o)
o.pile=w2; work(rect(0,438,1000,52) * wClip, o)
o.pile=w3; o.tool="filbert 12"; work(rect(0,486,1000,80) * wClip, o)

--@ chunk 24
reflM = mask(function(x,y) if y < 402 or y > 520 then return 0 end; return trees:at(x, 804 - y) end) * wClip
reflP = pile{{"raw umber",1},{"bone black",0.35},{"cobalt blue",0.3},{"green earth",0.2},{"lead white",0.9}}
work(reflM, {hand="body", tool="filbert 4", pile=reflP, angle=0, coverage=2.5, length={10,30}, fill=true, clip=wClip, angle_jitter=0.02})
local bm = rect(0,402,1000,100) * wClip
blend(bm, {angle=0})

--@ chunk 25
print(wait(180)); for _,p in ipairs({{850,385},{700,380},{820,300},{500,300}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 26
local holes = ellipse(816,388,9,5) + ellipse(860,386,12,6) + ellipse(906,388,9,5) + ellipse(934,385,5,4)
local gap = (rect(785,366,170,34):roughen(4,10,71) - holes:roughen(2,5,72))
treeP2 = pile{{"raw umber",1},{"bone black",0.4},{"green earth",0.4},{"Prussian blue",0.05},{"lead white",0.3}}
work(gap, {hand="detail", tool="round 4", pile=treeP2, angle=function(x,y) return -1.3 + 0.5*math.sin(x/11) end, coverage=2.5, length={5,12}, fill=true})
work(ellipse(818,306,14,10), {hand="detail", tool="round 4", pile=treeP, angle=-1.2, coverage=3, length={5,10}, fill=true})
local r = brush{kind="rigger", width=1.6, point=1}
r:load(treeP2, 0.7)
for _,t in ipairs({{812,380,398,1.4},{820,381,398,1.0},{856,378,398,1.8},{866,382,398,1.0},{903,381,398,1.4},{910,383,398,0.9},{933,380,398,1.0}}) do
  r:stroke({{t[1],t[2]},{t[1]+rand(-1,1),(t[2]+t[3])/2},{t[1]+rand(-1.5,1.5),t[3]}}, {pressure={0.4*t[4],0.6*t[4]}})
end

--@ chunk 27
mA = pile{{"raw umber",2},{"bone black",0.5},{"red earth",0.5},{"lead white",0.2}}
mB = pile{{"raw umber",1.5},{"red earth",0.4},{"lead white",0.6},{"cobalt blue",0.2}}
mC = pile{{"raw umber",1},{"lead white",1.3},{"cobalt blue",0.3},{"red earth",0.3},{"vermilion",0.05}}
local inner = mudL:shrink(20)
local rimZ = (mudL - mudL:shrink(26)) * rect(14,0,1000,694)
local o = {hand="body", tool="filbert 9", angle=function(x,y) return 0.04 end, coverage=2.5, length={30,90}, fill=true, clip=mudL, angle_jitter=0.04}
o.pile = mA; work(inner * rect(0,630,1000,100), o)
o.pile = mB; work(inner * rect(0,560,1000,85), o)
o.pile = mC; o.tool="filbert 6"; o.length={15,50}; work(rimZ, o)
blend(mudL, {angle=0.03})

--@ chunk 28
print(wait(2*24*60)); for _,p in ipairs({{850,385},{820,340},{200,600},{300,560},{500,300},{800,440}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 29
mudL2 = poly({{-10,566},{80,561},{170,566},{240,569},{300,574},{338,580},{346,589},{400,596},{428,606},{440,620},{468,630},{488,648},{503,670},{518,690},{532,720},{-10,720}}):roughen(2.5, 18, 81)
mudR2 = poly({{1010,578},{950,581},{900,586},{880,594},{862,601},{852,618},{846,640},{836,660},{828,690},{824,720},{1010,720}}):roughen(2.5, 18, 82)
local cut = ((mudL - mudL2) + (mudR - mudR2)):grow(3)
local wc = -(mudL2 + mudR2)
local o = {hand="body", tool="filbert 8", angle=0, coverage=3, length={25,70}, fill=true, clip=wc, angle_jitter=0.02}
o.pile = w3; work(cut * rect(0,520,1000,78), o)
o.pile = wat3; work(cut * rect(0,592,1000,130), o)

--@ chunk 30
local function lensP(cx, cy, rx, ry, seed)
  local nn = noise{seed=seed, period=25, octaves=2}
  return mask(function(x,y) local t = (x-cx)/rx; if math.abs(t) >= 1 then return 0 end; local w = ry*(1-t*t)^0.7*(0.7+0.5*nn(x,0)); return (math.abs(y-cy-2*nn(x,9)) < w) and 1 or 0 end)
end
pools = lensP(110,606,60,4,1) + lensP(250,598,38,3,2) + lensP(60,652,80,6,3) + lensP(320,632,55,4.5,4) + lensP(215,688,75,7,5) + lensP(400,662,40,4,6) + lensP(940,612,45,3.5,7) + lensP(920,668,60,6,8) + lensP(180,630,30,2.5,9)
pools = pools * (mudL2:shrink(6) + mudR2:shrink(6))
poolP = pile{{"lead white",2.2},{"cobalt blue",0.8},{"smalt",0.2},{"red earth",0.2},{"raw umber",0.25}}
work(pools, {hand="detail", tool="round 3", pile=poolP, angle=0, coverage=3, length={10,40}, fill=true, clip=true})

--@ chunk 31
mudArea = (mudL2 + mudR2) - pools:grow(1)
mG = pile{{"raw umber",1.5},{"green earth",0.6},{"bone black",0.2},{"lead white",0.3}}
local nn = noise{seed=91, period=90, octaves=3, stretch={0,4}}
work(mudArea, {hand="body", tool="flat 5", pile=mA, angle=0.02, coverage=0.9, load_at=function(x,y) return 0.25 + 0.4*nn:at01(x,y) end, length={10,35}, clip=mudArea, angle_jitter=0.05})
work(mudArea, {hand="body", tool="flat 4", pile=mG, angle=0.0, coverage=0.4, length={8,25}, load=0.4, clip=mudArea, angle_jitter=0.05})
work(mudArea * rect(0,560,1000,70), {hand="body", tool="flat 3", pile=mC, angle=0.0, coverage=0.5, length={8,30}, load=0.35, clip=mudArea, angle_jitter=0.04})

--@ chunk 32
blend(mudArea, {angle=0.02})

--@ chunk 33
chanClip = rect(0,515,1000,210) - mudL2 - mudR2
c1 = pile{{"lead white",3},{"cobalt blue",0.55},{"vermilion",0.08},{"red earth",0.22},{"raw umber",0.1}}
c2 = pile{{"lead white",2.2},{"cobalt blue",0.8},{"red earth",0.2},{"raw umber",0.2},{"smalt",0.2}}
c3 = pile{{"lead white",1.6},{"cobalt blue",0.95},{"smalt",0.3},{"red earth",0.2},{"raw umber",0.3}}
local o = {hand="body", tool="filbert 9", angle=0, coverage=3, length={30,90}, fill=true, clip=chanClip, angle_jitter=0.02}
o.pile=c1; work(rect(0,515,1000,60) * chanClip, o)
o.pile=c2; work(rect(0,570,1000,70) * chanClip, o)
o.pile=c3; work(rect(0,635,1000,90) * chanClip, o)
blend(chanClip, {angle=0})

--@ chunk 34
print(wait(2*24*60)); for _,p in ipairs({{200,640},{450,640},{700,650},{900,640},{850,385},{800,440}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 35
mudOnly = (mudL2 + mudR2) - pools
gz = pile{{"raw umber",1.5},{"bone black",0.6},{"red earth",0.3}, medium=0.7}
work(mudOnly * rect(0,595,1000,130), {hand="glaze", pile=gz, angle=0, coverage=1.6, clip=mudOnly})
work(mudOnly * rect(0,650,1000,80), {hand="glaze", pile=gz, angle=0.02, coverage=1.4, clip=mudOnly})
blend(mudOnly, {angle=0})
blend(mudOnly, {angle=0.1})

--@ chunk 36
mudAll2 = mudL2 + mudR2
mudD = pile{{"raw umber",2},{"bone black",0.6},{"cobalt blue",0.3},{"lead white",0.45}}
mudDM = pile{{"raw umber",1.5},{"bone black",0.3},{"cobalt blue",0.35},{"lead white",1},{"red earth",0.2}}
mudL3 = pile{{"raw umber",1},{"lead white",1.8},{"cobalt blue",0.45},{"red earth",0.25},{"vermilion",0.05}}
local o = {hand="body", tool="filbert 9", angle=0.01, coverage=2.8, length={30,100}, fill=true, clip=mudAll2, angle_jitter=0.03}
o.pile=mudD; work(mudAll2 * rect(0,640,1000,90), o)
o.pile=mudDM; work(mudAll2 * rect(0,596,1000,50), o)
o.pile=mudL3; o.tool="filbert 7"; work(mudAll2 * rect(0,555,1000,45), o)
blend(mudAll2, {angle=0})
blend(mudAll2, {angle=0.06})

--@ chunk 37
sheen = pile{{"lead white",2.4},{"cobalt blue",0.7},{"red earth",0.25},{"raw umber",0.15}}
darkLine = pile{{"raw umber",1.5},{"bone black",0.8},{"cobalt blue",0.2}}
local f = brush{kind="flat", width=3, stiffness=0.5}
local d = brush{kind="flat", width=2.2, stiffness=0.5}
local count = 0
for i = 1, 70 do
  local x = rand(0, 900); local y = rand(565, 712)
  if mudAll2:at(x, y) > 0.9 then
    local len = rand(25, 110) * (0.6 + (y-560)/150)
    local x2 = x + len
    local pts = {}
    for k = 0, 4 do local t = k/4; pts[#pts+1] = {lerp(x, x2, t), y + (t-0.5)*len*0.015 + randn(0, 0.4)} end
    if i % 3 == 0 then
      d:reload(darkLine, 0.5); d:stroke(pts, {pressure={0.5, 0.2}, ramps={0.2,0.5}, clip=mudAll2})
    else
      f:reload(sheen, 0.45); f:stroke(pts, {pressure={0.45, 0.15}, ramps={0.2,0.5}, clip=mudAll2})
    end
    count = count + 1
  end
end
print(count)

--@ chunk 38
local count = 0
for i = 1, 160 do
  local y = 560 + 155 * (rand(0,1)^1.3)
  local x = rand(-20, 960)
  if mudAll2:at(math.max(1,x), y) > 0.9 then
    local depth = (y - 555) / 160   -- 0 far .. 1 near
    local w = 1.2 + 5 * depth^1.5
    local len = rand(20, 80) * (0.5 + 1.5*depth)
    local pts = {}
    for k = 0, 4 do local t = k/4; pts[#pts+1] = {x + len*t, y + (t-0.5)*len*0.01 + randn(0, 0.3*w)} end
    local b = brush{kind="flat", width=w, stiffness=0.45}
    if rand(0,1) < 0.35 then b:load(darkLine, 0.4) else b:load(sheen, 0.3 + 0.2*rand(0,1)) end
    b:stroke(pts, {pressure={rand(0.3,0.6), 0.1}, ramps={0.15,0.6}, clip=mudAll2})
    count = count + 1
  end
end
print(count)

--@ chunk 39
mudL4 = poly({{-10,566},{80,561},{170,566},{260,569},{360,572},{450,574},{505,577},{516,583},{500,598},{486,606},{476,622},{452,646},{436,668},{414,692},{398,720},{-10,720}}):roughen(2.5, 18, 91)
mudR4 = poly({{1010,578},{900,580},{800,582},{735,584},{716,589},{728,601},{748,618},{772,640},{800,664},{826,688},{850,720},{1010,720}}):roughen(2.5, 18, 92)
mud4 = mudL4 + mudR4
local add = mud4 - mudAll2:shrink(2)
local o = {hand="body", tool="filbert 9", angle=0.01, coverage=3, length={25,80}, fill=true, clip=mud4, angle_jitter=0.03}
o.pile=mudD; work(add * rect(0,640,1000,90), o)
o.pile=mudDM; work(add * rect(0,596,1000,50), o)
o.pile=mudL3; o.tool="filbert 7"; work(add * rect(0,555,1000,45), o)
blend(mud4, {angle=0})

--@ chunk 40
local chan = -(mud4)
local rem = (mudAll2 - mud4):grow(3) * rect(0,560,1000,170)
local o = {hand="body", tool="filbert 7", angle=0, coverage=3, length={20,60}, fill=true, clip=chan * rect(0,560,1000,170), angle_jitter=0.02}
o.pile=c2; work(rem * rect(0,560,1000,80), o)
o.pile=c3; work(rem * rect(0,635,1000,90), o)

--@ chunk 41
print(wait(36*60)); for _,p in ipairs({{200,640},{480,610},{900,650},{450,700},{600,650},{820,340}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 42
chanM = rect(0,562,1000,160) - mud4
local o = {hand="body", tool="filbert 9", angle=0, coverage=3.2, length={25,80}, fill=true, clip=chanM, angle_jitter=0.02, load=0.9}
o.pile=c1; work(chanM * rect(0,560,1000,40), o)
o.pile=c2; work(chanM * rect(0,595,1000,55), o)
o.pile=c3; work(chanM * rect(0,645,1000,80), o)
blend(chanM, {angle=0})
blend(chanM, {angle=0.1})

--@ chunk 43
print(wait(2*24*60)); for _,p in ipairs({{200,640},{480,610},{900,650},{450,700},{600,650},{620,580}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 44
gz2 = pile{{"raw umber",1},{"bone black",0.8},{"cobalt blue",0.4}, medium=0.7}
work(mud4 * rect(0,615,1000,110), {hand="glaze", pile=gz2, angle=0, coverage=1.5, clip=mud4})
work(mud4 * rect(0,660,1000,70), {hand="glaze", pile=gz2, angle=0.03, coverage=1.5, clip=mud4})
blend(mud4, {angle=0})
blend(mud4, {angle=-0.08})

--@ chunk 45
print(wait(3*24*60)); for _,p in ipairs({{200,640},{480,610},{900,650},{450,700},{600,650},{620,580}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 46
warmSheen = pile{{"lead white",4},{"yellow ochre",0.45},{"vermilion",0.15},{"cobalt blue",0.15},{"raw umber",0.05}}
violetSheen = pile{{"lead white",3},{"cobalt blue",0.5},{"red earth",0.25},{"raw umber",0.1}}
local nL = 0
local y = 569
while y < 640 do
  local depth = (y - 560) / 80
  local w = 0.9 + 2.2*depth
  local x = rand(-30, 200)
  while x < 1000 do
    local len = rand(60, 260) * (0.7 + depth)
    local x2 = x + len
    local b = brush{kind="flat", width=w, stiffness=0.4}
    if y < 600 then b:load(warmSheen, rand(0.35, 0.6)) else b:load(violetSheen, rand(0.3, 0.55)) end
    local pts = {}
    for k=0,6 do local t=k/6; pts[#pts+1] = {lerp(x,x2,t), y + randn(0,0.25*w) + math.sin(t*3+x)*0.6} end
    if rand(0,1) < 0.7 then b:stroke(pts, {pressure={rand(0.3,0.6), 0.05}, ramps={0.1, 0.5}, clip=mud4:shrink(1.5)}); nL = nL + 1 end
    x = x2 + rand(30, 160)
  end
  y = y + 3.5 + 9*depth^1.4 + rand(0, 3)
end
print(nL)

--@ chunk 47
vs2 = pile{{"lead white",3},{"cobalt blue",0.5},{"red earth",0.3},{"raw umber",0.3}, medium=0.3}
md2 = pile{{"raw umber",1.5},{"bone black",0.3},{"cobalt blue",0.35},{"lead white",0.9},{"red earth",0.2}, medium=0.35}
work(mud4 * rect(0,558,1000,36), {hand="scumble", pile=vs2, angle=0, coverage=1.1, load=0.35, clip=mud4})
work(mud4 * rect(0,592,1000,50), {hand="scumble", pile=md2, angle=0, coverage=1.0, load=0.35, clip=mud4})

--@ chunk 48
blend(mud4, {angle=0}); blend(mud4, {angle=0.05}); blend(mud4, {angle=-0.05})

--@ chunk 49
farP2 = pile{{"lead white",3},{"cobalt blue",0.5},{"red earth",0.3},{"raw umber",0.25},{"yellow ochre",0.2}}
work(farShore, {hand="detail", tool="round 2.5", pile=farP2, angle=0, coverage=2.5, length={8,25}, fill=true})
rip1 = pile{{"lead white",6},{"chrome yellow",0.6},{"yellow ochre",0.3},{"vermilion",0.06}}
rip2 = pile{{"lead white",2.6},{"cobalt blue",0.55},{"red earth",0.3},{"raw umber",0.15}}
local waterMid = rect(0,404,1000,160) - bar:grow(2) - barR:grow(2) - mud4:grow(2)
local n = 0
for i = 1, 140 do
  local y = 405 + 155 * (rand(0,1)^1.25)
  local x = rand(-20, 1000)
  if waterMid:at(clamp(x,1,999), y) > 0.5 then
    local depth = (y - 400) / 165
    local w = 0.7 + 1.6*depth
    local len = rand(15, 90) * (0.6 + depth)
    local b = brush{kind="flat", width=w, stiffness=0.4}
    local light = (x > 250 and x < 560 and rand(0,1) < 0.75) or rand(0,1) < 0.45
    if light then b:load(rip1, rand(0.3,0.5)) else b:load(rip2, rand(0.25,0.45)) end
    local pts = {}
    for k=0,4 do local t=k/4; pts[#pts+1] = {x + len*t, y + randn(0, 0.2)} end
    b:stroke(pts, {pressure={rand(0.25,0.5), 0.05}, ramps={0.25,0.5}, clip=waterMid})
    n = n + 1
  end
end
print(n)

--@ chunk 50
print(wait(30*60)); for _,p in ipairs({{230,600},{480,600},{900,620}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 51
hull = poly({{178,598},{205,602},{230,604},{260,602},{283,598},{297,591},{291,603},{276,611},{246,617},{214,617},{190,613},{180,608}}, true)
inner = poly({{181,597},{200,595},{240,594},{278,591},{296,590},{283,598},{260,602},{230,604},{205,602}}, true)
shadowB = poly({{170,612},{200,618},{250,621},{290,614},{305,610},{300,617},{250,625},{200,623},{168,617}}, true):soften(1.5)
hullP = pile{{"bone black",0.5},{"Prussian blue",0.08},{"raw umber",0.7},{"lead white",0.35}}
innerP = pile{{"raw umber",1},{"yellow ochre",0.5},{"lead white",0.9},{"red earth",0.2}}
shP = pile{{"raw umber",1},{"bone black",0.7},{"cobalt blue",0.2}, medium=0.3}
work(shadowB, {hand="detail", tool="round 3", pile=shP, angle=0, coverage=2, length={8,20}, fill=true})
work(inner, {hand="detail", tool="round 2", pile=innerP, angle=-0.05, coverage=2.5, length={6,15}, fill=true})
work(hull, {hand="detail", tool="round 2.5", pile=hullP, angle=-0.05, coverage=3, length={6,18}, fill=true})

--@ chunk 52
innerP2 = pile{{"raw umber",1},{"lead white",0.6},{"cobalt blue",0.2},{"red earth",0.15}}
work(inner, {hand="detail", tool="round 2", pile=innerP2, angle=-0.05, coverage=2, length={6,15}, fill=true})
contact = poly({{186,612},{214,617},{246,617},{276,611},{292,605},{298,609},{280,617},{246,623},{210,623},{182,618}}, true)
work(contact, {hand="detail", tool="round 2", pile=shP, angle=0, coverage=2.5, length={6,15}, fill=true})
local r = brush{kind="rigger", width=1.4, point=1}
r:load(hullP, 0.8)
r:stroke({{238,599},{234,560},{229,515}}, {pressure={0.9,0.4}})
r:reload(hullP, 0.5)
r:stroke({{229,517},{262,556},{296,588}}, {pressure={0.25,0.2}})
r:reload(hullP, 0.6)
r:stroke({{179,597},{181,605},{183,611}}, {pressure={0.8,0.8}})
r:stroke({{292,592},{298,586},{301,583}}, {pressure={0.7,0.4}})
gl = pile{{"lead white",5},{"yellow ochre",0.4},{"vermilion",0.1}}
local g = brush{kind="round", width=1.2, point=0.7}
g:load(gl, 0.5)
g:stroke({{183,597},{205,599},{235,600},{262,598},{285,594},{297,590}}, {pressure={0.35,0.2}})

--@ chunk 53
postP = pile{{"raw umber",1},{"bone black",0.7},{"lead white",0.15}}
reflPost = pile{{"raw umber",1},{"bone black",0.4},{"cobalt blue",0.4},{"lead white",0.9}}
posts = {{765,642,44,4.5},{741,614,30,3.4},{722,597,21,2.6},{708,587,15,2},{698,580,10,1.5},{691,576,7,1.2}}
for i,p in ipairs(posts) do
  local x, yb, h, w = p[1], p[2], p[3], p[4]
  local tilt = randn(0, 0.04)
  local b = brush{kind="flat", width=w, stiffness=0.6}
  b:load(postP, 0.8)
  b:stroke({{x + tilt*h, yb - h}, {x + tilt*h*0.5, yb - h*0.5}, {x, yb}}, {pressure={0.8,0.9}, orient="across"})
  local rb = brush{kind="flat", width=w*0.9, stiffness=0.5}
  rb:load(reflPost, 0.6)
  local pts = {}
  for k=0,6 do local t = k/6; pts[#pts+1] = {x + math.sin(t*9 + i)*w*0.35, yb + 1 + t*h*0.85} end
  rb:stroke(pts, {pressure={0.7,0.1}, ramps={0.05,0.7}, orient="across"})
end

--@ chunk 54
local L = {{505,577},{516,583},{500,598},{486,606},{476,622},{452,646},{436,668},{414,692},{398,720}}
local R = {{735,584},{716,589},{728,601},{748,618},{772,640},{800,664},{826,688},{850,720}}
local function offs(pts, dx) local o = {} for i,p in ipairs(pts) do local k = (p[2]-570)/150; o[i] = {p[1] + dx*(0.4+k), p[2] + 1} end return o end
local chan = -(mud4)
local bankRefl = pile{{"raw umber",1},{"bone black",0.3},{"cobalt blue",0.5},{"lead white",1.1}, medium=0.2}
local lip = pile{{"lead white",3},{"cobalt blue",0.45},{"red earth",0.3},{"raw umber",0.2}}
for _,side in ipairs({{L, 1}, {R, -1}}) do
  local pts, sgn = side[1], side[2]
  for j = 1, #pts-1 do
    local a, b2 = pts[j], pts[j+1]
    local k = (a[2]-570)/150
    local br = brush{kind="flat", width=2 + 5*k, stiffness=0.45}
    br:load(bankRefl, 0.55)
    br:stroke({{a[1] + sgn*(2+3*k), a[2]+1}, {b2[1] + sgn*(2+3*k), b2[2]+1}}, {pressure={0.5,0.4}, clip=chan})
  end
  for j = 2, #pts-1 do
    local a, b2 = pts[j], pts[j+1]
    local k = (a[2]-570)/150
    local br = brush{kind="round", width=1 + 2*k, point=0.3}
    br:load(lip, 0.45)
    br:stroke({{a[1] - sgn*(1.5+2*k), a[2]}, {b2[1] - sgn*(1.5+2*k), b2[2]}}, {pressure={0.4,0.3}, clip=mud4})
  end
end

--@ chunk 55
local s = (mud4:grow(12) - mud4:shrink(6)) * rect(380,570,480,160); blend(s, {angle=0, tool={kind="badger", width=14}}); blend(s, {angle=1.2, tool={kind="badger", width=14}})

--@ chunk 56
print(wait(40*60)); for _,p in ipairs({{230,610},{740,600},{765,630},{800,580}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 57
local notMud = -(mud4)
local o = {hand="detail", tool="round 3", angle=0, coverage=3, length={8,25}, fill=true, clip=notMud}
o.pile = c1; work(rect(690,564,170,24) * notMud, o)
local margin = (mud4:grow(9) - mud4) * rect(690,585,180,140)
o.pile = c1; work(margin * rect(0,585,1000,35), o)
o.pile = c2; work(margin * rect(0,618,1000,110), o)
local marginL = (mud4:grow(9) - mud4) * rect(380,570,150,150)
o.pile = c1; work(marginL * rect(0,570,1000,45), o)
o.pile = c2; work(marginL * rect(0,613,1000,110), o)

--@ chunk 58
local holes2 = ellipse(818,390,6,5) + ellipse(862,389,8,5) + ellipse(907,391,6,3.5)
local droop = (rect(780,362,175,40):roughen(4,9,171) - holes2:roughen(1.5,4,172))
work(droop, {hand="detail", tool="round 3.5", pile=treeP2, angle=function(x,y) return -1.4 + 0.5*math.sin(x/9) end, coverage=3, length={4,10}, fill=true})
work(ellipse(818,306,16,12), {hand="detail", tool="round 3", pile=treeP, angle=-1.0, coverage=3, length={4,9}, fill=true})
local r = brush{kind="rigger", width=1.5, point=1}
r:load(treeP2, 0.7)
for _,t in ipairs({{816,386},{861,384},{906,388},{866,386}}) do r:stroke({{t[1],t[2]},{t[1]+rand(-0.8,0.8),t[2]+6},{t[1]+rand(-1,1),399}}, {pressure={0.5,0.7}}) end
litEdge = mask(function(x,y) if x < 690 or y > 395 then return 0 end; return trees:at(x,y) * (1 - trees:at(x-5,y-6)) end)
treeLit = pile{{"raw umber",1},{"green earth",0.6},{"yellow ochre",0.35},{"lead white",0.5},{"bone black",0.1}}
stipple(litEdge, {pile=treeLit, width=2.4, coverage=0.9, pressure={0.3,0.6}, cluster=0.5, feather=0.4})

--@ chunk 59
local rim = ellipse(818,306,16,12):rim(4, 2) * trees:grow(3); stipple(rim, {pile=treeP, width=2.6, coverage=1.2, pressure={0.3,0.6}, cluster=0.5}); stipple(rim * litEdge:grow(3), {pile=treeLit, width=2.2, coverage=0.6, pressure={0.3,0.5}, cluster=0.5})

--@ chunk 60
local b = brush{kind="round", width=3.2, point=0.2}
b:load(treeP, 0.7)
for i = 1, 40 do
  local a = rand(math.pi*0.85, math.pi*2.05)
  local rr = 1 + rand(-0.12, 0.08)
  local x = 818 + 16*rr*math.cos(a); local y = 306 + 12*rr*math.sin(a)
  if i % 10 == 0 then b:load(treeP, 0.6) end
  b:touch(x, y, {pressure=rand(0.4,0.8), drag={rand(-1,1), rand(-1,1)}})
end
local l = brush{kind="round", width=2.2, point=0.2}
l:load(treeLit, 0.5)
for i = 1, 14 do
  local a = rand(math.pi*1.0, math.pi*1.5)
  local x = 818 + 15*math.cos(a); local y = 306 + 11*math.sin(a)
  l:touch(x+rand(-1.5,1.5), y+rand(-1.5,1.5), {pressure=rand(0.3,0.5)})
end

--@ chunk 61
local b = brush{kind="round", width=3, point=0.2}; b:load(treeP, 0.6); for _,p in ipairs({{824,295},{828,294},{832,295},{821,296}}) do b:touch(p[1],p[2],{pressure=0.6}) end

--@ chunk 62
print(wait(20*60)); for _,p in ipairs({{230,610},{740,600},{765,630},{820,380}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 63
posts = {{766,644,46,4.6},{742,615,31,3.4},{724,598,22,2.7},{710,588,16,2.1},{700,581,11,1.6},{693,577,7,1.2}}
local topLit = pile{{"lead white",3},{"yellow ochre",0.4},{"raw umber",0.4}}
for i,p in ipairs(posts) do
  local x, yb, h, w = p[1], p[2], p[3], p[4]
  h = h * (0.85 + 0.3*rand(0,1))
  local tilt = randn(0, 0.05)
  local b = brush{kind="flat", width=w, stiffness=0.7}
  b:load(postP, 0.9)
  b:stroke({{x, yb}, {x + tilt*h*0.5, yb - h*0.5}, {x + tilt*h, yb - h}}, {pressure={0.9,0.85}, ramps={0.02,0.02}, orient="across"})
  local rb = brush{kind="flat", width=w*0.85, stiffness=0.5}
  rb:load(reflPost, 0.55)
  local pts = {}
  for k=0,8 do local t = k/8; pts[#pts+1] = {x + math.sin(t*10 + i)*w*0.45, yb + 1.5 + t*h*0.9} end
  rb:stroke(pts, {pressure={0.75,0.05}, ramps={0.02,0.8}, orient="across"})
  local tb = brush{kind="flat", width=w*0.9, stiffness=0.5}
  tb:load(topLit, 0.4)
  tb:touch(x + tilt*h - w*0.1, yb - h + w*0.3, {pressure=0.3, drag={w*0.4, 0}})
end

--@ chunk 64
litCloud = pile{{"lead white",4},{"vermilion",0.3},{"chrome yellow",0.25},{"yellow ochre",0.3}}
litCloud2 = pile{{"lead white",5},{"chrome yellow",0.5},{"vermilion",0.15}}
local n = 0
local x = -20
while x < 780 do
  local len = rand(40, 150)
  local y = 232 + 0.004*x + randn(0, 2.5)
  local b = brush{kind="flat", width=rand(1.5, 3), stiffness=0.35}
  b:load(litCloud, rand(0.2, 0.4))
  local pts = {}
  for k=0,5 do local t=k/5; pts[#pts+1] = {x + len*t, y + math.sin(t*2.5 + x)*1.5} end
  b:stroke(pts, {pressure={0.35, 0.05}, ramps={0.3, 0.5}})
  x = x + len + rand(10, 70); n = n + 1
end
for i = 1, 9 do
  local x = rand(80, 700); local y = rand(280, 318); local len = rand(40, 120)
  local b = brush{kind="flat", width=rand(1.2, 2.2), stiffness=0.35}
  b:load(litCloud2, rand(0.2, 0.35))
  b:stroke({{x, y}, {x + len*0.5, y - 0.5}, {x + len, y + 0.3}}, {pressure={0.3, 0.05}, ramps={0.3, 0.5}})
end
print(n)

--@ chunk 65
local m = rect(0,222,800,26):soften(5); blend(m, {angle=0.02, tool={kind="badger", width=20}}); local m2 = rect(60,272,700,52):soften(5); blend(m2, {angle=0, tool={kind="badger", width=20}})

--@ chunk 66
print(wait(2*24*60)); for _,p in ipairs({{230,600},{250,610},{766,620},{400,230}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 67
hgz = pile{{"bone black",0.6},{"Prussian blue",0.1},{"raw umber",0.6}, medium=0.6}
igz = pile{{"raw umber",1},{"cobalt blue",0.35},{"bone black",0.15}, medium=0.65}
work(hull, {hand="detail", tool="round 3", pile=hgz, angle=-0.05, coverage=2.5, length={8,20}, fill=true, clip=hull})
work(inner, {hand="detail", tool="round 2", pile=igz, angle=-0.05, coverage=1.5, length={8,20}, fill=true, clip=inner})
local g = brush{kind="round", width=1.3, point=0.6}
g:load(gl, 0.6)
g:stroke({{184,598.5},{205,601},{232,602.5},{260,600.5},{283,596.5},{296,590.5}}, {pressure={0.45,0.3}})
local bird = brush{kind="rigger", width=1.0, point=1}
bird:load(pile{{"raw umber",1},{"bone black",0.4},{"lead white",0.8}}, 0.5)
for _,p in ipairs({{318,296,5},{334,289,4},{345,301,3.5}}) do
  local x,y,s = p[1],p[2],p[3]
  bird:stroke({{x-s, y-s*0.35},{x-s*0.45,y-s*0.45},{x,y}}, {pressure={0.1,0.45}})
  bird:stroke({{x,y},{x+s*0.45,y-s*0.5},{x+s,y-s*0.3}}, {pressure={0.45,0.1}})
end

--@ chunk 68
local b = brush{kind="round", width=3.2, point=0.2}
b:load(treeP2, 0.7)
local holesC = {{818,390,6,5},{862,389,8,5},{907,391,6,3.5}}
for hi,h in ipairs(holesC) do
  for i = 1, 16 do
    local a = rand(0, math.pi*2); local r = rand(0.1, 0.9)
    local x = h[1] + h[3]*r*math.cos(a); local y = h[2] + h[4]*r*math.sin(a)
    if not (hi == 2 and y > h[2] + 1 and math.abs(x - h[1] - 3) < 3) then
      b:touch(x, y, {pressure=rand(0.4,0.7), drag={randn(0,0.5), rand(0,1.5)}})
    end
    if i % 8 == 0 then b:load(treeP2, 0.6) end
  end
end

--@ chunk 69
glowP = pile{{"lead white",8},{"chrome yellow",0.7},{"vermilion",0.04}, medium=0.35}
local core = ellipse(385, 382, 150, 20) * rect(0,0,1000,396)
work(core, {hand="body", tool="filbert 8", pile=glowP, angle=0, coverage=1.5, length={40,110}, load=0.5, clip=rect(0,0,1000,396) - farShore:grow(1)})
local bm = ellipse(385, 378, 200, 34):soften(12) * rect(0,0,1000,395)
blend(bm, {angle=0, tool={kind="badger", width=30}})
blend(bm, {angle=0.1, tool={kind="badger", width=30}})

--@ chunk 70
weedP = pile{{"raw umber",1},{"bone black",0.4},{"green earth",0.5},{"yellow ochre",0.15},{"lead white",0.15}}
stoneP = pile{{"raw umber",1},{"bone black",0.5},{"cobalt blue",0.2},{"lead white",0.5}}
stoneHi = pile{{"lead white",3},{"cobalt blue",0.4},{"red earth",0.25},{"raw umber",0.2}}
local n = 0
for i = 1, 90 do
  local y = 566 + 150 * (rand(0,1)^1.1)
  local x = rand(0, 1000)
  if mud4:shrink(4):at(x, y) > 0.9 and not (x > 160 and x < 310 and y > 585 and y < 630) then
    local d = (y - 560) / 155
    local s = 1.2 + 7 * d^1.3
    if rand(0,1) < 0.55 then
      local c = ellipse(x, y, s*rand(1.2,2.5), s*0.45):roughen(s*0.3, s*0.8, i)
      stipple(c, {pile=weedP, width=math.max(1.2, s*0.5), coverage=1.4, pressure={0.3,0.6}, cluster=0.4})
    else
      local b = brush{kind="round", width=math.max(1, s*0.9), point=0.1}
      b:load(stoneP, 0.6)
      b:touch(x, y, {pressure=0.6, drag={s*0.4, 0}})
      local h = brush{kind="round", width=math.max(0.7, s*0.4), point=0.2}
      h:load(stoneHi, 0.4)
      h:touch(x - s*0.1, y - s*0.25, {pressure=0.35, drag={s*0.25, 0}})
    end
    n = n + 1
  end
end
print(n)

--@ chunk 71
weedD = pile{{"raw umber",1},{"bone black",0.6},{"green earth",0.4},{"red earth",0.15}}
local spots = {{70,690,14},{120,700,10},{95,676,8},{190,705,12},{250,684,7},{40,660,6},{300,700,9},{160,668,6},{930,700,12},{975,680,8},{880,690,7},{350,672,5}}
for i,s in ipairs(spots) do
  local x,y,r = s[1],s[2],s[3]
  local c = ellipse(x, y, r*2.2, r*0.55):roughen(r*0.35, r*0.9, 200+i)
  stipple(c, {pile=weedD, width=math.max(2, r*0.35), coverage=1.6, pressure={0.4,0.7}, cluster=0.5, drag={r*0.3, 0}})
  if i % 2 == 0 then
    local b = brush{kind="round", width=r*0.8, point=0.1}
    b:load(stoneP, 0.7)
    b:touch(x + r*0.8, y - r*0.1, {pressure=0.7, drag={r*0.5, 0}})
    local h = brush{kind="round", width=r*0.35, point=0.2}
    h:load(stoneHi, 0.45)
    h:touch(x + r*0.7, y - r*0.35, {pressure=0.35, drag={r*0.35, 0}})
  end
end

--@ chunk 72
print(wait(2*24*60)); for _,p in ipairs({{70,690},{230,650},{900,690},{380,380}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 73
local gm = mud4 - (hull + inner + contact):grow(3)
gz3 = pile{{"raw umber",1},{"bone black",0.9},{"cobalt blue",0.35},{"red earth",0.1}, medium=0.65}
work(gm * rect(0,622,1000,110), {hand="glaze", pile=gz3, angle=0, coverage=1.6, clip=gm})
work(gm * rect(0,660,1000,70), {hand="glaze", pile=gz3, angle=0.02, coverage=1.6, clip=gm})
work(gm * (rect(0,640,180,90) + rect(880,640,130,90)), {hand="glaze", pile=gz3, angle=-0.03, coverage=1.4, clip=gm})
blend(gm, {angle=0})
blend(gm, {angle=0.07})

--@ chunk 74
local ch = rect(0,580,1000,140) - mud4:grow(2)
ripD = pile{{"lead white",1.8},{"cobalt blue",0.9},{"smalt",0.3},{"raw umber",0.3},{"red earth",0.15}}
ripL = pile{{"lead white",4},{"cobalt blue",0.3},{"vermilion",0.08},{"yellow ochre",0.2}}
local y = 588
local n = 0
while y < 714 do
  local d = (y - 575) / 140
  local x = rand(380, 560)
  while x < 860 do
    local len = rand(20, 70) * (0.6 + 1.2*d)
    if ch:at(clamp(x + len/2, 1, 999), y) > 0.5 and rand(0,1) < 0.6 then
      local b = brush{kind="flat", width=0.8 + 2.4*d, stiffness=0.4}
      if rand(0,1) < 0.55 then b:load(ripD, rand(0.25,0.45)) else b:load(ripL, rand(0.25,0.4)) end
      local pts = {}
      for k=0,4 do local t=k/4; pts[#pts+1] = {x + len*t, y + math.sin(t*3.1)*0.8*d + randn(0,0.2)} end
      b:stroke(pts, {pressure={rand(0.25,0.45), 0.05}, ramps={0.3,0.5}, clip=ch})
      n = n + 1
    end
    x = x + len + rand(15, 80)
  end
  y = y + 4 + 14*d^1.3 + rand(0, 4)
end
print(n)

--@ chunk 75
blend(rect(0,580,1000,140) - mud4:grow(2), {angle=0, tool={kind="badger", width=16}})

--@ chunk 76
print(wait(2*24*60))

--@ chunk 77
reflD = pile{{"raw umber",1},{"bone black",0.5},{"cobalt blue",0.3},{"green earth",0.2},{"lead white",0.55}}
local wc = rect(0,402.5,1000,110) - bar - barR
local n = 0
local y = 403.5
while y < 470 do
  local fade = 1 - (y - 403)/70
  local x = 590 + rand(0, 10)
  while x < 1000 do
    local len = rand(8, 40)
    local xm = clamp(x + len/2, 1, 999)
    if trees:at(xm, 804 - y) > 0.5 and rand(0,1) < 0.25 + 0.7*fade and wc:at(xm, y) > 0.5 then
      local b = brush{kind="flat", width=rand(1.2, 2.2), stiffness=0.4}
      b:load(reflD, 0.3 + 0.35*fade)
      b:stroke({{x, y + randn(0,0.2)}, {x + len, y + randn(0,0.2)}}, {pressure={0.3 + 0.3*fade, 0.1}, ramps={0.2,0.4}, clip=wc})
      n = n + 1
    end
    x = x + len + rand(2, 14)
  end
  y = y + 1.6 + 2.2*(1-fade) + rand(0, 1)
end
print(n)

--@ chunk 78
local rm = mask(function(x,y) if y < 403 or y > 480 then return 0 end; return trees:at(x, 804 - y) end):grow(4):soften(4) * (rect(0,403,1000,110) - bar - barR); blend(rm, {angle=0, tool={kind="badger", width=18}}); blend(rm, {angle=0.03, tool={kind="badger", width=18}})

--@ chunk 79
print(wait(120*24*60)); local all = true; for x = 20, 980, 60 do for y = 20, 700, 60 do if drying(x,y) ~= "dry" then all = false; print(x,y,drying(x,y)) end end end; print(all)

--@ chunk 80
varnish{coats=0.3, vary=0.08}
