-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box hopper
--@ engine 5

--@ chunk 1
canvas{size=900, aspect=1.5, linen={18,16}, seed=4127,
  ground={{pile={{"lead white",10},{"yellow ochre",0.25}}, um=120, apply="knife", texture=0.25}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
pen = pencil("2B")
local R = function(a,b,c,d,p) pen:rule({a,b},{c,d},{pressure=p or 0.3}) end
-- main facade
R(0,70,690,70) R(0,100,690,100) R(0,108,690,108)
R(690,62,690,575)
-- side wall
R(690,70,760,198) R(760,198,760,535) R(690,575,760,535)
-- windows rows
for _,cx in ipairs({100,270,440,610}) do
  for _,yy in ipairs({{140,232},{262,350}}) do
    local x0, x1 = cx-38, cx+38
    R(x0,yy[1],x1,yy[1]) R(x0,yy[2],x1,yy[2]) R(x0,yy[1],x0,yy[2]) R(x1,yy[1],x1,yy[2])
  end
end
-- sign band and storefront
R(0,368,690,368) R(0,402,690,402)
R(30,410,420,410) R(30,410,30,548) R(420,410,420,548) R(30,548,420,548)
R(445,410,535,410) R(445,410,445,575) R(535,410,535,575)
R(555,410,672,410) R(555,410,555,548) R(672,410,672,548) R(555,548,672,548)
R(0,575,690,575)
-- curb
R(0,640,1000,640) R(0,648,1000,648)
-- distant block
R(770,300,1000,300) R(770,300,770,525) R(760,525,1000,525)
-- big shadow diagonal
R(0,330,470,575,0.2)
R(470,575,620,640,0.2)

--@ chunk 3
-- geometry
G = {}
G.facade = rect(0,70,690,505)          -- 70..575
G.side = poly({{690,70},{745,175},{745,545},{690,575}})
G.distant = rect(745,300,255,225)      -- 300..525
G.walk = rect(0,575,1000,65) - poly({{690,575},{745,545},{1000,545},{1000,575}})
G.cross = poly({{690,575},{745,545},{1000,525},{1000,575}})  -- placeholder side street floor
G.street = rect(0,640,1000,27)
G.sky = -(G.facade + G.side + G.distant + G.walk + G.street + G.cross)
-- big shadow on facade: below the line from (0,330) to (470,575)
G.fshadow = poly({{0,330},{470,575},{0,575}}) * G.facade
G.wshadow = poly({{0,575},{470,575},{620,640},{0,640}})
print(G.sky:area(), G.facade:area())

--@ chunk 4
G.walk = rect(0,575,1000,65)
G.cross = poly({{690,575},{745,545},{745,525},{1000,525},{1000,575}})
G.sky = -(G.facade + G.side + G.distant + G.walk + G.street + G.cross)
L = {}
L.sky = pile{{"lead white",3},{"cerulean blue",1},{"cobalt blue",0.3}, turps=0.6, name="L sky"}
L.brick = pile{{"red earth",2},{"burnt sienna",1},{"yellow ochre",0.6},{"lead white",1}, turps=0.65, name="L brick"}
L.shad = pile{{"red earth",1},{"ultramarine blue",1.2},{"burnt sienna",0.6},{"lead white",0.5}, turps=0.65, name="L shadow"}
L.dark = pile{{"viridian",1},{"bone black",1},{"burnt sienna",0.6}, turps=0.65, name="L dark"}
L.walk = pile{{"lead white",2},{"yellow ochre",0.5},{"red earth",0.15},{"bone black",0.08}, turps=0.6, name="L walk"}
L.dist = pile{{"lead white",2},{"yellow ochre",0.8},{"red earth",0.3}, turps=0.6, name="L distant"}

--@ chunk 5
L.shad:add{{"burnt sienna",0.6},{"red earth",0.3}}
work(G.sky, {hand="broad", pile=L.sky, angle=0.05, coverage=1.2})
work(G.facade - G.fshadow, {hand="broad", pile=L.brick, angle=0, coverage=1.2})
work(G.fshadow, {hand="body", pile=L.shad, angle=-0.5, coverage=1.2})
work(G.side, {hand="body", pile=L.shad, angle=1.3, coverage=1.2})
work(G.distant, {hand="body", pile=L.dist, angle=0, coverage=1.2})
work(G.walk - G.wshadow, {hand="body", pile=L.walk, angle=0, coverage=1.2})
work(G.wshadow, {hand="body", pile=L.shad, angle=0, coverage=1})
work(G.street + G.cross, {hand="body", pile=L.shad, angle=0, coverage=1})

--@ chunk 6
local o = {fill=true, clip=true}
work(G.sky, {hand="broad", pile=L.sky, angle=0.05, coverage=2, fill=true, clip=true})
work(G.facade - G.fshadow, {hand="broad", pile=L.brick, angle=0, coverage=2, fill=true, clip=true})
work(G.fshadow, {hand="body", pile=L.shad, angle=0, coverage=2, fill=true, clip=true})
work(G.side, {hand="body", pile=L.shad, angle=1.4, coverage=2, fill=true, clip=true})
work(G.distant, {hand="body", pile=L.dist, angle=0, coverage=2, fill=true, clip=true})
work(G.walk - G.wshadow, {hand="body", pile=L.walk, angle=0, coverage=2, fill=true, clip=true})
work(G.wshadow, {hand="body", pile=L.shad, angle=0, coverage=2, fill=true, clip=true})
work(G.street + G.cross, {hand="body", pile=L.shad, angle=0, coverage=2, fill=true, clip=true})

--@ chunk 7
print(wait(20*60))
print(drying(300,200), drying(100,500), drying(800,100))

--@ chunk 8
print(wait(24*60))
print(drying(300,200), drying(100,500), drying(800,100), drying(300,620), drying(900,400))

--@ chunk 9
print(wait(36*60))
print(drying(300,200), drying(100,500), drying(800,100), drying(300,620), drying(900,400), drying(720,300))

--@ chunk 10
P = {}
P.skyhi = pile{{"lead white",3},{"cerulean blue",1},{"cobalt blue",0.35}, medium=0.1, name="sky high"}
P.skylo = pile{{"lead white",6},{"cerulean blue",0.6},{"pale cadmium",0.08}, medium=0.1, name="sky low"}
P.brick = pile{{"red earth",2},{"burnt sienna",0.4},{"yellow ochre",0.5},{"lead white",1.2},{"bone black",0.05}, medium=0.1, name="brick lit"}
P.bshad = pile{{"red earth",1.5},{"burnt sienna",0.8},{"ultramarine blue",0.5},{"bone black",0.15},{"lead white",0.5}, medium=0.1, name="brick shadow"}

--@ chunk 11
P.skyhi:add{{"cerulean blue",0.4},{"cobalt blue",0.15}}
work(G.sky, {hand="broad", piles={{P.skyhi, function(x,y) return 1-smoothstep(0,520,y) end},{P.skylo, function(x,y) return smoothstep(0,520,y) end}},
  angle=0.02, coverage=2.5, fill=true, clip=true})

--@ chunk 12
P.brick2 = pile{{"red earth",2},{"burnt sienna",0.3},{"yellow ochre",0.8},{"lead white",1.8}, medium=0.1, name="brick lit warm"}
G.upper = rect(0,108,690,260)
G.storey = rect(0,402,690,173)
local m = G.upper - G.fshadow
work(m, {hand="body", piles={{P.brick, function(x,y) return 0.4+smoothstep(100,370,y) end},{P.brick2, function(x,y) return 1-smoothstep(100,370,y) end}},
  angle=0, angle_jitter=0.05, coverage=2.5, fill=true, clip=true, length={30,70}})
work(G.upper * G.fshadow, {hand="body", pile=P.bshad, angle=0, coverage=2.5, fill=true, clip=true, length={30,70}})

--@ chunk 13
P.stone = pile{{"lead white",4},{"yellow ochre",0.5},{"red earth",0.1}, medium=0.1, name="stone lit"}
P.green = pile{{"viridian",1},{"bone black",0.7},{"yellow ochre",0.4},{"lead white",0.3}, medium=0.1, name="sign green"}
P.side = pile{{"red earth",1.2},{"burnt sienna",0.6},{"ultramarine blue",0.5},{"lead white",0.8},{"bone black",0.08}, medium=0.1, name="side wall"}
P.walk = pile{{"lead white",4},{"yellow ochre",0.5},{"red earth",0.15},{"bone black",0.1}, medium=0.1, name="walk lit"}
P.wshad = pile{{"lead white",1.5},{"ultramarine blue",0.5},{"red earth",0.4},{"bone black",0.3}, medium=0.1, name="walk shadow"}
P.street = pile{{"lead white",1},{"ultramarine blue",0.3},{"burnt sienna",0.4},{"bone black",0.4}, medium=0.1, name="street"}
P.dist = pile{{"lead white",4},{"yellow ochre",0.7},{"red earth",0.25},{"cerulean blue",0.08}, medium=0.1, name="distant lit"}

--@ chunk 14
P.green:add{{"bone black",0.4},{"viridian",0.3}}
P.wshad:add{{"yellow ochre",0.15},{"bone black",0.1}}
P.street:add{{"bone black",0.2},{"burnt sienna",0.15}}
work(rect(0,70,690,30), {hand="body", pile=P.stone, angle=0, coverage=2.5, fill=true, clip=true, length={30,70}})
work(G.side, {hand="body", pile=P.side, angle=1.45, coverage=2.5, fill=true, clip=true, length={20,50}})
work(rect(0,368,690,34), {hand="body", pile=P.green, angle=0, coverage=2.5, fill=true, clip=true, length={30,70}})
work(G.distant, {hand="body", pile=P.dist, angle=0, coverage=2.5, fill=true, clip=true, length={20,50}})
work(G.walk - G.wshadow, {hand="body", pile=P.walk, angle=0, coverage=2.5, fill=true, clip=true, length={30,80}})
work(G.wshadow, {hand="body", pile=P.wshad, angle=0, coverage=2.5, fill=true, clip=true, length={30,80}})
work(G.street + G.cross, {hand="body", pile=P.street, angle=0, coverage=2.5, fill=true, clip=true, length={30,80}})

--@ chunk 15
print(wait(2*24*60))
local s = {}
for _,p in ipairs({{300,200},{720,300},{300,385},{850,400},{800,600},{200,600},{300,655},{100,500}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 16
print(wait(3*24*60))
local s = {}
for _,p in ipairs({{300,200},{720,300},{300,385},{850,400},{800,600},{200,600},{300,655},{100,500}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 17
P.side2 = pile{{"red earth",1},{"burnt sienna",0.8},{"ultramarine blue",0.6},{"bone black",0.12},{"lead white",0.4}, medium=0.08, name="side wall dark"}
work(G.side, {hand="body", pile=P.side2, angle=1.45, coverage=2.5, fill=true, clip=true, length={20,50}})
P.brick3 = pile{{"red earth",2},{"deep cadmium",0.25},{"yellow ochre",0.4},{"lead white",0.7},{"burnt sienna",0.2}, medium=0.08, name="brick rich"}
-- trial patch inside where a window will go
work(rect(240,150,60,70), {hand="body", pile=P.brick3, angle=0, coverage=2.5, fill=true, clip=true, length={20,40}})

--@ chunk 18
P.brick4 = pile{{"red earth",2},{"burnt sienna",0.4},{"deep cadmium",0.12},{"yellow ochre",0.25},{"lead white",0.6}, medium=0.08, name="brick red"}
work(rect(410,150,60,70), {hand="body", pile=P.brick4, angle=0, coverage=2.5, fill=true, clip=true, length={20,40}})

--@ chunk 19
local m = G.upper - G.fshadow
work(m, {hand="body", piles={{P.brick4, function(x,y) return 0.6+0.4*smoothstep(108,368,y) end},{P.brick3, function(x,y) return 0.35*(1-smoothstep(108,368,y)) end}},
  angle=0, angle_jitter=0.04, coverage=2.5, fill=true, clip=true, length={30,70}})

--@ chunk 20
P.creamS = pile{{"lead white",1},{"yellow ochre",0.2},{"ultramarine blue",0.22},{"red earth",0.2},{"bone black",0.08}, medium=0.08, name="cream shade"}
P.glass = pile{{"bone black",1},{"viridian",0.4},{"ultramarine blue",0.4},{"burnt sienna",0.3},{"lead white",0.12}, medium=0.1, name="glass dark"}
P.bshad2 = pile{{"red earth",1.2},{"burnt sienna",1},{"ultramarine blue",0.3},{"bone black",0.15},{"lead white",0.3}, medium=0.08, name="brick shade 2"}
G.glassL = rect(30,410,390,138)
G.glassR = rect(555,410,117,138)
G.door = rect(445,410,90,165)
G.frame = G.storey - G.glassL - G.glassR - G.door
work(G.frame - G.fshadow, {hand="body", pile=P.stone, angle=0, coverage=2.5, fill=true, clip=true, length={15,40}})
work(G.frame * G.fshadow, {hand="body", pile=P.creamS, angle=0, coverage=2.5, fill=true, clip=true, length={15,40}})

--@ chunk 21
P.glassLit = pile{{"bone black",0.8},{"viridian",0.4},{"ultramarine blue",0.4},{"burnt sienna",0.2},{"lead white",0.45}, medium=0.1, name="glass lit"}
local glass = G.glassL + G.glassR
work(glass * G.fshadow, {hand="body", pile=P.glass, angle=0, coverage=2.5, fill=true, clip=true, length={20,50}})
work(glass - G.fshadow, {hand="body", pile=P.glassLit, angle=0, coverage=2.5, fill=true, clip=true, length={20,50}})
work(G.door, {hand="body", pile=P.glass, angle=1.5708, coverage=2.5, fill=true, clip=true, length={20,50}})

--@ chunk 22
print(wait(3*24*60))
local s = {}
for _,p in ipairs({{300,200},{720,300},{300,385},{100,385},{20,500},{200,480},{490,500},{800,600},{200,600}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 23
print(wait(4*24*60))
local s = {}
for _,p in ipairs({{300,200},{720,300},{300,385},{100,385},{20,500},{200,480},{490,500},{800,600},{200,600}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 24
P.creamS2 = pile{{"lead white",1},{"yellow ochre",0.25},{"ultramarine blue",0.35},{"red earth",0.3},{"bone black",0.18}, medium=0.08, name="cream shade 2"}
P.greenS = pile{{"viridian",1},{"bone black",1.6},{"burnt sienna",0.3}, medium=0.08, name="green shade"}
P.wshad2 = pile{{"lead white",1.2},{"ultramarine blue",0.35},{"burnt sienna",0.3},{"bone black",0.2},{"yellow ochre",0.15}, medium=0.08, name="walk shade 2"}
P.side3 = pile{{"red earth",1.2},{"burnt sienna",1},{"ultramarine blue",0.4},{"bone black",0.1},{"lead white",0.55}, medium=0.08, name="side wall 3"}

--@ chunk 25
P.creamS2:add{{"ultramarine blue",0.15},{"red earth",0.15},{"bone black",0.12},{"yellow ochre",0.1}}
P.wshad2:add{{"ultramarine blue",0.12},{"burnt sienna",0.12},{"bone black",0.1}}
P.side4 = mix{{P.bshad2,0.6},{P.side3,0.4}, name="side wall 4"}

--@ chunk 26
local S = G.fshadow
work(G.upper * S, {hand="body", pile=P.bshad2, angle=0, coverage=2.5, fill=true, clip=true, length={20,50}})
work(rect(0,368,690,34) * S, {hand="body", pile=P.greenS, angle=0, coverage=2.5, fill=true, clip=true, length={20,50}})
work(G.frame * S, {hand="body", pile=P.creamS2, angle=0, coverage=2.5, fill=true, clip=true, length={15,40}})
work(G.side, {hand="body", pile=P.side4, angle=1.45, coverage=3, fill=true, clip=true, length={20,50}})
work(G.wshadow, {hand="body", pile=P.wshad2, angle=0, coverage=2.5, fill=true, clip=true, length={30,80}})

--@ chunk 27
P.shade = pile{{"lead white",3},{"yellow ochre",0.7},{"cadmium yellow",0.12},{"red earth",0.05}, medium=0.08, name="blind"}
P.shadeS = pile{{"lead white",1},{"yellow ochre",0.6},{"red earth",0.2},{"ultramarine blue",0.15},{"bone black",0.1}, medium=0.08, name="blind shade"}
P.sash = pile{{"lead white",5},{"yellow ochre",0.3}, medium=0.08, name="sash white"}
WIN = {
  {100,140,232, 0.55}, {270,140,232, 0.9}, {440,140,232, 0.3}, {610,140,232, 0.0},
  {100,262,350, 0.75}, {270,262,350, 0.15}, {440,262,350, 0.5}, {610,262,350, 0.95},
}
function fillm(m, p, ang, tool)
  work(m, {hand="body", pile=p, angle=ang or 0, coverage=3, fill=true, clip=true, length={6,20}, tool=tool or "filbert 5"})
end
for _,w in ipairs(WIN) do
  local cx,y0,y1 = w[1],w[2],w[3]
  fillm(rect(cx-46,y0-12,92,12), P.stone)   -- lintel
  fillm(rect(cx-44,y1,88,7), P.stone)       -- sill
  fillm(rect(cx-38,y0,76,y1-y0), P.glass, 1.5708)
end

--@ chunk 28
print(wait(5*24*60))
local s = {}
for _,p in ipairs({{100,180},{440,300},{100,135},{500,500}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 29
print(wait(4*24*60))
local s = {}
for _,p in ipairs({{100,180},{440,300},{100,135},{500,500}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 30
for _,w in ipairs(WIN) do
  local cx,y0,y1,f = w[1],w[2],w[3],w[4]
  local h = y1-y0
  if f > 0 then
    local sh = rect(cx-36, y0, 72, f*h)
    local shadow = rect(cx-38, y0, 10, h) + rect(cx-38, y0, 76, 7)
    fillm(sh - shadow, P.shade, 1.5708, "filbert 4")
    fillm(sh * shadow, P.shadeS, 1.5708, "filbert 3")
  end
end

--@ chunk 31
print(wait(6*24*60))
local s = {}
for _,p in ipairs({{100,180},{440,300},{270,180},{610,300}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 32
print(wait(10*24*60))
local s = {}
for _,p in ipairs({{100,180},{440,300},{270,180},{610,300},{100,280}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 33
for _,w in ipairs(WIN) do
  local cx,y0,y1,f = w[1],w[2],w[3],w[4]
  local h = y1-y0
  if f > 0 then
    local sh = rect(cx-36, y0, 72, f*h)
    local shadow = rect(cx-38, y0, 10, h) + rect(cx-38, y0, 76, 7)
    fillm(sh - shadow, P.shade, 1.5708, "filbert 4")
    fillm(sh * shadow, P.shadeS, 1.5708, "filbert 3")
  end
end

--@ chunk 34
P.dist2 = pile{{"lead white",3},{"yellow ochre",0.5},{"cerulean blue",0.12},{"red earth",0.1}, medium=0.08, name="distant 2"}
P.distWin = pile{{"bone black",0.6},{"ultramarine blue",0.4},{"burnt sienna",0.3},{"lead white",0.6}, medium=0.08, name="distant win"}
-- right building (880..1000): lower, top 330, sky above it from 300..330
fillm(rect(880,300,120,30), P.skylo, 0, "filbert 6")
fillm(rect(880,330,120,195), P.dist2, 0, "filbert 6")
-- cornice band left building
fillm(rect(745,300,135,8), P.stone, 0, "filbert 3")

--@ chunk 35
P.skymid = mix{{P.skyhi,0.3},{P.skylo,0.7}, name="sky mid"}
fillm(rect(880,300,120,30), P.skymid, 0, "filbert 6")
P.dist3 = pile{{"lead white",2.2},{"yellow ochre",0.7},{"red earth",0.35},{"bone black",0.08},{"cerulean blue",0.05}, medium=0.08, name="distant 3"}
fillm(rect(880,330,120,195), P.dist3, 0, "filbert 6")

--@ chunk 36
fillm(rect(0,95,690,9), P.creamS2, 0, "filbert 4")
fillm(rect(0,104,690,9), P.bshad2, 0, "filbert 4")
-- lintel and sill cast shadows on brick (sun upper left: shadows fall below and to the right)
for _,w in ipairs(WIN) do
  local cx,y0,y1 = w[1],w[2],w[3]
  fillm(poly({{cx-40,y0},{cx+46,y0},{cx+46,y0+5}, {cx+38,y0+5},{cx+38,y0}}) , P.bshad2, 0, "round 2")
  fillm(poly({{cx-40,y1+7},{cx+44,y1+7},{cx+50,y1+13},{cx-34,y1+13}}), P.bshad2, 0, "filbert 3")
end

--@ chunk 37
P.distS = pile{{"lead white",1.4},{"yellow ochre",0.4},{"red earth",0.3},{"ultramarine blue",0.25},{"bone black",0.08}, medium=0.08, name="distant shade"}
-- distant left block windows: 3 rows, cols
local function dwin(x,y,w,h)
  fillm(rect(x,y,w,h), P.distWin, 1.5708, "round 2.5")
end
for _,yy in ipairs({325,375,425}) do
  for _,xx in ipairs({770,805,840}) do dwin(xx,yy,16,28) end
end
for _,yy in ipairs({355,410}) do
  for _,xx in ipairs({900,940,975}) do dwin(xx,yy,18,30) end
end
-- distant storefront dark band and awning
fillm(rect(752,478,120,40), P.distWin, 0, "filbert 3")
fillm(rect(885,470,115,48), P.distWin, 0, "filbert 3")

--@ chunk 38
print(wait(12*24*60))
local s = {}
for _,p in ipairs({{950,400},{920,380},{780,340},{800,500}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 39
P.distWin2 = pile{{"bone black",0.8},{"ultramarine blue",0.3},{"burnt sienna",0.45},{"lead white",0.5}, medium=0.08, name="distant win 2"}
for _,yy in ipairs({355,410}) do
  for _,xx in ipairs({900,940,975}) do fillm(rect(xx,yy,18,30), P.distWin2, 1.5708, "round 2.5") end
end
fillm(rect(885,470,115,48), P.distWin2, 0, "filbert 3")
-- water tower on right block roof
P.wood = pile{{"burnt sienna",1},{"bone black",0.4},{"lead white",0.6},{"yellow ochre",0.3}, medium=0.08, name="tower wood"}
P.woodS = pile{{"burnt sienna",0.8},{"bone black",0.6},{"ultramarine blue",0.2},{"lead white",0.35}, medium=0.08, name="tower shade"}
local barrel = poly({{928,262},{966,262},{964,305},{930,305}})
fillm(barrel * rect(928,250,24,60), P.wood, 1.5708, "round 2.5")
fillm(barrel * rect(952,250,20,60), P.woodS, 1.5708, "round 2.5")
fillm(poly({{924,263},{947,240},{970,263}}), P.woodS, 0, "round 2.5")
local lb = brush{kind="round", width=1.6, point=0.3}
lb:load(P.woodS, 0.8)
for _,x in ipairs({933,946,960}) do lb:stroke({{x,305},{x+ (x-946)*0.08,330}}, {pressure=0.7}) end
lb:stroke({{931,318},{962,318}}, {pressure=0.5})

--@ chunk 40
for _,yy in ipairs({325,375,425}) do
  for _,xx in ipairs({770,805,840}) do fillm(rect(xx,yy,16,28), P.distWin2, 1.5708, "round 2.5") end
end
fillm(rect(752,478,120,40), P.distWin2, 0, "filbert 3")
local lb = brush{kind="round", width=1.8}
lb:load(P.woodS, 1)
for _,x in ipairs({933,946,960}) do lb:stroke({{x,304},{x+(x-946)*0.08,331}}, {pressure=0.9}) end
lb:load(P.woodS, 1)
lb:stroke({{931,318},{962,318}}, {pressure=0.8})

--@ chunk 41
P.skytop = pile{{"lead white",2.2},{"cerulean blue",1},{"cobalt blue",0.5},{"ultramarine blue",0.08}, medium=0.08, name="sky top"}
P.skyhz = pile{{"lead white",7},{"cerulean blue",0.45},{"pale cadmium",0.15},{"yellow ochre",0.05}, medium=0.08, name="sky horizon"}
local tower = rect(920,236,58,98)
G.skyall = (G.sky + rect(880,300,120,30)) - tower
work(G.skyall, {hand="broad", piles={
   {P.skytop, function(x,y) return 1-smoothstep(0,260,y) end},
   {P.skymid, function(x,y) local t=smoothstep(0,260,y) return t*(1-smoothstep(200,330,y)) end},
   {P.skyhz, function(x,y) return smoothstep(200,330,y) end}},
  angle=0.0, angle_jitter=0.03, coverage=3, fill=true, clip=true})

--@ chunk 42
TOWER = poly({{928,262},{966,262},{964,305},{930,305}}) + poly({{924,263},{947,240},{970,263}})
  + ribbon({{933,304},{932,331}},2.4) + ribbon({{946,304},{946,331}},2.4) + ribbon({{960,304},{961,331}},2.4) + ribbon({{931,318},{962,318}},2.2)
local box = rect(920,236,58,98) - TOWER:grow(0.5)
work(box, {hand="body", tool="filbert 4", piles={
   {P.skymid, function(x,y) return 1-smoothstep(200,330,y) end},
   {P.skyhz, function(x,y) return smoothstep(200,330,y) end}},
  angle=0, coverage=3, fill=true, clip=true, length={6,16}})

--@ chunk 43
local s = {}
for _,p in ipairs({{500,620},{300,600},{300,655},{720,300},{100,500}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))
-- correct shadow geometry on the ground: edge recedes toward VP (900,455)
local function xat(y) return 470 - (y-575)/0.279 end
G.wshadow2 = poly({{0,575},{470,575},{xat(667),667},{0,667}})
G.walklit = rect(0,575,1000,65) - G.wshadow2
print(xat(640), xat(667))

--@ chunk 44
P.walk2 = pile{{"lead white",4},{"yellow ochre",0.6},{"red earth",0.2},{"bone black",0.06},{"cadmium yellow",0.05}, medium=0.08, name="walk lit 2"}
P.walk3 = pile{{"lead white",3},{"yellow ochre",0.6},{"red earth",0.3},{"bone black",0.12},{"ultramarine blue",0.04}, medium=0.08, name="walk lit far"}
local lit = rect(0,575,1000,65) - G.wshadow2
work(lit, {hand="body", piles={{P.walk2, function(x,y) return smoothstep(560,640,y) end},{P.walk3, function(x,y) return 1-smoothstep(560,640,y) end}},
   angle=0, coverage=3, fill=true, clip=true, length={30,80}})

--@ chunk 45
P.gshad = pile{{"lead white",1.3},{"ultramarine blue",0.3},{"burnt sienna",0.25},{"yellow ochre",0.25},{"bone black",0.22}, medium=0.08, name="ground shade"}
P.asph = pile{{"lead white",1},{"bone black",0.35},{"burnt sienna",0.25},{"yellow ochre",0.25},{"ultramarine blue",0.08}, medium=0.08, name="asphalt lit"}
P.asphS = pile{{"lead white",0.6},{"bone black",0.5},{"ultramarine blue",0.25},{"burnt sienna",0.25}, medium=0.08, name="asphalt shade"}
P.curb = pile{{"lead white",3},{"yellow ochre",0.4},{"red earth",0.15},{"bone black",0.15}, medium=0.08, name="curb lit"}

--@ chunk 46
P.asphS:add{{"burnt sienna",0.12},{"yellow ochre",0.08}}
P.curbS = mix{{P.gshad,0.8},{P.curb,0.2}, name="curb shade"}
local function xat(y) return 470 - (y-575)/0.279 end
local SH = G.wshadow2
work(rect(0,575,1000,65) * SH, {hand="body", pile=P.gshad, angle=0, coverage=3, fill=true, clip=true, length={30,80}})
local curb = rect(0,640,1000,9)
work(curb - SH, {hand="body", pile=P.curb, angle=0, coverage=3, fill=true, clip=true, length={20,60}, tool="filbert 5"})
work(curb * SH, {hand="body", pile=P.curbS, angle=0, coverage=3, fill=true, clip=true, length={20,60}, tool="filbert 5"})
local st = rect(0,649,1000,18)
work(st - SH, {hand="body", pile=P.asph, angle=0, coverage=3, fill=true, clip=true, length={30,80}, tool="filbert 7"})
work(st * SH, {hand="body", pile=P.asphS, angle=0, coverage=3, fill=true, clip=true, length={30,80}, tool="filbert 7"})

--@ chunk 47
local cross = poly({{690,575},{745,545},{745,525},{1000,525},{1000,575}})
local shline = poly({{690,575},{745,545},{745,520},{1000,520},{1000,548}})
local farwalk = rect(745,518,255,9)
work(farwalk, {hand="body", pile=P.walk3, angle=0, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 4"})
work((cross*shline) - farwalk, {hand="body", pile=P.asphS, angle=0, coverage=3, fill=true, clip=true, length={20,50}, tool="filbert 5"})
work(cross - shline, {hand="body", pile=P.asph, angle=0, coverage=3, fill=true, clip=true, length={20,50}, tool="filbert 5"})

--@ chunk 48
print(wait(10*24*60))
local s = {}
for _,p in ipairs({{300,600},{900,560},{900,540},{500,655},{300,660}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 49
P.curt = pile{{"lead white",4},{"yellow ochre",0.35},{"cadmium yellow",0.06}, medium=0.08, name="curtain lit"}
P.curtS = pile{{"lead white",1.2},{"yellow ochre",0.3},{"ultramarine blue",0.25},{"red earth",0.15},{"bone black",0.2}, medium=0.08, name="curtain shade"}
local S = G.fshadow
CURT = rect(32,480,386,66) + rect(557,480,113,66)
work(CURT - S, {hand="body", pile=P.curt, angle=1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 4"})
work(CURT * S, {hand="body", pile=P.curtS, angle=1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 4"})

--@ chunk 50
print(wait(4*24*60))
print(drying(200,520), drying(380,520))

--@ chunk 51
print(wait(8*24*60))
print(drying(200,520), drying(380,520))

--@ chunk 52
P.curtS2 = pile{{"lead white",0.8},{"yellow ochre",0.3},{"ultramarine blue",0.3},{"red earth",0.2},{"bone black",0.3},{"viridian",0.05}, medium=0.08, name="curtain shade 2"}
work(CURT * G.fshadow, {hand="body", pile=P.curtS2, angle=1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 4"})

--@ chunk 53
P.lamp = pile{{"lead white",4},{"yellow ochre",0.4},{"cadmium yellow",0.15}, medium=0.08, name="lamp"}
P.interior = pile{{"bone black",0.7},{"burnt sienna",0.5},{"yellow ochre",0.2},{"lead white",0.35}, medium=0.08, name="interior warm"}
-- dim warm back wall band inside, behind the dark glass, above curtain
local S = G.fshadow
local band = rect(32,445,386,35) + rect(557,445,113,35)
work(band - S, {hand="body", pile=P.interior, angle=0, coverage=2, fill=true, clip=true, length={10,30}, tool="filbert 3"})
-- hanging globe lamps
for _,x in ipairs({120,250,380,615}) do
  local bb = brush{kind="round", width=1}
  bb:load(P.glass,0.8) bb:stroke({{x,412},{x,428}},{pressure=0.6})
  local m = ellipse(x,433,7,6)
  fillm(m, P.lamp, 0, "round 2")
end

--@ chunk 54
P.shirt = pile{{"lead white",5},{"yellow ochre",0.25},{"cadmium yellow",0.04}, medium=0.08, name="shirt lit"}
P.shirtS = pile{{"lead white",1.5},{"ultramarine blue",0.2},{"yellow ochre",0.25},{"bone black",0.12},{"red earth",0.08}, medium=0.08, name="shirt shade"}
P.skin = pile{{"lead white",1.5},{"red earth",0.5},{"yellow ochre",0.5},{"deep cadmium",0.08}, medium=0.08, name="skin lit"}
P.skinS = pile{{"red earth",0.6},{"burnt sienna",0.4},{"lead white",0.6},{"ultramarine blue",0.1}, medium=0.08, name="skin shade"}
P.trous = pile{{"bone black",0.6},{"ultramarine blue",0.3},{"lead white",0.35},{"burnt sienna",0.1}, medium=0.08, name="trousers"}
P.hair = pile{{"bone black",0.6},{"burnt sienna",0.5},{"lead white",0.1}, medium=0.08, name="hair"}

--@ chunk 55
P.shirtS:add{{"bone black",0.12},{"ultramarine blue",0.1}}
P.skinS:add{{"burnt sienna",0.25},{"bone black",0.08},{"yellow ochre",0.15}}
P.trous:add{{"bone black",0.4},{"burnt sienna",0.1}}
FIG = {}
FIG.head = ellipse(492,463,6.5,8.5)
FIG.neck = rect(488.5,469,7,8)
FIG.torso = poly({{476,477},{508,477},{511,488},{507,519},{479,519},{474,488}}, true)
FIG.arms = poly({{474,486},{479,496},{508,498},{512,488},{510,507},{476,504}}, true)
FIG.forearm = poly({{478,496},{507,499},{507,506},{478,503}}, true)
FIG.apron = poly({{479,514},{507,514},{510,549},{477,549}})
FIG.legs = poly({{480,545},{492,545},{492,571},{481,571}}) + poly({{494,545},{506,545},{505,571},{495,571}})
FIG.all = FIG.head + FIG.neck + FIG.torso + FIG.arms + FIG.apron + FIG.legs
LITX = function(x0,x1) return mask(function(x,y) return x < x0 + (y-450)*0 and 1 or 0 end) end
FIG.litside = rect(440,440,56,140)  -- x<496 lit
print(FIG.all:area())

--@ chunk 56
local L = FIG.litside
local function f(m,p,t) work(m, {hand="body", pile=p, angle=1.5708, coverage=3, fill=true, clip=true, length={3,10}, tool=t or "round 2"}) end
f(FIG.legs, P.trous)
local shirtm = FIG.torso + FIG.arms - FIG.forearm
f(shirtm - L, P.shirtS)
f((shirtm * L), P.shirt)
f(FIG.apron - L, P.shirtS)
f(FIG.apron * L, P.shirt)
f(FIG.forearm * L, P.skin, "round 1.5")
f(FIG.forearm - L, P.skinS, "round 1.5")
f((FIG.head + FIG.neck) * rect(440,440,51,140), P.skin, "round 1.5")
f((FIG.head + FIG.neck) - rect(440,440,51,140), P.skinS, "round 1.5")
f(FIG.head * poly({{484,452},{500,452},{500,461},{491,458},{484,462}}), P.hair, "round 1.5")

--@ chunk 57
print(wait(10*24*60))
print(drying(485,500), drying(500,500), drying(490,460))

--@ chunk 58
P.shirtM = mix{{P.shirt,0.55},{P.shirtS,0.45}, name="shirt mid"}
P.shirtS2 = pile{{"lead white",1.2},{"ultramarine blue",0.3},{"yellow ochre",0.2},{"bone black",0.3},{"red earth",0.1}, medium=0.08, name="shirt shade 2"}
local function f(m,p,t) work(m, {hand="body", pile=p, angle=1.5708, coverage=3, fill=true, clip=true, length={3,10}, tool=t or "round 1.5"}) end
-- slope shoulders: door dark over corners
f(poly({{472,474},{486,474},{474,486}}), P.glass)
f(poly({{499,474},{513,474},{513,490}}), P.glass)
-- clean apron hem with trousers top
f(rect(479,546,28,5), P.trous)
-- form shading: mid tone band and deeper shade right edge
local shirtm = FIG.torso + FIG.arms - FIG.forearm + FIG.apron - poly({{472,474},{486,474},{474,486}}) - poly({{499,474},{513,474},{513,490}})
local mid = mask(function(x,y) local c = 494 + (y-500)*0.05 return (x>c-4 and x<c+4) and 1 or 0 end)
local deep = mask(function(x,y) local c = 494 + (y-500)*0.05 return (x>=c+4) and 1 or 0 end)
f(shirtm * mid, P.shirtM)
f(shirtm * deep, P.shirtS2)

--@ chunk 59
print(wait(10*24*60))
local s = {}
for _,p in ipairs({{485,500},{600,600},{950,320},{300,200},{120,433}}) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 60
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 0, coverage=3, fill=true, clip=true, length={4,14}, tool=t or "round 2"}) end
-- roof line under tower: restore building top
f(rect(918,329,62,7) - TOWER:grow(0.5), P.dist3, "round 2")
-- speckled storefront right distant
f(rect(885,470,115,48), P.distWin2, "filbert 3")
-- interior band in shadow part of left window
P.interiorS = pile{{"bone black",0.9},{"burnt sienna",0.5},{"ultramarine blue",0.15},{"lead white",0.15}, medium=0.08, name="interior shade"}
local band = rect(32,445,386,35)
f(band * G.fshadow, P.interiorS, "filbert 3")

--@ chunk 61
FONT = {
 L={{{0,0},{0,1},{0.8,1}}},
 U={{{0,0},{0,0.72},{0.12,0.93},{0.5,1},{0.88,0.93},{1,0.72},{1,0}}},
 N={{{0,1},{0,0},{1,1},{1,0}}},
 C={{{1,0.18},{0.78,0.02},{0.4,0},{0.08,0.18},{0,0.5},{0.08,0.82},{0.4,1},{0.78,0.98},{1,0.82}}},
 H={{{0,0},{0,1}},{{1,0},{1,1}},{{0,0.5},{1,0.5}}},
 E={{{0.9,0},{0,0},{0,1},{0.9,1}},{{0,0.5},{0.7,0.5}}},
 O={{{0.5,0},{0.12,0.12},{0,0.5},{0.12,0.88},{0.5,1},{0.88,0.88},{1,0.5},{0.88,0.12},{0.5,0}}},
 T={{{0,0},{1,0}},{{0.5,0},{0.5,1}}},
}
function letters(text, x0, y0, w, h, pitch, br, p)
  local x = x0
  for i=1,#text do
    local ch = text:sub(i,i)
    local g = FONT[ch]
    if g then
      for _,s in ipairs(g) do
        local pts = {}
        for _,q in ipairs(s) do pts[#pts+1] = {x + q[1]*w, y0 + q[2]*h} end
        br:load(p, 0.7)
        br:stroke(pts, {pressure=0.75, shake=0.3})
      end
    end
    x = x + pitch
  end
end
P.letter = pile{{"lead white",3},{"yellow ochre",0.5},{"red earth",0.05}, medium=0.12, name="letter"}
local br = brush{kind="round", width=2.4}
local txt = "LUNCHEONETTE"
local pitch, w = 17, 11
local total = (#txt-1)*pitch + w
letters(txt, 360 - total/2, 377, w, 16, pitch, br, P.letter)

--@ chunk 62
P.sashS = pile{{"lead white",1.2},{"yellow ochre",0.25},{"ultramarine blue",0.2},{"red earth",0.15},{"bone black",0.12}, medium=0.08, name="sash shade"}
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 0, coverage=3, fill=true, clip=true, length={4,14}, tool=t or "round 1.5"}) end
for _,w in ipairs(WIN) do
  local cx,y0,y1,fr = w[1],w[2],w[3],w[4]
  local h = y1-y0
  local ym = y0 + 0.5*h
  local rail = rect(cx-38, ym-1.5, 76, 3.5)
  local blindBottom = y0 + fr*h
  local lit = rail - rect(cx-38,y0,10,h)
  f(lit, P.sash)
  f(rail * rect(cx-38,y0,10,h), P.sashS)
  -- right stile, lit
  f(rect(cx+34, y0+7, 4, h-7), P.sash, "round 1.5", 1.5708)
end

--@ chunk 63
print(wait(5*24*60))
local holes = rect(0,0,1,1)
for _,w in ipairs(WIN) do
  local cx,y0,y1 = w[1],w[2],w[3]
  holes = holes + rect(cx-47,y0-13,94,y1-y0+22) + poly({{cx-40,y1+7},{cx+44,y1+7},{cx+50,y1+13},{cx-34,y1+13}})
end
BRICK = rect(0,113,690,255) - holes
BRICKLIT = BRICK - G.fshadow
P.brickHi = pile{{"red earth",1.6},{"yellow ochre",0.4},{"deep cadmium",0.12},{"lead white",1.0}, medium=0.1, name="brick hi"}
P.brickLo = pile{{"red earth",1.6},{"burnt sienna",0.6},{"ultramarine blue",0.1},{"lead white",0.45}, medium=0.1, name="brick lo"}
print(drying(300,150))

--@ chunk 64
work(BRICKLIT * rect(150,115,70,240), {hand="scumble", pile=P.brickHi, coverage=0.6, load=0.3, angle=0, clip=true, tool="filbert 6"})

--@ chunk 65
local r = rag{width=40}
r:dip(0.6)
r:wipe(rect(146,112,78,246), {pressure=0.7, angle=1.5708, passes=2, refold=0.4})
print(r)

--@ chunk 66
local r = rag{width=30}
r:dip(1)
r:wipe(rect(156,118,58,236), {pressure=0.8, angle=1.5708, passes=3, refold=0.3})
r:refold() r:dip(1)
-- edges of windows where smeared
r:wipe({{142,150},{142,350}}, {pressure=0.6})
r:refold() r:dip(1)
r:wipe({{228,150},{228,350}}, {pressure=0.6})
print(r)

--@ chunk 67
print(wait(6*24*60))
print(drying(185,200), drying(140,300), drying(225,500))

--@ chunk 68
local m = BRICKLIT * rect(138,108,100,262)
work(m, {hand="body", piles={{P.brick4, function(x,y) return 0.6+0.4*smoothstep(108,368,y) end},{P.brick3, function(x,y) return 0.35*(1-smoothstep(108,368,y)) end}},
  angle=0, angle_jitter=0.04, coverage=3, fill=true, clip=true, length={20,50}, tool="filbert 6"})

--@ chunk 69
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={4,14}, tool=t or "round 2"}) end
-- window cx=100: right part of glass/blind near x 125..134 and stile 134..138
for _,yy in ipairs({{140,232,0.55},{262,350,0.75}}) do
  local y0,y1,fr = yy[1],yy[2],yy[3]
  local h=y1-y0
  local yb = y0 + fr*h
  f(rect(124,y0+7,10,yb-y0-7), P.shade)
  f(rect(124,yb,10,y1-yb), P.glass)
end
-- window cx=270: left reveal x 232..242
for _,yy in ipairs({{140,232,0.9},{262,350,0.15}}) do
  local y0,y1,fr = yy[1],yy[2],yy[3]
  local h=y1-y0
  local yb = y0 + fr*h
  f(rect(232,y0,10,yb-y0), P.shadeS)
  f(rect(232,yb,10,y1-yb), P.glass)
end
-- lintels / sills ends
for _,y in ipairs({128,250}) do f(rect(136,y,10,12), P.stone, "round 2", 0) f(rect(224,y,10,12), P.stone, "round 2", 0) end
for _,y in ipairs({232,350}) do f(rect(136,y,8,7), P.stone, "round 2", 0) f(rect(226,y,8,7), P.stone, "round 2", 0) end

--@ chunk 70
print(wait(12*24*60))
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 0, coverage=3, fill=true, clip=true, length={4,14}, tool=t or "round 1.5"}) end
for _,w in ipairs({{100,140,232},{100,262,350},{270,140,232},{270,262,350}}) do
  local cx,y0,y1 = w[1],w[2],w[3]
  local ym = y0 + 0.5*(y1-y0)
  local rail = rect(cx-38, ym-1.5, 76, 3.5)
  f(rail - rect(cx-38,y0,10,200), P.sash)
  f(rail * rect(cx-38,y0,10,200), P.sashS)
end
-- brick strips beside windows
local strip = (rect(138,113,8,250) + rect(224,113,8,250)) * BRICKLIT
work(strip, {hand="body", pile=P.brick4, angle=1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="round 3"})

--@ chunk 71
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 0, coverage=3, fill=true, clip=true, length={6,20}, tool=t or "filbert 3"}) end
local function yv(y0, x) return y0 + (x-690)*(455-y0)/210 end
P.corS = pile{{"lead white",1},{"yellow ochre",0.3},{"ultramarine blue",0.3},{"red earth",0.25},{"bone black",0.15}, medium=0.08, name="cornice shade"}
-- side return (shade)
local ret = poly({{690,64},{745,yv(64,745)},{745,yv(101,745)},{690,101}})
f(ret, P.corS, "filbert 3", 1.1)
f(poly({{690,101},{745,yv(101,745)},{745,yv(107,745)+4},{690,108}}), P.side4, "round 2", 1.1)
-- front
f(rect(0,64,690,8), P.sash, "filbert 3")
f(rect(0,72,690,3), P.corS, "round 1.5")
f(rect(0,75,690,15), P.stone, "filbert 3")
f(rect(0,96,690,6), P.corS, "round 2")
f(rect(0,102,690,14), P.bshad2, "filbert 3")

--@ chunk 72
P.sideD = pile{{"red earth",1},{"burnt sienna",1},{"ultramarine blue",0.6},{"bone black",0.2},{"lead white",0.35}, medium=0.08, name="side wall deep"}
P.bshadD = pile{{"red earth",1.2},{"burnt sienna",1},{"ultramarine blue",0.45},{"bone black",0.22},{"lead white",0.2}, medium=0.08, name="brick shade deep"}

--@ chunk 73
sideD = P.sideD
bshadD = P.bshadD
print(drying(720,300), drying(100,350))

--@ chunk 74
HOLES = rect(0,0,1,1)
for _,w in ipairs(WIN) do
  local cx,y0,y1 = w[1],w[2],w[3]
  HOLES = HOLES + rect(cx-47,y0-13,94,y1-y0+22) + poly({{cx-40,y1+7},{cx+44,y1+7},{cx+50,y1+13},{cx-34,y1+13}})
end
local function yv(y0, x) return y0 + (x-690)*(455-y0)/210 end
SIDEW = G.side - poly({{690,0},{745,0},{745,yv(108,745)+3},{690,109}})
work(SIDEW, {hand="body", pile=P.sideD, angle=1.45, coverage=3, fill=true, clip=true, length={20,50}, tool="filbert 6"})
work(G.upper * G.fshadow - HOLES, {hand="body", pile=P.bshadD, angle=0, coverage=3, fill=true, clip=true, length={20,50}, tool="filbert 6"})

--@ chunk 75
local function yv(y0, x) return y0 + (x-690)*(455-y0)/210 end
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={4,14}, tool=t or "round 2"}) end
P.glassSide = pile{{"bone black",1},{"ultramarine blue",0.5},{"burnt sienna",0.4},{"lead white",0.25}, medium=0.08, name="side glass"}
P.stoneSide = pile{{"lead white",1},{"yellow ochre",0.25},{"ultramarine blue",0.25},{"red earth",0.25},{"bone black",0.12}, medium=0.08, name="stone side"}
for _,row in ipairs({{140,232},{262,350}}) do
  for _,xs in ipairs({{702,715},{726,737}}) do
    local a,b = xs[1], xs[2]
    local win = poly({{a,yv(row[1],a)},{b,yv(row[1],b)},{b,yv(row[2],b)},{a,yv(row[2],a)}})
    f(win, P.glassSide, "round 1.5")
    local sill = poly({{a-1,yv(row[2],a-1)},{b+1,yv(row[2],b+1)},{b+1,yv(row[2]+6,b+1)},{a-1,yv(row[2]+6,a-1)}})
    f(sill, P.stoneSide, "round 1.5", 1.1)
    local lint = poly({{a-1,yv(row[1]-11,a-1)},{b+1,yv(row[1]-11,b+1)},{b+1,yv(row[1],b+1)},{a-1,yv(row[1],a-1)}})
    f(lint, P.stoneSide, "round 1.5", 1.1)
  end
end

--@ chunk 76
print(wait(14*24*60))
print(drying(708,200), drying(720,400), drying(100,350))

--@ chunk 77
local function yv(y0, x) return y0 + (x-690)*(455-y0)/210 end
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={4,14}, tool=t or "round 2"}) end
for _,row in ipairs({{140,232},{262,350}}) do
  for _,xs in ipairs({{702,715},{726,737}}) do
    local a,b = xs[1], xs[2]
    local win = poly({{a,yv(row[1],a)},{b,yv(row[1],b)},{b,yv(row[2],b)},{a,yv(row[2],a)}})
    f(win, P.glassSide, "round 1.5")
    local sill = poly({{a-1,yv(row[2],a-1)},{b+1,yv(row[2],b+1)},{b+1,yv(row[2]+6,b+1)},{a-1,yv(row[2]+6,a-1)}})
    f(sill, P.stoneSide, "round 1.5", 1.1)
    local lint = poly({{a-1,yv(row[1]-11,a-1)},{b+1,yv(row[1]-11,b+1)},{b+1,yv(row[1],b+1)},{a-1,yv(row[1],a-1)}})
    f(lint, P.stoneSide, "round 1.5", 1.1)
  end
end
-- sign band and storefront wrap the corner
P.greenSide = pile{{"viridian",1},{"bone black",1.4},{"burnt sienna",0.3},{"lead white",0.1}, medium=0.08, name="green side"}
local band = poly({{690,368},{745,yv(368,745)},{745,yv(402,745)},{690,402}})
f(band, P.greenSide, "round 2", 1.1)
local base = poly({{690,402},{700,yv(402,700)},{700,yv(575,700)},{690,575}})
f(base, P.stoneSide, "round 2", 1.5708)

--@ chunk 78
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={3,10}, tool=t or "round 1.5"}) end
-- open window: cover old meeting rail with dark, new rail under blind
f(rect(402,183.5,76,6), P.glass, "round 2", 0)
f(rect(412,166,66,4), P.sash, "round 1.5", 0)
f(rect(402,166,10,4), P.sashS, "round 1.5", 0)
P.dress = pile{{"lead white",3},{"red earth",0.35},{"deep cadmium",0.08},{"yellow ochre",0.15}, medium=0.08, name="dress lit"}
P.dressS = pile{{"lead white",1},{"red earth",0.4},{"ultramarine blue",0.2},{"bone black",0.12}, medium=0.08, name="dress shade"}
WOM = {}
WOM.head = ellipse(446,186,5.2,6.6)
WOM.neck = rect(443.5,191,5,5)
WOM.torso = poly({{432,197},{459,197},{462,206},{463,226},{429,226},{430,206}}, true)
WOM.arms = poly({{422,221},{470,221},{472,229},{420,229}}, true)
WOM.hair = poly({{439,180},{446,178.5},{453,180},{454,190},{451,196},{452,186},{441,183},{440,190}}, true)
print("ok")

--@ chunk 79
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={3,8}, tool=t or "round 1.2"}) end
local Lt = rect(380,150,66,90)
local body = WOM.torso - WOM.arms
f(body * Lt, P.dress)
f(body - Lt, P.dressS)
f(WOM.arms * rect(380,150,62,90), P.skin, "round 1.2", 0)
f(WOM.arms - rect(380,150,62,90), P.skinS, "round 1.2", 0)
f((WOM.head + WOM.neck) * rect(380,150,64,90), P.skin)
f((WOM.head + WOM.neck) - rect(380,150,64,90), P.skinS)
P.hairW = pile{{"burnt sienna",0.8},{"bone black",0.35},{"red earth",0.3},{"lead white",0.1}, medium=0.08, name="hair auburn"}
f(WOM.hair, P.hairW)

--@ chunk 80
print(wait(12*24*60))
print(drying(446,210), drying(446,186), drying(440,225))

--@ chunk 81
print(wait(6*24*60))
print(drying(446,210), drying(446,186), drying(440,225))

--@ chunk 82
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={2,7}, tool=t or "round 1.2"}) end
-- reshape: dark interior over ends of arm bar and boxy shoulders
local darkfix = rect(412,219,15,11) + rect(466,219,12,11) + poly({{429,196},{437,196},{429,206}}) + poly({{455,196},{463,196},{463,206}})
f(darkfix, P.glass, "round 1.5")
-- forearms: two crossed, lit left one
local farmL = poly({{428,220},{452,223},{452,229},{427,228}}, true)
local farmR = poly({{441,223},{465,219},{466,226},{442,229}}, true)
f(farmR, P.skinS, "round 1.2", 0)
f(farmL, P.skin, "round 1.2", 0)
-- face: skin on front of face, hair at the sides/top
local face = ellipse(445.5,188,3.6,4.8)
f(face * rect(380,150,66,90), P.skin, "round 1")
f(face - rect(380,150,66,90), P.skinS, "round 1")
-- shoulder curve shading: soft mid along torso
P.dressM = mix{{P.dress,0.5},{P.dressS,0.5}, name="dress mid"}
f(poly({{444,198},{450,198},{450,219},{444,219}}), P.dressM, "round 1.2")

--@ chunk 83
P.seam = pile{{"lead white",1.5},{"yellow ochre",0.35},{"red earth",0.15},{"bone black",0.15},{"ultramarine blue",0.05}, medium=0.1, name="seam"}
P.seamS = pile{{"lead white",0.8},{"ultramarine blue",0.3},{"burnt sienna",0.25},{"bone black",0.3}, medium=0.1, name="seam shade"}
local sb = brush{kind="round", width=1.3, point=0.2}
local function xat(y) return 470 - (y-575)/0.279 end
-- seams converge to VP (900,455): from curb y=640 to building base y=575
for _,xc in ipairs({60, 230, 400, 570, 740, 910}) do
  local t = (575-640)/(455-640)
  local xb = xc + (900-xc)*t
  local shaded = xc < xat(640)
  sb:load(shaded and P.seamS or P.seam, 0.6)
  sb:stroke({{xc,639},{xb,576}}, {pressure=0.45, shake=0.4})
end
-- one transverse seam
sb:load(P.seam, 0.6)
sb:stroke({{xat(606)+4,606},{990,606}}, {pressure=0.4, shake=0.5})
sb:load(P.seamS, 0.6)
sb:stroke({{5,606},{xat(606)-4,606}}, {pressure=0.4, shake=0.5})

--@ chunk 84
P.post = pile{{"bone black",0.8},{"viridian",0.35},{"burnt sienna",0.2},{"lead white",0.25}, medium=0.08, name="post"}
P.postLit = pile{{"bone black",0.5},{"viridian",0.3},{"yellow ochre",0.25},{"lead white",0.7}, medium=0.08, name="post lit"}
POST = poly({{815,644},{827,644},{826,612},{823,604},{822.5,200},{819.5,200},{819,604},{816,612}})
POSTLIT = POST * rect(810,150,9.7,500)
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={6,20}, tool=t or "round 1.5"}) end
f(POST, P.post)
f(POSTLIT, P.postLit, "round 1")
-- lamp: arm and globe
GLOBE = ellipse(821,186,8,10)
f(rect(818,196,6,6), P.post, "round 1.5", 0)
f(GLOBE, P.lamp, "round 1.5")
f(GLOBE * rect(823,170,10,30), P.curt, "round 1.2")
f(poly({{814,177},{828,177},{825,172},{817,172}}), P.post, "round 1.2", 0)

--@ chunk 85
local m = poly({{816,645},{827,644},{1000,596},{1000,592}}, false)
work(m, {hand="body", pile=P.gshad, angle=-0.27, coverage=3, fill=true, clip=true, length={10,40}, tool="round 2"})

--@ chunk 86
work(rect(258,374,15,22), {hand="body", pile=P.green, angle=0, coverage=3, fill=true, clip=true, length={4,10}, tool="round 2"})

--@ chunk 87
local br = brush{kind="round", width=2.4}
br:load(P.letter, 0.7)
br:stroke({{266,377},{266,393},{275,393}}, {pressure=0.75, shake=0.3})

--@ chunk 88
print(wait(14*24*60))
print(drying(446,210), drying(446,188), drying(440,225), drying(266,385))

--@ chunk 89
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={2,6}, tool=t or "round 1"}) end
-- sloping shoulders: cut corners with interior dark
f(poly({{427,195},{441,195},{441,197},{430,204},{427,212}}), P.glass, "round 1.2")
f(poly({{451,195},{465,195},{465,212},{462,204},{451,197}}), P.glass, "round 1.2")
-- neck narrower in shade
P.dressS2 = pile{{"lead white",1.2},{"red earth",0.35},{"ultramarine blue",0.12},{"yellow ochre",0.15},{"bone black",0.1}, medium=0.08, name="dress shade 2"}
local torso = poly({{438,196},{455,196},{461,204},{463,222},{429,222},{431,204}}, true)
f(torso * rect(446,190,30,40), P.dressS2, "round 1.2")
-- elbows/upper arms down to sill
f(poly({{429,206},{433,206},{432,222},{426,222}}), P.skin, "round 1")
f(poly({{460,206},{464,206},{467,222},{461,222}}), P.skinS, "round 1")
-- face: unify
local face = ellipse(445.5,188.5,3.4,4.6)
f(face, P.skin, "round 1")
f(face * rect(447,180,6,20), P.skinS, "round 1")

--@ chunk 90
P.walkHi = pile{{"lead white",5},{"yellow ochre",0.5},{"cadmium yellow",0.08},{"red earth",0.1}, medium=0.15, name="walk hi"}
local trial = rect(870,615,120,22)
work(trial, {hand="scumble", pile=P.walkHi, coverage=0.5, load=0.25, angle=0, tool="filbert 8", pressure={0.2,0.4}})

--@ chunk 91
local r = rag{width=25}
r:dip(1)
r:wipe(rect(868,612,124,28), {pressure=0.8, angle=0, passes=3, refold=0.3})

--@ chunk 92
print(wait(12*24*60))
print(drying(900,625), drying(950,630))

--@ chunk 93
local function xat(y) return 470 - (y-575)/0.279 end
local lit = rect(0,575,1000,65) - G.wshadow2 - POST:grow(0.5)
work(lit, {hand="body", piles={{P.walk2, function(x,y) return smoothstep(570,640,y) end},{P.walk3, function(x,y) return 1-smoothstep(570,640,y) end}},
   angle=0, coverage=3, fill=true, clip=true, length={30,80}})
local m = poly({{816,645},{827,644},{1000,596},{1000,592}})
work(m, {hand="body", pile=P.gshad, angle=-0.27, coverage=3, fill=true, clip=true, length={10,40}, tool="round 2"})
local sb = brush{kind="round", width=1.3, point=0.2}
for _,xc in ipairs({570, 740, 910}) do
  local t = (575-640)/(455-640)
  local xb = xc + (900-xc)*t
  sb:load(P.seam, 0.6)
  sb:stroke({{xc,639},{xb,576}}, {pressure=0.45, shake=0.4})
end
sb:load(P.seam, 0.6)
sb:stroke({{xat(606)+4,606},{812,606}}, {pressure=0.4, shake=0.5})
sb:load(P.seam, 0.6)
sb:stroke({{830,606},{990,606}}, {pressure=0.4, shake=0.5})
-- the earlier seam at x=400 crosses the lit area near the base
sb:load(P.seam, 0.6)
local xc=400; local t=(575-640)/(455-640); 
sb:stroke({{xat(620),620},{xc+(900-xc)*t,576}}, {pressure=0.45, shake=0.4})

--@ chunk 94
work(rect(259,374,15,22), {hand="body", pile=P.green, angle=0, coverage=3, fill=true, clip=true, length={4,10}, tool="round 2"})

--@ chunk 95
print(wait(10*24*60), drying(266,385))

--@ chunk 96
print(wait(10*24*60), drying(266,385))

--@ chunk 97
local br = brush{kind="round", width=2.4}
br:load(P.letter, 0.9)
br:stroke({{266,377},{266,393},{275,393}}, {pressure=0.8, shake=0.3})
br:load(P.letter, 0.9)
br:stroke({{266,378},{266,392}}, {pressure=0.8, shake=0.2})

--@ chunk 98
local function keys(t) local r={} for k,v in pairs(t) do r[#r+1]=tostring(k)..":"..type(v) end return table.concat(r,", ") end
print("G", keys(G)) print("L", keys(L)) print("P", keys(P)) print("WIN", keys(WIN)) print("FONT", keys(FONT)) print("FIG", keys(FIG)) print("pen", keys(pen))
print(drying(350,385), drying(300,470), drying(100,600))

--@ chunk 99
print(palette())

--@ chunk 100
local band = rect(0,368,690,34) - G.fshadow
work(band, {hand="body", pile=P.green, angle=0, coverage=3, fill=true, clip=true, length={30,70}, tool="filbert 6"})

--@ chunk 101
print(wait(8*24*60), drying(265,385), drying(400,385))

--@ chunk 102
print(wait(8*24*60), drying(265,385), drying(400,385))

--@ chunk 103
local band = rect(0,368,690,34) - G.fshadow
work(band, {hand="body", pile=P.green, angle=0, coverage=3, fill=true, clip=true, length={30,70}, tool="filbert 6"})
print(wait(16*24*60), drying(265,385), drying(400,385), drying(600,395))

--@ chunk 104
FONT.E = {{{1,0},{0,0},{0,1},{1,1}},{{0,0.5},{0.8,0.5}}}
FONT.L = {{{0,0},{0,1},{1,1}}}
LW = {L=9,U=11,N=11.5,C=11,H=11,E=9,O=12.5,T=11}
LG = {3.5,4.5,4.5,4.5,4.5,4,4.5,4.5,3,1.5,3}
function layout(txt, cx, s)
  local tot = 0
  for i=1,#txt do tot = tot + LW[txt:sub(i,i)]*s if i<#txt then tot = tot + LG[i]*s end end
  local xs, x = {}, cx - tot/2
  for i=1,#txt do xs[i]=x x = x + LW[txt:sub(i,i)]*s + (LG[i] or 0)*s end
  return xs, tot
end
local xs, tot = layout("LUNCHEONETTE", 360, 1.15)
local t = {} for i,x in ipairs(xs) do t[#t+1]=string.format("%.1f",x) end
print(tot, table.concat(t," "))

--@ chunk 105
local txt = "LUNCHEONETTE"
local s = 1.15
local xs = layout(txt, 360, s)
local br = brush{kind="round", width=2.3}
local y0, h = 375.5, 19
for i=1,#txt do
  local ch = txt:sub(i,i)
  local w = LW[ch]*s
  for _,st in ipairs(FONT[ch]) do
    local pts = {}
    for _,q in ipairs(st) do pts[#pts+1] = {xs[i] + q[1]*w, y0 + q[2]*h} end
    br:load(P.letter, 0.7)
    br:stroke(pts, {pressure=0.75, shake=0.15})
  end
end

--@ chunk 106
P.curt2 = pile{{"lead white",3},{"yellow ochre",0.45},{"red earth",0.07},{"bone black",0.035},{"cadmium yellow",0.04}, medium=0.08, name="curtain behind glass"}
local m = CURT * rect(555,470,120,80)
work(m, {hand="body", pile=P.curt2, angle=1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 4"})

--@ chunk 107
P.curt3 = pile{{"lead white",3},{"yellow ochre",0.6},{"red earth",0.1},{"bone black",0.07},{"cadmium yellow",0.04}, medium=0.08, name="curtain deeper"}
local m = CURT * rect(555,470,120,80)
work(m, {hand="body", pile=P.curt3, angle=1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 4"})

--@ chunk 108
local m = (CURT * rect(28,470,395,80)) - G.fshadow
work(m, {hand="body", pile=P.curt3, angle=1.5708, coverage=3, fill=true, clip=true, length={10,30}, tool="filbert 4"})

--@ chunk 109
print(wait(5*24*60), drying(600,510), drying(380,500))

--@ chunk 110
print(wait(6*24*60), drying(600,510), drying(380,500), drying(600,470))

--@ chunk 111
P.urn = pile{{"lead white",1},{"bone black",0.4},{"burnt sienna",0.15},{"yellow ochre",0.15},{"ultramarine blue",0.05}, medium=0.08, name="urn metal"}
P.urnHi = pile{{"lead white",3},{"yellow ochre",0.25},{"bone black",0.08}, medium=0.08, name="urn hi"}
P.brass = pile{{"yellow ochre",1},{"burnt sienna",0.5},{"lead white",0.4},{"bone black",0.1}, medium=0.08, name="brass"}
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={3,9}, tool=t or "round 1.5"}) end
URNS = {}
for _,cx in ipairs({588, 643}) do
  local u = poly({{cx-6,481},{cx-6.5,462},{cx-4.5,457},{cx,455.5},{cx+4.5,457},{cx+6.5,462},{cx+6,481}}, true)
  URNS[#URNS+1] = u
  f(u, P.urn)
  f(u * rect(cx-4.5,450,2.5,32), P.urnHi, "round 1")
  f(rect(cx-1,452.5,2,3.5), P.urn, "round 1")
end
local rb = brush{kind="round", width=1.4}
rb:load(P.brass, 0.7)
rb:stroke({{557,480.5},{670,480.5}}, {pressure=0.6, shake=0.1})

--@ chunk 112
print(wait(10*24*60), drying(588,470), drying(600,480.5))

--@ chunk 113
P.urnD = pile{{"lead white",0.7},{"bone black",0.5},{"burnt sienna",0.2},{"yellow ochre",0.12},{"ultramarine blue",0.08}, medium=0.08, name="urn dark"}
P.urnM = mix{{P.urn,0.5},{P.urnHi,0.5}, name="urn mid"}
P.rod = pile{{"yellow ochre",0.6},{"burnt sienna",0.4},{"bone black",0.3},{"lead white",0.3}, medium=0.08, name="rod"}
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={3,9}, tool=t or "round 1.5"}) end
for _,cx in ipairs({588, 643}) do
  local u = rect(cx-7,458,14,22.5) + poly({{cx-7,458.5},{cx-5.5,455},{cx+5.5,455},{cx+7,458.5}}) + ellipse(cx,455.5,5,3)
  f(u, P.urn)
  f(rect(cx-5,450,2.2,31) * u, P.urnM, "round 1")
  f(rect(cx+3,450,4.5,31) * u, P.urnD, "round 1")
  f(rect(cx-1,451,2,3), P.urnD, "round 1")
  f(rect(cx-7,467,14,1.4), P.urnD, "round 1", 0)
end
f(rect(557,479.6,113,1.8), P.rod, "round 1", 0)

--@ chunk 114
local m = rect(468.5,474,5,18) - FIG.all:grow(0.6)
work(m, {hand="body", pile=P.glass, angle=1.5708, coverage=3, fill=true, clip=true, length={3,8}, tool="round 1.2"})

--@ chunk 115
work(rect(470,476,3.0,13), {hand="body", pile=P.glass, angle=1.5708, coverage=3, fill=true, clip=true, length={3,8}, tool="round 1"})

--@ chunk 116
P.gshadD = pile{{"lead white",1.1},{"ultramarine blue",0.32},{"burnt sienna",0.25},{"yellow ochre",0.2},{"bone black",0.12}, medium=0.08, name="ground shade deep"}
P.gshadW = pile{{"lead white",1.4},{"ultramarine blue",0.22},{"burnt sienna",0.2},{"yellow ochre",0.35},{"red earth",0.08},{"bone black",0.07}, medium=0.08, name="ground shade warm"}

--@ chunk 117
local o = {hand="body", angle=0, coverage=3, fill=true, clip=true, length={8,20}, tool="filbert 4"}
o.pile=P.gshadD work(rect(5,612,30,22), o)
o.pile=P.gshadW work(rect(35,612,30,22), o)

--@ chunk 118
P.gshadMid = mix{{P.gshad,0.55},{P.gshadW,0.45}, name="ground shade mid"}
local m = rect(0,575,1000,65) * G.wshadow2
work(m, {hand="body", piles={{P.gshadD, function(x,y) return 1-smoothstep(575,640,y) end},{P.gshadMid, function(x,y) return smoothstep(575,640,y) end}},
  angle=0, coverage=3, fill=true, clip=true, length={30,80}, tool="filbert 8"})

--@ chunk 119
print(wait(8*24*60), drying(100,590), drying(300,630))

--@ chunk 120
P.seamS2 = mix{{P.gshadD,0.6},{P.seamS,0.4}, name="seam shade 2"}
local sb = brush{kind="round", width=1.2, point=0.2}
local function xat(y) return 470 - (y-575)/0.279 end
local t = (575-640)/(455-640)
for _,xc in ipairs({60, 230}) do
  local xb = xc + (900-xc)*t
  sb:load(P.seamS2, 0.6)
  sb:stroke({{xc,639},{xb,576}}, {pressure=0.4, shake=0.4})
end
-- x=400 seam: only shaded portion (from curb up to where it crosses the edge)
local xc=400; local xb = xc+(900-xc)*t
-- param y where seam meets shadow edge
local function sx(y) return xc + (xb-xc)*(639-y)/(639-576) end
local yy = 639
for y=639,576,-0.5 do if sx(y) >= xat(y)-2 then yy=y break end end
sb:load(P.seamS2, 0.6)
sb:stroke({{xc,639},{sx(yy),yy}}, {pressure=0.4, shake=0.4})
sb:load(P.seamS2, 0.6)
sb:stroke({{5,606},{xat(606)-3,606}}, {pressure=0.35, shake=0.5})
print(yy, sx(yy))

--@ chunk 121
P.rodS = pile{{"burnt sienna",0.4},{"bone black",0.4},{"yellow ochre",0.2},{"lead white",0.15}, medium=0.08, name="rod shade"}
P.glassD = pile{{"lead white",2},{"ultramarine blue",0.1},{"yellow ochre",0.15},{"bone black",0.06}, medium=0.08, name="dome glass"}
local function f(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 0, coverage=3, fill=true, clip=true, length={3,9}, tool=t or "round 1"}) end
local xs = 330 / 0.5213 -- placeholder
local function xsh(y) return (y-330)/0.5213 end
f(rect(32,479.6,xsh(480)-32,1.8), P.rodS)
f(rect(xsh(480),479.6,418-xsh(480),1.8), P.rod)
-- cake stand with glass dome in lit part, sitting on the counter back
local cx = 372
f(rect(cx-9,476,18,1.6), P.urnD)            -- plate
f(rect(cx-1,477.5,2,3), P.urnD)              -- stem
local dome = ellipse(cx,476,8.5,12) * rect(cx-10,462,20,14)
f(dome, P.glassD)
f(dome * rect(cx+3,460,8,20), P.urnM)
f(ellipse(cx,470.5,5.5,4)*rect(cx-6,469,12,6), P.dress)  -- cake inside, pinkish
f(rect(cx-1,461,2,2), P.urnD)

--@ chunk 122
print(P.skytop, P.skymid, P.skyhz)
local function yv(y0, x) return y0 + (x-690)*(455-y0)/210 end
print(yv(64,745))

--@ chunk 123
SKY = (G.sky + rect(880,300,120,30)) - TOWER:grow(0.8) - rect(916,328,66,10) - rect(0,58,693,15)
  - poly({{687,58},{748,160},{748,185},{687,80}}) - POST:grow(0.8) - GLOBE:grow(0.8)
  - poly({{812,170},{830,170},{830,179},{812,179}}) - rect(816,194,10,10)
for _,p in ipairs({{500,30},{500,66},{720,110},{745,170},{821,186},{821,300},{947,250},{940,320},{990,320},{860,295},{760,290}}) do
  io = nil
  print(p[1],p[2], string.format("%.2f", SKY:at(p[1],p[2])))
end

--@ chunk 124
work(SKY, {hand="broad", piles={
   {P.skytop, function(x,y) return 1-smoothstep(0,260,y) end},
   {P.skymid, function(x,y) local t=smoothstep(0,260,y) return t*(1-smoothstep(200,330,y)) end},
   {P.skyhz, function(x,y) return smoothstep(200,330,y) end}},
  angle=0.0, angle_jitter=0.03, coverage=3, fill=true, clip=true})

--@ chunk 125
local m = poly({{686,57},{749,159},{749,167},{745.3,166.4},{690.3,63.8},{686,63.8}}) - rect(0,58,690.3,15)
work(m, {hand="body", tool="round 2", piles={
   {P.skytop, function(x,y) return 1-smoothstep(0,260,y) end},
   {P.skymid, function(x,y) local t=smoothstep(0,260,y) return t*(1-smoothstep(200,330,y)) end}},
  angle=1.08, coverage=3, fill=true, clip=true, length={6,16}})

--@ chunk 126
print(drying(446,200), drying(440,178))
work(rect(403,171,74,61), {hand="body", pile=P.glass, angle=1.5708, coverage=3, fill=true, clip=true, length={6,16}, tool="round 2"})

--@ chunk 127
print(wait(12*24*60), drying(440,200), drying(470,225))

--@ chunk 128
print(wait(6*24*60), drying(440,200), drying(470,225), drying(410,180))

--@ chunk 129
W2 = {}
W2.hair = poly({{440,181},{443,177.5},{448,177},{452,179},{453.5,184},{453.5,192},{455,198},{450,199},{450,188},{442,188},{441.5,196},{438.5,198},{439,190}}, true)
W2.face = ellipse(445.8,188,4.2,5.6)
W2.neck = rect(443.6,192,4.6,6)
W2.torso = poly({{439,196.5},{452,196.5},{459,199},{462,204},{463.5,214},{464,223},{428,223},{428.5,214},{430,204},{433,199}}, true)
W2.arms = poly({{424,221},{445,222.5},{467,220.5},{469,224},{468,231},{423,231},{422,225}}, true)
W2.all = W2.hair + W2.face + W2.neck + W2.torso + W2.arms
print(W2.all:area())

--@ chunk 130
function wf(m,p,t,a) work(m, {hand="body", pile=p, angle=a or 1.5708, coverage=3, fill=true, clip=true, length={2,7}, tool=t or "round 1.2"}) end
function paintWoman()
  local LT = rect(380,150,64,100)
  local MID = rect(444,150,6,100)
  local SH = rect(450,150,40,100)
  wf(W2.torso * LT, P.dress)
  wf(W2.torso * MID, P.dressM)
  wf(W2.torso * SH, P.dressS2)
  wf(W2.arms * rect(380,150,72,100), P.skin, "round 1.2", 0)
  wf(W2.arms * rect(452,150,40,100), P.skinS, "round 1.2", 0)
  wf(W2.neck, P.skinS, "round 1")
  wf(W2.face * rect(380,150,67,100), P.skin, "round 1")
  wf(W2.face * rect(447,150,20,100), P.skinS, "round 1")
  wf(W2.hair - W2.face, P.hairW, "round 1")
end
paintWoman()

--@ chunk 131
print(wait(14*24*60), drying(440,210), drying(446,186), drying(460,226))

--@ chunk 132
W3 = {}
W3.torso = poly({{440,196.5},{452,196.5},{457.5,199.5},{458,207},{456,221},{436,221},{434,207},{434.5,199.5}}, true)
W3.uarmL = poly({{434.5,199},{438,202},{435,214},{431,225},{425,226},{427,214},{430,203}}, true)
W3.uarmR = poly({{457.5,199},{462,203},{465,214},{467,226},{461,225},{457,214},{455,202}}, true)
W3.farmR = poly({{466,224},{467.5,228.5},{448,230},{437,228.5},{437,225},{449,224.5}}, true)  -- right forearm lying across to left
W3.farmL = poly({{424,224.5},{440,226},{455,224.5},{457,227.5},{455,230.5},{440,231},{424,230.5},{423,227}}, true) -- front, lit
W3.fig = W3.torso + W3.uarmL + W3.uarmR + W3.farmR + W3.farmL + W2.hair + W2.face + W2.neck
-- trim: dark over old shape outside new
wf((W2.all:grow(1) - W3.fig) * rect(403,171,74,61), P.glass, "round 1.2")

--@ chunk 133
local sleeve = rect(420,195,60,12)
wf(W3.uarmL * sleeve, P.dress, "round 1")
wf(W3.uarmR * sleeve, P.dressS2, "round 1")
wf(W3.uarmL - sleeve, P.skin, "round 1")
wf(W3.uarmR - sleeve, P.skinS, "round 1")
wf(W3.farmR, P.skinS, "round 1", 0)
wf(W3.farmL * rect(400,200,52,40), P.skin, "round 1", 0)
wf(W3.farmL * rect(452,200,30,40), P.skinS, "round 1", 0)

--@ chunk 134
print(wait(20*24*60), drying(440,228), drying(372,470))

--@ chunk 135
print(drying(600,200), drying(100,228))
local function f(m) work(m, {hand="body", pile=P.glass, angle=0, coverage=3, fill=true, clip=true, length={6,16}, tool="round 2"}) end
f(rect(63,222,70,10))
f(rect(233,225,70,7))
f(rect(573,147,70,37) + rect(573,189,70,43))
f(rect(445,262+44,0.1,0.1))

--@ chunk 136
print(wait(14*24*60), drying(600,200), drying(950,658))

--@ chunk 137
P.sig = pile{{"burnt sienna",0.6},{"red earth",0.4},{"bone black",0.3},{"lead white",0.1}, medium=0.12, name="signature"}
local br = brush{kind="round", width=1.1, point=0.4}
local x0, y0, w, h = 958, 652, 6, 8
local pts = {}
for _,q in ipairs(FONT.C[1]) do pts[#pts+1] = {x0+q[1]*w, y0+q[2]*h} end
br:load(P.sig, 0.6)
br:stroke(pts, {pressure=0.6, shake=0.2})
br:load(P.sig, 0.5)
br:stroke({{967,659},{968,660}}, {pressure=0.6})
br:load(P.sig, 0.5)
br:stroke({{972,652},{972,660}}, {pressure=0.55})
br:load(P.sig, 0.5)
br:stroke({{972,660},{977,660}}, {pressure=0.55})

--@ chunk 138
print(wait(20*24*60))

--@ chunk 139
for k,v in pairs(WOM) do print("WOM",k,v) end
for k,v in pairs(W2) do print("W2",k,v) end
for k,v in pairs(W3) do print("W3",k,v) end
for k,v in pairs(FIG) do print("FIG",k,v) end
local names={} for k,v in pairs(P) do names[#names+1]=tostring(k) end print(table.concat(names,", "))

--@ chunk 140
local function bbox(m) local x0,y0,x1,y1=1e9,1e9,-1e9,-1e9
for y=150,250,0.5 do for x=400,490,0.5 do if m:at(x,y)>0.5 then x0=math.min(x0,x);x1=math.max(x1,x);y0=math.min(y0,y);y1=math.max(y1,y) end end end
return string.format("%.1f %.1f %.1f %.1f",x0,y0,x1,y1) end
for _,k in ipairs{"farmL","farmR","uarmL","uarmR","torso","fig"} do print(k,bbox(W3[k])) end
-- farm rows
for y=218,232,1 do local s="" for x=415,475,1 do s=s..(W3.farmL:at(x,y)>0.5 and "L" or (W3.farmR:at(x,y)>0.5 and "R" or ".")) end print(y,s) end
print(drying(440,225), drying(440,200))

--@ chunk 141
print(P.skin) print(P.skinS) print(P.dress) print(P.dressS) print(P.shirt) print(P.shirtS) print(P.glassD) print(P.glass)

--@ chunk 142
local function bbox(m,X0,X1,Y0,Y1) local x0,y0,x1,y1=1e9,1e9,-1e9,-1e9
for y=Y0,Y1,0.5 do for x=X0,X1,0.5 do if m:at(x,y)>0.5 then x0=math.min(x0,x);x1=math.max(x1,x);y0=math.min(y0,y);y1=math.max(y1,y) end end end
return string.format("%.1f %.1f %.1f %.1f",x0,y0,x1,y1) end
for _,k in ipairs{"apron","legs","torso","litside","all"} do print(k,bbox(FIG[k],440,540,440,580)) end
for y=540,556 do local s="" for x=470,516 do local a=FIG.apron:at(x,y)>0.5 or FIG.torso:at(x,y)>0.5; local l=FIG.legs:at(x,y)>0.5; s=s..(a and (FIG.litside:at(x,y)>0.5 and "A" or "a") or (l and "l" or ".")) end print(y,s) end

--@ chunk 143
local band = FIG.apron * rect(470, 542.5, 50, 7)
local lit = band * FIG.litside
local b = brush{kind="round", width=1.6}
b:load(P.shirt, 0.7)
for y=543,548.4,0.9 do b:stroke({{477.5,y},{496.5,y}}, {pressure=0.6, clip=lit}) if b:fullness()<0.3 then b:load(P.shirt,0.7) end end
print(drying(485,546))

--@ chunk 144
print(P.shirtM) print(P.shirtS2)

--@ chunk 145
local band = FIG.apron * rect(470, 542.5, 50, 7) * (-FIG.litside)
local b = brush{kind="round", width=1.6}
b:load(P.shirtM, 0.7)
for y=543,548.4,0.9 do b:stroke({{496,y},{510,y}}, {pressure=0.6, clip=band}) end

--@ chunk 146
P.coatS = mix{{P.shirtM,0.55},{P.shirtS,0.45}, name="coatS"}
local band = FIG.apron * rect(501, 542.5, 12, 7)
local b = brush{kind="round", width=1.6}
b:load(P.coatS, 0.7)
for y=543,548.4,0.9 do b:stroke({{501,y},{510,y}}, {pressure=0.6, clip=band}) end

--@ chunk 147
wait(60*24*12)
local b = brush{kind="round", width=0.9, point=0.6}
b:load(P.skinS, 0.5)
b:stroke({{437,225.6},{444,225.4},{450,224.9},{456,224.8}}, {pressure={0.35,0.25}, clip=W3.farmR:grow(0.6)})
print(drying(485,546))

--@ chunk 148
print(wait(60*24*20)) print(drying(485,546), drying(445,225))

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
