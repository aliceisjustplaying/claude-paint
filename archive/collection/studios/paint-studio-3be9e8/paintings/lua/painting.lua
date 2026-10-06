-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=460, aspect=1.25, linen={17,13}, seed=7, ground={{pile={{"lead white",1}}, um=120, apply="knife", texture=0.3}, {pile={{"lead white",3},{"yellow ochre",1},{"raw umber",1}}, um=25, apply="brush"}}}

--@ chunk 2
c = chalk()
-- table edges
c:sketch({{0,522},{300,520},{700,519},{1000,521}}, {pressure=0.3})
c:sketch({{0,690},{500,689},{1000,691}}, {pressure=0.3})
-- jug
jugL = {{300,192},{296,215},{302,240},{275,300},{240,380},{236,440},{252,510},{280,570},{300,600}}
jugR = {{420,192},{424,215},{418,240},{445,300},{480,380},{484,440},{468,510},{440,570},{420,600}}
c:sketch(jugL, {pressure=0.35})
c:sketch(jugR, {pressure=0.35})
c:sketch({{300,600},{360,610},{420,600}}, {pressure=0.3})
-- rim ellipse and spout
c:sketch({{300,192},{330,180},{360,177},{400,180},{420,192},{390,202},{360,204},{320,200},{300,192}}, {pressure=0.3})
c:sketch({{300,192},{275,178},{262,172}}, {pressure=0.3})
-- handle
c:sketch({{422,225},{470,222},{510,245},{520,300},{500,360},{475,400}}, {pressure=0.35})
c:sketch({{425,250},{465,248},{490,270},{493,310},{478,355},{470,375}}, {pressure=0.3})
-- lemons
c:sketch({{540,595},{560,560},{610,545},{660,555},{680,590},{660,625},{600,638},{555,625},{540,595}}, {pressure=0.3})
c:sketch({{650,555},{675,525},{720,515},{765,528},{780,560},{770,580}}, {pressure=0.3})
-- knife
c:sketch({{560,670},{700,690},{800,712}}, {pressure=0.3})
c:sketch({{560,670},{700,702},{800,724}}, {pressure=0.3})
c:sketch({{790,705},{870,735},{940,762},{950,778},{930,780},{860,757},{785,728}}, {pressure=0.35})

--@ chunk 3
local pts = {}
for _, p in ipairs(jugL) do pts[#pts+1] = p end
pts[#pts+1] = {360, 608}
for i = #jugR, 1, -1 do pts[#pts+1] = jugR[i] end
pts[#pts+1] = {360, 176}
jugBody = poly(pts, true)
spout = poly({{300,186},{268,170},{258,172},{285,195},{302,205}}, true)
handle = ribbon({{420,236},{468,234},{505,256},{510,300},{492,355},{474,392}}, {22,22,20,18,16,14})
jug = jugBody + spout + handle
lemon1 = ellipse(610, 592, 72, 46)
lemon2 = poly({{652,560},{672,527},{720,515},{765,527},{782,558},{770,582},{720,590},{668,582}}, true)
lemons = lemon1 + lemon2
table_top = rect(0, 521, 1000, 169)
front = rect(0, 690, 1000, 110)
wall = rect(0, 0, 1000, 521)
print(jug:area(), lemons:area())

--@ chunk 4
bgDark = pile{{"raw umber",3},{"bone black",1},{"red earth",0.6}, medium=0.2}
bgWarm = pile{{"raw umber",3},{"yellow ochre",1.2},{"lead white",0.6},{"bone black",0.4}, medium=0.15}
local n = noise{seed=11, period=300}
local m = wall - jug:grow(2)
work(m, {hand="broad", pile=bgDark, coverage=2.2, angle=function(x,y) return 0.6 + 0.5*n(x,y) end, edge="firm", fill=true})

--@ chunk 5
bgOlive = pile{{"raw umber",2},{"bone black",0.7},{"yellow ochre",0.8},{"green earth",1},{"lead white",0.4}, medium=0.1}
bgLight = pile{{"raw umber",2},{"yellow ochre",1.2},{"green earth",0.8},{"lead white",1.4},{"bone black",0.2}, medium=0.1}
local m = wall - jug:grow(2)
work(m, {hand="body", tool="filbert 14", pile=bgOlive, coverage=1.6, angle=function(x,y) return 0.9 + 0.4*math.sin(x/170 + y/230) end, edge="firm", fill=true, length={30,80}})
local n = noise{seed=5, period=180}
local light = mask(function(x,y) local d = math.sqrt(((x-760)/330)^2 + ((y-250)/260)^2); return clamp(1.1 - d + 0.25*n(x,y), 0, 1) end) * m
work(light, {hand="scumble", tool="filbert 14", pile=bgLight, coverage=1.2, threshold=0.25, edge="soft"})
blend(m, {angle=0.9})

--@ chunk 6
print(drying(750,250), drying(100,100))

--@ chunk 7
bgMid = pile{{"raw umber",2},{"bone black",0.5},{"yellow ochre",1},{"green earth",1},{"lead white",0.8}, medium=0.1}
local m = wall - jug:grow(2)
local zone = mask(function(x,y) local d = math.sqrt(((x-760)/420)^2 + ((y-260)/330)^2); return clamp(1.3 - d, 0, 1) end) * m
work(zone, {hand="body", tool="filbert 14", pile=bgMid, coverage=1.5, threshold=0.2, angle=function(x,y) return 1.0 + 0.3*math.sin(x/140) end, edge="soft", length={30,70}})
blend(zone:grow(30) * m, {angle=0.4})
blend(zone:grow(30) * m, {angle=2.0})

--@ chunk 8
local m = wall - jug:grow(3)
blend(m, {angle=0.7})
blend(m, {angle=2.3})

--@ chunk 9
tTop = pile{{"lead white",2},{"yellow ochre",1},{"raw umber",1.2},{"bone black",0.15}}
tBack = pile{{"lead white",1.2},{"yellow ochre",0.8},{"raw umber",1.6},{"bone black",0.3}}
tFront = pile{{"raw umber",2},{"yellow ochre",0.6},{"bone black",0.6},{"lead white",0.5}}
local objs = (jug + lemons):grow(1)
local top = table_top - objs
work(top, {hand="body", tool="filbert 12", pile=tTop, coverage=2, angle=0, angle_jitter=0.05, length={50,120}, edge="firm", fill=true})
local backband = rect(0, 521, 1000, 50):soften(20) * top
work(backband, {hand="body", tool="filbert 10", pile=tBack, coverage=1.2, angle=0, length={40,100}, threshold=0.35, edge="soft"})
work(front, {hand="body", tool="filbert 12", pile=tFront, coverage=2, angle=0, angle_jitter=0.04, length={60,140}, edge="firm", fill=true})

--@ chunk 10
local objs = (jug + lemons):grow(2)
blend(table_top - objs, {angle=0})
blend(front, {angle=0})

--@ chunk 11
jf = form{ {body.ellipsoid({362,425,0},{124,180,110})}, {body.ellipsoid({360,235,0},{64,62,55})}, light={from={-1,-0.6}, front=0.55, ambient=0.2} }
for _, pt in ipairs({{280,380},{360,420},{430,420},{470,450},{360,560},{320,230},{410,240}}) do
  print(pt[1], pt[2], string.format("%.2f", jf:value(pt[1], pt[2])))
end

--@ chunk 12
jHalf = pile{{"lead white",4},{"yellow ochre",0.45},{"raw umber",0.35},{"green earth",0.3}}
jShadow = pile{{"lead white",2},{"raw umber",1},{"yellow ochre",0.4},{"bone black",0.2},{"green earth",0.5}}
jLight = pile{{"lead white",6},{"yellow ochre",0.35}}
local jb = jugBody + spout
local val = mask(function(x,y) return jf:value(x,y) end)
jShadowM = jb * val:map(function(v) return clamp((0.42 - v)/0.1, 0, 1) end)
jLightM = jb * val:map(function(v) return clamp((v - 0.6)/0.1, 0, 1) end)
local across = function(x,y) return 0.12*math.sin((y-420)/300) end
work(jb, {hand="body", tool="filbert 10", pile=jHalf, coverage=2.2, angle=0, angle_jitter=0.15, curve={0.3,0.1}, length={25,60}, edge="firm", fill=true})
work(jShadowM, {hand="body", tool="filbert 9", pile=jShadow, coverage=1.8, angle=0.2, angle_jitter=0.2, curve={0.3,0.1}, length={20,50}, edge="soft", threshold=0.4})
work(jLightM, {hand="body", tool="filbert 9", pile=jLight, coverage=2.0, angle=-0.1, angle_jitter=0.2, curve={0.3,0.1}, length={20,50}, edge="soft", threshold=0.4})

--@ chunk 13
local jb = jugBody + spout
blend(jb, {angle=0.15})
blend(jb, {angle=1.4})

--@ chunk 14
lf = form{ {body.ellipsoid({610,592,0},{72,46,45}):turn({610,592,0},0,0,-0.08)}, {body.ellipsoid({718,553,0},{64,38,40}):turn({718,553,0},0,0,0.1)}, light={from={-1,-0.6}, front=0.5, ambient=0.2} }
lMid = pile{{"chrome yellow",3},{"yellow ochre",0.6},{"lead white",0.8}}
lShadow = pile{{"yellow ochre",2},{"raw umber",1},{"green earth",1},{"chrome yellow",0.6}}
lLight = pile{{"chrome yellow",2},{"lead white",2}}
local val = mask(function(x,y) return lf:value(x,y) end)
work(lemons, {hand="body", tool="filbert 7", pile=lMid, coverage=2.2, angle=function(x,y) return lf:across(x,y) end, length={15,35}, edge="firm", fill=true})
local sh = lemons * val:map(function(v) return clamp((0.4 - v)/0.12, 0, 1) end)
work(sh, {hand="body", tool="filbert 6", pile=lShadow, coverage=1.4, angle=function(x,y) return lf:across(x,y) end, length={12,30}, edge="soft", threshold=0.35})
local li = lemons * val:map(function(v) return clamp((v - 0.68)/0.1, 0, 1) end)
work(li, {hand="body", tool="filbert 5", pile=lLight, coverage=1.4, angle=function(x,y) return lf:across(x,y) end, length={10,25}, edge="soft", threshold=0.4})

--@ chunk 15
blend(lemon1, {angle=-0.5})
blend(lemon2, {angle=-0.5})

--@ chunk 16
print(wait(24*60)); print(drying(360,420), drying(610,592), drying(100,100), drying(500,600), drying(500,750))

--@ chunk 17
print(wait(18*60)); print(drying(360,420), drying(610,592), drying(100,100), drying(500,600), drying(500,750))

--@ chunk 18
print(wait(24*60)); print(drying(360,420), drying(610,592), drying(100,100), drying(800,300))

--@ chunk 19
jDeep = pile{{"lead white",1.5},{"raw umber",1},{"bone black",0.35},{"green earth",0.6},{"yellow ochre",0.3}}
local jb = jugBody + spout
jVal = mask(function(x,y) return jf:value(x,y) end)
local sh = jb * jVal:map(function(v) return v < 0.42 and 1 or 0 end)
sh = sh:soften(4)
work(sh, {hand="body", tool="filbert 8", pile=jDeep, coverage=2.5, angle=function(x,y) return jf:across(x,y) end, length={20,45}, edge="firm", fill=true})

--@ chunk 20
local jb = jugBody + spout
jSil = jf:silhouette{}
jEdgeL = (jb - jSil) * rect(0, 150, 362, 480)
jLightEdge = pile{{"lead white",5},{"yellow ochre",0.5},{"raw umber",0.2},{"green earth",0.1}}
work(jEdgeL:grow(3) * jb, {hand="body", tool="filbert 6", pile=jLightEdge, coverage=2.5, angle=function(x,y) return jf:across(x,y) + 0.6 end, length={12,30}, edge="firm", fill=true})
-- terminator band
local band = jb * jSil * jVal:map(function(v) return clamp(1 - math.abs(v - 0.42)/0.07, 0, 1) end)
blend(band:grow(4), {tool={kind="badger", width=12}, angle=function(x,y) return jf:across(x,y) end})

--@ chunk 21
print(wait(40*60)); print(drying(420,420), drying(300,300), drying(270,180))

--@ chunk 22
print(wait(20*60)); print(drying(420,420), drying(300,300), drying(270,180))

--@ chunk 23
print(wait(36*60)); print(drying(420,420), drying(300,300), drying(270,180))

--@ chunk 24
function edgeX(list, y)
  if y <= list[1][2] then return list[1][1] end
  for i = 1, #list - 1 do
    local a, b = list[i], list[i+1]
    if y >= a[2] and y <= b[2] then
      local u = (y - a[2]) / (b[2] - a[2])
      return lerp(a[1], b[1], u)
    end
  end
  return list[#list][1]
end
jp = {
  pile{{"lead white",8},{"yellow ochre",0.2}},
  pile{{"lead white",6},{"yellow ochre",0.4},{"raw umber",0.1}},
  pile{{"lead white",5},{"yellow ochre",0.5},{"raw umber",0.3},{"green earth",0.2}},
  pile{{"lead white",4},{"yellow ochre",0.5},{"raw umber",0.6},{"green earth",0.4},{"bone black",0.05}},
  pile{{"lead white",3},{"raw umber",0.9},{"yellow ochre",0.4},{"green earth",0.5},{"bone black",0.15}},
  pile{{"lead white",2},{"raw umber",1},{"green earth",0.6},{"bone black",0.3},{"yellow ochre",0.3}},
  pile{{"lead white",3},{"yellow ochre",0.8},{"raw umber",0.7},{"red earth",0.1}},
}
function jIdx(t, y)
  local i
  if t < 0.06 then i = 3
  elseif t < 0.14 then i = 2
  elseif t < 0.34 then i = (y > 290 and y < 470) and 1 or 2
  elseif t < 0.44 then i = 2
  elseif t < 0.52 then i = 3
  elseif t < 0.60 then i = 4
  elseif t < 0.68 then i = 5
  elseif t < 0.86 then i = 6
  else i = (y > 360) and 7 or 5 end
  if y > 530 and i < 6 then i = i + 1 end
  if y > 570 and i < 6 then i = i + 1 end
  return i
end

--@ chunk 25
local segs = {}
local nstrip = 20
for s = 0, nstrip - 1 do
  local t = (s + 0.5) / nstrip + rand(-0.01, 0.01)
  local y = 198 + rand(0, 6)
  local cur, pts = nil, {}
  local function flush()
    if cur and #pts >= 2 then segs[#segs+1] = {idx=cur, pts=pts} end
  end
  while y <= 600 do
    local xl, xr = edgeX(jugL, y), edgeX(jugR, y)
    local x = lerp(xl, xr, t)
    local i = jIdx(t + rand(-0.015, 0.015), y)
    if i ~= cur or #pts > 7 + math.random(0, 3) then
      local last = pts[#pts]
      flush()
      pts = last and {last} or {}
      cur = i
    end
    pts[#pts+1] = {x, y}
    y = y + 9
  end
  flush()
end
local order = {6, 5, 7, 4, 3, 2, 1}
local big = brush("filbert", 13)
local small = brush("filbert", 7)
local clipm = (jugBody + spout):grow(1)
local n = 0
for _, k in ipairs(order) do
  for _, sg in ipairs(segs) do
    if sg.idx == k then
      local b = (sg.pts[1][2] < 290) and small or big
      b:reload(jp[k], 0.75)
      b:stroke(sg.pts, {pressure={0.65, 0.55}, ramps={0.1, 0.25}, orient="across", clip=clipm})
      n = n + 1
    end
  end
end
print(n)

--@ chunk 26
blend(jugBody, {angle=1.57, tool={kind="badger", width=20}})

--@ chunk 27
lemon1 = ellipse(610,592,72,46):soften(1.5) + ribbon({{668,586},{681,584},{690,583}},{22,12,5}) + ribbon({{552,600},{542,601},{535,602}},{18,10,4})
lemon2 = poly({{652,560},{672,527},{720,515},{765,527},{782,550},{775,575},{720,590},{668,582}}, true) + ribbon({{775,548},{788,545},{796,544}},{20,11,5})
lemons = lemon1 + lemon2
lBase = pile{{"chrome yellow",3},{"lead white",1},{"yellow ochre",0.3}}
local b = brush("filbert", 8)
work(lemon2 - lemon1, {tool=b, pile=lBase, coverage=2.5, angle=function(x,y) return lf:across(x,y) end, length={15,35}, edge="firm", clip=true, fill=true})
work(lemon1, {tool=b, pile=lBase, coverage=2.5, angle=function(x,y) return lf:across(x,y) end, length={15,35}, edge="firm", clip=true, fill=true})

--@ chunk 28
print(wait(48*60)); print(drying(420,420), drying(300,300), drying(610,592), drying(720,550))

--@ chunk 29
function jT(x, y)
  local xl, xr = edgeX(jugL, y), edgeX(jugR, y)
  return (x - xl) / math.max(1, xr - xl)
end
local jb = jugBody:grow(2)
gzA = pile{{"raw umber",1},{"bone black",0.5},{"green earth",0.8},{"lead white",0.7}, medium=0.45}
local g1 = jb * mask(function(x,y)
  if y < 180 or y > 615 then return 0 end
  local t = jT(x,y)
  local s = smoothstep(0.40, 0.58, t)
  local b = smoothstep(500, 600, y) * 0.8
  return math.max(s, b * smoothstep(0.1, 0.4, t))
end)
work(g1, {hand="glaze", pile=gzA, clip=g1, angle=1.5708, angle_jitter=0.08, length={60,160}, coverage=1.6})

--@ chunk 30
local jb = jugBody:grow(1)
local zone = jb * mask(function(x,y) if y < 180 or y > 615 then return 0 end; return smoothstep(0.3, 0.45, jT(x,y)) + smoothstep(480,560,y) end):map(function(v) return math.min(v,1) end)
blend(zone, {angle=1.5708, tool={kind="badger", width=24}})
blend(zone, {angle=0.2, tool={kind="badger", width=24}})

--@ chunk 31
local pts = {}
for _, p in ipairs(jugL) do pts[#pts+1] = {p[1]+95, p[2]+18} end
for i = #jugR, 1, -1 do pts[#pts+1] = {jugR[i][1]+95, jugR[i][2]+18} end
local wsh = (poly(pts, true):blur(18) * wall - jug:grow(1)) * rect(0,0,1000,521)
wallShadow = wsh
shG = pile{{"raw umber",1},{"bone black",0.5},{"green earth",0.4}, medium=0.55}
work(wsh, {hand="glaze", pile=shG, clip=wsh, angle=1.2, length={40,120}, coverage=2})
blend(wsh:grow(10) * wall - jug:grow(2), {angle=0.8, tool={kind="badger", width=24}})

--@ chunk 32
local z = (wallShadow:grow(45) * wall) - jug:grow(2)
blend(z, {angle=0.3})
blend(z, {angle=1.9})

--@ chunk 33
local m = wall - jug:grow(2)
blend(m, {angle=0.6})
blend(m, {angle=2.2})

--@ chunk 34
print(wait(36*60)); print(drying(420,420), drying(530,400), drying(610,592), drying(720,550))

--@ chunk 35
local jb = jugBody:grow(1)
function jZone(t0, t1, t2, t3, y0, y1)
  return jb * mask(function(x,y)
    if y < y0 or y > y1 then return 0 end
    local t = jT(x,y)
    local ey = smoothstep(y0, y0+25, y) * (1 - smoothstep(y1-25, y1, y))
    return smoothstep(t0, t1, t) * (1 - smoothstep(t2, t3, t)) * ey
  end)
end
local A = jZone(-0.1, 0.0, 0.5, 0.62, 170, 610)
work(A, {hand="scumble", pile=jp[2], clip=A, coverage=1.6, angle=1.5708, angle_jitter=0.3, load=0.5})
local B = jZone(0.1, 0.18, 0.36, 0.46, 250, 520)
work(B, {hand="scumble", pile=jp[1], clip=B, coverage=1.3, angle=1.5708, angle_jitter=0.3, load=0.5})
local C = jZone(0.48, 0.55, 0.66, 0.74, 200, 600)
work(C, {hand="scumble", pile=jp[4], clip=C, coverage=1.2, angle=1.5708, angle_jitter=0.3, load=0.4})
local D = jZone(0.84, 0.92, 1.05, 1.1, 340, 600)
work(D, {hand="scumble", tool="filbert 6", pile=jp[7], clip=D, coverage=1.2, angle=1.5708, angle_jitter=0.3, load=0.35})

--@ chunk 36
function strips(t0, t1, p, b, y0, y1, n, opt)
  opt = opt or {}
  local clipm = opt.clip or jugBody:grow(1)
  for s = 0, n - 1 do
    local t = lerp(t0, t1, (s + 0.5) / n) + rand(-0.008, 0.008)
    local y = y0 + rand(0, 10)
    while y < y1 do
      local len = rand(45, 90)
      local pts = {}
      local yy = y
      while yy <= math.min(y + len, y1) do
        pts[#pts+1] = {lerp(edgeX(jugL, yy), edgeX(jugR, yy), t), yy}
        yy = yy + 6
      end
      if #pts >= 2 then
        b:reload(p, opt.load or 0.7)
        b:stroke(pts, {pressure=opt.pressure or {0.6, 0.45}, ramps={0.15, 0.3}, orient="across", clip=clipm})
      end
      y = y + len * rand(0.75, 0.95)
    end
  end
end
jDark = pile{{"lead white",1.5},{"raw umber",1},{"green earth",0.7},{"bone black",0.35},{"yellow ochre",0.3}}
local b = brush("filbert", 10)
strips(0.68, 0.9, jDark, b, 215, 600, 5)
strips(0.60, 0.70, jp[5], b, 205, 600, 3)
strips(0.50, 0.61, jp[4], b, 200, 600, 3)

--@ chunk 37
blend(jugBody, {angle=1.5708, tool={kind="badger", width=16}}); blend(jugBody*mask(function(x,y) return smoothstep(0.35,0.5,jT(x,y)) end), {angle=0.1, tool={kind="badger", width=16}})

--@ chunk 38
print(wait(48*60)); print(drying(420,420), drying(470,500), drying(300,300))

--@ chunk 39
print(wait(48*60)); print(drying(420,420), drying(470,500), drying(300,300))

--@ chunk 40
gz1 = pile{{"raw umber",1},{"green earth",1},{"bone black",0.25}, medium=0.6}
local jb = jugBody:grow(1)
local G1 = jb * mask(function(x,y)
  if y < 185 or y > 612 then return 0 end
  local t = jT(x,y)
  return math.max(smoothstep(0.5, 0.62, t), smoothstep(545, 590, y) * smoothstep(0.05, 0.3, t))
end)
work(G1, {hand="glaze", pile=gz1, clip=jb, angle=1.5708, angle_jitter=0.1, length={40,90}, coverage=3, threshold=0.5})
local Z = jb * mask(function(x,y) if y < 180 then return 0 end; return math.max(smoothstep(0.3, 0.42, jT(x,y)), smoothstep(500,540,y)) end)
blend(Z, {angle=1.5708, tool={kind="badger", width=18}})

--@ chunk 41
local b = brush{kind="flat", width=14, stiffness=0.6}
local jb = jugBody:grow(1)
local n = 0
for pass = 1, 2 do
for s = 0, 13 do
  local t = lerp(0.42, 1.0, (s + 0.5) / 14) + rand(-0.01, 0.01)
  local y = 190 + rand(0, 20)
  while y < 610 do
    local len = rand(50, 90)
    local pts = {}
    local yy = y
    while yy <= math.min(y + len, 612) do
      pts[#pts+1] = {lerp(edgeX(jugL, yy), edgeX(jugR, yy), t), yy}
      yy = yy + 6
    end
    if #pts >= 2 then
      b:stroke(pts, {pressure={0.7, 0.6}, ramps={0.1, 0.1}, orient="across", clip=jb})
      b:wipe(1)
      n = n + 1
    end
    y = y + len * 0.85
  end
end
end
print(n)

--@ chunk 42
local jb = jugBody:grow(1)
local Z = jb * mask(function(x,y) if y < 180 then return 0 end; return math.max(smoothstep(0.22, 0.4, jT(x,y)), smoothstep(520,560,y)) end)
blend(Z, {angle=0.15, tool={kind="badger", width=18}})
blend(Z, {angle=1.5708, tool={kind="badger", width=18}})

--@ chunk 43
lz = pile{{"yellow ochre",1},{"raw umber",0.5},{"green earth",1}, medium=0.65}
local L = lemons:grow(1)
local S = L * mask(function(x,y) return lf:value(x,y) end):map(function(v) return 1 - smoothstep(0.3, 0.5, v) end)
work(S, {hand="glaze", tool={kind="filbert", width=10, stiffness=0.3}, pile=lz, clip=L, angle=function(x,y) return lf:across(x,y) end, length={20,45}, coverage=2.5, threshold=0.4})
blend(L, {angle=function(x,y) return lf:across(x,y) end, tool={kind="badger", width=12}})

--@ chunk 44
local b = brush{kind="flat", width=10, stiffness=0.6}
local L = lemons:grow(2)
local n = 0
for pass = 1, 2 do
  for y = 515, 645, 7 do
    for x0 = 520, 800, 45 do
      local a = pass == 1 and -0.15 or 0.15
      local x1 = x0 + 50
      local pts = {{x0, y + (x0-660)*a*0.1}, {x1, y + (x1-660)*a*0.1}}
      if L:at(x0+25, y) > 0.1 then
        b:stroke(pts, {pressure={0.7,0.6}, ramps={0.1,0.1}, orient="across", clip=L})
        b:wipe(1)
        n = n + 1
      end
    end
  end
end
print(n)

--@ chunk 45
blend(lemons:grow(1), {angle=-0.4, tool={kind="badger", width=14}})

--@ chunk 46
print(wait(60*60)); print(drying(610,592), drying(720,550), drying(420,420))

--@ chunk 47
lp = {
  pile{{"yellow ochre",2},{"raw umber",1.1},{"green earth",0.7},{"chrome yellow",0.5}},
  pile{{"yellow ochre",2},{"chrome yellow",1.2},{"raw umber",0.4},{"green earth",0.3}},
  pile{{"chrome yellow",3},{"yellow ochre",1}},
  pile{{"chrome yellow",3},{"lead white",0.6}},
  pile{{"chrome yellow",2},{"lead white",2.2}},
}
function lemonPaint(cx, cy, rx, ry, rot, clipm, b, step)
  local Lx, Ly, Lz = -1, -0.75, 0.9
  local ln = math.sqrt(Lx*Lx+Ly*Ly+Lz*Lz); Lx, Ly, Lz = Lx/ln, Ly/ln, Lz/ln
  local c, s = math.cos(rot), math.sin(rot)
  local function P(u, v)
    local x = u * rx
    local y = v * ry + 0.12 * ry * (1 - u*u)
    return {cx + c*x - s*y, cy + s*x + c*y}
  end
  local function idx(u, v)
    local w = math.max(0, 1 - u*u - v*v)
    local nz = math.sqrt(w)
    local d = u*Lx + v*Ly + nz*Lz
    local k
    if d < 0.12 then k = 1 elseif d < 0.35 then k = 2 elseif d < 0.62 then k = 3 elseif d < 0.85 then k = 4 else k = 5 end
    if v > 0.75 and k <= 2 and u > -0.5 then k = 2 end
    return k
  end
  local segs = {}
  local v = -0.97
  while v < 0.97 do
    local vv = v + rand(-0.03, 0.03)
    local umax = math.sqrt(math.max(0, 1 - vv*vv)) + 0.05
    local cur, pts = nil, {}
    local u = -umax
    while u <= umax do
      local k = idx(u + rand(-0.04,0.04), vv)
      if k ~= cur or #pts > 5 then
        if cur and #pts >= 2 then segs[#segs+1] = {k=cur, pts=pts} end
        pts = (#pts > 0) and {pts[#pts]} or {}
        cur = k
      end
      pts[#pts+1] = P(u, vv)
      u = u + 0.1
    end
    if cur and #pts >= 2 then segs[#segs+1] = {k=cur, pts=pts} end
    v = v + step
  end
  for k = 1, 5 do
    for _, sg in ipairs(segs) do
      if sg.k == k then
        b:reload(lp[k], 0.7)
        b:stroke(sg.pts, {pressure={0.6, 0.5}, ramps={0.15, 0.3}, orient="across", clip=clipm})
      end
    end
  end
  return #segs
end
local b = brush("filbert", 7)
print(lemonPaint(722, 553, 68, 38, 0.05, lemon2:grow(1) - lemon1, b, 0.13))
print(lemonPaint(610, 592, 78, 47, -0.08, lemon1:grow(1), b, 0.11))

--@ chunk 48
blend(lemon2:grow(1) - lemon1, {angle=0.6, tool={kind="badger", width=10}}); blend(lemon1:grow(1), {angle=0.6, tool={kind="badger", width=10}})

--@ chunk 49
tTop2 = pile{{"lead white",1.5},{"yellow ochre",0.8},{"raw umber",1.3},{"bone black",0.2},{"green earth",0.3}}
tShad = pile{{"raw umber",1.5},{"bone black",0.45},{"lead white",0.6},{"yellow ochre",0.4},{"green earth",0.3}}
local objs = (jug + lemons):grow(1)
local top = table_top - objs
work(top, {hand="body", tool="filbert 12", pile=tTop2, coverage=2.2, angle=0, angle_jitter=0.04, length={50,130}, edge="firm", fill=true, clip=table_top})
jugTabShadow = poly({{300,606},{425,603},{570,521},{345,521}}) - objs
lemShadow = (ellipse(655, 606, 72, 30) + ellipse(768, 566, 55, 20)) - objs
local sh = (jugTabShadow + lemShadow) * table_top
work(sh, {hand="body", tool="filbert 8", pile=tShad, coverage=2, angle=0, angle_jitter=0.1, length={20,60}, edge="soft", fill=true, clip=table_top - objs})

--@ chunk 50
inDark = pile{{"raw umber",1.4},{"bone black",0.4},{"lead white",0.5},{"green earth",0.4}}
inMid = pile{{"lead white",1.5},{"raw umber",0.9},{"yellow ochre",0.3},{"green earth",0.3}}
local cx, cy, rx, ry = 362, 191, 57, 11
local function E(a, sx, sy) return {cx + (rx*(sx or 1))*math.cos(a), cy + (ry*(sy or 1))*math.sin(a)} end
opening = ellipse(cx, cy, rx-3, ry-2)
local b = brush("filbert", 5)
-- interior: dark left, mid right
for i = 0, 5 do
  local yy = cy - 7 + i * 2.6
  local half = (rx-4) * math.sqrt(math.max(0, 1 - ((yy-cy)/(ry-2))^2))
  b:reload(inDark, 0.7)
  b:stroke({{cx - half, yy}, {cx + half*0.35, yy}}, {pressure={0.5,0.5}, ramps={0.1,0.2}, clip=opening})
  b:reload(inMid, 0.7)
  b:stroke({{cx + half*0.25, yy}, {cx + half, yy}}, {pressure={0.5,0.5}, ramps={0.2,0.1}, clip=opening})
end
-- front rim: light, lit left
local r = brush{kind="round", width=4, point=0.3}
local pts = {}
for a = math.pi*0.98, math.pi*0.2, -0.08 do pts[#pts+1] = E(a, 1.0, 1.15) end
r:reload(jp[1], 0.8)
r:stroke({pts[1],pts[2],pts[3],pts[4],pts[5],pts[6]}, {pressure={0.8,0.7}, ramps={0.1,0.2}})
r:reload(jp[3], 0.8)
local rest = {}
for i = 6, #pts do rest[#rest+1] = pts[i] end
r:stroke(rest, {pressure={0.7,0.6}, ramps={0.1,0.3}})
-- back rim: thin line
local back = {}
for a = math.pi*1.05, math.pi*1.95, 0.08 do back[#back+1] = E(a, 1.0, 1.1) end
local h = brush{kind="round", width=3, point=0.5}
h:reload(jp[2], 0.6)
h:stroke(back, {pressure={0.5,0.4}, ramps={0.2,0.3}})
-- spout lip
local sp = brush("filbert", 6)
sp:reload(jp[1], 0.8)
sp:stroke({{306,188},{290,182},{274,176},{262,174}}, {pressure={0.7,0.3}, ramps={0.1,0.4}})
sp:reload(jp[4], 0.6)
sp:stroke({{305,198},{290,192},{276,184}}, {pressure={0.5,0.2}, ramps={0.1,0.5}})

--@ chunk 51
local H = {{420,236},{445,231},{470,233},{492,244},{505,265},{510,295},{505,325},{495,352},{484,374},{474,393}}
local function off(pts, d)
  local out = {}
  for i = 1, #pts do
    local a = pts[math.max(1, i-1)]; local c = pts[math.min(#pts, i+1)]
    local dx, dy = c[1]-a[1], c[2]-a[2]
    local l = math.sqrt(dx*dx+dy*dy)
    out[i] = {pts[i][1] - dy/l*d, pts[i][2] + dx/l*d}
  end
  return out
end
-- normal (-dy, dx): for path going right then down, positive d points ... test outward = up/right
local b = brush("filbert", 9)
b:reload(jp[5], 0.8)
b:stroke(H, {pressure={0.7,0.6}, ramps={0.05,0.1}, orient="across"})
b:reload(jp[5], 0.8)
b:stroke(off(H, 4), {pressure={0.6,0.5}, ramps={0.05,0.1}, orient="across"})
b:reload(jp[5], 0.8)
b:stroke(off(H, -4), {pressure={0.6,0.5}, ramps={0.05,0.1}, orient="across"})
local d = brush("filbert", 5)
d:reload(jDark, 0.7)
d:stroke(off({H[4],H[5],H[6],H[7],H[8],H[9],H[10]}, 7), {pressure={0.5,0.4}, ramps={0.1,0.2}, orient="across"})
d:reload(jDark, 0.7)
d:stroke(off({H[5],H[6],H[7],H[8],H[9],H[10]}, -7), {pressure={0.4,0.3}, ramps={0.2,0.2}, orient="across"})
local l = brush{kind="round", width=4, point=0.4}
l:reload(jp[2], 0.7)
l:stroke(off({H[1],H[2],H[3],H[4],H[5]}, -6), {pressure={0.7,0.2}, ramps={0.1,0.5}})
l:reload(jp[3], 0.6)
l:stroke(off({H[4],H[5],H[6],H[7]}, -2), {pressure={0.5,0.1}, ramps={0.1,0.6}})

--@ chunk 52
print(wait(48*60)); print(drying(610,592), drying(720,550), drying(470,300), drying(600,650))

--@ chunk 53
lp = {
  pile{{"yellow ochre",2},{"raw umber",1.2},{"green earth",0.7},{"chrome yellow",0.4}},
  pile{{"yellow ochre",2},{"raw umber",0.7},{"green earth",0.4},{"chrome yellow",0.8}},
  pile{{"yellow ochre",1.5},{"chrome yellow",1.5},{"raw umber",0.3},{"green earth",0.2}},
  pile{{"chrome yellow",3},{"yellow ochre",1}},
  pile{{"chrome yellow",3},{"yellow ochre",0.3},{"lead white",0.3}},
  pile{{"chrome yellow",3},{"lead white",1}},
  pile{{"chrome yellow",2},{"lead white",2.5}},
}
local th = {0.05, 0.2, 0.35, 0.5, 0.65, 0.82}
function lemonPaint2(cx, cy, rx, ry, rot, clipm, b, step)
  local Lx, Ly, Lz = -1, -0.75, 0.9
  local ln = math.sqrt(Lx*Lx+Ly*Ly+Lz*Lz); Lx, Ly, Lz = Lx/ln, Ly/ln, Lz/ln
  local c, s = math.cos(rot), math.sin(rot)
  local function P(u, v)
    local x = u * rx
    local y = v * ry + 0.12 * ry * (1 - u*u)
    return {cx + c*x - s*y, cy + s*x + c*y}
  end
  local function idx(u, v)
    local w = math.max(0, 1 - u*u - v*v)
    local d = u*Lx + v*Ly + math.sqrt(w)*Lz + rand(-0.05, 0.05)
    local k = 7
    for i = 1, 6 do if d < th[i] then k = i; break end end
    if v > 0.8 and k < 2 then k = 2 end
    return k
  end
  local segs = {}
  local v = -0.98
  while v < 0.98 do
    local vv = v + rand(-0.02, 0.02)
    local umax = math.sqrt(math.max(0, 1 - vv*vv)) + 0.06
    local cur, pts = nil, {}
    local u = -umax + rand(0, 0.05)
    while u <= umax do
      local k = idx(u, vv)
      if k ~= cur or #pts > 4 then
        if cur and #pts >= 2 then segs[#segs+1] = {k=cur, pts=pts} end
        pts = (#pts > 0) and {pts[#pts]} or {}
        cur = k
      end
      pts[#pts+1] = P(u, vv)
      u = u + 0.08
    end
    if cur and #pts >= 2 then segs[#segs+1] = {k=cur, pts=pts} end
    v = v + step
  end
  for k = 1, 7 do
    for _, sg in ipairs(segs) do
      if sg.k == k then
        b:reload(lp[k], 0.65)
        b:stroke(sg.pts, {pressure={0.55, 0.45}, ramps={0.2, 0.3}, orient="across", clip=clipm})
      end
    end
  end
  return #segs
end
local b = brush("filbert", 7)
print(lemonPaint2(722, 553, 68, 38, 0.05, lemon2:grow(1) - lemon1, b, 0.085))
print(lemonPaint2(610, 592, 78, 47, -0.08, lemon1:grow(1), b, 0.075))

--@ chunk 54
blade = poly({{518,650},{560,654},{620,661},{700,672},{766,681},{766,695},{700,688},{630,678},{575,668},{540,659}}, true)
bolster = poly({{764,679},{779,681},{779,698},{764,696}})
kHandle = ribbon({{779,690},{830,697},{880,704},{930,711}}, {17,18,18,16})
knife = blade + bolster + kHandle
bBase = pile{{"lead white",1.5},{"bone black",0.45},{"raw umber",0.3},{"pale smalt",0.4}}
bHi = pile{{"lead white",3},{"pale smalt",0.2},{"yellow ochre",0.05}}
bDark = pile{{"bone black",1},{"lead white",0.6},{"raw umber",0.4}}
hWood = pile{{"raw umber",1},{"bone black",0.7},{"red earth",0.4}}
hLight = pile{{"raw umber",1},{"lead white",0.7},{"yellow ochre",0.3},{"red earth",0.2}}
kShadow = pile{{"bone black",1},{"raw umber",1.2},{"lead white",0.15}}
-- cast shadow of the blade on the table, and of the handle on the front face
local b = brush("filbert", 6)
b:reload(kShadow, 0.6)
b:stroke({{540,664},{620,680},{700,692}}, {pressure={0.3,0.6}, ramps={0.3,0.1}, clip=table_top})
b:reload(kShadow, 0.6)
b:stroke({{690,690},{760,698}}, {pressure={0.6,0.6}, clip=table_top})
local fsh = poly({{790,700},{940,720},{950,748},{800,730}}, true):blur(6) * front
local fb = brush("filbert", 10)
for i = 0, 3 do
  fb:reload(kShadow, 0.6)
  fb:stroke({{795, 708 + i*6},{870, 718 + i*6},{945, 728 + i*6}}, {pressure={0.6,0.5}, clip=fsh})
end
-- blade
local bb = brush("filbert", 5)
for i = 0, 3 do
  bb:reload(bBase, 0.7)
  bb:stroke({{522 + i*6, 652 + i*1.5}, {620, 664 + i*2.5}, {700, 676 + i*3}, {766, 684 + i*3}}, {pressure={0.4,0.6}, ramps={0.3,0.1}, clip=blade})
end
bb:reload(bDark, 0.6)
bb:stroke({{560,665},{640,677},{766,693}}, {pressure={0.3,0.5}, ramps={0.3,0.1}, clip=blade})
local r = brush{kind="round", width=3, point=0.5}
r:reload(bHi, 0.8)
r:stroke({{530,652},{620,662},{700,673},{764,682}}, {pressure={0.2,0.6}, ramps={0.4,0.1}, clip=blade:grow(1)})
-- bolster
local s = brush("flat", 5)
s:reload(bBase, 0.7)
s:stroke({{771,680},{771,697}}, {pressure={0.6,0.6}, clip=bolster})
s:reload(bHi, 0.5)
s:stroke({{768,681},{768,690}}, {pressure={0.4,0.2}, clip=bolster})
-- handle
local h = brush("filbert", 8)
for i = -1, 1 do
  h:reload(hWood, 0.8)
  h:stroke({{780, 690 + i*5}, {855, 700.5 + i*5}, {930, 711 + i*5}}, {pressure={0.6,0.6}, ramps={0.1,0.1}, clip=kHandle})
end
local hl = brush{kind="round", width=4, point=0.3}
hl:reload(hLight, 0.7)
hl:stroke({{784,685},{850,694},{925,705}}, {pressure={0.5,0.3}, ramps={0.2,0.4}, clip=kHandle})

--@ chunk 55
local fsh = poly({{785,700},{945,720},{955,752},{795,734}}, true):grow(6) * front
blend(fsh, {angle=0.14, tool={kind="badger", width=14}})
kHandle2 = ribbon({{776,690},{830,697},{880,704},{928,711}}, {19,20,20,19})
local h = brush("filbert", 10)
for i = -1, 1 do
  h:reload(hWood, 0.8)
  h:stroke({{778, 690 + i*5.5}, {855, 700.5 + i*5.5}, {929, 711 + i*5.5}}, {pressure={0.7,0.7}, ramps={0.05,0.05}, orient="across", clip=kHandle2})
end
h:reload(hWood, 0.6)
h:touch(930, 711, {pressure=0.7, clip=kHandle2})
local hl = brush{kind="round", width=5, point=0.3}
hl:reload(hLight, 0.8)
hl:stroke({{782,684},{830,690},{880,697},{924,703}}, {pressure={0.55,0.35}, ramps={0.2,0.3}, clip=kHandle2})
local rv = brush{kind="round", width=3, point=0.2}
for _, p in ipairs({{812,695},{865,702},{905,707}}) do
  rv:reload(bHi, 0.6)
  rv:touch(p[1], p[2], {pressure=0.6})
end
-- lit arris of the ledge
local a = brush{kind="round", width=3, point=0.3}
local edgeLight = pile{{"lead white",2},{"yellow ochre",0.8},{"raw umber",0.5}}
local x = 0
while x < 1000 do
  local x1 = math.min(1000, x + rand(80, 160))
  if not (x1 > 770 and x < 935) then
    a:reload(edgeLight, 0.6)
    a:stroke({{x, 690 + rand(-0.5,0.5)}, {x1, 690 + rand(-0.5,0.5)}}, {pressure={0.45,0.35}, ramps={0.2,0.2}})
  end
  if x1 >= 1000 then break end
  x = x1 - 5
end

--@ chunk 56
local b = brush{kind="flat", width=10, stiffness=0.6}
local keep = kHandle2:grow(2)
local n = 0
for pass = 1, 2 do
  for off = 16, 70, 6 do
    local pts = {{755, 697 + off}, {870, 713 + off}, {985, 729 + off}}
    b:stroke(pts, {pressure={0.7,0.7}, ramps={0.05,0.05}, orient="across", clip=front - keep})
    b:wipe(1)
    n = n + 1
  end
end
print(n)

--@ chunk 57
local b = brush{kind="flat", width=12, stiffness=0.6}
local keep = kHandle2:grow(1)
for pass = 1, 2 do
  for off = 19, 72, 5 do
    b:stroke({{750, 697 + off + pass*2}, {870, 713 + off + pass*2}, {990, 729 + off + pass*2}}, {pressure={0.8,0.8}, ramps={0.05,0.05}, orient="across", clip=front - keep})
    b:wipe(1)
  end
end
for y = 700, 745, 6 do
  b:stroke({{938, y}, {995, y + 8}}, {pressure={0.8,0.8}, ramps={0.05,0.05}, orient="across", clip=front - keep})
  b:wipe(1)
end

--@ chunk 58
print(wait(60*60)); print(drying(850,700), drying(850,725), drying(360,190), drying(470,300))

--@ chunk 59
tFrontL = pile{{"raw umber",2},{"yellow ochre",0.8},{"bone black",0.4},{"lead white",0.9}}
local F = front * rect(0, 692, 1000, 110) - kHandle2:grow(1)
work(F, {hand="body", tool="filbert 12", pile=tFront, coverage=2, angle=0, angle_jitter=0.03, length={60,150}, edge="firm", fill=true, clip=F})
local left = F * mask(function(x,y) return 1 - smoothstep(150, 450, x) end)
work(left, {hand="body", tool="filbert 12", pile=tFrontL, coverage=1.2, angle=0, length={60,140}, threshold=0.4, clip=F})
local sb = brush("filbert", 10)
for i = 0, 2 do
  sb:reload(kShadow, 0.7)
  sb:stroke({{782, 705 + i*5}, {860, 716 + i*5}, {935, 726 + i*5}}, {pressure={0.6,0.5}, ramps={0.1,0.3}, orient="across", clip=F})
end
blend(F, {angle=0.02})

--@ chunk 60
-- wall colour cut back along the lit contour
local outside = (wall - (jugBody + spout)) * rect(0, 150, 360, 371)
local b = brush("filbert", 7)
local seg = {}
for y = 205, 515, 6 do seg[#seg+1] = {edgeX(jugL, y) - 6, y} end
local i = 1
while i < #seg do
  local j = math.min(#seg, i + 6)
  local pts = {}
  for k = i, j do pts[#pts+1] = seg[k] end
  b:reload(bgOlive, 0.6)
  b:stroke(pts, {pressure={0.55,0.5}, ramps={0.15,0.15}, orient="across", clip=outside})
  i = j
end
-- jug shadow side: broken strokes, low load, no blending
local s = brush("filbert", 9)
strips(0.64, 0.9, jp[5], s, 212, 604, 5, {load=0.45, pressure={0.5,0.4}})
strips(0.7, 0.86, jDark, s, 240, 600, 3, {load=0.35, pressure={0.45,0.35}})
strips(0.53, 0.65, jp[4], s, 205, 604, 3, {load=0.4, pressure={0.45,0.35}})

--@ chunk 61
local jb = jugBody:grow(1)
local Z = jb * mask(function(x,y) if y < 200 then return 0 end; return smoothstep(0.4, 0.52, jT(x,y)) end)
blend(Z, {angle=1.5708, tool={kind="badger", width=20}})
blend(Z, {angle=0.1, tool={kind="badger", width=20}})

--@ chunk 62
print(wait(60*60)); print(drying(420,400), drying(460,500), drying(850,720))

--@ chunk 63
local jb = jugBody:grow(1)
local Z = jb * mask(function(x,y) if y < 200 then return 0 end; return smoothstep(0.4, 0.52, jT(x,y)) end)
local core = pile{{"lead white",1},{"raw umber",0.9},{"yellow ochre",0.3},{"bone black",0.15}}
local half = pile{{"lead white",2},{"raw umber",0.6},{"yellow ochre",0.3},{"bone black",0.05}}
local refl = pile{{"lead white",1.6},{"raw umber",0.5},{"yellow ochre",0.6},{"red earth",0.1}}
local b = brush("filbert", 12)
local function band(t0, t1, p, n, load)
  for k = 0, n-1 do
    local t = t0 + (t1 - t0) * (k + 0.5) / n
    local pts = {}
    for y = 208, 600, 14 do
      local xl, xr = edgeX(jugL, y), edgeX(jugR, y)
      pts[#pts+1] = {xl + (xr - xl) * t, y}
    end
    b:reload(p, load)
    b:stroke(pts, {pressure={0.6,0.6}, ramps={0.05,0.1}, orient="across", clip=Z})
  end
end
band(0.5, 0.62, half, 3, 0.6)
band(0.62, 0.88, core, 5, 0.7)
band(0.88, 0.99, refl, 2, 0.55)
blend(Z, {angle=1.5708, tool={kind="badger", width=16}})

--@ chunk 64
print(help and help("pile") or "")

--@ chunk 65
print(wait(72*60)); print(drying(420,400), drying(470,500), drying(380,300))

--@ chunk 66
local jb = jugBody:grow(1)
gCore = pile{{"raw umber",1},{"yellow ochre",0.3},{"bone black",0.1}, medium=0.88}
local C = jb * mask(function(x,y)
  if y < 196 then return 0 end
  local t = jT(x,y)
  return smoothstep(0.52, 0.7, t) * (1 - smoothstep(0.86, 0.97, t))
end)
work(C, {hand="glaze", pile=gCore, angle=1.5708, angle_jitter=0.08, coverage=1.5, threshold=0.25, clip=jb})

--@ chunk 67
local jb = jugBody:grow(1)
local Z = jb * mask(function(x,y) if y < 196 then return 0 end; return smoothstep(0.42, 0.55, jT(x,y)) end)
blend(Z, {angle=0.05, tool={kind="badger", width=24}})
blend(Z, {angle=1.5708, tool={kind="badger", width=24}})
blend(Z, {angle=0.05, tool={kind="badger", width=30}})

--@ chunk 68
print(wait(72*60)); print(drying(420,400), drying(470,500))

--@ chunk 69
local jb = jugBody:grow(1)
gCore2 = pile{{"raw umber",1},{"bone black",0.25},{"green earth",0.2}, medium=0.85}
local C = jb * mask(function(x,y)
  if y < 196 then return 0 end
  local t = jT(x,y)
  return smoothstep(0.58, 0.72, t) * (1 - smoothstep(0.84, 0.94, t))
end)
work(C, {hand="glaze", pile=gCore2, angle=1.5708, angle_jitter=0.05, coverage=2, threshold=0.4, clip=jb})
local Z = jb * mask(function(x,y) if y < 196 then return 0 end; return smoothstep(0.42, 0.55, jT(x,y)) end)
blend(Z, {angle=0.05, tool={kind="badger", width=24}})
blend(Z, {angle=1.5708, tool={kind="badger", width=24}})
blend(Z, {angle=0.05, tool={kind="badger", width=30}})

--@ chunk 70
local jb = jugBody:grow(1)
blend(jb * rect(0, 196, 1000, 420), {angle=0.0, tool={kind="badger", width=20}})

--@ chunk 71
print(wait(96*60)); print(drying(420,400), drying(470,500))

--@ chunk 72
print(bgMid); print(bgOlive); print(bgLight); print(bgWarm); print(bgDark)

--@ chunk 73
local J = (jugBody + spout):grow(1)
local band = (J:grow(18) - J) * rect(0, 150, 420, 371)
work(band, {hand="body", tool="filbert 8", pile=bgMid, coverage=2.5, angle=1.5708, angle_jitter=0.3, length={20,50}, fill=true, clip=wall - J})
blend(rect(0,0,1000,521) - J - handle:grow(1), {angle=0.3, tool={kind="badger", width=30}})

--@ chunk 74
local J = (jugBody + spout):grow(1)
local W = rect(0,0,1000,521) - J - handle:grow(1)
blend(W, {angle=1.5708, tool={kind="badger", width=40}})
blend(W, {angle=0, tool={kind="badger", width=40}})
blend(W, {angle=0.8, tool={kind="badger", width=40}})

--@ chunk 75
print(wait(72*60)); print(drying(250,300), drying(420,200))

--@ chunk 76
print(edgeX(jugL,598), edgeX(jugR,598), edgeX(jugL,590), edgeX(jugR,590), edgeX(jugL,560), edgeX(jugR,560))

--@ chunk 77
local cx, cy, rx, ry = 362, 191, 57, 11
local function E(a, sx, sy) return {cx + (rx*(sx or 1))*math.cos(a), cy + (ry*(sy or 1))*math.sin(a)} end
local b = brush("filbert", 5)
local inLit = pile{{"lead white",1.3},{"raw umber",0.9},{"yellow ochre",0.4},{"green earth",0.2}}
for i = 0, 5 do
  local yy = cy - 7 + i * 2.6
  local half = (rx-4) * math.sqrt(math.max(0, 1 - ((yy-cy)/(ry-2))^2))
  b:reload(inLit, 0.7)
  b:stroke({{cx + half*0.3, yy}, {cx + half, yy}}, {pressure={0.5,0.5}, ramps={0.3,0.05}, clip=opening})
end
-- rim: back line and front right part in the half-tone
local r = brush{kind="round", width=3, point=0.4}
local back = {}
for a = math.pi*1.02, math.pi*1.98, 0.07 do back[#back+1] = E(a, 1.02, 1.12) end
r:reload(jp[2], 0.7)
r:stroke(back, {pressure={0.5,0.45}, ramps={0.2,0.2}})
local front = {}
for a = math.pi*0.55, -0.05, -0.07 do front[#front+1] = E(a, 1.0, 1.15) end
r:reload(jp[4], 0.7)
r:stroke(front, {pressure={0.6,0.5}, ramps={0.2,0.2}})
-- handle shadow: glaze on the descending part, stronger on the outer side
local H = {{420,236},{445,231},{470,233},{492,244},{505,265},{510,295},{505,325},{495,352},{484,374},{474,393}}
local function off(pts, d)
  local out = {}
  for i = 1, #pts do
    local a = pts[math.max(1, i-1)]; local c = pts[math.min(#pts, i+1)]
    local dx, dy = c[1]-a[1], c[2]-a[2]
    local l = math.sqrt(dx*dx+dy*dy)
    out[i] = {pts[i][1] - dy/l*d, pts[i][2] + dx/l*d}
  end
  return out
end
local hm = handle:grow(1)
local g = brush("filbert", 8)
g:reload(gCore2, 0.6)
g:stroke(off({H[4],H[5],H[6],H[7],H[8],H[9],H[10]}, -3), {pressure={0.3,0.6}, ramps={0.3,0.05}, clip=hm})
g:reload(gCore2, 0.5)
g:stroke(off({H[6],H[7],H[8],H[9],H[10]}, 3), {pressure={0.2,0.4}, ramps={0.4,0.05}, clip=hm})
g:reload(gCore2, 0.5)
g:stroke({H[1],H[2],H[3]}, {pressure={0.2,0.2}, clip=hm * rect(400,240,40,20)})
-- contact shadow under the jug
local cs = brush("filbert", 5)
cs:reload(kShadow, 0.7)
cs:stroke({{296,602},{330,605},{370,606},{405,604},{425,600}}, {pressure={0.45,0.6}, ramps={0.2,0.1}, clip=table_top - jugBody})

--@ chunk 78
blend(handle:grow(1) * rect(470,250,60,150), {angle=1.75, tool={kind="badger", width=10}}); blend(handle:grow(1) * rect(470,250,60,150), {angle=0.2, tool={kind="badger", width=8}})

--@ chunk 79
print(wait(72*60)); print(drying(505,300), drying(362,191), drying(360,605))

--@ chunk 80
local H = {{420,236},{445,231},{470,233},{492,244},{505,265},{510,295},{505,325},{495,352},{484,374},{474,393}}
local function off(pts, d)
  local out = {}
  for i = 1, #pts do
    local a = pts[math.max(1, i-1)]; local c = pts[math.min(#pts, i+1)]
    local dx, dy = c[1]-a[1], c[2]-a[2]
    local l = math.sqrt(dx*dx+dy*dy)
    out[i] = {pts[i][1] - dy/l*d, pts[i][2] + dx/l*d}
  end
  return out
end
local function sub(i, j) local o = {} for k = i, j do o[#o+1] = H[k] end return o end
hShade = pile{{"lead white",1},{"raw umber",0.8},{"yellow ochre",0.35},{"bone black",0.1}}
hMid = pile{{"lead white",1.6},{"raw umber",0.6},{"yellow ochre",0.3}}
local hm = handle:grow(1)
local b = brush("filbert", 6)
for _, d in ipairs({-4, 0, 3}) do
  b:reload(hShade, 0.7)
  b:stroke(off(sub(4,10), d), {pressure={0.3,0.6}, ramps={0.35,0.05}, orient="across", clip=hm})
end
b:reload(hMid, 0.7)
b:stroke(off(sub(1,5), 4), {pressure={0.5,0.3}, ramps={0.1,0.3}, orient="across", clip=hm})
local r = brush{kind="round", width=3, point=0.4}
r:reload(hMid, 0.6)
r:stroke(off(sub(6,10), 5), {pressure={0.3,0.4}, ramps={0.3,0.2}, clip=hm})
-- orange speck in the loop
local t = brush{kind="round", width=6, point=0.3}
t:reload(bgMid, 0.7)
t:stroke({{444,241},{460,241}}, {pressure={0.6,0.6}, clip=wall - jugBody - hm})
-- lower-left belly: light half-tone strokes curving round the form
local s = brush("filbert", 7)
for y = 506, 597, 7 do
  local xl, xr = edgeX(jugL, y), edgeX(jugR, y)
  local x0, x1 = xl + 3, xl + (xr - xl) * 0.42
  s:reload(jp[3], 0.5)
  s:stroke({{x0, y - 1}, {(x0+x1)/2, y + 1.5}, {x1, y + 2}}, {pressure={0.5,0.25}, ramps={0.1,0.5}, orient="across", clip=jugBody})
end
-- highlight
local hl = brush("filbert", 6)
hl:reload(jp[1], 0.8)
local hx = function(y, t) local xl, xr = edgeX(jugL, y), edgeX(jugR, y); return xl + (xr - xl) * t end
hl:stroke({{hx(335,0.2),335},{hx(380,0.17),380},{hx(430,0.17),430}}, {pressure={0.2,0.2}, ramps={0.4,0.5}, clip=jugBody})
hl:reload(jp[1], 0.8)
hl:stroke({{hx(215,0.22),215},{hx(245,0.2),245}}, {pressure={0.1,0.1}, ramps={0.4,0.5}, clip=jugBody})

--@ chunk 81
blend(jugBody * rect(0,498,1000,112), {angle=0.03, tool={kind="badger", width=14}})

--@ chunk 82
local b = brush{kind="flat", width=10, stiffness=0.6}
local R = jugBody * mask(function(x,y) return smoothstep(0.4, 0.5, jT(x,y)) end)
for pass = 1, 2 do
  for y = 500, 604, 5 do
    local xl, xr = edgeX(jugL, y), edgeX(jugR, y)
    b:stroke({{xr + 2, y + pass}, {xl + (xr-xl)*0.42, y + pass}}, {pressure={0.8,0.6}, ramps={0.05,0.3}, orient="across", clip=R})
    b:wipe(1)
  end
end

--@ chunk 83
print(wait(72*60)); print(drying(505,300), drying(400,560), drying(300,560))

--@ chunk 84
local jb = jugBody:grow(1)
local C = jb * mask(function(x,y)
  if y < 440 then return 0 end
  local t = jT(x,y)
  return smoothstep(0.5, 0.64, t) * (1 - smoothstep(0.9, 0.98, t)) * smoothstep(440, 490, y)
end)
work(C, {hand="glaze", pile=gCore2, angle=0.05, angle_jitter=0.05, coverage=1.5, threshold=0.3, clip=jb})
local Z = jb * mask(function(x,y) if y < 196 then return 0 end; return smoothstep(0.44, 0.56, jT(x,y)) end)
blend(Z, {angle=1.5708, tool={kind="badger", width=24}})
blend(Z, {angle=0.05, tool={kind="badger", width=24}})
-- handle: darker strap on the shadowed descending part
local H = {{420,236},{445,231},{470,233},{492,244},{505,265},{510,295},{505,325},{495,352},{484,374},{474,393}}
local function off(pts, d)
  local out = {}
  for i = 1, #pts do
    local a = pts[math.max(1, i-1)]; local c = pts[math.min(#pts, i+1)]
    local dx, dy = c[1]-a[1], c[2]-a[2]
    local l = math.sqrt(dx*dx+dy*dy)
    out[i] = {pts[i][1] - dy/l*d, pts[i][2] + dx/l*d}
  end
  return out
end
local function sub(i, j) local o = {} for k = i, j do o[#o+1] = H[k] end return o end
local hDark = pile{{"raw umber",1},{"lead white",0.6},{"yellow ochre",0.3},{"bone black",0.15}}
local hm = handle:grow(1)
local b = brush("filbert", 5)
b:reload(hDark, 0.7)
b:stroke(off(sub(4,10), -4), {pressure={0.3,0.6}, ramps={0.4,0.05}, orient="across", clip=hm})
b:reload(hDark, 0.6)
b:stroke(off(sub(5,10), 0), {pressure={0.2,0.5}, ramps={0.5,0.05}, orient="across", clip=hm})

--@ chunk 85
local b = brush{kind="flat", width=10, stiffness=0.6}
local L = jugBody:grow(2) * mask(function(x,y) return 1 - smoothstep(0.42, 0.5, jT(x,y)) end)
for pass = 1, 3 do
  for y = 436, 500, 5 do
    local xl, xr = edgeX(jugL, y), edgeX(jugR, y)
    b:stroke({{xl + (xr-xl)*0.5, y + pass}, {xl - 3, y + pass}}, {pressure={0.8,0.8}, ramps={0.2,0.05}, orient="across", clip=L})
    b:wipe(1)
  end
end

--@ chunk 86
local L1, L2 = lemon1:grow(1), lemon2:grow(1) - lemon1
local function P(cx, cy, rx, ry, rot, u, v)
  local c, s = math.cos(rot), math.sin(rot)
  local x, y = u*rx, v*ry + 0.12*ry*(1-u*u)
  return {cx + c*x - s*y, cy + s*x + c*y}
end
-- olive shadow of the front lemon on the back one
local g = pile{{"raw umber",1},{"green earth",0.6},{"yellow ochre",0.8}, medium=0.8}
local b = brush("filbert", 8)
b:reload(g, 0.5)
b:stroke({{676,578},{690,586},{705,590}}, {pressure={0.5,0.3}, ramps={0.1,0.4}, clip=L2})
-- contact shadows
local cs = brush("filbert", 5)
cs:reload(kShadow, 0.7)
cs:stroke({P(610,592,78,47,-0.08,-0.55,1.0), P(610,592,78,47,-0.08,0,1.08), P(610,592,78,47,-0.08,0.6,0.98)}, {pressure={0.3,0.6}, ramps={0.3,0.2}, clip=table_top - (lemons:grow(0))})
cs:reload(kShadow, 0.7)
cs:stroke({P(722,553,68,38,0.05,-0.2,1.08), P(722,553,68,38,0.05,0.3,1.08), P(722,553,68,38,0.05,0.75,0.9)}, {pressure={0.3,0.6}, ramps={0.3,0.2}, clip=table_top - (lemons:grow(0))})
-- pores
stipple(L1, {pile=lp[3], width=2, coverage=0.12})
stipple(L2, {pile=lp[3], width=2, coverage=0.12})
-- highlights
local h = brush{kind="round", width=5, point=0.3}
local hp = pile{{"lead white",3},{"chrome yellow",0.4}}
h:reload(hp, 0.8)
h:stroke({P(610,592,78,47,-0.08,-0.45,-0.55), P(610,592,78,47,-0.08,-0.2,-0.68)}, {pressure={0.5,0.2}, ramps={0.3,0.5}, clip=L1})
h:reload(hp, 0.7)
h:stroke({P(722,553,68,38,0.05,-0.4,-0.55), P(722,553,68,38,0.05,-0.15,-0.68)}, {pressure={0.4,0.15}, ramps={0.3,0.5}, clip=L2})
local d = brush{kind="round", width=2.5, point=0.3}
d:reload(pile{{"lead white",3}}, 0.8)
d:touch(P(610,592,78,47,-0.08,-0.4,-0.58)[1], P(610,592,78,47,-0.08,-0.4,-0.58)[2], {pressure=0.6})
-- nibs: tip accents
local n = brush{kind="round", width=4, point=0.4}
n:reload(lp[2], 0.7)
n:stroke({P(610,592,78,47,-0.08,-1.0,0.02), P(610,592,78,47,-0.08,-1.07,0.05)}, {pressure={0.5,0.3}})
n:reload(lp[1], 0.7)
n:stroke({P(722,553,68,38,0.05,1.0,0.0), P(722,553,68,38,0.05,1.08,0.02)}, {pressure={0.5,0.3}})

--@ chunk 87
local b = brush{kind="flat", width=8, stiffness=0.6}
local T = table_top - lemons:grow(1)
for pass = 1, 3 do
  for _, seg in ipairs({{{565,642},{600,648},{660,646}}, {{700,594},{740,597},{780,590}}}) do
    for dy = -3, 3, 3 do
      local pts = {}
      for i, p in ipairs(seg) do pts[i] = {p[1], p[2] + dy} end
      b:stroke(pts, {pressure={0.8,0.8}, clip=T})
      b:wipe(1)
    end
  end
end

--@ chunk 88
local b = brush{kind="flat", width=14, stiffness=0.4}
local T = table_top - lemons:grow(1)
for pass = 1, 4 do
  for _, seg in ipairs({{{565,642},{600,648},{660,646}}, {{700,594},{740,597},{780,590}}}) do
    for dy = -3, 3, 3 do
      local pts = {}
      for i, p in ipairs(seg) do pts[i] = {p[1], p[2] + dy} end
      b:stroke(pts, {pressure={0.8,0.8}, clip=T})
      b:wipe(1)
    end
  end
end

--@ chunk 89
print(wait(96*60)); print(drying(620,600), drying(730,560), drying(500,330), drying(640,640))

--@ chunk 90
local T = table_top - lemons
local b = brush("filbert", 10)
for _, s in ipairs({{{555,640},{610,645},{670,642}}, {{560,650},{615,654},{672,650}}, {{690,592},{740,596},{792,590}}, {{700,600},{745,603},{795,598}}}) do
  b:reload(tTop2, 0.7)
  b:stroke(s, {pressure={0.6,0.6}, ramps={0.2,0.2}, orient="across", clip=T})
end
local function arc(cx, cy, rx, ry, a0, a1)
  local pts = {}
  for a = a0, a1, -0.12 do pts[#pts+1] = {cx + rx*math.cos(a), cy + ry*math.sin(a) + 1} end
  return pts
end
local cs = brush("filbert", 5)
cs:reload(kShadow, 0.7)
cs:stroke(arc(610, 592, 72, 46, 2.2, 0.5), {pressure={0.25,0.55}, ramps={0.4,0.15}, clip=T})
cs:reload(kShadow, 0.7)
cs:stroke({{700,588},{720,591},{745,589},{770,583},{778,576}}, {pressure={0.25,0.5}, ramps={0.4,0.2}, clip=T})

--@ chunk 91
local H = {{420,236},{445,231},{470,233},{492,244},{505,265},{510,295},{505,325},{495,352},{484,374},{474,393}}
local function off(pts, d)
  local out = {}
  for i = 1, #pts do
    local a = pts[math.max(1, i-1)]; local c = pts[math.min(#pts, i+1)]
    local dx, dy = c[1]-a[1], c[2]-a[2]
    local l = math.sqrt(dx*dx+dy*dy)
    out[i] = {pts[i][1] - dy/l*d, pts[i][2] + dx/l*d}
  end
  return out
end
local function sub(i, j) local o = {} for k = i, j do o[#o+1] = H[k] end return o end
local hDark2 = pile{{"raw umber",1},{"lead white",0.45},{"yellow ochre",0.35},{"bone black",0.2}}
local hm = handle:grow(1)
local b = brush("filbert", 7)
b:reload(hDark2, 0.75)
b:stroke(off(sub(5,10), -2), {pressure={0.3,0.65}, ramps={0.45,0.05}, orient="across", clip=hm})
b:reload(hDark2, 0.6)
b:stroke(off(sub(3,6), 4), {pressure={0.1,0.35}, ramps={0.5,0.3}, orient="across", clip=hm})

--@ chunk 92
local t = brush{kind="round", width=5, point=0.3}; t:reload(hMid, 0.7); t:stroke({{446,240},{460,241}}, {pressure={0.6,0.6}})

--@ chunk 93
print(wait(120*60)); print(drying(620,640), drying(490,330), drying(740,590), drying(450,240))

--@ chunk 94
varnish{coats=0.3, vary=0.08}
