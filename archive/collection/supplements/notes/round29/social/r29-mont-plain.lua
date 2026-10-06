-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box giverny
--@ engine 5

--@ chunk 1
canvas{size=1300, aspect=1.5, linen={13,16}, seed=1917,
 ground={{pile={{"lead white",10},{"yellow ochre",0.15}}, um=120, apply="knife", texture=0.3},
         {pile={{"lead white",10},{"cobalt blue",0.03}}, um=60, apply="brush"}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
gold = pile{{"lead white",6},{"pale cadmium",0.6},{"rose madder",0.15}, turps=0.5, name="gold"}
lilac = pile{{"lead white",5},{"cobalt violet",1.2},{"cobalt blue",0.3}, turps=0.5, name="lilac"}
deep = pile{{"ultramarine blue",2},{"cobalt blue",1},{"lead white",1.5},{"cobalt violet",0.6}, turps=0.5, name="deep"}
wdark = pile{{"viridian",1.5},{"ultramarine blue",1.5},{"rose madder",0.8},{"cobalt violet",0.5}, turps=0.5, name="wdark"}
print(gold, lilac, deep, wdark)

--@ chunk 3
lilac:add{{"cobalt violet",1.5},{"rose madder",0.25},{"ultramarine blue",0.15}}
gold:add{{"deep cadmium",0.1},{"pale cadmium",0.3}}
wdark:add{{"viridian",0.6}}
print(lilac, gold, wdark)

--@ chunk 4
wL = {{0,0},{400,0},{392,60},{365,120},{374,190},{342,262},{320,330},{302,252},{282,352},{252,300},{234,410},{206,332},{182,442},{152,352},{122,422},{92,332},{62,392},{32,322},{0,362}}
wR = {{742,0},{1000,0},{1000,302},{976,252},{952,322},{926,242},{902,292},{872,212},{846,262},{822,182},{802,222},{782,142},{762,92}}
willowM = (poly(wL,true) + poly(wR,true)):roughen(10, 40, 3)
local function gw(x,y) local dx=(x-560)/260; local d = math.max(0, 1 - dx*dx - (y/330)^2); return d*1.4 + 0.05 end
local function lw(x,y) return math.exp(-((y-330)/120)^2) + 0.15 end
local function dw(x,y) return smoothstep(330, 560, y)*1.6 + 0.02 end
work(everywhere(), {hand="broad", piles={{gold,gw},{lilac,lw},{deep,dw}}, angle=function(x,y) return 0.03*math.sin(x/90) end, coverage=1.3, pressure={0.5,0.8}, seed=11})
print("done")

--@ chunk 5
local function gw(x,y) local dx=(x-560)/260; local d = math.max(0, 1 - dx*dx - (y/330)^2); return d*1.4 + 0.05 end
local function lw(x,y) return math.exp(-((y-330)/120)^2) + 0.15 end
local function dw(x,y) return smoothstep(330, 560, y)*1.6 + 0.02 end
work(everywhere(), {hand="broad", piles={{gold,gw},{lilac,lw},{deep,dw}}, angle=function(x,y) return 0.04*math.sin(x/70+y/50) end, coverage=1.6, fill=true, length={120,260}, pressure={0.6,0.9}, seed=12})
print("done")

--@ chunk 6
print(wait(18*60)); print(drying(500,100), drying(200,300), drying(500,600))

--@ chunk 7
print(wait(14*60)); print(drying(500,100), drying(200,300), drying(500,600), drying(900,300))

--@ chunk 8
wait(14*60)
wd2 = pile{{"viridian",2},{"ultramarine blue",1.6},{"rose madder",0.8},{"cobalt violet",0.6},{"lead white",0.3}, turps=0.25, name="wd2"}
wgreen = pile{{"viridian",2},{"ultramarine blue",0.8},{"yellow ochre",0.6},{"lead white",0.6}, turps=0.25, name="wgreen"}
wviol = pile{{"ultramarine blue",1.5},{"cobalt violet",1.5},{"rose madder",0.6},{"lead white",0.8}, turps=0.25, name="wviol"}
local n = noise{seed=5, period=120}
work(willowM, {hand="body", piles={{wd2, 1.0},{wgreen, function(x,y) return 0.5+0.5*n(x,y) end},{wviol, function(x,y) return 0.3+0.4*smoothstep(150,400,y) end}},
  angle=function(x,y) return math.pi/2 + 0.08*math.sin(x/40) end, length={40,110}, coverage=1.6, fill=true, edge={soft=0.5, lost=0.3, found=0.2, period=50}, pressure={0.5,0.85}, seed=21})
print(drying(200,100))

--@ chunk 9
wmid = pile{{"cobalt blue",1},{"viridian",0.7},{"lead white",1.6},{"ultramarine blue",0.3}, name="wmid"}
wlil = pile{{"lead white",3},{"cobalt violet",1.2},{"cobalt blue",0.4},{"rose madder",0.2}, name="wlil"}
local n = noise{seed=8, period=60, stretch={math.pi/2, 4}}
local m = willowM:times(function(x,y) return clamp(0.5 + 0.9*n(x,y), 0, 1) end)
work(m, {hand="body", tool="filbert 5", piles={{wmid,1},{wlil,0.6}}, angle=function(x,y) return math.pi/2 + 0.06*math.sin(x/30) end, length={30,120}, coverage=0.7, pressure={0.3,0.6}, load=0.5, threshold=0.5, seed=31})
print("ok")

--@ chunk 10
blend(willowM, {angle=math.pi/2, seed=41})
print(drying(200,100))

--@ chunk 11
fr = brush{kind="round", width=7, point=0.6, stiffness=0.4}
frpile = pile{{"viridian",1.5},{"ultramarine blue",1.5},{"cobalt violet",0.8},{"rose madder",0.4},{"lead white",0.6}, medium=0.15, name="frond"}
local starts = {}
for i=1,30 do
  local side = (i<=20) and 0 or 1
  local x, y0
  if side==0 then x = rand(10, 360) else x = rand(790, 995) end
  -- find bottom edge of willow mask at x
  local yb = 0
  for yy=0,500,4 do if willowM:at(x,yy) > 0.5 then yb = yy end end
  y0 = yb - rand(40,90)
  local len = rand(30, 110)
  fr:load(frpile, rand(0.4,0.7))
  local dx = rand(-6,6)
  fr:gesture({{x, y0, 0.8},{x+dx*0.5, yb, 0.6},{x+dx, yb+len*0.6, 0.35},{x+dx*1.3, yb+len, 0.0}}, {wobble=2})
end
print("ok")

--@ chunk 12
local edgeband = (willowM:grow(25) - willowM:shrink(70)):times(function(x,y) return smoothstep(120, 260, y) end)
wblu = pile{{"lead white",2.5},{"cobalt blue",1},{"cobalt violet",0.5}, name="wblu"}
work(edgeband, {hand="body", tool="filbert 6", piles={{wlil,1},{wblu,0.8}}, angle=function(x,y) return 0.05*math.sin(x/25) end, length={14,40}, coverage=0.55, pressure={0.35,0.65}, load=0.5, threshold=0.4, seed=51})
print("ok")

--@ chunk 13
local eb = (willowM:grow(35) - willowM:shrink(80)):times(function(x,y) return smoothstep(110, 240, y) end):soften(10)
blend(eb, {angle=0, seed=61})
blend(eb, {angle=0.05, seed=62})
print("ok")

--@ chunk 14
r = rag{width=60}
r:dip(0.7)
r:wipe(willowM:grow(40), {pressure=0.7, angle=math.pi/2, passes=2, refold=0.5, seed=3})
r = rag{width=60}; r:dip(0.8)
r:wipe(willowM:grow(40), {pressure=0.8, angle=0, passes=2, refold=0.4, seed=4})
print(drying(200,100), drying(150,350))

--@ chunk 15
r = rag{width=30}; r:dip(0.8)
for _,p in ipairs({{55,440,500},{108,440,500},{178,450,505},{325,350,410},{335,350,410},{898,320,370},{962,330,370},{987,330,370}}) do
  r:wipe({{p[1],p[2]},{p[1],p[3]}}, {pressure=0.8}); r:wipe({{p[1]+3,p[3]},{p[1]-3,p[2]}}, {pressure=0.8}); r:refold()
end
r = rag{width=40}; r:dip(0.8)
r:wipe({{0,4},{1000,4}}, {pressure=0.8}); r:refold(); r:wipe({{1000,6},{0,6}}, {pressure=0.8})
r:refold(); r:wipe({{4,0},{4,400}}, {pressure=0.8}); r:refold(); r:wipe({{996,0},{996,400}}, {pressure=0.8})
print("ok")

--@ chunk 16
wait(20*60)
crownL = {{0,0},{420,0},{405,50},{395,95},{370,120},{372,165},{345,190},{340,240},{310,262},{300,310},{262,330},{240,370},{196,378},{160,400},{118,388},{84,402},{46,380},{18,392},{0,385}}
crownR = {{735,0},{1000,0},{1000,290},{972,300},{944,282},{915,292},{888,262},{858,250},{842,215},{815,198},{805,160},{780,135},{770,90},{748,60}}
crownM = (poly(crownL,true) + poly(crownR,true)):roughen(8, 30, 7)
print(crownM:area(), drying(200,100))

--@ chunk 17
wait(26*60) print(drying(200,100), drying(150,350), drying(850,200))

--@ chunk 18
rv1 = pile{{"ultramarine blue",2},{"cobalt violet",1},{"viridian",0.4},{"lead white",0.8}, medium=0.1, name="rv1"}
rv2 = pile{{"viridian",1.5},{"cobalt blue",1},{"lead white",1.2},{"yellow ochre",0.2}, medium=0.1, name="rv2"}
rv3 = pile{{"cobalt violet",1.5},{"ultramarine blue",1},{"rose madder",0.5},{"lead white",1.2}, medium=0.1, name="rv3"}
local t = rect(20,20,120,110)
work(t, {hand="body", tool="filbert 12", piles={{rv1,1},{rv2,0.6},{rv3,0.5}}, angle=math.pi/2, length={50,110}, coverage=1.2, fill=true, pressure={0.6,0.9}, clip=true, seed=71})
print("ok")

--@ chunk 19
rv1:add{{"ultramarine blue",0.5},{"viridian",0.3}}
local n1 = noise{seed=12, period=90, stretch={math.pi/2, 3}}
local n2 = noise{seed=13, period=90, stretch={math.pi/2, 3}}
work(crownM, {hand="body", tool="filbert 12", pile=rv1, angle=function(x,y) return math.pi/2 + 0.05*math.sin(x/37) end, length={60,150}, coverage=1.1, fill=true, pressure={0.6,0.95}, edge={soft=0.6, lost=0.3, found=0.1, period=60}, seed=72})
work(crownM:times(function(x,y) return clamp(0.5+n1(x,y),0,1) end), {hand="body", tool="filbert 10", pile=rv2, angle=math.pi/2, length={40,120}, coverage=0.7, pressure={0.5,0.85}, threshold=0.5, edge="soft", seed=73})
work(crownM:times(function(x,y) return clamp(0.3+n2(x,y)+0.4*smoothstep(150,350,y),0,1) end), {hand="body", tool="filbert 10", pile=rv3, angle=math.pi/2, length={40,110}, coverage=0.6, pressure={0.5,0.85}, threshold=0.5, edge="soft", seed=74})
print("ok")

--@ chunk 20
local b = brush("filbert", 10); print(b:mark_width(0.3), b:mark_width(0.6), b:mark_width(0.9))
local b2 = brush("filbert", 22); print(b2:mark_width(0.3), b2:mark_width(0.6), b2:mark_width(0.9))

--@ chunk 21
blend(crownM:shrink(4), {angle=math.pi/2, seed=81})
print("ok")

--@ chunk 22
wedge = mix{{lilac,0.7},{rv3,0.3}, name="wedge"}
print(wedge)
local nlose = lose(crownM, {pile=wedge, where=function(x,y) return smoothstep(120,260,y) end, tool="filbert 8", reach={30,25}, load=0.25, pressure={0.4,0.05}, seed=91})
print(nlose)

--@ chunk 23
local eb = crownM:rim(36, 8):times(function(x,y) return smoothstep(100,220,y) end)
blend(eb, {angle=0.0, seed=92})
print("ok")

--@ chunk 24
wait(3*24*60) print(drying(200,100), drying(150,350), drying(850,200), drying(500,500))

--@ chunk 25
wait(4*24*60) print(drying(200,100), drying(150,350), drying(850,200), drying(60,380))

--@ chunk 26
lw1 = pile{{"ultramarine blue",1},{"cobalt violet",1},{"lead white",2},{"rose madder",0.2}, medium=0.1, name="lw1"}
lw2 = pile{{"viridian",1},{"cobalt blue",1},{"lead white",2.2}, medium=0.1, name="lw2"}
lw3 = pile{{"ultramarine blue",2},{"cobalt blue",0.6},{"cobalt violet",0.5},{"lead white",0.8}, medium=0.1, name="lw3"}
lw4 = pile{{"lead white",4},{"cobalt violet",0.8},{"pale cadmium",0.3},{"rose madder",0.15}, medium=0.1, name="lw4"}
local n = noise{seed=21, period=160, stretch={0,3}}
local m = below(function(x) return 370 + 25*math.sin(x/120) end)
work(m, {hand="body", tool="filbert 10", piles={{lw1,function(x,y) return 0.6+0.5*n(x,y) end},{lw2,function(x,y) return 0.5-0.5*n(x,y) end},{lw3,function(x,y) return 0.2+1.2*smoothstep(450,667,y) end},{lw4,function(x,y) return 0.6*(1-smoothstep(380,480,y)) end}},
  angle=function(x,y) return 0.03*math.sin(x/80) end, length={30,90}, coverage=0.9, pressure={0.45,0.8}, curve={0.1,0.2}, scale_at=function(x,y) return 0.7+0.6*smoothstep(380,667,y) end, seed=101})
print("ok")

--@ chunk 27
local m = below(function(x) return 330 + 25*math.sin(x/120) end):soften(30)
blend(m, {angle=0, seed=111})
print("ok")

--@ chunk 28
dk = pile{{"ultramarine blue",2.5},{"cobalt blue",0.8},{"cobalt violet",0.8},{"lead white",0.5},{"viridian",0.2}, medium=0.1, name="dk"}
gl = pile{{"lead white",5},{"pale cadmium",0.5},{"cobalt violet",0.4},{"rose madder",0.1}, name="gl"}
local n = noise{seed=31, period=140, stretch={0,4}}
local md = below(function(x) return 430 + 20*math.sin(x/90) end):times(function(x,y) return clamp(0.4+0.8*n(x,y)+0.5*smoothstep(450,650,y),0,1) end)
work(md, {hand="body", tool="filbert 8", pile=dk, angle=function(x,y) return 0.02*math.sin(x/60) end, length={25,80}, coverage=0.6, pressure={0.4,0.75}, load=0.6, threshold=0.45, scale_at=function(x,y) return 0.6+0.8*smoothstep(400,667,y) end, seed=121})
local mg = ellipse(560, 420, 210, 70):times(function(x,y) return clamp(0.5+n(x+300,y),0,1) end)
work(mg, {hand="body", tool="filbert 6", pile=gl, angle=0, length={15,50}, coverage=0.5, pressure={0.35,0.6}, load=0.5, threshold=0.4, seed=122})
print("ok")

--@ chunk 29
local m = below(function(x) return 340 + 25*math.sin(x/120) end):soften(20)
blend(m, {angle=0.02, seed=131})
print("ok")

--@ chunk 30
wait(5*24*60) print(drying(300,600), drying(500,420), drying(800,500), drying(200,200))

--@ chunk 31
padG = pile{{"viridian",1},{"barium yellow",1.2},{"lead white",1.5}, name="padG"}
padB = pile{{"viridian",1},{"cobalt blue",0.5},{"lead white",1.6},{"yellow ochre",0.3}, name="padB"}
padD = pile{{"viridian",1},{"ultramarine blue",0.6},{"yellow ochre",0.5},{"rose madder",0.25},{"lead white",0.4}, name="padD"}
padW = pile{{"lead white",3},{"barium yellow",0.7},{"viridian",0.2}, name="padW"}
function pad(x, y, rx, ry, base, top, seedv)
  local b = brush{kind="filbert", width=math.max(2.5, ry*1.1), stiffness=0.5}
  b:load(base, 0.7)
  local nst = math.max(2, math.floor(ry/4)+2)
  for i=1,nst do
    local t = (i-0.5)/nst*2-1
    local w = rx*math.sqrt(math.max(0.05,1-t*t))
    local yy = y + t*ry*0.7
    b:stroke({{x-w, yy+rand(-1,1)}, {x, yy+rand(-1.5,1.5)}, {x+w, yy+rand(-1,1)}}, {pressure={0.55,0.45}, ramps={0.15,0.25}})
  end
  if top then
    local b2 = brush{kind="filbert", width=math.max(2, ry*0.6), stiffness=0.5}
    b2:load(top, 0.5)
    local side = rand(-0.6,0.2)
    b2:stroke({{x-rx*0.6, y+side*ry}, {x+rx*0.3, y+side*ry - 1}}, {pressure={0.5,0.2}, ramps={0.1,0.4}})
  end
end
-- trial cluster over mid-right water
local pts = {{640,440,30,8},{690,452,24,7},{735,438,20,6},{600,458,22,7},{665,470,28,8},{775,452,18,5}}
for i,p in ipairs(pts) do pad(p[1],p[2],p[3],p[4], (i%2==0) and padB or padD, padG) end
print("ok")

--@ chunk 32
r = rag{width=40}; r:dip(0.8)
r:wipe(rect(570,425,230,58), {pressure=0.8, angle=0, passes=2, refold=0.3, seed=5})
print("ok")

--@ chunk 33
function island(cx, cy, len, th, seedv, k)
  local m = ellipse(cx, cy, len*0.5, th*0.5)
  for i=1,k do
    m = m + ellipse(cx + rand(-0.45,0.45)*len, cy + rand(-0.35,0.35)*th, rand(0.1,0.25)*len, rand(0.25,0.45)*th)
  end
  return m:roughen(th*0.25, th*0.8, seedv)
end
isl = {}
isl[1] = island(260, 575, 430, 85, 1, 7)    -- near left, big
isl[2] = island(720, 455, 300, 38, 2, 6)    -- mid right
isl[3] = island(540, 262, 200, 16, 3, 5)    -- far in gold
isl[4] = island(250, 365, 260, 28, 4, 6)    -- across willow foot / horizon line
isl[5] = island(905, 615, 220, 55, 5, 4)    -- near right, cut by edge
isl[6] = island(880, 340, 160, 20, 6, 4)    -- right willow foot
isl[7] = island(470, 470, 120, 22, 7, 3)    -- small mid
allIsl = isl[1]+isl[2]+isl[3]+isl[4]+isl[5]+isl[6]+isl[7]
print(allIsl:area())

--@ chunk 34
padR = pile{{"yellow ochre",1},{"rose madder",0.4},{"lead white",1.2},{"viridian",0.2}, name="padR"}
local n = noise{seed=41, period=50}
function islandPass(m, sc, seedv)
  work(m, {hand="body", tool="filbert 7", piles={{padB,1},{padD,function(x,y) return 0.7+0.6*n(x,y) end},{padG,function(x,y) return 0.5-0.6*n(x,y) end},{padR,0.25}},
    angle=function(x,y) return 0.04*math.sin(x/30+y) end, length={10,30}, coverage=1.3, fill=true, pressure={0.45,0.8}, scale_at=sc, edge={found=0.4, soft=0.5, lost=0.1, period=25}, curve={0.15,0.1}, seed=seedv})
end
islandPass(isl[2], 0.75, 201)
print("ok")

--@ chunk 35
padDk = pile{{"viridian",1},{"ultramarine blue",0.8},{"rose madder",0.3},{"lead white",0.25}, name="padDk"}
wslit = pile{{"lead white",2},{"cobalt blue",0.6},{"cobalt violet",0.35}, name="wslit"}
padY = pile{{"lead white",2},{"barium yellow",1},{"viridian",0.35},{"cadmium yellow",0.2}, name="padY"}
function padMarks(m, cx0, cy0, len, th, N, sc, seedv)
  local bD = brush{kind="filbert", width=4*sc, stiffness=0.5}
  local bL = brush{kind="filbert", width=4*sc, stiffness=0.5}
  local bS = brush{kind="filbert", width=2.6*sc, stiffness=0.5}
  local placed = 0
  for i=1,N*4 do
    if placed >= N then break end
    local x = cx0 + rand(-0.5,0.5)*len; local y = cy0 + rand(-0.5,0.5)*th
    if m:at(x,y) > 0.5 then
      placed = placed + 1
      local rx = rand(8,18)*sc; local ry = rx*rand(0.22,0.32)
      -- dark underside
      bD:load(padDk, 0.5)
      bD:stroke({{x-rx, y+ry*0.5}, {x, y+ry*0.9}, {x+rx*0.9, y+ry*0.4}}, {pressure={0.5,0.3}, ramps={0.1,0.3}})
      -- light top
      bL:load((i%3==0) and padY or padG, 0.5)
      bL:stroke({{x-rx*0.8, y-ry*0.3}, {x+rx*0.2, y-ry*0.6}, {x+rx*0.7, y-ry*0.2}}, {pressure={0.5,0.25}, ramps={0.1,0.3}})
      -- water slit between pads
      if i%2==0 then
        bS:load(wslit, 0.5)
        local sx = x + rx*rand(0.8,1.4)
        bS:stroke({{sx-rx*0.5, y+rand(-1,1)}, {sx+rx*0.4, y+rand(-1,1)}}, {pressure={0.45,0.2}, ramps={0.1,0.4}})
      end
    end
  end
  return placed
end
print(padMarks(isl[2], 720, 455, 300, 38, 22, 0.8, 1))

--@ chunk 36
r = rag{width=50}; r:dip(0.9)
r:wipe(isl[2]:grow(12), {pressure=0.85, angle=0, passes=3, refold=0.25, seed=6})
r = rag{width=50}; r:dip(0.9)
r:wipe(isl[2]:grow(12), {pressure=0.85, angle=0.1, passes=2, refold=0.25, seed=7})
print("ok")

--@ chunk 37
local n = noise{seed=42, period=40, stretch={0,2.5}}
function islandBroken(m, sc, seedv)
  -- deep layer, lower half weighted
  work(m, {hand="body", tool="filbert 6", piles={{padD,1},{padDk,0.5},{padB,function(x,y) return 0.5+0.5*n(x,y) end}},
    angle=function(x,y) return 0.05*n(x*2,y) end, length={8,22}, coverage=0.8, pressure={0.4,0.75}, load=0.6, scale_at=sc, edge={soft=0.6, lost=0.2, found=0.2, period=20}, curve={0.2,0.1}, seed=seedv})
  -- light tops
  work(m:times(function(x,y) return clamp(0.5+n(x,y),0,1) end), {hand="body", tool="filbert 5", piles={{padG,1},{padY,0.4},{padR,0.15}},
    angle=0, length={6,18}, coverage=0.6, pressure={0.35,0.65}, load=0.5, threshold=0.5, scale_at=sc, curve={0.25,0.1}, seed=seedv+1})
end
islandBroken(isl[2], 0.8, 301)
print("ok")

--@ chunk 38
blend(isl[2]:grow(6):soften(4), {angle=0, seed=311})
print("ok")

--@ chunk 39
local n = noise{seed=43, period=40, stretch={0,2.5}}
function islandUnder(m, sc, seedv, extra)
  local ps = {{padD,0.8},{padB,function(x,y) return 0.6+0.5*n(x,y) end},{padG,function(x,y) return 0.4-0.5*n(x,y) end}}
  if extra then table.insert(ps, extra) end
  work(m, {hand="body", tool="filbert 6", piles=ps,
    angle=function(x,y) return 0.05*n(x*2,y) end, length={8,22}, coverage=1.0, pressure={0.4,0.75}, load=0.6, scale_at=sc, edge={soft=0.6, lost=0.2, found=0.2, period=20}, curve={0.2,0.1}, seed=seedv})
end
islandUnder(isl[1], 1.3, 401)
islandUnder(isl[5], 1.2, 402)
islandUnder(isl[7], 0.8, 403)
islandUnder(isl[4], 0.6, 404, {wslit, 0.4})
islandUnder(isl[6], 0.55, 405, {wslit, 0.4})
islandUnder(isl[3], 0.45, 406, {padR, 0.5})
print("ok")

--@ chunk 40
for i,sc in pairs({[1]=1.3,[3]=0.45,[4]=0.6,[5]=1.2,[6]=0.55,[7]=0.8}) do
  blend(isl[i]:grow(4*sc):soften(3), {angle=0, tool={kind="badger", width=math.floor(16*sc)}, seed=500+i})
end
print("ok")

--@ chunk 41
wait(6*24*60) print(drying(250,575), drying(720,455), drying(540,262), drying(900,620))

--@ chunk 42
sk1 = pile{{"lead white",5},{"pale cadmium",0.5},{"rose madder",0.2},{"cobalt violet",0.2}, blot=0.2, name="sk1"}
sk2 = pile{{"lead white",4},{"cobalt violet",0.8},{"rose madder",0.2},{"cobalt blue",0.15}, blot=0.2, name="sk2"}
work(rect(450,100,150,80), {hand="scumble", tool={kind="filbert", width=7, stiffness=0.7}, piles={{sk1,1},{sk2,0.6}}, angle=0, length={20,50}, coverage=0.7, pressure={0.3,0.55}, load=0.35, seed=601})
print("ok")

--@ chunk 43
sk3 = pile{{"lead white",5},{"pale cadmium",0.9},{"deep cadmium",0.15},{"rose madder",0.1}, blot=0.3, name="sk3"}
local b = brush{kind="flat", width=8, stiffness=0.8}
for i=1,12 do
  b:load((i%2==0) and sk3 or sk2, 0.15)
  local x = rand(620,700); local y = rand(110,190)
  b:stroke({{x,y},{x+rand(30,60), y+rand(-2,2)}}, {pressure={0.25,0.15}, ramps={0.05,0.3}})
end
print("ok")

--@ chunk 44
rose = pile{{"lead white",5},{"rose madder",0.6},{"vermilion",0.15},{"pale cadmium",0.2}, blot=0.15, name="rose"}
sk3:add{{"deep cadmium",0.1}}
local gap = (rect(300,0,520,370) - crownM:grow(6)):soften(6)
local n = noise{seed=61, period=110, stretch={0,3}}
local function core(x,y) local dx=(x-580)/200; return math.max(0, 1-dx*dx-((y-120)/230)^2) end
work(gap, {hand="body", tool={kind="flat", width=7, stiffness=0.7}, piles={{sk3,function(x,y) return 0.2+1.6*core(x,y) end},{sk1,0.6},{rose,function(x,y) return 0.25+0.5*n(x,y)+0.6*smoothstep(150,330,y) end},{sk2,function(x,y) return 0.1+0.9*smoothstep(200,360,y) end}},
  angle=function(x,y) return 0.03*n(x,y*2) end, length={15,45}, coverage=1.1, pressure={0.25,0.45}, load=0.3, edge="soft", curve={0.1,0.1}, seed=611})
print("ok")

--@ chunk 45
r = rag{width=50}; r:dip(0.9)
local m = (rect(290,225,545,160) + rect(760,120,75,265)) 
r:wipe(m, {pressure=0.85, angle=0, passes=3, refold=0.25, seed=8})
r = rag{width=50}; r:dip(0.9)
r:wipe(m, {pressure=0.85, angle=0.08, passes=2, refold=0.25, seed=9})
print("ok")

--@ chunk 46
tb = pile{{"lead white",3},{"cobalt blue",0.6},{"cobalt violet",0.6},{"rose madder",0.1}, blot=0.1, name="tb"}
tb2 = pile{{"lead white",2.5},{"cobalt blue",0.9},{"ultramarine blue",0.3},{"cobalt violet",0.3}, blot=0.1, name="tb2"}
local band = mask(function(x,y) return math.exp(-((y-(355+20*math.sin(x/120)))/35)^2) end) - crownM:grow(5)
local n = noise{seed=71, period=90, stretch={0,3}}
work(band, {hand="body", tool={kind="flat", width=7, stiffness=0.7}, piles={{tb,function(x,y) return 1-smoothstep(330,400,y)+0.2 end},{tb2,function(x,y) return smoothstep(330,400,y)+0.2 end},{sk2,0.3}},
  angle=function(x,y) return 0.03*n(x,y) end, length={25,70}, coverage=0.9, pressure={0.25,0.45}, load=0.3, threshold=0.35, curve={0.1,0.1}, seed=701})
print("ok")

--@ chunk 47
r = rag{width=50}; r:dip(0.9)
local m = mask(function(x,y) return math.exp(-((y-(360+20*math.sin(x/120)))/30)^2) > 0.3 and 1 or 0 end) - crownM:grow(5)
r:wipe(m, {pressure=0.85, angle=0, passes=3, refold=0.25, seed=10})
print("ok")

--@ chunk 48
r = rag{width=40}; r:dip(0.9)
r:wipe(rect(20,355,280,70), {pressure=0.9, angle=0, passes=3, refold=0.2, seed=11})
r = rag{width=40}; r:dip(0.9)
r:wipe(rect(800,320,180,40), {pressure=0.9, angle=0, passes=3, refold=0.2, seed=12})
print("ok")

--@ chunk 49
wait(2*24*60)
gl1 = pile{{"lead white",4},{"cobalt violet",0.7},{"rose madder",0.15},{"pale cadmium",0.15}, blot=0.15, name="gl1"}
gl2 = pile{{"lead white",4},{"pale cadmium",0.6},{"cobalt violet",0.2}, blot=0.15, name="gl2"}
gl3 = pile{{"lead white",2},{"cobalt blue",0.6},{"viridian",0.3}, blot=0.15, name="gl3"}
-- trial: vertical glints in willow near the gap, left side
local b = brush{kind="round", width=4, point=0.5, stiffness=0.6}
for i=1,14 do
  local x = rand(250,370); local y = rand(60,300)
  if crownM:at(x,y) > 0.8 then
    b:load(({gl1,gl2,gl3})[i%3+1], 0.3)
    local L = rand(15,40)
    b:gesture({{x,y,0.2},{x+rand(-1,1),y+L*0.5,0.5},{x+rand(-2,2),y+L,0.05}}, {wobble=1})
  end
end
print(drying(300,150))

--@ chunk 50
r = rag{width=30}; r:dip(0.9)
r:wipe(rect(250,60,130,250), {pressure=0.85, angle=math.pi/2, passes=3, refold=0.2, seed=13})
print("ok")

--@ chunk 51
local edge = crownM:rim(60, 20) * crownM
local n = noise{seed=81, period=50, stretch={math.pi/2, 3}}
local m = edge:times(function(x,y) return clamp(0.4+0.9*n(x,y),0,1) end)
work(m * rect(0,0,500,420), {hand="scumble", tool={kind="filbert", width=8, stiffness=0.75}, piles={{gl1,1},{gl2,0.5},{gl3,0.5}}, angle=math.pi/2, length={15,40}, coverage=0.6, pressure={0.2,0.4}, load=0.2, threshold=0.4, seed=801})
print("ok")

--@ chunk 52
r = rag{width=40}; r:dip(0.9)
r:wipe(crownM:rim(70,20)*crownM*rect(0,0,500,430), {pressure=0.85, angle=math.pi/2, passes=3, refold=0.2, seed=14})
r = rag{width=40}; r:dip(0.9)
r:wipe(crownM:rim(70,20)*crownM*rect(0,0,500,430), {pressure=0.85, angle=0.1, passes=2, refold=0.2, seed=15})
print("ok")

--@ chunk 53
zv = pile{{"zinc white",3},{"cobalt violet",0.6},{"rose madder",0.1}, medium=0.35, name="zv"}
zg = pile{{"zinc white",3},{"pale cadmium",0.4}, thinner=0.5, name="zg"}
work(rect(30,50,90,100), {hand="scumble", tool={kind="filbert", width=8, stiffness=0.75}, pile=zv, angle=math.pi/2, length={15,40}, coverage=0.6, pressure={0.2,0.4}, load=0.25, clip=true, seed=811})
work(rect(140,50,90,100), {hand="scumble", tool={kind="filbert", width=8, stiffness=0.75}, pile=zg, angle=math.pi/2, length={15,40}, coverage=0.6, pressure={0.2,0.4}, load=0.25, clip=true, seed=812})
print("ok")

--@ chunk 54
r = rag{width=40}; r:dip(0.9)
r:wipe(rect(20,40,220,120), {pressure=0.9, angle=math.pi/2, passes=3, refold=0.2, seed=16})
print("ok")

--@ chunk 55
r = rag{width=30}; r:dip(1.0)
for _,y in ipairs({48,52,148,152}) do r:wipe({{20,y},{240,y}}, {pressure=0.95}); r:refold() end
for _,x in ipairs({30,34,118,122,140,232}) do r:wipe({{x,40},{x,160}}, {pressure=0.95}); r:refold() end
r = rag{width=40}; r:dip(1.0)
r:wipe(rect(80,330,240,80), {pressure=0.95, angle=0, passes=3, refold=0.2, seed=17})
r:wipe(rect(250,290,60,60), {pressure=0.95, angle=0, passes=2, refold=0.2, seed=18})
print("ok")

--@ chunk 56
t1 = pile{{"cobalt blue",1},{"viridian",0.8},{"lead white",0.7},{"barium yellow",0.2}, thinner=0.45, name="t1"}
t2 = pile{{"cobalt violet",1.2},{"rose madder",0.4},{"cobalt blue",0.4},{"lead white",0.5}, thinner=0.45, name="t2"}
t3 = pile{{"ultramarine blue",1},{"cobalt blue",0.6},{"lead white",0.8},{"cobalt violet",0.3}, thinner=0.45, name="t3"}
work(rect(30,50,200,130), {hand="body", tool={kind="filbert", width=6, stiffness=0.6}, piles={{t1,1},{t2,0.7},{t3,0.7}}, angle=function(x,y) return math.pi/2+0.1*math.sin(x/13) end, length={30,80}, coverage=1.5, pressure={0.3,0.6}, load=0.5, clip=true, seed=821})
print("ok")

--@ chunk 57
r = rag{width=40}; r:dip(1.0)
r:wipe(rect(25,45,210,140), {pressure=0.95, angle=math.pi/2, passes=3, refold=0.2, seed=19})
r:dip(1.0)
r:wipe(rect(25,45,210,140), {pressure=0.95, angle=0, passes=2, refold=0.2, seed=20})
print("ok")

--@ chunk 58
function colHalf(y) return lerp(230, 140, smoothstep(250, 667, y)) end
function colC(y) return lerp(570, 520, smoothstep(250,667,y)) + 15*math.sin(y/70) end
local col = mask(function(x,y) local d = math.abs(x-colC(y))/colHalf(y); return 1 - smoothstep(0.75, 1.15, d) end)
islAll = allIsl
lowerSides = (below(function(x) return 300 end) * (-col) * (-(allIsl:grow(4)))):soften(4)
print(lowerSides:area())

--@ chunk 59
ld1 = pile{{"ultramarine blue",2.2},{"cobalt violet",1},{"viridian",0.5},{"lead white",0.9},{"rose madder",0.15}, medium=0.12, name="ld1"}
ld2 = pile{{"viridian",1.4},{"cobalt blue",1},{"ultramarine blue",0.5},{"lead white",1}, medium=0.12, name="ld2"}
ld3 = pile{{"cobalt violet",1.5},{"ultramarine blue",1},{"rose madder",0.5},{"lead white",1}, medium=0.12, name="ld3"}
ld4 = pile{{"ultramarine blue",2},{"cobalt blue",1.2},{"lead white",1.4},{"cobalt violet",0.4}, medium=0.12, name="ld4"}
local n1 = noise{seed=91, period=80, stretch={math.pi/2, 3}}
local n2 = noise{seed=92, period=80, stretch={math.pi/2, 3}}
work(lowerSides, {hand="body", tool="filbert 10", piles={{ld1,1},{ld2,function(x,y) return clamp(0.3+0.8*n1(x,y),0,1) end},{ld3,function(x,y) return clamp(0.2+0.8*n2(x,y),0,1) end},{ld4,function(x,y) return 0.2+1.2*smoothstep(450,667,y) end}},
  angle=function(x,y) return math.pi/2 + 0.07*math.sin(x/35) end, length={30,90}, coverage=1.3, fill=true, pressure={0.55,0.9}, edge={soft=0.6,lost=0.3,found=0.1,period=50}, seed=901})
print("ok")

--@ chunk 60
local m = rect(0,270,420,397) + rect(650,270,350,397)
for k=1,2 do
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=math.pi/2, passes=2, refold=0.2, seed=30+k})
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=0, passes=2, refold=0.2, seed=40+k})
end
print("ok")

--@ chunk 61
wa = pile{{"ultramarine blue",1.5},{"cobalt blue",1},{"lead white",1.6},{"cobalt violet",0.4}, name="wa"}
wb = pile{{"cobalt blue",1},{"viridian",0.4},{"lead white",1.5},{"ultramarine blue",0.4}, name="wb"}
wc = pile{{"cobalt violet",1},{"ultramarine blue",1},{"lead white",1.3},{"rose madder",0.15}, name="wc"}
wdd = pile{{"ultramarine blue",2},{"cobalt violet",0.5},{"viridian",0.3},{"lead white",0.5}, name="wdd"}
work(rect(480,560,130,90), {hand="body", tool={kind="filbert", width=8, stiffness=0.6}, piles={{wa,1},{wb,0.6},{wc,0.6},{wdd,0.5}}, angle=function(x,y) return 0.05*math.sin(x/20+y/9) end, length={18,45}, coverage=2.2, pressure={0.45,0.8}, load=0.7, clip=true, curve={0.15,0.15}, seed=1001})
print("ok")

--@ chunk 62
r = rag{width=40}; r:dip(1.0)
r:wipe(rect(475,555,140,100), {pressure=0.95, angle=0, passes=3, refold=0.2, seed=50})
wab = pile{{"ultramarine blue",1.5},{"cobalt blue",1},{"lead white",1.6},{"cobalt violet",0.4}, blot=0.3, name="wab"}
wbb = pile{{"cobalt blue",1},{"viridian",0.5},{"lead white",1.5},{"ultramarine blue",0.4}, blot=0.3, name="wbb"}
wcb = pile{{"cobalt violet",1},{"ultramarine blue",1},{"lead white",1.2},{"rose madder",0.2}, blot=0.3, name="wcb"}
local tb = brush{kind="filbert", width=8, stiffness=0.7, lay=3, hair=0.6}
work(rect(480,560,130,90), {hand="body", tool=tb, piles={{wab,1},{wbb,0.7},{wcb,0.7}}, angle=function(x,y) return 0.05*math.sin(x/20+y/9) end, length={15,35}, coverage=1.2, pressure={0.4,0.7}, load=0.6, clip=true, curve={0.15,0.15}, seed=1002})
print("ok")

--@ chunk 63
r = rag{width=40}; r:dip(1.0)
r:wipe(rect(470,550,150,110), {pressure=0.95, angle=0, passes=3, refold=0.2, seed=51})
r:dip(1.0); r:wipe(rect(470,550,150,110), {pressure=0.95, angle=1.57, passes=2, refold=0.2, seed=52})
q1 = pile{{"ultramarine blue",2},{"cobalt blue",0.8},{"lead white",0.8},{"cobalt violet",0.3}, blot=0.25, name="q1"}
q2 = pile{{"cobalt blue",1.2},{"viridian",0.9},{"lead white",0.9}, blot=0.25, name="q2"}
q3 = pile{{"cobalt violet",1.5},{"ultramarine blue",0.8},{"rose madder",0.4},{"lead white",0.7}, blot=0.25, name="q3"}
q4 = pile{{"lead white",2},{"cobalt blue",0.8},{"cobalt violet",0.4},{"rose madder",0.1}, blot=0.25, name="q4"}
print("ok")

--@ chunk 64
local tb = brush{kind="filbert", width=8, stiffness=0.7, lay=3, hair=0.6}
work(rect(480,560,130,90), {hand="body", tool=tb, piles={{q1,1},{q2,0.7},{q3,0.7},{q4,0.3}}, mix_jitter=0.6, angle=function(x,y) return 0.05*math.sin(x/20+y/9) end, length={15,35}, coverage=1.0, pressure={0.4,0.7}, load=0.6, clip=true, curve={0.15,0.15}, seed=1003})
print("ok")

--@ chunk 65
u1 = pile{{"lead white",3},{"cobalt blue",0.6},{"cobalt violet",0.5},{"rose madder",0.1}, blot=0.25, name="u1"}
u2 = pile{{"lead white",3.5},{"pale cadmium",0.35},{"cobalt violet",0.35}, blot=0.25, name="u2"}
u3 = pile{{"lead white",2.5},{"cobalt blue",0.7},{"viridian",0.3}, blot=0.25, name="u3"}
local function deep(y) return smoothstep(430, 560, y) end
local cen = function(x,y) return math.exp(-((x-560)/170)^2) end
local m = (below(function(x) return 375 + 12*math.sin(x/100) end) - allIsl:grow(3)):soften(3)
local tb = brush{kind="filbert", width=8, stiffness=0.7, lay=3, hair=0.6}
work(m, {hand="body", tool=tb, piles={
   {q1,function(x,y) return 0.05+1.0*deep(y) end},{q2,function(x,y) return 0.05+0.6*deep(y) end},{q3,function(x,y) return 0.05+0.6*deep(y) end},{q4,function(x,y) return 0.3+0.3*(1-deep(y)) end},
   {u1,function(x,y) return 0.9*(1-deep(y)) end},{u2,function(x,y) return (1-deep(y))*cen(x,y)*1.2 end},{u3,function(x,y) return 0.5*(1-deep(y)) end}},
  mix_jitter=0.6, angle=function(x,y) return 0.05*math.sin(x/20+y/9) end, length={15,35}, coverage=1.0, pressure={0.4,0.7}, load=0.6, curve={0.15,0.15},
  scale_at=function(x,y) return 0.55+0.75*smoothstep(370,667,y) end, seed=1101})
print("ok")

--@ chunk 66
local m = (below(function(x) return 375 + 12*math.sin(x/100) end) - allIsl:grow(3)):soften(3)
blend(m, {angle=0, seed=1111})
print("ok")

--@ chunk 67
wait(4*24*60) print(drying(300,500), drying(550,420), drying(800,620), drying(250,575))

--@ chunk 68
wait(4*24*60) print(drying(300,500), drying(550,420), drying(800,620), drying(100,450), drying(520,600))

--@ chunk 69
wait(5*24*60) print(drying(300,500), drying(550,420), drying(800,620), drying(100,450), drying(520,600))

--@ chunk 70
g1 = pile{{"viridian",1},{"barium yellow",1},{"lead white",1.2}, blot=0.2, name="g1"}
g2 = pile{{"viridian",1},{"cobalt blue",0.5},{"lead white",1},{"yellow ochre",0.3}, blot=0.2, name="g2"}
g3 = pile{{"viridian",1},{"ultramarine blue",0.6},{"cobalt violet",0.4},{"lead white",0.6}, blot=0.2, name="g3"}
g4 = pile{{"barium yellow",1},{"lead white",1.5},{"viridian",0.3},{"pale cadmium",0.3}, blot=0.2, name="g4"}
g5 = pile{{"yellow ochre",0.8},{"rose madder",0.3},{"viridian",0.4},{"lead white",1}, blot=0.2, name="g5"}
local nn = noise{seed=131, period=30}
padBrush = brush{kind="filbert", width=6, stiffness=0.65, lay=2.5, hair=0.5}
function padsOn(m, sc, seedv, cov)
  work(m, {hand="body", tool=padBrush, piles={{g2,1},{g3,function(x,y) return 0.5+0.5*nn(x,y) end},{g1,function(x,y) return 0.6-0.5*nn(x,y) end},{g4,0.25},{g5,0.2}}, mix_jitter=0.5,
    angle=function(x,y) return 0.12*nn(x*3,y*3) end, length={8,20}, coverage=cov or 1.4, pressure={0.4,0.75}, load=0.6, curve={0.3,0.1}, scale_at=sc, edge={found=0.3,soft=0.5,lost=0.2,period=20}, seed=seedv})
end
padsOn(isl[7], 0.8, 1201)
print("ok")

--@ chunk 71
padsOn(isl[1], 1.5, 1202)
padsOn(isl[5], 1.4, 1203)
padsOn(isl[2], 0.9, 1204)
padsOn(isl[4], 0.7, 1205)
padsOn(isl[6], 0.6, 1206)
padsOn(isl[3], 0.5, 1207)
print("ok")

--@ chunk 72
for i,sc in pairs({[1]=1.5,[2]=0.9,[3]=0.5,[4]=0.7,[5]=1.4,[6]=0.6,[7]=0.8}) do
  blend(isl[i]:grow(3*sc):soften(2), {angle=0, tool={kind="badger", width=math.floor(18*sc)}, seed=1300+i})
end
print("ok")

--@ chunk 73
wait(4*24*60) print(drying(300,500), drying(520,600), drying(250,575), drying(700,450), drying(340,300))

--@ chunk 74
wf1 = pile{{"ultramarine blue",1.5},{"cobalt violet",1},{"lead white",1.2},{"viridian",0.3}, blot=0.2, name="wf1"}
wf2 = pile{{"lead white",2.5},{"cobalt violet",0.8},{"cobalt blue",0.4},{"rose madder",0.15}, blot=0.2, name="wf2"}
wf3 = pile{{"viridian",1},{"cobalt blue",0.8},{"lead white",1.1},{"ultramarine blue",0.3}, blot=0.2, name="wf3"}
local foot = (crownM:grow(45) - crownM:shrink(55)):times(function(x,y) return smoothstep(170,260,y) end) - allIsl:grow(3)
local tb = brush{kind="filbert", width=7, stiffness=0.7, lay=2.5, hair=0.6}
work(foot, {hand="body", tool=tb, piles={{wf1,1},{wf2,0.8},{wf3,0.6},{rv3,0.4}}, mix_jitter=0.6, angle=function(x,y) return 0.04*math.sin(x/17+y/7) end, length={14,40}, coverage=1.3, pressure={0.4,0.7}, load=0.55, curve={0.15,0.15}, threshold=0.4, seed=1401})
print("ok")

--@ chunk 75
local foot = (crownM:grow(55) - crownM:shrink(65)):times(function(x,y) return smoothstep(160,250,y) end):soften(6) - allIsl:grow(3)
blend(foot, {angle=0, seed=1411})
print("ok")

--@ chunk 76
local foot = (crownM:grow(70) - crownM:shrink(80)):times(function(x,y) return smoothstep(150,240,y) end)
for k=1,2 do
  r = rag{width=50}; r:dip(1.0)
  r:wipe(foot, {pressure=0.95, angle=0, passes=2, refold=0.2, seed=60+k})
  r = rag{width=50}; r:dip(1.0)
  r:wipe(foot, {pressure=0.95, angle=1.57, passes=1, refold=0.2, seed=70+k})
end
print("ok")

--@ chunk 77
wait(3*24*60) print(drying(250,575), drying(700,450), drying(550,150), drying(100,350))

--@ chunk 78
c1 = pile{{"lead white",5},{"rose madder",0.35},{"cobalt violet",0.25},{"pale cadmium",0.2}, name="c1"}
c2 = pile{{"lead white",5},{"cobalt blue",0.45},{"cobalt violet",0.45}, name="c2"}
c3 = pile{{"lead white",5},{"pale cadmium",0.9},{"deep cadmium",0.12}, name="c3"}
c4 = pile{{"lead white",5},{"barium yellow",0.6},{"viridian",0.06}, name="c4"}
gapM = (rect(280,0,560,380) - crownM:grow(3) - allIsl:grow(3)):soften(3)
local na = noise{seed=141, period=150, stretch={0,5}}
local nb = noise{seed=142, period=120, stretch={0,5}}
local tb = brush{kind="filbert", width=9, stiffness=0.6}
work(gapM, {hand="body", tool=tb, piles={{c3,function(x,y) return 0.3+0.8*(1-smoothstep(60,260,y)) end},{c1,function(x,y) return clamp(0.1+0.9*na(x,y),0,1) end},{c2,function(x,y) return clamp(0.05+0.9*nb(x,y),0,1)+0.4*smoothstep(200,380,y) end},{c4,0.2}}, mix_jitter=0.5,
  angle=function(x,y) return 0.03*math.sin(x/40+y/11) end, length={25,60}, coverage=1.2, pressure={0.45,0.75}, load=0.6, curve={0.1,0.1}, seed=1501})
print("ok")

--@ chunk 79
blend(rect(270,0,600,390), {angle=0, seed=1511})
print("ok")

--@ chunk 80
local m = rect(260,0,620,400)
for k=1,3 do
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=0, passes=2, refold=0.15, seed=80+k})
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=1.57, passes=1, refold=0.15, seed=90+k})
end
print(drying(540,262), drying(880,340))

--@ chunk 81
wait(5*24*60) print(drying(250,575), drying(700,450), drying(540,262), drying(880,340), drying(900,620))

--@ chunk 82
slitA = pile{{"ultramarine blue",1.2},{"cobalt blue",0.8},{"cobalt violet",0.5},{"lead white",1.3}, blot=0.15, name="slitA"}
accD = pile{{"viridian",1},{"ultramarine blue",0.9},{"cobalt violet",0.5},{"rose madder",0.2},{"lead white",0.3}, blot=0.15, name="accD"}
topY = pile{{"lead white",2},{"barium yellow",1},{"viridian",0.3},{"pale cadmium",0.3}, blot=0.15, name="topY"}
flP = pile{{"lead white",3},{"rose madder",0.8},{"vermilion",0.15}, blot=0.2, name="flP"}
flW = pile{{"lead white",4},{"pale cadmium",0.15}, blot=0.2, name="flW"}
function structureIsland(m, cx, cy, len, th, sc, nSlit, nAcc, nTop, nFl)
  local b = brush{kind="filbert", width=3.2*sc, stiffness=0.6}
  local inside = function(x,y) return m:at(x,y) > 0.5 end
  local function pick()
    for t=1,40 do local x = cx + rand(-0.5,0.5)*len; local y = cy + rand(-0.5,0.5)*th; if inside(x,y) then return x,y end end
    return nil
  end
  for i=1,nSlit do local x,y = pick(); if x then
    b:load(slitA, 0.5); local L = rand(10,26)*sc
    b:stroke({{x-L/2, y+rand(-0.5,0.5)}, {x, y+rand(-1,1)*0.6}, {x+L/2, y+rand(-0.5,0.5)}}, {pressure={0.45,0.15}, ramps={0.1,0.4}}) end end
  for i=1,nAcc do local x,y = pick(); if x then
    b:load(accD, 0.45); local L = rand(6,16)*sc
    b:stroke({{x-L/2, y}, {x+L/2, y+rand(-0.6,0.6)}}, {pressure={0.4,0.15}, ramps={0.1,0.5}}) end end
  for i=1,nTop do local x,y = pick(); if x then
    b:load(topY, 0.4); local L = rand(5,13)*sc
    b:stroke({{x-L/2, y}, {x+L/2, y+rand(-0.6,0.6)}}, {pressure={0.4,0.2}, ramps={0.1,0.5}}) end end
  local fb = brush{kind="round", width=3.5*sc, stiffness=0.5}
  for i=1,nFl do local x,y = pick(); if x then
    fb:load((i%3==0) and flW or flP, 0.6)
    fb:touch(x, y, {pressure=0.6, drag={2*sc, 0}})
    fb:touch(x+1.2*sc, y-0.8*sc, {pressure=0.4, drag={1.5*sc, 0}}) end end
end
structureIsland(isl[2], 720, 455, 300, 38, 0.9, 14, 18, 14, 4)
print("ok")

--@ chunk 83
r = rag{width=40}; r:dip(1.0)
r:wipe(isl[2]:grow(10), {pressure=0.95, angle=0, passes=3, refold=0.15, seed=101})
print("ok")

--@ chunk 84
local nn = noise{seed=151, period=25}
padB2 = brush{kind="filbert", width=5, stiffness=0.65, lay=2, hair=0.5}
function padsOn2(m, sc, seedv, cov)
  work(m, {hand="body", tool=padB2, piles={{g2,1},{g3,function(x,y) return 0.6+0.5*nn(x,y) end},{g1,function(x,y) return 0.5-0.5*nn(x,y) end},{g4,0.2},{g5,0.2},{accD,0.35},{slitA,0.3}}, mix_jitter=0.6,
    angle=function(x,y) return 0.06*nn(x*3,y*3) end, length={10,22}, coverage=cov or 1.1, pressure={0.35,0.6}, load=0.55, curve={0.35,0.05}, scale_at=sc, edge={found=0.3,soft=0.5,lost=0.2,period=20}, clip=m:grow(4*sc):soften(2), seed=seedv})
end
padsOn2(isl[2], 0.85, 1601)
print("ok")

--@ chunk 85
padsOn2(isl[1], 1.5, 1602)
padsOn2(isl[5], 1.4, 1603)
padsOn2(isl[7], 0.85, 1604)
padsOn2(isl[4], 0.7, 1605)
padsOn2(isl[6], 0.6, 1606)
padsOn2(isl[3], 0.5, 1607)
print("ok")

--@ chunk 86
wait(6*24*60) print(drying(250,575), drying(700,450), drying(540,262), drying(500,620))

--@ chunk 87
wait(3*24*60)
gz = pile{{"ultramarine blue",1.5},{"cobalt violet",0.6},{"rose madder",0.15},{"viridian",0.15}, medium=0.7, name="gz"}
local m = (below(function(x) return 470 end) - allIsl:grow(2)):times(function(x,y) return smoothstep(470,600,y) end)
work(rect(420,600,120,60) - allIsl:grow(2), {hand="glaze", pile=gz, angle=0, coverage=1, clip=true, seed=1701})
print(drying(700,450))

--@ chunk 88
r = rag{width=40}; r:dip(1.0)
r:wipe(rect(410,590,140,77), {pressure=0.95, angle=0, passes=3, refold=0.15, seed=111})
gz2 = pile{{"ultramarine blue",1},{"cobalt violet",0.5},{"rose madder",0.1}, medium=0.9, name="gz2"}
work(rect(420,600,120,60), {hand="glaze", pile=gz2, angle=0, coverage=1, clip=true, seed=1702})
r = rag{width=50}
r:wipe(rect(420,600,60,60), {pressure=0.4, angle=0, passes=1, seed=112})
print("ok")

--@ chunk 89
for k=1,2 do r = rag{width=40}; r:dip(1.0)
r:wipe(rect(410,590,140,77), {pressure=0.95, angle=0, passes=2, refold=0.15, seed=113+k}) end
print("ok")

--@ chunk 90
e1 = pile{{"ultramarine blue",2},{"cobalt violet",0.8},{"lead white",0.7},{"viridian",0.2}, blot=0.2, name="e1"}
e2 = pile{{"ultramarine blue",1.2},{"viridian",0.8},{"cobalt blue",0.5},{"lead white",0.6}, blot=0.2, name="e2"}
e3 = pile{{"cobalt violet",1.4},{"ultramarine blue",1},{"rose madder",0.4},{"lead white",0.6}, blot=0.2, name="e3"}
local m = (below(function(x) return 470 + 15*math.sin(x/90) end) - allIsl:grow(3)):times(function(x,y) return smoothstep(470,590,y) end)
lowWater = m
local tb = brush{kind="filbert", width=9, stiffness=0.7, lay=2.5, hair=0.6}
work(m, {hand="body", tool=tb, piles={{e1,1},{e2,0.6},{e3,0.6},{q4,0.15}}, mix_jitter=0.6, angle=function(x,y) return 0.05*math.sin(x/23+y/9) end, length={18,40}, coverage=0.85, pressure={0.4,0.7}, load=0.55, curve={0.15,0.15}, threshold=0.25, scale_at=1.2, clip=below(function(x) return 455 end) - allIsl:grow(2), seed=1801})
blend(m:grow(6):soften(8) - allIsl:grow(2), {angle=0, seed=1802})
print("ok")

--@ chunk 91
local band = (rect(0,485,1000,60):soften(10)) - allIsl:grow(2)
blend(band, {angle=0, seed=1811})
blend(band, {angle=0.03, seed=1812})
blend(band, {angle=-0.03, seed=1813})
print("ok")

--@ chunk 92
local band = rect(0,460,1000,50):soften(14) - allIsl:grow(2)
blend(band, {angle=math.pi/2, seed=1821})
blend(band, {angle=math.pi/2+0.2, seed=1822})
print(drying(300,480), drying(300,510))

--@ chunk 93
wait(7*24*60) print(drying(300,500), drying(450,620), drying(250,575), drying(700,450), drying(540,150))

--@ chunk 94
fv1 = pile{{"ultramarine blue",1.2},{"viridian",0.8},{"cobalt violet",0.6},{"lead white",0.6}, thinner=0.3, name="fv1"}
fv2 = pile{{"cobalt violet",1.2},{"rose madder",0.4},{"ultramarine blue",0.6},{"lead white",0.8}, thinner=0.3, name="fv2"}
fv3 = pile{{"viridian",1},{"cobalt blue",0.6},{"yellow ochre",0.3},{"lead white",0.9}, thinner=0.3, name="fv3"}
frB = brush{kind="round", width=5, point=0.7, stiffness=0.5}
function frond(x, y0, len, pile_, p)
  frB:load(pile_, rand(0.45,0.65))
  local dx1, dx2 = rand(-3,3), rand(-6,6)
  frB:gesture({{x, y0, p*0.6},{x+dx1, y0+len*0.35, p},{x+dx2*0.7, y0+len*0.7, p*0.7},{x+dx2, y0+len, 0.0}}, {wobble=1.5})
end
for i=1,10 do
  local x = rand(345, 420)
  local y0 = rand(0, 200)
  frond(x, y0, rand(60,160), ({fv1,fv2,fv3})[i%3+1], rand(0.45,0.7))
end
print("ok")

--@ chunk 95
r = rag{width=30}; r:dip(1.0)
r:wipe(rect(340,0,90,330), {pressure=0.9, angle=math.pi/2, passes=2, refold=0.15, seed=121})
fv1b = pile{{"ultramarine blue",1.2},{"viridian",0.8},{"cobalt violet",0.6},{"lead white",0.6}, medium=0.1, name="fv1b"}
fv2b = pile{{"cobalt violet",1.2},{"rose madder",0.4},{"ultramarine blue",0.6},{"lead white",0.9}, medium=0.1, name="fv2b"}
fv3b = pile{{"viridian",1},{"cobalt blue",0.6},{"yellow ochre",0.3},{"lead white",1}, medium=0.1, name="fv3b"}
frB = brush{kind="filbert", width=6, stiffness=0.5}
for i=1,10 do
  local x = rand(345, 420)
  local y0 = rand(0, 200)
  frB:load(({fv1b,fv2b,fv3b})[i%3+1], rand(0.35,0.55))
  local len = rand(60,160)
  frB:gesture({{x, y0, 0.5},{x+rand(-2,2), y0+len*0.4, 0.6},{x+rand(-4,4), y0+len, 0.15}}, {wobble=1.5})
end
print("ok")

--@ chunk 96
r = rag{width=30}; r:dip(1.0)
r:wipe(rect(340,50,90,290), {pressure=0.95, angle=math.pi/2, passes=3, refold=0.15, seed=122})
frB = brush{kind="filbert", width=9, stiffness=0.8}
for i=1,8 do
  local x = rand(350, 420)
  local y0 = rand(20, 180)
  frB:reload(({fv1b,fv2b,fv3b})[i%3+1], 0.12)
  local len = rand(60,150)
  frB:stroke({{x, y0},{x+rand(-2,2), y0+len*0.5},{x+rand(-3,3), y0+len}}, {pressure={0.3,0.15}, ramps={0.1,0.4}})
end
print("ok")

--@ chunk 97
r = rag{width=30}; r:dip(1.0)
r:wipe(rect(340,50,90,290), {pressure=0.95, angle=math.pi/2, passes=3, refold=0.15, seed=123})
frB = brush{kind="filbert", width=7, stiffness=0.5}
for i=1,9 do
  local x = rand(345, 425)
  local y0 = rand(10, 160)
  frB:reload(({fv1b,fv2b,fv3b})[i%3+1], 0.45)
  local len = rand(70,170)
  frB:stroke({{x, y0},{x+rand(-2,2), y0+len*0.5},{x+rand(-3,3), y0+len}}, {pressure={0.55,0.2}, ramps={0.1,0.5}})
end
blend(rect(330,0,110,350):soften(10), {angle=math.pi/2, tool={kind="badger", width=20}, seed=1901})
print("ok")

--@ chunk 98
function frondPatch(x0, x1, y0a, y0b, lmin, lmax, n, piles, load, bmask)
  local fb = brush{kind="filbert", width=7, stiffness=0.5}
  for i=1,n do
    local x = rand(x0, x1); local y0 = rand(y0a, y0b)
    fb:reload(piles[(i % #piles)+1], load)
    local len = rand(lmin,lmax)
    fb:stroke({{x, y0},{x+rand(-2,2), y0+len*0.5},{x+rand(-3,3), y0+len}}, {pressure={0.55,0.2}, ramps={0.1,0.5}})
  end
  blend(bmask, {angle=math.pi/2, tool={kind="badger", width=20}, seed=math.floor(x0+y0a)})
end
-- right cavity edge
frondPatch(705, 790, 0, 160, 70, 170, 9, {fv1b,fv2b,fv3b}, 0.45, rect(695,0,110,330):soften(10) - allIsl:grow(3))
-- left upper edge near top
frondPatch(395, 440, 0, 60, 50, 120, 4, {fv1b,fv3b}, 0.4, rect(385,0,65,200):soften(8))
print("ok")

--@ chunk 99
blend(rect(675,0,40,335):soften(12) - allIsl:grow(3), {angle=0, tool={kind="badger", width=14}, seed=1951})
blend(rect(675,0,40,335):soften(12) - allIsl:grow(3), {angle=math.pi/2, tool={kind="badger", width=14}, seed=1952})
print("ok")

--@ chunk 100
wait(5*24*60) print(drying(380,150), drying(740,150), drying(450,620), drying(560,100))

--@ chunk 101
h1 = pile{{"lead white",4},{"pale cadmium",0.8},{"deep cadmium",0.2},{"rose madder",0.1}, blot=0.15, name="h1"}
h2 = pile{{"lead white",4},{"rose madder",0.3},{"pale cadmium",0.3},{"cobalt violet",0.15}, blot=0.15, name="h2"}
h3 = pile{{"lead white",4},{"barium yellow",0.7},{"pale cadmium",0.2}, blot=0.15, name="h3"}
h4 = pile{{"lead white",4},{"cobalt violet",0.6},{"rose madder",0.1},{"cobalt blue",0.1}, blot=0.15, name="h4"}
local na = noise{seed=161, period=90, stretch={0,4}}
local nb = noise{seed=162, period=70, stretch={0,4}}
local tm = rect(470,40,150,90)
local tb = brush{kind="filbert", width=8, stiffness=0.6, lay=2}
work(tm, {hand="body", tool=tb, piles={{h1,function(x,y) return clamp(0.4+0.8*na(x,y),0,1) end},{h2,function(x,y) return clamp(0.3+0.8*nb(x,y),0,1) end},{h3,0.4},{h4,0.25}}, mix_jitter=0.5, angle=function(x,y) return 0.03*na(x*2,y*2) end, length={20,45}, coverage=1.0, pressure={0.4,0.65}, load=0.5, curve={0.1,0.1}, clip=true, seed=2001})
blend(tm, {angle=0, seed=2002})
print("ok")

--@ chunk 102
local na = noise{seed=163, period=110, stretch={0,4}}
local nb = noise{seed=164, period=80, stretch={0,4}}
goldM = (ellipse(565, 90, 200, 190):soften(25) - crownM:grow(15)):times(function(x,y) return 1 end)
local tb = brush{kind="filbert", width=9, stiffness=0.6, lay=2}
work(goldM, {hand="body", tool=tb, piles={{h1,function(x,y) return clamp(0.3+0.8*na(x,y),0,1)*(1-smoothstep(120,260,y))+0.05 end},{h2,function(x,y) return clamp(0.2+0.8*nb(x,y),0,1)+0.3*smoothstep(150,280,y) end},{h3,function(x,y) return 0.5*(1-smoothstep(60,200,y)) end},{h4,function(x,y) return 0.1+0.5*smoothstep(170,300,y) end}}, mix_jitter=0.5,
  angle=function(x,y) return 0.03*na(x*2,y*2) end, length={20,55}, coverage=1.0, pressure={0.4,0.65}, load=0.5, curve={0.1,0.1}, threshold=0.35, clip=-(crownM:grow(4)), seed=2011})
blend(goldM:soften(10), {angle=0, seed=2012})
print("ok")

--@ chunk 103
local m = ellipse(565, 110, 230, 200) - crownM:shrink(10)
for k=1,3 do
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=0, passes=2, refold=0.15, seed=130+k})
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=1.57, passes=1, refold=0.15, seed=140+k})
end
print("ok")

--@ chunk 104
print(drying(470,470), drying(250,575), drying(720,455))
k1 = pile{{"viridian",1},{"ultramarine blue",0.5},{"yellow ochre",0.4},{"lead white",0.5}, blot=0.15, name="k1"}
k2 = pile{{"viridian",1},{"barium yellow",0.6},{"cobalt blue",0.3},{"lead white",0.6}, blot=0.15, name="k2"}
local nn = noise{seed=171, period=20}
function padsDark(m, sc, seedv, cov)
  local tb = brush{kind="filbert", width=5, stiffness=0.6, lay=2, hair=0.5}
  work(m:shrink(2*sc), {hand="body", tool=tb, piles={{k1,1},{padDk,0.6},{accD,0.5},{k2,function(x,y) return 0.4+0.4*nn(x,y) end},{slitA,0.25}}, mix_jitter=0.6,
    angle=function(x,y) return 0.05*nn(x*3,y*3) end, length={9,20}, coverage=cov or 0.8, pressure={0.4,0.65}, load=0.5, curve={0.35,0.05}, scale_at=sc, clip=m, seed=seedv})
end
padsDark(isl[7], 0.85, 2101)
print("ok")

--@ chunk 105
r = rag{width=40}; r:dip(1.0)
r:wipe(isl[7]:grow(6), {pressure=0.95, angle=0, passes=3, refold=0.15, seed=150})
padBr = brush{kind="filbert", width=6, stiffness=0.55}
function onePad(x, y, rx, lightP, darkP)
  local w = rx*0.55
  padBr = brush{kind="filbert", width=math.max(2.5,w), stiffness=0.55}
  padBr:load(darkP, 0.6)
  padBr:load(lightP, 0.6, {side=-1, share=0.45})
  local tilt = rand(-0.04,0.04)
  padBr:stroke({{x-rx, y+rx*tilt}, {x, y-rx*0.05}, {x+rx, y-rx*tilt}}, {pressure={0.7,0.5}, ramps={0.2,0.35}, orient="along"})
end
local cnt=0
for i=1,60 do
  local x = 470 + rand(-60,60); local y = 470 + rand(-12,12)
  if isl[7]:at(x,y) > 0.6 then cnt=cnt+1; onePad(x, y, rand(7,13), (i%3==0) and topY or k2, (i%2==0) and k1 or accD) end
  if cnt >= 14 then break end
end
print(cnt)

--@ chunk 106
r = rag{width=40}; r:dip(1.0)
r:wipe(isl[7]:grow(14), {pressure=0.95, angle=0, passes=3, refold=0.15, seed=151})
r = rag{width=40}; r:dip(1.0)
r:wipe(isl[7]:grow(14), {pressure=0.95, angle=0.2, passes=2, refold=0.15, seed=152})
print("ok")

--@ chunk 107
m1 = pile{{"lead white",3},{"cobalt violet",1},{"rose madder",0.2},{"cobalt blue",0.3}, blot=0.15, name="m1"}
m2 = pile{{"lead white",2.2},{"cobalt blue",1},{"cobalt violet",0.5},{"ultramarine blue",0.3}, blot=0.15, name="m2"}
m3 = pile{{"lead white",1.4},{"ultramarine blue",1},{"cobalt blue",0.8},{"cobalt violet",0.4}, blot=0.15, name="m3"}
m4 = pile{{"lead white",1.2},{"ultramarine blue",1},{"viridian",0.4},{"cobalt violet",0.5}, blot=0.15, name="m4"}
local function t(y) return smoothstep(330, 520, y) end
local side = function(x) return 1 - math.exp(-((x-560)/220)^2) end
bandM = (rect(0,330,1000,200):soften(18) - allIsl:grow(3) - crownM:grow(4)):times(function(x,y) return 1 end)
local tb = brush{kind="filbert", width=9, stiffness=0.65, lay=2, hair=0.6}
work(bandM, {hand="body", tool=tb, piles={
   {m1,function(x,y) return (1-t(y))*(1-0.7*side(x)) + 0.02 end},
   {m2,function(x,y) return math.exp(-((t(y)-0.4)/0.3)^2) + 0.3*side(x)*(1-t(y)) end},
   {m3,function(x,y) return math.exp(-((t(y)-0.75)/0.25)^2) end},
   {m4,function(x,y) return t(y)*0.6*side(x) + 0.02 end}},
  mix_jitter=0.5, angle=function(x,y) return 0.04*math.sin(x/23+y/9) end, length={18,45}, coverage=1.1, pressure={0.45,0.75}, load=0.6, curve={0.12,0.12}, threshold=0.3,
  scale_at=function(x,y) return 0.7+0.4*t(y) end, clip=-(allIsl:grow(2)) * -(crownM:grow(2)), seed=2201})
blend(bandM, {angle=0, seed=2202})
print("ok")

--@ chunk 108
local s1 = (rect(0,312,1000,36):soften(10) - allIsl:grow(3) - crownM:grow(3))
local s2 = (rect(0,515,1000,40):soften(10) - allIsl:grow(3))
for k=1,2 do
  blend(s1, {angle=math.pi/2 + 0.15*(k-1.5), tool={kind="badger", width=24}, seed=2210+k})
  blend(s2, {angle=math.pi/2 + 0.15*(k-1.5), tool={kind="badger", width=24}, seed=2220+k})
end
print("ok")

--@ chunk 109
local tb = brush{kind="filbert", width=8, stiffness=0.65, lay=2, hair=0.6}
local seamA = (rect(250,292,750,30) - allIsl:grow(3) - crownM:grow(3)):roughen(10, 40, 21)
local seamB = (rect(0,540,1000,30) - allIsl:grow(3)):roughen(10, 40, 22)
local n = noise{seed=181, period=60}
work(seamA, {hand="body", tool=tb, piles={{m1,1},{lilac,0.5},{m2,function(x,y) return 0.3+0.3*n(x,y) end}}, mix_jitter=0.5, angle=function(x,y) return 0.05*n(x*3,y) end, length={15,40}, coverage=1.2, pressure={0.4,0.7}, load=0.5, curve={0.15,0.1}, threshold=0.3, clip=-(allIsl:grow(2)) * -(crownM:grow(2)), seed=2301})
work(seamB, {hand="body", tool=tb, piles={{m3,1},{m4,0.6},{e1,0.5},{e3,0.3}}, mix_jitter=0.5, angle=function(x,y) return 0.05*n(x*3,y) end, length={18,45}, coverage=1.2, pressure={0.4,0.7}, load=0.5, curve={0.15,0.1}, threshold=0.3, scale_at=1.2, clip=-(allIsl:grow(2)), seed=2302})
print("ok")

--@ chunk 110
local seamA = (rect(250,285,750,45) - allIsl:grow(3) - crownM:grow(3)):roughen(12, 50, 31)
local seamB = (rect(0,532,1000,45) - allIsl:grow(3)):roughen(12, 50, 32)
blend(seamA, {angle=0, seed=2311})
blend(seamB, {angle=0, seed=2312})
print("ok")

--@ chunk 111
wait(6*24*60) print(drying(500,400), drying(300,320), drying(500,550), drying(250,575))

--@ chunk 112
islC = {{260,575,85,1.5},{720,455,38,0.9},{540,262,16,0.5},{250,365,28,0.7},{905,615,55,1.4},{880,340,20,0.6},{470,470,22,0.85}}
local nn = noise{seed=191, period=18}
function shadeIsland(i, seedv, cov)
  local c = islC[i]; local cy, th, sc = c[2], c[3], c[4]
  local m = isl[i]:times(function(x,y) return clamp(smoothstep(cy - th*0.15, cy + th*0.35, y) + 0.35*nn(x,y), 0, 1) end)
  local tb = brush{kind="filbert", width=5, stiffness=0.6, lay=1.5, hair=0.5}
  work(m, {hand="body", tool=tb, piles={{accD,1},{padDk,0.7},{k1,0.5},{slitA,0.3}}, mix_jitter=0.5, angle=function(x,y) return 0.04*nn(x*2,y*2) end, length={10,24}, coverage=cov or 0.9, pressure={0.4,0.65}, load=0.5, curve={0.3,0.05}, threshold=0.45, scale_at=sc, clip=isl[i]:grow(2*sc), seed=seedv})
end
shadeIsland(7, 2401)
print("ok")

--@ chunk 113
blend(isl[7]:grow(2), {angle=0, tool={kind="badger", width=12}, seed=2402})
print("ok")

--@ chunk 114
for _,i in ipairs({1,2,3,4,5,6}) do
  shadeIsland(i, 2410+i)
  local sc = islC[i][4]
  blend(isl[i]:grow(2*sc), {angle=0, tool={kind="badger", width=math.floor(14*sc)}, seed=2420+i})
end
print("ok")

--@ chunk 115
wait(5*24*60) print(drying(250,575), drying(720,455), drying(470,470), drying(540,262))

--@ chunk 116
lp1 = pile{{"lead white",2},{"barium yellow",1},{"viridian",0.4}, blot=0.2, name="lp1"}
lp2 = pile{{"lead white",1.6},{"viridian",0.7},{"barium yellow",0.6},{"cobalt blue",0.2}, blot=0.2, name="lp2"}
lp3 = pile{{"lead white",2},{"cobalt blue",0.5},{"viridian",0.4}, blot=0.2, name="lp3"}
local nn = noise{seed=201, period=16}
function lightPads(i, seedv, cov)
  local c = islC[i]; local cy, th, sc = c[2], c[3], c[4]
  local m = isl[i]:shrink(1.5*sc):times(function(x,y) return clamp((1 - smoothstep(cy - th*0.2, cy + th*0.3, y)) + 0.6*nn(x,y), 0, 1) end)
  local tb = brush{kind="filbert", width=4.5, stiffness=0.6, lay=2, hair=0.5}
  work(m, {hand="body", tool=tb, piles={{lp1,1},{lp2,1},{lp3,0.5},{padR,0.15}}, mix_jitter=0.5, angle=function(x,y) return 0.05*nn(x*2,y*2) end, length={7,16}, coverage=cov or 0.45, pressure={0.4,0.6}, load=0.5, curve={0.4,0.05}, threshold=0.55, scale_at=sc, clip=isl[i], seed=seedv})
end
lightPads(7, 2501)
print("ok")

--@ chunk 117
r = rag{width=30}; r:dip(1.0)
r:wipe(isl[7]:grow(4), {pressure=0.95, angle=0, passes=3, refold=0.15, seed=160})
lp4 = pile{{"viridian",1},{"barium yellow",0.8},{"lead white",0.8},{"yellow ochre",0.3}, blot=0.2, name="lp4"}
lp5 = pile{{"viridian",1},{"cobalt blue",0.3},{"lead white",0.9},{"barium yellow",0.3}, blot=0.2, name="lp5"}
local nn = noise{seed=202, period=16}
function lightPads2(i, seedv, cov)
  local c = islC[i]; local cy, th, sc = c[2], c[3], c[4]
  local m = isl[i]:shrink(1.5*sc):times(function(x,y) return clamp((1 - smoothstep(cy - th*0.2, cy + th*0.3, y)) + 0.6*nn(x,y), 0, 1) end)
  local tb = brush{kind="filbert", width=3.5, stiffness=0.6, lay=1.8, hair=0.5}
  work(m, {hand="body", tool=tb, piles={{lp4,1},{lp5,1},{lp2,0.4},{padR,0.12}}, mix_jitter=0.5, angle=function(x,y) return 0.04*nn(x*2,y*2) end, length={10,20}, coverage=cov or 0.5, pressure={0.4,0.6}, load=0.5, curve={0.35,0.03}, threshold=0.55, scale_at=sc, clip=isl[i], seed=seedv})
end
lightPads2(7, 2502)
print("ok")

--@ chunk 118
for _,i in ipairs({1,2,4,5,6}) do lightPads2(i, 2510+i) end
lightPads2(3, 2513, 0.4)
print("ok")

--@ chunk 119
s1 = pile{{"lead white",4},{"rose madder",0.25},{"pale cadmium",0.35}, thinner=0.5, name="s1"}
s2 = pile{{"lead white",4},{"cobalt violet",0.35},{"pale cadmium",0.2}, thinner=0.5, name="s2"}
s3 = pile{{"lead white",4},{"pale cadmium",0.9},{"deep cadmium",0.1}, thinner=0.5, name="s3"}
local tb = brush{kind="filbert", width=12, stiffness=0.7}
work(rect(480,40,140,70), {hand="scumble", tool=tb, piles={{s1,1},{s2,0.7},{s3,0.8}}, mix_jitter=0.4, angle=0, length={30,70}, coverage=0.8, pressure={0.3,0.5}, load=0.4, clip=true, seed=2601})
print("ok")

--@ chunk 120
s4 = pile{{"lead white",3},{"pale cadmium",1},{"deep cadmium",0.25},{"rose madder",0.05}, thinner=0.5, name="s4"}
s5 = pile{{"lead white",3.5},{"cobalt violet",0.6},{"rose madder",0.15},{"cobalt blue",0.1}, thinner=0.5, name="s5"}
local core = function(x,y) return math.exp(-((x-575)/110)^2 - ((y-70)/110)^2) end
local gap2 = (rect(300,0,520,300) - crownM:grow(4) - allIsl:grow(3)):roughen(6,30,41)
local tb = brush{kind="filbert", width=12, stiffness=0.7}
local n = noise{seed=211, period=120, stretch={0,4}}
work(gap2, {hand="scumble", tool=tb, piles={{s4,function(x,y) return 1.4*core(x,y) end},{s3,function(x,y) return 0.5*(1-smoothstep(80,220,y)) end},{s1,function(x,y) return clamp(0.3+0.6*n(x,y),0,1)*smoothstep(60,200,y)+0.1 end},{s5,function(x,y) return 0.05+0.8*smoothstep(170,300,y) end},{s2,0.25}}, mix_jitter=0.4,
  angle=function(x,y) return 0.02*n(x,y) end, length={30,80}, coverage=0.9, pressure={0.3,0.5}, load=0.4, clip=-(crownM:grow(3)) * -(allIsl:grow(2)), seed=2611})
print("ok")

--@ chunk 121
wait(3*24*60) print(drying(560,100), drying(560,250), drying(250,575))

--@ chunk 122
cl1 = pile{{"lead white",3},{"cobalt violet",0.7},{"cobalt blue",0.3},{"rose madder",0.1}, blot=0.1, name="cl1"}
cl2 = pile{{"lead white",3},{"rose madder",0.3},{"cobalt violet",0.3},{"pale cadmium",0.2}, blot=0.1, name="cl2"}
cl3 = pile{{"lead white",3},{"cobalt blue",0.5},{"viridian",0.15}, blot=0.1, name="cl3"}
local n = noise{seed=221, period=70, stretch={0,6}}
local inGap = (rect(320,0,500,300) - crownM:grow(3) - allIsl:grow(3))
local cm = inGap:times(function(x,y) local v = n(x,y); return smoothstep(0.15, 0.45, v) * (0.4 + 0.6*smoothstep(40,200,y)) end)
local tb = brush{kind="filbert", width=9, stiffness=0.6, lay=1.5}
work(cm, {hand="body", tool=tb, piles={{cl1,1},{cl2,0.7},{cl3,0.5}}, mix_jitter=0.5, angle=function(x,y) return 0.03*n(x*2,y) end, length={30,70}, coverage=0.9, pressure={0.35,0.6}, load=0.45, curve={0.08,0.08}, threshold=0.4, clip=inGap, seed=2701})
blend(inGap:shrink(2), {angle=0, seed=2702})
print("ok")

--@ chunk 123
vv1 = pile{{"lead white",2.5},{"cobalt violet",1},{"rose madder",0.2},{"cobalt blue",0.3}, blot=0.1, name="vv1"}
vv2 = pile{{"lead white",2},{"cobalt violet",0.8},{"cobalt blue",0.6},{"viridian",0.15}, blot=0.1, name="vv2"}
vv3 = pile{{"lead white",3},{"rose madder",0.35},{"pale cadmium",0.35},{"cobalt violet",0.2}, blot=0.1, name="vv3"}
local L = function(y) return 445 - 0.25*y end   -- left edge-ish of gold, drifting
local R = function(y) return 715 + 0.3*y end
local glow = mask(function(x,y)
  local dl = x - L(y); local dr = R(y) - x
  local d = math.min(dl, dr)
  if d < -30 then return 0 end
  return 1 - smoothstep(10, 85, d)
end) * rect(300,0,560,300)
local edgeM = (glow - crownM:grow(2) - allIsl:grow(3)):times(function(x,y) return 1 end)
local tb = brush{kind="filbert", width=9, stiffness=0.6, lay=1.5}
local n = noise{seed=231, period=60, stretch={math.pi/2, 3}}
work(edgeM, {hand="body", tool=tb, piles={{vv1,1},{vv2,function(x,y) return 0.5+0.5*n(x,y) end},{vv3,0.6}}, mix_jitter=0.5, angle=function(x,y) return math.pi/2 + 0.1*n(x,y) end, length={25,60}, coverage=1.3, pressure={0.4,0.65}, load=0.5, threshold=0.35, clip=-(crownM:grow(1)) * -(allIsl:grow(2)), seed=2801})
blend(edgeM:grow(20):soften(10) - allIsl:grow(3), {angle=0, seed=2802})
blend(edgeM:grow(20):soften(10) - allIsl:grow(3), {angle=math.pi/2, seed=2803})
print("ok")

--@ chunk 124
local m = rect(300,0,560,330) - allIsl:grow(2)
for k=1,3 do
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=math.pi/2, passes=2, refold=0.15, seed=170+k})
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=0, passes=1, refold=0.15, seed=180+k})
end
print("ok")

--@ chunk 125
tr1 = pile{{"lead white",4},{"pale cadmium",0.5},{"cobalt violet",0.35},{"rose madder",0.1}, blot=0.1, name="tr1"}
tr2 = pile{{"lead white",3.5},{"cobalt violet",0.6},{"rose madder",0.15},{"pale cadmium",0.15}, blot=0.1, name="tr2"}
tr3 = pile{{"lead white",3},{"cobalt violet",0.7},{"cobalt blue",0.3}, blot=0.1, name="tr3"}
local t = function(y) return smoothstep(220, 360, y) end
local zone = (rect(330,215,520,160):roughen(14, 60, 51) - crownM:grow(3) - allIsl:grow(3))
local tb = brush{kind="filbert", width=10, stiffness=0.6, lay=1.5}
local n = noise{seed=241, period=80, stretch={0,4}}
work(zone, {hand="body", tool=tb, piles={{tr1,function(x,y) return (1-t(y))+0.05 end},{tr2,function(x,y) return math.exp(-((t(y)-0.5)/0.3)^2) end},{tr3,function(x,y) return t(y)+0.05 end}}, mix_jitter=0.5,
  angle=function(x,y) return 0.03*n(x,y) end, length={25,60}, coverage=1.2, pressure={0.4,0.65}, load=0.5, threshold=0.35, clip=-(crownM:grow(1)) * -(allIsl:grow(2)), seed=2901})
blend(zone:grow(10):soften(8) - allIsl:grow(3) - crownM:grow(2), {angle=math.pi/2, seed=2902})
blend(zone:grow(10):soften(8) - allIsl:grow(3) - crownM:grow(2), {angle=0, seed=2903})
print("ok")

--@ chunk 126
local m = rect(300,190,580,210) - allIsl:grow(2)
for k=1,3 do
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=0, passes=2, refold=0.15, seed=190+k})
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=math.pi/2, passes=1, refold=0.15, seed=200+k})
end
print("ok")

--@ chunk 127
wait(2*24*60) print(drying(560,280), drying(250,575), drying(720,455))

--@ chunk 128
wl = pile{{"lead white",1.5},{"cobalt violet",0.8},{"rose madder",0.1},{"cobalt blue",0.2}, thinner=0.6, name="wl"}
local tb = brush{kind="filbert", width=14, stiffness=0.5}
work(rect(470,200,80,45), {hand="body", tool=tb, pile=wl, angle=0, length={40,80}, coverage=1.0, pressure={0.4,0.6}, load=0.5, clip=true, seed=3001})
print("ok")

--@ chunk 129
print(palette())

--@ chunk 130
wl2 = pile{{"lead white",1.5},{"cobalt violet",0.6},{"rose madder",0.2},{"pale cadmium",0.1}, thinner=0.6, name="wl2"}
local n = noise{seed=251, period=90, stretch={0,4}}
local zone = (rect(320,150,540,170) - crownM:grow(2) - allIsl:grow(3)):times(function(x,y) return smoothstep(140,290,y) end)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
for pass=1,2 do
work(zone, {hand="body", tool=tb, piles={{wl,1},{wl2,function(x,y) return 0.6+0.5*n(x,y) end}}, angle=function(x,y) return 0.03*n(x,y) end, length={40,90}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.3*pass, clip=-(crownM:grow(1)) * -(allIsl:grow(2)), seed=3002+pass})
end
print("ok")

--@ chunk 131
sd1 = pile{{"ultramarine blue",1.6},{"cobalt violet",1},{"lead white",1.2},{"viridian",0.3},{"rose madder",0.1}, blot=0.15, name="sd1"}
sd2 = pile{{"viridian",1},{"cobalt blue",0.9},{"ultramarine blue",0.4},{"lead white",1.2}, blot=0.15, name="sd2"}
sd3 = pile{{"cobalt violet",1.3},{"ultramarine blue",0.9},{"rose madder",0.35},{"lead white",1.2}, blot=0.15, name="sd3"}
function colEdgeL(y) return 455 - 0.12*(y-290) + 10*math.sin(y/37) end
function colEdgeR(y) return 705 + 0.15*(y-290) + 10*math.sin(y/41+1) end
sidesM = mask(function(x,y)
  if y < 270 then return 0 end
  local d = math.max(colEdgeL(y) - x, x - colEdgeR(y))  -- positive outside column
  local fy = smoothstep(270, 320, y) * (1 - smoothstep(470, 560, y))
  return smoothstep(-20, 40, d) * fy
end) - allIsl:grow(3)
local tb = brush{kind="filbert", width=9, stiffness=0.65, lay=1.5, hair=0.6}
local n1 = noise{seed=261, period=70, stretch={math.pi/2,3}}
work(sidesM, {hand="body", tool=tb, piles={{sd1,1},{sd2,function(x,y) return clamp(0.3+0.7*n1(x,y),0,1) end},{sd3,function(x,y) return clamp(0.3-0.7*n1(x,y),0,1) end}}, mix_jitter=0.5,
  angle=function(x,y) return math.pi/2 + 0.06*math.sin(x/29) end, length={25,70}, coverage=1.1, pressure={0.4,0.7}, load=0.55, threshold=0.4, clip=-(allIsl:grow(2)), seed=3101})
print("ok")

--@ chunk 132
local bm = sidesM:grow(15):soften(10) - allIsl:grow(3)
blend(bm, {angle=math.pi/2, seed=3111})
blend(bm, {angle=math.pi/2+0.08, seed=3112})
print("ok")

--@ chunk 133
local m = sidesM:grow(25)
for k=1,3 do
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=math.pi/2, passes=2, refold=0.15, seed=210+k})
  r = rag{width=60}; r:dip(1.0)
  r:wipe(m, {pressure=0.95, angle=0, passes=1, refold=0.15, seed=220+k})
end
print("ok")

--@ chunk 134
wait(2*24*60) print(drying(250,575), drying(720,455), drying(300,400), drying(900,620))

--@ chunk 135
fp1 = pile{{"viridian",1},{"barium yellow",0.7},{"lead white",1},{"yellow ochre",0.2}, blot=0.15, name="fp1"}
fp2 = pile{{"viridian",1},{"cobalt blue",0.4},{"lead white",0.8},{"yellow ochre",0.3}, blot=0.15, name="fp2"}
fp3 = pile{{"viridian",0.8},{"yellow ochre",0.6},{"rose madder",0.25},{"lead white",0.7}, blot=0.15, name="fp3"}
fp4 = pile{{"lead white",2},{"barium yellow",0.8},{"viridian",0.35}, blot=0.15, name="fp4"}
fw = pile{{"lead white",1.6},{"cobalt blue",0.8},{"cobalt violet",0.5},{"ultramarine blue",0.3}, blot=0.15, name="fw"}
function flatPads(i, seedv, cov)
  local c = islC[i]; local sc = c[4]
  local tb = brush{kind="flat", width=5, stiffness=0.6, lay=1.5}
  local n = noise{seed=seedv, period=25}
  work(isl[i]:grow(3*sc), {hand="body", tool=tb, piles={{fp1,1},{fp2,1},{fp3,0.5},{fp4,function(x,y) return 0.3+0.4*n(x,y) end}}, mix_jitter=0.5,
    angle=function(x,y) return 0.04*n(x*2,y*2) end, length={14,32}, coverage=cov or 0.8, pressure={0.35,0.6}, load=0.5, curve={0.03,0.02}, scale_at=sc, edge={found=0.2,soft=0.5,lost=0.3,period=20}, seed=seedv})
  -- water slits / breaking rims
  work(isl[i]:rim(4*sc, 2*sc) + isl[i]:times(function(x,y) return clamp(n(x+50,y)-0.3,0,1) end), {hand="body", tool=brush{kind="flat", width=4, stiffness=0.6}, pile=fw, angle=0, length={10,25}, coverage=0.35, pressure={0.3,0.55}, load=0.4, curve={0.02,0.02}, scale_at=sc, threshold=0.4, seed=seedv+1})
end
flatPads(7, 3201)
print("ok")

--@ chunk 136
flatPads(1, 3211, 0.8)
flatPads(2, 3221, 0.8)
flatPads(5, 3251, 0.8)
flatPads(4, 3241, 0.75)
flatPads(6, 3261, 0.75)
flatPads(3, 3231, 0.7)
print("ok")

--@ chunk 137
wait(3*24*60)
fl1 = pile{{"lead white",3},{"rose madder",0.7},{"vermilion",0.12}, blot=0.2, name="fl1"}
fl2 = pile{{"lead white",4},{"rose madder",0.25},{"pale cadmium",0.1}, blot=0.2, name="fl2"}
fl3 = pile{{"lead white",3},{"carmine lake",0.4},{"rose madder",0.4}, blot=0.2, name="fl3"}
flc = pile{{"pale cadmium",1},{"lead white",1},{"deep cadmium",0.2}, blot=0.2, name="flc"}
function flower(x, y, s, p)
  local b = brush{kind="filbert", width=3.2*s, stiffness=0.6, lay=2.5}
  b:load(p, 0.7)
  b:stroke({{x-2.2*s, y+0.3*s}, {x+2.2*s, y+0.2*s}}, {pressure={0.6,0.4}, ramps={0.2,0.3}})
  b:load(fl2, 0.5)
  b:stroke({{x-1.2*s, y-0.9*s}, {x+1.0*s, y-1.0*s}}, {pressure={0.45,0.3}, ramps={0.2,0.3}})
end
flower(470, 466, 1.7, fl1)
flower(140, 560, 2.6, fl1)
print("ok")

--@ chunk 138
fl4 = pile{{"lead white",1.5},{"rose madder",1},{"carmine lake",0.3},{"vermilion",0.1}, blot=0.2, name="fl4"}
function flower2(x, y, s)
  local b = brush{kind="filbert", width=3.5*s, stiffness=0.6, lay=2.5}
  b:load(fl4, 0.8)
  b:stroke({{x-2.4*s, y+0.4*s}, {x+2.4*s, y+0.3*s}}, {pressure={0.7,0.5}, ramps={0.2,0.3}})
  b = brush{kind="filbert", width=2.6*s, stiffness=0.6, lay=2.5}
  b:load(fl1, 0.7)
  b:stroke({{x-1.6*s, y-0.6*s}, {x+1.4*s, y-0.7*s}}, {pressure={0.55,0.35}, ramps={0.2,0.3}})
  b = brush{kind="round", width=1.6*s, stiffness=0.6, lay=2}
  b:load(fl2, 0.7)
  b:touch(x-0.2*s, y-1.3*s, {pressure=0.5, drag={1.2*s,0}})
end
local F = {{140,562,3.2},{330,578,3.4},{232,544,2.8},{410,600,3.0},{470,465,2.0},{640,450,2.0},{790,446,1.9},{860,612,3.0},{955,630,3.2},{880,338,1.4},{230,360,1.5},{560,258,1.0}}
for _,f in ipairs(F) do flower2(f[1], f[2], f[3]) end
print("ok")

--@ chunk 139
wait(2*24*60)
tv = pile{{"cobalt violet",1},{"ultramarine blue",0.6},{"lead white",0.8},{"rose madder",0.15}, thinner=0.55, name="tv"}
tg = pile{{"viridian",0.6},{"cobalt blue",0.6},{"lead white",0.8}, thinner=0.55, name="tg"}
local m = mask(function(x,y)
  if y < 150 or y > 320 then return 0 end
  return 1 end) * (crownM:grow(60) - crownM:shrink(10)) * rect(250,150,220,170)
local tb = brush{kind="filbert", width=12, stiffness=0.5}
for pass=1,2 do
work(m, {hand="body", tool=tb, piles={{tv,1},{tg,0.5}}, angle=function(x,y) return math.pi/2 + 0.1*math.sin(x/20) end, length={40,90}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.4, clip=-(allIsl:grow(2)), seed=3300+pass})
end
print("ok")

--@ chunk 140
local m = (crownM:grow(70) - crownM:shrink(10)) * rect(660,120,230,200)
local tb = brush{kind="filbert", width=12, stiffness=0.5}
for pass=1,2 do
work(m, {hand="body", tool=tb, piles={{tv,1},{tg,0.5}}, angle=function(x,y) return math.pi/2 + 0.1*math.sin(x/20) end, length={40,90}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.4, clip=-(allIsl:grow(2)), seed=3310+pass})
end
-- dab the stray white flecks
local b = brush{kind="filbert", width=8, stiffness=0.5}
for _,p in ipairs({{292,250},{272,292},{262,298},{318,252}}) do b:load(tv, 0.5); b:stroke({{p[1],p[2]-10},{p[1],p[2]+10}}, {pressure=0.5}) end
print("ok")

--@ chunk 141
wait(24*60)
tw1 = pile{{"lead white",1.5},{"pale cadmium",0.6},{"deep cadmium",0.25},{"rose madder",0.1}, thinner=0.55, name="tw1"}
tw2 = pile{{"lead white",1.5},{"rose madder",0.35},{"cobalt violet",0.3},{"pale cadmium",0.1}, thinner=0.55, name="tw2"}
local gapIn = rect(380,0,380,300) - crownM:grow(2) - allIsl:grow(3)
local core = gapIn:times(function(x,y) return math.exp(-((x-580)/120)^2 - ((y-60)/120)^2) end)
local lower = gapIn:times(function(x,y) return smoothstep(130,260,y) * (0.6 + 0.4*math.sin(x/37)) end)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
work(core, {hand="body", tool=tb, pile=tw1, angle=0, length={40,100}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.3, clip=gapIn, seed=3401})
work(lower, {hand="body", tool=tb, pile=tw2, angle=0, length={40,100}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.35, clip=gapIn, seed=3402})
print("ok")

--@ chunk 142
tb2 = pile{{"ultramarine blue",0.8},{"cobalt violet",0.6},{"lead white",0.8},{"viridian",0.15}, thinner=0.55, name="tb2"}
local m = rect(0,235,340,175):roughen(15,50,61) - allIsl:grow(3)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
for pass=1,3 do
work(m, {hand="body", tool=tb, piles={{tb2,1},{tv,0.6},{tg,0.3}}, angle=function(x,y) return math.pi/2 + 0.08*math.sin(x/23) end, length={40,100}, coverage=1.0, pressure={0.45,0.65}, load=0.55, threshold=0.4, clip=-(allIsl:grow(2)), seed=3500+pass})
end
print("ok")

--@ chunk 143
frondT1 = pile{{"cobalt violet",1},{"ultramarine blue",0.7},{"viridian",0.3},{"lead white",0.6}, thinner=0.45, name="frondT1"}
frondT2 = pile{{"viridian",0.8},{"cobalt blue",0.6},{"lead white",0.7},{"barium yellow",0.1}, thinner=0.45, name="frondT2"}
local fb = brush{kind="filbert", width=9, stiffness=0.5}
local xs = {448, 462, 474, 455, 489, 708, 696, 686, 720}
for i,x in ipairs(xs) do
  fb:reload((i%2==0) and frondT2 or frondT1, 0.6)
  local y0 = rand(0, 40); local len = rand(90, 200)
  fb:stroke({{x, y0},{x+rand(-2,2), y0+len*0.5},{x+rand(-4,4), y0+len}}, {pressure={0.55,0.15}, ramps={0.05,0.6}})
  fb:reload((i%2==0) and frondT2 or frondT1, 0.6)
  fb:stroke({{x+3, y0},{x+3+rand(-2,2), y0+len*0.45},{x+rand(-4,4), y0+len*0.9}}, {pressure={0.5,0.1}, ramps={0.05,0.6}})
end
print("ok")

--@ chunk 144
blend(rect(430,0,75,260):soften(10) - allIsl:grow(3), {angle=math.pi/2, tool={kind="badger", width=20}, seed=3601})
blend(rect(675,0,60,260):soften(10) - allIsl:grow(3), {angle=math.pi/2, tool={kind="badger", width=20}, seed=3602})
print("ok")

--@ chunk 145
local m = rect(700,140,300,200):roughen(15,50,71) - allIsl:grow(3)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
for pass=1,3 do
work(m, {hand="body", tool=tb, piles={{tb2,1},{tv,0.7},{tg,0.3}}, angle=function(x,y) return math.pi/2 + 0.08*math.sin(x/23) end, length={40,100}, coverage=1.0, pressure={0.45,0.65}, load=0.55, threshold=0.4, clip=-(allIsl:grow(2)), seed=3700+pass})
end
print("ok")

--@ chunk 146
print(drying(540,262), drying(250,365), drying(880,340), drying(720,455), drying(880,338))
wait(2*24*60)
print(drying(880,338), drying(560,258))

--@ chunk 147
wait(4*24*60)
print(drying(880,338), drying(560,258), drying(230,360))
tvv = pile{{"cobalt violet",0.8},{"cobalt blue",0.5},{"lead white",1.2},{"rose madder",0.1}, thinner=0.6, name="tvv"}
local tb = brush{kind="filbert", width=10, stiffness=0.5}
for _,i in ipairs({3,6,4}) do
  local sc = islC[i][4]
  work(isl[i]:grow(5*sc), {hand="body", tool=tb, pile=tvv, angle=0, length={30,70}, coverage=1.0, pressure={0.4,0.6}, load=0.5, clip=isl[i]:grow(6*sc):soften(3), seed=3800+i})
end
print("ok")

--@ chunk 148
local b = brush{kind="filbert", width=9, stiffness=0.5}
for _,p in ipairs({{297,68},{318,250},{306,256},{310,292},{325,244}}) do
  b:reload(tb2, 0.6)
  b:stroke({{p[1],p[2]-14},{p[1]+1,p[2]+14}}, {pressure=0.6})
  b:reload(tb2, 0.6)
  b:stroke({{p[1]+4,p[2]-12},{p[1]+3,p[2]+12}}, {pressure=0.6})
end
print("ok")

--@ chunk 149
local b = brush{kind="filbert", width=5, stiffness=0.5}
b:reload(rv1, 0.5); b:stroke({{299,58},{299,80}}, {pressure=0.6}); b:reload(rv1,0.4); b:stroke({{301,60},{300,78}}, {pressure=0.5})
b:reload(m2, 0.5)
for _,p in ipairs({{321,255},{309,258},{315,262},{306,290},{312,293}}) do b:reload(m2,0.45); b:stroke({{p[1]-6,p[2]},{p[1]+6,p[2]+1}}, {pressure=0.55}) end
print("ok")

--@ chunk 150
r = rag{width=18}; r:dip(1.0)
r:wipe({{299,52},{299,86}}, {pressure=0.9}); r:refold(); r:wipe({{301,86},{298,52}}, {pressure=0.9}); r:refold()
r:dip(1.0)
r:wipe({{296,256},{330,258}}, {pressure=0.9}); r:refold(); r:wipe({{330,262},{296,260}}, {pressure=0.9}); r:refold()
r:wipe({{296,291},{322,293}}, {pressure=0.9}); r:refold(); r:wipe({{322,295},{296,292}}, {pressure=0.9})
print("ok")

--@ chunk 151
fix1 = mix{{rv1,0.6},{wlil,0.4}, name="fix1"}
local b = brush{kind="round", width=4, stiffness=0.5}
b:reload(fix1, 0.5); b:stroke({{299,62},{299,76}}, {pressure=0.5})
fix2 = mix{{m1,0.5},{lilac,0.5}, name="fix2"}
for _,p in ipairs({{321,255},{309,258},{306,290},{313,292}}) do b:reload(fix2,0.4); b:touch(p[1],p[2], {pressure=0.5, drag={4,0}}) end
print("ok")

--@ chunk 152
shd = pile{{"ultramarine blue",1.5},{"cobalt violet",0.8},{"viridian",0.3},{"lead white",0.4}, thinner=0.35, name="shd"}
-- find lower edge of each island at sampled x, stroke a shadow under it
function underShadow(i, seedv)
  local c = islC[i]; local cx, cy, th, sc = c[1], c[2], c[3], c[4]
  local len = ({430,300,200,260,220,160,120})[i]
  local b = brush{kind="filbert", width=4*sc, stiffness=0.5}
  local x = cx - len*0.55
  while x < cx + len*0.55 do
    local yb = nil
    for yy = cy + th, cy - th, -1 do if isl[i]:at(x, yy) > 0.5 then yb = yy; break end end
    if yb then
      local L = rand(14, 34)*sc
      b:reload(shd, 0.5)
      b:stroke({{x, yb + 1.5*sc + rand(-1,1)}, {x + L, yb + 1.5*sc + rand(-1.5,1.5)}}, {pressure={0.55,0.25}, ramps={0.15,0.4}})
      x = x + L*rand(0.7,1.4)
    else x = x + 8*sc end
  end
end
underShadow(7, 1)
print("ok")

--@ chunk 153
wr1 = pile{{"ultramarine blue",1.5},{"cobalt blue",0.8},{"cobalt violet",0.5},{"lead white",1.1}, blot=0.15, name="wr1"}
wr2 = pile{{"cobalt blue",1},{"lead white",1.4},{"cobalt violet",0.4},{"viridian",0.15}, blot=0.15, name="wr2"}
wr3 = pile{{"ultramarine blue",1.6},{"cobalt violet",0.9},{"viridian",0.3},{"lead white",0.5}, blot=0.15, name="wr3"}
local tb = brush{kind="flat", width=6, stiffness=0.6, lay=1.5}
work(rect(480,600,150,60), {hand="body", tool=tb, piles={{wr1,1},{wr2,0.6},{wr3,0.6}}, mix_jitter=0.5, angle=0, length={18,45}, coverage=0.6, pressure={0.35,0.6}, load=0.5, curve={0.03,0.02}, clip=true, seed=3901})
print("ok")

--@ chunk 154
r = rag{width=40}; r:dip(1.0)
r:wipe(rect(470,590,170,77), {pressure=0.95, angle=0, passes=3, refold=0.15, seed=230})
r = rag{width=40}; r:dip(1.0)
r:wipe(rect(470,590,170,77), {pressure=0.95, angle=0.1, passes=2, refold=0.15, seed=231})
print("ok")

--@ chunk 155
vd = pile{{"ultramarine blue",1.2},{"cobalt violet",0.6},{"lead white",0.5},{"viridian",0.15}, thinner=0.55, name="vd"}
vl = pile{{"lead white",1.5},{"cobalt blue",0.6},{"cobalt violet",0.3},{"rose madder",0.05}, thinner=0.55, name="vl"}
local n = noise{seed=271, period=120, stretch={0,5}}
local lowM = (below(function(x) return 380 + 10*math.sin(x/80) end) - allIsl:grow(3))
local dM = lowM:times(function(x,y) return clamp(0.3 + 0.9*n(x,y), 0, 1) * (0.5 + 0.5*smoothstep(400,620,y)) end)
local lM = lowM:times(function(x,y) return clamp(0.2 - 0.9*n(x,y), 0, 1) end)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
work(dM, {hand="body", tool=tb, pile=vd, angle=function(x,y) return 0.03*n(x*2,y) end, length={50,130}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.35, clip=-(allIsl:grow(2)), seed=4001})
work(lM, {hand="body", tool=tb, pile=vl, angle=function(x,y) return 0.03*n(x*2,y) end, length={50,130}, coverage=0.8, pressure={0.4,0.6}, load=0.5, threshold=0.25, clip=-(allIsl:grow(2)), seed=4002})
print("ok")

--@ chunk 156
wait(4*24*60)
for _,i in ipairs({1,2,3,4,5,6}) do underShadow(i, i) end
print("ok")

--@ chunk 157
dv1 = pile{{"ultramarine blue",1.5},{"viridian",0.7},{"cobalt violet",0.6},{"rose madder",0.25},{"lead white",0.15}, thinner=0.5, name="dv1"}
dv2 = pile{{"ultramarine blue",1.2},{"cobalt violet",0.9},{"rose madder",0.4},{"lead white",0.15}, thinner=0.5, name="dv2"}
local n = noise{seed=281, period=70, stretch={math.pi/2, 4}}
local core = crownM:shrink(25):times(function(x,y) return clamp(0.35 + 0.9*n(x,y), 0, 1) * (1 - smoothstep(150, 300, y)) end)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
work(core, {hand="body", tool=tb, piles={{dv1,1},{dv2,function(x,y) return 0.5+0.5*n(x+70,y) end}}, angle=function(x,y) return math.pi/2 + 0.06*math.sin(x/27) end, length={50,130}, coverage=1.0, pressure={0.45,0.65}, load=0.55, threshold=0.4, clip=crownM:shrink(8), seed=4101})
print("ok")

--@ chunk 158
wait(3*24*60)
gzw = pile{{"ultramarine blue",1},{"viridian",0.6},{"rose madder",0.35},{"cobalt violet",0.3}, medium=0.6, name="gzw"}
local tb = brush{kind="filbert", width=16, stiffness=0.4}
local m = ellipse(120, 90, 70, 60):soften(15)
work(m, {hand="body", tool=tb, pile=gzw, angle=math.pi/2, length={50,110}, coverage=1.0, pressure={0.35,0.55}, load=0.35, threshold=0.3, seed=4201})
print("ok")

--@ chunk 159
blend(ellipse(120, 90, 95, 85):soften(15), {angle=math.pi/2, seed=4202})
blend(ellipse(120, 90, 95, 85):soften(15), {angle=0, seed=4203})
print("ok")

--@ chunk 160
local n = noise{seed=291, period=90, stretch={math.pi/2, 3}}
local core = crownM:shrink(25):times(function(x,y) return clamp(0.45 + 0.8*n(x,y), 0, 1) * (1 - smoothstep(120, 280, y)) end) - ellipse(120,90,95,85)
local tb = brush{kind="filbert", width=16, stiffness=0.4}
work(core, {hand="body", tool=tb, pile=gzw, angle=function(x,y) return math.pi/2 + 0.05*math.sin(x/31) end, length={50,120}, coverage=0.9, pressure={0.35,0.55}, load=0.35, threshold=0.4, clip=crownM:shrink(10), seed=4211})
local bm = crownM:shrink(12):times(function(x,y) return 1 - smoothstep(200, 320, y) end):soften(10)
blend(bm, {angle=math.pi/2, seed=4212})
blend(bm, {angle=0, seed=4213})
print("ok")

--@ chunk 161
wait(4*24*60)
print(drying(120,90), drying(560,100))
tw3 = pile{{"lead white",1},{"pale cadmium",0.6},{"deep cadmium",0.35},{"rose madder",0.12}, thinner=0.55, name="tw3"}
tw4 = pile{{"lead white",1.2},{"rose madder",0.4},{"cobalt violet",0.2},{"deep cadmium",0.1}, thinner=0.55, name="tw4"}
local gapIn = rect(420,0,320,280) - crownM:grow(2) - allIsl:grow(3)
local core = gapIn:times(function(x,y) return math.exp(-((x-585)/80)^2 - ((y-40)/90)^2) end)
local sides = gapIn:times(function(x,y) local c = math.exp(-((x-585)/90)^2); return (1-c) * (0.5 + 0.5*smoothstep(0,200,y)) end)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
work(core, {hand="body", tool=tb, pile=tw3, angle=0, length={40,100}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.3, clip=gapIn, seed=4301})
work(sides, {hand="body", tool=tb, pile=tw4, angle=math.pi/2, length={40,100}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.35, clip=gapIn, seed=4302})
print("ok")

--@ chunk 162
local fb = brush{kind="filbert", width=9, stiffness=0.5}
for i,x in ipairs({716, 726, 735, 744, 754}) do
  fb:reload((i%2==0) and frondT2 or frondT1, 0.55)
  local y0 = rand(0, 15); local len = rand(70, 140)
  fb:stroke({{x, y0},{x+rand(-2,2), y0+len*0.5},{x+rand(-3,3), y0+len}}, {pressure={0.55,0.15}, ramps={0.05,0.6}})
end
blend(rect(705,0,60,150):soften(10), {angle=math.pi/2, tool={kind="badger", width=18}, seed=4401})
-- stray light specks at left gap foot
local b = brush{kind="filbert", width=6, stiffness=0.5}
for _,p in ipairs({{318,253},{307,257},{306,292},{313,294},{310,248}}) do b:reload(tb2, 0.5); b:stroke({{p[1],p[2]-8},{p[1],p[2]+8}}, {pressure=0.5}) end
print("ok")

--@ chunk 163
frondPatch(712, 760, 0, 30, 60, 140, 7, {fv1b,fv2b,fv3b}, 0.45, rect(705,0,65,190):soften(10))
print("ok")

--@ chunk 164
wait(2*24*60)
wr = pile{{"rose madder",0.6},{"cobalt violet",0.8},{"lead white",0.8},{"ultramarine blue",0.2}, thinner=0.55, name="wr"}
wgv = pile{{"viridian",0.8},{"barium yellow",0.25},{"lead white",0.8},{"cobalt blue",0.2}, thinner=0.55, name="wgv"}
local n = noise{seed=301, period=60, stretch={math.pi/2, 5}}
local m1 = crownM:shrink(10):times(function(x,y) return clamp(n(x,y)*1.5 - 0.2, 0, 1) end)
local m2 = crownM:shrink(10):times(function(x,y) return clamp(-n(x,y)*1.5 - 0.35, 0, 1) end)
local tb = brush{kind="filbert", width=10, stiffness=0.5}
work(m1, {hand="body", tool=tb, pile=wr, angle=function(x,y) return math.pi/2 + 0.05*math.sin(x/19) end, length={40,110}, coverage=0.8, pressure={0.4,0.6}, load=0.5, threshold=0.4, clip=crownM, seed=4501})
work(m2, {hand="body", tool=tb, pile=wgv, angle=function(x,y) return math.pi/2 + 0.05*math.sin(x/19) end, length={40,110}, coverage=0.8, pressure={0.4,0.6}, load=0.5, threshold=0.4, clip=crownM, seed=4502})
print("ok")

--@ chunk 165
local b = brush{kind="filbert", width=12, stiffness=0.5}
-- pale wedge left of island 2
for k=1,5 do b:reload(vd, 0.5); local y = 418 + k*7; b:stroke({{515, y},{560, y+rand(-1,1)},{588, y}}, {pressure=0.55, clip=-(allIsl:grow(2))}) end
-- dark blot near 490,425
for k=1,3 do b:reload(vl, 0.5); local y = 418 + k*5; b:stroke({{460, y},{520, y}}, {pressure=0.5, clip=-(allIsl:grow(2))}) end
-- white blob under island 2 right
for k=1,3 do b:reload(vd, 0.5); local y = 470 + k*4; b:stroke({{830, y},{870, y}}, {pressure=0.55, clip=-(allIsl:grow(2))}) end
print("ok")

--@ chunk 166
local b = brush{kind="filbert", width=10, stiffness=0.5}
for k=1,4 do b:reload(vl, 0.5); local y = 505 + k*9; b:stroke({{735, y},{790, y+rand(-1,1)}}, {pressure=0.55, clip=-(allIsl:grow(2))}) end
for k=1,2 do b:reload(vl, 0.5); local y = 530 + k*5; b:stroke({{720, y},{800, y}}, {pressure=0.5, clip=-(allIsl:grow(2))}) end
for k=1,2 do b:reload(vl, 0.5); local y = 418 + k*6; b:stroke({{462, y},{515, y}}, {pressure=0.6, clip=-(allIsl:grow(2))}) end
print("ok")

--@ chunk 167
pw = pile{{"lead white",4},{"rose madder",0.1},{"pale cadmium",0.05}, blot=0.2, name="pw"}
pd = pile{{"carmine lake",0.6},{"rose madder",0.6},{"lead white",0.4}, blot=0.15, name="pd"}
function petals(x, y, s)
  local b = brush{kind="round", width=1.3*s, stiffness=0.6, lay=2}
  b:load(pw, 0.6)
  b:stroke({{x-2.2*s, y-0.9*s},{x-0.8*s, y-1.6*s}}, {pressure={0.5,0.2}})
  b:load(pw, 0.6)
  b:stroke({{x+0.6*s, y-1.7*s},{x+2.0*s, y-0.8*s}}, {pressure={0.5,0.2}})
  b:load(pd, 0.6)
  b:stroke({{x-1.6*s, y+0.9*s},{x+1.6*s, y+0.9*s}}, {pressure={0.5,0.3}})
end
petals(140, 562, 3.2)
print("ok")

--@ chunk 168
local F = {{330,578,3.4},{232,544,2.8},{410,600,3.0},{470,465,2.0},{640,450,2.0},{790,446,1.9},{860,612,3.0},{955,630,3.2}}
for _,f in ipairs(F) do petals(f[1], f[2], f[3]) end
print("ok")

--@ chunk 169
local tb = brush{kind="filbert", width=14, stiffness=0.5}
local edges = (rect(470,0,60,230) + rect(640,0,60,230)) - crownM:grow(2)
for pass=1,2 do
work(edges:roughen(10,40,80+pass), {hand="body", tool=tb, piles={{tw4,1},{wl2,0.5}}, angle=function(x,y) return math.pi/2 + 0.08*math.sin(y/30) end, length={40,100}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.4, clip=-(allIsl:grow(2)) * -(crownM:grow(1)), seed=4600+pass})
end
-- a few horizontal lilac veils inside the gold, broken
local inner = rect(520,30,130,200):times(function(x,y) return (math.sin(y/23 + x/90) > 0.6) and 1 or 0 end)
work(inner, {hand="body", tool=tb, pile=wl2, angle=0, length={40,90}, coverage=0.6, pressure={0.35,0.55}, load=0.45, threshold=0.5, seed=4610})
print("ok")

--@ chunk 170
wait(3*24*60)
print(drying(250,365), drying(880,340), drying(540,262))
tvv2 = pile{{"cobalt violet",0.8},{"cobalt blue",0.5},{"lead white",1.4},{"rose madder",0.15}, thinner=0.6, name="tvv2"}
local tb = brush{kind="filbert", width=12, stiffness=0.5}
for _,i in ipairs({4,6,3}) do
  local sc = islC[i][4]
  for pass=1,2 do
    work(isl[i]:grow(4*sc), {hand="body", tool=tb, pile=tvv2, angle=0, length={30,70}, coverage=1.0, pressure={0.4,0.6}, load=0.5, clip=isl[i]:grow(5*sc):soften(3), seed=4700+i*10+pass})
  end
end
print("ok")

--@ chunk 171
sig = pile{{"ultramarine blue",1},{"rose madder",0.6},{"cobalt violet",0.4},{"lead white",0.2}, medium=0.15, name="sig"}
local rg = brush{kind="rigger", width=1.6, point=0.9}
local ox, oy, h = 28, 652, 12
local function g(pts) rg:load(sig, 0.7); local q = {}; for i,p in ipairs(pts) do q[i] = {ox + p[1]*h, oy - p[2]*h, p[3] or 0.6} end; rg:gesture(q, {wobble=0.2}) end
-- C
g({{0.7,0.9,0.3},{0.35,1.0},{0.05,0.6},{0.1,0.15},{0.45,0.0},{0.75,0.15,0.2}})
-- l
g({{0.95,0.1,0.4},{1.15,0.8},{1.25,1.15},{1.15,1.1},{1.05,0.5},{1.1,0.05},{1.3,0.05,0.3}})
-- a
g({{1.85,0.45,0.4},{1.55,0.5},{1.42,0.2},{1.6,0.02},{1.85,0.3},{1.9,0.5},{1.88,0.15},{2.0,0.02,0.3}})
-- u
g({{2.15,0.5,0.4},{2.15,0.12},{2.35,0.02},{2.55,0.3},{2.58,0.5},{2.58,0.12},{2.7,0.02,0.3}})
-- d
g({{3.15,0.45,0.4},{2.88,0.5},{2.78,0.2},{2.95,0.02},{3.15,0.3},{3.2,1.15},{3.18,0.4},{3.25,0.02},{3.4,0.05,0.3}})
-- e
g({{3.55,0.22,0.4},{3.85,0.35},{3.75,0.5},{3.55,0.4},{3.5,0.15},{3.7,0.0},{3.95,0.1,0.25}})
print("ok")

--@ chunk 172
print(crownM ~= nil, allIsl ~= nil, isl ~= nil, #isl)
print(drying(560,300), drying(250,575), drying(560,258), drying(30,650))

--@ chunk 173
wait(6*24*60)
print(drying(560,258), drying(30,650), drying(140,562), drying(880,338))

--@ chunk 174
tg1 = pile{{"lead white",1.5},{"pale cadmium",0.5},{"rose madder",0.2},{"cobalt violet",0.1}, thinner=0.55, name="tg1"}
local tb = brush{kind="filbert", width=14, stiffness=0.5}
for i=1,6 do
  local x = 560 + i*11 + rand(-3,3)
  local y0 = rand(268,280); local len = rand(70,110)
  tb:reload((i%2==0) and tg1 or tw2, 0.5)
  tb:stroke({{x,y0},{x+rand(-2,2),y0+len*0.5},{x+rand(-3,3),y0+len}}, {pressure={0.6,0.15}, ramps={0.05,0.6}, clip=-(allIsl:grow(2))})
end
print("ok")

--@ chunk 175
local n = noise{seed=311, period=50, stretch={math.pi/2, 3}}
local band = (rect(300,262,420,100):roughen(12,40,91) - allIsl:grow(3) - crownM:grow(2)):times(function(x,y) return math.exp(-((y-292)/30)^2) end)
local tb = brush{kind="filbert", width=14, stiffness=0.5}
for pass=1,3 do
work(band, {hand="body", tool=tb, piles={{tg1,function(x,y) return clamp(0.5+0.6*n(x,y),0,1) end},{tw2,0.6},{wl,function(x,y) return clamp(0.4-0.6*n(x,y),0,1) end}}, angle=function(x,y) return math.pi/2 + 0.06*math.sin(x/21) end, length={40,90}, coverage=1.0, pressure={0.4,0.6}, load=0.5, threshold=0.3, clip=-(allIsl:grow(2)) * -(crownM:grow(1)), seed=5000+pass})
end
print("ok")

--@ chunk 176
pk1 = pile{{"lead white",3},{"pale cadmium",0.5},{"rose madder",0.2},{"cobalt violet",0.12}, medium=0.1, name="pk1"}
pk2 = pile{{"lead white",3},{"rose madder",0.3},{"cobalt violet",0.3},{"pale cadmium",0.2}, medium=0.1, name="pk2"}
local fb = brush{kind="filbert", width=7, stiffness=0.5}
for i,x in ipairs({528, 541, 556, 572, 590}) do
  fb:reload((i%2==0) and pk2 or pk1, 0.45)
  local y0 = rand(272,282); local len = rand(50,100)
  fb:stroke({{x,y0},{x+rand(-2,2),y0+len*0.5},{x+rand(-3,3),y0+len}}, {pressure={0.55,0.15}, ramps={0.05,0.6}, clip=-(allIsl:grow(2))})
end
print("ok")

--@ chunk 177
blend(rect(510,265,100,125):soften(10) - allIsl:grow(3), {angle=math.pi/2, tool={kind="badger", width=20}, seed=5101})
blend(rect(510,265,100,125):soften(10) - allIsl:grow(3), {angle=math.pi/2+0.1, tool={kind="badger", width=20}, seed=5102})
print("ok")

--@ chunk 178
frondPatch(468, 505, 0, 50, 40, 150, 4, {fv1b,fv2b,fv3b}, 0.4, rect(458,0,60,220):soften(10))
frondPatch(652, 690, 0, 40, 50, 170, 4, {fv2b,fv1b,fv3b}, 0.4, rect(642,0,58,230):soften(10))
print("ok")

--@ chunk 179
fv_far = pile{{"lead white",1.6},{"cobalt violet",0.45},{"rose madder",0.15},{"pale cadmium",0.2},{"viridian",0.08}, thinner=0.55, name="fv_far"}
local tb = brush{kind="filbert", width=12, stiffness=0.5}
local i=3; local sc = islC[i][4]
for pass=1,2 do
  work(isl[i]:grow(4*sc), {hand="body", tool=tb, pile=fv_far, angle=0, length={30,70}, coverage=1.0, pressure={0.4,0.6}, load=0.5, clip=isl[i]:grow(5*sc):soften(3), seed=5200+pass})
end
print("ok")

--@ chunk 180
pw2 = pile{{"lead white",4},{"pale cadmium",0.08},{"cobalt violet",0.05}, blot=0.2, name="pw2"}
pws = pile{{"lead white",2},{"cobalt violet",0.5},{"cobalt blue",0.2}, blot=0.2, name="pws"}
function whiteLily(x, y, s)
  local b = brush{kind="flat", width=1.6*s, stiffness=0.65, lay=2.5}
  b:load(pws, 0.6)   -- shadowed underside
  b:stroke({{x-2.4*s, y+0.6*s},{x+2.2*s, y+0.7*s}}, {pressure={0.55,0.35}})
  b:load(pw2, 0.7)
  b:stroke({{x-2.2*s, y-0.2*s},{x-0.4*s, y-1.5*s}}, {pressure={0.6,0.25}})
  b:load(pw2, 0.7)
  b:stroke({{x+0.3*s, y-1.6*s},{x+2.1*s, y-0.1*s}}, {pressure={0.6,0.25}})
  b:load(pw2, 0.6)
  b:stroke({{x-0.3*s, y-0.4*s},{x+0.2*s, y-2.0*s}}, {pressure={0.5,0.2}})
  local c = brush{kind="round", width=0.9*s, stiffness=0.6, lay=2}
  c:load(flc, 0.5); c:touch(x, y-0.4*s, {pressure=0.45, drag={0.6*s,0}})
end
whiteLily(820, 628, 3.0)
print("ok")

--@ chunk 181
print(isl[1]:at(195,588), isl[2]:at(705,458), isl[1]:at(290,560))
whiteLily(195, 588, 3.2)
whiteLily(705, 458, 2.0)
print("ok")

--@ chunk 182
print(drying(260,575), drying(720,455), drying(560,100))
ew1 = pile{{"lead white",1.5},{"yellow ochre",0.5},{"rose madder",0.25},{"pale cadmium",0.3},{"viridian",0.15}, blot=0.15, name="ew1"}
ew2 = pile{{"lead white",1.3},{"rose madder",0.35},{"cobalt violet",0.2},{"yellow ochre",0.25}, blot=0.15, name="ew2"}
local sc = 1.5
local b = brush{kind="flat", width=4*sc, stiffness=0.6, lay=1.5}
local spots = {{170,535},{215,528},{300,540},{360,530},{120,548},{395,560}}
for i,p in ipairs(spots) do
  if isl[1]:at(p[1],p[2]) > 0.5 then
    b:reload((i%2==0) and ew2 or ew1, 0.45)
    local L = rand(14,26)*sc*0.7
    b:stroke({{p[1],p[2]},{p[1]+L,p[2]+rand(-1,1)}}, {pressure={0.5,0.35}, ramps={0.15,0.3}})
  end
end
print("ok")

--@ chunk 183
r = rag{width=14}; r:dip(1.0)
r:wipe({{355,530},{395,530}}, {pressure=0.9}); r:refold(); r:wipe({{395,532},{355,532}}, {pressure=0.9}); r:refold()
r:dip(1.0)
r:wipe({{390,560},{430,560}}, {pressure=0.9}); r:refold(); r:wipe({{430,562},{390,562}}, {pressure=0.9})
print("ok")

--@ chunk 184
ew3 = pile{{"lead white",1.4},{"yellow ochre",0.45},{"rose madder",0.15},{"cobalt violet",0.12},{"viridian",0.2}, blot=0.15, name="ew3"}
function warmTouches(i, n, seedTry)
  local c = islC[i]; local cx, cy, th, sc = c[1], c[2], c[3], c[4]
  local len = ({430,300,200,260,220,160,120})[i]
  local b = brush{kind="flat", width=3.5*sc, stiffness=0.6, lay=1.5}
  local cnt, tries = 0, 0
  while cnt < n and tries < 200 do
    tries = tries + 1
    local x = cx + rand(-0.5,0.45)*len; local y = cy + rand(-th*0.45, th*0.1)
    if isl[i]:at(x,y) > 0.7 and isl[i]:at(x+12*sc,y) > 0.7 then
      cnt = cnt + 1
      b:reload((cnt%3==0) and ew1 or ew3, 0.4)
      local L = rand(9,16)*sc
      b:stroke({{x,y},{x+L,y+rand(-0.8,0.8)}}, {pressure={0.45,0.3}, ramps={0.2,0.3}})
    end
  end
  return cnt
end
print(warmTouches(1, 5), warmTouches(5, 5), warmTouches(2, 4), warmTouches(7, 2))
print("ok")

--@ chunk 185
wait(5*24*60) print(drying(300,540), drying(860,600))

--@ chunk 186
print(drying(445,280), drying(540,262))

--@ chunk 187
local b = brush{kind="filbert", width=5, stiffness=0.5}
for _,p in ipairs({{444,277},{450,279},{446,288},{452,290}}) do b:reload(fix2,0.4); b:stroke({{p[1]-5,p[2]},{p[1]+5,p[2]+0.5}}, {pressure=0.5, clip=-(allIsl:grow(1))}) end
print("ok")

--@ chunk 188
local b = brush{kind="filbert", width=4, stiffness=0.5}
for _,p in ipairs({{443,282},{454,288},{440,279}}) do b:reload(fix2,0.4); b:stroke({{p[1]-4,p[2]},{p[1]+4,p[2]}}, {pressure=0.5, clip=-(allIsl:grow(1))}) end
print("ok")

--@ chunk 189
wait(3*24*60) print(drying(445,282))

--@ chunk 190
print(drying(920,290), drying(880,340))
frondPatch(905, 955, 235, 260, 45, 75, 4, {fv1b,fv2b,fv1b}, 0.4, rect(895,230,70,95):soften(10) - allIsl:grow(3))
print("ok")

--@ chunk 191
frondPatch(842, 872, 238, 262, 35, 60, 3, {fv3b,fv1b,fv2b}, 0.38, rect(832,232,52,85):soften(10) - allIsl:grow(3))
print("ok")

--@ chunk 192
wait(4*24*60) print(drying(915,290), drying(860,270))

--@ chunk (finishing)
-- after the session: let the paint dry right through, a month at a time until touch-dry
-- (at most ten years), as scripts/finish_painting does; no varnish, no cracks
local function all_dry()
  for i = 0, 39 do for j = 0, 29 do
    if drying(12.5 + i * 25, (j + 0.5) * H / 30) ~= "dry" then return false end
  end end
  return true
end
local months = 0
while not all_dry() and months < 120 do wait(30 * 24 * 60); months = months + 1 end
print("dried for " .. months .. " months")
