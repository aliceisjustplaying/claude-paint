-- easel session "scratch": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box every
--@ engine 5

--@ chunk 1
canvas{aspect=1.3, ground={{apply="knife", pile={{"lead white", 6}, {"yellow ochre", 1}, {"red earth", 0.4}}, texture=0.4, um=120}}, linen={18, 16}, seed=417, size=650}

--@ chunk 2

d_plum = pile{{"permanent alizarin",0.20},{"cobalt violet",0.33},{"burnt sienna",0.20},{"ultramarine blue",0.10},{"lead white",0.17}, name="d_plum"}
d_dk = pile{{"cobalt violet",0.32},{"ultramarine blue",0.13},{"rose madder",0.10},{"lead white",0.39},{"burnt sienna",0.06}, name="d_dk"}
d_glow = pile{{"vermilion",0.21},{"orange chrome",0.21},{"cobalt violet",0.21},{"lead white",0.36}, name="d_glow"}
d_rim = pile{{"orange chrome",0.54},{"cadmium yellow",0.21},{"vermilion",0.14},{"lead white",0.11}, name="d_rim"}
d_eave = pile{{"ultramarine blue",0.40},{"permanent alizarin",0.24},{"burnt sienna",0.16},{"cobalt violet",0.20}, name="d_eave"}
-- underlayer: roof above y 100, drum below; dark band 100-107
work(rect(100,40,300,60), {hand="body", piles={{d_dk,function(x,y) return 1-(x-100)/300 end},{d_glow,function(x,y) return (x-100)/300 end}}, coverage=2.5, fill=true, angle=1.2})
work(rect(100,107,300,60), {hand="body", piles={{d_plum,1},{d_dk,0.6}}, coverage=2.5, fill=true, angle=math.pi/2})
work(rect(100,100,300,7), {hand="detail", pile=d_eave, coverage=2, angle=0})
-- some ticks
local b = brush{kind="round", width=2.5, point=0.6}
for i=1,20 do b:load(d_eave,0.5) local x=110+i*14 b:stroke({{x,86},{x,98}},{pressure=0.6}) end
wait(20*24*60)

--@ chunk 3

-- fringe trial: pointed round strands from above band down past it, tapering
fr = brush{kind="round", width=3, point=0.8}
local function pick(t)
  if t < 0.55 then return (math.random()<0.6) and d_plum or d_dk
  elseif t < 0.85 then return (math.random()<0.6) and d_glow or d_plum
  else return (math.random()<0.6) and d_rim or d_glow end
end
local x = 104
while x < 396 do
  local t = (x-100)/300
  local p = pick(t)
  fr:reload(p, rand(0.35,0.6))
  local lean = (t-0.5)*6 + randn(0,1.5)
  local y0 = rand(84,92)
  local y1 = 100 + rand(-1,10)
  fr:gesture({{x, y0, 0.7},{x+lean*0.5,(y0+y1)/2, 0.6},{x+lean, y1, 0.0}}, {wobble=0.6})
  x = x + rand(2.5,6)
end

--@ chunk 4

work(rect(100,240,300,60), {hand="body", piles={{d_dk,function(x,y) return 1-(x-100)/300 end},{d_glow,function(x,y) return (x-100)/300 end}}, coverage=2.5, fill=true, angle=1.2})
work(rect(100,307,300,60), {hand="body", piles={{d_plum,1},{d_dk,0.6}}, coverage=2.5, fill=true, angle=math.pi/2})
work(rect(100,300,300,7), {hand="detail", pile=d_eave, coverage=2, angle=0})
local b = brush{kind="round", width=2.5, point=0.6}
for i=1,20 do b:load(d_eave,0.5) local x=110+i*14 b:stroke({{x,286},{x,298}},{pressure=0.6}) end
wait(20*24*60)

--@ chunk 5

local roofb = brush("filbert", 5)
local function rpile(t)
  if t < 0.5 then return (math.random()<0.55) and d_plum or d_dk
  elseif t < 0.8 then return (math.random()<0.5) and d_glow or d_plum
  else return (math.random()<0.55) and d_rim or d_glow end
end
-- (a) roof straw ends, filbert, along the fall, blunt end ragged near the band
local x = 102
while x < 398 do
  local t = (x-100)/300
  roofb:reload(rpile(t), rand(0.4,0.65))
  local lean = (t-0.5)*5 + randn(0,1.2)
  local y0 = rand(276,284)
  local y1 = 300 + randn(1.5,2.5)
  roofb:stroke({{x-lean*0.6, y0},{x+lean*0.4, y1}}, {pressure={0.55,0.4}, ramps={0.2,0.1}})
  x = x + rand(4,8)
end
-- (b) underside: irregular dark lobes hanging below band
local db = brush("filbert", 4)
x = 104
while x < 396 do
  db:reload(d_eave, rand(0.3,0.5))
  local len = math.max(1.5, randn(5,3))
  db:stroke({{x, 304},{x+randn(0,0.8), 304+len}}, {pressure={0.5,0.25}})
  x = x + rand(5,12)
end

--@ chunk 6

work(rect(100,440,300,60), {hand="body", piles={{d_dk,function(x,y) return 1-(x-100)/300 end},{d_glow,function(x,y) return (x-100)/300 end}}, coverage=2.5, fill=true, angle=1.2})
work(rect(100,507,300,60), {hand="body", piles={{d_plum,1},{d_dk,0.6}}, coverage=2.5, fill=true, angle=math.pi/2})
work(rect(100,500,300,7), {hand="detail", pile=d_eave, coverage=2, angle=0})
local b = brush{kind="round", width=2.5, point=0.6}
for i=1,20 do b:load(d_eave,0.5) local x=110+i*14 b:stroke({{x,486},{x,498}},{pressure=0.6}) end
wait(20*24*60)

--@ chunk 7

function yA(x) local u=(x-250)/150 return 500 + 7*(1-u*u) end
local skirt = mask(function(x,y) local a=yA(x) return (x>100 and x<400 and y>a-16 and y<a+0.5) and 1 or 0 end):roughen(1.5, 12, 5)
work(skirt, {hand="body", tool="filbert 4", length={10,24}, piles={{d_dk,function(x,y) return math.max(0,0.8-(x-100)/300) end},{d_plum,0.5},{d_glow,function(x,y) return math.max(0,(x-220)/180) end},{d_rim,function(x,y) return math.max(0,(x-330)/70)*1.2 end}}, coverage=1.6, angle=function(x,y) return math.atan(-14*(x-250)/22500,1) + 0.25*((x-250)/150) end, clip=true})
local band = mask(function(x,y) local a=yA(x) return (x>102 and x<398 and y>a and y<a+6) and 1 or 0 end):roughen(2, 10, 8)
work(band, {hand="body", tool="filbert 3", length={8,20}, pile=d_eave, coverage=1.4, angle=function(x,y) return math.atan(-14*(x-250)/22500,1) end, load=0.5})
