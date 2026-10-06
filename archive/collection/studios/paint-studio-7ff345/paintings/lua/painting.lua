-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=700, aspect=1.4, linen={17,14}, seed=1807,
 ground={{pile={{"red earth",2},{"yellow ochre",2},{"lead white",1}}, um=90, apply="knife", texture=0.3},
         {pile={{"lead white",8},{"yellow ochre",0.25},{"raw umber",0.08}}, um=45, apply="brush"}}}
print(W,H)

--@ chunk 2
-- geometry, kept as globals
function knoll(x)
  local k = 468 - 52 * math.exp(-((x - 190) / 190)^2) + 3*math.sin(x/37)
  return k
end
function horizon(x) return 468 + 1.5*math.sin(x/53) end
function woodtop(x) return 452 + 5*math.sin(x/23) + 4*math.sin(x/61+1) + 3*math.sin(x/9.7) end
h = pencil("2H")
-- horizon line
local pts = {}
for x = 380, 1000, 20 do pts[#pts+1] = {x, horizon(x)} end
h:sketch(pts, {pressure=0.25})
-- knoll
pts = {}
for x = 0, 480, 20 do pts[#pts+1] = {x, math.min(knoll(x), horizon(x))} end
h:sketch(pts, {pressure=0.3})
-- distant woods top
pts = {}
for x = 380, 1000, 10 do pts[#pts+1] = {x, woodtop(x)} end
h:sketch(pts, {pressure=0.18})
-- church at x 640
h:line({{634, 466}, {634, 418}}, {pressure=0.25})
h:line({{646, 466}, {646, 418}}, {pressure=0.25})
h:line({{633, 418}, {640, 392}, {647, 418}}, {pressure=0.25, smooth=false})
h:line({{646, 440}, {690, 440}, {690, 466}}, {pressure=0.2, smooth=false})
-- mere
pts = {}
for i = 0, 36 do
  local a = i / 36 * 2 * math.pi
  pts[#pts+1] = {640 + 255*math.cos(a) + 12*math.sin(3*a), 574 + 36*math.sin(a) + 4*math.sin(5*a)}
end
h:sketch(pts, {pressure=0.25})
-- oak trunk
h:sketch({{215, 440}, {220, 380}, {224, 320}, {222, 270}}, {pressure=0.35})
h:sketch({{262, 440}, {254, 380}, {252, 320}, {256, 270}}, {pressure=0.35})
-- main boughs
h:sketch({{224, 280}, {180, 240}, {130, 225}, {80, 238}}, {pressure=0.3})
h:sketch({{236, 270}, {220, 200}, {205, 130}, {210, 70}}, {pressure=0.3})
h:sketch({{248, 272}, {280, 210}, {300, 150}, {290, 95}}, {pressure=0.3})
h:sketch({{254, 285}, {320, 255}, {380, 250}, {420, 270}}, {pressure=0.3})
-- spruces
for _, s in ipairs({{850, 506, 95}, {884, 510, 135}, {922, 508, 72}}) do
  h:sketch({{s[1] - s[3]*0.28, s[2]}, {s[1], s[2] - s[3]}, {s[1] + s[3]*0.28, s[2]}}, {pressure=0.25, smooth=false})
end
-- figure
h:line({{520, 530}, {520, 504}}, {pressure=0.3})
h:line({{528, 532}, {531, 502}}, {pressure=0.25})
-- path
h:sketch({{300, 714}, {390, 640}, {470, 575}, {515, 532}}, {pressure=0.18})

--@ chunk 3
sky1 = pile{{"smalt",3},{"lead white",3},{"bone black",0.25},{"red earth",0.1}, medium=0.15}
sky2 = pile{{"pale smalt",2},{"lead white",5},{"smalt",0.5},{"bone black",0.08}, medium=0.15}
sky3 = pile{{"lead white",7},{"pale smalt",1},{"yellow ochre",0.35}, medium=0.15}
sky4 = pile{{"lead white",8},{"yellow ochre",0.8},{"vermilion",0.07}, medium=0.15}
sky5 = pile{{"lead white",6},{"yellow ochre",1},{"vermilion",0.3},{"red earth",0.15}, medium=0.15}
local bands = {{sky1, -10, 125}, {sky2, 105, 235}, {sky3, 215, 335}, {sky4, 315, 405}, {sky5, 390, 485}}
for i, b in ipairs(bands) do
  work(rect(-10, b[2], 1020, b[3] - b[2]), {hand="body", pile=b[1], angle=0, angle_jitter=0.06,
     length={70, 170}, coverage=2.6, fill=true, pressure={0.5, 0.75}, seed=30+i})
end

--@ chunk 4
sky45 = pile{{"lead white",8},{"yellow ochre",0.7},{"vermilion",0.1},{"pale smalt",0.15}, medium=0.15}
work(rect(-10, 385, 1020, 75), {hand="body", pile=sky45, angle=0, angle_jitter=0.05,
     length={70, 170}, coverage=2.0, fill=true, pressure={0.5, 0.75}, seed=41})
blend(rect(-20, -20, 1040, 500), {angle=0})
blend(rect(-20, -20, 1040, 500), {angle=0})

--@ chunk 5
blend(rect(-20, 40, 1040, 220), {angle=0.03})

--@ chunk 6
print(wait(36*60)); for _,p in ipairs({{100,30},{500,150},{500,300},{800,440},{300,470}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 7
skyglaze = pile{{"smalt",3},{"bone black",0.25},{"lead white",0.6},{"red earth",0.08}, medium=0.8}
work(rect(-20, -20, 1040, 300), {hand="glaze", pile=skyglaze, angle=0, angle_jitter=0.03,
   load_at=function(x, y) local t = 1 - smoothstep(-10, 290, y); return 0.02 + 0.7 * t * t end,
   pressure={0.3, 0.5}, coverage=1.6, seed=71})
blend(rect(-20, -20, 1040, 320), {angle=0})

--@ chunk 8
woods = pile{{"lead white",4},{"smalt",1},{"pale smalt",1},{"vermilion",0.08},{"bone black",0.06}, medium=0.2}
local m = mask(function(x, y) if x > 330 and y > woodtop(x) + 3 and y < 474 then return 1 end return 0 end)
local n = 0
for i = 1, 140 do
  local x = rand(335, 1005)
  local r = rand(3, 8)
  m = m + ellipse(x, woodtop(x) + 3, r, r * rand(0.8, 1.2))
end
-- a few distant firs
for i = 1, 22 do
  local x = rand(350, 1000)
  local hh = rand(8, 18)
  local b = woodtop(x) + 4
  m = m + poly({{x - 3.5, b}, {x, b - hh}, {x + 3.5, b}})
end
woodsm = m:roughen(1.2, 6, 3)
work(woodsm, {hand="detail", pile=woods, load=0.8, pressure={0.5, 0.8}, coverage=3, fill=true, angle=0})

--@ chunk 9
churchp = pile{{"lead white",3},{"smalt",1},{"pale smalt",1},{"vermilion",0.1},{"bone black",0.12}, medium=0.2}
local tower = rect(634, 426, 12, 46)
local spire = poly({{632.5, 427}, {640, 396}, {647.5, 427}})
local nave = poly({{646, 472}, {646, 449}, {668, 441}, {692, 449}, {692, 472}})
local apse = poly({{692, 472}, {692, 452}, {700, 455}, {703, 462}, {703, 472}})
churchm = tower + spire + nave + apse
work(churchm, {hand="detail", pile=churchp, load=0.9, pressure={0.5, 0.8}, coverage=3.5, fill=true, angle=math.pi/2})

--@ chunk 10
function landtop(x) return math.min(knoll(x), horizon(x)) end
landm = below(landtop)
snowFar = pile{{"lead white",7},{"pale smalt",1},{"vermilion",0.06},{"yellow ochre",0.15}, medium=0.12}
snowMid = pile{{"lead white",5},{"pale smalt",1.2},{"smalt",0.4},{"vermilion",0.05}, medium=0.12}
snowNear = pile{{"lead white",4},{"smalt",1},{"pale smalt",0.5},{"bone black",0.1},{"red earth",0.05}, medium=0.12}
snowKnoll = pile{{"lead white",5},{"pale smalt",1},{"smalt",0.5},{"bone black",0.05},{"vermilion",0.04}, medium=0.12}
local kn = landm * mask(function(x, y) return (x < 470 and y < 505) and 1 or 0 end)
work(kn, {hand="body", pile=snowKnoll, angle=function(x, y) return 0.12 * math.cos((x - 190) / 120) end,
   length={30, 90}, coverage=2.6, fill=true, pressure={0.5, 0.75}, seed=101})
work(landm * rect(430, 400, 600, 125) - kn, {hand="body", pile=snowFar, angle=0, angle_jitter=0.04,
   length={50, 130}, coverage=2.6, fill=true, pressure={0.5, 0.75}, seed=102})
work(landm * rect(-10, 500, 1020, 105) - rect(-10,400,440,105), {hand="body", pile=snowMid, angle=0, angle_jitter=0.05,
   length={50, 140}, coverage=2.6, fill=true, pressure={0.5, 0.75}, seed=103})
work(rect(-10, 590, 1020, 140), {hand="body", pile=snowNear, angle=0, angle_jitter=0.07,
   length={50, 150}, coverage=2.6, fill=true, pressure={0.5, 0.75}, seed=104})
blend(landm * rect(-20, 480, 1040, 260), {angle=0})
blend(kn:grow(12) * landm, {angle=0.1})

--@ chunk 11
blend(landm * rect(400, 440, 160, 110), {angle=0}); blend(landm * rect(400, 440, 160, 110), {angle=0.2})

--@ chunk 12
print(wait(48*60)); for _,p in ipairs({{100,450},{500,520},{500,650},{800,700},{640,440}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 13
snowB1 = pile{{"lead white",5},{"pale smalt",1.5},{"vermilion",0.08},{"yellow ochre",0.1}, medium=0.15}
snowB2 = pile{{"lead white",3.5},{"smalt",1},{"pale smalt",0.6},{"bone black",0.12},{"vermilion",0.05}, medium=0.15}
snowB3 = pile{{"lead white",2.5},{"smalt",1.2},{"bone black",0.25},{"red earth",0.08}, medium=0.15}
knollP = pile{{"lead white",4},{"pale smalt",1.2},{"smalt",0.5},{"bone black",0.1},{"vermilion",0.05}, medium=0.15}
knm = landm * mask(function(x, y) local yy = 505 + 20*math.sin(x/70) ; return (x < 520 and y < yy - (x > 440 and (x-440)*0.4 or 0)) and 1 or 0 end)
work(knm, {hand="body", pile=knollP, angle=function(x, y) return 0.15 * math.cos((x - 190) / 120) end,
   length={30, 90}, coverage=2.4, fill=true, pressure={0.5, 0.75}, seed=131})
work(landm * rect(-10, 455, 1020, 90) - knm, {hand="body", pile=snowB1, angle=0, angle_jitter=0.04,
   length={50, 130}, coverage=2.4, fill=true, pressure={0.5, 0.75}, seed=132})
work(rect(-10, 530, 1020, 95) - knm, {hand="body", pile=snowB2, angle=0, angle_jitter=0.05,
   length={50, 140}, coverage=2.4, fill=true, pressure={0.5, 0.75}, seed=133})
work(rect(-10, 612, 1020, 120), {hand="body", pile=snowB3, angle=0, angle_jitter=0.07,
   length={50, 150}, coverage=2.4, fill=true, pressure={0.5, 0.75}, seed=134})
blend(landm * rect(-20, 440, 1040, 300), {angle=0})
blend(landm * rect(-20, 440, 1040, 300), {angle=0.08})

--@ chunk 14
print(wait(50*60)); for _,p in ipairs({{100,450},{500,520},{500,650},{800,700},{200,420}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 15
landglaze = pile{{"smalt",3},{"bone black",0.35},{"lead white",0.8},{"red earth",0.1}, medium=0.75}
work(landm * rect(-20, 440, 1040, 300), {hand="glaze", pile=landglaze, angle=0, angle_jitter=0.03,
   load_at=function(x, y) local t = smoothstep(480, 714, y); return 0.03 + 0.75 * t end,
   pressure={0.3, 0.5}, coverage=1.6, seed=151})
blend(landm * rect(-20, 440, 1040, 300), {angle=0})

--@ chunk 16
blend(landm * rect(-20, 500, 1040, 240), {angle=0.05})

--@ chunk 17
local b = brush{kind="round", width=8, point=0.7}; print(b:mark_width(0.2), b:mark_width(0.5), b:mark_width(1)); local r = brush{kind="round", width=4, point=1}; print(r:pressure_for(1.5), r:pressure_for(3)); local g = brush{kind="rigger", width=1.5, point=1}; print(g:mark_width(0.1), g:mark_width(0.6), g:mark_width(1))

--@ chunk 18
-- the oak's skeleton, drawn by hand: trunk and main boughs {points, widths}
oakB = {
 trunk = {pts={{238,432},{236,405},{239,375},{236,345},{238,318},{240,300}}, w={70,56,48,44,42,40}},
 L1 = {pts={{230,310},{205,290},{180,272},{150,258},{122,254},{95,258},{68,252},{44,258}}, w={22,18,15,12,10,8,6,4}},
 L2 = {pts={{230,300},{222,268},{214,238},{202,205},{186,176},{170,152},{158,126}}, w={20,17,15,12,10,8,6}},
 L3 = {pts={{240,296},{244,262},{247,228},{240,196},{242,165},{250,136},{246,104},{240,78}}, w={22,19,17,15,13,11,10,9}, dead=true},
 L4 = {pts={{248,298},{262,272},{280,244},{296,214},{306,186},{316,160},{314,130},{322,104}}, w={18,16,14,12,10,9,8,7}, dead=true},
 L5 = {pts={{250,318},{280,305},{312,296},{346,292},{378,297},{410,302},{442,296},{468,300}}, w={22,17,14,12,10,8,6,4}},
 L6 = {pts={{230,348},{210,340},{190,337},{176,341}}, w={11,9,8,8}, dead=true},
}
bark = pile{{"raw umber",2},{"bone black",1.2},{"smalt",0.5},{"lead white",0.35}, medium=0.1}
deadwood = pile{{"lead white",1.3},{"raw umber",1},{"bone black",0.55},{"smalt",0.2}, medium=0.1}
local function limbmask(b)
  return ribbon(b.pts, b.w):roughen(0.8, 5, 7)
end
oakLive = limbmask(oakB.trunk)
for _, k in ipairs({"L1","L2","L5"}) do oakLive = oakLive + limbmask(oakB[k]) end
oakDead = limbmask(oakB.L3)
for _, k in ipairs({"L4","L6"}) do oakDead = oakDead + limbmask(oakB[k]) end
oakDead = oakDead - oakLive
oakMain = oakLive + oakDead
work(oakLive, {hand="detail", pile=bark, load=1, pressure={0.6, 0.9}, coverage=3.5, fill=true,
   angle=function(x, y) return -math.pi/2 end, angle_jitter=0.3})
work(oakDead, {hand="detail", pile=deadwood, load=1, pressure={0.6, 0.9}, coverage=3.5, fill=true,
   angle=function(x, y) return -math.pi/2 end, angle_jitter=0.3})

--@ chunk 19
-- crotch: the trunk spreads into its boughs instead of ending in a knob
local crotch = poly({{214,322},{220,300},{226,284},{236,272},{246,266},{256,276},{264,292},{270,306},{262,322}}, true)
local flareL = poly({{206,300},{228,290},{236,318},{220,322}}, true)
local burr = ellipse(262, 350, 8, 12) + ellipse(214, 368, 7, 10) + ellipse(258, 392, 6, 8)
local waist = poly({{216,300},{212,330},{214,360},{220,380},{256,380},{262,360},{266,330},{262,300}}, true)
local m = (crotch + flareL + burr + waist):roughen(1.2, 6, 11)
work(m, {hand="detail", pile=bark, load=1, pressure={0.6, 0.9}, coverage=3.5, fill=true, angle=-math.pi/2, angle_jitter=0.4})
oakLive = oakLive + m

--@ chunk 20
-- secondary and higher branches, grown from the boughs by oak habit:
-- crooked, kinked at nodes, clustered forks at the tips, widths by the branch-area rule
oakSegs = {}
local function adiff(a, b) local d = a - b; while d > math.pi do d = d - 2*math.pi end; while d < -math.pi do d = d + 2*math.pi end; return d end
local function inCrown(x, y)
  local dx, dy = (x - 250) / 250, (y - 215) / 150
  return dx*dx + dy*dy
end
function growOak(x, y, a, w, depth, dead)
  local len = (dead and 7 or 10) * w + rand(10, 22)
  if w < 1.2 then len = rand(10, 24) end
  local n = math.max(2, math.floor(len / 9))
  local path, kink = {{x, y}}, (math.random() < 0.5) and 1 or -1
  for i = 1, n do
    kink = -kink
    a = a + kink * rand(0.08, 0.3) + randn(0, 0.12) - 0.10 * adiff(a, -math.pi/2) * (dead and 1.5 or 1)
    if math.sin(a) > 0.35 then a = a - 0.25 end
    local s = len / n
    x = x + math.cos(a) * s; y = y + math.sin(a) * s
    path[#path+1] = {x, y}
    if inCrown(x, y) > 1.05 and not dead then break end
  end
  local w1 = w * 0.7
  oakSegs[#oakSegs+1] = {pts=path, w0=w, w1=w1, dead=dead, depth=depth}
  if dead then
    if w > 3 and depth < 2 then growOak(x, y, a + rand(-0.6, 0.6), w * 0.6, depth + 1, true) end
    return
  end
  if w1 < 0.5 or depth > 6 then return end
  -- clustered fork at the tip (acrotony): 2-3 children
  local k = (math.random() < 0.4) and 3 or 2
  local shares, tot = {}, 0
  for i = 1, k do shares[i] = rand(0.5, 1.5); tot = tot + shares[i] end
  local spread = rand(0.5, 0.9)
  for i = 1, k do
    local cw = w1 * math.sqrt(shares[i] / tot) * 1.05
    local ca = a + (i - (k + 1) / 2) * spread + randn(0, 0.15)
    growOak(x, y, ca, cw, depth + 1, false)
  end
  -- a lateral from mid-segment
  if #path > 2 and w > 1.5 and math.random() < 0.7 then
    local p = path[math.floor(#path / 2) + 1]
    growOak(p[1], p[2], a + ((math.random() < 0.5) and -1 or 1) * rand(0.6, 1.0), w * rand(0.35, 0.5), depth + 1, false)
  end
end
-- spawn from the main boughs
local function along(b, every, dead, wscale)
  local pts, ws = b.pts, b.w
  local side = 1
  for i = 1, #pts - 1 do
    local x0, y0, x1, y1 = pts[i][1], pts[i][2], pts[i+1][1], pts[i+1][2]
    local a = math.atan(y1 - y0, x1 - x0)
    local segl = math.sqrt((x1-x0)^2 + (y1-y0)^2)
    local t = rand(0.2, 0.8)
    if i > 1 and math.random() < every then
      side = -side
      local w = (ws[i] + ws[i+1]) / 2
      local off = w / 2 * 0.8
      local px, py = lerp(x0, x1, t), lerp(y0, y1, t)
      local ca = a + side * rand(0.5, 1.0)
      px = px + math.cos(ca) * off * 0.5; py = py + math.sin(ca) * off * 0.5
      growOak(px, py, ca, w * rand(0.3, 0.55) * wscale, dead and 1 or 1, dead)
    end
  end
  -- continue the tip
  local n = #pts
  local a = math.atan(pts[n][2] - pts[n-1][2], pts[n][1] - pts[n-1][1])
  if not dead then
    growOak(pts[n][1], pts[n][2], a - 0.3, ws[n] * 0.75, 1, false)
    growOak(pts[n][1], pts[n][2], a + 0.4, ws[n] * 0.6, 1, false)
  end
end
along(oakB.L1, 0.95, false, 1)
along(oakB.L2, 0.95, false, 1)
along(oakB.L5, 0.95, false, 1)
along(oakB.L3, 0.5, true, 0.8)
along(oakB.L4, 0.5, true, 0.8)
-- a few boughs straight out of the trunk crotch into the live crown
growOak(226, 292, -2.3, 9, 1, false)
growOak(256, 296, -0.9, 8, 1, false)
growOak(218, 330, -2.8, 6, 1, false)
growOak(262, 326, -0.35, 6, 1, false)
local c = {0, 0, 0}
for _, s in ipairs(oakSegs) do
  if s.w0 >= 3 then c[1] = c[1] + 1 elseif s.w0 >= 1.2 then c[2] = c[2] + 1 else c[3] = c[3] + 1 end
end
print(#oakSegs, c[1], c[2], c[3])

--@ chunk 21
bThick = brush{kind="round", width=8, point=0.7}
bMid = brush{kind="round", width=4, point=1}
bTwig = brush{kind="rigger", width=1.5, point=1}
function paintSeg(s, minw, maxw)
  if s.w0 < minw or s.w0 >= maxw then return 0 end
  local b = (s.w0 >= 3) and bThick or ((s.w0 >= 1.2) and bMid or bTwig)
  local p = s.dead and deadwood or bark
  b:load(p, s.w0 >= 1.2 and 0.9 or 0.7)
  local p0 = math.max(b:pressure_for(s.w0), 0.15)
  local p1 = math.max(b:pressure_for(s.w1), 0.05)
  b:stroke(s.pts, {pressure={p0, p1}, ramps={0.0, 0.25}})
  return 1
end
local n = 0
for _, s in ipairs(oakSegs) do n = n + paintSeg(s, 1.2, 1000) end
print(n)

--@ chunk 22
local n = 0; for _, s in ipairs(oakSegs) do n = n + paintSeg(s, 0, 1.2) end; print(n)

--@ chunk 23
-- lower, drooping and horizontal branches under the big boughs, and a net of fine twiglets
oakSegs2 = {}
local function adiff(a, b) local d = a - b; while d > math.pi do d = d - 2*math.pi end; while d < -math.pi do d = d + 2*math.pi end; return d end
function growDroop(x, y, a, w, depth)
  local len = 9 * w + rand(12, 24)
  if w < 1.2 then len = rand(10, 22) end
  local n = math.max(2, math.floor(len / 9))
  local path, kink = {{x, y}}, (math.random() < 0.5) and 1 or -1
  for i = 1, n do
    kink = -kink
    -- droop first, then the tips turn up a little
    local g = (depth < 3) and 0.07 or -0.06
    a = a + kink * rand(0.08, 0.28) + randn(0, 0.1) + g * math.cos(a) * ((math.cos(a) > 0) and 1 or -1)
    x = x + math.cos(a) * len / n; y = y + math.sin(a) * len / n
    path[#path+1] = {x, y}
  end
  local w1 = w * 0.7
  oakSegs2[#oakSegs2+1] = {pts=path, w0=w, w1=w1, depth=depth}
  if w1 < 0.5 or depth > 6 or y > 395 then return end
  local k = (math.random() < 0.4) and 3 or 2
  local shares, tot = {}, 0
  for i = 1, k do shares[i] = rand(0.5, 1.5); tot = tot + shares[i] end
  for i = 1, k do
    growDroop(x, y, a + (i - (k + 1) / 2) * rand(0.5, 0.8) + randn(0, 0.12), w1 * math.sqrt(shares[i] / tot) * 1.05, depth + 1)
  end
end
-- from the undersides of L1 (going left) and L5 (going right)
for _, s in ipairs({{120, 257, math.pi - 0.1, 4}, {170, 268, math.pi - 0.35, 5}, {80, 256, math.pi + 0.05, 3},
                    {300, 300, 0.25, 5}, {360, 296, 0.15, 4}, {420, 302, 0.05, 3}, {205, 290, math.pi - 0.6, 4},
                    {228, 340, math.pi - 0.2, 3}, {262, 312, 0.5, 3.5}}) do
  growDroop(s[1], s[2], s[3], s[4], 1)
end
local n = 0
for _, s in ipairs(oakSegs2) do n = n + paintSeg(s, 0, 1000) end
print(#oakSegs2, n)

--@ chunk 24
local g = brush{kind="rigger", width=0.8, point=1}; print(g:mark_width(0.05), g:mark_width(0.3), g:mark_width(0.6), g:mark_width(1))

--@ chunk 25
-- the fine twiglets at every twig end: oak's crooked clustered shoots
twigP = pile{{"raw umber",2},{"bone black",1},{"lead white",0.8},{"smalt",0.4}, medium=0.3}
bFine = brush{kind="rigger", width=0.8, point=1}
local cnt = 0
local function twiglets(s)
  local pts = s.pts
  local n = #pts
  local x, y = pts[n][1], pts[n][2]
  local a = math.atan(pts[n][2] - pts[n-1][2], pts[n][1] - pts[n-1][1])
  local k = math.random(2, 4)
  for i = 1, k do
    local ca = a + rand(-0.9, 0.9)
    local L = rand(4, 10)
    local mx, my = x + math.cos(ca) * L * 0.5, y + math.sin(ca) * L * 0.5
    local ca2 = ca + rand(-0.35, 0.35)
    local ex, ey = mx + math.cos(ca2) * L * 0.5, my + math.sin(ca2) * L * 0.5
    if cnt % 6 == 0 then bFine:load(twigP, 0.55) end
    bFine:stroke({{x, y}, {mx, my}, {ex, ey}}, {pressure={0.55, 0.05}, ramps={0, 0.5}})
    cnt = cnt + 1
    -- a second tier on some
    if math.random() < 0.45 then
      local ca3 = ca2 + rand(-0.8, 0.8)
      bFine:stroke({{ex, ey}, {ex + math.cos(ca3) * L * 0.5, ey + math.sin(ca3) * L * 0.5}}, {pressure={0.35, 0.02}, ramps={0, 0.6}})
      cnt = cnt + 1
    end
  end
end
for _, s in ipairs(oakSegs) do if not s.dead and s.w1 < 0.62 then twiglets(s) end end
for _, s in ipairs(oakSegs2) do if s.w1 < 0.62 then twiglets(s) end end
print(cnt)

--@ chunk 26
print(wait(3*24*60)); for _,p in ipairs({{238,420},{238,360},{150,258},{500,600},{600,700},{245,150}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 27
-- the frozen mere
merePts = {{420,560},{470,551},{540,548},{620,547},{700,549},{780,548},{850,551},{905,557},{918,566},{895,578},{840,588},{760,596},{680,601},{600,601},{530,597},{470,590},{428,580},{410,570}}
merem = poly(merePts, true):roughen(1.5, 8, 21)
iceP = pile{{"lead white",2.2},{"smalt",1},{"pale smalt",1},{"bone black",0.22},{"vermilion",0.03}, medium=0.2}
iceGlow = pile{{"lead white",5},{"yellow ochre",0.45},{"vermilion",0.08},{"pale smalt",0.3}, medium=0.2}
work(merem, {hand="body", tool="filbert 5", pile=iceP, angle=0, angle_jitter=0.03, length={30, 90}, coverage=3, fill=true, clip=true, pressure={0.5, 0.8}, seed=271})
-- the glow of the horizon caught on the far half of the ice, in horizontal strokes
local glow = merem * mask(function(x, y) return (y < 566 + 6*math.sin(x/40)) and 1 or 0 end)
work(glow, {hand="body", tool="filbert 4", pile=iceGlow, angle=0, angle_jitter=0.01, length={40, 120}, coverage=1.3, clip=true, pressure={0.35, 0.6}, seed=272})
blend(merem, {angle=0})

--@ chunk 28
-- three young spruces on the far bank, right
spruceP = pile{{"Prussian blue",1},{"bone black",1.2},{"yellow ochre",0.8},{"raw umber",1},{"lead white",0.2}, medium=0.1}
sprTrunk = pile{{"raw umber",2},{"bone black",1},{"red earth",0.3}, medium=0.1}
spruces = {{cx=852, base=512, h=98}, {cx=884, base=516, h=140}, {cx=921, base=513, h=74}, {cx=958, base=518, h=46}}
bSpr = brush{kind="round", width=3, point=1}
bSprF = brush{kind="rigger", width=1.2, point=1}
sprTiers = {}
function paintSpruce(s)
  local top = s.base - s.h
  local tr = brush{kind="round", width=2.5, point=0.8}
  tr:load(sprTrunk, 0.8)
  tr:stroke({{s.cx, top + 4}, {s.cx + 0.5, s.base}}, {pressure={0.25, 0.7}})
  local y = top + 2
  local gap = 2.2
  local cnt = 0
  while y < s.base - 4 do
    local d = (y - top) / s.h
    local half = s.h * 0.29 * (0.25 + 0.75 * d) * rand(0.8, 1.12)
    if d < 0.08 then half = s.h * 0.05 end
    for _, side in ipairs({-1, 1}) do
      local L = half * rand(0.85, 1.1)
      local droop = L * rand(0.25, 0.45)
      local x0 = s.cx
      local pts = {{x0, y}, {x0 + side * L * 0.45, y + droop * 0.45}, {x0 + side * L * 0.85, y + droop * 0.95}, {x0 + side * L, y + droop * 0.85}}
      if cnt % 3 == 0 then bSpr:load(spruceP, 0.9) end
      bSpr:stroke(pts, {pressure={0.75, 0.25}, ramps={0.05, 0.4}})
      cnt = cnt + 1
      -- hanging fringe of second-order twigs under the branch (comb spruce)
      bSprF:load(spruceP, 0.7)
      local nf = math.floor(L / 3)
      for k = 1, nf do
        local t = k / (nf + 1)
        local fx = x0 + side * L * t
        local fy = y + droop * t * 1.0
        local fl = rand(2, 5) * (0.5 + d)
        bSprF:stroke({{fx, fy}, {fx + side * rand(0, 1), fy + fl}}, {pressure={0.6, 0.1}})
      end
      sprTiers[#sprTiers + 1] = {pts=pts, side=side, s=s}
    end
    gap = 2.2 + 5.5 * d
    y = y + gap * rand(0.8, 1.2)
  end
end
for _, s in ipairs(spruces) do paintSpruce(s) end
print(#sprTiers)

--@ chunk 29
-- fill the spruces into dense masses whose edges are the drooping tiers
local m = nil
for _, t in ipairs(sprTiers) do
  local p = t.pts
  local s = t.s
  local d = (p[1][2] - (s.base - s.h)) / s.h
  local th = 2.5 + 6 * d
  local x0, y0 = p[1][1], p[1][2]
  local tip = p[4]
  local w = poly({{x0, y0 - 1.5}, {p[2][1], p[2][2] - 1.2}, {p[3][1], p[3][2] - 0.6}, {tip[1], tip[2]},
                 {p[3][1] - t.side * 1.5, p[3][2] + th * 0.5}, {p[2][1], p[2][2] + th * 0.9}, {x0, y0 + th}})
  m = m and (m + w) or w
end
sprMask = m:roughen(0.7, 3, 5)
work(sprMask, {hand="detail", pile=spruceP, load=1, pressure={0.6, 0.9}, coverage=3.5, fill=true,
   angle=function(x, y) return 0.4 end, angle_jitter=0.6})

--@ chunk 30
-- leaders: a thin spire above each spruce, with the last small whorl
local r = brush{kind="round", width=2, point=1}
for _, s in ipairs(spruces) do
  local top = s.base - s.h
  r:load(spruceP, 0.8)
  r:stroke({{s.cx + 0.3, top + 8}, {s.cx, top - 1}, {s.cx - 0.2, top - 7}}, {pressure={0.8, 0.02}, ramps={0, 0.7}})
  for _, side in ipairs({-1, 1}) do
    r:stroke({{s.cx, top + 1}, {s.cx + side * 3, top + 2.5}}, {pressure={0.5, 0.05}})
    r:stroke({{s.cx, top + 5}, {s.cx + side * 4.5, top + 7}}, {pressure={0.6, 0.05}})
  end
end

--@ chunk 31
-- smooth the pagoda-like tops into cones with small drooping tier tips
local function cone(s, frac)
  local top = s.base - s.h
  local yb = top + s.h * frac
  local hw = s.h * 0.29 * (0.25 + 0.75 * frac) * 0.95
  local pts = {{s.cx, top - 2}}
  local n = 7
  for i = 1, n do
    local y = top + (yb - top) * i / n
    local w = hw * (i / n) * (i / n)^(-0.15)
    pts[#pts+1] = {s.cx + w, y + 1.5}
    pts[#pts+1] = {s.cx + w * 0.72, y + 0.2}
  end
  local left = {}
  for i = #pts, 2, -1 do left[#left+1] = {2 * s.cx - pts[i][1], pts[i][2]} end
  for _, p in ipairs(left) do pts[#pts+1] = p end
  return poly(pts):roughen(0.5, 2.5, 9)
end
local m = cone(spruces[2], 0.3) + cone(spruces[1], 0.3) + cone(spruces[3], 0.3) + cone(spruces[4], 0.3)
work(m, {hand="detail", pile=spruceP, load=1, pressure={0.6, 0.9}, coverage=3.5, fill=true, angle=0.4, angle_jitter=0.6})

--@ chunk 32
print(wait(30*60)); for _,p in ipairs({{238,420},{238,360},{600,575},{500,560},{884,450},{245,150}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 33
print(wait(3*24*60)); for _,p in ipairs({{238,420},{238,360},{600,575},{500,560},{884,450},{245,150},{150,258}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 34
iceGlaze = pile{{"smalt",2},{"Prussian blue",0.15},{"bone black",0.35},{"lead white",0.5}, medium=0.7}
work(merem, {hand="glaze", tool="filbert 10", pile=iceGlaze, angle=0, angle_jitter=0.01, clip=true,
   load_at=function(x, y) local t = smoothstep(552, 598, y); return 0.08 + 0.7 * t end,
   pressure={0.3, 0.5}, coverage=1.8, length={60, 160}, seed=341})
blend(merem, {angle=0})

--@ chunk 35
-- the knoll, restated: a clear crest against the glow, its face in cool shade, the oak's foot in the snow
local cx = {{-10,420},{0,418},{60,410},{120,403},{180,399},{240,405},{300,414},{360,426},{420,440},{480,455},{540,466},{600,469},{1010,469}}
function knollCrest(x)
  for i = 1, #cx - 1 do
    if x <= cx[i+1][1] then
      local t = (x - cx[i][1]) / (cx[i+1][1] - cx[i][1])
      t = t * t * (3 - 2 * t)
      return lerp(cx[i][2], cx[i+1][2], t) + 1.2 * math.sin(x / 11) + 0.8 * math.sin(x / 4.3)
    end
  end
  return 469
end
-- the drift at the oak's foot: a line across the trunk, banked up a little at its sides
function footLine(x)
  local d = (x - 238) / 40
  return 442 - 7 * math.exp(-d * d * 1.5) * 0 + 6 * d * d * 0 - 4 * math.exp(-((x - 205)/10)^2) - 5 * math.exp(-((x - 272)/10)^2)
end
knollShade = pile{{"lead white",3},{"pale smalt",1},{"smalt",0.6},{"bone black",0.12},{"vermilion",0.05}, medium=0.12}
local face = below(knollCrest) * above(function(x) return 520 end)
local trunkUp = oakLive * above(footLine)
knollFace = (face - trunkUp) * mask(function(x, y) return x < 620 and 1 or 0 end)
work(knollFace * mask(function(x,y) return y < 470 and 1 or 0 end), {hand="body", pile=knollShade,
   angle=function(x, y) return 0.1 * math.cos((x - 180) / 150) end,
   length={30, 80}, coverage=2.8, fill=true, pressure={0.5, 0.8}, seed=351, edge="found"})
-- let it fade into the plain below the knoll
local lowFade = knollFace * mask(function(x, y) return (y >= 462) and 1 or 0 end)
work(lowFade, {hand="body", pile=knollShade, angle=0.05, length={30, 80}, coverage=2.0, pressure={0.4, 0.65},
   load_at=function(x, y) return 0.7 * (1 - smoothstep(462, 510, y)) + 0.05 end, seed=352})
blend(knollFace * mask(function(x,y) return y > 430 and 1 or 0 end), {angle=0.05})

--@ chunk 36
-- distant woods: hazy bare crowns along the top of the silhouette, so the domes read as trees
woodsHaze = pile{{"lead white",3.5},{"smalt",1},{"pale smalt",1},{"vermilion",0.08},{"bone black",0.08}, medium=0.45}
local g = brush{kind="rigger", width=0.8, point=1}
local n = 0
for i = 1, 520 do
  local x = rand(500, 1000)
  if math.abs(x - 662) > 40 then
    local y0 = woodtop(x) + rand(3, 9)
    local L = rand(5, 14)
    local a = -math.pi/2 + randn(0, 0.25)
    if n % 8 == 0 then g:load(woodsHaze, 0.5) end
    g:stroke({{x, y0}, {x + math.cos(a) * L * 0.5, y0 + math.sin(a) * L * 0.5}, {x + math.cos(a + randn(0, 0.2)) * L, y0 + math.sin(a) * L}},
      {pressure={0.5, 0.02}, ramps={0, 0.6}})
    n = n + 1
  end
end
print(n)

--@ chunk 37
print(wait(36*60)); for _,p in ipairs({{238,440},{100,450},{884,450},{870,452},{600,580}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 38
-- paint the stray haze strokes out of the spruces
work(sprMask * rect(830, 425, 110, 45), {hand="detail", pile=spruceP, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, angle=0.4, angle_jitter=0.6, clip=true})

--@ chunk 39
-- the oak's foot: root flares running out into the snow
local rl = poly({{214,420},{206,434},{194,446},{180,455},{196,456},{214,451},{226,447}}, true)
local rr = poly({{262,420},{268,434},{280,446},{298,454},{284,456},{266,452},{252,447}}, true)
local rc = poly({{214,436},{216,452},{236,458},{258,454},{264,436}}, true)
local rs = poly({{230,448},{226,462},{240,463},{246,450}}, true)
footM = (rl + rr + rc + rs):roughen(1.0, 5, 13)
work(footM, {hand="detail", pile=bark, load=1, pressure={0.6, 0.9}, coverage=3.5, fill=true, angle=-math.pi/2, angle_jitter=0.5})

--@ chunk 40
-- a low drifted bank across the foreground, its face in shadow; the path cuts through it
function bankY(x)
  local y = 650 + 10 * math.sin(x / 130 + 0.5) + 4 * math.sin(x / 37) + 1.5 * math.sin(x / 9)
  y = y + 16 * math.exp(-((x - 335) / 38)^2)
  return y
end
bankM = below(bankY)
fgGlaze = pile{{"smalt",2.5},{"bone black",0.35},{"lead white",0.7},{"vermilion",0.06}, medium=0.72}
work(bankM, {hand="glaze", pile=fgGlaze, angle=function(x, y) return 0.05 * math.sin(x / 90) end, angle_jitter=0.05,
   load_at=function(x, y) local t = smoothstep(bankY(x), bankY(x) + 40, y); return 0.1 + 0.55 * t end,
   pressure={0.3, 0.5}, coverage=1.6, clip=true, seed=401})
blend(bankM, {angle=0})

--@ chunk 41
-- banks of the mere. far shore: the bank's shaded face as a thin dark lip over the ice
local far = {}
for x = 425, 912, 6 do
  -- the far edge of the ice, found from the outline points
  far[#far+1] = x
end
local function farEdge(x)
  -- interpolate the far (upper) edge of merePts
  local up = {{410,570},{420,560},{470,551},{540,548},{620,547},{700,549},{780,548},{850,551},{905,557},{918,566}}
  for i = 1, #up - 1 do
    if x <= up[i+1][1] then local t = (x - up[i][1]) / (up[i+1][1] - up[i][1]); return lerp(up[i][2], up[i+1][2], t) end
  end
  return 566
end
function nearEdge(x)
  local lo = {{410,570},{428,580},{470,590},{530,597},{600,601},{680,601},{760,596},{840,588},{895,578},{918,566}}
  for i = 1, #lo - 1 do
    if x <= lo[i+1][1] then local t = (x - lo[i][1]) / (lo[i+1][1] - lo[i][1]); return lerp(lo[i][2], lo[i+1][2], t) end
  end
  return 566
end
mereFar = farEdge
bankLip = pile{{"smalt",1.5},{"bone black",0.4},{"lead white",1},{"raw umber",0.3}, medium=0.2}
local lipPts, lipW = {}, {}
for x = 440, 900, 5 do lipPts[#lipPts+1] = {x, farEdge(x) + 1.2 + 0.6 * math.sin(x / 7)}; lipW[#lipW+1] = rand(1.4, 3.0) end
local lip = ribbon(lipPts, lipW):roughen(0.6, 4, 17)
work(lip, {hand="detail", pile=bankLip, load=0.8, pressure={0.5, 0.8}, coverage=3, fill=true, angle=0, clip=true})
-- near shore: snow lobes pushed out over the edge of the ice
nearSnow = pile{{"lead white",5},{"pale smalt",1.2},{"smalt",0.25},{"vermilion",0.05},{"yellow ochre",0.05}, medium=0.12}
local m = nil
for x = 415, 912, 7 do
  local e = nearEdge(x)
  local r = rand(4, 11)
  local l = ellipse(x + rand(-3, 3), e + rand(0, 3), r, r * rand(0.25, 0.45))
  m = m and (m + l) or l
end
nearBankM = (m:roughen(0.8, 5, 19) * below(function(x) return 540 end))
work(nearBankM, {hand="detail", pile=nearSnow, load=0.9, pressure={0.5, 0.8}, coverage=3, fill=true, angle=0, clip=true})

--@ chunk 42
blend(nearBankM:grow(4) - merem:shrink(2), {angle=0.0}); blend(nearBankM:grow(4) - merem:shrink(2), {angle=0.3})

--@ chunk 43
-- the walker, seen from behind, going toward the church
coatP = pile{{"bone black",1},{"raw umber",1},{"smalt",0.3},{"red earth",0.1}, medium=0.1}
local fx, fb = 478, 533
local coat = poly({{fx-3.2,fb-21},{fx+3.4,fb-21},{fx+4.2,fb-14},{fx+5.2,fb-5.5},{fx+3.5,fb-4.2},{fx-3.8,fb-4.4},{fx-5,fb-5.8},{fx-4.2,fb-14}}, true)
local shoulders = ellipse(fx, fb-20.5, 4.2, 2.2)
local head = ellipse(fx + 0.2, fb - 24.2, 2.1, 2.5)
local hat = ellipse(fx + 0.2, fb - 26.2, 3.0, 1.0) + ellipse(fx + 0.2, fb - 27.3, 1.9, 1.3)
local legL = poly({{fx-2.4,fb-5},{fx-0.6,fb-5},{fx-0.9,fb},{fx-2.6,fb}})
local legR = poly({{fx+0.8,fb-5},{fx+2.6,fb-5},{fx+2.9,fb-1.2},{fx+1.2,fb-1.2}})
local arm = poly({{fx+3,fb-19},{fx+5.2,fb-15},{fx+6.2,fb-12},{fx+4.8,fb-11.5},{fx+3.2,fb-15}}, true)
figM = coat + shoulders + head + hat + legL + legR + arm
work(figM, {hand="detail", tool={kind="round", width=1.2, point=0.8}, pile=coatP, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, angle=math.pi/2, clip=true})
-- the staff
local r = brush{kind="rigger", width=0.8, point=1}
r:load(coatP, 0.8)
r:stroke({{fx + 5.8, fb - 16}, {fx + 7.5, fb - 8}, {fx + 9.2, fb + 0.5}}, {pressure={0.9, 0.7}})

--@ chunk 44
print(wait(4*24*60)); for _,p in ipairs({{238,450},{195,452},{884,450},{150,258},{478,520},{600,598},{300,690}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 45
for _,p in ipairs({{95,258},{346,292},{410,302},{200,205},{250,300},{300,200},{320,255}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 46
-- deepen the oak's trunk and big boughs toward the evening silhouette; cooler and darker on the left of the trunk
barkDark = pile{{"bone black",1.2},{"raw umber",1},{"smalt",0.6},{"lead white",0.1}, medium=0.45}
local thick = (oakLive - footM:grow(0)) * above(function(x) return 441 end)
work(thick, {hand="detail", tool={kind="round", width=3, point=0.5}, pile=barkDark, load=0.9, pressure={0.5, 0.8}, coverage=2.5, fill=true, clip=true,
   angle=-math.pi/2, angle_jitter=0.4,
   load_at=function(x, y) return 0.5 + 0.4 * smoothstep(250, 210, x) end})
-- fissured bark on the trunk: long wavy dark furrows, and pale ridges catching the sky on the right flank
local r = brush{kind="round", width=1.6, point=1}
local fur = pile{{"bone black",1.5},{"raw umber",0.8},{"smalt",0.3}, medium=0.2}
local ridge = pile{{"lead white",2},{"raw umber",1},{"bone black",0.4},{"smalt",0.4}, medium=0.2}
for i = 1, 26 do
  local x0 = rand(210, 266)
  local y0 = rand(300, 440)
  local L = rand(14, 40)
  if oakLive:at(x0, y0) > 0.5 then
    local pts = {}
    for k = 0, 4 do pts[#pts+1] = {x0 + 1.5 * math.sin(k * 1.3 + i) + (k * 0.3), y0 - L * k / 4} end
    r:load(fur, 0.7)
    r:stroke(pts, {pressure={0.55, 0.2}, ramps={0.2, 0.3}, clip=oakLive})
  end
end
for i = 1, 16 do
  local x0 = rand(246, 268)
  local y0 = rand(305, 438)
  local L = rand(8, 22)
  if oakLive:at(x0, y0) > 0.5 and oakLive:at(x0 + 3, y0) < 0.5 then x0 = x0 - 3 end
  if oakLive:at(x0, y0) > 0.5 then
    r:load(ridge, 0.5)
    r:stroke({{x0, y0}, {x0 + 0.8, y0 - L / 2}, {x0 + 0.3, y0 - L}}, {pressure={0.35, 0.1}, ramps={0.3, 0.4}, clip=oakLive})
  end
end

--@ chunk 47
-- a lightning scar: the dead leader's bare wood torn down into the trunk, a dark rim of callus along it
local scar = poly({{236,262},{250,262},{252,280},{247,296},{245,315},{241,334},{238,350},{236,334},{234,312},{232,292},{233,276}}, true):roughen(1.2, 4, 23)
work(scar * oakLive, {hand="detail", tool={kind="round", width=1.5, point=0.6}, pile=deadwood, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=-math.pi/2, angle_jitter=0.3})
local r = brush{kind="round", width=1.8, point=1}
local fur = pile{{"bone black",1.5},{"raw umber",0.8},{"smalt",0.3}, medium=0.2}
r:load(fur, 0.8)
r:stroke({{235,264},{233,278},{232,293},{234,313},{237,334},{238,351}}, {pressure={0.6, 0.2}})
r:load(fur, 0.8)
r:stroke({{251,264},{253,281},{248,297},{246,316},{242,335},{238,351}}, {pressure={0.6, 0.2}})
-- grain in the dead wood
local g = brush{kind="rigger", width=0.8, point=1}
local grain = pile{{"raw umber",1},{"bone black",0.6},{"lead white",0.8}, medium=0.3}
for i = 1, 7 do
  g:load(grain, 0.5)
  local x = rand(237, 247)
  g:stroke({{x, 268}, {x - 1 + rand(-1, 1), 290}, {x - 3 + rand(-1, 1), 315}}, {pressure={0.4, 0.05}, clip=scar})
end

--@ chunk 48
-- model the dead limbs: shadowed left flank, grain, splintered broken tops
local deadShade = pile{{"raw umber",1},{"bone black",0.8},{"smalt",0.3},{"lead white",0.5}, medium=0.35}
local grain = pile{{"raw umber",1},{"bone black",0.6},{"lead white",0.8}, medium=0.3}
local r = brush{kind="round", width=4, point=0.8}
local g = brush{kind="rigger", width=0.8, point=1}
for _, k in ipairs({"L3", "L4", "L6"}) do
  local b = oakB[k]
  local left, mid = {}, {}
  for i = 1, #b.pts do
    local p = b.pts[i]
    local q = b.pts[math.min(i + 1, #b.pts)]
    local o = b.pts[math.max(i - 1, 1)]
    local a = math.atan(q[2] - o[2], q[1] - o[1])
    local nx, ny = math.sin(a), -math.cos(a)   -- normal pointing to the left of travel
    if nx > 0 then nx, ny = -nx, -ny end        -- keep it on the canvas-left side
    left[#left+1] = {p[1] + nx * b.w[i] * 0.28, p[2] + ny * b.w[i] * 0.28}
  end
  r:load(deadShade, 0.9)
  local wm = b.w[1] * 0.45
  r:stroke(left, {pressure={r:pressure_for(math.min(wm, 4)), r:pressure_for(b.w[#b.w] * 0.4)}, clip=oakDead})
  for j = 1, 4 do
    g:load(grain, 0.5)
    local off = rand(-0.3, 0.3)
    local pts = {}
    for i = 1, #b.pts do
      local p = b.pts[i]
      pts[#pts+1] = {p[1] + off * b.w[i] + rand(-0.4, 0.4), p[2]}
    end
    g:stroke(pts, {pressure={0.4, 0.1}, ramps={0.2, 0.3}, clip=oakDead})
  end
end
-- splintered tops of L3 and L4: jagged points of bare wood, dark in the break
local sp = poly({{234,82},{237,70},{239,76},{241,64},{244,74},{247,68},{248,82}}) + poly({{316,108},{318,95},{321,101},{324,90},{326,104},{329,108}})
work(sp, {hand="detail", tool={kind="round", width=1.2, point=0.8}, pile=deadwood, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=-math.pi/2})

--@ chunk 49
-- snow lying along the tops of the near-horizontal limbs, in broken lengths, heaped in the crotches
limbSnow = pile{{"lead white",6},{"pale smalt",1},{"smalt",0.15},{"vermilion",0.03}, medium=0.12}
local r = brush{kind="round", width=4, point=0.8}
local nS = 0
local function snowAlong(pts, w0, w1, prob)
  local n = #pts
  for i = 1, n - 1 do
    local p, q = pts[i], pts[i + 1]
    local dx, dy = q[1] - p[1], q[2] - p[2]
    local L = math.sqrt(dx * dx + dy * dy)
    local slope = math.abs(dy) / (math.abs(dx) + 1e-6)
    local w = lerp(w0, w1, (i - 0.5) / (n - 1))
    if slope < 0.75 and w >= 1.6 and math.random() < prob then
      local th = math.min(0.42 * w, 3.2) * (1 - slope * 0.8)
      local off = w / 2 - th * 0.35
      local t0, t1 = rand(0, 0.25), rand(0.7, 1)
      local a = {lerp(p[1], q[1], t0), lerp(p[2], q[2], t0) - off}
      local b = {lerp(p[1], q[1], t1), lerp(p[2], q[2], t1) - off}
      local m = {(a[1] + b[1]) / 2, (a[2] + b[2]) / 2 - th * 0.15}
      r:load(limbSnow, 0.7)
      r:stroke({a, m, b}, {pressure={r:pressure_for(th) * 0.6, r:pressure_for(th) * 0.5}, swell={0.6, 1.2, 0.5}, ramps={0.3, 0.35}})
      nS = nS + 1
    end
  end
end
for _, k in ipairs({"L1", "L5", "L2", "L6"}) do
  local b = oakB[k]
  for i = 1, #b.pts - 1 do snowAlong({b.pts[i], b.pts[i + 1]}, b.w[i], b.w[i + 1], 0.85) end
end
for _, s in ipairs(oakSegs) do snowAlong(s.pts, s.w0, s.w1, 0.55) end
for _, s in ipairs(oakSegs2) do snowAlong(s.pts, s.w0, s.w1, 0.55) end
print(nS)

--@ chunk 50
-- snow as a cushion sitting on the limb tops (masks), bulging a little above the bark's edge
local cm = nil
local function cushion(p, q, w, prob)
  local dx, dy = q[1] - p[1], q[2] - p[2]
  local slope = math.abs(dy) / (math.abs(dx) + 1e-6)
  if slope > 0.7 or w < 1.8 or math.random() > prob then return end
  local th = math.min(0.5 * w, 3.6) * (1 - slope * 0.7)
  local t0, t1 = rand(0.0, 0.3), rand(0.65, 1.0)
  local pts, ws = {}, {}
  local n = 6
  for k = 0, n do
    local t = lerp(t0, t1, k / n)
    local x, y = lerp(p[1], q[1], t), lerp(p[2], q[2], t)
    local taper = math.sin(math.pi * k / n) ^ 0.6
    pts[#pts+1] = {x, y - w / 2 + th * 0.1}
    ws[#ws+1] = math.max(0.3, th * taper)
  end
  local r = ribbon(pts, ws)
  cm = cm and (cm + r) or r
end
for _, k in ipairs({"L1", "L5", "L2", "L6"}) do
  local b = oakB[k]
  for i = 1, #b.pts - 1 do cushion(b.pts[i], b.pts[i + 1], (b.w[i] + b.w[i+1]) / 2 * 0.9, 0.9) end
end
for _, list in ipairs({oakSegs, oakSegs2}) do
  for _, s in ipairs(list) do
    for i = 1, #s.pts - 1 do
      local w = lerp(s.w0, s.w1, (i - 0.5) / (#s.pts - 1))
      cushion(s.pts[i], s.pts[i + 1], w, 0.5)
    end
  end
end
-- heaps in the main crotches
cm = cm + ellipse(214, 286, 5, 2.2) + ellipse(262, 296, 6, 2.4) + ellipse(230, 334, 4, 1.8)
limbSnowM = cm:roughen(0.4, 2.5, 29)
work(limbSnowM, {hand="detail", tool={kind="round", width=1.2, point=0.8}, pile=limbSnow, load=0.9, pressure={0.5, 0.8}, coverage=3.5, fill=true, clip=true, angle=0})

--@ chunk 51
print(wait(4*24*60)); for _,p in ipairs({{238,450},{195,452},{884,450},{478,520},{600,598},{430,575},{214,286}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 52
-- snow drifted over the oak's foot
function driftLine(x)
  return 447 + 1.8 * math.sin(x / 5.5) + 1.2 * math.sin(x / 2.7 + 1) - 3 * math.exp(-((x - 238) / 14)^2) + 4 * math.exp(-((x - 196) / 8)^2) + 3 * math.exp(-((x - 285) / 8)^2)
end
local dm = below(driftLine) * rect(160, 425, 160, 50)
driftM = dm:roughen(0.7, 4, 31)
work(driftM, {hand="detail", tool={kind="round", width=2, point=0.6}, pile=knollShade, load=1, pressure={0.6, 0.9}, coverage=3.5, fill=true, angle=0, clip=true})
-- the lit top of the drift
local r = brush{kind="round", width=2.5, point=0.8}
local pts = {}
for x = 176, 302, 3 do pts[#pts+1] = {x, driftLine(x) + 0.8} end
r:load(limbSnow, 0.6)
r:stroke(pts, {pressure={0.35, 0.3}, swell={0.7, 1, 1.2, 0.8, 1, 0.6}, clip=driftM})

--@ chunk 53
-- glaze the root flares down into the trunk's tone; model them with a dark underside
local roots = footM - driftM
work(roots, {hand="detail", tool={kind="round", width=3, point=0.5}, pile=barkDark, load=0.9, pressure={0.5, 0.8}, coverage=3, fill=true, clip=true, angle=-math.pi/2, angle_jitter=0.4})
local r = brush{kind="round", width=1.6, point=1}
local fur = pile{{"bone black",1.5},{"raw umber",0.8},{"smalt",0.3}, medium=0.2}
for _, pts in ipairs({{{214,424},{206,437},{194,447}}, {{262,424},{270,437},{284,447}}, {{238,430},{237,446}}}) do
  r:load(fur, 0.7); r:stroke(pts, {pressure={0.5, 0.15}, clip=roots})
end

--@ chunk 54
-- the mere: neutralise the cyan toward a violet-grey ice, and bring the bright near rim down into the snow
iceNeutral = pile{{"smalt",1},{"red earth",0.25},{"bone black",0.25},{"lead white",1.2},{"vermilion",0.05}, medium=0.75}
work(merem, {hand="glaze", tool="filbert 10", pile=iceNeutral, angle=0, angle_jitter=0.01, clip=true,
   load_at=function(x, y) return 0.25 + 0.35 * smoothstep(555, 600, y) end,
   pressure={0.3, 0.5}, coverage=1.5, length={60, 160}, seed=541})
blend(merem, {angle=0})
rimGlaze = pile{{"smalt",1.5},{"pale smalt",1},{"bone black",0.15},{"lead white",1.5},{"vermilion",0.05}, medium=0.75}
local rim = mask(function(x, y)
  if x < 400 or x > 930 then return 0 end
  local e = nearEdge(x)
  if y > e - 5 and y < e + 16 then return 1 end
  return 0
end) - merem:shrink(3)
rimM = rim
work(rim, {hand="glaze", tool="filbert 6", pile=rimGlaze, angle=0, angle_jitter=0.05, clip=true, load=0.45,
   pressure={0.3, 0.5}, coverage=1.6, length={30, 80}, seed=542})
blend(rim:grow(4) - merem:shrink(3), {angle=0})

--@ chunk 55
print(wait(3*24*60)); for _,p in ipairs({{600,575},{430,575},{600,605},{700,590},{478,520},{214,286}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 56
-- snow blown over the ice in long thin drifts, and a snowy tongue running in from the right shore
iceSnow = pile{{"lead white",5},{"pale smalt",1.3},{"smalt",0.3},{"vermilion",0.06},{"bone black",0.03}, medium=0.25}
local m = nil
for i = 1, 16 do
  local y = rand(556, 598)
  local x = rand(440, 880)
  local rx = rand(12, 60) * (0.6 + (y - 550) / 60)
  local ry = rand(0.8, 2.2) * (0.6 + (y - 550) / 50)
  local e = ellipse(x, y, rx, ry)
  m = m and (m + e) or e
end
m = m + poly({{930,560},{900,562},{870,566},{842,572},{828,576},{850,580},{884,578},{915,575}}, true)
m = m + poly({{405,568},{430,566},{462,570},{478,575},{455,578},{425,578}}, true)
iceSnowM = (m:roughen(1.2, 7, 37) * merem:grow(3))
work(iceSnowM, {hand="detail", tool={kind="round", width=2, point=0.5}, pile=iceSnow, load=0.7, pressure={0.4, 0.7}, coverage=2.5, fill=true, clip=true, angle=0, angle_jitter=0.05})
-- let the drifts' edges go soft against the ice
blend(iceSnowM:grow(3) * merem, {angle=0})

--@ chunk 57
blend(merem:grow(2), {angle=0.02}); blend(merem:grow(2), {angle=-0.03}); blend(merem:grow(2), {angle=0})

--@ chunk 58
print(wait(2*24*60)); for _,p in ipairs({{600,575},{450,575},{700,590},{880,572}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 59
-- wind-cleared ice: long dark patches between the pale drift streaks
darkIce = pile{{"smalt",2},{"bone black",0.4},{"red earth",0.1},{"lead white",0.6},{"Prussian blue",0.05}, medium=0.6}
local m = nil
local rows = {{556, 0.9}, {562, 1.0}, {569, 1.1}, {576, 1.2}, {583, 1.3}, {590, 1.3}}
for _, rw in ipairs(rows) do
  local x = rand(420, 470)
  while x < 900 do
    local rx = rand(25, 90) * rw[2]
    local ry = rand(1.6, 3.4) * rw[2]
    local e = ellipse(x + rx, rw[1] + rand(-2, 2), rx, ry)
    m = m and (m + e) or e
    x = x + 2 * rx + rand(8, 40)
  end
end
darkIceM = (m:roughen(0.9, 6, 41) * merem:shrink(4))
work(darkIceM, {hand="detail", tool={kind="round", width=2.5, point=0.4}, pile=darkIce, load=0.6, pressure={0.4, 0.7}, coverage=2.5, fill=true, clip=true, angle=0, angle_jitter=0.03})

--@ chunk 60
blend(merem:shrink(2), {angle=0.0}); blend(merem:shrink(2), {angle=0.04})

--@ chunk 61
-- reed beds at the ends of the mere and sparse reeds along the shores: stems bent by the snow, plumes, a few blades
reedP = pile{{"yellow ochre",2},{"raw umber",1.2},{"lead white",0.6},{"red earth",0.2}, medium=0.2}
reedDark = pile{{"raw umber",2},{"bone black",0.5},{"yellow ochre",0.6}, medium=0.2}
local g = brush{kind="rigger", width=1.2, point=1}
local f = brush{kind="rigger", width=0.8, point=1}
local cnt = 0
function reed(x, y, h, lean, pile)
  local a = -math.pi / 2 + lean + randn(0, 0.08)
  local pts = {{x, y}}
  local px, py = x, y
  local broken = math.random() < 0.18
  local n = 5
  for k = 1, n do
    local s = h / n
    if broken and k == 4 then a = a + ((math.random() < 0.5) and -1 or 1) * rand(0.9, 1.6) end
    a = a + 0.04 * lean * 3
    px, py = px + math.cos(a) * s, py + math.sin(a) * s
    pts[#pts+1] = {px, py}
  end
  if cnt % 5 == 0 then g:load(pile, 0.7) end
  g:stroke(pts, {pressure={0.55, 0.12}, ramps={0, 0.4}})
  cnt = cnt + 1
  -- plume: a small drooping feathered tuft
  if not broken and math.random() < 0.6 then
    f:load(reedDark, 0.6)
    for j = 1, 4 do
      local da = rand(0.3, 1.2)
      f:stroke({{px, py}, {px + math.cos(a + da) * 2.5 + 1, py + math.sin(a + da) * 2.5}, {px + math.cos(a + da) * 4 + 1.5, py + 2 + j * 0.6}}, {pressure={0.5, 0.05}})
    end
  end
  -- a leaf blade on some
  if math.random() < 0.35 then
    local i = math.random(2, 4)
    local q = pts[i]
    local side = (math.random() < 0.5) and -1 or 1
    f:load(pile, 0.6)
    f:stroke({q, {q[1] + side * rand(3, 6), q[2] - rand(1, 3)}, {q[1] + side * rand(6, 10), q[2] + rand(0, 4)}}, {pressure={0.6, 0.02}})
  end
end
function reedBed(cx, cy, wx, n, hmin, hmax, lean)
  for i = 1, n do
    local x = cx + randn(0, wx)
    local y = cy + rand(-3, 3)
    local d = math.abs(x - cx) / (2 * wx)
    reed(x, y, rand(hmin, hmax) * (1 - 0.5 * d), lean + randn(0, 0.1), (math.random() < 0.5) and reedP or reedDark)
  end
end
reedBed(418, 572, 14, 45, 14, 32, 0.12)
reedBed(452, 563, 10, 16, 10, 22, 0.1)
reedBed(912, 566, 12, 38, 14, 30, 0.15)
reedBed(870, 556, 10, 14, 8, 18, 0.12)
-- sparse along the far shore
for i = 1, 26 do local x = rand(480, 850); reed(x, mereFar(x) + 1, rand(5, 13), 0.12 + randn(0, 0.1), (math.random() < 0.5) and reedP or reedDark) end
print(cnt)

--@ chunk 62
-- the trodden path, from the walker back down to us through the gap in the bank
pathC = {{481,536},{462,542},{440,549},{412,558},{385,569},{364,585},{350,605},{342,630},{337,655},{333,685},{330,720}}
local ws = {}
for i, p in ipairs(pathC) do ws[i] = 2 + 30 * ((p[2] - 536) / 184)^1.4 end
pathM = ribbon(pathC, ws):roughen(1.0, 6, 43)
pathP = pile{{"smalt",1.5},{"pale smalt",1},{"bone black",0.2},{"lead white",1.5},{"vermilion",0.05}, medium=0.7}
work(pathM, {hand="detail", tool={kind="round", width=3, point=0.4}, pile=pathP, load=0.45, pressure={0.35, 0.6}, coverage=2, fill=true, clip=true, angle=function(x,y) return 1.2 end})
blend(pathM:grow(2), {angle=1.2})

--@ chunk 63
-- the young moon low over the glow, lit from below right; the evening star
moonP = pile{{"lead white",6},{"yellow ochre",0.25},{"chrome yellow",0.05}, medium=0.1}
local mx, my, mr = 752, 196, 8.5
moonM = (ellipse(mx, my, mr, mr) - ellipse(mx - 2.6, my - 2.4, mr * 0.93, mr * 0.93)):soften(0.3)
work(moonM, {hand="detail", tool={kind="round", width=1, point=0.8}, pile=moonP, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=2.4})
-- earthshine: the dark of the disc just paler than the sky
local es = pile{{"lead white",4},{"pale smalt",1.2},{"smalt",0.3},{"yellow ochre",0.1}, medium=0.6}
work(ellipse(mx, my, mr - 0.3, mr - 0.3) - moonM:grow(0.3), {hand="detail", tool={kind="round", width=1.5, point=0.5}, pile=es, load=0.3, pressure={0.3, 0.5}, coverage=1.5, clip=true, angle=0})
local r = brush{kind="round", width=1.2, point=0.6}
r:load(moonP, 0.9)
r:touch(662, 292, {pressure=0.45})

--@ chunk 64
-- deepen the upper sky toward the zenith of a winter dusk, keeping the young moon clear
skyGlaze2 = pile{{"smalt",3},{"bone black",0.3},{"red earth",0.12},{"vermilion",0.04},{"lead white",0.4}, medium=0.8}
local m = rect(-20, -20, 1040, 360) - moonM:grow(1.2)
work(m, {hand="glaze", pile=skyGlaze2, angle=0, angle_jitter=0.03, clip=true,
   load_at=function(x, y) local t = 1 - smoothstep(-10, 340, y); return 0.02 + 0.75 * t * t end,
   pressure={0.3, 0.5}, coverage=1.6, seed=641})
blend(m, {angle=0})
blend(m, {angle=0.05})

--@ chunk 65
print(wait(3*24*60)); for _,p in ipairs({{420,565},{350,620},{752,196},{500,100},{884,450},{600,575}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 66
-- snow lying on the spruce tiers
sprSnow = pile{{"lead white",5},{"pale smalt",1.2},{"smalt",0.25},{"vermilion",0.03}, medium=0.12}
local m = nil
for _, t in ipairs(sprTiers) do
  if math.random() < 0.8 then
    local p = t.pts
    local s = t.s
    local d = (p[1][2] - (s.base - s.h)) / s.h
    local k = 0.8 + 1.6 * d
    local t0 = rand(0.1, 0.35)
    local pts, ws = {}, {}
    for j = 0, 6 do
      local u = lerp(t0, rand(0.85, 1.0), j / 6)
      local x = lerp(p[1][1], p[4][1], u)
      -- follow the droop of the tier
      local y = p[1][2] + (p[4][2] - p[1][2]) * (1 - (1 - u) ^ 1.6) - 1.6 - 0.5 * d
      pts[#pts+1] = {x, y}
      ws[#ws+1] = math.max(0.4, k * math.sin(math.pi * (j + 0.5) / 7) ^ 0.7 * rand(0.7, 1.2))
    end
    local r = ribbon(pts, ws)
    m = m and (m + r) or r
  end
end
sprSnowM = m:roughen(0.4, 2, 47)
work(sprSnowM, {hand="detail", tool={kind="round", width=1.2, point=0.8}, pile=sprSnow, load=0.9, pressure={0.5, 0.8}, coverage=3.5, fill=true, clip=true, angle=0})

--@ chunk 67
-- crows: three flying in toward the oak, two sitting on its limbs
crowP = pile{{"bone black",1.5},{"raw umber",0.5},{"smalt",0.3}, medium=0.1}
local function flying(x, y, s, up, dir)
  -- body
  local b = ellipse(x, y, 2.4 * s, 0.9 * s) + ellipse(x + dir * 2.4 * s, y - 0.2 * s, 0.8 * s, 0.75 * s)
  local tail = poly({{x - dir * 1.8 * s, y - 0.5 * s}, {x - dir * 4.2 * s, y - 0.9 * s}, {x - dir * 4.4 * s, y + 0.8 * s}, {x - dir * 1.8 * s, y + 0.5 * s}})
  -- wings: bent at the wrist, fingered tips
  local wy = up and -1 or 0.6
  local w1 = poly({{x - 1.2 * s, y - 0.3 * s}, {x - 0.5 * s + dir * 1.0 * s, y + wy * 2.5 * s}, {x - dir * 1.5 * s, y + wy * 5.5 * s}, {x - dir * 2.8 * s, y + wy * 5.0 * s}, {x - dir * 1.2 * s, y + wy * 2.2 * s}, {x - dir * 1.6 * s, y + 0.3 * s}}, true)
  local w2 = poly({{x + 0.6 * s, y - 0.2 * s}, {x + dir * 2.0 * s, y + wy * 1.6 * s}, {x + dir * 1.4 * s, y + wy * 4.0 * s}, {x + dir * 0.2 * s, y + wy * 3.6 * s}, {x - dir * 0.2 * s, y + 0.2 * s}}, true)
  return b + tail + w1 + w2
end
local function perched(x, y, s, dir)
  local b = ellipse(x, y - 2.2 * s, 1.5 * s, 2.3 * s)
  local h = ellipse(x + dir * 0.9 * s, y - 4.8 * s, 1.0 * s, 0.95 * s)
  local beak = poly({{x + dir * 1.7 * s, y - 5.1 * s}, {x + dir * 3.0 * s, y - 4.8 * s}, {x + dir * 1.7 * s, y - 4.5 * s}})
  local tail = poly({{x - dir * 0.6 * s, y - 1.0 * s}, {x - dir * 2.0 * s, y + 2.4 * s}, {x - dir * 0.9 * s, y + 2.6 * s}, {x + dir * 0.4 * s, y - 0.6 * s}})
  return b + h + beak + tail
end
crowM = flying(566, 214, 1.25, true, -1) + flying(602, 236, 1.05, false, -1) + flying(528, 190, 0.9, true, -1)
      + perched(392, 296, 1.1, 1) + perched(318, 128, 1.0, -1) + perched(104, 252, 1.0, -1)
work(crowM, {hand="detail", tool={kind="round", width=1, point=0.8}, pile=crowP, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=0})
-- the evening star, restated
local r = brush{kind="round", width=1.5, point=0.6}
r:load(moonP, 1)
r:touch(662, 292, {pressure=0.7})

--@ chunk 68
-- dead grass through the snow: tufts thickening toward us, thinning into the distance
grassP = pile{{"yellow ochre",1.5},{"raw umber",1.5},{"red earth",0.3},{"lead white",0.3}, medium=0.2}
grassD = pile{{"raw umber",2},{"bone black",0.6},{"yellow ochre",0.5}, medium=0.2}
local g1 = brush{kind="rigger", width=1.2, point=1}
local g0 = brush{kind="rigger", width=0.8, point=1}
local cnt = 0
function tuft(x, y, h, n)
  local s = (h > 10) and g1 or g0
  for i = 1, n do
    local a = -math.pi / 2 + randn(0, 0.35) + 0.1
    local L = h * rand(0.4, 1.0)
    local bend = randn(0, 0.25)
    local bx = x + randn(0, h * 0.08)
    local pts = {{bx, y}}
    local px, py, aa = bx, y, a
    for k = 1, 3 do
      aa = aa + bend
      px, py = px + math.cos(aa) * L / 3, py + math.sin(aa) * L / 3
      pts[#pts+1] = {px, py}
    end
    if cnt % 6 == 0 then s:load((math.random() < 0.55) and grassP or grassD, 0.65) end
    s:stroke(pts, {pressure={0.5, 0.03}, ramps={0, 0.55}})
    cnt = cnt + 1
  end
end
local avoid = pathM:grow(3) + merem:grow(4) + oakLive + footM
-- foreground bank: crest tufts and scattered
for i = 1, 70 do
  local x = rand(0, 1000)
  local y = bankY(x) + rand(-3, 30)
  if avoid:at(x, y) < 0.5 then tuft(x, y, rand(8, 20) * (0.7 + (y - 640) / 100), math.random(4, 10)) end
end
-- middle ground: small and sparse
for i = 1, 120 do
  local x = rand(0, 1000)
  local y = rand(505, 640)
  if avoid:at(x, y) < 0.5 then tuft(x, y, 2 + (y - 500) / 140 * 10, math.random(2, 6)) end
end
-- knoll crest and face
for i = 1, 40 do
  local x = rand(0, 480)
  local y = knollCrest(x) + rand(1, 40)
  if avoid:at(x, y) < 0.5 then tuft(x, y, rand(2, 5), math.random(2, 5)) end
end
print(cnt)

--@ chunk 69
-- grass in clumps and drifts rather than sprinkles: dense stubble in the foreground corners, along the bank, by the reeds
local avoid = pathM:grow(2) + merem:grow(3) + oakLive + footM
local function patch(cx, cy, rx, ry, n, hmin, hmax)
  for i = 1, n do
    local x = cx + randn(0, rx)
    local y = cy + randn(0, ry)
    if avoid:at(x, y) < 0.5 and y < 716 then tuft(x, y, rand(hmin, hmax) * (0.6 + (y - 600) / 120), math.random(4, 11)) end
  end
end
patch(90, 690, 70, 14, 70, 12, 26)
patch(200, 700, 50, 10, 35, 12, 24)
patch(20, 660, 25, 12, 20, 10, 20)
patch(880, 700, 90, 10, 55, 12, 24)
patch(640, 705, 40, 8, 20, 10, 20)
-- along the bank crest, intermittent
for x = 0, 1000, 14 do
  if math.sin(x / 47) + 0.5 * math.sin(x / 13) > 0.3 then
    local y = bankY(x) + rand(0, 4)
    if avoid:at(x, y) < 0.5 then tuft(x, y, rand(6, 12), math.random(3, 8)) end
  end
end
-- by the reeds at the ends of the mere and along the near shore
patch(395, 590, 14, 6, 18, 5, 10)
patch(935, 580, 12, 6, 16, 5, 10)
patch(560, 612, 20, 3, 10, 3, 6)
patch(780, 604, 20, 3, 10, 3, 6)
print("ok")

--@ chunk 70
print(wait(3*24*60)); for _,p in ipairs({{90,690},{880,700},{884,450},{566,214},{420,565},{700,640}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 71
-- calm the zebra snow on the spruces: a dark transparent glaze, lighter toward the tips where the snow catches the sky
sprGlaze = pile{{"Prussian blue",0.4},{"bone black",1},{"raw umber",0.5},{"smalt",0.3}, medium=0.75}
local m = (sprMask + sprSnowM):grow(0.8)
work(m, {hand="detail", tool={kind="round", width=3, point=0.4}, pile=sprGlaze, load=0.4, pressure={0.35, 0.55}, coverage=2, fill=true, clip=true, angle=0.3, angle_jitter=0.5,
   load_at=function(x, y)
     local best = 1
     for _, s in ipairs(spruces) do
       local d = math.abs(x - s.cx) / (s.h * 0.29 + 1)
       if d < best then best = d end
     end
     return 0.5 - 0.3 * best
   end})

--@ chunk 72
-- footprints down the path, and the trough's shaded left wall
fpP = pile{{"smalt",1.5},{"bone black",0.3},{"lead white",1},{"vermilion",0.04}, medium=0.3}
local function pathAt(t)
  local n = #pathC
  local f = t * (n - 1) + 1
  local i = math.min(math.floor(f), n - 1)
  local u = f - i
  local p, q = pathC[i], pathC[i + 1]
  return lerp(p[1], q[1], u), lerp(p[2], q[2], u), math.atan(q[2] - p[2], q[1] - p[1])
end
local m = nil
local t, side = 0.005, 1
while t < 0.97 do
  local x, y, a = pathAt(t)
  local sc = 0.5 + 2.6 * ((y - 536) / 184)^1.3
  local nx, ny = -math.sin(a), math.cos(a)
  local fx, fy = x + nx * side * sc * 0.9, y + ny * side * sc * 0.9
  local e = ellipse(fx, fy, sc * 0.75, sc * 0.45)
  m = m and (m + e) or e
  side = -side
  t = t + 0.004 + 0.022 * sc / 3
end
fpM = m:roughen(0.3, 1.5, 53)
work(fpM, {hand="detail", tool={kind="round", width=1, point=0.6}, pile=fpP, load=0.7, pressure={0.5, 0.8}, coverage=3, fill=true, clip=true, angle=0})
-- the left wall of the trough in shade, a soft blue line
local r = brush{kind="round", width=3, point=0.8}
local pts = {}
for i, p in ipairs(pathC) do
  local w = 2 + 30 * ((p[2] - 536) / 184)^1.4
  pts[#pts+1] = {p[1] - w * 0.46, p[2] + 0.5}
end
r:load(fpP, 0.5)
r:stroke(pts, {pressure={0.12, 0.45}, ramps={0.2, 0.1}})

--@ chunk 73
print(wait(4*24*60)); for _,p in ipairs({{90,690},{880,700},{200,700},{566,214},{420,565},{350,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 74
-- a cool dusk glaze over the near ground: darkens the snow toward us and dulls the golden grass
fgGlaze2 = pile{{"smalt",2.5},{"bone black",0.3},{"red earth",0.12},{"lead white",0.5}, medium=0.78}
local m = landm * mask(function(x, y) return y > 505 and 1 or 0 end) - sprMask:grow(2) - figM:grow(1)
work(m, {hand="glaze", pile=fgGlaze2, angle=0, angle_jitter=0.04, clip=true,
   load_at=function(x, y) local t = smoothstep(505, 714, y); return 0.04 + 0.6 * t ^ 1.3 end,
   pressure={0.3, 0.5}, coverage=1.6, seed=741})
blend(m, {angle=0})

--@ chunk 75
print(nearEdge(500), nearEdge(660), nearEdge(850), mereFar(500), mereFar(660), mereFar(850))
print(drying(600,570), drying(660,600), drying(700,560))

--@ chunk 76
print(type(oakDead), type(oakSegs2), oakSegs2 and #oakSegs2)
local s = oakSegs2 and oakSegs2[1]
if type(s)=="table" then for k,v in pairs(s) do print(k, type(v)=="table" and #v or v) end end
print(drying(240,200), drying(300,150), drying(230,100))

--@ chunk 77
-- the dead limbs are too pale and flat against the dusk: glaze them down, and shade their right flanks
deadGlaze = pile{{"raw umber",1},{"smalt",0.6},{"bone black",0.3},{"lead white",0.6}, medium=0.75}
work(oakDead, {hand="detail", tool={kind="round", width=2, point=0.5}, pile=deadGlaze, load=0.3, pressure={0.35, 0.5}, coverage=2, fill=true, clip=true, angle=-1.4, angle_jitter=0.2})
local rightFlank = oakDead * mask(function(x, y) return (oakDead:at(x + 2.2, y) < 0.4) and 1 or 0 end)
work(rightFlank:soften(0.5), {hand="detail", tool={kind="round", width=1.2, point=0.5}, pile=deadGlaze, load=0.55, pressure={0.4, 0.6}, coverage=2.5, fill=true, clip=true, angle=-1.4})

--@ chunk 78
-- a few clusters of withered leaves still hanging on the lower twigs, as young oaks keep them
leafP = pile{{"red earth",1.2},{"raw umber",1.2},{"yellow ochre",0.4},{"bone black",0.1}, medium=0.15}
leafL = pile{{"yellow ochre",1},{"red earth",0.8},{"raw umber",0.5},{"lead white",0.3}, medium=0.15}
local cands = {}
for _, s in ipairs(oakSegs) do
  local p = s.pts[#s.pts]
  if s.depth and s.depth >= 3 and p[2] > 230 and p[2] < 390 and oakDead:at(p[1], p[2]) < 0.2 then cands[#cands+1] = p end
end
print(#cands)
local m = nil
local picked = 0
local used = {}
for i = 1, 400 do
  if picked >= 9 then break end
  local p = cands[math.random(#cands)]
  local ok = true
  for _, q in ipairs(used) do if math.abs(q[1]-p[1]) + math.abs(q[2]-p[2]) < 40 then ok = false end end
  if ok then
    used[#used+1] = p
    picked = picked + 1
    for k = 1, math.random(4, 8) do
      local lx, ly = p[1] + randn(0, 2.2), p[2] + randn(1.5, 1.8)
      local e = ellipse(lx, ly, rand(0.9, 1.6), rand(0.5, 0.9))
      m = m and (m + e) or e
    end
  end
end
leafM = m:roughen(0.3, 1, 61)
work(leafM, {hand="detail", tool={kind="round", width=1, point=0.6}, pile=leafP, load=0.9, pressure={0.5, 0.8}, coverage=3, fill=true, clip=true, angle=0.5, angle_jitter=1})
work(leafM:shrink(0.4) * mask(function(x, y) return (math.sin(x * 1.7 + y * 2.3) > 0.3) and 1 or 0 end), {hand="detail", tool={kind="round", width=0.8, point=0.6}, pile=leafL, load=0.6, pressure={0.4, 0.6}, coverage=2, fill=true, clip=true})
for _, q in ipairs(used) do print(q[1], q[2]) end

--@ chunk 79
print(wait(4*24*60)); for _,p in ipairs({{600,570},{660,600},{700,560},{478,540},{300,690}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 80
-- break the clean ellipse of the mere: snow tongues blown out onto the ice from both shores,
-- wind-streaks of snow on the ice, and the near rim broken with shadow
local tongues = nil
local xs = uneven(16, 420, 905, 0.7, 0.5, 81)
for i, x in ipairs(xs) do
  local near = (i % 3 ~= 0)
  local w = rand(6, 26)
  local d = rand(2.5, 8)
  local y0 = near and nearEdge(x) + 1.5 or mereFar(x) - 1
  local dir = near and -1 or 1
  local e = poly({{x - w, y0}, {x - w * 0.4, y0 + dir * d * rand(0.5, 1)}, {x + w * 0.1, y0 + dir * d}, {x + w * 0.6, y0 + dir * d * rand(0.3, 0.8)}, {x + w, y0}}, true)
  tongues = tongues and (tongues + e) or e
end
tonguesM = (tongues * merem:grow(1)):roughen(0.6, 3, 83)
work(tonguesM, {hand="detail", tool={kind="round", width=2, point=0.6}, pile=iceSnow, load=0.8, pressure={0.5, 0.75}, coverage=3, fill=true, clip=true, angle=0})
-- wind streaks: long thin drifts on the ice, lying with the wind from the left
local sm = nil
for i = 1, 14 do
  local x = rand(450, 880)
  local y = rand(553, 596)
  if merem:at(x, y) > 0.9 then
    local L = rand(15, 50)
    local r = ribbon({{x - L / 2, y + rand(-0.5, 0.5)}, {x, y}, {x + L / 2, y + rand(-0.8, 0.2)}}, {0.2, rand(0.6, 1.3), 0.2})
    sm = sm and (sm + r) or r
  end
end
streakM = (sm * merem):roughen(0.3, 2, 85)
work(streakM, {hand="detail", tool={kind="round", width=1.2, point=0.6}, pile=iceSnow, load=0.45, pressure={0.35, 0.55}, coverage=2, fill=true, clip=true, angle=0})
-- the rim: cool shadow in hollows along the near shore band
rimShade = pile{{"smalt",2},{"bone black",0.25},{"lead white",1.2},{"vermilion",0.05}, medium=0.6}
local rs = nil
for i = 1, 10 do
  local x = rand(430, 900)
  local y = nearEdge(x) + rand(3, 8)
  local e = ellipse(x, y, rand(10, 35), rand(2, 4))
  rs = rs and (rs + e) or e
end
rimShadeM = (rs - merem - pathM:grow(2)):roughen(1, 4, 87):soften(1)
work(rimShadeM, {hand="detail", tool={kind="round", width=3, point=0.4}, pile=rimShade, load=0.45, pressure={0.35, 0.55}, coverage=2, fill=true, clip=true, angle=0})
blend(rimShadeM:grow(2), {angle=0})

--@ chunk 81
-- an old fence running out of the near right toward the spruces: posts leaning, one down, snow caps
postP = pile{{"raw umber",1.5},{"bone black",0.5},{"smalt",0.3},{"lead white",0.25}, medium=0.15}
postL = pile{{"raw umber",1},{"yellow ochre",0.3},{"lead white",1.2},{"smalt",0.2}, medium=0.15}
capP = pile{{"lead white",5},{"pale smalt",1},{"smalt",0.2}, medium=0.1}
local A, B = {604, 700}, {836, 614}
posts = {}
local ts = {0, 0.2, 0.37, 0.52, 0.66, 0.79, 0.91}
local pm, cm, hm = nil, nil, nil
for i, t in ipairs(ts) do
  local x, y = lerp(A[1], B[1], t), lerp(A[2], B[2], t)
  local sc = lerp(1, 0.38, t ^ 0.8)
  local h, w = 38 * sc * rand(0.8, 1.15), 3.2 * sc
  local lean = randn(0, 0.12) + (i == 3 and 0.35 or 0)
  if i ~= 5 then
    local tx, ty = x + math.sin(lean) * h, y - math.cos(lean) * h
    local p = poly({{x - w / 2, y + 1}, {x + w / 2, y + 1}, {tx + w * 0.42, ty + w * 0.3}, {tx + w * 0.1, ty - w * 0.15}, {tx - w * 0.45, ty}})
    pm = pm and (pm + p) or p
    local lit = ribbon({{x - w * 0.35, y}, {tx - w * 0.3, ty + 1}}, w * 0.3)
    hm = hm and (hm + lit) or lit
    local c = ellipse(tx, ty - w * 0.1, w * 0.72, w * 0.42)
    cm = cm and (cm + c) or c
    posts[#posts+1] = {x, y, tx, ty, w}
  else
    -- the fallen post lying in the snow, half buried
    local p = poly({{x - 16 * sc, y - 1}, {x + 14 * sc, y - 3}, {x + 14 * sc, y - 3 + w * 0.8}, {x - 16 * sc, y + w * 0.6}})
    pm = pm + p
    cm = cm + ribbon({{x - 12 * sc, y - 1.2}, {x - 1 * sc, y - 2.3}, {x + 10 * sc, y - 3.2}}, {w * 0.3, w * 0.5, w * 0.2})
  end
end
postM = pm:roughen(0.3, 2, 91)
work(postM, {hand="detail", tool={kind="round", width=1.5, point=0.6}, pile=postP, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=-1.5, angle_jitter=0.1})
work(hm * postM:shrink(0.3), {hand="detail", tool={kind="round", width=0.8, point=0.6}, pile=postL, load=0.5, pressure={0.4, 0.6}, coverage=1.5, clip=true, angle=-1.5, angle_jitter=0.1})
postCapM = cm:roughen(0.3, 1.5, 93)
posts.cap = postCapM

--@ chunk 82
-- a second, stronger dusk glaze on the near ground: deepen the foreground so the eye goes to the lit distance
fgGlaze3 = pile{{"smalt",2.2},{"bone black",0.45},{"red earth",0.15},{"raw umber",0.2},{"lead white",0.25}, medium=0.8}
local n = noise{seed=821, period=120, octaves=3}
local m = landm * mask(function(x, y) return y > 600 and 1 or 0 end) - postM:grow(1.5) - merem:grow(1)
work(m, {hand="glaze", pile=fgGlaze3, angle=0.05, angle_jitter=0.05, clip=true,
   load_at=function(x, y) local t = smoothstep(605, 714, y); return (0.05 + 0.6 * t) * (0.8 + 0.3 * n(x, y)) end,
   pressure={0.3, 0.5}, coverage=1.6, seed=822})

--@ chunk 83
print(wait(3*24*60)); for _,p in ipairs({{300,690},{700,690},{640,680},{478,540},{250,480},{600,580}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 84
-- shadow into the land: the foreground dusk, soft undulations across the snow, and the oak's faint long shadow
landShade = pile{{"smalt",2.5},{"bone black",0.5},{"red earth",0.12},{"raw umber",0.15}, medium=0.85}
local un = noise{seed=841, period=90, octaves=3, kind="billow", stretch={0.08, 4}}
local nn = noise{seed=842, period=60, octaves=2}
local function segd(px, py, ax, ay, bx, by)
  local vx, vy = bx - ax, by - ay
  local t = clamp(((px - ax) * vx + (py - ay) * vy) / (vx * vx + vy * vy), 0, 1)
  return math.sqrt((px - ax - t * vx)^2 + (py - ay - t * vy)^2), t
end
local m = landm - merem:grow(1) - figM:grow(1.5) - postM:grow(1.5) - sprMask:grow(2) - oakLive:grow(1) - pathM:shrink(2)
work(m, {hand="glaze", pile=landShade, angle=0.05, angle_jitter=0.06, clip=true,
   load_at=function(x, y)
     local fg = 0.75 * smoothstep(610, 714, y) ^ 1.2
     local und = 0
     if y > 420 and y < 650 then und = 0.28 * math.max(0, un(x, y) + 0.1) * (0.5 + (y - 420) / 460) end
     local d, t = segd(x, y, 240, 450, 40, 600)
     local sh = 0
     if y > 440 then sh = 0.3 * (1 - smoothstep(4 + 30 * t, 14 + 50 * t, d)) * (1 - 0.6 * t) end
     return (fg + und + sh) * (0.85 + 0.25 * nn(x, y))
   end,
   pressure={0.3, 0.5}, coverage=1.6, seed=843})

--@ chunk 85
print(wait(3*24*60)); for _,p in ipairs({{300,690},{700,690},{640,680},{150,540},{250,480},{604,690}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 86
-- shadow into the land: the foreground dusk, soft undulations across the snow, and the oak's faint long shadow
landShade2 = pile{{"smalt",1.5},{"Prussian blue",0.3},{"bone black",0.35},{"red earth",0.25},{"raw umber",0.1}, medium=0.9}
local un = noise{seed=851, period=90, octaves=3, kind="billow", stretch={0.08, 4}}
local nn = noise{seed=852, period=60, octaves=2}
local function segd(px, py, ax, ay, bx, by)
  local vx, vy = bx - ax, by - ay
  local t = clamp(((px - ax) * vx + (py - ay) * vy) / (vx * vx + vy * vy), 0, 1)
  return math.sqrt((px - ax - t * vx)^2 + (py - ay - t * vy)^2), t
end
local m = landm - merem:grow(1) - figM:grow(1.5) - postM:grow(1.5) - sprMask:grow(2) - oakLive:grow(1) - pathM:shrink(2)
work(m, {hand="glaze", pile=landShade2, angle=0.05, angle_jitter=0.06, clip=true,
   load_at=function(x, y)
     local fg = 0.55 * smoothstep(610, 714, y) ^ 1.2
     local und = 0
     if y > 420 and y < 650 then und = 0.22 * math.max(0, un(x, y) + 0.1) * (0.5 + (y - 420) / 460) end
     local d, t = segd(x, y, 240, 450, 40, 600)
     local sh = 0
     if y > 440 then sh = 0.35 * (1 - smoothstep(4 + 30 * t, 14 + 50 * t, d)) * (1 - 0.6 * t) end
     return (fg + und + sh) * (0.85 + 0.25 * nn(x, y))
   end,
   pressure={0.3, 0.5}, coverage=1.6, seed=853})

--@ chunk 87
print(wait(1440))
for _, p in ipairs(posts) do print(p[1], p[2], p[3], p[4], drying(p[3], p[4])) end
-- snow caps on the stakes, and two slack strands of wire from post to post
local wire = brush{kind="rigger", width=0.5, point=1}
wire:load(pile{{"bone black",1},{"raw umber",1},{"smalt",0.3}, medium=0.2}, 0.5)
for i = 1, #posts - 1 do
  local a, b = posts[i], posts[i + 1]
  if not (i == 2) then  -- the wire hangs broken past the leaning post
    for k, f in ipairs({0.3, 0.62}) do
      local ax, ay = lerp(a[1], a[3], 1 - f), lerp(a[2], a[4], 1 - f)
      local bx, by = lerp(b[1], b[3], 1 - f), lerp(b[2], b[4], 1 - f)
      local sag = math.abs(bx - ax) * 0.07 + 1
      wire:stroke({{ax, ay}, {(ax + bx) / 2, (ay + by) / 2 + sag}, {bx, by}}, {pressure={0.25, 0.2}, ramps={0.05, 0.05}})
    end
  else
    -- a broken end dangling
    local ax, ay = lerp(a[1], a[3], 0.7), lerp(a[2], a[4], 0.7)
    wire:stroke({{ax, ay}, {ax + 6, ay + 5}, {ax + 9, ay + 11}}, {pressure={0.25, 0.1}})
  end
end
work(posts.cap, {hand="detail", tool={kind="round", width=1, point=0.8}, pile=capP, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=0})

--@ chunk 88
-- the glazes left pale haloes round the stakes: close them
local halo = (postM:grow(2.2) - postM:grow(0.2)) * mask(function(x, y) return y > 600 and 1 or 0 end) - posts.cap:grow(0.5)
work(halo, {hand="detail", tool={kind="round", width=1.2, point=0.5}, pile=landShade2, load=0.5, pressure={0.35, 0.5}, coverage=2.5, fill=true, clip=true, angle=-1.5,
  load_at=function(x, y) return 0.35 + 0.4 * smoothstep(610, 714, y) end})
-- the walker's faint shadow, falling toward us and to the left, very soft at dusk
local sh = poly({{474, 534}, {482, 534}, {470, 541}, {462, 543}}, true):soften(0.8)
work(sh - figM, {hand="detail", tool={kind="round", width=1.2, point=0.5}, pile=landShade2, load=0.45, pressure={0.35, 0.5}, coverage=2.5, fill=true, clip=true, angle=2.8})

--@ chunk 89
-- the shadow came out a hard blue sliver: soften it into the snow
local m = (ellipse(468, 539, 13, 5) - figM:grow(0.8))
work(m, {hand="blend", tool={kind="badger", width=4}, angle=2.9, coverage=3, clip=true})

--@ chunk 90
print(wait(3*24*60)); for _,p in ipairs({{560,478},{600,470},{700,480},{300,690},{150,540}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 91
-- lose the grey smear in the middle distance under a scumble of snow
local sm = poly({{490, 462}, {560, 458}, {640, 466}, {668, 484}, {650, 494}, {560, 488}, {495, 478}}, true):roughen(2, 10, 911):soften(3)
work(sm, {hand="scumble", pile=snowMid, load=0.4, pressure={0.3, 0.5}, coverage=1.5, angle=0.03, clip=true})
blend(sm:grow(3), {angle=0})
-- a far ditch with pollard willows across the middle distance
pollP = pile{{"lead white",2.5},{"smalt",1},{"red earth",0.25},{"bone black",0.35},{"raw umber",0.2}, medium=0.2}
local trunk = brush{kind="round", width=1.3, point=0.5}
local whip = brush{kind="rigger", width=0.45, point=1}
trunk:load(pollP, 0.9); whip:load(pollP, 0.7)
local xs = uneven(11, 500, 790, 0.6, 0.4, 913)
for i, x in ipairs(xs) do
  local y = lerp(484, 477, (x - 500) / 290) + randn(0, 0.5)
  local h = rand(4.5, 7.5)
  local lean = randn(0, 0.08)
  local tx, ty = x + lean * h, y - h
  trunk:stroke({{x, y}, {tx, ty}}, {pressure={0.8, 0.65}})
  trunk:touch(tx, ty, {pressure=0.8})
  for k = 1, math.random(7, 12) do
    local a = -math.pi / 2 + randn(0, 0.55)
    local L = rand(3, 7)
    whip:stroke({{tx, ty}, {tx + math.cos(a) * L * 0.5, ty + math.sin(a) * L * 0.55}, {tx + math.cos(a) * L, ty + math.sin(a) * L}}, {pressure={0.4, 0.02}})
    if k % 5 == 0 then whip:load(pollP, 0.7) end
  end
  trunk:load(pollP, 0.5)
end
-- the ditch: a faint cool line with snow on its lip
local d = brush{kind="round", width=1.2, point=0.5}
d:load(pile{{"lead white",2},{"smalt",1.2},{"bone black",0.2}, medium=0.4}, 0.4)
d:stroke({{495, 486}, {580, 484}, {690, 481}, {800, 478}}, {pressure={0.25, 0.35}, ramps={0.2, 0.3}, shake=0.4})

--@ chunk 92
-- the scumble spread warm and wide: cool it back into the snowfield while it is wet
coolSnow = pile{{"lead white",3},{"smalt",1.2},{"pale smalt",1},{"bone black",0.12},{"red earth",0.05}, medium=0.5}
local sm = poly({{484, 458}, {560, 452}, {650, 460}, {676, 484}, {656, 500}, {560, 496}, {488, 486}}, true):soften(4)
local pm = sm - ellipse(0,0,1,1)
work(sm, {hand="glaze", tool={kind="filbert", width=8}, pile=coolSnow, load=0.35, pressure={0.3, 0.45}, coverage=1.5, angle=0.02, clip=true,
  load_at=function(x, y) return 0.15 + 0.35 * smoothstep(455, 495, y) end})
blend(sm:grow(6), {angle=0})

--@ chunk 93
-- let it be a low mist lying in the hollow: lose its left edge, draw it out level
local m = poly({{420, 455}, {700, 448}, {720, 500}, {420, 505}}):soften(8) - figM:grow(3)
work(m, {hand="blend", angle=0, coverage=2.5, clip=true, length={60, 160}})

--@ chunk 94
print(wait(3*24*60)); for _,p in ipairs({{560,478},{300,690},{150,690},{800,690},{640,690}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 95
-- an evening glaze over all the land, to quiet the gold of the grass and bring the snow down under the sky
eveGlaze = pile{{"smalt",1.2},{"Prussian blue",0.2},{"bone black",0.3},{"raw umber",0.3},{"red earth",0.15}, medium=0.9}
local nn = noise{seed=951, period=80, octaves=3}
local m = landm - merem:grow(0.5) - figM:grow(1) - posts.cap:grow(0.5) - sprMask:grow(1) - oakLive:grow(0.5) - rect(470, 440, 330, 60):soften(10)
work(m, {hand="glaze", pile=eveGlaze, angle=0.03, angle_jitter=0.05, clip=true,
   load_at=function(x, y) return (0.12 + 0.35 * smoothstep(420, 714, y)) * (0.85 + 0.3 * nn(x, y)) end,
   pressure={0.3, 0.5}, coverage=1.6, seed=952})
-- the reed beds dulled toward brown in the dusk
reedGlaze = pile{{"raw umber",1.5},{"bone black",0.3},{"smalt",0.8},{"red earth",0.2}, medium=0.85}
local rb = (ellipse(428, 556, 36, 18) + ellipse(900, 553, 42, 16)):soften(4) + (rect(480, 530, 300, 20) * above(function(x) return mereFar(x) + 1 end)):soften(1)
work(rb, {hand="detail", tool={kind="round", width=3, point=0.4}, pile=reedGlaze, load=0.4, pressure={0.35, 0.5}, coverage=2, fill=true, clip=true, angle=-1.5, angle_jitter=0.3})

--@ chunk 96
-- the reed glaze went on opaque (raw umber hides): lift it off while it is open, with a clean brush wiped after every stroke
print(drying(600, 540), drying(428, 556), drying(900, 553))
rag = brush{kind="flat", width=4, pickup=1}
local function lift(m, x0, x1, y0, y1, ang)
  for pass = 1, 3 do
    for y = y0, y1, 1.6 do
      for x = x0, x1, 12 do
        local xx = x + rand(-3, 3)
        if m:at(xx + 5, y) > 0.05 then
          rag:wipe(1)
          rag:stroke({{xx, y}, {xx + 11, y + ang}}, {pressure={0.55, 0.55}, ramps={0.05, 0.05}})
        end
      end
    end
  end
end
local strip = (rect(480, 530, 300, 20) * above(function(x) return mereFar(x) + 1 end)):soften(1)
lift(strip:grow(2), 476, 784, 528, 551, 0)
lift(ellipse(428, 556, 40, 22), 385, 470, 536, 578, 0)
lift(ellipse(900, 553, 46, 20), 852, 948, 535, 573, 0)
rag:wipe(1)

--@ chunk 97
print(wait(3*24*60)); for _,p in ipairs({{600,540},{428,556},{900,553},{300,690},{150,500}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 98
-- the reeds, again: a transparent cool glaze of the low-hiding tubes only, to take the wheat-gold into dusk
reedGlaze2 = pile{{"smalt",2},{"Prussian blue",0.15},{"green earth",0.6}, medium=0.9}
local rb = (ellipse(428, 556, 34, 17) + ellipse(900, 553, 40, 15)):soften(4) + (rect(480, 528, 300, 22) * above(function(x) return mereFar(x) + 1 end)):soften(1)
work(rb - figM:grow(1), {hand="detail", tool={kind="round", width=3, point=0.4}, pile=reedGlaze2, load=0.5, pressure={0.35, 0.5}, coverage=2, fill=true, clip=true, angle=-1.5, angle_jitter=0.3})

--@ chunk 99
-- far too blue and far too visible: lift it all again
local function lift(m, x0, x1, y0, y1)
  for pass = 1, 4 do
    for y = y0, y1, 1.4 do
      for x = x0, x1, 12 do
        local xx = x + rand(-3, 3)
        if m:at(xx + 5, y) > 0.02 then
          rag:wipe(1)
          rag:stroke({{xx, y}, {xx + 11, y}}, {pressure={0.6, 0.6}, ramps={0.05, 0.05}})
        end
      end
    end
  end
end
local strip = (rect(480, 528, 300, 22) * above(function(x) return mereFar(x) + 1 end)):grow(3)
lift(strip, 474, 786, 524, 552)
lift(ellipse(428, 556, 40, 23), 383, 472, 532, 580)
lift(ellipse(900, 553, 46, 21), 850, 950, 531, 575)
rag:wipe(1)

--@ chunk 100
-- the reeds need no more glazing; instead a few darker stems and broken heads among them, for variety
local r = brush{kind="rigger", width=0.6, point=1}
local function stem(x, y, h, lean, bent)
  local tx, ty = x + lean * h, y - h
  local pts = {{x, y}, {x + lean * h * 0.5, y - h * 0.5}, {tx, ty}}
  if bent then pts[#pts+1] = {tx + h * 0.25 * (lean >= 0 and 1 or -1), ty + h * 0.2} end
  r:stroke(pts, {pressure={0.55, 0.05}, ramps={0.05, 0.5}})
end
r:load(reedDark, 0.7)
local n = 0
for i = 1, 14 do stem(rand(398, 458), rand(560, 574), rand(9, 18), randn(0.1, 0.12), math.random() < 0.3); n = n + 1; if n % 4 == 0 then r:load(reedDark, 0.7) end end
for i = 1, 12 do stem(rand(866, 932), rand(557, 568), rand(8, 15), randn(-0.05, 0.12), math.random() < 0.3); n = n + 1; if n % 4 == 0 then r:load(reedDark, 0.7) end end
for i = 1, 9 do local x = rand(490, 840); stem(x, mereFar(x) + 0.5, rand(4, 8), randn(0, 0.15), math.random() < 0.4); n = n + 1; if n % 4 == 0 then r:load(reedDark, 0.7) end end

--@ chunk 101
-- a limb fallen from the oak, lying in the snow downhill of it; stones by the path; a hare's track across the field
stoneP = pile{{"bone black",0.6},{"raw umber",0.8},{"smalt",0.8},{"lead white",1.2}, medium=0.15}
local b = {}
-- the fallen limb: a crooked main piece with two stubs
local lm = ribbon({{300, 474}, {318, 471}, {334, 473}, {352, 469}}, {2.0, 1.8, 1.5, 1.0})
         + ribbon({{318, 471}, {322, 464}, {327, 460}}, {1.0, 0.7, 0.3})
         + ribbon({{338, 472}, {344, 476}, {350, 478}}, {0.8, 0.6, 0.3})
fallenM = lm:roughen(0.3, 2, 1011)
work(fallenM, {hand="detail", tool={kind="round", width=1, point=0.6}, pile=barkDark, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=0})
-- stones by the path, half sunk
local sm = nil
for _, s in ipairs({{384, 662, 5, 2.6}, {394, 667, 3, 1.7}, {286, 702, 7, 3.2}, {372, 690, 3.5, 1.8}, {420, 606, 2.4, 1.2}}) do
  local e = ellipse(s[1], s[2], s[3], s[4]) * above(function(x) return s[2] + s[4] * 0.45 end)
  sm = sm and (sm + e) or e
end
stoneM = sm:roughen(0.4, 2, 1013)
work(stoneM, {hand="detail", tool={kind="round", width=1.2, point=0.6}, pile=stoneP, load=1, pressure={0.6, 0.9}, coverage=4, fill=true, clip=true, angle=0.4, angle_jitter=0.6})
-- hare track: pairs of long hind prints ahead of two small fore prints, bounding from the knoll toward the spruces
local tm = nil
local x, y = 520, 505
for i = 1, 16 do
  local s = 0.45 + (y - 490) / 90
  local f = {ellipse(x, y, 0.5 * s, 0.35 * s), ellipse(x + 1.5 * s, y + 0.8 * s, 0.5 * s, 0.35 * s),
             ellipse(x + 4 * s, y - 0.5 * s, 0.9 * s, 0.4 * s), ellipse(x + 4 * s, y + 1.1 * s, 0.9 * s, 0.4 * s)}
  for _, e in ipairs(f) do tm = tm and (tm + e) or e end
  x, y = x + rand(14, 20), y + rand(-0.6, 1.4)
end
hareM = tm
work(hareM, {hand="detail", tool={kind="round", width=0.8, point=0.6}, pile=fpP, load=0.6, pressure={0.4, 0.6}, coverage=2.5, fill=true, clip=true, angle=0})

--@ chunk 102
print(wait(2*24*60)); print(drying(320,471), drying(384,662), drying(286,702))

--@ chunk 103
print(wait(2*24*60)); print(drying(320,471), drying(384,662), drying(286,702))

--@ chunk 104
print(wait(5*24*60)); print(drying(320,471), drying(384,662), drying(286,702))

--@ chunk 105
-- snow lying along the top of the fallen limb and capping the stones
local top = fallenM * mask(function(x, y) return (fallenM:at(x, y - 1.2) < 0.4) and 1 or 0 end)
local capS = (stoneM * mask(function(x, y) return (stoneM:at(x, y - 1.3) < 0.4) and 1 or 0 end))
local m = (top:grow(0.5) + capS:grow(0.6)):roughen(0.2, 1, 1051)
work(m, {hand="detail", tool={kind="round", width=0.8, point=0.8}, pile=capP, load=0.9, pressure={0.5, 0.8}, coverage=3, fill=true, clip=true, angle=0})

--@ chunk 106
-- the snow took the whole of the thin limb: put back its dark underside and the stubs' shadowed sides
local r = brush{kind="rigger", width=0.9, point=1}
r:load(barkDark, 0.9)
r:stroke({{300, 475.3}, {318, 472.3}, {334, 474.2}, {352, 469.8}}, {pressure={0.7, 0.2}, ramps={0.05, 0.3}})
r:stroke({{319, 471}, {323.2, 464.5}, {327.8, 460.5}}, {pressure={0.45, 0.05}})
r:stroke({{338, 473}, {344, 476.8}, {350, 478.6}}, {pressure={0.45, 0.05}})
-- its cool shadow on the snow just below
local s = brush{kind="round", width=2, point=0.5}
s:load(landShade2, 0.35)
s:stroke({{298, 478}, {318, 476}, {336, 477}, {354, 473}}, {pressure={0.4, 0.2}, ramps={0.2, 0.4}})

--@ chunk 107
print(wait(2*24*60))
print(drying(320,472), drying(300,478), drying(150,500), drying(700,640))
-- the land is backlit: its face toward us should sit clearly below the glow of the sky.
-- a deeper glaze over it, leaving a thin lit rim along the knoll crest and the mist in the hollow.
duskLand = pile{{"smalt",1},{"Prussian blue",0.35},{"bone black",0.35},{"red earth",0.25},{"green earth",0.3}, medium=0.9}
local nn = noise{seed=1071, period=70, octaves=3}
local m = landm - merem:grow(0.5) - figM:grow(1) - posts.cap:grow(0.5) - sprMask:grow(1) - oakLive:grow(0.5)
          - fallenM:grow(1.5) - stoneM:grow(0.8)
work(m, {hand="glaze", pile=duskLand, angle=0.03, angle_jitter=0.05, clip=true,
   load_at=function(x, y)
     local below = y - knollCrest(math.min(x, 520))
     local rim = smoothstep(2, 14, below)
     local mist = 1 - 0.8 * math.exp(-((x - 590) / 110)^2 - ((y - 478) / 22)^2)
     local base = 0.28 + 0.3 * smoothstep(430, 714, y)
     return base * rim * mist * (0.85 + 0.3 * nn(x, y))
   end,
   pressure={0.3, 0.5}, coverage=1.8, seed=1072})

--@ chunk 108
print(wait(3*24*60)); print(drying(250,460), drying(320,480), drying(600,650))

--@ chunk 109
-- a hard-edged box has shown up round the oak's foot (an old passage's edges, sharpened by the glazes).
-- bury it under a broader drift: lit along its crest, cool on its face, lost at its edges
driftShade = pile{{"lead white",3},{"smalt",1.4},{"pale smalt",1},{"Prussian blue",0.05},{"bone black",0.15},{"red earth",0.06}, medium=0.3}
driftLit = pile{{"lead white",5},{"pale smalt",0.8},{"yellow ochre",0.08},{"vermilion",0.03}, medium=0.2}
local mound = poly({{140, 482}, {150, 462}, {178, 448}, {205, 444}, {290, 444}, {330, 452}, {350, 466}, {345, 484}, {300, 490}, {220, 489}}, true):roughen(2, 12, 1091)
local face = mound - oakLive:grow(0.5) - fallenM:grow(2)
work(face, {hand="body", pile=driftShade, load=0.6, pressure={0.4, 0.6}, coverage=2.2, angle=0.05, angle_jitter=0.2, edge={soft=0.5, lost=0.5, period=30, seed=1092}})
local crest = mound * below(function(x) return 446 + 0.0003 * (x - 240)^2 end) * above(function(x) return 455 + 0.0006 * (x - 240)^2 end) - oakLive:grow(0.5)
work(crest:soften(1.5), {hand="body", tool="filbert 4", pile=driftLit, load=0.5, pressure={0.35, 0.5}, coverage=1.5, angle=0, edge="soft"})
blend(mound:grow(4) - oakLive:grow(0.5) - fallenM:grow(2), {angle=0.02})

--@ chunk 110
print(wait(6*24*60)); print(drying(200,470), drying(300,480), drying(240,450))

--@ chunk 111
-- bring the new drift down into the field: glaze its face, keep its crest
local mound = poly({{130, 490}, {142, 460}, {178, 444}, {205, 440}, {290, 440}, {336, 450}, {360, 468}, {352, 490}, {300, 496}, {220, 496}}, true):soften(6)
local m = mound - oakLive:grow(0.5) - fallenM:grow(1)
work(m, {hand="glaze", tool={kind="filbert", width=14}, pile=duskLand, angle=0.03, clip=true,
  load_at=function(x, y) return 0.12 + 0.4 * smoothstep(450, 488, y) end, pressure={0.3, 0.45}, coverage=1.8, seed=1111})

--@ chunk 112
print(wait(3*24*60)); print(drying(330, 480), drying(250, 470))
-- the drift swallowed the fallen limb: lay it again, a little further down the slope, with only a hair of snow on it
local r = brush{kind="round", width=2.2, point=0.7}
r:load(barkDark, 1)
r:stroke({{318, 488}, {334, 485}, {350, 486.5}, {368, 482}}, {pressure={0.75, 0.35}, ramps={0.05, 0.3}})
local t = brush{kind="rigger", width=0.8, point=1}
t:load(barkDark, 0.9)
t:stroke({{333, 485}, {337, 479}, {342, 475}}, {pressure={0.6, 0.05}})
t:stroke({{352, 486}, {358, 490}, {363, 491.5}}, {pressure={0.5, 0.05}})
t:stroke({{320, 487.5}, {314, 484}, {310, 483.5}}, {pressure={0.5, 0.05}})
-- its shadow
local s = brush{kind="round", width=2, point=0.5}
s:load(duskLand, 0.4)
s:stroke({{316, 491}, {336, 489.5}, {352, 490.5}, {370, 486}}, {pressure={0.4, 0.2}, ramps={0.2, 0.4}})

--@ chunk 113
print(wait(4*24*60)); print(drying(334,485))

--@ chunk 114
-- a hair of snow along the top of the limb
local t = brush{kind="rigger", width=0.7, point=1}
t:load(capP, 0.9)
t:stroke({{319, 486.9}, {334, 483.9}, {350, 485.4}, {366, 481.2}}, {pressure={0.45, 0.2}, ramps={0.1, 0.3}, shake=0.3})

--@ chunk 115
print(wait(10*24*60)); for _,p in ipairs({{334,485},{240,460},{600,650},{884,450},{752,196},{566,214},{420,560},{700,690}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 116
varnish{coats=0.3, vary=0.1}
