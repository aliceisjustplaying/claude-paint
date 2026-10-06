-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=700, aspect=1.4, linen={16,13}, seed=1811,
 ground={{pile={{"red earth",2},{"yellow ochre",2},{"lead white",1}}, um=120, apply="knife", texture=0.35},
         {pile={{"lead white",6},{"yellow ochre",0.4},{"raw umber",0.1}}, um=60, apply="brush"}}}
print(W,H); print(table.concat(tubes(), ", "))

--@ chunk 2
h = pencil("H")
-- horizon and knoll
h:sketch({{0,455},{200,452},{400,456},{600,453},{800,457},{1000,454}}, {pressure=0.25})
h:sketch({{0,522},{150,515},{300,506},{450,492},{560,482},{640,478},{760,483},{880,497},{1000,508}}, {pressure=0.3})
-- dolmen: capstone and uprights
h:line({{552,458},{565,444},{600,438},{650,437},{700,441},{726,452},{722,463},{680,466},{610,466},{560,464},{552,458}}, {pressure=0.4})
for _,u in ipairs({{572,464,568,485},{612,466,615,482},{665,466,662,480},{705,463,708,484}}) do
  h:line({{u[1]-7,u[2]},{u[3]-8,u[4]}}, {pressure=0.35}); h:line({{u[1]+7,u[2]},{u[3]+8,u[4]}}, {pressure=0.35})
end
-- big oak trunk and main limbs
h:line({{385,508},{392,440},{396,380},{392,320},{380,280},{350,220},{320,160},{300,100}}, {pressure=0.35})
h:line({{425,508},{418,440},{414,380},{418,320},{430,280},{470,230},{520,170},{560,120}}, {pressure=0.35})
h:sketch({{400,300},{410,230},{405,160},{415,90},{410,50}}, {pressure=0.25})
h:sketch({{360,240},{300,215},{250,200},{220,170}}, {pressure=0.25})
h:sketch({{450,260},{520,250},{590,230},{620,200}}, {pressure=0.25})
-- right oak
h:line({{838,500},{842,430},{840,360},{830,300},{815,240}}, {pressure=0.35})
h:line({{862,500},{858,430},{862,370},{880,310},{910,250}}, {pressure=0.35})
h:sketch({{835,330},{790,300},{760,260}}, {pressure=0.25})
h:sketch({{870,340},{930,320},{970,280}}, {pressure=0.25})
-- figure
h:sketch({{505,470},{503,480},{500,500},{510,500},{508,480},{505,470}}, {pressure=0.3})
-- far church ruin
h:sketch({{190,455},{190,370},{205,360},{220,370},{220,455}}, {pressure=0.15})
-- moon
h:sketch({{752,100},{745,112},{750,126},{762,130}}, {pressure=0.2})

--@ chunk 3
sk1 = pile{{"lead white",5},{"smalt",2.2},{"raw umber",0.3},{"red earth",0.12}, medium=0.15}
sk2 = pile{{"lead white",7},{"smalt",1.2},{"red earth",0.08}, medium=0.15}
sk3 = pile{{"lead white",8},{"yellow ochre",0.5},{"vermilion",0.12}, medium=0.15}
sk4 = pile{{"lead white",8},{"chrome yellow",0.35},{"yellow ochre",0.3}, medium=0.15}
local hz = function(x) return 460 + 3*math.sin(x/70) end
local function band(a, b) return below(function(x) return a end) * above(function(x) return b end) end
skym = above(hz)
work(band(-10,170), {hand="broad", pile=sk1, angle=0, coverage=2, fill=true, angle_jitter=0.08})
work(band(150,290), {hand="broad", pile=sk2, angle=0, coverage=2, fill=true, angle_jitter=0.08})
work(band(270,395), {hand="broad", pile=sk3, angle=0, coverage=2, fill=true, angle_jitter=0.08})
work(band(380,470) * above(function(x) return 472 end), {hand="broad", pile=sk4, angle=0, coverage=2, fill=true, angle_jitter=0.08})

--@ chunk 4
blend(skym, {angle=0}); blend(skym, {angle=0.05})

--@ chunk 5
knoll = {{0,522},{150,515},{300,506},{450,492},{560,482},{640,478},{760,483},{880,497},{1000,508}}
hzf = function(x) return 460 + 3*math.sin(x/70) end
mist = pile{{"lead white",8},{"smalt",0.7},{"yellow ochre",0.25},{"raw umber",0.12}, medium=0.15}
sn1 = pile{{"lead white",7},{"smalt",0.9},{"raw umber",0.25},{"red earth",0.08}, medium=0.1}
sn2 = pile{{"lead white",5},{"smalt",1.4},{"raw umber",0.5},{"red earth",0.12}, medium=0.1}
distm = below(hzf) * above(knoll)
landm = below(knoll)
work(distm, {hand="body", pile=mist, angle=0, coverage=2.5, fill=true, length={30,80}})
-- near land: lighter at the crest, darker toward the bottom edge
work(landm * above(function(x) return 590 end), {hand="broad", pile=sn1, angle=0.03, coverage=2, fill=true})
work(landm * below(function(x) return 570 end), {hand="broad", pile=sn2, angle=-0.03, coverage=2, fill=true})
blend(landm, {angle=0})

--@ chunk 6
local tests = {
 pile{{"lead white",3},{"smalt",3},{"raw umber",0.4}},
 pile{{"lead white",4},{"cobalt blue",1},{"raw umber",0.2},{"red earth",0.1}},
 pile{{"lead white",6},{"Prussian blue",0.15},{"raw umber",0.2},{"red earth",0.1}},
 pile{{"lead white",4},{"smalt",2},{"vermilion",0.15},{"raw umber",0.3}},
 pile{{"raw umber",2},{"bone black",1},{"smalt",0.5}},
 pile{{"raw umber",2},{"red earth",0.5},{"lead white",1}},
 pile{{"lead white",5},{"red earth",0.3},{"smalt",1},{"yellow ochre",0.3}},
 pile{{"lead white",6},{"vermilion",0.3},{"yellow ochre",0.8}},
}
local b = brush("flat", 14)
for i,p in ipairs(tests) do
  local x = 30 + (i-1)*55
  b:reload(p, 0.9)
  for k=0,3 do b:stroke({{x, 660+k*9},{x+40, 660+k*9}}, {pressure=0.8}) end
end

--@ chunk 7
local tests = {
 pile{{"lead white",1},{"smalt",4},{"raw umber",0.3}},
 pile{{"lead white",1},{"cobalt blue",1.5},{"raw umber",0.2}},
 pile{{"lead white",3},{"Prussian blue",0.3},{"raw umber",0.3},{"red earth",0.1}},
 pile{{"lead white",1.5},{"smalt",3},{"red earth",0.25},{"raw umber",0.3}},
 pile{{"lead white",2},{"smalt",2},{"raw umber",0.8}},
 pile{{"lead white",3},{"vermilion",0.4},{"yellow ochre",1.2}},
 pile{{"lead white",3},{"chrome yellow",0.6},{"yellow ochre",0.4}},
 pile{{"lead white",3},{"smalt",1.5},{"vermilion",0.3},{"raw umber",0.2}},
}
local b = brush("flat", 14)
for i,p in ipairs(tests) do
  local x = 520 + (i-1)*58
  b:reload(p, 0.9)
  for k=0,3 do b:stroke({{x, 660+k*9},{x+40, 660+k*9}}, {pressure=0.8}) end
end

--@ chunk 8
print(wait(20*60)); print(drying(500,50), drying(500,420), drying(500,600), drying(700,670))

--@ chunk 9
print(wait(6*60)); print(drying(500,50), drying(500,420), drying(500,600), drying(700,670), drying(300,470))

--@ chunk 10
local function band(a, b) return below(function(x) return a end) * above(function(x) return b end) end
sA = pile{{"lead white",1.5},{"smalt",4},{"red earth",0.2},{"raw umber",0.3}, medium=0.2}
sB = pile{{"lead white",3},{"smalt",2.5},{"red earth",0.1},{"raw umber",0.15}, medium=0.2}
sC = pile{{"lead white",5},{"smalt",0.8},{"vermilion",0.08}, medium=0.2}
sD = pile{{"lead white",4},{"vermilion",0.2},{"yellow ochre",0.7}, medium=0.2}
sE = pile{{"lead white",4},{"chrome yellow",0.35},{"yellow ochre",0.3}, medium=0.2}
work(band(-10,130), {hand="broad", pile=sA, angle=0, coverage=2.2, fill=true, angle_jitter=0.06})
work(band(115,240), {hand="broad", pile=sB, angle=0, coverage=2.2, fill=true, angle_jitter=0.06})
work(band(225,330), {hand="broad", pile=sC, angle=0, coverage=2.2, fill=true, angle_jitter=0.06})
work(band(315,400), {hand="broad", pile=sD, angle=0, coverage=2.2, fill=true, angle_jitter=0.06})
work(band(388,466) * skym, {hand="broad", pile=sE, angle=0, coverage=2.2, fill=true, angle_jitter=0.06})
blend(skym, {angle=0}); blend(skym, {angle=0.03})

--@ chunk 11
local function band(a, b) return below(function(x) return a end) * above(function(x) return b end) end
stipple(band(-10,150), {pile=sA, width=7, coverage=function(x,y) return 1.4*clamp((170-y)/120,0,1) end, pressure={0.3,0.5}, feather=0.5})
stipple(band(120,260), {pile=sB, width=7, coverage=function(x,y) return 0.9*clamp(1-math.abs(y-180)/80,0,1) end, pressure={0.3,0.5}, feather=0.5})
blend(band(-10,300)*skym, {angle=0})

--@ chunk 12
local function band(a, b) return below(function(x) return a end) * above(function(x) return b end) end; blend(band(-10,330)*skym, {angle=0.02})

--@ chunk 13
print(wait(22*60)); print(drying(500,50), drying(500,440), drying(500,600))

--@ chunk 14
dist1 = pile{{"lead white",4},{"smalt",1.1},{"red earth",0.06},{"raw umber",0.1},{"yellow ochre",0.1}, medium=0.15}
dist2 = pile{{"lead white",4},{"smalt",1.5},{"red earth",0.08},{"raw umber",0.18}, medium=0.15}
work(distm * above(function(x) return 478 end), {hand="body", pile=dist1, angle=0, coverage=2.5, fill=true, clip=true, length={30,70}})
work(distm * below(function(x) return 472 end), {hand="body", pile=dist2, angle=0, coverage=2.5, fill=true, clip=true, length={30,70}})
blend(distm, {angle=0})

--@ chunk 15
crest = pile{{"lead white",5},{"yellow ochre",0.2},{"vermilion",0.08},{"smalt",0.25}, medium=0.1}
knb = pile{{"lead white",4},{"smalt",1},{"vermilion",0.06},{"raw umber",0.12}, medium=0.1}
fgs = pile{{"lead white",3},{"smalt",1.5},{"raw umber",0.3},{"red earth",0.1}, medium=0.1}
kang = function(x,y) local f = clamp(1-(y-480)/260,0,1); if x < 640 then return -0.07*f else return 0.09*f end end
knollf = function(x) -- piecewise linear over knoll points
  for i=1,#knoll-1 do local a,b = knoll[i],knoll[i+1]; if x>=a[1] and x<=b[1] then return lerp(a[2],b[2],(x-a[1])/(b[1]-a[1])) end end
  return knoll[#knoll][2] end
local crestm = landm * above(function(x) return knollf(x)+45 end)
local bodym = below(function(x) return knollf(x)+30 end) * above(function(x) return 610 end)
local fgm = below(function(x) return 590 end)
work(fgm, {hand="broad", pile=fgs, angle=kang, coverage=2, fill=true})
work(bodym, {hand="body", pile=knb, angle=kang, coverage=2.2, fill=true, length={40,90}})
work(crestm, {hand="body", pile=crest, angle=kang, coverage=2.5, fill=true, clip=true, length={30,80}})
blend(landm, {angle=kang})

--@ chunk 16
function firpoly(x, top, bot, w, tiers, seed)
  math.randomseed(seed)
  local L, R = {}, {}
  for i=0,tiers do
    local t = i/tiers
    local y = lerp(top, bot, t)
    local hw = w/2 * (0.15 + 0.85*t)
    local jig = (i%2==0) and 1 or 0.6
    L[#L+1] = {x - hw*jig*rand(0.85,1.1), y}
    R[#R+1] = {x + hw*jig*rand(0.85,1.1), y + rand(-1,1)}
  end
  local pts = {{x, top-3}}
  for i=1,#R do pts[#pts+1] = R[i] end
  for i=#L,1,-1 do pts[#pts+1] = L[i] end
  return poly(pts)
end
local hzy = 462
-- low distant wood band on the left
woodL = poly({{0,hzy+2},{0,446},{40,448},{80,443},{120,447},{160,441},{240,445},{290,448},{335,455},{350,hzy+2}}, true)
woodL = woodL:roughen(2.5, 12, 3)
firs = nil
for i,f in ipairs({{25,418,8},{58,424,7},{92,414,9},{132,428,6},{150,420,8},{255,426,7},{285,432,6},{312,437,5}}) do
  local m = firpoly(f[1], f[2], hzy, f[3]*2.2, 7, i*17)
  firs = firs and (firs + m) or m
end
woodR = poly({{860,hzy+2},{880,452},{920,447},{960,450},{1000,446},{1000,hzy+2}}, true):roughen(2.5, 10, 5)
for i,f in ipairs({{905,430,6},{935,424,7},{975,433,6}}) do woodR = woodR + firpoly(f[1], f[2], hzy, f[3]*2.2, 6, i*31) end
farwood = woodL + firs + woodR
fw = pile{{"lead white",3},{"smalt",1.6},{"raw umber",0.22},{"red earth",0.08}, medium=0.2}
work(farwood, {hand="detail", pile=fw, coverage=2.5, fill=true, angle=math.pi/2})

--@ chunk 17
function fir2(x, top, bot, w, tiers, seed)
  math.randomseed(seed)
  local R, L = {}, {}
  for i=1,tiers do
    local t = i/tiers
    local y = lerp(top, bot, t)
    local hw = w/2 * t^0.9
    R[#R+1] = {x + hw*rand(0.8,1.15), y + hw*0.25}   -- drooping branch tip
    R[#R+1] = {x + hw*0.35, y + (bot-top)/tiers*0.3}
    L[#L+1] = {x - hw*rand(0.8,1.15), y + hw*0.25 + rand(-0.5,0.5)}
    L[#L+1] = {x - hw*0.35, y + (bot-top)/tiers*0.3}
  end
  local pts = {{x, top-2}}
  for i=1,#R do pts[#pts+1] = R[i] end
  pts[#pts+1] = {x, bot+2}
  for i=#L,1,-1 do pts[#pts+1] = L[i] end
  return poly(pts)
end
local hzy = 462
math.randomseed(7)
local xs = uneven(26, 5, 330, 0.6, 0.5, 11)
firs2 = nil
for i,x in ipairs(xs) do
  local h = 22 + 22*math.abs(math.sin(x/47)) + rand(-5,6)
  if x > 260 then h = h*0.7 end
  local m = fir2(x, hzy-8-h, hzy-4, h*0.42, 8, 100+i)
  firs2 = firs2 and (firs2 + m) or m
end
local xr = uneven(7, 885, 998, 0.5, 0.4, 12)
for i,x in ipairs(xr) do firs2 = firs2 + fir2(x, hzy-12-rand(14,30), hzy-4, 14, 7, 200+i) end
work(firs2, {hand="detail", pile=fw, coverage=2.5, fill=true, angle=math.pi/2})

--@ chunk 18
chp = pile{{"lead white",4},{"smalt",1.3},{"raw umber",0.15},{"red earth",0.08}, medium=0.2}
church = poly({{168,455},{168,418},{171,414},{200,392},{204,390},{230,413},{234,417},{234,455}})
tower = poly({{236,455},{236,380},{239,376},{241,379},{243,373},{246,377},{249,371},{251,376},{252,455}})
-- buttress stubs
church = church + rect(165,425,5,30) + rect(232,422,5,33) + rect(198,386,5,8)
work(church + tower, {hand="detail", pile=chp, coverage=3, fill=true, angle=math.pi/2})
-- window openings showing the lemon sky
winp = pile{{"lead white",4},{"chrome yellow",0.3},{"yellow ochre",0.3}}
local wins = nil
for _,wx in ipairs({184,200,216}) do
  local wm = poly({{wx-3,445},{wx-3,420},{wx,414},{wx+3,420},{wx+3,445}})
  wins = wins and (wins+wm) or wm
end
wins = wins + poly({{241,410},{241,396},{244,392},{247,396},{247,410}})
work(wins, {hand="detail", pile=winp, coverage=3, fill=true, angle=math.pi/2, tool={kind="round", width=1.4, point=0.8}})
-- firs in front of the church, restated
work(firs2 * rect(150,380,110,90), {hand="detail", pile=fw, coverage=2.5, fill=true, angle=math.pi/2})

--@ chunk 19
print(wait(26*60)); print(drying(500,50), drying(500,470), drying(500,520), drying(500,650), drying(200,430))

--@ chunk 20
print(wait(14*60)); print(drying(500,470), drying(500,520), drying(200,430), drying(640,490))

--@ chunk 21
rkD = pile{{"raw umber",2},{"smalt",1},{"bone black",0.35},{"lead white",0.5}, medium=0.1}
rkM = pile{{"raw umber",1.5},{"lead white",1.5},{"smalt",0.7},{"red earth",0.2}, medium=0.1}
rkL = pile{{"lead white",2.5},{"raw umber",0.8},{"smalt",0.5},{"red earth",0.15},{"yellow ochre",0.2}, medium=0.1}
chamber = poly({{566,458},{716,456},{712,487},{570,489}}, true)
work(chamber, {hand="detail", pile=rkD, coverage=3, fill=true, angle=0})
ups = {}
local defs = {
 {{560,462,"c"},{556,475},{554,489,"c"},{584,491,"c"},{584,476},{580,462,"c"}},
 {{604,464,"c"},{601,478},{603,489,"c"},{626,488,"c"},{628,474},{622,464,"c"}},
 {{653,464,"c"},{650,478},{652,487,"c"},{674,488,"c"},{677,476},{672,464,"c"}},
 {{696,461,"c"},{694,474},{697,488,"c"},{722,487,"c"},{723,472},{716,460,"c"}},
}
upm = nil
for i,d in ipairs(defs) do d.char="broken"; d.seed=40+i; d.closed=true; local o = outline(d); ups[i]=o; upm = upm and (upm + o:mask()) or o:mask() end
work(upm, {hand="detail", pile=rkM, coverage=2.5, fill=true, angle=math.pi/2})
-- darker lower and left parts of each upright (backlit, shadowed toward us)
work(upm * below(function(x) return 474 end), {hand="detail", pile=rkD, coverage=1.5, fill=true, angle=math.pi/2})
cap = outline{{550,458,"c"},{556,447},{575,440},{610,435},{655,434},{695,437},{720,445},{729,454,"c"},{722,464},{690,467},{640,468},{590,467},{558,465}, closed=true, char="soft", seed=77}
capm = cap:mask()
work(capm, {hand="detail", pile=rkM, coverage=3, fill=true, angle=0.05})
work(capm * below(function(x) return 455 end), {hand="detail", pile=rkD, coverage=1.6, fill=true, angle=0.05})

--@ chunk 22
print(wait(6*60)); print(drying(640,450), drying(640,475), drying(590,480))

--@ chunk 23
print(wait(10*60)); print(drying(640,450), drying(640,475), drying(590,480))

--@ chunk 24
print(wait(10*60)); print(drying(640,450), drying(640,475), drying(590,480), drying(300,470))

--@ chunk 25
local b = brush{kind="round", width=4, point=1}; local r = brush{kind="rigger", width=1.5, point=1}; for _,p in ipairs({0.05,0.1,0.2,0.4,0.7,1}) do print(p, b:mark_width(p), r:mark_width(p)) end; for _,w in ipairs({0.3,0.6,1,2,3}) do print(w, b:pressure_for(w), r:pressure_for(w)) end

--@ chunk 26
print(ribbon({{100,100},{200,100}}, 10):area(), ribbon({{100,100},{200,100}}, {10,2}):area())

--@ chunk 27
function tree_new(cx, spread, o) return {limbs={}, cx=cx, spread=spread, o=o} end
function tree_grow(T, x, y, ang, len, w, order, dead, depthcap)
  local o = T.o
  local pts, ws = {{x,y}}, {w}
  local travelled, side = 0, (math.random()<0.5) and 1 or -1
  local seg = math.max(2.5, math.min(o.seglen, len/3.5))
  local nodes = 0
  while travelled < len and w >= o.wmin do
    local s = seg * rand(0.7, 1.3)
    ang = ang + randn(0, o.zig)
    local tgt = -math.pi/2 + clamp((x - T.cx)/T.spread, -1, 1) * o.fan
    if order >= o.droop_order then tgt = tgt + o.droop * (tgt > -math.pi/2 and 1 or -1) end
    ang = ang + (tgt - ang) * o.trop
    x = x + s*math.cos(ang); y = y + s*math.sin(ang)
    travelled = travelled + s
    w = w * (1 - o.taper*s/len)
    pts[#pts+1] = {x, y}; ws[#ws+1] = w
    nodes = nodes + 1
    local canbranch = not (dead and order >= 3) and (depthcap == nil or order < depthcap)
    if canbranch and travelled < len*0.97 and math.random() < o.branchp and w > o.wmin*1.25 then
      local cw = w * rand(o.cmin, o.cmax)
      local nw = math.sqrt(math.max(w*w - cw*cw*0.9, o.wmin^2))
      side = -side
      local ca = ang + side*rand(0.4, 1.05)
      local clen = (len - travelled)*rand(0.5, 0.9) + len*0.12
      tree_grow(T, x, y, ca, clen, cw, order+1, dead, depthcap)
      w = nw
      ang = ang - side*rand(0.05, 0.3)
    end
  end
  T.limbs[#T.limbs+1] = {pts=pts, ws=ws, order=order, dead=dead}
end
-- add a hand-placed limb (explicit polyline + widths) and sprout branches along it
function tree_main(T, pts, ws, order, dead, sprout)
  T.limbs[#T.limbs+1] = {pts=pts, ws=ws, order=order, dead=dead, main=true}
  if sprout then
    for i=2,#pts do
      local a, b = pts[i-1], pts[i]
      local ang = math.atan(b[2]-a[2], b[1]-a[1])
      local nb = sprout.n or 1
      for k=1,nb do
        if math.random() < (sprout.p or 0.8) then
          local t = rand(0.2, 1)
          local x, y = lerp(a[1],b[1],t), lerp(a[2],b[2],t)
          local w = lerp(ws[i-1], ws[i], t)
          local side = (math.random()<0.5) and 1 or -1
          local cw = w * rand(sprout.cmin or 0.3, sprout.cmax or 0.6)
          tree_grow(T, x, y, ang + side*rand(0.5,1.1), sprout.len * rand(0.6,1.1), cw, order+1, dead)
        end
      end
    end
    -- continue from the tip
    local a, b = pts[#pts-1], pts[#pts]
    if not dead and sprout.tip then
      tree_grow(T, b[1], b[2], math.atan(b[2]-a[2], b[1]-a[1]), sprout.tip, ws[#ws], order, dead)
    end
  end
end
function tree_stats(T)
  local n, x0, y0, x1, y1 = 0, 1e9, 1e9, -1e9, -1e9
  for _,l in ipairs(T.limbs) do n = n + #l.pts; for _,p in ipairs(l.pts) do x0=math.min(x0,p[1]); x1=math.max(x1,p[1]); y0=math.min(y0,p[2]); y1=math.max(y1,p[2]) end end
  return string.format("limbs %d pts %d box %.0f,%.0f - %.0f,%.0f", #T.limbs, n, x0,y0,x1,y1)
end
-- the thick parts as one mask
function tree_thick_mask(T, wthr)
  local m
  for _,l in ipairs(T.limbs) do
    if l.ws[1] >= wthr then
      local r = ribbon(l.pts, l.ws)
      m = m and (m + r) or r
    end
  end
  return m
end
-- paint the thin limbs as strokes; filter(l) picks which
function tree_paint_thin(T, p, wthr, filter, loadv)
  local bm = brush{kind="round", width=4, point=1}
  local bt = brush{kind="rigger", width=2, point=1}
  bm:load(p, 0.8); bt:load(p, 0.8)
  local n = 0
  for _,l in ipairs(T.limbs) do
    if l.ws[1] < wthr and (filter == nil or filter(l)) and #l.pts >= 2 then
      local b = (l.ws[1] > 1.6) and bm or bt
      if b:fullness() < 0.35 then b:load(p, loadv or 0.7) end
      local p0 = b:pressure_for(math.max(l.ws[1], 0.6))
      local p1 = b:pressure_for(math.max(l.ws[#l.ws], 0.6))
      b:stroke(l.pts, {pressure={p0, p1*0.6}, ramps={0.02, 0.25}, shake=0.3})
      n = n + 1
    end
  end
  return n
end

--@ chunk 28
math.randomseed(1774)
OAK = tree_new(410, 220, {wmin=0.6, zig=0.3, trop=0.07, fan=0.95, seglen=18, taper=0.3, branchp=0.5, cmin=0.45, cmax=0.8, droop_order=99, droop=0})
tree_main(OAK, {{405,512},{403,470},{400,430},{399,390},{401,355},{404,330}}, {46,40,36,33,31,30}, 1, false, nil)
tree_main(OAK, {{400,335},{385,300},{365,265},{350,235},{338,200},{322,165},{312,130},{300,105}}, {20,18,16,14,12,10,8,6}, 2, false, {len=85, p=0.9, n=2, tip=70, cmin=0.3, cmax=0.55})
tree_main(OAK, {{405,332},{412,295},{414,255},{408,215},{412,175}}, {20,18,16,14,12}, 2, false, {len=80, p=0.9, n=2, cmin=0.3, cmax=0.55})
tree_main(OAK, {{412,178},{418,140},{413,105},{419,75},{416,58}}, {11,9,7,5,3.5}, 2, true, {len=22, p=0.6, n=1, cmin=0.3, cmax=0.5})
tree_main(OAK, {{410,335},{435,305},{465,280},{500,255},{535,232},{570,212},{600,198}}, {18,16,14,12,10,8,6}, 2, false, {len=85, p=0.9, n=2, tip=70, cmin=0.3, cmax=0.55})
tree_main(OAK, {{399,385},{375,378},{345,372},{315,366},{285,357},{255,350},{230,338}}, {13,11,10,8.5,7,6,5}, 2, false, {len=65, p=0.9, n=2, tip=50, cmin=0.3, cmax=0.6})
tree_main(OAK, {{408,398},{435,392},{465,393},{495,390},{525,384},{552,374}}, {10,9,8,7,6,5}, 2, false, {len=55, p=0.9, n=2, tip=45, cmin=0.3, cmax=0.6})
tree_main(OAK, {{468,282},{488,242},{497,207},{509,172},{514,152}}, {7,6,5,4,3}, 2, true, {len=20, p=0.5, n=1, cmin=0.35, cmax=0.5})
tree_main(OAK, {{398,422},{384,416}}, {9,8}, 2, true, nil)
print(tree_stats(OAK))
local c = {0,0,0,0}
for _,l in ipairs(OAK.limbs) do local w=l.ws[1]; if w>=3.5 then c[1]=c[1]+1 elseif w>=1.6 then c[2]=c[2]+1 else c[3]=c[3]+1 end end
print(c[1], c[2], c[3])

--@ chunk 29
barkD = pile{{"raw umber",2},{"bone black",0.5},{"smalt",0.5},{"lead white",0.35}, medium=0.1}
twigP = pile{{"raw umber",2},{"bone black",0.35},{"smalt",0.6},{"lead white",0.7}, medium=0.15}
local trunkflare = poly({{372,514},{380,505},{385,490},{386,470},{388,440},{423,440},{424,470},{426,490},{432,505},{442,514}}, true)
OAKthick = tree_thick_mask(OAK, 3.5) + trunkflare
work(OAKthick, {hand="detail", pile=barkD, coverage=3, fill=true, angle=math.pi/2})
print(tree_paint_thin(OAK, barkD, 3.5, function(l) return l.ws[1] >= 1.6 end))
print(tree_paint_thin(OAK, twigP, 3.5, function(l) return l.ws[1] < 1.6 end))

--@ chunk 30
function tree_twigs(T, wmax, per, lenr, seed)
  math.randomseed(seed)
  local TT = tree_new(T.cx, T.spread, {wmin=0.55, zig=0.45, trop=0.05, fan=0.9, seglen=5, taper=0.35, branchp=0.45, cmin=0.6, cmax=0.85, droop_order=99, droop=0})
  local src = {}
  for _,l in ipairs(T.limbs) do if not l.dead and l.ws[1] < wmax and #l.pts >= 2 then src[#src+1] = l end end
  for _,l in ipairs(src) do
    for i=2,#l.pts do
      local a, b = l.pts[i-1], l.pts[i]
      local segl = math.sqrt((b[1]-a[1])^2 + (b[2]-a[2])^2)
      local k = math.floor(segl/10*per + math.random())
      local ang = math.atan(b[2]-a[2], b[1]-a[1])
      for j=1,k do
        local t = rand(0.1, 1)
        local x, y = lerp(a[1],b[1],t), lerp(a[2],b[2],t)
        local w = lerp(l.ws[i-1], l.ws[i], t)
        local side = (math.random()<0.5) and 1 or -1
        tree_grow(TT, x, y, ang + side*rand(0.4,1.2), rand(lenr[1], lenr[2]), math.min(w*0.7, rand(0.8,1.3)), 5, false)
      end
    end
    -- a brush of twigs at the tip
    local e = l.pts[#l.pts]; local d = l.pts[#l.pts-1]
    local ang = math.atan(e[2]-d[2], e[1]-d[1])
    for j=1,math.random(2,4) do tree_grow(TT, e[1], e[2], ang + rand(-0.7,0.7), rand(lenr[1], lenr[2])*1.2, rand(0.7,1.0), 5, false) end
  end
  return TT
end
OAKT = tree_twigs(OAK, 5, 1.2, {8, 26}, 99)
print(tree_stats(OAKT))

--@ chunk 31
print(tree_paint_thin(OAKT, twigP, 99, nil, 0.6))

--@ chunk 32
math.randomseed(4242)
OAKX = tree_new(OAK.cx, OAK.spread, OAK.o)
for _,l in ipairs(OAK.limbs) do
  local n = #l.pts
  if n >= 2 and not l.main and l.ws[n] > 1.2 and not l.dead then
    local e, d = l.pts[n], l.pts[n-1]
    local ang = math.atan(e[2]-d[2], e[1]-d[1])
    tree_grow(OAKX, e[1], e[2], ang, 10*l.ws[n] + 15, l.ws[n]*0.95, l.order, false)
  end
end
print(tree_stats(OAKX))
print(tree_paint_thin(OAKX, barkD, 99, function(l) return l.ws[1] >= 1.6 end))
print(tree_paint_thin(OAKX, twigP, 99, function(l) return l.ws[1] < 1.6 end))
OAKXT = tree_twigs(OAKX, 5, 1.2, {8, 24}, 98)
print(tree_paint_thin(OAKXT, twigP, 99, nil, 0.6))

--@ chunk 33
barkK = pile{{"raw umber",1.5},{"bone black",1},{"smalt",0.7},{"lead white",0.2}, medium=0.1}
local tf = poly({{378,508},{384,500},{387,488},{388,470},{390,440},{421,440},{422,470},{424,488},{428,500},{436,508}}, true)
local mx = tree_thick_mask(OAKX, 3.5)
OAKthick2 = tree_thick_mask(OAK, 3.5) + tf
if mx then OAKthick2 = OAKthick2 + mx end
OAKthick2 = OAKthick2:roughen(1.2, 7, 5)
work(OAKthick2, {hand="detail", pile=barkK, coverage=2.5, fill=true, angle=math.pi/2})
-- mid limbs darker too
local b = brush{kind="round", width=4, point=1}
b:load(barkK, 0.8)
for _,T in ipairs({OAK, OAKX}) do
  for _,l in ipairs(T.limbs) do
    if l.ws[1] < 3.5 and l.ws[1] >= 1.8 and #l.pts>=2 then
      if b:fullness() < 0.35 then b:load(barkK, 0.7) end
      b:stroke(l.pts, {pressure={b:pressure_for(l.ws[1]*0.9), b:pressure_for(math.max(l.ws[#l.ws]*0.9,0.6))*0.6}, ramps={0.02,0.25}, shake=0.3})
    end
  end
end

--@ chunk 34
math.randomseed(1810)
OAK2 = tree_new(868, 130, {wmin=0.6, zig=0.32, trop=0.07, fan=0.95, seglen=16, taper=0.3, branchp=0.5, cmin=0.45, cmax=0.8, droop_order=99, droop=0})
tree_main(OAK2, {{850,504},{848,460},{846,420},{848,380},{852,345}}, {30,26,24,22,21}, 1, false, nil)
tree_main(OAK2, {{848,352},{831,320},{819,302}}, {13,12,11}, 2, true, nil)
tree_main(OAK2, {{855,348},{868,310},{880,275},{886,240},{882,205},{890,170},{895,150}}, {16,14,12,10,8,6,4.5}, 2, false, {len=70, p=0.9, n=2, tip=45, cmin=0.3, cmax=0.55})
tree_main(OAK2, {{848,392},{825,380},{800,372},{775,356},{752,348}}, {9,8,7,6,5}, 2, false, {len=50, p=0.9, n=2, tip=40, cmin=0.3, cmax=0.6})
tree_main(OAK2, {{874,287},{905,272},{935,262},{962,246},{985,238}}, {8,7,6,5,4}, 2, false, {len=50, p=0.9, n=2, tip=35, cmin=0.3, cmax=0.6})
tree_main(OAK2, {{881,242},{863,212},{851,187},{846,166}}, {5,4,3.5,3}, 2, true, {len=15, p=0.5, n=1, cmin=0.35, cmax=0.5})
-- extend blunt ends
local ext = tree_new(OAK2.cx, OAK2.spread, OAK2.o)
for _,l in ipairs(OAK2.limbs) do
  local n = #l.pts
  if n >= 2 and not l.main and l.ws[n] > 1.2 and not l.dead then
    local e, d = l.pts[n], l.pts[n-1]
    tree_grow(ext, e[1], e[2], math.atan(e[2]-d[2], e[1]-d[1]), 10*l.ws[n]+12, l.ws[n]*0.95, l.order, false)
  end
end
for _,l in ipairs(ext.limbs) do OAK2.limbs[#OAK2.limbs+1] = l end
print(tree_stats(OAK2))
local tf = poly({{830,508},{836,500},{838,488},{839,470},{840,450},{857,450},{858,470},{860,488},{864,500},{872,508}}, true)
OAK2thick = (tree_thick_mask(OAK2, 3.5) + tf):roughen(1.2, 7, 6)
work(OAK2thick, {hand="detail", pile=barkD, coverage=3, fill=true, angle=math.pi/2})
print(tree_paint_thin(OAK2, barkD, 3.5, function(l) return l.ws[1] >= 1.6 end))
print(tree_paint_thin(OAK2, twigP, 3.5, function(l) return l.ws[1] < 1.6 end))
OAK2T = tree_twigs(OAK2, 5, 1.2, {7, 22}, 77)
print(tree_paint_thin(OAK2T, twigP, 99, nil, 0.6))

--@ chunk 35
print(wait(20*60)); print(drying(400,450), drying(420,250), drying(850,420), drying(640,450))

--@ chunk 36
print(wait(16*60)); print(drying(400,450), drying(420,250), drying(850,420), drying(640,450), drying(300,300))

--@ chunk 37
print(wait(24*60)); print(drying(400,450), drying(420,250), drying(850,420), drying(640,450), drying(560,300))

--@ chunk 38
rkK = pile{{"bone black",1},{"raw umber",1},{"smalt",0.3},{"lead white",0.1}, medium=0.05}
rkD2 = pile{{"raw umber",2},{"bone black",0.6},{"smalt",0.8},{"lead white",0.4}, medium=0.05}
rkL2 = pile{{"lead white",2},{"raw umber",0.8},{"smalt",0.6},{"yellow ochre",0.1}, medium=0.05}
-- chamber, darker
local ch = poly({{572,462},{712,460},{710,486},{574,488}})
work(ch, {hand="detail", pile=rkK, coverage=3, fill=true, angle=0})
-- uprights restated, darker, varied
local defs = {
 {{558,462,"c"},{553,476},{551,490,"c"},{585,492,"c"},{586,476},{581,461,"c"}},
 {{605,466,"c"},{600,479},{602,490,"c"},{627,489,"c"},{629,475},{623,465,"c"}},
 {{652,466,"c"},{648,479},{651,488,"c"},{676,489,"c"},{678,476},{672,465,"c"}},
 {{697,462,"c"},{694,474},{696,489,"c"},{724,488,"c"},{726,472},{717,460,"c"}},
}
UPM = nil
for i,d in ipairs(defs) do d.char="broken"; d.seed=60+i; d.closed=true; local o = outline(d); UPM = UPM and (UPM + o:mask()) or o:mask() end
work(UPM, {hand="detail", pile=rkD2, coverage=2.5, fill=true, angle=math.pi/2})
-- capstone: massive, irregular
CAP = outline{{546,456,"c"},{553,446},{570,441},{600,437},{640,436},{680,437},{712,441},{730,448},{735,457,"c"},{728,466},{705,470},{660,472},{615,471},{575,469},{552,465}, closed=true, char="broken", seed=91, amount=0.8}
CAPM = CAP:mask()
work(CAPM, {hand="detail", pile=rkD2, coverage=3, fill=true, angle=0.08})

--@ chunk 39
local ch = poly({{586,466},{602,466},{604,487},{586,488}}) + poly({{629,467},{651,467},{651,487},{628,488}}) + poly({{678,466},{696,465},{695,487},{677,488}})
work(ch, {hand="detail", pile=rkK, coverage=5, fill=true, angle=0, load=1, pressure={0.7,0.9}})

--@ chunk 40
work(OAKthick2, {hand="detail", pile=barkK, coverage=3, fill=true, angle=math.pi/2, load=1, pressure={0.6,0.9}})
work(OAK2thick, {hand="detail", pile=barkK, coverage=3, fill=true, angle=math.pi/2, load=1, pressure={0.6,0.9}})

--@ chunk 41
fg2 = pile{{"lead white",3},{"smalt",1.3},{"raw umber",0.25},{"red earth",0.08},{"vermilion",0.03}, medium=0.1}
fg3 = pile{{"lead white",2},{"smalt",1.5},{"raw umber",0.45},{"red earth",0.12}, medium=0.1}
litS = pile{{"lead white",5},{"yellow ochre",0.15},{"vermilion",0.05},{"smalt",0.3}, medium=0.1}
local ang = function(x,y) return 0.03*math.sin(x/160) + (x<500 and -0.02 or 0.02) end
work(below(function(x) return 575 + 10*math.sin(x/130) end) * above(function(x) return 655 end), {hand="broad", pile=fg2, angle=ang, coverage=2.2, fill=true})
work(below(function(x) return 640 + 8*math.sin(x/110) end), {hand="broad", pile=fg3, angle=ang, coverage=2.2, fill=true})
blend(below(function(x) return 570 end), {angle=0})

--@ chunk 42
shS = pile{{"lead white",2.2},{"smalt",1.6},{"raw umber",0.3},{"red earth",0.1}, medium=0.1}
local crests = {
 {{-10,603},{120,592},{260,586},{400,590},{500,598}},
 {{540,612},{680,600},{820,596},{1010,606}},
 {{-10,668},{150,656},{320,652},{520,668}},
 {{620,684},{800,668},{1010,662}},
}
local lit, sh
for _,c in ipairs(crests) do
  local ws = {}; local wsd = {}
  for i=1,#c do local t=(i-1)/(#c-1); ws[i] = 3 + 7*math.sin(math.pi*t); wsd[i] = 4 + 10*math.sin(math.pi*t) end
  local shp = {}; for i,p in ipairs(c) do shp[i] = {p[1], p[2] + ws[i]*0.5 + wsd[i]*0.5} end
  local r1 = ribbon(c, ws); local r2 = ribbon(shp, wsd)
  lit = lit and (lit + r1) or r1; sh = sh and (sh + r2) or r2
end
sh = sh - lit
work(sh, {hand="body", pile=shS, angle=0, coverage=2, fill=true, clip=true, length={30,80}})
work(lit, {hand="body", pile=litS, angle=0, coverage=2, fill=true, clip=true, length={30,80}})
blend((lit + sh):grow(6), {angle=0, clip=false})
blend(below(function(x) return 540 end)*above(function(x) return 600 end), {angle=0})

--@ chunk 43
print(wait(36*60)); print(drying(400,450), drying(420,250), drying(850,420), drying(640,450), drying(300,650), drying(600,478))

--@ chunk 44
print(wait(3*24*60)); print(drying(400,450), drying(420,250), drying(850,420), drying(640,450), drying(600,478), drying(640,475))

--@ chunk 45
print(wait(5*24*60)); print(drying(400,450), drying(420,250), drying(850,420), drying(600,478), drying(640,475))

--@ chunk 46
-- chamber openings, irregular
local g1 = poly({{584,468},{590,466},{603,468},{606,478},{604,490},{586,491},{583,480}}, true)
local g2 = poly({{628,469},{640,467},{652,469},{653,480},{651,490},{629,490},{627,480}}, true)
local g3 = poly({{677,468},{688,466},{697,467},{697,479},{696,490},{678,490},{676,480}}, true)
work(g1+g2+g3, {hand="detail", pile=rkK, coverage=5, fill=true, angle=0, load=1, pressure={0.7,0.95}})
-- uprights: darker body, then granite speckle
work(UPM, {hand="detail", pile=rkD2, coverage=3, fill=true, angle=math.pi/2, load=1, pressure={0.6,0.9}})
-- capstone rock face
work(CAPM, {hand="detail", pile=rkD2, coverage=3, fill=true, angle=0.06, load=1, pressure={0.6,0.9}})

--@ chunk 47
rkA = pile{{"raw umber",2},{"bone black",0.8},{"smalt",0.6},{"lead white",0.12}, medium=0.05}
rkB = pile{{"raw umber",2},{"red earth",0.3},{"bone black",0.5},{"lead white",0.25}, medium=0.05}
rkC = pile{{"lead white",1.2},{"raw umber",1},{"smalt",0.7},{"bone black",0.2}, medium=0.05}
work(UPM + CAPM, {hand="detail", pile=rkA, coverage=3, fill=true, angle=math.pi/2, load=1, pressure={0.6,0.9}})
-- warm patches and cooler facets, granite blotches
local n = noise{seed=5, period=18, octaves=3}
stipple(UPM + CAPM, {pile=rkB, width=3, coverage=function(x,y) return 0.8*clamp(n(x,y)+0.2,0,1) end, pressure={0.4,0.7}})
-- upper face of the capstone catches cool skylight
local capTop = CAPM * above(function(x) return 452 + 4*math.sin(x/23) end)
work(capTop, {hand="detail", pile=rkC, coverage=1.5, fill=false, angle=0.05, pressure={0.3,0.6}})

--@ chunk 48
barkL = pile{{"lead white",1.5},{"raw umber",0.8},{"smalt",0.6},{"bone black",0.1}, medium=0.1}
barkF = pile{{"bone black",1},{"raw umber",1},{"smalt",0.2}, medium=0.1}
lichen = pile{{"green earth",1.2},{"lead white",1},{"yellow ochre",0.2},{"raw umber",0.1}, medium=0.1}
local function trunk_bark(x0, x1, y0, y1, seed, cxf)
  math.randomseed(seed)
  local b = brush{kind="round", width=2, point=0.8}
  local f = brush{kind="round", width=1.6, point=1}
  -- light ridges: short vertical wavy strokes, denser toward the left edge
  b:load(barkL, 0.5)
  for i=1,140 do
    local y = rand(y0, y1)
    local cx, hw = cxf(y)
    local u = rand(-1, 1)
    if math.random() < 0.55 + 0.35*(-u) then
      local x = cx + u*hw*0.9
      local len = rand(6, 16)
      if b:fullness() < 0.2 then b:load(barkL, 0.5) end
      b:stroke({{x, y}, {x + rand(-1,1), y + len*0.5}, {x + rand(-1.5,1.5), y + len}}, {pressure={0.35,0.15}, shake=0.6})
    end
  end
  -- dark fissures
  f:load(barkF, 0.8)
  for i=1,110 do
    local y = rand(y0, y1)
    local cx, hw = cxf(y)
    local x = cx + rand(-0.9, 0.9)*hw
    local len = rand(8, 22)
    if f:fullness() < 0.3 then f:load(barkF, 0.8) end
    f:stroke({{x, y}, {x + rand(-1.2,1.2), y + len*0.5}, {x + rand(-1.5,1.5), y + len}}, {pressure={0.5,0.2}, shake=0.5})
  end
  -- lichen dabs, mostly on one side and low
  local s = brush{kind="round", width=2.5, point=0.3}
  s:load(lichen, 0.6)
  for i=1,35 do
    local y = rand(lerp(y0,y1,0.3), y1)
    local cx, hw = cxf(y)
    local x = cx + rand(0.1, 0.95)*hw
    if s:fullness() < 0.25 then s:load(lichen, 0.6) end
    s:touch(x, y, {pressure=rand(0.2,0.45), drag={rand(-1,1), rand(0,2)}})
  end
end
trunk_bark(386, 424, 335, 505, 11, function(y) local t=(y-330)/180; return 403 - 2*t, 15 + 5*t^3 end)
trunk_bark(838, 862, 350, 500, 12, function(y) local t=(y-350)/150; return 849, 10 + 4*t^3 end)
-- the hollow where the old limb broke off
local hole = ellipse(392, 424, 4, 6)
work(hole, {hand="detail", pile=rkK, coverage=4, fill=true, load=1, pressure={0.6,0.9}})
local lipb = brush{kind="round", width=1.5, point=1}; lipb:load(barkL, 0.5)
lipb:stroke({{388,418},{392,416.5},{397,419}}, {pressure={0.4,0.2}})

--@ chunk 49
print(wait(30*60)); print(drying(400,450), drying(850,420), drying(640,450), drying(600,478))

--@ chunk 50
glzD = pile{{"raw umber",1.5},{"bone black",0.8},{"smalt",0.4}, medium=0.65}
local t1 = OAKthick2 * rect(370,330,75,190)
work(t1, {hand="detail", pile=glzD, coverage=2, fill=true, angle=math.pi/2, tool={kind="filbert", width=5}, pressure={0.4,0.6}})

--@ chunk 51
moonP = pile{{"lead white",5},{"chrome yellow",0.2},{"yellow ochre",0.05}}
local mm = ellipse(752, 150, 8.5, 8.5) - ellipse(749.5, 146.2, 8.3, 8.3)
work(mm, {hand="detail", pile=moonP, coverage=4, fill=true, load=1, tool={kind="round", width=1.2, point=1}, pressure={0.5,0.8}})
-- faint ashen light on the dark part
local ash = pile{{"lead white",3},{"smalt",1.3},{"raw umber",0.1}, medium=0.5}
local am = ellipse(752, 150, 8.3, 8.3) - ellipse(752, 150, 8.5, 8.5):shrink(0) * (-(ellipse(749.5,146.2,8.3,8.3)))

--@ chunk 52
crowP = pile{{"bone black",1.2},{"raw umber",0.6},{"smalt",0.2}, medium=0.05}
function crow_fly(x, y, span, tilt, phase)
  local b = brush{kind="round", width=math.max(1.4, span*0.12), point=1}
  b:load(crowP, 0.9)
  local s = span/2
  local c, sn = math.cos(tilt), math.sin(tilt)
  local function P(u, v) return {x + u*c - v*sn, y + u*sn + v*c} end
  local up = phase  -- wing raise: positive = wings up
  -- left wing
  b:stroke({P(-0.5,0), P(-s*0.45, -s*0.28*up - s*0.05), P(-s, -s*0.12*up + s*0.12)}, {pressure={b:pressure_for(span*0.09), 0}, ramps={0.05,0.6}})
  b:stroke({P(0.5,0), P(s*0.45, -s*0.28*up - s*0.05), P(s, -s*0.12*up + s*0.12)}, {pressure={b:pressure_for(span*0.09), 0}, ramps={0.05,0.6}})
  -- body
  b:stroke({P(-span*0.02, -span*0.04), P(0, span*0.12)}, {pressure={b:pressure_for(span*0.1), b:pressure_for(span*0.06)}})
end
crow_fly(252, 92, 17, 0.1, 1)
crow_fly(575, 64, 14, -0.15, -0.4)
crow_fly(628, 104, 11, 0.2, 0.7)
crow_fly(190, 196, 9, -0.1, -0.6)
crow_fly(700, 55, 6, 0, 1)
crow_fly(725, 72, 5, 0.1, -0.5)
-- a crow perched on the dead top of the leader, facing right
local body = poly({{412,56},{413,49},{415,45},{418,44},{420,46},{420.5,50},{419,55},{416,58},{414,58}}, true)
local head = ellipse(419.5, 42.3, 2.3, 2.1)
local tail = poly({{413,55},{410,61},{412,62},{415,57}})
local beak = poly({{421,41.5},{424.5,42.5},{421.3,43.4}})
work(body + head + tail + beak, {hand="detail", pile=crowP, coverage=4, fill=true, load=1, tool={kind="round", width=1, point=1}, pressure={0.5,0.8}})

--@ chunk 53
print(drying(560,160), drying(600,120), drying(900,150))

--@ chunk 54
twigD = pile{{"raw umber",2},{"bone black",0.55},{"smalt",0.6},{"lead white",0.22}, medium=0.12}
function tree_restroke(T, p, wmax)
  local bm = brush{kind="round", width=4, point=1}
  local bt = brush{kind="rigger", width=2, point=1}
  bm:load(p, 0.8); bt:load(p, 0.8)
  local n = 0
  for _,l in ipairs(T.limbs) do
    if l.ws[1] < wmax and #l.pts >= 2 then
      local b = (l.ws[1] > 1.6) and bm or bt
      if b:fullness() < 0.35 then b:load(p, 0.7) end
      local p0 = b:pressure_for(math.max(l.ws[1], 0.6))
      local p1 = b:pressure_for(math.max(l.ws[#l.ws], 0.6))
      b:stroke(l.pts, {pressure={p0, p1*0.6}, ramps={0.02, 0.25}, shake=0})
      n = n + 1
    end
  end
  return n
end
local n = 0
for _,T in ipairs({OAK, OAKX, OAKT, OAKXT, OAK2, OAK2T}) do n = n + tree_restroke(T, twigD, 3.5) end
print(n)

--@ chunk 55
twigK = pile{{"bone black",1.2},{"raw umber",0.8},{"smalt",0.3}, medium=0.1}
local sub = {limbs={}}
for i,l in ipairs(OAKT.limbs) do if l.pts[1][1] < 330 and l.pts[1][2] < 200 then sub.limbs[#sub.limbs+1] = l end end
print(#sub.limbs, tree_restroke(sub, twigK, 3.5))

--@ chunk 56
local n = 0
for _,T in ipairs({OAK, OAKX, OAKT, OAKXT, OAK2, OAK2T}) do n = n + tree_restroke(T, twigK, 3.5) end
print(n)

--@ chunk 57
print(wait(24*60)); print(drying(400,450), drying(850,420), drying(400,300), drying(640,450), drying(600,478), drying(300,200))

--@ chunk 58
local up1 = OAKthick2 * above(function(x) return 334 end)
work(up1, {hand="detail", pile=glzD, coverage=2, fill=true, tool={kind="filbert", width=5}, pressure={0.4,0.6}})
work(OAK2thick, {hand="detail", pile=glzD, coverage=2, fill=true, angle=math.pi/2, tool={kind="filbert", width=5}, pressure={0.4,0.6}})

--@ chunk 59
stepS = pile{{"lead white",2},{"smalt",1.6},{"raw umber",0.25},{"red earth",0.05}, medium=0.1}
stepL = pile{{"lead white",5},{"yellow ochre",0.12},{"vermilion",0.04},{"smalt",0.3}, medium=0.1}
-- path of the walker, near to far
local path = {{150,725},{215,665},{300,608},{385,560},{450,527},{497,502}}
-- resample by arc length with perspective spacing
local function interp(t)
  local n = #path - 1; local f = t*n; local i = math.min(math.floor(f)+1, n); local u = f - (i-1)
  local a, b = path[i], path[i+1]
  return lerp(a[1],b[1],u), lerp(a[2],b[2],u), math.atan(b[2]-a[2], b[1]-a[1])
end
local t, side = 0, 1
local bs = brush{kind="filbert", width=4}
local bl = brush{kind="round", width=1.5, point=1}
bs:load(stepS, 0.8); bl:load(stepL, 0.6)
local count = 0
while t < 1 do
  local x, y, a = interp(t)
  local s = clamp((y - 462)/(714 - 462), 0.05, 1.2)
  local nx, ny = -math.sin(a), math.cos(a)
  local px, py = x + side*nx*3.2*s, y + side*ny*3.2*s*0.5
  local L = 9*s; local Wd = 4.5*s
  local ca, sa = math.cos(a), math.sin(a)
  -- the print as a short dragged mark along the walking direction (foreshortened)
  if bs:fullness() < 0.3 then bs:load(stepS, 0.8) end
  local w = brush{kind="filbert", width=math.max(1.2, Wd)}
  w:load(stepS, 0.8)
  w:stroke({{px - ca*L*0.5, py - sa*L*0.25}, {px + ca*L*0.5, py + sa*L*0.25}}, {pressure={0.7,0.5}, orient="across"})
  -- lit back rim (far edge)
  if bl:fullness() < 0.3 then bl:load(stepL, 0.6) end
  bl:stroke({{px - ca*L*0.4 - Wd*0.3, py - sa*L*0.2 - Wd*0.35}, {px + ca*L*0.3 - Wd*0.3*0, py + sa*L*0.15 - Wd*0.45}}, {pressure={bl:pressure_for(math.max(0.6, 1.2*s)), 0.1}})
  side = -side
  t = t + 0.012 * (0.35 + s)
  count = count + 1
end
print(count)

--@ chunk 60
stepS2 = pile{{"lead white",1.2},{"smalt",1.8},{"raw umber",0.35},{"red earth",0.08}, medium=0.1}
local path = {{150,725},{215,665},{300,608},{385,560},{450,527},{497,502}}
local function interp(t)
  local n = #path - 1; local f = t*n; local i = math.min(math.floor(f)+1, n); local u = f - (i-1)
  local a, b = path[i], path[i+1]
  return lerp(a[1],b[1],u), lerp(a[2],b[2],u), math.atan(b[2]-a[2], b[1]-a[1])
end
local t, side = 0, 1
local bl = brush{kind="round", width=2, point=1}
bl:load(stepL, 0.6)
while t < 1 do
  local x, y, a = interp(t)
  local s = clamp((y - 462)/(714 - 462), 0.05, 1.2)
  local nx, ny = -math.sin(a), math.cos(a)
  local px, py = x + side*nx*3.2*s + rand(-0.8,0.8)*s, y + side*ny*3.2*s*0.5
  local L = 11*s*rand(0.85,1.15); local Wd = 6*s
  local ca, sa = math.cos(a), math.sin(a)
  local w = brush{kind="filbert", width=math.max(1.3, Wd)}
  w:load(stepS2, 0.9)
  w:stroke({{px - ca*L*0.5, py - sa*L*0.3}, {px + ca*L*0.5, py + sa*L*0.3}}, {pressure={0.85,0.6}, orient="across"})
  if bl:fullness() < 0.3 then bl:load(stepL, 0.6) end
  bl:stroke({{px - ca*L*0.45, py - sa*L*0.3 - Wd*0.55}, {px + ca*L*0.35, py + sa*L*0.2 - Wd*0.6}}, {pressure={bl:pressure_for(math.max(0.6, 1.6*s)), 0.1}})
  side = -side
  t = t + 0.012 * (0.35 + s)
end

--@ chunk 61
straw = pile{{"yellow ochre",1},{"lead white",0.8},{"raw umber",0.35}, medium=0.1}
strawD = pile{{"raw umber",1.5},{"bone black",0.2},{"yellow ochre",0.5}, medium=0.1}
function tuft(x, y, s, n, seed, lean)
  math.randomseed(seed)
  local b = brush{kind="rigger", width=math.max(1.2, 1.6*s), point=1}
  for i=1,n do
    local p = (math.random() < 0.55) and straw or strawD
    b:reload(p, 0.6)
    local bx = x + randn(0, 3*s)
    local h = rand(8, 22) * s
    local a = -math.pi/2 + randn(lean or 0.15, 0.35)
    local bend = randn(0.25, 0.2)
    local pts = {{bx, y + rand(-1,1)*s}}
    local cx, cy, ca = bx, pts[1][2], a
    for k=1,4 do
      ca = ca + bend*0.25
      cx = cx + math.cos(ca)*h/4; cy = cy + math.sin(ca)*h/4
      pts[#pts+1] = {cx, cy}
    end
    b:stroke(pts, {pressure={b:pressure_for(math.max(0.6, 0.9*s)), 0}, ramps={0.02, 0.5}})
    -- a seed head or a broken bent-over stem now and then
    if math.random() < 0.15 then
      b:stroke({{cx, cy}, {cx + 3*s, cy + 2*s}, {cx + 5*s, cy + 5*s}}, {pressure={0.3, 0}})
    end
  end
end
local list = {
 {40,640,1.1,14},{90,684,1.3,16},{230,702,1.4,12},{62,592,0.9,10},{270,642,1.0,9},{18,700,1.4,12},{140,620,1.0,7},
 {722,602,0.9,10},{762,632,1.0,12},{884,642,1.1,14},{944,692,1.3,16},{982,612,0.9,10},{652,694,1.3,10},{820,690,1.3,8},
 {300,542,0.55,8},{560,522,0.5,7},{782,522,0.5,8},{932,532,0.55,9},{122,547,0.55,9},{200,560,0.6,6},{640,540,0.5,6},
 {372,512,0.45,7},{444,514,0.45,6},{828,508,0.45,7},{878,506,0.45,6},{545,489,0.35,6},{742,487,0.35,6},{470,500,0.4,5}}
for i,t in ipairs(list) do tuft(t[1], t[2], t[3], t[4], 500+i, 0.2) end

--@ chunk 62
print(wait(8*60)); print(drying(300,608), drying(215,665), drying(640,450), drying(600,478))

--@ chunk 63
stepK = pile{{"lead white",0.7},{"smalt",1.7},{"raw umber",0.5},{"bone black",0.08}, medium=0.1}
local path = {{150,725},{215,665},{300,608},{385,560},{450,527},{497,502}}
local function interp(t)
  local n = #path - 1; local f = t*n; local i = math.min(math.floor(f)+1, n); local u = f - (i-1)
  local a, b = path[i], path[i+1]
  return lerp(a[1],b[1],u), lerp(a[2],b[2],u), math.atan(b[2]-a[2], b[1]-a[1])
end
local t, side = 0, 1
local bk = brush{kind="round", width=3, point=0.7}
local bl = brush{kind="round", width=2, point=1}
bk:load(stepK, 0.8); bl:load(litS, 0.6)
while t < 1 do
  local x, y, a = interp(t)
  local s = clamp((y - 462)/(714 - 462), 0.05, 1.2)
  local nx, ny = -math.sin(a), math.cos(a)
  local px, py = x + side*nx*3.2*s, y + side*ny*3.2*s*0.5
  local L = 11*s; local Wd = 6*s
  local ca, sa = math.cos(a), math.sin(a)
  if bk:fullness() < 0.3 then bk:load(stepK, 0.8) end
  -- dark far wall: a crescent along the upper edge
  bk:stroke({{px - ca*L*0.5, py - sa*L*0.3 - Wd*0.1}, {px, py - Wd*0.38}, {px + ca*L*0.5, py + sa*L*0.3 - Wd*0.15}}, {pressure={bk:pressure_for(math.max(0.6, Wd*0.35)), bk:pressure_for(math.max(0.6, Wd*0.2))}, ramps={0.2,0.3}})
  -- light near lip, below
  if bl:fullness() < 0.3 then bl:load(litS, 0.6) end
  bl:stroke({{px - ca*L*0.45, py - sa*L*0.3 + Wd*0.55}, {px + ca*L*0.4, py + sa*L*0.25 + Wd*0.5}}, {pressure={bl:pressure_for(math.max(0.6, 1.3*s)), 0.1}})
  side = -side
  t = t + 0.012 * (0.35 + s)
end

--@ chunk 64
print(wait(20*60)); print(drying(300,608), drying(215,665), drying(640,450), drying(600,478), drying(90,684))

--@ chunk 65
trailP = pile{{"lead white",2.2},{"smalt",1.5},{"raw umber",0.3},{"red earth",0.08}, medium=0.1}
local path = {{150,725},{215,665},{300,608},{385,560},{450,527},{497,502}}
local ws = {}
for i,p in ipairs(path) do local s = clamp((p[2]-462)/252, 0.05, 1.2); ws[i] = 20*s + 1.5 end
TRAIL = ribbon(path, ws):roughen(1.5, 10, 8)
work(TRAIL, {hand="detail", pile=trailP, coverage=2.5, fill=true, angle=function(x,y) return -0.6 end, tool={kind="filbert", width=4}, pressure={0.4,0.7}})
-- pocks: darker holes, irregular, along the trail
local bk = brush{kind="round", width=3, point=0.6}
bk:load(stepK, 0.8)
local n = #path - 1
for k=1,120 do
  local t = rand(0,1)
  local f = t*n; local i = math.min(math.floor(f)+1, n); local u = f-(i-1)
  local a, b = path[i], path[i+1]
  local x, y = lerp(a[1],b[1],u), lerp(a[2],b[2],u)
  local s = clamp((y-462)/252, 0.05, 1.2)
  x = x + randn(0, 3*s); y = y + randn(0, 1.5*s)
  if bk:fullness() < 0.3 then bk:load(stepK, 0.8) end
  bk:stroke({{x - 3*s, y}, {x, y - 1*s}, {x + 3*s, y + 0.3*s}}, {pressure={bk:pressure_for(math.max(0.6, 2.2*s)), bk:pressure_for(math.max(0.6,1.2*s))}, ramps={0.3,0.3}})
end
-- lit lips of snow thrown up along the trail edges
local bl = brush{kind="round", width=2.5, point=0.8}
bl:load(litS, 0.6)
for k=1,70 do
  local t = rand(0,1)
  local f = t*n; local i = math.min(math.floor(f)+1, n); local u = f-(i-1)
  local a, b = path[i], path[i+1]
  local x, y = lerp(a[1],b[1],u), lerp(a[2],b[2],u)
  local s = clamp((y-462)/252, 0.05, 1.2)
  local side = (math.random()<0.5) and -1 or 1
  x = x + side*10*s*rand(0.8,1.1); y = y + side*3*s + 5*s
  if bl:fullness() < 0.3 then bl:load(litS, 0.6) end
  bl:stroke({{x - 4*s, y}, {x + 4*s, y - 2.5*s}}, {pressure={bl:pressure_for(math.max(0.6, 1.5*s)), 0.1}})
end

--@ chunk 66
print(wait(3*24*60)); print(drying(300,608), drying(640,450), drying(600,478), drying(400,450))

--@ chunk 67
capShade = pile{{"lead white",4},{"smalt",0.8},{"raw umber",0.1},{"vermilion",0.03}, medium=0.08}
snowTop = pile{{"lead white",5},{"yellow ochre",0.1},{"vermilion",0.04},{"smalt",0.15}, medium=0.08}
local top = {{543,454},{549,446},{562,439},{582,434},{604,430},{628,428},{652,428},{676,429},{698,432},{716,436},{729,442},{737,450},{739,455}}
local bot = {{739,455},{731,452},{722,454},{711,451},{699,455},{687,452},{676,455},{663,452},{650,456},{637,452},{624,455},{611,452},{598,456},{585,452},{572,455},{560,452},{550,456},{543,454}}
local pts = {}
for _,p in ipairs(top) do pts[#pts+1] = p end
for _,p in ipairs(bot) do pts[#pts+1] = p end
SNOWCAP = poly(pts, true)
work(SNOWCAP, {hand="detail", pile=capShade, coverage=3.5, fill=true, angle=0, load=1, pressure={0.6,0.9}})
local ws = {}; for i=1,#top do ws[i] = 5 end
local rim = ribbon(top, ws) * SNOWCAP:grow(1)
work(rim, {hand="detail", pile=snowTop, coverage=3, fill=true, angle=0, load=0.9, pressure={0.5,0.8}})

--@ chunk 68
SLAB = outline{{736,492,"c"},{741,472},{749,459,"c"},{762,457,"c"},{774,468},{784,493,"c"}, closed=true, char="broken", seed=123}
work(SLAB:mask(), {hand="detail", pile=rkA, coverage=3, fill=true, angle=1.2, load=1, pressure={0.6,0.9}})
stipple(SLAB:mask(), {pile=rkB, width=3, coverage=0.5, pressure={0.4,0.7}})
local stones = {{519,489,8,5,1},{537,492,6,4,2},{801,495,9,5.5,3},{821,498,6,4,4},{612,501,10,4.5,5},{683,503,7,4,6},{760,500,6,3.5,7},{498,486,5,3,8}}
KERB = {}
local km
for i,s in ipairs(stones) do
  local x, y, rx, ry = s[1], s[2], s[3], s[4]
  local o = outline{{x-rx,y+ry*0.6,"c"},{x-rx*0.8,y-ry*0.4},{x-rx*0.2,y-ry},{x+rx*0.5,y-ry*0.8},{x+rx,y-ry*0.1},{x+rx*0.9,y+ry*0.6,"c"}, closed=true, char="broken", seed=300+i}
  KERB[i] = {x=x, y=y, rx=rx, ry=ry, m=o:mask()}
  km = km and (km + KERB[i].m) or KERB[i].m
end
KERBM = km
work(km, {hand="detail", pile=rkA, coverage=3, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.8, point=0.6}})

--@ chunk 69
print(wait(28*60)); print(drying(640,440), drying(760,475), drying(300,608), drying(612,500))

--@ chunk 70
trailK = pile{{"lead white",1.2},{"smalt",1.6},{"raw umber",0.45},{"red earth",0.1}, medium=0.1}
local path = {{150,725},{215,665},{300,608},{385,560},{450,527},{497,502}}
local sh, ws = {}, {}
for i,p in ipairs(path) do local s = clamp((p[2]-462)/252, 0.05, 1.2); sh[i] = {p[1] - 3*s, p[2] - 2.5*s}; ws[i] = 9*s + 1 end
local m = ribbon(sh, ws):roughen(1.2, 7, 21) * TRAIL:grow(1)
work(m, {hand="detail", pile=trailK, coverage=2.5, fill=true, angle=-0.6, tool={kind="filbert", width=3}, pressure={0.4,0.7}})
local bk = brush{kind="round", width=4, point=0.6}
bk:load(stepK, 0.9)
local n = #path - 1
for k=1,90 do
  local t = (k - rand(0,1))/90
  local f = t*n; local i = math.min(math.floor(f)+1, n); local u = f-(i-1)
  local a, b = path[i], path[i+1]
  local x, y = lerp(a[1],b[1],u), lerp(a[2],b[2],u)
  local s = clamp((y-462)/252, 0.05, 1.2)
  local side = (k % 2 == 0) and 1 or -1
  x = x + side*3*s + randn(0, 1.2*s); y = y + side*1.2*s
  if bk:fullness() < 0.3 then bk:load(stepK, 0.9) end
  bk:stroke({{x - 4*s, y + 0.5*s}, {x, y - 1.5*s}, {x + 4*s, y}}, {pressure={bk:pressure_for(math.max(0.6, 3*s)), bk:pressure_for(math.max(0.6,1.5*s))}, ramps={0.3,0.3}})
end

--@ chunk 71
coatP = pile{{"bone black",0.7},{"raw umber",1},{"green earth",0.6},{"smalt",0.2},{"lead white",0.25}, medium=0.05}
capP = pile{{"bone black",1},{"raw umber",0.6},{"red earth",0.15}, medium=0.05}
local X, F = 502, 499   -- center, feet
local coat = poly({{X-4.6,F-31},{X-2,F-32.3},{X+2.2,F-32.3},{X+5,F-30.8},{X+6.2,F-27},{X+7.8,F-21.5},{X+8.3,F-19.5},{X+7,F-18.5},{X+7.2,F-10},{X+8,F-3.5},{X-7.5,F-3},{X-6.4,F-12},{X-5.8,F-22},{X-5.6,F-27.5}}, true)
local neck = rect(X-1.8, F-34.5, 3.6, 3)
local head = ellipse(X+0.3, F-36.2, 2.5, 3.0)
local cap = ellipse(X+0.1, F-38.3, 3.2, 1.7) + ellipse(X+1.2, F-38.9, 2.2, 1.4)
local legs = rect(X-4.2, F-3.5, 2.6, 3.5) + rect(X+1.8, F-3.5, 2.6, 3.8)
work(coat + neck + legs, {hand="detail", pile=coatP, coverage=4, fill=true, angle=math.pi/2, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.2, point=0.8}})
work(head, {hand="detail", pile=capP, coverage=4, fill=true, load=1, pressure={0.5,0.8}, tool={kind="round", width=1, point=0.8}})
work(cap, {hand="detail", pile=capP, coverage=4, fill=true, load=1, pressure={0.5,0.8}, tool={kind="round", width=1, point=0.8}})
-- staff
local st = brush{kind="round", width=1.4, point=1}
st:load(capP, 0.8)
st:stroke({{X+8.5, F-20}, {X+10.2, F-10}, {X+11.8, F+0.5}}, {pressure={st:pressure_for(0.9), st:pressure_for(0.8)}})
st:stroke({{X+8.2, F-21.5}, {X+8.4, F-23.5}}, {pressure={st:pressure_for(0.9), st:pressure_for(0.8)}})

--@ chunk 72
print(wait(2*24*60)); print(drying(640,440), drying(760,475), drying(612,500), drying(502,480))

--@ chunk 73
print(wait(3*24*60)); print(drying(640,440), drying(760,475), drying(612,500), drying(502,480))

--@ chunk 74
local rockbits = poly({{542,455},{546,447},{553,442},{559,444},{562,451},{558,458},{548,459}}, true)
  + poly({{709,441},{719,438},{729,441},{737,448},{740,456},{729,458},{716,455},{710,448}}, true)
  + poly({{586,457},{590,449},{598,446},{606,448},{611,456}}, true)
  + poly({{655,458},{659,450},{668,447},{678,449},{683,457}}, true)
  + poly({{690,456},{694,452},{700,452},{703,456}}, true)
work(rockbits, {hand="detail", pile=rkA, coverage=3.5, fill=true, angle=0.2, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.8, point=0.6}})
-- cool skylight on the upper faces of the exposed rock
local lit = (rockbits * above(function(x) return 447 + 0.03*(x-640) end))
work(lit, {hand="detail", pile=rkC, coverage=1.5, fill=false, angle=0.1, pressure={0.3,0.5}, tool={kind="round", width=1.5, point=0.6}})
-- extra snow: a heavier mound on the left-center top, and a long drip
local mound = poly({{565,440},{578,431},{596,425},{618,423},{640,425},{652,429},{640,432},{610,433},{585,437}}, true)
work(mound, {hand="detail", pile=snowTop, coverage=3, fill=true, angle=0, load=1, pressure={0.5,0.8}, tool={kind="round", width=2, point=0.6}})
local drip = poly({{619,452},{624,452},{630,453},{636,452},{633,458},{629,461},{625,459}}, true) + poly({{566,452},{572,452},{570,457},{567,456}}, true)
work(drip, {hand="detail", pile=capShade, coverage=3, fill=true, angle=math.pi/2, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.3, point=0.8}})

--@ chunk 75
local function sl(x) return 452.5 + 1.8*math.sin(x/9.7) + 1.1*math.sin(x/4.1 + 1) end
local mid = mask(function(x,y) if x < 568 or x > 712 then return 0 end; if y > 440 and y < sl(x) then return 1 end; return 0 end)
work(mid, {hand="detail", pile=capShade, coverage=3.5, fill=true, angle=0, load=1, pressure={0.5,0.85}, tool={kind="round", width=1.8, point=0.6}})
-- right end: snow over the top of the shoulder, following its curve
local rtop = poly({{706,440},{718,437},{728,440},{735,446},{738,451},{732,449},{724,445},{714,444},{706,446}}, true)
local ltop = poly({{548,447},{553,442},{560,441},{566,444},{566,448},{559,446},{553,447}}, true)
work(rtop + ltop, {hand="detail", pile=capShade, coverage=3.5, fill=true, angle=0.2, load=1, pressure={0.5,0.85}, tool={kind="round", width=1.5, point=0.6}})
-- flecks of snow clinging to the rough rock face below the line
local fb = brush{kind="round", width=1.6, point=0.5}
fb:load(capShade, 0.6)
for i=1,40 do
  local x = rand(552, 735); local y = sl(x) + rand(1, 12)
  if CAPM:at(x, y) > 0.5 then
    if fb:fullness() < 0.3 then fb:load(capShade, 0.6) end
    fb:stroke({{x, y}, {x + rand(1,3), y + rand(-0.3,0.3)}}, {pressure={rand(0.2,0.4), 0.1}})
  end
end

--@ chunk 76
print(wait(3*24*60)); print(drying(600,450), drying(720,445), drying(502,480))

--@ chunk 77
print(wait(3*24*60)); print(drying(600,450), drying(720,445), drying(560,445))

--@ chunk 78
print(wait(3*24*60)); print(drying(600,450), drying(720,445), drying(560,445), drying(640,450))

--@ chunk 79
CAPTOP = {{543,454},{550,446},{560,440},{585,431},{612,426},{640,425},{668,427},{695,431},{716,436},{731,443},{739,451},{741,456}}
function captop(x)
  for i=1,#CAPTOP-1 do local a,b = CAPTOP[i], CAPTOP[i+1]; if x>=a[1] and x<=b[1] then return lerp(a[2],b[2],(x-a[1])/(b[1]-a[1])) end end
  return nil
end
function capbot(x)
  local t = captop(x); if not t then return nil end
  local b = 451.5 + 1.6*math.sin(x/9.7) + 1.0*math.sin(x/4.1 + 1)
  -- the snow thins toward the rounded ends
  local e = math.min(x - 543, 741 - x)
  local th = clamp(e/22, 0.25, 1) * (b - t)
  return t + th
end
SNOWM = mask(function(x,y) local t = captop(x); if not t then return 0 end; local b = capbot(x); if y >= t and y <= b then return 1 end; return 0 end)
work(SNOWM, {hand="detail", pile=capShade, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.5}})
local LIT = mask(function(x,y) local t = captop(x); if not t then return 0 end; if y >= t and y <= t + 5 + 1.5*math.sin(x/13) then return 1 end; return 0 end)
work(LIT, {hand="detail", pile=snowTop, coverage=3.5, fill=true, angle=0, load=1, pressure={0.5,0.85}, tool={kind="round", width=1.8, point=0.5}})

--@ chunk 80
local sm
for i,k in ipairs(KERB) do
  local cut = k.y - k.ry*0.15 + rand(-0.5, 0.5)
  local m = (k.m:grow(0.6) * above(function(x) return cut + 0.8*math.sin(x*1.3 + i) end))
  sm = sm and (sm + m) or m
end
work(sm, {hand="detail", pile=capShade, coverage=4, fill=true, angle=0, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.2, point=0.6}})
-- the top edges catch the sky
local tb = brush{kind="round", width=1.4, point=0.8}
tb:load(snowTop, 0.7)
for i,k in ipairs(KERB) do
  if tb:fullness() < 0.3 then tb:load(snowTop, 0.7) end
  tb:stroke({{k.x - k.rx*0.8, k.y - k.ry*0.35}, {k.x - k.rx*0.2, k.y - k.ry*0.95}, {k.x + k.rx*0.5, k.y - k.ry*0.8}, {k.x + k.rx*0.9, k.y - k.ry*0.1}}, {pressure={tb:pressure_for(0.9), tb:pressure_for(0.6)}})
end
-- the leaning slab: snow along its upper edges
local slabS = poly({{740,474},{744,466},{749,459},{762,457},{770,462},{768,465},{760,462},{751,463},{746,469},{743,476}}, true)
work(slabS, {hand="detail", pile=capShade, coverage=4, fill=true, angle=0.9, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.4, point=0.6}})
tb:reload(snowTop, 0.7)
tb:stroke({{741,472},{745,464},{750,459},{761,457},{768,461}}, {pressure={tb:pressure_for(1.2), tb:pressure_for(0.7)}})
-- a lighter facet on the slab face, a darker right side
local face = SLAB:mask() * mask(function(x,y) return (x - 742) < (y - 455)*0.9 + 10 and 1 or 0 end)
work(face, {hand="detail", pile=rkC, coverage=1.5, fill=false, angle=1.2, pressure={0.3,0.5}, tool={kind="round", width=1.5, point=0.6}})

--@ chunk 81
print(wait(3*24*60)); print(drying(600,440), drying(760,470), drying(612,498))

--@ chunk 82
ROCKBAND = (CAPM:grow(1.5) + UPM) * mask(function(x,y) local b = capbot(x); if not b then return 0 end; return (y > b + 0.4) and 1 or 0 end)
work(ROCKBAND, {hand="detail", pile=rkA, coverage=3.5, fill=true, angle=0.1, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.5}})
local n = noise{seed=9, period=14, octaves=3}
stipple(ROCKBAND, {pile=rkB, width=2.5, coverage=function(x,y) return 0.7*clamp(n(x,y)+0.1,0,1) end, pressure={0.35,0.6}})
-- slab: solid dark, then one lighter upper-left facet
local slabS = poly({{740,474},{744,466},{749,459},{762,457},{770,462},{768,465},{760,462},{751,463},{746,469},{743,476}}, true)
local sb = SLAB:mask() - slabS:grow(0.5)
work(sb, {hand="detail", pile=rkA, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.5}})
local facet = sb * poly({{744,478},{750,466},{760,464},{768,468},{765,480},{754,490},{745,490}})
work(facet, {hand="detail", pile=rkB, coverage=2.5, fill=true, angle=0.3, load=0.9, pressure={0.5,0.8}, tool={kind="round", width=2, point=0.5}})

--@ chunk 83
print(wait(4*24*60)); print(drying(595,478), drying(640,478), drying(686,478), drying(760,480), drying(640,445))

--@ chunk 84
-- openings under the capstone: the misty plain and the knoll's snow seen through
local o1 = outline{{588,472,"c"},{597,470},{604,473,"c"},{606,482},{605,489,"c"},{589,489,"c"},{587,481}, closed=true, char="broken", seed=401, amount=0.6}
local o2 = outline{{631,473,"c"},{641,471},{650,473,"c"},{651,481},{650,488,"c"},{632,488,"c"},{630,481}, closed=true, char="broken", seed=402, amount=0.6}
local o3 = outline{{680,472,"c"},{688,470},{694,472,"c"},{695,480},{694,488,"c"},{681,488,"c"},{679,480}, closed=true, char="broken", seed=403, amount=0.6}
OPEN = o1:mask() + o2:mask() + o3:mask()
local far = OPEN * above(function(x) return 479 end)
local near = OPEN * below(function(x) return 479 end)
work(far, {hand="detail", pile=dist2, coverage=4, fill=true, angle=0, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.5, point=0.5}})
work(near, {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.5, point=0.5}})

--@ chunk 85
-- widen the left gap into a leaning wedge
local w1 = poly({{586,474},{588,472},{588,489},{577,490},{582,482}}, true)
work(w1 * above(function(x) return 479 end), {hand="detail", pile=dist2, coverage=4, fill=true, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.3, point=0.5}})
work(w1 * below(function(x) return 479 end), {hand="detail", pile=crest, coverage=4, fill=true, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.3, point=0.5}})
-- narrow the right opening: rock back over its right half, leaning
local r3 = poly({{689,471},{695,471},{696,489},{684,489},{687,480}}, true)
work(r3, {hand="detail", pile=rkA, coverage=4, fill=true, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.3, point=0.5}})
-- a thin sliver of light under the capstone at the right
local sl = poly({{703,467.5},{716,466.5},{717,468.5},{704,469.5}}, true)
work(sl, {hand="detail", pile=dist1, coverage=4, fill=true, load=1, pressure={0.4,0.7}, tool={kind="round", width=1, point=0.8}})
-- a fallen stone inside the middle opening, half in snow
local fs = poly({{634,489},{636,483},{641,481},{646,483},{648,489}}, true)
work(fs, {hand="detail", pile=rkA, coverage=4, fill=true, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.2, point=0.5}})

--@ chunk 86
local drift = poly({{352,516},{368,511},{380,508},{392,509},{403,507.5},{414,508.5},{426,507},{438,509},{452,512},{468,517},{472,545},{348,545}}, true)
work(drift, {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=2.5, point=0.4}})
blend(drift:grow(4) - drift:shrink(4), {angle=0})
-- the right oak
local drift2 = poly({{820,508},{832,505},{842,506},{850,505},{860,506},{870,504},{882,507},{886,520},{818,520}}, true)
work(drift2, {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.4}})

--@ chunk 87
local rootL = poly({{388,494},{385,503},{378,507},{368,511},{372,512.5},{384,510},{392,509}}, true)
local rootR = poly({{420,494},{423,503},{431,506},{444,510},{440,511.5},{428,509.5},{418,509}}, true)
local rootM = poly({{399,508},{404,505},{410,508},{405,510}}, true)
work(rootL + rootR + rootM, {hand="detail", pile=barkK, coverage=4, fill=true, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.6, point=0.6}})
local rootL2 = poly({{842,500},{838,505},{830,508},{836,509},{844,507}}, true)
local rootR2 = poly({{857,500},{861,505},{870,507},{864,508.5},{856,507}}, true)
work(rootL2 + rootR2, {hand="detail", pile=barkK, coverage=4, fill=true, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.4, point=0.6}})

--@ chunk 88
print(wait(4*24*60)); print(drying(400,515), drying(380,508), drying(850,508), drying(640,485), drying(595,480))

--@ chunk 89
print(wait(4*24*60)); print(drying(400,515), drying(380,508), drying(850,508), drying(640,485), drying(595,480))

--@ chunk 90
-- roots, darker and fuller
local rootL = poly({{387,490},{386,500},{381,505},{372,509},{364,512},{370,513},{381,511},{390,509.5},{394,500}}, true)
local rootR = poly({{419,490},{421,500},{427,504},{437,507.5},{447,511},{441,512},{430,510},{420,509},{416,500}}, true)
local rootM = poly({{397,509.5},{401,505.5},{408,505},{412,509.5}}, true)
work(rootL + rootR + rootM, {hand="detail", pile=barkK, coverage=4, fill=true, load=1, pressure={0.7,0.95}, tool={kind="round", width=1.6, point=0.5}})
local rootL2 = poly({{842,498},{840,504},{833,507.5},{828,509},{836,509.5},{845,507}}, true)
local rootR2 = poly({{856,498},{859,504},{866,506.5},{872,508},{864,509},{855,507}}, true)
work(rootL2 + rootR2, {hand="detail", pile=barkK, coverage=4, fill=true, load=1, pressure={0.7,0.95}, tool={kind="round", width=1.4, point=0.5}})
-- figure: restate the coat's left edge over the stone's snow
local X, F = 502, 499
local coatL = poly({{X-5.8,F-22},{X-6.4,F-12},{X-7.5,F-3},{X-3,F-3},{X-3,F-22}}, true)
work(coatL, {hand="detail", pile=coatP, coverage=4, fill=true, angle=math.pi/2, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.1, point=0.8}})
-- trail restored through the drift by the oak
local path = {{385,560},{450,527},{497,502}}
local sh, ws = {}, {}
for i,p in ipairs(path) do local s = clamp((p[2]-462)/252, 0.05, 1.2); sh[i] = {p[1] - 3*s, p[2] - 2.5*s}; ws[i] = 9*s + 1 end
local m = ribbon(sh, ws):roughen(1.2, 7, 22) * rect(420, 505, 60, 40)
work(m, {hand="detail", pile=trailK, coverage=2.5, fill=true, angle=-0.5, tool={kind="filbert", width=2.5}, pressure={0.4,0.7}})

--@ chunk 91
-- snow drifted against the foot of the dolmen
local prof = {{546,492},{552,488},{560,486.5},{572,487.5},{584,486},{596,488.5},{607,487},{618,488},{630,486.5},{643,488.5},{655,486},{668,488},{680,487},{692,488.5},{704,486.5},{716,487.5},{728,486},{738,489},{748,490},{760,489},{772,490.5},{786,492.5},{790,497}}
local pts = {}
for _,p in ipairs(prof) do pts[#pts+1] = p end
pts[#pts+1] = {790,500}; pts[#pts+1] = {546,500}
BASED = poly(pts, true)
work(BASED, {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.4}})
-- the lit top line of the drift
local tb = brush{kind="round", width=1.6, point=0.8}
tb:load(snowTop, 0.7)
tb:stroke(prof, {pressure={tb:pressure_for(1.0), tb:pressure_for(0.8)}, shake=0.4})

--@ chunk 92
BOULDER = outline{{768,676,"c"},{772,655},{785,636},{808,624},{835,620},{862,626},{884,640},{897,660},{900,677,"c"}, closed=true, char="broken", seed=611, amount=0.7}
BOULDER2 = outline{{905,680,"c"},{909,668},{920,661},{934,662},{944,671},{946,682,"c"}, closed=true, char="broken", seed=612, amount=0.7}
local bm = BOULDER:mask() + BOULDER2:mask()
work(bm, {hand="detail", pile=rkA, coverage=3.5, fill=true, angle=0.3, load=1, pressure={0.6,0.9}, tool={kind="round", width=3, point=0.4}})
local n = noise{seed=13, period=20, octaves=3}
stipple(bm, {pile=rkB, width=4, coverage=function(x,y) return 0.6*clamp(n(x,y)+0.2,0,1) end, pressure={0.4,0.7}})
-- stump, lower left
STUMP = poly({{48,652},{52,640},{53,626},{55,618},{60,621},{64,614},{69,619},{74,612},{78,618},{84,616},{86,626},{88,640},{94,652},{101,656},{84,658},{62,658},{42,657}}, true)
work(STUMP, {hand="detail", pile=barkK, coverage=4, fill=true, angle=math.pi/2, load=1, pressure={0.6,0.9}, tool={kind="round", width=2.5, point=0.4}})

--@ chunk 93
print(wait(4*24*60)); print(drying(800,650), drying(70,640), drying(600,490), drying(450,525), drying(400,530))

--@ chunk 94
snowG = pile{{"lead white",4.5},{"smalt",0.9},{"vermilion",0.06},{"raw umber",0.12},{"yellow ochre",0.05}, medium=0.1}
local ov = poly({{340,520},{360,514},{380,512},{430,511},{465,514},{480,522},{482,548},{340,550}}, true)
work(ov, {hand="body", pile=snowG, angle=-0.03, coverage=2.5, fill=true, length={20,50}, edge="soft", tool={kind="filbert", width=6}})
blend(ov:grow(3), {angle=0, clip=true})
-- trail through here: near-side light band and the dark far wall
local path = {{385,560},{450,527},{497,502}}
local c, wsP, sh, wsK = {}, {}, {}, {}
for i,p in ipairs(path) do local s = clamp((p[2]-462)/252, 0.05, 1.2); c[i] = p; wsP[i] = 20*s + 1.5; sh[i] = {p[1]-3*s, p[2]-2.5*s}; wsK[i] = 9*s + 1 end
local zone = rect(400, 500, 90, 60)
work(ribbon(c, wsP):roughen(1.2, 8, 31) * zone, {hand="detail", pile=trailP, coverage=2.5, fill=true, angle=-0.5, tool={kind="filbert", width=3}, pressure={0.4,0.7}})

--@ chunk 95
print(wait(5*24*60)); print(drying(800,650), drying(70,640), drying(600,490), drying(450,525), drying(400,530))

--@ chunk 96
local bcap = BOULDER:mask():grow(1) * above(function(x) return 641 + 7*math.sin((x-770)/130*math.pi)*0 + 3*math.sin(x/11) + math.abs(x-835)*0.12 end)
work(bcap, {hand="detail", pile=capShade, coverage=4, fill=true, angle=0.1, load=1, pressure={0.6,0.9}, tool={kind="round", width=2.5, point=0.4}})
local blit = BOULDER:mask():grow(1) * above(function(x) return 630 + math.abs(x-835)*0.1 + 1.5*math.sin(x/9) end)
work(blit, {hand="detail", pile=snowTop, coverage=3.5, fill=true, angle=0.1, load=1, pressure={0.5,0.85}, tool={kind="round", width=2.2, point=0.4}})
local b2cap = BOULDER2:mask():grow(0.8) * above(function(x) return 669 + 1.2*math.sin(x/5) end)
work(b2cap, {hand="detail", pile=capShade, coverage=4, fill=true, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.6, point=0.5}})
-- drifts at the boulders' feet
local bd = poly({{760,684},{775,672},{790,675},{805,671},{822,676},{840,672},{858,677},{875,672},{893,676},{905,678},{920,676},{935,679},{952,681},{962,690},{755,692}}, true)
work(bd, {hand="detail", pile=snowG, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=2.5, point=0.4}})
-- stump: snow on the broken top, and around the foot
local stop = poly({{53,628},{55,619},{60,622},{64,616},{69,620},{74,614},{78,619},{84,618},{86,627},{80,624},{72,626},{64,625},{58,627}}, true)
work(stop, {hand="detail", pile=capShade, coverage=4, fill=true, load=1, pressure={0.5,0.85}, tool={kind="round", width=1.5, point=0.5}})
local sd = poly({{36,660},{45,654},{56,652},{70,655},{84,653},{97,655},{108,660},{100,664},{40,664}}, true)
work(sd, {hand="detail", pile=snowG, coverage=4, fill=true, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.4}})
-- roots: a thin lip of snow on each
local tb = brush{kind="round", width=1.4, point=0.8}
tb:load(snowTop, 0.7)
tb:stroke({{366,511},{374,508},{382,505.5},{386,501}}, {pressure={tb:pressure_for(1.0), tb:pressure_for(0.6)}})
tb:stroke({{421,501},{427,504},{436,506.5},{445,510}}, {pressure={tb:pressure_for(0.6), tb:pressure_for(1.0)}})
tb:stroke({{830,508},{836,506},{840,503}}, {pressure={tb:pressure_for(0.9), tb:pressure_for(0.6)}})
tb:stroke({{859,503},{865,505.5},{870,507.5}}, {pressure={tb:pressure_for(0.6), tb:pressure_for(0.9)}})
-- trail's dark far wall, restated near the oak
local path = {{385,560},{450,527},{497,502}}
local sh, wsK = {}, {}
for i,p in ipairs(path) do local s = clamp((p[2]-462)/252, 0.05, 1.2); sh[i] = {p[1]-3*s, p[2]-2.5*s}; wsK[i] = 9*s + 1 end
work(ribbon(sh, wsK):roughen(1.2, 7, 33) * rect(400,500,90,60), {hand="detail", pile=trailK, coverage=2.5, fill=true, angle=-0.5, tool={kind="filbert", width=2.5}, pressure={0.4,0.7}})

--@ chunk 97
limbSnow = pile{{"lead white",5},{"smalt",0.35},{"yellow ochre",0.06},{"vermilion",0.03}, medium=0.05}
function limb_snow(T, wmin, seed)
  math.randomseed(seed)
  local b = brush{kind="round", width=2.4, point=1}
  b:load(limbSnow, 0.7)
  local n = 0
  for _,l in ipairs(T.limbs) do
    if l.ws[1] >= wmin and #l.pts >= 2 and not l.dead then
      local run = {}
      local function flush()
        if #run >= 2 then
          if b:fullness() < 0.3 then b:load(limbSnow, 0.7) end
          local wa = run.w or 1
          b:stroke(run, {pressure={b:pressure_for(math.max(0.6, wa)), b:pressure_for(math.max(0.6, wa*0.7))}, ramps={0.25, 0.35}, shake=0.2})
          n = n + 1
        end
        run = {}
      end
      for i=2,#l.pts do
        local a, c = l.pts[i-1], l.pts[i]
        local dx, dy = c[1]-a[1], c[2]-a[2]
        local len = math.sqrt(dx*dx+dy*dy)
        local slope = math.abs(dy)/math.max(len, 0.01)
        local w = l.ws[i]
        if slope < 0.55 and math.random() < 0.75 and w > 1.2 then
          -- normal pointing up
          local nx, ny = -dy/len, dx/len
          if ny > 0 then nx, ny = -nx, -ny end
          local off = l.ws[i-1]*0.5 - 0.25
          if #run == 0 then run[1] = {a[1] + nx*off, a[2] + ny*off} end
          run[#run+1] = {c[1] + nx*(w*0.5-0.25), c[2] + ny*(w*0.5-0.25)}
          run.w = math.min(2.0, 0.3*w + 0.4) * (1 - 0.6*slope)
        else
          flush()
        end
      end
      flush()
    end
  end
  return n
end
print(limb_snow(OAK, 2.0, 1), limb_snow(OAKX, 2.0, 2), limb_snow(OAK2, 2.0, 3))

--@ chunk 98
print(wait(5*24*60)); print(drying(800,650), drying(830,680), drying(70,655), drying(450,525), drying(300,300))

--@ chunk 99
print(wait(3*24*60)); print(drying(70,655), drying(70,622))

--@ chunk 100
fgGlz = pile{{"smalt",2},{"raw umber",0.3},{"red earth",0.12}, medium=0.75}
local m1 = below(function(x) return 600 + 12*math.sin(x/140) end)
work(m1, {hand="glaze", pile=fgGlz, angle=0.0, coverage=1.2, angle_jitter=0.05})
blend(m1:grow(-2), {angle=0})

--@ chunk 101
local band = below(function(x) return 578 + 12*math.sin(x/140) end) * above(function(x) return 625 + 12*math.sin(x/140) end)
blend(band, {angle=0, clip=false})
blend(band, {angle=0.1, clip=false})

--@ chunk 102
print(wait(5*24*60)); print(drying(300,650), drying(100,700), drying(900,690), drying(500,600))

--@ chunk 103
woodP = pile{{"lead white",2},{"yellow ochre",0.4},{"raw umber",0.5},{"smalt",0.15}, medium=0.05}
-- new broken top: dark bark with a tall splinter at left
local top = poly({{52,634},{53,622},{55,611},{57.5,603},{60,611},{62,620},{66,617},{70,621},{75,616.5},{80,620},{86,623},{88,634}}, true)
work(top, {hand="detail", pile=barkK, coverage=4, fill=true, angle=math.pi/2, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.6}})
-- pale broken wood on the right face of the break and inside the splinter
local wood = poly({{71,622},{75,618.5},{80,621.5},{85,624.5},{84,627},{78,625},{73,625}}, true) + poly({{57.5,606},{59.3,611},{60.5,619},{58.8,619}}, true)
work(wood, {hand="detail", pile=woodP, coverage=4, fill=true, angle=math.pi/2, load=1, pressure={0.5,0.8}, tool={kind="round", width=1, point=0.8}})
-- bark fissures on the stump
local f = brush{kind="round", width=1.4, point=1}
f:load(barkL, 0.4)
for i=1,10 do local x = rand(55, 85); f:stroke({{x, rand(628,634)}, {x + rand(-1,1), rand(640,650)}}, {pressure={0.3,0.1}}) end

--@ chunk 104
fgMid = pile{{"lead white",2.5},{"smalt",1.4},{"raw umber",0.28},{"red earth",0.1}, medium=0.1}
local peaks = poly({{61,619},{64,611},{69,613},{74,610},{79,613},{85,614},{88,622},{86,623},{80,620},{75,616.5},{70,621},{66,617},{62,620}}, true)
work(peaks, {hand="detail", pile=fgMid, coverage=4, fill=true, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.2, point=0.6}})
local drift = poly({{34,662},{44,655},{60,652.5},{80,653},{100,655},{112,661},{100,668},{40,668}}, true)
work(drift, {hand="body", pile=fgMid, coverage=2, fill=true, angle=0, edge="soft", tool={kind="filbert", width=5}, length={12,30}})
-- keep a lit lip of snow at the foot of the stump
local tb = brush{kind="round", width=1.6, point=0.8}
tb:load(capShade, 0.7)
tb:stroke({{44,655.5},{56,652.5},{70,654},{86,652.5},{99,655}}, {pressure={tb:pressure_for(1.2), tb:pressure_for(0.8)}, shake=0.4})

--@ chunk 105
print(wait(5*24*60)); print(drying(70,660), drying(70,615))

--@ chunk 106
strawM = pile{{"yellow ochre",0.8},{"lead white",1.0},{"raw umber",0.6},{"smalt",0.25}, medium=0.1}
strawD2 = pile{{"raw umber",1.5},{"bone black",0.35},{"yellow ochre",0.3},{"smalt",0.2}, medium=0.1}
strawL = pile{{"lead white",2},{"yellow ochre",0.5},{"raw umber",0.3}, medium=0.1}
function tuft2(x, y, s, n, seed, lean, spread)
  math.randomseed(seed)
  local b = brush{kind="rigger", width=math.max(1.2, 1.5*s), point=1}
  for i=1,n do
    local r = math.random()
    local p = (r < 0.45) and strawM or ((r < 0.8) and strawD2 or strawL)
    b:reload(p, 0.6)
    local bx = x + randn(0, (spread or 4)*s)
    local by = y + randn(0, 1.2*s)
    local h = rand(7, 26) * s
    local a = -math.pi/2 + randn(lean or 0.15, 0.3)
    local bend = randn(0.3, 0.25)
    if math.random() < 0.12 then bend = bend + 1.2 end   -- a stem bent over
    local pts = {{bx, by}}
    local cx, cy, ca = bx, by, a
    for k=1,5 do ca = ca + bend*0.2; cx = cx + math.cos(ca)*h/5; cy = cy + math.sin(ca)*h/5; pts[#pts+1] = {cx, cy} end
    b:stroke(pts, {pressure={b:pressure_for(math.max(0.6, 0.8*s)), 0}, ramps={0.02, 0.55}})
    if math.random() < 0.18 then
      -- a seed head
      local hb = brush{kind="round", width=math.max(1.2, 1.8*s), point=0.8}
      hb:load(strawD2, 0.7)
      hb:stroke({{cx, cy}, {cx + math.cos(ca)*2.5*s, cy + math.sin(ca)*2.5*s}}, {pressure={0.6, 0.2}})
    end
  end
end
-- restate the old tufts muted, and many more
local old = {
 {40,640,1.1,14},{90,684,1.3,16},{230,702,1.4,12},{62,592,0.9,10},{270,642,1.0,9},{18,700,1.4,12},{140,620,1.0,7},
 {722,602,0.9,10},{762,632,1.0,12},{884,642,1.1,14},{944,692,1.3,16},{982,612,0.9,10},{652,694,1.3,10},{820,690,1.3,8},
 {300,542,0.55,8},{560,522,0.5,7},{782,522,0.5,8},{932,532,0.55,9},{122,547,0.55,9},{200,560,0.6,6},{640,540,0.5,6},
 {372,512,0.45,7},{444,514,0.45,6},{828,508,0.45,7},{878,506,0.45,6},{545,489,0.35,6},{742,487,0.35,6},{470,500,0.4,5}}
for i,t in ipairs(old) do tuft2(t[1], t[2], t[3], t[4] + 10, 900+i, 0.2) end
local new = {
 {38,664,1.4,40,6},{104,664,1.4,35,5},{70,670,1.5,30,8},{772,688,1.4,30,6},{860,692,1.5,25,7},{955,692,1.5,30,6},{905,690,1.4,20,5},
 {130,706,1.6,25,6},{300,706,1.6,18,5},{380,690,1.5,12,4},{560,700,1.6,14,5},{700,706,1.6,16,5},{990,650,1.2,18,5},
 {180,640,1.1,12,4},{330,620,1.0,10,4},{600,640,1.0,10,4},{680,610,0.9,10,4},{960,600,0.9,12,4},{520,660,1.1,10,4},
 {250,575,0.7,10,3},{420,590,0.8,8,3},{700,570,0.7,9,3},{860,575,0.7,10,3},{60,560,0.6,10,3},{980,560,0.6,9,3},
 {330,528,0.45,8,3},{600,515,0.4,6,3},{700,512,0.4,6,3},{900,522,0.45,8,3},{160,530,0.45,8,3},{250,520,0.4,6,3}}
for i,t in ipairs(new) do tuft2(t[1], t[2], t[3], t[4], 1000+i, 0.22, t[5]) end

--@ chunk 107
print(type(TRAIL)); if type(TRAIL)=="table" then for k,v in pairs(TRAIL) do print(k, type(v)) end end

--@ chunk 108
local c = {{372,566},{390,555},{408,545},{426,536}}
local w = {8,7.5,7,6.5}
work(ribbon(c, w):roughen(0.8, 6, 41), {hand="detail", pile=trailP, coverage=3, fill=true, angle=-0.5, load=1, pressure={0.5,0.8}, tool={kind="filbert", width=2.5}})
local e = {{371,563},{389,552},{407,542},{425,533}}
work(ribbon(e, {2.4,2.2,2,1.9}):roughen(0.5, 5, 43), {hand="detail", pile=trailK, coverage=3, fill=true, angle=-0.5, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.5}})

--@ chunk 109
print(type(ROCKBAND), type(UPM), type(CAPM)); print(drying(620,460), drying(600,480)); print(UPM:at(560,480), ROCKBAND:at(600,460), UPM:area(), ROCKBAND:area())

--@ chunk 110
print(capbot(552), capbot(640), capbot(735), captop(640), type(rkC), type(capbot))

--@ chunk 111
local g = noise{seed=77, octaves=3, period=7, kind="ridged"}; print(g(600,460))

--@ chunk 112
rkMid = pile{{"lead white",1.2},{"raw umber",0.9},{"smalt",0.45},{"bone black",0.25},{"yellow ochre",0.1}, medium=0.1}
rkDeep = pile{{"bone black",1.0},{"raw umber",0.8},{"smalt",0.3}, medium=0.1}
lichG = pile{{"lead white",1.2},{"yellow ochre",0.5},{"raw umber",0.3},{"smalt",0.3}, medium=0.1}
local rock = (ROCKBAND + UPM) * rect(540, 440, 205, 55)
local g = noise{seed=77, octaves=3, period=7, kind="ridged", stretch={0.1, 1.6}}
work(rock:shrink(0.8):times(function(x,y) return clamp(g(x,y)*1.6-0.35, 0, 1) end), {hand="detail", pile=rkMid, coverage=1.5, fill=true, angle=0.1, load=0.8, pressure={0.35,0.6}, tool={kind="round", width=1.3, point=0.8}})
-- cracks and joints
local b = brush{kind="rigger", width=1.2, point=1}
b:load(rkDeep, 0.6)
local cracks = {
 {{566,452},{570,458},{569,463}}, {{612,450},{615,456},{621,459}}, {{660,452},{657,457},{661,462}},
 {{700,451},{703,457}}, {{590,473},{592,480},{591,488}}, {{617,476},{616,486}}, {{667,473},{669,482},{667,489}}, {{711,474},{713,484}}}
for _,c in ipairs(cracks) do b:stroke(c, {pressure={0.6,0.2}}) end
-- sky-lit left edges of the uprights and the capstone's left end
local R = (ROCKBAND + UPM)
local rim = mask(function(x,y) if x < 540 or x > 750 or y < 440 or y > 495 then return 0 end return R:at(x,y) * (1 - R:at(x-2.2,y)) end)
work(rim, {hand="detail", pile=rkC, coverage=3, fill=true, angle=math.pi/2, load=0.8, pressure={0.4,0.7}, tool={kind="round", width=1, point=1}})
-- lichen specks
math.randomseed(5)
local lb2 = brush{kind="round", width=1.2, point=0.8}
lb2:load(lichG, 0.5)
for i=1,26 do local x, y = rand(552,735), rand(452,488) if R:at(x,y) > 0.9 then lb2:stroke({{x,y},{x+rand(0.5,1.5),y+rand(-0.4,0.4)}}, {pressure={0.4,0.3}}) end end

--@ chunk 113
print(wait(5*24*60)); print(drying(600,465), drying(560,475))

--@ chunk 114
rockGlz = pile{{"bone black",1.2},{"raw umber",1.0},{"smalt",0.35}, medium=0.7}
local R = (ROCKBAND + UPM) * rect(540, 440, 205, 55)
work(R:grow(0.6), {hand="glaze", pile=rockGlz, coverage=2.5, angle=0.1})

--@ chunk 115
print(wait(6*24*60)); print(drying(600,465), drying(640,480), drying(538,460)); print(OPEN:area(), BASED:area(), ROCKBAND:area(), UPM:area()); for _,p in ipairs({{595,478},{640,480},{688,478},{715,468},{740,470},{548,470}}) do print(p[1],p[2], OPEN:at(p[1],p[2]), ROCKBAND:at(p[1],p[2]), UPM:at(p[1],p[2]), CAPM:at(p[1],p[2]), knollf(p[1])) end

--@ chunk 116
rkBase = pile{{"bone black",0.8},{"raw umber",1.2},{"smalt",0.35},{"lead white",0.35},{"red earth",0.08}, medium=0.08}
rkGrain = pile{{"bone black",0.6},{"raw umber",1.0},{"smalt",0.4},{"lead white",0.7},{"yellow ochre",0.08}, medium=0.08}
local R = ((ROCKBAND + UPM) - OPEN) * rect(540, 440, 205, 55)
work(R, {hand="detail", pile=rkBase, coverage=4, fill=true, angle=0.08, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.5}})
local g = noise{seed=91, octaves=3, period=9, kind="fbm", stretch={0.1, 1.8}}
work(R:shrink(1):times(function(x,y) return clamp(g(x,y)*1.5 - 0.25, 0, 1) end), {hand="detail", pile=rkGrain, coverage=1.2, fill=true, angle=0.1, load=0.7, pressure={0.35,0.55}, tool={kind="round", width=1.4, point=0.8}})
-- the openings: misty plain above the knoll line, snow below
local kn = below(function(x) return knollf(x) - 1 end)
work(OPEN - kn, {hand="detail", pile=mist, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(OPEN * kn, {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
-- the spill to the left of the dolmen
local L = rect(526, 442, 24, 46) - (ROCKBAND + UPM + CAPM):grow(0.3)
work(L - kn, {hand="detail", pile=mist, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(L * kn, {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})

--@ chunk 117
for _,n in ipairs({"sD","sE","mist","dist1","dist2","crest","knb"}) do print(n, tostring(_G[n])) end

--@ chunk 118
print(type(SLAB), tostring(SLAB))

--@ chunk 119
plainP = pile{{"lead white",6},{"smalt",1.0},{"raw umber",0.12},{"yellow ochre",0.15},{"red earth",0.05}, medium=0.15}
local kn = below(function(x) return knollf(x) - 1 end)
local sky = above(function(x) return 461 end)
local ROCKS = (ROCKBAND + UPM + CAPM):grow(0.3)
local SM = SLAB:mask()
local L = rect(526, 442, 24, 46) - ROCKS
work(L * sky, {hand="detail", pile=sE, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(L - sky - kn, {hand="detail", pile=plainP, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
local G = rect(722, 452, 26, 38) - ROCKS - SM
work(G - kn, {hand="detail", pile=plainP, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(G * kn, {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(SM, {hand="detail", pile=rkBase, coverage=4, fill=true, angle=1.2, load=1, pressure={0.6,0.9}, tool={kind="round", width=2, point=0.5}})
local g = noise{seed=93, octaves=3, period=9, kind="fbm", stretch={1.2, 1.8}}
work(SM:shrink(1):times(function(x,y) return clamp(g(x,y)*1.5 - 0.25, 0, 1) end), {hand="detail", pile=rkGrain, coverage=1.2, fill=true, angle=1.2, load=0.7, pressure={0.35,0.55}, tool={kind="round", width=1.4, point=0.8}})

--@ chunk 120
plainP2 = pile{{"lead white",5},{"smalt",1.1},{"raw umber",0.15},{"yellow ochre",0.12},{"red earth",0.05}, medium=0.15}
local kn = below(function(x) return knollf(x) - 1 end)
local sky = above(function(x) return 461 end)
local ROCKS = (ROCKBAND + UPM + CAPM):grow(0.3)
local SM = SLAB:mask():grow(0.3)
local G = rect(720, 440, 30, 21) - ROCKS - SM
work(G * sky, {hand="detail", pile=sE, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
local P = (rect(514, 461, 38, 21) + rect(718, 461, 34, 20)) - ROCKS - SM - kn
work(P, {hand="body", pile=plainP2, coverage=2.5, fill=true, angle=0, edge="soft", length={8,20}, tool={kind="filbert", width=3}})
blend((rect(512, 455, 42, 30) + rect(716, 440, 38, 45)) - ROCKS - SM, {angle=0})

--@ chunk 121
print(wait(5*24*60)); print(drying(530,470), drying(730,470), drying(600,465), drying(760,470))

--@ chunk 122
print(wait(4*24*60)); print(drying(530,470), drying(730,470), drying(600,465), drying(760,470), drying(640,480))

--@ chunk 123
plainA = pile{{"lead white",3},{"smalt",1.2},{"raw umber",0.25},{"red earth",0.06},{"yellow ochre",0.08}, medium=0.15}
plainB = pile{{"lead white",3},{"smalt",0.9},{"raw umber",0.2},{"red earth",0.05},{"yellow ochre",0.12}, medium=0.15}
work(rect(518,466,8,6), {hand="detail", pile=plainA, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(rect(536,466,8,6), {hand="detail", pile=plainB, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})

--@ chunk 124
plainA = pile{{"lead white",1.5},{"smalt",1.0},{"raw umber",0.3},{"red earth",0.05},{"yellow ochre",0.15}, medium=0.15}
plainB = pile{{"lead white",1.0},{"smalt",1.0},{"raw umber",0.35},{"red earth",0.05},{"yellow ochre",0.15}, medium=0.15}
work(rect(518,474,8,5), {hand="detail", pile=plainA, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(rect(536,474,8,5), {hand="detail", pile=plainB, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})

--@ chunk 125
plainC = pile{{"lead white",1.8},{"smalt",1.0},{"raw umber",0.28},{"red earth",0.05},{"yellow ochre",0.15}, medium=0.15}
local kn = below(function(x) return knollf(x) - 0.5 end)
local ROCKS = (ROCKBAND + UPM + CAPM):grow(0.2)
local SM = SLAB:mask():grow(0.2)
local P = (rect(512, 461, 42, 24) + rect(718, 461, 34, 22)) - ROCKS - SM - kn
work(P, {hand="detail", pile=plainC, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})

--@ chunk 126
print(wait(7*24*60)); print(drying(520,470), drying(540,470), drying(730,470))

--@ chunk 127
print(wait(4*24*60)); print(drying(520,470), drying(540,470), drying(530,450))

--@ chunk 128
skyS = pile{{"lead white",4},{"chrome yellow",0.45},{"yellow ochre",0.45},{"vermilion",0.05}, medium=0.2}
local kn = below(function(x) return knollf(x) - 0.5 end)
local ROCKS = (ROCKBAND + UPM + CAPM):grow(0.2)
work(rect(511, 461, 43, 24) - ROCKS - kn, {hand="detail", pile=plainC, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})
work(rect(538, 444, 7, 5) - ROCKS, {hand="detail", pile=skyS, coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.5, point=0.5}})

--@ chunk 129
local kn = below(function(x) return knollf(x) - 0.5 end)
local O1 = poly({{583,470.5},{595,470},{606,469},{606.5,489},{582,489}})
local I1 = poly({{581,469},{585.5,469.5},{589.5,489},{581,489}})
local O2 = poly({{629,471.5},{640,471.2},{651,470.5},{651,489},{629,489}})
local I2 = poly({{631,489.5},{633,485},{637,482.5},{642,482},{646,483.5},{649.5,486.5},{650.5,489.5}}, true)
local O3 = poly({{677,470},{687,470.4},{696,471},{696,489},{677,489}})
local I3 = poly({{689.5,470.5},{697,470.5},{697,489},{693.5,489},{691.5,483},{689,477}})
local light = (O1 - I1) + (O2 - I2) + (O3 - I3)
local op = {hand="detail", coverage=4, fill=true, angle=0, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.2, point=0.5}}
op.pile = plainC; work(light - kn, op)
op.pile = crest; work(light * kn, op)
op.pile = rkBase; op.angle = 1.4; work(I1 + I2 + I3, op)
-- snow on the slab's upper-left edge
local sl = {{739,482},{743,473},{748,464},{752,459},{757,456.5}}
work(ribbon(sl, {1.2,1.8,2.2,2.2,1.6}):roughen(0.4, 4, 7), {hand="detail", pile=capShade, coverage=4, fill=true, load=1, pressure={0.5,0.8}, tool={kind="round", width=1, point=0.8}})
-- low drifts banked against the uprights' feet
local d = ellipse(566,490,13,2.6) + ellipse(617,490.5,11,2.4) + ellipse(664,490,12,2.6) + ellipse(711,490.5,13,2.6) + ellipse(760,491,18,3)
work(d:roughen(0.6, 5, 9), {hand="detail", pile=crest, coverage=4, fill=true, angle=0, load=1, pressure={0.5,0.8}, tool={kind="round", width=1.2, point=0.5}})

--@ chunk 130
print(wait(7*24*60)); print(drying(540,446), drying(640,480), drying(566,490), drying(750,465), drying(530,470))

--@ chunk 131
skyS2 = pile{{"lead white",4},{"chrome yellow",0.3},{"yellow ochre",0.38},{"vermilion",0.04}, medium=0.2}
local ROCKS = (ROCKBAND + UPM + CAPM):grow(0.4)
local sky = above(function(x) return 461 end)
local m = (rect(508, 435, 46, 27) * sky - ROCKS):soften(2)
work(m, {hand="body", pile=skyS2, coverage=2.5, fill=true, angle=0, edge="soft", length={10,25}, tool={kind="filbert", width=3}})
-- the snow cap's dirtied left end
local c = CAPM * rect(545, 432, 32, 22)
work(c, {hand="detail", pile=capShade, coverage=4, fill=true, angle=0.3, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.3, point=0.5}})

--@ chunk 132
print(wait(4*24*60)); print(drying(560,447), drying(530,450))

--@ chunk 133
local sh = poly({{545.5,458},{546.5,452},{550,448},{557,445.5},{568,444.5},{582,446},{582,452},{570,450.5},{560,451.5},{552,454},{548,458}}, true)
work(sh, {hand="detail", pile=capShade, coverage=4, fill=true, angle=0.4, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.2, point=0.5}})
local tb = brush{kind="round", width=1.2, point=0.8}
tb:load(snowTop, 0.6)
tb:stroke({{547,452},{550,448.3},{557,445.8},{568,444.8},{580,445.8}}, {pressure={tb:pressure_for(0.7), tb:pressure_for(1.0)}})

--@ chunk 134
fgVeil = pile{{"lead white",1.2},{"smalt",1.0},{"raw umber",0.22},{"red earth",0.06}, medium=0.55}
local bd = poly({{744,694},{760,684},{775,679},{790,681},{805,678},{822,682},{840,679},{858,683},{875,679},{893,682},{905,684},{920,682},{935,685},{952,686},{968,693},{940,697},{780,697}}, true)
work(bd:soften(2), {hand="body", pile=fgVeil, coverage=1.6, fill=true, angle=0, edge="soft", length={10,25}, tool={kind="filbert", width=3}})
-- the snow cap: a cool shadowed lower part so the top stays lit
local capLow = BOULDER:mask():grow(1) * below(function(x) return 641 + math.abs(x-835)*0.12 + 1.5*math.sin(x/7) end) * above(function(x) return 648 + math.abs(x-835)*0.1 end)
work(capLow, {hand="detail", pile=fgMid, coverage=3, fill=true, angle=0.1, load=0.8, pressure={0.4,0.7}, tool={kind="round", width=1.5, point=0.5}})

--@ chunk 135
local band = BOULDER:mask():grow(1) * above(function(x) return 648.5 + math.abs(x-835)*0.1 + 0.8*math.sin(x/6) end) * below(function(x) return 637 + math.abs(x-835)*0.1 end)
work(band, {hand="detail", pile=fgMid, coverage=4, fill=true, angle=0.1, load=1, pressure={0.6,0.9}, tool={kind="round", width=1.3, point=0.5}})
local edge = rect(740, 690, 235, 12)
blend(edge, {angle=math.pi/2})

--@ chunk 136
print(wait(7*24*60)); print(drying(830,645), drying(850,690), drying(800,694))

--@ chunk 137
fgLav = pile{{"lead white",2.2},{"smalt",1.2},{"raw umber",0.25},{"red earth",0.1},{"vermilion",0.03}, medium=0.12}
-- a scumbled, broken lower edge: short horizontal dabs of the ground's lavender, dragged over the drift's bottom
math.randomseed(71)
local b = brush{kind="filbert", width=3}
for i=1,70 do
  if b:fullness() < 0.3 then b:load(fgLav, 0.5) end
  local x = rand(742, 975); local y = rand(689, 700)
  local l = rand(4, 14)
  b:stroke({{x, y}, {x + l, y + rand(-0.8, 0.8)}}, {pressure={0.35, 0.15}, shake=0.3})
end
-- tufts across the drift
for i,t in ipairs({{770,697,1.3,14,6},{812,700,1.4,12,5},{880,699,1.3,12,5},{938,701,1.4,14,6},{850,705,1.5,10,5}}) do tuft2(t[1], t[2], t[3], t[4], 2000+i, 0.22, t[5]) end

--@ chunk 138
print(wait(21*24*60)); local pts={{830,645},{850,690},{540,450},{640,480},{70,660},{300,650},{400,300},{500,100},{760,470}}; local s={} for _,p in ipairs(pts) do s[#s+1]=drying(p[1],p[2]) end print(table.concat(s," "))

--@ chunk 139
varnish{coats=0.3, vary=0.1}
