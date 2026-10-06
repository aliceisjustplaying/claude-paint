-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=450, aspect=1.4, linen={15,13}, seed=1823, ground={{pile={{"red earth",2},{"yellow ochre",2},{"lead white",1}}, um=120, apply="knife", texture=0.3},{pile={{"lead white",6},{"yellow ochre",0.3},{"raw umber",0.1}}, um=60, apply="brush"}}}
print(W,H)

--@ chunk 2
HZ = 470
mound = {{0,478},{60,466},{130,446},{210,425},{300,414},{390,418},{470,434},{540,455},{610,471},{700,476}}
h = pencil("2H")
-- horizon line, ruled
h:rule({560,HZ},{1000,HZ},{pressure=0.25})
-- mound
h:sketch(mound,{pressure=0.3, passes=2})
-- distant wood line
h:sketch({{560,466},{620,462},{700,460},{760,463},{830,458},{900,461},{1000,457}},{pressure=0.2,passes=2})
-- church
h:line({{786,462},{786,432},{796,432},{796,462}},{pressure=0.3,smooth=false})
h:line({{786,432},{791,412},{796,432}},{pressure=0.3,smooth=false})
h:line({{796,446},{820,446},{820,462}},{pressure=0.25,smooth=false})
-- dolmen
h:line({{282,402},{300,392},{350,386},{400,388},{418,396},{414,406},{360,404},{300,410},{282,402}},{pressure=0.35})
for _,u in ipairs({{298,305,322,436},{344,352,368,428},{388,396,408,428}}) do
  h:line({{u[1],408},{u[2]-2,u[4]},{u[3],u[4]-2},{u[3]-2,406}},{pressure=0.35,smooth=false})
end
-- oak trunk
h:sketch({{176,436},{182,380},{186,330},{180,290},{176,260}},{pressure=0.35,passes=2})
h:sketch({{208,432},{204,380},{206,330},{214,295},{222,270}},{pressure=0.35,passes=2})
-- main limbs
h:sketch({{182,290},{150,250},{120,215},{100,170}},{pressure=0.3})
h:sketch({{195,285},{200,230},{190,170},{200,110},{210,70}},{pressure=0.3})
h:sketch({{214,290},{260,250},{300,210},{330,165},{360,140}},{pressure=0.3})
h:sketch({{210,300},{270,295},{330,280},{380,280}},{pressure=0.3})
h:sketch({{178,310},{130,300},{80,290},{50,270}},{pressure=0.3})
-- moon
h:line({{752,142},{760,156},{770,160}},{pressure=0.25})
-- figure
h:line({{602,556},{600,535},{604,522},{610,522},{612,535},{610,556}},{pressure=0.3})
-- spruces right
for _,s in ipairs({{880,494,38},{905,498,52},{930,496,34},{952,500,24}}) do
  h:line({{s[1]-s[3]*0.25,s[2]},{s[1],s[2]-s[3]},{s[1]+s[3]*0.25,s[2]}},{pressure=0.25,smooth=false})
end
-- fence posts foreground left
h:line({{88,640},{94,572}},{pressure=0.3}); h:line({{150,622},{152,566}},{pressure=0.3}); h:line({{215,606},{214,560}},{pressure=0.3})
-- foreground rise
h:sketch({{0,640},{150,600},{350,585},{600,600},{800,620},{1000,610}},{pressure=0.2,passes=1})

--@ chunk 3
skyline = {{0,478},{60,466},{130,446},{210,425},{300,414},{390,418},{470,434},{540,455},{610,471},{700,474},{1000,474}}
sky = above(skyline)
skyA = pile{{"lead white",3},{"pale smalt",3},{"smalt",1},{"bone black",0.12}, medium=0.15}
skyB = pile{{"lead white",5},{"pale smalt",2},{"smalt",0.3},{"raw umber",0.08}, medium=0.15}
skyC = pile{{"lead white",7},{"yellow ochre",0.35},{"pale smalt",0.5},{"vermilion",0.04}, medium=0.15}
skyD = pile{{"lead white",7},{"yellow ochre",0.9},{"vermilion",0.14}, medium=0.15}
local bands = {{skyA,-20,170},{skyB,140,300},{skyC,270,400},{skyD,370,500}}
for i,b in ipairs(bands) do
  local m = sky * rect(-20, b[2], 1040, b[3]-b[2])
  work(m, {hand="broad", pile=b[1], angle=0, coverage=2, fill=true, angle_jitter=0.05, pressure={0.5,0.8}})
end

--@ chunk 4
blend(sky, {angle=0}); blend(sky, {angle=0.03})

--@ chunk 5
skyTop = pile{{"lead white",2},{"pale smalt",2},{"smalt",1.5},{"bone black",0.15},{"raw umber",0.05}, medium=0.15}
work(rect(-20,-20,1040,125), {hand="broad", pile=skyTop, angle=0, coverage=1.6, fill=true, angle_jitter=0.05, pressure={0.4,0.7}})
blend(rect(0,0,1000,250), {angle=0}); blend(rect(0,0,1000,200), {angle=0.02})

--@ chunk 6
land = -sky
snowFar  = pile{{"lead white",6},{"pale smalt",1},{"yellow ochre",0.15},{"vermilion",0.03}, medium=0.12}
snowMid  = pile{{"lead white",5},{"pale smalt",1.6},{"smalt",0.3},{"raw umber",0.1}, medium=0.12}
snowNear = pile{{"lead white",4},{"pale smalt",2},{"smalt",0.6},{"raw umber",0.2},{"bone black",0.04}, medium=0.12}
moundP   = pile{{"lead white",4.5},{"pale smalt",1.8},{"smalt",0.4},{"raw umber",0.15},{"vermilion",0.02}, medium=0.12}
local moundM = land * below({{0,470},{100,470},{200,470},{600,470},{700,480}}):map(function(v) return 1-v end)
work(land * rect(-20,460,1040,80), {hand="broad", pile=snowFar, angle=0, coverage=2, fill=true, angle_jitter=0.03, pressure={0.4,0.7}})
work(land * rect(-20,520,1040,90), {hand="broad", pile=snowMid, angle=0, coverage=2, fill=true, angle_jitter=0.04, pressure={0.4,0.7}})
work(land * rect(-20,590,1040,140), {hand="broad", pile=snowNear, angle=0.02, coverage=2, fill=true, angle_jitter=0.06, pressure={0.4,0.8}})
work(moundM, {hand="body", pile=moundP, angle=0.05, coverage=2, fill=true, angle_jitter=0.1, pressure={0.4,0.7}})

--@ chunk 7
local L = land
work(L * rect(-20,465,1040,70), {hand="body", pile=snowFar, angle=0, coverage=2.5, fill=true, angle_jitter=0.04, pressure={0.5,0.8}, length={40,90}, clip=L})
work(L * rect(-20,530,1040,80), {hand="body", pile=snowMid, angle=0, coverage=2.5, fill=true, angle_jitter=0.05, pressure={0.5,0.8}, length={40,90}})
work(L * rect(-20,600,1040,130), {hand="body", pile=snowNear, angle=0.02, coverage=2.5, fill=true, angle_jitter=0.06, pressure={0.5,0.8}, length={40,100}})
local moundM = L * rect(0,400,640,72)
work(moundM, {hand="body", pile=moundP, angle=0.0, coverage=2.5, fill=true, angle_jitter=0.08, pressure={0.5,0.8}, length={30,70}, clip=L})

--@ chunk 8
blend(land, {angle=0}); blend(land * rect(0,380,1000,120), {angle=0.02})

--@ chunk 9
snowShade = pile{{"lead white",3},{"pale smalt",2},{"smalt",0.6},{"raw umber",0.25},{"vermilion",0.03}, medium=0.15}
snowLit = pile{{"lead white",7},{"yellow ochre",0.25},{"vermilion",0.05},{"pale smalt",0.2}, medium=0.12}
local drift = {{-10,628},{150,606},{320,598},{520,612},{720,630},{900,628},{1010,618}}
local d1 = below(drift) * above({{-10,668},{150,640},{320,632},{520,648},{720,672},{900,668},{1010,656}})
work(d1, {hand="body", pile=snowShade, angle=0, coverage=1.5, pressure={0.4,0.6}, length={40,90}, angle_jitter=0.06})
work(below({{-10,690},{400,680},{1010,660}}), {hand="body", pile=snowShade, angle=0, coverage=1.2, pressure={0.4,0.6}, length={40,90}})
-- lit crest of the drift
work(ribbon(drift, 8), {hand="body", pile=snowLit, angle=0, coverage=1.5, pressure={0.4,0.6}, length={30,80}, angle_jitter=0.03})
-- faint shade along the foot of the mound
work(land * rect(40,468,540,22), {hand="body", pile=snowShade, angle=0, coverage=0.8, pressure={0.3,0.5}, length={40,80}})
-- lit far snow near the horizon on the right
work(land * rect(600,472,400,14), {hand="body", pile=snowLit, angle=0, coverage=1.2, pressure={0.4,0.6}, length={40,90}})
blend(land * rect(0,590,1000,130), {angle=0})
blend(land * rect(0,465,1000,40), {angle=0})

--@ chunk 10
print(wait(30*60)); for _,p in ipairs({{500,100},{500,420},{300,440},{500,560},{500,680}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 11
print(wait(14*60)); for _,p in ipairs({{500,100},{500,420},{300,440},{500,560},{500,680},{100,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 12
farWood = pile{{"lead white",3},{"pale smalt",2},{"smalt",0.5},{"raw umber",0.35},{"vermilion",0.06}, medium=0.2}
local n = noise{seed=7, period=14, octaves=3}
local n2 = noise{seed=8, period=60, octaves=2}
local topf = function(x)
  local base = 463 - 4*math.max(0, n2(x,0)) 
  if x < 600 then base = base + (600-x)*0.08 end
  return base - 3.5*math.abs(n(x,0))
end
local pts = {}
for x = 560, 1010, 3 do pts[#pts+1] = {x, topf(x)} end
woodM = above({{540,475},{1010,475}}) * below(pts) * rect(560,440,450,40)
work(woodM, {hand="detail", pile=farWood, angle=0, coverage=3, pressure={0.5,0.8}, load=0.8})
stipple(woodM:rim(4, 2) , {pile=farWood, width=2, coverage=0.8, pressure={0.3,0.6}, feather=0.5, cluster=0.4})

--@ chunk 13
farWood2 = pile{{"lead white",2.5},{"pale smalt",2},{"smalt",0.6},{"raw umber",0.45},{"vermilion",0.06}, medium=0.2}
local clumps = {{640,700,449},{712,760,456},{830,905,447},{930,975,455},{600,630,458}}
local M = nil
for i,c in ipairs(clumps) do
  local x0,x1,top = c[1],c[2],c[3]
  local pts = {}
  local nn = noise{seed=20+i, period=6, octaves=2}
  for x = x0, x1, 2 do
    local t = (x-x0)/(x1-x0)
    local hgt = (472-top) * math.sin(math.pi*t)^0.5
    pts[#pts+1] = {x, 472 - hgt - 2.5*math.abs(nn(x,0))}
  end
  local m = below(pts) * rect(x0,430,x1-x0,46)
  M = M and (M + m) or m
end
woodClumps = M
work(M, {hand="detail", pile=farWood2, angle=-1.4, coverage=2.5, pressure={0.4,0.7}, load=0.7, length={3,8}})
stipple(M:rim(3,1), {pile=farWood2, width=1.6, coverage=1.2, pressure={0.25,0.5}, feather=0.6, cluster=0.5})

--@ chunk 14
local groups = {{595,640,8,14},{628,775,14,26},{820,915,14,28},{922,1005,10,22}}
local M = nil
local tops = {}
for gi,g in ipairs(groups) do
  local x = g[1]
  while x < g[2] do
    local w = rand(5, 14)
    local t = (x-g[1])/(g[2]-g[1])
    local h = rand(g[3], g[4]) * (0.55 + 0.45*math.sin(math.pi*t))
    local cx = x + w/2
    local e = ellipse(cx, 472 - h*0.55, w*0.62, h*0.55)
    M = M and (M + e) or e
    tops[#tops+1] = {cx, 472 - h, w}
    x = x + w*rand(0.45, 0.9)
  end
end
M = (M * rect(560,400,450,72)):roughen(1.5, 5, 31)
woodsM = M + woodClumps
work(M, {hand="detail", pile=farWood2, angle=-math.pi/2, angle_jitter=0.4, coverage=3, pressure={0.4,0.7}, load=0.7, length={3,7}, fill=true})
-- airy twig tops above each crown: short upward strokes
local r = brush{kind="round", width=1.2, point=1}
r:load(farWood, 0.5)
for i,t in ipairs(tops) do
  for k=1,3 do
    local x = t[1] + rand(-t[3]*0.4, t[3]*0.4)
    local y = t[2] + rand(1,4)
    r:stroke({{x,y+3},{x+rand(-1.5,1.5),y-rand(2,5)}}, {pressure={0.35,0}})
    if k==3 then r:load(farWood, 0.5) end
  end
end

--@ chunk 15
churchP = pile{{"lead white",2},{"pale smalt",2},{"smalt",0.7},{"raw umber",0.6},{"vermilion",0.05}, medium=0.15}
local tower = poly({{785,472},{785,436},{797,436},{797,472}})
local spire = poly({{784.5,437},{791,410},{797.5,437}})
local nave = poly({{797,472},{797,452},{808,443},{824,443},{824,472}})
churchM = tower + spire + nave
work(churchM, {hand="detail", pile=churchP, angle=-math.pi/2, coverage=4, pressure={0.6,0.9}, load=1, fill=true})
-- snow on the nave roof, lit from the west glow
local roof = poly({{797.5,452},{808,443.5},{824,443.5},{824,447},{809,447},{799,455}})
work(roof, {hand="detail", pile=snowLit, angle=0, coverage=3, pressure={0.5,0.8}, load=0.8, tool={kind="round", width=1.2}})
-- snowfield strip in front of the woods
local strip = rect(555,471,455,8)
work(strip, {hand="detail", pile=snowFar, angle=0, coverage=3, pressure={0.5,0.8}, load=0.8, length={8,20}, fill=true})

--@ chunk 16
-- the oak: a skeleton of designed limbs, grown further by rule
oak = {thick={}, thin={}, dead={}, snow={}}
local function addseg(list, pts, ws) list[#list+1] = {pts=pts, ws=ws} end
function growLimb(x, y, ang, len, w, depth, dead, up)
  local pts, ws = {{x,y}}, {w}
  local nseg = math.max(2, math.floor(len / 14))
  local sl = len / nseg
  local cw = w
  for i = 1, nseg do
    ang = ang + randn(0, 0.22)
    -- mild upward tendency, stronger on upright limbs
    ang = ang + (-math.pi/2 - ang) * (up or 0.06)
    x = x + math.cos(ang) * sl * rand(0.8, 1.2)
    y = y + math.sin(ang) * sl * rand(0.8, 1.2)
    local wc = nil
    if depth > 0 and i < nseg and math.random() < 0.75 then
      wc = cw * rand(0.45, 0.72)
      local side = (math.random() < 0.5) and -1 or 1
      local ca = ang + side * rand(0.45, 1.0)
      if not dead then
        growLimb(x, y, ca, len * rand(0.45, 0.7), wc, depth - 1, false, up)
      end
      cw = math.max(cw * 0.6, math.sqrt(math.max(cw*cw - wc*wc, 0)))
    else
      cw = cw * 0.9
    end
    pts[#pts+1] = {x, y}; ws[#ws+1] = math.max(cw, 0.4)
  end
  if dead then
    addseg(oak.dead, pts, ws)
  elseif w >= 2.2 then
    addseg(oak.thick, pts, ws)
    if depth > 0 then
      -- twig brush at the tip
      for k = 1, 3 do growLimb(x, y, ang + randn(0, 0.6), len * rand(0.35, 0.6), cw * 0.7, depth - 1, false, up) end
    end
  else
    addseg(oak.thin, pts, ws)
    if depth > 0 then
      for k = 1, 2 do growLimb(x, y, ang + randn(0, 0.55), len * rand(0.45, 0.7), cw * 0.75, depth - 1, false, up) end
    end
  end
end
-- trunk and designed limbs
oakTrunk = {pts={{190,446},{191,425},{193,400},{192,370},{194,340},{197,312}}, ws={46,32,28,27,26,29}}
local L = {
  -- x, y, angle, length, width, depth, dead, up
  {186,318, math.pi+0.25, 150, 12, 4, false, 0.02},  -- low left sweep
  {189,305, -2.05, 170, 13, 4, false, 0.05},          -- left up
  {205,304, -1.0, 170, 13, 4, false, 0.05},           -- right up
  {209,318, -0.18, 180, 11, 4, false, 0.0},           -- right long horizontal
  {195,300, -1.62, 90, 15, 3, false, 0.1},            -- central leader (living part)
}
for _,l in ipairs(L) do growLimb(l[1],l[2],l[3],l[4],l[5],l[6],l[7],l[8]) end
-- dead stag-head limbs, bare and broken
addseg(oak.dead, {{197,215},{200,180},{194,150},{199,120},{208,92},{206,74}}, {10,9,8,6.5,5,4})
addseg(oak.dead, {{199,165},{222,140},{236,116},{233,104}}, {5,4,3.4,3})
addseg(oak.dead, {{196,128},{178,108},{170,92}}, {4,3.2,2.6})
addseg(oak.dead, {{262,238},{272,200},{268,168},{276,140}}, {7,6,5,4})
addseg(oak.dead, {{184,338},{165,332},{152,322}}, {9,8,7.5})
print(#oak.thick, #oak.thin, #oak.dead)

--@ chunk 17
local x0,x1,y0,y1=1e9,-1e9,1e9,-1e9
local hist={}
for _,s in ipairs(oak.thin) do for _,p in ipairs(s.pts) do x0=math.min(x0,p[1]);x1=math.max(x1,p[1]);y0=math.min(y0,p[2]);y1=math.max(y1,p[2]) 
local bx=math.floor(p[1]/50); local by=math.floor(p[2]/50); hist[by]=hist[by] or {}; hist[by][bx]=(hist[by][bx] or 0)+1 end end
print(x0,x1,y0,y1)
for by=-2,9 do local row={} for bx=-3,10 do row[#row+1]=string.format("%4d",(hist[by] or {})[bx] or 0) end print(by*50, table.concat(row)) end

--@ chunk 18
oak = {thick={}, thin={}, dead={}}
local function env(x, y)
  local e = ((x-212)/190)^2 + ((y-192)/128)^2
  if y > 322 then e = math.max(e, 1 + (y-322)/20) end
  return e
end
oakEnv = env
local function addseg(list, pts, ws) list[#list+1] = {pts=pts, ws=ws} end
function growLimb(x, y, ang, len, w, depth, up)
  local pts, ws = {{x,y}}, {w}
  local nseg = math.max(2, math.floor(len / 12))
  local sl = len / nseg
  local cw = w
  for i = 1, nseg do
    ang = ang + randn(0, 0.24)
    ang = ang + (-math.pi/2 - ang) * up
    local nx = x + math.cos(ang) * sl * rand(0.8, 1.2)
    local ny = y + math.sin(ang) * sl * rand(0.8, 1.2)
    local e = env(nx, ny)
    if e > 1 then
      -- steer back toward the crown's centre
      local ta = math.atan(192 - y, 212 - x)
      local d = ta - ang
      while d > math.pi do d = d - 2*math.pi end
      while d < -math.pi do d = d + 2*math.pi end
      ang = ang + d * clamp((e-1)*2.5, 0.15, 0.7)
      nx = x + math.cos(ang) * sl * 0.7
      ny = y + math.sin(ang) * sl * 0.7
      if env(nx, ny) > 1.12 then break end
    end
    x, y = nx, ny
    if depth > 0 and i < nseg and math.random() < 0.7 then
      local wc = cw * rand(0.45, 0.72)
      local side = (math.random() < 0.5) and -1 or 1
      growLimb(x, y, ang + side * rand(0.45, 1.0), len * rand(0.45, 0.68), wc, depth - 1, up)
      cw = math.max(cw * 0.62, math.sqrt(math.max(cw*cw - wc*wc, 0)))
    else
      cw = cw * 0.9
    end
    pts[#pts+1] = {x, y}; ws[#ws+1] = math.max(cw, 0.4)
  end
  if #pts < 2 then return end
  if w >= 2.2 then addseg(oak.thick, pts, ws) else addseg(oak.thin, pts, ws) end
  if depth > 0 then
    local nk = (w >= 2.2) and 3 or 2
    for k = 1, nk do growLimb(x, y, ang + randn(0, 0.6), len * rand(0.35, 0.6), cw * 0.72, depth - 1, up) end
  end
end
local L = {
  {186,318, math.pi+0.3, 130, 12, 4, 0.03},
  {189,305, -2.1, 140, 13, 4, 0.06},
  {205,304, -1.0, 140, 13, 4, 0.06},
  {209,318, -0.2, 150, 11, 4, 0.02},
  {195,300, -1.62, 85, 15, 3, 0.1},
}
for _,l in ipairs(L) do growLimb(l[1],l[2],l[3],l[4],l[5],l[6],l[7]) end
addseg(oak.dead, {{197,222},{200,185},{194,152},{199,120},{208,92},{206,72}}, {10,9,8,6.5,5,4})
addseg(oak.dead, {{199,168},{222,142},{236,118},{233,104}}, {5,4,3.4,3})
addseg(oak.dead, {{196,130},{178,110},{170,94}}, {4,3.2,2.6})
addseg(oak.dead, {{262,238},{272,200},{268,168},{276,138}}, {7,6,5,4})
addseg(oak.dead, {{184,338},{165,332},{152,322}}, {9,8,7.5})
local x0,x1,y0,y1=1e9,-1e9,1e9,-1e9
local hist={}
for _,s in ipairs(oak.thin) do for _,p in ipairs(s.pts) do x0=math.min(x0,p[1]);x1=math.max(x1,p[1]);y0=math.min(y0,p[2]);y1=math.max(y1,p[2])
local bx=math.floor(p[1]/50); local by=math.floor(p[2]/50); hist[by]=hist[by] or {}; hist[by][bx]=(hist[by][bx] or 0)+1 end end
print(#oak.thick, #oak.thin, x0,x1,y0,y1)
for by=0,7 do local row={} for bx=0,9 do row[#row+1]=string.format("%4d",(hist[by] or {})[bx] or 0) end print(by*50, table.concat(row)) end

--@ chunk 19
barkDark = pile{{"raw umber",2},{"bone black",1.2},{"smalt",0.4},{"lead white",0.5}, medium=0.1}
deadWood = pile{{"lead white",1.6},{"raw umber",1},{"bone black",0.45},{"pale smalt",0.6}, medium=0.1}
local tm = ribbon(oakTrunk.pts, oakTrunk.ws)
-- root flare
tm = tm + poly({{160,452},{172,436},{182,420},{202,420},{212,436},{226,452}}, true)
local lm = nil
for _,s in ipairs(oak.thick) do local r = ribbon(s.pts, s.ws); lm = lm and (lm + r) or r end
oakThickM = (tm + lm):roughen(0.8, 6, 5)
local dm = nil
for _,s in ipairs(oak.dead) do local r = ribbon(s.pts, s.ws); dm = dm and (dm + r) or r end
oakDeadM = dm:roughen(0.6, 5, 6)
work(oakThickM, {hand="detail", pile=barkDark, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true, angle=-math.pi/2, angle_jitter=0.5})
work(oakDeadM, {hand="detail", pile=deadWood, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true, angle=-math.pi/2, angle_jitter=0.5})

--@ chunk 20
local n0 = #oak.thick
-- two great low limbs, elbowed, as an old field oak carries them
oak.thick[#oak.thick+1] = {pts={{182,352},{156,344},{128,333},{104,318},{88,302}}, ws={17,14,12,10,9}}
oak.thick[#oak.thick+1] = {pts={{208,348},{236,340},{266,330},{296,316},{322,300}}, ws={16,13,11,10,9}}
growLimb(88,302, -2.3, 90, 8, 3, 0.05)
growLimb(104,318, math.pi+0.1, 70, 6, 3, 0.02)
growLimb(322,300, -0.7, 90, 8, 3, 0.05)
growLimb(266,330, -0.05, 90, 6, 3, 0.02)
local m = ribbon({{190,450},{190,420},{192,390},{194,360},{197,336}}, {52,44,40,39,44})
m = m + poly({{150,456},{166,440},{176,420},{210,420},{220,440},{238,458}}, true)
for i = n0+1, #oak.thick do local s = oak.thick[i]; m = m + ribbon(s.pts, s.ws) end
m = m:roughen(1.0, 7, 9)
oakThickM = oakThickM + m
work(m, {hand="detail", pile=barkDark, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true, angle=-math.pi/2, angle_jitter=0.5})
print(#oak.thick, #oak.thin)

--@ chunk 21
twigP = pile{{"raw umber",2},{"bone black",0.9},{"lead white",1.0},{"pale smalt",0.5}, medium=0.15}
local b = brush{kind="round", width=3, point=1, stiffness=0.5}
b:load(twigP, 0.8)
local cnt = 0
for _,s in ipairs(oak.thin) do
  local w0 = s.ws[1]; local w1 = s.ws[#s.ws]
  local p0 = math.max(b:pressure_for(w0), 0.15)
  local p1 = math.max(b:pressure_for(w1) * 0.6, 0.02)
  b:stroke(s.pts, {pressure={p0, p1}, ramps={0.0, 0.5}})
  cnt = cnt + 1
  if cnt % 12 == 0 then b:load(twigP, 0.7) end
end
print(cnt)

--@ chunk 22
print(wait(20*60)); for _,p in ipairs({{193,400},{200,150},{120,200},{165,330}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 23
deadWood2 = pile{{"raw umber",1.2},{"bone black",0.8},{"lead white",1.1},{"pale smalt",0.6}, medium=0.1}
deadExt = {
  {pts={{197,222},{200,185},{194,152},{199,120},{208,92},{206,72},{211,52},{207,36},{213,22}}, ws={10.5,9.5,8.5,7,5.5,4.6,4,3.4,3}},
  {pts={{199,168},{222,142},{236,118},{233,104},{241,82},{251,66}}, ws={5.5,4.5,3.8,3.4,3,2.6}},
  {pts={{196,130},{178,110},{170,94},{160,72},{164,56}}, ws={4.5,3.6,3,2.6,2.2}},
  {pts={{262,238},{272,200},{268,168},{276,138},{284,110},{280,86},{289,68}}, ws={7.5,6.5,5.5,4.5,4,3.4,2.8}},
  {pts={{207,36},{198,24}}, ws={2,1.6}},
  {pts={{280,86},{268,72}}, ws={2.4,1.8}},
  {pts={{211,52},{226,40}}, ws={2.4,1.8}},
}
local dm = nil
for _,s in ipairs(deadExt) do local r = ribbon(s.pts, s.ws); dm = dm and (dm + r) or r end
oakDeadM2 = dm:roughen(0.6, 5, 16)
work(oakDeadM2, {hand="detail", pile=deadWood2, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=-math.pi/2, angle_jitter=0.4})
-- the pale stub on the bole becomes a short dark broken stub
local stub = ribbon({{186,338},{168,334},{156,327}}, {11,9,8.5}):roughen(0.6,5,17)
work(stub, {hand="detail", pile=barkDark, coverage=4, pressure={0.6,0.9}, load=1, fill=true})

--@ chunk 24
-- living bark wraps the foot of each dead limb
local joins = {
  {pts={{194,262},{195,238},{197,214},{198,200}}, ws={13,12,11,10}},
  {pts={{222,274},{240,258},{254,247},{263,232}}, ws={10,9,8.5,8}},
  {pts={{200,176},{203,168},{210,160}}, ws={7,6.5,6}},
}
local m = nil
for _,s in ipairs(joins) do local r = ribbon(s.pts, s.ws); m = m and (m + r) or r end
m = m + ellipse(152,323,5,5)
m = m:roughen(0.7,5,21)
work(m, {hand="detail", pile=barkDark, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=-math.pi/2, angle_jitter=0.4})
-- ragged bark edge where living wood gives out: a few tongues up the dead wood
local r = brush{kind="round", width=3, point=1}
r:load(barkDark, 0.7)
for _,t in ipairs({{193,202,191,186},{201,203,203,190},{258,236,262,222},{267,238,270,226},{209,162,214,154}}) do
  r:stroke({{t[1],t[2]},{t[3],t[4]}}, {pressure={0.7,0.05}})
end

--@ chunk 25
twigDark = pile{{"raw umber",2},{"bone black",1.2},{"lead white",0.45},{"pale smalt",0.3}, medium=0.1}
local b = brush{kind="round", width=2.5, point=1, stiffness=0.55}
b:load(twigDark, 0.7)
local cnt = 0
local function spurs(s, every)
  local pts, ws = s.pts, s.ws
  for i = 1, #pts-1 do
    local x0,y0 = pts[i][1], pts[i][2]
    local x1,y1 = pts[i+1][1], pts[i+1][2]
    local a = math.atan(y1-y0, x1-x0)
    local L = math.sqrt((x1-x0)^2 + (y1-y0)^2)
    local n = math.floor(L / every + math.random())
    for k = 1, n do
      local t = math.random()
      local x = lerp(x0,x1,t); local y = lerp(y0,y1,t)
      local w = lerp(ws[i], ws[i+1], t)
      local side = (math.random() < 0.5) and -1 or 1
      local sa = a + side * rand(0.5, 1.3)
      -- start at the branch edge
      x = x + math.cos(sa) * w * 0.4; y = y + math.sin(sa) * w * 0.4
      local l1 = rand(2.5, 6); local l2 = rand(2, 5)
      local mx, my = x + math.cos(sa)*l1, y + math.sin(sa)*l1
      local sa2 = sa + randn(0, 0.5)
      local ex, ey = mx + math.cos(sa2)*l2, my + math.sin(sa2)*l2
      local p0 = math.max(b:pressure_for(math.min(w*0.45, 1.6)), 0.2)
      b:stroke({{x,y},{mx,my},{ex,ey}}, {pressure={p0, 0.02}, ramps={0,0.5}})
      -- a small clustered fork at the tip, as oak buds bunch
      if math.random() < 0.45 then
        local sa3 = sa2 + (math.random()<0.5 and -1 or 1)*rand(0.4,0.9)
        b:stroke({{mx,my},{mx+math.cos(sa3)*l2*0.9, my+math.sin(sa3)*l2*0.9}}, {pressure={0.2,0.01}})
      end
      cnt = cnt + 1
      if cnt % 14 == 0 then b:load(twigDark, 0.6) end
    end
  end
end
for _,s in ipairs(oak.thick) do spurs(s, 9) end
for _,s in ipairs(oak.thin) do if s.ws[1] > 0.9 then spurs(s, 14) end end
print(cnt)

--@ chunk 26
extra = {thick={}, thin={}}
local function env(x, y)
  local e = ((x-212)/240)^2 + ((y-185)/150)^2
  if y > 330 then e = math.max(e, 1 + (y-330)/20) end
  if x < 4 then e = math.max(e, 1.2) end
  return e
end
local function grow(x, y, ang, len, w, depth, up)
  local pts, ws = {{x,y}}, {w}
  local nseg = math.max(2, math.floor(len / 10))
  local sl = len / nseg
  local cw = w
  for i = 1, nseg do
    ang = ang + randn(0, 0.3)
    ang = ang + (-math.pi/2 - ang) * up
    local nx = x + math.cos(ang) * sl * rand(0.8, 1.2)
    local ny = y + math.sin(ang) * sl * rand(0.8, 1.2)
    if env(nx, ny) > 1 then break end
    x, y = nx, ny
    if depth > 0 and i < nseg and math.random() < 0.7 then
      local wc = cw * rand(0.45, 0.72)
      grow(x, y, ang + ((math.random()<0.5) and -1 or 1) * rand(0.5, 1.1), len * rand(0.4, 0.65), wc, depth - 1, up)
      cw = math.max(cw * 0.62, math.sqrt(math.max(cw*cw - wc*wc, 0)))
    else cw = cw * 0.9 end
    pts[#pts+1] = {x, y}; ws[#ws+1] = math.max(cw, 0.4)
  end
  if #pts < 2 then return end
  if w >= 2.2 then extra.thick[#extra.thick+1] = {pts=pts, ws=ws} else extra.thin[#extra.thin+1] = {pts=pts, ws=ws} end
  if depth > 0 then for k = 1, 2 do grow(x, y, ang + randn(0, 0.6), len * rand(0.35, 0.6), cw * 0.72, depth - 1, up) end end
end
grow(300,150, -0.55, 95, 3.6, 3, 0.03)
grow(130,112, -2.05, 70, 3.0, 3, 0.04)
grow(345,262, 0.05, 95, 3.4, 3, -0.02)
grow(62,236, -2.6, 60, 2.6, 3, 0.03)
grow(250,100, -0.9, 60, 2.4, 2, 0.02)
local m = nil
for _,s in ipairs(extra.thick) do local r = ribbon(s.pts, s.ws); m = m and (m + r) or r end
if m then work(m, {hand="detail", pile=twigDark, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true}) end
local b = brush{kind="round", width=3, point=1, stiffness=0.5}
b:load(twigP, 0.8)
for i,s in ipairs(extra.thin) do
  local p0 = math.max(b:pressure_for(s.ws[1]), 0.15)
  b:stroke(s.pts, {pressure={p0, 0.02}, ramps={0.0, 0.5}})
  if i % 12 == 0 then b:load(twigP, 0.7) end
end
print(#extra.thick, #extra.thin)

--@ chunk 27
local starts = {{300,150,3.6},{130,112,3.0},{345,262,3.4},{62,236,2.6},{250,100,2.4}}
local m = nil
for _,st in ipairs(starts) do
  local best, bd, bw = nil, 1e9, 0
  for _,s in ipairs(oak.thick) do
    for i,p in ipairs(s.pts) do
      local d = (p[1]-st[1])^2 + (p[2]-st[2])^2
      -- prefer points below (closer to the trunk) and thicker than the start
      if p[2] > st[2] - 5 and s.ws[i] >= st[3] and d < bd and d > 16 then best, bd, bw = p, d, s.ws[i] end
    end
  end
  if best then
    local mx = (best[1]+st[1])/2 + randn(0,3); local my = (best[2]+st[2])/2 + randn(0,3)
    print(st[1],st[2],"<-",best[1],best[2], math.sqrt(bd))
    local r = ribbon({{best[1],best[2]},{mx,my},{st[1],st[2]}}, {math.min(bw, st[3]*1.4), st[3]*1.15, st[3]})
    m = m and (m + r) or r
  end
end
work(m, {hand="detail", pile=twigDark, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true})

--@ chunk 28
stoneDark = pile{{"bone black",1},{"raw umber",1},{"lead white",1.3},{"pale smalt",0.8}, medium=0.08}
stoneMid  = pile{{"bone black",0.6},{"raw umber",0.7},{"lead white",2.2},{"pale smalt",1.0},{"yellow ochre",0.1}, medium=0.08}
chamber   = pile{{"bone black",1.4},{"raw umber",1.2},{"smalt",0.3},{"lead white",0.3}, medium=0.08}
cap = outline{{274,386,"c"},{282,374},{300,366},{330,362},{362,363},{394,366},{416,371},{428,380,"c"},{424,392,"c"},{400,397},{360,395},{320,399},{290,398},{274,394,"c"}, char="broken", seed=41, closed=true, amount=0.6}
u1 = outline{{287,432,"c"},{285,410},{290,392,"c"},{314,390,"c"},{320,412},{318,431,"c"}, char="broken", seed=42, closed=true, amount=0.6}
u2 = outline{{341,428,"c"},{340,410},{344,394,"c"},{364,394,"c"},{368,412},{366,428,"c"}, char="broken", seed=43, closed=true, amount=0.6}
u3 = outline{{384,431,"c"},{383,410},{388,392,"c"},{410,392,"c"},{416,410},{414,432,"c"}, char="broken", seed=44, closed=true, amount=0.6}
f1 = outline{{438,434,"c"},{444,420},{458,414},{474,418},{480,430,"c"}, char="broken", seed=45, closed=true, amount=0.5}
f2 = outline{{244,432,"c"},{248,422},{262,419},{270,426},{268,434,"c"}, char="broken", seed=46, closed=true, amount=0.5}
-- the chamber: the dark between the uprights, under the slab
chamberM = poly({{300,396},{410,394},{410,428},{300,430}})
work(chamberM, {hand="detail", pile=chamber, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0})
stonesM = cap:mask() + u1:mask() + u2:mask() + u3:mask() + f1:mask() + f2:mask()
work(stonesM, {hand="detail", pile=stoneDark, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=-0.3, angle_jitter=0.6})

--@ chunk 29
print(wait(16*60)); for _,p in ipairs({{350,380},{330,410},{193,400},{200,150},{255,230},{120,200}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 30
sprP = pile{{"Prussian blue",0.4},{"raw umber",1.5},{"yellow ochre",0.4},{"bone black",0.5},{"lead white",0.9}, medium=0.1}
spruces = {{872,497,34},{893,502,60},{915,499,44},{937,503,29},{953,500,19},{858,500,16}}
local b = brush{kind="round", width=2.2, point=1, stiffness=0.6}
b:load(sprP, 0.8)
local n = 0
for _,s in ipairs(spruces) do
  local x0, yb, h = s[1], s[2], s[3]
  local top = yb - h
  -- stem
  b:stroke({{x0, yb+1}, {x0+randn(0,0.4), top}}, {pressure={0.45, 0.02}})
  local y = yb - 2
  while y > top + 1 do
    local t = (yb - y) / h
    local hw = h * 0.21 * (1 - t)^0.85 + 1.2
    for _,side in ipairs({-1, 1}) do
      local len = hw * rand(0.8, 1.15)
      local droop = rand(0.25, 0.55)
      local ex = x0 + side * len
      local ey = y + len * droop * 0.6
      b:stroke({{x0, y - 1}, {x0 + side*len*0.55, y + len*droop*0.35}, {ex, ey}, {ex + side*1.2, ey - 0.6}}, {pressure={0.55, 0.05}, ramps={0, 0.4}})
      n = n + 1
      if n % 10 == 0 then b:load(sprP, 0.7) end
    end
    y = y - rand(1.8, 3.0)
  end
  -- leader
  b:stroke({{x0, top+3},{x0, top-2.5}}, {pressure={0.3,0.0}})
end
print(n)

--@ chunk 31
local M = nil
sprTiers = {}
for _,s in ipairs(spruces) do
  local x0, yb, h = s[1], s[2], s[3]
  local top = yb - h
  local y = yb - 1
  while y > top + 2 do
    local t = (yb - y) / h
    local hw = h * 0.2 * (1 - t)^0.85 + 1.0
    local dy = rand(2.2, 3.4)
    local drop = hw * rand(0.3, 0.45)
    local p = poly({{x0, y - dy*1.2}, {x0 + hw*0.5, y - dy*0.3}, {x0 + hw, y + drop*0.5}, {x0 + hw*0.6, y + drop*0.25}, {x0, y + 1}, {x0 - hw*0.6, y + drop*0.25}, {x0 - hw, y + drop*0.5}, {x0 - hw*0.5, y - dy*0.3}})
    M = M and (M + p) or p
    sprTiers[#sprTiers+1] = {x0, y, hw, drop}
    y = y - dy
  end
  M = M + poly({{x0-0.8, top+3}, {x0, top - 2}, {x0+0.8, top+3}})
end
sprM = M:roughen(0.5, 2, 51)
work(sprM, {hand="detail", pile=sprP, coverage=3, pressure={0.5,0.8}, load=0.9, fill=true, angle=0.3, angle_jitter=0.8, length={2,5}})

--@ chunk 32
coatP = pile{{"bone black",1.2},{"raw umber",0.8},{"smalt",0.25},{"lead white",0.2}, medium=0.06}
FX, FY = 560, 548
local fx, fy = FX, FY
local coat = poly({{fx-5.2,fy-32.5},{fx-2,fy-34},{fx+2.5,fy-34},{fx+5.5,fy-32},{fx+6.5,fy-24},{fx+7.2,fy-14},{fx+8.2,fy-6.5},{fx+3,fy-5.5},{fx-2,fy-6},{fx-7.8,fy-6.8},{fx-7,fy-16},{fx-6.4,fy-25}}, true)
local head = ellipse(fx-0.2, fy-36.8, 2.9, 3.4)
local hat = poly({{fx-4.2,fy-39},{fx-2.5,fy-42.4},{fx+2.2,fy-42.6},{fx+4.0,fy-39.6},{fx+1,fy-38.6},{fx-2,fy-38.5}}, true)
local legs = rect(fx-4.2, fy-7, 2.8, 7.2) + rect(fx+1.2, fy-7, 2.8, 7)
local arm = ribbon({{fx-5.4,fy-31},{fx-7.8,fy-24},{fx-8.6,fy-19}}, {3.2,3.0,2.8})
figM = coat + head + hat + legs + arm
work(figM, {hand="detail", pile=coatP, coverage=4, pressure={0.5,0.8}, load=1, fill=true, angle=-math.pi/2, tool={kind="round", width=1.6}})
-- the stick
local r = brush{kind="round", width=2, point=1}
r:load(coatP, 0.7)
r:stroke({{fx-9.6,fy-21},{fx-10.6,fy-10},{fx-11.6,fy+1}}, {pressure={0.55,0.45}})

--@ chunk 33
print(wait(10*60)); for _,p in ipairs({{350,380},{330,410},{305,420},{893,480},{560,530}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 34
snowTop = pile{{"lead white",7},{"pale smalt",0.5},{"yellow ochre",0.12},{"vermilion",0.03}, medium=0.08}
snowSide = pile{{"lead white",4},{"pale smalt",1.6},{"smalt",0.4},{"raw umber",0.15}, medium=0.08}
-- underside of the capstone in its own shadow
local under = cap:mask() * below({{270,387},{320,390},{360,388},{400,390},{430,386}})
work(under, {hand="detail", pile=chamber, coverage=3, pressure={0.5,0.8}, load=0.9, fill=true, angle=0})
-- facets on the front face of the slab: planes turned a little to the sky
local fac = poly({{296,372},{322,366},{350,367},{346,378},{318,382},{298,381}}) + poly({{372,368},{398,370},{412,376},{404,383},{380,381}})
work(fac * cap:mask(), {hand="detail", pile=stoneMid, coverage=2.5, pressure={0.4,0.7}, load=0.7, fill=true, angle=-0.2})
-- cracks
local r = brush{kind="round", width=2, point=1}
r:load(chamber, 0.7)
r:stroke({{352,366},{349,375},{353,383},{350,390}}, {pressure={0.45,0.1}})
r:stroke({{318,383},{330,381},{346,384}}, {pressure={0.35,0.05}})
r:stroke({{402,372},{396,381},{399,389}}, {pressure={0.4,0.05}})
r:stroke({{303,396},{305,410},{302,422}}, {pressure={0.35,0.05}})
r:stroke({{396,398},{399,414}}, {pressure={0.3,0.05}})
-- the snow cap on the slab
local topc = {{272,386},{280,373},{298,364},{330,359.5},{362,360.5},{394,363.5},{417,368.5},{430,379}}
local bot = {}
for i = #topc, 1, -1 do local p = topc[i]; bot[#bot+1] = {p[1] + randn(0,1), p[2] + rand(4.5, 9)} end
local pts = {}
for _,p in ipairs(topc) do pts[#pts+1] = p end
for _,p in ipairs(bot) do pts[#pts+1] = p end
capSnow = poly(pts, true):roughen(0.8, 4, 55)
work(capSnow, {hand="detail", pile=snowTop, coverage=3.5, pressure={0.5,0.8}, load=1, fill=true, angle=0})
-- snow on the fallen stones and the tops of the uprights where they show
local s2 = poly({{442,424},{448,416},{458,412},{472,415},{478,423},{468,421},{456,419},{447,423}}, true) + poly({{246,425},{252,419},{262,417},{269,423},{262,422},{252,424}}, true)
work(s2, {hand="detail", pile=snowTop, coverage=3.5, pressure={0.5,0.8}, load=1, fill=true, angle=0})

--@ chunk 35
barkLight = pile{{"raw umber",1},{"bone black",0.4},{"lead white",1.6},{"pale smalt",0.6}, medium=0.1}
local r = brush{kind="round", width=2.5, point=1, stiffness=0.6}
-- trunk spans roughly x 168..214 (base) to 176..212 at the fork; y 440..330
local function halfw(y) return lerp(24, 20, (440 - y) / 110) end
local function cx(y) return lerp(190, 196, (440 - y) / 110) end
r:load(chamber, 0.7)
for i = 1, 26 do
  local u = rand(-0.85, 0.85)
  local y0 = rand(335, 430); local L = rand(18, 50)
  local pts = {}
  for k = 0, 4 do
    local y = y0 - L * k / 4
    if y < 332 then break end
    pts[#pts+1] = {cx(y) + u * halfw(y) + randn(0, 0.8), y}
  end
  if #pts >= 2 then r:stroke(pts, {pressure={rand(0.35,0.6), 0.05}, ramps={0.2,0.4}}) end
  if i % 6 == 0 then r:load(chamber, 0.7) end
end
-- ridges catching the cool sky light, mostly on the left flank
r:load(barkLight, 0.6)
for i = 1, 22 do
  local u = rand(-0.95, -0.3)
  if i % 4 == 0 then u = rand(0.5, 0.92) end
  local y0 = rand(340, 432); local L = rand(10, 30)
  local pts = {}
  for k = 0, 3 do
    local y = y0 - L * k / 3
    pts[#pts+1] = {cx(y) + u * halfw(y) + randn(0, 0.6), y}
  end
  r:stroke(pts, {pressure={rand(0.25,0.45), 0.03}, ramps={0.3,0.4}})
  if i % 6 == 0 then r:load(barkLight, 0.6) end
end
-- a burr and an old wound on the bole
local burr = ellipse(204, 372, 6, 5):roughen(0.7, 3, 61)
work(burr, {hand="detail", pile=chamber, coverage=3, pressure={0.4,0.7}, load=0.8, fill=true})
r:load(barkLight, 0.5)
r:stroke({{199,368},{203,366},{208,368}}, {pressure={0.3,0.05}})

--@ chunk 36
print(wait(22*60)); for _,p in ipairs({{350,380},{350,362},{330,410},{193,400},{893,480},{560,530},{200,372}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 37
cap2 = outline{{270,390,"c"},{276,376},{292,362},{318,354},{350,350.5},{382,352},{408,358},{426,368},{434,380,"c"},{430,392,"c"},{404,398},{360,396},{320,400},{288,399},{270,396,"c"}, char="broken", seed=71, closed=true, amount=0.5}
local cm = cap2:mask()
-- restate the slab as granite: dark body, then a broken stipple of lighter grains
work(cm, {hand="detail", pile=stoneDark, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true, angle=-0.2, angle_jitter=0.7, length={3,8}})
stipple(cm * above({{260,386},{440,384}}), {pile=stoneMid, width=2.2, coverage=0.5, pressure={0.3,0.6}, cluster=0.6, feather=0.3})
local under = cm * below({{268,388},{320,391},{360,389},{400,391},{436,387}})
work(under, {hand="detail", pile=chamber, coverage=3, pressure={0.5,0.8}, load=0.9, fill=true, angle=0})

--@ chunk 38
local top = {{268,390},{274,377},{290,361},{316,351},{350,346.5},{383,348},{409,354},{428,365},{437,379}}
local lower = {{437,381},{424,376},{414,374},{404,372},{392,368},{382,370},{370,366},{356,368},{344,365},{330,368},{318,366},{306,372},{296,371},{286,378},{276,383},{268,391}}
local pts = {}
for _,p in ipairs(top) do pts[#pts+1] = p end
for _,p in ipairs(lower) do pts[#pts+1] = {p[1], p[2] + randn(0, 0.8)} end
capSnowM = poly(pts, true):roughen(0.7, 3, 81)
-- shaded body of the snow first, then the lit crown of it
work(capSnowM, {hand="detail", pile=snowSide, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true, angle=0})
local lit = capSnowM * above({{260,382},{300,364},{350,357},{400,360},{440,374}})
work(lit, {hand="detail", pile=snowTop, coverage=3.5, pressure={0.6,0.9}, load=1, fill=true, angle=0})

--@ chunk 39
printP = pile{{"lead white",2.5},{"pale smalt",2},{"smalt",0.7},{"raw umber",0.35},{"vermilion",0.03}, medium=0.1}
-- the track: from the lower edge up to where the man stands, getting smaller with distance
local path = {{742,722},{712,680},{676,636},{640,600},{606,574},{578,556},{566,551}}
local function at(t)
  local n = #path - 1
  local f = t * n; local i = math.min(math.floor(f) + 1, n); local u = f - (i - 1)
  local a, b = path[i], path[i+1]
  return lerp(a[1], b[1], u), lerp(a[2], b[2], u), math.atan(b[2]-a[2], b[1]-a[1])
end
local t = 0.0
local side = 1
local m = nil
while t < 0.985 do
  local x, y, ang = at(t)
  local s = lerp(1, 0.18, t^0.7)          -- scale with distance
  local nx, ny = -math.sin(ang), math.cos(ang)
  local px, py = x + nx * side * 5 * s, y + ny * side * 5 * s
  local e = ellipse(px, py, 7 * s + 0.6, 3.2 * s + 0.4)
  m = m and (m + e) or e
  side = -side
  t = t + 0.022 * s + 0.004
end
trackM = m:roughen(0.4, 2, 91)
work(trackM, {hand="detail", pile=printP, coverage=2.5, pressure={0.4,0.7}, load=0.6, fill=true, angle=0, tool={kind="round", width=1.8}})
-- the lit far lips of the larger prints
local r = brush{kind="round", width=2, point=1}
r:load(snowTop, 0.5)
t = 0
side = 1
while t < 0.6 do
  local x, y, ang = at(t)
  local s = lerp(1, 0.18, t^0.7)
  local nx, ny = -math.sin(ang), math.cos(ang)
  local px, py = x + nx * side * 5 * s, y + ny * side * 5 * s
  r:stroke({{px - 6*s, py - 3*s}, {px, py - 3.8*s}, {px + 6*s, py - 3*s}}, {pressure={0.3, 0.05}})
  side = -side
  t = t + 0.022 * s + 0.004
end

--@ chunk 40
trackPath = {{744,722},{712,680},{676,636},{640,600},{606,574},{580,557},{568,551}}
local ws = {30,24,18,13,8,5,3.5}
furrowM = ribbon(trackPath, ws):roughen(1.5, 6, 101)
local furrowP = pile{{"lead white",4},{"pale smalt",1.8},{"smalt",0.45},{"raw umber",0.2},{"vermilion",0.03}, medium=0.1}
work(furrowM, {hand="detail", pile=furrowP, coverage=3.5, pressure={0.5,0.8}, load=0.9, fill=true, angle=-0.9, angle_jitter=0.6, length={4,10}})

--@ chunk 41
postP = pile{{"raw umber",1.5},{"bone black",0.6},{"lead white",0.9},{"yellow ochre",0.3}, medium=0.08}
posts = {
  {pts={{90,646},{93,610},{97,574}}, ws={7.5,7,6.5}},
  {pts={{150,628},{151,600},{152,580}}, ws={6.5,6.2,6}},
  {pts={{213,612},{214,590},{216,574}}, ws={5.5,5.2,5}},
}
local m = nil
for _,p in ipairs(posts) do local r = ribbon(p.pts, p.ws); m = m and (m + r) or r end
-- a split, broken top on the middle post
m = m + poly({{148,582},{149,573},{151,577},{153,570},{155,581}})
-- the fallen rail, half sunk in the snow
local rail = ribbon({{96,600},{124,606},{150,612},{170,617}}, {4.5,4.2,4.2,4})
postsM = (m + rail):roughen(0.5, 3, 111)
work(postsM, {hand="detail", pile=postP, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=-math.pi/2, angle_jitter=0.3})
-- grain and cracks of the weathered wood
local r = brush{kind="round", width=2, point=1}
r:load(chamber, 0.6)
r:stroke({{92,640},{94,612},{96,580}}, {pressure={0.3,0.05}})
r:stroke({{151,624},{151,596}}, {pressure={0.3,0.05}})
r:stroke({{214,608},{215,585}}, {pressure={0.28,0.05}})

--@ chunk 42
grassP = pile{{"yellow ochre",1.5},{"raw umber",1.2},{"lead white",0.5},{"red earth",0.2}, medium=0.12}
grassD = pile{{"raw umber",1.6},{"bone black",0.5},{"yellow ochre",0.5},{"lead white",0.3}, medium=0.1}
function grassClump(x, y, h, n, pile, seedHeads)
  local r = brush{kind="rigger", width=1.6, point=1}
  r:load(pile, 0.6)
  for i = 1, n do
    local bx = x + randn(0, h*0.12)
    local lean = randn(0, 0.35) + 0.08
    local hh = h * rand(0.4, 1.0)
    local tx = bx + math.sin(lean) * hh
    local ty = y - math.cos(lean) * hh
    local mx = bx + math.sin(lean) * hh * 0.5 + randn(0, hh*0.03)
    local my = y - math.cos(lean) * hh * 0.5
    -- some stalks bend over at the top, broken by the snow
    if math.random() < 0.25 then tx = tx + hh*0.25*(lean>0 and 1 or -1); ty = ty + hh*0.3 end
    r:stroke({{bx, y}, {mx, my}, {tx, ty}}, {pressure={rand(0.45,0.7), 0.0}, ramps={0.05, 0.6}})
    if seedHeads and math.random() < 0.3 then
      r:stroke({{tx, ty}, {tx + math.sin(lean)*2.5, ty - 3}}, {pressure={0.75, 0.3}})
    end
    if i % 6 == 0 then r:load(pile, 0.6) end
  end
end
-- lower left corner
grassClump(30, 690, 34, 22, grassP, true)
grassClump(60, 702, 26, 14, grassD, false)
grassClump(14, 668, 22, 10, grassP, true)
-- lower right, by the track
grassClump(820, 690, 30, 18, grassP, true)
grassClump(850, 704, 22, 10, grassD, false)
grassClump(935, 670, 24, 14, grassP, true)
grassClump(975, 700, 28, 12, grassD, true)
-- smaller, farther tufts
grassClump(420, 610, 12, 9, grassP, false)
grassClump(460, 632, 14, 8, grassD, false)
grassClump(700, 585, 9, 7, grassP, false)
grassClump(330, 560, 8, 6, grassP, false)
grassClump(760, 548, 6, 5, grassP, false)

--@ chunk 43
print(wait(3*24*60)); for _,p in ipairs({{350,360},{350,380},{330,410},{193,400},{893,480},{94,620},{700,650},{560,530}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 44
print(wait(4*24*60)); for _,p in ipairs({{350,360},{350,380},{330,410},{193,400},{893,480},{94,620},{700,650},{560,530},{200,150},{120,200}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 45
moundShade = pile{{"lead white",3.5},{"pale smalt",1.8},{"smalt",0.5},{"raw umber",0.2},{"vermilion",0.04}, medium=0.12}
local crest = {{0,478},{60,466},{130,446},{210,425},{300,414},{390,418},{470,434},{540,455},{610,471},{700,474}}
local rimBelow = {}
for _,p in ipairs(crest) do rimBelow[#rimBelow+1] = {p[1], p[2] + 5} end
local objects = (stonesM + cap2:mask() + oakThickM + chamberM):grow(0.5)
slopeM = land * below(rimBelow) * above({{0,474},{300,472},{620,476}}) - objects
work(slopeM, {hand="body", pile=moundShade, angle=0.05, coverage=2.5, pressure={0.5,0.8}, fill=true, length={20,50}, clip=true, angle_jitter=0.15})
-- the transition into the field, stippled as he did
stipple(land * rect(0,466,640,34) - objects, {pile=moundShade, width=2.2, coverage=function(x,y) return clamp((496-y)/28, 0, 1)*1.4 end, feather=0.8, pressure={0.3,0.6}, cluster=0.2})

--@ chunk 46
local objects = (stonesM + cap2:mask() + oakThickM + chamberM):grow(1)
local band = land * rect(0,462,630,28) - objects
stipple(band, {pile=moundShade, width=2.5, coverage=function(x,y) return clamp((488-y)/22, 0, 1)*2.0 end, feather=0.7, pressure={0.35,0.65}, cluster=0.3})
blend(land * rect(0,455,630,40) - objects, {angle=0.02})

--@ chunk 47
local dTop = {{272,420},{282,416.5},{292,414},{305,415.5},{322,413},{340,416.5},{355,414},{370,415.5},{388,412},{404,414.5},{420,413},{434,417},{446,422},{460,420},{476,424},{486,430}}
driftDolmen = below(dTop) * above({{260,440},{500,440}}) * rect(262,400,230,45)
local tTop = {{140,452},{156,446},{168,440},{176,436.5},{186,438},{196,437.5},{206,435},{214,434},{224,438},{240,442},{252,441}}
driftTrunk = below(tTop) * above({{130,470},{260,470}}) * rect(135,425,125,45)
work(driftDolmen + driftTrunk, {hand="detail", pile=moundShade, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0, angle_jitter=0.2, length={4,12}})
-- the lit lip of each drift
local r = brush{kind="round", width=2.2, point=1}
r:load(snowTop, 0.6)
r:stroke(dTop, {pressure={0.35,0.25}})
r:load(snowTop, 0.6)
r:stroke(tTop, {pressure={0.35,0.25}})

--@ chunk 48
local wedge = poly({{482,427},{490,428.5},{500,433},{512,440},{524,447},{500,448},{484,446}}, true)
work(wedge, {hand="detail", pile=moundShade, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0.3})
local r = brush{kind="round", width=2.2, point=1}
r:load(snowTop, 0.5)
r:stroke({{476,424},{488,428},{500,433},{512,440},{526,448}}, {pressure={0.3,0.1}})
-- the fallen stone to the right shows again under its cap
local st = poly({{443,425},{447,419.5},{458,417},{472,418.5},{478,424},{474,427.5},{452,428}}, true)
work(st - capSnowM, {hand="detail", pile=stoneDark, coverage=4, pressure={0.6,0.9}, load=1, fill=true, tool={kind="round", width=1.6}})

--@ chunk 49
local function snowOn(list, minw)
  local m = nil
  for _,s in ipairs(list) do
    local run, runw = {}, {}
    local function flush()
      if #run >= 2 then local r = ribbon(run, runw); m = m and (m + r) or r end
      run, runw = {}, {}
    end
    for i = 1, #s.pts - 1 do
      local a, b = s.pts[i], s.pts[i+1]
      local dx, dy = b[1]-a[1], b[2]-a[2]
      local L = math.sqrt(dx*dx + dy*dy)
      if L > 0 then
        local ang = math.atan(dy, dx)
        local h = math.abs(math.cos(ang))       -- 1 when level
        local w = (s.ws[i] + s.ws[i+1]) / 2
        if w >= minw and h > 0.55 and math.random() < 0.85 then
          local nx, ny = -dy/L, dx/L
          if ny > 0 then nx, ny = -nx, -ny end
          local t = clamp(w * 0.38, 0.9, 3.2) * (h - 0.4) / 0.6
          local off = w/2 - t*0.35
          if #run == 0 then run[1] = {a[1] + nx*off, a[2] + ny*off}; runw[1] = t*0.5 end
          run[#run+1] = {b[1] + nx*off, b[2] + ny*off}; runw[#runw+1] = t
        else flush() end
      end
    end
    flush()
  end
  return m
end
local m = snowOn(oak.thick, 2.2)
local m2 = snowOn(extra.thick, 2.2)
if m2 then m = m + m2 end
limbSnowM = m:roughen(0.4, 3, 121)
work(limbSnowM, {hand="detail", pile=snowTop, coverage=3.5, pressure={0.5,0.8}, load=0.9, fill=true, angle=0, tool={kind="round", width=1.4}})

--@ chunk 50
for _,p in ipairs({{660,500},{660,600},{560,470},{190,470}}) do print(p[1],p[2],drying(p[1],p[2]), slopeM and "" ) end

--@ chunk 51
print(wait(30*60)); for _,p in ipairs({{660,500},{660,600},{350,360},{350,380},{520,440},{300,420}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 52
fieldTop = {{-10,492},{540,492},{590,486},{630,480},{1010,480}}
local keep = (figM:grow(0.8) + postsM:grow(0.8) + sprM:grow(0.8))
fieldM = land * below(fieldTop) - keep
fieldA = pile{{"lead white",6},{"pale smalt",1.2},{"yellow ochre",0.15},{"vermilion",0.04}, medium=0.12}
fieldB = pile{{"lead white",5},{"pale smalt",1.8},{"smalt",0.35},{"raw umber",0.12},{"vermilion",0.02}, medium=0.12}
fieldC = pile{{"lead white",3.6},{"pale smalt",2.2},{"smalt",0.75},{"raw umber",0.24},{"bone black",0.04}, medium=0.12}
work(fieldM * below({{-10,470},{1010,470}}) * above({{-10,545},{1010,545}}), {hand="body", pile=fieldA, angle=0, coverage=2.5, fill=true, angle_jitter=0.04, pressure={0.5,0.8}, length={40,90}, clip=true})
work(fieldM * below({{-10,535},{1010,535}}) * above({{-10,625},{1010,625}}), {hand="body", pile=fieldB, angle=0, coverage=2.5, fill=true, angle_jitter=0.04, pressure={0.5,0.8}, length={40,90}, clip=true})
work(fieldM * below({{-10,615},{1010,615}}), {hand="body", pile=fieldC, angle=0.02, coverage=2.5, fill=true, angle_jitter=0.06, pressure={0.5,0.8}, length={40,100}, clip=true})
blend(fieldM, {angle=0})
blend(fieldM * below({{-10,520},{1010,520}}), {angle=0.02})

--@ chunk 53
local blob = poly({{160,462},{175,456},{200,456},{222,462},{215,478},{170,478}}, true)
work(blob, {hand="detail", pile=moundShade, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0})
local band = land * below({{-10,478},{560,480},{600,476},{1010,476}}) * above({{-10,502},{560,502},{620,492},{1010,492}}) - figM:grow(1)
stipple(band, {pile=moundShade, width=2.5, coverage=function(x,y) return clamp((500-y)/20, 0, 1)*1.6 end, feather=0.7, pressure={0.35,0.6}, cluster=0.3})
blend(band, {angle=0.02})

--@ chunk 54
local crestL = {{-10,470},{20,465},{50,459},{85,450.5},{120,441.5},{150,434.5},{175,430},{200,426},{1010,426}}
local m = below(crestL) * above({{-10,482},{1010,482}}) * rect(-10,420,190,70) - oakThickM:grow(0.5)
work(m, {hand="detail", pile=moundShade, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=-0.2, length={6,14}})
local r = brush{kind="round", width=2.4, point=1}
r:load(snowTop, 0.6)
r:stroke({{-5,469.5},{20,464.5},{50,458.5},{85,450},{120,441}}, {pressure={0.35,0.3}})
r:load(snowTop, 0.6)
r:stroke({{120,441},{150,434},{168,431}}, {pressure={0.3,0.1}})

--@ chunk 55
for _,p in ipairs({{350,352},{350,362},{300,370},{400,362},{460,420}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 56
snowSide2 = pile{{"lead white",4.5},{"pale smalt",1.5},{"smalt",0.35},{"raw umber",0.12},{"vermilion",0.03}, medium=0.08}
local top = {{268,389},{273,377},{289,361},{315,350.5},{350,346},{383,347.5},{409,353.5},{428,364.5},{437,379}}
local lip = {{437,381},{430,378},{420,377},{410,373},{400,374},{390,370},{378,372},{366,368},{352,370},{340,367},{328,370},{316,368},{305,373},{295,372},{285,378},{276,384},{268,391}}
local pts = {}
for _,p in ipairs(top) do pts[#pts+1] = p end
for _,p in ipairs(lip) do pts[#pts+1] = p end
local snowM = poly(pts, true):roughen(0.6, 3, 141)
work(snowM, {hand="detail", pile=snowSide2, coverage=4.5, pressure={0.7,1.0}, load=1, fill=true, angle=0, tool={kind="round", width=2}})
local litM = snowM * above({{260,380},{290,364},{320,356},{350,353},{385,354},{415,360},{440,372}})
work(litM, {hand="detail", pile=snowTop, coverage=4.5, pressure={0.7,1.0}, load=1, fill=true, angle=0, tool={kind="round", width=2}})
-- the shadow line under the overhanging lip
local r = brush{kind="round", width=2, point=1}
r:load(chamber, 0.6)
local sh = {}
for _,p in ipairs(lip) do sh[#sh+1] = {p[1], p[2] + 1.3} end
r:stroke(sh, {pressure={0.3,0.3}})
-- snow cap on the fallen stone to the right, and on the left one
local c1 = poly({{444,423},{447,418.5},{456,415.5},{468,416},{477,421},{471,420.5},{460,420},{450,422}}, true)
local c2 = poly({{246,425},{250,419.5},{260,417},{268,421},{263,421.5},{254,422.5}}, true)
work(c1 + c2, {hand="detail", pile=snowTop, coverage=4.5, pressure={0.7,1.0}, load=1, fill=true, angle=0, tool={kind="round", width=1.5}})

--@ chunk 57
-- break the scalloped lip: irregular tongues of snow lower in places, stone restated in others
local tongues = nil
local xs = uneven(9, 282, 428, 0.8, 0.5, 151)
for i, x in ipairs(xs) do
  local base = 366 + (math.abs(x - 350) / 80)^2 * 10
  local w = rand(5, 14); local d = rand(2.5, 7)
  local p = poly({{x - w/2, base - 3}, {x - w*0.3, base + d*0.7}, {x + randn(0,1.5), base + d}, {x + w*0.35, base + d*0.6}, {x + w/2, base - 3}}, true)
  tongues = tongues and (tongues + p) or p
end
tongues = tongues:roughen(0.6, 2.5, 152)
work(tongues * cap2:mask(), {hand="detail", pile=snowSide2, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0, tool={kind="round", width=1.4}})
-- the stone restated along some stretches just under the lip, to break the line
local r = brush{kind="round", width=2.2, point=1}
r:load(stoneDark, 0.7)
for i = 1, 7 do
  local x = rand(290, 420)
  local base = 368 + (math.abs(x - 350) / 80)^2 * 10
  r:stroke({{x, base + 2.5}, {x + rand(4, 9), base + 2.2 + randn(0, 0.8)}}, {pressure={0.55, 0.4}})
end
-- a few grains of granite catching light on the front face of the slab
stipple(cap2:mask() * below({{260,378},{440,378}}) * above({{260,392},{440,392}}), {pile=stoneMid, width=1.4, coverage=0.35, pressure={0.3,0.5}, cluster=0.5})

--@ chunk 58
local lip = {{437,381.5},{432,381},{423,378},{414,374.5},{405,372.5},{398,376},{390,373.5},{378,373},{370,375.5},{362,373.5},{350,374.5},{338,374},{331,376.5},{322,374.5},{311,375.5},{303,378.5},{296,377},{288,380},{280,382},{272,386.5}}
local upper = {{272,383},{285,372},{300,366},{330,362},{360,361},{390,362},{414,366},{430,373},{437,379}}
local pts = {}
for _,p in ipairs(upper) do pts[#pts+1] = p end
for _,p in ipairs(lip) do pts[#pts+1] = {p[1], p[2] + randn(0, 0.4)} end
local m = poly(pts, true):roughen(0.5, 2.5, 161)
work(m, {hand="detail", pile=snowSide2, coverage=4.5, pressure={0.7,1.0}, load=1, fill=true, angle=0, tool={kind="round", width=1.6}})
-- broken shadow under the lip
local r = brush{kind="round", width=2, point=1}
r:load(chamber, 0.5)
local i = 1
while i < #lip do
  if math.random() < 0.55 then
    local a, b = lip[i], lip[i+1]
    r:stroke({{a[1], a[2] + 1.4}, {b[1], b[2] + 1.4}}, {pressure={0.28, 0.12}})
  end
  i = i + 1
end

--@ chunk 59
local r = brush{kind="round", width=1.6, point=1}
r:load(snowTop, 0.6)
local n = 0
for _,t in ipairs(sprTiers) do
  local x0, y, hw, drop = t[1], t[2], t[3], t[4]
  for _,side in ipairs({-1, 1}) do
    if math.random() < 0.7 then
      local a = rand(0.1, 0.35); local b = rand(0.6, 0.95)
      local xa, ya = x0 + side*hw*a, y - 1.8 + drop*0.5*a
      local xb, yb = x0 + side*hw*b, y - 1.0 + drop*0.5*b
      r:stroke({{xa, ya}, {xb, yb}}, {pressure={0.5, 0.2}})
      n = n + 1
      if n % 8 == 0 then r:load(snowTop, 0.6) end
    end
  end
end
-- small drifts and shadow at the feet of the spruces
local feet = nil
for _,s in ipairs(spruces) do
  local e = ellipse(s[1], s[2] + 0.5, s[3]*0.2 + 2, 1.6)
  feet = feet and (feet + e) or e
end
work(feet, {hand="detail", pile=moundShade, coverage=3, pressure={0.5,0.8}, load=0.8, fill=true, angle=0, tool={kind="round", width=1.2}})
print(n)

--@ chunk 60
print(wait(26*60)); for _,p in ipairs({{893,480},{660,600},{300,650},{350,375}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 61
sprDark = pile{{"Prussian blue",0.5},{"raw umber",1.5},{"bone black",0.8},{"yellow ochre",0.3},{"lead white",0.4}, medium=0.08}
local r = brush{kind="round", width=1.8, point=1}
r:load(sprDark, 0.7)
local n = 0
for _,t in ipairs(sprTiers) do
  local x0, y, hw, drop = t[1], t[2], t[3], t[4]
  for _,side in ipairs({-1, 1}) do
    -- the shadowed underside of each tier, stem outward, drooping
    local xb, yb = x0 + side*hw*rand(0.75, 0.95), y + drop*0.45
    r:stroke({{x0, y + 0.8}, {x0 + side*hw*0.5, y + drop*0.3 + 0.6}, {xb, yb}}, {pressure={0.5, 0.1}})
    n = n + 1
    if n % 8 == 0 then r:load(sprDark, 0.7) end
  end
end
-- the stems show dark between the tiers
for _,s in ipairs(spruces) do r:stroke({{s[1], s[2]}, {s[1], s[2] - s[3]*0.8}}, {pressure={0.4, 0.1}}) end

--@ chunk 62
local f, E, cx, hz = 1000, 3.4, 500, 470
local function proj(X, Z) return cx + X*f/Z, hz + f*E/Z end
printShade = pile{{"lead white",3.2},{"pale smalt",2},{"smalt",0.55},{"raw umber",0.22},{"vermilion",0.03}, medium=0.1}
printDeep  = pile{{"lead white",2},{"pale smalt",2},{"smalt",0.8},{"raw umber",0.35},{"vermilion",0.03}, medium=0.1}
-- the shallow trench the legs plough between the steps
local tp, tw = {}, {}
for Z = 12.5, 43.2, 0.5 do
  local X = lerp(3.4, 2.87, (Z-13)/30.6)
  local x, y = proj(X, Z)
  tp[#tp+1] = {x, y}; tw[#tw+1] = 0.34*f/Z
end
trench = ribbon(tp, tw):roughen(0.8, 3, 171)
work(trench, {hand="detail", pile=printShade, coverage=1.6, pressure={0.35,0.6}, load=0.5, angle=0, tool={kind="round", width=2}})
-- the steps
local m = nil
local side = 1
local Z = 12.6
while Z < 43.4 do
  local X = lerp(3.4, 2.87, (Z-13)/30.6) + side*0.12 + randn(0, 0.02)
  local x, y = proj(X, Z)
  local w = 0.19*f/Z * rand(0.85, 1.15)
  local h = 0.34*f*E/(Z*Z) + 0.35
  local e = ellipse(x, y, w/2, h/2)
  m = m and (m + e) or e
  side = -side
  Z = Z + rand(0.62, 0.78)
end
stepsM = m:roughen(0.4, 2, 172)
work(stepsM, {hand="detail", pile=printDeep, coverage=3, pressure={0.5,0.8}, load=0.8, fill=true, angle=0, tool={kind="round", width=1.6}})

--@ chunk 63
straw = pile{{"yellow ochre",1.5},{"lead white",1.1},{"raw umber",0.45},{"red earth",0.1}, medium=0.12}
local piles = {grassP, grassD, straw}
local centers = {{40,680},{120,660},{175,640},{250,690},{95,612},{230,612},{330,700},{400,655},{470,610},{860,690},{930,650},{985,700},{780,640},{700,600},{900,590},{620,660},{520,700},{300,590},{40,560},{820,540},{960,540},{430,560},{650,530}}
local n = 0
for ci, c in ipairs(centers) do
  local k = math.random(2, 6)
  for j = 1, k do
    local y = c[2] + randn(0, 6)
    local sc = (y - 470) / 3.4
    local x = c[1] + randn(0, sc*0.9)
    -- keep off the footpath
    local h = 0.42 * sc * rand(0.6, 1.3)
    local stalks = math.floor(5 + h * 0.5 + math.random(0, 4))
    grassClump(x, y, h, stalks, piles[math.random(1, 3)], math.random() < 0.5)
    n = n + 1
  end
end
print(n)

--@ chunk 64
print(wait(14*60)); for _,p in ipairs({{650,640},{560,530},{420,560},{700,600},{350,375}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 65
veilA = pile{{"lead white",6},{"pale smalt",1.4},{"yellow ochre",0.12},{"vermilion",0.03},{"smalt",0.1}, medium=0.3}
veilB = pile{{"lead white",5},{"pale smalt",1.8},{"smalt",0.35},{"raw umber",0.12},{"vermilion",0.02}, medium=0.3}
function buryTufts(cx, cy, pile, strength)
  local sc = (cy - 470) / 3.4
  local m = ellipse(cx, cy - 0.15*sc, 2.4*sc + 4, 0.45*sc + 4) - figM:grow(2) - stepsM:grow(1) - trench:grow(1)
  stipple(m, {pile=pile, width=2.2, coverage=function(x,y) local d = ((x-cx)/(2.4*sc+4))^2 + ((y-cy+0.15*sc)/(0.45*sc+4))^2; return clamp((1-d)*2.2, 0, 1) * strength end, feather=0.8, pressure={0.35,0.6}, cluster=0.2})
end
buryTufts(650, 530, veilA, 2.5)

--@ chunk 66
for _,c in ipairs({{40,560,"A"},{820,540,"A"},{960,540,"A"},{430,560,"A"},{300,590,"B"},{700,600,"B"},{900,590,"B"},{470,610,"B"},{780,640,"B"}}) do
  buryTufts(c[1], c[2], c[3] == "A" and veilA or veilB, 1.8)
end

--@ chunk 67
veilC = pile{{"lead white",3.8},{"pale smalt",2.2},{"smalt",0.7},{"raw umber",0.22},{"bone black",0.03}, medium=0.3}
for _,c in ipairs({{400,655},{620,660},{520,700},{860,690},{780,640}}) do buryTufts(c[1], c[2], veilC, 1.4) end

--@ chunk 68
local function h(y) return 0.34*(y-470)^2/3400 + 0.35 end
local S = stepsM
local far = mask(function(x, y)
  if y < 560 or y > 716 or x < 580 or x > 780 then return 0 end
  if S:at(x, y) < 0.5 then return 0 end
  local d = math.max(0.45 * h(y), 0.6)
  return (S:at(x, y - d) < 0.5) and 1 or 0
end)
local near = mask(function(x, y)
  if y < 580 or y > 716 or x < 580 or x > 780 then return 0 end
  if S:at(x, y) > 0.5 then return 0 end
  local d = math.max(0.3 * h(y), 0.6)
  return (S:at(x, y - d) > 0.5) and 1 or 0
end)
print(far:area(), near:area())
work(far, {hand="detail", pile=chamber, coverage=2, pressure={0.35,0.6}, load=0.5, fill=true, angle=0, tool={kind="round", width=1.2}})
work(near, {hand="detail", pile=snowTop, coverage=2.5, pressure={0.4,0.7}, load=0.7, fill=true, angle=0, tool={kind="round", width=1.2}})

--@ chunk 69
work(stepsM:grow(0.3), {hand="detail", pile=printDeep, coverage=2.2, pressure={0.4,0.6}, load=0.7, fill=true, angle=0, tool={kind="round", width=1.4}})
blend(stepsM:grow(1.2) * rect(560,540,240,180), {angle=0, tool={kind="badger", width=6}})

--@ chunk 70
local fx, fy = FX, FY
local r = brush{kind="round", width=1.6, point=1}
-- the stick, restated dark and firm, planted in the snow
r:load(coatP, 0.8)
r:stroke({{fx-9.4,fy-21.5},{fx-10.5,fy-10},{fx-11.4,fy+0.5}}, {pressure={0.75,0.7}})
-- the hand on the stick
local hand = ellipse(fx-9.2, fy-20.4, 1.3, 1.5)
work(hand, {hand="detail", pile=coatP, coverage=3, pressure={0.5,0.8}, load=0.8, fill=true, tool={kind="round", width=1}})
-- cool light from the upper sky along the hat and the left shoulder
local rim = pile{{"lead white",4},{"pale smalt",2},{"smalt",0.5},{"raw umber",0.3}, medium=0.08}
r:load(rim, 0.5)
r:stroke({{fx-3.2,fy-41.6},{fx-0.5,fy-42.9},{fx+2.4,fy-42.6}}, {pressure={0.22,0.1}})
r:stroke({{fx-5.6,fy-31.8},{fx-3,fy-33.8},{fx-0.5,fy-34.2}}, {pressure={0.2,0.08}})
-- the fold down the back of the coat, and the belt
r:load(chamber, 0.5)
r:stroke({{fx+0.8,fy-30},{fx+1.2,fy-20},{fx+1.6,fy-8}}, {pressure={0.25,0.1}})
r:stroke({{fx-6.2,fy-21.5},{fx,fy-21},{fx+6.4,fy-21.6}}, {pressure={0.22,0.15}})
-- a little snow kicked up at his boots and the shadow he stands in
local base = ellipse(fx, fy + 0.8, 8.5, 1.6) - figM
work(base, {hand="detail", pile=printDeep, coverage=2.5, pressure={0.4,0.7}, load=0.6, fill=true, angle=0, tool={kind="round", width=1.2}})

--@ chunk 71
local fx, fy = FX, FY
local m = (ribbon({{fx+0.8,fy-30},{fx+1.2,fy-20},{fx+1.6,fy-8}}, 2.4) + ribbon({{fx-6.2,fy-21.5},{fx,fy-21},{fx+6.4,fy-21.6}}, 2.4)) * figM:shrink(0.3)
work(m, {hand="detail", pile=coatP, coverage=5, pressure={0.7,1.0}, load=1, fill=true, tool={kind="round", width=1.2}})

--@ chunk 72
moonP = pile{{"lead white",6},{"chrome yellow",0.12},{"yellow ochre",0.1}, medium=0.05}
local mx, my, rr = 742, 128, 8.5
local cres = ellipse(mx, my, rr, rr) - ellipse(mx - 2.6, my - 3.4, rr*0.93, rr*0.93)
moonM = cres:soften(0.3)
work(moonM, {hand="detail", pile=moonP, coverage=4, pressure={0.6,0.9}, load=1, fill=true, tool={kind="round", width=1.2}})
-- crows
crowP = pile{{"bone black",1.5},{"raw umber",0.6},{"lead white",0.15}, medium=0.06}
local r = brush{kind="round", width=1.8, point=1}
r:load(crowP, 0.8)
local function flying(x, y, s, up)
  local wy = up and -1 or 0.4
  -- body
  r:stroke({{x - s*0.35, y + 0.2*s}, {x + s*0.4, y - 0.05*s}}, {pressure={0.75, 0.45}})
  -- wings, bent at the wrist
  r:stroke({{x, y}, {x - s*0.45, y + wy*s*0.55}, {x - s*1.0, y + wy*s*0.35 + s*0.2}}, {pressure={0.6, 0.05}})
  r:stroke({{x + 0.1*s, y}, {x + s*0.4, y + wy*s*0.6}, {x + s*0.95, y + wy*s*0.4 + s*0.25}}, {pressure={0.6, 0.05}})
end
flying(470, 205, 6.5, true)
flying(505, 228, 5.2, false)
flying(540, 196, 4.4, true)
flying(612, 250, 3.6, false)
flying(430, 262, 5.8, false)
-- two perched on the dead limbs of the oak
local function perched(x, y, s)
  local body = ellipse(x, y - s*0.45, s*0.32, s*0.5) + ellipse(x + s*0.18, y - s*1.0, s*0.2, s*0.2)
  work(body, {hand="detail", pile=crowP, coverage=4, pressure={0.6,0.9}, load=1, fill=true, tool={kind="round", width=0.9}})
  r:load(crowP, 0.7)
  r:stroke({{x + s*0.35, y - s*1.02}, {x + s*0.6, y - s*0.96}}, {pressure={0.4, 0.05}})
  r:stroke({{x - s*0.15, y - s*0.2}, {x - s*0.45, y + s*0.25}}, {pressure={0.5, 0.1}})
end
perched(236, 116, 6)
perched(289, 70, 5.4)

--@ chunk 73
rim = pile{{"lead white",4},{"pale smalt",2},{"smalt",0.5},{"raw umber",0.3}, medium=0.08}
-- the posts: shadowed right flank, cool light down the left, snow caps, drifts at the feet
local r = brush{kind="round", width=2.4, point=1}
r:load(chamber, 0.7)
r:stroke({{95.5,642},{97,610},{99.5,578}}, {pressure={0.5,0.3}})
r:stroke({{153.5,626},{154,600},{154.5,584}}, {pressure={0.45,0.3}})
r:stroke({{216,610},{217,590},{218,578}}, {pressure={0.4,0.3}})
r:load(rim, 0.5)
r:stroke({{87.5,640},{90,610},{94,578}}, {pressure={0.25,0.15}})
r:stroke({{147.8,624},{148.6,600},{149.5,586}}, {pressure={0.22,0.12}})
-- caps
local caps = ellipse(97, 574, 4.6, 2.2) + ellipse(216, 574, 3.4, 1.8) + poly({{147,583},{149,578},{152,580},{155,582},{151,584}}, true)
local railSnow = ribbon({{97,597.5},{124,603.3},{150,609.4},{170,614.4}}, {1.6,1.8,1.8,1.4})
work(caps + railSnow, {hand="detail", pile=snowTop, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0, tool={kind="round", width=1.2}})
-- drifts piled at the feet, a little on the windward left
local d1 = poly({{72,648},{82,640},{89,634},{97,636},{104,641},{112,648}}, true)
local d2 = poly({{136,630},{144,624},{150,620},{156,622},{162,626},{170,631}}, true)
local d3 = poly({{202,613},{208,608},{213,605},{219,607},{226,612}}, true)
local rail2 = poly({{160,618},{166,613},{172,612},{182,619}}, true)
work(d1 + d2 + d3 + rail2, {hand="detail", pile=fieldB, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0, tool={kind="round", width=1.5}})
r:load(snowTop, 0.5)
r:stroke({{74,647},{82,640},{89,634.5}}, {pressure={0.3,0.1}})
r:stroke({{138,629.5},{144,624},{150,620.5}}, {pressure={0.3,0.1}})

--@ chunk 74
local m = ellipse(93, 645, 22, 4.5) + ellipse(152, 627, 18, 3.8) + ellipse(214, 610, 14, 3.2)
m = m - postsM
work(m, {hand="detail", pile=fieldB, coverage=3, pressure={0.5,0.8}, load=0.9, fill=true, angle=0, tool={kind="round", width=1.6}})
blend((m + ellipse(93, 640, 22, 9) + ellipse(152, 622, 18, 8) + ellipse(214, 606, 14, 6)) - postsM, {angle=0, tool={kind="badger", width=8}})

--@ chunk 75
print(wait(4*24*60)); for _,p in ipairs({{93,645},{97,574},{650,640},{560,530},{893,480},{350,375},{742,128},{700,690}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 76
glazeCool = pile{{"smalt",1},{"pale smalt",1},{"raw umber",0.25},{"lead white",0.25},{"vermilion",0.03}, medium=0.75}
work(rect(880,672,120,44), {hand="glaze", pile=glazeCool, coverage=1.2, clip=true, angle=0})

--@ chunk 77
local b = brush{kind="flat", width=8, stiffness=0.6}
for pass = 1, 3 do
  for y = 672, 716, 5 do
    b:wipe(1)
    b:stroke({{876, y + pass}, {940, y + pass + 0.5}}, {pressure={0.8,0.8}})
    b:wipe(1)
    b:stroke({{936, y + pass}, {1004, y + pass + 0.5}}, {pressure={0.8,0.8}})
  end
end
print(b:fullness())

--@ chunk 78
local b = brush{kind="flat", width=6, stiffness=0.7}
for pass = 1, 2 do
  for x = 874, 1004, 4 do
    b:wipe(1)
    b:stroke({{x + pass*2, 668}, {x + pass*2 + 1, 716}}, {pressure={0.9,0.9}})
  end
  for y = 670, 716, 3 do
    b:wipe(1)
    b:stroke({{872, y + pass}, {1004, y + pass}}, {pressure={0.9,0.9}})
  end
end

--@ chunk 79
glazeThin = pile{{"smalt",1},{"pale smalt",1.5},{"raw umber",0.3},{"lead white",0.6},{"vermilion",0.03}, medium=0.92}
work(rect(880,676,120,40), {hand="glaze", pile=glazeThin, coverage=1, clip=true, angle=0, load=0.25, pressure={0.2,0.35}})

--@ chunk 80
local b = brush{kind="flat", width=6, stiffness=0.7}
for pass = 1, 2 do
  for y = 672, 716, 3 do
    b:wipe(1)
    b:stroke({{872, y + pass}, {1004, y + pass}}, {pressure={0.9,0.9}})
  end
  for x = 874, 1004, 4 do
    b:wipe(1)
    b:stroke({{x + pass*2, 668}, {x + pass*2 + 1, 716}}, {pressure={0.9,0.9}})
  end
end

--@ chunk 81
printDark = pile{{"lead white",1.6},{"pale smalt",2},{"smalt",1.0},{"raw umber",0.4},{"vermilion",0.03}, medium=0.1}
local function h(y) return 0.34*(y-470)^2/3400 + 0.35 end
local S = stepsM
local upper = mask(function(x, y)
  if y < 556 or y > 716 or x < 570 or x > 790 then return 0 end
  if S:at(x, y) < 0.5 then return 0 end
  local d = math.max(0.55 * h(y), 0.5)
  return (S:at(x, y - d) < 0.5) and 1 or 0
end)
work(upper, {hand="detail", pile=printDark, coverage=3, pressure={0.5,0.8}, load=0.8, fill=true, angle=0, tool={kind="round", width=1.2}})

--@ chunk 82
local n=0; for y=560,714,1 do for x=570,790,0.5 do if stepsM:at(x,y)>0.5 then n=n+1 end end end print(n, stepsM:area())

--@ chunk 83
local printMid = pile{{"lead white",2.6},{"pale smalt",2},{"smalt",0.7},{"raw umber",0.3},{"vermilion",0.03}, medium=0.08}
work(stepsM:shrink(0.2), {hand="detail", pile=printMid, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0, tool={kind="round", width=1.2}})
local function h(y) return 0.34*(y-470)^2/3400 + 0.35 end
local S = stepsM
local upper = mask(function(x, y)
  if y < 556 or y > 716 or x < 570 or x > 790 then return 0 end
  if S:at(x, y) < 0.5 then return 0 end
  local d = math.max(0.4 * h(y), 0.5)
  return (S:at(x, y - d) < 0.5) and 1 or 0
end)
work(upper, {hand="detail", pile=printDark, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0, tool={kind="round", width=1.0}})

--@ chunk 84
local best; for y=680,714,0.5 do for x=690,760,0.5 do if stepsM:at(x,y)>0.9 then best={x,y} break end end if best then break end end print(best[1],best[2]); print(stepsM:shrink(0.2):area(), drying(best[1],best[2]+2))

--@ chunk 85
printDark2 = pile{{"lead white",1},{"pale smalt",2},{"smalt",1.2},{"raw umber",0.45},{"bone black",0.05},{"vermilion",0.03}, medium=0.08}
local function h(y) return 0.34*(y-470)^2/3400 + 0.35 end
local S = stepsM
local upper = mask(function(x, y)
  if y < 552 or y > 716 or x < 565 or x > 790 then return 0 end
  if S:at(x, y) < 0.5 then return 0 end
  local d = math.max(0.6 * h(y), 0.5)
  return (S:at(x, y - d) < 0.5) and 1 or 0
end)
work(upper, {hand="detail", pile=printDark2, coverage=4, pressure={0.6,0.9}, load=1, fill=true, angle=0, tool={kind="round", width=1.0}})

--@ chunk 86
glazeSoft = pile{{"pale smalt",1},{"raw umber",0.12},{"lead white",0.25},{"vermilion",0.02}, medium=0.85}
local m = rect(880,676,120,40)
work(m, {hand="glaze", pile=glazeSoft, coverage=1.2, clip=true, angle=0, load=0.5, pressure={0.3,0.5}})
blend(m, {angle=0})

--@ chunk 87
local b = brush{kind="flat", width=6, stiffness=0.7}
for pass = 1, 2 do
  for y = 672, 716, 3 do
    b:wipe(1)
    b:stroke({{872, y + pass}, {1004, y + pass}}, {pressure={0.9,0.9}})
  end
  for x = 874, 1004, 4 do
    b:wipe(1)
    b:stroke({{x + pass*2, 668}, {x + pass*2 + 1, 716}}, {pressure={0.9,0.9}})
  end
end

--@ chunk 88
print(wait(5*24*60)); for _,p in ipairs({{93,645},{700,682},{560,530},{600,570},{880,700}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 89
glazeM = land * below({{-10,528},{1010,528}})
work(glazeM, {hand="glaze", pile=glazeSoft, coverage=1.3, clip=true, angle=0, pressure={0.3,0.5}, load=0.5,
  load_at=function(x, y) return clamp((y - 535) / 150, 0, 1) * 0.6 + 0.04 end})
blend(glazeM, {angle=0})

--@ chunk 90
print(wait(5*24*60)); for _,p in ipairs({{300,700},{700,650},{500,560},{93,645}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 91
local m = land * below({{-10,590},{1010,590}})
work(m, {hand="glaze", pile=glazeSoft, coverage=1.3, clip=true, angle=0, pressure={0.3,0.5}, load=0.5,
  load_at=function(x, y) return clamp((y - 600) / 110, 0, 1) * 0.65 + 0.03 end})
-- the drift pads at the fence feet sit in the same half-light
local pads = (ellipse(93, 643, 24, 7) + ellipse(152, 625, 20, 6) + ellipse(214, 608, 15, 5)) - postsM
work(pads, {hand="glaze", pile=glazeSoft, coverage=1.2, clip=true, angle=0, pressure={0.3,0.5}, load=0.6})
blend(m, {angle=0})

--@ chunk 92
trunkMid = pile{{"raw umber",1.4},{"lead white",1.3},{"bone black",0.2},{"yellow ochre",0.1}, medium=0.15}
local burr = ellipse(203, 371, 11, 10)
work(burr, {hand="detail", pile=trunkMid, coverage=3, fill=true, angle=85, pressure={0.6,0.9}, load=0.9, tool={kind="round", width=2}})
blend(ellipse(203, 371, 13, 12), {angle=85})
-- a swelling suggested by a curved crack and a shadow underneath
local b = brush{kind="round", width=1.3}
b:load(barkDark, 0.8)
b:stroke({{195,366},{199,372},{204,376},{210,375}}, {pressure={0.5,0.7,0.5,0.2}})
b:stroke({{197,379},{203,381},{209,380}}, {pressure={0.4,0.6,0.3}})
b:stroke({{201,362},{203,368},{202,374}}, {pressure={0.3,0.5,0.2}})

--@ chunk 93
local b = brush{kind="flat", width=4, stiffness=0.7}
for pass=1,2 do
  for x = 190, 217, 2 do
    b:wipe(1)
    b:stroke({{x+pass*0.7, 358}, {x+pass*0.7+1, 385}}, {pressure={0.9,0.9}})
  end
end

--@ chunk 94
print(wait(4*24*60), drying(203,371))
local m = rect(186, 355, 34, 34):blur(2) * oakThickM
work(m, {hand="detail", pile=barkDark, coverage=3, fill=true, angle=88, pressure={0.6,0.9}, load=0.9, tool={kind="round", width=1.6}})

--@ chunk 95
local function U(a,b) return a + (b-a)*math.random() end
local sl = rect(211, 360, 9, 28) * oakThickM:grow(1.5)
work(sl, {hand="detail", pile=barkDark, coverage=3, fill=true, angle=88, pressure={0.6,0.9}, load=0.9, tool={kind="round", width=1.4}})
math.randomseed(77)
local b = brush{kind="round", width=0.9}
for i=1,7 do
  local x = 188 + i*3.6 + U(-1,1)
  b:load(barkLight, 0.5)
  b:stroke({{x, 352 + U(0,4)}, {x + U(-1.5,1.5), 370}, {x + U(-2,2), 392 - U(0,4)}}, {pressure={0.2,0.45,0.15}})
end

--@ chunk 96
print(wait(10*24*60)); for _,p in ipairs({{203,371},{215,370},{600,690},{93,645}}) do print(drying(p[1],p[2])) end

--@ chunk 97
varnish{coats=0.3}
