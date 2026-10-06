-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box hopper
--@ engine 5

--@ chunk 1
canvas{size=1000, aspect=1.6, linen={18,16}, seed=4117,
 ground={{pile={{"lead white",10},{"yellow ochre",0.25}}, um=120, apply="knife", texture=0.3}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
CX=330; RVP=3200; LVP=-250; EY=400
function RY(x,yc) return EY + (yc-EY)*(RVP-x)/(RVP-CX) end
function LY(x,yc) return EY + (yc-EY)*(x-LVP)/(CX-LVP) end
-- right-face rectangle (x0..x1, yc0..yc1) as perspective quad points
function RQ(x0,x1,a,b) return {{x0,RY(x0,a)},{x1,RY(x1,a)},{x1,RY(x1,b)},{x0,RY(x0,b)}} end
function LQ(x0,x1,a,b) return {{x0,LY(x0,a)},{x1,LY(x1,a)},{x1,LY(x1,b)},{x0,LY(x0,b)}} end
REND=880
h = pencil("2H")
-- corner and outline
h:rule({CX,92},{CX,475},{pressure=0.35})
h:rule({CX,92},{REND,RY(REND,92)},{pressure=0.35})
h:rule({CX,475},{REND,RY(REND,475)},{pressure=0.3})
h:rule({REND,RY(REND,92)},{REND,RY(REND,475)},{pressure=0.3})
h:rule({CX,92},{0,LY(0,92)},{pressure=0.35})
h:rule({CX,475},{0,LY(0,475)},{pressure=0.3})
h:rule({CX,118},{REND,RY(REND,118)},{pressure=0.25})
h:rule({CX,118},{0,LY(0,118)},{pressure=0.25})
-- sign band
for _,yc in ipairs({292,335}) do h:rule({CX,yc},{REND,RY(REND,yc)},{pressure=0.25}) end
-- upper windows right
for _,s in ipairs({{380,440},{510,568},{640,696},{765,818}}) do
  local q=RQ(s[1],s[2],165,265)
  h:line({q[1],q[2],q[3],q[4],q[1]},{pressure=0.3,smooth=false})
end
-- shop window, door, upstairs door
for _,s in ipairs({{355,590,345,440},{615,670,345,475},{810,855,350,475}}) do
  local q=RQ(s[1],s[2],s[3],s[4])
  h:line({q[1],q[2],q[3],q[4],q[1]},{pressure=0.3,smooth=false})
end
-- left face upper windows
for _,s in ipairs({{90,135},{215,268}}) do
  local q=LQ(s[1],s[2],165,265)
  h:line({q[1],q[2],q[3],q[4],q[1]},{pressure=0.3,smooth=false})
end
-- curb
h:line({{0,465},{200,515},{290,542},{320,546},{360,544},{600,528},{1000,510}},{pressure=0.3})
print(RY(REND,92), RY(REND,475), LY(0,92), LY(0,475))

--@ chunk 3
-- background building at left behind, and distant blocks right
h:rule({0,48},{235,48},{pressure=0.25})
h:rule({235,48},{235,LY(235,92)},{pressure=0.25})
h:line({{880,248},{945,248},{945,280},{1000,280}},{pressure=0.25,smooth=false})
-- figure placement sketch: seated man near x 735
h:sketch({{728,420},{730,440},{740,452},{760,452},{762,470}},{pressure=0.25})
h:sketch({{735,402},{733,420}},{pressure=0.25})
-- awning
local a1={355-8,RY(347,338)}; local a2={598,RY(598,338)}
h:line({a1,a2,{604,RY(604,385)},{343,RY(343,385)},a1},{pressure=0.25,smooth=false})

--@ chunk 4
M_rf = poly({{CX,92},{REND,RY(REND,92)},{REND,RY(REND,475)},{CX,475}})
M_lf = poly({{0,LY(0,92)},{CX,92},{CX,475},{0,LY(0,475)}})
M_bgL = poly({{0,48},{235,48},{235,LY(235,93)},{0,LY(0,93)}}) - M_lf
M_bgR = poly({{REND,248},{945,248},{945,280},{1000,280},{1000,470},{REND,470}})
M_curb = {{0,465},{200,515},{290,542},{320,546},{360,544},{600,528},{1000,510}}
M_walk = poly({{0,LY(0,475)},{CX,475},{REND,RY(REND,475)},{1000,458},{1000,510},{600,528},{360,544},{320,546},{290,542},{200,515},{0,465}}) - M_lf - M_rf
M_street = below(M_curb) - M_walk
M_sky = -(M_rf + M_lf + M_bgL + M_bgR + M_walk + M_street)
print(M_sky:area(), M_rf:area(), M_lf:area(), M_walk:area(), M_street:area())

--@ chunk 5
t_sky = pile{{"lead white",3},{"cerulean blue",1.2},{"ultramarine blue",0.5}, turps=0.6, name="t_sky"}
t_brick = pile{{"lead white",1.5},{"red earth",2},{"yellow ochre",0.6},{"deep cadmium",0.3}, turps=0.6, name="t_brick"}
t_shad = pile{{"red earth",1.5},{"ultramarine blue",1.2},{"bone black",0.3},{"lead white",0.8}, turps=0.6, name="t_shad"}
t_walk = pile{{"lead white",3},{"yellow ochre",0.6},{"red earth",0.15},{"bone black",0.1}, turps=0.6, name="t_walk"}
t_street = pile{{"lead white",1.5},{"bone black",0.4},{"ultramarine blue",0.3},{"red earth",0.3}, turps=0.6, name="t_street"}
t_green = pile{{"viridian",1},{"bone black",0.6},{"yellow ochre",0.5},{"lead white",0.3}, turps=0.6, name="t_green"}
t_dark = pile{{"ultramarine blue",1},{"burnt sienna",1},{"bone black",0.3}, turps=0.6, name="t_dark"}
t_cream = pile{{"lead white",4},{"yellow ochre",0.5},{"pale cadmium",0.2}, turps=0.6, name="t_cream"}

--@ chunk 6
work(M_sky, {hand="broad", pile=t_sky, angle=0.05, coverage=1.3, edge="firm"})
work(M_rf, {hand="broad", pile=t_brick, angle=function(x,y) return math.atan(RY(1000,200)-RY(330,200),670) end, coverage=1.2, clip=true})
work(M_lf + M_bgL, {hand="broad", pile=t_shad, angle=0.3, coverage=1.2, clip=true})
work(M_walk, {hand="body", pile=t_walk, angle=-0.05, coverage=1.2, clip=true})
work(M_street, {hand="broad", pile=t_street, angle=-0.05, coverage=1.2, clip=true})
work(M_bgR, {hand="body", pile=t_cream, angle=0, coverage=1.1, clip=true})

--@ chunk 7
t_brick2 = pile{{"lead white",1.2},{"red earth",2.2},{"yellow ochre",0.5},{"bone black",0.08}, turps=0.5, name="t_brick2"}
t_shad2 = pile{{"red earth",1.6},{"ultramarine blue",0.7},{"bone black",0.35},{"lead white",0.7},{"burnt sienna",0.3}, turps=0.5, name="t_shad2"}
local ang = math.atan(RY(1000,200)-RY(330,200),670)
work(M_rf, {hand="body", tool="filbert 14", pile=t_brick2, angle=ang, coverage=2.2, fill=true, clip=true})
work(M_lf + M_bgL, {hand="body", tool="filbert 14", pile=t_shad2, angle=0.35, coverage=2.2, fill=true, clip=true})
work(M_sky, {hand="body", tool="filbert 16", pile=t_sky, angle=0.03, coverage=2, fill=true, clip=true})
work(M_walk, {hand="body", pile=t_walk, angle=-0.05, coverage=2, fill=true, clip=true})
work(M_street, {hand="body", tool="filbert 16", pile=t_street, angle=-0.05, coverage=2, fill=true, clip=true})
work(M_bgR, {hand="body", pile=t_cream, angle=0, coverage=2, fill=true, clip=true})

--@ chunk 8
print(wait(24*60)); print(drying(600,300), drying(100,300), drying(500,100), drying(500,580))

--@ chunk 9
print(wait(2*24*60)); print(drying(600,300), drying(100,300), drying(500,100), drying(500,580))

--@ chunk 10
print(wait(3*24*60)); print(drying(600,300), drying(100,300), drying(500,100), drying(500,580), drying(950,350))

--@ chunk 11
sky_hi = pile{{"lead white",3},{"cobalt blue",1.2},{"ultramarine blue",0.5},{"cerulean blue",0.6}, medium=0.08, name="sky_hi"}
sky_lo = pile{{"lead white",5},{"cerulean blue",1.0},{"yellow ochre",0.08}, medium=0.08, name="sky_lo"}

--@ chunk 12
sky_hi:add{{"cobalt blue",0.6},{"ultramarine blue",0.4}}

--@ chunk 13
work(M_sky, {hand="body", tool="filbert 14", piles={{sky_hi, function(x,y) return clamp(1 - y/300,0,1)^1.3 end},{sky_lo, function(x,y) return clamp(y/300,0,1) end}},
  angle=function(x,y) return 0.02 end, length={40,110}, coverage=2.5, fill=true, clip=true})

--@ chunk 14
blend(M_sky:shrink(3), {angle=0.02, coverage=1})

--@ chunk 15
bgl = pile{{"lead white",4},{"yellow ochre",0.9},{"red earth",0.25},{"bone black",0.08},{"cerulean blue",0.1}, medium=0.05, name="bgl"}
local m = poly({{0,48},{235,48},{235,LY(235,92)+2},{0,LY(0,92)+2}}) - M_lf:shrink(1)
M_bgL2 = m
work(m, {hand="body", tool="filbert 12", pile=bgl, angle=1.5708, length={30,80}, coverage=2.5, fill=true, clip=true})

--@ chunk 16
function fill(m, p, opt)
  opt = opt or {}
  work(m, {hand=opt.hand or "body", tool=opt.tool or "filbert 6", pile=p, angle=opt.angle or 0, length=opt.length or {10,40},
    coverage=opt.coverage or 2.2, fill=true, clip=true, pressure=opt.pressure})
end
RANG = math.atan(RY(1000,200)-RY(330,200),670)
cream = pile{{"lead white",5},{"yellow ochre",0.45},{"pale cadmium",0.15}, medium=0.05, name="cream"}
cream_sh = pile{{"lead white",2},{"ultramarine blue",0.25},{"red earth",0.35},{"bone black",0.12},{"yellow ochre",0.2}, medium=0.05, name="cream_sh"}
glass = pile{{"ultramarine blue",1},{"burnt sienna",0.8},{"bone black",0.4},{"lead white",0.35}, medium=0.1, name="glass"}
shade_g = pile{{"lead white",2.5},{"yellow ochre",0.8},{"viridian",0.25},{"bone black",0.05}, medium=0.05, name="shade_g"}

--@ chunk 17
RWIN = {{380,440,212},{510,568,232},{640,696,198},{765,818,240}}
for i,s in ipairs(RWIN) do
  local x0,x1,sh = s[1],s[2],s[3]
  -- opening dark glass
  fill(poly(RQ(x0,x1,165,265)), glass, {tool="filbert 5", angle=1.5708})
end
for i,s in ipairs(RWIN) do
  local x0,x1,sh = s[1],s[2],s[3]
  fill(poly(RQ(x0-6,x1+6,153,165)), cream, {tool="flat 4", angle=RANG})   -- lintel
  fill(poly(RQ(x0-7,x1+7,265,273)), cream, {tool="flat 4", angle=RANG})   -- sill
  fill(poly(RQ(x0+3,x1-3,167,sh)), shade_g, {tool="filbert 4", angle=1.5708}) -- drawn shade
end

--@ chunk 18
green_sun = pile{{"viridian",1},{"yellow ochre",0.9},{"bone black",0.35},{"lead white",0.6}, medium=0.05, name="green_sun"}
green_sh = pile{{"viridian",1},{"bone black",0.6},{"yellow ochre",0.3},{"lead white",0.2},{"ultramarine blue",0.2}, medium=0.05, name="green_sh"}
fill(poly(RQ(330,700,335,475)), green_sun, {tool="filbert 8", angle=1.5708, length={20,60}})
fill(poly(LQ(225,330,335,475)), green_sh, {tool="filbert 8", angle=1.5708, length={20,60}})
-- glass of shop windows and doors
fill(poly(RQ(356,590,347,438)), glass, {tool="filbert 7", angle=RANG, length={20,60}})
fill(poly(RQ(614,668,350,472)), glass, {tool="filbert 5", angle=1.5708})
fill(poly(LQ(238,318,347,438)), glass, {tool="filbert 5", angle=1.5708})
fill(poly(RQ(812,852,355,472)), glass, {tool="filbert 5", angle=1.5708})
-- left face upper windows: dark glass
for _,s in ipairs({{90,135},{215,268}}) do fill(poly(LQ(s[1],s[2],165,265)), glass, {tool="filbert 5", angle=1.5708}) end

--@ chunk 19
print(wait(5*24*60)); for _,p in ipairs({{400,200},{470,390},{640,400},{280,400},{100,100},{500,580}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 20
print(wait(6*24*60)); for _,p in ipairs({{400,200},{470,390},{640,400},{280,400},{100,100},{500,30}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 21
print(wait(4*24*60)); for _,p in ipairs({{400,200},{410,240},{280,400},{240,240}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 22
green_sun2 = pile{{"viridian",1},{"yellow ochre",0.45},{"bone black",0.4},{"lead white",0.35},{"burnt sienna",0.1}, medium=0.04, name="green_sun2"}
green_sh2 = pile{{"viridian",1},{"bone black",0.7},{"red earth",0.2},{"lead white",0.15},{"ultramarine blue",0.2}, medium=0.04, name="green_sh2"}
glass2 = pile{{"ultramarine blue",1},{"burnt sienna",0.9},{"bone black",0.5},{"lead white",0.25}, medium=0.04, name="glass2"}
local openR = poly(RQ(356,590,347,438)) + poly(RQ(614,668,350,472))
local openL = poly(LQ(238,318,347,438))
fill(poly(RQ(330,700,335,475)) - openR, green_sun2, {tool="filbert 6", angle=1.5708, length={15,40}})
fill(poly(LQ(225,330,335,475)) - openL, green_sh2, {tool="filbert 6", angle=1.5708, length={15,40}})
fill(openR + openL + poly(RQ(812,852,355,472)), glass2, {tool="filbert 6", angle=1.5708, length={15,50}})
for _,s in ipairs(RWIN) do fill(poly(RQ(s[1],s[2],s[3]-3,265)), glass2, {tool="filbert 5", angle=1.5708}) end
for _,s in ipairs({{90,135},{215,268}}) do fill(poly(LQ(s[1],s[2],165,265)), glass2, {tool="filbert 5", angle=1.5708}) end

--@ chunk 23
corn = pile{{"lead white",4},{"yellow ochre",0.7},{"red earth",0.12},{"bone black",0.05}, medium=0.04, name="corn"}
soffit = pile{{"lead white",1},{"red earth",0.5},{"ultramarine blue",0.3},{"bone black",0.12},{"yellow ochre",0.2}, medium=0.04, name="soffit"}
brick_sh = pile{{"red earth",1.6},{"ultramarine blue",0.6},{"bone black",0.3},{"lead white",0.5},{"burnt sienna",0.4}, medium=0.04, name="brick_sh"}
fill(poly(RQ(330,REND,91,108)), corn, {tool="flat 5", angle=RANG, length={30,80}})
fill(poly(RQ(330,REND,108,117)), soffit, {tool="flat 4", angle=RANG, length={30,80}})
fill(poly(LQ(0,330,91,108)), cream_sh, {tool="flat 5", angle=0.4, length={30,80}})
fill(poly(LQ(0,330,108,117)), brick_sh, {tool="flat 4", angle=0.4, length={30,80}})

--@ chunk 24
shade_y = pile{{"lead white",3},{"yellow ochre",1},{"deep cadmium",0.08},{"red earth",0.05}, medium=0.03, name="shade_y"}
shade_gr = pile{{"viridian",1},{"yellow ochre",1},{"lead white",1.2},{"bone black",0.15}, medium=0.03, name="shade_gr"}
for i,s in ipairs(RWIN) do
  local p = (i==2 or i==4) and shade_gr or shade_y
  fill(poly(RQ(s[1]+2,s[2]-2,166,s[3]-4)), p, {tool="flat 4", angle=1.5708, length={8,25}, coverage=3})
end

--@ chunk 25
SHD = mask(function(x,y) return (85*(x-330)+160*(y-475)) < 0 and 1 or 0 end)
walk_sun = pile{{"lead white",4},{"yellow ochre",0.55},{"red earth",0.12},{"bone black",0.06}, medium=0.03, name="walk_sun"}
walk_sh = pile{{"lead white",2},{"ultramarine blue",0.35},{"red earth",0.3},{"bone black",0.15},{"yellow ochre",0.15}, medium=0.03, name="walk_sh"}
fill(M_walk - SHD, walk_sun, {tool="filbert 10", angle=-0.05, length={30,80}})
fill(M_walk * SHD, walk_sh, {tool="filbert 10", angle=0.25, length={30,80}})

--@ chunk 26
walk_sh2 = pile{{"lead white",1},{"ultramarine blue",0.4},{"red earth",0.35},{"bone black",0.2},{"burnt sienna",0.1}, medium=0.03, name="walk_sh2"}
street_sun = pile{{"lead white",2.2},{"bone black",0.3},{"red earth",0.25},{"yellow ochre",0.35},{"ultramarine blue",0.12}, medium=0.03, name="street_sun"}
street_sh = pile{{"lead white",0.8},{"ultramarine blue",0.45},{"bone black",0.3},{"red earth",0.3},{"burnt sienna",0.1}, medium=0.03, name="street_sh"}
fill(M_walk * SHD, walk_sh2, {tool="filbert 10", angle=0.25, length={30,80}, coverage=2.5})
fill(M_street - SHD, street_sun, {tool="filbert 14", angle=-0.03, length={40,120}})
fill(M_street * SHD, street_sh, {tool="filbert 14", angle=-0.03, length={40,120}})

--@ chunk 27
print(drying(100,100), drying(950,350), drying(900,300))
bgwin = pile{{"ultramarine blue",0.8},{"burnt sienna",0.6},{"bone black",0.3},{"lead white",0.9}, medium=0.03, name="bgwin"}
local m = nil
for r,y in ipairs({66,118,170,222}) do
  for c,x in ipairs({18,76,134,190}) do
    local w = poly({{x,y},{x+28,y},{x+28,y+34},{x,y+34}})
    m = m and (m + w) or w
  end
end
m = m * M_bgL2:shrink(2)
fill(m, bgwin, {tool="flat 3", angle=1.5708, length={8,20}, coverage=2.5})

--@ chunk 28
bgr1 = pile{{"lead white",3.5},{"yellow ochre",0.7},{"red earth",0.2},{"cerulean blue",0.12},{"bone black",0.05}, medium=0.03, name="bgr1"}
bgr2 = pile{{"lead white",1.2},{"red earth",1},{"yellow ochre",0.3},{"cerulean blue",0.3},{"bone black",0.1}, medium=0.03, name="bgr2"}
fill(poly({{882,248},{945,248},{945,462},{882,462}}), bgr1, {tool="filbert 6", angle=1.5708, length={20,50}})
fill(poly({{945,280},{1000,280},{1000,460},{945,460}}), bgr2, {tool="filbert 6", angle=1.5708, length={20,50}})

--@ chunk 29
print(wait(5*24*60)); for _,p in ipairs({{400,400},{340,400},{470,390},{100,550},{600,580},{900,350},{420,180},{30,80}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 30
print(wait(5*24*60)); for _,p in ipairs({{400,400},{340,400},{470,390},{100,550},{600,580},{900,350},{420,180},{30,80}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 31
brick_sh2 = pile{{"red earth",1.6},{"burnt sienna",0.5},{"ultramarine blue",0.6},{"bone black",0.3},{"lead white",0.3}, medium=0.03, name="brick_sh2"}

--@ chunk 32
brick_sh3 = pile{{"red earth",1.4},{"burnt sienna",0.7},{"ultramarine blue",0.7},{"bone black",0.45}, medium=0.03, name="brick_sh3"}

--@ chunk 33
brick_shM = mix{{brick_sh3,0.6},{brick_sh2,0.4}, name="brick_shM"}
local wins = poly(LQ(90,135,165,265)) + poly(LQ(215,268,165,265)) + poly(LQ(225,330,335,476))
local face = poly({{0,LY(0,117)},{CX,117},{CX,476},{0,LY(0,476)}})
fill(face - wins, brick_shM, {tool="filbert 10", angle=0.38, length={30,90}, coverage=2.4})

--@ chunk 34
SVP = {-1500, 400}
function svp_line_y(px,py,x) return py + (x-px)*(py-SVP[2])/(px-SVP[1]) end
SHD2 = mask(function(x,y) return (x<CX and y < svp_line_y(CX,475,x)) and 1 or 0 end)
FGS = mask(function(x,y) return (y > svp_line_y(1000,598,x)) and 1 or 0 end)
fill(M_walk - SHD2, walk_sun, {tool="filbert 10", angle=-0.03, length={30,80}, coverage=2.4})
fill(M_street - FGS, street_sun, {tool="filbert 14", angle=0.06, length={40,120}, coverage=2.4})
print(svp_line_y(CX,475,0), svp_line_y(1000,598,0))

--@ chunk 35
fill(M_street * FGS, street_sh, {tool="filbert 14", angle=0.06, length={40,120}, coverage=2.4})

--@ chunk 36
sash = pile{{"lead white",5},{"yellow ochre",0.35},{"pale cadmium",0.1}, medium=0.05, name="sash"}
local b = brush{kind="flat", width=2.6}
local function seg(x0,y0,x1,y1) b:stroke({{x0,y0},{x1,y1}},{pressure=0.7}) end
for i,s in ipairs(RWIN) do
  local x0,x1,sh=s[1],s[2],s[3]
  b:reload(sash,0.7)
  -- jambs
  seg(x0+1.3,RY(x0,165),x0+1.3,RY(x0,265))
  seg(x1-1.3,RY(x1,165),x1-1.3,RY(x1,265))
  -- bottom rail
  seg(x0,RY(x0,262),x1,RY(x1,262))
  if sh < 214 then b:load(sash,0.5); seg(x0,RY(x0,216),x1,RY(x1,216)) end
  -- shade bottom edge as a darker line? leave
end

--@ chunk 37
shade_y_sh = pile{{"lead white",1.5},{"yellow ochre",0.8},{"red earth",0.25},{"ultramarine blue",0.25},{"bone black",0.1}, medium=0.05, name="shade_y_sh"}
shade_gr_sh = pile{{"viridian",1},{"yellow ochre",0.7},{"lead white",0.6},{"bone black",0.3},{"red earth",0.15}, medium=0.05, name="shade_gr_sh"}
for i,s in ipairs(RWIN) do
  local x0,x1,sh=s[1],s[2],s[3]
  local p = (i==2 or i==4) and shade_gr_sh or shade_y_sh
  local m = poly({{x0+3,RY(x0,166)},{x1-3,RY(x1,166)},{x1-3,RY(x1,sh-4)},{x1-9,RY(x1-9,sh-4)},{x1-9,RY(x1-9,172)},{x0+3,RY(x0,172)}})
  fill(m, p, {hand="detail", tool="round 2.5", angle=1.5708, length={4,12}, coverage=3})
end

--@ chunk 38
brick_cast = pile{{"red earth",1.6},{"burnt sienna",0.4},{"ultramarine blue",0.45},{"bone black",0.2},{"lead white",0.45}, medium=0.04, name="brick_cast"}
-- cornice shadow on the right face: a band below soffit, slightly wider at the far end
local cs = poly({{CX,RY(CX,117)},{REND,RY(REND,117)},{REND,RY(REND,129)},{CX+40,RY(CX+40,130)},{CX,RY(CX,126)}})
fill(cs, brick_cast, {hand="detail", tool="flat 4", angle=RANG, length={20,50}, coverage=2.5})
-- under sills: a small cast shadow slanting left
for i,s in ipairs(RWIN) do
  local x0,x1=s[1],s[2]
  local m = poly({{x0-7,RY(x0-7,273)},{x1+7,RY(x1+7,273)},{x1+3,RY(x1+3,279)},{x0-11,RY(x0-11,279)}})
  fill(m, brick_cast, {hand="detail", tool="flat 2.5", angle=RANG, length={10,30}, coverage=2.5})
  local m2 = poly({{x0-7,RY(x0-7,165)},{x0,RY(x0,165)},{x0,RY(x0,172)},{x0-7,RY(x0-7,170)}})
end

--@ chunk 39
aw_g = pile{{"viridian",1},{"yellow ochre",0.8},{"lead white",0.7},{"bone black",0.2}, medium=0.04, name="aw_g"}
aw_c = pile{{"lead white",5},{"yellow ochre",0.5},{"pale cadmium",0.25}, medium=0.04, name="aw_c"}
AW = {x0=346, x1=602, dx=16, dy=40, val=13}
function aw_top(t) local x=lerp(AW.x0,AW.x1,t); return {x, RY(x,338)} end
function aw_front(t) local p=aw_top(t); return {p[1]+AW.dx, p[2]+AW.dy} end
local n=11
for i=0,n-1 do
  local t0,t1=i/n,(i+1)/n
  local a,b,c,d=aw_top(t0),aw_top(t1),aw_front(t1),aw_front(t0)
  local m=poly({a,b,c,d})
  local v=poly({d,c,{c[1],c[2]+AW.val},{d[1],d[2]+AW.val}})
  fill(m+v, (i%2==0) and aw_g or aw_c, {hand="detail", tool="flat 4", angle=math.atan(AW.dy,AW.dx), length={10,30}, coverage=2.6})
end

--@ chunk 40
print(wait(6*24*60)); for _,p in ipairs({{400,330},{500,200},{100,300},{600,560},{420,120}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 41
AW = {x0=346, x1=602, dx=5, dy=34, val=12}
local n=11
local sl = math.atan(AW.dy,AW.dx)
aw_g_v = pile{{"viridian",1},{"yellow ochre",0.9},{"lead white",1.0},{"bone black",0.15}, medium=0.04, name="aw_g_v"}
-- patch the sliver from the old awning on the right
fill(poly({{603,RY(603,338)-1},{622,RY(622,338)},{622,RY(622,392)},{603,RY(603,392)}}), green_sun2, {hand="detail", tool="flat 3", angle=1.5708, length={6,20}, coverage=2.6})
for i=0,n-1 do
  local t0,t1=i/n,(i+1)/n
  local a,b,c,d=aw_top(t0),aw_top(t1),aw_front(t1),aw_front(t0)
  fill(poly({a,b,c,d}), (i%2==0) and aw_g or aw_c, {hand="detail", tool="flat 4", angle=sl, length={10,30}, coverage=2.6})
end

--@ chunk 42
local n=11
aw_c_v = pile{{"lead white",5},{"yellow ochre",0.6},{"pale cadmium",0.3},{"red earth",0.05}, medium=0.04, name="aw_c_v"}
for i=0,n-1 do
  local t0,t1=i/n,(i+1)/n
  local c,d=aw_front(t1),aw_front(t0)
  fill(poly({d,c,{c[1],c[2]+AW.val},{d[1],d[2]+AW.val}}), (i%2==0) and aw_g_v or aw_c_v, {hand="detail", tool="flat 3", angle=1.5708, length={5,14}, coverage=2.8})
end
-- awning cast shadow on the shopfront below valance
shopsh_g = pile{{"viridian",1},{"bone black",0.8},{"ultramarine blue",0.3},{"burnt sienna",0.2},{"lead white",0.1}, medium=0.04, name="shopsh_g"}
glass_d = pile{{"ultramarine blue",1},{"burnt sienna",1},{"bone black",0.8},{"lead white",0.1}, medium=0.04, name="glass_d"}
local L,R = aw_front(0), aw_front(1)
local top0, top1 = L[2]+AW.val, R[2]+AW.val
local shadow = poly({{L[1]-14,top0-6},{R[1],top1},{R[1]-8,top1+16},{L[1]-14,top0+14}})
local glass_r = poly(RQ(356,590,347,438))
fill(shadow*glass_r, glass_d, {hand="detail", tool="flat 3", angle=RANG, length={10,30}, coverage=2.6})
fill(shadow-glass_r, shopsh_g, {hand="detail", tool="flat 3", angle=RANG, length={10,30}, coverage=2.6})

--@ chunk 43
print(wait(5*24*60)); print(drying(450,385), drying(450,375), drying(400,360))

--@ chunk 44
print(wait(5*24*60)); print(drying(450,385), drying(450,375), drying(400,360), drying(355,392))

--@ chunk 45
print(wait(4*24*60)); print(drying(450,385), drying(450,375), drying(400,360), drying(355,392))

--@ chunk 46
print(wait(3*24*60)); print(drying(450,385), drying(450,375), drying(400,360), drying(355,392), drying(420,350))

--@ chunk 47
local L,R = aw_front(0), aw_front(1)
local b0, b1 = L[2]+AW.val, R[2]+AW.val
local shadow = poly({{L[1]-12,b0},{R[1]-2,b1},{R[1]-10,b1+15},{L[1]-12,b0+15}})
local glass_r = poly(RQ(356,590,347,438))
fill(shadow*glass_r, glass_d, {hand="detail", tool="flat 3", angle=RANG, length={10,30}, coverage=2.6})
fill(shadow-glass_r, shopsh_g, {hand="detail", tool="flat 3", angle=RANG, length={10,30}, coverage=2.6})
local n=11
for i=0,n-1 do
  local t0,t1=i/n,(i+1)/n
  local c,d=aw_front(t1),aw_front(t0)
  fill(poly({d,c,{c[1],c[2]+AW.val},{d[1],d[2]+AW.val}}), (i%2==0) and aw_g_v or aw_c_v, {hand="detail", tool="flat 2.5", angle=1.5708, length={5,12}, coverage=3})
end

--@ chunk 48
-- figure geometry (seated man, chair tilted back against wall)
FIG = {
 head={757,401,6.2,7},
 torso={{748,412},{763,411},{767,425},{766,444},{749,446},{745,428}},
 thigh={{750,438},{767,434},{792,436},{794,447},{760,452},{750,450}},
 shin={{786,437},{796,438},{800,466},{791,467}},
}
local m = poly(FIG.torso,true) + poly(FIG.thigh,true) + poly(FIG.shin,true) + ellipse(FIG.head[1],FIG.head[2],FIG.head[3],FIG.head[4])
-- shadow on the wall: shifted left and down slightly, then stopping at the wall base
local sw = (poly({{748-24,412+6},{763-24,411+6},{767-24,425+6},{766-24,444+6},{749-24,446+6},{745-24,428+6}},true)
  + ellipse(757-26,401+6,6.5,7.5) + poly({{750-24,438+6},{767-24,434+6},{790-24,436+6},{790-24,447+6},{760-24,452+6},{750-24,450+6}},true)) * above(function(x) return RY(x,475) end)
-- chair shadow: back slats on wall
sw = sw + ribbon({{752-22,410+5},{748-22,462}},2.5) * above(function(x) return RY(x,475) end)
fill(sw, brick_cast, {hand="detail", tool="round 3", angle=1.5708, length={5,15}, coverage=3})
-- ground shadow: from feet/chair legs toward the left along the SVP direction
local function gdir(x,y,len) local dx = -(len); local dy = dx*(y-SVP[2])/(x-SVP[1]); return {x+dx,y+dy} end
local gs = poly({{748,467},{800,468},gdir(800,468,95),{gdir(748,467,40)[1], gdir(748,467,40)[2]-2}, {gdir(748,467,40)[1]-10, RY(gdir(748,467,40)[1],475)}, {748, RY(748,475)}})
fill(gs, walk_sh2, {hand="detail", tool="round 3", angle=0.1, length={5,20}, coverage=3})
FIGM = m

--@ chunk 49
fill(poly({{705,463.5},{800,464.5},{803,470},{760,472},{705,467}}), walk_sh2, {hand="detail", tool="round 2.5", angle=0.0, length={5,20}, coverage=3})
shirt_l = pile{{"lead white",5},{"yellow ochre",0.25},{"pale cadmium",0.12}, medium=0.04, name="shirt_l"}
shirt_s = pile{{"lead white",2},{"ultramarine blue",0.25},{"red earth",0.15},{"bone black",0.1}, medium=0.04, name="shirt_s"}
trou = pile{{"bone black",0.6},{"ultramarine blue",0.5},{"burnt sienna",0.3},{"lead white",0.5}, medium=0.04, name="trou"}
trou_l = pile{{"bone black",0.4},{"ultramarine blue",0.4},{"burnt sienna",0.2},{"lead white",1.0},{"yellow ochre",0.2}, medium=0.04, name="trou_l"}
skin_l = pile{{"lead white",2},{"red earth",0.5},{"yellow ochre",0.5},{"deep cadmium",0.1}, medium=0.04, name="skin_l"}
skin_s = pile{{"red earth",1},{"burnt sienna",0.4},{"ultramarine blue",0.25},{"lead white",0.5}, medium=0.04, name="skin_s"}
chairw = pile{{"burnt sienna",1},{"red earth",0.6},{"yellow ochre",0.5},{"lead white",0.3}, medium=0.04, name="chairw"}
straw = pile{{"lead white",2},{"yellow ochre",1.2},{"pale cadmium",0.2},{"red earth",0.08}, medium=0.04, name="straw"}
hatband = pile{{"bone black",0.6},{"ultramarine blue",0.3},{"burnt sienna",0.2}, medium=0.04, name="hatband"}

--@ chunk 50
local b = brush{kind="round", width=2.2}
b:reload(chairw, 0.7)
b:stroke({{751,467},{748,440},{745,413}},{pressure=0.8})
b:stroke({{756,467},{753,446}},{pressure=0.7})
b:load(chairw,0.6)
b:stroke({{779,447},{784,463}},{pressure=0.7})
b:stroke({{775,447},{778,462}},{pressure=0.6})
b:stroke({{746,449},{780,447}},{pressure=0.9})
-- trousers
local th = poly({{750,437},{768,434},{790,435},{794,446},{760,450},{750,449}},true)
local sh = poly({{786,436},{796,437},{801,463},{791,464}},true)
fill(th+sh, trou, {hand="detail", tool="round 3", angle=0.1, length={5,14}, coverage=3})
-- lit tops of trousers
fill(poly({{760,435},{790,435},{795,440},{762,439}},true) + poly({{795,439},{797,439},{801,462},{798,462}}), trou_l, {hand="detail", tool="round 2", angle=0.1, length={4,10}, coverage=2.5})
-- shoes
local sb = brush{kind="round", width=3.2}
sb:reload(hatband,0.7); sb:stroke({{791,465},{804,465}},{pressure=0.9})

--@ chunk 51
local torso = poly({{749,413},{762,411},{766,418},{766,432},{764,442},{752,443},{748,430}},true)
fill(torso, shirt_s, {hand="detail", tool="round 3", angle=1.5708, length={5,14}, coverage=3})
local lit = poly({{757,412},{762,411},{766,418},{766,432},{764,441},{759,441},{760,425}},true)
fill(lit, shirt_l, {hand="detail", tool="round 2.5", angle=1.5708, length={4,12}, coverage=3})
-- neck and head
local neck = poly({{754,407},{760,407},{760,413},{754,413}})
fill(neck, skin_s, {hand="detail", tool="round 1.6", angle=1.5708, length={3,6}, coverage=3})
local head = ellipse(757.5,402,5.2,6.2)
fill(head, skin_s, {hand="detail", tool="round 2", angle=1.5708, length={3,8}, coverage=3})
fill(ellipse(759.5,402.5,3.4,5.5), skin_l, {hand="detail", tool="round 1.6", angle=1.5708, length={3,6}, coverage=3})

--@ chunk 52
-- arm: sleeve
local sl = poly({{760,414},{766,414},{770,428},{765,431}},true)
fill(sl, shirt_l, {hand="detail", tool="round 2", angle=1.2, length={4,10}, coverage=3})
-- forearm
local b = brush{kind="round", width=3.2, point=0.5}
b:reload(skin_l,0.6)
b:stroke({{767,429},{775,433},{783,434}},{pressure=0.75})
b:reload(skin_s,0.4)
b:stroke({{767,431.5},{776,435.5},{782,436}},{pressure=0.4})
-- hat: crown and brim
local crown = poly({{751.5,390},{763.5,390},{764,396.5},{751,396.5}})
fill(crown, straw, {hand="detail", tool="flat 2", angle=0, length={4,10}, coverage=3})
local hb = brush{kind="flat", width=2}
hb:reload(hatband,0.6); hb:stroke({{751,395},{764,395}},{pressure=0.8})
local br = brush{kind="flat", width=2.2}
br:reload(straw,0.7); br:stroke({{747,397.5},{768,397.3}},{pressure=0.9})
-- brim shadow on face
local fs = brush{kind="round", width=1.8}
fs:reload(skin_s,0.5); fs:stroke({{754,399.5},{762,399.5}},{pressure=0.6})

--@ chunk 53
BSH = mask(function(x,y) return (x < CX and y > 470 and y < 475 + (CX-x)*(73/330) + 0) and 0 or 0 end)
-- building shadow: region left of line from (330,475) to (0,548), below the building base
function bsh_y(x) return 475 + (CX-x)*(73/330) end
BSH = mask(function(x,y) return (x <= CX+2 and y >= LY(x,475)-1 and y <= bsh_y(x)) and 1 or 0 end)
fill((BSH * M_walk), walk_sh2, {tool="filbert 8", angle=0.25, length={20,60}, coverage=2.5})
fill((BSH * M_street) - FGS, street_sh, {tool="filbert 8", angle=0.25, length={20,60}, coverage=2.5})

--@ chunk 54
curb_face = pile{{"lead white",1.6},{"ultramarine blue",0.3},{"red earth",0.25},{"bone black",0.15},{"yellow ochre",0.2}, medium=0.04, name="curb_face"}
curb_top = pile{{"lead white",5},{"yellow ochre",0.4},{"pale cadmium",0.1}, medium=0.04, name="curb_top"}
-- right curb line from corner (320,546) to (1000,510): face below
local function cy(x) return 546 + (x-320)*(510-546)/680 end
local pts = {}
for x=300,1000,20 do pts[#pts+1] = {x, cy(x)} end
local face = poly({{290,542},{320,546},{1000,510},{1000,515.5},{320,552},{290,548}})
fill(face - FGS, curb_face, {hand="detail", tool="flat 3", angle=-0.05, length={20,60}, coverage=3})
local b = brush{kind="flat", width=2}
b:reload(curb_top,0.6)
b:stroke({{326,545.5},{620,530},{1000,509.5}},{pressure=0.6})

--@ chunk 55
print(wait(8*24*60)); for _,p in ipairs({{758,425},{770,445},{760,402},{735,430},{200,500},{500,537}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 56
bgwin2 = pile{{"ultramarine blue",0.6},{"burnt sienna",0.4},{"bone black",0.2},{"lead white",1.4},{"cerulean blue",0.2}, medium=0.04, name="bgwin2"}
local m
local function add(w) m = m and (m+w) or w end
for _,y in ipairs({262,300,338,376,414}) do
  add(rect(892,y,14,20)); add(rect(921,y,14,20))
end
for _,y in ipairs({292,330,368,406}) do
  add(rect(956,y,12,18)); add(rect(980,y,12,18))
end
fill(m, bgwin2, {hand="detail", tool="flat 2.5", angle=1.5708, length={5,14}, coverage=3})
-- cornice line on the distant buildings
local b = brush{kind="flat", width=3}
b:reload(curb_face,0.5); b:stroke({{882,252},{945,252}},{pressure=0.7})
b:load(curb_face,0.5); b:stroke({{945,284},{1000,284}},{pressure=0.7})

--@ chunk 57
cream_sh2 = pile{{"lead white",1.6},{"ultramarine blue",0.25},{"red earth",0.3},{"bone black",0.1},{"yellow ochre",0.3}, medium=0.04, name="cream_sh2"}
for _,s in ipairs({{90,135},{215,268}}) do
  fill(poly(LQ(s[1]-4,s[2]+5,153,165)), cream_sh2, {hand="detail", tool="flat 3", angle=0.4, length={6,20}, coverage=3})
  fill(poly(LQ(s[1]-5,s[2]+6,265,273)), cream_sh2, {hand="detail", tool="flat 3", angle=0.4, length={6,20}, coverage=3})
end
-- a shade half down in the nearer window, in shadow
shade_sh = pile{{"lead white",1},{"yellow ochre",0.6},{"red earth",0.25},{"ultramarine blue",0.3},{"bone black",0.12}, medium=0.04, name="shade_sh"}
fill(poly(LQ(218,265,167,205)), shade_sh, {hand="detail", tool="flat 2.5", angle=1.5708, length={5,14}, coverage=3})
local b = brush{kind="flat", width=2}
b:reload(cream_sh2,0.6)
for _,s in ipairs({{90,135},{215,268}}) do
  b:stroke({{s[1]+1,LY(s[1],165)},{s[1]+1,LY(s[1],265)}},{pressure=0.6})
  b:stroke({{s[2]-1,LY(s[2],165)},{s[2]-1,LY(s[2],265)}},{pressure=0.6})
  b:load(cream_sh2,0.5)
  b:stroke({{s[1],LY(s[1],215)},{s[2],LY(s[2],215)}},{pressure=0.6})
end

--@ chunk 58
print(drying(500,320), drying(650,320))
fascia = pile{{"viridian",1},{"bone black",0.55},{"yellow ochre",0.35},{"lead white",0.25}, medium=0.04, name="fascia"}
fill(poly(RQ(330,700,300,335.5)), fascia, {hand="detail", tool="flat 4", angle=RANG, length={15,40}, coverage=3})
fill(poly(LQ(225,330,300,335.5)), green_sh2, {hand="detail", tool="flat 4", angle=0.4, length={15,40}, coverage=3})
-- a cream moulding line at fascia top
local b = brush{kind="flat", width=2.2}
b:reload(aw_c,0.6); b:stroke({{330,RY(330,299)},{700,RY(700,299)}},{pressure=0.7})
b:reload(cream_sh2,0.6); b:stroke({{225,LY(225,299)},{330,299}},{pressure=0.7})

--@ chunk 59
-- door lower panel and frame
fill(poly(RQ(614,668,432,473)), green_sun2, {hand="detail", tool="flat 3", angle=1.5708, length={6,16}, coverage=3})
fill(poly(RQ(812,852,432,473)), brick_cast, {hand="detail", tool="flat 3", angle=1.5708, length={6,16}, coverage=3})
-- mullions in shop window
local b = brush{kind="flat", width=2.4}
for _,x in ipairs({433,512}) do
  b:reload(green_sun2,0.6); b:stroke({{x,RY(x,392)},{x,RY(x,438)}},{pressure=0.7})
end
-- barber pole
PX=713
local pole = poly({{PX-3.5,RY(PX,362)},{PX+3.5,RY(PX,362)},{PX+3.5,RY(PX,416)},{PX-3.5,RY(PX,416)}})
fill(pole, shirt_l, {hand="detail", tool="round 2", angle=1.5708, length={4,10}, coverage=3})
POLE=pole

--@ chunk 60
print(drying(610,360), drying(605,385))
fill(poly({{603,RY(603,337)},{613,RY(613,337)},{613,RY(613,351)},{603,RY(603,351)}}) + poly({{606,RY(606,351)},{613,RY(613,351)},{613,RY(613,394)},{606,RY(606,394)}}), green_sun2, {hand="detail", tool="flat 2.5", angle=1.5708, length={5,12}, coverage=3})

--@ chunk 61
print(wait(10*24*60)); for _,p in ipairs({{500,318},{713,390},{758,425},{608,370},{930,300},{240,190}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 62
pole_red = pile{{"red earth",0.6},{"deep cadmium",0.5},{"lead white",0.15},{"burnt sienna",0.1}, medium=0.04, name="pole_red"}
pole_blue = pile{{"ultramarine blue",1},{"lead white",0.4},{"bone black",0.1}, medium=0.04, name="pole_blue"}
local b = brush{kind="round", width=1.8}
local y0, y1 = RY(PX,362), RY(PX,416)
local k=0
for y=y0+2, y1-4, 6 do
  k=k+1
  b:reload((k%2==0) and pole_blue or pole_red, 0.5)
  b:stroke({{PX-3.2,y},{PX+3.2,y+4.5}},{pressure=0.7})
end
-- caps
local c = brush{kind="flat", width=2.6}
c:reload(fascia,0.6); c:stroke({{PX-4.8,y0-1},{PX+4.8,y0-1}},{pressure=0.9})
c:load(fascia,0.6); c:stroke({{PX-4.8,y1+1},{PX+4.8,y1+1}},{pressure=0.9})
-- shading on left side of pole
local s = brush{kind="round", width=1.3}
s:reload(shirt_s,0.4); s:stroke({{PX-3,y0+1},{PX-3,y1-1}},{pressure=0.5})
-- pole shadow on the wall, left and slightly lower
fill(poly({{PX-20,y0+5},{PX-14,y0+5},{PX-14,y1+6},{PX-20,y1+6}}), brick_cast, {hand="detail", tool="round 2", angle=1.5708, length={4,10}, coverage=3})

--@ chunk 63
print(wait(6*24*60)); for _,p in ipairs({{500,318},{713,390},{608,370},{713,380}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 64
brick_lt = pile{{"lead white",1.6},{"red earth",2},{"yellow ochre",0.8},{"deep cadmium",0.25}, blot=0.15, name="brick_lt"}
brick_dk = pile{{"red earth",2},{"burnt sienna",0.5},{"lead white",0.8},{"yellow ochre",0.3},{"ultramarine blue",0.1}, blot=0.15, name="brick_dk"}
local excl = nil
local function ex(m) excl = excl and (excl+m) or m end
for _,s in ipairs(RWIN) do ex(poly(RQ(s[1]-8,s[2]+8,151,281))) end
ex(above(function(x) return RY(x,131) end))
ex(poly(RQ(326,702,296,480)))
ex(rect(PX-22,RY(PX,356),30,RY(PX,422)-RY(PX,356)))
ex(poly({{716,384},{770,384},{772,410},{808,430},{810,478},{700,478},{716,430}}))
ex(poly(RQ(808,856,348,480)))
BRICK_OPEN = (M_rf - excl):shrink(0.5)
local trial = BRICK_OPEN * rect(440,170,70,100)
work(trial, {hand="scumble", pile=brick_lt, coverage=0.7, clip=true, angle=RANG, seed=11})

--@ chunk 65
r = rag()
r:dip(0.6)
r:wipe(BRICK_OPEN * rect(438,168,74,104), {pressure=0.7, angle=1.5708, passes=3, refold=0.3})

--@ chunk 66
r = rag{width=20}
r:dip(0.8)
r:wipe(rect(442,170,66,100), {pressure=0.8, angle=1.5708, passes=3, refold=0.15})
r:refold(); r:dip(0.8)
r:wipe(rect(424,170,18,98), {pressure=0.6, angle=1.5708, passes=2, refold=0.15})

--@ chunk 67
r = rag{width=10}
r:dip(0.9)
r:wipe({{445,RY(445,172)},{505,RY(505,172)}}, {pressure=0.8})
r:refold(); r:dip(0.9)
r:wipe({{445,RY(445,262)},{505,RY(505,262)}}, {pressure=0.8})
r:refold(); r:dip(0.9)
r:wipe({{508,RY(508,175)},{508,RY(508,265)}}, {pressure=0.6})
r:refold(); r:dip(0.9)
r:wipe({{440,RY(440,175)},{440,RY(440,265)}}, {pressure=0.6})

--@ chunk 68
brick_a = pile{{"lead white",1.2},{"red earth",2.2},{"yellow ochre",0.55},{"bone black",0.06}, medium=0.03, name="brick_a"}
brick_b = pile{{"lead white",1.5},{"red earth",2.0},{"yellow ochre",0.7},{"deep cadmium",0.12}, medium=0.03, name="brick_b"}
brick_c = pile{{"lead white",1.0},{"red earth",2.2},{"burnt sienna",0.3},{"ultramarine blue",0.08},{"yellow ochre",0.4}, medium=0.03, name="brick_c"}
NZ = noise{seed=7, period=140, octaves=3}
WALLPILES = {{brick_a, 1},{brick_b, function(x,y) return clamp(0.2+NZ(x,y)*0.9,0,1) end},{brick_c, function(x,y) return clamp(0.15-NZ(x,y)*0.9,0,1) end}}
work(BRICK_OPEN * rect(441,165,68,110), {hand="body", tool="filbert 6", piles=WALLPILES, angle=RANG, length={15,45}, coverage=2.4, fill=true, clip=true, seed=21})

--@ chunk 69
for _,p in ipairs({{470,410},{270,400},{640,410},{830,410}}) do print(p[1],p[2],drying(p[1],p[2])) end
-- letters: strokes in local units (0..1 wide, 0..1 tall)
LET = {
 B={{{0,0},{0,1}},{{0,0},{0.7,0},{0.85,0.12},{0.85,0.35},{0.7,0.48},{0,0.48}},{{0.7,0.48},{0.9,0.6},{0.9,0.88},{0.75,1},{0,1}}},
 A={{{0,1},{0.45,0},{0.9,1}},{{0.2,0.62},{0.7,0.62}}},
 R={{{0,1},{0,0},{0.7,0},{0.85,0.13},{0.85,0.36},{0.7,0.5},{0,0.5}},{{0.45,0.5},{0.9,1}}},
 E={{{0.85,0},{0,0},{0,1},{0.85,1}},{{0,0.5},{0.65,0.5}}},
 S={{{0.85,0.12},{0.7,0},{0.15,0},{0,0.15},{0.05,0.38},{0.8,0.58},{0.9,0.8},{0.75,1},{0.15,1},{0,0.86}}},
 H={{{0,0},{0,1}},{{0.85,0},{0.85,1}},{{0,0.5},{0.85,0.5}}},
 O={{{0.45,0},{0.1,0.12},{0,0.5},{0.1,0.88},{0.45,1},{0.8,0.88},{0.9,0.5},{0.8,0.12},{0.45,0}}},
 P={{{0,1},{0,0},{0.7,0},{0.88,0.15},{0.88,0.38},{0.7,0.52},{0,0.52}}},
}

--@ chunk 70
letter_p = pile{{"lead white",5},{"yellow ochre",0.4},{"pale cadmium",0.15}, medium=0.08, name="letter_p"}
function write(word, x0, adv, w, yc0, yc1, b)
  local x = x0
  for i=1,#word do
    local ch = word:sub(i,i)
    if LET[ch] then
      for _,st in ipairs(LET[ch]) do
        local pts = {}
        for _,q in ipairs(st) do
          local px = x + q[1]*w
          local py = lerp(RY(px,yc0), RY(px,yc1), q[2])
          pts[#pts+1] = {px,py}
        end
        b:load(letter_p, 0.5)
        b:stroke(pts, {pressure=0.75})
      end
    end
    x = x + adv
  end
end
LB = brush{kind="round", width=2.0}
LB:reload(letter_p,0.6)
write("B", 425, 17, 11, 308, 327, LB)

--@ chunk 71
write(" ARBER SHOP", 425, 17, 11, 308, 327, LB)

--@ chunk 72
glass_ref = pile{{"lead white",1},{"ultramarine blue",0.35},{"bone black",0.15},{"red earth",0.1}, turps=0.5, name="glass_ref"}
local g = poly(RQ(357,589,398,437))
local band = poly({{470,440},{500,385},{530,385},{500,440}}) + poly({{535,440},{560,385},{572,385},{547,440}})
work(g*band, {hand="body", tool="filbert 5", pile=glass_ref, angle=-1.1, length={10,30}, coverage=1.6, clip=true, seed=3})

--@ chunk 73
r = rag{width=12}
r:dip(0.9)
r:wipe(poly({{466,442},{498,382},{534,382},{504,442}}) + poly({{531,442},{558,382},{576,382},{551,442}}), {pressure=0.75, angle=-1.1, passes=3, refold=0.15})

--@ chunk 74
blend(poly(RQ(357,589,399,437)) * poly({{455,442},{495,380},{580,380},{545,442}}), {angle=-1.1})

--@ chunk 75
work(BRICK_OPEN - rect(441,165,68,110), {hand="body", tool="filbert 7", piles=WALLPILES, angle=RANG, length={15,50}, coverage=2.2, fill=true, clip=true, seed=22})

--@ chunk 76
print(wait(12*24*60)); for _,p in ipairs({{600,200},{500,410},{500,320},{800,330}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 77
glaze_dk = pile{{"ultramarine blue",1},{"burnt sienna",0.9},{"bone black",0.5}, medium=0.6, name="glaze_dk"}
local g = poly(RQ(357,589,399,437)) * poly({{440,445},{490,375},{600,375},{560,445}})
work(g, {hand="glaze", tool={kind="filbert", width=8, stiffness=0.3}, pile=glaze_dk, angle=-1.1, coverage=1.2, clip=true, seed=5})

--@ chunk 78
r = rag{width=14}
r:dip(0.9)
r:wipe(poly(RQ(357,589,399,437)) * poly({{440,445},{490,375},{600,375},{560,445}}), {pressure=0.8, angle=-1.1, passes=4, refold=0.1})

--@ chunk 79
glass_lo = pile{{"ultramarine blue",1},{"burnt sienna",0.7},{"bone black",0.4},{"lead white",0.8},{"viridian",0.15}, medium=0.04, name="glass_lo"}
local g = poly(RQ(357,589,399,437))
work(g, {hand="body", tool="filbert 5", piles={{glass2, function(x,y) return clamp((RY(x,437)-y)/30,0,1) end},{glass_lo, function(x,y) return clamp(1-(RY(x,437)-y)/30,0,1) end}},
  angle=RANG, length={15,40}, coverage=2.6, fill=true, clip=true, seed=8})

--@ chunk 80
fill(poly(RQ(470,589,393,402)), glass_d, {hand="detail", tool="flat 3", angle=RANG, length={10,25}, coverage=3})
fill(poly(RQ(460,580,435,441)), green_sun2, {hand="detail", tool="flat 2.5", angle=RANG, length={10,25}, coverage=3})

--@ chunk 81
print(wait(14*24*60)); for _,p in ipairs({{600,200},{500,410},{420,420},{560,440},{470,395}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 82
local n=11
AWM = poly({aw_top(0),aw_top(1),{aw_front(1)[1],aw_front(1)[2]+AW.val},{aw_front(0)[1],aw_front(0)[2]+AW.val}})
local L,R = aw_front(0), aw_front(1)
local b0, b1 = L[2]+AW.val, R[2]+AW.val
AWSH = poly({{L[1]-12,b0},{R[1]-2,b1},{R[1]-10,b1+15},{L[1]-12,b0+15}})
GLASS = poly(RQ(357,589,399,437))
DOOR = poly(RQ(614,668,350,472))
-- the shopfront green field: pilaster strip between awning/glass and door, and bulkhead
local front = poly(RQ(330,700,336,475))
local field = front - AWM - AWSH:grow(0.5) - GLASS - DOOR - POLE:grow(1)
SHOPFIELD = field
local m = field * (poly(RQ(590,700,336,475)) + poly(RQ(330,700,437,475)) + poly(RQ(330,357,336,475)))
fill(m, green_sun2, {hand="body", tool="filbert 5", angle=1.5708, length={10,30}, coverage=2.4})

--@ chunk 83
fill(poly(RQ(355,462,434.5,441)), green_sun2, {hand="detail", tool="flat 2.5", angle=RANG, length={10,25}, coverage=3})
joint = pile{{"lead white",2},{"yellow ochre",0.4},{"red earth",0.2},{"bone black",0.15},{"ultramarine blue",0.08}, medium=0.05, name="joint"}
local function cy(x) return 546 + (x-320)*(510-546)/680 end
local jb = brush{kind="round", width=1.3}
for _,xb in ipairs({420, 520, 625, 735, 850, 965}) do
  local yb = RY(xb,475)+1
  -- direction toward left VP: line from (xb,yb) heading to (-250,400) extended downward the other way
  -- the joint is perpendicular to the facade: goes toward left VP; downward toward viewer is away from VP
  local dx, dy = xb-LVP, yb-EY
  local t = 0.0
  local x2 = xb; local y2 = yb
  -- extend until reaching curb
  for s=1,400 do
    local xx = xb + dx*s/400*0.5; local yy = yb + dy*s/400*0.5
    if yy >= cy(xx)-1 then x2,y2 = xx,yy break end
  end
  jb:reload(joint,0.4)
  jb:stroke({{xb,yb},{x2,y2}},{pressure=0.45})
end

--@ chunk 84
r = rag{width=8}
local function cy(x) return 546 + (x-320)*(510-546)/680 end
for _,xb in ipairs({420, 520, 625, 735, 850, 965}) do
  local yb = RY(xb,475)+1
  local dx, dy = xb-LVP, yb-EY
  local x2,y2 = xb+dx*0.5, yb+dy*0.5
  r:refold(); r:dip(0.9)
  r:wipe({{xb,yb},{x2,y2}}, {pressure=0.7})
end

--@ chunk 85
print(wait(12*24*60)); for _,p in ipairs({{620,450},{400,440},{640,365},{520,470},{350,400}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 86
print(wait(5*24*60)); print(drying(350,400), drying(340,450))
-- door glass, cleaned edges
fill(poly(RQ(616,666,352,430)), glass2, {hand="detail", tool="flat 3", angle=1.5708, length={8,20}, coverage=3})
-- door frame dark green line
dkgreen = pile{{"viridian",1},{"bone black",0.9},{"burnt sienna",0.2},{"lead white",0.1}, medium=0.05, name="dkgreen"}
local b = brush{kind="flat", width=2.2}
local function q(x0,yc0,x1,yc1,p) b:reload(p,0.5); b:stroke({{x0,RY(x0,yc0)},{x1,RY(x1,yc1)}},{pressure=0.75}) end
q(613,349,613,474,dkgreen); q(669,349,669,474,dkgreen); q(613,348,669,348,dkgreen)
q(616,431,666,431,dkgreen)
-- shop window frame
q(356,397,356,438,dkgreen); q(590,397,590,438,dkgreen); q(356,438.5,590,438.5,dkgreen)
-- mullions
for _,x in ipairs({433,512}) do q(x,400,x,437,dkgreen) end
-- lit edge of mullions (sun from right)
local c = brush{kind="round", width=1.0}
for _,x in ipairs({434.6,513.6}) do c:reload(green_sun,0.4); c:stroke({{x,RY(x,401)},{x,RY(x,437)}},{pressure=0.5}) end

--@ chunk 87
sky_top = pile{{"lead white",2.2},{"cobalt blue",1.2},{"ultramarine blue",0.7},{"cerulean blue",0.4}, medium=0.05, name="sky_top"}
sky_mid = pile{{"lead white",3.5},{"cobalt blue",0.9},{"cerulean blue",0.6},{"ultramarine blue",0.2}, medium=0.05, name="sky_mid"}
sky_low = pile{{"lead white",5},{"cerulean blue",0.7},{"yellow ochre",0.12},{"pale cadmium",0.04}, medium=0.05, name="sky_low"}

--@ chunk 88
sky_top:add{{"cobalt blue",0.5},{"ultramarine blue",0.4}}
local function wt(y) return y end
local SK = M_sky:shrink(0.8)
work(SK, {hand="body", tool="filbert 14", piles={
   {sky_top, function(x,y) return clamp(1 - y/170,0,1)^1.2 end},
   {sky_mid, function(x,y) return clamp(1 - math.abs(y-190)/150,0,1) end},
   {sky_low, function(x,y) return clamp((y-200)/120,0,1) end}},
  angle=0.02, length={40,120}, coverage=2.6, fill=true, clip=true, seed=31})

--@ chunk 89
blend(M_sky:shrink(3), {angle=0.02})

--@ chunk 90
bounce = pile{{"burnt sienna",1},{"red earth",0.6},{"deep cadmium",0.15}, medium=0.7, name="bounce"}
local face = poly({{0,LY(0,117)},{CX,117},{CX,476},{0,LY(0,476)}}) - poly(LQ(225,331,295,480)) - poly(LQ(85,140,150,280)) - poly(LQ(210,273,150,280))
local low = mask(function(x,y) local t=(y-LY(x,300))/(LY(x,476)-LY(x,300)); return clamp(t,0,1) end)
BOUNCE_M = face * low
work(face * rect(120,250,90,250), {hand="glaze", pile=bounce, coverage=1, clip=true, angle=0.38, load_at=function(x,y) return 0.2+0.6*clamp((y-LY(x,300))/(LY(x,476)-LY(x,300)),0,1) end, seed=4})

--@ chunk 91
r = rag{width=24}
r:dip(0.9)
r:wipe(rect(115,245,100,262), {pressure=0.8, angle=0.38, passes=4, refold=0.1})
r = rag{width=24}; r:dip(0.9)
r:wipe(rect(115,245,100,262), {pressure=0.85, angle=1.5708, passes=3, refold=0.1})

--@ chunk 92
brick_shW = pile{{"red earth",1.6},{"burnt sienna",0.6},{"ultramarine blue",0.55},{"bone black",0.3},{"lead white",0.45},{"yellow ochre",0.15}, medium=0.03, name="brick_shW"}

--@ chunk 93
local face = poly({{0,LY(0,240)},{CX-1,240},{CX-1,476},{0,LY(0,476)}}) - poly(LQ(223,332,295,480)) - poly(LQ(84,141,150,280)) - poly(LQ(209,274,150,280))
local function t(x,y) return clamp((y-LY(x,300))/(LY(x,476)-LY(x,300)),0,1) end
work(face, {hand="body", tool="filbert 8", piles={{brick_shM, function(x,y) return 1-t(x,y) end},{brick_shW, function(x,y) return t(x,y) end}}, angle=0.38, length={20,60}, coverage=2.4, fill=true, clip=true, seed=41})

--@ chunk 94
print(wait(14*24*60)); for _,p in ipairs({{150,30},{700,50},{150,420},{760,402},{830,420}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 95
door_w = pile{{"burnt sienna",1},{"bone black",0.5},{"viridian",0.3},{"lead white",0.25},{"red earth",0.3}, medium=0.04, name="door_w"}
door_wl = pile{{"burnt sienna",1},{"red earth",0.5},{"yellow ochre",0.4},{"lead white",0.6},{"bone black",0.1}, medium=0.04, name="door_wl"}
-- door: x 812..852, yc 352..474
fill(poly(RQ(812,852,352,474)), door_w, {hand="detail", tool="flat 3", angle=1.5708, length={8,25}, coverage=3})
DOORR = poly(RQ(812,852,352,474))

--@ chunk 96
door_sh = pile{{"burnt sienna",0.8},{"bone black",0.7},{"ultramarine blue",0.4},{"lead white",0.1}, medium=0.04, name="door_sh"}
-- cast shadow inside recess: right side and top
fill(poly({{840,RY(840,352)},{852,RY(852,352)},{852,RY(852,474)},{838,RY(838,474)}}) + poly(RQ(812,852,352,362)), door_sh, {hand="detail", tool="flat 2.5", angle=1.5708, length={6,20}, coverage=3})
-- glass pane upper door
fill(poly(RQ(819,838,372,410)), glass_d, {hand="detail", tool="round 2", angle=1.5708, length={5,14}, coverage=3})
-- lit left jamb
local b = brush{kind="flat", width=2.2}
b:reload(brick_b,0.6); b:stroke({{813.3,RY(813,353)},{813.3,RY(813,474)}},{pressure=0.7})
-- stone lintel above door
fill(poly(RQ(806,858,343,352)), corn, {hand="detail", tool="flat 3", angle=RANG, length={10,25}, coverage=3})
-- step
fill(poly({{808,RY(808,474)},{858,RY(858,474)},{860,RY(860,474)+5},{806,RY(806,474)+5}}), corn, {hand="detail", tool="flat 2.5", angle=0, length={10,25}, coverage=3})

--@ chunk 97
-- hat shadow on wall: brim + crown over head shadow
fill(poly({{719,RY(719,401)+0},{744,RY(744,401)},{744,RY(744,401)+3},{719,RY(719,401)+3}}) + rect(724,RY(724,393)-1,15,9), brick_cast, {hand="detail", tool="round 1.6", angle=0, length={3,8}, coverage=3})
-- face: lit plane, nose, ear
local b = brush{kind="round", width=2.2, point=0.6}
b:reload(skin_l,0.5)
b:stroke({{760,400.5},{761,405}},{pressure=0.6})
b:load(skin_l,0.4)
b:stroke({{762.5,401.5},{763.6,403},{762.4,404}},{pressure=0.35})
local e = brush{kind="round", width=1.6}
e:reload(skin_s,0.5); e:touch(755.5,402.5,{pressure=0.5})
-- jaw shadow under
e:load(skin_s,0.4); e:stroke({{755,406},{761,407}},{pressure=0.35})

--@ chunk 98
bgshade = pile{{"lead white",3},{"yellow ochre",0.7},{"red earth",0.15},{"cerulean blue",0.15},{"bone black",0.05}, medium=0.04, name="bgshade"}
-- some windows with shades pulled partway (top portions)
local list = {{18,66,14},{134,66,22},{76,118,10},{190,118,26},{18,170,20},{134,170,8},{76,222,16}}
for _,w in ipairs(list) do
  local x,y,h = w[1],w[2],w[3]
  fill(rect(x+1,y+1,26,h) * M_bgL2:shrink(2), bgshade, {hand="detail", tool="flat 2.5", angle=1.5708, length={4,12}, coverage=3})
end
-- cornice on bg building
local b = brush{kind="flat", width=3}
b:reload(cream_sh2,0.6); b:stroke({{0,53},{235,53}},{pressure=0.8})
-- water tower on its roof
tank = pile{{"burnt sienna",0.6},{"red earth",0.4},{"bone black",0.3},{"ultramarine blue",0.3},{"lead white",0.6}, medium=0.04, name="tank"}
tank_l = pile{{"red earth",0.5},{"yellow ochre",0.6},{"lead white",1},{"burnt sienna",0.3}, medium=0.04, name="tank_l"}
local tk = poly({{140,12},{176,12},{177,36},{139,36}})
fill(tk, tank, {hand="detail", tool="flat 3", angle=1.5708, length={5,15}, coverage=3})
fill(poly({{160,13},{176,12},{177,36},{161,36}}), tank_l, {hand="detail", tool="flat 2.5", angle=1.5708, length={5,15}, coverage=3})
fill(poly({{137,12},{158,-2},{179,12}}), tank, {hand="detail", tool="flat 2.5", angle=0, length={5,15}, coverage=3})
local l = brush{kind="round", width=1.4}
for _,x in ipairs({143,157,173}) do l:reload(tank,0.4); l:stroke({{x,36},{x,48}},{pressure=0.6}) end
l:reload(tank,0.4); l:stroke({{141,42},{175,42}},{pressure=0.5})

--@ chunk 99
hyd = pile{{"red earth",1},{"deep cadmium",0.25},{"lead white",0.25},{"burnt sienna",0.15}, medium=0.04, name="hyd"}
hyd_s = pile{{"red earth",0.8},{"burnt sienna",0.4},{"ultramarine blue",0.35},{"bone black",0.2}, medium=0.04, name="hyd_s"}
HX, HY = 575, 521   -- base center
-- shadow first (on the sidewalk, toward left along the sun direction)
local function gdir(x,y,len) local dx=-len; local dy = dx*(y-SVP[2])/(x-SVP[1]); return {x+dx,y+dy} end
local s1, s2 = gdir(HX-6,HY+1,70), gdir(HX+6,HY+2,70)
fill(poly({{HX-6,HY-1},{HX+6,HY+2},{s2[1],s2[2]+1},{s1[1]-4,s1[2]-2.5},{s1[1]-6,s1[2]+1}}), walk_sh2, {hand="detail", tool="round 2.5", angle=0, length={6,20}, coverage=3})
-- body
local body = poly({{HX-6,HY},{HX+6,HY},{HX+5.5,HY-16},{HX-5.5,HY-16}}) + ellipse(HX,HY-17,6,4.5) + rect(HX-8,HY-3,16,3) + rect(HX-9,HY-11,18,4)
fill(body, hyd_s, {hand="detail", tool="round 2", angle=1.5708, length={3,8}, coverage=3})
fill(poly({{HX,HY-1},{HX+6,HY-1},{HX+5.5,HY-16},{HX+1,HY-18},{HX+1,HY-11},{HX+9,HY-11},{HX+9,HY-7},{HX+1,HY-7}}), hyd, {hand="detail", tool="round 1.6", angle=1.5708, length={3,8}, coverage=3})

--@ chunk 100
local function gdir(x,y,len) local dx=-len; local dy = dx*(y-SVP[2])/(x-SVP[1]); return {x+dx,y+dy} end
local a,b = gdir(HX,HY-1.5,60), gdir(HX,HY+3,60)
local tip = gdir(HX,HY+0.5,74)
fill(poly({{HX-5,HY-2},{HX+4,HY+3},{b[1],b[2]},{tip[1],tip[2]},{a[1],a[2]}},true) + ellipse(a[1]-3,(a[2]+b[2])/2,4,3), walk_sh2, {hand="detail", tool="round 2.5", angle=0, length={6,20}, coverage=3})
local n = brush{kind="round", width=2.4}
n:reload(hyd_s,0.5); n:stroke({{HX,HY-21},{HX,HY-23.5}},{pressure=0.8})

--@ chunk 101
street_sh_dk = pile{{"lead white",0.5},{"ultramarine blue",0.45},{"bone black",0.35},{"red earth",0.3},{"burnt sienna",0.15}, medium=0.03, name="street_sh_dk"}
local m = (M_street * FGS) + ((BSH * M_street) - FGS)
m = m * mask(function(x,y) return y > 520 and 1 or 0 end)
work(m * mask(function(x,y) return y>svp_line_y(1000,598,x)+4 and 1 or 0 end), {hand="body", tool="filbert 14", piles={{street_sh, function(x,y) return clamp(1-(y-540)/85,0,1) end},{street_sh_dk, function(x,y) return clamp((y-540)/85,0,1) end}},
   angle=0.06, length={40,120}, coverage=2.2, fill=true, clip=true, seed=51})

--@ chunk 102
curb_face_sh = pile{{"lead white",0.6},{"ultramarine blue",0.4},{"bone black",0.3},{"red earth",0.25}, medium=0.04, name="curb_face_sh"}
curb_top_sh = pile{{"lead white",1.6},{"ultramarine blue",0.3},{"red earth",0.25},{"bone black",0.1},{"yellow ochre",0.1}, medium=0.04, name="curb_top_sh"}
local function ly(x) return 465 + x*(77/290) end
-- curb face: a band below the curb line on the left, x 0..290 (narrower toward far end)
local face = poly({{0,ly(0)},{290,ly(290)},{305,546},{320,547},{320,552.5},{300,551},{288,548},{0,ly(0)+3}},false)
fill(face, curb_face_sh, {hand="detail", tool="flat 2.5", angle=0.26, length={15,40}, coverage=3})
local b = brush{kind="flat", width=1.6}
b:reload(curb_top_sh,0.6); b:stroke({{0,ly(0)-0.8},{171,ly(171)-0.8}},{pressure=0.6})
b:reload(curb_top,0.6); b:stroke({{171,ly(171)-0.8},{290,ly(290)-0.8},{305,545},{326,545.5}},{pressure=0.7})

--@ chunk 103
walk_w = pile{{"lead white",4},{"yellow ochre",0.65},{"red earth",0.18},{"deep cadmium",0.05},{"bone black",0.04}, medium=0.03, name="walk_w"}
walk_c = pile{{"lead white",4},{"yellow ochre",0.4},{"cerulean blue",0.08},{"red earth",0.08},{"bone black",0.07}, medium=0.03, name="walk_c"}
local excl = rect(560,495,30,30) + rect(495,505,75,22) + rect(700,455,110,22) + rect(805,455,60,25)
local sunwalk = (M_walk - SHD2 - BSH) * mask(function(x,y) return y > RY(x,475)+2 and y < 546 + (x-320)*(510-546)/680 - 2 and 1 or 0 end)
SUNWALK = sunwalk - excl
local N2 = noise{seed=12, period=90, octaves=3}
work(SUNWALK * rect(600,470,150,50), {hand="scumble", tool="filbert 8", piles={{walk_w, function(x,y) return clamp(0.5+N2(x,y),0,1) end},{walk_c, function(x,y) return clamp(0.5-N2(x,y),0,1) end}}, angle=-0.03, coverage=1.2, clip=true, seed=61})

--@ chunk 104
local N2 = noise{seed=12, period=90, octaves=3}
work(SUNWALK - rect(600,470,150,50), {hand="scumble", tool="filbert 8", piles={{walk_w, function(x,y) return clamp(0.5+N2(x,y),0,1) end},{walk_c, function(x,y) return clamp(0.5-N2(x,y),0,1) end}}, angle=-0.03, coverage=1.2, clip=true, seed=62})

--@ chunk 105
asph = pile{{"lead white",1.4},{"bone black",0.3},{"red earth",0.3},{"yellow ochre",0.35},{"ultramarine blue",0.15}, medium=0.03, name="asph"}

--@ chunk 106
asph:add{{"bone black",0.15},{"red earth",0.1},{"burnt sienna",0.1}}

--@ chunk 107
local function cy(x) return 546 + (x-320)*(510-546)/680 end
local m = mask(function(x,y) return (x>=300 and y > cy(x)+5.5 and y < svp_line_y(1000,598,x)) and 1 or 0 end) - BSH
local m2 = mask(function(x,y) return (x<300 and y > bsh_y(x) and y < svp_line_y(1000,598,x) and y > 465 + x*(77/290)+3) and 1 or 0 end)
STREETSUN = m + m2
work(STREETSUN, {hand="body", tool="filbert 12", pile=asph, angle=-0.03, length={40,120}, coverage=2.4, fill=true, clip=true, seed=71})

--@ chunk 108
print(wait(12*24*60)); for _,p in ipairs({{650,500},{800,560},{575,510},{830,420},{150,30}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 109
local sw = (M_walk - SHD2 - BSH) * mask(function(x,y) return y > RY(x,475)+2 and y < 546 + (x-320)*(510-546)/680 - 1.5 and 1 or 0 end)
local patch = sw * rect(492,492,100,34)
local N2 = noise{seed=12, period=90, octaves=3}
work(patch, {hand="body", tool="filbert 6", piles={{walk_w, function(x,y) return clamp(0.5+N2(x,y),0,1) end},{walk_c, function(x,y) return clamp(0.5-N2(x,y),0,1) end}}, angle=-0.03, length={15,40}, coverage=3, fill=true, clip=true, seed=81})

--@ chunk 110
print(wait(14*24*60)); print(drying(540,510))

--@ chunk 111
sig = pile{{"red earth",0.8},{"burnt sienna",0.6},{"bone black",0.3},{"lead white",0.2}, medium=0.1, name="sig"}
local b = brush{kind="round", width=1.6, point=0.7}
b:reload(sig,0.5)
-- "Claude" in a quick hand, lower right
local x0,y0 = 905,608
local function s(pts) local q={} for _,p in ipairs(pts) do q[#q+1]={x0+p[1],y0+p[2]} end b:load(sig,0.3); b:stroke(q,{pressure=0.5}) end
s({{6,-6},{2,-8},{-1,-5},{-1,-1},{2,1},{6,0}})   -- C
s({{9,-11},{8,1}})                                -- l
s({{17,-5},{13,-5},{12,-1},{14,1},{17,-1},{17,-5},{18,1}}) -- a
s({{21,-5},{21,-1},{23,1},{26,-1},{26,-5},{27,1}}) -- u
s({{36,-11},{35,1},{32,1},{30,-1},{31,-4},{35,-4}}) -- d
s({{39,-2},{43,-3},{42,-5},{39,-4},{38,-1},{40,1},{43,0}}) -- e

--@ chunk 112
for _,p in ipairs{{400,230},{560,250},{470,410},{640,390},{910,350},{960,350},{200,250},{300,380},{150,120}} do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 113
for i,w in ipairs(RWIN) do local s="" for k,v in pairs(w) do s=s..tostring(k).."="..tostring(v).." " end print(i,s) end
for k,v in pairs(AW) do print("AW",k,v) end
for k,v in pairs(FIG) do print("FIG",k,v) end

--@ chunk 114
curt = pile{{"lead white",1},{"ultramarine blue",0.35},{"burnt sienna",0.3},{"bone black",0.2},{"yellow ochre",0.15}, medium=0.04, name="curtain"}
curt2 = pile{{"lead white",1.6},{"ultramarine blue",0.3},{"burnt sienna",0.25},{"bone black",0.15},{"yellow ochre",0.25}, medium=0.04, name="curtain_lt"}

--@ chunk 115
local m = poly({{426,223},{436,223},{436,265},{429,265},{427,250},{425,236}}, true)
local b = brush{kind="filbert", width=4, stiffness=0.5}
b:load(curt, 0.6)
for i,x in ipairs{428,431,434} do
  b:gesture({{x,223,0.6},{x-1+ (i==1 and 0 or 0.5),245,0.6},{x+ (i==1 and 2 or 0),265,0.4}}, {clip=m})
end
b:reload(curt2, 0.4)
b:gesture({{433,224,0.4},{433.5,245,0.45},{434,264,0.3}}, {clip=m})

--@ chunk 116
curt3 = mix{{curt,0.7},{curt2,0.3}, name="curt3"}
curt3:add{{"yellow ochre",0.08},{"red earth",0.04}}
local m = poly({{644,238},{656,238},{658,252},{655,274},{644,274}}, true)
local b = brush{kind="filbert", width=4, stiffness=0.5}
b:load(curt3, 0.6)
for i,x in ipairs{646,649,652} do
  b:gesture({{x,238,0.6},{x+1.5,256,0.6},{x,274,0.4}}, {clip=m})
end
b:reload(curt2, 0.35)
b:gesture({{647,239,0.4},{648,256,0.45},{647,273,0.3}}, {clip=m})

--@ chunk 117
local b = brush{kind="round", width=3, point=0.5}
b:load(glass2, 0.5)
b:gesture({{657,240,0.3},{655.5,256,0.6},{657,272,0.3}})
b:gesture({{659,240,0.5},{657.5,256,0.7},{659,272,0.5}})

--@ chunk 118
mirror = pile{{"ultramarine blue",1},{"burnt sienna",0.7},{"bone black",0.35},{"lead white",1.1},{"cerulean blue",0.15}, medium=0.04, name="mirror"}
chairc = pile{{"lead white",1.2},{"ultramarine blue",0.3},{"burnt sienna",0.25},{"bone black",0.18},{"yellow ochre",0.2}, medium=0.04, name="chairc"}
local b = brush{kind="flat", width=5, stiffness=0.5}
b:load(mirror, 0.5)
local mm = rect(368,404,50,15)
for y=405,418,3.5 do b:stroke({{369,y},{417,y}}, {pressure=0.5, clip=mm}) end
local mm2 = rect(523,404,50,15)
b:reload(mirror,0.5)
for y=405,418,3.5 do b:stroke({{524,y},{572,y}}, {pressure=0.5, clip=mm2}) end

--@ chunk 119
for _,m in ipairs{rect(366,402,54,19), rect(521,402,54,19)} do blend(m, {angle=0, tool={kind="badger", width=10}, coverage=2}) end

--@ chunk 120
local b = brush{kind="round", width=2, point=0.6}
b:load(glass_d, 0.6)
-- shelf/counter line under mirrors
b:stroke({{367,421},{419,421}}, {pressure=0.6})
b:stroke({{522,421},{574,421}}, {pressure=0.6})
-- chair backs (headrest + back) as dark silhouettes
local function chair(cx)
  local m = poly({{cx-3,409},{cx+3,409},{cx+4,413},{cx+5,414},{cx+6,428},{cx-6,428},{cx-5,414},{cx-4,413}}, true)
  work(m, {hand="detail", pile=glass_d, coverage=2, clip=true})
end
chair(394); chair(548)

--@ chunk 121
local b = brush{kind="round", width=2, point=0.4}
b:load(chairc, 0.5)
for _,cx in ipairs{394,548} do
  b:stroke({{cx-10,423},{cx-5,423.5}}, {pressure=0.6})
  b:stroke({{cx+5,423.5},{cx+10,423}}, {pressure=0.6})
end
local d = brush{kind="round", width=1.5, point=0.6}
d:load(mirror, 0.4)
for _,cx in ipairs{394,548} do d:stroke({{cx-4,412.5},{cx+4,412.5}}, {pressure=0.4}) end

--@ chunk 122
local m = poly({{617,357},{667,357},{667,377},{617,376.5}})
work(m, {hand="detail", pile=shade_y_sh, coverage=2.5, angle=0, clip=true, tool={kind="flat", width=4}})
local b = brush{kind="round", width=1.6, point=0.5}
b:load(shade_sh, 0.5)
b:stroke({{617,377.3},{667,377.8}}, {pressure=0.5})

--@ chunk 123
local b = brush{kind="flat", width=3, stiffness=0.5}
local function shadeIn(x0,x1,y0,y1)
  work(rect(x0,y0,x1-x0,y1-y0), {hand="detail", pile=bgshade, coverage=2.2, angle=0, clip=true, tool={kind="flat", width=3}})
end
shadeIn(891,905,260,268)
shadeIn(920,934,337,346)
shadeIn(891,905,375,389)
shadeIn(950,962,325,333)
shadeIn(975,988,403,411)
shadeIn(975,988,285,297)
-- sills on red building
local s = brush{kind="round", width=1.6, point=0.3}
s:load(bgr1, 0.6)
for _,y in ipairs{306.5,346.5,385.5,424.5} do
  s:stroke({{948,y},{964,y}}, {pressure=0.6}); s:stroke({{973,y},{990,y}}, {pressure=0.6})
end
s:reload(bgshade, 0.6)
for _,y in ipairs{281.5,320.5,359.5,397.5,435.5} do
  s:stroke({{889,y},{907,y}}, {pressure=0.6}); s:stroke({{918,y},{936,y}}, {pressure=0.6})
end

--@ chunk 124
local rr = rag{width=10}
local spots = {{950,962,325,333},{975,988,285,297},{975,988,403,411},{891,905,260,268},{920,934,337,346},{891,905,375,389}}
for _,s in ipairs(spots) do
  rr:dip(0.85)
  local m = rect(s[1]-1,s[3]-1,s[2]-s[1]+2,s[4]-s[3]+2)
  rr:wipe(m, {pressure=0.75, angle=0, passes=3, refold=0.3})
  rr:refold()
end

--@ chunk 125
local rr = rag{width=8}
local spots = {{950,962,325,333},{975,988,285,297},{975,988,403,411},{891,905,260,268},{920,934,337,346},{891,905,375,389}}
for _,s in ipairs(spots) do
  for k=1,3 do
  rr:dip(0.95)
  local m = rect(s[1]-1,s[3]-1,s[2]-s[1]+2,s[4]-s[3]+2)
  rr:wipe(m, {pressure=0.9, angle=(k%2)*1.57, passes=2, refold=0.2})
  rr:refold()
  end
end

--@ chunk 126
print(wait(14*24*60))
for _,p in ipairs{{960,300},{985,410},{898,265},{648,256},{430,245},{390,412},{640,365}} do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 127
local wins = rect(948,290,13,18)+rect(972,290,13,18)+rect(948,328,13,18)+rect(972,328,13,18)+rect(948,404,13,18)+rect(972,404,13,18)
local zone = rect(968,281,24,19)+rect(943,318,22,18)+rect(968,396,24,18)
work(zone - wins, {hand="detail", pile=bgr2, coverage=2.5, clip=true, tool={kind="flat", width=3}})
work(zone * wins, {hand="detail", pile=bgwin2, coverage=2.5, clip=true, tool={kind="flat", width=3}})

--@ chunk 128
local rr = rag{width=8}
local zones = {{968,281,24,19},{943,318,22,18},{968,396,24,18}}
for _,z in ipairs(zones) do
  for k=1,4 do
    rr:dip(0.95)
    rr:wipe(rect(z[1],z[2],z[3],z[4]), {pressure=0.9, angle=(k%2)*1.57, passes=2, refold=0.15})
    rr:refold()
  end
end

--@ chunk 129
print(wait(12*24*60)); print(drying(985,290), drying(950,330), drying(985,405))

--@ chunk 130
local wins = rect(955,291,12,20)+rect(979,291,13,20)+rect(955,330,12,18)+rect(979,330,13,18)+rect(955,405,12,19)+rect(979,405,13,19)
local zone = rect(970,284,30,18)+rect(938,320,24,17)+rect(970,398,30,19)
-- keep away from the cream building edge (x<944) 
zone = zone * rect(945,280,60,160)
work(zone - wins, {hand="detail", pile=bgr2, coverage=2.5, clip=true, tool={kind="flat", width=3}})
work(zone * wins, {hand="detail", pile=bgwin2, coverage=2.5, clip=true, tool={kind="flat", width=3}})

--@ chunk 131
print(wait(12*24*60)); print(drying(898,265), drying(927,340), drying(898,380), drying(960,300))

--@ chunk 132
local w = rect(891,260,14,20)+rect(920,336,14,20)+rect(891,373,14,20)
work(w, {hand="detail", pile=bgwin2, coverage=3, clip=true, angle=1.57, tool={kind="flat", width=3}})
-- small cream patch where the cream/red edge got smeared
work(rect(941,316,4,16), {hand="detail", pile=bgl, coverage=2.5, clip=true, angle=1.57, tool={kind="flat", width=2}})

--@ chunk 133
inner = pile{{"ultramarine blue",1},{"burnt sienna",0.8},{"bone black",0.35},{"lead white",0.9},{"red earth",0.15}, medium=0.04, name="inner"}

--@ chunk 134
local m = poly({{294,352},{314,350},{314,433},{294,432}})
work(m, {hand="body", pile=inner, tool={kind="filbert", width=6}, coverage=1.6, angle=1.57, clip=rect(240,349,77,88), edge="soft"})
blend(m:grow(4) * rect(240,349,77,88), {angle=1.57, tool={kind="badger", width=12}})

--@ chunk 135
local rr = rag{width=10}
for k=1,3 do rr:dip(0.9); rr:wipe(rect(287,350,30,86), {pressure=0.8, angle=1.57, passes=2, refold=0.2}); rr:refold() end

--@ chunk 136
local rr = rag{width=8}
for k=1,4 do rr:dip(0.95); rr:wipe(rect(287,349,31,90), {pressure=0.9, angle=(k%2)*1.57, passes=2, refold=0.12}); rr:refold() end
print(drying(300,400))

--@ chunk 137
print(wait(14*24*60)); print(drying(300,400), drying(913,345), drying(398,412))

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
