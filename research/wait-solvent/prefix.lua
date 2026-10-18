-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box every
--@ engine 5

--@ chunk 1
canvas{size=600, aspect=4/3, linen={18,16}, seed=4127,
 ground={{pile={{"lead white",6},{"red earth",1},{"raw umber",1}}, um=120, apply="knife", texture=0.3}}}
print(W,H)

--@ chunk 2
draw_p = pile{{"burnt sienna",2},{"ultramarine blue",0.5}, turps=0.6}
local b = brush{kind="round", width=4, point=0.7}
b:load(draw_p, 0.8)
local function L(pts) b:gesture(pts, {wobble=1.5}); if b:fullness()<0.3 then b:load(draw_p,0.8) end end
-- valance bottom (scalloped), top
L({{0,72,0.5},{330,70,0.5},{670,74,0.5}})
local pts={}
for i=0,12 do local x=i*55; pts[#pts+1]={x,118,0.5}; pts[#pts+1]={x+27,128,0.5} end
L(pts)
-- post
L({{648,74,0.6},{650,400,0.6},{652,690,0.6}})
-- wire and bulb
L({{330,118,0.3},{330,150,0.3}})
L({{318,162,0.5},{330,150,0.5},{342,162,0.5},{338,182,0.5},{322,182,0.5},{318,162,0.5}})
-- crates
L({{15,305,0.5},{300,300,0.5},{625,305,0.5}})
L({{0,425,0.6},{360,420,0.6},{640,425,0.6}})
L({{0,592,0.6},{640,592,0.6}})
L({{300,300,0.5},{300,420,0.5}})
L({{360,420,0.5},{360,592,0.5}})
L({{0,690,0.6},{652,690,0.6}})
-- street
L({{652,520,0.4},{1000,505,0.4}})
L({{880,250,0.4},{882,505,0.4}})
L({{700,180,0.4},{700,420,0.4},{860,410,0.4},{860,200,0.4}})

--@ chunk 3
VPX, VPY = 2400, 380
function py(y0, x) return y0 + (VPY - y0) * (x / VPX) end
function band(y0a, y0b, x0, x1)  -- region between two receding lines
  return poly({{x0, py(y0a,x0)}, {x1, py(y0a,x1)}, {x1, py(y0b,x1)}, {x0, py(y0b,x0)}})
end
STALL_X = 662
CORNER_X = 770
-- masks (globals: used later)
m_valance = band(15, 108, -5, 690)
m_interior = band(100, 300, -5, STALL_X)
m_tier1 = band(255, 372, -5, STALL_X)
m_tier2 = band(365, 565, -5, STALL_X)
m_front = band(560, 735, -5, STALL_X+6)
m_facade_up = mask(function(x,y) if x < CORNER_X and y < py(20,x) then return 1 end return 0 end)
m_wall_side = poly({{STALL_X, py(15,STALL_X)}, {CORNER_X, py(-150,CORNER_X)}, {CORNER_X, py(740,CORNER_X)}, {STALL_X, py(740,STALL_X)}})
print(py(15,662), py(108,662), py(565,662), py(735,662), py(-150,770), py(740,770))

--@ chunk 4
night_t = pile{{"Prussian blue",1},{"ultramarine blue",1},{"burnt sienna",1.2},{"bone black",0.3}, turps=0.5, name="night_t"}
skylow_t = pile{{"ultramarine blue",1},{"lead white",1.2},{"permanent alizarin",0.25},{"raw umber",0.4}, turps=0.5, name="skylow_t"}
stalldark_t = pile{{"burnt sienna",2},{"ultramarine blue",1},{"raw umber",1}, turps=0.5, name="stalldark_t"}
glow_t = pile{{"burnt sienna",1},{"raw sienna",1},{"yellow ochre",0.6}, turps=0.5, name="glow_t"}
local sky = rect(CORNER_X-10, 0, 1000-CORNER_X+20, 340)
work(sky, {hand="broad", piles={{night_t, function(x,y) return 1.2 - y/340 end},{skylow_t, function(x,y) return 0.1 + y/300 end}}, angle=0.1, coverage=1.6})

--@ chunk 5
local sky = rect(CORNER_X-10, 0, 1000-CORNER_X+20, 340)
work(sky, {hand="body", tool="flat 16", piles={{night_t, function(x,y) return 1.2 - y/340 end},{skylow_t, function(x,y) return 0.1 + y/300 end}}, angle=0.05, coverage=2.5, length={40,110}, fill=true})

--@ chunk 6
facade_t = mix{{night_t, 0.6},{stalldark_t,0.4}, name="facade_t"}
local up = mask(function(x,y) if x < CORNER_X and y < py(22,x) then return 1 end return 0 end)
local side = poly({{STALL_X-4, py(10,STALL_X)}, {CORNER_X, py(10,CORNER_X)}, {CORNER_X, py(745,CORNER_X)}, {STALL_X-4, py(745,STALL_X)}})
m_facade = up + side
work(m_facade, {hand="body", tool="flat 16", pile=facade_t, angle=function(x,y) return math.atan(VPY-y, VPX-x) end, coverage=2.5, length={40,110}, fill=true, clip=true})

--@ chunk 7
BX, BY = 300, 215
local inter = band(104, 266, -5, STALL_X)
local function near(x,y) local d=math.sqrt((x-BX)^2+((y-BY)*1.4)^2); return math.max(0, 1 - d/300) end
work(inter, {hand="body", tool="flat 14", piles={{glow_t, function(x,y) return 0.05+1.5*near(x,y) end},{stalldark_t, function(x,y) return 1.1 - near(x,y) end}}, angle=function(x,y) return math.atan(VPY-y, VPX-x) end, coverage=2.5, length={30,90}, fill=true, clip=true})

--@ chunk 8
pom_t = pile{{"cadmium red",2},{"permanent alizarin",1}, turps=0.4, name="pom_t"}
lime_t = pile{{"cadmium yellow",2},{"emerald green",1.5},{"viridian",0.3}, turps=0.4, name="lime_t"}
orange_t = pile{{"cadmium yellow",2},{"cadmium red",1}, turps=0.4, name="orange_t"}
lemon_t = pile{{"lemon chrome",3},{"cadmium yellow",0.5}, turps=0.4, name="lemon_t"}
local function near(x,y) local d=math.sqrt((x-BX)^2+((y-BY)*1.1)^2); return math.max(0, 1 - d/480) end
local split1 = 330
local t1 = band(258, 372, -5, STALL_X)
local leftm = mask(function(x,y) return x < split1 + (y-300)*0.15 and 1 or 0 end)
local A = t1 * leftm
local B = t1 - leftm
local dk = function(x,y) return 0.15 + 0.9*(1-near(x,y)) end
work(A, {hand="body", tool="flat 12", piles={{pom_t, function(x,y) return 1 end},{stalldark_t, dk}}, coverage=2.2, length={20,50}, fill=true, clip=true, angle=0.2})
work(B, {hand="body", tool="flat 12", piles={{lime_t, function(x,y) return 1 end},{stalldark_t, dk}}, coverage=2.2, length={20,50}, fill=true, clip=true, angle=-0.2})
local t2 = band(368, 565, -5, STALL_X)
local leftm2 = mask(function(x,y) return x < 400 + (y-450)*0.2 and 1 or 0 end)
work(t2*leftm2, {hand="body", tool="flat 14", piles={{orange_t, function(x,y) return 1 end},{stalldark_t, dk}}, coverage=2.2, length={25,60}, fill=true, clip=true, angle=0.3})
work(t2-leftm2, {hand="body", tool="flat 14", piles={{lemon_t, function(x,y) return 1 end},{stalldark_t, dk}}, coverage=2.2, length={25,60}, fill=true, clip=true, angle=-0.3})

--@ chunk 9
wood_t = pile{{"burnt sienna",1.5},{"bone brown",1},{"ultramarine blue",0.6}, turps=0.4, name="wood_t"}
work(band(560, 738, -5, STALL_X+2), {hand="body", tool="flat 14", pile=wood_t, coverage=2.2, length={30,80}, fill=true, clip=true, angle=function(x,y) return math.atan(VPY-y, VPX-x) end})
work(band(12, 110, -5, STALL_X+28), {hand="body", tool="flat 12", piles={{glow_t,1},{pom_t,0.6}}, coverage=2.2, length={20,40}, fill=true, clip=true, angle=math.pi/2})
farb_t = mix{{night_t,0.5},{skylow_t,0.5}, name="farb_t"}
local farb = poly({{CORNER_X,335},{830,335},{830,300},{905,300},{905,345},{1005,330},{1005,500},{CORNER_X,505}})
work(farb, {hand="body", tool="flat 12", pile=farb_t, coverage=2.2, length={20,50}, fill=true, clip=true, angle=math.pi/2})

--@ chunk 10
m_pave = mask(function(x,y)
  local g
  if x < CORNER_X then g = py(733, x) else g = 498 end
  return y > g and 1 or 0 end)
local function warm(x,y) return math.max(0, 1 - math.abs(x-320)/450) * 0.9 end
work(m_pave, {hand="body", tool="flat 16", piles={{night_t, function(x,y) return 1.1 - warm(x,y) end},{glow_t, warm},{skylow_t, function(x,y) return x>CORNER_X and 0.4 or 0.05 end}}, coverage=2.2, length={40,100}, fill=true, clip=true, angle=0})

--@ chunk 11
print(wait(20*60))
for _,p in ipairs({{100,50},{300,200},{150,320},{200,450},{500,450},{300,650},{700,300},{900,150},{900,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 12
print(wait(48*60))
for _,p in ipairs({{100,50},{300,200},{150,320},{200,450},{500,450},{300,650},{700,300},{900,150},{900,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 13
print(wait(3*24*60))
for _,p in ipairs({{100,50},{300,200},{150,320},{200,450},{500,450},{300,650},{700,300},{900,150},{900,650}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 14
print(wait(2*24*60))
for _,p in ipairs({{150,320},{200,450},{300,650},{50,300}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 15
sky_top = pile{{"Prussian blue",1},{"ultramarine blue",1.2},{"burnt sienna",0.9},{"bone black",0.2},{"lead white",0.25}, name="sky_top"}
sky_mid = pile{{"ultramarine blue",1.2},{"Prussian blue",0.3},{"lead white",1},{"raw umber",0.5},{"permanent alizarin",0.1}, name="sky_mid"}
sky_low = pile{{"lead white",2},{"ultramarine blue",0.6},{"raw umber",0.6},{"yellow ochre",0.35},{"permanent alizarin",0.1}, name="sky_low"}
m_sky = rect(CORNER_X+2, 0, 240, 345)
local function t(y) return clamp(y/340,0,1) end
work(m_sky, {hand="body", tool="filbert 10", piles={
  {sky_top, function(x,y) return math.max(0, 1 - t(y)*1.6) end},
  {sky_mid, function(x,y) return math.max(0, 1 - math.abs(t(y)-0.6)*2.5) end},
  {sky_low, function(x,y) return math.max(0, (t(y)-0.6)*2.5) end}},
  coverage=3, length={30,80}, angle=function(x,y) return 0.05 + 0.1*math.sin(x/70+y/50) end, fill=true, clip=true})

--@ chunk 16
far_b = pile{{"ultramarine blue",1},{"Prussian blue",0.35},{"burnt sienna",0.8},{"lead white",0.45},{"raw umber",0.3}, name="far_b"}
far_d = pile{{"ultramarine blue",1},{"Prussian blue",0.4},{"burnt sienna",1},{"lead white",0.15},{"bone black",0.15}, name="far_d"}
m_farb = poly({{CORNER_X-2,268},{800,268},{800,255},{808,255},{808,268},{850,272},{850,300},{915,296},{915,280},{935,280},{935,298},{1005,302},{1005,505},{CORNER_X-2,505}})
work(m_farb, {hand="body", tool="flat 8", piles={{far_b, function(x,y) return 0.3 + (x-770)/300 end},{far_d, function(x,y) return 1 - (x-770)/400 end}}, coverage=3, length={15,40}, angle=math.pi/2, fill=true, clip=true})

--@ chunk 17
wall_lit = pile{{"raw sienna",1},{"yellow ochre",1},{"burnt sienna",0.4},{"lead white",0.35}, name="wall_lit"}
wall_mid = pile{{"burnt sienna",1},{"raw umber",0.8},{"yellow ochre",0.3}, name="wall_mid"}
wall_dark = pile{{"raw umber",1},{"ultramarine blue",0.6},{"burnt sienna",0.7}, name="wall_dark"}
function bulbnear(x,y,R) local d=math.sqrt((x-BX)^2+((y-BY)*1.3)^2); return math.max(0, 1 - d/(R or 300)) end
m_backwall = band(146, 266, -5, STALL_X)
work(m_backwall, {hand="body", tool="flat 9", piles={
  {wall_lit, function(x,y) return 2.2*bulbnear(x,y,260)^1.5 end},
  {wall_mid, function(x,y) return 0.2 + bulbnear(x,y,420) end},
  {wall_dark, function(x,y) return math.max(0, 1 - bulbnear(x,y,420)*1.5) end}},
  coverage=2.5, length={25,60}, angle=math.pi/2, angle_jitter=0.1, fill=true, clip=true})

--@ chunk 18
canvas_lit = pile{{"Naples yellow",2},{"lead white",1},{"cadmium yellow",0.3}, name="canvas_lit"}
canvas_mid = pile{{"yellow ochre",1.5},{"raw sienna",0.6},{"lead white",0.5}, name="canvas_mid"}
stripe_red = pile{{"cadmium red",2},{"vermilion",1},{"cadmium yellow",0.3}, name="stripe_red"}
stripe_dk = pile{{"cadmium red",1},{"permanent alizarin",1},{"burnt sienna",0.6}, name="stripe_dk"}
m_under = band(102, 150, -5, STALL_X)
work(m_under, {hand="body", tool="flat 8", piles={
  {canvas_lit, function(x,y) return 2*bulbnear(x,y,280) end},
  {canvas_mid, function(x,y) return 0.3 + 0.8*(1-bulbnear(x,y,280)) end},
  {wall_mid, function(x,y) return math.max(0, 0.8 - bulbnear(x,y,500)*1.6) end}},
  coverage=2.5, length={15,35}, angle=function(x,y) return math.atan(y-380, x+1500) end, fill=true, clip=true})
-- stripes on underside
LVX, LVY = -1500, 380
local sb = brush("flat", 10)
local x = 8
while x < STALL_X do
  local s = 1 - x/VPX
  local w = 34*s
  local y1 = py(102, x)
  local y2 = py(152, x)
  -- direction toward left VP
  local ang = math.atan(LVY - y1, LVX - x)
  local L = (y2 - y1) / math.sin(ang)
  local bri = bulbnear(x, 140, 320)
  sb:reload(bri > 0.25 and stripe_red or stripe_dk, 0.8)
  for k=0,1 do
    local ox = x + k*w*0.35
    sb:stroke({{ox, py(100,ox)}, {ox + math.cos(ang)*L, py(100,ox) + math.sin(ang)*L}}, {pressure=0.8, clip=m_under})
  end
  x = x + w*2
end

--@ chunk 19
val_cream = pile{{"Naples yellow",2},{"lead white",0.6},{"yellow ochre",0.5},{"cadmium yellow",0.3}, name="val_cream"}
val_cream_dk = pile{{"yellow ochre",1.5},{"raw sienna",0.8},{"raw umber",0.4},{"lead white",0.3}, name="val_cream_dk"}
-- scalloped valance mask
local pts = {}
local x = -5
pts[#pts+1] = {-5, py(14,-5)}
pts[#pts+1] = {STALL_X+28, py(14,STALL_X+28)}
local sx = STALL_X+28
local scal = {}
local xx = STALL_X+28
while xx > -40 do
  local s = 1 - xx/VPX
  local w = 52*s
  -- one scallop from xx to xx-w: dip
  for k=0,6 do
    local t = k/6
    local px = xx - w*t
    pts[#pts+1] = {px, py(96,px) + 12*s*math.sin(math.pi*t)}
  end
  xx = xx - w
end
m_valance = poly(pts)
print(m_valance:at(300,100), m_valance:at(300,60), m_valance:at(300,30))
-- base: graded glow
local function vglow(x,y) return math.max(0, 1 - math.abs(x-BX)/420) end
work(m_valance, {hand="body", tool="flat 9", piles={{val_cream, function(x,y) return 0.2+1.5*vglow(x,y) end},{val_cream_dk, function(x,y) return 1.2-vglow(x,y) end}}, coverage=2.5, length={20,50}, angle=math.pi/2, fill=true, clip=true})

--@ chunk 20
local sb = brush("flat", 12)
local x = 4
local i = 0
while x < STALL_X + 24 do
  local s = 1 - x/VPX
  local w = 26*s
  local bri = math.max(0, 1 - math.abs(x-BX)/400)
  local m = mask(function(px,py_) return (px > x and px < x + w) and 1 or 0 end) * m_valance
  work(m, {hand="body", tool={kind="flat", width=w*0.7}, piles={{stripe_red, function() return 0.2 + 1.4*bri end},{stripe_dk, function() return 1.1 - bri end}}, coverage=2.4, length={25,60}, angle=math.pi/2, fill=true, clip=true})
  x = x + 2*w
end

--@ chunk 21
FR = FR or {}
function fruit(x, y, r, P, opts)
  opts = opts or {}
  local sq = opts.sq or 0.95
  local rot = opts.rot or 0
  local I = opts.I or clamp(bulbnear(x, y, 560)*1.25, 0.05, 1)
  local dx, dy = BX - x, BY - y
  local d = math.sqrt(dx*dx+dy*dy); dx, dy = dx/d, dy/d
  local m
  if rot ~= 0 then
    local pts = {}
    for k=0,23 do local a=k/24*2*math.pi; local ex,ey=math.cos(a)*r, math.sin(a)*r*sq
      pts[#pts+1] = {x + ex*math.cos(rot) - ey*math.sin(rot), y + ex*math.sin(rot) + ey*math.cos(rot)} end
    m = poly(pts, true)
  else m = ellipse(x, y, r, r*sq) end
  m = m:roughen(r*0.05, r*0.8, math.floor(x*7+y*3))
  local function lit(px,py_) local nx,ny=(px-x)/r,(py_-y)/r; return clamp(I*(0.45 + 0.85*(nx*dx+ny*dy)), 0, 1) end
  work(m, {hand="body", tool={kind="filbert", width=math.max(2.5,r*0.32)}, piles={
     {P[1], function(px,py_) return math.max(0, 0.5 - lit(px,py_)) * 3 end},
     {P[2], function(px,py_) return math.max(0.05, 1 - math.abs(lit(px,py_)-0.5)*1.7) end},
     {P[3], function(px,py_) return math.max(0, lit(px,py_)-0.58) * 3.5 end}},
     length={r*0.4, r*0.9}, coverage=2.2, clip=true, fill=true,
     angle=function(px,py_) return math.atan(py_-y, px-x) + math.pi/2 end})
  if P[4] and I > 0.35 then
    local b4 = brush{kind="round", width=math.max(1.8, r*0.17), point=0.6}
    b4:reload(P[4], 0.7)
    local hx, hy = x + dx*r*0.5, y + dy*r*0.5*sq
    b4:stroke({{hx - dy*r*0.09, hy + dx*r*0.09},{hx + dy*r*0.09, hy - dx*r*0.09}}, {pressure={0.65,0.15}})
  end
  FR[#FR+1] = {x=x, y=y, r=r, sq=sq, rot=rot}
end
pm_dark = pile{{"permanent alizarin",1},{"burnt sienna",0.6},{"ultramarine blue",0.3}, name="pm_dark"}
pm_body = pile{{"cadmium red",1.5},{"permanent alizarin",1},{"vermilion",0.5}, name="pm_body"}
pm_lite = pile{{"vermilion",1},{"cadmium red",1},{"orange chrome",0.3},{"lead white",0.25}, name="pm_lite"}
hi_w = pile{{"lead white",3},{"Naples yellow",1}, name="hi_w"}
-- pomegranates tier 1 left: rows back to front
local P = {pm_dark, pm_body, pm_lite, hi_w}
local rows = {{y0=272, r=19},{y0=300, r=21},{y0=330, r=23},{y0=356, r=24}}
for ri, row in ipairs(rows) do
  local x = rand(-10, 10) + (ri%2)*12
  while x < 320 do
    local s = 1 - x/VPX
    local r = row.r * s * rand(0.88, 1.1)
    local y = py(row.y0, x) + rand(-4, 4)
    fruit(x, y, r, P, {sq=rand(0.88,0.98)})
    x = x + r*1.85 + rand(-3,5)
  end
end

--@ chunk 22
gap_dk = pile{{"burnt sienna",1},{"ultramarine blue",1},{"permanent alizarin",0.4},{"bone brown",0.5}, name="gap_dk"}
local fr = nil
for _,f in ipairs(FR) do local e = ellipse(f.x, f.y, f.r*0.97, f.r*f.sq*0.97); fr = fr and (fr + e) or e end
FRUITMASK = fr
local t1a = band(262, 370, -5, 330) 
local gaps = t1a - fr
work(gaps, {hand="detail", pile=gap_dk, coverage=2.5, fill=true, clip=true})

--@ chunk 23
gap_t = pile{{"burnt sienna",1.2},{"ultramarine blue",0.8},{"permanent alizarin",0.2},{"bone brown",0.5}, turps=0.35, name="gap_t"}
T1B = band(262, 370, 330, STALL_X) 
T2 = band(372, 562, -5, STALL_X)
work(T1B + T2, {hand="body", tool="flat 12", pile=gap_t, coverage=2.2, length={20,50}, fill=true, clip=true, angle=0.1})
wait(2*24*60)
print(drying(500,300), drying(200,450), drying(500,450), drying(150,300))

--@ chunk 24
wait(2*24*60)
print(drying(500,300), drying(200,450), drying(500,450), drying(150,300), drying(100,500), drying(600,520))

--@ chunk 25
li_dark = pile{{"viridian",1},{"cadmium yellow",0.6},{"burnt sienna",0.4},{"ultramarine blue",0.2}, name="li_dark"}
li_body = pile{{"cadmium yellow",2},{"emerald green",1.5},{"viridian",0.3}, name="li_body"}
li_lite = pile{{"lemon chrome",2},{"emerald green",1},{"lead white",0.2}, name="li_lite"}
local P = {li_dark, li_body, li_lite, hi_w}
local rows = {{y0=270, r=15},{y0=292, r=16},{y0=314, r=17},{y0=336, r=18},{y0=358, r=19}}
for ri, row in ipairs(rows) do
  local x = 336 + rand(0, 8) + (ri%2)*10
  while x < STALL_X - 8 do
    local s = 1 - x/VPX
    local r = row.r * s * rand(0.88, 1.1)
    local y = py(row.y0, x) + rand(-3, 3)
    fruit(x, y, r, P, {sq=rand(0.85,0.95), rot=rand(-0.4,0.4)})
    x = x + r*1.85 + rand(-2,4)
  end
end

--@ chunk 26
o_dark = pile{{"cadmium red",1},{"burnt sienna",1},{"permanent alizarin",0.3}, name="o_dark"}
o_body = pile{{"cadmium yellow",2},{"cadmium red",1}, name="o_body"}
o_lite = pile{{"cadmium yellow",3},{"orange chrome",1},{"lead white",0.3}, name="o_lite"}
local P = {o_dark, o_body, o_lite, hi_w}
local rows = {{y0=382, r=21},{y0=410, r=23},{y0=440, r=25},{y0=472, r=27},{y0=505, r=29},{y0=538, r=30}}
for ri, row in ipairs(rows) do
  local x = rand(-12, 0) + (ri%2)*14
  local xmax = 400 + (row.y0-450)*0.2
  while x < xmax - 10 do
    local s = 1 - x/VPX
    local r = row.r * s * rand(0.9, 1.08)
    local y = py(row.y0, x) + rand(-4, 4)
    fruit(x, y, r, P, {sq=rand(0.9,0.98)})
    x = x + r*1.85 + rand(-3,5)
  end
end

--@ chunk 27
le_dark = pile{{"raw sienna",1},{"cadmium yellow",1},{"raw umber",0.3}, name="le_dark"}
le_body = pile{{"lemon chrome",3},{"cadmium yellow",0.7}, name="le_body"}
le_lite = pile{{"lemon chrome",2},{"lead white",0.6},{"lead-tin yellow",0.6}, name="le_lite"}
local P = {le_dark, le_body, le_lite, hi_w}
local rows = {{y0=382, r=17},{y0=405, r=18},{y0=430, r=19},{y0=456, r=20},{y0=483, r=21},{y0=511, r=22},{y0=540, r=23}}
for ri, row in ipairs(rows) do
  local xmin = 400 + (row.y0-450)*0.2
  local x = xmin + 8 + rand(0, 8) + (ri%2)*10
  while x < STALL_X - 10 do
    local s = 1 - x/VPX
    local r = row.r * s * rand(0.88, 1.08)
    local y = py(row.y0, x) + rand(-3, 3)
    fruit(x, y, r*1.12, P, {sq=rand(0.72,0.82), rot=rand(-0.5,0.3)})
    x = x + r*2.1 + rand(-2,4)
  end
end

--@ chunk 28
plank_lit = pile{{"burnt sienna",1},{"raw sienna",1},{"yellow ochre",0.4}, name="plank_lit"}
plank_mid = pile{{"burnt sienna",1},{"raw umber",0.8},{"bone brown",0.3}, name="plank_mid"}
plank_dk = pile{{"raw umber",1},{"ultramarine blue",0.7},{"bone brown",0.6},{"burnt sienna",0.4}, name="plank_dk"}
M_COUNTER = band(562, 736, -5, STALL_X+4)
-- planks: vertical boards, light from top
local x = -5
local i = 0
while x < STALL_X do
  local s = 1 - x/VPX
  local w = rand(42, 58)*s
  local x2 = math.min(x + w, STALL_X + 4)
  local m = M_COUNTER * poly({{x+1, 0},{x2-1,0},{x2-1,750},{x+1,750}})
  local top = py(562, x)
  local bot = py(736, x)
  local function lt(px,py_) return clamp(1 - (py_ - top)/(bot-top), 0, 1) * clamp(bulbnear(px, 380, 650)*1.3, 0, 1) end
  work(m, {hand="body", tool={kind="flat", width=w*0.5}, piles={
    {plank_lit, function(px,py_) return 2*lt(px,py_)^1.5 end},
    {plank_mid, function(px,py_) return 0.5 end},
    {plank_dk, function(px,py_) return 1.4*(1-lt(px,py_)) end}},
    coverage=2.4, length={40,90}, angle=math.pi/2 + rand(-0.03,0.03), fill=true, clip=true, mix_jitter=0.1})
  x = x2
  i = i + 1
end
-- seams between planks
local sb = brush{kind="round", width=2, point=0.5}
x = -5
while x < STALL_X do
  local s = 1 - x/VPX
  x = x + rand(42,58)*s
  if x < STALL_X then
    sb:reload(plank_dk, 0.7)
    sb:stroke({{x, py(564,x)},{x+rand(-1,1), py(734,x)}}, {pressure={0.6,0.5}})
  end
end

--@ chunk 29
BX, BY = 430, 215
bldg_dk = pile{{"ultramarine blue",1},{"Prussian blue",0.5},{"burnt sienna",1},{"bone black",0.3},{"lead white",0.12}, name="bldg_dk"}
bldg_mid = pile{{"ultramarine blue",1},{"Prussian blue",0.3},{"burnt sienna",0.8},{"lead white",0.3},{"permanent alizarin",0.1}, name="bldg_mid"}
-- far building masses: two blocks with different rooflines
local A = poly({{CORNER_X-2,250},{842,250},{842,236},{850,236},{850,250},{862,250},{862,505},{CORNER_X-2,505}})
local Bm = poly({{860,292},{906,292},{906,276},{930,276},{930,292},{1005,300},{1005,505},{860,505}})
work(A, {hand="body", tool="flat 8", piles={{bldg_dk,1},{bldg_mid,0.3}}, coverage=2.6, length={15,40}, angle=math.pi/2, fill=true, clip=true})
work(Bm, {hand="body", tool="flat 8", piles={{bldg_dk,0.5},{bldg_mid,0.8}}, coverage=2.6, length={15,40}, angle=math.pi/2, fill=true, clip=true})

--@ chunk 30
win_lit = pile{{"cadmium yellow",1},{"Naples yellow",1},{"orange chrome",0.3},{"lead white",0.3}, name="win_lit"}
win_warm = pile{{"orange chrome",1},{"raw sienna",0.6},{"Naples yellow",0.6}, name="win_warm"}
win_dk = pile{{"ultramarine blue",1},{"bone black",0.5},{"burnt sienna",0.5}, name="win_dk"}
local wins = {
 {785,275,14,22,"L"},{815,275,14,22,"D"},{845,275,10,22,"D"},
 {785,320,14,22,"D"},{815,320,14,22,"W"},{845,320,10,22,"D"},
 {785,365,14,22,"L"},{815,365,14,22,"D"},{845,365,10,22,"D"},
 {785,410,14,22,"D"},{815,410,14,22,"D"},{845,410,10,22,"W"},
 {872,315,9,16,"D"},{892,315,9,16,"L"},{940,318,9,16,"D"},{962,320,9,16,"D"},{984,321,9,16,"W"},
 {872,350,9,16,"W"},{892,350,9,16,"D"},{940,352,9,16,"L"},{962,353,9,16,"D"},{984,354,9,16,"D"},
 {872,385,9,16,"D"},{892,385,9,16,"D"},{940,387,9,16,"D"},{962,388,9,16,"L"},{984,389,9,16,"D"},
}
for _,w in ipairs(wins) do
  local p = (w[5]=="L" and win_lit) or (w[5]=="W" and win_warm) or win_dk
  local m = rect(w[1], w[2], w[3], w[4]):roughen(0.8, 6, math.floor(w[1]+w[2]))
  work(m, {hand="detail", pile=p, coverage=3, fill=true, clip=true, angle=math.pi/2})
end

--@ chunk 31
wait(6*24*60)
for _,p in ipairs({{100,300},{200,450},{500,450},{500,320},{300,650},{800,300},{780,282},{430,240}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 32
wait(5*24*60)
for _,p in ipairs({{100,300},{200,450},{500,450},{500,320},{300,650},{800,300},{780,282},{600,500}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 33
wait(4*24*60)
for _,p in ipairs({{100,300},{200,450},{500,450},{500,320},{600,500},{50,280}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 34
veil = pile{{"burnt sienna",1},{"ultramarine blue",0.5},{"permanent alizarin",0.1}, medium=0.3, thinner=0.6, name="veil"}
local inside = band(146, 562, -5, STALL_X)
local function dmask(lo, hi)
  return mask(function(x,y) local d = math.sqrt((x-BX)^2 + ((y-BY)*0.8)^2); return (d>=lo and d<hi) and 1 or 0 end):soften(15)
end
local zones = {{200,280,1},{280,360,2},{360,450,3},{450,2000,4}}
for _,z in ipairs(zones) do
  work(inside * dmask(z[1], z[2]), {hand="body", tool={kind="filbert", width=14, stiffness=0.3}, pile=veil, coverage=z[3], length={20,50}, clip=inside, angle=0.15, seed=z[1]})
end
