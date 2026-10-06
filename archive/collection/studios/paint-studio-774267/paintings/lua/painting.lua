-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=720, aspect=1.55, linen={15,13}, seed=7, ground={{pile={{"red earth",2},{"yellow ochre",2},{"lead white",1}}, um=90, apply="knife", texture=0.3},{pile={{"lead white",6},{"yellow ochre",0.25},{"raw umber",0.08}}, um=45, apply="brush"}}}
print(W,H); print(table.concat(tubes(), ", "))

--@ chunk 2
HZ = 392
function far(x) return HZ - 3 - 4*math.sin(x/70+1) - 2*math.sin(x/23) - 7*smoothstep(760,1000,x) end
function mid1(x) return 440 + 6*math.sin(x/120) + 3*math.sin(x/37+2) end   -- far edge of near meadow band
function fg(x) return 520 - 30*smoothstep(0,350,350-x)*0 + 8*math.sin(x/90+0.5) - 22*math.exp(-((x-250)/180)^2) end -- foreground bank top
h = pencil("2H")
-- horizon line, lightly
local pts = {}
for x=0,1000,20 do pts[#pts+1] = {x, far(x)} end
h:sketch(pts, {pressure=0.25, passes=2})
local p2 = {}
for x=0,1000,25 do p2[#p2+1] = {x, fg(x)} end
h:sketch(p2, {pressure=0.25, passes=2})
-- stream (ditch) centre line
STREAM = {{1000,428},{900,433},{800,441},{700,447},{600,452},{520,458},{450,466},{380,474},{310,478},{240,482},{160,490},{80,500},{0,510}}
h:sketch(STREAM, {pressure=0.25, passes=2})
-- towers
TW = {{x=602, top=312, w=9, base=HZ-2, sp=34}, {x=668, top=334, w=12, base=HZ-2, sp=12}, {x=724, top=326, w=8, base=HZ-2, sp=26}}
for _,t in ipairs(TW) do
  local tb = t.top + t.sp
  h:line({{t.x - t.w/2, t.base}, {t.x - t.w/2, tb}}, {pressure=0.35, smooth=false})
  h:line({{t.x + t.w/2, t.base}, {t.x + t.w/2, tb}}, {pressure=0.35, smooth=false})
  h:line({{t.x - t.w/2 - 1, tb}, {t.x, t.top}, {t.x + t.w/2 + 1, tb}}, {pressure=0.35, smooth=false})
end
-- figures, a man and a woman seen from behind
FIG = {man={x=322, feet=583, h=64}, woman={x=338, feet=584, h=57}}
local m = FIG.man
h:sketch({{m.x, m.feet-m.h}, {m.x-2, m.feet-m.h+10}, {m.x-7, m.feet-m.h+16}, {m.x-8, m.feet-30}, {m.x-5, m.feet}}, {pressure=0.3})
h:sketch({{m.x, m.feet-m.h}, {m.x+3, m.feet-m.h+10}, {m.x+7, m.feet-m.h+16}, {m.x+6, m.feet-30}, {m.x+3, m.feet}}, {pressure=0.3})
local wo = FIG.woman
h:sketch({{wo.x, wo.feet-wo.h}, {wo.x-3, wo.feet-wo.h+12}, {wo.x-7, wo.feet-wo.h+20}, {wo.x-11, wo.feet}}, {pressure=0.3})
h:sketch({{wo.x, wo.feet-wo.h}, {wo.x+3, wo.feet-wo.h+12}, {wo.x+7, wo.feet-wo.h+20}, {wo.x+11, wo.feet}}, {pressure=0.3})
-- windmills
MILLS = {{x=150, base=far(150)+1, h=20}, {x=872, base=far(872)+1, h=17}}
for _,ml in ipairs(MILLS) do
  h:sketch({{ml.x-4, ml.base}, {ml.x-3, ml.base-ml.h}, {ml.x+3, ml.base-ml.h}, {ml.x+4, ml.base}}, {pressure=0.3})
end
-- willows along the ditch
WILLOWS = {{x=70, y=500, h=58}, {x=128, y=493, h=50}, {x=196, y=486, h=44}, {x=262, y=481, h=36}, {x=318, y=477, h=30}}
for _,wl in ipairs(WILLOWS) do
  h:sketch({{wl.x-3, wl.y}, {wl.x-2, wl.y-wl.h*0.45}}, {pressure=0.3})
  h:sketch({{wl.x+3, wl.y}, {wl.x+2, wl.y-wl.h*0.45}}, {pressure=0.3})
end

--@ chunk 3
SKY = above(function(x) return far(x)+5 end)
local function skypile(t)
  local s = {}
  s[#s+1] = {"lead white", lerp(3.2, 10, t^0.8)}
  if t < 0.85 then s[#s+1] = {"smalt", 1.6*(1-t/0.85)^1.3} end
  if t < 0.6 then s[#s+1] = {"cobalt blue", 0.45*(1-t/0.6)^1.5} end
  s[#s+1] = {"pale smalt", lerp(1.1, 0.15, t)}
  if t > 0.55 then s[#s+1] = {"yellow ochre", 0.4*smoothstep(0.55,1,t)} end
  if t > 0.65 then s[#s+1] = {"vermilion", 0.035*math.sin(math.pi*smoothstep(0.65,1.05,t))} end
  if t > 0.8 then s[#s+1] = {"chrome yellow", 0.18*smoothstep(0.8,1,t)} end
  s.medium = 0.18
  return pile(s)
end
local ys = {0, 50, 100, 150, 200, 245, 285, 320, 350, 372, 400}
SKYP = {}
for i=1,#ys-1 do
  local t = (i-1)/(#ys-2)
  local p = skypile(t)
  SKYP[i] = p
  local m = rect(-10, ys[i]-4, 1020, ys[i+1]-ys[i]+8) * SKY
  work(m, {hand="body", tool="flat 12", pile=p, angle=0, angle_jitter=0.06, length={70,180}, coverage=2.6, fill=true, pressure={0.5,0.8}, seed=100+i})
end
print(SKYP[1], SKYP[10])

--@ chunk 4
local ys = {0, 45, 90, 135, 180, 225, 265}
for i=1,#ys-1 do
  local t = (i-1)/(#ys-2)
  local p = pile{{"lead white", lerp(1.6, 5, t)}, {"smalt", lerp(3, 1.2, t)}, {"cobalt blue", lerp(1.4, 0.2, t)}, {"pale smalt", 0.8}, {"Prussian blue", 0.04*(1-t)+0.001}, medium=0.2}
  local m = rect(-10, ys[i]-4, 1020, ys[i+1]-ys[i]+8)
  work(m, {hand="body", tool="flat 12", pile=p, angle=0, angle_jitter=0.05, length={80,200}, coverage=2.2, fill=true, pressure={0.5,0.8}, seed=200+i})
end

--@ chunk 5
local pt = pile{{"lead white", 8}, {"smalt", 0.9}, {"pale smalt", 1}, {"cobalt blue", 0.1}, {"yellow ochre", 0.08}, medium=0.2}
work(rect(-10, 245, 1020, 70), {hand="body", tool="flat 12", pile=pt, angle=0, angle_jitter=0.05, length={80,200}, coverage=1.8, fill=true, pressure={0.4,0.7}, seed=301})
for k=1,3 do
  blend(SKY, {angle=0, ruler=true, length={150,400}, coverage=2, seed=310+k})
end

--@ chunk 6
-- low cumulus bank above the left horizon
local cm = ellipse(60, 372, 90, 22) + ellipse(150, 362, 70, 26) + ellipse(230, 370, 80, 20) + ellipse(320, 376, 70, 14) + ellipse(110, 350, 40, 16) + ellipse(185, 342, 36, 18) + ellipse(400, 380, 60, 9)
CLOUD1 = (cm:roughen(8, 40, 11) * SKY)
CLOUDBODY = pile{{"lead white", 6}, {"pale smalt", 1.6}, {"red earth", 0.12}, {"raw umber", 0.08}, {"vermilion", 0.02}, medium=0.2}
CLOUDLIT = pile{{"lead white", 9}, {"yellow ochre", 0.35}, {"vermilion", 0.06}, medium=0.2}
work(CLOUD1, {hand="body", tool="filbert 7", pile=CLOUDBODY, angle=0, angle_jitter=0.3, length={15,40}, coverage=2.2, fill=true, edge="soft", seed=401})
-- lit upper-left rims of the cloud heads (sun low on the left)
local lit = CLOUD1 - CLOUD1:offset(0):grow(0) -- placeholder so the var exists
local tops = (CLOUD1 - (cm:roughen(8,40,11)):shrink(0)) -- unused
local rimm = mask(function(x, y)
  local v = CLOUD1:at(x, y); if v < 0.5 then return 0 end
  local up = CLOUD1:at(x - 4, y - 7)
  return (up < 0.5) and 1 or 0 end)
work(rimm:grow(2), {hand="body", tool="filbert 5", pile=CLOUDLIT, angle=-0.2, angle_jitter=0.4, length={8,22}, coverage=2, fill=true, edge="soft", seed=402})
blend(CLOUD1:grow(6), {angle=0, length={20,60}, coverage=1.2, seed=403})

--@ chunk 7
-- long evening streaks, thin, catching warm light
local wisp = pile{{"lead white", 9}, {"yellow ochre", 0.2}, {"vermilion", 0.07}, {"pale smalt", 0.3}, medium=0.25}
local gray = pile{{"lead white", 5}, {"pale smalt", 1.5}, {"red earth", 0.1}, {"raw umber", 0.06}, medium=0.25}
local n = noise{seed=21, octaves=3, period=90}
local function streak(x0, x1, y0, dy, w, seed)
  local pts, ws = {}, {}
  for i=0,40 do
    local t = i/40
    local x = lerp(x0, x1, t)
    pts[#pts+1] = {x, y0 + dy*t + 2*n(x, y0)}
    ws[#ws+1] = w * math.sin(math.pi*t)^0.7 * (0.7 + 0.5*n:at01(x*2, y0+seed))
  end
  return ribbon(pts, ws):roughen(1.5, 12, seed)
end
WISPS = streak(560, 980, 212, -8, 7, 1) + streak(610, 890, 232, -4, 4, 2) + streak(700, 1010, 252, -6, 5, 3) + streak(480, 700, 268, 2, 3.5, 4) + streak(20, 330, 238, 4, 4, 5) + streak(90, 260, 222, 2, 2.5, 6)
GRAYS = streak(600, 940, 219, -7, 3, 7) + streak(720, 1000, 258, -5, 2.5, 8)
work(GRAYS, {hand="body", tool="filbert 4", pile=gray, angle=0, angle_jitter=0.08, length={30,80}, coverage=1.5, pressure={0.3,0.5}, seed=501})
work(WISPS, {hand="body", tool="filbert 5", pile=wisp, angle=0, angle_jitter=0.08, length={30,90}, coverage=1.8, pressure={0.3,0.6}, seed=502})
blend((WISPS+GRAYS):grow(5), {angle=0, ruler=true, length={40,120}, coverage=1, seed=503})

--@ chunk 8
LAND = below(function(x) return far(x) end)
local bnds = {function(x) return far(x) end, function(x) return 403 + 2*math.sin(x/80) end, function(x) return 424 + 3*math.sin(x/110+1) end, function(x) return 466 + 5*math.sin(x/140+2) end, function(x) return fg(x) end, function(x) return 700 end}
ZONES = {}
for i=1,5 do
  local a, b = bnds[i], bnds[i+1]
  ZONES[i] = mask(function(x, y) local ya, yb = a(x), b(x); return (y >= ya and y < yb) and 1 or 0 end)
end
ZP = {
  pile{{"lead white", 3}, {"pale smalt", 1.6}, {"green earth", 1}, {"yellow ochre", 0.25}, {"red earth", 0.05}, medium=0.15},
  pile{{"lead white", 2}, {"green earth", 1.6}, {"yellow ochre", 0.7}, {"pale smalt", 0.7}, medium=0.15},
  pile{{"yellow ochre", 1.6}, {"green earth", 1.6}, {"lead white", 1.4}, {"chrome yellow", 0.2}, {"Prussian blue", 0.05}, medium=0.15},
  pile{{"yellow ochre", 1.4}, {"green earth", 2}, {"lead white", 0.8}, {"Prussian blue", 0.1}, {"raw umber", 0.15}, medium=0.15},
  pile{{"raw umber", 1}, {"green earth", 2}, {"yellow ochre", 1}, {"Prussian blue", 0.12}, {"bone black", 0.08}, medium=0.15},
}
local tools = {"flat 5", "flat 7", "flat 9", "filbert 10", "filbert 12"}
for i=1,5 do
  work(ZONES[i], {hand="body", tool=tools[i], pile=ZP[i], angle=function(x,y) return 0.03*math.sin(x/50) end, angle_jitter=0.1, length={25+i*10, 60+i*20}, coverage=2.3, fill=true, pressure={0.5,0.8}, seed=600+i})
end

--@ chunk 9
local midm = ZONES[1] + ZONES[2] + ZONES[3] + ZONES[4]
blend(midm, {angle=0, ruler=true, length={100,300}, coverage=1.5, seed=701})
blend(ZONES[5], {angle=0.05, length={60,160}, coverage=1.2, seed=702})

--@ chunk 10
print(wait(26*60)); for _,p in ipairs({{500,100},{500,350},{500,410},{500,450},{500,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 11
print(wait(24*60)); for _,p in ipairs({{500,100},{500,350},{500,410},{500,450},{500,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 12
print(wait(20*60)); for _,p in ipairs({{500,100},{200,30},{500,350},{500,410},{500,450},{500,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 13
-- distant tree lines along the horizon
local function copse(x0, x1, hmax, seed)
  local m = nil
  local xs = uneven(math.floor((x1-x0)/6), x0, x1, 0.6, 0.4, seed)
  for i, x in ipairs(xs) do
    local hh = hmax * (0.45 + 0.55*math.sin(math.pi*(x-x0)/(x1-x0))^0.6) * (0.75 + 0.5*((i*37 % 11)/11))
    local e = ellipse(x, far(x) - hh*0.55, 4 + hh*0.35, hh*0.6)
    m = m and (m + e) or e
  end
  return m
end
FARTREES = (copse(-10, 120, 14, 1) + copse(190, 250, 8, 2) + copse(290, 330, 6, 3) + copse(420, 545, 11, 4) + copse(745, 800, 9, 5) + copse(830, 860, 7, 6) + copse(900, 1010, 16, 7)):roughen(1.5, 8, 31)
FARTREES = FARTREES * above(function(x) return far(x) + 3 end)
local fartree = pile{{"pale smalt", 2}, {"lead white", 1.4}, {"green earth", 0.8}, {"raw umber", 0.3}, {"bone black", 0.08}, medium=0.1}
stipple(FARTREES, {pile=fartree, width=2.2, coverage=2.5, cluster=0.3, clip=true, seed=1301})
work(FARTREES:shrink(1.5), {hand="detail", pile=fartree, coverage=1.2, seed=1302})

--@ chunk 14
local band = mask(function(x, y) local f = far(x); return (y > f - 3.5 - 1.5*math.sin(x/9) and y < f + 1) and 1 or 0 end) * (rect(-10,0,140,700) + rect(180,0,160,700) + rect(410,0,150,700) + rect(740,0,130,700) + rect(890,0,130,700))
local extra = (ellipse(268, far(268)-7, 7, 8) + ellipse(338, far(338)-6, 8, 6) + ellipse(352, far(352)-4, 6, 4)):roughen(1.2, 6, 5)
FARTREES = FARTREES + band:roughen(1, 6, 7) + extra
local haze = pile{{"pale smalt", 2}, {"lead white", 3}, {"green earth", 0.3}, {"raw umber", 0.12}, {"vermilion", 0.01}, medium=0.15}
stipple(FARTREES, {pile=haze, width=2, coverage=2.2, cluster=0.2, clip=true, seed=1401})

--@ chunk 15
local function b(x) return far(x) + 1.5 end
local parts = {}
local function add(m) parts[#parts+1] = m end
-- houses: {x0, x1, wall, roof}
local houses = {{556,566,4,4},{565,574,5,5},{574,586,4,3},{586,596,6,5},{607,622,9,7},{622,652,10,9},{652,662,6,4},{674,712,11,10},{712,720,6,5},{729,758,8,8},{758,768,5,4},{768,780,4,4},{780,792,3,3},{792,800,3,2}}
for _,h in ipairs(houses) do
  local x0, x1, wl, rf = h[1], h[2], h[3], h[4]
  local by = math.max(b(x0), b(x1))
  add(poly({{x0, by}, {x0, by - wl}, {(x0+x1)/2, by - wl - rf}, {x1, by - wl}, {x1, by}}))
end
for _,t in ipairs(TW) do
  local tb = t.top + t.sp
  local by = b(t.x)
  add(rect(t.x - t.w/2, tb, t.w, by - tb))
  if t.x == 668 then
    add(poly({{t.x - t.w/2 - 0.8, tb}, {t.x - 3, tb - 6}, {t.x + 3, tb - 6}, {t.x + t.w/2 + 0.8, tb}}))
    add(poly({{t.x - 1.6, tb - 6}, {t.x, t.top - 8}, {t.x + 1.6, tb - 6}}))
  else
    add(poly({{t.x - t.w/2 - 0.8, tb}, {t.x, t.top}, {t.x + t.w/2 + 0.8, tb}}))
  end
end
TOWN = parts[1]
for i=2,#parts do TOWN = TOWN + parts[i] end
TOWNP = pile{{"pale smalt", 2}, {"lead white", 2}, {"red earth", 0.15}, {"raw umber", 0.22}, {"bone black", 0.05}, medium=0.1}
work(TOWN, {hand="detail", pile=TOWNP, coverage=2.5, fill=true, seed=1501})

--@ chunk 16
local function b(x) return far(x) + 1.5 end
TW = {{x=602, top=310, w=12, sp=30, body=348}, {x=668, top=322, w=15, sp=0, body=338}, {x=726, top=324, w=11, sp=26, body=352}}
local parts = {}
local function add(m) parts[#parts+1] = m end
local houses = {{554,563,4,4},{563,572,5,5},{572,584,4,3},{584,594,6,5},
  {606,652,13,12},{652,660,6,4},{675,716,14,13},{716,722,6,5},{731,764,10,9},{764,772,5,4},{772,784,4,4},{784,795,3,3},{795,803,3,2}}
for _,h in ipairs(houses) do
  local x0, x1, wl, rf = h[1], h[2], h[3], h[4]
  local by = math.max(b(x0), b(x1))
  add(poly({{x0, by}, {x0, by - wl}, {x0 + 1.5, by - wl - rf}, {x1 - 1.5, by - wl - rf}, {x1, by - wl}, {x1, by}}))
end
for _,t in ipairs(TW) do
  local by = b(t.x)
  add(rect(t.x - t.w/2, t.body, t.w, by - t.body))
  if t.sp == 0 then
    -- the great tower: a low hipped cap with a slim lantern and needle
    add(poly({{t.x - t.w/2 - 0.8, t.body}, {t.x - 3.5, t.body - 7}, {t.x + 3.5, t.body - 7}, {t.x + t.w/2 + 0.8, t.body}}))
    add(rect(t.x - 1.8, t.body - 12, 3.6, 5.5))
    add(poly({{t.x - 2.3, t.body - 12}, {t.x, t.top}, {t.x + 2.3, t.body - 12}}))
  else
    add(poly({{t.x - t.w/2 - 0.8, t.body}, {t.x, t.top}, {t.x + t.w/2 + 0.8, t.body}}))
  end
end
TOWN = parts[1]
for i=2,#parts do TOWN = TOWN + parts[i] end
TOWNP = pile{{"pale smalt", 2}, {"smalt", 0.6}, {"lead white", 1.1}, {"raw umber", 0.4}, {"red earth", 0.18}, {"bone black", 0.1}, medium=0.08}
work(TOWN, {hand="detail", pile=TOWNP, coverage=3, fill=true, seed=1601})

--@ chunk 17
for _,p in ipairs({{600,360},{150,380},{500,420},{500,480},{300,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 18
local kinds = {
  gold = {{"lead white", 2}, {"yellow ochre", 1.5}, {"chrome yellow", 0.25}, {"green earth", 0.3}},
  green = {{"lead white", 1.4}, {"green earth", 1.6}, {"yellow ochre", 0.9}, {"pale smalt", 0.3}},
  shade = {{"lead white", 1.1}, {"green earth", 1.5}, {"pale smalt", 0.8}, {"raw umber", 0.12}, {"yellow ochre", 0.3}},
  fallow = {{"lead white", 1.5}, {"yellow ochre", 1}, {"red earth", 0.3}, {"raw umber", 0.25}},
  lush = {{"lead white", 1}, {"green earth", 1.4}, {"yellow ochre", 0.9}, {"chrome yellow", 0.15}, {"Prussian blue", 0.04}},
}
local order = {"green","gold","shade","lush","fallow","green","gold","lush","shade","green","gold","fallow","lush"}
local function mkpile(kind, t)
  local s = {}
  for _,c in ipairs(kinds[kind]) do s[#s+1] = {c[1], c[2]} end
  s[#s+1] = {"lead white", 2.2*(1-t)}
  s[#s+1] = {"pale smalt", 1.0*(1-t) + 0.01}
  s.medium = 0.2
  return pile(s)
end
local y = 398
local ybs = {}
while y < 466 do ybs[#ybs+1] = y; y = y + 3 + (y-398)*0.13 end
ybs[#ybs+1] = 468
FIELDS = {}
local k = 0
for i=1,#ybs-1 do
  local ya, yb = ybs[i], ybs[i+1]
  local t = ((ya+yb)/2 - 398)/70
  local xs = uneven(math.random(2,4), 60, 940, 0.8, 0.2, 1800+i)
  local xb = {-20}
  for _,x in ipairs(xs) do xb[#xb+1] = x end
  xb[#xb+1] = 1020
  for j=1,#xb-1 do
    k = k + 1
    local sl = rand(-0.9, 0.9)
    local x0, x1 = xb[j], xb[j+1]
    local sa, sb = randn(0, 0.6), randn(0, 0.6)
    local m = mask(function(px, py)
      local top = ya + sa + 0.004*(px-500)*(i%2==0 and 1 or -1)
      local bot = yb + sb + 0.004*(px-500)*((i+1)%2==0 and 1 or -1)
      local l = x0 + (py - ya)*sl
      local r = x1 + (py - ya)*sl
      return (py >= top and py < bot and px >= l and px < r) and 1 or 0 end) * LAND
    local kind = order[(k*7 + i) % #order + 1]
    FIELDS[#FIELDS+1] = {m=m, kind=kind}
    local w = 2 + t*4
    work(m, {hand="body", tool={kind="flat", width=w}, pile=mkpile(kind, t), angle=0, angle_jitter=0.04, length={20,70}, coverage=1.6, fill=true, pressure={0.4,0.7}, edge="firm", seed=1800+k})
  end
end
print(k)

--@ chunk 19
local m = mask(function(x, y) return (y > far(x) + 2 and y < 470) and 1 or 0 end)
blend(m, {angle=0, ruler=true, length={60,200}, coverage=1.3, seed=1901})
-- a veil of warm haze over the far half, wet into wet
local veil = pile{{"lead white", 4}, {"pale smalt", 1}, {"yellow ochre", 0.3}, {"green earth", 0.4}, medium=0.3}
local vm = mask(function(x, y) return (y > far(x) + 2 and y < 440) and 1 or 0 end)
work(vm, {hand="body", tool="flat 8", pile=veil, angle=0, angle_jitter=0.03, length={80,200}, coverage=1.0, pressure={0.25,0.4}, load=0.5, seed=1902})
blend(vm, {angle=0, ruler=true, length={100,300}, coverage=1, seed=1903})

--@ chunk 20
local m = mask(function(x, y) return (y > far(x) + 1.5 and y < 468) and 1 or 0 end)
for k=1,2 do blend(m, {angle=0, ruler=true, length={150,400}, coverage=2, seed=2000+k}) end

--@ chunk 21
local z4 = mask(function(x, y) local a = 467 + 1.5*math.sin(x/50); return (y >= a and y < fg(x) + 2) and 1 or 0 end)
Z4 = z4
local zu = z4 - below(function(x) return 482 + 4*math.sin(x/60) end)
local zl = z4 - zu
local pu = pile{{"green earth", 2}, {"yellow ochre", 1.3}, {"chrome yellow", 0.2}, {"lead white", 1}, {"Prussian blue", 0.04}, medium=0.15}
local pl = pile{{"green earth", 2.2}, {"yellow ochre", 1.1}, {"chrome yellow", 0.1}, {"lead white", 0.5}, {"Prussian blue", 0.08}, {"raw umber", 0.15}, medium=0.15}
work(zu, {hand="body", tool="flat 6", pile=pu, angle=0, angle_jitter=0.05, length={30,90}, coverage=2, fill=true, edge="firm", seed=2101})
work(zl, {hand="body", tool="flat 7", pile=pl, angle=0.02, angle_jitter=0.06, length={30,90}, coverage=2, fill=true, edge="firm", seed=2102})
blend(z4, {angle=0, ruler=true, length={80,250}, coverage=1.3, seed=2103})

--@ chunk 22
FGM = below(function(x) return fg(x) end)
local cuts = {0, 12, 35, 70, 400}
local piles = {
  pile{{"green earth", 2}, {"yellow ochre", 1.6}, {"lead white", 0.6}, {"chrome yellow", 0.12}, {"raw umber", 0.1}, medium=0.15},
  pile{{"green earth", 2}, {"yellow ochre", 1.2}, {"lead white", 0.3}, {"raw umber", 0.3}, {"Prussian blue", 0.04}, medium=0.15},
  pile{{"green earth", 2}, {"yellow ochre", 0.9}, {"raw umber", 0.6}, {"Prussian blue", 0.07}, {"bone black", 0.05}, medium=0.15},
  pile{{"green earth", 1.6}, {"raw umber", 1}, {"yellow ochre", 0.6}, {"Prussian blue", 0.1}, {"bone black", 0.12}, medium=0.15},
}
for i=1,4 do
  local a, b = cuts[i], cuts[i+1]
  local m = mask(function(x, y) local d = y - fg(x); return (d >= a and d < b) and 1 or 0 end)
  work(m, {hand="body", tool="filbert 8", pile=piles[i], angle=function(x,y) return 0.08*math.sin(x/70) end, angle_jitter=0.15, length={25,70}, coverage=2.2, fill=true, edge=(i==1) and "firm" or "soft", seed=2200+i})
end
blend(FGM, {angle=0.05, length={50,150}, coverage=1.2, seed=2205})

--@ chunk 23
print(wait(28*60)); for _,p in ipairs({{600,360},{500,420},{500,450},{500,480},{300,520},{300,620}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 24
print(wait(36*60)); for _,p in ipairs({{600,360},{668,370},{500,420},{500,450},{500,480},{300,520},{300,620}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 25
print(wait(30*60)); for _,p in ipairs({{600,360},{668,370},{500,420},{500,450},{500,480},{300,520},{300,620},{50,375}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 26
-- subtle field patches in the far and middle meadows: thin scumbles of slightly different greens and golds
local sets = {
  {{"lead white", 2.5}, {"green earth", 1.2}, {"pale smalt", 0.6}, {"yellow ochre", 0.3}},
  {{"lead white", 2}, {"yellow ochre", 1.2}, {"chrome yellow", 0.12}, {"green earth", 0.4}},
  {{"lead white", 1.5}, {"green earth", 1.5}, {"yellow ochre", 0.7}, {"Prussian blue", 0.02}},
  {{"lead white", 2}, {"yellow ochre", 0.8}, {"red earth", 0.15}, {"raw umber", 0.15}},
}
local rows = {{398,404},{404,411},{411,419},{419,428},{428,439},{439,452},{452,467}}
local k = 0
for i,r in ipairs(rows) do
  local xs = uneven(3 + (i % 3), 0, 1000, 0.9, 0.3, 2600+i)
  local prev = -30
  table.insert(xs, 1030)
  for j,x in ipairs(xs) do
    k = k + 1
    if (k % 3) ~= 0 then
      local s = sets[(k*5 + i) % 4 + 1]
      local t = (i-1)/6
      local ps = {}
      for _,c in ipairs(s) do ps[#ps+1] = {c[1], c[2]} end
      ps[#ps+1] = {"lead white", 1.5*(1-t)+0.01}
      ps[#ps+1] = {"pale smalt", 0.6*(1-t)+0.01}
      ps.medium = 0.45
      local sl = rand(-1, 1)
      local x0, x1 = prev, x
      local m = mask(function(px, py) local l = x0 + (py-r[1])*sl; local rr = x1 + (py-r[1])*sl; return (py >= r[1] and py < r[2] and px >= l and px < rr) and 1 or 0 end) * LAND
      work(m, {hand="scumble", tool={kind="flat", width=2 + t*4}, pile=pile(ps), angle=0, angle_jitter=0.05, length={10,30}, coverage=1.1, pressure={0.25,0.45}, load=0.5, clip=true, seed=2600+k})
    end
    prev = x
  end
end
print(k)

--@ chunk 27
local m = mask(function(x, y) return (y > far(x) + 2 and y < 468) and 1 or 0 end)
blend(m, {angle=0, ruler=true, length={60,160}, coverage=1.2, seed=2701})
blend(m, {angle=math.pi/2, length={6,14}, coverage=0.6, seed=2702})

--@ chunk 28
print(wait(40*60)); for _,p in ipairs({{600,360},{668,370},{300,420},{500,440},{500,460},{300,520},{330,540},{50,375}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 29
local dark = pile{{"pale smalt", 1.5}, {"smalt", 0.8}, {"lead white", 0.9}, {"raw umber", 0.5}, {"red earth", 0.2}, {"bone black", 0.1}, medium=0.1}
work(TOWN, {hand="detail", pile=dark, coverage=2.5, fill=true, seed=2901})
-- lit west faces of the towers and gable ends
local litp = pile{{"lead white", 2.2}, {"pale smalt", 0.9}, {"red earth", 0.25}, {"yellow ochre", 0.3}, {"raw umber", 0.1}, medium=0.1}
local litm = nil
for _,t in ipairs(TW) do
  local by = far(t.x) + 1.5
  local r = rect(t.x - t.w/2, t.body, t.w*0.38, by - t.body - 4)
  litm = litm and (litm + r) or r
  if t.sp > 0 then
    local s = poly({{t.x - t.w/2 - 0.8, t.body}, {t.x, t.top}, {t.x - 0.2, t.body}})
    litm = litm + s
  end
end
litm = litm * TOWN
work(litm, {hand="detail", pile=litp, coverage=2, fill=true, seed=2902})

--@ chunk 30
local sh = pile{{"smalt", 1.2}, {"pale smalt", 0.8}, {"raw umber", 0.5}, {"bone black", 0.12}, {"lead white", 0.5}, medium=0.3}
local shm = nil
for _,t in ipairs(TW) do
  local by = far(t.x) + 1.5
  local r = rect(t.x - t.w/2 + t.w*0.38, t.body - 20, t.w*0.62 + 1.2, by - t.body + 20)
  shm = shm and (shm + r) or r
end
shm = (shm + rect(540, 355, 300, 40) - rect(0,0,1000,352)) * TOWN
TOWNSH = shm
work(shm, {hand="detail", pile=sh, coverage=1.6, fill=true, seed=3001})

--@ chunk 31
local t3 = TW[3]
local spl = poly({{719.4, 352}, {724.6, 325.5}, {725.6, 323.6}, {725.6, 352}})
local spr = poly({{725.6, 352}, {725.6, 323.6}, {726.4, 324.2}, {732.4, 352}})
local litp = pile{{"lead white", 2.2}, {"pale smalt", 0.9}, {"red earth", 0.25}, {"yellow ochre", 0.3}, {"raw umber", 0.1}, medium=0.1}
local shp = pile{{"pale smalt", 1.2}, {"smalt", 0.7}, {"lead white", 0.9}, {"raw umber", 0.45}, {"red earth", 0.15}, {"bone black", 0.08}, medium=0.1}
work(spl, {hand="detail", pile=litp, tool={kind="round", width=1.2, point=0.8}, coverage=2.5, fill=true, seed=3101})
work(spr, {hand="detail", pile=shp, tool={kind="round", width=1.2, point=0.8}, coverage=2.5, fill=true, seed=3102})
-- belfry openings and nave windows, small dark marks
local dk = pile{{"smalt", 1}, {"raw umber", 0.8}, {"bone black", 0.35}, {"lead white", 0.3}, medium=0.15}
local r = brush{kind="round", width=1.0, point=1, stiffness=0.4}
r:load(dk, 0.5)
local function slit(x, y0, y1, pr)
  r:stroke({{x, y0}, {x, y1}}, {pressure={pr or 0.45, pr or 0.45}, ramps={0.1,0.1}})
end
for _,t in ipairs(TW) do
  local yb = t.body + 4
  local n = (t.w > 13) and 3 or 2
  for i=1,n do
    local x = t.x - t.w/2 + t.w*(i)/(n+1)
    slit(x, yb, yb + 7)
    slit(x, yb + 14, yb + 19, 0.35)
  end
  r:load(dk, 0.3)
end
-- nave windows
for _,nv in ipairs({{608,650,6},{677,714,6},{733,762,5}}) do
  local y1 = far((nv[1]+nv[2])/2) - 2
  for x = nv[1]+4, nv[2]-3, nv[3] do slit(x, y1 - 9, y1 - 3, 0.35) end
  r:load(dk, 0.3)
end

--@ chunk 32
local function crowns(x0, x1, hmax, seed)
  local m
  local nn = noise{seed=seed, octaves=2, period=40}
  local x = x0
  local i = 0
  while x < x1 do
    i = i + 1
    local env = math.sin(math.pi*clamp((x-x0)/(x1-x0), 0, 1))^0.5
    local hh = hmax * (0.35 + 0.65*env) * (0.7 + 0.6*nn:at01(x, seed))
    local rw = 2.5 + hh*0.28 + rand(-0.8, 1.2)
    local base = far(x) + 1
    local e = ellipse(x, base - hh + rw*0.9, rw, rw*1.05) + rect(x - rw*0.9, base - hh + rw, rw*1.8, hh - rw)
    m = m and (m + e) or e
    x = x + rw*rand(0.9, 1.5)
  end
  return m
end
local function poplar(x, h)
  local base = far(x) + 1
  return ellipse(x, base - h*0.55, 2.2, h*0.5) + rect(x-0.4, base - h*0.1, 0.8, h*0.1)
end
TREES2 = crowns(-8, 128, 17, 1) + crowns(178, 258, 9, 2) + crowns(255, 278, 13, 3) + crowns(288, 362, 8, 4) + crowns(414, 552, 13, 5) + crowns(742, 808, 10, 6) + crowns(826, 866, 7, 7) + crowns(884, 1008, 19, 8)
TREES2 = TREES2 + poplar(392, 24) + poplar(398, 20) + poplar(812, 22) + poplar(817, 18) + poplar(164, 16)
TREES2 = (TREES2:roughen(1.2, 5, 41) + FARTREES:grow(0.8)) * above(function(x) return far(x) + 2.5 end) - TOWN
local tp = pile{{"pale smalt", 1.4}, {"lead white", 1.1}, {"green earth", 0.7}, {"raw umber", 0.35}, {"bone black", 0.07}, medium=0.1}
work(TREES2, {hand="detail", pile=tp, coverage=2.2, fill=true, seed=3201})
stipple(TREES2:rim(2, 1), {pile=tp, width=1.6, coverage=1.2, cluster=0.4, clip=false, seed=3202})

--@ chunk 33
-- cover the stray spike near x 290
local bush = (ellipse(292, far(292) - 5, 9, 6) + ellipse(283, far(283) - 3, 6, 4) + ellipse(301, far(301) - 3.5, 5, 4)):roughen(1, 4, 9) * above(function(x) return far(x) + 2.5 end)
local tp = pile{{"pale smalt", 1.4}, {"lead white", 1.1}, {"green earth", 0.7}, {"raw umber", 0.35}, {"bone black", 0.07}, medium=0.1}
work(bush, {hand="detail", pile=tp, coverage=2.2, fill=true, seed=3301})
TREES2 = TREES2 + bush
-- light on the crowns from the low sun at left: upper-left rims
local litrim = mask(function(x, y)
  if TREES2:at(x, y) < 0.5 then return 0 end
  return (TREES2:at(x - 2.2, y - 2.2) < 0.5) and 1 or 0 end)
local lp = pile{{"lead white", 2.2}, {"pale smalt", 0.7}, {"yellow ochre", 0.45}, {"green earth", 0.5}, {"red earth", 0.04}, medium=0.1}
stipple(litrim:grow(0.6), {pile=lp, width=1.3, coverage=1.4, cluster=0.5, clip=true, seed=3302})
-- darker undersides and gaps between crowns
local lower = TREES2 * mask(function(x, y) local f = far(x); return (y > f - 4 and y < f + 3) and 1 or 0 end)
local dp = pile{{"pale smalt", 1.2}, {"smalt", 0.5}, {"raw umber", 0.5}, {"bone black", 0.1}, {"lead white", 0.6}, medium=0.15}
stipple(lower, {pile=dp, width=1.4, coverage=0.9, cluster=0.5, clip=true, seed=3303})

--@ chunk 34
print(wait(40*60)); for _,p in ipairs({{300,420},{500,440},{700,410},{50,380},{500,386},{900,375}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 35
print(wait(20*60)); for _,p in ipairs({{300,420},{500,440},{700,455},{50,380},{500,386},{900,375}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 36
local cuts = {0, 405, 424, 446, 480}
local ps = {
  pile{{"lead white", 1.6}, {"pale smalt", 1.2}, {"green earth", 1.2}, {"yellow ochre", 0.45}, medium=0.2},
  pile{{"lead white", 1.2}, {"green earth", 1.4}, {"yellow ochre", 0.8}, {"pale smalt", 0.55}, medium=0.2},
  pile{{"green earth", 1.6}, {"yellow ochre", 1.2}, {"lead white", 0.9}, {"chrome yellow", 0.12}, {"pale smalt", 0.15}, medium=0.2},
  pile{{"green earth", 2}, {"yellow ochre", 1.3}, {"chrome yellow", 0.18}, {"lead white", 0.9}, {"Prussian blue", 0.03}, medium=0.2},
}
for i=1,4 do
  local a, b = cuts[i], cuts[i+1]
  local m = mask(function(x, y)
    local top = (i == 1) and far(x) + 1.2 or a + 1.5*math.sin(x/37 + i)
    local bot = (i == 4) and 470 + 1.5*math.sin(x/50) or b + 1.5*math.sin(x/37 + i + 1)
    return (y >= top and y < bot) and 1 or 0 end) - TREES2 - TOWN
  work(m, {hand="body", tool={kind="flat", width=3 + i*1.2}, pile=ps[i], angle=0, angle_jitter=0.03, length={40,120}, coverage=1.5, fill=true, pressure={0.4,0.6}, load=0.7, edge="soft", clip=(i==1), seed=3600+i})
end
local all = mask(function(x, y) return (y > far(x) + 1.2 and y < 471) and 1 or 0 end) - TREES2 - TOWN
blend(all, {angle=0, ruler=true, length={100,300}, coverage=1.2, seed=3605})

--@ chunk 37
-- the footpath: from the bottom edge up the rise, past the couple, over the crest
PATHPTS = {{505, 660}, {490, 636}, {462, 612}, {420, 592}, {375, 578}, {340, 570}, {310, 561}, {285, 545}, {270, 528}, {262, 512}, {258, 503}}
local ws = {46, 40, 32, 24, 18, 15, 12, 9, 7, 5, 3.5}
local pts, wd = {}, {}
-- densify
for i=1,#PATHPTS-1 do
  for s=0,9 do
    local t = s/10
    pts[#pts+1] = {lerp(PATHPTS[i][1], PATHPTS[i+1][1], t), lerp(PATHPTS[i][2], PATHPTS[i+1][2], t)}
    wd[#wd+1] = lerp(ws[i], ws[i+1], t) * (0.85 + 0.3*math.random())
  end
end
pts[#pts+1] = PATHPTS[#PATHPTS]; wd[#wd+1] = ws[#ws]
PATHM = ribbon(pts, wd):roughen(2, 10, 51) * below(function(x) return fg(x) + 2 end)
local pp = pile{{"yellow ochre", 1.2}, {"lead white", 0.7}, {"raw umber", 0.5}, {"red earth", 0.15}, {"green earth", 0.3}, medium=0.15}
work(PATHM, {hand="body", tool="filbert 5", pile=pp, angle=function(x, y) return -0.45 end, angle_jitter=0.3, length={10,30}, coverage=2, fill=true, edge="soft", seed=3701})
-- worn ruts, darker, and the lit upper-left edge
local rut = pile{{"raw umber", 1}, {"yellow ochre", 0.6}, {"green earth", 0.4}, {"lead white", 0.2}, medium=0.2}
local b = brush{kind="round", width=2.2, point=0.6}
b:load(rut, 0.5)
local r1, r2 = {}, {}
for i,p in ipairs(pts) do
  local w = wd[i]
  if i % 3 == 1 then
    r1[#r1+1] = {p[1] - 0.18*w, p[2] + 0.05*w}
    r2[#r2+1] = {p[1] + 0.2*w, p[2] + 0.08*w}
  end
end
b:stroke(r1, {pressure={0.5, 0.05}, ramps={0.05, 0.4}, shake=1})
b:load(rut, 0.5)
b:stroke(r2, {pressure={0.55, 0.05}, ramps={0.05, 0.4}, shake=1})

--@ chunk 38
local dk = pile{{"raw umber", 1}, {"green earth", 1.1}, {"yellow ochre", 0.6}, {"bone black", 0.05}, medium=0.45}
work(PATHM, {hand="scumble", tool="filbert 6", pile=dk, angle=-0.4, angle_jitter=0.3, length={10,25}, coverage=1.4, load_at=function(x, y) return clamp((y - 540)/90, 0.05, 0.8) end, clip=true, seed=3801})
local gr = pile{{"green earth", 2}, {"yellow ochre", 0.8}, {"raw umber", 0.6}, {"Prussian blue", 0.05}, medium=0.2}
lose(PATHM, {pile=gr, where=1, tool="filbert 3", reach={4, 5}, load=0.3, seed=3802})
blend(PATHM:grow(3), {angle=-0.45, length={8,20}, coverage=0.8, seed=3803})

--@ chunk 39
print(wait(36*60)); for _,p in ipairs({{300,420},{500,440},{700,455},{50,380},{900,375},{400,590},{300,555}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 40
print(wait(30*60)); for _,p in ipairs({{300,420},{500,440},{700,455},{400,590},{300,555},{480,620}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 41
local litrim = mask(function(x, y)
  if TREES2:at(x, y) < 0.5 then return 0 end
  return (TREES2:at(x - 2.5, y - 2) < 0.5) and 1 or 0 end)
local lp = pile{{"lead white", 2.4}, {"pale smalt", 0.6}, {"yellow ochre", 0.5}, {"green earth", 0.5}, {"red earth", 0.05}, medium=0.1}
stipple(litrim:grow(0.5), {pile=lp, width=1.2, coverage=1.1, cluster=0.5, clip=true, dips={10, 0.4, 0.5}, seed=4101})
-- a few darker crown separations inside the woods
local inner = TREES2:shrink(2.5)
local dp = pile{{"pale smalt", 1.1}, {"smalt", 0.5}, {"raw umber", 0.55}, {"bone black", 0.1}, {"lead white", 0.5}, medium=0.15}
stipple(inner, {pile=dp, width=1.2, coverage=0.35, cluster=0.8, clip=true, dips={12, 0.4, 0.5}, seed=4102})

--@ chunk 42
-- the ditch: a spline through STREAM, densified, width growing toward us
local pts, wd, top = {}, {}, {}
local n = noise{seed=77, octaves=2, period=60}
for i=1,#STREAM-1 do
  for s=0,11 do
    local t = s/12
    local x = lerp(STREAM[i][1], STREAM[i+1][1], t)
    local y = lerp(STREAM[i][2], STREAM[i+1][2], t) + 0.8*n(x, 0)
    pts[#pts+1] = {x, y}
    local w = 1.2 + ((y - 428)/82)^1.3 * 6.5
    wd[#wd+1] = w * (0.85 + 0.3*n:at01(x*3, 50))
  end
end
pts[#pts+1] = STREAM[#STREAM]; wd[#wd+1] = 8
STREAMPTS, STREAMW = pts, wd
-- flatten the ribbon vertically: water seen at a grazing angle
WATER = ribbon(pts, wd):roughen(0.6, 5, 61)
local far_w = pile{{"lead white", 5}, {"yellow ochre", 0.25}, {"chrome yellow", 0.06}, {"pale smalt", 0.5}, medium=0.15}
local near_w = pile{{"lead white", 3.5}, {"pale smalt", 1.2}, {"smalt", 0.5}, {"yellow ochre", 0.05}, medium=0.15}
local wfar = WATER * mask(function(x, y) return x > 520 and 1 or 0 end)
local wnear = WATER - wfar
work(wfar, {hand="detail", tool={kind="round", width=1.4, point=0.5}, pile=far_w, angle=-0.08, coverage=2.5, fill=true, seed=4201})
work(wnear, {hand="detail", tool={kind="round", width=2.2, point=0.5}, pile=near_w, angle=-0.12, coverage=2.5, fill=true, seed=4202})
blend(WATER, {angle=-0.1, length={10,40}, coverage=1, seed=4203})

--@ chunk 43
print(wait(30*60)); for _,p in ipairs({{600,452},{800,441},{200,485},{400,590},{300,555},{480,620},{500,440}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 44
print(wait(30*60)); for _,p in ipairs({{600,452},{800,441},{900,434},{200,485},{400,590},{480,620},{700,455}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 45
print(wait(48*60)); for _,p in ipairs({{600,452},{800,441},{900,434},{200,485},{400,590},{480,620},{270,525}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 46
local keep = rect(505, 0, 16, 700) + rect(762, 0, 9, 700)
local cover = (WATER:grow(1.8) * mask(function(x, y) return x > 336 + 6*math.sin(y) and 1 or 0 end)) - keep
local segs = {
  {336, 480, pile{{"green earth", 2}, {"yellow ochre", 1.3}, {"chrome yellow", 0.18}, {"lead white", 0.95}, {"Prussian blue", 0.03}, medium=0.15}},
  {480, 640, pile{{"green earth", 1.9}, {"yellow ochre", 1.3}, {"chrome yellow", 0.16}, {"lead white", 1.0}, {"Prussian blue", 0.02}, medium=0.15}},
  {640, 820, pile{{"green earth", 1.7}, {"yellow ochre", 1.25}, {"chrome yellow", 0.13}, {"lead white", 1.0}, {"pale smalt", 0.12}, medium=0.15}},
  {820, 1010, pile{{"green earth", 1.6}, {"yellow ochre", 1.2}, {"lead white", 1.0}, {"chrome yellow", 0.1}, {"pale smalt", 0.2}, medium=0.15}},
}
for i,s in ipairs(segs) do
  local m = cover * rect(s[1], 0, s[2]-s[1], 700)
  work(m, {hand="detail", tool={kind="round", width=2.4, point=0.3}, pile=s[3], angle=0, angle_jitter=0.1, coverage=3, fill=true, seed=4600+i})
end
blend(cover:grow(2), {angle=0, length={10,30}, coverage=0.8, seed=4605})

--@ chunk 47
local ws = {30, 26, 20, 15, 11, 9, 7, 5, 3.5, 2.5, 1.6}
local pts, wd = {}, {}
local nn = noise{seed=5, octaves=2, period=30}
for i=1,#PATHPTS-1 do
  for s=0,9 do
    local t = s/10
    local x, y = lerp(PATHPTS[i][1], PATHPTS[i+1][1], t), lerp(PATHPTS[i][2], PATHPTS[i+1][2], t)
    pts[#pts+1] = {x + 1.5*nn(x, y), y}
    wd[#wd+1] = lerp(ws[i], ws[i+1], t) * (0.8 + 0.4*nn:at01(x*2, y))
  end
end
pts[#pts+1] = PATHPTS[#PATHPTS]; wd[#wd+1] = 1.5
NARROW = ribbon(pts, wd):roughen(1.2, 6, 71) * below(function(x) return fg(x) + 1 end)
local edge = PATHM:grow(2.5) - NARROW
local cuts = {0, 12, 35, 70, 400}
local piles = {
  pile{{"green earth", 2}, {"yellow ochre", 1.6}, {"lead white", 0.6}, {"chrome yellow", 0.12}, {"raw umber", 0.1}, medium=0.15},
  pile{{"green earth", 2}, {"yellow ochre", 1.2}, {"lead white", 0.3}, {"raw umber", 0.3}, {"Prussian blue", 0.04}, medium=0.15},
  pile{{"green earth", 2}, {"yellow ochre", 0.9}, {"raw umber", 0.6}, {"Prussian blue", 0.07}, {"bone black", 0.05}, medium=0.15},
  pile{{"green earth", 1.6}, {"raw umber", 1}, {"yellow ochre", 0.6}, {"Prussian blue", 0.1}, {"bone black", 0.12}, medium=0.15},
}
FGP = piles
for i=1,4 do
  local a, b = cuts[i], cuts[i+1]
  local m = edge * mask(function(x, y) local d = y - fg(x); return (d >= a and d < b) and 1 or 0 end)
  work(m, {hand="detail", tool={kind="round", width=3, point=0.3}, pile=piles[i], angle=-0.3, angle_jitter=0.4, coverage=3, fill=true, seed=4700+i})
end
-- darken the remaining track: a thin earth glaze, heavier toward the bottom
local g = pile{{"raw umber", 1}, {"green earth", 0.8}, {"yellow ochre", 0.4}, medium=0.6}
work(NARROW:grow(1), {hand="glaze", tool="filbert 6", pile=g, angle=-0.5, length={20,50}, coverage=1.2, clip=true, load_at=function(x, y) return clamp((y-520)/110, 0.15, 0.7) end, seed=4705})

--@ chunk 48
local ang = function(x, y) return lerp(0.45, 1.1, clamp((600-y)/90, 0, 1)) end
blend(NARROW:grow(0.5), {angle=ang, length={15,40}, coverage=1.5, seed=4801})
local edge = PATHM:grow(8) - NARROW:grow(0.3)
local cuts = {0, 12, 35, 70, 400}
for i=1,4 do
  local a, b = cuts[i], cuts[i+1]
  local m = edge * mask(function(x, y) local d = y - fg(x); return (d >= a and d < b) and 1 or 0 end)
  work(m, {hand="detail", tool={kind="round", width=3.2, point=0.3}, pile=FGP[i], angle=ang, angle_jitter=0.5, coverage=3.5, fill=true, seed=4810+i})
end

--@ chunk 49
blend(ellipse(262, 514, 30, 24):soften(6) - NARROW, {angle=0.2, length={8,20}, coverage=1.2, seed=4901})

--@ chunk 50
LWATER = WATER * mask(function(x, y) return x < 330 and 1 or 0 end)
local wp = pile{{"lead white", 2.6}, {"pale smalt", 1.4}, {"smalt", 0.55}, {"raw umber", 0.05}, {"yellow ochre", 0.05}, medium=0.15}
work(LWATER, {hand="detail", tool={kind="round", width=2.2, point=0.4}, pile=wp, angle=-0.12, coverage=2.5, fill=true, seed=5001})
-- the far bank: a dark shaded lip along the upper edge of the water
local bank = mask(function(x, y) if x > 332 then return 0 end
  local v = LWATER:at(x, y); if v > 0.5 then return 0 end
  return (LWATER:at(x, y + 1.6) > 0.5) and 1 or 0 end):grow(0.5)
local bp = pile{{"raw umber", 1}, {"green earth", 1}, {"bone black", 0.15}, {"yellow ochre", 0.3}, medium=0.15}
work(bank, {hand="detail", tool={kind="round", width=1.2, point=0.6}, pile=bp, angle=-0.12, coverage=2, fill=true, seed=5002})
-- taper the end: grass over the last stretch
local endm = WATER:grow(2) * mask(function(x, y) return x > 300 and 1 or 0 end) * mask(function(x, y) return (x - 300)/36 > rand(0,1)*0 + ((y*7.3) % 1)*0.9 and 1 or 0 end)
local gp = pile{{"green earth", 2}, {"yellow ochre", 1.2}, {"lead white", 0.5}, {"raw umber", 0.2}, {"Prussian blue", 0.04}, medium=0.15}
work(WATER:grow(2) * rect(318, 0, 30, 700), {hand="detail", tool={kind="round", width=1.6, point=0.6}, pile=gp, angle=-0.15, coverage=2.2, fill=true, seed=5003})

--@ chunk 51
local mp = pile{{"pale smalt", 1.3}, {"smalt", 0.5}, {"raw umber", 0.55}, {"lead white", 0.8}, {"red earth", 0.1}, {"bone black", 0.1}, medium=0.1}
local lp = pile{{"lead white", 2.2}, {"pale smalt", 0.8}, {"red earth", 0.15}, {"yellow ochre", 0.35}, {"raw umber", 0.1}, medium=0.1}
MILLS = {{x=150, s=1.0, rot=0.35}, {x=872, s=0.85, rot=0.9}}
local fine = brush{kind="round", width=0.9, point=1, stiffness=0.4}
for _,ml in ipairs(MILLS) do
  local s = ml.s
  local base = far(ml.x) + 1.2
  local post = base - 5*s
  -- trestle legs and post
  fine:load(mp, 0.5)
  fine:stroke({{ml.x - 3*s, base}, {ml.x, post}}, {pressure={0.35,0.3}})
  fine:stroke({{ml.x + 3*s, base}, {ml.x, post}}, {pressure={0.35,0.3}})
  fine:stroke({{ml.x, base}, {ml.x, post}}, {pressure={0.45,0.4}})
  -- body with pitched roof
  local bx0, bx1 = ml.x - 3*s, ml.x + 3*s
  local by0, by1 = post - 7.5*s, post
  local body = poly({{bx0, by1}, {bx0, by0}, {ml.x, by0 - 2.5*s}, {bx1, by0}, {bx1, by1}})
  work(body, {hand="detail", tool={kind="round", width=1.2, point=0.6}, pile=mp, coverage=3, fill=true, seed=5100 + ml.x})
  work(body * rect(bx0, 0, 2*s, 700), {hand="detail", tool={kind="round", width=1, point=0.6}, pile=lp, coverage=2, fill=true, seed=5110 + ml.x})
  -- sails: four arms in an X from the hub
  local hx, hy = ml.x + 0.5*s, by0 + 2*s
  fine:load(mp, 0.6)
  for k=0,3 do
    local a = ml.rot + k*math.pi/2
    local L = 13*s
    local ex, ey = hx + L*math.cos(a), hy + L*math.sin(a)
    fine:stroke({{hx, hy}, {ex, ey}}, {pressure={0.5,0.35}, ramps={0.05,0.1}})
    -- the lattice of the sail: a thin parallel line and a few cross-bars
    local nx, ny = -math.sin(a), math.cos(a)
    local off = 1.6*s
    fine:stroke({{hx + 3*s*math.cos(a) + off*nx, hy + 3*s*math.sin(a) + off*ny}, {ex + off*nx, ey + off*ny}}, {pressure={0.25,0.2}})
    for j=1,4 do
      local t = 0.3 + 0.17*j
      local px, py = hx + L*t*math.cos(a), hy + L*t*math.sin(a)
      fine:stroke({{px, py}, {px + off*nx, py + off*ny}}, {pressure={0.2,0.2}})
    end
    fine:load(mp, 0.5)
  end
end

--@ chunk 52
local hedges = {{200,480,401},{600,760,404},{800,960,407},{0,250,412},{350,700,418},{700,1000,425},{0,180,434},{420,580,436},{620,760,444}}
local nn = noise{seed=9, octaves=3, period=25}
HEDGES = nil
for i,h in ipairs(hedges) do
  local x0, x1, y = h[1], h[2], h[3]
  local t = (y - 398)/50
  local pts, wd = {}, {}
  local slope = rand(-0.01, 0.01)
  for x = x0, x1, 3 do
    pts[#pts+1] = {x, y + slope*(x - x0) + 0.4*nn(x, y)}
    wd[#wd+1] = (0.9 + 1.8*t) * (0.6 + 0.8*nn:at01(x, y*3))
  end
  local m = ribbon(pts, wd)
  -- bushes and small trees along it
  local nb = math.floor((x1 - x0)/22)
  for j=1,nb do
    local bx = rand(x0 + 4, x1 - 4)
    local by = y + slope*(bx - x0)
    local r = (1 + 2.2*t) * rand(0.7, 1.6)
    m = m + ellipse(bx, by - r*0.7, r*1.2, r)
  end
  m = m:roughen(0.5, 3, 90 + i)
  HEDGES = HEDGES and (HEDGES + m) or m
  local hp = pile{{"green earth", 1.2}, {"pale smalt", lerp(1.2, 0.4, t)}, {"raw umber", 0.4}, {"lead white", lerp(1.1, 0.3, t)}, {"Prussian blue", 0.03 + 0.05*t}, {"bone black", 0.03}, medium=0.12}
  work(m, {hand="detail", tool={kind="round", width=1 + t, point=0.5}, pile=hp, angle=0, coverage=2.5, fill=true, seed=5200+i})
end
-- sunlit tops of the hedges, from the left
local lit = mask(function(x, y) if HEDGES:at(x, y) < 0.5 then return 0 end; return (HEDGES:at(x - 1.2, y - 1.4) < 0.5) and 1 or 0 end)
local lp = pile{{"lead white", 1.4}, {"green earth", 1}, {"yellow ochre", 0.8}, {"chrome yellow", 0.1}, medium=0.1}
stipple(lit, {pile=lp, width=1, coverage=1, cluster=0.4, clip=true, seed=5210})

--@ chunk 53
blend(HEDGES:grow(1.5), {angle=0, length={6,16}, coverage=0.9, clip=true, seed=5301})

--@ chunk 54
local function streamy(x)
  for i=1,#STREAMPTS-1 do
    local a, b = STREAMPTS[i], STREAMPTS[i+1]
    if (x <= a[1] and x >= b[1]) then
      local t = (a[1] - x)/(a[1] - b[1]); return lerp(a[2], b[2], t), lerp(STREAMW[i], STREAMW[i+1], t)
    end
  end
  return 510, 8
end
STREAMY = streamy
WILLOWS = {{x=58, h=60}, {x=112, h=52}, {x=168, h=46}, {x=226, h=40}, {x=281, h=34}}
local trunkp = pile{{"raw umber", 1}, {"bone black", 0.2}, {"lead white", 0.3}, {"yellow ochre", 0.2}, medium=0.1}
local trunkl = pile{{"lead white", 1}, {"raw umber", 0.6}, {"yellow ochre", 0.5}, {"pale smalt", 0.2}, medium=0.1}
local darkf = pile{{"green earth", 1.5}, {"raw umber", 0.55}, {"Prussian blue", 0.05}, {"bone black", 0.04}, {"lead white", 0.25}, medium=0.1}
local midf = pile{{"lead white", 1.1}, {"green earth", 1.5}, {"pale smalt", 0.5}, {"yellow ochre", 0.35}, {"raw umber", 0.2}, medium=0.1}
local litf = pile{{"lead white", 2}, {"green earth", 0.7}, {"yellow ochre", 0.5}, {"pale smalt", 0.2}, medium=0.1}
WILLOWM = nil
for wi, w in ipairs(WILLOWS) do
  local sy, sw = streamy(w.x)
  w.y = sy - sw/2 - 0.3
  local s = w.h/60
  local th = w.h*0.36
  local tw = 7*s
  local hx, hy = w.x + rand(-1.5, 1.5), w.y - th
  w.hx, w.hy = hx, hy
  -- trunk: stout, slightly leaning, with a swollen head
  local trunk = poly({{w.x - tw/2 - 1*s, w.y + 0.5}, {w.x - tw/2 + 0.6*s, w.y - th*0.5}, {hx - tw/2 - 0.5*s, hy + 1}, {hx - tw*0.8, hy - 1.5*s}, {hx, hy - 3*s}, {hx + tw*0.8, hy - 1.5*s}, {hx + tw/2 + 0.5*s, hy + 1}, {w.x + tw/2 - 0.3*s, w.y - th*0.5}, {w.x + tw/2 + 1.2*s, w.y + 0.5}}, true)
  -- the crown: a fan of shoots, clumped foliage around them
  local shoots = {}
  local crown = nil
  local ns = 16
  for k=1,ns do
    local a = -math.pi/2 + lerp(-1.05, 1.05, (k-0.5)/ns) + rand(-0.08, 0.08)
    local L = w.h*0.66 * (0.75 + 0.3*math.cos(a + math.pi/2)) * rand(0.85, 1.1)
    local bend = rand(-0.15, 0.15)
    local pts = {}
    for j=0,6 do
      local t = j/6
      local aa = a + bend*t + 0.25*t*(math.cos(a) > 0 and 1 or -1)*math.abs(math.cos(a))
      pts[#pts+1] = {hx + L*t*math.cos(aa), hy - 2*s + L*t*math.sin(aa)}
    end
    shoots[#shoots+1] = pts
    local fol = nil
    for j=2,7 do
      local p = pts[j]
      local r = (1.2 + 3.2*s) * rand(0.7, 1.3) * (0.6 + 0.4*math.sin(math.pi*(j-1)/6))
      local e = ellipse(p[1] + rand(-1,1), p[2] + rand(-1,1), r*1.1, r)
      fol = fol and (fol + e) or e
    end
    crown = crown and (crown + fol) or fol
  end
  crown = crown:roughen(1.2*s + 0.5, 4, 700 + wi)
  w.crown, w.trunk, w.shoots = crown, trunk, shoots
  local wm = crown + trunk
  WILLOWM = WILLOWM and (WILLOWM + wm) or wm
  work(crown, {hand="detail", tool={kind="round", width=1.6, point=0.5}, pile=midf, coverage=2.4, fill=true, seed=5400+wi})
  -- shadowed right and lower parts of the crown
  local shade = crown * mask(function(x, y) return ((x - hx)*0.8 + (y - (hy - w.h*0.3))*0.6 > rand(-2, 2)*0 + 2*s) and 1 or 0 end)
  stipple(shade, {pile=darkf, width=1.6*s + 0.6, coverage=1.6, cluster=0.6, clip=true, seed=5410+wi})
  stipple(crown, {pile=darkf, width=1.2, coverage=0.3, cluster=0.8, clip=true, seed=5420+wi})
  work(trunk, {hand="detail", tool={kind="round", width=1.4, point=0.5}, pile=trunkp, angle=math.pi/2, coverage=3, fill=true, seed=5430+wi})
  work(trunk * rect(w.x - 10, 0, 10 - tw*0.1, 700), {hand="detail", tool={kind="round", width=1, point=0.5}, pile=trunkl, angle=math.pi/2, coverage=1.3, seed=5440+wi})
end

--@ chunk 55
print(wait(40*60)); for _,p in ipairs({{100,470},{58,480},{200,480},{300,430},{700,425},{150,500},{150,380}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 56
local det = function(m, p, w, seed, ang) work(m, {hand="detail", tool={kind="round", width=w or 1.4, point=0.5}, pile=p, angle=ang or math.pi/2, coverage=3, fill=true, seed=seed}) end
-- cast shadows falling right along the slope
local shp = pile{{"green earth", 1.4}, {"raw umber", 1}, {"Prussian blue", 0.08}, {"bone black", 0.1}, medium=0.35}
local s1 = poly({{316, 566}, {330, 566}, {400, 571}, {428, 570}, {402, 568}}, true)
local s2 = poly({{334, 567}, {354, 567}, {430, 574}, {452, 573}, {426, 570}}, true)
SHADOWS = (s1 + s2):soften(1)
det(SHADOWS, shp, 2.2, 5601, 0.05)
-- the man
local legs = poly({{315.5, 545}, {321, 545}, {320.5, 563}, {316.5, 563}}) + poly({{323, 545}, {328.5, 545}, {327.5, 563}, {323.5, 563}})
local boots = ellipse(318.2, 564.6, 2.8, 1.9) + ellipse(325.8, 564.8, 2.8, 1.9)
local coat = poly({{314, 503}, {318, 500.5}, {326, 500.5}, {330.5, 503}, {332, 510}, {331.5, 524}, {330, 530}, {333.5, 546}, {321.5, 548}, {310.8, 546}, {314, 530}, {312.8, 524}, {312.5, 510}}, true)
local collar = poly({{316.5, 501.5}, {322, 499}, {327.5, 501.5}, {326, 504}, {318, 504}}, true)
local head = ellipse(322.2, 495, 3.4, 4.6)
local hat = ellipse(322.4, 490.8, 5.2, 2.1) + ellipse(322.2, 489.6, 4.2, 2.2)
local stick = ribbon({{331.5, 528}, {333.6, 547}, {335.4, 566}}, {1.0, 0.9, 0.8})
MAN = legs + boots + coat + collar + head + hat + stick
local coatp = pile{{"bone black", 0.8}, {"Prussian blue", 0.12}, {"raw umber", 0.6}, {"green earth", 0.4}, {"lead white", 0.12}, medium=0.08}
local trou = pile{{"raw umber", 0.6}, {"lead white", 0.55}, {"yellow ochre", 0.25}, {"bone black", 0.05}, medium=0.08}
local blk = pile{{"bone black", 1}, {"raw umber", 0.4}, medium=0.08}
local hair = pile{{"raw umber", 1}, {"bone black", 0.25}, {"red earth", 0.1}, medium=0.08}
det(legs, trou, 1.3, 5602)
det(boots, blk, 1.2, 5603, 0)
det(coat, coatp, 1.6, 5604)
det(head, hair, 1.2, 5605)
det(hat, blk, 1.2, 5606, 0)
det(stick, pile{{"raw umber", 1}, {"red earth", 0.2}, {"lead white", 0.2}, medium=0.08}, 0.8, 5607)
-- the woman
local skirt = poly({{336, 509}, {348.5, 509}, {350.5, 516}, {352.5, 540}, {355.5, 566.5}, {343, 568}, {331.5, 566.5}, {333.5, 540}, {334.5, 516}}, true)
local shawl = poly({{335, 509.5}, {339, 506.5}, {346, 506.5}, {350, 509.5}, {351, 515}, {346, 522}, {342.5, 530}, {339, 522}, {334.2, 515}}, true)
local whead = ellipse(342.4, 503, 3.1, 4.2)
local bun = ellipse(342.2, 498.6, 2.4, 1.9)
WOMAN = skirt + shawl + whead + bun
local dress = pile{{"red earth", 1}, {"vermilion", 0.25}, {"raw umber", 0.45}, {"bone black", 0.1}, medium=0.08}
local cream = pile{{"lead white", 2}, {"yellow ochre", 0.25}, {"raw umber", 0.08}, medium=0.08}
local whair = pile{{"raw umber", 1}, {"red earth", 0.35}, {"yellow ochre", 0.15}, medium=0.08}
det(skirt, dress, 1.6, 5610)
det(whead + bun, whair, 1.1, 5611)
det(shawl, cream, 1.3, 5612)

--@ chunk 57
function horse_mask(x, yf, H, f, grazing)
  local m = ellipse(x, yf - 0.68*H, 0.55*H, 0.22*H)
  m = m + ellipse(x + f*0.42*H, yf - 0.68*H, 0.2*H, 0.24*H) + ellipse(x - f*0.42*H, yf - 0.7*H, 0.22*H, 0.25*H)
  if grazing then
    m = m + ribbon({{x + f*0.5*H, yf - 0.78*H}, {x + f*0.78*H, yf - 0.42*H}, {x + f*0.9*H, yf - 0.12*H}}, {0.24*H, 0.13*H, 0.1*H})
    m = m + ellipse(x + f*0.93*H, yf - 0.1*H, 0.08*H, 0.12*H)
  else
    m = m + ribbon({{x + f*0.5*H, yf - 0.8*H}, {x + f*0.72*H, yf - 1.08*H}}, {0.26*H, 0.14*H})
    m = m + ribbon({{x + f*0.7*H, yf - 1.12*H}, {x + f*0.98*H, yf - 0.9*H}}, {0.13*H, 0.08*H})
  end
  local lw = 0.075*H
  for _,lx in ipairs({0.46, 0.34}) do
    m = m + ribbon({{x + f*lx*H, yf - 0.55*H}, {x + f*(lx+0.02)*H, yf - 0.25*H}, {x + f*(lx+0.01)*H, yf}}, {lw*1.3, lw, lw*0.9})
  end
  for _,lx in ipairs({-0.42, -0.52}) do
    m = m + ribbon({{x + f*lx*H, yf - 0.6*H}, {x + f*(lx-0.06)*H, yf - 0.3*H}, {x + f*(lx-0.02)*H, yf}}, {lw*1.8, lw, lw*0.9})
  end
  m = m + ribbon({{x - f*0.64*H, yf - 0.82*H}, {x - f*0.72*H, yf - 0.6*H}, {x - f*0.7*H, yf - 0.36*H}}, {0.06*H, 0.09*H, 0.05*H})
  return m
end
local bay = pile{{"red earth", 1}, {"raw umber", 0.9}, {"bone black", 0.1}, {"yellow ochre", 0.2}, medium=0.08}
local dark = pile{{"raw umber", 1}, {"bone black", 0.45}, {"red earth", 0.2}, medium=0.08}
local gray = pile{{"lead white", 1.6}, {"raw umber", 0.4}, {"pale smalt", 0.3}, {"yellow ochre", 0.2}, medium=0.08}
local chest = pile{{"red earth", 0.8}, {"yellow ochre", 0.7}, {"raw umber", 0.4}, {"lead white", 0.2}, medium=0.08}
HORSES = {
  {x=452, y=476, H=13, f=1, g=true, p=bay},
  {x=487, y=472, H=12, f=-1, g=false, p=gray},
  {x=522, y=478, H=13.5, f=1, g=true, p=dark},
  {x=556, y=471, H=11, f=1, g=true, p=chest},
}
HORSEM = nil
for i,h in ipairs(HORSES) do
  local m = horse_mask(h.x, h.y, h.H, h.f, h.g)
  h.m = m
  HORSEM = HORSEM and (HORSEM + m) or m
  work(m, {hand="detail", tool={kind="round", width=0.9, point=0.6}, pile=h.p, angle=0, coverage=3, fill=true, seed=5700+i})
end

--@ chunk 58
local g1 = WATER:grow(2.2) * rect(500, 0, 26, 700)
local g2 = WATER:grow(2.2) * rect(757, 0, 19, 700)
local p1 = pile{{"green earth", 1.9}, {"yellow ochre", 1.3}, {"chrome yellow", 0.16}, {"lead white", 1.0}, {"Prussian blue", 0.02}, medium=0.15}
local p2 = pile{{"green earth", 1.7}, {"yellow ochre", 1.25}, {"chrome yellow", 0.13}, {"lead white", 1.0}, {"pale smalt", 0.12}, medium=0.15}
work(g1, {hand="detail", tool={kind="round", width=2, point=0.3}, pile=p1, angle=0, coverage=3.5, fill=true, seed=5801})
work(g2, {hand="detail", tool={kind="round", width=2, point=0.3}, pile=p2, angle=0, coverage=3.5, fill=true, seed=5802})

--@ chunk 59
local hay = pile{{"yellow ochre", 1.4}, {"lead white", 0.8}, {"raw umber", 0.25}, {"green earth", 0.15}, medium=0.08}
local haylit = pile{{"lead white", 1.5}, {"yellow ochre", 1}, {"chrome yellow", 0.1}, {"red earth", 0.05}, medium=0.08}
local hayshade = pile{{"raw umber", 0.8}, {"yellow ochre", 0.6}, {"green earth", 0.4}, {"pale smalt", 0.2}, {"lead white", 0.2}, medium=0.1}
local cast = pile{{"green earth", 1.3}, {"raw umber", 0.7}, {"Prussian blue", 0.06}, {"yellow ochre", 0.3}, medium=0.3}
HAY = {}
local spots = {{712,449},{748,452},{786,447},{822,455},{858,450},{905,457},{945,452},{770,462},{840,468},{690,462},{960,466},{620,455}}
for i,s in ipairs(spots) do
  local x, y = s[1], s[2]
  local sc = (y - 392)/60
  local H = 7.5*sc*rand(0.9, 1.1)
  local Wd = H*1.25
  local m = poly({{x - Wd/2, y}, {x - Wd*0.42, y - H*0.55}, {x - Wd*0.2, y - H*0.92}, {x, y - H}, {x + Wd*0.2, y - H*0.92}, {x + Wd*0.42, y - H*0.55}, {x + Wd/2, y}}, true):roughen(0.35, 2, 100 + i)
  HAY[#HAY+1] = {x=x, y=y, H=H, W=Wd, m=m}
  local sh = poly({{x, y - 0.3}, {x + Wd*0.5, y - 0.8}, {x + Wd*1.6, y - 0.2}, {x + Wd*1.5, y + 0.9}, {x, y + 0.9}}, true)
  work(sh, {hand="detail", tool={kind="round", width=0.9, point=0.6}, pile=cast, angle=0, coverage=2, fill=true, seed=5900+i})
  work(m, {hand="detail", tool={kind="round", width=0.9, point=0.6}, pile=hay, angle=math.pi/2, coverage=3, fill=true, seed=5920+i})
  work(m * rect(x + Wd*0.08, 0, Wd, 700), {hand="detail", tool={kind="round", width=0.8, point=0.6}, pile=hayshade, angle=math.pi/2, coverage=1.6, fill=true, seed=5940+i})
  work(m * rect(x - Wd, 0, Wd*0.72, 700) * rect(0, y - H, 2000, H*0.75), {hand="detail", tool={kind="round", width=0.7, point=0.6}, pile=haylit, angle=math.pi/2 + 0.3, coverage=1.3, seed=5960+i})
end

--@ chunk 60
print(wait(48*60)); for _,p in ipairs({{58,470},{168,470},{322,520},{342,540},{452,470},{712,446},{100,500}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 61
local s1 = body.ellipsoid({815, 606, 0}, {66, 38, 44}):turn({815, 606, 0}, 0, 0, -0.08):rough(4, 50, 3)
local s2 = body.ellipsoid({905, 622, 0}, {24, 14, 18}):rough(2, 30, 4)
local s3 = body.ellipsoid({735, 626, 0}, {15, 9, 12}):rough(1.5, 20, 5)
local s4 = body.ellipsoid({960, 612, 0}, {10, 6, 8}):rough(1, 15, 6)
BF = form{{s1}, {s2}, {s3}, {s4}, light={from={-1, -0.55}, front=0.45, ambient=0.18}}
local ground = function(x) return 620 + 0.06*(x - 815) + 3*math.sin(x/17) end
local cut = below(function(x) return 628 + 3*math.sin(x/13) end)
local sil = BF:silhouette{parts={1,2,3,4}} - cut - below(function(x) return x < 850 and 618 + 3*math.sin(x/11) or 634 end)
BOULD = sil
-- cast shadow on the grass to the right
local cs = pile{{"green earth", 1.2}, {"raw umber", 0.9}, {"Prussian blue", 0.1}, {"bone black", 0.1}, medium=0.3}
local shad = (poly({{840, 612}, {900, 590}, {960, 596}, {1010, 610}, {1010, 632}, {860, 630}}, true) + poly({{920, 626}, {950, 618}, {990, 624}, {960, 632}}, true)) - sil
work(shad:soften(2), {hand="body", tool="filbert 5", pile=cs, angle=0.05, length={15,40}, coverage=1.8, fill=true, edge="soft", seed=6101})
local dk = pile{{"raw umber", 0.6}, {"pale smalt", 0.8}, {"bone black", 0.15}, {"lead white", 0.4}, medium=0.08}
local md = pile{{"lead white", 1}, {"raw umber", 0.5}, {"pale smalt", 0.45}, {"yellow ochre", 0.2}, medium=0.08}
local lt = pile{{"lead white", 2}, {"yellow ochre", 0.4}, {"raw umber", 0.2}, {"red earth", 0.05}, medium=0.08}
work(sil, {hand="body", tool="filbert 4", pile=dk, angle=BF:field("across"), length={6,16}, coverage=2.5, fill=true, clip=true, seed=6102})
local litm = BF:lit{parts={1,2,3,4}, soft=0.15} * sil
work(litm, {hand="body", tool="filbert 3", pile=md, angle=BF:field("across"), length={5,14}, coverage=2.2, fill=true, clip=true, seed=6103})
local hi = mask(function(x, y) return (BF:value(x, y) > 0.72) and 1 or 0 end) * sil
work(hi, {hand="detail", tool={kind="round", width=2, point=0.4}, pile=lt, angle=BF:field("across"), coverage=1.8, fill=true, seed=6104})
print(BF:value(790, 590), BF:value(850, 610), sil:area())

--@ chunk 62
print(wait(40*60)); for _,p in ipairs({{58,470},{168,470},{322,520},{342,540},{452,470},{712,446},{800,600},{860,610}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 63
local mid2 = pile{{"green earth", 1.5}, {"raw umber", 0.35}, {"pale smalt", 0.45}, {"lead white", 0.5}, {"yellow ochre", 0.2}, {"Prussian blue", 0.03}, medium=0.12}
local dark2 = pile{{"green earth", 1.3}, {"raw umber", 0.7}, {"Prussian blue", 0.07}, {"bone black", 0.08}, {"lead white", 0.1}, medium=0.12}
local lit2 = pile{{"lead white", 2.2}, {"green earth", 0.6}, {"yellow ochre", 0.45}, {"pale smalt", 0.25}, medium=0.1}
local shootp = pile{{"raw umber", 0.8}, {"green earth", 0.6}, {"lead white", 0.3}, {"bone black", 0.08}, medium=0.1}
local fine = brush{kind="round", width=0.7, point=1, stiffness=0.4}
for wi, w in ipairs(WILLOWS) do
  local s = w.h/60
  stipple(w.crown, {pile=mid2, width=1.5*s + 0.6, coverage=1.3, cluster=0.55, clip=true, dips={16, 0.5, 0.5}, seed=6300+wi})
  local shade = w.crown * mask(function(x, y) return ((x - w.hx)*0.75 + (y - (w.hy - w.h*0.3))*0.65 > 1*s) and 1 or 0 end)
  stipple(shade, {pile=dark2, width=1.4*s + 0.6, coverage=1.4, cluster=0.6, clip=true, dips={16, 0.5, 0.5}, seed=6320+wi})
  -- inner hollows
  stipple(w.crown:shrink(2.5*s), {pile=dark2, width=1.2*s + 0.5, coverage=0.35, cluster=0.85, clip=true, seed=6340+wi})
  -- silvery lights on the upper-left of the leaf clumps
  local litm = mask(function(x, y) if w.crown:at(x, y) < 0.5 then return 0 end; return (w.crown:at(x - 3*s, y - 3*s) < 0.5 or ((x - w.hx) < -2 and (y < w.hy - w.h*0.35))) and 1 or 0 end)
  stipple(litm, {pile=lit2, width=1.1*s + 0.5, coverage=0.9, cluster=0.5, clip=true, feather=0.3, seed=6360+wi})
  -- the rods: fine shoots rising from the head, seen at the crown edge
  fine:load(shootp, 0.4)
  for k, pts in ipairs(w.shoots) do
    if k % 2 == 0 then
      local sub = {}
      for j=1,#pts do sub[#sub+1] = pts[j] end
      fine:stroke(sub, {pressure={0.4, 0.02}, ramps={0.05, 0.6}, shake=0.5})
      if k % 6 == 0 then fine:load(shootp, 0.4) end
    end
  end
end

--@ chunk 64
for _,p in ipairs({{100,500},{200,484},{300,478},{58,475}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 65
-- water: a thin veil to lower its value and cool it, heavier toward the near left
local veil = pile{{"lead white", 1.4}, {"pale smalt", 1.1}, {"smalt", 0.5}, {"raw umber", 0.12}, medium=0.4}
work(LWATER, {hand="detail", tool={kind="round", width=2, point=0.3}, pile=veil, angle=-0.1, coverage=1.6, fill=true, seed=6501})
-- reflections of the trunks, dark, broken by ripples
local refl = pile{{"raw umber", 0.8}, {"green earth", 0.7}, {"bone black", 0.15}, {"pale smalt", 0.4}, {"lead white", 0.2}, medium=0.12}
local rb = brush{kind="round", width=1.3, point=0.5}
for wi, w in ipairs(WILLOWS) do
  local sy, sw = STREAMY(w.x)
  local s = w.h/60
  rb:load(refl, 0.6)
  local x0 = w.x - 3.5*s
  local x1 = w.x + 3.5*s
  local y = sy - sw/2 + 0.2
  while y < sy + sw/2 do
    local jit = rand(-0.8, 0.8)
    rb:stroke({{x0 + jit, y}, {x1 + jit + rand(-0.5, 1), y + rand(-0.1, 0.1)}}, {pressure={0.4, 0.3}, clip=LWATER})
    y = y + rand(0.6, 1.1)
  end
  -- the crown's reflection just tinges the water to the right of each trunk
  rb:load(refl, 0.25)
  rb:stroke({{w.x + 4*s, sy + sw*0.2}, {w.x + 12*s, sy + sw*0.25}}, {pressure={0.3, 0.1}, clip=LWATER})
end
-- trunks: grayer, darker bark, a knobbly head
local bark = pile{{"raw umber", 0.9}, {"bone black", 0.25}, {"lead white", 0.35}, {"pale smalt", 0.2}, medium=0.1}
local barkl = pile{{"lead white", 1.2}, {"raw umber", 0.5}, {"yellow ochre", 0.35}, {"pale smalt", 0.15}, medium=0.1}
for wi, w in ipairs(WILLOWS) do
  local s = w.h/60
  work(w.trunk - w.crown, {hand="detail", tool={kind="round", width=1.1, point=0.6}, pile=bark, angle=math.pi/2, coverage=2.2, fill=true, seed=6510+wi})
  local lx = w.x - 3.5*s
  work((w.trunk - w.crown) * rect(lx - 3, 0, 3.2*s + 1, 700), {hand="detail", tool={kind="round", width=0.8, point=0.6}, pile=barkl, angle=math.pi/2, coverage=1.3, seed=6520+wi})
  -- knobs at the head
  local knob = ellipse(w.hx - 2.5*s, w.hy - 1.5*s, 2.6*s, 2*s) + ellipse(w.hx + 2.8*s, w.hy - 1.8*s, 2.4*s, 1.9*s)
  work(knob, {hand="detail", tool={kind="round", width=0.9, point=0.6}, pile=bark, angle=0, coverage=2, fill=true, seed=6530+wi})
end

--@ chunk 66
local protect = (MAN + WOMAN):grow(1.5) + BOULD:grow(1) + NARROW:shrink(1)
local fgz = below(function(x) return fg(x) + 3 end) - protect
local nz = noise{seed=31, octaves=3, period=60, stretch={0, 2.5}}
local dk = pile{{"green earth", 1.6}, {"raw umber", 0.8}, {"Prussian blue", 0.08}, {"yellow ochre", 0.4}, {"bone black", 0.05}, medium=0.15}
local lt = pile{{"green earth", 1.5}, {"yellow ochre", 1.3}, {"chrome yellow", 0.12}, {"lead white", 0.4}, medium=0.15}
stipple(fgz, {pile=dk, width=2.6, coverage=function(x, y) return 0.9*clamp(nz:at01(x, y)*1.6 - 0.5, 0, 1) end, cluster=0.5, drag={1.5, -0.2}, dips={20, 0.5, 0.5}, seed=6601})
stipple(fgz * mask(function(x, y) return (y - fg(x) < 60) and 1 or 0 end), {pile=lt, width=2.2, coverage=function(x, y) return 0.7*clamp(0.9 - nz:at01(x + 300, y)*1.4, 0, 1) end, cluster=0.5, drag={1.5, -0.2}, dips={20, 0.5, 0.5}, seed=6602})

--@ chunk 67
print(wait(36*60)); for _,p in ipairs({{322,520},{342,540},{712,446},{800,600},{860,610},{905,620},{58,475},{500,560}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 68
local sil = BOULD
local shade = sil - BF:lit{parts={1,2,3,4}, soft=0.1}
local deep = pile{{"raw umber", 1}, {"smalt", 0.5}, {"bone black", 0.3}, {"lead white", 0.15}, medium=0.2}
local midg = pile{{"raw umber", 0.7}, {"pale smalt", 0.7}, {"lead white", 0.6}, {"bone black", 0.08}, {"yellow ochre", 0.15}, medium=0.15}
-- overall mid-tone over everything but the brightest crown
local notop = sil * mask(function(x, y) return (BF:value(x, y) < 0.66) and 1 or 0 end)
work(notop, {hand="scumble", tool="filbert 3", pile=midg, angle=BF:field("across"), length={5,12}, coverage=1.6, clip=true, seed=6801})
work(shade, {hand="body", tool="filbert 3", pile=deep, angle=BF:field("across"), length={5,14}, coverage=2, clip=true, fill=true, seed=6802})
-- granite speckle and lichen
local spk = pile{{"bone black", 0.6}, {"raw umber", 0.6}, {"lead white", 0.2}, medium=0.1}
local wspk = pile{{"lead white", 2}, {"yellow ochre", 0.15}, medium=0.1}
local lich = pile{{"yellow ochre", 1}, {"lead white", 0.8}, {"green earth", 0.5}, {"chrome yellow", 0.08}, medium=0.1}
stipple(sil:shrink(1), {pile=spk, width=0.9, coverage=0.25, cluster=0.3, seed=6803})
stipple(BF:lit{parts={1,2,3,4}, soft=0.1} * sil:shrink(1.5), {pile=wspk, width=0.8, coverage=0.2, cluster=0.3, seed=6804})
local ln = noise{seed=44, octaves=3, period=14}
stipple(sil:shrink(2) * mask(function(x, y) return (ln(x, y) > 0.3) and 1 or 0 end), {pile=lich, width=1.1, coverage=0.8, cluster=0.6, seed=6805})
-- a crack across the big stone
local cr = brush{kind="round", width=0.8, point=1}
cr:load(pile{{"bone black", 0.8}, {"raw umber", 0.5}, medium=0.1}, 0.4)
cr:stroke({{790, 578}, {796, 590}, {793, 602}, {801, 614}}, {pressure={0.5, 0.1}, shake=1})
cr:stroke({{825, 572}, {829, 580}}, {pressure={0.4, 0.05}, shake=1})

--@ chunk 69
print(wait(48*60)); for _,p in ipairs({{322,520},{342,540},{712,446},{800,600},{830,590},{905,620},{58,475}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 70
local det = function(m, p, w, seed, cov) work(m, {hand="detail", tool={kind="round", width=w or 1, point=0.6}, pile=p, angle=math.pi/2, coverage=cov or 2.5, fill=true, seed=seed}) end
-- shawl, clean
local shawl = poly({{335, 509.5}, {339, 506.5}, {346, 506.5}, {350, 509.5}, {351, 515}, {346, 522}, {342.5, 530}, {339, 522}, {334.2, 515}}, true)
local cream = pile{{"lead white", 2.2}, {"yellow ochre", 0.22}, {"raw umber", 0.06}, medium=0.08}
local creamsh = pile{{"lead white", 1.4}, {"raw umber", 0.3}, {"pale smalt", 0.35}, {"yellow ochre", 0.1}, medium=0.08}
det(shawl, cream, 1.1, 7001, 3)
det(shawl * rect(343, 0, 20, 700), creamsh, 0.9, 7002, 1.8)
-- a fringe edge along the shawl's point
local fr = brush{kind="round", width=0.5, point=1}
fr:load(creamsh, 0.4)
for i=0,8 do local t = i/8; local x = lerp(335.5, 342.5, t); local y = lerp(516, 530, t); fr:stroke({{x, y}, {x + 0.2, y + 1.5}}, {pressure={0.3, 0.05}}) end
for i=0,8 do local t = i/8; local x = lerp(342.5, 350.5, t); local y = lerp(530, 516, t); fr:stroke({{x, y}, {x + 0.2, y + 1.5}}, {pressure={0.3, 0.05}}) end
-- dress: shade on the right, folds, lit left edge
local skirt = poly({{336, 509}, {348.5, 509}, {350.5, 516}, {352.5, 540}, {355.5, 566.5}, {343, 568}, {331.5, 566.5}, {333.5, 540}, {334.5, 516}}, true)
local dsh = pile{{"red earth", 0.8}, {"raw umber", 0.7}, {"bone black", 0.25}, {"vermilion", 0.05}, medium=0.2}
local dlt = pile{{"red earth", 0.8}, {"vermilion", 0.35}, {"yellow ochre", 0.3}, {"lead white", 0.15}, medium=0.1}
det(skirt * poly({{346, 505}, {360, 505}, {360, 570}, {349, 570}}), dsh, 1.2, 7003, 2)
local fb = brush{kind="round", width=0.8, point=1}
fb:load(dsh, 0.5)
for _,f in ipairs({{341, 532, 339.5, 566}, {345.5, 534, 346.5, 567}, {349, 540, 351.5, 566}, {337.5, 540, 335, 566}}) do
  fb:stroke({{f[1], f[2]}, {(f[1]+f[3])/2 + 0.3, (f[2]+f[4])/2}, {f[3], f[4]}}, {pressure={0.05, 0.5}, ramps={0.4, 0.1}})
end
det(skirt * poly({{325, 505}, {336.8, 505}, {334, 540}, {331.8, 570}, {325, 570}}), dlt, 0.8, 7004, 1.5)
-- the man: warm light along his left contour, sleeve on the right, shading
local coat = poly({{314, 503}, {318, 500.5}, {326, 500.5}, {330.5, 503}, {332, 510}, {331.5, 524}, {330, 530}, {333.5, 546}, {321.5, 548}, {310.8, 546}, {314, 530}, {312.8, 524}, {312.5, 510}}, true)
local coatl = pile{{"bone black", 0.5}, {"Prussian blue", 0.1}, {"raw umber", 0.6}, {"lead white", 0.45}, {"yellow ochre", 0.25}, medium=0.1}
local coatd = pile{{"bone black", 1}, {"Prussian blue", 0.15}, {"raw umber", 0.4}, medium=0.1}
det((coat - coat:offset(0):shrink(1.4)) * rect(300, 0, 16.5, 700), coatl, 0.7, 7005, 1.6)
local sleeve = poly({{328.5, 503.5}, {332, 507}, {333.2, 518}, {331.8, 527}, {329.4, 527}, {330, 518}, {328.4, 508}}, true)
det(sleeve, coatd, 0.8, 7006, 2.5)
local sl = brush{kind="round", width=0.6, point=1}
sl:load(coatl, 0.4)
sl:stroke({{317, 503}, {315.5, 515}, {316.5, 527}}, {pressure={0.3, 0.1}})
sl:stroke({{318.5, 530}, {317.8, 544}}, {pressure={0.25, 0.05}})
-- hat brim catching light, and the collar edge
sl:load(pile{{"bone black", 0.6}, {"raw umber", 0.5}, {"lead white", 0.5}, medium=0.1}, 0.4)
sl:stroke({{317.4, 491.5}, {320, 492.3}, {324, 492.3}}, {pressure={0.35, 0.1}})
-- right trouser leg in shade
det(poly({{323, 545}, {328.5, 545}, {327.5, 563}, {323.5, 563}}) * rect(325.6, 0, 5, 700), pile{{"raw umber", 0.8}, {"lead white", 0.35}, {"bone black", 0.1}, medium=0.1}, 0.7, 7007, 1.6)

--@ chunk 71
for _,p in ipairs({{880,462},{850,462},{452,470},{520,470},{905,452}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 72
local det = function(m, p, w, seed, cov, ang) work(m, {hand="detail", tool={kind="round", width=w or 0.8, point=0.6}, pile=p, angle=ang or 0, coverage=cov or 3, fill=true, seed=seed}) end
local gx, gy = 884, 470
local hay = pile{{"yellow ochre", 1.4}, {"lead white", 0.8}, {"raw umber", 0.25}, {"green earth", 0.15}, medium=0.08}
local haylit = pile{{"lead white", 1.5}, {"yellow ochre", 1}, {"chrome yellow", 0.1}, {"red earth", 0.05}, medium=0.08}
local hayshade = pile{{"raw umber", 0.8}, {"yellow ochre", 0.6}, {"green earth", 0.4}, {"pale smalt", 0.2}, {"lead white", 0.2}, medium=0.1}
local wood = pile{{"raw umber", 1}, {"bone black", 0.3}, {"red earth", 0.1}, {"lead white", 0.1}, medium=0.08}
local cast = pile{{"green earth", 1.3}, {"raw umber", 0.7}, {"Prussian blue", 0.06}, {"yellow ochre", 0.3}, medium=0.3}
-- shadow to the right
det(poly({{gx - 10, gy - 0.5}, {gx + 12, gy - 1.5}, {gx + 40, gy - 0.5}, {gx + 38, gy + 1.2}, {gx - 10, gy + 1.2}}, true), cast, 1, 7201, 2)
-- load of hay on the wagon
local load = poly({{gx - 13, gy - 5}, {gx - 14, gy - 10}, {gx - 10, gy - 15.5}, {gx + 9, gy - 16}, {gx + 13.5, gy - 11}, {gx + 12.5, gy - 5}}, true):roughen(0.4, 2, 7)
det(load, hay, 0.9, 7202, 3, math.pi/2)
det(load * rect(gx + 2, 0, 20, 700), hayshade, 0.8, 7203, 1.6, math.pi/2)
det(load * rect(gx - 20, gy - 17, 16, 7), haylit, 0.7, 7204, 1.4, math.pi/2)
-- wagon bed, ladder side, wheels
local wb = brush{kind="round", width=0.6, point=1}
wb:load(wood, 0.5)
wb:stroke({{gx - 13, gy - 4.6}, {gx + 12.5, gy - 4.6}}, {pressure={0.6, 0.6}})
wb:stroke({{gx - 12.5, gy - 7.5}, {gx + 12, gy - 7.8}}, {pressure={0.35, 0.3}})
for i=0,6 do local x = gx - 12 + i*4; wb:stroke({{x, gy - 4.6}, {x + 0.3, gy - 8}}, {pressure={0.3, 0.3}}) end
for _,wx in ipairs({gx - 8, gx + 7.5}) do
  local pts = {}
  for k=0,16 do local a = k/16*2*math.pi; pts[#pts+1] = {wx + 2.9*math.cos(a), gy - 2.9 + 2.9*math.sin(a)} end
  wb:stroke(pts, {pressure={0.45, 0.45}})
  wb:stroke({{wx - 2.5, gy - 2.9}, {wx + 2.5, gy - 2.9}}, {pressure={0.2, 0.2}})
  wb:stroke({{wx, gy - 5.4}, {wx, gy - 0.4}}, {pressure={0.2, 0.2}})
end
-- the pole
wb:stroke({{gx - 13, gy - 3.5}, {gx - 24, gy - 5}}, {pressure={0.35, 0.3}})
-- two horses harnessed, facing left
local h1 = horse_mask(gx - 30, gy + 0.5, 12, -1, false)
local h2 = horse_mask(gx - 27, gy + 1, 12, -1, false)
det(h2, pile{{"raw umber", 1}, {"bone black", 0.45}, {"red earth", 0.2}, medium=0.08}, 0.8, 7205)
det(h1, pile{{"red earth", 1}, {"raw umber", 0.9}, {"bone black", 0.1}, {"yellow ochre", 0.2}, medium=0.08}, 0.8, 7206)
-- a man on top of the load with a fork, and one on the ground handing up hay
local shirt = pile{{"lead white", 2}, {"yellow ochre", 0.2}, {"raw umber", 0.1}, medium=0.08}
local dk = pile{{"raw umber", 1}, {"bone black", 0.3}, {"pale smalt", 0.2}, medium=0.08}
local top = poly({{gx - 1.5, gy - 16}, {gx - 1.8, gy - 21}, {gx + 0.2, gy - 22.5}, {gx + 1.6, gy - 21}, {gx + 1.3, gy - 16}}, true)
det(top, shirt, 0.6, 7207, 3, math.pi/2)
det(ellipse(gx + 0.1, gy - 23.5, 1.1, 1.2), pile{{"yellow ochre", 1}, {"lead white", 0.6}, {"red earth", 0.2}, medium=0.08}, 0.5, 7208)
det(rect(gx - 1.5, gy - 16.5, 3, 1.2), dk, 0.5, 7209)
local gm_x = gx + 19
local low = poly({{gm_x - 1.3, gy}, {gm_x - 1.6, gy - 5}, {gm_x - 1.4, gy - 9.5}, {gm_x + 0.3, gy - 10.5}, {gm_x + 1.5, gy - 9.5}, {gm_x + 1.5, gy - 5}, {gm_x + 1.2, gy}}, true)
det(low * rect(0, gy - 11, 2000, 6), shirt, 0.6, 7210, 3, math.pi/2)
det(low * rect(0, gy - 5, 2000, 6), dk, 0.6, 7211, 3, math.pi/2)
det(ellipse(gm_x + 0.1, gy - 11.6, 1.1, 1.2), pile{{"yellow ochre", 1}, {"lead white", 0.6}, {"red earth", 0.2}, medium=0.08}, 0.5, 7212)
wb:load(wood, 0.4)
wb:stroke({{gx + 3, gy - 20}, {gx + 7, gy - 26}}, {pressure={0.25, 0.2}})
wb:stroke({{gm_x - 1, gy - 8}, {gm_x - 6, gy - 20}}, {pressure={0.25, 0.2}})
wb:load(hay, 0.5)
wb:touch(gm_x - 6.4, gy - 21, {pressure=0.6})

--@ chunk 73
local bp = pile{{"raw umber", 0.8}, {"bone black", 0.4}, {"pale smalt", 0.5}, {"lead white", 0.4}, medium=0.1}
local b = brush{kind="round", width=0.7, point=1, stiffness=0.4}
local birds = {{452, 262, 4.2, 0.1}, {466, 256, 3.6, -0.2}, {478, 266, 3.2, 0.3}, {491, 259, 3.4, 0}, {437, 271, 2.8, 0.2}, {503, 268, 2.6, -0.1}, {515, 253, 2.2, 0.15}}
for i,bd in ipairs(birds) do
  local x, y, s, tilt = bd[1], bd[2], bd[3], bd[4]
  b:load(bp, 0.35)
  local up = (i % 2 == 0) and -0.55 or 0.15
  b:stroke({{x - s, y + s*up + tilt*s}, {x - s*0.45, y - s*0.25}, {x, y}}, {pressure={0.05, 0.45}, ramps={0.5, 0.1}})
  b:stroke({{x, y}, {x + s*0.45, y - s*0.25}, {x + s, y + s*up - tilt*s}}, {pressure={0.45, 0.05}, ramps={0.1, 0.5}})
end

--@ chunk 74
local cast = pile{{"green earth", 1.3}, {"raw umber", 0.7}, {"Prussian blue", 0.06}, {"yellow ochre", 0.3}, medium=0.3}
local b = brush{kind="round", width=1.2, point=0.5}
for i,h in ipairs(HORSES) do
  b:load(cast, 0.5)
  local y = h.y + 0.4
  b:stroke({{h.x - 0.6*h.H, y}, {h.x + 0.6*h.H, y - 0.4}, {h.x + 2.2*h.H, y - 0.2}}, {pressure={0.5, 0.05}, ramps={0.1, 0.6}, clip=-h.m})
  b:load(cast, 0.4)
  b:stroke({{h.x - 0.5*h.H, y + 0.8}, {h.x + 1.8*h.H, y + 0.5}}, {pressure={0.4, 0.05}, ramps={0.1, 0.6}, clip=-h.m})
end
-- evening light along the backs
local lp = {pile{{"red earth", 0.6}, {"yellow ochre", 0.8}, {"lead white", 0.4}, medium=0.08}, pile{{"lead white", 2}, {"yellow ochre", 0.3}, medium=0.08}, pile{{"raw umber", 0.6}, {"lead white", 0.5}, {"yellow ochre", 0.3}, medium=0.08}, pile{{"yellow ochre", 0.9}, {"red earth", 0.4}, {"lead white", 0.5}, medium=0.08}}
local fine = brush{kind="round", width=0.6, point=1}
for i,h in ipairs(HORSES) do
  local rim = mask(function(x, y) if h.m:at(x, y) < 0.5 then return 0 end; return (h.m:at(x - 0.7, y - 1.1) < 0.5) and 1 or 0 end)
  work(rim, {hand="detail", tool=fine, pile=lp[i], angle=0, coverage=1.5, seed=7400+i})
end

--@ chunk 75
-- green the hedges, darker toward us; bluish only at the far rows
local near = HEDGES * mask(function(x, y) return y > 415 and 1 or 0 end)
local farh = HEDGES - near
local gnear = pile{{"green earth", 1.5}, {"raw umber", 0.45}, {"yellow ochre", 0.3}, {"Prussian blue", 0.05}, {"lead white", 0.35}, medium=0.12}
local gfar = pile{{"green earth", 1.2}, {"pale smalt", 0.6}, {"lead white", 0.7}, {"raw umber", 0.25}, {"yellow ochre", 0.2}, medium=0.12}
stipple(near:grow(0.4), {pile=gnear, width=1.2, coverage=1.6, cluster=0.5, clip=true, seed=7501})
stipple(farh:grow(0.3), {pile=gfar, width=1.0, coverage=1.4, cluster=0.5, clip=true, seed=7502})
local lit = mask(function(x, y) if HEDGES:at(x, y) < 0.5 then return 0 end; return (HEDGES:at(x - 1.2, y - 1.5) < 0.5) and 1 or 0 end)
local lp = pile{{"lead white", 1.2}, {"green earth", 1}, {"yellow ochre", 0.9}, {"chrome yellow", 0.12}, medium=0.1}
stipple(lit, {pile=lp, width=0.9, coverage=0.9, cluster=0.4, clip=true, seed=7503})

--@ chunk 76
print(wait(30*60)); for _,p in ipairs({{800,600},{830,590},{860,605},{905,620},{780,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 77
print(wait(72*60)); for _,p in ipairs({{800,600},{830,590},{860,605},{905,620},{820,585}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 78
local sil = BOULD
local ang = BF:field("across")
local base = pile{{"raw umber", 0.8}, {"pale smalt", 0.8}, {"bone black", 0.25}, {"lead white", 0.5}, {"yellow ochre", 0.1}, medium=0.08}
local dark = pile{{"raw umber", 1}, {"smalt", 0.4}, {"bone black", 0.4}, {"lead white", 0.12}, medium=0.08}
local lit = pile{{"lead white", 1.2}, {"raw umber", 0.5}, {"yellow ochre", 0.3}, {"pale smalt", 0.2}, medium=0.08}
local hi = pile{{"lead white", 1.8}, {"yellow ochre", 0.4}, {"raw umber", 0.15}, {"red earth", 0.05}, medium=0.08}
work(sil, {hand="body", tool="filbert 3", pile=base, angle=ang, length={5,14}, coverage=3, fill=true, clip=true, seed=7801})
local sh = sil * mask(function(x, y) return (BF:value(x, y) < 0.4) and 1 or 0 end)
work(sh, {hand="body", tool="filbert 3", pile=dark, angle=ang, length={5,14}, coverage=2.5, fill=true, clip=true, seed=7802})
local lm = sil * mask(function(x, y) return (BF:value(x, y) > 0.6) and 1 or 0 end)
work(lm, {hand="scumble", tool="filbert 3", pile=lit, angle=ang, length={4,10}, coverage=1.6, clip=true, seed=7803})
local hm = sil * mask(function(x, y) return (BF:value(x, y) > 0.8) and 1 or 0 end)
work(hm, {hand="detail", tool={kind="round", width=1.4, point=0.4}, pile=hi, angle=ang, coverage=1.2, seed=7804})
print(sh:area(), lm:area(), hm:area())

--@ chunk 79
local sil = BOULD
local ang = BF:field("across")
local midp = pile{{"raw umber", 0.7}, {"pale smalt", 0.6}, {"lead white", 0.55}, {"bone black", 0.15}, {"yellow ochre", 0.1}, medium=0.08}
-- lower part of the stone turns away from the light and into the grass shade
local low = sil * mask(function(x, y)
  local top = 572 + 0.004*(x - 800)^2
  return (y > top + 14 + 6*math.sin(x/9)) and 1 or 0 end)
work(low, {hand="body", tool="filbert 3", pile=midp, angle=ang, length={5,12}, coverage=2.2, fill=true, clip=true, seed=7901})
local dark = pile{{"raw umber", 1}, {"smalt", 0.4}, {"bone black", 0.45}, {"lead white", 0.1}, medium=0.08}
local base = sil * mask(function(x, y) return (y > 612 + 3*math.sin(x/7)) and 1 or 0 end)
work(base, {hand="body", tool="filbert 3", pile=dark, angle=0, length={5,12}, coverage=2, fill=true, clip=true, seed=7902})
blend(sil, {angle=ang, length={6,14}, coverage=0.7, seed=7903})

--@ chunk 80
local avoid = (MAN + WOMAN):grow(1) + BOULD:grow(0.5)
local dk = pile{{"green earth", 1.4}, {"raw umber", 0.9}, {"Prussian blue", 0.08}, {"bone black", 0.08}, medium=0.12}
local md = pile{{"green earth", 1.6}, {"yellow ochre", 0.9}, {"raw umber", 0.3}, {"Prussian blue", 0.03}, medium=0.12}
local lt = pile{{"yellow ochre", 1.2}, {"green earth", 1}, {"lead white", 0.5}, {"chrome yellow", 0.12}, medium=0.12}
local bl = brush{kind="round", width=1.4, point=1, stiffness=0.5}
local count = 0
local piles = {dk, dk, md, md, lt}
local cur = nil
for i=1,2600 do
  local x = rand(-5, 1005)
  local t = rand(0, 1)^0.7
  local y = lerp(fg(x) + 45, 650, t)
  if avoid:at(x, y) < 0.3 then
    local depth = (y - 560)/90
    local L = lerp(3, 15, clamp(depth, 0, 1)) * rand(0.6, 1.3)
    local w = lerp(0.5, 1.3, clamp(depth, 0, 1))
    local p = piles[math.random(1, 5)]
    if p ~= cur or bl:fullness() < 0.15 then bl:reload(p, 0.5); cur = p end
    local a = -math.pi/2 + randn(0, 0.3) + 0.15
    local bend = randn(0, 0.2)
    local ex, ey = x + L*math.cos(a), y + L*math.sin(a)
    local mx, my = x + 0.5*L*math.cos(a - bend), y + 0.5*L*math.sin(a - bend)
    bl:stroke({{x, y}, {mx, my}, {ex + bend*L*0.3, ey}}, {pressure={bl:pressure_for(w), 0}, ramps={0.05, 0.7}, clip=-avoid})
    count = count + 1
  end
end
print(count)

--@ chunk 81
local avoid = (MAN + WOMAN):grow(1) + BOULD:grow(0.5)
local dk = pile{{"green earth", 1.4}, {"raw umber", 0.9}, {"Prussian blue", 0.08}, {"bone black", 0.08}, medium=0.12}
local dk2 = pile{{"green earth", 1.6}, {"raw umber", 0.6}, {"Prussian blue", 0.12}, {"yellow ochre", 0.3}, medium=0.12}
local md = pile{{"green earth", 1.6}, {"yellow ochre", 0.9}, {"raw umber", 0.3}, {"Prussian blue", 0.03}, medium=0.12}
local bl = brush{kind="round", width=1.4, point=1, stiffness=0.5}
local cl = worley{seed=12, period=18, jitter=0.9}
local piles = {dk, dk2, dk2, md}
local cur = nil
local count = 0
for i=1,5200 do
  local x = rand(-5, 1005)
  local t = rand(0, 1)^0.8
  local y = lerp(fg(x) + 40, 652, t)
  if avoid:at(x, y) < 0.3 then
    local depth = clamp((y - 560)/90, 0, 1)
    local L = lerp(3, 13, depth) * rand(0.6, 1.3)
    local w = lerp(0.45, 1.1, depth)
    local p = piles[math.random(1, 4)]
    if p ~= cur or bl:fullness() < 0.15 then bl:reload(p, 0.5); cur = p end
    local c = cl:at(x, y)
    local lean = 0.25*math.sin(x/37 + y/23)
    local a = -math.pi/2 + lean + randn(0, 0.28)
    local bend = randn(0, 0.2)
    local ex, ey = x + L*math.cos(a), y + L*math.sin(a)
    local mx, my = x + 0.5*L*math.cos(a - bend), y + 0.5*L*math.sin(a - bend)
    bl:stroke({{x, y}, {mx, my}, {ex + bend*L*0.3, ey}}, {pressure={bl:pressure_for(w), 0}, ramps={0.05, 0.7}, clip=-avoid})
    count = count + 1
  end
end
print(count)

--@ chunk 82
print(drying(150, 490), drying(100, 460), drying(200, 470))
-- the ditch: a cool transparent glaze, a little heavier toward the near left, so it sits in the meadow
local g = pile{{"pale smalt", 1}, {"smalt", 0.6}, {"raw umber", 0.35}, {"green earth", 0.3}, medium=0.6}
work(LWATER:grow(0.3), {hand="detail", tool={kind="round", width=2, point=0.3}, pile=g, angle=-0.1, coverage=1.4, load_at=function(x, y) return lerp(0.7, 0.35, clamp(x/330, 0, 1)) end, seed=8201})
-- a few bright sky glints on the water's far side
local gl = pile{{"lead white", 3}, {"yellow ochre", 0.12}, {"pale smalt", 0.2}, medium=0.1}
local b = brush{kind="round", width=0.8, point=1}
b:load(gl, 0.4)
for i=1,9 do
  local x = rand(10, 320)
  local sy, sw = STREAMY(x)
  local y = sy - sw*0.25 + rand(-0.5, 0.5)
  b:stroke({{x, y}, {x + rand(4, 12), y - 0.3}}, {pressure={0.35, 0.05}, clip=LWATER})
end

--@ chunk 83
local g = pile{{"green earth", 1.5}, {"raw umber", 0.4}, {"pale smalt", 0.3}, {"Prussian blue", 0.02}, medium=0.6}
for wi, w in ipairs(WILLOWS) do
  work(w.crown, {hand="detail", tool={kind="round", width=2.2, point=0.3}, pile=g, angle=-1.2, coverage=1.3, load_at=function(x, y) return clamp(0.3 + (x - w.hx)/(w.h*0.8) + (y - w.hy + w.h*0.5)/(w.h*1.2), 0.2, 0.75) end, seed=8300+wi})
end

--@ chunk 84
for wi, w in ipairs(WILLOWS) do blend(w.crown, {angle=-1.3, length={4,10}, coverage=1.5, seed=8400+wi}) end

--@ chunk 85
local det = function(m, p, w, seed, cov, ang) work(m, {hand="detail", tool={kind="round", width=w or 0.7, point=0.6}, pile=p, angle=ang or 0, coverage=cov or 3, fill=true, seed=seed}) end
function cow_mask(x, yf, H, f)
  local m = ellipse(x, yf - 0.62*H, 0.62*H, 0.26*H)
  m = m + ellipse(x - f*0.45*H, yf - 0.66*H, 0.22*H, 0.27*H) + ellipse(x + f*0.4*H, yf - 0.6*H, 0.22*H, 0.28*H)
  m = m + ribbon({{x + f*0.55*H, yf - 0.62*H}, {x + f*0.8*H, yf - 0.3*H}}, {0.26*H, 0.16*H})
  m = m + ellipse(x + f*0.84*H, yf - 0.22*H, 0.1*H, 0.14*H)
  local lw = 0.09*H
  for _,lx in ipairs({0.42, 0.3, -0.38, -0.5}) do
    m = m + ribbon({{x + f*lx*H, yf - 0.45*H}, {x + f*lx*H, yf}}, {lw*1.2, lw})
  end
  m = m + ribbon({{x - f*0.66*H, yf - 0.8*H}, {x - f*0.7*H, yf - 0.35*H}}, {0.05*H, 0.05*H})
  return m
end
local red = pile{{"red earth", 1}, {"raw umber", 0.6}, {"yellow ochre", 0.3}, {"lead white", 0.15}, medium=0.08}
local blk = pile{{"bone black", 0.6}, {"raw umber", 0.6}, {"lead white", 0.15}, medium=0.08}
local wht = pile{{"lead white", 2}, {"yellow ochre", 0.15}, {"raw umber", 0.1}, medium=0.08}
local cast = pile{{"green earth", 1.3}, {"raw umber", 0.7}, {"Prussian blue", 0.06}, {"yellow ochre", 0.3}, medium=0.3}
COWS = {{x=376, y=460, H=8.5, f=1, p=red}, {x=399, y=457, H=7.8, f=-1, p=blk}, {x=421, y=462, H=8.8, f=1, p=red}}
local sb = brush{kind="round", width=1, point=0.5}
for i,c in ipairs(COWS) do
  local m = cow_mask(c.x, c.y, c.H, c.f)
  sb:load(cast, 0.5)
  sb:stroke({{c.x - 0.5*c.H, c.y + 0.3}, {c.x + 2*c.H, c.y}}, {pressure={0.5, 0.05}, ramps={0.1, 0.6}, clip=-m})
  det(m, c.p, 0.7, 8500+i)
  if c.p == blk then
    det(ellipse(c.x + 0.1*c.H, c.y - 0.62*c.H, 0.22*c.H, 0.14*c.H) + ellipse(c.x - 0.4*c.H, c.y - 0.7*c.H, 0.12*c.H, 0.1*c.H), wht, 0.5, 8510+i)
  else
    det(ellipse(c.x - 0.1*c.H, c.y - 0.5*c.H, 0.18*c.H, 0.1*c.H), wht, 0.5, 8510+i)
  end
end
-- a white stork in the meadow on the near side of the ditch
local sx, sy = 238, 501
local body = ellipse(sx, sy - 6.2, 2.6, 1.5):offset(0)
local neck = ribbon({{sx + 1.8, sy - 6.8}, {sx + 2.6, sy - 9}, {sx + 2.4, sy - 10.6}}, {0.9, 0.7, 0.8})
local tail = poly({{sx - 2.4, sy - 6.9}, {sx - 3.8, sy - 5.6}, {sx - 1.5, sy - 5.2}})
det(body + neck, wht, 0.5, 8521)
det(tail + ellipse(sx - 0.8, sy - 5.6, 1.8, 0.7), blk, 0.4, 8522)
local rb = brush{kind="round", width=0.35, point=1}
rb:load(pile{{"vermilion", 1}, {"red earth", 0.4}, medium=0.08}, 0.4)
rb:stroke({{sx - 0.2, sy - 5}, {sx - 0.3, sy}}, {pressure={0.4, 0.4}})
rb:stroke({{sx + 0.6, sy - 5}, {sx + 0.9, sy - 0.2}}, {pressure={0.4, 0.4}})
rb:stroke({{sx + 2.9, sy - 10.6}, {sx + 5.2, sy - 9.4}}, {pressure={0.45, 0.1}})

--@ chunk 86
print(wait(72*60)); for _,p in ipairs({{800,600},{830,590},{100,450},{170,460},{500,620},{300,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 87
local silver = pile{{"lead white", 2}, {"green earth", 0.8}, {"pale smalt", 0.35}, {"yellow ochre", 0.3}, medium=0.1}
local silver2 = pile{{"lead white", 2.6}, {"yellow ochre", 0.35}, {"green earth", 0.4}, {"red earth", 0.03}, medium=0.1}
local midg = pile{{"lead white", 1}, {"green earth", 1.4}, {"pale smalt", 0.5}, {"yellow ochre", 0.3}, medium=0.1}
for wi, w in ipairs(WILLOWS) do
  local s = w.h/60
  local upl = w.crown * mask(function(x, y) return ((x - w.hx)*0.7 + (y - (w.hy - w.h*0.35))*0.7 < 2*s) and 1 or 0 end)
  stipple(upl, {pile=midg, width=1.3*s + 0.5, coverage=1.1, cluster=0.6, clip=true, dips={14, 0.5, 0.5}, seed=8700+wi})
  local rim = mask(function(x, y) if w.crown:at(x, y) < 0.5 then return 0 end; return (w.crown:at(x - 2.2*s, y - 2.5*s) < 0.5) and 1 or 0 end)
  stipple(rim:grow(0.8), {pile=silver, width=1.1*s + 0.45, coverage=1.2, cluster=0.5, clip=true, seed=8720+wi})
  stipple(upl * rim:grow(2*s), {pile=silver2, width=0.9*s + 0.4, coverage=0.5, cluster=0.5, clip=true, seed=8740+wi})
  -- scattered leaf-light through the middle of the crown
  stipple(w.crown:shrink(2*s), {pile=silver, width=0.9*s + 0.4, coverage=0.25, cluster=0.7, clip=true, seed=8760+wi})
end

--@ chunk 88
local avoid = (MAN + WOMAN):grow(2) + BOULD:grow(1)
local stalk = pile{{"green earth", 1.4}, {"yellow ochre", 0.7}, {"raw umber", 0.4}, medium=0.1}
local wh = pile{{"lead white", 2.2}, {"yellow ochre", 0.12}, medium=0.08}
local ye = pile{{"chrome yellow", 1}, {"yellow ochre", 0.4}, {"lead white", 0.4}, medium=0.08}
local bl = pile{{"cobalt blue", 1}, {"smalt", 0.5}, {"lead white", 0.4}, medium=0.08}
local rd = pile{{"vermilion", 0.5}, {"red earth", 0.3}, {"lead white", 1}, medium=0.08}
local st = brush{kind="round", width=0.6, point=1}
local tip = brush{kind="round", width=1.2, point=0.5}
local function flower(x, y, h, p, r, n)
  if avoid:at(x, y) > 0.2 then return end
  st:load(stalk, 0.4)
  local tx, ty = x + rand(-1, 1)*h*0.15, y - h
  st:stroke({{x, y}, {(x + tx)/2 + rand(-0.4, 0.4), (y + ty)/2}, {tx, ty}}, {pressure={0.35, 0.15}})
  tip:load(p, 0.5)
  for k=1,n do tip:touch(tx + randn(0, r*0.5), ty + randn(0, r*0.3), {pressure=clamp(r*0.35, 0.15, 0.7)}) end
end
for i=1,70 do local x = rand(0, 1000); local y = rand(fg(x) + 50, 645); local d = clamp((y - 560)/90, 0.2, 1); flower(x, y, 6*d*rand(0.7, 1.4), wh, 1.2*d + 0.4, 3) end
for i=1,55 do local x = rand(0, 1000); local y = rand(fg(x) + 45, 645); local d = clamp((y - 560)/90, 0.2, 1); flower(x, y, 4*d*rand(0.7, 1.4), ye, 0.9*d + 0.3, 1) end
for i=1,14 do local x = rand(0, 700); local y = rand(590, 645); local d = clamp((y - 560)/90, 0.2, 1); flower(x, y, 7*d*rand(0.8, 1.3), bl, 1*d + 0.4, 2) end
for i=1,22 do local x = rand(0, 1000); local y = rand(fg(x) + 50, 645); local d = clamp((y - 560)/90, 0.2, 1); flower(x, y, 4*d*rand(0.7, 1.3), rd, 1*d + 0.3, 2) end
-- seed-headed grass stalks, taller, near the bottom and around the stone
local seedp = pile{{"yellow ochre", 1}, {"raw umber", 0.4}, {"lead white", 0.3}, medium=0.08}
for i=1,40 do
  local x = (i <= 12) and rand(720, 990) or rand(0, 700)
  local y = (i <= 12) and rand(618, 645) or rand(605, 648)
  if avoid:at(x, y) < 0.2 then
    local h = rand(14, 30)
    local lean = rand(-0.25, 0.25)
    local tx, ty = x + h*math.sin(lean), y - h*math.cos(lean)
    st:load(stalk, 0.45)
    st:stroke({{x, y}, {(x + tx)/2 + lean*3, (y + ty)/2}, {tx, ty}}, {pressure={0.4, 0.1}})
    tip:load(seedp, 0.4)
    tip:stroke({{tx - 0.2*math.sin(lean)*3, ty + 3}, {tx, ty}, {tx + math.sin(lean)*2, ty - 2.5}}, {pressure={0.45, 0.05}})
  end
end

--@ chunk 89
local avoid = (MAN + WOMAN):grow(2) + BOULD:grow(1) + NARROW
local wh = pile{{"lead white", 2.2}, {"yellow ochre", 0.1}, medium=0.08}
local ye = pile{{"chrome yellow", 1}, {"yellow ochre", 0.3}, {"lead white", 0.3}, medium=0.08}
local bl = pile{{"cobalt blue", 1}, {"smalt", 0.4}, {"lead white", 0.5}, medium=0.08}
local rd = pile{{"vermilion", 0.6}, {"red earth", 0.2}, {"lead white", 0.9}, medium=0.08}
local t = brush{kind="round", width=2.4, point=0.3, stiffness=0.5}
local t2 = brush{kind="round", width=1.2, point=0.3, stiffness=0.5}
print(t:mark_width(0.3), t:mark_width(0.6))
local function yarrow(x, y, d)
  t2:load(wh, 0.5)
  for k=1,6 do t2:touch(x + randn(0, 1.4*d), y + randn(0, 0.5*d), {pressure=0.35 + 0.2*d}) end
end
local function daisy(x, y, d)
  t:load(wh, 0.6); t:touch(x, y, {pressure=0.25 + 0.3*d})
  t2:load(ye, 0.5); t2:touch(x + 0.1, y + 0.05, {pressure=0.2})
end
local function dot(x, y, d, p) t2:load(p, 0.5); t2:touch(x, y, {pressure=0.3 + 0.35*d}) end
local n = 0
for i=1,260 do
  local x = rand(0, 1000)
  local y = lerp(fg(x) + 50, 646, rand(0,1)^0.6)
  local d = clamp((y - 560)/90, 0.1, 1)
  if avoid:at(x, y) < 0.2 then
    local r = math.random()
    if r < 0.25 then yarrow(x, y, d) elseif r < 0.5 then daisy(x, y, d) elseif r < 0.8 then dot(x, y, d, ye) elseif r < 0.9 then dot(x, y, d, rd) else dot(x, y, d, bl) end
    n = n + 1
  end
end
print(n)

--@ chunk 90
print(wait(48*60)); for _,p in ipairs({{800,600},{830,590},{860,605},{780,610},{810,580}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 91
local sil = BOULD
local gz = pile{{"raw umber", 1}, {"smalt", 0.5}, {"bone black", 0.3}, medium=0.6}
local function ld(x, y)
  local v = BF:value(x, y)
  local top = 572 + 0.004*(x - 800)^2
  local down = clamp((y - top)/40, 0, 1)
  return clamp(0.15 + (1 - v)*0.55 + down*0.35, 0.1, 0.85)
end
work(sil, {hand="detail", tool={kind="round", width=3, point=0.3}, pile=gz, angle=BF:field("across"), coverage=1.8, fill=true, load_at=ld, seed=9101})

--@ chunk 92
local sil = BOULD
local ang = BF:field("across")
local cool = pile{{"lead white", 1}, {"pale smalt", 0.7}, {"raw umber", 0.3}, {"bone black", 0.05}, medium=0.1}
local warm = pile{{"lead white", 1.6}, {"yellow ochre", 0.35}, {"raw umber", 0.2}, {"red earth", 0.04}, medium=0.1}
local midm = sil * mask(function(x, y) return (BF:value(x, y) > 0.42) and 1 or 0 end)
work(midm, {hand="scumble", tool="filbert 3", pile=cool, angle=ang, length={4,10}, coverage=1.3, pressure={0.3, 0.5}, clip=true, seed=9201})
local top = sil * mask(function(x, y)
  local tp = 572 + 0.004*(x - 800)^2
  return (BF:value(x, y) > 0.6 and y < tp + 14 + 4*math.sin(x/6)) and 1 or 0 end)
work(top, {hand="scumble", tool="filbert 2", pile=warm, angle=ang, length={3,8}, coverage=1.4, pressure={0.35, 0.55}, clip=true, seed=9202})

--@ chunk 93
print(wait(72*60)); for _,p in ipairs({{800,600},{830,590},{860,605},{780,610},{810,580},{905,618}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 94
print(drying(100, 450), drying(60, 440))
local g = pile{{"green earth", 1.5}, {"yellow ochre", 0.3}, {"pale smalt", 0.3}, {"raw umber", 0.1}, medium=0.75}
for wi, w in ipairs(WILLOWS) do
  work(w.crown, {hand="detail", tool={kind="round", width=2.2, point=0.3}, pile=g, angle=-1.2, coverage=1, load=0.3, seed=9400+wi})
end

--@ chunk 95
print(wait(96*60)); for _,p in ipairs({{800,600},{830,590},{860,605},{780,610},{810,580},{905,618},{770,590}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 96
print(wait(96*60)); for _,p in ipairs({{800,600},{780,610},{810,580},{770,590},{790,575}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 97
local sil = BOULD
local veil = pile{{"pale smalt", 1}, {"lead white", 0.5}, {"raw umber", 0.45}, {"bone black", 0.08}, medium=0.45}
work(sil, {hand="detail", tool={kind="round", width=2.5, point=0.3}, pile=veil, angle=BF:field("across"), coverage=2, fill=true, load=0.55, seed=9701})

--@ chunk 98
print(wait(72*60)); for _,p in ipairs({{800,600},{780,610},{810,580},{770,590},{860,600},{905,620}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 99
print(wait(48*60)); for _,p in ipairs({{800,600},{780,610},{790,612},{820,615}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 100
local sil = BOULD
local gz = pile{{"raw umber", 0.6}, {"smalt", 0.6}, {"bone black", 0.25}, medium=0.7}
local function ld(x, y)
  local v = BF:value(x, y)
  local top = 572 + 0.004*(x - 800)^2
  local down = clamp((y - top)/45, 0, 1)
  return clamp((0.75 - v)*0.9 + down*0.3, 0.0, 0.8)
end
local shadowish = sil * mask(function(x, y) return (ld(x, y) > 0.08) and 1 or 0 end)
work(shadowish, {hand="detail", tool={kind="round", width=2.4, point=0.3}, pile=gz, angle=BF:field("across"), coverage=1.8, fill=true, load_at=ld, seed=10001})

--@ chunk 101
local sil = BOULD
local warm = pile{{"lead white", 1.6}, {"yellow ochre", 0.35}, {"raw umber", 0.25}, {"red earth", 0.04}, medium=0.1}
local ridge = sil * mask(function(x, y)
  local top = 572 + 0.004*(x - 800)^2
  return (BF:value(x, y) > 0.62 and y < top + 7 + 3*math.sin(x/5) and x < 830) and 1 or 0 end)
stipple(ridge, {pile=warm, width=1.3, coverage=0.9, cluster=0.5, drag={1.2, -0.3}, clip=true, seed=10101})
-- grass over the foot of the stones
local dk = pile{{"green earth", 1.4}, {"raw umber", 0.9}, {"Prussian blue", 0.08}, {"bone black", 0.08}, medium=0.12}
local md = pile{{"green earth", 1.6}, {"yellow ochre", 0.9}, {"raw umber", 0.3}, medium=0.12}
local bl = brush{kind="round", width=1.3, point=1, stiffness=0.5}
for i=1,260 do
  local x = rand(740, 990)
  local yb = nil
  -- find the stone's lower edge at this x
  for y = 640, 560, -1 do if sil:at(x, y) > 0.5 then yb = y; break end end
  if yb then
    local y = yb + rand(-2, 4)
    local L = rand(4, 11)
    bl:reload((i % 3 == 0) and md or dk, 0.5)
    local a = -math.pi/2 + randn(0, 0.3)
    bl:stroke({{x, y}, {x + L*0.5*math.cos(a + 0.1), y + L*0.5*math.sin(a + 0.1)}, {x + L*math.cos(a), y + L*math.sin(a)}}, {pressure={bl:pressure_for(0.9), 0}, ramps={0.05, 0.7}})
  end
end

--@ chunk 102
local sil = BOULD
local lt = pile{{"yellow ochre", 1.2}, {"green earth", 1}, {"lead white", 0.4}, {"chrome yellow", 0.1}, medium=0.12}
local md = pile{{"green earth", 1.6}, {"yellow ochre", 1.1}, {"raw umber", 0.2}, {"lead white", 0.2}, medium=0.12}
local bl = brush{kind="round", width=1.3, point=1, stiffness=0.5}
local n = 0
for i=1,400 do
  local x = rand(745, 985)
  local yb
  for y = 645, 560, -1 do if sil:at(x, y) > 0.5 then yb = y; break end end
  if yb then
    n = n + 1
    local y = yb + rand(1, 7)
    local L = rand(6, 14)
    bl:reload((i % 4 == 0) and lt or md, 0.5)
    local a = -math.pi/2 + randn(0, 0.35)
    bl:stroke({{x, y}, {x + L*0.5*math.cos(a + 0.12), y + L*0.5*math.sin(a + 0.12)}, {x + L*math.cos(a), y + L*math.sin(a)}}, {pressure={bl:pressure_for(0.8), 0}, ramps={0.05, 0.7}})
  end
end
print(n)

--@ chunk 103
local sil = BOULD
local dk = pile{{"green earth", 1.3}, {"raw umber", 1}, {"Prussian blue", 0.1}, {"bone black", 0.15}, medium=0.12}
local bl = brush{kind="round", width=1.5, point=1, stiffness=0.5}
for i=1,420 do
  local x = rand(745, 985)
  local yb
  for y = 645, 560, -1 do if sil:at(x, y) > 0.5 then yb = y; break end end
  if yb then
    local y = yb + rand(0, 9)
    local L = rand(5, 15)
    bl:reload(dk, 0.55)
    local a = -math.pi/2 + randn(0, 0.4)
    bl:stroke({{x, y}, {x + L*0.5*math.cos(a - 0.1), y + L*0.5*math.sin(a - 0.1)}, {x + L*math.cos(a), y + L*math.sin(a)}}, {pressure={bl:pressure_for(1.0), 0}, ramps={0.05, 0.7}})
  end
end

--@ chunk 104
print(drying(700, 495), drying(900, 505))
local wood = pile{{"raw umber", 1}, {"lead white", 0.35}, {"bone black", 0.12}, {"pale smalt", 0.2}, medium=0.08}
local lit = pile{{"lead white", 1.4}, {"yellow ochre", 0.35}, {"raw umber", 0.35}, medium=0.08}
local b = brush{kind="round", width=1.0, point=0.8, stiffness=0.5}
local rb = brush{kind="round", width=0.7, point=1, stiffness=0.5}
local x0, y0, x1, y1 = 1008, 511, 560, 484
local posts = {}
local t = 0
while t < 1 do
  local x, y = lerp(x0, x1, t), lerp(y0, y1, t)
  local sc = (y - 392)/(511 - 392)
  posts[#posts+1] = {x=x + rand(-0.4, 0.4), y=y, h=7.5*sc*rand(0.9, 1.1), w=1.1*sc + 0.35, lean=rand(-0.06, 0.06)}
  t = t + (15*sc)/(x0 - x1)
end
-- rails first (behind the posts' lit faces), two, sagging a little
for r=1,2 do
  local pts = {}
  for i,p in ipairs(posts) do
    local fr = (r == 1) and 0.85 or 0.45
    pts[#pts+1] = {p.x, p.y - p.h*fr + ((i % 2 == 0) and 0.3 or 0)}
  end
  rb:load(wood, 0.6)
  for i=1,#pts-1 do
    if i % 5 == 0 then rb:load(wood, 0.6) end
    local sc = (posts[i].y - 392)/(119)
    rb:stroke({pts[i], {(pts[i][1] + pts[i+1][1])/2, (pts[i][2] + pts[i+1][2])/2 + 0.3*sc}, pts[i+1]}, {pressure={rb:pressure_for(0.55*sc + 0.25), rb:pressure_for(0.55*sc + 0.25)}})
  end
end
for i,p in ipairs(posts) do
  b:load(wood, 0.5)
  b:stroke({{p.x, p.y + 0.3}, {p.x + p.lean*p.h, p.y - p.h}}, {pressure={b:pressure_for(p.w), b:pressure_for(p.w*0.9)}, ramps={0.02, 0.02}})
  rb:load(lit, 0.4)
  rb:stroke({{p.x - p.w*0.3 + p.lean*p.h, p.y - p.h + 0.2}, {p.x - p.w*0.3, p.y - p.h*0.35}}, {pressure={rb:pressure_for(p.w*0.35), 0.05}})
end
print(#posts)

--@ chunk 105
print(drying(200, 630), drying(600, 630), drying(345, 540))
local gz = pile{{"raw umber", 0.8}, {"green earth", 0.8}, {"Prussian blue", 0.06}, {"bone black", 0.1}, medium=0.75}
local m = mask(function(x, y) return (y > 592 and x < 736) and 1 or 0 end) - (MAN + WOMAN):grow(1)
work(m, {hand="glaze", tool="filbert 14", pile=gz, angle=0.05, length={100,250}, coverage=1.2, clip=true, load_at=function(x, y) return clamp((y - 592)/60, 0, 1)*0.55 end, seed=10501})
-- the dress: a deeper, cooler red glaze
local skirt = poly({{336, 509}, {348.5, 509}, {350.5, 516}, {352.5, 540}, {355.5, 566.5}, {343, 568}, {331.5, 566.5}, {333.5, 540}, {334.5, 516}}, true)
local rg = pile{{"red earth", 0.8}, {"raw umber", 0.5}, {"bone black", 0.12}, {"smalt", 0.15}, medium=0.65}
work(skirt, {hand="detail", tool={kind="round", width=1.8, point=0.3}, pile=rg, angle=math.pi/2, coverage=1.5, fill=true, load=0.5, seed=10502})

--@ chunk 106
local dk = pile{{"smalt", 0.8}, {"raw umber", 0.8}, {"bone black", 0.3}, {"lead white", 0.35}, medium=0.1}
local lt = pile{{"lead white", 2.2}, {"yellow ochre", 0.3}, {"red earth", 0.12}, {"pale smalt", 0.4}, medium=0.1}
local f = brush{kind="round", width=0.5, point=1, stiffness=0.4}
-- finials: a thin rod with a ball and a vane on each spire tip
for _,t in ipairs(TW) do
  f:load(dk, 0.35)
  f:stroke({{t.x, t.top + 0.5}, {t.x, t.top - 4.5}}, {pressure={0.3, 0.25}})
  f:touch(t.x, t.top - 2, {pressure=0.35})
  f:stroke({{t.x - 1.2, t.top - 3.6}, {t.x + 1.2, t.top - 3.6}}, {pressure={0.2, 0.2}})
end
-- eaves and ridges of the naves: dark eave line, light along the west hip
for _,nv in ipairs({{606,652,13,12},{675,716,14,13},{731,764,10,9}}) do
  local by = math.max(far(nv[1]) + 1.5, far(nv[2]) + 1.5)
  local ey = by - nv[3]
  f:load(dk, 0.4)
  f:stroke({{nv[1] + 0.3, ey + 0.4}, {nv[2] - 0.3, ey + 0.4}}, {pressure={0.3, 0.3}})
  f:load(lt, 0.35)
  f:stroke({{nv[1] + 0.4, ey - 0.3}, {nv[1] + 1.6, ey - nv[4] + 0.5}}, {pressure={0.35, 0.2}})
  f:stroke({{nv[1] + 1.8, ey - nv[4] + 0.4}, {nv[2] - 1.8, ey - nv[4] + 0.4}}, {pressure={0.18, 0.12}})
end

--@ chunk 107
local wool = pile{{"lead white", 2}, {"yellow ochre", 0.25}, {"raw umber", 0.12}, medium=0.08}
local woolsh = pile{{"lead white", 1}, {"raw umber", 0.4}, {"pale smalt", 0.3}, medium=0.08}
local dk = pile{{"raw umber", 1}, {"bone black", 0.3}, {"pale smalt", 0.2}, medium=0.08}
local t = brush{kind="round", width=1.6, point=0.3}
local f = brush{kind="round", width=0.5, point=1}
local cx, cy = 640, 428
for i=1,19 do
  local x = cx + randn(0, 11)
  local y = cy + randn(0, 1.6)
  local s = (y - 392)/36
  t:load(woolsh, 0.4); t:stroke({{x - 0.9*s, y - 0.5*s}, {x + 0.9*s, y - 0.55*s}}, {pressure={0.45, 0.45}})
  t:load(wool, 0.4); t:stroke({{x - 0.9*s, y - 0.8*s}, {x + 0.7*s, y - 0.85*s}}, {pressure={0.3, 0.25}})
  f:load(dk, 0.3); f:touch(x + (i % 2 == 0 and 1.1 or -1.1)*s, y - 0.5*s, {pressure=0.3})
end
-- the shepherd with his crook, and his dog
local sx, sy = 668, 430
f:load(dk, 0.5)
f:stroke({{sx, sy}, {sx, sy - 3.4}}, {pressure={0.9, 0.8}})
f:stroke({{sx - 0.4, sy - 1.5}, {sx - 0.2, sy - 4}}, {pressure={0.8, 0.7}})
f:touch(sx - 0.1, sy - 4.6, {pressure=0.6})
f:stroke({{sx + 1.3, sy + 0.1}, {sx + 1.6, sy - 6}}, {pressure={0.3, 0.3}})
f:stroke({{sx + 1.6, sy - 6}, {sx + 2.3, sy - 6.4}, {sx + 2.5, sy - 5.8}}, {pressure={0.25, 0.2}})
f:stroke({{sx - 5, sy + 0.4}, {sx - 3.5, sy + 0.3}}, {pressure={0.7, 0.6}})
f:touch(sx - 3.2, sy - 0.3, {pressure=0.45})

--@ chunk 108
print(wait(60*60)); for _,p in ipairs({{342,515},{342,540},{338,476}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 109
local pu = pile{{"green earth", 2}, {"yellow ochre", 1.3}, {"chrome yellow", 0.18}, {"lead white", 0.95}, {"Prussian blue", 0.04}, medium=0.12}
-- converging banks: cover the water's edges more and more toward its end
local taper = LWATER:grow(0.5) * mask(function(x, y)
  if x < 312 then return 0 end
  local sy, sw = STREAMY(x)
  local k = clamp((x - 312)/18, 0, 1)
  local half = sw/2 * (1 - k)
  return (math.abs(y - sy) > half) and 1 or 0 end)
local patch = (WATER:grow(3) * rect(329, 0, 22, 700)) + taper
work(patch, {hand="detail", tool={kind="round", width=1.2, point=0.5}, pile=pu, angle=-0.1, coverage=3, fill=true, seed=10901})
-- a few rushes where the ditch runs out
local rush = pile{{"green earth", 1.4}, {"raw umber", 0.6}, {"yellow ochre", 0.5}, medium=0.1}
local rl = pile{{"yellow ochre", 1}, {"green earth", 0.8}, {"lead white", 0.5}, medium=0.1}
local b = brush{kind="round", width=0.6, point=1}
for i=1,26 do
  local x = rand(318, 336)
  local sy = STREAMY(x)
  local y = sy + rand(-2.5, 2.5)
  local h = rand(3, 7)
  b:load((i % 3 == 0) and rl or rush, 0.4)
  b:stroke({{x, y}, {x + rand(-1, 1), y - h}}, {pressure={0.35, 0}, ramps={0.05, 0.6}})
end

--@ chunk 110
print(wait(72*60)); for _,p in ipairs({{342,515},{342,540},{340,520},{345,512}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 111
local det = function(m, p, w, seed, cov) work(m, {hand="detail", tool={kind="round", width=w or 1, point=0.6}, pile=p, angle=math.pi/2, coverage=cov or 2.5, fill=true, seed=seed}) end
local shawl = poly({{335, 509.5}, {339, 506.5}, {346, 506.5}, {350, 509.5}, {351, 515}, {346, 522}, {342.5, 530}, {339, 522}, {334.2, 515}}, true)
local cream = pile{{"lead white", 2.2}, {"yellow ochre", 0.22}, {"raw umber", 0.06}, medium=0.08}
local creamsh = pile{{"lead white", 1.4}, {"raw umber", 0.3}, {"pale smalt", 0.35}, {"yellow ochre", 0.1}, medium=0.08}
det(shawl, cream, 1.0, 11101, 3.5)

--@ chunk 112
local shawl = poly({{335, 509.5}, {339, 506.5}, {346, 506.5}, {350, 509.5}, {351, 515}, {346, 522}, {342.5, 530}, {339, 522}, {334.2, 515}}, true)
local creamsh = pile{{"lead white", 1.3}, {"raw umber", 0.32}, {"pale smalt", 0.4}, {"yellow ochre", 0.1}, medium=0.08}
work(shawl * poly({{344.5, 500}, {360, 500}, {360, 535}, {343, 535}}), {hand="detail", tool={kind="round", width=0.8, point=0.6}, pile=creamsh, angle=math.pi/2, coverage=1.6, fill=true, seed=11201})
local fr = brush{kind="round", width=0.45, point=1}
fr:load(creamsh, 0.4)
for i=0,7 do local t = i/7; local x = lerp(336, 342.5, t); local y = lerp(516.5, 530, t); fr:stroke({{x, y}, {x + 0.15, y + 1.4}}, {pressure={0.3, 0.05}}) end
for i=0,7 do local t = i/7; local x = lerp(342.5, 350, t); local y = lerp(530, 516.5, t); fr:stroke({{x, y}, {x + 0.15, y + 1.4}}, {pressure={0.3, 0.05}}) end
-- a fold line of the shawl down its back
fr:load(creamsh, 0.3)
fr:stroke({{342.4, 509}, {342.2, 518}, {342.5, 527}}, {pressure={0.2, 0.05}})

--@ chunk 113
local hayrow = pile{{"yellow ochre", 1.2}, {"lead white", 1}, {"green earth", 0.3}, {"raw umber", 0.1}, medium=0.15}
local b = brush{kind="round", width=1.2, point=0.5}
local avoid = nil
for _,h in ipairs(HAY) do avoid = avoid and (avoid + h.m:grow(1.5)) or h.m:grow(1.5) end
local rows = {441, 445.5, 450.5, 456, 462, 468.5}
for i,y0 in ipairs(rows) do
  local sc = (y0 - 392)/70
  local x = 610 + rand(0, 30)
  while x < 1000 do
    local L = rand(25, 70)
    b:load(hayrow, 0.3)
    local y = y0 + (x - 800)*0.008
    b:stroke({{x, y}, {x + L*0.5, y + rand(-0.3, 0.3)}, {x + L, y + L*0.008}}, {pressure={b:pressure_for(0.5 + 0.7*sc), 0.1}, ramps={0.2, 0.3}, clip=-avoid, shake=0.5})
    x = x + L + rand(6, 20)
  end
end

--@ chunk 114
print(wait(45*24*60))

--@ chunk 115
varnish{coats=0.25, vary=0.1}
